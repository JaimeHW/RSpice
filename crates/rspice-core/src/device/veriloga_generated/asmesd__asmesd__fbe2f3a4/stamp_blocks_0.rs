#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_arg_slot: &mut f64,
        var_arg0_slot: &mut f64,
        var_arg0_dn3_slot: &mut f64,
        var_arg_dn3_slot: &mut f64,
        var_arg_dn4_slot: &mut f64,
        var_arg_dn5_slot: &mut f64,
        var_arg_dn6_slot: &mut f64,
        var_argbv_slot: &mut f64,
        var_argbv_dn3_slot: &mut f64,
        var_argbv_dn4_slot: &mut f64,
        var_argbv_dn5_slot: &mut f64,
        var_argbv_dn6_slot: &mut f64,
        var_argbvvt_slot: &mut f64,
        var_argbvvt_dn3_slot: &mut f64,
        var_argt_slot: &mut f64,
        var_argt_dn3_slot: &mut f64,
        var_argtr_slot: &mut f64,
        var_argtr_dn3_slot: &mut f64,
        var_bf_t_slot: &mut f64,
        var_bf_t_dn3_slot: &mut f64,
        var_bf_t_dn4_slot: &mut f64,
        var_bf_t_dn5_slot: &mut f64,
        var_br_t_slot: &mut f64,
        var_br_t_dn3_slot: &mut f64,
        var_bvr_t_slot: &mut f64,
        var_bvr_t_dn3_slot: &mut f64,
        var_cjc_i_slot: &mut f64,
        var_cjc_t_slot: &mut f64,
        var_cjc_t_dn3_slot: &mut f64,
        var_cje_i_slot: &mut f64,
        var_cje_t_slot: &mut f64,
        var_cje_t_dn3_slot: &mut f64,
        var_cjs_i_slot: &mut f64,
        var_cjs_t_slot: &mut f64,
        var_cjs_t_dn3_slot: &mut f64,
        var_cjt_slot: &mut f64,
        var_cjt_dn3_slot: &mut f64,
        var_egfet_slot: &mut f64,
        var_egfet_dn3_slot: &mut f64,
        var_fact1_slot: &mut f64,
        var_fact2_slot: &mut f64,
        var_fact2_dn3_slot: &mut f64,
        var_fbwm_slot: &mut f64,
        var_fbwm_dn4_slot: &mut f64,
        var_fbwm_dn5_slot: &mut f64,
        var_gmanew_slot: &mut f64,
        var_gmanew_dn3_slot: &mut f64,
        var_gmaold_slot: &mut f64,
        var_gmaold_dn3_slot: &mut f64,
        var_guard3_slot: &mut f64,
        var_guard4_slot: &mut f64,
        var_ijbv_t_slot: &mut f64,
        var_ijbv_t_dn3_slot: &mut f64,
        var_ijbvc_t_slot: &mut f64,
        var_ijbvc_t_dn3_slot: &mut f64,
        var_is_t_slot: &mut f64,
        var_is_t_dn3_slot: &mut f64,
        var_isc_t_slot: &mut f64,
        var_isc_t_dn3_slot: &mut f64,
        var_ise_t_slot: &mut f64,
        var_ise_t_dn3_slot: &mut f64,
        var_isr_t_slot: &mut f64,
        var_isr_t_dn3_slot: &mut f64,
        var_lnrt_slot: &mut f64,
        var_lnrt_dn3_slot: &mut f64,
        var_oikf_slot: &mut f64,
        var_oikf_dn4_slot: &mut f64,
        var_oikf_dn5_slot: &mut f64,
        var_oikr_slot: &mut f64,
        var_ovaf_slot: &mut f64,
        var_ovar_slot: &mut f64,
        var_pbfact_slot: &mut f64,
        var_pbfact_dn3_slot: &mut f64,
        var_pbo_slot: &mut f64,
        var_pbo_dn3_slot: &mut f64,
        var_rt_slot: &mut f64,
        var_rt_dn3_slot: &mut f64,
        var_tamb_slot: &mut f64,
        var_tamb_dn3_slot: &mut f64,
        var_tbeta_slot: &mut f64,
        var_tbeta_dn3_slot: &mut f64,
        var_tdev_slot: &mut f64,
        var_tdev_dn3_slot: &mut f64,
        var_theexp_t_slot: &mut f64,
        var_theexp_t_dn3_slot: &mut f64,
        var_tnom_slot: &mut f64,
        var_ttype_slot: &mut f64,
        var_vbbi_slot: &mut f64,
        var_vbbi_dn1_slot: &mut f64,
        var_vbbi_dn5_slot: &mut f64,
        var_vbc_slot: &mut f64,
        var_vbc_dn4_slot: &mut f64,
        var_vbc_dn5_slot: &mut f64,
        var_vbci_slot: &mut f64,
        var_vbci_dn1_slot: &mut f64,
        var_vbci_dn4_slot: &mut f64,
        var_vbici_slot: &mut f64,
        var_vbici_dn4_slot: &mut f64,
        var_vbici_dn5_slot: &mut f64,
        var_vbiei_slot: &mut f64,
        var_vbiei_dn5_slot: &mut f64,
        var_vbiei_dn6_slot: &mut f64,
        var_veci_slot: &mut f64,
        var_veci_dn2_slot: &mut f64,
        var_veci_dn4_slot: &mut f64,
        var_veei_slot: &mut f64,
        var_veei_dn2_slot: &mut f64,
        var_veei_dn6_slot: &mut f64,
        var_vjc_t_slot: &mut f64,
        var_vjc_t_dn3_slot: &mut f64,
        var_vje_t_slot: &mut f64,
        var_vje_t_dn3_slot: &mut f64,
        var_vjs_t_slot: &mut f64,
        var_vjs_t_dn3_slot: &mut f64,
        var_vt_slot: &mut f64,
        var_vt_dn3_slot: &mut f64,
        var_weff_slot: &mut f64,
    ) {
        let ctx_temp = ctx.temperature();
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let mut var_arg: f64 = *var_arg_slot;
        let mut var_arg0: f64 = *var_arg0_slot;
        let mut var_arg0_dn3: f64 = *var_arg0_dn3_slot;
        let mut var_arg_dn3: f64 = *var_arg_dn3_slot;
        let mut var_arg_dn4: f64 = *var_arg_dn4_slot;
        let mut var_arg_dn5: f64 = *var_arg_dn5_slot;
        let mut var_arg_dn6: f64 = *var_arg_dn6_slot;
        let mut var_argbv: f64 = *var_argbv_slot;
        let mut var_argbv_dn3: f64 = *var_argbv_dn3_slot;
        let mut var_argbv_dn4: f64 = *var_argbv_dn4_slot;
        let mut var_argbv_dn5: f64 = *var_argbv_dn5_slot;
        let mut var_argbv_dn6: f64 = *var_argbv_dn6_slot;
        let mut var_argbvvt: f64 = *var_argbvvt_slot;
        let mut var_argbvvt_dn3: f64 = *var_argbvvt_dn3_slot;
        let mut var_argt: f64 = *var_argt_slot;
        let mut var_argt_dn3: f64 = *var_argt_dn3_slot;
        let mut var_argtr: f64 = *var_argtr_slot;
        let mut var_argtr_dn3: f64 = *var_argtr_dn3_slot;
        let mut var_bf_t: f64 = *var_bf_t_slot;
        let mut var_bf_t_dn3: f64 = *var_bf_t_dn3_slot;
        let mut var_bf_t_dn4: f64 = *var_bf_t_dn4_slot;
        let mut var_bf_t_dn5: f64 = *var_bf_t_dn5_slot;
        let mut var_br_t: f64 = *var_br_t_slot;
        let mut var_br_t_dn3: f64 = *var_br_t_dn3_slot;
        let mut var_bvr_t: f64 = *var_bvr_t_slot;
        let mut var_bvr_t_dn3: f64 = *var_bvr_t_dn3_slot;
        let mut var_cjc_i: f64 = *var_cjc_i_slot;
        let mut var_cjc_t: f64 = *var_cjc_t_slot;
        let mut var_cjc_t_dn3: f64 = *var_cjc_t_dn3_slot;
        let mut var_cje_i: f64 = *var_cje_i_slot;
        let mut var_cje_t: f64 = *var_cje_t_slot;
        let mut var_cje_t_dn3: f64 = *var_cje_t_dn3_slot;
        let mut var_cjs_i: f64 = *var_cjs_i_slot;
        let mut var_cjs_t: f64 = *var_cjs_t_slot;
        let mut var_cjs_t_dn3: f64 = *var_cjs_t_dn3_slot;
        let mut var_cjt: f64 = *var_cjt_slot;
        let mut var_cjt_dn3: f64 = *var_cjt_dn3_slot;
        let mut var_egfet: f64 = *var_egfet_slot;
        let mut var_egfet_dn3: f64 = *var_egfet_dn3_slot;
        let mut var_fact1: f64 = *var_fact1_slot;
        let mut var_fact2: f64 = *var_fact2_slot;
        let mut var_fact2_dn3: f64 = *var_fact2_dn3_slot;
        let mut var_fbwm: f64 = *var_fbwm_slot;
        let mut var_fbwm_dn4: f64 = *var_fbwm_dn4_slot;
        let mut var_fbwm_dn5: f64 = *var_fbwm_dn5_slot;
        let mut var_gmanew: f64 = *var_gmanew_slot;
        let mut var_gmanew_dn3: f64 = *var_gmanew_dn3_slot;
        let mut var_gmaold: f64 = *var_gmaold_slot;
        let mut var_gmaold_dn3: f64 = *var_gmaold_dn3_slot;
        let mut var_guard3: f64 = *var_guard3_slot;
        let mut var_guard4: f64 = *var_guard4_slot;
        let mut var_ijbv_t: f64 = *var_ijbv_t_slot;
        let mut var_ijbv_t_dn3: f64 = *var_ijbv_t_dn3_slot;
        let mut var_ijbvc_t: f64 = *var_ijbvc_t_slot;
        let mut var_ijbvc_t_dn3: f64 = *var_ijbvc_t_dn3_slot;
        let mut var_is_t: f64 = *var_is_t_slot;
        let mut var_is_t_dn3: f64 = *var_is_t_dn3_slot;
        let mut var_isc_t: f64 = *var_isc_t_slot;
        let mut var_isc_t_dn3: f64 = *var_isc_t_dn3_slot;
        let mut var_ise_t: f64 = *var_ise_t_slot;
        let mut var_ise_t_dn3: f64 = *var_ise_t_dn3_slot;
        let mut var_isr_t: f64 = *var_isr_t_slot;
        let mut var_isr_t_dn3: f64 = *var_isr_t_dn3_slot;
        let mut var_lnrt: f64 = *var_lnrt_slot;
        let mut var_lnrt_dn3: f64 = *var_lnrt_dn3_slot;
        let mut var_oikf: f64 = *var_oikf_slot;
        let mut var_oikf_dn4: f64 = *var_oikf_dn4_slot;
        let mut var_oikf_dn5: f64 = *var_oikf_dn5_slot;
        let mut var_oikr: f64 = *var_oikr_slot;
        let mut var_ovaf: f64 = *var_ovaf_slot;
        let mut var_ovar: f64 = *var_ovar_slot;
        let mut var_pbfact: f64 = *var_pbfact_slot;
        let mut var_pbfact_dn3: f64 = *var_pbfact_dn3_slot;
        let mut var_pbo: f64 = *var_pbo_slot;
        let mut var_pbo_dn3: f64 = *var_pbo_dn3_slot;
        let mut var_rt: f64 = *var_rt_slot;
        let mut var_rt_dn3: f64 = *var_rt_dn3_slot;
        let mut var_tamb: f64 = *var_tamb_slot;
        let mut var_tamb_dn3: f64 = *var_tamb_dn3_slot;
        let mut var_tbeta: f64 = *var_tbeta_slot;
        let mut var_tbeta_dn3: f64 = *var_tbeta_dn3_slot;
        let mut var_tdev: f64 = *var_tdev_slot;
        let mut var_tdev_dn3: f64 = *var_tdev_dn3_slot;
        let mut var_theexp_t: f64 = *var_theexp_t_slot;
        let mut var_theexp_t_dn3: f64 = *var_theexp_t_dn3_slot;
        let mut var_tnom: f64 = *var_tnom_slot;
        let mut var_ttype: f64 = *var_ttype_slot;
        let mut var_vbbi: f64 = *var_vbbi_slot;
        let mut var_vbbi_dn1: f64 = *var_vbbi_dn1_slot;
        let mut var_vbbi_dn5: f64 = *var_vbbi_dn5_slot;
        let mut var_vbc: f64 = *var_vbc_slot;
        let mut var_vbc_dn4: f64 = *var_vbc_dn4_slot;
        let mut var_vbc_dn5: f64 = *var_vbc_dn5_slot;
        let mut var_vbci: f64 = *var_vbci_slot;
        let mut var_vbci_dn1: f64 = *var_vbci_dn1_slot;
        let mut var_vbci_dn4: f64 = *var_vbci_dn4_slot;
        let mut var_vbici: f64 = *var_vbici_slot;
        let mut var_vbici_dn4: f64 = *var_vbici_dn4_slot;
        let mut var_vbici_dn5: f64 = *var_vbici_dn5_slot;
        let mut var_vbiei: f64 = *var_vbiei_slot;
        let mut var_vbiei_dn5: f64 = *var_vbiei_dn5_slot;
        let mut var_vbiei_dn6: f64 = *var_vbiei_dn6_slot;
        let mut var_veci: f64 = *var_veci_slot;
        let mut var_veci_dn2: f64 = *var_veci_dn2_slot;
        let mut var_veci_dn4: f64 = *var_veci_dn4_slot;
        let mut var_veei: f64 = *var_veei_slot;
        let mut var_veei_dn2: f64 = *var_veei_dn2_slot;
        let mut var_veei_dn6: f64 = *var_veei_dn6_slot;
        let mut var_vjc_t: f64 = *var_vjc_t_slot;
        let mut var_vjc_t_dn3: f64 = *var_vjc_t_dn3_slot;
        let mut var_vje_t: f64 = *var_vje_t_slot;
        let mut var_vje_t_dn3: f64 = *var_vje_t_dn3_slot;
        let mut var_vjs_t: f64 = *var_vjs_t_slot;
        let mut var_vjs_t_dn3: f64 = *var_vjs_t_dn3_slot;
        let mut var_vt: f64 = *var_vt_slot;
        let mut var_vt_dn3: f64 = *var_vt_dn3_slot;
        let mut var_weff: f64 = *var_weff_slot;

        let assign00_e447: f64 = ctx_temp;
        let assign00_e449: f64 = (assign00_e447 + (nv3 - 0.0));
        let assign00_e451: f64 = (assign00_e449 + p.p45);
        var_tamb = assign00_e451;
        var_tamb_dn3 = 1.0;

        let assign10_e454: f64 = (1026.85 + 273.15);
        let assign10_e457: f64 = (-100.0);
        let assign10_e459: f64 = (assign10_e457 + 273.15);
        let (assign10_e466, assign10_e466_d_n3,) = {
    if (var_tamb > assign10_e459) {
        (var_tamb, var_tamb_dn3,)
    } else {
        let assign10_e463: f64 = (-100.0);
        let assign10_e465: f64 = (assign10_e463 + 273.15);
        (assign10_e465, 0.0,)
    }
};
        let (assign10_e483, assign10_e483_d_n3,) = {
    if (assign10_e454 < assign10_e466) {
        let assign10_e470: f64 = (1026.85 + 273.15);
        (assign10_e470, 0.0,)
    } else {
        let assign10_e473: f64 = (-100.0);
        let assign10_e475: f64 = (assign10_e473 + 273.15);
        let (assign10_e482, assign10_e482_d_n3,) = {
            if (var_tamb > assign10_e475) {
                (var_tamb, var_tamb_dn3,)
            } else {
                let assign10_e479: f64 = (-100.0);
                let assign10_e481: f64 = (assign10_e479 + 273.15);
                (assign10_e481, 0.0,)
            }
        };
        (assign10_e482, assign10_e482_d_n3,)
    }
};
        var_tdev = assign10_e483;
        var_tdev_dn3 = assign10_e483_d_n3;

        let assign40_e493: f64 = (p.p43 * p.p42);
        var_weff = assign40_e493;

        let assign50_e496: f64 = (p.p29 * (nv5 - nv4));
        var_vbc = assign50_e496;
        var_vbc_dn4 = (-p.p29);
        var_vbc_dn5 = p.p29;

        let assign60_e501: f64 = (var_vbc).min(0.0);
        let assign60_e502: f64 = (-assign60_e501);
        let assign60_e504: f64 = (assign60_e502).powf(p.p80);
        let assign60_e505: f64 = (p.p79 * assign60_e504);
        let assign60_e506: f64 = (1.0 + assign60_e505);
        var_fbwm = assign60_e506;
        var_fbwm_dn4 = (p.p79 * if 0.0 == 0.0 && ((p.p80) as f64).is_finite() && ((p.p80) as f64).fract() == 0.0 { if p.p80 == 0.0 { 0.0 } else { (p.p80 * ((assign60_e502).powf(p.p80 - 1.0) * (-if var_vbc <= 0.0 { var_vbc_dn4 } else { 0.0 }))) } } else { (assign60_e504 * (p.p80 * ((-if var_vbc <= 0.0 { var_vbc_dn4 } else { 0.0 }) / assign60_e502))) });
        var_fbwm_dn5 = (p.p79 * if 0.0 == 0.0 && ((p.p80) as f64).is_finite() && ((p.p80) as f64).fract() == 0.0 { if p.p80 == 0.0 { 0.0 } else { (p.p80 * ((assign60_e502).powf(p.p80 - 1.0) * (-if var_vbc <= 0.0 { var_vbc_dn5 } else { 0.0 }))) } } else { (assign60_e504 * (p.p80 * ((-if var_vbc <= 0.0 { var_vbc_dn5 } else { 0.0 }) / assign60_e502))) });

        let assign70_e509: f64 = (p.p25 + 273.15);
        var_tnom = assign70_e509;

        let assign80_e512: f64 = (8.6170869e-5 * var_tdev);
        var_vt = assign80_e512;
        var_vt_dn3 = (8.6170869e-5 * var_tdev_dn3);

        let assign90_e515: f64 = (var_tdev / var_tnom);
        var_rt = assign90_e515;
        var_rt_dn3 = (var_tdev_dn3 / var_tnom);

        let assign100_e517: f64 = (var_rt).ln();
        var_lnrt = assign100_e517;
        var_lnrt_dn3 = (var_rt_dn3 / var_rt);

        let assign110_e520: f64 = (p.p77 * var_lnrt);
        let assign110_e521: f64 = (assign110_e520).exp();
        var_tbeta = assign110_e521;
        var_tbeta_dn3 = (assign110_e521 * (p.p77 * var_lnrt_dn3));

        let assign120_e524: f64 = (p.p52 * var_tbeta);
        let assign120_e526: f64 = (assign120_e524 * var_fbwm);
        var_bf_t = assign120_e526;
        var_bf_t_dn3 = ((p.p52 * var_tbeta_dn3) * var_fbwm);
        var_bf_t_dn4 = (assign120_e524 * var_fbwm_dn4);
        var_bf_t_dn5 = (assign120_e524 * var_fbwm_dn5);

        let assign130_e529: f64 = (p.p60 * var_tbeta);
        var_br_t = assign130_e529;
        var_br_t_dn3 = (p.p60 * var_tbeta_dn3);

        let (assign140_e537,) = {
    if (p.p53 > 0.0) {
        let assign140_e535: f64 = (1.0 / p.p53);
        (assign140_e535,)
    } else {
        (0.0,)
    }
};
        var_ovaf = assign140_e537;

        let (assign150_e545,) = {
    if (p.p62 > 0.0) {
        let assign150_e543: f64 = (1.0 / p.p62);
        (assign150_e543,)
    } else {
        (0.0,)
    }
};
        var_ovar = assign150_e545;

        let (assign160_e553,) = {
    if (p.p54 > 0.0) {
        let assign160_e551: f64 = (1.0 / p.p54);
        (assign160_e551,)
    } else {
        (0.0,)
    }
};
        var_oikf = assign160_e553;
        var_oikf_dn4 = 0.0;
        var_oikf_dn5 = 0.0;

        let (assign170_e561,) = {
    if (p.p63 > 0.0) {
        let assign170_e559: f64 = (1.0 / p.p63);
        (assign170_e559,)
    } else {
        (0.0,)
    }
};
        var_oikr = assign170_e561;

        let assign180_e564: f64 = (p.p22 * var_lnrt);
        let assign180_e568: f64 = (var_rt - 1.0);
        let assign180_e569: f64 = (p.p21 * assign180_e568);
        let assign180_e571: f64 = (assign180_e569 / var_vt);
        let assign180_e572: f64 = (assign180_e564 + assign180_e571);
        var_argt = assign180_e572;
        var_argt_dn3 = ((p.p22 * var_lnrt_dn3) + ((((p.p21 * var_rt_dn3) * var_vt) - (assign180_e569 * var_vt_dn3)) / (var_vt * var_vt)));

        let assign190_e575: f64 = (p.p23 * var_lnrt);
        var_argtr = assign190_e575;
        var_argtr_dn3 = (p.p23 * var_lnrt_dn3);

        let assign200_e578: f64 = (var_argt).exp();
        let assign200_e579: f64 = (p.p0 * assign200_e578);
        var_is_t = assign200_e579;
        var_is_t_dn3 = (p.p0 * (assign200_e578 * var_argt_dn3));

        let assign210_e582: f64 = (var_argtr).exp();
        let assign210_e583: f64 = (p.p2 * assign210_e582);
        var_isr_t = assign210_e583;
        var_isr_t_dn3 = (p.p2 * (assign210_e582 * var_argtr_dn3));

        let assign220_e587: f64 = (var_argt / p.p59);
        let assign220_e588: f64 = (assign220_e587).exp();
        let assign220_e589: f64 = (p.p58 * assign220_e588);
        let assign220_e591: f64 = (assign220_e589 / var_tbeta);
        var_ise_t = assign220_e591;
        var_ise_t_dn3 = ((((p.p58 * (assign220_e588 * (var_argt_dn3 / p.p59))) * var_tbeta) - (assign220_e589 * var_tbeta_dn3)) / (var_tbeta * var_tbeta));

        let assign230_e595: f64 = (var_argt / p.p65);
        let assign230_e596: f64 = (assign230_e595).exp();
        let assign230_e597: f64 = (p.p64 * assign230_e596);
        let assign230_e599: f64 = (assign230_e597 / var_tbeta);
        var_isc_t = assign230_e599;
        var_isc_t_dn3 = ((((p.p64 * (assign230_e596 * (var_argt_dn3 / p.p65))) * var_tbeta) - (assign230_e597 * var_tbeta_dn3)) / (var_tbeta * var_tbeta));

        let assign240_e605: f64 = (var_rt - 1.0);
        let assign240_e606: f64 = (p.p7 * assign240_e605);
        let assign240_e607: f64 = (1.0 + assign240_e606);
        let assign240_e608: f64 = (p.p47 * assign240_e607);
        var_ijbv_t = assign240_e608;
        var_ijbv_t_dn3 = (p.p47 * (p.p7 * var_rt_dn3));

        let assign250_e614: f64 = (var_rt - 1.0);
        let assign250_e615: f64 = (p.p6 * assign250_e614);
        let assign250_e616: f64 = (1.0 + assign250_e615);
        let assign250_e617: f64 = (p.p5 * assign250_e616);
        var_bvr_t = assign250_e617;
        var_bvr_t_dn3 = (p.p5 * (p.p6 * var_rt_dn3));

        let assign260_e623: f64 = (var_rt - 1.0);
        let assign260_e624: f64 = (p.p10 * assign260_e623);
        let assign260_e625: f64 = (1.0 + assign260_e624);
        let assign260_e626: f64 = (p.p9 * assign260_e625);
        var_theexp_t = assign260_e626;
        var_theexp_t_dn3 = (p.p9 * (p.p10 * var_rt_dn3));

        let assign270_e632: f64 = (var_rt - 1.0);
        let assign270_e633: f64 = (p.p55 * assign270_e632);
        let assign270_e634: f64 = (1.0 + assign270_e633);
        let assign270_e635: f64 = (p.p56 * assign270_e634);
        var_ijbvc_t = assign270_e635;
        var_ijbvc_t_dn3 = (p.p56 * (p.p55 * var_rt_dn3));

        var_cje_i = p.p16;

        var_cjc_i = p.p69;

        var_cjs_i = p.p74;

        let assign310_e641: f64 = (var_tnom / 300.15);
        var_fact1 = assign310_e641;

        let assign320_e644: f64 = (var_tdev / 300.15);
        var_fact2 = assign320_e644;
        var_fact2_dn3 = (var_tdev_dn3 / 300.15);

        let assign330_e648: f64 = (0.000702 * var_tdev);
        let assign330_e650: f64 = (assign330_e648 * var_tdev);
        let assign330_e653: f64 = (1108.0 + var_tdev);
        let assign330_e654: f64 = (assign330_e650 / assign330_e653);
        let assign330_e655: f64 = (1.16 - assign330_e654);
        var_egfet = assign330_e655;
        var_egfet_dn3 = (-((((((0.000702 * var_tdev_dn3) * var_tdev) + (assign330_e648 * var_tdev_dn3)) * assign330_e653) - (assign330_e650 * var_tdev_dn3)) / (assign330_e653 * assign330_e653)));

        let assign340_e657: f64 = (-var_egfet);
        let assign340_e661: f64 = (var_tdev + var_tdev);
        let assign340_e662: f64 = (1.3806226e-23 * assign340_e661);
        let assign340_e663: f64 = (assign340_e657 / assign340_e662);
        let assign340_e668: f64 = (300.15 + 300.15);
        let assign340_e669: f64 = (1.3806226e-23 * assign340_e668);
        let assign340_e670: f64 = (1.1150877 / assign340_e669);
        let assign340_e671: f64 = (assign340_e663 + assign340_e670);
        var_arg0 = assign340_e671;
        var_arg0_dn3 = ((((-var_egfet_dn3) * assign340_e662) - (assign340_e657 * (1.3806226e-23 * (var_tdev_dn3 + var_tdev_dn3)))) / (assign340_e662 * assign340_e662));

        let assign350_e674: f64 = (var_vt + var_vt);
        let assign350_e675: f64 = (-assign350_e674);
        let assign350_e678: f64 = (var_fact2).ln();
        let assign350_e679: f64 = (1.5 * assign350_e678);
        let assign350_e682: f64 = (1.6021918e-19 * var_arg0);
        let assign350_e683: f64 = (assign350_e679 + assign350_e682);
        let assign350_e684: f64 = (assign350_e675 * assign350_e683);
        var_pbfact = assign350_e684;
        var_pbfact_dn3 = (((-(var_vt_dn3 + var_vt_dn3)) * assign350_e683) + (assign350_e675 * ((1.5 * (var_fact2_dn3 / var_fact2)) + (1.6021918e-19 * var_arg0_dn3))));

        let assign360_e687: f64 = (p.p17 - var_pbfact);
        let assign360_e689: f64 = (assign360_e687 / var_fact1);
        var_pbo = assign360_e689;
        var_pbo_dn3 = ((-var_pbfact_dn3) / var_fact1);

        let assign370_e692: f64 = (p.p17 - var_pbo);
        let assign370_e694: f64 = (assign370_e692 / var_pbo);
        var_gmaold = assign370_e694;
        var_gmaold_dn3 = ((((-var_pbo_dn3) * var_pbo) - (assign370_e692 * var_pbo_dn3)) / (var_pbo * var_pbo));

        let assign380_e701: f64 = (var_tnom - 300.15);
        let assign380_e702: f64 = (0.0004 * assign380_e701);
        let assign380_e704: f64 = (assign380_e702 - var_gmaold);
        let assign380_e705: f64 = (p.p18 * assign380_e704);
        let assign380_e706: f64 = (1.0 + assign380_e705);
        let assign380_e707: f64 = (var_cje_i / assign380_e706);
        var_cjt = assign380_e707;
        var_cjt_dn3 = (-((var_cje_i * (p.p18 * (-var_gmaold_dn3))) / (assign380_e706 * assign380_e706)));

        let assign390_e710: f64 = (var_fact2 * var_pbo);
        let assign390_e712: f64 = (assign390_e710 + var_pbfact);
        var_vje_t = assign390_e712;
        var_vje_t_dn3 = (((var_fact2_dn3 * var_pbo) + (var_fact2 * var_pbo_dn3)) + var_pbfact_dn3);

        let assign400_e715: f64 = (var_vje_t - var_pbo);
        let assign400_e717: f64 = (assign400_e715 / var_pbo);
        var_gmanew = assign400_e717;
        var_gmanew_dn3 = ((((var_vje_t_dn3 - var_pbo_dn3) * var_pbo) - (assign400_e715 * var_pbo_dn3)) / (var_pbo * var_pbo));

        let assign410_e724: f64 = (var_tdev - 300.15);
        let assign410_e725: f64 = (0.0004 * assign410_e724);
        let assign410_e727: f64 = (assign410_e725 - var_gmanew);
        let assign410_e728: f64 = (p.p18 * assign410_e727);
        let assign410_e729: f64 = (1.0 + assign410_e728);
        let assign410_e730: f64 = (var_cjt * assign410_e729);
        var_cje_t = assign410_e730;
        var_cje_t_dn3 = ((var_cjt_dn3 * assign410_e729) + (var_cjt * (p.p18 * ((0.0004 * var_tdev_dn3) - var_gmanew_dn3))));

        let assign420_e733: f64 = (var_tnom / 300.15);
        var_fact1 = assign420_e733;

        let assign430_e736: f64 = (var_tdev / 300.15);
        var_fact2 = assign430_e736;
        var_fact2_dn3 = (var_tdev_dn3 / 300.15);

        let assign440_e740: f64 = (0.000702 * var_tdev);
        let assign440_e742: f64 = (assign440_e740 * var_tdev);
        let assign440_e745: f64 = (1108.0 + var_tdev);
        let assign440_e746: f64 = (assign440_e742 / assign440_e745);
        let assign440_e747: f64 = (1.16 - assign440_e746);
        var_egfet = assign440_e747;
        var_egfet_dn3 = (-((((((0.000702 * var_tdev_dn3) * var_tdev) + (assign440_e740 * var_tdev_dn3)) * assign440_e745) - (assign440_e742 * var_tdev_dn3)) / (assign440_e745 * assign440_e745)));

        let assign450_e749: f64 = (-var_egfet);
        let assign450_e753: f64 = (var_tdev + var_tdev);
        let assign450_e754: f64 = (1.3806226e-23 * assign450_e753);
        let assign450_e755: f64 = (assign450_e749 / assign450_e754);
        let assign450_e760: f64 = (300.15 + 300.15);
        let assign450_e761: f64 = (1.3806226e-23 * assign450_e760);
        let assign450_e762: f64 = (1.1150877 / assign450_e761);
        let assign450_e763: f64 = (assign450_e755 + assign450_e762);
        var_arg0 = assign450_e763;
        var_arg0_dn3 = ((((-var_egfet_dn3) * assign450_e754) - (assign450_e749 * (1.3806226e-23 * (var_tdev_dn3 + var_tdev_dn3)))) / (assign450_e754 * assign450_e754));

        let assign460_e766: f64 = (var_vt + var_vt);
        let assign460_e767: f64 = (-assign460_e766);
        let assign460_e770: f64 = (var_fact2).ln();
        let assign460_e771: f64 = (1.5 * assign460_e770);
        let assign460_e774: f64 = (1.6021918e-19 * var_arg0);
        let assign460_e775: f64 = (assign460_e771 + assign460_e774);
        let assign460_e776: f64 = (assign460_e767 * assign460_e775);
        var_pbfact = assign460_e776;
        var_pbfact_dn3 = (((-(var_vt_dn3 + var_vt_dn3)) * assign460_e775) + (assign460_e767 * ((1.5 * (var_fact2_dn3 / var_fact2)) + (1.6021918e-19 * var_arg0_dn3))));

        let assign470_e779: f64 = (p.p70 - var_pbfact);
        let assign470_e781: f64 = (assign470_e779 / var_fact1);
        var_pbo = assign470_e781;
        var_pbo_dn3 = ((-var_pbfact_dn3) / var_fact1);

        let assign480_e784: f64 = (p.p70 - var_pbo);
        let assign480_e786: f64 = (assign480_e784 / var_pbo);
        var_gmaold = assign480_e786;
        var_gmaold_dn3 = ((((-var_pbo_dn3) * var_pbo) - (assign480_e784 * var_pbo_dn3)) / (var_pbo * var_pbo));

        let assign490_e793: f64 = (var_tnom - 300.15);
        let assign490_e794: f64 = (0.0004 * assign490_e793);
        let assign490_e796: f64 = (assign490_e794 - var_gmaold);
        let assign490_e797: f64 = (p.p71 * assign490_e796);
        let assign490_e798: f64 = (1.0 + assign490_e797);
        let assign490_e799: f64 = (var_cjc_i / assign490_e798);
        var_cjt = assign490_e799;
        var_cjt_dn3 = (-((var_cjc_i * (p.p71 * (-var_gmaold_dn3))) / (assign490_e798 * assign490_e798)));

        let assign500_e802: f64 = (var_fact2 * var_pbo);
        let assign500_e804: f64 = (assign500_e802 + var_pbfact);
        var_vjc_t = assign500_e804;
        var_vjc_t_dn3 = (((var_fact2_dn3 * var_pbo) + (var_fact2 * var_pbo_dn3)) + var_pbfact_dn3);

        let assign510_e807: f64 = (var_vjc_t - var_pbo);
        let assign510_e809: f64 = (assign510_e807 / var_pbo);
        var_gmanew = assign510_e809;
        var_gmanew_dn3 = ((((var_vjc_t_dn3 - var_pbo_dn3) * var_pbo) - (assign510_e807 * var_pbo_dn3)) / (var_pbo * var_pbo));

        let assign520_e816: f64 = (var_tdev - 300.15);
        let assign520_e817: f64 = (0.0004 * assign520_e816);
        let assign520_e819: f64 = (assign520_e817 - var_gmanew);
        let assign520_e820: f64 = (p.p71 * assign520_e819);
        let assign520_e821: f64 = (1.0 + assign520_e820);
        let assign520_e822: f64 = (var_cjt * assign520_e821);
        var_cjc_t = assign520_e822;
        var_cjc_t_dn3 = ((var_cjt_dn3 * assign520_e821) + (var_cjt * (p.p71 * ((0.0004 * var_tdev_dn3) - var_gmanew_dn3))));

        let assign530_e825: f64 = (var_tnom / 300.15);
        var_fact1 = assign530_e825;

        let assign540_e828: f64 = (var_tdev / 300.15);
        var_fact2 = assign540_e828;
        var_fact2_dn3 = (var_tdev_dn3 / 300.15);

        let assign550_e832: f64 = (0.000702 * var_tdev);
        let assign550_e834: f64 = (assign550_e832 * var_tdev);
        let assign550_e837: f64 = (1108.0 + var_tdev);
        let assign550_e838: f64 = (assign550_e834 / assign550_e837);
        let assign550_e839: f64 = (1.16 - assign550_e838);
        var_egfet = assign550_e839;
        var_egfet_dn3 = (-((((((0.000702 * var_tdev_dn3) * var_tdev) + (assign550_e832 * var_tdev_dn3)) * assign550_e837) - (assign550_e834 * var_tdev_dn3)) / (assign550_e837 * assign550_e837)));

        let assign560_e841: f64 = (-var_egfet);
        let assign560_e845: f64 = (var_tdev + var_tdev);
        let assign560_e846: f64 = (1.3806226e-23 * assign560_e845);
        let assign560_e847: f64 = (assign560_e841 / assign560_e846);
        let assign560_e852: f64 = (300.15 + 300.15);
        let assign560_e853: f64 = (1.3806226e-23 * assign560_e852);
        let assign560_e854: f64 = (1.1150877 / assign560_e853);
        let assign560_e855: f64 = (assign560_e847 + assign560_e854);
        var_arg0 = assign560_e855;
        var_arg0_dn3 = ((((-var_egfet_dn3) * assign560_e846) - (assign560_e841 * (1.3806226e-23 * (var_tdev_dn3 + var_tdev_dn3)))) / (assign560_e846 * assign560_e846));

        let assign570_e858: f64 = (var_vt + var_vt);
        let assign570_e859: f64 = (-assign570_e858);
        let assign570_e862: f64 = (var_fact2).ln();
        let assign570_e863: f64 = (1.5 * assign570_e862);
        let assign570_e866: f64 = (1.6021918e-19 * var_arg0);
        let assign570_e867: f64 = (assign570_e863 + assign570_e866);
        let assign570_e868: f64 = (assign570_e859 * assign570_e867);
        var_pbfact = assign570_e868;
        var_pbfact_dn3 = (((-(var_vt_dn3 + var_vt_dn3)) * assign570_e867) + (assign570_e859 * ((1.5 * (var_fact2_dn3 / var_fact2)) + (1.6021918e-19 * var_arg0_dn3))));

        let assign580_e871: f64 = (p.p75 - var_pbfact);
        let assign580_e873: f64 = (assign580_e871 / var_fact1);
        var_pbo = assign580_e873;
        var_pbo_dn3 = ((-var_pbfact_dn3) / var_fact1);

        let assign590_e876: f64 = (p.p75 - var_pbo);
        let assign590_e878: f64 = (assign590_e876 / var_pbo);
        var_gmaold = assign590_e878;
        var_gmaold_dn3 = ((((-var_pbo_dn3) * var_pbo) - (assign590_e876 * var_pbo_dn3)) / (var_pbo * var_pbo));

        let assign600_e885: f64 = (var_tnom - 300.15);
        let assign600_e886: f64 = (0.0004 * assign600_e885);
        let assign600_e888: f64 = (assign600_e886 - var_gmaold);
        let assign600_e889: f64 = (p.p76 * assign600_e888);
        let assign600_e890: f64 = (1.0 + assign600_e889);
        let assign600_e891: f64 = (var_cjs_i / assign600_e890);
        var_cjt = assign600_e891;
        var_cjt_dn3 = (-((var_cjs_i * (p.p76 * (-var_gmaold_dn3))) / (assign600_e890 * assign600_e890)));

        let assign610_e894: f64 = (var_fact2 * var_pbo);
        let assign610_e896: f64 = (assign610_e894 + var_pbfact);
        var_vjs_t = assign610_e896;
        var_vjs_t_dn3 = (((var_fact2_dn3 * var_pbo) + (var_fact2 * var_pbo_dn3)) + var_pbfact_dn3);

        let assign620_e899: f64 = (var_vjs_t - var_pbo);
        let assign620_e901: f64 = (assign620_e899 / var_pbo);
        var_gmanew = assign620_e901;
        var_gmanew_dn3 = ((((var_vjs_t_dn3 - var_pbo_dn3) * var_pbo) - (assign620_e899 * var_pbo_dn3)) / (var_pbo * var_pbo));

        let assign630_e908: f64 = (var_tdev - 300.15);
        let assign630_e909: f64 = (0.0004 * assign630_e908);
        let assign630_e911: f64 = (assign630_e909 - var_gmanew);
        let assign630_e912: f64 = (p.p76 * assign630_e911);
        let assign630_e913: f64 = (1.0 + assign630_e912);
        let assign630_e914: f64 = (var_cjt * assign630_e913);
        var_cjs_t = assign630_e914;
        var_cjs_t_dn3 = ((var_cjt_dn3 * assign630_e913) + (var_cjt * (p.p76 * ((0.0004 * var_tdev_dn3) - var_gmanew_dn3))));

        var_ttype = p.p29;

        let assign650_e918: f64 = (var_ttype * (nv2 - nv4));
        var_veci = assign650_e918;
        var_veci_dn2 = var_ttype;
        var_veci_dn4 = (-var_ttype);

        let assign660_e921: f64 = (var_ttype * (nv5 - nv6));
        var_vbiei = assign660_e921;
        var_vbiei_dn5 = var_ttype;
        var_vbiei_dn6 = (-var_ttype);

        let assign670_e924: f64 = (var_ttype * (nv5 - nv4));
        var_vbici = assign670_e924;
        var_vbici_dn4 = (-var_ttype);
        var_vbici_dn5 = var_ttype;

        let assign680_e927: f64 = (var_ttype * (nv1 - nv4));
        var_vbci = assign680_e927;
        var_vbci_dn1 = var_ttype;
        var_vbci_dn4 = (-var_ttype);

        let assign690_e930: f64 = (var_ttype * (nv1 - nv5));
        var_vbbi = assign690_e930;
        var_vbbi_dn1 = var_ttype;
        var_vbbi_dn5 = (-var_ttype);

        let assign700_e933: f64 = (var_ttype * (nv2 - nv6));
        var_veei = assign700_e933;
        var_veei_dn2 = var_ttype;
        var_veei_dn6 = (-var_ttype);

        let assign710_e936: f64 = if var_is_t > 0.0 { 1.0 } else { 0.0 };
        var_guard3 = assign710_e936;

        let (assign720_e944, assign720_e944_d_n3, assign720_e944_d_n4, assign720_e944_d_n5, assign720_e944_d_n6,) = {
    if (var_guard3 != 0.0) {
        let assign720_e941: f64 = (p.p1 * var_vt);
        let assign720_e942: f64 = (var_vbiei / assign720_e941);
        (assign720_e942, (-((var_vbiei * (p.p1 * var_vt_dn3)) / (assign720_e941 * assign720_e941))), 0.0, (var_vbiei_dn5 / assign720_e941), (var_vbiei_dn6 / assign720_e941),)
    } else {
        (var_arg, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6,)
    }
};
        var_arg = assign720_e944;
        var_arg_dn3 = assign720_e944_d_n3;
        var_arg_dn4 = assign720_e944_d_n4;
        var_arg_dn5 = assign720_e944_d_n5;
        var_arg_dn6 = assign720_e944_d_n6;

        let (assign730_e955, assign730_e955_d_n3, assign730_e955_d_n4, assign730_e955_d_n5, assign730_e955_d_n6,) = {
    if (var_guard3 != 0.0) {
        let assign730_e947: f64 = (-var_vbiei);
        let assign730_e949: f64 = (assign730_e947 - var_bvr_t);
        let assign730_e952: f64 = (p.p11 * var_vt);
        let assign730_e953: f64 = (assign730_e949 / assign730_e952);
        (assign730_e953, ((((-var_bvr_t_dn3) * assign730_e952) - (assign730_e949 * (p.p11 * var_vt_dn3))) / (assign730_e952 * assign730_e952)), 0.0, ((-var_vbiei_dn5) / assign730_e952), ((-var_vbiei_dn6) / assign730_e952),)
    } else {
        (var_argbv, var_argbv_dn3, var_argbv_dn4, var_argbv_dn5, var_argbv_dn6,)
    }
};
        var_argbv = assign730_e955;
        var_argbv_dn3 = assign730_e955_d_n3;
        var_argbv_dn4 = assign730_e955_d_n4;
        var_argbv_dn5 = assign730_e955_d_n5;
        var_argbv_dn6 = assign730_e955_d_n6;

        let (assign740_e964, assign740_e964_d_n3,) = {
    if (var_guard3 != 0.0) {
        let assign740_e958: f64 = (-var_bvr_t);
        let assign740_e961: f64 = (p.p11 * var_vt);
        let assign740_e962: f64 = (assign740_e958 / assign740_e961);
        (assign740_e962, ((((-var_bvr_t_dn3) * assign740_e961) - (assign740_e958 * (p.p11 * var_vt_dn3))) / (assign740_e961 * assign740_e961)),)
    } else {
        (var_argbvvt, var_argbvvt_dn3,)
    }
};
        var_argbvvt = assign740_e964;
        var_argbvvt_dn3 = assign740_e964_d_n3;

        let assign750_e967: f64 = if var_arg > 80.0 { 1.0 } else { 0.0 };
        var_guard4 = assign750_e967;

        *var_arg_slot = var_arg;
        *var_arg0_slot = var_arg0;
        *var_arg0_dn3_slot = var_arg0_dn3;
        *var_arg_dn3_slot = var_arg_dn3;
        *var_arg_dn4_slot = var_arg_dn4;
        *var_arg_dn5_slot = var_arg_dn5;
        *var_arg_dn6_slot = var_arg_dn6;
        *var_argbv_slot = var_argbv;
        *var_argbv_dn3_slot = var_argbv_dn3;
        *var_argbv_dn4_slot = var_argbv_dn4;
        *var_argbv_dn5_slot = var_argbv_dn5;
        *var_argbv_dn6_slot = var_argbv_dn6;
        *var_argbvvt_slot = var_argbvvt;
        *var_argbvvt_dn3_slot = var_argbvvt_dn3;
        *var_argt_slot = var_argt;
        *var_argt_dn3_slot = var_argt_dn3;
        *var_argtr_slot = var_argtr;
        *var_argtr_dn3_slot = var_argtr_dn3;
        *var_bf_t_slot = var_bf_t;
        *var_bf_t_dn3_slot = var_bf_t_dn3;
        *var_bf_t_dn4_slot = var_bf_t_dn4;
        *var_bf_t_dn5_slot = var_bf_t_dn5;
        *var_br_t_slot = var_br_t;
        *var_br_t_dn3_slot = var_br_t_dn3;
        *var_bvr_t_slot = var_bvr_t;
        *var_bvr_t_dn3_slot = var_bvr_t_dn3;
        *var_cjc_i_slot = var_cjc_i;
        *var_cjc_t_slot = var_cjc_t;
        *var_cjc_t_dn3_slot = var_cjc_t_dn3;
        *var_cje_i_slot = var_cje_i;
        *var_cje_t_slot = var_cje_t;
        *var_cje_t_dn3_slot = var_cje_t_dn3;
        *var_cjs_i_slot = var_cjs_i;
        *var_cjs_t_slot = var_cjs_t;
        *var_cjs_t_dn3_slot = var_cjs_t_dn3;
        *var_cjt_slot = var_cjt;
        *var_cjt_dn3_slot = var_cjt_dn3;
        *var_egfet_slot = var_egfet;
        *var_egfet_dn3_slot = var_egfet_dn3;
        *var_fact1_slot = var_fact1;
        *var_fact2_slot = var_fact2;
        *var_fact2_dn3_slot = var_fact2_dn3;
        *var_fbwm_slot = var_fbwm;
        *var_fbwm_dn4_slot = var_fbwm_dn4;
        *var_fbwm_dn5_slot = var_fbwm_dn5;
        *var_gmanew_slot = var_gmanew;
        *var_gmanew_dn3_slot = var_gmanew_dn3;
        *var_gmaold_slot = var_gmaold;
        *var_gmaold_dn3_slot = var_gmaold_dn3;
        *var_guard3_slot = var_guard3;
        *var_guard4_slot = var_guard4;
        *var_ijbv_t_slot = var_ijbv_t;
        *var_ijbv_t_dn3_slot = var_ijbv_t_dn3;
        *var_ijbvc_t_slot = var_ijbvc_t;
        *var_ijbvc_t_dn3_slot = var_ijbvc_t_dn3;
        *var_is_t_slot = var_is_t;
        *var_is_t_dn3_slot = var_is_t_dn3;
        *var_isc_t_slot = var_isc_t;
        *var_isc_t_dn3_slot = var_isc_t_dn3;
        *var_ise_t_slot = var_ise_t;
        *var_ise_t_dn3_slot = var_ise_t_dn3;
        *var_isr_t_slot = var_isr_t;
        *var_isr_t_dn3_slot = var_isr_t_dn3;
        *var_lnrt_slot = var_lnrt;
        *var_lnrt_dn3_slot = var_lnrt_dn3;
        *var_oikf_slot = var_oikf;
        *var_oikf_dn4_slot = var_oikf_dn4;
        *var_oikf_dn5_slot = var_oikf_dn5;
        *var_oikr_slot = var_oikr;
        *var_ovaf_slot = var_ovaf;
        *var_ovar_slot = var_ovar;
        *var_pbfact_slot = var_pbfact;
        *var_pbfact_dn3_slot = var_pbfact_dn3;
        *var_pbo_slot = var_pbo;
        *var_pbo_dn3_slot = var_pbo_dn3;
        *var_rt_slot = var_rt;
        *var_rt_dn3_slot = var_rt_dn3;
        *var_tamb_slot = var_tamb;
        *var_tamb_dn3_slot = var_tamb_dn3;
        *var_tbeta_slot = var_tbeta;
        *var_tbeta_dn3_slot = var_tbeta_dn3;
        *var_tdev_slot = var_tdev;
        *var_tdev_dn3_slot = var_tdev_dn3;
        *var_theexp_t_slot = var_theexp_t;
        *var_theexp_t_dn3_slot = var_theexp_t_dn3;
        *var_tnom_slot = var_tnom;
        *var_ttype_slot = var_ttype;
        *var_vbbi_slot = var_vbbi;
        *var_vbbi_dn1_slot = var_vbbi_dn1;
        *var_vbbi_dn5_slot = var_vbbi_dn5;
        *var_vbc_slot = var_vbc;
        *var_vbc_dn4_slot = var_vbc_dn4;
        *var_vbc_dn5_slot = var_vbc_dn5;
        *var_vbci_slot = var_vbci;
        *var_vbci_dn1_slot = var_vbci_dn1;
        *var_vbci_dn4_slot = var_vbci_dn4;
        *var_vbici_slot = var_vbici;
        *var_vbici_dn4_slot = var_vbici_dn4;
        *var_vbici_dn5_slot = var_vbici_dn5;
        *var_vbiei_slot = var_vbiei;
        *var_vbiei_dn5_slot = var_vbiei_dn5;
        *var_vbiei_dn6_slot = var_vbiei_dn6;
        *var_veci_slot = var_veci;
        *var_veci_dn2_slot = var_veci_dn2;
        *var_veci_dn4_slot = var_veci_dn4;
        *var_veei_slot = var_veei;
        *var_veei_dn2_slot = var_veei_dn2;
        *var_veei_dn6_slot = var_veei_dn6;
        *var_vjc_t_slot = var_vjc_t;
        *var_vjc_t_dn3_slot = var_vjc_t_dn3;
        *var_vje_t_slot = var_vje_t;
        *var_vje_t_dn3_slot = var_vje_t_dn3;
        *var_vjs_t_slot = var_vjs_t;
        *var_vjs_t_dn3_slot = var_vjs_t_dn3;
        *var_vt_slot = var_vt;
        *var_vt_dn3_slot = var_vt_dn3;
        *var_weff_slot = var_weff;
    }

    pub(super) fn stamp_transient_block_1(
        p: &Parameters,
        var_bvr_t: f64,
        var_bvr_t_dn3: f64,
        var_guard3: f64,
        var_guard4: f64,
        var_ijbv_t: f64,
        var_ijbv_t_dn3: f64,
        var_is_t: f64,
        var_is_t_dn3: f64,
        var_ise_t: f64,
        var_ise_t_dn3: f64,
        var_isr_t: f64,
        var_isr_t_dn3: f64,
        var_theexp_t: f64,
        var_theexp_t_dn3: f64,
        var_vbici: f64,
        var_vbici_dn4: f64,
        var_vbici_dn5: f64,
        var_vbiei: f64,
        var_vbiei_dn5: f64,
        var_vbiei_dn6: f64,
        var_vt: f64,
        var_vt_dn3: f64,
        var_arg_slot: &mut f64,
        var_arg_dn3_slot: &mut f64,
        var_arg_dn4_slot: &mut f64,
        var_arg_dn5_slot: &mut f64,
        var_arg_dn6_slot: &mut f64,
        var_argbv_slot: &mut f64,
        var_argbv_dn3_slot: &mut f64,
        var_argbv_dn4_slot: &mut f64,
        var_argbv_dn5_slot: &mut f64,
        var_argbv_dn6_slot: &mut f64,
        var_argbvvt_slot: &mut f64,
        var_argbvvt_dn3_slot: &mut f64,
        var_guard5_slot: &mut f64,
        var_guard6_slot: &mut f64,
        var_guard7_slot: &mut f64,
        var_guard8_slot: &mut f64,
        var_guard9_slot: &mut f64,
        var_ibe2_slot: &mut f64,
        var_ibe2_dn3_slot: &mut f64,
        var_ibe2_dn4_slot: &mut f64,
        var_ibe2_dn5_slot: &mut f64,
        var_ibe2_dn6_slot: &mut f64,
        var_ifwd_slot: &mut f64,
        var_ifwd_dn3_slot: &mut f64,
        var_ifwd_dn4_slot: &mut f64,
        var_ifwd_dn5_slot: &mut f64,
        var_ifwd_dn6_slot: &mut f64,
        var_itrev_slot: &mut f64,
        var_itrev_dn3_slot: &mut f64,
        var_itrev_dn4_slot: &mut f64,
        var_itrev_dn5_slot: &mut f64,
        var_itrev_dn6_slot: &mut f64,
        var_le_slot: &mut f64,
        var_le_dn3_slot: &mut f64,
        var_le_dn4_slot: &mut f64,
        var_le_dn5_slot: &mut f64,
        var_le_dn6_slot: &mut f64,
        var_lebv_slot: &mut f64,
        var_lebv_dn3_slot: &mut f64,
        var_lebv_dn4_slot: &mut f64,
        var_lebv_dn5_slot: &mut f64,
        var_lebv_dn6_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
    ) {
        let mut var_arg: f64 = *var_arg_slot;
        let mut var_arg_dn3: f64 = *var_arg_dn3_slot;
        let mut var_arg_dn4: f64 = *var_arg_dn4_slot;
        let mut var_arg_dn5: f64 = *var_arg_dn5_slot;
        let mut var_arg_dn6: f64 = *var_arg_dn6_slot;
        let mut var_argbv: f64 = *var_argbv_slot;
        let mut var_argbv_dn3: f64 = *var_argbv_dn3_slot;
        let mut var_argbv_dn4: f64 = *var_argbv_dn4_slot;
        let mut var_argbv_dn5: f64 = *var_argbv_dn5_slot;
        let mut var_argbv_dn6: f64 = *var_argbv_dn6_slot;
        let mut var_argbvvt: f64 = *var_argbvvt_slot;
        let mut var_argbvvt_dn3: f64 = *var_argbvvt_dn3_slot;
        let mut var_guard5: f64 = *var_guard5_slot;
        let mut var_guard6: f64 = *var_guard6_slot;
        let mut var_guard7: f64 = *var_guard7_slot;
        let mut var_guard8: f64 = *var_guard8_slot;
        let mut var_guard9: f64 = *var_guard9_slot;
        let mut var_ibe2: f64 = *var_ibe2_slot;
        let mut var_ibe2_dn3: f64 = *var_ibe2_dn3_slot;
        let mut var_ibe2_dn4: f64 = *var_ibe2_dn4_slot;
        let mut var_ibe2_dn5: f64 = *var_ibe2_dn5_slot;
        let mut var_ibe2_dn6: f64 = *var_ibe2_dn6_slot;
        let mut var_ifwd: f64 = *var_ifwd_slot;
        let mut var_ifwd_dn3: f64 = *var_ifwd_dn3_slot;
        let mut var_ifwd_dn4: f64 = *var_ifwd_dn4_slot;
        let mut var_ifwd_dn5: f64 = *var_ifwd_dn5_slot;
        let mut var_ifwd_dn6: f64 = *var_ifwd_dn6_slot;
        let mut var_itrev: f64 = *var_itrev_slot;
        let mut var_itrev_dn3: f64 = *var_itrev_dn3_slot;
        let mut var_itrev_dn4: f64 = *var_itrev_dn4_slot;
        let mut var_itrev_dn5: f64 = *var_itrev_dn5_slot;
        let mut var_itrev_dn6: f64 = *var_itrev_dn6_slot;
        let mut var_le: f64 = *var_le_slot;
        let mut var_le_dn3: f64 = *var_le_dn3_slot;
        let mut var_le_dn4: f64 = *var_le_dn4_slot;
        let mut var_le_dn5: f64 = *var_le_dn5_slot;
        let mut var_le_dn6: f64 = *var_le_dn6_slot;
        let mut var_lebv: f64 = *var_lebv_slot;
        let mut var_lebv_dn3: f64 = *var_lebv_dn3_slot;
        let mut var_lebv_dn4: f64 = *var_lebv_dn4_slot;
        let mut var_lebv_dn5: f64 = *var_lebv_dn5_slot;
        let mut var_lebv_dn6: f64 = *var_lebv_dn6_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;

        let (assign760_e977, assign760_e977_d_n3, assign760_e977_d_n4, assign760_e977_d_n5, assign760_e977_d_n6,) = {
    if ((var_guard3 != 0.0) && (var_guard4 != 0.0)) {
        let assign760_e974: f64 = (var_arg - 80.0);
        let assign760_e975: f64 = (1.0 + assign760_e974);
        (assign760_e975, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6,)
    } else {
        (var_le, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6,)
    }
};
        var_le = assign760_e977;
        var_le_dn3 = assign760_e977_d_n3;
        var_le_dn4 = assign760_e977_d_n4;
        var_le_dn5 = assign760_e977_d_n5;
        var_le_dn6 = assign760_e977_d_n6;

        let (assign770_e983, assign770_e983_d_n3, assign770_e983_d_n4, assign770_e983_d_n5, assign770_e983_d_n6,) = {
    if ((var_guard3 != 0.0) && (var_guard4 != 0.0)) {
        (80.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6,)
    }
};
        var_arg = assign770_e983;
        var_arg_dn3 = assign770_e983_d_n3;
        var_arg_dn4 = assign770_e983_d_n4;
        var_arg_dn5 = assign770_e983_d_n5;
        var_arg_dn6 = assign770_e983_d_n6;

        let (assign780_e990, assign780_e990_d_n3, assign780_e990_d_n4, assign780_e990_d_n5, assign780_e990_d_n6,) = {
    if ((var_guard3 != 0.0) && (var_guard4 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_le, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6,)
    }
};
        var_le = assign780_e990;
        var_le_dn3 = assign780_e990_d_n3;
        var_le_dn4 = assign780_e990_d_n4;
        var_le_dn5 = assign780_e990_d_n5;
        var_le_dn6 = assign780_e990_d_n6;

        let (assign790_e997, assign790_e997_d_n3, assign790_e997_d_n4, assign790_e997_d_n5, assign790_e997_d_n6,) = {
    if (var_guard3 != 0.0) {
        let assign790_e994: f64 = (var_arg).exp();
        let assign790_e995: f64 = (var_le * assign790_e994);
        (assign790_e995, ((var_le_dn3 * assign790_e994) + (var_le * (assign790_e994 * var_arg_dn3))), ((var_le_dn4 * assign790_e994) + (var_le * (assign790_e994 * var_arg_dn4))), ((var_le_dn5 * assign790_e994) + (var_le * (assign790_e994 * var_arg_dn5))), ((var_le_dn6 * assign790_e994) + (var_le * (assign790_e994 * var_arg_dn6))),)
    } else {
        (var_le, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6,)
    }
};
        var_le = assign790_e997;
        var_le_dn3 = assign790_e997_d_n3;
        var_le_dn4 = assign790_e997_d_n4;
        var_le_dn5 = assign790_e997_d_n5;
        var_le_dn6 = assign790_e997_d_n6;

        let (assign800_e1069, assign800_e1069_d_n3, assign800_e1069_d_n4, assign800_e1069_d_n5, assign800_e1069_d_n6,) = {
    if (var_guard3 != 0.0) {
        let assign800_e1005: f64 = (-37.0);
        let (assign800_e1032, assign800_e1032_d_n3, assign800_e1032_d_n4, assign800_e1032_d_n5, assign800_e1032_d_n6,) = {
            if ((!(var_argbv >= 37.0)) && (!(var_argbv <= assign800_e1005))) {
                let assign800_e1010: f64 = (var_argbv).exp();
                let assign800_e1012: f64 = (assign800_e1010 + 1.0);
                let assign800_e1013: f64 = (assign800_e1012).ln();
                (assign800_e1013, ((assign800_e1010 * var_argbv_dn3) / assign800_e1012), ((assign800_e1010 * var_argbv_dn4) / assign800_e1012), ((assign800_e1010 * var_argbv_dn5) / assign800_e1012), ((assign800_e1010 * var_argbv_dn6) / assign800_e1012),)
            } else {
                let assign800_e1020: f64 = (-37.0);
                let (assign800_e1031, assign800_e1031_d_n3, assign800_e1031_d_n4, assign800_e1031_d_n5, assign800_e1031_d_n6,) = {
                    if ((!(var_argbv >= 37.0)) && (var_argbv <= assign800_e1020)) {
                        let assign800_e1024: f64 = (var_argbv).exp();
                        (assign800_e1024, (assign800_e1024 * var_argbv_dn3), (assign800_e1024 * var_argbv_dn4), (assign800_e1024 * var_argbv_dn5), (assign800_e1024 * var_argbv_dn6),)
                    } else {
                        let (assign800_e1030, assign800_e1030_d_n3, assign800_e1030_d_n4, assign800_e1030_d_n5, assign800_e1030_d_n6,) = {
                            if (var_argbv >= 37.0) {
                                (var_argbv, var_argbv_dn3, var_argbv_dn4, var_argbv_dn5, var_argbv_dn6,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign800_e1030, assign800_e1030_d_n3, assign800_e1030_d_n4, assign800_e1030_d_n5, assign800_e1030_d_n6,)
                    }
                };
                (assign800_e1031, assign800_e1031_d_n3, assign800_e1031_d_n4, assign800_e1031_d_n5, assign800_e1031_d_n6,)
            }
        };
        let assign800_e1039: f64 = (-37.0);
        let (assign800_e1066, assign800_e1066_d_n3,) = {
            if ((!(var_argbvvt >= 37.0)) && (!(var_argbvvt <= assign800_e1039))) {
                let assign800_e1044: f64 = (var_argbvvt).exp();
                let assign800_e1046: f64 = (assign800_e1044 + 1.0);
                let assign800_e1047: f64 = (assign800_e1046).ln();
                (assign800_e1047, ((assign800_e1044 * var_argbvvt_dn3) / assign800_e1046),)
            } else {
                let assign800_e1054: f64 = (-37.0);
                let (assign800_e1065, assign800_e1065_d_n3,) = {
                    if ((!(var_argbvvt >= 37.0)) && (var_argbvvt <= assign800_e1054)) {
                        let assign800_e1058: f64 = (var_argbvvt).exp();
                        (assign800_e1058, (assign800_e1058 * var_argbvvt_dn3),)
                    } else {
                        let (assign800_e1064, assign800_e1064_d_n3,) = {
                            if (var_argbvvt >= 37.0) {
                                (var_argbvvt, var_argbvvt_dn3,)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign800_e1064, assign800_e1064_d_n3,)
                    }
                };
                (assign800_e1065, assign800_e1065_d_n3,)
            }
        };
        let assign800_e1067: f64 = (assign800_e1032 - assign800_e1066);
        (assign800_e1067, (assign800_e1032_d_n3 - assign800_e1066_d_n3), assign800_e1032_d_n4, assign800_e1032_d_n5, assign800_e1032_d_n6,)
    } else {
        (var_lebv, var_lebv_dn3, var_lebv_dn4, var_lebv_dn5, var_lebv_dn6,)
    }
};
        var_lebv = assign800_e1069;
        var_lebv_dn3 = assign800_e1069_d_n3;
        var_lebv_dn4 = assign800_e1069_d_n4;
        var_lebv_dn5 = assign800_e1069_d_n5;
        var_lebv_dn6 = assign800_e1069_d_n6;

        let (assign810_e1090, assign810_e1090_d_n3, assign810_e1090_d_n4, assign810_e1090_d_n5, assign810_e1090_d_n6,) = {
    if (var_guard3 != 0.0) {
        let assign810_e1074: f64 = (var_le - 1.0);
        let assign810_e1075: f64 = (var_is_t * assign810_e1074);
        let assign810_e1078: f64 = (var_ijbv_t * var_lebv);
        let assign810_e1082: f64 = (var_vbiei).abs();
        let assign810_e1084: f64 = (assign810_e1082).powf(var_theexp_t);
        let assign810_e1085: f64 = (p.p8 * assign810_e1084);
        let assign810_e1086: f64 = (1.0 + assign810_e1085);
        let assign810_e1087: f64 = (assign810_e1078 / assign810_e1086);
        let assign810_e1088: f64 = (assign810_e1075 - assign810_e1087);
        (assign810_e1088, (((var_is_t_dn3 * assign810_e1074) + (var_is_t * var_le_dn3)) - (((((var_ijbv_t_dn3 * var_lebv) + (var_ijbv_t * var_lebv_dn3)) * assign810_e1086) - (assign810_e1078 * (p.p8 * if var_theexp_t_dn3 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { 0.0 } else { (assign810_e1084 * (var_theexp_t_dn3 * (assign810_e1082).ln())) }))) / (assign810_e1086 * assign810_e1086))), ((var_is_t * var_le_dn4) - ((var_ijbv_t * var_lebv_dn4) / assign810_e1086)), ((var_is_t * var_le_dn5) - ((((var_ijbv_t * var_lebv_dn5) * assign810_e1086) - (assign810_e1078 * (p.p8 * if 0.0 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign810_e1082).powf(var_theexp_t - 1.0) * if var_vbiei >= 0.0 { var_vbiei_dn5 } else { (-var_vbiei_dn5) })) } } else { (assign810_e1084 * (var_theexp_t * (if var_vbiei >= 0.0 { var_vbiei_dn5 } else { (-var_vbiei_dn5) } / assign810_e1082))) }))) / (assign810_e1086 * assign810_e1086))), ((var_is_t * var_le_dn6) - ((((var_ijbv_t * var_lebv_dn6) * assign810_e1086) - (assign810_e1078 * (p.p8 * if 0.0 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign810_e1082).powf(var_theexp_t - 1.0) * if var_vbiei >= 0.0 { var_vbiei_dn6 } else { (-var_vbiei_dn6) })) } } else { (assign810_e1084 * (var_theexp_t * (if var_vbiei >= 0.0 { var_vbiei_dn6 } else { (-var_vbiei_dn6) } / assign810_e1082))) }))) / (assign810_e1086 * assign810_e1086))),)
    } else {
        (var_ifwd, var_ifwd_dn3, var_ifwd_dn4, var_ifwd_dn5, var_ifwd_dn6,)
    }
};
        var_ifwd = assign810_e1090;
        var_ifwd_dn3 = assign810_e1090_d_n3;
        var_ifwd_dn4 = assign810_e1090_d_n4;
        var_ifwd_dn5 = assign810_e1090_d_n5;
        var_ifwd_dn6 = assign810_e1090_d_n6;

        let (assign820_e1095, assign820_e1095_d_n3, assign820_e1095_d_n4, assign820_e1095_d_n5, assign820_e1095_d_n6,) = {
    if (var_guard3 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ifwd, var_ifwd_dn3, var_ifwd_dn4, var_ifwd_dn5, var_ifwd_dn6,)
    }
};
        var_ifwd = assign820_e1095;
        var_ifwd_dn3 = assign820_e1095_d_n3;
        var_ifwd_dn4 = assign820_e1095_d_n4;
        var_ifwd_dn5 = assign820_e1095_d_n5;
        var_ifwd_dn6 = assign820_e1095_d_n6;

        let assign830_e1098: f64 = if var_isr_t > 0.0 { 1.0 } else { 0.0 };
        var_guard5 = assign830_e1098;

        let (assign840_e1106, assign840_e1106_d_n5, assign840_e1106_d_n6,) = {
    if (var_guard5 != 0.0) {
        let assign840_e1102: f64 = (p.p4 - var_vbiei);
        let assign840_e1104: f64 = (assign840_e1102).max(0.001);
        (assign840_e1104, if assign840_e1102 >= 0.001 { (-var_vbiei_dn5) } else { 0.0 }, if assign840_e1102 >= 0.001 { (-var_vbiei_dn6) } else { 0.0 },)
    } else {
        (var_t0, var_t0_dn5, var_t0_dn6,)
    }
};
        var_t0 = assign840_e1106;
        var_t0_dn5 = assign840_e1106_d_n5;
        var_t0_dn6 = assign840_e1106_d_n6;

        let (assign850_e1121, assign850_e1121_d_n3, assign850_e1121_d_n4, assign850_e1121_d_n5, assign850_e1121_d_n6,) = {
    if (var_guard5 != 0.0) {
        let assign850_e1109: f64 = (-1.0);
        let assign850_e1111: f64 = (assign850_e1109 * var_vbiei);
        let assign850_e1113: f64 = (assign850_e1111 * p.p4);
        let assign850_e1116: f64 = (p.p3 * var_vt);
        let assign850_e1118: f64 = (assign850_e1116 * var_t0);
        let assign850_e1119: f64 = (assign850_e1113 / assign850_e1118);
        (assign850_e1119, (-((assign850_e1113 * ((p.p3 * var_vt_dn3) * var_t0)) / (assign850_e1118 * assign850_e1118))), 0.0, (((((assign850_e1109 * var_vbiei_dn5) * p.p4) * assign850_e1118) - (assign850_e1113 * (assign850_e1116 * var_t0_dn5))) / (assign850_e1118 * assign850_e1118)), (((((assign850_e1109 * var_vbiei_dn6) * p.p4) * assign850_e1118) - (assign850_e1113 * (assign850_e1116 * var_t0_dn6))) / (assign850_e1118 * assign850_e1118)),)
    } else {
        (var_arg, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6,)
    }
};
        var_arg = assign850_e1121;
        var_arg_dn3 = assign850_e1121_d_n3;
        var_arg_dn4 = assign850_e1121_d_n4;
        var_arg_dn5 = assign850_e1121_d_n5;
        var_arg_dn6 = assign850_e1121_d_n6;

        let assign860_e1124: f64 = if var_arg > 80.0 { 1.0 } else { 0.0 };
        var_guard6 = assign860_e1124;

        let (assign870_e1134, assign870_e1134_d_n3, assign870_e1134_d_n4, assign870_e1134_d_n5, assign870_e1134_d_n6,) = {
    if ((var_guard5 != 0.0) && (var_guard6 != 0.0)) {
        let assign870_e1131: f64 = (var_arg - 80.0);
        let assign870_e1132: f64 = (1.0 + assign870_e1131);
        (assign870_e1132, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6,)
    } else {
        (var_le, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6,)
    }
};
        var_le = assign870_e1134;
        var_le_dn3 = assign870_e1134_d_n3;
        var_le_dn4 = assign870_e1134_d_n4;
        var_le_dn5 = assign870_e1134_d_n5;
        var_le_dn6 = assign870_e1134_d_n6;

        let (assign880_e1140, assign880_e1140_d_n3, assign880_e1140_d_n4, assign880_e1140_d_n5, assign880_e1140_d_n6,) = {
    if ((var_guard5 != 0.0) && (var_guard6 != 0.0)) {
        (80.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6,)
    }
};
        var_arg = assign880_e1140;
        var_arg_dn3 = assign880_e1140_d_n3;
        var_arg_dn4 = assign880_e1140_d_n4;
        var_arg_dn5 = assign880_e1140_d_n5;
        var_arg_dn6 = assign880_e1140_d_n6;

        let (assign890_e1147, assign890_e1147_d_n3, assign890_e1147_d_n4, assign890_e1147_d_n5, assign890_e1147_d_n6,) = {
    if ((var_guard5 != 0.0) && (var_guard6 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_le, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6,)
    }
};
        var_le = assign890_e1147;
        var_le_dn3 = assign890_e1147_d_n3;
        var_le_dn4 = assign890_e1147_d_n4;
        var_le_dn5 = assign890_e1147_d_n5;
        var_le_dn6 = assign890_e1147_d_n6;

        let (assign900_e1154, assign900_e1154_d_n3, assign900_e1154_d_n4, assign900_e1154_d_n5, assign900_e1154_d_n6,) = {
    if (var_guard5 != 0.0) {
        let assign900_e1151: f64 = (var_arg).exp();
        let assign900_e1152: f64 = (var_le * assign900_e1151);
        (assign900_e1152, ((var_le_dn3 * assign900_e1151) + (var_le * (assign900_e1151 * var_arg_dn3))), ((var_le_dn4 * assign900_e1151) + (var_le * (assign900_e1151 * var_arg_dn4))), ((var_le_dn5 * assign900_e1151) + (var_le * (assign900_e1151 * var_arg_dn5))), ((var_le_dn6 * assign900_e1151) + (var_le * (assign900_e1151 * var_arg_dn6))),)
    } else {
        (var_le, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6,)
    }
};
        var_le = assign900_e1154;
        var_le_dn3 = assign900_e1154_d_n3;
        var_le_dn4 = assign900_e1154_d_n4;
        var_le_dn5 = assign900_e1154_d_n5;
        var_le_dn6 = assign900_e1154_d_n6;

        let (assign910_e1162, assign910_e1162_d_n3, assign910_e1162_d_n4, assign910_e1162_d_n5, assign910_e1162_d_n6,) = {
    if (var_guard5 != 0.0) {
        let assign910_e1159: f64 = (var_le - 1.0);
        let assign910_e1160: f64 = (var_isr_t * assign910_e1159);
        (assign910_e1160, ((var_isr_t_dn3 * assign910_e1159) + (var_isr_t * var_le_dn3)), (var_isr_t * var_le_dn4), (var_isr_t * var_le_dn5), (var_isr_t * var_le_dn6),)
    } else {
        (var_itrev, var_itrev_dn3, var_itrev_dn4, var_itrev_dn5, var_itrev_dn6,)
    }
};
        var_itrev = assign910_e1162;
        var_itrev_dn3 = assign910_e1162_d_n3;
        var_itrev_dn4 = assign910_e1162_d_n4;
        var_itrev_dn5 = assign910_e1162_d_n5;
        var_itrev_dn6 = assign910_e1162_d_n6;

        let (assign920_e1167, assign920_e1167_d_n3, assign920_e1167_d_n4, assign920_e1167_d_n5, assign920_e1167_d_n6,) = {
    if (var_guard5 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itrev, var_itrev_dn3, var_itrev_dn4, var_itrev_dn5, var_itrev_dn6,)
    }
};
        var_itrev = assign920_e1167;
        var_itrev_dn3 = assign920_e1167_d_n3;
        var_itrev_dn4 = assign920_e1167_d_n4;
        var_itrev_dn5 = assign920_e1167_d_n5;
        var_itrev_dn6 = assign920_e1167_d_n6;

        let assign930_e1170: f64 = if var_ise_t > 0.0 { 1.0 } else { 0.0 };
        var_guard7 = assign930_e1170;

        let (assign940_e1178, assign940_e1178_d_n3, assign940_e1178_d_n4, assign940_e1178_d_n5, assign940_e1178_d_n6,) = {
    if (var_guard7 != 0.0) {
        let assign940_e1175: f64 = (p.p59 * var_vt);
        let assign940_e1176: f64 = (var_vbiei / assign940_e1175);
        (assign940_e1176, (-((var_vbiei * (p.p59 * var_vt_dn3)) / (assign940_e1175 * assign940_e1175))), 0.0, (var_vbiei_dn5 / assign940_e1175), (var_vbiei_dn6 / assign940_e1175),)
    } else {
        (var_arg, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6,)
    }
};
        var_arg = assign940_e1178;
        var_arg_dn3 = assign940_e1178_d_n3;
        var_arg_dn4 = assign940_e1178_d_n4;
        var_arg_dn5 = assign940_e1178_d_n5;
        var_arg_dn6 = assign940_e1178_d_n6;

        let (assign950_e1189, assign950_e1189_d_n3, assign950_e1189_d_n4, assign950_e1189_d_n5, assign950_e1189_d_n6,) = {
    if (var_guard7 != 0.0) {
        let assign950_e1181: f64 = (-var_vbiei);
        let assign950_e1183: f64 = (assign950_e1181 - var_bvr_t);
        let assign950_e1186: f64 = (p.p57 * var_vt);
        let assign950_e1187: f64 = (assign950_e1183 / assign950_e1186);
        (assign950_e1187, ((((-var_bvr_t_dn3) * assign950_e1186) - (assign950_e1183 * (p.p57 * var_vt_dn3))) / (assign950_e1186 * assign950_e1186)), 0.0, ((-var_vbiei_dn5) / assign950_e1186), ((-var_vbiei_dn6) / assign950_e1186),)
    } else {
        (var_argbv, var_argbv_dn3, var_argbv_dn4, var_argbv_dn5, var_argbv_dn6,)
    }
};
        var_argbv = assign950_e1189;
        var_argbv_dn3 = assign950_e1189_d_n3;
        var_argbv_dn4 = assign950_e1189_d_n4;
        var_argbv_dn5 = assign950_e1189_d_n5;
        var_argbv_dn6 = assign950_e1189_d_n6;

        let (assign960_e1198, assign960_e1198_d_n3,) = {
    if (var_guard7 != 0.0) {
        let assign960_e1192: f64 = (-var_bvr_t);
        let assign960_e1195: f64 = (p.p57 * var_vt);
        let assign960_e1196: f64 = (assign960_e1192 / assign960_e1195);
        (assign960_e1196, ((((-var_bvr_t_dn3) * assign960_e1195) - (assign960_e1192 * (p.p57 * var_vt_dn3))) / (assign960_e1195 * assign960_e1195)),)
    } else {
        (var_argbvvt, var_argbvvt_dn3,)
    }
};
        var_argbvvt = assign960_e1198;
        var_argbvvt_dn3 = assign960_e1198_d_n3;

        let assign970_e1201: f64 = if var_arg > 80.0 { 1.0 } else { 0.0 };
        var_guard8 = assign970_e1201;

        let (assign980_e1211, assign980_e1211_d_n3, assign980_e1211_d_n4, assign980_e1211_d_n5, assign980_e1211_d_n6,) = {
    if ((var_guard7 != 0.0) && (var_guard8 != 0.0)) {
        let assign980_e1208: f64 = (var_arg - 80.0);
        let assign980_e1209: f64 = (1.0 + assign980_e1208);
        (assign980_e1209, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6,)
    } else {
        (var_le, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6,)
    }
};
        var_le = assign980_e1211;
        var_le_dn3 = assign980_e1211_d_n3;
        var_le_dn4 = assign980_e1211_d_n4;
        var_le_dn5 = assign980_e1211_d_n5;
        var_le_dn6 = assign980_e1211_d_n6;

        let (assign990_e1217, assign990_e1217_d_n3, assign990_e1217_d_n4, assign990_e1217_d_n5, assign990_e1217_d_n6,) = {
    if ((var_guard7 != 0.0) && (var_guard8 != 0.0)) {
        (80.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6,)
    }
};
        var_arg = assign990_e1217;
        var_arg_dn3 = assign990_e1217_d_n3;
        var_arg_dn4 = assign990_e1217_d_n4;
        var_arg_dn5 = assign990_e1217_d_n5;
        var_arg_dn6 = assign990_e1217_d_n6;

        let (assign1000_e1224, assign1000_e1224_d_n3, assign1000_e1224_d_n4, assign1000_e1224_d_n5, assign1000_e1224_d_n6,) = {
    if ((var_guard7 != 0.0) && (var_guard8 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_le, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6,)
    }
};
        var_le = assign1000_e1224;
        var_le_dn3 = assign1000_e1224_d_n3;
        var_le_dn4 = assign1000_e1224_d_n4;
        var_le_dn5 = assign1000_e1224_d_n5;
        var_le_dn6 = assign1000_e1224_d_n6;

        let (assign1010_e1231, assign1010_e1231_d_n3, assign1010_e1231_d_n4, assign1010_e1231_d_n5, assign1010_e1231_d_n6,) = {
    if (var_guard7 != 0.0) {
        let assign1010_e1228: f64 = (var_arg).exp();
        let assign1010_e1229: f64 = (var_le * assign1010_e1228);
        (assign1010_e1229, ((var_le_dn3 * assign1010_e1228) + (var_le * (assign1010_e1228 * var_arg_dn3))), ((var_le_dn4 * assign1010_e1228) + (var_le * (assign1010_e1228 * var_arg_dn4))), ((var_le_dn5 * assign1010_e1228) + (var_le * (assign1010_e1228 * var_arg_dn5))), ((var_le_dn6 * assign1010_e1228) + (var_le * (assign1010_e1228 * var_arg_dn6))),)
    } else {
        (var_le, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6,)
    }
};
        var_le = assign1010_e1231;
        var_le_dn3 = assign1010_e1231_d_n3;
        var_le_dn4 = assign1010_e1231_d_n4;
        var_le_dn5 = assign1010_e1231_d_n5;
        var_le_dn6 = assign1010_e1231_d_n6;

        let (assign1020_e1303, assign1020_e1303_d_n3, assign1020_e1303_d_n4, assign1020_e1303_d_n5, assign1020_e1303_d_n6,) = {
    if (var_guard7 != 0.0) {
        let assign1020_e1239: f64 = (-37.0);
        let (assign1020_e1266, assign1020_e1266_d_n3, assign1020_e1266_d_n4, assign1020_e1266_d_n5, assign1020_e1266_d_n6,) = {
            if ((!(var_argbv >= 37.0)) && (!(var_argbv <= assign1020_e1239))) {
                let assign1020_e1244: f64 = (var_argbv).exp();
                let assign1020_e1246: f64 = (assign1020_e1244 + 1.0);
                let assign1020_e1247: f64 = (assign1020_e1246).ln();
                (assign1020_e1247, ((assign1020_e1244 * var_argbv_dn3) / assign1020_e1246), ((assign1020_e1244 * var_argbv_dn4) / assign1020_e1246), ((assign1020_e1244 * var_argbv_dn5) / assign1020_e1246), ((assign1020_e1244 * var_argbv_dn6) / assign1020_e1246),)
            } else {
                let assign1020_e1254: f64 = (-37.0);
                let (assign1020_e1265, assign1020_e1265_d_n3, assign1020_e1265_d_n4, assign1020_e1265_d_n5, assign1020_e1265_d_n6,) = {
                    if ((!(var_argbv >= 37.0)) && (var_argbv <= assign1020_e1254)) {
                        let assign1020_e1258: f64 = (var_argbv).exp();
                        (assign1020_e1258, (assign1020_e1258 * var_argbv_dn3), (assign1020_e1258 * var_argbv_dn4), (assign1020_e1258 * var_argbv_dn5), (assign1020_e1258 * var_argbv_dn6),)
                    } else {
                        let (assign1020_e1264, assign1020_e1264_d_n3, assign1020_e1264_d_n4, assign1020_e1264_d_n5, assign1020_e1264_d_n6,) = {
                            if (var_argbv >= 37.0) {
                                (var_argbv, var_argbv_dn3, var_argbv_dn4, var_argbv_dn5, var_argbv_dn6,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign1020_e1264, assign1020_e1264_d_n3, assign1020_e1264_d_n4, assign1020_e1264_d_n5, assign1020_e1264_d_n6,)
                    }
                };
                (assign1020_e1265, assign1020_e1265_d_n3, assign1020_e1265_d_n4, assign1020_e1265_d_n5, assign1020_e1265_d_n6,)
            }
        };
        let assign1020_e1273: f64 = (-37.0);
        let (assign1020_e1300, assign1020_e1300_d_n3,) = {
            if ((!(var_argbvvt >= 37.0)) && (!(var_argbvvt <= assign1020_e1273))) {
                let assign1020_e1278: f64 = (var_argbvvt).exp();
                let assign1020_e1280: f64 = (assign1020_e1278 + 1.0);
                let assign1020_e1281: f64 = (assign1020_e1280).ln();
                (assign1020_e1281, ((assign1020_e1278 * var_argbvvt_dn3) / assign1020_e1280),)
            } else {
                let assign1020_e1288: f64 = (-37.0);
                let (assign1020_e1299, assign1020_e1299_d_n3,) = {
                    if ((!(var_argbvvt >= 37.0)) && (var_argbvvt <= assign1020_e1288)) {
                        let assign1020_e1292: f64 = (var_argbvvt).exp();
                        (assign1020_e1292, (assign1020_e1292 * var_argbvvt_dn3),)
                    } else {
                        let (assign1020_e1298, assign1020_e1298_d_n3,) = {
                            if (var_argbvvt >= 37.0) {
                                (var_argbvvt, var_argbvvt_dn3,)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign1020_e1298, assign1020_e1298_d_n3,)
                    }
                };
                (assign1020_e1299, assign1020_e1299_d_n3,)
            }
        };
        let assign1020_e1301: f64 = (assign1020_e1266 - assign1020_e1300);
        (assign1020_e1301, (assign1020_e1266_d_n3 - assign1020_e1300_d_n3), assign1020_e1266_d_n4, assign1020_e1266_d_n5, assign1020_e1266_d_n6,)
    } else {
        (var_lebv, var_lebv_dn3, var_lebv_dn4, var_lebv_dn5, var_lebv_dn6,)
    }
};
        var_lebv = assign1020_e1303;
        var_lebv_dn3 = assign1020_e1303_d_n3;
        var_lebv_dn4 = assign1020_e1303_d_n4;
        var_lebv_dn5 = assign1020_e1303_d_n5;
        var_lebv_dn6 = assign1020_e1303_d_n6;

        let (assign1030_e1324, assign1030_e1324_d_n3, assign1030_e1324_d_n4, assign1030_e1324_d_n5, assign1030_e1324_d_n6,) = {
    if (var_guard7 != 0.0) {
        let assign1030_e1308: f64 = (var_le - 1.0);
        let assign1030_e1309: f64 = (var_ise_t * assign1030_e1308);
        let assign1030_e1316: f64 = (var_vbiei).abs();
        let assign1030_e1318: f64 = (assign1030_e1316).powf(var_theexp_t);
        let assign1030_e1319: f64 = (p.p8 * assign1030_e1318);
        let assign1030_e1320: f64 = (1.0 + assign1030_e1319);
        let assign1030_e1321: f64 = 0.0;
        let assign1030_e1322: f64 = (assign1030_e1309 - assign1030_e1321);
        (assign1030_e1322, ((var_ise_t_dn3 * assign1030_e1308) + (var_ise_t * var_le_dn3)), (var_ise_t * var_le_dn4), (var_ise_t * var_le_dn5), (var_ise_t * var_le_dn6),)
    } else {
        (var_ibe2, var_ibe2_dn3, var_ibe2_dn4, var_ibe2_dn5, var_ibe2_dn6,)
    }
};
        var_ibe2 = assign1030_e1324;
        var_ibe2_dn3 = assign1030_e1324_d_n3;
        var_ibe2_dn4 = assign1030_e1324_d_n4;
        var_ibe2_dn5 = assign1030_e1324_d_n5;
        var_ibe2_dn6 = assign1030_e1324_d_n6;

        let (assign1040_e1329, assign1040_e1329_d_n3, assign1040_e1329_d_n4, assign1040_e1329_d_n5, assign1040_e1329_d_n6,) = {
    if (var_guard7 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibe2, var_ibe2_dn3, var_ibe2_dn4, var_ibe2_dn5, var_ibe2_dn6,)
    }
};
        var_ibe2 = assign1040_e1329;
        var_ibe2_dn3 = assign1040_e1329_d_n3;
        var_ibe2_dn4 = assign1040_e1329_d_n4;
        var_ibe2_dn5 = assign1040_e1329_d_n5;
        var_ibe2_dn6 = assign1040_e1329_d_n6;

        let assign1050_e1332: f64 = if var_is_t > 0.0 { 1.0 } else { 0.0 };
        var_guard9 = assign1050_e1332;

        let (assign1060_e1340, assign1060_e1340_d_n3, assign1060_e1340_d_n4, assign1060_e1340_d_n5, assign1060_e1340_d_n6,) = {
    if (var_guard9 != 0.0) {
        let assign1060_e1337: f64 = (p.p61 * var_vt);
        let assign1060_e1338: f64 = (var_vbici / assign1060_e1337);
        (assign1060_e1338, (-((var_vbici * (p.p61 * var_vt_dn3)) / (assign1060_e1337 * assign1060_e1337))), (var_vbici_dn4 / assign1060_e1337), (var_vbici_dn5 / assign1060_e1337), 0.0,)
    } else {
        (var_arg, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6,)
    }
};
        var_arg = assign1060_e1340;
        var_arg_dn3 = assign1060_e1340_d_n3;
        var_arg_dn4 = assign1060_e1340_d_n4;
        var_arg_dn5 = assign1060_e1340_d_n5;
        var_arg_dn6 = assign1060_e1340_d_n6;

        *var_arg_slot = var_arg;
        *var_arg_dn3_slot = var_arg_dn3;
        *var_arg_dn4_slot = var_arg_dn4;
        *var_arg_dn5_slot = var_arg_dn5;
        *var_arg_dn6_slot = var_arg_dn6;
        *var_argbv_slot = var_argbv;
        *var_argbv_dn3_slot = var_argbv_dn3;
        *var_argbv_dn4_slot = var_argbv_dn4;
        *var_argbv_dn5_slot = var_argbv_dn5;
        *var_argbv_dn6_slot = var_argbv_dn6;
        *var_argbvvt_slot = var_argbvvt;
        *var_argbvvt_dn3_slot = var_argbvvt_dn3;
        *var_guard5_slot = var_guard5;
        *var_guard6_slot = var_guard6;
        *var_guard7_slot = var_guard7;
        *var_guard8_slot = var_guard8;
        *var_guard9_slot = var_guard9;
        *var_ibe2_slot = var_ibe2;
        *var_ibe2_dn3_slot = var_ibe2_dn3;
        *var_ibe2_dn4_slot = var_ibe2_dn4;
        *var_ibe2_dn5_slot = var_ibe2_dn5;
        *var_ibe2_dn6_slot = var_ibe2_dn6;
        *var_ifwd_slot = var_ifwd;
        *var_ifwd_dn3_slot = var_ifwd_dn3;
        *var_ifwd_dn4_slot = var_ifwd_dn4;
        *var_ifwd_dn5_slot = var_ifwd_dn5;
        *var_ifwd_dn6_slot = var_ifwd_dn6;
        *var_itrev_slot = var_itrev;
        *var_itrev_dn3_slot = var_itrev_dn3;
        *var_itrev_dn4_slot = var_itrev_dn4;
        *var_itrev_dn5_slot = var_itrev_dn5;
        *var_itrev_dn6_slot = var_itrev_dn6;
        *var_le_slot = var_le;
        *var_le_dn3_slot = var_le_dn3;
        *var_le_dn4_slot = var_le_dn4;
        *var_le_dn5_slot = var_le_dn5;
        *var_le_dn6_slot = var_le_dn6;
        *var_lebv_slot = var_lebv;
        *var_lebv_dn3_slot = var_lebv_dn3;
        *var_lebv_dn4_slot = var_lebv_dn4;
        *var_lebv_dn5_slot = var_lebv_dn5;
        *var_lebv_dn6_slot = var_lebv_dn6;
        *var_t0_slot = var_t0;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
    }

    pub(super) fn stamp_transient_block_2(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_bf_t: f64,
        var_bf_t_dn3: f64,
        var_bf_t_dn4: f64,
        var_bf_t_dn5: f64,
        var_br_t: f64,
        var_br_t_dn3: f64,
        var_bvr_t: f64,
        var_bvr_t_dn3: f64,
        var_guard9: f64,
        var_ibe2: f64,
        var_ibe2_dn3: f64,
        var_ibe2_dn4: f64,
        var_ibe2_dn5: f64,
        var_ibe2_dn6: f64,
        var_ifwd: f64,
        var_ifwd_dn3: f64,
        var_ifwd_dn4: f64,
        var_ifwd_dn5: f64,
        var_ifwd_dn6: f64,
        var_ijbvc_t: f64,
        var_ijbvc_t_dn3: f64,
        var_is_t: f64,
        var_is_t_dn3: f64,
        var_isc_t: f64,
        var_isc_t_dn3: f64,
        var_itrev: f64,
        var_itrev_dn3: f64,
        var_itrev_dn4: f64,
        var_itrev_dn5: f64,
        var_itrev_dn6: f64,
        var_oikr: f64,
        var_ovaf: f64,
        var_ovar: f64,
        var_theexp_t: f64,
        var_theexp_t_dn3: f64,
        var_vbbi: f64,
        var_vbbi_dn1: f64,
        var_vbbi_dn5: f64,
        var_vbici: f64,
        var_vbici_dn4: f64,
        var_vbici_dn5: f64,
        var_vbiei: f64,
        var_vbiei_dn5: f64,
        var_vbiei_dn6: f64,
        var_vt: f64,
        var_vt_dn3: f64,
        var_arg_slot: &mut f64,
        var_arg_dn3_slot: &mut f64,
        var_arg_dn4_slot: &mut f64,
        var_arg_dn5_slot: &mut f64,
        var_arg_dn6_slot: &mut f64,
        var_argbv_slot: &mut f64,
        var_argbv_dn3_slot: &mut f64,
        var_argbv_dn4_slot: &mut f64,
        var_argbv_dn5_slot: &mut f64,
        var_argbv_dn6_slot: &mut f64,
        var_argbvvt_slot: &mut f64,
        var_argbvvt_dn3_slot: &mut f64,
        var_d_ratio_slot: &mut f64,
        var_d_ratio_dn5_slot: &mut f64,
        var_d_ratio_dn6_slot: &mut f64,
        var_d_ratio_dn9_slot: &mut f64,
        var_dkqb_slot: &mut f64,
        var_dkqb_dn3_slot: &mut f64,
        var_dkqb_dn4_slot: &mut f64,
        var_dkqb_dn5_slot: &mut f64,
        var_dkqb_dn6_slot: &mut f64,
        var_guard10_slot: &mut f64,
        var_guard11_slot: &mut f64,
        var_guard12_slot: &mut f64,
        var_ibc_slot: &mut f64,
        var_ibc2_slot: &mut f64,
        var_ibc2_dn3_slot: &mut f64,
        var_ibc2_dn4_slot: &mut f64,
        var_ibc2_dn5_slot: &mut f64,
        var_ibc2_dn6_slot: &mut f64,
        var_ibc_dn3_slot: &mut f64,
        var_ibc_dn4_slot: &mut f64,
        var_ibc_dn5_slot: &mut f64,
        var_ibc_dn6_slot: &mut f64,
        var_ibe_slot: &mut f64,
        var_ibe_dn3_slot: &mut f64,
        var_ibe_dn4_slot: &mut f64,
        var_ibe_dn5_slot: &mut f64,
        var_ibe_dn6_slot: &mut f64,
        var_ibwd_slot: &mut f64,
        var_ibwd_dn3_slot: &mut f64,
        var_ibwd_dn4_slot: &mut f64,
        var_ibwd_dn5_slot: &mut f64,
        var_ibwd_dn6_slot: &mut f64,
        var_ikq1_slot: &mut f64,
        var_ikq1_dn4_slot: &mut f64,
        var_ikq1_dn5_slot: &mut f64,
        var_ikq1_dn6_slot: &mut f64,
        var_ikqb_slot: &mut f64,
        var_ikqb_dn3_slot: &mut f64,
        var_ikqb_dn4_slot: &mut f64,
        var_ikqb_dn5_slot: &mut f64,
        var_ikqb_dn6_slot: &mut f64,
        var_itr_slot: &mut f64,
        var_itr_dn3_slot: &mut f64,
        var_itr_dn4_slot: &mut f64,
        var_itr_dn5_slot: &mut f64,
        var_itr_dn6_slot: &mut f64,
        var_itzf_slot: &mut f64,
        var_itzf_dn3_slot: &mut f64,
        var_itzf_dn4_slot: &mut f64,
        var_itzf_dn5_slot: &mut f64,
        var_itzf_dn6_slot: &mut f64,
        var_itzf_f_slot: &mut f64,
        var_itzf_f_dn3_slot: &mut f64,
        var_itzf_f_dn4_slot: &mut f64,
        var_itzf_f_dn5_slot: &mut f64,
        var_itzf_f_dn6_slot: &mut f64,
        var_itzf_f_dn9_slot: &mut f64,
        var_kq2_slot: &mut f64,
        var_kq2_dn3_slot: &mut f64,
        var_kq2_dn4_slot: &mut f64,
        var_kq2_dn5_slot: &mut f64,
        var_kq2_dn6_slot: &mut f64,
        var_le_slot: &mut f64,
        var_le_dn3_slot: &mut f64,
        var_le_dn4_slot: &mut f64,
        var_le_dn5_slot: &mut f64,
        var_le_dn6_slot: &mut f64,
        var_lebv_slot: &mut f64,
        var_lebv_dn3_slot: &mut f64,
        var_lebv_dn4_slot: &mut f64,
        var_lebv_dn5_slot: &mut f64,
        var_lebv_dn6_slot: &mut f64,
        var_oikf_slot: &mut f64,
        var_oikf_dn4_slot: &mut f64,
        var_oikf_dn5_slot: &mut f64,
        var_vbesat_slot: &mut f64,
        var_vbesat_dn1_slot: &mut f64,
        var_vbesat_dn5_slot: &mut f64,
    ) {
        let nv9 = ctx.node_voltage(nodes[9]);
        let mut var_arg: f64 = *var_arg_slot;
        let mut var_arg_dn3: f64 = *var_arg_dn3_slot;
        let mut var_arg_dn4: f64 = *var_arg_dn4_slot;
        let mut var_arg_dn5: f64 = *var_arg_dn5_slot;
        let mut var_arg_dn6: f64 = *var_arg_dn6_slot;
        let mut var_argbv: f64 = *var_argbv_slot;
        let mut var_argbv_dn3: f64 = *var_argbv_dn3_slot;
        let mut var_argbv_dn4: f64 = *var_argbv_dn4_slot;
        let mut var_argbv_dn5: f64 = *var_argbv_dn5_slot;
        let mut var_argbv_dn6: f64 = *var_argbv_dn6_slot;
        let mut var_argbvvt: f64 = *var_argbvvt_slot;
        let mut var_argbvvt_dn3: f64 = *var_argbvvt_dn3_slot;
        let mut var_d_ratio: f64 = *var_d_ratio_slot;
        let mut var_d_ratio_dn5: f64 = *var_d_ratio_dn5_slot;
        let mut var_d_ratio_dn6: f64 = *var_d_ratio_dn6_slot;
        let mut var_d_ratio_dn9: f64 = *var_d_ratio_dn9_slot;
        let mut var_dkqb: f64 = *var_dkqb_slot;
        let mut var_dkqb_dn3: f64 = *var_dkqb_dn3_slot;
        let mut var_dkqb_dn4: f64 = *var_dkqb_dn4_slot;
        let mut var_dkqb_dn5: f64 = *var_dkqb_dn5_slot;
        let mut var_dkqb_dn6: f64 = *var_dkqb_dn6_slot;
        let mut var_guard10: f64 = *var_guard10_slot;
        let mut var_guard11: f64 = *var_guard11_slot;
        let mut var_guard12: f64 = *var_guard12_slot;
        let mut var_ibc: f64 = *var_ibc_slot;
        let mut var_ibc2: f64 = *var_ibc2_slot;
        let mut var_ibc2_dn3: f64 = *var_ibc2_dn3_slot;
        let mut var_ibc2_dn4: f64 = *var_ibc2_dn4_slot;
        let mut var_ibc2_dn5: f64 = *var_ibc2_dn5_slot;
        let mut var_ibc2_dn6: f64 = *var_ibc2_dn6_slot;
        let mut var_ibc_dn3: f64 = *var_ibc_dn3_slot;
        let mut var_ibc_dn4: f64 = *var_ibc_dn4_slot;
        let mut var_ibc_dn5: f64 = *var_ibc_dn5_slot;
        let mut var_ibc_dn6: f64 = *var_ibc_dn6_slot;
        let mut var_ibe: f64 = *var_ibe_slot;
        let mut var_ibe_dn3: f64 = *var_ibe_dn3_slot;
        let mut var_ibe_dn4: f64 = *var_ibe_dn4_slot;
        let mut var_ibe_dn5: f64 = *var_ibe_dn5_slot;
        let mut var_ibe_dn6: f64 = *var_ibe_dn6_slot;
        let mut var_ibwd: f64 = *var_ibwd_slot;
        let mut var_ibwd_dn3: f64 = *var_ibwd_dn3_slot;
        let mut var_ibwd_dn4: f64 = *var_ibwd_dn4_slot;
        let mut var_ibwd_dn5: f64 = *var_ibwd_dn5_slot;
        let mut var_ibwd_dn6: f64 = *var_ibwd_dn6_slot;
        let mut var_ikq1: f64 = *var_ikq1_slot;
        let mut var_ikq1_dn4: f64 = *var_ikq1_dn4_slot;
        let mut var_ikq1_dn5: f64 = *var_ikq1_dn5_slot;
        let mut var_ikq1_dn6: f64 = *var_ikq1_dn6_slot;
        let mut var_ikqb: f64 = *var_ikqb_slot;
        let mut var_ikqb_dn3: f64 = *var_ikqb_dn3_slot;
        let mut var_ikqb_dn4: f64 = *var_ikqb_dn4_slot;
        let mut var_ikqb_dn5: f64 = *var_ikqb_dn5_slot;
        let mut var_ikqb_dn6: f64 = *var_ikqb_dn6_slot;
        let mut var_itr: f64 = *var_itr_slot;
        let mut var_itr_dn3: f64 = *var_itr_dn3_slot;
        let mut var_itr_dn4: f64 = *var_itr_dn4_slot;
        let mut var_itr_dn5: f64 = *var_itr_dn5_slot;
        let mut var_itr_dn6: f64 = *var_itr_dn6_slot;
        let mut var_itzf: f64 = *var_itzf_slot;
        let mut var_itzf_dn3: f64 = *var_itzf_dn3_slot;
        let mut var_itzf_dn4: f64 = *var_itzf_dn4_slot;
        let mut var_itzf_dn5: f64 = *var_itzf_dn5_slot;
        let mut var_itzf_dn6: f64 = *var_itzf_dn6_slot;
        let mut var_itzf_f: f64 = *var_itzf_f_slot;
        let mut var_itzf_f_dn3: f64 = *var_itzf_f_dn3_slot;
        let mut var_itzf_f_dn4: f64 = *var_itzf_f_dn4_slot;
        let mut var_itzf_f_dn5: f64 = *var_itzf_f_dn5_slot;
        let mut var_itzf_f_dn6: f64 = *var_itzf_f_dn6_slot;
        let mut var_itzf_f_dn9: f64 = *var_itzf_f_dn9_slot;
        let mut var_kq2: f64 = *var_kq2_slot;
        let mut var_kq2_dn3: f64 = *var_kq2_dn3_slot;
        let mut var_kq2_dn4: f64 = *var_kq2_dn4_slot;
        let mut var_kq2_dn5: f64 = *var_kq2_dn5_slot;
        let mut var_kq2_dn6: f64 = *var_kq2_dn6_slot;
        let mut var_le: f64 = *var_le_slot;
        let mut var_le_dn3: f64 = *var_le_dn3_slot;
        let mut var_le_dn4: f64 = *var_le_dn4_slot;
        let mut var_le_dn5: f64 = *var_le_dn5_slot;
        let mut var_le_dn6: f64 = *var_le_dn6_slot;
        let mut var_lebv: f64 = *var_lebv_slot;
        let mut var_lebv_dn3: f64 = *var_lebv_dn3_slot;
        let mut var_lebv_dn4: f64 = *var_lebv_dn4_slot;
        let mut var_lebv_dn5: f64 = *var_lebv_dn5_slot;
        let mut var_lebv_dn6: f64 = *var_lebv_dn6_slot;
        let mut var_oikf: f64 = *var_oikf_slot;
        let mut var_oikf_dn4: f64 = *var_oikf_dn4_slot;
        let mut var_oikf_dn5: f64 = *var_oikf_dn5_slot;
        let mut var_vbesat: f64 = *var_vbesat_slot;
        let mut var_vbesat_dn1: f64 = *var_vbesat_dn1_slot;
        let mut var_vbesat_dn5: f64 = *var_vbesat_dn5_slot;

        let (assign1070_e1351, assign1070_e1351_d_n3, assign1070_e1351_d_n4, assign1070_e1351_d_n5, assign1070_e1351_d_n6,) = {
    if (var_guard9 != 0.0) {
        let assign1070_e1343: f64 = (-var_vbici);
        let assign1070_e1345: f64 = (assign1070_e1343 - var_bvr_t);
        let assign1070_e1348: f64 = (p.p57 * var_vt);
        let assign1070_e1349: f64 = (assign1070_e1345 / assign1070_e1348);
        (assign1070_e1349, ((((-var_bvr_t_dn3) * assign1070_e1348) - (assign1070_e1345 * (p.p57 * var_vt_dn3))) / (assign1070_e1348 * assign1070_e1348)), ((-var_vbici_dn4) / assign1070_e1348), ((-var_vbici_dn5) / assign1070_e1348), 0.0,)
    } else {
        (var_argbv, var_argbv_dn3, var_argbv_dn4, var_argbv_dn5, var_argbv_dn6,)
    }
};
        var_argbv = assign1070_e1351;
        var_argbv_dn3 = assign1070_e1351_d_n3;
        var_argbv_dn4 = assign1070_e1351_d_n4;
        var_argbv_dn5 = assign1070_e1351_d_n5;
        var_argbv_dn6 = assign1070_e1351_d_n6;

        let (assign1080_e1360, assign1080_e1360_d_n3,) = {
    if (var_guard9 != 0.0) {
        let assign1080_e1354: f64 = (-var_bvr_t);
        let assign1080_e1357: f64 = (p.p57 * var_vt);
        let assign1080_e1358: f64 = (assign1080_e1354 / assign1080_e1357);
        (assign1080_e1358, ((((-var_bvr_t_dn3) * assign1080_e1357) - (assign1080_e1354 * (p.p57 * var_vt_dn3))) / (assign1080_e1357 * assign1080_e1357)),)
    } else {
        (var_argbvvt, var_argbvvt_dn3,)
    }
};
        var_argbvvt = assign1080_e1360;
        var_argbvvt_dn3 = assign1080_e1360_d_n3;

        let assign1090_e1363: f64 = if var_arg > 80.0 { 1.0 } else { 0.0 };
        var_guard10 = assign1090_e1363;

        let (assign1100_e1373, assign1100_e1373_d_n3, assign1100_e1373_d_n4, assign1100_e1373_d_n5, assign1100_e1373_d_n6,) = {
    if ((var_guard9 != 0.0) && (var_guard10 != 0.0)) {
        let assign1100_e1370: f64 = (var_arg - 80.0);
        let assign1100_e1371: f64 = (1.0 + assign1100_e1370);
        (assign1100_e1371, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6,)
    } else {
        (var_le, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6,)
    }
};
        var_le = assign1100_e1373;
        var_le_dn3 = assign1100_e1373_d_n3;
        var_le_dn4 = assign1100_e1373_d_n4;
        var_le_dn5 = assign1100_e1373_d_n5;
        var_le_dn6 = assign1100_e1373_d_n6;

        let (assign1110_e1379, assign1110_e1379_d_n3, assign1110_e1379_d_n4, assign1110_e1379_d_n5, assign1110_e1379_d_n6,) = {
    if ((var_guard9 != 0.0) && (var_guard10 != 0.0)) {
        (80.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6,)
    }
};
        var_arg = assign1110_e1379;
        var_arg_dn3 = assign1110_e1379_d_n3;
        var_arg_dn4 = assign1110_e1379_d_n4;
        var_arg_dn5 = assign1110_e1379_d_n5;
        var_arg_dn6 = assign1110_e1379_d_n6;

        let (assign1120_e1386, assign1120_e1386_d_n3, assign1120_e1386_d_n4, assign1120_e1386_d_n5, assign1120_e1386_d_n6,) = {
    if ((var_guard9 != 0.0) && (var_guard10 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_le, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6,)
    }
};
        var_le = assign1120_e1386;
        var_le_dn3 = assign1120_e1386_d_n3;
        var_le_dn4 = assign1120_e1386_d_n4;
        var_le_dn5 = assign1120_e1386_d_n5;
        var_le_dn6 = assign1120_e1386_d_n6;

        let (assign1130_e1393, assign1130_e1393_d_n3, assign1130_e1393_d_n4, assign1130_e1393_d_n5, assign1130_e1393_d_n6,) = {
    if (var_guard9 != 0.0) {
        let assign1130_e1390: f64 = (var_arg).exp();
        let assign1130_e1391: f64 = (var_le * assign1130_e1390);
        (assign1130_e1391, ((var_le_dn3 * assign1130_e1390) + (var_le * (assign1130_e1390 * var_arg_dn3))), ((var_le_dn4 * assign1130_e1390) + (var_le * (assign1130_e1390 * var_arg_dn4))), ((var_le_dn5 * assign1130_e1390) + (var_le * (assign1130_e1390 * var_arg_dn5))), ((var_le_dn6 * assign1130_e1390) + (var_le * (assign1130_e1390 * var_arg_dn6))),)
    } else {
        (var_le, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6,)
    }
};
        var_le = assign1130_e1393;
        var_le_dn3 = assign1130_e1393_d_n3;
        var_le_dn4 = assign1130_e1393_d_n4;
        var_le_dn5 = assign1130_e1393_d_n5;
        var_le_dn6 = assign1130_e1393_d_n6;

        let (assign1140_e1465, assign1140_e1465_d_n3, assign1140_e1465_d_n4, assign1140_e1465_d_n5, assign1140_e1465_d_n6,) = {
    if (var_guard9 != 0.0) {
        let assign1140_e1401: f64 = (-37.0);
        let (assign1140_e1428, assign1140_e1428_d_n3, assign1140_e1428_d_n4, assign1140_e1428_d_n5, assign1140_e1428_d_n6,) = {
            if ((!(var_argbv >= 37.0)) && (!(var_argbv <= assign1140_e1401))) {
                let assign1140_e1406: f64 = (var_argbv).exp();
                let assign1140_e1408: f64 = (assign1140_e1406 + 1.0);
                let assign1140_e1409: f64 = (assign1140_e1408).ln();
                (assign1140_e1409, ((assign1140_e1406 * var_argbv_dn3) / assign1140_e1408), ((assign1140_e1406 * var_argbv_dn4) / assign1140_e1408), ((assign1140_e1406 * var_argbv_dn5) / assign1140_e1408), ((assign1140_e1406 * var_argbv_dn6) / assign1140_e1408),)
            } else {
                let assign1140_e1416: f64 = (-37.0);
                let (assign1140_e1427, assign1140_e1427_d_n3, assign1140_e1427_d_n4, assign1140_e1427_d_n5, assign1140_e1427_d_n6,) = {
                    if ((!(var_argbv >= 37.0)) && (var_argbv <= assign1140_e1416)) {
                        let assign1140_e1420: f64 = (var_argbv).exp();
                        (assign1140_e1420, (assign1140_e1420 * var_argbv_dn3), (assign1140_e1420 * var_argbv_dn4), (assign1140_e1420 * var_argbv_dn5), (assign1140_e1420 * var_argbv_dn6),)
                    } else {
                        let (assign1140_e1426, assign1140_e1426_d_n3, assign1140_e1426_d_n4, assign1140_e1426_d_n5, assign1140_e1426_d_n6,) = {
                            if (var_argbv >= 37.0) {
                                (var_argbv, var_argbv_dn3, var_argbv_dn4, var_argbv_dn5, var_argbv_dn6,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign1140_e1426, assign1140_e1426_d_n3, assign1140_e1426_d_n4, assign1140_e1426_d_n5, assign1140_e1426_d_n6,)
                    }
                };
                (assign1140_e1427, assign1140_e1427_d_n3, assign1140_e1427_d_n4, assign1140_e1427_d_n5, assign1140_e1427_d_n6,)
            }
        };
        let assign1140_e1435: f64 = (-37.0);
        let (assign1140_e1462, assign1140_e1462_d_n3,) = {
            if ((!(var_argbvvt >= 37.0)) && (!(var_argbvvt <= assign1140_e1435))) {
                let assign1140_e1440: f64 = (var_argbvvt).exp();
                let assign1140_e1442: f64 = (assign1140_e1440 + 1.0);
                let assign1140_e1443: f64 = (assign1140_e1442).ln();
                (assign1140_e1443, ((assign1140_e1440 * var_argbvvt_dn3) / assign1140_e1442),)
            } else {
                let assign1140_e1450: f64 = (-37.0);
                let (assign1140_e1461, assign1140_e1461_d_n3,) = {
                    if ((!(var_argbvvt >= 37.0)) && (var_argbvvt <= assign1140_e1450)) {
                        let assign1140_e1454: f64 = (var_argbvvt).exp();
                        (assign1140_e1454, (assign1140_e1454 * var_argbvvt_dn3),)
                    } else {
                        let (assign1140_e1460, assign1140_e1460_d_n3,) = {
                            if (var_argbvvt >= 37.0) {
                                (var_argbvvt, var_argbvvt_dn3,)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign1140_e1460, assign1140_e1460_d_n3,)
                    }
                };
                (assign1140_e1461, assign1140_e1461_d_n3,)
            }
        };
        let assign1140_e1463: f64 = (assign1140_e1428 - assign1140_e1462);
        (assign1140_e1463, (assign1140_e1428_d_n3 - assign1140_e1462_d_n3), assign1140_e1428_d_n4, assign1140_e1428_d_n5, assign1140_e1428_d_n6,)
    } else {
        (var_lebv, var_lebv_dn3, var_lebv_dn4, var_lebv_dn5, var_lebv_dn6,)
    }
};
        var_lebv = assign1140_e1465;
        var_lebv_dn3 = assign1140_e1465_d_n3;
        var_lebv_dn4 = assign1140_e1465_d_n4;
        var_lebv_dn5 = assign1140_e1465_d_n5;
        var_lebv_dn6 = assign1140_e1465_d_n6;

        let (assign1150_e1486, assign1150_e1486_d_n3, assign1150_e1486_d_n4, assign1150_e1486_d_n5, assign1150_e1486_d_n6,) = {
    if (var_guard9 != 0.0) {
        let assign1150_e1470: f64 = (var_le - 1.0);
        let assign1150_e1471: f64 = (var_is_t * assign1150_e1470);
        let assign1150_e1474: f64 = (var_ijbvc_t * var_lebv);
        let assign1150_e1478: f64 = (var_vbici).abs();
        let assign1150_e1480: f64 = (assign1150_e1478).powf(var_theexp_t);
        let assign1150_e1481: f64 = (p.p8 * assign1150_e1480);
        let assign1150_e1482: f64 = (1.0 + assign1150_e1481);
        let assign1150_e1483: f64 = (assign1150_e1474 / assign1150_e1482);
        let assign1150_e1484: f64 = (assign1150_e1471 - assign1150_e1483);
        (assign1150_e1484, (((var_is_t_dn3 * assign1150_e1470) + (var_is_t * var_le_dn3)) - (((((var_ijbvc_t_dn3 * var_lebv) + (var_ijbvc_t * var_lebv_dn3)) * assign1150_e1482) - (assign1150_e1474 * (p.p8 * if var_theexp_t_dn3 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { 0.0 } else { (assign1150_e1480 * (var_theexp_t_dn3 * (assign1150_e1478).ln())) }))) / (assign1150_e1482 * assign1150_e1482))), ((var_is_t * var_le_dn4) - ((((var_ijbvc_t * var_lebv_dn4) * assign1150_e1482) - (assign1150_e1474 * (p.p8 * if 0.0 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign1150_e1478).powf(var_theexp_t - 1.0) * if var_vbici >= 0.0 { var_vbici_dn4 } else { (-var_vbici_dn4) })) } } else { (assign1150_e1480 * (var_theexp_t * (if var_vbici >= 0.0 { var_vbici_dn4 } else { (-var_vbici_dn4) } / assign1150_e1478))) }))) / (assign1150_e1482 * assign1150_e1482))), ((var_is_t * var_le_dn5) - ((((var_ijbvc_t * var_lebv_dn5) * assign1150_e1482) - (assign1150_e1474 * (p.p8 * if 0.0 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign1150_e1478).powf(var_theexp_t - 1.0) * if var_vbici >= 0.0 { var_vbici_dn5 } else { (-var_vbici_dn5) })) } } else { (assign1150_e1480 * (var_theexp_t * (if var_vbici >= 0.0 { var_vbici_dn5 } else { (-var_vbici_dn5) } / assign1150_e1478))) }))) / (assign1150_e1482 * assign1150_e1482))), ((var_is_t * var_le_dn6) - ((var_ijbvc_t * var_lebv_dn6) / assign1150_e1482)),)
    } else {
        (var_ibwd, var_ibwd_dn3, var_ibwd_dn4, var_ibwd_dn5, var_ibwd_dn6,)
    }
};
        var_ibwd = assign1150_e1486;
        var_ibwd_dn3 = assign1150_e1486_d_n3;
        var_ibwd_dn4 = assign1150_e1486_d_n4;
        var_ibwd_dn5 = assign1150_e1486_d_n5;
        var_ibwd_dn6 = assign1150_e1486_d_n6;

        let (assign1160_e1491, assign1160_e1491_d_n3, assign1160_e1491_d_n4, assign1160_e1491_d_n5, assign1160_e1491_d_n6,) = {
    if (var_guard9 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibwd, var_ibwd_dn3, var_ibwd_dn4, var_ibwd_dn5, var_ibwd_dn6,)
    }
};
        var_ibwd = assign1160_e1491;
        var_ibwd_dn3 = assign1160_e1491_d_n3;
        var_ibwd_dn4 = assign1160_e1491_d_n4;
        var_ibwd_dn5 = assign1160_e1491_d_n5;
        var_ibwd_dn6 = assign1160_e1491_d_n6;

        let assign1170_e1494: f64 = if var_isc_t > 0.0 { 1.0 } else { 0.0 };
        var_guard11 = assign1170_e1494;

        let (assign1180_e1502, assign1180_e1502_d_n3, assign1180_e1502_d_n4, assign1180_e1502_d_n5, assign1180_e1502_d_n6,) = {
    if (var_guard11 != 0.0) {
        let assign1180_e1499: f64 = (p.p65 * var_vt);
        let assign1180_e1500: f64 = (var_vbici / assign1180_e1499);
        (assign1180_e1500, (-((var_vbici * (p.p65 * var_vt_dn3)) / (assign1180_e1499 * assign1180_e1499))), (var_vbici_dn4 / assign1180_e1499), (var_vbici_dn5 / assign1180_e1499), 0.0,)
    } else {
        (var_arg, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6,)
    }
};
        var_arg = assign1180_e1502;
        var_arg_dn3 = assign1180_e1502_d_n3;
        var_arg_dn4 = assign1180_e1502_d_n4;
        var_arg_dn5 = assign1180_e1502_d_n5;
        var_arg_dn6 = assign1180_e1502_d_n6;

        let (assign1190_e1513, assign1190_e1513_d_n3, assign1190_e1513_d_n4, assign1190_e1513_d_n5, assign1190_e1513_d_n6,) = {
    if (var_guard11 != 0.0) {
        let assign1190_e1505: f64 = (-var_vbici);
        let assign1190_e1507: f64 = (assign1190_e1505 - var_bvr_t);
        let assign1190_e1510: f64 = (p.p57 * var_vt);
        let assign1190_e1511: f64 = (assign1190_e1507 / assign1190_e1510);
        (assign1190_e1511, ((((-var_bvr_t_dn3) * assign1190_e1510) - (assign1190_e1507 * (p.p57 * var_vt_dn3))) / (assign1190_e1510 * assign1190_e1510)), ((-var_vbici_dn4) / assign1190_e1510), ((-var_vbici_dn5) / assign1190_e1510), 0.0,)
    } else {
        (var_argbv, var_argbv_dn3, var_argbv_dn4, var_argbv_dn5, var_argbv_dn6,)
    }
};
        var_argbv = assign1190_e1513;
        var_argbv_dn3 = assign1190_e1513_d_n3;
        var_argbv_dn4 = assign1190_e1513_d_n4;
        var_argbv_dn5 = assign1190_e1513_d_n5;
        var_argbv_dn6 = assign1190_e1513_d_n6;

        let (assign1200_e1522, assign1200_e1522_d_n3,) = {
    if (var_guard11 != 0.0) {
        let assign1200_e1516: f64 = (-var_bvr_t);
        let assign1200_e1519: f64 = (p.p57 * var_vt);
        let assign1200_e1520: f64 = (assign1200_e1516 / assign1200_e1519);
        (assign1200_e1520, ((((-var_bvr_t_dn3) * assign1200_e1519) - (assign1200_e1516 * (p.p57 * var_vt_dn3))) / (assign1200_e1519 * assign1200_e1519)),)
    } else {
        (var_argbvvt, var_argbvvt_dn3,)
    }
};
        var_argbvvt = assign1200_e1522;
        var_argbvvt_dn3 = assign1200_e1522_d_n3;

        let assign1210_e1525: f64 = if var_arg > 80.0 { 1.0 } else { 0.0 };
        var_guard12 = assign1210_e1525;

        let (assign1220_e1535, assign1220_e1535_d_n3, assign1220_e1535_d_n4, assign1220_e1535_d_n5, assign1220_e1535_d_n6,) = {
    if ((var_guard11 != 0.0) && (var_guard12 != 0.0)) {
        let assign1220_e1532: f64 = (var_arg - 80.0);
        let assign1220_e1533: f64 = (1.0 + assign1220_e1532);
        (assign1220_e1533, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6,)
    } else {
        (var_le, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6,)
    }
};
        var_le = assign1220_e1535;
        var_le_dn3 = assign1220_e1535_d_n3;
        var_le_dn4 = assign1220_e1535_d_n4;
        var_le_dn5 = assign1220_e1535_d_n5;
        var_le_dn6 = assign1220_e1535_d_n6;

        let (assign1230_e1541, assign1230_e1541_d_n3, assign1230_e1541_d_n4, assign1230_e1541_d_n5, assign1230_e1541_d_n6,) = {
    if ((var_guard11 != 0.0) && (var_guard12 != 0.0)) {
        (80.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6,)
    }
};
        var_arg = assign1230_e1541;
        var_arg_dn3 = assign1230_e1541_d_n3;
        var_arg_dn4 = assign1230_e1541_d_n4;
        var_arg_dn5 = assign1230_e1541_d_n5;
        var_arg_dn6 = assign1230_e1541_d_n6;

        let (assign1240_e1548, assign1240_e1548_d_n3, assign1240_e1548_d_n4, assign1240_e1548_d_n5, assign1240_e1548_d_n6,) = {
    if ((var_guard11 != 0.0) && (var_guard12 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_le, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6,)
    }
};
        var_le = assign1240_e1548;
        var_le_dn3 = assign1240_e1548_d_n3;
        var_le_dn4 = assign1240_e1548_d_n4;
        var_le_dn5 = assign1240_e1548_d_n5;
        var_le_dn6 = assign1240_e1548_d_n6;

        let (assign1250_e1555, assign1250_e1555_d_n3, assign1250_e1555_d_n4, assign1250_e1555_d_n5, assign1250_e1555_d_n6,) = {
    if (var_guard11 != 0.0) {
        let assign1250_e1552: f64 = (var_arg).exp();
        let assign1250_e1553: f64 = (var_le * assign1250_e1552);
        (assign1250_e1553, ((var_le_dn3 * assign1250_e1552) + (var_le * (assign1250_e1552 * var_arg_dn3))), ((var_le_dn4 * assign1250_e1552) + (var_le * (assign1250_e1552 * var_arg_dn4))), ((var_le_dn5 * assign1250_e1552) + (var_le * (assign1250_e1552 * var_arg_dn5))), ((var_le_dn6 * assign1250_e1552) + (var_le * (assign1250_e1552 * var_arg_dn6))),)
    } else {
        (var_le, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6,)
    }
};
        var_le = assign1250_e1555;
        var_le_dn3 = assign1250_e1555_d_n3;
        var_le_dn4 = assign1250_e1555_d_n4;
        var_le_dn5 = assign1250_e1555_d_n5;
        var_le_dn6 = assign1250_e1555_d_n6;

        let (assign1260_e1627, assign1260_e1627_d_n3, assign1260_e1627_d_n4, assign1260_e1627_d_n5, assign1260_e1627_d_n6,) = {
    if (var_guard11 != 0.0) {
        let assign1260_e1563: f64 = (-37.0);
        let (assign1260_e1590, assign1260_e1590_d_n3, assign1260_e1590_d_n4, assign1260_e1590_d_n5, assign1260_e1590_d_n6,) = {
            if ((!(var_argbv >= 37.0)) && (!(var_argbv <= assign1260_e1563))) {
                let assign1260_e1568: f64 = (var_argbv).exp();
                let assign1260_e1570: f64 = (assign1260_e1568 + 1.0);
                let assign1260_e1571: f64 = (assign1260_e1570).ln();
                (assign1260_e1571, ((assign1260_e1568 * var_argbv_dn3) / assign1260_e1570), ((assign1260_e1568 * var_argbv_dn4) / assign1260_e1570), ((assign1260_e1568 * var_argbv_dn5) / assign1260_e1570), ((assign1260_e1568 * var_argbv_dn6) / assign1260_e1570),)
            } else {
                let assign1260_e1578: f64 = (-37.0);
                let (assign1260_e1589, assign1260_e1589_d_n3, assign1260_e1589_d_n4, assign1260_e1589_d_n5, assign1260_e1589_d_n6,) = {
                    if ((!(var_argbv >= 37.0)) && (var_argbv <= assign1260_e1578)) {
                        let assign1260_e1582: f64 = (var_argbv).exp();
                        (assign1260_e1582, (assign1260_e1582 * var_argbv_dn3), (assign1260_e1582 * var_argbv_dn4), (assign1260_e1582 * var_argbv_dn5), (assign1260_e1582 * var_argbv_dn6),)
                    } else {
                        let (assign1260_e1588, assign1260_e1588_d_n3, assign1260_e1588_d_n4, assign1260_e1588_d_n5, assign1260_e1588_d_n6,) = {
                            if (var_argbv >= 37.0) {
                                (var_argbv, var_argbv_dn3, var_argbv_dn4, var_argbv_dn5, var_argbv_dn6,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign1260_e1588, assign1260_e1588_d_n3, assign1260_e1588_d_n4, assign1260_e1588_d_n5, assign1260_e1588_d_n6,)
                    }
                };
                (assign1260_e1589, assign1260_e1589_d_n3, assign1260_e1589_d_n4, assign1260_e1589_d_n5, assign1260_e1589_d_n6,)
            }
        };
        let assign1260_e1597: f64 = (-37.0);
        let (assign1260_e1624, assign1260_e1624_d_n3,) = {
            if ((!(var_argbvvt >= 37.0)) && (!(var_argbvvt <= assign1260_e1597))) {
                let assign1260_e1602: f64 = (var_argbvvt).exp();
                let assign1260_e1604: f64 = (assign1260_e1602 + 1.0);
                let assign1260_e1605: f64 = (assign1260_e1604).ln();
                (assign1260_e1605, ((assign1260_e1602 * var_argbvvt_dn3) / assign1260_e1604),)
            } else {
                let assign1260_e1612: f64 = (-37.0);
                let (assign1260_e1623, assign1260_e1623_d_n3,) = {
                    if ((!(var_argbvvt >= 37.0)) && (var_argbvvt <= assign1260_e1612)) {
                        let assign1260_e1616: f64 = (var_argbvvt).exp();
                        (assign1260_e1616, (assign1260_e1616 * var_argbvvt_dn3),)
                    } else {
                        let (assign1260_e1622, assign1260_e1622_d_n3,) = {
                            if (var_argbvvt >= 37.0) {
                                (var_argbvvt, var_argbvvt_dn3,)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign1260_e1622, assign1260_e1622_d_n3,)
                    }
                };
                (assign1260_e1623, assign1260_e1623_d_n3,)
            }
        };
        let assign1260_e1625: f64 = (assign1260_e1590 - assign1260_e1624);
        (assign1260_e1625, (assign1260_e1590_d_n3 - assign1260_e1624_d_n3), assign1260_e1590_d_n4, assign1260_e1590_d_n5, assign1260_e1590_d_n6,)
    } else {
        (var_lebv, var_lebv_dn3, var_lebv_dn4, var_lebv_dn5, var_lebv_dn6,)
    }
};
        var_lebv = assign1260_e1627;
        var_lebv_dn3 = assign1260_e1627_d_n3;
        var_lebv_dn4 = assign1260_e1627_d_n4;
        var_lebv_dn5 = assign1260_e1627_d_n5;
        var_lebv_dn6 = assign1260_e1627_d_n6;

        let (assign1270_e1648, assign1270_e1648_d_n3, assign1270_e1648_d_n4, assign1270_e1648_d_n5, assign1270_e1648_d_n6,) = {
    if (var_guard11 != 0.0) {
        let assign1270_e1632: f64 = (var_le - 1.0);
        let assign1270_e1633: f64 = (var_isc_t * assign1270_e1632);
        let assign1270_e1640: f64 = (var_vbici).abs();
        let assign1270_e1642: f64 = (assign1270_e1640).powf(p.p9);
        let assign1270_e1643: f64 = (p.p8 * assign1270_e1642);
        let assign1270_e1644: f64 = (1.0 + assign1270_e1643);
        let assign1270_e1645: f64 = 0.0;
        let assign1270_e1646: f64 = (assign1270_e1633 - assign1270_e1645);
        (assign1270_e1646, ((var_isc_t_dn3 * assign1270_e1632) + (var_isc_t * var_le_dn3)), (var_isc_t * var_le_dn4), (var_isc_t * var_le_dn5), (var_isc_t * var_le_dn6),)
    } else {
        (var_ibc2, var_ibc2_dn3, var_ibc2_dn4, var_ibc2_dn5, var_ibc2_dn6,)
    }
};
        var_ibc2 = assign1270_e1648;
        var_ibc2_dn3 = assign1270_e1648_d_n3;
        var_ibc2_dn4 = assign1270_e1648_d_n4;
        var_ibc2_dn5 = assign1270_e1648_d_n5;
        var_ibc2_dn6 = assign1270_e1648_d_n6;

        let (assign1280_e1653, assign1280_e1653_d_n3, assign1280_e1653_d_n4, assign1280_e1653_d_n5, assign1280_e1653_d_n6,) = {
    if (var_guard11 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibc2, var_ibc2_dn3, var_ibc2_dn4, var_ibc2_dn5, var_ibc2_dn6,)
    }
};
        var_ibc2 = assign1280_e1653;
        var_ibc2_dn3 = assign1280_e1653_d_n3;
        var_ibc2_dn4 = assign1280_e1653_d_n4;
        var_ibc2_dn5 = assign1280_e1653_d_n5;
        var_ibc2_dn6 = assign1280_e1653_d_n6;

        let assign1290_e1656: f64 = ((nv9 - 0.0)).min(var_vbiei);
        let assign1290_e1658: f64 = (var_vbiei).abs();
        let assign1290_e1660: f64 = (assign1290_e1658).max(1e-9);
        let assign1290_e1661: f64 = (assign1290_e1656 / assign1290_e1660);
        let assign1290_e1662: f64 = (assign1290_e1661).abs();
        var_d_ratio = assign1290_e1662;
        var_d_ratio_dn5 = if assign1290_e1661 >= 0.0 { (((if (nv9 - 0.0) <= var_vbiei { 0.0 } else { var_vbiei_dn5 } * assign1290_e1660) - (assign1290_e1656 * if assign1290_e1658 >= 1e-9 { if var_vbiei >= 0.0 { var_vbiei_dn5 } else { (-var_vbiei_dn5) } } else { 0.0 })) / (assign1290_e1660 * assign1290_e1660)) } else { (-(((if (nv9 - 0.0) <= var_vbiei { 0.0 } else { var_vbiei_dn5 } * assign1290_e1660) - (assign1290_e1656 * if assign1290_e1658 >= 1e-9 { if var_vbiei >= 0.0 { var_vbiei_dn5 } else { (-var_vbiei_dn5) } } else { 0.0 })) / (assign1290_e1660 * assign1290_e1660))) };
        var_d_ratio_dn6 = if assign1290_e1661 >= 0.0 { (((if (nv9 - 0.0) <= var_vbiei { 0.0 } else { var_vbiei_dn6 } * assign1290_e1660) - (assign1290_e1656 * if assign1290_e1658 >= 1e-9 { if var_vbiei >= 0.0 { var_vbiei_dn6 } else { (-var_vbiei_dn6) } } else { 0.0 })) / (assign1290_e1660 * assign1290_e1660)) } else { (-(((if (nv9 - 0.0) <= var_vbiei { 0.0 } else { var_vbiei_dn6 } * assign1290_e1660) - (assign1290_e1656 * if assign1290_e1658 >= 1e-9 { if var_vbiei >= 0.0 { var_vbiei_dn6 } else { (-var_vbiei_dn6) } } else { 0.0 })) / (assign1290_e1660 * assign1290_e1660))) };
        var_d_ratio_dn9 = if assign1290_e1661 >= 0.0 { (if (nv9 - 0.0) <= var_vbiei { 1.0 } else { 0.0 } / assign1290_e1660) } else { (-(if (nv9 - 0.0) <= var_vbiei { 1.0 } else { 0.0 } / assign1290_e1660)) };

        let assign1300_e1665: f64 = (var_ifwd - var_itrev);
        let assign1300_e1667: f64 = (assign1300_e1665 / var_bf_t);
        let assign1300_e1669: f64 = (assign1300_e1667 + var_ibe2);
        var_ibe = assign1300_e1669;
        var_ibe_dn3 = (((((var_ifwd_dn3 - var_itrev_dn3) * var_bf_t) - (assign1300_e1665 * var_bf_t_dn3)) / (var_bf_t * var_bf_t)) + var_ibe2_dn3);
        var_ibe_dn4 = (((((var_ifwd_dn4 - var_itrev_dn4) * var_bf_t) - (assign1300_e1665 * var_bf_t_dn4)) / (var_bf_t * var_bf_t)) + var_ibe2_dn4);
        var_ibe_dn5 = (((((var_ifwd_dn5 - var_itrev_dn5) * var_bf_t) - (assign1300_e1665 * var_bf_t_dn5)) / (var_bf_t * var_bf_t)) + var_ibe2_dn5);
        var_ibe_dn6 = (((var_ifwd_dn6 - var_itrev_dn6) / var_bf_t) + var_ibe2_dn6);

        let assign1310_e1672: f64 = (var_ibwd / var_br_t);
        let assign1310_e1674: f64 = (assign1310_e1672 + var_ibc2);
        var_ibc = assign1310_e1674;
        var_ibc_dn3 = ((((var_ibwd_dn3 * var_br_t) - (var_ibwd * var_br_t_dn3)) / (var_br_t * var_br_t)) + var_ibc2_dn3);
        var_ibc_dn4 = ((var_ibwd_dn4 / var_br_t) + var_ibc2_dn4);
        var_ibc_dn5 = ((var_ibwd_dn5 / var_br_t) + var_ibc2_dn5);
        var_ibc_dn6 = ((var_ibwd_dn6 / var_br_t) + var_ibc2_dn6);

        let assign1320_e1679: f64 = (var_vbici * p.p81);
        let assign1320_e1680: f64 = (1.0 + assign1320_e1679);
        let assign1320_e1681: f64 = (var_oikf * assign1320_e1680);
        var_oikf = assign1320_e1681;
        var_oikf_dn4 = ((var_oikf_dn4 * assign1320_e1680) + (var_oikf * (var_vbici_dn4 * p.p81)));
        var_oikf_dn5 = ((var_oikf_dn5 * assign1320_e1680) + (var_oikf * (var_vbici_dn5 * p.p81)));

        let assign1330_e1684: f64 = (var_ifwd * var_oikf);
        let assign1330_e1687: f64 = (var_ibwd * var_oikr);
        let assign1330_e1688: f64 = (assign1330_e1684 + assign1330_e1687);
        var_kq2 = assign1330_e1688;
        var_kq2_dn3 = ((var_ifwd_dn3 * var_oikf) + (var_ibwd_dn3 * var_oikr));
        var_kq2_dn4 = (((var_ifwd_dn4 * var_oikf) + (var_ifwd * var_oikf_dn4)) + (var_ibwd_dn4 * var_oikr));
        var_kq2_dn5 = (((var_ifwd_dn5 * var_oikf) + (var_ifwd * var_oikf_dn5)) + (var_ibwd_dn5 * var_oikr));
        var_kq2_dn6 = ((var_ifwd_dn6 * var_oikf) + (var_ibwd_dn6 * var_oikr));

        let assign1340_e1692: f64 = (var_vbiei * var_ovar);
        let assign1340_e1693: f64 = (1.0 - assign1340_e1692);
        let assign1340_e1696: f64 = (var_vbici * var_ovaf);
        let assign1340_e1697: f64 = (assign1340_e1693 - assign1340_e1696);
        var_ikq1 = assign1340_e1697;
        var_ikq1_dn4 = (-(var_vbici_dn4 * var_ovaf));
        var_ikq1_dn5 = ((-(var_vbiei_dn5 * var_ovar)) - (var_vbici_dn5 * var_ovaf));
        var_ikq1_dn6 = (-(var_vbiei_dn6 * var_ovar));

        let assign1350_e1702: f64 = (4.0 * var_kq2);
        let assign1350_e1703: f64 = (1.0 + assign1350_e1702);
        let assign1350_e1704: f64 = (assign1350_e1703).abs();
        let assign1350_e1706: f64 = (assign1350_e1704).powf(p.p82);
        let assign1350_e1707: f64 = (1.0 + assign1350_e1706);
        var_dkqb = assign1350_e1707;
        var_dkqb_dn3 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn3) } else { (-(4.0 * var_kq2_dn3)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn3) } else { (-(4.0 * var_kq2_dn3)) } / assign1350_e1704))) };
        var_dkqb_dn4 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn4) } else { (-(4.0 * var_kq2_dn4)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn4) } else { (-(4.0 * var_kq2_dn4)) } / assign1350_e1704))) };
        var_dkqb_dn5 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn5) } else { (-(4.0 * var_kq2_dn5)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn5) } else { (-(4.0 * var_kq2_dn5)) } / assign1350_e1704))) };
        var_dkqb_dn6 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn6) } else { (-(4.0 * var_kq2_dn6)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn6) } else { (-(4.0 * var_kq2_dn6)) } / assign1350_e1704))) };

        let assign1360_e1710: f64 = (2.0 * var_ikq1);
        let assign1360_e1712: f64 = (assign1360_e1710 / var_dkqb);
        var_ikqb = assign1360_e1712;
        var_ikqb_dn3 = (-((assign1360_e1710 * var_dkqb_dn3) / (var_dkqb * var_dkqb)));
        var_ikqb_dn4 = ((((2.0 * var_ikq1_dn4) * var_dkqb) - (assign1360_e1710 * var_dkqb_dn4)) / (var_dkqb * var_dkqb));
        var_ikqb_dn5 = ((((2.0 * var_ikq1_dn5) * var_dkqb) - (assign1360_e1710 * var_dkqb_dn5)) / (var_dkqb * var_dkqb));
        var_ikqb_dn6 = ((((2.0 * var_ikq1_dn6) * var_dkqb) - (assign1360_e1710 * var_dkqb_dn6)) / (var_dkqb * var_dkqb));

        let assign1370_e1715: f64 = (var_ibwd * var_ikqb);
        var_itr = assign1370_e1715;
        var_itr_dn3 = ((var_ibwd_dn3 * var_ikqb) + (var_ibwd * var_ikqb_dn3));
        var_itr_dn4 = ((var_ibwd_dn4 * var_ikqb) + (var_ibwd * var_ikqb_dn4));
        var_itr_dn5 = ((var_ibwd_dn5 * var_ikqb) + (var_ibwd * var_ikqb_dn5));
        var_itr_dn6 = ((var_ibwd_dn6 * var_ikqb) + (var_ibwd * var_ikqb_dn6));

        let assign1380_e1718: f64 = (var_ifwd * var_ikqb);
        var_itzf = assign1380_e1718;
        var_itzf_dn3 = ((var_ifwd_dn3 * var_ikqb) + (var_ifwd * var_ikqb_dn3));
        var_itzf_dn4 = ((var_ifwd_dn4 * var_ikqb) + (var_ifwd * var_ikqb_dn4));
        var_itzf_dn5 = ((var_ifwd_dn5 * var_ikqb) + (var_ifwd * var_ikqb_dn5));
        var_itzf_dn6 = ((var_ifwd_dn6 * var_ikqb) + (var_ifwd * var_ikqb_dn6));

        let assign1390_e1721: f64 = (var_ifwd * var_ikqb);
        let assign1390_e1723: f64 = (assign1390_e1721 * var_d_ratio);
        let assign1390_e1725: f64 = (assign1390_e1723 * p.p84);
        let assign1390_e1728: f64 = (1.0 - p.p84);
        let assign1390_e1730: f64 = (assign1390_e1728 * var_ifwd);
        let assign1390_e1732: f64 = (assign1390_e1730 * var_ikqb);
        let assign1390_e1733: f64 = (assign1390_e1725 + assign1390_e1732);
        var_itzf_f = assign1390_e1733;
        var_itzf_f_dn3 = (((((var_ifwd_dn3 * var_ikqb) + (var_ifwd * var_ikqb_dn3)) * var_d_ratio) * p.p84) + (((assign1390_e1728 * var_ifwd_dn3) * var_ikqb) + (assign1390_e1730 * var_ikqb_dn3)));
        var_itzf_f_dn4 = (((((var_ifwd_dn4 * var_ikqb) + (var_ifwd * var_ikqb_dn4)) * var_d_ratio) * p.p84) + (((assign1390_e1728 * var_ifwd_dn4) * var_ikqb) + (assign1390_e1730 * var_ikqb_dn4)));
        var_itzf_f_dn5 = ((((((var_ifwd_dn5 * var_ikqb) + (var_ifwd * var_ikqb_dn5)) * var_d_ratio) + (assign1390_e1721 * var_d_ratio_dn5)) * p.p84) + (((assign1390_e1728 * var_ifwd_dn5) * var_ikqb) + (assign1390_e1730 * var_ikqb_dn5)));
        var_itzf_f_dn6 = ((((((var_ifwd_dn6 * var_ikqb) + (var_ifwd * var_ikqb_dn6)) * var_d_ratio) + (assign1390_e1721 * var_d_ratio_dn6)) * p.p84) + (((assign1390_e1728 * var_ifwd_dn6) * var_ikqb) + (assign1390_e1730 * var_ikqb_dn6)));
        var_itzf_f_dn9 = ((assign1390_e1721 * var_d_ratio_dn9) * p.p84);

        let assign1400_e1737: f64 = (var_vbbi / p.p48);
        let assign1400_e1738: f64 = (assign1400_e1737).abs();
        let assign1400_e1740: f64 = (assign1400_e1738).powf(p.p49);
        let assign1400_e1741: f64 = (1.0 + assign1400_e1740);
        var_vbesat = assign1400_e1741;
        var_vbesat_dn1 = if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((assign1400_e1738).powf(p.p49 - 1.0) * if assign1400_e1737 >= 0.0 { (var_vbbi_dn1 / p.p48) } else { (-(var_vbbi_dn1 / p.p48)) })) } } else { (assign1400_e1740 * (p.p49 * (if assign1400_e1737 >= 0.0 { (var_vbbi_dn1 / p.p48) } else { (-(var_vbbi_dn1 / p.p48)) } / assign1400_e1738))) };
        var_vbesat_dn5 = if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((assign1400_e1738).powf(p.p49 - 1.0) * if assign1400_e1737 >= 0.0 { (var_vbbi_dn5 / p.p48) } else { (-(var_vbbi_dn5 / p.p48)) })) } } else { (assign1400_e1740 * (p.p49 * (if assign1400_e1737 >= 0.0 { (var_vbbi_dn5 / p.p48) } else { (-(var_vbbi_dn5 / p.p48)) } / assign1400_e1738))) };

        *var_arg_slot = var_arg;
        *var_arg_dn3_slot = var_arg_dn3;
        *var_arg_dn4_slot = var_arg_dn4;
        *var_arg_dn5_slot = var_arg_dn5;
        *var_arg_dn6_slot = var_arg_dn6;
        *var_argbv_slot = var_argbv;
        *var_argbv_dn3_slot = var_argbv_dn3;
        *var_argbv_dn4_slot = var_argbv_dn4;
        *var_argbv_dn5_slot = var_argbv_dn5;
        *var_argbv_dn6_slot = var_argbv_dn6;
        *var_argbvvt_slot = var_argbvvt;
        *var_argbvvt_dn3_slot = var_argbvvt_dn3;
        *var_d_ratio_slot = var_d_ratio;
        *var_d_ratio_dn5_slot = var_d_ratio_dn5;
        *var_d_ratio_dn6_slot = var_d_ratio_dn6;
        *var_d_ratio_dn9_slot = var_d_ratio_dn9;
        *var_dkqb_slot = var_dkqb;
        *var_dkqb_dn3_slot = var_dkqb_dn3;
        *var_dkqb_dn4_slot = var_dkqb_dn4;
        *var_dkqb_dn5_slot = var_dkqb_dn5;
        *var_dkqb_dn6_slot = var_dkqb_dn6;
        *var_guard10_slot = var_guard10;
        *var_guard11_slot = var_guard11;
        *var_guard12_slot = var_guard12;
        *var_ibc_slot = var_ibc;
        *var_ibc2_slot = var_ibc2;
        *var_ibc2_dn3_slot = var_ibc2_dn3;
        *var_ibc2_dn4_slot = var_ibc2_dn4;
        *var_ibc2_dn5_slot = var_ibc2_dn5;
        *var_ibc2_dn6_slot = var_ibc2_dn6;
        *var_ibc_dn3_slot = var_ibc_dn3;
        *var_ibc_dn4_slot = var_ibc_dn4;
        *var_ibc_dn5_slot = var_ibc_dn5;
        *var_ibc_dn6_slot = var_ibc_dn6;
        *var_ibe_slot = var_ibe;
        *var_ibe_dn3_slot = var_ibe_dn3;
        *var_ibe_dn4_slot = var_ibe_dn4;
        *var_ibe_dn5_slot = var_ibe_dn5;
        *var_ibe_dn6_slot = var_ibe_dn6;
        *var_ibwd_slot = var_ibwd;
        *var_ibwd_dn3_slot = var_ibwd_dn3;
        *var_ibwd_dn4_slot = var_ibwd_dn4;
        *var_ibwd_dn5_slot = var_ibwd_dn5;
        *var_ibwd_dn6_slot = var_ibwd_dn6;
        *var_ikq1_slot = var_ikq1;
        *var_ikq1_dn4_slot = var_ikq1_dn4;
        *var_ikq1_dn5_slot = var_ikq1_dn5;
        *var_ikq1_dn6_slot = var_ikq1_dn6;
        *var_ikqb_slot = var_ikqb;
        *var_ikqb_dn3_slot = var_ikqb_dn3;
        *var_ikqb_dn4_slot = var_ikqb_dn4;
        *var_ikqb_dn5_slot = var_ikqb_dn5;
        *var_ikqb_dn6_slot = var_ikqb_dn6;
        *var_itr_slot = var_itr;
        *var_itr_dn3_slot = var_itr_dn3;
        *var_itr_dn4_slot = var_itr_dn4;
        *var_itr_dn5_slot = var_itr_dn5;
        *var_itr_dn6_slot = var_itr_dn6;
        *var_itzf_slot = var_itzf;
        *var_itzf_dn3_slot = var_itzf_dn3;
        *var_itzf_dn4_slot = var_itzf_dn4;
        *var_itzf_dn5_slot = var_itzf_dn5;
        *var_itzf_dn6_slot = var_itzf_dn6;
        *var_itzf_f_slot = var_itzf_f;
        *var_itzf_f_dn3_slot = var_itzf_f_dn3;
        *var_itzf_f_dn4_slot = var_itzf_f_dn4;
        *var_itzf_f_dn5_slot = var_itzf_f_dn5;
        *var_itzf_f_dn6_slot = var_itzf_f_dn6;
        *var_itzf_f_dn9_slot = var_itzf_f_dn9;
        *var_kq2_slot = var_kq2;
        *var_kq2_dn3_slot = var_kq2_dn3;
        *var_kq2_dn4_slot = var_kq2_dn4;
        *var_kq2_dn5_slot = var_kq2_dn5;
        *var_kq2_dn6_slot = var_kq2_dn6;
        *var_le_slot = var_le;
        *var_le_dn3_slot = var_le_dn3;
        *var_le_dn4_slot = var_le_dn4;
        *var_le_dn5_slot = var_le_dn5;
        *var_le_dn6_slot = var_le_dn6;
        *var_lebv_slot = var_lebv;
        *var_lebv_dn3_slot = var_lebv_dn3;
        *var_lebv_dn4_slot = var_lebv_dn4;
        *var_lebv_dn5_slot = var_lebv_dn5;
        *var_lebv_dn6_slot = var_lebv_dn6;
        *var_oikf_slot = var_oikf;
        *var_oikf_dn4_slot = var_oikf_dn4;
        *var_oikf_dn5_slot = var_oikf_dn5;
        *var_vbesat_slot = var_vbesat;
        *var_vbesat_dn1_slot = var_vbesat_dn1;
        *var_vbesat_dn5_slot = var_vbesat_dn5;
    }

    pub(super) fn stamp_transient_block_3(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_cjc_t: f64,
        var_cjc_t_dn3: f64,
        var_cje_t: f64,
        var_cje_t_dn3: f64,
        var_cjs_t: f64,
        var_cjs_t_dn3: f64,
        var_ifwd: f64,
        var_ifwd_dn3: f64,
        var_ifwd_dn4: f64,
        var_ifwd_dn5: f64,
        var_ifwd_dn6: f64,
        var_itr: f64,
        var_itr_dn3: f64,
        var_itr_dn4: f64,
        var_itr_dn5: f64,
        var_itr_dn6: f64,
        var_lnrt: f64,
        var_lnrt_dn3: f64,
        var_vbci: f64,
        var_vbci_dn1: f64,
        var_vbci_dn4: f64,
        var_vbesat: f64,
        var_vbesat_dn1: f64,
        var_vbesat_dn5: f64,
        var_vbici: f64,
        var_vbici_dn4: f64,
        var_vbici_dn5: f64,
        var_vbiei: f64,
        var_vbiei_dn5: f64,
        var_vbiei_dn6: f64,
        var_veci: f64,
        var_veci_dn2: f64,
        var_veci_dn4: f64,
        var_veei: f64,
        var_veei_dn2: f64,
        var_veei_dn6: f64,
        var_vjc_t: f64,
        var_vjc_t_dn3: f64,
        var_vje_t: f64,
        var_vje_t_dn3: f64,
        var_vjs_t: f64,
        var_vjs_t_dn3: f64,
        var_dv0_slot: &mut f64,
        var_dv0_dn3_slot: &mut f64,
        var_dvh_slot: &mut f64,
        var_dvh_dn1_slot: &mut f64,
        var_dvh_dn3_slot: &mut f64,
        var_dvh_dn4_slot: &mut f64,
        var_dvh_dn5_slot: &mut f64,
        var_dvh_dn6_slot: &mut f64,
        var_guard13_slot: &mut f64,
        var_guard14_slot: &mut f64,
        var_guard15_slot: &mut f64,
        var_guard16_slot: &mut f64,
        var_guard17_slot: &mut f64,
        var_guard18_slot: &mut f64,
        var_pwq_slot: &mut f64,
        var_qdc_slot: &mut f64,
        var_qdc_dn3_slot: &mut f64,
        var_qdc_dn4_slot: &mut f64,
        var_qdc_dn5_slot: &mut f64,
        var_qdc_dn6_slot: &mut f64,
        var_qde_slot: &mut f64,
        var_qde_dn1_slot: &mut f64,
        var_qde_dn2_slot: &mut f64,
        var_qde_dn3_slot: &mut f64,
        var_qde_dn4_slot: &mut f64,
        var_qde_dn5_slot: &mut f64,
        var_qde_dn6_slot: &mut f64,
        var_qhi_slot: &mut f64,
        var_qhi_dn1_slot: &mut f64,
        var_qhi_dn3_slot: &mut f64,
        var_qhi_dn4_slot: &mut f64,
        var_qhi_dn5_slot: &mut f64,
        var_qhi_dn6_slot: &mut f64,
        var_qjcx_slot: &mut f64,
        var_qjcx_1_slot: &mut f64,
        var_qjcx_1_dn1_slot: &mut f64,
        var_qjcx_1_dn3_slot: &mut f64,
        var_qjcx_1_dn4_slot: &mut f64,
        var_qjcx_1_dn5_slot: &mut f64,
        var_qjcx_1_dn6_slot: &mut f64,
        var_qjcx_dn1_slot: &mut f64,
        var_qjcx_dn3_slot: &mut f64,
        var_qjcx_dn4_slot: &mut f64,
        var_qjcx_dn5_slot: &mut f64,
        var_qjcx_dn6_slot: &mut f64,
        var_qje_slot: &mut f64,
        var_qje_dn1_slot: &mut f64,
        var_qje_dn3_slot: &mut f64,
        var_qje_dn4_slot: &mut f64,
        var_qje_dn5_slot: &mut f64,
        var_qje_dn6_slot: &mut f64,
        var_qjs_slot: &mut f64,
        var_qjs_dn2_slot: &mut f64,
        var_qjs_dn3_slot: &mut f64,
        var_qjs_dn4_slot: &mut f64,
        var_qlo_slot: &mut f64,
        var_qlo_dn1_slot: &mut f64,
        var_qlo_dn3_slot: &mut f64,
        var_qlo_dn4_slot: &mut f64,
        var_qlo_dn5_slot: &mut f64,
        var_qlo_dn6_slot: &mut f64,
        var_rb_slot: &mut f64,
        var_rb_dn1_slot: &mut f64,
        var_rb_dn3_slot: &mut f64,
        var_rb_dn5_slot: &mut f64,
        var_rb_dn8_slot: &mut f64,
        var_rc_slot: &mut f64,
        var_rc_dn3_slot: &mut f64,
        var_re_slot: &mut f64,
        var_re_dn2_slot: &mut f64,
        var_re_dn3_slot: &mut f64,
        var_re_dn6_slot: &mut f64,
        var_tff_slot: &mut f64,
        var_tff_dn1_slot: &mut f64,
        var_tff_dn2_slot: &mut f64,
        var_veesat_slot: &mut f64,
        var_veesat_dn2_slot: &mut f64,
        var_veesat_dn6_slot: &mut f64,
        var_vtff_slot: &mut f64,
        var_vtff1_slot: &mut f64,
        var_vtff1_dn1_slot: &mut f64,
        var_vtff1_dn2_slot: &mut f64,
        var_vtff_dn1_slot: &mut f64,
        var_vtff_dn2_slot: &mut f64,
    ) {
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let mut var_dv0: f64 = *var_dv0_slot;
        let mut var_dv0_dn3: f64 = *var_dv0_dn3_slot;
        let mut var_dvh: f64 = *var_dvh_slot;
        let mut var_dvh_dn1: f64 = *var_dvh_dn1_slot;
        let mut var_dvh_dn3: f64 = *var_dvh_dn3_slot;
        let mut var_dvh_dn4: f64 = *var_dvh_dn4_slot;
        let mut var_dvh_dn5: f64 = *var_dvh_dn5_slot;
        let mut var_dvh_dn6: f64 = *var_dvh_dn6_slot;
        let mut var_guard13: f64 = *var_guard13_slot;
        let mut var_guard14: f64 = *var_guard14_slot;
        let mut var_guard15: f64 = *var_guard15_slot;
        let mut var_guard16: f64 = *var_guard16_slot;
        let mut var_guard17: f64 = *var_guard17_slot;
        let mut var_guard18: f64 = *var_guard18_slot;
        let mut var_pwq: f64 = *var_pwq_slot;
        let mut var_qdc: f64 = *var_qdc_slot;
        let mut var_qdc_dn3: f64 = *var_qdc_dn3_slot;
        let mut var_qdc_dn4: f64 = *var_qdc_dn4_slot;
        let mut var_qdc_dn5: f64 = *var_qdc_dn5_slot;
        let mut var_qdc_dn6: f64 = *var_qdc_dn6_slot;
        let mut var_qde: f64 = *var_qde_slot;
        let mut var_qde_dn1: f64 = *var_qde_dn1_slot;
        let mut var_qde_dn2: f64 = *var_qde_dn2_slot;
        let mut var_qde_dn3: f64 = *var_qde_dn3_slot;
        let mut var_qde_dn4: f64 = *var_qde_dn4_slot;
        let mut var_qde_dn5: f64 = *var_qde_dn5_slot;
        let mut var_qde_dn6: f64 = *var_qde_dn6_slot;
        let mut var_qhi: f64 = *var_qhi_slot;
        let mut var_qhi_dn1: f64 = *var_qhi_dn1_slot;
        let mut var_qhi_dn3: f64 = *var_qhi_dn3_slot;
        let mut var_qhi_dn4: f64 = *var_qhi_dn4_slot;
        let mut var_qhi_dn5: f64 = *var_qhi_dn5_slot;
        let mut var_qhi_dn6: f64 = *var_qhi_dn6_slot;
        let mut var_qjcx: f64 = *var_qjcx_slot;
        let mut var_qjcx_1: f64 = *var_qjcx_1_slot;
        let mut var_qjcx_1_dn1: f64 = *var_qjcx_1_dn1_slot;
        let mut var_qjcx_1_dn3: f64 = *var_qjcx_1_dn3_slot;
        let mut var_qjcx_1_dn4: f64 = *var_qjcx_1_dn4_slot;
        let mut var_qjcx_1_dn5: f64 = *var_qjcx_1_dn5_slot;
        let mut var_qjcx_1_dn6: f64 = *var_qjcx_1_dn6_slot;
        let mut var_qjcx_dn1: f64 = *var_qjcx_dn1_slot;
        let mut var_qjcx_dn3: f64 = *var_qjcx_dn3_slot;
        let mut var_qjcx_dn4: f64 = *var_qjcx_dn4_slot;
        let mut var_qjcx_dn5: f64 = *var_qjcx_dn5_slot;
        let mut var_qjcx_dn6: f64 = *var_qjcx_dn6_slot;
        let mut var_qje: f64 = *var_qje_slot;
        let mut var_qje_dn1: f64 = *var_qje_dn1_slot;
        let mut var_qje_dn3: f64 = *var_qje_dn3_slot;
        let mut var_qje_dn4: f64 = *var_qje_dn4_slot;
        let mut var_qje_dn5: f64 = *var_qje_dn5_slot;
        let mut var_qje_dn6: f64 = *var_qje_dn6_slot;
        let mut var_qjs: f64 = *var_qjs_slot;
        let mut var_qjs_dn2: f64 = *var_qjs_dn2_slot;
        let mut var_qjs_dn3: f64 = *var_qjs_dn3_slot;
        let mut var_qjs_dn4: f64 = *var_qjs_dn4_slot;
        let mut var_qlo: f64 = *var_qlo_slot;
        let mut var_qlo_dn1: f64 = *var_qlo_dn1_slot;
        let mut var_qlo_dn3: f64 = *var_qlo_dn3_slot;
        let mut var_qlo_dn4: f64 = *var_qlo_dn4_slot;
        let mut var_qlo_dn5: f64 = *var_qlo_dn5_slot;
        let mut var_qlo_dn6: f64 = *var_qlo_dn6_slot;
        let mut var_rb: f64 = *var_rb_slot;
        let mut var_rb_dn1: f64 = *var_rb_dn1_slot;
        let mut var_rb_dn3: f64 = *var_rb_dn3_slot;
        let mut var_rb_dn5: f64 = *var_rb_dn5_slot;
        let mut var_rb_dn8: f64 = *var_rb_dn8_slot;
        let mut var_rc: f64 = *var_rc_slot;
        let mut var_rc_dn3: f64 = *var_rc_dn3_slot;
        let mut var_re: f64 = *var_re_slot;
        let mut var_re_dn2: f64 = *var_re_dn2_slot;
        let mut var_re_dn3: f64 = *var_re_dn3_slot;
        let mut var_re_dn6: f64 = *var_re_dn6_slot;
        let mut var_tff: f64 = *var_tff_slot;
        let mut var_tff_dn1: f64 = *var_tff_dn1_slot;
        let mut var_tff_dn2: f64 = *var_tff_dn2_slot;
        let mut var_veesat: f64 = *var_veesat_slot;
        let mut var_veesat_dn2: f64 = *var_veesat_dn2_slot;
        let mut var_veesat_dn6: f64 = *var_veesat_dn6_slot;
        let mut var_vtff: f64 = *var_vtff_slot;
        let mut var_vtff1: f64 = *var_vtff1_slot;
        let mut var_vtff1_dn1: f64 = *var_vtff1_dn1_slot;
        let mut var_vtff1_dn2: f64 = *var_vtff1_dn2_slot;
        let mut var_vtff_dn1: f64 = *var_vtff_dn1_slot;
        let mut var_vtff_dn2: f64 = *var_vtff_dn2_slot;

        let assign1410_e1745: f64 = (var_veei / p.p50);
        let assign1410_e1746: f64 = (assign1410_e1745).abs();
        let assign1410_e1748: f64 = (assign1410_e1746).powf(p.p51);
        let assign1410_e1749: f64 = (1.0 + assign1410_e1748);
        var_veesat = assign1410_e1749;
        var_veesat_dn2 = if 0.0 == 0.0 && ((p.p51) as f64).is_finite() && ((p.p51) as f64).fract() == 0.0 { if p.p51 == 0.0 { 0.0 } else { (p.p51 * ((assign1410_e1746).powf(p.p51 - 1.0) * if assign1410_e1745 >= 0.0 { (var_veei_dn2 / p.p50) } else { (-(var_veei_dn2 / p.p50)) })) } } else { (assign1410_e1748 * (p.p51 * (if assign1410_e1745 >= 0.0 { (var_veei_dn2 / p.p50) } else { (-(var_veei_dn2 / p.p50)) } / assign1410_e1746))) };
        var_veesat_dn6 = if 0.0 == 0.0 && ((p.p51) as f64).is_finite() && ((p.p51) as f64).fract() == 0.0 { if p.p51 == 0.0 { 0.0 } else { (p.p51 * ((assign1410_e1746).powf(p.p51 - 1.0) * if assign1410_e1745 >= 0.0 { (var_veei_dn6 / p.p50) } else { (-(var_veei_dn6 / p.p50)) })) } } else { (assign1410_e1748 * (p.p51 * (if assign1410_e1745 >= 0.0 { (var_veei_dn6 / p.p50) } else { (-(var_veei_dn6 / p.p50)) } / assign1410_e1746))) };

        let assign1420_e1753: f64 = (var_lnrt * p.p37);
        let assign1420_e1754: f64 = (assign1420_e1753).exp();
        let assign1420_e1755: f64 = (p.p12 * assign1420_e1754);
        let assign1420_e1759: f64 = (1.0 / p.p49);
        let assign1420_e1760: f64 = (var_vbesat).powf(assign1420_e1759);
        let assign1420_e1761: f64 = (assign1420_e1755 * assign1420_e1760);
        var_rb = assign1420_e1761;
        var_rb_dn1 = (assign1420_e1755 * if 0.0 == 0.0 && ((assign1420_e1759) as f64).is_finite() && ((assign1420_e1759) as f64).fract() == 0.0 { if assign1420_e1759 == 0.0 { 0.0 } else { (assign1420_e1759 * ((var_vbesat).powf(assign1420_e1759 - 1.0) * var_vbesat_dn1)) } } else { (assign1420_e1760 * (assign1420_e1759 * (var_vbesat_dn1 / var_vbesat))) });
        var_rb_dn3 = ((p.p12 * (assign1420_e1754 * (var_lnrt_dn3 * p.p37))) * assign1420_e1760);
        var_rb_dn5 = (assign1420_e1755 * if 0.0 == 0.0 && ((assign1420_e1759) as f64).is_finite() && ((assign1420_e1759) as f64).fract() == 0.0 { if assign1420_e1759 == 0.0 { 0.0 } else { (assign1420_e1759 * ((var_vbesat).powf(assign1420_e1759 - 1.0) * var_vbesat_dn5)) } } else { (assign1420_e1760 * (assign1420_e1759 * (var_vbesat_dn5 / var_vbesat))) });
        var_rb_dn8 = 0.0;

        let assign1430_e1765: f64 = (var_lnrt * p.p78);
        let assign1430_e1766: f64 = (assign1430_e1765).exp();
        let assign1430_e1767: f64 = (p.p66 * assign1430_e1766);
        var_rc = assign1430_e1767;
        var_rc_dn3 = (p.p66 * (assign1430_e1766 * (var_lnrt_dn3 * p.p78)));

        let assign1440_e1771: f64 = (var_lnrt * p.p38);
        let assign1440_e1772: f64 = (assign1440_e1771).exp();
        let assign1440_e1773: f64 = (p.p14 * assign1440_e1772);
        let assign1440_e1777: f64 = (1.0 / p.p51);
        let assign1440_e1778: f64 = (var_veesat).powf(assign1440_e1777);
        let assign1440_e1779: f64 = (assign1440_e1773 * assign1440_e1778);
        var_re = assign1440_e1779;
        var_re_dn2 = (assign1440_e1773 * if 0.0 == 0.0 && ((assign1440_e1777) as f64).is_finite() && ((assign1440_e1777) as f64).fract() == 0.0 { if assign1440_e1777 == 0.0 { 0.0 } else { (assign1440_e1777 * ((var_veesat).powf(assign1440_e1777 - 1.0) * var_veesat_dn2)) } } else { (assign1440_e1778 * (assign1440_e1777 * (var_veesat_dn2 / var_veesat))) });
        var_re_dn3 = ((p.p14 * (assign1440_e1772 * (var_lnrt_dn3 * p.p38))) * assign1440_e1778);
        var_re_dn6 = (assign1440_e1773 * if 0.0 == 0.0 && ((assign1440_e1777) as f64).is_finite() && ((assign1440_e1777) as f64).fract() == 0.0 { if assign1440_e1777 == 0.0 { 0.0 } else { (assign1440_e1777 * ((var_veesat).powf(assign1440_e1777 - 1.0) * var_veesat_dn6)) } } else { (assign1440_e1778 * (assign1440_e1777 * (var_veesat_dn6 / var_veesat))) });

        let assign1450_e1782: f64 = ((nv1 - nv2) / p.p40);
        let assign1450_e1783: f64 = (assign1450_e1782).abs();
        let assign1450_e1785: f64 = (assign1450_e1783).powf(p.p39);
        var_vtff = assign1450_e1785;
        var_vtff_dn1 = if 0.0 == 0.0 && ((p.p39) as f64).is_finite() && ((p.p39) as f64).fract() == 0.0 { if p.p39 == 0.0 { 0.0 } else { (p.p39 * ((assign1450_e1783).powf(p.p39 - 1.0) * if assign1450_e1782 >= 0.0 { (1.0 / p.p40) } else { (-(1.0 / p.p40)) })) } } else { (assign1450_e1785 * (p.p39 * (if assign1450_e1782 >= 0.0 { (1.0 / p.p40) } else { (-(1.0 / p.p40)) } / assign1450_e1783))) };
        var_vtff_dn2 = if 0.0 == 0.0 && ((p.p39) as f64).is_finite() && ((p.p39) as f64).fract() == 0.0 { if p.p39 == 0.0 { 0.0 } else { (p.p39 * ((assign1450_e1783).powf(p.p39 - 1.0) * if assign1450_e1782 >= 0.0 { (-1.0 / p.p40) } else { (-(-1.0 / p.p40)) })) } } else { (assign1450_e1785 * (p.p39 * (if assign1450_e1782 >= 0.0 { (-1.0 / p.p40) } else { (-(-1.0 / p.p40)) } / assign1450_e1783))) };

        let assign1460_e1788: f64 = (1.0 + var_vtff);
        let assign1460_e1791: f64 = (1.0 / p.p39);
        let assign1460_e1792: f64 = (assign1460_e1788).powf(assign1460_e1791);
        let assign1460_e1794: f64 = (assign1460_e1792 - 1.0);
        var_vtff1 = assign1460_e1794;
        var_vtff1_dn1 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * var_vtff_dn1)) } } else { (assign1460_e1792 * (assign1460_e1791 * (var_vtff_dn1 / assign1460_e1788))) };
        var_vtff1_dn2 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * var_vtff_dn2)) } } else { (assign1460_e1792 * (assign1460_e1791 * (var_vtff_dn2 / assign1460_e1788))) };

        let assign1470_e1799: f64 = (p.p41 * var_vtff1);
        let assign1470_e1800: f64 = (1.0 + assign1470_e1799);
        let assign1470_e1801: f64 = (p.p19 * assign1470_e1800);
        var_tff = assign1470_e1801;
        var_tff_dn1 = (p.p19 * (p.p41 * var_vtff1_dn1));
        var_tff_dn2 = (p.p19 * (p.p41 * var_vtff1_dn2));

        let assign1480_e1804: f64 = (var_tff * var_ifwd);
        var_qde = assign1480_e1804;
        var_qde_dn1 = (var_tff_dn1 * var_ifwd);
        var_qde_dn2 = (var_tff_dn2 * var_ifwd);
        var_qde_dn3 = (var_tff * var_ifwd_dn3);
        var_qde_dn4 = (var_tff * var_ifwd_dn4);
        var_qde_dn5 = (var_tff * var_ifwd_dn5);
        var_qde_dn6 = (var_tff * var_ifwd_dn6);

        let assign1490_e1807: f64 = (p.p73 * var_itr);
        var_qdc = assign1490_e1807;
        var_qdc_dn3 = (p.p73 * var_itr_dn3);
        var_qdc_dn4 = (p.p73 * var_itr_dn4);
        var_qdc_dn5 = (p.p73 * var_itr_dn5);
        var_qdc_dn6 = (p.p73 * var_itr_dn6);

        let assign1500_e1810: f64 = if p.p32 == 1.0 { 1.0 } else { 0.0 };
        var_guard13 = assign1500_e1810;

        let (assign1510_e1823, assign1510_e1823_d_n1, assign1510_e1823_d_n3, assign1510_e1823_d_n5, assign1510_e1823_d_n8,) = {
    if (var_guard13 != 0.0) {
        let assign1510_e1815: f64 = ((nv8 - 0.0)).abs();
        let assign1510_e1817: f64 = (assign1510_e1815 / p.p20);
        let assign1510_e1819: f64 = (assign1510_e1817).powf(p.p44);
        let assign1510_e1820: f64 = (1.0 + assign1510_e1819);
        let assign1510_e1821: f64 = (var_rb / assign1510_e1820);
        (assign1510_e1821, (var_rb_dn1 / assign1510_e1820), (var_rb_dn3 / assign1510_e1820), (var_rb_dn5 / assign1510_e1820), (((var_rb_dn8 * assign1510_e1820) - (var_rb * if 0.0 == 0.0 && ((p.p44) as f64).is_finite() && ((p.p44) as f64).fract() == 0.0 { if p.p44 == 0.0 { 0.0 } else { (p.p44 * ((assign1510_e1817).powf(p.p44 - 1.0) * (if (nv8 - 0.0) >= 0.0 { 1.0 } else { (-1.0) } / p.p20))) } } else { (assign1510_e1819 * (p.p44 * ((if (nv8 - 0.0) >= 0.0 { 1.0 } else { (-1.0) } / p.p20) / assign1510_e1817))) })) / (assign1510_e1820 * assign1510_e1820)),)
    } else {
        (var_rb, var_rb_dn1, var_rb_dn3, var_rb_dn5, var_rb_dn8,)
    }
};
        var_rb = assign1510_e1823;
        var_rb_dn1 = assign1510_e1823_d_n1;
        var_rb_dn3 = assign1510_e1823_d_n3;
        var_rb_dn5 = assign1510_e1823_d_n5;
        var_rb_dn8 = assign1510_e1823_d_n8;

        let (assign1520_e1828, assign1520_e1828_d_n1, assign1520_e1828_d_n3, assign1520_e1828_d_n5, assign1520_e1828_d_n8,) = {
    if (var_guard13 == 0.0) {
        (var_rb, var_rb_dn1, var_rb_dn3, var_rb_dn5, var_rb_dn8,)
    } else {
        (var_rb, var_rb_dn1, var_rb_dn3, var_rb_dn5, var_rb_dn8,)
    }
};
        var_rb = assign1520_e1828;
        var_rb_dn1 = assign1520_e1828_d_n1;
        var_rb_dn3 = assign1520_e1828_d_n3;
        var_rb_dn5 = assign1520_e1828_d_n5;
        var_rb_dn8 = assign1520_e1828_d_n8;

        let assign1530_e1831: f64 = if p.p31 == 1.0 { 1.0 } else { 0.0 };
        var_guard14 = assign1530_e1831;

        let (assign1540_e1837, assign1540_e1837_d_n1, assign1540_e1837_d_n3, assign1540_e1837_d_n5, assign1540_e1837_d_n8,) = {
    if (var_guard14 != 0.0) {
        let assign1540_e1835: f64 = (var_rb + p.p13);
        (assign1540_e1835, var_rb_dn1, var_rb_dn3, var_rb_dn5, var_rb_dn8,)
    } else {
        (var_rb, var_rb_dn1, var_rb_dn3, var_rb_dn5, var_rb_dn8,)
    }
};
        var_rb = assign1540_e1837;
        var_rb_dn1 = assign1540_e1837_d_n1;
        var_rb_dn3 = assign1540_e1837_d_n3;
        var_rb_dn5 = assign1540_e1837_d_n5;
        var_rb_dn8 = assign1540_e1837_d_n8;

        let (assign1550_e1843, assign1550_e1843_d_n3,) = {
    if (var_guard14 != 0.0) {
        let assign1550_e1841: f64 = (var_rc + p.p67);
        (assign1550_e1841, var_rc_dn3,)
    } else {
        (var_rc, var_rc_dn3,)
    }
};
        var_rc = assign1550_e1843;
        var_rc_dn3 = assign1550_e1843_d_n3;

        let (assign1560_e1849, assign1560_e1849_d_n2, assign1560_e1849_d_n3, assign1560_e1849_d_n6,) = {
    if (var_guard14 != 0.0) {
        let assign1560_e1847: f64 = (var_re + p.p15);
        (assign1560_e1847, var_re_dn2, var_re_dn3, var_re_dn6,)
    } else {
        (var_re, var_re_dn2, var_re_dn3, var_re_dn6,)
    }
};
        var_re = assign1560_e1849;
        var_re_dn2 = assign1560_e1849_d_n2;
        var_re_dn3 = assign1560_e1849_d_n3;
        var_re_dn6 = assign1560_e1849_d_n6;

        let assign1570_e1852: f64 = if var_veci <= 0.0 { 1.0 } else { 0.0 };
        var_guard15 = assign1570_e1852;

        let (assign1580_e1876, assign1580_e1876_d_n2, assign1580_e1876_d_n3, assign1580_e1876_d_n4,) = {
    if (var_guard15 != 0.0) {
        let assign1580_e1856: f64 = (var_cjs_t * var_vjs_t);
        let assign1580_e1860: f64 = (1.0 - p.p76);
        let assign1580_e1864: f64 = (var_veci / var_vjs_t);
        let assign1580_e1865: f64 = (1.0 - assign1580_e1864);
        let assign1580_e1866: f64 = (assign1580_e1865).ln();
        let assign1580_e1867: f64 = (assign1580_e1860 * assign1580_e1866);
        let assign1580_e1868: f64 = (assign1580_e1867).exp();
        let assign1580_e1869: f64 = (1.0 - assign1580_e1868);
        let assign1580_e1870: f64 = (assign1580_e1856 * assign1580_e1869);
        let assign1580_e1873: f64 = (1.0 - p.p76);
        let assign1580_e1874: f64 = (assign1580_e1870 / assign1580_e1873);
        (assign1580_e1874, ((assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(var_veci_dn2 / var_vjs_t)) / assign1580_e1865))))) / assign1580_e1873), (((((var_cjs_t_dn3 * var_vjs_t) + (var_cjs_t * var_vjs_t_dn3)) * assign1580_e1869) + (assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(-((var_veci * var_vjs_t_dn3) / (var_vjs_t * var_vjs_t)))) / assign1580_e1865)))))) / assign1580_e1873), ((assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(var_veci_dn4 / var_vjs_t)) / assign1580_e1865))))) / assign1580_e1873),)
    } else {
        (var_qjs, var_qjs_dn2, var_qjs_dn3, var_qjs_dn4,)
    }
};
        var_qjs = assign1580_e1876;
        var_qjs_dn2 = assign1580_e1876_d_n2;
        var_qjs_dn3 = assign1580_e1876_d_n3;
        var_qjs_dn4 = assign1580_e1876_d_n4;

        let (assign1590_e1893, assign1590_e1893_d_n2, assign1590_e1893_d_n3, assign1590_e1893_d_n4,) = {
    if (var_guard15 == 0.0) {
        let assign1590_e1881: f64 = (var_cjs_t * var_veci);
        let assign1590_e1885: f64 = (0.5 * p.p76);
        let assign1590_e1887: f64 = (assign1590_e1885 * var_veci);
        let assign1590_e1889: f64 = (assign1590_e1887 / var_vjs_t);
        let assign1590_e1890: f64 = (1.0 + assign1590_e1889);
        let assign1590_e1891: f64 = (assign1590_e1881 * assign1590_e1890);
        (assign1590_e1891, (((var_cjs_t * var_veci_dn2) * assign1590_e1890) + (assign1590_e1881 * ((assign1590_e1885 * var_veci_dn2) / var_vjs_t))), (((var_cjs_t_dn3 * var_veci) * assign1590_e1890) + (assign1590_e1881 * (-((assign1590_e1887 * var_vjs_t_dn3) / (var_vjs_t * var_vjs_t))))), (((var_cjs_t * var_veci_dn4) * assign1590_e1890) + (assign1590_e1881 * ((assign1590_e1885 * var_veci_dn4) / var_vjs_t))),)
    } else {
        (var_qjs, var_qjs_dn2, var_qjs_dn3, var_qjs_dn4,)
    }
};
        var_qjs = assign1590_e1893;
        var_qjs_dn2 = assign1590_e1893_d_n2;
        var_qjs_dn3 = assign1590_e1893_d_n3;
        var_qjs_dn4 = assign1590_e1893_d_n4;

        let assign1600_e1895: f64 = (-var_vje_t);
        let assign1600_e1897: f64 = (assign1600_e1895 * p.p24);
        var_dv0 = assign1600_e1897;
        var_dv0_dn3 = ((-var_vje_t_dn3) * p.p24);

        let assign1610_e1900: f64 = (var_vbiei + var_dv0);
        var_dvh = assign1610_e1900;
        var_dvh_dn1 = 0.0;
        var_dvh_dn3 = var_dv0_dn3;
        var_dvh_dn4 = 0.0;
        var_dvh_dn5 = var_vbiei_dn5;
        var_dvh_dn6 = var_vbiei_dn6;

        let assign1620_e1903: f64 = if var_dvh > 0.0 { 1.0 } else { 0.0 };
        var_guard16 = assign1620_e1903;

        let (assign1630_e1916,) = {
    if (var_guard16 != 0.0) {
        let assign1630_e1906: f64 = (-1.0);
        let assign1630_e1908: f64 = (assign1630_e1906 - p.p18);
        let assign1630_e1911: f64 = (1.0 - p.p24);
        let assign1630_e1912: f64 = (assign1630_e1911).ln();
        let assign1630_e1913: f64 = (assign1630_e1908 * assign1630_e1912);
        let assign1630_e1914: f64 = (assign1630_e1913).exp();
        (assign1630_e1914,)
    } else {
        (var_pwq,)
    }
};
        var_pwq = assign1630_e1916;

        let (assign1640_e1936, assign1640_e1936_d_n1, assign1640_e1936_d_n3, assign1640_e1936_d_n4, assign1640_e1936_d_n5, assign1640_e1936_d_n6,) = {
    if (var_guard16 != 0.0) {
        let assign1640_e1923: f64 = (1.0 - p.p24);
        let assign1640_e1924: f64 = (var_pwq * assign1640_e1923);
        let assign1640_e1927: f64 = (1.0 - p.p24);
        let assign1640_e1928: f64 = (assign1640_e1924 * assign1640_e1927);
        let assign1640_e1929: f64 = (1.0 - assign1640_e1928);
        let assign1640_e1930: f64 = (var_vje_t * assign1640_e1929);
        let assign1640_e1933: f64 = (1.0 - p.p18);
        let assign1640_e1934: f64 = (assign1640_e1930 / assign1640_e1933);
        (assign1640_e1934, 0.0, ((var_vje_t_dn3 * assign1640_e1929) / assign1640_e1933), 0.0, 0.0, 0.0,)
    } else {
        (var_qlo, var_qlo_dn1, var_qlo_dn3, var_qlo_dn4, var_qlo_dn5, var_qlo_dn6,)
    }
};
        var_qlo = assign1640_e1936;
        var_qlo_dn1 = assign1640_e1936_d_n1;
        var_qlo_dn3 = assign1640_e1936_d_n3;
        var_qlo_dn4 = assign1640_e1936_d_n4;
        var_qlo_dn5 = assign1640_e1936_d_n5;
        var_qlo_dn6 = assign1640_e1936_d_n6;

        let (assign1650_e1954, assign1650_e1954_d_n1, assign1650_e1954_d_n3, assign1650_e1954_d_n4, assign1650_e1954_d_n5, assign1650_e1954_d_n6,) = {
    if (var_guard16 != 0.0) {
        let assign1650_e1941: f64 = (1.0 - p.p24);
        let assign1650_e1944: f64 = (0.5 * p.p18);
        let assign1650_e1946: f64 = (assign1650_e1944 * var_dvh);
        let assign1650_e1948: f64 = (assign1650_e1946 / var_vje_t);
        let assign1650_e1949: f64 = (assign1650_e1941 + assign1650_e1948);
        let assign1650_e1950: f64 = (var_dvh * assign1650_e1949);
        let assign1650_e1952: f64 = (assign1650_e1950 * var_pwq);
        (assign1650_e1952, (((var_dvh_dn1 * assign1650_e1949) + (var_dvh * ((assign1650_e1944 * var_dvh_dn1) / var_vje_t))) * var_pwq), (((var_dvh_dn3 * assign1650_e1949) + (var_dvh * ((((assign1650_e1944 * var_dvh_dn3) * var_vje_t) - (assign1650_e1946 * var_vje_t_dn3)) / (var_vje_t * var_vje_t)))) * var_pwq), (((var_dvh_dn4 * assign1650_e1949) + (var_dvh * ((assign1650_e1944 * var_dvh_dn4) / var_vje_t))) * var_pwq), (((var_dvh_dn5 * assign1650_e1949) + (var_dvh * ((assign1650_e1944 * var_dvh_dn5) / var_vje_t))) * var_pwq), (((var_dvh_dn6 * assign1650_e1949) + (var_dvh * ((assign1650_e1944 * var_dvh_dn6) / var_vje_t))) * var_pwq),)
    } else {
        (var_qhi, var_qhi_dn1, var_qhi_dn3, var_qhi_dn4, var_qhi_dn5, var_qhi_dn6,)
    }
};
        var_qhi = assign1650_e1954;
        var_qhi_dn1 = assign1650_e1954_d_n1;
        var_qhi_dn3 = assign1650_e1954_d_n3;
        var_qhi_dn4 = assign1650_e1954_d_n4;
        var_qhi_dn5 = assign1650_e1954_d_n5;
        var_qhi_dn6 = assign1650_e1954_d_n6;

        let (assign1660_e1977, assign1660_e1977_d_n1, assign1660_e1977_d_n3, assign1660_e1977_d_n4, assign1660_e1977_d_n5, assign1660_e1977_d_n6,) = {
    if (var_guard16 == 0.0) {
        let assign1660_e1961: f64 = (1.0 - p.p18);
        let assign1660_e1965: f64 = (var_vbiei / var_vje_t);
        let assign1660_e1966: f64 = (1.0 - assign1660_e1965);
        let assign1660_e1967: f64 = (assign1660_e1966).ln();
        let assign1660_e1968: f64 = (assign1660_e1961 * assign1660_e1967);
        let assign1660_e1969: f64 = (assign1660_e1968).exp();
        let assign1660_e1970: f64 = (1.0 - assign1660_e1969);
        let assign1660_e1971: f64 = (var_vje_t * assign1660_e1970);
        let assign1660_e1974: f64 = (1.0 - p.p18);
        let assign1660_e1975: f64 = (assign1660_e1971 / assign1660_e1974);
        (assign1660_e1975, 0.0, (((var_vje_t_dn3 * assign1660_e1970) + (var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(-((var_vbiei * var_vje_t_dn3) / (var_vje_t * var_vje_t)))) / assign1660_e1966)))))) / assign1660_e1974), 0.0, ((var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(var_vbiei_dn5 / var_vje_t)) / assign1660_e1966))))) / assign1660_e1974), ((var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(var_vbiei_dn6 / var_vje_t)) / assign1660_e1966))))) / assign1660_e1974),)
    } else {
        (var_qlo, var_qlo_dn1, var_qlo_dn3, var_qlo_dn4, var_qlo_dn5, var_qlo_dn6,)
    }
};
        var_qlo = assign1660_e1977;
        var_qlo_dn1 = assign1660_e1977_d_n1;
        var_qlo_dn3 = assign1660_e1977_d_n3;
        var_qlo_dn4 = assign1660_e1977_d_n4;
        var_qlo_dn5 = assign1660_e1977_d_n5;
        var_qlo_dn6 = assign1660_e1977_d_n6;

        let (assign1670_e1982, assign1670_e1982_d_n1, assign1670_e1982_d_n3, assign1670_e1982_d_n4, assign1670_e1982_d_n5, assign1670_e1982_d_n6,) = {
    if (var_guard16 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qhi, var_qhi_dn1, var_qhi_dn3, var_qhi_dn4, var_qhi_dn5, var_qhi_dn6,)
    }
};
        var_qhi = assign1670_e1982;
        var_qhi_dn1 = assign1670_e1982_d_n1;
        var_qhi_dn3 = assign1670_e1982_d_n3;
        var_qhi_dn4 = assign1670_e1982_d_n4;
        var_qhi_dn5 = assign1670_e1982_d_n5;
        var_qhi_dn6 = assign1670_e1982_d_n6;

        let assign1680_e1986: f64 = (var_qlo + var_qhi);
        let assign1680_e1987: f64 = (var_cje_t * assign1680_e1986);
        var_qje = assign1680_e1987;
        var_qje_dn1 = (var_cje_t * (var_qlo_dn1 + var_qhi_dn1));
        var_qje_dn3 = ((var_cje_t_dn3 * assign1680_e1986) + (var_cje_t * (var_qlo_dn3 + var_qhi_dn3)));
        var_qje_dn4 = (var_cje_t * (var_qlo_dn4 + var_qhi_dn4));
        var_qje_dn5 = (var_cje_t * (var_qlo_dn5 + var_qhi_dn5));
        var_qje_dn6 = (var_cje_t * (var_qlo_dn6 + var_qhi_dn6));

        let assign1690_e1989: f64 = (-var_vjc_t);
        let assign1690_e1991: f64 = (assign1690_e1989 * p.p24);
        var_dv0 = assign1690_e1991;
        var_dv0_dn3 = ((-var_vjc_t_dn3) * p.p24);

        let assign1700_e1994: f64 = (var_vbci + var_dv0);
        var_dvh = assign1700_e1994;
        var_dvh_dn1 = var_vbci_dn1;
        var_dvh_dn3 = var_dv0_dn3;
        var_dvh_dn4 = var_vbci_dn4;
        var_dvh_dn5 = 0.0;
        var_dvh_dn6 = 0.0;

        let assign1710_e1997: f64 = if var_dvh > 0.0 { 1.0 } else { 0.0 };
        var_guard17 = assign1710_e1997;

        let (assign1720_e2010,) = {
    if (var_guard17 != 0.0) {
        let assign1720_e2000: f64 = (-1.0);
        let assign1720_e2002: f64 = (assign1720_e2000 - p.p71);
        let assign1720_e2005: f64 = (1.0 - p.p24);
        let assign1720_e2006: f64 = (assign1720_e2005).ln();
        let assign1720_e2007: f64 = (assign1720_e2002 * assign1720_e2006);
        let assign1720_e2008: f64 = (assign1720_e2007).exp();
        (assign1720_e2008,)
    } else {
        (var_pwq,)
    }
};
        var_pwq = assign1720_e2010;

        let (assign1730_e2030, assign1730_e2030_d_n1, assign1730_e2030_d_n3, assign1730_e2030_d_n4, assign1730_e2030_d_n5, assign1730_e2030_d_n6,) = {
    if (var_guard17 != 0.0) {
        let assign1730_e2017: f64 = (1.0 - p.p24);
        let assign1730_e2018: f64 = (var_pwq * assign1730_e2017);
        let assign1730_e2021: f64 = (1.0 - p.p24);
        let assign1730_e2022: f64 = (assign1730_e2018 * assign1730_e2021);
        let assign1730_e2023: f64 = (1.0 - assign1730_e2022);
        let assign1730_e2024: f64 = (var_vjc_t * assign1730_e2023);
        let assign1730_e2027: f64 = (1.0 - p.p71);
        let assign1730_e2028: f64 = (assign1730_e2024 / assign1730_e2027);
        (assign1730_e2028, 0.0, ((var_vjc_t_dn3 * assign1730_e2023) / assign1730_e2027), 0.0, 0.0, 0.0,)
    } else {
        (var_qlo, var_qlo_dn1, var_qlo_dn3, var_qlo_dn4, var_qlo_dn5, var_qlo_dn6,)
    }
};
        var_qlo = assign1730_e2030;
        var_qlo_dn1 = assign1730_e2030_d_n1;
        var_qlo_dn3 = assign1730_e2030_d_n3;
        var_qlo_dn4 = assign1730_e2030_d_n4;
        var_qlo_dn5 = assign1730_e2030_d_n5;
        var_qlo_dn6 = assign1730_e2030_d_n6;

        let (assign1740_e2048, assign1740_e2048_d_n1, assign1740_e2048_d_n3, assign1740_e2048_d_n4, assign1740_e2048_d_n5, assign1740_e2048_d_n6,) = {
    if (var_guard17 != 0.0) {
        let assign1740_e2035: f64 = (1.0 - p.p24);
        let assign1740_e2038: f64 = (0.5 * p.p71);
        let assign1740_e2040: f64 = (assign1740_e2038 * var_dvh);
        let assign1740_e2042: f64 = (assign1740_e2040 / var_vjc_t);
        let assign1740_e2043: f64 = (assign1740_e2035 + assign1740_e2042);
        let assign1740_e2044: f64 = (var_dvh * assign1740_e2043);
        let assign1740_e2046: f64 = (assign1740_e2044 * var_pwq);
        (assign1740_e2046, (((var_dvh_dn1 * assign1740_e2043) + (var_dvh * ((assign1740_e2038 * var_dvh_dn1) / var_vjc_t))) * var_pwq), (((var_dvh_dn3 * assign1740_e2043) + (var_dvh * ((((assign1740_e2038 * var_dvh_dn3) * var_vjc_t) - (assign1740_e2040 * var_vjc_t_dn3)) / (var_vjc_t * var_vjc_t)))) * var_pwq), (((var_dvh_dn4 * assign1740_e2043) + (var_dvh * ((assign1740_e2038 * var_dvh_dn4) / var_vjc_t))) * var_pwq), (((var_dvh_dn5 * assign1740_e2043) + (var_dvh * ((assign1740_e2038 * var_dvh_dn5) / var_vjc_t))) * var_pwq), (((var_dvh_dn6 * assign1740_e2043) + (var_dvh * ((assign1740_e2038 * var_dvh_dn6) / var_vjc_t))) * var_pwq),)
    } else {
        (var_qhi, var_qhi_dn1, var_qhi_dn3, var_qhi_dn4, var_qhi_dn5, var_qhi_dn6,)
    }
};
        var_qhi = assign1740_e2048;
        var_qhi_dn1 = assign1740_e2048_d_n1;
        var_qhi_dn3 = assign1740_e2048_d_n3;
        var_qhi_dn4 = assign1740_e2048_d_n4;
        var_qhi_dn5 = assign1740_e2048_d_n5;
        var_qhi_dn6 = assign1740_e2048_d_n6;

        let (assign1750_e2071, assign1750_e2071_d_n1, assign1750_e2071_d_n3, assign1750_e2071_d_n4, assign1750_e2071_d_n5, assign1750_e2071_d_n6,) = {
    if (var_guard17 == 0.0) {
        let assign1750_e2055: f64 = (1.0 - p.p71);
        let assign1750_e2059: f64 = (var_vbci / var_vjc_t);
        let assign1750_e2060: f64 = (1.0 - assign1750_e2059);
        let assign1750_e2061: f64 = (assign1750_e2060).ln();
        let assign1750_e2062: f64 = (assign1750_e2055 * assign1750_e2061);
        let assign1750_e2063: f64 = (assign1750_e2062).exp();
        let assign1750_e2064: f64 = (1.0 - assign1750_e2063);
        let assign1750_e2065: f64 = (var_vjc_t * assign1750_e2064);
        let assign1750_e2068: f64 = (1.0 - p.p71);
        let assign1750_e2069: f64 = (assign1750_e2065 / assign1750_e2068);
        (assign1750_e2069, ((var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(var_vbci_dn1 / var_vjc_t)) / assign1750_e2060))))) / assign1750_e2068), (((var_vjc_t_dn3 * assign1750_e2064) + (var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(-((var_vbci * var_vjc_t_dn3) / (var_vjc_t * var_vjc_t)))) / assign1750_e2060)))))) / assign1750_e2068), ((var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(var_vbci_dn4 / var_vjc_t)) / assign1750_e2060))))) / assign1750_e2068), 0.0, 0.0,)
    } else {
        (var_qlo, var_qlo_dn1, var_qlo_dn3, var_qlo_dn4, var_qlo_dn5, var_qlo_dn6,)
    }
};
        var_qlo = assign1750_e2071;
        var_qlo_dn1 = assign1750_e2071_d_n1;
        var_qlo_dn3 = assign1750_e2071_d_n3;
        var_qlo_dn4 = assign1750_e2071_d_n4;
        var_qlo_dn5 = assign1750_e2071_d_n5;
        var_qlo_dn6 = assign1750_e2071_d_n6;

        let (assign1760_e2076, assign1760_e2076_d_n1, assign1760_e2076_d_n3, assign1760_e2076_d_n4, assign1760_e2076_d_n5, assign1760_e2076_d_n6,) = {
    if (var_guard17 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qhi, var_qhi_dn1, var_qhi_dn3, var_qhi_dn4, var_qhi_dn5, var_qhi_dn6,)
    }
};
        var_qhi = assign1760_e2076;
        var_qhi_dn1 = assign1760_e2076_d_n1;
        var_qhi_dn3 = assign1760_e2076_d_n3;
        var_qhi_dn4 = assign1760_e2076_d_n4;
        var_qhi_dn5 = assign1760_e2076_d_n5;
        var_qhi_dn6 = assign1760_e2076_d_n6;

        let assign1770_e2080: f64 = (var_qlo + var_qhi);
        let assign1770_e2081: f64 = (var_cjc_t * assign1770_e2080);
        var_qjcx = assign1770_e2081;
        var_qjcx_dn1 = (var_cjc_t * (var_qlo_dn1 + var_qhi_dn1));
        var_qjcx_dn3 = ((var_cjc_t_dn3 * assign1770_e2080) + (var_cjc_t * (var_qlo_dn3 + var_qhi_dn3)));
        var_qjcx_dn4 = (var_cjc_t * (var_qlo_dn4 + var_qhi_dn4));
        var_qjcx_dn5 = (var_cjc_t * (var_qlo_dn5 + var_qhi_dn5));
        var_qjcx_dn6 = (var_cjc_t * (var_qlo_dn6 + var_qhi_dn6));

        let assign1780_e2084: f64 = (1.0 - p.p72);
        let assign1780_e2086: f64 = (assign1780_e2084 * var_qjcx);
        var_qjcx_1 = assign1780_e2086;
        var_qjcx_1_dn1 = (assign1780_e2084 * var_qjcx_dn1);
        var_qjcx_1_dn3 = (assign1780_e2084 * var_qjcx_dn3);
        var_qjcx_1_dn4 = (assign1780_e2084 * var_qjcx_dn4);
        var_qjcx_1_dn5 = (assign1780_e2084 * var_qjcx_dn5);
        var_qjcx_1_dn6 = (assign1780_e2084 * var_qjcx_dn6);

        let assign1790_e2088: f64 = (-var_vjc_t);
        let assign1790_e2090: f64 = (assign1790_e2088 * p.p24);
        var_dv0 = assign1790_e2090;
        var_dv0_dn3 = ((-var_vjc_t_dn3) * p.p24);

        let assign1800_e2093: f64 = (var_vbici + var_dv0);
        var_dvh = assign1800_e2093;
        var_dvh_dn1 = 0.0;
        var_dvh_dn3 = var_dv0_dn3;
        var_dvh_dn4 = var_vbici_dn4;
        var_dvh_dn5 = var_vbici_dn5;
        var_dvh_dn6 = 0.0;

        let assign1810_e2096: f64 = if var_dvh > 0.0 { 1.0 } else { 0.0 };
        var_guard18 = assign1810_e2096;

        let (assign1820_e2109,) = {
    if (var_guard18 != 0.0) {
        let assign1820_e2099: f64 = (-1.0);
        let assign1820_e2101: f64 = (assign1820_e2099 - p.p71);
        let assign1820_e2104: f64 = (1.0 - p.p24);
        let assign1820_e2105: f64 = (assign1820_e2104).ln();
        let assign1820_e2106: f64 = (assign1820_e2101 * assign1820_e2105);
        let assign1820_e2107: f64 = (assign1820_e2106).exp();
        (assign1820_e2107,)
    } else {
        (var_pwq,)
    }
};
        var_pwq = assign1820_e2109;

        let (assign1830_e2129, assign1830_e2129_d_n1, assign1830_e2129_d_n3, assign1830_e2129_d_n4, assign1830_e2129_d_n5, assign1830_e2129_d_n6,) = {
    if (var_guard18 != 0.0) {
        let assign1830_e2116: f64 = (1.0 - p.p24);
        let assign1830_e2117: f64 = (var_pwq * assign1830_e2116);
        let assign1830_e2120: f64 = (1.0 - p.p24);
        let assign1830_e2121: f64 = (assign1830_e2117 * assign1830_e2120);
        let assign1830_e2122: f64 = (1.0 - assign1830_e2121);
        let assign1830_e2123: f64 = (var_vjc_t * assign1830_e2122);
        let assign1830_e2126: f64 = (1.0 - p.p71);
        let assign1830_e2127: f64 = (assign1830_e2123 / assign1830_e2126);
        (assign1830_e2127, 0.0, ((var_vjc_t_dn3 * assign1830_e2122) / assign1830_e2126), 0.0, 0.0, 0.0,)
    } else {
        (var_qlo, var_qlo_dn1, var_qlo_dn3, var_qlo_dn4, var_qlo_dn5, var_qlo_dn6,)
    }
};
        var_qlo = assign1830_e2129;
        var_qlo_dn1 = assign1830_e2129_d_n1;
        var_qlo_dn3 = assign1830_e2129_d_n3;
        var_qlo_dn4 = assign1830_e2129_d_n4;
        var_qlo_dn5 = assign1830_e2129_d_n5;
        var_qlo_dn6 = assign1830_e2129_d_n6;

        *var_dv0_slot = var_dv0;
        *var_dv0_dn3_slot = var_dv0_dn3;
        *var_dvh_slot = var_dvh;
        *var_dvh_dn1_slot = var_dvh_dn1;
        *var_dvh_dn3_slot = var_dvh_dn3;
        *var_dvh_dn4_slot = var_dvh_dn4;
        *var_dvh_dn5_slot = var_dvh_dn5;
        *var_dvh_dn6_slot = var_dvh_dn6;
        *var_guard13_slot = var_guard13;
        *var_guard14_slot = var_guard14;
        *var_guard15_slot = var_guard15;
        *var_guard16_slot = var_guard16;
        *var_guard17_slot = var_guard17;
        *var_guard18_slot = var_guard18;
        *var_pwq_slot = var_pwq;
        *var_qdc_slot = var_qdc;
        *var_qdc_dn3_slot = var_qdc_dn3;
        *var_qdc_dn4_slot = var_qdc_dn4;
        *var_qdc_dn5_slot = var_qdc_dn5;
        *var_qdc_dn6_slot = var_qdc_dn6;
        *var_qde_slot = var_qde;
        *var_qde_dn1_slot = var_qde_dn1;
        *var_qde_dn2_slot = var_qde_dn2;
        *var_qde_dn3_slot = var_qde_dn3;
        *var_qde_dn4_slot = var_qde_dn4;
        *var_qde_dn5_slot = var_qde_dn5;
        *var_qde_dn6_slot = var_qde_dn6;
        *var_qhi_slot = var_qhi;
        *var_qhi_dn1_slot = var_qhi_dn1;
        *var_qhi_dn3_slot = var_qhi_dn3;
        *var_qhi_dn4_slot = var_qhi_dn4;
        *var_qhi_dn5_slot = var_qhi_dn5;
        *var_qhi_dn6_slot = var_qhi_dn6;
        *var_qjcx_slot = var_qjcx;
        *var_qjcx_1_slot = var_qjcx_1;
        *var_qjcx_1_dn1_slot = var_qjcx_1_dn1;
        *var_qjcx_1_dn3_slot = var_qjcx_1_dn3;
        *var_qjcx_1_dn4_slot = var_qjcx_1_dn4;
        *var_qjcx_1_dn5_slot = var_qjcx_1_dn5;
        *var_qjcx_1_dn6_slot = var_qjcx_1_dn6;
        *var_qjcx_dn1_slot = var_qjcx_dn1;
        *var_qjcx_dn3_slot = var_qjcx_dn3;
        *var_qjcx_dn4_slot = var_qjcx_dn4;
        *var_qjcx_dn5_slot = var_qjcx_dn5;
        *var_qjcx_dn6_slot = var_qjcx_dn6;
        *var_qje_slot = var_qje;
        *var_qje_dn1_slot = var_qje_dn1;
        *var_qje_dn3_slot = var_qje_dn3;
        *var_qje_dn4_slot = var_qje_dn4;
        *var_qje_dn5_slot = var_qje_dn5;
        *var_qje_dn6_slot = var_qje_dn6;
        *var_qjs_slot = var_qjs;
        *var_qjs_dn2_slot = var_qjs_dn2;
        *var_qjs_dn3_slot = var_qjs_dn3;
        *var_qjs_dn4_slot = var_qjs_dn4;
        *var_qlo_slot = var_qlo;
        *var_qlo_dn1_slot = var_qlo_dn1;
        *var_qlo_dn3_slot = var_qlo_dn3;
        *var_qlo_dn4_slot = var_qlo_dn4;
        *var_qlo_dn5_slot = var_qlo_dn5;
        *var_qlo_dn6_slot = var_qlo_dn6;
        *var_rb_slot = var_rb;
        *var_rb_dn1_slot = var_rb_dn1;
        *var_rb_dn3_slot = var_rb_dn3;
        *var_rb_dn5_slot = var_rb_dn5;
        *var_rb_dn8_slot = var_rb_dn8;
        *var_rc_slot = var_rc;
        *var_rc_dn3_slot = var_rc_dn3;
        *var_re_slot = var_re;
        *var_re_dn2_slot = var_re_dn2;
        *var_re_dn3_slot = var_re_dn3;
        *var_re_dn6_slot = var_re_dn6;
        *var_tff_slot = var_tff;
        *var_tff_dn1_slot = var_tff_dn1;
        *var_tff_dn2_slot = var_tff_dn2;
        *var_veesat_slot = var_veesat;
        *var_veesat_dn2_slot = var_veesat_dn2;
        *var_veesat_dn6_slot = var_veesat_dn6;
        *var_vtff_slot = var_vtff;
        *var_vtff1_slot = var_vtff1;
        *var_vtff1_dn1_slot = var_vtff1_dn1;
        *var_vtff1_dn2_slot = var_vtff1_dn2;
        *var_vtff_dn1_slot = var_vtff_dn1;
        *var_vtff_dn2_slot = var_vtff_dn2;
    }

    pub(super) fn stamp_transient_block_4(
        p: &Parameters,
        var_cjc_t: f64,
        var_cjc_t_dn3: f64,
        var_dvh: f64,
        var_dvh_dn1: f64,
        var_dvh_dn3: f64,
        var_dvh_dn4: f64,
        var_dvh_dn5: f64,
        var_dvh_dn6: f64,
        var_guard18: f64,
        var_itzf: f64,
        var_itzf_dn3: f64,
        var_itzf_dn4: f64,
        var_itzf_dn5: f64,
        var_itzf_dn6: f64,
        var_pwq: f64,
        var_ttype: f64,
        var_vbici: f64,
        var_vbici_dn4: f64,
        var_vbici_dn5: f64,
        var_vjc_t: f64,
        var_vjc_t_dn3: f64,
        var_weff: f64,
        var_guard19_slot: &mut f64,
        var_guard20_slot: &mut f64,
        var_guard21_slot: &mut f64,
        var_guard22_slot: &mut f64,
        var_guard23_slot: &mut f64,
        var_guard24_slot: &mut f64,
        var_guard25_slot: &mut f64,
        var_qhi_slot: &mut f64,
        var_qhi_dn1_slot: &mut f64,
        var_qhi_dn3_slot: &mut f64,
        var_qhi_dn4_slot: &mut f64,
        var_qhi_dn5_slot: &mut f64,
        var_qhi_dn6_slot: &mut f64,
        var_qjci_slot: &mut f64,
        var_qjci_1_slot: &mut f64,
        var_qjci_1_dn1_slot: &mut f64,
        var_qjci_1_dn3_slot: &mut f64,
        var_qjci_1_dn4_slot: &mut f64,
        var_qjci_1_dn5_slot: &mut f64,
        var_qjci_1_dn6_slot: &mut f64,
        var_qjci_dn1_slot: &mut f64,
        var_qjci_dn3_slot: &mut f64,
        var_qjci_dn4_slot: &mut f64,
        var_qjci_dn5_slot: &mut f64,
        var_qjci_dn6_slot: &mut f64,
        var_qlo_slot: &mut f64,
        var_qlo_dn1_slot: &mut f64,
        var_qlo_dn3_slot: &mut f64,
        var_qlo_dn4_slot: &mut f64,
        var_qlo_dn5_slot: &mut f64,
        var_qlo_dn6_slot: &mut f64,
        var_qxf1_slot: &mut f64,
        var_qxf1_dn3_slot: &mut f64,
        var_qxf1_dn4_slot: &mut f64,
        var_qxf1_dn5_slot: &mut f64,
        var_qxf1_dn6_slot: &mut f64,
        var_rb_nom_slot: &mut f64,
        var_rc_nom_slot: &mut f64,
        var_re_nom_slot: &mut f64,
    ) {
        let mut var_guard19: f64 = *var_guard19_slot;
        let mut var_guard20: f64 = *var_guard20_slot;
        let mut var_guard21: f64 = *var_guard21_slot;
        let mut var_guard22: f64 = *var_guard22_slot;
        let mut var_guard23: f64 = *var_guard23_slot;
        let mut var_guard24: f64 = *var_guard24_slot;
        let mut var_guard25: f64 = *var_guard25_slot;
        let mut var_qhi: f64 = *var_qhi_slot;
        let mut var_qhi_dn1: f64 = *var_qhi_dn1_slot;
        let mut var_qhi_dn3: f64 = *var_qhi_dn3_slot;
        let mut var_qhi_dn4: f64 = *var_qhi_dn4_slot;
        let mut var_qhi_dn5: f64 = *var_qhi_dn5_slot;
        let mut var_qhi_dn6: f64 = *var_qhi_dn6_slot;
        let mut var_qjci: f64 = *var_qjci_slot;
        let mut var_qjci_1: f64 = *var_qjci_1_slot;
        let mut var_qjci_1_dn1: f64 = *var_qjci_1_dn1_slot;
        let mut var_qjci_1_dn3: f64 = *var_qjci_1_dn3_slot;
        let mut var_qjci_1_dn4: f64 = *var_qjci_1_dn4_slot;
        let mut var_qjci_1_dn5: f64 = *var_qjci_1_dn5_slot;
        let mut var_qjci_1_dn6: f64 = *var_qjci_1_dn6_slot;
        let mut var_qjci_dn1: f64 = *var_qjci_dn1_slot;
        let mut var_qjci_dn3: f64 = *var_qjci_dn3_slot;
        let mut var_qjci_dn4: f64 = *var_qjci_dn4_slot;
        let mut var_qjci_dn5: f64 = *var_qjci_dn5_slot;
        let mut var_qjci_dn6: f64 = *var_qjci_dn6_slot;
        let mut var_qlo: f64 = *var_qlo_slot;
        let mut var_qlo_dn1: f64 = *var_qlo_dn1_slot;
        let mut var_qlo_dn3: f64 = *var_qlo_dn3_slot;
        let mut var_qlo_dn4: f64 = *var_qlo_dn4_slot;
        let mut var_qlo_dn5: f64 = *var_qlo_dn5_slot;
        let mut var_qlo_dn6: f64 = *var_qlo_dn6_slot;
        let mut var_qxf1: f64 = *var_qxf1_slot;
        let mut var_qxf1_dn3: f64 = *var_qxf1_dn3_slot;
        let mut var_qxf1_dn4: f64 = *var_qxf1_dn4_slot;
        let mut var_qxf1_dn5: f64 = *var_qxf1_dn5_slot;
        let mut var_qxf1_dn6: f64 = *var_qxf1_dn6_slot;
        let mut var_rb_nom: f64 = *var_rb_nom_slot;
        let mut var_rc_nom: f64 = *var_rc_nom_slot;
        let mut var_re_nom: f64 = *var_re_nom_slot;

        let (assign1840_e2147, assign1840_e2147_d_n1, assign1840_e2147_d_n3, assign1840_e2147_d_n4, assign1840_e2147_d_n5, assign1840_e2147_d_n6,) = {
    if (var_guard18 != 0.0) {
        let assign1840_e2134: f64 = (1.0 - p.p24);
        let assign1840_e2137: f64 = (0.5 * p.p71);
        let assign1840_e2139: f64 = (assign1840_e2137 * var_dvh);
        let assign1840_e2141: f64 = (assign1840_e2139 / var_vjc_t);
        let assign1840_e2142: f64 = (assign1840_e2134 + assign1840_e2141);
        let assign1840_e2143: f64 = (var_dvh * assign1840_e2142);
        let assign1840_e2145: f64 = (assign1840_e2143 * var_pwq);
        (assign1840_e2145, (((var_dvh_dn1 * assign1840_e2142) + (var_dvh * ((assign1840_e2137 * var_dvh_dn1) / var_vjc_t))) * var_pwq), (((var_dvh_dn3 * assign1840_e2142) + (var_dvh * ((((assign1840_e2137 * var_dvh_dn3) * var_vjc_t) - (assign1840_e2139 * var_vjc_t_dn3)) / (var_vjc_t * var_vjc_t)))) * var_pwq), (((var_dvh_dn4 * assign1840_e2142) + (var_dvh * ((assign1840_e2137 * var_dvh_dn4) / var_vjc_t))) * var_pwq), (((var_dvh_dn5 * assign1840_e2142) + (var_dvh * ((assign1840_e2137 * var_dvh_dn5) / var_vjc_t))) * var_pwq), (((var_dvh_dn6 * assign1840_e2142) + (var_dvh * ((assign1840_e2137 * var_dvh_dn6) / var_vjc_t))) * var_pwq),)
    } else {
        (var_qhi, var_qhi_dn1, var_qhi_dn3, var_qhi_dn4, var_qhi_dn5, var_qhi_dn6,)
    }
};
        var_qhi = assign1840_e2147;
        var_qhi_dn1 = assign1840_e2147_d_n1;
        var_qhi_dn3 = assign1840_e2147_d_n3;
        var_qhi_dn4 = assign1840_e2147_d_n4;
        var_qhi_dn5 = assign1840_e2147_d_n5;
        var_qhi_dn6 = assign1840_e2147_d_n6;

        let (assign1850_e2170, assign1850_e2170_d_n1, assign1850_e2170_d_n3, assign1850_e2170_d_n4, assign1850_e2170_d_n5, assign1850_e2170_d_n6,) = {
    if (var_guard18 == 0.0) {
        let assign1850_e2154: f64 = (1.0 - p.p71);
        let assign1850_e2158: f64 = (var_vbici / var_vjc_t);
        let assign1850_e2159: f64 = (1.0 - assign1850_e2158);
        let assign1850_e2160: f64 = (assign1850_e2159).ln();
        let assign1850_e2161: f64 = (assign1850_e2154 * assign1850_e2160);
        let assign1850_e2162: f64 = (assign1850_e2161).exp();
        let assign1850_e2163: f64 = (1.0 - assign1850_e2162);
        let assign1850_e2164: f64 = (var_vjc_t * assign1850_e2163);
        let assign1850_e2167: f64 = (1.0 - p.p71);
        let assign1850_e2168: f64 = (assign1850_e2164 / assign1850_e2167);
        (assign1850_e2168, 0.0, (((var_vjc_t_dn3 * assign1850_e2163) + (var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(-((var_vbici * var_vjc_t_dn3) / (var_vjc_t * var_vjc_t)))) / assign1850_e2159)))))) / assign1850_e2167), ((var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(var_vbici_dn4 / var_vjc_t)) / assign1850_e2159))))) / assign1850_e2167), ((var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(var_vbici_dn5 / var_vjc_t)) / assign1850_e2159))))) / assign1850_e2167), 0.0,)
    } else {
        (var_qlo, var_qlo_dn1, var_qlo_dn3, var_qlo_dn4, var_qlo_dn5, var_qlo_dn6,)
    }
};
        var_qlo = assign1850_e2170;
        var_qlo_dn1 = assign1850_e2170_d_n1;
        var_qlo_dn3 = assign1850_e2170_d_n3;
        var_qlo_dn4 = assign1850_e2170_d_n4;
        var_qlo_dn5 = assign1850_e2170_d_n5;
        var_qlo_dn6 = assign1850_e2170_d_n6;

        let (assign1860_e2175, assign1860_e2175_d_n1, assign1860_e2175_d_n3, assign1860_e2175_d_n4, assign1860_e2175_d_n5, assign1860_e2175_d_n6,) = {
    if (var_guard18 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qhi, var_qhi_dn1, var_qhi_dn3, var_qhi_dn4, var_qhi_dn5, var_qhi_dn6,)
    }
};
        var_qhi = assign1860_e2175;
        var_qhi_dn1 = assign1860_e2175_d_n1;
        var_qhi_dn3 = assign1860_e2175_d_n3;
        var_qhi_dn4 = assign1860_e2175_d_n4;
        var_qhi_dn5 = assign1860_e2175_d_n5;
        var_qhi_dn6 = assign1860_e2175_d_n6;

        let assign1870_e2179: f64 = (var_qlo + var_qhi);
        let assign1870_e2180: f64 = (var_cjc_t * assign1870_e2179);
        var_qjci = assign1870_e2180;
        var_qjci_dn1 = (var_cjc_t * (var_qlo_dn1 + var_qhi_dn1));
        var_qjci_dn3 = ((var_cjc_t_dn3 * assign1870_e2179) + (var_cjc_t * (var_qlo_dn3 + var_qhi_dn3)));
        var_qjci_dn4 = (var_cjc_t * (var_qlo_dn4 + var_qhi_dn4));
        var_qjci_dn5 = (var_cjc_t * (var_qlo_dn5 + var_qhi_dn5));
        var_qjci_dn6 = (var_cjc_t * (var_qlo_dn6 + var_qhi_dn6));

        let assign1880_e2183: f64 = (p.p72 * var_qjci);
        var_qjci_1 = assign1880_e2183;
        var_qjci_1_dn1 = (p.p72 * var_qjci_dn1);
        var_qjci_1_dn3 = (p.p72 * var_qjci_dn3);
        var_qjci_1_dn4 = (p.p72 * var_qjci_dn4);
        var_qjci_1_dn5 = (p.p72 * var_qjci_dn5);
        var_qjci_1_dn6 = (p.p72 * var_qjci_dn6);

        let assign1890_e2190: f64 = if ((p.p68 != 0.0) && (p.p19 != 0.0)) { 1.0 } else { 0.0 };
        var_guard19 = assign1890_e2190;

        let (assign1900_e2204, assign1900_e2204_d_n3, assign1900_e2204_d_n4, assign1900_e2204_d_n5, assign1900_e2204_d_n6,) = {
    if (var_guard19 != 0.0) {
        let assign1900_e2194: f64 = (var_ttype * p.p68);
        let assign1900_e2196: f64 = (assign1900_e2194 * 3.141592653589793);
        let assign1900_e2198: f64 = (assign1900_e2196 / 180.0);
        let assign1900_e2200: f64 = (assign1900_e2198 * p.p19);
        let assign1900_e2202: f64 = (assign1900_e2200 * var_itzf);
        (assign1900_e2202, (assign1900_e2200 * var_itzf_dn3), (assign1900_e2200 * var_itzf_dn4), (assign1900_e2200 * var_itzf_dn5), (assign1900_e2200 * var_itzf_dn6),)
    } else {
        (var_qxf1, var_qxf1_dn3, var_qxf1_dn4, var_qxf1_dn5, var_qxf1_dn6,)
    }
};
        var_qxf1 = assign1900_e2204;
        var_qxf1_dn3 = assign1900_e2204_d_n3;
        var_qxf1_dn4 = assign1900_e2204_d_n4;
        var_qxf1_dn5 = assign1900_e2204_d_n5;
        var_qxf1_dn6 = assign1900_e2204_d_n6;

        let (assign1910_e2209, assign1910_e2209_d_n3, assign1910_e2209_d_n4, assign1910_e2209_d_n5, assign1910_e2209_d_n6,) = {
    if (var_guard19 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qxf1, var_qxf1_dn3, var_qxf1_dn4, var_qxf1_dn5, var_qxf1_dn6,)
    }
};
        var_qxf1 = assign1910_e2209;
        var_qxf1_dn3 = assign1910_e2209_d_n3;
        var_qxf1_dn4 = assign1910_e2209_d_n4;
        var_qxf1_dn5 = assign1910_e2209_d_n5;
        var_qxf1_dn6 = assign1910_e2209_d_n6;

        let assign1920_e2216: f64 = if ((p.p30 == 1.0) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };
        var_guard20 = assign1920_e2216;

        let assign1930_e2227: f64 = if (((p.p30 == 2.0) && (p.p33 > 0.0)) && (p.p35 > 0.0)) { 1.0 } else { 0.0 };
        var_guard21 = assign1930_e2227;

        let assign1940_e2230: f64 = (-1.0);
        let assign1940_e2231: f64 = if p.p30 == assign1940_e2230 { 1.0 } else { 0.0 };
        var_guard22 = assign1940_e2231;

        let assign1960_e2240: f64 = (p.p31 * p.p13);
        let assign1960_e2241: f64 = (p.p12 + assign1960_e2240);
        let assign1960_e2243: f64 = (assign1960_e2241 / var_weff);
        var_rb_nom = assign1960_e2243;

        let assign1970_e2247: f64 = (p.p31 * p.p15);
        let assign1970_e2248: f64 = (p.p14 + assign1970_e2247);
        let assign1970_e2250: f64 = (assign1970_e2248 / var_weff);
        var_re_nom = assign1970_e2250;

        let assign1980_e2254: f64 = (p.p31 * p.p67);
        let assign1980_e2255: f64 = (p.p66 + assign1980_e2254);
        let assign1980_e2257: f64 = (assign1980_e2255 / var_weff);
        var_rc_nom = assign1980_e2257;

        let assign1990_e2264: f64 = if ((var_rb_nom > 0.0) && (var_rb_nom >= p.p46)) { 1.0 } else { 0.0 };
        var_guard23 = assign1990_e2264;

        let assign2010_e2286: f64 = if ((var_re_nom > 0.0) && (var_re_nom >= p.p46)) { 1.0 } else { 0.0 };
        var_guard24 = assign2010_e2286;

        let assign2030_e2308: f64 = if ((var_rc_nom > 0.0) && (var_rc_nom >= p.p46)) { 1.0 } else { 0.0 };
        var_guard25 = assign2030_e2308;

        *var_guard19_slot = var_guard19;
        *var_guard20_slot = var_guard20;
        *var_guard21_slot = var_guard21;
        *var_guard22_slot = var_guard22;
        *var_guard23_slot = var_guard23;
        *var_guard24_slot = var_guard24;
        *var_guard25_slot = var_guard25;
        *var_qhi_slot = var_qhi;
        *var_qhi_dn1_slot = var_qhi_dn1;
        *var_qhi_dn3_slot = var_qhi_dn3;
        *var_qhi_dn4_slot = var_qhi_dn4;
        *var_qhi_dn5_slot = var_qhi_dn5;
        *var_qhi_dn6_slot = var_qhi_dn6;
        *var_qjci_slot = var_qjci;
        *var_qjci_1_slot = var_qjci_1;
        *var_qjci_1_dn1_slot = var_qjci_1_dn1;
        *var_qjci_1_dn3_slot = var_qjci_1_dn3;
        *var_qjci_1_dn4_slot = var_qjci_1_dn4;
        *var_qjci_1_dn5_slot = var_qjci_1_dn5;
        *var_qjci_1_dn6_slot = var_qjci_1_dn6;
        *var_qjci_dn1_slot = var_qjci_dn1;
        *var_qjci_dn3_slot = var_qjci_dn3;
        *var_qjci_dn4_slot = var_qjci_dn4;
        *var_qjci_dn5_slot = var_qjci_dn5;
        *var_qjci_dn6_slot = var_qjci_dn6;
        *var_qlo_slot = var_qlo;
        *var_qlo_dn1_slot = var_qlo_dn1;
        *var_qlo_dn3_slot = var_qlo_dn3;
        *var_qlo_dn4_slot = var_qlo_dn4;
        *var_qlo_dn5_slot = var_qlo_dn5;
        *var_qlo_dn6_slot = var_qlo_dn6;
        *var_qxf1_slot = var_qxf1;
        *var_qxf1_dn3_slot = var_qxf1_dn3;
        *var_qxf1_dn4_slot = var_qxf1_dn4;
        *var_qxf1_dn5_slot = var_qxf1_dn5;
        *var_qxf1_dn6_slot = var_qxf1_dn6;
        *var_rb_nom_slot = var_rb_nom;
        *var_rc_nom_slot = var_rc_nom;
        *var_re_nom_slot = var_re_nom;
    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_arg_slot: &mut f64,
        var_arg0_slot: &mut f64,
        var_arg0_dn3_slot: &mut f64,
        var_arg0_rv_slot: &mut f64,
        var_arg_dn3_slot: &mut f64,
        var_arg_dn4_slot: &mut f64,
        var_arg_dn5_slot: &mut f64,
        var_arg_dn6_slot: &mut f64,
        var_arg_rv_slot: &mut f64,
        var_argt_slot: &mut f64,
        var_argt_dn3_slot: &mut f64,
        var_argt_rv_slot: &mut f64,
        var_argtr_slot: &mut f64,
        var_argtr_dn3_slot: &mut f64,
        var_argtr_rv_slot: &mut f64,
        var_bvr_t_slot: &mut f64,
        var_bvr_t_dn3_slot: &mut f64,
        var_bvr_t_rv_slot: &mut f64,
        var_cjc_i_slot: &mut f64,
        var_cjc_i_rv_slot: &mut f64,
        var_cjc_t_slot: &mut f64,
        var_cjc_t_dn3_slot: &mut f64,
        var_cjc_t_rv_slot: &mut f64,
        var_cje_i_slot: &mut f64,
        var_cje_i_rv_slot: &mut f64,
        var_cje_t_slot: &mut f64,
        var_cje_t_dn3_slot: &mut f64,
        var_cje_t_rv_slot: &mut f64,
        var_cjs_i_slot: &mut f64,
        var_cjs_i_rv_slot: &mut f64,
        var_cjs_t_slot: &mut f64,
        var_cjs_t_dn3_slot: &mut f64,
        var_cjs_t_rv_slot: &mut f64,
        var_cjt_slot: &mut f64,
        var_cjt_dn3_slot: &mut f64,
        var_cjt_rv_slot: &mut f64,
        var_egfet_slot: &mut f64,
        var_egfet_dn3_slot: &mut f64,
        var_egfet_rv_slot: &mut f64,
        var_fact1_slot: &mut f64,
        var_fact1_rv_slot: &mut f64,
        var_fact2_slot: &mut f64,
        var_fact2_dn3_slot: &mut f64,
        var_fact2_rv_slot: &mut f64,
        var_gmanew_slot: &mut f64,
        var_gmanew_dn3_slot: &mut f64,
        var_gmanew_rv_slot: &mut f64,
        var_gmaold_slot: &mut f64,
        var_gmaold_dn3_slot: &mut f64,
        var_gmaold_rv_slot: &mut f64,
        var_guard3_slot: &mut f64,
        var_guard3_rv_slot: &mut f64,
        var_ijbv_t_slot: &mut f64,
        var_ijbv_t_dn3_slot: &mut f64,
        var_ijbv_t_rv_slot: &mut f64,
        var_ijbvc_t_slot: &mut f64,
        var_ijbvc_t_dn3_slot: &mut f64,
        var_ijbvc_t_rv_slot: &mut f64,
        var_is_t_slot: &mut f64,
        var_is_t_dn3_slot: &mut f64,
        var_is_t_rv_slot: &mut f64,
        var_isc_t_slot: &mut f64,
        var_isc_t_dn3_slot: &mut f64,
        var_isc_t_rv_slot: &mut f64,
        var_ise_t_slot: &mut f64,
        var_ise_t_dn3_slot: &mut f64,
        var_ise_t_rv_slot: &mut f64,
        var_isr_t_slot: &mut f64,
        var_isr_t_dn3_slot: &mut f64,
        var_isr_t_rv_slot: &mut f64,
        var_lnrt_slot: &mut f64,
        var_lnrt_dn3_slot: &mut f64,
        var_lnrt_rv_slot: &mut f64,
        var_oikf_slot: &mut f64,
        var_oikf_dn4_slot: &mut f64,
        var_oikf_dn5_slot: &mut f64,
        var_oikf_rv_slot: &mut f64,
        var_oikr_slot: &mut f64,
        var_oikr_rv_slot: &mut f64,
        var_ovaf_slot: &mut f64,
        var_ovaf_rv_slot: &mut f64,
        var_ovar_slot: &mut f64,
        var_ovar_rv_slot: &mut f64,
        var_pbfact_slot: &mut f64,
        var_pbfact_dn3_slot: &mut f64,
        var_pbfact_rv_slot: &mut f64,
        var_pbo_slot: &mut f64,
        var_pbo_dn3_slot: &mut f64,
        var_pbo_rv_slot: &mut f64,
        var_rt_slot: &mut f64,
        var_rt_dn3_slot: &mut f64,
        var_rt_rv_slot: &mut f64,
        var_tamb_slot: &mut f64,
        var_tamb_dn3_slot: &mut f64,
        var_tamb_rv_slot: &mut f64,
        var_tbeta_slot: &mut f64,
        var_tbeta_dn3_slot: &mut f64,
        var_tbeta_rv_slot: &mut f64,
        var_tdev_slot: &mut f64,
        var_tdev_dn3_slot: &mut f64,
        var_tdev_rv_slot: &mut f64,
        var_theexp_t_slot: &mut f64,
        var_theexp_t_dn3_slot: &mut f64,
        var_theexp_t_rv_slot: &mut f64,
        var_tnom_slot: &mut f64,
        var_tnom_rv_slot: &mut f64,
        var_ttype_slot: &mut f64,
        var_ttype_rv_slot: &mut f64,
        var_vbci_slot: &mut f64,
        var_vbci_dn1_slot: &mut f64,
        var_vbci_dn4_slot: &mut f64,
        var_vbci_rv_slot: &mut f64,
        var_vbici_slot: &mut f64,
        var_vbici_dn4_slot: &mut f64,
        var_vbici_dn5_slot: &mut f64,
        var_vbici_rv_slot: &mut f64,
        var_vbiei_slot: &mut f64,
        var_vbiei_dn5_slot: &mut f64,
        var_vbiei_dn6_slot: &mut f64,
        var_vbiei_rv_slot: &mut f64,
        var_veci_slot: &mut f64,
        var_veci_dn2_slot: &mut f64,
        var_veci_dn4_slot: &mut f64,
        var_veci_rv_slot: &mut f64,
        var_vjc_t_slot: &mut f64,
        var_vjc_t_dn3_slot: &mut f64,
        var_vjc_t_rv_slot: &mut f64,
        var_vje_t_slot: &mut f64,
        var_vje_t_dn3_slot: &mut f64,
        var_vje_t_rv_slot: &mut f64,
        var_vjs_t_slot: &mut f64,
        var_vjs_t_dn3_slot: &mut f64,
        var_vjs_t_rv_slot: &mut f64,
        var_vt_slot: &mut f64,
        var_vt_dn3_slot: &mut f64,
        var_vt_rv_slot: &mut f64,
        var_weff_slot: &mut f64,
        var_weff_rv_slot: &mut f64,
    ) {
        let ctx_temp = ctx.temperature();
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let mut var_arg: f64 = *var_arg_slot;
        let mut var_arg0: f64 = *var_arg0_slot;
        let mut var_arg0_dn3: f64 = *var_arg0_dn3_slot;
        let mut var_arg0_rv: f64 = *var_arg0_rv_slot;
        let mut var_arg_dn3: f64 = *var_arg_dn3_slot;
        let mut var_arg_dn4: f64 = *var_arg_dn4_slot;
        let mut var_arg_dn5: f64 = *var_arg_dn5_slot;
        let mut var_arg_dn6: f64 = *var_arg_dn6_slot;
        let mut var_arg_rv: f64 = *var_arg_rv_slot;
        let mut var_argt: f64 = *var_argt_slot;
        let mut var_argt_dn3: f64 = *var_argt_dn3_slot;
        let mut var_argt_rv: f64 = *var_argt_rv_slot;
        let mut var_argtr: f64 = *var_argtr_slot;
        let mut var_argtr_dn3: f64 = *var_argtr_dn3_slot;
        let mut var_argtr_rv: f64 = *var_argtr_rv_slot;
        let mut var_bvr_t: f64 = *var_bvr_t_slot;
        let mut var_bvr_t_dn3: f64 = *var_bvr_t_dn3_slot;
        let mut var_bvr_t_rv: f64 = *var_bvr_t_rv_slot;
        let mut var_cjc_i: f64 = *var_cjc_i_slot;
        let mut var_cjc_i_rv: f64 = *var_cjc_i_rv_slot;
        let mut var_cjc_t: f64 = *var_cjc_t_slot;
        let mut var_cjc_t_dn3: f64 = *var_cjc_t_dn3_slot;
        let mut var_cjc_t_rv: f64 = *var_cjc_t_rv_slot;
        let mut var_cje_i: f64 = *var_cje_i_slot;
        let mut var_cje_i_rv: f64 = *var_cje_i_rv_slot;
        let mut var_cje_t: f64 = *var_cje_t_slot;
        let mut var_cje_t_dn3: f64 = *var_cje_t_dn3_slot;
        let mut var_cje_t_rv: f64 = *var_cje_t_rv_slot;
        let mut var_cjs_i: f64 = *var_cjs_i_slot;
        let mut var_cjs_i_rv: f64 = *var_cjs_i_rv_slot;
        let mut var_cjs_t: f64 = *var_cjs_t_slot;
        let mut var_cjs_t_dn3: f64 = *var_cjs_t_dn3_slot;
        let mut var_cjs_t_rv: f64 = *var_cjs_t_rv_slot;
        let mut var_cjt: f64 = *var_cjt_slot;
        let mut var_cjt_dn3: f64 = *var_cjt_dn3_slot;
        let mut var_cjt_rv: f64 = *var_cjt_rv_slot;
        let mut var_egfet: f64 = *var_egfet_slot;
        let mut var_egfet_dn3: f64 = *var_egfet_dn3_slot;
        let mut var_egfet_rv: f64 = *var_egfet_rv_slot;
        let mut var_fact1: f64 = *var_fact1_slot;
        let mut var_fact1_rv: f64 = *var_fact1_rv_slot;
        let mut var_fact2: f64 = *var_fact2_slot;
        let mut var_fact2_dn3: f64 = *var_fact2_dn3_slot;
        let mut var_fact2_rv: f64 = *var_fact2_rv_slot;
        let mut var_gmanew: f64 = *var_gmanew_slot;
        let mut var_gmanew_dn3: f64 = *var_gmanew_dn3_slot;
        let mut var_gmanew_rv: f64 = *var_gmanew_rv_slot;
        let mut var_gmaold: f64 = *var_gmaold_slot;
        let mut var_gmaold_dn3: f64 = *var_gmaold_dn3_slot;
        let mut var_gmaold_rv: f64 = *var_gmaold_rv_slot;
        let mut var_guard3: f64 = *var_guard3_slot;
        let mut var_guard3_rv: f64 = *var_guard3_rv_slot;
        let mut var_ijbv_t: f64 = *var_ijbv_t_slot;
        let mut var_ijbv_t_dn3: f64 = *var_ijbv_t_dn3_slot;
        let mut var_ijbv_t_rv: f64 = *var_ijbv_t_rv_slot;
        let mut var_ijbvc_t: f64 = *var_ijbvc_t_slot;
        let mut var_ijbvc_t_dn3: f64 = *var_ijbvc_t_dn3_slot;
        let mut var_ijbvc_t_rv: f64 = *var_ijbvc_t_rv_slot;
        let mut var_is_t: f64 = *var_is_t_slot;
        let mut var_is_t_dn3: f64 = *var_is_t_dn3_slot;
        let mut var_is_t_rv: f64 = *var_is_t_rv_slot;
        let mut var_isc_t: f64 = *var_isc_t_slot;
        let mut var_isc_t_dn3: f64 = *var_isc_t_dn3_slot;
        let mut var_isc_t_rv: f64 = *var_isc_t_rv_slot;
        let mut var_ise_t: f64 = *var_ise_t_slot;
        let mut var_ise_t_dn3: f64 = *var_ise_t_dn3_slot;
        let mut var_ise_t_rv: f64 = *var_ise_t_rv_slot;
        let mut var_isr_t: f64 = *var_isr_t_slot;
        let mut var_isr_t_dn3: f64 = *var_isr_t_dn3_slot;
        let mut var_isr_t_rv: f64 = *var_isr_t_rv_slot;
        let mut var_lnrt: f64 = *var_lnrt_slot;
        let mut var_lnrt_dn3: f64 = *var_lnrt_dn3_slot;
        let mut var_lnrt_rv: f64 = *var_lnrt_rv_slot;
        let mut var_oikf: f64 = *var_oikf_slot;
        let mut var_oikf_dn4: f64 = *var_oikf_dn4_slot;
        let mut var_oikf_dn5: f64 = *var_oikf_dn5_slot;
        let mut var_oikf_rv: f64 = *var_oikf_rv_slot;
        let mut var_oikr: f64 = *var_oikr_slot;
        let mut var_oikr_rv: f64 = *var_oikr_rv_slot;
        let mut var_ovaf: f64 = *var_ovaf_slot;
        let mut var_ovaf_rv: f64 = *var_ovaf_rv_slot;
        let mut var_ovar: f64 = *var_ovar_slot;
        let mut var_ovar_rv: f64 = *var_ovar_rv_slot;
        let mut var_pbfact: f64 = *var_pbfact_slot;
        let mut var_pbfact_dn3: f64 = *var_pbfact_dn3_slot;
        let mut var_pbfact_rv: f64 = *var_pbfact_rv_slot;
        let mut var_pbo: f64 = *var_pbo_slot;
        let mut var_pbo_dn3: f64 = *var_pbo_dn3_slot;
        let mut var_pbo_rv: f64 = *var_pbo_rv_slot;
        let mut var_rt: f64 = *var_rt_slot;
        let mut var_rt_dn3: f64 = *var_rt_dn3_slot;
        let mut var_rt_rv: f64 = *var_rt_rv_slot;
        let mut var_tamb: f64 = *var_tamb_slot;
        let mut var_tamb_dn3: f64 = *var_tamb_dn3_slot;
        let mut var_tamb_rv: f64 = *var_tamb_rv_slot;
        let mut var_tbeta: f64 = *var_tbeta_slot;
        let mut var_tbeta_dn3: f64 = *var_tbeta_dn3_slot;
        let mut var_tbeta_rv: f64 = *var_tbeta_rv_slot;
        let mut var_tdev: f64 = *var_tdev_slot;
        let mut var_tdev_dn3: f64 = *var_tdev_dn3_slot;
        let mut var_tdev_rv: f64 = *var_tdev_rv_slot;
        let mut var_theexp_t: f64 = *var_theexp_t_slot;
        let mut var_theexp_t_dn3: f64 = *var_theexp_t_dn3_slot;
        let mut var_theexp_t_rv: f64 = *var_theexp_t_rv_slot;
        let mut var_tnom: f64 = *var_tnom_slot;
        let mut var_tnom_rv: f64 = *var_tnom_rv_slot;
        let mut var_ttype: f64 = *var_ttype_slot;
        let mut var_ttype_rv: f64 = *var_ttype_rv_slot;
        let mut var_vbci: f64 = *var_vbci_slot;
        let mut var_vbci_dn1: f64 = *var_vbci_dn1_slot;
        let mut var_vbci_dn4: f64 = *var_vbci_dn4_slot;
        let mut var_vbci_rv: f64 = *var_vbci_rv_slot;
        let mut var_vbici: f64 = *var_vbici_slot;
        let mut var_vbici_dn4: f64 = *var_vbici_dn4_slot;
        let mut var_vbici_dn5: f64 = *var_vbici_dn5_slot;
        let mut var_vbici_rv: f64 = *var_vbici_rv_slot;
        let mut var_vbiei: f64 = *var_vbiei_slot;
        let mut var_vbiei_dn5: f64 = *var_vbiei_dn5_slot;
        let mut var_vbiei_dn6: f64 = *var_vbiei_dn6_slot;
        let mut var_vbiei_rv: f64 = *var_vbiei_rv_slot;
        let mut var_veci: f64 = *var_veci_slot;
        let mut var_veci_dn2: f64 = *var_veci_dn2_slot;
        let mut var_veci_dn4: f64 = *var_veci_dn4_slot;
        let mut var_veci_rv: f64 = *var_veci_rv_slot;
        let mut var_vjc_t: f64 = *var_vjc_t_slot;
        let mut var_vjc_t_dn3: f64 = *var_vjc_t_dn3_slot;
        let mut var_vjc_t_rv: f64 = *var_vjc_t_rv_slot;
        let mut var_vje_t: f64 = *var_vje_t_slot;
        let mut var_vje_t_dn3: f64 = *var_vje_t_dn3_slot;
        let mut var_vje_t_rv: f64 = *var_vje_t_rv_slot;
        let mut var_vjs_t: f64 = *var_vjs_t_slot;
        let mut var_vjs_t_dn3: f64 = *var_vjs_t_dn3_slot;
        let mut var_vjs_t_rv: f64 = *var_vjs_t_rv_slot;
        let mut var_vt: f64 = *var_vt_slot;
        let mut var_vt_dn3: f64 = *var_vt_dn3_slot;
        let mut var_vt_rv: f64 = *var_vt_rv_slot;
        let mut var_weff: f64 = *var_weff_slot;
        let mut var_weff_rv: f64 = *var_weff_rv_slot;

        let assign00_e447: f64 = ctx_temp;
        let assign00_e449: f64 = (assign00_e447 + (nv3 - 0.0));
        let assign00_e451: f64 = (assign00_e449 + p.p45);
        var_tamb = assign00_e451;
        var_tamb_dn3 = 1.0;
        var_tamb_rv = 0.0;

        let assign10_e454: f64 = (1026.85 + 273.15);
        let assign10_e457: f64 = (-100.0);
        let assign10_e459: f64 = (assign10_e457 + 273.15);
        let (assign10_e466, assign10_e466_d_n3,) = {
    if (var_tamb > assign10_e459) {
        (var_tamb, var_tamb_dn3,)
    } else {
        let assign10_e463: f64 = (-100.0);
        let assign10_e465: f64 = (assign10_e463 + 273.15);
        (assign10_e465, 0.0,)
    }
};
        let (assign10_e483, assign10_e483_d_n3,) = {
    if (assign10_e454 < assign10_e466) {
        let assign10_e470: f64 = (1026.85 + 273.15);
        (assign10_e470, 0.0,)
    } else {
        let assign10_e473: f64 = (-100.0);
        let assign10_e475: f64 = (assign10_e473 + 273.15);
        let (assign10_e482, assign10_e482_d_n3,) = {
            if (var_tamb > assign10_e475) {
                (var_tamb, var_tamb_dn3,)
            } else {
                let assign10_e479: f64 = (-100.0);
                let assign10_e481: f64 = (assign10_e479 + 273.15);
                (assign10_e481, 0.0,)
            }
        };
        (assign10_e482, assign10_e482_d_n3,)
    }
};
        var_tdev = assign10_e483;
        var_tdev_dn3 = assign10_e483_d_n3;
        var_tdev_rv = 0.0;

        let assign40_e493: f64 = (p.p43 * p.p42);
        var_weff = assign40_e493;
        var_weff_rv = 0.0;

        let assign70_e509: f64 = (p.p25 + 273.15);
        var_tnom = assign70_e509;
        var_tnom_rv = 0.0;

        let assign80_e512: f64 = (8.6170869e-5 * var_tdev);
        var_vt = assign80_e512;
        var_vt_dn3 = (8.6170869e-5 * var_tdev_dn3);
        var_vt_rv = 0.0;

        let assign90_e515: f64 = (var_tdev / var_tnom);
        var_rt = assign90_e515;
        var_rt_dn3 = (var_tdev_dn3 / var_tnom);
        var_rt_rv = 0.0;

        let assign100_e517: f64 = (var_rt).ln();
        var_lnrt = assign100_e517;
        var_lnrt_dn3 = (var_rt_dn3 / var_rt);
        var_lnrt_rv = 0.0;

        let assign110_e520: f64 = (p.p77 * var_lnrt);
        let assign110_e521: f64 = (assign110_e520).exp();
        var_tbeta = assign110_e521;
        var_tbeta_dn3 = (assign110_e521 * (p.p77 * var_lnrt_dn3));
        var_tbeta_rv = 0.0;

        let (assign140_e537,) = {
    if (p.p53 > 0.0) {
        let assign140_e535: f64 = (1.0 / p.p53);
        (assign140_e535,)
    } else {
        (0.0,)
    }
};
        var_ovaf = assign140_e537;
        var_ovaf_rv = 0.0;

        let (assign150_e545,) = {
    if (p.p62 > 0.0) {
        let assign150_e543: f64 = (1.0 / p.p62);
        (assign150_e543,)
    } else {
        (0.0,)
    }
};
        var_ovar = assign150_e545;
        var_ovar_rv = 0.0;

        let (assign160_e553,) = {
    if (p.p54 > 0.0) {
        let assign160_e551: f64 = (1.0 / p.p54);
        (assign160_e551,)
    } else {
        (0.0,)
    }
};
        var_oikf = assign160_e553;
        var_oikf_dn4 = 0.0;
        var_oikf_dn5 = 0.0;
        var_oikf_rv = 0.0;

        let (assign170_e561,) = {
    if (p.p63 > 0.0) {
        let assign170_e559: f64 = (1.0 / p.p63);
        (assign170_e559,)
    } else {
        (0.0,)
    }
};
        var_oikr = assign170_e561;
        var_oikr_rv = 0.0;

        let assign180_e564: f64 = (p.p22 * var_lnrt);
        let assign180_e568: f64 = (var_rt - 1.0);
        let assign180_e569: f64 = (p.p21 * assign180_e568);
        let assign180_e571: f64 = (assign180_e569 / var_vt);
        let assign180_e572: f64 = (assign180_e564 + assign180_e571);
        var_argt = assign180_e572;
        var_argt_dn3 = ((p.p22 * var_lnrt_dn3) + ((((p.p21 * var_rt_dn3) * var_vt) - (assign180_e569 * var_vt_dn3)) / (var_vt * var_vt)));
        var_argt_rv = 0.0;

        let assign190_e575: f64 = (p.p23 * var_lnrt);
        var_argtr = assign190_e575;
        var_argtr_dn3 = (p.p23 * var_lnrt_dn3);
        var_argtr_rv = 0.0;

        let assign200_e578: f64 = (var_argt).exp();
        let assign200_e579: f64 = (p.p0 * assign200_e578);
        var_is_t = assign200_e579;
        var_is_t_dn3 = (p.p0 * (assign200_e578 * var_argt_dn3));
        var_is_t_rv = 0.0;

        let assign210_e582: f64 = (var_argtr).exp();
        let assign210_e583: f64 = (p.p2 * assign210_e582);
        var_isr_t = assign210_e583;
        var_isr_t_dn3 = (p.p2 * (assign210_e582 * var_argtr_dn3));
        var_isr_t_rv = 0.0;

        let assign220_e587: f64 = (var_argt / p.p59);
        let assign220_e588: f64 = (assign220_e587).exp();
        let assign220_e589: f64 = (p.p58 * assign220_e588);
        let assign220_e591: f64 = (assign220_e589 / var_tbeta);
        var_ise_t = assign220_e591;
        var_ise_t_dn3 = ((((p.p58 * (assign220_e588 * (var_argt_dn3 / p.p59))) * var_tbeta) - (assign220_e589 * var_tbeta_dn3)) / (var_tbeta * var_tbeta));
        var_ise_t_rv = 0.0;

        let assign230_e595: f64 = (var_argt / p.p65);
        let assign230_e596: f64 = (assign230_e595).exp();
        let assign230_e597: f64 = (p.p64 * assign230_e596);
        let assign230_e599: f64 = (assign230_e597 / var_tbeta);
        var_isc_t = assign230_e599;
        var_isc_t_dn3 = ((((p.p64 * (assign230_e596 * (var_argt_dn3 / p.p65))) * var_tbeta) - (assign230_e597 * var_tbeta_dn3)) / (var_tbeta * var_tbeta));
        var_isc_t_rv = 0.0;

        let assign240_e605: f64 = (var_rt - 1.0);
        let assign240_e606: f64 = (p.p7 * assign240_e605);
        let assign240_e607: f64 = (1.0 + assign240_e606);
        let assign240_e608: f64 = (p.p47 * assign240_e607);
        var_ijbv_t = assign240_e608;
        var_ijbv_t_dn3 = (p.p47 * (p.p7 * var_rt_dn3));
        var_ijbv_t_rv = 0.0;

        let assign250_e614: f64 = (var_rt - 1.0);
        let assign250_e615: f64 = (p.p6 * assign250_e614);
        let assign250_e616: f64 = (1.0 + assign250_e615);
        let assign250_e617: f64 = (p.p5 * assign250_e616);
        var_bvr_t = assign250_e617;
        var_bvr_t_dn3 = (p.p5 * (p.p6 * var_rt_dn3));
        var_bvr_t_rv = 0.0;

        let assign260_e623: f64 = (var_rt - 1.0);
        let assign260_e624: f64 = (p.p10 * assign260_e623);
        let assign260_e625: f64 = (1.0 + assign260_e624);
        let assign260_e626: f64 = (p.p9 * assign260_e625);
        var_theexp_t = assign260_e626;
        var_theexp_t_dn3 = (p.p9 * (p.p10 * var_rt_dn3));
        var_theexp_t_rv = 0.0;

        let assign270_e632: f64 = (var_rt - 1.0);
        let assign270_e633: f64 = (p.p55 * assign270_e632);
        let assign270_e634: f64 = (1.0 + assign270_e633);
        let assign270_e635: f64 = (p.p56 * assign270_e634);
        var_ijbvc_t = assign270_e635;
        var_ijbvc_t_dn3 = (p.p56 * (p.p55 * var_rt_dn3));
        var_ijbvc_t_rv = 0.0;

        var_cje_i = p.p16;
        var_cje_i_rv = 0.0;

        var_cjc_i = p.p69;
        var_cjc_i_rv = 0.0;

        var_cjs_i = p.p74;
        var_cjs_i_rv = 0.0;

        let assign310_e641: f64 = (var_tnom / 300.15);
        var_fact1 = assign310_e641;
        var_fact1_rv = 0.0;

        let assign320_e644: f64 = (var_tdev / 300.15);
        var_fact2 = assign320_e644;
        var_fact2_dn3 = (var_tdev_dn3 / 300.15);
        var_fact2_rv = 0.0;

        let assign330_e648: f64 = (0.000702 * var_tdev);
        let assign330_e650: f64 = (assign330_e648 * var_tdev);
        let assign330_e653: f64 = (1108.0 + var_tdev);
        let assign330_e654: f64 = (assign330_e650 / assign330_e653);
        let assign330_e655: f64 = (1.16 - assign330_e654);
        var_egfet = assign330_e655;
        var_egfet_dn3 = (-((((((0.000702 * var_tdev_dn3) * var_tdev) + (assign330_e648 * var_tdev_dn3)) * assign330_e653) - (assign330_e650 * var_tdev_dn3)) / (assign330_e653 * assign330_e653)));
        var_egfet_rv = 0.0;

        let assign340_e657: f64 = (-var_egfet);
        let assign340_e661: f64 = (var_tdev + var_tdev);
        let assign340_e662: f64 = (1.3806226e-23 * assign340_e661);
        let assign340_e663: f64 = (assign340_e657 / assign340_e662);
        let assign340_e668: f64 = (300.15 + 300.15);
        let assign340_e669: f64 = (1.3806226e-23 * assign340_e668);
        let assign340_e670: f64 = (1.1150877 / assign340_e669);
        let assign340_e671: f64 = (assign340_e663 + assign340_e670);
        var_arg0 = assign340_e671;
        var_arg0_dn3 = ((((-var_egfet_dn3) * assign340_e662) - (assign340_e657 * (1.3806226e-23 * (var_tdev_dn3 + var_tdev_dn3)))) / (assign340_e662 * assign340_e662));
        var_arg0_rv = 0.0;

        let assign350_e674: f64 = (var_vt + var_vt);
        let assign350_e675: f64 = (-assign350_e674);
        let assign350_e678: f64 = (var_fact2).ln();
        let assign350_e679: f64 = (1.5 * assign350_e678);
        let assign350_e682: f64 = (1.6021918e-19 * var_arg0);
        let assign350_e683: f64 = (assign350_e679 + assign350_e682);
        let assign350_e684: f64 = (assign350_e675 * assign350_e683);
        var_pbfact = assign350_e684;
        var_pbfact_dn3 = (((-(var_vt_dn3 + var_vt_dn3)) * assign350_e683) + (assign350_e675 * ((1.5 * (var_fact2_dn3 / var_fact2)) + (1.6021918e-19 * var_arg0_dn3))));
        var_pbfact_rv = 0.0;

        let assign360_e687: f64 = (p.p17 - var_pbfact);
        let assign360_e689: f64 = (assign360_e687 / var_fact1);
        var_pbo = assign360_e689;
        var_pbo_dn3 = ((-var_pbfact_dn3) / var_fact1);
        var_pbo_rv = 0.0;

        let assign370_e692: f64 = (p.p17 - var_pbo);
        let assign370_e694: f64 = (assign370_e692 / var_pbo);
        var_gmaold = assign370_e694;
        var_gmaold_dn3 = ((((-var_pbo_dn3) * var_pbo) - (assign370_e692 * var_pbo_dn3)) / (var_pbo * var_pbo));
        var_gmaold_rv = 0.0;

        let assign380_e701: f64 = (var_tnom - 300.15);
        let assign380_e702: f64 = (0.0004 * assign380_e701);
        let assign380_e704: f64 = (assign380_e702 - var_gmaold);
        let assign380_e705: f64 = (p.p18 * assign380_e704);
        let assign380_e706: f64 = (1.0 + assign380_e705);
        let assign380_e707: f64 = (var_cje_i / assign380_e706);
        var_cjt = assign380_e707;
        var_cjt_dn3 = (-((var_cje_i * (p.p18 * (-var_gmaold_dn3))) / (assign380_e706 * assign380_e706)));
        var_cjt_rv = 0.0;

        let assign390_e710: f64 = (var_fact2 * var_pbo);
        let assign390_e712: f64 = (assign390_e710 + var_pbfact);
        var_vje_t = assign390_e712;
        var_vje_t_dn3 = (((var_fact2_dn3 * var_pbo) + (var_fact2 * var_pbo_dn3)) + var_pbfact_dn3);
        var_vje_t_rv = 0.0;

        let assign400_e715: f64 = (var_vje_t - var_pbo);
        let assign400_e717: f64 = (assign400_e715 / var_pbo);
        var_gmanew = assign400_e717;
        var_gmanew_dn3 = ((((var_vje_t_dn3 - var_pbo_dn3) * var_pbo) - (assign400_e715 * var_pbo_dn3)) / (var_pbo * var_pbo));
        var_gmanew_rv = 0.0;

        let assign410_e724: f64 = (var_tdev - 300.15);
        let assign410_e725: f64 = (0.0004 * assign410_e724);
        let assign410_e727: f64 = (assign410_e725 - var_gmanew);
        let assign410_e728: f64 = (p.p18 * assign410_e727);
        let assign410_e729: f64 = (1.0 + assign410_e728);
        let assign410_e730: f64 = (var_cjt * assign410_e729);
        var_cje_t = assign410_e730;
        var_cje_t_dn3 = ((var_cjt_dn3 * assign410_e729) + (var_cjt * (p.p18 * ((0.0004 * var_tdev_dn3) - var_gmanew_dn3))));
        var_cje_t_rv = 0.0;

        let assign420_e733: f64 = (var_tnom / 300.15);
        var_fact1 = assign420_e733;
        var_fact1_rv = 0.0;

        let assign430_e736: f64 = (var_tdev / 300.15);
        var_fact2 = assign430_e736;
        var_fact2_dn3 = (var_tdev_dn3 / 300.15);
        var_fact2_rv = 0.0;

        let assign440_e740: f64 = (0.000702 * var_tdev);
        let assign440_e742: f64 = (assign440_e740 * var_tdev);
        let assign440_e745: f64 = (1108.0 + var_tdev);
        let assign440_e746: f64 = (assign440_e742 / assign440_e745);
        let assign440_e747: f64 = (1.16 - assign440_e746);
        var_egfet = assign440_e747;
        var_egfet_dn3 = (-((((((0.000702 * var_tdev_dn3) * var_tdev) + (assign440_e740 * var_tdev_dn3)) * assign440_e745) - (assign440_e742 * var_tdev_dn3)) / (assign440_e745 * assign440_e745)));
        var_egfet_rv = 0.0;

        let assign450_e749: f64 = (-var_egfet);
        let assign450_e753: f64 = (var_tdev + var_tdev);
        let assign450_e754: f64 = (1.3806226e-23 * assign450_e753);
        let assign450_e755: f64 = (assign450_e749 / assign450_e754);
        let assign450_e760: f64 = (300.15 + 300.15);
        let assign450_e761: f64 = (1.3806226e-23 * assign450_e760);
        let assign450_e762: f64 = (1.1150877 / assign450_e761);
        let assign450_e763: f64 = (assign450_e755 + assign450_e762);
        var_arg0 = assign450_e763;
        var_arg0_dn3 = ((((-var_egfet_dn3) * assign450_e754) - (assign450_e749 * (1.3806226e-23 * (var_tdev_dn3 + var_tdev_dn3)))) / (assign450_e754 * assign450_e754));
        var_arg0_rv = 0.0;

        let assign460_e766: f64 = (var_vt + var_vt);
        let assign460_e767: f64 = (-assign460_e766);
        let assign460_e770: f64 = (var_fact2).ln();
        let assign460_e771: f64 = (1.5 * assign460_e770);
        let assign460_e774: f64 = (1.6021918e-19 * var_arg0);
        let assign460_e775: f64 = (assign460_e771 + assign460_e774);
        let assign460_e776: f64 = (assign460_e767 * assign460_e775);
        var_pbfact = assign460_e776;
        var_pbfact_dn3 = (((-(var_vt_dn3 + var_vt_dn3)) * assign460_e775) + (assign460_e767 * ((1.5 * (var_fact2_dn3 / var_fact2)) + (1.6021918e-19 * var_arg0_dn3))));
        var_pbfact_rv = 0.0;

        let assign470_e779: f64 = (p.p70 - var_pbfact);
        let assign470_e781: f64 = (assign470_e779 / var_fact1);
        var_pbo = assign470_e781;
        var_pbo_dn3 = ((-var_pbfact_dn3) / var_fact1);
        var_pbo_rv = 0.0;

        let assign480_e784: f64 = (p.p70 - var_pbo);
        let assign480_e786: f64 = (assign480_e784 / var_pbo);
        var_gmaold = assign480_e786;
        var_gmaold_dn3 = ((((-var_pbo_dn3) * var_pbo) - (assign480_e784 * var_pbo_dn3)) / (var_pbo * var_pbo));
        var_gmaold_rv = 0.0;

        let assign490_e793: f64 = (var_tnom - 300.15);
        let assign490_e794: f64 = (0.0004 * assign490_e793);
        let assign490_e796: f64 = (assign490_e794 - var_gmaold);
        let assign490_e797: f64 = (p.p71 * assign490_e796);
        let assign490_e798: f64 = (1.0 + assign490_e797);
        let assign490_e799: f64 = (var_cjc_i / assign490_e798);
        var_cjt = assign490_e799;
        var_cjt_dn3 = (-((var_cjc_i * (p.p71 * (-var_gmaold_dn3))) / (assign490_e798 * assign490_e798)));
        var_cjt_rv = 0.0;

        let assign500_e802: f64 = (var_fact2 * var_pbo);
        let assign500_e804: f64 = (assign500_e802 + var_pbfact);
        var_vjc_t = assign500_e804;
        var_vjc_t_dn3 = (((var_fact2_dn3 * var_pbo) + (var_fact2 * var_pbo_dn3)) + var_pbfact_dn3);
        var_vjc_t_rv = 0.0;

        let assign510_e807: f64 = (var_vjc_t - var_pbo);
        let assign510_e809: f64 = (assign510_e807 / var_pbo);
        var_gmanew = assign510_e809;
        var_gmanew_dn3 = ((((var_vjc_t_dn3 - var_pbo_dn3) * var_pbo) - (assign510_e807 * var_pbo_dn3)) / (var_pbo * var_pbo));
        var_gmanew_rv = 0.0;

        let assign520_e816: f64 = (var_tdev - 300.15);
        let assign520_e817: f64 = (0.0004 * assign520_e816);
        let assign520_e819: f64 = (assign520_e817 - var_gmanew);
        let assign520_e820: f64 = (p.p71 * assign520_e819);
        let assign520_e821: f64 = (1.0 + assign520_e820);
        let assign520_e822: f64 = (var_cjt * assign520_e821);
        var_cjc_t = assign520_e822;
        var_cjc_t_dn3 = ((var_cjt_dn3 * assign520_e821) + (var_cjt * (p.p71 * ((0.0004 * var_tdev_dn3) - var_gmanew_dn3))));
        var_cjc_t_rv = 0.0;

        let assign530_e825: f64 = (var_tnom / 300.15);
        var_fact1 = assign530_e825;
        var_fact1_rv = 0.0;

        let assign540_e828: f64 = (var_tdev / 300.15);
        var_fact2 = assign540_e828;
        var_fact2_dn3 = (var_tdev_dn3 / 300.15);
        var_fact2_rv = 0.0;

        let assign550_e832: f64 = (0.000702 * var_tdev);
        let assign550_e834: f64 = (assign550_e832 * var_tdev);
        let assign550_e837: f64 = (1108.0 + var_tdev);
        let assign550_e838: f64 = (assign550_e834 / assign550_e837);
        let assign550_e839: f64 = (1.16 - assign550_e838);
        var_egfet = assign550_e839;
        var_egfet_dn3 = (-((((((0.000702 * var_tdev_dn3) * var_tdev) + (assign550_e832 * var_tdev_dn3)) * assign550_e837) - (assign550_e834 * var_tdev_dn3)) / (assign550_e837 * assign550_e837)));
        var_egfet_rv = 0.0;

        let assign560_e841: f64 = (-var_egfet);
        let assign560_e845: f64 = (var_tdev + var_tdev);
        let assign560_e846: f64 = (1.3806226e-23 * assign560_e845);
        let assign560_e847: f64 = (assign560_e841 / assign560_e846);
        let assign560_e852: f64 = (300.15 + 300.15);
        let assign560_e853: f64 = (1.3806226e-23 * assign560_e852);
        let assign560_e854: f64 = (1.1150877 / assign560_e853);
        let assign560_e855: f64 = (assign560_e847 + assign560_e854);
        var_arg0 = assign560_e855;
        var_arg0_dn3 = ((((-var_egfet_dn3) * assign560_e846) - (assign560_e841 * (1.3806226e-23 * (var_tdev_dn3 + var_tdev_dn3)))) / (assign560_e846 * assign560_e846));
        var_arg0_rv = 0.0;

        let assign570_e858: f64 = (var_vt + var_vt);
        let assign570_e859: f64 = (-assign570_e858);
        let assign570_e862: f64 = (var_fact2).ln();
        let assign570_e863: f64 = (1.5 * assign570_e862);
        let assign570_e866: f64 = (1.6021918e-19 * var_arg0);
        let assign570_e867: f64 = (assign570_e863 + assign570_e866);
        let assign570_e868: f64 = (assign570_e859 * assign570_e867);
        var_pbfact = assign570_e868;
        var_pbfact_dn3 = (((-(var_vt_dn3 + var_vt_dn3)) * assign570_e867) + (assign570_e859 * ((1.5 * (var_fact2_dn3 / var_fact2)) + (1.6021918e-19 * var_arg0_dn3))));
        var_pbfact_rv = 0.0;

        let assign580_e871: f64 = (p.p75 - var_pbfact);
        let assign580_e873: f64 = (assign580_e871 / var_fact1);
        var_pbo = assign580_e873;
        var_pbo_dn3 = ((-var_pbfact_dn3) / var_fact1);
        var_pbo_rv = 0.0;

        let assign590_e876: f64 = (p.p75 - var_pbo);
        let assign590_e878: f64 = (assign590_e876 / var_pbo);
        var_gmaold = assign590_e878;
        var_gmaold_dn3 = ((((-var_pbo_dn3) * var_pbo) - (assign590_e876 * var_pbo_dn3)) / (var_pbo * var_pbo));
        var_gmaold_rv = 0.0;

        let assign600_e885: f64 = (var_tnom - 300.15);
        let assign600_e886: f64 = (0.0004 * assign600_e885);
        let assign600_e888: f64 = (assign600_e886 - var_gmaold);
        let assign600_e889: f64 = (p.p76 * assign600_e888);
        let assign600_e890: f64 = (1.0 + assign600_e889);
        let assign600_e891: f64 = (var_cjs_i / assign600_e890);
        var_cjt = assign600_e891;
        var_cjt_dn3 = (-((var_cjs_i * (p.p76 * (-var_gmaold_dn3))) / (assign600_e890 * assign600_e890)));
        var_cjt_rv = 0.0;

        let assign610_e894: f64 = (var_fact2 * var_pbo);
        let assign610_e896: f64 = (assign610_e894 + var_pbfact);
        var_vjs_t = assign610_e896;
        var_vjs_t_dn3 = (((var_fact2_dn3 * var_pbo) + (var_fact2 * var_pbo_dn3)) + var_pbfact_dn3);
        var_vjs_t_rv = 0.0;

        let assign620_e899: f64 = (var_vjs_t - var_pbo);
        let assign620_e901: f64 = (assign620_e899 / var_pbo);
        var_gmanew = assign620_e901;
        var_gmanew_dn3 = ((((var_vjs_t_dn3 - var_pbo_dn3) * var_pbo) - (assign620_e899 * var_pbo_dn3)) / (var_pbo * var_pbo));
        var_gmanew_rv = 0.0;

        let assign630_e908: f64 = (var_tdev - 300.15);
        let assign630_e909: f64 = (0.0004 * assign630_e908);
        let assign630_e911: f64 = (assign630_e909 - var_gmanew);
        let assign630_e912: f64 = (p.p76 * assign630_e911);
        let assign630_e913: f64 = (1.0 + assign630_e912);
        let assign630_e914: f64 = (var_cjt * assign630_e913);
        var_cjs_t = assign630_e914;
        var_cjs_t_dn3 = ((var_cjt_dn3 * assign630_e913) + (var_cjt * (p.p76 * ((0.0004 * var_tdev_dn3) - var_gmanew_dn3))));
        var_cjs_t_rv = 0.0;

        var_ttype = p.p29;
        var_ttype_rv = 0.0;

        let assign650_e918: f64 = (var_ttype * (nv2 - nv4));
        var_veci = assign650_e918;
        var_veci_dn2 = var_ttype;
        var_veci_dn4 = (-var_ttype);
        var_veci_rv = 0.0;

        let assign660_e921: f64 = (var_ttype * (nv5 - nv6));
        var_vbiei = assign660_e921;
        var_vbiei_dn5 = var_ttype;
        var_vbiei_dn6 = (-var_ttype);
        var_vbiei_rv = 0.0;

        let assign670_e924: f64 = (var_ttype * (nv5 - nv4));
        var_vbici = assign670_e924;
        var_vbici_dn4 = (-var_ttype);
        var_vbici_dn5 = var_ttype;
        var_vbici_rv = 0.0;

        let assign680_e927: f64 = (var_ttype * (nv1 - nv4));
        var_vbci = assign680_e927;
        var_vbci_dn1 = var_ttype;
        var_vbci_dn4 = (-var_ttype);
        var_vbci_rv = 0.0;

        let assign710_e936: f64 = if var_is_t > 0.0 { 1.0 } else { 0.0 };
        var_guard3 = assign710_e936;
        var_guard3_rv = 0.0;

        let (assign720_e944, assign720_e944_d_n3, assign720_e944_d_n4, assign720_e944_d_n5, assign720_e944_d_n6,) = {
    if (var_guard3 != 0.0) {
        let assign720_e941: f64 = (p.p1 * var_vt);
        let assign720_e942: f64 = (var_vbiei / assign720_e941);
        (assign720_e942, (-((var_vbiei * (p.p1 * var_vt_dn3)) / (assign720_e941 * assign720_e941))), 0.0, (var_vbiei_dn5 / assign720_e941), (var_vbiei_dn6 / assign720_e941),)
    } else {
        (var_arg, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6,)
    }
};
        var_arg = assign720_e944;
        var_arg_dn3 = assign720_e944_d_n3;
        var_arg_dn4 = assign720_e944_d_n4;
        var_arg_dn5 = assign720_e944_d_n5;
        var_arg_dn6 = assign720_e944_d_n6;
        var_arg_rv = 0.0;

        *var_arg_slot = var_arg;
        *var_arg0_slot = var_arg0;
        *var_arg0_dn3_slot = var_arg0_dn3;
        *var_arg0_rv_slot = var_arg0_rv;
        *var_arg_dn3_slot = var_arg_dn3;
        *var_arg_dn4_slot = var_arg_dn4;
        *var_arg_dn5_slot = var_arg_dn5;
        *var_arg_dn6_slot = var_arg_dn6;
        *var_arg_rv_slot = var_arg_rv;
        *var_argt_slot = var_argt;
        *var_argt_dn3_slot = var_argt_dn3;
        *var_argt_rv_slot = var_argt_rv;
        *var_argtr_slot = var_argtr;
        *var_argtr_dn3_slot = var_argtr_dn3;
        *var_argtr_rv_slot = var_argtr_rv;
        *var_bvr_t_slot = var_bvr_t;
        *var_bvr_t_dn3_slot = var_bvr_t_dn3;
        *var_bvr_t_rv_slot = var_bvr_t_rv;
        *var_cjc_i_slot = var_cjc_i;
        *var_cjc_i_rv_slot = var_cjc_i_rv;
        *var_cjc_t_slot = var_cjc_t;
        *var_cjc_t_dn3_slot = var_cjc_t_dn3;
        *var_cjc_t_rv_slot = var_cjc_t_rv;
        *var_cje_i_slot = var_cje_i;
        *var_cje_i_rv_slot = var_cje_i_rv;
        *var_cje_t_slot = var_cje_t;
        *var_cje_t_dn3_slot = var_cje_t_dn3;
        *var_cje_t_rv_slot = var_cje_t_rv;
        *var_cjs_i_slot = var_cjs_i;
        *var_cjs_i_rv_slot = var_cjs_i_rv;
        *var_cjs_t_slot = var_cjs_t;
        *var_cjs_t_dn3_slot = var_cjs_t_dn3;
        *var_cjs_t_rv_slot = var_cjs_t_rv;
        *var_cjt_slot = var_cjt;
        *var_cjt_dn3_slot = var_cjt_dn3;
        *var_cjt_rv_slot = var_cjt_rv;
        *var_egfet_slot = var_egfet;
        *var_egfet_dn3_slot = var_egfet_dn3;
        *var_egfet_rv_slot = var_egfet_rv;
        *var_fact1_slot = var_fact1;
        *var_fact1_rv_slot = var_fact1_rv;
        *var_fact2_slot = var_fact2;
        *var_fact2_dn3_slot = var_fact2_dn3;
        *var_fact2_rv_slot = var_fact2_rv;
        *var_gmanew_slot = var_gmanew;
        *var_gmanew_dn3_slot = var_gmanew_dn3;
        *var_gmanew_rv_slot = var_gmanew_rv;
        *var_gmaold_slot = var_gmaold;
        *var_gmaold_dn3_slot = var_gmaold_dn3;
        *var_gmaold_rv_slot = var_gmaold_rv;
        *var_guard3_slot = var_guard3;
        *var_guard3_rv_slot = var_guard3_rv;
        *var_ijbv_t_slot = var_ijbv_t;
        *var_ijbv_t_dn3_slot = var_ijbv_t_dn3;
        *var_ijbv_t_rv_slot = var_ijbv_t_rv;
        *var_ijbvc_t_slot = var_ijbvc_t;
        *var_ijbvc_t_dn3_slot = var_ijbvc_t_dn3;
        *var_ijbvc_t_rv_slot = var_ijbvc_t_rv;
        *var_is_t_slot = var_is_t;
        *var_is_t_dn3_slot = var_is_t_dn3;
        *var_is_t_rv_slot = var_is_t_rv;
        *var_isc_t_slot = var_isc_t;
        *var_isc_t_dn3_slot = var_isc_t_dn3;
        *var_isc_t_rv_slot = var_isc_t_rv;
        *var_ise_t_slot = var_ise_t;
        *var_ise_t_dn3_slot = var_ise_t_dn3;
        *var_ise_t_rv_slot = var_ise_t_rv;
        *var_isr_t_slot = var_isr_t;
        *var_isr_t_dn3_slot = var_isr_t_dn3;
        *var_isr_t_rv_slot = var_isr_t_rv;
        *var_lnrt_slot = var_lnrt;
        *var_lnrt_dn3_slot = var_lnrt_dn3;
        *var_lnrt_rv_slot = var_lnrt_rv;
        *var_oikf_slot = var_oikf;
        *var_oikf_dn4_slot = var_oikf_dn4;
        *var_oikf_dn5_slot = var_oikf_dn5;
        *var_oikf_rv_slot = var_oikf_rv;
        *var_oikr_slot = var_oikr;
        *var_oikr_rv_slot = var_oikr_rv;
        *var_ovaf_slot = var_ovaf;
        *var_ovaf_rv_slot = var_ovaf_rv;
        *var_ovar_slot = var_ovar;
        *var_ovar_rv_slot = var_ovar_rv;
        *var_pbfact_slot = var_pbfact;
        *var_pbfact_dn3_slot = var_pbfact_dn3;
        *var_pbfact_rv_slot = var_pbfact_rv;
        *var_pbo_slot = var_pbo;
        *var_pbo_dn3_slot = var_pbo_dn3;
        *var_pbo_rv_slot = var_pbo_rv;
        *var_rt_slot = var_rt;
        *var_rt_dn3_slot = var_rt_dn3;
        *var_rt_rv_slot = var_rt_rv;
        *var_tamb_slot = var_tamb;
        *var_tamb_dn3_slot = var_tamb_dn3;
        *var_tamb_rv_slot = var_tamb_rv;
        *var_tbeta_slot = var_tbeta;
        *var_tbeta_dn3_slot = var_tbeta_dn3;
        *var_tbeta_rv_slot = var_tbeta_rv;
        *var_tdev_slot = var_tdev;
        *var_tdev_dn3_slot = var_tdev_dn3;
        *var_tdev_rv_slot = var_tdev_rv;
        *var_theexp_t_slot = var_theexp_t;
        *var_theexp_t_dn3_slot = var_theexp_t_dn3;
        *var_theexp_t_rv_slot = var_theexp_t_rv;
        *var_tnom_slot = var_tnom;
        *var_tnom_rv_slot = var_tnom_rv;
        *var_ttype_slot = var_ttype;
        *var_ttype_rv_slot = var_ttype_rv;
        *var_vbci_slot = var_vbci;
        *var_vbci_dn1_slot = var_vbci_dn1;
        *var_vbci_dn4_slot = var_vbci_dn4;
        *var_vbci_rv_slot = var_vbci_rv;
        *var_vbici_slot = var_vbici;
        *var_vbici_dn4_slot = var_vbici_dn4;
        *var_vbici_dn5_slot = var_vbici_dn5;
        *var_vbici_rv_slot = var_vbici_rv;
        *var_vbiei_slot = var_vbiei;
        *var_vbiei_dn5_slot = var_vbiei_dn5;
        *var_vbiei_dn6_slot = var_vbiei_dn6;
        *var_vbiei_rv_slot = var_vbiei_rv;
        *var_veci_slot = var_veci;
        *var_veci_dn2_slot = var_veci_dn2;
        *var_veci_dn4_slot = var_veci_dn4;
        *var_veci_rv_slot = var_veci_rv;
        *var_vjc_t_slot = var_vjc_t;
        *var_vjc_t_dn3_slot = var_vjc_t_dn3;
        *var_vjc_t_rv_slot = var_vjc_t_rv;
        *var_vje_t_slot = var_vje_t;
        *var_vje_t_dn3_slot = var_vje_t_dn3;
        *var_vje_t_rv_slot = var_vje_t_rv;
        *var_vjs_t_slot = var_vjs_t;
        *var_vjs_t_dn3_slot = var_vjs_t_dn3;
        *var_vjs_t_rv_slot = var_vjs_t_rv;
        *var_vt_slot = var_vt;
        *var_vt_dn3_slot = var_vt_dn3;
        *var_vt_rv_slot = var_vt_rv;
        *var_weff_slot = var_weff;
        *var_weff_rv_slot = var_weff_rv;
    }

    pub(super) fn stamp_reactive_block_1(
        p: &Parameters,
        var_bvr_t: f64,
        var_bvr_t_dn3: f64,
        var_guard3: f64,
        var_ijbv_t: f64,
        var_ijbv_t_dn3: f64,
        var_is_t: f64,
        var_is_t_dn3: f64,
        var_ise_t: f64,
        var_isr_t: f64,
        var_theexp_t: f64,
        var_theexp_t_dn3: f64,
        var_vbici: f64,
        var_vbici_dn4: f64,
        var_vbici_dn5: f64,
        var_vbiei: f64,
        var_vbiei_dn5: f64,
        var_vbiei_dn6: f64,
        var_vt: f64,
        var_vt_dn3: f64,
        var_arg_slot: &mut f64,
        var_arg_dn3_slot: &mut f64,
        var_arg_dn4_slot: &mut f64,
        var_arg_dn5_slot: &mut f64,
        var_arg_dn6_slot: &mut f64,
        var_arg_rv_slot: &mut f64,
        var_argbv_slot: &mut f64,
        var_argbv_dn3_slot: &mut f64,
        var_argbv_dn4_slot: &mut f64,
        var_argbv_dn5_slot: &mut f64,
        var_argbv_dn6_slot: &mut f64,
        var_argbv_rv_slot: &mut f64,
        var_argbvvt_slot: &mut f64,
        var_argbvvt_dn3_slot: &mut f64,
        var_argbvvt_rv_slot: &mut f64,
        var_guard4_slot: &mut f64,
        var_guard4_rv_slot: &mut f64,
        var_guard5_slot: &mut f64,
        var_guard5_rv_slot: &mut f64,
        var_guard6_slot: &mut f64,
        var_guard6_rv_slot: &mut f64,
        var_guard7_slot: &mut f64,
        var_guard7_rv_slot: &mut f64,
        var_guard8_slot: &mut f64,
        var_guard8_rv_slot: &mut f64,
        var_guard9_slot: &mut f64,
        var_guard9_rv_slot: &mut f64,
        var_ifwd_slot: &mut f64,
        var_ifwd_dn3_slot: &mut f64,
        var_ifwd_dn4_slot: &mut f64,
        var_ifwd_dn5_slot: &mut f64,
        var_ifwd_dn6_slot: &mut f64,
        var_ifwd_rv_slot: &mut f64,
        var_le_slot: &mut f64,
        var_le_dn3_slot: &mut f64,
        var_le_dn4_slot: &mut f64,
        var_le_dn5_slot: &mut f64,
        var_le_dn6_slot: &mut f64,
        var_le_rv_slot: &mut f64,
        var_lebv_slot: &mut f64,
        var_lebv_dn3_slot: &mut f64,
        var_lebv_dn4_slot: &mut f64,
        var_lebv_dn5_slot: &mut f64,
        var_lebv_dn6_slot: &mut f64,
        var_lebv_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
    ) {
        let mut var_arg: f64 = *var_arg_slot;
        let mut var_arg_dn3: f64 = *var_arg_dn3_slot;
        let mut var_arg_dn4: f64 = *var_arg_dn4_slot;
        let mut var_arg_dn5: f64 = *var_arg_dn5_slot;
        let mut var_arg_dn6: f64 = *var_arg_dn6_slot;
        let mut var_arg_rv: f64 = *var_arg_rv_slot;
        let mut var_argbv: f64 = *var_argbv_slot;
        let mut var_argbv_dn3: f64 = *var_argbv_dn3_slot;
        let mut var_argbv_dn4: f64 = *var_argbv_dn4_slot;
        let mut var_argbv_dn5: f64 = *var_argbv_dn5_slot;
        let mut var_argbv_dn6: f64 = *var_argbv_dn6_slot;
        let mut var_argbv_rv: f64 = *var_argbv_rv_slot;
        let mut var_argbvvt: f64 = *var_argbvvt_slot;
        let mut var_argbvvt_dn3: f64 = *var_argbvvt_dn3_slot;
        let mut var_argbvvt_rv: f64 = *var_argbvvt_rv_slot;
        let mut var_guard4: f64 = *var_guard4_slot;
        let mut var_guard4_rv: f64 = *var_guard4_rv_slot;
        let mut var_guard5: f64 = *var_guard5_slot;
        let mut var_guard5_rv: f64 = *var_guard5_rv_slot;
        let mut var_guard6: f64 = *var_guard6_slot;
        let mut var_guard6_rv: f64 = *var_guard6_rv_slot;
        let mut var_guard7: f64 = *var_guard7_slot;
        let mut var_guard7_rv: f64 = *var_guard7_rv_slot;
        let mut var_guard8: f64 = *var_guard8_slot;
        let mut var_guard8_rv: f64 = *var_guard8_rv_slot;
        let mut var_guard9: f64 = *var_guard9_slot;
        let mut var_guard9_rv: f64 = *var_guard9_rv_slot;
        let mut var_ifwd: f64 = *var_ifwd_slot;
        let mut var_ifwd_dn3: f64 = *var_ifwd_dn3_slot;
        let mut var_ifwd_dn4: f64 = *var_ifwd_dn4_slot;
        let mut var_ifwd_dn5: f64 = *var_ifwd_dn5_slot;
        let mut var_ifwd_dn6: f64 = *var_ifwd_dn6_slot;
        let mut var_ifwd_rv: f64 = *var_ifwd_rv_slot;
        let mut var_le: f64 = *var_le_slot;
        let mut var_le_dn3: f64 = *var_le_dn3_slot;
        let mut var_le_dn4: f64 = *var_le_dn4_slot;
        let mut var_le_dn5: f64 = *var_le_dn5_slot;
        let mut var_le_dn6: f64 = *var_le_dn6_slot;
        let mut var_le_rv: f64 = *var_le_rv_slot;
        let mut var_lebv: f64 = *var_lebv_slot;
        let mut var_lebv_dn3: f64 = *var_lebv_dn3_slot;
        let mut var_lebv_dn4: f64 = *var_lebv_dn4_slot;
        let mut var_lebv_dn5: f64 = *var_lebv_dn5_slot;
        let mut var_lebv_dn6: f64 = *var_lebv_dn6_slot;
        let mut var_lebv_rv: f64 = *var_lebv_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;

        let (assign730_e955, assign730_e955_d_n3, assign730_e955_d_n4, assign730_e955_d_n5, assign730_e955_d_n6,) = {
    if (var_guard3 != 0.0) {
        let assign730_e947: f64 = (-var_vbiei);
        let assign730_e949: f64 = (assign730_e947 - var_bvr_t);
        let assign730_e952: f64 = (p.p11 * var_vt);
        let assign730_e953: f64 = (assign730_e949 / assign730_e952);
        (assign730_e953, ((((-var_bvr_t_dn3) * assign730_e952) - (assign730_e949 * (p.p11 * var_vt_dn3))) / (assign730_e952 * assign730_e952)), 0.0, ((-var_vbiei_dn5) / assign730_e952), ((-var_vbiei_dn6) / assign730_e952),)
    } else {
        (var_argbv, var_argbv_dn3, var_argbv_dn4, var_argbv_dn5, var_argbv_dn6,)
    }
};
        var_argbv = assign730_e955;
        var_argbv_dn3 = assign730_e955_d_n3;
        var_argbv_dn4 = assign730_e955_d_n4;
        var_argbv_dn5 = assign730_e955_d_n5;
        var_argbv_dn6 = assign730_e955_d_n6;
        var_argbv_rv = 0.0;

        let (assign740_e964, assign740_e964_d_n3,) = {
    if (var_guard3 != 0.0) {
        let assign740_e958: f64 = (-var_bvr_t);
        let assign740_e961: f64 = (p.p11 * var_vt);
        let assign740_e962: f64 = (assign740_e958 / assign740_e961);
        (assign740_e962, ((((-var_bvr_t_dn3) * assign740_e961) - (assign740_e958 * (p.p11 * var_vt_dn3))) / (assign740_e961 * assign740_e961)),)
    } else {
        (var_argbvvt, var_argbvvt_dn3,)
    }
};
        var_argbvvt = assign740_e964;
        var_argbvvt_dn3 = assign740_e964_d_n3;
        var_argbvvt_rv = 0.0;

        let assign750_e967: f64 = if var_arg > 80.0 { 1.0 } else { 0.0 };
        var_guard4 = assign750_e967;
        var_guard4_rv = 0.0;

        let (assign760_e977, assign760_e977_d_n3, assign760_e977_d_n4, assign760_e977_d_n5, assign760_e977_d_n6,) = {
    if ((var_guard3 != 0.0) && (var_guard4 != 0.0)) {
        let assign760_e974: f64 = (var_arg - 80.0);
        let assign760_e975: f64 = (1.0 + assign760_e974);
        (assign760_e975, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6,)
    } else {
        (var_le, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6,)
    }
};
        var_le = assign760_e977;
        var_le_dn3 = assign760_e977_d_n3;
        var_le_dn4 = assign760_e977_d_n4;
        var_le_dn5 = assign760_e977_d_n5;
        var_le_dn6 = assign760_e977_d_n6;
        var_le_rv = 0.0;

        let (assign770_e983, assign770_e983_d_n3, assign770_e983_d_n4, assign770_e983_d_n5, assign770_e983_d_n6,) = {
    if ((var_guard3 != 0.0) && (var_guard4 != 0.0)) {
        (80.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6,)
    }
};
        var_arg = assign770_e983;
        var_arg_dn3 = assign770_e983_d_n3;
        var_arg_dn4 = assign770_e983_d_n4;
        var_arg_dn5 = assign770_e983_d_n5;
        var_arg_dn6 = assign770_e983_d_n6;
        var_arg_rv = 0.0;

        let (assign780_e990, assign780_e990_d_n3, assign780_e990_d_n4, assign780_e990_d_n5, assign780_e990_d_n6,) = {
    if ((var_guard3 != 0.0) && (var_guard4 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_le, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6,)
    }
};
        var_le = assign780_e990;
        var_le_dn3 = assign780_e990_d_n3;
        var_le_dn4 = assign780_e990_d_n4;
        var_le_dn5 = assign780_e990_d_n5;
        var_le_dn6 = assign780_e990_d_n6;
        var_le_rv = 0.0;

        let (assign790_e997, assign790_e997_d_n3, assign790_e997_d_n4, assign790_e997_d_n5, assign790_e997_d_n6,) = {
    if (var_guard3 != 0.0) {
        let assign790_e994: f64 = (var_arg).exp();
        let assign790_e995: f64 = (var_le * assign790_e994);
        (assign790_e995, ((var_le_dn3 * assign790_e994) + (var_le * (assign790_e994 * var_arg_dn3))), ((var_le_dn4 * assign790_e994) + (var_le * (assign790_e994 * var_arg_dn4))), ((var_le_dn5 * assign790_e994) + (var_le * (assign790_e994 * var_arg_dn5))), ((var_le_dn6 * assign790_e994) + (var_le * (assign790_e994 * var_arg_dn6))),)
    } else {
        (var_le, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6,)
    }
};
        var_le = assign790_e997;
        var_le_dn3 = assign790_e997_d_n3;
        var_le_dn4 = assign790_e997_d_n4;
        var_le_dn5 = assign790_e997_d_n5;
        var_le_dn6 = assign790_e997_d_n6;
        var_le_rv = 0.0;

        let (assign800_e1069, assign800_e1069_d_n3, assign800_e1069_d_n4, assign800_e1069_d_n5, assign800_e1069_d_n6,) = {
    if (var_guard3 != 0.0) {
        let assign800_e1005: f64 = (-37.0);
        let (assign800_e1032, assign800_e1032_d_n3, assign800_e1032_d_n4, assign800_e1032_d_n5, assign800_e1032_d_n6,) = {
            if ((!(var_argbv >= 37.0)) && (!(var_argbv <= assign800_e1005))) {
                let assign800_e1010: f64 = (var_argbv).exp();
                let assign800_e1012: f64 = (assign800_e1010 + 1.0);
                let assign800_e1013: f64 = (assign800_e1012).ln();
                (assign800_e1013, ((assign800_e1010 * var_argbv_dn3) / assign800_e1012), ((assign800_e1010 * var_argbv_dn4) / assign800_e1012), ((assign800_e1010 * var_argbv_dn5) / assign800_e1012), ((assign800_e1010 * var_argbv_dn6) / assign800_e1012),)
            } else {
                let assign800_e1020: f64 = (-37.0);
                let (assign800_e1031, assign800_e1031_d_n3, assign800_e1031_d_n4, assign800_e1031_d_n5, assign800_e1031_d_n6,) = {
                    if ((!(var_argbv >= 37.0)) && (var_argbv <= assign800_e1020)) {
                        let assign800_e1024: f64 = (var_argbv).exp();
                        (assign800_e1024, (assign800_e1024 * var_argbv_dn3), (assign800_e1024 * var_argbv_dn4), (assign800_e1024 * var_argbv_dn5), (assign800_e1024 * var_argbv_dn6),)
                    } else {
                        let (assign800_e1030, assign800_e1030_d_n3, assign800_e1030_d_n4, assign800_e1030_d_n5, assign800_e1030_d_n6,) = {
                            if (var_argbv >= 37.0) {
                                (var_argbv, var_argbv_dn3, var_argbv_dn4, var_argbv_dn5, var_argbv_dn6,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign800_e1030, assign800_e1030_d_n3, assign800_e1030_d_n4, assign800_e1030_d_n5, assign800_e1030_d_n6,)
                    }
                };
                (assign800_e1031, assign800_e1031_d_n3, assign800_e1031_d_n4, assign800_e1031_d_n5, assign800_e1031_d_n6,)
            }
        };
        let assign800_e1039: f64 = (-37.0);
        let (assign800_e1066, assign800_e1066_d_n3,) = {
            if ((!(var_argbvvt >= 37.0)) && (!(var_argbvvt <= assign800_e1039))) {
                let assign800_e1044: f64 = (var_argbvvt).exp();
                let assign800_e1046: f64 = (assign800_e1044 + 1.0);
                let assign800_e1047: f64 = (assign800_e1046).ln();
                (assign800_e1047, ((assign800_e1044 * var_argbvvt_dn3) / assign800_e1046),)
            } else {
                let assign800_e1054: f64 = (-37.0);
                let (assign800_e1065, assign800_e1065_d_n3,) = {
                    if ((!(var_argbvvt >= 37.0)) && (var_argbvvt <= assign800_e1054)) {
                        let assign800_e1058: f64 = (var_argbvvt).exp();
                        (assign800_e1058, (assign800_e1058 * var_argbvvt_dn3),)
                    } else {
                        let (assign800_e1064, assign800_e1064_d_n3,) = {
                            if (var_argbvvt >= 37.0) {
                                (var_argbvvt, var_argbvvt_dn3,)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign800_e1064, assign800_e1064_d_n3,)
                    }
                };
                (assign800_e1065, assign800_e1065_d_n3,)
            }
        };
        let assign800_e1067: f64 = (assign800_e1032 - assign800_e1066);
        (assign800_e1067, (assign800_e1032_d_n3 - assign800_e1066_d_n3), assign800_e1032_d_n4, assign800_e1032_d_n5, assign800_e1032_d_n6,)
    } else {
        (var_lebv, var_lebv_dn3, var_lebv_dn4, var_lebv_dn5, var_lebv_dn6,)
    }
};
        var_lebv = assign800_e1069;
        var_lebv_dn3 = assign800_e1069_d_n3;
        var_lebv_dn4 = assign800_e1069_d_n4;
        var_lebv_dn5 = assign800_e1069_d_n5;
        var_lebv_dn6 = assign800_e1069_d_n6;
        var_lebv_rv = 0.0;

        let (assign810_e1090, assign810_e1090_d_n3, assign810_e1090_d_n4, assign810_e1090_d_n5, assign810_e1090_d_n6,) = {
    if (var_guard3 != 0.0) {
        let assign810_e1074: f64 = (var_le - 1.0);
        let assign810_e1075: f64 = (var_is_t * assign810_e1074);
        let assign810_e1078: f64 = (var_ijbv_t * var_lebv);
        let assign810_e1082: f64 = (var_vbiei).abs();
        let assign810_e1084: f64 = (assign810_e1082).powf(var_theexp_t);
        let assign810_e1085: f64 = (p.p8 * assign810_e1084);
        let assign810_e1086: f64 = (1.0 + assign810_e1085);
        let assign810_e1087: f64 = (assign810_e1078 / assign810_e1086);
        let assign810_e1088: f64 = (assign810_e1075 - assign810_e1087);
        (assign810_e1088, (((var_is_t_dn3 * assign810_e1074) + (var_is_t * var_le_dn3)) - (((((var_ijbv_t_dn3 * var_lebv) + (var_ijbv_t * var_lebv_dn3)) * assign810_e1086) - (assign810_e1078 * (p.p8 * if var_theexp_t_dn3 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { 0.0 } else { (assign810_e1084 * (var_theexp_t_dn3 * (assign810_e1082).ln())) }))) / (assign810_e1086 * assign810_e1086))), ((var_is_t * var_le_dn4) - ((var_ijbv_t * var_lebv_dn4) / assign810_e1086)), ((var_is_t * var_le_dn5) - ((((var_ijbv_t * var_lebv_dn5) * assign810_e1086) - (assign810_e1078 * (p.p8 * if 0.0 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign810_e1082).powf(var_theexp_t - 1.0) * if var_vbiei >= 0.0 { var_vbiei_dn5 } else { (-var_vbiei_dn5) })) } } else { (assign810_e1084 * (var_theexp_t * (if var_vbiei >= 0.0 { var_vbiei_dn5 } else { (-var_vbiei_dn5) } / assign810_e1082))) }))) / (assign810_e1086 * assign810_e1086))), ((var_is_t * var_le_dn6) - ((((var_ijbv_t * var_lebv_dn6) * assign810_e1086) - (assign810_e1078 * (p.p8 * if 0.0 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign810_e1082).powf(var_theexp_t - 1.0) * if var_vbiei >= 0.0 { var_vbiei_dn6 } else { (-var_vbiei_dn6) })) } } else { (assign810_e1084 * (var_theexp_t * (if var_vbiei >= 0.0 { var_vbiei_dn6 } else { (-var_vbiei_dn6) } / assign810_e1082))) }))) / (assign810_e1086 * assign810_e1086))),)
    } else {
        (var_ifwd, var_ifwd_dn3, var_ifwd_dn4, var_ifwd_dn5, var_ifwd_dn6,)
    }
};
        var_ifwd = assign810_e1090;
        var_ifwd_dn3 = assign810_e1090_d_n3;
        var_ifwd_dn4 = assign810_e1090_d_n4;
        var_ifwd_dn5 = assign810_e1090_d_n5;
        var_ifwd_dn6 = assign810_e1090_d_n6;
        var_ifwd_rv = 0.0;

        let (assign820_e1095, assign820_e1095_d_n3, assign820_e1095_d_n4, assign820_e1095_d_n5, assign820_e1095_d_n6,) = {
    if (var_guard3 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ifwd, var_ifwd_dn3, var_ifwd_dn4, var_ifwd_dn5, var_ifwd_dn6,)
    }
};
        var_ifwd = assign820_e1095;
        var_ifwd_dn3 = assign820_e1095_d_n3;
        var_ifwd_dn4 = assign820_e1095_d_n4;
        var_ifwd_dn5 = assign820_e1095_d_n5;
        var_ifwd_dn6 = assign820_e1095_d_n6;
        var_ifwd_rv = 0.0;

        let assign830_e1098: f64 = if var_isr_t > 0.0 { 1.0 } else { 0.0 };
        var_guard5 = assign830_e1098;
        var_guard5_rv = 0.0;

        let (assign840_e1106, assign840_e1106_d_n5, assign840_e1106_d_n6,) = {
    if (var_guard5 != 0.0) {
        let assign840_e1102: f64 = (p.p4 - var_vbiei);
        let assign840_e1104: f64 = (assign840_e1102).max(0.001);
        (assign840_e1104, if assign840_e1102 >= 0.001 { (-var_vbiei_dn5) } else { 0.0 }, if assign840_e1102 >= 0.001 { (-var_vbiei_dn6) } else { 0.0 },)
    } else {
        (var_t0, var_t0_dn5, var_t0_dn6,)
    }
};
        var_t0 = assign840_e1106;
        var_t0_dn5 = assign840_e1106_d_n5;
        var_t0_dn6 = assign840_e1106_d_n6;
        var_t0_rv = 0.0;

        let (assign850_e1121, assign850_e1121_d_n3, assign850_e1121_d_n4, assign850_e1121_d_n5, assign850_e1121_d_n6,) = {
    if (var_guard5 != 0.0) {
        let assign850_e1109: f64 = (-1.0);
        let assign850_e1111: f64 = (assign850_e1109 * var_vbiei);
        let assign850_e1113: f64 = (assign850_e1111 * p.p4);
        let assign850_e1116: f64 = (p.p3 * var_vt);
        let assign850_e1118: f64 = (assign850_e1116 * var_t0);
        let assign850_e1119: f64 = (assign850_e1113 / assign850_e1118);
        (assign850_e1119, (-((assign850_e1113 * ((p.p3 * var_vt_dn3) * var_t0)) / (assign850_e1118 * assign850_e1118))), 0.0, (((((assign850_e1109 * var_vbiei_dn5) * p.p4) * assign850_e1118) - (assign850_e1113 * (assign850_e1116 * var_t0_dn5))) / (assign850_e1118 * assign850_e1118)), (((((assign850_e1109 * var_vbiei_dn6) * p.p4) * assign850_e1118) - (assign850_e1113 * (assign850_e1116 * var_t0_dn6))) / (assign850_e1118 * assign850_e1118)),)
    } else {
        (var_arg, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6,)
    }
};
        var_arg = assign850_e1121;
        var_arg_dn3 = assign850_e1121_d_n3;
        var_arg_dn4 = assign850_e1121_d_n4;
        var_arg_dn5 = assign850_e1121_d_n5;
        var_arg_dn6 = assign850_e1121_d_n6;
        var_arg_rv = 0.0;

        let assign860_e1124: f64 = if var_arg > 80.0 { 1.0 } else { 0.0 };
        var_guard6 = assign860_e1124;
        var_guard6_rv = 0.0;

        let (assign870_e1134, assign870_e1134_d_n3, assign870_e1134_d_n4, assign870_e1134_d_n5, assign870_e1134_d_n6,) = {
    if ((var_guard5 != 0.0) && (var_guard6 != 0.0)) {
        let assign870_e1131: f64 = (var_arg - 80.0);
        let assign870_e1132: f64 = (1.0 + assign870_e1131);
        (assign870_e1132, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6,)
    } else {
        (var_le, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6,)
    }
};
        var_le = assign870_e1134;
        var_le_dn3 = assign870_e1134_d_n3;
        var_le_dn4 = assign870_e1134_d_n4;
        var_le_dn5 = assign870_e1134_d_n5;
        var_le_dn6 = assign870_e1134_d_n6;
        var_le_rv = 0.0;

        let (assign880_e1140, assign880_e1140_d_n3, assign880_e1140_d_n4, assign880_e1140_d_n5, assign880_e1140_d_n6,) = {
    if ((var_guard5 != 0.0) && (var_guard6 != 0.0)) {
        (80.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6,)
    }
};
        var_arg = assign880_e1140;
        var_arg_dn3 = assign880_e1140_d_n3;
        var_arg_dn4 = assign880_e1140_d_n4;
        var_arg_dn5 = assign880_e1140_d_n5;
        var_arg_dn6 = assign880_e1140_d_n6;
        var_arg_rv = 0.0;

        let (assign890_e1147, assign890_e1147_d_n3, assign890_e1147_d_n4, assign890_e1147_d_n5, assign890_e1147_d_n6,) = {
    if ((var_guard5 != 0.0) && (var_guard6 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_le, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6,)
    }
};
        var_le = assign890_e1147;
        var_le_dn3 = assign890_e1147_d_n3;
        var_le_dn4 = assign890_e1147_d_n4;
        var_le_dn5 = assign890_e1147_d_n5;
        var_le_dn6 = assign890_e1147_d_n6;
        var_le_rv = 0.0;

        let (assign900_e1154, assign900_e1154_d_n3, assign900_e1154_d_n4, assign900_e1154_d_n5, assign900_e1154_d_n6,) = {
    if (var_guard5 != 0.0) {
        let assign900_e1151: f64 = (var_arg).exp();
        let assign900_e1152: f64 = (var_le * assign900_e1151);
        (assign900_e1152, ((var_le_dn3 * assign900_e1151) + (var_le * (assign900_e1151 * var_arg_dn3))), ((var_le_dn4 * assign900_e1151) + (var_le * (assign900_e1151 * var_arg_dn4))), ((var_le_dn5 * assign900_e1151) + (var_le * (assign900_e1151 * var_arg_dn5))), ((var_le_dn6 * assign900_e1151) + (var_le * (assign900_e1151 * var_arg_dn6))),)
    } else {
        (var_le, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6,)
    }
};
        var_le = assign900_e1154;
        var_le_dn3 = assign900_e1154_d_n3;
        var_le_dn4 = assign900_e1154_d_n4;
        var_le_dn5 = assign900_e1154_d_n5;
        var_le_dn6 = assign900_e1154_d_n6;
        var_le_rv = 0.0;

        let assign930_e1170: f64 = if var_ise_t > 0.0 { 1.0 } else { 0.0 };
        var_guard7 = assign930_e1170;
        var_guard7_rv = 0.0;

        let (assign940_e1178, assign940_e1178_d_n3, assign940_e1178_d_n4, assign940_e1178_d_n5, assign940_e1178_d_n6,) = {
    if (var_guard7 != 0.0) {
        let assign940_e1175: f64 = (p.p59 * var_vt);
        let assign940_e1176: f64 = (var_vbiei / assign940_e1175);
        (assign940_e1176, (-((var_vbiei * (p.p59 * var_vt_dn3)) / (assign940_e1175 * assign940_e1175))), 0.0, (var_vbiei_dn5 / assign940_e1175), (var_vbiei_dn6 / assign940_e1175),)
    } else {
        (var_arg, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6,)
    }
};
        var_arg = assign940_e1178;
        var_arg_dn3 = assign940_e1178_d_n3;
        var_arg_dn4 = assign940_e1178_d_n4;
        var_arg_dn5 = assign940_e1178_d_n5;
        var_arg_dn6 = assign940_e1178_d_n6;
        var_arg_rv = 0.0;

        let (assign950_e1189, assign950_e1189_d_n3, assign950_e1189_d_n4, assign950_e1189_d_n5, assign950_e1189_d_n6,) = {
    if (var_guard7 != 0.0) {
        let assign950_e1181: f64 = (-var_vbiei);
        let assign950_e1183: f64 = (assign950_e1181 - var_bvr_t);
        let assign950_e1186: f64 = (p.p57 * var_vt);
        let assign950_e1187: f64 = (assign950_e1183 / assign950_e1186);
        (assign950_e1187, ((((-var_bvr_t_dn3) * assign950_e1186) - (assign950_e1183 * (p.p57 * var_vt_dn3))) / (assign950_e1186 * assign950_e1186)), 0.0, ((-var_vbiei_dn5) / assign950_e1186), ((-var_vbiei_dn6) / assign950_e1186),)
    } else {
        (var_argbv, var_argbv_dn3, var_argbv_dn4, var_argbv_dn5, var_argbv_dn6,)
    }
};
        var_argbv = assign950_e1189;
        var_argbv_dn3 = assign950_e1189_d_n3;
        var_argbv_dn4 = assign950_e1189_d_n4;
        var_argbv_dn5 = assign950_e1189_d_n5;
        var_argbv_dn6 = assign950_e1189_d_n6;
        var_argbv_rv = 0.0;

        let (assign960_e1198, assign960_e1198_d_n3,) = {
    if (var_guard7 != 0.0) {
        let assign960_e1192: f64 = (-var_bvr_t);
        let assign960_e1195: f64 = (p.p57 * var_vt);
        let assign960_e1196: f64 = (assign960_e1192 / assign960_e1195);
        (assign960_e1196, ((((-var_bvr_t_dn3) * assign960_e1195) - (assign960_e1192 * (p.p57 * var_vt_dn3))) / (assign960_e1195 * assign960_e1195)),)
    } else {
        (var_argbvvt, var_argbvvt_dn3,)
    }
};
        var_argbvvt = assign960_e1198;
        var_argbvvt_dn3 = assign960_e1198_d_n3;
        var_argbvvt_rv = 0.0;

        let assign970_e1201: f64 = if var_arg > 80.0 { 1.0 } else { 0.0 };
        var_guard8 = assign970_e1201;
        var_guard8_rv = 0.0;

        let (assign980_e1211, assign980_e1211_d_n3, assign980_e1211_d_n4, assign980_e1211_d_n5, assign980_e1211_d_n6,) = {
    if ((var_guard7 != 0.0) && (var_guard8 != 0.0)) {
        let assign980_e1208: f64 = (var_arg - 80.0);
        let assign980_e1209: f64 = (1.0 + assign980_e1208);
        (assign980_e1209, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6,)
    } else {
        (var_le, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6,)
    }
};
        var_le = assign980_e1211;
        var_le_dn3 = assign980_e1211_d_n3;
        var_le_dn4 = assign980_e1211_d_n4;
        var_le_dn5 = assign980_e1211_d_n5;
        var_le_dn6 = assign980_e1211_d_n6;
        var_le_rv = 0.0;

        let (assign990_e1217, assign990_e1217_d_n3, assign990_e1217_d_n4, assign990_e1217_d_n5, assign990_e1217_d_n6,) = {
    if ((var_guard7 != 0.0) && (var_guard8 != 0.0)) {
        (80.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6,)
    }
};
        var_arg = assign990_e1217;
        var_arg_dn3 = assign990_e1217_d_n3;
        var_arg_dn4 = assign990_e1217_d_n4;
        var_arg_dn5 = assign990_e1217_d_n5;
        var_arg_dn6 = assign990_e1217_d_n6;
        var_arg_rv = 0.0;

        let (assign1000_e1224, assign1000_e1224_d_n3, assign1000_e1224_d_n4, assign1000_e1224_d_n5, assign1000_e1224_d_n6,) = {
    if ((var_guard7 != 0.0) && (var_guard8 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_le, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6,)
    }
};
        var_le = assign1000_e1224;
        var_le_dn3 = assign1000_e1224_d_n3;
        var_le_dn4 = assign1000_e1224_d_n4;
        var_le_dn5 = assign1000_e1224_d_n5;
        var_le_dn6 = assign1000_e1224_d_n6;
        var_le_rv = 0.0;

        let (assign1010_e1231, assign1010_e1231_d_n3, assign1010_e1231_d_n4, assign1010_e1231_d_n5, assign1010_e1231_d_n6,) = {
    if (var_guard7 != 0.0) {
        let assign1010_e1228: f64 = (var_arg).exp();
        let assign1010_e1229: f64 = (var_le * assign1010_e1228);
        (assign1010_e1229, ((var_le_dn3 * assign1010_e1228) + (var_le * (assign1010_e1228 * var_arg_dn3))), ((var_le_dn4 * assign1010_e1228) + (var_le * (assign1010_e1228 * var_arg_dn4))), ((var_le_dn5 * assign1010_e1228) + (var_le * (assign1010_e1228 * var_arg_dn5))), ((var_le_dn6 * assign1010_e1228) + (var_le * (assign1010_e1228 * var_arg_dn6))),)
    } else {
        (var_le, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6,)
    }
};
        var_le = assign1010_e1231;
        var_le_dn3 = assign1010_e1231_d_n3;
        var_le_dn4 = assign1010_e1231_d_n4;
        var_le_dn5 = assign1010_e1231_d_n5;
        var_le_dn6 = assign1010_e1231_d_n6;
        var_le_rv = 0.0;

        let (assign1020_e1303, assign1020_e1303_d_n3, assign1020_e1303_d_n4, assign1020_e1303_d_n5, assign1020_e1303_d_n6,) = {
    if (var_guard7 != 0.0) {
        let assign1020_e1239: f64 = (-37.0);
        let (assign1020_e1266, assign1020_e1266_d_n3, assign1020_e1266_d_n4, assign1020_e1266_d_n5, assign1020_e1266_d_n6,) = {
            if ((!(var_argbv >= 37.0)) && (!(var_argbv <= assign1020_e1239))) {
                let assign1020_e1244: f64 = (var_argbv).exp();
                let assign1020_e1246: f64 = (assign1020_e1244 + 1.0);
                let assign1020_e1247: f64 = (assign1020_e1246).ln();
                (assign1020_e1247, ((assign1020_e1244 * var_argbv_dn3) / assign1020_e1246), ((assign1020_e1244 * var_argbv_dn4) / assign1020_e1246), ((assign1020_e1244 * var_argbv_dn5) / assign1020_e1246), ((assign1020_e1244 * var_argbv_dn6) / assign1020_e1246),)
            } else {
                let assign1020_e1254: f64 = (-37.0);
                let (assign1020_e1265, assign1020_e1265_d_n3, assign1020_e1265_d_n4, assign1020_e1265_d_n5, assign1020_e1265_d_n6,) = {
                    if ((!(var_argbv >= 37.0)) && (var_argbv <= assign1020_e1254)) {
                        let assign1020_e1258: f64 = (var_argbv).exp();
                        (assign1020_e1258, (assign1020_e1258 * var_argbv_dn3), (assign1020_e1258 * var_argbv_dn4), (assign1020_e1258 * var_argbv_dn5), (assign1020_e1258 * var_argbv_dn6),)
                    } else {
                        let (assign1020_e1264, assign1020_e1264_d_n3, assign1020_e1264_d_n4, assign1020_e1264_d_n5, assign1020_e1264_d_n6,) = {
                            if (var_argbv >= 37.0) {
                                (var_argbv, var_argbv_dn3, var_argbv_dn4, var_argbv_dn5, var_argbv_dn6,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign1020_e1264, assign1020_e1264_d_n3, assign1020_e1264_d_n4, assign1020_e1264_d_n5, assign1020_e1264_d_n6,)
                    }
                };
                (assign1020_e1265, assign1020_e1265_d_n3, assign1020_e1265_d_n4, assign1020_e1265_d_n5, assign1020_e1265_d_n6,)
            }
        };
        let assign1020_e1273: f64 = (-37.0);
        let (assign1020_e1300, assign1020_e1300_d_n3,) = {
            if ((!(var_argbvvt >= 37.0)) && (!(var_argbvvt <= assign1020_e1273))) {
                let assign1020_e1278: f64 = (var_argbvvt).exp();
                let assign1020_e1280: f64 = (assign1020_e1278 + 1.0);
                let assign1020_e1281: f64 = (assign1020_e1280).ln();
                (assign1020_e1281, ((assign1020_e1278 * var_argbvvt_dn3) / assign1020_e1280),)
            } else {
                let assign1020_e1288: f64 = (-37.0);
                let (assign1020_e1299, assign1020_e1299_d_n3,) = {
                    if ((!(var_argbvvt >= 37.0)) && (var_argbvvt <= assign1020_e1288)) {
                        let assign1020_e1292: f64 = (var_argbvvt).exp();
                        (assign1020_e1292, (assign1020_e1292 * var_argbvvt_dn3),)
                    } else {
                        let (assign1020_e1298, assign1020_e1298_d_n3,) = {
                            if (var_argbvvt >= 37.0) {
                                (var_argbvvt, var_argbvvt_dn3,)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign1020_e1298, assign1020_e1298_d_n3,)
                    }
                };
                (assign1020_e1299, assign1020_e1299_d_n3,)
            }
        };
        let assign1020_e1301: f64 = (assign1020_e1266 - assign1020_e1300);
        (assign1020_e1301, (assign1020_e1266_d_n3 - assign1020_e1300_d_n3), assign1020_e1266_d_n4, assign1020_e1266_d_n5, assign1020_e1266_d_n6,)
    } else {
        (var_lebv, var_lebv_dn3, var_lebv_dn4, var_lebv_dn5, var_lebv_dn6,)
    }
};
        var_lebv = assign1020_e1303;
        var_lebv_dn3 = assign1020_e1303_d_n3;
        var_lebv_dn4 = assign1020_e1303_d_n4;
        var_lebv_dn5 = assign1020_e1303_d_n5;
        var_lebv_dn6 = assign1020_e1303_d_n6;
        var_lebv_rv = 0.0;

        let assign1050_e1332: f64 = if var_is_t > 0.0 { 1.0 } else { 0.0 };
        var_guard9 = assign1050_e1332;
        var_guard9_rv = 0.0;

        let (assign1060_e1340, assign1060_e1340_d_n3, assign1060_e1340_d_n4, assign1060_e1340_d_n5, assign1060_e1340_d_n6,) = {
    if (var_guard9 != 0.0) {
        let assign1060_e1337: f64 = (p.p61 * var_vt);
        let assign1060_e1338: f64 = (var_vbici / assign1060_e1337);
        (assign1060_e1338, (-((var_vbici * (p.p61 * var_vt_dn3)) / (assign1060_e1337 * assign1060_e1337))), (var_vbici_dn4 / assign1060_e1337), (var_vbici_dn5 / assign1060_e1337), 0.0,)
    } else {
        (var_arg, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6,)
    }
};
        var_arg = assign1060_e1340;
        var_arg_dn3 = assign1060_e1340_d_n3;
        var_arg_dn4 = assign1060_e1340_d_n4;
        var_arg_dn5 = assign1060_e1340_d_n5;
        var_arg_dn6 = assign1060_e1340_d_n6;
        var_arg_rv = 0.0;

        *var_arg_slot = var_arg;
        *var_arg_dn3_slot = var_arg_dn3;
        *var_arg_dn4_slot = var_arg_dn4;
        *var_arg_dn5_slot = var_arg_dn5;
        *var_arg_dn6_slot = var_arg_dn6;
        *var_arg_rv_slot = var_arg_rv;
        *var_argbv_slot = var_argbv;
        *var_argbv_dn3_slot = var_argbv_dn3;
        *var_argbv_dn4_slot = var_argbv_dn4;
        *var_argbv_dn5_slot = var_argbv_dn5;
        *var_argbv_dn6_slot = var_argbv_dn6;
        *var_argbv_rv_slot = var_argbv_rv;
        *var_argbvvt_slot = var_argbvvt;
        *var_argbvvt_dn3_slot = var_argbvvt_dn3;
        *var_argbvvt_rv_slot = var_argbvvt_rv;
        *var_guard4_slot = var_guard4;
        *var_guard4_rv_slot = var_guard4_rv;
        *var_guard5_slot = var_guard5;
        *var_guard5_rv_slot = var_guard5_rv;
        *var_guard6_slot = var_guard6;
        *var_guard6_rv_slot = var_guard6_rv;
        *var_guard7_slot = var_guard7;
        *var_guard7_rv_slot = var_guard7_rv;
        *var_guard8_slot = var_guard8;
        *var_guard8_rv_slot = var_guard8_rv;
        *var_guard9_slot = var_guard9;
        *var_guard9_rv_slot = var_guard9_rv;
        *var_ifwd_slot = var_ifwd;
        *var_ifwd_dn3_slot = var_ifwd_dn3;
        *var_ifwd_dn4_slot = var_ifwd_dn4;
        *var_ifwd_dn5_slot = var_ifwd_dn5;
        *var_ifwd_dn6_slot = var_ifwd_dn6;
        *var_ifwd_rv_slot = var_ifwd_rv;
        *var_le_slot = var_le;
        *var_le_dn3_slot = var_le_dn3;
        *var_le_dn4_slot = var_le_dn4;
        *var_le_dn5_slot = var_le_dn5;
        *var_le_dn6_slot = var_le_dn6;
        *var_le_rv_slot = var_le_rv;
        *var_lebv_slot = var_lebv;
        *var_lebv_dn3_slot = var_lebv_dn3;
        *var_lebv_dn4_slot = var_lebv_dn4;
        *var_lebv_dn5_slot = var_lebv_dn5;
        *var_lebv_dn6_slot = var_lebv_dn6;
        *var_lebv_rv_slot = var_lebv_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_rv_slot = var_t0_rv;
    }

    pub(super) fn stamp_reactive_block_2(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_bvr_t: f64,
        var_bvr_t_dn3: f64,
        var_guard9: f64,
        var_ifwd: f64,
        var_ifwd_dn3: f64,
        var_ifwd_dn4: f64,
        var_ifwd_dn5: f64,
        var_ifwd_dn6: f64,
        var_ijbvc_t: f64,
        var_ijbvc_t_dn3: f64,
        var_is_t: f64,
        var_is_t_dn3: f64,
        var_isc_t: f64,
        var_oikr: f64,
        var_ovaf: f64,
        var_ovar: f64,
        var_theexp_t: f64,
        var_theexp_t_dn3: f64,
        var_vbici: f64,
        var_vbici_dn4: f64,
        var_vbici_dn5: f64,
        var_vbiei: f64,
        var_vbiei_dn5: f64,
        var_vbiei_dn6: f64,
        var_veci: f64,
        var_vt: f64,
        var_vt_dn3: f64,
        var_arg_slot: &mut f64,
        var_arg_dn3_slot: &mut f64,
        var_arg_dn4_slot: &mut f64,
        var_arg_dn5_slot: &mut f64,
        var_arg_dn6_slot: &mut f64,
        var_arg_rv_slot: &mut f64,
        var_argbv_slot: &mut f64,
        var_argbv_dn3_slot: &mut f64,
        var_argbv_dn4_slot: &mut f64,
        var_argbv_dn5_slot: &mut f64,
        var_argbv_dn6_slot: &mut f64,
        var_argbv_rv_slot: &mut f64,
        var_argbvvt_slot: &mut f64,
        var_argbvvt_dn3_slot: &mut f64,
        var_argbvvt_rv_slot: &mut f64,
        var_dkqb_slot: &mut f64,
        var_dkqb_dn3_slot: &mut f64,
        var_dkqb_dn4_slot: &mut f64,
        var_dkqb_dn5_slot: &mut f64,
        var_dkqb_dn6_slot: &mut f64,
        var_dkqb_rv_slot: &mut f64,
        var_guard10_slot: &mut f64,
        var_guard10_rv_slot: &mut f64,
        var_guard11_slot: &mut f64,
        var_guard11_rv_slot: &mut f64,
        var_guard12_slot: &mut f64,
        var_guard12_rv_slot: &mut f64,
        var_guard13_slot: &mut f64,
        var_guard13_rv_slot: &mut f64,
        var_guard15_slot: &mut f64,
        var_guard15_rv_slot: &mut f64,
        var_ibwd_slot: &mut f64,
        var_ibwd_dn3_slot: &mut f64,
        var_ibwd_dn4_slot: &mut f64,
        var_ibwd_dn5_slot: &mut f64,
        var_ibwd_dn6_slot: &mut f64,
        var_ibwd_rv_slot: &mut f64,
        var_ikq1_slot: &mut f64,
        var_ikq1_dn4_slot: &mut f64,
        var_ikq1_dn5_slot: &mut f64,
        var_ikq1_dn6_slot: &mut f64,
        var_ikq1_rv_slot: &mut f64,
        var_ikqb_slot: &mut f64,
        var_ikqb_dn3_slot: &mut f64,
        var_ikqb_dn4_slot: &mut f64,
        var_ikqb_dn5_slot: &mut f64,
        var_ikqb_dn6_slot: &mut f64,
        var_ikqb_rv_slot: &mut f64,
        var_itr_slot: &mut f64,
        var_itr_dn3_slot: &mut f64,
        var_itr_dn4_slot: &mut f64,
        var_itr_dn5_slot: &mut f64,
        var_itr_dn6_slot: &mut f64,
        var_itr_rv_slot: &mut f64,
        var_itzf_slot: &mut f64,
        var_itzf_dn3_slot: &mut f64,
        var_itzf_dn4_slot: &mut f64,
        var_itzf_dn5_slot: &mut f64,
        var_itzf_dn6_slot: &mut f64,
        var_itzf_rv_slot: &mut f64,
        var_kq2_slot: &mut f64,
        var_kq2_dn3_slot: &mut f64,
        var_kq2_dn4_slot: &mut f64,
        var_kq2_dn5_slot: &mut f64,
        var_kq2_dn6_slot: &mut f64,
        var_kq2_rv_slot: &mut f64,
        var_le_slot: &mut f64,
        var_le_dn3_slot: &mut f64,
        var_le_dn4_slot: &mut f64,
        var_le_dn5_slot: &mut f64,
        var_le_dn6_slot: &mut f64,
        var_le_rv_slot: &mut f64,
        var_lebv_slot: &mut f64,
        var_lebv_dn3_slot: &mut f64,
        var_lebv_dn4_slot: &mut f64,
        var_lebv_dn5_slot: &mut f64,
        var_lebv_dn6_slot: &mut f64,
        var_lebv_rv_slot: &mut f64,
        var_oikf_slot: &mut f64,
        var_oikf_dn4_slot: &mut f64,
        var_oikf_dn5_slot: &mut f64,
        var_oikf_rv_slot: &mut f64,
        var_qdc_slot: &mut f64,
        var_qdc_dn3_slot: &mut f64,
        var_qdc_dn4_slot: &mut f64,
        var_qdc_dn5_slot: &mut f64,
        var_qdc_dn6_slot: &mut f64,
        var_qdc_rv_slot: &mut f64,
        var_qde_slot: &mut f64,
        var_qde_dn1_slot: &mut f64,
        var_qde_dn2_slot: &mut f64,
        var_qde_dn3_slot: &mut f64,
        var_qde_dn4_slot: &mut f64,
        var_qde_dn5_slot: &mut f64,
        var_qde_dn6_slot: &mut f64,
        var_qde_rv_slot: &mut f64,
        var_tff_slot: &mut f64,
        var_tff_dn1_slot: &mut f64,
        var_tff_dn2_slot: &mut f64,
        var_tff_rv_slot: &mut f64,
        var_vtff_slot: &mut f64,
        var_vtff1_slot: &mut f64,
        var_vtff1_dn1_slot: &mut f64,
        var_vtff1_dn2_slot: &mut f64,
        var_vtff1_rv_slot: &mut f64,
        var_vtff_dn1_slot: &mut f64,
        var_vtff_dn2_slot: &mut f64,
        var_vtff_rv_slot: &mut f64,
    ) {
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let mut var_arg: f64 = *var_arg_slot;
        let mut var_arg_dn3: f64 = *var_arg_dn3_slot;
        let mut var_arg_dn4: f64 = *var_arg_dn4_slot;
        let mut var_arg_dn5: f64 = *var_arg_dn5_slot;
        let mut var_arg_dn6: f64 = *var_arg_dn6_slot;
        let mut var_arg_rv: f64 = *var_arg_rv_slot;
        let mut var_argbv: f64 = *var_argbv_slot;
        let mut var_argbv_dn3: f64 = *var_argbv_dn3_slot;
        let mut var_argbv_dn4: f64 = *var_argbv_dn4_slot;
        let mut var_argbv_dn5: f64 = *var_argbv_dn5_slot;
        let mut var_argbv_dn6: f64 = *var_argbv_dn6_slot;
        let mut var_argbv_rv: f64 = *var_argbv_rv_slot;
        let mut var_argbvvt: f64 = *var_argbvvt_slot;
        let mut var_argbvvt_dn3: f64 = *var_argbvvt_dn3_slot;
        let mut var_argbvvt_rv: f64 = *var_argbvvt_rv_slot;
        let mut var_dkqb: f64 = *var_dkqb_slot;
        let mut var_dkqb_dn3: f64 = *var_dkqb_dn3_slot;
        let mut var_dkqb_dn4: f64 = *var_dkqb_dn4_slot;
        let mut var_dkqb_dn5: f64 = *var_dkqb_dn5_slot;
        let mut var_dkqb_dn6: f64 = *var_dkqb_dn6_slot;
        let mut var_dkqb_rv: f64 = *var_dkqb_rv_slot;
        let mut var_guard10: f64 = *var_guard10_slot;
        let mut var_guard10_rv: f64 = *var_guard10_rv_slot;
        let mut var_guard11: f64 = *var_guard11_slot;
        let mut var_guard11_rv: f64 = *var_guard11_rv_slot;
        let mut var_guard12: f64 = *var_guard12_slot;
        let mut var_guard12_rv: f64 = *var_guard12_rv_slot;
        let mut var_guard13: f64 = *var_guard13_slot;
        let mut var_guard13_rv: f64 = *var_guard13_rv_slot;
        let mut var_guard15: f64 = *var_guard15_slot;
        let mut var_guard15_rv: f64 = *var_guard15_rv_slot;
        let mut var_ibwd: f64 = *var_ibwd_slot;
        let mut var_ibwd_dn3: f64 = *var_ibwd_dn3_slot;
        let mut var_ibwd_dn4: f64 = *var_ibwd_dn4_slot;
        let mut var_ibwd_dn5: f64 = *var_ibwd_dn5_slot;
        let mut var_ibwd_dn6: f64 = *var_ibwd_dn6_slot;
        let mut var_ibwd_rv: f64 = *var_ibwd_rv_slot;
        let mut var_ikq1: f64 = *var_ikq1_slot;
        let mut var_ikq1_dn4: f64 = *var_ikq1_dn4_slot;
        let mut var_ikq1_dn5: f64 = *var_ikq1_dn5_slot;
        let mut var_ikq1_dn6: f64 = *var_ikq1_dn6_slot;
        let mut var_ikq1_rv: f64 = *var_ikq1_rv_slot;
        let mut var_ikqb: f64 = *var_ikqb_slot;
        let mut var_ikqb_dn3: f64 = *var_ikqb_dn3_slot;
        let mut var_ikqb_dn4: f64 = *var_ikqb_dn4_slot;
        let mut var_ikqb_dn5: f64 = *var_ikqb_dn5_slot;
        let mut var_ikqb_dn6: f64 = *var_ikqb_dn6_slot;
        let mut var_ikqb_rv: f64 = *var_ikqb_rv_slot;
        let mut var_itr: f64 = *var_itr_slot;
        let mut var_itr_dn3: f64 = *var_itr_dn3_slot;
        let mut var_itr_dn4: f64 = *var_itr_dn4_slot;
        let mut var_itr_dn5: f64 = *var_itr_dn5_slot;
        let mut var_itr_dn6: f64 = *var_itr_dn6_slot;
        let mut var_itr_rv: f64 = *var_itr_rv_slot;
        let mut var_itzf: f64 = *var_itzf_slot;
        let mut var_itzf_dn3: f64 = *var_itzf_dn3_slot;
        let mut var_itzf_dn4: f64 = *var_itzf_dn4_slot;
        let mut var_itzf_dn5: f64 = *var_itzf_dn5_slot;
        let mut var_itzf_dn6: f64 = *var_itzf_dn6_slot;
        let mut var_itzf_rv: f64 = *var_itzf_rv_slot;
        let mut var_kq2: f64 = *var_kq2_slot;
        let mut var_kq2_dn3: f64 = *var_kq2_dn3_slot;
        let mut var_kq2_dn4: f64 = *var_kq2_dn4_slot;
        let mut var_kq2_dn5: f64 = *var_kq2_dn5_slot;
        let mut var_kq2_dn6: f64 = *var_kq2_dn6_slot;
        let mut var_kq2_rv: f64 = *var_kq2_rv_slot;
        let mut var_le: f64 = *var_le_slot;
        let mut var_le_dn3: f64 = *var_le_dn3_slot;
        let mut var_le_dn4: f64 = *var_le_dn4_slot;
        let mut var_le_dn5: f64 = *var_le_dn5_slot;
        let mut var_le_dn6: f64 = *var_le_dn6_slot;
        let mut var_le_rv: f64 = *var_le_rv_slot;
        let mut var_lebv: f64 = *var_lebv_slot;
        let mut var_lebv_dn3: f64 = *var_lebv_dn3_slot;
        let mut var_lebv_dn4: f64 = *var_lebv_dn4_slot;
        let mut var_lebv_dn5: f64 = *var_lebv_dn5_slot;
        let mut var_lebv_dn6: f64 = *var_lebv_dn6_slot;
        let mut var_lebv_rv: f64 = *var_lebv_rv_slot;
        let mut var_oikf: f64 = *var_oikf_slot;
        let mut var_oikf_dn4: f64 = *var_oikf_dn4_slot;
        let mut var_oikf_dn5: f64 = *var_oikf_dn5_slot;
        let mut var_oikf_rv: f64 = *var_oikf_rv_slot;
        let mut var_qdc: f64 = *var_qdc_slot;
        let mut var_qdc_dn3: f64 = *var_qdc_dn3_slot;
        let mut var_qdc_dn4: f64 = *var_qdc_dn4_slot;
        let mut var_qdc_dn5: f64 = *var_qdc_dn5_slot;
        let mut var_qdc_dn6: f64 = *var_qdc_dn6_slot;
        let mut var_qdc_rv: f64 = *var_qdc_rv_slot;
        let mut var_qde: f64 = *var_qde_slot;
        let mut var_qde_dn1: f64 = *var_qde_dn1_slot;
        let mut var_qde_dn2: f64 = *var_qde_dn2_slot;
        let mut var_qde_dn3: f64 = *var_qde_dn3_slot;
        let mut var_qde_dn4: f64 = *var_qde_dn4_slot;
        let mut var_qde_dn5: f64 = *var_qde_dn5_slot;
        let mut var_qde_dn6: f64 = *var_qde_dn6_slot;
        let mut var_qde_rv: f64 = *var_qde_rv_slot;
        let mut var_tff: f64 = *var_tff_slot;
        let mut var_tff_dn1: f64 = *var_tff_dn1_slot;
        let mut var_tff_dn2: f64 = *var_tff_dn2_slot;
        let mut var_tff_rv: f64 = *var_tff_rv_slot;
        let mut var_vtff: f64 = *var_vtff_slot;
        let mut var_vtff1: f64 = *var_vtff1_slot;
        let mut var_vtff1_dn1: f64 = *var_vtff1_dn1_slot;
        let mut var_vtff1_dn2: f64 = *var_vtff1_dn2_slot;
        let mut var_vtff1_rv: f64 = *var_vtff1_rv_slot;
        let mut var_vtff_dn1: f64 = *var_vtff_dn1_slot;
        let mut var_vtff_dn2: f64 = *var_vtff_dn2_slot;
        let mut var_vtff_rv: f64 = *var_vtff_rv_slot;

        let (assign1070_e1351, assign1070_e1351_d_n3, assign1070_e1351_d_n4, assign1070_e1351_d_n5, assign1070_e1351_d_n6,) = {
    if (var_guard9 != 0.0) {
        let assign1070_e1343: f64 = (-var_vbici);
        let assign1070_e1345: f64 = (assign1070_e1343 - var_bvr_t);
        let assign1070_e1348: f64 = (p.p57 * var_vt);
        let assign1070_e1349: f64 = (assign1070_e1345 / assign1070_e1348);
        (assign1070_e1349, ((((-var_bvr_t_dn3) * assign1070_e1348) - (assign1070_e1345 * (p.p57 * var_vt_dn3))) / (assign1070_e1348 * assign1070_e1348)), ((-var_vbici_dn4) / assign1070_e1348), ((-var_vbici_dn5) / assign1070_e1348), 0.0,)
    } else {
        (var_argbv, var_argbv_dn3, var_argbv_dn4, var_argbv_dn5, var_argbv_dn6,)
    }
};
        var_argbv = assign1070_e1351;
        var_argbv_dn3 = assign1070_e1351_d_n3;
        var_argbv_dn4 = assign1070_e1351_d_n4;
        var_argbv_dn5 = assign1070_e1351_d_n5;
        var_argbv_dn6 = assign1070_e1351_d_n6;
        var_argbv_rv = 0.0;

        let (assign1080_e1360, assign1080_e1360_d_n3,) = {
    if (var_guard9 != 0.0) {
        let assign1080_e1354: f64 = (-var_bvr_t);
        let assign1080_e1357: f64 = (p.p57 * var_vt);
        let assign1080_e1358: f64 = (assign1080_e1354 / assign1080_e1357);
        (assign1080_e1358, ((((-var_bvr_t_dn3) * assign1080_e1357) - (assign1080_e1354 * (p.p57 * var_vt_dn3))) / (assign1080_e1357 * assign1080_e1357)),)
    } else {
        (var_argbvvt, var_argbvvt_dn3,)
    }
};
        var_argbvvt = assign1080_e1360;
        var_argbvvt_dn3 = assign1080_e1360_d_n3;
        var_argbvvt_rv = 0.0;

        let assign1090_e1363: f64 = if var_arg > 80.0 { 1.0 } else { 0.0 };
        var_guard10 = assign1090_e1363;
        var_guard10_rv = 0.0;

        let (assign1100_e1373, assign1100_e1373_d_n3, assign1100_e1373_d_n4, assign1100_e1373_d_n5, assign1100_e1373_d_n6,) = {
    if ((var_guard9 != 0.0) && (var_guard10 != 0.0)) {
        let assign1100_e1370: f64 = (var_arg - 80.0);
        let assign1100_e1371: f64 = (1.0 + assign1100_e1370);
        (assign1100_e1371, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6,)
    } else {
        (var_le, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6,)
    }
};
        var_le = assign1100_e1373;
        var_le_dn3 = assign1100_e1373_d_n3;
        var_le_dn4 = assign1100_e1373_d_n4;
        var_le_dn5 = assign1100_e1373_d_n5;
        var_le_dn6 = assign1100_e1373_d_n6;
        var_le_rv = 0.0;

        let (assign1110_e1379, assign1110_e1379_d_n3, assign1110_e1379_d_n4, assign1110_e1379_d_n5, assign1110_e1379_d_n6,) = {
    if ((var_guard9 != 0.0) && (var_guard10 != 0.0)) {
        (80.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6,)
    }
};
        var_arg = assign1110_e1379;
        var_arg_dn3 = assign1110_e1379_d_n3;
        var_arg_dn4 = assign1110_e1379_d_n4;
        var_arg_dn5 = assign1110_e1379_d_n5;
        var_arg_dn6 = assign1110_e1379_d_n6;
        var_arg_rv = 0.0;

        let (assign1120_e1386, assign1120_e1386_d_n3, assign1120_e1386_d_n4, assign1120_e1386_d_n5, assign1120_e1386_d_n6,) = {
    if ((var_guard9 != 0.0) && (var_guard10 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_le, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6,)
    }
};
        var_le = assign1120_e1386;
        var_le_dn3 = assign1120_e1386_d_n3;
        var_le_dn4 = assign1120_e1386_d_n4;
        var_le_dn5 = assign1120_e1386_d_n5;
        var_le_dn6 = assign1120_e1386_d_n6;
        var_le_rv = 0.0;

        let (assign1130_e1393, assign1130_e1393_d_n3, assign1130_e1393_d_n4, assign1130_e1393_d_n5, assign1130_e1393_d_n6,) = {
    if (var_guard9 != 0.0) {
        let assign1130_e1390: f64 = (var_arg).exp();
        let assign1130_e1391: f64 = (var_le * assign1130_e1390);
        (assign1130_e1391, ((var_le_dn3 * assign1130_e1390) + (var_le * (assign1130_e1390 * var_arg_dn3))), ((var_le_dn4 * assign1130_e1390) + (var_le * (assign1130_e1390 * var_arg_dn4))), ((var_le_dn5 * assign1130_e1390) + (var_le * (assign1130_e1390 * var_arg_dn5))), ((var_le_dn6 * assign1130_e1390) + (var_le * (assign1130_e1390 * var_arg_dn6))),)
    } else {
        (var_le, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6,)
    }
};
        var_le = assign1130_e1393;
        var_le_dn3 = assign1130_e1393_d_n3;
        var_le_dn4 = assign1130_e1393_d_n4;
        var_le_dn5 = assign1130_e1393_d_n5;
        var_le_dn6 = assign1130_e1393_d_n6;
        var_le_rv = 0.0;

        let (assign1140_e1465, assign1140_e1465_d_n3, assign1140_e1465_d_n4, assign1140_e1465_d_n5, assign1140_e1465_d_n6,) = {
    if (var_guard9 != 0.0) {
        let assign1140_e1401: f64 = (-37.0);
        let (assign1140_e1428, assign1140_e1428_d_n3, assign1140_e1428_d_n4, assign1140_e1428_d_n5, assign1140_e1428_d_n6,) = {
            if ((!(var_argbv >= 37.0)) && (!(var_argbv <= assign1140_e1401))) {
                let assign1140_e1406: f64 = (var_argbv).exp();
                let assign1140_e1408: f64 = (assign1140_e1406 + 1.0);
                let assign1140_e1409: f64 = (assign1140_e1408).ln();
                (assign1140_e1409, ((assign1140_e1406 * var_argbv_dn3) / assign1140_e1408), ((assign1140_e1406 * var_argbv_dn4) / assign1140_e1408), ((assign1140_e1406 * var_argbv_dn5) / assign1140_e1408), ((assign1140_e1406 * var_argbv_dn6) / assign1140_e1408),)
            } else {
                let assign1140_e1416: f64 = (-37.0);
                let (assign1140_e1427, assign1140_e1427_d_n3, assign1140_e1427_d_n4, assign1140_e1427_d_n5, assign1140_e1427_d_n6,) = {
                    if ((!(var_argbv >= 37.0)) && (var_argbv <= assign1140_e1416)) {
                        let assign1140_e1420: f64 = (var_argbv).exp();
                        (assign1140_e1420, (assign1140_e1420 * var_argbv_dn3), (assign1140_e1420 * var_argbv_dn4), (assign1140_e1420 * var_argbv_dn5), (assign1140_e1420 * var_argbv_dn6),)
                    } else {
                        let (assign1140_e1426, assign1140_e1426_d_n3, assign1140_e1426_d_n4, assign1140_e1426_d_n5, assign1140_e1426_d_n6,) = {
                            if (var_argbv >= 37.0) {
                                (var_argbv, var_argbv_dn3, var_argbv_dn4, var_argbv_dn5, var_argbv_dn6,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign1140_e1426, assign1140_e1426_d_n3, assign1140_e1426_d_n4, assign1140_e1426_d_n5, assign1140_e1426_d_n6,)
                    }
                };
                (assign1140_e1427, assign1140_e1427_d_n3, assign1140_e1427_d_n4, assign1140_e1427_d_n5, assign1140_e1427_d_n6,)
            }
        };
        let assign1140_e1435: f64 = (-37.0);
        let (assign1140_e1462, assign1140_e1462_d_n3,) = {
            if ((!(var_argbvvt >= 37.0)) && (!(var_argbvvt <= assign1140_e1435))) {
                let assign1140_e1440: f64 = (var_argbvvt).exp();
                let assign1140_e1442: f64 = (assign1140_e1440 + 1.0);
                let assign1140_e1443: f64 = (assign1140_e1442).ln();
                (assign1140_e1443, ((assign1140_e1440 * var_argbvvt_dn3) / assign1140_e1442),)
            } else {
                let assign1140_e1450: f64 = (-37.0);
                let (assign1140_e1461, assign1140_e1461_d_n3,) = {
                    if ((!(var_argbvvt >= 37.0)) && (var_argbvvt <= assign1140_e1450)) {
                        let assign1140_e1454: f64 = (var_argbvvt).exp();
                        (assign1140_e1454, (assign1140_e1454 * var_argbvvt_dn3),)
                    } else {
                        let (assign1140_e1460, assign1140_e1460_d_n3,) = {
                            if (var_argbvvt >= 37.0) {
                                (var_argbvvt, var_argbvvt_dn3,)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign1140_e1460, assign1140_e1460_d_n3,)
                    }
                };
                (assign1140_e1461, assign1140_e1461_d_n3,)
            }
        };
        let assign1140_e1463: f64 = (assign1140_e1428 - assign1140_e1462);
        (assign1140_e1463, (assign1140_e1428_d_n3 - assign1140_e1462_d_n3), assign1140_e1428_d_n4, assign1140_e1428_d_n5, assign1140_e1428_d_n6,)
    } else {
        (var_lebv, var_lebv_dn3, var_lebv_dn4, var_lebv_dn5, var_lebv_dn6,)
    }
};
        var_lebv = assign1140_e1465;
        var_lebv_dn3 = assign1140_e1465_d_n3;
        var_lebv_dn4 = assign1140_e1465_d_n4;
        var_lebv_dn5 = assign1140_e1465_d_n5;
        var_lebv_dn6 = assign1140_e1465_d_n6;
        var_lebv_rv = 0.0;

        let (assign1150_e1486, assign1150_e1486_d_n3, assign1150_e1486_d_n4, assign1150_e1486_d_n5, assign1150_e1486_d_n6,) = {
    if (var_guard9 != 0.0) {
        let assign1150_e1470: f64 = (var_le - 1.0);
        let assign1150_e1471: f64 = (var_is_t * assign1150_e1470);
        let assign1150_e1474: f64 = (var_ijbvc_t * var_lebv);
        let assign1150_e1478: f64 = (var_vbici).abs();
        let assign1150_e1480: f64 = (assign1150_e1478).powf(var_theexp_t);
        let assign1150_e1481: f64 = (p.p8 * assign1150_e1480);
        let assign1150_e1482: f64 = (1.0 + assign1150_e1481);
        let assign1150_e1483: f64 = (assign1150_e1474 / assign1150_e1482);
        let assign1150_e1484: f64 = (assign1150_e1471 - assign1150_e1483);
        (assign1150_e1484, (((var_is_t_dn3 * assign1150_e1470) + (var_is_t * var_le_dn3)) - (((((var_ijbvc_t_dn3 * var_lebv) + (var_ijbvc_t * var_lebv_dn3)) * assign1150_e1482) - (assign1150_e1474 * (p.p8 * if var_theexp_t_dn3 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { 0.0 } else { (assign1150_e1480 * (var_theexp_t_dn3 * (assign1150_e1478).ln())) }))) / (assign1150_e1482 * assign1150_e1482))), ((var_is_t * var_le_dn4) - ((((var_ijbvc_t * var_lebv_dn4) * assign1150_e1482) - (assign1150_e1474 * (p.p8 * if 0.0 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign1150_e1478).powf(var_theexp_t - 1.0) * if var_vbici >= 0.0 { var_vbici_dn4 } else { (-var_vbici_dn4) })) } } else { (assign1150_e1480 * (var_theexp_t * (if var_vbici >= 0.0 { var_vbici_dn4 } else { (-var_vbici_dn4) } / assign1150_e1478))) }))) / (assign1150_e1482 * assign1150_e1482))), ((var_is_t * var_le_dn5) - ((((var_ijbvc_t * var_lebv_dn5) * assign1150_e1482) - (assign1150_e1474 * (p.p8 * if 0.0 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign1150_e1478).powf(var_theexp_t - 1.0) * if var_vbici >= 0.0 { var_vbici_dn5 } else { (-var_vbici_dn5) })) } } else { (assign1150_e1480 * (var_theexp_t * (if var_vbici >= 0.0 { var_vbici_dn5 } else { (-var_vbici_dn5) } / assign1150_e1478))) }))) / (assign1150_e1482 * assign1150_e1482))), ((var_is_t * var_le_dn6) - ((var_ijbvc_t * var_lebv_dn6) / assign1150_e1482)),)
    } else {
        (var_ibwd, var_ibwd_dn3, var_ibwd_dn4, var_ibwd_dn5, var_ibwd_dn6,)
    }
};
        var_ibwd = assign1150_e1486;
        var_ibwd_dn3 = assign1150_e1486_d_n3;
        var_ibwd_dn4 = assign1150_e1486_d_n4;
        var_ibwd_dn5 = assign1150_e1486_d_n5;
        var_ibwd_dn6 = assign1150_e1486_d_n6;
        var_ibwd_rv = 0.0;

        let (assign1160_e1491, assign1160_e1491_d_n3, assign1160_e1491_d_n4, assign1160_e1491_d_n5, assign1160_e1491_d_n6,) = {
    if (var_guard9 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibwd, var_ibwd_dn3, var_ibwd_dn4, var_ibwd_dn5, var_ibwd_dn6,)
    }
};
        var_ibwd = assign1160_e1491;
        var_ibwd_dn3 = assign1160_e1491_d_n3;
        var_ibwd_dn4 = assign1160_e1491_d_n4;
        var_ibwd_dn5 = assign1160_e1491_d_n5;
        var_ibwd_dn6 = assign1160_e1491_d_n6;
        var_ibwd_rv = 0.0;

        let assign1170_e1494: f64 = if var_isc_t > 0.0 { 1.0 } else { 0.0 };
        var_guard11 = assign1170_e1494;
        var_guard11_rv = 0.0;

        let (assign1180_e1502, assign1180_e1502_d_n3, assign1180_e1502_d_n4, assign1180_e1502_d_n5, assign1180_e1502_d_n6,) = {
    if (var_guard11 != 0.0) {
        let assign1180_e1499: f64 = (p.p65 * var_vt);
        let assign1180_e1500: f64 = (var_vbici / assign1180_e1499);
        (assign1180_e1500, (-((var_vbici * (p.p65 * var_vt_dn3)) / (assign1180_e1499 * assign1180_e1499))), (var_vbici_dn4 / assign1180_e1499), (var_vbici_dn5 / assign1180_e1499), 0.0,)
    } else {
        (var_arg, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6,)
    }
};
        var_arg = assign1180_e1502;
        var_arg_dn3 = assign1180_e1502_d_n3;
        var_arg_dn4 = assign1180_e1502_d_n4;
        var_arg_dn5 = assign1180_e1502_d_n5;
        var_arg_dn6 = assign1180_e1502_d_n6;
        var_arg_rv = 0.0;

        let (assign1190_e1513, assign1190_e1513_d_n3, assign1190_e1513_d_n4, assign1190_e1513_d_n5, assign1190_e1513_d_n6,) = {
    if (var_guard11 != 0.0) {
        let assign1190_e1505: f64 = (-var_vbici);
        let assign1190_e1507: f64 = (assign1190_e1505 - var_bvr_t);
        let assign1190_e1510: f64 = (p.p57 * var_vt);
        let assign1190_e1511: f64 = (assign1190_e1507 / assign1190_e1510);
        (assign1190_e1511, ((((-var_bvr_t_dn3) * assign1190_e1510) - (assign1190_e1507 * (p.p57 * var_vt_dn3))) / (assign1190_e1510 * assign1190_e1510)), ((-var_vbici_dn4) / assign1190_e1510), ((-var_vbici_dn5) / assign1190_e1510), 0.0,)
    } else {
        (var_argbv, var_argbv_dn3, var_argbv_dn4, var_argbv_dn5, var_argbv_dn6,)
    }
};
        var_argbv = assign1190_e1513;
        var_argbv_dn3 = assign1190_e1513_d_n3;
        var_argbv_dn4 = assign1190_e1513_d_n4;
        var_argbv_dn5 = assign1190_e1513_d_n5;
        var_argbv_dn6 = assign1190_e1513_d_n6;
        var_argbv_rv = 0.0;

        let (assign1200_e1522, assign1200_e1522_d_n3,) = {
    if (var_guard11 != 0.0) {
        let assign1200_e1516: f64 = (-var_bvr_t);
        let assign1200_e1519: f64 = (p.p57 * var_vt);
        let assign1200_e1520: f64 = (assign1200_e1516 / assign1200_e1519);
        (assign1200_e1520, ((((-var_bvr_t_dn3) * assign1200_e1519) - (assign1200_e1516 * (p.p57 * var_vt_dn3))) / (assign1200_e1519 * assign1200_e1519)),)
    } else {
        (var_argbvvt, var_argbvvt_dn3,)
    }
};
        var_argbvvt = assign1200_e1522;
        var_argbvvt_dn3 = assign1200_e1522_d_n3;
        var_argbvvt_rv = 0.0;

        let assign1210_e1525: f64 = if var_arg > 80.0 { 1.0 } else { 0.0 };
        var_guard12 = assign1210_e1525;
        var_guard12_rv = 0.0;

        let (assign1220_e1535, assign1220_e1535_d_n3, assign1220_e1535_d_n4, assign1220_e1535_d_n5, assign1220_e1535_d_n6,) = {
    if ((var_guard11 != 0.0) && (var_guard12 != 0.0)) {
        let assign1220_e1532: f64 = (var_arg - 80.0);
        let assign1220_e1533: f64 = (1.0 + assign1220_e1532);
        (assign1220_e1533, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6,)
    } else {
        (var_le, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6,)
    }
};
        var_le = assign1220_e1535;
        var_le_dn3 = assign1220_e1535_d_n3;
        var_le_dn4 = assign1220_e1535_d_n4;
        var_le_dn5 = assign1220_e1535_d_n5;
        var_le_dn6 = assign1220_e1535_d_n6;
        var_le_rv = 0.0;

        let (assign1230_e1541, assign1230_e1541_d_n3, assign1230_e1541_d_n4, assign1230_e1541_d_n5, assign1230_e1541_d_n6,) = {
    if ((var_guard11 != 0.0) && (var_guard12 != 0.0)) {
        (80.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6,)
    }
};
        var_arg = assign1230_e1541;
        var_arg_dn3 = assign1230_e1541_d_n3;
        var_arg_dn4 = assign1230_e1541_d_n4;
        var_arg_dn5 = assign1230_e1541_d_n5;
        var_arg_dn6 = assign1230_e1541_d_n6;
        var_arg_rv = 0.0;

        let (assign1240_e1548, assign1240_e1548_d_n3, assign1240_e1548_d_n4, assign1240_e1548_d_n5, assign1240_e1548_d_n6,) = {
    if ((var_guard11 != 0.0) && (var_guard12 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_le, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6,)
    }
};
        var_le = assign1240_e1548;
        var_le_dn3 = assign1240_e1548_d_n3;
        var_le_dn4 = assign1240_e1548_d_n4;
        var_le_dn5 = assign1240_e1548_d_n5;
        var_le_dn6 = assign1240_e1548_d_n6;
        var_le_rv = 0.0;

        let (assign1250_e1555, assign1250_e1555_d_n3, assign1250_e1555_d_n4, assign1250_e1555_d_n5, assign1250_e1555_d_n6,) = {
    if (var_guard11 != 0.0) {
        let assign1250_e1552: f64 = (var_arg).exp();
        let assign1250_e1553: f64 = (var_le * assign1250_e1552);
        (assign1250_e1553, ((var_le_dn3 * assign1250_e1552) + (var_le * (assign1250_e1552 * var_arg_dn3))), ((var_le_dn4 * assign1250_e1552) + (var_le * (assign1250_e1552 * var_arg_dn4))), ((var_le_dn5 * assign1250_e1552) + (var_le * (assign1250_e1552 * var_arg_dn5))), ((var_le_dn6 * assign1250_e1552) + (var_le * (assign1250_e1552 * var_arg_dn6))),)
    } else {
        (var_le, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6,)
    }
};
        var_le = assign1250_e1555;
        var_le_dn3 = assign1250_e1555_d_n3;
        var_le_dn4 = assign1250_e1555_d_n4;
        var_le_dn5 = assign1250_e1555_d_n5;
        var_le_dn6 = assign1250_e1555_d_n6;
        var_le_rv = 0.0;

        let (assign1260_e1627, assign1260_e1627_d_n3, assign1260_e1627_d_n4, assign1260_e1627_d_n5, assign1260_e1627_d_n6,) = {
    if (var_guard11 != 0.0) {
        let assign1260_e1563: f64 = (-37.0);
        let (assign1260_e1590, assign1260_e1590_d_n3, assign1260_e1590_d_n4, assign1260_e1590_d_n5, assign1260_e1590_d_n6,) = {
            if ((!(var_argbv >= 37.0)) && (!(var_argbv <= assign1260_e1563))) {
                let assign1260_e1568: f64 = (var_argbv).exp();
                let assign1260_e1570: f64 = (assign1260_e1568 + 1.0);
                let assign1260_e1571: f64 = (assign1260_e1570).ln();
                (assign1260_e1571, ((assign1260_e1568 * var_argbv_dn3) / assign1260_e1570), ((assign1260_e1568 * var_argbv_dn4) / assign1260_e1570), ((assign1260_e1568 * var_argbv_dn5) / assign1260_e1570), ((assign1260_e1568 * var_argbv_dn6) / assign1260_e1570),)
            } else {
                let assign1260_e1578: f64 = (-37.0);
                let (assign1260_e1589, assign1260_e1589_d_n3, assign1260_e1589_d_n4, assign1260_e1589_d_n5, assign1260_e1589_d_n6,) = {
                    if ((!(var_argbv >= 37.0)) && (var_argbv <= assign1260_e1578)) {
                        let assign1260_e1582: f64 = (var_argbv).exp();
                        (assign1260_e1582, (assign1260_e1582 * var_argbv_dn3), (assign1260_e1582 * var_argbv_dn4), (assign1260_e1582 * var_argbv_dn5), (assign1260_e1582 * var_argbv_dn6),)
                    } else {
                        let (assign1260_e1588, assign1260_e1588_d_n3, assign1260_e1588_d_n4, assign1260_e1588_d_n5, assign1260_e1588_d_n6,) = {
                            if (var_argbv >= 37.0) {
                                (var_argbv, var_argbv_dn3, var_argbv_dn4, var_argbv_dn5, var_argbv_dn6,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign1260_e1588, assign1260_e1588_d_n3, assign1260_e1588_d_n4, assign1260_e1588_d_n5, assign1260_e1588_d_n6,)
                    }
                };
                (assign1260_e1589, assign1260_e1589_d_n3, assign1260_e1589_d_n4, assign1260_e1589_d_n5, assign1260_e1589_d_n6,)
            }
        };
        let assign1260_e1597: f64 = (-37.0);
        let (assign1260_e1624, assign1260_e1624_d_n3,) = {
            if ((!(var_argbvvt >= 37.0)) && (!(var_argbvvt <= assign1260_e1597))) {
                let assign1260_e1602: f64 = (var_argbvvt).exp();
                let assign1260_e1604: f64 = (assign1260_e1602 + 1.0);
                let assign1260_e1605: f64 = (assign1260_e1604).ln();
                (assign1260_e1605, ((assign1260_e1602 * var_argbvvt_dn3) / assign1260_e1604),)
            } else {
                let assign1260_e1612: f64 = (-37.0);
                let (assign1260_e1623, assign1260_e1623_d_n3,) = {
                    if ((!(var_argbvvt >= 37.0)) && (var_argbvvt <= assign1260_e1612)) {
                        let assign1260_e1616: f64 = (var_argbvvt).exp();
                        (assign1260_e1616, (assign1260_e1616 * var_argbvvt_dn3),)
                    } else {
                        let (assign1260_e1622, assign1260_e1622_d_n3,) = {
                            if (var_argbvvt >= 37.0) {
                                (var_argbvvt, var_argbvvt_dn3,)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign1260_e1622, assign1260_e1622_d_n3,)
                    }
                };
                (assign1260_e1623, assign1260_e1623_d_n3,)
            }
        };
        let assign1260_e1625: f64 = (assign1260_e1590 - assign1260_e1624);
        (assign1260_e1625, (assign1260_e1590_d_n3 - assign1260_e1624_d_n3), assign1260_e1590_d_n4, assign1260_e1590_d_n5, assign1260_e1590_d_n6,)
    } else {
        (var_lebv, var_lebv_dn3, var_lebv_dn4, var_lebv_dn5, var_lebv_dn6,)
    }
};
        var_lebv = assign1260_e1627;
        var_lebv_dn3 = assign1260_e1627_d_n3;
        var_lebv_dn4 = assign1260_e1627_d_n4;
        var_lebv_dn5 = assign1260_e1627_d_n5;
        var_lebv_dn6 = assign1260_e1627_d_n6;
        var_lebv_rv = 0.0;

        let assign1320_e1679: f64 = (var_vbici * p.p81);
        let assign1320_e1680: f64 = (1.0 + assign1320_e1679);
        let assign1320_e1681: f64 = (var_oikf * assign1320_e1680);
        var_oikf = assign1320_e1681;
        var_oikf_dn4 = ((var_oikf_dn4 * assign1320_e1680) + (var_oikf * (var_vbici_dn4 * p.p81)));
        var_oikf_dn5 = ((var_oikf_dn5 * assign1320_e1680) + (var_oikf * (var_vbici_dn5 * p.p81)));
        var_oikf_rv = 0.0;

        let assign1330_e1684: f64 = (var_ifwd * var_oikf);
        let assign1330_e1687: f64 = (var_ibwd * var_oikr);
        let assign1330_e1688: f64 = (assign1330_e1684 + assign1330_e1687);
        var_kq2 = assign1330_e1688;
        var_kq2_dn3 = ((var_ifwd_dn3 * var_oikf) + (var_ibwd_dn3 * var_oikr));
        var_kq2_dn4 = (((var_ifwd_dn4 * var_oikf) + (var_ifwd * var_oikf_dn4)) + (var_ibwd_dn4 * var_oikr));
        var_kq2_dn5 = (((var_ifwd_dn5 * var_oikf) + (var_ifwd * var_oikf_dn5)) + (var_ibwd_dn5 * var_oikr));
        var_kq2_dn6 = ((var_ifwd_dn6 * var_oikf) + (var_ibwd_dn6 * var_oikr));
        var_kq2_rv = 0.0;

        let assign1340_e1692: f64 = (var_vbiei * var_ovar);
        let assign1340_e1693: f64 = (1.0 - assign1340_e1692);
        let assign1340_e1696: f64 = (var_vbici * var_ovaf);
        let assign1340_e1697: f64 = (assign1340_e1693 - assign1340_e1696);
        var_ikq1 = assign1340_e1697;
        var_ikq1_dn4 = (-(var_vbici_dn4 * var_ovaf));
        var_ikq1_dn5 = ((-(var_vbiei_dn5 * var_ovar)) - (var_vbici_dn5 * var_ovaf));
        var_ikq1_dn6 = (-(var_vbiei_dn6 * var_ovar));
        var_ikq1_rv = 0.0;

        let assign1350_e1702: f64 = (4.0 * var_kq2);
        let assign1350_e1703: f64 = (1.0 + assign1350_e1702);
        let assign1350_e1704: f64 = (assign1350_e1703).abs();
        let assign1350_e1706: f64 = (assign1350_e1704).powf(p.p82);
        let assign1350_e1707: f64 = (1.0 + assign1350_e1706);
        var_dkqb = assign1350_e1707;
        var_dkqb_dn3 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn3) } else { (-(4.0 * var_kq2_dn3)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn3) } else { (-(4.0 * var_kq2_dn3)) } / assign1350_e1704))) };
        var_dkqb_dn4 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn4) } else { (-(4.0 * var_kq2_dn4)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn4) } else { (-(4.0 * var_kq2_dn4)) } / assign1350_e1704))) };
        var_dkqb_dn5 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn5) } else { (-(4.0 * var_kq2_dn5)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn5) } else { (-(4.0 * var_kq2_dn5)) } / assign1350_e1704))) };
        var_dkqb_dn6 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn6) } else { (-(4.0 * var_kq2_dn6)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn6) } else { (-(4.0 * var_kq2_dn6)) } / assign1350_e1704))) };
        var_dkqb_rv = 0.0;

        let assign1360_e1710: f64 = (2.0 * var_ikq1);
        let assign1360_e1712: f64 = (assign1360_e1710 / var_dkqb);
        var_ikqb = assign1360_e1712;
        var_ikqb_dn3 = (-((assign1360_e1710 * var_dkqb_dn3) / (var_dkqb * var_dkqb)));
        var_ikqb_dn4 = ((((2.0 * var_ikq1_dn4) * var_dkqb) - (assign1360_e1710 * var_dkqb_dn4)) / (var_dkqb * var_dkqb));
        var_ikqb_dn5 = ((((2.0 * var_ikq1_dn5) * var_dkqb) - (assign1360_e1710 * var_dkqb_dn5)) / (var_dkqb * var_dkqb));
        var_ikqb_dn6 = ((((2.0 * var_ikq1_dn6) * var_dkqb) - (assign1360_e1710 * var_dkqb_dn6)) / (var_dkqb * var_dkqb));
        var_ikqb_rv = 0.0;

        let assign1370_e1715: f64 = (var_ibwd * var_ikqb);
        var_itr = assign1370_e1715;
        var_itr_dn3 = ((var_ibwd_dn3 * var_ikqb) + (var_ibwd * var_ikqb_dn3));
        var_itr_dn4 = ((var_ibwd_dn4 * var_ikqb) + (var_ibwd * var_ikqb_dn4));
        var_itr_dn5 = ((var_ibwd_dn5 * var_ikqb) + (var_ibwd * var_ikqb_dn5));
        var_itr_dn6 = ((var_ibwd_dn6 * var_ikqb) + (var_ibwd * var_ikqb_dn6));
        var_itr_rv = 0.0;

        let assign1380_e1718: f64 = (var_ifwd * var_ikqb);
        var_itzf = assign1380_e1718;
        var_itzf_dn3 = ((var_ifwd_dn3 * var_ikqb) + (var_ifwd * var_ikqb_dn3));
        var_itzf_dn4 = ((var_ifwd_dn4 * var_ikqb) + (var_ifwd * var_ikqb_dn4));
        var_itzf_dn5 = ((var_ifwd_dn5 * var_ikqb) + (var_ifwd * var_ikqb_dn5));
        var_itzf_dn6 = ((var_ifwd_dn6 * var_ikqb) + (var_ifwd * var_ikqb_dn6));
        var_itzf_rv = 0.0;

        let assign1450_e1782: f64 = ((nv1 - nv2) / p.p40);
        let assign1450_e1783: f64 = (assign1450_e1782).abs();
        let assign1450_e1785: f64 = (assign1450_e1783).powf(p.p39);
        var_vtff = assign1450_e1785;
        var_vtff_dn1 = if 0.0 == 0.0 && ((p.p39) as f64).is_finite() && ((p.p39) as f64).fract() == 0.0 { if p.p39 == 0.0 { 0.0 } else { (p.p39 * ((assign1450_e1783).powf(p.p39 - 1.0) * if assign1450_e1782 >= 0.0 { (1.0 / p.p40) } else { (-(1.0 / p.p40)) })) } } else { (assign1450_e1785 * (p.p39 * (if assign1450_e1782 >= 0.0 { (1.0 / p.p40) } else { (-(1.0 / p.p40)) } / assign1450_e1783))) };
        var_vtff_dn2 = if 0.0 == 0.0 && ((p.p39) as f64).is_finite() && ((p.p39) as f64).fract() == 0.0 { if p.p39 == 0.0 { 0.0 } else { (p.p39 * ((assign1450_e1783).powf(p.p39 - 1.0) * if assign1450_e1782 >= 0.0 { (-1.0 / p.p40) } else { (-(-1.0 / p.p40)) })) } } else { (assign1450_e1785 * (p.p39 * (if assign1450_e1782 >= 0.0 { (-1.0 / p.p40) } else { (-(-1.0 / p.p40)) } / assign1450_e1783))) };
        var_vtff_rv = 0.0;

        let assign1460_e1788: f64 = (1.0 + var_vtff);
        let assign1460_e1791: f64 = (1.0 / p.p39);
        let assign1460_e1792: f64 = (assign1460_e1788).powf(assign1460_e1791);
        let assign1460_e1794: f64 = (assign1460_e1792 - 1.0);
        var_vtff1 = assign1460_e1794;
        var_vtff1_dn1 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * var_vtff_dn1)) } } else { (assign1460_e1792 * (assign1460_e1791 * (var_vtff_dn1 / assign1460_e1788))) };
        var_vtff1_dn2 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * var_vtff_dn2)) } } else { (assign1460_e1792 * (assign1460_e1791 * (var_vtff_dn2 / assign1460_e1788))) };
        var_vtff1_rv = 0.0;

        let assign1470_e1799: f64 = (p.p41 * var_vtff1);
        let assign1470_e1800: f64 = (1.0 + assign1470_e1799);
        let assign1470_e1801: f64 = (p.p19 * assign1470_e1800);
        var_tff = assign1470_e1801;
        var_tff_dn1 = (p.p19 * (p.p41 * var_vtff1_dn1));
        var_tff_dn2 = (p.p19 * (p.p41 * var_vtff1_dn2));
        var_tff_rv = 0.0;

        let assign1480_e1804: f64 = (var_tff * var_ifwd);
        var_qde = assign1480_e1804;
        var_qde_dn1 = (var_tff_dn1 * var_ifwd);
        var_qde_dn2 = (var_tff_dn2 * var_ifwd);
        var_qde_dn3 = (var_tff * var_ifwd_dn3);
        var_qde_dn4 = (var_tff * var_ifwd_dn4);
        var_qde_dn5 = (var_tff * var_ifwd_dn5);
        var_qde_dn6 = (var_tff * var_ifwd_dn6);
        var_qde_rv = 0.0;

        let assign1490_e1807: f64 = (p.p73 * var_itr);
        var_qdc = assign1490_e1807;
        var_qdc_dn3 = (p.p73 * var_itr_dn3);
        var_qdc_dn4 = (p.p73 * var_itr_dn4);
        var_qdc_dn5 = (p.p73 * var_itr_dn5);
        var_qdc_dn6 = (p.p73 * var_itr_dn6);
        var_qdc_rv = 0.0;

        let assign1500_e1810: f64 = if p.p32 == 1.0 { 1.0 } else { 0.0 };
        var_guard13 = assign1500_e1810;
        var_guard13_rv = 0.0;

        let assign1570_e1852: f64 = if var_veci <= 0.0 { 1.0 } else { 0.0 };
        var_guard15 = assign1570_e1852;
        var_guard15_rv = 0.0;

        *var_arg_slot = var_arg;
        *var_arg_dn3_slot = var_arg_dn3;
        *var_arg_dn4_slot = var_arg_dn4;
        *var_arg_dn5_slot = var_arg_dn5;
        *var_arg_dn6_slot = var_arg_dn6;
        *var_arg_rv_slot = var_arg_rv;
        *var_argbv_slot = var_argbv;
        *var_argbv_dn3_slot = var_argbv_dn3;
        *var_argbv_dn4_slot = var_argbv_dn4;
        *var_argbv_dn5_slot = var_argbv_dn5;
        *var_argbv_dn6_slot = var_argbv_dn6;
        *var_argbv_rv_slot = var_argbv_rv;
        *var_argbvvt_slot = var_argbvvt;
        *var_argbvvt_dn3_slot = var_argbvvt_dn3;
        *var_argbvvt_rv_slot = var_argbvvt_rv;
        *var_dkqb_slot = var_dkqb;
        *var_dkqb_dn3_slot = var_dkqb_dn3;
        *var_dkqb_dn4_slot = var_dkqb_dn4;
        *var_dkqb_dn5_slot = var_dkqb_dn5;
        *var_dkqb_dn6_slot = var_dkqb_dn6;
        *var_dkqb_rv_slot = var_dkqb_rv;
        *var_guard10_slot = var_guard10;
        *var_guard10_rv_slot = var_guard10_rv;
        *var_guard11_slot = var_guard11;
        *var_guard11_rv_slot = var_guard11_rv;
        *var_guard12_slot = var_guard12;
        *var_guard12_rv_slot = var_guard12_rv;
        *var_guard13_slot = var_guard13;
        *var_guard13_rv_slot = var_guard13_rv;
        *var_guard15_slot = var_guard15;
        *var_guard15_rv_slot = var_guard15_rv;
        *var_ibwd_slot = var_ibwd;
        *var_ibwd_dn3_slot = var_ibwd_dn3;
        *var_ibwd_dn4_slot = var_ibwd_dn4;
        *var_ibwd_dn5_slot = var_ibwd_dn5;
        *var_ibwd_dn6_slot = var_ibwd_dn6;
        *var_ibwd_rv_slot = var_ibwd_rv;
        *var_ikq1_slot = var_ikq1;
        *var_ikq1_dn4_slot = var_ikq1_dn4;
        *var_ikq1_dn5_slot = var_ikq1_dn5;
        *var_ikq1_dn6_slot = var_ikq1_dn6;
        *var_ikq1_rv_slot = var_ikq1_rv;
        *var_ikqb_slot = var_ikqb;
        *var_ikqb_dn3_slot = var_ikqb_dn3;
        *var_ikqb_dn4_slot = var_ikqb_dn4;
        *var_ikqb_dn5_slot = var_ikqb_dn5;
        *var_ikqb_dn6_slot = var_ikqb_dn6;
        *var_ikqb_rv_slot = var_ikqb_rv;
        *var_itr_slot = var_itr;
        *var_itr_dn3_slot = var_itr_dn3;
        *var_itr_dn4_slot = var_itr_dn4;
        *var_itr_dn5_slot = var_itr_dn5;
        *var_itr_dn6_slot = var_itr_dn6;
        *var_itr_rv_slot = var_itr_rv;
        *var_itzf_slot = var_itzf;
        *var_itzf_dn3_slot = var_itzf_dn3;
        *var_itzf_dn4_slot = var_itzf_dn4;
        *var_itzf_dn5_slot = var_itzf_dn5;
        *var_itzf_dn6_slot = var_itzf_dn6;
        *var_itzf_rv_slot = var_itzf_rv;
        *var_kq2_slot = var_kq2;
        *var_kq2_dn3_slot = var_kq2_dn3;
        *var_kq2_dn4_slot = var_kq2_dn4;
        *var_kq2_dn5_slot = var_kq2_dn5;
        *var_kq2_dn6_slot = var_kq2_dn6;
        *var_kq2_rv_slot = var_kq2_rv;
        *var_le_slot = var_le;
        *var_le_dn3_slot = var_le_dn3;
        *var_le_dn4_slot = var_le_dn4;
        *var_le_dn5_slot = var_le_dn5;
        *var_le_dn6_slot = var_le_dn6;
        *var_le_rv_slot = var_le_rv;
        *var_lebv_slot = var_lebv;
        *var_lebv_dn3_slot = var_lebv_dn3;
        *var_lebv_dn4_slot = var_lebv_dn4;
        *var_lebv_dn5_slot = var_lebv_dn5;
        *var_lebv_dn6_slot = var_lebv_dn6;
        *var_lebv_rv_slot = var_lebv_rv;
        *var_oikf_slot = var_oikf;
        *var_oikf_dn4_slot = var_oikf_dn4;
        *var_oikf_dn5_slot = var_oikf_dn5;
        *var_oikf_rv_slot = var_oikf_rv;
        *var_qdc_slot = var_qdc;
        *var_qdc_dn3_slot = var_qdc_dn3;
        *var_qdc_dn4_slot = var_qdc_dn4;
        *var_qdc_dn5_slot = var_qdc_dn5;
        *var_qdc_dn6_slot = var_qdc_dn6;
        *var_qdc_rv_slot = var_qdc_rv;
        *var_qde_slot = var_qde;
        *var_qde_dn1_slot = var_qde_dn1;
        *var_qde_dn2_slot = var_qde_dn2;
        *var_qde_dn3_slot = var_qde_dn3;
        *var_qde_dn4_slot = var_qde_dn4;
        *var_qde_dn5_slot = var_qde_dn5;
        *var_qde_dn6_slot = var_qde_dn6;
        *var_qde_rv_slot = var_qde_rv;
        *var_tff_slot = var_tff;
        *var_tff_dn1_slot = var_tff_dn1;
        *var_tff_dn2_slot = var_tff_dn2;
        *var_tff_rv_slot = var_tff_rv;
        *var_vtff_slot = var_vtff;
        *var_vtff1_slot = var_vtff1;
        *var_vtff1_dn1_slot = var_vtff1_dn1;
        *var_vtff1_dn2_slot = var_vtff1_dn2;
        *var_vtff1_rv_slot = var_vtff1_rv;
        *var_vtff_dn1_slot = var_vtff_dn1;
        *var_vtff_dn2_slot = var_vtff_dn2;
        *var_vtff_rv_slot = var_vtff_rv;
    }

    pub(super) fn stamp_reactive_block_3(
        p: &Parameters,
        var_cjc_t: f64,
        var_cjc_t_dn3: f64,
        var_cje_t: f64,
        var_cje_t_dn3: f64,
        var_cjs_t: f64,
        var_cjs_t_dn3: f64,
        var_guard15: f64,
        var_itzf: f64,
        var_itzf_dn3: f64,
        var_itzf_dn4: f64,
        var_itzf_dn5: f64,
        var_itzf_dn6: f64,
        var_ttype: f64,
        var_vbci: f64,
        var_vbci_dn1: f64,
        var_vbci_dn4: f64,
        var_vbici: f64,
        var_vbici_dn4: f64,
        var_vbici_dn5: f64,
        var_vbiei: f64,
        var_vbiei_dn5: f64,
        var_vbiei_dn6: f64,
        var_veci: f64,
        var_veci_dn2: f64,
        var_veci_dn4: f64,
        var_vjc_t: f64,
        var_vjc_t_dn3: f64,
        var_vje_t: f64,
        var_vje_t_dn3: f64,
        var_vjs_t: f64,
        var_vjs_t_dn3: f64,
        var_dv0_slot: &mut f64,
        var_dv0_dn3_slot: &mut f64,
        var_dv0_rv_slot: &mut f64,
        var_dvh_slot: &mut f64,
        var_dvh_dn1_slot: &mut f64,
        var_dvh_dn3_slot: &mut f64,
        var_dvh_dn4_slot: &mut f64,
        var_dvh_dn5_slot: &mut f64,
        var_dvh_dn6_slot: &mut f64,
        var_dvh_rv_slot: &mut f64,
        var_guard16_slot: &mut f64,
        var_guard16_rv_slot: &mut f64,
        var_guard17_slot: &mut f64,
        var_guard17_rv_slot: &mut f64,
        var_guard18_slot: &mut f64,
        var_guard18_rv_slot: &mut f64,
        var_guard19_slot: &mut f64,
        var_guard19_rv_slot: &mut f64,
        var_guard20_slot: &mut f64,
        var_guard20_rv_slot: &mut f64,
        var_guard21_slot: &mut f64,
        var_guard21_rv_slot: &mut f64,
        var_pwq_slot: &mut f64,
        var_pwq_rv_slot: &mut f64,
        var_qhi_slot: &mut f64,
        var_qhi_dn1_slot: &mut f64,
        var_qhi_dn3_slot: &mut f64,
        var_qhi_dn4_slot: &mut f64,
        var_qhi_dn5_slot: &mut f64,
        var_qhi_dn6_slot: &mut f64,
        var_qhi_rv_slot: &mut f64,
        var_qjci_slot: &mut f64,
        var_qjci_1_slot: &mut f64,
        var_qjci_1_dn1_slot: &mut f64,
        var_qjci_1_dn3_slot: &mut f64,
        var_qjci_1_dn4_slot: &mut f64,
        var_qjci_1_dn5_slot: &mut f64,
        var_qjci_1_dn6_slot: &mut f64,
        var_qjci_1_rv_slot: &mut f64,
        var_qjci_dn1_slot: &mut f64,
        var_qjci_dn3_slot: &mut f64,
        var_qjci_dn4_slot: &mut f64,
        var_qjci_dn5_slot: &mut f64,
        var_qjci_dn6_slot: &mut f64,
        var_qjci_rv_slot: &mut f64,
        var_qjcx_slot: &mut f64,
        var_qjcx_1_slot: &mut f64,
        var_qjcx_1_dn1_slot: &mut f64,
        var_qjcx_1_dn3_slot: &mut f64,
        var_qjcx_1_dn4_slot: &mut f64,
        var_qjcx_1_dn5_slot: &mut f64,
        var_qjcx_1_dn6_slot: &mut f64,
        var_qjcx_1_rv_slot: &mut f64,
        var_qjcx_dn1_slot: &mut f64,
        var_qjcx_dn3_slot: &mut f64,
        var_qjcx_dn4_slot: &mut f64,
        var_qjcx_dn5_slot: &mut f64,
        var_qjcx_dn6_slot: &mut f64,
        var_qjcx_rv_slot: &mut f64,
        var_qje_slot: &mut f64,
        var_qje_dn1_slot: &mut f64,
        var_qje_dn3_slot: &mut f64,
        var_qje_dn4_slot: &mut f64,
        var_qje_dn5_slot: &mut f64,
        var_qje_dn6_slot: &mut f64,
        var_qje_rv_slot: &mut f64,
        var_qjs_slot: &mut f64,
        var_qjs_dn2_slot: &mut f64,
        var_qjs_dn3_slot: &mut f64,
        var_qjs_dn4_slot: &mut f64,
        var_qjs_rv_slot: &mut f64,
        var_qlo_slot: &mut f64,
        var_qlo_dn1_slot: &mut f64,
        var_qlo_dn3_slot: &mut f64,
        var_qlo_dn4_slot: &mut f64,
        var_qlo_dn5_slot: &mut f64,
        var_qlo_dn6_slot: &mut f64,
        var_qlo_rv_slot: &mut f64,
        var_qxf1_slot: &mut f64,
        var_qxf1_dn3_slot: &mut f64,
        var_qxf1_dn4_slot: &mut f64,
        var_qxf1_dn5_slot: &mut f64,
        var_qxf1_dn6_slot: &mut f64,
        var_qxf1_rv_slot: &mut f64,
    ) {
        let mut var_dv0: f64 = *var_dv0_slot;
        let mut var_dv0_dn3: f64 = *var_dv0_dn3_slot;
        let mut var_dv0_rv: f64 = *var_dv0_rv_slot;
        let mut var_dvh: f64 = *var_dvh_slot;
        let mut var_dvh_dn1: f64 = *var_dvh_dn1_slot;
        let mut var_dvh_dn3: f64 = *var_dvh_dn3_slot;
        let mut var_dvh_dn4: f64 = *var_dvh_dn4_slot;
        let mut var_dvh_dn5: f64 = *var_dvh_dn5_slot;
        let mut var_dvh_dn6: f64 = *var_dvh_dn6_slot;
        let mut var_dvh_rv: f64 = *var_dvh_rv_slot;
        let mut var_guard16: f64 = *var_guard16_slot;
        let mut var_guard16_rv: f64 = *var_guard16_rv_slot;
        let mut var_guard17: f64 = *var_guard17_slot;
        let mut var_guard17_rv: f64 = *var_guard17_rv_slot;
        let mut var_guard18: f64 = *var_guard18_slot;
        let mut var_guard18_rv: f64 = *var_guard18_rv_slot;
        let mut var_guard19: f64 = *var_guard19_slot;
        let mut var_guard19_rv: f64 = *var_guard19_rv_slot;
        let mut var_guard20: f64 = *var_guard20_slot;
        let mut var_guard20_rv: f64 = *var_guard20_rv_slot;
        let mut var_guard21: f64 = *var_guard21_slot;
        let mut var_guard21_rv: f64 = *var_guard21_rv_slot;
        let mut var_pwq: f64 = *var_pwq_slot;
        let mut var_pwq_rv: f64 = *var_pwq_rv_slot;
        let mut var_qhi: f64 = *var_qhi_slot;
        let mut var_qhi_dn1: f64 = *var_qhi_dn1_slot;
        let mut var_qhi_dn3: f64 = *var_qhi_dn3_slot;
        let mut var_qhi_dn4: f64 = *var_qhi_dn4_slot;
        let mut var_qhi_dn5: f64 = *var_qhi_dn5_slot;
        let mut var_qhi_dn6: f64 = *var_qhi_dn6_slot;
        let mut var_qhi_rv: f64 = *var_qhi_rv_slot;
        let mut var_qjci: f64 = *var_qjci_slot;
        let mut var_qjci_1: f64 = *var_qjci_1_slot;
        let mut var_qjci_1_dn1: f64 = *var_qjci_1_dn1_slot;
        let mut var_qjci_1_dn3: f64 = *var_qjci_1_dn3_slot;
        let mut var_qjci_1_dn4: f64 = *var_qjci_1_dn4_slot;
        let mut var_qjci_1_dn5: f64 = *var_qjci_1_dn5_slot;
        let mut var_qjci_1_dn6: f64 = *var_qjci_1_dn6_slot;
        let mut var_qjci_1_rv: f64 = *var_qjci_1_rv_slot;
        let mut var_qjci_dn1: f64 = *var_qjci_dn1_slot;
        let mut var_qjci_dn3: f64 = *var_qjci_dn3_slot;
        let mut var_qjci_dn4: f64 = *var_qjci_dn4_slot;
        let mut var_qjci_dn5: f64 = *var_qjci_dn5_slot;
        let mut var_qjci_dn6: f64 = *var_qjci_dn6_slot;
        let mut var_qjci_rv: f64 = *var_qjci_rv_slot;
        let mut var_qjcx: f64 = *var_qjcx_slot;
        let mut var_qjcx_1: f64 = *var_qjcx_1_slot;
        let mut var_qjcx_1_dn1: f64 = *var_qjcx_1_dn1_slot;
        let mut var_qjcx_1_dn3: f64 = *var_qjcx_1_dn3_slot;
        let mut var_qjcx_1_dn4: f64 = *var_qjcx_1_dn4_slot;
        let mut var_qjcx_1_dn5: f64 = *var_qjcx_1_dn5_slot;
        let mut var_qjcx_1_dn6: f64 = *var_qjcx_1_dn6_slot;
        let mut var_qjcx_1_rv: f64 = *var_qjcx_1_rv_slot;
        let mut var_qjcx_dn1: f64 = *var_qjcx_dn1_slot;
        let mut var_qjcx_dn3: f64 = *var_qjcx_dn3_slot;
        let mut var_qjcx_dn4: f64 = *var_qjcx_dn4_slot;
        let mut var_qjcx_dn5: f64 = *var_qjcx_dn5_slot;
        let mut var_qjcx_dn6: f64 = *var_qjcx_dn6_slot;
        let mut var_qjcx_rv: f64 = *var_qjcx_rv_slot;
        let mut var_qje: f64 = *var_qje_slot;
        let mut var_qje_dn1: f64 = *var_qje_dn1_slot;
        let mut var_qje_dn3: f64 = *var_qje_dn3_slot;
        let mut var_qje_dn4: f64 = *var_qje_dn4_slot;
        let mut var_qje_dn5: f64 = *var_qje_dn5_slot;
        let mut var_qje_dn6: f64 = *var_qje_dn6_slot;
        let mut var_qje_rv: f64 = *var_qje_rv_slot;
        let mut var_qjs: f64 = *var_qjs_slot;
        let mut var_qjs_dn2: f64 = *var_qjs_dn2_slot;
        let mut var_qjs_dn3: f64 = *var_qjs_dn3_slot;
        let mut var_qjs_dn4: f64 = *var_qjs_dn4_slot;
        let mut var_qjs_rv: f64 = *var_qjs_rv_slot;
        let mut var_qlo: f64 = *var_qlo_slot;
        let mut var_qlo_dn1: f64 = *var_qlo_dn1_slot;
        let mut var_qlo_dn3: f64 = *var_qlo_dn3_slot;
        let mut var_qlo_dn4: f64 = *var_qlo_dn4_slot;
        let mut var_qlo_dn5: f64 = *var_qlo_dn5_slot;
        let mut var_qlo_dn6: f64 = *var_qlo_dn6_slot;
        let mut var_qlo_rv: f64 = *var_qlo_rv_slot;
        let mut var_qxf1: f64 = *var_qxf1_slot;
        let mut var_qxf1_dn3: f64 = *var_qxf1_dn3_slot;
        let mut var_qxf1_dn4: f64 = *var_qxf1_dn4_slot;
        let mut var_qxf1_dn5: f64 = *var_qxf1_dn5_slot;
        let mut var_qxf1_dn6: f64 = *var_qxf1_dn6_slot;
        let mut var_qxf1_rv: f64 = *var_qxf1_rv_slot;

        let (assign1580_e1876, assign1580_e1876_d_n2, assign1580_e1876_d_n3, assign1580_e1876_d_n4,) = {
    if (var_guard15 != 0.0) {
        let assign1580_e1856: f64 = (var_cjs_t * var_vjs_t);
        let assign1580_e1860: f64 = (1.0 - p.p76);
        let assign1580_e1864: f64 = (var_veci / var_vjs_t);
        let assign1580_e1865: f64 = (1.0 - assign1580_e1864);
        let assign1580_e1866: f64 = (assign1580_e1865).ln();
        let assign1580_e1867: f64 = (assign1580_e1860 * assign1580_e1866);
        let assign1580_e1868: f64 = (assign1580_e1867).exp();
        let assign1580_e1869: f64 = (1.0 - assign1580_e1868);
        let assign1580_e1870: f64 = (assign1580_e1856 * assign1580_e1869);
        let assign1580_e1873: f64 = (1.0 - p.p76);
        let assign1580_e1874: f64 = (assign1580_e1870 / assign1580_e1873);
        (assign1580_e1874, ((assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(var_veci_dn2 / var_vjs_t)) / assign1580_e1865))))) / assign1580_e1873), (((((var_cjs_t_dn3 * var_vjs_t) + (var_cjs_t * var_vjs_t_dn3)) * assign1580_e1869) + (assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(-((var_veci * var_vjs_t_dn3) / (var_vjs_t * var_vjs_t)))) / assign1580_e1865)))))) / assign1580_e1873), ((assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(var_veci_dn4 / var_vjs_t)) / assign1580_e1865))))) / assign1580_e1873),)
    } else {
        (var_qjs, var_qjs_dn2, var_qjs_dn3, var_qjs_dn4,)
    }
};
        var_qjs = assign1580_e1876;
        var_qjs_dn2 = assign1580_e1876_d_n2;
        var_qjs_dn3 = assign1580_e1876_d_n3;
        var_qjs_dn4 = assign1580_e1876_d_n4;
        var_qjs_rv = 0.0;

        let (assign1590_e1893, assign1590_e1893_d_n2, assign1590_e1893_d_n3, assign1590_e1893_d_n4,) = {
    if (var_guard15 == 0.0) {
        let assign1590_e1881: f64 = (var_cjs_t * var_veci);
        let assign1590_e1885: f64 = (0.5 * p.p76);
        let assign1590_e1887: f64 = (assign1590_e1885 * var_veci);
        let assign1590_e1889: f64 = (assign1590_e1887 / var_vjs_t);
        let assign1590_e1890: f64 = (1.0 + assign1590_e1889);
        let assign1590_e1891: f64 = (assign1590_e1881 * assign1590_e1890);
        (assign1590_e1891, (((var_cjs_t * var_veci_dn2) * assign1590_e1890) + (assign1590_e1881 * ((assign1590_e1885 * var_veci_dn2) / var_vjs_t))), (((var_cjs_t_dn3 * var_veci) * assign1590_e1890) + (assign1590_e1881 * (-((assign1590_e1887 * var_vjs_t_dn3) / (var_vjs_t * var_vjs_t))))), (((var_cjs_t * var_veci_dn4) * assign1590_e1890) + (assign1590_e1881 * ((assign1590_e1885 * var_veci_dn4) / var_vjs_t))),)
    } else {
        (var_qjs, var_qjs_dn2, var_qjs_dn3, var_qjs_dn4,)
    }
};
        var_qjs = assign1590_e1893;
        var_qjs_dn2 = assign1590_e1893_d_n2;
        var_qjs_dn3 = assign1590_e1893_d_n3;
        var_qjs_dn4 = assign1590_e1893_d_n4;
        var_qjs_rv = 0.0;

        let assign1600_e1895: f64 = (-var_vje_t);
        let assign1600_e1897: f64 = (assign1600_e1895 * p.p24);
        var_dv0 = assign1600_e1897;
        var_dv0_dn3 = ((-var_vje_t_dn3) * p.p24);
        var_dv0_rv = 0.0;

        let assign1610_e1900: f64 = (var_vbiei + var_dv0);
        var_dvh = assign1610_e1900;
        var_dvh_dn1 = 0.0;
        var_dvh_dn3 = var_dv0_dn3;
        var_dvh_dn4 = 0.0;
        var_dvh_dn5 = var_vbiei_dn5;
        var_dvh_dn6 = var_vbiei_dn6;
        var_dvh_rv = 0.0;

        let assign1620_e1903: f64 = if var_dvh > 0.0 { 1.0 } else { 0.0 };
        var_guard16 = assign1620_e1903;
        var_guard16_rv = 0.0;

        let (assign1630_e1916,) = {
    if (var_guard16 != 0.0) {
        let assign1630_e1906: f64 = (-1.0);
        let assign1630_e1908: f64 = (assign1630_e1906 - p.p18);
        let assign1630_e1911: f64 = (1.0 - p.p24);
        let assign1630_e1912: f64 = (assign1630_e1911).ln();
        let assign1630_e1913: f64 = (assign1630_e1908 * assign1630_e1912);
        let assign1630_e1914: f64 = (assign1630_e1913).exp();
        (assign1630_e1914,)
    } else {
        (var_pwq,)
    }
};
        var_pwq = assign1630_e1916;
        var_pwq_rv = 0.0;

        let (assign1640_e1936, assign1640_e1936_d_n1, assign1640_e1936_d_n3, assign1640_e1936_d_n4, assign1640_e1936_d_n5, assign1640_e1936_d_n6,) = {
    if (var_guard16 != 0.0) {
        let assign1640_e1923: f64 = (1.0 - p.p24);
        let assign1640_e1924: f64 = (var_pwq * assign1640_e1923);
        let assign1640_e1927: f64 = (1.0 - p.p24);
        let assign1640_e1928: f64 = (assign1640_e1924 * assign1640_e1927);
        let assign1640_e1929: f64 = (1.0 - assign1640_e1928);
        let assign1640_e1930: f64 = (var_vje_t * assign1640_e1929);
        let assign1640_e1933: f64 = (1.0 - p.p18);
        let assign1640_e1934: f64 = (assign1640_e1930 / assign1640_e1933);
        (assign1640_e1934, 0.0, ((var_vje_t_dn3 * assign1640_e1929) / assign1640_e1933), 0.0, 0.0, 0.0,)
    } else {
        (var_qlo, var_qlo_dn1, var_qlo_dn3, var_qlo_dn4, var_qlo_dn5, var_qlo_dn6,)
    }
};
        var_qlo = assign1640_e1936;
        var_qlo_dn1 = assign1640_e1936_d_n1;
        var_qlo_dn3 = assign1640_e1936_d_n3;
        var_qlo_dn4 = assign1640_e1936_d_n4;
        var_qlo_dn5 = assign1640_e1936_d_n5;
        var_qlo_dn6 = assign1640_e1936_d_n6;
        var_qlo_rv = 0.0;

        let (assign1650_e1954, assign1650_e1954_d_n1, assign1650_e1954_d_n3, assign1650_e1954_d_n4, assign1650_e1954_d_n5, assign1650_e1954_d_n6,) = {
    if (var_guard16 != 0.0) {
        let assign1650_e1941: f64 = (1.0 - p.p24);
        let assign1650_e1944: f64 = (0.5 * p.p18);
        let assign1650_e1946: f64 = (assign1650_e1944 * var_dvh);
        let assign1650_e1948: f64 = (assign1650_e1946 / var_vje_t);
        let assign1650_e1949: f64 = (assign1650_e1941 + assign1650_e1948);
        let assign1650_e1950: f64 = (var_dvh * assign1650_e1949);
        let assign1650_e1952: f64 = (assign1650_e1950 * var_pwq);
        (assign1650_e1952, (((var_dvh_dn1 * assign1650_e1949) + (var_dvh * ((assign1650_e1944 * var_dvh_dn1) / var_vje_t))) * var_pwq), (((var_dvh_dn3 * assign1650_e1949) + (var_dvh * ((((assign1650_e1944 * var_dvh_dn3) * var_vje_t) - (assign1650_e1946 * var_vje_t_dn3)) / (var_vje_t * var_vje_t)))) * var_pwq), (((var_dvh_dn4 * assign1650_e1949) + (var_dvh * ((assign1650_e1944 * var_dvh_dn4) / var_vje_t))) * var_pwq), (((var_dvh_dn5 * assign1650_e1949) + (var_dvh * ((assign1650_e1944 * var_dvh_dn5) / var_vje_t))) * var_pwq), (((var_dvh_dn6 * assign1650_e1949) + (var_dvh * ((assign1650_e1944 * var_dvh_dn6) / var_vje_t))) * var_pwq),)
    } else {
        (var_qhi, var_qhi_dn1, var_qhi_dn3, var_qhi_dn4, var_qhi_dn5, var_qhi_dn6,)
    }
};
        var_qhi = assign1650_e1954;
        var_qhi_dn1 = assign1650_e1954_d_n1;
        var_qhi_dn3 = assign1650_e1954_d_n3;
        var_qhi_dn4 = assign1650_e1954_d_n4;
        var_qhi_dn5 = assign1650_e1954_d_n5;
        var_qhi_dn6 = assign1650_e1954_d_n6;
        var_qhi_rv = 0.0;

        let (assign1660_e1977, assign1660_e1977_d_n1, assign1660_e1977_d_n3, assign1660_e1977_d_n4, assign1660_e1977_d_n5, assign1660_e1977_d_n6,) = {
    if (var_guard16 == 0.0) {
        let assign1660_e1961: f64 = (1.0 - p.p18);
        let assign1660_e1965: f64 = (var_vbiei / var_vje_t);
        let assign1660_e1966: f64 = (1.0 - assign1660_e1965);
        let assign1660_e1967: f64 = (assign1660_e1966).ln();
        let assign1660_e1968: f64 = (assign1660_e1961 * assign1660_e1967);
        let assign1660_e1969: f64 = (assign1660_e1968).exp();
        let assign1660_e1970: f64 = (1.0 - assign1660_e1969);
        let assign1660_e1971: f64 = (var_vje_t * assign1660_e1970);
        let assign1660_e1974: f64 = (1.0 - p.p18);
        let assign1660_e1975: f64 = (assign1660_e1971 / assign1660_e1974);
        (assign1660_e1975, 0.0, (((var_vje_t_dn3 * assign1660_e1970) + (var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(-((var_vbiei * var_vje_t_dn3) / (var_vje_t * var_vje_t)))) / assign1660_e1966)))))) / assign1660_e1974), 0.0, ((var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(var_vbiei_dn5 / var_vje_t)) / assign1660_e1966))))) / assign1660_e1974), ((var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(var_vbiei_dn6 / var_vje_t)) / assign1660_e1966))))) / assign1660_e1974),)
    } else {
        (var_qlo, var_qlo_dn1, var_qlo_dn3, var_qlo_dn4, var_qlo_dn5, var_qlo_dn6,)
    }
};
        var_qlo = assign1660_e1977;
        var_qlo_dn1 = assign1660_e1977_d_n1;
        var_qlo_dn3 = assign1660_e1977_d_n3;
        var_qlo_dn4 = assign1660_e1977_d_n4;
        var_qlo_dn5 = assign1660_e1977_d_n5;
        var_qlo_dn6 = assign1660_e1977_d_n6;
        var_qlo_rv = 0.0;

        let (assign1670_e1982, assign1670_e1982_d_n1, assign1670_e1982_d_n3, assign1670_e1982_d_n4, assign1670_e1982_d_n5, assign1670_e1982_d_n6,) = {
    if (var_guard16 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qhi, var_qhi_dn1, var_qhi_dn3, var_qhi_dn4, var_qhi_dn5, var_qhi_dn6,)
    }
};
        var_qhi = assign1670_e1982;
        var_qhi_dn1 = assign1670_e1982_d_n1;
        var_qhi_dn3 = assign1670_e1982_d_n3;
        var_qhi_dn4 = assign1670_e1982_d_n4;
        var_qhi_dn5 = assign1670_e1982_d_n5;
        var_qhi_dn6 = assign1670_e1982_d_n6;
        var_qhi_rv = 0.0;

        let assign1680_e1986: f64 = (var_qlo + var_qhi);
        let assign1680_e1987: f64 = (var_cje_t * assign1680_e1986);
        var_qje = assign1680_e1987;
        var_qje_dn1 = (var_cje_t * (var_qlo_dn1 + var_qhi_dn1));
        var_qje_dn3 = ((var_cje_t_dn3 * assign1680_e1986) + (var_cje_t * (var_qlo_dn3 + var_qhi_dn3)));
        var_qje_dn4 = (var_cje_t * (var_qlo_dn4 + var_qhi_dn4));
        var_qje_dn5 = (var_cje_t * (var_qlo_dn5 + var_qhi_dn5));
        var_qje_dn6 = (var_cje_t * (var_qlo_dn6 + var_qhi_dn6));
        var_qje_rv = 0.0;

        let assign1690_e1989: f64 = (-var_vjc_t);
        let assign1690_e1991: f64 = (assign1690_e1989 * p.p24);
        var_dv0 = assign1690_e1991;
        var_dv0_dn3 = ((-var_vjc_t_dn3) * p.p24);
        var_dv0_rv = 0.0;

        let assign1700_e1994: f64 = (var_vbci + var_dv0);
        var_dvh = assign1700_e1994;
        var_dvh_dn1 = var_vbci_dn1;
        var_dvh_dn3 = var_dv0_dn3;
        var_dvh_dn4 = var_vbci_dn4;
        var_dvh_dn5 = 0.0;
        var_dvh_dn6 = 0.0;
        var_dvh_rv = 0.0;

        let assign1710_e1997: f64 = if var_dvh > 0.0 { 1.0 } else { 0.0 };
        var_guard17 = assign1710_e1997;
        var_guard17_rv = 0.0;

        let (assign1720_e2010,) = {
    if (var_guard17 != 0.0) {
        let assign1720_e2000: f64 = (-1.0);
        let assign1720_e2002: f64 = (assign1720_e2000 - p.p71);
        let assign1720_e2005: f64 = (1.0 - p.p24);
        let assign1720_e2006: f64 = (assign1720_e2005).ln();
        let assign1720_e2007: f64 = (assign1720_e2002 * assign1720_e2006);
        let assign1720_e2008: f64 = (assign1720_e2007).exp();
        (assign1720_e2008,)
    } else {
        (var_pwq,)
    }
};
        var_pwq = assign1720_e2010;
        var_pwq_rv = 0.0;

        let (assign1730_e2030, assign1730_e2030_d_n1, assign1730_e2030_d_n3, assign1730_e2030_d_n4, assign1730_e2030_d_n5, assign1730_e2030_d_n6,) = {
    if (var_guard17 != 0.0) {
        let assign1730_e2017: f64 = (1.0 - p.p24);
        let assign1730_e2018: f64 = (var_pwq * assign1730_e2017);
        let assign1730_e2021: f64 = (1.0 - p.p24);
        let assign1730_e2022: f64 = (assign1730_e2018 * assign1730_e2021);
        let assign1730_e2023: f64 = (1.0 - assign1730_e2022);
        let assign1730_e2024: f64 = (var_vjc_t * assign1730_e2023);
        let assign1730_e2027: f64 = (1.0 - p.p71);
        let assign1730_e2028: f64 = (assign1730_e2024 / assign1730_e2027);
        (assign1730_e2028, 0.0, ((var_vjc_t_dn3 * assign1730_e2023) / assign1730_e2027), 0.0, 0.0, 0.0,)
    } else {
        (var_qlo, var_qlo_dn1, var_qlo_dn3, var_qlo_dn4, var_qlo_dn5, var_qlo_dn6,)
    }
};
        var_qlo = assign1730_e2030;
        var_qlo_dn1 = assign1730_e2030_d_n1;
        var_qlo_dn3 = assign1730_e2030_d_n3;
        var_qlo_dn4 = assign1730_e2030_d_n4;
        var_qlo_dn5 = assign1730_e2030_d_n5;
        var_qlo_dn6 = assign1730_e2030_d_n6;
        var_qlo_rv = 0.0;

        let (assign1740_e2048, assign1740_e2048_d_n1, assign1740_e2048_d_n3, assign1740_e2048_d_n4, assign1740_e2048_d_n5, assign1740_e2048_d_n6,) = {
    if (var_guard17 != 0.0) {
        let assign1740_e2035: f64 = (1.0 - p.p24);
        let assign1740_e2038: f64 = (0.5 * p.p71);
        let assign1740_e2040: f64 = (assign1740_e2038 * var_dvh);
        let assign1740_e2042: f64 = (assign1740_e2040 / var_vjc_t);
        let assign1740_e2043: f64 = (assign1740_e2035 + assign1740_e2042);
        let assign1740_e2044: f64 = (var_dvh * assign1740_e2043);
        let assign1740_e2046: f64 = (assign1740_e2044 * var_pwq);
        (assign1740_e2046, (((var_dvh_dn1 * assign1740_e2043) + (var_dvh * ((assign1740_e2038 * var_dvh_dn1) / var_vjc_t))) * var_pwq), (((var_dvh_dn3 * assign1740_e2043) + (var_dvh * ((((assign1740_e2038 * var_dvh_dn3) * var_vjc_t) - (assign1740_e2040 * var_vjc_t_dn3)) / (var_vjc_t * var_vjc_t)))) * var_pwq), (((var_dvh_dn4 * assign1740_e2043) + (var_dvh * ((assign1740_e2038 * var_dvh_dn4) / var_vjc_t))) * var_pwq), (((var_dvh_dn5 * assign1740_e2043) + (var_dvh * ((assign1740_e2038 * var_dvh_dn5) / var_vjc_t))) * var_pwq), (((var_dvh_dn6 * assign1740_e2043) + (var_dvh * ((assign1740_e2038 * var_dvh_dn6) / var_vjc_t))) * var_pwq),)
    } else {
        (var_qhi, var_qhi_dn1, var_qhi_dn3, var_qhi_dn4, var_qhi_dn5, var_qhi_dn6,)
    }
};
        var_qhi = assign1740_e2048;
        var_qhi_dn1 = assign1740_e2048_d_n1;
        var_qhi_dn3 = assign1740_e2048_d_n3;
        var_qhi_dn4 = assign1740_e2048_d_n4;
        var_qhi_dn5 = assign1740_e2048_d_n5;
        var_qhi_dn6 = assign1740_e2048_d_n6;
        var_qhi_rv = 0.0;

        let (assign1750_e2071, assign1750_e2071_d_n1, assign1750_e2071_d_n3, assign1750_e2071_d_n4, assign1750_e2071_d_n5, assign1750_e2071_d_n6,) = {
    if (var_guard17 == 0.0) {
        let assign1750_e2055: f64 = (1.0 - p.p71);
        let assign1750_e2059: f64 = (var_vbci / var_vjc_t);
        let assign1750_e2060: f64 = (1.0 - assign1750_e2059);
        let assign1750_e2061: f64 = (assign1750_e2060).ln();
        let assign1750_e2062: f64 = (assign1750_e2055 * assign1750_e2061);
        let assign1750_e2063: f64 = (assign1750_e2062).exp();
        let assign1750_e2064: f64 = (1.0 - assign1750_e2063);
        let assign1750_e2065: f64 = (var_vjc_t * assign1750_e2064);
        let assign1750_e2068: f64 = (1.0 - p.p71);
        let assign1750_e2069: f64 = (assign1750_e2065 / assign1750_e2068);
        (assign1750_e2069, ((var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(var_vbci_dn1 / var_vjc_t)) / assign1750_e2060))))) / assign1750_e2068), (((var_vjc_t_dn3 * assign1750_e2064) + (var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(-((var_vbci * var_vjc_t_dn3) / (var_vjc_t * var_vjc_t)))) / assign1750_e2060)))))) / assign1750_e2068), ((var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(var_vbci_dn4 / var_vjc_t)) / assign1750_e2060))))) / assign1750_e2068), 0.0, 0.0,)
    } else {
        (var_qlo, var_qlo_dn1, var_qlo_dn3, var_qlo_dn4, var_qlo_dn5, var_qlo_dn6,)
    }
};
        var_qlo = assign1750_e2071;
        var_qlo_dn1 = assign1750_e2071_d_n1;
        var_qlo_dn3 = assign1750_e2071_d_n3;
        var_qlo_dn4 = assign1750_e2071_d_n4;
        var_qlo_dn5 = assign1750_e2071_d_n5;
        var_qlo_dn6 = assign1750_e2071_d_n6;
        var_qlo_rv = 0.0;

        let (assign1760_e2076, assign1760_e2076_d_n1, assign1760_e2076_d_n3, assign1760_e2076_d_n4, assign1760_e2076_d_n5, assign1760_e2076_d_n6,) = {
    if (var_guard17 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qhi, var_qhi_dn1, var_qhi_dn3, var_qhi_dn4, var_qhi_dn5, var_qhi_dn6,)
    }
};
        var_qhi = assign1760_e2076;
        var_qhi_dn1 = assign1760_e2076_d_n1;
        var_qhi_dn3 = assign1760_e2076_d_n3;
        var_qhi_dn4 = assign1760_e2076_d_n4;
        var_qhi_dn5 = assign1760_e2076_d_n5;
        var_qhi_dn6 = assign1760_e2076_d_n6;
        var_qhi_rv = 0.0;

        let assign1770_e2080: f64 = (var_qlo + var_qhi);
        let assign1770_e2081: f64 = (var_cjc_t * assign1770_e2080);
        var_qjcx = assign1770_e2081;
        var_qjcx_dn1 = (var_cjc_t * (var_qlo_dn1 + var_qhi_dn1));
        var_qjcx_dn3 = ((var_cjc_t_dn3 * assign1770_e2080) + (var_cjc_t * (var_qlo_dn3 + var_qhi_dn3)));
        var_qjcx_dn4 = (var_cjc_t * (var_qlo_dn4 + var_qhi_dn4));
        var_qjcx_dn5 = (var_cjc_t * (var_qlo_dn5 + var_qhi_dn5));
        var_qjcx_dn6 = (var_cjc_t * (var_qlo_dn6 + var_qhi_dn6));
        var_qjcx_rv = 0.0;

        let assign1780_e2084: f64 = (1.0 - p.p72);
        let assign1780_e2086: f64 = (assign1780_e2084 * var_qjcx);
        var_qjcx_1 = assign1780_e2086;
        var_qjcx_1_dn1 = (assign1780_e2084 * var_qjcx_dn1);
        var_qjcx_1_dn3 = (assign1780_e2084 * var_qjcx_dn3);
        var_qjcx_1_dn4 = (assign1780_e2084 * var_qjcx_dn4);
        var_qjcx_1_dn5 = (assign1780_e2084 * var_qjcx_dn5);
        var_qjcx_1_dn6 = (assign1780_e2084 * var_qjcx_dn6);
        var_qjcx_1_rv = 0.0;

        let assign1790_e2088: f64 = (-var_vjc_t);
        let assign1790_e2090: f64 = (assign1790_e2088 * p.p24);
        var_dv0 = assign1790_e2090;
        var_dv0_dn3 = ((-var_vjc_t_dn3) * p.p24);
        var_dv0_rv = 0.0;

        let assign1800_e2093: f64 = (var_vbici + var_dv0);
        var_dvh = assign1800_e2093;
        var_dvh_dn1 = 0.0;
        var_dvh_dn3 = var_dv0_dn3;
        var_dvh_dn4 = var_vbici_dn4;
        var_dvh_dn5 = var_vbici_dn5;
        var_dvh_dn6 = 0.0;
        var_dvh_rv = 0.0;

        let assign1810_e2096: f64 = if var_dvh > 0.0 { 1.0 } else { 0.0 };
        var_guard18 = assign1810_e2096;
        var_guard18_rv = 0.0;

        let (assign1820_e2109,) = {
    if (var_guard18 != 0.0) {
        let assign1820_e2099: f64 = (-1.0);
        let assign1820_e2101: f64 = (assign1820_e2099 - p.p71);
        let assign1820_e2104: f64 = (1.0 - p.p24);
        let assign1820_e2105: f64 = (assign1820_e2104).ln();
        let assign1820_e2106: f64 = (assign1820_e2101 * assign1820_e2105);
        let assign1820_e2107: f64 = (assign1820_e2106).exp();
        (assign1820_e2107,)
    } else {
        (var_pwq,)
    }
};
        var_pwq = assign1820_e2109;
        var_pwq_rv = 0.0;

        let (assign1830_e2129, assign1830_e2129_d_n1, assign1830_e2129_d_n3, assign1830_e2129_d_n4, assign1830_e2129_d_n5, assign1830_e2129_d_n6,) = {
    if (var_guard18 != 0.0) {
        let assign1830_e2116: f64 = (1.0 - p.p24);
        let assign1830_e2117: f64 = (var_pwq * assign1830_e2116);
        let assign1830_e2120: f64 = (1.0 - p.p24);
        let assign1830_e2121: f64 = (assign1830_e2117 * assign1830_e2120);
        let assign1830_e2122: f64 = (1.0 - assign1830_e2121);
        let assign1830_e2123: f64 = (var_vjc_t * assign1830_e2122);
        let assign1830_e2126: f64 = (1.0 - p.p71);
        let assign1830_e2127: f64 = (assign1830_e2123 / assign1830_e2126);
        (assign1830_e2127, 0.0, ((var_vjc_t_dn3 * assign1830_e2122) / assign1830_e2126), 0.0, 0.0, 0.0,)
    } else {
        (var_qlo, var_qlo_dn1, var_qlo_dn3, var_qlo_dn4, var_qlo_dn5, var_qlo_dn6,)
    }
};
        var_qlo = assign1830_e2129;
        var_qlo_dn1 = assign1830_e2129_d_n1;
        var_qlo_dn3 = assign1830_e2129_d_n3;
        var_qlo_dn4 = assign1830_e2129_d_n4;
        var_qlo_dn5 = assign1830_e2129_d_n5;
        var_qlo_dn6 = assign1830_e2129_d_n6;
        var_qlo_rv = 0.0;

        let (assign1840_e2147, assign1840_e2147_d_n1, assign1840_e2147_d_n3, assign1840_e2147_d_n4, assign1840_e2147_d_n5, assign1840_e2147_d_n6,) = {
    if (var_guard18 != 0.0) {
        let assign1840_e2134: f64 = (1.0 - p.p24);
        let assign1840_e2137: f64 = (0.5 * p.p71);
        let assign1840_e2139: f64 = (assign1840_e2137 * var_dvh);
        let assign1840_e2141: f64 = (assign1840_e2139 / var_vjc_t);
        let assign1840_e2142: f64 = (assign1840_e2134 + assign1840_e2141);
        let assign1840_e2143: f64 = (var_dvh * assign1840_e2142);
        let assign1840_e2145: f64 = (assign1840_e2143 * var_pwq);
        (assign1840_e2145, (((var_dvh_dn1 * assign1840_e2142) + (var_dvh * ((assign1840_e2137 * var_dvh_dn1) / var_vjc_t))) * var_pwq), (((var_dvh_dn3 * assign1840_e2142) + (var_dvh * ((((assign1840_e2137 * var_dvh_dn3) * var_vjc_t) - (assign1840_e2139 * var_vjc_t_dn3)) / (var_vjc_t * var_vjc_t)))) * var_pwq), (((var_dvh_dn4 * assign1840_e2142) + (var_dvh * ((assign1840_e2137 * var_dvh_dn4) / var_vjc_t))) * var_pwq), (((var_dvh_dn5 * assign1840_e2142) + (var_dvh * ((assign1840_e2137 * var_dvh_dn5) / var_vjc_t))) * var_pwq), (((var_dvh_dn6 * assign1840_e2142) + (var_dvh * ((assign1840_e2137 * var_dvh_dn6) / var_vjc_t))) * var_pwq),)
    } else {
        (var_qhi, var_qhi_dn1, var_qhi_dn3, var_qhi_dn4, var_qhi_dn5, var_qhi_dn6,)
    }
};
        var_qhi = assign1840_e2147;
        var_qhi_dn1 = assign1840_e2147_d_n1;
        var_qhi_dn3 = assign1840_e2147_d_n3;
        var_qhi_dn4 = assign1840_e2147_d_n4;
        var_qhi_dn5 = assign1840_e2147_d_n5;
        var_qhi_dn6 = assign1840_e2147_d_n6;
        var_qhi_rv = 0.0;

        let (assign1850_e2170, assign1850_e2170_d_n1, assign1850_e2170_d_n3, assign1850_e2170_d_n4, assign1850_e2170_d_n5, assign1850_e2170_d_n6,) = {
    if (var_guard18 == 0.0) {
        let assign1850_e2154: f64 = (1.0 - p.p71);
        let assign1850_e2158: f64 = (var_vbici / var_vjc_t);
        let assign1850_e2159: f64 = (1.0 - assign1850_e2158);
        let assign1850_e2160: f64 = (assign1850_e2159).ln();
        let assign1850_e2161: f64 = (assign1850_e2154 * assign1850_e2160);
        let assign1850_e2162: f64 = (assign1850_e2161).exp();
        let assign1850_e2163: f64 = (1.0 - assign1850_e2162);
        let assign1850_e2164: f64 = (var_vjc_t * assign1850_e2163);
        let assign1850_e2167: f64 = (1.0 - p.p71);
        let assign1850_e2168: f64 = (assign1850_e2164 / assign1850_e2167);
        (assign1850_e2168, 0.0, (((var_vjc_t_dn3 * assign1850_e2163) + (var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(-((var_vbici * var_vjc_t_dn3) / (var_vjc_t * var_vjc_t)))) / assign1850_e2159)))))) / assign1850_e2167), ((var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(var_vbici_dn4 / var_vjc_t)) / assign1850_e2159))))) / assign1850_e2167), ((var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(var_vbici_dn5 / var_vjc_t)) / assign1850_e2159))))) / assign1850_e2167), 0.0,)
    } else {
        (var_qlo, var_qlo_dn1, var_qlo_dn3, var_qlo_dn4, var_qlo_dn5, var_qlo_dn6,)
    }
};
        var_qlo = assign1850_e2170;
        var_qlo_dn1 = assign1850_e2170_d_n1;
        var_qlo_dn3 = assign1850_e2170_d_n3;
        var_qlo_dn4 = assign1850_e2170_d_n4;
        var_qlo_dn5 = assign1850_e2170_d_n5;
        var_qlo_dn6 = assign1850_e2170_d_n6;
        var_qlo_rv = 0.0;

        let (assign1860_e2175, assign1860_e2175_d_n1, assign1860_e2175_d_n3, assign1860_e2175_d_n4, assign1860_e2175_d_n5, assign1860_e2175_d_n6,) = {
    if (var_guard18 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qhi, var_qhi_dn1, var_qhi_dn3, var_qhi_dn4, var_qhi_dn5, var_qhi_dn6,)
    }
};
        var_qhi = assign1860_e2175;
        var_qhi_dn1 = assign1860_e2175_d_n1;
        var_qhi_dn3 = assign1860_e2175_d_n3;
        var_qhi_dn4 = assign1860_e2175_d_n4;
        var_qhi_dn5 = assign1860_e2175_d_n5;
        var_qhi_dn6 = assign1860_e2175_d_n6;
        var_qhi_rv = 0.0;

        let assign1870_e2179: f64 = (var_qlo + var_qhi);
        let assign1870_e2180: f64 = (var_cjc_t * assign1870_e2179);
        var_qjci = assign1870_e2180;
        var_qjci_dn1 = (var_cjc_t * (var_qlo_dn1 + var_qhi_dn1));
        var_qjci_dn3 = ((var_cjc_t_dn3 * assign1870_e2179) + (var_cjc_t * (var_qlo_dn3 + var_qhi_dn3)));
        var_qjci_dn4 = (var_cjc_t * (var_qlo_dn4 + var_qhi_dn4));
        var_qjci_dn5 = (var_cjc_t * (var_qlo_dn5 + var_qhi_dn5));
        var_qjci_dn6 = (var_cjc_t * (var_qlo_dn6 + var_qhi_dn6));
        var_qjci_rv = 0.0;

        let assign1880_e2183: f64 = (p.p72 * var_qjci);
        var_qjci_1 = assign1880_e2183;
        var_qjci_1_dn1 = (p.p72 * var_qjci_dn1);
        var_qjci_1_dn3 = (p.p72 * var_qjci_dn3);
        var_qjci_1_dn4 = (p.p72 * var_qjci_dn4);
        var_qjci_1_dn5 = (p.p72 * var_qjci_dn5);
        var_qjci_1_dn6 = (p.p72 * var_qjci_dn6);
        var_qjci_1_rv = 0.0;

        let assign1890_e2190: f64 = if ((p.p68 != 0.0) && (p.p19 != 0.0)) { 1.0 } else { 0.0 };
        var_guard19 = assign1890_e2190;
        var_guard19_rv = 0.0;

        let (assign1900_e2204, assign1900_e2204_d_n3, assign1900_e2204_d_n4, assign1900_e2204_d_n5, assign1900_e2204_d_n6,) = {
    if (var_guard19 != 0.0) {
        let assign1900_e2194: f64 = (var_ttype * p.p68);
        let assign1900_e2196: f64 = (assign1900_e2194 * 3.141592653589793);
        let assign1900_e2198: f64 = (assign1900_e2196 / 180.0);
        let assign1900_e2200: f64 = (assign1900_e2198 * p.p19);
        let assign1900_e2202: f64 = (assign1900_e2200 * var_itzf);
        (assign1900_e2202, (assign1900_e2200 * var_itzf_dn3), (assign1900_e2200 * var_itzf_dn4), (assign1900_e2200 * var_itzf_dn5), (assign1900_e2200 * var_itzf_dn6),)
    } else {
        (var_qxf1, var_qxf1_dn3, var_qxf1_dn4, var_qxf1_dn5, var_qxf1_dn6,)
    }
};
        var_qxf1 = assign1900_e2204;
        var_qxf1_dn3 = assign1900_e2204_d_n3;
        var_qxf1_dn4 = assign1900_e2204_d_n4;
        var_qxf1_dn5 = assign1900_e2204_d_n5;
        var_qxf1_dn6 = assign1900_e2204_d_n6;
        var_qxf1_rv = 0.0;

        let (assign1910_e2209, assign1910_e2209_d_n3, assign1910_e2209_d_n4, assign1910_e2209_d_n5, assign1910_e2209_d_n6,) = {
    if (var_guard19 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qxf1, var_qxf1_dn3, var_qxf1_dn4, var_qxf1_dn5, var_qxf1_dn6,)
    }
};
        var_qxf1 = assign1910_e2209;
        var_qxf1_dn3 = assign1910_e2209_d_n3;
        var_qxf1_dn4 = assign1910_e2209_d_n4;
        var_qxf1_dn5 = assign1910_e2209_d_n5;
        var_qxf1_dn6 = assign1910_e2209_d_n6;
        var_qxf1_rv = 0.0;

        let assign1920_e2216: f64 = if ((p.p30 == 1.0) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };
        var_guard20 = assign1920_e2216;
        var_guard20_rv = 0.0;

        let assign1930_e2227: f64 = if (((p.p30 == 2.0) && (p.p33 > 0.0)) && (p.p35 > 0.0)) { 1.0 } else { 0.0 };
        var_guard21 = assign1930_e2227;
        var_guard21_rv = 0.0;

        *var_dv0_slot = var_dv0;
        *var_dv0_dn3_slot = var_dv0_dn3;
        *var_dv0_rv_slot = var_dv0_rv;
        *var_dvh_slot = var_dvh;
        *var_dvh_dn1_slot = var_dvh_dn1;
        *var_dvh_dn3_slot = var_dvh_dn3;
        *var_dvh_dn4_slot = var_dvh_dn4;
        *var_dvh_dn5_slot = var_dvh_dn5;
        *var_dvh_dn6_slot = var_dvh_dn6;
        *var_dvh_rv_slot = var_dvh_rv;
        *var_guard16_slot = var_guard16;
        *var_guard16_rv_slot = var_guard16_rv;
        *var_guard17_slot = var_guard17;
        *var_guard17_rv_slot = var_guard17_rv;
        *var_guard18_slot = var_guard18;
        *var_guard18_rv_slot = var_guard18_rv;
        *var_guard19_slot = var_guard19;
        *var_guard19_rv_slot = var_guard19_rv;
        *var_guard20_slot = var_guard20;
        *var_guard20_rv_slot = var_guard20_rv;
        *var_guard21_slot = var_guard21;
        *var_guard21_rv_slot = var_guard21_rv;
        *var_pwq_slot = var_pwq;
        *var_pwq_rv_slot = var_pwq_rv;
        *var_qhi_slot = var_qhi;
        *var_qhi_dn1_slot = var_qhi_dn1;
        *var_qhi_dn3_slot = var_qhi_dn3;
        *var_qhi_dn4_slot = var_qhi_dn4;
        *var_qhi_dn5_slot = var_qhi_dn5;
        *var_qhi_dn6_slot = var_qhi_dn6;
        *var_qhi_rv_slot = var_qhi_rv;
        *var_qjci_slot = var_qjci;
        *var_qjci_1_slot = var_qjci_1;
        *var_qjci_1_dn1_slot = var_qjci_1_dn1;
        *var_qjci_1_dn3_slot = var_qjci_1_dn3;
        *var_qjci_1_dn4_slot = var_qjci_1_dn4;
        *var_qjci_1_dn5_slot = var_qjci_1_dn5;
        *var_qjci_1_dn6_slot = var_qjci_1_dn6;
        *var_qjci_1_rv_slot = var_qjci_1_rv;
        *var_qjci_dn1_slot = var_qjci_dn1;
        *var_qjci_dn3_slot = var_qjci_dn3;
        *var_qjci_dn4_slot = var_qjci_dn4;
        *var_qjci_dn5_slot = var_qjci_dn5;
        *var_qjci_dn6_slot = var_qjci_dn6;
        *var_qjci_rv_slot = var_qjci_rv;
        *var_qjcx_slot = var_qjcx;
        *var_qjcx_1_slot = var_qjcx_1;
        *var_qjcx_1_dn1_slot = var_qjcx_1_dn1;
        *var_qjcx_1_dn3_slot = var_qjcx_1_dn3;
        *var_qjcx_1_dn4_slot = var_qjcx_1_dn4;
        *var_qjcx_1_dn5_slot = var_qjcx_1_dn5;
        *var_qjcx_1_dn6_slot = var_qjcx_1_dn6;
        *var_qjcx_1_rv_slot = var_qjcx_1_rv;
        *var_qjcx_dn1_slot = var_qjcx_dn1;
        *var_qjcx_dn3_slot = var_qjcx_dn3;
        *var_qjcx_dn4_slot = var_qjcx_dn4;
        *var_qjcx_dn5_slot = var_qjcx_dn5;
        *var_qjcx_dn6_slot = var_qjcx_dn6;
        *var_qjcx_rv_slot = var_qjcx_rv;
        *var_qje_slot = var_qje;
        *var_qje_dn1_slot = var_qje_dn1;
        *var_qje_dn3_slot = var_qje_dn3;
        *var_qje_dn4_slot = var_qje_dn4;
        *var_qje_dn5_slot = var_qje_dn5;
        *var_qje_dn6_slot = var_qje_dn6;
        *var_qje_rv_slot = var_qje_rv;
        *var_qjs_slot = var_qjs;
        *var_qjs_dn2_slot = var_qjs_dn2;
        *var_qjs_dn3_slot = var_qjs_dn3;
        *var_qjs_dn4_slot = var_qjs_dn4;
        *var_qjs_rv_slot = var_qjs_rv;
        *var_qlo_slot = var_qlo;
        *var_qlo_dn1_slot = var_qlo_dn1;
        *var_qlo_dn3_slot = var_qlo_dn3;
        *var_qlo_dn4_slot = var_qlo_dn4;
        *var_qlo_dn5_slot = var_qlo_dn5;
        *var_qlo_dn6_slot = var_qlo_dn6;
        *var_qlo_rv_slot = var_qlo_rv;
        *var_qxf1_slot = var_qxf1;
        *var_qxf1_dn3_slot = var_qxf1_dn3;
        *var_qxf1_dn4_slot = var_qxf1_dn4;
        *var_qxf1_dn5_slot = var_qxf1_dn5;
        *var_qxf1_dn6_slot = var_qxf1_dn6;
        *var_qxf1_rv_slot = var_qxf1_rv;
    }

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
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
        var_bf_t: f64,
        var_bf_t_dn3: f64,
        var_bf_t_dn4: f64,
        var_bf_t_dn5: f64,
        var_guard13: f64,
        var_guard20: f64,
        var_guard21: f64,
        var_guard22: f64,
        var_guard23: f64,
        var_guard24: f64,
        var_guard25: f64,
        var_ibc: f64,
        var_ibc_dn3: f64,
        var_ibc_dn4: f64,
        var_ibc_dn5: f64,
        var_ibc_dn6: f64,
        var_ibe: f64,
        var_ibe_dn3: f64,
        var_ibe_dn4: f64,
        var_ibe_dn5: f64,
        var_ibe_dn6: f64,
        var_ifwd: f64,
        var_ifwd_dn3: f64,
        var_ifwd_dn4: f64,
        var_ifwd_dn5: f64,
        var_ifwd_dn6: f64,
        var_rb: f64,
        var_rb_dn1: f64,
        var_rb_dn3: f64,
        var_rb_dn5: f64,
        var_rb_dn8: f64,
        var_rc: f64,
        var_rc_dn3: f64,
        var_re: f64,
        var_re_dn2: f64,
        var_re_dn3: f64,
        var_re_dn6: f64,
        var_tff: f64,
        var_tff_dn1: f64,
        var_tff_dn2: f64,
        var_ttype: f64,
        var_weff: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let eq2_e98: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, (nv9 - 0.0));
        let eq2_e99: f64 = (p.p83 * eq2_e98);
        let eq2_value: f64 = eq2_e99;
        stamper.stamp_current_node1_local(
            Some(9),
            None,
            multiplicity * (eq2_value),
            9,
            multiplicity * ((p.p83 * ddt_scale)),
        );
        let (eq3_e108, eq3_e108_d_n1, eq3_e108_d_n2, eq3_e108_d_n3, eq3_e108_d_n4, eq3_e108_d_n5, eq3_e108_d_n6,) = {
    if (var_guard13 != 0.0) {
        let eq3_e103: f64 = (var_ifwd / var_bf_t);
        let __rspice_inv_cse_0: f64 = 1.0 / (var_bf_t * var_bf_t);
        let eq3_e103_d_n3: f64 = (((var_ifwd_dn3 * var_bf_t) - (var_ifwd * var_bf_t_dn3)) * __rspice_inv_cse_0);
        let eq3_e103_d_n4: f64 = (((var_ifwd_dn4 * var_bf_t) - (var_ifwd * var_bf_t_dn4)) * __rspice_inv_cse_0);
        let eq3_e103_d_n5: f64 = (((var_ifwd_dn5 * var_bf_t) - (var_ifwd * var_bf_t_dn5)) * __rspice_inv_cse_0);
        let eq3_e103_d_n6: f64 = (var_ifwd_dn6 / var_bf_t);
        let eq3_e104: f64 = (-eq3_e103);
        let eq3_e106: f64 = (eq3_e104 * var_tff);
        let eq3_e106_d_n1: f64 = (eq3_e104 * var_tff_dn1);
        let eq3_e106_d_n2: f64 = (eq3_e104 * var_tff_dn2);
        let eq3_e106_d_n3: f64 = ((-eq3_e103_d_n3) * var_tff);
        let eq3_e106_d_n4: f64 = ((-eq3_e103_d_n4) * var_tff);
        let eq3_e106_d_n5: f64 = ((-eq3_e103_d_n5) * var_tff);
        let eq3_e106_d_n6: f64 = ((-eq3_e103_d_n6) * var_tff);
        (eq3_e106, eq3_e106_d_n1, eq3_e106_d_n2, eq3_e106_d_n3, eq3_e106_d_n4, eq3_e106_d_n5, eq3_e106_d_n6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e108;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(8),
            None,
            multiplicity * (eq3_value),
            [1, 2, 3, 4, 5, 6],
            [multiplicity * (eq3_e108_d_n1), multiplicity * (eq3_e108_d_n2), multiplicity * (eq3_e108_d_n3), multiplicity * (eq3_e108_d_n4), multiplicity * (eq3_e108_d_n5), multiplicity * (eq3_e108_d_n6)],
            [],
            [],
            1.0,
        );
        let (eq5_e121, eq5_e121_d_n1, eq5_e121_d_n2, eq5_e121_d_n8,) = {
    if (var_guard13 != 0.0) {
        let eq5_e118: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, (nv8 - 0.0));
        let eq5_e119: f64 = (var_tff * eq5_e118);
        let eq5_e119_d_n1: f64 = (var_tff_dn1 * eq5_e118);
        let eq5_e119_d_n2: f64 = (var_tff_dn2 * eq5_e118);
        (eq5_e119, eq5_e119_d_n1, eq5_e119_d_n2, (var_tff * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e121;
        stamper.stamp_current_node3_local(
            Some(8),
            None,
            multiplicity * (eq5_value),
            1,
            multiplicity * (eq5_e121_d_n1),
            2,
            multiplicity * (eq5_e121_d_n2),
            8,
            multiplicity * (eq5_e121_d_n8),
        );
        let (eq7_e141, eq7_e141_d_n0, eq7_e141_d_n1, eq7_e141_d_n2, eq7_e141_d_n3, eq7_e141_d_n4, eq7_e141_d_n5, eq7_e141_d_n6,) = {
    if (var_guard20 != 0.0) {
        let eq7_e129: f64 = (-1.0);
        let eq7_e132: f64 = (var_ibe * (nv1 - nv2));
        let eq7_e132_d_n3: f64 = (var_ibe_dn3 * (nv1 - nv2));
        let eq7_e132_d_n4: f64 = (var_ibe_dn4 * (nv1 - nv2));
        let eq7_e132_d_n5: f64 = (var_ibe_dn5 * (nv1 - nv2));
        let eq7_e132_d_n6: f64 = (var_ibe_dn6 * (nv1 - nv2));
        let eq7_e133: f64 = (eq7_e132).abs();
        let eq7_e133_d_n1: f64 = if eq7_e132 >= 0.0 { var_ibe } else { (-var_ibe) };
        let eq7_e133_d_n2: f64 = if eq7_e132 >= 0.0 { (-var_ibe) } else { (-(-var_ibe)) };
        let eq7_e133_d_n3: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_n3 } else { (-eq7_e132_d_n3) };
        let eq7_e133_d_n4: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_n4 } else { (-eq7_e132_d_n4) };
        let eq7_e133_d_n5: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_n5 } else { (-eq7_e132_d_n5) };
        let eq7_e133_d_n6: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_n6 } else { (-eq7_e132_d_n6) };
        let eq7_e134: f64 = (eq7_e129 * eq7_e133);
        let eq7_e134_d_n1: f64 = (eq7_e129 * eq7_e133_d_n1);
        let eq7_e134_d_n2: f64 = (eq7_e129 * eq7_e133_d_n2);
        let eq7_e134_d_n3: f64 = (eq7_e129 * eq7_e133_d_n3);
        let eq7_e134_d_n4: f64 = (eq7_e129 * eq7_e133_d_n4);
        let eq7_e134_d_n5: f64 = (eq7_e129 * eq7_e133_d_n5);
        let eq7_e134_d_n6: f64 = (eq7_e129 * eq7_e133_d_n6);
        let eq7_e137: f64 = (var_ibc * (nv1 - nv0));
        let eq7_e137_d_n3: f64 = (var_ibc_dn3 * (nv1 - nv0));
        let eq7_e137_d_n4: f64 = (var_ibc_dn4 * (nv1 - nv0));
        let eq7_e137_d_n5: f64 = (var_ibc_dn5 * (nv1 - nv0));
        let eq7_e137_d_n6: f64 = (var_ibc_dn6 * (nv1 - nv0));
        let eq7_e138: f64 = (eq7_e137).abs();
        let eq7_e138_d_n0: f64 = if eq7_e137 >= 0.0 { (-var_ibc) } else { (-(-var_ibc)) };
        let eq7_e138_d_n1: f64 = if eq7_e137 >= 0.0 { var_ibc } else { (-var_ibc) };
        let eq7_e138_d_n3: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_n3 } else { (-eq7_e137_d_n3) };
        let eq7_e138_d_n4: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_n4 } else { (-eq7_e137_d_n4) };
        let eq7_e138_d_n5: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_n5 } else { (-eq7_e137_d_n5) };
        let eq7_e138_d_n6: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_n6 } else { (-eq7_e137_d_n6) };
        let eq7_e139: f64 = (eq7_e134 - eq7_e138);
        let eq7_e139_d_n1: f64 = (eq7_e134_d_n1 - eq7_e138_d_n1);
        let eq7_e139_d_n3: f64 = (eq7_e134_d_n3 - eq7_e138_d_n3);
        let eq7_e139_d_n4: f64 = (eq7_e134_d_n4 - eq7_e138_d_n4);
        let eq7_e139_d_n5: f64 = (eq7_e134_d_n5 - eq7_e138_d_n5);
        let eq7_e139_d_n6: f64 = (eq7_e134_d_n6 - eq7_e138_d_n6);
        (eq7_e139, (-eq7_e138_d_n0), eq7_e139_d_n1, eq7_e134_d_n2, eq7_e139_d_n3, eq7_e139_d_n4, eq7_e139_d_n5, eq7_e139_d_n6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e141;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(3),
            None,
            multiplicity * (eq7_value),
            [0, 1, 2, 3, 4, 5, 6],
            [multiplicity * (eq7_e141_d_n0), multiplicity * (eq7_e141_d_n1), multiplicity * (eq7_e141_d_n2), multiplicity * (eq7_e141_d_n3), multiplicity * (eq7_e141_d_n4), multiplicity * (eq7_e141_d_n5), multiplicity * (eq7_e141_d_n6)],
            [],
            [],
            1.0,
        );
        let (eq8_e147, eq8_e147_d_n3,) = {
    if (var_guard20 != 0.0) {
        let __rspice_inv_cse_1: f64 = 1.0 / p.p33;
        let eq8_e145: f64 = ((nv3 - 0.0) * __rspice_inv_cse_1);
        let eq8_e145_d_n3: f64 = (1.0 * __rspice_inv_cse_1);
        (eq8_e145, eq8_e145_d_n3,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e147;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (eq8_value),
            3,
            multiplicity * (eq8_e147_d_n3),
        );
        let (eq9_e154, eq9_e154_d_n3,) = {
    if (var_guard20 != 0.0) {
        let eq9_e151: f64 = ((nv3 - 0.0) * p.p34);
        let eq9_e152: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq9_e151);
        (eq9_e152, (p.p34 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e154;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (eq9_value),
            3,
            multiplicity * (eq9_e154_d_n3),
        );
        let (eq11_e176, eq11_e176_d_n0, eq11_e176_d_n1, eq11_e176_d_n2, eq11_e176_d_n3, eq11_e176_d_n4, eq11_e176_d_n5, eq11_e176_d_n6,) = {
    if ((var_guard20 == 0.0) && (var_guard21 != 0.0)) {
        let eq11_e164: f64 = (-1.0);
        let eq11_e167: f64 = (var_ibe * (nv1 - nv2));
        let eq11_e167_d_n3: f64 = (var_ibe_dn3 * (nv1 - nv2));
        let eq11_e167_d_n4: f64 = (var_ibe_dn4 * (nv1 - nv2));
        let eq11_e167_d_n5: f64 = (var_ibe_dn5 * (nv1 - nv2));
        let eq11_e167_d_n6: f64 = (var_ibe_dn6 * (nv1 - nv2));
        let eq11_e168: f64 = (eq11_e167).abs();
        let eq11_e168_d_n1: f64 = if eq11_e167 >= 0.0 { var_ibe } else { (-var_ibe) };
        let eq11_e168_d_n2: f64 = if eq11_e167 >= 0.0 { (-var_ibe) } else { (-(-var_ibe)) };
        let eq11_e168_d_n3: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_n3 } else { (-eq11_e167_d_n3) };
        let eq11_e168_d_n4: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_n4 } else { (-eq11_e167_d_n4) };
        let eq11_e168_d_n5: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_n5 } else { (-eq11_e167_d_n5) };
        let eq11_e168_d_n6: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_n6 } else { (-eq11_e167_d_n6) };
        let eq11_e169: f64 = (eq11_e164 * eq11_e168);
        let eq11_e169_d_n1: f64 = (eq11_e164 * eq11_e168_d_n1);
        let eq11_e169_d_n2: f64 = (eq11_e164 * eq11_e168_d_n2);
        let eq11_e169_d_n3: f64 = (eq11_e164 * eq11_e168_d_n3);
        let eq11_e169_d_n4: f64 = (eq11_e164 * eq11_e168_d_n4);
        let eq11_e169_d_n5: f64 = (eq11_e164 * eq11_e168_d_n5);
        let eq11_e169_d_n6: f64 = (eq11_e164 * eq11_e168_d_n6);
        let eq11_e172: f64 = (var_ibc * (nv1 - nv0));
        let eq11_e172_d_n3: f64 = (var_ibc_dn3 * (nv1 - nv0));
        let eq11_e172_d_n4: f64 = (var_ibc_dn4 * (nv1 - nv0));
        let eq11_e172_d_n5: f64 = (var_ibc_dn5 * (nv1 - nv0));
        let eq11_e172_d_n6: f64 = (var_ibc_dn6 * (nv1 - nv0));
        let eq11_e173: f64 = (eq11_e172).abs();
        let eq11_e173_d_n0: f64 = if eq11_e172 >= 0.0 { (-var_ibc) } else { (-(-var_ibc)) };
        let eq11_e173_d_n1: f64 = if eq11_e172 >= 0.0 { var_ibc } else { (-var_ibc) };
        let eq11_e173_d_n3: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_n3 } else { (-eq11_e172_d_n3) };
        let eq11_e173_d_n4: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_n4 } else { (-eq11_e172_d_n4) };
        let eq11_e173_d_n5: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_n5 } else { (-eq11_e172_d_n5) };
        let eq11_e173_d_n6: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_n6 } else { (-eq11_e172_d_n6) };
        let eq11_e174: f64 = (eq11_e169 - eq11_e173);
        let eq11_e174_d_n1: f64 = (eq11_e169_d_n1 - eq11_e173_d_n1);
        let eq11_e174_d_n3: f64 = (eq11_e169_d_n3 - eq11_e173_d_n3);
        let eq11_e174_d_n4: f64 = (eq11_e169_d_n4 - eq11_e173_d_n4);
        let eq11_e174_d_n5: f64 = (eq11_e169_d_n5 - eq11_e173_d_n5);
        let eq11_e174_d_n6: f64 = (eq11_e169_d_n6 - eq11_e173_d_n6);
        (eq11_e174, (-eq11_e173_d_n0), eq11_e174_d_n1, eq11_e169_d_n2, eq11_e174_d_n3, eq11_e174_d_n4, eq11_e174_d_n5, eq11_e174_d_n6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e176;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(3),
            None,
            multiplicity * (eq11_value),
            [0, 1, 2, 3, 4, 5, 6],
            [multiplicity * (eq11_e176_d_n0), multiplicity * (eq11_e176_d_n1), multiplicity * (eq11_e176_d_n2), multiplicity * (eq11_e176_d_n3), multiplicity * (eq11_e176_d_n4), multiplicity * (eq11_e176_d_n5), multiplicity * (eq11_e176_d_n6)],
            [],
            [],
            1.0,
        );
        let (eq12_e185, eq12_e185_d_n3, eq12_e185_d_n7,) = {
    if ((var_guard20 == 0.0) && (var_guard21 != 0.0)) {
        let __rspice_inv_cse_2: f64 = 1.0 / p.p33;
        let eq12_e183: f64 = ((nv3 - nv7) * __rspice_inv_cse_2);
        let eq12_e183_d_n3: f64 = (1.0 * __rspice_inv_cse_2);
        let eq12_e183_d_n7: f64 = ((-1.0) * __rspice_inv_cse_2);
        (eq12_e183, eq12_e183_d_n3, eq12_e183_d_n7,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq12_value: f64 = eq12_e185;
        stamper.stamp_current_node2_local(
            Some(3),
            Some(7),
            multiplicity * (eq12_value),
            3,
            multiplicity * (eq12_e185_d_n3),
            7,
            multiplicity * (eq12_e185_d_n7),
        );
        let (eq13_e195, eq13_e195_d_n3,) = {
    if ((var_guard20 == 0.0) && (var_guard21 != 0.0)) {
        let eq13_e192: f64 = (p.p34 * (nv3 - 0.0));
        let eq13_e193: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq13_e192);
        (eq13_e193, (p.p34 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq13_value: f64 = eq13_e195;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (eq13_value),
            3,
            multiplicity * (eq13_e195_d_n3),
        );
        let (eq14_e204, eq14_e204_d_n7,) = {
    if ((var_guard20 == 0.0) && (var_guard21 != 0.0)) {
        let __rspice_inv_cse_3: f64 = 1.0 / p.p35;
        let eq14_e202: f64 = ((nv7 - 0.0) * __rspice_inv_cse_3);
        let eq14_e202_d_n7: f64 = (1.0 * __rspice_inv_cse_3);
        (eq14_e202, eq14_e202_d_n7,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq14_value: f64 = eq14_e204;
        stamper.stamp_current_node1_local(
            Some(7),
            None,
            multiplicity * (eq14_value),
            7,
            multiplicity * (eq14_e204_d_n7),
        );
        let (eq15_e214, eq15_e214_d_n7,) = {
    if ((var_guard20 == 0.0) && (var_guard21 != 0.0)) {
        let eq15_e211: f64 = (p.p36 * (nv7 - 0.0));
        let eq15_e212: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq15_e211);
        (eq15_e212, (p.p36 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e214;
        stamper.stamp_current_node1_local(
            Some(7),
            None,
            multiplicity * (eq15_value),
            7,
            multiplicity * (eq15_e214_d_n7),
        );
        let (eq16_e235, eq16_e235_d_n0, eq16_e235_d_n1, eq16_e235_d_n2, eq16_e235_d_n3, eq16_e235_d_n4, eq16_e235_d_n5, eq16_e235_d_n6,) = {
    if (((var_guard20 == 0.0) && (var_guard21 == 0.0)) && (var_guard22 != 0.0)) {
        let eq16_e223: f64 = (-1.0);
        let eq16_e226: f64 = (var_ibe * (nv1 - nv2));
        let eq16_e226_d_n3: f64 = (var_ibe_dn3 * (nv1 - nv2));
        let eq16_e226_d_n4: f64 = (var_ibe_dn4 * (nv1 - nv2));
        let eq16_e226_d_n5: f64 = (var_ibe_dn5 * (nv1 - nv2));
        let eq16_e226_d_n6: f64 = (var_ibe_dn6 * (nv1 - nv2));
        let eq16_e227: f64 = (eq16_e226).abs();
        let eq16_e227_d_n1: f64 = if eq16_e226 >= 0.0 { var_ibe } else { (-var_ibe) };
        let eq16_e227_d_n2: f64 = if eq16_e226 >= 0.0 { (-var_ibe) } else { (-(-var_ibe)) };
        let eq16_e227_d_n3: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_n3 } else { (-eq16_e226_d_n3) };
        let eq16_e227_d_n4: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_n4 } else { (-eq16_e226_d_n4) };
        let eq16_e227_d_n5: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_n5 } else { (-eq16_e226_d_n5) };
        let eq16_e227_d_n6: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_n6 } else { (-eq16_e226_d_n6) };
        let eq16_e228: f64 = (eq16_e223 * eq16_e227);
        let eq16_e228_d_n1: f64 = (eq16_e223 * eq16_e227_d_n1);
        let eq16_e228_d_n2: f64 = (eq16_e223 * eq16_e227_d_n2);
        let eq16_e228_d_n3: f64 = (eq16_e223 * eq16_e227_d_n3);
        let eq16_e228_d_n4: f64 = (eq16_e223 * eq16_e227_d_n4);
        let eq16_e228_d_n5: f64 = (eq16_e223 * eq16_e227_d_n5);
        let eq16_e228_d_n6: f64 = (eq16_e223 * eq16_e227_d_n6);
        let eq16_e231: f64 = (var_ibc * (nv1 - nv0));
        let eq16_e231_d_n3: f64 = (var_ibc_dn3 * (nv1 - nv0));
        let eq16_e231_d_n4: f64 = (var_ibc_dn4 * (nv1 - nv0));
        let eq16_e231_d_n5: f64 = (var_ibc_dn5 * (nv1 - nv0));
        let eq16_e231_d_n6: f64 = (var_ibc_dn6 * (nv1 - nv0));
        let eq16_e232: f64 = (eq16_e231).abs();
        let eq16_e232_d_n0: f64 = if eq16_e231 >= 0.0 { (-var_ibc) } else { (-(-var_ibc)) };
        let eq16_e232_d_n1: f64 = if eq16_e231 >= 0.0 { var_ibc } else { (-var_ibc) };
        let eq16_e232_d_n3: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_n3 } else { (-eq16_e231_d_n3) };
        let eq16_e232_d_n4: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_n4 } else { (-eq16_e231_d_n4) };
        let eq16_e232_d_n5: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_n5 } else { (-eq16_e231_d_n5) };
        let eq16_e232_d_n6: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_n6 } else { (-eq16_e231_d_n6) };
        let eq16_e233: f64 = (eq16_e228 - eq16_e232);
        let eq16_e233_d_n1: f64 = (eq16_e228_d_n1 - eq16_e232_d_n1);
        let eq16_e233_d_n3: f64 = (eq16_e228_d_n3 - eq16_e232_d_n3);
        let eq16_e233_d_n4: f64 = (eq16_e228_d_n4 - eq16_e232_d_n4);
        let eq16_e233_d_n5: f64 = (eq16_e228_d_n5 - eq16_e232_d_n5);
        let eq16_e233_d_n6: f64 = (eq16_e228_d_n6 - eq16_e232_d_n6);
        (eq16_e233, (-eq16_e232_d_n0), eq16_e233_d_n1, eq16_e228_d_n2, eq16_e233_d_n3, eq16_e233_d_n4, eq16_e233_d_n5, eq16_e233_d_n6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq16_value: f64 = eq16_e235;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(3),
            None,
            multiplicity * (eq16_value),
            [0, 1, 2, 3, 4, 5, 6],
            [multiplicity * (eq16_e235_d_n0), multiplicity * (eq16_e235_d_n1), multiplicity * (eq16_e235_d_n2), multiplicity * (eq16_e235_d_n3), multiplicity * (eq16_e235_d_n4), multiplicity * (eq16_e235_d_n5), multiplicity * (eq16_e235_d_n6)],
            [],
            [],
            1.0,
        );
        let (eq23_e297, eq23_e297_d_n1, eq23_e297_d_n3, eq23_e297_d_n5, eq23_e297_d_n8,) = {
    if (var_guard23 != 0.0) {
        let __rspice_inv_cse_4: f64 = 1.0 / var_weff;
        let eq23_e287: f64 = (var_rb * __rspice_inv_cse_4);
        let eq23_e287_d_n1: f64 = (var_rb_dn1 * __rspice_inv_cse_4);
        let eq23_e287_d_n3: f64 = (var_rb_dn3 * __rspice_inv_cse_4);
        let eq23_e287_d_n5: f64 = (var_rb_dn5 * __rspice_inv_cse_4);
        let eq23_e287_d_n8: f64 = (var_rb_dn8 * __rspice_inv_cse_4);
        let (eq23_e294, eq23_e294_d_n1, eq23_e294_d_n3, eq23_e294_d_n5, eq23_e294_d_n8,) = {
            if (eq23_e287 > p.p46) {
                let __rspice_inv_cse_5: f64 = 1.0 / var_weff;
                let eq23_e292: f64 = (var_rb * __rspice_inv_cse_5);
                let eq23_e292_d_n1: f64 = (var_rb_dn1 * __rspice_inv_cse_5);
                let eq23_e292_d_n3: f64 = (var_rb_dn3 * __rspice_inv_cse_5);
                let eq23_e292_d_n5: f64 = (var_rb_dn5 * __rspice_inv_cse_5);
                let eq23_e292_d_n8: f64 = (var_rb_dn8 * __rspice_inv_cse_5);
                (eq23_e292, eq23_e292_d_n1, eq23_e292_d_n3, eq23_e292_d_n5, eq23_e292_d_n8,)
            } else {
                (p.p46, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let eq23_e295: f64 = ((nv1 - nv5) / eq23_e294);
        let eq23_e295_d_n1: f64 = ((eq23_e294 - ((nv1 - nv5) * eq23_e294_d_n1)) / (eq23_e294 * eq23_e294));
        let eq23_e295_d_n3: f64 = (-(((nv1 - nv5) * eq23_e294_d_n3) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_n5: f64 = (((-eq23_e294) - ((nv1 - nv5) * eq23_e294_d_n5)) / (eq23_e294 * eq23_e294));
        let eq23_e295_d_n8: f64 = (-(((nv1 - nv5) * eq23_e294_d_n8) / (eq23_e294 * eq23_e294)));
        (eq23_e295, eq23_e295_d_n1, eq23_e295_d_n3, eq23_e295_d_n5, eq23_e295_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq23_value: f64 = eq23_e297;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(1),
            Some(5),
            multiplicity * (eq23_value),
            [1, 3, 5, 8],
            [multiplicity * (eq23_e297_d_n1), multiplicity * (eq23_e297_d_n3), multiplicity * (eq23_e297_d_n5), multiplicity * (eq23_e297_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq26_e323, eq26_e323_d_n2, eq26_e323_d_n3, eq26_e323_d_n6,) = {
    if (var_guard24 != 0.0) {
        let __rspice_inv_cse_6: f64 = 1.0 / var_weff;
        let eq26_e313: f64 = (var_re * __rspice_inv_cse_6);
        let eq26_e313_d_n2: f64 = (var_re_dn2 * __rspice_inv_cse_6);
        let eq26_e313_d_n3: f64 = (var_re_dn3 * __rspice_inv_cse_6);
        let eq26_e313_d_n6: f64 = (var_re_dn6 * __rspice_inv_cse_6);
        let (eq26_e320, eq26_e320_d_n2, eq26_e320_d_n3, eq26_e320_d_n6,) = {
            if (eq26_e313 > p.p46) {
                let __rspice_inv_cse_7: f64 = 1.0 / var_weff;
                let eq26_e318: f64 = (var_re * __rspice_inv_cse_7);
                let eq26_e318_d_n2: f64 = (var_re_dn2 * __rspice_inv_cse_7);
                let eq26_e318_d_n3: f64 = (var_re_dn3 * __rspice_inv_cse_7);
                let eq26_e318_d_n6: f64 = (var_re_dn6 * __rspice_inv_cse_7);
                (eq26_e318, eq26_e318_d_n2, eq26_e318_d_n3, eq26_e318_d_n6,)
            } else {
                (p.p46, 0.0, 0.0, 0.0,)
            }
        };
        let eq26_e321: f64 = ((nv2 - nv6) / eq26_e320);
        let eq26_e321_d_n2: f64 = ((eq26_e320 - ((nv2 - nv6) * eq26_e320_d_n2)) / (eq26_e320 * eq26_e320));
        let eq26_e321_d_n3: f64 = (-(((nv2 - nv6) * eq26_e320_d_n3) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_n6: f64 = (((-eq26_e320) - ((nv2 - nv6) * eq26_e320_d_n6)) / (eq26_e320 * eq26_e320));
        (eq26_e321, eq26_e321_d_n2, eq26_e321_d_n3, eq26_e321_d_n6,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e323;
        stamper.stamp_current_node3_local(
            Some(2),
            Some(6),
            multiplicity * (eq26_value),
            2,
            multiplicity * (eq26_e323_d_n2),
            3,
            multiplicity * (eq26_e323_d_n3),
            6,
            multiplicity * (eq26_e323_d_n6),
        );
        let (eq29_e349, eq29_e349_d_n0, eq29_e349_d_n3, eq29_e349_d_n4,) = {
    if (var_guard25 != 0.0) {
        let __rspice_inv_cse_8: f64 = 1.0 / var_weff;
        let eq29_e339: f64 = (var_rc * __rspice_inv_cse_8);
        let eq29_e339_d_n3: f64 = (var_rc_dn3 * __rspice_inv_cse_8);
        let (eq29_e346, eq29_e346_d_n3,) = {
            if (eq29_e339 > p.p46) {
                let __rspice_inv_cse_9: f64 = 1.0 / var_weff;
                let eq29_e344: f64 = (var_rc * __rspice_inv_cse_9);
                let eq29_e344_d_n3: f64 = (var_rc_dn3 * __rspice_inv_cse_9);
                (eq29_e344, eq29_e344_d_n3,)
            } else {
                (p.p46, 0.0,)
            }
        };
        let __rspice_inv_cse_10: f64 = 1.0 / eq29_e346;
        let eq29_e347: f64 = ((nv0 - nv4) * __rspice_inv_cse_10);
        let eq29_e347_d_n0: f64 = (1.0 * __rspice_inv_cse_10);
        let eq29_e347_d_n3: f64 = (-(((nv0 - nv4) * eq29_e346_d_n3) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_n4: f64 = (-1.0 / eq29_e346);
        (eq29_e347, eq29_e347_d_n0, eq29_e347_d_n3, eq29_e347_d_n4,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq29_value: f64 = eq29_e349;
        stamper.stamp_current_node3_local(
            Some(0),
            Some(4),
            multiplicity * (eq29_value),
            0,
            multiplicity * (eq29_e349_d_n0),
            3,
            multiplicity * (eq29_e349_d_n3),
            4,
            multiplicity * (eq29_e349_d_n4),
        );
        let eq32_e363: f64 = (var_ttype * var_ibe);
        let eq32_e363_d_n3: f64 = (var_ttype * var_ibe_dn3);
        let eq32_e363_d_n4: f64 = (var_ttype * var_ibe_dn4);
        let eq32_e363_d_n5: f64 = (var_ttype * var_ibe_dn5);
        let eq32_e363_d_n6: f64 = (var_ttype * var_ibe_dn6);
        let eq32_e365: f64 = (eq32_e363 * var_weff);
        let eq32_e365_d_n3: f64 = (eq32_e363_d_n3 * var_weff);
        let eq32_e365_d_n4: f64 = (eq32_e363_d_n4 * var_weff);
        let eq32_e365_d_n5: f64 = (eq32_e363_d_n5 * var_weff);
        let eq32_e365_d_n6: f64 = (eq32_e363_d_n6 * var_weff);
        let eq32_value: f64 = eq32_e365;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq32_value),
            [3, 4, 5, 6],
            [multiplicity * (eq32_e365_d_n3), multiplicity * (eq32_e365_d_n4), multiplicity * (eq32_e365_d_n5), multiplicity * (eq32_e365_d_n6)],
            [],
            [],
            1.0,
        );
        let eq33_e368: f64 = (var_ttype * var_ibc);
        let eq33_e368_d_n3: f64 = (var_ttype * var_ibc_dn3);
        let eq33_e368_d_n4: f64 = (var_ttype * var_ibc_dn4);
        let eq33_e368_d_n5: f64 = (var_ttype * var_ibc_dn5);
        let eq33_e368_d_n6: f64 = (var_ttype * var_ibc_dn6);
        let eq33_e370: f64 = (eq33_e368 * var_weff);
        let eq33_e370_d_n3: f64 = (eq33_e368_d_n3 * var_weff);
        let eq33_e370_d_n4: f64 = (eq33_e368_d_n4 * var_weff);
        let eq33_e370_d_n5: f64 = (eq33_e368_d_n5 * var_weff);
        let eq33_e370_d_n6: f64 = (eq33_e368_d_n6 * var_weff);
        let eq33_value: f64 = eq33_e370;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * (eq33_value),
            [3, 4, 5, 6],
            [multiplicity * (eq33_e370_d_n3), multiplicity * (eq33_e370_d_n4), multiplicity * (eq33_e370_d_n5), multiplicity * (eq33_e370_d_n6)],
            [],
            [],
            1.0,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        stamper: &mut GeneratedStamper<'_>,
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
        var_itr: f64,
        var_itr_dn3: f64,
        var_itr_dn4: f64,
        var_itr_dn5: f64,
        var_itr_dn6: f64,
        var_itzf_f: f64,
        var_itzf_f_dn3: f64,
        var_itzf_f_dn4: f64,
        var_itzf_f_dn5: f64,
        var_itzf_f_dn6: f64,
        var_itzf_f_dn9: f64,
        var_qdc: f64,
        var_qdc_dn3: f64,
        var_qdc_dn4: f64,
        var_qdc_dn5: f64,
        var_qdc_dn6: f64,
        var_qde: f64,
        var_qde_dn1: f64,
        var_qde_dn2: f64,
        var_qde_dn3: f64,
        var_qde_dn4: f64,
        var_qde_dn5: f64,
        var_qde_dn6: f64,
        var_qjci_1: f64,
        var_qjci_1_dn1: f64,
        var_qjci_1_dn3: f64,
        var_qjci_1_dn4: f64,
        var_qjci_1_dn5: f64,
        var_qjci_1_dn6: f64,
        var_qjcx_1: f64,
        var_qjcx_1_dn1: f64,
        var_qjcx_1_dn3: f64,
        var_qjcx_1_dn4: f64,
        var_qjcx_1_dn5: f64,
        var_qjcx_1_dn6: f64,
        var_qje: f64,
        var_qje_dn1: f64,
        var_qje_dn3: f64,
        var_qje_dn4: f64,
        var_qje_dn5: f64,
        var_qje_dn6: f64,
        var_qjs: f64,
        var_qjs_dn2: f64,
        var_qjs_dn3: f64,
        var_qjs_dn4: f64,
        var_qxf1: f64,
        var_qxf1_dn3: f64,
        var_qxf1_dn4: f64,
        var_qxf1_dn5: f64,
        var_qxf1_dn6: f64,
        var_ttype: f64,
        var_weff: f64,
    ) {
        let eq34_e373: f64 = (-var_itr);
        let eq34_e375: f64 = (eq34_e373 * var_weff);
        let eq34_e375_d_n3: f64 = ((-var_itr_dn3) * var_weff);
        let eq34_e375_d_n4: f64 = ((-var_itr_dn4) * var_weff);
        let eq34_e375_d_n5: f64 = ((-var_itr_dn5) * var_weff);
        let eq34_e375_d_n6: f64 = ((-var_itr_dn6) * var_weff);
        let eq34_e376: f64 = (var_ttype * eq34_e375);
        let eq34_e376_d_n3: f64 = (var_ttype * eq34_e375_d_n3);
        let eq34_e376_d_n4: f64 = (var_ttype * eq34_e375_d_n4);
        let eq34_e376_d_n5: f64 = (var_ttype * eq34_e375_d_n5);
        let eq34_e376_d_n6: f64 = (var_ttype * eq34_e375_d_n6);
        let eq34_value: f64 = eq34_e376;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(4),
            Some(6),
            multiplicity * (eq34_value),
            [3, 4, 5, 6],
            [multiplicity * (eq34_e376_d_n3), multiplicity * (eq34_e376_d_n4), multiplicity * (eq34_e376_d_n5), multiplicity * (eq34_e376_d_n6)],
            [],
            [],
            1.0,
        );
        let eq35_e379: f64 = (var_ttype * var_itzf_f);
        let eq35_e379_d_n3: f64 = (var_ttype * var_itzf_f_dn3);
        let eq35_e379_d_n4: f64 = (var_ttype * var_itzf_f_dn4);
        let eq35_e379_d_n5: f64 = (var_ttype * var_itzf_f_dn5);
        let eq35_e379_d_n6: f64 = (var_ttype * var_itzf_f_dn6);
        let eq35_e379_d_n9: f64 = (var_ttype * var_itzf_f_dn9);
        let eq35_e381: f64 = (eq35_e379 * var_weff);
        let eq35_e381_d_n3: f64 = (eq35_e379_d_n3 * var_weff);
        let eq35_e381_d_n4: f64 = (eq35_e379_d_n4 * var_weff);
        let eq35_e381_d_n5: f64 = (eq35_e379_d_n5 * var_weff);
        let eq35_e381_d_n6: f64 = (eq35_e379_d_n6 * var_weff);
        let eq35_e381_d_n9: f64 = (eq35_e379_d_n9 * var_weff);
        let eq35_value: f64 = eq35_e381;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            Some(6),
            multiplicity * (eq35_value),
            [3, 4, 5, 6, 9],
            [multiplicity * (eq35_e381_d_n3), multiplicity * (eq35_e381_d_n4), multiplicity * (eq35_e381_d_n5), multiplicity * (eq35_e381_d_n6), multiplicity * (eq35_e381_d_n9)],
            [],
            [],
            1.0,
        );
        let eq36_e384: f64 = (var_ttype * var_qje);
        let eq36_e384_d_n1: f64 = (var_ttype * var_qje_dn1);
        let eq36_e384_d_n3: f64 = (var_ttype * var_qje_dn3);
        let eq36_e384_d_n4: f64 = (var_ttype * var_qje_dn4);
        let eq36_e384_d_n5: f64 = (var_ttype * var_qje_dn5);
        let eq36_e384_d_n6: f64 = (var_ttype * var_qje_dn6);
        let eq36_e386: f64 = (eq36_e384 * var_weff);
        let eq36_e386_d_n1: f64 = (eq36_e384_d_n1 * var_weff);
        let eq36_e386_d_n3: f64 = (eq36_e384_d_n3 * var_weff);
        let eq36_e386_d_n4: f64 = (eq36_e384_d_n4 * var_weff);
        let eq36_e386_d_n5: f64 = (eq36_e384_d_n5 * var_weff);
        let eq36_e386_d_n6: f64 = (eq36_e384_d_n6 * var_weff);
        let eq36_e387: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq36_e386);
        let eq36_value: f64 = eq36_e387;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq36_value),
            [1, 3, 4, 5, 6],
            [multiplicity * ((eq36_e386_d_n1 * ddt_scale)), multiplicity * ((eq36_e386_d_n3 * ddt_scale)), multiplicity * ((eq36_e386_d_n4 * ddt_scale)), multiplicity * ((eq36_e386_d_n5 * ddt_scale)), multiplicity * ((eq36_e386_d_n6 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq37_e390: f64 = (var_ttype * var_qde);
        let eq37_e390_d_n1: f64 = (var_ttype * var_qde_dn1);
        let eq37_e390_d_n2: f64 = (var_ttype * var_qde_dn2);
        let eq37_e390_d_n3: f64 = (var_ttype * var_qde_dn3);
        let eq37_e390_d_n4: f64 = (var_ttype * var_qde_dn4);
        let eq37_e390_d_n5: f64 = (var_ttype * var_qde_dn5);
        let eq37_e390_d_n6: f64 = (var_ttype * var_qde_dn6);
        let eq37_e392: f64 = (eq37_e390 * var_weff);
        let eq37_e392_d_n1: f64 = (eq37_e390_d_n1 * var_weff);
        let eq37_e392_d_n2: f64 = (eq37_e390_d_n2 * var_weff);
        let eq37_e392_d_n3: f64 = (eq37_e390_d_n3 * var_weff);
        let eq37_e392_d_n4: f64 = (eq37_e390_d_n4 * var_weff);
        let eq37_e392_d_n5: f64 = (eq37_e390_d_n5 * var_weff);
        let eq37_e392_d_n6: f64 = (eq37_e390_d_n6 * var_weff);
        let eq37_e393: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, eq37_e392);
        let eq37_value: f64 = eq37_e393;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq37_value),
            [1, 2, 3, 4, 5, 6],
            [multiplicity * ((eq37_e392_d_n1 * ddt_scale)), multiplicity * ((eq37_e392_d_n2 * ddt_scale)), multiplicity * ((eq37_e392_d_n3 * ddt_scale)), multiplicity * ((eq37_e392_d_n4 * ddt_scale)), multiplicity * ((eq37_e392_d_n5 * ddt_scale)), multiplicity * ((eq37_e392_d_n6 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq38_e396: f64 = (var_ttype * var_qjcx_1);
        let eq38_e396_d_n1: f64 = (var_ttype * var_qjcx_1_dn1);
        let eq38_e396_d_n3: f64 = (var_ttype * var_qjcx_1_dn3);
        let eq38_e396_d_n4: f64 = (var_ttype * var_qjcx_1_dn4);
        let eq38_e396_d_n5: f64 = (var_ttype * var_qjcx_1_dn5);
        let eq38_e396_d_n6: f64 = (var_ttype * var_qjcx_1_dn6);
        let eq38_e398: f64 = (eq38_e396 * var_weff);
        let eq38_e398_d_n1: f64 = (eq38_e396_d_n1 * var_weff);
        let eq38_e398_d_n3: f64 = (eq38_e396_d_n3 * var_weff);
        let eq38_e398_d_n4: f64 = (eq38_e396_d_n4 * var_weff);
        let eq38_e398_d_n5: f64 = (eq38_e396_d_n5 * var_weff);
        let eq38_e398_d_n6: f64 = (eq38_e396_d_n6 * var_weff);
        let eq38_e399: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq38_e398);
        let eq38_value: f64 = eq38_e399;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(1),
            Some(4),
            multiplicity * (eq38_value),
            [1, 3, 4, 5, 6],
            [multiplicity * ((eq38_e398_d_n1 * ddt_scale)), multiplicity * ((eq38_e398_d_n3 * ddt_scale)), multiplicity * ((eq38_e398_d_n4 * ddt_scale)), multiplicity * ((eq38_e398_d_n5 * ddt_scale)), multiplicity * ((eq38_e398_d_n6 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq39_e402: f64 = (var_ttype * var_qjci_1);
        let eq39_e402_d_n1: f64 = (var_ttype * var_qjci_1_dn1);
        let eq39_e402_d_n3: f64 = (var_ttype * var_qjci_1_dn3);
        let eq39_e402_d_n4: f64 = (var_ttype * var_qjci_1_dn4);
        let eq39_e402_d_n5: f64 = (var_ttype * var_qjci_1_dn5);
        let eq39_e402_d_n6: f64 = (var_ttype * var_qjci_1_dn6);
        let eq39_e404: f64 = (eq39_e402 * var_weff);
        let eq39_e404_d_n1: f64 = (eq39_e402_d_n1 * var_weff);
        let eq39_e404_d_n3: f64 = (eq39_e402_d_n3 * var_weff);
        let eq39_e404_d_n4: f64 = (eq39_e402_d_n4 * var_weff);
        let eq39_e404_d_n5: f64 = (eq39_e402_d_n5 * var_weff);
        let eq39_e404_d_n6: f64 = (eq39_e402_d_n6 * var_weff);
        let eq39_e405: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq39_e404);
        let eq39_value: f64 = eq39_e405;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(4),
            multiplicity * (eq39_value),
            [1, 3, 4, 5, 6],
            [multiplicity * ((eq39_e404_d_n1 * ddt_scale)), multiplicity * ((eq39_e404_d_n3 * ddt_scale)), multiplicity * ((eq39_e404_d_n4 * ddt_scale)), multiplicity * ((eq39_e404_d_n5 * ddt_scale)), multiplicity * ((eq39_e404_d_n6 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq40_e408: f64 = (var_ttype * var_qdc);
        let eq40_e408_d_n3: f64 = (var_ttype * var_qdc_dn3);
        let eq40_e408_d_n4: f64 = (var_ttype * var_qdc_dn4);
        let eq40_e408_d_n5: f64 = (var_ttype * var_qdc_dn5);
        let eq40_e408_d_n6: f64 = (var_ttype * var_qdc_dn6);
        let eq40_e410: f64 = (eq40_e408 * var_weff);
        let eq40_e410_d_n3: f64 = (eq40_e408_d_n3 * var_weff);
        let eq40_e410_d_n4: f64 = (eq40_e408_d_n4 * var_weff);
        let eq40_e410_d_n5: f64 = (eq40_e408_d_n5 * var_weff);
        let eq40_e410_d_n6: f64 = (eq40_e408_d_n6 * var_weff);
        let eq40_e411: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq40_e410);
        let eq40_value: f64 = eq40_e411;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * (eq40_value),
            [3, 4, 5, 6],
            [multiplicity * ((eq40_e410_d_n3 * ddt_scale)), multiplicity * ((eq40_e410_d_n4 * ddt_scale)), multiplicity * ((eq40_e410_d_n5 * ddt_scale)), multiplicity * ((eq40_e410_d_n6 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq41_e414: f64 = (var_ttype * var_qjs);
        let eq41_e414_d_n2: f64 = (var_ttype * var_qjs_dn2);
        let eq41_e414_d_n3: f64 = (var_ttype * var_qjs_dn3);
        let eq41_e414_d_n4: f64 = (var_ttype * var_qjs_dn4);
        let eq41_e416: f64 = (eq41_e414 * var_weff);
        let eq41_e416_d_n2: f64 = (eq41_e414_d_n2 * var_weff);
        let eq41_e416_d_n3: f64 = (eq41_e414_d_n3 * var_weff);
        let eq41_e416_d_n4: f64 = (eq41_e414_d_n4 * var_weff);
        let eq41_e417: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, eq41_e416);
        let eq41_value: f64 = eq41_e417;
        stamper.stamp_current_node3_local(
            Some(2),
            Some(4),
            multiplicity * (eq41_value),
            2,
            multiplicity * ((eq41_e416_d_n2 * ddt_scale)),
            3,
            multiplicity * ((eq41_e416_d_n3 * ddt_scale)),
            4,
            multiplicity * ((eq41_e416_d_n4 * ddt_scale)),
        );
        let eq42_e419: f64 = (-var_qxf1);
        let eq42_e421: f64 = (eq42_e419 * var_weff);
        let eq42_e421_d_n3: f64 = ((-var_qxf1_dn3) * var_weff);
        let eq42_e421_d_n4: f64 = ((-var_qxf1_dn4) * var_weff);
        let eq42_e421_d_n5: f64 = ((-var_qxf1_dn5) * var_weff);
        let eq42_e421_d_n6: f64 = ((-var_qxf1_dn6) * var_weff);
        let eq42_e422: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, eq42_e421);
        let eq42_value: f64 = eq42_e422;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq42_value),
            [3, 4, 5, 6],
            [multiplicity * ((eq42_e421_d_n3 * ddt_scale)), multiplicity * ((eq42_e421_d_n4 * ddt_scale)), multiplicity * ((eq42_e421_d_n5 * ddt_scale)), multiplicity * ((eq42_e421_d_n6 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq43_e425: f64 = (var_qxf1 * var_weff);
        let eq43_e425_d_n3: f64 = (var_qxf1_dn3 * var_weff);
        let eq43_e425_d_n4: f64 = (var_qxf1_dn4 * var_weff);
        let eq43_e425_d_n5: f64 = (var_qxf1_dn5 * var_weff);
        let eq43_e425_d_n6: f64 = (var_qxf1_dn6 * var_weff);
        let eq43_e426: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, eq43_e425);
        let eq43_value: f64 = eq43_e426;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * (eq43_value),
            [3, 4, 5, 6],
            [multiplicity * ((eq43_e425_d_n3 * ddt_scale)), multiplicity * ((eq43_e425_d_n4 * ddt_scale)), multiplicity * ((eq43_e425_d_n5 * ddt_scale)), multiplicity * ((eq43_e425_d_n6 * ddt_scale))],
            [],
            [],
            1.0,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        var_guard13: f64,
        var_guard20: f64,
        var_guard21: f64,
        var_qdc: f64,
        var_qdc_dn3: f64,
        var_qdc_dn4: f64,
        var_qdc_dn5: f64,
        var_qdc_dn6: f64,
        var_qde: f64,
        var_qde_dn1: f64,
        var_qde_dn2: f64,
        var_qde_dn3: f64,
        var_qde_dn4: f64,
        var_qde_dn5: f64,
        var_qde_dn6: f64,
        var_qjci_1: f64,
        var_qjci_1_dn1: f64,
        var_qjci_1_dn3: f64,
        var_qjci_1_dn4: f64,
        var_qjci_1_dn5: f64,
        var_qjci_1_dn6: f64,
        var_qjcx_1: f64,
        var_qjcx_1_dn1: f64,
        var_qjcx_1_dn3: f64,
        var_qjcx_1_dn4: f64,
        var_qjcx_1_dn5: f64,
        var_qjcx_1_dn6: f64,
        var_qje: f64,
        var_qje_dn1: f64,
        var_qje_dn3: f64,
        var_qje_dn4: f64,
        var_qje_dn5: f64,
        var_qje_dn6: f64,
        var_qjs: f64,
        var_qjs_dn2: f64,
        var_qjs_dn3: f64,
        var_qjs_dn4: f64,
        var_qxf1: f64,
        var_qxf1_dn3: f64,
        var_qxf1_dn4: f64,
        var_qxf1_dn5: f64,
        var_qxf1_dn6: f64,
        var_tff: f64,
        var_tff_dn1: f64,
        var_tff_dn2: f64,
        var_ttype: f64,
        var_weff: f64,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let eq2_e98_q: f64 = (nv9 - 0.0);
        let eq2_e99: f64 = (p.p83 * (nv9 - 0.0));
        let eq2_e99_q: f64 = (p.p83 * eq2_e98_q);
        stamper.stamp_current_reactive_node1(
            Some(nodes[9]),
            None,
            nodes[9],
            multiplicity * (p.p83),
        );
        let (eq5_e121, eq5_e121_d_n1, eq5_e121_d_n2, eq5_e121_d_n8, eq5_e121_q, eq5_e121_q_d_n1, eq5_e121_q_d_n2,) = {
    if (var_guard13 != 0.0) {
        let eq5_e118_q: f64 = (nv8 - 0.0);
        let eq5_e119: f64 = (var_tff * (nv8 - 0.0));
        let eq5_e119_d_n1: f64 = (var_tff_dn1 * (nv8 - 0.0));
        let eq5_e119_d_n2: f64 = (var_tff_dn2 * (nv8 - 0.0));
        let eq5_e119_q: f64 = (var_tff * eq5_e118_q);
        let eq5_e119_q_d_n1: f64 = (var_tff_dn1 * eq5_e118_q);
        let eq5_e119_q_d_n2: f64 = (var_tff_dn2 * eq5_e118_q);
        (eq5_e119, eq5_e119_d_n1, eq5_e119_d_n2, var_tff, eq5_e119_q, eq5_e119_q_d_n1, eq5_e119_q_d_n2,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node3(
            Some(nodes[8]),
            None,
            nodes[1],
            multiplicity * (eq5_e121_q_d_n1),
            nodes[2],
            multiplicity * (eq5_e121_q_d_n2),
            nodes[8],
            multiplicity * (eq5_e121_d_n8),
        );
        let (eq9_e154, eq9_e154_d_n3, eq9_e154_q,) = {
    if (var_guard20 != 0.0) {
        let eq9_e151: f64 = ((nv3 - 0.0) * p.p34);
        let eq9_e152_q: f64 = eq9_e151;
        (eq9_e151, p.p34, eq9_e152_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[3]),
            None,
            nodes[3],
            multiplicity * (eq9_e154_d_n3),
        );
        let (eq13_e195, eq13_e195_d_n3, eq13_e195_q,) = {
    if ((var_guard20 == 0.0) && (var_guard21 != 0.0)) {
        let eq13_e192: f64 = (p.p34 * (nv3 - 0.0));
        let eq13_e193_q: f64 = eq13_e192;
        (eq13_e192, p.p34, eq13_e193_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[3]),
            None,
            nodes[3],
            multiplicity * (eq13_e195_d_n3),
        );
        let (eq15_e214, eq15_e214_d_n7, eq15_e214_q,) = {
    if ((var_guard20 == 0.0) && (var_guard21 != 0.0)) {
        let eq15_e211: f64 = (p.p36 * (nv7 - 0.0));
        let eq15_e212_q: f64 = eq15_e211;
        (eq15_e211, p.p36, eq15_e212_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[7]),
            None,
            nodes[7],
            multiplicity * (eq15_e214_d_n7),
        );
        let eq36_e384: f64 = (var_ttype * var_qje);
        let eq36_e384_d_n1: f64 = (var_ttype * var_qje_dn1);
        let eq36_e384_d_n3: f64 = (var_ttype * var_qje_dn3);
        let eq36_e384_d_n4: f64 = (var_ttype * var_qje_dn4);
        let eq36_e384_d_n5: f64 = (var_ttype * var_qje_dn5);
        let eq36_e384_d_n6: f64 = (var_ttype * var_qje_dn6);
        let eq36_e386: f64 = (eq36_e384 * var_weff);
        let eq36_e386_d_n1: f64 = (eq36_e384_d_n1 * var_weff);
        let eq36_e386_d_n3: f64 = (eq36_e384_d_n3 * var_weff);
        let eq36_e386_d_n4: f64 = (eq36_e384_d_n4 * var_weff);
        let eq36_e386_d_n5: f64 = (eq36_e384_d_n5 * var_weff);
        let eq36_e386_d_n6: f64 = (eq36_e384_d_n6 * var_weff);
        let eq36_e387_q: f64 = eq36_e386;
        let eq36_reactive_node_derivatives: [f64; 10] = [0.0, eq36_e386_d_n1, 0.0, eq36_e386_d_n3, eq36_e386_d_n4, eq36_e386_d_n5, eq36_e386_d_n6, 0.0, 0.0, 0.0];
        let eq36_reactive_branch_derivatives: [f64; 8] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes,
            &eq36_reactive_node_derivatives,
            branches,
            &eq36_reactive_branch_derivatives,
            multiplicity,
        );
        let eq37_e390: f64 = (var_ttype * var_qde);
        let eq37_e390_d_n1: f64 = (var_ttype * var_qde_dn1);
        let eq37_e390_d_n2: f64 = (var_ttype * var_qde_dn2);
        let eq37_e390_d_n3: f64 = (var_ttype * var_qde_dn3);
        let eq37_e390_d_n4: f64 = (var_ttype * var_qde_dn4);
        let eq37_e390_d_n5: f64 = (var_ttype * var_qde_dn5);
        let eq37_e390_d_n6: f64 = (var_ttype * var_qde_dn6);
        let eq37_e392: f64 = (eq37_e390 * var_weff);
        let eq37_e392_d_n1: f64 = (eq37_e390_d_n1 * var_weff);
        let eq37_e392_d_n2: f64 = (eq37_e390_d_n2 * var_weff);
        let eq37_e392_d_n3: f64 = (eq37_e390_d_n3 * var_weff);
        let eq37_e392_d_n4: f64 = (eq37_e390_d_n4 * var_weff);
        let eq37_e392_d_n5: f64 = (eq37_e390_d_n5 * var_weff);
        let eq37_e392_d_n6: f64 = (eq37_e390_d_n6 * var_weff);
        let eq37_e393_q: f64 = eq37_e392;
        let eq37_reactive_node_derivatives: [f64; 10] = [0.0, eq37_e392_d_n1, eq37_e392_d_n2, eq37_e392_d_n3, eq37_e392_d_n4, eq37_e392_d_n5, eq37_e392_d_n6, 0.0, 0.0, 0.0];
        let eq37_reactive_branch_derivatives: [f64; 8] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes,
            &eq37_reactive_node_derivatives,
            branches,
            &eq37_reactive_branch_derivatives,
            multiplicity,
        );
        let eq38_e396: f64 = (var_ttype * var_qjcx_1);
        let eq38_e396_d_n1: f64 = (var_ttype * var_qjcx_1_dn1);
        let eq38_e396_d_n3: f64 = (var_ttype * var_qjcx_1_dn3);
        let eq38_e396_d_n4: f64 = (var_ttype * var_qjcx_1_dn4);
        let eq38_e396_d_n5: f64 = (var_ttype * var_qjcx_1_dn5);
        let eq38_e396_d_n6: f64 = (var_ttype * var_qjcx_1_dn6);
        let eq38_e398: f64 = (eq38_e396 * var_weff);
        let eq38_e398_d_n1: f64 = (eq38_e396_d_n1 * var_weff);
        let eq38_e398_d_n3: f64 = (eq38_e396_d_n3 * var_weff);
        let eq38_e398_d_n4: f64 = (eq38_e396_d_n4 * var_weff);
        let eq38_e398_d_n5: f64 = (eq38_e396_d_n5 * var_weff);
        let eq38_e398_d_n6: f64 = (eq38_e396_d_n6 * var_weff);
        let eq38_e399_q: f64 = eq38_e398;
        let eq38_reactive_node_derivatives: [f64; 10] = [0.0, eq38_e398_d_n1, 0.0, eq38_e398_d_n3, eq38_e398_d_n4, eq38_e398_d_n5, eq38_e398_d_n6, 0.0, 0.0, 0.0];
        let eq38_reactive_branch_derivatives: [f64; 8] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[4]),
            nodes,
            &eq38_reactive_node_derivatives,
            branches,
            &eq38_reactive_branch_derivatives,
            multiplicity,
        );
        let eq39_e402: f64 = (var_ttype * var_qjci_1);
        let eq39_e402_d_n1: f64 = (var_ttype * var_qjci_1_dn1);
        let eq39_e402_d_n3: f64 = (var_ttype * var_qjci_1_dn3);
        let eq39_e402_d_n4: f64 = (var_ttype * var_qjci_1_dn4);
        let eq39_e402_d_n5: f64 = (var_ttype * var_qjci_1_dn5);
        let eq39_e402_d_n6: f64 = (var_ttype * var_qjci_1_dn6);
        let eq39_e404: f64 = (eq39_e402 * var_weff);
        let eq39_e404_d_n1: f64 = (eq39_e402_d_n1 * var_weff);
        let eq39_e404_d_n3: f64 = (eq39_e402_d_n3 * var_weff);
        let eq39_e404_d_n4: f64 = (eq39_e402_d_n4 * var_weff);
        let eq39_e404_d_n5: f64 = (eq39_e402_d_n5 * var_weff);
        let eq39_e404_d_n6: f64 = (eq39_e402_d_n6 * var_weff);
        let eq39_e405_q: f64 = eq39_e404;
        let eq39_reactive_node_derivatives: [f64; 10] = [0.0, eq39_e404_d_n1, 0.0, eq39_e404_d_n3, eq39_e404_d_n4, eq39_e404_d_n5, eq39_e404_d_n6, 0.0, 0.0, 0.0];
        let eq39_reactive_branch_derivatives: [f64; 8] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            nodes,
            &eq39_reactive_node_derivatives,
            branches,
            &eq39_reactive_branch_derivatives,
            multiplicity,
        );
        let eq40_e408: f64 = (var_ttype * var_qdc);
        let eq40_e408_d_n3: f64 = (var_ttype * var_qdc_dn3);
        let eq40_e408_d_n4: f64 = (var_ttype * var_qdc_dn4);
        let eq40_e408_d_n5: f64 = (var_ttype * var_qdc_dn5);
        let eq40_e408_d_n6: f64 = (var_ttype * var_qdc_dn6);
        let eq40_e410: f64 = (eq40_e408 * var_weff);
        let eq40_e410_d_n3: f64 = (eq40_e408_d_n3 * var_weff);
        let eq40_e410_d_n4: f64 = (eq40_e408_d_n4 * var_weff);
        let eq40_e410_d_n5: f64 = (eq40_e408_d_n5 * var_weff);
        let eq40_e410_d_n6: f64 = (eq40_e408_d_n6 * var_weff);
        let eq40_e411_q: f64 = eq40_e410;
        stamper.stamp_current_reactive(
            Some(nodes[5]),
            Some(nodes[4]),
            &[
                GeneratedDerivative::node(nodes[3], multiplicity * (eq40_e410_d_n3)),
                GeneratedDerivative::node(nodes[4], multiplicity * (eq40_e410_d_n4)),
                GeneratedDerivative::node(nodes[5], multiplicity * (eq40_e410_d_n5)),
                GeneratedDerivative::node(nodes[6], multiplicity * (eq40_e410_d_n6)),
            ],
        );
        let eq41_e414: f64 = (var_ttype * var_qjs);
        let eq41_e414_d_n2: f64 = (var_ttype * var_qjs_dn2);
        let eq41_e414_d_n3: f64 = (var_ttype * var_qjs_dn3);
        let eq41_e414_d_n4: f64 = (var_ttype * var_qjs_dn4);
        let eq41_e416: f64 = (eq41_e414 * var_weff);
        let eq41_e416_d_n2: f64 = (eq41_e414_d_n2 * var_weff);
        let eq41_e416_d_n3: f64 = (eq41_e414_d_n3 * var_weff);
        let eq41_e416_d_n4: f64 = (eq41_e414_d_n4 * var_weff);
        let eq41_e417_q: f64 = eq41_e416;
        stamper.stamp_current_reactive_node3(
            Some(nodes[2]),
            Some(nodes[4]),
            nodes[2],
            multiplicity * (eq41_e416_d_n2),
            nodes[3],
            multiplicity * (eq41_e416_d_n3),
            nodes[4],
            multiplicity * (eq41_e416_d_n4),
        );
        let eq42_e419: f64 = (-var_qxf1);
        let eq42_e421: f64 = (eq42_e419 * var_weff);
        let eq42_e421_d_n3: f64 = ((-var_qxf1_dn3) * var_weff);
        let eq42_e421_d_n4: f64 = ((-var_qxf1_dn4) * var_weff);
        let eq42_e421_d_n5: f64 = ((-var_qxf1_dn5) * var_weff);
        let eq42_e421_d_n6: f64 = ((-var_qxf1_dn6) * var_weff);
        let eq42_e422_q: f64 = eq42_e421;
        stamper.stamp_current_reactive(
            Some(nodes[5]),
            Some(nodes[6]),
            &[
                GeneratedDerivative::node(nodes[3], multiplicity * (eq42_e421_d_n3)),
                GeneratedDerivative::node(nodes[4], multiplicity * (eq42_e421_d_n4)),
                GeneratedDerivative::node(nodes[5], multiplicity * (eq42_e421_d_n5)),
                GeneratedDerivative::node(nodes[6], multiplicity * (eq42_e421_d_n6)),
            ],
        );
        let eq43_e425: f64 = (var_qxf1 * var_weff);
        let eq43_e425_d_n3: f64 = (var_qxf1_dn3 * var_weff);
        let eq43_e425_d_n4: f64 = (var_qxf1_dn4 * var_weff);
        let eq43_e425_d_n5: f64 = (var_qxf1_dn5 * var_weff);
        let eq43_e425_d_n6: f64 = (var_qxf1_dn6 * var_weff);
        let eq43_e426_q: f64 = eq43_e425;
        stamper.stamp_current_reactive(
            Some(nodes[5]),
            Some(nodes[4]),
            &[
                GeneratedDerivative::node(nodes[3], multiplicity * (eq43_e425_d_n3)),
                GeneratedDerivative::node(nodes[4], multiplicity * (eq43_e425_d_n4)),
                GeneratedDerivative::node(nodes[5], multiplicity * (eq43_e425_d_n5)),
                GeneratedDerivative::node(nodes[6], multiplicity * (eq43_e425_d_n6)),
            ],
        );
    }
}
