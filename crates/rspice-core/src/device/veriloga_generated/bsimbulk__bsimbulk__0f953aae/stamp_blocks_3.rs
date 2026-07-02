#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_5(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign4000_e5242: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard37 = assign4000_e5242;
        locals.var_guard37_rv = 0.0;

        if (locals.var_guard37 != 0.0) {
            let assign4010_e5247: f64 = (1.0 + locals.var_t0);
            let assign4010_e5249: f64 = (assign4010_e5247 + locals.var_t1);
            let assign4010_e5250: f64 = (locals.var_alpha0r_i * assign4010_e5249);
            (locals.var_alpha0r_i, locals.var_alpha0r_i_dn0, locals.var_alpha0r_i_dn2, locals.var_alpha0r_i_dn3, locals.var_alpha0r_i_dn4, locals.var_alpha0r_i_dn5, locals.var_alpha0r_i_dn6, locals.var_alpha0r_i_dn7, locals.var_alpha0r_i_dn8, locals.var_alpha0r_i_dn9, locals.var_alpha0r_i_dn10, locals.var_alpha0r_i_dn11, locals.var_alpha0r_i_dn12, locals.var_alpha0r_i_dn13, locals.var_alpha0r_i_dn14, ) = (assign4010_e5250, ((locals.var_alpha0r_i_dn0 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn0 + locals.var_t1_dn0))), ((locals.var_alpha0r_i_dn2 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn2 + locals.var_t1_dn2))), ((locals.var_alpha0r_i_dn3 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn3 + locals.var_t1_dn3))), ((locals.var_alpha0r_i_dn4 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn4 + locals.var_t1_dn4))), ((locals.var_alpha0r_i_dn5 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn5 + locals.var_t1_dn5))), ((locals.var_alpha0r_i_dn6 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn6 + locals.var_t1_dn6))), ((locals.var_alpha0r_i_dn7 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn7 + locals.var_t1_dn7))), ((locals.var_alpha0r_i_dn8 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn8 + locals.var_t1_dn8))), ((locals.var_alpha0r_i_dn9 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn9 + locals.var_t1_dn9))), ((locals.var_alpha0r_i_dn10 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn10 + locals.var_t1_dn10))), ((locals.var_alpha0r_i_dn11 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn11 + locals.var_t1_dn11))), ((locals.var_alpha0r_i_dn12 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn12 + locals.var_t1_dn12))), ((locals.var_alpha0r_i_dn13 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn13 + locals.var_t1_dn13))), ((locals.var_alpha0r_i_dn14 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn14 + locals.var_t1_dn14))), );
            locals.var_alpha0r_i_rv = 0.0;
        }

        let assign4020_e5256: f64 = (locals.var_inv_w).powf(p.p496);
        let assign4020_e5259: f64 = (locals.var_inv_wwide).powf(p.p496);
        let assign4020_e5260: f64 = (assign4020_e5256 - assign4020_e5259);
        let assign4020_e5262: f64 = (assign4020_e5260).max(0.0);
        let assign4020_e5263: f64 = (p.p495 * assign4020_e5262);
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, ) = (assign4020_e5263, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_t1_rv = 0.0;

        let assign4030_e5267: f64 = (1.0 + locals.var_t1);
        let assign4030_e5268: f64 = (locals.var_beta0_i * assign4030_e5267);
        (locals.var_beta0_i, locals.var_beta0_i_dn0, locals.var_beta0_i_dn2, locals.var_beta0_i_dn3, locals.var_beta0_i_dn4, locals.var_beta0_i_dn5, locals.var_beta0_i_dn6, locals.var_beta0_i_dn7, locals.var_beta0_i_dn8, locals.var_beta0_i_dn9, locals.var_beta0_i_dn10, locals.var_beta0_i_dn11, locals.var_beta0_i_dn12, locals.var_beta0_i_dn13, locals.var_beta0_i_dn14, ) = (assign4030_e5268, ((locals.var_beta0_i_dn0 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn0)), ((locals.var_beta0_i_dn2 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn2)), ((locals.var_beta0_i_dn3 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn3)), ((locals.var_beta0_i_dn4 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn4)), ((locals.var_beta0_i_dn5 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn5)), ((locals.var_beta0_i_dn6 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn6)), ((locals.var_beta0_i_dn7 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn7)), ((locals.var_beta0_i_dn8 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn8)), ((locals.var_beta0_i_dn9 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn9)), ((locals.var_beta0_i_dn10 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn10)), ((locals.var_beta0_i_dn11 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn11)), ((locals.var_beta0_i_dn12 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn12)), ((locals.var_beta0_i_dn13 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn13)), ((locals.var_beta0_i_dn14 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn14)), );
        locals.var_beta0_i_rv = 0.0;

        let assign4040_e5272: f64 = (locals.var_inv_w).powf(p.p520);
        let assign4040_e5275: f64 = (locals.var_inv_wwide).powf(p.p520);
        let assign4040_e5276: f64 = (assign4040_e5272 - assign4040_e5275);
        let assign4040_e5278: f64 = (assign4040_e5276).max(0.0);
        let assign4040_e5279: f64 = (p.p519 * assign4040_e5278);
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, ) = (assign4040_e5279, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_t1_rv = 0.0;

        (locals.var_beta1_i, locals.var_beta1_i_dn0, locals.var_beta1_i_dn2, locals.var_beta1_i_dn3, locals.var_beta1_i_dn4, locals.var_beta1_i_dn5, locals.var_beta1_i_dn6, locals.var_beta1_i_dn7, locals.var_beta1_i_dn8, locals.var_beta1_i_dn9, locals.var_beta1_i_dn10, locals.var_beta1_i_dn11, locals.var_beta1_i_dn12, locals.var_beta1_i_dn13, locals.var_beta1_i_dn14, ) = (p.p518, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_beta1_i_rv = 0.0;

        let assign4060_e5284: f64 = (1.0 + locals.var_t1);
        let assign4060_e5285: f64 = (locals.var_beta1_i * assign4060_e5284);
        (locals.var_beta1_i, locals.var_beta1_i_dn0, locals.var_beta1_i_dn2, locals.var_beta1_i_dn3, locals.var_beta1_i_dn4, locals.var_beta1_i_dn5, locals.var_beta1_i_dn6, locals.var_beta1_i_dn7, locals.var_beta1_i_dn8, locals.var_beta1_i_dn9, locals.var_beta1_i_dn10, locals.var_beta1_i_dn11, locals.var_beta1_i_dn12, locals.var_beta1_i_dn13, locals.var_beta1_i_dn14, ) = (assign4060_e5285, ((locals.var_beta1_i_dn0 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn0)), ((locals.var_beta1_i_dn2 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn2)), ((locals.var_beta1_i_dn3 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn3)), ((locals.var_beta1_i_dn4 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn4)), ((locals.var_beta1_i_dn5 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn5)), ((locals.var_beta1_i_dn6 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn6)), ((locals.var_beta1_i_dn7 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn7)), ((locals.var_beta1_i_dn8 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn8)), ((locals.var_beta1_i_dn9 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn9)), ((locals.var_beta1_i_dn10 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn10)), ((locals.var_beta1_i_dn11 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn11)), ((locals.var_beta1_i_dn12 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn12)), ((locals.var_beta1_i_dn13 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn13)), ((locals.var_beta1_i_dn14 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn14)), );
        locals.var_beta1_i_rv = 0.0;

        let assign4070_e5289: f64 = (locals.var_inv_w).powf(p.p523);
        let assign4070_e5292: f64 = (locals.var_inv_wwide).powf(p.p523);
        let assign4070_e5293: f64 = (assign4070_e5289 - assign4070_e5292);
        let assign4070_e5295: f64 = (assign4070_e5293).max(0.0);
        let assign4070_e5296: f64 = (p.p522 * assign4070_e5295);
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, ) = (assign4070_e5296, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_t1_rv = 0.0;

        (locals.var_beta2_i, locals.var_beta2_i_dn0, locals.var_beta2_i_dn2, locals.var_beta2_i_dn3, locals.var_beta2_i_dn4, locals.var_beta2_i_dn5, locals.var_beta2_i_dn6, locals.var_beta2_i_dn7, locals.var_beta2_i_dn8, locals.var_beta2_i_dn9, locals.var_beta2_i_dn10, locals.var_beta2_i_dn11, locals.var_beta2_i_dn12, locals.var_beta2_i_dn13, locals.var_beta2_i_dn14, ) = (p.p521, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_beta2_i_rv = 0.0;

        let assign4090_e5301: f64 = (1.0 + locals.var_t1);
        let assign4090_e5302: f64 = (locals.var_beta2_i * assign4090_e5301);
        (locals.var_beta2_i, locals.var_beta2_i_dn0, locals.var_beta2_i_dn2, locals.var_beta2_i_dn3, locals.var_beta2_i_dn4, locals.var_beta2_i_dn5, locals.var_beta2_i_dn6, locals.var_beta2_i_dn7, locals.var_beta2_i_dn8, locals.var_beta2_i_dn9, locals.var_beta2_i_dn10, locals.var_beta2_i_dn11, locals.var_beta2_i_dn12, locals.var_beta2_i_dn13, locals.var_beta2_i_dn14, ) = (assign4090_e5302, ((locals.var_beta2_i_dn0 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn0)), ((locals.var_beta2_i_dn2 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn2)), ((locals.var_beta2_i_dn3 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn3)), ((locals.var_beta2_i_dn4 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn4)), ((locals.var_beta2_i_dn5 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn5)), ((locals.var_beta2_i_dn6 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn6)), ((locals.var_beta2_i_dn7 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn7)), ((locals.var_beta2_i_dn8 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn8)), ((locals.var_beta2_i_dn9 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn9)), ((locals.var_beta2_i_dn10 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn10)), ((locals.var_beta2_i_dn11 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn11)), ((locals.var_beta2_i_dn12 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn12)), ((locals.var_beta2_i_dn13 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn13)), ((locals.var_beta2_i_dn14 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn14)), );
        locals.var_beta2_i_rv = 0.0;

        let assign4100_e5307: f64 = (p.p631 * locals.var_inv_l);
        let assign4100_e5308: f64 = (1.0 + assign4100_e5307);
        let assign4100_e5311: f64 = (p.p632 * locals.var_inv_w);
        let assign4100_e5312: f64 = (assign4100_e5308 + assign4100_e5311);
        let assign4100_e5313: f64 = (locals.var_agidl_i * assign4100_e5312);
        locals.var_agidl_i = assign4100_e5313;
        locals.var_agidl_i_rv = 0.0;

        let assign4110_e5318: f64 = (p.p649 * locals.var_inv_l);
        let assign4110_e5319: f64 = (1.0 + assign4110_e5318);
        let assign4110_e5322: f64 = (p.p650 * locals.var_inv_w);
        let assign4110_e5323: f64 = (assign4110_e5319 + assign4110_e5322);
        let assign4110_e5324: f64 = (locals.var_agisl_i * assign4110_e5323);
        locals.var_agisl_i = assign4110_e5324;
        locals.var_agisl_i_rv = 0.0;

        let assign4120_e5329: f64 = (p.p557 * locals.var_inv_l);
        let assign4120_e5330: f64 = (1.0 + assign4120_e5329);
        let assign4120_e5333: f64 = (p.p558 * locals.var_inv_w);
        let assign4120_e5334: f64 = (assign4120_e5330 + assign4120_e5333);
        let assign4120_e5335: f64 = (locals.var_aigc_i * assign4120_e5334);
        locals.var_aigc_i = assign4120_e5335;
        locals.var_aigc_i_rv = 0.0;

        let assign4130_e5340: f64 = (p.p559 * locals.var_inv_l);
        let assign4130_e5341: f64 = (1.0 + assign4130_e5340);
        let assign4130_e5344: f64 = (p.p560 * locals.var_inv_w);
        let assign4130_e5345: f64 = (assign4130_e5341 + assign4130_e5344);
        let assign4130_e5346: f64 = (locals.var_aigs_i * assign4130_e5345);
        locals.var_aigs_i = assign4130_e5346;
        locals.var_aigs_i_rv = 0.0;

        let assign4140_e5351: f64 = (p.p561 * locals.var_inv_l);
        let assign4140_e5352: f64 = (1.0 + assign4140_e5351);
        let assign4140_e5355: f64 = (p.p562 * locals.var_inv_w);
        let assign4140_e5356: f64 = (assign4140_e5352 + assign4140_e5355);
        let assign4140_e5357: f64 = (locals.var_aigd_i * assign4140_e5356);
        locals.var_aigd_i = assign4140_e5357;
        locals.var_aigd_i_rv = 0.0;

        let assign4150_e5362: f64 = (p.p563 * locals.var_inv_l);
        let assign4150_e5363: f64 = (1.0 + assign4150_e5362);
        let assign4150_e5364: f64 = (p.p556 * assign4150_e5363);
        locals.var_pigcd_i = assign4150_e5364;
        locals.var_pigcd_i_rv = 0.0;

        let assign4160_e5368: f64 = (locals.var_inv_lact).powf(p.p94);
        let assign4160_e5371: f64 = (locals.var_inv_llong).powf(p.p94);
        let assign4160_e5372: f64 = (assign4160_e5368 - assign4160_e5371);
        let assign4160_e5374: f64 = (assign4160_e5372).max(0.0);
        let assign4160_e5375: f64 = (p.p93 * assign4160_e5374);
        let assign4160_e5379: f64 = (locals.var_inv_lact).powf(p.p96);
        let assign4160_e5382: f64 = (locals.var_inv_llong).powf(p.p96);
        let assign4160_e5383: f64 = (assign4160_e5379 - assign4160_e5382);
        let assign4160_e5385: f64 = (assign4160_e5383).max(0.0);
        let assign4160_e5386: f64 = (p.p95 * assign4160_e5385);
        let assign4160_e5387: f64 = (assign4160_e5375 + assign4160_e5386);
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign4160_e5387, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_t0_rv = 0.0;

        let assign4170_e5391: f64 = (locals.var_inv_wact).powf(p.p98);
        let assign4170_e5394: f64 = (locals.var_inv_wwide).powf(p.p98);
        let assign4170_e5395: f64 = (assign4170_e5391 - assign4170_e5394);
        let assign4170_e5397: f64 = (assign4170_e5395).max(0.0);
        let assign4170_e5398: f64 = (p.p97 * assign4170_e5397);
        let assign4170_e5402: f64 = (locals.var_inv_wact * locals.var_inv_lact);
        let assign4170_e5404: f64 = (assign4170_e5402).powf(p.p100);
        let assign4170_e5405: f64 = (p.p99 * assign4170_e5404);
        let assign4170_e5406: f64 = (assign4170_e5398 + assign4170_e5405);
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, ) = (assign4170_e5406, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_t1_rv = 0.0;

        let assign4180_e5410: f64 = (1.0 + locals.var_t0);
        let assign4180_e5412: f64 = (assign4180_e5410 + locals.var_t1);
        let assign4180_e5413: f64 = (locals.var_ndepcv_i * assign4180_e5412);
        (locals.var_ndepcv_i, locals.var_ndepcv_i_dn0, locals.var_ndepcv_i_dn2, locals.var_ndepcv_i_dn3, locals.var_ndepcv_i_dn4, locals.var_ndepcv_i_dn5, locals.var_ndepcv_i_dn6, locals.var_ndepcv_i_dn7, locals.var_ndepcv_i_dn8, locals.var_ndepcv_i_dn9, locals.var_ndepcv_i_dn10, locals.var_ndepcv_i_dn11, locals.var_ndepcv_i_dn12, locals.var_ndepcv_i_dn13, locals.var_ndepcv_i_dn14, ) = (assign4180_e5413, ((locals.var_ndepcv_i_dn0 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn0 + locals.var_t1_dn0))), ((locals.var_ndepcv_i_dn2 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn2 + locals.var_t1_dn2))), ((locals.var_ndepcv_i_dn3 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn3 + locals.var_t1_dn3))), ((locals.var_ndepcv_i_dn4 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn4 + locals.var_t1_dn4))), ((locals.var_ndepcv_i_dn5 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn5 + locals.var_t1_dn5))), ((locals.var_ndepcv_i_dn6 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn6 + locals.var_t1_dn6))), ((locals.var_ndepcv_i_dn7 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn7 + locals.var_t1_dn7))), ((locals.var_ndepcv_i_dn8 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn8 + locals.var_t1_dn8))), ((locals.var_ndepcv_i_dn9 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn9 + locals.var_t1_dn9))), ((locals.var_ndepcv_i_dn10 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn10 + locals.var_t1_dn10))), ((locals.var_ndepcv_i_dn11 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn11 + locals.var_t1_dn11))), ((locals.var_ndepcv_i_dn12 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn12 + locals.var_t1_dn12))), ((locals.var_ndepcv_i_dn13 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn13 + locals.var_t1_dn13))), ((locals.var_ndepcv_i_dn14 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn14 + locals.var_t1_dn14))), );
        locals.var_ndepcv_i_rv = 0.0;

        let assign4190_e5417: f64 = (locals.var_inv_lact).powf(p.p121);
        let assign4190_e5420: f64 = (locals.var_inv_llong).powf(p.p121);
        let assign4190_e5421: f64 = (assign4190_e5417 - assign4190_e5420);
        let assign4190_e5423: f64 = (assign4190_e5421).max(0.0);
        let assign4190_e5424: f64 = (p.p120 * assign4190_e5423);
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign4190_e5424, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_t0_rv = 0.0;

        let assign4200_e5428: f64 = (locals.var_inv_wact).powf(p.p123);
        let assign4200_e5431: f64 = (locals.var_inv_wwide).powf(p.p123);
        let assign4200_e5432: f64 = (assign4200_e5428 - assign4200_e5431);
        let assign4200_e5434: f64 = (assign4200_e5432).max(0.0);
        let assign4200_e5435: f64 = (p.p122 * assign4200_e5434);
        let assign4200_e5439: f64 = (locals.var_inv_wl).powf(p.p125);
        let assign4200_e5440: f64 = (p.p124 * assign4200_e5439);
        let assign4200_e5441: f64 = (assign4200_e5435 + assign4200_e5440);
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, ) = (assign4200_e5441, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_t1_rv = 0.0;

        let assign4210_e5445: f64 = (1.0 + locals.var_t0);
        let assign4210_e5447: f64 = (assign4210_e5445 + locals.var_t1);
        let assign4210_e5448: f64 = (locals.var_vfb_i * assign4210_e5447);
        (locals.var_vfb_i, locals.var_vfb_i_dn0, locals.var_vfb_i_dn2, locals.var_vfb_i_dn3, locals.var_vfb_i_dn4, locals.var_vfb_i_dn5, locals.var_vfb_i_dn6, locals.var_vfb_i_dn7, locals.var_vfb_i_dn8, locals.var_vfb_i_dn9, locals.var_vfb_i_dn10, locals.var_vfb_i_dn11, locals.var_vfb_i_dn12, locals.var_vfb_i_dn13, locals.var_vfb_i_dn14, ) = (assign4210_e5448, ((locals.var_vfb_i_dn0 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn0 + locals.var_t1_dn0))), ((locals.var_vfb_i_dn2 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn2 + locals.var_t1_dn2))), ((locals.var_vfb_i_dn3 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn3 + locals.var_t1_dn3))), ((locals.var_vfb_i_dn4 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn4 + locals.var_t1_dn4))), ((locals.var_vfb_i_dn5 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn5 + locals.var_t1_dn5))), ((locals.var_vfb_i_dn6 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn6 + locals.var_t1_dn6))), ((locals.var_vfb_i_dn7 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn7 + locals.var_t1_dn7))), ((locals.var_vfb_i_dn8 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn8 + locals.var_t1_dn8))), ((locals.var_vfb_i_dn9 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn9 + locals.var_t1_dn9))), ((locals.var_vfb_i_dn10 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn10 + locals.var_t1_dn10))), ((locals.var_vfb_i_dn11 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn11 + locals.var_t1_dn11))), ((locals.var_vfb_i_dn12 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn12 + locals.var_t1_dn12))), ((locals.var_vfb_i_dn13 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn13 + locals.var_t1_dn13))), ((locals.var_vfb_i_dn14 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn14 + locals.var_t1_dn14))), );
        locals.var_vfb_i_rv = 0.0;

        let assign4220_e5452: f64 = (locals.var_inv_lact).powf(p.p131);
        let assign4220_e5455: f64 = (locals.var_inv_llong).powf(p.p131);
        let assign4220_e5456: f64 = (assign4220_e5452 - assign4220_e5455);
        let assign4220_e5458: f64 = (assign4220_e5456).max(0.0);
        let assign4220_e5459: f64 = (p.p130 * assign4220_e5458);
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign4220_e5459, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_t0_rv = 0.0;

        let assign4230_e5463: f64 = (locals.var_inv_wact).powf(p.p133);
        let assign4230_e5466: f64 = (locals.var_inv_wwide).powf(p.p133);
        let assign4230_e5467: f64 = (assign4230_e5463 - assign4230_e5466);
        let assign4230_e5469: f64 = (assign4230_e5467).max(0.0);
        let assign4230_e5470: f64 = (p.p132 * assign4230_e5469);
        let assign4230_e5474: f64 = (locals.var_inv_wl).powf(p.p135);
        let assign4230_e5475: f64 = (p.p134 * assign4230_e5474);
        let assign4230_e5476: f64 = (assign4230_e5470 + assign4230_e5475);
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, ) = (assign4230_e5476, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_t1_rv = 0.0;

        let assign4240_e5480: f64 = (1.0 + locals.var_t0);
        let assign4240_e5482: f64 = (assign4240_e5480 + locals.var_t1);
        let assign4240_e5483: f64 = (locals.var_vfbcv_i * assign4240_e5482);
        (locals.var_vfbcv_i, locals.var_vfbcv_i_dn0, locals.var_vfbcv_i_dn2, locals.var_vfbcv_i_dn3, locals.var_vfbcv_i_dn4, locals.var_vfbcv_i_dn5, locals.var_vfbcv_i_dn6, locals.var_vfbcv_i_dn7, locals.var_vfbcv_i_dn8, locals.var_vfbcv_i_dn9, locals.var_vfbcv_i_dn10, locals.var_vfbcv_i_dn11, locals.var_vfbcv_i_dn12, locals.var_vfbcv_i_dn13, locals.var_vfbcv_i_dn14, ) = (assign4240_e5483, ((locals.var_vfbcv_i_dn0 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn0 + locals.var_t1_dn0))), ((locals.var_vfbcv_i_dn2 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn2 + locals.var_t1_dn2))), ((locals.var_vfbcv_i_dn3 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn3 + locals.var_t1_dn3))), ((locals.var_vfbcv_i_dn4 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn4 + locals.var_t1_dn4))), ((locals.var_vfbcv_i_dn5 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn5 + locals.var_t1_dn5))), ((locals.var_vfbcv_i_dn6 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn6 + locals.var_t1_dn6))), ((locals.var_vfbcv_i_dn7 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn7 + locals.var_t1_dn7))), ((locals.var_vfbcv_i_dn8 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn8 + locals.var_t1_dn8))), ((locals.var_vfbcv_i_dn9 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn9 + locals.var_t1_dn9))), ((locals.var_vfbcv_i_dn10 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn10 + locals.var_t1_dn10))), ((locals.var_vfbcv_i_dn11 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn11 + locals.var_t1_dn11))), ((locals.var_vfbcv_i_dn12 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn12 + locals.var_t1_dn12))), ((locals.var_vfbcv_i_dn13 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn13 + locals.var_t1_dn13))), ((locals.var_vfbcv_i_dn14 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn14 + locals.var_t1_dn14))), );
        locals.var_vfbcv_i_rv = 0.0;

        let assign4250_e5487: f64 = (locals.var_inv_lact).powf(p.p264);
        let assign4250_e5490: f64 = (locals.var_inv_llong).powf(p.p264);
        let assign4250_e5491: f64 = (assign4250_e5487 - assign4250_e5490);
        let assign4250_e5493: f64 = (assign4250_e5491).max(0.0);
        let assign4250_e5494: f64 = (p.p263 * assign4250_e5493);
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign4250_e5494, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_t0_rv = 0.0;

        let assign4260_e5498: f64 = (locals.var_inv_w).powf(p.p266);
        let assign4260_e5501: f64 = (locals.var_inv_wwide).powf(p.p266);
        let assign4260_e5502: f64 = (assign4260_e5498 - assign4260_e5501);
        let assign4260_e5504: f64 = (assign4260_e5502).max(0.0);
        let assign4260_e5505: f64 = (p.p265 * assign4260_e5504);
        let assign4260_e5509: f64 = (locals.var_inv_wl).powf(p.p268);
        let assign4260_e5510: f64 = (p.p267 * assign4260_e5509);
        let assign4260_e5511: f64 = (assign4260_e5505 + assign4260_e5510);
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, ) = (assign4260_e5511, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_t1_rv = 0.0;

        let assign4270_e5515: f64 = (1.0 + locals.var_t0);
        let assign4270_e5517: f64 = (assign4270_e5515 + locals.var_t1);
        let assign4270_e5518: f64 = (locals.var_vsatcv_i * assign4270_e5517);
        (locals.var_vsatcv_i, locals.var_vsatcv_i_dn0, locals.var_vsatcv_i_dn2, locals.var_vsatcv_i_dn3, locals.var_vsatcv_i_dn4, locals.var_vsatcv_i_dn5, locals.var_vsatcv_i_dn6, locals.var_vsatcv_i_dn7, locals.var_vsatcv_i_dn8, locals.var_vsatcv_i_dn9, locals.var_vsatcv_i_dn10, locals.var_vsatcv_i_dn11, locals.var_vsatcv_i_dn12, locals.var_vsatcv_i_dn13, locals.var_vsatcv_i_dn14, ) = (assign4270_e5518, ((locals.var_vsatcv_i_dn0 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn0 + locals.var_t1_dn0))), ((locals.var_vsatcv_i_dn2 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn2 + locals.var_t1_dn2))), ((locals.var_vsatcv_i_dn3 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn3 + locals.var_t1_dn3))), ((locals.var_vsatcv_i_dn4 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn4 + locals.var_t1_dn4))), ((locals.var_vsatcv_i_dn5 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn5 + locals.var_t1_dn5))), ((locals.var_vsatcv_i_dn6 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn6 + locals.var_t1_dn6))), ((locals.var_vsatcv_i_dn7 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn7 + locals.var_t1_dn7))), ((locals.var_vsatcv_i_dn8 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn8 + locals.var_t1_dn8))), ((locals.var_vsatcv_i_dn9 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn9 + locals.var_t1_dn9))), ((locals.var_vsatcv_i_dn10 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn10 + locals.var_t1_dn10))), ((locals.var_vsatcv_i_dn11 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn11 + locals.var_t1_dn11))), ((locals.var_vsatcv_i_dn12 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn12 + locals.var_t1_dn12))), ((locals.var_vsatcv_i_dn13 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn13 + locals.var_t1_dn13))), ((locals.var_vsatcv_i_dn14 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn14 + locals.var_t1_dn14))), );
        locals.var_vsatcv_i_rv = 0.0;

        let assign4280_e5524: f64 = (locals.var_inv_lact).powf(p.p353);
        let assign4280_e5527: f64 = (locals.var_inv_llong).powf(p.p353);
        let assign4280_e5528: f64 = (assign4280_e5524 - assign4280_e5527);
        let assign4280_e5530: f64 = (assign4280_e5528).max(0.0);
        let assign4280_e5531: f64 = (p.p352 * assign4280_e5530);
        let assign4280_e5532: f64 = (1.0 + assign4280_e5531);
        let assign4280_e5533: f64 = (locals.var_pclmcv_i * assign4280_e5532);
        locals.var_pclmcv_i = assign4280_e5533;
        locals.var_pclmcv_i_rv = 0.0;

        let assign4290_e5536: f64 = (locals.var_pclmcv_i).max(0.0);
        locals.var_pclmcv_i = assign4290_e5536;
        locals.var_pclmcv_i_rv = 0.0;

        let assign4300_e5540: f64 = (locals.var_inv_l).powf(p.p187);
        let assign4300_e5543: f64 = (locals.var_inv_llong).powf(p.p187);
        let assign4300_e5544: f64 = (assign4300_e5540 - assign4300_e5543);
        let assign4300_e5546: f64 = (assign4300_e5544).max(0.0);
        let assign4300_e5547: f64 = (p.p186 * assign4300_e5546);
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign4300_e5547, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_t0_rv = 0.0;

        let assign4310_e5551: f64 = (locals.var_inv_w).powf(p.p189);
        let assign4310_e5554: f64 = (locals.var_inv_wwide).powf(p.p189);
        let assign4310_e5555: f64 = (assign4310_e5551 - assign4310_e5554);
        let assign4310_e5557: f64 = (assign4310_e5555).max(0.0);
        let assign4310_e5558: f64 = (p.p188 * assign4310_e5557);
        let assign4310_e5562: f64 = (locals.var_inv_wl).powf(p.p191);
        let assign4310_e5563: f64 = (p.p190 * assign4310_e5562);
        let assign4310_e5564: f64 = (assign4310_e5558 + assign4310_e5563);
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, ) = (assign4310_e5564, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_t1_rv = 0.0;

        let assign4320_e5568: f64 = (1.0 + locals.var_t0);
        let assign4320_e5570: f64 = (assign4320_e5568 + locals.var_t1);
        let assign4320_e5571: f64 = (locals.var_k1_i * assign4320_e5570);
        (locals.var_k1_i, locals.var_k1_i_dn0, locals.var_k1_i_dn2, locals.var_k1_i_dn3, locals.var_k1_i_dn4, locals.var_k1_i_dn5, locals.var_k1_i_dn6, locals.var_k1_i_dn7, locals.var_k1_i_dn8, locals.var_k1_i_dn9, locals.var_k1_i_dn10, locals.var_k1_i_dn11, locals.var_k1_i_dn12, locals.var_k1_i_dn13, locals.var_k1_i_dn14, ) = (assign4320_e5571, ((locals.var_k1_i_dn0 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn0 + locals.var_t1_dn0))), ((locals.var_k1_i_dn2 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn2 + locals.var_t1_dn2))), ((locals.var_k1_i_dn3 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn3 + locals.var_t1_dn3))), ((locals.var_k1_i_dn4 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn4 + locals.var_t1_dn4))), ((locals.var_k1_i_dn5 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn5 + locals.var_t1_dn5))), ((locals.var_k1_i_dn6 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn6 + locals.var_t1_dn6))), ((locals.var_k1_i_dn7 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn7 + locals.var_t1_dn7))), ((locals.var_k1_i_dn8 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn8 + locals.var_t1_dn8))), ((locals.var_k1_i_dn9 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn9 + locals.var_t1_dn9))), ((locals.var_k1_i_dn10 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn10 + locals.var_t1_dn10))), ((locals.var_k1_i_dn11 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn11 + locals.var_t1_dn11))), ((locals.var_k1_i_dn12 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn12 + locals.var_t1_dn12))), ((locals.var_k1_i_dn13 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn13 + locals.var_t1_dn13))), ((locals.var_k1_i_dn14 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn14 + locals.var_t1_dn14))), );
        locals.var_k1_i_rv = 0.0;

        let assign4330_e5575: f64 = (locals.var_inv_l).powf(p.p197);
        let assign4330_e5578: f64 = (locals.var_inv_llong).powf(p.p197);
        let assign4330_e5579: f64 = (assign4330_e5575 - assign4330_e5578);
        let assign4330_e5581: f64 = (assign4330_e5579).max(0.0);
        let assign4330_e5582: f64 = (p.p196 * assign4330_e5581);
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign4330_e5582, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_t0_rv = 0.0;

        let assign4340_e5586: f64 = (locals.var_inv_w).powf(p.p199);
        let assign4340_e5589: f64 = (locals.var_inv_wwide).powf(p.p199);
        let assign4340_e5590: f64 = (assign4340_e5586 - assign4340_e5589);
        let assign4340_e5592: f64 = (assign4340_e5590).max(0.0);
        let assign4340_e5593: f64 = (p.p198 * assign4340_e5592);
        let assign4340_e5597: f64 = (locals.var_inv_wl).powf(p.p201);
        let assign4340_e5598: f64 = (p.p200 * assign4340_e5597);
        let assign4340_e5599: f64 = (assign4340_e5593 + assign4340_e5598);
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, ) = (assign4340_e5599, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_t1_rv = 0.0;

        let assign4350_e5603: f64 = (1.0 + locals.var_t0);
        let assign4350_e5605: f64 = (assign4350_e5603 + locals.var_t1);
        let assign4350_e5606: f64 = (locals.var_k2_i * assign4350_e5605);
        (locals.var_k2_i, locals.var_k2_i_dn0, locals.var_k2_i_dn2, locals.var_k2_i_dn3, locals.var_k2_i_dn4, locals.var_k2_i_dn5, locals.var_k2_i_dn6, locals.var_k2_i_dn7, locals.var_k2_i_dn8, locals.var_k2_i_dn9, locals.var_k2_i_dn10, locals.var_k2_i_dn11, locals.var_k2_i_dn12, locals.var_k2_i_dn13, locals.var_k2_i_dn14, ) = (assign4350_e5606, ((locals.var_k2_i_dn0 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn0 + locals.var_t1_dn0))), ((locals.var_k2_i_dn2 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn2 + locals.var_t1_dn2))), ((locals.var_k2_i_dn3 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn3 + locals.var_t1_dn3))), ((locals.var_k2_i_dn4 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn4 + locals.var_t1_dn4))), ((locals.var_k2_i_dn5 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn5 + locals.var_t1_dn5))), ((locals.var_k2_i_dn6 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn6 + locals.var_t1_dn6))), ((locals.var_k2_i_dn7 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn7 + locals.var_t1_dn7))), ((locals.var_k2_i_dn8 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn8 + locals.var_t1_dn8))), ((locals.var_k2_i_dn9 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn9 + locals.var_t1_dn9))), ((locals.var_k2_i_dn10 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn10 + locals.var_t1_dn10))), ((locals.var_k2_i_dn11 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn11 + locals.var_t1_dn11))), ((locals.var_k2_i_dn12 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn12 + locals.var_t1_dn12))), ((locals.var_k2_i_dn13 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn13 + locals.var_t1_dn13))), ((locals.var_k2_i_dn14 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn14 + locals.var_t1_dn14))), );
        locals.var_k2_i_rv = 0.0;

        let assign4360_e5612: f64 = (locals.var_inv_l).powf(p.p384);
        let assign4360_e5615: f64 = (locals.var_inv_llong).powf(p.p384);
        let assign4360_e5616: f64 = (assign4360_e5612 - assign4360_e5615);
        let assign4360_e5618: f64 = (assign4360_e5616).max(0.0);
        let assign4360_e5619: f64 = (p.p383 * assign4360_e5618);
        let assign4360_e5620: f64 = (1.0 + assign4360_e5619);
        let assign4360_e5621: f64 = (locals.var_prwb_i * assign4360_e5620);
        locals.var_prwb_i = assign4360_e5621;
        locals.var_prwb_i_rv = 0.0;

        let assign4370_e5626: f64 = (locals.var_inv_l * p.p828);
        let assign4370_e5627: f64 = (1.0 + assign4370_e5626);
        let assign4370_e5628: f64 = (locals.var_ute_i * assign4370_e5627);
        locals.var_ute_i = assign4370_e5628;
        locals.var_ute_i_rv = 0.0;

        let assign4380_e5633: f64 = (locals.var_inv_l * p.p833);
        let assign4380_e5634: f64 = (1.0 + assign4380_e5633);
        let assign4380_e5635: f64 = (locals.var_ua1_i * assign4380_e5634);
        locals.var_ua1_i = assign4380_e5635;
        locals.var_ua1_i_rv = 0.0;

        let assign4390_e5640: f64 = (locals.var_inv_l * p.p842);
        let assign4390_e5641: f64 = (1.0 + assign4390_e5640);
        let assign4390_e5642: f64 = (locals.var_ud1_i * assign4390_e5641);
        locals.var_ud1_i = assign4390_e5642;
        locals.var_ud1_i_rv = 0.0;

        let assign4400_e5647: f64 = (locals.var_inv_l * p.p860);
        let assign4400_e5648: f64 = (1.0 + assign4400_e5647);
        let assign4400_e5649: f64 = (locals.var_at_i * assign4400_e5648);
        locals.var_at_i = assign4400_e5649;
        locals.var_at_i_rv = 0.0;

        let assign4410_e5654: f64 = (locals.var_inv_l * p.p866);
        let assign4410_e5655: f64 = (1.0 + assign4410_e5654);
        let assign4410_e5656: f64 = (locals.var_ptwgt_i * assign4410_e5655);
        locals.var_ptwgt_i = assign4410_e5656;
        locals.var_ptwgt_i_rv = 0.0;

        let assign4440_e5670: f64 = if p.p42 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard40 = assign4440_e5670;
        locals.var_guard40_rv = 0.0;

        if (locals.var_guard40 != 0.0) {
            let assign4450_e5677: f64 = (locals.var_inv_l).powf(p.p398);
            let assign4450_e5680: f64 = (locals.var_inv_llong).powf(p.p398);
            let assign4450_e5681: f64 = (assign4450_e5677 - assign4450_e5680);
            let assign4450_e5683: f64 = (assign4450_e5681).max(0.0);
            let assign4450_e5684: f64 = (p.p397 * assign4450_e5683);
            let assign4450_e5685: f64 = (1.0 + assign4450_e5684);
            let assign4450_e5686: f64 = (locals.var_rsw_i * assign4450_e5685);
            locals.var_rsw_i = assign4450_e5686;
            locals.var_rsw_i_rv = 0.0;
        }

        if (locals.var_guard40 != 0.0) {
            let assign4460_e5695: f64 = (locals.var_inv_l).powf(p.p408);
            let assign4460_e5698: f64 = (locals.var_inv_llong).powf(p.p408);
            let assign4460_e5699: f64 = (assign4460_e5695 - assign4460_e5698);
            let assign4460_e5701: f64 = (assign4460_e5699).max(0.0);
            let assign4460_e5702: f64 = (p.p407 * assign4460_e5701);
            let assign4460_e5703: f64 = (1.0 + assign4460_e5702);
            let assign4460_e5704: f64 = (locals.var_rdw_i * assign4460_e5703);
            locals.var_rdw_i = assign4460_e5704;
            locals.var_rdw_i_rv = 0.0;
        }

        if (locals.var_guard40 == 0.0) {
            let assign4470_e5714: f64 = (locals.var_inv_l).powf(p.p415);
            let assign4470_e5717: f64 = (locals.var_inv_llong).powf(p.p415);
            let assign4470_e5718: f64 = (assign4470_e5714 - assign4470_e5717);
            let assign4470_e5720: f64 = (assign4470_e5718).max(0.0);
            let assign4470_e5721: f64 = (p.p414 * assign4470_e5720);
            let assign4470_e5722: f64 = (1.0 + assign4470_e5721);
            let assign4470_e5723: f64 = (locals.var_rdsw_i * assign4470_e5722);
            locals.var_rdsw_i = assign4470_e5723;
            locals.var_rdsw_i_rv = 0.0;
        }

        let assign4480_e5728: f64 = if locals.var_ucs_i < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard41 = assign4480_e5728;
        locals.var_guard41_rv = 0.0;

        if (locals.var_guard41 != 0.0) {
            locals.var_ucs_i = 1.0;
            locals.var_ucs_i_rv = 0.0;
        }

        let assign4500_e5735: f64 = if locals.var_ucs_i > 2.0 { 1.0 } else { 0.0 };
        locals.var_guard42 = assign4500_e5735;
        locals.var_guard42_rv = 0.0;

        if ((locals.var_guard41 == 0.0) && (locals.var_guard42 != 0.0)) {
            locals.var_ucs_i = 2.0;
            locals.var_ucs_i_rv = 0.0;
        }

        let assign4520_e5745: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard43 = assign4520_e5745;
        locals.var_guard43_rv = 0.0;

        let assign4530_e5748: f64 = if locals.var_ucsr_i < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard44 = assign4530_e5748;
        locals.var_guard44_rv = 0.0;

        if ((locals.var_guard43 != 0.0) && (locals.var_guard44 != 0.0)) {
            locals.var_ucsr_i = 1.0;
            locals.var_ucsr_i_rv = 0.0;
        }

        let assign4550_e5757: f64 = if locals.var_ucsr_i > 2.0 { 1.0 } else { 0.0 };
        locals.var_guard45 = assign4550_e5757;
        locals.var_guard45_rv = 0.0;

        if (((locals.var_guard43 != 0.0) && (locals.var_guard44 == 0.0)) && (locals.var_guard45 != 0.0)) {
            locals.var_ucsr_i = 2.0;
            locals.var_ucsr_i_rv = 0.0;
        }

        let assign4800_e5840: f64 = if locals.var_m0_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard67 = assign4800_e5840;
        locals.var_guard67_rv = 0.0;

        if (locals.var_guard67 != 0.0) {
            locals.var_m0_i = 0.0;
            locals.var_m0_i_rv = 0.0;
        }

        let assign4820_e5847: f64 = if locals.var_u0_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard68 = assign4820_e5847;
        locals.var_guard68_rv = 0.0;

        if (locals.var_guard68 != 0.0) {
            locals.var_u0_i = 0.067;
            locals.var_u0_i_rv = 0.0;
        }

        let assign4840_e5854: f64 = if locals.var_ua_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard69 = assign4840_e5854;
        locals.var_guard69_rv = 0.0;

        if (locals.var_guard69 != 0.0) {
            (locals.var_ua_i, locals.var_ua_i_dn0, locals.var_ua_i_dn2, locals.var_ua_i_dn3, locals.var_ua_i_dn4, locals.var_ua_i_dn5, locals.var_ua_i_dn6, locals.var_ua_i_dn7, locals.var_ua_i_dn8, locals.var_ua_i_dn9, locals.var_ua_i_dn10, locals.var_ua_i_dn11, locals.var_ua_i_dn12, locals.var_ua_i_dn13, locals.var_ua_i_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_ua_i_rv = 0.0;
        }

        let assign4860_e5861: f64 = if locals.var_eu_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard70 = assign4860_e5861;
        locals.var_guard70_rv = 0.0;

        if (locals.var_guard70 != 0.0) {
            (locals.var_eu_i, locals.var_eu_i_dn0, locals.var_eu_i_dn2, locals.var_eu_i_dn3, locals.var_eu_i_dn4, locals.var_eu_i_dn5, locals.var_eu_i_dn6, locals.var_eu_i_dn7, locals.var_eu_i_dn8, locals.var_eu_i_dn9, locals.var_eu_i_dn10, locals.var_eu_i_dn11, locals.var_eu_i_dn12, locals.var_eu_i_dn13, locals.var_eu_i_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_eu_i_rv = 0.0;
        }

        let assign4880_e5868: f64 = if locals.var_ud_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard71 = assign4880_e5868;
        locals.var_guard71_rv = 0.0;

        if (locals.var_guard71 != 0.0) {
            (locals.var_ud_i, locals.var_ud_i_dn0, locals.var_ud_i_dn2, locals.var_ud_i_dn3, locals.var_ud_i_dn4, locals.var_ud_i_dn5, locals.var_ud_i_dn6, locals.var_ud_i_dn7, locals.var_ud_i_dn8, locals.var_ud_i_dn9, locals.var_ud_i_dn10, locals.var_ud_i_dn11, locals.var_ud_i_dn12, locals.var_ud_i_dn13, locals.var_ud_i_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_ud_i_rv = 0.0;
        }

        let assign4900_e5875: f64 = if locals.var_ucs_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard72 = assign4900_e5875;
        locals.var_guard72_rv = 0.0;

        if (locals.var_guard72 != 0.0) {
            locals.var_ucs_i = 0.0;
            locals.var_ucs_i_rv = 0.0;
        }

        let assign4920_e5882: f64 = if locals.var_beta1_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard73 = assign4920_e5882;
        locals.var_guard73_rv = 0.0;

        if (locals.var_guard73 != 0.0) {
            (locals.var_beta1_i, locals.var_beta1_i_dn0, locals.var_beta1_i_dn2, locals.var_beta1_i_dn3, locals.var_beta1_i_dn4, locals.var_beta1_i_dn5, locals.var_beta1_i_dn6, locals.var_beta1_i_dn7, locals.var_beta1_i_dn8, locals.var_beta1_i_dn9, locals.var_beta1_i_dn10, locals.var_beta1_i_dn11, locals.var_beta1_i_dn12, locals.var_beta1_i_dn13, locals.var_beta1_i_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_beta1_i_rv = 0.0;
        }

        let assign4940_e5889: f64 = if p.p1065 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard74 = assign4940_e5889;
        locals.var_guard74_rv = 0.0;

        if (locals.var_guard74 != 0.0) {
            locals.var_lh1 = p.p1066;
            locals.var_lh1_rv = 0.0;
        }

        let assign4960_e5896: f64 = if locals.var_leff > locals.var_lh1 { 1.0 } else { 0.0 };
        locals.var_guard75 = assign4960_e5896;
        locals.var_guard75_rv = 0.0;

        if ((locals.var_guard74 != 0.0) && (locals.var_guard75 != 0.0)) {
            let assign4970_e5902: f64 = (locals.var_leff - locals.var_lh1);
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign4970_e5902, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t0_rv = 0.0;
        }

        if ((locals.var_guard74 != 0.0) && (locals.var_guard75 == 0.0)) {
            locals.var_lh1 = locals.var_leff;
            locals.var_lh1_rv = 0.0;
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (locals.var_lh1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t0_rv = 0.0;
        }

        let assign5000_e5922: f64 = (locals.var_t0 / 2.0);
        let assign5000_e5923: f64 = if p.p801 >= assign5000_e5922 { 1.0 } else { 0.0 };
        locals.var_guard76 = assign5000_e5923;
        locals.var_guard76_rv = 0.0;

        if ((locals.var_guard74 != 0.0) && (locals.var_guard76 != 0.0)) {
            locals.var_lintnoi_i = 0.0;
            locals.var_lintnoi_i_rv = 0.0;
        }

        if ((locals.var_guard74 != 0.0) && (locals.var_guard76 == 0.0)) {
            locals.var_lintnoi_i = p.p801;
            locals.var_lintnoi_i_rv = 0.0;
        }

        locals.var_nuendd = 0.0;
        locals.var_nuendd_rv = 0.0;

        locals.var_nuends = 0.0;
        locals.var_nuends_rv = 0.0;

        locals.var_nuintd = 0.0;
        locals.var_nuintd_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_6(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        locals.var_nuints = 0.0;
        locals.var_nuints_rv = 0.0;

        locals.var_rend = 0.0;
        locals.var_rend_rv = 0.0;

        locals.var_rint = 0.0;
        locals.var_rint_rv = 0.0;

        let assign5090_e5945: f64 = (p.p695 - p.p698);
        locals.var_dmcgeff = assign5090_e5945;
        locals.var_dmcgeff_rv = 0.0;

        locals.var_dmcieff = p.p696;
        locals.var_dmcieff_rv = 0.0;

        let assign5110_e5949: f64 = (p.p697 - p.p698);
        locals.var_dmdgeff = assign5110_e5949;
        locals.var_dmdgeff_rv = 0.0;

        let assign5120_e5951: f64 = if param_given[3] { 1.0 } else { 0.0 };
        locals.var_guard77 = assign5120_e5951;
        locals.var_guard77_rv = 0.0;

        if (locals.var_guard77 != 0.0) {
            let assign5130_e5955: f64 = (p.p374 * p.p3);
            locals.var_rsourcegeo = assign5130_e5955;
            locals.var_rsourcegeo_rv = 0.0;
        }

        let assign5140_e5964: f64 = if ((p.p10 > 0.0) && (p.p374 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard78 = assign5140_e5964;
        locals.var_guard78_rv = 0.0;

        let assign5150_e5967: f64 = if p.p9 < 9.0 { 1.0 } else { 0.0 };
        locals.var_guard79 = assign5150_e5967;
        locals.var_guard79_rv = 0.0;

        let assign5160_e5970: f64 = (p.p2 % 2.0);
        let assign5160_e5972: f64 = if assign5160_e5970 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard80 = assign5160_e5972;
        locals.var_guard80_rv = 0.0;

        if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) {
            locals.var_nuendd = 1.0;
            locals.var_nuendd_rv = 0.0;
            locals.var_nuends = 1.0;
            locals.var_nuends_rv = 0.0;
        }

        if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) {
            let assign5190_e6006: f64 = (p.p2 - 1.0);
            let assign5190_e6008: f64 = (assign5190_e6006 / 2.0);
            let assign5190_e6010: f64 = (assign5190_e6008).max(0.0);
            let assign5190_e6011: f64 = (2.0 * assign5190_e6010);
            locals.var_nuintd = assign5190_e6011;
            locals.var_nuintd_rv = 0.0;
        }

        if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) {
            locals.var_nuints = locals.var_nuintd;
            locals.var_nuints_rv = 0.0;
        }

        let assign5210_e6027: f64 = if p.p6 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard81 = assign5210_e6027;
        locals.var_guard81_rv = 0.0;

        if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 == 0.0)) && (locals.var_guard81 != 0.0)) {
            locals.var_nuendd = 2.0;
            locals.var_nuendd_rv = 0.0;
        }

        if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 == 0.0)) && (locals.var_guard81 != 0.0)) {
            let assign5230_e6056: f64 = (p.p2 / 2.0);
            let assign5230_e6058: f64 = (assign5230_e6056 - 1.0);
            let assign5230_e6060: f64 = (assign5230_e6058).max(0.0);
            let assign5230_e6061: f64 = (2.0 * assign5230_e6060);
            locals.var_nuintd = assign5230_e6061;
            locals.var_nuintd_rv = 0.0;
        }

        if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 == 0.0)) && (locals.var_guard81 != 0.0)) {
            locals.var_nuends = 0.0;
            locals.var_nuends_rv = 0.0;
            locals.var_nuints = p.p2;
            locals.var_nuints_rv = 0.0;
        }

        if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 == 0.0)) && (locals.var_guard81 == 0.0)) {
            locals.var_nuendd = 0.0;
            locals.var_nuendd_rv = 0.0;
            locals.var_nuintd = p.p2;
            locals.var_nuintd_rv = 0.0;
            locals.var_nuends = 2.0;
            locals.var_nuends_rv = 0.0;
        }

        if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 == 0.0)) && (locals.var_guard81 == 0.0)) {
            let assign5290_e6152: f64 = (p.p2 / 2.0);
            let assign5290_e6154: f64 = (assign5290_e6152 - 1.0);
            let assign5290_e6156: f64 = (assign5290_e6154).max(0.0);
            let assign5290_e6157: f64 = (2.0 * assign5290_e6156);
            locals.var_nuints = assign5290_e6157;
            locals.var_nuints_rv = 0.0;
        }

        let assign5300_e6162: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard82 = assign5300_e6162;
        locals.var_guard82_rv = 0.0;

        let assign5310_e6165: f64 = if locals.var_nuints == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard83 = assign5310_e6165;
        locals.var_guard83_rv = 0.0;

        if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard82 != 0.0)) && (locals.var_guard83 != 0.0)) {
            locals.var_rint = 0.0;
            locals.var_rint_rv = 0.0;
        }

        if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard82 != 0.0)) && (locals.var_guard83 == 0.0)) {
            let assign5330_e6192: f64 = (p.p374 * locals.var_dmcgeff);
            let assign5330_e6195: f64 = (locals.var_weff * locals.var_nuints);
            let assign5330_e6196: f64 = (assign5330_e6192 / assign5330_e6195);
            locals.var_rint = assign5330_e6196;
            locals.var_rint_rv = 0.0;
        }

        let assign5340_e6201: f64 = if locals.var_nuintd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard84 = assign5340_e6201;
        locals.var_guard84_rv = 0.0;

        if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard82 == 0.0)) && (locals.var_guard84 != 0.0)) {
            locals.var_rint = 0.0;
            locals.var_rint_rv = 0.0;
        }

        if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard82 == 0.0)) && (locals.var_guard84 == 0.0)) {
            let assign5360_e6230: f64 = (p.p374 * locals.var_dmcgeff);
            let assign5360_e6233: f64 = (locals.var_weff * locals.var_nuintd);
            let assign5360_e6234: f64 = (assign5360_e6230 / assign5360_e6233);
            locals.var_rint = assign5360_e6234;
            locals.var_rint_rv = 0.0;
        }

        let assign5370_e6239: f64 = if p.p9 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard85 = assign5370_e6239;
        locals.var_guard85_rv = 0.0;

        let assign5380_e6242: f64 = if p.p9 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard86 = assign5380_e6242;
        locals.var_guard86_rv = 0.0;

        let assign5390_e6245: f64 = if p.p9 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard87 = assign5390_e6245;
        locals.var_guard87_rv = 0.0;

        let assign5400_e6248: f64 = if p.p9 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard88 = assign5400_e6248;
        locals.var_guard88_rv = 0.0;

        let assign5410_e6251: f64 = if p.p9 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard89 = assign5410_e6251;
        locals.var_guard89_rv = 0.0;

        let assign5420_e6254: f64 = if p.p9 == 5.0 { 1.0 } else { 0.0 };
        locals.var_guard90 = assign5420_e6254;
        locals.var_guard90_rv = 0.0;

        let assign5430_e6257: f64 = if p.p9 == 6.0 { 1.0 } else { 0.0 };
        locals.var_guard91 = assign5430_e6257;
        locals.var_guard91_rv = 0.0;

        let assign5440_e6260: f64 = if p.p9 == 7.0 { 1.0 } else { 0.0 };
        locals.var_guard92 = assign5440_e6260;
        locals.var_guard92_rv = 0.0;

        let assign5450_e6263: f64 = if p.p9 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard93 = assign5450_e6263;
        locals.var_guard93_rv = 0.0;

        let assign5460_e6266: f64 = if p.p9 == 9.0 { 1.0 } else { 0.0 };
        locals.var_guard94 = assign5460_e6266;
        locals.var_guard94_rv = 0.0;

        let assign5470_e6269: f64 = if p.p9 == 10.0 { 1.0 } else { 0.0 };
        locals.var_guard95 = assign5470_e6269;
        locals.var_guard95_rv = 0.0;

        let assign5480_e6272: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard96 = assign5480_e6272;
        locals.var_guard96_rv = 0.0;

        let assign5490_e6275: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard97 = assign5490_e6275;
        locals.var_guard97_rv = 0.0;

        let assign5500_e6286: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard98 = assign5500_e6286;
        locals.var_guard98_rv = 0.0;

        let assign5510_e6297: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard99 = assign5510_e6297;
        locals.var_guard99_rv = 0.0;

        let assign5520_e6300: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard100 = assign5520_e6300;
        locals.var_guard100_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && (locals.var_guard100 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && (locals.var_guard100 == 0.0)) {
            let assign5540_e6335: f64 = (p.p374 * locals.var_dmcgeff);
            let assign5540_e6338: f64 = (locals.var_weff * locals.var_nuends);
            let assign5540_e6339: f64 = (assign5540_e6335 / assign5540_e6338);
            locals.var_rend = assign5540_e6339;
            locals.var_rend_rv = 0.0;
        }

        let assign5560_e6352: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign5560_e6355: f64 = if ((locals.var_nuends == 0.0) || (assign5560_e6352 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard102 = assign5560_e6355;
        locals.var_guard102_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 != 0.0)) && ((locals.var_guard99 != 0.0) && (locals.var_guard98 == 0.0))) && (locals.var_guard102 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 != 0.0)) && ((locals.var_guard99 != 0.0) && (locals.var_guard98 == 0.0))) && (locals.var_guard102 == 0.0)) {
            let assign5580_e6396: f64 = (p.p374 * locals.var_weff);
            let assign5580_e6399: f64 = (3.0 * locals.var_nuends);
            let assign5580_e6402: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign5580_e6403: f64 = (assign5580_e6399 * assign5580_e6402);
            let assign5580_e6404: f64 = (assign5580_e6396 / assign5580_e6403);
            locals.var_rend = assign5580_e6404;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 != 0.0)) && (!((locals.var_guard98 != 0.0) || (locals.var_guard99 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign5600_e6435: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard103 = assign5600_e6435;
        locals.var_guard103_rv = 0.0;

        let assign5610_e6446: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard104 = assign5610_e6446;
        locals.var_guard104_rv = 0.0;

        let assign5620_e6449: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard105 = assign5620_e6449;
        locals.var_guard105_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard103 != 0.0)) && (locals.var_guard105 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard103 != 0.0)) && (locals.var_guard105 == 0.0)) {
            let assign5640_e6486: f64 = (p.p374 * locals.var_dmcgeff);
            let assign5640_e6489: f64 = (locals.var_weff * locals.var_nuends);
            let assign5640_e6490: f64 = (assign5640_e6486 / assign5640_e6489);
            locals.var_rend = assign5640_e6490;
            locals.var_rend_rv = 0.0;
        }

        let assign5660_e6503: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign5660_e6506: f64 = if ((locals.var_nuends == 0.0) || (assign5660_e6503 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard107 = assign5660_e6506;
        locals.var_guard107_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 == 0.0)) && ((locals.var_guard104 != 0.0) && (locals.var_guard103 == 0.0))) && (locals.var_guard107 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 == 0.0)) && ((locals.var_guard104 != 0.0) && (locals.var_guard103 == 0.0))) && (locals.var_guard107 == 0.0)) {
            let assign5680_e6549: f64 = (p.p374 * locals.var_weff);
            let assign5680_e6552: f64 = (3.0 * locals.var_nuends);
            let assign5680_e6555: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign5680_e6556: f64 = (assign5680_e6552 * assign5680_e6555);
            let assign5680_e6557: f64 = (assign5680_e6549 / assign5680_e6556);
            locals.var_rend = assign5680_e6557;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 == 0.0)) && (!((locals.var_guard103 != 0.0) || (locals.var_guard104 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign5700_e6581: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard108 = assign5700_e6581;
        locals.var_guard108_rv = 0.0;

        let assign5710_e6592: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard109 = assign5710_e6592;
        locals.var_guard109_rv = 0.0;

        let assign5720_e6603: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard110 = assign5720_e6603;
        locals.var_guard110_rv = 0.0;

        let assign5730_e6606: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard111 = assign5730_e6606;
        locals.var_guard111_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 == 0.0)) && (locals.var_guard108 != 0.0)) && (locals.var_guard109 != 0.0)) && (locals.var_guard111 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 == 0.0)) && (locals.var_guard108 != 0.0)) && (locals.var_guard109 != 0.0)) && (locals.var_guard111 == 0.0)) {
            let assign5750_e6643: f64 = (p.p374 * locals.var_dmcgeff);
            let assign5750_e6646: f64 = (locals.var_weff * locals.var_nuendd);
            let assign5750_e6647: f64 = (assign5750_e6643 / assign5750_e6646);
            locals.var_rend = assign5750_e6647;
            locals.var_rend_rv = 0.0;
        }

        let assign5770_e6660: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign5770_e6663: f64 = if ((locals.var_nuendd == 0.0) || (assign5770_e6660 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard113 = assign5770_e6663;
        locals.var_guard113_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 == 0.0)) && (locals.var_guard108 != 0.0)) && ((locals.var_guard110 != 0.0) && (locals.var_guard109 == 0.0))) && (locals.var_guard113 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 == 0.0)) && (locals.var_guard108 != 0.0)) && ((locals.var_guard110 != 0.0) && (locals.var_guard109 == 0.0))) && (locals.var_guard113 == 0.0)) {
            let assign5790_e6706: f64 = (p.p374 * locals.var_weff);
            let assign5790_e6709: f64 = (3.0 * locals.var_nuendd);
            let assign5790_e6712: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign5790_e6713: f64 = (assign5790_e6709 * assign5790_e6712);
            let assign5790_e6714: f64 = (assign5790_e6706 / assign5790_e6713);
            locals.var_rend = assign5790_e6714;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 == 0.0)) && (locals.var_guard108 != 0.0)) && (!((locals.var_guard109 != 0.0) || (locals.var_guard110 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign5810_e6746: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard114 = assign5810_e6746;
        locals.var_guard114_rv = 0.0;

        let assign5820_e6757: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard115 = assign5820_e6757;
        locals.var_guard115_rv = 0.0;

        let assign5830_e6760: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard116 = assign5830_e6760;
        locals.var_guard116_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 == 0.0)) && (locals.var_guard108 == 0.0)) && (locals.var_guard114 != 0.0)) && (locals.var_guard116 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 == 0.0)) && (locals.var_guard108 == 0.0)) && (locals.var_guard114 != 0.0)) && (locals.var_guard116 == 0.0)) {
            let assign5850_e6799: f64 = (p.p374 * locals.var_dmcgeff);
            let assign5850_e6802: f64 = (locals.var_weff * locals.var_nuendd);
            let assign5850_e6803: f64 = (assign5850_e6799 / assign5850_e6802);
            locals.var_rend = assign5850_e6803;
            locals.var_rend_rv = 0.0;
        }

        let assign5870_e6816: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign5870_e6819: f64 = if ((locals.var_nuendd == 0.0) || (assign5870_e6816 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard118 = assign5870_e6819;
        locals.var_guard118_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 == 0.0)) && (locals.var_guard108 == 0.0)) && ((locals.var_guard115 != 0.0) && (locals.var_guard114 == 0.0))) && (locals.var_guard118 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 == 0.0)) && (locals.var_guard108 == 0.0)) && ((locals.var_guard115 != 0.0) && (locals.var_guard114 == 0.0))) && (locals.var_guard118 == 0.0)) {
            let assign5890_e6864: f64 = (p.p374 * locals.var_weff);
            let assign5890_e6867: f64 = (3.0 * locals.var_nuendd);
            let assign5890_e6870: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign5890_e6871: f64 = (assign5890_e6867 * assign5890_e6870);
            let assign5890_e6872: f64 = (assign5890_e6864 / assign5890_e6871);
            locals.var_rend = assign5890_e6872;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 == 0.0)) && (locals.var_guard108 == 0.0)) && (!((locals.var_guard114 != 0.0) || (locals.var_guard115 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign5910_e6897: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard119 = assign5910_e6897;
        locals.var_guard119_rv = 0.0;

        let assign5920_e6900: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard120 = assign5920_e6900;
        locals.var_guard120_rv = 0.0;

        let assign5930_e6911: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard121 = assign5930_e6911;
        locals.var_guard121_rv = 0.0;

        let assign5940_e6922: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard122 = assign5940_e6922;
        locals.var_guard122_rv = 0.0;

        let assign5950_e6925: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard123 = assign5950_e6925;
        locals.var_guard123_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 != 0.0)) && (locals.var_guard120 != 0.0)) && (locals.var_guard121 != 0.0)) && (locals.var_guard123 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 != 0.0)) && (locals.var_guard120 != 0.0)) && (locals.var_guard121 != 0.0)) && (locals.var_guard123 == 0.0)) {
            let assign5970_e6966: f64 = (p.p374 * locals.var_dmcgeff);
            let assign5970_e6969: f64 = (locals.var_weff * locals.var_nuends);
            let assign5970_e6970: f64 = (assign5970_e6966 / assign5970_e6969);
            locals.var_rend = assign5970_e6970;
            locals.var_rend_rv = 0.0;
        }

        let assign5990_e6983: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign5990_e6986: f64 = if ((locals.var_nuends == 0.0) || (assign5990_e6983 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard125 = assign5990_e6986;
        locals.var_guard125_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 != 0.0)) && (locals.var_guard120 != 0.0)) && ((locals.var_guard122 != 0.0) && (locals.var_guard121 == 0.0))) && (locals.var_guard125 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 != 0.0)) && (locals.var_guard120 != 0.0)) && ((locals.var_guard122 != 0.0) && (locals.var_guard121 == 0.0))) && (locals.var_guard125 == 0.0)) {
            let assign6010_e7033: f64 = (p.p374 * locals.var_weff);
            let assign6010_e7036: f64 = (3.0 * locals.var_nuends);
            let assign6010_e7039: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign6010_e7040: f64 = (assign6010_e7036 * assign6010_e7039);
            let assign6010_e7041: f64 = (assign6010_e7033 / assign6010_e7040);
            locals.var_rend = assign6010_e7041;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 != 0.0)) && (locals.var_guard120 != 0.0)) && (!((locals.var_guard121 != 0.0) || (locals.var_guard122 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign6030_e7075: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard126 = assign6030_e7075;
        locals.var_guard126_rv = 0.0;

        let assign6040_e7086: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard127 = assign6040_e7086;
        locals.var_guard127_rv = 0.0;

        let assign6050_e7089: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard128 = assign6050_e7089;
        locals.var_guard128_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 != 0.0)) && (locals.var_guard120 == 0.0)) && (locals.var_guard126 != 0.0)) && (locals.var_guard128 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 != 0.0)) && (locals.var_guard120 == 0.0)) && (locals.var_guard126 != 0.0)) && (locals.var_guard128 == 0.0)) {
            let assign6070_e7132: f64 = (p.p374 * locals.var_dmcgeff);
            let assign6070_e7135: f64 = (locals.var_weff * locals.var_nuends);
            let assign6070_e7136: f64 = (assign6070_e7132 / assign6070_e7135);
            locals.var_rend = assign6070_e7136;
            locals.var_rend_rv = 0.0;
        }

        let assign6090_e7149: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign6090_e7152: f64 = if ((locals.var_nuends == 0.0) || (assign6090_e7149 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard130 = assign6090_e7152;
        locals.var_guard130_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 != 0.0)) && (locals.var_guard120 == 0.0)) && ((locals.var_guard127 != 0.0) && (locals.var_guard126 == 0.0))) && (locals.var_guard130 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_7(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 != 0.0)) && (locals.var_guard120 == 0.0)) && ((locals.var_guard127 != 0.0) && (locals.var_guard126 == 0.0))) && (locals.var_guard130 == 0.0)) {
            let assign6110_e7201: f64 = (p.p374 * locals.var_weff);
            let assign6110_e7204: f64 = (3.0 * locals.var_nuends);
            let assign6110_e7207: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign6110_e7208: f64 = (assign6110_e7204 * assign6110_e7207);
            let assign6110_e7209: f64 = (assign6110_e7201 / assign6110_e7208);
            locals.var_rend = assign6110_e7209;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 != 0.0)) && (locals.var_guard120 == 0.0)) && (!((locals.var_guard126 != 0.0) || (locals.var_guard127 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign6130_e7236: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard131 = assign6130_e7236;
        locals.var_guard131_rv = 0.0;

        let assign6140_e7247: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard132 = assign6140_e7247;
        locals.var_guard132_rv = 0.0;

        let assign6150_e7258: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard133 = assign6150_e7258;
        locals.var_guard133_rv = 0.0;

        let assign6160_e7261: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard134 = assign6160_e7261;
        locals.var_guard134_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 == 0.0)) && (locals.var_guard131 != 0.0)) && (locals.var_guard132 != 0.0)) && (locals.var_guard134 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 == 0.0)) && (locals.var_guard131 != 0.0)) && (locals.var_guard132 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6180_e7304: f64 = (p.p374 * locals.var_dmcgeff);
            let assign6180_e7307: f64 = (locals.var_weff * locals.var_nuendd);
            let assign6180_e7308: f64 = (assign6180_e7304 / assign6180_e7307);
            locals.var_rend = assign6180_e7308;
            locals.var_rend_rv = 0.0;
        }

        let assign6200_e7320: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard136 = assign6200_e7320;
        locals.var_guard136_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 == 0.0)) && (locals.var_guard131 != 0.0)) && ((locals.var_guard133 != 0.0) && (locals.var_guard132 == 0.0))) && (locals.var_guard136 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 == 0.0)) && (locals.var_guard131 != 0.0)) && ((locals.var_guard133 != 0.0) && (locals.var_guard132 == 0.0))) && (locals.var_guard136 == 0.0)) {
            let assign6220_e7369: f64 = (p.p374 * locals.var_weff);
            let assign6220_e7372: f64 = (6.0 * locals.var_nuendd);
            let assign6220_e7374: f64 = (assign6220_e7372 * locals.var_dmcgeff);
            let assign6220_e7375: f64 = (assign6220_e7369 / assign6220_e7374);
            locals.var_rend = assign6220_e7375;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 == 0.0)) && (locals.var_guard131 != 0.0)) && (!((locals.var_guard132 != 0.0) || (locals.var_guard133 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign6240_e7410: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard137 = assign6240_e7410;
        locals.var_guard137_rv = 0.0;

        let assign6250_e7421: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard138 = assign6250_e7421;
        locals.var_guard138_rv = 0.0;

        let assign6260_e7424: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard139 = assign6260_e7424;
        locals.var_guard139_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 == 0.0)) && (locals.var_guard131 == 0.0)) && (locals.var_guard137 != 0.0)) && (locals.var_guard139 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 == 0.0)) && (locals.var_guard131 == 0.0)) && (locals.var_guard137 != 0.0)) && (locals.var_guard139 == 0.0)) {
            let assign6280_e7469: f64 = (p.p374 * locals.var_dmcgeff);
            let assign6280_e7472: f64 = (locals.var_weff * locals.var_nuendd);
            let assign6280_e7473: f64 = (assign6280_e7469 / assign6280_e7472);
            locals.var_rend = assign6280_e7473;
            locals.var_rend_rv = 0.0;
        }

        let assign6300_e7485: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard141 = assign6300_e7485;
        locals.var_guard141_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 == 0.0)) && (locals.var_guard131 == 0.0)) && ((locals.var_guard138 != 0.0) && (locals.var_guard137 == 0.0))) && (locals.var_guard141 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 == 0.0)) && (locals.var_guard131 == 0.0)) && ((locals.var_guard138 != 0.0) && (locals.var_guard137 == 0.0))) && (locals.var_guard141 == 0.0)) {
            let assign6320_e7536: f64 = (p.p374 * locals.var_weff);
            let assign6320_e7539: f64 = (6.0 * locals.var_nuendd);
            let assign6320_e7541: f64 = (assign6320_e7539 * locals.var_dmcgeff);
            let assign6320_e7542: f64 = (assign6320_e7536 / assign6320_e7541);
            locals.var_rend = assign6320_e7542;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 == 0.0)) && (locals.var_guard131 == 0.0)) && (!((locals.var_guard137 != 0.0) || (locals.var_guard138 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign6340_e7570: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard142 = assign6340_e7570;
        locals.var_guard142_rv = 0.0;

        let assign6350_e7573: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard143 = assign6350_e7573;
        locals.var_guard143_rv = 0.0;

        let assign6360_e7584: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard144 = assign6360_e7584;
        locals.var_guard144_rv = 0.0;

        let assign6370_e7595: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard145 = assign6370_e7595;
        locals.var_guard145_rv = 0.0;

        let assign6380_e7598: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard146 = assign6380_e7598;
        locals.var_guard146_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 != 0.0)) && (locals.var_guard143 != 0.0)) && (locals.var_guard144 != 0.0)) && (locals.var_guard146 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 != 0.0)) && (locals.var_guard143 != 0.0)) && (locals.var_guard144 != 0.0)) && (locals.var_guard146 == 0.0)) {
            let assign6400_e7643: f64 = (p.p374 * locals.var_dmcgeff);
            let assign6400_e7646: f64 = (locals.var_weff * locals.var_nuends);
            let assign6400_e7647: f64 = (assign6400_e7643 / assign6400_e7646);
            locals.var_rend = assign6400_e7647;
            locals.var_rend_rv = 0.0;
        }

        let assign6420_e7659: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard148 = assign6420_e7659;
        locals.var_guard148_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 != 0.0)) && (locals.var_guard143 != 0.0)) && ((locals.var_guard145 != 0.0) && (locals.var_guard144 == 0.0))) && (locals.var_guard148 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 != 0.0)) && (locals.var_guard143 != 0.0)) && ((locals.var_guard145 != 0.0) && (locals.var_guard144 == 0.0))) && (locals.var_guard148 == 0.0)) {
            let assign6440_e7710: f64 = (p.p374 * locals.var_weff);
            let assign6440_e7713: f64 = (6.0 * locals.var_nuends);
            let assign6440_e7715: f64 = (assign6440_e7713 * locals.var_dmcgeff);
            let assign6440_e7716: f64 = (assign6440_e7710 / assign6440_e7715);
            locals.var_rend = assign6440_e7716;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 != 0.0)) && (locals.var_guard143 != 0.0)) && (!((locals.var_guard144 != 0.0) || (locals.var_guard145 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign6460_e7752: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard149 = assign6460_e7752;
        locals.var_guard149_rv = 0.0;

        let assign6470_e7763: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard150 = assign6470_e7763;
        locals.var_guard150_rv = 0.0;

        let assign6480_e7766: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard151 = assign6480_e7766;
        locals.var_guard151_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 != 0.0)) && (locals.var_guard143 == 0.0)) && (locals.var_guard149 != 0.0)) && (locals.var_guard151 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 != 0.0)) && (locals.var_guard143 == 0.0)) && (locals.var_guard149 != 0.0)) && (locals.var_guard151 == 0.0)) {
            let assign6500_e7813: f64 = (p.p374 * locals.var_dmcgeff);
            let assign6500_e7816: f64 = (locals.var_weff * locals.var_nuends);
            let assign6500_e7817: f64 = (assign6500_e7813 / assign6500_e7816);
            locals.var_rend = assign6500_e7817;
            locals.var_rend_rv = 0.0;
        }

        let assign6520_e7829: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard153 = assign6520_e7829;
        locals.var_guard153_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 != 0.0)) && (locals.var_guard143 == 0.0)) && ((locals.var_guard150 != 0.0) && (locals.var_guard149 == 0.0))) && (locals.var_guard153 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 != 0.0)) && (locals.var_guard143 == 0.0)) && ((locals.var_guard150 != 0.0) && (locals.var_guard149 == 0.0))) && (locals.var_guard153 == 0.0)) {
            let assign6540_e7882: f64 = (p.p374 * locals.var_weff);
            let assign6540_e7885: f64 = (6.0 * locals.var_nuends);
            let assign6540_e7887: f64 = (assign6540_e7885 * locals.var_dmcgeff);
            let assign6540_e7888: f64 = (assign6540_e7882 / assign6540_e7887);
            locals.var_rend = assign6540_e7888;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 != 0.0)) && (locals.var_guard143 == 0.0)) && (!((locals.var_guard149 != 0.0) || (locals.var_guard150 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign6560_e7917: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard154 = assign6560_e7917;
        locals.var_guard154_rv = 0.0;

        let assign6570_e7928: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard155 = assign6570_e7928;
        locals.var_guard155_rv = 0.0;

        let assign6580_e7939: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard156 = assign6580_e7939;
        locals.var_guard156_rv = 0.0;

        let assign6590_e7942: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard157 = assign6590_e7942;
        locals.var_guard157_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 == 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard155 != 0.0)) && (locals.var_guard157 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 == 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard155 != 0.0)) && (locals.var_guard157 == 0.0)) {
            let assign6610_e7989: f64 = (p.p374 * locals.var_dmcgeff);
            let assign6610_e7992: f64 = (locals.var_weff * locals.var_nuendd);
            let assign6610_e7993: f64 = (assign6610_e7989 / assign6610_e7992);
            locals.var_rend = assign6610_e7993;
            locals.var_rend_rv = 0.0;
        }

        let assign6630_e8006: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign6630_e8009: f64 = if ((locals.var_nuendd == 0.0) || (assign6630_e8006 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard159 = assign6630_e8009;
        locals.var_guard159_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 == 0.0)) && (locals.var_guard154 != 0.0)) && ((locals.var_guard156 != 0.0) && (locals.var_guard155 == 0.0))) && (locals.var_guard159 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 == 0.0)) && (locals.var_guard154 != 0.0)) && ((locals.var_guard156 != 0.0) && (locals.var_guard155 == 0.0))) && (locals.var_guard159 == 0.0)) {
            let assign6650_e8062: f64 = (p.p374 * locals.var_weff);
            let assign6650_e8065: f64 = (3.0 * locals.var_nuendd);
            let assign6650_e8068: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign6650_e8069: f64 = (assign6650_e8065 * assign6650_e8068);
            let assign6650_e8070: f64 = (assign6650_e8062 / assign6650_e8069);
            locals.var_rend = assign6650_e8070;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 == 0.0)) && (locals.var_guard154 != 0.0)) && (!((locals.var_guard155 != 0.0) || (locals.var_guard156 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign6670_e8107: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard160 = assign6670_e8107;
        locals.var_guard160_rv = 0.0;

        let assign6680_e8118: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard161 = assign6680_e8118;
        locals.var_guard161_rv = 0.0;

        let assign6690_e8121: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard162 = assign6690_e8121;
        locals.var_guard162_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 == 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard160 != 0.0)) && (locals.var_guard162 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 == 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard160 != 0.0)) && (locals.var_guard162 == 0.0)) {
            let assign6710_e8170: f64 = (p.p374 * locals.var_dmcgeff);
            let assign6710_e8173: f64 = (locals.var_weff * locals.var_nuendd);
            let assign6710_e8174: f64 = (assign6710_e8170 / assign6710_e8173);
            locals.var_rend = assign6710_e8174;
            locals.var_rend_rv = 0.0;
        }

        let assign6730_e8187: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign6730_e8190: f64 = if ((locals.var_nuendd == 0.0) || (assign6730_e8187 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard164 = assign6730_e8190;
        locals.var_guard164_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 == 0.0)) && (locals.var_guard154 == 0.0)) && ((locals.var_guard161 != 0.0) && (locals.var_guard160 == 0.0))) && (locals.var_guard164 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 == 0.0)) && (locals.var_guard154 == 0.0)) && ((locals.var_guard161 != 0.0) && (locals.var_guard160 == 0.0))) && (locals.var_guard164 == 0.0)) {
            let assign6750_e8245: f64 = (p.p374 * locals.var_weff);
            let assign6750_e8248: f64 = (3.0 * locals.var_nuendd);
            let assign6750_e8251: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign6750_e8252: f64 = (assign6750_e8248 * assign6750_e8251);
            let assign6750_e8253: f64 = (assign6750_e8245 / assign6750_e8252);
            locals.var_rend = assign6750_e8253;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 == 0.0)) && (locals.var_guard154 == 0.0)) && (!((locals.var_guard160 != 0.0) || (locals.var_guard161 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign6770_e8283: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard165 = assign6770_e8283;
        locals.var_guard165_rv = 0.0;

        let assign6780_e8286: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard166 = assign6780_e8286;
        locals.var_guard166_rv = 0.0;

        let assign6790_e8297: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard167 = assign6790_e8297;
        locals.var_guard167_rv = 0.0;

        let assign6800_e8308: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard168 = assign6800_e8308;
        locals.var_guard168_rv = 0.0;

        let assign6810_e8311: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard169 = assign6810_e8311;
        locals.var_guard169_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 != 0.0)) && (locals.var_guard167 != 0.0)) && (locals.var_guard169 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 != 0.0)) && (locals.var_guard167 != 0.0)) && (locals.var_guard169 == 0.0)) {
            let assign6830_e8360: f64 = (p.p374 * locals.var_dmcgeff);
            let assign6830_e8363: f64 = (locals.var_weff * locals.var_nuends);
            let assign6830_e8364: f64 = (assign6830_e8360 / assign6830_e8363);
            locals.var_rend = assign6830_e8364;
            locals.var_rend_rv = 0.0;
        }

        let assign6850_e8376: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard171 = assign6850_e8376;
        locals.var_guard171_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 != 0.0)) && ((locals.var_guard168 != 0.0) && (locals.var_guard167 == 0.0))) && (locals.var_guard171 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 != 0.0)) && ((locals.var_guard168 != 0.0) && (locals.var_guard167 == 0.0))) && (locals.var_guard171 == 0.0)) {
            let assign6870_e8431: f64 = (p.p374 * locals.var_weff);
            let assign6870_e8434: f64 = (6.0 * locals.var_nuends);
            let assign6870_e8436: f64 = (assign6870_e8434 * locals.var_dmcgeff);
            let assign6870_e8437: f64 = (assign6870_e8431 / assign6870_e8436);
            locals.var_rend = assign6870_e8437;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 != 0.0)) && (!((locals.var_guard167 != 0.0) || (locals.var_guard168 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign6890_e8475: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard172 = assign6890_e8475;
        locals.var_guard172_rv = 0.0;

        let assign6900_e8486: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard173 = assign6900_e8486;
        locals.var_guard173_rv = 0.0;

        let assign6910_e8489: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard174 = assign6910_e8489;
        locals.var_guard174_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard172 != 0.0)) && (locals.var_guard174 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard172 != 0.0)) && (locals.var_guard174 == 0.0)) {
            let assign6930_e8540: f64 = (p.p374 * locals.var_dmcgeff);
            let assign6930_e8543: f64 = (locals.var_weff * locals.var_nuends);
            let assign6930_e8544: f64 = (assign6930_e8540 / assign6930_e8543);
            locals.var_rend = assign6930_e8544;
            locals.var_rend_rv = 0.0;
        }

        let assign6950_e8556: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard176 = assign6950_e8556;
        locals.var_guard176_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 == 0.0)) && ((locals.var_guard173 != 0.0) && (locals.var_guard172 == 0.0))) && (locals.var_guard176 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 == 0.0)) && ((locals.var_guard173 != 0.0) && (locals.var_guard172 == 0.0))) && (locals.var_guard176 == 0.0)) {
            let assign6970_e8613: f64 = (p.p374 * locals.var_weff);
            let assign6970_e8616: f64 = (6.0 * locals.var_nuends);
            let assign6970_e8618: f64 = (assign6970_e8616 * locals.var_dmcgeff);
            let assign6970_e8619: f64 = (assign6970_e8613 / assign6970_e8618);
            locals.var_rend = assign6970_e8619;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 == 0.0)) && (!((locals.var_guard172 != 0.0) || (locals.var_guard173 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign6990_e8650: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard177 = assign6990_e8650;
        locals.var_guard177_rv = 0.0;

        let assign7000_e8661: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard178 = assign7000_e8661;
        locals.var_guard178_rv = 0.0;

        let assign7010_e8672: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard179 = assign7010_e8672;
        locals.var_guard179_rv = 0.0;

        let assign7020_e8675: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard180 = assign7020_e8675;
        locals.var_guard180_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 == 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard178 != 0.0)) && (locals.var_guard180 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 == 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard178 != 0.0)) && (locals.var_guard180 == 0.0)) {
            let assign7040_e8726: f64 = (p.p374 * locals.var_dmcgeff);
            let assign7040_e8729: f64 = (locals.var_weff * locals.var_nuendd);
            let assign7040_e8730: f64 = (assign7040_e8726 / assign7040_e8729);
            locals.var_rend = assign7040_e8730;
            locals.var_rend_rv = 0.0;
        }

        let assign7060_e8742: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard182 = assign7060_e8742;
        locals.var_guard182_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 == 0.0)) && (locals.var_guard177 != 0.0)) && ((locals.var_guard179 != 0.0) && (locals.var_guard178 == 0.0))) && (locals.var_guard182 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 == 0.0)) && (locals.var_guard177 != 0.0)) && ((locals.var_guard179 != 0.0) && (locals.var_guard178 == 0.0))) && (locals.var_guard182 == 0.0)) {
            let assign7080_e8799: f64 = (p.p374 * locals.var_weff);
            let assign7080_e8802: f64 = (6.0 * locals.var_nuendd);
            let assign7080_e8804: f64 = (assign7080_e8802 * locals.var_dmcgeff);
            let assign7080_e8805: f64 = (assign7080_e8799 / assign7080_e8804);
            locals.var_rend = assign7080_e8805;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 == 0.0)) && (locals.var_guard177 != 0.0)) && (!((locals.var_guard178 != 0.0) || (locals.var_guard179 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign7100_e8844: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard183 = assign7100_e8844;
        locals.var_guard183_rv = 0.0;

        let assign7110_e8855: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard184 = assign7110_e8855;
        locals.var_guard184_rv = 0.0;

        let assign7120_e8858: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard185 = assign7120_e8858;
        locals.var_guard185_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 == 0.0)) && (locals.var_guard177 == 0.0)) && (locals.var_guard183 != 0.0)) && (locals.var_guard185 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 == 0.0)) && (locals.var_guard177 == 0.0)) && (locals.var_guard183 != 0.0)) && (locals.var_guard185 == 0.0)) {
            let assign7140_e8911: f64 = (p.p374 * locals.var_dmcgeff);
            let assign7140_e8914: f64 = (locals.var_weff * locals.var_nuendd);
            let assign7140_e8915: f64 = (assign7140_e8911 / assign7140_e8914);
            locals.var_rend = assign7140_e8915;
            locals.var_rend_rv = 0.0;
        }

        let assign7160_e8927: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard187 = assign7160_e8927;
        locals.var_guard187_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_8(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 == 0.0)) && (locals.var_guard177 == 0.0)) && ((locals.var_guard184 != 0.0) && (locals.var_guard183 == 0.0))) && (locals.var_guard187 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 == 0.0)) && (locals.var_guard177 == 0.0)) && ((locals.var_guard184 != 0.0) && (locals.var_guard183 == 0.0))) && (locals.var_guard187 == 0.0)) {
            let assign7180_e8986: f64 = (p.p374 * locals.var_weff);
            let assign7180_e8989: f64 = (6.0 * locals.var_nuendd);
            let assign7180_e8991: f64 = (assign7180_e8989 * locals.var_dmcgeff);
            let assign7180_e8992: f64 = (assign7180_e8986 / assign7180_e8991);
            locals.var_rend = assign7180_e8992;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 == 0.0)) && (locals.var_guard177 == 0.0)) && (!((locals.var_guard183 != 0.0) || (locals.var_guard184 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign7200_e9024: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard188 = assign7200_e9024;
        locals.var_guard188_rv = 0.0;

        let assign7210_e9027: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard189 = assign7210_e9027;
        locals.var_guard189_rv = 0.0;

        let assign7220_e9038: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard190 = assign7220_e9038;
        locals.var_guard190_rv = 0.0;

        let assign7230_e9049: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard191 = assign7230_e9049;
        locals.var_guard191_rv = 0.0;

        let assign7240_e9052: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard192 = assign7240_e9052;
        locals.var_guard192_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 != 0.0)) && (locals.var_guard189 != 0.0)) && (locals.var_guard190 != 0.0)) && (locals.var_guard192 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 != 0.0)) && (locals.var_guard189 != 0.0)) && (locals.var_guard190 != 0.0)) && (locals.var_guard192 == 0.0)) {
            let assign7260_e9105: f64 = (p.p374 * locals.var_dmcgeff);
            let assign7260_e9108: f64 = (locals.var_weff * locals.var_nuends);
            let assign7260_e9109: f64 = (assign7260_e9105 / assign7260_e9108);
            locals.var_rend = assign7260_e9109;
            locals.var_rend_rv = 0.0;
        }

        let assign7280_e9122: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign7280_e9125: f64 = if ((locals.var_nuends == 0.0) || (assign7280_e9122 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard194 = assign7280_e9125;
        locals.var_guard194_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 != 0.0)) && (locals.var_guard189 != 0.0)) && ((locals.var_guard191 != 0.0) && (locals.var_guard190 == 0.0))) && (locals.var_guard194 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 != 0.0)) && (locals.var_guard189 != 0.0)) && ((locals.var_guard191 != 0.0) && (locals.var_guard190 == 0.0))) && (locals.var_guard194 == 0.0)) {
            let assign7300_e9184: f64 = (p.p374 * locals.var_weff);
            let assign7300_e9187: f64 = (3.0 * locals.var_nuends);
            let assign7300_e9190: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign7300_e9191: f64 = (assign7300_e9187 * assign7300_e9190);
            let assign7300_e9192: f64 = (assign7300_e9184 / assign7300_e9191);
            locals.var_rend = assign7300_e9192;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 != 0.0)) && (locals.var_guard189 != 0.0)) && (!((locals.var_guard190 != 0.0) || (locals.var_guard191 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign7320_e9232: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard195 = assign7320_e9232;
        locals.var_guard195_rv = 0.0;

        let assign7330_e9243: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard196 = assign7330_e9243;
        locals.var_guard196_rv = 0.0;

        let assign7340_e9246: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard197 = assign7340_e9246;
        locals.var_guard197_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 != 0.0)) && (locals.var_guard189 == 0.0)) && (locals.var_guard195 != 0.0)) && (locals.var_guard197 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 != 0.0)) && (locals.var_guard189 == 0.0)) && (locals.var_guard195 != 0.0)) && (locals.var_guard197 == 0.0)) {
            let assign7360_e9301: f64 = (p.p374 * locals.var_dmcgeff);
            let assign7360_e9304: f64 = (locals.var_weff * locals.var_nuends);
            let assign7360_e9305: f64 = (assign7360_e9301 / assign7360_e9304);
            locals.var_rend = assign7360_e9305;
            locals.var_rend_rv = 0.0;
        }

        let assign7380_e9318: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign7380_e9321: f64 = if ((locals.var_nuends == 0.0) || (assign7380_e9318 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard199 = assign7380_e9321;
        locals.var_guard199_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 != 0.0)) && (locals.var_guard189 == 0.0)) && ((locals.var_guard196 != 0.0) && (locals.var_guard195 == 0.0))) && (locals.var_guard199 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 != 0.0)) && (locals.var_guard189 == 0.0)) && ((locals.var_guard196 != 0.0) && (locals.var_guard195 == 0.0))) && (locals.var_guard199 == 0.0)) {
            let assign7400_e9382: f64 = (p.p374 * locals.var_weff);
            let assign7400_e9385: f64 = (3.0 * locals.var_nuends);
            let assign7400_e9388: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign7400_e9389: f64 = (assign7400_e9385 * assign7400_e9388);
            let assign7400_e9390: f64 = (assign7400_e9382 / assign7400_e9389);
            locals.var_rend = assign7400_e9390;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 != 0.0)) && (locals.var_guard189 == 0.0)) && (!((locals.var_guard195 != 0.0) || (locals.var_guard196 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 == 0.0)) {
            let assign7420_e9441: f64 = (p.p374 * locals.var_dmdgeff);
            let assign7420_e9443: f64 = (assign7420_e9441 / locals.var_weff);
            locals.var_rend = assign7420_e9443;
            locals.var_rend_rv = 0.0;
        }

        let assign7430_e9448: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard200 = assign7430_e9448;
        locals.var_guard200_rv = 0.0;

        let assign7440_e9451: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard201 = assign7440_e9451;
        locals.var_guard201_rv = 0.0;

        let assign7450_e9462: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard202 = assign7450_e9462;
        locals.var_guard202_rv = 0.0;

        let assign7460_e9473: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard203 = assign7460_e9473;
        locals.var_guard203_rv = 0.0;

        let assign7470_e9476: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard204 = assign7470_e9476;
        locals.var_guard204_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 != 0.0)) && (locals.var_guard202 != 0.0)) && (locals.var_guard204 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 != 0.0)) && (locals.var_guard202 != 0.0)) && (locals.var_guard204 == 0.0)) {
            let assign7490_e9533: f64 = (p.p374 * locals.var_dmcgeff);
            let assign7490_e9536: f64 = (locals.var_weff * locals.var_nuends);
            let assign7490_e9537: f64 = (assign7490_e9533 / assign7490_e9536);
            locals.var_rend = assign7490_e9537;
            locals.var_rend_rv = 0.0;
        }

        let assign7510_e9549: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard206 = assign7510_e9549;
        locals.var_guard206_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 != 0.0)) && ((locals.var_guard203 != 0.0) && (locals.var_guard202 == 0.0))) && (locals.var_guard206 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 != 0.0)) && ((locals.var_guard203 != 0.0) && (locals.var_guard202 == 0.0))) && (locals.var_guard206 == 0.0)) {
            let assign7530_e9612: f64 = (p.p374 * locals.var_weff);
            let assign7530_e9615: f64 = (6.0 * locals.var_nuends);
            let assign7530_e9617: f64 = (assign7530_e9615 * locals.var_dmcgeff);
            let assign7530_e9618: f64 = (assign7530_e9612 / assign7530_e9617);
            locals.var_rend = assign7530_e9618;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 != 0.0)) && (!((locals.var_guard202 != 0.0) || (locals.var_guard203 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign7550_e9660: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard207 = assign7550_e9660;
        locals.var_guard207_rv = 0.0;

        let assign7560_e9671: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard208 = assign7560_e9671;
        locals.var_guard208_rv = 0.0;

        let assign7570_e9674: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard209 = assign7570_e9674;
        locals.var_guard209_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 == 0.0)) && (locals.var_guard207 != 0.0)) && (locals.var_guard209 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 == 0.0)) && (locals.var_guard207 != 0.0)) && (locals.var_guard209 == 0.0)) {
            let assign7590_e9733: f64 = (p.p374 * locals.var_dmcgeff);
            let assign7590_e9736: f64 = (locals.var_weff * locals.var_nuends);
            let assign7590_e9737: f64 = (assign7590_e9733 / assign7590_e9736);
            locals.var_rend = assign7590_e9737;
            locals.var_rend_rv = 0.0;
        }

        let assign7610_e9749: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard211 = assign7610_e9749;
        locals.var_guard211_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 == 0.0)) && ((locals.var_guard208 != 0.0) && (locals.var_guard207 == 0.0))) && (locals.var_guard211 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 == 0.0)) && ((locals.var_guard208 != 0.0) && (locals.var_guard207 == 0.0))) && (locals.var_guard211 == 0.0)) {
            let assign7630_e9814: f64 = (p.p374 * locals.var_weff);
            let assign7630_e9817: f64 = (6.0 * locals.var_nuends);
            let assign7630_e9819: f64 = (assign7630_e9817 * locals.var_dmcgeff);
            let assign7630_e9820: f64 = (assign7630_e9814 / assign7630_e9819);
            locals.var_rend = assign7630_e9820;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 == 0.0)) && (!((locals.var_guard207 != 0.0) || (locals.var_guard208 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign7650_e9855: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard212 = assign7650_e9855;
        locals.var_guard212_rv = 0.0;

        if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 == 0.0)) && (locals.var_guard212 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 == 0.0)) && (locals.var_guard212 == 0.0)) {
            let assign7670_e9906: f64 = (p.p374 * locals.var_dmdgeff);
            let assign7670_e9909: f64 = (locals.var_weff * locals.var_nuendd);
            let assign7670_e9910: f64 = (assign7670_e9906 / assign7670_e9909);
            locals.var_rend = assign7670_e9910;
            locals.var_rend_rv = 0.0;
        }

        let assign7680_e9915: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard213 = assign7680_e9915;
        locals.var_guard213_rv = 0.0;

        if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 != 0.0)) {
            let assign7690_e9939: f64 = (p.p374 * locals.var_dmdgeff);
            let assign7690_e9941: f64 = (assign7690_e9939 / locals.var_weff);
            locals.var_rend = assign7690_e9941;
            locals.var_rend_rv = 0.0;
        }

        let assign7700_e9946: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard214 = assign7700_e9946;
        locals.var_guard214_rv = 0.0;

        let assign7710_e9957: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard215 = assign7710_e9957;
        locals.var_guard215_rv = 0.0;

        let assign7720_e9968: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard216 = assign7720_e9968;
        locals.var_guard216_rv = 0.0;

        let assign7730_e9971: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard217 = assign7730_e9971;
        locals.var_guard217_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 != 0.0)) && (locals.var_guard215 != 0.0)) && (locals.var_guard217 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 != 0.0)) && (locals.var_guard215 != 0.0)) && (locals.var_guard217 == 0.0)) {
            let assign7750_e10034: f64 = (p.p374 * locals.var_dmcgeff);
            let assign7750_e10037: f64 = (locals.var_weff * locals.var_nuendd);
            let assign7750_e10038: f64 = (assign7750_e10034 / assign7750_e10037);
            locals.var_rend = assign7750_e10038;
            locals.var_rend_rv = 0.0;
        }

        let assign7770_e10051: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign7770_e10054: f64 = if ((locals.var_nuendd == 0.0) || (assign7770_e10051 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard219 = assign7770_e10054;
        locals.var_guard219_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 != 0.0)) && ((locals.var_guard216 != 0.0) && (locals.var_guard215 == 0.0))) && (locals.var_guard219 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 != 0.0)) && ((locals.var_guard216 != 0.0) && (locals.var_guard215 == 0.0))) && (locals.var_guard219 == 0.0)) {
            let assign7790_e10123: f64 = (p.p374 * locals.var_weff);
            let assign7790_e10126: f64 = (3.0 * locals.var_nuendd);
            let assign7790_e10129: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign7790_e10130: f64 = (assign7790_e10126 * assign7790_e10129);
            let assign7790_e10131: f64 = (assign7790_e10123 / assign7790_e10130);
            locals.var_rend = assign7790_e10131;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 != 0.0)) && (!((locals.var_guard215 != 0.0) || (locals.var_guard216 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign7810_e10176: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard220 = assign7810_e10176;
        locals.var_guard220_rv = 0.0;

        let assign7820_e10187: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard221 = assign7820_e10187;
        locals.var_guard221_rv = 0.0;

        let assign7830_e10190: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard222 = assign7830_e10190;
        locals.var_guard222_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 == 0.0)) && (locals.var_guard220 != 0.0)) && (locals.var_guard222 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 == 0.0)) && (locals.var_guard220 != 0.0)) && (locals.var_guard222 == 0.0)) {
            let assign7850_e10255: f64 = (p.p374 * locals.var_dmcgeff);
            let assign7850_e10258: f64 = (locals.var_weff * locals.var_nuendd);
            let assign7850_e10259: f64 = (assign7850_e10255 / assign7850_e10258);
            locals.var_rend = assign7850_e10259;
            locals.var_rend_rv = 0.0;
        }

        let assign7870_e10272: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign7870_e10275: f64 = if ((locals.var_nuendd == 0.0) || (assign7870_e10272 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard224 = assign7870_e10275;
        locals.var_guard224_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 == 0.0)) && ((locals.var_guard221 != 0.0) && (locals.var_guard220 == 0.0))) && (locals.var_guard224 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 == 0.0)) && ((locals.var_guard221 != 0.0) && (locals.var_guard220 == 0.0))) && (locals.var_guard224 == 0.0)) {
            let assign7890_e10346: f64 = (p.p374 * locals.var_weff);
            let assign7890_e10349: f64 = (3.0 * locals.var_nuendd);
            let assign7890_e10352: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign7890_e10353: f64 = (assign7890_e10349 * assign7890_e10352);
            let assign7890_e10354: f64 = (assign7890_e10346 / assign7890_e10353);
            locals.var_rend = assign7890_e10354;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 == 0.0)) && (!((locals.var_guard220 != 0.0) || (locals.var_guard221 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign7910_e10392: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard225 = assign7910_e10392;
        locals.var_guard225_rv = 0.0;

        let assign7920_e10395: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard226 = assign7920_e10395;
        locals.var_guard226_rv = 0.0;

        if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 != 0.0)) && (locals.var_guard226 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 != 0.0)) && (locals.var_guard226 == 0.0)) {
            let assign7940_e10452: f64 = (p.p374 * locals.var_dmdgeff);
            let assign7940_e10455: f64 = (locals.var_weff * locals.var_nuends);
            let assign7940_e10456: f64 = (assign7940_e10452 / assign7940_e10455);
            locals.var_rend = assign7940_e10456;
            locals.var_rend_rv = 0.0;
        }

        let assign7950_e10461: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard227 = assign7950_e10461;
        locals.var_guard227_rv = 0.0;

        let assign7960_e10472: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard228 = assign7960_e10472;
        locals.var_guard228_rv = 0.0;

        let assign7970_e10483: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard229 = assign7970_e10483;
        locals.var_guard229_rv = 0.0;

        let assign7980_e10486: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard230 = assign7980_e10486;
        locals.var_guard230_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 == 0.0)) && (locals.var_guard227 != 0.0)) && (locals.var_guard228 != 0.0)) && (locals.var_guard230 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 == 0.0)) && (locals.var_guard227 != 0.0)) && (locals.var_guard228 != 0.0)) && (locals.var_guard230 == 0.0)) {
            let assign8000_e10553: f64 = (p.p374 * locals.var_dmcgeff);
            let assign8000_e10556: f64 = (locals.var_weff * locals.var_nuendd);
            let assign8000_e10557: f64 = (assign8000_e10553 / assign8000_e10556);
            locals.var_rend = assign8000_e10557;
            locals.var_rend_rv = 0.0;
        }

        let assign8020_e10569: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard232 = assign8020_e10569;
        locals.var_guard232_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 == 0.0)) && (locals.var_guard227 != 0.0)) && ((locals.var_guard229 != 0.0) && (locals.var_guard228 == 0.0))) && (locals.var_guard232 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 == 0.0)) && (locals.var_guard227 != 0.0)) && ((locals.var_guard229 != 0.0) && (locals.var_guard228 == 0.0))) && (locals.var_guard232 == 0.0)) {
            let assign8040_e10642: f64 = (p.p374 * locals.var_weff);
            let assign8040_e10645: f64 = (6.0 * locals.var_nuendd);
            let assign8040_e10647: f64 = (assign8040_e10645 * locals.var_dmcgeff);
            let assign8040_e10648: f64 = (assign8040_e10642 / assign8040_e10647);
            locals.var_rend = assign8040_e10648;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 == 0.0)) && (locals.var_guard227 != 0.0)) && (!((locals.var_guard228 != 0.0) || (locals.var_guard229 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign8060_e10695: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard233 = assign8060_e10695;
        locals.var_guard233_rv = 0.0;

        let assign8070_e10706: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard234 = assign8070_e10706;
        locals.var_guard234_rv = 0.0;

        let assign8080_e10709: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard235 = assign8080_e10709;
        locals.var_guard235_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 == 0.0)) && (locals.var_guard227 == 0.0)) && (locals.var_guard233 != 0.0)) && (locals.var_guard235 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 == 0.0)) && (locals.var_guard227 == 0.0)) && (locals.var_guard233 != 0.0)) && (locals.var_guard235 == 0.0)) {
            let assign8100_e10778: f64 = (p.p374 * locals.var_dmcgeff);
            let assign8100_e10781: f64 = (locals.var_weff * locals.var_nuendd);
            let assign8100_e10782: f64 = (assign8100_e10778 / assign8100_e10781);
            locals.var_rend = assign8100_e10782;
            locals.var_rend_rv = 0.0;
        }

        let assign8120_e10794: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard237 = assign8120_e10794;
        locals.var_guard237_rv = 0.0;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 == 0.0)) && (locals.var_guard227 == 0.0)) && ((locals.var_guard234 != 0.0) && (locals.var_guard233 == 0.0))) && (locals.var_guard237 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 == 0.0)) && (locals.var_guard227 == 0.0)) && ((locals.var_guard234 != 0.0) && (locals.var_guard233 == 0.0))) && (locals.var_guard237 == 0.0)) {
            let assign8140_e10869: f64 = (p.p374 * locals.var_weff);
            let assign8140_e10872: f64 = (6.0 * locals.var_nuendd);
            let assign8140_e10874: f64 = (assign8140_e10872 * locals.var_dmcgeff);
            let assign8140_e10875: f64 = (assign8140_e10869 / assign8140_e10874);
            locals.var_rend = assign8140_e10875;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 == 0.0)) && (locals.var_guard227 == 0.0)) && (!((locals.var_guard233 != 0.0) || (locals.var_guard234 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard93 != 0.0) && (!((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0))))) {
            let assign8160_e10938: f64 = (p.p374 * locals.var_dmdgeff);
            let assign8160_e10940: f64 = (assign8160_e10938 / locals.var_weff);
            locals.var_rend = assign8160_e10940;
            locals.var_rend_rv = 0.0;
        }

        let assign8170_e10945: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard238 = assign8170_e10945;
        locals.var_guard238_rv = 0.0;

        if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard94 != 0.0) && (!(((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0))))) && (locals.var_guard238 != 0.0)) {
            let assign8180_e10975: f64 = (0.5 * p.p374);
            let assign8180_e10977: f64 = (assign8180_e10975 * locals.var_dmcgeff);
            let assign8180_e10979: f64 = (assign8180_e10977 / locals.var_weff);
            locals.var_rend = assign8180_e10979;
            locals.var_rend_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_9(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let assign8190_e10984: f64 = if p.p2 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard239 = assign8190_e10984;
        locals.var_guard239_rv = 0.0;

        if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard94 != 0.0) && (!(((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0))))) && (locals.var_guard238 != 0.0)) && (locals.var_guard239 != 0.0)) {
            locals.var_rint = 0.0;
            locals.var_rint_rv = 0.0;
        }

        if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard94 != 0.0) && (!(((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0))))) && (locals.var_guard238 != 0.0)) && (locals.var_guard239 == 0.0)) {
            let assign8210_e11049: f64 = (p.p374 * locals.var_dmcgeff);
            let assign8210_e11053: f64 = (p.p2 - 2.0);
            let assign8210_e11054: f64 = (locals.var_weff * assign8210_e11053);
            let assign8210_e11055: f64 = (assign8210_e11049 / assign8210_e11054);
            locals.var_rint = assign8210_e11055;
            locals.var_rint_rv = 0.0;
        }

        if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard94 != 0.0) && (!(((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0))))) && (locals.var_guard238 == 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard94 != 0.0) && (!(((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0))))) && (locals.var_guard238 == 0.0)) {
            let assign8230_e11119: f64 = (p.p374 * locals.var_dmcgeff);
            let assign8230_e11122: f64 = (locals.var_weff * p.p2);
            let assign8230_e11123: f64 = (assign8230_e11119 / assign8230_e11122);
            locals.var_rint = assign8230_e11123;
            locals.var_rint_rv = 0.0;
        }

        let assign8240_e11128: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard240 = assign8240_e11128;
        locals.var_guard240_rv = 0.0;

        if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard95 != 0.0) && (!((((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0))))) && (locals.var_guard240 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard95 != 0.0) && (!((((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0))))) && (locals.var_guard240 != 0.0)) {
            let assign8260_e11192: f64 = (p.p374 * locals.var_dmcgeff);
            let assign8260_e11195: f64 = (locals.var_weff * p.p2);
            let assign8260_e11196: f64 = (assign8260_e11192 / assign8260_e11195);
            locals.var_rint = assign8260_e11196;
            locals.var_rint_rv = 0.0;
        }

        if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard95 != 0.0) && (!((((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0))))) && (locals.var_guard240 == 0.0)) {
            let assign8270_e11231: f64 = (0.5 * p.p374);
            let assign8270_e11233: f64 = (assign8270_e11231 * locals.var_dmcgeff);
            let assign8270_e11235: f64 = (assign8270_e11233 / locals.var_weff);
            locals.var_rend = assign8270_e11235;
            locals.var_rend_rv = 0.0;
        }

        let assign8280_e11240: f64 = if p.p2 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard241 = assign8280_e11240;
        locals.var_guard241_rv = 0.0;

        if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard95 != 0.0) && (!((((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0))))) && (locals.var_guard240 == 0.0)) && (locals.var_guard241 != 0.0)) {
            locals.var_rint = 0.0;
            locals.var_rint_rv = 0.0;
        }

        if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard95 != 0.0) && (!((((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0))))) && (locals.var_guard240 == 0.0)) && (locals.var_guard241 == 0.0)) {
            let assign8300_e11311: f64 = (p.p374 * locals.var_dmcgeff);
            let assign8300_e11315: f64 = (p.p2 - 2.0);
            let assign8300_e11316: f64 = (locals.var_weff * assign8300_e11315);
            let assign8300_e11317: f64 = (assign8300_e11311 / assign8300_e11316);
            locals.var_rint = assign8300_e11317;
            locals.var_rint_rv = 0.0;
        }

        if (((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (!(((((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0)) || (locals.var_guard95 != 0.0)))) {
            locals.var_rint = 0.0;
            locals.var_rint_rv = 0.0;
        }

        let assign8320_e11352: f64 = if locals.var_rint <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard242 = assign8320_e11352;
        locals.var_guard242_rv = 0.0;

        if (((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard242 != 0.0)) {
            locals.var_rsourcegeo = locals.var_rend;
            locals.var_rsourcegeo_rv = 0.0;
        }

        let assign8340_e11364: f64 = if locals.var_rend <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard243 = assign8340_e11364;
        locals.var_guard243_rv = 0.0;

        if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard242 == 0.0)) && (locals.var_guard243 != 0.0)) {
            locals.var_rsourcegeo = locals.var_rint;
            locals.var_rsourcegeo_rv = 0.0;
        }

        if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard242 == 0.0)) && (locals.var_guard243 == 0.0)) {
            let assign8360_e11389: f64 = (locals.var_rint * locals.var_rend);
            let assign8360_e11392: f64 = (locals.var_rint + locals.var_rend);
            let assign8360_e11393: f64 = (assign8360_e11389 / assign8360_e11392);
            locals.var_rsourcegeo = assign8360_e11393;
            locals.var_rsourcegeo_rv = 0.0;
        }

        if ((locals.var_guard77 == 0.0) && (locals.var_guard78 == 0.0)) {
            locals.var_rsourcegeo = 0.0;
            locals.var_rsourcegeo_rv = 0.0;
        }

        let assign8390_e11408: f64 = if param_given[4] { 1.0 } else { 0.0 };
        locals.var_guard245 = assign8390_e11408;
        locals.var_guard245_rv = 0.0;

        if (locals.var_guard245 != 0.0) {
            let assign8400_e11412: f64 = (p.p374 * p.p4);
            locals.var_rdraingeo = assign8400_e11412;
            locals.var_rdraingeo_rv = 0.0;
        }

        let assign8410_e11421: f64 = if ((p.p10 > 0.0) && (p.p374 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard246 = assign8410_e11421;
        locals.var_guard246_rv = 0.0;

        let assign8420_e11424: f64 = if p.p9 < 9.0 { 1.0 } else { 0.0 };
        locals.var_guard247 = assign8420_e11424;
        locals.var_guard247_rv = 0.0;

        let assign8430_e11427: f64 = (p.p2 % 2.0);
        let assign8430_e11429: f64 = if assign8430_e11427 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard248 = assign8430_e11429;
        locals.var_guard248_rv = 0.0;

        if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) {
            locals.var_nuendd = 1.0;
            locals.var_nuendd_rv = 0.0;
            locals.var_nuends = 1.0;
            locals.var_nuends_rv = 0.0;
        }

        if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) {
            let assign8460_e11463: f64 = (p.p2 - 1.0);
            let assign8460_e11465: f64 = (assign8460_e11463 / 2.0);
            let assign8460_e11467: f64 = (assign8460_e11465).max(0.0);
            let assign8460_e11468: f64 = (2.0 * assign8460_e11467);
            locals.var_nuintd = assign8460_e11468;
            locals.var_nuintd_rv = 0.0;
        }

        if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) {
            locals.var_nuints = locals.var_nuintd;
            locals.var_nuints_rv = 0.0;
        }

        let assign8480_e11484: f64 = if p.p6 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard249 = assign8480_e11484;
        locals.var_guard249_rv = 0.0;

        if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 == 0.0)) && (locals.var_guard249 != 0.0)) {
            locals.var_nuendd = 2.0;
            locals.var_nuendd_rv = 0.0;
        }

        if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 == 0.0)) && (locals.var_guard249 != 0.0)) {
            let assign8500_e11513: f64 = (p.p2 / 2.0);
            let assign8500_e11515: f64 = (assign8500_e11513 - 1.0);
            let assign8500_e11517: f64 = (assign8500_e11515).max(0.0);
            let assign8500_e11518: f64 = (2.0 * assign8500_e11517);
            locals.var_nuintd = assign8500_e11518;
            locals.var_nuintd_rv = 0.0;
        }

        if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 == 0.0)) && (locals.var_guard249 != 0.0)) {
            locals.var_nuends = 0.0;
            locals.var_nuends_rv = 0.0;
            locals.var_nuints = p.p2;
            locals.var_nuints_rv = 0.0;
        }

        if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 == 0.0)) && (locals.var_guard249 == 0.0)) {
            locals.var_nuendd = 0.0;
            locals.var_nuendd_rv = 0.0;
            locals.var_nuintd = p.p2;
            locals.var_nuintd_rv = 0.0;
            locals.var_nuends = 2.0;
            locals.var_nuends_rv = 0.0;
        }

        if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 == 0.0)) && (locals.var_guard249 == 0.0)) {
            let assign8560_e11609: f64 = (p.p2 / 2.0);
            let assign8560_e11611: f64 = (assign8560_e11609 - 1.0);
            let assign8560_e11613: f64 = (assign8560_e11611).max(0.0);
            let assign8560_e11614: f64 = (2.0 * assign8560_e11613);
            locals.var_nuints = assign8560_e11614;
            locals.var_nuints_rv = 0.0;
        }

        let assign8570_e11619: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard250 = assign8570_e11619;
        locals.var_guard250_rv = 0.0;

        let assign8580_e11622: f64 = if locals.var_nuints == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard251 = assign8580_e11622;
        locals.var_guard251_rv = 0.0;

        if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard250 != 0.0)) && (locals.var_guard251 != 0.0)) {
            locals.var_rint = 0.0;
            locals.var_rint_rv = 0.0;
        }

        if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard250 != 0.0)) && (locals.var_guard251 == 0.0)) {
            let assign8600_e11649: f64 = (p.p374 * locals.var_dmcgeff);
            let assign8600_e11652: f64 = (locals.var_weff * locals.var_nuints);
            let assign8600_e11653: f64 = (assign8600_e11649 / assign8600_e11652);
            locals.var_rint = assign8600_e11653;
            locals.var_rint_rv = 0.0;
        }

        let assign8610_e11658: f64 = if locals.var_nuintd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard252 = assign8610_e11658;
        locals.var_guard252_rv = 0.0;

        if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard250 == 0.0)) && (locals.var_guard252 != 0.0)) {
            locals.var_rint = 0.0;
            locals.var_rint_rv = 0.0;
        }

        if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard250 == 0.0)) && (locals.var_guard252 == 0.0)) {
            let assign8630_e11687: f64 = (p.p374 * locals.var_dmcgeff);
            let assign8630_e11690: f64 = (locals.var_weff * locals.var_nuintd);
            let assign8630_e11691: f64 = (assign8630_e11687 / assign8630_e11690);
            locals.var_rint = assign8630_e11691;
            locals.var_rint_rv = 0.0;
        }

        let assign8640_e11696: f64 = if p.p9 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard253 = assign8640_e11696;
        locals.var_guard253_rv = 0.0;

        let assign8650_e11699: f64 = if p.p9 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard254 = assign8650_e11699;
        locals.var_guard254_rv = 0.0;

        let assign8660_e11702: f64 = if p.p9 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard255 = assign8660_e11702;
        locals.var_guard255_rv = 0.0;

        let assign8670_e11705: f64 = if p.p9 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard256 = assign8670_e11705;
        locals.var_guard256_rv = 0.0;

        let assign8680_e11708: f64 = if p.p9 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard257 = assign8680_e11708;
        locals.var_guard257_rv = 0.0;

        let assign8690_e11711: f64 = if p.p9 == 5.0 { 1.0 } else { 0.0 };
        locals.var_guard258 = assign8690_e11711;
        locals.var_guard258_rv = 0.0;

        let assign8700_e11714: f64 = if p.p9 == 6.0 { 1.0 } else { 0.0 };
        locals.var_guard259 = assign8700_e11714;
        locals.var_guard259_rv = 0.0;

        let assign8710_e11717: f64 = if p.p9 == 7.0 { 1.0 } else { 0.0 };
        locals.var_guard260 = assign8710_e11717;
        locals.var_guard260_rv = 0.0;

        let assign8720_e11720: f64 = if p.p9 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard261 = assign8720_e11720;
        locals.var_guard261_rv = 0.0;

        let assign8730_e11723: f64 = if p.p9 == 9.0 { 1.0 } else { 0.0 };
        locals.var_guard262 = assign8730_e11723;
        locals.var_guard262_rv = 0.0;

        let assign8740_e11726: f64 = if p.p9 == 10.0 { 1.0 } else { 0.0 };
        locals.var_guard263 = assign8740_e11726;
        locals.var_guard263_rv = 0.0;

        let assign8750_e11729: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard264 = assign8750_e11729;
        locals.var_guard264_rv = 0.0;

        let assign8760_e11732: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard265 = assign8760_e11732;
        locals.var_guard265_rv = 0.0;

        let assign8770_e11743: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard266 = assign8770_e11743;
        locals.var_guard266_rv = 0.0;

        let assign8780_e11754: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard267 = assign8780_e11754;
        locals.var_guard267_rv = 0.0;

        let assign8790_e11757: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard268 = assign8790_e11757;
        locals.var_guard268_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 != 0.0)) && (locals.var_guard265 != 0.0)) && (locals.var_guard266 != 0.0)) && (locals.var_guard268 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 != 0.0)) && (locals.var_guard265 != 0.0)) && (locals.var_guard266 != 0.0)) && (locals.var_guard268 == 0.0)) {
            let assign8810_e11792: f64 = (p.p374 * locals.var_dmcgeff);
            let assign8810_e11795: f64 = (locals.var_weff * locals.var_nuends);
            let assign8810_e11796: f64 = (assign8810_e11792 / assign8810_e11795);
            locals.var_rend = assign8810_e11796;
            locals.var_rend_rv = 0.0;
        }

        let assign8830_e11809: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign8830_e11812: f64 = if ((locals.var_nuends == 0.0) || (assign8830_e11809 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard270 = assign8830_e11812;
        locals.var_guard270_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 != 0.0)) && (locals.var_guard265 != 0.0)) && ((locals.var_guard267 != 0.0) && (locals.var_guard266 == 0.0))) && (locals.var_guard270 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 != 0.0)) && (locals.var_guard265 != 0.0)) && ((locals.var_guard267 != 0.0) && (locals.var_guard266 == 0.0))) && (locals.var_guard270 == 0.0)) {
            let assign8850_e11853: f64 = (p.p374 * locals.var_weff);
            let assign8850_e11856: f64 = (3.0 * locals.var_nuends);
            let assign8850_e11859: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign8850_e11860: f64 = (assign8850_e11856 * assign8850_e11859);
            let assign8850_e11861: f64 = (assign8850_e11853 / assign8850_e11860);
            locals.var_rend = assign8850_e11861;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 != 0.0)) && (locals.var_guard265 != 0.0)) && (!((locals.var_guard266 != 0.0) || (locals.var_guard267 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign8870_e11892: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard271 = assign8870_e11892;
        locals.var_guard271_rv = 0.0;

        let assign8880_e11903: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard272 = assign8880_e11903;
        locals.var_guard272_rv = 0.0;

        let assign8890_e11906: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard273 = assign8890_e11906;
        locals.var_guard273_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard271 != 0.0)) && (locals.var_guard273 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard271 != 0.0)) && (locals.var_guard273 == 0.0)) {
            let assign8910_e11943: f64 = (p.p374 * locals.var_dmcgeff);
            let assign8910_e11946: f64 = (locals.var_weff * locals.var_nuends);
            let assign8910_e11947: f64 = (assign8910_e11943 / assign8910_e11946);
            locals.var_rend = assign8910_e11947;
            locals.var_rend_rv = 0.0;
        }

        let assign8930_e11960: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign8930_e11963: f64 = if ((locals.var_nuends == 0.0) || (assign8930_e11960 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard275 = assign8930_e11963;
        locals.var_guard275_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 != 0.0)) && (locals.var_guard265 == 0.0)) && ((locals.var_guard272 != 0.0) && (locals.var_guard271 == 0.0))) && (locals.var_guard275 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 != 0.0)) && (locals.var_guard265 == 0.0)) && ((locals.var_guard272 != 0.0) && (locals.var_guard271 == 0.0))) && (locals.var_guard275 == 0.0)) {
            let assign8950_e12006: f64 = (p.p374 * locals.var_weff);
            let assign8950_e12009: f64 = (3.0 * locals.var_nuends);
            let assign8950_e12012: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign8950_e12013: f64 = (assign8950_e12009 * assign8950_e12012);
            let assign8950_e12014: f64 = (assign8950_e12006 / assign8950_e12013);
            locals.var_rend = assign8950_e12014;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 != 0.0)) && (locals.var_guard265 == 0.0)) && (!((locals.var_guard271 != 0.0) || (locals.var_guard272 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign8970_e12038: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard276 = assign8970_e12038;
        locals.var_guard276_rv = 0.0;

        let assign8980_e12049: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard277 = assign8980_e12049;
        locals.var_guard277_rv = 0.0;

        let assign8990_e12060: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard278 = assign8990_e12060;
        locals.var_guard278_rv = 0.0;

        let assign9000_e12063: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard279 = assign9000_e12063;
        locals.var_guard279_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 == 0.0)) && (locals.var_guard276 != 0.0)) && (locals.var_guard277 != 0.0)) && (locals.var_guard279 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 == 0.0)) && (locals.var_guard276 != 0.0)) && (locals.var_guard277 != 0.0)) && (locals.var_guard279 == 0.0)) {
            let assign9020_e12100: f64 = (p.p374 * locals.var_dmcgeff);
            let assign9020_e12103: f64 = (locals.var_weff * locals.var_nuendd);
            let assign9020_e12104: f64 = (assign9020_e12100 / assign9020_e12103);
            locals.var_rend = assign9020_e12104;
            locals.var_rend_rv = 0.0;
        }

        let assign9040_e12117: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign9040_e12120: f64 = if ((locals.var_nuendd == 0.0) || (assign9040_e12117 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard281 = assign9040_e12120;
        locals.var_guard281_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 == 0.0)) && (locals.var_guard276 != 0.0)) && ((locals.var_guard278 != 0.0) && (locals.var_guard277 == 0.0))) && (locals.var_guard281 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 == 0.0)) && (locals.var_guard276 != 0.0)) && ((locals.var_guard278 != 0.0) && (locals.var_guard277 == 0.0))) && (locals.var_guard281 == 0.0)) {
            let assign9060_e12163: f64 = (p.p374 * locals.var_weff);
            let assign9060_e12166: f64 = (3.0 * locals.var_nuendd);
            let assign9060_e12169: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign9060_e12170: f64 = (assign9060_e12166 * assign9060_e12169);
            let assign9060_e12171: f64 = (assign9060_e12163 / assign9060_e12170);
            locals.var_rend = assign9060_e12171;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 == 0.0)) && (locals.var_guard276 != 0.0)) && (!((locals.var_guard277 != 0.0) || (locals.var_guard278 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign9080_e12203: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard282 = assign9080_e12203;
        locals.var_guard282_rv = 0.0;

        let assign9090_e12214: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard283 = assign9090_e12214;
        locals.var_guard283_rv = 0.0;

        let assign9100_e12217: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard284 = assign9100_e12217;
        locals.var_guard284_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 == 0.0)) && (locals.var_guard276 == 0.0)) && (locals.var_guard282 != 0.0)) && (locals.var_guard284 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 == 0.0)) && (locals.var_guard276 == 0.0)) && (locals.var_guard282 != 0.0)) && (locals.var_guard284 == 0.0)) {
            let assign9120_e12256: f64 = (p.p374 * locals.var_dmcgeff);
            let assign9120_e12259: f64 = (locals.var_weff * locals.var_nuendd);
            let assign9120_e12260: f64 = (assign9120_e12256 / assign9120_e12259);
            locals.var_rend = assign9120_e12260;
            locals.var_rend_rv = 0.0;
        }

        let assign9140_e12273: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign9140_e12276: f64 = if ((locals.var_nuendd == 0.0) || (assign9140_e12273 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard286 = assign9140_e12276;
        locals.var_guard286_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 == 0.0)) && (locals.var_guard276 == 0.0)) && ((locals.var_guard283 != 0.0) && (locals.var_guard282 == 0.0))) && (locals.var_guard286 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 == 0.0)) && (locals.var_guard276 == 0.0)) && ((locals.var_guard283 != 0.0) && (locals.var_guard282 == 0.0))) && (locals.var_guard286 == 0.0)) {
            let assign9160_e12321: f64 = (p.p374 * locals.var_weff);
            let assign9160_e12324: f64 = (3.0 * locals.var_nuendd);
            let assign9160_e12327: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign9160_e12328: f64 = (assign9160_e12324 * assign9160_e12327);
            let assign9160_e12329: f64 = (assign9160_e12321 / assign9160_e12328);
            locals.var_rend = assign9160_e12329;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 == 0.0)) && (locals.var_guard276 == 0.0)) && (!((locals.var_guard282 != 0.0) || (locals.var_guard283 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign9180_e12354: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard287 = assign9180_e12354;
        locals.var_guard287_rv = 0.0;

        let assign9190_e12357: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard288 = assign9190_e12357;
        locals.var_guard288_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_10(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign9200_e12368: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard289 = assign9200_e12368;
        locals.var_guard289_rv = 0.0;

        let assign9210_e12379: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard290 = assign9210_e12379;
        locals.var_guard290_rv = 0.0;

        let assign9220_e12382: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard291 = assign9220_e12382;
        locals.var_guard291_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 != 0.0)) && (locals.var_guard288 != 0.0)) && (locals.var_guard289 != 0.0)) && (locals.var_guard291 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 != 0.0)) && (locals.var_guard288 != 0.0)) && (locals.var_guard289 != 0.0)) && (locals.var_guard291 == 0.0)) {
            let assign9240_e12423: f64 = (p.p374 * locals.var_dmcgeff);
            let assign9240_e12426: f64 = (locals.var_weff * locals.var_nuends);
            let assign9240_e12427: f64 = (assign9240_e12423 / assign9240_e12426);
            locals.var_rend = assign9240_e12427;
            locals.var_rend_rv = 0.0;
        }

        let assign9260_e12440: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign9260_e12443: f64 = if ((locals.var_nuends == 0.0) || (assign9260_e12440 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard293 = assign9260_e12443;
        locals.var_guard293_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 != 0.0)) && (locals.var_guard288 != 0.0)) && ((locals.var_guard290 != 0.0) && (locals.var_guard289 == 0.0))) && (locals.var_guard293 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 != 0.0)) && (locals.var_guard288 != 0.0)) && ((locals.var_guard290 != 0.0) && (locals.var_guard289 == 0.0))) && (locals.var_guard293 == 0.0)) {
            let assign9280_e12490: f64 = (p.p374 * locals.var_weff);
            let assign9280_e12493: f64 = (3.0 * locals.var_nuends);
            let assign9280_e12496: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign9280_e12497: f64 = (assign9280_e12493 * assign9280_e12496);
            let assign9280_e12498: f64 = (assign9280_e12490 / assign9280_e12497);
            locals.var_rend = assign9280_e12498;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 != 0.0)) && (locals.var_guard288 != 0.0)) && (!((locals.var_guard289 != 0.0) || (locals.var_guard290 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign9300_e12532: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard294 = assign9300_e12532;
        locals.var_guard294_rv = 0.0;

        let assign9310_e12543: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard295 = assign9310_e12543;
        locals.var_guard295_rv = 0.0;

        let assign9320_e12546: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard296 = assign9320_e12546;
        locals.var_guard296_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 != 0.0)) && (locals.var_guard288 == 0.0)) && (locals.var_guard294 != 0.0)) && (locals.var_guard296 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 != 0.0)) && (locals.var_guard288 == 0.0)) && (locals.var_guard294 != 0.0)) && (locals.var_guard296 == 0.0)) {
            let assign9340_e12589: f64 = (p.p374 * locals.var_dmcgeff);
            let assign9340_e12592: f64 = (locals.var_weff * locals.var_nuends);
            let assign9340_e12593: f64 = (assign9340_e12589 / assign9340_e12592);
            locals.var_rend = assign9340_e12593;
            locals.var_rend_rv = 0.0;
        }

        let assign9360_e12606: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign9360_e12609: f64 = if ((locals.var_nuends == 0.0) || (assign9360_e12606 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard298 = assign9360_e12609;
        locals.var_guard298_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 != 0.0)) && (locals.var_guard288 == 0.0)) && ((locals.var_guard295 != 0.0) && (locals.var_guard294 == 0.0))) && (locals.var_guard298 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 != 0.0)) && (locals.var_guard288 == 0.0)) && ((locals.var_guard295 != 0.0) && (locals.var_guard294 == 0.0))) && (locals.var_guard298 == 0.0)) {
            let assign9380_e12658: f64 = (p.p374 * locals.var_weff);
            let assign9380_e12661: f64 = (3.0 * locals.var_nuends);
            let assign9380_e12664: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign9380_e12665: f64 = (assign9380_e12661 * assign9380_e12664);
            let assign9380_e12666: f64 = (assign9380_e12658 / assign9380_e12665);
            locals.var_rend = assign9380_e12666;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 != 0.0)) && (locals.var_guard288 == 0.0)) && (!((locals.var_guard294 != 0.0) || (locals.var_guard295 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign9400_e12693: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard299 = assign9400_e12693;
        locals.var_guard299_rv = 0.0;

        let assign9410_e12704: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard300 = assign9410_e12704;
        locals.var_guard300_rv = 0.0;

        let assign9420_e12715: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard301 = assign9420_e12715;
        locals.var_guard301_rv = 0.0;

        let assign9430_e12718: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard302 = assign9430_e12718;
        locals.var_guard302_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 == 0.0)) && (locals.var_guard299 != 0.0)) && (locals.var_guard300 != 0.0)) && (locals.var_guard302 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 == 0.0)) && (locals.var_guard299 != 0.0)) && (locals.var_guard300 != 0.0)) && (locals.var_guard302 == 0.0)) {
            let assign9450_e12761: f64 = (p.p374 * locals.var_dmcgeff);
            let assign9450_e12764: f64 = (locals.var_weff * locals.var_nuendd);
            let assign9450_e12765: f64 = (assign9450_e12761 / assign9450_e12764);
            locals.var_rend = assign9450_e12765;
            locals.var_rend_rv = 0.0;
        }

        let assign9470_e12777: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard304 = assign9470_e12777;
        locals.var_guard304_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 == 0.0)) && (locals.var_guard299 != 0.0)) && ((locals.var_guard301 != 0.0) && (locals.var_guard300 == 0.0))) && (locals.var_guard304 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 == 0.0)) && (locals.var_guard299 != 0.0)) && ((locals.var_guard301 != 0.0) && (locals.var_guard300 == 0.0))) && (locals.var_guard304 == 0.0)) {
            let assign9490_e12826: f64 = (p.p374 * locals.var_weff);
            let assign9490_e12829: f64 = (6.0 * locals.var_nuendd);
            let assign9490_e12831: f64 = (assign9490_e12829 * locals.var_dmcgeff);
            let assign9490_e12832: f64 = (assign9490_e12826 / assign9490_e12831);
            locals.var_rend = assign9490_e12832;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 == 0.0)) && (locals.var_guard299 != 0.0)) && (!((locals.var_guard300 != 0.0) || (locals.var_guard301 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign9510_e12867: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard305 = assign9510_e12867;
        locals.var_guard305_rv = 0.0;

        let assign9520_e12878: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard306 = assign9520_e12878;
        locals.var_guard306_rv = 0.0;

        let assign9530_e12881: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard307 = assign9530_e12881;
        locals.var_guard307_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 == 0.0)) && (locals.var_guard299 == 0.0)) && (locals.var_guard305 != 0.0)) && (locals.var_guard307 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 == 0.0)) && (locals.var_guard299 == 0.0)) && (locals.var_guard305 != 0.0)) && (locals.var_guard307 == 0.0)) {
            let assign9550_e12926: f64 = (p.p374 * locals.var_dmcgeff);
            let assign9550_e12929: f64 = (locals.var_weff * locals.var_nuendd);
            let assign9550_e12930: f64 = (assign9550_e12926 / assign9550_e12929);
            locals.var_rend = assign9550_e12930;
            locals.var_rend_rv = 0.0;
        }

        let assign9570_e12942: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard309 = assign9570_e12942;
        locals.var_guard309_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 == 0.0)) && (locals.var_guard299 == 0.0)) && ((locals.var_guard306 != 0.0) && (locals.var_guard305 == 0.0))) && (locals.var_guard309 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 == 0.0)) && (locals.var_guard299 == 0.0)) && ((locals.var_guard306 != 0.0) && (locals.var_guard305 == 0.0))) && (locals.var_guard309 == 0.0)) {
            let assign9590_e12993: f64 = (p.p374 * locals.var_weff);
            let assign9590_e12996: f64 = (6.0 * locals.var_nuendd);
            let assign9590_e12998: f64 = (assign9590_e12996 * locals.var_dmcgeff);
            let assign9590_e12999: f64 = (assign9590_e12993 / assign9590_e12998);
            locals.var_rend = assign9590_e12999;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 == 0.0)) && (locals.var_guard299 == 0.0)) && (!((locals.var_guard305 != 0.0) || (locals.var_guard306 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign9610_e13027: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard310 = assign9610_e13027;
        locals.var_guard310_rv = 0.0;

        let assign9620_e13030: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard311 = assign9620_e13030;
        locals.var_guard311_rv = 0.0;

        let assign9630_e13041: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard312 = assign9630_e13041;
        locals.var_guard312_rv = 0.0;

        let assign9640_e13052: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard313 = assign9640_e13052;
        locals.var_guard313_rv = 0.0;

        let assign9650_e13055: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard314 = assign9650_e13055;
        locals.var_guard314_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 != 0.0)) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 != 0.0)) && (locals.var_guard314 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 != 0.0)) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 != 0.0)) && (locals.var_guard314 == 0.0)) {
            let assign9670_e13100: f64 = (p.p374 * locals.var_dmcgeff);
            let assign9670_e13103: f64 = (locals.var_weff * locals.var_nuends);
            let assign9670_e13104: f64 = (assign9670_e13100 / assign9670_e13103);
            locals.var_rend = assign9670_e13104;
            locals.var_rend_rv = 0.0;
        }

        let assign9690_e13116: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard316 = assign9690_e13116;
        locals.var_guard316_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 != 0.0)) && (locals.var_guard311 != 0.0)) && ((locals.var_guard313 != 0.0) && (locals.var_guard312 == 0.0))) && (locals.var_guard316 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 != 0.0)) && (locals.var_guard311 != 0.0)) && ((locals.var_guard313 != 0.0) && (locals.var_guard312 == 0.0))) && (locals.var_guard316 == 0.0)) {
            let assign9710_e13167: f64 = (p.p374 * locals.var_weff);
            let assign9710_e13170: f64 = (6.0 * locals.var_nuends);
            let assign9710_e13172: f64 = (assign9710_e13170 * locals.var_dmcgeff);
            let assign9710_e13173: f64 = (assign9710_e13167 / assign9710_e13172);
            locals.var_rend = assign9710_e13173;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 != 0.0)) && (locals.var_guard311 != 0.0)) && (!((locals.var_guard312 != 0.0) || (locals.var_guard313 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign9730_e13209: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard317 = assign9730_e13209;
        locals.var_guard317_rv = 0.0;

        let assign9740_e13220: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard318 = assign9740_e13220;
        locals.var_guard318_rv = 0.0;

        let assign9750_e13223: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard319 = assign9750_e13223;
        locals.var_guard319_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 != 0.0)) && (locals.var_guard311 == 0.0)) && (locals.var_guard317 != 0.0)) && (locals.var_guard319 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 != 0.0)) && (locals.var_guard311 == 0.0)) && (locals.var_guard317 != 0.0)) && (locals.var_guard319 == 0.0)) {
            let assign9770_e13270: f64 = (p.p374 * locals.var_dmcgeff);
            let assign9770_e13273: f64 = (locals.var_weff * locals.var_nuends);
            let assign9770_e13274: f64 = (assign9770_e13270 / assign9770_e13273);
            locals.var_rend = assign9770_e13274;
            locals.var_rend_rv = 0.0;
        }

        let assign9790_e13286: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard321 = assign9790_e13286;
        locals.var_guard321_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 != 0.0)) && (locals.var_guard311 == 0.0)) && ((locals.var_guard318 != 0.0) && (locals.var_guard317 == 0.0))) && (locals.var_guard321 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 != 0.0)) && (locals.var_guard311 == 0.0)) && ((locals.var_guard318 != 0.0) && (locals.var_guard317 == 0.0))) && (locals.var_guard321 == 0.0)) {
            let assign9810_e13339: f64 = (p.p374 * locals.var_weff);
            let assign9810_e13342: f64 = (6.0 * locals.var_nuends);
            let assign9810_e13344: f64 = (assign9810_e13342 * locals.var_dmcgeff);
            let assign9810_e13345: f64 = (assign9810_e13339 / assign9810_e13344);
            locals.var_rend = assign9810_e13345;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 != 0.0)) && (locals.var_guard311 == 0.0)) && (!((locals.var_guard317 != 0.0) || (locals.var_guard318 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign9830_e13374: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard322 = assign9830_e13374;
        locals.var_guard322_rv = 0.0;

        let assign9840_e13385: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard323 = assign9840_e13385;
        locals.var_guard323_rv = 0.0;

        let assign9850_e13396: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard324 = assign9850_e13396;
        locals.var_guard324_rv = 0.0;

        let assign9860_e13399: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard325 = assign9860_e13399;
        locals.var_guard325_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 == 0.0)) && (locals.var_guard322 != 0.0)) && (locals.var_guard323 != 0.0)) && (locals.var_guard325 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 == 0.0)) && (locals.var_guard322 != 0.0)) && (locals.var_guard323 != 0.0)) && (locals.var_guard325 == 0.0)) {
            let assign9880_e13446: f64 = (p.p374 * locals.var_dmcgeff);
            let assign9880_e13449: f64 = (locals.var_weff * locals.var_nuendd);
            let assign9880_e13450: f64 = (assign9880_e13446 / assign9880_e13449);
            locals.var_rend = assign9880_e13450;
            locals.var_rend_rv = 0.0;
        }

        let assign9900_e13463: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign9900_e13466: f64 = if ((locals.var_nuendd == 0.0) || (assign9900_e13463 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard327 = assign9900_e13466;
        locals.var_guard327_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 == 0.0)) && (locals.var_guard322 != 0.0)) && ((locals.var_guard324 != 0.0) && (locals.var_guard323 == 0.0))) && (locals.var_guard327 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 == 0.0)) && (locals.var_guard322 != 0.0)) && ((locals.var_guard324 != 0.0) && (locals.var_guard323 == 0.0))) && (locals.var_guard327 == 0.0)) {
            let assign9920_e13519: f64 = (p.p374 * locals.var_weff);
            let assign9920_e13522: f64 = (3.0 * locals.var_nuendd);
            let assign9920_e13525: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign9920_e13526: f64 = (assign9920_e13522 * assign9920_e13525);
            let assign9920_e13527: f64 = (assign9920_e13519 / assign9920_e13526);
            locals.var_rend = assign9920_e13527;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 == 0.0)) && (locals.var_guard322 != 0.0)) && (!((locals.var_guard323 != 0.0) || (locals.var_guard324 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign9940_e13564: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard328 = assign9940_e13564;
        locals.var_guard328_rv = 0.0;

        let assign9950_e13575: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard329 = assign9950_e13575;
        locals.var_guard329_rv = 0.0;

        let assign9960_e13578: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard330 = assign9960_e13578;
        locals.var_guard330_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 == 0.0)) && (locals.var_guard322 == 0.0)) && (locals.var_guard328 != 0.0)) && (locals.var_guard330 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 == 0.0)) && (locals.var_guard322 == 0.0)) && (locals.var_guard328 != 0.0)) && (locals.var_guard330 == 0.0)) {
            let assign9980_e13627: f64 = (p.p374 * locals.var_dmcgeff);
            let assign9980_e13630: f64 = (locals.var_weff * locals.var_nuendd);
            let assign9980_e13631: f64 = (assign9980_e13627 / assign9980_e13630);
            locals.var_rend = assign9980_e13631;
            locals.var_rend_rv = 0.0;
        }

        let assign10000_e13644: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign10000_e13647: f64 = if ((locals.var_nuendd == 0.0) || (assign10000_e13644 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard332 = assign10000_e13647;
        locals.var_guard332_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 == 0.0)) && (locals.var_guard322 == 0.0)) && ((locals.var_guard329 != 0.0) && (locals.var_guard328 == 0.0))) && (locals.var_guard332 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 == 0.0)) && (locals.var_guard322 == 0.0)) && ((locals.var_guard329 != 0.0) && (locals.var_guard328 == 0.0))) && (locals.var_guard332 == 0.0)) {
            let assign10020_e13702: f64 = (p.p374 * locals.var_weff);
            let assign10020_e13705: f64 = (3.0 * locals.var_nuendd);
            let assign10020_e13708: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign10020_e13709: f64 = (assign10020_e13705 * assign10020_e13708);
            let assign10020_e13710: f64 = (assign10020_e13702 / assign10020_e13709);
            locals.var_rend = assign10020_e13710;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 == 0.0)) && (locals.var_guard322 == 0.0)) && (!((locals.var_guard328 != 0.0) || (locals.var_guard329 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign10040_e13740: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard333 = assign10040_e13740;
        locals.var_guard333_rv = 0.0;

        let assign10050_e13743: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard334 = assign10050_e13743;
        locals.var_guard334_rv = 0.0;

        let assign10060_e13754: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard335 = assign10060_e13754;
        locals.var_guard335_rv = 0.0;

        let assign10070_e13765: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard336 = assign10070_e13765;
        locals.var_guard336_rv = 0.0;

        let assign10080_e13768: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard337 = assign10080_e13768;
        locals.var_guard337_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 != 0.0)) && (locals.var_guard337 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 != 0.0)) && (locals.var_guard337 == 0.0)) {
            let assign10100_e13817: f64 = (p.p374 * locals.var_dmcgeff);
            let assign10100_e13820: f64 = (locals.var_weff * locals.var_nuends);
            let assign10100_e13821: f64 = (assign10100_e13817 / assign10100_e13820);
            locals.var_rend = assign10100_e13821;
            locals.var_rend_rv = 0.0;
        }

        let assign10120_e13833: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard339 = assign10120_e13833;
        locals.var_guard339_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 != 0.0)) && ((locals.var_guard336 != 0.0) && (locals.var_guard335 == 0.0))) && (locals.var_guard339 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 != 0.0)) && ((locals.var_guard336 != 0.0) && (locals.var_guard335 == 0.0))) && (locals.var_guard339 == 0.0)) {
            let assign10140_e13888: f64 = (p.p374 * locals.var_weff);
            let assign10140_e13891: f64 = (6.0 * locals.var_nuends);
            let assign10140_e13893: f64 = (assign10140_e13891 * locals.var_dmcgeff);
            let assign10140_e13894: f64 = (assign10140_e13888 / assign10140_e13893);
            locals.var_rend = assign10140_e13894;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 != 0.0)) && (!((locals.var_guard335 != 0.0) || (locals.var_guard336 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign10160_e13932: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard340 = assign10160_e13932;
        locals.var_guard340_rv = 0.0;

        let assign10170_e13943: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard341 = assign10170_e13943;
        locals.var_guard341_rv = 0.0;

        let assign10180_e13946: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard342 = assign10180_e13946;
        locals.var_guard342_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 == 0.0)) && (locals.var_guard340 != 0.0)) && (locals.var_guard342 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 == 0.0)) && (locals.var_guard340 != 0.0)) && (locals.var_guard342 == 0.0)) {
            let assign10200_e13997: f64 = (p.p374 * locals.var_dmcgeff);
            let assign10200_e14000: f64 = (locals.var_weff * locals.var_nuends);
            let assign10200_e14001: f64 = (assign10200_e13997 / assign10200_e14000);
            locals.var_rend = assign10200_e14001;
            locals.var_rend_rv = 0.0;
        }

        let assign10220_e14013: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard344 = assign10220_e14013;
        locals.var_guard344_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 == 0.0)) && ((locals.var_guard341 != 0.0) && (locals.var_guard340 == 0.0))) && (locals.var_guard344 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 == 0.0)) && ((locals.var_guard341 != 0.0) && (locals.var_guard340 == 0.0))) && (locals.var_guard344 == 0.0)) {
            let assign10240_e14070: f64 = (p.p374 * locals.var_weff);
            let assign10240_e14073: f64 = (6.0 * locals.var_nuends);
            let assign10240_e14075: f64 = (assign10240_e14073 * locals.var_dmcgeff);
            let assign10240_e14076: f64 = (assign10240_e14070 / assign10240_e14075);
            locals.var_rend = assign10240_e14076;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 == 0.0)) && (!((locals.var_guard340 != 0.0) || (locals.var_guard341 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_11(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign10260_e14107: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard345 = assign10260_e14107;
        locals.var_guard345_rv = 0.0;

        let assign10270_e14118: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard346 = assign10270_e14118;
        locals.var_guard346_rv = 0.0;

        let assign10280_e14129: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard347 = assign10280_e14129;
        locals.var_guard347_rv = 0.0;

        let assign10290_e14132: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard348 = assign10290_e14132;
        locals.var_guard348_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 == 0.0)) && (locals.var_guard345 != 0.0)) && (locals.var_guard346 != 0.0)) && (locals.var_guard348 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 == 0.0)) && (locals.var_guard345 != 0.0)) && (locals.var_guard346 != 0.0)) && (locals.var_guard348 == 0.0)) {
            let assign10310_e14183: f64 = (p.p374 * locals.var_dmcgeff);
            let assign10310_e14186: f64 = (locals.var_weff * locals.var_nuendd);
            let assign10310_e14187: f64 = (assign10310_e14183 / assign10310_e14186);
            locals.var_rend = assign10310_e14187;
            locals.var_rend_rv = 0.0;
        }

        let assign10330_e14199: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard350 = assign10330_e14199;
        locals.var_guard350_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 == 0.0)) && (locals.var_guard345 != 0.0)) && ((locals.var_guard347 != 0.0) && (locals.var_guard346 == 0.0))) && (locals.var_guard350 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 == 0.0)) && (locals.var_guard345 != 0.0)) && ((locals.var_guard347 != 0.0) && (locals.var_guard346 == 0.0))) && (locals.var_guard350 == 0.0)) {
            let assign10350_e14256: f64 = (p.p374 * locals.var_weff);
            let assign10350_e14259: f64 = (6.0 * locals.var_nuendd);
            let assign10350_e14261: f64 = (assign10350_e14259 * locals.var_dmcgeff);
            let assign10350_e14262: f64 = (assign10350_e14256 / assign10350_e14261);
            locals.var_rend = assign10350_e14262;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 == 0.0)) && (locals.var_guard345 != 0.0)) && (!((locals.var_guard346 != 0.0) || (locals.var_guard347 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign10370_e14301: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard351 = assign10370_e14301;
        locals.var_guard351_rv = 0.0;

        let assign10380_e14312: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard352 = assign10380_e14312;
        locals.var_guard352_rv = 0.0;

        let assign10390_e14315: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard353 = assign10390_e14315;
        locals.var_guard353_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 == 0.0)) && (locals.var_guard345 == 0.0)) && (locals.var_guard351 != 0.0)) && (locals.var_guard353 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 == 0.0)) && (locals.var_guard345 == 0.0)) && (locals.var_guard351 != 0.0)) && (locals.var_guard353 == 0.0)) {
            let assign10410_e14368: f64 = (p.p374 * locals.var_dmcgeff);
            let assign10410_e14371: f64 = (locals.var_weff * locals.var_nuendd);
            let assign10410_e14372: f64 = (assign10410_e14368 / assign10410_e14371);
            locals.var_rend = assign10410_e14372;
            locals.var_rend_rv = 0.0;
        }

        let assign10430_e14384: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard355 = assign10430_e14384;
        locals.var_guard355_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 == 0.0)) && (locals.var_guard345 == 0.0)) && ((locals.var_guard352 != 0.0) && (locals.var_guard351 == 0.0))) && (locals.var_guard355 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 == 0.0)) && (locals.var_guard345 == 0.0)) && ((locals.var_guard352 != 0.0) && (locals.var_guard351 == 0.0))) && (locals.var_guard355 == 0.0)) {
            let assign10450_e14443: f64 = (p.p374 * locals.var_weff);
            let assign10450_e14446: f64 = (6.0 * locals.var_nuendd);
            let assign10450_e14448: f64 = (assign10450_e14446 * locals.var_dmcgeff);
            let assign10450_e14449: f64 = (assign10450_e14443 / assign10450_e14448);
            locals.var_rend = assign10450_e14449;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 == 0.0)) && (locals.var_guard345 == 0.0)) && (!((locals.var_guard351 != 0.0) || (locals.var_guard352 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign10470_e14481: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard356 = assign10470_e14481;
        locals.var_guard356_rv = 0.0;

        let assign10480_e14484: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard357 = assign10480_e14484;
        locals.var_guard357_rv = 0.0;

        let assign10490_e14495: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard358 = assign10490_e14495;
        locals.var_guard358_rv = 0.0;

        let assign10500_e14506: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard359 = assign10500_e14506;
        locals.var_guard359_rv = 0.0;

        let assign10510_e14509: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard360 = assign10510_e14509;
        locals.var_guard360_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard257 != 0.0) && (!((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard356 != 0.0)) && (locals.var_guard357 != 0.0)) && (locals.var_guard358 != 0.0)) && (locals.var_guard360 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard257 != 0.0) && (!((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard356 != 0.0)) && (locals.var_guard357 != 0.0)) && (locals.var_guard358 != 0.0)) && (locals.var_guard360 == 0.0)) {
            let assign10530_e14562: f64 = (p.p374 * locals.var_dmcgeff);
            let assign10530_e14565: f64 = (locals.var_weff * locals.var_nuends);
            let assign10530_e14566: f64 = (assign10530_e14562 / assign10530_e14565);
            locals.var_rend = assign10530_e14566;
            locals.var_rend_rv = 0.0;
        }

        let assign10550_e14579: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign10550_e14582: f64 = if ((locals.var_nuends == 0.0) || (assign10550_e14579 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard362 = assign10550_e14582;
        locals.var_guard362_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard257 != 0.0) && (!((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard356 != 0.0)) && (locals.var_guard357 != 0.0)) && ((locals.var_guard359 != 0.0) && (locals.var_guard358 == 0.0))) && (locals.var_guard362 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard257 != 0.0) && (!((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard356 != 0.0)) && (locals.var_guard357 != 0.0)) && ((locals.var_guard359 != 0.0) && (locals.var_guard358 == 0.0))) && (locals.var_guard362 == 0.0)) {
            let assign10570_e14641: f64 = (p.p374 * locals.var_weff);
            let assign10570_e14644: f64 = (3.0 * locals.var_nuends);
            let assign10570_e14647: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign10570_e14648: f64 = (assign10570_e14644 * assign10570_e14647);
            let assign10570_e14649: f64 = (assign10570_e14641 / assign10570_e14648);
            locals.var_rend = assign10570_e14649;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard257 != 0.0) && (!((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard356 != 0.0)) && (locals.var_guard357 != 0.0)) && (!((locals.var_guard358 != 0.0) || (locals.var_guard359 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign10590_e14689: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard363 = assign10590_e14689;
        locals.var_guard363_rv = 0.0;

        let assign10600_e14700: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard364 = assign10600_e14700;
        locals.var_guard364_rv = 0.0;

        let assign10610_e14703: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard365 = assign10610_e14703;
        locals.var_guard365_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard257 != 0.0) && (!((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard356 != 0.0)) && (locals.var_guard357 == 0.0)) && (locals.var_guard363 != 0.0)) && (locals.var_guard365 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard257 != 0.0) && (!((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard356 != 0.0)) && (locals.var_guard357 == 0.0)) && (locals.var_guard363 != 0.0)) && (locals.var_guard365 == 0.0)) {
            let assign10630_e14758: f64 = (p.p374 * locals.var_dmcgeff);
            let assign10630_e14761: f64 = (locals.var_weff * locals.var_nuends);
            let assign10630_e14762: f64 = (assign10630_e14758 / assign10630_e14761);
            locals.var_rend = assign10630_e14762;
            locals.var_rend_rv = 0.0;
        }

        let assign10650_e14775: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign10650_e14778: f64 = if ((locals.var_nuends == 0.0) || (assign10650_e14775 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard367 = assign10650_e14778;
        locals.var_guard367_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard257 != 0.0) && (!((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard356 != 0.0)) && (locals.var_guard357 == 0.0)) && ((locals.var_guard364 != 0.0) && (locals.var_guard363 == 0.0))) && (locals.var_guard367 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard257 != 0.0) && (!((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard356 != 0.0)) && (locals.var_guard357 == 0.0)) && ((locals.var_guard364 != 0.0) && (locals.var_guard363 == 0.0))) && (locals.var_guard367 == 0.0)) {
            let assign10670_e14839: f64 = (p.p374 * locals.var_weff);
            let assign10670_e14842: f64 = (3.0 * locals.var_nuends);
            let assign10670_e14845: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign10670_e14846: f64 = (assign10670_e14842 * assign10670_e14845);
            let assign10670_e14847: f64 = (assign10670_e14839 / assign10670_e14846);
            locals.var_rend = assign10670_e14847;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard257 != 0.0) && (!((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard356 != 0.0)) && (locals.var_guard357 == 0.0)) && (!((locals.var_guard363 != 0.0) || (locals.var_guard364 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard257 != 0.0) && (!((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard356 == 0.0)) {
            let assign10690_e14898: f64 = (p.p374 * locals.var_dmdgeff);
            let assign10690_e14900: f64 = (assign10690_e14898 / locals.var_weff);
            locals.var_rend = assign10690_e14900;
            locals.var_rend_rv = 0.0;
        }

        let assign10700_e14905: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard368 = assign10700_e14905;
        locals.var_guard368_rv = 0.0;

        let assign10710_e14908: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard369 = assign10710_e14908;
        locals.var_guard369_rv = 0.0;

        let assign10720_e14919: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard370 = assign10720_e14919;
        locals.var_guard370_rv = 0.0;

        let assign10730_e14930: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard371 = assign10730_e14930;
        locals.var_guard371_rv = 0.0;

        let assign10740_e14933: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard372 = assign10740_e14933;
        locals.var_guard372_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 != 0.0)) && (locals.var_guard369 != 0.0)) && (locals.var_guard370 != 0.0)) && (locals.var_guard372 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 != 0.0)) && (locals.var_guard369 != 0.0)) && (locals.var_guard370 != 0.0)) && (locals.var_guard372 == 0.0)) {
            let assign10760_e14990: f64 = (p.p374 * locals.var_dmcgeff);
            let assign10760_e14993: f64 = (locals.var_weff * locals.var_nuends);
            let assign10760_e14994: f64 = (assign10760_e14990 / assign10760_e14993);
            locals.var_rend = assign10760_e14994;
            locals.var_rend_rv = 0.0;
        }

        let assign10780_e15006: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard374 = assign10780_e15006;
        locals.var_guard374_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 != 0.0)) && (locals.var_guard369 != 0.0)) && ((locals.var_guard371 != 0.0) && (locals.var_guard370 == 0.0))) && (locals.var_guard374 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 != 0.0)) && (locals.var_guard369 != 0.0)) && ((locals.var_guard371 != 0.0) && (locals.var_guard370 == 0.0))) && (locals.var_guard374 == 0.0)) {
            let assign10800_e15069: f64 = (p.p374 * locals.var_weff);
            let assign10800_e15072: f64 = (6.0 * locals.var_nuends);
            let assign10800_e15074: f64 = (assign10800_e15072 * locals.var_dmcgeff);
            let assign10800_e15075: f64 = (assign10800_e15069 / assign10800_e15074);
            locals.var_rend = assign10800_e15075;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 != 0.0)) && (locals.var_guard369 != 0.0)) && (!((locals.var_guard370 != 0.0) || (locals.var_guard371 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign10820_e15117: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard375 = assign10820_e15117;
        locals.var_guard375_rv = 0.0;

        let assign10830_e15128: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard376 = assign10830_e15128;
        locals.var_guard376_rv = 0.0;

        let assign10840_e15131: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard377 = assign10840_e15131;
        locals.var_guard377_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 != 0.0)) && (locals.var_guard369 == 0.0)) && (locals.var_guard375 != 0.0)) && (locals.var_guard377 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 != 0.0)) && (locals.var_guard369 == 0.0)) && (locals.var_guard375 != 0.0)) && (locals.var_guard377 == 0.0)) {
            let assign10860_e15190: f64 = (p.p374 * locals.var_dmcgeff);
            let assign10860_e15193: f64 = (locals.var_weff * locals.var_nuends);
            let assign10860_e15194: f64 = (assign10860_e15190 / assign10860_e15193);
            locals.var_rend = assign10860_e15194;
            locals.var_rend_rv = 0.0;
        }

        let assign10880_e15206: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard379 = assign10880_e15206;
        locals.var_guard379_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 != 0.0)) && (locals.var_guard369 == 0.0)) && ((locals.var_guard376 != 0.0) && (locals.var_guard375 == 0.0))) && (locals.var_guard379 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 != 0.0)) && (locals.var_guard369 == 0.0)) && ((locals.var_guard376 != 0.0) && (locals.var_guard375 == 0.0))) && (locals.var_guard379 == 0.0)) {
            let assign10900_e15271: f64 = (p.p374 * locals.var_weff);
            let assign10900_e15274: f64 = (6.0 * locals.var_nuends);
            let assign10900_e15276: f64 = (assign10900_e15274 * locals.var_dmcgeff);
            let assign10900_e15277: f64 = (assign10900_e15271 / assign10900_e15276);
            locals.var_rend = assign10900_e15277;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 != 0.0)) && (locals.var_guard369 == 0.0)) && (!((locals.var_guard375 != 0.0) || (locals.var_guard376 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign10920_e15312: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard380 = assign10920_e15312;
        locals.var_guard380_rv = 0.0;

        if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 == 0.0)) && (locals.var_guard380 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 == 0.0)) && (locals.var_guard380 == 0.0)) {
            let assign10940_e15363: f64 = (p.p374 * locals.var_dmdgeff);
            let assign10940_e15366: f64 = (locals.var_weff * locals.var_nuendd);
            let assign10940_e15367: f64 = (assign10940_e15363 / assign10940_e15366);
            locals.var_rend = assign10940_e15367;
            locals.var_rend_rv = 0.0;
        }

        let assign10950_e15372: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard381 = assign10950_e15372;
        locals.var_guard381_rv = 0.0;

        if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 != 0.0)) {
            let assign10960_e15396: f64 = (p.p374 * locals.var_dmdgeff);
            let assign10960_e15398: f64 = (assign10960_e15396 / locals.var_weff);
            locals.var_rend = assign10960_e15398;
            locals.var_rend_rv = 0.0;
        }

        let assign10970_e15403: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard382 = assign10970_e15403;
        locals.var_guard382_rv = 0.0;

        let assign10980_e15414: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard383 = assign10980_e15414;
        locals.var_guard383_rv = 0.0;

        let assign10990_e15425: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard384 = assign10990_e15425;
        locals.var_guard384_rv = 0.0;

        let assign11000_e15428: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard385 = assign11000_e15428;
        locals.var_guard385_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 == 0.0)) && (locals.var_guard382 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard385 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 == 0.0)) && (locals.var_guard382 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard385 == 0.0)) {
            let assign11020_e15491: f64 = (p.p374 * locals.var_dmcgeff);
            let assign11020_e15494: f64 = (locals.var_weff * locals.var_nuendd);
            let assign11020_e15495: f64 = (assign11020_e15491 / assign11020_e15494);
            locals.var_rend = assign11020_e15495;
            locals.var_rend_rv = 0.0;
        }

        let assign11040_e15508: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign11040_e15511: f64 = if ((locals.var_nuendd == 0.0) || (assign11040_e15508 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard387 = assign11040_e15511;
        locals.var_guard387_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 == 0.0)) && (locals.var_guard382 != 0.0)) && ((locals.var_guard384 != 0.0) && (locals.var_guard383 == 0.0))) && (locals.var_guard387 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 == 0.0)) && (locals.var_guard382 != 0.0)) && ((locals.var_guard384 != 0.0) && (locals.var_guard383 == 0.0))) && (locals.var_guard387 == 0.0)) {
            let assign11060_e15580: f64 = (p.p374 * locals.var_weff);
            let assign11060_e15583: f64 = (3.0 * locals.var_nuendd);
            let assign11060_e15586: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign11060_e15587: f64 = (assign11060_e15583 * assign11060_e15586);
            let assign11060_e15588: f64 = (assign11060_e15580 / assign11060_e15587);
            locals.var_rend = assign11060_e15588;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 == 0.0)) && (locals.var_guard382 != 0.0)) && (!((locals.var_guard383 != 0.0) || (locals.var_guard384 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign11080_e15633: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard388 = assign11080_e15633;
        locals.var_guard388_rv = 0.0;

        let assign11090_e15644: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard389 = assign11090_e15644;
        locals.var_guard389_rv = 0.0;

        let assign11100_e15647: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard390 = assign11100_e15647;
        locals.var_guard390_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 == 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard388 != 0.0)) && (locals.var_guard390 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 == 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard388 != 0.0)) && (locals.var_guard390 == 0.0)) {
            let assign11120_e15712: f64 = (p.p374 * locals.var_dmcgeff);
            let assign11120_e15715: f64 = (locals.var_weff * locals.var_nuendd);
            let assign11120_e15716: f64 = (assign11120_e15712 / assign11120_e15715);
            locals.var_rend = assign11120_e15716;
            locals.var_rend_rv = 0.0;
        }

        let assign11140_e15729: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign11140_e15732: f64 = if ((locals.var_nuendd == 0.0) || (assign11140_e15729 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard392 = assign11140_e15732;
        locals.var_guard392_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 == 0.0)) && (locals.var_guard382 == 0.0)) && ((locals.var_guard389 != 0.0) && (locals.var_guard388 == 0.0))) && (locals.var_guard392 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 == 0.0)) && (locals.var_guard382 == 0.0)) && ((locals.var_guard389 != 0.0) && (locals.var_guard388 == 0.0))) && (locals.var_guard392 == 0.0)) {
            let assign11160_e15803: f64 = (p.p374 * locals.var_weff);
            let assign11160_e15806: f64 = (3.0 * locals.var_nuendd);
            let assign11160_e15809: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign11160_e15810: f64 = (assign11160_e15806 * assign11160_e15809);
            let assign11160_e15811: f64 = (assign11160_e15803 / assign11160_e15810);
            locals.var_rend = assign11160_e15811;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 == 0.0)) && (locals.var_guard382 == 0.0)) && (!((locals.var_guard388 != 0.0) || (locals.var_guard389 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign11180_e15849: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard393 = assign11180_e15849;
        locals.var_guard393_rv = 0.0;

        let assign11190_e15852: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard394 = assign11190_e15852;
        locals.var_guard394_rv = 0.0;

        if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 != 0.0)) && (locals.var_guard394 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 != 0.0)) && (locals.var_guard394 == 0.0)) {
            let assign11210_e15909: f64 = (p.p374 * locals.var_dmdgeff);
            let assign11210_e15912: f64 = (locals.var_weff * locals.var_nuends);
            let assign11210_e15913: f64 = (assign11210_e15909 / assign11210_e15912);
            locals.var_rend = assign11210_e15913;
            locals.var_rend_rv = 0.0;
        }

        let assign11220_e15918: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard395 = assign11220_e15918;
        locals.var_guard395_rv = 0.0;

        let assign11230_e15929: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard396 = assign11230_e15929;
        locals.var_guard396_rv = 0.0;

        let assign11240_e15940: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard397 = assign11240_e15940;
        locals.var_guard397_rv = 0.0;

        let assign11250_e15943: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard398 = assign11250_e15943;
        locals.var_guard398_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 == 0.0)) && (locals.var_guard395 != 0.0)) && (locals.var_guard396 != 0.0)) && (locals.var_guard398 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 == 0.0)) && (locals.var_guard395 != 0.0)) && (locals.var_guard396 != 0.0)) && (locals.var_guard398 == 0.0)) {
            let assign11270_e16010: f64 = (p.p374 * locals.var_dmcgeff);
            let assign11270_e16013: f64 = (locals.var_weff * locals.var_nuendd);
            let assign11270_e16014: f64 = (assign11270_e16010 / assign11270_e16013);
            locals.var_rend = assign11270_e16014;
            locals.var_rend_rv = 0.0;
        }

        let assign11290_e16026: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard400 = assign11290_e16026;
        locals.var_guard400_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 == 0.0)) && (locals.var_guard395 != 0.0)) && ((locals.var_guard397 != 0.0) && (locals.var_guard396 == 0.0))) && (locals.var_guard400 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_12(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let nv4 = ctx.node_voltage(nodes[4]);
        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 == 0.0)) && (locals.var_guard395 != 0.0)) && ((locals.var_guard397 != 0.0) && (locals.var_guard396 == 0.0))) && (locals.var_guard400 == 0.0)) {
            let assign11310_e16099: f64 = (p.p374 * locals.var_weff);
            let assign11310_e16102: f64 = (6.0 * locals.var_nuendd);
            let assign11310_e16104: f64 = (assign11310_e16102 * locals.var_dmcgeff);
            let assign11310_e16105: f64 = (assign11310_e16099 / assign11310_e16104);
            locals.var_rend = assign11310_e16105;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 == 0.0)) && (locals.var_guard395 != 0.0)) && (!((locals.var_guard396 != 0.0) || (locals.var_guard397 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        let assign11330_e16152: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard401 = assign11330_e16152;
        locals.var_guard401_rv = 0.0;

        let assign11340_e16163: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard402 = assign11340_e16163;
        locals.var_guard402_rv = 0.0;

        let assign11350_e16166: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard403 = assign11350_e16166;
        locals.var_guard403_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 == 0.0)) && (locals.var_guard395 == 0.0)) && (locals.var_guard401 != 0.0)) && (locals.var_guard403 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 == 0.0)) && (locals.var_guard395 == 0.0)) && (locals.var_guard401 != 0.0)) && (locals.var_guard403 == 0.0)) {
            let assign11370_e16235: f64 = (p.p374 * locals.var_dmcgeff);
            let assign11370_e16238: f64 = (locals.var_weff * locals.var_nuendd);
            let assign11370_e16239: f64 = (assign11370_e16235 / assign11370_e16238);
            locals.var_rend = assign11370_e16239;
            locals.var_rend_rv = 0.0;
        }

        let assign11390_e16251: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard405 = assign11390_e16251;
        locals.var_guard405_rv = 0.0;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 == 0.0)) && (locals.var_guard395 == 0.0)) && ((locals.var_guard402 != 0.0) && (locals.var_guard401 == 0.0))) && (locals.var_guard405 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 == 0.0)) && (locals.var_guard395 == 0.0)) && ((locals.var_guard402 != 0.0) && (locals.var_guard401 == 0.0))) && (locals.var_guard405 == 0.0)) {
            let assign11410_e16326: f64 = (p.p374 * locals.var_weff);
            let assign11410_e16329: f64 = (6.0 * locals.var_nuendd);
            let assign11410_e16331: f64 = (assign11410_e16329 * locals.var_dmcgeff);
            let assign11410_e16332: f64 = (assign11410_e16326 / assign11410_e16331);
            locals.var_rend = assign11410_e16332;
            locals.var_rend_rv = 0.0;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 == 0.0)) && (locals.var_guard395 == 0.0)) && (!((locals.var_guard401 != 0.0) || (locals.var_guard402 != 0.0)))) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if (((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard261 != 0.0) && (!((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0))))) {
            let assign11430_e16395: f64 = (p.p374 * locals.var_dmdgeff);
            let assign11430_e16397: f64 = (assign11430_e16395 / locals.var_weff);
            locals.var_rend = assign11430_e16397;
            locals.var_rend_rv = 0.0;
        }

        let assign11440_e16402: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard406 = assign11440_e16402;
        locals.var_guard406_rv = 0.0;

        if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard262 != 0.0) && (!(((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0))))) && (locals.var_guard406 != 0.0)) {
            let assign11450_e16432: f64 = (0.5 * p.p374);
            let assign11450_e16434: f64 = (assign11450_e16432 * locals.var_dmcgeff);
            let assign11450_e16436: f64 = (assign11450_e16434 / locals.var_weff);
            locals.var_rend = assign11450_e16436;
            locals.var_rend_rv = 0.0;
        }

        let assign11460_e16441: f64 = if p.p2 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard407 = assign11460_e16441;
        locals.var_guard407_rv = 0.0;

        if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard262 != 0.0) && (!(((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0))))) && (locals.var_guard406 != 0.0)) && (locals.var_guard407 != 0.0)) {
            locals.var_rint = 0.0;
            locals.var_rint_rv = 0.0;
        }

        if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard262 != 0.0) && (!(((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0))))) && (locals.var_guard406 != 0.0)) && (locals.var_guard407 == 0.0)) {
            let assign11480_e16506: f64 = (p.p374 * locals.var_dmcgeff);
            let assign11480_e16510: f64 = (p.p2 - 2.0);
            let assign11480_e16511: f64 = (locals.var_weff * assign11480_e16510);
            let assign11480_e16512: f64 = (assign11480_e16506 / assign11480_e16511);
            locals.var_rint = assign11480_e16512;
            locals.var_rint_rv = 0.0;
        }

        if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard262 != 0.0) && (!(((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0))))) && (locals.var_guard406 == 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard262 != 0.0) && (!(((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0))))) && (locals.var_guard406 == 0.0)) {
            let assign11500_e16576: f64 = (p.p374 * locals.var_dmcgeff);
            let assign11500_e16579: f64 = (locals.var_weff * p.p2);
            let assign11500_e16580: f64 = (assign11500_e16576 / assign11500_e16579);
            locals.var_rint = assign11500_e16580;
            locals.var_rint_rv = 0.0;
        }

        let assign11510_e16585: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard408 = assign11510_e16585;
        locals.var_guard408_rv = 0.0;

        if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard263 != 0.0) && (!((((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0))))) && (locals.var_guard408 != 0.0)) {
            locals.var_rend = 0.0;
            locals.var_rend_rv = 0.0;
        }

        if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard263 != 0.0) && (!((((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0))))) && (locals.var_guard408 != 0.0)) {
            let assign11530_e16649: f64 = (p.p374 * locals.var_dmcgeff);
            let assign11530_e16652: f64 = (locals.var_weff * p.p2);
            let assign11530_e16653: f64 = (assign11530_e16649 / assign11530_e16652);
            locals.var_rint = assign11530_e16653;
            locals.var_rint_rv = 0.0;
        }

        if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard263 != 0.0) && (!((((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0))))) && (locals.var_guard408 == 0.0)) {
            let assign11540_e16688: f64 = (0.5 * p.p374);
            let assign11540_e16690: f64 = (assign11540_e16688 * locals.var_dmcgeff);
            let assign11540_e16692: f64 = (assign11540_e16690 / locals.var_weff);
            locals.var_rend = assign11540_e16692;
            locals.var_rend_rv = 0.0;
        }

        let assign11550_e16697: f64 = if p.p2 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard409 = assign11550_e16697;
        locals.var_guard409_rv = 0.0;

        if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard263 != 0.0) && (!((((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0))))) && (locals.var_guard408 == 0.0)) && (locals.var_guard409 != 0.0)) {
            locals.var_rint = 0.0;
            locals.var_rint_rv = 0.0;
        }

        if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard263 != 0.0) && (!((((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0))))) && (locals.var_guard408 == 0.0)) && (locals.var_guard409 == 0.0)) {
            let assign11570_e16768: f64 = (p.p374 * locals.var_dmcgeff);
            let assign11570_e16772: f64 = (p.p2 - 2.0);
            let assign11570_e16773: f64 = (locals.var_weff * assign11570_e16772);
            let assign11570_e16774: f64 = (assign11570_e16768 / assign11570_e16773);
            locals.var_rint = assign11570_e16774;
            locals.var_rint_rv = 0.0;
        }

        if (((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (!(((((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0)) || (locals.var_guard263 != 0.0)))) {
            locals.var_rint = 0.0;
            locals.var_rint_rv = 0.0;
        }

        let assign11590_e16809: f64 = if locals.var_rint <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard410 = assign11590_e16809;
        locals.var_guard410_rv = 0.0;

        if (((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard410 != 0.0)) {
            locals.var_rdraingeo = locals.var_rend;
            locals.var_rdraingeo_rv = 0.0;
        }

        let assign11610_e16821: f64 = if locals.var_rend <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard411 = assign11610_e16821;
        locals.var_guard411_rv = 0.0;

        if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard410 == 0.0)) && (locals.var_guard411 != 0.0)) {
            locals.var_rdraingeo = locals.var_rint;
            locals.var_rdraingeo_rv = 0.0;
        }

        if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard410 == 0.0)) && (locals.var_guard411 == 0.0)) {
            let assign11630_e16846: f64 = (locals.var_rint * locals.var_rend);
            let assign11630_e16849: f64 = (locals.var_rint + locals.var_rend);
            let assign11630_e16850: f64 = (assign11630_e16846 / assign11630_e16849);
            locals.var_rdraingeo = assign11630_e16850;
            locals.var_rdraingeo_rv = 0.0;
        }

        if ((locals.var_guard245 == 0.0) && (locals.var_guard246 == 0.0)) {
            locals.var_rdraingeo = 0.0;
            locals.var_rdraingeo_rv = 0.0;
        }

        let assign11660_e16866: f64 = if p.p42 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard413 = assign11660_e16866;
        locals.var_guard413_rv = 0.0;

        let assign11670_e16869: f64 = if locals.var_rsourcegeo < p.p1093 { 1.0 } else { 0.0 };
        locals.var_guard414 = assign11670_e16869;
        locals.var_guard414_rv = 0.0;

        if ((locals.var_guard413 != 0.0) && (locals.var_guard414 != 0.0)) {
            locals.var_rsourcegeo = 0.0;
            locals.var_rsourcegeo_rv = 0.0;
        }

        let assign11690_e16878: f64 = if locals.var_rdraingeo < p.p1093 { 1.0 } else { 0.0 };
        locals.var_guard415 = assign11690_e16878;
        locals.var_guard415_rv = 0.0;

        if ((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) {
            locals.var_rdraingeo = 0.0;
            locals.var_rdraingeo_rv = 0.0;
        }

        let assign11710_e16887: f64 = if locals.var_rsourcegeo <= p.p1093 { 1.0 } else { 0.0 };
        locals.var_guard416 = assign11710_e16887;
        locals.var_guard416_rv = 0.0;

        if ((locals.var_guard413 == 0.0) && (locals.var_guard416 != 0.0)) {
            locals.var_rsourcegeo = p.p1093;
            locals.var_rsourcegeo_rv = 0.0;
        }

        let assign11730_e16897: f64 = if locals.var_rdraingeo <= p.p1093 { 1.0 } else { 0.0 };
        locals.var_guard417 = assign11730_e16897;
        locals.var_guard417_rv = 0.0;

        if ((locals.var_guard413 == 0.0) && (locals.var_guard417 != 0.0)) {
            locals.var_rdraingeo = p.p1093;
            locals.var_rdraingeo_rv = 0.0;
        }

        let assign11750_e16907: f64 = if p.p42 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard418 = assign11750_e16907;
        locals.var_guard418_rv = 0.0;

        let assign11760_e16910: f64 = if locals.var_rswmin_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard419 = assign11760_e16910;
        locals.var_guard419_rv = 0.0;

        if ((locals.var_guard418 != 0.0) && (locals.var_guard419 != 0.0)) {
            locals.var_rswmin_i = 0.0;
            locals.var_rswmin_i_rv = 0.0;
        }

        let assign11780_e16919: f64 = if locals.var_rdwmin_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard420 = assign11780_e16919;
        locals.var_guard420_rv = 0.0;

        if ((locals.var_guard418 != 0.0) && (locals.var_guard420 != 0.0)) {
            locals.var_rdwmin_i = 0.0;
            locals.var_rdwmin_i_rv = 0.0;
        }

        let assign11800_e16928: f64 = if locals.var_rsw_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard421 = assign11800_e16928;
        locals.var_guard421_rv = 0.0;

        if ((locals.var_guard418 != 0.0) && (locals.var_guard421 != 0.0)) {
            locals.var_rsw_i = 0.0;
            locals.var_rsw_i_rv = 0.0;
        }

        let assign11820_e16937: f64 = if locals.var_rdw_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard422 = assign11820_e16937;
        locals.var_guard422_rv = 0.0;

        if ((locals.var_guard418 != 0.0) && (locals.var_guard422 != 0.0)) {
            locals.var_rdw_i = 0.0;
            locals.var_rdw_i_rv = 0.0;
        }

        let assign11840_e16946: f64 = if locals.var_rdswmin_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard423 = assign11840_e16946;
        locals.var_guard423_rv = 0.0;

        if ((locals.var_guard418 == 0.0) && (locals.var_guard423 != 0.0)) {
            locals.var_rdswmin_i = 0.0;
            locals.var_rdswmin_i_rv = 0.0;
        }

        let assign11860_e16956: f64 = if locals.var_rdsw_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard424 = assign11860_e16956;
        locals.var_guard424_rv = 0.0;

        if ((locals.var_guard418 == 0.0) && (locals.var_guard424 != 0.0)) {
            locals.var_rdsw_i = 0.0;
            locals.var_rdsw_i_rv = 0.0;
        }

        let assign12580_e17615: f64 = if p.p1097 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard443 = assign12580_e17615;
        locals.var_guard443_rv = 0.0;

        if (locals.var_guard443 != 0.0) {
            let assign12620_e17639: f64 = (1.0 - p.p1128);
            locals.var_oneminusxpart = assign12620_e17639;
            locals.var_oneminusxpart_rv = 0.0;
        }

        if (locals.var_guard443 == 0.0) {
            locals.var_oneminusxpart = 1.0;
            locals.var_oneminusxpart_rv = 0.0;
        }

        let assign12640_e17651: f64 = (locals.var_weffcj / 3.0);
        let assign12640_e17653: f64 = (assign12640_e17651 / p.p32);
        let assign12640_e17654: f64 = (p.p31 + assign12640_e17653);
        let assign12640_e17655: f64 = (p.p700 * assign12640_e17654);
        let assign12640_e17658: f64 = (p.p32 * p.p2);
        let assign12640_e17661: f64 = (locals.var_lnew - p.p699);
        let assign12640_e17662: f64 = (assign12640_e17658 * assign12640_e17661);
        let assign12640_e17663: f64 = (assign12640_e17655 / assign12640_e17662);
        locals.var_grgeltd = assign12640_e17663;
        locals.var_grgeltd_rv = 0.0;

        let assign12650_e17666: f64 = if locals.var_grgeltd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard445 = assign12650_e17666;
        locals.var_guard445_rv = 0.0;

        if (locals.var_guard445 != 0.0) {
            let assign12660_e17670: f64 = (1.0 / locals.var_grgeltd);
            locals.var_grgeltd = assign12660_e17670;
            locals.var_grgeltd_rv = 0.0;
        }

        if (locals.var_guard445 == 0.0) {
            locals.var_grgeltd = 1000.0;
            locals.var_grgeltd_rv = 0.0;
        }

        let assign12690_e17683: f64 = (p.p77 * p.p77);
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign12690_e17683, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_t0_rv = 0.0;

        let assign12700_e17686: f64 = (p.p77 * locals.var_poxedge_i);
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, ) = (assign12700_e17686, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_t1_rv = 0.0;

        let assign12710_e17689: f64 = (locals.var_t1 * locals.var_t1);
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14, ) = (assign12710_e17689, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)), ((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12)), ((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)), ((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)), );
        locals.var_t2_rv = 0.0;

        let (assign12750_e17723,) = {
    if (p.p39 == 1.0) {
        (745669000000.0,)
    } else {
        (1166450000000.0,)
    }
};
        locals.var_bechvb = assign12750_e17723;
        locals.var_bechvb_rv = 0.0;

        let assign12770_e17730: f64 = (-locals.var_bechvb);
        let assign12770_e17732: f64 = (assign12770_e17730 * p.p77);
        let assign12770_e17734: f64 = (assign12770_e17732 * locals.var_poxedge_i);
        locals.var_bechvbedge = assign12770_e17734;
        locals.var_bechvbedge_rv = 0.0;

        let assign12790_e17743: f64 = (-locals.var_bechvb);
        let assign12790_e17745: f64 = (assign12790_e17743 * p.p77);
        locals.var_bechvb = assign12790_e17745;
        locals.var_bechvb_rv = 0.0;

        let assign12800_e17748: f64 = (p.p911 + locals.var_weff);
        locals.var_weff_sh = assign12800_e17748;
        locals.var_weff_sh_rv = 0.0;

        let assign12810_e17759: f64 = if (((p.p49 != 0.0) && (p.p909 > 0.0)) && (locals.var_weff_sh > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard447 = assign12810_e17759;
        locals.var_guard447_rv = 0.0;

        if (locals.var_guard447 != 0.0) {
            let assign12820_e17763: f64 = (locals.var_weff_sh * p.p2);
            let assign12820_e17765: f64 = (assign12820_e17763 / p.p909);
            locals.var_gth = assign12820_e17765;
            locals.var_gth_rv = 0.0;
        }

        if (locals.var_guard447 != 0.0) {
            let assign12830_e17771: f64 = (p.p910 * locals.var_weff_sh);
            let assign12830_e17773: f64 = (assign12830_e17771 * p.p2);
            locals.var_cth = assign12830_e17773;
            locals.var_cth_rv = 0.0;
        }

        if (locals.var_guard447 == 0.0) {
            locals.var_gth = 1.0;
            locals.var_gth_rv = 0.0;
            locals.var_cth = 0.0;
            locals.var_cth_rv = 0.0;
        }

        let assign12860_e17788: f64 = (-273.15);
        let assign12860_e17789: f64 = if p.p820 <= assign12860_e17788 { 1.0 } else { 0.0 };
        locals.var_guard448 = assign12860_e17789;
        locals.var_guard448_rv = 0.0;

        if (locals.var_guard448 != 0.0) {
            let assign12870_e17793: f64 = (300.15 - 273.15);
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign12870_e17793, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t0_rv = 0.0;
        }

        if (locals.var_guard448 != 0.0) {
            locals.var_tnom = 300.15;
            locals.var_tnom_rv = 0.0;
        }

        if (locals.var_guard448 == 0.0) {
            let assign12890_e17804: f64 = (p.p820 + 273.15);
            locals.var_tnom = assign12890_e17804;
            locals.var_tnom_rv = 0.0;
        }

        let assign12900_e17807: f64 = ctx_temp;
        let assign12900_e17809: f64 = (assign12900_e17807 + p.p33);
        (locals.var_devtemp, locals.var_devtemp_dn4, ) = (assign12900_e17809, 0.0, );
        locals.var_devtemp_rv = 0.0;

        let assign12910_e17820: f64 = if (((p.p49 != 0.0) && (p.p909 > 0.0)) && (locals.var_weff_sh > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard449 = assign12910_e17820;
        locals.var_guard449_rv = 0.0;

        if (locals.var_guard449 != 0.0) {
            (locals.var_deltemp1, locals.var_deltemp1_dn4, ) = ((nv4 - 0.0), 1.0, );
            locals.var_deltemp1_rv = 0.0;
        }

        if (locals.var_guard449 == 0.0) {
            (locals.var_deltemp1, locals.var_deltemp1_dn4, ) = (0.0, 0.0, );
            locals.var_deltemp1_rv = 0.0;
        }

        let assign12940_e17832: f64 = (locals.var_deltemp1 + locals.var_devtemp);
        (locals.var_devtemp, locals.var_devtemp_dn4, ) = (assign12940_e17832, (locals.var_deltemp1_dn4 + locals.var_devtemp_dn4), );
        locals.var_devtemp_rv = 0.0;

        let assign12980_e17840: f64 = (8.617087e-5 * locals.var_devtemp);
        (locals.var_vt, locals.var_vt_dn4, ) = (assign12980_e17840, (8.617087e-5 * locals.var_devtemp_dn4), );
        locals.var_vt_rv = 0.0;

        let assign12990_e17843: f64 = (1.0 / locals.var_vt);
        (locals.var_inv_vt, locals.var_inv_vt_dn4, ) = (assign12990_e17843, (-(locals.var_vt_dn4 / (locals.var_vt * locals.var_vt))), );
        locals.var_inv_vt_rv = 0.0;

        let assign13000_e17846: f64 = (locals.var_devtemp / locals.var_tnom);
        (locals.var_tratio, locals.var_tratio_dn4, ) = (assign13000_e17846, (locals.var_devtemp_dn4 / locals.var_tnom), );
        locals.var_tratio_rv = 0.0;

        let assign13010_e17849: f64 = (locals.var_devtemp - locals.var_tnom);
        (locals.var_deltemp, locals.var_deltemp_dn4, ) = (assign13010_e17849, locals.var_devtemp_dn4, );
        locals.var_deltemp_rv = 0.0;

        let assign13020_e17852: f64 = (8.617087e-5 * locals.var_devtemp);
        (locals.var_vtm, locals.var_vtm_dn4, ) = (assign13020_e17852, (8.617087e-5 * locals.var_devtemp_dn4), );
        locals.var_vtm_rv = 0.0;

        let assign13030_e17855: f64 = (8.617087e-5 * locals.var_tnom);
        locals.var_vtm0 = assign13030_e17855;
        locals.var_vtm0_rv = 0.0;

        let assign13040_e17859: f64 = (p.p821 * locals.var_devtemp);
        let assign13040_e17861: f64 = (assign13040_e17859 * locals.var_devtemp);
        let assign13040_e17864: f64 = (locals.var_devtemp + p.p822);
        let assign13040_e17865: f64 = (assign13040_e17861 / assign13040_e17864);
        let assign13040_e17866: f64 = (p.p109 - assign13040_e17865);
        (locals.var_eg, locals.var_eg_dn4, ) = (assign13040_e17866, (-((((((p.p821 * locals.var_devtemp_dn4) * locals.var_devtemp) + (assign13040_e17859 * locals.var_devtemp_dn4)) * assign13040_e17864) - (assign13040_e17861 * locals.var_devtemp_dn4)) / (assign13040_e17864 * assign13040_e17864))), );
        locals.var_eg_rv = 0.0;

        let assign13050_e17870: f64 = (p.p821 * locals.var_tnom);
        let assign13050_e17872: f64 = (assign13050_e17870 * locals.var_tnom);
        let assign13050_e17875: f64 = (locals.var_tnom + p.p822);
        let assign13050_e17876: f64 = (assign13050_e17872 / assign13050_e17875);
        let assign13050_e17877: f64 = (p.p109 - assign13050_e17876);
        locals.var_eg0 = assign13050_e17877;
        locals.var_eg0_rv = 0.0;

        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_tnom;
        let assign13060_e17880: f64 = (locals.var_devtemp * __rspice_inv_cse_0);
        let assign13060_e17883: f64 = (locals.var_devtemp * __rspice_inv_cse_0);
        let assign13060_e17884: f64 = (assign13060_e17883).sqrt();
        let assign13060_e17885: f64 = (assign13060_e17880 * assign13060_e17884);
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, ) = (assign13060_e17885, 0.0, 0.0, 0.0, (((locals.var_devtemp_dn4 / locals.var_tnom) * assign13060_e17884) + (assign13060_e17880 * ((locals.var_devtemp_dn4 / locals.var_tnom) / (2.0 * assign13060_e17884)))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_t1_rv = 0.0;

        let assign13070_e17888: f64 = (p.p108 * locals.var_t1);
        let assign13070_e17892: f64 = (2.0 * locals.var_vtm0);
        let assign13070_e17893: f64 = (locals.var_eg / assign13070_e17892);
        let assign13070_e17897: f64 = (2.0 * locals.var_vtm);
        let assign13070_e17898: f64 = (locals.var_eg / assign13070_e17897);
        let assign13070_e17899: f64 = (assign13070_e17893 - assign13070_e17898);
        let assign13070_e17900: f64 = { let limited_exp_arg = assign13070_e17899; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13070_e17901: f64 = (assign13070_e17888 * assign13070_e17900);
        (locals.var_ni, locals.var_ni_dn0, locals.var_ni_dn2, locals.var_ni_dn3, locals.var_ni_dn4, locals.var_ni_dn5, locals.var_ni_dn6, locals.var_ni_dn7, locals.var_ni_dn8, locals.var_ni_dn9, locals.var_ni_dn10, locals.var_ni_dn11, locals.var_ni_dn12, locals.var_ni_dn13, locals.var_ni_dn14, ) = (assign13070_e17901, ((p.p108 * locals.var_t1_dn0) * assign13070_e17900), ((p.p108 * locals.var_t1_dn2) * assign13070_e17900), ((p.p108 * locals.var_t1_dn3) * assign13070_e17900), (((p.p108 * locals.var_t1_dn4) * assign13070_e17900) + (assign13070_e17888 * ({ let limited_exp_arg = assign13070_e17899; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_eg_dn4 / assign13070_e17892) - (((locals.var_eg_dn4 * assign13070_e17897) - (locals.var_eg * (2.0 * locals.var_vtm_dn4))) / (assign13070_e17897 * assign13070_e17897)))))), ((p.p108 * locals.var_t1_dn5) * assign13070_e17900), ((p.p108 * locals.var_t1_dn6) * assign13070_e17900), ((p.p108 * locals.var_t1_dn7) * assign13070_e17900), ((p.p108 * locals.var_t1_dn8) * assign13070_e17900), ((p.p108 * locals.var_t1_dn9) * assign13070_e17900), ((p.p108 * locals.var_t1_dn10) * assign13070_e17900), ((p.p108 * locals.var_t1_dn11) * assign13070_e17900), ((p.p108 * locals.var_t1_dn12) * assign13070_e17900), ((p.p108 * locals.var_t1_dn13) * assign13070_e17900), ((p.p108 * locals.var_t1_dn14) * assign13070_e17900), );
        locals.var_ni_rv = 0.0;

        let assign13080_e17912: f64 = if (((p.p49 != 0.0) && (p.p909 > 0.0)) && (locals.var_weff_sh > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard450 = assign13080_e17912;
        locals.var_guard450_rv = 0.0;

        if (locals.var_guard450 != 0.0) {
            let assign13090_e17916: f64 = (locals.var_ndep_i / locals.var_ni);
            let assign13090_e17918: f64 = (assign13090_e17916).max(1e-38);
            let assign13090_e17919: f64 = (assign13090_e17918).ln();
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign13090_e17919, (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn0 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn0)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn2 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn2)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn3 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn3)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn4 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn4)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn5 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn5)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn6 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn6)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn7 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn7)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn8 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn8)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn9 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn9)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn10 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn10)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn11 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn11)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn12 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn12)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn13 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn13)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn14 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn14)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), );
            locals.var_t0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_13(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard450 != 0.0) {
            let assign13100_e17925: f64 = (locals.var_t0 * locals.var_t0);
            let assign13100_e17927: f64 = (assign13100_e17925 + 1e-6);
            let assign13100_e17928: f64 = (assign13100_e17927).sqrt();
            (locals.var_phib, locals.var_phib_dn0, locals.var_phib_dn2, locals.var_phib_dn3, locals.var_phib_dn4, locals.var_phib_dn5, locals.var_phib_dn6, locals.var_phib_dn7, locals.var_phib_dn8, locals.var_phib_dn9, locals.var_phib_dn10, locals.var_phib_dn11, locals.var_phib_dn12, locals.var_phib_dn13, locals.var_phib_dn14, ) = (assign13100_e17928, (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn12 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn12)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)) / (2.0 * assign13100_e17928)), );
            locals.var_phib_rv = 0.0;
        }

        if (locals.var_guard450 == 0.0) {
            let assign13110_e17935: f64 = (locals.var_ndep_i / locals.var_ni);
            let assign13110_e17937: f64 = (assign13110_e17935).max(1e-38);
            let assign13110_e17938: f64 = (assign13110_e17937).ln();
            (locals.var_phib, locals.var_phib_dn0, locals.var_phib_dn2, locals.var_phib_dn3, locals.var_phib_dn4, locals.var_phib_dn5, locals.var_phib_dn6, locals.var_phib_dn7, locals.var_phib_dn8, locals.var_phib_dn9, locals.var_phib_dn10, locals.var_phib_dn11, locals.var_phib_dn12, locals.var_phib_dn13, locals.var_phib_dn14, ) = (assign13110_e17938, (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn0 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn0)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn2 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn2)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn3 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn3)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn4 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn4)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn5 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn5)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn6 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn6)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn7 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn7)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn8 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn8)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn9 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn9)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn10 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn10)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn11 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn11)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn12 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn12)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn13 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn13)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn14 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn14)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), );
            locals.var_phib_rv = 0.0;
        }

        let assign13120_e17951: f64 = if (((p.p49 != 0.0) && (p.p909 > 0.0)) && (locals.var_weff_sh > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard451 = assign13120_e17951;
        locals.var_guard451_rv = 0.0;

        if (locals.var_guard451 != 0.0) {
            let assign13130_e17955: f64 = (locals.var_ndepedge_i * locals.var_nsd_i);
            let assign13130_e17958: f64 = (locals.var_ni * locals.var_ni);
            let assign13130_e17959: f64 = (assign13130_e17955 / assign13130_e17958);
            let assign13130_e17961: f64 = (assign13130_e17959).max(1e-38);
            let assign13130_e17962: f64 = (assign13130_e17961).ln();
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign13130_e17962, (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn0 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn0))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn2 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn2))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn3 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn3))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn4 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn4))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn5 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn5))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn6 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn6))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn7 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn7))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn8 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn8))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn9 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn9))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn10 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn10))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn11 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn11))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn12 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn12))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn13 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn13))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn14 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn14))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), );
            locals.var_t0_rv = 0.0;
        }

        if (locals.var_guard451 != 0.0) {
            let assign13140_e17968: f64 = (locals.var_t0 * locals.var_t0);
            let assign13140_e17970: f64 = (assign13140_e17968 + 1e-6);
            let assign13140_e17971: f64 = (assign13140_e17970).sqrt();
            (locals.var_vbi_edge, locals.var_vbi_edge_dn0, locals.var_vbi_edge_dn2, locals.var_vbi_edge_dn3, locals.var_vbi_edge_dn4, locals.var_vbi_edge_dn5, locals.var_vbi_edge_dn6, locals.var_vbi_edge_dn7, locals.var_vbi_edge_dn8, locals.var_vbi_edge_dn9, locals.var_vbi_edge_dn10, locals.var_vbi_edge_dn11, locals.var_vbi_edge_dn12, locals.var_vbi_edge_dn13, locals.var_vbi_edge_dn14, ) = (assign13140_e17971, (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn12 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn12)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)) / (2.0 * assign13140_e17971)), );
            locals.var_vbi_edge_rv = 0.0;
        }

        if (locals.var_guard451 == 0.0) {
            let assign13150_e17978: f64 = (locals.var_ndepedge_i * locals.var_nsd_i);
            let assign13150_e17981: f64 = (locals.var_ni * locals.var_ni);
            let assign13150_e17982: f64 = (assign13150_e17978 / assign13150_e17981);
            let assign13150_e17984: f64 = (assign13150_e17982).max(1e-38);
            let assign13150_e17985: f64 = (assign13150_e17984).ln();
            (locals.var_vbi_edge, locals.var_vbi_edge_dn0, locals.var_vbi_edge_dn2, locals.var_vbi_edge_dn3, locals.var_vbi_edge_dn4, locals.var_vbi_edge_dn5, locals.var_vbi_edge_dn6, locals.var_vbi_edge_dn7, locals.var_vbi_edge_dn8, locals.var_vbi_edge_dn9, locals.var_vbi_edge_dn10, locals.var_vbi_edge_dn11, locals.var_vbi_edge_dn12, locals.var_vbi_edge_dn13, locals.var_vbi_edge_dn14, ) = (assign13150_e17985, (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn0 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn0))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn2 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn2))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn3 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn3))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn4 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn4))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn5 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn5))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn6 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn6))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn7 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn7))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn8 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn8))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn9 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn9))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn10 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn10))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn11 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn11))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn12 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn12))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn13 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn13))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn14 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn14))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), );
            locals.var_vbi_edge_rv = 0.0;
        }

        let assign13160_e17990: f64 = if locals.var_ngate_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard452 = assign13160_e17990;
        locals.var_guard452_rv = 0.0;

        if (locals.var_guard452 != 0.0) {
            let assign13170_e17993: f64 = (-locals.var_devsign);
            let assign13170_e17995: f64 = (assign13170_e17993 * locals.var_vt);
            let assign13170_e17998: f64 = (locals.var_ngate_i / locals.var_nsd_i);
            let assign13170_e18000: f64 = (assign13170_e17998).max(1e-38);
            let assign13170_e18001: f64 = (assign13170_e18000).ln();
            let assign13170_e18002: f64 = (assign13170_e17995 * assign13170_e18001);
            let assign13170_e18004: f64 = (assign13170_e18002 + p.p5);
            (locals.var_vfbsdr, locals.var_vfbsdr_dn4, ) = (assign13170_e18004, ((assign13170_e17993 * locals.var_vt_dn4) * assign13170_e18001), );
            locals.var_vfbsdr_rv = 0.0;
        }

        if (locals.var_guard452 == 0.0) {
            (locals.var_vfbsdr, locals.var_vfbsdr_dn4, ) = (0.0, 0.0, );
            locals.var_vfbsdr_rv = 0.0;
        }

        let assign13190_e18015: f64 = (locals.var_vt * locals.var_phib);
        let assign13190_e18016: f64 = (0.4 + assign13190_e18015);
        let assign13190_e18018: f64 = (assign13190_e18016 + locals.var_phin_i);
        let assign13190_e18020: f64 = (assign13190_e18018).max(0.4);
        (locals.var_phist, locals.var_phist_dn0, locals.var_phist_dn2, locals.var_phist_dn3, locals.var_phist_dn4, locals.var_phist_dn5, locals.var_phist_dn6, locals.var_phist_dn7, locals.var_phist_dn8, locals.var_phist_dn9, locals.var_phist_dn10, locals.var_phist_dn11, locals.var_phist_dn12, locals.var_phist_dn13, locals.var_phist_dn14, ) = (assign13190_e18020, if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn0) } else { 0.0 }, if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn2) } else { 0.0 }, if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn3) } else { 0.0 }, if assign13190_e18018 >= 0.4 { ((locals.var_vt_dn4 * locals.var_phib) + (locals.var_vt * locals.var_phib_dn4)) } else { 0.0 }, if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn5) } else { 0.0 }, if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn6) } else { 0.0 }, if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn7) } else { 0.0 }, if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn8) } else { 0.0 }, if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn9) } else { 0.0 }, if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn10) } else { 0.0 }, if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn11) } else { 0.0 }, if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn12) } else { 0.0 }, if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn13) } else { 0.0 }, if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn14) } else { 0.0 }, );
        locals.var_phist_rv = 0.0;

        let assign13200_e18022: f64 = (locals.var_phist).sqrt();
        (locals.var_sqrtphist, locals.var_sqrtphist_dn0, locals.var_sqrtphist_dn2, locals.var_sqrtphist_dn3, locals.var_sqrtphist_dn4, locals.var_sqrtphist_dn5, locals.var_sqrtphist_dn6, locals.var_sqrtphist_dn7, locals.var_sqrtphist_dn8, locals.var_sqrtphist_dn9, locals.var_sqrtphist_dn10, locals.var_sqrtphist_dn11, locals.var_sqrtphist_dn12, locals.var_sqrtphist_dn13, locals.var_sqrtphist_dn14, ) = (assign13200_e18022, (locals.var_phist_dn0 / (2.0 * assign13200_e18022)), (locals.var_phist_dn2 / (2.0 * assign13200_e18022)), (locals.var_phist_dn3 / (2.0 * assign13200_e18022)), (locals.var_phist_dn4 / (2.0 * assign13200_e18022)), (locals.var_phist_dn5 / (2.0 * assign13200_e18022)), (locals.var_phist_dn6 / (2.0 * assign13200_e18022)), (locals.var_phist_dn7 / (2.0 * assign13200_e18022)), (locals.var_phist_dn8 / (2.0 * assign13200_e18022)), (locals.var_phist_dn9 / (2.0 * assign13200_e18022)), (locals.var_phist_dn10 / (2.0 * assign13200_e18022)), (locals.var_phist_dn11 / (2.0 * assign13200_e18022)), (locals.var_phist_dn12 / (2.0 * assign13200_e18022)), (locals.var_phist_dn13 / (2.0 * assign13200_e18022)), (locals.var_phist_dn14 / (2.0 * assign13200_e18022)), );
        locals.var_sqrtphist_rv = 0.0;

        let assign13210_e18025: f64 = (2.0 * locals.var_epssi);
        let assign13210_e18028: f64 = (1.60219e-19 * locals.var_ndep_i);
        let assign13210_e18029: f64 = (assign13210_e18025 / assign13210_e18028);
        let assign13210_e18030: f64 = (assign13210_e18029).sqrt();
        (locals.var_t1dep, locals.var_t1dep_dn0, locals.var_t1dep_dn2, locals.var_t1dep_dn3, locals.var_t1dep_dn4, locals.var_t1dep_dn5, locals.var_t1dep_dn6, locals.var_t1dep_dn7, locals.var_t1dep_dn8, locals.var_t1dep_dn9, locals.var_t1dep_dn10, locals.var_t1dep_dn11, locals.var_t1dep_dn12, locals.var_t1dep_dn13, locals.var_t1dep_dn14, ) = (assign13210_e18030, ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn0)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030)), ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn2)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030)), ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn3)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030)), ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn4)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030)), ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn5)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030)), ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn6)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030)), ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn7)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030)), ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn8)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030)), ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn9)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030)), ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn10)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030)), ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn11)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030)), ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn12)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030)), ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn13)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030)), ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn14)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030)), );
        locals.var_t1dep_rv = 0.0;

        let assign13220_e18033: f64 = (locals.var_epssi / locals.var_epsox);
        let assign13220_e18035: f64 = (assign13220_e18033 * p.p77);
        let assign13220_e18037: f64 = (assign13220_e18035 * locals.var_xj_i);
        let assign13220_e18038: f64 = (assign13220_e18037).sqrt();
        locals.var_litl = assign13220_e18038;
        locals.var_litl_rv = 0.0;

        let assign13230_e18044: f64 = (locals.var_tratio - 1.0);
        let assign13230_e18045: f64 = (p.p823 * assign13230_e18044);
        let assign13230_e18046: f64 = (1.0 + assign13230_e18045);
        let assign13230_e18048: f64 = (-10000.0);
        let assign13230_e18050: f64 = (assign13230_e18048 * 0.001);
        let (assign13230_e18111, assign13230_e18111_d_n4,) = {
    if (!(assign13230_e18046 < assign13230_e18050)) {
        let assign13230_e18058: f64 = (locals.var_tratio - 1.0);
        let assign13230_e18059: f64 = (p.p823 * assign13230_e18058);
        let assign13230_e18060: f64 = (1.0 + assign13230_e18059);
        let assign13230_e18065: f64 = (locals.var_tratio - 1.0);
        let assign13230_e18066: f64 = (p.p823 * assign13230_e18065);
        let assign13230_e18067: f64 = (1.0 + assign13230_e18066);
        let assign13230_e18072: f64 = (locals.var_tratio - 1.0);
        let assign13230_e18073: f64 = (p.p823 * assign13230_e18072);
        let assign13230_e18074: f64 = (1.0 + assign13230_e18073);
        let assign13230_e18075: f64 = (assign13230_e18067 * assign13230_e18074);
        let assign13230_e18078: f64 = (4.0 * 0.001);
        let assign13230_e18080: f64 = (assign13230_e18078 * 0.001);
        let assign13230_e18081: f64 = (assign13230_e18075 + assign13230_e18080);
        let assign13230_e18082: f64 = (assign13230_e18081).sqrt();
        let assign13230_e18083: f64 = (assign13230_e18060 + assign13230_e18082);
        let assign13230_e18084: f64 = (0.5 * assign13230_e18083);
        (assign13230_e18084, (0.5 * ((p.p823 * locals.var_tratio_dn4) + ((((p.p823 * locals.var_tratio_dn4) * assign13230_e18074) + (assign13230_e18067 * (p.p823 * locals.var_tratio_dn4))) / (2.0 * assign13230_e18082)))),)
    } else {
        let assign13230_e18089: f64 = (locals.var_tratio - 1.0);
        let assign13230_e18090: f64 = (p.p823 * assign13230_e18089);
        let assign13230_e18091: f64 = (1.0 + assign13230_e18090);
        let assign13230_e18093: f64 = (-10000.0);
        let assign13230_e18095: f64 = (assign13230_e18093 * 0.001);
        let (assign13230_e18110, assign13230_e18110_d_n4,) = {
            if (assign13230_e18091 < assign13230_e18095) {
                let assign13230_e18098: f64 = (-0.001);
                let assign13230_e18100: f64 = (assign13230_e18098 * 0.001);
                let assign13230_e18105: f64 = (locals.var_tratio - 1.0);
                let assign13230_e18106: f64 = (p.p823 * assign13230_e18105);
                let assign13230_e18107: f64 = (1.0 + assign13230_e18106);
                let assign13230_e18108: f64 = (assign13230_e18100 / assign13230_e18107);
                (assign13230_e18108, (-((assign13230_e18100 * (p.p823 * locals.var_tratio_dn4)) / (assign13230_e18107 * assign13230_e18107))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13230_e18110, assign13230_e18110_d_n4,)
    }
};
        let assign13230_e18112: f64 = (locals.var_nfactor_i * assign13230_e18111);
        (locals.var_nfactor_t, locals.var_nfactor_t_dn0, locals.var_nfactor_t_dn2, locals.var_nfactor_t_dn3, locals.var_nfactor_t_dn4, locals.var_nfactor_t_dn5, locals.var_nfactor_t_dn6, locals.var_nfactor_t_dn7, locals.var_nfactor_t_dn8, locals.var_nfactor_t_dn9, locals.var_nfactor_t_dn10, locals.var_nfactor_t_dn11, locals.var_nfactor_t_dn12, locals.var_nfactor_t_dn13, locals.var_nfactor_t_dn14, ) = (assign13230_e18112, (locals.var_nfactor_i_dn0 * assign13230_e18111), (locals.var_nfactor_i_dn2 * assign13230_e18111), (locals.var_nfactor_i_dn3 * assign13230_e18111), ((locals.var_nfactor_i_dn4 * assign13230_e18111) + (locals.var_nfactor_i * assign13230_e18111_d_n4)), (locals.var_nfactor_i_dn5 * assign13230_e18111), (locals.var_nfactor_i_dn6 * assign13230_e18111), (locals.var_nfactor_i_dn7 * assign13230_e18111), (locals.var_nfactor_i_dn8 * assign13230_e18111), (locals.var_nfactor_i_dn9 * assign13230_e18111), (locals.var_nfactor_i_dn10 * assign13230_e18111), (locals.var_nfactor_i_dn11 * assign13230_e18111), (locals.var_nfactor_i_dn12 * assign13230_e18111), (locals.var_nfactor_i_dn13 * assign13230_e18111), (locals.var_nfactor_i_dn14 * assign13230_e18111), );
        locals.var_nfactor_t_rv = 0.0;

        let assign13240_e18118: f64 = (locals.var_tratio - 1.0);
        let assign13240_e18119: f64 = (p.p851 * assign13240_e18118);
        let assign13240_e18120: f64 = (1.0 + assign13240_e18119);
        let assign13240_e18121: f64 = (locals.var_eta0_i * assign13240_e18120);
        (locals.var_eta0_t, locals.var_eta0_t_dn0, locals.var_eta0_t_dn2, locals.var_eta0_t_dn3, locals.var_eta0_t_dn4, locals.var_eta0_t_dn5, locals.var_eta0_t_dn6, locals.var_eta0_t_dn7, locals.var_eta0_t_dn8, locals.var_eta0_t_dn9, locals.var_eta0_t_dn10, locals.var_eta0_t_dn11, locals.var_eta0_t_dn12, locals.var_eta0_t_dn13, locals.var_eta0_t_dn14, ) = (assign13240_e18121, (locals.var_eta0_i_dn0 * assign13240_e18120), (locals.var_eta0_i_dn2 * assign13240_e18120), (locals.var_eta0_i_dn3 * assign13240_e18120), ((locals.var_eta0_i_dn4 * assign13240_e18120) + (locals.var_eta0_i * (p.p851 * locals.var_tratio_dn4))), (locals.var_eta0_i_dn5 * assign13240_e18120), (locals.var_eta0_i_dn6 * assign13240_e18120), (locals.var_eta0_i_dn7 * assign13240_e18120), (locals.var_eta0_i_dn8 * assign13240_e18120), (locals.var_eta0_i_dn9 * assign13240_e18120), (locals.var_eta0_i_dn10 * assign13240_e18120), (locals.var_eta0_i_dn11 * assign13240_e18120), (locals.var_eta0_i_dn12 * assign13240_e18120), (locals.var_eta0_i_dn13 * assign13240_e18120), (locals.var_eta0_i_dn14 * assign13240_e18120), );
        locals.var_eta0_t_rv = 0.0;

        let assign13250_e18124: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard453 = assign13250_e18124;
        locals.var_guard453_rv = 0.0;

        if (locals.var_guard453 != 0.0) {
            let assign13260_e18131: f64 = (locals.var_tratio - 1.0);
            let assign13260_e18132: f64 = (p.p851 * assign13260_e18131);
            let assign13260_e18133: f64 = (1.0 + assign13260_e18132);
            let assign13260_e18134: f64 = (locals.var_eta0r_i * assign13260_e18133);
            (locals.var_eta0r_t, locals.var_eta0r_t_dn0, locals.var_eta0r_t_dn2, locals.var_eta0r_t_dn3, locals.var_eta0r_t_dn4, locals.var_eta0r_t_dn5, locals.var_eta0r_t_dn6, locals.var_eta0r_t_dn7, locals.var_eta0r_t_dn8, locals.var_eta0r_t_dn9, locals.var_eta0r_t_dn10, locals.var_eta0r_t_dn11, locals.var_eta0r_t_dn12, locals.var_eta0r_t_dn13, locals.var_eta0r_t_dn14, ) = (assign13260_e18134, (locals.var_eta0r_i_dn0 * assign13260_e18133), (locals.var_eta0r_i_dn2 * assign13260_e18133), (locals.var_eta0r_i_dn3 * assign13260_e18133), ((locals.var_eta0r_i_dn4 * assign13260_e18133) + (locals.var_eta0r_i * (p.p851 * locals.var_tratio_dn4))), (locals.var_eta0r_i_dn5 * assign13260_e18133), (locals.var_eta0r_i_dn6 * assign13260_e18133), (locals.var_eta0r_i_dn7 * assign13260_e18133), (locals.var_eta0r_i_dn8 * assign13260_e18133), (locals.var_eta0r_i_dn9 * assign13260_e18133), (locals.var_eta0r_i_dn10 * assign13260_e18133), (locals.var_eta0r_i_dn11 * assign13260_e18133), (locals.var_eta0r_i_dn12 * assign13260_e18133), (locals.var_eta0r_i_dn13 * assign13260_e18133), (locals.var_eta0r_i_dn14 * assign13260_e18133), );
            locals.var_eta0r_t_rv = 0.0;
        }

        let (assign13270_e18146,) = {
    if (p.p39 != 1.0) {
        let assign13270_e18142: f64 = (0.3333333333333333 * p.p283);
        (assign13270_e18142,)
    } else {
        let assign13270_e18145: f64 = (0.5 * p.p283);
        (assign13270_e18145,)
    }
};
        locals.var_eta_mu = assign13270_e18146;
        locals.var_eta_mu_rv = 0.0;

        let assign13280_e18150: f64 = (locals.var_tratio).powf(locals.var_ute_i);
        let assign13280_e18151: f64 = (locals.var_u0_i * assign13280_e18150);
        (locals.var_u0_t, locals.var_u0_t_dn0, locals.var_u0_t_dn2, locals.var_u0_t_dn3, locals.var_u0_t_dn4, locals.var_u0_t_dn5, locals.var_u0_t_dn6, locals.var_u0_t_dn7, locals.var_u0_t_dn8, locals.var_u0_t_dn9, locals.var_u0_t_dn10, locals.var_u0_t_dn11, locals.var_u0_t_dn12, locals.var_u0_t_dn13, locals.var_u0_t_dn14, ) = (assign13280_e18151, 0.0, 0.0, 0.0, (locals.var_u0_i * if 0.0 == 0.0 && ((locals.var_ute_i) as f64).is_finite() && ((locals.var_ute_i) as f64).fract() == 0.0 { if locals.var_ute_i == 0.0 { 0.0 } else { (locals.var_ute_i * ((locals.var_tratio).powf(locals.var_ute_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13280_e18150 * (locals.var_ute_i * (locals.var_tratio_dn4 / locals.var_tratio))) }), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_u0_t_rv = 0.0;

        let assign13290_e18156: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign13290_e18157: f64 = (1.0 + assign13290_e18156);
        let assign13290_e18159: f64 = (assign13290_e18157 - 1e-6);
        let assign13290_e18161: f64 = (-10000.0);
        let assign13290_e18163: f64 = (assign13290_e18161 * 0.001);
        let (assign13290_e18224, assign13290_e18224_d_n4,) = {
    if (!(assign13290_e18159 < assign13290_e18163)) {
        let assign13290_e18170: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign13290_e18171: f64 = (1.0 + assign13290_e18170);
        let assign13290_e18173: f64 = (assign13290_e18171 - 1e-6);
        let assign13290_e18177: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign13290_e18178: f64 = (1.0 + assign13290_e18177);
        let assign13290_e18180: f64 = (assign13290_e18178 - 1e-6);
        let assign13290_e18184: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign13290_e18185: f64 = (1.0 + assign13290_e18184);
        let assign13290_e18187: f64 = (assign13290_e18185 - 1e-6);
        let assign13290_e18188: f64 = (assign13290_e18180 * assign13290_e18187);
        let assign13290_e18191: f64 = (4.0 * 0.001);
        let assign13290_e18193: f64 = (assign13290_e18191 * 0.001);
        let assign13290_e18194: f64 = (assign13290_e18188 + assign13290_e18193);
        let assign13290_e18195: f64 = (assign13290_e18194).sqrt();
        let assign13290_e18196: f64 = (assign13290_e18173 + assign13290_e18195);
        let assign13290_e18197: f64 = (0.5 * assign13290_e18196);
        (assign13290_e18197, (0.5 * ((locals.var_ua1_i * locals.var_deltemp_dn4) + ((((locals.var_ua1_i * locals.var_deltemp_dn4) * assign13290_e18187) + (assign13290_e18180 * (locals.var_ua1_i * locals.var_deltemp_dn4))) / (2.0 * assign13290_e18195)))),)
    } else {
        let assign13290_e18201: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign13290_e18202: f64 = (1.0 + assign13290_e18201);
        let assign13290_e18204: f64 = (assign13290_e18202 - 1e-6);
        let assign13290_e18206: f64 = (-10000.0);
        let assign13290_e18208: f64 = (assign13290_e18206 * 0.001);
        let (assign13290_e18223, assign13290_e18223_d_n4,) = {
            if (assign13290_e18204 < assign13290_e18208) {
                let assign13290_e18211: f64 = (-0.001);
                let assign13290_e18213: f64 = (assign13290_e18211 * 0.001);
                let assign13290_e18217: f64 = (locals.var_ua1_i * locals.var_deltemp);
                let assign13290_e18218: f64 = (1.0 + assign13290_e18217);
                let assign13290_e18220: f64 = (assign13290_e18218 - 1e-6);
                let assign13290_e18221: f64 = (assign13290_e18213 / assign13290_e18220);
                (assign13290_e18221, (-((assign13290_e18213 * (locals.var_ua1_i * locals.var_deltemp_dn4)) / (assign13290_e18220 * assign13290_e18220))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13290_e18223, assign13290_e18223_d_n4,)
    }
};
        let assign13290_e18225: f64 = (locals.var_ua_i * assign13290_e18224);
        (locals.var_ua_t, locals.var_ua_t_dn0, locals.var_ua_t_dn2, locals.var_ua_t_dn3, locals.var_ua_t_dn4, locals.var_ua_t_dn5, locals.var_ua_t_dn6, locals.var_ua_t_dn7, locals.var_ua_t_dn8, locals.var_ua_t_dn9, locals.var_ua_t_dn10, locals.var_ua_t_dn11, locals.var_ua_t_dn12, locals.var_ua_t_dn13, locals.var_ua_t_dn14, ) = (assign13290_e18225, (locals.var_ua_i_dn0 * assign13290_e18224), (locals.var_ua_i_dn2 * assign13290_e18224), (locals.var_ua_i_dn3 * assign13290_e18224), ((locals.var_ua_i_dn4 * assign13290_e18224) + (locals.var_ua_i * assign13290_e18224_d_n4)), (locals.var_ua_i_dn5 * assign13290_e18224), (locals.var_ua_i_dn6 * assign13290_e18224), (locals.var_ua_i_dn7 * assign13290_e18224), (locals.var_ua_i_dn8 * assign13290_e18224), (locals.var_ua_i_dn9 * assign13290_e18224), (locals.var_ua_i_dn10 * assign13290_e18224), (locals.var_ua_i_dn11 * assign13290_e18224), (locals.var_ua_i_dn12 * assign13290_e18224), (locals.var_ua_i_dn13 * assign13290_e18224), (locals.var_ua_i_dn14 * assign13290_e18224), );
        locals.var_ua_t_rv = 0.0;

        let assign13300_e18230: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign13300_e18231: f64 = (1.0 + assign13300_e18230);
        let assign13300_e18233: f64 = (assign13300_e18231 - 1e-6);
        let assign13300_e18235: f64 = (-10000.0);
        let assign13300_e18237: f64 = (assign13300_e18235 * 0.001);
        let (assign13300_e18298, assign13300_e18298_d_n4,) = {
    if (!(assign13300_e18233 < assign13300_e18237)) {
        let assign13300_e18244: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign13300_e18245: f64 = (1.0 + assign13300_e18244);
        let assign13300_e18247: f64 = (assign13300_e18245 - 1e-6);
        let assign13300_e18251: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign13300_e18252: f64 = (1.0 + assign13300_e18251);
        let assign13300_e18254: f64 = (assign13300_e18252 - 1e-6);
        let assign13300_e18258: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign13300_e18259: f64 = (1.0 + assign13300_e18258);
        let assign13300_e18261: f64 = (assign13300_e18259 - 1e-6);
        let assign13300_e18262: f64 = (assign13300_e18254 * assign13300_e18261);
        let assign13300_e18265: f64 = (4.0 * 0.001);
        let assign13300_e18267: f64 = (assign13300_e18265 * 0.001);
        let assign13300_e18268: f64 = (assign13300_e18262 + assign13300_e18267);
        let assign13300_e18269: f64 = (assign13300_e18268).sqrt();
        let assign13300_e18270: f64 = (assign13300_e18247 + assign13300_e18269);
        let assign13300_e18271: f64 = (0.5 * assign13300_e18270);
        (assign13300_e18271, (0.5 * ((locals.var_uc1_i * locals.var_deltemp_dn4) + ((((locals.var_uc1_i * locals.var_deltemp_dn4) * assign13300_e18261) + (assign13300_e18254 * (locals.var_uc1_i * locals.var_deltemp_dn4))) / (2.0 * assign13300_e18269)))),)
    } else {
        let assign13300_e18275: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign13300_e18276: f64 = (1.0 + assign13300_e18275);
        let assign13300_e18278: f64 = (assign13300_e18276 - 1e-6);
        let assign13300_e18280: f64 = (-10000.0);
        let assign13300_e18282: f64 = (assign13300_e18280 * 0.001);
        let (assign13300_e18297, assign13300_e18297_d_n4,) = {
            if (assign13300_e18278 < assign13300_e18282) {
                let assign13300_e18285: f64 = (-0.001);
                let assign13300_e18287: f64 = (assign13300_e18285 * 0.001);
                let assign13300_e18291: f64 = (locals.var_uc1_i * locals.var_deltemp);
                let assign13300_e18292: f64 = (1.0 + assign13300_e18291);
                let assign13300_e18294: f64 = (assign13300_e18292 - 1e-6);
                let assign13300_e18295: f64 = (assign13300_e18287 / assign13300_e18294);
                (assign13300_e18295, (-((assign13300_e18287 * (locals.var_uc1_i * locals.var_deltemp_dn4)) / (assign13300_e18294 * assign13300_e18294))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13300_e18297, assign13300_e18297_d_n4,)
    }
};
        let assign13300_e18299: f64 = (locals.var_uc_i * assign13300_e18298);
        (locals.var_uc_t, locals.var_uc_t_dn0, locals.var_uc_t_dn2, locals.var_uc_t_dn3, locals.var_uc_t_dn4, locals.var_uc_t_dn5, locals.var_uc_t_dn6, locals.var_uc_t_dn7, locals.var_uc_t_dn8, locals.var_uc_t_dn9, locals.var_uc_t_dn10, locals.var_uc_t_dn11, locals.var_uc_t_dn12, locals.var_uc_t_dn13, locals.var_uc_t_dn14, ) = (assign13300_e18299, (locals.var_uc_i_dn0 * assign13300_e18298), (locals.var_uc_i_dn2 * assign13300_e18298), (locals.var_uc_i_dn3 * assign13300_e18298), ((locals.var_uc_i_dn4 * assign13300_e18298) + (locals.var_uc_i * assign13300_e18298_d_n4)), (locals.var_uc_i_dn5 * assign13300_e18298), (locals.var_uc_i_dn6 * assign13300_e18298), (locals.var_uc_i_dn7 * assign13300_e18298), (locals.var_uc_i_dn8 * assign13300_e18298), (locals.var_uc_i_dn9 * assign13300_e18298), (locals.var_uc_i_dn10 * assign13300_e18298), (locals.var_uc_i_dn11 * assign13300_e18298), (locals.var_uc_i_dn12 * assign13300_e18298), (locals.var_uc_i_dn13 * assign13300_e18298), (locals.var_uc_i_dn14 * assign13300_e18298), );
        locals.var_uc_t_rv = 0.0;

        let assign13310_e18303: f64 = (locals.var_tratio).powf(locals.var_ud1_i);
        let assign13310_e18304: f64 = (locals.var_ud_i * assign13310_e18303);
        (locals.var_ud_t, locals.var_ud_t_dn0, locals.var_ud_t_dn2, locals.var_ud_t_dn3, locals.var_ud_t_dn4, locals.var_ud_t_dn5, locals.var_ud_t_dn6, locals.var_ud_t_dn7, locals.var_ud_t_dn8, locals.var_ud_t_dn9, locals.var_ud_t_dn10, locals.var_ud_t_dn11, locals.var_ud_t_dn12, locals.var_ud_t_dn13, locals.var_ud_t_dn14, ) = (assign13310_e18304, (locals.var_ud_i_dn0 * assign13310_e18303), (locals.var_ud_i_dn2 * assign13310_e18303), (locals.var_ud_i_dn3 * assign13310_e18303), ((locals.var_ud_i_dn4 * assign13310_e18303) + (locals.var_ud_i * if 0.0 == 0.0 && ((locals.var_ud1_i) as f64).is_finite() && ((locals.var_ud1_i) as f64).fract() == 0.0 { if locals.var_ud1_i == 0.0 { 0.0 } else { (locals.var_ud1_i * ((locals.var_tratio).powf(locals.var_ud1_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13310_e18303 * (locals.var_ud1_i * (locals.var_tratio_dn4 / locals.var_tratio))) })), (locals.var_ud_i_dn5 * assign13310_e18303), (locals.var_ud_i_dn6 * assign13310_e18303), (locals.var_ud_i_dn7 * assign13310_e18303), (locals.var_ud_i_dn8 * assign13310_e18303), (locals.var_ud_i_dn9 * assign13310_e18303), (locals.var_ud_i_dn10 * assign13310_e18303), (locals.var_ud_i_dn11 * assign13310_e18303), (locals.var_ud_i_dn12 * assign13310_e18303), (locals.var_ud_i_dn13 * assign13310_e18303), (locals.var_ud_i_dn14 * assign13310_e18303), );
        locals.var_ud_t_rv = 0.0;

        let assign13320_e18308: f64 = (locals.var_tratio).powf(locals.var_ucste_i);
        let assign13320_e18309: f64 = (locals.var_ucs_i * assign13320_e18308);
        (locals.var_ucs_t, locals.var_ucs_t_dn4, ) = (assign13320_e18309, (locals.var_ucs_i * if 0.0 == 0.0 && ((locals.var_ucste_i) as f64).is_finite() && ((locals.var_ucste_i) as f64).fract() == 0.0 { if locals.var_ucste_i == 0.0 { 0.0 } else { (locals.var_ucste_i * ((locals.var_tratio).powf(locals.var_ucste_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13320_e18308 * (locals.var_ucste_i * (locals.var_tratio_dn4 / locals.var_tratio))) }), );
        locals.var_ucs_t_rv = 0.0;

        let assign13330_e18315: f64 = (locals.var_tratio - 1.0);
        let assign13330_e18316: f64 = (locals.var_eu1_i * assign13330_e18315);
        let assign13330_e18317: f64 = (1.0 + assign13330_e18316);
        let assign13330_e18319: f64 = (-10000.0);
        let assign13330_e18321: f64 = (assign13330_e18319 * 0.001);
        let (assign13330_e18382, assign13330_e18382_d_n4,) = {
    if (!(assign13330_e18317 < assign13330_e18321)) {
        let assign13330_e18329: f64 = (locals.var_tratio - 1.0);
        let assign13330_e18330: f64 = (locals.var_eu1_i * assign13330_e18329);
        let assign13330_e18331: f64 = (1.0 + assign13330_e18330);
        let assign13330_e18336: f64 = (locals.var_tratio - 1.0);
        let assign13330_e18337: f64 = (locals.var_eu1_i * assign13330_e18336);
        let assign13330_e18338: f64 = (1.0 + assign13330_e18337);
        let assign13330_e18343: f64 = (locals.var_tratio - 1.0);
        let assign13330_e18344: f64 = (locals.var_eu1_i * assign13330_e18343);
        let assign13330_e18345: f64 = (1.0 + assign13330_e18344);
        let assign13330_e18346: f64 = (assign13330_e18338 * assign13330_e18345);
        let assign13330_e18349: f64 = (4.0 * 0.001);
        let assign13330_e18351: f64 = (assign13330_e18349 * 0.001);
        let assign13330_e18352: f64 = (assign13330_e18346 + assign13330_e18351);
        let assign13330_e18353: f64 = (assign13330_e18352).sqrt();
        let assign13330_e18354: f64 = (assign13330_e18331 + assign13330_e18353);
        let assign13330_e18355: f64 = (0.5 * assign13330_e18354);
        (assign13330_e18355, (0.5 * ((locals.var_eu1_i * locals.var_tratio_dn4) + ((((locals.var_eu1_i * locals.var_tratio_dn4) * assign13330_e18345) + (assign13330_e18338 * (locals.var_eu1_i * locals.var_tratio_dn4))) / (2.0 * assign13330_e18353)))),)
    } else {
        let assign13330_e18360: f64 = (locals.var_tratio - 1.0);
        let assign13330_e18361: f64 = (locals.var_eu1_i * assign13330_e18360);
        let assign13330_e18362: f64 = (1.0 + assign13330_e18361);
        let assign13330_e18364: f64 = (-10000.0);
        let assign13330_e18366: f64 = (assign13330_e18364 * 0.001);
        let (assign13330_e18381, assign13330_e18381_d_n4,) = {
            if (assign13330_e18362 < assign13330_e18366) {
                let assign13330_e18369: f64 = (-0.001);
                let assign13330_e18371: f64 = (assign13330_e18369 * 0.001);
                let assign13330_e18376: f64 = (locals.var_tratio - 1.0);
                let assign13330_e18377: f64 = (locals.var_eu1_i * assign13330_e18376);
                let assign13330_e18378: f64 = (1.0 + assign13330_e18377);
                let assign13330_e18379: f64 = (assign13330_e18371 / assign13330_e18378);
                (assign13330_e18379, (-((assign13330_e18371 * (locals.var_eu1_i * locals.var_tratio_dn4)) / (assign13330_e18378 * assign13330_e18378))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13330_e18381, assign13330_e18381_d_n4,)
    }
};
        let assign13330_e18383: f64 = (locals.var_eu_i * assign13330_e18382);
        (locals.var_eu_t, locals.var_eu_t_dn0, locals.var_eu_t_dn2, locals.var_eu_t_dn3, locals.var_eu_t_dn4, locals.var_eu_t_dn5, locals.var_eu_t_dn6, locals.var_eu_t_dn7, locals.var_eu_t_dn8, locals.var_eu_t_dn9, locals.var_eu_t_dn10, locals.var_eu_t_dn11, locals.var_eu_t_dn12, locals.var_eu_t_dn13, locals.var_eu_t_dn14, ) = (assign13330_e18383, (locals.var_eu_i_dn0 * assign13330_e18382), (locals.var_eu_i_dn2 * assign13330_e18382), (locals.var_eu_i_dn3 * assign13330_e18382), ((locals.var_eu_i_dn4 * assign13330_e18382) + (locals.var_eu_i * assign13330_e18382_d_n4)), (locals.var_eu_i_dn5 * assign13330_e18382), (locals.var_eu_i_dn6 * assign13330_e18382), (locals.var_eu_i_dn7 * assign13330_e18382), (locals.var_eu_i_dn8 * assign13330_e18382), (locals.var_eu_i_dn9 * assign13330_e18382), (locals.var_eu_i_dn10 * assign13330_e18382), (locals.var_eu_i_dn11 * assign13330_e18382), (locals.var_eu_i_dn12 * assign13330_e18382), (locals.var_eu_i_dn13 * assign13330_e18382), (locals.var_eu_i_dn14 * assign13330_e18382), );
        locals.var_eu_t_rv = 0.0;

        let assign13340_e18386: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard454 = assign13340_e18386;
        locals.var_guard454_rv = 0.0;

        if (locals.var_guard454 != 0.0) {
            let assign13350_e18391: f64 = (locals.var_tratio).powf(locals.var_ute_i);
            let assign13350_e18392: f64 = (locals.var_u0r_i * assign13350_e18391);
            (locals.var_u0r_t, locals.var_u0r_t_dn4, ) = (assign13350_e18392, (locals.var_u0r_i * if 0.0 == 0.0 && ((locals.var_ute_i) as f64).is_finite() && ((locals.var_ute_i) as f64).fract() == 0.0 { if locals.var_ute_i == 0.0 { 0.0 } else { (locals.var_ute_i * ((locals.var_tratio).powf(locals.var_ute_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13350_e18391 * (locals.var_ute_i * (locals.var_tratio_dn4 / locals.var_tratio))) }), );
            locals.var_u0r_t_rv = 0.0;
        }

        if (locals.var_guard454 != 0.0) {
            let assign13360_e18400: f64 = (locals.var_ua1_i * locals.var_deltemp);
            let assign13360_e18401: f64 = (1.0 + assign13360_e18400);
            let assign13360_e18403: f64 = (assign13360_e18401 - 1e-6);
            let assign13360_e18405: f64 = (-10000.0);
            let assign13360_e18407: f64 = (assign13360_e18405 * 0.001);
            let (assign13360_e18468, assign13360_e18468_d_n4,) = {
    if (!(assign13360_e18403 < assign13360_e18407)) {
        let assign13360_e18414: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign13360_e18415: f64 = (1.0 + assign13360_e18414);
        let assign13360_e18417: f64 = (assign13360_e18415 - 1e-6);
        let assign13360_e18421: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign13360_e18422: f64 = (1.0 + assign13360_e18421);
        let assign13360_e18424: f64 = (assign13360_e18422 - 1e-6);
        let assign13360_e18428: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign13360_e18429: f64 = (1.0 + assign13360_e18428);
        let assign13360_e18431: f64 = (assign13360_e18429 - 1e-6);
        let assign13360_e18432: f64 = (assign13360_e18424 * assign13360_e18431);
        let assign13360_e18435: f64 = (4.0 * 0.001);
        let assign13360_e18437: f64 = (assign13360_e18435 * 0.001);
        let assign13360_e18438: f64 = (assign13360_e18432 + assign13360_e18437);
        let assign13360_e18439: f64 = (assign13360_e18438).sqrt();
        let assign13360_e18440: f64 = (assign13360_e18417 + assign13360_e18439);
        let assign13360_e18441: f64 = (0.5 * assign13360_e18440);
        (assign13360_e18441, (0.5 * ((locals.var_ua1_i * locals.var_deltemp_dn4) + ((((locals.var_ua1_i * locals.var_deltemp_dn4) * assign13360_e18431) + (assign13360_e18424 * (locals.var_ua1_i * locals.var_deltemp_dn4))) / (2.0 * assign13360_e18439)))),)
    } else {
        let assign13360_e18445: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign13360_e18446: f64 = (1.0 + assign13360_e18445);
        let assign13360_e18448: f64 = (assign13360_e18446 - 1e-6);
        let assign13360_e18450: f64 = (-10000.0);
        let assign13360_e18452: f64 = (assign13360_e18450 * 0.001);
        let (assign13360_e18467, assign13360_e18467_d_n4,) = {
            if (assign13360_e18448 < assign13360_e18452) {
                let assign13360_e18455: f64 = (-0.001);
                let assign13360_e18457: f64 = (assign13360_e18455 * 0.001);
                let assign13360_e18461: f64 = (locals.var_ua1_i * locals.var_deltemp);
                let assign13360_e18462: f64 = (1.0 + assign13360_e18461);
                let assign13360_e18464: f64 = (assign13360_e18462 - 1e-6);
                let assign13360_e18465: f64 = (assign13360_e18457 / assign13360_e18464);
                (assign13360_e18465, (-((assign13360_e18457 * (locals.var_ua1_i * locals.var_deltemp_dn4)) / (assign13360_e18464 * assign13360_e18464))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13360_e18467, assign13360_e18467_d_n4,)
    }
};
            let assign13360_e18469: f64 = (locals.var_uar_i * assign13360_e18468);
            (locals.var_uar_t, locals.var_uar_t_dn0, locals.var_uar_t_dn2, locals.var_uar_t_dn3, locals.var_uar_t_dn4, locals.var_uar_t_dn5, locals.var_uar_t_dn6, locals.var_uar_t_dn7, locals.var_uar_t_dn8, locals.var_uar_t_dn9, locals.var_uar_t_dn10, locals.var_uar_t_dn11, locals.var_uar_t_dn12, locals.var_uar_t_dn13, locals.var_uar_t_dn14, ) = (assign13360_e18469, (locals.var_uar_i_dn0 * assign13360_e18468), (locals.var_uar_i_dn2 * assign13360_e18468), (locals.var_uar_i_dn3 * assign13360_e18468), ((locals.var_uar_i_dn4 * assign13360_e18468) + (locals.var_uar_i * assign13360_e18468_d_n4)), (locals.var_uar_i_dn5 * assign13360_e18468), (locals.var_uar_i_dn6 * assign13360_e18468), (locals.var_uar_i_dn7 * assign13360_e18468), (locals.var_uar_i_dn8 * assign13360_e18468), (locals.var_uar_i_dn9 * assign13360_e18468), (locals.var_uar_i_dn10 * assign13360_e18468), (locals.var_uar_i_dn11 * assign13360_e18468), (locals.var_uar_i_dn12 * assign13360_e18468), (locals.var_uar_i_dn13 * assign13360_e18468), (locals.var_uar_i_dn14 * assign13360_e18468), );
            locals.var_uar_t_rv = 0.0;
        }

        if (locals.var_guard454 != 0.0) {
            let assign13370_e18477: f64 = (locals.var_uc1_i * locals.var_deltemp);
            let assign13370_e18478: f64 = (1.0 + assign13370_e18477);
            let assign13370_e18480: f64 = (assign13370_e18478 - 1e-6);
            let assign13370_e18482: f64 = (-10000.0);
            let assign13370_e18484: f64 = (assign13370_e18482 * 0.001);
            let (assign13370_e18545, assign13370_e18545_d_n4,) = {
    if (!(assign13370_e18480 < assign13370_e18484)) {
        let assign13370_e18491: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign13370_e18492: f64 = (1.0 + assign13370_e18491);
        let assign13370_e18494: f64 = (assign13370_e18492 - 1e-6);
        let assign13370_e18498: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign13370_e18499: f64 = (1.0 + assign13370_e18498);
        let assign13370_e18501: f64 = (assign13370_e18499 - 1e-6);
        let assign13370_e18505: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign13370_e18506: f64 = (1.0 + assign13370_e18505);
        let assign13370_e18508: f64 = (assign13370_e18506 - 1e-6);
        let assign13370_e18509: f64 = (assign13370_e18501 * assign13370_e18508);
        let assign13370_e18512: f64 = (4.0 * 0.001);
        let assign13370_e18514: f64 = (assign13370_e18512 * 0.001);
        let assign13370_e18515: f64 = (assign13370_e18509 + assign13370_e18514);
        let assign13370_e18516: f64 = (assign13370_e18515).sqrt();
        let assign13370_e18517: f64 = (assign13370_e18494 + assign13370_e18516);
        let assign13370_e18518: f64 = (0.5 * assign13370_e18517);
        (assign13370_e18518, (0.5 * ((locals.var_uc1_i * locals.var_deltemp_dn4) + ((((locals.var_uc1_i * locals.var_deltemp_dn4) * assign13370_e18508) + (assign13370_e18501 * (locals.var_uc1_i * locals.var_deltemp_dn4))) / (2.0 * assign13370_e18516)))),)
    } else {
        let assign13370_e18522: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign13370_e18523: f64 = (1.0 + assign13370_e18522);
        let assign13370_e18525: f64 = (assign13370_e18523 - 1e-6);
        let assign13370_e18527: f64 = (-10000.0);
        let assign13370_e18529: f64 = (assign13370_e18527 * 0.001);
        let (assign13370_e18544, assign13370_e18544_d_n4,) = {
            if (assign13370_e18525 < assign13370_e18529) {
                let assign13370_e18532: f64 = (-0.001);
                let assign13370_e18534: f64 = (assign13370_e18532 * 0.001);
                let assign13370_e18538: f64 = (locals.var_uc1_i * locals.var_deltemp);
                let assign13370_e18539: f64 = (1.0 + assign13370_e18538);
                let assign13370_e18541: f64 = (assign13370_e18539 - 1e-6);
                let assign13370_e18542: f64 = (assign13370_e18534 / assign13370_e18541);
                (assign13370_e18542, (-((assign13370_e18534 * (locals.var_uc1_i * locals.var_deltemp_dn4)) / (assign13370_e18541 * assign13370_e18541))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13370_e18544, assign13370_e18544_d_n4,)
    }
};
            let assign13370_e18546: f64 = (locals.var_ucr_i * assign13370_e18545);
            (locals.var_ucr_t, locals.var_ucr_t_dn0, locals.var_ucr_t_dn2, locals.var_ucr_t_dn3, locals.var_ucr_t_dn4, locals.var_ucr_t_dn5, locals.var_ucr_t_dn6, locals.var_ucr_t_dn7, locals.var_ucr_t_dn8, locals.var_ucr_t_dn9, locals.var_ucr_t_dn10, locals.var_ucr_t_dn11, locals.var_ucr_t_dn12, locals.var_ucr_t_dn13, locals.var_ucr_t_dn14, ) = (assign13370_e18546, (locals.var_ucr_i_dn0 * assign13370_e18545), (locals.var_ucr_i_dn2 * assign13370_e18545), (locals.var_ucr_i_dn3 * assign13370_e18545), ((locals.var_ucr_i_dn4 * assign13370_e18545) + (locals.var_ucr_i * assign13370_e18545_d_n4)), (locals.var_ucr_i_dn5 * assign13370_e18545), (locals.var_ucr_i_dn6 * assign13370_e18545), (locals.var_ucr_i_dn7 * assign13370_e18545), (locals.var_ucr_i_dn8 * assign13370_e18545), (locals.var_ucr_i_dn9 * assign13370_e18545), (locals.var_ucr_i_dn10 * assign13370_e18545), (locals.var_ucr_i_dn11 * assign13370_e18545), (locals.var_ucr_i_dn12 * assign13370_e18545), (locals.var_ucr_i_dn13 * assign13370_e18545), (locals.var_ucr_i_dn14 * assign13370_e18545), );
            locals.var_ucr_t_rv = 0.0;
        }

        if (locals.var_guard454 != 0.0) {
            let assign13380_e18553: f64 = (locals.var_tratio).powf(locals.var_ud1_i);
            let assign13380_e18554: f64 = (locals.var_udr_i * assign13380_e18553);
            (locals.var_udr_t, locals.var_udr_t_dn0, locals.var_udr_t_dn2, locals.var_udr_t_dn3, locals.var_udr_t_dn4, locals.var_udr_t_dn5, locals.var_udr_t_dn6, locals.var_udr_t_dn7, locals.var_udr_t_dn8, locals.var_udr_t_dn9, locals.var_udr_t_dn10, locals.var_udr_t_dn11, locals.var_udr_t_dn12, locals.var_udr_t_dn13, locals.var_udr_t_dn14, ) = (assign13380_e18554, (locals.var_udr_i_dn0 * assign13380_e18553), (locals.var_udr_i_dn2 * assign13380_e18553), (locals.var_udr_i_dn3 * assign13380_e18553), ((locals.var_udr_i_dn4 * assign13380_e18553) + (locals.var_udr_i * if 0.0 == 0.0 && ((locals.var_ud1_i) as f64).is_finite() && ((locals.var_ud1_i) as f64).fract() == 0.0 { if locals.var_ud1_i == 0.0 { 0.0 } else { (locals.var_ud1_i * ((locals.var_tratio).powf(locals.var_ud1_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13380_e18553 * (locals.var_ud1_i * (locals.var_tratio_dn4 / locals.var_tratio))) })), (locals.var_udr_i_dn5 * assign13380_e18553), (locals.var_udr_i_dn6 * assign13380_e18553), (locals.var_udr_i_dn7 * assign13380_e18553), (locals.var_udr_i_dn8 * assign13380_e18553), (locals.var_udr_i_dn9 * assign13380_e18553), (locals.var_udr_i_dn10 * assign13380_e18553), (locals.var_udr_i_dn11 * assign13380_e18553), (locals.var_udr_i_dn12 * assign13380_e18553), (locals.var_udr_i_dn13 * assign13380_e18553), (locals.var_udr_i_dn14 * assign13380_e18553), );
            locals.var_udr_t_rv = 0.0;
        }

        if (locals.var_guard454 != 0.0) {
            let assign13390_e18561: f64 = (locals.var_tratio).powf(locals.var_ucste_i);
            let assign13390_e18562: f64 = (locals.var_ucsr_i * assign13390_e18561);
            (locals.var_ucsr_t, locals.var_ucsr_t_dn4, ) = (assign13390_e18562, (locals.var_ucsr_i * if 0.0 == 0.0 && ((locals.var_ucste_i) as f64).is_finite() && ((locals.var_ucste_i) as f64).fract() == 0.0 { if locals.var_ucste_i == 0.0 { 0.0 } else { (locals.var_ucste_i * ((locals.var_tratio).powf(locals.var_ucste_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13390_e18561 * (locals.var_ucste_i * (locals.var_tratio_dn4 / locals.var_tratio))) }), );
            locals.var_ucsr_t_rv = 0.0;
        }

        let assign13400_e18567: f64 = (locals.var_tratio).powf(locals.var_prt_i);
        (locals.var_rdstemp, locals.var_rdstemp_dn4, ) = (assign13400_e18567, if 0.0 == 0.0 && ((locals.var_prt_i) as f64).is_finite() && ((locals.var_prt_i) as f64).fract() == 0.0 { if locals.var_prt_i == 0.0 { 0.0 } else { (locals.var_prt_i * ((locals.var_tratio).powf(locals.var_prt_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13400_e18567 * (locals.var_prt_i * (locals.var_tratio_dn4 / locals.var_tratio))) }, );
        locals.var_rdstemp_rv = 0.0;

        let assign13410_e18571: f64 = (-locals.var_at_i);
        let assign13410_e18572: f64 = (locals.var_tratio).powf(assign13410_e18571);
        let assign13410_e18573: f64 = (locals.var_vsat_i * assign13410_e18572);
        (locals.var_vsat_t, locals.var_vsat_t_dn0, locals.var_vsat_t_dn2, locals.var_vsat_t_dn3, locals.var_vsat_t_dn4, locals.var_vsat_t_dn5, locals.var_vsat_t_dn6, locals.var_vsat_t_dn7, locals.var_vsat_t_dn8, locals.var_vsat_t_dn9, locals.var_vsat_t_dn10, locals.var_vsat_t_dn11, locals.var_vsat_t_dn12, locals.var_vsat_t_dn13, locals.var_vsat_t_dn14, ) = (assign13410_e18573, (locals.var_vsat_i_dn0 * assign13410_e18572), (locals.var_vsat_i_dn2 * assign13410_e18572), (locals.var_vsat_i_dn3 * assign13410_e18572), ((locals.var_vsat_i_dn4 * assign13410_e18572) + (locals.var_vsat_i * if 0.0 == 0.0 && ((assign13410_e18571) as f64).is_finite() && ((assign13410_e18571) as f64).fract() == 0.0 { if assign13410_e18571 == 0.0 { 0.0 } else { (assign13410_e18571 * ((locals.var_tratio).powf(assign13410_e18571 - 1.0) * locals.var_tratio_dn4)) } } else { (assign13410_e18572 * (assign13410_e18571 * (locals.var_tratio_dn4 / locals.var_tratio))) })), (locals.var_vsat_i_dn5 * assign13410_e18572), (locals.var_vsat_i_dn6 * assign13410_e18572), (locals.var_vsat_i_dn7 * assign13410_e18572), (locals.var_vsat_i_dn8 * assign13410_e18572), (locals.var_vsat_i_dn9 * assign13410_e18572), (locals.var_vsat_i_dn10 * assign13410_e18572), (locals.var_vsat_i_dn11 * assign13410_e18572), (locals.var_vsat_i_dn12 * assign13410_e18572), (locals.var_vsat_i_dn13 * assign13410_e18572), (locals.var_vsat_i_dn14 * assign13410_e18572), );
        locals.var_vsat_t_rv = 0.0;

        let assign13420_e18576: f64 = if locals.var_vsat_t < 100.0 { 1.0 } else { 0.0 };
        locals.var_guard455 = assign13420_e18576;
        locals.var_guard455_rv = 0.0;

        if (locals.var_guard455 != 0.0) {
            (locals.var_vsat_t, locals.var_vsat_t_dn0, locals.var_vsat_t_dn2, locals.var_vsat_t_dn3, locals.var_vsat_t_dn4, locals.var_vsat_t_dn5, locals.var_vsat_t_dn6, locals.var_vsat_t_dn7, locals.var_vsat_t_dn8, locals.var_vsat_t_dn9, locals.var_vsat_t_dn10, locals.var_vsat_t_dn11, locals.var_vsat_t_dn12, locals.var_vsat_t_dn13, locals.var_vsat_t_dn14, ) = (100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_vsat_t_rv = 0.0;
        }

        let assign13440_e18583: f64 = if p.p1094 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard456 = assign13440_e18583;
        locals.var_guard456_rv = 0.0;

        if (locals.var_guard456 != 0.0) {
            let assign13450_e18587: f64 = (locals.var_tratio).powf(p.p1120);
            (locals.var_rdstemphv, locals.var_rdstemphv_dn4, ) = (assign13450_e18587, if 0.0 == 0.0 && ((p.p1120) as f64).is_finite() && ((p.p1120) as f64).fract() == 0.0 { if p.p1120 == 0.0 { 0.0 } else { (p.p1120 * ((locals.var_tratio).powf(p.p1120 - 1.0) * locals.var_tratio_dn4)) } } else { (assign13450_e18587 * (p.p1120 * (locals.var_tratio_dn4 / locals.var_tratio))) }, );
            locals.var_rdstemphv_rv = 0.0;
        }

        if (locals.var_guard456 != 0.0) {
            let assign13460_e18594: f64 = (-p.p1121);
            let assign13460_e18595: f64 = (locals.var_tratio).powf(assign13460_e18594);
            let assign13460_e18596: f64 = (p.p1100 * assign13460_e18595);
            (locals.var_vdrift_t, locals.var_vdrift_t_dn4, ) = (assign13460_e18596, (p.p1100 * if 0.0 == 0.0 && ((assign13460_e18594) as f64).is_finite() && ((assign13460_e18594) as f64).fract() == 0.0 { if assign13460_e18594 == 0.0 { 0.0 } else { (assign13460_e18594 * ((locals.var_tratio).powf(assign13460_e18594 - 1.0) * locals.var_tratio_dn4)) } } else { (assign13460_e18595 * (assign13460_e18594 * (locals.var_tratio_dn4 / locals.var_tratio))) }), );
            locals.var_vdrift_t_rv = 0.0;
        }

        let assign13470_e18601: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard457 = assign13470_e18601;
        locals.var_guard457_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_14(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard457 != 0.0) {
            let assign13480_e18606: f64 = (-locals.var_at_i);
            let assign13480_e18607: f64 = (locals.var_tratio).powf(assign13480_e18606);
            let assign13480_e18608: f64 = (locals.var_vsatr_i * assign13480_e18607);
            (locals.var_vsatr_t, locals.var_vsatr_t_dn0, locals.var_vsatr_t_dn2, locals.var_vsatr_t_dn3, locals.var_vsatr_t_dn4, locals.var_vsatr_t_dn5, locals.var_vsatr_t_dn6, locals.var_vsatr_t_dn7, locals.var_vsatr_t_dn8, locals.var_vsatr_t_dn9, locals.var_vsatr_t_dn10, locals.var_vsatr_t_dn11, locals.var_vsatr_t_dn12, locals.var_vsatr_t_dn13, locals.var_vsatr_t_dn14, ) = (assign13480_e18608, (locals.var_vsatr_i_dn0 * assign13480_e18607), (locals.var_vsatr_i_dn2 * assign13480_e18607), (locals.var_vsatr_i_dn3 * assign13480_e18607), ((locals.var_vsatr_i_dn4 * assign13480_e18607) + (locals.var_vsatr_i * if 0.0 == 0.0 && ((assign13480_e18606) as f64).is_finite() && ((assign13480_e18606) as f64).fract() == 0.0 { if assign13480_e18606 == 0.0 { 0.0 } else { (assign13480_e18606 * ((locals.var_tratio).powf(assign13480_e18606 - 1.0) * locals.var_tratio_dn4)) } } else { (assign13480_e18607 * (assign13480_e18606 * (locals.var_tratio_dn4 / locals.var_tratio))) })), (locals.var_vsatr_i_dn5 * assign13480_e18607), (locals.var_vsatr_i_dn6 * assign13480_e18607), (locals.var_vsatr_i_dn7 * assign13480_e18607), (locals.var_vsatr_i_dn8 * assign13480_e18607), (locals.var_vsatr_i_dn9 * assign13480_e18607), (locals.var_vsatr_i_dn10 * assign13480_e18607), (locals.var_vsatr_i_dn11 * assign13480_e18607), (locals.var_vsatr_i_dn12 * assign13480_e18607), (locals.var_vsatr_i_dn13 * assign13480_e18607), (locals.var_vsatr_i_dn14 * assign13480_e18607), );
            locals.var_vsatr_t_rv = 0.0;
        }

        let assign13490_e18613: f64 = if locals.var_vsatr_t < 100.0 { 1.0 } else { 0.0 };
        locals.var_guard458 = assign13490_e18613;
        locals.var_guard458_rv = 0.0;

        if ((locals.var_guard457 != 0.0) && (locals.var_guard458 != 0.0)) {
            (locals.var_vsatr_t, locals.var_vsatr_t_dn0, locals.var_vsatr_t_dn2, locals.var_vsatr_t_dn3, locals.var_vsatr_t_dn4, locals.var_vsatr_t_dn5, locals.var_vsatr_t_dn6, locals.var_vsatr_t_dn7, locals.var_vsatr_t_dn8, locals.var_vsatr_t_dn9, locals.var_vsatr_t_dn10, locals.var_vsatr_t_dn11, locals.var_vsatr_t_dn12, locals.var_vsatr_t_dn13, locals.var_vsatr_t_dn14, ) = (100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_vsatr_t_rv = 0.0;
        }

        let assign13510_e18623: f64 = (-locals.var_at_i);
        let assign13510_e18624: f64 = (locals.var_tratio).powf(assign13510_e18623);
        let assign13510_e18625: f64 = (locals.var_vsatcv_i * assign13510_e18624);
        (locals.var_vsatcv_t, locals.var_vsatcv_t_dn0, locals.var_vsatcv_t_dn2, locals.var_vsatcv_t_dn3, locals.var_vsatcv_t_dn4, locals.var_vsatcv_t_dn5, locals.var_vsatcv_t_dn6, locals.var_vsatcv_t_dn7, locals.var_vsatcv_t_dn8, locals.var_vsatcv_t_dn9, locals.var_vsatcv_t_dn10, locals.var_vsatcv_t_dn11, locals.var_vsatcv_t_dn12, locals.var_vsatcv_t_dn13, locals.var_vsatcv_t_dn14, ) = (assign13510_e18625, (locals.var_vsatcv_i_dn0 * assign13510_e18624), (locals.var_vsatcv_i_dn2 * assign13510_e18624), (locals.var_vsatcv_i_dn3 * assign13510_e18624), ((locals.var_vsatcv_i_dn4 * assign13510_e18624) + (locals.var_vsatcv_i * if 0.0 == 0.0 && ((assign13510_e18623) as f64).is_finite() && ((assign13510_e18623) as f64).fract() == 0.0 { if assign13510_e18623 == 0.0 { 0.0 } else { (assign13510_e18623 * ((locals.var_tratio).powf(assign13510_e18623 - 1.0) * locals.var_tratio_dn4)) } } else { (assign13510_e18624 * (assign13510_e18623 * (locals.var_tratio_dn4 / locals.var_tratio))) })), (locals.var_vsatcv_i_dn5 * assign13510_e18624), (locals.var_vsatcv_i_dn6 * assign13510_e18624), (locals.var_vsatcv_i_dn7 * assign13510_e18624), (locals.var_vsatcv_i_dn8 * assign13510_e18624), (locals.var_vsatcv_i_dn9 * assign13510_e18624), (locals.var_vsatcv_i_dn10 * assign13510_e18624), (locals.var_vsatcv_i_dn11 * assign13510_e18624), (locals.var_vsatcv_i_dn12 * assign13510_e18624), (locals.var_vsatcv_i_dn13 * assign13510_e18624), (locals.var_vsatcv_i_dn14 * assign13510_e18624), );
        locals.var_vsatcv_t_rv = 0.0;

        let assign13520_e18628: f64 = if locals.var_vsatcv_t < 100.0 { 1.0 } else { 0.0 };
        locals.var_guard459 = assign13520_e18628;
        locals.var_guard459_rv = 0.0;

        if (locals.var_guard459 != 0.0) {
            (locals.var_vsatcv_t, locals.var_vsatcv_t_dn0, locals.var_vsatcv_t_dn2, locals.var_vsatcv_t_dn3, locals.var_vsatcv_t_dn4, locals.var_vsatcv_t_dn5, locals.var_vsatcv_t_dn6, locals.var_vsatcv_t_dn7, locals.var_vsatcv_t_dn8, locals.var_vsatcv_t_dn9, locals.var_vsatcv_t_dn10, locals.var_vsatcv_t_dn11, locals.var_vsatcv_t_dn12, locals.var_vsatcv_t_dn13, locals.var_vsatcv_t_dn14, ) = (100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_vsatcv_t_rv = 0.0;
        }

        let assign13540_e18636: f64 = (1.0 / locals.var_delta_i);
        let assign13540_e18640: f64 = (p.p861 * locals.var_deltemp);
        let assign13540_e18641: f64 = (1.0 + assign13540_e18640);
        let assign13540_e18642: f64 = (assign13540_e18636 * assign13540_e18641);
        let assign13540_e18644: f64 = (assign13540_e18642 - 2.0);
        let assign13540_e18646: f64 = (-10000.0);
        let assign13540_e18648: f64 = (assign13540_e18646 * 0.001);
        let (assign13540_e18729, assign13540_e18729_d_n0, assign13540_e18729_d_n2, assign13540_e18729_d_n3, assign13540_e18729_d_n4, assign13540_e18729_d_n5, assign13540_e18729_d_n6, assign13540_e18729_d_n7, assign13540_e18729_d_n8, assign13540_e18729_d_n9, assign13540_e18729_d_n10, assign13540_e18729_d_n11, assign13540_e18729_d_n12, assign13540_e18729_d_n13, assign13540_e18729_d_n14,) = {
    if (!(assign13540_e18644 < assign13540_e18648)) {
        let assign13540_e18654: f64 = (1.0 / locals.var_delta_i);
        let assign13540_e18658: f64 = (p.p861 * locals.var_deltemp);
        let assign13540_e18659: f64 = (1.0 + assign13540_e18658);
        let assign13540_e18660: f64 = (assign13540_e18654 * assign13540_e18659);
        let assign13540_e18662: f64 = (assign13540_e18660 - 2.0);
        let assign13540_e18665: f64 = (1.0 / locals.var_delta_i);
        let assign13540_e18669: f64 = (p.p861 * locals.var_deltemp);
        let assign13540_e18670: f64 = (1.0 + assign13540_e18669);
        let assign13540_e18671: f64 = (assign13540_e18665 * assign13540_e18670);
        let assign13540_e18673: f64 = (assign13540_e18671 - 2.0);
        let assign13540_e18676: f64 = (1.0 / locals.var_delta_i);
        let assign13540_e18680: f64 = (p.p861 * locals.var_deltemp);
        let assign13540_e18681: f64 = (1.0 + assign13540_e18680);
        let assign13540_e18682: f64 = (assign13540_e18676 * assign13540_e18681);
        let assign13540_e18684: f64 = (assign13540_e18682 - 2.0);
        let assign13540_e18685: f64 = (assign13540_e18673 * assign13540_e18684);
        let assign13540_e18688: f64 = (4.0 * 0.001);
        let assign13540_e18690: f64 = (assign13540_e18688 * 0.001);
        let assign13540_e18691: f64 = (assign13540_e18685 + assign13540_e18690);
        let assign13540_e18692: f64 = (assign13540_e18691).sqrt();
        let assign13540_e18693: f64 = (assign13540_e18662 + assign13540_e18692);
        let assign13540_e18694: f64 = (0.5 * assign13540_e18693);
        (assign13540_e18694, (0.5 * (((-(locals.var_delta_i_dn0 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (((((-(locals.var_delta_i_dn0 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) * assign13540_e18684) + (assign13540_e18673 * ((-(locals.var_delta_i_dn0 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681))) / (2.0 * assign13540_e18692)))), (0.5 * (((-(locals.var_delta_i_dn2 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (((((-(locals.var_delta_i_dn2 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) * assign13540_e18684) + (assign13540_e18673 * ((-(locals.var_delta_i_dn2 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681))) / (2.0 * assign13540_e18692)))), (0.5 * (((-(locals.var_delta_i_dn3 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (((((-(locals.var_delta_i_dn3 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) * assign13540_e18684) + (assign13540_e18673 * ((-(locals.var_delta_i_dn3 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681))) / (2.0 * assign13540_e18692)))), (0.5 * ((((-(locals.var_delta_i_dn4 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (assign13540_e18654 * (p.p861 * locals.var_deltemp_dn4))) + ((((((-(locals.var_delta_i_dn4 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) + (assign13540_e18665 * (p.p861 * locals.var_deltemp_dn4))) * assign13540_e18684) + (assign13540_e18673 * (((-(locals.var_delta_i_dn4 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681) + (assign13540_e18676 * (p.p861 * locals.var_deltemp_dn4))))) / (2.0 * assign13540_e18692)))), (0.5 * (((-(locals.var_delta_i_dn5 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (((((-(locals.var_delta_i_dn5 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) * assign13540_e18684) + (assign13540_e18673 * ((-(locals.var_delta_i_dn5 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681))) / (2.0 * assign13540_e18692)))), (0.5 * (((-(locals.var_delta_i_dn6 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (((((-(locals.var_delta_i_dn6 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) * assign13540_e18684) + (assign13540_e18673 * ((-(locals.var_delta_i_dn6 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681))) / (2.0 * assign13540_e18692)))), (0.5 * (((-(locals.var_delta_i_dn7 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (((((-(locals.var_delta_i_dn7 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) * assign13540_e18684) + (assign13540_e18673 * ((-(locals.var_delta_i_dn7 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681))) / (2.0 * assign13540_e18692)))), (0.5 * (((-(locals.var_delta_i_dn8 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (((((-(locals.var_delta_i_dn8 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) * assign13540_e18684) + (assign13540_e18673 * ((-(locals.var_delta_i_dn8 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681))) / (2.0 * assign13540_e18692)))), (0.5 * (((-(locals.var_delta_i_dn9 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (((((-(locals.var_delta_i_dn9 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) * assign13540_e18684) + (assign13540_e18673 * ((-(locals.var_delta_i_dn9 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681))) / (2.0 * assign13540_e18692)))), (0.5 * (((-(locals.var_delta_i_dn10 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (((((-(locals.var_delta_i_dn10 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) * assign13540_e18684) + (assign13540_e18673 * ((-(locals.var_delta_i_dn10 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681))) / (2.0 * assign13540_e18692)))), (0.5 * (((-(locals.var_delta_i_dn11 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (((((-(locals.var_delta_i_dn11 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) * assign13540_e18684) + (assign13540_e18673 * ((-(locals.var_delta_i_dn11 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681))) / (2.0 * assign13540_e18692)))), (0.5 * (((-(locals.var_delta_i_dn12 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (((((-(locals.var_delta_i_dn12 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) * assign13540_e18684) + (assign13540_e18673 * ((-(locals.var_delta_i_dn12 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681))) / (2.0 * assign13540_e18692)))), (0.5 * (((-(locals.var_delta_i_dn13 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (((((-(locals.var_delta_i_dn13 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) * assign13540_e18684) + (assign13540_e18673 * ((-(locals.var_delta_i_dn13 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681))) / (2.0 * assign13540_e18692)))), (0.5 * (((-(locals.var_delta_i_dn14 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (((((-(locals.var_delta_i_dn14 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) * assign13540_e18684) + (assign13540_e18673 * ((-(locals.var_delta_i_dn14 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681))) / (2.0 * assign13540_e18692)))),)
    } else {
        let assign13540_e18697: f64 = (1.0 / locals.var_delta_i);
        let assign13540_e18701: f64 = (p.p861 * locals.var_deltemp);
        let assign13540_e18702: f64 = (1.0 + assign13540_e18701);
        let assign13540_e18703: f64 = (assign13540_e18697 * assign13540_e18702);
        let assign13540_e18705: f64 = (assign13540_e18703 - 2.0);
        let assign13540_e18707: f64 = (-10000.0);
        let assign13540_e18709: f64 = (assign13540_e18707 * 0.001);
        let (assign13540_e18728, assign13540_e18728_d_n0, assign13540_e18728_d_n2, assign13540_e18728_d_n3, assign13540_e18728_d_n4, assign13540_e18728_d_n5, assign13540_e18728_d_n6, assign13540_e18728_d_n7, assign13540_e18728_d_n8, assign13540_e18728_d_n9, assign13540_e18728_d_n10, assign13540_e18728_d_n11, assign13540_e18728_d_n12, assign13540_e18728_d_n13, assign13540_e18728_d_n14,) = {
            if (assign13540_e18705 < assign13540_e18709) {
                let assign13540_e18712: f64 = (-0.001);
                let assign13540_e18714: f64 = (assign13540_e18712 * 0.001);
                let assign13540_e18717: f64 = (1.0 / locals.var_delta_i);
                let assign13540_e18721: f64 = (p.p861 * locals.var_deltemp);
                let assign13540_e18722: f64 = (1.0 + assign13540_e18721);
                let assign13540_e18723: f64 = (assign13540_e18717 * assign13540_e18722);
                let assign13540_e18725: f64 = (assign13540_e18723 - 2.0);
                let assign13540_e18726: f64 = (assign13540_e18714 / assign13540_e18725);
                (assign13540_e18726, (-((assign13540_e18714 * ((-(locals.var_delta_i_dn0 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722)) / (assign13540_e18725 * assign13540_e18725))), (-((assign13540_e18714 * ((-(locals.var_delta_i_dn2 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722)) / (assign13540_e18725 * assign13540_e18725))), (-((assign13540_e18714 * ((-(locals.var_delta_i_dn3 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722)) / (assign13540_e18725 * assign13540_e18725))), (-((assign13540_e18714 * (((-(locals.var_delta_i_dn4 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722) + (assign13540_e18717 * (p.p861 * locals.var_deltemp_dn4)))) / (assign13540_e18725 * assign13540_e18725))), (-((assign13540_e18714 * ((-(locals.var_delta_i_dn5 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722)) / (assign13540_e18725 * assign13540_e18725))), (-((assign13540_e18714 * ((-(locals.var_delta_i_dn6 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722)) / (assign13540_e18725 * assign13540_e18725))), (-((assign13540_e18714 * ((-(locals.var_delta_i_dn7 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722)) / (assign13540_e18725 * assign13540_e18725))), (-((assign13540_e18714 * ((-(locals.var_delta_i_dn8 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722)) / (assign13540_e18725 * assign13540_e18725))), (-((assign13540_e18714 * ((-(locals.var_delta_i_dn9 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722)) / (assign13540_e18725 * assign13540_e18725))), (-((assign13540_e18714 * ((-(locals.var_delta_i_dn10 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722)) / (assign13540_e18725 * assign13540_e18725))), (-((assign13540_e18714 * ((-(locals.var_delta_i_dn11 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722)) / (assign13540_e18725 * assign13540_e18725))), (-((assign13540_e18714 * ((-(locals.var_delta_i_dn12 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722)) / (assign13540_e18725 * assign13540_e18725))), (-((assign13540_e18714 * ((-(locals.var_delta_i_dn13 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722)) / (assign13540_e18725 * assign13540_e18725))), (-((assign13540_e18714 * ((-(locals.var_delta_i_dn14 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722)) / (assign13540_e18725 * assign13540_e18725))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign13540_e18728, assign13540_e18728_d_n0, assign13540_e18728_d_n2, assign13540_e18728_d_n3, assign13540_e18728_d_n4, assign13540_e18728_d_n5, assign13540_e18728_d_n6, assign13540_e18728_d_n7, assign13540_e18728_d_n8, assign13540_e18728_d_n9, assign13540_e18728_d_n10, assign13540_e18728_d_n11, assign13540_e18728_d_n12, assign13540_e18728_d_n13, assign13540_e18728_d_n14,)
    }
};
        let assign13540_e18731: f64 = (assign13540_e18729 + 2.0);
        let assign13540_e18732: f64 = (1.0 / assign13540_e18731);
        (locals.var_delta_t, locals.var_delta_t_dn0, locals.var_delta_t_dn2, locals.var_delta_t_dn3, locals.var_delta_t_dn4, locals.var_delta_t_dn5, locals.var_delta_t_dn6, locals.var_delta_t_dn7, locals.var_delta_t_dn8, locals.var_delta_t_dn9, locals.var_delta_t_dn10, locals.var_delta_t_dn11, locals.var_delta_t_dn12, locals.var_delta_t_dn13, locals.var_delta_t_dn14, ) = (assign13540_e18732, (-(assign13540_e18729_d_n0 / (assign13540_e18731 * assign13540_e18731))), (-(assign13540_e18729_d_n2 / (assign13540_e18731 * assign13540_e18731))), (-(assign13540_e18729_d_n3 / (assign13540_e18731 * assign13540_e18731))), (-(assign13540_e18729_d_n4 / (assign13540_e18731 * assign13540_e18731))), (-(assign13540_e18729_d_n5 / (assign13540_e18731 * assign13540_e18731))), (-(assign13540_e18729_d_n6 / (assign13540_e18731 * assign13540_e18731))), (-(assign13540_e18729_d_n7 / (assign13540_e18731 * assign13540_e18731))), (-(assign13540_e18729_d_n8 / (assign13540_e18731 * assign13540_e18731))), (-(assign13540_e18729_d_n9 / (assign13540_e18731 * assign13540_e18731))), (-(assign13540_e18729_d_n10 / (assign13540_e18731 * assign13540_e18731))), (-(assign13540_e18729_d_n11 / (assign13540_e18731 * assign13540_e18731))), (-(assign13540_e18729_d_n12 / (assign13540_e18731 * assign13540_e18731))), (-(assign13540_e18729_d_n13 / (assign13540_e18731 * assign13540_e18731))), (-(assign13540_e18729_d_n14 / (assign13540_e18731 * assign13540_e18731))), );
        locals.var_delta_t_rv = 0.0;

        let assign13550_e18737: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
        let assign13550_e18738: f64 = (1.0 - assign13550_e18737);
        let assign13550_e18740: f64 = (assign13550_e18738 - 1e-6);
        let assign13550_e18742: f64 = (-10000.0);
        let assign13550_e18744: f64 = (assign13550_e18742 * 0.001);
        let (assign13550_e18805, assign13550_e18805_d_n4,) = {
    if (!(assign13550_e18740 < assign13550_e18744)) {
        let assign13550_e18751: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
        let assign13550_e18752: f64 = (1.0 - assign13550_e18751);
        let assign13550_e18754: f64 = (assign13550_e18752 - 1e-6);
        let assign13550_e18758: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
        let assign13550_e18759: f64 = (1.0 - assign13550_e18758);
        let assign13550_e18761: f64 = (assign13550_e18759 - 1e-6);
        let assign13550_e18765: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
        let assign13550_e18766: f64 = (1.0 - assign13550_e18765);
        let assign13550_e18768: f64 = (assign13550_e18766 - 1e-6);
        let assign13550_e18769: f64 = (assign13550_e18761 * assign13550_e18768);
        let assign13550_e18772: f64 = (4.0 * 0.001);
        let assign13550_e18774: f64 = (assign13550_e18772 * 0.001);
        let assign13550_e18775: f64 = (assign13550_e18769 + assign13550_e18774);
        let assign13550_e18776: f64 = (assign13550_e18775).sqrt();
        let assign13550_e18777: f64 = (assign13550_e18754 + assign13550_e18776);
        let assign13550_e18778: f64 = (0.5 * assign13550_e18777);
        (assign13550_e18778, (0.5 * ((-(locals.var_ptwgt_i * locals.var_deltemp_dn4)) + ((((-(locals.var_ptwgt_i * locals.var_deltemp_dn4)) * assign13550_e18768) + (assign13550_e18761 * (-(locals.var_ptwgt_i * locals.var_deltemp_dn4)))) / (2.0 * assign13550_e18776)))),)
    } else {
        let assign13550_e18782: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
        let assign13550_e18783: f64 = (1.0 - assign13550_e18782);
        let assign13550_e18785: f64 = (assign13550_e18783 - 1e-6);
        let assign13550_e18787: f64 = (-10000.0);
        let assign13550_e18789: f64 = (assign13550_e18787 * 0.001);
        let (assign13550_e18804, assign13550_e18804_d_n4,) = {
            if (assign13550_e18785 < assign13550_e18789) {
                let assign13550_e18792: f64 = (-0.001);
                let assign13550_e18794: f64 = (assign13550_e18792 * 0.001);
                let assign13550_e18798: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
                let assign13550_e18799: f64 = (1.0 - assign13550_e18798);
                let assign13550_e18801: f64 = (assign13550_e18799 - 1e-6);
                let assign13550_e18802: f64 = (assign13550_e18794 / assign13550_e18801);
                (assign13550_e18802, (-((assign13550_e18794 * (-(locals.var_ptwgt_i * locals.var_deltemp_dn4))) / (assign13550_e18801 * assign13550_e18801))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13550_e18804, assign13550_e18804_d_n4,)
    }
};
        let assign13550_e18806: f64 = (locals.var_ptwg_i * assign13550_e18805);
        (locals.var_ptwg_t, locals.var_ptwg_t_dn0, locals.var_ptwg_t_dn2, locals.var_ptwg_t_dn3, locals.var_ptwg_t_dn4, locals.var_ptwg_t_dn5, locals.var_ptwg_t_dn6, locals.var_ptwg_t_dn7, locals.var_ptwg_t_dn8, locals.var_ptwg_t_dn9, locals.var_ptwg_t_dn10, locals.var_ptwg_t_dn11, locals.var_ptwg_t_dn12, locals.var_ptwg_t_dn13, locals.var_ptwg_t_dn14, ) = (assign13550_e18806, (locals.var_ptwg_i_dn0 * assign13550_e18805), (locals.var_ptwg_i_dn2 * assign13550_e18805), (locals.var_ptwg_i_dn3 * assign13550_e18805), ((locals.var_ptwg_i_dn4 * assign13550_e18805) + (locals.var_ptwg_i * assign13550_e18805_d_n4)), (locals.var_ptwg_i_dn5 * assign13550_e18805), (locals.var_ptwg_i_dn6 * assign13550_e18805), (locals.var_ptwg_i_dn7 * assign13550_e18805), (locals.var_ptwg_i_dn8 * assign13550_e18805), (locals.var_ptwg_i_dn9 * assign13550_e18805), (locals.var_ptwg_i_dn10 * assign13550_e18805), (locals.var_ptwg_i_dn11 * assign13550_e18805), (locals.var_ptwg_i_dn12 * assign13550_e18805), (locals.var_ptwg_i_dn13 * assign13550_e18805), (locals.var_ptwg_i_dn14 * assign13550_e18805), );
        locals.var_ptwg_t_rv = 0.0;

        let assign13560_e18809: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard460 = assign13560_e18809;
        locals.var_guard460_rv = 0.0;

        if (locals.var_guard460 != 0.0) {
            let assign13570_e18815: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
            let assign13570_e18816: f64 = (1.0 - assign13570_e18815);
            let assign13570_e18818: f64 = (assign13570_e18816 - 1e-6);
            let assign13570_e18820: f64 = (-10000.0);
            let assign13570_e18822: f64 = (assign13570_e18820 * 0.001);
            let (assign13570_e18883, assign13570_e18883_d_n4,) = {
    if (!(assign13570_e18818 < assign13570_e18822)) {
        let assign13570_e18829: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
        let assign13570_e18830: f64 = (1.0 - assign13570_e18829);
        let assign13570_e18832: f64 = (assign13570_e18830 - 1e-6);
        let assign13570_e18836: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
        let assign13570_e18837: f64 = (1.0 - assign13570_e18836);
        let assign13570_e18839: f64 = (assign13570_e18837 - 1e-6);
        let assign13570_e18843: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
        let assign13570_e18844: f64 = (1.0 - assign13570_e18843);
        let assign13570_e18846: f64 = (assign13570_e18844 - 1e-6);
        let assign13570_e18847: f64 = (assign13570_e18839 * assign13570_e18846);
        let assign13570_e18850: f64 = (4.0 * 0.001);
        let assign13570_e18852: f64 = (assign13570_e18850 * 0.001);
        let assign13570_e18853: f64 = (assign13570_e18847 + assign13570_e18852);
        let assign13570_e18854: f64 = (assign13570_e18853).sqrt();
        let assign13570_e18855: f64 = (assign13570_e18832 + assign13570_e18854);
        let assign13570_e18856: f64 = (0.5 * assign13570_e18855);
        (assign13570_e18856, (0.5 * ((-(locals.var_ptwgt_i * locals.var_deltemp_dn4)) + ((((-(locals.var_ptwgt_i * locals.var_deltemp_dn4)) * assign13570_e18846) + (assign13570_e18839 * (-(locals.var_ptwgt_i * locals.var_deltemp_dn4)))) / (2.0 * assign13570_e18854)))),)
    } else {
        let assign13570_e18860: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
        let assign13570_e18861: f64 = (1.0 - assign13570_e18860);
        let assign13570_e18863: f64 = (assign13570_e18861 - 1e-6);
        let assign13570_e18865: f64 = (-10000.0);
        let assign13570_e18867: f64 = (assign13570_e18865 * 0.001);
        let (assign13570_e18882, assign13570_e18882_d_n4,) = {
            if (assign13570_e18863 < assign13570_e18867) {
                let assign13570_e18870: f64 = (-0.001);
                let assign13570_e18872: f64 = (assign13570_e18870 * 0.001);
                let assign13570_e18876: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
                let assign13570_e18877: f64 = (1.0 - assign13570_e18876);
                let assign13570_e18879: f64 = (assign13570_e18877 - 1e-6);
                let assign13570_e18880: f64 = (assign13570_e18872 / assign13570_e18879);
                (assign13570_e18880, (-((assign13570_e18872 * (-(locals.var_ptwgt_i * locals.var_deltemp_dn4))) / (assign13570_e18879 * assign13570_e18879))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13570_e18882, assign13570_e18882_d_n4,)
    }
};
            let assign13570_e18884: f64 = (locals.var_ptwgr_i * assign13570_e18883);
            (locals.var_ptwgr_t, locals.var_ptwgr_t_dn0, locals.var_ptwgr_t_dn2, locals.var_ptwgr_t_dn3, locals.var_ptwgr_t_dn4, locals.var_ptwgr_t_dn5, locals.var_ptwgr_t_dn6, locals.var_ptwgr_t_dn7, locals.var_ptwgr_t_dn8, locals.var_ptwgr_t_dn9, locals.var_ptwgr_t_dn10, locals.var_ptwgr_t_dn11, locals.var_ptwgr_t_dn12, locals.var_ptwgr_t_dn13, locals.var_ptwgr_t_dn14, ) = (assign13570_e18884, (locals.var_ptwgr_i_dn0 * assign13570_e18883), (locals.var_ptwgr_i_dn2 * assign13570_e18883), (locals.var_ptwgr_i_dn3 * assign13570_e18883), ((locals.var_ptwgr_i_dn4 * assign13570_e18883) + (locals.var_ptwgr_i * assign13570_e18883_d_n4)), (locals.var_ptwgr_i_dn5 * assign13570_e18883), (locals.var_ptwgr_i_dn6 * assign13570_e18883), (locals.var_ptwgr_i_dn7 * assign13570_e18883), (locals.var_ptwgr_i_dn8 * assign13570_e18883), (locals.var_ptwgr_i_dn9 * assign13570_e18883), (locals.var_ptwgr_i_dn10 * assign13570_e18883), (locals.var_ptwgr_i_dn11 * assign13570_e18883), (locals.var_ptwgr_i_dn12 * assign13570_e18883), (locals.var_ptwgr_i_dn13 * assign13570_e18883), (locals.var_ptwgr_i_dn14 * assign13570_e18883), );
            locals.var_ptwgr_t_rv = 0.0;
        }

        let assign13580_e18891: f64 = (locals.var_a11_i * locals.var_deltemp);
        let assign13580_e18892: f64 = (1.0 + assign13580_e18891);
        let assign13580_e18894: f64 = (assign13580_e18892 - 1e-6);
        let assign13580_e18896: f64 = (-10000.0);
        let assign13580_e18898: f64 = (assign13580_e18896 * 0.001);
        let (assign13580_e18959, assign13580_e18959_d_n4,) = {
    if (!(assign13580_e18894 < assign13580_e18898)) {
        let assign13580_e18905: f64 = (locals.var_a11_i * locals.var_deltemp);
        let assign13580_e18906: f64 = (1.0 + assign13580_e18905);
        let assign13580_e18908: f64 = (assign13580_e18906 - 1e-6);
        let assign13580_e18912: f64 = (locals.var_a11_i * locals.var_deltemp);
        let assign13580_e18913: f64 = (1.0 + assign13580_e18912);
        let assign13580_e18915: f64 = (assign13580_e18913 - 1e-6);
        let assign13580_e18919: f64 = (locals.var_a11_i * locals.var_deltemp);
        let assign13580_e18920: f64 = (1.0 + assign13580_e18919);
        let assign13580_e18922: f64 = (assign13580_e18920 - 1e-6);
        let assign13580_e18923: f64 = (assign13580_e18915 * assign13580_e18922);
        let assign13580_e18926: f64 = (4.0 * 0.001);
        let assign13580_e18928: f64 = (assign13580_e18926 * 0.001);
        let assign13580_e18929: f64 = (assign13580_e18923 + assign13580_e18928);
        let assign13580_e18930: f64 = (assign13580_e18929).sqrt();
        let assign13580_e18931: f64 = (assign13580_e18908 + assign13580_e18930);
        let assign13580_e18932: f64 = (0.5 * assign13580_e18931);
        (assign13580_e18932, (0.5 * ((locals.var_a11_i * locals.var_deltemp_dn4) + ((((locals.var_a11_i * locals.var_deltemp_dn4) * assign13580_e18922) + (assign13580_e18915 * (locals.var_a11_i * locals.var_deltemp_dn4))) / (2.0 * assign13580_e18930)))),)
    } else {
        let assign13580_e18936: f64 = (locals.var_a11_i * locals.var_deltemp);
        let assign13580_e18937: f64 = (1.0 + assign13580_e18936);
        let assign13580_e18939: f64 = (assign13580_e18937 - 1e-6);
        let assign13580_e18941: f64 = (-10000.0);
        let assign13580_e18943: f64 = (assign13580_e18941 * 0.001);
        let (assign13580_e18958, assign13580_e18958_d_n4,) = {
            if (assign13580_e18939 < assign13580_e18943) {
                let assign13580_e18946: f64 = (-0.001);
                let assign13580_e18948: f64 = (assign13580_e18946 * 0.001);
                let assign13580_e18952: f64 = (locals.var_a11_i * locals.var_deltemp);
                let assign13580_e18953: f64 = (1.0 + assign13580_e18952);
                let assign13580_e18955: f64 = (assign13580_e18953 - 1e-6);
                let assign13580_e18956: f64 = (assign13580_e18948 / assign13580_e18955);
                (assign13580_e18956, (-((assign13580_e18948 * (locals.var_a11_i * locals.var_deltemp_dn4)) / (assign13580_e18955 * assign13580_e18955))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13580_e18958, assign13580_e18958_d_n4,)
    }
};
        let assign13580_e18960: f64 = (locals.var_a1_i * assign13580_e18959);
        (locals.var_a1_t, locals.var_a1_t_dn4, ) = (assign13580_e18960, (locals.var_a1_i * assign13580_e18959_d_n4), );
        locals.var_a1_t_rv = 0.0;

        let assign13590_e18965: f64 = (locals.var_a21_i * locals.var_deltemp);
        let assign13590_e18966: f64 = (1.0 + assign13590_e18965);
        let assign13590_e18968: f64 = (assign13590_e18966 - 1e-6);
        let assign13590_e18970: f64 = (-10000.0);
        let assign13590_e18972: f64 = (assign13590_e18970 * 0.001);
        let (assign13590_e19033, assign13590_e19033_d_n4,) = {
    if (!(assign13590_e18968 < assign13590_e18972)) {
        let assign13590_e18979: f64 = (locals.var_a21_i * locals.var_deltemp);
        let assign13590_e18980: f64 = (1.0 + assign13590_e18979);
        let assign13590_e18982: f64 = (assign13590_e18980 - 1e-6);
        let assign13590_e18986: f64 = (locals.var_a21_i * locals.var_deltemp);
        let assign13590_e18987: f64 = (1.0 + assign13590_e18986);
        let assign13590_e18989: f64 = (assign13590_e18987 - 1e-6);
        let assign13590_e18993: f64 = (locals.var_a21_i * locals.var_deltemp);
        let assign13590_e18994: f64 = (1.0 + assign13590_e18993);
        let assign13590_e18996: f64 = (assign13590_e18994 - 1e-6);
        let assign13590_e18997: f64 = (assign13590_e18989 * assign13590_e18996);
        let assign13590_e19000: f64 = (4.0 * 0.001);
        let assign13590_e19002: f64 = (assign13590_e19000 * 0.001);
        let assign13590_e19003: f64 = (assign13590_e18997 + assign13590_e19002);
        let assign13590_e19004: f64 = (assign13590_e19003).sqrt();
        let assign13590_e19005: f64 = (assign13590_e18982 + assign13590_e19004);
        let assign13590_e19006: f64 = (0.5 * assign13590_e19005);
        (assign13590_e19006, (0.5 * ((locals.var_a21_i * locals.var_deltemp_dn4) + ((((locals.var_a21_i * locals.var_deltemp_dn4) * assign13590_e18996) + (assign13590_e18989 * (locals.var_a21_i * locals.var_deltemp_dn4))) / (2.0 * assign13590_e19004)))),)
    } else {
        let assign13590_e19010: f64 = (locals.var_a21_i * locals.var_deltemp);
        let assign13590_e19011: f64 = (1.0 + assign13590_e19010);
        let assign13590_e19013: f64 = (assign13590_e19011 - 1e-6);
        let assign13590_e19015: f64 = (-10000.0);
        let assign13590_e19017: f64 = (assign13590_e19015 * 0.001);
        let (assign13590_e19032, assign13590_e19032_d_n4,) = {
            if (assign13590_e19013 < assign13590_e19017) {
                let assign13590_e19020: f64 = (-0.001);
                let assign13590_e19022: f64 = (assign13590_e19020 * 0.001);
                let assign13590_e19026: f64 = (locals.var_a21_i * locals.var_deltemp);
                let assign13590_e19027: f64 = (1.0 + assign13590_e19026);
                let assign13590_e19029: f64 = (assign13590_e19027 - 1e-6);
                let assign13590_e19030: f64 = (assign13590_e19022 / assign13590_e19029);
                (assign13590_e19030, (-((assign13590_e19022 * (locals.var_a21_i * locals.var_deltemp_dn4)) / (assign13590_e19029 * assign13590_e19029))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13590_e19032, assign13590_e19032_d_n4,)
    }
};
        let assign13590_e19034: f64 = (locals.var_a2_i * assign13590_e19033);
        (locals.var_a2_t, locals.var_a2_t_dn4, ) = (assign13590_e19034, (locals.var_a2_i * assign13590_e19033_d_n4), );
        locals.var_a2_t_rv = 0.0;

        let assign13600_e19038: f64 = (locals.var_tratio).powf(locals.var_iit_i);
        let assign13600_e19039: f64 = (locals.var_beta0_i * assign13600_e19038);
        (locals.var_beta0_t, locals.var_beta0_t_dn0, locals.var_beta0_t_dn2, locals.var_beta0_t_dn3, locals.var_beta0_t_dn4, locals.var_beta0_t_dn5, locals.var_beta0_t_dn6, locals.var_beta0_t_dn7, locals.var_beta0_t_dn8, locals.var_beta0_t_dn9, locals.var_beta0_t_dn10, locals.var_beta0_t_dn11, locals.var_beta0_t_dn12, locals.var_beta0_t_dn13, locals.var_beta0_t_dn14, ) = (assign13600_e19039, (locals.var_beta0_i_dn0 * assign13600_e19038), (locals.var_beta0_i_dn2 * assign13600_e19038), (locals.var_beta0_i_dn3 * assign13600_e19038), ((locals.var_beta0_i_dn4 * assign13600_e19038) + (locals.var_beta0_i * if 0.0 == 0.0 && ((locals.var_iit_i) as f64).is_finite() && ((locals.var_iit_i) as f64).fract() == 0.0 { if locals.var_iit_i == 0.0 { 0.0 } else { (locals.var_iit_i * ((locals.var_tratio).powf(locals.var_iit_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13600_e19038 * (locals.var_iit_i * (locals.var_tratio_dn4 / locals.var_tratio))) })), (locals.var_beta0_i_dn5 * assign13600_e19038), (locals.var_beta0_i_dn6 * assign13600_e19038), (locals.var_beta0_i_dn7 * assign13600_e19038), (locals.var_beta0_i_dn8 * assign13600_e19038), (locals.var_beta0_i_dn9 * assign13600_e19038), (locals.var_beta0_i_dn10 * assign13600_e19038), (locals.var_beta0_i_dn11 * assign13600_e19038), (locals.var_beta0_i_dn12 * assign13600_e19038), (locals.var_beta0_i_dn13 * assign13600_e19038), (locals.var_beta0_i_dn14 * assign13600_e19038), );
        locals.var_beta0_t_rv = 0.0;

        let assign13610_e19042: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard461 = assign13610_e19042;
        locals.var_guard461_rv = 0.0;

        if (locals.var_guard461 != 0.0) {
            let assign13620_e19047: f64 = (locals.var_tratio).powf(locals.var_iit_i);
            let assign13620_e19048: f64 = (locals.var_beta0r_i * assign13620_e19047);
            (locals.var_beta0r_t, locals.var_beta0r_t_dn4, ) = (assign13620_e19048, (locals.var_beta0r_i * if 0.0 == 0.0 && ((locals.var_iit_i) as f64).is_finite() && ((locals.var_iit_i) as f64).fract() == 0.0 { if locals.var_iit_i == 0.0 { 0.0 } else { (locals.var_iit_i * ((locals.var_tratio).powf(locals.var_iit_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13620_e19047 * (locals.var_iit_i * (locals.var_tratio_dn4 / locals.var_tratio))) }), );
            locals.var_beta0r_t_rv = 0.0;
        }

        let assign13630_e19055: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign13630_e19056: f64 = (1.0 + assign13630_e19055);
        let assign13630_e19058: f64 = (assign13630_e19056 - 1e-6);
        let assign13630_e19060: f64 = (-10000.0);
        let assign13630_e19062: f64 = (assign13630_e19060 * 0.001);
        let (assign13630_e19123, assign13630_e19123_d_n4,) = {
    if (!(assign13630_e19058 < assign13630_e19062)) {
        let assign13630_e19069: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign13630_e19070: f64 = (1.0 + assign13630_e19069);
        let assign13630_e19072: f64 = (assign13630_e19070 - 1e-6);
        let assign13630_e19076: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign13630_e19077: f64 = (1.0 + assign13630_e19076);
        let assign13630_e19079: f64 = (assign13630_e19077 - 1e-6);
        let assign13630_e19083: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign13630_e19084: f64 = (1.0 + assign13630_e19083);
        let assign13630_e19086: f64 = (assign13630_e19084 - 1e-6);
        let assign13630_e19087: f64 = (assign13630_e19079 * assign13630_e19086);
        let assign13630_e19090: f64 = (4.0 * 0.001);
        let assign13630_e19092: f64 = (assign13630_e19090 * 0.001);
        let assign13630_e19093: f64 = (assign13630_e19087 + assign13630_e19092);
        let assign13630_e19094: f64 = (assign13630_e19093).sqrt();
        let assign13630_e19095: f64 = (assign13630_e19072 + assign13630_e19094);
        let assign13630_e19096: f64 = (0.5 * assign13630_e19095);
        (assign13630_e19096, (0.5 * ((locals.var_tgidl_i * locals.var_deltemp_dn4) + ((((locals.var_tgidl_i * locals.var_deltemp_dn4) * assign13630_e19086) + (assign13630_e19079 * (locals.var_tgidl_i * locals.var_deltemp_dn4))) / (2.0 * assign13630_e19094)))),)
    } else {
        let assign13630_e19100: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign13630_e19101: f64 = (1.0 + assign13630_e19100);
        let assign13630_e19103: f64 = (assign13630_e19101 - 1e-6);
        let assign13630_e19105: f64 = (-10000.0);
        let assign13630_e19107: f64 = (assign13630_e19105 * 0.001);
        let (assign13630_e19122, assign13630_e19122_d_n4,) = {
            if (assign13630_e19103 < assign13630_e19107) {
                let assign13630_e19110: f64 = (-0.001);
                let assign13630_e19112: f64 = (assign13630_e19110 * 0.001);
                let assign13630_e19116: f64 = (locals.var_tgidl_i * locals.var_deltemp);
                let assign13630_e19117: f64 = (1.0 + assign13630_e19116);
                let assign13630_e19119: f64 = (assign13630_e19117 - 1e-6);
                let assign13630_e19120: f64 = (assign13630_e19112 / assign13630_e19119);
                (assign13630_e19120, (-((assign13630_e19112 * (locals.var_tgidl_i * locals.var_deltemp_dn4)) / (assign13630_e19119 * assign13630_e19119))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13630_e19122, assign13630_e19122_d_n4,)
    }
};
        let assign13630_e19124: f64 = (locals.var_bgidl_i * assign13630_e19123);
        (locals.var_bgidl_t, locals.var_bgidl_t_dn4, ) = (assign13630_e19124, (locals.var_bgidl_i * assign13630_e19123_d_n4), );
        locals.var_bgidl_t_rv = 0.0;

        let assign13640_e19129: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign13640_e19130: f64 = (1.0 + assign13640_e19129);
        let assign13640_e19132: f64 = (assign13640_e19130 - 1e-6);
        let assign13640_e19134: f64 = (-10000.0);
        let assign13640_e19136: f64 = (assign13640_e19134 * 0.001);
        let (assign13640_e19197, assign13640_e19197_d_n4,) = {
    if (!(assign13640_e19132 < assign13640_e19136)) {
        let assign13640_e19143: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign13640_e19144: f64 = (1.0 + assign13640_e19143);
        let assign13640_e19146: f64 = (assign13640_e19144 - 1e-6);
        let assign13640_e19150: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign13640_e19151: f64 = (1.0 + assign13640_e19150);
        let assign13640_e19153: f64 = (assign13640_e19151 - 1e-6);
        let assign13640_e19157: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign13640_e19158: f64 = (1.0 + assign13640_e19157);
        let assign13640_e19160: f64 = (assign13640_e19158 - 1e-6);
        let assign13640_e19161: f64 = (assign13640_e19153 * assign13640_e19160);
        let assign13640_e19164: f64 = (4.0 * 0.001);
        let assign13640_e19166: f64 = (assign13640_e19164 * 0.001);
        let assign13640_e19167: f64 = (assign13640_e19161 + assign13640_e19166);
        let assign13640_e19168: f64 = (assign13640_e19167).sqrt();
        let assign13640_e19169: f64 = (assign13640_e19146 + assign13640_e19168);
        let assign13640_e19170: f64 = (0.5 * assign13640_e19169);
        (assign13640_e19170, (0.5 * ((locals.var_tgidl_i * locals.var_deltemp_dn4) + ((((locals.var_tgidl_i * locals.var_deltemp_dn4) * assign13640_e19160) + (assign13640_e19153 * (locals.var_tgidl_i * locals.var_deltemp_dn4))) / (2.0 * assign13640_e19168)))),)
    } else {
        let assign13640_e19174: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign13640_e19175: f64 = (1.0 + assign13640_e19174);
        let assign13640_e19177: f64 = (assign13640_e19175 - 1e-6);
        let assign13640_e19179: f64 = (-10000.0);
        let assign13640_e19181: f64 = (assign13640_e19179 * 0.001);
        let (assign13640_e19196, assign13640_e19196_d_n4,) = {
            if (assign13640_e19177 < assign13640_e19181) {
                let assign13640_e19184: f64 = (-0.001);
                let assign13640_e19186: f64 = (assign13640_e19184 * 0.001);
                let assign13640_e19190: f64 = (locals.var_tgidl_i * locals.var_deltemp);
                let assign13640_e19191: f64 = (1.0 + assign13640_e19190);
                let assign13640_e19193: f64 = (assign13640_e19191 - 1e-6);
                let assign13640_e19194: f64 = (assign13640_e19186 / assign13640_e19193);
                (assign13640_e19194, (-((assign13640_e19186 * (locals.var_tgidl_i * locals.var_deltemp_dn4)) / (assign13640_e19193 * assign13640_e19193))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13640_e19196, assign13640_e19196_d_n4,)
    }
};
        let assign13640_e19198: f64 = (locals.var_bgisl_i * assign13640_e19197);
        (locals.var_bgisl_t, locals.var_bgisl_t_dn4, ) = (assign13640_e19198, (locals.var_bgisl_i * assign13640_e19197_d_n4), );
        locals.var_bgisl_t_rv = 0.0;

        let assign13660_e19210: f64 = (locals.var_k01_i * locals.var_deltemp);
        let assign13660_e19211: f64 = (1.0 + assign13660_e19210);
        let assign13660_e19213: f64 = (assign13660_e19211 - 1e-6);
        let assign13660_e19215: f64 = (-10000.0);
        let assign13660_e19217: f64 = (assign13660_e19215 * 0.001);
        let (assign13660_e19278, assign13660_e19278_d_n4,) = {
    if (!(assign13660_e19213 < assign13660_e19217)) {
        let assign13660_e19224: f64 = (locals.var_k01_i * locals.var_deltemp);
        let assign13660_e19225: f64 = (1.0 + assign13660_e19224);
        let assign13660_e19227: f64 = (assign13660_e19225 - 1e-6);
        let assign13660_e19231: f64 = (locals.var_k01_i * locals.var_deltemp);
        let assign13660_e19232: f64 = (1.0 + assign13660_e19231);
        let assign13660_e19234: f64 = (assign13660_e19232 - 1e-6);
        let assign13660_e19238: f64 = (locals.var_k01_i * locals.var_deltemp);
        let assign13660_e19239: f64 = (1.0 + assign13660_e19238);
        let assign13660_e19241: f64 = (assign13660_e19239 - 1e-6);
        let assign13660_e19242: f64 = (assign13660_e19234 * assign13660_e19241);
        let assign13660_e19245: f64 = (4.0 * 0.001);
        let assign13660_e19247: f64 = (assign13660_e19245 * 0.001);
        let assign13660_e19248: f64 = (assign13660_e19242 + assign13660_e19247);
        let assign13660_e19249: f64 = (assign13660_e19248).sqrt();
        let assign13660_e19250: f64 = (assign13660_e19227 + assign13660_e19249);
        let assign13660_e19251: f64 = (0.5 * assign13660_e19250);
        (assign13660_e19251, (0.5 * ((locals.var_k01_i * locals.var_deltemp_dn4) + ((((locals.var_k01_i * locals.var_deltemp_dn4) * assign13660_e19241) + (assign13660_e19234 * (locals.var_k01_i * locals.var_deltemp_dn4))) / (2.0 * assign13660_e19249)))),)
    } else {
        let assign13660_e19255: f64 = (locals.var_k01_i * locals.var_deltemp);
        let assign13660_e19256: f64 = (1.0 + assign13660_e19255);
        let assign13660_e19258: f64 = (assign13660_e19256 - 1e-6);
        let assign13660_e19260: f64 = (-10000.0);
        let assign13660_e19262: f64 = (assign13660_e19260 * 0.001);
        let (assign13660_e19277, assign13660_e19277_d_n4,) = {
            if (assign13660_e19258 < assign13660_e19262) {
                let assign13660_e19265: f64 = (-0.001);
                let assign13660_e19267: f64 = (assign13660_e19265 * 0.001);
                let assign13660_e19271: f64 = (locals.var_k01_i * locals.var_deltemp);
                let assign13660_e19272: f64 = (1.0 + assign13660_e19271);
                let assign13660_e19274: f64 = (assign13660_e19272 - 1e-6);
                let assign13660_e19275: f64 = (assign13660_e19267 / assign13660_e19274);
                (assign13660_e19275, (-((assign13660_e19267 * (locals.var_k01_i * locals.var_deltemp_dn4)) / (assign13660_e19274 * assign13660_e19274))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13660_e19277, assign13660_e19277_d_n4,)
    }
};
        let assign13660_e19279: f64 = (locals.var_k0_i * assign13660_e19278);
        (locals.var_k0_t, locals.var_k0_t_dn4, ) = (assign13660_e19279, (locals.var_k0_i * assign13660_e19278_d_n4), );
        locals.var_k0_t_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_15(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign13670_e19284: f64 = (locals.var_m01_i * locals.var_deltemp);
        let assign13670_e19285: f64 = (1.0 + assign13670_e19284);
        let assign13670_e19287: f64 = (assign13670_e19285 - 1e-6);
        let assign13670_e19289: f64 = (-10000.0);
        let assign13670_e19291: f64 = (assign13670_e19289 * 0.001);
        let (assign13670_e19352, assign13670_e19352_d_n4,) = {
    if (!(assign13670_e19287 < assign13670_e19291)) {
        let assign13670_e19298: f64 = (locals.var_m01_i * locals.var_deltemp);
        let assign13670_e19299: f64 = (1.0 + assign13670_e19298);
        let assign13670_e19301: f64 = (assign13670_e19299 - 1e-6);
        let assign13670_e19305: f64 = (locals.var_m01_i * locals.var_deltemp);
        let assign13670_e19306: f64 = (1.0 + assign13670_e19305);
        let assign13670_e19308: f64 = (assign13670_e19306 - 1e-6);
        let assign13670_e19312: f64 = (locals.var_m01_i * locals.var_deltemp);
        let assign13670_e19313: f64 = (1.0 + assign13670_e19312);
        let assign13670_e19315: f64 = (assign13670_e19313 - 1e-6);
        let assign13670_e19316: f64 = (assign13670_e19308 * assign13670_e19315);
        let assign13670_e19319: f64 = (4.0 * 0.001);
        let assign13670_e19321: f64 = (assign13670_e19319 * 0.001);
        let assign13670_e19322: f64 = (assign13670_e19316 + assign13670_e19321);
        let assign13670_e19323: f64 = (assign13670_e19322).sqrt();
        let assign13670_e19324: f64 = (assign13670_e19301 + assign13670_e19323);
        let assign13670_e19325: f64 = (0.5 * assign13670_e19324);
        (assign13670_e19325, (0.5 * ((locals.var_m01_i * locals.var_deltemp_dn4) + ((((locals.var_m01_i * locals.var_deltemp_dn4) * assign13670_e19315) + (assign13670_e19308 * (locals.var_m01_i * locals.var_deltemp_dn4))) / (2.0 * assign13670_e19323)))),)
    } else {
        let assign13670_e19329: f64 = (locals.var_m01_i * locals.var_deltemp);
        let assign13670_e19330: f64 = (1.0 + assign13670_e19329);
        let assign13670_e19332: f64 = (assign13670_e19330 - 1e-6);
        let assign13670_e19334: f64 = (-10000.0);
        let assign13670_e19336: f64 = (assign13670_e19334 * 0.001);
        let (assign13670_e19351, assign13670_e19351_d_n4,) = {
            if (assign13670_e19332 < assign13670_e19336) {
                let assign13670_e19339: f64 = (-0.001);
                let assign13670_e19341: f64 = (assign13670_e19339 * 0.001);
                let assign13670_e19345: f64 = (locals.var_m01_i * locals.var_deltemp);
                let assign13670_e19346: f64 = (1.0 + assign13670_e19345);
                let assign13670_e19348: f64 = (assign13670_e19346 - 1e-6);
                let assign13670_e19349: f64 = (assign13670_e19341 / assign13670_e19348);
                (assign13670_e19349, (-((assign13670_e19341 * (locals.var_m01_i * locals.var_deltemp_dn4)) / (assign13670_e19348 * assign13670_e19348))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13670_e19351, assign13670_e19351_d_n4,)
    }
};
        let assign13670_e19353: f64 = (locals.var_m0_i * assign13670_e19352);
        (locals.var_m0_t, locals.var_m0_t_dn4, ) = (assign13670_e19353, (locals.var_m0_i * assign13670_e19352_d_n4), );
        locals.var_m0_t_rv = 0.0;

        let assign13680_e19358: f64 = (locals.var_c01_i * locals.var_deltemp);
        let assign13680_e19359: f64 = (1.0 + assign13680_e19358);
        let assign13680_e19361: f64 = (assign13680_e19359 - 1e-6);
        let assign13680_e19363: f64 = (-10000.0);
        let assign13680_e19365: f64 = (assign13680_e19363 * 0.001);
        let (assign13680_e19426, assign13680_e19426_d_n4,) = {
    if (!(assign13680_e19361 < assign13680_e19365)) {
        let assign13680_e19372: f64 = (locals.var_c01_i * locals.var_deltemp);
        let assign13680_e19373: f64 = (1.0 + assign13680_e19372);
        let assign13680_e19375: f64 = (assign13680_e19373 - 1e-6);
        let assign13680_e19379: f64 = (locals.var_c01_i * locals.var_deltemp);
        let assign13680_e19380: f64 = (1.0 + assign13680_e19379);
        let assign13680_e19382: f64 = (assign13680_e19380 - 1e-6);
        let assign13680_e19386: f64 = (locals.var_c01_i * locals.var_deltemp);
        let assign13680_e19387: f64 = (1.0 + assign13680_e19386);
        let assign13680_e19389: f64 = (assign13680_e19387 - 1e-6);
        let assign13680_e19390: f64 = (assign13680_e19382 * assign13680_e19389);
        let assign13680_e19393: f64 = (4.0 * 0.001);
        let assign13680_e19395: f64 = (assign13680_e19393 * 0.001);
        let assign13680_e19396: f64 = (assign13680_e19390 + assign13680_e19395);
        let assign13680_e19397: f64 = (assign13680_e19396).sqrt();
        let assign13680_e19398: f64 = (assign13680_e19375 + assign13680_e19397);
        let assign13680_e19399: f64 = (0.5 * assign13680_e19398);
        (assign13680_e19399, (0.5 * ((locals.var_c01_i * locals.var_deltemp_dn4) + ((((locals.var_c01_i * locals.var_deltemp_dn4) * assign13680_e19389) + (assign13680_e19382 * (locals.var_c01_i * locals.var_deltemp_dn4))) / (2.0 * assign13680_e19397)))),)
    } else {
        let assign13680_e19403: f64 = (locals.var_c01_i * locals.var_deltemp);
        let assign13680_e19404: f64 = (1.0 + assign13680_e19403);
        let assign13680_e19406: f64 = (assign13680_e19404 - 1e-6);
        let assign13680_e19408: f64 = (-10000.0);
        let assign13680_e19410: f64 = (assign13680_e19408 * 0.001);
        let (assign13680_e19425, assign13680_e19425_d_n4,) = {
            if (assign13680_e19406 < assign13680_e19410) {
                let assign13680_e19413: f64 = (-0.001);
                let assign13680_e19415: f64 = (assign13680_e19413 * 0.001);
                let assign13680_e19419: f64 = (locals.var_c01_i * locals.var_deltemp);
                let assign13680_e19420: f64 = (1.0 + assign13680_e19419);
                let assign13680_e19422: f64 = (assign13680_e19420 - 1e-6);
                let assign13680_e19423: f64 = (assign13680_e19415 / assign13680_e19422);
                (assign13680_e19423, (-((assign13680_e19415 * (locals.var_c01_i * locals.var_deltemp_dn4)) / (assign13680_e19422 * assign13680_e19422))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13680_e19425, assign13680_e19425_d_n4,)
    }
};
        let assign13680_e19427: f64 = (locals.var_c0_i * assign13680_e19426);
        (locals.var_c0_t, locals.var_c0_t_dn4, ) = (assign13680_e19427, (locals.var_c0_i * assign13680_e19426_d_n4), );
        locals.var_c0_t_rv = 0.0;

        let assign13690_e19432: f64 = (locals.var_c0si1_i * locals.var_deltemp);
        let assign13690_e19433: f64 = (1.0 + assign13690_e19432);
        let assign13690_e19435: f64 = (assign13690_e19433 - 1e-6);
        let assign13690_e19437: f64 = (-10000.0);
        let assign13690_e19439: f64 = (assign13690_e19437 * 0.001);
        let (assign13690_e19500, assign13690_e19500_d_n4,) = {
    if (!(assign13690_e19435 < assign13690_e19439)) {
        let assign13690_e19446: f64 = (locals.var_c0si1_i * locals.var_deltemp);
        let assign13690_e19447: f64 = (1.0 + assign13690_e19446);
        let assign13690_e19449: f64 = (assign13690_e19447 - 1e-6);
        let assign13690_e19453: f64 = (locals.var_c0si1_i * locals.var_deltemp);
        let assign13690_e19454: f64 = (1.0 + assign13690_e19453);
        let assign13690_e19456: f64 = (assign13690_e19454 - 1e-6);
        let assign13690_e19460: f64 = (locals.var_c0si1_i * locals.var_deltemp);
        let assign13690_e19461: f64 = (1.0 + assign13690_e19460);
        let assign13690_e19463: f64 = (assign13690_e19461 - 1e-6);
        let assign13690_e19464: f64 = (assign13690_e19456 * assign13690_e19463);
        let assign13690_e19467: f64 = (4.0 * 0.001);
        let assign13690_e19469: f64 = (assign13690_e19467 * 0.001);
        let assign13690_e19470: f64 = (assign13690_e19464 + assign13690_e19469);
        let assign13690_e19471: f64 = (assign13690_e19470).sqrt();
        let assign13690_e19472: f64 = (assign13690_e19449 + assign13690_e19471);
        let assign13690_e19473: f64 = (0.5 * assign13690_e19472);
        (assign13690_e19473, (0.5 * ((locals.var_c0si1_i * locals.var_deltemp_dn4) + ((((locals.var_c0si1_i * locals.var_deltemp_dn4) * assign13690_e19463) + (assign13690_e19456 * (locals.var_c0si1_i * locals.var_deltemp_dn4))) / (2.0 * assign13690_e19471)))),)
    } else {
        let assign13690_e19477: f64 = (locals.var_c0si1_i * locals.var_deltemp);
        let assign13690_e19478: f64 = (1.0 + assign13690_e19477);
        let assign13690_e19480: f64 = (assign13690_e19478 - 1e-6);
        let assign13690_e19482: f64 = (-10000.0);
        let assign13690_e19484: f64 = (assign13690_e19482 * 0.001);
        let (assign13690_e19499, assign13690_e19499_d_n4,) = {
            if (assign13690_e19480 < assign13690_e19484) {
                let assign13690_e19487: f64 = (-0.001);
                let assign13690_e19489: f64 = (assign13690_e19487 * 0.001);
                let assign13690_e19493: f64 = (locals.var_c0si1_i * locals.var_deltemp);
                let assign13690_e19494: f64 = (1.0 + assign13690_e19493);
                let assign13690_e19496: f64 = (assign13690_e19494 - 1e-6);
                let assign13690_e19497: f64 = (assign13690_e19489 / assign13690_e19496);
                (assign13690_e19497, (-((assign13690_e19489 * (locals.var_c0si1_i * locals.var_deltemp_dn4)) / (assign13690_e19496 * assign13690_e19496))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13690_e19499, assign13690_e19499_d_n4,)
    }
};
        let assign13690_e19501: f64 = (locals.var_c0si_i * assign13690_e19500);
        (locals.var_c0si_t, locals.var_c0si_t_dn4, ) = (assign13690_e19501, (locals.var_c0si_i * assign13690_e19500_d_n4), );
        locals.var_c0si_t_rv = 0.0;

        let assign13700_e19506: f64 = (locals.var_c0sisat1_i * locals.var_deltemp);
        let assign13700_e19507: f64 = (1.0 + assign13700_e19506);
        let assign13700_e19509: f64 = (assign13700_e19507 - 1e-6);
        let assign13700_e19511: f64 = (-10000.0);
        let assign13700_e19513: f64 = (assign13700_e19511 * 0.001);
        let (assign13700_e19574, assign13700_e19574_d_n4,) = {
    if (!(assign13700_e19509 < assign13700_e19513)) {
        let assign13700_e19520: f64 = (locals.var_c0sisat1_i * locals.var_deltemp);
        let assign13700_e19521: f64 = (1.0 + assign13700_e19520);
        let assign13700_e19523: f64 = (assign13700_e19521 - 1e-6);
        let assign13700_e19527: f64 = (locals.var_c0sisat1_i * locals.var_deltemp);
        let assign13700_e19528: f64 = (1.0 + assign13700_e19527);
        let assign13700_e19530: f64 = (assign13700_e19528 - 1e-6);
        let assign13700_e19534: f64 = (locals.var_c0sisat1_i * locals.var_deltemp);
        let assign13700_e19535: f64 = (1.0 + assign13700_e19534);
        let assign13700_e19537: f64 = (assign13700_e19535 - 1e-6);
        let assign13700_e19538: f64 = (assign13700_e19530 * assign13700_e19537);
        let assign13700_e19541: f64 = (4.0 * 0.001);
        let assign13700_e19543: f64 = (assign13700_e19541 * 0.001);
        let assign13700_e19544: f64 = (assign13700_e19538 + assign13700_e19543);
        let assign13700_e19545: f64 = (assign13700_e19544).sqrt();
        let assign13700_e19546: f64 = (assign13700_e19523 + assign13700_e19545);
        let assign13700_e19547: f64 = (0.5 * assign13700_e19546);
        (assign13700_e19547, (0.5 * ((locals.var_c0sisat1_i * locals.var_deltemp_dn4) + ((((locals.var_c0sisat1_i * locals.var_deltemp_dn4) * assign13700_e19537) + (assign13700_e19530 * (locals.var_c0sisat1_i * locals.var_deltemp_dn4))) / (2.0 * assign13700_e19545)))),)
    } else {
        let assign13700_e19551: f64 = (locals.var_c0sisat1_i * locals.var_deltemp);
        let assign13700_e19552: f64 = (1.0 + assign13700_e19551);
        let assign13700_e19554: f64 = (assign13700_e19552 - 1e-6);
        let assign13700_e19556: f64 = (-10000.0);
        let assign13700_e19558: f64 = (assign13700_e19556 * 0.001);
        let (assign13700_e19573, assign13700_e19573_d_n4,) = {
            if (assign13700_e19554 < assign13700_e19558) {
                let assign13700_e19561: f64 = (-0.001);
                let assign13700_e19563: f64 = (assign13700_e19561 * 0.001);
                let assign13700_e19567: f64 = (locals.var_c0sisat1_i * locals.var_deltemp);
                let assign13700_e19568: f64 = (1.0 + assign13700_e19567);
                let assign13700_e19570: f64 = (assign13700_e19568 - 1e-6);
                let assign13700_e19571: f64 = (assign13700_e19563 / assign13700_e19570);
                (assign13700_e19571, (-((assign13700_e19563 * (locals.var_c0sisat1_i * locals.var_deltemp_dn4)) / (assign13700_e19570 * assign13700_e19570))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13700_e19573, assign13700_e19573_d_n4,)
    }
};
        let assign13700_e19575: f64 = (locals.var_c0sisat_i * assign13700_e19574);
        (locals.var_c0sisat_t, locals.var_c0sisat_t_dn4, ) = (assign13700_e19575, (locals.var_c0sisat_i * assign13700_e19574_d_n4), );
        locals.var_c0sisat_t_rv = 0.0;

        let assign13710_e19580: f64 = (p.p889 * locals.var_deltemp);
        let assign13710_e19581: f64 = (1.0 + assign13710_e19580);
        let assign13710_e19583: f64 = (assign13710_e19581 - 1e-6);
        let assign13710_e19585: f64 = (-10000.0);
        let assign13710_e19587: f64 = (assign13710_e19585 * 0.001);
        let (assign13710_e19648, assign13710_e19648_d_n4,) = {
    if (!(assign13710_e19583 < assign13710_e19587)) {
        let assign13710_e19594: f64 = (p.p889 * locals.var_deltemp);
        let assign13710_e19595: f64 = (1.0 + assign13710_e19594);
        let assign13710_e19597: f64 = (assign13710_e19595 - 1e-6);
        let assign13710_e19601: f64 = (p.p889 * locals.var_deltemp);
        let assign13710_e19602: f64 = (1.0 + assign13710_e19601);
        let assign13710_e19604: f64 = (assign13710_e19602 - 1e-6);
        let assign13710_e19608: f64 = (p.p889 * locals.var_deltemp);
        let assign13710_e19609: f64 = (1.0 + assign13710_e19608);
        let assign13710_e19611: f64 = (assign13710_e19609 - 1e-6);
        let assign13710_e19612: f64 = (assign13710_e19604 * assign13710_e19611);
        let assign13710_e19615: f64 = (4.0 * 0.001);
        let assign13710_e19617: f64 = (assign13710_e19615 * 0.001);
        let assign13710_e19618: f64 = (assign13710_e19612 + assign13710_e19617);
        let assign13710_e19619: f64 = (assign13710_e19618).sqrt();
        let assign13710_e19620: f64 = (assign13710_e19597 + assign13710_e19619);
        let assign13710_e19621: f64 = (0.5 * assign13710_e19620);
        (assign13710_e19621, (0.5 * ((p.p889 * locals.var_deltemp_dn4) + ((((p.p889 * locals.var_deltemp_dn4) * assign13710_e19611) + (assign13710_e19604 * (p.p889 * locals.var_deltemp_dn4))) / (2.0 * assign13710_e19619)))),)
    } else {
        let assign13710_e19625: f64 = (p.p889 * locals.var_deltemp);
        let assign13710_e19626: f64 = (1.0 + assign13710_e19625);
        let assign13710_e19628: f64 = (assign13710_e19626 - 1e-6);
        let assign13710_e19630: f64 = (-10000.0);
        let assign13710_e19632: f64 = (assign13710_e19630 * 0.001);
        let (assign13710_e19647, assign13710_e19647_d_n4,) = {
            if (assign13710_e19628 < assign13710_e19632) {
                let assign13710_e19635: f64 = (-0.001);
                let assign13710_e19637: f64 = (assign13710_e19635 * 0.001);
                let assign13710_e19641: f64 = (p.p889 * locals.var_deltemp);
                let assign13710_e19642: f64 = (1.0 + assign13710_e19641);
                let assign13710_e19644: f64 = (assign13710_e19642 - 1e-6);
                let assign13710_e19645: f64 = (assign13710_e19637 / assign13710_e19644);
                (assign13710_e19645, (-((assign13710_e19637 * (p.p889 * locals.var_deltemp_dn4)) / (assign13710_e19644 * assign13710_e19644))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13710_e19647, assign13710_e19647_d_n4,)
    }
};
        let assign13710_e19649: f64 = (p.p701 * assign13710_e19648);
        (locals.var_cjs_t, locals.var_cjs_t_dn4, ) = (assign13710_e19649, (p.p701 * assign13710_e19648_d_n4), );
        locals.var_cjs_t_rv = 0.0;

        let assign13720_e19654: f64 = (p.p889 * locals.var_deltemp);
        let assign13720_e19655: f64 = (1.0 + assign13720_e19654);
        let assign13720_e19657: f64 = (assign13720_e19655 - 1e-6);
        let assign13720_e19659: f64 = (-10000.0);
        let assign13720_e19661: f64 = (assign13720_e19659 * 0.001);
        let (assign13720_e19722, assign13720_e19722_d_n4,) = {
    if (!(assign13720_e19657 < assign13720_e19661)) {
        let assign13720_e19668: f64 = (p.p889 * locals.var_deltemp);
        let assign13720_e19669: f64 = (1.0 + assign13720_e19668);
        let assign13720_e19671: f64 = (assign13720_e19669 - 1e-6);
        let assign13720_e19675: f64 = (p.p889 * locals.var_deltemp);
        let assign13720_e19676: f64 = (1.0 + assign13720_e19675);
        let assign13720_e19678: f64 = (assign13720_e19676 - 1e-6);
        let assign13720_e19682: f64 = (p.p889 * locals.var_deltemp);
        let assign13720_e19683: f64 = (1.0 + assign13720_e19682);
        let assign13720_e19685: f64 = (assign13720_e19683 - 1e-6);
        let assign13720_e19686: f64 = (assign13720_e19678 * assign13720_e19685);
        let assign13720_e19689: f64 = (4.0 * 0.001);
        let assign13720_e19691: f64 = (assign13720_e19689 * 0.001);
        let assign13720_e19692: f64 = (assign13720_e19686 + assign13720_e19691);
        let assign13720_e19693: f64 = (assign13720_e19692).sqrt();
        let assign13720_e19694: f64 = (assign13720_e19671 + assign13720_e19693);
        let assign13720_e19695: f64 = (0.5 * assign13720_e19694);
        (assign13720_e19695, (0.5 * ((p.p889 * locals.var_deltemp_dn4) + ((((p.p889 * locals.var_deltemp_dn4) * assign13720_e19685) + (assign13720_e19678 * (p.p889 * locals.var_deltemp_dn4))) / (2.0 * assign13720_e19693)))),)
    } else {
        let assign13720_e19699: f64 = (p.p889 * locals.var_deltemp);
        let assign13720_e19700: f64 = (1.0 + assign13720_e19699);
        let assign13720_e19702: f64 = (assign13720_e19700 - 1e-6);
        let assign13720_e19704: f64 = (-10000.0);
        let assign13720_e19706: f64 = (assign13720_e19704 * 0.001);
        let (assign13720_e19721, assign13720_e19721_d_n4,) = {
            if (assign13720_e19702 < assign13720_e19706) {
                let assign13720_e19709: f64 = (-0.001);
                let assign13720_e19711: f64 = (assign13720_e19709 * 0.001);
                let assign13720_e19715: f64 = (p.p889 * locals.var_deltemp);
                let assign13720_e19716: f64 = (1.0 + assign13720_e19715);
                let assign13720_e19718: f64 = (assign13720_e19716 - 1e-6);
                let assign13720_e19719: f64 = (assign13720_e19711 / assign13720_e19718);
                (assign13720_e19719, (-((assign13720_e19711 * (p.p889 * locals.var_deltemp_dn4)) / (assign13720_e19718 * assign13720_e19718))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13720_e19721, assign13720_e19721_d_n4,)
    }
};
        let assign13720_e19723: f64 = (p.p702 * assign13720_e19722);
        (locals.var_cjd_t, locals.var_cjd_t_dn4, ) = (assign13720_e19723, (p.p702 * assign13720_e19722_d_n4), );
        locals.var_cjd_t_rv = 0.0;

        let assign13730_e19728: f64 = (p.p890 * locals.var_deltemp);
        let assign13730_e19729: f64 = (1.0 + assign13730_e19728);
        let assign13730_e19731: f64 = (assign13730_e19729 - 1e-6);
        let assign13730_e19733: f64 = (-10000.0);
        let assign13730_e19735: f64 = (assign13730_e19733 * 0.001);
        let (assign13730_e19796, assign13730_e19796_d_n4,) = {
    if (!(assign13730_e19731 < assign13730_e19735)) {
        let assign13730_e19742: f64 = (p.p890 * locals.var_deltemp);
        let assign13730_e19743: f64 = (1.0 + assign13730_e19742);
        let assign13730_e19745: f64 = (assign13730_e19743 - 1e-6);
        let assign13730_e19749: f64 = (p.p890 * locals.var_deltemp);
        let assign13730_e19750: f64 = (1.0 + assign13730_e19749);
        let assign13730_e19752: f64 = (assign13730_e19750 - 1e-6);
        let assign13730_e19756: f64 = (p.p890 * locals.var_deltemp);
        let assign13730_e19757: f64 = (1.0 + assign13730_e19756);
        let assign13730_e19759: f64 = (assign13730_e19757 - 1e-6);
        let assign13730_e19760: f64 = (assign13730_e19752 * assign13730_e19759);
        let assign13730_e19763: f64 = (4.0 * 0.001);
        let assign13730_e19765: f64 = (assign13730_e19763 * 0.001);
        let assign13730_e19766: f64 = (assign13730_e19760 + assign13730_e19765);
        let assign13730_e19767: f64 = (assign13730_e19766).sqrt();
        let assign13730_e19768: f64 = (assign13730_e19745 + assign13730_e19767);
        let assign13730_e19769: f64 = (0.5 * assign13730_e19768);
        (assign13730_e19769, (0.5 * ((p.p890 * locals.var_deltemp_dn4) + ((((p.p890 * locals.var_deltemp_dn4) * assign13730_e19759) + (assign13730_e19752 * (p.p890 * locals.var_deltemp_dn4))) / (2.0 * assign13730_e19767)))),)
    } else {
        let assign13730_e19773: f64 = (p.p890 * locals.var_deltemp);
        let assign13730_e19774: f64 = (1.0 + assign13730_e19773);
        let assign13730_e19776: f64 = (assign13730_e19774 - 1e-6);
        let assign13730_e19778: f64 = (-10000.0);
        let assign13730_e19780: f64 = (assign13730_e19778 * 0.001);
        let (assign13730_e19795, assign13730_e19795_d_n4,) = {
            if (assign13730_e19776 < assign13730_e19780) {
                let assign13730_e19783: f64 = (-0.001);
                let assign13730_e19785: f64 = (assign13730_e19783 * 0.001);
                let assign13730_e19789: f64 = (p.p890 * locals.var_deltemp);
                let assign13730_e19790: f64 = (1.0 + assign13730_e19789);
                let assign13730_e19792: f64 = (assign13730_e19790 - 1e-6);
                let assign13730_e19793: f64 = (assign13730_e19785 / assign13730_e19792);
                (assign13730_e19793, (-((assign13730_e19785 * (p.p890 * locals.var_deltemp_dn4)) / (assign13730_e19792 * assign13730_e19792))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13730_e19795, assign13730_e19795_d_n4,)
    }
};
        let assign13730_e19797: f64 = (p.p703 * assign13730_e19796);
        (locals.var_cjsws_t, locals.var_cjsws_t_dn4, ) = (assign13730_e19797, (p.p703 * assign13730_e19796_d_n4), );
        locals.var_cjsws_t_rv = 0.0;

        let assign13740_e19802: f64 = (p.p890 * locals.var_deltemp);
        let assign13740_e19803: f64 = (1.0 + assign13740_e19802);
        let assign13740_e19805: f64 = (assign13740_e19803 - 1e-6);
        let assign13740_e19807: f64 = (-10000.0);
        let assign13740_e19809: f64 = (assign13740_e19807 * 0.001);
        let (assign13740_e19870, assign13740_e19870_d_n4,) = {
    if (!(assign13740_e19805 < assign13740_e19809)) {
        let assign13740_e19816: f64 = (p.p890 * locals.var_deltemp);
        let assign13740_e19817: f64 = (1.0 + assign13740_e19816);
        let assign13740_e19819: f64 = (assign13740_e19817 - 1e-6);
        let assign13740_e19823: f64 = (p.p890 * locals.var_deltemp);
        let assign13740_e19824: f64 = (1.0 + assign13740_e19823);
        let assign13740_e19826: f64 = (assign13740_e19824 - 1e-6);
        let assign13740_e19830: f64 = (p.p890 * locals.var_deltemp);
        let assign13740_e19831: f64 = (1.0 + assign13740_e19830);
        let assign13740_e19833: f64 = (assign13740_e19831 - 1e-6);
        let assign13740_e19834: f64 = (assign13740_e19826 * assign13740_e19833);
        let assign13740_e19837: f64 = (4.0 * 0.001);
        let assign13740_e19839: f64 = (assign13740_e19837 * 0.001);
        let assign13740_e19840: f64 = (assign13740_e19834 + assign13740_e19839);
        let assign13740_e19841: f64 = (assign13740_e19840).sqrt();
        let assign13740_e19842: f64 = (assign13740_e19819 + assign13740_e19841);
        let assign13740_e19843: f64 = (0.5 * assign13740_e19842);
        (assign13740_e19843, (0.5 * ((p.p890 * locals.var_deltemp_dn4) + ((((p.p890 * locals.var_deltemp_dn4) * assign13740_e19833) + (assign13740_e19826 * (p.p890 * locals.var_deltemp_dn4))) / (2.0 * assign13740_e19841)))),)
    } else {
        let assign13740_e19847: f64 = (p.p890 * locals.var_deltemp);
        let assign13740_e19848: f64 = (1.0 + assign13740_e19847);
        let assign13740_e19850: f64 = (assign13740_e19848 - 1e-6);
        let assign13740_e19852: f64 = (-10000.0);
        let assign13740_e19854: f64 = (assign13740_e19852 * 0.001);
        let (assign13740_e19869, assign13740_e19869_d_n4,) = {
            if (assign13740_e19850 < assign13740_e19854) {
                let assign13740_e19857: f64 = (-0.001);
                let assign13740_e19859: f64 = (assign13740_e19857 * 0.001);
                let assign13740_e19863: f64 = (p.p890 * locals.var_deltemp);
                let assign13740_e19864: f64 = (1.0 + assign13740_e19863);
                let assign13740_e19866: f64 = (assign13740_e19864 - 1e-6);
                let assign13740_e19867: f64 = (assign13740_e19859 / assign13740_e19866);
                (assign13740_e19867, (-((assign13740_e19859 * (p.p890 * locals.var_deltemp_dn4)) / (assign13740_e19866 * assign13740_e19866))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13740_e19869, assign13740_e19869_d_n4,)
    }
};
        let assign13740_e19871: f64 = (p.p704 * assign13740_e19870);
        (locals.var_cjswd_t, locals.var_cjswd_t_dn4, ) = (assign13740_e19871, (p.p704 * assign13740_e19870_d_n4), );
        locals.var_cjswd_t_rv = 0.0;

        let assign13750_e19876: f64 = (p.p891 * locals.var_deltemp);
        let assign13750_e19877: f64 = (1.0 + assign13750_e19876);
        let assign13750_e19879: f64 = (assign13750_e19877 - 1e-6);
        let assign13750_e19881: f64 = (-10000.0);
        let assign13750_e19883: f64 = (assign13750_e19881 * 0.001);
        let (assign13750_e19944, assign13750_e19944_d_n4,) = {
    if (!(assign13750_e19879 < assign13750_e19883)) {
        let assign13750_e19890: f64 = (p.p891 * locals.var_deltemp);
        let assign13750_e19891: f64 = (1.0 + assign13750_e19890);
        let assign13750_e19893: f64 = (assign13750_e19891 - 1e-6);
        let assign13750_e19897: f64 = (p.p891 * locals.var_deltemp);
        let assign13750_e19898: f64 = (1.0 + assign13750_e19897);
        let assign13750_e19900: f64 = (assign13750_e19898 - 1e-6);
        let assign13750_e19904: f64 = (p.p891 * locals.var_deltemp);
        let assign13750_e19905: f64 = (1.0 + assign13750_e19904);
        let assign13750_e19907: f64 = (assign13750_e19905 - 1e-6);
        let assign13750_e19908: f64 = (assign13750_e19900 * assign13750_e19907);
        let assign13750_e19911: f64 = (4.0 * 0.001);
        let assign13750_e19913: f64 = (assign13750_e19911 * 0.001);
        let assign13750_e19914: f64 = (assign13750_e19908 + assign13750_e19913);
        let assign13750_e19915: f64 = (assign13750_e19914).sqrt();
        let assign13750_e19916: f64 = (assign13750_e19893 + assign13750_e19915);
        let assign13750_e19917: f64 = (0.5 * assign13750_e19916);
        (assign13750_e19917, (0.5 * ((p.p891 * locals.var_deltemp_dn4) + ((((p.p891 * locals.var_deltemp_dn4) * assign13750_e19907) + (assign13750_e19900 * (p.p891 * locals.var_deltemp_dn4))) / (2.0 * assign13750_e19915)))),)
    } else {
        let assign13750_e19921: f64 = (p.p891 * locals.var_deltemp);
        let assign13750_e19922: f64 = (1.0 + assign13750_e19921);
        let assign13750_e19924: f64 = (assign13750_e19922 - 1e-6);
        let assign13750_e19926: f64 = (-10000.0);
        let assign13750_e19928: f64 = (assign13750_e19926 * 0.001);
        let (assign13750_e19943, assign13750_e19943_d_n4,) = {
            if (assign13750_e19924 < assign13750_e19928) {
                let assign13750_e19931: f64 = (-0.001);
                let assign13750_e19933: f64 = (assign13750_e19931 * 0.001);
                let assign13750_e19937: f64 = (p.p891 * locals.var_deltemp);
                let assign13750_e19938: f64 = (1.0 + assign13750_e19937);
                let assign13750_e19940: f64 = (assign13750_e19938 - 1e-6);
                let assign13750_e19941: f64 = (assign13750_e19933 / assign13750_e19940);
                (assign13750_e19941, (-((assign13750_e19933 * (p.p891 * locals.var_deltemp_dn4)) / (assign13750_e19940 * assign13750_e19940))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13750_e19943, assign13750_e19943_d_n4,)
    }
};
        let assign13750_e19945: f64 = (p.p705 * assign13750_e19944);
        (locals.var_cjswgs_t, locals.var_cjswgs_t_dn4, ) = (assign13750_e19945, (p.p705 * assign13750_e19944_d_n4), );
        locals.var_cjswgs_t_rv = 0.0;

        let assign13760_e19950: f64 = (p.p891 * locals.var_deltemp);
        let assign13760_e19951: f64 = (1.0 + assign13760_e19950);
        let assign13760_e19953: f64 = (assign13760_e19951 - 1e-6);
        let assign13760_e19955: f64 = (-10000.0);
        let assign13760_e19957: f64 = (assign13760_e19955 * 0.001);
        let (assign13760_e20018, assign13760_e20018_d_n4,) = {
    if (!(assign13760_e19953 < assign13760_e19957)) {
        let assign13760_e19964: f64 = (p.p891 * locals.var_deltemp);
        let assign13760_e19965: f64 = (1.0 + assign13760_e19964);
        let assign13760_e19967: f64 = (assign13760_e19965 - 1e-6);
        let assign13760_e19971: f64 = (p.p891 * locals.var_deltemp);
        let assign13760_e19972: f64 = (1.0 + assign13760_e19971);
        let assign13760_e19974: f64 = (assign13760_e19972 - 1e-6);
        let assign13760_e19978: f64 = (p.p891 * locals.var_deltemp);
        let assign13760_e19979: f64 = (1.0 + assign13760_e19978);
        let assign13760_e19981: f64 = (assign13760_e19979 - 1e-6);
        let assign13760_e19982: f64 = (assign13760_e19974 * assign13760_e19981);
        let assign13760_e19985: f64 = (4.0 * 0.001);
        let assign13760_e19987: f64 = (assign13760_e19985 * 0.001);
        let assign13760_e19988: f64 = (assign13760_e19982 + assign13760_e19987);
        let assign13760_e19989: f64 = (assign13760_e19988).sqrt();
        let assign13760_e19990: f64 = (assign13760_e19967 + assign13760_e19989);
        let assign13760_e19991: f64 = (0.5 * assign13760_e19990);
        (assign13760_e19991, (0.5 * ((p.p891 * locals.var_deltemp_dn4) + ((((p.p891 * locals.var_deltemp_dn4) * assign13760_e19981) + (assign13760_e19974 * (p.p891 * locals.var_deltemp_dn4))) / (2.0 * assign13760_e19989)))),)
    } else {
        let assign13760_e19995: f64 = (p.p891 * locals.var_deltemp);
        let assign13760_e19996: f64 = (1.0 + assign13760_e19995);
        let assign13760_e19998: f64 = (assign13760_e19996 - 1e-6);
        let assign13760_e20000: f64 = (-10000.0);
        let assign13760_e20002: f64 = (assign13760_e20000 * 0.001);
        let (assign13760_e20017, assign13760_e20017_d_n4,) = {
            if (assign13760_e19998 < assign13760_e20002) {
                let assign13760_e20005: f64 = (-0.001);
                let assign13760_e20007: f64 = (assign13760_e20005 * 0.001);
                let assign13760_e20011: f64 = (p.p891 * locals.var_deltemp);
                let assign13760_e20012: f64 = (1.0 + assign13760_e20011);
                let assign13760_e20014: f64 = (assign13760_e20012 - 1e-6);
                let assign13760_e20015: f64 = (assign13760_e20007 / assign13760_e20014);
                (assign13760_e20015, (-((assign13760_e20007 * (p.p891 * locals.var_deltemp_dn4)) / (assign13760_e20014 * assign13760_e20014))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13760_e20017, assign13760_e20017_d_n4,)
    }
};
        let assign13760_e20019: f64 = (p.p706 * assign13760_e20018);
        (locals.var_cjswgd_t, locals.var_cjswgd_t_dn4, ) = (assign13760_e20019, (p.p706 * assign13760_e20018_d_n4), );
        locals.var_cjswgd_t_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_16(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign13770_e20023: f64 = (p.p892 * locals.var_deltemp);
        let assign13770_e20024: f64 = (p.p707 - assign13770_e20023);
        let assign13770_e20026: f64 = (assign13770_e20024 - 0.01);
        let assign13770_e20028: f64 = (-10000.0);
        let assign13770_e20030: f64 = (assign13770_e20028 * 0.001);
        let (assign13770_e20091, assign13770_e20091_d_n4,) = {
    if (!(assign13770_e20026 < assign13770_e20030)) {
        let assign13770_e20037: f64 = (p.p892 * locals.var_deltemp);
        let assign13770_e20038: f64 = (p.p707 - assign13770_e20037);
        let assign13770_e20040: f64 = (assign13770_e20038 - 0.01);
        let assign13770_e20044: f64 = (p.p892 * locals.var_deltemp);
        let assign13770_e20045: f64 = (p.p707 - assign13770_e20044);
        let assign13770_e20047: f64 = (assign13770_e20045 - 0.01);
        let assign13770_e20051: f64 = (p.p892 * locals.var_deltemp);
        let assign13770_e20052: f64 = (p.p707 - assign13770_e20051);
        let assign13770_e20054: f64 = (assign13770_e20052 - 0.01);
        let assign13770_e20055: f64 = (assign13770_e20047 * assign13770_e20054);
        let assign13770_e20058: f64 = (4.0 * 0.001);
        let assign13770_e20060: f64 = (assign13770_e20058 * 0.001);
        let assign13770_e20061: f64 = (assign13770_e20055 + assign13770_e20060);
        let assign13770_e20062: f64 = (assign13770_e20061).sqrt();
        let assign13770_e20063: f64 = (assign13770_e20040 + assign13770_e20062);
        let assign13770_e20064: f64 = (0.5 * assign13770_e20063);
        (assign13770_e20064, (0.5 * ((-(p.p892 * locals.var_deltemp_dn4)) + ((((-(p.p892 * locals.var_deltemp_dn4)) * assign13770_e20054) + (assign13770_e20047 * (-(p.p892 * locals.var_deltemp_dn4)))) / (2.0 * assign13770_e20062)))),)
    } else {
        let assign13770_e20068: f64 = (p.p892 * locals.var_deltemp);
        let assign13770_e20069: f64 = (p.p707 - assign13770_e20068);
        let assign13770_e20071: f64 = (assign13770_e20069 - 0.01);
        let assign13770_e20073: f64 = (-10000.0);
        let assign13770_e20075: f64 = (assign13770_e20073 * 0.001);
        let (assign13770_e20090, assign13770_e20090_d_n4,) = {
            if (assign13770_e20071 < assign13770_e20075) {
                let assign13770_e20078: f64 = (-0.001);
                let assign13770_e20080: f64 = (assign13770_e20078 * 0.001);
                let assign13770_e20084: f64 = (p.p892 * locals.var_deltemp);
                let assign13770_e20085: f64 = (p.p707 - assign13770_e20084);
                let assign13770_e20087: f64 = (assign13770_e20085 - 0.01);
                let assign13770_e20088: f64 = (assign13770_e20080 / assign13770_e20087);
                (assign13770_e20088, (-((assign13770_e20080 * (-(p.p892 * locals.var_deltemp_dn4))) / (assign13770_e20087 * assign13770_e20087))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13770_e20090, assign13770_e20090_d_n4,)
    }
};
        let assign13770_e20093: f64 = (assign13770_e20091 + 0.01);
        (locals.var_pbs_t, locals.var_pbs_t_dn4, ) = (assign13770_e20093, assign13770_e20091_d_n4, );
        locals.var_pbs_t_rv = 0.0;

        let assign13780_e20097: f64 = (p.p892 * locals.var_deltemp);
        let assign13780_e20098: f64 = (p.p708 - assign13780_e20097);
        let assign13780_e20100: f64 = (assign13780_e20098 - 0.01);
        let assign13780_e20102: f64 = (-10000.0);
        let assign13780_e20104: f64 = (assign13780_e20102 * 0.001);
        let (assign13780_e20165, assign13780_e20165_d_n4,) = {
    if (!(assign13780_e20100 < assign13780_e20104)) {
        let assign13780_e20111: f64 = (p.p892 * locals.var_deltemp);
        let assign13780_e20112: f64 = (p.p708 - assign13780_e20111);
        let assign13780_e20114: f64 = (assign13780_e20112 - 0.01);
        let assign13780_e20118: f64 = (p.p892 * locals.var_deltemp);
        let assign13780_e20119: f64 = (p.p708 - assign13780_e20118);
        let assign13780_e20121: f64 = (assign13780_e20119 - 0.01);
        let assign13780_e20125: f64 = (p.p892 * locals.var_deltemp);
        let assign13780_e20126: f64 = (p.p708 - assign13780_e20125);
        let assign13780_e20128: f64 = (assign13780_e20126 - 0.01);
        let assign13780_e20129: f64 = (assign13780_e20121 * assign13780_e20128);
        let assign13780_e20132: f64 = (4.0 * 0.001);
        let assign13780_e20134: f64 = (assign13780_e20132 * 0.001);
        let assign13780_e20135: f64 = (assign13780_e20129 + assign13780_e20134);
        let assign13780_e20136: f64 = (assign13780_e20135).sqrt();
        let assign13780_e20137: f64 = (assign13780_e20114 + assign13780_e20136);
        let assign13780_e20138: f64 = (0.5 * assign13780_e20137);
        (assign13780_e20138, (0.5 * ((-(p.p892 * locals.var_deltemp_dn4)) + ((((-(p.p892 * locals.var_deltemp_dn4)) * assign13780_e20128) + (assign13780_e20121 * (-(p.p892 * locals.var_deltemp_dn4)))) / (2.0 * assign13780_e20136)))),)
    } else {
        let assign13780_e20142: f64 = (p.p892 * locals.var_deltemp);
        let assign13780_e20143: f64 = (p.p708 - assign13780_e20142);
        let assign13780_e20145: f64 = (assign13780_e20143 - 0.01);
        let assign13780_e20147: f64 = (-10000.0);
        let assign13780_e20149: f64 = (assign13780_e20147 * 0.001);
        let (assign13780_e20164, assign13780_e20164_d_n4,) = {
            if (assign13780_e20145 < assign13780_e20149) {
                let assign13780_e20152: f64 = (-0.001);
                let assign13780_e20154: f64 = (assign13780_e20152 * 0.001);
                let assign13780_e20158: f64 = (p.p892 * locals.var_deltemp);
                let assign13780_e20159: f64 = (p.p708 - assign13780_e20158);
                let assign13780_e20161: f64 = (assign13780_e20159 - 0.01);
                let assign13780_e20162: f64 = (assign13780_e20154 / assign13780_e20161);
                (assign13780_e20162, (-((assign13780_e20154 * (-(p.p892 * locals.var_deltemp_dn4))) / (assign13780_e20161 * assign13780_e20161))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13780_e20164, assign13780_e20164_d_n4,)
    }
};
        let assign13780_e20167: f64 = (assign13780_e20165 + 0.01);
        (locals.var_pbd_t, locals.var_pbd_t_dn4, ) = (assign13780_e20167, assign13780_e20165_d_n4, );
        locals.var_pbd_t_rv = 0.0;

        let assign13790_e20171: f64 = (p.p893 * locals.var_deltemp);
        let assign13790_e20172: f64 = (p.p709 - assign13790_e20171);
        let assign13790_e20174: f64 = (assign13790_e20172 - 0.01);
        let assign13790_e20176: f64 = (-10000.0);
        let assign13790_e20178: f64 = (assign13790_e20176 * 0.001);
        let (assign13790_e20239, assign13790_e20239_d_n4,) = {
    if (!(assign13790_e20174 < assign13790_e20178)) {
        let assign13790_e20185: f64 = (p.p893 * locals.var_deltemp);
        let assign13790_e20186: f64 = (p.p709 - assign13790_e20185);
        let assign13790_e20188: f64 = (assign13790_e20186 - 0.01);
        let assign13790_e20192: f64 = (p.p893 * locals.var_deltemp);
        let assign13790_e20193: f64 = (p.p709 - assign13790_e20192);
        let assign13790_e20195: f64 = (assign13790_e20193 - 0.01);
        let assign13790_e20199: f64 = (p.p893 * locals.var_deltemp);
        let assign13790_e20200: f64 = (p.p709 - assign13790_e20199);
        let assign13790_e20202: f64 = (assign13790_e20200 - 0.01);
        let assign13790_e20203: f64 = (assign13790_e20195 * assign13790_e20202);
        let assign13790_e20206: f64 = (4.0 * 0.001);
        let assign13790_e20208: f64 = (assign13790_e20206 * 0.001);
        let assign13790_e20209: f64 = (assign13790_e20203 + assign13790_e20208);
        let assign13790_e20210: f64 = (assign13790_e20209).sqrt();
        let assign13790_e20211: f64 = (assign13790_e20188 + assign13790_e20210);
        let assign13790_e20212: f64 = (0.5 * assign13790_e20211);
        (assign13790_e20212, (0.5 * ((-(p.p893 * locals.var_deltemp_dn4)) + ((((-(p.p893 * locals.var_deltemp_dn4)) * assign13790_e20202) + (assign13790_e20195 * (-(p.p893 * locals.var_deltemp_dn4)))) / (2.0 * assign13790_e20210)))),)
    } else {
        let assign13790_e20216: f64 = (p.p893 * locals.var_deltemp);
        let assign13790_e20217: f64 = (p.p709 - assign13790_e20216);
        let assign13790_e20219: f64 = (assign13790_e20217 - 0.01);
        let assign13790_e20221: f64 = (-10000.0);
        let assign13790_e20223: f64 = (assign13790_e20221 * 0.001);
        let (assign13790_e20238, assign13790_e20238_d_n4,) = {
            if (assign13790_e20219 < assign13790_e20223) {
                let assign13790_e20226: f64 = (-0.001);
                let assign13790_e20228: f64 = (assign13790_e20226 * 0.001);
                let assign13790_e20232: f64 = (p.p893 * locals.var_deltemp);
                let assign13790_e20233: f64 = (p.p709 - assign13790_e20232);
                let assign13790_e20235: f64 = (assign13790_e20233 - 0.01);
                let assign13790_e20236: f64 = (assign13790_e20228 / assign13790_e20235);
                (assign13790_e20236, (-((assign13790_e20228 * (-(p.p893 * locals.var_deltemp_dn4))) / (assign13790_e20235 * assign13790_e20235))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13790_e20238, assign13790_e20238_d_n4,)
    }
};
        let assign13790_e20241: f64 = (assign13790_e20239 + 0.01);
        (locals.var_pbsws_t, locals.var_pbsws_t_dn4, ) = (assign13790_e20241, assign13790_e20239_d_n4, );
        locals.var_pbsws_t_rv = 0.0;

        let assign13800_e20245: f64 = (p.p893 * locals.var_deltemp);
        let assign13800_e20246: f64 = (p.p710 - assign13800_e20245);
        let assign13800_e20248: f64 = (assign13800_e20246 - 0.01);
        let assign13800_e20250: f64 = (-10000.0);
        let assign13800_e20252: f64 = (assign13800_e20250 * 0.001);
        let (assign13800_e20313, assign13800_e20313_d_n4,) = {
    if (!(assign13800_e20248 < assign13800_e20252)) {
        let assign13800_e20259: f64 = (p.p893 * locals.var_deltemp);
        let assign13800_e20260: f64 = (p.p710 - assign13800_e20259);
        let assign13800_e20262: f64 = (assign13800_e20260 - 0.01);
        let assign13800_e20266: f64 = (p.p893 * locals.var_deltemp);
        let assign13800_e20267: f64 = (p.p710 - assign13800_e20266);
        let assign13800_e20269: f64 = (assign13800_e20267 - 0.01);
        let assign13800_e20273: f64 = (p.p893 * locals.var_deltemp);
        let assign13800_e20274: f64 = (p.p710 - assign13800_e20273);
        let assign13800_e20276: f64 = (assign13800_e20274 - 0.01);
        let assign13800_e20277: f64 = (assign13800_e20269 * assign13800_e20276);
        let assign13800_e20280: f64 = (4.0 * 0.001);
        let assign13800_e20282: f64 = (assign13800_e20280 * 0.001);
        let assign13800_e20283: f64 = (assign13800_e20277 + assign13800_e20282);
        let assign13800_e20284: f64 = (assign13800_e20283).sqrt();
        let assign13800_e20285: f64 = (assign13800_e20262 + assign13800_e20284);
        let assign13800_e20286: f64 = (0.5 * assign13800_e20285);
        (assign13800_e20286, (0.5 * ((-(p.p893 * locals.var_deltemp_dn4)) + ((((-(p.p893 * locals.var_deltemp_dn4)) * assign13800_e20276) + (assign13800_e20269 * (-(p.p893 * locals.var_deltemp_dn4)))) / (2.0 * assign13800_e20284)))),)
    } else {
        let assign13800_e20290: f64 = (p.p893 * locals.var_deltemp);
        let assign13800_e20291: f64 = (p.p710 - assign13800_e20290);
        let assign13800_e20293: f64 = (assign13800_e20291 - 0.01);
        let assign13800_e20295: f64 = (-10000.0);
        let assign13800_e20297: f64 = (assign13800_e20295 * 0.001);
        let (assign13800_e20312, assign13800_e20312_d_n4,) = {
            if (assign13800_e20293 < assign13800_e20297) {
                let assign13800_e20300: f64 = (-0.001);
                let assign13800_e20302: f64 = (assign13800_e20300 * 0.001);
                let assign13800_e20306: f64 = (p.p893 * locals.var_deltemp);
                let assign13800_e20307: f64 = (p.p710 - assign13800_e20306);
                let assign13800_e20309: f64 = (assign13800_e20307 - 0.01);
                let assign13800_e20310: f64 = (assign13800_e20302 / assign13800_e20309);
                (assign13800_e20310, (-((assign13800_e20302 * (-(p.p893 * locals.var_deltemp_dn4))) / (assign13800_e20309 * assign13800_e20309))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13800_e20312, assign13800_e20312_d_n4,)
    }
};
        let assign13800_e20315: f64 = (assign13800_e20313 + 0.01);
        (locals.var_pbswd_t, locals.var_pbswd_t_dn4, ) = (assign13800_e20315, assign13800_e20313_d_n4, );
        locals.var_pbswd_t_rv = 0.0;

        let assign13810_e20319: f64 = (p.p894 * locals.var_deltemp);
        let assign13810_e20320: f64 = (p.p711 - assign13810_e20319);
        let assign13810_e20322: f64 = (assign13810_e20320 - 0.01);
        let assign13810_e20324: f64 = (-10000.0);
        let assign13810_e20326: f64 = (assign13810_e20324 * 0.001);
        let (assign13810_e20387, assign13810_e20387_d_n4,) = {
    if (!(assign13810_e20322 < assign13810_e20326)) {
        let assign13810_e20333: f64 = (p.p894 * locals.var_deltemp);
        let assign13810_e20334: f64 = (p.p711 - assign13810_e20333);
        let assign13810_e20336: f64 = (assign13810_e20334 - 0.01);
        let assign13810_e20340: f64 = (p.p894 * locals.var_deltemp);
        let assign13810_e20341: f64 = (p.p711 - assign13810_e20340);
        let assign13810_e20343: f64 = (assign13810_e20341 - 0.01);
        let assign13810_e20347: f64 = (p.p894 * locals.var_deltemp);
        let assign13810_e20348: f64 = (p.p711 - assign13810_e20347);
        let assign13810_e20350: f64 = (assign13810_e20348 - 0.01);
        let assign13810_e20351: f64 = (assign13810_e20343 * assign13810_e20350);
        let assign13810_e20354: f64 = (4.0 * 0.001);
        let assign13810_e20356: f64 = (assign13810_e20354 * 0.001);
        let assign13810_e20357: f64 = (assign13810_e20351 + assign13810_e20356);
        let assign13810_e20358: f64 = (assign13810_e20357).sqrt();
        let assign13810_e20359: f64 = (assign13810_e20336 + assign13810_e20358);
        let assign13810_e20360: f64 = (0.5 * assign13810_e20359);
        (assign13810_e20360, (0.5 * ((-(p.p894 * locals.var_deltemp_dn4)) + ((((-(p.p894 * locals.var_deltemp_dn4)) * assign13810_e20350) + (assign13810_e20343 * (-(p.p894 * locals.var_deltemp_dn4)))) / (2.0 * assign13810_e20358)))),)
    } else {
        let assign13810_e20364: f64 = (p.p894 * locals.var_deltemp);
        let assign13810_e20365: f64 = (p.p711 - assign13810_e20364);
        let assign13810_e20367: f64 = (assign13810_e20365 - 0.01);
        let assign13810_e20369: f64 = (-10000.0);
        let assign13810_e20371: f64 = (assign13810_e20369 * 0.001);
        let (assign13810_e20386, assign13810_e20386_d_n4,) = {
            if (assign13810_e20367 < assign13810_e20371) {
                let assign13810_e20374: f64 = (-0.001);
                let assign13810_e20376: f64 = (assign13810_e20374 * 0.001);
                let assign13810_e20380: f64 = (p.p894 * locals.var_deltemp);
                let assign13810_e20381: f64 = (p.p711 - assign13810_e20380);
                let assign13810_e20383: f64 = (assign13810_e20381 - 0.01);
                let assign13810_e20384: f64 = (assign13810_e20376 / assign13810_e20383);
                (assign13810_e20384, (-((assign13810_e20376 * (-(p.p894 * locals.var_deltemp_dn4))) / (assign13810_e20383 * assign13810_e20383))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13810_e20386, assign13810_e20386_d_n4,)
    }
};
        let assign13810_e20389: f64 = (assign13810_e20387 + 0.01);
        (locals.var_pbswgs_t, locals.var_pbswgs_t_dn4, ) = (assign13810_e20389, assign13810_e20387_d_n4, );
        locals.var_pbswgs_t_rv = 0.0;

        let assign13820_e20393: f64 = (p.p894 * locals.var_deltemp);
        let assign13820_e20394: f64 = (p.p712 - assign13820_e20393);
        let assign13820_e20396: f64 = (assign13820_e20394 - 0.01);
        let assign13820_e20398: f64 = (-10000.0);
        let assign13820_e20400: f64 = (assign13820_e20398 * 0.001);
        let (assign13820_e20461, assign13820_e20461_d_n4,) = {
    if (!(assign13820_e20396 < assign13820_e20400)) {
        let assign13820_e20407: f64 = (p.p894 * locals.var_deltemp);
        let assign13820_e20408: f64 = (p.p712 - assign13820_e20407);
        let assign13820_e20410: f64 = (assign13820_e20408 - 0.01);
        let assign13820_e20414: f64 = (p.p894 * locals.var_deltemp);
        let assign13820_e20415: f64 = (p.p712 - assign13820_e20414);
        let assign13820_e20417: f64 = (assign13820_e20415 - 0.01);
        let assign13820_e20421: f64 = (p.p894 * locals.var_deltemp);
        let assign13820_e20422: f64 = (p.p712 - assign13820_e20421);
        let assign13820_e20424: f64 = (assign13820_e20422 - 0.01);
        let assign13820_e20425: f64 = (assign13820_e20417 * assign13820_e20424);
        let assign13820_e20428: f64 = (4.0 * 0.001);
        let assign13820_e20430: f64 = (assign13820_e20428 * 0.001);
        let assign13820_e20431: f64 = (assign13820_e20425 + assign13820_e20430);
        let assign13820_e20432: f64 = (assign13820_e20431).sqrt();
        let assign13820_e20433: f64 = (assign13820_e20410 + assign13820_e20432);
        let assign13820_e20434: f64 = (0.5 * assign13820_e20433);
        (assign13820_e20434, (0.5 * ((-(p.p894 * locals.var_deltemp_dn4)) + ((((-(p.p894 * locals.var_deltemp_dn4)) * assign13820_e20424) + (assign13820_e20417 * (-(p.p894 * locals.var_deltemp_dn4)))) / (2.0 * assign13820_e20432)))),)
    } else {
        let assign13820_e20438: f64 = (p.p894 * locals.var_deltemp);
        let assign13820_e20439: f64 = (p.p712 - assign13820_e20438);
        let assign13820_e20441: f64 = (assign13820_e20439 - 0.01);
        let assign13820_e20443: f64 = (-10000.0);
        let assign13820_e20445: f64 = (assign13820_e20443 * 0.001);
        let (assign13820_e20460, assign13820_e20460_d_n4,) = {
            if (assign13820_e20441 < assign13820_e20445) {
                let assign13820_e20448: f64 = (-0.001);
                let assign13820_e20450: f64 = (assign13820_e20448 * 0.001);
                let assign13820_e20454: f64 = (p.p894 * locals.var_deltemp);
                let assign13820_e20455: f64 = (p.p712 - assign13820_e20454);
                let assign13820_e20457: f64 = (assign13820_e20455 - 0.01);
                let assign13820_e20458: f64 = (assign13820_e20450 / assign13820_e20457);
                (assign13820_e20458, (-((assign13820_e20450 * (-(p.p894 * locals.var_deltemp_dn4))) / (assign13820_e20457 * assign13820_e20457))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13820_e20460, assign13820_e20460_d_n4,)
    }
};
        let assign13820_e20463: f64 = (assign13820_e20461 + 0.01);
        (locals.var_pbswgd_t, locals.var_pbswgd_t_dn4, ) = (assign13820_e20463, assign13820_e20461_d_n4, );
        locals.var_pbswgd_t_rv = 0.0;

        let assign13830_e20466: f64 = (locals.var_eg0 / locals.var_vtm0);
        let assign13830_e20469: f64 = (locals.var_eg / locals.var_vtm);
        let assign13830_e20470: f64 = (assign13830_e20466 - assign13830_e20469);
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign13830_e20470, 0.0, 0.0, 0.0, (-(((locals.var_eg_dn4 * locals.var_vtm) - (locals.var_eg * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_t0_rv = 0.0;

        let assign13840_e20473: f64 = (locals.var_tratio).max(1e-38);
        let assign13840_e20474: f64 = (assign13840_e20473).ln();
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, ) = (assign13840_e20474, 0.0, 0.0, 0.0, (if locals.var_tratio >= 1e-38 { locals.var_tratio_dn4 } else { 0.0 } / assign13840_e20473), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_t1_rv = 0.0;

        let assign13850_e20478: f64 = (p.p895 * locals.var_t1);
        let assign13850_e20479: f64 = (locals.var_t0 + assign13850_e20478);
        let assign13850_e20481: f64 = (assign13850_e20479 / p.p725);
        let assign13850_e20482: f64 = { let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14, ) = (assign13850_e20482, ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn0 + (p.p895 * locals.var_t1_dn0)) / p.p725)), ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn2 + (p.p895 * locals.var_t1_dn2)) / p.p725)), ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn3 + (p.p895 * locals.var_t1_dn3)) / p.p725)), ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn4 + (p.p895 * locals.var_t1_dn4)) / p.p725)), ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn5 + (p.p895 * locals.var_t1_dn5)) / p.p725)), ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn6 + (p.p895 * locals.var_t1_dn6)) / p.p725)), ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn7 + (p.p895 * locals.var_t1_dn7)) / p.p725)), ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn8 + (p.p895 * locals.var_t1_dn8)) / p.p725)), ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn9 + (p.p895 * locals.var_t1_dn9)) / p.p725)), ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn10 + (p.p895 * locals.var_t1_dn10)) / p.p725)), ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn11 + (p.p895 * locals.var_t1_dn11)) / p.p725)), ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn12 + (p.p895 * locals.var_t1_dn12)) / p.p725)), ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn13 + (p.p895 * locals.var_t1_dn13)) / p.p725)), ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn14 + (p.p895 * locals.var_t1_dn14)) / p.p725)), );
        locals.var_t3_rv = 0.0;

        let assign13860_e20485: f64 = (p.p719 * locals.var_t3);
        (locals.var_jss_t, locals.var_jss_t_dn0, locals.var_jss_t_dn2, locals.var_jss_t_dn3, locals.var_jss_t_dn4, locals.var_jss_t_dn5, locals.var_jss_t_dn6, locals.var_jss_t_dn7, locals.var_jss_t_dn8, locals.var_jss_t_dn9, locals.var_jss_t_dn10, locals.var_jss_t_dn11, locals.var_jss_t_dn12, locals.var_jss_t_dn13, locals.var_jss_t_dn14, ) = (assign13860_e20485, (p.p719 * locals.var_t3_dn0), (p.p719 * locals.var_t3_dn2), (p.p719 * locals.var_t3_dn3), (p.p719 * locals.var_t3_dn4), (p.p719 * locals.var_t3_dn5), (p.p719 * locals.var_t3_dn6), (p.p719 * locals.var_t3_dn7), (p.p719 * locals.var_t3_dn8), (p.p719 * locals.var_t3_dn9), (p.p719 * locals.var_t3_dn10), (p.p719 * locals.var_t3_dn11), (p.p719 * locals.var_t3_dn12), (p.p719 * locals.var_t3_dn13), (p.p719 * locals.var_t3_dn14), );
        locals.var_jss_t_rv = 0.0;

        let assign13870_e20488: f64 = (p.p721 * locals.var_t3);
        (locals.var_jsws_t, locals.var_jsws_t_dn0, locals.var_jsws_t_dn2, locals.var_jsws_t_dn3, locals.var_jsws_t_dn4, locals.var_jsws_t_dn5, locals.var_jsws_t_dn6, locals.var_jsws_t_dn7, locals.var_jsws_t_dn8, locals.var_jsws_t_dn9, locals.var_jsws_t_dn10, locals.var_jsws_t_dn11, locals.var_jsws_t_dn12, locals.var_jsws_t_dn13, locals.var_jsws_t_dn14, ) = (assign13870_e20488, (p.p721 * locals.var_t3_dn0), (p.p721 * locals.var_t3_dn2), (p.p721 * locals.var_t3_dn3), (p.p721 * locals.var_t3_dn4), (p.p721 * locals.var_t3_dn5), (p.p721 * locals.var_t3_dn6), (p.p721 * locals.var_t3_dn7), (p.p721 * locals.var_t3_dn8), (p.p721 * locals.var_t3_dn9), (p.p721 * locals.var_t3_dn10), (p.p721 * locals.var_t3_dn11), (p.p721 * locals.var_t3_dn12), (p.p721 * locals.var_t3_dn13), (p.p721 * locals.var_t3_dn14), );
        locals.var_jsws_t_rv = 0.0;

        let assign13880_e20491: f64 = (p.p723 * locals.var_t3);
        (locals.var_jswgs_t, locals.var_jswgs_t_dn0, locals.var_jswgs_t_dn2, locals.var_jswgs_t_dn3, locals.var_jswgs_t_dn4, locals.var_jswgs_t_dn5, locals.var_jswgs_t_dn6, locals.var_jswgs_t_dn7, locals.var_jswgs_t_dn8, locals.var_jswgs_t_dn9, locals.var_jswgs_t_dn10, locals.var_jswgs_t_dn11, locals.var_jswgs_t_dn12, locals.var_jswgs_t_dn13, locals.var_jswgs_t_dn14, ) = (assign13880_e20491, (p.p723 * locals.var_t3_dn0), (p.p723 * locals.var_t3_dn2), (p.p723 * locals.var_t3_dn3), (p.p723 * locals.var_t3_dn4), (p.p723 * locals.var_t3_dn5), (p.p723 * locals.var_t3_dn6), (p.p723 * locals.var_t3_dn7), (p.p723 * locals.var_t3_dn8), (p.p723 * locals.var_t3_dn9), (p.p723 * locals.var_t3_dn10), (p.p723 * locals.var_t3_dn11), (p.p723 * locals.var_t3_dn12), (p.p723 * locals.var_t3_dn13), (p.p723 * locals.var_t3_dn14), );
        locals.var_jswgs_t_rv = 0.0;

        let assign13890_e20495: f64 = (p.p896 * locals.var_t1);
        let assign13890_e20496: f64 = (locals.var_t0 + assign13890_e20495);
        let assign13890_e20498: f64 = (assign13890_e20496 / p.p726);
        let assign13890_e20499: f64 = { let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14, ) = (assign13890_e20499, ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn0 + (p.p896 * locals.var_t1_dn0)) / p.p726)), ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn2 + (p.p896 * locals.var_t1_dn2)) / p.p726)), ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn3 + (p.p896 * locals.var_t1_dn3)) / p.p726)), ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn4 + (p.p896 * locals.var_t1_dn4)) / p.p726)), ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn5 + (p.p896 * locals.var_t1_dn5)) / p.p726)), ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn6 + (p.p896 * locals.var_t1_dn6)) / p.p726)), ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn7 + (p.p896 * locals.var_t1_dn7)) / p.p726)), ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn8 + (p.p896 * locals.var_t1_dn8)) / p.p726)), ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn9 + (p.p896 * locals.var_t1_dn9)) / p.p726)), ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn10 + (p.p896 * locals.var_t1_dn10)) / p.p726)), ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn11 + (p.p896 * locals.var_t1_dn11)) / p.p726)), ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn12 + (p.p896 * locals.var_t1_dn12)) / p.p726)), ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn13 + (p.p896 * locals.var_t1_dn13)) / p.p726)), ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn14 + (p.p896 * locals.var_t1_dn14)) / p.p726)), );
        locals.var_t3_rv = 0.0;

        let assign13900_e20502: f64 = (p.p720 * locals.var_t3);
        (locals.var_jsd_t, locals.var_jsd_t_dn0, locals.var_jsd_t_dn2, locals.var_jsd_t_dn3, locals.var_jsd_t_dn4, locals.var_jsd_t_dn5, locals.var_jsd_t_dn6, locals.var_jsd_t_dn7, locals.var_jsd_t_dn8, locals.var_jsd_t_dn9, locals.var_jsd_t_dn10, locals.var_jsd_t_dn11, locals.var_jsd_t_dn12, locals.var_jsd_t_dn13, locals.var_jsd_t_dn14, ) = (assign13900_e20502, (p.p720 * locals.var_t3_dn0), (p.p720 * locals.var_t3_dn2), (p.p720 * locals.var_t3_dn3), (p.p720 * locals.var_t3_dn4), (p.p720 * locals.var_t3_dn5), (p.p720 * locals.var_t3_dn6), (p.p720 * locals.var_t3_dn7), (p.p720 * locals.var_t3_dn8), (p.p720 * locals.var_t3_dn9), (p.p720 * locals.var_t3_dn10), (p.p720 * locals.var_t3_dn11), (p.p720 * locals.var_t3_dn12), (p.p720 * locals.var_t3_dn13), (p.p720 * locals.var_t3_dn14), );
        locals.var_jsd_t_rv = 0.0;

        let assign13910_e20505: f64 = (p.p722 * locals.var_t3);
        (locals.var_jswd_t, locals.var_jswd_t_dn0, locals.var_jswd_t_dn2, locals.var_jswd_t_dn3, locals.var_jswd_t_dn4, locals.var_jswd_t_dn5, locals.var_jswd_t_dn6, locals.var_jswd_t_dn7, locals.var_jswd_t_dn8, locals.var_jswd_t_dn9, locals.var_jswd_t_dn10, locals.var_jswd_t_dn11, locals.var_jswd_t_dn12, locals.var_jswd_t_dn13, locals.var_jswd_t_dn14, ) = (assign13910_e20505, (p.p722 * locals.var_t3_dn0), (p.p722 * locals.var_t3_dn2), (p.p722 * locals.var_t3_dn3), (p.p722 * locals.var_t3_dn4), (p.p722 * locals.var_t3_dn5), (p.p722 * locals.var_t3_dn6), (p.p722 * locals.var_t3_dn7), (p.p722 * locals.var_t3_dn8), (p.p722 * locals.var_t3_dn9), (p.p722 * locals.var_t3_dn10), (p.p722 * locals.var_t3_dn11), (p.p722 * locals.var_t3_dn12), (p.p722 * locals.var_t3_dn13), (p.p722 * locals.var_t3_dn14), );
        locals.var_jswd_t_rv = 0.0;

        let assign13920_e20508: f64 = (p.p724 * locals.var_t3);
        (locals.var_jswgd_t, locals.var_jswgd_t_dn0, locals.var_jswgd_t_dn2, locals.var_jswgd_t_dn3, locals.var_jswgd_t_dn4, locals.var_jswgd_t_dn5, locals.var_jswgd_t_dn6, locals.var_jswgd_t_dn7, locals.var_jswgd_t_dn8, locals.var_jswgd_t_dn9, locals.var_jswgd_t_dn10, locals.var_jswgd_t_dn11, locals.var_jswgd_t_dn12, locals.var_jswgd_t_dn13, locals.var_jswgd_t_dn14, ) = (assign13920_e20508, (p.p724 * locals.var_t3_dn0), (p.p724 * locals.var_t3_dn2), (p.p724 * locals.var_t3_dn3), (p.p724 * locals.var_t3_dn4), (p.p724 * locals.var_t3_dn5), (p.p724 * locals.var_t3_dn6), (p.p724 * locals.var_t3_dn7), (p.p724 * locals.var_t3_dn8), (p.p724 * locals.var_t3_dn9), (p.p724 * locals.var_t3_dn10), (p.p724 * locals.var_t3_dn11), (p.p724 * locals.var_t3_dn12), (p.p724 * locals.var_t3_dn13), (p.p724 * locals.var_t3_dn14), );
        locals.var_jswgd_t_rv = 0.0;

        let assign13930_e20512: f64 = (locals.var_eg0 * p.p897);
        let assign13930_e20515: f64 = (locals.var_tratio - 1.0);
        let assign13930_e20516: f64 = (assign13930_e20512 * assign13930_e20515);
        let assign13930_e20518: f64 = (assign13930_e20516 / locals.var_vtm);
        let assign13930_e20519: f64 = { let limited_exp_arg = assign13930_e20518; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13930_e20520: f64 = (p.p735 * assign13930_e20519);
        (locals.var_jtss_t, locals.var_jtss_t_dn4, ) = (assign13930_e20520, (p.p735 * ({ let limited_exp_arg = assign13930_e20518; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign13930_e20512 * locals.var_tratio_dn4) * locals.var_vtm) - (assign13930_e20516 * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm)))), );
        locals.var_jtss_t_rv = 0.0;

        let assign13940_e20524: f64 = (locals.var_eg0 * p.p899);
        let assign13940_e20527: f64 = (locals.var_tratio - 1.0);
        let assign13940_e20528: f64 = (assign13940_e20524 * assign13940_e20527);
        let assign13940_e20530: f64 = (assign13940_e20528 / locals.var_vtm);
        let assign13940_e20531: f64 = { let limited_exp_arg = assign13940_e20530; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13940_e20532: f64 = (p.p737 * assign13940_e20531);
        (locals.var_jtssws_t, locals.var_jtssws_t_dn4, ) = (assign13940_e20532, (p.p737 * ({ let limited_exp_arg = assign13940_e20530; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign13940_e20524 * locals.var_tratio_dn4) * locals.var_vtm) - (assign13940_e20528 * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm)))), );
        locals.var_jtssws_t_rv = 0.0;

        let assign13950_e20536: f64 = (p.p741 / locals.var_weffcj);
        let assign13950_e20537: f64 = (assign13950_e20536).sqrt();
        let assign13950_e20539: f64 = (assign13950_e20537 + 1.0);
        let assign13950_e20540: f64 = (p.p739 * assign13950_e20539);
        let assign13950_e20543: f64 = (locals.var_eg0 * p.p901);
        let assign13950_e20546: f64 = (locals.var_tratio - 1.0);
        let assign13950_e20547: f64 = (assign13950_e20543 * assign13950_e20546);
        let assign13950_e20549: f64 = (assign13950_e20547 / locals.var_vtm);
        let assign13950_e20550: f64 = { let limited_exp_arg = assign13950_e20549; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13950_e20551: f64 = (assign13950_e20540 * assign13950_e20550);
        (locals.var_jtsswgs_t, locals.var_jtsswgs_t_dn4, ) = (assign13950_e20551, (assign13950_e20540 * ({ let limited_exp_arg = assign13950_e20549; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign13950_e20543 * locals.var_tratio_dn4) * locals.var_vtm) - (assign13950_e20547 * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm)))), );
        locals.var_jtsswgs_t_rv = 0.0;

        let assign13960_e20555: f64 = (locals.var_eg0 * p.p898);
        let assign13960_e20558: f64 = (locals.var_tratio - 1.0);
        let assign13960_e20559: f64 = (assign13960_e20555 * assign13960_e20558);
        let assign13960_e20561: f64 = (assign13960_e20559 / locals.var_vtm);
        let assign13960_e20562: f64 = { let limited_exp_arg = assign13960_e20561; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13960_e20563: f64 = (p.p736 * assign13960_e20562);
        (locals.var_jtsd_t, locals.var_jtsd_t_dn4, ) = (assign13960_e20563, (p.p736 * ({ let limited_exp_arg = assign13960_e20561; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign13960_e20555 * locals.var_tratio_dn4) * locals.var_vtm) - (assign13960_e20559 * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm)))), );
        locals.var_jtsd_t_rv = 0.0;

        let assign13970_e20567: f64 = (locals.var_eg0 * p.p900);
        let assign13970_e20570: f64 = (locals.var_tratio - 1.0);
        let assign13970_e20571: f64 = (assign13970_e20567 * assign13970_e20570);
        let assign13970_e20573: f64 = (assign13970_e20571 / locals.var_vtm);
        let assign13970_e20574: f64 = { let limited_exp_arg = assign13970_e20573; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13970_e20575: f64 = (p.p738 * assign13970_e20574);
        (locals.var_jtsswd_t, locals.var_jtsswd_t_dn4, ) = (assign13970_e20575, (p.p738 * ({ let limited_exp_arg = assign13970_e20573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign13970_e20567 * locals.var_tratio_dn4) * locals.var_vtm) - (assign13970_e20571 * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm)))), );
        locals.var_jtsswd_t_rv = 0.0;

        let assign13980_e20579: f64 = (p.p741 / locals.var_weffcj);
        let assign13980_e20580: f64 = (assign13980_e20579).sqrt();
        let assign13980_e20582: f64 = (assign13980_e20580 + 1.0);
        let assign13980_e20583: f64 = (p.p740 * assign13980_e20582);
        let assign13980_e20586: f64 = (locals.var_eg0 * p.p902);
        let assign13980_e20589: f64 = (locals.var_tratio - 1.0);
        let assign13980_e20590: f64 = (assign13980_e20586 * assign13980_e20589);
        let assign13980_e20592: f64 = (assign13980_e20590 / locals.var_vtm);
        let assign13980_e20593: f64 = { let limited_exp_arg = assign13980_e20592; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13980_e20594: f64 = (assign13980_e20583 * assign13980_e20593);
        (locals.var_jtsswgd_t, locals.var_jtsswgd_t_dn4, ) = (assign13980_e20594, (assign13980_e20583 * ({ let limited_exp_arg = assign13980_e20592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign13980_e20586 * locals.var_tratio_dn4) * locals.var_vtm) - (assign13980_e20590 * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm)))), );
        locals.var_jtsswgd_t_rv = 0.0;

        let assign13990_e20600: f64 = (locals.var_tratio - 1.0);
        let assign13990_e20601: f64 = (p.p903 * assign13990_e20600);
        let assign13990_e20602: f64 = (1.0 + assign13990_e20601);
        let assign13990_e20603: f64 = (p.p742 * assign13990_e20602);
        let assign13990_e20605: f64 = (assign13990_e20603 - 0.01);
        let assign13990_e20607: f64 = (-10000.0);
        let assign13990_e20609: f64 = (assign13990_e20607 * 0.001);
        let (assign13990_e20690, assign13990_e20690_d_n4,) = {
    if (!(assign13990_e20605 < assign13990_e20609)) {
        let assign13990_e20618: f64 = (locals.var_tratio - 1.0);
        let assign13990_e20619: f64 = (p.p903 * assign13990_e20618);
        let assign13990_e20620: f64 = (1.0 + assign13990_e20619);
        let assign13990_e20621: f64 = (p.p742 * assign13990_e20620);
        let assign13990_e20623: f64 = (assign13990_e20621 - 0.01);
        let assign13990_e20629: f64 = (locals.var_tratio - 1.0);
        let assign13990_e20630: f64 = (p.p903 * assign13990_e20629);
        let assign13990_e20631: f64 = (1.0 + assign13990_e20630);
        let assign13990_e20632: f64 = (p.p742 * assign13990_e20631);
        let assign13990_e20634: f64 = (assign13990_e20632 - 0.01);
        let assign13990_e20640: f64 = (locals.var_tratio - 1.0);
        let assign13990_e20641: f64 = (p.p903 * assign13990_e20640);
        let assign13990_e20642: f64 = (1.0 + assign13990_e20641);
        let assign13990_e20643: f64 = (p.p742 * assign13990_e20642);
        let assign13990_e20645: f64 = (assign13990_e20643 - 0.01);
        let assign13990_e20646: f64 = (assign13990_e20634 * assign13990_e20645);
        let assign13990_e20649: f64 = (4.0 * 0.001);
        let assign13990_e20651: f64 = (assign13990_e20649 * 0.001);
        let assign13990_e20652: f64 = (assign13990_e20646 + assign13990_e20651);
        let assign13990_e20653: f64 = (assign13990_e20652).sqrt();
        let assign13990_e20654: f64 = (assign13990_e20623 + assign13990_e20653);
        let assign13990_e20655: f64 = (0.5 * assign13990_e20654);
        (assign13990_e20655, (0.5 * ((p.p742 * (p.p903 * locals.var_tratio_dn4)) + ((((p.p742 * (p.p903 * locals.var_tratio_dn4)) * assign13990_e20645) + (assign13990_e20634 * (p.p742 * (p.p903 * locals.var_tratio_dn4)))) / (2.0 * assign13990_e20653)))),)
    } else {
        let assign13990_e20661: f64 = (locals.var_tratio - 1.0);
        let assign13990_e20662: f64 = (p.p903 * assign13990_e20661);
        let assign13990_e20663: f64 = (1.0 + assign13990_e20662);
        let assign13990_e20664: f64 = (p.p742 * assign13990_e20663);
        let assign13990_e20666: f64 = (assign13990_e20664 - 0.01);
        let assign13990_e20668: f64 = (-10000.0);
        let assign13990_e20670: f64 = (assign13990_e20668 * 0.001);
        let (assign13990_e20689, assign13990_e20689_d_n4,) = {
            if (assign13990_e20666 < assign13990_e20670) {
                let assign13990_e20673: f64 = (-0.001);
                let assign13990_e20675: f64 = (assign13990_e20673 * 0.001);
                let assign13990_e20681: f64 = (locals.var_tratio - 1.0);
                let assign13990_e20682: f64 = (p.p903 * assign13990_e20681);
                let assign13990_e20683: f64 = (1.0 + assign13990_e20682);
                let assign13990_e20684: f64 = (p.p742 * assign13990_e20683);
                let assign13990_e20686: f64 = (assign13990_e20684 - 0.01);
                let assign13990_e20687: f64 = (assign13990_e20675 / assign13990_e20686);
                (assign13990_e20687, (-((assign13990_e20675 * (p.p742 * (p.p903 * locals.var_tratio_dn4))) / (assign13990_e20686 * assign13990_e20686))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13990_e20689, assign13990_e20689_d_n4,)
    }
};
        let assign13990_e20692: f64 = (assign13990_e20690 + 0.01);
        (locals.var_njts_t, locals.var_njts_t_dn4, ) = (assign13990_e20692, assign13990_e20690_d_n4, );
        locals.var_njts_t_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_17(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign14000_e20698: f64 = (locals.var_tratio - 1.0);
        let assign14000_e20699: f64 = (p.p905 * assign14000_e20698);
        let assign14000_e20700: f64 = (1.0 + assign14000_e20699);
        let assign14000_e20701: f64 = (p.p744 * assign14000_e20700);
        let assign14000_e20703: f64 = (assign14000_e20701 - 0.01);
        let assign14000_e20705: f64 = (-10000.0);
        let assign14000_e20707: f64 = (assign14000_e20705 * 0.001);
        let (assign14000_e20788, assign14000_e20788_d_n4,) = {
    if (!(assign14000_e20703 < assign14000_e20707)) {
        let assign14000_e20716: f64 = (locals.var_tratio - 1.0);
        let assign14000_e20717: f64 = (p.p905 * assign14000_e20716);
        let assign14000_e20718: f64 = (1.0 + assign14000_e20717);
        let assign14000_e20719: f64 = (p.p744 * assign14000_e20718);
        let assign14000_e20721: f64 = (assign14000_e20719 - 0.01);
        let assign14000_e20727: f64 = (locals.var_tratio - 1.0);
        let assign14000_e20728: f64 = (p.p905 * assign14000_e20727);
        let assign14000_e20729: f64 = (1.0 + assign14000_e20728);
        let assign14000_e20730: f64 = (p.p744 * assign14000_e20729);
        let assign14000_e20732: f64 = (assign14000_e20730 - 0.01);
        let assign14000_e20738: f64 = (locals.var_tratio - 1.0);
        let assign14000_e20739: f64 = (p.p905 * assign14000_e20738);
        let assign14000_e20740: f64 = (1.0 + assign14000_e20739);
        let assign14000_e20741: f64 = (p.p744 * assign14000_e20740);
        let assign14000_e20743: f64 = (assign14000_e20741 - 0.01);
        let assign14000_e20744: f64 = (assign14000_e20732 * assign14000_e20743);
        let assign14000_e20747: f64 = (4.0 * 0.001);
        let assign14000_e20749: f64 = (assign14000_e20747 * 0.001);
        let assign14000_e20750: f64 = (assign14000_e20744 + assign14000_e20749);
        let assign14000_e20751: f64 = (assign14000_e20750).sqrt();
        let assign14000_e20752: f64 = (assign14000_e20721 + assign14000_e20751);
        let assign14000_e20753: f64 = (0.5 * assign14000_e20752);
        (assign14000_e20753, (0.5 * ((p.p744 * (p.p905 * locals.var_tratio_dn4)) + ((((p.p744 * (p.p905 * locals.var_tratio_dn4)) * assign14000_e20743) + (assign14000_e20732 * (p.p744 * (p.p905 * locals.var_tratio_dn4)))) / (2.0 * assign14000_e20751)))),)
    } else {
        let assign14000_e20759: f64 = (locals.var_tratio - 1.0);
        let assign14000_e20760: f64 = (p.p905 * assign14000_e20759);
        let assign14000_e20761: f64 = (1.0 + assign14000_e20760);
        let assign14000_e20762: f64 = (p.p744 * assign14000_e20761);
        let assign14000_e20764: f64 = (assign14000_e20762 - 0.01);
        let assign14000_e20766: f64 = (-10000.0);
        let assign14000_e20768: f64 = (assign14000_e20766 * 0.001);
        let (assign14000_e20787, assign14000_e20787_d_n4,) = {
            if (assign14000_e20764 < assign14000_e20768) {
                let assign14000_e20771: f64 = (-0.001);
                let assign14000_e20773: f64 = (assign14000_e20771 * 0.001);
                let assign14000_e20779: f64 = (locals.var_tratio - 1.0);
                let assign14000_e20780: f64 = (p.p905 * assign14000_e20779);
                let assign14000_e20781: f64 = (1.0 + assign14000_e20780);
                let assign14000_e20782: f64 = (p.p744 * assign14000_e20781);
                let assign14000_e20784: f64 = (assign14000_e20782 - 0.01);
                let assign14000_e20785: f64 = (assign14000_e20773 / assign14000_e20784);
                (assign14000_e20785, (-((assign14000_e20773 * (p.p744 * (p.p905 * locals.var_tratio_dn4))) / (assign14000_e20784 * assign14000_e20784))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign14000_e20787, assign14000_e20787_d_n4,)
    }
};
        let assign14000_e20790: f64 = (assign14000_e20788 + 0.01);
        (locals.var_njtssw_t, locals.var_njtssw_t_dn4, ) = (assign14000_e20790, assign14000_e20788_d_n4, );
        locals.var_njtssw_t_rv = 0.0;

        let assign14010_e20796: f64 = (locals.var_tratio - 1.0);
        let assign14010_e20797: f64 = (p.p907 * assign14010_e20796);
        let assign14010_e20798: f64 = (1.0 + assign14010_e20797);
        let assign14010_e20799: f64 = (p.p746 * assign14010_e20798);
        let assign14010_e20801: f64 = (assign14010_e20799 - 0.01);
        let assign14010_e20803: f64 = (-10000.0);
        let assign14010_e20805: f64 = (assign14010_e20803 * 0.001);
        let (assign14010_e20886, assign14010_e20886_d_n4,) = {
    if (!(assign14010_e20801 < assign14010_e20805)) {
        let assign14010_e20814: f64 = (locals.var_tratio - 1.0);
        let assign14010_e20815: f64 = (p.p907 * assign14010_e20814);
        let assign14010_e20816: f64 = (1.0 + assign14010_e20815);
        let assign14010_e20817: f64 = (p.p746 * assign14010_e20816);
        let assign14010_e20819: f64 = (assign14010_e20817 - 0.01);
        let assign14010_e20825: f64 = (locals.var_tratio - 1.0);
        let assign14010_e20826: f64 = (p.p907 * assign14010_e20825);
        let assign14010_e20827: f64 = (1.0 + assign14010_e20826);
        let assign14010_e20828: f64 = (p.p746 * assign14010_e20827);
        let assign14010_e20830: f64 = (assign14010_e20828 - 0.01);
        let assign14010_e20836: f64 = (locals.var_tratio - 1.0);
        let assign14010_e20837: f64 = (p.p907 * assign14010_e20836);
        let assign14010_e20838: f64 = (1.0 + assign14010_e20837);
        let assign14010_e20839: f64 = (p.p746 * assign14010_e20838);
        let assign14010_e20841: f64 = (assign14010_e20839 - 0.01);
        let assign14010_e20842: f64 = (assign14010_e20830 * assign14010_e20841);
        let assign14010_e20845: f64 = (4.0 * 0.001);
        let assign14010_e20847: f64 = (assign14010_e20845 * 0.001);
        let assign14010_e20848: f64 = (assign14010_e20842 + assign14010_e20847);
        let assign14010_e20849: f64 = (assign14010_e20848).sqrt();
        let assign14010_e20850: f64 = (assign14010_e20819 + assign14010_e20849);
        let assign14010_e20851: f64 = (0.5 * assign14010_e20850);
        (assign14010_e20851, (0.5 * ((p.p746 * (p.p907 * locals.var_tratio_dn4)) + ((((p.p746 * (p.p907 * locals.var_tratio_dn4)) * assign14010_e20841) + (assign14010_e20830 * (p.p746 * (p.p907 * locals.var_tratio_dn4)))) / (2.0 * assign14010_e20849)))),)
    } else {
        let assign14010_e20857: f64 = (locals.var_tratio - 1.0);
        let assign14010_e20858: f64 = (p.p907 * assign14010_e20857);
        let assign14010_e20859: f64 = (1.0 + assign14010_e20858);
        let assign14010_e20860: f64 = (p.p746 * assign14010_e20859);
        let assign14010_e20862: f64 = (assign14010_e20860 - 0.01);
        let assign14010_e20864: f64 = (-10000.0);
        let assign14010_e20866: f64 = (assign14010_e20864 * 0.001);
        let (assign14010_e20885, assign14010_e20885_d_n4,) = {
            if (assign14010_e20862 < assign14010_e20866) {
                let assign14010_e20869: f64 = (-0.001);
                let assign14010_e20871: f64 = (assign14010_e20869 * 0.001);
                let assign14010_e20877: f64 = (locals.var_tratio - 1.0);
                let assign14010_e20878: f64 = (p.p907 * assign14010_e20877);
                let assign14010_e20879: f64 = (1.0 + assign14010_e20878);
                let assign14010_e20880: f64 = (p.p746 * assign14010_e20879);
                let assign14010_e20882: f64 = (assign14010_e20880 - 0.01);
                let assign14010_e20883: f64 = (assign14010_e20871 / assign14010_e20882);
                (assign14010_e20883, (-((assign14010_e20871 * (p.p746 * (p.p907 * locals.var_tratio_dn4))) / (assign14010_e20882 * assign14010_e20882))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign14010_e20885, assign14010_e20885_d_n4,)
    }
};
        let assign14010_e20888: f64 = (assign14010_e20886 + 0.01);
        (locals.var_njtsswg_t, locals.var_njtsswg_t_dn4, ) = (assign14010_e20888, assign14010_e20886_d_n4, );
        locals.var_njtsswg_t_rv = 0.0;

        let assign14020_e20894: f64 = (locals.var_tratio - 1.0);
        let assign14020_e20895: f64 = (p.p904 * assign14020_e20894);
        let assign14020_e20896: f64 = (1.0 + assign14020_e20895);
        let assign14020_e20897: f64 = (p.p743 * assign14020_e20896);
        let assign14020_e20899: f64 = (assign14020_e20897 - 0.01);
        let assign14020_e20901: f64 = (-10000.0);
        let assign14020_e20903: f64 = (assign14020_e20901 * 0.001);
        let (assign14020_e20984, assign14020_e20984_d_n4,) = {
    if (!(assign14020_e20899 < assign14020_e20903)) {
        let assign14020_e20912: f64 = (locals.var_tratio - 1.0);
        let assign14020_e20913: f64 = (p.p904 * assign14020_e20912);
        let assign14020_e20914: f64 = (1.0 + assign14020_e20913);
        let assign14020_e20915: f64 = (p.p743 * assign14020_e20914);
        let assign14020_e20917: f64 = (assign14020_e20915 - 0.01);
        let assign14020_e20923: f64 = (locals.var_tratio - 1.0);
        let assign14020_e20924: f64 = (p.p904 * assign14020_e20923);
        let assign14020_e20925: f64 = (1.0 + assign14020_e20924);
        let assign14020_e20926: f64 = (p.p743 * assign14020_e20925);
        let assign14020_e20928: f64 = (assign14020_e20926 - 0.01);
        let assign14020_e20934: f64 = (locals.var_tratio - 1.0);
        let assign14020_e20935: f64 = (p.p904 * assign14020_e20934);
        let assign14020_e20936: f64 = (1.0 + assign14020_e20935);
        let assign14020_e20937: f64 = (p.p743 * assign14020_e20936);
        let assign14020_e20939: f64 = (assign14020_e20937 - 0.01);
        let assign14020_e20940: f64 = (assign14020_e20928 * assign14020_e20939);
        let assign14020_e20943: f64 = (4.0 * 0.001);
        let assign14020_e20945: f64 = (assign14020_e20943 * 0.001);
        let assign14020_e20946: f64 = (assign14020_e20940 + assign14020_e20945);
        let assign14020_e20947: f64 = (assign14020_e20946).sqrt();
        let assign14020_e20948: f64 = (assign14020_e20917 + assign14020_e20947);
        let assign14020_e20949: f64 = (0.5 * assign14020_e20948);
        (assign14020_e20949, (0.5 * ((p.p743 * (p.p904 * locals.var_tratio_dn4)) + ((((p.p743 * (p.p904 * locals.var_tratio_dn4)) * assign14020_e20939) + (assign14020_e20928 * (p.p743 * (p.p904 * locals.var_tratio_dn4)))) / (2.0 * assign14020_e20947)))),)
    } else {
        let assign14020_e20955: f64 = (locals.var_tratio - 1.0);
        let assign14020_e20956: f64 = (p.p904 * assign14020_e20955);
        let assign14020_e20957: f64 = (1.0 + assign14020_e20956);
        let assign14020_e20958: f64 = (p.p743 * assign14020_e20957);
        let assign14020_e20960: f64 = (assign14020_e20958 - 0.01);
        let assign14020_e20962: f64 = (-10000.0);
        let assign14020_e20964: f64 = (assign14020_e20962 * 0.001);
        let (assign14020_e20983, assign14020_e20983_d_n4,) = {
            if (assign14020_e20960 < assign14020_e20964) {
                let assign14020_e20967: f64 = (-0.001);
                let assign14020_e20969: f64 = (assign14020_e20967 * 0.001);
                let assign14020_e20975: f64 = (locals.var_tratio - 1.0);
                let assign14020_e20976: f64 = (p.p904 * assign14020_e20975);
                let assign14020_e20977: f64 = (1.0 + assign14020_e20976);
                let assign14020_e20978: f64 = (p.p743 * assign14020_e20977);
                let assign14020_e20980: f64 = (assign14020_e20978 - 0.01);
                let assign14020_e20981: f64 = (assign14020_e20969 / assign14020_e20980);
                (assign14020_e20981, (-((assign14020_e20969 * (p.p743 * (p.p904 * locals.var_tratio_dn4))) / (assign14020_e20980 * assign14020_e20980))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign14020_e20983, assign14020_e20983_d_n4,)
    }
};
        let assign14020_e20986: f64 = (assign14020_e20984 + 0.01);
        (locals.var_njtsd_t, locals.var_njtsd_t_dn4, ) = (assign14020_e20986, assign14020_e20984_d_n4, );
        locals.var_njtsd_t_rv = 0.0;

        let assign14030_e20992: f64 = (locals.var_tratio - 1.0);
        let assign14030_e20993: f64 = (p.p906 * assign14030_e20992);
        let assign14030_e20994: f64 = (1.0 + assign14030_e20993);
        let assign14030_e20995: f64 = (p.p745 * assign14030_e20994);
        let assign14030_e20997: f64 = (assign14030_e20995 - 0.01);
        let assign14030_e20999: f64 = (-10000.0);
        let assign14030_e21001: f64 = (assign14030_e20999 * 0.001);
        let (assign14030_e21082, assign14030_e21082_d_n4,) = {
    if (!(assign14030_e20997 < assign14030_e21001)) {
        let assign14030_e21010: f64 = (locals.var_tratio - 1.0);
        let assign14030_e21011: f64 = (p.p906 * assign14030_e21010);
        let assign14030_e21012: f64 = (1.0 + assign14030_e21011);
        let assign14030_e21013: f64 = (p.p745 * assign14030_e21012);
        let assign14030_e21015: f64 = (assign14030_e21013 - 0.01);
        let assign14030_e21021: f64 = (locals.var_tratio - 1.0);
        let assign14030_e21022: f64 = (p.p906 * assign14030_e21021);
        let assign14030_e21023: f64 = (1.0 + assign14030_e21022);
        let assign14030_e21024: f64 = (p.p745 * assign14030_e21023);
        let assign14030_e21026: f64 = (assign14030_e21024 - 0.01);
        let assign14030_e21032: f64 = (locals.var_tratio - 1.0);
        let assign14030_e21033: f64 = (p.p906 * assign14030_e21032);
        let assign14030_e21034: f64 = (1.0 + assign14030_e21033);
        let assign14030_e21035: f64 = (p.p745 * assign14030_e21034);
        let assign14030_e21037: f64 = (assign14030_e21035 - 0.01);
        let assign14030_e21038: f64 = (assign14030_e21026 * assign14030_e21037);
        let assign14030_e21041: f64 = (4.0 * 0.001);
        let assign14030_e21043: f64 = (assign14030_e21041 * 0.001);
        let assign14030_e21044: f64 = (assign14030_e21038 + assign14030_e21043);
        let assign14030_e21045: f64 = (assign14030_e21044).sqrt();
        let assign14030_e21046: f64 = (assign14030_e21015 + assign14030_e21045);
        let assign14030_e21047: f64 = (0.5 * assign14030_e21046);
        (assign14030_e21047, (0.5 * ((p.p745 * (p.p906 * locals.var_tratio_dn4)) + ((((p.p745 * (p.p906 * locals.var_tratio_dn4)) * assign14030_e21037) + (assign14030_e21026 * (p.p745 * (p.p906 * locals.var_tratio_dn4)))) / (2.0 * assign14030_e21045)))),)
    } else {
        let assign14030_e21053: f64 = (locals.var_tratio - 1.0);
        let assign14030_e21054: f64 = (p.p906 * assign14030_e21053);
        let assign14030_e21055: f64 = (1.0 + assign14030_e21054);
        let assign14030_e21056: f64 = (p.p745 * assign14030_e21055);
        let assign14030_e21058: f64 = (assign14030_e21056 - 0.01);
        let assign14030_e21060: f64 = (-10000.0);
        let assign14030_e21062: f64 = (assign14030_e21060 * 0.001);
        let (assign14030_e21081, assign14030_e21081_d_n4,) = {
            if (assign14030_e21058 < assign14030_e21062) {
                let assign14030_e21065: f64 = (-0.001);
                let assign14030_e21067: f64 = (assign14030_e21065 * 0.001);
                let assign14030_e21073: f64 = (locals.var_tratio - 1.0);
                let assign14030_e21074: f64 = (p.p906 * assign14030_e21073);
                let assign14030_e21075: f64 = (1.0 + assign14030_e21074);
                let assign14030_e21076: f64 = (p.p745 * assign14030_e21075);
                let assign14030_e21078: f64 = (assign14030_e21076 - 0.01);
                let assign14030_e21079: f64 = (assign14030_e21067 / assign14030_e21078);
                (assign14030_e21079, (-((assign14030_e21067 * (p.p745 * (p.p906 * locals.var_tratio_dn4))) / (assign14030_e21078 * assign14030_e21078))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign14030_e21081, assign14030_e21081_d_n4,)
    }
};
        let assign14030_e21084: f64 = (assign14030_e21082 + 0.01);
        (locals.var_njtsswd_t, locals.var_njtsswd_t_dn4, ) = (assign14030_e21084, assign14030_e21082_d_n4, );
        locals.var_njtsswd_t_rv = 0.0;

        let assign14040_e21090: f64 = (locals.var_tratio - 1.0);
        let assign14040_e21091: f64 = (p.p908 * assign14040_e21090);
        let assign14040_e21092: f64 = (1.0 + assign14040_e21091);
        let assign14040_e21093: f64 = (p.p747 * assign14040_e21092);
        let assign14040_e21095: f64 = (assign14040_e21093 - 0.01);
        let assign14040_e21097: f64 = (-10000.0);
        let assign14040_e21099: f64 = (assign14040_e21097 * 0.001);
        let (assign14040_e21180, assign14040_e21180_d_n4,) = {
    if (!(assign14040_e21095 < assign14040_e21099)) {
        let assign14040_e21108: f64 = (locals.var_tratio - 1.0);
        let assign14040_e21109: f64 = (p.p908 * assign14040_e21108);
        let assign14040_e21110: f64 = (1.0 + assign14040_e21109);
        let assign14040_e21111: f64 = (p.p747 * assign14040_e21110);
        let assign14040_e21113: f64 = (assign14040_e21111 - 0.01);
        let assign14040_e21119: f64 = (locals.var_tratio - 1.0);
        let assign14040_e21120: f64 = (p.p908 * assign14040_e21119);
        let assign14040_e21121: f64 = (1.0 + assign14040_e21120);
        let assign14040_e21122: f64 = (p.p747 * assign14040_e21121);
        let assign14040_e21124: f64 = (assign14040_e21122 - 0.01);
        let assign14040_e21130: f64 = (locals.var_tratio - 1.0);
        let assign14040_e21131: f64 = (p.p908 * assign14040_e21130);
        let assign14040_e21132: f64 = (1.0 + assign14040_e21131);
        let assign14040_e21133: f64 = (p.p747 * assign14040_e21132);
        let assign14040_e21135: f64 = (assign14040_e21133 - 0.01);
        let assign14040_e21136: f64 = (assign14040_e21124 * assign14040_e21135);
        let assign14040_e21139: f64 = (4.0 * 0.001);
        let assign14040_e21141: f64 = (assign14040_e21139 * 0.001);
        let assign14040_e21142: f64 = (assign14040_e21136 + assign14040_e21141);
        let assign14040_e21143: f64 = (assign14040_e21142).sqrt();
        let assign14040_e21144: f64 = (assign14040_e21113 + assign14040_e21143);
        let assign14040_e21145: f64 = (0.5 * assign14040_e21144);
        (assign14040_e21145, (0.5 * ((p.p747 * (p.p908 * locals.var_tratio_dn4)) + ((((p.p747 * (p.p908 * locals.var_tratio_dn4)) * assign14040_e21135) + (assign14040_e21124 * (p.p747 * (p.p908 * locals.var_tratio_dn4)))) / (2.0 * assign14040_e21143)))),)
    } else {
        let assign14040_e21151: f64 = (locals.var_tratio - 1.0);
        let assign14040_e21152: f64 = (p.p908 * assign14040_e21151);
        let assign14040_e21153: f64 = (1.0 + assign14040_e21152);
        let assign14040_e21154: f64 = (p.p747 * assign14040_e21153);
        let assign14040_e21156: f64 = (assign14040_e21154 - 0.01);
        let assign14040_e21158: f64 = (-10000.0);
        let assign14040_e21160: f64 = (assign14040_e21158 * 0.001);
        let (assign14040_e21179, assign14040_e21179_d_n4,) = {
            if (assign14040_e21156 < assign14040_e21160) {
                let assign14040_e21163: f64 = (-0.001);
                let assign14040_e21165: f64 = (assign14040_e21163 * 0.001);
                let assign14040_e21171: f64 = (locals.var_tratio - 1.0);
                let assign14040_e21172: f64 = (p.p908 * assign14040_e21171);
                let assign14040_e21173: f64 = (1.0 + assign14040_e21172);
                let assign14040_e21174: f64 = (p.p747 * assign14040_e21173);
                let assign14040_e21176: f64 = (assign14040_e21174 - 0.01);
                let assign14040_e21177: f64 = (assign14040_e21165 / assign14040_e21176);
                (assign14040_e21177, (-((assign14040_e21165 * (p.p747 * (p.p908 * locals.var_tratio_dn4))) / (assign14040_e21176 * assign14040_e21176))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign14040_e21179, assign14040_e21179_d_n4,)
    }
};
        let assign14040_e21182: f64 = (assign14040_e21180 + 0.01);
        (locals.var_njtsswgd_t, locals.var_njtsswgd_t_dn4, ) = (assign14040_e21182, assign14040_e21180_d_n4, );
        locals.var_njtsswgd_t_rv = 0.0;

        let assign14050_e21185: f64 = if p.p9 < 9.0 { 1.0 } else { 0.0 };
        locals.var_guard462 = assign14050_e21185;
        locals.var_guard462_rv = 0.0;

        let assign14060_e21188: f64 = (p.p2 % 2.0);
        let assign14060_e21190: f64 = if assign14060_e21188 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard463 = assign14060_e21190;
        locals.var_guard463_rv = 0.0;

        if ((locals.var_guard462 != 0.0) && (locals.var_guard463 != 0.0)) {
            locals.var_nuendd = 1.0;
            locals.var_nuendd_rv = 0.0;
            locals.var_nuends = 1.0;
            locals.var_nuends_rv = 0.0;
        }

        if ((locals.var_guard462 != 0.0) && (locals.var_guard463 != 0.0)) {
            let assign14090_e21209: f64 = (p.p2 - 1.0);
            let assign14090_e21211: f64 = (assign14090_e21209 / 2.0);
            let assign14090_e21213: f64 = (assign14090_e21211).max(0.0);
            let assign14090_e21214: f64 = (2.0 * assign14090_e21213);
            locals.var_nuintd = assign14090_e21214;
            locals.var_nuintd_rv = 0.0;
        }

        if ((locals.var_guard462 != 0.0) && (locals.var_guard463 != 0.0)) {
            locals.var_nuints = locals.var_nuintd;
            locals.var_nuints_rv = 0.0;
        }

        let assign14110_e21225: f64 = if p.p6 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard464 = assign14110_e21225;
        locals.var_guard464_rv = 0.0;

        if (((locals.var_guard462 != 0.0) && (locals.var_guard463 == 0.0)) && (locals.var_guard464 != 0.0)) {
            locals.var_nuendd = 2.0;
            locals.var_nuendd_rv = 0.0;
        }

        if (((locals.var_guard462 != 0.0) && (locals.var_guard463 == 0.0)) && (locals.var_guard464 != 0.0)) {
            let assign14130_e21244: f64 = (p.p2 / 2.0);
            let assign14130_e21246: f64 = (assign14130_e21244 - 1.0);
            let assign14130_e21248: f64 = (assign14130_e21246).max(0.0);
            let assign14130_e21249: f64 = (2.0 * assign14130_e21248);
            locals.var_nuintd = assign14130_e21249;
            locals.var_nuintd_rv = 0.0;
        }

        if (((locals.var_guard462 != 0.0) && (locals.var_guard463 == 0.0)) && (locals.var_guard464 != 0.0)) {
            locals.var_nuends = 0.0;
            locals.var_nuends_rv = 0.0;
            locals.var_nuints = p.p2;
            locals.var_nuints_rv = 0.0;
        }

        if (((locals.var_guard462 != 0.0) && (locals.var_guard463 == 0.0)) && (locals.var_guard464 == 0.0)) {
            locals.var_nuendd = 0.0;
            locals.var_nuendd_rv = 0.0;
            locals.var_nuintd = p.p2;
            locals.var_nuintd_rv = 0.0;
            locals.var_nuends = 2.0;
            locals.var_nuends_rv = 0.0;
        }

        if (((locals.var_guard462 != 0.0) && (locals.var_guard463 == 0.0)) && (locals.var_guard464 == 0.0)) {
            let assign14190_e21310: f64 = (p.p2 / 2.0);
            let assign14190_e21312: f64 = (assign14190_e21310 - 1.0);
            let assign14190_e21314: f64 = (assign14190_e21312).max(0.0);
            let assign14190_e21315: f64 = (2.0 * assign14190_e21314);
            locals.var_nuints = assign14190_e21315;
            locals.var_nuints_rv = 0.0;
        }

        let assign14200_e21320: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign14200_e21320, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_t0_rv = 0.0;

        let assign14210_e21323: f64 = (locals.var_dmcgeff + locals.var_dmcgeff);
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, ) = (assign14210_e21323, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_t1_rv = 0.0;

        let assign14220_e21326: f64 = (locals.var_dmdgeff + locals.var_dmdgeff);
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14, ) = (assign14220_e21326, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_t2_rv = 0.0;

        let assign14230_e21329: f64 = (locals.var_t0 + locals.var_t0);
        let assign14230_e21331: f64 = (assign14230_e21329 + locals.var_weffcj);
        (locals.var_psiso, locals.var_psiso_dn0, locals.var_psiso_dn2, locals.var_psiso_dn3, locals.var_psiso_dn4, locals.var_psiso_dn5, locals.var_psiso_dn6, locals.var_psiso_dn7, locals.var_psiso_dn8, locals.var_psiso_dn9, locals.var_psiso_dn10, locals.var_psiso_dn11, locals.var_psiso_dn12, locals.var_psiso_dn13, locals.var_psiso_dn14, ) = (assign14230_e21331, (locals.var_t0_dn0 + locals.var_t0_dn0), (locals.var_t0_dn2 + locals.var_t0_dn2), (locals.var_t0_dn3 + locals.var_t0_dn3), (locals.var_t0_dn4 + locals.var_t0_dn4), (locals.var_t0_dn5 + locals.var_t0_dn5), (locals.var_t0_dn6 + locals.var_t0_dn6), (locals.var_t0_dn7 + locals.var_t0_dn7), (locals.var_t0_dn8 + locals.var_t0_dn8), (locals.var_t0_dn9 + locals.var_t0_dn9), (locals.var_t0_dn10 + locals.var_t0_dn10), (locals.var_t0_dn11 + locals.var_t0_dn11), (locals.var_t0_dn12 + locals.var_t0_dn12), (locals.var_t0_dn13 + locals.var_t0_dn13), (locals.var_t0_dn14 + locals.var_t0_dn14), );
        locals.var_psiso_rv = 0.0;

        let assign14240_e21334: f64 = (locals.var_t0 + locals.var_t0);
        let assign14240_e21336: f64 = (assign14240_e21334 + locals.var_weffcj);
        (locals.var_pdiso, locals.var_pdiso_dn0, locals.var_pdiso_dn2, locals.var_pdiso_dn3, locals.var_pdiso_dn4, locals.var_pdiso_dn5, locals.var_pdiso_dn6, locals.var_pdiso_dn7, locals.var_pdiso_dn8, locals.var_pdiso_dn9, locals.var_pdiso_dn10, locals.var_pdiso_dn11, locals.var_pdiso_dn12, locals.var_pdiso_dn13, locals.var_pdiso_dn14, ) = (assign14240_e21336, (locals.var_t0_dn0 + locals.var_t0_dn0), (locals.var_t0_dn2 + locals.var_t0_dn2), (locals.var_t0_dn3 + locals.var_t0_dn3), (locals.var_t0_dn4 + locals.var_t0_dn4), (locals.var_t0_dn5 + locals.var_t0_dn5), (locals.var_t0_dn6 + locals.var_t0_dn6), (locals.var_t0_dn7 + locals.var_t0_dn7), (locals.var_t0_dn8 + locals.var_t0_dn8), (locals.var_t0_dn9 + locals.var_t0_dn9), (locals.var_t0_dn10 + locals.var_t0_dn10), (locals.var_t0_dn11 + locals.var_t0_dn11), (locals.var_t0_dn12 + locals.var_t0_dn12), (locals.var_t0_dn13 + locals.var_t0_dn13), (locals.var_t0_dn14 + locals.var_t0_dn14), );
        locals.var_pdiso_rv = 0.0;

        (locals.var_pssha, locals.var_pssha_dn0, locals.var_pssha_dn2, locals.var_pssha_dn3, locals.var_pssha_dn4, locals.var_pssha_dn5, locals.var_pssha_dn6, locals.var_pssha_dn7, locals.var_pssha_dn8, locals.var_pssha_dn9, locals.var_pssha_dn10, locals.var_pssha_dn11, locals.var_pssha_dn12, locals.var_pssha_dn13, locals.var_pssha_dn14, ) = (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, );
        locals.var_pssha_rv = 0.0;

        (locals.var_pdsha, locals.var_pdsha_dn0, locals.var_pdsha_dn2, locals.var_pdsha_dn3, locals.var_pdsha_dn4, locals.var_pdsha_dn5, locals.var_pdsha_dn6, locals.var_pdsha_dn7, locals.var_pdsha_dn8, locals.var_pdsha_dn9, locals.var_pdsha_dn10, locals.var_pdsha_dn11, locals.var_pdsha_dn12, locals.var_pdsha_dn13, locals.var_pdsha_dn14, ) = (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, );
        locals.var_pdsha_rv = 0.0;

        (locals.var_psmer, locals.var_psmer_dn0, locals.var_psmer_dn2, locals.var_psmer_dn3, locals.var_psmer_dn4, locals.var_psmer_dn5, locals.var_psmer_dn6, locals.var_psmer_dn7, locals.var_psmer_dn8, locals.var_psmer_dn9, locals.var_psmer_dn10, locals.var_psmer_dn11, locals.var_psmer_dn12, locals.var_psmer_dn13, locals.var_psmer_dn14, ) = (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14, );
        locals.var_psmer_rv = 0.0;

        (locals.var_pdmer, locals.var_pdmer_dn0, locals.var_pdmer_dn2, locals.var_pdmer_dn3, locals.var_pdmer_dn4, locals.var_pdmer_dn5, locals.var_pdmer_dn6, locals.var_pdmer_dn7, locals.var_pdmer_dn8, locals.var_pdmer_dn9, locals.var_pdmer_dn10, locals.var_pdmer_dn11, locals.var_pdmer_dn12, locals.var_pdmer_dn13, locals.var_pdmer_dn14, ) = (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14, );
        locals.var_pdmer_rv = 0.0;

        let assign14290_e21343: f64 = (locals.var_t0 * locals.var_weffcj);
        (locals.var_asiso, locals.var_asiso_dn0, locals.var_asiso_dn2, locals.var_asiso_dn3, locals.var_asiso_dn4, locals.var_asiso_dn5, locals.var_asiso_dn6, locals.var_asiso_dn7, locals.var_asiso_dn8, locals.var_asiso_dn9, locals.var_asiso_dn10, locals.var_asiso_dn11, locals.var_asiso_dn12, locals.var_asiso_dn13, locals.var_asiso_dn14, ) = (assign14290_e21343, (locals.var_t0_dn0 * locals.var_weffcj), (locals.var_t0_dn2 * locals.var_weffcj), (locals.var_t0_dn3 * locals.var_weffcj), (locals.var_t0_dn4 * locals.var_weffcj), (locals.var_t0_dn5 * locals.var_weffcj), (locals.var_t0_dn6 * locals.var_weffcj), (locals.var_t0_dn7 * locals.var_weffcj), (locals.var_t0_dn8 * locals.var_weffcj), (locals.var_t0_dn9 * locals.var_weffcj), (locals.var_t0_dn10 * locals.var_weffcj), (locals.var_t0_dn11 * locals.var_weffcj), (locals.var_t0_dn12 * locals.var_weffcj), (locals.var_t0_dn13 * locals.var_weffcj), (locals.var_t0_dn14 * locals.var_weffcj), );
        locals.var_asiso_rv = 0.0;

        let assign14300_e21346: f64 = (locals.var_t0 * locals.var_weffcj);
        (locals.var_adiso, locals.var_adiso_dn0, locals.var_adiso_dn2, locals.var_adiso_dn3, locals.var_adiso_dn4, locals.var_adiso_dn5, locals.var_adiso_dn6, locals.var_adiso_dn7, locals.var_adiso_dn8, locals.var_adiso_dn9, locals.var_adiso_dn10, locals.var_adiso_dn11, locals.var_adiso_dn12, locals.var_adiso_dn13, locals.var_adiso_dn14, ) = (assign14300_e21346, (locals.var_t0_dn0 * locals.var_weffcj), (locals.var_t0_dn2 * locals.var_weffcj), (locals.var_t0_dn3 * locals.var_weffcj), (locals.var_t0_dn4 * locals.var_weffcj), (locals.var_t0_dn5 * locals.var_weffcj), (locals.var_t0_dn6 * locals.var_weffcj), (locals.var_t0_dn7 * locals.var_weffcj), (locals.var_t0_dn8 * locals.var_weffcj), (locals.var_t0_dn9 * locals.var_weffcj), (locals.var_t0_dn10 * locals.var_weffcj), (locals.var_t0_dn11 * locals.var_weffcj), (locals.var_t0_dn12 * locals.var_weffcj), (locals.var_t0_dn13 * locals.var_weffcj), (locals.var_t0_dn14 * locals.var_weffcj), );
        locals.var_adiso_rv = 0.0;

        let assign14310_e21349: f64 = (locals.var_dmcgeff * locals.var_weffcj);
        locals.var_assha = assign14310_e21349;
        locals.var_assha_rv = 0.0;

        let assign14320_e21352: f64 = (locals.var_dmcgeff * locals.var_weffcj);
        locals.var_adsha = assign14320_e21352;
        locals.var_adsha_rv = 0.0;

        let assign14330_e21355: f64 = (locals.var_dmdgeff * locals.var_weffcj);
        locals.var_asmer = assign14330_e21355;
        locals.var_asmer_rv = 0.0;

        let assign14340_e21358: f64 = (locals.var_dmdgeff * locals.var_weffcj);
        locals.var_admer = assign14340_e21358;
        locals.var_admer_rv = 0.0;

        let assign14350_e21361: f64 = if p.p9 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard465 = assign14350_e21361;
        locals.var_guard465_rv = 0.0;

        let assign14360_e21364: f64 = if p.p9 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard466 = assign14360_e21364;
        locals.var_guard466_rv = 0.0;

        let assign14370_e21367: f64 = if p.p9 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard467 = assign14370_e21367;
        locals.var_guard467_rv = 0.0;

        let assign14380_e21370: f64 = if p.p9 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard468 = assign14380_e21370;
        locals.var_guard468_rv = 0.0;

        let assign14390_e21373: f64 = if p.p9 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard469 = assign14390_e21373;
        locals.var_guard469_rv = 0.0;

        let assign14400_e21376: f64 = if p.p9 == 5.0 { 1.0 } else { 0.0 };
        locals.var_guard470 = assign14400_e21376;
        locals.var_guard470_rv = 0.0;

        let assign14410_e21379: f64 = if p.p9 == 6.0 { 1.0 } else { 0.0 };
        locals.var_guard471 = assign14410_e21379;
        locals.var_guard471_rv = 0.0;

        let assign14420_e21382: f64 = if p.p9 == 7.0 { 1.0 } else { 0.0 };
        locals.var_guard472 = assign14420_e21382;
        locals.var_guard472_rv = 0.0;

        let assign14430_e21385: f64 = if p.p9 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard473 = assign14430_e21385;
        locals.var_guard473_rv = 0.0;

        let assign14440_e21388: f64 = if p.p9 == 9.0 { 1.0 } else { 0.0 };
        locals.var_guard474 = assign14440_e21388;
        locals.var_guard474_rv = 0.0;

        let assign14450_e21391: f64 = if p.p9 == 10.0 { 1.0 } else { 0.0 };
        locals.var_guard475 = assign14450_e21391;
        locals.var_guard475_rv = 0.0;

        if (locals.var_guard465 != 0.0) {
            let assign14460_e21395: f64 = (locals.var_nuends * locals.var_psiso);
            let assign14460_e21398: f64 = (locals.var_nuints * locals.var_pssha);
            let assign14460_e21399: f64 = (assign14460_e21395 + assign14460_e21398);
            (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14, ) = (assign14460_e21399, ((locals.var_nuends * locals.var_psiso_dn0) + (locals.var_nuints * locals.var_pssha_dn0)), ((locals.var_nuends * locals.var_psiso_dn2) + (locals.var_nuints * locals.var_pssha_dn2)), ((locals.var_nuends * locals.var_psiso_dn3) + (locals.var_nuints * locals.var_pssha_dn3)), ((locals.var_nuends * locals.var_psiso_dn4) + (locals.var_nuints * locals.var_pssha_dn4)), ((locals.var_nuends * locals.var_psiso_dn5) + (locals.var_nuints * locals.var_pssha_dn5)), ((locals.var_nuends * locals.var_psiso_dn6) + (locals.var_nuints * locals.var_pssha_dn6)), ((locals.var_nuends * locals.var_psiso_dn7) + (locals.var_nuints * locals.var_pssha_dn7)), ((locals.var_nuends * locals.var_psiso_dn8) + (locals.var_nuints * locals.var_pssha_dn8)), ((locals.var_nuends * locals.var_psiso_dn9) + (locals.var_nuints * locals.var_pssha_dn9)), ((locals.var_nuends * locals.var_psiso_dn10) + (locals.var_nuints * locals.var_pssha_dn10)), ((locals.var_nuends * locals.var_psiso_dn11) + (locals.var_nuints * locals.var_pssha_dn11)), ((locals.var_nuends * locals.var_psiso_dn12) + (locals.var_nuints * locals.var_pssha_dn12)), ((locals.var_nuends * locals.var_psiso_dn13) + (locals.var_nuints * locals.var_pssha_dn13)), ((locals.var_nuends * locals.var_psiso_dn14) + (locals.var_nuints * locals.var_pssha_dn14)), );
            locals.var_temp_pseff_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_18(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard465 != 0.0) {
            let assign14470_e21405: f64 = (locals.var_nuendd * locals.var_pdiso);
            let assign14470_e21408: f64 = (locals.var_nuintd * locals.var_pdsha);
            let assign14470_e21409: f64 = (assign14470_e21405 + assign14470_e21408);
            (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14, ) = (assign14470_e21409, ((locals.var_nuendd * locals.var_pdiso_dn0) + (locals.var_nuintd * locals.var_pdsha_dn0)), ((locals.var_nuendd * locals.var_pdiso_dn2) + (locals.var_nuintd * locals.var_pdsha_dn2)), ((locals.var_nuendd * locals.var_pdiso_dn3) + (locals.var_nuintd * locals.var_pdsha_dn3)), ((locals.var_nuendd * locals.var_pdiso_dn4) + (locals.var_nuintd * locals.var_pdsha_dn4)), ((locals.var_nuendd * locals.var_pdiso_dn5) + (locals.var_nuintd * locals.var_pdsha_dn5)), ((locals.var_nuendd * locals.var_pdiso_dn6) + (locals.var_nuintd * locals.var_pdsha_dn6)), ((locals.var_nuendd * locals.var_pdiso_dn7) + (locals.var_nuintd * locals.var_pdsha_dn7)), ((locals.var_nuendd * locals.var_pdiso_dn8) + (locals.var_nuintd * locals.var_pdsha_dn8)), ((locals.var_nuendd * locals.var_pdiso_dn9) + (locals.var_nuintd * locals.var_pdsha_dn9)), ((locals.var_nuendd * locals.var_pdiso_dn10) + (locals.var_nuintd * locals.var_pdsha_dn10)), ((locals.var_nuendd * locals.var_pdiso_dn11) + (locals.var_nuintd * locals.var_pdsha_dn11)), ((locals.var_nuendd * locals.var_pdiso_dn12) + (locals.var_nuintd * locals.var_pdsha_dn12)), ((locals.var_nuendd * locals.var_pdiso_dn13) + (locals.var_nuintd * locals.var_pdsha_dn13)), ((locals.var_nuendd * locals.var_pdiso_dn14) + (locals.var_nuintd * locals.var_pdsha_dn14)), );
            locals.var_temp_pdeff_rv = 0.0;
        }

        if (locals.var_guard465 != 0.0) {
            let assign14480_e21415: f64 = (locals.var_nuends * locals.var_asiso);
            let assign14480_e21418: f64 = (locals.var_nuints * locals.var_assha);
            let assign14480_e21419: f64 = (assign14480_e21415 + assign14480_e21418);
            (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14, ) = (assign14480_e21419, (locals.var_nuends * locals.var_asiso_dn0), (locals.var_nuends * locals.var_asiso_dn2), (locals.var_nuends * locals.var_asiso_dn3), (locals.var_nuends * locals.var_asiso_dn4), (locals.var_nuends * locals.var_asiso_dn5), (locals.var_nuends * locals.var_asiso_dn6), (locals.var_nuends * locals.var_asiso_dn7), (locals.var_nuends * locals.var_asiso_dn8), (locals.var_nuends * locals.var_asiso_dn9), (locals.var_nuends * locals.var_asiso_dn10), (locals.var_nuends * locals.var_asiso_dn11), (locals.var_nuends * locals.var_asiso_dn12), (locals.var_nuends * locals.var_asiso_dn13), (locals.var_nuends * locals.var_asiso_dn14), );
            locals.var_temp_aseff_rv = 0.0;
        }

        if (locals.var_guard465 != 0.0) {
            let assign14490_e21425: f64 = (locals.var_nuendd * locals.var_adiso);
            let assign14490_e21428: f64 = (locals.var_nuintd * locals.var_adsha);
            let assign14490_e21429: f64 = (assign14490_e21425 + assign14490_e21428);
            (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14, ) = (assign14490_e21429, (locals.var_nuendd * locals.var_adiso_dn0), (locals.var_nuendd * locals.var_adiso_dn2), (locals.var_nuendd * locals.var_adiso_dn3), (locals.var_nuendd * locals.var_adiso_dn4), (locals.var_nuendd * locals.var_adiso_dn5), (locals.var_nuendd * locals.var_adiso_dn6), (locals.var_nuendd * locals.var_adiso_dn7), (locals.var_nuendd * locals.var_adiso_dn8), (locals.var_nuendd * locals.var_adiso_dn9), (locals.var_nuendd * locals.var_adiso_dn10), (locals.var_nuendd * locals.var_adiso_dn11), (locals.var_nuendd * locals.var_adiso_dn12), (locals.var_nuendd * locals.var_adiso_dn13), (locals.var_nuendd * locals.var_adiso_dn14), );
            locals.var_temp_adeff_rv = 0.0;
        }

        if ((locals.var_guard466 != 0.0) && (locals.var_guard465 == 0.0)) {
            let assign14500_e21438: f64 = (locals.var_nuends * locals.var_psiso);
            let assign14500_e21441: f64 = (locals.var_nuints * locals.var_pssha);
            let assign14500_e21442: f64 = (assign14500_e21438 + assign14500_e21441);
            (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14, ) = (assign14500_e21442, ((locals.var_nuends * locals.var_psiso_dn0) + (locals.var_nuints * locals.var_pssha_dn0)), ((locals.var_nuends * locals.var_psiso_dn2) + (locals.var_nuints * locals.var_pssha_dn2)), ((locals.var_nuends * locals.var_psiso_dn3) + (locals.var_nuints * locals.var_pssha_dn3)), ((locals.var_nuends * locals.var_psiso_dn4) + (locals.var_nuints * locals.var_pssha_dn4)), ((locals.var_nuends * locals.var_psiso_dn5) + (locals.var_nuints * locals.var_pssha_dn5)), ((locals.var_nuends * locals.var_psiso_dn6) + (locals.var_nuints * locals.var_pssha_dn6)), ((locals.var_nuends * locals.var_psiso_dn7) + (locals.var_nuints * locals.var_pssha_dn7)), ((locals.var_nuends * locals.var_psiso_dn8) + (locals.var_nuints * locals.var_pssha_dn8)), ((locals.var_nuends * locals.var_psiso_dn9) + (locals.var_nuints * locals.var_pssha_dn9)), ((locals.var_nuends * locals.var_psiso_dn10) + (locals.var_nuints * locals.var_pssha_dn10)), ((locals.var_nuends * locals.var_psiso_dn11) + (locals.var_nuints * locals.var_pssha_dn11)), ((locals.var_nuends * locals.var_psiso_dn12) + (locals.var_nuints * locals.var_pssha_dn12)), ((locals.var_nuends * locals.var_psiso_dn13) + (locals.var_nuints * locals.var_pssha_dn13)), ((locals.var_nuends * locals.var_psiso_dn14) + (locals.var_nuints * locals.var_pssha_dn14)), );
            locals.var_temp_pseff_rv = 0.0;
        }

        if ((locals.var_guard466 != 0.0) && (locals.var_guard465 == 0.0)) {
            let assign14510_e21451: f64 = (locals.var_nuendd + locals.var_nuintd);
            let assign14510_e21453: f64 = (assign14510_e21451 * locals.var_pdsha);
            (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14, ) = (assign14510_e21453, (assign14510_e21451 * locals.var_pdsha_dn0), (assign14510_e21451 * locals.var_pdsha_dn2), (assign14510_e21451 * locals.var_pdsha_dn3), (assign14510_e21451 * locals.var_pdsha_dn4), (assign14510_e21451 * locals.var_pdsha_dn5), (assign14510_e21451 * locals.var_pdsha_dn6), (assign14510_e21451 * locals.var_pdsha_dn7), (assign14510_e21451 * locals.var_pdsha_dn8), (assign14510_e21451 * locals.var_pdsha_dn9), (assign14510_e21451 * locals.var_pdsha_dn10), (assign14510_e21451 * locals.var_pdsha_dn11), (assign14510_e21451 * locals.var_pdsha_dn12), (assign14510_e21451 * locals.var_pdsha_dn13), (assign14510_e21451 * locals.var_pdsha_dn14), );
            locals.var_temp_pdeff_rv = 0.0;
        }

        if ((locals.var_guard466 != 0.0) && (locals.var_guard465 == 0.0)) {
            let assign14520_e21462: f64 = (locals.var_nuends * locals.var_asiso);
            let assign14520_e21465: f64 = (locals.var_nuints * locals.var_assha);
            let assign14520_e21466: f64 = (assign14520_e21462 + assign14520_e21465);
            (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14, ) = (assign14520_e21466, (locals.var_nuends * locals.var_asiso_dn0), (locals.var_nuends * locals.var_asiso_dn2), (locals.var_nuends * locals.var_asiso_dn3), (locals.var_nuends * locals.var_asiso_dn4), (locals.var_nuends * locals.var_asiso_dn5), (locals.var_nuends * locals.var_asiso_dn6), (locals.var_nuends * locals.var_asiso_dn7), (locals.var_nuends * locals.var_asiso_dn8), (locals.var_nuends * locals.var_asiso_dn9), (locals.var_nuends * locals.var_asiso_dn10), (locals.var_nuends * locals.var_asiso_dn11), (locals.var_nuends * locals.var_asiso_dn12), (locals.var_nuends * locals.var_asiso_dn13), (locals.var_nuends * locals.var_asiso_dn14), );
            locals.var_temp_aseff_rv = 0.0;
        }

        if ((locals.var_guard466 != 0.0) && (locals.var_guard465 == 0.0)) {
            let assign14530_e21475: f64 = (locals.var_nuendd + locals.var_nuintd);
            let assign14530_e21477: f64 = (assign14530_e21475 * locals.var_adsha);
            (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14, ) = (assign14530_e21477, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_temp_adeff_rv = 0.0;
        }

        if ((locals.var_guard467 != 0.0) && (!((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)))) {
            let assign14540_e21488: f64 = (locals.var_nuends + locals.var_nuints);
            let assign14540_e21490: f64 = (assign14540_e21488 * locals.var_pssha);
            (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14, ) = (assign14540_e21490, (assign14540_e21488 * locals.var_pssha_dn0), (assign14540_e21488 * locals.var_pssha_dn2), (assign14540_e21488 * locals.var_pssha_dn3), (assign14540_e21488 * locals.var_pssha_dn4), (assign14540_e21488 * locals.var_pssha_dn5), (assign14540_e21488 * locals.var_pssha_dn6), (assign14540_e21488 * locals.var_pssha_dn7), (assign14540_e21488 * locals.var_pssha_dn8), (assign14540_e21488 * locals.var_pssha_dn9), (assign14540_e21488 * locals.var_pssha_dn10), (assign14540_e21488 * locals.var_pssha_dn11), (assign14540_e21488 * locals.var_pssha_dn12), (assign14540_e21488 * locals.var_pssha_dn13), (assign14540_e21488 * locals.var_pssha_dn14), );
            locals.var_temp_pseff_rv = 0.0;
        }

        if ((locals.var_guard467 != 0.0) && (!((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)))) {
            let assign14550_e21501: f64 = (locals.var_nuendd * locals.var_pdiso);
            let assign14550_e21504: f64 = (locals.var_nuintd * locals.var_pdsha);
            let assign14550_e21505: f64 = (assign14550_e21501 + assign14550_e21504);
            (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14, ) = (assign14550_e21505, ((locals.var_nuendd * locals.var_pdiso_dn0) + (locals.var_nuintd * locals.var_pdsha_dn0)), ((locals.var_nuendd * locals.var_pdiso_dn2) + (locals.var_nuintd * locals.var_pdsha_dn2)), ((locals.var_nuendd * locals.var_pdiso_dn3) + (locals.var_nuintd * locals.var_pdsha_dn3)), ((locals.var_nuendd * locals.var_pdiso_dn4) + (locals.var_nuintd * locals.var_pdsha_dn4)), ((locals.var_nuendd * locals.var_pdiso_dn5) + (locals.var_nuintd * locals.var_pdsha_dn5)), ((locals.var_nuendd * locals.var_pdiso_dn6) + (locals.var_nuintd * locals.var_pdsha_dn6)), ((locals.var_nuendd * locals.var_pdiso_dn7) + (locals.var_nuintd * locals.var_pdsha_dn7)), ((locals.var_nuendd * locals.var_pdiso_dn8) + (locals.var_nuintd * locals.var_pdsha_dn8)), ((locals.var_nuendd * locals.var_pdiso_dn9) + (locals.var_nuintd * locals.var_pdsha_dn9)), ((locals.var_nuendd * locals.var_pdiso_dn10) + (locals.var_nuintd * locals.var_pdsha_dn10)), ((locals.var_nuendd * locals.var_pdiso_dn11) + (locals.var_nuintd * locals.var_pdsha_dn11)), ((locals.var_nuendd * locals.var_pdiso_dn12) + (locals.var_nuintd * locals.var_pdsha_dn12)), ((locals.var_nuendd * locals.var_pdiso_dn13) + (locals.var_nuintd * locals.var_pdsha_dn13)), ((locals.var_nuendd * locals.var_pdiso_dn14) + (locals.var_nuintd * locals.var_pdsha_dn14)), );
            locals.var_temp_pdeff_rv = 0.0;
        }

        if ((locals.var_guard467 != 0.0) && (!((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)))) {
            let assign14560_e21516: f64 = (locals.var_nuends + locals.var_nuints);
            let assign14560_e21518: f64 = (assign14560_e21516 * locals.var_assha);
            (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14, ) = (assign14560_e21518, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_temp_aseff_rv = 0.0;
        }

        if ((locals.var_guard467 != 0.0) && (!((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)))) {
            let assign14570_e21529: f64 = (locals.var_nuendd * locals.var_adiso);
            let assign14570_e21532: f64 = (locals.var_nuintd * locals.var_adsha);
            let assign14570_e21533: f64 = (assign14570_e21529 + assign14570_e21532);
            (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14, ) = (assign14570_e21533, (locals.var_nuendd * locals.var_adiso_dn0), (locals.var_nuendd * locals.var_adiso_dn2), (locals.var_nuendd * locals.var_adiso_dn3), (locals.var_nuendd * locals.var_adiso_dn4), (locals.var_nuendd * locals.var_adiso_dn5), (locals.var_nuendd * locals.var_adiso_dn6), (locals.var_nuendd * locals.var_adiso_dn7), (locals.var_nuendd * locals.var_adiso_dn8), (locals.var_nuendd * locals.var_adiso_dn9), (locals.var_nuendd * locals.var_adiso_dn10), (locals.var_nuendd * locals.var_adiso_dn11), (locals.var_nuendd * locals.var_adiso_dn12), (locals.var_nuendd * locals.var_adiso_dn13), (locals.var_nuendd * locals.var_adiso_dn14), );
            locals.var_temp_adeff_rv = 0.0;
        }

        if ((locals.var_guard468 != 0.0) && (!(((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)))) {
            let assign14580_e21546: f64 = (locals.var_nuends + locals.var_nuints);
            let assign14580_e21548: f64 = (assign14580_e21546 * locals.var_pssha);
            (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14, ) = (assign14580_e21548, (assign14580_e21546 * locals.var_pssha_dn0), (assign14580_e21546 * locals.var_pssha_dn2), (assign14580_e21546 * locals.var_pssha_dn3), (assign14580_e21546 * locals.var_pssha_dn4), (assign14580_e21546 * locals.var_pssha_dn5), (assign14580_e21546 * locals.var_pssha_dn6), (assign14580_e21546 * locals.var_pssha_dn7), (assign14580_e21546 * locals.var_pssha_dn8), (assign14580_e21546 * locals.var_pssha_dn9), (assign14580_e21546 * locals.var_pssha_dn10), (assign14580_e21546 * locals.var_pssha_dn11), (assign14580_e21546 * locals.var_pssha_dn12), (assign14580_e21546 * locals.var_pssha_dn13), (assign14580_e21546 * locals.var_pssha_dn14), );
            locals.var_temp_pseff_rv = 0.0;
        }

        if ((locals.var_guard468 != 0.0) && (!(((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)))) {
            let assign14590_e21561: f64 = (locals.var_nuendd + locals.var_nuintd);
            let assign14590_e21563: f64 = (assign14590_e21561 * locals.var_pdsha);
            (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14, ) = (assign14590_e21563, (assign14590_e21561 * locals.var_pdsha_dn0), (assign14590_e21561 * locals.var_pdsha_dn2), (assign14590_e21561 * locals.var_pdsha_dn3), (assign14590_e21561 * locals.var_pdsha_dn4), (assign14590_e21561 * locals.var_pdsha_dn5), (assign14590_e21561 * locals.var_pdsha_dn6), (assign14590_e21561 * locals.var_pdsha_dn7), (assign14590_e21561 * locals.var_pdsha_dn8), (assign14590_e21561 * locals.var_pdsha_dn9), (assign14590_e21561 * locals.var_pdsha_dn10), (assign14590_e21561 * locals.var_pdsha_dn11), (assign14590_e21561 * locals.var_pdsha_dn12), (assign14590_e21561 * locals.var_pdsha_dn13), (assign14590_e21561 * locals.var_pdsha_dn14), );
            locals.var_temp_pdeff_rv = 0.0;
        }

        if ((locals.var_guard468 != 0.0) && (!(((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)))) {
            let assign14600_e21576: f64 = (locals.var_nuends + locals.var_nuints);
            let assign14600_e21578: f64 = (assign14600_e21576 * locals.var_assha);
            (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14, ) = (assign14600_e21578, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_temp_aseff_rv = 0.0;
        }

        if ((locals.var_guard468 != 0.0) && (!(((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)))) {
            let assign14610_e21591: f64 = (locals.var_nuendd + locals.var_nuintd);
            let assign14610_e21593: f64 = (assign14610_e21591 * locals.var_adsha);
            (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14, ) = (assign14610_e21593, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_temp_adeff_rv = 0.0;
        }

        if ((locals.var_guard469 != 0.0) && (!((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)))) {
            let assign14620_e21608: f64 = (locals.var_nuends * locals.var_psiso);
            let assign14620_e21611: f64 = (locals.var_nuints * locals.var_pssha);
            let assign14620_e21612: f64 = (assign14620_e21608 + assign14620_e21611);
            (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14, ) = (assign14620_e21612, ((locals.var_nuends * locals.var_psiso_dn0) + (locals.var_nuints * locals.var_pssha_dn0)), ((locals.var_nuends * locals.var_psiso_dn2) + (locals.var_nuints * locals.var_pssha_dn2)), ((locals.var_nuends * locals.var_psiso_dn3) + (locals.var_nuints * locals.var_pssha_dn3)), ((locals.var_nuends * locals.var_psiso_dn4) + (locals.var_nuints * locals.var_pssha_dn4)), ((locals.var_nuends * locals.var_psiso_dn5) + (locals.var_nuints * locals.var_pssha_dn5)), ((locals.var_nuends * locals.var_psiso_dn6) + (locals.var_nuints * locals.var_pssha_dn6)), ((locals.var_nuends * locals.var_psiso_dn7) + (locals.var_nuints * locals.var_pssha_dn7)), ((locals.var_nuends * locals.var_psiso_dn8) + (locals.var_nuints * locals.var_pssha_dn8)), ((locals.var_nuends * locals.var_psiso_dn9) + (locals.var_nuints * locals.var_pssha_dn9)), ((locals.var_nuends * locals.var_psiso_dn10) + (locals.var_nuints * locals.var_pssha_dn10)), ((locals.var_nuends * locals.var_psiso_dn11) + (locals.var_nuints * locals.var_pssha_dn11)), ((locals.var_nuends * locals.var_psiso_dn12) + (locals.var_nuints * locals.var_pssha_dn12)), ((locals.var_nuends * locals.var_psiso_dn13) + (locals.var_nuints * locals.var_pssha_dn13)), ((locals.var_nuends * locals.var_psiso_dn14) + (locals.var_nuints * locals.var_pssha_dn14)), );
            locals.var_temp_pseff_rv = 0.0;
        }

        if ((locals.var_guard469 != 0.0) && (!((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)))) {
            let assign14630_e21627: f64 = (locals.var_nuendd * locals.var_pdmer);
            let assign14630_e21630: f64 = (locals.var_nuintd * locals.var_pdsha);
            let assign14630_e21631: f64 = (assign14630_e21627 + assign14630_e21630);
            (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14, ) = (assign14630_e21631, ((locals.var_nuendd * locals.var_pdmer_dn0) + (locals.var_nuintd * locals.var_pdsha_dn0)), ((locals.var_nuendd * locals.var_pdmer_dn2) + (locals.var_nuintd * locals.var_pdsha_dn2)), ((locals.var_nuendd * locals.var_pdmer_dn3) + (locals.var_nuintd * locals.var_pdsha_dn3)), ((locals.var_nuendd * locals.var_pdmer_dn4) + (locals.var_nuintd * locals.var_pdsha_dn4)), ((locals.var_nuendd * locals.var_pdmer_dn5) + (locals.var_nuintd * locals.var_pdsha_dn5)), ((locals.var_nuendd * locals.var_pdmer_dn6) + (locals.var_nuintd * locals.var_pdsha_dn6)), ((locals.var_nuendd * locals.var_pdmer_dn7) + (locals.var_nuintd * locals.var_pdsha_dn7)), ((locals.var_nuendd * locals.var_pdmer_dn8) + (locals.var_nuintd * locals.var_pdsha_dn8)), ((locals.var_nuendd * locals.var_pdmer_dn9) + (locals.var_nuintd * locals.var_pdsha_dn9)), ((locals.var_nuendd * locals.var_pdmer_dn10) + (locals.var_nuintd * locals.var_pdsha_dn10)), ((locals.var_nuendd * locals.var_pdmer_dn11) + (locals.var_nuintd * locals.var_pdsha_dn11)), ((locals.var_nuendd * locals.var_pdmer_dn12) + (locals.var_nuintd * locals.var_pdsha_dn12)), ((locals.var_nuendd * locals.var_pdmer_dn13) + (locals.var_nuintd * locals.var_pdsha_dn13)), ((locals.var_nuendd * locals.var_pdmer_dn14) + (locals.var_nuintd * locals.var_pdsha_dn14)), );
            locals.var_temp_pdeff_rv = 0.0;
        }

        if ((locals.var_guard469 != 0.0) && (!((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)))) {
            let assign14640_e21646: f64 = (locals.var_nuends * locals.var_asiso);
            let assign14640_e21649: f64 = (locals.var_nuints * locals.var_assha);
            let assign14640_e21650: f64 = (assign14640_e21646 + assign14640_e21649);
            (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14, ) = (assign14640_e21650, (locals.var_nuends * locals.var_asiso_dn0), (locals.var_nuends * locals.var_asiso_dn2), (locals.var_nuends * locals.var_asiso_dn3), (locals.var_nuends * locals.var_asiso_dn4), (locals.var_nuends * locals.var_asiso_dn5), (locals.var_nuends * locals.var_asiso_dn6), (locals.var_nuends * locals.var_asiso_dn7), (locals.var_nuends * locals.var_asiso_dn8), (locals.var_nuends * locals.var_asiso_dn9), (locals.var_nuends * locals.var_asiso_dn10), (locals.var_nuends * locals.var_asiso_dn11), (locals.var_nuends * locals.var_asiso_dn12), (locals.var_nuends * locals.var_asiso_dn13), (locals.var_nuends * locals.var_asiso_dn14), );
            locals.var_temp_aseff_rv = 0.0;
        }

        if ((locals.var_guard469 != 0.0) && (!((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)))) {
            let assign14650_e21665: f64 = (locals.var_nuendd * locals.var_admer);
            let assign14650_e21668: f64 = (locals.var_nuintd * locals.var_adsha);
            let assign14650_e21669: f64 = (assign14650_e21665 + assign14650_e21668);
            (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14, ) = (assign14650_e21669, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_temp_adeff_rv = 0.0;
        }

        if ((locals.var_guard470 != 0.0) && (!(((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)))) {
            let assign14660_e21686: f64 = (locals.var_nuends + locals.var_nuints);
            let assign14660_e21688: f64 = (assign14660_e21686 * locals.var_pssha);
            (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14, ) = (assign14660_e21688, (assign14660_e21686 * locals.var_pssha_dn0), (assign14660_e21686 * locals.var_pssha_dn2), (assign14660_e21686 * locals.var_pssha_dn3), (assign14660_e21686 * locals.var_pssha_dn4), (assign14660_e21686 * locals.var_pssha_dn5), (assign14660_e21686 * locals.var_pssha_dn6), (assign14660_e21686 * locals.var_pssha_dn7), (assign14660_e21686 * locals.var_pssha_dn8), (assign14660_e21686 * locals.var_pssha_dn9), (assign14660_e21686 * locals.var_pssha_dn10), (assign14660_e21686 * locals.var_pssha_dn11), (assign14660_e21686 * locals.var_pssha_dn12), (assign14660_e21686 * locals.var_pssha_dn13), (assign14660_e21686 * locals.var_pssha_dn14), );
            locals.var_temp_pseff_rv = 0.0;
        }

        if ((locals.var_guard470 != 0.0) && (!(((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)))) {
            let assign14670_e21705: f64 = (locals.var_nuendd * locals.var_pdmer);
            let assign14670_e21708: f64 = (locals.var_nuintd * locals.var_pdsha);
            let assign14670_e21709: f64 = (assign14670_e21705 + assign14670_e21708);
            (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14, ) = (assign14670_e21709, ((locals.var_nuendd * locals.var_pdmer_dn0) + (locals.var_nuintd * locals.var_pdsha_dn0)), ((locals.var_nuendd * locals.var_pdmer_dn2) + (locals.var_nuintd * locals.var_pdsha_dn2)), ((locals.var_nuendd * locals.var_pdmer_dn3) + (locals.var_nuintd * locals.var_pdsha_dn3)), ((locals.var_nuendd * locals.var_pdmer_dn4) + (locals.var_nuintd * locals.var_pdsha_dn4)), ((locals.var_nuendd * locals.var_pdmer_dn5) + (locals.var_nuintd * locals.var_pdsha_dn5)), ((locals.var_nuendd * locals.var_pdmer_dn6) + (locals.var_nuintd * locals.var_pdsha_dn6)), ((locals.var_nuendd * locals.var_pdmer_dn7) + (locals.var_nuintd * locals.var_pdsha_dn7)), ((locals.var_nuendd * locals.var_pdmer_dn8) + (locals.var_nuintd * locals.var_pdsha_dn8)), ((locals.var_nuendd * locals.var_pdmer_dn9) + (locals.var_nuintd * locals.var_pdsha_dn9)), ((locals.var_nuendd * locals.var_pdmer_dn10) + (locals.var_nuintd * locals.var_pdsha_dn10)), ((locals.var_nuendd * locals.var_pdmer_dn11) + (locals.var_nuintd * locals.var_pdsha_dn11)), ((locals.var_nuendd * locals.var_pdmer_dn12) + (locals.var_nuintd * locals.var_pdsha_dn12)), ((locals.var_nuendd * locals.var_pdmer_dn13) + (locals.var_nuintd * locals.var_pdsha_dn13)), ((locals.var_nuendd * locals.var_pdmer_dn14) + (locals.var_nuintd * locals.var_pdsha_dn14)), );
            locals.var_temp_pdeff_rv = 0.0;
        }

        if ((locals.var_guard470 != 0.0) && (!(((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)))) {
            let assign14680_e21726: f64 = (locals.var_nuends + locals.var_nuints);
            let assign14680_e21728: f64 = (assign14680_e21726 * locals.var_assha);
            (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14, ) = (assign14680_e21728, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_temp_aseff_rv = 0.0;
        }

        if ((locals.var_guard470 != 0.0) && (!(((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)))) {
            let assign14690_e21745: f64 = (locals.var_nuendd * locals.var_admer);
            let assign14690_e21748: f64 = (locals.var_nuintd * locals.var_adsha);
            let assign14690_e21749: f64 = (assign14690_e21745 + assign14690_e21748);
            (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14, ) = (assign14690_e21749, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_temp_adeff_rv = 0.0;
        }

        if ((locals.var_guard471 != 0.0) && (!((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)))) {
            let assign14700_e21768: f64 = (locals.var_nuends * locals.var_psmer);
            let assign14700_e21771: f64 = (locals.var_nuints * locals.var_pssha);
            let assign14700_e21772: f64 = (assign14700_e21768 + assign14700_e21771);
            (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14, ) = (assign14700_e21772, ((locals.var_nuends * locals.var_psmer_dn0) + (locals.var_nuints * locals.var_pssha_dn0)), ((locals.var_nuends * locals.var_psmer_dn2) + (locals.var_nuints * locals.var_pssha_dn2)), ((locals.var_nuends * locals.var_psmer_dn3) + (locals.var_nuints * locals.var_pssha_dn3)), ((locals.var_nuends * locals.var_psmer_dn4) + (locals.var_nuints * locals.var_pssha_dn4)), ((locals.var_nuends * locals.var_psmer_dn5) + (locals.var_nuints * locals.var_pssha_dn5)), ((locals.var_nuends * locals.var_psmer_dn6) + (locals.var_nuints * locals.var_pssha_dn6)), ((locals.var_nuends * locals.var_psmer_dn7) + (locals.var_nuints * locals.var_pssha_dn7)), ((locals.var_nuends * locals.var_psmer_dn8) + (locals.var_nuints * locals.var_pssha_dn8)), ((locals.var_nuends * locals.var_psmer_dn9) + (locals.var_nuints * locals.var_pssha_dn9)), ((locals.var_nuends * locals.var_psmer_dn10) + (locals.var_nuints * locals.var_pssha_dn10)), ((locals.var_nuends * locals.var_psmer_dn11) + (locals.var_nuints * locals.var_pssha_dn11)), ((locals.var_nuends * locals.var_psmer_dn12) + (locals.var_nuints * locals.var_pssha_dn12)), ((locals.var_nuends * locals.var_psmer_dn13) + (locals.var_nuints * locals.var_pssha_dn13)), ((locals.var_nuends * locals.var_psmer_dn14) + (locals.var_nuints * locals.var_pssha_dn14)), );
            locals.var_temp_pseff_rv = 0.0;
        }

        if ((locals.var_guard471 != 0.0) && (!((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)))) {
            let assign14710_e21791: f64 = (locals.var_nuendd * locals.var_pdiso);
            let assign14710_e21794: f64 = (locals.var_nuintd * locals.var_pdsha);
            let assign14710_e21795: f64 = (assign14710_e21791 + assign14710_e21794);
            (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14, ) = (assign14710_e21795, ((locals.var_nuendd * locals.var_pdiso_dn0) + (locals.var_nuintd * locals.var_pdsha_dn0)), ((locals.var_nuendd * locals.var_pdiso_dn2) + (locals.var_nuintd * locals.var_pdsha_dn2)), ((locals.var_nuendd * locals.var_pdiso_dn3) + (locals.var_nuintd * locals.var_pdsha_dn3)), ((locals.var_nuendd * locals.var_pdiso_dn4) + (locals.var_nuintd * locals.var_pdsha_dn4)), ((locals.var_nuendd * locals.var_pdiso_dn5) + (locals.var_nuintd * locals.var_pdsha_dn5)), ((locals.var_nuendd * locals.var_pdiso_dn6) + (locals.var_nuintd * locals.var_pdsha_dn6)), ((locals.var_nuendd * locals.var_pdiso_dn7) + (locals.var_nuintd * locals.var_pdsha_dn7)), ((locals.var_nuendd * locals.var_pdiso_dn8) + (locals.var_nuintd * locals.var_pdsha_dn8)), ((locals.var_nuendd * locals.var_pdiso_dn9) + (locals.var_nuintd * locals.var_pdsha_dn9)), ((locals.var_nuendd * locals.var_pdiso_dn10) + (locals.var_nuintd * locals.var_pdsha_dn10)), ((locals.var_nuendd * locals.var_pdiso_dn11) + (locals.var_nuintd * locals.var_pdsha_dn11)), ((locals.var_nuendd * locals.var_pdiso_dn12) + (locals.var_nuintd * locals.var_pdsha_dn12)), ((locals.var_nuendd * locals.var_pdiso_dn13) + (locals.var_nuintd * locals.var_pdsha_dn13)), ((locals.var_nuendd * locals.var_pdiso_dn14) + (locals.var_nuintd * locals.var_pdsha_dn14)), );
            locals.var_temp_pdeff_rv = 0.0;
        }

        if ((locals.var_guard471 != 0.0) && (!((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)))) {
            let assign14720_e21814: f64 = (locals.var_nuends * locals.var_asmer);
            let assign14720_e21817: f64 = (locals.var_nuints * locals.var_assha);
            let assign14720_e21818: f64 = (assign14720_e21814 + assign14720_e21817);
            (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14, ) = (assign14720_e21818, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_temp_aseff_rv = 0.0;
        }

        if ((locals.var_guard471 != 0.0) && (!((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)))) {
            let assign14730_e21837: f64 = (locals.var_nuendd * locals.var_adiso);
            let assign14730_e21840: f64 = (locals.var_nuintd * locals.var_adsha);
            let assign14730_e21841: f64 = (assign14730_e21837 + assign14730_e21840);
            (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14, ) = (assign14730_e21841, (locals.var_nuendd * locals.var_adiso_dn0), (locals.var_nuendd * locals.var_adiso_dn2), (locals.var_nuendd * locals.var_adiso_dn3), (locals.var_nuendd * locals.var_adiso_dn4), (locals.var_nuendd * locals.var_adiso_dn5), (locals.var_nuendd * locals.var_adiso_dn6), (locals.var_nuendd * locals.var_adiso_dn7), (locals.var_nuendd * locals.var_adiso_dn8), (locals.var_nuendd * locals.var_adiso_dn9), (locals.var_nuendd * locals.var_adiso_dn10), (locals.var_nuendd * locals.var_adiso_dn11), (locals.var_nuendd * locals.var_adiso_dn12), (locals.var_nuendd * locals.var_adiso_dn13), (locals.var_nuendd * locals.var_adiso_dn14), );
            locals.var_temp_adeff_rv = 0.0;
        }

        if ((locals.var_guard472 != 0.0) && (!(((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)))) {
            let assign14740_e21862: f64 = (locals.var_nuends * locals.var_psmer);
            let assign14740_e21865: f64 = (locals.var_nuints * locals.var_pssha);
            let assign14740_e21866: f64 = (assign14740_e21862 + assign14740_e21865);
            (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14, ) = (assign14740_e21866, ((locals.var_nuends * locals.var_psmer_dn0) + (locals.var_nuints * locals.var_pssha_dn0)), ((locals.var_nuends * locals.var_psmer_dn2) + (locals.var_nuints * locals.var_pssha_dn2)), ((locals.var_nuends * locals.var_psmer_dn3) + (locals.var_nuints * locals.var_pssha_dn3)), ((locals.var_nuends * locals.var_psmer_dn4) + (locals.var_nuints * locals.var_pssha_dn4)), ((locals.var_nuends * locals.var_psmer_dn5) + (locals.var_nuints * locals.var_pssha_dn5)), ((locals.var_nuends * locals.var_psmer_dn6) + (locals.var_nuints * locals.var_pssha_dn6)), ((locals.var_nuends * locals.var_psmer_dn7) + (locals.var_nuints * locals.var_pssha_dn7)), ((locals.var_nuends * locals.var_psmer_dn8) + (locals.var_nuints * locals.var_pssha_dn8)), ((locals.var_nuends * locals.var_psmer_dn9) + (locals.var_nuints * locals.var_pssha_dn9)), ((locals.var_nuends * locals.var_psmer_dn10) + (locals.var_nuints * locals.var_pssha_dn10)), ((locals.var_nuends * locals.var_psmer_dn11) + (locals.var_nuints * locals.var_pssha_dn11)), ((locals.var_nuends * locals.var_psmer_dn12) + (locals.var_nuints * locals.var_pssha_dn12)), ((locals.var_nuends * locals.var_psmer_dn13) + (locals.var_nuints * locals.var_pssha_dn13)), ((locals.var_nuends * locals.var_psmer_dn14) + (locals.var_nuints * locals.var_pssha_dn14)), );
            locals.var_temp_pseff_rv = 0.0;
        }

        if ((locals.var_guard472 != 0.0) && (!(((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)))) {
            let assign14750_e21887: f64 = (locals.var_nuendd + locals.var_nuintd);
            let assign14750_e21889: f64 = (assign14750_e21887 * locals.var_pdsha);
            (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14, ) = (assign14750_e21889, (assign14750_e21887 * locals.var_pdsha_dn0), (assign14750_e21887 * locals.var_pdsha_dn2), (assign14750_e21887 * locals.var_pdsha_dn3), (assign14750_e21887 * locals.var_pdsha_dn4), (assign14750_e21887 * locals.var_pdsha_dn5), (assign14750_e21887 * locals.var_pdsha_dn6), (assign14750_e21887 * locals.var_pdsha_dn7), (assign14750_e21887 * locals.var_pdsha_dn8), (assign14750_e21887 * locals.var_pdsha_dn9), (assign14750_e21887 * locals.var_pdsha_dn10), (assign14750_e21887 * locals.var_pdsha_dn11), (assign14750_e21887 * locals.var_pdsha_dn12), (assign14750_e21887 * locals.var_pdsha_dn13), (assign14750_e21887 * locals.var_pdsha_dn14), );
            locals.var_temp_pdeff_rv = 0.0;
        }

        if ((locals.var_guard472 != 0.0) && (!(((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)))) {
            let assign14760_e21910: f64 = (locals.var_nuends * locals.var_asmer);
            let assign14760_e21913: f64 = (locals.var_nuints * locals.var_assha);
            let assign14760_e21914: f64 = (assign14760_e21910 + assign14760_e21913);
            (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14, ) = (assign14760_e21914, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_temp_aseff_rv = 0.0;
        }

        if ((locals.var_guard472 != 0.0) && (!(((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)))) {
            let assign14770_e21935: f64 = (locals.var_nuendd + locals.var_nuintd);
            let assign14770_e21937: f64 = (assign14770_e21935 * locals.var_adsha);
            (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14, ) = (assign14770_e21937, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_temp_adeff_rv = 0.0;
        }

        if ((locals.var_guard473 != 0.0) && (!((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)))) {
            let assign14780_e21960: f64 = (locals.var_nuends * locals.var_psmer);
            let assign14780_e21963: f64 = (locals.var_nuints * locals.var_pssha);
            let assign14780_e21964: f64 = (assign14780_e21960 + assign14780_e21963);
            (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14, ) = (assign14780_e21964, ((locals.var_nuends * locals.var_psmer_dn0) + (locals.var_nuints * locals.var_pssha_dn0)), ((locals.var_nuends * locals.var_psmer_dn2) + (locals.var_nuints * locals.var_pssha_dn2)), ((locals.var_nuends * locals.var_psmer_dn3) + (locals.var_nuints * locals.var_pssha_dn3)), ((locals.var_nuends * locals.var_psmer_dn4) + (locals.var_nuints * locals.var_pssha_dn4)), ((locals.var_nuends * locals.var_psmer_dn5) + (locals.var_nuints * locals.var_pssha_dn5)), ((locals.var_nuends * locals.var_psmer_dn6) + (locals.var_nuints * locals.var_pssha_dn6)), ((locals.var_nuends * locals.var_psmer_dn7) + (locals.var_nuints * locals.var_pssha_dn7)), ((locals.var_nuends * locals.var_psmer_dn8) + (locals.var_nuints * locals.var_pssha_dn8)), ((locals.var_nuends * locals.var_psmer_dn9) + (locals.var_nuints * locals.var_pssha_dn9)), ((locals.var_nuends * locals.var_psmer_dn10) + (locals.var_nuints * locals.var_pssha_dn10)), ((locals.var_nuends * locals.var_psmer_dn11) + (locals.var_nuints * locals.var_pssha_dn11)), ((locals.var_nuends * locals.var_psmer_dn12) + (locals.var_nuints * locals.var_pssha_dn12)), ((locals.var_nuends * locals.var_psmer_dn13) + (locals.var_nuints * locals.var_pssha_dn13)), ((locals.var_nuends * locals.var_psmer_dn14) + (locals.var_nuints * locals.var_pssha_dn14)), );
            locals.var_temp_pseff_rv = 0.0;
        }

        if ((locals.var_guard473 != 0.0) && (!((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)))) {
            let assign14790_e21987: f64 = (locals.var_nuendd * locals.var_pdmer);
            let assign14790_e21990: f64 = (locals.var_nuintd * locals.var_pdsha);
            let assign14790_e21991: f64 = (assign14790_e21987 + assign14790_e21990);
            (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14, ) = (assign14790_e21991, ((locals.var_nuendd * locals.var_pdmer_dn0) + (locals.var_nuintd * locals.var_pdsha_dn0)), ((locals.var_nuendd * locals.var_pdmer_dn2) + (locals.var_nuintd * locals.var_pdsha_dn2)), ((locals.var_nuendd * locals.var_pdmer_dn3) + (locals.var_nuintd * locals.var_pdsha_dn3)), ((locals.var_nuendd * locals.var_pdmer_dn4) + (locals.var_nuintd * locals.var_pdsha_dn4)), ((locals.var_nuendd * locals.var_pdmer_dn5) + (locals.var_nuintd * locals.var_pdsha_dn5)), ((locals.var_nuendd * locals.var_pdmer_dn6) + (locals.var_nuintd * locals.var_pdsha_dn6)), ((locals.var_nuendd * locals.var_pdmer_dn7) + (locals.var_nuintd * locals.var_pdsha_dn7)), ((locals.var_nuendd * locals.var_pdmer_dn8) + (locals.var_nuintd * locals.var_pdsha_dn8)), ((locals.var_nuendd * locals.var_pdmer_dn9) + (locals.var_nuintd * locals.var_pdsha_dn9)), ((locals.var_nuendd * locals.var_pdmer_dn10) + (locals.var_nuintd * locals.var_pdsha_dn10)), ((locals.var_nuendd * locals.var_pdmer_dn11) + (locals.var_nuintd * locals.var_pdsha_dn11)), ((locals.var_nuendd * locals.var_pdmer_dn12) + (locals.var_nuintd * locals.var_pdsha_dn12)), ((locals.var_nuendd * locals.var_pdmer_dn13) + (locals.var_nuintd * locals.var_pdsha_dn13)), ((locals.var_nuendd * locals.var_pdmer_dn14) + (locals.var_nuintd * locals.var_pdsha_dn14)), );
            locals.var_temp_pdeff_rv = 0.0;
        }

        if ((locals.var_guard473 != 0.0) && (!((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)))) {
            let assign14800_e22014: f64 = (locals.var_nuends * locals.var_asmer);
            let assign14800_e22017: f64 = (locals.var_nuints * locals.var_assha);
            let assign14800_e22018: f64 = (assign14800_e22014 + assign14800_e22017);
            (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14, ) = (assign14800_e22018, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_temp_aseff_rv = 0.0;
        }

        if ((locals.var_guard473 != 0.0) && (!((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)))) {
            let assign14810_e22041: f64 = (locals.var_nuendd * locals.var_admer);
            let assign14810_e22044: f64 = (locals.var_nuintd * locals.var_adsha);
            let assign14810_e22045: f64 = (assign14810_e22041 + assign14810_e22044);
            (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14, ) = (assign14810_e22045, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_temp_adeff_rv = 0.0;
        }

        if ((locals.var_guard474 != 0.0) && (!(((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)) || (locals.var_guard473 != 0.0)))) {
            let assign14820_e22071: f64 = (p.p2 - 1.0);
            let assign14820_e22073: f64 = (assign14820_e22071 * locals.var_pssha);
            let assign14820_e22074: f64 = (locals.var_psiso + assign14820_e22073);
            (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14, ) = (assign14820_e22074, (locals.var_psiso_dn0 + (assign14820_e22071 * locals.var_pssha_dn0)), (locals.var_psiso_dn2 + (assign14820_e22071 * locals.var_pssha_dn2)), (locals.var_psiso_dn3 + (assign14820_e22071 * locals.var_pssha_dn3)), (locals.var_psiso_dn4 + (assign14820_e22071 * locals.var_pssha_dn4)), (locals.var_psiso_dn5 + (assign14820_e22071 * locals.var_pssha_dn5)), (locals.var_psiso_dn6 + (assign14820_e22071 * locals.var_pssha_dn6)), (locals.var_psiso_dn7 + (assign14820_e22071 * locals.var_pssha_dn7)), (locals.var_psiso_dn8 + (assign14820_e22071 * locals.var_pssha_dn8)), (locals.var_psiso_dn9 + (assign14820_e22071 * locals.var_pssha_dn9)), (locals.var_psiso_dn10 + (assign14820_e22071 * locals.var_pssha_dn10)), (locals.var_psiso_dn11 + (assign14820_e22071 * locals.var_pssha_dn11)), (locals.var_psiso_dn12 + (assign14820_e22071 * locals.var_pssha_dn12)), (locals.var_psiso_dn13 + (assign14820_e22071 * locals.var_pssha_dn13)), (locals.var_psiso_dn14 + (assign14820_e22071 * locals.var_pssha_dn14)), );
            locals.var_temp_pseff_rv = 0.0;
        }

        if ((locals.var_guard474 != 0.0) && (!(((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)) || (locals.var_guard473 != 0.0)))) {
            let assign14830_e22099: f64 = (p.p2 * locals.var_pdsha);
            (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14, ) = (assign14830_e22099, (p.p2 * locals.var_pdsha_dn0), (p.p2 * locals.var_pdsha_dn2), (p.p2 * locals.var_pdsha_dn3), (p.p2 * locals.var_pdsha_dn4), (p.p2 * locals.var_pdsha_dn5), (p.p2 * locals.var_pdsha_dn6), (p.p2 * locals.var_pdsha_dn7), (p.p2 * locals.var_pdsha_dn8), (p.p2 * locals.var_pdsha_dn9), (p.p2 * locals.var_pdsha_dn10), (p.p2 * locals.var_pdsha_dn11), (p.p2 * locals.var_pdsha_dn12), (p.p2 * locals.var_pdsha_dn13), (p.p2 * locals.var_pdsha_dn14), );
            locals.var_temp_pdeff_rv = 0.0;
        }

        if ((locals.var_guard474 != 0.0) && (!(((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)) || (locals.var_guard473 != 0.0)))) {
            let assign14840_e22125: f64 = (p.p2 - 1.0);
            let assign14840_e22127: f64 = (assign14840_e22125 * locals.var_assha);
            let assign14840_e22128: f64 = (locals.var_asiso + assign14840_e22127);
            (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14, ) = (assign14840_e22128, locals.var_asiso_dn0, locals.var_asiso_dn2, locals.var_asiso_dn3, locals.var_asiso_dn4, locals.var_asiso_dn5, locals.var_asiso_dn6, locals.var_asiso_dn7, locals.var_asiso_dn8, locals.var_asiso_dn9, locals.var_asiso_dn10, locals.var_asiso_dn11, locals.var_asiso_dn12, locals.var_asiso_dn13, locals.var_asiso_dn14, );
            locals.var_temp_aseff_rv = 0.0;
        }

        if ((locals.var_guard474 != 0.0) && (!(((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)) || (locals.var_guard473 != 0.0)))) {
            let assign14850_e22153: f64 = (p.p2 * locals.var_adsha);
            (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14, ) = (assign14850_e22153, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_temp_adeff_rv = 0.0;
        }

        if ((locals.var_guard475 != 0.0) && (!((((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)) || (locals.var_guard473 != 0.0)) || (locals.var_guard474 != 0.0)))) {
            let assign14860_e22180: f64 = (p.p2 * locals.var_pssha);
            (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14, ) = (assign14860_e22180, (p.p2 * locals.var_pssha_dn0), (p.p2 * locals.var_pssha_dn2), (p.p2 * locals.var_pssha_dn3), (p.p2 * locals.var_pssha_dn4), (p.p2 * locals.var_pssha_dn5), (p.p2 * locals.var_pssha_dn6), (p.p2 * locals.var_pssha_dn7), (p.p2 * locals.var_pssha_dn8), (p.p2 * locals.var_pssha_dn9), (p.p2 * locals.var_pssha_dn10), (p.p2 * locals.var_pssha_dn11), (p.p2 * locals.var_pssha_dn12), (p.p2 * locals.var_pssha_dn13), (p.p2 * locals.var_pssha_dn14), );
            locals.var_temp_pseff_rv = 0.0;
        }

        if ((locals.var_guard475 != 0.0) && (!((((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)) || (locals.var_guard473 != 0.0)) || (locals.var_guard474 != 0.0)))) {
            let assign14870_e22208: f64 = (p.p2 - 1.0);
            let assign14870_e22210: f64 = (assign14870_e22208 * locals.var_pdsha);
            let assign14870_e22211: f64 = (locals.var_pdiso + assign14870_e22210);
            (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14, ) = (assign14870_e22211, (locals.var_pdiso_dn0 + (assign14870_e22208 * locals.var_pdsha_dn0)), (locals.var_pdiso_dn2 + (assign14870_e22208 * locals.var_pdsha_dn2)), (locals.var_pdiso_dn3 + (assign14870_e22208 * locals.var_pdsha_dn3)), (locals.var_pdiso_dn4 + (assign14870_e22208 * locals.var_pdsha_dn4)), (locals.var_pdiso_dn5 + (assign14870_e22208 * locals.var_pdsha_dn5)), (locals.var_pdiso_dn6 + (assign14870_e22208 * locals.var_pdsha_dn6)), (locals.var_pdiso_dn7 + (assign14870_e22208 * locals.var_pdsha_dn7)), (locals.var_pdiso_dn8 + (assign14870_e22208 * locals.var_pdsha_dn8)), (locals.var_pdiso_dn9 + (assign14870_e22208 * locals.var_pdsha_dn9)), (locals.var_pdiso_dn10 + (assign14870_e22208 * locals.var_pdsha_dn10)), (locals.var_pdiso_dn11 + (assign14870_e22208 * locals.var_pdsha_dn11)), (locals.var_pdiso_dn12 + (assign14870_e22208 * locals.var_pdsha_dn12)), (locals.var_pdiso_dn13 + (assign14870_e22208 * locals.var_pdsha_dn13)), (locals.var_pdiso_dn14 + (assign14870_e22208 * locals.var_pdsha_dn14)), );
            locals.var_temp_pdeff_rv = 0.0;
        }

        if ((locals.var_guard475 != 0.0) && (!((((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)) || (locals.var_guard473 != 0.0)) || (locals.var_guard474 != 0.0)))) {
            let assign14880_e22238: f64 = (p.p2 * locals.var_assha);
            (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14, ) = (assign14880_e22238, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_temp_aseff_rv = 0.0;
        }

        if ((locals.var_guard475 != 0.0) && (!((((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)) || (locals.var_guard473 != 0.0)) || (locals.var_guard474 != 0.0)))) {
            let assign14890_e22266: f64 = (p.p2 - 1.0);
            let assign14890_e22268: f64 = (assign14890_e22266 * locals.var_adsha);
            let assign14890_e22269: f64 = (locals.var_adiso + assign14890_e22268);
            (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14, ) = (assign14890_e22269, locals.var_adiso_dn0, locals.var_adiso_dn2, locals.var_adiso_dn3, locals.var_adiso_dn4, locals.var_adiso_dn5, locals.var_adiso_dn6, locals.var_adiso_dn7, locals.var_adiso_dn8, locals.var_adiso_dn9, locals.var_adiso_dn10, locals.var_adiso_dn11, locals.var_adiso_dn12, locals.var_adiso_dn13, locals.var_adiso_dn14, );
            locals.var_temp_adeff_rv = 0.0;
        }

        if (!(((((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)) || (locals.var_guard473 != 0.0)) || (locals.var_guard474 != 0.0)) || (locals.var_guard475 != 0.0))) {
            (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_temp_pseff_rv = 0.0;
            (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_temp_pdeff_rv = 0.0;
            (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_temp_aseff_rv = 0.0;
            (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_temp_adeff_rv = 0.0;
        }

        let assign14940_e22373: f64 = if param_given[24] { 1.0 } else { 0.0 };
        locals.var_guard476 = assign14940_e22373;
        locals.var_guard476_rv = 0.0;

        if (locals.var_guard476 != 0.0) {
            let assign14950_e22377: f64 = (p.p24 * p.p53);
            let assign14950_e22379: f64 = (assign14950_e22377 * p.p52);
            (locals.var_aseff, locals.var_aseff_dn0, locals.var_aseff_dn2, locals.var_aseff_dn3, locals.var_aseff_dn4, locals.var_aseff_dn5, locals.var_aseff_dn6, locals.var_aseff_dn7, locals.var_aseff_dn8, locals.var_aseff_dn9, locals.var_aseff_dn10, locals.var_aseff_dn11, locals.var_aseff_dn12, locals.var_aseff_dn13, locals.var_aseff_dn14, ) = (assign14950_e22379, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_aseff_rv = 0.0;
        }

        if (locals.var_guard476 == 0.0) {
            (locals.var_aseff, locals.var_aseff_dn0, locals.var_aseff_dn2, locals.var_aseff_dn3, locals.var_aseff_dn4, locals.var_aseff_dn5, locals.var_aseff_dn6, locals.var_aseff_dn7, locals.var_aseff_dn8, locals.var_aseff_dn9, locals.var_aseff_dn10, locals.var_aseff_dn11, locals.var_aseff_dn12, locals.var_aseff_dn13, locals.var_aseff_dn14, ) = (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14, );
            locals.var_aseff_rv = 0.0;
        }

        let assign14970_e22389: f64 = if locals.var_aseff < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard477 = assign14970_e22389;
        locals.var_guard477_rv = 0.0;

        if (locals.var_guard477 != 0.0) {
            (locals.var_aseff, locals.var_aseff_dn0, locals.var_aseff_dn2, locals.var_aseff_dn3, locals.var_aseff_dn4, locals.var_aseff_dn5, locals.var_aseff_dn6, locals.var_aseff_dn7, locals.var_aseff_dn8, locals.var_aseff_dn9, locals.var_aseff_dn10, locals.var_aseff_dn11, locals.var_aseff_dn12, locals.var_aseff_dn13, locals.var_aseff_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_aseff_rv = 0.0;
        }

        let assign14990_e22395: f64 = if param_given[25] { 1.0 } else { 0.0 };
        locals.var_guard478 = assign14990_e22395;
        locals.var_guard478_rv = 0.0;

        if (locals.var_guard478 != 0.0) {
            let assign15000_e22399: f64 = (p.p25 * p.p53);
            let assign15000_e22401: f64 = (assign15000_e22399 * p.p52);
            (locals.var_adeff, locals.var_adeff_dn0, locals.var_adeff_dn2, locals.var_adeff_dn3, locals.var_adeff_dn4, locals.var_adeff_dn5, locals.var_adeff_dn6, locals.var_adeff_dn7, locals.var_adeff_dn8, locals.var_adeff_dn9, locals.var_adeff_dn10, locals.var_adeff_dn11, locals.var_adeff_dn12, locals.var_adeff_dn13, locals.var_adeff_dn14, ) = (assign15000_e22401, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_adeff_rv = 0.0;
        }

        if (locals.var_guard478 == 0.0) {
            (locals.var_adeff, locals.var_adeff_dn0, locals.var_adeff_dn2, locals.var_adeff_dn3, locals.var_adeff_dn4, locals.var_adeff_dn5, locals.var_adeff_dn6, locals.var_adeff_dn7, locals.var_adeff_dn8, locals.var_adeff_dn9, locals.var_adeff_dn10, locals.var_adeff_dn11, locals.var_adeff_dn12, locals.var_adeff_dn13, locals.var_adeff_dn14, ) = (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14, );
            locals.var_adeff_rv = 0.0;
        }

        let assign15020_e22411: f64 = if locals.var_adeff < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard479 = assign15020_e22411;
        locals.var_guard479_rv = 0.0;

        if (locals.var_guard479 != 0.0) {
            (locals.var_adeff, locals.var_adeff_dn0, locals.var_adeff_dn2, locals.var_adeff_dn3, locals.var_adeff_dn4, locals.var_adeff_dn5, locals.var_adeff_dn6, locals.var_adeff_dn7, locals.var_adeff_dn8, locals.var_adeff_dn9, locals.var_adeff_dn10, locals.var_adeff_dn11, locals.var_adeff_dn12, locals.var_adeff_dn13, locals.var_adeff_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_adeff_rv = 0.0;
        }

        let assign15040_e22417: f64 = if param_given[26] { 1.0 } else { 0.0 };
        locals.var_guard480 = assign15040_e22417;
        locals.var_guard480_rv = 0.0;

        let assign15050_e22420: f64 = if p.p137 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard481 = assign15050_e22420;
        locals.var_guard481_rv = 0.0;

        if ((locals.var_guard480 != 0.0) && (locals.var_guard481 != 0.0)) {
            let assign15060_e22426: f64 = (p.p26 * p.p53);
            (locals.var_pseff, locals.var_pseff_dn0, locals.var_pseff_dn2, locals.var_pseff_dn3, locals.var_pseff_dn4, locals.var_pseff_dn5, locals.var_pseff_dn6, locals.var_pseff_dn7, locals.var_pseff_dn8, locals.var_pseff_dn9, locals.var_pseff_dn10, locals.var_pseff_dn11, locals.var_pseff_dn12, locals.var_pseff_dn13, locals.var_pseff_dn14, ) = (assign15060_e22426, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_pseff_rv = 0.0;
        }

        if ((locals.var_guard480 != 0.0) && (locals.var_guard481 == 0.0)) {
            let assign15070_e22435: f64 = (p.p26 * p.p53);
            let assign15070_e22438: f64 = (locals.var_weffcj * p.p2);
            let assign15070_e22439: f64 = (assign15070_e22435 - assign15070_e22438);
            let assign15070_e22441: f64 = (assign15070_e22439).max(0.0);
            (locals.var_pseff, locals.var_pseff_dn0, locals.var_pseff_dn2, locals.var_pseff_dn3, locals.var_pseff_dn4, locals.var_pseff_dn5, locals.var_pseff_dn6, locals.var_pseff_dn7, locals.var_pseff_dn8, locals.var_pseff_dn9, locals.var_pseff_dn10, locals.var_pseff_dn11, locals.var_pseff_dn12, locals.var_pseff_dn13, locals.var_pseff_dn14, ) = (assign15070_e22441, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_pseff_rv = 0.0;
        }

        if (locals.var_guard480 == 0.0) {
            (locals.var_pseff, locals.var_pseff_dn0, locals.var_pseff_dn2, locals.var_pseff_dn3, locals.var_pseff_dn4, locals.var_pseff_dn5, locals.var_pseff_dn6, locals.var_pseff_dn7, locals.var_pseff_dn8, locals.var_pseff_dn9, locals.var_pseff_dn10, locals.var_pseff_dn11, locals.var_pseff_dn12, locals.var_pseff_dn13, locals.var_pseff_dn14, ) = (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14, );
            locals.var_pseff_rv = 0.0;
        }

        let assign15090_e22451: f64 = if locals.var_pseff < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard482 = assign15090_e22451;
        locals.var_guard482_rv = 0.0;

        if ((locals.var_guard480 == 0.0) && (locals.var_guard482 != 0.0)) {
            (locals.var_pseff, locals.var_pseff_dn0, locals.var_pseff_dn2, locals.var_pseff_dn3, locals.var_pseff_dn4, locals.var_pseff_dn5, locals.var_pseff_dn6, locals.var_pseff_dn7, locals.var_pseff_dn8, locals.var_pseff_dn9, locals.var_pseff_dn10, locals.var_pseff_dn11, locals.var_pseff_dn12, locals.var_pseff_dn13, locals.var_pseff_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_pseff_rv = 0.0;
        }

        let assign15110_e22460: f64 = if param_given[27] { 1.0 } else { 0.0 };
        locals.var_guard483 = assign15110_e22460;
        locals.var_guard483_rv = 0.0;

        let assign15120_e22463: f64 = if p.p137 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard484 = assign15120_e22463;
        locals.var_guard484_rv = 0.0;

        if ((locals.var_guard483 != 0.0) && (locals.var_guard484 != 0.0)) {
            let assign15130_e22469: f64 = (p.p27 * p.p53);
            (locals.var_pdeff, locals.var_pdeff_dn0, locals.var_pdeff_dn2, locals.var_pdeff_dn3, locals.var_pdeff_dn4, locals.var_pdeff_dn5, locals.var_pdeff_dn6, locals.var_pdeff_dn7, locals.var_pdeff_dn8, locals.var_pdeff_dn9, locals.var_pdeff_dn10, locals.var_pdeff_dn11, locals.var_pdeff_dn12, locals.var_pdeff_dn13, locals.var_pdeff_dn14, ) = (assign15130_e22469, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_pdeff_rv = 0.0;
        }

        if ((locals.var_guard483 != 0.0) && (locals.var_guard484 == 0.0)) {
            let assign15140_e22478: f64 = (p.p27 * p.p53);
            let assign15140_e22481: f64 = (locals.var_weffcj * p.p2);
            let assign15140_e22482: f64 = (assign15140_e22478 - assign15140_e22481);
            let assign15140_e22484: f64 = (assign15140_e22482).max(0.0);
            (locals.var_pdeff, locals.var_pdeff_dn0, locals.var_pdeff_dn2, locals.var_pdeff_dn3, locals.var_pdeff_dn4, locals.var_pdeff_dn5, locals.var_pdeff_dn6, locals.var_pdeff_dn7, locals.var_pdeff_dn8, locals.var_pdeff_dn9, locals.var_pdeff_dn10, locals.var_pdeff_dn11, locals.var_pdeff_dn12, locals.var_pdeff_dn13, locals.var_pdeff_dn14, ) = (assign15140_e22484, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_pdeff_rv = 0.0;
        }

        if (locals.var_guard483 == 0.0) {
            (locals.var_pdeff, locals.var_pdeff_dn0, locals.var_pdeff_dn2, locals.var_pdeff_dn3, locals.var_pdeff_dn4, locals.var_pdeff_dn5, locals.var_pdeff_dn6, locals.var_pdeff_dn7, locals.var_pdeff_dn8, locals.var_pdeff_dn9, locals.var_pdeff_dn10, locals.var_pdeff_dn11, locals.var_pdeff_dn12, locals.var_pdeff_dn13, locals.var_pdeff_dn14, ) = (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14, );
            locals.var_pdeff_rv = 0.0;
        }

        let assign15160_e22494: f64 = if locals.var_pdeff < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard485 = assign15160_e22494;
        locals.var_guard485_rv = 0.0;

        if ((locals.var_guard483 == 0.0) && (locals.var_guard485 != 0.0)) {
            (locals.var_pdeff, locals.var_pdeff_dn0, locals.var_pdeff_dn2, locals.var_pdeff_dn3, locals.var_pdeff_dn4, locals.var_pdeff_dn5, locals.var_pdeff_dn6, locals.var_pdeff_dn7, locals.var_pdeff_dn8, locals.var_pdeff_dn9, locals.var_pdeff_dn10, locals.var_pdeff_dn11, locals.var_pdeff_dn12, locals.var_pdeff_dn13, locals.var_pdeff_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_pdeff_rv = 0.0;
        }

        let assign15180_e22504: f64 = (locals.var_aseff * locals.var_jss_t);
        let assign15180_e22507: f64 = (locals.var_pseff * locals.var_jsws_t);
        let assign15180_e22508: f64 = (assign15180_e22504 + assign15180_e22507);
        let assign15180_e22511: f64 = (locals.var_weffcj * p.p2);
        let assign15180_e22513: f64 = (assign15180_e22511 * locals.var_jswgs_t);
        let assign15180_e22514: f64 = (assign15180_e22508 + assign15180_e22513);
        (locals.var_isbs, locals.var_isbs_dn0, locals.var_isbs_dn2, locals.var_isbs_dn3, locals.var_isbs_dn4, locals.var_isbs_dn5, locals.var_isbs_dn6, locals.var_isbs_dn7, locals.var_isbs_dn8, locals.var_isbs_dn9, locals.var_isbs_dn10, locals.var_isbs_dn11, locals.var_isbs_dn12, locals.var_isbs_dn13, locals.var_isbs_dn14, ) = (assign15180_e22514, ((((locals.var_aseff_dn0 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn0)) + ((locals.var_pseff_dn0 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn0))) + (assign15180_e22511 * locals.var_jswgs_t_dn0)), ((((locals.var_aseff_dn2 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn2)) + ((locals.var_pseff_dn2 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn2))) + (assign15180_e22511 * locals.var_jswgs_t_dn2)), ((((locals.var_aseff_dn3 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn3)) + ((locals.var_pseff_dn3 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn3))) + (assign15180_e22511 * locals.var_jswgs_t_dn3)), ((((locals.var_aseff_dn4 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn4)) + ((locals.var_pseff_dn4 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn4))) + (assign15180_e22511 * locals.var_jswgs_t_dn4)), ((((locals.var_aseff_dn5 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn5)) + ((locals.var_pseff_dn5 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn5))) + (assign15180_e22511 * locals.var_jswgs_t_dn5)), ((((locals.var_aseff_dn6 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn6)) + ((locals.var_pseff_dn6 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn6))) + (assign15180_e22511 * locals.var_jswgs_t_dn6)), ((((locals.var_aseff_dn7 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn7)) + ((locals.var_pseff_dn7 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn7))) + (assign15180_e22511 * locals.var_jswgs_t_dn7)), ((((locals.var_aseff_dn8 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn8)) + ((locals.var_pseff_dn8 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn8))) + (assign15180_e22511 * locals.var_jswgs_t_dn8)), ((((locals.var_aseff_dn9 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn9)) + ((locals.var_pseff_dn9 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn9))) + (assign15180_e22511 * locals.var_jswgs_t_dn9)), ((((locals.var_aseff_dn10 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn10)) + ((locals.var_pseff_dn10 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn10))) + (assign15180_e22511 * locals.var_jswgs_t_dn10)), ((((locals.var_aseff_dn11 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn11)) + ((locals.var_pseff_dn11 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn11))) + (assign15180_e22511 * locals.var_jswgs_t_dn11)), ((((locals.var_aseff_dn12 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn12)) + ((locals.var_pseff_dn12 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn12))) + (assign15180_e22511 * locals.var_jswgs_t_dn12)), ((((locals.var_aseff_dn13 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn13)) + ((locals.var_pseff_dn13 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn13))) + (assign15180_e22511 * locals.var_jswgs_t_dn13)), ((((locals.var_aseff_dn14 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn14)) + ((locals.var_pseff_dn14 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn14))) + (assign15180_e22511 * locals.var_jswgs_t_dn14)), );
        locals.var_isbs_rv = 0.0;

        let assign15190_e22517: f64 = if locals.var_isbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard486 = assign15190_e22517;
        locals.var_guard486_rv = 0.0;

        if (locals.var_guard486 != 0.0) {
            let assign15200_e22521: f64 = (locals.var_vtm * p.p725);
            (locals.var_nvtms, locals.var_nvtms_dn4, ) = (assign15200_e22521, (locals.var_vtm_dn4 * p.p725), );
            locals.var_nvtms_rv = 0.0;
        }

        if (locals.var_guard486 != 0.0) {
            let assign15210_e22526: f64 = (-p.p731);
            let assign15210_e22528: f64 = (assign15210_e22526 / locals.var_nvtms);
            let assign15210_e22529: f64 = { let limited_exp_arg = assign15210_e22528; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
            let assign15210_e22531: f64 = (assign15210_e22529 * p.p733);
            (locals.var_xexpbvs, locals.var_xexpbvs_dn4, ) = (assign15210_e22531, (({ let limited_exp_arg = assign15210_e22528; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-((assign15210_e22526 * locals.var_nvtms_dn4) / (locals.var_nvtms * locals.var_nvtms)))) * p.p733), );
            locals.var_xexpbvs_rv = 0.0;
        }

        if (locals.var_guard486 != 0.0) {
            let assign15220_e22537: f64 = (p.p727 / locals.var_isbs);
            let assign15220_e22539: f64 = (assign15220_e22537).max(10.0);
            (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14, ) = (assign15220_e22539, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn0) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn2) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn3) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn4) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn5) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn6) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn7) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn8) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn9) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn10) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn11) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn12) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn13) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn14) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, );
            locals.var_t2_rv = 0.0;
        }

        if (locals.var_guard486 != 0.0) {
            let assign15230_e22545: f64 = (1.0 + locals.var_t2);
            let assign15230_e22547: f64 = (assign15230_e22545 - locals.var_xexpbvs);
            (locals.var_tb, locals.var_tb_dn0, locals.var_tb_dn2, locals.var_tb_dn3, locals.var_tb_dn4, locals.var_tb_dn5, locals.var_tb_dn6, locals.var_tb_dn7, locals.var_tb_dn8, locals.var_tb_dn9, locals.var_tb_dn10, locals.var_tb_dn11, locals.var_tb_dn12, locals.var_tb_dn13, locals.var_tb_dn14, ) = (assign15230_e22547, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, (locals.var_t2_dn4 - locals.var_xexpbvs_dn4), locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14, );
            locals.var_tb_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_19(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard486 != 0.0) {
            let assign15240_e22556: f64 = (locals.var_tb * locals.var_tb);
            let assign15240_e22559: f64 = (4.0 * locals.var_xexpbvs);
            let assign15240_e22560: f64 = (assign15240_e22556 + assign15240_e22559);
            let assign15240_e22561: f64 = (assign15240_e22560).sqrt();
            let assign15240_e22562: f64 = (locals.var_tb + assign15240_e22561);
            let assign15240_e22563: f64 = (0.5 * assign15240_e22562);
            let assign15240_e22565: f64 = (assign15240_e22563).max(1e-38);
            let assign15240_e22566: f64 = (assign15240_e22565).ln();
            let assign15240_e22567: f64 = (locals.var_nvtms * assign15240_e22566);
            (locals.var_vjsmfwd, locals.var_vjsmfwd_dn0, locals.var_vjsmfwd_dn2, locals.var_vjsmfwd_dn3, locals.var_vjsmfwd_dn4, locals.var_vjsmfwd_dn5, locals.var_vjsmfwd_dn6, locals.var_vjsmfwd_dn7, locals.var_vjsmfwd_dn8, locals.var_vjsmfwd_dn9, locals.var_vjsmfwd_dn10, locals.var_vjsmfwd_dn11, locals.var_vjsmfwd_dn12, locals.var_vjsmfwd_dn13, locals.var_vjsmfwd_dn14, ) = (assign15240_e22567, (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn0 + (((locals.var_tb_dn0 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn0)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565)), (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn2 + (((locals.var_tb_dn2 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn2)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565)), (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn3 + (((locals.var_tb_dn3 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn3)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565)), ((locals.var_nvtms_dn4 * assign15240_e22566) + (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn4 + ((((locals.var_tb_dn4 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn4)) + (4.0 * locals.var_xexpbvs_dn4)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565))), (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn5 + (((locals.var_tb_dn5 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn5)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565)), (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn6 + (((locals.var_tb_dn6 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn6)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565)), (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn7 + (((locals.var_tb_dn7 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn7)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565)), (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn8 + (((locals.var_tb_dn8 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn8)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565)), (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn9 + (((locals.var_tb_dn9 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn9)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565)), (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn10 + (((locals.var_tb_dn10 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn10)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565)), (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn11 + (((locals.var_tb_dn11 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn11)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565)), (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn12 + (((locals.var_tb_dn12 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn12)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565)), (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn13 + (((locals.var_tb_dn13 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn13)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565)), (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn14 + (((locals.var_tb_dn14 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn14)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565)), );
            locals.var_vjsmfwd_rv = 0.0;
        }

        if (locals.var_guard486 != 0.0) {
            let assign15250_e22573: f64 = (locals.var_vjsmfwd / locals.var_nvtms);
            let assign15250_e22574: f64 = { let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign15250_e22574, ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn0 / locals.var_nvtms)), ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn2 / locals.var_nvtms)), ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn3 / locals.var_nvtms)), ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vjsmfwd_dn4 * locals.var_nvtms) - (locals.var_vjsmfwd * locals.var_nvtms_dn4)) / (locals.var_nvtms * locals.var_nvtms))), ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn5 / locals.var_nvtms)), ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn6 / locals.var_nvtms)), ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn7 / locals.var_nvtms)), ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn8 / locals.var_nvtms)), ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn9 / locals.var_nvtms)), ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn10 / locals.var_nvtms)), ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn11 / locals.var_nvtms)), ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn12 / locals.var_nvtms)), ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn13 / locals.var_nvtms)), ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn14 / locals.var_nvtms)), );
            locals.var_t0_rv = 0.0;
        }

        if (locals.var_guard486 != 0.0) {
            let assign15260_e22582: f64 = (locals.var_xexpbvs / locals.var_t0);
            let assign15260_e22583: f64 = (locals.var_t0 - assign15260_e22582);
            let assign15260_e22585: f64 = (assign15260_e22583 + locals.var_xexpbvs);
            let assign15260_e22587: f64 = (assign15260_e22585 - 1.0);
            let assign15260_e22588: f64 = (locals.var_isbs * assign15260_e22587);
            (locals.var_ivjsmfwd, locals.var_ivjsmfwd_dn0, locals.var_ivjsmfwd_dn2, locals.var_ivjsmfwd_dn3, locals.var_ivjsmfwd_dn4, locals.var_ivjsmfwd_dn5, locals.var_ivjsmfwd_dn6, locals.var_ivjsmfwd_dn7, locals.var_ivjsmfwd_dn8, locals.var_ivjsmfwd_dn9, locals.var_ivjsmfwd_dn10, locals.var_ivjsmfwd_dn11, locals.var_ivjsmfwd_dn12, locals.var_ivjsmfwd_dn13, locals.var_ivjsmfwd_dn14, ) = (assign15260_e22588, ((locals.var_isbs_dn0 * assign15260_e22587) + (locals.var_isbs * (locals.var_t0_dn0 - (-((locals.var_xexpbvs * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn2 * assign15260_e22587) + (locals.var_isbs * (locals.var_t0_dn2 - (-((locals.var_xexpbvs * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn3 * assign15260_e22587) + (locals.var_isbs * (locals.var_t0_dn3 - (-((locals.var_xexpbvs * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn4 * assign15260_e22587) + (locals.var_isbs * ((locals.var_t0_dn4 - (((locals.var_xexpbvs_dn4 * locals.var_t0) - (locals.var_xexpbvs * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0))) + locals.var_xexpbvs_dn4))), ((locals.var_isbs_dn5 * assign15260_e22587) + (locals.var_isbs * (locals.var_t0_dn5 - (-((locals.var_xexpbvs * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn6 * assign15260_e22587) + (locals.var_isbs * (locals.var_t0_dn6 - (-((locals.var_xexpbvs * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn7 * assign15260_e22587) + (locals.var_isbs * (locals.var_t0_dn7 - (-((locals.var_xexpbvs * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn8 * assign15260_e22587) + (locals.var_isbs * (locals.var_t0_dn8 - (-((locals.var_xexpbvs * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn9 * assign15260_e22587) + (locals.var_isbs * (locals.var_t0_dn9 - (-((locals.var_xexpbvs * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn10 * assign15260_e22587) + (locals.var_isbs * (locals.var_t0_dn10 - (-((locals.var_xexpbvs * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn11 * assign15260_e22587) + (locals.var_isbs * (locals.var_t0_dn11 - (-((locals.var_xexpbvs * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn12 * assign15260_e22587) + (locals.var_isbs * (locals.var_t0_dn12 - (-((locals.var_xexpbvs * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn13 * assign15260_e22587) + (locals.var_isbs * (locals.var_t0_dn13 - (-((locals.var_xexpbvs * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn14 * assign15260_e22587) + (locals.var_isbs * (locals.var_t0_dn14 - (-((locals.var_xexpbvs * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0)))))), );
            locals.var_ivjsmfwd_rv = 0.0;
        }

        if (locals.var_guard486 != 0.0) {
            let assign15270_e22596: f64 = (locals.var_xexpbvs / locals.var_t0);
            let assign15270_e22597: f64 = (locals.var_t0 + assign15270_e22596);
            let assign15270_e22598: f64 = (locals.var_isbs * assign15270_e22597);
            let assign15270_e22600: f64 = (assign15270_e22598 / locals.var_nvtms);
            (locals.var_sslpfwd, locals.var_sslpfwd_dn0, locals.var_sslpfwd_dn2, locals.var_sslpfwd_dn3, locals.var_sslpfwd_dn4, locals.var_sslpfwd_dn5, locals.var_sslpfwd_dn6, locals.var_sslpfwd_dn7, locals.var_sslpfwd_dn8, locals.var_sslpfwd_dn9, locals.var_sslpfwd_dn10, locals.var_sslpfwd_dn11, locals.var_sslpfwd_dn12, locals.var_sslpfwd_dn13, locals.var_sslpfwd_dn14, ) = (assign15270_e22600, (((locals.var_isbs_dn0 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn0 + (-((locals.var_xexpbvs * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn2 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn2 + (-((locals.var_xexpbvs * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn3 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn3 + (-((locals.var_xexpbvs * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((((locals.var_isbs_dn4 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn4 + (((locals.var_xexpbvs_dn4 * locals.var_t0) - (locals.var_xexpbvs * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0))))) * locals.var_nvtms) - (assign15270_e22598 * locals.var_nvtms_dn4)) / (locals.var_nvtms * locals.var_nvtms)), (((locals.var_isbs_dn5 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn5 + (-((locals.var_xexpbvs * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn6 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn6 + (-((locals.var_xexpbvs * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn7 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn7 + (-((locals.var_xexpbvs * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn8 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn8 + (-((locals.var_xexpbvs * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn9 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn9 + (-((locals.var_xexpbvs * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn10 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn10 + (-((locals.var_xexpbvs * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn11 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn11 + (-((locals.var_xexpbvs * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn12 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn12 + (-((locals.var_xexpbvs * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn13 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn13 + (-((locals.var_xexpbvs * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn14 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn14 + (-((locals.var_xexpbvs * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), );
            locals.var_sslpfwd_rv = 0.0;
        }

        if (locals.var_guard486 != 0.0) {
            let assign15280_e22606: f64 = (p.p729 / locals.var_isbs);
            let assign15280_e22608: f64 = (assign15280_e22606 - 10.0);
            let assign15280_e22610: f64 = (-10000.0);
            let assign15280_e22612: f64 = (assign15280_e22610 * 0.001);
            let (assign15280_e22663, assign15280_e22663_d_n0, assign15280_e22663_d_n2, assign15280_e22663_d_n3, assign15280_e22663_d_n4, assign15280_e22663_d_n5, assign15280_e22663_d_n6, assign15280_e22663_d_n7, assign15280_e22663_d_n8, assign15280_e22663_d_n9, assign15280_e22663_d_n10, assign15280_e22663_d_n11, assign15280_e22663_d_n12, assign15280_e22663_d_n13, assign15280_e22663_d_n14,) = {
    if (!(assign15280_e22608 < assign15280_e22612)) {
        let assign15280_e22618: f64 = (p.p729 / locals.var_isbs);
        let assign15280_e22620: f64 = (assign15280_e22618 - 10.0);
        let assign15280_e22623: f64 = (p.p729 / locals.var_isbs);
        let assign15280_e22625: f64 = (assign15280_e22623 - 10.0);
        let assign15280_e22628: f64 = (p.p729 / locals.var_isbs);
        let assign15280_e22630: f64 = (assign15280_e22628 - 10.0);
        let assign15280_e22631: f64 = (assign15280_e22625 * assign15280_e22630);
        let assign15280_e22634: f64 = (4.0 * 0.001);
        let assign15280_e22636: f64 = (assign15280_e22634 * 0.001);
        let assign15280_e22637: f64 = (assign15280_e22631 + assign15280_e22636);
        let assign15280_e22638: f64 = (assign15280_e22637).sqrt();
        let assign15280_e22639: f64 = (assign15280_e22620 + assign15280_e22638);
        let assign15280_e22640: f64 = (0.5 * assign15280_e22639);
        (assign15280_e22640, (0.5 * ((-((p.p729 * locals.var_isbs_dn0) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn0) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn0) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))), (0.5 * ((-((p.p729 * locals.var_isbs_dn2) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn2) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn2) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))), (0.5 * ((-((p.p729 * locals.var_isbs_dn3) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn3) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn3) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))), (0.5 * ((-((p.p729 * locals.var_isbs_dn4) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn4) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn4) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))), (0.5 * ((-((p.p729 * locals.var_isbs_dn5) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn5) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn5) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))), (0.5 * ((-((p.p729 * locals.var_isbs_dn6) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn6) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn6) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))), (0.5 * ((-((p.p729 * locals.var_isbs_dn7) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn7) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn7) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))), (0.5 * ((-((p.p729 * locals.var_isbs_dn8) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn8) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn8) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))), (0.5 * ((-((p.p729 * locals.var_isbs_dn9) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn9) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn9) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))), (0.5 * ((-((p.p729 * locals.var_isbs_dn10) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn10) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn10) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))), (0.5 * ((-((p.p729 * locals.var_isbs_dn11) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn11) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn11) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))), (0.5 * ((-((p.p729 * locals.var_isbs_dn12) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn12) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn12) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))), (0.5 * ((-((p.p729 * locals.var_isbs_dn13) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn13) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn13) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))), (0.5 * ((-((p.p729 * locals.var_isbs_dn14) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn14) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn14) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))),)
    } else {
        let assign15280_e22643: f64 = (p.p729 / locals.var_isbs);
        let assign15280_e22645: f64 = (assign15280_e22643 - 10.0);
        let assign15280_e22647: f64 = (-10000.0);
        let assign15280_e22649: f64 = (assign15280_e22647 * 0.001);
        let (assign15280_e22662, assign15280_e22662_d_n0, assign15280_e22662_d_n2, assign15280_e22662_d_n3, assign15280_e22662_d_n4, assign15280_e22662_d_n5, assign15280_e22662_d_n6, assign15280_e22662_d_n7, assign15280_e22662_d_n8, assign15280_e22662_d_n9, assign15280_e22662_d_n10, assign15280_e22662_d_n11, assign15280_e22662_d_n12, assign15280_e22662_d_n13, assign15280_e22662_d_n14,) = {
            if (assign15280_e22645 < assign15280_e22649) {
                let assign15280_e22652: f64 = (-0.001);
                let assign15280_e22654: f64 = (assign15280_e22652 * 0.001);
                let assign15280_e22657: f64 = (p.p729 / locals.var_isbs);
                let assign15280_e22659: f64 = (assign15280_e22657 - 10.0);
                let assign15280_e22660: f64 = (assign15280_e22654 / assign15280_e22659);
                (assign15280_e22660, (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn0) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))), (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn2) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))), (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn3) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))), (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn4) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))), (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn5) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))), (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn6) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))), (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn7) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))), (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn8) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))), (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn9) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))), (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn10) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))), (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn11) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))), (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn12) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))), (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn13) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))), (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn14) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign15280_e22662, assign15280_e22662_d_n0, assign15280_e22662_d_n2, assign15280_e22662_d_n3, assign15280_e22662_d_n4, assign15280_e22662_d_n5, assign15280_e22662_d_n6, assign15280_e22662_d_n7, assign15280_e22662_d_n8, assign15280_e22662_d_n9, assign15280_e22662_d_n10, assign15280_e22662_d_n11, assign15280_e22662_d_n12, assign15280_e22662_d_n13, assign15280_e22662_d_n14,)
    }
};
            let assign15280_e22665: f64 = (assign15280_e22663 + 10.0);
            (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14, ) = (assign15280_e22665, assign15280_e22663_d_n0, assign15280_e22663_d_n2, assign15280_e22663_d_n3, assign15280_e22663_d_n4, assign15280_e22663_d_n5, assign15280_e22663_d_n6, assign15280_e22663_d_n7, assign15280_e22663_d_n8, assign15280_e22663_d_n9, assign15280_e22663_d_n10, assign15280_e22663_d_n11, assign15280_e22663_d_n12, assign15280_e22663_d_n13, assign15280_e22663_d_n14, );
            locals.var_t2_rv = 0.0;
        }

        if (locals.var_guard486 != 0.0) {
            let assign15290_e22670: f64 = (-p.p731);
            let assign15290_e22674: f64 = (locals.var_t2 - 1.0);
            let assign15290_e22676: f64 = (assign15290_e22674 / p.p733);
            let assign15290_e22678: f64 = (assign15290_e22676).max(1e-38);
            let assign15290_e22679: f64 = (assign15290_e22678).ln();
            let assign15290_e22680: f64 = (locals.var_nvtms * assign15290_e22679);
            let assign15290_e22681: f64 = (assign15290_e22670 - assign15290_e22680);
            (locals.var_vjsmrev, locals.var_vjsmrev_dn0, locals.var_vjsmrev_dn2, locals.var_vjsmrev_dn3, locals.var_vjsmrev_dn4, locals.var_vjsmrev_dn5, locals.var_vjsmrev_dn6, locals.var_vjsmrev_dn7, locals.var_vjsmrev_dn8, locals.var_vjsmrev_dn9, locals.var_vjsmrev_dn10, locals.var_vjsmrev_dn11, locals.var_vjsmrev_dn12, locals.var_vjsmrev_dn13, locals.var_vjsmrev_dn14, ) = (assign15290_e22681, (-(locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn0 / p.p733) } else { 0.0 } / assign15290_e22678))), (-(locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn2 / p.p733) } else { 0.0 } / assign15290_e22678))), (-(locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn3 / p.p733) } else { 0.0 } / assign15290_e22678))), (-((locals.var_nvtms_dn4 * assign15290_e22679) + (locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn4 / p.p733) } else { 0.0 } / assign15290_e22678)))), (-(locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn5 / p.p733) } else { 0.0 } / assign15290_e22678))), (-(locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn6 / p.p733) } else { 0.0 } / assign15290_e22678))), (-(locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn7 / p.p733) } else { 0.0 } / assign15290_e22678))), (-(locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn8 / p.p733) } else { 0.0 } / assign15290_e22678))), (-(locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn9 / p.p733) } else { 0.0 } / assign15290_e22678))), (-(locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn10 / p.p733) } else { 0.0 } / assign15290_e22678))), (-(locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn11 / p.p733) } else { 0.0 } / assign15290_e22678))), (-(locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn12 / p.p733) } else { 0.0 } / assign15290_e22678))), (-(locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn13 / p.p733) } else { 0.0 } / assign15290_e22678))), (-(locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn14 / p.p733) } else { 0.0 } / assign15290_e22678))), );
            locals.var_vjsmrev_rv = 0.0;
        }

        if (locals.var_guard486 != 0.0) {
            let assign15300_e22688: f64 = (p.p731 + locals.var_vjsmrev);
            let assign15300_e22689: f64 = (-assign15300_e22688);
            let assign15300_e22691: f64 = (assign15300_e22689 / locals.var_nvtms);
            let assign15300_e22692: f64 = { let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
            let assign15300_e22693: f64 = (p.p733 * assign15300_e22692);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, ) = (assign15300_e22693, (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn0) / locals.var_nvtms))), (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn2) / locals.var_nvtms))), (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn3) / locals.var_nvtms))), (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((-locals.var_vjsmrev_dn4) * locals.var_nvtms) - (assign15300_e22689 * locals.var_nvtms_dn4)) / (locals.var_nvtms * locals.var_nvtms)))), (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn5) / locals.var_nvtms))), (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn6) / locals.var_nvtms))), (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn7) / locals.var_nvtms))), (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn8) / locals.var_nvtms))), (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn9) / locals.var_nvtms))), (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn10) / locals.var_nvtms))), (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn11) / locals.var_nvtms))), (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn12) / locals.var_nvtms))), (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn13) / locals.var_nvtms))), (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn14) / locals.var_nvtms))), );
            locals.var_t1_rv = 0.0;
        }

        if (locals.var_guard486 != 0.0) {
            let assign15310_e22700: f64 = (1.0 + locals.var_t1);
            let assign15310_e22701: f64 = (locals.var_isbs * assign15310_e22700);
            (locals.var_ivjsmrev, locals.var_ivjsmrev_dn0, locals.var_ivjsmrev_dn2, locals.var_ivjsmrev_dn3, locals.var_ivjsmrev_dn4, locals.var_ivjsmrev_dn5, locals.var_ivjsmrev_dn6, locals.var_ivjsmrev_dn7, locals.var_ivjsmrev_dn8, locals.var_ivjsmrev_dn9, locals.var_ivjsmrev_dn10, locals.var_ivjsmrev_dn11, locals.var_ivjsmrev_dn12, locals.var_ivjsmrev_dn13, locals.var_ivjsmrev_dn14, ) = (assign15310_e22701, ((locals.var_isbs_dn0 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn0)), ((locals.var_isbs_dn2 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn2)), ((locals.var_isbs_dn3 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn3)), ((locals.var_isbs_dn4 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn4)), ((locals.var_isbs_dn5 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn5)), ((locals.var_isbs_dn6 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn6)), ((locals.var_isbs_dn7 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn7)), ((locals.var_isbs_dn8 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn8)), ((locals.var_isbs_dn9 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn9)), ((locals.var_isbs_dn10 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn10)), ((locals.var_isbs_dn11 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn11)), ((locals.var_isbs_dn12 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn12)), ((locals.var_isbs_dn13 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn13)), ((locals.var_isbs_dn14 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn14)), );
            locals.var_ivjsmrev_rv = 0.0;
        }

        if (locals.var_guard486 != 0.0) {
            let assign15320_e22706: f64 = (-locals.var_isbs);
            let assign15320_e22708: f64 = (assign15320_e22706 * locals.var_t1);
            let assign15320_e22710: f64 = (assign15320_e22708 / locals.var_nvtms);
            (locals.var_sslprev, locals.var_sslprev_dn0, locals.var_sslprev_dn2, locals.var_sslprev_dn3, locals.var_sslprev_dn4, locals.var_sslprev_dn5, locals.var_sslprev_dn6, locals.var_sslprev_dn7, locals.var_sslprev_dn8, locals.var_sslprev_dn9, locals.var_sslprev_dn10, locals.var_sslprev_dn11, locals.var_sslprev_dn12, locals.var_sslprev_dn13, locals.var_sslprev_dn14, ) = (assign15320_e22710, ((((-locals.var_isbs_dn0) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn0)) / locals.var_nvtms), ((((-locals.var_isbs_dn2) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn2)) / locals.var_nvtms), ((((-locals.var_isbs_dn3) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn3)) / locals.var_nvtms), ((((((-locals.var_isbs_dn4) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn4)) * locals.var_nvtms) - (assign15320_e22708 * locals.var_nvtms_dn4)) / (locals.var_nvtms * locals.var_nvtms)), ((((-locals.var_isbs_dn5) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn5)) / locals.var_nvtms), ((((-locals.var_isbs_dn6) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn6)) / locals.var_nvtms), ((((-locals.var_isbs_dn7) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn7)) / locals.var_nvtms), ((((-locals.var_isbs_dn8) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn8)) / locals.var_nvtms), ((((-locals.var_isbs_dn9) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn9)) / locals.var_nvtms), ((((-locals.var_isbs_dn10) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn10)) / locals.var_nvtms), ((((-locals.var_isbs_dn11) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn11)) / locals.var_nvtms), ((((-locals.var_isbs_dn12) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn12)) / locals.var_nvtms), ((((-locals.var_isbs_dn13) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn13)) / locals.var_nvtms), ((((-locals.var_isbs_dn14) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn14)) / locals.var_nvtms), );
            locals.var_sslprev_rv = 0.0;
        }

        if (locals.var_guard486 == 0.0) {
            (locals.var_nvtms, locals.var_nvtms_dn4, ) = (0.0, 0.0, );
            locals.var_nvtms_rv = 0.0;
            (locals.var_xexpbvs, locals.var_xexpbvs_dn4, ) = (0.0, 0.0, );
            locals.var_xexpbvs_rv = 0.0;
            (locals.var_vjsmfwd, locals.var_vjsmfwd_dn0, locals.var_vjsmfwd_dn2, locals.var_vjsmfwd_dn3, locals.var_vjsmfwd_dn4, locals.var_vjsmfwd_dn5, locals.var_vjsmfwd_dn6, locals.var_vjsmfwd_dn7, locals.var_vjsmfwd_dn8, locals.var_vjsmfwd_dn9, locals.var_vjsmfwd_dn10, locals.var_vjsmfwd_dn11, locals.var_vjsmfwd_dn12, locals.var_vjsmfwd_dn13, locals.var_vjsmfwd_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_vjsmfwd_rv = 0.0;
            (locals.var_ivjsmfwd, locals.var_ivjsmfwd_dn0, locals.var_ivjsmfwd_dn2, locals.var_ivjsmfwd_dn3, locals.var_ivjsmfwd_dn4, locals.var_ivjsmfwd_dn5, locals.var_ivjsmfwd_dn6, locals.var_ivjsmfwd_dn7, locals.var_ivjsmfwd_dn8, locals.var_ivjsmfwd_dn9, locals.var_ivjsmfwd_dn10, locals.var_ivjsmfwd_dn11, locals.var_ivjsmfwd_dn12, locals.var_ivjsmfwd_dn13, locals.var_ivjsmfwd_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_ivjsmfwd_rv = 0.0;
            (locals.var_sslpfwd, locals.var_sslpfwd_dn0, locals.var_sslpfwd_dn2, locals.var_sslpfwd_dn3, locals.var_sslpfwd_dn4, locals.var_sslpfwd_dn5, locals.var_sslpfwd_dn6, locals.var_sslpfwd_dn7, locals.var_sslpfwd_dn8, locals.var_sslpfwd_dn9, locals.var_sslpfwd_dn10, locals.var_sslpfwd_dn11, locals.var_sslpfwd_dn12, locals.var_sslpfwd_dn13, locals.var_sslpfwd_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_sslpfwd_rv = 0.0;
            (locals.var_vjsmrev, locals.var_vjsmrev_dn0, locals.var_vjsmrev_dn2, locals.var_vjsmrev_dn3, locals.var_vjsmrev_dn4, locals.var_vjsmrev_dn5, locals.var_vjsmrev_dn6, locals.var_vjsmrev_dn7, locals.var_vjsmrev_dn8, locals.var_vjsmrev_dn9, locals.var_vjsmrev_dn10, locals.var_vjsmrev_dn11, locals.var_vjsmrev_dn12, locals.var_vjsmrev_dn13, locals.var_vjsmrev_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_vjsmrev_rv = 0.0;
            (locals.var_ivjsmrev, locals.var_ivjsmrev_dn0, locals.var_ivjsmrev_dn2, locals.var_ivjsmrev_dn3, locals.var_ivjsmrev_dn4, locals.var_ivjsmrev_dn5, locals.var_ivjsmrev_dn6, locals.var_ivjsmrev_dn7, locals.var_ivjsmrev_dn8, locals.var_ivjsmrev_dn9, locals.var_ivjsmrev_dn10, locals.var_ivjsmrev_dn11, locals.var_ivjsmrev_dn12, locals.var_ivjsmrev_dn13, locals.var_ivjsmrev_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_ivjsmrev_rv = 0.0;
            (locals.var_sslprev, locals.var_sslprev_dn0, locals.var_sslprev_dn2, locals.var_sslprev_dn3, locals.var_sslprev_dn4, locals.var_sslprev_dn5, locals.var_sslprev_dn6, locals.var_sslprev_dn7, locals.var_sslprev_dn8, locals.var_sslprev_dn9, locals.var_sslprev_dn10, locals.var_sslprev_dn11, locals.var_sslprev_dn12, locals.var_sslprev_dn13, locals.var_sslprev_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_sslprev_rv = 0.0;
        }

        let assign15410_e22755: f64 = (locals.var_adeff * locals.var_jsd_t);
        let assign15410_e22758: f64 = (locals.var_pdeff * locals.var_jswd_t);
        let assign15410_e22759: f64 = (assign15410_e22755 + assign15410_e22758);
        let assign15410_e22762: f64 = (locals.var_weffcj * p.p2);
        let assign15410_e22764: f64 = (assign15410_e22762 * locals.var_jswgd_t);
        let assign15410_e22765: f64 = (assign15410_e22759 + assign15410_e22764);
        (locals.var_isbd, locals.var_isbd_dn0, locals.var_isbd_dn2, locals.var_isbd_dn3, locals.var_isbd_dn4, locals.var_isbd_dn5, locals.var_isbd_dn6, locals.var_isbd_dn7, locals.var_isbd_dn8, locals.var_isbd_dn9, locals.var_isbd_dn10, locals.var_isbd_dn11, locals.var_isbd_dn12, locals.var_isbd_dn13, locals.var_isbd_dn14, ) = (assign15410_e22765, ((((locals.var_adeff_dn0 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn0)) + ((locals.var_pdeff_dn0 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn0))) + (assign15410_e22762 * locals.var_jswgd_t_dn0)), ((((locals.var_adeff_dn2 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn2)) + ((locals.var_pdeff_dn2 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn2))) + (assign15410_e22762 * locals.var_jswgd_t_dn2)), ((((locals.var_adeff_dn3 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn3)) + ((locals.var_pdeff_dn3 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn3))) + (assign15410_e22762 * locals.var_jswgd_t_dn3)), ((((locals.var_adeff_dn4 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn4)) + ((locals.var_pdeff_dn4 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn4))) + (assign15410_e22762 * locals.var_jswgd_t_dn4)), ((((locals.var_adeff_dn5 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn5)) + ((locals.var_pdeff_dn5 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn5))) + (assign15410_e22762 * locals.var_jswgd_t_dn5)), ((((locals.var_adeff_dn6 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn6)) + ((locals.var_pdeff_dn6 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn6))) + (assign15410_e22762 * locals.var_jswgd_t_dn6)), ((((locals.var_adeff_dn7 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn7)) + ((locals.var_pdeff_dn7 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn7))) + (assign15410_e22762 * locals.var_jswgd_t_dn7)), ((((locals.var_adeff_dn8 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn8)) + ((locals.var_pdeff_dn8 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn8))) + (assign15410_e22762 * locals.var_jswgd_t_dn8)), ((((locals.var_adeff_dn9 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn9)) + ((locals.var_pdeff_dn9 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn9))) + (assign15410_e22762 * locals.var_jswgd_t_dn9)), ((((locals.var_adeff_dn10 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn10)) + ((locals.var_pdeff_dn10 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn10))) + (assign15410_e22762 * locals.var_jswgd_t_dn10)), ((((locals.var_adeff_dn11 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn11)) + ((locals.var_pdeff_dn11 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn11))) + (assign15410_e22762 * locals.var_jswgd_t_dn11)), ((((locals.var_adeff_dn12 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn12)) + ((locals.var_pdeff_dn12 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn12))) + (assign15410_e22762 * locals.var_jswgd_t_dn12)), ((((locals.var_adeff_dn13 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn13)) + ((locals.var_pdeff_dn13 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn13))) + (assign15410_e22762 * locals.var_jswgd_t_dn13)), ((((locals.var_adeff_dn14 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn14)) + ((locals.var_pdeff_dn14 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn14))) + (assign15410_e22762 * locals.var_jswgd_t_dn14)), );
        locals.var_isbd_rv = 0.0;

        let assign15420_e22768: f64 = if locals.var_isbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard487 = assign15420_e22768;
        locals.var_guard487_rv = 0.0;

        if (locals.var_guard487 != 0.0) {
            let assign15430_e22772: f64 = (locals.var_vtm * p.p726);
            (locals.var_nvtmd, locals.var_nvtmd_dn4, ) = (assign15430_e22772, (locals.var_vtm_dn4 * p.p726), );
            locals.var_nvtmd_rv = 0.0;
        }

        if (locals.var_guard487 != 0.0) {
            let assign15440_e22777: f64 = (-p.p732);
            let assign15440_e22779: f64 = (assign15440_e22777 / locals.var_nvtmd);
            let assign15440_e22780: f64 = { let limited_exp_arg = assign15440_e22779; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
            let assign15440_e22782: f64 = (assign15440_e22780 * p.p734);
            (locals.var_xexpbvd, locals.var_xexpbvd_dn4, ) = (assign15440_e22782, (({ let limited_exp_arg = assign15440_e22779; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-((assign15440_e22777 * locals.var_nvtmd_dn4) / (locals.var_nvtmd * locals.var_nvtmd)))) * p.p734), );
            locals.var_xexpbvd_rv = 0.0;
        }

        if (locals.var_guard487 != 0.0) {
            let assign15450_e22788: f64 = (p.p728 / locals.var_isbd);
            let assign15450_e22790: f64 = (assign15450_e22788).max(10.0);
            (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14, ) = (assign15450_e22790, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn0) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn2) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn3) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn4) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn5) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn6) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn7) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn8) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn9) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn10) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn11) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn12) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn13) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn14) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, );
            locals.var_t2_rv = 0.0;
        }

        if (locals.var_guard487 != 0.0) {
            let assign15460_e22796: f64 = (1.0 + locals.var_t2);
            let assign15460_e22798: f64 = (assign15460_e22796 - locals.var_xexpbvd);
            (locals.var_tb, locals.var_tb_dn0, locals.var_tb_dn2, locals.var_tb_dn3, locals.var_tb_dn4, locals.var_tb_dn5, locals.var_tb_dn6, locals.var_tb_dn7, locals.var_tb_dn8, locals.var_tb_dn9, locals.var_tb_dn10, locals.var_tb_dn11, locals.var_tb_dn12, locals.var_tb_dn13, locals.var_tb_dn14, ) = (assign15460_e22798, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, (locals.var_t2_dn4 - locals.var_xexpbvd_dn4), locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14, );
            locals.var_tb_rv = 0.0;
        }

        if (locals.var_guard487 != 0.0) {
            let assign15470_e22807: f64 = (locals.var_tb * locals.var_tb);
            let assign15470_e22810: f64 = (4.0 * locals.var_xexpbvd);
            let assign15470_e22811: f64 = (assign15470_e22807 + assign15470_e22810);
            let assign15470_e22812: f64 = (assign15470_e22811).sqrt();
            let assign15470_e22813: f64 = (locals.var_tb + assign15470_e22812);
            let assign15470_e22814: f64 = (0.5 * assign15470_e22813);
            let assign15470_e22816: f64 = (assign15470_e22814).max(1e-38);
            let assign15470_e22817: f64 = (assign15470_e22816).ln();
            let assign15470_e22818: f64 = (locals.var_nvtmd * assign15470_e22817);
            (locals.var_vjdmfwd, locals.var_vjdmfwd_dn0, locals.var_vjdmfwd_dn2, locals.var_vjdmfwd_dn3, locals.var_vjdmfwd_dn4, locals.var_vjdmfwd_dn5, locals.var_vjdmfwd_dn6, locals.var_vjdmfwd_dn7, locals.var_vjdmfwd_dn8, locals.var_vjdmfwd_dn9, locals.var_vjdmfwd_dn10, locals.var_vjdmfwd_dn11, locals.var_vjdmfwd_dn12, locals.var_vjdmfwd_dn13, locals.var_vjdmfwd_dn14, ) = (assign15470_e22818, (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn0 + (((locals.var_tb_dn0 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn0)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816)), (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn2 + (((locals.var_tb_dn2 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn2)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816)), (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn3 + (((locals.var_tb_dn3 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn3)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816)), ((locals.var_nvtmd_dn4 * assign15470_e22817) + (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn4 + ((((locals.var_tb_dn4 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn4)) + (4.0 * locals.var_xexpbvd_dn4)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816))), (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn5 + (((locals.var_tb_dn5 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn5)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816)), (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn6 + (((locals.var_tb_dn6 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn6)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816)), (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn7 + (((locals.var_tb_dn7 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn7)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816)), (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn8 + (((locals.var_tb_dn8 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn8)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816)), (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn9 + (((locals.var_tb_dn9 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn9)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816)), (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn10 + (((locals.var_tb_dn10 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn10)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816)), (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn11 + (((locals.var_tb_dn11 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn11)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816)), (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn12 + (((locals.var_tb_dn12 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn12)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816)), (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn13 + (((locals.var_tb_dn13 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn13)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816)), (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn14 + (((locals.var_tb_dn14 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn14)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816)), );
            locals.var_vjdmfwd_rv = 0.0;
        }

        if (locals.var_guard487 != 0.0) {
            let assign15480_e22824: f64 = (locals.var_vjdmfwd / locals.var_nvtmd);
            let assign15480_e22825: f64 = { let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign15480_e22825, ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn0 / locals.var_nvtmd)), ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn2 / locals.var_nvtmd)), ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn3 / locals.var_nvtmd)), ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vjdmfwd_dn4 * locals.var_nvtmd) - (locals.var_vjdmfwd * locals.var_nvtmd_dn4)) / (locals.var_nvtmd * locals.var_nvtmd))), ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn5 / locals.var_nvtmd)), ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn6 / locals.var_nvtmd)), ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn7 / locals.var_nvtmd)), ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn8 / locals.var_nvtmd)), ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn9 / locals.var_nvtmd)), ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn10 / locals.var_nvtmd)), ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn11 / locals.var_nvtmd)), ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn12 / locals.var_nvtmd)), ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn13 / locals.var_nvtmd)), ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn14 / locals.var_nvtmd)), );
            locals.var_t0_rv = 0.0;
        }

        if (locals.var_guard487 != 0.0) {
            let assign15490_e22833: f64 = (locals.var_xexpbvd / locals.var_t0);
            let assign15490_e22834: f64 = (locals.var_t0 - assign15490_e22833);
            let assign15490_e22836: f64 = (assign15490_e22834 + locals.var_xexpbvd);
            let assign15490_e22838: f64 = (assign15490_e22836 - 1.0);
            let assign15490_e22839: f64 = (locals.var_isbd * assign15490_e22838);
            (locals.var_ivjdmfwd, locals.var_ivjdmfwd_dn0, locals.var_ivjdmfwd_dn2, locals.var_ivjdmfwd_dn3, locals.var_ivjdmfwd_dn4, locals.var_ivjdmfwd_dn5, locals.var_ivjdmfwd_dn6, locals.var_ivjdmfwd_dn7, locals.var_ivjdmfwd_dn8, locals.var_ivjdmfwd_dn9, locals.var_ivjdmfwd_dn10, locals.var_ivjdmfwd_dn11, locals.var_ivjdmfwd_dn12, locals.var_ivjdmfwd_dn13, locals.var_ivjdmfwd_dn14, ) = (assign15490_e22839, ((locals.var_isbd_dn0 * assign15490_e22838) + (locals.var_isbd * (locals.var_t0_dn0 - (-((locals.var_xexpbvd * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn2 * assign15490_e22838) + (locals.var_isbd * (locals.var_t0_dn2 - (-((locals.var_xexpbvd * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn3 * assign15490_e22838) + (locals.var_isbd * (locals.var_t0_dn3 - (-((locals.var_xexpbvd * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn4 * assign15490_e22838) + (locals.var_isbd * ((locals.var_t0_dn4 - (((locals.var_xexpbvd_dn4 * locals.var_t0) - (locals.var_xexpbvd * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0))) + locals.var_xexpbvd_dn4))), ((locals.var_isbd_dn5 * assign15490_e22838) + (locals.var_isbd * (locals.var_t0_dn5 - (-((locals.var_xexpbvd * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn6 * assign15490_e22838) + (locals.var_isbd * (locals.var_t0_dn6 - (-((locals.var_xexpbvd * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn7 * assign15490_e22838) + (locals.var_isbd * (locals.var_t0_dn7 - (-((locals.var_xexpbvd * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn8 * assign15490_e22838) + (locals.var_isbd * (locals.var_t0_dn8 - (-((locals.var_xexpbvd * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn9 * assign15490_e22838) + (locals.var_isbd * (locals.var_t0_dn9 - (-((locals.var_xexpbvd * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn10 * assign15490_e22838) + (locals.var_isbd * (locals.var_t0_dn10 - (-((locals.var_xexpbvd * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn11 * assign15490_e22838) + (locals.var_isbd * (locals.var_t0_dn11 - (-((locals.var_xexpbvd * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn12 * assign15490_e22838) + (locals.var_isbd * (locals.var_t0_dn12 - (-((locals.var_xexpbvd * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn13 * assign15490_e22838) + (locals.var_isbd * (locals.var_t0_dn13 - (-((locals.var_xexpbvd * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn14 * assign15490_e22838) + (locals.var_isbd * (locals.var_t0_dn14 - (-((locals.var_xexpbvd * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0)))))), );
            locals.var_ivjdmfwd_rv = 0.0;
        }

        if (locals.var_guard487 != 0.0) {
            let assign15500_e22847: f64 = (locals.var_xexpbvd / locals.var_t0);
            let assign15500_e22848: f64 = (locals.var_t0 + assign15500_e22847);
            let assign15500_e22849: f64 = (locals.var_isbd * assign15500_e22848);
            let assign15500_e22851: f64 = (assign15500_e22849 / locals.var_nvtmd);
            (locals.var_dslpfwd, locals.var_dslpfwd_dn0, locals.var_dslpfwd_dn2, locals.var_dslpfwd_dn3, locals.var_dslpfwd_dn4, locals.var_dslpfwd_dn5, locals.var_dslpfwd_dn6, locals.var_dslpfwd_dn7, locals.var_dslpfwd_dn8, locals.var_dslpfwd_dn9, locals.var_dslpfwd_dn10, locals.var_dslpfwd_dn11, locals.var_dslpfwd_dn12, locals.var_dslpfwd_dn13, locals.var_dslpfwd_dn14, ) = (assign15500_e22851, (((locals.var_isbd_dn0 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn0 + (-((locals.var_xexpbvd * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn2 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn2 + (-((locals.var_xexpbvd * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn3 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn3 + (-((locals.var_xexpbvd * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((((locals.var_isbd_dn4 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn4 + (((locals.var_xexpbvd_dn4 * locals.var_t0) - (locals.var_xexpbvd * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0))))) * locals.var_nvtmd) - (assign15500_e22849 * locals.var_nvtmd_dn4)) / (locals.var_nvtmd * locals.var_nvtmd)), (((locals.var_isbd_dn5 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn5 + (-((locals.var_xexpbvd * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn6 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn6 + (-((locals.var_xexpbvd * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn7 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn7 + (-((locals.var_xexpbvd * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn8 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn8 + (-((locals.var_xexpbvd * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn9 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn9 + (-((locals.var_xexpbvd * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn10 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn10 + (-((locals.var_xexpbvd * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn11 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn11 + (-((locals.var_xexpbvd * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn12 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn12 + (-((locals.var_xexpbvd * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn13 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn13 + (-((locals.var_xexpbvd * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn14 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn14 + (-((locals.var_xexpbvd * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), );
            locals.var_dslpfwd_rv = 0.0;
        }

        if (locals.var_guard487 != 0.0) {
            let assign15510_e22857: f64 = (p.p730 / locals.var_isbd);
            let assign15510_e22859: f64 = (assign15510_e22857 - 10.0);
            let assign15510_e22861: f64 = (-10000.0);
            let assign15510_e22863: f64 = (assign15510_e22861 * 0.001);
            let (assign15510_e22914, assign15510_e22914_d_n0, assign15510_e22914_d_n2, assign15510_e22914_d_n3, assign15510_e22914_d_n4, assign15510_e22914_d_n5, assign15510_e22914_d_n6, assign15510_e22914_d_n7, assign15510_e22914_d_n8, assign15510_e22914_d_n9, assign15510_e22914_d_n10, assign15510_e22914_d_n11, assign15510_e22914_d_n12, assign15510_e22914_d_n13, assign15510_e22914_d_n14,) = {
    if (!(assign15510_e22859 < assign15510_e22863)) {
        let assign15510_e22869: f64 = (p.p730 / locals.var_isbd);
        let assign15510_e22871: f64 = (assign15510_e22869 - 10.0);
        let assign15510_e22874: f64 = (p.p730 / locals.var_isbd);
        let assign15510_e22876: f64 = (assign15510_e22874 - 10.0);
        let assign15510_e22879: f64 = (p.p730 / locals.var_isbd);
        let assign15510_e22881: f64 = (assign15510_e22879 - 10.0);
        let assign15510_e22882: f64 = (assign15510_e22876 * assign15510_e22881);
        let assign15510_e22885: f64 = (4.0 * 0.001);
        let assign15510_e22887: f64 = (assign15510_e22885 * 0.001);
        let assign15510_e22888: f64 = (assign15510_e22882 + assign15510_e22887);
        let assign15510_e22889: f64 = (assign15510_e22888).sqrt();
        let assign15510_e22890: f64 = (assign15510_e22871 + assign15510_e22889);
        let assign15510_e22891: f64 = (0.5 * assign15510_e22890);
        (assign15510_e22891, (0.5 * ((-((p.p730 * locals.var_isbd_dn0) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn0) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn0) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))), (0.5 * ((-((p.p730 * locals.var_isbd_dn2) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn2) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn2) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))), (0.5 * ((-((p.p730 * locals.var_isbd_dn3) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn3) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn3) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))), (0.5 * ((-((p.p730 * locals.var_isbd_dn4) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn4) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn4) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))), (0.5 * ((-((p.p730 * locals.var_isbd_dn5) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn5) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn5) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))), (0.5 * ((-((p.p730 * locals.var_isbd_dn6) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn6) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn6) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))), (0.5 * ((-((p.p730 * locals.var_isbd_dn7) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn7) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn7) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))), (0.5 * ((-((p.p730 * locals.var_isbd_dn8) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn8) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn8) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))), (0.5 * ((-((p.p730 * locals.var_isbd_dn9) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn9) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn9) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))), (0.5 * ((-((p.p730 * locals.var_isbd_dn10) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn10) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn10) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))), (0.5 * ((-((p.p730 * locals.var_isbd_dn11) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn11) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn11) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))), (0.5 * ((-((p.p730 * locals.var_isbd_dn12) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn12) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn12) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))), (0.5 * ((-((p.p730 * locals.var_isbd_dn13) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn13) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn13) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))), (0.5 * ((-((p.p730 * locals.var_isbd_dn14) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn14) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn14) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))),)
    } else {
        let assign15510_e22894: f64 = (p.p730 / locals.var_isbd);
        let assign15510_e22896: f64 = (assign15510_e22894 - 10.0);
        let assign15510_e22898: f64 = (-10000.0);
        let assign15510_e22900: f64 = (assign15510_e22898 * 0.001);
        let (assign15510_e22913, assign15510_e22913_d_n0, assign15510_e22913_d_n2, assign15510_e22913_d_n3, assign15510_e22913_d_n4, assign15510_e22913_d_n5, assign15510_e22913_d_n6, assign15510_e22913_d_n7, assign15510_e22913_d_n8, assign15510_e22913_d_n9, assign15510_e22913_d_n10, assign15510_e22913_d_n11, assign15510_e22913_d_n12, assign15510_e22913_d_n13, assign15510_e22913_d_n14,) = {
            if (assign15510_e22896 < assign15510_e22900) {
                let assign15510_e22903: f64 = (-0.001);
                let assign15510_e22905: f64 = (assign15510_e22903 * 0.001);
                let assign15510_e22908: f64 = (p.p730 / locals.var_isbd);
                let assign15510_e22910: f64 = (assign15510_e22908 - 10.0);
                let assign15510_e22911: f64 = (assign15510_e22905 / assign15510_e22910);
                (assign15510_e22911, (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn0) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))), (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn2) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))), (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn3) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))), (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn4) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))), (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn5) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))), (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn6) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))), (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn7) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))), (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn8) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))), (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn9) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))), (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn10) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))), (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn11) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))), (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn12) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))), (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn13) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))), (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn14) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign15510_e22913, assign15510_e22913_d_n0, assign15510_e22913_d_n2, assign15510_e22913_d_n3, assign15510_e22913_d_n4, assign15510_e22913_d_n5, assign15510_e22913_d_n6, assign15510_e22913_d_n7, assign15510_e22913_d_n8, assign15510_e22913_d_n9, assign15510_e22913_d_n10, assign15510_e22913_d_n11, assign15510_e22913_d_n12, assign15510_e22913_d_n13, assign15510_e22913_d_n14,)
    }
};
            let assign15510_e22916: f64 = (assign15510_e22914 + 10.0);
            (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14, ) = (assign15510_e22916, assign15510_e22914_d_n0, assign15510_e22914_d_n2, assign15510_e22914_d_n3, assign15510_e22914_d_n4, assign15510_e22914_d_n5, assign15510_e22914_d_n6, assign15510_e22914_d_n7, assign15510_e22914_d_n8, assign15510_e22914_d_n9, assign15510_e22914_d_n10, assign15510_e22914_d_n11, assign15510_e22914_d_n12, assign15510_e22914_d_n13, assign15510_e22914_d_n14, );
            locals.var_t2_rv = 0.0;
        }

        if (locals.var_guard487 != 0.0) {
            let assign15520_e22921: f64 = (-p.p732);
            let assign15520_e22925: f64 = (locals.var_t2 - 1.0);
            let assign15520_e22927: f64 = (assign15520_e22925 / p.p734);
            let assign15520_e22929: f64 = (assign15520_e22927).max(1e-38);
            let assign15520_e22930: f64 = (assign15520_e22929).ln();
            let assign15520_e22931: f64 = (locals.var_nvtmd * assign15520_e22930);
            let assign15520_e22932: f64 = (assign15520_e22921 - assign15520_e22931);
            (locals.var_vjdmrev, locals.var_vjdmrev_dn0, locals.var_vjdmrev_dn2, locals.var_vjdmrev_dn3, locals.var_vjdmrev_dn4, locals.var_vjdmrev_dn5, locals.var_vjdmrev_dn6, locals.var_vjdmrev_dn7, locals.var_vjdmrev_dn8, locals.var_vjdmrev_dn9, locals.var_vjdmrev_dn10, locals.var_vjdmrev_dn11, locals.var_vjdmrev_dn12, locals.var_vjdmrev_dn13, locals.var_vjdmrev_dn14, ) = (assign15520_e22932, (-(locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn0 / p.p734) } else { 0.0 } / assign15520_e22929))), (-(locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn2 / p.p734) } else { 0.0 } / assign15520_e22929))), (-(locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn3 / p.p734) } else { 0.0 } / assign15520_e22929))), (-((locals.var_nvtmd_dn4 * assign15520_e22930) + (locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn4 / p.p734) } else { 0.0 } / assign15520_e22929)))), (-(locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn5 / p.p734) } else { 0.0 } / assign15520_e22929))), (-(locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn6 / p.p734) } else { 0.0 } / assign15520_e22929))), (-(locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn7 / p.p734) } else { 0.0 } / assign15520_e22929))), (-(locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn8 / p.p734) } else { 0.0 } / assign15520_e22929))), (-(locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn9 / p.p734) } else { 0.0 } / assign15520_e22929))), (-(locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn10 / p.p734) } else { 0.0 } / assign15520_e22929))), (-(locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn11 / p.p734) } else { 0.0 } / assign15520_e22929))), (-(locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn12 / p.p734) } else { 0.0 } / assign15520_e22929))), (-(locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn13 / p.p734) } else { 0.0 } / assign15520_e22929))), (-(locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn14 / p.p734) } else { 0.0 } / assign15520_e22929))), );
            locals.var_vjdmrev_rv = 0.0;
        }

        if (locals.var_guard487 != 0.0) {
            let assign15530_e22939: f64 = (p.p732 + locals.var_vjdmrev);
            let assign15530_e22940: f64 = (-assign15530_e22939);
            let assign15530_e22942: f64 = (assign15530_e22940 / locals.var_nvtmd);
            let assign15530_e22943: f64 = { let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
            let assign15530_e22944: f64 = (p.p734 * assign15530_e22943);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, ) = (assign15530_e22944, (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn0) / locals.var_nvtmd))), (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn2) / locals.var_nvtmd))), (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn3) / locals.var_nvtmd))), (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((-locals.var_vjdmrev_dn4) * locals.var_nvtmd) - (assign15530_e22940 * locals.var_nvtmd_dn4)) / (locals.var_nvtmd * locals.var_nvtmd)))), (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn5) / locals.var_nvtmd))), (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn6) / locals.var_nvtmd))), (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn7) / locals.var_nvtmd))), (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn8) / locals.var_nvtmd))), (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn9) / locals.var_nvtmd))), (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn10) / locals.var_nvtmd))), (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn11) / locals.var_nvtmd))), (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn12) / locals.var_nvtmd))), (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn13) / locals.var_nvtmd))), (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn14) / locals.var_nvtmd))), );
            locals.var_t1_rv = 0.0;
        }

        if (locals.var_guard487 != 0.0) {
            let assign15540_e22951: f64 = (1.0 + locals.var_t1);
            let assign15540_e22952: f64 = (locals.var_isbd * assign15540_e22951);
            (locals.var_ivjdmrev, locals.var_ivjdmrev_dn0, locals.var_ivjdmrev_dn2, locals.var_ivjdmrev_dn3, locals.var_ivjdmrev_dn4, locals.var_ivjdmrev_dn5, locals.var_ivjdmrev_dn6, locals.var_ivjdmrev_dn7, locals.var_ivjdmrev_dn8, locals.var_ivjdmrev_dn9, locals.var_ivjdmrev_dn10, locals.var_ivjdmrev_dn11, locals.var_ivjdmrev_dn12, locals.var_ivjdmrev_dn13, locals.var_ivjdmrev_dn14, ) = (assign15540_e22952, ((locals.var_isbd_dn0 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn0)), ((locals.var_isbd_dn2 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn2)), ((locals.var_isbd_dn3 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn3)), ((locals.var_isbd_dn4 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn4)), ((locals.var_isbd_dn5 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn5)), ((locals.var_isbd_dn6 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn6)), ((locals.var_isbd_dn7 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn7)), ((locals.var_isbd_dn8 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn8)), ((locals.var_isbd_dn9 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn9)), ((locals.var_isbd_dn10 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn10)), ((locals.var_isbd_dn11 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn11)), ((locals.var_isbd_dn12 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn12)), ((locals.var_isbd_dn13 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn13)), ((locals.var_isbd_dn14 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn14)), );
            locals.var_ivjdmrev_rv = 0.0;
        }

        if (locals.var_guard487 != 0.0) {
            let assign15550_e22957: f64 = (-locals.var_isbd);
            let assign15550_e22959: f64 = (assign15550_e22957 * locals.var_t1);
            let assign15550_e22961: f64 = (assign15550_e22959 / locals.var_nvtmd);
            (locals.var_dslprev, locals.var_dslprev_dn0, locals.var_dslprev_dn2, locals.var_dslprev_dn3, locals.var_dslprev_dn4, locals.var_dslprev_dn5, locals.var_dslprev_dn6, locals.var_dslprev_dn7, locals.var_dslprev_dn8, locals.var_dslprev_dn9, locals.var_dslprev_dn10, locals.var_dslprev_dn11, locals.var_dslprev_dn12, locals.var_dslprev_dn13, locals.var_dslprev_dn14, ) = (assign15550_e22961, ((((-locals.var_isbd_dn0) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn0)) / locals.var_nvtmd), ((((-locals.var_isbd_dn2) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn2)) / locals.var_nvtmd), ((((-locals.var_isbd_dn3) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn3)) / locals.var_nvtmd), ((((((-locals.var_isbd_dn4) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn4)) * locals.var_nvtmd) - (assign15550_e22959 * locals.var_nvtmd_dn4)) / (locals.var_nvtmd * locals.var_nvtmd)), ((((-locals.var_isbd_dn5) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn5)) / locals.var_nvtmd), ((((-locals.var_isbd_dn6) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn6)) / locals.var_nvtmd), ((((-locals.var_isbd_dn7) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn7)) / locals.var_nvtmd), ((((-locals.var_isbd_dn8) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn8)) / locals.var_nvtmd), ((((-locals.var_isbd_dn9) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn9)) / locals.var_nvtmd), ((((-locals.var_isbd_dn10) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn10)) / locals.var_nvtmd), ((((-locals.var_isbd_dn11) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn11)) / locals.var_nvtmd), ((((-locals.var_isbd_dn12) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn12)) / locals.var_nvtmd), ((((-locals.var_isbd_dn13) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn13)) / locals.var_nvtmd), ((((-locals.var_isbd_dn14) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn14)) / locals.var_nvtmd), );
            locals.var_dslprev_rv = 0.0;
        }

        if (locals.var_guard487 == 0.0) {
            (locals.var_nvtmd, locals.var_nvtmd_dn4, ) = (0.0, 0.0, );
            locals.var_nvtmd_rv = 0.0;
            (locals.var_xexpbvd, locals.var_xexpbvd_dn4, ) = (0.0, 0.0, );
            locals.var_xexpbvd_rv = 0.0;
            (locals.var_vjdmfwd, locals.var_vjdmfwd_dn0, locals.var_vjdmfwd_dn2, locals.var_vjdmfwd_dn3, locals.var_vjdmfwd_dn4, locals.var_vjdmfwd_dn5, locals.var_vjdmfwd_dn6, locals.var_vjdmfwd_dn7, locals.var_vjdmfwd_dn8, locals.var_vjdmfwd_dn9, locals.var_vjdmfwd_dn10, locals.var_vjdmfwd_dn11, locals.var_vjdmfwd_dn12, locals.var_vjdmfwd_dn13, locals.var_vjdmfwd_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_vjdmfwd_rv = 0.0;
            (locals.var_ivjdmfwd, locals.var_ivjdmfwd_dn0, locals.var_ivjdmfwd_dn2, locals.var_ivjdmfwd_dn3, locals.var_ivjdmfwd_dn4, locals.var_ivjdmfwd_dn5, locals.var_ivjdmfwd_dn6, locals.var_ivjdmfwd_dn7, locals.var_ivjdmfwd_dn8, locals.var_ivjdmfwd_dn9, locals.var_ivjdmfwd_dn10, locals.var_ivjdmfwd_dn11, locals.var_ivjdmfwd_dn12, locals.var_ivjdmfwd_dn13, locals.var_ivjdmfwd_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_ivjdmfwd_rv = 0.0;
            (locals.var_dslpfwd, locals.var_dslpfwd_dn0, locals.var_dslpfwd_dn2, locals.var_dslpfwd_dn3, locals.var_dslpfwd_dn4, locals.var_dslpfwd_dn5, locals.var_dslpfwd_dn6, locals.var_dslpfwd_dn7, locals.var_dslpfwd_dn8, locals.var_dslpfwd_dn9, locals.var_dslpfwd_dn10, locals.var_dslpfwd_dn11, locals.var_dslpfwd_dn12, locals.var_dslpfwd_dn13, locals.var_dslpfwd_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_dslpfwd_rv = 0.0;
            (locals.var_vjdmrev, locals.var_vjdmrev_dn0, locals.var_vjdmrev_dn2, locals.var_vjdmrev_dn3, locals.var_vjdmrev_dn4, locals.var_vjdmrev_dn5, locals.var_vjdmrev_dn6, locals.var_vjdmrev_dn7, locals.var_vjdmrev_dn8, locals.var_vjdmrev_dn9, locals.var_vjdmrev_dn10, locals.var_vjdmrev_dn11, locals.var_vjdmrev_dn12, locals.var_vjdmrev_dn13, locals.var_vjdmrev_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_vjdmrev_rv = 0.0;
            (locals.var_ivjdmrev, locals.var_ivjdmrev_dn0, locals.var_ivjdmrev_dn2, locals.var_ivjdmrev_dn3, locals.var_ivjdmrev_dn4, locals.var_ivjdmrev_dn5, locals.var_ivjdmrev_dn6, locals.var_ivjdmrev_dn7, locals.var_ivjdmrev_dn8, locals.var_ivjdmrev_dn9, locals.var_ivjdmrev_dn10, locals.var_ivjdmrev_dn11, locals.var_ivjdmrev_dn12, locals.var_ivjdmrev_dn13, locals.var_ivjdmrev_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_ivjdmrev_rv = 0.0;
            (locals.var_dslprev, locals.var_dslprev_dn0, locals.var_dslprev_dn2, locals.var_dslprev_dn3, locals.var_dslprev_dn4, locals.var_dslprev_dn5, locals.var_dslprev_dn6, locals.var_dslprev_dn7, locals.var_dslprev_dn8, locals.var_dslprev_dn9, locals.var_dslprev_dn10, locals.var_dslprev_dn11, locals.var_dslprev_dn12, locals.var_dslprev_dn13, locals.var_dslprev_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_dslprev_rv = 0.0;
        }

        let assign15640_e23022: f64 = if (((p.p17 > 0.0) && (p.p18 > 0.0)) && ((p.p2 == 1.0) || ((p.p2 > 1.0) && (p.p19 > 0.0)))) { 1.0 } else { 0.0 };
        locals.var_guard488 = assign15640_e23022;
        locals.var_guard488_rv = 0.0;

        if (locals.var_guard488 != 0.0) {
            let assign15650_e23026: f64 = (locals.var_lnew).powf(p.p921);
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign15650_e23026, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t0_rv = 0.0;
        }

        if (locals.var_guard488 != 0.0) {
            let assign15660_e23032: f64 = (locals.var_wnew + p.p914);
            locals.var_w_tmp_stress = assign15660_e23032;
            locals.var_w_tmp_stress_rv = 0.0;
        }

        if (locals.var_guard488 != 0.0) {
            let assign15670_e23038: f64 = (locals.var_w_tmp_stress).powf(p.p922);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, ) = (assign15670_e23038, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1_rv = 0.0;
        }

        if (locals.var_guard488 != 0.0) {
            let assign15680_e23044: f64 = (p.p918 / locals.var_t0);
            let assign15680_e23047: f64 = (p.p919 / locals.var_t1);
            let assign15680_e23048: f64 = (assign15680_e23044 + assign15680_e23047);
            let assign15680_e23052: f64 = (locals.var_t0 * locals.var_t1);
            let assign15680_e23053: f64 = (p.p920 / assign15680_e23052);
            let assign15680_e23054: f64 = (assign15680_e23048 + assign15680_e23053);
            (locals.var_tmp1_stress, locals.var_tmp1_stress_dn0, locals.var_tmp1_stress_dn2, locals.var_tmp1_stress_dn3, locals.var_tmp1_stress_dn4, locals.var_tmp1_stress_dn5, locals.var_tmp1_stress_dn6, locals.var_tmp1_stress_dn7, locals.var_tmp1_stress_dn8, locals.var_tmp1_stress_dn9, locals.var_tmp1_stress_dn10, locals.var_tmp1_stress_dn11, locals.var_tmp1_stress_dn12, locals.var_tmp1_stress_dn13, locals.var_tmp1_stress_dn14, ) = (assign15680_e23054, (((-((p.p918 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn0 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn0))) / (assign15680_e23052 * assign15680_e23052)))), (((-((p.p918 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn2 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn2))) / (assign15680_e23052 * assign15680_e23052)))), (((-((p.p918 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn3) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn3 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn3))) / (assign15680_e23052 * assign15680_e23052)))), (((-((p.p918 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn4 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn4))) / (assign15680_e23052 * assign15680_e23052)))), (((-((p.p918 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn5 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn5))) / (assign15680_e23052 * assign15680_e23052)))), (((-((p.p918 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn6 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn6))) / (assign15680_e23052 * assign15680_e23052)))), (((-((p.p918 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn7 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn7))) / (assign15680_e23052 * assign15680_e23052)))), (((-((p.p918 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn8 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn8))) / (assign15680_e23052 * assign15680_e23052)))), (((-((p.p918 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn9 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn9))) / (assign15680_e23052 * assign15680_e23052)))), (((-((p.p918 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn10 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn10))) / (assign15680_e23052 * assign15680_e23052)))), (((-((p.p918 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn11 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn11))) / (assign15680_e23052 * assign15680_e23052)))), (((-((p.p918 * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn12 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn12))) / (assign15680_e23052 * assign15680_e23052)))), (((-((p.p918 * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn13) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn13 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn13))) / (assign15680_e23052 * assign15680_e23052)))), (((-((p.p918 * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn14) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn14 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn14))) / (assign15680_e23052 * assign15680_e23052)))), );
            locals.var_tmp1_stress_rv = 0.0;
        }

        if (locals.var_guard488 != 0.0) {
            let assign15690_e23060: f64 = (1.0 + locals.var_tmp1_stress);
            (locals.var_kstress_u0, locals.var_kstress_u0_dn0, locals.var_kstress_u0_dn2, locals.var_kstress_u0_dn3, locals.var_kstress_u0_dn4, locals.var_kstress_u0_dn5, locals.var_kstress_u0_dn6, locals.var_kstress_u0_dn7, locals.var_kstress_u0_dn8, locals.var_kstress_u0_dn9, locals.var_kstress_u0_dn10, locals.var_kstress_u0_dn11, locals.var_kstress_u0_dn12, locals.var_kstress_u0_dn13, locals.var_kstress_u0_dn14, ) = (assign15690_e23060, locals.var_tmp1_stress_dn0, locals.var_tmp1_stress_dn2, locals.var_tmp1_stress_dn3, locals.var_tmp1_stress_dn4, locals.var_tmp1_stress_dn5, locals.var_tmp1_stress_dn6, locals.var_tmp1_stress_dn7, locals.var_tmp1_stress_dn8, locals.var_tmp1_stress_dn9, locals.var_tmp1_stress_dn10, locals.var_tmp1_stress_dn11, locals.var_tmp1_stress_dn12, locals.var_tmp1_stress_dn13, locals.var_tmp1_stress_dn14, );
            locals.var_kstress_u0_rv = 0.0;
        }

        if (locals.var_guard488 != 0.0) {
            let assign15700_e23066: f64 = (locals.var_lnew).powf(p.p927);
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign15700_e23066, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t0_rv = 0.0;
        }

        if (locals.var_guard488 != 0.0) {
            let assign15710_e23072: f64 = (locals.var_w_tmp_stress).powf(p.p928);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, ) = (assign15710_e23072, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1_rv = 0.0;
        }

        if (locals.var_guard488 != 0.0) {
            let assign15720_e23078: f64 = (p.p924 / locals.var_t0);
            let assign15720_e23081: f64 = (p.p925 / locals.var_t1);
            let assign15720_e23082: f64 = (assign15720_e23078 + assign15720_e23081);
            let assign15720_e23086: f64 = (locals.var_t0 * locals.var_t1);
            let assign15720_e23087: f64 = (p.p926 / assign15720_e23086);
            let assign15720_e23088: f64 = (assign15720_e23082 + assign15720_e23087);
            (locals.var_tmp1_stress_vth, locals.var_tmp1_stress_vth_dn0, locals.var_tmp1_stress_vth_dn2, locals.var_tmp1_stress_vth_dn3, locals.var_tmp1_stress_vth_dn4, locals.var_tmp1_stress_vth_dn5, locals.var_tmp1_stress_vth_dn6, locals.var_tmp1_stress_vth_dn7, locals.var_tmp1_stress_vth_dn8, locals.var_tmp1_stress_vth_dn9, locals.var_tmp1_stress_vth_dn10, locals.var_tmp1_stress_vth_dn11, locals.var_tmp1_stress_vth_dn12, locals.var_tmp1_stress_vth_dn13, locals.var_tmp1_stress_vth_dn14, ) = (assign15720_e23088, (((-((p.p924 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn0 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn0))) / (assign15720_e23086 * assign15720_e23086)))), (((-((p.p924 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn2 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn2))) / (assign15720_e23086 * assign15720_e23086)))), (((-((p.p924 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn3) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn3 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn3))) / (assign15720_e23086 * assign15720_e23086)))), (((-((p.p924 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn4 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn4))) / (assign15720_e23086 * assign15720_e23086)))), (((-((p.p924 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn5 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn5))) / (assign15720_e23086 * assign15720_e23086)))), (((-((p.p924 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn6 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn6))) / (assign15720_e23086 * assign15720_e23086)))), (((-((p.p924 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn7 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn7))) / (assign15720_e23086 * assign15720_e23086)))), (((-((p.p924 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn8 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn8))) / (assign15720_e23086 * assign15720_e23086)))), (((-((p.p924 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn9 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn9))) / (assign15720_e23086 * assign15720_e23086)))), (((-((p.p924 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn10 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn10))) / (assign15720_e23086 * assign15720_e23086)))), (((-((p.p924 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn11 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn11))) / (assign15720_e23086 * assign15720_e23086)))), (((-((p.p924 * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn12 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn12))) / (assign15720_e23086 * assign15720_e23086)))), (((-((p.p924 * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn13) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn13 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn13))) / (assign15720_e23086 * assign15720_e23086)))), (((-((p.p924 * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn14) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn14 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn14))) / (assign15720_e23086 * assign15720_e23086)))), );
            locals.var_tmp1_stress_vth_rv = 0.0;
        }

        if (locals.var_guard488 != 0.0) {
            let assign15730_e23094: f64 = (1.0 + locals.var_tmp1_stress_vth);
            (locals.var_kstress_vth0, locals.var_kstress_vth0_dn0, locals.var_kstress_vth0_dn2, locals.var_kstress_vth0_dn3, locals.var_kstress_vth0_dn4, locals.var_kstress_vth0_dn5, locals.var_kstress_vth0_dn6, locals.var_kstress_vth0_dn7, locals.var_kstress_vth0_dn8, locals.var_kstress_vth0_dn9, locals.var_kstress_vth0_dn10, locals.var_kstress_vth0_dn11, locals.var_kstress_vth0_dn12, locals.var_kstress_vth0_dn13, locals.var_kstress_vth0_dn14, ) = (assign15730_e23094, locals.var_tmp1_stress_vth_dn0, locals.var_tmp1_stress_vth_dn2, locals.var_tmp1_stress_vth_dn3, locals.var_tmp1_stress_vth_dn4, locals.var_tmp1_stress_vth_dn5, locals.var_tmp1_stress_vth_dn6, locals.var_tmp1_stress_vth_dn7, locals.var_tmp1_stress_vth_dn8, locals.var_tmp1_stress_vth_dn9, locals.var_tmp1_stress_vth_dn10, locals.var_tmp1_stress_vth_dn11, locals.var_tmp1_stress_vth_dn12, locals.var_tmp1_stress_vth_dn13, locals.var_tmp1_stress_vth_dn14, );
            locals.var_kstress_vth0_rv = 0.0;
        }

        if (locals.var_guard488 != 0.0) {
            let assign15740_e23100: f64 = (locals.var_tratio - 1.0);
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign15740_e23100, 0.0, 0.0, 0.0, locals.var_tratio_dn4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t0_rv = 0.0;
        }

        if (locals.var_guard488 != 0.0) {
            let assign15750_e23108: f64 = (p.p917 * locals.var_t0);
            let assign15750_e23109: f64 = (1.0 + assign15750_e23108);
            let assign15750_e23110: f64 = (locals.var_kstress_u0 * assign15750_e23109);
            let assign15750_e23112: f64 = (assign15750_e23110 + 1e-9);
            (locals.var_ku0_temp, locals.var_ku0_temp_dn0, locals.var_ku0_temp_dn2, locals.var_ku0_temp_dn3, locals.var_ku0_temp_dn4, locals.var_ku0_temp_dn5, locals.var_ku0_temp_dn6, locals.var_ku0_temp_dn7, locals.var_ku0_temp_dn8, locals.var_ku0_temp_dn9, locals.var_ku0_temp_dn10, locals.var_ku0_temp_dn11, locals.var_ku0_temp_dn12, locals.var_ku0_temp_dn13, locals.var_ku0_temp_dn14, ) = (assign15750_e23112, ((locals.var_kstress_u0_dn0 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn0))), ((locals.var_kstress_u0_dn2 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn2))), ((locals.var_kstress_u0_dn3 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn3))), ((locals.var_kstress_u0_dn4 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn4))), ((locals.var_kstress_u0_dn5 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn5))), ((locals.var_kstress_u0_dn6 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn6))), ((locals.var_kstress_u0_dn7 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn7))), ((locals.var_kstress_u0_dn8 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn8))), ((locals.var_kstress_u0_dn9 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn9))), ((locals.var_kstress_u0_dn10 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn10))), ((locals.var_kstress_u0_dn11 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn11))), ((locals.var_kstress_u0_dn12 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn12))), ((locals.var_kstress_u0_dn13 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn13))), ((locals.var_kstress_u0_dn14 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn14))), );
            locals.var_ku0_temp_rv = 0.0;
        }

        if (locals.var_guard488 != 0.0) {
            locals.var_i = 0.0;
            locals.var_i_rv = 0.0;
        }

        let mut assign15770_loop_guard: usize = 0;
        while {
            let assign15770_cond_e23123: f64 = if ((locals.var_guard488 != 0.0) && (locals.var_i < p.p2)) { 1.0 } else { 0.0 };
            assign15770_cond_e23123 != 0.0
        } {
            assign15770_loop_guard += 1;
            assert!(assign15770_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (locals.var_guard488 != 0.0) {
                let assign15770_body0_e23127: f64 = (1.0 / p.p2);
                let assign15770_body0_e23131: f64 = (0.5 * locals.var_l_mult);
                let assign15770_body0_e23132: f64 = (p.p17 + assign15770_body0_e23131);
                let assign15770_body0_e23136: f64 = (p.p19 + locals.var_l_mult);
                let assign15770_body0_e23137: f64 = (locals.var_i * assign15770_body0_e23136);
                let assign15770_body0_e23138: f64 = (assign15770_body0_e23132 + assign15770_body0_e23137);
                let assign15770_body0_e23139: f64 = (assign15770_body0_e23127 / assign15770_body0_e23138);
                (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign15770_body0_e23139, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
                locals.var_t0_rv = 0.0;
            }
            if (locals.var_guard488 != 0.0) {
                let assign15770_body1_e23145: f64 = (1.0 / p.p2);
                let assign15770_body1_e23149: f64 = (0.5 * locals.var_l_mult);
                let assign15770_body1_e23150: f64 = (p.p18 + assign15770_body1_e23149);
                let assign15770_body1_e23154: f64 = (p.p19 + locals.var_l_mult);
                let assign15770_body1_e23155: f64 = (locals.var_i * assign15770_body1_e23154);
                let assign15770_body1_e23156: f64 = (assign15770_body1_e23150 + assign15770_body1_e23155);
                let assign15770_body1_e23157: f64 = (assign15770_body1_e23145 / assign15770_body1_e23156);
                (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, ) = (assign15770_body1_e23157, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
                locals.var_t1_rv = 0.0;
            }
            if (locals.var_guard488 != 0.0) {
                let assign15770_body2_e23163: f64 = (locals.var_inv_sa + locals.var_t0);
                (locals.var_inv_sa, locals.var_inv_sa_dn0, locals.var_inv_sa_dn2, locals.var_inv_sa_dn3, locals.var_inv_sa_dn4, locals.var_inv_sa_dn5, locals.var_inv_sa_dn6, locals.var_inv_sa_dn7, locals.var_inv_sa_dn8, locals.var_inv_sa_dn9, locals.var_inv_sa_dn10, locals.var_inv_sa_dn11, locals.var_inv_sa_dn12, locals.var_inv_sa_dn13, locals.var_inv_sa_dn14, ) = (assign15770_body2_e23163, (locals.var_inv_sa_dn0 + locals.var_t0_dn0), (locals.var_inv_sa_dn2 + locals.var_t0_dn2), (locals.var_inv_sa_dn3 + locals.var_t0_dn3), (locals.var_inv_sa_dn4 + locals.var_t0_dn4), (locals.var_inv_sa_dn5 + locals.var_t0_dn5), (locals.var_inv_sa_dn6 + locals.var_t0_dn6), (locals.var_inv_sa_dn7 + locals.var_t0_dn7), (locals.var_inv_sa_dn8 + locals.var_t0_dn8), (locals.var_inv_sa_dn9 + locals.var_t0_dn9), (locals.var_inv_sa_dn10 + locals.var_t0_dn10), (locals.var_inv_sa_dn11 + locals.var_t0_dn11), (locals.var_inv_sa_dn12 + locals.var_t0_dn12), (locals.var_inv_sa_dn13 + locals.var_t0_dn13), (locals.var_inv_sa_dn14 + locals.var_t0_dn14), );
                locals.var_inv_sa_rv = 0.0;
            }
            if (locals.var_guard488 != 0.0) {
                let assign15770_body3_e23169: f64 = (locals.var_inv_sb + locals.var_t1);
                (locals.var_inv_sb, locals.var_inv_sb_dn0, locals.var_inv_sb_dn2, locals.var_inv_sb_dn3, locals.var_inv_sb_dn4, locals.var_inv_sb_dn5, locals.var_inv_sb_dn6, locals.var_inv_sb_dn7, locals.var_inv_sb_dn8, locals.var_inv_sb_dn9, locals.var_inv_sb_dn10, locals.var_inv_sb_dn11, locals.var_inv_sb_dn12, locals.var_inv_sb_dn13, locals.var_inv_sb_dn14, ) = (assign15770_body3_e23169, (locals.var_inv_sb_dn0 + locals.var_t1_dn0), (locals.var_inv_sb_dn2 + locals.var_t1_dn2), (locals.var_inv_sb_dn3 + locals.var_t1_dn3), (locals.var_inv_sb_dn4 + locals.var_t1_dn4), (locals.var_inv_sb_dn5 + locals.var_t1_dn5), (locals.var_inv_sb_dn6 + locals.var_t1_dn6), (locals.var_inv_sb_dn7 + locals.var_t1_dn7), (locals.var_inv_sb_dn8 + locals.var_t1_dn8), (locals.var_inv_sb_dn9 + locals.var_t1_dn9), (locals.var_inv_sb_dn10 + locals.var_t1_dn10), (locals.var_inv_sb_dn11 + locals.var_t1_dn11), (locals.var_inv_sb_dn12 + locals.var_t1_dn12), (locals.var_inv_sb_dn13 + locals.var_t1_dn13), (locals.var_inv_sb_dn14 + locals.var_t1_dn14), );
                locals.var_inv_sb_rv = 0.0;
            }
            if (locals.var_guard488 != 0.0) {
                let assign15770_body4_e23175: f64 = (locals.var_i + 1.0);
                locals.var_i = assign15770_body4_e23175;
                locals.var_i_rv = 0.0;
            }
        }

        if (locals.var_guard488 != 0.0) {
            let assign15780_e23183: f64 = (0.5 * locals.var_l_mult);
            let assign15780_e23184: f64 = (p.p912 + assign15780_e23183);
            let assign15780_e23185: f64 = (1.0 / assign15780_e23184);
            locals.var_inv_saref = assign15780_e23185;
            locals.var_inv_saref_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_20(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        if (locals.var_guard488 != 0.0) {
            let assign15790_e23193: f64 = (0.5 * locals.var_l_mult);
            let assign15790_e23194: f64 = (p.p913 + assign15790_e23193);
            let assign15790_e23195: f64 = (1.0 / assign15790_e23194);
            locals.var_inv_sbref = assign15790_e23195;
            locals.var_inv_sbref_rv = 0.0;
        }

        if (locals.var_guard488 != 0.0) {
            let assign15800_e23201: f64 = (locals.var_inv_saref + locals.var_inv_sbref);
            locals.var_inv_odref = assign15800_e23201;
            locals.var_inv_odref_rv = 0.0;
        }

        if (locals.var_guard488 != 0.0) {
            let assign15810_e23207: f64 = (p.p915 / locals.var_ku0_temp);
            let assign15810_e23209: f64 = (assign15810_e23207 * locals.var_inv_odref);
            (locals.var_rho_ref, locals.var_rho_ref_dn0, locals.var_rho_ref_dn2, locals.var_rho_ref_dn3, locals.var_rho_ref_dn4, locals.var_rho_ref_dn5, locals.var_rho_ref_dn6, locals.var_rho_ref_dn7, locals.var_rho_ref_dn8, locals.var_rho_ref_dn9, locals.var_rho_ref_dn10, locals.var_rho_ref_dn11, locals.var_rho_ref_dn12, locals.var_rho_ref_dn13, locals.var_rho_ref_dn14, ) = (assign15810_e23209, ((-((p.p915 * locals.var_ku0_temp_dn0) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p915 * locals.var_ku0_temp_dn2) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p915 * locals.var_ku0_temp_dn3) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p915 * locals.var_ku0_temp_dn4) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p915 * locals.var_ku0_temp_dn5) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p915 * locals.var_ku0_temp_dn6) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p915 * locals.var_ku0_temp_dn7) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p915 * locals.var_ku0_temp_dn8) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p915 * locals.var_ku0_temp_dn9) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p915 * locals.var_ku0_temp_dn10) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p915 * locals.var_ku0_temp_dn11) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p915 * locals.var_ku0_temp_dn12) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p915 * locals.var_ku0_temp_dn13) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p915 * locals.var_ku0_temp_dn14) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), );
            locals.var_rho_ref_rv = 0.0;
        }

        if (locals.var_guard488 != 0.0) {
            let assign15820_e23215: f64 = (locals.var_inv_sa + locals.var_inv_sb);
            (locals.var_inv_od, locals.var_inv_od_dn0, locals.var_inv_od_dn2, locals.var_inv_od_dn3, locals.var_inv_od_dn4, locals.var_inv_od_dn5, locals.var_inv_od_dn6, locals.var_inv_od_dn7, locals.var_inv_od_dn8, locals.var_inv_od_dn9, locals.var_inv_od_dn10, locals.var_inv_od_dn11, locals.var_inv_od_dn12, locals.var_inv_od_dn13, locals.var_inv_od_dn14, ) = (assign15820_e23215, (locals.var_inv_sa_dn0 + locals.var_inv_sb_dn0), (locals.var_inv_sa_dn2 + locals.var_inv_sb_dn2), (locals.var_inv_sa_dn3 + locals.var_inv_sb_dn3), (locals.var_inv_sa_dn4 + locals.var_inv_sb_dn4), (locals.var_inv_sa_dn5 + locals.var_inv_sb_dn5), (locals.var_inv_sa_dn6 + locals.var_inv_sb_dn6), (locals.var_inv_sa_dn7 + locals.var_inv_sb_dn7), (locals.var_inv_sa_dn8 + locals.var_inv_sb_dn8), (locals.var_inv_sa_dn9 + locals.var_inv_sb_dn9), (locals.var_inv_sa_dn10 + locals.var_inv_sb_dn10), (locals.var_inv_sa_dn11 + locals.var_inv_sb_dn11), (locals.var_inv_sa_dn12 + locals.var_inv_sb_dn12), (locals.var_inv_sa_dn13 + locals.var_inv_sb_dn13), (locals.var_inv_sa_dn14 + locals.var_inv_sb_dn14), );
            locals.var_inv_od_rv = 0.0;
        }

        if (locals.var_guard488 != 0.0) {
            let assign15830_e23221: f64 = (p.p915 / locals.var_ku0_temp);
            let assign15830_e23223: f64 = (assign15830_e23221 * locals.var_inv_od);
            (locals.var_rho, locals.var_rho_dn0, locals.var_rho_dn2, locals.var_rho_dn3, locals.var_rho_dn4, locals.var_rho_dn5, locals.var_rho_dn6, locals.var_rho_dn7, locals.var_rho_dn8, locals.var_rho_dn9, locals.var_rho_dn10, locals.var_rho_dn11, locals.var_rho_dn12, locals.var_rho_dn13, locals.var_rho_dn14, ) = (assign15830_e23223, (((-((p.p915 * locals.var_ku0_temp_dn0) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15830_e23221 * locals.var_inv_od_dn0)), (((-((p.p915 * locals.var_ku0_temp_dn2) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15830_e23221 * locals.var_inv_od_dn2)), (((-((p.p915 * locals.var_ku0_temp_dn3) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15830_e23221 * locals.var_inv_od_dn3)), (((-((p.p915 * locals.var_ku0_temp_dn4) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15830_e23221 * locals.var_inv_od_dn4)), (((-((p.p915 * locals.var_ku0_temp_dn5) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15830_e23221 * locals.var_inv_od_dn5)), (((-((p.p915 * locals.var_ku0_temp_dn6) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15830_e23221 * locals.var_inv_od_dn6)), (((-((p.p915 * locals.var_ku0_temp_dn7) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15830_e23221 * locals.var_inv_od_dn7)), (((-((p.p915 * locals.var_ku0_temp_dn8) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15830_e23221 * locals.var_inv_od_dn8)), (((-((p.p915 * locals.var_ku0_temp_dn9) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15830_e23221 * locals.var_inv_od_dn9)), (((-((p.p915 * locals.var_ku0_temp_dn10) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15830_e23221 * locals.var_inv_od_dn10)), (((-((p.p915 * locals.var_ku0_temp_dn11) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15830_e23221 * locals.var_inv_od_dn11)), (((-((p.p915 * locals.var_ku0_temp_dn12) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15830_e23221 * locals.var_inv_od_dn12)), (((-((p.p915 * locals.var_ku0_temp_dn13) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15830_e23221 * locals.var_inv_od_dn13)), (((-((p.p915 * locals.var_ku0_temp_dn14) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15830_e23221 * locals.var_inv_od_dn14)), );
            locals.var_rho_rv = 0.0;
        }

        if (locals.var_guard488 != 0.0) {
            let assign15840_e23229: f64 = (1.0 + locals.var_rho);
            let assign15840_e23232: f64 = (1.0 + locals.var_rho_ref);
            let assign15840_e23233: f64 = (assign15840_e23229 / assign15840_e23232);
            (locals.var_mu0_mult, locals.var_mu0_mult_dn0, locals.var_mu0_mult_dn2, locals.var_mu0_mult_dn3, locals.var_mu0_mult_dn4, locals.var_mu0_mult_dn5, locals.var_mu0_mult_dn6, locals.var_mu0_mult_dn7, locals.var_mu0_mult_dn8, locals.var_mu0_mult_dn9, locals.var_mu0_mult_dn10, locals.var_mu0_mult_dn11, locals.var_mu0_mult_dn12, locals.var_mu0_mult_dn13, locals.var_mu0_mult_dn14, ) = (assign15840_e23233, (((locals.var_rho_dn0 * assign15840_e23232) - (assign15840_e23229 * locals.var_rho_ref_dn0)) / (assign15840_e23232 * assign15840_e23232)), (((locals.var_rho_dn2 * assign15840_e23232) - (assign15840_e23229 * locals.var_rho_ref_dn2)) / (assign15840_e23232 * assign15840_e23232)), (((locals.var_rho_dn3 * assign15840_e23232) - (assign15840_e23229 * locals.var_rho_ref_dn3)) / (assign15840_e23232 * assign15840_e23232)), (((locals.var_rho_dn4 * assign15840_e23232) - (assign15840_e23229 * locals.var_rho_ref_dn4)) / (assign15840_e23232 * assign15840_e23232)), (((locals.var_rho_dn5 * assign15840_e23232) - (assign15840_e23229 * locals.var_rho_ref_dn5)) / (assign15840_e23232 * assign15840_e23232)), (((locals.var_rho_dn6 * assign15840_e23232) - (assign15840_e23229 * locals.var_rho_ref_dn6)) / (assign15840_e23232 * assign15840_e23232)), (((locals.var_rho_dn7 * assign15840_e23232) - (assign15840_e23229 * locals.var_rho_ref_dn7)) / (assign15840_e23232 * assign15840_e23232)), (((locals.var_rho_dn8 * assign15840_e23232) - (assign15840_e23229 * locals.var_rho_ref_dn8)) / (assign15840_e23232 * assign15840_e23232)), (((locals.var_rho_dn9 * assign15840_e23232) - (assign15840_e23229 * locals.var_rho_ref_dn9)) / (assign15840_e23232 * assign15840_e23232)), (((locals.var_rho_dn10 * assign15840_e23232) - (assign15840_e23229 * locals.var_rho_ref_dn10)) / (assign15840_e23232 * assign15840_e23232)), (((locals.var_rho_dn11 * assign15840_e23232) - (assign15840_e23229 * locals.var_rho_ref_dn11)) / (assign15840_e23232 * assign15840_e23232)), (((locals.var_rho_dn12 * assign15840_e23232) - (assign15840_e23229 * locals.var_rho_ref_dn12)) / (assign15840_e23232 * assign15840_e23232)), (((locals.var_rho_dn13 * assign15840_e23232) - (assign15840_e23229 * locals.var_rho_ref_dn13)) / (assign15840_e23232 * assign15840_e23232)), (((locals.var_rho_dn14 * assign15840_e23232) - (assign15840_e23229 * locals.var_rho_ref_dn14)) / (assign15840_e23232 * assign15840_e23232)), );
            locals.var_mu0_mult_rv = 0.0;
        }

        if (locals.var_guard488 != 0.0) {
            let assign15850_e23240: f64 = (locals.var_rho * p.p916);
            let assign15850_e23241: f64 = (1.0 + assign15850_e23240);
            let assign15850_e23245: f64 = (locals.var_rho_ref * p.p916);
            let assign15850_e23246: f64 = (1.0 + assign15850_e23245);
            let assign15850_e23247: f64 = (assign15850_e23241 / assign15850_e23246);
            (locals.var_vsat_mult, locals.var_vsat_mult_dn0, locals.var_vsat_mult_dn2, locals.var_vsat_mult_dn3, locals.var_vsat_mult_dn4, locals.var_vsat_mult_dn5, locals.var_vsat_mult_dn6, locals.var_vsat_mult_dn7, locals.var_vsat_mult_dn8, locals.var_vsat_mult_dn9, locals.var_vsat_mult_dn10, locals.var_vsat_mult_dn11, locals.var_vsat_mult_dn12, locals.var_vsat_mult_dn13, locals.var_vsat_mult_dn14, ) = (assign15850_e23247, ((((locals.var_rho_dn0 * p.p916) * assign15850_e23246) - (assign15850_e23241 * (locals.var_rho_ref_dn0 * p.p916))) / (assign15850_e23246 * assign15850_e23246)), ((((locals.var_rho_dn2 * p.p916) * assign15850_e23246) - (assign15850_e23241 * (locals.var_rho_ref_dn2 * p.p916))) / (assign15850_e23246 * assign15850_e23246)), ((((locals.var_rho_dn3 * p.p916) * assign15850_e23246) - (assign15850_e23241 * (locals.var_rho_ref_dn3 * p.p916))) / (assign15850_e23246 * assign15850_e23246)), ((((locals.var_rho_dn4 * p.p916) * assign15850_e23246) - (assign15850_e23241 * (locals.var_rho_ref_dn4 * p.p916))) / (assign15850_e23246 * assign15850_e23246)), ((((locals.var_rho_dn5 * p.p916) * assign15850_e23246) - (assign15850_e23241 * (locals.var_rho_ref_dn5 * p.p916))) / (assign15850_e23246 * assign15850_e23246)), ((((locals.var_rho_dn6 * p.p916) * assign15850_e23246) - (assign15850_e23241 * (locals.var_rho_ref_dn6 * p.p916))) / (assign15850_e23246 * assign15850_e23246)), ((((locals.var_rho_dn7 * p.p916) * assign15850_e23246) - (assign15850_e23241 * (locals.var_rho_ref_dn7 * p.p916))) / (assign15850_e23246 * assign15850_e23246)), ((((locals.var_rho_dn8 * p.p916) * assign15850_e23246) - (assign15850_e23241 * (locals.var_rho_ref_dn8 * p.p916))) / (assign15850_e23246 * assign15850_e23246)), ((((locals.var_rho_dn9 * p.p916) * assign15850_e23246) - (assign15850_e23241 * (locals.var_rho_ref_dn9 * p.p916))) / (assign15850_e23246 * assign15850_e23246)), ((((locals.var_rho_dn10 * p.p916) * assign15850_e23246) - (assign15850_e23241 * (locals.var_rho_ref_dn10 * p.p916))) / (assign15850_e23246 * assign15850_e23246)), ((((locals.var_rho_dn11 * p.p916) * assign15850_e23246) - (assign15850_e23241 * (locals.var_rho_ref_dn11 * p.p916))) / (assign15850_e23246 * assign15850_e23246)), ((((locals.var_rho_dn12 * p.p916) * assign15850_e23246) - (assign15850_e23241 * (locals.var_rho_ref_dn12 * p.p916))) / (assign15850_e23246 * assign15850_e23246)), ((((locals.var_rho_dn13 * p.p916) * assign15850_e23246) - (assign15850_e23241 * (locals.var_rho_ref_dn13 * p.p916))) / (assign15850_e23246 * assign15850_e23246)), ((((locals.var_rho_dn14 * p.p916) * assign15850_e23246) - (assign15850_e23241 * (locals.var_rho_ref_dn14 * p.p916))) / (assign15850_e23246 * assign15850_e23246)), );
            locals.var_vsat_mult_rv = 0.0;
        }

        if (locals.var_guard488 != 0.0) {
            let assign15860_e23253: f64 = (p.p923 / locals.var_kstress_vth0);
            let assign15860_e23256: f64 = (locals.var_inv_od - locals.var_inv_odref);
            let assign15860_e23257: f64 = (assign15860_e23253 * assign15860_e23256);
            (locals.var_vth0_stress, locals.var_vth0_stress_dn0, locals.var_vth0_stress_dn2, locals.var_vth0_stress_dn3, locals.var_vth0_stress_dn4, locals.var_vth0_stress_dn5, locals.var_vth0_stress_dn6, locals.var_vth0_stress_dn7, locals.var_vth0_stress_dn8, locals.var_vth0_stress_dn9, locals.var_vth0_stress_dn10, locals.var_vth0_stress_dn11, locals.var_vth0_stress_dn12, locals.var_vth0_stress_dn13, locals.var_vth0_stress_dn14, ) = (assign15860_e23257, (((-((p.p923 * locals.var_kstress_vth0_dn0) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15860_e23256) + (assign15860_e23253 * locals.var_inv_od_dn0)), (((-((p.p923 * locals.var_kstress_vth0_dn2) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15860_e23256) + (assign15860_e23253 * locals.var_inv_od_dn2)), (((-((p.p923 * locals.var_kstress_vth0_dn3) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15860_e23256) + (assign15860_e23253 * locals.var_inv_od_dn3)), (((-((p.p923 * locals.var_kstress_vth0_dn4) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15860_e23256) + (assign15860_e23253 * locals.var_inv_od_dn4)), (((-((p.p923 * locals.var_kstress_vth0_dn5) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15860_e23256) + (assign15860_e23253 * locals.var_inv_od_dn5)), (((-((p.p923 * locals.var_kstress_vth0_dn6) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15860_e23256) + (assign15860_e23253 * locals.var_inv_od_dn6)), (((-((p.p923 * locals.var_kstress_vth0_dn7) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15860_e23256) + (assign15860_e23253 * locals.var_inv_od_dn7)), (((-((p.p923 * locals.var_kstress_vth0_dn8) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15860_e23256) + (assign15860_e23253 * locals.var_inv_od_dn8)), (((-((p.p923 * locals.var_kstress_vth0_dn9) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15860_e23256) + (assign15860_e23253 * locals.var_inv_od_dn9)), (((-((p.p923 * locals.var_kstress_vth0_dn10) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15860_e23256) + (assign15860_e23253 * locals.var_inv_od_dn10)), (((-((p.p923 * locals.var_kstress_vth0_dn11) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15860_e23256) + (assign15860_e23253 * locals.var_inv_od_dn11)), (((-((p.p923 * locals.var_kstress_vth0_dn12) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15860_e23256) + (assign15860_e23253 * locals.var_inv_od_dn12)), (((-((p.p923 * locals.var_kstress_vth0_dn13) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15860_e23256) + (assign15860_e23253 * locals.var_inv_od_dn13)), (((-((p.p923 * locals.var_kstress_vth0_dn14) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15860_e23256) + (assign15860_e23253 * locals.var_inv_od_dn14)), );
            locals.var_vth0_stress_rv = 0.0;
        }

        if (locals.var_guard488 != 0.0) {
            let assign15870_e23264: f64 = (locals.var_kstress_vth0).powf(p.p930);
            let assign15870_e23265: f64 = (p.p929 / assign15870_e23264);
            let assign15870_e23268: f64 = (locals.var_inv_od - locals.var_inv_odref);
            let assign15870_e23269: f64 = (assign15870_e23265 * assign15870_e23268);
            (locals.var_k2_stress, locals.var_k2_stress_dn0, locals.var_k2_stress_dn2, locals.var_k2_stress_dn3, locals.var_k2_stress_dn4, locals.var_k2_stress_dn5, locals.var_k2_stress_dn6, locals.var_k2_stress_dn7, locals.var_k2_stress_dn8, locals.var_k2_stress_dn9, locals.var_k2_stress_dn10, locals.var_k2_stress_dn11, locals.var_k2_stress_dn12, locals.var_k2_stress_dn13, locals.var_k2_stress_dn14, ) = (assign15870_e23269, (((-((p.p929 * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn0)) } } else { (assign15870_e23264 * (p.p930 * (locals.var_kstress_vth0_dn0 / locals.var_kstress_vth0))) }) / (assign15870_e23264 * assign15870_e23264))) * assign15870_e23268) + (assign15870_e23265 * locals.var_inv_od_dn0)), (((-((p.p929 * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn2)) } } else { (assign15870_e23264 * (p.p930 * (locals.var_kstress_vth0_dn2 / locals.var_kstress_vth0))) }) / (assign15870_e23264 * assign15870_e23264))) * assign15870_e23268) + (assign15870_e23265 * locals.var_inv_od_dn2)), (((-((p.p929 * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn3)) } } else { (assign15870_e23264 * (p.p930 * (locals.var_kstress_vth0_dn3 / locals.var_kstress_vth0))) }) / (assign15870_e23264 * assign15870_e23264))) * assign15870_e23268) + (assign15870_e23265 * locals.var_inv_od_dn3)), (((-((p.p929 * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn4)) } } else { (assign15870_e23264 * (p.p930 * (locals.var_kstress_vth0_dn4 / locals.var_kstress_vth0))) }) / (assign15870_e23264 * assign15870_e23264))) * assign15870_e23268) + (assign15870_e23265 * locals.var_inv_od_dn4)), (((-((p.p929 * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn5)) } } else { (assign15870_e23264 * (p.p930 * (locals.var_kstress_vth0_dn5 / locals.var_kstress_vth0))) }) / (assign15870_e23264 * assign15870_e23264))) * assign15870_e23268) + (assign15870_e23265 * locals.var_inv_od_dn5)), (((-((p.p929 * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn6)) } } else { (assign15870_e23264 * (p.p930 * (locals.var_kstress_vth0_dn6 / locals.var_kstress_vth0))) }) / (assign15870_e23264 * assign15870_e23264))) * assign15870_e23268) + (assign15870_e23265 * locals.var_inv_od_dn6)), (((-((p.p929 * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn7)) } } else { (assign15870_e23264 * (p.p930 * (locals.var_kstress_vth0_dn7 / locals.var_kstress_vth0))) }) / (assign15870_e23264 * assign15870_e23264))) * assign15870_e23268) + (assign15870_e23265 * locals.var_inv_od_dn7)), (((-((p.p929 * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn8)) } } else { (assign15870_e23264 * (p.p930 * (locals.var_kstress_vth0_dn8 / locals.var_kstress_vth0))) }) / (assign15870_e23264 * assign15870_e23264))) * assign15870_e23268) + (assign15870_e23265 * locals.var_inv_od_dn8)), (((-((p.p929 * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn9)) } } else { (assign15870_e23264 * (p.p930 * (locals.var_kstress_vth0_dn9 / locals.var_kstress_vth0))) }) / (assign15870_e23264 * assign15870_e23264))) * assign15870_e23268) + (assign15870_e23265 * locals.var_inv_od_dn9)), (((-((p.p929 * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn10)) } } else { (assign15870_e23264 * (p.p930 * (locals.var_kstress_vth0_dn10 / locals.var_kstress_vth0))) }) / (assign15870_e23264 * assign15870_e23264))) * assign15870_e23268) + (assign15870_e23265 * locals.var_inv_od_dn10)), (((-((p.p929 * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn11)) } } else { (assign15870_e23264 * (p.p930 * (locals.var_kstress_vth0_dn11 / locals.var_kstress_vth0))) }) / (assign15870_e23264 * assign15870_e23264))) * assign15870_e23268) + (assign15870_e23265 * locals.var_inv_od_dn11)), (((-((p.p929 * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn12)) } } else { (assign15870_e23264 * (p.p930 * (locals.var_kstress_vth0_dn12 / locals.var_kstress_vth0))) }) / (assign15870_e23264 * assign15870_e23264))) * assign15870_e23268) + (assign15870_e23265 * locals.var_inv_od_dn12)), (((-((p.p929 * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn13)) } } else { (assign15870_e23264 * (p.p930 * (locals.var_kstress_vth0_dn13 / locals.var_kstress_vth0))) }) / (assign15870_e23264 * assign15870_e23264))) * assign15870_e23268) + (assign15870_e23265 * locals.var_inv_od_dn13)), (((-((p.p929 * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn14)) } } else { (assign15870_e23264 * (p.p930 * (locals.var_kstress_vth0_dn14 / locals.var_kstress_vth0))) }) / (assign15870_e23264 * assign15870_e23264))) * assign15870_e23268) + (assign15870_e23265 * locals.var_inv_od_dn14)), );
            locals.var_k2_stress_rv = 0.0;
        }

        if (locals.var_guard488 != 0.0) {
            let assign15880_e23276: f64 = (locals.var_kstress_vth0).powf(p.p932);
            let assign15880_e23277: f64 = (p.p931 / assign15880_e23276);
            let assign15880_e23280: f64 = (locals.var_inv_od - locals.var_inv_odref);
            let assign15880_e23281: f64 = (assign15880_e23277 * assign15880_e23280);
            (locals.var_eta_stress, locals.var_eta_stress_dn0, locals.var_eta_stress_dn2, locals.var_eta_stress_dn3, locals.var_eta_stress_dn4, locals.var_eta_stress_dn5, locals.var_eta_stress_dn6, locals.var_eta_stress_dn7, locals.var_eta_stress_dn8, locals.var_eta_stress_dn9, locals.var_eta_stress_dn10, locals.var_eta_stress_dn11, locals.var_eta_stress_dn12, locals.var_eta_stress_dn13, locals.var_eta_stress_dn14, ) = (assign15880_e23281, (((-((p.p931 * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn0)) } } else { (assign15880_e23276 * (p.p932 * (locals.var_kstress_vth0_dn0 / locals.var_kstress_vth0))) }) / (assign15880_e23276 * assign15880_e23276))) * assign15880_e23280) + (assign15880_e23277 * locals.var_inv_od_dn0)), (((-((p.p931 * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn2)) } } else { (assign15880_e23276 * (p.p932 * (locals.var_kstress_vth0_dn2 / locals.var_kstress_vth0))) }) / (assign15880_e23276 * assign15880_e23276))) * assign15880_e23280) + (assign15880_e23277 * locals.var_inv_od_dn2)), (((-((p.p931 * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn3)) } } else { (assign15880_e23276 * (p.p932 * (locals.var_kstress_vth0_dn3 / locals.var_kstress_vth0))) }) / (assign15880_e23276 * assign15880_e23276))) * assign15880_e23280) + (assign15880_e23277 * locals.var_inv_od_dn3)), (((-((p.p931 * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn4)) } } else { (assign15880_e23276 * (p.p932 * (locals.var_kstress_vth0_dn4 / locals.var_kstress_vth0))) }) / (assign15880_e23276 * assign15880_e23276))) * assign15880_e23280) + (assign15880_e23277 * locals.var_inv_od_dn4)), (((-((p.p931 * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn5)) } } else { (assign15880_e23276 * (p.p932 * (locals.var_kstress_vth0_dn5 / locals.var_kstress_vth0))) }) / (assign15880_e23276 * assign15880_e23276))) * assign15880_e23280) + (assign15880_e23277 * locals.var_inv_od_dn5)), (((-((p.p931 * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn6)) } } else { (assign15880_e23276 * (p.p932 * (locals.var_kstress_vth0_dn6 / locals.var_kstress_vth0))) }) / (assign15880_e23276 * assign15880_e23276))) * assign15880_e23280) + (assign15880_e23277 * locals.var_inv_od_dn6)), (((-((p.p931 * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn7)) } } else { (assign15880_e23276 * (p.p932 * (locals.var_kstress_vth0_dn7 / locals.var_kstress_vth0))) }) / (assign15880_e23276 * assign15880_e23276))) * assign15880_e23280) + (assign15880_e23277 * locals.var_inv_od_dn7)), (((-((p.p931 * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn8)) } } else { (assign15880_e23276 * (p.p932 * (locals.var_kstress_vth0_dn8 / locals.var_kstress_vth0))) }) / (assign15880_e23276 * assign15880_e23276))) * assign15880_e23280) + (assign15880_e23277 * locals.var_inv_od_dn8)), (((-((p.p931 * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn9)) } } else { (assign15880_e23276 * (p.p932 * (locals.var_kstress_vth0_dn9 / locals.var_kstress_vth0))) }) / (assign15880_e23276 * assign15880_e23276))) * assign15880_e23280) + (assign15880_e23277 * locals.var_inv_od_dn9)), (((-((p.p931 * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn10)) } } else { (assign15880_e23276 * (p.p932 * (locals.var_kstress_vth0_dn10 / locals.var_kstress_vth0))) }) / (assign15880_e23276 * assign15880_e23276))) * assign15880_e23280) + (assign15880_e23277 * locals.var_inv_od_dn10)), (((-((p.p931 * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn11)) } } else { (assign15880_e23276 * (p.p932 * (locals.var_kstress_vth0_dn11 / locals.var_kstress_vth0))) }) / (assign15880_e23276 * assign15880_e23276))) * assign15880_e23280) + (assign15880_e23277 * locals.var_inv_od_dn11)), (((-((p.p931 * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn12)) } } else { (assign15880_e23276 * (p.p932 * (locals.var_kstress_vth0_dn12 / locals.var_kstress_vth0))) }) / (assign15880_e23276 * assign15880_e23276))) * assign15880_e23280) + (assign15880_e23277 * locals.var_inv_od_dn12)), (((-((p.p931 * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn13)) } } else { (assign15880_e23276 * (p.p932 * (locals.var_kstress_vth0_dn13 / locals.var_kstress_vth0))) }) / (assign15880_e23276 * assign15880_e23276))) * assign15880_e23280) + (assign15880_e23277 * locals.var_inv_od_dn13)), (((-((p.p931 * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn14)) } } else { (assign15880_e23276 * (p.p932 * (locals.var_kstress_vth0_dn14 / locals.var_kstress_vth0))) }) / (assign15880_e23276 * assign15880_e23276))) * assign15880_e23280) + (assign15880_e23277 * locals.var_inv_od_dn14)), );
            locals.var_eta_stress_rv = 0.0;
        }

        if (locals.var_guard488 != 0.0) {
            let assign15890_e23287: f64 = (locals.var_u0_t * locals.var_mu0_mult);
            (locals.var_u0_t, locals.var_u0_t_dn0, locals.var_u0_t_dn2, locals.var_u0_t_dn3, locals.var_u0_t_dn4, locals.var_u0_t_dn5, locals.var_u0_t_dn6, locals.var_u0_t_dn7, locals.var_u0_t_dn8, locals.var_u0_t_dn9, locals.var_u0_t_dn10, locals.var_u0_t_dn11, locals.var_u0_t_dn12, locals.var_u0_t_dn13, locals.var_u0_t_dn14, ) = (assign15890_e23287, ((locals.var_u0_t_dn0 * locals.var_mu0_mult) + (locals.var_u0_t * locals.var_mu0_mult_dn0)), ((locals.var_u0_t_dn2 * locals.var_mu0_mult) + (locals.var_u0_t * locals.var_mu0_mult_dn2)), ((locals.var_u0_t_dn3 * locals.var_mu0_mult) + (locals.var_u0_t * locals.var_mu0_mult_dn3)), ((locals.var_u0_t_dn4 * locals.var_mu0_mult) + (locals.var_u0_t * locals.var_mu0_mult_dn4)), ((locals.var_u0_t_dn5 * locals.var_mu0_mult) + (locals.var_u0_t * locals.var_mu0_mult_dn5)), ((locals.var_u0_t_dn6 * locals.var_mu0_mult) + (locals.var_u0_t * locals.var_mu0_mult_dn6)), ((locals.var_u0_t_dn7 * locals.var_mu0_mult) + (locals.var_u0_t * locals.var_mu0_mult_dn7)), ((locals.var_u0_t_dn8 * locals.var_mu0_mult) + (locals.var_u0_t * locals.var_mu0_mult_dn8)), ((locals.var_u0_t_dn9 * locals.var_mu0_mult) + (locals.var_u0_t * locals.var_mu0_mult_dn9)), ((locals.var_u0_t_dn10 * locals.var_mu0_mult) + (locals.var_u0_t * locals.var_mu0_mult_dn10)), ((locals.var_u0_t_dn11 * locals.var_mu0_mult) + (locals.var_u0_t * locals.var_mu0_mult_dn11)), ((locals.var_u0_t_dn12 * locals.var_mu0_mult) + (locals.var_u0_t * locals.var_mu0_mult_dn12)), ((locals.var_u0_t_dn13 * locals.var_mu0_mult) + (locals.var_u0_t * locals.var_mu0_mult_dn13)), ((locals.var_u0_t_dn14 * locals.var_mu0_mult) + (locals.var_u0_t * locals.var_mu0_mult_dn14)), );
            locals.var_u0_t_rv = 0.0;
        }

        if (locals.var_guard488 != 0.0) {
            let assign15900_e23293: f64 = (locals.var_vsat_t * locals.var_vsat_mult);
            (locals.var_vsat_t, locals.var_vsat_t_dn0, locals.var_vsat_t_dn2, locals.var_vsat_t_dn3, locals.var_vsat_t_dn4, locals.var_vsat_t_dn5, locals.var_vsat_t_dn6, locals.var_vsat_t_dn7, locals.var_vsat_t_dn8, locals.var_vsat_t_dn9, locals.var_vsat_t_dn10, locals.var_vsat_t_dn11, locals.var_vsat_t_dn12, locals.var_vsat_t_dn13, locals.var_vsat_t_dn14, ) = (assign15900_e23293, ((locals.var_vsat_t_dn0 * locals.var_vsat_mult) + (locals.var_vsat_t * locals.var_vsat_mult_dn0)), ((locals.var_vsat_t_dn2 * locals.var_vsat_mult) + (locals.var_vsat_t * locals.var_vsat_mult_dn2)), ((locals.var_vsat_t_dn3 * locals.var_vsat_mult) + (locals.var_vsat_t * locals.var_vsat_mult_dn3)), ((locals.var_vsat_t_dn4 * locals.var_vsat_mult) + (locals.var_vsat_t * locals.var_vsat_mult_dn4)), ((locals.var_vsat_t_dn5 * locals.var_vsat_mult) + (locals.var_vsat_t * locals.var_vsat_mult_dn5)), ((locals.var_vsat_t_dn6 * locals.var_vsat_mult) + (locals.var_vsat_t * locals.var_vsat_mult_dn6)), ((locals.var_vsat_t_dn7 * locals.var_vsat_mult) + (locals.var_vsat_t * locals.var_vsat_mult_dn7)), ((locals.var_vsat_t_dn8 * locals.var_vsat_mult) + (locals.var_vsat_t * locals.var_vsat_mult_dn8)), ((locals.var_vsat_t_dn9 * locals.var_vsat_mult) + (locals.var_vsat_t * locals.var_vsat_mult_dn9)), ((locals.var_vsat_t_dn10 * locals.var_vsat_mult) + (locals.var_vsat_t * locals.var_vsat_mult_dn10)), ((locals.var_vsat_t_dn11 * locals.var_vsat_mult) + (locals.var_vsat_t * locals.var_vsat_mult_dn11)), ((locals.var_vsat_t_dn12 * locals.var_vsat_mult) + (locals.var_vsat_t * locals.var_vsat_mult_dn12)), ((locals.var_vsat_t_dn13 * locals.var_vsat_mult) + (locals.var_vsat_t * locals.var_vsat_mult_dn13)), ((locals.var_vsat_t_dn14 * locals.var_vsat_mult) + (locals.var_vsat_t * locals.var_vsat_mult_dn14)), );
            locals.var_vsat_t_rv = 0.0;
        }

        if (locals.var_guard488 != 0.0) {
            let assign15910_e23299: f64 = (locals.var_k2_i + locals.var_k2_stress);
            (locals.var_k2_i, locals.var_k2_i_dn0, locals.var_k2_i_dn2, locals.var_k2_i_dn3, locals.var_k2_i_dn4, locals.var_k2_i_dn5, locals.var_k2_i_dn6, locals.var_k2_i_dn7, locals.var_k2_i_dn8, locals.var_k2_i_dn9, locals.var_k2_i_dn10, locals.var_k2_i_dn11, locals.var_k2_i_dn12, locals.var_k2_i_dn13, locals.var_k2_i_dn14, ) = (assign15910_e23299, (locals.var_k2_i_dn0 + locals.var_k2_stress_dn0), (locals.var_k2_i_dn2 + locals.var_k2_stress_dn2), (locals.var_k2_i_dn3 + locals.var_k2_stress_dn3), (locals.var_k2_i_dn4 + locals.var_k2_stress_dn4), (locals.var_k2_i_dn5 + locals.var_k2_stress_dn5), (locals.var_k2_i_dn6 + locals.var_k2_stress_dn6), (locals.var_k2_i_dn7 + locals.var_k2_stress_dn7), (locals.var_k2_i_dn8 + locals.var_k2_stress_dn8), (locals.var_k2_i_dn9 + locals.var_k2_stress_dn9), (locals.var_k2_i_dn10 + locals.var_k2_stress_dn10), (locals.var_k2_i_dn11 + locals.var_k2_stress_dn11), (locals.var_k2_i_dn12 + locals.var_k2_stress_dn12), (locals.var_k2_i_dn13 + locals.var_k2_stress_dn13), (locals.var_k2_i_dn14 + locals.var_k2_stress_dn14), );
            locals.var_k2_i_rv = 0.0;
        }

        if (locals.var_guard488 != 0.0) {
            let assign15920_e23305: f64 = (locals.var_eta0_t + locals.var_eta_stress);
            (locals.var_eta0_t, locals.var_eta0_t_dn0, locals.var_eta0_t_dn2, locals.var_eta0_t_dn3, locals.var_eta0_t_dn4, locals.var_eta0_t_dn5, locals.var_eta0_t_dn6, locals.var_eta0_t_dn7, locals.var_eta0_t_dn8, locals.var_eta0_t_dn9, locals.var_eta0_t_dn10, locals.var_eta0_t_dn11, locals.var_eta0_t_dn12, locals.var_eta0_t_dn13, locals.var_eta0_t_dn14, ) = (assign15920_e23305, (locals.var_eta0_t_dn0 + locals.var_eta_stress_dn0), (locals.var_eta0_t_dn2 + locals.var_eta_stress_dn2), (locals.var_eta0_t_dn3 + locals.var_eta_stress_dn3), (locals.var_eta0_t_dn4 + locals.var_eta_stress_dn4), (locals.var_eta0_t_dn5 + locals.var_eta_stress_dn5), (locals.var_eta0_t_dn6 + locals.var_eta_stress_dn6), (locals.var_eta0_t_dn7 + locals.var_eta_stress_dn7), (locals.var_eta0_t_dn8 + locals.var_eta_stress_dn8), (locals.var_eta0_t_dn9 + locals.var_eta_stress_dn9), (locals.var_eta0_t_dn10 + locals.var_eta_stress_dn10), (locals.var_eta0_t_dn11 + locals.var_eta_stress_dn11), (locals.var_eta0_t_dn12 + locals.var_eta_stress_dn12), (locals.var_eta0_t_dn13 + locals.var_eta_stress_dn13), (locals.var_eta0_t_dn14 + locals.var_eta_stress_dn14), );
            locals.var_eta0_t_rv = 0.0;
        }

        let assign15930_e23310: f64 = if p.p37 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard489 = assign15930_e23310;
        locals.var_guard489_rv = 0.0;

        if ((locals.var_guard488 != 0.0) && (locals.var_guard489 != 0.0)) {
            let assign15940_e23316: f64 = (locals.var_kvth0edge_i / locals.var_kstress_vth0);
            let assign15940_e23319: f64 = (locals.var_inv_od - locals.var_inv_odref);
            let assign15940_e23320: f64 = (assign15940_e23316 * assign15940_e23319);
            (locals.var_vth0_stress_edge, locals.var_vth0_stress_edge_dn0, locals.var_vth0_stress_edge_dn2, locals.var_vth0_stress_edge_dn3, locals.var_vth0_stress_edge_dn4, locals.var_vth0_stress_edge_dn5, locals.var_vth0_stress_edge_dn6, locals.var_vth0_stress_edge_dn7, locals.var_vth0_stress_edge_dn8, locals.var_vth0_stress_edge_dn9, locals.var_vth0_stress_edge_dn10, locals.var_vth0_stress_edge_dn11, locals.var_vth0_stress_edge_dn12, locals.var_vth0_stress_edge_dn13, locals.var_vth0_stress_edge_dn14, ) = (assign15940_e23320, (((-((locals.var_kvth0edge_i * locals.var_kstress_vth0_dn0) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15940_e23319) + (assign15940_e23316 * locals.var_inv_od_dn0)), (((-((locals.var_kvth0edge_i * locals.var_kstress_vth0_dn2) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15940_e23319) + (assign15940_e23316 * locals.var_inv_od_dn2)), (((-((locals.var_kvth0edge_i * locals.var_kstress_vth0_dn3) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15940_e23319) + (assign15940_e23316 * locals.var_inv_od_dn3)), (((-((locals.var_kvth0edge_i * locals.var_kstress_vth0_dn4) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15940_e23319) + (assign15940_e23316 * locals.var_inv_od_dn4)), (((-((locals.var_kvth0edge_i * locals.var_kstress_vth0_dn5) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15940_e23319) + (assign15940_e23316 * locals.var_inv_od_dn5)), (((-((locals.var_kvth0edge_i * locals.var_kstress_vth0_dn6) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15940_e23319) + (assign15940_e23316 * locals.var_inv_od_dn6)), (((-((locals.var_kvth0edge_i * locals.var_kstress_vth0_dn7) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15940_e23319) + (assign15940_e23316 * locals.var_inv_od_dn7)), (((-((locals.var_kvth0edge_i * locals.var_kstress_vth0_dn8) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15940_e23319) + (assign15940_e23316 * locals.var_inv_od_dn8)), (((-((locals.var_kvth0edge_i * locals.var_kstress_vth0_dn9) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15940_e23319) + (assign15940_e23316 * locals.var_inv_od_dn9)), (((-((locals.var_kvth0edge_i * locals.var_kstress_vth0_dn10) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15940_e23319) + (assign15940_e23316 * locals.var_inv_od_dn10)), (((-((locals.var_kvth0edge_i * locals.var_kstress_vth0_dn11) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15940_e23319) + (assign15940_e23316 * locals.var_inv_od_dn11)), (((-((locals.var_kvth0edge_i * locals.var_kstress_vth0_dn12) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15940_e23319) + (assign15940_e23316 * locals.var_inv_od_dn12)), (((-((locals.var_kvth0edge_i * locals.var_kstress_vth0_dn13) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15940_e23319) + (assign15940_e23316 * locals.var_inv_od_dn13)), (((-((locals.var_kvth0edge_i * locals.var_kstress_vth0_dn14) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15940_e23319) + (assign15940_e23316 * locals.var_inv_od_dn14)), );
            locals.var_vth0_stress_edge_rv = 0.0;
        }

        if ((locals.var_guard488 != 0.0) && (locals.var_guard489 != 0.0)) {
            let assign15950_e23329: f64 = (locals.var_kstress_vth0).powf(p.p930);
            let assign15950_e23330: f64 = (locals.var_stk2edge_i / assign15950_e23329);
            let assign15950_e23333: f64 = (locals.var_inv_od - locals.var_inv_odref);
            let assign15950_e23334: f64 = (assign15950_e23330 * assign15950_e23333);
            (locals.var_k2_stress_edge, locals.var_k2_stress_edge_dn0, locals.var_k2_stress_edge_dn2, locals.var_k2_stress_edge_dn3, locals.var_k2_stress_edge_dn4, locals.var_k2_stress_edge_dn5, locals.var_k2_stress_edge_dn6, locals.var_k2_stress_edge_dn7, locals.var_k2_stress_edge_dn8, locals.var_k2_stress_edge_dn9, locals.var_k2_stress_edge_dn10, locals.var_k2_stress_edge_dn11, locals.var_k2_stress_edge_dn12, locals.var_k2_stress_edge_dn13, locals.var_k2_stress_edge_dn14, ) = (assign15950_e23334, (((-((locals.var_stk2edge_i * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn0)) } } else { (assign15950_e23329 * (p.p930 * (locals.var_kstress_vth0_dn0 / locals.var_kstress_vth0))) }) / (assign15950_e23329 * assign15950_e23329))) * assign15950_e23333) + (assign15950_e23330 * locals.var_inv_od_dn0)), (((-((locals.var_stk2edge_i * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn2)) } } else { (assign15950_e23329 * (p.p930 * (locals.var_kstress_vth0_dn2 / locals.var_kstress_vth0))) }) / (assign15950_e23329 * assign15950_e23329))) * assign15950_e23333) + (assign15950_e23330 * locals.var_inv_od_dn2)), (((-((locals.var_stk2edge_i * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn3)) } } else { (assign15950_e23329 * (p.p930 * (locals.var_kstress_vth0_dn3 / locals.var_kstress_vth0))) }) / (assign15950_e23329 * assign15950_e23329))) * assign15950_e23333) + (assign15950_e23330 * locals.var_inv_od_dn3)), (((-((locals.var_stk2edge_i * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn4)) } } else { (assign15950_e23329 * (p.p930 * (locals.var_kstress_vth0_dn4 / locals.var_kstress_vth0))) }) / (assign15950_e23329 * assign15950_e23329))) * assign15950_e23333) + (assign15950_e23330 * locals.var_inv_od_dn4)), (((-((locals.var_stk2edge_i * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn5)) } } else { (assign15950_e23329 * (p.p930 * (locals.var_kstress_vth0_dn5 / locals.var_kstress_vth0))) }) / (assign15950_e23329 * assign15950_e23329))) * assign15950_e23333) + (assign15950_e23330 * locals.var_inv_od_dn5)), (((-((locals.var_stk2edge_i * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn6)) } } else { (assign15950_e23329 * (p.p930 * (locals.var_kstress_vth0_dn6 / locals.var_kstress_vth0))) }) / (assign15950_e23329 * assign15950_e23329))) * assign15950_e23333) + (assign15950_e23330 * locals.var_inv_od_dn6)), (((-((locals.var_stk2edge_i * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn7)) } } else { (assign15950_e23329 * (p.p930 * (locals.var_kstress_vth0_dn7 / locals.var_kstress_vth0))) }) / (assign15950_e23329 * assign15950_e23329))) * assign15950_e23333) + (assign15950_e23330 * locals.var_inv_od_dn7)), (((-((locals.var_stk2edge_i * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn8)) } } else { (assign15950_e23329 * (p.p930 * (locals.var_kstress_vth0_dn8 / locals.var_kstress_vth0))) }) / (assign15950_e23329 * assign15950_e23329))) * assign15950_e23333) + (assign15950_e23330 * locals.var_inv_od_dn8)), (((-((locals.var_stk2edge_i * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn9)) } } else { (assign15950_e23329 * (p.p930 * (locals.var_kstress_vth0_dn9 / locals.var_kstress_vth0))) }) / (assign15950_e23329 * assign15950_e23329))) * assign15950_e23333) + (assign15950_e23330 * locals.var_inv_od_dn9)), (((-((locals.var_stk2edge_i * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn10)) } } else { (assign15950_e23329 * (p.p930 * (locals.var_kstress_vth0_dn10 / locals.var_kstress_vth0))) }) / (assign15950_e23329 * assign15950_e23329))) * assign15950_e23333) + (assign15950_e23330 * locals.var_inv_od_dn10)), (((-((locals.var_stk2edge_i * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn11)) } } else { (assign15950_e23329 * (p.p930 * (locals.var_kstress_vth0_dn11 / locals.var_kstress_vth0))) }) / (assign15950_e23329 * assign15950_e23329))) * assign15950_e23333) + (assign15950_e23330 * locals.var_inv_od_dn11)), (((-((locals.var_stk2edge_i * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn12)) } } else { (assign15950_e23329 * (p.p930 * (locals.var_kstress_vth0_dn12 / locals.var_kstress_vth0))) }) / (assign15950_e23329 * assign15950_e23329))) * assign15950_e23333) + (assign15950_e23330 * locals.var_inv_od_dn12)), (((-((locals.var_stk2edge_i * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn13)) } } else { (assign15950_e23329 * (p.p930 * (locals.var_kstress_vth0_dn13 / locals.var_kstress_vth0))) }) / (assign15950_e23329 * assign15950_e23329))) * assign15950_e23333) + (assign15950_e23330 * locals.var_inv_od_dn13)), (((-((locals.var_stk2edge_i * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn14)) } } else { (assign15950_e23329 * (p.p930 * (locals.var_kstress_vth0_dn14 / locals.var_kstress_vth0))) }) / (assign15950_e23329 * assign15950_e23329))) * assign15950_e23333) + (assign15950_e23330 * locals.var_inv_od_dn14)), );
            locals.var_k2_stress_edge_rv = 0.0;
        }

        if ((locals.var_guard488 != 0.0) && (locals.var_guard489 != 0.0)) {
            let assign15960_e23343: f64 = (locals.var_kstress_vth0).powf(p.p932);
            let assign15960_e23344: f64 = (locals.var_steta0edge_i / assign15960_e23343);
            let assign15960_e23347: f64 = (locals.var_inv_od - locals.var_inv_odref);
            let assign15960_e23348: f64 = (assign15960_e23344 * assign15960_e23347);
            (locals.var_eta_stress_edge, locals.var_eta_stress_edge_dn0, locals.var_eta_stress_edge_dn2, locals.var_eta_stress_edge_dn3, locals.var_eta_stress_edge_dn4, locals.var_eta_stress_edge_dn5, locals.var_eta_stress_edge_dn6, locals.var_eta_stress_edge_dn7, locals.var_eta_stress_edge_dn8, locals.var_eta_stress_edge_dn9, locals.var_eta_stress_edge_dn10, locals.var_eta_stress_edge_dn11, locals.var_eta_stress_edge_dn12, locals.var_eta_stress_edge_dn13, locals.var_eta_stress_edge_dn14, ) = (assign15960_e23348, (((-((locals.var_steta0edge_i * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn0)) } } else { (assign15960_e23343 * (p.p932 * (locals.var_kstress_vth0_dn0 / locals.var_kstress_vth0))) }) / (assign15960_e23343 * assign15960_e23343))) * assign15960_e23347) + (assign15960_e23344 * locals.var_inv_od_dn0)), (((-((locals.var_steta0edge_i * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn2)) } } else { (assign15960_e23343 * (p.p932 * (locals.var_kstress_vth0_dn2 / locals.var_kstress_vth0))) }) / (assign15960_e23343 * assign15960_e23343))) * assign15960_e23347) + (assign15960_e23344 * locals.var_inv_od_dn2)), (((-((locals.var_steta0edge_i * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn3)) } } else { (assign15960_e23343 * (p.p932 * (locals.var_kstress_vth0_dn3 / locals.var_kstress_vth0))) }) / (assign15960_e23343 * assign15960_e23343))) * assign15960_e23347) + (assign15960_e23344 * locals.var_inv_od_dn3)), (((-((locals.var_steta0edge_i * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn4)) } } else { (assign15960_e23343 * (p.p932 * (locals.var_kstress_vth0_dn4 / locals.var_kstress_vth0))) }) / (assign15960_e23343 * assign15960_e23343))) * assign15960_e23347) + (assign15960_e23344 * locals.var_inv_od_dn4)), (((-((locals.var_steta0edge_i * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn5)) } } else { (assign15960_e23343 * (p.p932 * (locals.var_kstress_vth0_dn5 / locals.var_kstress_vth0))) }) / (assign15960_e23343 * assign15960_e23343))) * assign15960_e23347) + (assign15960_e23344 * locals.var_inv_od_dn5)), (((-((locals.var_steta0edge_i * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn6)) } } else { (assign15960_e23343 * (p.p932 * (locals.var_kstress_vth0_dn6 / locals.var_kstress_vth0))) }) / (assign15960_e23343 * assign15960_e23343))) * assign15960_e23347) + (assign15960_e23344 * locals.var_inv_od_dn6)), (((-((locals.var_steta0edge_i * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn7)) } } else { (assign15960_e23343 * (p.p932 * (locals.var_kstress_vth0_dn7 / locals.var_kstress_vth0))) }) / (assign15960_e23343 * assign15960_e23343))) * assign15960_e23347) + (assign15960_e23344 * locals.var_inv_od_dn7)), (((-((locals.var_steta0edge_i * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn8)) } } else { (assign15960_e23343 * (p.p932 * (locals.var_kstress_vth0_dn8 / locals.var_kstress_vth0))) }) / (assign15960_e23343 * assign15960_e23343))) * assign15960_e23347) + (assign15960_e23344 * locals.var_inv_od_dn8)), (((-((locals.var_steta0edge_i * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn9)) } } else { (assign15960_e23343 * (p.p932 * (locals.var_kstress_vth0_dn9 / locals.var_kstress_vth0))) }) / (assign15960_e23343 * assign15960_e23343))) * assign15960_e23347) + (assign15960_e23344 * locals.var_inv_od_dn9)), (((-((locals.var_steta0edge_i * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn10)) } } else { (assign15960_e23343 * (p.p932 * (locals.var_kstress_vth0_dn10 / locals.var_kstress_vth0))) }) / (assign15960_e23343 * assign15960_e23343))) * assign15960_e23347) + (assign15960_e23344 * locals.var_inv_od_dn10)), (((-((locals.var_steta0edge_i * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn11)) } } else { (assign15960_e23343 * (p.p932 * (locals.var_kstress_vth0_dn11 / locals.var_kstress_vth0))) }) / (assign15960_e23343 * assign15960_e23343))) * assign15960_e23347) + (assign15960_e23344 * locals.var_inv_od_dn11)), (((-((locals.var_steta0edge_i * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn12)) } } else { (assign15960_e23343 * (p.p932 * (locals.var_kstress_vth0_dn12 / locals.var_kstress_vth0))) }) / (assign15960_e23343 * assign15960_e23343))) * assign15960_e23347) + (assign15960_e23344 * locals.var_inv_od_dn12)), (((-((locals.var_steta0edge_i * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn13)) } } else { (assign15960_e23343 * (p.p932 * (locals.var_kstress_vth0_dn13 / locals.var_kstress_vth0))) }) / (assign15960_e23343 * assign15960_e23343))) * assign15960_e23347) + (assign15960_e23344 * locals.var_inv_od_dn13)), (((-((locals.var_steta0edge_i * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn14)) } } else { (assign15960_e23343 * (p.p932 * (locals.var_kstress_vth0_dn14 / locals.var_kstress_vth0))) }) / (assign15960_e23343 * assign15960_e23343))) * assign15960_e23347) + (assign15960_e23344 * locals.var_inv_od_dn14)), );
            locals.var_eta_stress_edge_rv = 0.0;
        }

        if (locals.var_guard488 != 0.0) {
            let assign15970_e23354: f64 = (locals.var_k2edge_i + locals.var_k2_stress_edge);
            (locals.var_k2edge_i, locals.var_k2edge_i_dn0, locals.var_k2edge_i_dn2, locals.var_k2edge_i_dn3, locals.var_k2edge_i_dn4, locals.var_k2edge_i_dn5, locals.var_k2edge_i_dn6, locals.var_k2edge_i_dn7, locals.var_k2edge_i_dn8, locals.var_k2edge_i_dn9, locals.var_k2edge_i_dn10, locals.var_k2edge_i_dn11, locals.var_k2edge_i_dn12, locals.var_k2edge_i_dn13, locals.var_k2edge_i_dn14, ) = (assign15970_e23354, (locals.var_k2edge_i_dn0 + locals.var_k2_stress_edge_dn0), (locals.var_k2edge_i_dn2 + locals.var_k2_stress_edge_dn2), (locals.var_k2edge_i_dn3 + locals.var_k2_stress_edge_dn3), (locals.var_k2edge_i_dn4 + locals.var_k2_stress_edge_dn4), (locals.var_k2edge_i_dn5 + locals.var_k2_stress_edge_dn5), (locals.var_k2edge_i_dn6 + locals.var_k2_stress_edge_dn6), (locals.var_k2edge_i_dn7 + locals.var_k2_stress_edge_dn7), (locals.var_k2edge_i_dn8 + locals.var_k2_stress_edge_dn8), (locals.var_k2edge_i_dn9 + locals.var_k2_stress_edge_dn9), (locals.var_k2edge_i_dn10 + locals.var_k2_stress_edge_dn10), (locals.var_k2edge_i_dn11 + locals.var_k2_stress_edge_dn11), (locals.var_k2edge_i_dn12 + locals.var_k2_stress_edge_dn12), (locals.var_k2edge_i_dn13 + locals.var_k2_stress_edge_dn13), (locals.var_k2edge_i_dn14 + locals.var_k2_stress_edge_dn14), );
            locals.var_k2edge_i_rv = 0.0;
        }

        if (locals.var_guard488 != 0.0) {
            let assign15980_e23360: f64 = (locals.var_eta0edge_i + locals.var_eta_stress_edge);
            (locals.var_eta0edge_i, locals.var_eta0edge_i_dn0, locals.var_eta0edge_i_dn2, locals.var_eta0edge_i_dn3, locals.var_eta0edge_i_dn4, locals.var_eta0edge_i_dn5, locals.var_eta0edge_i_dn6, locals.var_eta0edge_i_dn7, locals.var_eta0edge_i_dn8, locals.var_eta0edge_i_dn9, locals.var_eta0edge_i_dn10, locals.var_eta0edge_i_dn11, locals.var_eta0edge_i_dn12, locals.var_eta0edge_i_dn13, locals.var_eta0edge_i_dn14, ) = (assign15980_e23360, (locals.var_eta0edge_i_dn0 + locals.var_eta_stress_edge_dn0), (locals.var_eta0edge_i_dn2 + locals.var_eta_stress_edge_dn2), (locals.var_eta0edge_i_dn3 + locals.var_eta_stress_edge_dn3), (locals.var_eta0edge_i_dn4 + locals.var_eta_stress_edge_dn4), (locals.var_eta0edge_i_dn5 + locals.var_eta_stress_edge_dn5), (locals.var_eta0edge_i_dn6 + locals.var_eta_stress_edge_dn6), (locals.var_eta0edge_i_dn7 + locals.var_eta_stress_edge_dn7), (locals.var_eta0edge_i_dn8 + locals.var_eta_stress_edge_dn8), (locals.var_eta0edge_i_dn9 + locals.var_eta_stress_edge_dn9), (locals.var_eta0edge_i_dn10 + locals.var_eta_stress_edge_dn10), (locals.var_eta0edge_i_dn11 + locals.var_eta_stress_edge_dn11), (locals.var_eta0edge_i_dn12 + locals.var_eta_stress_edge_dn12), (locals.var_eta0edge_i_dn13 + locals.var_eta_stress_edge_dn13), (locals.var_eta0edge_i_dn14 + locals.var_eta_stress_edge_dn14), );
            locals.var_eta0edge_i_rv = 0.0;
        }

        if (locals.var_guard488 == 0.0) {
            (locals.var_vth0_stress, locals.var_vth0_stress_dn0, locals.var_vth0_stress_dn2, locals.var_vth0_stress_dn3, locals.var_vth0_stress_dn4, locals.var_vth0_stress_dn5, locals.var_vth0_stress_dn6, locals.var_vth0_stress_dn7, locals.var_vth0_stress_dn8, locals.var_vth0_stress_dn9, locals.var_vth0_stress_dn10, locals.var_vth0_stress_dn11, locals.var_vth0_stress_dn12, locals.var_vth0_stress_dn13, locals.var_vth0_stress_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_vth0_stress_rv = 0.0;
            (locals.var_vth0_stress_edge, locals.var_vth0_stress_edge_dn0, locals.var_vth0_stress_edge_dn2, locals.var_vth0_stress_edge_dn3, locals.var_vth0_stress_edge_dn4, locals.var_vth0_stress_edge_dn5, locals.var_vth0_stress_edge_dn6, locals.var_vth0_stress_edge_dn7, locals.var_vth0_stress_edge_dn8, locals.var_vth0_stress_edge_dn9, locals.var_vth0_stress_edge_dn10, locals.var_vth0_stress_edge_dn11, locals.var_vth0_stress_edge_dn12, locals.var_vth0_stress_edge_dn13, locals.var_vth0_stress_edge_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_vth0_stress_edge_rv = 0.0;
        }

        let assign16010_e23375: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard490 = assign16010_e23375;
        locals.var_guard490_rv = 0.0;

        if (locals.var_guard490 != 0.0) {
            let assign16020_e23379: f64 = (p.p1 / p.p2);
            locals.var_wdrn = assign16020_e23379;
            locals.var_wdrn_rv = 0.0;
        }

        if (locals.var_guard490 != 0.0) {
            (locals.var_local_sca, locals.var_local_sca_dn0, locals.var_local_sca_dn2, locals.var_local_sca_dn3, locals.var_local_sca_dn4, locals.var_local_sca_dn5, locals.var_local_sca_dn6, locals.var_local_sca_dn7, locals.var_local_sca_dn8, locals.var_local_sca_dn9, locals.var_local_sca_dn10, locals.var_local_sca_dn11, locals.var_local_sca_dn12, locals.var_local_sca_dn13, locals.var_local_sca_dn14, ) = (p.p20, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_local_sca_rv = 0.0;
            (locals.var_local_scb, locals.var_local_scb_dn0, locals.var_local_scb_dn2, locals.var_local_scb_dn3, locals.var_local_scb_dn4, locals.var_local_scb_dn5, locals.var_local_scb_dn6, locals.var_local_scb_dn7, locals.var_local_scb_dn8, locals.var_local_scb_dn9, locals.var_local_scb_dn10, locals.var_local_scb_dn11, locals.var_local_scb_dn12, locals.var_local_scb_dn13, locals.var_local_scb_dn14, ) = (p.p21, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_local_scb_rv = 0.0;
            (locals.var_local_scc, locals.var_local_scc_dn0, locals.var_local_scc_dn2, locals.var_local_scc_dn3, locals.var_local_scc_dn4, locals.var_local_scc_dn5, locals.var_local_scc_dn6, locals.var_local_scc_dn7, locals.var_local_scc_dn8, locals.var_local_scc_dn9, locals.var_local_scc_dn10, locals.var_local_scc_dn11, locals.var_local_scc_dn12, locals.var_local_scc_dn13, locals.var_local_scc_dn14, ) = (p.p22, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_local_scc_rv = 0.0;
        }

        let assign16060_e23404: f64 = if (((!param_given[20]) && (!param_given[21])) && (!param_given[22])) { 1.0 } else { 0.0 };
        locals.var_guard491 = assign16060_e23404;
        locals.var_guard491_rv = 0.0;

        let assign16070_e23410: f64 = if (param_given[23] && (p.p23 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard492 = assign16070_e23410;
        locals.var_guard492_rv = 0.0;

        if (((locals.var_guard490 != 0.0) && (locals.var_guard491 != 0.0)) && (locals.var_guard492 != 0.0)) {
            let assign16080_e23418: f64 = (p.p23 + locals.var_wdrn);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, ) = (assign16080_e23418, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1_rv = 0.0;
        }

        if (((locals.var_guard490 != 0.0) && (locals.var_guard491 != 0.0)) && (locals.var_guard492 != 0.0)) {
            let assign16090_e23428: f64 = (1.0 / p.p947);
            (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14, ) = (assign16090_e23428, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t2_rv = 0.0;
        }

        if (((locals.var_guard490 != 0.0) && (locals.var_guard491 != 0.0)) && (locals.var_guard492 != 0.0)) {
            let assign16100_e23438: f64 = (p.p947 * p.p947);
            let assign16100_e23441: f64 = (p.p23 * locals.var_t1);
            let assign16100_e23442: f64 = (assign16100_e23438 / assign16100_e23441);
            (locals.var_local_sca, locals.var_local_sca_dn0, locals.var_local_sca_dn2, locals.var_local_sca_dn3, locals.var_local_sca_dn4, locals.var_local_sca_dn5, locals.var_local_sca_dn6, locals.var_local_sca_dn7, locals.var_local_sca_dn8, locals.var_local_sca_dn9, locals.var_local_sca_dn10, locals.var_local_sca_dn11, locals.var_local_sca_dn12, locals.var_local_sca_dn13, locals.var_local_sca_dn14, ) = (assign16100_e23442, (-((assign16100_e23438 * (p.p23 * locals.var_t1_dn0)) / (assign16100_e23441 * assign16100_e23441))), (-((assign16100_e23438 * (p.p23 * locals.var_t1_dn2)) / (assign16100_e23441 * assign16100_e23441))), (-((assign16100_e23438 * (p.p23 * locals.var_t1_dn3)) / (assign16100_e23441 * assign16100_e23441))), (-((assign16100_e23438 * (p.p23 * locals.var_t1_dn4)) / (assign16100_e23441 * assign16100_e23441))), (-((assign16100_e23438 * (p.p23 * locals.var_t1_dn5)) / (assign16100_e23441 * assign16100_e23441))), (-((assign16100_e23438 * (p.p23 * locals.var_t1_dn6)) / (assign16100_e23441 * assign16100_e23441))), (-((assign16100_e23438 * (p.p23 * locals.var_t1_dn7)) / (assign16100_e23441 * assign16100_e23441))), (-((assign16100_e23438 * (p.p23 * locals.var_t1_dn8)) / (assign16100_e23441 * assign16100_e23441))), (-((assign16100_e23438 * (p.p23 * locals.var_t1_dn9)) / (assign16100_e23441 * assign16100_e23441))), (-((assign16100_e23438 * (p.p23 * locals.var_t1_dn10)) / (assign16100_e23441 * assign16100_e23441))), (-((assign16100_e23438 * (p.p23 * locals.var_t1_dn11)) / (assign16100_e23441 * assign16100_e23441))), (-((assign16100_e23438 * (p.p23 * locals.var_t1_dn12)) / (assign16100_e23441 * assign16100_e23441))), (-((assign16100_e23438 * (p.p23 * locals.var_t1_dn13)) / (assign16100_e23441 * assign16100_e23441))), (-((assign16100_e23438 * (p.p23 * locals.var_t1_dn14)) / (assign16100_e23441 * assign16100_e23441))), );
            locals.var_local_sca_rv = 0.0;
        }

        if (((locals.var_guard490 != 0.0) && (locals.var_guard491 != 0.0)) && (locals.var_guard492 != 0.0)) {
            let assign16110_e23452: f64 = (0.1 * p.p23);
            let assign16110_e23455: f64 = (0.01 * p.p947);
            let assign16110_e23456: f64 = (assign16110_e23452 + assign16110_e23455);
            let assign16110_e23458: f64 = (-10.0);
            let assign16110_e23460: f64 = (assign16110_e23458 * p.p23);
            let assign16110_e23462: f64 = (assign16110_e23460 * locals.var_t2);
            let assign16110_e23463: f64 = { let limited_exp_arg = assign16110_e23462; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
            let assign16110_e23464: f64 = (assign16110_e23456 * assign16110_e23463);
            let assign16110_e23467: f64 = (0.1 * locals.var_t1);
            let assign16110_e23470: f64 = (0.01 * p.p947);
            let assign16110_e23471: f64 = (assign16110_e23467 + assign16110_e23470);
            let assign16110_e23473: f64 = (-10.0);
            let assign16110_e23475: f64 = (assign16110_e23473 * locals.var_t1);
            let assign16110_e23477: f64 = (assign16110_e23475 * locals.var_t2);
            let assign16110_e23478: f64 = { let limited_exp_arg = assign16110_e23477; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
            let assign16110_e23479: f64 = (assign16110_e23471 * assign16110_e23478);
            let assign16110_e23480: f64 = (assign16110_e23464 - assign16110_e23479);
            let assign16110_e23482: f64 = (assign16110_e23480 / locals.var_wdrn);
            (locals.var_local_scb, locals.var_local_scb_dn0, locals.var_local_scb_dn2, locals.var_local_scb_dn3, locals.var_local_scb_dn4, locals.var_local_scb_dn5, locals.var_local_scb_dn6, locals.var_local_scb_dn7, locals.var_local_scb_dn8, locals.var_local_scb_dn9, locals.var_local_scb_dn10, locals.var_local_scb_dn11, locals.var_local_scb_dn12, locals.var_local_scb_dn13, locals.var_local_scb_dn14, ) = (assign16110_e23482, (((assign16110_e23456 * ({ let limited_exp_arg = assign16110_e23462; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16110_e23460 * locals.var_t2_dn0))) - (((0.1 * locals.var_t1_dn0) * assign16110_e23478) + (assign16110_e23471 * ({ let limited_exp_arg = assign16110_e23477; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16110_e23473 * locals.var_t1_dn0) * locals.var_t2) + (assign16110_e23475 * locals.var_t2_dn0)))))) / locals.var_wdrn), (((assign16110_e23456 * ({ let limited_exp_arg = assign16110_e23462; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16110_e23460 * locals.var_t2_dn2))) - (((0.1 * locals.var_t1_dn2) * assign16110_e23478) + (assign16110_e23471 * ({ let limited_exp_arg = assign16110_e23477; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16110_e23473 * locals.var_t1_dn2) * locals.var_t2) + (assign16110_e23475 * locals.var_t2_dn2)))))) / locals.var_wdrn), (((assign16110_e23456 * ({ let limited_exp_arg = assign16110_e23462; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16110_e23460 * locals.var_t2_dn3))) - (((0.1 * locals.var_t1_dn3) * assign16110_e23478) + (assign16110_e23471 * ({ let limited_exp_arg = assign16110_e23477; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16110_e23473 * locals.var_t1_dn3) * locals.var_t2) + (assign16110_e23475 * locals.var_t2_dn3)))))) / locals.var_wdrn), (((assign16110_e23456 * ({ let limited_exp_arg = assign16110_e23462; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16110_e23460 * locals.var_t2_dn4))) - (((0.1 * locals.var_t1_dn4) * assign16110_e23478) + (assign16110_e23471 * ({ let limited_exp_arg = assign16110_e23477; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16110_e23473 * locals.var_t1_dn4) * locals.var_t2) + (assign16110_e23475 * locals.var_t2_dn4)))))) / locals.var_wdrn), (((assign16110_e23456 * ({ let limited_exp_arg = assign16110_e23462; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16110_e23460 * locals.var_t2_dn5))) - (((0.1 * locals.var_t1_dn5) * assign16110_e23478) + (assign16110_e23471 * ({ let limited_exp_arg = assign16110_e23477; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16110_e23473 * locals.var_t1_dn5) * locals.var_t2) + (assign16110_e23475 * locals.var_t2_dn5)))))) / locals.var_wdrn), (((assign16110_e23456 * ({ let limited_exp_arg = assign16110_e23462; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16110_e23460 * locals.var_t2_dn6))) - (((0.1 * locals.var_t1_dn6) * assign16110_e23478) + (assign16110_e23471 * ({ let limited_exp_arg = assign16110_e23477; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16110_e23473 * locals.var_t1_dn6) * locals.var_t2) + (assign16110_e23475 * locals.var_t2_dn6)))))) / locals.var_wdrn), (((assign16110_e23456 * ({ let limited_exp_arg = assign16110_e23462; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16110_e23460 * locals.var_t2_dn7))) - (((0.1 * locals.var_t1_dn7) * assign16110_e23478) + (assign16110_e23471 * ({ let limited_exp_arg = assign16110_e23477; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16110_e23473 * locals.var_t1_dn7) * locals.var_t2) + (assign16110_e23475 * locals.var_t2_dn7)))))) / locals.var_wdrn), (((assign16110_e23456 * ({ let limited_exp_arg = assign16110_e23462; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16110_e23460 * locals.var_t2_dn8))) - (((0.1 * locals.var_t1_dn8) * assign16110_e23478) + (assign16110_e23471 * ({ let limited_exp_arg = assign16110_e23477; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16110_e23473 * locals.var_t1_dn8) * locals.var_t2) + (assign16110_e23475 * locals.var_t2_dn8)))))) / locals.var_wdrn), (((assign16110_e23456 * ({ let limited_exp_arg = assign16110_e23462; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16110_e23460 * locals.var_t2_dn9))) - (((0.1 * locals.var_t1_dn9) * assign16110_e23478) + (assign16110_e23471 * ({ let limited_exp_arg = assign16110_e23477; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16110_e23473 * locals.var_t1_dn9) * locals.var_t2) + (assign16110_e23475 * locals.var_t2_dn9)))))) / locals.var_wdrn), (((assign16110_e23456 * ({ let limited_exp_arg = assign16110_e23462; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16110_e23460 * locals.var_t2_dn10))) - (((0.1 * locals.var_t1_dn10) * assign16110_e23478) + (assign16110_e23471 * ({ let limited_exp_arg = assign16110_e23477; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16110_e23473 * locals.var_t1_dn10) * locals.var_t2) + (assign16110_e23475 * locals.var_t2_dn10)))))) / locals.var_wdrn), (((assign16110_e23456 * ({ let limited_exp_arg = assign16110_e23462; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16110_e23460 * locals.var_t2_dn11))) - (((0.1 * locals.var_t1_dn11) * assign16110_e23478) + (assign16110_e23471 * ({ let limited_exp_arg = assign16110_e23477; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16110_e23473 * locals.var_t1_dn11) * locals.var_t2) + (assign16110_e23475 * locals.var_t2_dn11)))))) / locals.var_wdrn), (((assign16110_e23456 * ({ let limited_exp_arg = assign16110_e23462; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16110_e23460 * locals.var_t2_dn12))) - (((0.1 * locals.var_t1_dn12) * assign16110_e23478) + (assign16110_e23471 * ({ let limited_exp_arg = assign16110_e23477; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16110_e23473 * locals.var_t1_dn12) * locals.var_t2) + (assign16110_e23475 * locals.var_t2_dn12)))))) / locals.var_wdrn), (((assign16110_e23456 * ({ let limited_exp_arg = assign16110_e23462; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16110_e23460 * locals.var_t2_dn13))) - (((0.1 * locals.var_t1_dn13) * assign16110_e23478) + (assign16110_e23471 * ({ let limited_exp_arg = assign16110_e23477; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16110_e23473 * locals.var_t1_dn13) * locals.var_t2) + (assign16110_e23475 * locals.var_t2_dn13)))))) / locals.var_wdrn), (((assign16110_e23456 * ({ let limited_exp_arg = assign16110_e23462; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16110_e23460 * locals.var_t2_dn14))) - (((0.1 * locals.var_t1_dn14) * assign16110_e23478) + (assign16110_e23471 * ({ let limited_exp_arg = assign16110_e23477; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16110_e23473 * locals.var_t1_dn14) * locals.var_t2) + (assign16110_e23475 * locals.var_t2_dn14)))))) / locals.var_wdrn), );
            locals.var_local_scb_rv = 0.0;
        }

        if (((locals.var_guard490 != 0.0) && (locals.var_guard491 != 0.0)) && (locals.var_guard492 != 0.0)) {
            let assign16120_e23492: f64 = (0.05 * p.p23);
            let assign16120_e23495: f64 = (0.0025 * p.p947);
            let assign16120_e23496: f64 = (assign16120_e23492 + assign16120_e23495);
            let assign16120_e23498: f64 = (-20.0);
            let assign16120_e23500: f64 = (assign16120_e23498 * p.p23);
            let assign16120_e23502: f64 = (assign16120_e23500 * locals.var_t2);
            let assign16120_e23503: f64 = { let limited_exp_arg = assign16120_e23502; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
            let assign16120_e23504: f64 = (assign16120_e23496 * assign16120_e23503);
            let assign16120_e23507: f64 = (0.05 * locals.var_t1);
            let assign16120_e23510: f64 = (0.0025 * p.p947);
            let assign16120_e23511: f64 = (assign16120_e23507 + assign16120_e23510);
            let assign16120_e23513: f64 = (-20.0);
            let assign16120_e23515: f64 = (assign16120_e23513 * locals.var_t1);
            let assign16120_e23517: f64 = (assign16120_e23515 * locals.var_t2);
            let assign16120_e23518: f64 = { let limited_exp_arg = assign16120_e23517; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
            let assign16120_e23519: f64 = (assign16120_e23511 * assign16120_e23518);
            let assign16120_e23520: f64 = (assign16120_e23504 - assign16120_e23519);
            let assign16120_e23522: f64 = (assign16120_e23520 / locals.var_wdrn);
            (locals.var_local_scc, locals.var_local_scc_dn0, locals.var_local_scc_dn2, locals.var_local_scc_dn3, locals.var_local_scc_dn4, locals.var_local_scc_dn5, locals.var_local_scc_dn6, locals.var_local_scc_dn7, locals.var_local_scc_dn8, locals.var_local_scc_dn9, locals.var_local_scc_dn10, locals.var_local_scc_dn11, locals.var_local_scc_dn12, locals.var_local_scc_dn13, locals.var_local_scc_dn14, ) = (assign16120_e23522, (((assign16120_e23496 * ({ let limited_exp_arg = assign16120_e23502; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16120_e23500 * locals.var_t2_dn0))) - (((0.05 * locals.var_t1_dn0) * assign16120_e23518) + (assign16120_e23511 * ({ let limited_exp_arg = assign16120_e23517; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16120_e23513 * locals.var_t1_dn0) * locals.var_t2) + (assign16120_e23515 * locals.var_t2_dn0)))))) / locals.var_wdrn), (((assign16120_e23496 * ({ let limited_exp_arg = assign16120_e23502; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16120_e23500 * locals.var_t2_dn2))) - (((0.05 * locals.var_t1_dn2) * assign16120_e23518) + (assign16120_e23511 * ({ let limited_exp_arg = assign16120_e23517; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16120_e23513 * locals.var_t1_dn2) * locals.var_t2) + (assign16120_e23515 * locals.var_t2_dn2)))))) / locals.var_wdrn), (((assign16120_e23496 * ({ let limited_exp_arg = assign16120_e23502; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16120_e23500 * locals.var_t2_dn3))) - (((0.05 * locals.var_t1_dn3) * assign16120_e23518) + (assign16120_e23511 * ({ let limited_exp_arg = assign16120_e23517; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16120_e23513 * locals.var_t1_dn3) * locals.var_t2) + (assign16120_e23515 * locals.var_t2_dn3)))))) / locals.var_wdrn), (((assign16120_e23496 * ({ let limited_exp_arg = assign16120_e23502; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16120_e23500 * locals.var_t2_dn4))) - (((0.05 * locals.var_t1_dn4) * assign16120_e23518) + (assign16120_e23511 * ({ let limited_exp_arg = assign16120_e23517; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16120_e23513 * locals.var_t1_dn4) * locals.var_t2) + (assign16120_e23515 * locals.var_t2_dn4)))))) / locals.var_wdrn), (((assign16120_e23496 * ({ let limited_exp_arg = assign16120_e23502; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16120_e23500 * locals.var_t2_dn5))) - (((0.05 * locals.var_t1_dn5) * assign16120_e23518) + (assign16120_e23511 * ({ let limited_exp_arg = assign16120_e23517; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16120_e23513 * locals.var_t1_dn5) * locals.var_t2) + (assign16120_e23515 * locals.var_t2_dn5)))))) / locals.var_wdrn), (((assign16120_e23496 * ({ let limited_exp_arg = assign16120_e23502; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16120_e23500 * locals.var_t2_dn6))) - (((0.05 * locals.var_t1_dn6) * assign16120_e23518) + (assign16120_e23511 * ({ let limited_exp_arg = assign16120_e23517; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16120_e23513 * locals.var_t1_dn6) * locals.var_t2) + (assign16120_e23515 * locals.var_t2_dn6)))))) / locals.var_wdrn), (((assign16120_e23496 * ({ let limited_exp_arg = assign16120_e23502; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16120_e23500 * locals.var_t2_dn7))) - (((0.05 * locals.var_t1_dn7) * assign16120_e23518) + (assign16120_e23511 * ({ let limited_exp_arg = assign16120_e23517; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16120_e23513 * locals.var_t1_dn7) * locals.var_t2) + (assign16120_e23515 * locals.var_t2_dn7)))))) / locals.var_wdrn), (((assign16120_e23496 * ({ let limited_exp_arg = assign16120_e23502; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16120_e23500 * locals.var_t2_dn8))) - (((0.05 * locals.var_t1_dn8) * assign16120_e23518) + (assign16120_e23511 * ({ let limited_exp_arg = assign16120_e23517; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16120_e23513 * locals.var_t1_dn8) * locals.var_t2) + (assign16120_e23515 * locals.var_t2_dn8)))))) / locals.var_wdrn), (((assign16120_e23496 * ({ let limited_exp_arg = assign16120_e23502; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16120_e23500 * locals.var_t2_dn9))) - (((0.05 * locals.var_t1_dn9) * assign16120_e23518) + (assign16120_e23511 * ({ let limited_exp_arg = assign16120_e23517; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16120_e23513 * locals.var_t1_dn9) * locals.var_t2) + (assign16120_e23515 * locals.var_t2_dn9)))))) / locals.var_wdrn), (((assign16120_e23496 * ({ let limited_exp_arg = assign16120_e23502; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16120_e23500 * locals.var_t2_dn10))) - (((0.05 * locals.var_t1_dn10) * assign16120_e23518) + (assign16120_e23511 * ({ let limited_exp_arg = assign16120_e23517; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16120_e23513 * locals.var_t1_dn10) * locals.var_t2) + (assign16120_e23515 * locals.var_t2_dn10)))))) / locals.var_wdrn), (((assign16120_e23496 * ({ let limited_exp_arg = assign16120_e23502; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16120_e23500 * locals.var_t2_dn11))) - (((0.05 * locals.var_t1_dn11) * assign16120_e23518) + (assign16120_e23511 * ({ let limited_exp_arg = assign16120_e23517; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16120_e23513 * locals.var_t1_dn11) * locals.var_t2) + (assign16120_e23515 * locals.var_t2_dn11)))))) / locals.var_wdrn), (((assign16120_e23496 * ({ let limited_exp_arg = assign16120_e23502; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16120_e23500 * locals.var_t2_dn12))) - (((0.05 * locals.var_t1_dn12) * assign16120_e23518) + (assign16120_e23511 * ({ let limited_exp_arg = assign16120_e23517; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16120_e23513 * locals.var_t1_dn12) * locals.var_t2) + (assign16120_e23515 * locals.var_t2_dn12)))))) / locals.var_wdrn), (((assign16120_e23496 * ({ let limited_exp_arg = assign16120_e23502; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16120_e23500 * locals.var_t2_dn13))) - (((0.05 * locals.var_t1_dn13) * assign16120_e23518) + (assign16120_e23511 * ({ let limited_exp_arg = assign16120_e23517; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16120_e23513 * locals.var_t1_dn13) * locals.var_t2) + (assign16120_e23515 * locals.var_t2_dn13)))))) / locals.var_wdrn), (((assign16120_e23496 * ({ let limited_exp_arg = assign16120_e23502; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16120_e23500 * locals.var_t2_dn14))) - (((0.05 * locals.var_t1_dn14) * assign16120_e23518) + (assign16120_e23511 * ({ let limited_exp_arg = assign16120_e23517; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16120_e23513 * locals.var_t1_dn14) * locals.var_t2) + (assign16120_e23515 * locals.var_t2_dn14)))))) / locals.var_wdrn), );
            locals.var_local_scc_rv = 0.0;
        }

        let assign16130_e23529: f64 = (p.p933 * locals.var_local_scb);
        let assign16130_e23530: f64 = (locals.var_local_sca + assign16130_e23529);
        let assign16130_e23533: f64 = (p.p934 * locals.var_local_scc);
        let assign16130_e23534: f64 = (assign16130_e23530 + assign16130_e23533);
        let assign16130_e23535: f64 = (locals.var_kvth0we_i * assign16130_e23534);
        (locals.var_vth0_well, locals.var_vth0_well_dn0, locals.var_vth0_well_dn2, locals.var_vth0_well_dn3, locals.var_vth0_well_dn4, locals.var_vth0_well_dn5, locals.var_vth0_well_dn6, locals.var_vth0_well_dn7, locals.var_vth0_well_dn8, locals.var_vth0_well_dn9, locals.var_vth0_well_dn10, locals.var_vth0_well_dn11, locals.var_vth0_well_dn12, locals.var_vth0_well_dn13, locals.var_vth0_well_dn14, ) = (assign16130_e23535, (locals.var_kvth0we_i * ((locals.var_local_sca_dn0 + (p.p933 * locals.var_local_scb_dn0)) + (p.p934 * locals.var_local_scc_dn0))), (locals.var_kvth0we_i * ((locals.var_local_sca_dn2 + (p.p933 * locals.var_local_scb_dn2)) + (p.p934 * locals.var_local_scc_dn2))), (locals.var_kvth0we_i * ((locals.var_local_sca_dn3 + (p.p933 * locals.var_local_scb_dn3)) + (p.p934 * locals.var_local_scc_dn3))), (locals.var_kvth0we_i * ((locals.var_local_sca_dn4 + (p.p933 * locals.var_local_scb_dn4)) + (p.p934 * locals.var_local_scc_dn4))), (locals.var_kvth0we_i * ((locals.var_local_sca_dn5 + (p.p933 * locals.var_local_scb_dn5)) + (p.p934 * locals.var_local_scc_dn5))), (locals.var_kvth0we_i * ((locals.var_local_sca_dn6 + (p.p933 * locals.var_local_scb_dn6)) + (p.p934 * locals.var_local_scc_dn6))), (locals.var_kvth0we_i * ((locals.var_local_sca_dn7 + (p.p933 * locals.var_local_scb_dn7)) + (p.p934 * locals.var_local_scc_dn7))), (locals.var_kvth0we_i * ((locals.var_local_sca_dn8 + (p.p933 * locals.var_local_scb_dn8)) + (p.p934 * locals.var_local_scc_dn8))), (locals.var_kvth0we_i * ((locals.var_local_sca_dn9 + (p.p933 * locals.var_local_scb_dn9)) + (p.p934 * locals.var_local_scc_dn9))), (locals.var_kvth0we_i * ((locals.var_local_sca_dn10 + (p.p933 * locals.var_local_scb_dn10)) + (p.p934 * locals.var_local_scc_dn10))), (locals.var_kvth0we_i * ((locals.var_local_sca_dn11 + (p.p933 * locals.var_local_scb_dn11)) + (p.p934 * locals.var_local_scc_dn11))), (locals.var_kvth0we_i * ((locals.var_local_sca_dn12 + (p.p933 * locals.var_local_scb_dn12)) + (p.p934 * locals.var_local_scc_dn12))), (locals.var_kvth0we_i * ((locals.var_local_sca_dn13 + (p.p933 * locals.var_local_scb_dn13)) + (p.p934 * locals.var_local_scc_dn13))), (locals.var_kvth0we_i * ((locals.var_local_sca_dn14 + (p.p933 * locals.var_local_scb_dn14)) + (p.p934 * locals.var_local_scc_dn14))), );
        locals.var_vth0_well_rv = 0.0;

        let assign16140_e23540: f64 = (p.p933 * locals.var_local_scb);
        let assign16140_e23541: f64 = (locals.var_local_sca + assign16140_e23540);
        let assign16140_e23544: f64 = (p.p934 * locals.var_local_scc);
        let assign16140_e23545: f64 = (assign16140_e23541 + assign16140_e23544);
        let assign16140_e23546: f64 = (locals.var_k2we_i * assign16140_e23545);
        (locals.var_k2_well, locals.var_k2_well_dn0, locals.var_k2_well_dn2, locals.var_k2_well_dn3, locals.var_k2_well_dn4, locals.var_k2_well_dn5, locals.var_k2_well_dn6, locals.var_k2_well_dn7, locals.var_k2_well_dn8, locals.var_k2_well_dn9, locals.var_k2_well_dn10, locals.var_k2_well_dn11, locals.var_k2_well_dn12, locals.var_k2_well_dn13, locals.var_k2_well_dn14, ) = (assign16140_e23546, (locals.var_k2we_i * ((locals.var_local_sca_dn0 + (p.p933 * locals.var_local_scb_dn0)) + (p.p934 * locals.var_local_scc_dn0))), (locals.var_k2we_i * ((locals.var_local_sca_dn2 + (p.p933 * locals.var_local_scb_dn2)) + (p.p934 * locals.var_local_scc_dn2))), (locals.var_k2we_i * ((locals.var_local_sca_dn3 + (p.p933 * locals.var_local_scb_dn3)) + (p.p934 * locals.var_local_scc_dn3))), (locals.var_k2we_i * ((locals.var_local_sca_dn4 + (p.p933 * locals.var_local_scb_dn4)) + (p.p934 * locals.var_local_scc_dn4))), (locals.var_k2we_i * ((locals.var_local_sca_dn5 + (p.p933 * locals.var_local_scb_dn5)) + (p.p934 * locals.var_local_scc_dn5))), (locals.var_k2we_i * ((locals.var_local_sca_dn6 + (p.p933 * locals.var_local_scb_dn6)) + (p.p934 * locals.var_local_scc_dn6))), (locals.var_k2we_i * ((locals.var_local_sca_dn7 + (p.p933 * locals.var_local_scb_dn7)) + (p.p934 * locals.var_local_scc_dn7))), (locals.var_k2we_i * ((locals.var_local_sca_dn8 + (p.p933 * locals.var_local_scb_dn8)) + (p.p934 * locals.var_local_scc_dn8))), (locals.var_k2we_i * ((locals.var_local_sca_dn9 + (p.p933 * locals.var_local_scb_dn9)) + (p.p934 * locals.var_local_scc_dn9))), (locals.var_k2we_i * ((locals.var_local_sca_dn10 + (p.p933 * locals.var_local_scb_dn10)) + (p.p934 * locals.var_local_scc_dn10))), (locals.var_k2we_i * ((locals.var_local_sca_dn11 + (p.p933 * locals.var_local_scb_dn11)) + (p.p934 * locals.var_local_scc_dn11))), (locals.var_k2we_i * ((locals.var_local_sca_dn12 + (p.p933 * locals.var_local_scb_dn12)) + (p.p934 * locals.var_local_scc_dn12))), (locals.var_k2we_i * ((locals.var_local_sca_dn13 + (p.p933 * locals.var_local_scb_dn13)) + (p.p934 * locals.var_local_scc_dn13))), (locals.var_k2we_i * ((locals.var_local_sca_dn14 + (p.p933 * locals.var_local_scb_dn14)) + (p.p934 * locals.var_local_scc_dn14))), );
        locals.var_k2_well_rv = 0.0;

        let assign16150_e23551: f64 = (p.p933 * locals.var_local_scb);
        let assign16150_e23552: f64 = (locals.var_local_sca + assign16150_e23551);
        let assign16150_e23555: f64 = (p.p934 * locals.var_local_scc);
        let assign16150_e23556: f64 = (assign16150_e23552 + assign16150_e23555);
        let assign16150_e23557: f64 = (locals.var_kvth0edgewe_i * assign16150_e23556);
        (locals.var_vth0_well_edge, locals.var_vth0_well_edge_dn0, locals.var_vth0_well_edge_dn2, locals.var_vth0_well_edge_dn3, locals.var_vth0_well_edge_dn4, locals.var_vth0_well_edge_dn5, locals.var_vth0_well_edge_dn6, locals.var_vth0_well_edge_dn7, locals.var_vth0_well_edge_dn8, locals.var_vth0_well_edge_dn9, locals.var_vth0_well_edge_dn10, locals.var_vth0_well_edge_dn11, locals.var_vth0_well_edge_dn12, locals.var_vth0_well_edge_dn13, locals.var_vth0_well_edge_dn14, ) = (assign16150_e23557, (locals.var_kvth0edgewe_i * ((locals.var_local_sca_dn0 + (p.p933 * locals.var_local_scb_dn0)) + (p.p934 * locals.var_local_scc_dn0))), (locals.var_kvth0edgewe_i * ((locals.var_local_sca_dn2 + (p.p933 * locals.var_local_scb_dn2)) + (p.p934 * locals.var_local_scc_dn2))), (locals.var_kvth0edgewe_i * ((locals.var_local_sca_dn3 + (p.p933 * locals.var_local_scb_dn3)) + (p.p934 * locals.var_local_scc_dn3))), (locals.var_kvth0edgewe_i * ((locals.var_local_sca_dn4 + (p.p933 * locals.var_local_scb_dn4)) + (p.p934 * locals.var_local_scc_dn4))), (locals.var_kvth0edgewe_i * ((locals.var_local_sca_dn5 + (p.p933 * locals.var_local_scb_dn5)) + (p.p934 * locals.var_local_scc_dn5))), (locals.var_kvth0edgewe_i * ((locals.var_local_sca_dn6 + (p.p933 * locals.var_local_scb_dn6)) + (p.p934 * locals.var_local_scc_dn6))), (locals.var_kvth0edgewe_i * ((locals.var_local_sca_dn7 + (p.p933 * locals.var_local_scb_dn7)) + (p.p934 * locals.var_local_scc_dn7))), (locals.var_kvth0edgewe_i * ((locals.var_local_sca_dn8 + (p.p933 * locals.var_local_scb_dn8)) + (p.p934 * locals.var_local_scc_dn8))), (locals.var_kvth0edgewe_i * ((locals.var_local_sca_dn9 + (p.p933 * locals.var_local_scb_dn9)) + (p.p934 * locals.var_local_scc_dn9))), (locals.var_kvth0edgewe_i * ((locals.var_local_sca_dn10 + (p.p933 * locals.var_local_scb_dn10)) + (p.p934 * locals.var_local_scc_dn10))), (locals.var_kvth0edgewe_i * ((locals.var_local_sca_dn11 + (p.p933 * locals.var_local_scb_dn11)) + (p.p934 * locals.var_local_scc_dn11))), (locals.var_kvth0edgewe_i * ((locals.var_local_sca_dn12 + (p.p933 * locals.var_local_scb_dn12)) + (p.p934 * locals.var_local_scc_dn12))), (locals.var_kvth0edgewe_i * ((locals.var_local_sca_dn13 + (p.p933 * locals.var_local_scb_dn13)) + (p.p934 * locals.var_local_scc_dn13))), (locals.var_kvth0edgewe_i * ((locals.var_local_sca_dn14 + (p.p933 * locals.var_local_scb_dn14)) + (p.p934 * locals.var_local_scc_dn14))), );
        locals.var_vth0_well_edge_rv = 0.0;

        let assign16160_e23562: f64 = (p.p933 * locals.var_local_scb);
        let assign16160_e23563: f64 = (locals.var_local_sca + assign16160_e23562);
        let assign16160_e23566: f64 = (p.p934 * locals.var_local_scc);
        let assign16160_e23567: f64 = (assign16160_e23563 + assign16160_e23566);
        let assign16160_e23568: f64 = (locals.var_k2edgewe_i * assign16160_e23567);
        (locals.var_k2_well_edge, locals.var_k2_well_edge_dn0, locals.var_k2_well_edge_dn2, locals.var_k2_well_edge_dn3, locals.var_k2_well_edge_dn4, locals.var_k2_well_edge_dn5, locals.var_k2_well_edge_dn6, locals.var_k2_well_edge_dn7, locals.var_k2_well_edge_dn8, locals.var_k2_well_edge_dn9, locals.var_k2_well_edge_dn10, locals.var_k2_well_edge_dn11, locals.var_k2_well_edge_dn12, locals.var_k2_well_edge_dn13, locals.var_k2_well_edge_dn14, ) = (assign16160_e23568, (locals.var_k2edgewe_i * ((locals.var_local_sca_dn0 + (p.p933 * locals.var_local_scb_dn0)) + (p.p934 * locals.var_local_scc_dn0))), (locals.var_k2edgewe_i * ((locals.var_local_sca_dn2 + (p.p933 * locals.var_local_scb_dn2)) + (p.p934 * locals.var_local_scc_dn2))), (locals.var_k2edgewe_i * ((locals.var_local_sca_dn3 + (p.p933 * locals.var_local_scb_dn3)) + (p.p934 * locals.var_local_scc_dn3))), (locals.var_k2edgewe_i * ((locals.var_local_sca_dn4 + (p.p933 * locals.var_local_scb_dn4)) + (p.p934 * locals.var_local_scc_dn4))), (locals.var_k2edgewe_i * ((locals.var_local_sca_dn5 + (p.p933 * locals.var_local_scb_dn5)) + (p.p934 * locals.var_local_scc_dn5))), (locals.var_k2edgewe_i * ((locals.var_local_sca_dn6 + (p.p933 * locals.var_local_scb_dn6)) + (p.p934 * locals.var_local_scc_dn6))), (locals.var_k2edgewe_i * ((locals.var_local_sca_dn7 + (p.p933 * locals.var_local_scb_dn7)) + (p.p934 * locals.var_local_scc_dn7))), (locals.var_k2edgewe_i * ((locals.var_local_sca_dn8 + (p.p933 * locals.var_local_scb_dn8)) + (p.p934 * locals.var_local_scc_dn8))), (locals.var_k2edgewe_i * ((locals.var_local_sca_dn9 + (p.p933 * locals.var_local_scb_dn9)) + (p.p934 * locals.var_local_scc_dn9))), (locals.var_k2edgewe_i * ((locals.var_local_sca_dn10 + (p.p933 * locals.var_local_scb_dn10)) + (p.p934 * locals.var_local_scc_dn10))), (locals.var_k2edgewe_i * ((locals.var_local_sca_dn11 + (p.p933 * locals.var_local_scb_dn11)) + (p.p934 * locals.var_local_scc_dn11))), (locals.var_k2edgewe_i * ((locals.var_local_sca_dn12 + (p.p933 * locals.var_local_scb_dn12)) + (p.p934 * locals.var_local_scc_dn12))), (locals.var_k2edgewe_i * ((locals.var_local_sca_dn13 + (p.p933 * locals.var_local_scb_dn13)) + (p.p934 * locals.var_local_scc_dn13))), (locals.var_k2edgewe_i * ((locals.var_local_sca_dn14 + (p.p933 * locals.var_local_scb_dn14)) + (p.p934 * locals.var_local_scc_dn14))), );
        locals.var_k2_well_edge_rv = 0.0;

        let assign16170_e23574: f64 = (p.p933 * locals.var_local_scb);
        let assign16170_e23575: f64 = (locals.var_local_sca + assign16170_e23574);
        let assign16170_e23578: f64 = (p.p934 * locals.var_local_scc);
        let assign16170_e23579: f64 = (assign16170_e23575 + assign16170_e23578);
        let assign16170_e23580: f64 = (locals.var_ku0we_i * assign16170_e23579);
        let assign16170_e23581: f64 = (1.0 + assign16170_e23580);
        (locals.var_mu_well, locals.var_mu_well_dn0, locals.var_mu_well_dn2, locals.var_mu_well_dn3, locals.var_mu_well_dn4, locals.var_mu_well_dn5, locals.var_mu_well_dn6, locals.var_mu_well_dn7, locals.var_mu_well_dn8, locals.var_mu_well_dn9, locals.var_mu_well_dn10, locals.var_mu_well_dn11, locals.var_mu_well_dn12, locals.var_mu_well_dn13, locals.var_mu_well_dn14, ) = (assign16170_e23581, (locals.var_ku0we_i * ((locals.var_local_sca_dn0 + (p.p933 * locals.var_local_scb_dn0)) + (p.p934 * locals.var_local_scc_dn0))), (locals.var_ku0we_i * ((locals.var_local_sca_dn2 + (p.p933 * locals.var_local_scb_dn2)) + (p.p934 * locals.var_local_scc_dn2))), (locals.var_ku0we_i * ((locals.var_local_sca_dn3 + (p.p933 * locals.var_local_scb_dn3)) + (p.p934 * locals.var_local_scc_dn3))), (locals.var_ku0we_i * ((locals.var_local_sca_dn4 + (p.p933 * locals.var_local_scb_dn4)) + (p.p934 * locals.var_local_scc_dn4))), (locals.var_ku0we_i * ((locals.var_local_sca_dn5 + (p.p933 * locals.var_local_scb_dn5)) + (p.p934 * locals.var_local_scc_dn5))), (locals.var_ku0we_i * ((locals.var_local_sca_dn6 + (p.p933 * locals.var_local_scb_dn6)) + (p.p934 * locals.var_local_scc_dn6))), (locals.var_ku0we_i * ((locals.var_local_sca_dn7 + (p.p933 * locals.var_local_scb_dn7)) + (p.p934 * locals.var_local_scc_dn7))), (locals.var_ku0we_i * ((locals.var_local_sca_dn8 + (p.p933 * locals.var_local_scb_dn8)) + (p.p934 * locals.var_local_scc_dn8))), (locals.var_ku0we_i * ((locals.var_local_sca_dn9 + (p.p933 * locals.var_local_scb_dn9)) + (p.p934 * locals.var_local_scc_dn9))), (locals.var_ku0we_i * ((locals.var_local_sca_dn10 + (p.p933 * locals.var_local_scb_dn10)) + (p.p934 * locals.var_local_scc_dn10))), (locals.var_ku0we_i * ((locals.var_local_sca_dn11 + (p.p933 * locals.var_local_scb_dn11)) + (p.p934 * locals.var_local_scc_dn11))), (locals.var_ku0we_i * ((locals.var_local_sca_dn12 + (p.p933 * locals.var_local_scb_dn12)) + (p.p934 * locals.var_local_scc_dn12))), (locals.var_ku0we_i * ((locals.var_local_sca_dn13 + (p.p933 * locals.var_local_scb_dn13)) + (p.p934 * locals.var_local_scc_dn13))), (locals.var_ku0we_i * ((locals.var_local_sca_dn14 + (p.p933 * locals.var_local_scb_dn14)) + (p.p934 * locals.var_local_scc_dn14))), );
        locals.var_mu_well_rv = 0.0;

        let assign16180_e23584: f64 = (locals.var_u0_t * locals.var_mu_well);
        (locals.var_u0_t, locals.var_u0_t_dn0, locals.var_u0_t_dn2, locals.var_u0_t_dn3, locals.var_u0_t_dn4, locals.var_u0_t_dn5, locals.var_u0_t_dn6, locals.var_u0_t_dn7, locals.var_u0_t_dn8, locals.var_u0_t_dn9, locals.var_u0_t_dn10, locals.var_u0_t_dn11, locals.var_u0_t_dn12, locals.var_u0_t_dn13, locals.var_u0_t_dn14, ) = (assign16180_e23584, ((locals.var_u0_t_dn0 * locals.var_mu_well) + (locals.var_u0_t * locals.var_mu_well_dn0)), ((locals.var_u0_t_dn2 * locals.var_mu_well) + (locals.var_u0_t * locals.var_mu_well_dn2)), ((locals.var_u0_t_dn3 * locals.var_mu_well) + (locals.var_u0_t * locals.var_mu_well_dn3)), ((locals.var_u0_t_dn4 * locals.var_mu_well) + (locals.var_u0_t * locals.var_mu_well_dn4)), ((locals.var_u0_t_dn5 * locals.var_mu_well) + (locals.var_u0_t * locals.var_mu_well_dn5)), ((locals.var_u0_t_dn6 * locals.var_mu_well) + (locals.var_u0_t * locals.var_mu_well_dn6)), ((locals.var_u0_t_dn7 * locals.var_mu_well) + (locals.var_u0_t * locals.var_mu_well_dn7)), ((locals.var_u0_t_dn8 * locals.var_mu_well) + (locals.var_u0_t * locals.var_mu_well_dn8)), ((locals.var_u0_t_dn9 * locals.var_mu_well) + (locals.var_u0_t * locals.var_mu_well_dn9)), ((locals.var_u0_t_dn10 * locals.var_mu_well) + (locals.var_u0_t * locals.var_mu_well_dn10)), ((locals.var_u0_t_dn11 * locals.var_mu_well) + (locals.var_u0_t * locals.var_mu_well_dn11)), ((locals.var_u0_t_dn12 * locals.var_mu_well) + (locals.var_u0_t * locals.var_mu_well_dn12)), ((locals.var_u0_t_dn13 * locals.var_mu_well) + (locals.var_u0_t * locals.var_mu_well_dn13)), ((locals.var_u0_t_dn14 * locals.var_mu_well) + (locals.var_u0_t * locals.var_mu_well_dn14)), );
        locals.var_u0_t_rv = 0.0;

        let assign16190_e23587: f64 = (locals.var_k2_i + locals.var_k2_well);
        (locals.var_k2_i, locals.var_k2_i_dn0, locals.var_k2_i_dn2, locals.var_k2_i_dn3, locals.var_k2_i_dn4, locals.var_k2_i_dn5, locals.var_k2_i_dn6, locals.var_k2_i_dn7, locals.var_k2_i_dn8, locals.var_k2_i_dn9, locals.var_k2_i_dn10, locals.var_k2_i_dn11, locals.var_k2_i_dn12, locals.var_k2_i_dn13, locals.var_k2_i_dn14, ) = (assign16190_e23587, (locals.var_k2_i_dn0 + locals.var_k2_well_dn0), (locals.var_k2_i_dn2 + locals.var_k2_well_dn2), (locals.var_k2_i_dn3 + locals.var_k2_well_dn3), (locals.var_k2_i_dn4 + locals.var_k2_well_dn4), (locals.var_k2_i_dn5 + locals.var_k2_well_dn5), (locals.var_k2_i_dn6 + locals.var_k2_well_dn6), (locals.var_k2_i_dn7 + locals.var_k2_well_dn7), (locals.var_k2_i_dn8 + locals.var_k2_well_dn8), (locals.var_k2_i_dn9 + locals.var_k2_well_dn9), (locals.var_k2_i_dn10 + locals.var_k2_well_dn10), (locals.var_k2_i_dn11 + locals.var_k2_well_dn11), (locals.var_k2_i_dn12 + locals.var_k2_well_dn12), (locals.var_k2_i_dn13 + locals.var_k2_well_dn13), (locals.var_k2_i_dn14 + locals.var_k2_well_dn14), );
        locals.var_k2_i_rv = 0.0;

        let assign16200_e23590: f64 = (locals.var_devsign * (nv9 - nv11));
        (locals.var_vg, locals.var_vg_dn9, locals.var_vg_dn11, ) = (assign16200_e23590, locals.var_devsign, (-locals.var_devsign), );
        locals.var_vg_rv = 0.0;

        let assign16210_e23593: f64 = (locals.var_devsign * (nv5 - nv11));
        (locals.var_vd, locals.var_vd_dn5, locals.var_vd_dn7, locals.var_vd_dn11, ) = (assign16210_e23593, locals.var_devsign, 0.0, (-locals.var_devsign), );
        locals.var_vd_rv = 0.0;

        let assign16220_e23596: f64 = (locals.var_devsign * (nv7 - nv11));
        (locals.var_vs, locals.var_vs_dn5, locals.var_vs_dn7, locals.var_vs_dn11, ) = (assign16220_e23596, 0.0, locals.var_devsign, (-locals.var_devsign), );
        locals.var_vs_rv = 0.0;

        let assign16230_e23599: f64 = (locals.var_vd - locals.var_vs);
        (locals.var_vds, locals.var_vds_dn5, locals.var_vds_dn7, locals.var_vds_dn11, ) = (assign16230_e23599, (locals.var_vd_dn5 - locals.var_vs_dn5), (locals.var_vd_dn7 - locals.var_vs_dn7), (locals.var_vd_dn11 - locals.var_vs_dn11), );
        locals.var_vds_rv = 0.0;

        (locals.var_vdcv, locals.var_vdcv_dn5, locals.var_vdcv_dn6, locals.var_vdcv_dn7, locals.var_vdcv_dn11, ) = (locals.var_vd, locals.var_vd_dn5, 0.0, locals.var_vd_dn7, locals.var_vd_dn11, );
        locals.var_vdcv_rv = 0.0;

        (locals.var_vds_noswap, locals.var_vds_noswap_dn5, locals.var_vds_noswap_dn7, locals.var_vds_noswap_dn11, ) = (locals.var_vds, locals.var_vds_dn5, locals.var_vds_dn7, locals.var_vds_dn11, );
        locals.var_vds_noswap_rv = 0.0;

        (locals.var_vsb_noswap, locals.var_vsb_noswap_dn5, locals.var_vsb_noswap_dn7, locals.var_vsb_noswap_dn11, ) = (locals.var_vs, locals.var_vs_dn5, locals.var_vs_dn7, locals.var_vs_dn11, );
        locals.var_vsb_noswap_rv = 0.0;

        (locals.var_vdb_noswap, locals.var_vdb_noswap_dn5, locals.var_vdb_noswap_dn7, locals.var_vdb_noswap_dn11, ) = (locals.var_vd, locals.var_vd_dn5, locals.var_vd_dn7, locals.var_vd_dn11, );
        locals.var_vdb_noswap_rv = 0.0;

        let assign16280_e23606: f64 = (locals.var_devsign * (nv12 - nv7));
        (locals.var_vbs_jct, locals.var_vbs_jct_dn7, locals.var_vbs_jct_dn12, ) = (assign16280_e23606, (-locals.var_devsign), locals.var_devsign, );
        locals.var_vbs_jct_rv = 0.0;

        let assign16290_e23609: f64 = (locals.var_devsign * (nv13 - nv5));
        (locals.var_vbd_jct, locals.var_vbd_jct_dn5, locals.var_vbd_jct_dn13, ) = (assign16290_e23609, (-locals.var_devsign), locals.var_devsign, );
        locals.var_vbd_jct_rv = 0.0;

        let assign16300_e23612: f64 = (locals.var_devsign * (nv13 - nv5));
        (locals.var_vbd_jctcv, locals.var_vbd_jctcv_dn5, locals.var_vbd_jctcv_dn6, locals.var_vbd_jctcv_dn7, locals.var_vbd_jctcv_dn11, locals.var_vbd_jctcv_dn13, ) = (assign16300_e23612, (-locals.var_devsign), 0.0, 0.0, 0.0, locals.var_devsign, );
        locals.var_vbd_jctcv_rv = 0.0;

        let assign16310_e23615: f64 = (locals.var_devsign * (nv13 - nv14));
        (locals.var_vbd_ext, locals.var_vbd_ext_dn13, locals.var_vbd_ext_dn14, ) = (assign16310_e23615, locals.var_devsign, (-locals.var_devsign), );
        locals.var_vbd_ext_rv = 0.0;

        let assign16320_e23618: f64 = (locals.var_vg - locals.var_vd);
        (locals.var_vgd_noswap, locals.var_vgd_noswap_dn5, locals.var_vgd_noswap_dn7, locals.var_vgd_noswap_dn9, locals.var_vgd_noswap_dn11, ) = (assign16320_e23618, (-locals.var_vd_dn5), (-locals.var_vd_dn7), locals.var_vg_dn9, (locals.var_vg_dn11 - locals.var_vd_dn11), );
        locals.var_vgd_noswap_rv = 0.0;

        let assign16330_e23621: f64 = (locals.var_vg - locals.var_vs);
        (locals.var_vgs_noswap, locals.var_vgs_noswap_dn5, locals.var_vgs_noswap_dn7, locals.var_vgs_noswap_dn9, locals.var_vgs_noswap_dn11, ) = (assign16330_e23621, (-locals.var_vs_dn5), (-locals.var_vs_dn7), locals.var_vg_dn9, (locals.var_vg_dn11 - locals.var_vs_dn11), );
        locals.var_vgs_noswap_rv = 0.0;

        let assign16340_e23624: f64 = (locals.var_devsign * (nv10 - nv5));
        (locals.var_vgd_ov_noswap, locals.var_vgd_ov_noswap_dn5, locals.var_vgd_ov_noswap_dn10, ) = (assign16340_e23624, (-locals.var_devsign), locals.var_devsign, );
        locals.var_vgd_ov_noswap_rv = 0.0;

        let assign16350_e23627: f64 = (locals.var_devsign * (nv10 - nv7));
        (locals.var_vgs_ov_noswap, locals.var_vgs_ov_noswap_dn7, locals.var_vgs_ov_noswap_dn10, ) = (assign16350_e23627, (-locals.var_devsign), locals.var_devsign, );
        locals.var_vgs_ov_noswap_rv = 0.0;

        (locals.var_vgd_ov_noswapcv, locals.var_vgd_ov_noswapcv_dn5, locals.var_vgd_ov_noswapcv_dn6, locals.var_vgd_ov_noswapcv_dn7, locals.var_vgd_ov_noswapcv_dn10, locals.var_vgd_ov_noswapcv_dn11, ) = (locals.var_vgd_ov_noswap, locals.var_vgd_ov_noswap_dn5, 0.0, 0.0, locals.var_vgd_ov_noswap_dn10, 0.0, );
        locals.var_vgd_ov_noswapcv_rv = 0.0;

        let assign16370_e23643: f64 = if ((((p.p1110 != 0.0) && (p.p42 == 1.0)) && (p.p1095 == 1.0)) && (p.p1094 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard493 = assign16370_e23643;
        locals.var_guard493_rv = 0.0;

        if (locals.var_guard493 != 0.0) {
            let assign16380_e23650: f64 = (p.p1111 / p.p1110);
            let assign16380_e23651: f64 = (1.0 - assign16380_e23650);
            let assign16380_e23652: f64 = (locals.var_devsign * assign16380_e23651);
            let assign16380_e23654: f64 = (assign16380_e23652 * (nv6 - nv5));
            let assign16380_e23655: f64 = (locals.var_vd + assign16380_e23654);
            (locals.var_vdcv, locals.var_vdcv_dn5, locals.var_vdcv_dn6, locals.var_vdcv_dn7, locals.var_vdcv_dn11, ) = (assign16380_e23655, (locals.var_vd_dn5 + (-assign16380_e23652)), assign16380_e23652, locals.var_vd_dn7, locals.var_vd_dn11, );
            locals.var_vdcv_rv = 0.0;
        }

        if (locals.var_guard493 != 0.0) {
            let assign16390_e23661: f64 = (locals.var_vbd_jct + locals.var_vd);
            let assign16390_e23663: f64 = (assign16390_e23661 - locals.var_vdcv);
            (locals.var_vbd_jctcv, locals.var_vbd_jctcv_dn5, locals.var_vbd_jctcv_dn6, locals.var_vbd_jctcv_dn7, locals.var_vbd_jctcv_dn11, locals.var_vbd_jctcv_dn13, ) = (assign16390_e23663, ((locals.var_vbd_jct_dn5 + locals.var_vd_dn5) - locals.var_vdcv_dn5), (-locals.var_vdcv_dn6), (locals.var_vd_dn7 - locals.var_vdcv_dn7), (locals.var_vd_dn11 - locals.var_vdcv_dn11), locals.var_vbd_jct_dn13, );
            locals.var_vbd_jctcv_rv = 0.0;
        }

        if (locals.var_guard493 != 0.0) {
            let assign16400_e23669: f64 = (locals.var_vgd_ov_noswap + locals.var_vd);
            let assign16400_e23671: f64 = (assign16400_e23669 - locals.var_vdcv);
            (locals.var_vgd_ov_noswapcv, locals.var_vgd_ov_noswapcv_dn5, locals.var_vgd_ov_noswapcv_dn6, locals.var_vgd_ov_noswapcv_dn7, locals.var_vgd_ov_noswapcv_dn10, locals.var_vgd_ov_noswapcv_dn11, ) = (assign16400_e23671, ((locals.var_vgd_ov_noswap_dn5 + locals.var_vd_dn5) - locals.var_vdcv_dn5), (-locals.var_vdcv_dn6), (locals.var_vd_dn7 - locals.var_vdcv_dn7), locals.var_vgd_ov_noswap_dn10, (locals.var_vd_dn11 - locals.var_vdcv_dn11), );
            locals.var_vgd_ov_noswapcv_rv = 0.0;
        }

        (locals.var_vdcv_noswap, locals.var_vdcv_noswap_dn5, locals.var_vdcv_noswap_dn6, locals.var_vdcv_noswap_dn7, locals.var_vdcv_noswap_dn11, ) = (locals.var_vdcv, locals.var_vdcv_dn5, locals.var_vdcv_dn6, locals.var_vdcv_dn7, locals.var_vdcv_dn11, );
        locals.var_vdcv_noswap_rv = 0.0;

        let assign16420_e23677: f64 = (locals.var_devsign * (nv7 - nv11));
        (locals.var_vscv, locals.var_vscv_dn5, locals.var_vscv_dn6, locals.var_vscv_dn7, locals.var_vscv_dn11, ) = (assign16420_e23677, 0.0, 0.0, locals.var_devsign, (-locals.var_devsign), );
        locals.var_vscv_rv = 0.0;

        locals.var_sigvds = 1.0;
        locals.var_sigvds_rv = 0.0;

        let assign16440_e23681: f64 = if locals.var_vds < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard494 = assign16440_e23681;
        locals.var_guard494_rv = 0.0;

        if (locals.var_guard494 != 0.0) {
            let assign16450_e23684: f64 = (-1.0);
            locals.var_sigvds = assign16450_e23684;
            locals.var_sigvds_rv = 0.0;
        }

        if (locals.var_guard494 != 0.0) {
            let assign16460_e23690: f64 = (locals.var_devsign * (nv7 - nv11));
            (locals.var_vd, locals.var_vd_dn5, locals.var_vd_dn7, locals.var_vd_dn11, ) = (assign16460_e23690, 0.0, locals.var_devsign, (-locals.var_devsign), );
            locals.var_vd_rv = 0.0;
        }

        if (locals.var_guard494 != 0.0) {
            let assign16470_e23696: f64 = (locals.var_devsign * (nv5 - nv11));
            (locals.var_vs, locals.var_vs_dn5, locals.var_vs_dn7, locals.var_vs_dn11, ) = (assign16470_e23696, locals.var_devsign, 0.0, (-locals.var_devsign), );
            locals.var_vs_rv = 0.0;
        }

        if (locals.var_guard494 != 0.0) {
            (locals.var_vscv, locals.var_vscv_dn5, locals.var_vscv_dn6, locals.var_vscv_dn7, locals.var_vscv_dn11, ) = (locals.var_vdcv_noswap, locals.var_vdcv_noswap_dn5, locals.var_vdcv_noswap_dn6, locals.var_vdcv_noswap_dn7, locals.var_vdcv_noswap_dn11, );
            locals.var_vscv_rv = 0.0;
        }

        if (locals.var_guard494 != 0.0) {
            let assign16490_e23706: f64 = (locals.var_devsign * (nv7 - nv11));
            (locals.var_vdcv, locals.var_vdcv_dn5, locals.var_vdcv_dn6, locals.var_vdcv_dn7, locals.var_vdcv_dn11, ) = (assign16490_e23706, 0.0, 0.0, locals.var_devsign, (-locals.var_devsign), );
            locals.var_vdcv_rv = 0.0;
        }

        let assign16500_e23711: f64 = (locals.var_vd - locals.var_vs);
        (locals.var_vds, locals.var_vds_dn5, locals.var_vds_dn7, locals.var_vds_dn11, ) = (assign16500_e23711, (locals.var_vd_dn5 - locals.var_vs_dn5), (locals.var_vd_dn7 - locals.var_vs_dn7), (locals.var_vd_dn11 - locals.var_vs_dn11), );
        locals.var_vds_rv = 0.0;

        let assign16510_e23714: f64 = (locals.var_vdcv - locals.var_vscv);
        (locals.var_vdscv, locals.var_vdscv_dn5, locals.var_vdscv_dn6, locals.var_vdscv_dn7, locals.var_vdscv_dn11, ) = (assign16510_e23714, (locals.var_vdcv_dn5 - locals.var_vscv_dn5), (locals.var_vdcv_dn6 - locals.var_vscv_dn6), (locals.var_vdcv_dn7 - locals.var_vscv_dn7), (locals.var_vdcv_dn11 - locals.var_vscv_dn11), );
        locals.var_vdscv_rv = 0.0;

        let assign16520_e23717: f64 = (p.p956 * locals.var_vdscv);
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign16520_e23717, 0.0, 0.0, 0.0, 0.0, (p.p956 * locals.var_vdscv_dn5), (p.p956 * locals.var_vdscv_dn6), (p.p956 * locals.var_vdscv_dn7), 0.0, 0.0, 0.0, (p.p956 * locals.var_vdscv_dn11), 0.0, 0.0, 0.0, );
        locals.var_t0_rv = 0.0;

        let assign16530_e23724: f64 = (-37.0);
        let (assign16530_e23751, assign16530_e23751_d_n0, assign16530_e23751_d_n2, assign16530_e23751_d_n3, assign16530_e23751_d_n4, assign16530_e23751_d_n5, assign16530_e23751_d_n6, assign16530_e23751_d_n7, assign16530_e23751_d_n8, assign16530_e23751_d_n9, assign16530_e23751_d_n10, assign16530_e23751_d_n11, assign16530_e23751_d_n12, assign16530_e23751_d_n13, assign16530_e23751_d_n14,) = {
    if ((!(locals.var_t0 > 37.0)) && (!(locals.var_t0 < assign16530_e23724))) {
        let assign16530_e23730: f64 = (locals.var_t0).exp();
        let assign16530_e23731: f64 = (1.0 + assign16530_e23730);
        let assign16530_e23732: f64 = (assign16530_e23731).ln();
        (assign16530_e23732, ((assign16530_e23730 * locals.var_t0_dn0) / assign16530_e23731), ((assign16530_e23730 * locals.var_t0_dn2) / assign16530_e23731), ((assign16530_e23730 * locals.var_t0_dn3) / assign16530_e23731), ((assign16530_e23730 * locals.var_t0_dn4) / assign16530_e23731), ((assign16530_e23730 * locals.var_t0_dn5) / assign16530_e23731), ((assign16530_e23730 * locals.var_t0_dn6) / assign16530_e23731), ((assign16530_e23730 * locals.var_t0_dn7) / assign16530_e23731), ((assign16530_e23730 * locals.var_t0_dn8) / assign16530_e23731), ((assign16530_e23730 * locals.var_t0_dn9) / assign16530_e23731), ((assign16530_e23730 * locals.var_t0_dn10) / assign16530_e23731), ((assign16530_e23730 * locals.var_t0_dn11) / assign16530_e23731), ((assign16530_e23730 * locals.var_t0_dn12) / assign16530_e23731), ((assign16530_e23730 * locals.var_t0_dn13) / assign16530_e23731), ((assign16530_e23730 * locals.var_t0_dn14) / assign16530_e23731),)
    } else {
        let assign16530_e23739: f64 = (-37.0);
        let (assign16530_e23750, assign16530_e23750_d_n0, assign16530_e23750_d_n2, assign16530_e23750_d_n3, assign16530_e23750_d_n4, assign16530_e23750_d_n5, assign16530_e23750_d_n6, assign16530_e23750_d_n7, assign16530_e23750_d_n8, assign16530_e23750_d_n9, assign16530_e23750_d_n10, assign16530_e23750_d_n11, assign16530_e23750_d_n12, assign16530_e23750_d_n13, assign16530_e23750_d_n14,) = {
            if ((!(locals.var_t0 > 37.0)) && (locals.var_t0 < assign16530_e23739)) {
                let assign16530_e23743: f64 = (locals.var_t0).exp();
                (assign16530_e23743, (assign16530_e23743 * locals.var_t0_dn0), (assign16530_e23743 * locals.var_t0_dn2), (assign16530_e23743 * locals.var_t0_dn3), (assign16530_e23743 * locals.var_t0_dn4), (assign16530_e23743 * locals.var_t0_dn5), (assign16530_e23743 * locals.var_t0_dn6), (assign16530_e23743 * locals.var_t0_dn7), (assign16530_e23743 * locals.var_t0_dn8), (assign16530_e23743 * locals.var_t0_dn9), (assign16530_e23743 * locals.var_t0_dn10), (assign16530_e23743 * locals.var_t0_dn11), (assign16530_e23743 * locals.var_t0_dn12), (assign16530_e23743 * locals.var_t0_dn13), (assign16530_e23743 * locals.var_t0_dn14),)
            } else {
                let (assign16530_e23749, assign16530_e23749_d_n0, assign16530_e23749_d_n2, assign16530_e23749_d_n3, assign16530_e23749_d_n4, assign16530_e23749_d_n5, assign16530_e23749_d_n6, assign16530_e23749_d_n7, assign16530_e23749_d_n8, assign16530_e23749_d_n9, assign16530_e23749_d_n10, assign16530_e23749_d_n11, assign16530_e23749_d_n12, assign16530_e23749_d_n13, assign16530_e23749_d_n14,) = {
                    if (locals.var_t0 > 37.0) {
                        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign16530_e23749, assign16530_e23749_d_n0, assign16530_e23749_d_n2, assign16530_e23749_d_n3, assign16530_e23749_d_n4, assign16530_e23749_d_n5, assign16530_e23749_d_n6, assign16530_e23749_d_n7, assign16530_e23749_d_n8, assign16530_e23749_d_n9, assign16530_e23749_d_n10, assign16530_e23749_d_n11, assign16530_e23749_d_n12, assign16530_e23749_d_n13, assign16530_e23749_d_n14,)
            }
        };
        (assign16530_e23750, assign16530_e23750_d_n0, assign16530_e23750_d_n2, assign16530_e23750_d_n3, assign16530_e23750_d_n4, assign16530_e23750_d_n5, assign16530_e23750_d_n6, assign16530_e23750_d_n7, assign16530_e23750_d_n8, assign16530_e23750_d_n9, assign16530_e23750_d_n10, assign16530_e23750_d_n11, assign16530_e23750_d_n12, assign16530_e23750_d_n13, assign16530_e23750_d_n14,)
    }
};
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, ) = (assign16530_e23751, assign16530_e23751_d_n0, assign16530_e23751_d_n2, assign16530_e23751_d_n3, assign16530_e23751_d_n4, assign16530_e23751_d_n5, assign16530_e23751_d_n6, assign16530_e23751_d_n7, assign16530_e23751_d_n8, assign16530_e23751_d_n9, assign16530_e23751_d_n10, assign16530_e23751_d_n11, assign16530_e23751_d_n12, assign16530_e23751_d_n13, assign16530_e23751_d_n14, );
        locals.var_t1_rv = 0.0;

        let assign16540_e23754: f64 = (2.0 / p.p956);
        let assign16540_e23756: f64 = (assign16540_e23754 * locals.var_t1);
        let assign16540_e23758: f64 = (assign16540_e23756 - locals.var_vdscv);
        let assign16540_e23761: f64 = (2.0 / p.p956);
        let assign16540_e23763: f64 = (2.0_f64).ln();
        let assign16540_e23764: f64 = (assign16540_e23761 * assign16540_e23763);
        let assign16540_e23765: f64 = (assign16540_e23758 - assign16540_e23764);
        (locals.var_vdsx, locals.var_vdsx_dn0, locals.var_vdsx_dn2, locals.var_vdsx_dn3, locals.var_vdsx_dn4, locals.var_vdsx_dn5, locals.var_vdsx_dn6, locals.var_vdsx_dn7, locals.var_vdsx_dn8, locals.var_vdsx_dn9, locals.var_vdsx_dn10, locals.var_vdsx_dn11, locals.var_vdsx_dn12, locals.var_vdsx_dn13, locals.var_vdsx_dn14, ) = (assign16540_e23765, (assign16540_e23754 * locals.var_t1_dn0), (assign16540_e23754 * locals.var_t1_dn2), (assign16540_e23754 * locals.var_t1_dn3), (assign16540_e23754 * locals.var_t1_dn4), ((assign16540_e23754 * locals.var_t1_dn5) - locals.var_vdscv_dn5), ((assign16540_e23754 * locals.var_t1_dn6) - locals.var_vdscv_dn6), ((assign16540_e23754 * locals.var_t1_dn7) - locals.var_vdscv_dn7), (assign16540_e23754 * locals.var_t1_dn8), (assign16540_e23754 * locals.var_t1_dn9), (assign16540_e23754 * locals.var_t1_dn10), ((assign16540_e23754 * locals.var_t1_dn11) - locals.var_vdscv_dn11), (assign16540_e23754 * locals.var_t1_dn12), (assign16540_e23754 * locals.var_t1_dn13), (assign16540_e23754 * locals.var_t1_dn14), );
        locals.var_vdsx_rv = 0.0;

        let assign16550_e23770: f64 = (locals.var_vdscv - locals.var_vdsx);
        let assign16550_e23771: f64 = (0.5 * assign16550_e23770);
        let assign16550_e23772: f64 = (locals.var_vscv + assign16550_e23771);
        let assign16550_e23773: f64 = (-assign16550_e23772);
        (locals.var_vbsxcv, locals.var_vbsxcv_dn0, locals.var_vbsxcv_dn2, locals.var_vbsxcv_dn3, locals.var_vbsxcv_dn4, locals.var_vbsxcv_dn5, locals.var_vbsxcv_dn6, locals.var_vbsxcv_dn7, locals.var_vbsxcv_dn8, locals.var_vbsxcv_dn9, locals.var_vbsxcv_dn10, locals.var_vbsxcv_dn11, locals.var_vbsxcv_dn12, locals.var_vbsxcv_dn13, locals.var_vbsxcv_dn14, ) = (assign16550_e23773, (-(0.5 * (-locals.var_vdsx_dn0))), (-(0.5 * (-locals.var_vdsx_dn2))), (-(0.5 * (-locals.var_vdsx_dn3))), (-(0.5 * (-locals.var_vdsx_dn4))), (-(locals.var_vscv_dn5 + (0.5 * (locals.var_vdscv_dn5 - locals.var_vdsx_dn5)))), (-(locals.var_vscv_dn6 + (0.5 * (locals.var_vdscv_dn6 - locals.var_vdsx_dn6)))), (-(locals.var_vscv_dn7 + (0.5 * (locals.var_vdscv_dn7 - locals.var_vdsx_dn7)))), (-(0.5 * (-locals.var_vdsx_dn8))), (-(0.5 * (-locals.var_vdsx_dn9))), (-(0.5 * (-locals.var_vdsx_dn10))), (-(locals.var_vscv_dn11 + (0.5 * (locals.var_vdscv_dn11 - locals.var_vdsx_dn11)))), (-(0.5 * (-locals.var_vdsx_dn12))), (-(0.5 * (-locals.var_vdsx_dn13))), (-(0.5 * (-locals.var_vdsx_dn14))), );
        locals.var_vbsxcv_rv = 0.0;

        let assign16560_e23776: f64 = (p.p956 * locals.var_vds);
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign16560_e23776, 0.0, 0.0, 0.0, 0.0, (p.p956 * locals.var_vds_dn5), 0.0, (p.p956 * locals.var_vds_dn7), 0.0, 0.0, 0.0, (p.p956 * locals.var_vds_dn11), 0.0, 0.0, 0.0, );
        locals.var_t0_rv = 0.0;

    }
}
