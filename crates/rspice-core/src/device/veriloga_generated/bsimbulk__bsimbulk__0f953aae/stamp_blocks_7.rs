#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_9(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign4030_e5267: f64 = (1.0 + locals.var_t1);
        let assign4030_e5268: f64 = (locals.var_beta0_i * assign4030_e5267);
        locals.var_beta0_i = assign4030_e5268;
        locals.var_beta0_i_dn0 = ((locals.var_beta0_i_dn0 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn0));
        locals.var_beta0_i_dn2 = ((locals.var_beta0_i_dn2 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn2));
        locals.var_beta0_i_dn3 = ((locals.var_beta0_i_dn3 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn3));
        locals.var_beta0_i_dn4 = ((locals.var_beta0_i_dn4 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn4));
        locals.var_beta0_i_dn5 = ((locals.var_beta0_i_dn5 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn5));
        locals.var_beta0_i_dn6 = ((locals.var_beta0_i_dn6 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn6));
        locals.var_beta0_i_dn7 = ((locals.var_beta0_i_dn7 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn7));
        locals.var_beta0_i_dn8 = ((locals.var_beta0_i_dn8 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn8));
        locals.var_beta0_i_dn9 = ((locals.var_beta0_i_dn9 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn9));
        locals.var_beta0_i_dn10 = ((locals.var_beta0_i_dn10 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn10));
        locals.var_beta0_i_dn11 = ((locals.var_beta0_i_dn11 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn11));
        locals.var_beta0_i_dn12 = ((locals.var_beta0_i_dn12 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn12));
        locals.var_beta0_i_dn13 = ((locals.var_beta0_i_dn13 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn13));
        locals.var_beta0_i_dn14 = ((locals.var_beta0_i_dn14 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn14));
        locals.var_beta0_i_rv = 0.0;

        let assign4040_e5272: f64 = (locals.var_inv_w).powf(p.p520);
        let assign4040_e5275: f64 = (locals.var_inv_wwide).powf(p.p520);
        let assign4040_e5276: f64 = (assign4040_e5272 - assign4040_e5275);
        let assign4040_e5278: f64 = (assign4040_e5276).max(0.0);
        let assign4040_e5279: f64 = (p.p519 * assign4040_e5278);
        locals.var_t1 = assign4040_e5279;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        locals.var_beta1_i = p.p518;
        locals.var_beta1_i_dn0 = 0.0;
        locals.var_beta1_i_dn2 = 0.0;
        locals.var_beta1_i_dn3 = 0.0;
        locals.var_beta1_i_dn4 = 0.0;
        locals.var_beta1_i_dn5 = 0.0;
        locals.var_beta1_i_dn6 = 0.0;
        locals.var_beta1_i_dn7 = 0.0;
        locals.var_beta1_i_dn8 = 0.0;
        locals.var_beta1_i_dn9 = 0.0;
        locals.var_beta1_i_dn10 = 0.0;
        locals.var_beta1_i_dn11 = 0.0;
        locals.var_beta1_i_dn12 = 0.0;
        locals.var_beta1_i_dn13 = 0.0;
        locals.var_beta1_i_dn14 = 0.0;
        locals.var_beta1_i_rv = 0.0;

        let assign4060_e5284: f64 = (1.0 + locals.var_t1);
        let assign4060_e5285: f64 = (locals.var_beta1_i * assign4060_e5284);
        locals.var_beta1_i = assign4060_e5285;
        locals.var_beta1_i_dn0 = ((locals.var_beta1_i_dn0 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn0));
        locals.var_beta1_i_dn2 = ((locals.var_beta1_i_dn2 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn2));
        locals.var_beta1_i_dn3 = ((locals.var_beta1_i_dn3 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn3));
        locals.var_beta1_i_dn4 = ((locals.var_beta1_i_dn4 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn4));
        locals.var_beta1_i_dn5 = ((locals.var_beta1_i_dn5 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn5));
        locals.var_beta1_i_dn6 = ((locals.var_beta1_i_dn6 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn6));
        locals.var_beta1_i_dn7 = ((locals.var_beta1_i_dn7 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn7));
        locals.var_beta1_i_dn8 = ((locals.var_beta1_i_dn8 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn8));
        locals.var_beta1_i_dn9 = ((locals.var_beta1_i_dn9 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn9));
        locals.var_beta1_i_dn10 = ((locals.var_beta1_i_dn10 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn10));
        locals.var_beta1_i_dn11 = ((locals.var_beta1_i_dn11 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn11));
        locals.var_beta1_i_dn12 = ((locals.var_beta1_i_dn12 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn12));
        locals.var_beta1_i_dn13 = ((locals.var_beta1_i_dn13 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn13));
        locals.var_beta1_i_dn14 = ((locals.var_beta1_i_dn14 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn14));
        locals.var_beta1_i_rv = 0.0;

        let assign4070_e5289: f64 = (locals.var_inv_w).powf(p.p523);
        let assign4070_e5292: f64 = (locals.var_inv_wwide).powf(p.p523);
        let assign4070_e5293: f64 = (assign4070_e5289 - assign4070_e5292);
        let assign4070_e5295: f64 = (assign4070_e5293).max(0.0);
        let assign4070_e5296: f64 = (p.p522 * assign4070_e5295);
        locals.var_t1 = assign4070_e5296;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        locals.var_beta2_i = p.p521;
        locals.var_beta2_i_dn0 = 0.0;
        locals.var_beta2_i_dn2 = 0.0;
        locals.var_beta2_i_dn3 = 0.0;
        locals.var_beta2_i_dn4 = 0.0;
        locals.var_beta2_i_dn5 = 0.0;
        locals.var_beta2_i_dn6 = 0.0;
        locals.var_beta2_i_dn7 = 0.0;
        locals.var_beta2_i_dn8 = 0.0;
        locals.var_beta2_i_dn9 = 0.0;
        locals.var_beta2_i_dn10 = 0.0;
        locals.var_beta2_i_dn11 = 0.0;
        locals.var_beta2_i_dn12 = 0.0;
        locals.var_beta2_i_dn13 = 0.0;
        locals.var_beta2_i_dn14 = 0.0;
        locals.var_beta2_i_rv = 0.0;

        let assign4090_e5301: f64 = (1.0 + locals.var_t1);
        let assign4090_e5302: f64 = (locals.var_beta2_i * assign4090_e5301);
        locals.var_beta2_i = assign4090_e5302;
        locals.var_beta2_i_dn0 = ((locals.var_beta2_i_dn0 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn0));
        locals.var_beta2_i_dn2 = ((locals.var_beta2_i_dn2 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn2));
        locals.var_beta2_i_dn3 = ((locals.var_beta2_i_dn3 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn3));
        locals.var_beta2_i_dn4 = ((locals.var_beta2_i_dn4 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn4));
        locals.var_beta2_i_dn5 = ((locals.var_beta2_i_dn5 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn5));
        locals.var_beta2_i_dn6 = ((locals.var_beta2_i_dn6 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn6));
        locals.var_beta2_i_dn7 = ((locals.var_beta2_i_dn7 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn7));
        locals.var_beta2_i_dn8 = ((locals.var_beta2_i_dn8 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn8));
        locals.var_beta2_i_dn9 = ((locals.var_beta2_i_dn9 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn9));
        locals.var_beta2_i_dn10 = ((locals.var_beta2_i_dn10 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn10));
        locals.var_beta2_i_dn11 = ((locals.var_beta2_i_dn11 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn11));
        locals.var_beta2_i_dn12 = ((locals.var_beta2_i_dn12 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn12));
        locals.var_beta2_i_dn13 = ((locals.var_beta2_i_dn13 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn13));
        locals.var_beta2_i_dn14 = ((locals.var_beta2_i_dn14 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn14));
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
        locals.var_t0 = assign4160_e5387;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;
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
        locals.var_t1 = assign4170_e5406;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign4180_e5410: f64 = (1.0 + locals.var_t0);
        let assign4180_e5412: f64 = (assign4180_e5410 + locals.var_t1);
        let assign4180_e5413: f64 = (locals.var_ndepcv_i * assign4180_e5412);
        locals.var_ndepcv_i = assign4180_e5413;
        locals.var_ndepcv_i_dn0 = ((locals.var_ndepcv_i_dn0 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn0 + locals.var_t1_dn0)));
        locals.var_ndepcv_i_dn2 = ((locals.var_ndepcv_i_dn2 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn2 + locals.var_t1_dn2)));
        locals.var_ndepcv_i_dn3 = ((locals.var_ndepcv_i_dn3 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_ndepcv_i_dn4 = ((locals.var_ndepcv_i_dn4 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_ndepcv_i_dn5 = ((locals.var_ndepcv_i_dn5 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_ndepcv_i_dn6 = ((locals.var_ndepcv_i_dn6 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_ndepcv_i_dn7 = ((locals.var_ndepcv_i_dn7 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_ndepcv_i_dn8 = ((locals.var_ndepcv_i_dn8 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_ndepcv_i_dn9 = ((locals.var_ndepcv_i_dn9 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_ndepcv_i_dn10 = ((locals.var_ndepcv_i_dn10 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_ndepcv_i_dn11 = ((locals.var_ndepcv_i_dn11 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_ndepcv_i_dn12 = ((locals.var_ndepcv_i_dn12 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn12 + locals.var_t1_dn12)));
        locals.var_ndepcv_i_dn13 = ((locals.var_ndepcv_i_dn13 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn13 + locals.var_t1_dn13)));
        locals.var_ndepcv_i_dn14 = ((locals.var_ndepcv_i_dn14 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn14 + locals.var_t1_dn14)));
        locals.var_ndepcv_i_rv = 0.0;

        let assign4190_e5417: f64 = (locals.var_inv_lact).powf(p.p121);
        let assign4190_e5420: f64 = (locals.var_inv_llong).powf(p.p121);
        let assign4190_e5421: f64 = (assign4190_e5417 - assign4190_e5420);
        let assign4190_e5423: f64 = (assign4190_e5421).max(0.0);
        let assign4190_e5424: f64 = (p.p120 * assign4190_e5423);
        locals.var_t0 = assign4190_e5424;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign4200_e5428: f64 = (locals.var_inv_wact).powf(p.p123);
        let assign4200_e5431: f64 = (locals.var_inv_wwide).powf(p.p123);
        let assign4200_e5432: f64 = (assign4200_e5428 - assign4200_e5431);
        let assign4200_e5434: f64 = (assign4200_e5432).max(0.0);
        let assign4200_e5435: f64 = (p.p122 * assign4200_e5434);
        let assign4200_e5439: f64 = (locals.var_inv_wl).powf(p.p125);
        let assign4200_e5440: f64 = (p.p124 * assign4200_e5439);
        let assign4200_e5441: f64 = (assign4200_e5435 + assign4200_e5440);
        locals.var_t1 = assign4200_e5441;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign4210_e5445: f64 = (1.0 + locals.var_t0);
        let assign4210_e5447: f64 = (assign4210_e5445 + locals.var_t1);
        let assign4210_e5448: f64 = (locals.var_vfb_i * assign4210_e5447);
        locals.var_vfb_i = assign4210_e5448;
        locals.var_vfb_i_dn0 = ((locals.var_vfb_i_dn0 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn0 + locals.var_t1_dn0)));
        locals.var_vfb_i_dn2 = ((locals.var_vfb_i_dn2 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn2 + locals.var_t1_dn2)));
        locals.var_vfb_i_dn3 = ((locals.var_vfb_i_dn3 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_vfb_i_dn4 = ((locals.var_vfb_i_dn4 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_vfb_i_dn5 = ((locals.var_vfb_i_dn5 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_vfb_i_dn6 = ((locals.var_vfb_i_dn6 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_vfb_i_dn7 = ((locals.var_vfb_i_dn7 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_vfb_i_dn8 = ((locals.var_vfb_i_dn8 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_vfb_i_dn9 = ((locals.var_vfb_i_dn9 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_vfb_i_dn10 = ((locals.var_vfb_i_dn10 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_vfb_i_dn11 = ((locals.var_vfb_i_dn11 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_vfb_i_dn12 = ((locals.var_vfb_i_dn12 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn12 + locals.var_t1_dn12)));
        locals.var_vfb_i_dn13 = ((locals.var_vfb_i_dn13 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn13 + locals.var_t1_dn13)));
        locals.var_vfb_i_dn14 = ((locals.var_vfb_i_dn14 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn14 + locals.var_t1_dn14)));
        locals.var_vfb_i_rv = 0.0;

        let assign4220_e5452: f64 = (locals.var_inv_lact).powf(p.p131);
        let assign4220_e5455: f64 = (locals.var_inv_llong).powf(p.p131);
        let assign4220_e5456: f64 = (assign4220_e5452 - assign4220_e5455);
        let assign4220_e5458: f64 = (assign4220_e5456).max(0.0);
        let assign4220_e5459: f64 = (p.p130 * assign4220_e5458);
        locals.var_t0 = assign4220_e5459;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign4230_e5463: f64 = (locals.var_inv_wact).powf(p.p133);
        let assign4230_e5466: f64 = (locals.var_inv_wwide).powf(p.p133);
        let assign4230_e5467: f64 = (assign4230_e5463 - assign4230_e5466);
        let assign4230_e5469: f64 = (assign4230_e5467).max(0.0);
        let assign4230_e5470: f64 = (p.p132 * assign4230_e5469);
        let assign4230_e5474: f64 = (locals.var_inv_wl).powf(p.p135);
        let assign4230_e5475: f64 = (p.p134 * assign4230_e5474);
        let assign4230_e5476: f64 = (assign4230_e5470 + assign4230_e5475);
        locals.var_t1 = assign4230_e5476;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign4240_e5480: f64 = (1.0 + locals.var_t0);
        let assign4240_e5482: f64 = (assign4240_e5480 + locals.var_t1);
        let assign4240_e5483: f64 = (locals.var_vfbcv_i * assign4240_e5482);
        locals.var_vfbcv_i = assign4240_e5483;
        locals.var_vfbcv_i_dn0 = ((locals.var_vfbcv_i_dn0 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn0 + locals.var_t1_dn0)));
        locals.var_vfbcv_i_dn2 = ((locals.var_vfbcv_i_dn2 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn2 + locals.var_t1_dn2)));
        locals.var_vfbcv_i_dn3 = ((locals.var_vfbcv_i_dn3 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_vfbcv_i_dn4 = ((locals.var_vfbcv_i_dn4 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_vfbcv_i_dn5 = ((locals.var_vfbcv_i_dn5 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_vfbcv_i_dn6 = ((locals.var_vfbcv_i_dn6 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_vfbcv_i_dn7 = ((locals.var_vfbcv_i_dn7 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_vfbcv_i_dn8 = ((locals.var_vfbcv_i_dn8 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_vfbcv_i_dn9 = ((locals.var_vfbcv_i_dn9 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_vfbcv_i_dn10 = ((locals.var_vfbcv_i_dn10 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_vfbcv_i_dn11 = ((locals.var_vfbcv_i_dn11 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_vfbcv_i_dn12 = ((locals.var_vfbcv_i_dn12 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn12 + locals.var_t1_dn12)));
        locals.var_vfbcv_i_dn13 = ((locals.var_vfbcv_i_dn13 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn13 + locals.var_t1_dn13)));
        locals.var_vfbcv_i_dn14 = ((locals.var_vfbcv_i_dn14 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn14 + locals.var_t1_dn14)));
        locals.var_vfbcv_i_rv = 0.0;

        let assign4250_e5487: f64 = (locals.var_inv_lact).powf(p.p264);
        let assign4250_e5490: f64 = (locals.var_inv_llong).powf(p.p264);
        let assign4250_e5491: f64 = (assign4250_e5487 - assign4250_e5490);
        let assign4250_e5493: f64 = (assign4250_e5491).max(0.0);
        let assign4250_e5494: f64 = (p.p263 * assign4250_e5493);
        locals.var_t0 = assign4250_e5494;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign4260_e5498: f64 = (locals.var_inv_w).powf(p.p266);
        let assign4260_e5501: f64 = (locals.var_inv_wwide).powf(p.p266);
        let assign4260_e5502: f64 = (assign4260_e5498 - assign4260_e5501);
        let assign4260_e5504: f64 = (assign4260_e5502).max(0.0);
        let assign4260_e5505: f64 = (p.p265 * assign4260_e5504);
        let assign4260_e5509: f64 = (locals.var_inv_wl).powf(p.p268);
        let assign4260_e5510: f64 = (p.p267 * assign4260_e5509);
        let assign4260_e5511: f64 = (assign4260_e5505 + assign4260_e5510);
        locals.var_t1 = assign4260_e5511;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign4270_e5515: f64 = (1.0 + locals.var_t0);
        let assign4270_e5517: f64 = (assign4270_e5515 + locals.var_t1);
        let assign4270_e5518: f64 = (locals.var_vsatcv_i * assign4270_e5517);
        locals.var_vsatcv_i = assign4270_e5518;
        locals.var_vsatcv_i_dn0 = ((locals.var_vsatcv_i_dn0 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn0 + locals.var_t1_dn0)));
        locals.var_vsatcv_i_dn2 = ((locals.var_vsatcv_i_dn2 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn2 + locals.var_t1_dn2)));
        locals.var_vsatcv_i_dn3 = ((locals.var_vsatcv_i_dn3 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_vsatcv_i_dn4 = ((locals.var_vsatcv_i_dn4 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_vsatcv_i_dn5 = ((locals.var_vsatcv_i_dn5 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_vsatcv_i_dn6 = ((locals.var_vsatcv_i_dn6 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_vsatcv_i_dn7 = ((locals.var_vsatcv_i_dn7 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_vsatcv_i_dn8 = ((locals.var_vsatcv_i_dn8 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_vsatcv_i_dn9 = ((locals.var_vsatcv_i_dn9 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_vsatcv_i_dn10 = ((locals.var_vsatcv_i_dn10 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_vsatcv_i_dn11 = ((locals.var_vsatcv_i_dn11 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_vsatcv_i_dn12 = ((locals.var_vsatcv_i_dn12 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn12 + locals.var_t1_dn12)));
        locals.var_vsatcv_i_dn13 = ((locals.var_vsatcv_i_dn13 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn13 + locals.var_t1_dn13)));
        locals.var_vsatcv_i_dn14 = ((locals.var_vsatcv_i_dn14 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn14 + locals.var_t1_dn14)));
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
        locals.var_t0 = assign4300_e5547;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_10(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign4310_e5551: f64 = (locals.var_inv_w).powf(p.p189);
        let assign4310_e5554: f64 = (locals.var_inv_wwide).powf(p.p189);
        let assign4310_e5555: f64 = (assign4310_e5551 - assign4310_e5554);
        let assign4310_e5557: f64 = (assign4310_e5555).max(0.0);
        let assign4310_e5558: f64 = (p.p188 * assign4310_e5557);
        let assign4310_e5562: f64 = (locals.var_inv_wl).powf(p.p191);
        let assign4310_e5563: f64 = (p.p190 * assign4310_e5562);
        let assign4310_e5564: f64 = (assign4310_e5558 + assign4310_e5563);
        locals.var_t1 = assign4310_e5564;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign4320_e5568: f64 = (1.0 + locals.var_t0);
        let assign4320_e5570: f64 = (assign4320_e5568 + locals.var_t1);
        let assign4320_e5571: f64 = (locals.var_k1_i * assign4320_e5570);
        locals.var_k1_i = assign4320_e5571;
        locals.var_k1_i_dn0 = ((locals.var_k1_i_dn0 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn0 + locals.var_t1_dn0)));
        locals.var_k1_i_dn2 = ((locals.var_k1_i_dn2 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn2 + locals.var_t1_dn2)));
        locals.var_k1_i_dn3 = ((locals.var_k1_i_dn3 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_k1_i_dn4 = ((locals.var_k1_i_dn4 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_k1_i_dn5 = ((locals.var_k1_i_dn5 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_k1_i_dn6 = ((locals.var_k1_i_dn6 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_k1_i_dn7 = ((locals.var_k1_i_dn7 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_k1_i_dn8 = ((locals.var_k1_i_dn8 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_k1_i_dn9 = ((locals.var_k1_i_dn9 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_k1_i_dn10 = ((locals.var_k1_i_dn10 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_k1_i_dn11 = ((locals.var_k1_i_dn11 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_k1_i_dn12 = ((locals.var_k1_i_dn12 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn12 + locals.var_t1_dn12)));
        locals.var_k1_i_dn13 = ((locals.var_k1_i_dn13 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn13 + locals.var_t1_dn13)));
        locals.var_k1_i_dn14 = ((locals.var_k1_i_dn14 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn14 + locals.var_t1_dn14)));
        locals.var_k1_i_rv = 0.0;

        let assign4330_e5575: f64 = (locals.var_inv_l).powf(p.p197);
        let assign4330_e5578: f64 = (locals.var_inv_llong).powf(p.p197);
        let assign4330_e5579: f64 = (assign4330_e5575 - assign4330_e5578);
        let assign4330_e5581: f64 = (assign4330_e5579).max(0.0);
        let assign4330_e5582: f64 = (p.p196 * assign4330_e5581);
        locals.var_t0 = assign4330_e5582;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign4340_e5586: f64 = (locals.var_inv_w).powf(p.p199);
        let assign4340_e5589: f64 = (locals.var_inv_wwide).powf(p.p199);
        let assign4340_e5590: f64 = (assign4340_e5586 - assign4340_e5589);
        let assign4340_e5592: f64 = (assign4340_e5590).max(0.0);
        let assign4340_e5593: f64 = (p.p198 * assign4340_e5592);
        let assign4340_e5597: f64 = (locals.var_inv_wl).powf(p.p201);
        let assign4340_e5598: f64 = (p.p200 * assign4340_e5597);
        let assign4340_e5599: f64 = (assign4340_e5593 + assign4340_e5598);
        locals.var_t1 = assign4340_e5599;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign4350_e5603: f64 = (1.0 + locals.var_t0);
        let assign4350_e5605: f64 = (assign4350_e5603 + locals.var_t1);
        let assign4350_e5606: f64 = (locals.var_k2_i * assign4350_e5605);
        locals.var_k2_i = assign4350_e5606;
        locals.var_k2_i_dn0 = ((locals.var_k2_i_dn0 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn0 + locals.var_t1_dn0)));
        locals.var_k2_i_dn2 = ((locals.var_k2_i_dn2 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn2 + locals.var_t1_dn2)));
        locals.var_k2_i_dn3 = ((locals.var_k2_i_dn3 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_k2_i_dn4 = ((locals.var_k2_i_dn4 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_k2_i_dn5 = ((locals.var_k2_i_dn5 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_k2_i_dn6 = ((locals.var_k2_i_dn6 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_k2_i_dn7 = ((locals.var_k2_i_dn7 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_k2_i_dn8 = ((locals.var_k2_i_dn8 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_k2_i_dn9 = ((locals.var_k2_i_dn9 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_k2_i_dn10 = ((locals.var_k2_i_dn10 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_k2_i_dn11 = ((locals.var_k2_i_dn11 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_k2_i_dn12 = ((locals.var_k2_i_dn12 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn12 + locals.var_t1_dn12)));
        locals.var_k2_i_dn13 = ((locals.var_k2_i_dn13 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn13 + locals.var_t1_dn13)));
        locals.var_k2_i_dn14 = ((locals.var_k2_i_dn14 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn14 + locals.var_t1_dn14)));
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

        let (assign4450_e5688,) = {
    if (locals.var_guard40 != 0.0) {
        let assign4450_e5677: f64 = (locals.var_inv_l).powf(p.p398);
        let assign4450_e5680: f64 = (locals.var_inv_llong).powf(p.p398);
        let assign4450_e5681: f64 = (assign4450_e5677 - assign4450_e5680);
        let assign4450_e5683: f64 = (assign4450_e5681).max(0.0);
        let assign4450_e5684: f64 = (p.p397 * assign4450_e5683);
        let assign4450_e5685: f64 = (1.0 + assign4450_e5684);
        let assign4450_e5686: f64 = (locals.var_rsw_i * assign4450_e5685);
        (assign4450_e5686,)
    } else {
        (locals.var_rsw_i,)
    }
};
        locals.var_rsw_i = assign4450_e5688;
        locals.var_rsw_i_rv = 0.0;

        let (assign4460_e5706,) = {
    if (locals.var_guard40 != 0.0) {
        let assign4460_e5695: f64 = (locals.var_inv_l).powf(p.p408);
        let assign4460_e5698: f64 = (locals.var_inv_llong).powf(p.p408);
        let assign4460_e5699: f64 = (assign4460_e5695 - assign4460_e5698);
        let assign4460_e5701: f64 = (assign4460_e5699).max(0.0);
        let assign4460_e5702: f64 = (p.p407 * assign4460_e5701);
        let assign4460_e5703: f64 = (1.0 + assign4460_e5702);
        let assign4460_e5704: f64 = (locals.var_rdw_i * assign4460_e5703);
        (assign4460_e5704,)
    } else {
        (locals.var_rdw_i,)
    }
};
        locals.var_rdw_i = assign4460_e5706;
        locals.var_rdw_i_rv = 0.0;

        let (assign4470_e5725,) = {
    if (locals.var_guard40 == 0.0) {
        let assign4470_e5714: f64 = (locals.var_inv_l).powf(p.p415);
        let assign4470_e5717: f64 = (locals.var_inv_llong).powf(p.p415);
        let assign4470_e5718: f64 = (assign4470_e5714 - assign4470_e5717);
        let assign4470_e5720: f64 = (assign4470_e5718).max(0.0);
        let assign4470_e5721: f64 = (p.p414 * assign4470_e5720);
        let assign4470_e5722: f64 = (1.0 + assign4470_e5721);
        let assign4470_e5723: f64 = (locals.var_rdsw_i * assign4470_e5722);
        (assign4470_e5723,)
    } else {
        (locals.var_rdsw_i,)
    }
};
        locals.var_rdsw_i = assign4470_e5725;
        locals.var_rdsw_i_rv = 0.0;

        let assign4480_e5728: f64 = if locals.var_ucs_i < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard41 = assign4480_e5728;
        locals.var_guard41_rv = 0.0;

        let (assign4490_e5732,) = {
    if (locals.var_guard41 != 0.0) {
        (1.0,)
    } else {
        (locals.var_ucs_i,)
    }
};
        locals.var_ucs_i = assign4490_e5732;
        locals.var_ucs_i_rv = 0.0;

        let assign4500_e5735: f64 = if locals.var_ucs_i > 2.0 { 1.0 } else { 0.0 };
        locals.var_guard42 = assign4500_e5735;
        locals.var_guard42_rv = 0.0;

        let (assign4510_e5742,) = {
    if ((locals.var_guard41 == 0.0) && (locals.var_guard42 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_ucs_i,)
    }
};
        locals.var_ucs_i = assign4510_e5742;
        locals.var_ucs_i_rv = 0.0;

        let assign4520_e5745: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard43 = assign4520_e5745;
        locals.var_guard43_rv = 0.0;

        let assign4530_e5748: f64 = if locals.var_ucsr_i < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard44 = assign4530_e5748;
        locals.var_guard44_rv = 0.0;

        let (assign4540_e5754,) = {
    if ((locals.var_guard43 != 0.0) && (locals.var_guard44 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_ucsr_i,)
    }
};
        locals.var_ucsr_i = assign4540_e5754;
        locals.var_ucsr_i_rv = 0.0;

        let assign4550_e5757: f64 = if locals.var_ucsr_i > 2.0 { 1.0 } else { 0.0 };
        locals.var_guard45 = assign4550_e5757;
        locals.var_guard45_rv = 0.0;

        let (assign4560_e5766,) = {
    if (((locals.var_guard43 != 0.0) && (locals.var_guard44 == 0.0)) && (locals.var_guard45 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_ucsr_i,)
    }
};
        locals.var_ucsr_i = assign4560_e5766;
        locals.var_ucsr_i_rv = 0.0;

        let assign4800_e5840: f64 = if locals.var_m0_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard67 = assign4800_e5840;
        locals.var_guard67_rv = 0.0;

        let (assign4810_e5844,) = {
    if (locals.var_guard67 != 0.0) {
        (0.0,)
    } else {
        (locals.var_m0_i,)
    }
};
        locals.var_m0_i = assign4810_e5844;
        locals.var_m0_i_rv = 0.0;

        let assign4820_e5847: f64 = if locals.var_u0_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard68 = assign4820_e5847;
        locals.var_guard68_rv = 0.0;

        let (assign4830_e5851,) = {
    if (locals.var_guard68 != 0.0) {
        (0.067,)
    } else {
        (locals.var_u0_i,)
    }
};
        locals.var_u0_i = assign4830_e5851;
        locals.var_u0_i_rv = 0.0;

        let assign4840_e5854: f64 = if locals.var_ua_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard69 = assign4840_e5854;
        locals.var_guard69_rv = 0.0;

        let (assign4850_e5858, assign4850_e5858_d_n0, assign4850_e5858_d_n2, assign4850_e5858_d_n3, assign4850_e5858_d_n4, assign4850_e5858_d_n5, assign4850_e5858_d_n6, assign4850_e5858_d_n7, assign4850_e5858_d_n8, assign4850_e5858_d_n9, assign4850_e5858_d_n10, assign4850_e5858_d_n11, assign4850_e5858_d_n12, assign4850_e5858_d_n13, assign4850_e5858_d_n14,) = {
    if (locals.var_guard69 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ua_i, locals.var_ua_i_dn0, locals.var_ua_i_dn2, locals.var_ua_i_dn3, locals.var_ua_i_dn4, locals.var_ua_i_dn5, locals.var_ua_i_dn6, locals.var_ua_i_dn7, locals.var_ua_i_dn8, locals.var_ua_i_dn9, locals.var_ua_i_dn10, locals.var_ua_i_dn11, locals.var_ua_i_dn12, locals.var_ua_i_dn13, locals.var_ua_i_dn14,)
    }
};
        locals.var_ua_i = assign4850_e5858;
        locals.var_ua_i_dn0 = assign4850_e5858_d_n0;
        locals.var_ua_i_dn2 = assign4850_e5858_d_n2;
        locals.var_ua_i_dn3 = assign4850_e5858_d_n3;
        locals.var_ua_i_dn4 = assign4850_e5858_d_n4;
        locals.var_ua_i_dn5 = assign4850_e5858_d_n5;
        locals.var_ua_i_dn6 = assign4850_e5858_d_n6;
        locals.var_ua_i_dn7 = assign4850_e5858_d_n7;
        locals.var_ua_i_dn8 = assign4850_e5858_d_n8;
        locals.var_ua_i_dn9 = assign4850_e5858_d_n9;
        locals.var_ua_i_dn10 = assign4850_e5858_d_n10;
        locals.var_ua_i_dn11 = assign4850_e5858_d_n11;
        locals.var_ua_i_dn12 = assign4850_e5858_d_n12;
        locals.var_ua_i_dn13 = assign4850_e5858_d_n13;
        locals.var_ua_i_dn14 = assign4850_e5858_d_n14;
        locals.var_ua_i_rv = 0.0;

        let assign4860_e5861: f64 = if locals.var_eu_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard70 = assign4860_e5861;
        locals.var_guard70_rv = 0.0;

        let (assign4870_e5865, assign4870_e5865_d_n0, assign4870_e5865_d_n2, assign4870_e5865_d_n3, assign4870_e5865_d_n4, assign4870_e5865_d_n5, assign4870_e5865_d_n6, assign4870_e5865_d_n7, assign4870_e5865_d_n8, assign4870_e5865_d_n9, assign4870_e5865_d_n10, assign4870_e5865_d_n11, assign4870_e5865_d_n12, assign4870_e5865_d_n13, assign4870_e5865_d_n14,) = {
    if (locals.var_guard70 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_eu_i, locals.var_eu_i_dn0, locals.var_eu_i_dn2, locals.var_eu_i_dn3, locals.var_eu_i_dn4, locals.var_eu_i_dn5, locals.var_eu_i_dn6, locals.var_eu_i_dn7, locals.var_eu_i_dn8, locals.var_eu_i_dn9, locals.var_eu_i_dn10, locals.var_eu_i_dn11, locals.var_eu_i_dn12, locals.var_eu_i_dn13, locals.var_eu_i_dn14,)
    }
};
        locals.var_eu_i = assign4870_e5865;
        locals.var_eu_i_dn0 = assign4870_e5865_d_n0;
        locals.var_eu_i_dn2 = assign4870_e5865_d_n2;
        locals.var_eu_i_dn3 = assign4870_e5865_d_n3;
        locals.var_eu_i_dn4 = assign4870_e5865_d_n4;
        locals.var_eu_i_dn5 = assign4870_e5865_d_n5;
        locals.var_eu_i_dn6 = assign4870_e5865_d_n6;
        locals.var_eu_i_dn7 = assign4870_e5865_d_n7;
        locals.var_eu_i_dn8 = assign4870_e5865_d_n8;
        locals.var_eu_i_dn9 = assign4870_e5865_d_n9;
        locals.var_eu_i_dn10 = assign4870_e5865_d_n10;
        locals.var_eu_i_dn11 = assign4870_e5865_d_n11;
        locals.var_eu_i_dn12 = assign4870_e5865_d_n12;
        locals.var_eu_i_dn13 = assign4870_e5865_d_n13;
        locals.var_eu_i_dn14 = assign4870_e5865_d_n14;
        locals.var_eu_i_rv = 0.0;

        let assign4880_e5868: f64 = if locals.var_ud_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard71 = assign4880_e5868;
        locals.var_guard71_rv = 0.0;

        let (assign4890_e5872, assign4890_e5872_d_n0, assign4890_e5872_d_n2, assign4890_e5872_d_n3, assign4890_e5872_d_n4, assign4890_e5872_d_n5, assign4890_e5872_d_n6, assign4890_e5872_d_n7, assign4890_e5872_d_n8, assign4890_e5872_d_n9, assign4890_e5872_d_n10, assign4890_e5872_d_n11, assign4890_e5872_d_n12, assign4890_e5872_d_n13, assign4890_e5872_d_n14,) = {
    if (locals.var_guard71 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ud_i, locals.var_ud_i_dn0, locals.var_ud_i_dn2, locals.var_ud_i_dn3, locals.var_ud_i_dn4, locals.var_ud_i_dn5, locals.var_ud_i_dn6, locals.var_ud_i_dn7, locals.var_ud_i_dn8, locals.var_ud_i_dn9, locals.var_ud_i_dn10, locals.var_ud_i_dn11, locals.var_ud_i_dn12, locals.var_ud_i_dn13, locals.var_ud_i_dn14,)
    }
};
        locals.var_ud_i = assign4890_e5872;
        locals.var_ud_i_dn0 = assign4890_e5872_d_n0;
        locals.var_ud_i_dn2 = assign4890_e5872_d_n2;
        locals.var_ud_i_dn3 = assign4890_e5872_d_n3;
        locals.var_ud_i_dn4 = assign4890_e5872_d_n4;
        locals.var_ud_i_dn5 = assign4890_e5872_d_n5;
        locals.var_ud_i_dn6 = assign4890_e5872_d_n6;
        locals.var_ud_i_dn7 = assign4890_e5872_d_n7;
        locals.var_ud_i_dn8 = assign4890_e5872_d_n8;
        locals.var_ud_i_dn9 = assign4890_e5872_d_n9;
        locals.var_ud_i_dn10 = assign4890_e5872_d_n10;
        locals.var_ud_i_dn11 = assign4890_e5872_d_n11;
        locals.var_ud_i_dn12 = assign4890_e5872_d_n12;
        locals.var_ud_i_dn13 = assign4890_e5872_d_n13;
        locals.var_ud_i_dn14 = assign4890_e5872_d_n14;
        locals.var_ud_i_rv = 0.0;

        let assign4900_e5875: f64 = if locals.var_ucs_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard72 = assign4900_e5875;
        locals.var_guard72_rv = 0.0;

        let (assign4910_e5879,) = {
    if (locals.var_guard72 != 0.0) {
        (0.0,)
    } else {
        (locals.var_ucs_i,)
    }
};
        locals.var_ucs_i = assign4910_e5879;
        locals.var_ucs_i_rv = 0.0;

        let assign4920_e5882: f64 = if locals.var_beta1_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard73 = assign4920_e5882;
        locals.var_guard73_rv = 0.0;

        let (assign4930_e5886, assign4930_e5886_d_n0, assign4930_e5886_d_n2, assign4930_e5886_d_n3, assign4930_e5886_d_n4, assign4930_e5886_d_n5, assign4930_e5886_d_n6, assign4930_e5886_d_n7, assign4930_e5886_d_n8, assign4930_e5886_d_n9, assign4930_e5886_d_n10, assign4930_e5886_d_n11, assign4930_e5886_d_n12, assign4930_e5886_d_n13, assign4930_e5886_d_n14,) = {
    if (locals.var_guard73 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_beta1_i, locals.var_beta1_i_dn0, locals.var_beta1_i_dn2, locals.var_beta1_i_dn3, locals.var_beta1_i_dn4, locals.var_beta1_i_dn5, locals.var_beta1_i_dn6, locals.var_beta1_i_dn7, locals.var_beta1_i_dn8, locals.var_beta1_i_dn9, locals.var_beta1_i_dn10, locals.var_beta1_i_dn11, locals.var_beta1_i_dn12, locals.var_beta1_i_dn13, locals.var_beta1_i_dn14,)
    }
};
        locals.var_beta1_i = assign4930_e5886;
        locals.var_beta1_i_dn0 = assign4930_e5886_d_n0;
        locals.var_beta1_i_dn2 = assign4930_e5886_d_n2;
        locals.var_beta1_i_dn3 = assign4930_e5886_d_n3;
        locals.var_beta1_i_dn4 = assign4930_e5886_d_n4;
        locals.var_beta1_i_dn5 = assign4930_e5886_d_n5;
        locals.var_beta1_i_dn6 = assign4930_e5886_d_n6;
        locals.var_beta1_i_dn7 = assign4930_e5886_d_n7;
        locals.var_beta1_i_dn8 = assign4930_e5886_d_n8;
        locals.var_beta1_i_dn9 = assign4930_e5886_d_n9;
        locals.var_beta1_i_dn10 = assign4930_e5886_d_n10;
        locals.var_beta1_i_dn11 = assign4930_e5886_d_n11;
        locals.var_beta1_i_dn12 = assign4930_e5886_d_n12;
        locals.var_beta1_i_dn13 = assign4930_e5886_d_n13;
        locals.var_beta1_i_dn14 = assign4930_e5886_d_n14;
        locals.var_beta1_i_rv = 0.0;

        let assign4940_e5889: f64 = if p.p1065 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard74 = assign4940_e5889;
        locals.var_guard74_rv = 0.0;

        let (assign4950_e5893,) = {
    if (locals.var_guard74 != 0.0) {
        (p.p1066,)
    } else {
        (locals.var_lh1,)
    }
};
        locals.var_lh1 = assign4950_e5893;
        locals.var_lh1_rv = 0.0;

        let assign4960_e5896: f64 = if locals.var_leff > locals.var_lh1 { 1.0 } else { 0.0 };
        locals.var_guard75 = assign4960_e5896;
        locals.var_guard75_rv = 0.0;

        let (assign4970_e5904, assign4970_e5904_d_n0, assign4970_e5904_d_n2, assign4970_e5904_d_n3, assign4970_e5904_d_n4, assign4970_e5904_d_n5, assign4970_e5904_d_n6, assign4970_e5904_d_n7, assign4970_e5904_d_n8, assign4970_e5904_d_n9, assign4970_e5904_d_n10, assign4970_e5904_d_n11, assign4970_e5904_d_n12, assign4970_e5904_d_n13, assign4970_e5904_d_n14,) = {
    if ((locals.var_guard74 != 0.0) && (locals.var_guard75 != 0.0)) {
        let assign4970_e5902: f64 = (locals.var_leff - locals.var_lh1);
        (assign4970_e5902, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign4970_e5904;
        locals.var_t0_dn0 = assign4970_e5904_d_n0;
        locals.var_t0_dn2 = assign4970_e5904_d_n2;
        locals.var_t0_dn3 = assign4970_e5904_d_n3;
        locals.var_t0_dn4 = assign4970_e5904_d_n4;
        locals.var_t0_dn5 = assign4970_e5904_d_n5;
        locals.var_t0_dn6 = assign4970_e5904_d_n6;
        locals.var_t0_dn7 = assign4970_e5904_d_n7;
        locals.var_t0_dn8 = assign4970_e5904_d_n8;
        locals.var_t0_dn9 = assign4970_e5904_d_n9;
        locals.var_t0_dn10 = assign4970_e5904_d_n10;
        locals.var_t0_dn11 = assign4970_e5904_d_n11;
        locals.var_t0_dn12 = assign4970_e5904_d_n12;
        locals.var_t0_dn13 = assign4970_e5904_d_n13;
        locals.var_t0_dn14 = assign4970_e5904_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign4980_e5911,) = {
    if ((locals.var_guard74 != 0.0) && (locals.var_guard75 == 0.0)) {
        (locals.var_leff,)
    } else {
        (locals.var_lh1,)
    }
};
        locals.var_lh1 = assign4980_e5911;
        locals.var_lh1_rv = 0.0;

        let (assign4990_e5918, assign4990_e5918_d_n0, assign4990_e5918_d_n2, assign4990_e5918_d_n3, assign4990_e5918_d_n4, assign4990_e5918_d_n5, assign4990_e5918_d_n6, assign4990_e5918_d_n7, assign4990_e5918_d_n8, assign4990_e5918_d_n9, assign4990_e5918_d_n10, assign4990_e5918_d_n11, assign4990_e5918_d_n12, assign4990_e5918_d_n13, assign4990_e5918_d_n14,) = {
    if ((locals.var_guard74 != 0.0) && (locals.var_guard75 == 0.0)) {
        (locals.var_lh1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign4990_e5918;
        locals.var_t0_dn0 = assign4990_e5918_d_n0;
        locals.var_t0_dn2 = assign4990_e5918_d_n2;
        locals.var_t0_dn3 = assign4990_e5918_d_n3;
        locals.var_t0_dn4 = assign4990_e5918_d_n4;
        locals.var_t0_dn5 = assign4990_e5918_d_n5;
        locals.var_t0_dn6 = assign4990_e5918_d_n6;
        locals.var_t0_dn7 = assign4990_e5918_d_n7;
        locals.var_t0_dn8 = assign4990_e5918_d_n8;
        locals.var_t0_dn9 = assign4990_e5918_d_n9;
        locals.var_t0_dn10 = assign4990_e5918_d_n10;
        locals.var_t0_dn11 = assign4990_e5918_d_n11;
        locals.var_t0_dn12 = assign4990_e5918_d_n12;
        locals.var_t0_dn13 = assign4990_e5918_d_n13;
        locals.var_t0_dn14 = assign4990_e5918_d_n14;
        locals.var_t0_rv = 0.0;

        let assign5000_e5922: f64 = (locals.var_t0 / 2.0);
        let assign5000_e5923: f64 = if p.p801 >= assign5000_e5922 { 1.0 } else { 0.0 };
        locals.var_guard76 = assign5000_e5923;
        locals.var_guard76_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_11(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign5010_e5929,) = {
    if ((locals.var_guard74 != 0.0) && (locals.var_guard76 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_lintnoi_i,)
    }
};
        locals.var_lintnoi_i = assign5010_e5929;
        locals.var_lintnoi_i_rv = 0.0;

        let (assign5020_e5936,) = {
    if ((locals.var_guard74 != 0.0) && (locals.var_guard76 == 0.0)) {
        (p.p801,)
    } else {
        (locals.var_lintnoi_i,)
    }
};
        locals.var_lintnoi_i = assign5020_e5936;
        locals.var_lintnoi_i_rv = 0.0;

        locals.var_nuendd = 0.0;
        locals.var_nuendd_rv = 0.0;

        locals.var_nuends = 0.0;
        locals.var_nuends_rv = 0.0;

        locals.var_nuintd = 0.0;
        locals.var_nuintd_rv = 0.0;

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

        let (assign5130_e5957,) = {
    if (locals.var_guard77 != 0.0) {
        let assign5130_e5955: f64 = (p.p374 * p.p3);
        (assign5130_e5955,)
    } else {
        (locals.var_rsourcegeo,)
    }
};
        locals.var_rsourcegeo = assign5130_e5957;
        locals.var_rsourcegeo_rv = 0.0;

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

        let (assign5170_e5983,) = {
    if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_nuendd,)
    }
};
        locals.var_nuendd = assign5170_e5983;
        locals.var_nuendd_rv = 0.0;

        let (assign5180_e5994,) = {
    if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_nuends,)
    }
};
        locals.var_nuends = assign5180_e5994;
        locals.var_nuends_rv = 0.0;

        let (assign5190_e6013,) = {
    if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) {
        let assign5190_e6006: f64 = (p.p2 - 1.0);
        let assign5190_e6008: f64 = (assign5190_e6006 / 2.0);
        let assign5190_e6010: f64 = (assign5190_e6008).max(0.0);
        let assign5190_e6011: f64 = (2.0 * assign5190_e6010);
        (assign5190_e6011,)
    } else {
        (locals.var_nuintd,)
    }
};
        locals.var_nuintd = assign5190_e6013;
        locals.var_nuintd_rv = 0.0;

        let (assign5200_e6024,) = {
    if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) {
        (locals.var_nuintd,)
    } else {
        (locals.var_nuints,)
    }
};
        locals.var_nuints = assign5200_e6024;
        locals.var_nuints_rv = 0.0;

        let assign5210_e6027: f64 = if p.p6 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard81 = assign5210_e6027;
        locals.var_guard81_rv = 0.0;

        let (assign5220_e6041,) = {
    if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 == 0.0)) && (locals.var_guard81 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_nuendd,)
    }
};
        locals.var_nuendd = assign5220_e6041;
        locals.var_nuendd_rv = 0.0;

        let (assign5230_e6063,) = {
    if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 == 0.0)) && (locals.var_guard81 != 0.0)) {
        let assign5230_e6056: f64 = (p.p2 / 2.0);
        let assign5230_e6058: f64 = (assign5230_e6056 - 1.0);
        let assign5230_e6060: f64 = (assign5230_e6058).max(0.0);
        let assign5230_e6061: f64 = (2.0 * assign5230_e6060);
        (assign5230_e6061,)
    } else {
        (locals.var_nuintd,)
    }
};
        locals.var_nuintd = assign5230_e6063;
        locals.var_nuintd_rv = 0.0;

        let (assign5240_e6077,) = {
    if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 == 0.0)) && (locals.var_guard81 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_nuends,)
    }
};
        locals.var_nuends = assign5240_e6077;
        locals.var_nuends_rv = 0.0;

        let (assign5250_e6091,) = {
    if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 == 0.0)) && (locals.var_guard81 != 0.0)) {
        (p.p2,)
    } else {
        (locals.var_nuints,)
    }
};
        locals.var_nuints = assign5250_e6091;
        locals.var_nuints_rv = 0.0;

        let (assign5260_e6106,) = {
    if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 == 0.0)) && (locals.var_guard81 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_nuendd,)
    }
};
        locals.var_nuendd = assign5260_e6106;
        locals.var_nuendd_rv = 0.0;

        let (assign5270_e6121,) = {
    if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 == 0.0)) && (locals.var_guard81 == 0.0)) {
        (p.p2,)
    } else {
        (locals.var_nuintd,)
    }
};
        locals.var_nuintd = assign5270_e6121;
        locals.var_nuintd_rv = 0.0;

        let (assign5280_e6136,) = {
    if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 == 0.0)) && (locals.var_guard81 == 0.0)) {
        (2.0,)
    } else {
        (locals.var_nuends,)
    }
};
        locals.var_nuends = assign5280_e6136;
        locals.var_nuends_rv = 0.0;

        let (assign5290_e6159,) = {
    if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 == 0.0)) && (locals.var_guard81 == 0.0)) {
        let assign5290_e6152: f64 = (p.p2 / 2.0);
        let assign5290_e6154: f64 = (assign5290_e6152 - 1.0);
        let assign5290_e6156: f64 = (assign5290_e6154).max(0.0);
        let assign5290_e6157: f64 = (2.0 * assign5290_e6156);
        (assign5290_e6157,)
    } else {
        (locals.var_nuints,)
    }
};
        locals.var_nuints = assign5290_e6159;
        locals.var_nuints_rv = 0.0;

        let assign5300_e6162: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard82 = assign5300_e6162;
        locals.var_guard82_rv = 0.0;

        let assign5310_e6165: f64 = if locals.var_nuints == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard83 = assign5310_e6165;
        locals.var_guard83_rv = 0.0;

        let (assign5320_e6178,) = {
    if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard82 != 0.0)) && (locals.var_guard83 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign5320_e6178;
        locals.var_rint_rv = 0.0;

        let (assign5330_e6198,) = {
    if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard82 != 0.0)) && (locals.var_guard83 == 0.0)) {
        let assign5330_e6192: f64 = (p.p374 * locals.var_dmcgeff);
        let assign5330_e6195: f64 = (locals.var_weff * locals.var_nuints);
        let assign5330_e6196: f64 = (assign5330_e6192 / assign5330_e6195);
        (assign5330_e6196,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign5330_e6198;
        locals.var_rint_rv = 0.0;

        let assign5340_e6201: f64 = if locals.var_nuintd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard84 = assign5340_e6201;
        locals.var_guard84_rv = 0.0;

        let (assign5350_e6215,) = {
    if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard82 == 0.0)) && (locals.var_guard84 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign5350_e6215;
        locals.var_rint_rv = 0.0;

        let (assign5360_e6236,) = {
    if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard82 == 0.0)) && (locals.var_guard84 == 0.0)) {
        let assign5360_e6230: f64 = (p.p374 * locals.var_dmcgeff);
        let assign5360_e6233: f64 = (locals.var_weff * locals.var_nuintd);
        let assign5360_e6234: f64 = (assign5360_e6230 / assign5360_e6233);
        (assign5360_e6234,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign5360_e6236;
        locals.var_rint_rv = 0.0;

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

        let (assign5530_e6317,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && (locals.var_guard100 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5530_e6317;
        locals.var_rend_rv = 0.0;

        let (assign5540_e6341,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && (locals.var_guard100 == 0.0)) {
        let assign5540_e6335: f64 = (p.p374 * locals.var_dmcgeff);
        let assign5540_e6338: f64 = (locals.var_weff * locals.var_nuends);
        let assign5540_e6339: f64 = (assign5540_e6335 / assign5540_e6338);
        (assign5540_e6339,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5540_e6341;
        locals.var_rend_rv = 0.0;

        let assign5560_e6352: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign5560_e6355: f64 = if ((locals.var_nuends == 0.0) || (assign5560_e6352 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard102 = assign5560_e6355;
        locals.var_guard102_rv = 0.0;

        let (assign5570_e6375,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 != 0.0)) && ((locals.var_guard99 != 0.0) && (locals.var_guard98 == 0.0))) && (locals.var_guard102 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5570_e6375;
        locals.var_rend_rv = 0.0;

        let (assign5580_e6406,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 != 0.0)) && ((locals.var_guard99 != 0.0) && (locals.var_guard98 == 0.0))) && (locals.var_guard102 == 0.0)) {
        let assign5580_e6396: f64 = (p.p374 * locals.var_weff);
        let assign5580_e6399: f64 = (3.0 * locals.var_nuends);
        let assign5580_e6402: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign5580_e6403: f64 = (assign5580_e6399 * assign5580_e6402);
        let assign5580_e6404: f64 = (assign5580_e6396 / assign5580_e6403);
        (assign5580_e6404,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5580_e6406;
        locals.var_rend_rv = 0.0;

        let (assign5590_e6424,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 != 0.0)) && (!((locals.var_guard98 != 0.0) || (locals.var_guard99 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5590_e6424;
        locals.var_rend_rv = 0.0;

        let assign5600_e6435: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard103 = assign5600_e6435;
        locals.var_guard103_rv = 0.0;

        let assign5610_e6446: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard104 = assign5610_e6446;
        locals.var_guard104_rv = 0.0;

        let assign5620_e6449: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard105 = assign5620_e6449;
        locals.var_guard105_rv = 0.0;

        let (assign5630_e6467,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard103 != 0.0)) && (locals.var_guard105 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5630_e6467;
        locals.var_rend_rv = 0.0;

        let (assign5640_e6492,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard103 != 0.0)) && (locals.var_guard105 == 0.0)) {
        let assign5640_e6486: f64 = (p.p374 * locals.var_dmcgeff);
        let assign5640_e6489: f64 = (locals.var_weff * locals.var_nuends);
        let assign5640_e6490: f64 = (assign5640_e6486 / assign5640_e6489);
        (assign5640_e6490,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5640_e6492;
        locals.var_rend_rv = 0.0;

        let assign5660_e6503: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign5660_e6506: f64 = if ((locals.var_nuends == 0.0) || (assign5660_e6503 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard107 = assign5660_e6506;
        locals.var_guard107_rv = 0.0;

        let (assign5670_e6527,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 == 0.0)) && ((locals.var_guard104 != 0.0) && (locals.var_guard103 == 0.0))) && (locals.var_guard107 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5670_e6527;
        locals.var_rend_rv = 0.0;

        let (assign5680_e6559,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 == 0.0)) && ((locals.var_guard104 != 0.0) && (locals.var_guard103 == 0.0))) && (locals.var_guard107 == 0.0)) {
        let assign5680_e6549: f64 = (p.p374 * locals.var_weff);
        let assign5680_e6552: f64 = (3.0 * locals.var_nuends);
        let assign5680_e6555: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign5680_e6556: f64 = (assign5680_e6552 * assign5680_e6555);
        let assign5680_e6557: f64 = (assign5680_e6549 / assign5680_e6556);
        (assign5680_e6557,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5680_e6559;
        locals.var_rend_rv = 0.0;

        let (assign5690_e6578,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 == 0.0)) && (!((locals.var_guard103 != 0.0) || (locals.var_guard104 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5690_e6578;
        locals.var_rend_rv = 0.0;

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

        let (assign5740_e6624,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 == 0.0)) && (locals.var_guard108 != 0.0)) && (locals.var_guard109 != 0.0)) && (locals.var_guard111 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5740_e6624;
        locals.var_rend_rv = 0.0;

        let (assign5750_e6649,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 == 0.0)) && (locals.var_guard108 != 0.0)) && (locals.var_guard109 != 0.0)) && (locals.var_guard111 == 0.0)) {
        let assign5750_e6643: f64 = (p.p374 * locals.var_dmcgeff);
        let assign5750_e6646: f64 = (locals.var_weff * locals.var_nuendd);
        let assign5750_e6647: f64 = (assign5750_e6643 / assign5750_e6646);
        (assign5750_e6647,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5750_e6649;
        locals.var_rend_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_12(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign5770_e6660: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign5770_e6663: f64 = if ((locals.var_nuendd == 0.0) || (assign5770_e6660 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard113 = assign5770_e6663;
        locals.var_guard113_rv = 0.0;

        let (assign5780_e6684,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 == 0.0)) && (locals.var_guard108 != 0.0)) && ((locals.var_guard110 != 0.0) && (locals.var_guard109 == 0.0))) && (locals.var_guard113 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5780_e6684;
        locals.var_rend_rv = 0.0;

        let (assign5790_e6716,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 == 0.0)) && (locals.var_guard108 != 0.0)) && ((locals.var_guard110 != 0.0) && (locals.var_guard109 == 0.0))) && (locals.var_guard113 == 0.0)) {
        let assign5790_e6706: f64 = (p.p374 * locals.var_weff);
        let assign5790_e6709: f64 = (3.0 * locals.var_nuendd);
        let assign5790_e6712: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign5790_e6713: f64 = (assign5790_e6709 * assign5790_e6712);
        let assign5790_e6714: f64 = (assign5790_e6706 / assign5790_e6713);
        (assign5790_e6714,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5790_e6716;
        locals.var_rend_rv = 0.0;

        let (assign5800_e6735,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 == 0.0)) && (locals.var_guard108 != 0.0)) && (!((locals.var_guard109 != 0.0) || (locals.var_guard110 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5800_e6735;
        locals.var_rend_rv = 0.0;

        let assign5810_e6746: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard114 = assign5810_e6746;
        locals.var_guard114_rv = 0.0;

        let assign5820_e6757: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard115 = assign5820_e6757;
        locals.var_guard115_rv = 0.0;

        let assign5830_e6760: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard116 = assign5830_e6760;
        locals.var_guard116_rv = 0.0;

        let (assign5840_e6779,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 == 0.0)) && (locals.var_guard108 == 0.0)) && (locals.var_guard114 != 0.0)) && (locals.var_guard116 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5840_e6779;
        locals.var_rend_rv = 0.0;

        let (assign5850_e6805,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 == 0.0)) && (locals.var_guard108 == 0.0)) && (locals.var_guard114 != 0.0)) && (locals.var_guard116 == 0.0)) {
        let assign5850_e6799: f64 = (p.p374 * locals.var_dmcgeff);
        let assign5850_e6802: f64 = (locals.var_weff * locals.var_nuendd);
        let assign5850_e6803: f64 = (assign5850_e6799 / assign5850_e6802);
        (assign5850_e6803,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5850_e6805;
        locals.var_rend_rv = 0.0;

        let assign5870_e6816: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign5870_e6819: f64 = if ((locals.var_nuendd == 0.0) || (assign5870_e6816 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard118 = assign5870_e6819;
        locals.var_guard118_rv = 0.0;

        let (assign5880_e6841,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 == 0.0)) && (locals.var_guard108 == 0.0)) && ((locals.var_guard115 != 0.0) && (locals.var_guard114 == 0.0))) && (locals.var_guard118 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5880_e6841;
        locals.var_rend_rv = 0.0;

        let (assign5890_e6874,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 == 0.0)) && (locals.var_guard108 == 0.0)) && ((locals.var_guard115 != 0.0) && (locals.var_guard114 == 0.0))) && (locals.var_guard118 == 0.0)) {
        let assign5890_e6864: f64 = (p.p374 * locals.var_weff);
        let assign5890_e6867: f64 = (3.0 * locals.var_nuendd);
        let assign5890_e6870: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign5890_e6871: f64 = (assign5890_e6867 * assign5890_e6870);
        let assign5890_e6872: f64 = (assign5890_e6864 / assign5890_e6871);
        (assign5890_e6872,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5890_e6874;
        locals.var_rend_rv = 0.0;

        let (assign5900_e6894,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 == 0.0)) && (locals.var_guard108 == 0.0)) && (!((locals.var_guard114 != 0.0) || (locals.var_guard115 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5900_e6894;
        locals.var_rend_rv = 0.0;

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

        let (assign5960_e6945,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 != 0.0)) && (locals.var_guard120 != 0.0)) && (locals.var_guard121 != 0.0)) && (locals.var_guard123 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5960_e6945;
        locals.var_rend_rv = 0.0;

        let (assign5970_e6972,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 != 0.0)) && (locals.var_guard120 != 0.0)) && (locals.var_guard121 != 0.0)) && (locals.var_guard123 == 0.0)) {
        let assign5970_e6966: f64 = (p.p374 * locals.var_dmcgeff);
        let assign5970_e6969: f64 = (locals.var_weff * locals.var_nuends);
        let assign5970_e6970: f64 = (assign5970_e6966 / assign5970_e6969);
        (assign5970_e6970,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5970_e6972;
        locals.var_rend_rv = 0.0;

        let assign5990_e6983: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign5990_e6986: f64 = if ((locals.var_nuends == 0.0) || (assign5990_e6983 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard125 = assign5990_e6986;
        locals.var_guard125_rv = 0.0;

        let (assign6000_e7009,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 != 0.0)) && (locals.var_guard120 != 0.0)) && ((locals.var_guard122 != 0.0) && (locals.var_guard121 == 0.0))) && (locals.var_guard125 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6000_e7009;
        locals.var_rend_rv = 0.0;

        let (assign6010_e7043,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 != 0.0)) && (locals.var_guard120 != 0.0)) && ((locals.var_guard122 != 0.0) && (locals.var_guard121 == 0.0))) && (locals.var_guard125 == 0.0)) {
        let assign6010_e7033: f64 = (p.p374 * locals.var_weff);
        let assign6010_e7036: f64 = (3.0 * locals.var_nuends);
        let assign6010_e7039: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign6010_e7040: f64 = (assign6010_e7036 * assign6010_e7039);
        let assign6010_e7041: f64 = (assign6010_e7033 / assign6010_e7040);
        (assign6010_e7041,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6010_e7043;
        locals.var_rend_rv = 0.0;

        let (assign6020_e7064,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 != 0.0)) && (locals.var_guard120 != 0.0)) && (!((locals.var_guard121 != 0.0) || (locals.var_guard122 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6020_e7064;
        locals.var_rend_rv = 0.0;

        let assign6030_e7075: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard126 = assign6030_e7075;
        locals.var_guard126_rv = 0.0;

        let assign6040_e7086: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard127 = assign6040_e7086;
        locals.var_guard127_rv = 0.0;

        let assign6050_e7089: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard128 = assign6050_e7089;
        locals.var_guard128_rv = 0.0;

        let (assign6060_e7110,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 != 0.0)) && (locals.var_guard120 == 0.0)) && (locals.var_guard126 != 0.0)) && (locals.var_guard128 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6060_e7110;
        locals.var_rend_rv = 0.0;

        let (assign6070_e7138,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 != 0.0)) && (locals.var_guard120 == 0.0)) && (locals.var_guard126 != 0.0)) && (locals.var_guard128 == 0.0)) {
        let assign6070_e7132: f64 = (p.p374 * locals.var_dmcgeff);
        let assign6070_e7135: f64 = (locals.var_weff * locals.var_nuends);
        let assign6070_e7136: f64 = (assign6070_e7132 / assign6070_e7135);
        (assign6070_e7136,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6070_e7138;
        locals.var_rend_rv = 0.0;

        let assign6090_e7149: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign6090_e7152: f64 = if ((locals.var_nuends == 0.0) || (assign6090_e7149 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard130 = assign6090_e7152;
        locals.var_guard130_rv = 0.0;

        let (assign6100_e7176,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 != 0.0)) && (locals.var_guard120 == 0.0)) && ((locals.var_guard127 != 0.0) && (locals.var_guard126 == 0.0))) && (locals.var_guard130 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6100_e7176;
        locals.var_rend_rv = 0.0;

        let (assign6110_e7211,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 != 0.0)) && (locals.var_guard120 == 0.0)) && ((locals.var_guard127 != 0.0) && (locals.var_guard126 == 0.0))) && (locals.var_guard130 == 0.0)) {
        let assign6110_e7201: f64 = (p.p374 * locals.var_weff);
        let assign6110_e7204: f64 = (3.0 * locals.var_nuends);
        let assign6110_e7207: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign6110_e7208: f64 = (assign6110_e7204 * assign6110_e7207);
        let assign6110_e7209: f64 = (assign6110_e7201 / assign6110_e7208);
        (assign6110_e7209,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6110_e7211;
        locals.var_rend_rv = 0.0;

        let (assign6120_e7233,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 != 0.0)) && (locals.var_guard120 == 0.0)) && (!((locals.var_guard126 != 0.0) || (locals.var_guard127 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6120_e7233;
        locals.var_rend_rv = 0.0;

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

        let (assign6170_e7282,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 == 0.0)) && (locals.var_guard131 != 0.0)) && (locals.var_guard132 != 0.0)) && (locals.var_guard134 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6170_e7282;
        locals.var_rend_rv = 0.0;

        let (assign6180_e7310,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 == 0.0)) && (locals.var_guard131 != 0.0)) && (locals.var_guard132 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6180_e7304: f64 = (p.p374 * locals.var_dmcgeff);
        let assign6180_e7307: f64 = (locals.var_weff * locals.var_nuendd);
        let assign6180_e7308: f64 = (assign6180_e7304 / assign6180_e7307);
        (assign6180_e7308,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6180_e7310;
        locals.var_rend_rv = 0.0;

        let assign6200_e7320: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard136 = assign6200_e7320;
        locals.var_guard136_rv = 0.0;

        let (assign6210_e7344,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 == 0.0)) && (locals.var_guard131 != 0.0)) && ((locals.var_guard133 != 0.0) && (locals.var_guard132 == 0.0))) && (locals.var_guard136 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6210_e7344;
        locals.var_rend_rv = 0.0;

        let (assign6220_e7377,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 == 0.0)) && (locals.var_guard131 != 0.0)) && ((locals.var_guard133 != 0.0) && (locals.var_guard132 == 0.0))) && (locals.var_guard136 == 0.0)) {
        let assign6220_e7369: f64 = (p.p374 * locals.var_weff);
        let assign6220_e7372: f64 = (6.0 * locals.var_nuendd);
        let assign6220_e7374: f64 = (assign6220_e7372 * locals.var_dmcgeff);
        let assign6220_e7375: f64 = (assign6220_e7369 / assign6220_e7374);
        (assign6220_e7375,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6220_e7377;
        locals.var_rend_rv = 0.0;

        let (assign6230_e7399,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 == 0.0)) && (locals.var_guard131 != 0.0)) && (!((locals.var_guard132 != 0.0) || (locals.var_guard133 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6230_e7399;
        locals.var_rend_rv = 0.0;

        let assign6240_e7410: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard137 = assign6240_e7410;
        locals.var_guard137_rv = 0.0;

        let assign6250_e7421: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard138 = assign6250_e7421;
        locals.var_guard138_rv = 0.0;

        let assign6260_e7424: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard139 = assign6260_e7424;
        locals.var_guard139_rv = 0.0;

        let (assign6270_e7446,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 == 0.0)) && (locals.var_guard131 == 0.0)) && (locals.var_guard137 != 0.0)) && (locals.var_guard139 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6270_e7446;
        locals.var_rend_rv = 0.0;

        let (assign6280_e7475,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 == 0.0)) && (locals.var_guard131 == 0.0)) && (locals.var_guard137 != 0.0)) && (locals.var_guard139 == 0.0)) {
        let assign6280_e7469: f64 = (p.p374 * locals.var_dmcgeff);
        let assign6280_e7472: f64 = (locals.var_weff * locals.var_nuendd);
        let assign6280_e7473: f64 = (assign6280_e7469 / assign6280_e7472);
        (assign6280_e7473,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6280_e7475;
        locals.var_rend_rv = 0.0;

        let assign6300_e7485: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard141 = assign6300_e7485;
        locals.var_guard141_rv = 0.0;

        let (assign6310_e7510,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 == 0.0)) && (locals.var_guard131 == 0.0)) && ((locals.var_guard138 != 0.0) && (locals.var_guard137 == 0.0))) && (locals.var_guard141 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6310_e7510;
        locals.var_rend_rv = 0.0;

        let (assign6320_e7544,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 == 0.0)) && (locals.var_guard131 == 0.0)) && ((locals.var_guard138 != 0.0) && (locals.var_guard137 == 0.0))) && (locals.var_guard141 == 0.0)) {
        let assign6320_e7536: f64 = (p.p374 * locals.var_weff);
        let assign6320_e7539: f64 = (6.0 * locals.var_nuendd);
        let assign6320_e7541: f64 = (assign6320_e7539 * locals.var_dmcgeff);
        let assign6320_e7542: f64 = (assign6320_e7536 / assign6320_e7541);
        (assign6320_e7542,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6320_e7544;
        locals.var_rend_rv = 0.0;

        let (assign6330_e7567,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 == 0.0)) && (locals.var_guard131 == 0.0)) && (!((locals.var_guard137 != 0.0) || (locals.var_guard138 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6330_e7567;
        locals.var_rend_rv = 0.0;

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

        let (assign6390_e7620,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 != 0.0)) && (locals.var_guard143 != 0.0)) && (locals.var_guard144 != 0.0)) && (locals.var_guard146 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6390_e7620;
        locals.var_rend_rv = 0.0;

        let (assign6400_e7649,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 != 0.0)) && (locals.var_guard143 != 0.0)) && (locals.var_guard144 != 0.0)) && (locals.var_guard146 == 0.0)) {
        let assign6400_e7643: f64 = (p.p374 * locals.var_dmcgeff);
        let assign6400_e7646: f64 = (locals.var_weff * locals.var_nuends);
        let assign6400_e7647: f64 = (assign6400_e7643 / assign6400_e7646);
        (assign6400_e7647,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6400_e7649;
        locals.var_rend_rv = 0.0;

        let assign6420_e7659: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard148 = assign6420_e7659;
        locals.var_guard148_rv = 0.0;

        let (assign6430_e7684,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 != 0.0)) && (locals.var_guard143 != 0.0)) && ((locals.var_guard145 != 0.0) && (locals.var_guard144 == 0.0))) && (locals.var_guard148 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6430_e7684;
        locals.var_rend_rv = 0.0;

        let (assign6440_e7718,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 != 0.0)) && (locals.var_guard143 != 0.0)) && ((locals.var_guard145 != 0.0) && (locals.var_guard144 == 0.0))) && (locals.var_guard148 == 0.0)) {
        let assign6440_e7710: f64 = (p.p374 * locals.var_weff);
        let assign6440_e7713: f64 = (6.0 * locals.var_nuends);
        let assign6440_e7715: f64 = (assign6440_e7713 * locals.var_dmcgeff);
        let assign6440_e7716: f64 = (assign6440_e7710 / assign6440_e7715);
        (assign6440_e7716,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6440_e7718;
        locals.var_rend_rv = 0.0;

        let (assign6450_e7741,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 != 0.0)) && (locals.var_guard143 != 0.0)) && (!((locals.var_guard144 != 0.0) || (locals.var_guard145 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6450_e7741;
        locals.var_rend_rv = 0.0;

        let assign6460_e7752: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard149 = assign6460_e7752;
        locals.var_guard149_rv = 0.0;

        let assign6470_e7763: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard150 = assign6470_e7763;
        locals.var_guard150_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_13(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign6480_e7766: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard151 = assign6480_e7766;
        locals.var_guard151_rv = 0.0;

        let (assign6490_e7789,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 != 0.0)) && (locals.var_guard143 == 0.0)) && (locals.var_guard149 != 0.0)) && (locals.var_guard151 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6490_e7789;
        locals.var_rend_rv = 0.0;

        let (assign6500_e7819,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 != 0.0)) && (locals.var_guard143 == 0.0)) && (locals.var_guard149 != 0.0)) && (locals.var_guard151 == 0.0)) {
        let assign6500_e7813: f64 = (p.p374 * locals.var_dmcgeff);
        let assign6500_e7816: f64 = (locals.var_weff * locals.var_nuends);
        let assign6500_e7817: f64 = (assign6500_e7813 / assign6500_e7816);
        (assign6500_e7817,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6500_e7819;
        locals.var_rend_rv = 0.0;

        let assign6520_e7829: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard153 = assign6520_e7829;
        locals.var_guard153_rv = 0.0;

        let (assign6530_e7855,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 != 0.0)) && (locals.var_guard143 == 0.0)) && ((locals.var_guard150 != 0.0) && (locals.var_guard149 == 0.0))) && (locals.var_guard153 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6530_e7855;
        locals.var_rend_rv = 0.0;

        let (assign6540_e7890,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 != 0.0)) && (locals.var_guard143 == 0.0)) && ((locals.var_guard150 != 0.0) && (locals.var_guard149 == 0.0))) && (locals.var_guard153 == 0.0)) {
        let assign6540_e7882: f64 = (p.p374 * locals.var_weff);
        let assign6540_e7885: f64 = (6.0 * locals.var_nuends);
        let assign6540_e7887: f64 = (assign6540_e7885 * locals.var_dmcgeff);
        let assign6540_e7888: f64 = (assign6540_e7882 / assign6540_e7887);
        (assign6540_e7888,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6540_e7890;
        locals.var_rend_rv = 0.0;

        let (assign6550_e7914,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 != 0.0)) && (locals.var_guard143 == 0.0)) && (!((locals.var_guard149 != 0.0) || (locals.var_guard150 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6550_e7914;
        locals.var_rend_rv = 0.0;

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

        let (assign6600_e7965,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 == 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard155 != 0.0)) && (locals.var_guard157 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6600_e7965;
        locals.var_rend_rv = 0.0;

        let (assign6610_e7995,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 == 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard155 != 0.0)) && (locals.var_guard157 == 0.0)) {
        let assign6610_e7989: f64 = (p.p374 * locals.var_dmcgeff);
        let assign6610_e7992: f64 = (locals.var_weff * locals.var_nuendd);
        let assign6610_e7993: f64 = (assign6610_e7989 / assign6610_e7992);
        (assign6610_e7993,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6610_e7995;
        locals.var_rend_rv = 0.0;

        let assign6630_e8006: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign6630_e8009: f64 = if ((locals.var_nuendd == 0.0) || (assign6630_e8006 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard159 = assign6630_e8009;
        locals.var_guard159_rv = 0.0;

        let (assign6640_e8035,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 == 0.0)) && (locals.var_guard154 != 0.0)) && ((locals.var_guard156 != 0.0) && (locals.var_guard155 == 0.0))) && (locals.var_guard159 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6640_e8035;
        locals.var_rend_rv = 0.0;

        let (assign6650_e8072,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 == 0.0)) && (locals.var_guard154 != 0.0)) && ((locals.var_guard156 != 0.0) && (locals.var_guard155 == 0.0))) && (locals.var_guard159 == 0.0)) {
        let assign6650_e8062: f64 = (p.p374 * locals.var_weff);
        let assign6650_e8065: f64 = (3.0 * locals.var_nuendd);
        let assign6650_e8068: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign6650_e8069: f64 = (assign6650_e8065 * assign6650_e8068);
        let assign6650_e8070: f64 = (assign6650_e8062 / assign6650_e8069);
        (assign6650_e8070,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6650_e8072;
        locals.var_rend_rv = 0.0;

        let (assign6660_e8096,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 == 0.0)) && (locals.var_guard154 != 0.0)) && (!((locals.var_guard155 != 0.0) || (locals.var_guard156 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6660_e8096;
        locals.var_rend_rv = 0.0;

        let assign6670_e8107: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard160 = assign6670_e8107;
        locals.var_guard160_rv = 0.0;

        let assign6680_e8118: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard161 = assign6680_e8118;
        locals.var_guard161_rv = 0.0;

        let assign6690_e8121: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard162 = assign6690_e8121;
        locals.var_guard162_rv = 0.0;

        let (assign6700_e8145,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 == 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard160 != 0.0)) && (locals.var_guard162 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6700_e8145;
        locals.var_rend_rv = 0.0;

        let (assign6710_e8176,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 == 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard160 != 0.0)) && (locals.var_guard162 == 0.0)) {
        let assign6710_e8170: f64 = (p.p374 * locals.var_dmcgeff);
        let assign6710_e8173: f64 = (locals.var_weff * locals.var_nuendd);
        let assign6710_e8174: f64 = (assign6710_e8170 / assign6710_e8173);
        (assign6710_e8174,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6710_e8176;
        locals.var_rend_rv = 0.0;

        let assign6730_e8187: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign6730_e8190: f64 = if ((locals.var_nuendd == 0.0) || (assign6730_e8187 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard164 = assign6730_e8190;
        locals.var_guard164_rv = 0.0;

        let (assign6740_e8217,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 == 0.0)) && (locals.var_guard154 == 0.0)) && ((locals.var_guard161 != 0.0) && (locals.var_guard160 == 0.0))) && (locals.var_guard164 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6740_e8217;
        locals.var_rend_rv = 0.0;

        let (assign6750_e8255,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 == 0.0)) && (locals.var_guard154 == 0.0)) && ((locals.var_guard161 != 0.0) && (locals.var_guard160 == 0.0))) && (locals.var_guard164 == 0.0)) {
        let assign6750_e8245: f64 = (p.p374 * locals.var_weff);
        let assign6750_e8248: f64 = (3.0 * locals.var_nuendd);
        let assign6750_e8251: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign6750_e8252: f64 = (assign6750_e8248 * assign6750_e8251);
        let assign6750_e8253: f64 = (assign6750_e8245 / assign6750_e8252);
        (assign6750_e8253,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6750_e8255;
        locals.var_rend_rv = 0.0;

        let (assign6760_e8280,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 == 0.0)) && (locals.var_guard154 == 0.0)) && (!((locals.var_guard160 != 0.0) || (locals.var_guard161 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6760_e8280;
        locals.var_rend_rv = 0.0;

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

        let (assign6820_e8335,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 != 0.0)) && (locals.var_guard167 != 0.0)) && (locals.var_guard169 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6820_e8335;
        locals.var_rend_rv = 0.0;

        let (assign6830_e8366,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 != 0.0)) && (locals.var_guard167 != 0.0)) && (locals.var_guard169 == 0.0)) {
        let assign6830_e8360: f64 = (p.p374 * locals.var_dmcgeff);
        let assign6830_e8363: f64 = (locals.var_weff * locals.var_nuends);
        let assign6830_e8364: f64 = (assign6830_e8360 / assign6830_e8363);
        (assign6830_e8364,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6830_e8366;
        locals.var_rend_rv = 0.0;

        let assign6850_e8376: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard171 = assign6850_e8376;
        locals.var_guard171_rv = 0.0;

        let (assign6860_e8403,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 != 0.0)) && ((locals.var_guard168 != 0.0) && (locals.var_guard167 == 0.0))) && (locals.var_guard171 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6860_e8403;
        locals.var_rend_rv = 0.0;

        let (assign6870_e8439,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 != 0.0)) && ((locals.var_guard168 != 0.0) && (locals.var_guard167 == 0.0))) && (locals.var_guard171 == 0.0)) {
        let assign6870_e8431: f64 = (p.p374 * locals.var_weff);
        let assign6870_e8434: f64 = (6.0 * locals.var_nuends);
        let assign6870_e8436: f64 = (assign6870_e8434 * locals.var_dmcgeff);
        let assign6870_e8437: f64 = (assign6870_e8431 / assign6870_e8436);
        (assign6870_e8437,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6870_e8439;
        locals.var_rend_rv = 0.0;

        let (assign6880_e8464,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 != 0.0)) && (!((locals.var_guard167 != 0.0) || (locals.var_guard168 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6880_e8464;
        locals.var_rend_rv = 0.0;

        let assign6890_e8475: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard172 = assign6890_e8475;
        locals.var_guard172_rv = 0.0;

        let assign6900_e8486: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard173 = assign6900_e8486;
        locals.var_guard173_rv = 0.0;

        let assign6910_e8489: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard174 = assign6910_e8489;
        locals.var_guard174_rv = 0.0;

        let (assign6920_e8514,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard172 != 0.0)) && (locals.var_guard174 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6920_e8514;
        locals.var_rend_rv = 0.0;

        let (assign6930_e8546,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard172 != 0.0)) && (locals.var_guard174 == 0.0)) {
        let assign6930_e8540: f64 = (p.p374 * locals.var_dmcgeff);
        let assign6930_e8543: f64 = (locals.var_weff * locals.var_nuends);
        let assign6930_e8544: f64 = (assign6930_e8540 / assign6930_e8543);
        (assign6930_e8544,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6930_e8546;
        locals.var_rend_rv = 0.0;

        let assign6950_e8556: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard176 = assign6950_e8556;
        locals.var_guard176_rv = 0.0;

        let (assign6960_e8584,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 == 0.0)) && ((locals.var_guard173 != 0.0) && (locals.var_guard172 == 0.0))) && (locals.var_guard176 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6960_e8584;
        locals.var_rend_rv = 0.0;

        let (assign6970_e8621,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 == 0.0)) && ((locals.var_guard173 != 0.0) && (locals.var_guard172 == 0.0))) && (locals.var_guard176 == 0.0)) {
        let assign6970_e8613: f64 = (p.p374 * locals.var_weff);
        let assign6970_e8616: f64 = (6.0 * locals.var_nuends);
        let assign6970_e8618: f64 = (assign6970_e8616 * locals.var_dmcgeff);
        let assign6970_e8619: f64 = (assign6970_e8613 / assign6970_e8618);
        (assign6970_e8619,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6970_e8621;
        locals.var_rend_rv = 0.0;

        let (assign6980_e8647,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 == 0.0)) && (!((locals.var_guard172 != 0.0) || (locals.var_guard173 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6980_e8647;
        locals.var_rend_rv = 0.0;

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

        let (assign7030_e8700,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 == 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard178 != 0.0)) && (locals.var_guard180 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7030_e8700;
        locals.var_rend_rv = 0.0;

        let (assign7040_e8732,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 == 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard178 != 0.0)) && (locals.var_guard180 == 0.0)) {
        let assign7040_e8726: f64 = (p.p374 * locals.var_dmcgeff);
        let assign7040_e8729: f64 = (locals.var_weff * locals.var_nuendd);
        let assign7040_e8730: f64 = (assign7040_e8726 / assign7040_e8729);
        (assign7040_e8730,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7040_e8732;
        locals.var_rend_rv = 0.0;

        let assign7060_e8742: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard182 = assign7060_e8742;
        locals.var_guard182_rv = 0.0;

        let (assign7070_e8770,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 == 0.0)) && (locals.var_guard177 != 0.0)) && ((locals.var_guard179 != 0.0) && (locals.var_guard178 == 0.0))) && (locals.var_guard182 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7070_e8770;
        locals.var_rend_rv = 0.0;

        let (assign7080_e8807,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 == 0.0)) && (locals.var_guard177 != 0.0)) && ((locals.var_guard179 != 0.0) && (locals.var_guard178 == 0.0))) && (locals.var_guard182 == 0.0)) {
        let assign7080_e8799: f64 = (p.p374 * locals.var_weff);
        let assign7080_e8802: f64 = (6.0 * locals.var_nuendd);
        let assign7080_e8804: f64 = (assign7080_e8802 * locals.var_dmcgeff);
        let assign7080_e8805: f64 = (assign7080_e8799 / assign7080_e8804);
        (assign7080_e8805,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7080_e8807;
        locals.var_rend_rv = 0.0;

        let (assign7090_e8833,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 == 0.0)) && (locals.var_guard177 != 0.0)) && (!((locals.var_guard178 != 0.0) || (locals.var_guard179 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7090_e8833;
        locals.var_rend_rv = 0.0;

        let assign7100_e8844: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard183 = assign7100_e8844;
        locals.var_guard183_rv = 0.0;

        let assign7110_e8855: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard184 = assign7110_e8855;
        locals.var_guard184_rv = 0.0;

        let assign7120_e8858: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard185 = assign7120_e8858;
        locals.var_guard185_rv = 0.0;

        let (assign7130_e8884,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 == 0.0)) && (locals.var_guard177 == 0.0)) && (locals.var_guard183 != 0.0)) && (locals.var_guard185 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7130_e8884;
        locals.var_rend_rv = 0.0;

        let (assign7140_e8917,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 == 0.0)) && (locals.var_guard177 == 0.0)) && (locals.var_guard183 != 0.0)) && (locals.var_guard185 == 0.0)) {
        let assign7140_e8911: f64 = (p.p374 * locals.var_dmcgeff);
        let assign7140_e8914: f64 = (locals.var_weff * locals.var_nuendd);
        let assign7140_e8915: f64 = (assign7140_e8911 / assign7140_e8914);
        (assign7140_e8915,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7140_e8917;
        locals.var_rend_rv = 0.0;

        let assign7160_e8927: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard187 = assign7160_e8927;
        locals.var_guard187_rv = 0.0;

        let (assign7170_e8956,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 == 0.0)) && (locals.var_guard177 == 0.0)) && ((locals.var_guard184 != 0.0) && (locals.var_guard183 == 0.0))) && (locals.var_guard187 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7170_e8956;
        locals.var_rend_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_14(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign7180_e8994,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 == 0.0)) && (locals.var_guard177 == 0.0)) && ((locals.var_guard184 != 0.0) && (locals.var_guard183 == 0.0))) && (locals.var_guard187 == 0.0)) {
        let assign7180_e8986: f64 = (p.p374 * locals.var_weff);
        let assign7180_e8989: f64 = (6.0 * locals.var_nuendd);
        let assign7180_e8991: f64 = (assign7180_e8989 * locals.var_dmcgeff);
        let assign7180_e8992: f64 = (assign7180_e8986 / assign7180_e8991);
        (assign7180_e8992,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7180_e8994;
        locals.var_rend_rv = 0.0;

        let (assign7190_e9021,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 == 0.0)) && (locals.var_guard177 == 0.0)) && (!((locals.var_guard183 != 0.0) || (locals.var_guard184 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7190_e9021;
        locals.var_rend_rv = 0.0;

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

        let (assign7250_e9078,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 != 0.0)) && (locals.var_guard189 != 0.0)) && (locals.var_guard190 != 0.0)) && (locals.var_guard192 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7250_e9078;
        locals.var_rend_rv = 0.0;

        let (assign7260_e9111,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 != 0.0)) && (locals.var_guard189 != 0.0)) && (locals.var_guard190 != 0.0)) && (locals.var_guard192 == 0.0)) {
        let assign7260_e9105: f64 = (p.p374 * locals.var_dmcgeff);
        let assign7260_e9108: f64 = (locals.var_weff * locals.var_nuends);
        let assign7260_e9109: f64 = (assign7260_e9105 / assign7260_e9108);
        (assign7260_e9109,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7260_e9111;
        locals.var_rend_rv = 0.0;

        let assign7280_e9122: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign7280_e9125: f64 = if ((locals.var_nuends == 0.0) || (assign7280_e9122 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard194 = assign7280_e9125;
        locals.var_guard194_rv = 0.0;

        let (assign7290_e9154,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 != 0.0)) && (locals.var_guard189 != 0.0)) && ((locals.var_guard191 != 0.0) && (locals.var_guard190 == 0.0))) && (locals.var_guard194 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7290_e9154;
        locals.var_rend_rv = 0.0;

        let (assign7300_e9194,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 != 0.0)) && (locals.var_guard189 != 0.0)) && ((locals.var_guard191 != 0.0) && (locals.var_guard190 == 0.0))) && (locals.var_guard194 == 0.0)) {
        let assign7300_e9184: f64 = (p.p374 * locals.var_weff);
        let assign7300_e9187: f64 = (3.0 * locals.var_nuends);
        let assign7300_e9190: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign7300_e9191: f64 = (assign7300_e9187 * assign7300_e9190);
        let assign7300_e9192: f64 = (assign7300_e9184 / assign7300_e9191);
        (assign7300_e9192,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7300_e9194;
        locals.var_rend_rv = 0.0;

        let (assign7310_e9221,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 != 0.0)) && (locals.var_guard189 != 0.0)) && (!((locals.var_guard190 != 0.0) || (locals.var_guard191 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7310_e9221;
        locals.var_rend_rv = 0.0;

        let assign7320_e9232: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard195 = assign7320_e9232;
        locals.var_guard195_rv = 0.0;

        let assign7330_e9243: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard196 = assign7330_e9243;
        locals.var_guard196_rv = 0.0;

        let assign7340_e9246: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard197 = assign7340_e9246;
        locals.var_guard197_rv = 0.0;

        let (assign7350_e9273,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 != 0.0)) && (locals.var_guard189 == 0.0)) && (locals.var_guard195 != 0.0)) && (locals.var_guard197 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7350_e9273;
        locals.var_rend_rv = 0.0;

        let (assign7360_e9307,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 != 0.0)) && (locals.var_guard189 == 0.0)) && (locals.var_guard195 != 0.0)) && (locals.var_guard197 == 0.0)) {
        let assign7360_e9301: f64 = (p.p374 * locals.var_dmcgeff);
        let assign7360_e9304: f64 = (locals.var_weff * locals.var_nuends);
        let assign7360_e9305: f64 = (assign7360_e9301 / assign7360_e9304);
        (assign7360_e9305,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7360_e9307;
        locals.var_rend_rv = 0.0;

        let assign7380_e9318: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign7380_e9321: f64 = if ((locals.var_nuends == 0.0) || (assign7380_e9318 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard199 = assign7380_e9321;
        locals.var_guard199_rv = 0.0;

        let (assign7390_e9351,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 != 0.0)) && (locals.var_guard189 == 0.0)) && ((locals.var_guard196 != 0.0) && (locals.var_guard195 == 0.0))) && (locals.var_guard199 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7390_e9351;
        locals.var_rend_rv = 0.0;

        let (assign7400_e9392,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 != 0.0)) && (locals.var_guard189 == 0.0)) && ((locals.var_guard196 != 0.0) && (locals.var_guard195 == 0.0))) && (locals.var_guard199 == 0.0)) {
        let assign7400_e9382: f64 = (p.p374 * locals.var_weff);
        let assign7400_e9385: f64 = (3.0 * locals.var_nuends);
        let assign7400_e9388: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign7400_e9389: f64 = (assign7400_e9385 * assign7400_e9388);
        let assign7400_e9390: f64 = (assign7400_e9382 / assign7400_e9389);
        (assign7400_e9390,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7400_e9392;
        locals.var_rend_rv = 0.0;

        let (assign7410_e9420,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 != 0.0)) && (locals.var_guard189 == 0.0)) && (!((locals.var_guard195 != 0.0) || (locals.var_guard196 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7410_e9420;
        locals.var_rend_rv = 0.0;

        let (assign7420_e9445,) = {
    if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 == 0.0)) {
        let assign7420_e9441: f64 = (p.p374 * locals.var_dmdgeff);
        let assign7420_e9443: f64 = (assign7420_e9441 / locals.var_weff);
        (assign7420_e9443,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7420_e9445;
        locals.var_rend_rv = 0.0;

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

        let (assign7480_e9504,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 != 0.0)) && (locals.var_guard202 != 0.0)) && (locals.var_guard204 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7480_e9504;
        locals.var_rend_rv = 0.0;

        let (assign7490_e9539,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 != 0.0)) && (locals.var_guard202 != 0.0)) && (locals.var_guard204 == 0.0)) {
        let assign7490_e9533: f64 = (p.p374 * locals.var_dmcgeff);
        let assign7490_e9536: f64 = (locals.var_weff * locals.var_nuends);
        let assign7490_e9537: f64 = (assign7490_e9533 / assign7490_e9536);
        (assign7490_e9537,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7490_e9539;
        locals.var_rend_rv = 0.0;

        let assign7510_e9549: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard206 = assign7510_e9549;
        locals.var_guard206_rv = 0.0;

        let (assign7520_e9580,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 != 0.0)) && ((locals.var_guard203 != 0.0) && (locals.var_guard202 == 0.0))) && (locals.var_guard206 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7520_e9580;
        locals.var_rend_rv = 0.0;

        let (assign7530_e9620,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 != 0.0)) && ((locals.var_guard203 != 0.0) && (locals.var_guard202 == 0.0))) && (locals.var_guard206 == 0.0)) {
        let assign7530_e9612: f64 = (p.p374 * locals.var_weff);
        let assign7530_e9615: f64 = (6.0 * locals.var_nuends);
        let assign7530_e9617: f64 = (assign7530_e9615 * locals.var_dmcgeff);
        let assign7530_e9618: f64 = (assign7530_e9612 / assign7530_e9617);
        (assign7530_e9618,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7530_e9620;
        locals.var_rend_rv = 0.0;

        let (assign7540_e9649,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 != 0.0)) && (!((locals.var_guard202 != 0.0) || (locals.var_guard203 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7540_e9649;
        locals.var_rend_rv = 0.0;

        let assign7550_e9660: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard207 = assign7550_e9660;
        locals.var_guard207_rv = 0.0;

        let assign7560_e9671: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard208 = assign7560_e9671;
        locals.var_guard208_rv = 0.0;

        let assign7570_e9674: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard209 = assign7570_e9674;
        locals.var_guard209_rv = 0.0;

        let (assign7580_e9703,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 == 0.0)) && (locals.var_guard207 != 0.0)) && (locals.var_guard209 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7580_e9703;
        locals.var_rend_rv = 0.0;

        let (assign7590_e9739,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 == 0.0)) && (locals.var_guard207 != 0.0)) && (locals.var_guard209 == 0.0)) {
        let assign7590_e9733: f64 = (p.p374 * locals.var_dmcgeff);
        let assign7590_e9736: f64 = (locals.var_weff * locals.var_nuends);
        let assign7590_e9737: f64 = (assign7590_e9733 / assign7590_e9736);
        (assign7590_e9737,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7590_e9739;
        locals.var_rend_rv = 0.0;

        let assign7610_e9749: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard211 = assign7610_e9749;
        locals.var_guard211_rv = 0.0;

        let (assign7620_e9781,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 == 0.0)) && ((locals.var_guard208 != 0.0) && (locals.var_guard207 == 0.0))) && (locals.var_guard211 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7620_e9781;
        locals.var_rend_rv = 0.0;

        let (assign7630_e9822,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 == 0.0)) && ((locals.var_guard208 != 0.0) && (locals.var_guard207 == 0.0))) && (locals.var_guard211 == 0.0)) {
        let assign7630_e9814: f64 = (p.p374 * locals.var_weff);
        let assign7630_e9817: f64 = (6.0 * locals.var_nuends);
        let assign7630_e9819: f64 = (assign7630_e9817 * locals.var_dmcgeff);
        let assign7630_e9820: f64 = (assign7630_e9814 / assign7630_e9819);
        (assign7630_e9820,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7630_e9822;
        locals.var_rend_rv = 0.0;

        let (assign7640_e9852,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 == 0.0)) && (!((locals.var_guard207 != 0.0) || (locals.var_guard208 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7640_e9852;
        locals.var_rend_rv = 0.0;

        let assign7650_e9855: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard212 = assign7650_e9855;
        locals.var_guard212_rv = 0.0;

        let (assign7660_e9880,) = {
    if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 == 0.0)) && (locals.var_guard212 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7660_e9880;
        locals.var_rend_rv = 0.0;

        let (assign7670_e9912,) = {
    if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 == 0.0)) && (locals.var_guard212 == 0.0)) {
        let assign7670_e9906: f64 = (p.p374 * locals.var_dmdgeff);
        let assign7670_e9909: f64 = (locals.var_weff * locals.var_nuendd);
        let assign7670_e9910: f64 = (assign7670_e9906 / assign7670_e9909);
        (assign7670_e9910,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7670_e9912;
        locals.var_rend_rv = 0.0;

        let assign7680_e9915: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard213 = assign7680_e9915;
        locals.var_guard213_rv = 0.0;

        let (assign7690_e9943,) = {
    if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 != 0.0)) {
        let assign7690_e9939: f64 = (p.p374 * locals.var_dmdgeff);
        let assign7690_e9941: f64 = (assign7690_e9939 / locals.var_weff);
        (assign7690_e9941,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7690_e9943;
        locals.var_rend_rv = 0.0;

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

        let (assign7740_e10002,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 != 0.0)) && (locals.var_guard215 != 0.0)) && (locals.var_guard217 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7740_e10002;
        locals.var_rend_rv = 0.0;

        let (assign7750_e10040,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 != 0.0)) && (locals.var_guard215 != 0.0)) && (locals.var_guard217 == 0.0)) {
        let assign7750_e10034: f64 = (p.p374 * locals.var_dmcgeff);
        let assign7750_e10037: f64 = (locals.var_weff * locals.var_nuendd);
        let assign7750_e10038: f64 = (assign7750_e10034 / assign7750_e10037);
        (assign7750_e10038,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7750_e10040;
        locals.var_rend_rv = 0.0;

        let assign7770_e10051: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign7770_e10054: f64 = if ((locals.var_nuendd == 0.0) || (assign7770_e10051 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard219 = assign7770_e10054;
        locals.var_guard219_rv = 0.0;

        let (assign7780_e10088,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 != 0.0)) && ((locals.var_guard216 != 0.0) && (locals.var_guard215 == 0.0))) && (locals.var_guard219 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7780_e10088;
        locals.var_rend_rv = 0.0;

        let (assign7790_e10133,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 != 0.0)) && ((locals.var_guard216 != 0.0) && (locals.var_guard215 == 0.0))) && (locals.var_guard219 == 0.0)) {
        let assign7790_e10123: f64 = (p.p374 * locals.var_weff);
        let assign7790_e10126: f64 = (3.0 * locals.var_nuendd);
        let assign7790_e10129: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign7790_e10130: f64 = (assign7790_e10126 * assign7790_e10129);
        let assign7790_e10131: f64 = (assign7790_e10123 / assign7790_e10130);
        (assign7790_e10131,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7790_e10133;
        locals.var_rend_rv = 0.0;

        let (assign7800_e10165,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 != 0.0)) && (!((locals.var_guard215 != 0.0) || (locals.var_guard216 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7800_e10165;
        locals.var_rend_rv = 0.0;

        let assign7810_e10176: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard220 = assign7810_e10176;
        locals.var_guard220_rv = 0.0;

        let assign7820_e10187: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard221 = assign7820_e10187;
        locals.var_guard221_rv = 0.0;

        let assign7830_e10190: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard222 = assign7830_e10190;
        locals.var_guard222_rv = 0.0;

        let (assign7840_e10222,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 == 0.0)) && (locals.var_guard220 != 0.0)) && (locals.var_guard222 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7840_e10222;
        locals.var_rend_rv = 0.0;

        let (assign7850_e10261,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 == 0.0)) && (locals.var_guard220 != 0.0)) && (locals.var_guard222 == 0.0)) {
        let assign7850_e10255: f64 = (p.p374 * locals.var_dmcgeff);
        let assign7850_e10258: f64 = (locals.var_weff * locals.var_nuendd);
        let assign7850_e10259: f64 = (assign7850_e10255 / assign7850_e10258);
        (assign7850_e10259,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7850_e10261;
        locals.var_rend_rv = 0.0;

        let assign7870_e10272: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign7870_e10275: f64 = if ((locals.var_nuendd == 0.0) || (assign7870_e10272 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard224 = assign7870_e10275;
        locals.var_guard224_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_15(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign7880_e10310,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 == 0.0)) && ((locals.var_guard221 != 0.0) && (locals.var_guard220 == 0.0))) && (locals.var_guard224 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7880_e10310;
        locals.var_rend_rv = 0.0;

        let (assign7890_e10356,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 == 0.0)) && ((locals.var_guard221 != 0.0) && (locals.var_guard220 == 0.0))) && (locals.var_guard224 == 0.0)) {
        let assign7890_e10346: f64 = (p.p374 * locals.var_weff);
        let assign7890_e10349: f64 = (3.0 * locals.var_nuendd);
        let assign7890_e10352: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign7890_e10353: f64 = (assign7890_e10349 * assign7890_e10352);
        let assign7890_e10354: f64 = (assign7890_e10346 / assign7890_e10353);
        (assign7890_e10354,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7890_e10356;
        locals.var_rend_rv = 0.0;

        let (assign7900_e10389,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 == 0.0)) && (!((locals.var_guard220 != 0.0) || (locals.var_guard221 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7900_e10389;
        locals.var_rend_rv = 0.0;

        let assign7910_e10392: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard225 = assign7910_e10392;
        locals.var_guard225_rv = 0.0;

        let assign7920_e10395: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard226 = assign7920_e10395;
        locals.var_guard226_rv = 0.0;

        let (assign7930_e10423,) = {
    if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 != 0.0)) && (locals.var_guard226 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7930_e10423;
        locals.var_rend_rv = 0.0;

        let (assign7940_e10458,) = {
    if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 != 0.0)) && (locals.var_guard226 == 0.0)) {
        let assign7940_e10452: f64 = (p.p374 * locals.var_dmdgeff);
        let assign7940_e10455: f64 = (locals.var_weff * locals.var_nuends);
        let assign7940_e10456: f64 = (assign7940_e10452 / assign7940_e10455);
        (assign7940_e10456,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7940_e10458;
        locals.var_rend_rv = 0.0;

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

        let (assign7990_e10519,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 == 0.0)) && (locals.var_guard227 != 0.0)) && (locals.var_guard228 != 0.0)) && (locals.var_guard230 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7990_e10519;
        locals.var_rend_rv = 0.0;

        let (assign8000_e10559,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 == 0.0)) && (locals.var_guard227 != 0.0)) && (locals.var_guard228 != 0.0)) && (locals.var_guard230 == 0.0)) {
        let assign8000_e10553: f64 = (p.p374 * locals.var_dmcgeff);
        let assign8000_e10556: f64 = (locals.var_weff * locals.var_nuendd);
        let assign8000_e10557: f64 = (assign8000_e10553 / assign8000_e10556);
        (assign8000_e10557,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8000_e10559;
        locals.var_rend_rv = 0.0;

        let assign8020_e10569: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard232 = assign8020_e10569;
        locals.var_guard232_rv = 0.0;

        let (assign8030_e10605,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 == 0.0)) && (locals.var_guard227 != 0.0)) && ((locals.var_guard229 != 0.0) && (locals.var_guard228 == 0.0))) && (locals.var_guard232 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8030_e10605;
        locals.var_rend_rv = 0.0;

        let (assign8040_e10650,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 == 0.0)) && (locals.var_guard227 != 0.0)) && ((locals.var_guard229 != 0.0) && (locals.var_guard228 == 0.0))) && (locals.var_guard232 == 0.0)) {
        let assign8040_e10642: f64 = (p.p374 * locals.var_weff);
        let assign8040_e10645: f64 = (6.0 * locals.var_nuendd);
        let assign8040_e10647: f64 = (assign8040_e10645 * locals.var_dmcgeff);
        let assign8040_e10648: f64 = (assign8040_e10642 / assign8040_e10647);
        (assign8040_e10648,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8040_e10650;
        locals.var_rend_rv = 0.0;

        let (assign8050_e10684,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 == 0.0)) && (locals.var_guard227 != 0.0)) && (!((locals.var_guard228 != 0.0) || (locals.var_guard229 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8050_e10684;
        locals.var_rend_rv = 0.0;

        let assign8060_e10695: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard233 = assign8060_e10695;
        locals.var_guard233_rv = 0.0;

        let assign8070_e10706: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard234 = assign8070_e10706;
        locals.var_guard234_rv = 0.0;

        let assign8080_e10709: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard235 = assign8080_e10709;
        locals.var_guard235_rv = 0.0;

        let (assign8090_e10743,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 == 0.0)) && (locals.var_guard227 == 0.0)) && (locals.var_guard233 != 0.0)) && (locals.var_guard235 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8090_e10743;
        locals.var_rend_rv = 0.0;

        let (assign8100_e10784,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 == 0.0)) && (locals.var_guard227 == 0.0)) && (locals.var_guard233 != 0.0)) && (locals.var_guard235 == 0.0)) {
        let assign8100_e10778: f64 = (p.p374 * locals.var_dmcgeff);
        let assign8100_e10781: f64 = (locals.var_weff * locals.var_nuendd);
        let assign8100_e10782: f64 = (assign8100_e10778 / assign8100_e10781);
        (assign8100_e10782,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8100_e10784;
        locals.var_rend_rv = 0.0;

        let assign8120_e10794: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard237 = assign8120_e10794;
        locals.var_guard237_rv = 0.0;

        let (assign8130_e10831,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 == 0.0)) && (locals.var_guard227 == 0.0)) && ((locals.var_guard234 != 0.0) && (locals.var_guard233 == 0.0))) && (locals.var_guard237 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8130_e10831;
        locals.var_rend_rv = 0.0;

        let (assign8140_e10877,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 == 0.0)) && (locals.var_guard227 == 0.0)) && ((locals.var_guard234 != 0.0) && (locals.var_guard233 == 0.0))) && (locals.var_guard237 == 0.0)) {
        let assign8140_e10869: f64 = (p.p374 * locals.var_weff);
        let assign8140_e10872: f64 = (6.0 * locals.var_nuendd);
        let assign8140_e10874: f64 = (assign8140_e10872 * locals.var_dmcgeff);
        let assign8140_e10875: f64 = (assign8140_e10869 / assign8140_e10874);
        (assign8140_e10875,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8140_e10877;
        locals.var_rend_rv = 0.0;

        let (assign8150_e10912,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 == 0.0)) && (locals.var_guard227 == 0.0)) && (!((locals.var_guard233 != 0.0) || (locals.var_guard234 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8150_e10912;
        locals.var_rend_rv = 0.0;

        let (assign8160_e10942,) = {
    if (((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard93 != 0.0) && (!((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0))))) {
        let assign8160_e10938: f64 = (p.p374 * locals.var_dmdgeff);
        let assign8160_e10940: f64 = (assign8160_e10938 / locals.var_weff);
        (assign8160_e10940,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8160_e10942;
        locals.var_rend_rv = 0.0;

        let assign8170_e10945: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard238 = assign8170_e10945;
        locals.var_guard238_rv = 0.0;

        let (assign8180_e10981,) = {
    if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard94 != 0.0) && (!(((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0))))) && (locals.var_guard238 != 0.0)) {
        let assign8180_e10975: f64 = (0.5 * p.p374);
        let assign8180_e10977: f64 = (assign8180_e10975 * locals.var_dmcgeff);
        let assign8180_e10979: f64 = (assign8180_e10977 / locals.var_weff);
        (assign8180_e10979,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8180_e10981;
        locals.var_rend_rv = 0.0;

        let assign8190_e10984: f64 = if p.p2 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard239 = assign8190_e10984;
        locals.var_guard239_rv = 0.0;

        let (assign8200_e11016,) = {
    if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard94 != 0.0) && (!(((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0))))) && (locals.var_guard238 != 0.0)) && (locals.var_guard239 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign8200_e11016;
        locals.var_rint_rv = 0.0;

        let (assign8210_e11057,) = {
    if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard94 != 0.0) && (!(((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0))))) && (locals.var_guard238 != 0.0)) && (locals.var_guard239 == 0.0)) {
        let assign8210_e11049: f64 = (p.p374 * locals.var_dmcgeff);
        let assign8210_e11053: f64 = (p.p2 - 2.0);
        let assign8210_e11054: f64 = (locals.var_weff * assign8210_e11053);
        let assign8210_e11055: f64 = (assign8210_e11049 / assign8210_e11054);
        (assign8210_e11055,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign8210_e11057;
        locals.var_rint_rv = 0.0;

        let (assign8220_e11088,) = {
    if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard94 != 0.0) && (!(((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0))))) && (locals.var_guard238 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8220_e11088;
        locals.var_rend_rv = 0.0;

        let (assign8230_e11125,) = {
    if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard94 != 0.0) && (!(((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0))))) && (locals.var_guard238 == 0.0)) {
        let assign8230_e11119: f64 = (p.p374 * locals.var_dmcgeff);
        let assign8230_e11122: f64 = (locals.var_weff * p.p2);
        let assign8230_e11123: f64 = (assign8230_e11119 / assign8230_e11122);
        (assign8230_e11123,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign8230_e11125;
        locals.var_rint_rv = 0.0;

        let assign8240_e11128: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard240 = assign8240_e11128;
        locals.var_guard240_rv = 0.0;

        let (assign8250_e11160,) = {
    if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard95 != 0.0) && (!((((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0))))) && (locals.var_guard240 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8250_e11160;
        locals.var_rend_rv = 0.0;

        let (assign8260_e11198,) = {
    if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard95 != 0.0) && (!((((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0))))) && (locals.var_guard240 != 0.0)) {
        let assign8260_e11192: f64 = (p.p374 * locals.var_dmcgeff);
        let assign8260_e11195: f64 = (locals.var_weff * p.p2);
        let assign8260_e11196: f64 = (assign8260_e11192 / assign8260_e11195);
        (assign8260_e11196,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign8260_e11198;
        locals.var_rint_rv = 0.0;

        let (assign8270_e11237,) = {
    if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard95 != 0.0) && (!((((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0))))) && (locals.var_guard240 == 0.0)) {
        let assign8270_e11231: f64 = (0.5 * p.p374);
        let assign8270_e11233: f64 = (assign8270_e11231 * locals.var_dmcgeff);
        let assign8270_e11235: f64 = (assign8270_e11233 / locals.var_weff);
        (assign8270_e11235,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8270_e11237;
        locals.var_rend_rv = 0.0;

        let assign8280_e11240: f64 = if p.p2 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard241 = assign8280_e11240;
        locals.var_guard241_rv = 0.0;

        let (assign8290_e11275,) = {
    if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard95 != 0.0) && (!((((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0))))) && (locals.var_guard240 == 0.0)) && (locals.var_guard241 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign8290_e11275;
        locals.var_rint_rv = 0.0;

        let (assign8300_e11319,) = {
    if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard95 != 0.0) && (!((((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0))))) && (locals.var_guard240 == 0.0)) && (locals.var_guard241 == 0.0)) {
        let assign8300_e11311: f64 = (p.p374 * locals.var_dmcgeff);
        let assign8300_e11315: f64 = (p.p2 - 2.0);
        let assign8300_e11316: f64 = (locals.var_weff * assign8300_e11315);
        let assign8300_e11317: f64 = (assign8300_e11311 / assign8300_e11316);
        (assign8300_e11317,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign8300_e11319;
        locals.var_rint_rv = 0.0;

        let (assign8310_e11349,) = {
    if (((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (!(((((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0)) || (locals.var_guard95 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign8310_e11349;
        locals.var_rint_rv = 0.0;

        let assign8320_e11352: f64 = if locals.var_rint <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard242 = assign8320_e11352;
        locals.var_guard242_rv = 0.0;

        let (assign8330_e11361,) = {
    if (((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard242 != 0.0)) {
        (locals.var_rend,)
    } else {
        (locals.var_rsourcegeo,)
    }
};
        locals.var_rsourcegeo = assign8330_e11361;
        locals.var_rsourcegeo_rv = 0.0;

        let assign8340_e11364: f64 = if locals.var_rend <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard243 = assign8340_e11364;
        locals.var_guard243_rv = 0.0;

        let (assign8350_e11376,) = {
    if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard242 == 0.0)) && (locals.var_guard243 != 0.0)) {
        (locals.var_rint,)
    } else {
        (locals.var_rsourcegeo,)
    }
};
        locals.var_rsourcegeo = assign8350_e11376;
        locals.var_rsourcegeo_rv = 0.0;

        let (assign8360_e11395,) = {
    if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard242 == 0.0)) && (locals.var_guard243 == 0.0)) {
        let assign8360_e11389: f64 = (locals.var_rint * locals.var_rend);
        let assign8360_e11392: f64 = (locals.var_rint + locals.var_rend);
        let assign8360_e11393: f64 = (assign8360_e11389 / assign8360_e11392);
        (assign8360_e11393,)
    } else {
        (locals.var_rsourcegeo,)
    }
};
        locals.var_rsourcegeo = assign8360_e11395;
        locals.var_rsourcegeo_rv = 0.0;

        let (assign8380_e11406,) = {
    if ((locals.var_guard77 == 0.0) && (locals.var_guard78 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_rsourcegeo,)
    }
};
        locals.var_rsourcegeo = assign8380_e11406;
        locals.var_rsourcegeo_rv = 0.0;

        let assign8390_e11408: f64 = if param_given[4] { 1.0 } else { 0.0 };
        locals.var_guard245 = assign8390_e11408;
        locals.var_guard245_rv = 0.0;

        let (assign8400_e11414,) = {
    if (locals.var_guard245 != 0.0) {
        let assign8400_e11412: f64 = (p.p374 * p.p4);
        (assign8400_e11412,)
    } else {
        (locals.var_rdraingeo,)
    }
};
        locals.var_rdraingeo = assign8400_e11414;
        locals.var_rdraingeo_rv = 0.0;

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

        let (assign8440_e11440,) = {
    if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_nuendd,)
    }
};
        locals.var_nuendd = assign8440_e11440;
        locals.var_nuendd_rv = 0.0;

        let (assign8450_e11451,) = {
    if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_nuends,)
    }
};
        locals.var_nuends = assign8450_e11451;
        locals.var_nuends_rv = 0.0;

        let (assign8460_e11470,) = {
    if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) {
        let assign8460_e11463: f64 = (p.p2 - 1.0);
        let assign8460_e11465: f64 = (assign8460_e11463 / 2.0);
        let assign8460_e11467: f64 = (assign8460_e11465).max(0.0);
        let assign8460_e11468: f64 = (2.0 * assign8460_e11467);
        (assign8460_e11468,)
    } else {
        (locals.var_nuintd,)
    }
};
        locals.var_nuintd = assign8460_e11470;
        locals.var_nuintd_rv = 0.0;

        let (assign8470_e11481,) = {
    if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) {
        (locals.var_nuintd,)
    } else {
        (locals.var_nuints,)
    }
};
        locals.var_nuints = assign8470_e11481;
        locals.var_nuints_rv = 0.0;

        let assign8480_e11484: f64 = if p.p6 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard249 = assign8480_e11484;
        locals.var_guard249_rv = 0.0;

        let (assign8490_e11498,) = {
    if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 == 0.0)) && (locals.var_guard249 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_nuendd,)
    }
};
        locals.var_nuendd = assign8490_e11498;
        locals.var_nuendd_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_16(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign8500_e11520,) = {
    if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 == 0.0)) && (locals.var_guard249 != 0.0)) {
        let assign8500_e11513: f64 = (p.p2 / 2.0);
        let assign8500_e11515: f64 = (assign8500_e11513 - 1.0);
        let assign8500_e11517: f64 = (assign8500_e11515).max(0.0);
        let assign8500_e11518: f64 = (2.0 * assign8500_e11517);
        (assign8500_e11518,)
    } else {
        (locals.var_nuintd,)
    }
};
        locals.var_nuintd = assign8500_e11520;
        locals.var_nuintd_rv = 0.0;

        let (assign8510_e11534,) = {
    if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 == 0.0)) && (locals.var_guard249 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_nuends,)
    }
};
        locals.var_nuends = assign8510_e11534;
        locals.var_nuends_rv = 0.0;

        let (assign8520_e11548,) = {
    if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 == 0.0)) && (locals.var_guard249 != 0.0)) {
        (p.p2,)
    } else {
        (locals.var_nuints,)
    }
};
        locals.var_nuints = assign8520_e11548;
        locals.var_nuints_rv = 0.0;

        let (assign8530_e11563,) = {
    if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 == 0.0)) && (locals.var_guard249 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_nuendd,)
    }
};
        locals.var_nuendd = assign8530_e11563;
        locals.var_nuendd_rv = 0.0;

        let (assign8540_e11578,) = {
    if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 == 0.0)) && (locals.var_guard249 == 0.0)) {
        (p.p2,)
    } else {
        (locals.var_nuintd,)
    }
};
        locals.var_nuintd = assign8540_e11578;
        locals.var_nuintd_rv = 0.0;

        let (assign8550_e11593,) = {
    if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 == 0.0)) && (locals.var_guard249 == 0.0)) {
        (2.0,)
    } else {
        (locals.var_nuends,)
    }
};
        locals.var_nuends = assign8550_e11593;
        locals.var_nuends_rv = 0.0;

        let (assign8560_e11616,) = {
    if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 == 0.0)) && (locals.var_guard249 == 0.0)) {
        let assign8560_e11609: f64 = (p.p2 / 2.0);
        let assign8560_e11611: f64 = (assign8560_e11609 - 1.0);
        let assign8560_e11613: f64 = (assign8560_e11611).max(0.0);
        let assign8560_e11614: f64 = (2.0 * assign8560_e11613);
        (assign8560_e11614,)
    } else {
        (locals.var_nuints,)
    }
};
        locals.var_nuints = assign8560_e11616;
        locals.var_nuints_rv = 0.0;

        let assign8570_e11619: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard250 = assign8570_e11619;
        locals.var_guard250_rv = 0.0;

        let assign8580_e11622: f64 = if locals.var_nuints == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard251 = assign8580_e11622;
        locals.var_guard251_rv = 0.0;

        let (assign8590_e11635,) = {
    if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard250 != 0.0)) && (locals.var_guard251 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign8590_e11635;
        locals.var_rint_rv = 0.0;

        let (assign8600_e11655,) = {
    if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard250 != 0.0)) && (locals.var_guard251 == 0.0)) {
        let assign8600_e11649: f64 = (p.p374 * locals.var_dmcgeff);
        let assign8600_e11652: f64 = (locals.var_weff * locals.var_nuints);
        let assign8600_e11653: f64 = (assign8600_e11649 / assign8600_e11652);
        (assign8600_e11653,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign8600_e11655;
        locals.var_rint_rv = 0.0;

        let assign8610_e11658: f64 = if locals.var_nuintd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard252 = assign8610_e11658;
        locals.var_guard252_rv = 0.0;

        let (assign8620_e11672,) = {
    if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard250 == 0.0)) && (locals.var_guard252 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign8620_e11672;
        locals.var_rint_rv = 0.0;

        let (assign8630_e11693,) = {
    if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard250 == 0.0)) && (locals.var_guard252 == 0.0)) {
        let assign8630_e11687: f64 = (p.p374 * locals.var_dmcgeff);
        let assign8630_e11690: f64 = (locals.var_weff * locals.var_nuintd);
        let assign8630_e11691: f64 = (assign8630_e11687 / assign8630_e11690);
        (assign8630_e11691,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign8630_e11693;
        locals.var_rint_rv = 0.0;

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

        let (assign8800_e11774,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 != 0.0)) && (locals.var_guard265 != 0.0)) && (locals.var_guard266 != 0.0)) && (locals.var_guard268 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8800_e11774;
        locals.var_rend_rv = 0.0;

        let (assign8810_e11798,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 != 0.0)) && (locals.var_guard265 != 0.0)) && (locals.var_guard266 != 0.0)) && (locals.var_guard268 == 0.0)) {
        let assign8810_e11792: f64 = (p.p374 * locals.var_dmcgeff);
        let assign8810_e11795: f64 = (locals.var_weff * locals.var_nuends);
        let assign8810_e11796: f64 = (assign8810_e11792 / assign8810_e11795);
        (assign8810_e11796,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8810_e11798;
        locals.var_rend_rv = 0.0;

        let assign8830_e11809: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign8830_e11812: f64 = if ((locals.var_nuends == 0.0) || (assign8830_e11809 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard270 = assign8830_e11812;
        locals.var_guard270_rv = 0.0;

        let (assign8840_e11832,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 != 0.0)) && (locals.var_guard265 != 0.0)) && ((locals.var_guard267 != 0.0) && (locals.var_guard266 == 0.0))) && (locals.var_guard270 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8840_e11832;
        locals.var_rend_rv = 0.0;

        let (assign8850_e11863,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 != 0.0)) && (locals.var_guard265 != 0.0)) && ((locals.var_guard267 != 0.0) && (locals.var_guard266 == 0.0))) && (locals.var_guard270 == 0.0)) {
        let assign8850_e11853: f64 = (p.p374 * locals.var_weff);
        let assign8850_e11856: f64 = (3.0 * locals.var_nuends);
        let assign8850_e11859: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign8850_e11860: f64 = (assign8850_e11856 * assign8850_e11859);
        let assign8850_e11861: f64 = (assign8850_e11853 / assign8850_e11860);
        (assign8850_e11861,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8850_e11863;
        locals.var_rend_rv = 0.0;

        let (assign8860_e11881,) = {
    if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 != 0.0)) && (locals.var_guard265 != 0.0)) && (!((locals.var_guard266 != 0.0) || (locals.var_guard267 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8860_e11881;
        locals.var_rend_rv = 0.0;

        let assign8870_e11892: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard271 = assign8870_e11892;
        locals.var_guard271_rv = 0.0;

        let assign8880_e11903: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard272 = assign8880_e11903;
        locals.var_guard272_rv = 0.0;

        let assign8890_e11906: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard273 = assign8890_e11906;
        locals.var_guard273_rv = 0.0;

        let (assign8900_e11924,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard271 != 0.0)) && (locals.var_guard273 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8900_e11924;
        locals.var_rend_rv = 0.0;

        let (assign8910_e11949,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard271 != 0.0)) && (locals.var_guard273 == 0.0)) {
        let assign8910_e11943: f64 = (p.p374 * locals.var_dmcgeff);
        let assign8910_e11946: f64 = (locals.var_weff * locals.var_nuends);
        let assign8910_e11947: f64 = (assign8910_e11943 / assign8910_e11946);
        (assign8910_e11947,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8910_e11949;
        locals.var_rend_rv = 0.0;

        let assign8930_e11960: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign8930_e11963: f64 = if ((locals.var_nuends == 0.0) || (assign8930_e11960 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard275 = assign8930_e11963;
        locals.var_guard275_rv = 0.0;

        let (assign8940_e11984,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 != 0.0)) && (locals.var_guard265 == 0.0)) && ((locals.var_guard272 != 0.0) && (locals.var_guard271 == 0.0))) && (locals.var_guard275 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8940_e11984;
        locals.var_rend_rv = 0.0;

        let (assign8950_e12016,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 != 0.0)) && (locals.var_guard265 == 0.0)) && ((locals.var_guard272 != 0.0) && (locals.var_guard271 == 0.0))) && (locals.var_guard275 == 0.0)) {
        let assign8950_e12006: f64 = (p.p374 * locals.var_weff);
        let assign8950_e12009: f64 = (3.0 * locals.var_nuends);
        let assign8950_e12012: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign8950_e12013: f64 = (assign8950_e12009 * assign8950_e12012);
        let assign8950_e12014: f64 = (assign8950_e12006 / assign8950_e12013);
        (assign8950_e12014,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8950_e12016;
        locals.var_rend_rv = 0.0;

        let (assign8960_e12035,) = {
    if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 != 0.0)) && (locals.var_guard265 == 0.0)) && (!((locals.var_guard271 != 0.0) || (locals.var_guard272 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8960_e12035;
        locals.var_rend_rv = 0.0;

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

        let (assign9010_e12081,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 == 0.0)) && (locals.var_guard276 != 0.0)) && (locals.var_guard277 != 0.0)) && (locals.var_guard279 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9010_e12081;
        locals.var_rend_rv = 0.0;

        let (assign9020_e12106,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 == 0.0)) && (locals.var_guard276 != 0.0)) && (locals.var_guard277 != 0.0)) && (locals.var_guard279 == 0.0)) {
        let assign9020_e12100: f64 = (p.p374 * locals.var_dmcgeff);
        let assign9020_e12103: f64 = (locals.var_weff * locals.var_nuendd);
        let assign9020_e12104: f64 = (assign9020_e12100 / assign9020_e12103);
        (assign9020_e12104,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9020_e12106;
        locals.var_rend_rv = 0.0;

        let assign9040_e12117: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign9040_e12120: f64 = if ((locals.var_nuendd == 0.0) || (assign9040_e12117 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard281 = assign9040_e12120;
        locals.var_guard281_rv = 0.0;

        let (assign9050_e12141,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 == 0.0)) && (locals.var_guard276 != 0.0)) && ((locals.var_guard278 != 0.0) && (locals.var_guard277 == 0.0))) && (locals.var_guard281 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9050_e12141;
        locals.var_rend_rv = 0.0;

        let (assign9060_e12173,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 == 0.0)) && (locals.var_guard276 != 0.0)) && ((locals.var_guard278 != 0.0) && (locals.var_guard277 == 0.0))) && (locals.var_guard281 == 0.0)) {
        let assign9060_e12163: f64 = (p.p374 * locals.var_weff);
        let assign9060_e12166: f64 = (3.0 * locals.var_nuendd);
        let assign9060_e12169: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign9060_e12170: f64 = (assign9060_e12166 * assign9060_e12169);
        let assign9060_e12171: f64 = (assign9060_e12163 / assign9060_e12170);
        (assign9060_e12171,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9060_e12173;
        locals.var_rend_rv = 0.0;

        let (assign9070_e12192,) = {
    if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 == 0.0)) && (locals.var_guard276 != 0.0)) && (!((locals.var_guard277 != 0.0) || (locals.var_guard278 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9070_e12192;
        locals.var_rend_rv = 0.0;

        let assign9080_e12203: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard282 = assign9080_e12203;
        locals.var_guard282_rv = 0.0;

        let assign9090_e12214: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard283 = assign9090_e12214;
        locals.var_guard283_rv = 0.0;

        let assign9100_e12217: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard284 = assign9100_e12217;
        locals.var_guard284_rv = 0.0;

        let (assign9110_e12236,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 == 0.0)) && (locals.var_guard276 == 0.0)) && (locals.var_guard282 != 0.0)) && (locals.var_guard284 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9110_e12236;
        locals.var_rend_rv = 0.0;

        let (assign9120_e12262,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 == 0.0)) && (locals.var_guard276 == 0.0)) && (locals.var_guard282 != 0.0)) && (locals.var_guard284 == 0.0)) {
        let assign9120_e12256: f64 = (p.p374 * locals.var_dmcgeff);
        let assign9120_e12259: f64 = (locals.var_weff * locals.var_nuendd);
        let assign9120_e12260: f64 = (assign9120_e12256 / assign9120_e12259);
        (assign9120_e12260,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9120_e12262;
        locals.var_rend_rv = 0.0;

        let assign9140_e12273: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign9140_e12276: f64 = if ((locals.var_nuendd == 0.0) || (assign9140_e12273 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard286 = assign9140_e12276;
        locals.var_guard286_rv = 0.0;

        let (assign9150_e12298,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 == 0.0)) && (locals.var_guard276 == 0.0)) && ((locals.var_guard283 != 0.0) && (locals.var_guard282 == 0.0))) && (locals.var_guard286 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9150_e12298;
        locals.var_rend_rv = 0.0;

        let (assign9160_e12331,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 == 0.0)) && (locals.var_guard276 == 0.0)) && ((locals.var_guard283 != 0.0) && (locals.var_guard282 == 0.0))) && (locals.var_guard286 == 0.0)) {
        let assign9160_e12321: f64 = (p.p374 * locals.var_weff);
        let assign9160_e12324: f64 = (3.0 * locals.var_nuendd);
        let assign9160_e12327: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign9160_e12328: f64 = (assign9160_e12324 * assign9160_e12327);
        let assign9160_e12329: f64 = (assign9160_e12321 / assign9160_e12328);
        (assign9160_e12329,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9160_e12331;
        locals.var_rend_rv = 0.0;

        let (assign9170_e12351,) = {
    if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 == 0.0)) && (locals.var_guard276 == 0.0)) && (!((locals.var_guard282 != 0.0) || (locals.var_guard283 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9170_e12351;
        locals.var_rend_rv = 0.0;

        let assign9180_e12354: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard287 = assign9180_e12354;
        locals.var_guard287_rv = 0.0;

        let assign9190_e12357: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard288 = assign9190_e12357;
        locals.var_guard288_rv = 0.0;

        let assign9200_e12368: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard289 = assign9200_e12368;
        locals.var_guard289_rv = 0.0;

        let assign9210_e12379: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard290 = assign9210_e12379;
        locals.var_guard290_rv = 0.0;

        let assign9220_e12382: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard291 = assign9220_e12382;
        locals.var_guard291_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_17(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign9230_e12402,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 != 0.0)) && (locals.var_guard288 != 0.0)) && (locals.var_guard289 != 0.0)) && (locals.var_guard291 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9230_e12402;
        locals.var_rend_rv = 0.0;

        let (assign9240_e12429,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 != 0.0)) && (locals.var_guard288 != 0.0)) && (locals.var_guard289 != 0.0)) && (locals.var_guard291 == 0.0)) {
        let assign9240_e12423: f64 = (p.p374 * locals.var_dmcgeff);
        let assign9240_e12426: f64 = (locals.var_weff * locals.var_nuends);
        let assign9240_e12427: f64 = (assign9240_e12423 / assign9240_e12426);
        (assign9240_e12427,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9240_e12429;
        locals.var_rend_rv = 0.0;

        let assign9260_e12440: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign9260_e12443: f64 = if ((locals.var_nuends == 0.0) || (assign9260_e12440 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard293 = assign9260_e12443;
        locals.var_guard293_rv = 0.0;

        let (assign9270_e12466,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 != 0.0)) && (locals.var_guard288 != 0.0)) && ((locals.var_guard290 != 0.0) && (locals.var_guard289 == 0.0))) && (locals.var_guard293 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9270_e12466;
        locals.var_rend_rv = 0.0;

        let (assign9280_e12500,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 != 0.0)) && (locals.var_guard288 != 0.0)) && ((locals.var_guard290 != 0.0) && (locals.var_guard289 == 0.0))) && (locals.var_guard293 == 0.0)) {
        let assign9280_e12490: f64 = (p.p374 * locals.var_weff);
        let assign9280_e12493: f64 = (3.0 * locals.var_nuends);
        let assign9280_e12496: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign9280_e12497: f64 = (assign9280_e12493 * assign9280_e12496);
        let assign9280_e12498: f64 = (assign9280_e12490 / assign9280_e12497);
        (assign9280_e12498,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9280_e12500;
        locals.var_rend_rv = 0.0;

        let (assign9290_e12521,) = {
    if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 != 0.0)) && (locals.var_guard288 != 0.0)) && (!((locals.var_guard289 != 0.0) || (locals.var_guard290 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9290_e12521;
        locals.var_rend_rv = 0.0;

        let assign9300_e12532: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard294 = assign9300_e12532;
        locals.var_guard294_rv = 0.0;

        let assign9310_e12543: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard295 = assign9310_e12543;
        locals.var_guard295_rv = 0.0;

        let assign9320_e12546: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard296 = assign9320_e12546;
        locals.var_guard296_rv = 0.0;

        let (assign9330_e12567,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 != 0.0)) && (locals.var_guard288 == 0.0)) && (locals.var_guard294 != 0.0)) && (locals.var_guard296 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9330_e12567;
        locals.var_rend_rv = 0.0;

        let (assign9340_e12595,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 != 0.0)) && (locals.var_guard288 == 0.0)) && (locals.var_guard294 != 0.0)) && (locals.var_guard296 == 0.0)) {
        let assign9340_e12589: f64 = (p.p374 * locals.var_dmcgeff);
        let assign9340_e12592: f64 = (locals.var_weff * locals.var_nuends);
        let assign9340_e12593: f64 = (assign9340_e12589 / assign9340_e12592);
        (assign9340_e12593,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9340_e12595;
        locals.var_rend_rv = 0.0;

        let assign9360_e12606: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign9360_e12609: f64 = if ((locals.var_nuends == 0.0) || (assign9360_e12606 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard298 = assign9360_e12609;
        locals.var_guard298_rv = 0.0;

        let (assign9370_e12633,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 != 0.0)) && (locals.var_guard288 == 0.0)) && ((locals.var_guard295 != 0.0) && (locals.var_guard294 == 0.0))) && (locals.var_guard298 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9370_e12633;
        locals.var_rend_rv = 0.0;

        let (assign9380_e12668,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 != 0.0)) && (locals.var_guard288 == 0.0)) && ((locals.var_guard295 != 0.0) && (locals.var_guard294 == 0.0))) && (locals.var_guard298 == 0.0)) {
        let assign9380_e12658: f64 = (p.p374 * locals.var_weff);
        let assign9380_e12661: f64 = (3.0 * locals.var_nuends);
        let assign9380_e12664: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign9380_e12665: f64 = (assign9380_e12661 * assign9380_e12664);
        let assign9380_e12666: f64 = (assign9380_e12658 / assign9380_e12665);
        (assign9380_e12666,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9380_e12668;
        locals.var_rend_rv = 0.0;

        let (assign9390_e12690,) = {
    if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 != 0.0)) && (locals.var_guard288 == 0.0)) && (!((locals.var_guard294 != 0.0) || (locals.var_guard295 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9390_e12690;
        locals.var_rend_rv = 0.0;

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

        let (assign9440_e12739,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 == 0.0)) && (locals.var_guard299 != 0.0)) && (locals.var_guard300 != 0.0)) && (locals.var_guard302 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9440_e12739;
        locals.var_rend_rv = 0.0;

        let (assign9450_e12767,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 == 0.0)) && (locals.var_guard299 != 0.0)) && (locals.var_guard300 != 0.0)) && (locals.var_guard302 == 0.0)) {
        let assign9450_e12761: f64 = (p.p374 * locals.var_dmcgeff);
        let assign9450_e12764: f64 = (locals.var_weff * locals.var_nuendd);
        let assign9450_e12765: f64 = (assign9450_e12761 / assign9450_e12764);
        (assign9450_e12765,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9450_e12767;
        locals.var_rend_rv = 0.0;

        let assign9470_e12777: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard304 = assign9470_e12777;
        locals.var_guard304_rv = 0.0;

        let (assign9480_e12801,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 == 0.0)) && (locals.var_guard299 != 0.0)) && ((locals.var_guard301 != 0.0) && (locals.var_guard300 == 0.0))) && (locals.var_guard304 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9480_e12801;
        locals.var_rend_rv = 0.0;

        let (assign9490_e12834,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 == 0.0)) && (locals.var_guard299 != 0.0)) && ((locals.var_guard301 != 0.0) && (locals.var_guard300 == 0.0))) && (locals.var_guard304 == 0.0)) {
        let assign9490_e12826: f64 = (p.p374 * locals.var_weff);
        let assign9490_e12829: f64 = (6.0 * locals.var_nuendd);
        let assign9490_e12831: f64 = (assign9490_e12829 * locals.var_dmcgeff);
        let assign9490_e12832: f64 = (assign9490_e12826 / assign9490_e12831);
        (assign9490_e12832,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9490_e12834;
        locals.var_rend_rv = 0.0;

        let (assign9500_e12856,) = {
    if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 == 0.0)) && (locals.var_guard299 != 0.0)) && (!((locals.var_guard300 != 0.0) || (locals.var_guard301 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9500_e12856;
        locals.var_rend_rv = 0.0;

        let assign9510_e12867: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard305 = assign9510_e12867;
        locals.var_guard305_rv = 0.0;

        let assign9520_e12878: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard306 = assign9520_e12878;
        locals.var_guard306_rv = 0.0;

        let assign9530_e12881: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard307 = assign9530_e12881;
        locals.var_guard307_rv = 0.0;

        let (assign9540_e12903,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 == 0.0)) && (locals.var_guard299 == 0.0)) && (locals.var_guard305 != 0.0)) && (locals.var_guard307 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9540_e12903;
        locals.var_rend_rv = 0.0;

        let (assign9550_e12932,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 == 0.0)) && (locals.var_guard299 == 0.0)) && (locals.var_guard305 != 0.0)) && (locals.var_guard307 == 0.0)) {
        let assign9550_e12926: f64 = (p.p374 * locals.var_dmcgeff);
        let assign9550_e12929: f64 = (locals.var_weff * locals.var_nuendd);
        let assign9550_e12930: f64 = (assign9550_e12926 / assign9550_e12929);
        (assign9550_e12930,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9550_e12932;
        locals.var_rend_rv = 0.0;

        let assign9570_e12942: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard309 = assign9570_e12942;
        locals.var_guard309_rv = 0.0;

        let (assign9580_e12967,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 == 0.0)) && (locals.var_guard299 == 0.0)) && ((locals.var_guard306 != 0.0) && (locals.var_guard305 == 0.0))) && (locals.var_guard309 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9580_e12967;
        locals.var_rend_rv = 0.0;

        let (assign9590_e13001,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 == 0.0)) && (locals.var_guard299 == 0.0)) && ((locals.var_guard306 != 0.0) && (locals.var_guard305 == 0.0))) && (locals.var_guard309 == 0.0)) {
        let assign9590_e12993: f64 = (p.p374 * locals.var_weff);
        let assign9590_e12996: f64 = (6.0 * locals.var_nuendd);
        let assign9590_e12998: f64 = (assign9590_e12996 * locals.var_dmcgeff);
        let assign9590_e12999: f64 = (assign9590_e12993 / assign9590_e12998);
        (assign9590_e12999,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9590_e13001;
        locals.var_rend_rv = 0.0;

        let (assign9600_e13024,) = {
    if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 == 0.0)) && (locals.var_guard299 == 0.0)) && (!((locals.var_guard305 != 0.0) || (locals.var_guard306 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9600_e13024;
        locals.var_rend_rv = 0.0;

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

        let (assign9660_e13077,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 != 0.0)) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 != 0.0)) && (locals.var_guard314 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9660_e13077;
        locals.var_rend_rv = 0.0;

        let (assign9670_e13106,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 != 0.0)) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 != 0.0)) && (locals.var_guard314 == 0.0)) {
        let assign9670_e13100: f64 = (p.p374 * locals.var_dmcgeff);
        let assign9670_e13103: f64 = (locals.var_weff * locals.var_nuends);
        let assign9670_e13104: f64 = (assign9670_e13100 / assign9670_e13103);
        (assign9670_e13104,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9670_e13106;
        locals.var_rend_rv = 0.0;

        let assign9690_e13116: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard316 = assign9690_e13116;
        locals.var_guard316_rv = 0.0;

        let (assign9700_e13141,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 != 0.0)) && (locals.var_guard311 != 0.0)) && ((locals.var_guard313 != 0.0) && (locals.var_guard312 == 0.0))) && (locals.var_guard316 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9700_e13141;
        locals.var_rend_rv = 0.0;

        let (assign9710_e13175,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 != 0.0)) && (locals.var_guard311 != 0.0)) && ((locals.var_guard313 != 0.0) && (locals.var_guard312 == 0.0))) && (locals.var_guard316 == 0.0)) {
        let assign9710_e13167: f64 = (p.p374 * locals.var_weff);
        let assign9710_e13170: f64 = (6.0 * locals.var_nuends);
        let assign9710_e13172: f64 = (assign9710_e13170 * locals.var_dmcgeff);
        let assign9710_e13173: f64 = (assign9710_e13167 / assign9710_e13172);
        (assign9710_e13173,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9710_e13175;
        locals.var_rend_rv = 0.0;

        let (assign9720_e13198,) = {
    if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 != 0.0)) && (locals.var_guard311 != 0.0)) && (!((locals.var_guard312 != 0.0) || (locals.var_guard313 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9720_e13198;
        locals.var_rend_rv = 0.0;

        let assign9730_e13209: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard317 = assign9730_e13209;
        locals.var_guard317_rv = 0.0;

        let assign9740_e13220: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard318 = assign9740_e13220;
        locals.var_guard318_rv = 0.0;

        let assign9750_e13223: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard319 = assign9750_e13223;
        locals.var_guard319_rv = 0.0;

        let (assign9760_e13246,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 != 0.0)) && (locals.var_guard311 == 0.0)) && (locals.var_guard317 != 0.0)) && (locals.var_guard319 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9760_e13246;
        locals.var_rend_rv = 0.0;

        let (assign9770_e13276,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 != 0.0)) && (locals.var_guard311 == 0.0)) && (locals.var_guard317 != 0.0)) && (locals.var_guard319 == 0.0)) {
        let assign9770_e13270: f64 = (p.p374 * locals.var_dmcgeff);
        let assign9770_e13273: f64 = (locals.var_weff * locals.var_nuends);
        let assign9770_e13274: f64 = (assign9770_e13270 / assign9770_e13273);
        (assign9770_e13274,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9770_e13276;
        locals.var_rend_rv = 0.0;

        let assign9790_e13286: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard321 = assign9790_e13286;
        locals.var_guard321_rv = 0.0;

        let (assign9800_e13312,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 != 0.0)) && (locals.var_guard311 == 0.0)) && ((locals.var_guard318 != 0.0) && (locals.var_guard317 == 0.0))) && (locals.var_guard321 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9800_e13312;
        locals.var_rend_rv = 0.0;

        let (assign9810_e13347,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 != 0.0)) && (locals.var_guard311 == 0.0)) && ((locals.var_guard318 != 0.0) && (locals.var_guard317 == 0.0))) && (locals.var_guard321 == 0.0)) {
        let assign9810_e13339: f64 = (p.p374 * locals.var_weff);
        let assign9810_e13342: f64 = (6.0 * locals.var_nuends);
        let assign9810_e13344: f64 = (assign9810_e13342 * locals.var_dmcgeff);
        let assign9810_e13345: f64 = (assign9810_e13339 / assign9810_e13344);
        (assign9810_e13345,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9810_e13347;
        locals.var_rend_rv = 0.0;

        let (assign9820_e13371,) = {
    if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 != 0.0)) && (locals.var_guard311 == 0.0)) && (!((locals.var_guard317 != 0.0) || (locals.var_guard318 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9820_e13371;
        locals.var_rend_rv = 0.0;

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

        let (assign9870_e13422,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 == 0.0)) && (locals.var_guard322 != 0.0)) && (locals.var_guard323 != 0.0)) && (locals.var_guard325 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9870_e13422;
        locals.var_rend_rv = 0.0;

        let (assign9880_e13452,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 == 0.0)) && (locals.var_guard322 != 0.0)) && (locals.var_guard323 != 0.0)) && (locals.var_guard325 == 0.0)) {
        let assign9880_e13446: f64 = (p.p374 * locals.var_dmcgeff);
        let assign9880_e13449: f64 = (locals.var_weff * locals.var_nuendd);
        let assign9880_e13450: f64 = (assign9880_e13446 / assign9880_e13449);
        (assign9880_e13450,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9880_e13452;
        locals.var_rend_rv = 0.0;

        let assign9900_e13463: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign9900_e13466: f64 = if ((locals.var_nuendd == 0.0) || (assign9900_e13463 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard327 = assign9900_e13466;
        locals.var_guard327_rv = 0.0;

        let (assign9910_e13492,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 == 0.0)) && (locals.var_guard322 != 0.0)) && ((locals.var_guard324 != 0.0) && (locals.var_guard323 == 0.0))) && (locals.var_guard327 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9910_e13492;
        locals.var_rend_rv = 0.0;

        let (assign9920_e13529,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 == 0.0)) && (locals.var_guard322 != 0.0)) && ((locals.var_guard324 != 0.0) && (locals.var_guard323 == 0.0))) && (locals.var_guard327 == 0.0)) {
        let assign9920_e13519: f64 = (p.p374 * locals.var_weff);
        let assign9920_e13522: f64 = (3.0 * locals.var_nuendd);
        let assign9920_e13525: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign9920_e13526: f64 = (assign9920_e13522 * assign9920_e13525);
        let assign9920_e13527: f64 = (assign9920_e13519 / assign9920_e13526);
        (assign9920_e13527,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9920_e13529;
        locals.var_rend_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_18(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign9930_e13553,) = {
    if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 == 0.0)) && (locals.var_guard322 != 0.0)) && (!((locals.var_guard323 != 0.0) || (locals.var_guard324 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9930_e13553;
        locals.var_rend_rv = 0.0;

        let assign9940_e13564: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard328 = assign9940_e13564;
        locals.var_guard328_rv = 0.0;

        let assign9950_e13575: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard329 = assign9950_e13575;
        locals.var_guard329_rv = 0.0;

        let assign9960_e13578: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard330 = assign9960_e13578;
        locals.var_guard330_rv = 0.0;

        let (assign9970_e13602,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 == 0.0)) && (locals.var_guard322 == 0.0)) && (locals.var_guard328 != 0.0)) && (locals.var_guard330 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9970_e13602;
        locals.var_rend_rv = 0.0;

        let (assign9980_e13633,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 == 0.0)) && (locals.var_guard322 == 0.0)) && (locals.var_guard328 != 0.0)) && (locals.var_guard330 == 0.0)) {
        let assign9980_e13627: f64 = (p.p374 * locals.var_dmcgeff);
        let assign9980_e13630: f64 = (locals.var_weff * locals.var_nuendd);
        let assign9980_e13631: f64 = (assign9980_e13627 / assign9980_e13630);
        (assign9980_e13631,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9980_e13633;
        locals.var_rend_rv = 0.0;

        let assign10000_e13644: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign10000_e13647: f64 = if ((locals.var_nuendd == 0.0) || (assign10000_e13644 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard332 = assign10000_e13647;
        locals.var_guard332_rv = 0.0;

        let (assign10010_e13674,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 == 0.0)) && (locals.var_guard322 == 0.0)) && ((locals.var_guard329 != 0.0) && (locals.var_guard328 == 0.0))) && (locals.var_guard332 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10010_e13674;
        locals.var_rend_rv = 0.0;

        let (assign10020_e13712,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 == 0.0)) && (locals.var_guard322 == 0.0)) && ((locals.var_guard329 != 0.0) && (locals.var_guard328 == 0.0))) && (locals.var_guard332 == 0.0)) {
        let assign10020_e13702: f64 = (p.p374 * locals.var_weff);
        let assign10020_e13705: f64 = (3.0 * locals.var_nuendd);
        let assign10020_e13708: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign10020_e13709: f64 = (assign10020_e13705 * assign10020_e13708);
        let assign10020_e13710: f64 = (assign10020_e13702 / assign10020_e13709);
        (assign10020_e13710,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10020_e13712;
        locals.var_rend_rv = 0.0;

        let (assign10030_e13737,) = {
    if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 == 0.0)) && (locals.var_guard322 == 0.0)) && (!((locals.var_guard328 != 0.0) || (locals.var_guard329 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10030_e13737;
        locals.var_rend_rv = 0.0;

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

        let (assign10090_e13792,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 != 0.0)) && (locals.var_guard337 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10090_e13792;
        locals.var_rend_rv = 0.0;

        let (assign10100_e13823,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 != 0.0)) && (locals.var_guard337 == 0.0)) {
        let assign10100_e13817: f64 = (p.p374 * locals.var_dmcgeff);
        let assign10100_e13820: f64 = (locals.var_weff * locals.var_nuends);
        let assign10100_e13821: f64 = (assign10100_e13817 / assign10100_e13820);
        (assign10100_e13821,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10100_e13823;
        locals.var_rend_rv = 0.0;

        let assign10120_e13833: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard339 = assign10120_e13833;
        locals.var_guard339_rv = 0.0;

        let (assign10130_e13860,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 != 0.0)) && ((locals.var_guard336 != 0.0) && (locals.var_guard335 == 0.0))) && (locals.var_guard339 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10130_e13860;
        locals.var_rend_rv = 0.0;

        let (assign10140_e13896,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 != 0.0)) && ((locals.var_guard336 != 0.0) && (locals.var_guard335 == 0.0))) && (locals.var_guard339 == 0.0)) {
        let assign10140_e13888: f64 = (p.p374 * locals.var_weff);
        let assign10140_e13891: f64 = (6.0 * locals.var_nuends);
        let assign10140_e13893: f64 = (assign10140_e13891 * locals.var_dmcgeff);
        let assign10140_e13894: f64 = (assign10140_e13888 / assign10140_e13893);
        (assign10140_e13894,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10140_e13896;
        locals.var_rend_rv = 0.0;

        let (assign10150_e13921,) = {
    if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 != 0.0)) && (!((locals.var_guard335 != 0.0) || (locals.var_guard336 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10150_e13921;
        locals.var_rend_rv = 0.0;

        let assign10160_e13932: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard340 = assign10160_e13932;
        locals.var_guard340_rv = 0.0;

        let assign10170_e13943: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard341 = assign10170_e13943;
        locals.var_guard341_rv = 0.0;

        let assign10180_e13946: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard342 = assign10180_e13946;
        locals.var_guard342_rv = 0.0;

        let (assign10190_e13971,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 == 0.0)) && (locals.var_guard340 != 0.0)) && (locals.var_guard342 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10190_e13971;
        locals.var_rend_rv = 0.0;

        let (assign10200_e14003,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 == 0.0)) && (locals.var_guard340 != 0.0)) && (locals.var_guard342 == 0.0)) {
        let assign10200_e13997: f64 = (p.p374 * locals.var_dmcgeff);
        let assign10200_e14000: f64 = (locals.var_weff * locals.var_nuends);
        let assign10200_e14001: f64 = (assign10200_e13997 / assign10200_e14000);
        (assign10200_e14001,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10200_e14003;
        locals.var_rend_rv = 0.0;

        let assign10220_e14013: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard344 = assign10220_e14013;
        locals.var_guard344_rv = 0.0;

        let (assign10230_e14041,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 == 0.0)) && ((locals.var_guard341 != 0.0) && (locals.var_guard340 == 0.0))) && (locals.var_guard344 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10230_e14041;
        locals.var_rend_rv = 0.0;

        let (assign10240_e14078,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 == 0.0)) && ((locals.var_guard341 != 0.0) && (locals.var_guard340 == 0.0))) && (locals.var_guard344 == 0.0)) {
        let assign10240_e14070: f64 = (p.p374 * locals.var_weff);
        let assign10240_e14073: f64 = (6.0 * locals.var_nuends);
        let assign10240_e14075: f64 = (assign10240_e14073 * locals.var_dmcgeff);
        let assign10240_e14076: f64 = (assign10240_e14070 / assign10240_e14075);
        (assign10240_e14076,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10240_e14078;
        locals.var_rend_rv = 0.0;

        let (assign10250_e14104,) = {
    if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 == 0.0)) && (!((locals.var_guard340 != 0.0) || (locals.var_guard341 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10250_e14104;
        locals.var_rend_rv = 0.0;

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

        let (assign10300_e14157,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 == 0.0)) && (locals.var_guard345 != 0.0)) && (locals.var_guard346 != 0.0)) && (locals.var_guard348 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10300_e14157;
        locals.var_rend_rv = 0.0;

        let (assign10310_e14189,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 == 0.0)) && (locals.var_guard345 != 0.0)) && (locals.var_guard346 != 0.0)) && (locals.var_guard348 == 0.0)) {
        let assign10310_e14183: f64 = (p.p374 * locals.var_dmcgeff);
        let assign10310_e14186: f64 = (locals.var_weff * locals.var_nuendd);
        let assign10310_e14187: f64 = (assign10310_e14183 / assign10310_e14186);
        (assign10310_e14187,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10310_e14189;
        locals.var_rend_rv = 0.0;

        let assign10330_e14199: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard350 = assign10330_e14199;
        locals.var_guard350_rv = 0.0;

        let (assign10340_e14227,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 == 0.0)) && (locals.var_guard345 != 0.0)) && ((locals.var_guard347 != 0.0) && (locals.var_guard346 == 0.0))) && (locals.var_guard350 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10340_e14227;
        locals.var_rend_rv = 0.0;

        let (assign10350_e14264,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 == 0.0)) && (locals.var_guard345 != 0.0)) && ((locals.var_guard347 != 0.0) && (locals.var_guard346 == 0.0))) && (locals.var_guard350 == 0.0)) {
        let assign10350_e14256: f64 = (p.p374 * locals.var_weff);
        let assign10350_e14259: f64 = (6.0 * locals.var_nuendd);
        let assign10350_e14261: f64 = (assign10350_e14259 * locals.var_dmcgeff);
        let assign10350_e14262: f64 = (assign10350_e14256 / assign10350_e14261);
        (assign10350_e14262,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10350_e14264;
        locals.var_rend_rv = 0.0;

        let (assign10360_e14290,) = {
    if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 == 0.0)) && (locals.var_guard345 != 0.0)) && (!((locals.var_guard346 != 0.0) || (locals.var_guard347 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10360_e14290;
        locals.var_rend_rv = 0.0;

        let assign10370_e14301: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard351 = assign10370_e14301;
        locals.var_guard351_rv = 0.0;

        let assign10380_e14312: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard352 = assign10380_e14312;
        locals.var_guard352_rv = 0.0;

        let assign10390_e14315: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard353 = assign10390_e14315;
        locals.var_guard353_rv = 0.0;

        let (assign10400_e14341,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 == 0.0)) && (locals.var_guard345 == 0.0)) && (locals.var_guard351 != 0.0)) && (locals.var_guard353 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10400_e14341;
        locals.var_rend_rv = 0.0;

        let (assign10410_e14374,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 == 0.0)) && (locals.var_guard345 == 0.0)) && (locals.var_guard351 != 0.0)) && (locals.var_guard353 == 0.0)) {
        let assign10410_e14368: f64 = (p.p374 * locals.var_dmcgeff);
        let assign10410_e14371: f64 = (locals.var_weff * locals.var_nuendd);
        let assign10410_e14372: f64 = (assign10410_e14368 / assign10410_e14371);
        (assign10410_e14372,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10410_e14374;
        locals.var_rend_rv = 0.0;

        let assign10430_e14384: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard355 = assign10430_e14384;
        locals.var_guard355_rv = 0.0;

        let (assign10440_e14413,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 == 0.0)) && (locals.var_guard345 == 0.0)) && ((locals.var_guard352 != 0.0) && (locals.var_guard351 == 0.0))) && (locals.var_guard355 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10440_e14413;
        locals.var_rend_rv = 0.0;

        let (assign10450_e14451,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 == 0.0)) && (locals.var_guard345 == 0.0)) && ((locals.var_guard352 != 0.0) && (locals.var_guard351 == 0.0))) && (locals.var_guard355 == 0.0)) {
        let assign10450_e14443: f64 = (p.p374 * locals.var_weff);
        let assign10450_e14446: f64 = (6.0 * locals.var_nuendd);
        let assign10450_e14448: f64 = (assign10450_e14446 * locals.var_dmcgeff);
        let assign10450_e14449: f64 = (assign10450_e14443 / assign10450_e14448);
        (assign10450_e14449,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10450_e14451;
        locals.var_rend_rv = 0.0;

        let (assign10460_e14478,) = {
    if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 == 0.0)) && (locals.var_guard345 == 0.0)) && (!((locals.var_guard351 != 0.0) || (locals.var_guard352 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10460_e14478;
        locals.var_rend_rv = 0.0;

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

        let (assign10520_e14535,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard257 != 0.0) && (!((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard356 != 0.0)) && (locals.var_guard357 != 0.0)) && (locals.var_guard358 != 0.0)) && (locals.var_guard360 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10520_e14535;
        locals.var_rend_rv = 0.0;

        let (assign10530_e14568,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard257 != 0.0) && (!((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard356 != 0.0)) && (locals.var_guard357 != 0.0)) && (locals.var_guard358 != 0.0)) && (locals.var_guard360 == 0.0)) {
        let assign10530_e14562: f64 = (p.p374 * locals.var_dmcgeff);
        let assign10530_e14565: f64 = (locals.var_weff * locals.var_nuends);
        let assign10530_e14566: f64 = (assign10530_e14562 / assign10530_e14565);
        (assign10530_e14566,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10530_e14568;
        locals.var_rend_rv = 0.0;

        let assign10550_e14579: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign10550_e14582: f64 = if ((locals.var_nuends == 0.0) || (assign10550_e14579 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard362 = assign10550_e14582;
        locals.var_guard362_rv = 0.0;

        let (assign10560_e14611,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard257 != 0.0) && (!((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard356 != 0.0)) && (locals.var_guard357 != 0.0)) && ((locals.var_guard359 != 0.0) && (locals.var_guard358 == 0.0))) && (locals.var_guard362 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10560_e14611;
        locals.var_rend_rv = 0.0;

        let (assign10570_e14651,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard257 != 0.0) && (!((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard356 != 0.0)) && (locals.var_guard357 != 0.0)) && ((locals.var_guard359 != 0.0) && (locals.var_guard358 == 0.0))) && (locals.var_guard362 == 0.0)) {
        let assign10570_e14641: f64 = (p.p374 * locals.var_weff);
        let assign10570_e14644: f64 = (3.0 * locals.var_nuends);
        let assign10570_e14647: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign10570_e14648: f64 = (assign10570_e14644 * assign10570_e14647);
        let assign10570_e14649: f64 = (assign10570_e14641 / assign10570_e14648);
        (assign10570_e14649,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10570_e14651;
        locals.var_rend_rv = 0.0;

        let (assign10580_e14678,) = {
    if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard257 != 0.0) && (!((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard356 != 0.0)) && (locals.var_guard357 != 0.0)) && (!((locals.var_guard358 != 0.0) || (locals.var_guard359 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10580_e14678;
        locals.var_rend_rv = 0.0;

        let assign10590_e14689: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard363 = assign10590_e14689;
        locals.var_guard363_rv = 0.0;

        let assign10600_e14700: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard364 = assign10600_e14700;
        locals.var_guard364_rv = 0.0;

        let assign10610_e14703: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard365 = assign10610_e14703;
        locals.var_guard365_rv = 0.0;

        let (assign10620_e14730,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard257 != 0.0) && (!((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard356 != 0.0)) && (locals.var_guard357 == 0.0)) && (locals.var_guard363 != 0.0)) && (locals.var_guard365 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10620_e14730;
        locals.var_rend_rv = 0.0;

        let (assign10630_e14764,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard257 != 0.0) && (!((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard356 != 0.0)) && (locals.var_guard357 == 0.0)) && (locals.var_guard363 != 0.0)) && (locals.var_guard365 == 0.0)) {
        let assign10630_e14758: f64 = (p.p374 * locals.var_dmcgeff);
        let assign10630_e14761: f64 = (locals.var_weff * locals.var_nuends);
        let assign10630_e14762: f64 = (assign10630_e14758 / assign10630_e14761);
        (assign10630_e14762,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10630_e14764;
        locals.var_rend_rv = 0.0;

        let assign10650_e14775: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign10650_e14778: f64 = if ((locals.var_nuends == 0.0) || (assign10650_e14775 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard367 = assign10650_e14778;
        locals.var_guard367_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_19(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign10660_e14808,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard257 != 0.0) && (!((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard356 != 0.0)) && (locals.var_guard357 == 0.0)) && ((locals.var_guard364 != 0.0) && (locals.var_guard363 == 0.0))) && (locals.var_guard367 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10660_e14808;
        locals.var_rend_rv = 0.0;

        let (assign10670_e14849,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard257 != 0.0) && (!((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard356 != 0.0)) && (locals.var_guard357 == 0.0)) && ((locals.var_guard364 != 0.0) && (locals.var_guard363 == 0.0))) && (locals.var_guard367 == 0.0)) {
        let assign10670_e14839: f64 = (p.p374 * locals.var_weff);
        let assign10670_e14842: f64 = (3.0 * locals.var_nuends);
        let assign10670_e14845: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign10670_e14846: f64 = (assign10670_e14842 * assign10670_e14845);
        let assign10670_e14847: f64 = (assign10670_e14839 / assign10670_e14846);
        (assign10670_e14847,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10670_e14849;
        locals.var_rend_rv = 0.0;

        let (assign10680_e14877,) = {
    if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard257 != 0.0) && (!((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard356 != 0.0)) && (locals.var_guard357 == 0.0)) && (!((locals.var_guard363 != 0.0) || (locals.var_guard364 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10680_e14877;
        locals.var_rend_rv = 0.0;

        let (assign10690_e14902,) = {
    if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard257 != 0.0) && (!((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard356 == 0.0)) {
        let assign10690_e14898: f64 = (p.p374 * locals.var_dmdgeff);
        let assign10690_e14900: f64 = (assign10690_e14898 / locals.var_weff);
        (assign10690_e14900,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10690_e14902;
        locals.var_rend_rv = 0.0;

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

        let (assign10750_e14961,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 != 0.0)) && (locals.var_guard369 != 0.0)) && (locals.var_guard370 != 0.0)) && (locals.var_guard372 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10750_e14961;
        locals.var_rend_rv = 0.0;

        let (assign10760_e14996,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 != 0.0)) && (locals.var_guard369 != 0.0)) && (locals.var_guard370 != 0.0)) && (locals.var_guard372 == 0.0)) {
        let assign10760_e14990: f64 = (p.p374 * locals.var_dmcgeff);
        let assign10760_e14993: f64 = (locals.var_weff * locals.var_nuends);
        let assign10760_e14994: f64 = (assign10760_e14990 / assign10760_e14993);
        (assign10760_e14994,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10760_e14996;
        locals.var_rend_rv = 0.0;

        let assign10780_e15006: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard374 = assign10780_e15006;
        locals.var_guard374_rv = 0.0;

        let (assign10790_e15037,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 != 0.0)) && (locals.var_guard369 != 0.0)) && ((locals.var_guard371 != 0.0) && (locals.var_guard370 == 0.0))) && (locals.var_guard374 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10790_e15037;
        locals.var_rend_rv = 0.0;

        let (assign10800_e15077,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 != 0.0)) && (locals.var_guard369 != 0.0)) && ((locals.var_guard371 != 0.0) && (locals.var_guard370 == 0.0))) && (locals.var_guard374 == 0.0)) {
        let assign10800_e15069: f64 = (p.p374 * locals.var_weff);
        let assign10800_e15072: f64 = (6.0 * locals.var_nuends);
        let assign10800_e15074: f64 = (assign10800_e15072 * locals.var_dmcgeff);
        let assign10800_e15075: f64 = (assign10800_e15069 / assign10800_e15074);
        (assign10800_e15075,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10800_e15077;
        locals.var_rend_rv = 0.0;

        let (assign10810_e15106,) = {
    if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 != 0.0)) && (locals.var_guard369 != 0.0)) && (!((locals.var_guard370 != 0.0) || (locals.var_guard371 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10810_e15106;
        locals.var_rend_rv = 0.0;

        let assign10820_e15117: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard375 = assign10820_e15117;
        locals.var_guard375_rv = 0.0;

        let assign10830_e15128: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard376 = assign10830_e15128;
        locals.var_guard376_rv = 0.0;

        let assign10840_e15131: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard377 = assign10840_e15131;
        locals.var_guard377_rv = 0.0;

        let (assign10850_e15160,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 != 0.0)) && (locals.var_guard369 == 0.0)) && (locals.var_guard375 != 0.0)) && (locals.var_guard377 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10850_e15160;
        locals.var_rend_rv = 0.0;

        let (assign10860_e15196,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 != 0.0)) && (locals.var_guard369 == 0.0)) && (locals.var_guard375 != 0.0)) && (locals.var_guard377 == 0.0)) {
        let assign10860_e15190: f64 = (p.p374 * locals.var_dmcgeff);
        let assign10860_e15193: f64 = (locals.var_weff * locals.var_nuends);
        let assign10860_e15194: f64 = (assign10860_e15190 / assign10860_e15193);
        (assign10860_e15194,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10860_e15196;
        locals.var_rend_rv = 0.0;

        let assign10880_e15206: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard379 = assign10880_e15206;
        locals.var_guard379_rv = 0.0;

        let (assign10890_e15238,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 != 0.0)) && (locals.var_guard369 == 0.0)) && ((locals.var_guard376 != 0.0) && (locals.var_guard375 == 0.0))) && (locals.var_guard379 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10890_e15238;
        locals.var_rend_rv = 0.0;

        let (assign10900_e15279,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 != 0.0)) && (locals.var_guard369 == 0.0)) && ((locals.var_guard376 != 0.0) && (locals.var_guard375 == 0.0))) && (locals.var_guard379 == 0.0)) {
        let assign10900_e15271: f64 = (p.p374 * locals.var_weff);
        let assign10900_e15274: f64 = (6.0 * locals.var_nuends);
        let assign10900_e15276: f64 = (assign10900_e15274 * locals.var_dmcgeff);
        let assign10900_e15277: f64 = (assign10900_e15271 / assign10900_e15276);
        (assign10900_e15277,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10900_e15279;
        locals.var_rend_rv = 0.0;

        let (assign10910_e15309,) = {
    if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 != 0.0)) && (locals.var_guard369 == 0.0)) && (!((locals.var_guard375 != 0.0) || (locals.var_guard376 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10910_e15309;
        locals.var_rend_rv = 0.0;

        let assign10920_e15312: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard380 = assign10920_e15312;
        locals.var_guard380_rv = 0.0;

        let (assign10930_e15337,) = {
    if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 == 0.0)) && (locals.var_guard380 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10930_e15337;
        locals.var_rend_rv = 0.0;

        let (assign10940_e15369,) = {
    if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 == 0.0)) && (locals.var_guard380 == 0.0)) {
        let assign10940_e15363: f64 = (p.p374 * locals.var_dmdgeff);
        let assign10940_e15366: f64 = (locals.var_weff * locals.var_nuendd);
        let assign10940_e15367: f64 = (assign10940_e15363 / assign10940_e15366);
        (assign10940_e15367,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10940_e15369;
        locals.var_rend_rv = 0.0;

        let assign10950_e15372: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard381 = assign10950_e15372;
        locals.var_guard381_rv = 0.0;

        let (assign10960_e15400,) = {
    if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 != 0.0)) {
        let assign10960_e15396: f64 = (p.p374 * locals.var_dmdgeff);
        let assign10960_e15398: f64 = (assign10960_e15396 / locals.var_weff);
        (assign10960_e15398,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10960_e15400;
        locals.var_rend_rv = 0.0;

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

        let (assign11010_e15459,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 == 0.0)) && (locals.var_guard382 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard385 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11010_e15459;
        locals.var_rend_rv = 0.0;

        let (assign11020_e15497,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 == 0.0)) && (locals.var_guard382 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard385 == 0.0)) {
        let assign11020_e15491: f64 = (p.p374 * locals.var_dmcgeff);
        let assign11020_e15494: f64 = (locals.var_weff * locals.var_nuendd);
        let assign11020_e15495: f64 = (assign11020_e15491 / assign11020_e15494);
        (assign11020_e15495,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11020_e15497;
        locals.var_rend_rv = 0.0;

        let assign11040_e15508: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign11040_e15511: f64 = if ((locals.var_nuendd == 0.0) || (assign11040_e15508 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard387 = assign11040_e15511;
        locals.var_guard387_rv = 0.0;

        let (assign11050_e15545,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 == 0.0)) && (locals.var_guard382 != 0.0)) && ((locals.var_guard384 != 0.0) && (locals.var_guard383 == 0.0))) && (locals.var_guard387 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11050_e15545;
        locals.var_rend_rv = 0.0;

        let (assign11060_e15590,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 == 0.0)) && (locals.var_guard382 != 0.0)) && ((locals.var_guard384 != 0.0) && (locals.var_guard383 == 0.0))) && (locals.var_guard387 == 0.0)) {
        let assign11060_e15580: f64 = (p.p374 * locals.var_weff);
        let assign11060_e15583: f64 = (3.0 * locals.var_nuendd);
        let assign11060_e15586: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign11060_e15587: f64 = (assign11060_e15583 * assign11060_e15586);
        let assign11060_e15588: f64 = (assign11060_e15580 / assign11060_e15587);
        (assign11060_e15588,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11060_e15590;
        locals.var_rend_rv = 0.0;

        let (assign11070_e15622,) = {
    if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 == 0.0)) && (locals.var_guard382 != 0.0)) && (!((locals.var_guard383 != 0.0) || (locals.var_guard384 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11070_e15622;
        locals.var_rend_rv = 0.0;

        let assign11080_e15633: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard388 = assign11080_e15633;
        locals.var_guard388_rv = 0.0;

        let assign11090_e15644: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard389 = assign11090_e15644;
        locals.var_guard389_rv = 0.0;

        let assign11100_e15647: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard390 = assign11100_e15647;
        locals.var_guard390_rv = 0.0;

        let (assign11110_e15679,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 == 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard388 != 0.0)) && (locals.var_guard390 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11110_e15679;
        locals.var_rend_rv = 0.0;

        let (assign11120_e15718,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 == 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard388 != 0.0)) && (locals.var_guard390 == 0.0)) {
        let assign11120_e15712: f64 = (p.p374 * locals.var_dmcgeff);
        let assign11120_e15715: f64 = (locals.var_weff * locals.var_nuendd);
        let assign11120_e15716: f64 = (assign11120_e15712 / assign11120_e15715);
        (assign11120_e15716,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11120_e15718;
        locals.var_rend_rv = 0.0;

        let assign11140_e15729: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign11140_e15732: f64 = if ((locals.var_nuendd == 0.0) || (assign11140_e15729 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard392 = assign11140_e15732;
        locals.var_guard392_rv = 0.0;

        let (assign11150_e15767,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 == 0.0)) && (locals.var_guard382 == 0.0)) && ((locals.var_guard389 != 0.0) && (locals.var_guard388 == 0.0))) && (locals.var_guard392 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11150_e15767;
        locals.var_rend_rv = 0.0;

        let (assign11160_e15813,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 == 0.0)) && (locals.var_guard382 == 0.0)) && ((locals.var_guard389 != 0.0) && (locals.var_guard388 == 0.0))) && (locals.var_guard392 == 0.0)) {
        let assign11160_e15803: f64 = (p.p374 * locals.var_weff);
        let assign11160_e15806: f64 = (3.0 * locals.var_nuendd);
        let assign11160_e15809: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign11160_e15810: f64 = (assign11160_e15806 * assign11160_e15809);
        let assign11160_e15811: f64 = (assign11160_e15803 / assign11160_e15810);
        (assign11160_e15811,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11160_e15813;
        locals.var_rend_rv = 0.0;

        let (assign11170_e15846,) = {
    if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 == 0.0)) && (locals.var_guard382 == 0.0)) && (!((locals.var_guard388 != 0.0) || (locals.var_guard389 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11170_e15846;
        locals.var_rend_rv = 0.0;

        let assign11180_e15849: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard393 = assign11180_e15849;
        locals.var_guard393_rv = 0.0;

        let assign11190_e15852: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard394 = assign11190_e15852;
        locals.var_guard394_rv = 0.0;

        let (assign11200_e15880,) = {
    if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 != 0.0)) && (locals.var_guard394 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11200_e15880;
        locals.var_rend_rv = 0.0;

        let (assign11210_e15915,) = {
    if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 != 0.0)) && (locals.var_guard394 == 0.0)) {
        let assign11210_e15909: f64 = (p.p374 * locals.var_dmdgeff);
        let assign11210_e15912: f64 = (locals.var_weff * locals.var_nuends);
        let assign11210_e15913: f64 = (assign11210_e15909 / assign11210_e15912);
        (assign11210_e15913,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11210_e15915;
        locals.var_rend_rv = 0.0;

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

        let (assign11260_e15976,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 == 0.0)) && (locals.var_guard395 != 0.0)) && (locals.var_guard396 != 0.0)) && (locals.var_guard398 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11260_e15976;
        locals.var_rend_rv = 0.0;

        let (assign11270_e16016,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 == 0.0)) && (locals.var_guard395 != 0.0)) && (locals.var_guard396 != 0.0)) && (locals.var_guard398 == 0.0)) {
        let assign11270_e16010: f64 = (p.p374 * locals.var_dmcgeff);
        let assign11270_e16013: f64 = (locals.var_weff * locals.var_nuendd);
        let assign11270_e16014: f64 = (assign11270_e16010 / assign11270_e16013);
        (assign11270_e16014,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11270_e16016;
        locals.var_rend_rv = 0.0;

        let assign11290_e16026: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard400 = assign11290_e16026;
        locals.var_guard400_rv = 0.0;

        let (assign11300_e16062,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 == 0.0)) && (locals.var_guard395 != 0.0)) && ((locals.var_guard397 != 0.0) && (locals.var_guard396 == 0.0))) && (locals.var_guard400 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11300_e16062;
        locals.var_rend_rv = 0.0;

        let (assign11310_e16107,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 == 0.0)) && (locals.var_guard395 != 0.0)) && ((locals.var_guard397 != 0.0) && (locals.var_guard396 == 0.0))) && (locals.var_guard400 == 0.0)) {
        let assign11310_e16099: f64 = (p.p374 * locals.var_weff);
        let assign11310_e16102: f64 = (6.0 * locals.var_nuendd);
        let assign11310_e16104: f64 = (assign11310_e16102 * locals.var_dmcgeff);
        let assign11310_e16105: f64 = (assign11310_e16099 / assign11310_e16104);
        (assign11310_e16105,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11310_e16107;
        locals.var_rend_rv = 0.0;

        let (assign11320_e16141,) = {
    if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 == 0.0)) && (locals.var_guard395 != 0.0)) && (!((locals.var_guard396 != 0.0) || (locals.var_guard397 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11320_e16141;
        locals.var_rend_rv = 0.0;

        let assign11330_e16152: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard401 = assign11330_e16152;
        locals.var_guard401_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_20(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign11340_e16163: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard402 = assign11340_e16163;
        locals.var_guard402_rv = 0.0;

        let assign11350_e16166: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard403 = assign11350_e16166;
        locals.var_guard403_rv = 0.0;

        let (assign11360_e16200,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 == 0.0)) && (locals.var_guard395 == 0.0)) && (locals.var_guard401 != 0.0)) && (locals.var_guard403 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11360_e16200;
        locals.var_rend_rv = 0.0;

        let (assign11370_e16241,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 == 0.0)) && (locals.var_guard395 == 0.0)) && (locals.var_guard401 != 0.0)) && (locals.var_guard403 == 0.0)) {
        let assign11370_e16235: f64 = (p.p374 * locals.var_dmcgeff);
        let assign11370_e16238: f64 = (locals.var_weff * locals.var_nuendd);
        let assign11370_e16239: f64 = (assign11370_e16235 / assign11370_e16238);
        (assign11370_e16239,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11370_e16241;
        locals.var_rend_rv = 0.0;

        let assign11390_e16251: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard405 = assign11390_e16251;
        locals.var_guard405_rv = 0.0;

        let (assign11400_e16288,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 == 0.0)) && (locals.var_guard395 == 0.0)) && ((locals.var_guard402 != 0.0) && (locals.var_guard401 == 0.0))) && (locals.var_guard405 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11400_e16288;
        locals.var_rend_rv = 0.0;

        let (assign11410_e16334,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 == 0.0)) && (locals.var_guard395 == 0.0)) && ((locals.var_guard402 != 0.0) && (locals.var_guard401 == 0.0))) && (locals.var_guard405 == 0.0)) {
        let assign11410_e16326: f64 = (p.p374 * locals.var_weff);
        let assign11410_e16329: f64 = (6.0 * locals.var_nuendd);
        let assign11410_e16331: f64 = (assign11410_e16329 * locals.var_dmcgeff);
        let assign11410_e16332: f64 = (assign11410_e16326 / assign11410_e16331);
        (assign11410_e16332,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11410_e16334;
        locals.var_rend_rv = 0.0;

        let (assign11420_e16369,) = {
    if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 == 0.0)) && (locals.var_guard395 == 0.0)) && (!((locals.var_guard401 != 0.0) || (locals.var_guard402 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11420_e16369;
        locals.var_rend_rv = 0.0;

        let (assign11430_e16399,) = {
    if (((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard261 != 0.0) && (!((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0))))) {
        let assign11430_e16395: f64 = (p.p374 * locals.var_dmdgeff);
        let assign11430_e16397: f64 = (assign11430_e16395 / locals.var_weff);
        (assign11430_e16397,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11430_e16399;
        locals.var_rend_rv = 0.0;

        let assign11440_e16402: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard406 = assign11440_e16402;
        locals.var_guard406_rv = 0.0;

        let (assign11450_e16438,) = {
    if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard262 != 0.0) && (!(((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0))))) && (locals.var_guard406 != 0.0)) {
        let assign11450_e16432: f64 = (0.5 * p.p374);
        let assign11450_e16434: f64 = (assign11450_e16432 * locals.var_dmcgeff);
        let assign11450_e16436: f64 = (assign11450_e16434 / locals.var_weff);
        (assign11450_e16436,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11450_e16438;
        locals.var_rend_rv = 0.0;

        let assign11460_e16441: f64 = if p.p2 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard407 = assign11460_e16441;
        locals.var_guard407_rv = 0.0;

        let (assign11470_e16473,) = {
    if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard262 != 0.0) && (!(((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0))))) && (locals.var_guard406 != 0.0)) && (locals.var_guard407 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign11470_e16473;
        locals.var_rint_rv = 0.0;

        let (assign11480_e16514,) = {
    if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard262 != 0.0) && (!(((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0))))) && (locals.var_guard406 != 0.0)) && (locals.var_guard407 == 0.0)) {
        let assign11480_e16506: f64 = (p.p374 * locals.var_dmcgeff);
        let assign11480_e16510: f64 = (p.p2 - 2.0);
        let assign11480_e16511: f64 = (locals.var_weff * assign11480_e16510);
        let assign11480_e16512: f64 = (assign11480_e16506 / assign11480_e16511);
        (assign11480_e16512,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign11480_e16514;
        locals.var_rint_rv = 0.0;

        let (assign11490_e16545,) = {
    if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard262 != 0.0) && (!(((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0))))) && (locals.var_guard406 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11490_e16545;
        locals.var_rend_rv = 0.0;

        let (assign11500_e16582,) = {
    if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard262 != 0.0) && (!(((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0))))) && (locals.var_guard406 == 0.0)) {
        let assign11500_e16576: f64 = (p.p374 * locals.var_dmcgeff);
        let assign11500_e16579: f64 = (locals.var_weff * p.p2);
        let assign11500_e16580: f64 = (assign11500_e16576 / assign11500_e16579);
        (assign11500_e16580,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign11500_e16582;
        locals.var_rint_rv = 0.0;

        let assign11510_e16585: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard408 = assign11510_e16585;
        locals.var_guard408_rv = 0.0;

        let (assign11520_e16617,) = {
    if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard263 != 0.0) && (!((((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0))))) && (locals.var_guard408 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11520_e16617;
        locals.var_rend_rv = 0.0;

        let (assign11530_e16655,) = {
    if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard263 != 0.0) && (!((((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0))))) && (locals.var_guard408 != 0.0)) {
        let assign11530_e16649: f64 = (p.p374 * locals.var_dmcgeff);
        let assign11530_e16652: f64 = (locals.var_weff * p.p2);
        let assign11530_e16653: f64 = (assign11530_e16649 / assign11530_e16652);
        (assign11530_e16653,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign11530_e16655;
        locals.var_rint_rv = 0.0;

        let (assign11540_e16694,) = {
    if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard263 != 0.0) && (!((((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0))))) && (locals.var_guard408 == 0.0)) {
        let assign11540_e16688: f64 = (0.5 * p.p374);
        let assign11540_e16690: f64 = (assign11540_e16688 * locals.var_dmcgeff);
        let assign11540_e16692: f64 = (assign11540_e16690 / locals.var_weff);
        (assign11540_e16692,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11540_e16694;
        locals.var_rend_rv = 0.0;

        let assign11550_e16697: f64 = if p.p2 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard409 = assign11550_e16697;
        locals.var_guard409_rv = 0.0;

        let (assign11560_e16732,) = {
    if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard263 != 0.0) && (!((((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0))))) && (locals.var_guard408 == 0.0)) && (locals.var_guard409 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign11560_e16732;
        locals.var_rint_rv = 0.0;

        let (assign11570_e16776,) = {
    if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard263 != 0.0) && (!((((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0))))) && (locals.var_guard408 == 0.0)) && (locals.var_guard409 == 0.0)) {
        let assign11570_e16768: f64 = (p.p374 * locals.var_dmcgeff);
        let assign11570_e16772: f64 = (p.p2 - 2.0);
        let assign11570_e16773: f64 = (locals.var_weff * assign11570_e16772);
        let assign11570_e16774: f64 = (assign11570_e16768 / assign11570_e16773);
        (assign11570_e16774,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign11570_e16776;
        locals.var_rint_rv = 0.0;

        let (assign11580_e16806,) = {
    if (((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (!(((((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0)) || (locals.var_guard263 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign11580_e16806;
        locals.var_rint_rv = 0.0;

        let assign11590_e16809: f64 = if locals.var_rint <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard410 = assign11590_e16809;
        locals.var_guard410_rv = 0.0;

        let (assign11600_e16818,) = {
    if (((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard410 != 0.0)) {
        (locals.var_rend,)
    } else {
        (locals.var_rdraingeo,)
    }
};
        locals.var_rdraingeo = assign11600_e16818;
        locals.var_rdraingeo_rv = 0.0;

        let assign11610_e16821: f64 = if locals.var_rend <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard411 = assign11610_e16821;
        locals.var_guard411_rv = 0.0;

        let (assign11620_e16833,) = {
    if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard410 == 0.0)) && (locals.var_guard411 != 0.0)) {
        (locals.var_rint,)
    } else {
        (locals.var_rdraingeo,)
    }
};
        locals.var_rdraingeo = assign11620_e16833;
        locals.var_rdraingeo_rv = 0.0;

        let (assign11630_e16852,) = {
    if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard410 == 0.0)) && (locals.var_guard411 == 0.0)) {
        let assign11630_e16846: f64 = (locals.var_rint * locals.var_rend);
        let assign11630_e16849: f64 = (locals.var_rint + locals.var_rend);
        let assign11630_e16850: f64 = (assign11630_e16846 / assign11630_e16849);
        (assign11630_e16850,)
    } else {
        (locals.var_rdraingeo,)
    }
};
        locals.var_rdraingeo = assign11630_e16852;
        locals.var_rdraingeo_rv = 0.0;

        let (assign11650_e16863,) = {
    if ((locals.var_guard245 == 0.0) && (locals.var_guard246 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_rdraingeo,)
    }
};
        locals.var_rdraingeo = assign11650_e16863;
        locals.var_rdraingeo_rv = 0.0;

        let assign11660_e16866: f64 = if p.p42 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard413 = assign11660_e16866;
        locals.var_guard413_rv = 0.0;

        let assign11670_e16869: f64 = if locals.var_rsourcegeo < p.p1093 { 1.0 } else { 0.0 };
        locals.var_guard414 = assign11670_e16869;
        locals.var_guard414_rv = 0.0;

        let (assign11680_e16875,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard414 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rsourcegeo,)
    }
};
        locals.var_rsourcegeo = assign11680_e16875;
        locals.var_rsourcegeo_rv = 0.0;

        let assign11690_e16878: f64 = if locals.var_rdraingeo < p.p1093 { 1.0 } else { 0.0 };
        locals.var_guard415 = assign11690_e16878;
        locals.var_guard415_rv = 0.0;

        let (assign11700_e16884,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rdraingeo,)
    }
};
        locals.var_rdraingeo = assign11700_e16884;
        locals.var_rdraingeo_rv = 0.0;

        let assign11710_e16887: f64 = if locals.var_rsourcegeo <= p.p1093 { 1.0 } else { 0.0 };
        locals.var_guard416 = assign11710_e16887;
        locals.var_guard416_rv = 0.0;

        let (assign11720_e16894,) = {
    if ((locals.var_guard413 == 0.0) && (locals.var_guard416 != 0.0)) {
        (p.p1093,)
    } else {
        (locals.var_rsourcegeo,)
    }
};
        locals.var_rsourcegeo = assign11720_e16894;
        locals.var_rsourcegeo_rv = 0.0;

        let assign11730_e16897: f64 = if locals.var_rdraingeo <= p.p1093 { 1.0 } else { 0.0 };
        locals.var_guard417 = assign11730_e16897;
        locals.var_guard417_rv = 0.0;

        let (assign11740_e16904,) = {
    if ((locals.var_guard413 == 0.0) && (locals.var_guard417 != 0.0)) {
        (p.p1093,)
    } else {
        (locals.var_rdraingeo,)
    }
};
        locals.var_rdraingeo = assign11740_e16904;
        locals.var_rdraingeo_rv = 0.0;

        let assign11750_e16907: f64 = if p.p42 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard418 = assign11750_e16907;
        locals.var_guard418_rv = 0.0;

        let assign11760_e16910: f64 = if locals.var_rswmin_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard419 = assign11760_e16910;
        locals.var_guard419_rv = 0.0;

        let (assign11770_e16916,) = {
    if ((locals.var_guard418 != 0.0) && (locals.var_guard419 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rswmin_i,)
    }
};
        locals.var_rswmin_i = assign11770_e16916;
        locals.var_rswmin_i_rv = 0.0;

        let assign11780_e16919: f64 = if locals.var_rdwmin_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard420 = assign11780_e16919;
        locals.var_guard420_rv = 0.0;

        let (assign11790_e16925,) = {
    if ((locals.var_guard418 != 0.0) && (locals.var_guard420 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rdwmin_i,)
    }
};
        locals.var_rdwmin_i = assign11790_e16925;
        locals.var_rdwmin_i_rv = 0.0;

        let assign11800_e16928: f64 = if locals.var_rsw_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard421 = assign11800_e16928;
        locals.var_guard421_rv = 0.0;

        let (assign11810_e16934,) = {
    if ((locals.var_guard418 != 0.0) && (locals.var_guard421 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rsw_i,)
    }
};
        locals.var_rsw_i = assign11810_e16934;
        locals.var_rsw_i_rv = 0.0;

        let assign11820_e16937: f64 = if locals.var_rdw_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard422 = assign11820_e16937;
        locals.var_guard422_rv = 0.0;

        let (assign11830_e16943,) = {
    if ((locals.var_guard418 != 0.0) && (locals.var_guard422 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rdw_i,)
    }
};
        locals.var_rdw_i = assign11830_e16943;
        locals.var_rdw_i_rv = 0.0;

        let assign11840_e16946: f64 = if locals.var_rdswmin_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard423 = assign11840_e16946;
        locals.var_guard423_rv = 0.0;

        let (assign11850_e16953,) = {
    if ((locals.var_guard418 == 0.0) && (locals.var_guard423 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rdswmin_i,)
    }
};
        locals.var_rdswmin_i = assign11850_e16953;
        locals.var_rdswmin_i_rv = 0.0;

        let assign11860_e16956: f64 = if locals.var_rdsw_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard424 = assign11860_e16956;
        locals.var_guard424_rv = 0.0;

        let (assign11870_e16963,) = {
    if ((locals.var_guard418 == 0.0) && (locals.var_guard424 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rdsw_i,)
    }
};
        locals.var_rdsw_i = assign11870_e16963;
        locals.var_rdsw_i_rv = 0.0;

        let assign12580_e17615: f64 = if p.p1097 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard443 = assign12580_e17615;
        locals.var_guard443_rv = 0.0;

        let (assign12620_e17641,) = {
    if (locals.var_guard443 != 0.0) {
        let assign12620_e17639: f64 = (1.0 - p.p1128);
        (assign12620_e17639,)
    } else {
        (locals.var_oneminusxpart,)
    }
};
        locals.var_oneminusxpart = assign12620_e17641;
        locals.var_oneminusxpart_rv = 0.0;

        let (assign12630_e17646,) = {
    if (locals.var_guard443 == 0.0) {
        (1.0,)
    } else {
        (locals.var_oneminusxpart,)
    }
};
        locals.var_oneminusxpart = assign12630_e17646;
        locals.var_oneminusxpart_rv = 0.0;

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

        let (assign12660_e17672,) = {
    if (locals.var_guard445 != 0.0) {
        let assign12660_e17670: f64 = (1.0 / locals.var_grgeltd);
        (assign12660_e17670,)
    } else {
        (locals.var_grgeltd,)
    }
};
        locals.var_grgeltd = assign12660_e17672;
        locals.var_grgeltd_rv = 0.0;

        let (assign12670_e17677,) = {
    if (locals.var_guard445 == 0.0) {
        (1000.0,)
    } else {
        (locals.var_grgeltd,)
    }
};
        locals.var_grgeltd = assign12670_e17677;
        locals.var_grgeltd_rv = 0.0;

        let assign12690_e17683: f64 = (p.p77 * p.p77);
        locals.var_t0 = assign12690_e17683;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_21(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let nv4 = ctx.node_voltage(nodes[4]);
        let assign12700_e17686: f64 = (p.p77 * locals.var_poxedge_i);
        locals.var_t1 = assign12700_e17686;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign12710_e17689: f64 = (locals.var_t1 * locals.var_t1);
        locals.var_t2 = assign12710_e17689;
        locals.var_t2_dn0 = ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0));
        locals.var_t2_dn2 = ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2));
        locals.var_t2_dn3 = ((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3));
        locals.var_t2_dn4 = ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4));
        locals.var_t2_dn5 = ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5));
        locals.var_t2_dn6 = ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6));
        locals.var_t2_dn7 = ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7));
        locals.var_t2_dn8 = ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8));
        locals.var_t2_dn9 = ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9));
        locals.var_t2_dn10 = ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10));
        locals.var_t2_dn11 = ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11));
        locals.var_t2_dn12 = ((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12));
        locals.var_t2_dn13 = ((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13));
        locals.var_t2_dn14 = ((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14));
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

        let (assign12820_e17767,) = {
    if (locals.var_guard447 != 0.0) {
        let assign12820_e17763: f64 = (locals.var_weff_sh * p.p2);
        let assign12820_e17765: f64 = (assign12820_e17763 / p.p909);
        (assign12820_e17765,)
    } else {
        (locals.var_gth,)
    }
};
        locals.var_gth = assign12820_e17767;
        locals.var_gth_rv = 0.0;

        let (assign12830_e17775,) = {
    if (locals.var_guard447 != 0.0) {
        let assign12830_e17771: f64 = (p.p910 * locals.var_weff_sh);
        let assign12830_e17773: f64 = (assign12830_e17771 * p.p2);
        (assign12830_e17773,)
    } else {
        (locals.var_cth,)
    }
};
        locals.var_cth = assign12830_e17775;
        locals.var_cth_rv = 0.0;

        let (assign12840_e17780,) = {
    if (locals.var_guard447 == 0.0) {
        (1.0,)
    } else {
        (locals.var_gth,)
    }
};
        locals.var_gth = assign12840_e17780;
        locals.var_gth_rv = 0.0;

        let (assign12850_e17785,) = {
    if (locals.var_guard447 == 0.0) {
        (0.0,)
    } else {
        (locals.var_cth,)
    }
};
        locals.var_cth = assign12850_e17785;
        locals.var_cth_rv = 0.0;

        let assign12860_e17788: f64 = (-273.15);
        let assign12860_e17789: f64 = if p.p820 <= assign12860_e17788 { 1.0 } else { 0.0 };
        locals.var_guard448 = assign12860_e17789;
        locals.var_guard448_rv = 0.0;

        let (assign12870_e17795, assign12870_e17795_d_n0, assign12870_e17795_d_n2, assign12870_e17795_d_n3, assign12870_e17795_d_n4, assign12870_e17795_d_n5, assign12870_e17795_d_n6, assign12870_e17795_d_n7, assign12870_e17795_d_n8, assign12870_e17795_d_n9, assign12870_e17795_d_n10, assign12870_e17795_d_n11, assign12870_e17795_d_n12, assign12870_e17795_d_n13, assign12870_e17795_d_n14,) = {
    if (locals.var_guard448 != 0.0) {
        let assign12870_e17793: f64 = (300.15 - 273.15);
        (assign12870_e17793, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign12870_e17795;
        locals.var_t0_dn0 = assign12870_e17795_d_n0;
        locals.var_t0_dn2 = assign12870_e17795_d_n2;
        locals.var_t0_dn3 = assign12870_e17795_d_n3;
        locals.var_t0_dn4 = assign12870_e17795_d_n4;
        locals.var_t0_dn5 = assign12870_e17795_d_n5;
        locals.var_t0_dn6 = assign12870_e17795_d_n6;
        locals.var_t0_dn7 = assign12870_e17795_d_n7;
        locals.var_t0_dn8 = assign12870_e17795_d_n8;
        locals.var_t0_dn9 = assign12870_e17795_d_n9;
        locals.var_t0_dn10 = assign12870_e17795_d_n10;
        locals.var_t0_dn11 = assign12870_e17795_d_n11;
        locals.var_t0_dn12 = assign12870_e17795_d_n12;
        locals.var_t0_dn13 = assign12870_e17795_d_n13;
        locals.var_t0_dn14 = assign12870_e17795_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign12880_e17799,) = {
    if (locals.var_guard448 != 0.0) {
        (300.15,)
    } else {
        (locals.var_tnom,)
    }
};
        locals.var_tnom = assign12880_e17799;
        locals.var_tnom_rv = 0.0;

        let (assign12890_e17806,) = {
    if (locals.var_guard448 == 0.0) {
        let assign12890_e17804: f64 = (p.p820 + 273.15);
        (assign12890_e17804,)
    } else {
        (locals.var_tnom,)
    }
};
        locals.var_tnom = assign12890_e17806;
        locals.var_tnom_rv = 0.0;

        let assign12900_e17807: f64 = ctx_temp;
        let assign12900_e17809: f64 = (assign12900_e17807 + p.p33);
        locals.var_devtemp = assign12900_e17809;
        locals.var_devtemp_dn4 = 0.0;
        locals.var_devtemp_rv = 0.0;

        let assign12910_e17820: f64 = if (((p.p49 != 0.0) && (p.p909 > 0.0)) && (locals.var_weff_sh > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard449 = assign12910_e17820;
        locals.var_guard449_rv = 0.0;

        let (assign12920_e17824, assign12920_e17824_d_n4,) = {
    if (locals.var_guard449 != 0.0) {
        ((nv4 - 0.0), 1.0,)
    } else {
        (locals.var_deltemp1, locals.var_deltemp1_dn4,)
    }
};
        locals.var_deltemp1 = assign12920_e17824;
        locals.var_deltemp1_dn4 = assign12920_e17824_d_n4;
        locals.var_deltemp1_rv = 0.0;

        let (assign12930_e17829, assign12930_e17829_d_n4,) = {
    if (locals.var_guard449 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_deltemp1, locals.var_deltemp1_dn4,)
    }
};
        locals.var_deltemp1 = assign12930_e17829;
        locals.var_deltemp1_dn4 = assign12930_e17829_d_n4;
        locals.var_deltemp1_rv = 0.0;

        let assign12940_e17832: f64 = (locals.var_deltemp1 + locals.var_devtemp);
        locals.var_devtemp = assign12940_e17832;
        locals.var_devtemp_dn4 = (locals.var_deltemp1_dn4 + locals.var_devtemp_dn4);
        locals.var_devtemp_rv = 0.0;

        let assign12980_e17840: f64 = (8.617087e-5 * locals.var_devtemp);
        locals.var_vt = assign12980_e17840;
        locals.var_vt_dn4 = (8.617087e-5 * locals.var_devtemp_dn4);
        locals.var_vt_rv = 0.0;

        let assign12990_e17843: f64 = (1.0 / locals.var_vt);
        locals.var_inv_vt = assign12990_e17843;
        locals.var_inv_vt_dn4 = (-(locals.var_vt_dn4 / (locals.var_vt * locals.var_vt)));
        locals.var_inv_vt_rv = 0.0;

        let assign13000_e17846: f64 = (locals.var_devtemp / locals.var_tnom);
        locals.var_tratio = assign13000_e17846;
        locals.var_tratio_dn4 = (locals.var_devtemp_dn4 / locals.var_tnom);
        locals.var_tratio_rv = 0.0;

        let assign13010_e17849: f64 = (locals.var_devtemp - locals.var_tnom);
        locals.var_deltemp = assign13010_e17849;
        locals.var_deltemp_dn4 = locals.var_devtemp_dn4;
        locals.var_deltemp_rv = 0.0;

        let assign13020_e17852: f64 = (8.617087e-5 * locals.var_devtemp);
        locals.var_vtm = assign13020_e17852;
        locals.var_vtm_dn4 = (8.617087e-5 * locals.var_devtemp_dn4);
        locals.var_vtm_rv = 0.0;

        let assign13030_e17855: f64 = (8.617087e-5 * locals.var_tnom);
        locals.var_vtm0 = assign13030_e17855;
        locals.var_vtm0_rv = 0.0;

        let assign13040_e17859: f64 = (p.p821 * locals.var_devtemp);
        let assign13040_e17861: f64 = (assign13040_e17859 * locals.var_devtemp);
        let assign13040_e17864: f64 = (locals.var_devtemp + p.p822);
        let assign13040_e17865: f64 = (assign13040_e17861 / assign13040_e17864);
        let assign13040_e17866: f64 = (p.p109 - assign13040_e17865);
        locals.var_eg = assign13040_e17866;
        locals.var_eg_dn4 = (-((((((p.p821 * locals.var_devtemp_dn4) * locals.var_devtemp) + (assign13040_e17859 * locals.var_devtemp_dn4)) * assign13040_e17864) - (assign13040_e17861 * locals.var_devtemp_dn4)) / (assign13040_e17864 * assign13040_e17864)));
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
        locals.var_t1 = assign13060_e17885;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = (((locals.var_devtemp_dn4 / locals.var_tnom) * assign13060_e17884) + (assign13060_e17880 * ((locals.var_devtemp_dn4 / locals.var_tnom) / (2.0 * assign13060_e17884))));
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign13070_e17888: f64 = (p.p108 * locals.var_t1);
        let assign13070_e17892: f64 = (2.0 * locals.var_vtm0);
        let assign13070_e17893: f64 = (locals.var_eg / assign13070_e17892);
        let assign13070_e17897: f64 = (2.0 * locals.var_vtm);
        let assign13070_e17898: f64 = (locals.var_eg / assign13070_e17897);
        let assign13070_e17899: f64 = (assign13070_e17893 - assign13070_e17898);
        let assign13070_e17900: f64 = { let limited_exp_arg = assign13070_e17899; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13070_e17901: f64 = (assign13070_e17888 * assign13070_e17900);
        locals.var_ni = assign13070_e17901;
        locals.var_ni_dn0 = ((p.p108 * locals.var_t1_dn0) * assign13070_e17900);
        locals.var_ni_dn2 = ((p.p108 * locals.var_t1_dn2) * assign13070_e17900);
        locals.var_ni_dn3 = ((p.p108 * locals.var_t1_dn3) * assign13070_e17900);
        locals.var_ni_dn4 = (((p.p108 * locals.var_t1_dn4) * assign13070_e17900) + (assign13070_e17888 * ({ let limited_exp_arg = assign13070_e17899; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_eg_dn4 / assign13070_e17892) - (((locals.var_eg_dn4 * assign13070_e17897) - (locals.var_eg * (2.0 * locals.var_vtm_dn4))) / (assign13070_e17897 * assign13070_e17897))))));
        locals.var_ni_dn5 = ((p.p108 * locals.var_t1_dn5) * assign13070_e17900);
        locals.var_ni_dn6 = ((p.p108 * locals.var_t1_dn6) * assign13070_e17900);
        locals.var_ni_dn7 = ((p.p108 * locals.var_t1_dn7) * assign13070_e17900);
        locals.var_ni_dn8 = ((p.p108 * locals.var_t1_dn8) * assign13070_e17900);
        locals.var_ni_dn9 = ((p.p108 * locals.var_t1_dn9) * assign13070_e17900);
        locals.var_ni_dn10 = ((p.p108 * locals.var_t1_dn10) * assign13070_e17900);
        locals.var_ni_dn11 = ((p.p108 * locals.var_t1_dn11) * assign13070_e17900);
        locals.var_ni_dn12 = ((p.p108 * locals.var_t1_dn12) * assign13070_e17900);
        locals.var_ni_dn13 = ((p.p108 * locals.var_t1_dn13) * assign13070_e17900);
        locals.var_ni_dn14 = ((p.p108 * locals.var_t1_dn14) * assign13070_e17900);
        locals.var_ni_rv = 0.0;

        let assign13080_e17912: f64 = if (((p.p49 != 0.0) && (p.p909 > 0.0)) && (locals.var_weff_sh > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard450 = assign13080_e17912;
        locals.var_guard450_rv = 0.0;

        let (assign13090_e17921, assign13090_e17921_d_n0, assign13090_e17921_d_n2, assign13090_e17921_d_n3, assign13090_e17921_d_n4, assign13090_e17921_d_n5, assign13090_e17921_d_n6, assign13090_e17921_d_n7, assign13090_e17921_d_n8, assign13090_e17921_d_n9, assign13090_e17921_d_n10, assign13090_e17921_d_n11, assign13090_e17921_d_n12, assign13090_e17921_d_n13, assign13090_e17921_d_n14,) = {
    if (locals.var_guard450 != 0.0) {
        let assign13090_e17916: f64 = (locals.var_ndep_i / locals.var_ni);
        let assign13090_e17918: f64 = (assign13090_e17916).max(1e-38);
        let assign13090_e17919: f64 = (assign13090_e17918).ln();
        (assign13090_e17919, (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn0 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn0)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn2 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn2)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn3 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn3)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn4 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn4)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn5 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn5)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn6 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn6)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn7 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn7)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn8 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn8)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn9 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn9)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn10 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn10)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn11 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn11)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn12 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn12)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn13 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn13)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn14 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn14)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign13090_e17921;
        locals.var_t0_dn0 = assign13090_e17921_d_n0;
        locals.var_t0_dn2 = assign13090_e17921_d_n2;
        locals.var_t0_dn3 = assign13090_e17921_d_n3;
        locals.var_t0_dn4 = assign13090_e17921_d_n4;
        locals.var_t0_dn5 = assign13090_e17921_d_n5;
        locals.var_t0_dn6 = assign13090_e17921_d_n6;
        locals.var_t0_dn7 = assign13090_e17921_d_n7;
        locals.var_t0_dn8 = assign13090_e17921_d_n8;
        locals.var_t0_dn9 = assign13090_e17921_d_n9;
        locals.var_t0_dn10 = assign13090_e17921_d_n10;
        locals.var_t0_dn11 = assign13090_e17921_d_n11;
        locals.var_t0_dn12 = assign13090_e17921_d_n12;
        locals.var_t0_dn13 = assign13090_e17921_d_n13;
        locals.var_t0_dn14 = assign13090_e17921_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign13100_e17930, assign13100_e17930_d_n0, assign13100_e17930_d_n2, assign13100_e17930_d_n3, assign13100_e17930_d_n4, assign13100_e17930_d_n5, assign13100_e17930_d_n6, assign13100_e17930_d_n7, assign13100_e17930_d_n8, assign13100_e17930_d_n9, assign13100_e17930_d_n10, assign13100_e17930_d_n11, assign13100_e17930_d_n12, assign13100_e17930_d_n13, assign13100_e17930_d_n14,) = {
    if (locals.var_guard450 != 0.0) {
        let assign13100_e17925: f64 = (locals.var_t0 * locals.var_t0);
        let assign13100_e17927: f64 = (assign13100_e17925 + 1e-6);
        let assign13100_e17928: f64 = (assign13100_e17927).sqrt();
        (assign13100_e17928, (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn12 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn12)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)) / (2.0 * assign13100_e17928)),)
    } else {
        (locals.var_phib, locals.var_phib_dn0, locals.var_phib_dn2, locals.var_phib_dn3, locals.var_phib_dn4, locals.var_phib_dn5, locals.var_phib_dn6, locals.var_phib_dn7, locals.var_phib_dn8, locals.var_phib_dn9, locals.var_phib_dn10, locals.var_phib_dn11, locals.var_phib_dn12, locals.var_phib_dn13, locals.var_phib_dn14,)
    }
};
        locals.var_phib = assign13100_e17930;
        locals.var_phib_dn0 = assign13100_e17930_d_n0;
        locals.var_phib_dn2 = assign13100_e17930_d_n2;
        locals.var_phib_dn3 = assign13100_e17930_d_n3;
        locals.var_phib_dn4 = assign13100_e17930_d_n4;
        locals.var_phib_dn5 = assign13100_e17930_d_n5;
        locals.var_phib_dn6 = assign13100_e17930_d_n6;
        locals.var_phib_dn7 = assign13100_e17930_d_n7;
        locals.var_phib_dn8 = assign13100_e17930_d_n8;
        locals.var_phib_dn9 = assign13100_e17930_d_n9;
        locals.var_phib_dn10 = assign13100_e17930_d_n10;
        locals.var_phib_dn11 = assign13100_e17930_d_n11;
        locals.var_phib_dn12 = assign13100_e17930_d_n12;
        locals.var_phib_dn13 = assign13100_e17930_d_n13;
        locals.var_phib_dn14 = assign13100_e17930_d_n14;
        locals.var_phib_rv = 0.0;

        let (assign13110_e17940, assign13110_e17940_d_n0, assign13110_e17940_d_n2, assign13110_e17940_d_n3, assign13110_e17940_d_n4, assign13110_e17940_d_n5, assign13110_e17940_d_n6, assign13110_e17940_d_n7, assign13110_e17940_d_n8, assign13110_e17940_d_n9, assign13110_e17940_d_n10, assign13110_e17940_d_n11, assign13110_e17940_d_n12, assign13110_e17940_d_n13, assign13110_e17940_d_n14,) = {
    if (locals.var_guard450 == 0.0) {
        let assign13110_e17935: f64 = (locals.var_ndep_i / locals.var_ni);
        let assign13110_e17937: f64 = (assign13110_e17935).max(1e-38);
        let assign13110_e17938: f64 = (assign13110_e17937).ln();
        (assign13110_e17938, (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn0 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn0)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn2 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn2)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn3 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn3)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn4 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn4)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn5 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn5)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn6 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn6)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn7 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn7)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn8 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn8)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn9 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn9)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn10 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn10)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn11 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn11)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn12 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn12)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn13 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn13)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn14 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn14)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937),)
    } else {
        (locals.var_phib, locals.var_phib_dn0, locals.var_phib_dn2, locals.var_phib_dn3, locals.var_phib_dn4, locals.var_phib_dn5, locals.var_phib_dn6, locals.var_phib_dn7, locals.var_phib_dn8, locals.var_phib_dn9, locals.var_phib_dn10, locals.var_phib_dn11, locals.var_phib_dn12, locals.var_phib_dn13, locals.var_phib_dn14,)
    }
};
        locals.var_phib = assign13110_e17940;
        locals.var_phib_dn0 = assign13110_e17940_d_n0;
        locals.var_phib_dn2 = assign13110_e17940_d_n2;
        locals.var_phib_dn3 = assign13110_e17940_d_n3;
        locals.var_phib_dn4 = assign13110_e17940_d_n4;
        locals.var_phib_dn5 = assign13110_e17940_d_n5;
        locals.var_phib_dn6 = assign13110_e17940_d_n6;
        locals.var_phib_dn7 = assign13110_e17940_d_n7;
        locals.var_phib_dn8 = assign13110_e17940_d_n8;
        locals.var_phib_dn9 = assign13110_e17940_d_n9;
        locals.var_phib_dn10 = assign13110_e17940_d_n10;
        locals.var_phib_dn11 = assign13110_e17940_d_n11;
        locals.var_phib_dn12 = assign13110_e17940_d_n12;
        locals.var_phib_dn13 = assign13110_e17940_d_n13;
        locals.var_phib_dn14 = assign13110_e17940_d_n14;
        locals.var_phib_rv = 0.0;

        let assign13120_e17951: f64 = if (((p.p49 != 0.0) && (p.p909 > 0.0)) && (locals.var_weff_sh > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard451 = assign13120_e17951;
        locals.var_guard451_rv = 0.0;

        let (assign13130_e17964, assign13130_e17964_d_n0, assign13130_e17964_d_n2, assign13130_e17964_d_n3, assign13130_e17964_d_n4, assign13130_e17964_d_n5, assign13130_e17964_d_n6, assign13130_e17964_d_n7, assign13130_e17964_d_n8, assign13130_e17964_d_n9, assign13130_e17964_d_n10, assign13130_e17964_d_n11, assign13130_e17964_d_n12, assign13130_e17964_d_n13, assign13130_e17964_d_n14,) = {
    if (locals.var_guard451 != 0.0) {
        let assign13130_e17955: f64 = (locals.var_ndepedge_i * locals.var_nsd_i);
        let assign13130_e17958: f64 = (locals.var_ni * locals.var_ni);
        let assign13130_e17959: f64 = (assign13130_e17955 / assign13130_e17958);
        let assign13130_e17961: f64 = (assign13130_e17959).max(1e-38);
        let assign13130_e17962: f64 = (assign13130_e17961).ln();
        (assign13130_e17962, (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn0 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn0))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn2 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn2))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn3 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn3))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn4 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn4))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn5 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn5))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn6 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn6))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn7 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn7))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn8 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn8))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn9 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn9))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn10 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn10))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn11 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn11))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn12 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn12))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn13 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn13))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn14 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn14))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign13130_e17964;
        locals.var_t0_dn0 = assign13130_e17964_d_n0;
        locals.var_t0_dn2 = assign13130_e17964_d_n2;
        locals.var_t0_dn3 = assign13130_e17964_d_n3;
        locals.var_t0_dn4 = assign13130_e17964_d_n4;
        locals.var_t0_dn5 = assign13130_e17964_d_n5;
        locals.var_t0_dn6 = assign13130_e17964_d_n6;
        locals.var_t0_dn7 = assign13130_e17964_d_n7;
        locals.var_t0_dn8 = assign13130_e17964_d_n8;
        locals.var_t0_dn9 = assign13130_e17964_d_n9;
        locals.var_t0_dn10 = assign13130_e17964_d_n10;
        locals.var_t0_dn11 = assign13130_e17964_d_n11;
        locals.var_t0_dn12 = assign13130_e17964_d_n12;
        locals.var_t0_dn13 = assign13130_e17964_d_n13;
        locals.var_t0_dn14 = assign13130_e17964_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign13140_e17973, assign13140_e17973_d_n0, assign13140_e17973_d_n2, assign13140_e17973_d_n3, assign13140_e17973_d_n4, assign13140_e17973_d_n5, assign13140_e17973_d_n6, assign13140_e17973_d_n7, assign13140_e17973_d_n8, assign13140_e17973_d_n9, assign13140_e17973_d_n10, assign13140_e17973_d_n11, assign13140_e17973_d_n12, assign13140_e17973_d_n13, assign13140_e17973_d_n14,) = {
    if (locals.var_guard451 != 0.0) {
        let assign13140_e17968: f64 = (locals.var_t0 * locals.var_t0);
        let assign13140_e17970: f64 = (assign13140_e17968 + 1e-6);
        let assign13140_e17971: f64 = (assign13140_e17970).sqrt();
        (assign13140_e17971, (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn12 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn12)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)) / (2.0 * assign13140_e17971)),)
    } else {
        (locals.var_vbi_edge, locals.var_vbi_edge_dn0, locals.var_vbi_edge_dn2, locals.var_vbi_edge_dn3, locals.var_vbi_edge_dn4, locals.var_vbi_edge_dn5, locals.var_vbi_edge_dn6, locals.var_vbi_edge_dn7, locals.var_vbi_edge_dn8, locals.var_vbi_edge_dn9, locals.var_vbi_edge_dn10, locals.var_vbi_edge_dn11, locals.var_vbi_edge_dn12, locals.var_vbi_edge_dn13, locals.var_vbi_edge_dn14,)
    }
};
        locals.var_vbi_edge = assign13140_e17973;
        locals.var_vbi_edge_dn0 = assign13140_e17973_d_n0;
        locals.var_vbi_edge_dn2 = assign13140_e17973_d_n2;
        locals.var_vbi_edge_dn3 = assign13140_e17973_d_n3;
        locals.var_vbi_edge_dn4 = assign13140_e17973_d_n4;
        locals.var_vbi_edge_dn5 = assign13140_e17973_d_n5;
        locals.var_vbi_edge_dn6 = assign13140_e17973_d_n6;
        locals.var_vbi_edge_dn7 = assign13140_e17973_d_n7;
        locals.var_vbi_edge_dn8 = assign13140_e17973_d_n8;
        locals.var_vbi_edge_dn9 = assign13140_e17973_d_n9;
        locals.var_vbi_edge_dn10 = assign13140_e17973_d_n10;
        locals.var_vbi_edge_dn11 = assign13140_e17973_d_n11;
        locals.var_vbi_edge_dn12 = assign13140_e17973_d_n12;
        locals.var_vbi_edge_dn13 = assign13140_e17973_d_n13;
        locals.var_vbi_edge_dn14 = assign13140_e17973_d_n14;
        locals.var_vbi_edge_rv = 0.0;

        let (assign13150_e17987, assign13150_e17987_d_n0, assign13150_e17987_d_n2, assign13150_e17987_d_n3, assign13150_e17987_d_n4, assign13150_e17987_d_n5, assign13150_e17987_d_n6, assign13150_e17987_d_n7, assign13150_e17987_d_n8, assign13150_e17987_d_n9, assign13150_e17987_d_n10, assign13150_e17987_d_n11, assign13150_e17987_d_n12, assign13150_e17987_d_n13, assign13150_e17987_d_n14,) = {
    if (locals.var_guard451 == 0.0) {
        let assign13150_e17978: f64 = (locals.var_ndepedge_i * locals.var_nsd_i);
        let assign13150_e17981: f64 = (locals.var_ni * locals.var_ni);
        let assign13150_e17982: f64 = (assign13150_e17978 / assign13150_e17981);
        let assign13150_e17984: f64 = (assign13150_e17982).max(1e-38);
        let assign13150_e17985: f64 = (assign13150_e17984).ln();
        (assign13150_e17985, (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn0 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn0))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn2 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn2))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn3 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn3))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn4 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn4))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn5 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn5))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn6 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn6))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn7 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn7))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn8 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn8))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn9 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn9))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn10 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn10))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn11 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn11))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn12 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn12))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn13 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn13))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn14 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn14))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984),)
    } else {
        (locals.var_vbi_edge, locals.var_vbi_edge_dn0, locals.var_vbi_edge_dn2, locals.var_vbi_edge_dn3, locals.var_vbi_edge_dn4, locals.var_vbi_edge_dn5, locals.var_vbi_edge_dn6, locals.var_vbi_edge_dn7, locals.var_vbi_edge_dn8, locals.var_vbi_edge_dn9, locals.var_vbi_edge_dn10, locals.var_vbi_edge_dn11, locals.var_vbi_edge_dn12, locals.var_vbi_edge_dn13, locals.var_vbi_edge_dn14,)
    }
};
        locals.var_vbi_edge = assign13150_e17987;
        locals.var_vbi_edge_dn0 = assign13150_e17987_d_n0;
        locals.var_vbi_edge_dn2 = assign13150_e17987_d_n2;
        locals.var_vbi_edge_dn3 = assign13150_e17987_d_n3;
        locals.var_vbi_edge_dn4 = assign13150_e17987_d_n4;
        locals.var_vbi_edge_dn5 = assign13150_e17987_d_n5;
        locals.var_vbi_edge_dn6 = assign13150_e17987_d_n6;
        locals.var_vbi_edge_dn7 = assign13150_e17987_d_n7;
        locals.var_vbi_edge_dn8 = assign13150_e17987_d_n8;
        locals.var_vbi_edge_dn9 = assign13150_e17987_d_n9;
        locals.var_vbi_edge_dn10 = assign13150_e17987_d_n10;
        locals.var_vbi_edge_dn11 = assign13150_e17987_d_n11;
        locals.var_vbi_edge_dn12 = assign13150_e17987_d_n12;
        locals.var_vbi_edge_dn13 = assign13150_e17987_d_n13;
        locals.var_vbi_edge_dn14 = assign13150_e17987_d_n14;
        locals.var_vbi_edge_rv = 0.0;

        let assign13160_e17990: f64 = if locals.var_ngate_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard452 = assign13160_e17990;
        locals.var_guard452_rv = 0.0;

        let (assign13170_e18006, assign13170_e18006_d_n4,) = {
    if (locals.var_guard452 != 0.0) {
        let assign13170_e17993: f64 = (-locals.var_devsign);
        let assign13170_e17995: f64 = (assign13170_e17993 * locals.var_vt);
        let assign13170_e17998: f64 = (locals.var_ngate_i / locals.var_nsd_i);
        let assign13170_e18000: f64 = (assign13170_e17998).max(1e-38);
        let assign13170_e18001: f64 = (assign13170_e18000).ln();
        let assign13170_e18002: f64 = (assign13170_e17995 * assign13170_e18001);
        let assign13170_e18004: f64 = (assign13170_e18002 + p.p5);
        (assign13170_e18004, ((assign13170_e17993 * locals.var_vt_dn4) * assign13170_e18001),)
    } else {
        (locals.var_vfbsdr, locals.var_vfbsdr_dn4,)
    }
};
        locals.var_vfbsdr = assign13170_e18006;
        locals.var_vfbsdr_dn4 = assign13170_e18006_d_n4;
        locals.var_vfbsdr_rv = 0.0;

        let (assign13180_e18011, assign13180_e18011_d_n4,) = {
    if (locals.var_guard452 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_vfbsdr, locals.var_vfbsdr_dn4,)
    }
};
        locals.var_vfbsdr = assign13180_e18011;
        locals.var_vfbsdr_dn4 = assign13180_e18011_d_n4;
        locals.var_vfbsdr_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_22(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign13190_e18015: f64 = (locals.var_vt * locals.var_phib);
        let assign13190_e18016: f64 = (0.4 + assign13190_e18015);
        let assign13190_e18018: f64 = (assign13190_e18016 + locals.var_phin_i);
        let assign13190_e18020: f64 = (assign13190_e18018).max(0.4);
        locals.var_phist = assign13190_e18020;
        locals.var_phist_dn0 = if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn0) } else { 0.0 };
        locals.var_phist_dn2 = if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn2) } else { 0.0 };
        locals.var_phist_dn3 = if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn3) } else { 0.0 };
        locals.var_phist_dn4 = if assign13190_e18018 >= 0.4 { ((locals.var_vt_dn4 * locals.var_phib) + (locals.var_vt * locals.var_phib_dn4)) } else { 0.0 };
        locals.var_phist_dn5 = if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn5) } else { 0.0 };
        locals.var_phist_dn6 = if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn6) } else { 0.0 };
        locals.var_phist_dn7 = if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn7) } else { 0.0 };
        locals.var_phist_dn8 = if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn8) } else { 0.0 };
        locals.var_phist_dn9 = if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn9) } else { 0.0 };
        locals.var_phist_dn10 = if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn10) } else { 0.0 };
        locals.var_phist_dn11 = if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn11) } else { 0.0 };
        locals.var_phist_dn12 = if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn12) } else { 0.0 };
        locals.var_phist_dn13 = if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn13) } else { 0.0 };
        locals.var_phist_dn14 = if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn14) } else { 0.0 };
        locals.var_phist_rv = 0.0;

        let assign13200_e18022: f64 = (locals.var_phist).sqrt();
        locals.var_sqrtphist = assign13200_e18022;
        locals.var_sqrtphist_dn0 = (locals.var_phist_dn0 / (2.0 * assign13200_e18022));
        locals.var_sqrtphist_dn2 = (locals.var_phist_dn2 / (2.0 * assign13200_e18022));
        locals.var_sqrtphist_dn3 = (locals.var_phist_dn3 / (2.0 * assign13200_e18022));
        locals.var_sqrtphist_dn4 = (locals.var_phist_dn4 / (2.0 * assign13200_e18022));
        locals.var_sqrtphist_dn5 = (locals.var_phist_dn5 / (2.0 * assign13200_e18022));
        locals.var_sqrtphist_dn6 = (locals.var_phist_dn6 / (2.0 * assign13200_e18022));
        locals.var_sqrtphist_dn7 = (locals.var_phist_dn7 / (2.0 * assign13200_e18022));
        locals.var_sqrtphist_dn8 = (locals.var_phist_dn8 / (2.0 * assign13200_e18022));
        locals.var_sqrtphist_dn9 = (locals.var_phist_dn9 / (2.0 * assign13200_e18022));
        locals.var_sqrtphist_dn10 = (locals.var_phist_dn10 / (2.0 * assign13200_e18022));
        locals.var_sqrtphist_dn11 = (locals.var_phist_dn11 / (2.0 * assign13200_e18022));
        locals.var_sqrtphist_dn12 = (locals.var_phist_dn12 / (2.0 * assign13200_e18022));
        locals.var_sqrtphist_dn13 = (locals.var_phist_dn13 / (2.0 * assign13200_e18022));
        locals.var_sqrtphist_dn14 = (locals.var_phist_dn14 / (2.0 * assign13200_e18022));
        locals.var_sqrtphist_rv = 0.0;

        let assign13210_e18025: f64 = (2.0 * locals.var_epssi);
        let assign13210_e18028: f64 = (1.60219e-19 * locals.var_ndep_i);
        let assign13210_e18029: f64 = (assign13210_e18025 / assign13210_e18028);
        let assign13210_e18030: f64 = (assign13210_e18029).sqrt();
        locals.var_t1dep = assign13210_e18030;
        locals.var_t1dep_dn0 = ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn0)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030));
        locals.var_t1dep_dn2 = ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn2)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030));
        locals.var_t1dep_dn3 = ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn3)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030));
        locals.var_t1dep_dn4 = ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn4)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030));
        locals.var_t1dep_dn5 = ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn5)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030));
        locals.var_t1dep_dn6 = ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn6)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030));
        locals.var_t1dep_dn7 = ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn7)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030));
        locals.var_t1dep_dn8 = ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn8)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030));
        locals.var_t1dep_dn9 = ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn9)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030));
        locals.var_t1dep_dn10 = ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn10)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030));
        locals.var_t1dep_dn11 = ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn11)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030));
        locals.var_t1dep_dn12 = ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn12)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030));
        locals.var_t1dep_dn13 = ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn13)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030));
        locals.var_t1dep_dn14 = ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn14)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030));
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
        locals.var_nfactor_t = assign13230_e18112;
        locals.var_nfactor_t_dn0 = (locals.var_nfactor_i_dn0 * assign13230_e18111);
        locals.var_nfactor_t_dn2 = (locals.var_nfactor_i_dn2 * assign13230_e18111);
        locals.var_nfactor_t_dn3 = (locals.var_nfactor_i_dn3 * assign13230_e18111);
        locals.var_nfactor_t_dn4 = ((locals.var_nfactor_i_dn4 * assign13230_e18111) + (locals.var_nfactor_i * assign13230_e18111_d_n4));
        locals.var_nfactor_t_dn5 = (locals.var_nfactor_i_dn5 * assign13230_e18111);
        locals.var_nfactor_t_dn6 = (locals.var_nfactor_i_dn6 * assign13230_e18111);
        locals.var_nfactor_t_dn7 = (locals.var_nfactor_i_dn7 * assign13230_e18111);
        locals.var_nfactor_t_dn8 = (locals.var_nfactor_i_dn8 * assign13230_e18111);
        locals.var_nfactor_t_dn9 = (locals.var_nfactor_i_dn9 * assign13230_e18111);
        locals.var_nfactor_t_dn10 = (locals.var_nfactor_i_dn10 * assign13230_e18111);
        locals.var_nfactor_t_dn11 = (locals.var_nfactor_i_dn11 * assign13230_e18111);
        locals.var_nfactor_t_dn12 = (locals.var_nfactor_i_dn12 * assign13230_e18111);
        locals.var_nfactor_t_dn13 = (locals.var_nfactor_i_dn13 * assign13230_e18111);
        locals.var_nfactor_t_dn14 = (locals.var_nfactor_i_dn14 * assign13230_e18111);
        locals.var_nfactor_t_rv = 0.0;

        let assign13240_e18118: f64 = (locals.var_tratio - 1.0);
        let assign13240_e18119: f64 = (p.p851 * assign13240_e18118);
        let assign13240_e18120: f64 = (1.0 + assign13240_e18119);
        let assign13240_e18121: f64 = (locals.var_eta0_i * assign13240_e18120);
        locals.var_eta0_t = assign13240_e18121;
        locals.var_eta0_t_dn0 = (locals.var_eta0_i_dn0 * assign13240_e18120);
        locals.var_eta0_t_dn2 = (locals.var_eta0_i_dn2 * assign13240_e18120);
        locals.var_eta0_t_dn3 = (locals.var_eta0_i_dn3 * assign13240_e18120);
        locals.var_eta0_t_dn4 = ((locals.var_eta0_i_dn4 * assign13240_e18120) + (locals.var_eta0_i * (p.p851 * locals.var_tratio_dn4)));
        locals.var_eta0_t_dn5 = (locals.var_eta0_i_dn5 * assign13240_e18120);
        locals.var_eta0_t_dn6 = (locals.var_eta0_i_dn6 * assign13240_e18120);
        locals.var_eta0_t_dn7 = (locals.var_eta0_i_dn7 * assign13240_e18120);
        locals.var_eta0_t_dn8 = (locals.var_eta0_i_dn8 * assign13240_e18120);
        locals.var_eta0_t_dn9 = (locals.var_eta0_i_dn9 * assign13240_e18120);
        locals.var_eta0_t_dn10 = (locals.var_eta0_i_dn10 * assign13240_e18120);
        locals.var_eta0_t_dn11 = (locals.var_eta0_i_dn11 * assign13240_e18120);
        locals.var_eta0_t_dn12 = (locals.var_eta0_i_dn12 * assign13240_e18120);
        locals.var_eta0_t_dn13 = (locals.var_eta0_i_dn13 * assign13240_e18120);
        locals.var_eta0_t_dn14 = (locals.var_eta0_i_dn14 * assign13240_e18120);
        locals.var_eta0_t_rv = 0.0;

        let assign13250_e18124: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard453 = assign13250_e18124;
        locals.var_guard453_rv = 0.0;

        let (assign13260_e18136, assign13260_e18136_d_n0, assign13260_e18136_d_n2, assign13260_e18136_d_n3, assign13260_e18136_d_n4, assign13260_e18136_d_n5, assign13260_e18136_d_n6, assign13260_e18136_d_n7, assign13260_e18136_d_n8, assign13260_e18136_d_n9, assign13260_e18136_d_n10, assign13260_e18136_d_n11, assign13260_e18136_d_n12, assign13260_e18136_d_n13, assign13260_e18136_d_n14,) = {
    if (locals.var_guard453 != 0.0) {
        let assign13260_e18131: f64 = (locals.var_tratio - 1.0);
        let assign13260_e18132: f64 = (p.p851 * assign13260_e18131);
        let assign13260_e18133: f64 = (1.0 + assign13260_e18132);
        let assign13260_e18134: f64 = (locals.var_eta0r_i * assign13260_e18133);
        (assign13260_e18134, (locals.var_eta0r_i_dn0 * assign13260_e18133), (locals.var_eta0r_i_dn2 * assign13260_e18133), (locals.var_eta0r_i_dn3 * assign13260_e18133), ((locals.var_eta0r_i_dn4 * assign13260_e18133) + (locals.var_eta0r_i * (p.p851 * locals.var_tratio_dn4))), (locals.var_eta0r_i_dn5 * assign13260_e18133), (locals.var_eta0r_i_dn6 * assign13260_e18133), (locals.var_eta0r_i_dn7 * assign13260_e18133), (locals.var_eta0r_i_dn8 * assign13260_e18133), (locals.var_eta0r_i_dn9 * assign13260_e18133), (locals.var_eta0r_i_dn10 * assign13260_e18133), (locals.var_eta0r_i_dn11 * assign13260_e18133), (locals.var_eta0r_i_dn12 * assign13260_e18133), (locals.var_eta0r_i_dn13 * assign13260_e18133), (locals.var_eta0r_i_dn14 * assign13260_e18133),)
    } else {
        (locals.var_eta0r_t, locals.var_eta0r_t_dn0, locals.var_eta0r_t_dn2, locals.var_eta0r_t_dn3, locals.var_eta0r_t_dn4, locals.var_eta0r_t_dn5, locals.var_eta0r_t_dn6, locals.var_eta0r_t_dn7, locals.var_eta0r_t_dn8, locals.var_eta0r_t_dn9, locals.var_eta0r_t_dn10, locals.var_eta0r_t_dn11, locals.var_eta0r_t_dn12, locals.var_eta0r_t_dn13, locals.var_eta0r_t_dn14,)
    }
};
        locals.var_eta0r_t = assign13260_e18136;
        locals.var_eta0r_t_dn0 = assign13260_e18136_d_n0;
        locals.var_eta0r_t_dn2 = assign13260_e18136_d_n2;
        locals.var_eta0r_t_dn3 = assign13260_e18136_d_n3;
        locals.var_eta0r_t_dn4 = assign13260_e18136_d_n4;
        locals.var_eta0r_t_dn5 = assign13260_e18136_d_n5;
        locals.var_eta0r_t_dn6 = assign13260_e18136_d_n6;
        locals.var_eta0r_t_dn7 = assign13260_e18136_d_n7;
        locals.var_eta0r_t_dn8 = assign13260_e18136_d_n8;
        locals.var_eta0r_t_dn9 = assign13260_e18136_d_n9;
        locals.var_eta0r_t_dn10 = assign13260_e18136_d_n10;
        locals.var_eta0r_t_dn11 = assign13260_e18136_d_n11;
        locals.var_eta0r_t_dn12 = assign13260_e18136_d_n12;
        locals.var_eta0r_t_dn13 = assign13260_e18136_d_n13;
        locals.var_eta0r_t_dn14 = assign13260_e18136_d_n14;
        locals.var_eta0r_t_rv = 0.0;

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
        locals.var_u0_t = assign13280_e18151;
        locals.var_u0_t_dn0 = 0.0;
        locals.var_u0_t_dn2 = 0.0;
        locals.var_u0_t_dn3 = 0.0;
        locals.var_u0_t_dn4 = (locals.var_u0_i * if 0.0 == 0.0 && ((locals.var_ute_i) as f64).is_finite() && ((locals.var_ute_i) as f64).fract() == 0.0 { if locals.var_ute_i == 0.0 { 0.0 } else { (locals.var_ute_i * ((locals.var_tratio).powf(locals.var_ute_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13280_e18150 * (locals.var_ute_i * (locals.var_tratio_dn4 / locals.var_tratio))) });
        locals.var_u0_t_dn5 = 0.0;
        locals.var_u0_t_dn6 = 0.0;
        locals.var_u0_t_dn7 = 0.0;
        locals.var_u0_t_dn8 = 0.0;
        locals.var_u0_t_dn9 = 0.0;
        locals.var_u0_t_dn10 = 0.0;
        locals.var_u0_t_dn11 = 0.0;
        locals.var_u0_t_dn12 = 0.0;
        locals.var_u0_t_dn13 = 0.0;
        locals.var_u0_t_dn14 = 0.0;
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
        locals.var_ua_t = assign13290_e18225;
        locals.var_ua_t_dn0 = (locals.var_ua_i_dn0 * assign13290_e18224);
        locals.var_ua_t_dn2 = (locals.var_ua_i_dn2 * assign13290_e18224);
        locals.var_ua_t_dn3 = (locals.var_ua_i_dn3 * assign13290_e18224);
        locals.var_ua_t_dn4 = ((locals.var_ua_i_dn4 * assign13290_e18224) + (locals.var_ua_i * assign13290_e18224_d_n4));
        locals.var_ua_t_dn5 = (locals.var_ua_i_dn5 * assign13290_e18224);
        locals.var_ua_t_dn6 = (locals.var_ua_i_dn6 * assign13290_e18224);
        locals.var_ua_t_dn7 = (locals.var_ua_i_dn7 * assign13290_e18224);
        locals.var_ua_t_dn8 = (locals.var_ua_i_dn8 * assign13290_e18224);
        locals.var_ua_t_dn9 = (locals.var_ua_i_dn9 * assign13290_e18224);
        locals.var_ua_t_dn10 = (locals.var_ua_i_dn10 * assign13290_e18224);
        locals.var_ua_t_dn11 = (locals.var_ua_i_dn11 * assign13290_e18224);
        locals.var_ua_t_dn12 = (locals.var_ua_i_dn12 * assign13290_e18224);
        locals.var_ua_t_dn13 = (locals.var_ua_i_dn13 * assign13290_e18224);
        locals.var_ua_t_dn14 = (locals.var_ua_i_dn14 * assign13290_e18224);
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
        locals.var_uc_t = assign13300_e18299;
        locals.var_uc_t_dn0 = (locals.var_uc_i_dn0 * assign13300_e18298);
        locals.var_uc_t_dn2 = (locals.var_uc_i_dn2 * assign13300_e18298);
        locals.var_uc_t_dn3 = (locals.var_uc_i_dn3 * assign13300_e18298);
        locals.var_uc_t_dn4 = ((locals.var_uc_i_dn4 * assign13300_e18298) + (locals.var_uc_i * assign13300_e18298_d_n4));
        locals.var_uc_t_dn5 = (locals.var_uc_i_dn5 * assign13300_e18298);
        locals.var_uc_t_dn6 = (locals.var_uc_i_dn6 * assign13300_e18298);
        locals.var_uc_t_dn7 = (locals.var_uc_i_dn7 * assign13300_e18298);
        locals.var_uc_t_dn8 = (locals.var_uc_i_dn8 * assign13300_e18298);
        locals.var_uc_t_dn9 = (locals.var_uc_i_dn9 * assign13300_e18298);
        locals.var_uc_t_dn10 = (locals.var_uc_i_dn10 * assign13300_e18298);
        locals.var_uc_t_dn11 = (locals.var_uc_i_dn11 * assign13300_e18298);
        locals.var_uc_t_dn12 = (locals.var_uc_i_dn12 * assign13300_e18298);
        locals.var_uc_t_dn13 = (locals.var_uc_i_dn13 * assign13300_e18298);
        locals.var_uc_t_dn14 = (locals.var_uc_i_dn14 * assign13300_e18298);
        locals.var_uc_t_rv = 0.0;

        let assign13310_e18303: f64 = (locals.var_tratio).powf(locals.var_ud1_i);
        let assign13310_e18304: f64 = (locals.var_ud_i * assign13310_e18303);
        locals.var_ud_t = assign13310_e18304;
        locals.var_ud_t_dn0 = (locals.var_ud_i_dn0 * assign13310_e18303);
        locals.var_ud_t_dn2 = (locals.var_ud_i_dn2 * assign13310_e18303);
        locals.var_ud_t_dn3 = (locals.var_ud_i_dn3 * assign13310_e18303);
        locals.var_ud_t_dn4 = ((locals.var_ud_i_dn4 * assign13310_e18303) + (locals.var_ud_i * if 0.0 == 0.0 && ((locals.var_ud1_i) as f64).is_finite() && ((locals.var_ud1_i) as f64).fract() == 0.0 { if locals.var_ud1_i == 0.0 { 0.0 } else { (locals.var_ud1_i * ((locals.var_tratio).powf(locals.var_ud1_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13310_e18303 * (locals.var_ud1_i * (locals.var_tratio_dn4 / locals.var_tratio))) }));
        locals.var_ud_t_dn5 = (locals.var_ud_i_dn5 * assign13310_e18303);
        locals.var_ud_t_dn6 = (locals.var_ud_i_dn6 * assign13310_e18303);
        locals.var_ud_t_dn7 = (locals.var_ud_i_dn7 * assign13310_e18303);
        locals.var_ud_t_dn8 = (locals.var_ud_i_dn8 * assign13310_e18303);
        locals.var_ud_t_dn9 = (locals.var_ud_i_dn9 * assign13310_e18303);
        locals.var_ud_t_dn10 = (locals.var_ud_i_dn10 * assign13310_e18303);
        locals.var_ud_t_dn11 = (locals.var_ud_i_dn11 * assign13310_e18303);
        locals.var_ud_t_dn12 = (locals.var_ud_i_dn12 * assign13310_e18303);
        locals.var_ud_t_dn13 = (locals.var_ud_i_dn13 * assign13310_e18303);
        locals.var_ud_t_dn14 = (locals.var_ud_i_dn14 * assign13310_e18303);
        locals.var_ud_t_rv = 0.0;

        let assign13320_e18308: f64 = (locals.var_tratio).powf(locals.var_ucste_i);
        let assign13320_e18309: f64 = (locals.var_ucs_i * assign13320_e18308);
        locals.var_ucs_t = assign13320_e18309;
        locals.var_ucs_t_dn4 = (locals.var_ucs_i * if 0.0 == 0.0 && ((locals.var_ucste_i) as f64).is_finite() && ((locals.var_ucste_i) as f64).fract() == 0.0 { if locals.var_ucste_i == 0.0 { 0.0 } else { (locals.var_ucste_i * ((locals.var_tratio).powf(locals.var_ucste_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13320_e18308 * (locals.var_ucste_i * (locals.var_tratio_dn4 / locals.var_tratio))) });
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
        locals.var_eu_t = assign13330_e18383;
        locals.var_eu_t_dn0 = (locals.var_eu_i_dn0 * assign13330_e18382);
        locals.var_eu_t_dn2 = (locals.var_eu_i_dn2 * assign13330_e18382);
        locals.var_eu_t_dn3 = (locals.var_eu_i_dn3 * assign13330_e18382);
        locals.var_eu_t_dn4 = ((locals.var_eu_i_dn4 * assign13330_e18382) + (locals.var_eu_i * assign13330_e18382_d_n4));
        locals.var_eu_t_dn5 = (locals.var_eu_i_dn5 * assign13330_e18382);
        locals.var_eu_t_dn6 = (locals.var_eu_i_dn6 * assign13330_e18382);
        locals.var_eu_t_dn7 = (locals.var_eu_i_dn7 * assign13330_e18382);
        locals.var_eu_t_dn8 = (locals.var_eu_i_dn8 * assign13330_e18382);
        locals.var_eu_t_dn9 = (locals.var_eu_i_dn9 * assign13330_e18382);
        locals.var_eu_t_dn10 = (locals.var_eu_i_dn10 * assign13330_e18382);
        locals.var_eu_t_dn11 = (locals.var_eu_i_dn11 * assign13330_e18382);
        locals.var_eu_t_dn12 = (locals.var_eu_i_dn12 * assign13330_e18382);
        locals.var_eu_t_dn13 = (locals.var_eu_i_dn13 * assign13330_e18382);
        locals.var_eu_t_dn14 = (locals.var_eu_i_dn14 * assign13330_e18382);
        locals.var_eu_t_rv = 0.0;

        let assign13340_e18386: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard454 = assign13340_e18386;
        locals.var_guard454_rv = 0.0;

        let (assign13350_e18394, assign13350_e18394_d_n4,) = {
    if (locals.var_guard454 != 0.0) {
        let assign13350_e18391: f64 = (locals.var_tratio).powf(locals.var_ute_i);
        let assign13350_e18392: f64 = (locals.var_u0r_i * assign13350_e18391);
        (assign13350_e18392, (locals.var_u0r_i * if 0.0 == 0.0 && ((locals.var_ute_i) as f64).is_finite() && ((locals.var_ute_i) as f64).fract() == 0.0 { if locals.var_ute_i == 0.0 { 0.0 } else { (locals.var_ute_i * ((locals.var_tratio).powf(locals.var_ute_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13350_e18391 * (locals.var_ute_i * (locals.var_tratio_dn4 / locals.var_tratio))) }),)
    } else {
        (locals.var_u0r_t, locals.var_u0r_t_dn4,)
    }
};
        locals.var_u0r_t = assign13350_e18394;
        locals.var_u0r_t_dn4 = assign13350_e18394_d_n4;
        locals.var_u0r_t_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_23(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign13360_e18471, assign13360_e18471_d_n0, assign13360_e18471_d_n2, assign13360_e18471_d_n3, assign13360_e18471_d_n4, assign13360_e18471_d_n5, assign13360_e18471_d_n6, assign13360_e18471_d_n7, assign13360_e18471_d_n8, assign13360_e18471_d_n9, assign13360_e18471_d_n10, assign13360_e18471_d_n11, assign13360_e18471_d_n12, assign13360_e18471_d_n13, assign13360_e18471_d_n14,) = {
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
        (assign13360_e18469, (locals.var_uar_i_dn0 * assign13360_e18468), (locals.var_uar_i_dn2 * assign13360_e18468), (locals.var_uar_i_dn3 * assign13360_e18468), ((locals.var_uar_i_dn4 * assign13360_e18468) + (locals.var_uar_i * assign13360_e18468_d_n4)), (locals.var_uar_i_dn5 * assign13360_e18468), (locals.var_uar_i_dn6 * assign13360_e18468), (locals.var_uar_i_dn7 * assign13360_e18468), (locals.var_uar_i_dn8 * assign13360_e18468), (locals.var_uar_i_dn9 * assign13360_e18468), (locals.var_uar_i_dn10 * assign13360_e18468), (locals.var_uar_i_dn11 * assign13360_e18468), (locals.var_uar_i_dn12 * assign13360_e18468), (locals.var_uar_i_dn13 * assign13360_e18468), (locals.var_uar_i_dn14 * assign13360_e18468),)
    } else {
        (locals.var_uar_t, locals.var_uar_t_dn0, locals.var_uar_t_dn2, locals.var_uar_t_dn3, locals.var_uar_t_dn4, locals.var_uar_t_dn5, locals.var_uar_t_dn6, locals.var_uar_t_dn7, locals.var_uar_t_dn8, locals.var_uar_t_dn9, locals.var_uar_t_dn10, locals.var_uar_t_dn11, locals.var_uar_t_dn12, locals.var_uar_t_dn13, locals.var_uar_t_dn14,)
    }
};
        locals.var_uar_t = assign13360_e18471;
        locals.var_uar_t_dn0 = assign13360_e18471_d_n0;
        locals.var_uar_t_dn2 = assign13360_e18471_d_n2;
        locals.var_uar_t_dn3 = assign13360_e18471_d_n3;
        locals.var_uar_t_dn4 = assign13360_e18471_d_n4;
        locals.var_uar_t_dn5 = assign13360_e18471_d_n5;
        locals.var_uar_t_dn6 = assign13360_e18471_d_n6;
        locals.var_uar_t_dn7 = assign13360_e18471_d_n7;
        locals.var_uar_t_dn8 = assign13360_e18471_d_n8;
        locals.var_uar_t_dn9 = assign13360_e18471_d_n9;
        locals.var_uar_t_dn10 = assign13360_e18471_d_n10;
        locals.var_uar_t_dn11 = assign13360_e18471_d_n11;
        locals.var_uar_t_dn12 = assign13360_e18471_d_n12;
        locals.var_uar_t_dn13 = assign13360_e18471_d_n13;
        locals.var_uar_t_dn14 = assign13360_e18471_d_n14;
        locals.var_uar_t_rv = 0.0;

        let (assign13370_e18548, assign13370_e18548_d_n0, assign13370_e18548_d_n2, assign13370_e18548_d_n3, assign13370_e18548_d_n4, assign13370_e18548_d_n5, assign13370_e18548_d_n6, assign13370_e18548_d_n7, assign13370_e18548_d_n8, assign13370_e18548_d_n9, assign13370_e18548_d_n10, assign13370_e18548_d_n11, assign13370_e18548_d_n12, assign13370_e18548_d_n13, assign13370_e18548_d_n14,) = {
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
        (assign13370_e18546, (locals.var_ucr_i_dn0 * assign13370_e18545), (locals.var_ucr_i_dn2 * assign13370_e18545), (locals.var_ucr_i_dn3 * assign13370_e18545), ((locals.var_ucr_i_dn4 * assign13370_e18545) + (locals.var_ucr_i * assign13370_e18545_d_n4)), (locals.var_ucr_i_dn5 * assign13370_e18545), (locals.var_ucr_i_dn6 * assign13370_e18545), (locals.var_ucr_i_dn7 * assign13370_e18545), (locals.var_ucr_i_dn8 * assign13370_e18545), (locals.var_ucr_i_dn9 * assign13370_e18545), (locals.var_ucr_i_dn10 * assign13370_e18545), (locals.var_ucr_i_dn11 * assign13370_e18545), (locals.var_ucr_i_dn12 * assign13370_e18545), (locals.var_ucr_i_dn13 * assign13370_e18545), (locals.var_ucr_i_dn14 * assign13370_e18545),)
    } else {
        (locals.var_ucr_t, locals.var_ucr_t_dn0, locals.var_ucr_t_dn2, locals.var_ucr_t_dn3, locals.var_ucr_t_dn4, locals.var_ucr_t_dn5, locals.var_ucr_t_dn6, locals.var_ucr_t_dn7, locals.var_ucr_t_dn8, locals.var_ucr_t_dn9, locals.var_ucr_t_dn10, locals.var_ucr_t_dn11, locals.var_ucr_t_dn12, locals.var_ucr_t_dn13, locals.var_ucr_t_dn14,)
    }
};
        locals.var_ucr_t = assign13370_e18548;
        locals.var_ucr_t_dn0 = assign13370_e18548_d_n0;
        locals.var_ucr_t_dn2 = assign13370_e18548_d_n2;
        locals.var_ucr_t_dn3 = assign13370_e18548_d_n3;
        locals.var_ucr_t_dn4 = assign13370_e18548_d_n4;
        locals.var_ucr_t_dn5 = assign13370_e18548_d_n5;
        locals.var_ucr_t_dn6 = assign13370_e18548_d_n6;
        locals.var_ucr_t_dn7 = assign13370_e18548_d_n7;
        locals.var_ucr_t_dn8 = assign13370_e18548_d_n8;
        locals.var_ucr_t_dn9 = assign13370_e18548_d_n9;
        locals.var_ucr_t_dn10 = assign13370_e18548_d_n10;
        locals.var_ucr_t_dn11 = assign13370_e18548_d_n11;
        locals.var_ucr_t_dn12 = assign13370_e18548_d_n12;
        locals.var_ucr_t_dn13 = assign13370_e18548_d_n13;
        locals.var_ucr_t_dn14 = assign13370_e18548_d_n14;
        locals.var_ucr_t_rv = 0.0;

        let (assign13380_e18556, assign13380_e18556_d_n0, assign13380_e18556_d_n2, assign13380_e18556_d_n3, assign13380_e18556_d_n4, assign13380_e18556_d_n5, assign13380_e18556_d_n6, assign13380_e18556_d_n7, assign13380_e18556_d_n8, assign13380_e18556_d_n9, assign13380_e18556_d_n10, assign13380_e18556_d_n11, assign13380_e18556_d_n12, assign13380_e18556_d_n13, assign13380_e18556_d_n14,) = {
    if (locals.var_guard454 != 0.0) {
        let assign13380_e18553: f64 = (locals.var_tratio).powf(locals.var_ud1_i);
        let assign13380_e18554: f64 = (locals.var_udr_i * assign13380_e18553);
        (assign13380_e18554, (locals.var_udr_i_dn0 * assign13380_e18553), (locals.var_udr_i_dn2 * assign13380_e18553), (locals.var_udr_i_dn3 * assign13380_e18553), ((locals.var_udr_i_dn4 * assign13380_e18553) + (locals.var_udr_i * if 0.0 == 0.0 && ((locals.var_ud1_i) as f64).is_finite() && ((locals.var_ud1_i) as f64).fract() == 0.0 { if locals.var_ud1_i == 0.0 { 0.0 } else { (locals.var_ud1_i * ((locals.var_tratio).powf(locals.var_ud1_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13380_e18553 * (locals.var_ud1_i * (locals.var_tratio_dn4 / locals.var_tratio))) })), (locals.var_udr_i_dn5 * assign13380_e18553), (locals.var_udr_i_dn6 * assign13380_e18553), (locals.var_udr_i_dn7 * assign13380_e18553), (locals.var_udr_i_dn8 * assign13380_e18553), (locals.var_udr_i_dn9 * assign13380_e18553), (locals.var_udr_i_dn10 * assign13380_e18553), (locals.var_udr_i_dn11 * assign13380_e18553), (locals.var_udr_i_dn12 * assign13380_e18553), (locals.var_udr_i_dn13 * assign13380_e18553), (locals.var_udr_i_dn14 * assign13380_e18553),)
    } else {
        (locals.var_udr_t, locals.var_udr_t_dn0, locals.var_udr_t_dn2, locals.var_udr_t_dn3, locals.var_udr_t_dn4, locals.var_udr_t_dn5, locals.var_udr_t_dn6, locals.var_udr_t_dn7, locals.var_udr_t_dn8, locals.var_udr_t_dn9, locals.var_udr_t_dn10, locals.var_udr_t_dn11, locals.var_udr_t_dn12, locals.var_udr_t_dn13, locals.var_udr_t_dn14,)
    }
};
        locals.var_udr_t = assign13380_e18556;
        locals.var_udr_t_dn0 = assign13380_e18556_d_n0;
        locals.var_udr_t_dn2 = assign13380_e18556_d_n2;
        locals.var_udr_t_dn3 = assign13380_e18556_d_n3;
        locals.var_udr_t_dn4 = assign13380_e18556_d_n4;
        locals.var_udr_t_dn5 = assign13380_e18556_d_n5;
        locals.var_udr_t_dn6 = assign13380_e18556_d_n6;
        locals.var_udr_t_dn7 = assign13380_e18556_d_n7;
        locals.var_udr_t_dn8 = assign13380_e18556_d_n8;
        locals.var_udr_t_dn9 = assign13380_e18556_d_n9;
        locals.var_udr_t_dn10 = assign13380_e18556_d_n10;
        locals.var_udr_t_dn11 = assign13380_e18556_d_n11;
        locals.var_udr_t_dn12 = assign13380_e18556_d_n12;
        locals.var_udr_t_dn13 = assign13380_e18556_d_n13;
        locals.var_udr_t_dn14 = assign13380_e18556_d_n14;
        locals.var_udr_t_rv = 0.0;

        let (assign13390_e18564, assign13390_e18564_d_n4,) = {
    if (locals.var_guard454 != 0.0) {
        let assign13390_e18561: f64 = (locals.var_tratio).powf(locals.var_ucste_i);
        let assign13390_e18562: f64 = (locals.var_ucsr_i * assign13390_e18561);
        (assign13390_e18562, (locals.var_ucsr_i * if 0.0 == 0.0 && ((locals.var_ucste_i) as f64).is_finite() && ((locals.var_ucste_i) as f64).fract() == 0.0 { if locals.var_ucste_i == 0.0 { 0.0 } else { (locals.var_ucste_i * ((locals.var_tratio).powf(locals.var_ucste_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13390_e18561 * (locals.var_ucste_i * (locals.var_tratio_dn4 / locals.var_tratio))) }),)
    } else {
        (locals.var_ucsr_t, locals.var_ucsr_t_dn4,)
    }
};
        locals.var_ucsr_t = assign13390_e18564;
        locals.var_ucsr_t_dn4 = assign13390_e18564_d_n4;
        locals.var_ucsr_t_rv = 0.0;

        let assign13400_e18567: f64 = (locals.var_tratio).powf(locals.var_prt_i);
        locals.var_rdstemp = assign13400_e18567;
        locals.var_rdstemp_dn4 = if 0.0 == 0.0 && ((locals.var_prt_i) as f64).is_finite() && ((locals.var_prt_i) as f64).fract() == 0.0 { if locals.var_prt_i == 0.0 { 0.0 } else { (locals.var_prt_i * ((locals.var_tratio).powf(locals.var_prt_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13400_e18567 * (locals.var_prt_i * (locals.var_tratio_dn4 / locals.var_tratio))) };
        locals.var_rdstemp_rv = 0.0;

        let assign13410_e18571: f64 = (-locals.var_at_i);
        let assign13410_e18572: f64 = (locals.var_tratio).powf(assign13410_e18571);
        let assign13410_e18573: f64 = (locals.var_vsat_i * assign13410_e18572);
        locals.var_vsat_t = assign13410_e18573;
        locals.var_vsat_t_dn0 = (locals.var_vsat_i_dn0 * assign13410_e18572);
        locals.var_vsat_t_dn2 = (locals.var_vsat_i_dn2 * assign13410_e18572);
        locals.var_vsat_t_dn3 = (locals.var_vsat_i_dn3 * assign13410_e18572);
        locals.var_vsat_t_dn4 = ((locals.var_vsat_i_dn4 * assign13410_e18572) + (locals.var_vsat_i * if 0.0 == 0.0 && ((assign13410_e18571) as f64).is_finite() && ((assign13410_e18571) as f64).fract() == 0.0 { if assign13410_e18571 == 0.0 { 0.0 } else { (assign13410_e18571 * ((locals.var_tratio).powf(assign13410_e18571 - 1.0) * locals.var_tratio_dn4)) } } else { (assign13410_e18572 * (assign13410_e18571 * (locals.var_tratio_dn4 / locals.var_tratio))) }));
        locals.var_vsat_t_dn5 = (locals.var_vsat_i_dn5 * assign13410_e18572);
        locals.var_vsat_t_dn6 = (locals.var_vsat_i_dn6 * assign13410_e18572);
        locals.var_vsat_t_dn7 = (locals.var_vsat_i_dn7 * assign13410_e18572);
        locals.var_vsat_t_dn8 = (locals.var_vsat_i_dn8 * assign13410_e18572);
        locals.var_vsat_t_dn9 = (locals.var_vsat_i_dn9 * assign13410_e18572);
        locals.var_vsat_t_dn10 = (locals.var_vsat_i_dn10 * assign13410_e18572);
        locals.var_vsat_t_dn11 = (locals.var_vsat_i_dn11 * assign13410_e18572);
        locals.var_vsat_t_dn12 = (locals.var_vsat_i_dn12 * assign13410_e18572);
        locals.var_vsat_t_dn13 = (locals.var_vsat_i_dn13 * assign13410_e18572);
        locals.var_vsat_t_dn14 = (locals.var_vsat_i_dn14 * assign13410_e18572);
        locals.var_vsat_t_rv = 0.0;

        let assign13420_e18576: f64 = if locals.var_vsat_t < 100.0 { 1.0 } else { 0.0 };
        locals.var_guard455 = assign13420_e18576;
        locals.var_guard455_rv = 0.0;

        let (assign13430_e18580, assign13430_e18580_d_n0, assign13430_e18580_d_n2, assign13430_e18580_d_n3, assign13430_e18580_d_n4, assign13430_e18580_d_n5, assign13430_e18580_d_n6, assign13430_e18580_d_n7, assign13430_e18580_d_n8, assign13430_e18580_d_n9, assign13430_e18580_d_n10, assign13430_e18580_d_n11, assign13430_e18580_d_n12, assign13430_e18580_d_n13, assign13430_e18580_d_n14,) = {
    if (locals.var_guard455 != 0.0) {
        (100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vsat_t, locals.var_vsat_t_dn0, locals.var_vsat_t_dn2, locals.var_vsat_t_dn3, locals.var_vsat_t_dn4, locals.var_vsat_t_dn5, locals.var_vsat_t_dn6, locals.var_vsat_t_dn7, locals.var_vsat_t_dn8, locals.var_vsat_t_dn9, locals.var_vsat_t_dn10, locals.var_vsat_t_dn11, locals.var_vsat_t_dn12, locals.var_vsat_t_dn13, locals.var_vsat_t_dn14,)
    }
};
        locals.var_vsat_t = assign13430_e18580;
        locals.var_vsat_t_dn0 = assign13430_e18580_d_n0;
        locals.var_vsat_t_dn2 = assign13430_e18580_d_n2;
        locals.var_vsat_t_dn3 = assign13430_e18580_d_n3;
        locals.var_vsat_t_dn4 = assign13430_e18580_d_n4;
        locals.var_vsat_t_dn5 = assign13430_e18580_d_n5;
        locals.var_vsat_t_dn6 = assign13430_e18580_d_n6;
        locals.var_vsat_t_dn7 = assign13430_e18580_d_n7;
        locals.var_vsat_t_dn8 = assign13430_e18580_d_n8;
        locals.var_vsat_t_dn9 = assign13430_e18580_d_n9;
        locals.var_vsat_t_dn10 = assign13430_e18580_d_n10;
        locals.var_vsat_t_dn11 = assign13430_e18580_d_n11;
        locals.var_vsat_t_dn12 = assign13430_e18580_d_n12;
        locals.var_vsat_t_dn13 = assign13430_e18580_d_n13;
        locals.var_vsat_t_dn14 = assign13430_e18580_d_n14;
        locals.var_vsat_t_rv = 0.0;

        let assign13440_e18583: f64 = if p.p1094 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard456 = assign13440_e18583;
        locals.var_guard456_rv = 0.0;

        let (assign13450_e18589, assign13450_e18589_d_n4,) = {
    if (locals.var_guard456 != 0.0) {
        let assign13450_e18587: f64 = (locals.var_tratio).powf(p.p1120);
        (assign13450_e18587, if 0.0 == 0.0 && ((p.p1120) as f64).is_finite() && ((p.p1120) as f64).fract() == 0.0 { if p.p1120 == 0.0 { 0.0 } else { (p.p1120 * ((locals.var_tratio).powf(p.p1120 - 1.0) * locals.var_tratio_dn4)) } } else { (assign13450_e18587 * (p.p1120 * (locals.var_tratio_dn4 / locals.var_tratio))) },)
    } else {
        (locals.var_rdstemphv, locals.var_rdstemphv_dn4,)
    }
};
        locals.var_rdstemphv = assign13450_e18589;
        locals.var_rdstemphv_dn4 = assign13450_e18589_d_n4;
        locals.var_rdstemphv_rv = 0.0;

        let (assign13460_e18598, assign13460_e18598_d_n4,) = {
    if (locals.var_guard456 != 0.0) {
        let assign13460_e18594: f64 = (-p.p1121);
        let assign13460_e18595: f64 = (locals.var_tratio).powf(assign13460_e18594);
        let assign13460_e18596: f64 = (p.p1100 * assign13460_e18595);
        (assign13460_e18596, (p.p1100 * if 0.0 == 0.0 && ((assign13460_e18594) as f64).is_finite() && ((assign13460_e18594) as f64).fract() == 0.0 { if assign13460_e18594 == 0.0 { 0.0 } else { (assign13460_e18594 * ((locals.var_tratio).powf(assign13460_e18594 - 1.0) * locals.var_tratio_dn4)) } } else { (assign13460_e18595 * (assign13460_e18594 * (locals.var_tratio_dn4 / locals.var_tratio))) }),)
    } else {
        (locals.var_vdrift_t, locals.var_vdrift_t_dn4,)
    }
};
        locals.var_vdrift_t = assign13460_e18598;
        locals.var_vdrift_t_dn4 = assign13460_e18598_d_n4;
        locals.var_vdrift_t_rv = 0.0;

        let assign13470_e18601: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard457 = assign13470_e18601;
        locals.var_guard457_rv = 0.0;

        let (assign13480_e18610, assign13480_e18610_d_n0, assign13480_e18610_d_n2, assign13480_e18610_d_n3, assign13480_e18610_d_n4, assign13480_e18610_d_n5, assign13480_e18610_d_n6, assign13480_e18610_d_n7, assign13480_e18610_d_n8, assign13480_e18610_d_n9, assign13480_e18610_d_n10, assign13480_e18610_d_n11, assign13480_e18610_d_n12, assign13480_e18610_d_n13, assign13480_e18610_d_n14,) = {
    if (locals.var_guard457 != 0.0) {
        let assign13480_e18606: f64 = (-locals.var_at_i);
        let assign13480_e18607: f64 = (locals.var_tratio).powf(assign13480_e18606);
        let assign13480_e18608: f64 = (locals.var_vsatr_i * assign13480_e18607);
        (assign13480_e18608, (locals.var_vsatr_i_dn0 * assign13480_e18607), (locals.var_vsatr_i_dn2 * assign13480_e18607), (locals.var_vsatr_i_dn3 * assign13480_e18607), ((locals.var_vsatr_i_dn4 * assign13480_e18607) + (locals.var_vsatr_i * if 0.0 == 0.0 && ((assign13480_e18606) as f64).is_finite() && ((assign13480_e18606) as f64).fract() == 0.0 { if assign13480_e18606 == 0.0 { 0.0 } else { (assign13480_e18606 * ((locals.var_tratio).powf(assign13480_e18606 - 1.0) * locals.var_tratio_dn4)) } } else { (assign13480_e18607 * (assign13480_e18606 * (locals.var_tratio_dn4 / locals.var_tratio))) })), (locals.var_vsatr_i_dn5 * assign13480_e18607), (locals.var_vsatr_i_dn6 * assign13480_e18607), (locals.var_vsatr_i_dn7 * assign13480_e18607), (locals.var_vsatr_i_dn8 * assign13480_e18607), (locals.var_vsatr_i_dn9 * assign13480_e18607), (locals.var_vsatr_i_dn10 * assign13480_e18607), (locals.var_vsatr_i_dn11 * assign13480_e18607), (locals.var_vsatr_i_dn12 * assign13480_e18607), (locals.var_vsatr_i_dn13 * assign13480_e18607), (locals.var_vsatr_i_dn14 * assign13480_e18607),)
    } else {
        (locals.var_vsatr_t, locals.var_vsatr_t_dn0, locals.var_vsatr_t_dn2, locals.var_vsatr_t_dn3, locals.var_vsatr_t_dn4, locals.var_vsatr_t_dn5, locals.var_vsatr_t_dn6, locals.var_vsatr_t_dn7, locals.var_vsatr_t_dn8, locals.var_vsatr_t_dn9, locals.var_vsatr_t_dn10, locals.var_vsatr_t_dn11, locals.var_vsatr_t_dn12, locals.var_vsatr_t_dn13, locals.var_vsatr_t_dn14,)
    }
};
        locals.var_vsatr_t = assign13480_e18610;
        locals.var_vsatr_t_dn0 = assign13480_e18610_d_n0;
        locals.var_vsatr_t_dn2 = assign13480_e18610_d_n2;
        locals.var_vsatr_t_dn3 = assign13480_e18610_d_n3;
        locals.var_vsatr_t_dn4 = assign13480_e18610_d_n4;
        locals.var_vsatr_t_dn5 = assign13480_e18610_d_n5;
        locals.var_vsatr_t_dn6 = assign13480_e18610_d_n6;
        locals.var_vsatr_t_dn7 = assign13480_e18610_d_n7;
        locals.var_vsatr_t_dn8 = assign13480_e18610_d_n8;
        locals.var_vsatr_t_dn9 = assign13480_e18610_d_n9;
        locals.var_vsatr_t_dn10 = assign13480_e18610_d_n10;
        locals.var_vsatr_t_dn11 = assign13480_e18610_d_n11;
        locals.var_vsatr_t_dn12 = assign13480_e18610_d_n12;
        locals.var_vsatr_t_dn13 = assign13480_e18610_d_n13;
        locals.var_vsatr_t_dn14 = assign13480_e18610_d_n14;
        locals.var_vsatr_t_rv = 0.0;

        let assign13490_e18613: f64 = if locals.var_vsatr_t < 100.0 { 1.0 } else { 0.0 };
        locals.var_guard458 = assign13490_e18613;
        locals.var_guard458_rv = 0.0;

        let (assign13500_e18619, assign13500_e18619_d_n0, assign13500_e18619_d_n2, assign13500_e18619_d_n3, assign13500_e18619_d_n4, assign13500_e18619_d_n5, assign13500_e18619_d_n6, assign13500_e18619_d_n7, assign13500_e18619_d_n8, assign13500_e18619_d_n9, assign13500_e18619_d_n10, assign13500_e18619_d_n11, assign13500_e18619_d_n12, assign13500_e18619_d_n13, assign13500_e18619_d_n14,) = {
    if ((locals.var_guard457 != 0.0) && (locals.var_guard458 != 0.0)) {
        (100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vsatr_t, locals.var_vsatr_t_dn0, locals.var_vsatr_t_dn2, locals.var_vsatr_t_dn3, locals.var_vsatr_t_dn4, locals.var_vsatr_t_dn5, locals.var_vsatr_t_dn6, locals.var_vsatr_t_dn7, locals.var_vsatr_t_dn8, locals.var_vsatr_t_dn9, locals.var_vsatr_t_dn10, locals.var_vsatr_t_dn11, locals.var_vsatr_t_dn12, locals.var_vsatr_t_dn13, locals.var_vsatr_t_dn14,)
    }
};
        locals.var_vsatr_t = assign13500_e18619;
        locals.var_vsatr_t_dn0 = assign13500_e18619_d_n0;
        locals.var_vsatr_t_dn2 = assign13500_e18619_d_n2;
        locals.var_vsatr_t_dn3 = assign13500_e18619_d_n3;
        locals.var_vsatr_t_dn4 = assign13500_e18619_d_n4;
        locals.var_vsatr_t_dn5 = assign13500_e18619_d_n5;
        locals.var_vsatr_t_dn6 = assign13500_e18619_d_n6;
        locals.var_vsatr_t_dn7 = assign13500_e18619_d_n7;
        locals.var_vsatr_t_dn8 = assign13500_e18619_d_n8;
        locals.var_vsatr_t_dn9 = assign13500_e18619_d_n9;
        locals.var_vsatr_t_dn10 = assign13500_e18619_d_n10;
        locals.var_vsatr_t_dn11 = assign13500_e18619_d_n11;
        locals.var_vsatr_t_dn12 = assign13500_e18619_d_n12;
        locals.var_vsatr_t_dn13 = assign13500_e18619_d_n13;
        locals.var_vsatr_t_dn14 = assign13500_e18619_d_n14;
        locals.var_vsatr_t_rv = 0.0;

        let assign13510_e18623: f64 = (-locals.var_at_i);
        let assign13510_e18624: f64 = (locals.var_tratio).powf(assign13510_e18623);
        let assign13510_e18625: f64 = (locals.var_vsatcv_i * assign13510_e18624);
        locals.var_vsatcv_t = assign13510_e18625;
        locals.var_vsatcv_t_dn0 = (locals.var_vsatcv_i_dn0 * assign13510_e18624);
        locals.var_vsatcv_t_dn2 = (locals.var_vsatcv_i_dn2 * assign13510_e18624);
        locals.var_vsatcv_t_dn3 = (locals.var_vsatcv_i_dn3 * assign13510_e18624);
        locals.var_vsatcv_t_dn4 = ((locals.var_vsatcv_i_dn4 * assign13510_e18624) + (locals.var_vsatcv_i * if 0.0 == 0.0 && ((assign13510_e18623) as f64).is_finite() && ((assign13510_e18623) as f64).fract() == 0.0 { if assign13510_e18623 == 0.0 { 0.0 } else { (assign13510_e18623 * ((locals.var_tratio).powf(assign13510_e18623 - 1.0) * locals.var_tratio_dn4)) } } else { (assign13510_e18624 * (assign13510_e18623 * (locals.var_tratio_dn4 / locals.var_tratio))) }));
        locals.var_vsatcv_t_dn5 = (locals.var_vsatcv_i_dn5 * assign13510_e18624);
        locals.var_vsatcv_t_dn6 = (locals.var_vsatcv_i_dn6 * assign13510_e18624);
        locals.var_vsatcv_t_dn7 = (locals.var_vsatcv_i_dn7 * assign13510_e18624);
        locals.var_vsatcv_t_dn8 = (locals.var_vsatcv_i_dn8 * assign13510_e18624);
        locals.var_vsatcv_t_dn9 = (locals.var_vsatcv_i_dn9 * assign13510_e18624);
        locals.var_vsatcv_t_dn10 = (locals.var_vsatcv_i_dn10 * assign13510_e18624);
        locals.var_vsatcv_t_dn11 = (locals.var_vsatcv_i_dn11 * assign13510_e18624);
        locals.var_vsatcv_t_dn12 = (locals.var_vsatcv_i_dn12 * assign13510_e18624);
        locals.var_vsatcv_t_dn13 = (locals.var_vsatcv_i_dn13 * assign13510_e18624);
        locals.var_vsatcv_t_dn14 = (locals.var_vsatcv_i_dn14 * assign13510_e18624);
        locals.var_vsatcv_t_rv = 0.0;

        let assign13520_e18628: f64 = if locals.var_vsatcv_t < 100.0 { 1.0 } else { 0.0 };
        locals.var_guard459 = assign13520_e18628;
        locals.var_guard459_rv = 0.0;

        let (assign13530_e18632, assign13530_e18632_d_n0, assign13530_e18632_d_n2, assign13530_e18632_d_n3, assign13530_e18632_d_n4, assign13530_e18632_d_n5, assign13530_e18632_d_n6, assign13530_e18632_d_n7, assign13530_e18632_d_n8, assign13530_e18632_d_n9, assign13530_e18632_d_n10, assign13530_e18632_d_n11, assign13530_e18632_d_n12, assign13530_e18632_d_n13, assign13530_e18632_d_n14,) = {
    if (locals.var_guard459 != 0.0) {
        (100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vsatcv_t, locals.var_vsatcv_t_dn0, locals.var_vsatcv_t_dn2, locals.var_vsatcv_t_dn3, locals.var_vsatcv_t_dn4, locals.var_vsatcv_t_dn5, locals.var_vsatcv_t_dn6, locals.var_vsatcv_t_dn7, locals.var_vsatcv_t_dn8, locals.var_vsatcv_t_dn9, locals.var_vsatcv_t_dn10, locals.var_vsatcv_t_dn11, locals.var_vsatcv_t_dn12, locals.var_vsatcv_t_dn13, locals.var_vsatcv_t_dn14,)
    }
};
        locals.var_vsatcv_t = assign13530_e18632;
        locals.var_vsatcv_t_dn0 = assign13530_e18632_d_n0;
        locals.var_vsatcv_t_dn2 = assign13530_e18632_d_n2;
        locals.var_vsatcv_t_dn3 = assign13530_e18632_d_n3;
        locals.var_vsatcv_t_dn4 = assign13530_e18632_d_n4;
        locals.var_vsatcv_t_dn5 = assign13530_e18632_d_n5;
        locals.var_vsatcv_t_dn6 = assign13530_e18632_d_n6;
        locals.var_vsatcv_t_dn7 = assign13530_e18632_d_n7;
        locals.var_vsatcv_t_dn8 = assign13530_e18632_d_n8;
        locals.var_vsatcv_t_dn9 = assign13530_e18632_d_n9;
        locals.var_vsatcv_t_dn10 = assign13530_e18632_d_n10;
        locals.var_vsatcv_t_dn11 = assign13530_e18632_d_n11;
        locals.var_vsatcv_t_dn12 = assign13530_e18632_d_n12;
        locals.var_vsatcv_t_dn13 = assign13530_e18632_d_n13;
        locals.var_vsatcv_t_dn14 = assign13530_e18632_d_n14;
        locals.var_vsatcv_t_rv = 0.0;

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
        locals.var_delta_t = assign13540_e18732;
        locals.var_delta_t_dn0 = (-(assign13540_e18729_d_n0 / (assign13540_e18731 * assign13540_e18731)));
        locals.var_delta_t_dn2 = (-(assign13540_e18729_d_n2 / (assign13540_e18731 * assign13540_e18731)));
        locals.var_delta_t_dn3 = (-(assign13540_e18729_d_n3 / (assign13540_e18731 * assign13540_e18731)));
        locals.var_delta_t_dn4 = (-(assign13540_e18729_d_n4 / (assign13540_e18731 * assign13540_e18731)));
        locals.var_delta_t_dn5 = (-(assign13540_e18729_d_n5 / (assign13540_e18731 * assign13540_e18731)));
        locals.var_delta_t_dn6 = (-(assign13540_e18729_d_n6 / (assign13540_e18731 * assign13540_e18731)));
        locals.var_delta_t_dn7 = (-(assign13540_e18729_d_n7 / (assign13540_e18731 * assign13540_e18731)));
        locals.var_delta_t_dn8 = (-(assign13540_e18729_d_n8 / (assign13540_e18731 * assign13540_e18731)));
        locals.var_delta_t_dn9 = (-(assign13540_e18729_d_n9 / (assign13540_e18731 * assign13540_e18731)));
        locals.var_delta_t_dn10 = (-(assign13540_e18729_d_n10 / (assign13540_e18731 * assign13540_e18731)));
        locals.var_delta_t_dn11 = (-(assign13540_e18729_d_n11 / (assign13540_e18731 * assign13540_e18731)));
        locals.var_delta_t_dn12 = (-(assign13540_e18729_d_n12 / (assign13540_e18731 * assign13540_e18731)));
        locals.var_delta_t_dn13 = (-(assign13540_e18729_d_n13 / (assign13540_e18731 * assign13540_e18731)));
        locals.var_delta_t_dn14 = (-(assign13540_e18729_d_n14 / (assign13540_e18731 * assign13540_e18731)));
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
        locals.var_ptwg_t = assign13550_e18806;
        locals.var_ptwg_t_dn0 = (locals.var_ptwg_i_dn0 * assign13550_e18805);
        locals.var_ptwg_t_dn2 = (locals.var_ptwg_i_dn2 * assign13550_e18805);
        locals.var_ptwg_t_dn3 = (locals.var_ptwg_i_dn3 * assign13550_e18805);
        locals.var_ptwg_t_dn4 = ((locals.var_ptwg_i_dn4 * assign13550_e18805) + (locals.var_ptwg_i * assign13550_e18805_d_n4));
        locals.var_ptwg_t_dn5 = (locals.var_ptwg_i_dn5 * assign13550_e18805);
        locals.var_ptwg_t_dn6 = (locals.var_ptwg_i_dn6 * assign13550_e18805);
        locals.var_ptwg_t_dn7 = (locals.var_ptwg_i_dn7 * assign13550_e18805);
        locals.var_ptwg_t_dn8 = (locals.var_ptwg_i_dn8 * assign13550_e18805);
        locals.var_ptwg_t_dn9 = (locals.var_ptwg_i_dn9 * assign13550_e18805);
        locals.var_ptwg_t_dn10 = (locals.var_ptwg_i_dn10 * assign13550_e18805);
        locals.var_ptwg_t_dn11 = (locals.var_ptwg_i_dn11 * assign13550_e18805);
        locals.var_ptwg_t_dn12 = (locals.var_ptwg_i_dn12 * assign13550_e18805);
        locals.var_ptwg_t_dn13 = (locals.var_ptwg_i_dn13 * assign13550_e18805);
        locals.var_ptwg_t_dn14 = (locals.var_ptwg_i_dn14 * assign13550_e18805);
        locals.var_ptwg_t_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_24(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign13560_e18809: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard460 = assign13560_e18809;
        locals.var_guard460_rv = 0.0;

        let (assign13570_e18886, assign13570_e18886_d_n0, assign13570_e18886_d_n2, assign13570_e18886_d_n3, assign13570_e18886_d_n4, assign13570_e18886_d_n5, assign13570_e18886_d_n6, assign13570_e18886_d_n7, assign13570_e18886_d_n8, assign13570_e18886_d_n9, assign13570_e18886_d_n10, assign13570_e18886_d_n11, assign13570_e18886_d_n12, assign13570_e18886_d_n13, assign13570_e18886_d_n14,) = {
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
        (assign13570_e18884, (locals.var_ptwgr_i_dn0 * assign13570_e18883), (locals.var_ptwgr_i_dn2 * assign13570_e18883), (locals.var_ptwgr_i_dn3 * assign13570_e18883), ((locals.var_ptwgr_i_dn4 * assign13570_e18883) + (locals.var_ptwgr_i * assign13570_e18883_d_n4)), (locals.var_ptwgr_i_dn5 * assign13570_e18883), (locals.var_ptwgr_i_dn6 * assign13570_e18883), (locals.var_ptwgr_i_dn7 * assign13570_e18883), (locals.var_ptwgr_i_dn8 * assign13570_e18883), (locals.var_ptwgr_i_dn9 * assign13570_e18883), (locals.var_ptwgr_i_dn10 * assign13570_e18883), (locals.var_ptwgr_i_dn11 * assign13570_e18883), (locals.var_ptwgr_i_dn12 * assign13570_e18883), (locals.var_ptwgr_i_dn13 * assign13570_e18883), (locals.var_ptwgr_i_dn14 * assign13570_e18883),)
    } else {
        (locals.var_ptwgr_t, locals.var_ptwgr_t_dn0, locals.var_ptwgr_t_dn2, locals.var_ptwgr_t_dn3, locals.var_ptwgr_t_dn4, locals.var_ptwgr_t_dn5, locals.var_ptwgr_t_dn6, locals.var_ptwgr_t_dn7, locals.var_ptwgr_t_dn8, locals.var_ptwgr_t_dn9, locals.var_ptwgr_t_dn10, locals.var_ptwgr_t_dn11, locals.var_ptwgr_t_dn12, locals.var_ptwgr_t_dn13, locals.var_ptwgr_t_dn14,)
    }
};
        locals.var_ptwgr_t = assign13570_e18886;
        locals.var_ptwgr_t_dn0 = assign13570_e18886_d_n0;
        locals.var_ptwgr_t_dn2 = assign13570_e18886_d_n2;
        locals.var_ptwgr_t_dn3 = assign13570_e18886_d_n3;
        locals.var_ptwgr_t_dn4 = assign13570_e18886_d_n4;
        locals.var_ptwgr_t_dn5 = assign13570_e18886_d_n5;
        locals.var_ptwgr_t_dn6 = assign13570_e18886_d_n6;
        locals.var_ptwgr_t_dn7 = assign13570_e18886_d_n7;
        locals.var_ptwgr_t_dn8 = assign13570_e18886_d_n8;
        locals.var_ptwgr_t_dn9 = assign13570_e18886_d_n9;
        locals.var_ptwgr_t_dn10 = assign13570_e18886_d_n10;
        locals.var_ptwgr_t_dn11 = assign13570_e18886_d_n11;
        locals.var_ptwgr_t_dn12 = assign13570_e18886_d_n12;
        locals.var_ptwgr_t_dn13 = assign13570_e18886_d_n13;
        locals.var_ptwgr_t_dn14 = assign13570_e18886_d_n14;
        locals.var_ptwgr_t_rv = 0.0;

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
        locals.var_a1_t = assign13580_e18960;
        locals.var_a1_t_dn4 = (locals.var_a1_i * assign13580_e18959_d_n4);
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
        locals.var_a2_t = assign13590_e19034;
        locals.var_a2_t_dn4 = (locals.var_a2_i * assign13590_e19033_d_n4);
        locals.var_a2_t_rv = 0.0;

        let assign13600_e19038: f64 = (locals.var_tratio).powf(locals.var_iit_i);
        let assign13600_e19039: f64 = (locals.var_beta0_i * assign13600_e19038);
        locals.var_beta0_t = assign13600_e19039;
        locals.var_beta0_t_dn0 = (locals.var_beta0_i_dn0 * assign13600_e19038);
        locals.var_beta0_t_dn2 = (locals.var_beta0_i_dn2 * assign13600_e19038);
        locals.var_beta0_t_dn3 = (locals.var_beta0_i_dn3 * assign13600_e19038);
        locals.var_beta0_t_dn4 = ((locals.var_beta0_i_dn4 * assign13600_e19038) + (locals.var_beta0_i * if 0.0 == 0.0 && ((locals.var_iit_i) as f64).is_finite() && ((locals.var_iit_i) as f64).fract() == 0.0 { if locals.var_iit_i == 0.0 { 0.0 } else { (locals.var_iit_i * ((locals.var_tratio).powf(locals.var_iit_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13600_e19038 * (locals.var_iit_i * (locals.var_tratio_dn4 / locals.var_tratio))) }));
        locals.var_beta0_t_dn5 = (locals.var_beta0_i_dn5 * assign13600_e19038);
        locals.var_beta0_t_dn6 = (locals.var_beta0_i_dn6 * assign13600_e19038);
        locals.var_beta0_t_dn7 = (locals.var_beta0_i_dn7 * assign13600_e19038);
        locals.var_beta0_t_dn8 = (locals.var_beta0_i_dn8 * assign13600_e19038);
        locals.var_beta0_t_dn9 = (locals.var_beta0_i_dn9 * assign13600_e19038);
        locals.var_beta0_t_dn10 = (locals.var_beta0_i_dn10 * assign13600_e19038);
        locals.var_beta0_t_dn11 = (locals.var_beta0_i_dn11 * assign13600_e19038);
        locals.var_beta0_t_dn12 = (locals.var_beta0_i_dn12 * assign13600_e19038);
        locals.var_beta0_t_dn13 = (locals.var_beta0_i_dn13 * assign13600_e19038);
        locals.var_beta0_t_dn14 = (locals.var_beta0_i_dn14 * assign13600_e19038);
        locals.var_beta0_t_rv = 0.0;

        let assign13610_e19042: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard461 = assign13610_e19042;
        locals.var_guard461_rv = 0.0;

        let (assign13620_e19050, assign13620_e19050_d_n4,) = {
    if (locals.var_guard461 != 0.0) {
        let assign13620_e19047: f64 = (locals.var_tratio).powf(locals.var_iit_i);
        let assign13620_e19048: f64 = (locals.var_beta0r_i * assign13620_e19047);
        (assign13620_e19048, (locals.var_beta0r_i * if 0.0 == 0.0 && ((locals.var_iit_i) as f64).is_finite() && ((locals.var_iit_i) as f64).fract() == 0.0 { if locals.var_iit_i == 0.0 { 0.0 } else { (locals.var_iit_i * ((locals.var_tratio).powf(locals.var_iit_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13620_e19047 * (locals.var_iit_i * (locals.var_tratio_dn4 / locals.var_tratio))) }),)
    } else {
        (locals.var_beta0r_t, locals.var_beta0r_t_dn4,)
    }
};
        locals.var_beta0r_t = assign13620_e19050;
        locals.var_beta0r_t_dn4 = assign13620_e19050_d_n4;
        locals.var_beta0r_t_rv = 0.0;

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
        locals.var_bgidl_t = assign13630_e19124;
        locals.var_bgidl_t_dn4 = (locals.var_bgidl_i * assign13630_e19123_d_n4);
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
        locals.var_bgisl_t = assign13640_e19198;
        locals.var_bgisl_t_dn4 = (locals.var_bgisl_i * assign13640_e19197_d_n4);
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
        locals.var_k0_t = assign13660_e19279;
        locals.var_k0_t_dn4 = (locals.var_k0_i * assign13660_e19278_d_n4);
        locals.var_k0_t_rv = 0.0;

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
        locals.var_m0_t = assign13670_e19353;
        locals.var_m0_t_dn4 = (locals.var_m0_i * assign13670_e19352_d_n4);
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
        locals.var_c0_t = assign13680_e19427;
        locals.var_c0_t_dn4 = (locals.var_c0_i * assign13680_e19426_d_n4);
        locals.var_c0_t_rv = 0.0;

    }
}
