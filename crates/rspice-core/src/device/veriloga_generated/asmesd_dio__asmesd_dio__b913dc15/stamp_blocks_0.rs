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
        var_arg0_dn2_slot: &mut f64,
        var_arg_dn2_slot: &mut f64,
        var_arg_dn3_slot: &mut f64,
        var_arg_dn4_slot: &mut f64,
        var_argbv_slot: &mut f64,
        var_argbv_dn2_slot: &mut f64,
        var_argbv_dn3_slot: &mut f64,
        var_argbv_dn4_slot: &mut f64,
        var_argbvvt_slot: &mut f64,
        var_argbvvt_dn2_slot: &mut f64,
        var_argt_slot: &mut f64,
        var_argt_dn2_slot: &mut f64,
        var_argtr_slot: &mut f64,
        var_argtr_dn2_slot: &mut f64,
        var_bvr_t_slot: &mut f64,
        var_bvr_t_dn2_slot: &mut f64,
        var_cje_i_slot: &mut f64,
        var_cje_t_slot: &mut f64,
        var_cje_t_dn2_slot: &mut f64,
        var_cjt_slot: &mut f64,
        var_cjt_dn2_slot: &mut f64,
        var_egfet_slot: &mut f64,
        var_egfet_dn2_slot: &mut f64,
        var_fact1_slot: &mut f64,
        var_fact2_slot: &mut f64,
        var_fact2_dn2_slot: &mut f64,
        var_gmanew_slot: &mut f64,
        var_gmanew_dn2_slot: &mut f64,
        var_gmaold_slot: &mut f64,
        var_gmaold_dn2_slot: &mut f64,
        var_guard3_slot: &mut f64,
        var_guard4_slot: &mut f64,
        var_guard5_slot: &mut f64,
        var_guard6_slot: &mut f64,
        var_ifwd_slot: &mut f64,
        var_ifwd_dn2_slot: &mut f64,
        var_ifwd_dn3_slot: &mut f64,
        var_ifwd_dn4_slot: &mut f64,
        var_ijbv_t_slot: &mut f64,
        var_ijbv_t_dn2_slot: &mut f64,
        var_is_t_slot: &mut f64,
        var_is_t_dn2_slot: &mut f64,
        var_isr_t_slot: &mut f64,
        var_isr_t_dn2_slot: &mut f64,
        var_itrev_slot: &mut f64,
        var_itrev_dn2_slot: &mut f64,
        var_itrev_dn3_slot: &mut f64,
        var_itrev_dn4_slot: &mut f64,
        var_le_slot: &mut f64,
        var_le_dn2_slot: &mut f64,
        var_le_dn3_slot: &mut f64,
        var_le_dn4_slot: &mut f64,
        var_lebv_slot: &mut f64,
        var_lebv_dn2_slot: &mut f64,
        var_lebv_dn3_slot: &mut f64,
        var_lebv_dn4_slot: &mut f64,
        var_lnrt_slot: &mut f64,
        var_lnrt_dn2_slot: &mut f64,
        var_pbfact_slot: &mut f64,
        var_pbfact_dn2_slot: &mut f64,
        var_pbo_slot: &mut f64,
        var_pbo_dn2_slot: &mut f64,
        var_rt_slot: &mut f64,
        var_rt_dn2_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_tamb_slot: &mut f64,
        var_tamb_dn2_slot: &mut f64,
        var_tdev_slot: &mut f64,
        var_tdev_dn2_slot: &mut f64,
        var_theexp_t_slot: &mut f64,
        var_theexp_t_dn2_slot: &mut f64,
        var_tnom_slot: &mut f64,
        var_ttype_slot: &mut f64,
        var_vbbi_slot: &mut f64,
        var_vbbi_dn0_slot: &mut f64,
        var_vbbi_dn3_slot: &mut f64,
        var_vbiei_slot: &mut f64,
        var_vbiei_dn3_slot: &mut f64,
        var_vbiei_dn4_slot: &mut f64,
        var_veei_slot: &mut f64,
        var_veei_dn1_slot: &mut f64,
        var_veei_dn4_slot: &mut f64,
        var_vje_t_slot: &mut f64,
        var_vje_t_dn2_slot: &mut f64,
        var_vt_slot: &mut f64,
        var_vt_dn2_slot: &mut f64,
        var_weff_slot: &mut f64,
    ) {
        let ctx_temp = ctx.temperature();
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let mut var_arg: f64 = *var_arg_slot;
        let mut var_arg0: f64 = *var_arg0_slot;
        let mut var_arg0_dn2: f64 = *var_arg0_dn2_slot;
        let mut var_arg_dn2: f64 = *var_arg_dn2_slot;
        let mut var_arg_dn3: f64 = *var_arg_dn3_slot;
        let mut var_arg_dn4: f64 = *var_arg_dn4_slot;
        let mut var_argbv: f64 = *var_argbv_slot;
        let mut var_argbv_dn2: f64 = *var_argbv_dn2_slot;
        let mut var_argbv_dn3: f64 = *var_argbv_dn3_slot;
        let mut var_argbv_dn4: f64 = *var_argbv_dn4_slot;
        let mut var_argbvvt: f64 = *var_argbvvt_slot;
        let mut var_argbvvt_dn2: f64 = *var_argbvvt_dn2_slot;
        let mut var_argt: f64 = *var_argt_slot;
        let mut var_argt_dn2: f64 = *var_argt_dn2_slot;
        let mut var_argtr: f64 = *var_argtr_slot;
        let mut var_argtr_dn2: f64 = *var_argtr_dn2_slot;
        let mut var_bvr_t: f64 = *var_bvr_t_slot;
        let mut var_bvr_t_dn2: f64 = *var_bvr_t_dn2_slot;
        let mut var_cje_i: f64 = *var_cje_i_slot;
        let mut var_cje_t: f64 = *var_cje_t_slot;
        let mut var_cje_t_dn2: f64 = *var_cje_t_dn2_slot;
        let mut var_cjt: f64 = *var_cjt_slot;
        let mut var_cjt_dn2: f64 = *var_cjt_dn2_slot;
        let mut var_egfet: f64 = *var_egfet_slot;
        let mut var_egfet_dn2: f64 = *var_egfet_dn2_slot;
        let mut var_fact1: f64 = *var_fact1_slot;
        let mut var_fact2: f64 = *var_fact2_slot;
        let mut var_fact2_dn2: f64 = *var_fact2_dn2_slot;
        let mut var_gmanew: f64 = *var_gmanew_slot;
        let mut var_gmanew_dn2: f64 = *var_gmanew_dn2_slot;
        let mut var_gmaold: f64 = *var_gmaold_slot;
        let mut var_gmaold_dn2: f64 = *var_gmaold_dn2_slot;
        let mut var_guard3: f64 = *var_guard3_slot;
        let mut var_guard4: f64 = *var_guard4_slot;
        let mut var_guard5: f64 = *var_guard5_slot;
        let mut var_guard6: f64 = *var_guard6_slot;
        let mut var_ifwd: f64 = *var_ifwd_slot;
        let mut var_ifwd_dn2: f64 = *var_ifwd_dn2_slot;
        let mut var_ifwd_dn3: f64 = *var_ifwd_dn3_slot;
        let mut var_ifwd_dn4: f64 = *var_ifwd_dn4_slot;
        let mut var_ijbv_t: f64 = *var_ijbv_t_slot;
        let mut var_ijbv_t_dn2: f64 = *var_ijbv_t_dn2_slot;
        let mut var_is_t: f64 = *var_is_t_slot;
        let mut var_is_t_dn2: f64 = *var_is_t_dn2_slot;
        let mut var_isr_t: f64 = *var_isr_t_slot;
        let mut var_isr_t_dn2: f64 = *var_isr_t_dn2_slot;
        let mut var_itrev: f64 = *var_itrev_slot;
        let mut var_itrev_dn2: f64 = *var_itrev_dn2_slot;
        let mut var_itrev_dn3: f64 = *var_itrev_dn3_slot;
        let mut var_itrev_dn4: f64 = *var_itrev_dn4_slot;
        let mut var_le: f64 = *var_le_slot;
        let mut var_le_dn2: f64 = *var_le_dn2_slot;
        let mut var_le_dn3: f64 = *var_le_dn3_slot;
        let mut var_le_dn4: f64 = *var_le_dn4_slot;
        let mut var_lebv: f64 = *var_lebv_slot;
        let mut var_lebv_dn2: f64 = *var_lebv_dn2_slot;
        let mut var_lebv_dn3: f64 = *var_lebv_dn3_slot;
        let mut var_lebv_dn4: f64 = *var_lebv_dn4_slot;
        let mut var_lnrt: f64 = *var_lnrt_slot;
        let mut var_lnrt_dn2: f64 = *var_lnrt_dn2_slot;
        let mut var_pbfact: f64 = *var_pbfact_slot;
        let mut var_pbfact_dn2: f64 = *var_pbfact_dn2_slot;
        let mut var_pbo: f64 = *var_pbo_slot;
        let mut var_pbo_dn2: f64 = *var_pbo_dn2_slot;
        let mut var_rt: f64 = *var_rt_slot;
        let mut var_rt_dn2: f64 = *var_rt_dn2_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_tamb: f64 = *var_tamb_slot;
        let mut var_tamb_dn2: f64 = *var_tamb_dn2_slot;
        let mut var_tdev: f64 = *var_tdev_slot;
        let mut var_tdev_dn2: f64 = *var_tdev_dn2_slot;
        let mut var_theexp_t: f64 = *var_theexp_t_slot;
        let mut var_theexp_t_dn2: f64 = *var_theexp_t_dn2_slot;
        let mut var_tnom: f64 = *var_tnom_slot;
        let mut var_ttype: f64 = *var_ttype_slot;
        let mut var_vbbi: f64 = *var_vbbi_slot;
        let mut var_vbbi_dn0: f64 = *var_vbbi_dn0_slot;
        let mut var_vbbi_dn3: f64 = *var_vbbi_dn3_slot;
        let mut var_vbiei: f64 = *var_vbiei_slot;
        let mut var_vbiei_dn3: f64 = *var_vbiei_dn3_slot;
        let mut var_vbiei_dn4: f64 = *var_vbiei_dn4_slot;
        let mut var_veei: f64 = *var_veei_slot;
        let mut var_veei_dn1: f64 = *var_veei_dn1_slot;
        let mut var_veei_dn4: f64 = *var_veei_dn4_slot;
        let mut var_vje_t: f64 = *var_vje_t_slot;
        let mut var_vje_t_dn2: f64 = *var_vje_t_dn2_slot;
        let mut var_vt: f64 = *var_vt_slot;
        let mut var_vt_dn2: f64 = *var_vt_dn2_slot;
        let mut var_weff: f64 = *var_weff_slot;

        let assign00_e291: f64 = ctx_temp;
        let assign00_e293: f64 = (assign00_e291 + (nv2 - 0.0));
        let assign00_e295: f64 = (assign00_e293 + p.p45);
        var_tamb = assign00_e295;
        var_tamb_dn2 = 1.0;

        let assign10_e298: f64 = (1026.85 + 273.15);
        let assign10_e301: f64 = (-100.0);
        let assign10_e303: f64 = (assign10_e301 + 273.15);
        let (assign10_e310, assign10_e310_d_n2,) = {
    if (var_tamb > assign10_e303) {
        (var_tamb, var_tamb_dn2,)
    } else {
        let assign10_e307: f64 = (-100.0);
        let assign10_e309: f64 = (assign10_e307 + 273.15);
        (assign10_e309, 0.0,)
    }
};
        let (assign10_e327, assign10_e327_d_n2,) = {
    if (assign10_e298 < assign10_e310) {
        let assign10_e314: f64 = (1026.85 + 273.15);
        (assign10_e314, 0.0,)
    } else {
        let assign10_e317: f64 = (-100.0);
        let assign10_e319: f64 = (assign10_e317 + 273.15);
        let (assign10_e326, assign10_e326_d_n2,) = {
            if (var_tamb > assign10_e319) {
                (var_tamb, var_tamb_dn2,)
            } else {
                let assign10_e323: f64 = (-100.0);
                let assign10_e325: f64 = (assign10_e323 + 273.15);
                (assign10_e325, 0.0,)
            }
        };
        (assign10_e326, assign10_e326_d_n2,)
    }
};
        var_tdev = assign10_e327;
        var_tdev_dn2 = assign10_e327_d_n2;

        let assign40_e337: f64 = (p.p43 * p.p42);
        var_weff = assign40_e337;

        let assign50_e340: f64 = (p.p25 + 273.15);
        var_tnom = assign50_e340;

        let assign60_e343: f64 = (8.6170869e-5 * var_tdev);
        var_vt = assign60_e343;
        var_vt_dn2 = (8.6170869e-5 * var_tdev_dn2);

        let assign70_e346: f64 = (var_tdev / var_tnom);
        var_rt = assign70_e346;
        var_rt_dn2 = (var_tdev_dn2 / var_tnom);

        let assign80_e348: f64 = (var_rt).ln();
        var_lnrt = assign80_e348;
        var_lnrt_dn2 = (var_rt_dn2 / var_rt);

        let assign90_e351: f64 = (p.p22 * var_lnrt);
        let assign90_e355: f64 = (var_rt - 1.0);
        let assign90_e356: f64 = (p.p21 * assign90_e355);
        let assign90_e358: f64 = (assign90_e356 / var_vt);
        let assign90_e359: f64 = (assign90_e351 + assign90_e358);
        var_argt = assign90_e359;
        var_argt_dn2 = ((p.p22 * var_lnrt_dn2) + ((((p.p21 * var_rt_dn2) * var_vt) - (assign90_e356 * var_vt_dn2)) / (var_vt * var_vt)));

        let assign100_e362: f64 = (p.p23 * var_lnrt);
        var_argtr = assign100_e362;
        var_argtr_dn2 = (p.p23 * var_lnrt_dn2);

        let assign110_e365: f64 = (var_argt).exp();
        let assign110_e366: f64 = (p.p0 * assign110_e365);
        var_is_t = assign110_e366;
        var_is_t_dn2 = (p.p0 * (assign110_e365 * var_argt_dn2));

        let assign120_e369: f64 = (var_argtr).exp();
        let assign120_e370: f64 = (p.p2 * assign120_e369);
        var_isr_t = assign120_e370;
        var_isr_t_dn2 = (p.p2 * (assign120_e369 * var_argtr_dn2));

        let assign130_e376: f64 = (var_rt - 1.0);
        let assign130_e377: f64 = (p.p7 * assign130_e376);
        let assign130_e378: f64 = (1.0 + assign130_e377);
        let assign130_e379: f64 = (p.p47 * assign130_e378);
        var_ijbv_t = assign130_e379;
        var_ijbv_t_dn2 = (p.p47 * (p.p7 * var_rt_dn2));

        let assign140_e385: f64 = (var_rt - 1.0);
        let assign140_e386: f64 = (p.p6 * assign140_e385);
        let assign140_e387: f64 = (1.0 + assign140_e386);
        let assign140_e388: f64 = (p.p5 * assign140_e387);
        var_bvr_t = assign140_e388;
        var_bvr_t_dn2 = (p.p5 * (p.p6 * var_rt_dn2));

        let assign150_e394: f64 = (var_rt - 1.0);
        let assign150_e395: f64 = (p.p10 * assign150_e394);
        let assign150_e396: f64 = (1.0 + assign150_e395);
        let assign150_e397: f64 = (p.p9 * assign150_e396);
        var_theexp_t = assign150_e397;
        var_theexp_t_dn2 = (p.p9 * (p.p10 * var_rt_dn2));

        var_cje_i = p.p16;

        let assign170_e401: f64 = (var_tnom / 300.15);
        var_fact1 = assign170_e401;

        let assign180_e404: f64 = (var_tdev / 300.15);
        var_fact2 = assign180_e404;
        var_fact2_dn2 = (var_tdev_dn2 / 300.15);

        let assign190_e408: f64 = (0.000702 * var_tdev);
        let assign190_e410: f64 = (assign190_e408 * var_tdev);
        let assign190_e413: f64 = (1108.0 + var_tdev);
        let assign190_e414: f64 = (assign190_e410 / assign190_e413);
        let assign190_e415: f64 = (1.16 - assign190_e414);
        var_egfet = assign190_e415;
        var_egfet_dn2 = (-((((((0.000702 * var_tdev_dn2) * var_tdev) + (assign190_e408 * var_tdev_dn2)) * assign190_e413) - (assign190_e410 * var_tdev_dn2)) / (assign190_e413 * assign190_e413)));

        let assign200_e417: f64 = (-var_egfet);
        let assign200_e421: f64 = (var_tdev + var_tdev);
        let assign200_e422: f64 = (1.3806226e-23 * assign200_e421);
        let assign200_e423: f64 = (assign200_e417 / assign200_e422);
        let assign200_e428: f64 = (300.15 + 300.15);
        let assign200_e429: f64 = (1.3806226e-23 * assign200_e428);
        let assign200_e430: f64 = (1.1150877 / assign200_e429);
        let assign200_e431: f64 = (assign200_e423 + assign200_e430);
        var_arg0 = assign200_e431;
        var_arg0_dn2 = ((((-var_egfet_dn2) * assign200_e422) - (assign200_e417 * (1.3806226e-23 * (var_tdev_dn2 + var_tdev_dn2)))) / (assign200_e422 * assign200_e422));

        let assign210_e434: f64 = (var_vt + var_vt);
        let assign210_e435: f64 = (-assign210_e434);
        let assign210_e438: f64 = (var_fact2).ln();
        let assign210_e439: f64 = (1.5 * assign210_e438);
        let assign210_e442: f64 = (1.6021918e-19 * var_arg0);
        let assign210_e443: f64 = (assign210_e439 + assign210_e442);
        let assign210_e444: f64 = (assign210_e435 * assign210_e443);
        var_pbfact = assign210_e444;
        var_pbfact_dn2 = (((-(var_vt_dn2 + var_vt_dn2)) * assign210_e443) + (assign210_e435 * ((1.5 * (var_fact2_dn2 / var_fact2)) + (1.6021918e-19 * var_arg0_dn2))));

        let assign220_e447: f64 = (p.p17 - var_pbfact);
        let assign220_e449: f64 = (assign220_e447 / var_fact1);
        var_pbo = assign220_e449;
        var_pbo_dn2 = ((-var_pbfact_dn2) / var_fact1);

        let assign230_e452: f64 = (p.p17 - var_pbo);
        let assign230_e454: f64 = (assign230_e452 / var_pbo);
        var_gmaold = assign230_e454;
        var_gmaold_dn2 = ((((-var_pbo_dn2) * var_pbo) - (assign230_e452 * var_pbo_dn2)) / (var_pbo * var_pbo));

        let assign240_e461: f64 = (var_tnom - 300.15);
        let assign240_e462: f64 = (0.0004 * assign240_e461);
        let assign240_e464: f64 = (assign240_e462 - var_gmaold);
        let assign240_e465: f64 = (p.p18 * assign240_e464);
        let assign240_e466: f64 = (1.0 + assign240_e465);
        let assign240_e467: f64 = (var_cje_i / assign240_e466);
        var_cjt = assign240_e467;
        var_cjt_dn2 = (-((var_cje_i * (p.p18 * (-var_gmaold_dn2))) / (assign240_e466 * assign240_e466)));

        let assign250_e470: f64 = (var_fact2 * var_pbo);
        let assign250_e472: f64 = (assign250_e470 + var_pbfact);
        var_vje_t = assign250_e472;
        var_vje_t_dn2 = (((var_fact2_dn2 * var_pbo) + (var_fact2 * var_pbo_dn2)) + var_pbfact_dn2);

        let assign260_e475: f64 = (var_vje_t - var_pbo);
        let assign260_e477: f64 = (assign260_e475 / var_pbo);
        var_gmanew = assign260_e477;
        var_gmanew_dn2 = ((((var_vje_t_dn2 - var_pbo_dn2) * var_pbo) - (assign260_e475 * var_pbo_dn2)) / (var_pbo * var_pbo));

        let assign270_e484: f64 = (var_tdev - 300.15);
        let assign270_e485: f64 = (0.0004 * assign270_e484);
        let assign270_e487: f64 = (assign270_e485 - var_gmanew);
        let assign270_e488: f64 = (p.p18 * assign270_e487);
        let assign270_e489: f64 = (1.0 + assign270_e488);
        let assign270_e490: f64 = (var_cjt * assign270_e489);
        var_cje_t = assign270_e490;
        var_cje_t_dn2 = ((var_cjt_dn2 * assign270_e489) + (var_cjt * (p.p18 * ((0.0004 * var_tdev_dn2) - var_gmanew_dn2))));

        var_ttype = p.p29;

        let assign290_e494: f64 = (var_ttype * (nv3 - nv4));
        var_vbiei = assign290_e494;
        var_vbiei_dn3 = var_ttype;
        var_vbiei_dn4 = (-var_ttype);

        let assign300_e497: f64 = (var_ttype * (nv0 - nv3));
        var_vbbi = assign300_e497;
        var_vbbi_dn0 = var_ttype;
        var_vbbi_dn3 = (-var_ttype);

        let assign310_e500: f64 = (var_ttype * (nv1 - nv4));
        var_veei = assign310_e500;
        var_veei_dn1 = var_ttype;
        var_veei_dn4 = (-var_ttype);

        let assign320_e503: f64 = if var_is_t > 0.0 { 1.0 } else { 0.0 };
        var_guard3 = assign320_e503;

        let (assign330_e511, assign330_e511_d_n2, assign330_e511_d_n3, assign330_e511_d_n4,) = {
    if (var_guard3 != 0.0) {
        let assign330_e508: f64 = (p.p1 * var_vt);
        let assign330_e509: f64 = (var_vbiei / assign330_e508);
        (assign330_e509, (-((var_vbiei * (p.p1 * var_vt_dn2)) / (assign330_e508 * assign330_e508))), (var_vbiei_dn3 / assign330_e508), (var_vbiei_dn4 / assign330_e508),)
    } else {
        (var_arg, var_arg_dn2, var_arg_dn3, var_arg_dn4,)
    }
};
        var_arg = assign330_e511;
        var_arg_dn2 = assign330_e511_d_n2;
        var_arg_dn3 = assign330_e511_d_n3;
        var_arg_dn4 = assign330_e511_d_n4;

        let (assign340_e522, assign340_e522_d_n2, assign340_e522_d_n3, assign340_e522_d_n4,) = {
    if (var_guard3 != 0.0) {
        let assign340_e514: f64 = (-var_vbiei);
        let assign340_e516: f64 = (assign340_e514 - var_bvr_t);
        let assign340_e519: f64 = (p.p11 * var_vt);
        let assign340_e520: f64 = (assign340_e516 / assign340_e519);
        (assign340_e520, ((((-var_bvr_t_dn2) * assign340_e519) - (assign340_e516 * (p.p11 * var_vt_dn2))) / (assign340_e519 * assign340_e519)), ((-var_vbiei_dn3) / assign340_e519), ((-var_vbiei_dn4) / assign340_e519),)
    } else {
        (var_argbv, var_argbv_dn2, var_argbv_dn3, var_argbv_dn4,)
    }
};
        var_argbv = assign340_e522;
        var_argbv_dn2 = assign340_e522_d_n2;
        var_argbv_dn3 = assign340_e522_d_n3;
        var_argbv_dn4 = assign340_e522_d_n4;

        let (assign350_e531, assign350_e531_d_n2,) = {
    if (var_guard3 != 0.0) {
        let assign350_e525: f64 = (-var_bvr_t);
        let assign350_e528: f64 = (p.p11 * var_vt);
        let assign350_e529: f64 = (assign350_e525 / assign350_e528);
        (assign350_e529, ((((-var_bvr_t_dn2) * assign350_e528) - (assign350_e525 * (p.p11 * var_vt_dn2))) / (assign350_e528 * assign350_e528)),)
    } else {
        (var_argbvvt, var_argbvvt_dn2,)
    }
};
        var_argbvvt = assign350_e531;
        var_argbvvt_dn2 = assign350_e531_d_n2;

        let assign360_e534: f64 = if var_arg > 80.0 { 1.0 } else { 0.0 };
        var_guard4 = assign360_e534;

        let (assign370_e544, assign370_e544_d_n2, assign370_e544_d_n3, assign370_e544_d_n4,) = {
    if ((var_guard3 != 0.0) && (var_guard4 != 0.0)) {
        let assign370_e541: f64 = (var_arg - 80.0);
        let assign370_e542: f64 = (1.0 + assign370_e541);
        (assign370_e542, var_arg_dn2, var_arg_dn3, var_arg_dn4,)
    } else {
        (var_le, var_le_dn2, var_le_dn3, var_le_dn4,)
    }
};
        var_le = assign370_e544;
        var_le_dn2 = assign370_e544_d_n2;
        var_le_dn3 = assign370_e544_d_n3;
        var_le_dn4 = assign370_e544_d_n4;

        let (assign380_e550, assign380_e550_d_n2, assign380_e550_d_n3, assign380_e550_d_n4,) = {
    if ((var_guard3 != 0.0) && (var_guard4 != 0.0)) {
        (80.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn2, var_arg_dn3, var_arg_dn4,)
    }
};
        var_arg = assign380_e550;
        var_arg_dn2 = assign380_e550_d_n2;
        var_arg_dn3 = assign380_e550_d_n3;
        var_arg_dn4 = assign380_e550_d_n4;

        let (assign390_e557, assign390_e557_d_n2, assign390_e557_d_n3, assign390_e557_d_n4,) = {
    if ((var_guard3 != 0.0) && (var_guard4 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0,)
    } else {
        (var_le, var_le_dn2, var_le_dn3, var_le_dn4,)
    }
};
        var_le = assign390_e557;
        var_le_dn2 = assign390_e557_d_n2;
        var_le_dn3 = assign390_e557_d_n3;
        var_le_dn4 = assign390_e557_d_n4;

        let (assign400_e564, assign400_e564_d_n2, assign400_e564_d_n3, assign400_e564_d_n4,) = {
    if (var_guard3 != 0.0) {
        let assign400_e561: f64 = (var_arg).exp();
        let assign400_e562: f64 = (var_le * assign400_e561);
        (assign400_e562, ((var_le_dn2 * assign400_e561) + (var_le * (assign400_e561 * var_arg_dn2))), ((var_le_dn3 * assign400_e561) + (var_le * (assign400_e561 * var_arg_dn3))), ((var_le_dn4 * assign400_e561) + (var_le * (assign400_e561 * var_arg_dn4))),)
    } else {
        (var_le, var_le_dn2, var_le_dn3, var_le_dn4,)
    }
};
        var_le = assign400_e564;
        var_le_dn2 = assign400_e564_d_n2;
        var_le_dn3 = assign400_e564_d_n3;
        var_le_dn4 = assign400_e564_d_n4;

        let (assign410_e636, assign410_e636_d_n2, assign410_e636_d_n3, assign410_e636_d_n4,) = {
    if (var_guard3 != 0.0) {
        let assign410_e572: f64 = (-37.0);
        let (assign410_e599, assign410_e599_d_n2, assign410_e599_d_n3, assign410_e599_d_n4,) = {
            if ((!(var_argbv >= 37.0)) && (!(var_argbv <= assign410_e572))) {
                let assign410_e577: f64 = (var_argbv).exp();
                let assign410_e579: f64 = (assign410_e577 + 1.0);
                let assign410_e580: f64 = (assign410_e579).ln();
                (assign410_e580, ((assign410_e577 * var_argbv_dn2) / assign410_e579), ((assign410_e577 * var_argbv_dn3) / assign410_e579), ((assign410_e577 * var_argbv_dn4) / assign410_e579),)
            } else {
                let assign410_e587: f64 = (-37.0);
                let (assign410_e598, assign410_e598_d_n2, assign410_e598_d_n3, assign410_e598_d_n4,) = {
                    if ((!(var_argbv >= 37.0)) && (var_argbv <= assign410_e587)) {
                        let assign410_e591: f64 = (var_argbv).exp();
                        (assign410_e591, (assign410_e591 * var_argbv_dn2), (assign410_e591 * var_argbv_dn3), (assign410_e591 * var_argbv_dn4),)
                    } else {
                        let (assign410_e597, assign410_e597_d_n2, assign410_e597_d_n3, assign410_e597_d_n4,) = {
                            if (var_argbv >= 37.0) {
                                (var_argbv, var_argbv_dn2, var_argbv_dn3, var_argbv_dn4,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign410_e597, assign410_e597_d_n2, assign410_e597_d_n3, assign410_e597_d_n4,)
                    }
                };
                (assign410_e598, assign410_e598_d_n2, assign410_e598_d_n3, assign410_e598_d_n4,)
            }
        };
        let assign410_e606: f64 = (-37.0);
        let (assign410_e633, assign410_e633_d_n2,) = {
            if ((!(var_argbvvt >= 37.0)) && (!(var_argbvvt <= assign410_e606))) {
                let assign410_e611: f64 = (var_argbvvt).exp();
                let assign410_e613: f64 = (assign410_e611 + 1.0);
                let assign410_e614: f64 = (assign410_e613).ln();
                (assign410_e614, ((assign410_e611 * var_argbvvt_dn2) / assign410_e613),)
            } else {
                let assign410_e621: f64 = (-37.0);
                let (assign410_e632, assign410_e632_d_n2,) = {
                    if ((!(var_argbvvt >= 37.0)) && (var_argbvvt <= assign410_e621)) {
                        let assign410_e625: f64 = (var_argbvvt).exp();
                        (assign410_e625, (assign410_e625 * var_argbvvt_dn2),)
                    } else {
                        let (assign410_e631, assign410_e631_d_n2,) = {
                            if (var_argbvvt >= 37.0) {
                                (var_argbvvt, var_argbvvt_dn2,)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign410_e631, assign410_e631_d_n2,)
                    }
                };
                (assign410_e632, assign410_e632_d_n2,)
            }
        };
        let assign410_e634: f64 = (assign410_e599 - assign410_e633);
        (assign410_e634, (assign410_e599_d_n2 - assign410_e633_d_n2), assign410_e599_d_n3, assign410_e599_d_n4,)
    } else {
        (var_lebv, var_lebv_dn2, var_lebv_dn3, var_lebv_dn4,)
    }
};
        var_lebv = assign410_e636;
        var_lebv_dn2 = assign410_e636_d_n2;
        var_lebv_dn3 = assign410_e636_d_n3;
        var_lebv_dn4 = assign410_e636_d_n4;

        let (assign420_e657, assign420_e657_d_n2, assign420_e657_d_n3, assign420_e657_d_n4,) = {
    if (var_guard3 != 0.0) {
        let assign420_e641: f64 = (var_le - 1.0);
        let assign420_e642: f64 = (var_is_t * assign420_e641);
        let assign420_e645: f64 = (var_ijbv_t * var_lebv);
        let assign420_e649: f64 = (var_vbiei).abs();
        let assign420_e651: f64 = (assign420_e649).powf(var_theexp_t);
        let assign420_e652: f64 = (p.p8 * assign420_e651);
        let assign420_e653: f64 = (1.0 + assign420_e652);
        let assign420_e654: f64 = (assign420_e645 / assign420_e653);
        let assign420_e655: f64 = (assign420_e642 - assign420_e654);
        (assign420_e655, (((var_is_t_dn2 * assign420_e641) + (var_is_t * var_le_dn2)) - (((((var_ijbv_t_dn2 * var_lebv) + (var_ijbv_t * var_lebv_dn2)) * assign420_e653) - (assign420_e645 * (p.p8 * if var_theexp_t_dn2 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { 0.0 } else { (assign420_e651 * (var_theexp_t_dn2 * (assign420_e649).ln())) }))) / (assign420_e653 * assign420_e653))), ((var_is_t * var_le_dn3) - ((((var_ijbv_t * var_lebv_dn3) * assign420_e653) - (assign420_e645 * (p.p8 * if 0.0 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign420_e649).powf(var_theexp_t - 1.0) * if var_vbiei >= 0.0 { var_vbiei_dn3 } else { (-var_vbiei_dn3) })) } } else { (assign420_e651 * (var_theexp_t * (if var_vbiei >= 0.0 { var_vbiei_dn3 } else { (-var_vbiei_dn3) } / assign420_e649))) }))) / (assign420_e653 * assign420_e653))), ((var_is_t * var_le_dn4) - ((((var_ijbv_t * var_lebv_dn4) * assign420_e653) - (assign420_e645 * (p.p8 * if 0.0 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign420_e649).powf(var_theexp_t - 1.0) * if var_vbiei >= 0.0 { var_vbiei_dn4 } else { (-var_vbiei_dn4) })) } } else { (assign420_e651 * (var_theexp_t * (if var_vbiei >= 0.0 { var_vbiei_dn4 } else { (-var_vbiei_dn4) } / assign420_e649))) }))) / (assign420_e653 * assign420_e653))),)
    } else {
        (var_ifwd, var_ifwd_dn2, var_ifwd_dn3, var_ifwd_dn4,)
    }
};
        var_ifwd = assign420_e657;
        var_ifwd_dn2 = assign420_e657_d_n2;
        var_ifwd_dn3 = assign420_e657_d_n3;
        var_ifwd_dn4 = assign420_e657_d_n4;

        let (assign430_e662, assign430_e662_d_n2, assign430_e662_d_n3, assign430_e662_d_n4,) = {
    if (var_guard3 == 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ifwd, var_ifwd_dn2, var_ifwd_dn3, var_ifwd_dn4,)
    }
};
        var_ifwd = assign430_e662;
        var_ifwd_dn2 = assign430_e662_d_n2;
        var_ifwd_dn3 = assign430_e662_d_n3;
        var_ifwd_dn4 = assign430_e662_d_n4;

        let assign440_e665: f64 = if var_isr_t > 0.0 { 1.0 } else { 0.0 };
        var_guard5 = assign440_e665;

        let (assign450_e673, assign450_e673_d_n3, assign450_e673_d_n4,) = {
    if (var_guard5 != 0.0) {
        let assign450_e669: f64 = (p.p4 - var_vbiei);
        let assign450_e671: f64 = (assign450_e669).max(0.001);
        (assign450_e671, if assign450_e669 >= 0.001 { (-var_vbiei_dn3) } else { 0.0 }, if assign450_e669 >= 0.001 { (-var_vbiei_dn4) } else { 0.0 },)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4,)
    }
};
        var_t0 = assign450_e673;
        var_t0_dn3 = assign450_e673_d_n3;
        var_t0_dn4 = assign450_e673_d_n4;

        let (assign460_e688, assign460_e688_d_n2, assign460_e688_d_n3, assign460_e688_d_n4,) = {
    if (var_guard5 != 0.0) {
        let assign460_e676: f64 = (-1.0);
        let assign460_e678: f64 = (assign460_e676 * var_vbiei);
        let assign460_e680: f64 = (assign460_e678 * p.p4);
        let assign460_e683: f64 = (p.p3 * var_vt);
        let assign460_e685: f64 = (assign460_e683 * var_t0);
        let assign460_e686: f64 = (assign460_e680 / assign460_e685);
        (assign460_e686, (-((assign460_e680 * ((p.p3 * var_vt_dn2) * var_t0)) / (assign460_e685 * assign460_e685))), (((((assign460_e676 * var_vbiei_dn3) * p.p4) * assign460_e685) - (assign460_e680 * (assign460_e683 * var_t0_dn3))) / (assign460_e685 * assign460_e685)), (((((assign460_e676 * var_vbiei_dn4) * p.p4) * assign460_e685) - (assign460_e680 * (assign460_e683 * var_t0_dn4))) / (assign460_e685 * assign460_e685)),)
    } else {
        (var_arg, var_arg_dn2, var_arg_dn3, var_arg_dn4,)
    }
};
        var_arg = assign460_e688;
        var_arg_dn2 = assign460_e688_d_n2;
        var_arg_dn3 = assign460_e688_d_n3;
        var_arg_dn4 = assign460_e688_d_n4;

        let assign470_e691: f64 = if var_arg > 80.0 { 1.0 } else { 0.0 };
        var_guard6 = assign470_e691;

        let (assign480_e701, assign480_e701_d_n2, assign480_e701_d_n3, assign480_e701_d_n4,) = {
    if ((var_guard5 != 0.0) && (var_guard6 != 0.0)) {
        let assign480_e698: f64 = (var_arg - 80.0);
        let assign480_e699: f64 = (1.0 + assign480_e698);
        (assign480_e699, var_arg_dn2, var_arg_dn3, var_arg_dn4,)
    } else {
        (var_le, var_le_dn2, var_le_dn3, var_le_dn4,)
    }
};
        var_le = assign480_e701;
        var_le_dn2 = assign480_e701_d_n2;
        var_le_dn3 = assign480_e701_d_n3;
        var_le_dn4 = assign480_e701_d_n4;

        let (assign490_e707, assign490_e707_d_n2, assign490_e707_d_n3, assign490_e707_d_n4,) = {
    if ((var_guard5 != 0.0) && (var_guard6 != 0.0)) {
        (80.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn2, var_arg_dn3, var_arg_dn4,)
    }
};
        var_arg = assign490_e707;
        var_arg_dn2 = assign490_e707_d_n2;
        var_arg_dn3 = assign490_e707_d_n3;
        var_arg_dn4 = assign490_e707_d_n4;

        let (assign500_e714, assign500_e714_d_n2, assign500_e714_d_n3, assign500_e714_d_n4,) = {
    if ((var_guard5 != 0.0) && (var_guard6 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0,)
    } else {
        (var_le, var_le_dn2, var_le_dn3, var_le_dn4,)
    }
};
        var_le = assign500_e714;
        var_le_dn2 = assign500_e714_d_n2;
        var_le_dn3 = assign500_e714_d_n3;
        var_le_dn4 = assign500_e714_d_n4;

        let (assign510_e721, assign510_e721_d_n2, assign510_e721_d_n3, assign510_e721_d_n4,) = {
    if (var_guard5 != 0.0) {
        let assign510_e718: f64 = (var_arg).exp();
        let assign510_e719: f64 = (var_le * assign510_e718);
        (assign510_e719, ((var_le_dn2 * assign510_e718) + (var_le * (assign510_e718 * var_arg_dn2))), ((var_le_dn3 * assign510_e718) + (var_le * (assign510_e718 * var_arg_dn3))), ((var_le_dn4 * assign510_e718) + (var_le * (assign510_e718 * var_arg_dn4))),)
    } else {
        (var_le, var_le_dn2, var_le_dn3, var_le_dn4,)
    }
};
        var_le = assign510_e721;
        var_le_dn2 = assign510_e721_d_n2;
        var_le_dn3 = assign510_e721_d_n3;
        var_le_dn4 = assign510_e721_d_n4;

        let (assign520_e729, assign520_e729_d_n2, assign520_e729_d_n3, assign520_e729_d_n4,) = {
    if (var_guard5 != 0.0) {
        let assign520_e726: f64 = (var_le - 1.0);
        let assign520_e727: f64 = (var_isr_t * assign520_e726);
        (assign520_e727, ((var_isr_t_dn2 * assign520_e726) + (var_isr_t * var_le_dn2)), (var_isr_t * var_le_dn3), (var_isr_t * var_le_dn4),)
    } else {
        (var_itrev, var_itrev_dn2, var_itrev_dn3, var_itrev_dn4,)
    }
};
        var_itrev = assign520_e729;
        var_itrev_dn2 = assign520_e729_d_n2;
        var_itrev_dn3 = assign520_e729_d_n3;
        var_itrev_dn4 = assign520_e729_d_n4;

        let (assign530_e734, assign530_e734_d_n2, assign530_e734_d_n3, assign530_e734_d_n4,) = {
    if (var_guard5 == 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itrev, var_itrev_dn2, var_itrev_dn3, var_itrev_dn4,)
    }
};
        var_itrev = assign530_e734;
        var_itrev_dn2 = assign530_e734_d_n2;
        var_itrev_dn3 = assign530_e734_d_n3;
        var_itrev_dn4 = assign530_e734_d_n4;

        *var_arg_slot = var_arg;
        *var_arg0_slot = var_arg0;
        *var_arg0_dn2_slot = var_arg0_dn2;
        *var_arg_dn2_slot = var_arg_dn2;
        *var_arg_dn3_slot = var_arg_dn3;
        *var_arg_dn4_slot = var_arg_dn4;
        *var_argbv_slot = var_argbv;
        *var_argbv_dn2_slot = var_argbv_dn2;
        *var_argbv_dn3_slot = var_argbv_dn3;
        *var_argbv_dn4_slot = var_argbv_dn4;
        *var_argbvvt_slot = var_argbvvt;
        *var_argbvvt_dn2_slot = var_argbvvt_dn2;
        *var_argt_slot = var_argt;
        *var_argt_dn2_slot = var_argt_dn2;
        *var_argtr_slot = var_argtr;
        *var_argtr_dn2_slot = var_argtr_dn2;
        *var_bvr_t_slot = var_bvr_t;
        *var_bvr_t_dn2_slot = var_bvr_t_dn2;
        *var_cje_i_slot = var_cje_i;
        *var_cje_t_slot = var_cje_t;
        *var_cje_t_dn2_slot = var_cje_t_dn2;
        *var_cjt_slot = var_cjt;
        *var_cjt_dn2_slot = var_cjt_dn2;
        *var_egfet_slot = var_egfet;
        *var_egfet_dn2_slot = var_egfet_dn2;
        *var_fact1_slot = var_fact1;
        *var_fact2_slot = var_fact2;
        *var_fact2_dn2_slot = var_fact2_dn2;
        *var_gmanew_slot = var_gmanew;
        *var_gmanew_dn2_slot = var_gmanew_dn2;
        *var_gmaold_slot = var_gmaold;
        *var_gmaold_dn2_slot = var_gmaold_dn2;
        *var_guard3_slot = var_guard3;
        *var_guard4_slot = var_guard4;
        *var_guard5_slot = var_guard5;
        *var_guard6_slot = var_guard6;
        *var_ifwd_slot = var_ifwd;
        *var_ifwd_dn2_slot = var_ifwd_dn2;
        *var_ifwd_dn3_slot = var_ifwd_dn3;
        *var_ifwd_dn4_slot = var_ifwd_dn4;
        *var_ijbv_t_slot = var_ijbv_t;
        *var_ijbv_t_dn2_slot = var_ijbv_t_dn2;
        *var_is_t_slot = var_is_t;
        *var_is_t_dn2_slot = var_is_t_dn2;
        *var_isr_t_slot = var_isr_t;
        *var_isr_t_dn2_slot = var_isr_t_dn2;
        *var_itrev_slot = var_itrev;
        *var_itrev_dn2_slot = var_itrev_dn2;
        *var_itrev_dn3_slot = var_itrev_dn3;
        *var_itrev_dn4_slot = var_itrev_dn4;
        *var_le_slot = var_le;
        *var_le_dn2_slot = var_le_dn2;
        *var_le_dn3_slot = var_le_dn3;
        *var_le_dn4_slot = var_le_dn4;
        *var_lebv_slot = var_lebv;
        *var_lebv_dn2_slot = var_lebv_dn2;
        *var_lebv_dn3_slot = var_lebv_dn3;
        *var_lebv_dn4_slot = var_lebv_dn4;
        *var_lnrt_slot = var_lnrt;
        *var_lnrt_dn2_slot = var_lnrt_dn2;
        *var_pbfact_slot = var_pbfact;
        *var_pbfact_dn2_slot = var_pbfact_dn2;
        *var_pbo_slot = var_pbo;
        *var_pbo_dn2_slot = var_pbo_dn2;
        *var_rt_slot = var_rt;
        *var_rt_dn2_slot = var_rt_dn2;
        *var_t0_slot = var_t0;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_tamb_slot = var_tamb;
        *var_tamb_dn2_slot = var_tamb_dn2;
        *var_tdev_slot = var_tdev;
        *var_tdev_dn2_slot = var_tdev_dn2;
        *var_theexp_t_slot = var_theexp_t;
        *var_theexp_t_dn2_slot = var_theexp_t_dn2;
        *var_tnom_slot = var_tnom;
        *var_ttype_slot = var_ttype;
        *var_vbbi_slot = var_vbbi;
        *var_vbbi_dn0_slot = var_vbbi_dn0;
        *var_vbbi_dn3_slot = var_vbbi_dn3;
        *var_vbiei_slot = var_vbiei;
        *var_vbiei_dn3_slot = var_vbiei_dn3;
        *var_vbiei_dn4_slot = var_vbiei_dn4;
        *var_veei_slot = var_veei;
        *var_veei_dn1_slot = var_veei_dn1;
        *var_veei_dn4_slot = var_veei_dn4;
        *var_vje_t_slot = var_vje_t;
        *var_vje_t_dn2_slot = var_vje_t_dn2;
        *var_vt_slot = var_vt;
        *var_vt_dn2_slot = var_vt_dn2;
        *var_weff_slot = var_weff;
    }

    pub(super) fn stamp_transient_block_1(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_cje_t: f64,
        var_cje_t_dn2: f64,
        var_ifwd: f64,
        var_ifwd_dn2: f64,
        var_ifwd_dn3: f64,
        var_ifwd_dn4: f64,
        var_itrev: f64,
        var_itrev_dn2: f64,
        var_itrev_dn3: f64,
        var_itrev_dn4: f64,
        var_lnrt: f64,
        var_lnrt_dn2: f64,
        var_vbbi: f64,
        var_vbbi_dn0: f64,
        var_vbbi_dn3: f64,
        var_vbiei: f64,
        var_vbiei_dn3: f64,
        var_vbiei_dn4: f64,
        var_veei: f64,
        var_veei_dn1: f64,
        var_veei_dn4: f64,
        var_vje_t: f64,
        var_vje_t_dn2: f64,
        var_weff: f64,
        var_dv0_slot: &mut f64,
        var_dv0_dn2_slot: &mut f64,
        var_dvh_slot: &mut f64,
        var_dvh_dn2_slot: &mut f64,
        var_dvh_dn3_slot: &mut f64,
        var_dvh_dn4_slot: &mut f64,
        var_guard10_slot: &mut f64,
        var_guard11_slot: &mut f64,
        var_guard12_slot: &mut f64,
        var_guard13_slot: &mut f64,
        var_guard14_slot: &mut f64,
        var_guard7_slot: &mut f64,
        var_guard8_slot: &mut f64,
        var_guard9_slot: &mut f64,
        var_ibe_slot: &mut f64,
        var_ibe_dn2_slot: &mut f64,
        var_ibe_dn3_slot: &mut f64,
        var_ibe_dn4_slot: &mut f64,
        var_itzf_slot: &mut f64,
        var_itzf_dn2_slot: &mut f64,
        var_itzf_dn3_slot: &mut f64,
        var_itzf_dn4_slot: &mut f64,
        var_pwq_slot: &mut f64,
        var_qde_slot: &mut f64,
        var_qde_dn0_slot: &mut f64,
        var_qde_dn1_slot: &mut f64,
        var_qde_dn2_slot: &mut f64,
        var_qde_dn3_slot: &mut f64,
        var_qde_dn4_slot: &mut f64,
        var_qhi_slot: &mut f64,
        var_qhi_dn2_slot: &mut f64,
        var_qhi_dn3_slot: &mut f64,
        var_qhi_dn4_slot: &mut f64,
        var_qje_slot: &mut f64,
        var_qje_dn2_slot: &mut f64,
        var_qje_dn3_slot: &mut f64,
        var_qje_dn4_slot: &mut f64,
        var_qlo_slot: &mut f64,
        var_qlo_dn2_slot: &mut f64,
        var_qlo_dn3_slot: &mut f64,
        var_qlo_dn4_slot: &mut f64,
        var_rb_slot: &mut f64,
        var_rb_dn0_slot: &mut f64,
        var_rb_dn2_slot: &mut f64,
        var_rb_dn3_slot: &mut f64,
        var_rb_dn6_slot: &mut f64,
        var_rb_nom_slot: &mut f64,
        var_re_slot: &mut f64,
        var_re_dn1_slot: &mut f64,
        var_re_dn2_slot: &mut f64,
        var_re_dn4_slot: &mut f64,
        var_re_nom_slot: &mut f64,
        var_tff_slot: &mut f64,
        var_tff_dn0_slot: &mut f64,
        var_tff_dn1_slot: &mut f64,
        var_vbesat_slot: &mut f64,
        var_vbesat_dn0_slot: &mut f64,
        var_vbesat_dn3_slot: &mut f64,
        var_veesat_slot: &mut f64,
        var_veesat_dn1_slot: &mut f64,
        var_veesat_dn4_slot: &mut f64,
        var_vtff_slot: &mut f64,
        var_vtff1_slot: &mut f64,
        var_vtff1_dn0_slot: &mut f64,
        var_vtff1_dn1_slot: &mut f64,
        var_vtff_dn0_slot: &mut f64,
        var_vtff_dn1_slot: &mut f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let mut var_dv0: f64 = *var_dv0_slot;
        let mut var_dv0_dn2: f64 = *var_dv0_dn2_slot;
        let mut var_dvh: f64 = *var_dvh_slot;
        let mut var_dvh_dn2: f64 = *var_dvh_dn2_slot;
        let mut var_dvh_dn3: f64 = *var_dvh_dn3_slot;
        let mut var_dvh_dn4: f64 = *var_dvh_dn4_slot;
        let mut var_guard10: f64 = *var_guard10_slot;
        let mut var_guard11: f64 = *var_guard11_slot;
        let mut var_guard12: f64 = *var_guard12_slot;
        let mut var_guard13: f64 = *var_guard13_slot;
        let mut var_guard14: f64 = *var_guard14_slot;
        let mut var_guard7: f64 = *var_guard7_slot;
        let mut var_guard8: f64 = *var_guard8_slot;
        let mut var_guard9: f64 = *var_guard9_slot;
        let mut var_ibe: f64 = *var_ibe_slot;
        let mut var_ibe_dn2: f64 = *var_ibe_dn2_slot;
        let mut var_ibe_dn3: f64 = *var_ibe_dn3_slot;
        let mut var_ibe_dn4: f64 = *var_ibe_dn4_slot;
        let mut var_itzf: f64 = *var_itzf_slot;
        let mut var_itzf_dn2: f64 = *var_itzf_dn2_slot;
        let mut var_itzf_dn3: f64 = *var_itzf_dn3_slot;
        let mut var_itzf_dn4: f64 = *var_itzf_dn4_slot;
        let mut var_pwq: f64 = *var_pwq_slot;
        let mut var_qde: f64 = *var_qde_slot;
        let mut var_qde_dn0: f64 = *var_qde_dn0_slot;
        let mut var_qde_dn1: f64 = *var_qde_dn1_slot;
        let mut var_qde_dn2: f64 = *var_qde_dn2_slot;
        let mut var_qde_dn3: f64 = *var_qde_dn3_slot;
        let mut var_qde_dn4: f64 = *var_qde_dn4_slot;
        let mut var_qhi: f64 = *var_qhi_slot;
        let mut var_qhi_dn2: f64 = *var_qhi_dn2_slot;
        let mut var_qhi_dn3: f64 = *var_qhi_dn3_slot;
        let mut var_qhi_dn4: f64 = *var_qhi_dn4_slot;
        let mut var_qje: f64 = *var_qje_slot;
        let mut var_qje_dn2: f64 = *var_qje_dn2_slot;
        let mut var_qje_dn3: f64 = *var_qje_dn3_slot;
        let mut var_qje_dn4: f64 = *var_qje_dn4_slot;
        let mut var_qlo: f64 = *var_qlo_slot;
        let mut var_qlo_dn2: f64 = *var_qlo_dn2_slot;
        let mut var_qlo_dn3: f64 = *var_qlo_dn3_slot;
        let mut var_qlo_dn4: f64 = *var_qlo_dn4_slot;
        let mut var_rb: f64 = *var_rb_slot;
        let mut var_rb_dn0: f64 = *var_rb_dn0_slot;
        let mut var_rb_dn2: f64 = *var_rb_dn2_slot;
        let mut var_rb_dn3: f64 = *var_rb_dn3_slot;
        let mut var_rb_dn6: f64 = *var_rb_dn6_slot;
        let mut var_rb_nom: f64 = *var_rb_nom_slot;
        let mut var_re: f64 = *var_re_slot;
        let mut var_re_dn1: f64 = *var_re_dn1_slot;
        let mut var_re_dn2: f64 = *var_re_dn2_slot;
        let mut var_re_dn4: f64 = *var_re_dn4_slot;
        let mut var_re_nom: f64 = *var_re_nom_slot;
        let mut var_tff: f64 = *var_tff_slot;
        let mut var_tff_dn0: f64 = *var_tff_dn0_slot;
        let mut var_tff_dn1: f64 = *var_tff_dn1_slot;
        let mut var_vbesat: f64 = *var_vbesat_slot;
        let mut var_vbesat_dn0: f64 = *var_vbesat_dn0_slot;
        let mut var_vbesat_dn3: f64 = *var_vbesat_dn3_slot;
        let mut var_veesat: f64 = *var_veesat_slot;
        let mut var_veesat_dn1: f64 = *var_veesat_dn1_slot;
        let mut var_veesat_dn4: f64 = *var_veesat_dn4_slot;
        let mut var_vtff: f64 = *var_vtff_slot;
        let mut var_vtff1: f64 = *var_vtff1_slot;
        let mut var_vtff1_dn0: f64 = *var_vtff1_dn0_slot;
        let mut var_vtff1_dn1: f64 = *var_vtff1_dn1_slot;
        let mut var_vtff_dn0: f64 = *var_vtff_dn0_slot;
        let mut var_vtff_dn1: f64 = *var_vtff_dn1_slot;

        let assign540_e737: f64 = (var_ifwd - var_itrev);
        var_ibe = assign540_e737;
        var_ibe_dn2 = (var_ifwd_dn2 - var_itrev_dn2);
        var_ibe_dn3 = (var_ifwd_dn3 - var_itrev_dn3);
        var_ibe_dn4 = (var_ifwd_dn4 - var_itrev_dn4);

        let assign550_e741: f64 = (var_vbbi / p.p48);
        let assign550_e742: f64 = (assign550_e741).abs();
        let assign550_e744: f64 = (assign550_e742).powf(p.p49);
        let assign550_e745: f64 = (1.0 + assign550_e744);
        var_vbesat = assign550_e745;
        var_vbesat_dn0 = if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((assign550_e742).powf(p.p49 - 1.0) * if assign550_e741 >= 0.0 { (var_vbbi_dn0 / p.p48) } else { (-(var_vbbi_dn0 / p.p48)) })) } } else { (assign550_e744 * (p.p49 * (if assign550_e741 >= 0.0 { (var_vbbi_dn0 / p.p48) } else { (-(var_vbbi_dn0 / p.p48)) } / assign550_e742))) };
        var_vbesat_dn3 = if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((assign550_e742).powf(p.p49 - 1.0) * if assign550_e741 >= 0.0 { (var_vbbi_dn3 / p.p48) } else { (-(var_vbbi_dn3 / p.p48)) })) } } else { (assign550_e744 * (p.p49 * (if assign550_e741 >= 0.0 { (var_vbbi_dn3 / p.p48) } else { (-(var_vbbi_dn3 / p.p48)) } / assign550_e742))) };

        let assign560_e749: f64 = (var_veei / p.p50);
        let assign560_e750: f64 = (assign560_e749).abs();
        let assign560_e752: f64 = (assign560_e750).powf(p.p51);
        let assign560_e753: f64 = (1.0 + assign560_e752);
        var_veesat = assign560_e753;
        var_veesat_dn1 = if 0.0 == 0.0 && ((p.p51) as f64).is_finite() && ((p.p51) as f64).fract() == 0.0 { if p.p51 == 0.0 { 0.0 } else { (p.p51 * ((assign560_e750).powf(p.p51 - 1.0) * if assign560_e749 >= 0.0 { (var_veei_dn1 / p.p50) } else { (-(var_veei_dn1 / p.p50)) })) } } else { (assign560_e752 * (p.p51 * (if assign560_e749 >= 0.0 { (var_veei_dn1 / p.p50) } else { (-(var_veei_dn1 / p.p50)) } / assign560_e750))) };
        var_veesat_dn4 = if 0.0 == 0.0 && ((p.p51) as f64).is_finite() && ((p.p51) as f64).fract() == 0.0 { if p.p51 == 0.0 { 0.0 } else { (p.p51 * ((assign560_e750).powf(p.p51 - 1.0) * if assign560_e749 >= 0.0 { (var_veei_dn4 / p.p50) } else { (-(var_veei_dn4 / p.p50)) })) } } else { (assign560_e752 * (p.p51 * (if assign560_e749 >= 0.0 { (var_veei_dn4 / p.p50) } else { (-(var_veei_dn4 / p.p50)) } / assign560_e750))) };

        let assign570_e757: f64 = (var_lnrt * p.p37);
        let assign570_e758: f64 = (assign570_e757).exp();
        let assign570_e759: f64 = (p.p12 * assign570_e758);
        let assign570_e763: f64 = (1.0 / p.p49);
        let assign570_e764: f64 = (var_vbesat).powf(assign570_e763);
        let assign570_e765: f64 = (assign570_e759 * assign570_e764);
        var_rb = assign570_e765;
        var_rb_dn0 = (assign570_e759 * if 0.0 == 0.0 && ((assign570_e763) as f64).is_finite() && ((assign570_e763) as f64).fract() == 0.0 { if assign570_e763 == 0.0 { 0.0 } else { (assign570_e763 * ((var_vbesat).powf(assign570_e763 - 1.0) * var_vbesat_dn0)) } } else { (assign570_e764 * (assign570_e763 * (var_vbesat_dn0 / var_vbesat))) });
        var_rb_dn2 = ((p.p12 * (assign570_e758 * (var_lnrt_dn2 * p.p37))) * assign570_e764);
        var_rb_dn3 = (assign570_e759 * if 0.0 == 0.0 && ((assign570_e763) as f64).is_finite() && ((assign570_e763) as f64).fract() == 0.0 { if assign570_e763 == 0.0 { 0.0 } else { (assign570_e763 * ((var_vbesat).powf(assign570_e763 - 1.0) * var_vbesat_dn3)) } } else { (assign570_e764 * (assign570_e763 * (var_vbesat_dn3 / var_vbesat))) });
        var_rb_dn6 = 0.0;

        let assign580_e769: f64 = (var_lnrt * p.p38);
        let assign580_e770: f64 = (assign580_e769).exp();
        let assign580_e771: f64 = (p.p14 * assign580_e770);
        let assign580_e775: f64 = (1.0 / p.p51);
        let assign580_e776: f64 = (var_veesat).powf(assign580_e775);
        let assign580_e777: f64 = (assign580_e771 * assign580_e776);
        var_re = assign580_e777;
        var_re_dn1 = (assign580_e771 * if 0.0 == 0.0 && ((assign580_e775) as f64).is_finite() && ((assign580_e775) as f64).fract() == 0.0 { if assign580_e775 == 0.0 { 0.0 } else { (assign580_e775 * ((var_veesat).powf(assign580_e775 - 1.0) * var_veesat_dn1)) } } else { (assign580_e776 * (assign580_e775 * (var_veesat_dn1 / var_veesat))) });
        var_re_dn2 = ((p.p14 * (assign580_e770 * (var_lnrt_dn2 * p.p38))) * assign580_e776);
        var_re_dn4 = (assign580_e771 * if 0.0 == 0.0 && ((assign580_e775) as f64).is_finite() && ((assign580_e775) as f64).fract() == 0.0 { if assign580_e775 == 0.0 { 0.0 } else { (assign580_e775 * ((var_veesat).powf(assign580_e775 - 1.0) * var_veesat_dn4)) } } else { (assign580_e776 * (assign580_e775 * (var_veesat_dn4 / var_veesat))) });

        let assign590_e780: f64 = if p.p31 == 1.0 { 1.0 } else { 0.0 };
        var_guard7 = assign590_e780;

        let (assign600_e786, assign600_e786_d_n0, assign600_e786_d_n2, assign600_e786_d_n3, assign600_e786_d_n6,) = {
    if (var_guard7 != 0.0) {
        let assign600_e784: f64 = (var_rb + p.p13);
        (assign600_e784, var_rb_dn0, var_rb_dn2, var_rb_dn3, var_rb_dn6,)
    } else {
        (var_rb, var_rb_dn0, var_rb_dn2, var_rb_dn3, var_rb_dn6,)
    }
};
        var_rb = assign600_e786;
        var_rb_dn0 = assign600_e786_d_n0;
        var_rb_dn2 = assign600_e786_d_n2;
        var_rb_dn3 = assign600_e786_d_n3;
        var_rb_dn6 = assign600_e786_d_n6;

        let (assign610_e792, assign610_e792_d_n1, assign610_e792_d_n2, assign610_e792_d_n4,) = {
    if (var_guard7 != 0.0) {
        let assign610_e790: f64 = (var_re + p.p15);
        (assign610_e790, var_re_dn1, var_re_dn2, var_re_dn4,)
    } else {
        (var_re, var_re_dn1, var_re_dn2, var_re_dn4,)
    }
};
        var_re = assign610_e792;
        var_re_dn1 = assign610_e792_d_n1;
        var_re_dn2 = assign610_e792_d_n2;
        var_re_dn4 = assign610_e792_d_n4;

        var_itzf = var_ifwd;
        var_itzf_dn2 = var_ifwd_dn2;
        var_itzf_dn3 = var_ifwd_dn3;
        var_itzf_dn4 = var_ifwd_dn4;

        let assign630_e796: f64 = ((nv0 - nv1) / p.p40);
        let assign630_e797: f64 = (assign630_e796).abs();
        let assign630_e799: f64 = (assign630_e797).powf(p.p39);
        var_vtff = assign630_e799;
        var_vtff_dn0 = if 0.0 == 0.0 && ((p.p39) as f64).is_finite() && ((p.p39) as f64).fract() == 0.0 { if p.p39 == 0.0 { 0.0 } else { (p.p39 * ((assign630_e797).powf(p.p39 - 1.0) * if assign630_e796 >= 0.0 { (1.0 / p.p40) } else { (-(1.0 / p.p40)) })) } } else { (assign630_e799 * (p.p39 * (if assign630_e796 >= 0.0 { (1.0 / p.p40) } else { (-(1.0 / p.p40)) } / assign630_e797))) };
        var_vtff_dn1 = if 0.0 == 0.0 && ((p.p39) as f64).is_finite() && ((p.p39) as f64).fract() == 0.0 { if p.p39 == 0.0 { 0.0 } else { (p.p39 * ((assign630_e797).powf(p.p39 - 1.0) * if assign630_e796 >= 0.0 { (-1.0 / p.p40) } else { (-(-1.0 / p.p40)) })) } } else { (assign630_e799 * (p.p39 * (if assign630_e796 >= 0.0 { (-1.0 / p.p40) } else { (-(-1.0 / p.p40)) } / assign630_e797))) };

        let assign640_e802: f64 = (1.0 + var_vtff);
        let assign640_e805: f64 = (1.0 / p.p39);
        let assign640_e806: f64 = (assign640_e802).powf(assign640_e805);
        let assign640_e808: f64 = (assign640_e806 - 1.0);
        var_vtff1 = assign640_e808;
        var_vtff1_dn0 = if 0.0 == 0.0 && ((assign640_e805) as f64).is_finite() && ((assign640_e805) as f64).fract() == 0.0 { if assign640_e805 == 0.0 { 0.0 } else { (assign640_e805 * ((assign640_e802).powf(assign640_e805 - 1.0) * var_vtff_dn0)) } } else { (assign640_e806 * (assign640_e805 * (var_vtff_dn0 / assign640_e802))) };
        var_vtff1_dn1 = if 0.0 == 0.0 && ((assign640_e805) as f64).is_finite() && ((assign640_e805) as f64).fract() == 0.0 { if assign640_e805 == 0.0 { 0.0 } else { (assign640_e805 * ((assign640_e802).powf(assign640_e805 - 1.0) * var_vtff_dn1)) } } else { (assign640_e806 * (assign640_e805 * (var_vtff_dn1 / assign640_e802))) };

        let assign650_e813: f64 = (p.p41 * var_vtff1);
        let assign650_e814: f64 = (1.0 + assign650_e813);
        let assign650_e815: f64 = (p.p19 * assign650_e814);
        var_tff = assign650_e815;
        var_tff_dn0 = (p.p19 * (p.p41 * var_vtff1_dn0));
        var_tff_dn1 = (p.p19 * (p.p41 * var_vtff1_dn1));

        let assign660_e818: f64 = (var_tff * var_itzf);
        var_qde = assign660_e818;
        var_qde_dn0 = (var_tff_dn0 * var_itzf);
        var_qde_dn1 = (var_tff_dn1 * var_itzf);
        var_qde_dn2 = (var_tff * var_itzf_dn2);
        var_qde_dn3 = (var_tff * var_itzf_dn3);
        var_qde_dn4 = (var_tff * var_itzf_dn4);

        let assign670_e821: f64 = if p.p32 == 1.0 { 1.0 } else { 0.0 };
        var_guard8 = assign670_e821;

        let (assign680_e834, assign680_e834_d_n0, assign680_e834_d_n2, assign680_e834_d_n3, assign680_e834_d_n6,) = {
    if (var_guard8 != 0.0) {
        let assign680_e826: f64 = ((nv6 - 0.0)).abs();
        let assign680_e828: f64 = (assign680_e826 / p.p20);
        let assign680_e830: f64 = (assign680_e828).powf(p.p44);
        let assign680_e831: f64 = (1.0 + assign680_e830);
        let assign680_e832: f64 = (var_rb / assign680_e831);
        (assign680_e832, (var_rb_dn0 / assign680_e831), (var_rb_dn2 / assign680_e831), (var_rb_dn3 / assign680_e831), (((var_rb_dn6 * assign680_e831) - (var_rb * if 0.0 == 0.0 && ((p.p44) as f64).is_finite() && ((p.p44) as f64).fract() == 0.0 { if p.p44 == 0.0 { 0.0 } else { (p.p44 * ((assign680_e828).powf(p.p44 - 1.0) * (if (nv6 - 0.0) >= 0.0 { 1.0 } else { (-1.0) } / p.p20))) } } else { (assign680_e830 * (p.p44 * ((if (nv6 - 0.0) >= 0.0 { 1.0 } else { (-1.0) } / p.p20) / assign680_e828))) })) / (assign680_e831 * assign680_e831)),)
    } else {
        (var_rb, var_rb_dn0, var_rb_dn2, var_rb_dn3, var_rb_dn6,)
    }
};
        var_rb = assign680_e834;
        var_rb_dn0 = assign680_e834_d_n0;
        var_rb_dn2 = assign680_e834_d_n2;
        var_rb_dn3 = assign680_e834_d_n3;
        var_rb_dn6 = assign680_e834_d_n6;

        let (assign690_e839, assign690_e839_d_n0, assign690_e839_d_n2, assign690_e839_d_n3, assign690_e839_d_n6,) = {
    if (var_guard8 == 0.0) {
        (var_rb, var_rb_dn0, var_rb_dn2, var_rb_dn3, var_rb_dn6,)
    } else {
        (var_rb, var_rb_dn0, var_rb_dn2, var_rb_dn3, var_rb_dn6,)
    }
};
        var_rb = assign690_e839;
        var_rb_dn0 = assign690_e839_d_n0;
        var_rb_dn2 = assign690_e839_d_n2;
        var_rb_dn3 = assign690_e839_d_n3;
        var_rb_dn6 = assign690_e839_d_n6;

        let assign700_e841: f64 = (-var_vje_t);
        let assign700_e843: f64 = (assign700_e841 * p.p24);
        var_dv0 = assign700_e843;
        var_dv0_dn2 = ((-var_vje_t_dn2) * p.p24);

        let assign710_e846: f64 = (var_vbiei + var_dv0);
        var_dvh = assign710_e846;
        var_dvh_dn2 = var_dv0_dn2;
        var_dvh_dn3 = var_vbiei_dn3;
        var_dvh_dn4 = var_vbiei_dn4;

        let assign720_e849: f64 = if var_dvh > 0.0 { 1.0 } else { 0.0 };
        var_guard9 = assign720_e849;

        let (assign730_e862,) = {
    if (var_guard9 != 0.0) {
        let assign730_e852: f64 = (-1.0);
        let assign730_e854: f64 = (assign730_e852 - p.p18);
        let assign730_e857: f64 = (1.0 - p.p24);
        let assign730_e858: f64 = (assign730_e857).ln();
        let assign730_e859: f64 = (assign730_e854 * assign730_e858);
        let assign730_e860: f64 = (assign730_e859).exp();
        (assign730_e860,)
    } else {
        (var_pwq,)
    }
};
        var_pwq = assign730_e862;

        let (assign740_e882, assign740_e882_d_n2, assign740_e882_d_n3, assign740_e882_d_n4,) = {
    if (var_guard9 != 0.0) {
        let assign740_e869: f64 = (1.0 - p.p24);
        let assign740_e870: f64 = (var_pwq * assign740_e869);
        let assign740_e873: f64 = (1.0 - p.p24);
        let assign740_e874: f64 = (assign740_e870 * assign740_e873);
        let assign740_e875: f64 = (1.0 - assign740_e874);
        let assign740_e876: f64 = (var_vje_t * assign740_e875);
        let assign740_e879: f64 = (1.0 - p.p18);
        let assign740_e880: f64 = (assign740_e876 / assign740_e879);
        (assign740_e880, ((var_vje_t_dn2 * assign740_e875) / assign740_e879), 0.0, 0.0,)
    } else {
        (var_qlo, var_qlo_dn2, var_qlo_dn3, var_qlo_dn4,)
    }
};
        var_qlo = assign740_e882;
        var_qlo_dn2 = assign740_e882_d_n2;
        var_qlo_dn3 = assign740_e882_d_n3;
        var_qlo_dn4 = assign740_e882_d_n4;

        let (assign750_e900, assign750_e900_d_n2, assign750_e900_d_n3, assign750_e900_d_n4,) = {
    if (var_guard9 != 0.0) {
        let assign750_e887: f64 = (1.0 - p.p24);
        let assign750_e890: f64 = (0.5 * p.p18);
        let assign750_e892: f64 = (assign750_e890 * var_dvh);
        let assign750_e894: f64 = (assign750_e892 / var_vje_t);
        let assign750_e895: f64 = (assign750_e887 + assign750_e894);
        let assign750_e896: f64 = (var_dvh * assign750_e895);
        let assign750_e898: f64 = (assign750_e896 * var_pwq);
        (assign750_e898, (((var_dvh_dn2 * assign750_e895) + (var_dvh * ((((assign750_e890 * var_dvh_dn2) * var_vje_t) - (assign750_e892 * var_vje_t_dn2)) / (var_vje_t * var_vje_t)))) * var_pwq), (((var_dvh_dn3 * assign750_e895) + (var_dvh * ((assign750_e890 * var_dvh_dn3) / var_vje_t))) * var_pwq), (((var_dvh_dn4 * assign750_e895) + (var_dvh * ((assign750_e890 * var_dvh_dn4) / var_vje_t))) * var_pwq),)
    } else {
        (var_qhi, var_qhi_dn2, var_qhi_dn3, var_qhi_dn4,)
    }
};
        var_qhi = assign750_e900;
        var_qhi_dn2 = assign750_e900_d_n2;
        var_qhi_dn3 = assign750_e900_d_n3;
        var_qhi_dn4 = assign750_e900_d_n4;

        let (assign760_e923, assign760_e923_d_n2, assign760_e923_d_n3, assign760_e923_d_n4,) = {
    if (var_guard9 == 0.0) {
        let assign760_e907: f64 = (1.0 - p.p18);
        let assign760_e911: f64 = (var_vbiei / var_vje_t);
        let assign760_e912: f64 = (1.0 - assign760_e911);
        let assign760_e913: f64 = (assign760_e912).ln();
        let assign760_e914: f64 = (assign760_e907 * assign760_e913);
        let assign760_e915: f64 = (assign760_e914).exp();
        let assign760_e916: f64 = (1.0 - assign760_e915);
        let assign760_e917: f64 = (var_vje_t * assign760_e916);
        let assign760_e920: f64 = (1.0 - p.p18);
        let assign760_e921: f64 = (assign760_e917 / assign760_e920);
        (assign760_e921, (((var_vje_t_dn2 * assign760_e916) + (var_vje_t * (-(assign760_e915 * (assign760_e907 * ((-(-((var_vbiei * var_vje_t_dn2) / (var_vje_t * var_vje_t)))) / assign760_e912)))))) / assign760_e920), ((var_vje_t * (-(assign760_e915 * (assign760_e907 * ((-(var_vbiei_dn3 / var_vje_t)) / assign760_e912))))) / assign760_e920), ((var_vje_t * (-(assign760_e915 * (assign760_e907 * ((-(var_vbiei_dn4 / var_vje_t)) / assign760_e912))))) / assign760_e920),)
    } else {
        (var_qlo, var_qlo_dn2, var_qlo_dn3, var_qlo_dn4,)
    }
};
        var_qlo = assign760_e923;
        var_qlo_dn2 = assign760_e923_d_n2;
        var_qlo_dn3 = assign760_e923_d_n3;
        var_qlo_dn4 = assign760_e923_d_n4;

        let (assign770_e928, assign770_e928_d_n2, assign770_e928_d_n3, assign770_e928_d_n4,) = {
    if (var_guard9 == 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qhi, var_qhi_dn2, var_qhi_dn3, var_qhi_dn4,)
    }
};
        var_qhi = assign770_e928;
        var_qhi_dn2 = assign770_e928_d_n2;
        var_qhi_dn3 = assign770_e928_d_n3;
        var_qhi_dn4 = assign770_e928_d_n4;

        let assign780_e932: f64 = (var_qlo + var_qhi);
        let assign780_e933: f64 = (var_cje_t * assign780_e932);
        var_qje = assign780_e933;
        var_qje_dn2 = ((var_cje_t_dn2 * assign780_e932) + (var_cje_t * (var_qlo_dn2 + var_qhi_dn2)));
        var_qje_dn3 = (var_cje_t * (var_qlo_dn3 + var_qhi_dn3));
        var_qje_dn4 = (var_cje_t * (var_qlo_dn4 + var_qhi_dn4));

        let assign790_e940: f64 = if ((p.p30 == 1.0) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };
        var_guard10 = assign790_e940;

        let assign800_e951: f64 = if (((p.p30 == 2.0) && (p.p33 > 0.0)) && (p.p35 > 0.0)) { 1.0 } else { 0.0 };
        var_guard11 = assign800_e951;

        let assign810_e954: f64 = (-1.0);
        let assign810_e955: f64 = if p.p30 == assign810_e954 { 1.0 } else { 0.0 };
        var_guard12 = assign810_e955;

        let assign830_e964: f64 = (p.p31 * p.p13);
        let assign830_e965: f64 = (p.p12 + assign830_e964);
        let assign830_e967: f64 = (assign830_e965 / var_weff);
        var_rb_nom = assign830_e967;

        let assign840_e971: f64 = (p.p31 * p.p15);
        let assign840_e972: f64 = (p.p14 + assign840_e971);
        let assign840_e974: f64 = (assign840_e972 / var_weff);
        var_re_nom = assign840_e974;

        let assign850_e981: f64 = if ((var_rb_nom > 0.0) && (var_rb_nom >= p.p46)) { 1.0 } else { 0.0 };
        var_guard13 = assign850_e981;

        let assign870_e1003: f64 = if ((var_re_nom > 0.0) && (var_re_nom >= p.p46)) { 1.0 } else { 0.0 };
        var_guard14 = assign870_e1003;

        *var_dv0_slot = var_dv0;
        *var_dv0_dn2_slot = var_dv0_dn2;
        *var_dvh_slot = var_dvh;
        *var_dvh_dn2_slot = var_dvh_dn2;
        *var_dvh_dn3_slot = var_dvh_dn3;
        *var_dvh_dn4_slot = var_dvh_dn4;
        *var_guard10_slot = var_guard10;
        *var_guard11_slot = var_guard11;
        *var_guard12_slot = var_guard12;
        *var_guard13_slot = var_guard13;
        *var_guard14_slot = var_guard14;
        *var_guard7_slot = var_guard7;
        *var_guard8_slot = var_guard8;
        *var_guard9_slot = var_guard9;
        *var_ibe_slot = var_ibe;
        *var_ibe_dn2_slot = var_ibe_dn2;
        *var_ibe_dn3_slot = var_ibe_dn3;
        *var_ibe_dn4_slot = var_ibe_dn4;
        *var_itzf_slot = var_itzf;
        *var_itzf_dn2_slot = var_itzf_dn2;
        *var_itzf_dn3_slot = var_itzf_dn3;
        *var_itzf_dn4_slot = var_itzf_dn4;
        *var_pwq_slot = var_pwq;
        *var_qde_slot = var_qde;
        *var_qde_dn0_slot = var_qde_dn0;
        *var_qde_dn1_slot = var_qde_dn1;
        *var_qde_dn2_slot = var_qde_dn2;
        *var_qde_dn3_slot = var_qde_dn3;
        *var_qde_dn4_slot = var_qde_dn4;
        *var_qhi_slot = var_qhi;
        *var_qhi_dn2_slot = var_qhi_dn2;
        *var_qhi_dn3_slot = var_qhi_dn3;
        *var_qhi_dn4_slot = var_qhi_dn4;
        *var_qje_slot = var_qje;
        *var_qje_dn2_slot = var_qje_dn2;
        *var_qje_dn3_slot = var_qje_dn3;
        *var_qje_dn4_slot = var_qje_dn4;
        *var_qlo_slot = var_qlo;
        *var_qlo_dn2_slot = var_qlo_dn2;
        *var_qlo_dn3_slot = var_qlo_dn3;
        *var_qlo_dn4_slot = var_qlo_dn4;
        *var_rb_slot = var_rb;
        *var_rb_dn0_slot = var_rb_dn0;
        *var_rb_dn2_slot = var_rb_dn2;
        *var_rb_dn3_slot = var_rb_dn3;
        *var_rb_dn6_slot = var_rb_dn6;
        *var_rb_nom_slot = var_rb_nom;
        *var_re_slot = var_re;
        *var_re_dn1_slot = var_re_dn1;
        *var_re_dn2_slot = var_re_dn2;
        *var_re_dn4_slot = var_re_dn4;
        *var_re_nom_slot = var_re_nom;
        *var_tff_slot = var_tff;
        *var_tff_dn0_slot = var_tff_dn0;
        *var_tff_dn1_slot = var_tff_dn1;
        *var_vbesat_slot = var_vbesat;
        *var_vbesat_dn0_slot = var_vbesat_dn0;
        *var_vbesat_dn3_slot = var_vbesat_dn3;
        *var_veesat_slot = var_veesat;
        *var_veesat_dn1_slot = var_veesat_dn1;
        *var_veesat_dn4_slot = var_veesat_dn4;
        *var_vtff_slot = var_vtff;
        *var_vtff1_slot = var_vtff1;
        *var_vtff1_dn0_slot = var_vtff1_dn0;
        *var_vtff1_dn1_slot = var_vtff1_dn1;
        *var_vtff_dn0_slot = var_vtff_dn0;
        *var_vtff_dn1_slot = var_vtff_dn1;
    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_argt_slot: &mut f64,
        var_argt_db0_slot: &mut f64,
        var_argt_db1_slot: &mut f64,
        var_argt_db2_slot: &mut f64,
        var_argt_db3_slot: &mut f64,
        var_argt_db4_slot: &mut f64,
        var_argt_db5_slot: &mut f64,
        var_argt_db6_slot: &mut f64,
        var_argt_dn0_slot: &mut f64,
        var_argt_dn1_slot: &mut f64,
        var_argt_dn2_slot: &mut f64,
        var_argt_dn3_slot: &mut f64,
        var_argt_dn4_slot: &mut f64,
        var_argt_dn5_slot: &mut f64,
        var_argt_dn6_slot: &mut f64,
        var_argt_rdb0_slot: &mut f64,
        var_argt_rdb1_slot: &mut f64,
        var_argt_rdb2_slot: &mut f64,
        var_argt_rdb3_slot: &mut f64,
        var_argt_rdb4_slot: &mut f64,
        var_argt_rdb5_slot: &mut f64,
        var_argt_rdb6_slot: &mut f64,
        var_argt_rdn0_slot: &mut f64,
        var_argt_rdn1_slot: &mut f64,
        var_argt_rdn2_slot: &mut f64,
        var_argt_rdn3_slot: &mut f64,
        var_argt_rdn4_slot: &mut f64,
        var_argt_rdn5_slot: &mut f64,
        var_argt_rdn6_slot: &mut f64,
        var_argt_rv_slot: &mut f64,
        var_argtr_slot: &mut f64,
        var_argtr_db0_slot: &mut f64,
        var_argtr_db1_slot: &mut f64,
        var_argtr_db2_slot: &mut f64,
        var_argtr_db3_slot: &mut f64,
        var_argtr_db4_slot: &mut f64,
        var_argtr_db5_slot: &mut f64,
        var_argtr_db6_slot: &mut f64,
        var_argtr_dn0_slot: &mut f64,
        var_argtr_dn1_slot: &mut f64,
        var_argtr_dn2_slot: &mut f64,
        var_argtr_dn3_slot: &mut f64,
        var_argtr_dn4_slot: &mut f64,
        var_argtr_dn5_slot: &mut f64,
        var_argtr_dn6_slot: &mut f64,
        var_argtr_rdb0_slot: &mut f64,
        var_argtr_rdb1_slot: &mut f64,
        var_argtr_rdb2_slot: &mut f64,
        var_argtr_rdb3_slot: &mut f64,
        var_argtr_rdb4_slot: &mut f64,
        var_argtr_rdb5_slot: &mut f64,
        var_argtr_rdb6_slot: &mut f64,
        var_argtr_rdn0_slot: &mut f64,
        var_argtr_rdn1_slot: &mut f64,
        var_argtr_rdn2_slot: &mut f64,
        var_argtr_rdn3_slot: &mut f64,
        var_argtr_rdn4_slot: &mut f64,
        var_argtr_rdn5_slot: &mut f64,
        var_argtr_rdn6_slot: &mut f64,
        var_argtr_rv_slot: &mut f64,
        var_bvr_t_slot: &mut f64,
        var_bvr_t_db0_slot: &mut f64,
        var_bvr_t_db1_slot: &mut f64,
        var_bvr_t_db2_slot: &mut f64,
        var_bvr_t_db3_slot: &mut f64,
        var_bvr_t_db4_slot: &mut f64,
        var_bvr_t_db5_slot: &mut f64,
        var_bvr_t_db6_slot: &mut f64,
        var_bvr_t_dn0_slot: &mut f64,
        var_bvr_t_dn1_slot: &mut f64,
        var_bvr_t_dn2_slot: &mut f64,
        var_bvr_t_dn3_slot: &mut f64,
        var_bvr_t_dn4_slot: &mut f64,
        var_bvr_t_dn5_slot: &mut f64,
        var_bvr_t_dn6_slot: &mut f64,
        var_bvr_t_rdb0_slot: &mut f64,
        var_bvr_t_rdb1_slot: &mut f64,
        var_bvr_t_rdb2_slot: &mut f64,
        var_bvr_t_rdb3_slot: &mut f64,
        var_bvr_t_rdb4_slot: &mut f64,
        var_bvr_t_rdb5_slot: &mut f64,
        var_bvr_t_rdb6_slot: &mut f64,
        var_bvr_t_rdn0_slot: &mut f64,
        var_bvr_t_rdn1_slot: &mut f64,
        var_bvr_t_rdn2_slot: &mut f64,
        var_bvr_t_rdn3_slot: &mut f64,
        var_bvr_t_rdn4_slot: &mut f64,
        var_bvr_t_rdn5_slot: &mut f64,
        var_bvr_t_rdn6_slot: &mut f64,
        var_bvr_t_rv_slot: &mut f64,
        var_ijbv_t_slot: &mut f64,
        var_ijbv_t_db0_slot: &mut f64,
        var_ijbv_t_db1_slot: &mut f64,
        var_ijbv_t_db2_slot: &mut f64,
        var_ijbv_t_db3_slot: &mut f64,
        var_ijbv_t_db4_slot: &mut f64,
        var_ijbv_t_db5_slot: &mut f64,
        var_ijbv_t_db6_slot: &mut f64,
        var_ijbv_t_dn0_slot: &mut f64,
        var_ijbv_t_dn1_slot: &mut f64,
        var_ijbv_t_dn2_slot: &mut f64,
        var_ijbv_t_dn3_slot: &mut f64,
        var_ijbv_t_dn4_slot: &mut f64,
        var_ijbv_t_dn5_slot: &mut f64,
        var_ijbv_t_dn6_slot: &mut f64,
        var_ijbv_t_rdb0_slot: &mut f64,
        var_ijbv_t_rdb1_slot: &mut f64,
        var_ijbv_t_rdb2_slot: &mut f64,
        var_ijbv_t_rdb3_slot: &mut f64,
        var_ijbv_t_rdb4_slot: &mut f64,
        var_ijbv_t_rdb5_slot: &mut f64,
        var_ijbv_t_rdb6_slot: &mut f64,
        var_ijbv_t_rdn0_slot: &mut f64,
        var_ijbv_t_rdn1_slot: &mut f64,
        var_ijbv_t_rdn2_slot: &mut f64,
        var_ijbv_t_rdn3_slot: &mut f64,
        var_ijbv_t_rdn4_slot: &mut f64,
        var_ijbv_t_rdn5_slot: &mut f64,
        var_ijbv_t_rdn6_slot: &mut f64,
        var_ijbv_t_rv_slot: &mut f64,
        var_is_t_slot: &mut f64,
        var_is_t_db0_slot: &mut f64,
        var_is_t_db1_slot: &mut f64,
        var_is_t_db2_slot: &mut f64,
        var_is_t_db3_slot: &mut f64,
        var_is_t_db4_slot: &mut f64,
        var_is_t_db5_slot: &mut f64,
        var_is_t_db6_slot: &mut f64,
        var_is_t_dn0_slot: &mut f64,
        var_is_t_dn1_slot: &mut f64,
        var_is_t_dn2_slot: &mut f64,
        var_is_t_dn3_slot: &mut f64,
        var_is_t_dn4_slot: &mut f64,
        var_is_t_dn5_slot: &mut f64,
        var_is_t_dn6_slot: &mut f64,
        var_is_t_rdb0_slot: &mut f64,
        var_is_t_rdb1_slot: &mut f64,
        var_is_t_rdb2_slot: &mut f64,
        var_is_t_rdb3_slot: &mut f64,
        var_is_t_rdb4_slot: &mut f64,
        var_is_t_rdb5_slot: &mut f64,
        var_is_t_rdb6_slot: &mut f64,
        var_is_t_rdn0_slot: &mut f64,
        var_is_t_rdn1_slot: &mut f64,
        var_is_t_rdn2_slot: &mut f64,
        var_is_t_rdn3_slot: &mut f64,
        var_is_t_rdn4_slot: &mut f64,
        var_is_t_rdn5_slot: &mut f64,
        var_is_t_rdn6_slot: &mut f64,
        var_is_t_rv_slot: &mut f64,
        var_isr_t_slot: &mut f64,
        var_isr_t_db0_slot: &mut f64,
        var_isr_t_db1_slot: &mut f64,
        var_isr_t_db2_slot: &mut f64,
        var_isr_t_db3_slot: &mut f64,
        var_isr_t_db4_slot: &mut f64,
        var_isr_t_db5_slot: &mut f64,
        var_isr_t_db6_slot: &mut f64,
        var_isr_t_dn0_slot: &mut f64,
        var_isr_t_dn1_slot: &mut f64,
        var_isr_t_dn2_slot: &mut f64,
        var_isr_t_dn3_slot: &mut f64,
        var_isr_t_dn4_slot: &mut f64,
        var_isr_t_dn5_slot: &mut f64,
        var_isr_t_dn6_slot: &mut f64,
        var_isr_t_rdb0_slot: &mut f64,
        var_isr_t_rdb1_slot: &mut f64,
        var_isr_t_rdb2_slot: &mut f64,
        var_isr_t_rdb3_slot: &mut f64,
        var_isr_t_rdb4_slot: &mut f64,
        var_isr_t_rdb5_slot: &mut f64,
        var_isr_t_rdb6_slot: &mut f64,
        var_isr_t_rdn0_slot: &mut f64,
        var_isr_t_rdn1_slot: &mut f64,
        var_isr_t_rdn2_slot: &mut f64,
        var_isr_t_rdn3_slot: &mut f64,
        var_isr_t_rdn4_slot: &mut f64,
        var_isr_t_rdn5_slot: &mut f64,
        var_isr_t_rdn6_slot: &mut f64,
        var_isr_t_rv_slot: &mut f64,
        var_lnrt_slot: &mut f64,
        var_lnrt_db0_slot: &mut f64,
        var_lnrt_db1_slot: &mut f64,
        var_lnrt_db2_slot: &mut f64,
        var_lnrt_db3_slot: &mut f64,
        var_lnrt_db4_slot: &mut f64,
        var_lnrt_db5_slot: &mut f64,
        var_lnrt_db6_slot: &mut f64,
        var_lnrt_dn0_slot: &mut f64,
        var_lnrt_dn1_slot: &mut f64,
        var_lnrt_dn2_slot: &mut f64,
        var_lnrt_dn3_slot: &mut f64,
        var_lnrt_dn4_slot: &mut f64,
        var_lnrt_dn5_slot: &mut f64,
        var_lnrt_dn6_slot: &mut f64,
        var_lnrt_rdb0_slot: &mut f64,
        var_lnrt_rdb1_slot: &mut f64,
        var_lnrt_rdb2_slot: &mut f64,
        var_lnrt_rdb3_slot: &mut f64,
        var_lnrt_rdb4_slot: &mut f64,
        var_lnrt_rdb5_slot: &mut f64,
        var_lnrt_rdb6_slot: &mut f64,
        var_lnrt_rdn0_slot: &mut f64,
        var_lnrt_rdn1_slot: &mut f64,
        var_lnrt_rdn2_slot: &mut f64,
        var_lnrt_rdn3_slot: &mut f64,
        var_lnrt_rdn4_slot: &mut f64,
        var_lnrt_rdn5_slot: &mut f64,
        var_lnrt_rdn6_slot: &mut f64,
        var_lnrt_rv_slot: &mut f64,
        var_rt_slot: &mut f64,
        var_rt_db0_slot: &mut f64,
        var_rt_db1_slot: &mut f64,
        var_rt_db2_slot: &mut f64,
        var_rt_db3_slot: &mut f64,
        var_rt_db4_slot: &mut f64,
        var_rt_db5_slot: &mut f64,
        var_rt_db6_slot: &mut f64,
        var_rt_dn0_slot: &mut f64,
        var_rt_dn1_slot: &mut f64,
        var_rt_dn2_slot: &mut f64,
        var_rt_dn3_slot: &mut f64,
        var_rt_dn4_slot: &mut f64,
        var_rt_dn5_slot: &mut f64,
        var_rt_dn6_slot: &mut f64,
        var_rt_rdb0_slot: &mut f64,
        var_rt_rdb1_slot: &mut f64,
        var_rt_rdb2_slot: &mut f64,
        var_rt_rdb3_slot: &mut f64,
        var_rt_rdb4_slot: &mut f64,
        var_rt_rdb5_slot: &mut f64,
        var_rt_rdb6_slot: &mut f64,
        var_rt_rdn0_slot: &mut f64,
        var_rt_rdn1_slot: &mut f64,
        var_rt_rdn2_slot: &mut f64,
        var_rt_rdn3_slot: &mut f64,
        var_rt_rdn4_slot: &mut f64,
        var_rt_rdn5_slot: &mut f64,
        var_rt_rdn6_slot: &mut f64,
        var_rt_rv_slot: &mut f64,
        var_tamb_slot: &mut f64,
        var_tamb_db0_slot: &mut f64,
        var_tamb_db1_slot: &mut f64,
        var_tamb_db2_slot: &mut f64,
        var_tamb_db3_slot: &mut f64,
        var_tamb_db4_slot: &mut f64,
        var_tamb_db5_slot: &mut f64,
        var_tamb_db6_slot: &mut f64,
        var_tamb_dn0_slot: &mut f64,
        var_tamb_dn1_slot: &mut f64,
        var_tamb_dn2_slot: &mut f64,
        var_tamb_dn3_slot: &mut f64,
        var_tamb_dn4_slot: &mut f64,
        var_tamb_dn5_slot: &mut f64,
        var_tamb_dn6_slot: &mut f64,
        var_tamb_rdb0_slot: &mut f64,
        var_tamb_rdb1_slot: &mut f64,
        var_tamb_rdb2_slot: &mut f64,
        var_tamb_rdb3_slot: &mut f64,
        var_tamb_rdb4_slot: &mut f64,
        var_tamb_rdb5_slot: &mut f64,
        var_tamb_rdb6_slot: &mut f64,
        var_tamb_rdn0_slot: &mut f64,
        var_tamb_rdn1_slot: &mut f64,
        var_tamb_rdn2_slot: &mut f64,
        var_tamb_rdn3_slot: &mut f64,
        var_tamb_rdn4_slot: &mut f64,
        var_tamb_rdn5_slot: &mut f64,
        var_tamb_rdn6_slot: &mut f64,
        var_tamb_rv_slot: &mut f64,
        var_tdev_slot: &mut f64,
        var_tdev_db0_slot: &mut f64,
        var_tdev_db1_slot: &mut f64,
        var_tdev_db2_slot: &mut f64,
        var_tdev_db3_slot: &mut f64,
        var_tdev_db4_slot: &mut f64,
        var_tdev_db5_slot: &mut f64,
        var_tdev_db6_slot: &mut f64,
        var_tdev_dn0_slot: &mut f64,
        var_tdev_dn1_slot: &mut f64,
        var_tdev_dn2_slot: &mut f64,
        var_tdev_dn3_slot: &mut f64,
        var_tdev_dn4_slot: &mut f64,
        var_tdev_dn5_slot: &mut f64,
        var_tdev_dn6_slot: &mut f64,
        var_tdev_rdb0_slot: &mut f64,
        var_tdev_rdb1_slot: &mut f64,
        var_tdev_rdb2_slot: &mut f64,
        var_tdev_rdb3_slot: &mut f64,
        var_tdev_rdb4_slot: &mut f64,
        var_tdev_rdb5_slot: &mut f64,
        var_tdev_rdb6_slot: &mut f64,
        var_tdev_rdn0_slot: &mut f64,
        var_tdev_rdn1_slot: &mut f64,
        var_tdev_rdn2_slot: &mut f64,
        var_tdev_rdn3_slot: &mut f64,
        var_tdev_rdn4_slot: &mut f64,
        var_tdev_rdn5_slot: &mut f64,
        var_tdev_rdn6_slot: &mut f64,
        var_tdev_rv_slot: &mut f64,
        var_theexp_t_slot: &mut f64,
        var_theexp_t_db0_slot: &mut f64,
        var_theexp_t_db1_slot: &mut f64,
        var_theexp_t_db2_slot: &mut f64,
        var_theexp_t_db3_slot: &mut f64,
        var_theexp_t_db4_slot: &mut f64,
        var_theexp_t_db5_slot: &mut f64,
        var_theexp_t_db6_slot: &mut f64,
        var_theexp_t_dn0_slot: &mut f64,
        var_theexp_t_dn1_slot: &mut f64,
        var_theexp_t_dn2_slot: &mut f64,
        var_theexp_t_dn3_slot: &mut f64,
        var_theexp_t_dn4_slot: &mut f64,
        var_theexp_t_dn5_slot: &mut f64,
        var_theexp_t_dn6_slot: &mut f64,
        var_theexp_t_rdb0_slot: &mut f64,
        var_theexp_t_rdb1_slot: &mut f64,
        var_theexp_t_rdb2_slot: &mut f64,
        var_theexp_t_rdb3_slot: &mut f64,
        var_theexp_t_rdb4_slot: &mut f64,
        var_theexp_t_rdb5_slot: &mut f64,
        var_theexp_t_rdb6_slot: &mut f64,
        var_theexp_t_rdn0_slot: &mut f64,
        var_theexp_t_rdn1_slot: &mut f64,
        var_theexp_t_rdn2_slot: &mut f64,
        var_theexp_t_rdn3_slot: &mut f64,
        var_theexp_t_rdn4_slot: &mut f64,
        var_theexp_t_rdn5_slot: &mut f64,
        var_theexp_t_rdn6_slot: &mut f64,
        var_theexp_t_rv_slot: &mut f64,
        var_tnom_slot: &mut f64,
        var_tnom_db0_slot: &mut f64,
        var_tnom_db1_slot: &mut f64,
        var_tnom_db2_slot: &mut f64,
        var_tnom_db3_slot: &mut f64,
        var_tnom_db4_slot: &mut f64,
        var_tnom_db5_slot: &mut f64,
        var_tnom_db6_slot: &mut f64,
        var_tnom_dn0_slot: &mut f64,
        var_tnom_dn1_slot: &mut f64,
        var_tnom_dn2_slot: &mut f64,
        var_tnom_dn3_slot: &mut f64,
        var_tnom_dn4_slot: &mut f64,
        var_tnom_dn5_slot: &mut f64,
        var_tnom_dn6_slot: &mut f64,
        var_tnom_rdb0_slot: &mut f64,
        var_tnom_rdb1_slot: &mut f64,
        var_tnom_rdb2_slot: &mut f64,
        var_tnom_rdb3_slot: &mut f64,
        var_tnom_rdb4_slot: &mut f64,
        var_tnom_rdb5_slot: &mut f64,
        var_tnom_rdb6_slot: &mut f64,
        var_tnom_rdn0_slot: &mut f64,
        var_tnom_rdn1_slot: &mut f64,
        var_tnom_rdn2_slot: &mut f64,
        var_tnom_rdn3_slot: &mut f64,
        var_tnom_rdn4_slot: &mut f64,
        var_tnom_rdn5_slot: &mut f64,
        var_tnom_rdn6_slot: &mut f64,
        var_tnom_rv_slot: &mut f64,
        var_vt_slot: &mut f64,
        var_vt_db0_slot: &mut f64,
        var_vt_db1_slot: &mut f64,
        var_vt_db2_slot: &mut f64,
        var_vt_db3_slot: &mut f64,
        var_vt_db4_slot: &mut f64,
        var_vt_db5_slot: &mut f64,
        var_vt_db6_slot: &mut f64,
        var_vt_dn0_slot: &mut f64,
        var_vt_dn1_slot: &mut f64,
        var_vt_dn2_slot: &mut f64,
        var_vt_dn3_slot: &mut f64,
        var_vt_dn4_slot: &mut f64,
        var_vt_dn5_slot: &mut f64,
        var_vt_dn6_slot: &mut f64,
        var_vt_rdb0_slot: &mut f64,
        var_vt_rdb1_slot: &mut f64,
        var_vt_rdb2_slot: &mut f64,
        var_vt_rdb3_slot: &mut f64,
        var_vt_rdb4_slot: &mut f64,
        var_vt_rdb5_slot: &mut f64,
        var_vt_rdb6_slot: &mut f64,
        var_vt_rdn0_slot: &mut f64,
        var_vt_rdn1_slot: &mut f64,
        var_vt_rdn2_slot: &mut f64,
        var_vt_rdn3_slot: &mut f64,
        var_vt_rdn4_slot: &mut f64,
        var_vt_rdn5_slot: &mut f64,
        var_vt_rdn6_slot: &mut f64,
        var_vt_rv_slot: &mut f64,
        var_weff_slot: &mut f64,
        var_weff_db0_slot: &mut f64,
        var_weff_db1_slot: &mut f64,
        var_weff_db2_slot: &mut f64,
        var_weff_db3_slot: &mut f64,
        var_weff_db4_slot: &mut f64,
        var_weff_db5_slot: &mut f64,
        var_weff_db6_slot: &mut f64,
        var_weff_dn0_slot: &mut f64,
        var_weff_dn1_slot: &mut f64,
        var_weff_dn2_slot: &mut f64,
        var_weff_dn3_slot: &mut f64,
        var_weff_dn4_slot: &mut f64,
        var_weff_dn5_slot: &mut f64,
        var_weff_dn6_slot: &mut f64,
        var_weff_rdb0_slot: &mut f64,
        var_weff_rdb1_slot: &mut f64,
        var_weff_rdb2_slot: &mut f64,
        var_weff_rdb3_slot: &mut f64,
        var_weff_rdb4_slot: &mut f64,
        var_weff_rdb5_slot: &mut f64,
        var_weff_rdb6_slot: &mut f64,
        var_weff_rdn0_slot: &mut f64,
        var_weff_rdn1_slot: &mut f64,
        var_weff_rdn2_slot: &mut f64,
        var_weff_rdn3_slot: &mut f64,
        var_weff_rdn4_slot: &mut f64,
        var_weff_rdn5_slot: &mut f64,
        var_weff_rdn6_slot: &mut f64,
        var_weff_rv_slot: &mut f64,
    ) {
        let ctx_temp = ctx.temperature();
        let nv2 = ctx.node_voltage(nodes[2]);
        let mut var_argt: f64 = *var_argt_slot;
        let mut var_argt_db0: f64 = *var_argt_db0_slot;
        let mut var_argt_db1: f64 = *var_argt_db1_slot;
        let mut var_argt_db2: f64 = *var_argt_db2_slot;
        let mut var_argt_db3: f64 = *var_argt_db3_slot;
        let mut var_argt_db4: f64 = *var_argt_db4_slot;
        let mut var_argt_db5: f64 = *var_argt_db5_slot;
        let mut var_argt_db6: f64 = *var_argt_db6_slot;
        let mut var_argt_dn0: f64 = *var_argt_dn0_slot;
        let mut var_argt_dn1: f64 = *var_argt_dn1_slot;
        let mut var_argt_dn2: f64 = *var_argt_dn2_slot;
        let mut var_argt_dn3: f64 = *var_argt_dn3_slot;
        let mut var_argt_dn4: f64 = *var_argt_dn4_slot;
        let mut var_argt_dn5: f64 = *var_argt_dn5_slot;
        let mut var_argt_dn6: f64 = *var_argt_dn6_slot;
        let mut var_argt_rdb0: f64 = *var_argt_rdb0_slot;
        let mut var_argt_rdb1: f64 = *var_argt_rdb1_slot;
        let mut var_argt_rdb2: f64 = *var_argt_rdb2_slot;
        let mut var_argt_rdb3: f64 = *var_argt_rdb3_slot;
        let mut var_argt_rdb4: f64 = *var_argt_rdb4_slot;
        let mut var_argt_rdb5: f64 = *var_argt_rdb5_slot;
        let mut var_argt_rdb6: f64 = *var_argt_rdb6_slot;
        let mut var_argt_rdn0: f64 = *var_argt_rdn0_slot;
        let mut var_argt_rdn1: f64 = *var_argt_rdn1_slot;
        let mut var_argt_rdn2: f64 = *var_argt_rdn2_slot;
        let mut var_argt_rdn3: f64 = *var_argt_rdn3_slot;
        let mut var_argt_rdn4: f64 = *var_argt_rdn4_slot;
        let mut var_argt_rdn5: f64 = *var_argt_rdn5_slot;
        let mut var_argt_rdn6: f64 = *var_argt_rdn6_slot;
        let mut var_argt_rv: f64 = *var_argt_rv_slot;
        let mut var_argtr: f64 = *var_argtr_slot;
        let mut var_argtr_db0: f64 = *var_argtr_db0_slot;
        let mut var_argtr_db1: f64 = *var_argtr_db1_slot;
        let mut var_argtr_db2: f64 = *var_argtr_db2_slot;
        let mut var_argtr_db3: f64 = *var_argtr_db3_slot;
        let mut var_argtr_db4: f64 = *var_argtr_db4_slot;
        let mut var_argtr_db5: f64 = *var_argtr_db5_slot;
        let mut var_argtr_db6: f64 = *var_argtr_db6_slot;
        let mut var_argtr_dn0: f64 = *var_argtr_dn0_slot;
        let mut var_argtr_dn1: f64 = *var_argtr_dn1_slot;
        let mut var_argtr_dn2: f64 = *var_argtr_dn2_slot;
        let mut var_argtr_dn3: f64 = *var_argtr_dn3_slot;
        let mut var_argtr_dn4: f64 = *var_argtr_dn4_slot;
        let mut var_argtr_dn5: f64 = *var_argtr_dn5_slot;
        let mut var_argtr_dn6: f64 = *var_argtr_dn6_slot;
        let mut var_argtr_rdb0: f64 = *var_argtr_rdb0_slot;
        let mut var_argtr_rdb1: f64 = *var_argtr_rdb1_slot;
        let mut var_argtr_rdb2: f64 = *var_argtr_rdb2_slot;
        let mut var_argtr_rdb3: f64 = *var_argtr_rdb3_slot;
        let mut var_argtr_rdb4: f64 = *var_argtr_rdb4_slot;
        let mut var_argtr_rdb5: f64 = *var_argtr_rdb5_slot;
        let mut var_argtr_rdb6: f64 = *var_argtr_rdb6_slot;
        let mut var_argtr_rdn0: f64 = *var_argtr_rdn0_slot;
        let mut var_argtr_rdn1: f64 = *var_argtr_rdn1_slot;
        let mut var_argtr_rdn2: f64 = *var_argtr_rdn2_slot;
        let mut var_argtr_rdn3: f64 = *var_argtr_rdn3_slot;
        let mut var_argtr_rdn4: f64 = *var_argtr_rdn4_slot;
        let mut var_argtr_rdn5: f64 = *var_argtr_rdn5_slot;
        let mut var_argtr_rdn6: f64 = *var_argtr_rdn6_slot;
        let mut var_argtr_rv: f64 = *var_argtr_rv_slot;
        let mut var_bvr_t: f64 = *var_bvr_t_slot;
        let mut var_bvr_t_db0: f64 = *var_bvr_t_db0_slot;
        let mut var_bvr_t_db1: f64 = *var_bvr_t_db1_slot;
        let mut var_bvr_t_db2: f64 = *var_bvr_t_db2_slot;
        let mut var_bvr_t_db3: f64 = *var_bvr_t_db3_slot;
        let mut var_bvr_t_db4: f64 = *var_bvr_t_db4_slot;
        let mut var_bvr_t_db5: f64 = *var_bvr_t_db5_slot;
        let mut var_bvr_t_db6: f64 = *var_bvr_t_db6_slot;
        let mut var_bvr_t_dn0: f64 = *var_bvr_t_dn0_slot;
        let mut var_bvr_t_dn1: f64 = *var_bvr_t_dn1_slot;
        let mut var_bvr_t_dn2: f64 = *var_bvr_t_dn2_slot;
        let mut var_bvr_t_dn3: f64 = *var_bvr_t_dn3_slot;
        let mut var_bvr_t_dn4: f64 = *var_bvr_t_dn4_slot;
        let mut var_bvr_t_dn5: f64 = *var_bvr_t_dn5_slot;
        let mut var_bvr_t_dn6: f64 = *var_bvr_t_dn6_slot;
        let mut var_bvr_t_rdb0: f64 = *var_bvr_t_rdb0_slot;
        let mut var_bvr_t_rdb1: f64 = *var_bvr_t_rdb1_slot;
        let mut var_bvr_t_rdb2: f64 = *var_bvr_t_rdb2_slot;
        let mut var_bvr_t_rdb3: f64 = *var_bvr_t_rdb3_slot;
        let mut var_bvr_t_rdb4: f64 = *var_bvr_t_rdb4_slot;
        let mut var_bvr_t_rdb5: f64 = *var_bvr_t_rdb5_slot;
        let mut var_bvr_t_rdb6: f64 = *var_bvr_t_rdb6_slot;
        let mut var_bvr_t_rdn0: f64 = *var_bvr_t_rdn0_slot;
        let mut var_bvr_t_rdn1: f64 = *var_bvr_t_rdn1_slot;
        let mut var_bvr_t_rdn2: f64 = *var_bvr_t_rdn2_slot;
        let mut var_bvr_t_rdn3: f64 = *var_bvr_t_rdn3_slot;
        let mut var_bvr_t_rdn4: f64 = *var_bvr_t_rdn4_slot;
        let mut var_bvr_t_rdn5: f64 = *var_bvr_t_rdn5_slot;
        let mut var_bvr_t_rdn6: f64 = *var_bvr_t_rdn6_slot;
        let mut var_bvr_t_rv: f64 = *var_bvr_t_rv_slot;
        let mut var_ijbv_t: f64 = *var_ijbv_t_slot;
        let mut var_ijbv_t_db0: f64 = *var_ijbv_t_db0_slot;
        let mut var_ijbv_t_db1: f64 = *var_ijbv_t_db1_slot;
        let mut var_ijbv_t_db2: f64 = *var_ijbv_t_db2_slot;
        let mut var_ijbv_t_db3: f64 = *var_ijbv_t_db3_slot;
        let mut var_ijbv_t_db4: f64 = *var_ijbv_t_db4_slot;
        let mut var_ijbv_t_db5: f64 = *var_ijbv_t_db5_slot;
        let mut var_ijbv_t_db6: f64 = *var_ijbv_t_db6_slot;
        let mut var_ijbv_t_dn0: f64 = *var_ijbv_t_dn0_slot;
        let mut var_ijbv_t_dn1: f64 = *var_ijbv_t_dn1_slot;
        let mut var_ijbv_t_dn2: f64 = *var_ijbv_t_dn2_slot;
        let mut var_ijbv_t_dn3: f64 = *var_ijbv_t_dn3_slot;
        let mut var_ijbv_t_dn4: f64 = *var_ijbv_t_dn4_slot;
        let mut var_ijbv_t_dn5: f64 = *var_ijbv_t_dn5_slot;
        let mut var_ijbv_t_dn6: f64 = *var_ijbv_t_dn6_slot;
        let mut var_ijbv_t_rdb0: f64 = *var_ijbv_t_rdb0_slot;
        let mut var_ijbv_t_rdb1: f64 = *var_ijbv_t_rdb1_slot;
        let mut var_ijbv_t_rdb2: f64 = *var_ijbv_t_rdb2_slot;
        let mut var_ijbv_t_rdb3: f64 = *var_ijbv_t_rdb3_slot;
        let mut var_ijbv_t_rdb4: f64 = *var_ijbv_t_rdb4_slot;
        let mut var_ijbv_t_rdb5: f64 = *var_ijbv_t_rdb5_slot;
        let mut var_ijbv_t_rdb6: f64 = *var_ijbv_t_rdb6_slot;
        let mut var_ijbv_t_rdn0: f64 = *var_ijbv_t_rdn0_slot;
        let mut var_ijbv_t_rdn1: f64 = *var_ijbv_t_rdn1_slot;
        let mut var_ijbv_t_rdn2: f64 = *var_ijbv_t_rdn2_slot;
        let mut var_ijbv_t_rdn3: f64 = *var_ijbv_t_rdn3_slot;
        let mut var_ijbv_t_rdn4: f64 = *var_ijbv_t_rdn4_slot;
        let mut var_ijbv_t_rdn5: f64 = *var_ijbv_t_rdn5_slot;
        let mut var_ijbv_t_rdn6: f64 = *var_ijbv_t_rdn6_slot;
        let mut var_ijbv_t_rv: f64 = *var_ijbv_t_rv_slot;
        let mut var_is_t: f64 = *var_is_t_slot;
        let mut var_is_t_db0: f64 = *var_is_t_db0_slot;
        let mut var_is_t_db1: f64 = *var_is_t_db1_slot;
        let mut var_is_t_db2: f64 = *var_is_t_db2_slot;
        let mut var_is_t_db3: f64 = *var_is_t_db3_slot;
        let mut var_is_t_db4: f64 = *var_is_t_db4_slot;
        let mut var_is_t_db5: f64 = *var_is_t_db5_slot;
        let mut var_is_t_db6: f64 = *var_is_t_db6_slot;
        let mut var_is_t_dn0: f64 = *var_is_t_dn0_slot;
        let mut var_is_t_dn1: f64 = *var_is_t_dn1_slot;
        let mut var_is_t_dn2: f64 = *var_is_t_dn2_slot;
        let mut var_is_t_dn3: f64 = *var_is_t_dn3_slot;
        let mut var_is_t_dn4: f64 = *var_is_t_dn4_slot;
        let mut var_is_t_dn5: f64 = *var_is_t_dn5_slot;
        let mut var_is_t_dn6: f64 = *var_is_t_dn6_slot;
        let mut var_is_t_rdb0: f64 = *var_is_t_rdb0_slot;
        let mut var_is_t_rdb1: f64 = *var_is_t_rdb1_slot;
        let mut var_is_t_rdb2: f64 = *var_is_t_rdb2_slot;
        let mut var_is_t_rdb3: f64 = *var_is_t_rdb3_slot;
        let mut var_is_t_rdb4: f64 = *var_is_t_rdb4_slot;
        let mut var_is_t_rdb5: f64 = *var_is_t_rdb5_slot;
        let mut var_is_t_rdb6: f64 = *var_is_t_rdb6_slot;
        let mut var_is_t_rdn0: f64 = *var_is_t_rdn0_slot;
        let mut var_is_t_rdn1: f64 = *var_is_t_rdn1_slot;
        let mut var_is_t_rdn2: f64 = *var_is_t_rdn2_slot;
        let mut var_is_t_rdn3: f64 = *var_is_t_rdn3_slot;
        let mut var_is_t_rdn4: f64 = *var_is_t_rdn4_slot;
        let mut var_is_t_rdn5: f64 = *var_is_t_rdn5_slot;
        let mut var_is_t_rdn6: f64 = *var_is_t_rdn6_slot;
        let mut var_is_t_rv: f64 = *var_is_t_rv_slot;
        let mut var_isr_t: f64 = *var_isr_t_slot;
        let mut var_isr_t_db0: f64 = *var_isr_t_db0_slot;
        let mut var_isr_t_db1: f64 = *var_isr_t_db1_slot;
        let mut var_isr_t_db2: f64 = *var_isr_t_db2_slot;
        let mut var_isr_t_db3: f64 = *var_isr_t_db3_slot;
        let mut var_isr_t_db4: f64 = *var_isr_t_db4_slot;
        let mut var_isr_t_db5: f64 = *var_isr_t_db5_slot;
        let mut var_isr_t_db6: f64 = *var_isr_t_db6_slot;
        let mut var_isr_t_dn0: f64 = *var_isr_t_dn0_slot;
        let mut var_isr_t_dn1: f64 = *var_isr_t_dn1_slot;
        let mut var_isr_t_dn2: f64 = *var_isr_t_dn2_slot;
        let mut var_isr_t_dn3: f64 = *var_isr_t_dn3_slot;
        let mut var_isr_t_dn4: f64 = *var_isr_t_dn4_slot;
        let mut var_isr_t_dn5: f64 = *var_isr_t_dn5_slot;
        let mut var_isr_t_dn6: f64 = *var_isr_t_dn6_slot;
        let mut var_isr_t_rdb0: f64 = *var_isr_t_rdb0_slot;
        let mut var_isr_t_rdb1: f64 = *var_isr_t_rdb1_slot;
        let mut var_isr_t_rdb2: f64 = *var_isr_t_rdb2_slot;
        let mut var_isr_t_rdb3: f64 = *var_isr_t_rdb3_slot;
        let mut var_isr_t_rdb4: f64 = *var_isr_t_rdb4_slot;
        let mut var_isr_t_rdb5: f64 = *var_isr_t_rdb5_slot;
        let mut var_isr_t_rdb6: f64 = *var_isr_t_rdb6_slot;
        let mut var_isr_t_rdn0: f64 = *var_isr_t_rdn0_slot;
        let mut var_isr_t_rdn1: f64 = *var_isr_t_rdn1_slot;
        let mut var_isr_t_rdn2: f64 = *var_isr_t_rdn2_slot;
        let mut var_isr_t_rdn3: f64 = *var_isr_t_rdn3_slot;
        let mut var_isr_t_rdn4: f64 = *var_isr_t_rdn4_slot;
        let mut var_isr_t_rdn5: f64 = *var_isr_t_rdn5_slot;
        let mut var_isr_t_rdn6: f64 = *var_isr_t_rdn6_slot;
        let mut var_isr_t_rv: f64 = *var_isr_t_rv_slot;
        let mut var_lnrt: f64 = *var_lnrt_slot;
        let mut var_lnrt_db0: f64 = *var_lnrt_db0_slot;
        let mut var_lnrt_db1: f64 = *var_lnrt_db1_slot;
        let mut var_lnrt_db2: f64 = *var_lnrt_db2_slot;
        let mut var_lnrt_db3: f64 = *var_lnrt_db3_slot;
        let mut var_lnrt_db4: f64 = *var_lnrt_db4_slot;
        let mut var_lnrt_db5: f64 = *var_lnrt_db5_slot;
        let mut var_lnrt_db6: f64 = *var_lnrt_db6_slot;
        let mut var_lnrt_dn0: f64 = *var_lnrt_dn0_slot;
        let mut var_lnrt_dn1: f64 = *var_lnrt_dn1_slot;
        let mut var_lnrt_dn2: f64 = *var_lnrt_dn2_slot;
        let mut var_lnrt_dn3: f64 = *var_lnrt_dn3_slot;
        let mut var_lnrt_dn4: f64 = *var_lnrt_dn4_slot;
        let mut var_lnrt_dn5: f64 = *var_lnrt_dn5_slot;
        let mut var_lnrt_dn6: f64 = *var_lnrt_dn6_slot;
        let mut var_lnrt_rdb0: f64 = *var_lnrt_rdb0_slot;
        let mut var_lnrt_rdb1: f64 = *var_lnrt_rdb1_slot;
        let mut var_lnrt_rdb2: f64 = *var_lnrt_rdb2_slot;
        let mut var_lnrt_rdb3: f64 = *var_lnrt_rdb3_slot;
        let mut var_lnrt_rdb4: f64 = *var_lnrt_rdb4_slot;
        let mut var_lnrt_rdb5: f64 = *var_lnrt_rdb5_slot;
        let mut var_lnrt_rdb6: f64 = *var_lnrt_rdb6_slot;
        let mut var_lnrt_rdn0: f64 = *var_lnrt_rdn0_slot;
        let mut var_lnrt_rdn1: f64 = *var_lnrt_rdn1_slot;
        let mut var_lnrt_rdn2: f64 = *var_lnrt_rdn2_slot;
        let mut var_lnrt_rdn3: f64 = *var_lnrt_rdn3_slot;
        let mut var_lnrt_rdn4: f64 = *var_lnrt_rdn4_slot;
        let mut var_lnrt_rdn5: f64 = *var_lnrt_rdn5_slot;
        let mut var_lnrt_rdn6: f64 = *var_lnrt_rdn6_slot;
        let mut var_lnrt_rv: f64 = *var_lnrt_rv_slot;
        let mut var_rt: f64 = *var_rt_slot;
        let mut var_rt_db0: f64 = *var_rt_db0_slot;
        let mut var_rt_db1: f64 = *var_rt_db1_slot;
        let mut var_rt_db2: f64 = *var_rt_db2_slot;
        let mut var_rt_db3: f64 = *var_rt_db3_slot;
        let mut var_rt_db4: f64 = *var_rt_db4_slot;
        let mut var_rt_db5: f64 = *var_rt_db5_slot;
        let mut var_rt_db6: f64 = *var_rt_db6_slot;
        let mut var_rt_dn0: f64 = *var_rt_dn0_slot;
        let mut var_rt_dn1: f64 = *var_rt_dn1_slot;
        let mut var_rt_dn2: f64 = *var_rt_dn2_slot;
        let mut var_rt_dn3: f64 = *var_rt_dn3_slot;
        let mut var_rt_dn4: f64 = *var_rt_dn4_slot;
        let mut var_rt_dn5: f64 = *var_rt_dn5_slot;
        let mut var_rt_dn6: f64 = *var_rt_dn6_slot;
        let mut var_rt_rdb0: f64 = *var_rt_rdb0_slot;
        let mut var_rt_rdb1: f64 = *var_rt_rdb1_slot;
        let mut var_rt_rdb2: f64 = *var_rt_rdb2_slot;
        let mut var_rt_rdb3: f64 = *var_rt_rdb3_slot;
        let mut var_rt_rdb4: f64 = *var_rt_rdb4_slot;
        let mut var_rt_rdb5: f64 = *var_rt_rdb5_slot;
        let mut var_rt_rdb6: f64 = *var_rt_rdb6_slot;
        let mut var_rt_rdn0: f64 = *var_rt_rdn0_slot;
        let mut var_rt_rdn1: f64 = *var_rt_rdn1_slot;
        let mut var_rt_rdn2: f64 = *var_rt_rdn2_slot;
        let mut var_rt_rdn3: f64 = *var_rt_rdn3_slot;
        let mut var_rt_rdn4: f64 = *var_rt_rdn4_slot;
        let mut var_rt_rdn5: f64 = *var_rt_rdn5_slot;
        let mut var_rt_rdn6: f64 = *var_rt_rdn6_slot;
        let mut var_rt_rv: f64 = *var_rt_rv_slot;
        let mut var_tamb: f64 = *var_tamb_slot;
        let mut var_tamb_db0: f64 = *var_tamb_db0_slot;
        let mut var_tamb_db1: f64 = *var_tamb_db1_slot;
        let mut var_tamb_db2: f64 = *var_tamb_db2_slot;
        let mut var_tamb_db3: f64 = *var_tamb_db3_slot;
        let mut var_tamb_db4: f64 = *var_tamb_db4_slot;
        let mut var_tamb_db5: f64 = *var_tamb_db5_slot;
        let mut var_tamb_db6: f64 = *var_tamb_db6_slot;
        let mut var_tamb_dn0: f64 = *var_tamb_dn0_slot;
        let mut var_tamb_dn1: f64 = *var_tamb_dn1_slot;
        let mut var_tamb_dn2: f64 = *var_tamb_dn2_slot;
        let mut var_tamb_dn3: f64 = *var_tamb_dn3_slot;
        let mut var_tamb_dn4: f64 = *var_tamb_dn4_slot;
        let mut var_tamb_dn5: f64 = *var_tamb_dn5_slot;
        let mut var_tamb_dn6: f64 = *var_tamb_dn6_slot;
        let mut var_tamb_rdb0: f64 = *var_tamb_rdb0_slot;
        let mut var_tamb_rdb1: f64 = *var_tamb_rdb1_slot;
        let mut var_tamb_rdb2: f64 = *var_tamb_rdb2_slot;
        let mut var_tamb_rdb3: f64 = *var_tamb_rdb3_slot;
        let mut var_tamb_rdb4: f64 = *var_tamb_rdb4_slot;
        let mut var_tamb_rdb5: f64 = *var_tamb_rdb5_slot;
        let mut var_tamb_rdb6: f64 = *var_tamb_rdb6_slot;
        let mut var_tamb_rdn0: f64 = *var_tamb_rdn0_slot;
        let mut var_tamb_rdn1: f64 = *var_tamb_rdn1_slot;
        let mut var_tamb_rdn2: f64 = *var_tamb_rdn2_slot;
        let mut var_tamb_rdn3: f64 = *var_tamb_rdn3_slot;
        let mut var_tamb_rdn4: f64 = *var_tamb_rdn4_slot;
        let mut var_tamb_rdn5: f64 = *var_tamb_rdn5_slot;
        let mut var_tamb_rdn6: f64 = *var_tamb_rdn6_slot;
        let mut var_tamb_rv: f64 = *var_tamb_rv_slot;
        let mut var_tdev: f64 = *var_tdev_slot;
        let mut var_tdev_db0: f64 = *var_tdev_db0_slot;
        let mut var_tdev_db1: f64 = *var_tdev_db1_slot;
        let mut var_tdev_db2: f64 = *var_tdev_db2_slot;
        let mut var_tdev_db3: f64 = *var_tdev_db3_slot;
        let mut var_tdev_db4: f64 = *var_tdev_db4_slot;
        let mut var_tdev_db5: f64 = *var_tdev_db5_slot;
        let mut var_tdev_db6: f64 = *var_tdev_db6_slot;
        let mut var_tdev_dn0: f64 = *var_tdev_dn0_slot;
        let mut var_tdev_dn1: f64 = *var_tdev_dn1_slot;
        let mut var_tdev_dn2: f64 = *var_tdev_dn2_slot;
        let mut var_tdev_dn3: f64 = *var_tdev_dn3_slot;
        let mut var_tdev_dn4: f64 = *var_tdev_dn4_slot;
        let mut var_tdev_dn5: f64 = *var_tdev_dn5_slot;
        let mut var_tdev_dn6: f64 = *var_tdev_dn6_slot;
        let mut var_tdev_rdb0: f64 = *var_tdev_rdb0_slot;
        let mut var_tdev_rdb1: f64 = *var_tdev_rdb1_slot;
        let mut var_tdev_rdb2: f64 = *var_tdev_rdb2_slot;
        let mut var_tdev_rdb3: f64 = *var_tdev_rdb3_slot;
        let mut var_tdev_rdb4: f64 = *var_tdev_rdb4_slot;
        let mut var_tdev_rdb5: f64 = *var_tdev_rdb5_slot;
        let mut var_tdev_rdb6: f64 = *var_tdev_rdb6_slot;
        let mut var_tdev_rdn0: f64 = *var_tdev_rdn0_slot;
        let mut var_tdev_rdn1: f64 = *var_tdev_rdn1_slot;
        let mut var_tdev_rdn2: f64 = *var_tdev_rdn2_slot;
        let mut var_tdev_rdn3: f64 = *var_tdev_rdn3_slot;
        let mut var_tdev_rdn4: f64 = *var_tdev_rdn4_slot;
        let mut var_tdev_rdn5: f64 = *var_tdev_rdn5_slot;
        let mut var_tdev_rdn6: f64 = *var_tdev_rdn6_slot;
        let mut var_tdev_rv: f64 = *var_tdev_rv_slot;
        let mut var_theexp_t: f64 = *var_theexp_t_slot;
        let mut var_theexp_t_db0: f64 = *var_theexp_t_db0_slot;
        let mut var_theexp_t_db1: f64 = *var_theexp_t_db1_slot;
        let mut var_theexp_t_db2: f64 = *var_theexp_t_db2_slot;
        let mut var_theexp_t_db3: f64 = *var_theexp_t_db3_slot;
        let mut var_theexp_t_db4: f64 = *var_theexp_t_db4_slot;
        let mut var_theexp_t_db5: f64 = *var_theexp_t_db5_slot;
        let mut var_theexp_t_db6: f64 = *var_theexp_t_db6_slot;
        let mut var_theexp_t_dn0: f64 = *var_theexp_t_dn0_slot;
        let mut var_theexp_t_dn1: f64 = *var_theexp_t_dn1_slot;
        let mut var_theexp_t_dn2: f64 = *var_theexp_t_dn2_slot;
        let mut var_theexp_t_dn3: f64 = *var_theexp_t_dn3_slot;
        let mut var_theexp_t_dn4: f64 = *var_theexp_t_dn4_slot;
        let mut var_theexp_t_dn5: f64 = *var_theexp_t_dn5_slot;
        let mut var_theexp_t_dn6: f64 = *var_theexp_t_dn6_slot;
        let mut var_theexp_t_rdb0: f64 = *var_theexp_t_rdb0_slot;
        let mut var_theexp_t_rdb1: f64 = *var_theexp_t_rdb1_slot;
        let mut var_theexp_t_rdb2: f64 = *var_theexp_t_rdb2_slot;
        let mut var_theexp_t_rdb3: f64 = *var_theexp_t_rdb3_slot;
        let mut var_theexp_t_rdb4: f64 = *var_theexp_t_rdb4_slot;
        let mut var_theexp_t_rdb5: f64 = *var_theexp_t_rdb5_slot;
        let mut var_theexp_t_rdb6: f64 = *var_theexp_t_rdb6_slot;
        let mut var_theexp_t_rdn0: f64 = *var_theexp_t_rdn0_slot;
        let mut var_theexp_t_rdn1: f64 = *var_theexp_t_rdn1_slot;
        let mut var_theexp_t_rdn2: f64 = *var_theexp_t_rdn2_slot;
        let mut var_theexp_t_rdn3: f64 = *var_theexp_t_rdn3_slot;
        let mut var_theexp_t_rdn4: f64 = *var_theexp_t_rdn4_slot;
        let mut var_theexp_t_rdn5: f64 = *var_theexp_t_rdn5_slot;
        let mut var_theexp_t_rdn6: f64 = *var_theexp_t_rdn6_slot;
        let mut var_theexp_t_rv: f64 = *var_theexp_t_rv_slot;
        let mut var_tnom: f64 = *var_tnom_slot;
        let mut var_tnom_db0: f64 = *var_tnom_db0_slot;
        let mut var_tnom_db1: f64 = *var_tnom_db1_slot;
        let mut var_tnom_db2: f64 = *var_tnom_db2_slot;
        let mut var_tnom_db3: f64 = *var_tnom_db3_slot;
        let mut var_tnom_db4: f64 = *var_tnom_db4_slot;
        let mut var_tnom_db5: f64 = *var_tnom_db5_slot;
        let mut var_tnom_db6: f64 = *var_tnom_db6_slot;
        let mut var_tnom_dn0: f64 = *var_tnom_dn0_slot;
        let mut var_tnom_dn1: f64 = *var_tnom_dn1_slot;
        let mut var_tnom_dn2: f64 = *var_tnom_dn2_slot;
        let mut var_tnom_dn3: f64 = *var_tnom_dn3_slot;
        let mut var_tnom_dn4: f64 = *var_tnom_dn4_slot;
        let mut var_tnom_dn5: f64 = *var_tnom_dn5_slot;
        let mut var_tnom_dn6: f64 = *var_tnom_dn6_slot;
        let mut var_tnom_rdb0: f64 = *var_tnom_rdb0_slot;
        let mut var_tnom_rdb1: f64 = *var_tnom_rdb1_slot;
        let mut var_tnom_rdb2: f64 = *var_tnom_rdb2_slot;
        let mut var_tnom_rdb3: f64 = *var_tnom_rdb3_slot;
        let mut var_tnom_rdb4: f64 = *var_tnom_rdb4_slot;
        let mut var_tnom_rdb5: f64 = *var_tnom_rdb5_slot;
        let mut var_tnom_rdb6: f64 = *var_tnom_rdb6_slot;
        let mut var_tnom_rdn0: f64 = *var_tnom_rdn0_slot;
        let mut var_tnom_rdn1: f64 = *var_tnom_rdn1_slot;
        let mut var_tnom_rdn2: f64 = *var_tnom_rdn2_slot;
        let mut var_tnom_rdn3: f64 = *var_tnom_rdn3_slot;
        let mut var_tnom_rdn4: f64 = *var_tnom_rdn4_slot;
        let mut var_tnom_rdn5: f64 = *var_tnom_rdn5_slot;
        let mut var_tnom_rdn6: f64 = *var_tnom_rdn6_slot;
        let mut var_tnom_rv: f64 = *var_tnom_rv_slot;
        let mut var_vt: f64 = *var_vt_slot;
        let mut var_vt_db0: f64 = *var_vt_db0_slot;
        let mut var_vt_db1: f64 = *var_vt_db1_slot;
        let mut var_vt_db2: f64 = *var_vt_db2_slot;
        let mut var_vt_db3: f64 = *var_vt_db3_slot;
        let mut var_vt_db4: f64 = *var_vt_db4_slot;
        let mut var_vt_db5: f64 = *var_vt_db5_slot;
        let mut var_vt_db6: f64 = *var_vt_db6_slot;
        let mut var_vt_dn0: f64 = *var_vt_dn0_slot;
        let mut var_vt_dn1: f64 = *var_vt_dn1_slot;
        let mut var_vt_dn2: f64 = *var_vt_dn2_slot;
        let mut var_vt_dn3: f64 = *var_vt_dn3_slot;
        let mut var_vt_dn4: f64 = *var_vt_dn4_slot;
        let mut var_vt_dn5: f64 = *var_vt_dn5_slot;
        let mut var_vt_dn6: f64 = *var_vt_dn6_slot;
        let mut var_vt_rdb0: f64 = *var_vt_rdb0_slot;
        let mut var_vt_rdb1: f64 = *var_vt_rdb1_slot;
        let mut var_vt_rdb2: f64 = *var_vt_rdb2_slot;
        let mut var_vt_rdb3: f64 = *var_vt_rdb3_slot;
        let mut var_vt_rdb4: f64 = *var_vt_rdb4_slot;
        let mut var_vt_rdb5: f64 = *var_vt_rdb5_slot;
        let mut var_vt_rdb6: f64 = *var_vt_rdb6_slot;
        let mut var_vt_rdn0: f64 = *var_vt_rdn0_slot;
        let mut var_vt_rdn1: f64 = *var_vt_rdn1_slot;
        let mut var_vt_rdn2: f64 = *var_vt_rdn2_slot;
        let mut var_vt_rdn3: f64 = *var_vt_rdn3_slot;
        let mut var_vt_rdn4: f64 = *var_vt_rdn4_slot;
        let mut var_vt_rdn5: f64 = *var_vt_rdn5_slot;
        let mut var_vt_rdn6: f64 = *var_vt_rdn6_slot;
        let mut var_vt_rv: f64 = *var_vt_rv_slot;
        let mut var_weff: f64 = *var_weff_slot;
        let mut var_weff_db0: f64 = *var_weff_db0_slot;
        let mut var_weff_db1: f64 = *var_weff_db1_slot;
        let mut var_weff_db2: f64 = *var_weff_db2_slot;
        let mut var_weff_db3: f64 = *var_weff_db3_slot;
        let mut var_weff_db4: f64 = *var_weff_db4_slot;
        let mut var_weff_db5: f64 = *var_weff_db5_slot;
        let mut var_weff_db6: f64 = *var_weff_db6_slot;
        let mut var_weff_dn0: f64 = *var_weff_dn0_slot;
        let mut var_weff_dn1: f64 = *var_weff_dn1_slot;
        let mut var_weff_dn2: f64 = *var_weff_dn2_slot;
        let mut var_weff_dn3: f64 = *var_weff_dn3_slot;
        let mut var_weff_dn4: f64 = *var_weff_dn4_slot;
        let mut var_weff_dn5: f64 = *var_weff_dn5_slot;
        let mut var_weff_dn6: f64 = *var_weff_dn6_slot;
        let mut var_weff_rdb0: f64 = *var_weff_rdb0_slot;
        let mut var_weff_rdb1: f64 = *var_weff_rdb1_slot;
        let mut var_weff_rdb2: f64 = *var_weff_rdb2_slot;
        let mut var_weff_rdb3: f64 = *var_weff_rdb3_slot;
        let mut var_weff_rdb4: f64 = *var_weff_rdb4_slot;
        let mut var_weff_rdb5: f64 = *var_weff_rdb5_slot;
        let mut var_weff_rdb6: f64 = *var_weff_rdb6_slot;
        let mut var_weff_rdn0: f64 = *var_weff_rdn0_slot;
        let mut var_weff_rdn1: f64 = *var_weff_rdn1_slot;
        let mut var_weff_rdn2: f64 = *var_weff_rdn2_slot;
        let mut var_weff_rdn3: f64 = *var_weff_rdn3_slot;
        let mut var_weff_rdn4: f64 = *var_weff_rdn4_slot;
        let mut var_weff_rdn5: f64 = *var_weff_rdn5_slot;
        let mut var_weff_rdn6: f64 = *var_weff_rdn6_slot;
        let mut var_weff_rv: f64 = *var_weff_rv_slot;

        let assign00_e291: f64 = ctx_temp;
        let assign00_e293: f64 = (assign00_e291 + (nv2 - 0.0));
        let assign00_e295: f64 = (assign00_e293 + p.p45);
        var_tamb = assign00_e295;
        var_tamb_dn0 = 0.0;
        var_tamb_dn1 = 0.0;
        var_tamb_dn2 = 1.0;
        var_tamb_dn3 = 0.0;
        var_tamb_dn4 = 0.0;
        var_tamb_dn5 = 0.0;
        var_tamb_dn6 = 0.0;
        var_tamb_db0 = 0.0;
        var_tamb_db1 = 0.0;
        var_tamb_db2 = 0.0;
        var_tamb_db3 = 0.0;
        var_tamb_db4 = 0.0;
        var_tamb_db5 = 0.0;
        var_tamb_db6 = 0.0;
        var_tamb_rv = 0.0;
        var_tamb_rdn0 = 0.0;
        var_tamb_rdn1 = 0.0;
        var_tamb_rdn2 = 0.0;
        var_tamb_rdn3 = 0.0;
        var_tamb_rdn4 = 0.0;
        var_tamb_rdn5 = 0.0;
        var_tamb_rdn6 = 0.0;
        var_tamb_rdb0 = 0.0;
        var_tamb_rdb1 = 0.0;
        var_tamb_rdb2 = 0.0;
        var_tamb_rdb3 = 0.0;
        var_tamb_rdb4 = 0.0;
        var_tamb_rdb5 = 0.0;
        var_tamb_rdb6 = 0.0;

        let assign10_e298: f64 = (1026.85 + 273.15);
        let assign10_e301: f64 = (-100.0);
        let assign10_e303: f64 = (assign10_e301 + 273.15);
        let (assign10_e310, assign10_e310_d_n0, assign10_e310_d_n1, assign10_e310_d_n2, assign10_e310_d_n3, assign10_e310_d_n4, assign10_e310_d_n5, assign10_e310_d_n6, assign10_e310_d_b0, assign10_e310_d_b1, assign10_e310_d_b2, assign10_e310_d_b3, assign10_e310_d_b4, assign10_e310_d_b5, assign10_e310_d_b6,) = {
    if (var_tamb > assign10_e303) {
        (var_tamb, var_tamb_dn0, var_tamb_dn1, var_tamb_dn2, var_tamb_dn3, var_tamb_dn4, var_tamb_dn5, var_tamb_dn6, var_tamb_db0, var_tamb_db1, var_tamb_db2, var_tamb_db3, var_tamb_db4, var_tamb_db5, var_tamb_db6,)
    } else {
        let assign10_e307: f64 = (-100.0);
        let assign10_e309: f64 = (assign10_e307 + 273.15);
        (assign10_e309, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let (assign10_e327, assign10_e327_d_n0, assign10_e327_d_n1, assign10_e327_d_n2, assign10_e327_d_n3, assign10_e327_d_n4, assign10_e327_d_n5, assign10_e327_d_n6, assign10_e327_d_b0, assign10_e327_d_b1, assign10_e327_d_b2, assign10_e327_d_b3, assign10_e327_d_b4, assign10_e327_d_b5, assign10_e327_d_b6,) = {
    if (assign10_e298 < assign10_e310) {
        let assign10_e314: f64 = (1026.85 + 273.15);
        (assign10_e314, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        let assign10_e317: f64 = (-100.0);
        let assign10_e319: f64 = (assign10_e317 + 273.15);
        let (assign10_e326, assign10_e326_d_n0, assign10_e326_d_n1, assign10_e326_d_n2, assign10_e326_d_n3, assign10_e326_d_n4, assign10_e326_d_n5, assign10_e326_d_n6, assign10_e326_d_b0, assign10_e326_d_b1, assign10_e326_d_b2, assign10_e326_d_b3, assign10_e326_d_b4, assign10_e326_d_b5, assign10_e326_d_b6,) = {
            if (var_tamb > assign10_e319) {
                (var_tamb, var_tamb_dn0, var_tamb_dn1, var_tamb_dn2, var_tamb_dn3, var_tamb_dn4, var_tamb_dn5, var_tamb_dn6, var_tamb_db0, var_tamb_db1, var_tamb_db2, var_tamb_db3, var_tamb_db4, var_tamb_db5, var_tamb_db6,)
            } else {
                let assign10_e323: f64 = (-100.0);
                let assign10_e325: f64 = (assign10_e323 + 273.15);
                (assign10_e325, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign10_e326, assign10_e326_d_n0, assign10_e326_d_n1, assign10_e326_d_n2, assign10_e326_d_n3, assign10_e326_d_n4, assign10_e326_d_n5, assign10_e326_d_n6, assign10_e326_d_b0, assign10_e326_d_b1, assign10_e326_d_b2, assign10_e326_d_b3, assign10_e326_d_b4, assign10_e326_d_b5, assign10_e326_d_b6,)
    }
};
        var_tdev = assign10_e327;
        var_tdev_dn0 = assign10_e327_d_n0;
        var_tdev_dn1 = assign10_e327_d_n1;
        var_tdev_dn2 = assign10_e327_d_n2;
        var_tdev_dn3 = assign10_e327_d_n3;
        var_tdev_dn4 = assign10_e327_d_n4;
        var_tdev_dn5 = assign10_e327_d_n5;
        var_tdev_dn6 = assign10_e327_d_n6;
        var_tdev_db0 = assign10_e327_d_b0;
        var_tdev_db1 = assign10_e327_d_b1;
        var_tdev_db2 = assign10_e327_d_b2;
        var_tdev_db3 = assign10_e327_d_b3;
        var_tdev_db4 = assign10_e327_d_b4;
        var_tdev_db5 = assign10_e327_d_b5;
        var_tdev_db6 = assign10_e327_d_b6;
        var_tdev_rv = 0.0;
        var_tdev_rdn0 = 0.0;
        var_tdev_rdn1 = 0.0;
        var_tdev_rdn2 = 0.0;
        var_tdev_rdn3 = 0.0;
        var_tdev_rdn4 = 0.0;
        var_tdev_rdn5 = 0.0;
        var_tdev_rdn6 = 0.0;
        var_tdev_rdb0 = 0.0;
        var_tdev_rdb1 = 0.0;
        var_tdev_rdb2 = 0.0;
        var_tdev_rdb3 = 0.0;
        var_tdev_rdb4 = 0.0;
        var_tdev_rdb5 = 0.0;
        var_tdev_rdb6 = 0.0;

        let assign40_e337: f64 = (p.p43 * p.p42);
        var_weff = assign40_e337;
        var_weff_dn0 = 0.0;
        var_weff_dn1 = 0.0;
        var_weff_dn2 = 0.0;
        var_weff_dn3 = 0.0;
        var_weff_dn4 = 0.0;
        var_weff_dn5 = 0.0;
        var_weff_dn6 = 0.0;
        var_weff_db0 = 0.0;
        var_weff_db1 = 0.0;
        var_weff_db2 = 0.0;
        var_weff_db3 = 0.0;
        var_weff_db4 = 0.0;
        var_weff_db5 = 0.0;
        var_weff_db6 = 0.0;
        var_weff_rv = 0.0;
        var_weff_rdn0 = 0.0;
        var_weff_rdn1 = 0.0;
        var_weff_rdn2 = 0.0;
        var_weff_rdn3 = 0.0;
        var_weff_rdn4 = 0.0;
        var_weff_rdn5 = 0.0;
        var_weff_rdn6 = 0.0;
        var_weff_rdb0 = 0.0;
        var_weff_rdb1 = 0.0;
        var_weff_rdb2 = 0.0;
        var_weff_rdb3 = 0.0;
        var_weff_rdb4 = 0.0;
        var_weff_rdb5 = 0.0;
        var_weff_rdb6 = 0.0;

        let assign50_e340: f64 = (p.p25 + 273.15);
        var_tnom = assign50_e340;
        var_tnom_dn0 = 0.0;
        var_tnom_dn1 = 0.0;
        var_tnom_dn2 = 0.0;
        var_tnom_dn3 = 0.0;
        var_tnom_dn4 = 0.0;
        var_tnom_dn5 = 0.0;
        var_tnom_dn6 = 0.0;
        var_tnom_db0 = 0.0;
        var_tnom_db1 = 0.0;
        var_tnom_db2 = 0.0;
        var_tnom_db3 = 0.0;
        var_tnom_db4 = 0.0;
        var_tnom_db5 = 0.0;
        var_tnom_db6 = 0.0;
        var_tnom_rv = 0.0;
        var_tnom_rdn0 = 0.0;
        var_tnom_rdn1 = 0.0;
        var_tnom_rdn2 = 0.0;
        var_tnom_rdn3 = 0.0;
        var_tnom_rdn4 = 0.0;
        var_tnom_rdn5 = 0.0;
        var_tnom_rdn6 = 0.0;
        var_tnom_rdb0 = 0.0;
        var_tnom_rdb1 = 0.0;
        var_tnom_rdb2 = 0.0;
        var_tnom_rdb3 = 0.0;
        var_tnom_rdb4 = 0.0;
        var_tnom_rdb5 = 0.0;
        var_tnom_rdb6 = 0.0;

        let assign60_e343: f64 = (8.6170869e-5 * var_tdev);
        var_vt = assign60_e343;
        var_vt_dn0 = (8.6170869e-5 * var_tdev_dn0);
        var_vt_dn1 = (8.6170869e-5 * var_tdev_dn1);
        var_vt_dn2 = (8.6170869e-5 * var_tdev_dn2);
        var_vt_dn3 = (8.6170869e-5 * var_tdev_dn3);
        var_vt_dn4 = (8.6170869e-5 * var_tdev_dn4);
        var_vt_dn5 = (8.6170869e-5 * var_tdev_dn5);
        var_vt_dn6 = (8.6170869e-5 * var_tdev_dn6);
        var_vt_db0 = (8.6170869e-5 * var_tdev_db0);
        var_vt_db1 = (8.6170869e-5 * var_tdev_db1);
        var_vt_db2 = (8.6170869e-5 * var_tdev_db2);
        var_vt_db3 = (8.6170869e-5 * var_tdev_db3);
        var_vt_db4 = (8.6170869e-5 * var_tdev_db4);
        var_vt_db5 = (8.6170869e-5 * var_tdev_db5);
        var_vt_db6 = (8.6170869e-5 * var_tdev_db6);
        var_vt_rv = 0.0;
        var_vt_rdn0 = 0.0;
        var_vt_rdn1 = 0.0;
        var_vt_rdn2 = 0.0;
        var_vt_rdn3 = 0.0;
        var_vt_rdn4 = 0.0;
        var_vt_rdn5 = 0.0;
        var_vt_rdn6 = 0.0;
        var_vt_rdb0 = 0.0;
        var_vt_rdb1 = 0.0;
        var_vt_rdb2 = 0.0;
        var_vt_rdb3 = 0.0;
        var_vt_rdb4 = 0.0;
        var_vt_rdb5 = 0.0;
        var_vt_rdb6 = 0.0;

        let assign70_e346: f64 = (var_tdev / var_tnom);
        var_rt = assign70_e346;
        var_rt_dn0 = (((var_tdev_dn0 * var_tnom) - (var_tdev * var_tnom_dn0)) / (var_tnom * var_tnom));
        var_rt_dn1 = (((var_tdev_dn1 * var_tnom) - (var_tdev * var_tnom_dn1)) / (var_tnom * var_tnom));
        var_rt_dn2 = (((var_tdev_dn2 * var_tnom) - (var_tdev * var_tnom_dn2)) / (var_tnom * var_tnom));
        var_rt_dn3 = (((var_tdev_dn3 * var_tnom) - (var_tdev * var_tnom_dn3)) / (var_tnom * var_tnom));
        var_rt_dn4 = (((var_tdev_dn4 * var_tnom) - (var_tdev * var_tnom_dn4)) / (var_tnom * var_tnom));
        var_rt_dn5 = (((var_tdev_dn5 * var_tnom) - (var_tdev * var_tnom_dn5)) / (var_tnom * var_tnom));
        var_rt_dn6 = (((var_tdev_dn6 * var_tnom) - (var_tdev * var_tnom_dn6)) / (var_tnom * var_tnom));
        var_rt_db0 = (((var_tdev_db0 * var_tnom) - (var_tdev * var_tnom_db0)) / (var_tnom * var_tnom));
        var_rt_db1 = (((var_tdev_db1 * var_tnom) - (var_tdev * var_tnom_db1)) / (var_tnom * var_tnom));
        var_rt_db2 = (((var_tdev_db2 * var_tnom) - (var_tdev * var_tnom_db2)) / (var_tnom * var_tnom));
        var_rt_db3 = (((var_tdev_db3 * var_tnom) - (var_tdev * var_tnom_db3)) / (var_tnom * var_tnom));
        var_rt_db4 = (((var_tdev_db4 * var_tnom) - (var_tdev * var_tnom_db4)) / (var_tnom * var_tnom));
        var_rt_db5 = (((var_tdev_db5 * var_tnom) - (var_tdev * var_tnom_db5)) / (var_tnom * var_tnom));
        var_rt_db6 = (((var_tdev_db6 * var_tnom) - (var_tdev * var_tnom_db6)) / (var_tnom * var_tnom));
        var_rt_rv = 0.0;
        var_rt_rdn0 = 0.0;
        var_rt_rdn1 = 0.0;
        var_rt_rdn2 = 0.0;
        var_rt_rdn3 = 0.0;
        var_rt_rdn4 = 0.0;
        var_rt_rdn5 = 0.0;
        var_rt_rdn6 = 0.0;
        var_rt_rdb0 = 0.0;
        var_rt_rdb1 = 0.0;
        var_rt_rdb2 = 0.0;
        var_rt_rdb3 = 0.0;
        var_rt_rdb4 = 0.0;
        var_rt_rdb5 = 0.0;
        var_rt_rdb6 = 0.0;

        let assign80_e348: f64 = (var_rt).ln();
        var_lnrt = assign80_e348;
        var_lnrt_dn0 = (var_rt_dn0 / var_rt);
        var_lnrt_dn1 = (var_rt_dn1 / var_rt);
        var_lnrt_dn2 = (var_rt_dn2 / var_rt);
        var_lnrt_dn3 = (var_rt_dn3 / var_rt);
        var_lnrt_dn4 = (var_rt_dn4 / var_rt);
        var_lnrt_dn5 = (var_rt_dn5 / var_rt);
        var_lnrt_dn6 = (var_rt_dn6 / var_rt);
        var_lnrt_db0 = (var_rt_db0 / var_rt);
        var_lnrt_db1 = (var_rt_db1 / var_rt);
        var_lnrt_db2 = (var_rt_db2 / var_rt);
        var_lnrt_db3 = (var_rt_db3 / var_rt);
        var_lnrt_db4 = (var_rt_db4 / var_rt);
        var_lnrt_db5 = (var_rt_db5 / var_rt);
        var_lnrt_db6 = (var_rt_db6 / var_rt);
        var_lnrt_rv = 0.0;
        var_lnrt_rdn0 = 0.0;
        var_lnrt_rdn1 = 0.0;
        var_lnrt_rdn2 = 0.0;
        var_lnrt_rdn3 = 0.0;
        var_lnrt_rdn4 = 0.0;
        var_lnrt_rdn5 = 0.0;
        var_lnrt_rdn6 = 0.0;
        var_lnrt_rdb0 = 0.0;
        var_lnrt_rdb1 = 0.0;
        var_lnrt_rdb2 = 0.0;
        var_lnrt_rdb3 = 0.0;
        var_lnrt_rdb4 = 0.0;
        var_lnrt_rdb5 = 0.0;
        var_lnrt_rdb6 = 0.0;

        let assign90_e351: f64 = (p.p22 * var_lnrt);
        let assign90_e355: f64 = (var_rt - 1.0);
        let assign90_e356: f64 = (p.p21 * assign90_e355);
        let assign90_e358: f64 = (assign90_e356 / var_vt);
        let assign90_e359: f64 = (assign90_e351 + assign90_e358);
        var_argt = assign90_e359;
        var_argt_dn0 = ((p.p22 * var_lnrt_dn0) + ((((p.p21 * var_rt_dn0) * var_vt) - (assign90_e356 * var_vt_dn0)) / (var_vt * var_vt)));
        var_argt_dn1 = ((p.p22 * var_lnrt_dn1) + ((((p.p21 * var_rt_dn1) * var_vt) - (assign90_e356 * var_vt_dn1)) / (var_vt * var_vt)));
        var_argt_dn2 = ((p.p22 * var_lnrt_dn2) + ((((p.p21 * var_rt_dn2) * var_vt) - (assign90_e356 * var_vt_dn2)) / (var_vt * var_vt)));
        var_argt_dn3 = ((p.p22 * var_lnrt_dn3) + ((((p.p21 * var_rt_dn3) * var_vt) - (assign90_e356 * var_vt_dn3)) / (var_vt * var_vt)));
        var_argt_dn4 = ((p.p22 * var_lnrt_dn4) + ((((p.p21 * var_rt_dn4) * var_vt) - (assign90_e356 * var_vt_dn4)) / (var_vt * var_vt)));
        var_argt_dn5 = ((p.p22 * var_lnrt_dn5) + ((((p.p21 * var_rt_dn5) * var_vt) - (assign90_e356 * var_vt_dn5)) / (var_vt * var_vt)));
        var_argt_dn6 = ((p.p22 * var_lnrt_dn6) + ((((p.p21 * var_rt_dn6) * var_vt) - (assign90_e356 * var_vt_dn6)) / (var_vt * var_vt)));
        var_argt_db0 = ((p.p22 * var_lnrt_db0) + ((((p.p21 * var_rt_db0) * var_vt) - (assign90_e356 * var_vt_db0)) / (var_vt * var_vt)));
        var_argt_db1 = ((p.p22 * var_lnrt_db1) + ((((p.p21 * var_rt_db1) * var_vt) - (assign90_e356 * var_vt_db1)) / (var_vt * var_vt)));
        var_argt_db2 = ((p.p22 * var_lnrt_db2) + ((((p.p21 * var_rt_db2) * var_vt) - (assign90_e356 * var_vt_db2)) / (var_vt * var_vt)));
        var_argt_db3 = ((p.p22 * var_lnrt_db3) + ((((p.p21 * var_rt_db3) * var_vt) - (assign90_e356 * var_vt_db3)) / (var_vt * var_vt)));
        var_argt_db4 = ((p.p22 * var_lnrt_db4) + ((((p.p21 * var_rt_db4) * var_vt) - (assign90_e356 * var_vt_db4)) / (var_vt * var_vt)));
        var_argt_db5 = ((p.p22 * var_lnrt_db5) + ((((p.p21 * var_rt_db5) * var_vt) - (assign90_e356 * var_vt_db5)) / (var_vt * var_vt)));
        var_argt_db6 = ((p.p22 * var_lnrt_db6) + ((((p.p21 * var_rt_db6) * var_vt) - (assign90_e356 * var_vt_db6)) / (var_vt * var_vt)));
        var_argt_rv = 0.0;
        var_argt_rdn0 = 0.0;
        var_argt_rdn1 = 0.0;
        var_argt_rdn2 = 0.0;
        var_argt_rdn3 = 0.0;
        var_argt_rdn4 = 0.0;
        var_argt_rdn5 = 0.0;
        var_argt_rdn6 = 0.0;
        var_argt_rdb0 = 0.0;
        var_argt_rdb1 = 0.0;
        var_argt_rdb2 = 0.0;
        var_argt_rdb3 = 0.0;
        var_argt_rdb4 = 0.0;
        var_argt_rdb5 = 0.0;
        var_argt_rdb6 = 0.0;

        let assign100_e362: f64 = (p.p23 * var_lnrt);
        var_argtr = assign100_e362;
        var_argtr_dn0 = (p.p23 * var_lnrt_dn0);
        var_argtr_dn1 = (p.p23 * var_lnrt_dn1);
        var_argtr_dn2 = (p.p23 * var_lnrt_dn2);
        var_argtr_dn3 = (p.p23 * var_lnrt_dn3);
        var_argtr_dn4 = (p.p23 * var_lnrt_dn4);
        var_argtr_dn5 = (p.p23 * var_lnrt_dn5);
        var_argtr_dn6 = (p.p23 * var_lnrt_dn6);
        var_argtr_db0 = (p.p23 * var_lnrt_db0);
        var_argtr_db1 = (p.p23 * var_lnrt_db1);
        var_argtr_db2 = (p.p23 * var_lnrt_db2);
        var_argtr_db3 = (p.p23 * var_lnrt_db3);
        var_argtr_db4 = (p.p23 * var_lnrt_db4);
        var_argtr_db5 = (p.p23 * var_lnrt_db5);
        var_argtr_db6 = (p.p23 * var_lnrt_db6);
        var_argtr_rv = 0.0;
        var_argtr_rdn0 = 0.0;
        var_argtr_rdn1 = 0.0;
        var_argtr_rdn2 = 0.0;
        var_argtr_rdn3 = 0.0;
        var_argtr_rdn4 = 0.0;
        var_argtr_rdn5 = 0.0;
        var_argtr_rdn6 = 0.0;
        var_argtr_rdb0 = 0.0;
        var_argtr_rdb1 = 0.0;
        var_argtr_rdb2 = 0.0;
        var_argtr_rdb3 = 0.0;
        var_argtr_rdb4 = 0.0;
        var_argtr_rdb5 = 0.0;
        var_argtr_rdb6 = 0.0;

        let assign110_e365: f64 = (var_argt).exp();
        let assign110_e366: f64 = (p.p0 * assign110_e365);
        var_is_t = assign110_e366;
        var_is_t_dn0 = (p.p0 * (assign110_e365 * var_argt_dn0));
        var_is_t_dn1 = (p.p0 * (assign110_e365 * var_argt_dn1));
        var_is_t_dn2 = (p.p0 * (assign110_e365 * var_argt_dn2));
        var_is_t_dn3 = (p.p0 * (assign110_e365 * var_argt_dn3));
        var_is_t_dn4 = (p.p0 * (assign110_e365 * var_argt_dn4));
        var_is_t_dn5 = (p.p0 * (assign110_e365 * var_argt_dn5));
        var_is_t_dn6 = (p.p0 * (assign110_e365 * var_argt_dn6));
        var_is_t_db0 = (p.p0 * (assign110_e365 * var_argt_db0));
        var_is_t_db1 = (p.p0 * (assign110_e365 * var_argt_db1));
        var_is_t_db2 = (p.p0 * (assign110_e365 * var_argt_db2));
        var_is_t_db3 = (p.p0 * (assign110_e365 * var_argt_db3));
        var_is_t_db4 = (p.p0 * (assign110_e365 * var_argt_db4));
        var_is_t_db5 = (p.p0 * (assign110_e365 * var_argt_db5));
        var_is_t_db6 = (p.p0 * (assign110_e365 * var_argt_db6));
        var_is_t_rv = 0.0;
        var_is_t_rdn0 = 0.0;
        var_is_t_rdn1 = 0.0;
        var_is_t_rdn2 = 0.0;
        var_is_t_rdn3 = 0.0;
        var_is_t_rdn4 = 0.0;
        var_is_t_rdn5 = 0.0;
        var_is_t_rdn6 = 0.0;
        var_is_t_rdb0 = 0.0;
        var_is_t_rdb1 = 0.0;
        var_is_t_rdb2 = 0.0;
        var_is_t_rdb3 = 0.0;
        var_is_t_rdb4 = 0.0;
        var_is_t_rdb5 = 0.0;
        var_is_t_rdb6 = 0.0;

        let assign120_e369: f64 = (var_argtr).exp();
        let assign120_e370: f64 = (p.p2 * assign120_e369);
        var_isr_t = assign120_e370;
        var_isr_t_dn0 = (p.p2 * (assign120_e369 * var_argtr_dn0));
        var_isr_t_dn1 = (p.p2 * (assign120_e369 * var_argtr_dn1));
        var_isr_t_dn2 = (p.p2 * (assign120_e369 * var_argtr_dn2));
        var_isr_t_dn3 = (p.p2 * (assign120_e369 * var_argtr_dn3));
        var_isr_t_dn4 = (p.p2 * (assign120_e369 * var_argtr_dn4));
        var_isr_t_dn5 = (p.p2 * (assign120_e369 * var_argtr_dn5));
        var_isr_t_dn6 = (p.p2 * (assign120_e369 * var_argtr_dn6));
        var_isr_t_db0 = (p.p2 * (assign120_e369 * var_argtr_db0));
        var_isr_t_db1 = (p.p2 * (assign120_e369 * var_argtr_db1));
        var_isr_t_db2 = (p.p2 * (assign120_e369 * var_argtr_db2));
        var_isr_t_db3 = (p.p2 * (assign120_e369 * var_argtr_db3));
        var_isr_t_db4 = (p.p2 * (assign120_e369 * var_argtr_db4));
        var_isr_t_db5 = (p.p2 * (assign120_e369 * var_argtr_db5));
        var_isr_t_db6 = (p.p2 * (assign120_e369 * var_argtr_db6));
        var_isr_t_rv = 0.0;
        var_isr_t_rdn0 = 0.0;
        var_isr_t_rdn1 = 0.0;
        var_isr_t_rdn2 = 0.0;
        var_isr_t_rdn3 = 0.0;
        var_isr_t_rdn4 = 0.0;
        var_isr_t_rdn5 = 0.0;
        var_isr_t_rdn6 = 0.0;
        var_isr_t_rdb0 = 0.0;
        var_isr_t_rdb1 = 0.0;
        var_isr_t_rdb2 = 0.0;
        var_isr_t_rdb3 = 0.0;
        var_isr_t_rdb4 = 0.0;
        var_isr_t_rdb5 = 0.0;
        var_isr_t_rdb6 = 0.0;

        let assign130_e376: f64 = (var_rt - 1.0);
        let assign130_e377: f64 = (p.p7 * assign130_e376);
        let assign130_e378: f64 = (1.0 + assign130_e377);
        let assign130_e379: f64 = (p.p47 * assign130_e378);
        var_ijbv_t = assign130_e379;
        var_ijbv_t_dn0 = (p.p47 * (p.p7 * var_rt_dn0));
        var_ijbv_t_dn1 = (p.p47 * (p.p7 * var_rt_dn1));
        var_ijbv_t_dn2 = (p.p47 * (p.p7 * var_rt_dn2));
        var_ijbv_t_dn3 = (p.p47 * (p.p7 * var_rt_dn3));
        var_ijbv_t_dn4 = (p.p47 * (p.p7 * var_rt_dn4));
        var_ijbv_t_dn5 = (p.p47 * (p.p7 * var_rt_dn5));
        var_ijbv_t_dn6 = (p.p47 * (p.p7 * var_rt_dn6));
        var_ijbv_t_db0 = (p.p47 * (p.p7 * var_rt_db0));
        var_ijbv_t_db1 = (p.p47 * (p.p7 * var_rt_db1));
        var_ijbv_t_db2 = (p.p47 * (p.p7 * var_rt_db2));
        var_ijbv_t_db3 = (p.p47 * (p.p7 * var_rt_db3));
        var_ijbv_t_db4 = (p.p47 * (p.p7 * var_rt_db4));
        var_ijbv_t_db5 = (p.p47 * (p.p7 * var_rt_db5));
        var_ijbv_t_db6 = (p.p47 * (p.p7 * var_rt_db6));
        var_ijbv_t_rv = 0.0;
        var_ijbv_t_rdn0 = 0.0;
        var_ijbv_t_rdn1 = 0.0;
        var_ijbv_t_rdn2 = 0.0;
        var_ijbv_t_rdn3 = 0.0;
        var_ijbv_t_rdn4 = 0.0;
        var_ijbv_t_rdn5 = 0.0;
        var_ijbv_t_rdn6 = 0.0;
        var_ijbv_t_rdb0 = 0.0;
        var_ijbv_t_rdb1 = 0.0;
        var_ijbv_t_rdb2 = 0.0;
        var_ijbv_t_rdb3 = 0.0;
        var_ijbv_t_rdb4 = 0.0;
        var_ijbv_t_rdb5 = 0.0;
        var_ijbv_t_rdb6 = 0.0;

        let assign140_e385: f64 = (var_rt - 1.0);
        let assign140_e386: f64 = (p.p6 * assign140_e385);
        let assign140_e387: f64 = (1.0 + assign140_e386);
        let assign140_e388: f64 = (p.p5 * assign140_e387);
        var_bvr_t = assign140_e388;
        var_bvr_t_dn0 = (p.p5 * (p.p6 * var_rt_dn0));
        var_bvr_t_dn1 = (p.p5 * (p.p6 * var_rt_dn1));
        var_bvr_t_dn2 = (p.p5 * (p.p6 * var_rt_dn2));
        var_bvr_t_dn3 = (p.p5 * (p.p6 * var_rt_dn3));
        var_bvr_t_dn4 = (p.p5 * (p.p6 * var_rt_dn4));
        var_bvr_t_dn5 = (p.p5 * (p.p6 * var_rt_dn5));
        var_bvr_t_dn6 = (p.p5 * (p.p6 * var_rt_dn6));
        var_bvr_t_db0 = (p.p5 * (p.p6 * var_rt_db0));
        var_bvr_t_db1 = (p.p5 * (p.p6 * var_rt_db1));
        var_bvr_t_db2 = (p.p5 * (p.p6 * var_rt_db2));
        var_bvr_t_db3 = (p.p5 * (p.p6 * var_rt_db3));
        var_bvr_t_db4 = (p.p5 * (p.p6 * var_rt_db4));
        var_bvr_t_db5 = (p.p5 * (p.p6 * var_rt_db5));
        var_bvr_t_db6 = (p.p5 * (p.p6 * var_rt_db6));
        var_bvr_t_rv = 0.0;
        var_bvr_t_rdn0 = 0.0;
        var_bvr_t_rdn1 = 0.0;
        var_bvr_t_rdn2 = 0.0;
        var_bvr_t_rdn3 = 0.0;
        var_bvr_t_rdn4 = 0.0;
        var_bvr_t_rdn5 = 0.0;
        var_bvr_t_rdn6 = 0.0;
        var_bvr_t_rdb0 = 0.0;
        var_bvr_t_rdb1 = 0.0;
        var_bvr_t_rdb2 = 0.0;
        var_bvr_t_rdb3 = 0.0;
        var_bvr_t_rdb4 = 0.0;
        var_bvr_t_rdb5 = 0.0;
        var_bvr_t_rdb6 = 0.0;

        let assign150_e394: f64 = (var_rt - 1.0);
        let assign150_e395: f64 = (p.p10 * assign150_e394);
        let assign150_e396: f64 = (1.0 + assign150_e395);
        let assign150_e397: f64 = (p.p9 * assign150_e396);
        var_theexp_t = assign150_e397;
        var_theexp_t_dn0 = (p.p9 * (p.p10 * var_rt_dn0));
        var_theexp_t_dn1 = (p.p9 * (p.p10 * var_rt_dn1));
        var_theexp_t_dn2 = (p.p9 * (p.p10 * var_rt_dn2));
        var_theexp_t_dn3 = (p.p9 * (p.p10 * var_rt_dn3));
        var_theexp_t_dn4 = (p.p9 * (p.p10 * var_rt_dn4));
        var_theexp_t_dn5 = (p.p9 * (p.p10 * var_rt_dn5));
        var_theexp_t_dn6 = (p.p9 * (p.p10 * var_rt_dn6));
        var_theexp_t_db0 = (p.p9 * (p.p10 * var_rt_db0));
        var_theexp_t_db1 = (p.p9 * (p.p10 * var_rt_db1));
        var_theexp_t_db2 = (p.p9 * (p.p10 * var_rt_db2));
        var_theexp_t_db3 = (p.p9 * (p.p10 * var_rt_db3));
        var_theexp_t_db4 = (p.p9 * (p.p10 * var_rt_db4));
        var_theexp_t_db5 = (p.p9 * (p.p10 * var_rt_db5));
        var_theexp_t_db6 = (p.p9 * (p.p10 * var_rt_db6));
        var_theexp_t_rv = 0.0;
        var_theexp_t_rdn0 = 0.0;
        var_theexp_t_rdn1 = 0.0;
        var_theexp_t_rdn2 = 0.0;
        var_theexp_t_rdn3 = 0.0;
        var_theexp_t_rdn4 = 0.0;
        var_theexp_t_rdn5 = 0.0;
        var_theexp_t_rdn6 = 0.0;
        var_theexp_t_rdb0 = 0.0;
        var_theexp_t_rdb1 = 0.0;
        var_theexp_t_rdb2 = 0.0;
        var_theexp_t_rdb3 = 0.0;
        var_theexp_t_rdb4 = 0.0;
        var_theexp_t_rdb5 = 0.0;
        var_theexp_t_rdb6 = 0.0;

        *var_argt_slot = var_argt;
        *var_argt_db0_slot = var_argt_db0;
        *var_argt_db1_slot = var_argt_db1;
        *var_argt_db2_slot = var_argt_db2;
        *var_argt_db3_slot = var_argt_db3;
        *var_argt_db4_slot = var_argt_db4;
        *var_argt_db5_slot = var_argt_db5;
        *var_argt_db6_slot = var_argt_db6;
        *var_argt_dn0_slot = var_argt_dn0;
        *var_argt_dn1_slot = var_argt_dn1;
        *var_argt_dn2_slot = var_argt_dn2;
        *var_argt_dn3_slot = var_argt_dn3;
        *var_argt_dn4_slot = var_argt_dn4;
        *var_argt_dn5_slot = var_argt_dn5;
        *var_argt_dn6_slot = var_argt_dn6;
        *var_argt_rdb0_slot = var_argt_rdb0;
        *var_argt_rdb1_slot = var_argt_rdb1;
        *var_argt_rdb2_slot = var_argt_rdb2;
        *var_argt_rdb3_slot = var_argt_rdb3;
        *var_argt_rdb4_slot = var_argt_rdb4;
        *var_argt_rdb5_slot = var_argt_rdb5;
        *var_argt_rdb6_slot = var_argt_rdb6;
        *var_argt_rdn0_slot = var_argt_rdn0;
        *var_argt_rdn1_slot = var_argt_rdn1;
        *var_argt_rdn2_slot = var_argt_rdn2;
        *var_argt_rdn3_slot = var_argt_rdn3;
        *var_argt_rdn4_slot = var_argt_rdn4;
        *var_argt_rdn5_slot = var_argt_rdn5;
        *var_argt_rdn6_slot = var_argt_rdn6;
        *var_argt_rv_slot = var_argt_rv;
        *var_argtr_slot = var_argtr;
        *var_argtr_db0_slot = var_argtr_db0;
        *var_argtr_db1_slot = var_argtr_db1;
        *var_argtr_db2_slot = var_argtr_db2;
        *var_argtr_db3_slot = var_argtr_db3;
        *var_argtr_db4_slot = var_argtr_db4;
        *var_argtr_db5_slot = var_argtr_db5;
        *var_argtr_db6_slot = var_argtr_db6;
        *var_argtr_dn0_slot = var_argtr_dn0;
        *var_argtr_dn1_slot = var_argtr_dn1;
        *var_argtr_dn2_slot = var_argtr_dn2;
        *var_argtr_dn3_slot = var_argtr_dn3;
        *var_argtr_dn4_slot = var_argtr_dn4;
        *var_argtr_dn5_slot = var_argtr_dn5;
        *var_argtr_dn6_slot = var_argtr_dn6;
        *var_argtr_rdb0_slot = var_argtr_rdb0;
        *var_argtr_rdb1_slot = var_argtr_rdb1;
        *var_argtr_rdb2_slot = var_argtr_rdb2;
        *var_argtr_rdb3_slot = var_argtr_rdb3;
        *var_argtr_rdb4_slot = var_argtr_rdb4;
        *var_argtr_rdb5_slot = var_argtr_rdb5;
        *var_argtr_rdb6_slot = var_argtr_rdb6;
        *var_argtr_rdn0_slot = var_argtr_rdn0;
        *var_argtr_rdn1_slot = var_argtr_rdn1;
        *var_argtr_rdn2_slot = var_argtr_rdn2;
        *var_argtr_rdn3_slot = var_argtr_rdn3;
        *var_argtr_rdn4_slot = var_argtr_rdn4;
        *var_argtr_rdn5_slot = var_argtr_rdn5;
        *var_argtr_rdn6_slot = var_argtr_rdn6;
        *var_argtr_rv_slot = var_argtr_rv;
        *var_bvr_t_slot = var_bvr_t;
        *var_bvr_t_db0_slot = var_bvr_t_db0;
        *var_bvr_t_db1_slot = var_bvr_t_db1;
        *var_bvr_t_db2_slot = var_bvr_t_db2;
        *var_bvr_t_db3_slot = var_bvr_t_db3;
        *var_bvr_t_db4_slot = var_bvr_t_db4;
        *var_bvr_t_db5_slot = var_bvr_t_db5;
        *var_bvr_t_db6_slot = var_bvr_t_db6;
        *var_bvr_t_dn0_slot = var_bvr_t_dn0;
        *var_bvr_t_dn1_slot = var_bvr_t_dn1;
        *var_bvr_t_dn2_slot = var_bvr_t_dn2;
        *var_bvr_t_dn3_slot = var_bvr_t_dn3;
        *var_bvr_t_dn4_slot = var_bvr_t_dn4;
        *var_bvr_t_dn5_slot = var_bvr_t_dn5;
        *var_bvr_t_dn6_slot = var_bvr_t_dn6;
        *var_bvr_t_rdb0_slot = var_bvr_t_rdb0;
        *var_bvr_t_rdb1_slot = var_bvr_t_rdb1;
        *var_bvr_t_rdb2_slot = var_bvr_t_rdb2;
        *var_bvr_t_rdb3_slot = var_bvr_t_rdb3;
        *var_bvr_t_rdb4_slot = var_bvr_t_rdb4;
        *var_bvr_t_rdb5_slot = var_bvr_t_rdb5;
        *var_bvr_t_rdb6_slot = var_bvr_t_rdb6;
        *var_bvr_t_rdn0_slot = var_bvr_t_rdn0;
        *var_bvr_t_rdn1_slot = var_bvr_t_rdn1;
        *var_bvr_t_rdn2_slot = var_bvr_t_rdn2;
        *var_bvr_t_rdn3_slot = var_bvr_t_rdn3;
        *var_bvr_t_rdn4_slot = var_bvr_t_rdn4;
        *var_bvr_t_rdn5_slot = var_bvr_t_rdn5;
        *var_bvr_t_rdn6_slot = var_bvr_t_rdn6;
        *var_bvr_t_rv_slot = var_bvr_t_rv;
        *var_ijbv_t_slot = var_ijbv_t;
        *var_ijbv_t_db0_slot = var_ijbv_t_db0;
        *var_ijbv_t_db1_slot = var_ijbv_t_db1;
        *var_ijbv_t_db2_slot = var_ijbv_t_db2;
        *var_ijbv_t_db3_slot = var_ijbv_t_db3;
        *var_ijbv_t_db4_slot = var_ijbv_t_db4;
        *var_ijbv_t_db5_slot = var_ijbv_t_db5;
        *var_ijbv_t_db6_slot = var_ijbv_t_db6;
        *var_ijbv_t_dn0_slot = var_ijbv_t_dn0;
        *var_ijbv_t_dn1_slot = var_ijbv_t_dn1;
        *var_ijbv_t_dn2_slot = var_ijbv_t_dn2;
        *var_ijbv_t_dn3_slot = var_ijbv_t_dn3;
        *var_ijbv_t_dn4_slot = var_ijbv_t_dn4;
        *var_ijbv_t_dn5_slot = var_ijbv_t_dn5;
        *var_ijbv_t_dn6_slot = var_ijbv_t_dn6;
        *var_ijbv_t_rdb0_slot = var_ijbv_t_rdb0;
        *var_ijbv_t_rdb1_slot = var_ijbv_t_rdb1;
        *var_ijbv_t_rdb2_slot = var_ijbv_t_rdb2;
        *var_ijbv_t_rdb3_slot = var_ijbv_t_rdb3;
        *var_ijbv_t_rdb4_slot = var_ijbv_t_rdb4;
        *var_ijbv_t_rdb5_slot = var_ijbv_t_rdb5;
        *var_ijbv_t_rdb6_slot = var_ijbv_t_rdb6;
        *var_ijbv_t_rdn0_slot = var_ijbv_t_rdn0;
        *var_ijbv_t_rdn1_slot = var_ijbv_t_rdn1;
        *var_ijbv_t_rdn2_slot = var_ijbv_t_rdn2;
        *var_ijbv_t_rdn3_slot = var_ijbv_t_rdn3;
        *var_ijbv_t_rdn4_slot = var_ijbv_t_rdn4;
        *var_ijbv_t_rdn5_slot = var_ijbv_t_rdn5;
        *var_ijbv_t_rdn6_slot = var_ijbv_t_rdn6;
        *var_ijbv_t_rv_slot = var_ijbv_t_rv;
        *var_is_t_slot = var_is_t;
        *var_is_t_db0_slot = var_is_t_db0;
        *var_is_t_db1_slot = var_is_t_db1;
        *var_is_t_db2_slot = var_is_t_db2;
        *var_is_t_db3_slot = var_is_t_db3;
        *var_is_t_db4_slot = var_is_t_db4;
        *var_is_t_db5_slot = var_is_t_db5;
        *var_is_t_db6_slot = var_is_t_db6;
        *var_is_t_dn0_slot = var_is_t_dn0;
        *var_is_t_dn1_slot = var_is_t_dn1;
        *var_is_t_dn2_slot = var_is_t_dn2;
        *var_is_t_dn3_slot = var_is_t_dn3;
        *var_is_t_dn4_slot = var_is_t_dn4;
        *var_is_t_dn5_slot = var_is_t_dn5;
        *var_is_t_dn6_slot = var_is_t_dn6;
        *var_is_t_rdb0_slot = var_is_t_rdb0;
        *var_is_t_rdb1_slot = var_is_t_rdb1;
        *var_is_t_rdb2_slot = var_is_t_rdb2;
        *var_is_t_rdb3_slot = var_is_t_rdb3;
        *var_is_t_rdb4_slot = var_is_t_rdb4;
        *var_is_t_rdb5_slot = var_is_t_rdb5;
        *var_is_t_rdb6_slot = var_is_t_rdb6;
        *var_is_t_rdn0_slot = var_is_t_rdn0;
        *var_is_t_rdn1_slot = var_is_t_rdn1;
        *var_is_t_rdn2_slot = var_is_t_rdn2;
        *var_is_t_rdn3_slot = var_is_t_rdn3;
        *var_is_t_rdn4_slot = var_is_t_rdn4;
        *var_is_t_rdn5_slot = var_is_t_rdn5;
        *var_is_t_rdn6_slot = var_is_t_rdn6;
        *var_is_t_rv_slot = var_is_t_rv;
        *var_isr_t_slot = var_isr_t;
        *var_isr_t_db0_slot = var_isr_t_db0;
        *var_isr_t_db1_slot = var_isr_t_db1;
        *var_isr_t_db2_slot = var_isr_t_db2;
        *var_isr_t_db3_slot = var_isr_t_db3;
        *var_isr_t_db4_slot = var_isr_t_db4;
        *var_isr_t_db5_slot = var_isr_t_db5;
        *var_isr_t_db6_slot = var_isr_t_db6;
        *var_isr_t_dn0_slot = var_isr_t_dn0;
        *var_isr_t_dn1_slot = var_isr_t_dn1;
        *var_isr_t_dn2_slot = var_isr_t_dn2;
        *var_isr_t_dn3_slot = var_isr_t_dn3;
        *var_isr_t_dn4_slot = var_isr_t_dn4;
        *var_isr_t_dn5_slot = var_isr_t_dn5;
        *var_isr_t_dn6_slot = var_isr_t_dn6;
        *var_isr_t_rdb0_slot = var_isr_t_rdb0;
        *var_isr_t_rdb1_slot = var_isr_t_rdb1;
        *var_isr_t_rdb2_slot = var_isr_t_rdb2;
        *var_isr_t_rdb3_slot = var_isr_t_rdb3;
        *var_isr_t_rdb4_slot = var_isr_t_rdb4;
        *var_isr_t_rdb5_slot = var_isr_t_rdb5;
        *var_isr_t_rdb6_slot = var_isr_t_rdb6;
        *var_isr_t_rdn0_slot = var_isr_t_rdn0;
        *var_isr_t_rdn1_slot = var_isr_t_rdn1;
        *var_isr_t_rdn2_slot = var_isr_t_rdn2;
        *var_isr_t_rdn3_slot = var_isr_t_rdn3;
        *var_isr_t_rdn4_slot = var_isr_t_rdn4;
        *var_isr_t_rdn5_slot = var_isr_t_rdn5;
        *var_isr_t_rdn6_slot = var_isr_t_rdn6;
        *var_isr_t_rv_slot = var_isr_t_rv;
        *var_lnrt_slot = var_lnrt;
        *var_lnrt_db0_slot = var_lnrt_db0;
        *var_lnrt_db1_slot = var_lnrt_db1;
        *var_lnrt_db2_slot = var_lnrt_db2;
        *var_lnrt_db3_slot = var_lnrt_db3;
        *var_lnrt_db4_slot = var_lnrt_db4;
        *var_lnrt_db5_slot = var_lnrt_db5;
        *var_lnrt_db6_slot = var_lnrt_db6;
        *var_lnrt_dn0_slot = var_lnrt_dn0;
        *var_lnrt_dn1_slot = var_lnrt_dn1;
        *var_lnrt_dn2_slot = var_lnrt_dn2;
        *var_lnrt_dn3_slot = var_lnrt_dn3;
        *var_lnrt_dn4_slot = var_lnrt_dn4;
        *var_lnrt_dn5_slot = var_lnrt_dn5;
        *var_lnrt_dn6_slot = var_lnrt_dn6;
        *var_lnrt_rdb0_slot = var_lnrt_rdb0;
        *var_lnrt_rdb1_slot = var_lnrt_rdb1;
        *var_lnrt_rdb2_slot = var_lnrt_rdb2;
        *var_lnrt_rdb3_slot = var_lnrt_rdb3;
        *var_lnrt_rdb4_slot = var_lnrt_rdb4;
        *var_lnrt_rdb5_slot = var_lnrt_rdb5;
        *var_lnrt_rdb6_slot = var_lnrt_rdb6;
        *var_lnrt_rdn0_slot = var_lnrt_rdn0;
        *var_lnrt_rdn1_slot = var_lnrt_rdn1;
        *var_lnrt_rdn2_slot = var_lnrt_rdn2;
        *var_lnrt_rdn3_slot = var_lnrt_rdn3;
        *var_lnrt_rdn4_slot = var_lnrt_rdn4;
        *var_lnrt_rdn5_slot = var_lnrt_rdn5;
        *var_lnrt_rdn6_slot = var_lnrt_rdn6;
        *var_lnrt_rv_slot = var_lnrt_rv;
        *var_rt_slot = var_rt;
        *var_rt_db0_slot = var_rt_db0;
        *var_rt_db1_slot = var_rt_db1;
        *var_rt_db2_slot = var_rt_db2;
        *var_rt_db3_slot = var_rt_db3;
        *var_rt_db4_slot = var_rt_db4;
        *var_rt_db5_slot = var_rt_db5;
        *var_rt_db6_slot = var_rt_db6;
        *var_rt_dn0_slot = var_rt_dn0;
        *var_rt_dn1_slot = var_rt_dn1;
        *var_rt_dn2_slot = var_rt_dn2;
        *var_rt_dn3_slot = var_rt_dn3;
        *var_rt_dn4_slot = var_rt_dn4;
        *var_rt_dn5_slot = var_rt_dn5;
        *var_rt_dn6_slot = var_rt_dn6;
        *var_rt_rdb0_slot = var_rt_rdb0;
        *var_rt_rdb1_slot = var_rt_rdb1;
        *var_rt_rdb2_slot = var_rt_rdb2;
        *var_rt_rdb3_slot = var_rt_rdb3;
        *var_rt_rdb4_slot = var_rt_rdb4;
        *var_rt_rdb5_slot = var_rt_rdb5;
        *var_rt_rdb6_slot = var_rt_rdb6;
        *var_rt_rdn0_slot = var_rt_rdn0;
        *var_rt_rdn1_slot = var_rt_rdn1;
        *var_rt_rdn2_slot = var_rt_rdn2;
        *var_rt_rdn3_slot = var_rt_rdn3;
        *var_rt_rdn4_slot = var_rt_rdn4;
        *var_rt_rdn5_slot = var_rt_rdn5;
        *var_rt_rdn6_slot = var_rt_rdn6;
        *var_rt_rv_slot = var_rt_rv;
        *var_tamb_slot = var_tamb;
        *var_tamb_db0_slot = var_tamb_db0;
        *var_tamb_db1_slot = var_tamb_db1;
        *var_tamb_db2_slot = var_tamb_db2;
        *var_tamb_db3_slot = var_tamb_db3;
        *var_tamb_db4_slot = var_tamb_db4;
        *var_tamb_db5_slot = var_tamb_db5;
        *var_tamb_db6_slot = var_tamb_db6;
        *var_tamb_dn0_slot = var_tamb_dn0;
        *var_tamb_dn1_slot = var_tamb_dn1;
        *var_tamb_dn2_slot = var_tamb_dn2;
        *var_tamb_dn3_slot = var_tamb_dn3;
        *var_tamb_dn4_slot = var_tamb_dn4;
        *var_tamb_dn5_slot = var_tamb_dn5;
        *var_tamb_dn6_slot = var_tamb_dn6;
        *var_tamb_rdb0_slot = var_tamb_rdb0;
        *var_tamb_rdb1_slot = var_tamb_rdb1;
        *var_tamb_rdb2_slot = var_tamb_rdb2;
        *var_tamb_rdb3_slot = var_tamb_rdb3;
        *var_tamb_rdb4_slot = var_tamb_rdb4;
        *var_tamb_rdb5_slot = var_tamb_rdb5;
        *var_tamb_rdb6_slot = var_tamb_rdb6;
        *var_tamb_rdn0_slot = var_tamb_rdn0;
        *var_tamb_rdn1_slot = var_tamb_rdn1;
        *var_tamb_rdn2_slot = var_tamb_rdn2;
        *var_tamb_rdn3_slot = var_tamb_rdn3;
        *var_tamb_rdn4_slot = var_tamb_rdn4;
        *var_tamb_rdn5_slot = var_tamb_rdn5;
        *var_tamb_rdn6_slot = var_tamb_rdn6;
        *var_tamb_rv_slot = var_tamb_rv;
        *var_tdev_slot = var_tdev;
        *var_tdev_db0_slot = var_tdev_db0;
        *var_tdev_db1_slot = var_tdev_db1;
        *var_tdev_db2_slot = var_tdev_db2;
        *var_tdev_db3_slot = var_tdev_db3;
        *var_tdev_db4_slot = var_tdev_db4;
        *var_tdev_db5_slot = var_tdev_db5;
        *var_tdev_db6_slot = var_tdev_db6;
        *var_tdev_dn0_slot = var_tdev_dn0;
        *var_tdev_dn1_slot = var_tdev_dn1;
        *var_tdev_dn2_slot = var_tdev_dn2;
        *var_tdev_dn3_slot = var_tdev_dn3;
        *var_tdev_dn4_slot = var_tdev_dn4;
        *var_tdev_dn5_slot = var_tdev_dn5;
        *var_tdev_dn6_slot = var_tdev_dn6;
        *var_tdev_rdb0_slot = var_tdev_rdb0;
        *var_tdev_rdb1_slot = var_tdev_rdb1;
        *var_tdev_rdb2_slot = var_tdev_rdb2;
        *var_tdev_rdb3_slot = var_tdev_rdb3;
        *var_tdev_rdb4_slot = var_tdev_rdb4;
        *var_tdev_rdb5_slot = var_tdev_rdb5;
        *var_tdev_rdb6_slot = var_tdev_rdb6;
        *var_tdev_rdn0_slot = var_tdev_rdn0;
        *var_tdev_rdn1_slot = var_tdev_rdn1;
        *var_tdev_rdn2_slot = var_tdev_rdn2;
        *var_tdev_rdn3_slot = var_tdev_rdn3;
        *var_tdev_rdn4_slot = var_tdev_rdn4;
        *var_tdev_rdn5_slot = var_tdev_rdn5;
        *var_tdev_rdn6_slot = var_tdev_rdn6;
        *var_tdev_rv_slot = var_tdev_rv;
        *var_theexp_t_slot = var_theexp_t;
        *var_theexp_t_db0_slot = var_theexp_t_db0;
        *var_theexp_t_db1_slot = var_theexp_t_db1;
        *var_theexp_t_db2_slot = var_theexp_t_db2;
        *var_theexp_t_db3_slot = var_theexp_t_db3;
        *var_theexp_t_db4_slot = var_theexp_t_db4;
        *var_theexp_t_db5_slot = var_theexp_t_db5;
        *var_theexp_t_db6_slot = var_theexp_t_db6;
        *var_theexp_t_dn0_slot = var_theexp_t_dn0;
        *var_theexp_t_dn1_slot = var_theexp_t_dn1;
        *var_theexp_t_dn2_slot = var_theexp_t_dn2;
        *var_theexp_t_dn3_slot = var_theexp_t_dn3;
        *var_theexp_t_dn4_slot = var_theexp_t_dn4;
        *var_theexp_t_dn5_slot = var_theexp_t_dn5;
        *var_theexp_t_dn6_slot = var_theexp_t_dn6;
        *var_theexp_t_rdb0_slot = var_theexp_t_rdb0;
        *var_theexp_t_rdb1_slot = var_theexp_t_rdb1;
        *var_theexp_t_rdb2_slot = var_theexp_t_rdb2;
        *var_theexp_t_rdb3_slot = var_theexp_t_rdb3;
        *var_theexp_t_rdb4_slot = var_theexp_t_rdb4;
        *var_theexp_t_rdb5_slot = var_theexp_t_rdb5;
        *var_theexp_t_rdb6_slot = var_theexp_t_rdb6;
        *var_theexp_t_rdn0_slot = var_theexp_t_rdn0;
        *var_theexp_t_rdn1_slot = var_theexp_t_rdn1;
        *var_theexp_t_rdn2_slot = var_theexp_t_rdn2;
        *var_theexp_t_rdn3_slot = var_theexp_t_rdn3;
        *var_theexp_t_rdn4_slot = var_theexp_t_rdn4;
        *var_theexp_t_rdn5_slot = var_theexp_t_rdn5;
        *var_theexp_t_rdn6_slot = var_theexp_t_rdn6;
        *var_theexp_t_rv_slot = var_theexp_t_rv;
        *var_tnom_slot = var_tnom;
        *var_tnom_db0_slot = var_tnom_db0;
        *var_tnom_db1_slot = var_tnom_db1;
        *var_tnom_db2_slot = var_tnom_db2;
        *var_tnom_db3_slot = var_tnom_db3;
        *var_tnom_db4_slot = var_tnom_db4;
        *var_tnom_db5_slot = var_tnom_db5;
        *var_tnom_db6_slot = var_tnom_db6;
        *var_tnom_dn0_slot = var_tnom_dn0;
        *var_tnom_dn1_slot = var_tnom_dn1;
        *var_tnom_dn2_slot = var_tnom_dn2;
        *var_tnom_dn3_slot = var_tnom_dn3;
        *var_tnom_dn4_slot = var_tnom_dn4;
        *var_tnom_dn5_slot = var_tnom_dn5;
        *var_tnom_dn6_slot = var_tnom_dn6;
        *var_tnom_rdb0_slot = var_tnom_rdb0;
        *var_tnom_rdb1_slot = var_tnom_rdb1;
        *var_tnom_rdb2_slot = var_tnom_rdb2;
        *var_tnom_rdb3_slot = var_tnom_rdb3;
        *var_tnom_rdb4_slot = var_tnom_rdb4;
        *var_tnom_rdb5_slot = var_tnom_rdb5;
        *var_tnom_rdb6_slot = var_tnom_rdb6;
        *var_tnom_rdn0_slot = var_tnom_rdn0;
        *var_tnom_rdn1_slot = var_tnom_rdn1;
        *var_tnom_rdn2_slot = var_tnom_rdn2;
        *var_tnom_rdn3_slot = var_tnom_rdn3;
        *var_tnom_rdn4_slot = var_tnom_rdn4;
        *var_tnom_rdn5_slot = var_tnom_rdn5;
        *var_tnom_rdn6_slot = var_tnom_rdn6;
        *var_tnom_rv_slot = var_tnom_rv;
        *var_vt_slot = var_vt;
        *var_vt_db0_slot = var_vt_db0;
        *var_vt_db1_slot = var_vt_db1;
        *var_vt_db2_slot = var_vt_db2;
        *var_vt_db3_slot = var_vt_db3;
        *var_vt_db4_slot = var_vt_db4;
        *var_vt_db5_slot = var_vt_db5;
        *var_vt_db6_slot = var_vt_db6;
        *var_vt_dn0_slot = var_vt_dn0;
        *var_vt_dn1_slot = var_vt_dn1;
        *var_vt_dn2_slot = var_vt_dn2;
        *var_vt_dn3_slot = var_vt_dn3;
        *var_vt_dn4_slot = var_vt_dn4;
        *var_vt_dn5_slot = var_vt_dn5;
        *var_vt_dn6_slot = var_vt_dn6;
        *var_vt_rdb0_slot = var_vt_rdb0;
        *var_vt_rdb1_slot = var_vt_rdb1;
        *var_vt_rdb2_slot = var_vt_rdb2;
        *var_vt_rdb3_slot = var_vt_rdb3;
        *var_vt_rdb4_slot = var_vt_rdb4;
        *var_vt_rdb5_slot = var_vt_rdb5;
        *var_vt_rdb6_slot = var_vt_rdb6;
        *var_vt_rdn0_slot = var_vt_rdn0;
        *var_vt_rdn1_slot = var_vt_rdn1;
        *var_vt_rdn2_slot = var_vt_rdn2;
        *var_vt_rdn3_slot = var_vt_rdn3;
        *var_vt_rdn4_slot = var_vt_rdn4;
        *var_vt_rdn5_slot = var_vt_rdn5;
        *var_vt_rdn6_slot = var_vt_rdn6;
        *var_vt_rv_slot = var_vt_rv;
        *var_weff_slot = var_weff;
        *var_weff_db0_slot = var_weff_db0;
        *var_weff_db1_slot = var_weff_db1;
        *var_weff_db2_slot = var_weff_db2;
        *var_weff_db3_slot = var_weff_db3;
        *var_weff_db4_slot = var_weff_db4;
        *var_weff_db5_slot = var_weff_db5;
        *var_weff_db6_slot = var_weff_db6;
        *var_weff_dn0_slot = var_weff_dn0;
        *var_weff_dn1_slot = var_weff_dn1;
        *var_weff_dn2_slot = var_weff_dn2;
        *var_weff_dn3_slot = var_weff_dn3;
        *var_weff_dn4_slot = var_weff_dn4;
        *var_weff_dn5_slot = var_weff_dn5;
        *var_weff_dn6_slot = var_weff_dn6;
        *var_weff_rdb0_slot = var_weff_rdb0;
        *var_weff_rdb1_slot = var_weff_rdb1;
        *var_weff_rdb2_slot = var_weff_rdb2;
        *var_weff_rdb3_slot = var_weff_rdb3;
        *var_weff_rdb4_slot = var_weff_rdb4;
        *var_weff_rdb5_slot = var_weff_rdb5;
        *var_weff_rdb6_slot = var_weff_rdb6;
        *var_weff_rdn0_slot = var_weff_rdn0;
        *var_weff_rdn1_slot = var_weff_rdn1;
        *var_weff_rdn2_slot = var_weff_rdn2;
        *var_weff_rdn3_slot = var_weff_rdn3;
        *var_weff_rdn4_slot = var_weff_rdn4;
        *var_weff_rdn5_slot = var_weff_rdn5;
        *var_weff_rdn6_slot = var_weff_rdn6;
        *var_weff_rv_slot = var_weff_rv;
    }

    pub(super) fn stamp_reactive_block_1(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_is_t: f64,
        var_tdev: f64,
        var_tdev_db0: f64,
        var_tdev_db1: f64,
        var_tdev_db2: f64,
        var_tdev_db3: f64,
        var_tdev_db4: f64,
        var_tdev_db5: f64,
        var_tdev_db6: f64,
        var_tdev_dn0: f64,
        var_tdev_dn1: f64,
        var_tdev_dn2: f64,
        var_tdev_dn3: f64,
        var_tdev_dn4: f64,
        var_tdev_dn5: f64,
        var_tdev_dn6: f64,
        var_tnom: f64,
        var_tnom_db0: f64,
        var_tnom_db1: f64,
        var_tnom_db2: f64,
        var_tnom_db3: f64,
        var_tnom_db4: f64,
        var_tnom_db5: f64,
        var_tnom_db6: f64,
        var_tnom_dn0: f64,
        var_tnom_dn1: f64,
        var_tnom_dn2: f64,
        var_tnom_dn3: f64,
        var_tnom_dn4: f64,
        var_tnom_dn5: f64,
        var_tnom_dn6: f64,
        var_vt: f64,
        var_vt_db0: f64,
        var_vt_db1: f64,
        var_vt_db2: f64,
        var_vt_db3: f64,
        var_vt_db4: f64,
        var_vt_db5: f64,
        var_vt_db6: f64,
        var_vt_dn0: f64,
        var_vt_dn1: f64,
        var_vt_dn2: f64,
        var_vt_dn3: f64,
        var_vt_dn4: f64,
        var_vt_dn5: f64,
        var_vt_dn6: f64,
        var_arg0_slot: &mut f64,
        var_arg0_db0_slot: &mut f64,
        var_arg0_db1_slot: &mut f64,
        var_arg0_db2_slot: &mut f64,
        var_arg0_db3_slot: &mut f64,
        var_arg0_db4_slot: &mut f64,
        var_arg0_db5_slot: &mut f64,
        var_arg0_db6_slot: &mut f64,
        var_arg0_dn0_slot: &mut f64,
        var_arg0_dn1_slot: &mut f64,
        var_arg0_dn2_slot: &mut f64,
        var_arg0_dn3_slot: &mut f64,
        var_arg0_dn4_slot: &mut f64,
        var_arg0_dn5_slot: &mut f64,
        var_arg0_dn6_slot: &mut f64,
        var_arg0_rdb0_slot: &mut f64,
        var_arg0_rdb1_slot: &mut f64,
        var_arg0_rdb2_slot: &mut f64,
        var_arg0_rdb3_slot: &mut f64,
        var_arg0_rdb4_slot: &mut f64,
        var_arg0_rdb5_slot: &mut f64,
        var_arg0_rdb6_slot: &mut f64,
        var_arg0_rdn0_slot: &mut f64,
        var_arg0_rdn1_slot: &mut f64,
        var_arg0_rdn2_slot: &mut f64,
        var_arg0_rdn3_slot: &mut f64,
        var_arg0_rdn4_slot: &mut f64,
        var_arg0_rdn5_slot: &mut f64,
        var_arg0_rdn6_slot: &mut f64,
        var_arg0_rv_slot: &mut f64,
        var_cje_i_slot: &mut f64,
        var_cje_i_db0_slot: &mut f64,
        var_cje_i_db1_slot: &mut f64,
        var_cje_i_db2_slot: &mut f64,
        var_cje_i_db3_slot: &mut f64,
        var_cje_i_db4_slot: &mut f64,
        var_cje_i_db5_slot: &mut f64,
        var_cje_i_db6_slot: &mut f64,
        var_cje_i_dn0_slot: &mut f64,
        var_cje_i_dn1_slot: &mut f64,
        var_cje_i_dn2_slot: &mut f64,
        var_cje_i_dn3_slot: &mut f64,
        var_cje_i_dn4_slot: &mut f64,
        var_cje_i_dn5_slot: &mut f64,
        var_cje_i_dn6_slot: &mut f64,
        var_cje_i_rdb0_slot: &mut f64,
        var_cje_i_rdb1_slot: &mut f64,
        var_cje_i_rdb2_slot: &mut f64,
        var_cje_i_rdb3_slot: &mut f64,
        var_cje_i_rdb4_slot: &mut f64,
        var_cje_i_rdb5_slot: &mut f64,
        var_cje_i_rdb6_slot: &mut f64,
        var_cje_i_rdn0_slot: &mut f64,
        var_cje_i_rdn1_slot: &mut f64,
        var_cje_i_rdn2_slot: &mut f64,
        var_cje_i_rdn3_slot: &mut f64,
        var_cje_i_rdn4_slot: &mut f64,
        var_cje_i_rdn5_slot: &mut f64,
        var_cje_i_rdn6_slot: &mut f64,
        var_cje_i_rv_slot: &mut f64,
        var_cje_t_slot: &mut f64,
        var_cje_t_db0_slot: &mut f64,
        var_cje_t_db1_slot: &mut f64,
        var_cje_t_db2_slot: &mut f64,
        var_cje_t_db3_slot: &mut f64,
        var_cje_t_db4_slot: &mut f64,
        var_cje_t_db5_slot: &mut f64,
        var_cje_t_db6_slot: &mut f64,
        var_cje_t_dn0_slot: &mut f64,
        var_cje_t_dn1_slot: &mut f64,
        var_cje_t_dn2_slot: &mut f64,
        var_cje_t_dn3_slot: &mut f64,
        var_cje_t_dn4_slot: &mut f64,
        var_cje_t_dn5_slot: &mut f64,
        var_cje_t_dn6_slot: &mut f64,
        var_cje_t_rdb0_slot: &mut f64,
        var_cje_t_rdb1_slot: &mut f64,
        var_cje_t_rdb2_slot: &mut f64,
        var_cje_t_rdb3_slot: &mut f64,
        var_cje_t_rdb4_slot: &mut f64,
        var_cje_t_rdb5_slot: &mut f64,
        var_cje_t_rdb6_slot: &mut f64,
        var_cje_t_rdn0_slot: &mut f64,
        var_cje_t_rdn1_slot: &mut f64,
        var_cje_t_rdn2_slot: &mut f64,
        var_cje_t_rdn3_slot: &mut f64,
        var_cje_t_rdn4_slot: &mut f64,
        var_cje_t_rdn5_slot: &mut f64,
        var_cje_t_rdn6_slot: &mut f64,
        var_cje_t_rv_slot: &mut f64,
        var_cjt_slot: &mut f64,
        var_cjt_db0_slot: &mut f64,
        var_cjt_db1_slot: &mut f64,
        var_cjt_db2_slot: &mut f64,
        var_cjt_db3_slot: &mut f64,
        var_cjt_db4_slot: &mut f64,
        var_cjt_db5_slot: &mut f64,
        var_cjt_db6_slot: &mut f64,
        var_cjt_dn0_slot: &mut f64,
        var_cjt_dn1_slot: &mut f64,
        var_cjt_dn2_slot: &mut f64,
        var_cjt_dn3_slot: &mut f64,
        var_cjt_dn4_slot: &mut f64,
        var_cjt_dn5_slot: &mut f64,
        var_cjt_dn6_slot: &mut f64,
        var_cjt_rdb0_slot: &mut f64,
        var_cjt_rdb1_slot: &mut f64,
        var_cjt_rdb2_slot: &mut f64,
        var_cjt_rdb3_slot: &mut f64,
        var_cjt_rdb4_slot: &mut f64,
        var_cjt_rdb5_slot: &mut f64,
        var_cjt_rdb6_slot: &mut f64,
        var_cjt_rdn0_slot: &mut f64,
        var_cjt_rdn1_slot: &mut f64,
        var_cjt_rdn2_slot: &mut f64,
        var_cjt_rdn3_slot: &mut f64,
        var_cjt_rdn4_slot: &mut f64,
        var_cjt_rdn5_slot: &mut f64,
        var_cjt_rdn6_slot: &mut f64,
        var_cjt_rv_slot: &mut f64,
        var_egfet_slot: &mut f64,
        var_egfet_db0_slot: &mut f64,
        var_egfet_db1_slot: &mut f64,
        var_egfet_db2_slot: &mut f64,
        var_egfet_db3_slot: &mut f64,
        var_egfet_db4_slot: &mut f64,
        var_egfet_db5_slot: &mut f64,
        var_egfet_db6_slot: &mut f64,
        var_egfet_dn0_slot: &mut f64,
        var_egfet_dn1_slot: &mut f64,
        var_egfet_dn2_slot: &mut f64,
        var_egfet_dn3_slot: &mut f64,
        var_egfet_dn4_slot: &mut f64,
        var_egfet_dn5_slot: &mut f64,
        var_egfet_dn6_slot: &mut f64,
        var_egfet_rdb0_slot: &mut f64,
        var_egfet_rdb1_slot: &mut f64,
        var_egfet_rdb2_slot: &mut f64,
        var_egfet_rdb3_slot: &mut f64,
        var_egfet_rdb4_slot: &mut f64,
        var_egfet_rdb5_slot: &mut f64,
        var_egfet_rdb6_slot: &mut f64,
        var_egfet_rdn0_slot: &mut f64,
        var_egfet_rdn1_slot: &mut f64,
        var_egfet_rdn2_slot: &mut f64,
        var_egfet_rdn3_slot: &mut f64,
        var_egfet_rdn4_slot: &mut f64,
        var_egfet_rdn5_slot: &mut f64,
        var_egfet_rdn6_slot: &mut f64,
        var_egfet_rv_slot: &mut f64,
        var_fact1_slot: &mut f64,
        var_fact1_db0_slot: &mut f64,
        var_fact1_db1_slot: &mut f64,
        var_fact1_db2_slot: &mut f64,
        var_fact1_db3_slot: &mut f64,
        var_fact1_db4_slot: &mut f64,
        var_fact1_db5_slot: &mut f64,
        var_fact1_db6_slot: &mut f64,
        var_fact1_dn0_slot: &mut f64,
        var_fact1_dn1_slot: &mut f64,
        var_fact1_dn2_slot: &mut f64,
        var_fact1_dn3_slot: &mut f64,
        var_fact1_dn4_slot: &mut f64,
        var_fact1_dn5_slot: &mut f64,
        var_fact1_dn6_slot: &mut f64,
        var_fact1_rdb0_slot: &mut f64,
        var_fact1_rdb1_slot: &mut f64,
        var_fact1_rdb2_slot: &mut f64,
        var_fact1_rdb3_slot: &mut f64,
        var_fact1_rdb4_slot: &mut f64,
        var_fact1_rdb5_slot: &mut f64,
        var_fact1_rdb6_slot: &mut f64,
        var_fact1_rdn0_slot: &mut f64,
        var_fact1_rdn1_slot: &mut f64,
        var_fact1_rdn2_slot: &mut f64,
        var_fact1_rdn3_slot: &mut f64,
        var_fact1_rdn4_slot: &mut f64,
        var_fact1_rdn5_slot: &mut f64,
        var_fact1_rdn6_slot: &mut f64,
        var_fact1_rv_slot: &mut f64,
        var_fact2_slot: &mut f64,
        var_fact2_db0_slot: &mut f64,
        var_fact2_db1_slot: &mut f64,
        var_fact2_db2_slot: &mut f64,
        var_fact2_db3_slot: &mut f64,
        var_fact2_db4_slot: &mut f64,
        var_fact2_db5_slot: &mut f64,
        var_fact2_db6_slot: &mut f64,
        var_fact2_dn0_slot: &mut f64,
        var_fact2_dn1_slot: &mut f64,
        var_fact2_dn2_slot: &mut f64,
        var_fact2_dn3_slot: &mut f64,
        var_fact2_dn4_slot: &mut f64,
        var_fact2_dn5_slot: &mut f64,
        var_fact2_dn6_slot: &mut f64,
        var_fact2_rdb0_slot: &mut f64,
        var_fact2_rdb1_slot: &mut f64,
        var_fact2_rdb2_slot: &mut f64,
        var_fact2_rdb3_slot: &mut f64,
        var_fact2_rdb4_slot: &mut f64,
        var_fact2_rdb5_slot: &mut f64,
        var_fact2_rdb6_slot: &mut f64,
        var_fact2_rdn0_slot: &mut f64,
        var_fact2_rdn1_slot: &mut f64,
        var_fact2_rdn2_slot: &mut f64,
        var_fact2_rdn3_slot: &mut f64,
        var_fact2_rdn4_slot: &mut f64,
        var_fact2_rdn5_slot: &mut f64,
        var_fact2_rdn6_slot: &mut f64,
        var_fact2_rv_slot: &mut f64,
        var_gmanew_slot: &mut f64,
        var_gmanew_db0_slot: &mut f64,
        var_gmanew_db1_slot: &mut f64,
        var_gmanew_db2_slot: &mut f64,
        var_gmanew_db3_slot: &mut f64,
        var_gmanew_db4_slot: &mut f64,
        var_gmanew_db5_slot: &mut f64,
        var_gmanew_db6_slot: &mut f64,
        var_gmanew_dn0_slot: &mut f64,
        var_gmanew_dn1_slot: &mut f64,
        var_gmanew_dn2_slot: &mut f64,
        var_gmanew_dn3_slot: &mut f64,
        var_gmanew_dn4_slot: &mut f64,
        var_gmanew_dn5_slot: &mut f64,
        var_gmanew_dn6_slot: &mut f64,
        var_gmanew_rdb0_slot: &mut f64,
        var_gmanew_rdb1_slot: &mut f64,
        var_gmanew_rdb2_slot: &mut f64,
        var_gmanew_rdb3_slot: &mut f64,
        var_gmanew_rdb4_slot: &mut f64,
        var_gmanew_rdb5_slot: &mut f64,
        var_gmanew_rdb6_slot: &mut f64,
        var_gmanew_rdn0_slot: &mut f64,
        var_gmanew_rdn1_slot: &mut f64,
        var_gmanew_rdn2_slot: &mut f64,
        var_gmanew_rdn3_slot: &mut f64,
        var_gmanew_rdn4_slot: &mut f64,
        var_gmanew_rdn5_slot: &mut f64,
        var_gmanew_rdn6_slot: &mut f64,
        var_gmanew_rv_slot: &mut f64,
        var_gmaold_slot: &mut f64,
        var_gmaold_db0_slot: &mut f64,
        var_gmaold_db1_slot: &mut f64,
        var_gmaold_db2_slot: &mut f64,
        var_gmaold_db3_slot: &mut f64,
        var_gmaold_db4_slot: &mut f64,
        var_gmaold_db5_slot: &mut f64,
        var_gmaold_db6_slot: &mut f64,
        var_gmaold_dn0_slot: &mut f64,
        var_gmaold_dn1_slot: &mut f64,
        var_gmaold_dn2_slot: &mut f64,
        var_gmaold_dn3_slot: &mut f64,
        var_gmaold_dn4_slot: &mut f64,
        var_gmaold_dn5_slot: &mut f64,
        var_gmaold_dn6_slot: &mut f64,
        var_gmaold_rdb0_slot: &mut f64,
        var_gmaold_rdb1_slot: &mut f64,
        var_gmaold_rdb2_slot: &mut f64,
        var_gmaold_rdb3_slot: &mut f64,
        var_gmaold_rdb4_slot: &mut f64,
        var_gmaold_rdb5_slot: &mut f64,
        var_gmaold_rdb6_slot: &mut f64,
        var_gmaold_rdn0_slot: &mut f64,
        var_gmaold_rdn1_slot: &mut f64,
        var_gmaold_rdn2_slot: &mut f64,
        var_gmaold_rdn3_slot: &mut f64,
        var_gmaold_rdn4_slot: &mut f64,
        var_gmaold_rdn5_slot: &mut f64,
        var_gmaold_rdn6_slot: &mut f64,
        var_gmaold_rv_slot: &mut f64,
        var_guard3_slot: &mut f64,
        var_guard3_db0_slot: &mut f64,
        var_guard3_db1_slot: &mut f64,
        var_guard3_db2_slot: &mut f64,
        var_guard3_db3_slot: &mut f64,
        var_guard3_db4_slot: &mut f64,
        var_guard3_db5_slot: &mut f64,
        var_guard3_db6_slot: &mut f64,
        var_guard3_dn0_slot: &mut f64,
        var_guard3_dn1_slot: &mut f64,
        var_guard3_dn2_slot: &mut f64,
        var_guard3_dn3_slot: &mut f64,
        var_guard3_dn4_slot: &mut f64,
        var_guard3_dn5_slot: &mut f64,
        var_guard3_dn6_slot: &mut f64,
        var_guard3_rdb0_slot: &mut f64,
        var_guard3_rdb1_slot: &mut f64,
        var_guard3_rdb2_slot: &mut f64,
        var_guard3_rdb3_slot: &mut f64,
        var_guard3_rdb4_slot: &mut f64,
        var_guard3_rdb5_slot: &mut f64,
        var_guard3_rdb6_slot: &mut f64,
        var_guard3_rdn0_slot: &mut f64,
        var_guard3_rdn1_slot: &mut f64,
        var_guard3_rdn2_slot: &mut f64,
        var_guard3_rdn3_slot: &mut f64,
        var_guard3_rdn4_slot: &mut f64,
        var_guard3_rdn5_slot: &mut f64,
        var_guard3_rdn6_slot: &mut f64,
        var_guard3_rv_slot: &mut f64,
        var_pbfact_slot: &mut f64,
        var_pbfact_db0_slot: &mut f64,
        var_pbfact_db1_slot: &mut f64,
        var_pbfact_db2_slot: &mut f64,
        var_pbfact_db3_slot: &mut f64,
        var_pbfact_db4_slot: &mut f64,
        var_pbfact_db5_slot: &mut f64,
        var_pbfact_db6_slot: &mut f64,
        var_pbfact_dn0_slot: &mut f64,
        var_pbfact_dn1_slot: &mut f64,
        var_pbfact_dn2_slot: &mut f64,
        var_pbfact_dn3_slot: &mut f64,
        var_pbfact_dn4_slot: &mut f64,
        var_pbfact_dn5_slot: &mut f64,
        var_pbfact_dn6_slot: &mut f64,
        var_pbfact_rdb0_slot: &mut f64,
        var_pbfact_rdb1_slot: &mut f64,
        var_pbfact_rdb2_slot: &mut f64,
        var_pbfact_rdb3_slot: &mut f64,
        var_pbfact_rdb4_slot: &mut f64,
        var_pbfact_rdb5_slot: &mut f64,
        var_pbfact_rdb6_slot: &mut f64,
        var_pbfact_rdn0_slot: &mut f64,
        var_pbfact_rdn1_slot: &mut f64,
        var_pbfact_rdn2_slot: &mut f64,
        var_pbfact_rdn3_slot: &mut f64,
        var_pbfact_rdn4_slot: &mut f64,
        var_pbfact_rdn5_slot: &mut f64,
        var_pbfact_rdn6_slot: &mut f64,
        var_pbfact_rv_slot: &mut f64,
        var_pbo_slot: &mut f64,
        var_pbo_db0_slot: &mut f64,
        var_pbo_db1_slot: &mut f64,
        var_pbo_db2_slot: &mut f64,
        var_pbo_db3_slot: &mut f64,
        var_pbo_db4_slot: &mut f64,
        var_pbo_db5_slot: &mut f64,
        var_pbo_db6_slot: &mut f64,
        var_pbo_dn0_slot: &mut f64,
        var_pbo_dn1_slot: &mut f64,
        var_pbo_dn2_slot: &mut f64,
        var_pbo_dn3_slot: &mut f64,
        var_pbo_dn4_slot: &mut f64,
        var_pbo_dn5_slot: &mut f64,
        var_pbo_dn6_slot: &mut f64,
        var_pbo_rdb0_slot: &mut f64,
        var_pbo_rdb1_slot: &mut f64,
        var_pbo_rdb2_slot: &mut f64,
        var_pbo_rdb3_slot: &mut f64,
        var_pbo_rdb4_slot: &mut f64,
        var_pbo_rdb5_slot: &mut f64,
        var_pbo_rdb6_slot: &mut f64,
        var_pbo_rdn0_slot: &mut f64,
        var_pbo_rdn1_slot: &mut f64,
        var_pbo_rdn2_slot: &mut f64,
        var_pbo_rdn3_slot: &mut f64,
        var_pbo_rdn4_slot: &mut f64,
        var_pbo_rdn5_slot: &mut f64,
        var_pbo_rdn6_slot: &mut f64,
        var_pbo_rv_slot: &mut f64,
        var_ttype_slot: &mut f64,
        var_ttype_db0_slot: &mut f64,
        var_ttype_db1_slot: &mut f64,
        var_ttype_db2_slot: &mut f64,
        var_ttype_db3_slot: &mut f64,
        var_ttype_db4_slot: &mut f64,
        var_ttype_db5_slot: &mut f64,
        var_ttype_db6_slot: &mut f64,
        var_ttype_dn0_slot: &mut f64,
        var_ttype_dn1_slot: &mut f64,
        var_ttype_dn2_slot: &mut f64,
        var_ttype_dn3_slot: &mut f64,
        var_ttype_dn4_slot: &mut f64,
        var_ttype_dn5_slot: &mut f64,
        var_ttype_dn6_slot: &mut f64,
        var_ttype_rdb0_slot: &mut f64,
        var_ttype_rdb1_slot: &mut f64,
        var_ttype_rdb2_slot: &mut f64,
        var_ttype_rdb3_slot: &mut f64,
        var_ttype_rdb4_slot: &mut f64,
        var_ttype_rdb5_slot: &mut f64,
        var_ttype_rdb6_slot: &mut f64,
        var_ttype_rdn0_slot: &mut f64,
        var_ttype_rdn1_slot: &mut f64,
        var_ttype_rdn2_slot: &mut f64,
        var_ttype_rdn3_slot: &mut f64,
        var_ttype_rdn4_slot: &mut f64,
        var_ttype_rdn5_slot: &mut f64,
        var_ttype_rdn6_slot: &mut f64,
        var_ttype_rv_slot: &mut f64,
        var_vbiei_slot: &mut f64,
        var_vbiei_db0_slot: &mut f64,
        var_vbiei_db1_slot: &mut f64,
        var_vbiei_db2_slot: &mut f64,
        var_vbiei_db3_slot: &mut f64,
        var_vbiei_db4_slot: &mut f64,
        var_vbiei_db5_slot: &mut f64,
        var_vbiei_db6_slot: &mut f64,
        var_vbiei_dn0_slot: &mut f64,
        var_vbiei_dn1_slot: &mut f64,
        var_vbiei_dn2_slot: &mut f64,
        var_vbiei_dn3_slot: &mut f64,
        var_vbiei_dn4_slot: &mut f64,
        var_vbiei_dn5_slot: &mut f64,
        var_vbiei_dn6_slot: &mut f64,
        var_vbiei_rdb0_slot: &mut f64,
        var_vbiei_rdb1_slot: &mut f64,
        var_vbiei_rdb2_slot: &mut f64,
        var_vbiei_rdb3_slot: &mut f64,
        var_vbiei_rdb4_slot: &mut f64,
        var_vbiei_rdb5_slot: &mut f64,
        var_vbiei_rdb6_slot: &mut f64,
        var_vbiei_rdn0_slot: &mut f64,
        var_vbiei_rdn1_slot: &mut f64,
        var_vbiei_rdn2_slot: &mut f64,
        var_vbiei_rdn3_slot: &mut f64,
        var_vbiei_rdn4_slot: &mut f64,
        var_vbiei_rdn5_slot: &mut f64,
        var_vbiei_rdn6_slot: &mut f64,
        var_vbiei_rv_slot: &mut f64,
        var_vje_t_slot: &mut f64,
        var_vje_t_db0_slot: &mut f64,
        var_vje_t_db1_slot: &mut f64,
        var_vje_t_db2_slot: &mut f64,
        var_vje_t_db3_slot: &mut f64,
        var_vje_t_db4_slot: &mut f64,
        var_vje_t_db5_slot: &mut f64,
        var_vje_t_db6_slot: &mut f64,
        var_vje_t_dn0_slot: &mut f64,
        var_vje_t_dn1_slot: &mut f64,
        var_vje_t_dn2_slot: &mut f64,
        var_vje_t_dn3_slot: &mut f64,
        var_vje_t_dn4_slot: &mut f64,
        var_vje_t_dn5_slot: &mut f64,
        var_vje_t_dn6_slot: &mut f64,
        var_vje_t_rdb0_slot: &mut f64,
        var_vje_t_rdb1_slot: &mut f64,
        var_vje_t_rdb2_slot: &mut f64,
        var_vje_t_rdb3_slot: &mut f64,
        var_vje_t_rdb4_slot: &mut f64,
        var_vje_t_rdb5_slot: &mut f64,
        var_vje_t_rdb6_slot: &mut f64,
        var_vje_t_rdn0_slot: &mut f64,
        var_vje_t_rdn1_slot: &mut f64,
        var_vje_t_rdn2_slot: &mut f64,
        var_vje_t_rdn3_slot: &mut f64,
        var_vje_t_rdn4_slot: &mut f64,
        var_vje_t_rdn5_slot: &mut f64,
        var_vje_t_rdn6_slot: &mut f64,
        var_vje_t_rv_slot: &mut f64,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let mut var_arg0: f64 = *var_arg0_slot;
        let mut var_arg0_db0: f64 = *var_arg0_db0_slot;
        let mut var_arg0_db1: f64 = *var_arg0_db1_slot;
        let mut var_arg0_db2: f64 = *var_arg0_db2_slot;
        let mut var_arg0_db3: f64 = *var_arg0_db3_slot;
        let mut var_arg0_db4: f64 = *var_arg0_db4_slot;
        let mut var_arg0_db5: f64 = *var_arg0_db5_slot;
        let mut var_arg0_db6: f64 = *var_arg0_db6_slot;
        let mut var_arg0_dn0: f64 = *var_arg0_dn0_slot;
        let mut var_arg0_dn1: f64 = *var_arg0_dn1_slot;
        let mut var_arg0_dn2: f64 = *var_arg0_dn2_slot;
        let mut var_arg0_dn3: f64 = *var_arg0_dn3_slot;
        let mut var_arg0_dn4: f64 = *var_arg0_dn4_slot;
        let mut var_arg0_dn5: f64 = *var_arg0_dn5_slot;
        let mut var_arg0_dn6: f64 = *var_arg0_dn6_slot;
        let mut var_arg0_rdb0: f64 = *var_arg0_rdb0_slot;
        let mut var_arg0_rdb1: f64 = *var_arg0_rdb1_slot;
        let mut var_arg0_rdb2: f64 = *var_arg0_rdb2_slot;
        let mut var_arg0_rdb3: f64 = *var_arg0_rdb3_slot;
        let mut var_arg0_rdb4: f64 = *var_arg0_rdb4_slot;
        let mut var_arg0_rdb5: f64 = *var_arg0_rdb5_slot;
        let mut var_arg0_rdb6: f64 = *var_arg0_rdb6_slot;
        let mut var_arg0_rdn0: f64 = *var_arg0_rdn0_slot;
        let mut var_arg0_rdn1: f64 = *var_arg0_rdn1_slot;
        let mut var_arg0_rdn2: f64 = *var_arg0_rdn2_slot;
        let mut var_arg0_rdn3: f64 = *var_arg0_rdn3_slot;
        let mut var_arg0_rdn4: f64 = *var_arg0_rdn4_slot;
        let mut var_arg0_rdn5: f64 = *var_arg0_rdn5_slot;
        let mut var_arg0_rdn6: f64 = *var_arg0_rdn6_slot;
        let mut var_arg0_rv: f64 = *var_arg0_rv_slot;
        let mut var_cje_i: f64 = *var_cje_i_slot;
        let mut var_cje_i_db0: f64 = *var_cje_i_db0_slot;
        let mut var_cje_i_db1: f64 = *var_cje_i_db1_slot;
        let mut var_cje_i_db2: f64 = *var_cje_i_db2_slot;
        let mut var_cje_i_db3: f64 = *var_cje_i_db3_slot;
        let mut var_cje_i_db4: f64 = *var_cje_i_db4_slot;
        let mut var_cje_i_db5: f64 = *var_cje_i_db5_slot;
        let mut var_cje_i_db6: f64 = *var_cje_i_db6_slot;
        let mut var_cje_i_dn0: f64 = *var_cje_i_dn0_slot;
        let mut var_cje_i_dn1: f64 = *var_cje_i_dn1_slot;
        let mut var_cje_i_dn2: f64 = *var_cje_i_dn2_slot;
        let mut var_cje_i_dn3: f64 = *var_cje_i_dn3_slot;
        let mut var_cje_i_dn4: f64 = *var_cje_i_dn4_slot;
        let mut var_cje_i_dn5: f64 = *var_cje_i_dn5_slot;
        let mut var_cje_i_dn6: f64 = *var_cje_i_dn6_slot;
        let mut var_cje_i_rdb0: f64 = *var_cje_i_rdb0_slot;
        let mut var_cje_i_rdb1: f64 = *var_cje_i_rdb1_slot;
        let mut var_cje_i_rdb2: f64 = *var_cje_i_rdb2_slot;
        let mut var_cje_i_rdb3: f64 = *var_cje_i_rdb3_slot;
        let mut var_cje_i_rdb4: f64 = *var_cje_i_rdb4_slot;
        let mut var_cje_i_rdb5: f64 = *var_cje_i_rdb5_slot;
        let mut var_cje_i_rdb6: f64 = *var_cje_i_rdb6_slot;
        let mut var_cje_i_rdn0: f64 = *var_cje_i_rdn0_slot;
        let mut var_cje_i_rdn1: f64 = *var_cje_i_rdn1_slot;
        let mut var_cje_i_rdn2: f64 = *var_cje_i_rdn2_slot;
        let mut var_cje_i_rdn3: f64 = *var_cje_i_rdn3_slot;
        let mut var_cje_i_rdn4: f64 = *var_cje_i_rdn4_slot;
        let mut var_cje_i_rdn5: f64 = *var_cje_i_rdn5_slot;
        let mut var_cje_i_rdn6: f64 = *var_cje_i_rdn6_slot;
        let mut var_cje_i_rv: f64 = *var_cje_i_rv_slot;
        let mut var_cje_t: f64 = *var_cje_t_slot;
        let mut var_cje_t_db0: f64 = *var_cje_t_db0_slot;
        let mut var_cje_t_db1: f64 = *var_cje_t_db1_slot;
        let mut var_cje_t_db2: f64 = *var_cje_t_db2_slot;
        let mut var_cje_t_db3: f64 = *var_cje_t_db3_slot;
        let mut var_cje_t_db4: f64 = *var_cje_t_db4_slot;
        let mut var_cje_t_db5: f64 = *var_cje_t_db5_slot;
        let mut var_cje_t_db6: f64 = *var_cje_t_db6_slot;
        let mut var_cje_t_dn0: f64 = *var_cje_t_dn0_slot;
        let mut var_cje_t_dn1: f64 = *var_cje_t_dn1_slot;
        let mut var_cje_t_dn2: f64 = *var_cje_t_dn2_slot;
        let mut var_cje_t_dn3: f64 = *var_cje_t_dn3_slot;
        let mut var_cje_t_dn4: f64 = *var_cje_t_dn4_slot;
        let mut var_cje_t_dn5: f64 = *var_cje_t_dn5_slot;
        let mut var_cje_t_dn6: f64 = *var_cje_t_dn6_slot;
        let mut var_cje_t_rdb0: f64 = *var_cje_t_rdb0_slot;
        let mut var_cje_t_rdb1: f64 = *var_cje_t_rdb1_slot;
        let mut var_cje_t_rdb2: f64 = *var_cje_t_rdb2_slot;
        let mut var_cje_t_rdb3: f64 = *var_cje_t_rdb3_slot;
        let mut var_cje_t_rdb4: f64 = *var_cje_t_rdb4_slot;
        let mut var_cje_t_rdb5: f64 = *var_cje_t_rdb5_slot;
        let mut var_cje_t_rdb6: f64 = *var_cje_t_rdb6_slot;
        let mut var_cje_t_rdn0: f64 = *var_cje_t_rdn0_slot;
        let mut var_cje_t_rdn1: f64 = *var_cje_t_rdn1_slot;
        let mut var_cje_t_rdn2: f64 = *var_cje_t_rdn2_slot;
        let mut var_cje_t_rdn3: f64 = *var_cje_t_rdn3_slot;
        let mut var_cje_t_rdn4: f64 = *var_cje_t_rdn4_slot;
        let mut var_cje_t_rdn5: f64 = *var_cje_t_rdn5_slot;
        let mut var_cje_t_rdn6: f64 = *var_cje_t_rdn6_slot;
        let mut var_cje_t_rv: f64 = *var_cje_t_rv_slot;
        let mut var_cjt: f64 = *var_cjt_slot;
        let mut var_cjt_db0: f64 = *var_cjt_db0_slot;
        let mut var_cjt_db1: f64 = *var_cjt_db1_slot;
        let mut var_cjt_db2: f64 = *var_cjt_db2_slot;
        let mut var_cjt_db3: f64 = *var_cjt_db3_slot;
        let mut var_cjt_db4: f64 = *var_cjt_db4_slot;
        let mut var_cjt_db5: f64 = *var_cjt_db5_slot;
        let mut var_cjt_db6: f64 = *var_cjt_db6_slot;
        let mut var_cjt_dn0: f64 = *var_cjt_dn0_slot;
        let mut var_cjt_dn1: f64 = *var_cjt_dn1_slot;
        let mut var_cjt_dn2: f64 = *var_cjt_dn2_slot;
        let mut var_cjt_dn3: f64 = *var_cjt_dn3_slot;
        let mut var_cjt_dn4: f64 = *var_cjt_dn4_slot;
        let mut var_cjt_dn5: f64 = *var_cjt_dn5_slot;
        let mut var_cjt_dn6: f64 = *var_cjt_dn6_slot;
        let mut var_cjt_rdb0: f64 = *var_cjt_rdb0_slot;
        let mut var_cjt_rdb1: f64 = *var_cjt_rdb1_slot;
        let mut var_cjt_rdb2: f64 = *var_cjt_rdb2_slot;
        let mut var_cjt_rdb3: f64 = *var_cjt_rdb3_slot;
        let mut var_cjt_rdb4: f64 = *var_cjt_rdb4_slot;
        let mut var_cjt_rdb5: f64 = *var_cjt_rdb5_slot;
        let mut var_cjt_rdb6: f64 = *var_cjt_rdb6_slot;
        let mut var_cjt_rdn0: f64 = *var_cjt_rdn0_slot;
        let mut var_cjt_rdn1: f64 = *var_cjt_rdn1_slot;
        let mut var_cjt_rdn2: f64 = *var_cjt_rdn2_slot;
        let mut var_cjt_rdn3: f64 = *var_cjt_rdn3_slot;
        let mut var_cjt_rdn4: f64 = *var_cjt_rdn4_slot;
        let mut var_cjt_rdn5: f64 = *var_cjt_rdn5_slot;
        let mut var_cjt_rdn6: f64 = *var_cjt_rdn6_slot;
        let mut var_cjt_rv: f64 = *var_cjt_rv_slot;
        let mut var_egfet: f64 = *var_egfet_slot;
        let mut var_egfet_db0: f64 = *var_egfet_db0_slot;
        let mut var_egfet_db1: f64 = *var_egfet_db1_slot;
        let mut var_egfet_db2: f64 = *var_egfet_db2_slot;
        let mut var_egfet_db3: f64 = *var_egfet_db3_slot;
        let mut var_egfet_db4: f64 = *var_egfet_db4_slot;
        let mut var_egfet_db5: f64 = *var_egfet_db5_slot;
        let mut var_egfet_db6: f64 = *var_egfet_db6_slot;
        let mut var_egfet_dn0: f64 = *var_egfet_dn0_slot;
        let mut var_egfet_dn1: f64 = *var_egfet_dn1_slot;
        let mut var_egfet_dn2: f64 = *var_egfet_dn2_slot;
        let mut var_egfet_dn3: f64 = *var_egfet_dn3_slot;
        let mut var_egfet_dn4: f64 = *var_egfet_dn4_slot;
        let mut var_egfet_dn5: f64 = *var_egfet_dn5_slot;
        let mut var_egfet_dn6: f64 = *var_egfet_dn6_slot;
        let mut var_egfet_rdb0: f64 = *var_egfet_rdb0_slot;
        let mut var_egfet_rdb1: f64 = *var_egfet_rdb1_slot;
        let mut var_egfet_rdb2: f64 = *var_egfet_rdb2_slot;
        let mut var_egfet_rdb3: f64 = *var_egfet_rdb3_slot;
        let mut var_egfet_rdb4: f64 = *var_egfet_rdb4_slot;
        let mut var_egfet_rdb5: f64 = *var_egfet_rdb5_slot;
        let mut var_egfet_rdb6: f64 = *var_egfet_rdb6_slot;
        let mut var_egfet_rdn0: f64 = *var_egfet_rdn0_slot;
        let mut var_egfet_rdn1: f64 = *var_egfet_rdn1_slot;
        let mut var_egfet_rdn2: f64 = *var_egfet_rdn2_slot;
        let mut var_egfet_rdn3: f64 = *var_egfet_rdn3_slot;
        let mut var_egfet_rdn4: f64 = *var_egfet_rdn4_slot;
        let mut var_egfet_rdn5: f64 = *var_egfet_rdn5_slot;
        let mut var_egfet_rdn6: f64 = *var_egfet_rdn6_slot;
        let mut var_egfet_rv: f64 = *var_egfet_rv_slot;
        let mut var_fact1: f64 = *var_fact1_slot;
        let mut var_fact1_db0: f64 = *var_fact1_db0_slot;
        let mut var_fact1_db1: f64 = *var_fact1_db1_slot;
        let mut var_fact1_db2: f64 = *var_fact1_db2_slot;
        let mut var_fact1_db3: f64 = *var_fact1_db3_slot;
        let mut var_fact1_db4: f64 = *var_fact1_db4_slot;
        let mut var_fact1_db5: f64 = *var_fact1_db5_slot;
        let mut var_fact1_db6: f64 = *var_fact1_db6_slot;
        let mut var_fact1_dn0: f64 = *var_fact1_dn0_slot;
        let mut var_fact1_dn1: f64 = *var_fact1_dn1_slot;
        let mut var_fact1_dn2: f64 = *var_fact1_dn2_slot;
        let mut var_fact1_dn3: f64 = *var_fact1_dn3_slot;
        let mut var_fact1_dn4: f64 = *var_fact1_dn4_slot;
        let mut var_fact1_dn5: f64 = *var_fact1_dn5_slot;
        let mut var_fact1_dn6: f64 = *var_fact1_dn6_slot;
        let mut var_fact1_rdb0: f64 = *var_fact1_rdb0_slot;
        let mut var_fact1_rdb1: f64 = *var_fact1_rdb1_slot;
        let mut var_fact1_rdb2: f64 = *var_fact1_rdb2_slot;
        let mut var_fact1_rdb3: f64 = *var_fact1_rdb3_slot;
        let mut var_fact1_rdb4: f64 = *var_fact1_rdb4_slot;
        let mut var_fact1_rdb5: f64 = *var_fact1_rdb5_slot;
        let mut var_fact1_rdb6: f64 = *var_fact1_rdb6_slot;
        let mut var_fact1_rdn0: f64 = *var_fact1_rdn0_slot;
        let mut var_fact1_rdn1: f64 = *var_fact1_rdn1_slot;
        let mut var_fact1_rdn2: f64 = *var_fact1_rdn2_slot;
        let mut var_fact1_rdn3: f64 = *var_fact1_rdn3_slot;
        let mut var_fact1_rdn4: f64 = *var_fact1_rdn4_slot;
        let mut var_fact1_rdn5: f64 = *var_fact1_rdn5_slot;
        let mut var_fact1_rdn6: f64 = *var_fact1_rdn6_slot;
        let mut var_fact1_rv: f64 = *var_fact1_rv_slot;
        let mut var_fact2: f64 = *var_fact2_slot;
        let mut var_fact2_db0: f64 = *var_fact2_db0_slot;
        let mut var_fact2_db1: f64 = *var_fact2_db1_slot;
        let mut var_fact2_db2: f64 = *var_fact2_db2_slot;
        let mut var_fact2_db3: f64 = *var_fact2_db3_slot;
        let mut var_fact2_db4: f64 = *var_fact2_db4_slot;
        let mut var_fact2_db5: f64 = *var_fact2_db5_slot;
        let mut var_fact2_db6: f64 = *var_fact2_db6_slot;
        let mut var_fact2_dn0: f64 = *var_fact2_dn0_slot;
        let mut var_fact2_dn1: f64 = *var_fact2_dn1_slot;
        let mut var_fact2_dn2: f64 = *var_fact2_dn2_slot;
        let mut var_fact2_dn3: f64 = *var_fact2_dn3_slot;
        let mut var_fact2_dn4: f64 = *var_fact2_dn4_slot;
        let mut var_fact2_dn5: f64 = *var_fact2_dn5_slot;
        let mut var_fact2_dn6: f64 = *var_fact2_dn6_slot;
        let mut var_fact2_rdb0: f64 = *var_fact2_rdb0_slot;
        let mut var_fact2_rdb1: f64 = *var_fact2_rdb1_slot;
        let mut var_fact2_rdb2: f64 = *var_fact2_rdb2_slot;
        let mut var_fact2_rdb3: f64 = *var_fact2_rdb3_slot;
        let mut var_fact2_rdb4: f64 = *var_fact2_rdb4_slot;
        let mut var_fact2_rdb5: f64 = *var_fact2_rdb5_slot;
        let mut var_fact2_rdb6: f64 = *var_fact2_rdb6_slot;
        let mut var_fact2_rdn0: f64 = *var_fact2_rdn0_slot;
        let mut var_fact2_rdn1: f64 = *var_fact2_rdn1_slot;
        let mut var_fact2_rdn2: f64 = *var_fact2_rdn2_slot;
        let mut var_fact2_rdn3: f64 = *var_fact2_rdn3_slot;
        let mut var_fact2_rdn4: f64 = *var_fact2_rdn4_slot;
        let mut var_fact2_rdn5: f64 = *var_fact2_rdn5_slot;
        let mut var_fact2_rdn6: f64 = *var_fact2_rdn6_slot;
        let mut var_fact2_rv: f64 = *var_fact2_rv_slot;
        let mut var_gmanew: f64 = *var_gmanew_slot;
        let mut var_gmanew_db0: f64 = *var_gmanew_db0_slot;
        let mut var_gmanew_db1: f64 = *var_gmanew_db1_slot;
        let mut var_gmanew_db2: f64 = *var_gmanew_db2_slot;
        let mut var_gmanew_db3: f64 = *var_gmanew_db3_slot;
        let mut var_gmanew_db4: f64 = *var_gmanew_db4_slot;
        let mut var_gmanew_db5: f64 = *var_gmanew_db5_slot;
        let mut var_gmanew_db6: f64 = *var_gmanew_db6_slot;
        let mut var_gmanew_dn0: f64 = *var_gmanew_dn0_slot;
        let mut var_gmanew_dn1: f64 = *var_gmanew_dn1_slot;
        let mut var_gmanew_dn2: f64 = *var_gmanew_dn2_slot;
        let mut var_gmanew_dn3: f64 = *var_gmanew_dn3_slot;
        let mut var_gmanew_dn4: f64 = *var_gmanew_dn4_slot;
        let mut var_gmanew_dn5: f64 = *var_gmanew_dn5_slot;
        let mut var_gmanew_dn6: f64 = *var_gmanew_dn6_slot;
        let mut var_gmanew_rdb0: f64 = *var_gmanew_rdb0_slot;
        let mut var_gmanew_rdb1: f64 = *var_gmanew_rdb1_slot;
        let mut var_gmanew_rdb2: f64 = *var_gmanew_rdb2_slot;
        let mut var_gmanew_rdb3: f64 = *var_gmanew_rdb3_slot;
        let mut var_gmanew_rdb4: f64 = *var_gmanew_rdb4_slot;
        let mut var_gmanew_rdb5: f64 = *var_gmanew_rdb5_slot;
        let mut var_gmanew_rdb6: f64 = *var_gmanew_rdb6_slot;
        let mut var_gmanew_rdn0: f64 = *var_gmanew_rdn0_slot;
        let mut var_gmanew_rdn1: f64 = *var_gmanew_rdn1_slot;
        let mut var_gmanew_rdn2: f64 = *var_gmanew_rdn2_slot;
        let mut var_gmanew_rdn3: f64 = *var_gmanew_rdn3_slot;
        let mut var_gmanew_rdn4: f64 = *var_gmanew_rdn4_slot;
        let mut var_gmanew_rdn5: f64 = *var_gmanew_rdn5_slot;
        let mut var_gmanew_rdn6: f64 = *var_gmanew_rdn6_slot;
        let mut var_gmanew_rv: f64 = *var_gmanew_rv_slot;
        let mut var_gmaold: f64 = *var_gmaold_slot;
        let mut var_gmaold_db0: f64 = *var_gmaold_db0_slot;
        let mut var_gmaold_db1: f64 = *var_gmaold_db1_slot;
        let mut var_gmaold_db2: f64 = *var_gmaold_db2_slot;
        let mut var_gmaold_db3: f64 = *var_gmaold_db3_slot;
        let mut var_gmaold_db4: f64 = *var_gmaold_db4_slot;
        let mut var_gmaold_db5: f64 = *var_gmaold_db5_slot;
        let mut var_gmaold_db6: f64 = *var_gmaold_db6_slot;
        let mut var_gmaold_dn0: f64 = *var_gmaold_dn0_slot;
        let mut var_gmaold_dn1: f64 = *var_gmaold_dn1_slot;
        let mut var_gmaold_dn2: f64 = *var_gmaold_dn2_slot;
        let mut var_gmaold_dn3: f64 = *var_gmaold_dn3_slot;
        let mut var_gmaold_dn4: f64 = *var_gmaold_dn4_slot;
        let mut var_gmaold_dn5: f64 = *var_gmaold_dn5_slot;
        let mut var_gmaold_dn6: f64 = *var_gmaold_dn6_slot;
        let mut var_gmaold_rdb0: f64 = *var_gmaold_rdb0_slot;
        let mut var_gmaold_rdb1: f64 = *var_gmaold_rdb1_slot;
        let mut var_gmaold_rdb2: f64 = *var_gmaold_rdb2_slot;
        let mut var_gmaold_rdb3: f64 = *var_gmaold_rdb3_slot;
        let mut var_gmaold_rdb4: f64 = *var_gmaold_rdb4_slot;
        let mut var_gmaold_rdb5: f64 = *var_gmaold_rdb5_slot;
        let mut var_gmaold_rdb6: f64 = *var_gmaold_rdb6_slot;
        let mut var_gmaold_rdn0: f64 = *var_gmaold_rdn0_slot;
        let mut var_gmaold_rdn1: f64 = *var_gmaold_rdn1_slot;
        let mut var_gmaold_rdn2: f64 = *var_gmaold_rdn2_slot;
        let mut var_gmaold_rdn3: f64 = *var_gmaold_rdn3_slot;
        let mut var_gmaold_rdn4: f64 = *var_gmaold_rdn4_slot;
        let mut var_gmaold_rdn5: f64 = *var_gmaold_rdn5_slot;
        let mut var_gmaold_rdn6: f64 = *var_gmaold_rdn6_slot;
        let mut var_gmaold_rv: f64 = *var_gmaold_rv_slot;
        let mut var_guard3: f64 = *var_guard3_slot;
        let mut var_guard3_db0: f64 = *var_guard3_db0_slot;
        let mut var_guard3_db1: f64 = *var_guard3_db1_slot;
        let mut var_guard3_db2: f64 = *var_guard3_db2_slot;
        let mut var_guard3_db3: f64 = *var_guard3_db3_slot;
        let mut var_guard3_db4: f64 = *var_guard3_db4_slot;
        let mut var_guard3_db5: f64 = *var_guard3_db5_slot;
        let mut var_guard3_db6: f64 = *var_guard3_db6_slot;
        let mut var_guard3_dn0: f64 = *var_guard3_dn0_slot;
        let mut var_guard3_dn1: f64 = *var_guard3_dn1_slot;
        let mut var_guard3_dn2: f64 = *var_guard3_dn2_slot;
        let mut var_guard3_dn3: f64 = *var_guard3_dn3_slot;
        let mut var_guard3_dn4: f64 = *var_guard3_dn4_slot;
        let mut var_guard3_dn5: f64 = *var_guard3_dn5_slot;
        let mut var_guard3_dn6: f64 = *var_guard3_dn6_slot;
        let mut var_guard3_rdb0: f64 = *var_guard3_rdb0_slot;
        let mut var_guard3_rdb1: f64 = *var_guard3_rdb1_slot;
        let mut var_guard3_rdb2: f64 = *var_guard3_rdb2_slot;
        let mut var_guard3_rdb3: f64 = *var_guard3_rdb3_slot;
        let mut var_guard3_rdb4: f64 = *var_guard3_rdb4_slot;
        let mut var_guard3_rdb5: f64 = *var_guard3_rdb5_slot;
        let mut var_guard3_rdb6: f64 = *var_guard3_rdb6_slot;
        let mut var_guard3_rdn0: f64 = *var_guard3_rdn0_slot;
        let mut var_guard3_rdn1: f64 = *var_guard3_rdn1_slot;
        let mut var_guard3_rdn2: f64 = *var_guard3_rdn2_slot;
        let mut var_guard3_rdn3: f64 = *var_guard3_rdn3_slot;
        let mut var_guard3_rdn4: f64 = *var_guard3_rdn4_slot;
        let mut var_guard3_rdn5: f64 = *var_guard3_rdn5_slot;
        let mut var_guard3_rdn6: f64 = *var_guard3_rdn6_slot;
        let mut var_guard3_rv: f64 = *var_guard3_rv_slot;
        let mut var_pbfact: f64 = *var_pbfact_slot;
        let mut var_pbfact_db0: f64 = *var_pbfact_db0_slot;
        let mut var_pbfact_db1: f64 = *var_pbfact_db1_slot;
        let mut var_pbfact_db2: f64 = *var_pbfact_db2_slot;
        let mut var_pbfact_db3: f64 = *var_pbfact_db3_slot;
        let mut var_pbfact_db4: f64 = *var_pbfact_db4_slot;
        let mut var_pbfact_db5: f64 = *var_pbfact_db5_slot;
        let mut var_pbfact_db6: f64 = *var_pbfact_db6_slot;
        let mut var_pbfact_dn0: f64 = *var_pbfact_dn0_slot;
        let mut var_pbfact_dn1: f64 = *var_pbfact_dn1_slot;
        let mut var_pbfact_dn2: f64 = *var_pbfact_dn2_slot;
        let mut var_pbfact_dn3: f64 = *var_pbfact_dn3_slot;
        let mut var_pbfact_dn4: f64 = *var_pbfact_dn4_slot;
        let mut var_pbfact_dn5: f64 = *var_pbfact_dn5_slot;
        let mut var_pbfact_dn6: f64 = *var_pbfact_dn6_slot;
        let mut var_pbfact_rdb0: f64 = *var_pbfact_rdb0_slot;
        let mut var_pbfact_rdb1: f64 = *var_pbfact_rdb1_slot;
        let mut var_pbfact_rdb2: f64 = *var_pbfact_rdb2_slot;
        let mut var_pbfact_rdb3: f64 = *var_pbfact_rdb3_slot;
        let mut var_pbfact_rdb4: f64 = *var_pbfact_rdb4_slot;
        let mut var_pbfact_rdb5: f64 = *var_pbfact_rdb5_slot;
        let mut var_pbfact_rdb6: f64 = *var_pbfact_rdb6_slot;
        let mut var_pbfact_rdn0: f64 = *var_pbfact_rdn0_slot;
        let mut var_pbfact_rdn1: f64 = *var_pbfact_rdn1_slot;
        let mut var_pbfact_rdn2: f64 = *var_pbfact_rdn2_slot;
        let mut var_pbfact_rdn3: f64 = *var_pbfact_rdn3_slot;
        let mut var_pbfact_rdn4: f64 = *var_pbfact_rdn4_slot;
        let mut var_pbfact_rdn5: f64 = *var_pbfact_rdn5_slot;
        let mut var_pbfact_rdn6: f64 = *var_pbfact_rdn6_slot;
        let mut var_pbfact_rv: f64 = *var_pbfact_rv_slot;
        let mut var_pbo: f64 = *var_pbo_slot;
        let mut var_pbo_db0: f64 = *var_pbo_db0_slot;
        let mut var_pbo_db1: f64 = *var_pbo_db1_slot;
        let mut var_pbo_db2: f64 = *var_pbo_db2_slot;
        let mut var_pbo_db3: f64 = *var_pbo_db3_slot;
        let mut var_pbo_db4: f64 = *var_pbo_db4_slot;
        let mut var_pbo_db5: f64 = *var_pbo_db5_slot;
        let mut var_pbo_db6: f64 = *var_pbo_db6_slot;
        let mut var_pbo_dn0: f64 = *var_pbo_dn0_slot;
        let mut var_pbo_dn1: f64 = *var_pbo_dn1_slot;
        let mut var_pbo_dn2: f64 = *var_pbo_dn2_slot;
        let mut var_pbo_dn3: f64 = *var_pbo_dn3_slot;
        let mut var_pbo_dn4: f64 = *var_pbo_dn4_slot;
        let mut var_pbo_dn5: f64 = *var_pbo_dn5_slot;
        let mut var_pbo_dn6: f64 = *var_pbo_dn6_slot;
        let mut var_pbo_rdb0: f64 = *var_pbo_rdb0_slot;
        let mut var_pbo_rdb1: f64 = *var_pbo_rdb1_slot;
        let mut var_pbo_rdb2: f64 = *var_pbo_rdb2_slot;
        let mut var_pbo_rdb3: f64 = *var_pbo_rdb3_slot;
        let mut var_pbo_rdb4: f64 = *var_pbo_rdb4_slot;
        let mut var_pbo_rdb5: f64 = *var_pbo_rdb5_slot;
        let mut var_pbo_rdb6: f64 = *var_pbo_rdb6_slot;
        let mut var_pbo_rdn0: f64 = *var_pbo_rdn0_slot;
        let mut var_pbo_rdn1: f64 = *var_pbo_rdn1_slot;
        let mut var_pbo_rdn2: f64 = *var_pbo_rdn2_slot;
        let mut var_pbo_rdn3: f64 = *var_pbo_rdn3_slot;
        let mut var_pbo_rdn4: f64 = *var_pbo_rdn4_slot;
        let mut var_pbo_rdn5: f64 = *var_pbo_rdn5_slot;
        let mut var_pbo_rdn6: f64 = *var_pbo_rdn6_slot;
        let mut var_pbo_rv: f64 = *var_pbo_rv_slot;
        let mut var_ttype: f64 = *var_ttype_slot;
        let mut var_ttype_db0: f64 = *var_ttype_db0_slot;
        let mut var_ttype_db1: f64 = *var_ttype_db1_slot;
        let mut var_ttype_db2: f64 = *var_ttype_db2_slot;
        let mut var_ttype_db3: f64 = *var_ttype_db3_slot;
        let mut var_ttype_db4: f64 = *var_ttype_db4_slot;
        let mut var_ttype_db5: f64 = *var_ttype_db5_slot;
        let mut var_ttype_db6: f64 = *var_ttype_db6_slot;
        let mut var_ttype_dn0: f64 = *var_ttype_dn0_slot;
        let mut var_ttype_dn1: f64 = *var_ttype_dn1_slot;
        let mut var_ttype_dn2: f64 = *var_ttype_dn2_slot;
        let mut var_ttype_dn3: f64 = *var_ttype_dn3_slot;
        let mut var_ttype_dn4: f64 = *var_ttype_dn4_slot;
        let mut var_ttype_dn5: f64 = *var_ttype_dn5_slot;
        let mut var_ttype_dn6: f64 = *var_ttype_dn6_slot;
        let mut var_ttype_rdb0: f64 = *var_ttype_rdb0_slot;
        let mut var_ttype_rdb1: f64 = *var_ttype_rdb1_slot;
        let mut var_ttype_rdb2: f64 = *var_ttype_rdb2_slot;
        let mut var_ttype_rdb3: f64 = *var_ttype_rdb3_slot;
        let mut var_ttype_rdb4: f64 = *var_ttype_rdb4_slot;
        let mut var_ttype_rdb5: f64 = *var_ttype_rdb5_slot;
        let mut var_ttype_rdb6: f64 = *var_ttype_rdb6_slot;
        let mut var_ttype_rdn0: f64 = *var_ttype_rdn0_slot;
        let mut var_ttype_rdn1: f64 = *var_ttype_rdn1_slot;
        let mut var_ttype_rdn2: f64 = *var_ttype_rdn2_slot;
        let mut var_ttype_rdn3: f64 = *var_ttype_rdn3_slot;
        let mut var_ttype_rdn4: f64 = *var_ttype_rdn4_slot;
        let mut var_ttype_rdn5: f64 = *var_ttype_rdn5_slot;
        let mut var_ttype_rdn6: f64 = *var_ttype_rdn6_slot;
        let mut var_ttype_rv: f64 = *var_ttype_rv_slot;
        let mut var_vbiei: f64 = *var_vbiei_slot;
        let mut var_vbiei_db0: f64 = *var_vbiei_db0_slot;
        let mut var_vbiei_db1: f64 = *var_vbiei_db1_slot;
        let mut var_vbiei_db2: f64 = *var_vbiei_db2_slot;
        let mut var_vbiei_db3: f64 = *var_vbiei_db3_slot;
        let mut var_vbiei_db4: f64 = *var_vbiei_db4_slot;
        let mut var_vbiei_db5: f64 = *var_vbiei_db5_slot;
        let mut var_vbiei_db6: f64 = *var_vbiei_db6_slot;
        let mut var_vbiei_dn0: f64 = *var_vbiei_dn0_slot;
        let mut var_vbiei_dn1: f64 = *var_vbiei_dn1_slot;
        let mut var_vbiei_dn2: f64 = *var_vbiei_dn2_slot;
        let mut var_vbiei_dn3: f64 = *var_vbiei_dn3_slot;
        let mut var_vbiei_dn4: f64 = *var_vbiei_dn4_slot;
        let mut var_vbiei_dn5: f64 = *var_vbiei_dn5_slot;
        let mut var_vbiei_dn6: f64 = *var_vbiei_dn6_slot;
        let mut var_vbiei_rdb0: f64 = *var_vbiei_rdb0_slot;
        let mut var_vbiei_rdb1: f64 = *var_vbiei_rdb1_slot;
        let mut var_vbiei_rdb2: f64 = *var_vbiei_rdb2_slot;
        let mut var_vbiei_rdb3: f64 = *var_vbiei_rdb3_slot;
        let mut var_vbiei_rdb4: f64 = *var_vbiei_rdb4_slot;
        let mut var_vbiei_rdb5: f64 = *var_vbiei_rdb5_slot;
        let mut var_vbiei_rdb6: f64 = *var_vbiei_rdb6_slot;
        let mut var_vbiei_rdn0: f64 = *var_vbiei_rdn0_slot;
        let mut var_vbiei_rdn1: f64 = *var_vbiei_rdn1_slot;
        let mut var_vbiei_rdn2: f64 = *var_vbiei_rdn2_slot;
        let mut var_vbiei_rdn3: f64 = *var_vbiei_rdn3_slot;
        let mut var_vbiei_rdn4: f64 = *var_vbiei_rdn4_slot;
        let mut var_vbiei_rdn5: f64 = *var_vbiei_rdn5_slot;
        let mut var_vbiei_rdn6: f64 = *var_vbiei_rdn6_slot;
        let mut var_vbiei_rv: f64 = *var_vbiei_rv_slot;
        let mut var_vje_t: f64 = *var_vje_t_slot;
        let mut var_vje_t_db0: f64 = *var_vje_t_db0_slot;
        let mut var_vje_t_db1: f64 = *var_vje_t_db1_slot;
        let mut var_vje_t_db2: f64 = *var_vje_t_db2_slot;
        let mut var_vje_t_db3: f64 = *var_vje_t_db3_slot;
        let mut var_vje_t_db4: f64 = *var_vje_t_db4_slot;
        let mut var_vje_t_db5: f64 = *var_vje_t_db5_slot;
        let mut var_vje_t_db6: f64 = *var_vje_t_db6_slot;
        let mut var_vje_t_dn0: f64 = *var_vje_t_dn0_slot;
        let mut var_vje_t_dn1: f64 = *var_vje_t_dn1_slot;
        let mut var_vje_t_dn2: f64 = *var_vje_t_dn2_slot;
        let mut var_vje_t_dn3: f64 = *var_vje_t_dn3_slot;
        let mut var_vje_t_dn4: f64 = *var_vje_t_dn4_slot;
        let mut var_vje_t_dn5: f64 = *var_vje_t_dn5_slot;
        let mut var_vje_t_dn6: f64 = *var_vje_t_dn6_slot;
        let mut var_vje_t_rdb0: f64 = *var_vje_t_rdb0_slot;
        let mut var_vje_t_rdb1: f64 = *var_vje_t_rdb1_slot;
        let mut var_vje_t_rdb2: f64 = *var_vje_t_rdb2_slot;
        let mut var_vje_t_rdb3: f64 = *var_vje_t_rdb3_slot;
        let mut var_vje_t_rdb4: f64 = *var_vje_t_rdb4_slot;
        let mut var_vje_t_rdb5: f64 = *var_vje_t_rdb5_slot;
        let mut var_vje_t_rdb6: f64 = *var_vje_t_rdb6_slot;
        let mut var_vje_t_rdn0: f64 = *var_vje_t_rdn0_slot;
        let mut var_vje_t_rdn1: f64 = *var_vje_t_rdn1_slot;
        let mut var_vje_t_rdn2: f64 = *var_vje_t_rdn2_slot;
        let mut var_vje_t_rdn3: f64 = *var_vje_t_rdn3_slot;
        let mut var_vje_t_rdn4: f64 = *var_vje_t_rdn4_slot;
        let mut var_vje_t_rdn5: f64 = *var_vje_t_rdn5_slot;
        let mut var_vje_t_rdn6: f64 = *var_vje_t_rdn6_slot;
        let mut var_vje_t_rv: f64 = *var_vje_t_rv_slot;

        var_cje_i = p.p16;
        var_cje_i_dn0 = 0.0;
        var_cje_i_dn1 = 0.0;
        var_cje_i_dn2 = 0.0;
        var_cje_i_dn3 = 0.0;
        var_cje_i_dn4 = 0.0;
        var_cje_i_dn5 = 0.0;
        var_cje_i_dn6 = 0.0;
        var_cje_i_db0 = 0.0;
        var_cje_i_db1 = 0.0;
        var_cje_i_db2 = 0.0;
        var_cje_i_db3 = 0.0;
        var_cje_i_db4 = 0.0;
        var_cje_i_db5 = 0.0;
        var_cje_i_db6 = 0.0;
        var_cje_i_rv = 0.0;
        var_cje_i_rdn0 = 0.0;
        var_cje_i_rdn1 = 0.0;
        var_cje_i_rdn2 = 0.0;
        var_cje_i_rdn3 = 0.0;
        var_cje_i_rdn4 = 0.0;
        var_cje_i_rdn5 = 0.0;
        var_cje_i_rdn6 = 0.0;
        var_cje_i_rdb0 = 0.0;
        var_cje_i_rdb1 = 0.0;
        var_cje_i_rdb2 = 0.0;
        var_cje_i_rdb3 = 0.0;
        var_cje_i_rdb4 = 0.0;
        var_cje_i_rdb5 = 0.0;
        var_cje_i_rdb6 = 0.0;

        let assign170_e401: f64 = (var_tnom / 300.15);
        var_fact1 = assign170_e401;
        var_fact1_dn0 = (var_tnom_dn0 / 300.15);
        var_fact1_dn1 = (var_tnom_dn1 / 300.15);
        var_fact1_dn2 = (var_tnom_dn2 / 300.15);
        var_fact1_dn3 = (var_tnom_dn3 / 300.15);
        var_fact1_dn4 = (var_tnom_dn4 / 300.15);
        var_fact1_dn5 = (var_tnom_dn5 / 300.15);
        var_fact1_dn6 = (var_tnom_dn6 / 300.15);
        var_fact1_db0 = (var_tnom_db0 / 300.15);
        var_fact1_db1 = (var_tnom_db1 / 300.15);
        var_fact1_db2 = (var_tnom_db2 / 300.15);
        var_fact1_db3 = (var_tnom_db3 / 300.15);
        var_fact1_db4 = (var_tnom_db4 / 300.15);
        var_fact1_db5 = (var_tnom_db5 / 300.15);
        var_fact1_db6 = (var_tnom_db6 / 300.15);
        var_fact1_rv = 0.0;
        var_fact1_rdn0 = 0.0;
        var_fact1_rdn1 = 0.0;
        var_fact1_rdn2 = 0.0;
        var_fact1_rdn3 = 0.0;
        var_fact1_rdn4 = 0.0;
        var_fact1_rdn5 = 0.0;
        var_fact1_rdn6 = 0.0;
        var_fact1_rdb0 = 0.0;
        var_fact1_rdb1 = 0.0;
        var_fact1_rdb2 = 0.0;
        var_fact1_rdb3 = 0.0;
        var_fact1_rdb4 = 0.0;
        var_fact1_rdb5 = 0.0;
        var_fact1_rdb6 = 0.0;

        let assign180_e404: f64 = (var_tdev / 300.15);
        var_fact2 = assign180_e404;
        var_fact2_dn0 = (var_tdev_dn0 / 300.15);
        var_fact2_dn1 = (var_tdev_dn1 / 300.15);
        var_fact2_dn2 = (var_tdev_dn2 / 300.15);
        var_fact2_dn3 = (var_tdev_dn3 / 300.15);
        var_fact2_dn4 = (var_tdev_dn4 / 300.15);
        var_fact2_dn5 = (var_tdev_dn5 / 300.15);
        var_fact2_dn6 = (var_tdev_dn6 / 300.15);
        var_fact2_db0 = (var_tdev_db0 / 300.15);
        var_fact2_db1 = (var_tdev_db1 / 300.15);
        var_fact2_db2 = (var_tdev_db2 / 300.15);
        var_fact2_db3 = (var_tdev_db3 / 300.15);
        var_fact2_db4 = (var_tdev_db4 / 300.15);
        var_fact2_db5 = (var_tdev_db5 / 300.15);
        var_fact2_db6 = (var_tdev_db6 / 300.15);
        var_fact2_rv = 0.0;
        var_fact2_rdn0 = 0.0;
        var_fact2_rdn1 = 0.0;
        var_fact2_rdn2 = 0.0;
        var_fact2_rdn3 = 0.0;
        var_fact2_rdn4 = 0.0;
        var_fact2_rdn5 = 0.0;
        var_fact2_rdn6 = 0.0;
        var_fact2_rdb0 = 0.0;
        var_fact2_rdb1 = 0.0;
        var_fact2_rdb2 = 0.0;
        var_fact2_rdb3 = 0.0;
        var_fact2_rdb4 = 0.0;
        var_fact2_rdb5 = 0.0;
        var_fact2_rdb6 = 0.0;

        let assign190_e408: f64 = (0.000702 * var_tdev);
        let assign190_e410: f64 = (assign190_e408 * var_tdev);
        let assign190_e413: f64 = (1108.0 + var_tdev);
        let assign190_e414: f64 = (assign190_e410 / assign190_e413);
        let assign190_e415: f64 = (1.16 - assign190_e414);
        var_egfet = assign190_e415;
        var_egfet_dn0 = (-((((((0.000702 * var_tdev_dn0) * var_tdev) + (assign190_e408 * var_tdev_dn0)) * assign190_e413) - (assign190_e410 * var_tdev_dn0)) / (assign190_e413 * assign190_e413)));
        var_egfet_dn1 = (-((((((0.000702 * var_tdev_dn1) * var_tdev) + (assign190_e408 * var_tdev_dn1)) * assign190_e413) - (assign190_e410 * var_tdev_dn1)) / (assign190_e413 * assign190_e413)));
        var_egfet_dn2 = (-((((((0.000702 * var_tdev_dn2) * var_tdev) + (assign190_e408 * var_tdev_dn2)) * assign190_e413) - (assign190_e410 * var_tdev_dn2)) / (assign190_e413 * assign190_e413)));
        var_egfet_dn3 = (-((((((0.000702 * var_tdev_dn3) * var_tdev) + (assign190_e408 * var_tdev_dn3)) * assign190_e413) - (assign190_e410 * var_tdev_dn3)) / (assign190_e413 * assign190_e413)));
        var_egfet_dn4 = (-((((((0.000702 * var_tdev_dn4) * var_tdev) + (assign190_e408 * var_tdev_dn4)) * assign190_e413) - (assign190_e410 * var_tdev_dn4)) / (assign190_e413 * assign190_e413)));
        var_egfet_dn5 = (-((((((0.000702 * var_tdev_dn5) * var_tdev) + (assign190_e408 * var_tdev_dn5)) * assign190_e413) - (assign190_e410 * var_tdev_dn5)) / (assign190_e413 * assign190_e413)));
        var_egfet_dn6 = (-((((((0.000702 * var_tdev_dn6) * var_tdev) + (assign190_e408 * var_tdev_dn6)) * assign190_e413) - (assign190_e410 * var_tdev_dn6)) / (assign190_e413 * assign190_e413)));
        var_egfet_db0 = (-((((((0.000702 * var_tdev_db0) * var_tdev) + (assign190_e408 * var_tdev_db0)) * assign190_e413) - (assign190_e410 * var_tdev_db0)) / (assign190_e413 * assign190_e413)));
        var_egfet_db1 = (-((((((0.000702 * var_tdev_db1) * var_tdev) + (assign190_e408 * var_tdev_db1)) * assign190_e413) - (assign190_e410 * var_tdev_db1)) / (assign190_e413 * assign190_e413)));
        var_egfet_db2 = (-((((((0.000702 * var_tdev_db2) * var_tdev) + (assign190_e408 * var_tdev_db2)) * assign190_e413) - (assign190_e410 * var_tdev_db2)) / (assign190_e413 * assign190_e413)));
        var_egfet_db3 = (-((((((0.000702 * var_tdev_db3) * var_tdev) + (assign190_e408 * var_tdev_db3)) * assign190_e413) - (assign190_e410 * var_tdev_db3)) / (assign190_e413 * assign190_e413)));
        var_egfet_db4 = (-((((((0.000702 * var_tdev_db4) * var_tdev) + (assign190_e408 * var_tdev_db4)) * assign190_e413) - (assign190_e410 * var_tdev_db4)) / (assign190_e413 * assign190_e413)));
        var_egfet_db5 = (-((((((0.000702 * var_tdev_db5) * var_tdev) + (assign190_e408 * var_tdev_db5)) * assign190_e413) - (assign190_e410 * var_tdev_db5)) / (assign190_e413 * assign190_e413)));
        var_egfet_db6 = (-((((((0.000702 * var_tdev_db6) * var_tdev) + (assign190_e408 * var_tdev_db6)) * assign190_e413) - (assign190_e410 * var_tdev_db6)) / (assign190_e413 * assign190_e413)));
        var_egfet_rv = 0.0;
        var_egfet_rdn0 = 0.0;
        var_egfet_rdn1 = 0.0;
        var_egfet_rdn2 = 0.0;
        var_egfet_rdn3 = 0.0;
        var_egfet_rdn4 = 0.0;
        var_egfet_rdn5 = 0.0;
        var_egfet_rdn6 = 0.0;
        var_egfet_rdb0 = 0.0;
        var_egfet_rdb1 = 0.0;
        var_egfet_rdb2 = 0.0;
        var_egfet_rdb3 = 0.0;
        var_egfet_rdb4 = 0.0;
        var_egfet_rdb5 = 0.0;
        var_egfet_rdb6 = 0.0;

        let assign200_e417: f64 = (-var_egfet);
        let assign200_e421: f64 = (var_tdev + var_tdev);
        let assign200_e422: f64 = (1.3806226e-23 * assign200_e421);
        let assign200_e423: f64 = (assign200_e417 / assign200_e422);
        let assign200_e428: f64 = (300.15 + 300.15);
        let assign200_e429: f64 = (1.3806226e-23 * assign200_e428);
        let assign200_e430: f64 = (1.1150877 / assign200_e429);
        let assign200_e431: f64 = (assign200_e423 + assign200_e430);
        var_arg0 = assign200_e431;
        var_arg0_dn0 = ((((-var_egfet_dn0) * assign200_e422) - (assign200_e417 * (1.3806226e-23 * (var_tdev_dn0 + var_tdev_dn0)))) / (assign200_e422 * assign200_e422));
        var_arg0_dn1 = ((((-var_egfet_dn1) * assign200_e422) - (assign200_e417 * (1.3806226e-23 * (var_tdev_dn1 + var_tdev_dn1)))) / (assign200_e422 * assign200_e422));
        var_arg0_dn2 = ((((-var_egfet_dn2) * assign200_e422) - (assign200_e417 * (1.3806226e-23 * (var_tdev_dn2 + var_tdev_dn2)))) / (assign200_e422 * assign200_e422));
        var_arg0_dn3 = ((((-var_egfet_dn3) * assign200_e422) - (assign200_e417 * (1.3806226e-23 * (var_tdev_dn3 + var_tdev_dn3)))) / (assign200_e422 * assign200_e422));
        var_arg0_dn4 = ((((-var_egfet_dn4) * assign200_e422) - (assign200_e417 * (1.3806226e-23 * (var_tdev_dn4 + var_tdev_dn4)))) / (assign200_e422 * assign200_e422));
        var_arg0_dn5 = ((((-var_egfet_dn5) * assign200_e422) - (assign200_e417 * (1.3806226e-23 * (var_tdev_dn5 + var_tdev_dn5)))) / (assign200_e422 * assign200_e422));
        var_arg0_dn6 = ((((-var_egfet_dn6) * assign200_e422) - (assign200_e417 * (1.3806226e-23 * (var_tdev_dn6 + var_tdev_dn6)))) / (assign200_e422 * assign200_e422));
        var_arg0_db0 = ((((-var_egfet_db0) * assign200_e422) - (assign200_e417 * (1.3806226e-23 * (var_tdev_db0 + var_tdev_db0)))) / (assign200_e422 * assign200_e422));
        var_arg0_db1 = ((((-var_egfet_db1) * assign200_e422) - (assign200_e417 * (1.3806226e-23 * (var_tdev_db1 + var_tdev_db1)))) / (assign200_e422 * assign200_e422));
        var_arg0_db2 = ((((-var_egfet_db2) * assign200_e422) - (assign200_e417 * (1.3806226e-23 * (var_tdev_db2 + var_tdev_db2)))) / (assign200_e422 * assign200_e422));
        var_arg0_db3 = ((((-var_egfet_db3) * assign200_e422) - (assign200_e417 * (1.3806226e-23 * (var_tdev_db3 + var_tdev_db3)))) / (assign200_e422 * assign200_e422));
        var_arg0_db4 = ((((-var_egfet_db4) * assign200_e422) - (assign200_e417 * (1.3806226e-23 * (var_tdev_db4 + var_tdev_db4)))) / (assign200_e422 * assign200_e422));
        var_arg0_db5 = ((((-var_egfet_db5) * assign200_e422) - (assign200_e417 * (1.3806226e-23 * (var_tdev_db5 + var_tdev_db5)))) / (assign200_e422 * assign200_e422));
        var_arg0_db6 = ((((-var_egfet_db6) * assign200_e422) - (assign200_e417 * (1.3806226e-23 * (var_tdev_db6 + var_tdev_db6)))) / (assign200_e422 * assign200_e422));
        var_arg0_rv = 0.0;
        var_arg0_rdn0 = 0.0;
        var_arg0_rdn1 = 0.0;
        var_arg0_rdn2 = 0.0;
        var_arg0_rdn3 = 0.0;
        var_arg0_rdn4 = 0.0;
        var_arg0_rdn5 = 0.0;
        var_arg0_rdn6 = 0.0;
        var_arg0_rdb0 = 0.0;
        var_arg0_rdb1 = 0.0;
        var_arg0_rdb2 = 0.0;
        var_arg0_rdb3 = 0.0;
        var_arg0_rdb4 = 0.0;
        var_arg0_rdb5 = 0.0;
        var_arg0_rdb6 = 0.0;

        let assign210_e434: f64 = (var_vt + var_vt);
        let assign210_e435: f64 = (-assign210_e434);
        let assign210_e438: f64 = (var_fact2).ln();
        let assign210_e439: f64 = (1.5 * assign210_e438);
        let assign210_e442: f64 = (1.6021918e-19 * var_arg0);
        let assign210_e443: f64 = (assign210_e439 + assign210_e442);
        let assign210_e444: f64 = (assign210_e435 * assign210_e443);
        var_pbfact = assign210_e444;
        var_pbfact_dn0 = (((-(var_vt_dn0 + var_vt_dn0)) * assign210_e443) + (assign210_e435 * ((1.5 * (var_fact2_dn0 / var_fact2)) + (1.6021918e-19 * var_arg0_dn0))));
        var_pbfact_dn1 = (((-(var_vt_dn1 + var_vt_dn1)) * assign210_e443) + (assign210_e435 * ((1.5 * (var_fact2_dn1 / var_fact2)) + (1.6021918e-19 * var_arg0_dn1))));
        var_pbfact_dn2 = (((-(var_vt_dn2 + var_vt_dn2)) * assign210_e443) + (assign210_e435 * ((1.5 * (var_fact2_dn2 / var_fact2)) + (1.6021918e-19 * var_arg0_dn2))));
        var_pbfact_dn3 = (((-(var_vt_dn3 + var_vt_dn3)) * assign210_e443) + (assign210_e435 * ((1.5 * (var_fact2_dn3 / var_fact2)) + (1.6021918e-19 * var_arg0_dn3))));
        var_pbfact_dn4 = (((-(var_vt_dn4 + var_vt_dn4)) * assign210_e443) + (assign210_e435 * ((1.5 * (var_fact2_dn4 / var_fact2)) + (1.6021918e-19 * var_arg0_dn4))));
        var_pbfact_dn5 = (((-(var_vt_dn5 + var_vt_dn5)) * assign210_e443) + (assign210_e435 * ((1.5 * (var_fact2_dn5 / var_fact2)) + (1.6021918e-19 * var_arg0_dn5))));
        var_pbfact_dn6 = (((-(var_vt_dn6 + var_vt_dn6)) * assign210_e443) + (assign210_e435 * ((1.5 * (var_fact2_dn6 / var_fact2)) + (1.6021918e-19 * var_arg0_dn6))));
        var_pbfact_db0 = (((-(var_vt_db0 + var_vt_db0)) * assign210_e443) + (assign210_e435 * ((1.5 * (var_fact2_db0 / var_fact2)) + (1.6021918e-19 * var_arg0_db0))));
        var_pbfact_db1 = (((-(var_vt_db1 + var_vt_db1)) * assign210_e443) + (assign210_e435 * ((1.5 * (var_fact2_db1 / var_fact2)) + (1.6021918e-19 * var_arg0_db1))));
        var_pbfact_db2 = (((-(var_vt_db2 + var_vt_db2)) * assign210_e443) + (assign210_e435 * ((1.5 * (var_fact2_db2 / var_fact2)) + (1.6021918e-19 * var_arg0_db2))));
        var_pbfact_db3 = (((-(var_vt_db3 + var_vt_db3)) * assign210_e443) + (assign210_e435 * ((1.5 * (var_fact2_db3 / var_fact2)) + (1.6021918e-19 * var_arg0_db3))));
        var_pbfact_db4 = (((-(var_vt_db4 + var_vt_db4)) * assign210_e443) + (assign210_e435 * ((1.5 * (var_fact2_db4 / var_fact2)) + (1.6021918e-19 * var_arg0_db4))));
        var_pbfact_db5 = (((-(var_vt_db5 + var_vt_db5)) * assign210_e443) + (assign210_e435 * ((1.5 * (var_fact2_db5 / var_fact2)) + (1.6021918e-19 * var_arg0_db5))));
        var_pbfact_db6 = (((-(var_vt_db6 + var_vt_db6)) * assign210_e443) + (assign210_e435 * ((1.5 * (var_fact2_db6 / var_fact2)) + (1.6021918e-19 * var_arg0_db6))));
        var_pbfact_rv = 0.0;
        var_pbfact_rdn0 = 0.0;
        var_pbfact_rdn1 = 0.0;
        var_pbfact_rdn2 = 0.0;
        var_pbfact_rdn3 = 0.0;
        var_pbfact_rdn4 = 0.0;
        var_pbfact_rdn5 = 0.0;
        var_pbfact_rdn6 = 0.0;
        var_pbfact_rdb0 = 0.0;
        var_pbfact_rdb1 = 0.0;
        var_pbfact_rdb2 = 0.0;
        var_pbfact_rdb3 = 0.0;
        var_pbfact_rdb4 = 0.0;
        var_pbfact_rdb5 = 0.0;
        var_pbfact_rdb6 = 0.0;

        let assign220_e447: f64 = (p.p17 - var_pbfact);
        let assign220_e449: f64 = (assign220_e447 / var_fact1);
        var_pbo = assign220_e449;
        var_pbo_dn0 = ((((-var_pbfact_dn0) * var_fact1) - (assign220_e447 * var_fact1_dn0)) / (var_fact1 * var_fact1));
        var_pbo_dn1 = ((((-var_pbfact_dn1) * var_fact1) - (assign220_e447 * var_fact1_dn1)) / (var_fact1 * var_fact1));
        var_pbo_dn2 = ((((-var_pbfact_dn2) * var_fact1) - (assign220_e447 * var_fact1_dn2)) / (var_fact1 * var_fact1));
        var_pbo_dn3 = ((((-var_pbfact_dn3) * var_fact1) - (assign220_e447 * var_fact1_dn3)) / (var_fact1 * var_fact1));
        var_pbo_dn4 = ((((-var_pbfact_dn4) * var_fact1) - (assign220_e447 * var_fact1_dn4)) / (var_fact1 * var_fact1));
        var_pbo_dn5 = ((((-var_pbfact_dn5) * var_fact1) - (assign220_e447 * var_fact1_dn5)) / (var_fact1 * var_fact1));
        var_pbo_dn6 = ((((-var_pbfact_dn6) * var_fact1) - (assign220_e447 * var_fact1_dn6)) / (var_fact1 * var_fact1));
        var_pbo_db0 = ((((-var_pbfact_db0) * var_fact1) - (assign220_e447 * var_fact1_db0)) / (var_fact1 * var_fact1));
        var_pbo_db1 = ((((-var_pbfact_db1) * var_fact1) - (assign220_e447 * var_fact1_db1)) / (var_fact1 * var_fact1));
        var_pbo_db2 = ((((-var_pbfact_db2) * var_fact1) - (assign220_e447 * var_fact1_db2)) / (var_fact1 * var_fact1));
        var_pbo_db3 = ((((-var_pbfact_db3) * var_fact1) - (assign220_e447 * var_fact1_db3)) / (var_fact1 * var_fact1));
        var_pbo_db4 = ((((-var_pbfact_db4) * var_fact1) - (assign220_e447 * var_fact1_db4)) / (var_fact1 * var_fact1));
        var_pbo_db5 = ((((-var_pbfact_db5) * var_fact1) - (assign220_e447 * var_fact1_db5)) / (var_fact1 * var_fact1));
        var_pbo_db6 = ((((-var_pbfact_db6) * var_fact1) - (assign220_e447 * var_fact1_db6)) / (var_fact1 * var_fact1));
        var_pbo_rv = 0.0;
        var_pbo_rdn0 = 0.0;
        var_pbo_rdn1 = 0.0;
        var_pbo_rdn2 = 0.0;
        var_pbo_rdn3 = 0.0;
        var_pbo_rdn4 = 0.0;
        var_pbo_rdn5 = 0.0;
        var_pbo_rdn6 = 0.0;
        var_pbo_rdb0 = 0.0;
        var_pbo_rdb1 = 0.0;
        var_pbo_rdb2 = 0.0;
        var_pbo_rdb3 = 0.0;
        var_pbo_rdb4 = 0.0;
        var_pbo_rdb5 = 0.0;
        var_pbo_rdb6 = 0.0;

        let assign230_e452: f64 = (p.p17 - var_pbo);
        let assign230_e454: f64 = (assign230_e452 / var_pbo);
        var_gmaold = assign230_e454;
        var_gmaold_dn0 = ((((-var_pbo_dn0) * var_pbo) - (assign230_e452 * var_pbo_dn0)) / (var_pbo * var_pbo));
        var_gmaold_dn1 = ((((-var_pbo_dn1) * var_pbo) - (assign230_e452 * var_pbo_dn1)) / (var_pbo * var_pbo));
        var_gmaold_dn2 = ((((-var_pbo_dn2) * var_pbo) - (assign230_e452 * var_pbo_dn2)) / (var_pbo * var_pbo));
        var_gmaold_dn3 = ((((-var_pbo_dn3) * var_pbo) - (assign230_e452 * var_pbo_dn3)) / (var_pbo * var_pbo));
        var_gmaold_dn4 = ((((-var_pbo_dn4) * var_pbo) - (assign230_e452 * var_pbo_dn4)) / (var_pbo * var_pbo));
        var_gmaold_dn5 = ((((-var_pbo_dn5) * var_pbo) - (assign230_e452 * var_pbo_dn5)) / (var_pbo * var_pbo));
        var_gmaold_dn6 = ((((-var_pbo_dn6) * var_pbo) - (assign230_e452 * var_pbo_dn6)) / (var_pbo * var_pbo));
        var_gmaold_db0 = ((((-var_pbo_db0) * var_pbo) - (assign230_e452 * var_pbo_db0)) / (var_pbo * var_pbo));
        var_gmaold_db1 = ((((-var_pbo_db1) * var_pbo) - (assign230_e452 * var_pbo_db1)) / (var_pbo * var_pbo));
        var_gmaold_db2 = ((((-var_pbo_db2) * var_pbo) - (assign230_e452 * var_pbo_db2)) / (var_pbo * var_pbo));
        var_gmaold_db3 = ((((-var_pbo_db3) * var_pbo) - (assign230_e452 * var_pbo_db3)) / (var_pbo * var_pbo));
        var_gmaold_db4 = ((((-var_pbo_db4) * var_pbo) - (assign230_e452 * var_pbo_db4)) / (var_pbo * var_pbo));
        var_gmaold_db5 = ((((-var_pbo_db5) * var_pbo) - (assign230_e452 * var_pbo_db5)) / (var_pbo * var_pbo));
        var_gmaold_db6 = ((((-var_pbo_db6) * var_pbo) - (assign230_e452 * var_pbo_db6)) / (var_pbo * var_pbo));
        var_gmaold_rv = 0.0;
        var_gmaold_rdn0 = 0.0;
        var_gmaold_rdn1 = 0.0;
        var_gmaold_rdn2 = 0.0;
        var_gmaold_rdn3 = 0.0;
        var_gmaold_rdn4 = 0.0;
        var_gmaold_rdn5 = 0.0;
        var_gmaold_rdn6 = 0.0;
        var_gmaold_rdb0 = 0.0;
        var_gmaold_rdb1 = 0.0;
        var_gmaold_rdb2 = 0.0;
        var_gmaold_rdb3 = 0.0;
        var_gmaold_rdb4 = 0.0;
        var_gmaold_rdb5 = 0.0;
        var_gmaold_rdb6 = 0.0;

        let assign240_e461: f64 = (var_tnom - 300.15);
        let assign240_e462: f64 = (0.0004 * assign240_e461);
        let assign240_e464: f64 = (assign240_e462 - var_gmaold);
        let assign240_e465: f64 = (p.p18 * assign240_e464);
        let assign240_e466: f64 = (1.0 + assign240_e465);
        let assign240_e467: f64 = (var_cje_i / assign240_e466);
        var_cjt = assign240_e467;
        var_cjt_dn0 = (((var_cje_i_dn0 * assign240_e466) - (var_cje_i * (p.p18 * ((0.0004 * var_tnom_dn0) - var_gmaold_dn0)))) / (assign240_e466 * assign240_e466));
        var_cjt_dn1 = (((var_cje_i_dn1 * assign240_e466) - (var_cje_i * (p.p18 * ((0.0004 * var_tnom_dn1) - var_gmaold_dn1)))) / (assign240_e466 * assign240_e466));
        var_cjt_dn2 = (((var_cje_i_dn2 * assign240_e466) - (var_cje_i * (p.p18 * ((0.0004 * var_tnom_dn2) - var_gmaold_dn2)))) / (assign240_e466 * assign240_e466));
        var_cjt_dn3 = (((var_cje_i_dn3 * assign240_e466) - (var_cje_i * (p.p18 * ((0.0004 * var_tnom_dn3) - var_gmaold_dn3)))) / (assign240_e466 * assign240_e466));
        var_cjt_dn4 = (((var_cje_i_dn4 * assign240_e466) - (var_cje_i * (p.p18 * ((0.0004 * var_tnom_dn4) - var_gmaold_dn4)))) / (assign240_e466 * assign240_e466));
        var_cjt_dn5 = (((var_cje_i_dn5 * assign240_e466) - (var_cje_i * (p.p18 * ((0.0004 * var_tnom_dn5) - var_gmaold_dn5)))) / (assign240_e466 * assign240_e466));
        var_cjt_dn6 = (((var_cje_i_dn6 * assign240_e466) - (var_cje_i * (p.p18 * ((0.0004 * var_tnom_dn6) - var_gmaold_dn6)))) / (assign240_e466 * assign240_e466));
        var_cjt_db0 = (((var_cje_i_db0 * assign240_e466) - (var_cje_i * (p.p18 * ((0.0004 * var_tnom_db0) - var_gmaold_db0)))) / (assign240_e466 * assign240_e466));
        var_cjt_db1 = (((var_cje_i_db1 * assign240_e466) - (var_cje_i * (p.p18 * ((0.0004 * var_tnom_db1) - var_gmaold_db1)))) / (assign240_e466 * assign240_e466));
        var_cjt_db2 = (((var_cje_i_db2 * assign240_e466) - (var_cje_i * (p.p18 * ((0.0004 * var_tnom_db2) - var_gmaold_db2)))) / (assign240_e466 * assign240_e466));
        var_cjt_db3 = (((var_cje_i_db3 * assign240_e466) - (var_cje_i * (p.p18 * ((0.0004 * var_tnom_db3) - var_gmaold_db3)))) / (assign240_e466 * assign240_e466));
        var_cjt_db4 = (((var_cje_i_db4 * assign240_e466) - (var_cje_i * (p.p18 * ((0.0004 * var_tnom_db4) - var_gmaold_db4)))) / (assign240_e466 * assign240_e466));
        var_cjt_db5 = (((var_cje_i_db5 * assign240_e466) - (var_cje_i * (p.p18 * ((0.0004 * var_tnom_db5) - var_gmaold_db5)))) / (assign240_e466 * assign240_e466));
        var_cjt_db6 = (((var_cje_i_db6 * assign240_e466) - (var_cje_i * (p.p18 * ((0.0004 * var_tnom_db6) - var_gmaold_db6)))) / (assign240_e466 * assign240_e466));
        var_cjt_rv = 0.0;
        var_cjt_rdn0 = 0.0;
        var_cjt_rdn1 = 0.0;
        var_cjt_rdn2 = 0.0;
        var_cjt_rdn3 = 0.0;
        var_cjt_rdn4 = 0.0;
        var_cjt_rdn5 = 0.0;
        var_cjt_rdn6 = 0.0;
        var_cjt_rdb0 = 0.0;
        var_cjt_rdb1 = 0.0;
        var_cjt_rdb2 = 0.0;
        var_cjt_rdb3 = 0.0;
        var_cjt_rdb4 = 0.0;
        var_cjt_rdb5 = 0.0;
        var_cjt_rdb6 = 0.0;

        let assign250_e470: f64 = (var_fact2 * var_pbo);
        let assign250_e472: f64 = (assign250_e470 + var_pbfact);
        var_vje_t = assign250_e472;
        var_vje_t_dn0 = (((var_fact2_dn0 * var_pbo) + (var_fact2 * var_pbo_dn0)) + var_pbfact_dn0);
        var_vje_t_dn1 = (((var_fact2_dn1 * var_pbo) + (var_fact2 * var_pbo_dn1)) + var_pbfact_dn1);
        var_vje_t_dn2 = (((var_fact2_dn2 * var_pbo) + (var_fact2 * var_pbo_dn2)) + var_pbfact_dn2);
        var_vje_t_dn3 = (((var_fact2_dn3 * var_pbo) + (var_fact2 * var_pbo_dn3)) + var_pbfact_dn3);
        var_vje_t_dn4 = (((var_fact2_dn4 * var_pbo) + (var_fact2 * var_pbo_dn4)) + var_pbfact_dn4);
        var_vje_t_dn5 = (((var_fact2_dn5 * var_pbo) + (var_fact2 * var_pbo_dn5)) + var_pbfact_dn5);
        var_vje_t_dn6 = (((var_fact2_dn6 * var_pbo) + (var_fact2 * var_pbo_dn6)) + var_pbfact_dn6);
        var_vje_t_db0 = (((var_fact2_db0 * var_pbo) + (var_fact2 * var_pbo_db0)) + var_pbfact_db0);
        var_vje_t_db1 = (((var_fact2_db1 * var_pbo) + (var_fact2 * var_pbo_db1)) + var_pbfact_db1);
        var_vje_t_db2 = (((var_fact2_db2 * var_pbo) + (var_fact2 * var_pbo_db2)) + var_pbfact_db2);
        var_vje_t_db3 = (((var_fact2_db3 * var_pbo) + (var_fact2 * var_pbo_db3)) + var_pbfact_db3);
        var_vje_t_db4 = (((var_fact2_db4 * var_pbo) + (var_fact2 * var_pbo_db4)) + var_pbfact_db4);
        var_vje_t_db5 = (((var_fact2_db5 * var_pbo) + (var_fact2 * var_pbo_db5)) + var_pbfact_db5);
        var_vje_t_db6 = (((var_fact2_db6 * var_pbo) + (var_fact2 * var_pbo_db6)) + var_pbfact_db6);
        var_vje_t_rv = 0.0;
        var_vje_t_rdn0 = 0.0;
        var_vje_t_rdn1 = 0.0;
        var_vje_t_rdn2 = 0.0;
        var_vje_t_rdn3 = 0.0;
        var_vje_t_rdn4 = 0.0;
        var_vje_t_rdn5 = 0.0;
        var_vje_t_rdn6 = 0.0;
        var_vje_t_rdb0 = 0.0;
        var_vje_t_rdb1 = 0.0;
        var_vje_t_rdb2 = 0.0;
        var_vje_t_rdb3 = 0.0;
        var_vje_t_rdb4 = 0.0;
        var_vje_t_rdb5 = 0.0;
        var_vje_t_rdb6 = 0.0;

        let assign260_e475: f64 = (var_vje_t - var_pbo);
        let assign260_e477: f64 = (assign260_e475 / var_pbo);
        var_gmanew = assign260_e477;
        var_gmanew_dn0 = ((((var_vje_t_dn0 - var_pbo_dn0) * var_pbo) - (assign260_e475 * var_pbo_dn0)) / (var_pbo * var_pbo));
        var_gmanew_dn1 = ((((var_vje_t_dn1 - var_pbo_dn1) * var_pbo) - (assign260_e475 * var_pbo_dn1)) / (var_pbo * var_pbo));
        var_gmanew_dn2 = ((((var_vje_t_dn2 - var_pbo_dn2) * var_pbo) - (assign260_e475 * var_pbo_dn2)) / (var_pbo * var_pbo));
        var_gmanew_dn3 = ((((var_vje_t_dn3 - var_pbo_dn3) * var_pbo) - (assign260_e475 * var_pbo_dn3)) / (var_pbo * var_pbo));
        var_gmanew_dn4 = ((((var_vje_t_dn4 - var_pbo_dn4) * var_pbo) - (assign260_e475 * var_pbo_dn4)) / (var_pbo * var_pbo));
        var_gmanew_dn5 = ((((var_vje_t_dn5 - var_pbo_dn5) * var_pbo) - (assign260_e475 * var_pbo_dn5)) / (var_pbo * var_pbo));
        var_gmanew_dn6 = ((((var_vje_t_dn6 - var_pbo_dn6) * var_pbo) - (assign260_e475 * var_pbo_dn6)) / (var_pbo * var_pbo));
        var_gmanew_db0 = ((((var_vje_t_db0 - var_pbo_db0) * var_pbo) - (assign260_e475 * var_pbo_db0)) / (var_pbo * var_pbo));
        var_gmanew_db1 = ((((var_vje_t_db1 - var_pbo_db1) * var_pbo) - (assign260_e475 * var_pbo_db1)) / (var_pbo * var_pbo));
        var_gmanew_db2 = ((((var_vje_t_db2 - var_pbo_db2) * var_pbo) - (assign260_e475 * var_pbo_db2)) / (var_pbo * var_pbo));
        var_gmanew_db3 = ((((var_vje_t_db3 - var_pbo_db3) * var_pbo) - (assign260_e475 * var_pbo_db3)) / (var_pbo * var_pbo));
        var_gmanew_db4 = ((((var_vje_t_db4 - var_pbo_db4) * var_pbo) - (assign260_e475 * var_pbo_db4)) / (var_pbo * var_pbo));
        var_gmanew_db5 = ((((var_vje_t_db5 - var_pbo_db5) * var_pbo) - (assign260_e475 * var_pbo_db5)) / (var_pbo * var_pbo));
        var_gmanew_db6 = ((((var_vje_t_db6 - var_pbo_db6) * var_pbo) - (assign260_e475 * var_pbo_db6)) / (var_pbo * var_pbo));
        var_gmanew_rv = 0.0;
        var_gmanew_rdn0 = 0.0;
        var_gmanew_rdn1 = 0.0;
        var_gmanew_rdn2 = 0.0;
        var_gmanew_rdn3 = 0.0;
        var_gmanew_rdn4 = 0.0;
        var_gmanew_rdn5 = 0.0;
        var_gmanew_rdn6 = 0.0;
        var_gmanew_rdb0 = 0.0;
        var_gmanew_rdb1 = 0.0;
        var_gmanew_rdb2 = 0.0;
        var_gmanew_rdb3 = 0.0;
        var_gmanew_rdb4 = 0.0;
        var_gmanew_rdb5 = 0.0;
        var_gmanew_rdb6 = 0.0;

        let assign270_e484: f64 = (var_tdev - 300.15);
        let assign270_e485: f64 = (0.0004 * assign270_e484);
        let assign270_e487: f64 = (assign270_e485 - var_gmanew);
        let assign270_e488: f64 = (p.p18 * assign270_e487);
        let assign270_e489: f64 = (1.0 + assign270_e488);
        let assign270_e490: f64 = (var_cjt * assign270_e489);
        var_cje_t = assign270_e490;
        var_cje_t_dn0 = ((var_cjt_dn0 * assign270_e489) + (var_cjt * (p.p18 * ((0.0004 * var_tdev_dn0) - var_gmanew_dn0))));
        var_cje_t_dn1 = ((var_cjt_dn1 * assign270_e489) + (var_cjt * (p.p18 * ((0.0004 * var_tdev_dn1) - var_gmanew_dn1))));
        var_cje_t_dn2 = ((var_cjt_dn2 * assign270_e489) + (var_cjt * (p.p18 * ((0.0004 * var_tdev_dn2) - var_gmanew_dn2))));
        var_cje_t_dn3 = ((var_cjt_dn3 * assign270_e489) + (var_cjt * (p.p18 * ((0.0004 * var_tdev_dn3) - var_gmanew_dn3))));
        var_cje_t_dn4 = ((var_cjt_dn4 * assign270_e489) + (var_cjt * (p.p18 * ((0.0004 * var_tdev_dn4) - var_gmanew_dn4))));
        var_cje_t_dn5 = ((var_cjt_dn5 * assign270_e489) + (var_cjt * (p.p18 * ((0.0004 * var_tdev_dn5) - var_gmanew_dn5))));
        var_cje_t_dn6 = ((var_cjt_dn6 * assign270_e489) + (var_cjt * (p.p18 * ((0.0004 * var_tdev_dn6) - var_gmanew_dn6))));
        var_cje_t_db0 = ((var_cjt_db0 * assign270_e489) + (var_cjt * (p.p18 * ((0.0004 * var_tdev_db0) - var_gmanew_db0))));
        var_cje_t_db1 = ((var_cjt_db1 * assign270_e489) + (var_cjt * (p.p18 * ((0.0004 * var_tdev_db1) - var_gmanew_db1))));
        var_cje_t_db2 = ((var_cjt_db2 * assign270_e489) + (var_cjt * (p.p18 * ((0.0004 * var_tdev_db2) - var_gmanew_db2))));
        var_cje_t_db3 = ((var_cjt_db3 * assign270_e489) + (var_cjt * (p.p18 * ((0.0004 * var_tdev_db3) - var_gmanew_db3))));
        var_cje_t_db4 = ((var_cjt_db4 * assign270_e489) + (var_cjt * (p.p18 * ((0.0004 * var_tdev_db4) - var_gmanew_db4))));
        var_cje_t_db5 = ((var_cjt_db5 * assign270_e489) + (var_cjt * (p.p18 * ((0.0004 * var_tdev_db5) - var_gmanew_db5))));
        var_cje_t_db6 = ((var_cjt_db6 * assign270_e489) + (var_cjt * (p.p18 * ((0.0004 * var_tdev_db6) - var_gmanew_db6))));
        var_cje_t_rv = 0.0;
        var_cje_t_rdn0 = 0.0;
        var_cje_t_rdn1 = 0.0;
        var_cje_t_rdn2 = 0.0;
        var_cje_t_rdn3 = 0.0;
        var_cje_t_rdn4 = 0.0;
        var_cje_t_rdn5 = 0.0;
        var_cje_t_rdn6 = 0.0;
        var_cje_t_rdb0 = 0.0;
        var_cje_t_rdb1 = 0.0;
        var_cje_t_rdb2 = 0.0;
        var_cje_t_rdb3 = 0.0;
        var_cje_t_rdb4 = 0.0;
        var_cje_t_rdb5 = 0.0;
        var_cje_t_rdb6 = 0.0;

        var_ttype = p.p29;
        var_ttype_dn0 = 0.0;
        var_ttype_dn1 = 0.0;
        var_ttype_dn2 = 0.0;
        var_ttype_dn3 = 0.0;
        var_ttype_dn4 = 0.0;
        var_ttype_dn5 = 0.0;
        var_ttype_dn6 = 0.0;
        var_ttype_db0 = 0.0;
        var_ttype_db1 = 0.0;
        var_ttype_db2 = 0.0;
        var_ttype_db3 = 0.0;
        var_ttype_db4 = 0.0;
        var_ttype_db5 = 0.0;
        var_ttype_db6 = 0.0;
        var_ttype_rv = 0.0;
        var_ttype_rdn0 = 0.0;
        var_ttype_rdn1 = 0.0;
        var_ttype_rdn2 = 0.0;
        var_ttype_rdn3 = 0.0;
        var_ttype_rdn4 = 0.0;
        var_ttype_rdn5 = 0.0;
        var_ttype_rdn6 = 0.0;
        var_ttype_rdb0 = 0.0;
        var_ttype_rdb1 = 0.0;
        var_ttype_rdb2 = 0.0;
        var_ttype_rdb3 = 0.0;
        var_ttype_rdb4 = 0.0;
        var_ttype_rdb5 = 0.0;
        var_ttype_rdb6 = 0.0;

        let assign290_e494: f64 = (var_ttype * (nv3 - nv4));
        var_vbiei = assign290_e494;
        var_vbiei_dn0 = (var_ttype_dn0 * (nv3 - nv4));
        var_vbiei_dn1 = (var_ttype_dn1 * (nv3 - nv4));
        var_vbiei_dn2 = (var_ttype_dn2 * (nv3 - nv4));
        var_vbiei_dn3 = ((var_ttype_dn3 * (nv3 - nv4)) + var_ttype);
        var_vbiei_dn4 = ((var_ttype_dn4 * (nv3 - nv4)) + (-var_ttype));
        var_vbiei_dn5 = (var_ttype_dn5 * (nv3 - nv4));
        var_vbiei_dn6 = (var_ttype_dn6 * (nv3 - nv4));
        var_vbiei_db0 = (var_ttype_db0 * (nv3 - nv4));
        var_vbiei_db1 = (var_ttype_db1 * (nv3 - nv4));
        var_vbiei_db2 = (var_ttype_db2 * (nv3 - nv4));
        var_vbiei_db3 = (var_ttype_db3 * (nv3 - nv4));
        var_vbiei_db4 = (var_ttype_db4 * (nv3 - nv4));
        var_vbiei_db5 = (var_ttype_db5 * (nv3 - nv4));
        var_vbiei_db6 = (var_ttype_db6 * (nv3 - nv4));
        var_vbiei_rv = 0.0;
        var_vbiei_rdn0 = 0.0;
        var_vbiei_rdn1 = 0.0;
        var_vbiei_rdn2 = 0.0;
        var_vbiei_rdn3 = 0.0;
        var_vbiei_rdn4 = 0.0;
        var_vbiei_rdn5 = 0.0;
        var_vbiei_rdn6 = 0.0;
        var_vbiei_rdb0 = 0.0;
        var_vbiei_rdb1 = 0.0;
        var_vbiei_rdb2 = 0.0;
        var_vbiei_rdb3 = 0.0;
        var_vbiei_rdb4 = 0.0;
        var_vbiei_rdb5 = 0.0;
        var_vbiei_rdb6 = 0.0;

        let assign320_e503: f64 = if var_is_t > 0.0 { 1.0 } else { 0.0 };
        var_guard3 = assign320_e503;
        var_guard3_dn0 = 0.0;
        var_guard3_dn1 = 0.0;
        var_guard3_dn2 = 0.0;
        var_guard3_dn3 = 0.0;
        var_guard3_dn4 = 0.0;
        var_guard3_dn5 = 0.0;
        var_guard3_dn6 = 0.0;
        var_guard3_db0 = 0.0;
        var_guard3_db1 = 0.0;
        var_guard3_db2 = 0.0;
        var_guard3_db3 = 0.0;
        var_guard3_db4 = 0.0;
        var_guard3_db5 = 0.0;
        var_guard3_db6 = 0.0;
        var_guard3_rv = 0.0;
        var_guard3_rdn0 = 0.0;
        var_guard3_rdn1 = 0.0;
        var_guard3_rdn2 = 0.0;
        var_guard3_rdn3 = 0.0;
        var_guard3_rdn4 = 0.0;
        var_guard3_rdn5 = 0.0;
        var_guard3_rdn6 = 0.0;
        var_guard3_rdb0 = 0.0;
        var_guard3_rdb1 = 0.0;
        var_guard3_rdb2 = 0.0;
        var_guard3_rdb3 = 0.0;
        var_guard3_rdb4 = 0.0;
        var_guard3_rdb5 = 0.0;
        var_guard3_rdb6 = 0.0;

        *var_arg0_slot = var_arg0;
        *var_arg0_db0_slot = var_arg0_db0;
        *var_arg0_db1_slot = var_arg0_db1;
        *var_arg0_db2_slot = var_arg0_db2;
        *var_arg0_db3_slot = var_arg0_db3;
        *var_arg0_db4_slot = var_arg0_db4;
        *var_arg0_db5_slot = var_arg0_db5;
        *var_arg0_db6_slot = var_arg0_db6;
        *var_arg0_dn0_slot = var_arg0_dn0;
        *var_arg0_dn1_slot = var_arg0_dn1;
        *var_arg0_dn2_slot = var_arg0_dn2;
        *var_arg0_dn3_slot = var_arg0_dn3;
        *var_arg0_dn4_slot = var_arg0_dn4;
        *var_arg0_dn5_slot = var_arg0_dn5;
        *var_arg0_dn6_slot = var_arg0_dn6;
        *var_arg0_rdb0_slot = var_arg0_rdb0;
        *var_arg0_rdb1_slot = var_arg0_rdb1;
        *var_arg0_rdb2_slot = var_arg0_rdb2;
        *var_arg0_rdb3_slot = var_arg0_rdb3;
        *var_arg0_rdb4_slot = var_arg0_rdb4;
        *var_arg0_rdb5_slot = var_arg0_rdb5;
        *var_arg0_rdb6_slot = var_arg0_rdb6;
        *var_arg0_rdn0_slot = var_arg0_rdn0;
        *var_arg0_rdn1_slot = var_arg0_rdn1;
        *var_arg0_rdn2_slot = var_arg0_rdn2;
        *var_arg0_rdn3_slot = var_arg0_rdn3;
        *var_arg0_rdn4_slot = var_arg0_rdn4;
        *var_arg0_rdn5_slot = var_arg0_rdn5;
        *var_arg0_rdn6_slot = var_arg0_rdn6;
        *var_arg0_rv_slot = var_arg0_rv;
        *var_cje_i_slot = var_cje_i;
        *var_cje_i_db0_slot = var_cje_i_db0;
        *var_cje_i_db1_slot = var_cje_i_db1;
        *var_cje_i_db2_slot = var_cje_i_db2;
        *var_cje_i_db3_slot = var_cje_i_db3;
        *var_cje_i_db4_slot = var_cje_i_db4;
        *var_cje_i_db5_slot = var_cje_i_db5;
        *var_cje_i_db6_slot = var_cje_i_db6;
        *var_cje_i_dn0_slot = var_cje_i_dn0;
        *var_cje_i_dn1_slot = var_cje_i_dn1;
        *var_cje_i_dn2_slot = var_cje_i_dn2;
        *var_cje_i_dn3_slot = var_cje_i_dn3;
        *var_cje_i_dn4_slot = var_cje_i_dn4;
        *var_cje_i_dn5_slot = var_cje_i_dn5;
        *var_cje_i_dn6_slot = var_cje_i_dn6;
        *var_cje_i_rdb0_slot = var_cje_i_rdb0;
        *var_cje_i_rdb1_slot = var_cje_i_rdb1;
        *var_cje_i_rdb2_slot = var_cje_i_rdb2;
        *var_cje_i_rdb3_slot = var_cje_i_rdb3;
        *var_cje_i_rdb4_slot = var_cje_i_rdb4;
        *var_cje_i_rdb5_slot = var_cje_i_rdb5;
        *var_cje_i_rdb6_slot = var_cje_i_rdb6;
        *var_cje_i_rdn0_slot = var_cje_i_rdn0;
        *var_cje_i_rdn1_slot = var_cje_i_rdn1;
        *var_cje_i_rdn2_slot = var_cje_i_rdn2;
        *var_cje_i_rdn3_slot = var_cje_i_rdn3;
        *var_cje_i_rdn4_slot = var_cje_i_rdn4;
        *var_cje_i_rdn5_slot = var_cje_i_rdn5;
        *var_cje_i_rdn6_slot = var_cje_i_rdn6;
        *var_cje_i_rv_slot = var_cje_i_rv;
        *var_cje_t_slot = var_cje_t;
        *var_cje_t_db0_slot = var_cje_t_db0;
        *var_cje_t_db1_slot = var_cje_t_db1;
        *var_cje_t_db2_slot = var_cje_t_db2;
        *var_cje_t_db3_slot = var_cje_t_db3;
        *var_cje_t_db4_slot = var_cje_t_db4;
        *var_cje_t_db5_slot = var_cje_t_db5;
        *var_cje_t_db6_slot = var_cje_t_db6;
        *var_cje_t_dn0_slot = var_cje_t_dn0;
        *var_cje_t_dn1_slot = var_cje_t_dn1;
        *var_cje_t_dn2_slot = var_cje_t_dn2;
        *var_cje_t_dn3_slot = var_cje_t_dn3;
        *var_cje_t_dn4_slot = var_cje_t_dn4;
        *var_cje_t_dn5_slot = var_cje_t_dn5;
        *var_cje_t_dn6_slot = var_cje_t_dn6;
        *var_cje_t_rdb0_slot = var_cje_t_rdb0;
        *var_cje_t_rdb1_slot = var_cje_t_rdb1;
        *var_cje_t_rdb2_slot = var_cje_t_rdb2;
        *var_cje_t_rdb3_slot = var_cje_t_rdb3;
        *var_cje_t_rdb4_slot = var_cje_t_rdb4;
        *var_cje_t_rdb5_slot = var_cje_t_rdb5;
        *var_cje_t_rdb6_slot = var_cje_t_rdb6;
        *var_cje_t_rdn0_slot = var_cje_t_rdn0;
        *var_cje_t_rdn1_slot = var_cje_t_rdn1;
        *var_cje_t_rdn2_slot = var_cje_t_rdn2;
        *var_cje_t_rdn3_slot = var_cje_t_rdn3;
        *var_cje_t_rdn4_slot = var_cje_t_rdn4;
        *var_cje_t_rdn5_slot = var_cje_t_rdn5;
        *var_cje_t_rdn6_slot = var_cje_t_rdn6;
        *var_cje_t_rv_slot = var_cje_t_rv;
        *var_cjt_slot = var_cjt;
        *var_cjt_db0_slot = var_cjt_db0;
        *var_cjt_db1_slot = var_cjt_db1;
        *var_cjt_db2_slot = var_cjt_db2;
        *var_cjt_db3_slot = var_cjt_db3;
        *var_cjt_db4_slot = var_cjt_db4;
        *var_cjt_db5_slot = var_cjt_db5;
        *var_cjt_db6_slot = var_cjt_db6;
        *var_cjt_dn0_slot = var_cjt_dn0;
        *var_cjt_dn1_slot = var_cjt_dn1;
        *var_cjt_dn2_slot = var_cjt_dn2;
        *var_cjt_dn3_slot = var_cjt_dn3;
        *var_cjt_dn4_slot = var_cjt_dn4;
        *var_cjt_dn5_slot = var_cjt_dn5;
        *var_cjt_dn6_slot = var_cjt_dn6;
        *var_cjt_rdb0_slot = var_cjt_rdb0;
        *var_cjt_rdb1_slot = var_cjt_rdb1;
        *var_cjt_rdb2_slot = var_cjt_rdb2;
        *var_cjt_rdb3_slot = var_cjt_rdb3;
        *var_cjt_rdb4_slot = var_cjt_rdb4;
        *var_cjt_rdb5_slot = var_cjt_rdb5;
        *var_cjt_rdb6_slot = var_cjt_rdb6;
        *var_cjt_rdn0_slot = var_cjt_rdn0;
        *var_cjt_rdn1_slot = var_cjt_rdn1;
        *var_cjt_rdn2_slot = var_cjt_rdn2;
        *var_cjt_rdn3_slot = var_cjt_rdn3;
        *var_cjt_rdn4_slot = var_cjt_rdn4;
        *var_cjt_rdn5_slot = var_cjt_rdn5;
        *var_cjt_rdn6_slot = var_cjt_rdn6;
        *var_cjt_rv_slot = var_cjt_rv;
        *var_egfet_slot = var_egfet;
        *var_egfet_db0_slot = var_egfet_db0;
        *var_egfet_db1_slot = var_egfet_db1;
        *var_egfet_db2_slot = var_egfet_db2;
        *var_egfet_db3_slot = var_egfet_db3;
        *var_egfet_db4_slot = var_egfet_db4;
        *var_egfet_db5_slot = var_egfet_db5;
        *var_egfet_db6_slot = var_egfet_db6;
        *var_egfet_dn0_slot = var_egfet_dn0;
        *var_egfet_dn1_slot = var_egfet_dn1;
        *var_egfet_dn2_slot = var_egfet_dn2;
        *var_egfet_dn3_slot = var_egfet_dn3;
        *var_egfet_dn4_slot = var_egfet_dn4;
        *var_egfet_dn5_slot = var_egfet_dn5;
        *var_egfet_dn6_slot = var_egfet_dn6;
        *var_egfet_rdb0_slot = var_egfet_rdb0;
        *var_egfet_rdb1_slot = var_egfet_rdb1;
        *var_egfet_rdb2_slot = var_egfet_rdb2;
        *var_egfet_rdb3_slot = var_egfet_rdb3;
        *var_egfet_rdb4_slot = var_egfet_rdb4;
        *var_egfet_rdb5_slot = var_egfet_rdb5;
        *var_egfet_rdb6_slot = var_egfet_rdb6;
        *var_egfet_rdn0_slot = var_egfet_rdn0;
        *var_egfet_rdn1_slot = var_egfet_rdn1;
        *var_egfet_rdn2_slot = var_egfet_rdn2;
        *var_egfet_rdn3_slot = var_egfet_rdn3;
        *var_egfet_rdn4_slot = var_egfet_rdn4;
        *var_egfet_rdn5_slot = var_egfet_rdn5;
        *var_egfet_rdn6_slot = var_egfet_rdn6;
        *var_egfet_rv_slot = var_egfet_rv;
        *var_fact1_slot = var_fact1;
        *var_fact1_db0_slot = var_fact1_db0;
        *var_fact1_db1_slot = var_fact1_db1;
        *var_fact1_db2_slot = var_fact1_db2;
        *var_fact1_db3_slot = var_fact1_db3;
        *var_fact1_db4_slot = var_fact1_db4;
        *var_fact1_db5_slot = var_fact1_db5;
        *var_fact1_db6_slot = var_fact1_db6;
        *var_fact1_dn0_slot = var_fact1_dn0;
        *var_fact1_dn1_slot = var_fact1_dn1;
        *var_fact1_dn2_slot = var_fact1_dn2;
        *var_fact1_dn3_slot = var_fact1_dn3;
        *var_fact1_dn4_slot = var_fact1_dn4;
        *var_fact1_dn5_slot = var_fact1_dn5;
        *var_fact1_dn6_slot = var_fact1_dn6;
        *var_fact1_rdb0_slot = var_fact1_rdb0;
        *var_fact1_rdb1_slot = var_fact1_rdb1;
        *var_fact1_rdb2_slot = var_fact1_rdb2;
        *var_fact1_rdb3_slot = var_fact1_rdb3;
        *var_fact1_rdb4_slot = var_fact1_rdb4;
        *var_fact1_rdb5_slot = var_fact1_rdb5;
        *var_fact1_rdb6_slot = var_fact1_rdb6;
        *var_fact1_rdn0_slot = var_fact1_rdn0;
        *var_fact1_rdn1_slot = var_fact1_rdn1;
        *var_fact1_rdn2_slot = var_fact1_rdn2;
        *var_fact1_rdn3_slot = var_fact1_rdn3;
        *var_fact1_rdn4_slot = var_fact1_rdn4;
        *var_fact1_rdn5_slot = var_fact1_rdn5;
        *var_fact1_rdn6_slot = var_fact1_rdn6;
        *var_fact1_rv_slot = var_fact1_rv;
        *var_fact2_slot = var_fact2;
        *var_fact2_db0_slot = var_fact2_db0;
        *var_fact2_db1_slot = var_fact2_db1;
        *var_fact2_db2_slot = var_fact2_db2;
        *var_fact2_db3_slot = var_fact2_db3;
        *var_fact2_db4_slot = var_fact2_db4;
        *var_fact2_db5_slot = var_fact2_db5;
        *var_fact2_db6_slot = var_fact2_db6;
        *var_fact2_dn0_slot = var_fact2_dn0;
        *var_fact2_dn1_slot = var_fact2_dn1;
        *var_fact2_dn2_slot = var_fact2_dn2;
        *var_fact2_dn3_slot = var_fact2_dn3;
        *var_fact2_dn4_slot = var_fact2_dn4;
        *var_fact2_dn5_slot = var_fact2_dn5;
        *var_fact2_dn6_slot = var_fact2_dn6;
        *var_fact2_rdb0_slot = var_fact2_rdb0;
        *var_fact2_rdb1_slot = var_fact2_rdb1;
        *var_fact2_rdb2_slot = var_fact2_rdb2;
        *var_fact2_rdb3_slot = var_fact2_rdb3;
        *var_fact2_rdb4_slot = var_fact2_rdb4;
        *var_fact2_rdb5_slot = var_fact2_rdb5;
        *var_fact2_rdb6_slot = var_fact2_rdb6;
        *var_fact2_rdn0_slot = var_fact2_rdn0;
        *var_fact2_rdn1_slot = var_fact2_rdn1;
        *var_fact2_rdn2_slot = var_fact2_rdn2;
        *var_fact2_rdn3_slot = var_fact2_rdn3;
        *var_fact2_rdn4_slot = var_fact2_rdn4;
        *var_fact2_rdn5_slot = var_fact2_rdn5;
        *var_fact2_rdn6_slot = var_fact2_rdn6;
        *var_fact2_rv_slot = var_fact2_rv;
        *var_gmanew_slot = var_gmanew;
        *var_gmanew_db0_slot = var_gmanew_db0;
        *var_gmanew_db1_slot = var_gmanew_db1;
        *var_gmanew_db2_slot = var_gmanew_db2;
        *var_gmanew_db3_slot = var_gmanew_db3;
        *var_gmanew_db4_slot = var_gmanew_db4;
        *var_gmanew_db5_slot = var_gmanew_db5;
        *var_gmanew_db6_slot = var_gmanew_db6;
        *var_gmanew_dn0_slot = var_gmanew_dn0;
        *var_gmanew_dn1_slot = var_gmanew_dn1;
        *var_gmanew_dn2_slot = var_gmanew_dn2;
        *var_gmanew_dn3_slot = var_gmanew_dn3;
        *var_gmanew_dn4_slot = var_gmanew_dn4;
        *var_gmanew_dn5_slot = var_gmanew_dn5;
        *var_gmanew_dn6_slot = var_gmanew_dn6;
        *var_gmanew_rdb0_slot = var_gmanew_rdb0;
        *var_gmanew_rdb1_slot = var_gmanew_rdb1;
        *var_gmanew_rdb2_slot = var_gmanew_rdb2;
        *var_gmanew_rdb3_slot = var_gmanew_rdb3;
        *var_gmanew_rdb4_slot = var_gmanew_rdb4;
        *var_gmanew_rdb5_slot = var_gmanew_rdb5;
        *var_gmanew_rdb6_slot = var_gmanew_rdb6;
        *var_gmanew_rdn0_slot = var_gmanew_rdn0;
        *var_gmanew_rdn1_slot = var_gmanew_rdn1;
        *var_gmanew_rdn2_slot = var_gmanew_rdn2;
        *var_gmanew_rdn3_slot = var_gmanew_rdn3;
        *var_gmanew_rdn4_slot = var_gmanew_rdn4;
        *var_gmanew_rdn5_slot = var_gmanew_rdn5;
        *var_gmanew_rdn6_slot = var_gmanew_rdn6;
        *var_gmanew_rv_slot = var_gmanew_rv;
        *var_gmaold_slot = var_gmaold;
        *var_gmaold_db0_slot = var_gmaold_db0;
        *var_gmaold_db1_slot = var_gmaold_db1;
        *var_gmaold_db2_slot = var_gmaold_db2;
        *var_gmaold_db3_slot = var_gmaold_db3;
        *var_gmaold_db4_slot = var_gmaold_db4;
        *var_gmaold_db5_slot = var_gmaold_db5;
        *var_gmaold_db6_slot = var_gmaold_db6;
        *var_gmaold_dn0_slot = var_gmaold_dn0;
        *var_gmaold_dn1_slot = var_gmaold_dn1;
        *var_gmaold_dn2_slot = var_gmaold_dn2;
        *var_gmaold_dn3_slot = var_gmaold_dn3;
        *var_gmaold_dn4_slot = var_gmaold_dn4;
        *var_gmaold_dn5_slot = var_gmaold_dn5;
        *var_gmaold_dn6_slot = var_gmaold_dn6;
        *var_gmaold_rdb0_slot = var_gmaold_rdb0;
        *var_gmaold_rdb1_slot = var_gmaold_rdb1;
        *var_gmaold_rdb2_slot = var_gmaold_rdb2;
        *var_gmaold_rdb3_slot = var_gmaold_rdb3;
        *var_gmaold_rdb4_slot = var_gmaold_rdb4;
        *var_gmaold_rdb5_slot = var_gmaold_rdb5;
        *var_gmaold_rdb6_slot = var_gmaold_rdb6;
        *var_gmaold_rdn0_slot = var_gmaold_rdn0;
        *var_gmaold_rdn1_slot = var_gmaold_rdn1;
        *var_gmaold_rdn2_slot = var_gmaold_rdn2;
        *var_gmaold_rdn3_slot = var_gmaold_rdn3;
        *var_gmaold_rdn4_slot = var_gmaold_rdn4;
        *var_gmaold_rdn5_slot = var_gmaold_rdn5;
        *var_gmaold_rdn6_slot = var_gmaold_rdn6;
        *var_gmaold_rv_slot = var_gmaold_rv;
        *var_guard3_slot = var_guard3;
        *var_guard3_db0_slot = var_guard3_db0;
        *var_guard3_db1_slot = var_guard3_db1;
        *var_guard3_db2_slot = var_guard3_db2;
        *var_guard3_db3_slot = var_guard3_db3;
        *var_guard3_db4_slot = var_guard3_db4;
        *var_guard3_db5_slot = var_guard3_db5;
        *var_guard3_db6_slot = var_guard3_db6;
        *var_guard3_dn0_slot = var_guard3_dn0;
        *var_guard3_dn1_slot = var_guard3_dn1;
        *var_guard3_dn2_slot = var_guard3_dn2;
        *var_guard3_dn3_slot = var_guard3_dn3;
        *var_guard3_dn4_slot = var_guard3_dn4;
        *var_guard3_dn5_slot = var_guard3_dn5;
        *var_guard3_dn6_slot = var_guard3_dn6;
        *var_guard3_rdb0_slot = var_guard3_rdb0;
        *var_guard3_rdb1_slot = var_guard3_rdb1;
        *var_guard3_rdb2_slot = var_guard3_rdb2;
        *var_guard3_rdb3_slot = var_guard3_rdb3;
        *var_guard3_rdb4_slot = var_guard3_rdb4;
        *var_guard3_rdb5_slot = var_guard3_rdb5;
        *var_guard3_rdb6_slot = var_guard3_rdb6;
        *var_guard3_rdn0_slot = var_guard3_rdn0;
        *var_guard3_rdn1_slot = var_guard3_rdn1;
        *var_guard3_rdn2_slot = var_guard3_rdn2;
        *var_guard3_rdn3_slot = var_guard3_rdn3;
        *var_guard3_rdn4_slot = var_guard3_rdn4;
        *var_guard3_rdn5_slot = var_guard3_rdn5;
        *var_guard3_rdn6_slot = var_guard3_rdn6;
        *var_guard3_rv_slot = var_guard3_rv;
        *var_pbfact_slot = var_pbfact;
        *var_pbfact_db0_slot = var_pbfact_db0;
        *var_pbfact_db1_slot = var_pbfact_db1;
        *var_pbfact_db2_slot = var_pbfact_db2;
        *var_pbfact_db3_slot = var_pbfact_db3;
        *var_pbfact_db4_slot = var_pbfact_db4;
        *var_pbfact_db5_slot = var_pbfact_db5;
        *var_pbfact_db6_slot = var_pbfact_db6;
        *var_pbfact_dn0_slot = var_pbfact_dn0;
        *var_pbfact_dn1_slot = var_pbfact_dn1;
        *var_pbfact_dn2_slot = var_pbfact_dn2;
        *var_pbfact_dn3_slot = var_pbfact_dn3;
        *var_pbfact_dn4_slot = var_pbfact_dn4;
        *var_pbfact_dn5_slot = var_pbfact_dn5;
        *var_pbfact_dn6_slot = var_pbfact_dn6;
        *var_pbfact_rdb0_slot = var_pbfact_rdb0;
        *var_pbfact_rdb1_slot = var_pbfact_rdb1;
        *var_pbfact_rdb2_slot = var_pbfact_rdb2;
        *var_pbfact_rdb3_slot = var_pbfact_rdb3;
        *var_pbfact_rdb4_slot = var_pbfact_rdb4;
        *var_pbfact_rdb5_slot = var_pbfact_rdb5;
        *var_pbfact_rdb6_slot = var_pbfact_rdb6;
        *var_pbfact_rdn0_slot = var_pbfact_rdn0;
        *var_pbfact_rdn1_slot = var_pbfact_rdn1;
        *var_pbfact_rdn2_slot = var_pbfact_rdn2;
        *var_pbfact_rdn3_slot = var_pbfact_rdn3;
        *var_pbfact_rdn4_slot = var_pbfact_rdn4;
        *var_pbfact_rdn5_slot = var_pbfact_rdn5;
        *var_pbfact_rdn6_slot = var_pbfact_rdn6;
        *var_pbfact_rv_slot = var_pbfact_rv;
        *var_pbo_slot = var_pbo;
        *var_pbo_db0_slot = var_pbo_db0;
        *var_pbo_db1_slot = var_pbo_db1;
        *var_pbo_db2_slot = var_pbo_db2;
        *var_pbo_db3_slot = var_pbo_db3;
        *var_pbo_db4_slot = var_pbo_db4;
        *var_pbo_db5_slot = var_pbo_db5;
        *var_pbo_db6_slot = var_pbo_db6;
        *var_pbo_dn0_slot = var_pbo_dn0;
        *var_pbo_dn1_slot = var_pbo_dn1;
        *var_pbo_dn2_slot = var_pbo_dn2;
        *var_pbo_dn3_slot = var_pbo_dn3;
        *var_pbo_dn4_slot = var_pbo_dn4;
        *var_pbo_dn5_slot = var_pbo_dn5;
        *var_pbo_dn6_slot = var_pbo_dn6;
        *var_pbo_rdb0_slot = var_pbo_rdb0;
        *var_pbo_rdb1_slot = var_pbo_rdb1;
        *var_pbo_rdb2_slot = var_pbo_rdb2;
        *var_pbo_rdb3_slot = var_pbo_rdb3;
        *var_pbo_rdb4_slot = var_pbo_rdb4;
        *var_pbo_rdb5_slot = var_pbo_rdb5;
        *var_pbo_rdb6_slot = var_pbo_rdb6;
        *var_pbo_rdn0_slot = var_pbo_rdn0;
        *var_pbo_rdn1_slot = var_pbo_rdn1;
        *var_pbo_rdn2_slot = var_pbo_rdn2;
        *var_pbo_rdn3_slot = var_pbo_rdn3;
        *var_pbo_rdn4_slot = var_pbo_rdn4;
        *var_pbo_rdn5_slot = var_pbo_rdn5;
        *var_pbo_rdn6_slot = var_pbo_rdn6;
        *var_pbo_rv_slot = var_pbo_rv;
        *var_ttype_slot = var_ttype;
        *var_ttype_db0_slot = var_ttype_db0;
        *var_ttype_db1_slot = var_ttype_db1;
        *var_ttype_db2_slot = var_ttype_db2;
        *var_ttype_db3_slot = var_ttype_db3;
        *var_ttype_db4_slot = var_ttype_db4;
        *var_ttype_db5_slot = var_ttype_db5;
        *var_ttype_db6_slot = var_ttype_db6;
        *var_ttype_dn0_slot = var_ttype_dn0;
        *var_ttype_dn1_slot = var_ttype_dn1;
        *var_ttype_dn2_slot = var_ttype_dn2;
        *var_ttype_dn3_slot = var_ttype_dn3;
        *var_ttype_dn4_slot = var_ttype_dn4;
        *var_ttype_dn5_slot = var_ttype_dn5;
        *var_ttype_dn6_slot = var_ttype_dn6;
        *var_ttype_rdb0_slot = var_ttype_rdb0;
        *var_ttype_rdb1_slot = var_ttype_rdb1;
        *var_ttype_rdb2_slot = var_ttype_rdb2;
        *var_ttype_rdb3_slot = var_ttype_rdb3;
        *var_ttype_rdb4_slot = var_ttype_rdb4;
        *var_ttype_rdb5_slot = var_ttype_rdb5;
        *var_ttype_rdb6_slot = var_ttype_rdb6;
        *var_ttype_rdn0_slot = var_ttype_rdn0;
        *var_ttype_rdn1_slot = var_ttype_rdn1;
        *var_ttype_rdn2_slot = var_ttype_rdn2;
        *var_ttype_rdn3_slot = var_ttype_rdn3;
        *var_ttype_rdn4_slot = var_ttype_rdn4;
        *var_ttype_rdn5_slot = var_ttype_rdn5;
        *var_ttype_rdn6_slot = var_ttype_rdn6;
        *var_ttype_rv_slot = var_ttype_rv;
        *var_vbiei_slot = var_vbiei;
        *var_vbiei_db0_slot = var_vbiei_db0;
        *var_vbiei_db1_slot = var_vbiei_db1;
        *var_vbiei_db2_slot = var_vbiei_db2;
        *var_vbiei_db3_slot = var_vbiei_db3;
        *var_vbiei_db4_slot = var_vbiei_db4;
        *var_vbiei_db5_slot = var_vbiei_db5;
        *var_vbiei_db6_slot = var_vbiei_db6;
        *var_vbiei_dn0_slot = var_vbiei_dn0;
        *var_vbiei_dn1_slot = var_vbiei_dn1;
        *var_vbiei_dn2_slot = var_vbiei_dn2;
        *var_vbiei_dn3_slot = var_vbiei_dn3;
        *var_vbiei_dn4_slot = var_vbiei_dn4;
        *var_vbiei_dn5_slot = var_vbiei_dn5;
        *var_vbiei_dn6_slot = var_vbiei_dn6;
        *var_vbiei_rdb0_slot = var_vbiei_rdb0;
        *var_vbiei_rdb1_slot = var_vbiei_rdb1;
        *var_vbiei_rdb2_slot = var_vbiei_rdb2;
        *var_vbiei_rdb3_slot = var_vbiei_rdb3;
        *var_vbiei_rdb4_slot = var_vbiei_rdb4;
        *var_vbiei_rdb5_slot = var_vbiei_rdb5;
        *var_vbiei_rdb6_slot = var_vbiei_rdb6;
        *var_vbiei_rdn0_slot = var_vbiei_rdn0;
        *var_vbiei_rdn1_slot = var_vbiei_rdn1;
        *var_vbiei_rdn2_slot = var_vbiei_rdn2;
        *var_vbiei_rdn3_slot = var_vbiei_rdn3;
        *var_vbiei_rdn4_slot = var_vbiei_rdn4;
        *var_vbiei_rdn5_slot = var_vbiei_rdn5;
        *var_vbiei_rdn6_slot = var_vbiei_rdn6;
        *var_vbiei_rv_slot = var_vbiei_rv;
        *var_vje_t_slot = var_vje_t;
        *var_vje_t_db0_slot = var_vje_t_db0;
        *var_vje_t_db1_slot = var_vje_t_db1;
        *var_vje_t_db2_slot = var_vje_t_db2;
        *var_vje_t_db3_slot = var_vje_t_db3;
        *var_vje_t_db4_slot = var_vje_t_db4;
        *var_vje_t_db5_slot = var_vje_t_db5;
        *var_vje_t_db6_slot = var_vje_t_db6;
        *var_vje_t_dn0_slot = var_vje_t_dn0;
        *var_vje_t_dn1_slot = var_vje_t_dn1;
        *var_vje_t_dn2_slot = var_vje_t_dn2;
        *var_vje_t_dn3_slot = var_vje_t_dn3;
        *var_vje_t_dn4_slot = var_vje_t_dn4;
        *var_vje_t_dn5_slot = var_vje_t_dn5;
        *var_vje_t_dn6_slot = var_vje_t_dn6;
        *var_vje_t_rdb0_slot = var_vje_t_rdb0;
        *var_vje_t_rdb1_slot = var_vje_t_rdb1;
        *var_vje_t_rdb2_slot = var_vje_t_rdb2;
        *var_vje_t_rdb3_slot = var_vje_t_rdb3;
        *var_vje_t_rdb4_slot = var_vje_t_rdb4;
        *var_vje_t_rdb5_slot = var_vje_t_rdb5;
        *var_vje_t_rdb6_slot = var_vje_t_rdb6;
        *var_vje_t_rdn0_slot = var_vje_t_rdn0;
        *var_vje_t_rdn1_slot = var_vje_t_rdn1;
        *var_vje_t_rdn2_slot = var_vje_t_rdn2;
        *var_vje_t_rdn3_slot = var_vje_t_rdn3;
        *var_vje_t_rdn4_slot = var_vje_t_rdn4;
        *var_vje_t_rdn5_slot = var_vje_t_rdn5;
        *var_vje_t_rdn6_slot = var_vje_t_rdn6;
        *var_vje_t_rv_slot = var_vje_t_rv;
    }

    pub(super) fn stamp_reactive_block_2(
        p: &Parameters,
        var_bvr_t: f64,
        var_bvr_t_db0: f64,
        var_bvr_t_db1: f64,
        var_bvr_t_db2: f64,
        var_bvr_t_db3: f64,
        var_bvr_t_db4: f64,
        var_bvr_t_db5: f64,
        var_bvr_t_db6: f64,
        var_bvr_t_dn0: f64,
        var_bvr_t_dn1: f64,
        var_bvr_t_dn2: f64,
        var_bvr_t_dn3: f64,
        var_bvr_t_dn4: f64,
        var_bvr_t_dn5: f64,
        var_bvr_t_dn6: f64,
        var_guard3: f64,
        var_ijbv_t: f64,
        var_ijbv_t_db0: f64,
        var_ijbv_t_db1: f64,
        var_ijbv_t_db2: f64,
        var_ijbv_t_db3: f64,
        var_ijbv_t_db4: f64,
        var_ijbv_t_db5: f64,
        var_ijbv_t_db6: f64,
        var_ijbv_t_dn0: f64,
        var_ijbv_t_dn1: f64,
        var_ijbv_t_dn2: f64,
        var_ijbv_t_dn3: f64,
        var_ijbv_t_dn4: f64,
        var_ijbv_t_dn5: f64,
        var_ijbv_t_dn6: f64,
        var_is_t: f64,
        var_is_t_db0: f64,
        var_is_t_db1: f64,
        var_is_t_db2: f64,
        var_is_t_db3: f64,
        var_is_t_db4: f64,
        var_is_t_db5: f64,
        var_is_t_db6: f64,
        var_is_t_dn0: f64,
        var_is_t_dn1: f64,
        var_is_t_dn2: f64,
        var_is_t_dn3: f64,
        var_is_t_dn4: f64,
        var_is_t_dn5: f64,
        var_is_t_dn6: f64,
        var_theexp_t: f64,
        var_theexp_t_db0: f64,
        var_theexp_t_db1: f64,
        var_theexp_t_db2: f64,
        var_theexp_t_db3: f64,
        var_theexp_t_db4: f64,
        var_theexp_t_db5: f64,
        var_theexp_t_db6: f64,
        var_theexp_t_dn0: f64,
        var_theexp_t_dn1: f64,
        var_theexp_t_dn2: f64,
        var_theexp_t_dn3: f64,
        var_theexp_t_dn4: f64,
        var_theexp_t_dn5: f64,
        var_theexp_t_dn6: f64,
        var_vbiei: f64,
        var_vbiei_db0: f64,
        var_vbiei_db1: f64,
        var_vbiei_db2: f64,
        var_vbiei_db3: f64,
        var_vbiei_db4: f64,
        var_vbiei_db5: f64,
        var_vbiei_db6: f64,
        var_vbiei_dn0: f64,
        var_vbiei_dn1: f64,
        var_vbiei_dn2: f64,
        var_vbiei_dn3: f64,
        var_vbiei_dn4: f64,
        var_vbiei_dn5: f64,
        var_vbiei_dn6: f64,
        var_vt: f64,
        var_vt_db0: f64,
        var_vt_db1: f64,
        var_vt_db2: f64,
        var_vt_db3: f64,
        var_vt_db4: f64,
        var_vt_db5: f64,
        var_vt_db6: f64,
        var_vt_dn0: f64,
        var_vt_dn1: f64,
        var_vt_dn2: f64,
        var_vt_dn3: f64,
        var_vt_dn4: f64,
        var_vt_dn5: f64,
        var_vt_dn6: f64,
        var_arg_slot: &mut f64,
        var_arg_db0_slot: &mut f64,
        var_arg_db1_slot: &mut f64,
        var_arg_db2_slot: &mut f64,
        var_arg_db3_slot: &mut f64,
        var_arg_db4_slot: &mut f64,
        var_arg_db5_slot: &mut f64,
        var_arg_db6_slot: &mut f64,
        var_arg_dn0_slot: &mut f64,
        var_arg_dn1_slot: &mut f64,
        var_arg_dn2_slot: &mut f64,
        var_arg_dn3_slot: &mut f64,
        var_arg_dn4_slot: &mut f64,
        var_arg_dn5_slot: &mut f64,
        var_arg_dn6_slot: &mut f64,
        var_arg_rdb0_slot: &mut f64,
        var_arg_rdb1_slot: &mut f64,
        var_arg_rdb2_slot: &mut f64,
        var_arg_rdb3_slot: &mut f64,
        var_arg_rdb4_slot: &mut f64,
        var_arg_rdb5_slot: &mut f64,
        var_arg_rdb6_slot: &mut f64,
        var_arg_rdn0_slot: &mut f64,
        var_arg_rdn1_slot: &mut f64,
        var_arg_rdn2_slot: &mut f64,
        var_arg_rdn3_slot: &mut f64,
        var_arg_rdn4_slot: &mut f64,
        var_arg_rdn5_slot: &mut f64,
        var_arg_rdn6_slot: &mut f64,
        var_arg_rv_slot: &mut f64,
        var_argbv_slot: &mut f64,
        var_argbv_db0_slot: &mut f64,
        var_argbv_db1_slot: &mut f64,
        var_argbv_db2_slot: &mut f64,
        var_argbv_db3_slot: &mut f64,
        var_argbv_db4_slot: &mut f64,
        var_argbv_db5_slot: &mut f64,
        var_argbv_db6_slot: &mut f64,
        var_argbv_dn0_slot: &mut f64,
        var_argbv_dn1_slot: &mut f64,
        var_argbv_dn2_slot: &mut f64,
        var_argbv_dn3_slot: &mut f64,
        var_argbv_dn4_slot: &mut f64,
        var_argbv_dn5_slot: &mut f64,
        var_argbv_dn6_slot: &mut f64,
        var_argbv_rdb0_slot: &mut f64,
        var_argbv_rdb1_slot: &mut f64,
        var_argbv_rdb2_slot: &mut f64,
        var_argbv_rdb3_slot: &mut f64,
        var_argbv_rdb4_slot: &mut f64,
        var_argbv_rdb5_slot: &mut f64,
        var_argbv_rdb6_slot: &mut f64,
        var_argbv_rdn0_slot: &mut f64,
        var_argbv_rdn1_slot: &mut f64,
        var_argbv_rdn2_slot: &mut f64,
        var_argbv_rdn3_slot: &mut f64,
        var_argbv_rdn4_slot: &mut f64,
        var_argbv_rdn5_slot: &mut f64,
        var_argbv_rdn6_slot: &mut f64,
        var_argbv_rv_slot: &mut f64,
        var_argbvvt_slot: &mut f64,
        var_argbvvt_db0_slot: &mut f64,
        var_argbvvt_db1_slot: &mut f64,
        var_argbvvt_db2_slot: &mut f64,
        var_argbvvt_db3_slot: &mut f64,
        var_argbvvt_db4_slot: &mut f64,
        var_argbvvt_db5_slot: &mut f64,
        var_argbvvt_db6_slot: &mut f64,
        var_argbvvt_dn0_slot: &mut f64,
        var_argbvvt_dn1_slot: &mut f64,
        var_argbvvt_dn2_slot: &mut f64,
        var_argbvvt_dn3_slot: &mut f64,
        var_argbvvt_dn4_slot: &mut f64,
        var_argbvvt_dn5_slot: &mut f64,
        var_argbvvt_dn6_slot: &mut f64,
        var_argbvvt_rdb0_slot: &mut f64,
        var_argbvvt_rdb1_slot: &mut f64,
        var_argbvvt_rdb2_slot: &mut f64,
        var_argbvvt_rdb3_slot: &mut f64,
        var_argbvvt_rdb4_slot: &mut f64,
        var_argbvvt_rdb5_slot: &mut f64,
        var_argbvvt_rdb6_slot: &mut f64,
        var_argbvvt_rdn0_slot: &mut f64,
        var_argbvvt_rdn1_slot: &mut f64,
        var_argbvvt_rdn2_slot: &mut f64,
        var_argbvvt_rdn3_slot: &mut f64,
        var_argbvvt_rdn4_slot: &mut f64,
        var_argbvvt_rdn5_slot: &mut f64,
        var_argbvvt_rdn6_slot: &mut f64,
        var_argbvvt_rv_slot: &mut f64,
        var_guard4_slot: &mut f64,
        var_guard4_db0_slot: &mut f64,
        var_guard4_db1_slot: &mut f64,
        var_guard4_db2_slot: &mut f64,
        var_guard4_db3_slot: &mut f64,
        var_guard4_db4_slot: &mut f64,
        var_guard4_db5_slot: &mut f64,
        var_guard4_db6_slot: &mut f64,
        var_guard4_dn0_slot: &mut f64,
        var_guard4_dn1_slot: &mut f64,
        var_guard4_dn2_slot: &mut f64,
        var_guard4_dn3_slot: &mut f64,
        var_guard4_dn4_slot: &mut f64,
        var_guard4_dn5_slot: &mut f64,
        var_guard4_dn6_slot: &mut f64,
        var_guard4_rdb0_slot: &mut f64,
        var_guard4_rdb1_slot: &mut f64,
        var_guard4_rdb2_slot: &mut f64,
        var_guard4_rdb3_slot: &mut f64,
        var_guard4_rdb4_slot: &mut f64,
        var_guard4_rdb5_slot: &mut f64,
        var_guard4_rdb6_slot: &mut f64,
        var_guard4_rdn0_slot: &mut f64,
        var_guard4_rdn1_slot: &mut f64,
        var_guard4_rdn2_slot: &mut f64,
        var_guard4_rdn3_slot: &mut f64,
        var_guard4_rdn4_slot: &mut f64,
        var_guard4_rdn5_slot: &mut f64,
        var_guard4_rdn6_slot: &mut f64,
        var_guard4_rv_slot: &mut f64,
        var_ifwd_slot: &mut f64,
        var_ifwd_db0_slot: &mut f64,
        var_ifwd_db1_slot: &mut f64,
        var_ifwd_db2_slot: &mut f64,
        var_ifwd_db3_slot: &mut f64,
        var_ifwd_db4_slot: &mut f64,
        var_ifwd_db5_slot: &mut f64,
        var_ifwd_db6_slot: &mut f64,
        var_ifwd_dn0_slot: &mut f64,
        var_ifwd_dn1_slot: &mut f64,
        var_ifwd_dn2_slot: &mut f64,
        var_ifwd_dn3_slot: &mut f64,
        var_ifwd_dn4_slot: &mut f64,
        var_ifwd_dn5_slot: &mut f64,
        var_ifwd_dn6_slot: &mut f64,
        var_ifwd_rdb0_slot: &mut f64,
        var_ifwd_rdb1_slot: &mut f64,
        var_ifwd_rdb2_slot: &mut f64,
        var_ifwd_rdb3_slot: &mut f64,
        var_ifwd_rdb4_slot: &mut f64,
        var_ifwd_rdb5_slot: &mut f64,
        var_ifwd_rdb6_slot: &mut f64,
        var_ifwd_rdn0_slot: &mut f64,
        var_ifwd_rdn1_slot: &mut f64,
        var_ifwd_rdn2_slot: &mut f64,
        var_ifwd_rdn3_slot: &mut f64,
        var_ifwd_rdn4_slot: &mut f64,
        var_ifwd_rdn5_slot: &mut f64,
        var_ifwd_rdn6_slot: &mut f64,
        var_ifwd_rv_slot: &mut f64,
        var_le_slot: &mut f64,
        var_le_db0_slot: &mut f64,
        var_le_db1_slot: &mut f64,
        var_le_db2_slot: &mut f64,
        var_le_db3_slot: &mut f64,
        var_le_db4_slot: &mut f64,
        var_le_db5_slot: &mut f64,
        var_le_db6_slot: &mut f64,
        var_le_dn0_slot: &mut f64,
        var_le_dn1_slot: &mut f64,
        var_le_dn2_slot: &mut f64,
        var_le_dn3_slot: &mut f64,
        var_le_dn4_slot: &mut f64,
        var_le_dn5_slot: &mut f64,
        var_le_dn6_slot: &mut f64,
        var_le_rdb0_slot: &mut f64,
        var_le_rdb1_slot: &mut f64,
        var_le_rdb2_slot: &mut f64,
        var_le_rdb3_slot: &mut f64,
        var_le_rdb4_slot: &mut f64,
        var_le_rdb5_slot: &mut f64,
        var_le_rdb6_slot: &mut f64,
        var_le_rdn0_slot: &mut f64,
        var_le_rdn1_slot: &mut f64,
        var_le_rdn2_slot: &mut f64,
        var_le_rdn3_slot: &mut f64,
        var_le_rdn4_slot: &mut f64,
        var_le_rdn5_slot: &mut f64,
        var_le_rdn6_slot: &mut f64,
        var_le_rv_slot: &mut f64,
        var_lebv_slot: &mut f64,
        var_lebv_db0_slot: &mut f64,
        var_lebv_db1_slot: &mut f64,
        var_lebv_db2_slot: &mut f64,
        var_lebv_db3_slot: &mut f64,
        var_lebv_db4_slot: &mut f64,
        var_lebv_db5_slot: &mut f64,
        var_lebv_db6_slot: &mut f64,
        var_lebv_dn0_slot: &mut f64,
        var_lebv_dn1_slot: &mut f64,
        var_lebv_dn2_slot: &mut f64,
        var_lebv_dn3_slot: &mut f64,
        var_lebv_dn4_slot: &mut f64,
        var_lebv_dn5_slot: &mut f64,
        var_lebv_dn6_slot: &mut f64,
        var_lebv_rdb0_slot: &mut f64,
        var_lebv_rdb1_slot: &mut f64,
        var_lebv_rdb2_slot: &mut f64,
        var_lebv_rdb3_slot: &mut f64,
        var_lebv_rdb4_slot: &mut f64,
        var_lebv_rdb5_slot: &mut f64,
        var_lebv_rdb6_slot: &mut f64,
        var_lebv_rdn0_slot: &mut f64,
        var_lebv_rdn1_slot: &mut f64,
        var_lebv_rdn2_slot: &mut f64,
        var_lebv_rdn3_slot: &mut f64,
        var_lebv_rdn4_slot: &mut f64,
        var_lebv_rdn5_slot: &mut f64,
        var_lebv_rdn6_slot: &mut f64,
        var_lebv_rv_slot: &mut f64,
    ) {
        let mut var_arg: f64 = *var_arg_slot;
        let mut var_arg_db0: f64 = *var_arg_db0_slot;
        let mut var_arg_db1: f64 = *var_arg_db1_slot;
        let mut var_arg_db2: f64 = *var_arg_db2_slot;
        let mut var_arg_db3: f64 = *var_arg_db3_slot;
        let mut var_arg_db4: f64 = *var_arg_db4_slot;
        let mut var_arg_db5: f64 = *var_arg_db5_slot;
        let mut var_arg_db6: f64 = *var_arg_db6_slot;
        let mut var_arg_dn0: f64 = *var_arg_dn0_slot;
        let mut var_arg_dn1: f64 = *var_arg_dn1_slot;
        let mut var_arg_dn2: f64 = *var_arg_dn2_slot;
        let mut var_arg_dn3: f64 = *var_arg_dn3_slot;
        let mut var_arg_dn4: f64 = *var_arg_dn4_slot;
        let mut var_arg_dn5: f64 = *var_arg_dn5_slot;
        let mut var_arg_dn6: f64 = *var_arg_dn6_slot;
        let mut var_arg_rdb0: f64 = *var_arg_rdb0_slot;
        let mut var_arg_rdb1: f64 = *var_arg_rdb1_slot;
        let mut var_arg_rdb2: f64 = *var_arg_rdb2_slot;
        let mut var_arg_rdb3: f64 = *var_arg_rdb3_slot;
        let mut var_arg_rdb4: f64 = *var_arg_rdb4_slot;
        let mut var_arg_rdb5: f64 = *var_arg_rdb5_slot;
        let mut var_arg_rdb6: f64 = *var_arg_rdb6_slot;
        let mut var_arg_rdn0: f64 = *var_arg_rdn0_slot;
        let mut var_arg_rdn1: f64 = *var_arg_rdn1_slot;
        let mut var_arg_rdn2: f64 = *var_arg_rdn2_slot;
        let mut var_arg_rdn3: f64 = *var_arg_rdn3_slot;
        let mut var_arg_rdn4: f64 = *var_arg_rdn4_slot;
        let mut var_arg_rdn5: f64 = *var_arg_rdn5_slot;
        let mut var_arg_rdn6: f64 = *var_arg_rdn6_slot;
        let mut var_arg_rv: f64 = *var_arg_rv_slot;
        let mut var_argbv: f64 = *var_argbv_slot;
        let mut var_argbv_db0: f64 = *var_argbv_db0_slot;
        let mut var_argbv_db1: f64 = *var_argbv_db1_slot;
        let mut var_argbv_db2: f64 = *var_argbv_db2_slot;
        let mut var_argbv_db3: f64 = *var_argbv_db3_slot;
        let mut var_argbv_db4: f64 = *var_argbv_db4_slot;
        let mut var_argbv_db5: f64 = *var_argbv_db5_slot;
        let mut var_argbv_db6: f64 = *var_argbv_db6_slot;
        let mut var_argbv_dn0: f64 = *var_argbv_dn0_slot;
        let mut var_argbv_dn1: f64 = *var_argbv_dn1_slot;
        let mut var_argbv_dn2: f64 = *var_argbv_dn2_slot;
        let mut var_argbv_dn3: f64 = *var_argbv_dn3_slot;
        let mut var_argbv_dn4: f64 = *var_argbv_dn4_slot;
        let mut var_argbv_dn5: f64 = *var_argbv_dn5_slot;
        let mut var_argbv_dn6: f64 = *var_argbv_dn6_slot;
        let mut var_argbv_rdb0: f64 = *var_argbv_rdb0_slot;
        let mut var_argbv_rdb1: f64 = *var_argbv_rdb1_slot;
        let mut var_argbv_rdb2: f64 = *var_argbv_rdb2_slot;
        let mut var_argbv_rdb3: f64 = *var_argbv_rdb3_slot;
        let mut var_argbv_rdb4: f64 = *var_argbv_rdb4_slot;
        let mut var_argbv_rdb5: f64 = *var_argbv_rdb5_slot;
        let mut var_argbv_rdb6: f64 = *var_argbv_rdb6_slot;
        let mut var_argbv_rdn0: f64 = *var_argbv_rdn0_slot;
        let mut var_argbv_rdn1: f64 = *var_argbv_rdn1_slot;
        let mut var_argbv_rdn2: f64 = *var_argbv_rdn2_slot;
        let mut var_argbv_rdn3: f64 = *var_argbv_rdn3_slot;
        let mut var_argbv_rdn4: f64 = *var_argbv_rdn4_slot;
        let mut var_argbv_rdn5: f64 = *var_argbv_rdn5_slot;
        let mut var_argbv_rdn6: f64 = *var_argbv_rdn6_slot;
        let mut var_argbv_rv: f64 = *var_argbv_rv_slot;
        let mut var_argbvvt: f64 = *var_argbvvt_slot;
        let mut var_argbvvt_db0: f64 = *var_argbvvt_db0_slot;
        let mut var_argbvvt_db1: f64 = *var_argbvvt_db1_slot;
        let mut var_argbvvt_db2: f64 = *var_argbvvt_db2_slot;
        let mut var_argbvvt_db3: f64 = *var_argbvvt_db3_slot;
        let mut var_argbvvt_db4: f64 = *var_argbvvt_db4_slot;
        let mut var_argbvvt_db5: f64 = *var_argbvvt_db5_slot;
        let mut var_argbvvt_db6: f64 = *var_argbvvt_db6_slot;
        let mut var_argbvvt_dn0: f64 = *var_argbvvt_dn0_slot;
        let mut var_argbvvt_dn1: f64 = *var_argbvvt_dn1_slot;
        let mut var_argbvvt_dn2: f64 = *var_argbvvt_dn2_slot;
        let mut var_argbvvt_dn3: f64 = *var_argbvvt_dn3_slot;
        let mut var_argbvvt_dn4: f64 = *var_argbvvt_dn4_slot;
        let mut var_argbvvt_dn5: f64 = *var_argbvvt_dn5_slot;
        let mut var_argbvvt_dn6: f64 = *var_argbvvt_dn6_slot;
        let mut var_argbvvt_rdb0: f64 = *var_argbvvt_rdb0_slot;
        let mut var_argbvvt_rdb1: f64 = *var_argbvvt_rdb1_slot;
        let mut var_argbvvt_rdb2: f64 = *var_argbvvt_rdb2_slot;
        let mut var_argbvvt_rdb3: f64 = *var_argbvvt_rdb3_slot;
        let mut var_argbvvt_rdb4: f64 = *var_argbvvt_rdb4_slot;
        let mut var_argbvvt_rdb5: f64 = *var_argbvvt_rdb5_slot;
        let mut var_argbvvt_rdb6: f64 = *var_argbvvt_rdb6_slot;
        let mut var_argbvvt_rdn0: f64 = *var_argbvvt_rdn0_slot;
        let mut var_argbvvt_rdn1: f64 = *var_argbvvt_rdn1_slot;
        let mut var_argbvvt_rdn2: f64 = *var_argbvvt_rdn2_slot;
        let mut var_argbvvt_rdn3: f64 = *var_argbvvt_rdn3_slot;
        let mut var_argbvvt_rdn4: f64 = *var_argbvvt_rdn4_slot;
        let mut var_argbvvt_rdn5: f64 = *var_argbvvt_rdn5_slot;
        let mut var_argbvvt_rdn6: f64 = *var_argbvvt_rdn6_slot;
        let mut var_argbvvt_rv: f64 = *var_argbvvt_rv_slot;
        let mut var_guard4: f64 = *var_guard4_slot;
        let mut var_guard4_db0: f64 = *var_guard4_db0_slot;
        let mut var_guard4_db1: f64 = *var_guard4_db1_slot;
        let mut var_guard4_db2: f64 = *var_guard4_db2_slot;
        let mut var_guard4_db3: f64 = *var_guard4_db3_slot;
        let mut var_guard4_db4: f64 = *var_guard4_db4_slot;
        let mut var_guard4_db5: f64 = *var_guard4_db5_slot;
        let mut var_guard4_db6: f64 = *var_guard4_db6_slot;
        let mut var_guard4_dn0: f64 = *var_guard4_dn0_slot;
        let mut var_guard4_dn1: f64 = *var_guard4_dn1_slot;
        let mut var_guard4_dn2: f64 = *var_guard4_dn2_slot;
        let mut var_guard4_dn3: f64 = *var_guard4_dn3_slot;
        let mut var_guard4_dn4: f64 = *var_guard4_dn4_slot;
        let mut var_guard4_dn5: f64 = *var_guard4_dn5_slot;
        let mut var_guard4_dn6: f64 = *var_guard4_dn6_slot;
        let mut var_guard4_rdb0: f64 = *var_guard4_rdb0_slot;
        let mut var_guard4_rdb1: f64 = *var_guard4_rdb1_slot;
        let mut var_guard4_rdb2: f64 = *var_guard4_rdb2_slot;
        let mut var_guard4_rdb3: f64 = *var_guard4_rdb3_slot;
        let mut var_guard4_rdb4: f64 = *var_guard4_rdb4_slot;
        let mut var_guard4_rdb5: f64 = *var_guard4_rdb5_slot;
        let mut var_guard4_rdb6: f64 = *var_guard4_rdb6_slot;
        let mut var_guard4_rdn0: f64 = *var_guard4_rdn0_slot;
        let mut var_guard4_rdn1: f64 = *var_guard4_rdn1_slot;
        let mut var_guard4_rdn2: f64 = *var_guard4_rdn2_slot;
        let mut var_guard4_rdn3: f64 = *var_guard4_rdn3_slot;
        let mut var_guard4_rdn4: f64 = *var_guard4_rdn4_slot;
        let mut var_guard4_rdn5: f64 = *var_guard4_rdn5_slot;
        let mut var_guard4_rdn6: f64 = *var_guard4_rdn6_slot;
        let mut var_guard4_rv: f64 = *var_guard4_rv_slot;
        let mut var_ifwd: f64 = *var_ifwd_slot;
        let mut var_ifwd_db0: f64 = *var_ifwd_db0_slot;
        let mut var_ifwd_db1: f64 = *var_ifwd_db1_slot;
        let mut var_ifwd_db2: f64 = *var_ifwd_db2_slot;
        let mut var_ifwd_db3: f64 = *var_ifwd_db3_slot;
        let mut var_ifwd_db4: f64 = *var_ifwd_db4_slot;
        let mut var_ifwd_db5: f64 = *var_ifwd_db5_slot;
        let mut var_ifwd_db6: f64 = *var_ifwd_db6_slot;
        let mut var_ifwd_dn0: f64 = *var_ifwd_dn0_slot;
        let mut var_ifwd_dn1: f64 = *var_ifwd_dn1_slot;
        let mut var_ifwd_dn2: f64 = *var_ifwd_dn2_slot;
        let mut var_ifwd_dn3: f64 = *var_ifwd_dn3_slot;
        let mut var_ifwd_dn4: f64 = *var_ifwd_dn4_slot;
        let mut var_ifwd_dn5: f64 = *var_ifwd_dn5_slot;
        let mut var_ifwd_dn6: f64 = *var_ifwd_dn6_slot;
        let mut var_ifwd_rdb0: f64 = *var_ifwd_rdb0_slot;
        let mut var_ifwd_rdb1: f64 = *var_ifwd_rdb1_slot;
        let mut var_ifwd_rdb2: f64 = *var_ifwd_rdb2_slot;
        let mut var_ifwd_rdb3: f64 = *var_ifwd_rdb3_slot;
        let mut var_ifwd_rdb4: f64 = *var_ifwd_rdb4_slot;
        let mut var_ifwd_rdb5: f64 = *var_ifwd_rdb5_slot;
        let mut var_ifwd_rdb6: f64 = *var_ifwd_rdb6_slot;
        let mut var_ifwd_rdn0: f64 = *var_ifwd_rdn0_slot;
        let mut var_ifwd_rdn1: f64 = *var_ifwd_rdn1_slot;
        let mut var_ifwd_rdn2: f64 = *var_ifwd_rdn2_slot;
        let mut var_ifwd_rdn3: f64 = *var_ifwd_rdn3_slot;
        let mut var_ifwd_rdn4: f64 = *var_ifwd_rdn4_slot;
        let mut var_ifwd_rdn5: f64 = *var_ifwd_rdn5_slot;
        let mut var_ifwd_rdn6: f64 = *var_ifwd_rdn6_slot;
        let mut var_ifwd_rv: f64 = *var_ifwd_rv_slot;
        let mut var_le: f64 = *var_le_slot;
        let mut var_le_db0: f64 = *var_le_db0_slot;
        let mut var_le_db1: f64 = *var_le_db1_slot;
        let mut var_le_db2: f64 = *var_le_db2_slot;
        let mut var_le_db3: f64 = *var_le_db3_slot;
        let mut var_le_db4: f64 = *var_le_db4_slot;
        let mut var_le_db5: f64 = *var_le_db5_slot;
        let mut var_le_db6: f64 = *var_le_db6_slot;
        let mut var_le_dn0: f64 = *var_le_dn0_slot;
        let mut var_le_dn1: f64 = *var_le_dn1_slot;
        let mut var_le_dn2: f64 = *var_le_dn2_slot;
        let mut var_le_dn3: f64 = *var_le_dn3_slot;
        let mut var_le_dn4: f64 = *var_le_dn4_slot;
        let mut var_le_dn5: f64 = *var_le_dn5_slot;
        let mut var_le_dn6: f64 = *var_le_dn6_slot;
        let mut var_le_rdb0: f64 = *var_le_rdb0_slot;
        let mut var_le_rdb1: f64 = *var_le_rdb1_slot;
        let mut var_le_rdb2: f64 = *var_le_rdb2_slot;
        let mut var_le_rdb3: f64 = *var_le_rdb3_slot;
        let mut var_le_rdb4: f64 = *var_le_rdb4_slot;
        let mut var_le_rdb5: f64 = *var_le_rdb5_slot;
        let mut var_le_rdb6: f64 = *var_le_rdb6_slot;
        let mut var_le_rdn0: f64 = *var_le_rdn0_slot;
        let mut var_le_rdn1: f64 = *var_le_rdn1_slot;
        let mut var_le_rdn2: f64 = *var_le_rdn2_slot;
        let mut var_le_rdn3: f64 = *var_le_rdn3_slot;
        let mut var_le_rdn4: f64 = *var_le_rdn4_slot;
        let mut var_le_rdn5: f64 = *var_le_rdn5_slot;
        let mut var_le_rdn6: f64 = *var_le_rdn6_slot;
        let mut var_le_rv: f64 = *var_le_rv_slot;
        let mut var_lebv: f64 = *var_lebv_slot;
        let mut var_lebv_db0: f64 = *var_lebv_db0_slot;
        let mut var_lebv_db1: f64 = *var_lebv_db1_slot;
        let mut var_lebv_db2: f64 = *var_lebv_db2_slot;
        let mut var_lebv_db3: f64 = *var_lebv_db3_slot;
        let mut var_lebv_db4: f64 = *var_lebv_db4_slot;
        let mut var_lebv_db5: f64 = *var_lebv_db5_slot;
        let mut var_lebv_db6: f64 = *var_lebv_db6_slot;
        let mut var_lebv_dn0: f64 = *var_lebv_dn0_slot;
        let mut var_lebv_dn1: f64 = *var_lebv_dn1_slot;
        let mut var_lebv_dn2: f64 = *var_lebv_dn2_slot;
        let mut var_lebv_dn3: f64 = *var_lebv_dn3_slot;
        let mut var_lebv_dn4: f64 = *var_lebv_dn4_slot;
        let mut var_lebv_dn5: f64 = *var_lebv_dn5_slot;
        let mut var_lebv_dn6: f64 = *var_lebv_dn6_slot;
        let mut var_lebv_rdb0: f64 = *var_lebv_rdb0_slot;
        let mut var_lebv_rdb1: f64 = *var_lebv_rdb1_slot;
        let mut var_lebv_rdb2: f64 = *var_lebv_rdb2_slot;
        let mut var_lebv_rdb3: f64 = *var_lebv_rdb3_slot;
        let mut var_lebv_rdb4: f64 = *var_lebv_rdb4_slot;
        let mut var_lebv_rdb5: f64 = *var_lebv_rdb5_slot;
        let mut var_lebv_rdb6: f64 = *var_lebv_rdb6_slot;
        let mut var_lebv_rdn0: f64 = *var_lebv_rdn0_slot;
        let mut var_lebv_rdn1: f64 = *var_lebv_rdn1_slot;
        let mut var_lebv_rdn2: f64 = *var_lebv_rdn2_slot;
        let mut var_lebv_rdn3: f64 = *var_lebv_rdn3_slot;
        let mut var_lebv_rdn4: f64 = *var_lebv_rdn4_slot;
        let mut var_lebv_rdn5: f64 = *var_lebv_rdn5_slot;
        let mut var_lebv_rdn6: f64 = *var_lebv_rdn6_slot;
        let mut var_lebv_rv: f64 = *var_lebv_rv_slot;

        let (assign330_e511, assign330_e511_d_n0, assign330_e511_d_n1, assign330_e511_d_n2, assign330_e511_d_n3, assign330_e511_d_n4, assign330_e511_d_n5, assign330_e511_d_n6, assign330_e511_d_b0, assign330_e511_d_b1, assign330_e511_d_b2, assign330_e511_d_b3, assign330_e511_d_b4, assign330_e511_d_b5, assign330_e511_d_b6,) = {
    if (var_guard3 != 0.0) {
        let assign330_e508: f64 = (p.p1 * var_vt);
        let assign330_e509: f64 = (var_vbiei / assign330_e508);
        (assign330_e509, (((var_vbiei_dn0 * assign330_e508) - (var_vbiei * (p.p1 * var_vt_dn0))) / (assign330_e508 * assign330_e508)), (((var_vbiei_dn1 * assign330_e508) - (var_vbiei * (p.p1 * var_vt_dn1))) / (assign330_e508 * assign330_e508)), (((var_vbiei_dn2 * assign330_e508) - (var_vbiei * (p.p1 * var_vt_dn2))) / (assign330_e508 * assign330_e508)), (((var_vbiei_dn3 * assign330_e508) - (var_vbiei * (p.p1 * var_vt_dn3))) / (assign330_e508 * assign330_e508)), (((var_vbiei_dn4 * assign330_e508) - (var_vbiei * (p.p1 * var_vt_dn4))) / (assign330_e508 * assign330_e508)), (((var_vbiei_dn5 * assign330_e508) - (var_vbiei * (p.p1 * var_vt_dn5))) / (assign330_e508 * assign330_e508)), (((var_vbiei_dn6 * assign330_e508) - (var_vbiei * (p.p1 * var_vt_dn6))) / (assign330_e508 * assign330_e508)), (((var_vbiei_db0 * assign330_e508) - (var_vbiei * (p.p1 * var_vt_db0))) / (assign330_e508 * assign330_e508)), (((var_vbiei_db1 * assign330_e508) - (var_vbiei * (p.p1 * var_vt_db1))) / (assign330_e508 * assign330_e508)), (((var_vbiei_db2 * assign330_e508) - (var_vbiei * (p.p1 * var_vt_db2))) / (assign330_e508 * assign330_e508)), (((var_vbiei_db3 * assign330_e508) - (var_vbiei * (p.p1 * var_vt_db3))) / (assign330_e508 * assign330_e508)), (((var_vbiei_db4 * assign330_e508) - (var_vbiei * (p.p1 * var_vt_db4))) / (assign330_e508 * assign330_e508)), (((var_vbiei_db5 * assign330_e508) - (var_vbiei * (p.p1 * var_vt_db5))) / (assign330_e508 * assign330_e508)), (((var_vbiei_db6 * assign330_e508) - (var_vbiei * (p.p1 * var_vt_db6))) / (assign330_e508 * assign330_e508)),)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn1, var_arg_dn2, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_db0, var_arg_db1, var_arg_db2, var_arg_db3, var_arg_db4, var_arg_db5, var_arg_db6,)
    }
};
        var_arg = assign330_e511;
        var_arg_dn0 = assign330_e511_d_n0;
        var_arg_dn1 = assign330_e511_d_n1;
        var_arg_dn2 = assign330_e511_d_n2;
        var_arg_dn3 = assign330_e511_d_n3;
        var_arg_dn4 = assign330_e511_d_n4;
        var_arg_dn5 = assign330_e511_d_n5;
        var_arg_dn6 = assign330_e511_d_n6;
        var_arg_db0 = assign330_e511_d_b0;
        var_arg_db1 = assign330_e511_d_b1;
        var_arg_db2 = assign330_e511_d_b2;
        var_arg_db3 = assign330_e511_d_b3;
        var_arg_db4 = assign330_e511_d_b4;
        var_arg_db5 = assign330_e511_d_b5;
        var_arg_db6 = assign330_e511_d_b6;
        var_arg_rv = 0.0;
        var_arg_rdn0 = 0.0;
        var_arg_rdn1 = 0.0;
        var_arg_rdn2 = 0.0;
        var_arg_rdn3 = 0.0;
        var_arg_rdn4 = 0.0;
        var_arg_rdn5 = 0.0;
        var_arg_rdn6 = 0.0;
        var_arg_rdb0 = 0.0;
        var_arg_rdb1 = 0.0;
        var_arg_rdb2 = 0.0;
        var_arg_rdb3 = 0.0;
        var_arg_rdb4 = 0.0;
        var_arg_rdb5 = 0.0;
        var_arg_rdb6 = 0.0;

        let (assign340_e522, assign340_e522_d_n0, assign340_e522_d_n1, assign340_e522_d_n2, assign340_e522_d_n3, assign340_e522_d_n4, assign340_e522_d_n5, assign340_e522_d_n6, assign340_e522_d_b0, assign340_e522_d_b1, assign340_e522_d_b2, assign340_e522_d_b3, assign340_e522_d_b4, assign340_e522_d_b5, assign340_e522_d_b6,) = {
    if (var_guard3 != 0.0) {
        let assign340_e514: f64 = (-var_vbiei);
        let assign340_e516: f64 = (assign340_e514 - var_bvr_t);
        let assign340_e519: f64 = (p.p11 * var_vt);
        let assign340_e520: f64 = (assign340_e516 / assign340_e519);
        (assign340_e520, (((((-var_vbiei_dn0) - var_bvr_t_dn0) * assign340_e519) - (assign340_e516 * (p.p11 * var_vt_dn0))) / (assign340_e519 * assign340_e519)), (((((-var_vbiei_dn1) - var_bvr_t_dn1) * assign340_e519) - (assign340_e516 * (p.p11 * var_vt_dn1))) / (assign340_e519 * assign340_e519)), (((((-var_vbiei_dn2) - var_bvr_t_dn2) * assign340_e519) - (assign340_e516 * (p.p11 * var_vt_dn2))) / (assign340_e519 * assign340_e519)), (((((-var_vbiei_dn3) - var_bvr_t_dn3) * assign340_e519) - (assign340_e516 * (p.p11 * var_vt_dn3))) / (assign340_e519 * assign340_e519)), (((((-var_vbiei_dn4) - var_bvr_t_dn4) * assign340_e519) - (assign340_e516 * (p.p11 * var_vt_dn4))) / (assign340_e519 * assign340_e519)), (((((-var_vbiei_dn5) - var_bvr_t_dn5) * assign340_e519) - (assign340_e516 * (p.p11 * var_vt_dn5))) / (assign340_e519 * assign340_e519)), (((((-var_vbiei_dn6) - var_bvr_t_dn6) * assign340_e519) - (assign340_e516 * (p.p11 * var_vt_dn6))) / (assign340_e519 * assign340_e519)), (((((-var_vbiei_db0) - var_bvr_t_db0) * assign340_e519) - (assign340_e516 * (p.p11 * var_vt_db0))) / (assign340_e519 * assign340_e519)), (((((-var_vbiei_db1) - var_bvr_t_db1) * assign340_e519) - (assign340_e516 * (p.p11 * var_vt_db1))) / (assign340_e519 * assign340_e519)), (((((-var_vbiei_db2) - var_bvr_t_db2) * assign340_e519) - (assign340_e516 * (p.p11 * var_vt_db2))) / (assign340_e519 * assign340_e519)), (((((-var_vbiei_db3) - var_bvr_t_db3) * assign340_e519) - (assign340_e516 * (p.p11 * var_vt_db3))) / (assign340_e519 * assign340_e519)), (((((-var_vbiei_db4) - var_bvr_t_db4) * assign340_e519) - (assign340_e516 * (p.p11 * var_vt_db4))) / (assign340_e519 * assign340_e519)), (((((-var_vbiei_db5) - var_bvr_t_db5) * assign340_e519) - (assign340_e516 * (p.p11 * var_vt_db5))) / (assign340_e519 * assign340_e519)), (((((-var_vbiei_db6) - var_bvr_t_db6) * assign340_e519) - (assign340_e516 * (p.p11 * var_vt_db6))) / (assign340_e519 * assign340_e519)),)
    } else {
        (var_argbv, var_argbv_dn0, var_argbv_dn1, var_argbv_dn2, var_argbv_dn3, var_argbv_dn4, var_argbv_dn5, var_argbv_dn6, var_argbv_db0, var_argbv_db1, var_argbv_db2, var_argbv_db3, var_argbv_db4, var_argbv_db5, var_argbv_db6,)
    }
};
        var_argbv = assign340_e522;
        var_argbv_dn0 = assign340_e522_d_n0;
        var_argbv_dn1 = assign340_e522_d_n1;
        var_argbv_dn2 = assign340_e522_d_n2;
        var_argbv_dn3 = assign340_e522_d_n3;
        var_argbv_dn4 = assign340_e522_d_n4;
        var_argbv_dn5 = assign340_e522_d_n5;
        var_argbv_dn6 = assign340_e522_d_n6;
        var_argbv_db0 = assign340_e522_d_b0;
        var_argbv_db1 = assign340_e522_d_b1;
        var_argbv_db2 = assign340_e522_d_b2;
        var_argbv_db3 = assign340_e522_d_b3;
        var_argbv_db4 = assign340_e522_d_b4;
        var_argbv_db5 = assign340_e522_d_b5;
        var_argbv_db6 = assign340_e522_d_b6;
        var_argbv_rv = 0.0;
        var_argbv_rdn0 = 0.0;
        var_argbv_rdn1 = 0.0;
        var_argbv_rdn2 = 0.0;
        var_argbv_rdn3 = 0.0;
        var_argbv_rdn4 = 0.0;
        var_argbv_rdn5 = 0.0;
        var_argbv_rdn6 = 0.0;
        var_argbv_rdb0 = 0.0;
        var_argbv_rdb1 = 0.0;
        var_argbv_rdb2 = 0.0;
        var_argbv_rdb3 = 0.0;
        var_argbv_rdb4 = 0.0;
        var_argbv_rdb5 = 0.0;
        var_argbv_rdb6 = 0.0;

        let (assign350_e531, assign350_e531_d_n0, assign350_e531_d_n1, assign350_e531_d_n2, assign350_e531_d_n3, assign350_e531_d_n4, assign350_e531_d_n5, assign350_e531_d_n6, assign350_e531_d_b0, assign350_e531_d_b1, assign350_e531_d_b2, assign350_e531_d_b3, assign350_e531_d_b4, assign350_e531_d_b5, assign350_e531_d_b6,) = {
    if (var_guard3 != 0.0) {
        let assign350_e525: f64 = (-var_bvr_t);
        let assign350_e528: f64 = (p.p11 * var_vt);
        let assign350_e529: f64 = (assign350_e525 / assign350_e528);
        (assign350_e529, ((((-var_bvr_t_dn0) * assign350_e528) - (assign350_e525 * (p.p11 * var_vt_dn0))) / (assign350_e528 * assign350_e528)), ((((-var_bvr_t_dn1) * assign350_e528) - (assign350_e525 * (p.p11 * var_vt_dn1))) / (assign350_e528 * assign350_e528)), ((((-var_bvr_t_dn2) * assign350_e528) - (assign350_e525 * (p.p11 * var_vt_dn2))) / (assign350_e528 * assign350_e528)), ((((-var_bvr_t_dn3) * assign350_e528) - (assign350_e525 * (p.p11 * var_vt_dn3))) / (assign350_e528 * assign350_e528)), ((((-var_bvr_t_dn4) * assign350_e528) - (assign350_e525 * (p.p11 * var_vt_dn4))) / (assign350_e528 * assign350_e528)), ((((-var_bvr_t_dn5) * assign350_e528) - (assign350_e525 * (p.p11 * var_vt_dn5))) / (assign350_e528 * assign350_e528)), ((((-var_bvr_t_dn6) * assign350_e528) - (assign350_e525 * (p.p11 * var_vt_dn6))) / (assign350_e528 * assign350_e528)), ((((-var_bvr_t_db0) * assign350_e528) - (assign350_e525 * (p.p11 * var_vt_db0))) / (assign350_e528 * assign350_e528)), ((((-var_bvr_t_db1) * assign350_e528) - (assign350_e525 * (p.p11 * var_vt_db1))) / (assign350_e528 * assign350_e528)), ((((-var_bvr_t_db2) * assign350_e528) - (assign350_e525 * (p.p11 * var_vt_db2))) / (assign350_e528 * assign350_e528)), ((((-var_bvr_t_db3) * assign350_e528) - (assign350_e525 * (p.p11 * var_vt_db3))) / (assign350_e528 * assign350_e528)), ((((-var_bvr_t_db4) * assign350_e528) - (assign350_e525 * (p.p11 * var_vt_db4))) / (assign350_e528 * assign350_e528)), ((((-var_bvr_t_db5) * assign350_e528) - (assign350_e525 * (p.p11 * var_vt_db5))) / (assign350_e528 * assign350_e528)), ((((-var_bvr_t_db6) * assign350_e528) - (assign350_e525 * (p.p11 * var_vt_db6))) / (assign350_e528 * assign350_e528)),)
    } else {
        (var_argbvvt, var_argbvvt_dn0, var_argbvvt_dn1, var_argbvvt_dn2, var_argbvvt_dn3, var_argbvvt_dn4, var_argbvvt_dn5, var_argbvvt_dn6, var_argbvvt_db0, var_argbvvt_db1, var_argbvvt_db2, var_argbvvt_db3, var_argbvvt_db4, var_argbvvt_db5, var_argbvvt_db6,)
    }
};
        var_argbvvt = assign350_e531;
        var_argbvvt_dn0 = assign350_e531_d_n0;
        var_argbvvt_dn1 = assign350_e531_d_n1;
        var_argbvvt_dn2 = assign350_e531_d_n2;
        var_argbvvt_dn3 = assign350_e531_d_n3;
        var_argbvvt_dn4 = assign350_e531_d_n4;
        var_argbvvt_dn5 = assign350_e531_d_n5;
        var_argbvvt_dn6 = assign350_e531_d_n6;
        var_argbvvt_db0 = assign350_e531_d_b0;
        var_argbvvt_db1 = assign350_e531_d_b1;
        var_argbvvt_db2 = assign350_e531_d_b2;
        var_argbvvt_db3 = assign350_e531_d_b3;
        var_argbvvt_db4 = assign350_e531_d_b4;
        var_argbvvt_db5 = assign350_e531_d_b5;
        var_argbvvt_db6 = assign350_e531_d_b6;
        var_argbvvt_rv = 0.0;
        var_argbvvt_rdn0 = 0.0;
        var_argbvvt_rdn1 = 0.0;
        var_argbvvt_rdn2 = 0.0;
        var_argbvvt_rdn3 = 0.0;
        var_argbvvt_rdn4 = 0.0;
        var_argbvvt_rdn5 = 0.0;
        var_argbvvt_rdn6 = 0.0;
        var_argbvvt_rdb0 = 0.0;
        var_argbvvt_rdb1 = 0.0;
        var_argbvvt_rdb2 = 0.0;
        var_argbvvt_rdb3 = 0.0;
        var_argbvvt_rdb4 = 0.0;
        var_argbvvt_rdb5 = 0.0;
        var_argbvvt_rdb6 = 0.0;

        let assign360_e534: f64 = if var_arg > 80.0 { 1.0 } else { 0.0 };
        var_guard4 = assign360_e534;
        var_guard4_dn0 = 0.0;
        var_guard4_dn1 = 0.0;
        var_guard4_dn2 = 0.0;
        var_guard4_dn3 = 0.0;
        var_guard4_dn4 = 0.0;
        var_guard4_dn5 = 0.0;
        var_guard4_dn6 = 0.0;
        var_guard4_db0 = 0.0;
        var_guard4_db1 = 0.0;
        var_guard4_db2 = 0.0;
        var_guard4_db3 = 0.0;
        var_guard4_db4 = 0.0;
        var_guard4_db5 = 0.0;
        var_guard4_db6 = 0.0;
        var_guard4_rv = 0.0;
        var_guard4_rdn0 = 0.0;
        var_guard4_rdn1 = 0.0;
        var_guard4_rdn2 = 0.0;
        var_guard4_rdn3 = 0.0;
        var_guard4_rdn4 = 0.0;
        var_guard4_rdn5 = 0.0;
        var_guard4_rdn6 = 0.0;
        var_guard4_rdb0 = 0.0;
        var_guard4_rdb1 = 0.0;
        var_guard4_rdb2 = 0.0;
        var_guard4_rdb3 = 0.0;
        var_guard4_rdb4 = 0.0;
        var_guard4_rdb5 = 0.0;
        var_guard4_rdb6 = 0.0;

        let (assign370_e544, assign370_e544_d_n0, assign370_e544_d_n1, assign370_e544_d_n2, assign370_e544_d_n3, assign370_e544_d_n4, assign370_e544_d_n5, assign370_e544_d_n6, assign370_e544_d_b0, assign370_e544_d_b1, assign370_e544_d_b2, assign370_e544_d_b3, assign370_e544_d_b4, assign370_e544_d_b5, assign370_e544_d_b6,) = {
    if ((var_guard3 != 0.0) && (var_guard4 != 0.0)) {
        let assign370_e541: f64 = (var_arg - 80.0);
        let assign370_e542: f64 = (1.0 + assign370_e541);
        (assign370_e542, var_arg_dn0, var_arg_dn1, var_arg_dn2, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_db0, var_arg_db1, var_arg_db2, var_arg_db3, var_arg_db4, var_arg_db5, var_arg_db6,)
    } else {
        (var_le, var_le_dn0, var_le_dn1, var_le_dn2, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6, var_le_db0, var_le_db1, var_le_db2, var_le_db3, var_le_db4, var_le_db5, var_le_db6,)
    }
};
        var_le = assign370_e544;
        var_le_dn0 = assign370_e544_d_n0;
        var_le_dn1 = assign370_e544_d_n1;
        var_le_dn2 = assign370_e544_d_n2;
        var_le_dn3 = assign370_e544_d_n3;
        var_le_dn4 = assign370_e544_d_n4;
        var_le_dn5 = assign370_e544_d_n5;
        var_le_dn6 = assign370_e544_d_n6;
        var_le_db0 = assign370_e544_d_b0;
        var_le_db1 = assign370_e544_d_b1;
        var_le_db2 = assign370_e544_d_b2;
        var_le_db3 = assign370_e544_d_b3;
        var_le_db4 = assign370_e544_d_b4;
        var_le_db5 = assign370_e544_d_b5;
        var_le_db6 = assign370_e544_d_b6;
        var_le_rv = 0.0;
        var_le_rdn0 = 0.0;
        var_le_rdn1 = 0.0;
        var_le_rdn2 = 0.0;
        var_le_rdn3 = 0.0;
        var_le_rdn4 = 0.0;
        var_le_rdn5 = 0.0;
        var_le_rdn6 = 0.0;
        var_le_rdb0 = 0.0;
        var_le_rdb1 = 0.0;
        var_le_rdb2 = 0.0;
        var_le_rdb3 = 0.0;
        var_le_rdb4 = 0.0;
        var_le_rdb5 = 0.0;
        var_le_rdb6 = 0.0;

        let (assign380_e550, assign380_e550_d_n0, assign380_e550_d_n1, assign380_e550_d_n2, assign380_e550_d_n3, assign380_e550_d_n4, assign380_e550_d_n5, assign380_e550_d_n6, assign380_e550_d_b0, assign380_e550_d_b1, assign380_e550_d_b2, assign380_e550_d_b3, assign380_e550_d_b4, assign380_e550_d_b5, assign380_e550_d_b6,) = {
    if ((var_guard3 != 0.0) && (var_guard4 != 0.0)) {
        (80.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn1, var_arg_dn2, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_db0, var_arg_db1, var_arg_db2, var_arg_db3, var_arg_db4, var_arg_db5, var_arg_db6,)
    }
};
        var_arg = assign380_e550;
        var_arg_dn0 = assign380_e550_d_n0;
        var_arg_dn1 = assign380_e550_d_n1;
        var_arg_dn2 = assign380_e550_d_n2;
        var_arg_dn3 = assign380_e550_d_n3;
        var_arg_dn4 = assign380_e550_d_n4;
        var_arg_dn5 = assign380_e550_d_n5;
        var_arg_dn6 = assign380_e550_d_n6;
        var_arg_db0 = assign380_e550_d_b0;
        var_arg_db1 = assign380_e550_d_b1;
        var_arg_db2 = assign380_e550_d_b2;
        var_arg_db3 = assign380_e550_d_b3;
        var_arg_db4 = assign380_e550_d_b4;
        var_arg_db5 = assign380_e550_d_b5;
        var_arg_db6 = assign380_e550_d_b6;
        var_arg_rv = 0.0;
        var_arg_rdn0 = 0.0;
        var_arg_rdn1 = 0.0;
        var_arg_rdn2 = 0.0;
        var_arg_rdn3 = 0.0;
        var_arg_rdn4 = 0.0;
        var_arg_rdn5 = 0.0;
        var_arg_rdn6 = 0.0;
        var_arg_rdb0 = 0.0;
        var_arg_rdb1 = 0.0;
        var_arg_rdb2 = 0.0;
        var_arg_rdb3 = 0.0;
        var_arg_rdb4 = 0.0;
        var_arg_rdb5 = 0.0;
        var_arg_rdb6 = 0.0;

        let (assign390_e557, assign390_e557_d_n0, assign390_e557_d_n1, assign390_e557_d_n2, assign390_e557_d_n3, assign390_e557_d_n4, assign390_e557_d_n5, assign390_e557_d_n6, assign390_e557_d_b0, assign390_e557_d_b1, assign390_e557_d_b2, assign390_e557_d_b3, assign390_e557_d_b4, assign390_e557_d_b5, assign390_e557_d_b6,) = {
    if ((var_guard3 != 0.0) && (var_guard4 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_le, var_le_dn0, var_le_dn1, var_le_dn2, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6, var_le_db0, var_le_db1, var_le_db2, var_le_db3, var_le_db4, var_le_db5, var_le_db6,)
    }
};
        var_le = assign390_e557;
        var_le_dn0 = assign390_e557_d_n0;
        var_le_dn1 = assign390_e557_d_n1;
        var_le_dn2 = assign390_e557_d_n2;
        var_le_dn3 = assign390_e557_d_n3;
        var_le_dn4 = assign390_e557_d_n4;
        var_le_dn5 = assign390_e557_d_n5;
        var_le_dn6 = assign390_e557_d_n6;
        var_le_db0 = assign390_e557_d_b0;
        var_le_db1 = assign390_e557_d_b1;
        var_le_db2 = assign390_e557_d_b2;
        var_le_db3 = assign390_e557_d_b3;
        var_le_db4 = assign390_e557_d_b4;
        var_le_db5 = assign390_e557_d_b5;
        var_le_db6 = assign390_e557_d_b6;
        var_le_rv = 0.0;
        var_le_rdn0 = 0.0;
        var_le_rdn1 = 0.0;
        var_le_rdn2 = 0.0;
        var_le_rdn3 = 0.0;
        var_le_rdn4 = 0.0;
        var_le_rdn5 = 0.0;
        var_le_rdn6 = 0.0;
        var_le_rdb0 = 0.0;
        var_le_rdb1 = 0.0;
        var_le_rdb2 = 0.0;
        var_le_rdb3 = 0.0;
        var_le_rdb4 = 0.0;
        var_le_rdb5 = 0.0;
        var_le_rdb6 = 0.0;

        let (assign400_e564, assign400_e564_d_n0, assign400_e564_d_n1, assign400_e564_d_n2, assign400_e564_d_n3, assign400_e564_d_n4, assign400_e564_d_n5, assign400_e564_d_n6, assign400_e564_d_b0, assign400_e564_d_b1, assign400_e564_d_b2, assign400_e564_d_b3, assign400_e564_d_b4, assign400_e564_d_b5, assign400_e564_d_b6,) = {
    if (var_guard3 != 0.0) {
        let assign400_e561: f64 = (var_arg).exp();
        let assign400_e562: f64 = (var_le * assign400_e561);
        (assign400_e562, ((var_le_dn0 * assign400_e561) + (var_le * (assign400_e561 * var_arg_dn0))), ((var_le_dn1 * assign400_e561) + (var_le * (assign400_e561 * var_arg_dn1))), ((var_le_dn2 * assign400_e561) + (var_le * (assign400_e561 * var_arg_dn2))), ((var_le_dn3 * assign400_e561) + (var_le * (assign400_e561 * var_arg_dn3))), ((var_le_dn4 * assign400_e561) + (var_le * (assign400_e561 * var_arg_dn4))), ((var_le_dn5 * assign400_e561) + (var_le * (assign400_e561 * var_arg_dn5))), ((var_le_dn6 * assign400_e561) + (var_le * (assign400_e561 * var_arg_dn6))), ((var_le_db0 * assign400_e561) + (var_le * (assign400_e561 * var_arg_db0))), ((var_le_db1 * assign400_e561) + (var_le * (assign400_e561 * var_arg_db1))), ((var_le_db2 * assign400_e561) + (var_le * (assign400_e561 * var_arg_db2))), ((var_le_db3 * assign400_e561) + (var_le * (assign400_e561 * var_arg_db3))), ((var_le_db4 * assign400_e561) + (var_le * (assign400_e561 * var_arg_db4))), ((var_le_db5 * assign400_e561) + (var_le * (assign400_e561 * var_arg_db5))), ((var_le_db6 * assign400_e561) + (var_le * (assign400_e561 * var_arg_db6))),)
    } else {
        (var_le, var_le_dn0, var_le_dn1, var_le_dn2, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6, var_le_db0, var_le_db1, var_le_db2, var_le_db3, var_le_db4, var_le_db5, var_le_db6,)
    }
};
        var_le = assign400_e564;
        var_le_dn0 = assign400_e564_d_n0;
        var_le_dn1 = assign400_e564_d_n1;
        var_le_dn2 = assign400_e564_d_n2;
        var_le_dn3 = assign400_e564_d_n3;
        var_le_dn4 = assign400_e564_d_n4;
        var_le_dn5 = assign400_e564_d_n5;
        var_le_dn6 = assign400_e564_d_n6;
        var_le_db0 = assign400_e564_d_b0;
        var_le_db1 = assign400_e564_d_b1;
        var_le_db2 = assign400_e564_d_b2;
        var_le_db3 = assign400_e564_d_b3;
        var_le_db4 = assign400_e564_d_b4;
        var_le_db5 = assign400_e564_d_b5;
        var_le_db6 = assign400_e564_d_b6;
        var_le_rv = 0.0;
        var_le_rdn0 = 0.0;
        var_le_rdn1 = 0.0;
        var_le_rdn2 = 0.0;
        var_le_rdn3 = 0.0;
        var_le_rdn4 = 0.0;
        var_le_rdn5 = 0.0;
        var_le_rdn6 = 0.0;
        var_le_rdb0 = 0.0;
        var_le_rdb1 = 0.0;
        var_le_rdb2 = 0.0;
        var_le_rdb3 = 0.0;
        var_le_rdb4 = 0.0;
        var_le_rdb5 = 0.0;
        var_le_rdb6 = 0.0;

        let (assign410_e636, assign410_e636_d_n0, assign410_e636_d_n1, assign410_e636_d_n2, assign410_e636_d_n3, assign410_e636_d_n4, assign410_e636_d_n5, assign410_e636_d_n6, assign410_e636_d_b0, assign410_e636_d_b1, assign410_e636_d_b2, assign410_e636_d_b3, assign410_e636_d_b4, assign410_e636_d_b5, assign410_e636_d_b6,) = {
    if (var_guard3 != 0.0) {
        let assign410_e572: f64 = (-37.0);
        let (assign410_e599, assign410_e599_d_n0, assign410_e599_d_n1, assign410_e599_d_n2, assign410_e599_d_n3, assign410_e599_d_n4, assign410_e599_d_n5, assign410_e599_d_n6, assign410_e599_d_b0, assign410_e599_d_b1, assign410_e599_d_b2, assign410_e599_d_b3, assign410_e599_d_b4, assign410_e599_d_b5, assign410_e599_d_b6,) = {
            if ((!(var_argbv >= 37.0)) && (!(var_argbv <= assign410_e572))) {
                let assign410_e577: f64 = (var_argbv).exp();
                let assign410_e579: f64 = (assign410_e577 + 1.0);
                let assign410_e580: f64 = (assign410_e579).ln();
                (assign410_e580, ((assign410_e577 * var_argbv_dn0) / assign410_e579), ((assign410_e577 * var_argbv_dn1) / assign410_e579), ((assign410_e577 * var_argbv_dn2) / assign410_e579), ((assign410_e577 * var_argbv_dn3) / assign410_e579), ((assign410_e577 * var_argbv_dn4) / assign410_e579), ((assign410_e577 * var_argbv_dn5) / assign410_e579), ((assign410_e577 * var_argbv_dn6) / assign410_e579), ((assign410_e577 * var_argbv_db0) / assign410_e579), ((assign410_e577 * var_argbv_db1) / assign410_e579), ((assign410_e577 * var_argbv_db2) / assign410_e579), ((assign410_e577 * var_argbv_db3) / assign410_e579), ((assign410_e577 * var_argbv_db4) / assign410_e579), ((assign410_e577 * var_argbv_db5) / assign410_e579), ((assign410_e577 * var_argbv_db6) / assign410_e579),)
            } else {
                let assign410_e587: f64 = (-37.0);
                let (assign410_e598, assign410_e598_d_n0, assign410_e598_d_n1, assign410_e598_d_n2, assign410_e598_d_n3, assign410_e598_d_n4, assign410_e598_d_n5, assign410_e598_d_n6, assign410_e598_d_b0, assign410_e598_d_b1, assign410_e598_d_b2, assign410_e598_d_b3, assign410_e598_d_b4, assign410_e598_d_b5, assign410_e598_d_b6,) = {
                    if ((!(var_argbv >= 37.0)) && (var_argbv <= assign410_e587)) {
                        let assign410_e591: f64 = (var_argbv).exp();
                        (assign410_e591, (assign410_e591 * var_argbv_dn0), (assign410_e591 * var_argbv_dn1), (assign410_e591 * var_argbv_dn2), (assign410_e591 * var_argbv_dn3), (assign410_e591 * var_argbv_dn4), (assign410_e591 * var_argbv_dn5), (assign410_e591 * var_argbv_dn6), (assign410_e591 * var_argbv_db0), (assign410_e591 * var_argbv_db1), (assign410_e591 * var_argbv_db2), (assign410_e591 * var_argbv_db3), (assign410_e591 * var_argbv_db4), (assign410_e591 * var_argbv_db5), (assign410_e591 * var_argbv_db6),)
                    } else {
                        let (assign410_e597, assign410_e597_d_n0, assign410_e597_d_n1, assign410_e597_d_n2, assign410_e597_d_n3, assign410_e597_d_n4, assign410_e597_d_n5, assign410_e597_d_n6, assign410_e597_d_b0, assign410_e597_d_b1, assign410_e597_d_b2, assign410_e597_d_b3, assign410_e597_d_b4, assign410_e597_d_b5, assign410_e597_d_b6,) = {
                            if (var_argbv >= 37.0) {
                                (var_argbv, var_argbv_dn0, var_argbv_dn1, var_argbv_dn2, var_argbv_dn3, var_argbv_dn4, var_argbv_dn5, var_argbv_dn6, var_argbv_db0, var_argbv_db1, var_argbv_db2, var_argbv_db3, var_argbv_db4, var_argbv_db5, var_argbv_db6,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign410_e597, assign410_e597_d_n0, assign410_e597_d_n1, assign410_e597_d_n2, assign410_e597_d_n3, assign410_e597_d_n4, assign410_e597_d_n5, assign410_e597_d_n6, assign410_e597_d_b0, assign410_e597_d_b1, assign410_e597_d_b2, assign410_e597_d_b3, assign410_e597_d_b4, assign410_e597_d_b5, assign410_e597_d_b6,)
                    }
                };
                (assign410_e598, assign410_e598_d_n0, assign410_e598_d_n1, assign410_e598_d_n2, assign410_e598_d_n3, assign410_e598_d_n4, assign410_e598_d_n5, assign410_e598_d_n6, assign410_e598_d_b0, assign410_e598_d_b1, assign410_e598_d_b2, assign410_e598_d_b3, assign410_e598_d_b4, assign410_e598_d_b5, assign410_e598_d_b6,)
            }
        };
        let assign410_e606: f64 = (-37.0);
        let (assign410_e633, assign410_e633_d_n0, assign410_e633_d_n1, assign410_e633_d_n2, assign410_e633_d_n3, assign410_e633_d_n4, assign410_e633_d_n5, assign410_e633_d_n6, assign410_e633_d_b0, assign410_e633_d_b1, assign410_e633_d_b2, assign410_e633_d_b3, assign410_e633_d_b4, assign410_e633_d_b5, assign410_e633_d_b6,) = {
            if ((!(var_argbvvt >= 37.0)) && (!(var_argbvvt <= assign410_e606))) {
                let assign410_e611: f64 = (var_argbvvt).exp();
                let assign410_e613: f64 = (assign410_e611 + 1.0);
                let assign410_e614: f64 = (assign410_e613).ln();
                (assign410_e614, ((assign410_e611 * var_argbvvt_dn0) / assign410_e613), ((assign410_e611 * var_argbvvt_dn1) / assign410_e613), ((assign410_e611 * var_argbvvt_dn2) / assign410_e613), ((assign410_e611 * var_argbvvt_dn3) / assign410_e613), ((assign410_e611 * var_argbvvt_dn4) / assign410_e613), ((assign410_e611 * var_argbvvt_dn5) / assign410_e613), ((assign410_e611 * var_argbvvt_dn6) / assign410_e613), ((assign410_e611 * var_argbvvt_db0) / assign410_e613), ((assign410_e611 * var_argbvvt_db1) / assign410_e613), ((assign410_e611 * var_argbvvt_db2) / assign410_e613), ((assign410_e611 * var_argbvvt_db3) / assign410_e613), ((assign410_e611 * var_argbvvt_db4) / assign410_e613), ((assign410_e611 * var_argbvvt_db5) / assign410_e613), ((assign410_e611 * var_argbvvt_db6) / assign410_e613),)
            } else {
                let assign410_e621: f64 = (-37.0);
                let (assign410_e632, assign410_e632_d_n0, assign410_e632_d_n1, assign410_e632_d_n2, assign410_e632_d_n3, assign410_e632_d_n4, assign410_e632_d_n5, assign410_e632_d_n6, assign410_e632_d_b0, assign410_e632_d_b1, assign410_e632_d_b2, assign410_e632_d_b3, assign410_e632_d_b4, assign410_e632_d_b5, assign410_e632_d_b6,) = {
                    if ((!(var_argbvvt >= 37.0)) && (var_argbvvt <= assign410_e621)) {
                        let assign410_e625: f64 = (var_argbvvt).exp();
                        (assign410_e625, (assign410_e625 * var_argbvvt_dn0), (assign410_e625 * var_argbvvt_dn1), (assign410_e625 * var_argbvvt_dn2), (assign410_e625 * var_argbvvt_dn3), (assign410_e625 * var_argbvvt_dn4), (assign410_e625 * var_argbvvt_dn5), (assign410_e625 * var_argbvvt_dn6), (assign410_e625 * var_argbvvt_db0), (assign410_e625 * var_argbvvt_db1), (assign410_e625 * var_argbvvt_db2), (assign410_e625 * var_argbvvt_db3), (assign410_e625 * var_argbvvt_db4), (assign410_e625 * var_argbvvt_db5), (assign410_e625 * var_argbvvt_db6),)
                    } else {
                        let (assign410_e631, assign410_e631_d_n0, assign410_e631_d_n1, assign410_e631_d_n2, assign410_e631_d_n3, assign410_e631_d_n4, assign410_e631_d_n5, assign410_e631_d_n6, assign410_e631_d_b0, assign410_e631_d_b1, assign410_e631_d_b2, assign410_e631_d_b3, assign410_e631_d_b4, assign410_e631_d_b5, assign410_e631_d_b6,) = {
                            if (var_argbvvt >= 37.0) {
                                (var_argbvvt, var_argbvvt_dn0, var_argbvvt_dn1, var_argbvvt_dn2, var_argbvvt_dn3, var_argbvvt_dn4, var_argbvvt_dn5, var_argbvvt_dn6, var_argbvvt_db0, var_argbvvt_db1, var_argbvvt_db2, var_argbvvt_db3, var_argbvvt_db4, var_argbvvt_db5, var_argbvvt_db6,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign410_e631, assign410_e631_d_n0, assign410_e631_d_n1, assign410_e631_d_n2, assign410_e631_d_n3, assign410_e631_d_n4, assign410_e631_d_n5, assign410_e631_d_n6, assign410_e631_d_b0, assign410_e631_d_b1, assign410_e631_d_b2, assign410_e631_d_b3, assign410_e631_d_b4, assign410_e631_d_b5, assign410_e631_d_b6,)
                    }
                };
                (assign410_e632, assign410_e632_d_n0, assign410_e632_d_n1, assign410_e632_d_n2, assign410_e632_d_n3, assign410_e632_d_n4, assign410_e632_d_n5, assign410_e632_d_n6, assign410_e632_d_b0, assign410_e632_d_b1, assign410_e632_d_b2, assign410_e632_d_b3, assign410_e632_d_b4, assign410_e632_d_b5, assign410_e632_d_b6,)
            }
        };
        let assign410_e634: f64 = (assign410_e599 - assign410_e633);
        (assign410_e634, (assign410_e599_d_n0 - assign410_e633_d_n0), (assign410_e599_d_n1 - assign410_e633_d_n1), (assign410_e599_d_n2 - assign410_e633_d_n2), (assign410_e599_d_n3 - assign410_e633_d_n3), (assign410_e599_d_n4 - assign410_e633_d_n4), (assign410_e599_d_n5 - assign410_e633_d_n5), (assign410_e599_d_n6 - assign410_e633_d_n6), (assign410_e599_d_b0 - assign410_e633_d_b0), (assign410_e599_d_b1 - assign410_e633_d_b1), (assign410_e599_d_b2 - assign410_e633_d_b2), (assign410_e599_d_b3 - assign410_e633_d_b3), (assign410_e599_d_b4 - assign410_e633_d_b4), (assign410_e599_d_b5 - assign410_e633_d_b5), (assign410_e599_d_b6 - assign410_e633_d_b6),)
    } else {
        (var_lebv, var_lebv_dn0, var_lebv_dn1, var_lebv_dn2, var_lebv_dn3, var_lebv_dn4, var_lebv_dn5, var_lebv_dn6, var_lebv_db0, var_lebv_db1, var_lebv_db2, var_lebv_db3, var_lebv_db4, var_lebv_db5, var_lebv_db6,)
    }
};
        var_lebv = assign410_e636;
        var_lebv_dn0 = assign410_e636_d_n0;
        var_lebv_dn1 = assign410_e636_d_n1;
        var_lebv_dn2 = assign410_e636_d_n2;
        var_lebv_dn3 = assign410_e636_d_n3;
        var_lebv_dn4 = assign410_e636_d_n4;
        var_lebv_dn5 = assign410_e636_d_n5;
        var_lebv_dn6 = assign410_e636_d_n6;
        var_lebv_db0 = assign410_e636_d_b0;
        var_lebv_db1 = assign410_e636_d_b1;
        var_lebv_db2 = assign410_e636_d_b2;
        var_lebv_db3 = assign410_e636_d_b3;
        var_lebv_db4 = assign410_e636_d_b4;
        var_lebv_db5 = assign410_e636_d_b5;
        var_lebv_db6 = assign410_e636_d_b6;
        var_lebv_rv = 0.0;
        var_lebv_rdn0 = 0.0;
        var_lebv_rdn1 = 0.0;
        var_lebv_rdn2 = 0.0;
        var_lebv_rdn3 = 0.0;
        var_lebv_rdn4 = 0.0;
        var_lebv_rdn5 = 0.0;
        var_lebv_rdn6 = 0.0;
        var_lebv_rdb0 = 0.0;
        var_lebv_rdb1 = 0.0;
        var_lebv_rdb2 = 0.0;
        var_lebv_rdb3 = 0.0;
        var_lebv_rdb4 = 0.0;
        var_lebv_rdb5 = 0.0;
        var_lebv_rdb6 = 0.0;

        let (assign420_e657, assign420_e657_d_n0, assign420_e657_d_n1, assign420_e657_d_n2, assign420_e657_d_n3, assign420_e657_d_n4, assign420_e657_d_n5, assign420_e657_d_n6, assign420_e657_d_b0, assign420_e657_d_b1, assign420_e657_d_b2, assign420_e657_d_b3, assign420_e657_d_b4, assign420_e657_d_b5, assign420_e657_d_b6,) = {
    if (var_guard3 != 0.0) {
        let assign420_e641: f64 = (var_le - 1.0);
        let assign420_e642: f64 = (var_is_t * assign420_e641);
        let assign420_e645: f64 = (var_ijbv_t * var_lebv);
        let assign420_e649: f64 = (var_vbiei).abs();
        let assign420_e651: f64 = (assign420_e649).powf(var_theexp_t);
        let assign420_e652: f64 = (p.p8 * assign420_e651);
        let assign420_e653: f64 = (1.0 + assign420_e652);
        let assign420_e654: f64 = (assign420_e645 / assign420_e653);
        let assign420_e655: f64 = (assign420_e642 - assign420_e654);
        (assign420_e655, (((var_is_t_dn0 * assign420_e641) + (var_is_t * var_le_dn0)) - (((((var_ijbv_t_dn0 * var_lebv) + (var_ijbv_t * var_lebv_dn0)) * assign420_e653) - (assign420_e645 * (p.p8 * if var_theexp_t_dn0 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign420_e649).powf(var_theexp_t - 1.0) * if var_vbiei >= 0.0 { var_vbiei_dn0 } else { (-var_vbiei_dn0) })) } } else { (assign420_e651 * ((var_theexp_t_dn0 * (assign420_e649).ln()) + (var_theexp_t * (if var_vbiei >= 0.0 { var_vbiei_dn0 } else { (-var_vbiei_dn0) } / assign420_e649)))) }))) / (assign420_e653 * assign420_e653))), (((var_is_t_dn1 * assign420_e641) + (var_is_t * var_le_dn1)) - (((((var_ijbv_t_dn1 * var_lebv) + (var_ijbv_t * var_lebv_dn1)) * assign420_e653) - (assign420_e645 * (p.p8 * if var_theexp_t_dn1 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign420_e649).powf(var_theexp_t - 1.0) * if var_vbiei >= 0.0 { var_vbiei_dn1 } else { (-var_vbiei_dn1) })) } } else { (assign420_e651 * ((var_theexp_t_dn1 * (assign420_e649).ln()) + (var_theexp_t * (if var_vbiei >= 0.0 { var_vbiei_dn1 } else { (-var_vbiei_dn1) } / assign420_e649)))) }))) / (assign420_e653 * assign420_e653))), (((var_is_t_dn2 * assign420_e641) + (var_is_t * var_le_dn2)) - (((((var_ijbv_t_dn2 * var_lebv) + (var_ijbv_t * var_lebv_dn2)) * assign420_e653) - (assign420_e645 * (p.p8 * if var_theexp_t_dn2 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign420_e649).powf(var_theexp_t - 1.0) * if var_vbiei >= 0.0 { var_vbiei_dn2 } else { (-var_vbiei_dn2) })) } } else { (assign420_e651 * ((var_theexp_t_dn2 * (assign420_e649).ln()) + (var_theexp_t * (if var_vbiei >= 0.0 { var_vbiei_dn2 } else { (-var_vbiei_dn2) } / assign420_e649)))) }))) / (assign420_e653 * assign420_e653))), (((var_is_t_dn3 * assign420_e641) + (var_is_t * var_le_dn3)) - (((((var_ijbv_t_dn3 * var_lebv) + (var_ijbv_t * var_lebv_dn3)) * assign420_e653) - (assign420_e645 * (p.p8 * if var_theexp_t_dn3 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign420_e649).powf(var_theexp_t - 1.0) * if var_vbiei >= 0.0 { var_vbiei_dn3 } else { (-var_vbiei_dn3) })) } } else { (assign420_e651 * ((var_theexp_t_dn3 * (assign420_e649).ln()) + (var_theexp_t * (if var_vbiei >= 0.0 { var_vbiei_dn3 } else { (-var_vbiei_dn3) } / assign420_e649)))) }))) / (assign420_e653 * assign420_e653))), (((var_is_t_dn4 * assign420_e641) + (var_is_t * var_le_dn4)) - (((((var_ijbv_t_dn4 * var_lebv) + (var_ijbv_t * var_lebv_dn4)) * assign420_e653) - (assign420_e645 * (p.p8 * if var_theexp_t_dn4 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign420_e649).powf(var_theexp_t - 1.0) * if var_vbiei >= 0.0 { var_vbiei_dn4 } else { (-var_vbiei_dn4) })) } } else { (assign420_e651 * ((var_theexp_t_dn4 * (assign420_e649).ln()) + (var_theexp_t * (if var_vbiei >= 0.0 { var_vbiei_dn4 } else { (-var_vbiei_dn4) } / assign420_e649)))) }))) / (assign420_e653 * assign420_e653))), (((var_is_t_dn5 * assign420_e641) + (var_is_t * var_le_dn5)) - (((((var_ijbv_t_dn5 * var_lebv) + (var_ijbv_t * var_lebv_dn5)) * assign420_e653) - (assign420_e645 * (p.p8 * if var_theexp_t_dn5 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign420_e649).powf(var_theexp_t - 1.0) * if var_vbiei >= 0.0 { var_vbiei_dn5 } else { (-var_vbiei_dn5) })) } } else { (assign420_e651 * ((var_theexp_t_dn5 * (assign420_e649).ln()) + (var_theexp_t * (if var_vbiei >= 0.0 { var_vbiei_dn5 } else { (-var_vbiei_dn5) } / assign420_e649)))) }))) / (assign420_e653 * assign420_e653))), (((var_is_t_dn6 * assign420_e641) + (var_is_t * var_le_dn6)) - (((((var_ijbv_t_dn6 * var_lebv) + (var_ijbv_t * var_lebv_dn6)) * assign420_e653) - (assign420_e645 * (p.p8 * if var_theexp_t_dn6 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign420_e649).powf(var_theexp_t - 1.0) * if var_vbiei >= 0.0 { var_vbiei_dn6 } else { (-var_vbiei_dn6) })) } } else { (assign420_e651 * ((var_theexp_t_dn6 * (assign420_e649).ln()) + (var_theexp_t * (if var_vbiei >= 0.0 { var_vbiei_dn6 } else { (-var_vbiei_dn6) } / assign420_e649)))) }))) / (assign420_e653 * assign420_e653))), (((var_is_t_db0 * assign420_e641) + (var_is_t * var_le_db0)) - (((((var_ijbv_t_db0 * var_lebv) + (var_ijbv_t * var_lebv_db0)) * assign420_e653) - (assign420_e645 * (p.p8 * if var_theexp_t_db0 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign420_e649).powf(var_theexp_t - 1.0) * if var_vbiei >= 0.0 { var_vbiei_db0 } else { (-var_vbiei_db0) })) } } else { (assign420_e651 * ((var_theexp_t_db0 * (assign420_e649).ln()) + (var_theexp_t * (if var_vbiei >= 0.0 { var_vbiei_db0 } else { (-var_vbiei_db0) } / assign420_e649)))) }))) / (assign420_e653 * assign420_e653))), (((var_is_t_db1 * assign420_e641) + (var_is_t * var_le_db1)) - (((((var_ijbv_t_db1 * var_lebv) + (var_ijbv_t * var_lebv_db1)) * assign420_e653) - (assign420_e645 * (p.p8 * if var_theexp_t_db1 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign420_e649).powf(var_theexp_t - 1.0) * if var_vbiei >= 0.0 { var_vbiei_db1 } else { (-var_vbiei_db1) })) } } else { (assign420_e651 * ((var_theexp_t_db1 * (assign420_e649).ln()) + (var_theexp_t * (if var_vbiei >= 0.0 { var_vbiei_db1 } else { (-var_vbiei_db1) } / assign420_e649)))) }))) / (assign420_e653 * assign420_e653))), (((var_is_t_db2 * assign420_e641) + (var_is_t * var_le_db2)) - (((((var_ijbv_t_db2 * var_lebv) + (var_ijbv_t * var_lebv_db2)) * assign420_e653) - (assign420_e645 * (p.p8 * if var_theexp_t_db2 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign420_e649).powf(var_theexp_t - 1.0) * if var_vbiei >= 0.0 { var_vbiei_db2 } else { (-var_vbiei_db2) })) } } else { (assign420_e651 * ((var_theexp_t_db2 * (assign420_e649).ln()) + (var_theexp_t * (if var_vbiei >= 0.0 { var_vbiei_db2 } else { (-var_vbiei_db2) } / assign420_e649)))) }))) / (assign420_e653 * assign420_e653))), (((var_is_t_db3 * assign420_e641) + (var_is_t * var_le_db3)) - (((((var_ijbv_t_db3 * var_lebv) + (var_ijbv_t * var_lebv_db3)) * assign420_e653) - (assign420_e645 * (p.p8 * if var_theexp_t_db3 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign420_e649).powf(var_theexp_t - 1.0) * if var_vbiei >= 0.0 { var_vbiei_db3 } else { (-var_vbiei_db3) })) } } else { (assign420_e651 * ((var_theexp_t_db3 * (assign420_e649).ln()) + (var_theexp_t * (if var_vbiei >= 0.0 { var_vbiei_db3 } else { (-var_vbiei_db3) } / assign420_e649)))) }))) / (assign420_e653 * assign420_e653))), (((var_is_t_db4 * assign420_e641) + (var_is_t * var_le_db4)) - (((((var_ijbv_t_db4 * var_lebv) + (var_ijbv_t * var_lebv_db4)) * assign420_e653) - (assign420_e645 * (p.p8 * if var_theexp_t_db4 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign420_e649).powf(var_theexp_t - 1.0) * if var_vbiei >= 0.0 { var_vbiei_db4 } else { (-var_vbiei_db4) })) } } else { (assign420_e651 * ((var_theexp_t_db4 * (assign420_e649).ln()) + (var_theexp_t * (if var_vbiei >= 0.0 { var_vbiei_db4 } else { (-var_vbiei_db4) } / assign420_e649)))) }))) / (assign420_e653 * assign420_e653))), (((var_is_t_db5 * assign420_e641) + (var_is_t * var_le_db5)) - (((((var_ijbv_t_db5 * var_lebv) + (var_ijbv_t * var_lebv_db5)) * assign420_e653) - (assign420_e645 * (p.p8 * if var_theexp_t_db5 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign420_e649).powf(var_theexp_t - 1.0) * if var_vbiei >= 0.0 { var_vbiei_db5 } else { (-var_vbiei_db5) })) } } else { (assign420_e651 * ((var_theexp_t_db5 * (assign420_e649).ln()) + (var_theexp_t * (if var_vbiei >= 0.0 { var_vbiei_db5 } else { (-var_vbiei_db5) } / assign420_e649)))) }))) / (assign420_e653 * assign420_e653))), (((var_is_t_db6 * assign420_e641) + (var_is_t * var_le_db6)) - (((((var_ijbv_t_db6 * var_lebv) + (var_ijbv_t * var_lebv_db6)) * assign420_e653) - (assign420_e645 * (p.p8 * if var_theexp_t_db6 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign420_e649).powf(var_theexp_t - 1.0) * if var_vbiei >= 0.0 { var_vbiei_db6 } else { (-var_vbiei_db6) })) } } else { (assign420_e651 * ((var_theexp_t_db6 * (assign420_e649).ln()) + (var_theexp_t * (if var_vbiei >= 0.0 { var_vbiei_db6 } else { (-var_vbiei_db6) } / assign420_e649)))) }))) / (assign420_e653 * assign420_e653))),)
    } else {
        (var_ifwd, var_ifwd_dn0, var_ifwd_dn1, var_ifwd_dn2, var_ifwd_dn3, var_ifwd_dn4, var_ifwd_dn5, var_ifwd_dn6, var_ifwd_db0, var_ifwd_db1, var_ifwd_db2, var_ifwd_db3, var_ifwd_db4, var_ifwd_db5, var_ifwd_db6,)
    }
};
        var_ifwd = assign420_e657;
        var_ifwd_dn0 = assign420_e657_d_n0;
        var_ifwd_dn1 = assign420_e657_d_n1;
        var_ifwd_dn2 = assign420_e657_d_n2;
        var_ifwd_dn3 = assign420_e657_d_n3;
        var_ifwd_dn4 = assign420_e657_d_n4;
        var_ifwd_dn5 = assign420_e657_d_n5;
        var_ifwd_dn6 = assign420_e657_d_n6;
        var_ifwd_db0 = assign420_e657_d_b0;
        var_ifwd_db1 = assign420_e657_d_b1;
        var_ifwd_db2 = assign420_e657_d_b2;
        var_ifwd_db3 = assign420_e657_d_b3;
        var_ifwd_db4 = assign420_e657_d_b4;
        var_ifwd_db5 = assign420_e657_d_b5;
        var_ifwd_db6 = assign420_e657_d_b6;
        var_ifwd_rv = 0.0;
        var_ifwd_rdn0 = 0.0;
        var_ifwd_rdn1 = 0.0;
        var_ifwd_rdn2 = 0.0;
        var_ifwd_rdn3 = 0.0;
        var_ifwd_rdn4 = 0.0;
        var_ifwd_rdn5 = 0.0;
        var_ifwd_rdn6 = 0.0;
        var_ifwd_rdb0 = 0.0;
        var_ifwd_rdb1 = 0.0;
        var_ifwd_rdb2 = 0.0;
        var_ifwd_rdb3 = 0.0;
        var_ifwd_rdb4 = 0.0;
        var_ifwd_rdb5 = 0.0;
        var_ifwd_rdb6 = 0.0;

        let (assign430_e662, assign430_e662_d_n0, assign430_e662_d_n1, assign430_e662_d_n2, assign430_e662_d_n3, assign430_e662_d_n4, assign430_e662_d_n5, assign430_e662_d_n6, assign430_e662_d_b0, assign430_e662_d_b1, assign430_e662_d_b2, assign430_e662_d_b3, assign430_e662_d_b4, assign430_e662_d_b5, assign430_e662_d_b6,) = {
    if (var_guard3 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ifwd, var_ifwd_dn0, var_ifwd_dn1, var_ifwd_dn2, var_ifwd_dn3, var_ifwd_dn4, var_ifwd_dn5, var_ifwd_dn6, var_ifwd_db0, var_ifwd_db1, var_ifwd_db2, var_ifwd_db3, var_ifwd_db4, var_ifwd_db5, var_ifwd_db6,)
    }
};
        var_ifwd = assign430_e662;
        var_ifwd_dn0 = assign430_e662_d_n0;
        var_ifwd_dn1 = assign430_e662_d_n1;
        var_ifwd_dn2 = assign430_e662_d_n2;
        var_ifwd_dn3 = assign430_e662_d_n3;
        var_ifwd_dn4 = assign430_e662_d_n4;
        var_ifwd_dn5 = assign430_e662_d_n5;
        var_ifwd_dn6 = assign430_e662_d_n6;
        var_ifwd_db0 = assign430_e662_d_b0;
        var_ifwd_db1 = assign430_e662_d_b1;
        var_ifwd_db2 = assign430_e662_d_b2;
        var_ifwd_db3 = assign430_e662_d_b3;
        var_ifwd_db4 = assign430_e662_d_b4;
        var_ifwd_db5 = assign430_e662_d_b5;
        var_ifwd_db6 = assign430_e662_d_b6;
        var_ifwd_rv = 0.0;
        var_ifwd_rdn0 = 0.0;
        var_ifwd_rdn1 = 0.0;
        var_ifwd_rdn2 = 0.0;
        var_ifwd_rdn3 = 0.0;
        var_ifwd_rdn4 = 0.0;
        var_ifwd_rdn5 = 0.0;
        var_ifwd_rdn6 = 0.0;
        var_ifwd_rdb0 = 0.0;
        var_ifwd_rdb1 = 0.0;
        var_ifwd_rdb2 = 0.0;
        var_ifwd_rdb3 = 0.0;
        var_ifwd_rdb4 = 0.0;
        var_ifwd_rdb5 = 0.0;
        var_ifwd_rdb6 = 0.0;

        *var_arg_slot = var_arg;
        *var_arg_db0_slot = var_arg_db0;
        *var_arg_db1_slot = var_arg_db1;
        *var_arg_db2_slot = var_arg_db2;
        *var_arg_db3_slot = var_arg_db3;
        *var_arg_db4_slot = var_arg_db4;
        *var_arg_db5_slot = var_arg_db5;
        *var_arg_db6_slot = var_arg_db6;
        *var_arg_dn0_slot = var_arg_dn0;
        *var_arg_dn1_slot = var_arg_dn1;
        *var_arg_dn2_slot = var_arg_dn2;
        *var_arg_dn3_slot = var_arg_dn3;
        *var_arg_dn4_slot = var_arg_dn4;
        *var_arg_dn5_slot = var_arg_dn5;
        *var_arg_dn6_slot = var_arg_dn6;
        *var_arg_rdb0_slot = var_arg_rdb0;
        *var_arg_rdb1_slot = var_arg_rdb1;
        *var_arg_rdb2_slot = var_arg_rdb2;
        *var_arg_rdb3_slot = var_arg_rdb3;
        *var_arg_rdb4_slot = var_arg_rdb4;
        *var_arg_rdb5_slot = var_arg_rdb5;
        *var_arg_rdb6_slot = var_arg_rdb6;
        *var_arg_rdn0_slot = var_arg_rdn0;
        *var_arg_rdn1_slot = var_arg_rdn1;
        *var_arg_rdn2_slot = var_arg_rdn2;
        *var_arg_rdn3_slot = var_arg_rdn3;
        *var_arg_rdn4_slot = var_arg_rdn4;
        *var_arg_rdn5_slot = var_arg_rdn5;
        *var_arg_rdn6_slot = var_arg_rdn6;
        *var_arg_rv_slot = var_arg_rv;
        *var_argbv_slot = var_argbv;
        *var_argbv_db0_slot = var_argbv_db0;
        *var_argbv_db1_slot = var_argbv_db1;
        *var_argbv_db2_slot = var_argbv_db2;
        *var_argbv_db3_slot = var_argbv_db3;
        *var_argbv_db4_slot = var_argbv_db4;
        *var_argbv_db5_slot = var_argbv_db5;
        *var_argbv_db6_slot = var_argbv_db6;
        *var_argbv_dn0_slot = var_argbv_dn0;
        *var_argbv_dn1_slot = var_argbv_dn1;
        *var_argbv_dn2_slot = var_argbv_dn2;
        *var_argbv_dn3_slot = var_argbv_dn3;
        *var_argbv_dn4_slot = var_argbv_dn4;
        *var_argbv_dn5_slot = var_argbv_dn5;
        *var_argbv_dn6_slot = var_argbv_dn6;
        *var_argbv_rdb0_slot = var_argbv_rdb0;
        *var_argbv_rdb1_slot = var_argbv_rdb1;
        *var_argbv_rdb2_slot = var_argbv_rdb2;
        *var_argbv_rdb3_slot = var_argbv_rdb3;
        *var_argbv_rdb4_slot = var_argbv_rdb4;
        *var_argbv_rdb5_slot = var_argbv_rdb5;
        *var_argbv_rdb6_slot = var_argbv_rdb6;
        *var_argbv_rdn0_slot = var_argbv_rdn0;
        *var_argbv_rdn1_slot = var_argbv_rdn1;
        *var_argbv_rdn2_slot = var_argbv_rdn2;
        *var_argbv_rdn3_slot = var_argbv_rdn3;
        *var_argbv_rdn4_slot = var_argbv_rdn4;
        *var_argbv_rdn5_slot = var_argbv_rdn5;
        *var_argbv_rdn6_slot = var_argbv_rdn6;
        *var_argbv_rv_slot = var_argbv_rv;
        *var_argbvvt_slot = var_argbvvt;
        *var_argbvvt_db0_slot = var_argbvvt_db0;
        *var_argbvvt_db1_slot = var_argbvvt_db1;
        *var_argbvvt_db2_slot = var_argbvvt_db2;
        *var_argbvvt_db3_slot = var_argbvvt_db3;
        *var_argbvvt_db4_slot = var_argbvvt_db4;
        *var_argbvvt_db5_slot = var_argbvvt_db5;
        *var_argbvvt_db6_slot = var_argbvvt_db6;
        *var_argbvvt_dn0_slot = var_argbvvt_dn0;
        *var_argbvvt_dn1_slot = var_argbvvt_dn1;
        *var_argbvvt_dn2_slot = var_argbvvt_dn2;
        *var_argbvvt_dn3_slot = var_argbvvt_dn3;
        *var_argbvvt_dn4_slot = var_argbvvt_dn4;
        *var_argbvvt_dn5_slot = var_argbvvt_dn5;
        *var_argbvvt_dn6_slot = var_argbvvt_dn6;
        *var_argbvvt_rdb0_slot = var_argbvvt_rdb0;
        *var_argbvvt_rdb1_slot = var_argbvvt_rdb1;
        *var_argbvvt_rdb2_slot = var_argbvvt_rdb2;
        *var_argbvvt_rdb3_slot = var_argbvvt_rdb3;
        *var_argbvvt_rdb4_slot = var_argbvvt_rdb4;
        *var_argbvvt_rdb5_slot = var_argbvvt_rdb5;
        *var_argbvvt_rdb6_slot = var_argbvvt_rdb6;
        *var_argbvvt_rdn0_slot = var_argbvvt_rdn0;
        *var_argbvvt_rdn1_slot = var_argbvvt_rdn1;
        *var_argbvvt_rdn2_slot = var_argbvvt_rdn2;
        *var_argbvvt_rdn3_slot = var_argbvvt_rdn3;
        *var_argbvvt_rdn4_slot = var_argbvvt_rdn4;
        *var_argbvvt_rdn5_slot = var_argbvvt_rdn5;
        *var_argbvvt_rdn6_slot = var_argbvvt_rdn6;
        *var_argbvvt_rv_slot = var_argbvvt_rv;
        *var_guard4_slot = var_guard4;
        *var_guard4_db0_slot = var_guard4_db0;
        *var_guard4_db1_slot = var_guard4_db1;
        *var_guard4_db2_slot = var_guard4_db2;
        *var_guard4_db3_slot = var_guard4_db3;
        *var_guard4_db4_slot = var_guard4_db4;
        *var_guard4_db5_slot = var_guard4_db5;
        *var_guard4_db6_slot = var_guard4_db6;
        *var_guard4_dn0_slot = var_guard4_dn0;
        *var_guard4_dn1_slot = var_guard4_dn1;
        *var_guard4_dn2_slot = var_guard4_dn2;
        *var_guard4_dn3_slot = var_guard4_dn3;
        *var_guard4_dn4_slot = var_guard4_dn4;
        *var_guard4_dn5_slot = var_guard4_dn5;
        *var_guard4_dn6_slot = var_guard4_dn6;
        *var_guard4_rdb0_slot = var_guard4_rdb0;
        *var_guard4_rdb1_slot = var_guard4_rdb1;
        *var_guard4_rdb2_slot = var_guard4_rdb2;
        *var_guard4_rdb3_slot = var_guard4_rdb3;
        *var_guard4_rdb4_slot = var_guard4_rdb4;
        *var_guard4_rdb5_slot = var_guard4_rdb5;
        *var_guard4_rdb6_slot = var_guard4_rdb6;
        *var_guard4_rdn0_slot = var_guard4_rdn0;
        *var_guard4_rdn1_slot = var_guard4_rdn1;
        *var_guard4_rdn2_slot = var_guard4_rdn2;
        *var_guard4_rdn3_slot = var_guard4_rdn3;
        *var_guard4_rdn4_slot = var_guard4_rdn4;
        *var_guard4_rdn5_slot = var_guard4_rdn5;
        *var_guard4_rdn6_slot = var_guard4_rdn6;
        *var_guard4_rv_slot = var_guard4_rv;
        *var_ifwd_slot = var_ifwd;
        *var_ifwd_db0_slot = var_ifwd_db0;
        *var_ifwd_db1_slot = var_ifwd_db1;
        *var_ifwd_db2_slot = var_ifwd_db2;
        *var_ifwd_db3_slot = var_ifwd_db3;
        *var_ifwd_db4_slot = var_ifwd_db4;
        *var_ifwd_db5_slot = var_ifwd_db5;
        *var_ifwd_db6_slot = var_ifwd_db6;
        *var_ifwd_dn0_slot = var_ifwd_dn0;
        *var_ifwd_dn1_slot = var_ifwd_dn1;
        *var_ifwd_dn2_slot = var_ifwd_dn2;
        *var_ifwd_dn3_slot = var_ifwd_dn3;
        *var_ifwd_dn4_slot = var_ifwd_dn4;
        *var_ifwd_dn5_slot = var_ifwd_dn5;
        *var_ifwd_dn6_slot = var_ifwd_dn6;
        *var_ifwd_rdb0_slot = var_ifwd_rdb0;
        *var_ifwd_rdb1_slot = var_ifwd_rdb1;
        *var_ifwd_rdb2_slot = var_ifwd_rdb2;
        *var_ifwd_rdb3_slot = var_ifwd_rdb3;
        *var_ifwd_rdb4_slot = var_ifwd_rdb4;
        *var_ifwd_rdb5_slot = var_ifwd_rdb5;
        *var_ifwd_rdb6_slot = var_ifwd_rdb6;
        *var_ifwd_rdn0_slot = var_ifwd_rdn0;
        *var_ifwd_rdn1_slot = var_ifwd_rdn1;
        *var_ifwd_rdn2_slot = var_ifwd_rdn2;
        *var_ifwd_rdn3_slot = var_ifwd_rdn3;
        *var_ifwd_rdn4_slot = var_ifwd_rdn4;
        *var_ifwd_rdn5_slot = var_ifwd_rdn5;
        *var_ifwd_rdn6_slot = var_ifwd_rdn6;
        *var_ifwd_rv_slot = var_ifwd_rv;
        *var_le_slot = var_le;
        *var_le_db0_slot = var_le_db0;
        *var_le_db1_slot = var_le_db1;
        *var_le_db2_slot = var_le_db2;
        *var_le_db3_slot = var_le_db3;
        *var_le_db4_slot = var_le_db4;
        *var_le_db5_slot = var_le_db5;
        *var_le_db6_slot = var_le_db6;
        *var_le_dn0_slot = var_le_dn0;
        *var_le_dn1_slot = var_le_dn1;
        *var_le_dn2_slot = var_le_dn2;
        *var_le_dn3_slot = var_le_dn3;
        *var_le_dn4_slot = var_le_dn4;
        *var_le_dn5_slot = var_le_dn5;
        *var_le_dn6_slot = var_le_dn6;
        *var_le_rdb0_slot = var_le_rdb0;
        *var_le_rdb1_slot = var_le_rdb1;
        *var_le_rdb2_slot = var_le_rdb2;
        *var_le_rdb3_slot = var_le_rdb3;
        *var_le_rdb4_slot = var_le_rdb4;
        *var_le_rdb5_slot = var_le_rdb5;
        *var_le_rdb6_slot = var_le_rdb6;
        *var_le_rdn0_slot = var_le_rdn0;
        *var_le_rdn1_slot = var_le_rdn1;
        *var_le_rdn2_slot = var_le_rdn2;
        *var_le_rdn3_slot = var_le_rdn3;
        *var_le_rdn4_slot = var_le_rdn4;
        *var_le_rdn5_slot = var_le_rdn5;
        *var_le_rdn6_slot = var_le_rdn6;
        *var_le_rv_slot = var_le_rv;
        *var_lebv_slot = var_lebv;
        *var_lebv_db0_slot = var_lebv_db0;
        *var_lebv_db1_slot = var_lebv_db1;
        *var_lebv_db2_slot = var_lebv_db2;
        *var_lebv_db3_slot = var_lebv_db3;
        *var_lebv_db4_slot = var_lebv_db4;
        *var_lebv_db5_slot = var_lebv_db5;
        *var_lebv_db6_slot = var_lebv_db6;
        *var_lebv_dn0_slot = var_lebv_dn0;
        *var_lebv_dn1_slot = var_lebv_dn1;
        *var_lebv_dn2_slot = var_lebv_dn2;
        *var_lebv_dn3_slot = var_lebv_dn3;
        *var_lebv_dn4_slot = var_lebv_dn4;
        *var_lebv_dn5_slot = var_lebv_dn5;
        *var_lebv_dn6_slot = var_lebv_dn6;
        *var_lebv_rdb0_slot = var_lebv_rdb0;
        *var_lebv_rdb1_slot = var_lebv_rdb1;
        *var_lebv_rdb2_slot = var_lebv_rdb2;
        *var_lebv_rdb3_slot = var_lebv_rdb3;
        *var_lebv_rdb4_slot = var_lebv_rdb4;
        *var_lebv_rdb5_slot = var_lebv_rdb5;
        *var_lebv_rdb6_slot = var_lebv_rdb6;
        *var_lebv_rdn0_slot = var_lebv_rdn0;
        *var_lebv_rdn1_slot = var_lebv_rdn1;
        *var_lebv_rdn2_slot = var_lebv_rdn2;
        *var_lebv_rdn3_slot = var_lebv_rdn3;
        *var_lebv_rdn4_slot = var_lebv_rdn4;
        *var_lebv_rdn5_slot = var_lebv_rdn5;
        *var_lebv_rdn6_slot = var_lebv_rdn6;
        *var_lebv_rv_slot = var_lebv_rv;
    }

    pub(super) fn stamp_reactive_block_3(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_ifwd: f64,
        var_ifwd_db0: f64,
        var_ifwd_db1: f64,
        var_ifwd_db2: f64,
        var_ifwd_db3: f64,
        var_ifwd_db4: f64,
        var_ifwd_db5: f64,
        var_ifwd_db6: f64,
        var_ifwd_dn0: f64,
        var_ifwd_dn1: f64,
        var_ifwd_dn2: f64,
        var_ifwd_dn3: f64,
        var_ifwd_dn4: f64,
        var_ifwd_dn5: f64,
        var_ifwd_dn6: f64,
        var_isr_t: f64,
        var_vbiei: f64,
        var_vbiei_db0: f64,
        var_vbiei_db1: f64,
        var_vbiei_db2: f64,
        var_vbiei_db3: f64,
        var_vbiei_db4: f64,
        var_vbiei_db5: f64,
        var_vbiei_db6: f64,
        var_vbiei_dn0: f64,
        var_vbiei_dn1: f64,
        var_vbiei_dn2: f64,
        var_vbiei_dn3: f64,
        var_vbiei_dn4: f64,
        var_vbiei_dn5: f64,
        var_vbiei_dn6: f64,
        var_vt: f64,
        var_vt_db0: f64,
        var_vt_db1: f64,
        var_vt_db2: f64,
        var_vt_db3: f64,
        var_vt_db4: f64,
        var_vt_db5: f64,
        var_vt_db6: f64,
        var_vt_dn0: f64,
        var_vt_dn1: f64,
        var_vt_dn2: f64,
        var_vt_dn3: f64,
        var_vt_dn4: f64,
        var_vt_dn5: f64,
        var_vt_dn6: f64,
        var_arg_slot: &mut f64,
        var_arg_db0_slot: &mut f64,
        var_arg_db1_slot: &mut f64,
        var_arg_db2_slot: &mut f64,
        var_arg_db3_slot: &mut f64,
        var_arg_db4_slot: &mut f64,
        var_arg_db5_slot: &mut f64,
        var_arg_db6_slot: &mut f64,
        var_arg_dn0_slot: &mut f64,
        var_arg_dn1_slot: &mut f64,
        var_arg_dn2_slot: &mut f64,
        var_arg_dn3_slot: &mut f64,
        var_arg_dn4_slot: &mut f64,
        var_arg_dn5_slot: &mut f64,
        var_arg_dn6_slot: &mut f64,
        var_arg_rdb0_slot: &mut f64,
        var_arg_rdb1_slot: &mut f64,
        var_arg_rdb2_slot: &mut f64,
        var_arg_rdb3_slot: &mut f64,
        var_arg_rdb4_slot: &mut f64,
        var_arg_rdb5_slot: &mut f64,
        var_arg_rdb6_slot: &mut f64,
        var_arg_rdn0_slot: &mut f64,
        var_arg_rdn1_slot: &mut f64,
        var_arg_rdn2_slot: &mut f64,
        var_arg_rdn3_slot: &mut f64,
        var_arg_rdn4_slot: &mut f64,
        var_arg_rdn5_slot: &mut f64,
        var_arg_rdn6_slot: &mut f64,
        var_arg_rv_slot: &mut f64,
        var_guard5_slot: &mut f64,
        var_guard5_db0_slot: &mut f64,
        var_guard5_db1_slot: &mut f64,
        var_guard5_db2_slot: &mut f64,
        var_guard5_db3_slot: &mut f64,
        var_guard5_db4_slot: &mut f64,
        var_guard5_db5_slot: &mut f64,
        var_guard5_db6_slot: &mut f64,
        var_guard5_dn0_slot: &mut f64,
        var_guard5_dn1_slot: &mut f64,
        var_guard5_dn2_slot: &mut f64,
        var_guard5_dn3_slot: &mut f64,
        var_guard5_dn4_slot: &mut f64,
        var_guard5_dn5_slot: &mut f64,
        var_guard5_dn6_slot: &mut f64,
        var_guard5_rdb0_slot: &mut f64,
        var_guard5_rdb1_slot: &mut f64,
        var_guard5_rdb2_slot: &mut f64,
        var_guard5_rdb3_slot: &mut f64,
        var_guard5_rdb4_slot: &mut f64,
        var_guard5_rdb5_slot: &mut f64,
        var_guard5_rdb6_slot: &mut f64,
        var_guard5_rdn0_slot: &mut f64,
        var_guard5_rdn1_slot: &mut f64,
        var_guard5_rdn2_slot: &mut f64,
        var_guard5_rdn3_slot: &mut f64,
        var_guard5_rdn4_slot: &mut f64,
        var_guard5_rdn5_slot: &mut f64,
        var_guard5_rdn6_slot: &mut f64,
        var_guard5_rv_slot: &mut f64,
        var_guard6_slot: &mut f64,
        var_guard6_db0_slot: &mut f64,
        var_guard6_db1_slot: &mut f64,
        var_guard6_db2_slot: &mut f64,
        var_guard6_db3_slot: &mut f64,
        var_guard6_db4_slot: &mut f64,
        var_guard6_db5_slot: &mut f64,
        var_guard6_db6_slot: &mut f64,
        var_guard6_dn0_slot: &mut f64,
        var_guard6_dn1_slot: &mut f64,
        var_guard6_dn2_slot: &mut f64,
        var_guard6_dn3_slot: &mut f64,
        var_guard6_dn4_slot: &mut f64,
        var_guard6_dn5_slot: &mut f64,
        var_guard6_dn6_slot: &mut f64,
        var_guard6_rdb0_slot: &mut f64,
        var_guard6_rdb1_slot: &mut f64,
        var_guard6_rdb2_slot: &mut f64,
        var_guard6_rdb3_slot: &mut f64,
        var_guard6_rdb4_slot: &mut f64,
        var_guard6_rdb5_slot: &mut f64,
        var_guard6_rdb6_slot: &mut f64,
        var_guard6_rdn0_slot: &mut f64,
        var_guard6_rdn1_slot: &mut f64,
        var_guard6_rdn2_slot: &mut f64,
        var_guard6_rdn3_slot: &mut f64,
        var_guard6_rdn4_slot: &mut f64,
        var_guard6_rdn5_slot: &mut f64,
        var_guard6_rdn6_slot: &mut f64,
        var_guard6_rv_slot: &mut f64,
        var_guard8_slot: &mut f64,
        var_guard8_db0_slot: &mut f64,
        var_guard8_db1_slot: &mut f64,
        var_guard8_db2_slot: &mut f64,
        var_guard8_db3_slot: &mut f64,
        var_guard8_db4_slot: &mut f64,
        var_guard8_db5_slot: &mut f64,
        var_guard8_db6_slot: &mut f64,
        var_guard8_dn0_slot: &mut f64,
        var_guard8_dn1_slot: &mut f64,
        var_guard8_dn2_slot: &mut f64,
        var_guard8_dn3_slot: &mut f64,
        var_guard8_dn4_slot: &mut f64,
        var_guard8_dn5_slot: &mut f64,
        var_guard8_dn6_slot: &mut f64,
        var_guard8_rdb0_slot: &mut f64,
        var_guard8_rdb1_slot: &mut f64,
        var_guard8_rdb2_slot: &mut f64,
        var_guard8_rdb3_slot: &mut f64,
        var_guard8_rdb4_slot: &mut f64,
        var_guard8_rdb5_slot: &mut f64,
        var_guard8_rdb6_slot: &mut f64,
        var_guard8_rdn0_slot: &mut f64,
        var_guard8_rdn1_slot: &mut f64,
        var_guard8_rdn2_slot: &mut f64,
        var_guard8_rdn3_slot: &mut f64,
        var_guard8_rdn4_slot: &mut f64,
        var_guard8_rdn5_slot: &mut f64,
        var_guard8_rdn6_slot: &mut f64,
        var_guard8_rv_slot: &mut f64,
        var_itzf_slot: &mut f64,
        var_itzf_db0_slot: &mut f64,
        var_itzf_db1_slot: &mut f64,
        var_itzf_db2_slot: &mut f64,
        var_itzf_db3_slot: &mut f64,
        var_itzf_db4_slot: &mut f64,
        var_itzf_db5_slot: &mut f64,
        var_itzf_db6_slot: &mut f64,
        var_itzf_dn0_slot: &mut f64,
        var_itzf_dn1_slot: &mut f64,
        var_itzf_dn2_slot: &mut f64,
        var_itzf_dn3_slot: &mut f64,
        var_itzf_dn4_slot: &mut f64,
        var_itzf_dn5_slot: &mut f64,
        var_itzf_dn6_slot: &mut f64,
        var_itzf_rdb0_slot: &mut f64,
        var_itzf_rdb1_slot: &mut f64,
        var_itzf_rdb2_slot: &mut f64,
        var_itzf_rdb3_slot: &mut f64,
        var_itzf_rdb4_slot: &mut f64,
        var_itzf_rdb5_slot: &mut f64,
        var_itzf_rdb6_slot: &mut f64,
        var_itzf_rdn0_slot: &mut f64,
        var_itzf_rdn1_slot: &mut f64,
        var_itzf_rdn2_slot: &mut f64,
        var_itzf_rdn3_slot: &mut f64,
        var_itzf_rdn4_slot: &mut f64,
        var_itzf_rdn5_slot: &mut f64,
        var_itzf_rdn6_slot: &mut f64,
        var_itzf_rv_slot: &mut f64,
        var_le_slot: &mut f64,
        var_le_db0_slot: &mut f64,
        var_le_db1_slot: &mut f64,
        var_le_db2_slot: &mut f64,
        var_le_db3_slot: &mut f64,
        var_le_db4_slot: &mut f64,
        var_le_db5_slot: &mut f64,
        var_le_db6_slot: &mut f64,
        var_le_dn0_slot: &mut f64,
        var_le_dn1_slot: &mut f64,
        var_le_dn2_slot: &mut f64,
        var_le_dn3_slot: &mut f64,
        var_le_dn4_slot: &mut f64,
        var_le_dn5_slot: &mut f64,
        var_le_dn6_slot: &mut f64,
        var_le_rdb0_slot: &mut f64,
        var_le_rdb1_slot: &mut f64,
        var_le_rdb2_slot: &mut f64,
        var_le_rdb3_slot: &mut f64,
        var_le_rdb4_slot: &mut f64,
        var_le_rdb5_slot: &mut f64,
        var_le_rdb6_slot: &mut f64,
        var_le_rdn0_slot: &mut f64,
        var_le_rdn1_slot: &mut f64,
        var_le_rdn2_slot: &mut f64,
        var_le_rdn3_slot: &mut f64,
        var_le_rdn4_slot: &mut f64,
        var_le_rdn5_slot: &mut f64,
        var_le_rdn6_slot: &mut f64,
        var_le_rv_slot: &mut f64,
        var_qde_slot: &mut f64,
        var_qde_db0_slot: &mut f64,
        var_qde_db1_slot: &mut f64,
        var_qde_db2_slot: &mut f64,
        var_qde_db3_slot: &mut f64,
        var_qde_db4_slot: &mut f64,
        var_qde_db5_slot: &mut f64,
        var_qde_db6_slot: &mut f64,
        var_qde_dn0_slot: &mut f64,
        var_qde_dn1_slot: &mut f64,
        var_qde_dn2_slot: &mut f64,
        var_qde_dn3_slot: &mut f64,
        var_qde_dn4_slot: &mut f64,
        var_qde_dn5_slot: &mut f64,
        var_qde_dn6_slot: &mut f64,
        var_qde_rdb0_slot: &mut f64,
        var_qde_rdb1_slot: &mut f64,
        var_qde_rdb2_slot: &mut f64,
        var_qde_rdb3_slot: &mut f64,
        var_qde_rdb4_slot: &mut f64,
        var_qde_rdb5_slot: &mut f64,
        var_qde_rdb6_slot: &mut f64,
        var_qde_rdn0_slot: &mut f64,
        var_qde_rdn1_slot: &mut f64,
        var_qde_rdn2_slot: &mut f64,
        var_qde_rdn3_slot: &mut f64,
        var_qde_rdn4_slot: &mut f64,
        var_qde_rdn5_slot: &mut f64,
        var_qde_rdn6_slot: &mut f64,
        var_qde_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_db0_slot: &mut f64,
        var_t0_db1_slot: &mut f64,
        var_t0_db2_slot: &mut f64,
        var_t0_db3_slot: &mut f64,
        var_t0_db4_slot: &mut f64,
        var_t0_db5_slot: &mut f64,
        var_t0_db6_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn1_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_rdb0_slot: &mut f64,
        var_t0_rdb1_slot: &mut f64,
        var_t0_rdb2_slot: &mut f64,
        var_t0_rdb3_slot: &mut f64,
        var_t0_rdb4_slot: &mut f64,
        var_t0_rdb5_slot: &mut f64,
        var_t0_rdb6_slot: &mut f64,
        var_t0_rdn0_slot: &mut f64,
        var_t0_rdn1_slot: &mut f64,
        var_t0_rdn2_slot: &mut f64,
        var_t0_rdn3_slot: &mut f64,
        var_t0_rdn4_slot: &mut f64,
        var_t0_rdn5_slot: &mut f64,
        var_t0_rdn6_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_tff_slot: &mut f64,
        var_tff_db0_slot: &mut f64,
        var_tff_db1_slot: &mut f64,
        var_tff_db2_slot: &mut f64,
        var_tff_db3_slot: &mut f64,
        var_tff_db4_slot: &mut f64,
        var_tff_db5_slot: &mut f64,
        var_tff_db6_slot: &mut f64,
        var_tff_dn0_slot: &mut f64,
        var_tff_dn1_slot: &mut f64,
        var_tff_dn2_slot: &mut f64,
        var_tff_dn3_slot: &mut f64,
        var_tff_dn4_slot: &mut f64,
        var_tff_dn5_slot: &mut f64,
        var_tff_dn6_slot: &mut f64,
        var_tff_rdb0_slot: &mut f64,
        var_tff_rdb1_slot: &mut f64,
        var_tff_rdb2_slot: &mut f64,
        var_tff_rdb3_slot: &mut f64,
        var_tff_rdb4_slot: &mut f64,
        var_tff_rdb5_slot: &mut f64,
        var_tff_rdb6_slot: &mut f64,
        var_tff_rdn0_slot: &mut f64,
        var_tff_rdn1_slot: &mut f64,
        var_tff_rdn2_slot: &mut f64,
        var_tff_rdn3_slot: &mut f64,
        var_tff_rdn4_slot: &mut f64,
        var_tff_rdn5_slot: &mut f64,
        var_tff_rdn6_slot: &mut f64,
        var_tff_rv_slot: &mut f64,
        var_vtff_slot: &mut f64,
        var_vtff1_slot: &mut f64,
        var_vtff1_db0_slot: &mut f64,
        var_vtff1_db1_slot: &mut f64,
        var_vtff1_db2_slot: &mut f64,
        var_vtff1_db3_slot: &mut f64,
        var_vtff1_db4_slot: &mut f64,
        var_vtff1_db5_slot: &mut f64,
        var_vtff1_db6_slot: &mut f64,
        var_vtff1_dn0_slot: &mut f64,
        var_vtff1_dn1_slot: &mut f64,
        var_vtff1_dn2_slot: &mut f64,
        var_vtff1_dn3_slot: &mut f64,
        var_vtff1_dn4_slot: &mut f64,
        var_vtff1_dn5_slot: &mut f64,
        var_vtff1_dn6_slot: &mut f64,
        var_vtff1_rdb0_slot: &mut f64,
        var_vtff1_rdb1_slot: &mut f64,
        var_vtff1_rdb2_slot: &mut f64,
        var_vtff1_rdb3_slot: &mut f64,
        var_vtff1_rdb4_slot: &mut f64,
        var_vtff1_rdb5_slot: &mut f64,
        var_vtff1_rdb6_slot: &mut f64,
        var_vtff1_rdn0_slot: &mut f64,
        var_vtff1_rdn1_slot: &mut f64,
        var_vtff1_rdn2_slot: &mut f64,
        var_vtff1_rdn3_slot: &mut f64,
        var_vtff1_rdn4_slot: &mut f64,
        var_vtff1_rdn5_slot: &mut f64,
        var_vtff1_rdn6_slot: &mut f64,
        var_vtff1_rv_slot: &mut f64,
        var_vtff_db0_slot: &mut f64,
        var_vtff_db1_slot: &mut f64,
        var_vtff_db2_slot: &mut f64,
        var_vtff_db3_slot: &mut f64,
        var_vtff_db4_slot: &mut f64,
        var_vtff_db5_slot: &mut f64,
        var_vtff_db6_slot: &mut f64,
        var_vtff_dn0_slot: &mut f64,
        var_vtff_dn1_slot: &mut f64,
        var_vtff_dn2_slot: &mut f64,
        var_vtff_dn3_slot: &mut f64,
        var_vtff_dn4_slot: &mut f64,
        var_vtff_dn5_slot: &mut f64,
        var_vtff_dn6_slot: &mut f64,
        var_vtff_rdb0_slot: &mut f64,
        var_vtff_rdb1_slot: &mut f64,
        var_vtff_rdb2_slot: &mut f64,
        var_vtff_rdb3_slot: &mut f64,
        var_vtff_rdb4_slot: &mut f64,
        var_vtff_rdb5_slot: &mut f64,
        var_vtff_rdb6_slot: &mut f64,
        var_vtff_rdn0_slot: &mut f64,
        var_vtff_rdn1_slot: &mut f64,
        var_vtff_rdn2_slot: &mut f64,
        var_vtff_rdn3_slot: &mut f64,
        var_vtff_rdn4_slot: &mut f64,
        var_vtff_rdn5_slot: &mut f64,
        var_vtff_rdn6_slot: &mut f64,
        var_vtff_rv_slot: &mut f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let mut var_arg: f64 = *var_arg_slot;
        let mut var_arg_db0: f64 = *var_arg_db0_slot;
        let mut var_arg_db1: f64 = *var_arg_db1_slot;
        let mut var_arg_db2: f64 = *var_arg_db2_slot;
        let mut var_arg_db3: f64 = *var_arg_db3_slot;
        let mut var_arg_db4: f64 = *var_arg_db4_slot;
        let mut var_arg_db5: f64 = *var_arg_db5_slot;
        let mut var_arg_db6: f64 = *var_arg_db6_slot;
        let mut var_arg_dn0: f64 = *var_arg_dn0_slot;
        let mut var_arg_dn1: f64 = *var_arg_dn1_slot;
        let mut var_arg_dn2: f64 = *var_arg_dn2_slot;
        let mut var_arg_dn3: f64 = *var_arg_dn3_slot;
        let mut var_arg_dn4: f64 = *var_arg_dn4_slot;
        let mut var_arg_dn5: f64 = *var_arg_dn5_slot;
        let mut var_arg_dn6: f64 = *var_arg_dn6_slot;
        let mut var_arg_rdb0: f64 = *var_arg_rdb0_slot;
        let mut var_arg_rdb1: f64 = *var_arg_rdb1_slot;
        let mut var_arg_rdb2: f64 = *var_arg_rdb2_slot;
        let mut var_arg_rdb3: f64 = *var_arg_rdb3_slot;
        let mut var_arg_rdb4: f64 = *var_arg_rdb4_slot;
        let mut var_arg_rdb5: f64 = *var_arg_rdb5_slot;
        let mut var_arg_rdb6: f64 = *var_arg_rdb6_slot;
        let mut var_arg_rdn0: f64 = *var_arg_rdn0_slot;
        let mut var_arg_rdn1: f64 = *var_arg_rdn1_slot;
        let mut var_arg_rdn2: f64 = *var_arg_rdn2_slot;
        let mut var_arg_rdn3: f64 = *var_arg_rdn3_slot;
        let mut var_arg_rdn4: f64 = *var_arg_rdn4_slot;
        let mut var_arg_rdn5: f64 = *var_arg_rdn5_slot;
        let mut var_arg_rdn6: f64 = *var_arg_rdn6_slot;
        let mut var_arg_rv: f64 = *var_arg_rv_slot;
        let mut var_guard5: f64 = *var_guard5_slot;
        let mut var_guard5_db0: f64 = *var_guard5_db0_slot;
        let mut var_guard5_db1: f64 = *var_guard5_db1_slot;
        let mut var_guard5_db2: f64 = *var_guard5_db2_slot;
        let mut var_guard5_db3: f64 = *var_guard5_db3_slot;
        let mut var_guard5_db4: f64 = *var_guard5_db4_slot;
        let mut var_guard5_db5: f64 = *var_guard5_db5_slot;
        let mut var_guard5_db6: f64 = *var_guard5_db6_slot;
        let mut var_guard5_dn0: f64 = *var_guard5_dn0_slot;
        let mut var_guard5_dn1: f64 = *var_guard5_dn1_slot;
        let mut var_guard5_dn2: f64 = *var_guard5_dn2_slot;
        let mut var_guard5_dn3: f64 = *var_guard5_dn3_slot;
        let mut var_guard5_dn4: f64 = *var_guard5_dn4_slot;
        let mut var_guard5_dn5: f64 = *var_guard5_dn5_slot;
        let mut var_guard5_dn6: f64 = *var_guard5_dn6_slot;
        let mut var_guard5_rdb0: f64 = *var_guard5_rdb0_slot;
        let mut var_guard5_rdb1: f64 = *var_guard5_rdb1_slot;
        let mut var_guard5_rdb2: f64 = *var_guard5_rdb2_slot;
        let mut var_guard5_rdb3: f64 = *var_guard5_rdb3_slot;
        let mut var_guard5_rdb4: f64 = *var_guard5_rdb4_slot;
        let mut var_guard5_rdb5: f64 = *var_guard5_rdb5_slot;
        let mut var_guard5_rdb6: f64 = *var_guard5_rdb6_slot;
        let mut var_guard5_rdn0: f64 = *var_guard5_rdn0_slot;
        let mut var_guard5_rdn1: f64 = *var_guard5_rdn1_slot;
        let mut var_guard5_rdn2: f64 = *var_guard5_rdn2_slot;
        let mut var_guard5_rdn3: f64 = *var_guard5_rdn3_slot;
        let mut var_guard5_rdn4: f64 = *var_guard5_rdn4_slot;
        let mut var_guard5_rdn5: f64 = *var_guard5_rdn5_slot;
        let mut var_guard5_rdn6: f64 = *var_guard5_rdn6_slot;
        let mut var_guard5_rv: f64 = *var_guard5_rv_slot;
        let mut var_guard6: f64 = *var_guard6_slot;
        let mut var_guard6_db0: f64 = *var_guard6_db0_slot;
        let mut var_guard6_db1: f64 = *var_guard6_db1_slot;
        let mut var_guard6_db2: f64 = *var_guard6_db2_slot;
        let mut var_guard6_db3: f64 = *var_guard6_db3_slot;
        let mut var_guard6_db4: f64 = *var_guard6_db4_slot;
        let mut var_guard6_db5: f64 = *var_guard6_db5_slot;
        let mut var_guard6_db6: f64 = *var_guard6_db6_slot;
        let mut var_guard6_dn0: f64 = *var_guard6_dn0_slot;
        let mut var_guard6_dn1: f64 = *var_guard6_dn1_slot;
        let mut var_guard6_dn2: f64 = *var_guard6_dn2_slot;
        let mut var_guard6_dn3: f64 = *var_guard6_dn3_slot;
        let mut var_guard6_dn4: f64 = *var_guard6_dn4_slot;
        let mut var_guard6_dn5: f64 = *var_guard6_dn5_slot;
        let mut var_guard6_dn6: f64 = *var_guard6_dn6_slot;
        let mut var_guard6_rdb0: f64 = *var_guard6_rdb0_slot;
        let mut var_guard6_rdb1: f64 = *var_guard6_rdb1_slot;
        let mut var_guard6_rdb2: f64 = *var_guard6_rdb2_slot;
        let mut var_guard6_rdb3: f64 = *var_guard6_rdb3_slot;
        let mut var_guard6_rdb4: f64 = *var_guard6_rdb4_slot;
        let mut var_guard6_rdb5: f64 = *var_guard6_rdb5_slot;
        let mut var_guard6_rdb6: f64 = *var_guard6_rdb6_slot;
        let mut var_guard6_rdn0: f64 = *var_guard6_rdn0_slot;
        let mut var_guard6_rdn1: f64 = *var_guard6_rdn1_slot;
        let mut var_guard6_rdn2: f64 = *var_guard6_rdn2_slot;
        let mut var_guard6_rdn3: f64 = *var_guard6_rdn3_slot;
        let mut var_guard6_rdn4: f64 = *var_guard6_rdn4_slot;
        let mut var_guard6_rdn5: f64 = *var_guard6_rdn5_slot;
        let mut var_guard6_rdn6: f64 = *var_guard6_rdn6_slot;
        let mut var_guard6_rv: f64 = *var_guard6_rv_slot;
        let mut var_guard8: f64 = *var_guard8_slot;
        let mut var_guard8_db0: f64 = *var_guard8_db0_slot;
        let mut var_guard8_db1: f64 = *var_guard8_db1_slot;
        let mut var_guard8_db2: f64 = *var_guard8_db2_slot;
        let mut var_guard8_db3: f64 = *var_guard8_db3_slot;
        let mut var_guard8_db4: f64 = *var_guard8_db4_slot;
        let mut var_guard8_db5: f64 = *var_guard8_db5_slot;
        let mut var_guard8_db6: f64 = *var_guard8_db6_slot;
        let mut var_guard8_dn0: f64 = *var_guard8_dn0_slot;
        let mut var_guard8_dn1: f64 = *var_guard8_dn1_slot;
        let mut var_guard8_dn2: f64 = *var_guard8_dn2_slot;
        let mut var_guard8_dn3: f64 = *var_guard8_dn3_slot;
        let mut var_guard8_dn4: f64 = *var_guard8_dn4_slot;
        let mut var_guard8_dn5: f64 = *var_guard8_dn5_slot;
        let mut var_guard8_dn6: f64 = *var_guard8_dn6_slot;
        let mut var_guard8_rdb0: f64 = *var_guard8_rdb0_slot;
        let mut var_guard8_rdb1: f64 = *var_guard8_rdb1_slot;
        let mut var_guard8_rdb2: f64 = *var_guard8_rdb2_slot;
        let mut var_guard8_rdb3: f64 = *var_guard8_rdb3_slot;
        let mut var_guard8_rdb4: f64 = *var_guard8_rdb4_slot;
        let mut var_guard8_rdb5: f64 = *var_guard8_rdb5_slot;
        let mut var_guard8_rdb6: f64 = *var_guard8_rdb6_slot;
        let mut var_guard8_rdn0: f64 = *var_guard8_rdn0_slot;
        let mut var_guard8_rdn1: f64 = *var_guard8_rdn1_slot;
        let mut var_guard8_rdn2: f64 = *var_guard8_rdn2_slot;
        let mut var_guard8_rdn3: f64 = *var_guard8_rdn3_slot;
        let mut var_guard8_rdn4: f64 = *var_guard8_rdn4_slot;
        let mut var_guard8_rdn5: f64 = *var_guard8_rdn5_slot;
        let mut var_guard8_rdn6: f64 = *var_guard8_rdn6_slot;
        let mut var_guard8_rv: f64 = *var_guard8_rv_slot;
        let mut var_itzf: f64 = *var_itzf_slot;
        let mut var_itzf_db0: f64 = *var_itzf_db0_slot;
        let mut var_itzf_db1: f64 = *var_itzf_db1_slot;
        let mut var_itzf_db2: f64 = *var_itzf_db2_slot;
        let mut var_itzf_db3: f64 = *var_itzf_db3_slot;
        let mut var_itzf_db4: f64 = *var_itzf_db4_slot;
        let mut var_itzf_db5: f64 = *var_itzf_db5_slot;
        let mut var_itzf_db6: f64 = *var_itzf_db6_slot;
        let mut var_itzf_dn0: f64 = *var_itzf_dn0_slot;
        let mut var_itzf_dn1: f64 = *var_itzf_dn1_slot;
        let mut var_itzf_dn2: f64 = *var_itzf_dn2_slot;
        let mut var_itzf_dn3: f64 = *var_itzf_dn3_slot;
        let mut var_itzf_dn4: f64 = *var_itzf_dn4_slot;
        let mut var_itzf_dn5: f64 = *var_itzf_dn5_slot;
        let mut var_itzf_dn6: f64 = *var_itzf_dn6_slot;
        let mut var_itzf_rdb0: f64 = *var_itzf_rdb0_slot;
        let mut var_itzf_rdb1: f64 = *var_itzf_rdb1_slot;
        let mut var_itzf_rdb2: f64 = *var_itzf_rdb2_slot;
        let mut var_itzf_rdb3: f64 = *var_itzf_rdb3_slot;
        let mut var_itzf_rdb4: f64 = *var_itzf_rdb4_slot;
        let mut var_itzf_rdb5: f64 = *var_itzf_rdb5_slot;
        let mut var_itzf_rdb6: f64 = *var_itzf_rdb6_slot;
        let mut var_itzf_rdn0: f64 = *var_itzf_rdn0_slot;
        let mut var_itzf_rdn1: f64 = *var_itzf_rdn1_slot;
        let mut var_itzf_rdn2: f64 = *var_itzf_rdn2_slot;
        let mut var_itzf_rdn3: f64 = *var_itzf_rdn3_slot;
        let mut var_itzf_rdn4: f64 = *var_itzf_rdn4_slot;
        let mut var_itzf_rdn5: f64 = *var_itzf_rdn5_slot;
        let mut var_itzf_rdn6: f64 = *var_itzf_rdn6_slot;
        let mut var_itzf_rv: f64 = *var_itzf_rv_slot;
        let mut var_le: f64 = *var_le_slot;
        let mut var_le_db0: f64 = *var_le_db0_slot;
        let mut var_le_db1: f64 = *var_le_db1_slot;
        let mut var_le_db2: f64 = *var_le_db2_slot;
        let mut var_le_db3: f64 = *var_le_db3_slot;
        let mut var_le_db4: f64 = *var_le_db4_slot;
        let mut var_le_db5: f64 = *var_le_db5_slot;
        let mut var_le_db6: f64 = *var_le_db6_slot;
        let mut var_le_dn0: f64 = *var_le_dn0_slot;
        let mut var_le_dn1: f64 = *var_le_dn1_slot;
        let mut var_le_dn2: f64 = *var_le_dn2_slot;
        let mut var_le_dn3: f64 = *var_le_dn3_slot;
        let mut var_le_dn4: f64 = *var_le_dn4_slot;
        let mut var_le_dn5: f64 = *var_le_dn5_slot;
        let mut var_le_dn6: f64 = *var_le_dn6_slot;
        let mut var_le_rdb0: f64 = *var_le_rdb0_slot;
        let mut var_le_rdb1: f64 = *var_le_rdb1_slot;
        let mut var_le_rdb2: f64 = *var_le_rdb2_slot;
        let mut var_le_rdb3: f64 = *var_le_rdb3_slot;
        let mut var_le_rdb4: f64 = *var_le_rdb4_slot;
        let mut var_le_rdb5: f64 = *var_le_rdb5_slot;
        let mut var_le_rdb6: f64 = *var_le_rdb6_slot;
        let mut var_le_rdn0: f64 = *var_le_rdn0_slot;
        let mut var_le_rdn1: f64 = *var_le_rdn1_slot;
        let mut var_le_rdn2: f64 = *var_le_rdn2_slot;
        let mut var_le_rdn3: f64 = *var_le_rdn3_slot;
        let mut var_le_rdn4: f64 = *var_le_rdn4_slot;
        let mut var_le_rdn5: f64 = *var_le_rdn5_slot;
        let mut var_le_rdn6: f64 = *var_le_rdn6_slot;
        let mut var_le_rv: f64 = *var_le_rv_slot;
        let mut var_qde: f64 = *var_qde_slot;
        let mut var_qde_db0: f64 = *var_qde_db0_slot;
        let mut var_qde_db1: f64 = *var_qde_db1_slot;
        let mut var_qde_db2: f64 = *var_qde_db2_slot;
        let mut var_qde_db3: f64 = *var_qde_db3_slot;
        let mut var_qde_db4: f64 = *var_qde_db4_slot;
        let mut var_qde_db5: f64 = *var_qde_db5_slot;
        let mut var_qde_db6: f64 = *var_qde_db6_slot;
        let mut var_qde_dn0: f64 = *var_qde_dn0_slot;
        let mut var_qde_dn1: f64 = *var_qde_dn1_slot;
        let mut var_qde_dn2: f64 = *var_qde_dn2_slot;
        let mut var_qde_dn3: f64 = *var_qde_dn3_slot;
        let mut var_qde_dn4: f64 = *var_qde_dn4_slot;
        let mut var_qde_dn5: f64 = *var_qde_dn5_slot;
        let mut var_qde_dn6: f64 = *var_qde_dn6_slot;
        let mut var_qde_rdb0: f64 = *var_qde_rdb0_slot;
        let mut var_qde_rdb1: f64 = *var_qde_rdb1_slot;
        let mut var_qde_rdb2: f64 = *var_qde_rdb2_slot;
        let mut var_qde_rdb3: f64 = *var_qde_rdb3_slot;
        let mut var_qde_rdb4: f64 = *var_qde_rdb4_slot;
        let mut var_qde_rdb5: f64 = *var_qde_rdb5_slot;
        let mut var_qde_rdb6: f64 = *var_qde_rdb6_slot;
        let mut var_qde_rdn0: f64 = *var_qde_rdn0_slot;
        let mut var_qde_rdn1: f64 = *var_qde_rdn1_slot;
        let mut var_qde_rdn2: f64 = *var_qde_rdn2_slot;
        let mut var_qde_rdn3: f64 = *var_qde_rdn3_slot;
        let mut var_qde_rdn4: f64 = *var_qde_rdn4_slot;
        let mut var_qde_rdn5: f64 = *var_qde_rdn5_slot;
        let mut var_qde_rdn6: f64 = *var_qde_rdn6_slot;
        let mut var_qde_rv: f64 = *var_qde_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_db0: f64 = *var_t0_db0_slot;
        let mut var_t0_db1: f64 = *var_t0_db1_slot;
        let mut var_t0_db2: f64 = *var_t0_db2_slot;
        let mut var_t0_db3: f64 = *var_t0_db3_slot;
        let mut var_t0_db4: f64 = *var_t0_db4_slot;
        let mut var_t0_db5: f64 = *var_t0_db5_slot;
        let mut var_t0_db6: f64 = *var_t0_db6_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn1: f64 = *var_t0_dn1_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_rdb0: f64 = *var_t0_rdb0_slot;
        let mut var_t0_rdb1: f64 = *var_t0_rdb1_slot;
        let mut var_t0_rdb2: f64 = *var_t0_rdb2_slot;
        let mut var_t0_rdb3: f64 = *var_t0_rdb3_slot;
        let mut var_t0_rdb4: f64 = *var_t0_rdb4_slot;
        let mut var_t0_rdb5: f64 = *var_t0_rdb5_slot;
        let mut var_t0_rdb6: f64 = *var_t0_rdb6_slot;
        let mut var_t0_rdn0: f64 = *var_t0_rdn0_slot;
        let mut var_t0_rdn1: f64 = *var_t0_rdn1_slot;
        let mut var_t0_rdn2: f64 = *var_t0_rdn2_slot;
        let mut var_t0_rdn3: f64 = *var_t0_rdn3_slot;
        let mut var_t0_rdn4: f64 = *var_t0_rdn4_slot;
        let mut var_t0_rdn5: f64 = *var_t0_rdn5_slot;
        let mut var_t0_rdn6: f64 = *var_t0_rdn6_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_tff: f64 = *var_tff_slot;
        let mut var_tff_db0: f64 = *var_tff_db0_slot;
        let mut var_tff_db1: f64 = *var_tff_db1_slot;
        let mut var_tff_db2: f64 = *var_tff_db2_slot;
        let mut var_tff_db3: f64 = *var_tff_db3_slot;
        let mut var_tff_db4: f64 = *var_tff_db4_slot;
        let mut var_tff_db5: f64 = *var_tff_db5_slot;
        let mut var_tff_db6: f64 = *var_tff_db6_slot;
        let mut var_tff_dn0: f64 = *var_tff_dn0_slot;
        let mut var_tff_dn1: f64 = *var_tff_dn1_slot;
        let mut var_tff_dn2: f64 = *var_tff_dn2_slot;
        let mut var_tff_dn3: f64 = *var_tff_dn3_slot;
        let mut var_tff_dn4: f64 = *var_tff_dn4_slot;
        let mut var_tff_dn5: f64 = *var_tff_dn5_slot;
        let mut var_tff_dn6: f64 = *var_tff_dn6_slot;
        let mut var_tff_rdb0: f64 = *var_tff_rdb0_slot;
        let mut var_tff_rdb1: f64 = *var_tff_rdb1_slot;
        let mut var_tff_rdb2: f64 = *var_tff_rdb2_slot;
        let mut var_tff_rdb3: f64 = *var_tff_rdb3_slot;
        let mut var_tff_rdb4: f64 = *var_tff_rdb4_slot;
        let mut var_tff_rdb5: f64 = *var_tff_rdb5_slot;
        let mut var_tff_rdb6: f64 = *var_tff_rdb6_slot;
        let mut var_tff_rdn0: f64 = *var_tff_rdn0_slot;
        let mut var_tff_rdn1: f64 = *var_tff_rdn1_slot;
        let mut var_tff_rdn2: f64 = *var_tff_rdn2_slot;
        let mut var_tff_rdn3: f64 = *var_tff_rdn3_slot;
        let mut var_tff_rdn4: f64 = *var_tff_rdn4_slot;
        let mut var_tff_rdn5: f64 = *var_tff_rdn5_slot;
        let mut var_tff_rdn6: f64 = *var_tff_rdn6_slot;
        let mut var_tff_rv: f64 = *var_tff_rv_slot;
        let mut var_vtff: f64 = *var_vtff_slot;
        let mut var_vtff1: f64 = *var_vtff1_slot;
        let mut var_vtff1_db0: f64 = *var_vtff1_db0_slot;
        let mut var_vtff1_db1: f64 = *var_vtff1_db1_slot;
        let mut var_vtff1_db2: f64 = *var_vtff1_db2_slot;
        let mut var_vtff1_db3: f64 = *var_vtff1_db3_slot;
        let mut var_vtff1_db4: f64 = *var_vtff1_db4_slot;
        let mut var_vtff1_db5: f64 = *var_vtff1_db5_slot;
        let mut var_vtff1_db6: f64 = *var_vtff1_db6_slot;
        let mut var_vtff1_dn0: f64 = *var_vtff1_dn0_slot;
        let mut var_vtff1_dn1: f64 = *var_vtff1_dn1_slot;
        let mut var_vtff1_dn2: f64 = *var_vtff1_dn2_slot;
        let mut var_vtff1_dn3: f64 = *var_vtff1_dn3_slot;
        let mut var_vtff1_dn4: f64 = *var_vtff1_dn4_slot;
        let mut var_vtff1_dn5: f64 = *var_vtff1_dn5_slot;
        let mut var_vtff1_dn6: f64 = *var_vtff1_dn6_slot;
        let mut var_vtff1_rdb0: f64 = *var_vtff1_rdb0_slot;
        let mut var_vtff1_rdb1: f64 = *var_vtff1_rdb1_slot;
        let mut var_vtff1_rdb2: f64 = *var_vtff1_rdb2_slot;
        let mut var_vtff1_rdb3: f64 = *var_vtff1_rdb3_slot;
        let mut var_vtff1_rdb4: f64 = *var_vtff1_rdb4_slot;
        let mut var_vtff1_rdb5: f64 = *var_vtff1_rdb5_slot;
        let mut var_vtff1_rdb6: f64 = *var_vtff1_rdb6_slot;
        let mut var_vtff1_rdn0: f64 = *var_vtff1_rdn0_slot;
        let mut var_vtff1_rdn1: f64 = *var_vtff1_rdn1_slot;
        let mut var_vtff1_rdn2: f64 = *var_vtff1_rdn2_slot;
        let mut var_vtff1_rdn3: f64 = *var_vtff1_rdn3_slot;
        let mut var_vtff1_rdn4: f64 = *var_vtff1_rdn4_slot;
        let mut var_vtff1_rdn5: f64 = *var_vtff1_rdn5_slot;
        let mut var_vtff1_rdn6: f64 = *var_vtff1_rdn6_slot;
        let mut var_vtff1_rv: f64 = *var_vtff1_rv_slot;
        let mut var_vtff_db0: f64 = *var_vtff_db0_slot;
        let mut var_vtff_db1: f64 = *var_vtff_db1_slot;
        let mut var_vtff_db2: f64 = *var_vtff_db2_slot;
        let mut var_vtff_db3: f64 = *var_vtff_db3_slot;
        let mut var_vtff_db4: f64 = *var_vtff_db4_slot;
        let mut var_vtff_db5: f64 = *var_vtff_db5_slot;
        let mut var_vtff_db6: f64 = *var_vtff_db6_slot;
        let mut var_vtff_dn0: f64 = *var_vtff_dn0_slot;
        let mut var_vtff_dn1: f64 = *var_vtff_dn1_slot;
        let mut var_vtff_dn2: f64 = *var_vtff_dn2_slot;
        let mut var_vtff_dn3: f64 = *var_vtff_dn3_slot;
        let mut var_vtff_dn4: f64 = *var_vtff_dn4_slot;
        let mut var_vtff_dn5: f64 = *var_vtff_dn5_slot;
        let mut var_vtff_dn6: f64 = *var_vtff_dn6_slot;
        let mut var_vtff_rdb0: f64 = *var_vtff_rdb0_slot;
        let mut var_vtff_rdb1: f64 = *var_vtff_rdb1_slot;
        let mut var_vtff_rdb2: f64 = *var_vtff_rdb2_slot;
        let mut var_vtff_rdb3: f64 = *var_vtff_rdb3_slot;
        let mut var_vtff_rdb4: f64 = *var_vtff_rdb4_slot;
        let mut var_vtff_rdb5: f64 = *var_vtff_rdb5_slot;
        let mut var_vtff_rdb6: f64 = *var_vtff_rdb6_slot;
        let mut var_vtff_rdn0: f64 = *var_vtff_rdn0_slot;
        let mut var_vtff_rdn1: f64 = *var_vtff_rdn1_slot;
        let mut var_vtff_rdn2: f64 = *var_vtff_rdn2_slot;
        let mut var_vtff_rdn3: f64 = *var_vtff_rdn3_slot;
        let mut var_vtff_rdn4: f64 = *var_vtff_rdn4_slot;
        let mut var_vtff_rdn5: f64 = *var_vtff_rdn5_slot;
        let mut var_vtff_rdn6: f64 = *var_vtff_rdn6_slot;
        let mut var_vtff_rv: f64 = *var_vtff_rv_slot;

        let assign440_e665: f64 = if var_isr_t > 0.0 { 1.0 } else { 0.0 };
        var_guard5 = assign440_e665;
        var_guard5_dn0 = 0.0;
        var_guard5_dn1 = 0.0;
        var_guard5_dn2 = 0.0;
        var_guard5_dn3 = 0.0;
        var_guard5_dn4 = 0.0;
        var_guard5_dn5 = 0.0;
        var_guard5_dn6 = 0.0;
        var_guard5_db0 = 0.0;
        var_guard5_db1 = 0.0;
        var_guard5_db2 = 0.0;
        var_guard5_db3 = 0.0;
        var_guard5_db4 = 0.0;
        var_guard5_db5 = 0.0;
        var_guard5_db6 = 0.0;
        var_guard5_rv = 0.0;
        var_guard5_rdn0 = 0.0;
        var_guard5_rdn1 = 0.0;
        var_guard5_rdn2 = 0.0;
        var_guard5_rdn3 = 0.0;
        var_guard5_rdn4 = 0.0;
        var_guard5_rdn5 = 0.0;
        var_guard5_rdn6 = 0.0;
        var_guard5_rdb0 = 0.0;
        var_guard5_rdb1 = 0.0;
        var_guard5_rdb2 = 0.0;
        var_guard5_rdb3 = 0.0;
        var_guard5_rdb4 = 0.0;
        var_guard5_rdb5 = 0.0;
        var_guard5_rdb6 = 0.0;

        let (assign450_e673, assign450_e673_d_n0, assign450_e673_d_n1, assign450_e673_d_n2, assign450_e673_d_n3, assign450_e673_d_n4, assign450_e673_d_n5, assign450_e673_d_n6, assign450_e673_d_b0, assign450_e673_d_b1, assign450_e673_d_b2, assign450_e673_d_b3, assign450_e673_d_b4, assign450_e673_d_b5, assign450_e673_d_b6,) = {
    if (var_guard5 != 0.0) {
        let assign450_e669: f64 = (p.p4 - var_vbiei);
        let assign450_e671: f64 = (assign450_e669).max(0.001);
        (assign450_e671, if assign450_e669 >= 0.001 { (-var_vbiei_dn0) } else { 0.0 }, if assign450_e669 >= 0.001 { (-var_vbiei_dn1) } else { 0.0 }, if assign450_e669 >= 0.001 { (-var_vbiei_dn2) } else { 0.0 }, if assign450_e669 >= 0.001 { (-var_vbiei_dn3) } else { 0.0 }, if assign450_e669 >= 0.001 { (-var_vbiei_dn4) } else { 0.0 }, if assign450_e669 >= 0.001 { (-var_vbiei_dn5) } else { 0.0 }, if assign450_e669 >= 0.001 { (-var_vbiei_dn6) } else { 0.0 }, if assign450_e669 >= 0.001 { (-var_vbiei_db0) } else { 0.0 }, if assign450_e669 >= 0.001 { (-var_vbiei_db1) } else { 0.0 }, if assign450_e669 >= 0.001 { (-var_vbiei_db2) } else { 0.0 }, if assign450_e669 >= 0.001 { (-var_vbiei_db3) } else { 0.0 }, if assign450_e669 >= 0.001 { (-var_vbiei_db4) } else { 0.0 }, if assign450_e669 >= 0.001 { (-var_vbiei_db5) } else { 0.0 }, if assign450_e669 >= 0.001 { (-var_vbiei_db6) } else { 0.0 },)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn1, var_t0_dn2, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_db0, var_t0_db1, var_t0_db2, var_t0_db3, var_t0_db4, var_t0_db5, var_t0_db6,)
    }
};
        var_t0 = assign450_e673;
        var_t0_dn0 = assign450_e673_d_n0;
        var_t0_dn1 = assign450_e673_d_n1;
        var_t0_dn2 = assign450_e673_d_n2;
        var_t0_dn3 = assign450_e673_d_n3;
        var_t0_dn4 = assign450_e673_d_n4;
        var_t0_dn5 = assign450_e673_d_n5;
        var_t0_dn6 = assign450_e673_d_n6;
        var_t0_db0 = assign450_e673_d_b0;
        var_t0_db1 = assign450_e673_d_b1;
        var_t0_db2 = assign450_e673_d_b2;
        var_t0_db3 = assign450_e673_d_b3;
        var_t0_db4 = assign450_e673_d_b4;
        var_t0_db5 = assign450_e673_d_b5;
        var_t0_db6 = assign450_e673_d_b6;
        var_t0_rv = 0.0;
        var_t0_rdn0 = 0.0;
        var_t0_rdn1 = 0.0;
        var_t0_rdn2 = 0.0;
        var_t0_rdn3 = 0.0;
        var_t0_rdn4 = 0.0;
        var_t0_rdn5 = 0.0;
        var_t0_rdn6 = 0.0;
        var_t0_rdb0 = 0.0;
        var_t0_rdb1 = 0.0;
        var_t0_rdb2 = 0.0;
        var_t0_rdb3 = 0.0;
        var_t0_rdb4 = 0.0;
        var_t0_rdb5 = 0.0;
        var_t0_rdb6 = 0.0;

        let (assign460_e688, assign460_e688_d_n0, assign460_e688_d_n1, assign460_e688_d_n2, assign460_e688_d_n3, assign460_e688_d_n4, assign460_e688_d_n5, assign460_e688_d_n6, assign460_e688_d_b0, assign460_e688_d_b1, assign460_e688_d_b2, assign460_e688_d_b3, assign460_e688_d_b4, assign460_e688_d_b5, assign460_e688_d_b6,) = {
    if (var_guard5 != 0.0) {
        let assign460_e676: f64 = (-1.0);
        let assign460_e678: f64 = (assign460_e676 * var_vbiei);
        let assign460_e680: f64 = (assign460_e678 * p.p4);
        let assign460_e683: f64 = (p.p3 * var_vt);
        let assign460_e685: f64 = (assign460_e683 * var_t0);
        let assign460_e686: f64 = (assign460_e680 / assign460_e685);
        (assign460_e686, (((((assign460_e676 * var_vbiei_dn0) * p.p4) * assign460_e685) - (assign460_e680 * (((p.p3 * var_vt_dn0) * var_t0) + (assign460_e683 * var_t0_dn0)))) / (assign460_e685 * assign460_e685)), (((((assign460_e676 * var_vbiei_dn1) * p.p4) * assign460_e685) - (assign460_e680 * (((p.p3 * var_vt_dn1) * var_t0) + (assign460_e683 * var_t0_dn1)))) / (assign460_e685 * assign460_e685)), (((((assign460_e676 * var_vbiei_dn2) * p.p4) * assign460_e685) - (assign460_e680 * (((p.p3 * var_vt_dn2) * var_t0) + (assign460_e683 * var_t0_dn2)))) / (assign460_e685 * assign460_e685)), (((((assign460_e676 * var_vbiei_dn3) * p.p4) * assign460_e685) - (assign460_e680 * (((p.p3 * var_vt_dn3) * var_t0) + (assign460_e683 * var_t0_dn3)))) / (assign460_e685 * assign460_e685)), (((((assign460_e676 * var_vbiei_dn4) * p.p4) * assign460_e685) - (assign460_e680 * (((p.p3 * var_vt_dn4) * var_t0) + (assign460_e683 * var_t0_dn4)))) / (assign460_e685 * assign460_e685)), (((((assign460_e676 * var_vbiei_dn5) * p.p4) * assign460_e685) - (assign460_e680 * (((p.p3 * var_vt_dn5) * var_t0) + (assign460_e683 * var_t0_dn5)))) / (assign460_e685 * assign460_e685)), (((((assign460_e676 * var_vbiei_dn6) * p.p4) * assign460_e685) - (assign460_e680 * (((p.p3 * var_vt_dn6) * var_t0) + (assign460_e683 * var_t0_dn6)))) / (assign460_e685 * assign460_e685)), (((((assign460_e676 * var_vbiei_db0) * p.p4) * assign460_e685) - (assign460_e680 * (((p.p3 * var_vt_db0) * var_t0) + (assign460_e683 * var_t0_db0)))) / (assign460_e685 * assign460_e685)), (((((assign460_e676 * var_vbiei_db1) * p.p4) * assign460_e685) - (assign460_e680 * (((p.p3 * var_vt_db1) * var_t0) + (assign460_e683 * var_t0_db1)))) / (assign460_e685 * assign460_e685)), (((((assign460_e676 * var_vbiei_db2) * p.p4) * assign460_e685) - (assign460_e680 * (((p.p3 * var_vt_db2) * var_t0) + (assign460_e683 * var_t0_db2)))) / (assign460_e685 * assign460_e685)), (((((assign460_e676 * var_vbiei_db3) * p.p4) * assign460_e685) - (assign460_e680 * (((p.p3 * var_vt_db3) * var_t0) + (assign460_e683 * var_t0_db3)))) / (assign460_e685 * assign460_e685)), (((((assign460_e676 * var_vbiei_db4) * p.p4) * assign460_e685) - (assign460_e680 * (((p.p3 * var_vt_db4) * var_t0) + (assign460_e683 * var_t0_db4)))) / (assign460_e685 * assign460_e685)), (((((assign460_e676 * var_vbiei_db5) * p.p4) * assign460_e685) - (assign460_e680 * (((p.p3 * var_vt_db5) * var_t0) + (assign460_e683 * var_t0_db5)))) / (assign460_e685 * assign460_e685)), (((((assign460_e676 * var_vbiei_db6) * p.p4) * assign460_e685) - (assign460_e680 * (((p.p3 * var_vt_db6) * var_t0) + (assign460_e683 * var_t0_db6)))) / (assign460_e685 * assign460_e685)),)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn1, var_arg_dn2, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_db0, var_arg_db1, var_arg_db2, var_arg_db3, var_arg_db4, var_arg_db5, var_arg_db6,)
    }
};
        var_arg = assign460_e688;
        var_arg_dn0 = assign460_e688_d_n0;
        var_arg_dn1 = assign460_e688_d_n1;
        var_arg_dn2 = assign460_e688_d_n2;
        var_arg_dn3 = assign460_e688_d_n3;
        var_arg_dn4 = assign460_e688_d_n4;
        var_arg_dn5 = assign460_e688_d_n5;
        var_arg_dn6 = assign460_e688_d_n6;
        var_arg_db0 = assign460_e688_d_b0;
        var_arg_db1 = assign460_e688_d_b1;
        var_arg_db2 = assign460_e688_d_b2;
        var_arg_db3 = assign460_e688_d_b3;
        var_arg_db4 = assign460_e688_d_b4;
        var_arg_db5 = assign460_e688_d_b5;
        var_arg_db6 = assign460_e688_d_b6;
        var_arg_rv = 0.0;
        var_arg_rdn0 = 0.0;
        var_arg_rdn1 = 0.0;
        var_arg_rdn2 = 0.0;
        var_arg_rdn3 = 0.0;
        var_arg_rdn4 = 0.0;
        var_arg_rdn5 = 0.0;
        var_arg_rdn6 = 0.0;
        var_arg_rdb0 = 0.0;
        var_arg_rdb1 = 0.0;
        var_arg_rdb2 = 0.0;
        var_arg_rdb3 = 0.0;
        var_arg_rdb4 = 0.0;
        var_arg_rdb5 = 0.0;
        var_arg_rdb6 = 0.0;

        let assign470_e691: f64 = if var_arg > 80.0 { 1.0 } else { 0.0 };
        var_guard6 = assign470_e691;
        var_guard6_dn0 = 0.0;
        var_guard6_dn1 = 0.0;
        var_guard6_dn2 = 0.0;
        var_guard6_dn3 = 0.0;
        var_guard6_dn4 = 0.0;
        var_guard6_dn5 = 0.0;
        var_guard6_dn6 = 0.0;
        var_guard6_db0 = 0.0;
        var_guard6_db1 = 0.0;
        var_guard6_db2 = 0.0;
        var_guard6_db3 = 0.0;
        var_guard6_db4 = 0.0;
        var_guard6_db5 = 0.0;
        var_guard6_db6 = 0.0;
        var_guard6_rv = 0.0;
        var_guard6_rdn0 = 0.0;
        var_guard6_rdn1 = 0.0;
        var_guard6_rdn2 = 0.0;
        var_guard6_rdn3 = 0.0;
        var_guard6_rdn4 = 0.0;
        var_guard6_rdn5 = 0.0;
        var_guard6_rdn6 = 0.0;
        var_guard6_rdb0 = 0.0;
        var_guard6_rdb1 = 0.0;
        var_guard6_rdb2 = 0.0;
        var_guard6_rdb3 = 0.0;
        var_guard6_rdb4 = 0.0;
        var_guard6_rdb5 = 0.0;
        var_guard6_rdb6 = 0.0;

        let (assign480_e701, assign480_e701_d_n0, assign480_e701_d_n1, assign480_e701_d_n2, assign480_e701_d_n3, assign480_e701_d_n4, assign480_e701_d_n5, assign480_e701_d_n6, assign480_e701_d_b0, assign480_e701_d_b1, assign480_e701_d_b2, assign480_e701_d_b3, assign480_e701_d_b4, assign480_e701_d_b5, assign480_e701_d_b6,) = {
    if ((var_guard5 != 0.0) && (var_guard6 != 0.0)) {
        let assign480_e698: f64 = (var_arg - 80.0);
        let assign480_e699: f64 = (1.0 + assign480_e698);
        (assign480_e699, var_arg_dn0, var_arg_dn1, var_arg_dn2, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_db0, var_arg_db1, var_arg_db2, var_arg_db3, var_arg_db4, var_arg_db5, var_arg_db6,)
    } else {
        (var_le, var_le_dn0, var_le_dn1, var_le_dn2, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6, var_le_db0, var_le_db1, var_le_db2, var_le_db3, var_le_db4, var_le_db5, var_le_db6,)
    }
};
        var_le = assign480_e701;
        var_le_dn0 = assign480_e701_d_n0;
        var_le_dn1 = assign480_e701_d_n1;
        var_le_dn2 = assign480_e701_d_n2;
        var_le_dn3 = assign480_e701_d_n3;
        var_le_dn4 = assign480_e701_d_n4;
        var_le_dn5 = assign480_e701_d_n5;
        var_le_dn6 = assign480_e701_d_n6;
        var_le_db0 = assign480_e701_d_b0;
        var_le_db1 = assign480_e701_d_b1;
        var_le_db2 = assign480_e701_d_b2;
        var_le_db3 = assign480_e701_d_b3;
        var_le_db4 = assign480_e701_d_b4;
        var_le_db5 = assign480_e701_d_b5;
        var_le_db6 = assign480_e701_d_b6;
        var_le_rv = 0.0;
        var_le_rdn0 = 0.0;
        var_le_rdn1 = 0.0;
        var_le_rdn2 = 0.0;
        var_le_rdn3 = 0.0;
        var_le_rdn4 = 0.0;
        var_le_rdn5 = 0.0;
        var_le_rdn6 = 0.0;
        var_le_rdb0 = 0.0;
        var_le_rdb1 = 0.0;
        var_le_rdb2 = 0.0;
        var_le_rdb3 = 0.0;
        var_le_rdb4 = 0.0;
        var_le_rdb5 = 0.0;
        var_le_rdb6 = 0.0;

        let (assign490_e707, assign490_e707_d_n0, assign490_e707_d_n1, assign490_e707_d_n2, assign490_e707_d_n3, assign490_e707_d_n4, assign490_e707_d_n5, assign490_e707_d_n6, assign490_e707_d_b0, assign490_e707_d_b1, assign490_e707_d_b2, assign490_e707_d_b3, assign490_e707_d_b4, assign490_e707_d_b5, assign490_e707_d_b6,) = {
    if ((var_guard5 != 0.0) && (var_guard6 != 0.0)) {
        (80.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn1, var_arg_dn2, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_db0, var_arg_db1, var_arg_db2, var_arg_db3, var_arg_db4, var_arg_db5, var_arg_db6,)
    }
};
        var_arg = assign490_e707;
        var_arg_dn0 = assign490_e707_d_n0;
        var_arg_dn1 = assign490_e707_d_n1;
        var_arg_dn2 = assign490_e707_d_n2;
        var_arg_dn3 = assign490_e707_d_n3;
        var_arg_dn4 = assign490_e707_d_n4;
        var_arg_dn5 = assign490_e707_d_n5;
        var_arg_dn6 = assign490_e707_d_n6;
        var_arg_db0 = assign490_e707_d_b0;
        var_arg_db1 = assign490_e707_d_b1;
        var_arg_db2 = assign490_e707_d_b2;
        var_arg_db3 = assign490_e707_d_b3;
        var_arg_db4 = assign490_e707_d_b4;
        var_arg_db5 = assign490_e707_d_b5;
        var_arg_db6 = assign490_e707_d_b6;
        var_arg_rv = 0.0;
        var_arg_rdn0 = 0.0;
        var_arg_rdn1 = 0.0;
        var_arg_rdn2 = 0.0;
        var_arg_rdn3 = 0.0;
        var_arg_rdn4 = 0.0;
        var_arg_rdn5 = 0.0;
        var_arg_rdn6 = 0.0;
        var_arg_rdb0 = 0.0;
        var_arg_rdb1 = 0.0;
        var_arg_rdb2 = 0.0;
        var_arg_rdb3 = 0.0;
        var_arg_rdb4 = 0.0;
        var_arg_rdb5 = 0.0;
        var_arg_rdb6 = 0.0;

        let (assign500_e714, assign500_e714_d_n0, assign500_e714_d_n1, assign500_e714_d_n2, assign500_e714_d_n3, assign500_e714_d_n4, assign500_e714_d_n5, assign500_e714_d_n6, assign500_e714_d_b0, assign500_e714_d_b1, assign500_e714_d_b2, assign500_e714_d_b3, assign500_e714_d_b4, assign500_e714_d_b5, assign500_e714_d_b6,) = {
    if ((var_guard5 != 0.0) && (var_guard6 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_le, var_le_dn0, var_le_dn1, var_le_dn2, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6, var_le_db0, var_le_db1, var_le_db2, var_le_db3, var_le_db4, var_le_db5, var_le_db6,)
    }
};
        var_le = assign500_e714;
        var_le_dn0 = assign500_e714_d_n0;
        var_le_dn1 = assign500_e714_d_n1;
        var_le_dn2 = assign500_e714_d_n2;
        var_le_dn3 = assign500_e714_d_n3;
        var_le_dn4 = assign500_e714_d_n4;
        var_le_dn5 = assign500_e714_d_n5;
        var_le_dn6 = assign500_e714_d_n6;
        var_le_db0 = assign500_e714_d_b0;
        var_le_db1 = assign500_e714_d_b1;
        var_le_db2 = assign500_e714_d_b2;
        var_le_db3 = assign500_e714_d_b3;
        var_le_db4 = assign500_e714_d_b4;
        var_le_db5 = assign500_e714_d_b5;
        var_le_db6 = assign500_e714_d_b6;
        var_le_rv = 0.0;
        var_le_rdn0 = 0.0;
        var_le_rdn1 = 0.0;
        var_le_rdn2 = 0.0;
        var_le_rdn3 = 0.0;
        var_le_rdn4 = 0.0;
        var_le_rdn5 = 0.0;
        var_le_rdn6 = 0.0;
        var_le_rdb0 = 0.0;
        var_le_rdb1 = 0.0;
        var_le_rdb2 = 0.0;
        var_le_rdb3 = 0.0;
        var_le_rdb4 = 0.0;
        var_le_rdb5 = 0.0;
        var_le_rdb6 = 0.0;

        let (assign510_e721, assign510_e721_d_n0, assign510_e721_d_n1, assign510_e721_d_n2, assign510_e721_d_n3, assign510_e721_d_n4, assign510_e721_d_n5, assign510_e721_d_n6, assign510_e721_d_b0, assign510_e721_d_b1, assign510_e721_d_b2, assign510_e721_d_b3, assign510_e721_d_b4, assign510_e721_d_b5, assign510_e721_d_b6,) = {
    if (var_guard5 != 0.0) {
        let assign510_e718: f64 = (var_arg).exp();
        let assign510_e719: f64 = (var_le * assign510_e718);
        (assign510_e719, ((var_le_dn0 * assign510_e718) + (var_le * (assign510_e718 * var_arg_dn0))), ((var_le_dn1 * assign510_e718) + (var_le * (assign510_e718 * var_arg_dn1))), ((var_le_dn2 * assign510_e718) + (var_le * (assign510_e718 * var_arg_dn2))), ((var_le_dn3 * assign510_e718) + (var_le * (assign510_e718 * var_arg_dn3))), ((var_le_dn4 * assign510_e718) + (var_le * (assign510_e718 * var_arg_dn4))), ((var_le_dn5 * assign510_e718) + (var_le * (assign510_e718 * var_arg_dn5))), ((var_le_dn6 * assign510_e718) + (var_le * (assign510_e718 * var_arg_dn6))), ((var_le_db0 * assign510_e718) + (var_le * (assign510_e718 * var_arg_db0))), ((var_le_db1 * assign510_e718) + (var_le * (assign510_e718 * var_arg_db1))), ((var_le_db2 * assign510_e718) + (var_le * (assign510_e718 * var_arg_db2))), ((var_le_db3 * assign510_e718) + (var_le * (assign510_e718 * var_arg_db3))), ((var_le_db4 * assign510_e718) + (var_le * (assign510_e718 * var_arg_db4))), ((var_le_db5 * assign510_e718) + (var_le * (assign510_e718 * var_arg_db5))), ((var_le_db6 * assign510_e718) + (var_le * (assign510_e718 * var_arg_db6))),)
    } else {
        (var_le, var_le_dn0, var_le_dn1, var_le_dn2, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6, var_le_db0, var_le_db1, var_le_db2, var_le_db3, var_le_db4, var_le_db5, var_le_db6,)
    }
};
        var_le = assign510_e721;
        var_le_dn0 = assign510_e721_d_n0;
        var_le_dn1 = assign510_e721_d_n1;
        var_le_dn2 = assign510_e721_d_n2;
        var_le_dn3 = assign510_e721_d_n3;
        var_le_dn4 = assign510_e721_d_n4;
        var_le_dn5 = assign510_e721_d_n5;
        var_le_dn6 = assign510_e721_d_n6;
        var_le_db0 = assign510_e721_d_b0;
        var_le_db1 = assign510_e721_d_b1;
        var_le_db2 = assign510_e721_d_b2;
        var_le_db3 = assign510_e721_d_b3;
        var_le_db4 = assign510_e721_d_b4;
        var_le_db5 = assign510_e721_d_b5;
        var_le_db6 = assign510_e721_d_b6;
        var_le_rv = 0.0;
        var_le_rdn0 = 0.0;
        var_le_rdn1 = 0.0;
        var_le_rdn2 = 0.0;
        var_le_rdn3 = 0.0;
        var_le_rdn4 = 0.0;
        var_le_rdn5 = 0.0;
        var_le_rdn6 = 0.0;
        var_le_rdb0 = 0.0;
        var_le_rdb1 = 0.0;
        var_le_rdb2 = 0.0;
        var_le_rdb3 = 0.0;
        var_le_rdb4 = 0.0;
        var_le_rdb5 = 0.0;
        var_le_rdb6 = 0.0;

        var_itzf = var_ifwd;
        var_itzf_dn0 = var_ifwd_dn0;
        var_itzf_dn1 = var_ifwd_dn1;
        var_itzf_dn2 = var_ifwd_dn2;
        var_itzf_dn3 = var_ifwd_dn3;
        var_itzf_dn4 = var_ifwd_dn4;
        var_itzf_dn5 = var_ifwd_dn5;
        var_itzf_dn6 = var_ifwd_dn6;
        var_itzf_db0 = var_ifwd_db0;
        var_itzf_db1 = var_ifwd_db1;
        var_itzf_db2 = var_ifwd_db2;
        var_itzf_db3 = var_ifwd_db3;
        var_itzf_db4 = var_ifwd_db4;
        var_itzf_db5 = var_ifwd_db5;
        var_itzf_db6 = var_ifwd_db6;
        var_itzf_rv = 0.0;
        var_itzf_rdn0 = 0.0;
        var_itzf_rdn1 = 0.0;
        var_itzf_rdn2 = 0.0;
        var_itzf_rdn3 = 0.0;
        var_itzf_rdn4 = 0.0;
        var_itzf_rdn5 = 0.0;
        var_itzf_rdn6 = 0.0;
        var_itzf_rdb0 = 0.0;
        var_itzf_rdb1 = 0.0;
        var_itzf_rdb2 = 0.0;
        var_itzf_rdb3 = 0.0;
        var_itzf_rdb4 = 0.0;
        var_itzf_rdb5 = 0.0;
        var_itzf_rdb6 = 0.0;

        let assign630_e796: f64 = ((nv0 - nv1) / p.p40);
        let assign630_e797: f64 = (assign630_e796).abs();
        let assign630_e799: f64 = (assign630_e797).powf(p.p39);
        var_vtff = assign630_e799;
        var_vtff_dn0 = if 0.0 == 0.0 && ((p.p39) as f64).is_finite() && ((p.p39) as f64).fract() == 0.0 { if p.p39 == 0.0 { 0.0 } else { (p.p39 * ((assign630_e797).powf(p.p39 - 1.0) * if assign630_e796 >= 0.0 { (1.0 / p.p40) } else { (-(1.0 / p.p40)) })) } } else { (assign630_e799 * (p.p39 * (if assign630_e796 >= 0.0 { (1.0 / p.p40) } else { (-(1.0 / p.p40)) } / assign630_e797))) };
        var_vtff_dn1 = if 0.0 == 0.0 && ((p.p39) as f64).is_finite() && ((p.p39) as f64).fract() == 0.0 { if p.p39 == 0.0 { 0.0 } else { (p.p39 * ((assign630_e797).powf(p.p39 - 1.0) * if assign630_e796 >= 0.0 { (-1.0 / p.p40) } else { (-(-1.0 / p.p40)) })) } } else { (assign630_e799 * (p.p39 * (if assign630_e796 >= 0.0 { (-1.0 / p.p40) } else { (-(-1.0 / p.p40)) } / assign630_e797))) };
        var_vtff_dn2 = 0.0;
        var_vtff_dn3 = 0.0;
        var_vtff_dn4 = 0.0;
        var_vtff_dn5 = 0.0;
        var_vtff_dn6 = 0.0;
        var_vtff_db0 = 0.0;
        var_vtff_db1 = 0.0;
        var_vtff_db2 = 0.0;
        var_vtff_db3 = 0.0;
        var_vtff_db4 = 0.0;
        var_vtff_db5 = 0.0;
        var_vtff_db6 = 0.0;
        var_vtff_rv = 0.0;
        var_vtff_rdn0 = 0.0;
        var_vtff_rdn1 = 0.0;
        var_vtff_rdn2 = 0.0;
        var_vtff_rdn3 = 0.0;
        var_vtff_rdn4 = 0.0;
        var_vtff_rdn5 = 0.0;
        var_vtff_rdn6 = 0.0;
        var_vtff_rdb0 = 0.0;
        var_vtff_rdb1 = 0.0;
        var_vtff_rdb2 = 0.0;
        var_vtff_rdb3 = 0.0;
        var_vtff_rdb4 = 0.0;
        var_vtff_rdb5 = 0.0;
        var_vtff_rdb6 = 0.0;

        let assign640_e802: f64 = (1.0 + var_vtff);
        let assign640_e805: f64 = (1.0 / p.p39);
        let assign640_e806: f64 = (assign640_e802).powf(assign640_e805);
        let assign640_e808: f64 = (assign640_e806 - 1.0);
        var_vtff1 = assign640_e808;
        var_vtff1_dn0 = if 0.0 == 0.0 && ((assign640_e805) as f64).is_finite() && ((assign640_e805) as f64).fract() == 0.0 { if assign640_e805 == 0.0 { 0.0 } else { (assign640_e805 * ((assign640_e802).powf(assign640_e805 - 1.0) * var_vtff_dn0)) } } else { (assign640_e806 * (assign640_e805 * (var_vtff_dn0 / assign640_e802))) };
        var_vtff1_dn1 = if 0.0 == 0.0 && ((assign640_e805) as f64).is_finite() && ((assign640_e805) as f64).fract() == 0.0 { if assign640_e805 == 0.0 { 0.0 } else { (assign640_e805 * ((assign640_e802).powf(assign640_e805 - 1.0) * var_vtff_dn1)) } } else { (assign640_e806 * (assign640_e805 * (var_vtff_dn1 / assign640_e802))) };
        var_vtff1_dn2 = if 0.0 == 0.0 && ((assign640_e805) as f64).is_finite() && ((assign640_e805) as f64).fract() == 0.0 { if assign640_e805 == 0.0 { 0.0 } else { (assign640_e805 * ((assign640_e802).powf(assign640_e805 - 1.0) * var_vtff_dn2)) } } else { (assign640_e806 * (assign640_e805 * (var_vtff_dn2 / assign640_e802))) };
        var_vtff1_dn3 = if 0.0 == 0.0 && ((assign640_e805) as f64).is_finite() && ((assign640_e805) as f64).fract() == 0.0 { if assign640_e805 == 0.0 { 0.0 } else { (assign640_e805 * ((assign640_e802).powf(assign640_e805 - 1.0) * var_vtff_dn3)) } } else { (assign640_e806 * (assign640_e805 * (var_vtff_dn3 / assign640_e802))) };
        var_vtff1_dn4 = if 0.0 == 0.0 && ((assign640_e805) as f64).is_finite() && ((assign640_e805) as f64).fract() == 0.0 { if assign640_e805 == 0.0 { 0.0 } else { (assign640_e805 * ((assign640_e802).powf(assign640_e805 - 1.0) * var_vtff_dn4)) } } else { (assign640_e806 * (assign640_e805 * (var_vtff_dn4 / assign640_e802))) };
        var_vtff1_dn5 = if 0.0 == 0.0 && ((assign640_e805) as f64).is_finite() && ((assign640_e805) as f64).fract() == 0.0 { if assign640_e805 == 0.0 { 0.0 } else { (assign640_e805 * ((assign640_e802).powf(assign640_e805 - 1.0) * var_vtff_dn5)) } } else { (assign640_e806 * (assign640_e805 * (var_vtff_dn5 / assign640_e802))) };
        var_vtff1_dn6 = if 0.0 == 0.0 && ((assign640_e805) as f64).is_finite() && ((assign640_e805) as f64).fract() == 0.0 { if assign640_e805 == 0.0 { 0.0 } else { (assign640_e805 * ((assign640_e802).powf(assign640_e805 - 1.0) * var_vtff_dn6)) } } else { (assign640_e806 * (assign640_e805 * (var_vtff_dn6 / assign640_e802))) };
        var_vtff1_db0 = if 0.0 == 0.0 && ((assign640_e805) as f64).is_finite() && ((assign640_e805) as f64).fract() == 0.0 { if assign640_e805 == 0.0 { 0.0 } else { (assign640_e805 * ((assign640_e802).powf(assign640_e805 - 1.0) * var_vtff_db0)) } } else { (assign640_e806 * (assign640_e805 * (var_vtff_db0 / assign640_e802))) };
        var_vtff1_db1 = if 0.0 == 0.0 && ((assign640_e805) as f64).is_finite() && ((assign640_e805) as f64).fract() == 0.0 { if assign640_e805 == 0.0 { 0.0 } else { (assign640_e805 * ((assign640_e802).powf(assign640_e805 - 1.0) * var_vtff_db1)) } } else { (assign640_e806 * (assign640_e805 * (var_vtff_db1 / assign640_e802))) };
        var_vtff1_db2 = if 0.0 == 0.0 && ((assign640_e805) as f64).is_finite() && ((assign640_e805) as f64).fract() == 0.0 { if assign640_e805 == 0.0 { 0.0 } else { (assign640_e805 * ((assign640_e802).powf(assign640_e805 - 1.0) * var_vtff_db2)) } } else { (assign640_e806 * (assign640_e805 * (var_vtff_db2 / assign640_e802))) };
        var_vtff1_db3 = if 0.0 == 0.0 && ((assign640_e805) as f64).is_finite() && ((assign640_e805) as f64).fract() == 0.0 { if assign640_e805 == 0.0 { 0.0 } else { (assign640_e805 * ((assign640_e802).powf(assign640_e805 - 1.0) * var_vtff_db3)) } } else { (assign640_e806 * (assign640_e805 * (var_vtff_db3 / assign640_e802))) };
        var_vtff1_db4 = if 0.0 == 0.0 && ((assign640_e805) as f64).is_finite() && ((assign640_e805) as f64).fract() == 0.0 { if assign640_e805 == 0.0 { 0.0 } else { (assign640_e805 * ((assign640_e802).powf(assign640_e805 - 1.0) * var_vtff_db4)) } } else { (assign640_e806 * (assign640_e805 * (var_vtff_db4 / assign640_e802))) };
        var_vtff1_db5 = if 0.0 == 0.0 && ((assign640_e805) as f64).is_finite() && ((assign640_e805) as f64).fract() == 0.0 { if assign640_e805 == 0.0 { 0.0 } else { (assign640_e805 * ((assign640_e802).powf(assign640_e805 - 1.0) * var_vtff_db5)) } } else { (assign640_e806 * (assign640_e805 * (var_vtff_db5 / assign640_e802))) };
        var_vtff1_db6 = if 0.0 == 0.0 && ((assign640_e805) as f64).is_finite() && ((assign640_e805) as f64).fract() == 0.0 { if assign640_e805 == 0.0 { 0.0 } else { (assign640_e805 * ((assign640_e802).powf(assign640_e805 - 1.0) * var_vtff_db6)) } } else { (assign640_e806 * (assign640_e805 * (var_vtff_db6 / assign640_e802))) };
        var_vtff1_rv = 0.0;
        var_vtff1_rdn0 = 0.0;
        var_vtff1_rdn1 = 0.0;
        var_vtff1_rdn2 = 0.0;
        var_vtff1_rdn3 = 0.0;
        var_vtff1_rdn4 = 0.0;
        var_vtff1_rdn5 = 0.0;
        var_vtff1_rdn6 = 0.0;
        var_vtff1_rdb0 = 0.0;
        var_vtff1_rdb1 = 0.0;
        var_vtff1_rdb2 = 0.0;
        var_vtff1_rdb3 = 0.0;
        var_vtff1_rdb4 = 0.0;
        var_vtff1_rdb5 = 0.0;
        var_vtff1_rdb6 = 0.0;

        let assign650_e813: f64 = (p.p41 * var_vtff1);
        let assign650_e814: f64 = (1.0 + assign650_e813);
        let assign650_e815: f64 = (p.p19 * assign650_e814);
        var_tff = assign650_e815;
        var_tff_dn0 = (p.p19 * (p.p41 * var_vtff1_dn0));
        var_tff_dn1 = (p.p19 * (p.p41 * var_vtff1_dn1));
        var_tff_dn2 = (p.p19 * (p.p41 * var_vtff1_dn2));
        var_tff_dn3 = (p.p19 * (p.p41 * var_vtff1_dn3));
        var_tff_dn4 = (p.p19 * (p.p41 * var_vtff1_dn4));
        var_tff_dn5 = (p.p19 * (p.p41 * var_vtff1_dn5));
        var_tff_dn6 = (p.p19 * (p.p41 * var_vtff1_dn6));
        var_tff_db0 = (p.p19 * (p.p41 * var_vtff1_db0));
        var_tff_db1 = (p.p19 * (p.p41 * var_vtff1_db1));
        var_tff_db2 = (p.p19 * (p.p41 * var_vtff1_db2));
        var_tff_db3 = (p.p19 * (p.p41 * var_vtff1_db3));
        var_tff_db4 = (p.p19 * (p.p41 * var_vtff1_db4));
        var_tff_db5 = (p.p19 * (p.p41 * var_vtff1_db5));
        var_tff_db6 = (p.p19 * (p.p41 * var_vtff1_db6));
        var_tff_rv = 0.0;
        var_tff_rdn0 = 0.0;
        var_tff_rdn1 = 0.0;
        var_tff_rdn2 = 0.0;
        var_tff_rdn3 = 0.0;
        var_tff_rdn4 = 0.0;
        var_tff_rdn5 = 0.0;
        var_tff_rdn6 = 0.0;
        var_tff_rdb0 = 0.0;
        var_tff_rdb1 = 0.0;
        var_tff_rdb2 = 0.0;
        var_tff_rdb3 = 0.0;
        var_tff_rdb4 = 0.0;
        var_tff_rdb5 = 0.0;
        var_tff_rdb6 = 0.0;

        let assign660_e818: f64 = (var_tff * var_itzf);
        var_qde = assign660_e818;
        var_qde_dn0 = ((var_tff_dn0 * var_itzf) + (var_tff * var_itzf_dn0));
        var_qde_dn1 = ((var_tff_dn1 * var_itzf) + (var_tff * var_itzf_dn1));
        var_qde_dn2 = ((var_tff_dn2 * var_itzf) + (var_tff * var_itzf_dn2));
        var_qde_dn3 = ((var_tff_dn3 * var_itzf) + (var_tff * var_itzf_dn3));
        var_qde_dn4 = ((var_tff_dn4 * var_itzf) + (var_tff * var_itzf_dn4));
        var_qde_dn5 = ((var_tff_dn5 * var_itzf) + (var_tff * var_itzf_dn5));
        var_qde_dn6 = ((var_tff_dn6 * var_itzf) + (var_tff * var_itzf_dn6));
        var_qde_db0 = ((var_tff_db0 * var_itzf) + (var_tff * var_itzf_db0));
        var_qde_db1 = ((var_tff_db1 * var_itzf) + (var_tff * var_itzf_db1));
        var_qde_db2 = ((var_tff_db2 * var_itzf) + (var_tff * var_itzf_db2));
        var_qde_db3 = ((var_tff_db3 * var_itzf) + (var_tff * var_itzf_db3));
        var_qde_db4 = ((var_tff_db4 * var_itzf) + (var_tff * var_itzf_db4));
        var_qde_db5 = ((var_tff_db5 * var_itzf) + (var_tff * var_itzf_db5));
        var_qde_db6 = ((var_tff_db6 * var_itzf) + (var_tff * var_itzf_db6));
        var_qde_rv = 0.0;
        var_qde_rdn0 = 0.0;
        var_qde_rdn1 = 0.0;
        var_qde_rdn2 = 0.0;
        var_qde_rdn3 = 0.0;
        var_qde_rdn4 = 0.0;
        var_qde_rdn5 = 0.0;
        var_qde_rdn6 = 0.0;
        var_qde_rdb0 = 0.0;
        var_qde_rdb1 = 0.0;
        var_qde_rdb2 = 0.0;
        var_qde_rdb3 = 0.0;
        var_qde_rdb4 = 0.0;
        var_qde_rdb5 = 0.0;
        var_qde_rdb6 = 0.0;

        let assign670_e821: f64 = if p.p32 == 1.0 { 1.0 } else { 0.0 };
        var_guard8 = assign670_e821;
        var_guard8_dn0 = 0.0;
        var_guard8_dn1 = 0.0;
        var_guard8_dn2 = 0.0;
        var_guard8_dn3 = 0.0;
        var_guard8_dn4 = 0.0;
        var_guard8_dn5 = 0.0;
        var_guard8_dn6 = 0.0;
        var_guard8_db0 = 0.0;
        var_guard8_db1 = 0.0;
        var_guard8_db2 = 0.0;
        var_guard8_db3 = 0.0;
        var_guard8_db4 = 0.0;
        var_guard8_db5 = 0.0;
        var_guard8_db6 = 0.0;
        var_guard8_rv = 0.0;
        var_guard8_rdn0 = 0.0;
        var_guard8_rdn1 = 0.0;
        var_guard8_rdn2 = 0.0;
        var_guard8_rdn3 = 0.0;
        var_guard8_rdn4 = 0.0;
        var_guard8_rdn5 = 0.0;
        var_guard8_rdn6 = 0.0;
        var_guard8_rdb0 = 0.0;
        var_guard8_rdb1 = 0.0;
        var_guard8_rdb2 = 0.0;
        var_guard8_rdb3 = 0.0;
        var_guard8_rdb4 = 0.0;
        var_guard8_rdb5 = 0.0;
        var_guard8_rdb6 = 0.0;

        *var_arg_slot = var_arg;
        *var_arg_db0_slot = var_arg_db0;
        *var_arg_db1_slot = var_arg_db1;
        *var_arg_db2_slot = var_arg_db2;
        *var_arg_db3_slot = var_arg_db3;
        *var_arg_db4_slot = var_arg_db4;
        *var_arg_db5_slot = var_arg_db5;
        *var_arg_db6_slot = var_arg_db6;
        *var_arg_dn0_slot = var_arg_dn0;
        *var_arg_dn1_slot = var_arg_dn1;
        *var_arg_dn2_slot = var_arg_dn2;
        *var_arg_dn3_slot = var_arg_dn3;
        *var_arg_dn4_slot = var_arg_dn4;
        *var_arg_dn5_slot = var_arg_dn5;
        *var_arg_dn6_slot = var_arg_dn6;
        *var_arg_rdb0_slot = var_arg_rdb0;
        *var_arg_rdb1_slot = var_arg_rdb1;
        *var_arg_rdb2_slot = var_arg_rdb2;
        *var_arg_rdb3_slot = var_arg_rdb3;
        *var_arg_rdb4_slot = var_arg_rdb4;
        *var_arg_rdb5_slot = var_arg_rdb5;
        *var_arg_rdb6_slot = var_arg_rdb6;
        *var_arg_rdn0_slot = var_arg_rdn0;
        *var_arg_rdn1_slot = var_arg_rdn1;
        *var_arg_rdn2_slot = var_arg_rdn2;
        *var_arg_rdn3_slot = var_arg_rdn3;
        *var_arg_rdn4_slot = var_arg_rdn4;
        *var_arg_rdn5_slot = var_arg_rdn5;
        *var_arg_rdn6_slot = var_arg_rdn6;
        *var_arg_rv_slot = var_arg_rv;
        *var_guard5_slot = var_guard5;
        *var_guard5_db0_slot = var_guard5_db0;
        *var_guard5_db1_slot = var_guard5_db1;
        *var_guard5_db2_slot = var_guard5_db2;
        *var_guard5_db3_slot = var_guard5_db3;
        *var_guard5_db4_slot = var_guard5_db4;
        *var_guard5_db5_slot = var_guard5_db5;
        *var_guard5_db6_slot = var_guard5_db6;
        *var_guard5_dn0_slot = var_guard5_dn0;
        *var_guard5_dn1_slot = var_guard5_dn1;
        *var_guard5_dn2_slot = var_guard5_dn2;
        *var_guard5_dn3_slot = var_guard5_dn3;
        *var_guard5_dn4_slot = var_guard5_dn4;
        *var_guard5_dn5_slot = var_guard5_dn5;
        *var_guard5_dn6_slot = var_guard5_dn6;
        *var_guard5_rdb0_slot = var_guard5_rdb0;
        *var_guard5_rdb1_slot = var_guard5_rdb1;
        *var_guard5_rdb2_slot = var_guard5_rdb2;
        *var_guard5_rdb3_slot = var_guard5_rdb3;
        *var_guard5_rdb4_slot = var_guard5_rdb4;
        *var_guard5_rdb5_slot = var_guard5_rdb5;
        *var_guard5_rdb6_slot = var_guard5_rdb6;
        *var_guard5_rdn0_slot = var_guard5_rdn0;
        *var_guard5_rdn1_slot = var_guard5_rdn1;
        *var_guard5_rdn2_slot = var_guard5_rdn2;
        *var_guard5_rdn3_slot = var_guard5_rdn3;
        *var_guard5_rdn4_slot = var_guard5_rdn4;
        *var_guard5_rdn5_slot = var_guard5_rdn5;
        *var_guard5_rdn6_slot = var_guard5_rdn6;
        *var_guard5_rv_slot = var_guard5_rv;
        *var_guard6_slot = var_guard6;
        *var_guard6_db0_slot = var_guard6_db0;
        *var_guard6_db1_slot = var_guard6_db1;
        *var_guard6_db2_slot = var_guard6_db2;
        *var_guard6_db3_slot = var_guard6_db3;
        *var_guard6_db4_slot = var_guard6_db4;
        *var_guard6_db5_slot = var_guard6_db5;
        *var_guard6_db6_slot = var_guard6_db6;
        *var_guard6_dn0_slot = var_guard6_dn0;
        *var_guard6_dn1_slot = var_guard6_dn1;
        *var_guard6_dn2_slot = var_guard6_dn2;
        *var_guard6_dn3_slot = var_guard6_dn3;
        *var_guard6_dn4_slot = var_guard6_dn4;
        *var_guard6_dn5_slot = var_guard6_dn5;
        *var_guard6_dn6_slot = var_guard6_dn6;
        *var_guard6_rdb0_slot = var_guard6_rdb0;
        *var_guard6_rdb1_slot = var_guard6_rdb1;
        *var_guard6_rdb2_slot = var_guard6_rdb2;
        *var_guard6_rdb3_slot = var_guard6_rdb3;
        *var_guard6_rdb4_slot = var_guard6_rdb4;
        *var_guard6_rdb5_slot = var_guard6_rdb5;
        *var_guard6_rdb6_slot = var_guard6_rdb6;
        *var_guard6_rdn0_slot = var_guard6_rdn0;
        *var_guard6_rdn1_slot = var_guard6_rdn1;
        *var_guard6_rdn2_slot = var_guard6_rdn2;
        *var_guard6_rdn3_slot = var_guard6_rdn3;
        *var_guard6_rdn4_slot = var_guard6_rdn4;
        *var_guard6_rdn5_slot = var_guard6_rdn5;
        *var_guard6_rdn6_slot = var_guard6_rdn6;
        *var_guard6_rv_slot = var_guard6_rv;
        *var_guard8_slot = var_guard8;
        *var_guard8_db0_slot = var_guard8_db0;
        *var_guard8_db1_slot = var_guard8_db1;
        *var_guard8_db2_slot = var_guard8_db2;
        *var_guard8_db3_slot = var_guard8_db3;
        *var_guard8_db4_slot = var_guard8_db4;
        *var_guard8_db5_slot = var_guard8_db5;
        *var_guard8_db6_slot = var_guard8_db6;
        *var_guard8_dn0_slot = var_guard8_dn0;
        *var_guard8_dn1_slot = var_guard8_dn1;
        *var_guard8_dn2_slot = var_guard8_dn2;
        *var_guard8_dn3_slot = var_guard8_dn3;
        *var_guard8_dn4_slot = var_guard8_dn4;
        *var_guard8_dn5_slot = var_guard8_dn5;
        *var_guard8_dn6_slot = var_guard8_dn6;
        *var_guard8_rdb0_slot = var_guard8_rdb0;
        *var_guard8_rdb1_slot = var_guard8_rdb1;
        *var_guard8_rdb2_slot = var_guard8_rdb2;
        *var_guard8_rdb3_slot = var_guard8_rdb3;
        *var_guard8_rdb4_slot = var_guard8_rdb4;
        *var_guard8_rdb5_slot = var_guard8_rdb5;
        *var_guard8_rdb6_slot = var_guard8_rdb6;
        *var_guard8_rdn0_slot = var_guard8_rdn0;
        *var_guard8_rdn1_slot = var_guard8_rdn1;
        *var_guard8_rdn2_slot = var_guard8_rdn2;
        *var_guard8_rdn3_slot = var_guard8_rdn3;
        *var_guard8_rdn4_slot = var_guard8_rdn4;
        *var_guard8_rdn5_slot = var_guard8_rdn5;
        *var_guard8_rdn6_slot = var_guard8_rdn6;
        *var_guard8_rv_slot = var_guard8_rv;
        *var_itzf_slot = var_itzf;
        *var_itzf_db0_slot = var_itzf_db0;
        *var_itzf_db1_slot = var_itzf_db1;
        *var_itzf_db2_slot = var_itzf_db2;
        *var_itzf_db3_slot = var_itzf_db3;
        *var_itzf_db4_slot = var_itzf_db4;
        *var_itzf_db5_slot = var_itzf_db5;
        *var_itzf_db6_slot = var_itzf_db6;
        *var_itzf_dn0_slot = var_itzf_dn0;
        *var_itzf_dn1_slot = var_itzf_dn1;
        *var_itzf_dn2_slot = var_itzf_dn2;
        *var_itzf_dn3_slot = var_itzf_dn3;
        *var_itzf_dn4_slot = var_itzf_dn4;
        *var_itzf_dn5_slot = var_itzf_dn5;
        *var_itzf_dn6_slot = var_itzf_dn6;
        *var_itzf_rdb0_slot = var_itzf_rdb0;
        *var_itzf_rdb1_slot = var_itzf_rdb1;
        *var_itzf_rdb2_slot = var_itzf_rdb2;
        *var_itzf_rdb3_slot = var_itzf_rdb3;
        *var_itzf_rdb4_slot = var_itzf_rdb4;
        *var_itzf_rdb5_slot = var_itzf_rdb5;
        *var_itzf_rdb6_slot = var_itzf_rdb6;
        *var_itzf_rdn0_slot = var_itzf_rdn0;
        *var_itzf_rdn1_slot = var_itzf_rdn1;
        *var_itzf_rdn2_slot = var_itzf_rdn2;
        *var_itzf_rdn3_slot = var_itzf_rdn3;
        *var_itzf_rdn4_slot = var_itzf_rdn4;
        *var_itzf_rdn5_slot = var_itzf_rdn5;
        *var_itzf_rdn6_slot = var_itzf_rdn6;
        *var_itzf_rv_slot = var_itzf_rv;
        *var_le_slot = var_le;
        *var_le_db0_slot = var_le_db0;
        *var_le_db1_slot = var_le_db1;
        *var_le_db2_slot = var_le_db2;
        *var_le_db3_slot = var_le_db3;
        *var_le_db4_slot = var_le_db4;
        *var_le_db5_slot = var_le_db5;
        *var_le_db6_slot = var_le_db6;
        *var_le_dn0_slot = var_le_dn0;
        *var_le_dn1_slot = var_le_dn1;
        *var_le_dn2_slot = var_le_dn2;
        *var_le_dn3_slot = var_le_dn3;
        *var_le_dn4_slot = var_le_dn4;
        *var_le_dn5_slot = var_le_dn5;
        *var_le_dn6_slot = var_le_dn6;
        *var_le_rdb0_slot = var_le_rdb0;
        *var_le_rdb1_slot = var_le_rdb1;
        *var_le_rdb2_slot = var_le_rdb2;
        *var_le_rdb3_slot = var_le_rdb3;
        *var_le_rdb4_slot = var_le_rdb4;
        *var_le_rdb5_slot = var_le_rdb5;
        *var_le_rdb6_slot = var_le_rdb6;
        *var_le_rdn0_slot = var_le_rdn0;
        *var_le_rdn1_slot = var_le_rdn1;
        *var_le_rdn2_slot = var_le_rdn2;
        *var_le_rdn3_slot = var_le_rdn3;
        *var_le_rdn4_slot = var_le_rdn4;
        *var_le_rdn5_slot = var_le_rdn5;
        *var_le_rdn6_slot = var_le_rdn6;
        *var_le_rv_slot = var_le_rv;
        *var_qde_slot = var_qde;
        *var_qde_db0_slot = var_qde_db0;
        *var_qde_db1_slot = var_qde_db1;
        *var_qde_db2_slot = var_qde_db2;
        *var_qde_db3_slot = var_qde_db3;
        *var_qde_db4_slot = var_qde_db4;
        *var_qde_db5_slot = var_qde_db5;
        *var_qde_db6_slot = var_qde_db6;
        *var_qde_dn0_slot = var_qde_dn0;
        *var_qde_dn1_slot = var_qde_dn1;
        *var_qde_dn2_slot = var_qde_dn2;
        *var_qde_dn3_slot = var_qde_dn3;
        *var_qde_dn4_slot = var_qde_dn4;
        *var_qde_dn5_slot = var_qde_dn5;
        *var_qde_dn6_slot = var_qde_dn6;
        *var_qde_rdb0_slot = var_qde_rdb0;
        *var_qde_rdb1_slot = var_qde_rdb1;
        *var_qde_rdb2_slot = var_qde_rdb2;
        *var_qde_rdb3_slot = var_qde_rdb3;
        *var_qde_rdb4_slot = var_qde_rdb4;
        *var_qde_rdb5_slot = var_qde_rdb5;
        *var_qde_rdb6_slot = var_qde_rdb6;
        *var_qde_rdn0_slot = var_qde_rdn0;
        *var_qde_rdn1_slot = var_qde_rdn1;
        *var_qde_rdn2_slot = var_qde_rdn2;
        *var_qde_rdn3_slot = var_qde_rdn3;
        *var_qde_rdn4_slot = var_qde_rdn4;
        *var_qde_rdn5_slot = var_qde_rdn5;
        *var_qde_rdn6_slot = var_qde_rdn6;
        *var_qde_rv_slot = var_qde_rv;
        *var_t0_slot = var_t0;
        *var_t0_db0_slot = var_t0_db0;
        *var_t0_db1_slot = var_t0_db1;
        *var_t0_db2_slot = var_t0_db2;
        *var_t0_db3_slot = var_t0_db3;
        *var_t0_db4_slot = var_t0_db4;
        *var_t0_db5_slot = var_t0_db5;
        *var_t0_db6_slot = var_t0_db6;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn1_slot = var_t0_dn1;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_rdb0_slot = var_t0_rdb0;
        *var_t0_rdb1_slot = var_t0_rdb1;
        *var_t0_rdb2_slot = var_t0_rdb2;
        *var_t0_rdb3_slot = var_t0_rdb3;
        *var_t0_rdb4_slot = var_t0_rdb4;
        *var_t0_rdb5_slot = var_t0_rdb5;
        *var_t0_rdb6_slot = var_t0_rdb6;
        *var_t0_rdn0_slot = var_t0_rdn0;
        *var_t0_rdn1_slot = var_t0_rdn1;
        *var_t0_rdn2_slot = var_t0_rdn2;
        *var_t0_rdn3_slot = var_t0_rdn3;
        *var_t0_rdn4_slot = var_t0_rdn4;
        *var_t0_rdn5_slot = var_t0_rdn5;
        *var_t0_rdn6_slot = var_t0_rdn6;
        *var_t0_rv_slot = var_t0_rv;
        *var_tff_slot = var_tff;
        *var_tff_db0_slot = var_tff_db0;
        *var_tff_db1_slot = var_tff_db1;
        *var_tff_db2_slot = var_tff_db2;
        *var_tff_db3_slot = var_tff_db3;
        *var_tff_db4_slot = var_tff_db4;
        *var_tff_db5_slot = var_tff_db5;
        *var_tff_db6_slot = var_tff_db6;
        *var_tff_dn0_slot = var_tff_dn0;
        *var_tff_dn1_slot = var_tff_dn1;
        *var_tff_dn2_slot = var_tff_dn2;
        *var_tff_dn3_slot = var_tff_dn3;
        *var_tff_dn4_slot = var_tff_dn4;
        *var_tff_dn5_slot = var_tff_dn5;
        *var_tff_dn6_slot = var_tff_dn6;
        *var_tff_rdb0_slot = var_tff_rdb0;
        *var_tff_rdb1_slot = var_tff_rdb1;
        *var_tff_rdb2_slot = var_tff_rdb2;
        *var_tff_rdb3_slot = var_tff_rdb3;
        *var_tff_rdb4_slot = var_tff_rdb4;
        *var_tff_rdb5_slot = var_tff_rdb5;
        *var_tff_rdb6_slot = var_tff_rdb6;
        *var_tff_rdn0_slot = var_tff_rdn0;
        *var_tff_rdn1_slot = var_tff_rdn1;
        *var_tff_rdn2_slot = var_tff_rdn2;
        *var_tff_rdn3_slot = var_tff_rdn3;
        *var_tff_rdn4_slot = var_tff_rdn4;
        *var_tff_rdn5_slot = var_tff_rdn5;
        *var_tff_rdn6_slot = var_tff_rdn6;
        *var_tff_rv_slot = var_tff_rv;
        *var_vtff_slot = var_vtff;
        *var_vtff1_slot = var_vtff1;
        *var_vtff1_db0_slot = var_vtff1_db0;
        *var_vtff1_db1_slot = var_vtff1_db1;
        *var_vtff1_db2_slot = var_vtff1_db2;
        *var_vtff1_db3_slot = var_vtff1_db3;
        *var_vtff1_db4_slot = var_vtff1_db4;
        *var_vtff1_db5_slot = var_vtff1_db5;
        *var_vtff1_db6_slot = var_vtff1_db6;
        *var_vtff1_dn0_slot = var_vtff1_dn0;
        *var_vtff1_dn1_slot = var_vtff1_dn1;
        *var_vtff1_dn2_slot = var_vtff1_dn2;
        *var_vtff1_dn3_slot = var_vtff1_dn3;
        *var_vtff1_dn4_slot = var_vtff1_dn4;
        *var_vtff1_dn5_slot = var_vtff1_dn5;
        *var_vtff1_dn6_slot = var_vtff1_dn6;
        *var_vtff1_rdb0_slot = var_vtff1_rdb0;
        *var_vtff1_rdb1_slot = var_vtff1_rdb1;
        *var_vtff1_rdb2_slot = var_vtff1_rdb2;
        *var_vtff1_rdb3_slot = var_vtff1_rdb3;
        *var_vtff1_rdb4_slot = var_vtff1_rdb4;
        *var_vtff1_rdb5_slot = var_vtff1_rdb5;
        *var_vtff1_rdb6_slot = var_vtff1_rdb6;
        *var_vtff1_rdn0_slot = var_vtff1_rdn0;
        *var_vtff1_rdn1_slot = var_vtff1_rdn1;
        *var_vtff1_rdn2_slot = var_vtff1_rdn2;
        *var_vtff1_rdn3_slot = var_vtff1_rdn3;
        *var_vtff1_rdn4_slot = var_vtff1_rdn4;
        *var_vtff1_rdn5_slot = var_vtff1_rdn5;
        *var_vtff1_rdn6_slot = var_vtff1_rdn6;
        *var_vtff1_rv_slot = var_vtff1_rv;
        *var_vtff_db0_slot = var_vtff_db0;
        *var_vtff_db1_slot = var_vtff_db1;
        *var_vtff_db2_slot = var_vtff_db2;
        *var_vtff_db3_slot = var_vtff_db3;
        *var_vtff_db4_slot = var_vtff_db4;
        *var_vtff_db5_slot = var_vtff_db5;
        *var_vtff_db6_slot = var_vtff_db6;
        *var_vtff_dn0_slot = var_vtff_dn0;
        *var_vtff_dn1_slot = var_vtff_dn1;
        *var_vtff_dn2_slot = var_vtff_dn2;
        *var_vtff_dn3_slot = var_vtff_dn3;
        *var_vtff_dn4_slot = var_vtff_dn4;
        *var_vtff_dn5_slot = var_vtff_dn5;
        *var_vtff_dn6_slot = var_vtff_dn6;
        *var_vtff_rdb0_slot = var_vtff_rdb0;
        *var_vtff_rdb1_slot = var_vtff_rdb1;
        *var_vtff_rdb2_slot = var_vtff_rdb2;
        *var_vtff_rdb3_slot = var_vtff_rdb3;
        *var_vtff_rdb4_slot = var_vtff_rdb4;
        *var_vtff_rdb5_slot = var_vtff_rdb5;
        *var_vtff_rdb6_slot = var_vtff_rdb6;
        *var_vtff_rdn0_slot = var_vtff_rdn0;
        *var_vtff_rdn1_slot = var_vtff_rdn1;
        *var_vtff_rdn2_slot = var_vtff_rdn2;
        *var_vtff_rdn3_slot = var_vtff_rdn3;
        *var_vtff_rdn4_slot = var_vtff_rdn4;
        *var_vtff_rdn5_slot = var_vtff_rdn5;
        *var_vtff_rdn6_slot = var_vtff_rdn6;
        *var_vtff_rv_slot = var_vtff_rv;
    }

    pub(super) fn stamp_reactive_block_4(
        p: &Parameters,
        var_cje_t: f64,
        var_cje_t_db0: f64,
        var_cje_t_db1: f64,
        var_cje_t_db2: f64,
        var_cje_t_db3: f64,
        var_cje_t_db4: f64,
        var_cje_t_db5: f64,
        var_cje_t_db6: f64,
        var_cje_t_dn0: f64,
        var_cje_t_dn1: f64,
        var_cje_t_dn2: f64,
        var_cje_t_dn3: f64,
        var_cje_t_dn4: f64,
        var_cje_t_dn5: f64,
        var_cje_t_dn6: f64,
        var_vbiei: f64,
        var_vbiei_db0: f64,
        var_vbiei_db1: f64,
        var_vbiei_db2: f64,
        var_vbiei_db3: f64,
        var_vbiei_db4: f64,
        var_vbiei_db5: f64,
        var_vbiei_db6: f64,
        var_vbiei_dn0: f64,
        var_vbiei_dn1: f64,
        var_vbiei_dn2: f64,
        var_vbiei_dn3: f64,
        var_vbiei_dn4: f64,
        var_vbiei_dn5: f64,
        var_vbiei_dn6: f64,
        var_vje_t: f64,
        var_vje_t_db0: f64,
        var_vje_t_db1: f64,
        var_vje_t_db2: f64,
        var_vje_t_db3: f64,
        var_vje_t_db4: f64,
        var_vje_t_db5: f64,
        var_vje_t_db6: f64,
        var_vje_t_dn0: f64,
        var_vje_t_dn1: f64,
        var_vje_t_dn2: f64,
        var_vje_t_dn3: f64,
        var_vje_t_dn4: f64,
        var_vje_t_dn5: f64,
        var_vje_t_dn6: f64,
        var_dv0_slot: &mut f64,
        var_dv0_db0_slot: &mut f64,
        var_dv0_db1_slot: &mut f64,
        var_dv0_db2_slot: &mut f64,
        var_dv0_db3_slot: &mut f64,
        var_dv0_db4_slot: &mut f64,
        var_dv0_db5_slot: &mut f64,
        var_dv0_db6_slot: &mut f64,
        var_dv0_dn0_slot: &mut f64,
        var_dv0_dn1_slot: &mut f64,
        var_dv0_dn2_slot: &mut f64,
        var_dv0_dn3_slot: &mut f64,
        var_dv0_dn4_slot: &mut f64,
        var_dv0_dn5_slot: &mut f64,
        var_dv0_dn6_slot: &mut f64,
        var_dv0_rdb0_slot: &mut f64,
        var_dv0_rdb1_slot: &mut f64,
        var_dv0_rdb2_slot: &mut f64,
        var_dv0_rdb3_slot: &mut f64,
        var_dv0_rdb4_slot: &mut f64,
        var_dv0_rdb5_slot: &mut f64,
        var_dv0_rdb6_slot: &mut f64,
        var_dv0_rdn0_slot: &mut f64,
        var_dv0_rdn1_slot: &mut f64,
        var_dv0_rdn2_slot: &mut f64,
        var_dv0_rdn3_slot: &mut f64,
        var_dv0_rdn4_slot: &mut f64,
        var_dv0_rdn5_slot: &mut f64,
        var_dv0_rdn6_slot: &mut f64,
        var_dv0_rv_slot: &mut f64,
        var_dvh_slot: &mut f64,
        var_dvh_db0_slot: &mut f64,
        var_dvh_db1_slot: &mut f64,
        var_dvh_db2_slot: &mut f64,
        var_dvh_db3_slot: &mut f64,
        var_dvh_db4_slot: &mut f64,
        var_dvh_db5_slot: &mut f64,
        var_dvh_db6_slot: &mut f64,
        var_dvh_dn0_slot: &mut f64,
        var_dvh_dn1_slot: &mut f64,
        var_dvh_dn2_slot: &mut f64,
        var_dvh_dn3_slot: &mut f64,
        var_dvh_dn4_slot: &mut f64,
        var_dvh_dn5_slot: &mut f64,
        var_dvh_dn6_slot: &mut f64,
        var_dvh_rdb0_slot: &mut f64,
        var_dvh_rdb1_slot: &mut f64,
        var_dvh_rdb2_slot: &mut f64,
        var_dvh_rdb3_slot: &mut f64,
        var_dvh_rdb4_slot: &mut f64,
        var_dvh_rdb5_slot: &mut f64,
        var_dvh_rdb6_slot: &mut f64,
        var_dvh_rdn0_slot: &mut f64,
        var_dvh_rdn1_slot: &mut f64,
        var_dvh_rdn2_slot: &mut f64,
        var_dvh_rdn3_slot: &mut f64,
        var_dvh_rdn4_slot: &mut f64,
        var_dvh_rdn5_slot: &mut f64,
        var_dvh_rdn6_slot: &mut f64,
        var_dvh_rv_slot: &mut f64,
        var_guard10_slot: &mut f64,
        var_guard10_db0_slot: &mut f64,
        var_guard10_db1_slot: &mut f64,
        var_guard10_db2_slot: &mut f64,
        var_guard10_db3_slot: &mut f64,
        var_guard10_db4_slot: &mut f64,
        var_guard10_db5_slot: &mut f64,
        var_guard10_db6_slot: &mut f64,
        var_guard10_dn0_slot: &mut f64,
        var_guard10_dn1_slot: &mut f64,
        var_guard10_dn2_slot: &mut f64,
        var_guard10_dn3_slot: &mut f64,
        var_guard10_dn4_slot: &mut f64,
        var_guard10_dn5_slot: &mut f64,
        var_guard10_dn6_slot: &mut f64,
        var_guard10_rdb0_slot: &mut f64,
        var_guard10_rdb1_slot: &mut f64,
        var_guard10_rdb2_slot: &mut f64,
        var_guard10_rdb3_slot: &mut f64,
        var_guard10_rdb4_slot: &mut f64,
        var_guard10_rdb5_slot: &mut f64,
        var_guard10_rdb6_slot: &mut f64,
        var_guard10_rdn0_slot: &mut f64,
        var_guard10_rdn1_slot: &mut f64,
        var_guard10_rdn2_slot: &mut f64,
        var_guard10_rdn3_slot: &mut f64,
        var_guard10_rdn4_slot: &mut f64,
        var_guard10_rdn5_slot: &mut f64,
        var_guard10_rdn6_slot: &mut f64,
        var_guard10_rv_slot: &mut f64,
        var_guard11_slot: &mut f64,
        var_guard11_db0_slot: &mut f64,
        var_guard11_db1_slot: &mut f64,
        var_guard11_db2_slot: &mut f64,
        var_guard11_db3_slot: &mut f64,
        var_guard11_db4_slot: &mut f64,
        var_guard11_db5_slot: &mut f64,
        var_guard11_db6_slot: &mut f64,
        var_guard11_dn0_slot: &mut f64,
        var_guard11_dn1_slot: &mut f64,
        var_guard11_dn2_slot: &mut f64,
        var_guard11_dn3_slot: &mut f64,
        var_guard11_dn4_slot: &mut f64,
        var_guard11_dn5_slot: &mut f64,
        var_guard11_dn6_slot: &mut f64,
        var_guard11_rdb0_slot: &mut f64,
        var_guard11_rdb1_slot: &mut f64,
        var_guard11_rdb2_slot: &mut f64,
        var_guard11_rdb3_slot: &mut f64,
        var_guard11_rdb4_slot: &mut f64,
        var_guard11_rdb5_slot: &mut f64,
        var_guard11_rdb6_slot: &mut f64,
        var_guard11_rdn0_slot: &mut f64,
        var_guard11_rdn1_slot: &mut f64,
        var_guard11_rdn2_slot: &mut f64,
        var_guard11_rdn3_slot: &mut f64,
        var_guard11_rdn4_slot: &mut f64,
        var_guard11_rdn5_slot: &mut f64,
        var_guard11_rdn6_slot: &mut f64,
        var_guard11_rv_slot: &mut f64,
        var_guard9_slot: &mut f64,
        var_guard9_db0_slot: &mut f64,
        var_guard9_db1_slot: &mut f64,
        var_guard9_db2_slot: &mut f64,
        var_guard9_db3_slot: &mut f64,
        var_guard9_db4_slot: &mut f64,
        var_guard9_db5_slot: &mut f64,
        var_guard9_db6_slot: &mut f64,
        var_guard9_dn0_slot: &mut f64,
        var_guard9_dn1_slot: &mut f64,
        var_guard9_dn2_slot: &mut f64,
        var_guard9_dn3_slot: &mut f64,
        var_guard9_dn4_slot: &mut f64,
        var_guard9_dn5_slot: &mut f64,
        var_guard9_dn6_slot: &mut f64,
        var_guard9_rdb0_slot: &mut f64,
        var_guard9_rdb1_slot: &mut f64,
        var_guard9_rdb2_slot: &mut f64,
        var_guard9_rdb3_slot: &mut f64,
        var_guard9_rdb4_slot: &mut f64,
        var_guard9_rdb5_slot: &mut f64,
        var_guard9_rdb6_slot: &mut f64,
        var_guard9_rdn0_slot: &mut f64,
        var_guard9_rdn1_slot: &mut f64,
        var_guard9_rdn2_slot: &mut f64,
        var_guard9_rdn3_slot: &mut f64,
        var_guard9_rdn4_slot: &mut f64,
        var_guard9_rdn5_slot: &mut f64,
        var_guard9_rdn6_slot: &mut f64,
        var_guard9_rv_slot: &mut f64,
        var_pwq_slot: &mut f64,
        var_pwq_db0_slot: &mut f64,
        var_pwq_db1_slot: &mut f64,
        var_pwq_db2_slot: &mut f64,
        var_pwq_db3_slot: &mut f64,
        var_pwq_db4_slot: &mut f64,
        var_pwq_db5_slot: &mut f64,
        var_pwq_db6_slot: &mut f64,
        var_pwq_dn0_slot: &mut f64,
        var_pwq_dn1_slot: &mut f64,
        var_pwq_dn2_slot: &mut f64,
        var_pwq_dn3_slot: &mut f64,
        var_pwq_dn4_slot: &mut f64,
        var_pwq_dn5_slot: &mut f64,
        var_pwq_dn6_slot: &mut f64,
        var_pwq_rdb0_slot: &mut f64,
        var_pwq_rdb1_slot: &mut f64,
        var_pwq_rdb2_slot: &mut f64,
        var_pwq_rdb3_slot: &mut f64,
        var_pwq_rdb4_slot: &mut f64,
        var_pwq_rdb5_slot: &mut f64,
        var_pwq_rdb6_slot: &mut f64,
        var_pwq_rdn0_slot: &mut f64,
        var_pwq_rdn1_slot: &mut f64,
        var_pwq_rdn2_slot: &mut f64,
        var_pwq_rdn3_slot: &mut f64,
        var_pwq_rdn4_slot: &mut f64,
        var_pwq_rdn5_slot: &mut f64,
        var_pwq_rdn6_slot: &mut f64,
        var_pwq_rv_slot: &mut f64,
        var_qhi_slot: &mut f64,
        var_qhi_db0_slot: &mut f64,
        var_qhi_db1_slot: &mut f64,
        var_qhi_db2_slot: &mut f64,
        var_qhi_db3_slot: &mut f64,
        var_qhi_db4_slot: &mut f64,
        var_qhi_db5_slot: &mut f64,
        var_qhi_db6_slot: &mut f64,
        var_qhi_dn0_slot: &mut f64,
        var_qhi_dn1_slot: &mut f64,
        var_qhi_dn2_slot: &mut f64,
        var_qhi_dn3_slot: &mut f64,
        var_qhi_dn4_slot: &mut f64,
        var_qhi_dn5_slot: &mut f64,
        var_qhi_dn6_slot: &mut f64,
        var_qhi_rdb0_slot: &mut f64,
        var_qhi_rdb1_slot: &mut f64,
        var_qhi_rdb2_slot: &mut f64,
        var_qhi_rdb3_slot: &mut f64,
        var_qhi_rdb4_slot: &mut f64,
        var_qhi_rdb5_slot: &mut f64,
        var_qhi_rdb6_slot: &mut f64,
        var_qhi_rdn0_slot: &mut f64,
        var_qhi_rdn1_slot: &mut f64,
        var_qhi_rdn2_slot: &mut f64,
        var_qhi_rdn3_slot: &mut f64,
        var_qhi_rdn4_slot: &mut f64,
        var_qhi_rdn5_slot: &mut f64,
        var_qhi_rdn6_slot: &mut f64,
        var_qhi_rv_slot: &mut f64,
        var_qje_slot: &mut f64,
        var_qje_db0_slot: &mut f64,
        var_qje_db1_slot: &mut f64,
        var_qje_db2_slot: &mut f64,
        var_qje_db3_slot: &mut f64,
        var_qje_db4_slot: &mut f64,
        var_qje_db5_slot: &mut f64,
        var_qje_db6_slot: &mut f64,
        var_qje_dn0_slot: &mut f64,
        var_qje_dn1_slot: &mut f64,
        var_qje_dn2_slot: &mut f64,
        var_qje_dn3_slot: &mut f64,
        var_qje_dn4_slot: &mut f64,
        var_qje_dn5_slot: &mut f64,
        var_qje_dn6_slot: &mut f64,
        var_qje_rdb0_slot: &mut f64,
        var_qje_rdb1_slot: &mut f64,
        var_qje_rdb2_slot: &mut f64,
        var_qje_rdb3_slot: &mut f64,
        var_qje_rdb4_slot: &mut f64,
        var_qje_rdb5_slot: &mut f64,
        var_qje_rdb6_slot: &mut f64,
        var_qje_rdn0_slot: &mut f64,
        var_qje_rdn1_slot: &mut f64,
        var_qje_rdn2_slot: &mut f64,
        var_qje_rdn3_slot: &mut f64,
        var_qje_rdn4_slot: &mut f64,
        var_qje_rdn5_slot: &mut f64,
        var_qje_rdn6_slot: &mut f64,
        var_qje_rv_slot: &mut f64,
        var_qlo_slot: &mut f64,
        var_qlo_db0_slot: &mut f64,
        var_qlo_db1_slot: &mut f64,
        var_qlo_db2_slot: &mut f64,
        var_qlo_db3_slot: &mut f64,
        var_qlo_db4_slot: &mut f64,
        var_qlo_db5_slot: &mut f64,
        var_qlo_db6_slot: &mut f64,
        var_qlo_dn0_slot: &mut f64,
        var_qlo_dn1_slot: &mut f64,
        var_qlo_dn2_slot: &mut f64,
        var_qlo_dn3_slot: &mut f64,
        var_qlo_dn4_slot: &mut f64,
        var_qlo_dn5_slot: &mut f64,
        var_qlo_dn6_slot: &mut f64,
        var_qlo_rdb0_slot: &mut f64,
        var_qlo_rdb1_slot: &mut f64,
        var_qlo_rdb2_slot: &mut f64,
        var_qlo_rdb3_slot: &mut f64,
        var_qlo_rdb4_slot: &mut f64,
        var_qlo_rdb5_slot: &mut f64,
        var_qlo_rdb6_slot: &mut f64,
        var_qlo_rdn0_slot: &mut f64,
        var_qlo_rdn1_slot: &mut f64,
        var_qlo_rdn2_slot: &mut f64,
        var_qlo_rdn3_slot: &mut f64,
        var_qlo_rdn4_slot: &mut f64,
        var_qlo_rdn5_slot: &mut f64,
        var_qlo_rdn6_slot: &mut f64,
        var_qlo_rv_slot: &mut f64,
    ) {
        let mut var_dv0: f64 = *var_dv0_slot;
        let mut var_dv0_db0: f64 = *var_dv0_db0_slot;
        let mut var_dv0_db1: f64 = *var_dv0_db1_slot;
        let mut var_dv0_db2: f64 = *var_dv0_db2_slot;
        let mut var_dv0_db3: f64 = *var_dv0_db3_slot;
        let mut var_dv0_db4: f64 = *var_dv0_db4_slot;
        let mut var_dv0_db5: f64 = *var_dv0_db5_slot;
        let mut var_dv0_db6: f64 = *var_dv0_db6_slot;
        let mut var_dv0_dn0: f64 = *var_dv0_dn0_slot;
        let mut var_dv0_dn1: f64 = *var_dv0_dn1_slot;
        let mut var_dv0_dn2: f64 = *var_dv0_dn2_slot;
        let mut var_dv0_dn3: f64 = *var_dv0_dn3_slot;
        let mut var_dv0_dn4: f64 = *var_dv0_dn4_slot;
        let mut var_dv0_dn5: f64 = *var_dv0_dn5_slot;
        let mut var_dv0_dn6: f64 = *var_dv0_dn6_slot;
        let mut var_dv0_rdb0: f64 = *var_dv0_rdb0_slot;
        let mut var_dv0_rdb1: f64 = *var_dv0_rdb1_slot;
        let mut var_dv0_rdb2: f64 = *var_dv0_rdb2_slot;
        let mut var_dv0_rdb3: f64 = *var_dv0_rdb3_slot;
        let mut var_dv0_rdb4: f64 = *var_dv0_rdb4_slot;
        let mut var_dv0_rdb5: f64 = *var_dv0_rdb5_slot;
        let mut var_dv0_rdb6: f64 = *var_dv0_rdb6_slot;
        let mut var_dv0_rdn0: f64 = *var_dv0_rdn0_slot;
        let mut var_dv0_rdn1: f64 = *var_dv0_rdn1_slot;
        let mut var_dv0_rdn2: f64 = *var_dv0_rdn2_slot;
        let mut var_dv0_rdn3: f64 = *var_dv0_rdn3_slot;
        let mut var_dv0_rdn4: f64 = *var_dv0_rdn4_slot;
        let mut var_dv0_rdn5: f64 = *var_dv0_rdn5_slot;
        let mut var_dv0_rdn6: f64 = *var_dv0_rdn6_slot;
        let mut var_dv0_rv: f64 = *var_dv0_rv_slot;
        let mut var_dvh: f64 = *var_dvh_slot;
        let mut var_dvh_db0: f64 = *var_dvh_db0_slot;
        let mut var_dvh_db1: f64 = *var_dvh_db1_slot;
        let mut var_dvh_db2: f64 = *var_dvh_db2_slot;
        let mut var_dvh_db3: f64 = *var_dvh_db3_slot;
        let mut var_dvh_db4: f64 = *var_dvh_db4_slot;
        let mut var_dvh_db5: f64 = *var_dvh_db5_slot;
        let mut var_dvh_db6: f64 = *var_dvh_db6_slot;
        let mut var_dvh_dn0: f64 = *var_dvh_dn0_slot;
        let mut var_dvh_dn1: f64 = *var_dvh_dn1_slot;
        let mut var_dvh_dn2: f64 = *var_dvh_dn2_slot;
        let mut var_dvh_dn3: f64 = *var_dvh_dn3_slot;
        let mut var_dvh_dn4: f64 = *var_dvh_dn4_slot;
        let mut var_dvh_dn5: f64 = *var_dvh_dn5_slot;
        let mut var_dvh_dn6: f64 = *var_dvh_dn6_slot;
        let mut var_dvh_rdb0: f64 = *var_dvh_rdb0_slot;
        let mut var_dvh_rdb1: f64 = *var_dvh_rdb1_slot;
        let mut var_dvh_rdb2: f64 = *var_dvh_rdb2_slot;
        let mut var_dvh_rdb3: f64 = *var_dvh_rdb3_slot;
        let mut var_dvh_rdb4: f64 = *var_dvh_rdb4_slot;
        let mut var_dvh_rdb5: f64 = *var_dvh_rdb5_slot;
        let mut var_dvh_rdb6: f64 = *var_dvh_rdb6_slot;
        let mut var_dvh_rdn0: f64 = *var_dvh_rdn0_slot;
        let mut var_dvh_rdn1: f64 = *var_dvh_rdn1_slot;
        let mut var_dvh_rdn2: f64 = *var_dvh_rdn2_slot;
        let mut var_dvh_rdn3: f64 = *var_dvh_rdn3_slot;
        let mut var_dvh_rdn4: f64 = *var_dvh_rdn4_slot;
        let mut var_dvh_rdn5: f64 = *var_dvh_rdn5_slot;
        let mut var_dvh_rdn6: f64 = *var_dvh_rdn6_slot;
        let mut var_dvh_rv: f64 = *var_dvh_rv_slot;
        let mut var_guard10: f64 = *var_guard10_slot;
        let mut var_guard10_db0: f64 = *var_guard10_db0_slot;
        let mut var_guard10_db1: f64 = *var_guard10_db1_slot;
        let mut var_guard10_db2: f64 = *var_guard10_db2_slot;
        let mut var_guard10_db3: f64 = *var_guard10_db3_slot;
        let mut var_guard10_db4: f64 = *var_guard10_db4_slot;
        let mut var_guard10_db5: f64 = *var_guard10_db5_slot;
        let mut var_guard10_db6: f64 = *var_guard10_db6_slot;
        let mut var_guard10_dn0: f64 = *var_guard10_dn0_slot;
        let mut var_guard10_dn1: f64 = *var_guard10_dn1_slot;
        let mut var_guard10_dn2: f64 = *var_guard10_dn2_slot;
        let mut var_guard10_dn3: f64 = *var_guard10_dn3_slot;
        let mut var_guard10_dn4: f64 = *var_guard10_dn4_slot;
        let mut var_guard10_dn5: f64 = *var_guard10_dn5_slot;
        let mut var_guard10_dn6: f64 = *var_guard10_dn6_slot;
        let mut var_guard10_rdb0: f64 = *var_guard10_rdb0_slot;
        let mut var_guard10_rdb1: f64 = *var_guard10_rdb1_slot;
        let mut var_guard10_rdb2: f64 = *var_guard10_rdb2_slot;
        let mut var_guard10_rdb3: f64 = *var_guard10_rdb3_slot;
        let mut var_guard10_rdb4: f64 = *var_guard10_rdb4_slot;
        let mut var_guard10_rdb5: f64 = *var_guard10_rdb5_slot;
        let mut var_guard10_rdb6: f64 = *var_guard10_rdb6_slot;
        let mut var_guard10_rdn0: f64 = *var_guard10_rdn0_slot;
        let mut var_guard10_rdn1: f64 = *var_guard10_rdn1_slot;
        let mut var_guard10_rdn2: f64 = *var_guard10_rdn2_slot;
        let mut var_guard10_rdn3: f64 = *var_guard10_rdn3_slot;
        let mut var_guard10_rdn4: f64 = *var_guard10_rdn4_slot;
        let mut var_guard10_rdn5: f64 = *var_guard10_rdn5_slot;
        let mut var_guard10_rdn6: f64 = *var_guard10_rdn6_slot;
        let mut var_guard10_rv: f64 = *var_guard10_rv_slot;
        let mut var_guard11: f64 = *var_guard11_slot;
        let mut var_guard11_db0: f64 = *var_guard11_db0_slot;
        let mut var_guard11_db1: f64 = *var_guard11_db1_slot;
        let mut var_guard11_db2: f64 = *var_guard11_db2_slot;
        let mut var_guard11_db3: f64 = *var_guard11_db3_slot;
        let mut var_guard11_db4: f64 = *var_guard11_db4_slot;
        let mut var_guard11_db5: f64 = *var_guard11_db5_slot;
        let mut var_guard11_db6: f64 = *var_guard11_db6_slot;
        let mut var_guard11_dn0: f64 = *var_guard11_dn0_slot;
        let mut var_guard11_dn1: f64 = *var_guard11_dn1_slot;
        let mut var_guard11_dn2: f64 = *var_guard11_dn2_slot;
        let mut var_guard11_dn3: f64 = *var_guard11_dn3_slot;
        let mut var_guard11_dn4: f64 = *var_guard11_dn4_slot;
        let mut var_guard11_dn5: f64 = *var_guard11_dn5_slot;
        let mut var_guard11_dn6: f64 = *var_guard11_dn6_slot;
        let mut var_guard11_rdb0: f64 = *var_guard11_rdb0_slot;
        let mut var_guard11_rdb1: f64 = *var_guard11_rdb1_slot;
        let mut var_guard11_rdb2: f64 = *var_guard11_rdb2_slot;
        let mut var_guard11_rdb3: f64 = *var_guard11_rdb3_slot;
        let mut var_guard11_rdb4: f64 = *var_guard11_rdb4_slot;
        let mut var_guard11_rdb5: f64 = *var_guard11_rdb5_slot;
        let mut var_guard11_rdb6: f64 = *var_guard11_rdb6_slot;
        let mut var_guard11_rdn0: f64 = *var_guard11_rdn0_slot;
        let mut var_guard11_rdn1: f64 = *var_guard11_rdn1_slot;
        let mut var_guard11_rdn2: f64 = *var_guard11_rdn2_slot;
        let mut var_guard11_rdn3: f64 = *var_guard11_rdn3_slot;
        let mut var_guard11_rdn4: f64 = *var_guard11_rdn4_slot;
        let mut var_guard11_rdn5: f64 = *var_guard11_rdn5_slot;
        let mut var_guard11_rdn6: f64 = *var_guard11_rdn6_slot;
        let mut var_guard11_rv: f64 = *var_guard11_rv_slot;
        let mut var_guard9: f64 = *var_guard9_slot;
        let mut var_guard9_db0: f64 = *var_guard9_db0_slot;
        let mut var_guard9_db1: f64 = *var_guard9_db1_slot;
        let mut var_guard9_db2: f64 = *var_guard9_db2_slot;
        let mut var_guard9_db3: f64 = *var_guard9_db3_slot;
        let mut var_guard9_db4: f64 = *var_guard9_db4_slot;
        let mut var_guard9_db5: f64 = *var_guard9_db5_slot;
        let mut var_guard9_db6: f64 = *var_guard9_db6_slot;
        let mut var_guard9_dn0: f64 = *var_guard9_dn0_slot;
        let mut var_guard9_dn1: f64 = *var_guard9_dn1_slot;
        let mut var_guard9_dn2: f64 = *var_guard9_dn2_slot;
        let mut var_guard9_dn3: f64 = *var_guard9_dn3_slot;
        let mut var_guard9_dn4: f64 = *var_guard9_dn4_slot;
        let mut var_guard9_dn5: f64 = *var_guard9_dn5_slot;
        let mut var_guard9_dn6: f64 = *var_guard9_dn6_slot;
        let mut var_guard9_rdb0: f64 = *var_guard9_rdb0_slot;
        let mut var_guard9_rdb1: f64 = *var_guard9_rdb1_slot;
        let mut var_guard9_rdb2: f64 = *var_guard9_rdb2_slot;
        let mut var_guard9_rdb3: f64 = *var_guard9_rdb3_slot;
        let mut var_guard9_rdb4: f64 = *var_guard9_rdb4_slot;
        let mut var_guard9_rdb5: f64 = *var_guard9_rdb5_slot;
        let mut var_guard9_rdb6: f64 = *var_guard9_rdb6_slot;
        let mut var_guard9_rdn0: f64 = *var_guard9_rdn0_slot;
        let mut var_guard9_rdn1: f64 = *var_guard9_rdn1_slot;
        let mut var_guard9_rdn2: f64 = *var_guard9_rdn2_slot;
        let mut var_guard9_rdn3: f64 = *var_guard9_rdn3_slot;
        let mut var_guard9_rdn4: f64 = *var_guard9_rdn4_slot;
        let mut var_guard9_rdn5: f64 = *var_guard9_rdn5_slot;
        let mut var_guard9_rdn6: f64 = *var_guard9_rdn6_slot;
        let mut var_guard9_rv: f64 = *var_guard9_rv_slot;
        let mut var_pwq: f64 = *var_pwq_slot;
        let mut var_pwq_db0: f64 = *var_pwq_db0_slot;
        let mut var_pwq_db1: f64 = *var_pwq_db1_slot;
        let mut var_pwq_db2: f64 = *var_pwq_db2_slot;
        let mut var_pwq_db3: f64 = *var_pwq_db3_slot;
        let mut var_pwq_db4: f64 = *var_pwq_db4_slot;
        let mut var_pwq_db5: f64 = *var_pwq_db5_slot;
        let mut var_pwq_db6: f64 = *var_pwq_db6_slot;
        let mut var_pwq_dn0: f64 = *var_pwq_dn0_slot;
        let mut var_pwq_dn1: f64 = *var_pwq_dn1_slot;
        let mut var_pwq_dn2: f64 = *var_pwq_dn2_slot;
        let mut var_pwq_dn3: f64 = *var_pwq_dn3_slot;
        let mut var_pwq_dn4: f64 = *var_pwq_dn4_slot;
        let mut var_pwq_dn5: f64 = *var_pwq_dn5_slot;
        let mut var_pwq_dn6: f64 = *var_pwq_dn6_slot;
        let mut var_pwq_rdb0: f64 = *var_pwq_rdb0_slot;
        let mut var_pwq_rdb1: f64 = *var_pwq_rdb1_slot;
        let mut var_pwq_rdb2: f64 = *var_pwq_rdb2_slot;
        let mut var_pwq_rdb3: f64 = *var_pwq_rdb3_slot;
        let mut var_pwq_rdb4: f64 = *var_pwq_rdb4_slot;
        let mut var_pwq_rdb5: f64 = *var_pwq_rdb5_slot;
        let mut var_pwq_rdb6: f64 = *var_pwq_rdb6_slot;
        let mut var_pwq_rdn0: f64 = *var_pwq_rdn0_slot;
        let mut var_pwq_rdn1: f64 = *var_pwq_rdn1_slot;
        let mut var_pwq_rdn2: f64 = *var_pwq_rdn2_slot;
        let mut var_pwq_rdn3: f64 = *var_pwq_rdn3_slot;
        let mut var_pwq_rdn4: f64 = *var_pwq_rdn4_slot;
        let mut var_pwq_rdn5: f64 = *var_pwq_rdn5_slot;
        let mut var_pwq_rdn6: f64 = *var_pwq_rdn6_slot;
        let mut var_pwq_rv: f64 = *var_pwq_rv_slot;
        let mut var_qhi: f64 = *var_qhi_slot;
        let mut var_qhi_db0: f64 = *var_qhi_db0_slot;
        let mut var_qhi_db1: f64 = *var_qhi_db1_slot;
        let mut var_qhi_db2: f64 = *var_qhi_db2_slot;
        let mut var_qhi_db3: f64 = *var_qhi_db3_slot;
        let mut var_qhi_db4: f64 = *var_qhi_db4_slot;
        let mut var_qhi_db5: f64 = *var_qhi_db5_slot;
        let mut var_qhi_db6: f64 = *var_qhi_db6_slot;
        let mut var_qhi_dn0: f64 = *var_qhi_dn0_slot;
        let mut var_qhi_dn1: f64 = *var_qhi_dn1_slot;
        let mut var_qhi_dn2: f64 = *var_qhi_dn2_slot;
        let mut var_qhi_dn3: f64 = *var_qhi_dn3_slot;
        let mut var_qhi_dn4: f64 = *var_qhi_dn4_slot;
        let mut var_qhi_dn5: f64 = *var_qhi_dn5_slot;
        let mut var_qhi_dn6: f64 = *var_qhi_dn6_slot;
        let mut var_qhi_rdb0: f64 = *var_qhi_rdb0_slot;
        let mut var_qhi_rdb1: f64 = *var_qhi_rdb1_slot;
        let mut var_qhi_rdb2: f64 = *var_qhi_rdb2_slot;
        let mut var_qhi_rdb3: f64 = *var_qhi_rdb3_slot;
        let mut var_qhi_rdb4: f64 = *var_qhi_rdb4_slot;
        let mut var_qhi_rdb5: f64 = *var_qhi_rdb5_slot;
        let mut var_qhi_rdb6: f64 = *var_qhi_rdb6_slot;
        let mut var_qhi_rdn0: f64 = *var_qhi_rdn0_slot;
        let mut var_qhi_rdn1: f64 = *var_qhi_rdn1_slot;
        let mut var_qhi_rdn2: f64 = *var_qhi_rdn2_slot;
        let mut var_qhi_rdn3: f64 = *var_qhi_rdn3_slot;
        let mut var_qhi_rdn4: f64 = *var_qhi_rdn4_slot;
        let mut var_qhi_rdn5: f64 = *var_qhi_rdn5_slot;
        let mut var_qhi_rdn6: f64 = *var_qhi_rdn6_slot;
        let mut var_qhi_rv: f64 = *var_qhi_rv_slot;
        let mut var_qje: f64 = *var_qje_slot;
        let mut var_qje_db0: f64 = *var_qje_db0_slot;
        let mut var_qje_db1: f64 = *var_qje_db1_slot;
        let mut var_qje_db2: f64 = *var_qje_db2_slot;
        let mut var_qje_db3: f64 = *var_qje_db3_slot;
        let mut var_qje_db4: f64 = *var_qje_db4_slot;
        let mut var_qje_db5: f64 = *var_qje_db5_slot;
        let mut var_qje_db6: f64 = *var_qje_db6_slot;
        let mut var_qje_dn0: f64 = *var_qje_dn0_slot;
        let mut var_qje_dn1: f64 = *var_qje_dn1_slot;
        let mut var_qje_dn2: f64 = *var_qje_dn2_slot;
        let mut var_qje_dn3: f64 = *var_qje_dn3_slot;
        let mut var_qje_dn4: f64 = *var_qje_dn4_slot;
        let mut var_qje_dn5: f64 = *var_qje_dn5_slot;
        let mut var_qje_dn6: f64 = *var_qje_dn6_slot;
        let mut var_qje_rdb0: f64 = *var_qje_rdb0_slot;
        let mut var_qje_rdb1: f64 = *var_qje_rdb1_slot;
        let mut var_qje_rdb2: f64 = *var_qje_rdb2_slot;
        let mut var_qje_rdb3: f64 = *var_qje_rdb3_slot;
        let mut var_qje_rdb4: f64 = *var_qje_rdb4_slot;
        let mut var_qje_rdb5: f64 = *var_qje_rdb5_slot;
        let mut var_qje_rdb6: f64 = *var_qje_rdb6_slot;
        let mut var_qje_rdn0: f64 = *var_qje_rdn0_slot;
        let mut var_qje_rdn1: f64 = *var_qje_rdn1_slot;
        let mut var_qje_rdn2: f64 = *var_qje_rdn2_slot;
        let mut var_qje_rdn3: f64 = *var_qje_rdn3_slot;
        let mut var_qje_rdn4: f64 = *var_qje_rdn4_slot;
        let mut var_qje_rdn5: f64 = *var_qje_rdn5_slot;
        let mut var_qje_rdn6: f64 = *var_qje_rdn6_slot;
        let mut var_qje_rv: f64 = *var_qje_rv_slot;
        let mut var_qlo: f64 = *var_qlo_slot;
        let mut var_qlo_db0: f64 = *var_qlo_db0_slot;
        let mut var_qlo_db1: f64 = *var_qlo_db1_slot;
        let mut var_qlo_db2: f64 = *var_qlo_db2_slot;
        let mut var_qlo_db3: f64 = *var_qlo_db3_slot;
        let mut var_qlo_db4: f64 = *var_qlo_db4_slot;
        let mut var_qlo_db5: f64 = *var_qlo_db5_slot;
        let mut var_qlo_db6: f64 = *var_qlo_db6_slot;
        let mut var_qlo_dn0: f64 = *var_qlo_dn0_slot;
        let mut var_qlo_dn1: f64 = *var_qlo_dn1_slot;
        let mut var_qlo_dn2: f64 = *var_qlo_dn2_slot;
        let mut var_qlo_dn3: f64 = *var_qlo_dn3_slot;
        let mut var_qlo_dn4: f64 = *var_qlo_dn4_slot;
        let mut var_qlo_dn5: f64 = *var_qlo_dn5_slot;
        let mut var_qlo_dn6: f64 = *var_qlo_dn6_slot;
        let mut var_qlo_rdb0: f64 = *var_qlo_rdb0_slot;
        let mut var_qlo_rdb1: f64 = *var_qlo_rdb1_slot;
        let mut var_qlo_rdb2: f64 = *var_qlo_rdb2_slot;
        let mut var_qlo_rdb3: f64 = *var_qlo_rdb3_slot;
        let mut var_qlo_rdb4: f64 = *var_qlo_rdb4_slot;
        let mut var_qlo_rdb5: f64 = *var_qlo_rdb5_slot;
        let mut var_qlo_rdb6: f64 = *var_qlo_rdb6_slot;
        let mut var_qlo_rdn0: f64 = *var_qlo_rdn0_slot;
        let mut var_qlo_rdn1: f64 = *var_qlo_rdn1_slot;
        let mut var_qlo_rdn2: f64 = *var_qlo_rdn2_slot;
        let mut var_qlo_rdn3: f64 = *var_qlo_rdn3_slot;
        let mut var_qlo_rdn4: f64 = *var_qlo_rdn4_slot;
        let mut var_qlo_rdn5: f64 = *var_qlo_rdn5_slot;
        let mut var_qlo_rdn6: f64 = *var_qlo_rdn6_slot;
        let mut var_qlo_rv: f64 = *var_qlo_rv_slot;

        let assign700_e841: f64 = (-var_vje_t);
        let assign700_e843: f64 = (assign700_e841 * p.p24);
        var_dv0 = assign700_e843;
        var_dv0_dn0 = ((-var_vje_t_dn0) * p.p24);
        var_dv0_dn1 = ((-var_vje_t_dn1) * p.p24);
        var_dv0_dn2 = ((-var_vje_t_dn2) * p.p24);
        var_dv0_dn3 = ((-var_vje_t_dn3) * p.p24);
        var_dv0_dn4 = ((-var_vje_t_dn4) * p.p24);
        var_dv0_dn5 = ((-var_vje_t_dn5) * p.p24);
        var_dv0_dn6 = ((-var_vje_t_dn6) * p.p24);
        var_dv0_db0 = ((-var_vje_t_db0) * p.p24);
        var_dv0_db1 = ((-var_vje_t_db1) * p.p24);
        var_dv0_db2 = ((-var_vje_t_db2) * p.p24);
        var_dv0_db3 = ((-var_vje_t_db3) * p.p24);
        var_dv0_db4 = ((-var_vje_t_db4) * p.p24);
        var_dv0_db5 = ((-var_vje_t_db5) * p.p24);
        var_dv0_db6 = ((-var_vje_t_db6) * p.p24);
        var_dv0_rv = 0.0;
        var_dv0_rdn0 = 0.0;
        var_dv0_rdn1 = 0.0;
        var_dv0_rdn2 = 0.0;
        var_dv0_rdn3 = 0.0;
        var_dv0_rdn4 = 0.0;
        var_dv0_rdn5 = 0.0;
        var_dv0_rdn6 = 0.0;
        var_dv0_rdb0 = 0.0;
        var_dv0_rdb1 = 0.0;
        var_dv0_rdb2 = 0.0;
        var_dv0_rdb3 = 0.0;
        var_dv0_rdb4 = 0.0;
        var_dv0_rdb5 = 0.0;
        var_dv0_rdb6 = 0.0;

        let assign710_e846: f64 = (var_vbiei + var_dv0);
        var_dvh = assign710_e846;
        var_dvh_dn0 = (var_vbiei_dn0 + var_dv0_dn0);
        var_dvh_dn1 = (var_vbiei_dn1 + var_dv0_dn1);
        var_dvh_dn2 = (var_vbiei_dn2 + var_dv0_dn2);
        var_dvh_dn3 = (var_vbiei_dn3 + var_dv0_dn3);
        var_dvh_dn4 = (var_vbiei_dn4 + var_dv0_dn4);
        var_dvh_dn5 = (var_vbiei_dn5 + var_dv0_dn5);
        var_dvh_dn6 = (var_vbiei_dn6 + var_dv0_dn6);
        var_dvh_db0 = (var_vbiei_db0 + var_dv0_db0);
        var_dvh_db1 = (var_vbiei_db1 + var_dv0_db1);
        var_dvh_db2 = (var_vbiei_db2 + var_dv0_db2);
        var_dvh_db3 = (var_vbiei_db3 + var_dv0_db3);
        var_dvh_db4 = (var_vbiei_db4 + var_dv0_db4);
        var_dvh_db5 = (var_vbiei_db5 + var_dv0_db5);
        var_dvh_db6 = (var_vbiei_db6 + var_dv0_db6);
        var_dvh_rv = 0.0;
        var_dvh_rdn0 = 0.0;
        var_dvh_rdn1 = 0.0;
        var_dvh_rdn2 = 0.0;
        var_dvh_rdn3 = 0.0;
        var_dvh_rdn4 = 0.0;
        var_dvh_rdn5 = 0.0;
        var_dvh_rdn6 = 0.0;
        var_dvh_rdb0 = 0.0;
        var_dvh_rdb1 = 0.0;
        var_dvh_rdb2 = 0.0;
        var_dvh_rdb3 = 0.0;
        var_dvh_rdb4 = 0.0;
        var_dvh_rdb5 = 0.0;
        var_dvh_rdb6 = 0.0;

        let assign720_e849: f64 = if var_dvh > 0.0 { 1.0 } else { 0.0 };
        var_guard9 = assign720_e849;
        var_guard9_dn0 = 0.0;
        var_guard9_dn1 = 0.0;
        var_guard9_dn2 = 0.0;
        var_guard9_dn3 = 0.0;
        var_guard9_dn4 = 0.0;
        var_guard9_dn5 = 0.0;
        var_guard9_dn6 = 0.0;
        var_guard9_db0 = 0.0;
        var_guard9_db1 = 0.0;
        var_guard9_db2 = 0.0;
        var_guard9_db3 = 0.0;
        var_guard9_db4 = 0.0;
        var_guard9_db5 = 0.0;
        var_guard9_db6 = 0.0;
        var_guard9_rv = 0.0;
        var_guard9_rdn0 = 0.0;
        var_guard9_rdn1 = 0.0;
        var_guard9_rdn2 = 0.0;
        var_guard9_rdn3 = 0.0;
        var_guard9_rdn4 = 0.0;
        var_guard9_rdn5 = 0.0;
        var_guard9_rdn6 = 0.0;
        var_guard9_rdb0 = 0.0;
        var_guard9_rdb1 = 0.0;
        var_guard9_rdb2 = 0.0;
        var_guard9_rdb3 = 0.0;
        var_guard9_rdb4 = 0.0;
        var_guard9_rdb5 = 0.0;
        var_guard9_rdb6 = 0.0;

        let (assign730_e862, assign730_e862_d_n0, assign730_e862_d_n1, assign730_e862_d_n2, assign730_e862_d_n3, assign730_e862_d_n4, assign730_e862_d_n5, assign730_e862_d_n6, assign730_e862_d_b0, assign730_e862_d_b1, assign730_e862_d_b2, assign730_e862_d_b3, assign730_e862_d_b4, assign730_e862_d_b5, assign730_e862_d_b6,) = {
    if (var_guard9 != 0.0) {
        let assign730_e852: f64 = (-1.0);
        let assign730_e854: f64 = (assign730_e852 - p.p18);
        let assign730_e857: f64 = (1.0 - p.p24);
        let assign730_e858: f64 = (assign730_e857).ln();
        let assign730_e859: f64 = (assign730_e854 * assign730_e858);
        let assign730_e860: f64 = (assign730_e859).exp();
        (assign730_e860, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_pwq, var_pwq_dn0, var_pwq_dn1, var_pwq_dn2, var_pwq_dn3, var_pwq_dn4, var_pwq_dn5, var_pwq_dn6, var_pwq_db0, var_pwq_db1, var_pwq_db2, var_pwq_db3, var_pwq_db4, var_pwq_db5, var_pwq_db6,)
    }
};
        var_pwq = assign730_e862;
        var_pwq_dn0 = assign730_e862_d_n0;
        var_pwq_dn1 = assign730_e862_d_n1;
        var_pwq_dn2 = assign730_e862_d_n2;
        var_pwq_dn3 = assign730_e862_d_n3;
        var_pwq_dn4 = assign730_e862_d_n4;
        var_pwq_dn5 = assign730_e862_d_n5;
        var_pwq_dn6 = assign730_e862_d_n6;
        var_pwq_db0 = assign730_e862_d_b0;
        var_pwq_db1 = assign730_e862_d_b1;
        var_pwq_db2 = assign730_e862_d_b2;
        var_pwq_db3 = assign730_e862_d_b3;
        var_pwq_db4 = assign730_e862_d_b4;
        var_pwq_db5 = assign730_e862_d_b5;
        var_pwq_db6 = assign730_e862_d_b6;
        var_pwq_rv = 0.0;
        var_pwq_rdn0 = 0.0;
        var_pwq_rdn1 = 0.0;
        var_pwq_rdn2 = 0.0;
        var_pwq_rdn3 = 0.0;
        var_pwq_rdn4 = 0.0;
        var_pwq_rdn5 = 0.0;
        var_pwq_rdn6 = 0.0;
        var_pwq_rdb0 = 0.0;
        var_pwq_rdb1 = 0.0;
        var_pwq_rdb2 = 0.0;
        var_pwq_rdb3 = 0.0;
        var_pwq_rdb4 = 0.0;
        var_pwq_rdb5 = 0.0;
        var_pwq_rdb6 = 0.0;

        let (assign740_e882, assign740_e882_d_n0, assign740_e882_d_n1, assign740_e882_d_n2, assign740_e882_d_n3, assign740_e882_d_n4, assign740_e882_d_n5, assign740_e882_d_n6, assign740_e882_d_b0, assign740_e882_d_b1, assign740_e882_d_b2, assign740_e882_d_b3, assign740_e882_d_b4, assign740_e882_d_b5, assign740_e882_d_b6,) = {
    if (var_guard9 != 0.0) {
        let assign740_e869: f64 = (1.0 - p.p24);
        let assign740_e870: f64 = (var_pwq * assign740_e869);
        let assign740_e873: f64 = (1.0 - p.p24);
        let assign740_e874: f64 = (assign740_e870 * assign740_e873);
        let assign740_e875: f64 = (1.0 - assign740_e874);
        let assign740_e876: f64 = (var_vje_t * assign740_e875);
        let assign740_e879: f64 = (1.0 - p.p18);
        let assign740_e880: f64 = (assign740_e876 / assign740_e879);
        (assign740_e880, (((var_vje_t_dn0 * assign740_e875) + (var_vje_t * (-((var_pwq_dn0 * assign740_e869) * assign740_e873)))) / assign740_e879), (((var_vje_t_dn1 * assign740_e875) + (var_vje_t * (-((var_pwq_dn1 * assign740_e869) * assign740_e873)))) / assign740_e879), (((var_vje_t_dn2 * assign740_e875) + (var_vje_t * (-((var_pwq_dn2 * assign740_e869) * assign740_e873)))) / assign740_e879), (((var_vje_t_dn3 * assign740_e875) + (var_vje_t * (-((var_pwq_dn3 * assign740_e869) * assign740_e873)))) / assign740_e879), (((var_vje_t_dn4 * assign740_e875) + (var_vje_t * (-((var_pwq_dn4 * assign740_e869) * assign740_e873)))) / assign740_e879), (((var_vje_t_dn5 * assign740_e875) + (var_vje_t * (-((var_pwq_dn5 * assign740_e869) * assign740_e873)))) / assign740_e879), (((var_vje_t_dn6 * assign740_e875) + (var_vje_t * (-((var_pwq_dn6 * assign740_e869) * assign740_e873)))) / assign740_e879), (((var_vje_t_db0 * assign740_e875) + (var_vje_t * (-((var_pwq_db0 * assign740_e869) * assign740_e873)))) / assign740_e879), (((var_vje_t_db1 * assign740_e875) + (var_vje_t * (-((var_pwq_db1 * assign740_e869) * assign740_e873)))) / assign740_e879), (((var_vje_t_db2 * assign740_e875) + (var_vje_t * (-((var_pwq_db2 * assign740_e869) * assign740_e873)))) / assign740_e879), (((var_vje_t_db3 * assign740_e875) + (var_vje_t * (-((var_pwq_db3 * assign740_e869) * assign740_e873)))) / assign740_e879), (((var_vje_t_db4 * assign740_e875) + (var_vje_t * (-((var_pwq_db4 * assign740_e869) * assign740_e873)))) / assign740_e879), (((var_vje_t_db5 * assign740_e875) + (var_vje_t * (-((var_pwq_db5 * assign740_e869) * assign740_e873)))) / assign740_e879), (((var_vje_t_db6 * assign740_e875) + (var_vje_t * (-((var_pwq_db6 * assign740_e869) * assign740_e873)))) / assign740_e879),)
    } else {
        (var_qlo, var_qlo_dn0, var_qlo_dn1, var_qlo_dn2, var_qlo_dn3, var_qlo_dn4, var_qlo_dn5, var_qlo_dn6, var_qlo_db0, var_qlo_db1, var_qlo_db2, var_qlo_db3, var_qlo_db4, var_qlo_db5, var_qlo_db6,)
    }
};
        var_qlo = assign740_e882;
        var_qlo_dn0 = assign740_e882_d_n0;
        var_qlo_dn1 = assign740_e882_d_n1;
        var_qlo_dn2 = assign740_e882_d_n2;
        var_qlo_dn3 = assign740_e882_d_n3;
        var_qlo_dn4 = assign740_e882_d_n4;
        var_qlo_dn5 = assign740_e882_d_n5;
        var_qlo_dn6 = assign740_e882_d_n6;
        var_qlo_db0 = assign740_e882_d_b0;
        var_qlo_db1 = assign740_e882_d_b1;
        var_qlo_db2 = assign740_e882_d_b2;
        var_qlo_db3 = assign740_e882_d_b3;
        var_qlo_db4 = assign740_e882_d_b4;
        var_qlo_db5 = assign740_e882_d_b5;
        var_qlo_db6 = assign740_e882_d_b6;
        var_qlo_rv = 0.0;
        var_qlo_rdn0 = 0.0;
        var_qlo_rdn1 = 0.0;
        var_qlo_rdn2 = 0.0;
        var_qlo_rdn3 = 0.0;
        var_qlo_rdn4 = 0.0;
        var_qlo_rdn5 = 0.0;
        var_qlo_rdn6 = 0.0;
        var_qlo_rdb0 = 0.0;
        var_qlo_rdb1 = 0.0;
        var_qlo_rdb2 = 0.0;
        var_qlo_rdb3 = 0.0;
        var_qlo_rdb4 = 0.0;
        var_qlo_rdb5 = 0.0;
        var_qlo_rdb6 = 0.0;

        let (assign750_e900, assign750_e900_d_n0, assign750_e900_d_n1, assign750_e900_d_n2, assign750_e900_d_n3, assign750_e900_d_n4, assign750_e900_d_n5, assign750_e900_d_n6, assign750_e900_d_b0, assign750_e900_d_b1, assign750_e900_d_b2, assign750_e900_d_b3, assign750_e900_d_b4, assign750_e900_d_b5, assign750_e900_d_b6,) = {
    if (var_guard9 != 0.0) {
        let assign750_e887: f64 = (1.0 - p.p24);
        let assign750_e890: f64 = (0.5 * p.p18);
        let assign750_e892: f64 = (assign750_e890 * var_dvh);
        let assign750_e894: f64 = (assign750_e892 / var_vje_t);
        let assign750_e895: f64 = (assign750_e887 + assign750_e894);
        let assign750_e896: f64 = (var_dvh * assign750_e895);
        let assign750_e898: f64 = (assign750_e896 * var_pwq);
        (assign750_e898, ((((var_dvh_dn0 * assign750_e895) + (var_dvh * ((((assign750_e890 * var_dvh_dn0) * var_vje_t) - (assign750_e892 * var_vje_t_dn0)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign750_e896 * var_pwq_dn0)), ((((var_dvh_dn1 * assign750_e895) + (var_dvh * ((((assign750_e890 * var_dvh_dn1) * var_vje_t) - (assign750_e892 * var_vje_t_dn1)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign750_e896 * var_pwq_dn1)), ((((var_dvh_dn2 * assign750_e895) + (var_dvh * ((((assign750_e890 * var_dvh_dn2) * var_vje_t) - (assign750_e892 * var_vje_t_dn2)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign750_e896 * var_pwq_dn2)), ((((var_dvh_dn3 * assign750_e895) + (var_dvh * ((((assign750_e890 * var_dvh_dn3) * var_vje_t) - (assign750_e892 * var_vje_t_dn3)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign750_e896 * var_pwq_dn3)), ((((var_dvh_dn4 * assign750_e895) + (var_dvh * ((((assign750_e890 * var_dvh_dn4) * var_vje_t) - (assign750_e892 * var_vje_t_dn4)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign750_e896 * var_pwq_dn4)), ((((var_dvh_dn5 * assign750_e895) + (var_dvh * ((((assign750_e890 * var_dvh_dn5) * var_vje_t) - (assign750_e892 * var_vje_t_dn5)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign750_e896 * var_pwq_dn5)), ((((var_dvh_dn6 * assign750_e895) + (var_dvh * ((((assign750_e890 * var_dvh_dn6) * var_vje_t) - (assign750_e892 * var_vje_t_dn6)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign750_e896 * var_pwq_dn6)), ((((var_dvh_db0 * assign750_e895) + (var_dvh * ((((assign750_e890 * var_dvh_db0) * var_vje_t) - (assign750_e892 * var_vje_t_db0)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign750_e896 * var_pwq_db0)), ((((var_dvh_db1 * assign750_e895) + (var_dvh * ((((assign750_e890 * var_dvh_db1) * var_vje_t) - (assign750_e892 * var_vje_t_db1)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign750_e896 * var_pwq_db1)), ((((var_dvh_db2 * assign750_e895) + (var_dvh * ((((assign750_e890 * var_dvh_db2) * var_vje_t) - (assign750_e892 * var_vje_t_db2)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign750_e896 * var_pwq_db2)), ((((var_dvh_db3 * assign750_e895) + (var_dvh * ((((assign750_e890 * var_dvh_db3) * var_vje_t) - (assign750_e892 * var_vje_t_db3)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign750_e896 * var_pwq_db3)), ((((var_dvh_db4 * assign750_e895) + (var_dvh * ((((assign750_e890 * var_dvh_db4) * var_vje_t) - (assign750_e892 * var_vje_t_db4)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign750_e896 * var_pwq_db4)), ((((var_dvh_db5 * assign750_e895) + (var_dvh * ((((assign750_e890 * var_dvh_db5) * var_vje_t) - (assign750_e892 * var_vje_t_db5)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign750_e896 * var_pwq_db5)), ((((var_dvh_db6 * assign750_e895) + (var_dvh * ((((assign750_e890 * var_dvh_db6) * var_vje_t) - (assign750_e892 * var_vje_t_db6)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign750_e896 * var_pwq_db6)),)
    } else {
        (var_qhi, var_qhi_dn0, var_qhi_dn1, var_qhi_dn2, var_qhi_dn3, var_qhi_dn4, var_qhi_dn5, var_qhi_dn6, var_qhi_db0, var_qhi_db1, var_qhi_db2, var_qhi_db3, var_qhi_db4, var_qhi_db5, var_qhi_db6,)
    }
};
        var_qhi = assign750_e900;
        var_qhi_dn0 = assign750_e900_d_n0;
        var_qhi_dn1 = assign750_e900_d_n1;
        var_qhi_dn2 = assign750_e900_d_n2;
        var_qhi_dn3 = assign750_e900_d_n3;
        var_qhi_dn4 = assign750_e900_d_n4;
        var_qhi_dn5 = assign750_e900_d_n5;
        var_qhi_dn6 = assign750_e900_d_n6;
        var_qhi_db0 = assign750_e900_d_b0;
        var_qhi_db1 = assign750_e900_d_b1;
        var_qhi_db2 = assign750_e900_d_b2;
        var_qhi_db3 = assign750_e900_d_b3;
        var_qhi_db4 = assign750_e900_d_b4;
        var_qhi_db5 = assign750_e900_d_b5;
        var_qhi_db6 = assign750_e900_d_b6;
        var_qhi_rv = 0.0;
        var_qhi_rdn0 = 0.0;
        var_qhi_rdn1 = 0.0;
        var_qhi_rdn2 = 0.0;
        var_qhi_rdn3 = 0.0;
        var_qhi_rdn4 = 0.0;
        var_qhi_rdn5 = 0.0;
        var_qhi_rdn6 = 0.0;
        var_qhi_rdb0 = 0.0;
        var_qhi_rdb1 = 0.0;
        var_qhi_rdb2 = 0.0;
        var_qhi_rdb3 = 0.0;
        var_qhi_rdb4 = 0.0;
        var_qhi_rdb5 = 0.0;
        var_qhi_rdb6 = 0.0;

        let (assign760_e923, assign760_e923_d_n0, assign760_e923_d_n1, assign760_e923_d_n2, assign760_e923_d_n3, assign760_e923_d_n4, assign760_e923_d_n5, assign760_e923_d_n6, assign760_e923_d_b0, assign760_e923_d_b1, assign760_e923_d_b2, assign760_e923_d_b3, assign760_e923_d_b4, assign760_e923_d_b5, assign760_e923_d_b6,) = {
    if (var_guard9 == 0.0) {
        let assign760_e907: f64 = (1.0 - p.p18);
        let assign760_e911: f64 = (var_vbiei / var_vje_t);
        let assign760_e912: f64 = (1.0 - assign760_e911);
        let assign760_e913: f64 = (assign760_e912).ln();
        let assign760_e914: f64 = (assign760_e907 * assign760_e913);
        let assign760_e915: f64 = (assign760_e914).exp();
        let assign760_e916: f64 = (1.0 - assign760_e915);
        let assign760_e917: f64 = (var_vje_t * assign760_e916);
        let assign760_e920: f64 = (1.0 - p.p18);
        let assign760_e921: f64 = (assign760_e917 / assign760_e920);
        (assign760_e921, (((var_vje_t_dn0 * assign760_e916) + (var_vje_t * (-(assign760_e915 * (assign760_e907 * ((-(((var_vbiei_dn0 * var_vje_t) - (var_vbiei * var_vje_t_dn0)) / (var_vje_t * var_vje_t))) / assign760_e912)))))) / assign760_e920), (((var_vje_t_dn1 * assign760_e916) + (var_vje_t * (-(assign760_e915 * (assign760_e907 * ((-(((var_vbiei_dn1 * var_vje_t) - (var_vbiei * var_vje_t_dn1)) / (var_vje_t * var_vje_t))) / assign760_e912)))))) / assign760_e920), (((var_vje_t_dn2 * assign760_e916) + (var_vje_t * (-(assign760_e915 * (assign760_e907 * ((-(((var_vbiei_dn2 * var_vje_t) - (var_vbiei * var_vje_t_dn2)) / (var_vje_t * var_vje_t))) / assign760_e912)))))) / assign760_e920), (((var_vje_t_dn3 * assign760_e916) + (var_vje_t * (-(assign760_e915 * (assign760_e907 * ((-(((var_vbiei_dn3 * var_vje_t) - (var_vbiei * var_vje_t_dn3)) / (var_vje_t * var_vje_t))) / assign760_e912)))))) / assign760_e920), (((var_vje_t_dn4 * assign760_e916) + (var_vje_t * (-(assign760_e915 * (assign760_e907 * ((-(((var_vbiei_dn4 * var_vje_t) - (var_vbiei * var_vje_t_dn4)) / (var_vje_t * var_vje_t))) / assign760_e912)))))) / assign760_e920), (((var_vje_t_dn5 * assign760_e916) + (var_vje_t * (-(assign760_e915 * (assign760_e907 * ((-(((var_vbiei_dn5 * var_vje_t) - (var_vbiei * var_vje_t_dn5)) / (var_vje_t * var_vje_t))) / assign760_e912)))))) / assign760_e920), (((var_vje_t_dn6 * assign760_e916) + (var_vje_t * (-(assign760_e915 * (assign760_e907 * ((-(((var_vbiei_dn6 * var_vje_t) - (var_vbiei * var_vje_t_dn6)) / (var_vje_t * var_vje_t))) / assign760_e912)))))) / assign760_e920), (((var_vje_t_db0 * assign760_e916) + (var_vje_t * (-(assign760_e915 * (assign760_e907 * ((-(((var_vbiei_db0 * var_vje_t) - (var_vbiei * var_vje_t_db0)) / (var_vje_t * var_vje_t))) / assign760_e912)))))) / assign760_e920), (((var_vje_t_db1 * assign760_e916) + (var_vje_t * (-(assign760_e915 * (assign760_e907 * ((-(((var_vbiei_db1 * var_vje_t) - (var_vbiei * var_vje_t_db1)) / (var_vje_t * var_vje_t))) / assign760_e912)))))) / assign760_e920), (((var_vje_t_db2 * assign760_e916) + (var_vje_t * (-(assign760_e915 * (assign760_e907 * ((-(((var_vbiei_db2 * var_vje_t) - (var_vbiei * var_vje_t_db2)) / (var_vje_t * var_vje_t))) / assign760_e912)))))) / assign760_e920), (((var_vje_t_db3 * assign760_e916) + (var_vje_t * (-(assign760_e915 * (assign760_e907 * ((-(((var_vbiei_db3 * var_vje_t) - (var_vbiei * var_vje_t_db3)) / (var_vje_t * var_vje_t))) / assign760_e912)))))) / assign760_e920), (((var_vje_t_db4 * assign760_e916) + (var_vje_t * (-(assign760_e915 * (assign760_e907 * ((-(((var_vbiei_db4 * var_vje_t) - (var_vbiei * var_vje_t_db4)) / (var_vje_t * var_vje_t))) / assign760_e912)))))) / assign760_e920), (((var_vje_t_db5 * assign760_e916) + (var_vje_t * (-(assign760_e915 * (assign760_e907 * ((-(((var_vbiei_db5 * var_vje_t) - (var_vbiei * var_vje_t_db5)) / (var_vje_t * var_vje_t))) / assign760_e912)))))) / assign760_e920), (((var_vje_t_db6 * assign760_e916) + (var_vje_t * (-(assign760_e915 * (assign760_e907 * ((-(((var_vbiei_db6 * var_vje_t) - (var_vbiei * var_vje_t_db6)) / (var_vje_t * var_vje_t))) / assign760_e912)))))) / assign760_e920),)
    } else {
        (var_qlo, var_qlo_dn0, var_qlo_dn1, var_qlo_dn2, var_qlo_dn3, var_qlo_dn4, var_qlo_dn5, var_qlo_dn6, var_qlo_db0, var_qlo_db1, var_qlo_db2, var_qlo_db3, var_qlo_db4, var_qlo_db5, var_qlo_db6,)
    }
};
        var_qlo = assign760_e923;
        var_qlo_dn0 = assign760_e923_d_n0;
        var_qlo_dn1 = assign760_e923_d_n1;
        var_qlo_dn2 = assign760_e923_d_n2;
        var_qlo_dn3 = assign760_e923_d_n3;
        var_qlo_dn4 = assign760_e923_d_n4;
        var_qlo_dn5 = assign760_e923_d_n5;
        var_qlo_dn6 = assign760_e923_d_n6;
        var_qlo_db0 = assign760_e923_d_b0;
        var_qlo_db1 = assign760_e923_d_b1;
        var_qlo_db2 = assign760_e923_d_b2;
        var_qlo_db3 = assign760_e923_d_b3;
        var_qlo_db4 = assign760_e923_d_b4;
        var_qlo_db5 = assign760_e923_d_b5;
        var_qlo_db6 = assign760_e923_d_b6;
        var_qlo_rv = 0.0;
        var_qlo_rdn0 = 0.0;
        var_qlo_rdn1 = 0.0;
        var_qlo_rdn2 = 0.0;
        var_qlo_rdn3 = 0.0;
        var_qlo_rdn4 = 0.0;
        var_qlo_rdn5 = 0.0;
        var_qlo_rdn6 = 0.0;
        var_qlo_rdb0 = 0.0;
        var_qlo_rdb1 = 0.0;
        var_qlo_rdb2 = 0.0;
        var_qlo_rdb3 = 0.0;
        var_qlo_rdb4 = 0.0;
        var_qlo_rdb5 = 0.0;
        var_qlo_rdb6 = 0.0;

        let (assign770_e928, assign770_e928_d_n0, assign770_e928_d_n1, assign770_e928_d_n2, assign770_e928_d_n3, assign770_e928_d_n4, assign770_e928_d_n5, assign770_e928_d_n6, assign770_e928_d_b0, assign770_e928_d_b1, assign770_e928_d_b2, assign770_e928_d_b3, assign770_e928_d_b4, assign770_e928_d_b5, assign770_e928_d_b6,) = {
    if (var_guard9 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qhi, var_qhi_dn0, var_qhi_dn1, var_qhi_dn2, var_qhi_dn3, var_qhi_dn4, var_qhi_dn5, var_qhi_dn6, var_qhi_db0, var_qhi_db1, var_qhi_db2, var_qhi_db3, var_qhi_db4, var_qhi_db5, var_qhi_db6,)
    }
};
        var_qhi = assign770_e928;
        var_qhi_dn0 = assign770_e928_d_n0;
        var_qhi_dn1 = assign770_e928_d_n1;
        var_qhi_dn2 = assign770_e928_d_n2;
        var_qhi_dn3 = assign770_e928_d_n3;
        var_qhi_dn4 = assign770_e928_d_n4;
        var_qhi_dn5 = assign770_e928_d_n5;
        var_qhi_dn6 = assign770_e928_d_n6;
        var_qhi_db0 = assign770_e928_d_b0;
        var_qhi_db1 = assign770_e928_d_b1;
        var_qhi_db2 = assign770_e928_d_b2;
        var_qhi_db3 = assign770_e928_d_b3;
        var_qhi_db4 = assign770_e928_d_b4;
        var_qhi_db5 = assign770_e928_d_b5;
        var_qhi_db6 = assign770_e928_d_b6;
        var_qhi_rv = 0.0;
        var_qhi_rdn0 = 0.0;
        var_qhi_rdn1 = 0.0;
        var_qhi_rdn2 = 0.0;
        var_qhi_rdn3 = 0.0;
        var_qhi_rdn4 = 0.0;
        var_qhi_rdn5 = 0.0;
        var_qhi_rdn6 = 0.0;
        var_qhi_rdb0 = 0.0;
        var_qhi_rdb1 = 0.0;
        var_qhi_rdb2 = 0.0;
        var_qhi_rdb3 = 0.0;
        var_qhi_rdb4 = 0.0;
        var_qhi_rdb5 = 0.0;
        var_qhi_rdb6 = 0.0;

        let assign780_e932: f64 = (var_qlo + var_qhi);
        let assign780_e933: f64 = (var_cje_t * assign780_e932);
        var_qje = assign780_e933;
        var_qje_dn0 = ((var_cje_t_dn0 * assign780_e932) + (var_cje_t * (var_qlo_dn0 + var_qhi_dn0)));
        var_qje_dn1 = ((var_cje_t_dn1 * assign780_e932) + (var_cje_t * (var_qlo_dn1 + var_qhi_dn1)));
        var_qje_dn2 = ((var_cje_t_dn2 * assign780_e932) + (var_cje_t * (var_qlo_dn2 + var_qhi_dn2)));
        var_qje_dn3 = ((var_cje_t_dn3 * assign780_e932) + (var_cje_t * (var_qlo_dn3 + var_qhi_dn3)));
        var_qje_dn4 = ((var_cje_t_dn4 * assign780_e932) + (var_cje_t * (var_qlo_dn4 + var_qhi_dn4)));
        var_qje_dn5 = ((var_cje_t_dn5 * assign780_e932) + (var_cje_t * (var_qlo_dn5 + var_qhi_dn5)));
        var_qje_dn6 = ((var_cje_t_dn6 * assign780_e932) + (var_cje_t * (var_qlo_dn6 + var_qhi_dn6)));
        var_qje_db0 = ((var_cje_t_db0 * assign780_e932) + (var_cje_t * (var_qlo_db0 + var_qhi_db0)));
        var_qje_db1 = ((var_cje_t_db1 * assign780_e932) + (var_cje_t * (var_qlo_db1 + var_qhi_db1)));
        var_qje_db2 = ((var_cje_t_db2 * assign780_e932) + (var_cje_t * (var_qlo_db2 + var_qhi_db2)));
        var_qje_db3 = ((var_cje_t_db3 * assign780_e932) + (var_cje_t * (var_qlo_db3 + var_qhi_db3)));
        var_qje_db4 = ((var_cje_t_db4 * assign780_e932) + (var_cje_t * (var_qlo_db4 + var_qhi_db4)));
        var_qje_db5 = ((var_cje_t_db5 * assign780_e932) + (var_cje_t * (var_qlo_db5 + var_qhi_db5)));
        var_qje_db6 = ((var_cje_t_db6 * assign780_e932) + (var_cje_t * (var_qlo_db6 + var_qhi_db6)));
        var_qje_rv = 0.0;
        var_qje_rdn0 = 0.0;
        var_qje_rdn1 = 0.0;
        var_qje_rdn2 = 0.0;
        var_qje_rdn3 = 0.0;
        var_qje_rdn4 = 0.0;
        var_qje_rdn5 = 0.0;
        var_qje_rdn6 = 0.0;
        var_qje_rdb0 = 0.0;
        var_qje_rdb1 = 0.0;
        var_qje_rdb2 = 0.0;
        var_qje_rdb3 = 0.0;
        var_qje_rdb4 = 0.0;
        var_qje_rdb5 = 0.0;
        var_qje_rdb6 = 0.0;

        let assign790_e940: f64 = if ((p.p30 == 1.0) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };
        var_guard10 = assign790_e940;
        var_guard10_dn0 = 0.0;
        var_guard10_dn1 = 0.0;
        var_guard10_dn2 = 0.0;
        var_guard10_dn3 = 0.0;
        var_guard10_dn4 = 0.0;
        var_guard10_dn5 = 0.0;
        var_guard10_dn6 = 0.0;
        var_guard10_db0 = 0.0;
        var_guard10_db1 = 0.0;
        var_guard10_db2 = 0.0;
        var_guard10_db3 = 0.0;
        var_guard10_db4 = 0.0;
        var_guard10_db5 = 0.0;
        var_guard10_db6 = 0.0;
        var_guard10_rv = 0.0;
        var_guard10_rdn0 = 0.0;
        var_guard10_rdn1 = 0.0;
        var_guard10_rdn2 = 0.0;
        var_guard10_rdn3 = 0.0;
        var_guard10_rdn4 = 0.0;
        var_guard10_rdn5 = 0.0;
        var_guard10_rdn6 = 0.0;
        var_guard10_rdb0 = 0.0;
        var_guard10_rdb1 = 0.0;
        var_guard10_rdb2 = 0.0;
        var_guard10_rdb3 = 0.0;
        var_guard10_rdb4 = 0.0;
        var_guard10_rdb5 = 0.0;
        var_guard10_rdb6 = 0.0;

        let assign800_e951: f64 = if (((p.p30 == 2.0) && (p.p33 > 0.0)) && (p.p35 > 0.0)) { 1.0 } else { 0.0 };
        var_guard11 = assign800_e951;
        var_guard11_dn0 = 0.0;
        var_guard11_dn1 = 0.0;
        var_guard11_dn2 = 0.0;
        var_guard11_dn3 = 0.0;
        var_guard11_dn4 = 0.0;
        var_guard11_dn5 = 0.0;
        var_guard11_dn6 = 0.0;
        var_guard11_db0 = 0.0;
        var_guard11_db1 = 0.0;
        var_guard11_db2 = 0.0;
        var_guard11_db3 = 0.0;
        var_guard11_db4 = 0.0;
        var_guard11_db5 = 0.0;
        var_guard11_db6 = 0.0;
        var_guard11_rv = 0.0;
        var_guard11_rdn0 = 0.0;
        var_guard11_rdn1 = 0.0;
        var_guard11_rdn2 = 0.0;
        var_guard11_rdn3 = 0.0;
        var_guard11_rdn4 = 0.0;
        var_guard11_rdn5 = 0.0;
        var_guard11_rdn6 = 0.0;
        var_guard11_rdb0 = 0.0;
        var_guard11_rdb1 = 0.0;
        var_guard11_rdb2 = 0.0;
        var_guard11_rdb3 = 0.0;
        var_guard11_rdb4 = 0.0;
        var_guard11_rdb5 = 0.0;
        var_guard11_rdb6 = 0.0;

        *var_dv0_slot = var_dv0;
        *var_dv0_db0_slot = var_dv0_db0;
        *var_dv0_db1_slot = var_dv0_db1;
        *var_dv0_db2_slot = var_dv0_db2;
        *var_dv0_db3_slot = var_dv0_db3;
        *var_dv0_db4_slot = var_dv0_db4;
        *var_dv0_db5_slot = var_dv0_db5;
        *var_dv0_db6_slot = var_dv0_db6;
        *var_dv0_dn0_slot = var_dv0_dn0;
        *var_dv0_dn1_slot = var_dv0_dn1;
        *var_dv0_dn2_slot = var_dv0_dn2;
        *var_dv0_dn3_slot = var_dv0_dn3;
        *var_dv0_dn4_slot = var_dv0_dn4;
        *var_dv0_dn5_slot = var_dv0_dn5;
        *var_dv0_dn6_slot = var_dv0_dn6;
        *var_dv0_rdb0_slot = var_dv0_rdb0;
        *var_dv0_rdb1_slot = var_dv0_rdb1;
        *var_dv0_rdb2_slot = var_dv0_rdb2;
        *var_dv0_rdb3_slot = var_dv0_rdb3;
        *var_dv0_rdb4_slot = var_dv0_rdb4;
        *var_dv0_rdb5_slot = var_dv0_rdb5;
        *var_dv0_rdb6_slot = var_dv0_rdb6;
        *var_dv0_rdn0_slot = var_dv0_rdn0;
        *var_dv0_rdn1_slot = var_dv0_rdn1;
        *var_dv0_rdn2_slot = var_dv0_rdn2;
        *var_dv0_rdn3_slot = var_dv0_rdn3;
        *var_dv0_rdn4_slot = var_dv0_rdn4;
        *var_dv0_rdn5_slot = var_dv0_rdn5;
        *var_dv0_rdn6_slot = var_dv0_rdn6;
        *var_dv0_rv_slot = var_dv0_rv;
        *var_dvh_slot = var_dvh;
        *var_dvh_db0_slot = var_dvh_db0;
        *var_dvh_db1_slot = var_dvh_db1;
        *var_dvh_db2_slot = var_dvh_db2;
        *var_dvh_db3_slot = var_dvh_db3;
        *var_dvh_db4_slot = var_dvh_db4;
        *var_dvh_db5_slot = var_dvh_db5;
        *var_dvh_db6_slot = var_dvh_db6;
        *var_dvh_dn0_slot = var_dvh_dn0;
        *var_dvh_dn1_slot = var_dvh_dn1;
        *var_dvh_dn2_slot = var_dvh_dn2;
        *var_dvh_dn3_slot = var_dvh_dn3;
        *var_dvh_dn4_slot = var_dvh_dn4;
        *var_dvh_dn5_slot = var_dvh_dn5;
        *var_dvh_dn6_slot = var_dvh_dn6;
        *var_dvh_rdb0_slot = var_dvh_rdb0;
        *var_dvh_rdb1_slot = var_dvh_rdb1;
        *var_dvh_rdb2_slot = var_dvh_rdb2;
        *var_dvh_rdb3_slot = var_dvh_rdb3;
        *var_dvh_rdb4_slot = var_dvh_rdb4;
        *var_dvh_rdb5_slot = var_dvh_rdb5;
        *var_dvh_rdb6_slot = var_dvh_rdb6;
        *var_dvh_rdn0_slot = var_dvh_rdn0;
        *var_dvh_rdn1_slot = var_dvh_rdn1;
        *var_dvh_rdn2_slot = var_dvh_rdn2;
        *var_dvh_rdn3_slot = var_dvh_rdn3;
        *var_dvh_rdn4_slot = var_dvh_rdn4;
        *var_dvh_rdn5_slot = var_dvh_rdn5;
        *var_dvh_rdn6_slot = var_dvh_rdn6;
        *var_dvh_rv_slot = var_dvh_rv;
        *var_guard10_slot = var_guard10;
        *var_guard10_db0_slot = var_guard10_db0;
        *var_guard10_db1_slot = var_guard10_db1;
        *var_guard10_db2_slot = var_guard10_db2;
        *var_guard10_db3_slot = var_guard10_db3;
        *var_guard10_db4_slot = var_guard10_db4;
        *var_guard10_db5_slot = var_guard10_db5;
        *var_guard10_db6_slot = var_guard10_db6;
        *var_guard10_dn0_slot = var_guard10_dn0;
        *var_guard10_dn1_slot = var_guard10_dn1;
        *var_guard10_dn2_slot = var_guard10_dn2;
        *var_guard10_dn3_slot = var_guard10_dn3;
        *var_guard10_dn4_slot = var_guard10_dn4;
        *var_guard10_dn5_slot = var_guard10_dn5;
        *var_guard10_dn6_slot = var_guard10_dn6;
        *var_guard10_rdb0_slot = var_guard10_rdb0;
        *var_guard10_rdb1_slot = var_guard10_rdb1;
        *var_guard10_rdb2_slot = var_guard10_rdb2;
        *var_guard10_rdb3_slot = var_guard10_rdb3;
        *var_guard10_rdb4_slot = var_guard10_rdb4;
        *var_guard10_rdb5_slot = var_guard10_rdb5;
        *var_guard10_rdb6_slot = var_guard10_rdb6;
        *var_guard10_rdn0_slot = var_guard10_rdn0;
        *var_guard10_rdn1_slot = var_guard10_rdn1;
        *var_guard10_rdn2_slot = var_guard10_rdn2;
        *var_guard10_rdn3_slot = var_guard10_rdn3;
        *var_guard10_rdn4_slot = var_guard10_rdn4;
        *var_guard10_rdn5_slot = var_guard10_rdn5;
        *var_guard10_rdn6_slot = var_guard10_rdn6;
        *var_guard10_rv_slot = var_guard10_rv;
        *var_guard11_slot = var_guard11;
        *var_guard11_db0_slot = var_guard11_db0;
        *var_guard11_db1_slot = var_guard11_db1;
        *var_guard11_db2_slot = var_guard11_db2;
        *var_guard11_db3_slot = var_guard11_db3;
        *var_guard11_db4_slot = var_guard11_db4;
        *var_guard11_db5_slot = var_guard11_db5;
        *var_guard11_db6_slot = var_guard11_db6;
        *var_guard11_dn0_slot = var_guard11_dn0;
        *var_guard11_dn1_slot = var_guard11_dn1;
        *var_guard11_dn2_slot = var_guard11_dn2;
        *var_guard11_dn3_slot = var_guard11_dn3;
        *var_guard11_dn4_slot = var_guard11_dn4;
        *var_guard11_dn5_slot = var_guard11_dn5;
        *var_guard11_dn6_slot = var_guard11_dn6;
        *var_guard11_rdb0_slot = var_guard11_rdb0;
        *var_guard11_rdb1_slot = var_guard11_rdb1;
        *var_guard11_rdb2_slot = var_guard11_rdb2;
        *var_guard11_rdb3_slot = var_guard11_rdb3;
        *var_guard11_rdb4_slot = var_guard11_rdb4;
        *var_guard11_rdb5_slot = var_guard11_rdb5;
        *var_guard11_rdb6_slot = var_guard11_rdb6;
        *var_guard11_rdn0_slot = var_guard11_rdn0;
        *var_guard11_rdn1_slot = var_guard11_rdn1;
        *var_guard11_rdn2_slot = var_guard11_rdn2;
        *var_guard11_rdn3_slot = var_guard11_rdn3;
        *var_guard11_rdn4_slot = var_guard11_rdn4;
        *var_guard11_rdn5_slot = var_guard11_rdn5;
        *var_guard11_rdn6_slot = var_guard11_rdn6;
        *var_guard11_rv_slot = var_guard11_rv;
        *var_guard9_slot = var_guard9;
        *var_guard9_db0_slot = var_guard9_db0;
        *var_guard9_db1_slot = var_guard9_db1;
        *var_guard9_db2_slot = var_guard9_db2;
        *var_guard9_db3_slot = var_guard9_db3;
        *var_guard9_db4_slot = var_guard9_db4;
        *var_guard9_db5_slot = var_guard9_db5;
        *var_guard9_db6_slot = var_guard9_db6;
        *var_guard9_dn0_slot = var_guard9_dn0;
        *var_guard9_dn1_slot = var_guard9_dn1;
        *var_guard9_dn2_slot = var_guard9_dn2;
        *var_guard9_dn3_slot = var_guard9_dn3;
        *var_guard9_dn4_slot = var_guard9_dn4;
        *var_guard9_dn5_slot = var_guard9_dn5;
        *var_guard9_dn6_slot = var_guard9_dn6;
        *var_guard9_rdb0_slot = var_guard9_rdb0;
        *var_guard9_rdb1_slot = var_guard9_rdb1;
        *var_guard9_rdb2_slot = var_guard9_rdb2;
        *var_guard9_rdb3_slot = var_guard9_rdb3;
        *var_guard9_rdb4_slot = var_guard9_rdb4;
        *var_guard9_rdb5_slot = var_guard9_rdb5;
        *var_guard9_rdb6_slot = var_guard9_rdb6;
        *var_guard9_rdn0_slot = var_guard9_rdn0;
        *var_guard9_rdn1_slot = var_guard9_rdn1;
        *var_guard9_rdn2_slot = var_guard9_rdn2;
        *var_guard9_rdn3_slot = var_guard9_rdn3;
        *var_guard9_rdn4_slot = var_guard9_rdn4;
        *var_guard9_rdn5_slot = var_guard9_rdn5;
        *var_guard9_rdn6_slot = var_guard9_rdn6;
        *var_guard9_rv_slot = var_guard9_rv;
        *var_pwq_slot = var_pwq;
        *var_pwq_db0_slot = var_pwq_db0;
        *var_pwq_db1_slot = var_pwq_db1;
        *var_pwq_db2_slot = var_pwq_db2;
        *var_pwq_db3_slot = var_pwq_db3;
        *var_pwq_db4_slot = var_pwq_db4;
        *var_pwq_db5_slot = var_pwq_db5;
        *var_pwq_db6_slot = var_pwq_db6;
        *var_pwq_dn0_slot = var_pwq_dn0;
        *var_pwq_dn1_slot = var_pwq_dn1;
        *var_pwq_dn2_slot = var_pwq_dn2;
        *var_pwq_dn3_slot = var_pwq_dn3;
        *var_pwq_dn4_slot = var_pwq_dn4;
        *var_pwq_dn5_slot = var_pwq_dn5;
        *var_pwq_dn6_slot = var_pwq_dn6;
        *var_pwq_rdb0_slot = var_pwq_rdb0;
        *var_pwq_rdb1_slot = var_pwq_rdb1;
        *var_pwq_rdb2_slot = var_pwq_rdb2;
        *var_pwq_rdb3_slot = var_pwq_rdb3;
        *var_pwq_rdb4_slot = var_pwq_rdb4;
        *var_pwq_rdb5_slot = var_pwq_rdb5;
        *var_pwq_rdb6_slot = var_pwq_rdb6;
        *var_pwq_rdn0_slot = var_pwq_rdn0;
        *var_pwq_rdn1_slot = var_pwq_rdn1;
        *var_pwq_rdn2_slot = var_pwq_rdn2;
        *var_pwq_rdn3_slot = var_pwq_rdn3;
        *var_pwq_rdn4_slot = var_pwq_rdn4;
        *var_pwq_rdn5_slot = var_pwq_rdn5;
        *var_pwq_rdn6_slot = var_pwq_rdn6;
        *var_pwq_rv_slot = var_pwq_rv;
        *var_qhi_slot = var_qhi;
        *var_qhi_db0_slot = var_qhi_db0;
        *var_qhi_db1_slot = var_qhi_db1;
        *var_qhi_db2_slot = var_qhi_db2;
        *var_qhi_db3_slot = var_qhi_db3;
        *var_qhi_db4_slot = var_qhi_db4;
        *var_qhi_db5_slot = var_qhi_db5;
        *var_qhi_db6_slot = var_qhi_db6;
        *var_qhi_dn0_slot = var_qhi_dn0;
        *var_qhi_dn1_slot = var_qhi_dn1;
        *var_qhi_dn2_slot = var_qhi_dn2;
        *var_qhi_dn3_slot = var_qhi_dn3;
        *var_qhi_dn4_slot = var_qhi_dn4;
        *var_qhi_dn5_slot = var_qhi_dn5;
        *var_qhi_dn6_slot = var_qhi_dn6;
        *var_qhi_rdb0_slot = var_qhi_rdb0;
        *var_qhi_rdb1_slot = var_qhi_rdb1;
        *var_qhi_rdb2_slot = var_qhi_rdb2;
        *var_qhi_rdb3_slot = var_qhi_rdb3;
        *var_qhi_rdb4_slot = var_qhi_rdb4;
        *var_qhi_rdb5_slot = var_qhi_rdb5;
        *var_qhi_rdb6_slot = var_qhi_rdb6;
        *var_qhi_rdn0_slot = var_qhi_rdn0;
        *var_qhi_rdn1_slot = var_qhi_rdn1;
        *var_qhi_rdn2_slot = var_qhi_rdn2;
        *var_qhi_rdn3_slot = var_qhi_rdn3;
        *var_qhi_rdn4_slot = var_qhi_rdn4;
        *var_qhi_rdn5_slot = var_qhi_rdn5;
        *var_qhi_rdn6_slot = var_qhi_rdn6;
        *var_qhi_rv_slot = var_qhi_rv;
        *var_qje_slot = var_qje;
        *var_qje_db0_slot = var_qje_db0;
        *var_qje_db1_slot = var_qje_db1;
        *var_qje_db2_slot = var_qje_db2;
        *var_qje_db3_slot = var_qje_db3;
        *var_qje_db4_slot = var_qje_db4;
        *var_qje_db5_slot = var_qje_db5;
        *var_qje_db6_slot = var_qje_db6;
        *var_qje_dn0_slot = var_qje_dn0;
        *var_qje_dn1_slot = var_qje_dn1;
        *var_qje_dn2_slot = var_qje_dn2;
        *var_qje_dn3_slot = var_qje_dn3;
        *var_qje_dn4_slot = var_qje_dn4;
        *var_qje_dn5_slot = var_qje_dn5;
        *var_qje_dn6_slot = var_qje_dn6;
        *var_qje_rdb0_slot = var_qje_rdb0;
        *var_qje_rdb1_slot = var_qje_rdb1;
        *var_qje_rdb2_slot = var_qje_rdb2;
        *var_qje_rdb3_slot = var_qje_rdb3;
        *var_qje_rdb4_slot = var_qje_rdb4;
        *var_qje_rdb5_slot = var_qje_rdb5;
        *var_qje_rdb6_slot = var_qje_rdb6;
        *var_qje_rdn0_slot = var_qje_rdn0;
        *var_qje_rdn1_slot = var_qje_rdn1;
        *var_qje_rdn2_slot = var_qje_rdn2;
        *var_qje_rdn3_slot = var_qje_rdn3;
        *var_qje_rdn4_slot = var_qje_rdn4;
        *var_qje_rdn5_slot = var_qje_rdn5;
        *var_qje_rdn6_slot = var_qje_rdn6;
        *var_qje_rv_slot = var_qje_rv;
        *var_qlo_slot = var_qlo;
        *var_qlo_db0_slot = var_qlo_db0;
        *var_qlo_db1_slot = var_qlo_db1;
        *var_qlo_db2_slot = var_qlo_db2;
        *var_qlo_db3_slot = var_qlo_db3;
        *var_qlo_db4_slot = var_qlo_db4;
        *var_qlo_db5_slot = var_qlo_db5;
        *var_qlo_db6_slot = var_qlo_db6;
        *var_qlo_dn0_slot = var_qlo_dn0;
        *var_qlo_dn1_slot = var_qlo_dn1;
        *var_qlo_dn2_slot = var_qlo_dn2;
        *var_qlo_dn3_slot = var_qlo_dn3;
        *var_qlo_dn4_slot = var_qlo_dn4;
        *var_qlo_dn5_slot = var_qlo_dn5;
        *var_qlo_dn6_slot = var_qlo_dn6;
        *var_qlo_rdb0_slot = var_qlo_rdb0;
        *var_qlo_rdb1_slot = var_qlo_rdb1;
        *var_qlo_rdb2_slot = var_qlo_rdb2;
        *var_qlo_rdb3_slot = var_qlo_rdb3;
        *var_qlo_rdb4_slot = var_qlo_rdb4;
        *var_qlo_rdb5_slot = var_qlo_rdb5;
        *var_qlo_rdb6_slot = var_qlo_rdb6;
        *var_qlo_rdn0_slot = var_qlo_rdn0;
        *var_qlo_rdn1_slot = var_qlo_rdn1;
        *var_qlo_rdn2_slot = var_qlo_rdn2;
        *var_qlo_rdn3_slot = var_qlo_rdn3;
        *var_qlo_rdn4_slot = var_qlo_rdn4;
        *var_qlo_rdn5_slot = var_qlo_rdn5;
        *var_qlo_rdn6_slot = var_qlo_rdn6;
        *var_qlo_rv_slot = var_qlo_rv;
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
        var_guard10: f64,
        var_guard11: f64,
        var_guard12: f64,
        var_guard13: f64,
        var_guard14: f64,
        var_guard8: f64,
        var_ibe: f64,
        var_ibe_dn2: f64,
        var_ibe_dn3: f64,
        var_ibe_dn4: f64,
        var_ifwd: f64,
        var_ifwd_dn2: f64,
        var_ifwd_dn3: f64,
        var_ifwd_dn4: f64,
        var_qde: f64,
        var_qde_dn0: f64,
        var_qde_dn1: f64,
        var_qde_dn2: f64,
        var_qde_dn3: f64,
        var_qde_dn4: f64,
        var_qje: f64,
        var_qje_dn2: f64,
        var_qje_dn3: f64,
        var_qje_dn4: f64,
        var_rb: f64,
        var_rb_dn0: f64,
        var_rb_dn2: f64,
        var_rb_dn3: f64,
        var_rb_dn6: f64,
        var_re: f64,
        var_re_dn1: f64,
        var_re_dn2: f64,
        var_re_dn4: f64,
        var_tff: f64,
        var_tff_dn0: f64,
        var_tff_dn1: f64,
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
        let (eq0_e60, eq0_e60_d_n0, eq0_e60_d_n1, eq0_e60_d_n2, eq0_e60_d_n3, eq0_e60_d_n4,) = {
    if (var_guard8 != 0.0) {
        let eq0_e56: f64 = (-var_ifwd);
        let eq0_e58: f64 = (eq0_e56 * var_tff);
        let eq0_e58_d_n0: f64 = (eq0_e56 * var_tff_dn0);
        let eq0_e58_d_n1: f64 = (eq0_e56 * var_tff_dn1);
        let eq0_e58_d_n2: f64 = ((-var_ifwd_dn2) * var_tff);
        let eq0_e58_d_n3: f64 = ((-var_ifwd_dn3) * var_tff);
        let eq0_e58_d_n4: f64 = ((-var_ifwd_dn4) * var_tff);
        (eq0_e58, eq0_e58_d_n0, eq0_e58_d_n1, eq0_e58_d_n2, eq0_e58_d_n3, eq0_e58_d_n4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e60;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            None,
            multiplicity * (eq0_value),
            [0, 1, 2, 3, 4],
            [multiplicity * (eq0_e60_d_n0), multiplicity * (eq0_e60_d_n1), multiplicity * (eq0_e60_d_n2), multiplicity * (eq0_e60_d_n3), multiplicity * (eq0_e60_d_n4)],
            [],
            [],
            1.0,
        );
        let (eq2_e73, eq2_e73_d_n0, eq2_e73_d_n1, eq2_e73_d_n6,) = {
    if (var_guard8 != 0.0) {
        let eq2_e70: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, (nv6 - 0.0));
        let eq2_e71: f64 = (var_tff * eq2_e70);
        let eq2_e71_d_n0: f64 = (var_tff_dn0 * eq2_e70);
        let eq2_e71_d_n1: f64 = (var_tff_dn1 * eq2_e70);
        (eq2_e71, eq2_e71_d_n0, eq2_e71_d_n1, (var_tff * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e73;
        stamper.stamp_current_node3_local(
            Some(6),
            None,
            multiplicity * (eq2_value),
            0,
            multiplicity * (eq2_e73_d_n0),
            1,
            multiplicity * (eq2_e73_d_n1),
            6,
            multiplicity * (eq2_e73_d_n6),
        );
        let (eq4_e88, eq4_e88_d_n0, eq4_e88_d_n1, eq4_e88_d_n2, eq4_e88_d_n3, eq4_e88_d_n4,) = {
    if (var_guard10 != 0.0) {
        let eq4_e81: f64 = (-1.0);
        let eq4_e84: f64 = (var_ibe * (nv0 - nv1));
        let eq4_e84_d_n2: f64 = (var_ibe_dn2 * (nv0 - nv1));
        let eq4_e84_d_n3: f64 = (var_ibe_dn3 * (nv0 - nv1));
        let eq4_e84_d_n4: f64 = (var_ibe_dn4 * (nv0 - nv1));
        let eq4_e85: f64 = (eq4_e84).abs();
        let eq4_e85_d_n0: f64 = if eq4_e84 >= 0.0 { var_ibe } else { (-var_ibe) };
        let eq4_e85_d_n1: f64 = if eq4_e84 >= 0.0 { (-var_ibe) } else { (-(-var_ibe)) };
        let eq4_e85_d_n2: f64 = if eq4_e84 >= 0.0 { eq4_e84_d_n2 } else { (-eq4_e84_d_n2) };
        let eq4_e85_d_n3: f64 = if eq4_e84 >= 0.0 { eq4_e84_d_n3 } else { (-eq4_e84_d_n3) };
        let eq4_e85_d_n4: f64 = if eq4_e84 >= 0.0 { eq4_e84_d_n4 } else { (-eq4_e84_d_n4) };
        let eq4_e86: f64 = (eq4_e81 * eq4_e85);
        let eq4_e86_d_n0: f64 = (eq4_e81 * eq4_e85_d_n0);
        let eq4_e86_d_n1: f64 = (eq4_e81 * eq4_e85_d_n1);
        let eq4_e86_d_n2: f64 = (eq4_e81 * eq4_e85_d_n2);
        let eq4_e86_d_n3: f64 = (eq4_e81 * eq4_e85_d_n3);
        let eq4_e86_d_n4: f64 = (eq4_e81 * eq4_e85_d_n4);
        (eq4_e86, eq4_e86_d_n0, eq4_e86_d_n1, eq4_e86_d_n2, eq4_e86_d_n3, eq4_e86_d_n4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e88;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            None,
            multiplicity * (eq4_value),
            [0, 1, 2, 3, 4],
            [multiplicity * (eq4_e88_d_n0), multiplicity * (eq4_e88_d_n1), multiplicity * (eq4_e88_d_n2), multiplicity * (eq4_e88_d_n3), multiplicity * (eq4_e88_d_n4)],
            [],
            [],
            1.0,
        );
        let (eq5_e94, eq5_e94_d_n2,) = {
    if (var_guard10 != 0.0) {
        let __rspice_inv_cse_0: f64 = 1.0 / p.p33;
        let eq5_e92: f64 = ((nv2 - 0.0) * __rspice_inv_cse_0);
        let eq5_e92_d_n2: f64 = (1.0 * __rspice_inv_cse_0);
        (eq5_e92, eq5_e92_d_n2,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e94;
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * (eq5_value),
            2,
            multiplicity * (eq5_e94_d_n2),
        );
        let (eq6_e101, eq6_e101_d_n2,) = {
    if (var_guard10 != 0.0) {
        let eq6_e98: f64 = ((nv2 - 0.0) * p.p34);
        let eq6_e99: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq6_e98);
        (eq6_e99, (p.p34 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e101;
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * (eq6_value),
            2,
            multiplicity * (eq6_e101_d_n2),
        );
        let (eq8_e118, eq8_e118_d_n0, eq8_e118_d_n1, eq8_e118_d_n2, eq8_e118_d_n3, eq8_e118_d_n4,) = {
    if ((var_guard10 == 0.0) && (var_guard11 != 0.0)) {
        let eq8_e111: f64 = (-1.0);
        let eq8_e114: f64 = (var_ibe * (nv0 - nv1));
        let eq8_e114_d_n2: f64 = (var_ibe_dn2 * (nv0 - nv1));
        let eq8_e114_d_n3: f64 = (var_ibe_dn3 * (nv0 - nv1));
        let eq8_e114_d_n4: f64 = (var_ibe_dn4 * (nv0 - nv1));
        let eq8_e115: f64 = (eq8_e114).abs();
        let eq8_e115_d_n0: f64 = if eq8_e114 >= 0.0 { var_ibe } else { (-var_ibe) };
        let eq8_e115_d_n1: f64 = if eq8_e114 >= 0.0 { (-var_ibe) } else { (-(-var_ibe)) };
        let eq8_e115_d_n2: f64 = if eq8_e114 >= 0.0 { eq8_e114_d_n2 } else { (-eq8_e114_d_n2) };
        let eq8_e115_d_n3: f64 = if eq8_e114 >= 0.0 { eq8_e114_d_n3 } else { (-eq8_e114_d_n3) };
        let eq8_e115_d_n4: f64 = if eq8_e114 >= 0.0 { eq8_e114_d_n4 } else { (-eq8_e114_d_n4) };
        let eq8_e116: f64 = (eq8_e111 * eq8_e115);
        let eq8_e116_d_n0: f64 = (eq8_e111 * eq8_e115_d_n0);
        let eq8_e116_d_n1: f64 = (eq8_e111 * eq8_e115_d_n1);
        let eq8_e116_d_n2: f64 = (eq8_e111 * eq8_e115_d_n2);
        let eq8_e116_d_n3: f64 = (eq8_e111 * eq8_e115_d_n3);
        let eq8_e116_d_n4: f64 = (eq8_e111 * eq8_e115_d_n4);
        (eq8_e116, eq8_e116_d_n0, eq8_e116_d_n1, eq8_e116_d_n2, eq8_e116_d_n3, eq8_e116_d_n4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e118;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            None,
            multiplicity * (eq8_value),
            [0, 1, 2, 3, 4],
            [multiplicity * (eq8_e118_d_n0), multiplicity * (eq8_e118_d_n1), multiplicity * (eq8_e118_d_n2), multiplicity * (eq8_e118_d_n3), multiplicity * (eq8_e118_d_n4)],
            [],
            [],
            1.0,
        );
        let (eq9_e127, eq9_e127_d_n2, eq9_e127_d_n5,) = {
    if ((var_guard10 == 0.0) && (var_guard11 != 0.0)) {
        let __rspice_inv_cse_1: f64 = 1.0 / p.p33;
        let eq9_e125: f64 = ((nv2 - nv5) * __rspice_inv_cse_1);
        let eq9_e125_d_n2: f64 = (1.0 * __rspice_inv_cse_1);
        let eq9_e125_d_n5: f64 = ((-1.0) * __rspice_inv_cse_1);
        (eq9_e125, eq9_e125_d_n2, eq9_e125_d_n5,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e127;
        stamper.stamp_current_node2_local(
            Some(2),
            Some(5),
            multiplicity * (eq9_value),
            2,
            multiplicity * (eq9_e127_d_n2),
            5,
            multiplicity * (eq9_e127_d_n5),
        );
        let (eq10_e137, eq10_e137_d_n2,) = {
    if ((var_guard10 == 0.0) && (var_guard11 != 0.0)) {
        let eq10_e134: f64 = (p.p34 * (nv2 - 0.0));
        let eq10_e135: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq10_e134);
        (eq10_e135, (p.p34 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e137;
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * (eq10_value),
            2,
            multiplicity * (eq10_e137_d_n2),
        );
        let (eq11_e146, eq11_e146_d_n5,) = {
    if ((var_guard10 == 0.0) && (var_guard11 != 0.0)) {
        let __rspice_inv_cse_2: f64 = 1.0 / p.p35;
        let eq11_e144: f64 = ((nv5 - 0.0) * __rspice_inv_cse_2);
        let eq11_e144_d_n5: f64 = (1.0 * __rspice_inv_cse_2);
        (eq11_e144, eq11_e144_d_n5,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e146;
        stamper.stamp_current_node1_local(
            Some(5),
            None,
            multiplicity * (eq11_value),
            5,
            multiplicity * (eq11_e146_d_n5),
        );
        let (eq12_e156, eq12_e156_d_n5,) = {
    if ((var_guard10 == 0.0) && (var_guard11 != 0.0)) {
        let eq12_e153: f64 = (p.p36 * (nv5 - 0.0));
        let eq12_e154: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq12_e153);
        (eq12_e154, (p.p36 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq12_value: f64 = eq12_e156;
        stamper.stamp_current_node1_local(
            Some(5),
            None,
            multiplicity * (eq12_value),
            5,
            multiplicity * (eq12_e156_d_n5),
        );
        let (eq13_e172, eq13_e172_d_n0, eq13_e172_d_n1, eq13_e172_d_n2, eq13_e172_d_n3, eq13_e172_d_n4,) = {
    if (((var_guard10 == 0.0) && (var_guard11 == 0.0)) && (var_guard12 != 0.0)) {
        let eq13_e165: f64 = (-1.0);
        let eq13_e168: f64 = (var_ibe * (nv0 - nv1));
        let eq13_e168_d_n2: f64 = (var_ibe_dn2 * (nv0 - nv1));
        let eq13_e168_d_n3: f64 = (var_ibe_dn3 * (nv0 - nv1));
        let eq13_e168_d_n4: f64 = (var_ibe_dn4 * (nv0 - nv1));
        let eq13_e169: f64 = (eq13_e168).abs();
        let eq13_e169_d_n0: f64 = if eq13_e168 >= 0.0 { var_ibe } else { (-var_ibe) };
        let eq13_e169_d_n1: f64 = if eq13_e168 >= 0.0 { (-var_ibe) } else { (-(-var_ibe)) };
        let eq13_e169_d_n2: f64 = if eq13_e168 >= 0.0 { eq13_e168_d_n2 } else { (-eq13_e168_d_n2) };
        let eq13_e169_d_n3: f64 = if eq13_e168 >= 0.0 { eq13_e168_d_n3 } else { (-eq13_e168_d_n3) };
        let eq13_e169_d_n4: f64 = if eq13_e168 >= 0.0 { eq13_e168_d_n4 } else { (-eq13_e168_d_n4) };
        let eq13_e170: f64 = (eq13_e165 * eq13_e169);
        let eq13_e170_d_n0: f64 = (eq13_e165 * eq13_e169_d_n0);
        let eq13_e170_d_n1: f64 = (eq13_e165 * eq13_e169_d_n1);
        let eq13_e170_d_n2: f64 = (eq13_e165 * eq13_e169_d_n2);
        let eq13_e170_d_n3: f64 = (eq13_e165 * eq13_e169_d_n3);
        let eq13_e170_d_n4: f64 = (eq13_e165 * eq13_e169_d_n4);
        (eq13_e170, eq13_e170_d_n0, eq13_e170_d_n1, eq13_e170_d_n2, eq13_e170_d_n3, eq13_e170_d_n4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq13_value: f64 = eq13_e172;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            None,
            multiplicity * (eq13_value),
            [0, 1, 2, 3, 4],
            [multiplicity * (eq13_e172_d_n0), multiplicity * (eq13_e172_d_n1), multiplicity * (eq13_e172_d_n2), multiplicity * (eq13_e172_d_n3), multiplicity * (eq13_e172_d_n4)],
            [],
            [],
            1.0,
        );
        let (eq18_e224, eq18_e224_d_n0, eq18_e224_d_n2, eq18_e224_d_n3, eq18_e224_d_n6,) = {
    if (var_guard13 != 0.0) {
        let __rspice_inv_cse_3: f64 = 1.0 / var_weff;
        let eq18_e214: f64 = (var_rb * __rspice_inv_cse_3);
        let eq18_e214_d_n0: f64 = (var_rb_dn0 * __rspice_inv_cse_3);
        let eq18_e214_d_n2: f64 = (var_rb_dn2 * __rspice_inv_cse_3);
        let eq18_e214_d_n3: f64 = (var_rb_dn3 * __rspice_inv_cse_3);
        let eq18_e214_d_n6: f64 = (var_rb_dn6 * __rspice_inv_cse_3);
        let (eq18_e221, eq18_e221_d_n0, eq18_e221_d_n2, eq18_e221_d_n3, eq18_e221_d_n6,) = {
            if (eq18_e214 > p.p46) {
                let __rspice_inv_cse_4: f64 = 1.0 / var_weff;
                let eq18_e219: f64 = (var_rb * __rspice_inv_cse_4);
                let eq18_e219_d_n0: f64 = (var_rb_dn0 * __rspice_inv_cse_4);
                let eq18_e219_d_n2: f64 = (var_rb_dn2 * __rspice_inv_cse_4);
                let eq18_e219_d_n3: f64 = (var_rb_dn3 * __rspice_inv_cse_4);
                let eq18_e219_d_n6: f64 = (var_rb_dn6 * __rspice_inv_cse_4);
                (eq18_e219, eq18_e219_d_n0, eq18_e219_d_n2, eq18_e219_d_n3, eq18_e219_d_n6,)
            } else {
                (p.p46, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let eq18_e222: f64 = ((nv0 - nv3) / eq18_e221);
        let eq18_e222_d_n0: f64 = ((eq18_e221 - ((nv0 - nv3) * eq18_e221_d_n0)) / (eq18_e221 * eq18_e221));
        let eq18_e222_d_n2: f64 = (-(((nv0 - nv3) * eq18_e221_d_n2) / (eq18_e221 * eq18_e221)));
        let eq18_e222_d_n3: f64 = (((-eq18_e221) - ((nv0 - nv3) * eq18_e221_d_n3)) / (eq18_e221 * eq18_e221));
        let eq18_e222_d_n6: f64 = (-(((nv0 - nv3) * eq18_e221_d_n6) / (eq18_e221 * eq18_e221)));
        (eq18_e222, eq18_e222_d_n0, eq18_e222_d_n2, eq18_e222_d_n3, eq18_e222_d_n6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e224;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(0),
            Some(3),
            multiplicity * (eq18_value),
            [0, 2, 3, 6],
            [multiplicity * (eq18_e224_d_n0), multiplicity * (eq18_e224_d_n2), multiplicity * (eq18_e224_d_n3), multiplicity * (eq18_e224_d_n6)],
            [],
            [],
            1.0,
        );
        let (eq21_e250, eq21_e250_d_n1, eq21_e250_d_n2, eq21_e250_d_n4,) = {
    if (var_guard14 != 0.0) {
        let __rspice_inv_cse_5: f64 = 1.0 / var_weff;
        let eq21_e240: f64 = (var_re * __rspice_inv_cse_5);
        let eq21_e240_d_n1: f64 = (var_re_dn1 * __rspice_inv_cse_5);
        let eq21_e240_d_n2: f64 = (var_re_dn2 * __rspice_inv_cse_5);
        let eq21_e240_d_n4: f64 = (var_re_dn4 * __rspice_inv_cse_5);
        let (eq21_e247, eq21_e247_d_n1, eq21_e247_d_n2, eq21_e247_d_n4,) = {
            if (eq21_e240 > p.p46) {
                let __rspice_inv_cse_6: f64 = 1.0 / var_weff;
                let eq21_e245: f64 = (var_re * __rspice_inv_cse_6);
                let eq21_e245_d_n1: f64 = (var_re_dn1 * __rspice_inv_cse_6);
                let eq21_e245_d_n2: f64 = (var_re_dn2 * __rspice_inv_cse_6);
                let eq21_e245_d_n4: f64 = (var_re_dn4 * __rspice_inv_cse_6);
                (eq21_e245, eq21_e245_d_n1, eq21_e245_d_n2, eq21_e245_d_n4,)
            } else {
                (p.p46, 0.0, 0.0, 0.0,)
            }
        };
        let eq21_e248: f64 = ((nv1 - nv4) / eq21_e247);
        let eq21_e248_d_n1: f64 = ((eq21_e247 - ((nv1 - nv4) * eq21_e247_d_n1)) / (eq21_e247 * eq21_e247));
        let eq21_e248_d_n2: f64 = (-(((nv1 - nv4) * eq21_e247_d_n2) / (eq21_e247 * eq21_e247)));
        let eq21_e248_d_n4: f64 = (((-eq21_e247) - ((nv1 - nv4) * eq21_e247_d_n4)) / (eq21_e247 * eq21_e247));
        (eq21_e248, eq21_e248_d_n1, eq21_e248_d_n2, eq21_e248_d_n4,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e250;
        stamper.stamp_current_node3_local(
            Some(1),
            Some(4),
            multiplicity * (eq21_value),
            1,
            multiplicity * (eq21_e250_d_n1),
            2,
            multiplicity * (eq21_e250_d_n2),
            4,
            multiplicity * (eq21_e250_d_n4),
        );
        let eq24_e264: f64 = (var_ttype * var_ibe);
        let eq24_e264_d_n2: f64 = (var_ttype * var_ibe_dn2);
        let eq24_e264_d_n3: f64 = (var_ttype * var_ibe_dn3);
        let eq24_e264_d_n4: f64 = (var_ttype * var_ibe_dn4);
        let eq24_e266: f64 = (eq24_e264 * var_weff);
        let eq24_e266_d_n2: f64 = (eq24_e264_d_n2 * var_weff);
        let eq24_e266_d_n3: f64 = (eq24_e264_d_n3 * var_weff);
        let eq24_e266_d_n4: f64 = (eq24_e264_d_n4 * var_weff);
        let eq24_value: f64 = eq24_e266;
        stamper.stamp_current_node3_local(
            Some(3),
            Some(4),
            multiplicity * (eq24_value),
            2,
            multiplicity * (eq24_e266_d_n2),
            3,
            multiplicity * (eq24_e266_d_n3),
            4,
            multiplicity * (eq24_e266_d_n4),
        );
        let eq25_e269: f64 = (var_ttype * var_qje);
        let eq25_e269_d_n2: f64 = (var_ttype * var_qje_dn2);
        let eq25_e269_d_n3: f64 = (var_ttype * var_qje_dn3);
        let eq25_e269_d_n4: f64 = (var_ttype * var_qje_dn4);
        let eq25_e271: f64 = (eq25_e269 * var_weff);
        let eq25_e271_d_n2: f64 = (eq25_e269_d_n2 * var_weff);
        let eq25_e271_d_n3: f64 = (eq25_e269_d_n3 * var_weff);
        let eq25_e271_d_n4: f64 = (eq25_e269_d_n4 * var_weff);
        let eq25_e272: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq25_e271);
        let eq25_value: f64 = eq25_e272;
        stamper.stamp_current_node3_local(
            Some(3),
            Some(4),
            multiplicity * (eq25_value),
            2,
            multiplicity * ((eq25_e271_d_n2 * ddt_scale)),
            3,
            multiplicity * ((eq25_e271_d_n3 * ddt_scale)),
            4,
            multiplicity * ((eq25_e271_d_n4 * ddt_scale)),
        );
        let eq26_e275: f64 = (var_ttype * var_qde);
        let eq26_e275_d_n0: f64 = (var_ttype * var_qde_dn0);
        let eq26_e275_d_n1: f64 = (var_ttype * var_qde_dn1);
        let eq26_e275_d_n2: f64 = (var_ttype * var_qde_dn2);
        let eq26_e275_d_n3: f64 = (var_ttype * var_qde_dn3);
        let eq26_e275_d_n4: f64 = (var_ttype * var_qde_dn4);
        let eq26_e277: f64 = (eq26_e275 * var_weff);
        let eq26_e277_d_n0: f64 = (eq26_e275_d_n0 * var_weff);
        let eq26_e277_d_n1: f64 = (eq26_e275_d_n1 * var_weff);
        let eq26_e277_d_n2: f64 = (eq26_e275_d_n2 * var_weff);
        let eq26_e277_d_n3: f64 = (eq26_e275_d_n3 * var_weff);
        let eq26_e277_d_n4: f64 = (eq26_e275_d_n4 * var_weff);
        let eq26_e278: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq26_e277);
        let eq26_value: f64 = eq26_e278;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(3),
            Some(4),
            multiplicity * (eq26_value),
            [0, 1, 2, 3, 4],
            [multiplicity * ((eq26_e277_d_n0 * ddt_scale)), multiplicity * ((eq26_e277_d_n1 * ddt_scale)), multiplicity * ((eq26_e277_d_n2 * ddt_scale)), multiplicity * ((eq26_e277_d_n3 * ddt_scale)), multiplicity * ((eq26_e277_d_n4 * ddt_scale))],
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
        var_guard10: f64,
        var_guard11: f64,
        var_guard8: f64,
        var_qde: f64,
        var_qde_db0: f64,
        var_qde_db1: f64,
        var_qde_db2: f64,
        var_qde_db3: f64,
        var_qde_db4: f64,
        var_qde_db5: f64,
        var_qde_db6: f64,
        var_qde_dn0: f64,
        var_qde_dn1: f64,
        var_qde_dn2: f64,
        var_qde_dn3: f64,
        var_qde_dn4: f64,
        var_qde_dn5: f64,
        var_qde_dn6: f64,
        var_qje: f64,
        var_qje_db0: f64,
        var_qje_db1: f64,
        var_qje_db2: f64,
        var_qje_db3: f64,
        var_qje_db4: f64,
        var_qje_db5: f64,
        var_qje_db6: f64,
        var_qje_dn0: f64,
        var_qje_dn1: f64,
        var_qje_dn2: f64,
        var_qje_dn3: f64,
        var_qje_dn4: f64,
        var_qje_dn5: f64,
        var_qje_dn6: f64,
        var_tff: f64,
        var_tff_db0: f64,
        var_tff_db1: f64,
        var_tff_db2: f64,
        var_tff_db3: f64,
        var_tff_db4: f64,
        var_tff_db5: f64,
        var_tff_db6: f64,
        var_tff_dn0: f64,
        var_tff_dn1: f64,
        var_tff_dn2: f64,
        var_tff_dn3: f64,
        var_tff_dn4: f64,
        var_tff_dn5: f64,
        var_tff_dn6: f64,
        var_ttype: f64,
        var_ttype_db0: f64,
        var_ttype_db1: f64,
        var_ttype_db2: f64,
        var_ttype_db3: f64,
        var_ttype_db4: f64,
        var_ttype_db5: f64,
        var_ttype_db6: f64,
        var_ttype_dn0: f64,
        var_ttype_dn1: f64,
        var_ttype_dn2: f64,
        var_ttype_dn3: f64,
        var_ttype_dn4: f64,
        var_ttype_dn5: f64,
        var_ttype_dn6: f64,
        var_weff: f64,
        var_weff_db0: f64,
        var_weff_db1: f64,
        var_weff_db2: f64,
        var_weff_db3: f64,
        var_weff_db4: f64,
        var_weff_db5: f64,
        var_weff_db6: f64,
        var_weff_dn0: f64,
        var_weff_dn1: f64,
        var_weff_dn2: f64,
        var_weff_dn3: f64,
        var_weff_dn4: f64,
        var_weff_dn5: f64,
        var_weff_dn6: f64,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq2_e73, eq2_e73_d_n0, eq2_e73_d_n1, eq2_e73_d_n2, eq2_e73_d_n3, eq2_e73_d_n4, eq2_e73_d_n5, eq2_e73_d_n6, eq2_e73_d_b0, eq2_e73_d_b1, eq2_e73_d_b2, eq2_e73_d_b3, eq2_e73_d_b4, eq2_e73_d_b5, eq2_e73_d_b6, eq2_e73_q, eq2_e73_q_d_n0, eq2_e73_q_d_n1, eq2_e73_q_d_n2, eq2_e73_q_d_n3, eq2_e73_q_d_n4, eq2_e73_q_d_n5, eq2_e73_q_d_n6, eq2_e73_q_d_b0, eq2_e73_q_d_b1, eq2_e73_q_d_b2, eq2_e73_q_d_b3, eq2_e73_q_d_b4, eq2_e73_q_d_b5, eq2_e73_q_d_b6,) = {
    if (var_guard8 != 0.0) {
        let eq2_e70_q: f64 = (nv6 - 0.0);
        let eq2_e71: f64 = (var_tff * (nv6 - 0.0));
        let eq2_e71_d_n0: f64 = (var_tff_dn0 * (nv6 - 0.0));
        let eq2_e71_d_n1: f64 = (var_tff_dn1 * (nv6 - 0.0));
        let eq2_e71_d_n2: f64 = (var_tff_dn2 * (nv6 - 0.0));
        let eq2_e71_d_n3: f64 = (var_tff_dn3 * (nv6 - 0.0));
        let eq2_e71_d_n4: f64 = (var_tff_dn4 * (nv6 - 0.0));
        let eq2_e71_d_n5: f64 = (var_tff_dn5 * (nv6 - 0.0));
        let eq2_e71_d_n6: f64 = ((var_tff_dn6 * (nv6 - 0.0)) + var_tff);
        let eq2_e71_d_b0: f64 = (var_tff_db0 * (nv6 - 0.0));
        let eq2_e71_d_b1: f64 = (var_tff_db1 * (nv6 - 0.0));
        let eq2_e71_d_b2: f64 = (var_tff_db2 * (nv6 - 0.0));
        let eq2_e71_d_b3: f64 = (var_tff_db3 * (nv6 - 0.0));
        let eq2_e71_d_b4: f64 = (var_tff_db4 * (nv6 - 0.0));
        let eq2_e71_d_b5: f64 = (var_tff_db5 * (nv6 - 0.0));
        let eq2_e71_d_b6: f64 = (var_tff_db6 * (nv6 - 0.0));
        let eq2_e71_q: f64 = (var_tff * eq2_e70_q);
        let eq2_e71_q_d_n0: f64 = (var_tff_dn0 * eq2_e70_q);
        let eq2_e71_q_d_n1: f64 = (var_tff_dn1 * eq2_e70_q);
        let eq2_e71_q_d_n2: f64 = (var_tff_dn2 * eq2_e70_q);
        let eq2_e71_q_d_n3: f64 = (var_tff_dn3 * eq2_e70_q);
        let eq2_e71_q_d_n4: f64 = (var_tff_dn4 * eq2_e70_q);
        let eq2_e71_q_d_n5: f64 = (var_tff_dn5 * eq2_e70_q);
        let eq2_e71_q_d_n6: f64 = ((var_tff_dn6 * eq2_e70_q) + var_tff);
        let eq2_e71_q_d_b0: f64 = (var_tff_db0 * eq2_e70_q);
        let eq2_e71_q_d_b1: f64 = (var_tff_db1 * eq2_e70_q);
        let eq2_e71_q_d_b2: f64 = (var_tff_db2 * eq2_e70_q);
        let eq2_e71_q_d_b3: f64 = (var_tff_db3 * eq2_e70_q);
        let eq2_e71_q_d_b4: f64 = (var_tff_db4 * eq2_e70_q);
        let eq2_e71_q_d_b5: f64 = (var_tff_db5 * eq2_e70_q);
        let eq2_e71_q_d_b6: f64 = (var_tff_db6 * eq2_e70_q);
        (eq2_e71, eq2_e71_d_n0, eq2_e71_d_n1, eq2_e71_d_n2, eq2_e71_d_n3, eq2_e71_d_n4, eq2_e71_d_n5, eq2_e71_d_n6, eq2_e71_d_b0, eq2_e71_d_b1, eq2_e71_d_b2, eq2_e71_d_b3, eq2_e71_d_b4, eq2_e71_d_b5, eq2_e71_d_b6, eq2_e71_q, eq2_e71_q_d_n0, eq2_e71_q_d_n1, eq2_e71_q_d_n2, eq2_e71_q_d_n3, eq2_e71_q_d_n4, eq2_e71_q_d_n5, eq2_e71_q_d_n6, eq2_e71_q_d_b0, eq2_e71_q_d_b1, eq2_e71_q_d_b2, eq2_e71_q_d_b3, eq2_e71_q_d_b4, eq2_e71_q_d_b5, eq2_e71_q_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_reactive_node_derivatives: [f64; 7] = [eq2_e73_q_d_n0, eq2_e73_q_d_n1, eq2_e73_q_d_n2, eq2_e73_q_d_n3, eq2_e73_q_d_n4, eq2_e73_q_d_n5, eq2_e73_q_d_n6];
        let eq2_reactive_branch_derivatives: [f64; 7] = [eq2_e73_q_d_b0, eq2_e73_q_d_b1, eq2_e73_q_d_b2, eq2_e73_q_d_b3, eq2_e73_q_d_b4, eq2_e73_q_d_b5, eq2_e73_q_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            None,
            nodes,
            &eq2_reactive_node_derivatives,
            branches,
            &eq2_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq6_e101, eq6_e101_d_n2, eq6_e101_q,) = {
    if (var_guard10 != 0.0) {
        let eq6_e98: f64 = ((nv2 - 0.0) * p.p34);
        let eq6_e99_q: f64 = eq6_e98;
        (eq6_e98, p.p34, eq6_e99_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[2]),
            None,
            nodes[2],
            multiplicity * (eq6_e101_d_n2),
        );
        let (eq10_e137, eq10_e137_d_n2, eq10_e137_q,) = {
    if ((var_guard10 == 0.0) && (var_guard11 != 0.0)) {
        let eq10_e134: f64 = (p.p34 * (nv2 - 0.0));
        let eq10_e135_q: f64 = eq10_e134;
        (eq10_e134, p.p34, eq10_e135_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[2]),
            None,
            nodes[2],
            multiplicity * (eq10_e137_d_n2),
        );
        let (eq12_e156, eq12_e156_d_n5, eq12_e156_q,) = {
    if ((var_guard10 == 0.0) && (var_guard11 != 0.0)) {
        let eq12_e153: f64 = (p.p36 * (nv5 - 0.0));
        let eq12_e154_q: f64 = eq12_e153;
        (eq12_e153, p.p36, eq12_e154_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[5]),
            None,
            nodes[5],
            multiplicity * (eq12_e156_d_n5),
        );
        let eq25_e269: f64 = (var_ttype * var_qje);
        let eq25_e269_d_n0: f64 = ((var_ttype_dn0 * var_qje) + (var_ttype * var_qje_dn0));
        let eq25_e269_d_n1: f64 = ((var_ttype_dn1 * var_qje) + (var_ttype * var_qje_dn1));
        let eq25_e269_d_n2: f64 = ((var_ttype_dn2 * var_qje) + (var_ttype * var_qje_dn2));
        let eq25_e269_d_n3: f64 = ((var_ttype_dn3 * var_qje) + (var_ttype * var_qje_dn3));
        let eq25_e269_d_n4: f64 = ((var_ttype_dn4 * var_qje) + (var_ttype * var_qje_dn4));
        let eq25_e269_d_n5: f64 = ((var_ttype_dn5 * var_qje) + (var_ttype * var_qje_dn5));
        let eq25_e269_d_n6: f64 = ((var_ttype_dn6 * var_qje) + (var_ttype * var_qje_dn6));
        let eq25_e269_d_b0: f64 = ((var_ttype_db0 * var_qje) + (var_ttype * var_qje_db0));
        let eq25_e269_d_b1: f64 = ((var_ttype_db1 * var_qje) + (var_ttype * var_qje_db1));
        let eq25_e269_d_b2: f64 = ((var_ttype_db2 * var_qje) + (var_ttype * var_qje_db2));
        let eq25_e269_d_b3: f64 = ((var_ttype_db3 * var_qje) + (var_ttype * var_qje_db3));
        let eq25_e269_d_b4: f64 = ((var_ttype_db4 * var_qje) + (var_ttype * var_qje_db4));
        let eq25_e269_d_b5: f64 = ((var_ttype_db5 * var_qje) + (var_ttype * var_qje_db5));
        let eq25_e269_d_b6: f64 = ((var_ttype_db6 * var_qje) + (var_ttype * var_qje_db6));
        let eq25_e271: f64 = (eq25_e269 * var_weff);
        let eq25_e271_d_n0: f64 = ((eq25_e269_d_n0 * var_weff) + (eq25_e269 * var_weff_dn0));
        let eq25_e271_d_n1: f64 = ((eq25_e269_d_n1 * var_weff) + (eq25_e269 * var_weff_dn1));
        let eq25_e271_d_n2: f64 = ((eq25_e269_d_n2 * var_weff) + (eq25_e269 * var_weff_dn2));
        let eq25_e271_d_n3: f64 = ((eq25_e269_d_n3 * var_weff) + (eq25_e269 * var_weff_dn3));
        let eq25_e271_d_n4: f64 = ((eq25_e269_d_n4 * var_weff) + (eq25_e269 * var_weff_dn4));
        let eq25_e271_d_n5: f64 = ((eq25_e269_d_n5 * var_weff) + (eq25_e269 * var_weff_dn5));
        let eq25_e271_d_n6: f64 = ((eq25_e269_d_n6 * var_weff) + (eq25_e269 * var_weff_dn6));
        let eq25_e271_d_b0: f64 = ((eq25_e269_d_b0 * var_weff) + (eq25_e269 * var_weff_db0));
        let eq25_e271_d_b1: f64 = ((eq25_e269_d_b1 * var_weff) + (eq25_e269 * var_weff_db1));
        let eq25_e271_d_b2: f64 = ((eq25_e269_d_b2 * var_weff) + (eq25_e269 * var_weff_db2));
        let eq25_e271_d_b3: f64 = ((eq25_e269_d_b3 * var_weff) + (eq25_e269 * var_weff_db3));
        let eq25_e271_d_b4: f64 = ((eq25_e269_d_b4 * var_weff) + (eq25_e269 * var_weff_db4));
        let eq25_e271_d_b5: f64 = ((eq25_e269_d_b5 * var_weff) + (eq25_e269 * var_weff_db5));
        let eq25_e271_d_b6: f64 = ((eq25_e269_d_b6 * var_weff) + (eq25_e269 * var_weff_db6));
        let eq25_e272_q: f64 = eq25_e271;
        let eq25_reactive_node_derivatives: [f64; 7] = [eq25_e271_d_n0, eq25_e271_d_n1, eq25_e271_d_n2, eq25_e271_d_n3, eq25_e271_d_n4, eq25_e271_d_n5, eq25_e271_d_n6];
        let eq25_reactive_branch_derivatives: [f64; 7] = [eq25_e271_d_b0, eq25_e271_d_b1, eq25_e271_d_b2, eq25_e271_d_b3, eq25_e271_d_b4, eq25_e271_d_b5, eq25_e271_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[4]),
            nodes,
            &eq25_reactive_node_derivatives,
            branches,
            &eq25_reactive_branch_derivatives,
            multiplicity,
        );
        let eq26_e275: f64 = (var_ttype * var_qde);
        let eq26_e275_d_n0: f64 = ((var_ttype_dn0 * var_qde) + (var_ttype * var_qde_dn0));
        let eq26_e275_d_n1: f64 = ((var_ttype_dn1 * var_qde) + (var_ttype * var_qde_dn1));
        let eq26_e275_d_n2: f64 = ((var_ttype_dn2 * var_qde) + (var_ttype * var_qde_dn2));
        let eq26_e275_d_n3: f64 = ((var_ttype_dn3 * var_qde) + (var_ttype * var_qde_dn3));
        let eq26_e275_d_n4: f64 = ((var_ttype_dn4 * var_qde) + (var_ttype * var_qde_dn4));
        let eq26_e275_d_n5: f64 = ((var_ttype_dn5 * var_qde) + (var_ttype * var_qde_dn5));
        let eq26_e275_d_n6: f64 = ((var_ttype_dn6 * var_qde) + (var_ttype * var_qde_dn6));
        let eq26_e275_d_b0: f64 = ((var_ttype_db0 * var_qde) + (var_ttype * var_qde_db0));
        let eq26_e275_d_b1: f64 = ((var_ttype_db1 * var_qde) + (var_ttype * var_qde_db1));
        let eq26_e275_d_b2: f64 = ((var_ttype_db2 * var_qde) + (var_ttype * var_qde_db2));
        let eq26_e275_d_b3: f64 = ((var_ttype_db3 * var_qde) + (var_ttype * var_qde_db3));
        let eq26_e275_d_b4: f64 = ((var_ttype_db4 * var_qde) + (var_ttype * var_qde_db4));
        let eq26_e275_d_b5: f64 = ((var_ttype_db5 * var_qde) + (var_ttype * var_qde_db5));
        let eq26_e275_d_b6: f64 = ((var_ttype_db6 * var_qde) + (var_ttype * var_qde_db6));
        let eq26_e277: f64 = (eq26_e275 * var_weff);
        let eq26_e277_d_n0: f64 = ((eq26_e275_d_n0 * var_weff) + (eq26_e275 * var_weff_dn0));
        let eq26_e277_d_n1: f64 = ((eq26_e275_d_n1 * var_weff) + (eq26_e275 * var_weff_dn1));
        let eq26_e277_d_n2: f64 = ((eq26_e275_d_n2 * var_weff) + (eq26_e275 * var_weff_dn2));
        let eq26_e277_d_n3: f64 = ((eq26_e275_d_n3 * var_weff) + (eq26_e275 * var_weff_dn3));
        let eq26_e277_d_n4: f64 = ((eq26_e275_d_n4 * var_weff) + (eq26_e275 * var_weff_dn4));
        let eq26_e277_d_n5: f64 = ((eq26_e275_d_n5 * var_weff) + (eq26_e275 * var_weff_dn5));
        let eq26_e277_d_n6: f64 = ((eq26_e275_d_n6 * var_weff) + (eq26_e275 * var_weff_dn6));
        let eq26_e277_d_b0: f64 = ((eq26_e275_d_b0 * var_weff) + (eq26_e275 * var_weff_db0));
        let eq26_e277_d_b1: f64 = ((eq26_e275_d_b1 * var_weff) + (eq26_e275 * var_weff_db1));
        let eq26_e277_d_b2: f64 = ((eq26_e275_d_b2 * var_weff) + (eq26_e275 * var_weff_db2));
        let eq26_e277_d_b3: f64 = ((eq26_e275_d_b3 * var_weff) + (eq26_e275 * var_weff_db3));
        let eq26_e277_d_b4: f64 = ((eq26_e275_d_b4 * var_weff) + (eq26_e275 * var_weff_db4));
        let eq26_e277_d_b5: f64 = ((eq26_e275_d_b5 * var_weff) + (eq26_e275 * var_weff_db5));
        let eq26_e277_d_b6: f64 = ((eq26_e275_d_b6 * var_weff) + (eq26_e275 * var_weff_db6));
        let eq26_e278_q: f64 = eq26_e277;
        let eq26_reactive_node_derivatives: [f64; 7] = [eq26_e277_d_n0, eq26_e277_d_n1, eq26_e277_d_n2, eq26_e277_d_n3, eq26_e277_d_n4, eq26_e277_d_n5, eq26_e277_d_n6];
        let eq26_reactive_branch_derivatives: [f64; 7] = [eq26_e277_d_b0, eq26_e277_d_b1, eq26_e277_d_b2, eq26_e277_d_b3, eq26_e277_d_b4, eq26_e277_d_b5, eq26_e277_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[4]),
            nodes,
            &eq26_reactive_node_derivatives,
            branches,
            &eq26_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
