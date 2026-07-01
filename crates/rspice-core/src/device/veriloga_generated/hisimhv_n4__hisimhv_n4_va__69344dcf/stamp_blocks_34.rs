#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_169(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign48920_e73349, assign48920_e73349_d_n0, assign48920_e73349_d_n2, assign48920_e73349_d_n4, assign48920_e73349_d_n5, assign48920_e73349_d_n6, assign48920_e73349_d_n7, assign48920_e73349_d_n8, assign48920_e73349_d_n9, assign48920_e73349_d_n10, assign48920_e73349_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) {
        let assign48920_e73346: f64 = (locals.var_phi_sl_dep__blk1090 - locals.var_phi_bl_dep__blk1093);
        let assign48920_e73347: f64 = (locals.var_beta * assign48920_e73346);
        (assign48920_e73347, ((locals.var_beta_dn0 * assign48920_e73346) + (locals.var_beta * (locals.var_phi_sl_dep__blk1090_dn0 - locals.var_phi_bl_dep__blk1093_dn0))), ((locals.var_beta_dn2 * assign48920_e73346) + (locals.var_beta * (locals.var_phi_sl_dep__blk1090_dn2 - locals.var_phi_bl_dep__blk1093_dn2))), ((locals.var_beta_dn4 * assign48920_e73346) + (locals.var_beta * (locals.var_phi_sl_dep__blk1090_dn4 - locals.var_phi_bl_dep__blk1093_dn4))), ((locals.var_beta_dn5 * assign48920_e73346) + (locals.var_beta * (locals.var_phi_sl_dep__blk1090_dn5 - locals.var_phi_bl_dep__blk1093_dn5))), ((locals.var_beta_dn6 * assign48920_e73346) + (locals.var_beta * (locals.var_phi_sl_dep__blk1090_dn6 - locals.var_phi_bl_dep__blk1093_dn6))), ((locals.var_beta_dn7 * assign48920_e73346) + (locals.var_beta * (locals.var_phi_sl_dep__blk1090_dn7 - locals.var_phi_bl_dep__blk1093_dn7))), ((locals.var_beta_dn8 * assign48920_e73346) + (locals.var_beta * (locals.var_phi_sl_dep__blk1090_dn8 - locals.var_phi_bl_dep__blk1093_dn8))), ((locals.var_beta_dn9 * assign48920_e73346) + (locals.var_beta * (locals.var_phi_sl_dep__blk1090_dn9 - locals.var_phi_bl_dep__blk1093_dn9))), ((locals.var_beta_dn10 * assign48920_e73346) + (locals.var_beta * (locals.var_phi_sl_dep__blk1090_dn10 - locals.var_phi_bl_dep__blk1093_dn10))), ((locals.var_beta_dn13 * assign48920_e73346) + (locals.var_beta * (locals.var_phi_sl_dep__blk1090_dn13 - locals.var_phi_bl_dep__blk1093_dn13))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign48920_e73349;
        locals.var_t5_dn0 = assign48920_e73349_d_n0;
        locals.var_t5_dn2 = assign48920_e73349_d_n2;
        locals.var_t5_dn4 = assign48920_e73349_d_n4;
        locals.var_t5_dn5 = assign48920_e73349_d_n5;
        locals.var_t5_dn6 = assign48920_e73349_d_n6;
        locals.var_t5_dn7 = assign48920_e73349_d_n7;
        locals.var_t5_dn8 = assign48920_e73349_d_n8;
        locals.var_t5_dn9 = assign48920_e73349_d_n9;
        locals.var_t5_dn10 = assign48920_e73349_d_n10;
        locals.var_t5_dn13 = assign48920_e73349_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign48930_e73364, assign48930_e73364_d_n0, assign48930_e73364_d_n2, assign48930_e73364_d_n4, assign48930_e73364_d_n5, assign48930_e73364_d_n6, assign48930_e73364_d_n7, assign48930_e73364_d_n8, assign48930_e73364_d_n9, assign48930_e73364_d_n10, assign48930_e73364_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) {
        let assign48930_e73362: f64 = (locals.var_t5).exp();
        (assign48930_e73362, (assign48930_e73362 * locals.var_t5_dn0), (assign48930_e73362 * locals.var_t5_dn2), (assign48930_e73362 * locals.var_t5_dn4), (assign48930_e73362 * locals.var_t5_dn5), (assign48930_e73362 * locals.var_t5_dn6), (assign48930_e73362 * locals.var_t5_dn7), (assign48930_e73362 * locals.var_t5_dn8), (assign48930_e73362 * locals.var_t5_dn9), (assign48930_e73362 * locals.var_t5_dn10), (assign48930_e73362 * locals.var_t5_dn13),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign48930_e73364;
        locals.var_t6_dn0 = assign48930_e73364_d_n0;
        locals.var_t6_dn2 = assign48930_e73364_d_n2;
        locals.var_t6_dn4 = assign48930_e73364_d_n4;
        locals.var_t6_dn5 = assign48930_e73364_d_n5;
        locals.var_t6_dn6 = assign48930_e73364_d_n6;
        locals.var_t6_dn7 = assign48930_e73364_d_n7;
        locals.var_t6_dn8 = assign48930_e73364_d_n8;
        locals.var_t6_dn9 = assign48930_e73364_d_n9;
        locals.var_t6_dn10 = assign48930_e73364_d_n10;
        locals.var_t6_dn13 = assign48930_e73364_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign48940_e73384, assign48940_e73384_d_n0, assign48940_e73384_d_n2, assign48940_e73384_d_n4, assign48940_e73384_d_n5, assign48940_e73384_d_n6, assign48940_e73384_d_n7, assign48940_e73384_d_n8, assign48940_e73384_d_n9, assign48940_e73384_d_n10, assign48940_e73384_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) {
        let assign48940_e73378: f64 = (locals.var_t6 - 1.0);
        let assign48940_e73380: f64 = (assign48940_e73378 - locals.var_t5);
        let assign48940_e73382: f64 = (assign48940_e73380 + 1e-15);
        (assign48940_e73382, (locals.var_t6_dn0 - locals.var_t5_dn0), (locals.var_t6_dn2 - locals.var_t5_dn2), (locals.var_t6_dn4 - locals.var_t5_dn4), (locals.var_t6_dn5 - locals.var_t5_dn5), (locals.var_t6_dn6 - locals.var_t5_dn6), (locals.var_t6_dn7 - locals.var_t5_dn7), (locals.var_t6_dn8 - locals.var_t5_dn8), (locals.var_t6_dn9 - locals.var_t5_dn9), (locals.var_t6_dn10 - locals.var_t5_dn10), (locals.var_t6_dn13 - locals.var_t5_dn13),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign48940_e73384;
        locals.var_t10_dn0 = assign48940_e73384_d_n0;
        locals.var_t10_dn2 = assign48940_e73384_d_n2;
        locals.var_t10_dn4 = assign48940_e73384_d_n4;
        locals.var_t10_dn5 = assign48940_e73384_d_n5;
        locals.var_t10_dn6 = assign48940_e73384_d_n6;
        locals.var_t10_dn7 = assign48940_e73384_d_n7;
        locals.var_t10_dn8 = assign48940_e73384_d_n8;
        locals.var_t10_dn9 = assign48940_e73384_d_n9;
        locals.var_t10_dn10 = assign48940_e73384_d_n10;
        locals.var_t10_dn13 = assign48940_e73384_d_n13;
        locals.var_t10_rv = 0.0;

        let (assign48950_e73410, assign48950_e73410_d_n0, assign48950_e73410_d_n2, assign48950_e73410_d_n4, assign48950_e73410_d_n5, assign48950_e73410_d_n6, assign48950_e73410_d_n7, assign48950_e73410_d_n8, assign48950_e73410_d_n9, assign48950_e73410_d_n10, assign48950_e73410_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) {
        let (assign48950_e73408, assign48950_e73408_d_n0, assign48950_e73408_d_n2, assign48950_e73408_d_n4, assign48950_e73408_d_n5, assign48950_e73408_d_n6, assign48950_e73408_d_n7, assign48950_e73408_d_n8, assign48950_e73408_d_n9, assign48950_e73408_d_n10, assign48950_e73408_d_n13,) = {
            if (locals.var_phi_sl_dep__blk1090 > locals.var_phi_bl_dep__blk1093) {
                let assign48950_e73400: f64 = (-locals.var_cnst0);
                let assign48950_e73402: f64 = (locals.var_t10).sqrt();
                let assign48950_e73403: f64 = (assign48950_e73400 * assign48950_e73402);
                (assign48950_e73403, (((-locals.var_cnst0_dn0) * assign48950_e73402) + (assign48950_e73400 * (locals.var_t10_dn0 / (2.0 * assign48950_e73402)))), (((-locals.var_cnst0_dn2) * assign48950_e73402) + (assign48950_e73400 * (locals.var_t10_dn2 / (2.0 * assign48950_e73402)))), (((-locals.var_cnst0_dn4) * assign48950_e73402) + (assign48950_e73400 * (locals.var_t10_dn4 / (2.0 * assign48950_e73402)))), (((-locals.var_cnst0_dn5) * assign48950_e73402) + (assign48950_e73400 * (locals.var_t10_dn5 / (2.0 * assign48950_e73402)))), (((-locals.var_cnst0_dn6) * assign48950_e73402) + (assign48950_e73400 * (locals.var_t10_dn6 / (2.0 * assign48950_e73402)))), (((-locals.var_cnst0_dn7) * assign48950_e73402) + (assign48950_e73400 * (locals.var_t10_dn7 / (2.0 * assign48950_e73402)))), (((-locals.var_cnst0_dn8) * assign48950_e73402) + (assign48950_e73400 * (locals.var_t10_dn8 / (2.0 * assign48950_e73402)))), (((-locals.var_cnst0_dn9) * assign48950_e73402) + (assign48950_e73400 * (locals.var_t10_dn9 / (2.0 * assign48950_e73402)))), (((-locals.var_cnst0_dn10) * assign48950_e73402) + (assign48950_e73400 * (locals.var_t10_dn10 / (2.0 * assign48950_e73402)))), (((-locals.var_cnst0_dn13) * assign48950_e73402) + (assign48950_e73400 * (locals.var_t10_dn13 / (2.0 * assign48950_e73402)))),)
            } else {
                let assign48950_e73406: f64 = (locals.var_t10).sqrt();
                let assign48950_e73407: f64 = (locals.var_cnst0 * assign48950_e73406);
                (assign48950_e73407, ((locals.var_cnst0_dn0 * assign48950_e73406) + (locals.var_cnst0 * (locals.var_t10_dn0 / (2.0 * assign48950_e73406)))), ((locals.var_cnst0_dn2 * assign48950_e73406) + (locals.var_cnst0 * (locals.var_t10_dn2 / (2.0 * assign48950_e73406)))), ((locals.var_cnst0_dn4 * assign48950_e73406) + (locals.var_cnst0 * (locals.var_t10_dn4 / (2.0 * assign48950_e73406)))), ((locals.var_cnst0_dn5 * assign48950_e73406) + (locals.var_cnst0 * (locals.var_t10_dn5 / (2.0 * assign48950_e73406)))), ((locals.var_cnst0_dn6 * assign48950_e73406) + (locals.var_cnst0 * (locals.var_t10_dn6 / (2.0 * assign48950_e73406)))), ((locals.var_cnst0_dn7 * assign48950_e73406) + (locals.var_cnst0 * (locals.var_t10_dn7 / (2.0 * assign48950_e73406)))), ((locals.var_cnst0_dn8 * assign48950_e73406) + (locals.var_cnst0 * (locals.var_t10_dn8 / (2.0 * assign48950_e73406)))), ((locals.var_cnst0_dn9 * assign48950_e73406) + (locals.var_cnst0 * (locals.var_t10_dn9 / (2.0 * assign48950_e73406)))), ((locals.var_cnst0_dn10 * assign48950_e73406) + (locals.var_cnst0 * (locals.var_t10_dn10 / (2.0 * assign48950_e73406)))), ((locals.var_cnst0_dn13 * assign48950_e73406) + (locals.var_cnst0 * (locals.var_t10_dn13 / (2.0 * assign48950_e73406)))),)
            }
        };
        (assign48950_e73408, assign48950_e73408_d_n0, assign48950_e73408_d_n2, assign48950_e73408_d_n4, assign48950_e73408_d_n5, assign48950_e73408_d_n6, assign48950_e73408_d_n7, assign48950_e73408_d_n8, assign48950_e73408_d_n9, assign48950_e73408_d_n10, assign48950_e73408_d_n13,)
    } else {
        (locals.var_q_nl__blk1123, locals.var_q_nl__blk1123_dn0, locals.var_q_nl__blk1123_dn2, locals.var_q_nl__blk1123_dn4, locals.var_q_nl__blk1123_dn5, locals.var_q_nl__blk1123_dn6, locals.var_q_nl__blk1123_dn7, locals.var_q_nl__blk1123_dn8, locals.var_q_nl__blk1123_dn9, locals.var_q_nl__blk1123_dn10, locals.var_q_nl__blk1123_dn13,)
    }
};
        locals.var_q_nl__blk1123 = assign48950_e73410;
        locals.var_q_nl__blk1123_dn0 = assign48950_e73410_d_n0;
        locals.var_q_nl__blk1123_dn2 = assign48950_e73410_d_n2;
        locals.var_q_nl__blk1123_dn4 = assign48950_e73410_d_n4;
        locals.var_q_nl__blk1123_dn5 = assign48950_e73410_d_n5;
        locals.var_q_nl__blk1123_dn6 = assign48950_e73410_d_n6;
        locals.var_q_nl__blk1123_dn7 = assign48950_e73410_d_n7;
        locals.var_q_nl__blk1123_dn8 = assign48950_e73410_d_n8;
        locals.var_q_nl__blk1123_dn9 = assign48950_e73410_d_n9;
        locals.var_q_nl__blk1123_dn10 = assign48950_e73410_d_n10;
        locals.var_q_nl__blk1123_dn13 = assign48950_e73410_d_n13;
        locals.var_q_nl__blk1123_rv = 0.0;

        let assign48960_e73413: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1277 = assign48960_e73413;
        locals.var_guard1277_rv = 0.0;

        let assign48970_e73416: f64 = (locals.var_phi_sl_dep__blk1090 - locals.var_vds_maxbl__blk1088);
        let assign48970_e73419: f64 = p.p403;
        let assign48970_e73424: f64 = if ((assign48970_e73416 < assign48970_e73419) && (p.p403 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1278 = assign48970_e73424;
        locals.var_guard1278_rv = 0.0;

        let (assign48980_e73448, assign48980_e73448_d_n0, assign48980_e73448_d_n2, assign48980_e73448_d_n4, assign48980_e73448_d_n5, assign48980_e73448_d_n6, assign48980_e73448_d_n7, assign48980_e73448_d_n8, assign48980_e73448_d_n9, assign48980_e73448_d_n10, assign48980_e73448_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) && (locals.var_guard1278 != 0.0)) {
        let assign48980_e73442: f64 = p.p403;
        let assign48980_e73445: f64 = (locals.var_phi_sl_dep__blk1090 - locals.var_vds_maxbl__blk1088);
        let assign48980_e73446: f64 = (assign48980_e73442 - assign48980_e73445);
        (assign48980_e73446, (-(locals.var_phi_sl_dep__blk1090_dn0 - locals.var_vds_maxbl__blk1088_dn0)), (-(locals.var_phi_sl_dep__blk1090_dn2 - locals.var_vds_maxbl__blk1088_dn2)), (-(locals.var_phi_sl_dep__blk1090_dn4 - locals.var_vds_maxbl__blk1088_dn4)), (-(locals.var_phi_sl_dep__blk1090_dn5 - locals.var_vds_maxbl__blk1088_dn5)), (-(locals.var_phi_sl_dep__blk1090_dn6 - locals.var_vds_maxbl__blk1088_dn6)), (-(locals.var_phi_sl_dep__blk1090_dn7 - locals.var_vds_maxbl__blk1088_dn7)), (-(locals.var_phi_sl_dep__blk1090_dn8 - locals.var_vds_maxbl__blk1088_dn8)), (-(locals.var_phi_sl_dep__blk1090_dn9 - locals.var_vds_maxbl__blk1088_dn9)), (-(locals.var_phi_sl_dep__blk1090_dn10 - locals.var_vds_maxbl__blk1088_dn10)), (-(locals.var_phi_sl_dep__blk1090_dn13 - locals.var_vds_maxbl__blk1088_dn13)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign48980_e73448;
        locals.var_tmf1_dn0 = assign48980_e73448_d_n0;
        locals.var_tmf1_dn2 = assign48980_e73448_d_n2;
        locals.var_tmf1_dn4 = assign48980_e73448_d_n4;
        locals.var_tmf1_dn5 = assign48980_e73448_d_n5;
        locals.var_tmf1_dn6 = assign48980_e73448_d_n6;
        locals.var_tmf1_dn7 = assign48980_e73448_d_n7;
        locals.var_tmf1_dn8 = assign48980_e73448_d_n8;
        locals.var_tmf1_dn9 = assign48980_e73448_d_n9;
        locals.var_tmf1_dn10 = assign48980_e73448_d_n10;
        locals.var_tmf1_dn13 = assign48980_e73448_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign48990_e73468, assign48990_e73468_d_n0, assign48990_e73468_d_n2, assign48990_e73468_d_n4, assign48990_e73468_d_n5, assign48990_e73468_d_n6, assign48990_e73468_d_n7, assign48990_e73468_d_n8, assign48990_e73468_d_n9, assign48990_e73468_d_n10, assign48990_e73468_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) && (locals.var_guard1278 != 0.0)) {
        let assign48990_e73466: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign48990_e73466, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign48990_e73468;
        locals.var_x2_dn0 = assign48990_e73468_d_n0;
        locals.var_x2_dn2 = assign48990_e73468_d_n2;
        locals.var_x2_dn4 = assign48990_e73468_d_n4;
        locals.var_x2_dn5 = assign48990_e73468_d_n5;
        locals.var_x2_dn6 = assign48990_e73468_d_n6;
        locals.var_x2_dn7 = assign48990_e73468_d_n7;
        locals.var_x2_dn8 = assign48990_e73468_d_n8;
        locals.var_x2_dn9 = assign48990_e73468_d_n9;
        locals.var_x2_dn10 = assign48990_e73468_d_n10;
        locals.var_x2_dn13 = assign48990_e73468_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign49000_e73488, assign49000_e73488_d_n0, assign49000_e73488_d_n2, assign49000_e73488_d_n4, assign49000_e73488_d_n5, assign49000_e73488_d_n6, assign49000_e73488_d_n7, assign49000_e73488_d_n8, assign49000_e73488_d_n9, assign49000_e73488_d_n10, assign49000_e73488_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) && (locals.var_guard1278 != 0.0)) {
        let assign49000_e73486: f64 = (p.p403 * p.p403);
        (assign49000_e73486, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign49000_e73488;
        locals.var_xmax2_dn0 = assign49000_e73488_d_n0;
        locals.var_xmax2_dn2 = assign49000_e73488_d_n2;
        locals.var_xmax2_dn4 = assign49000_e73488_d_n4;
        locals.var_xmax2_dn5 = assign49000_e73488_d_n5;
        locals.var_xmax2_dn6 = assign49000_e73488_d_n6;
        locals.var_xmax2_dn7 = assign49000_e73488_d_n7;
        locals.var_xmax2_dn8 = assign49000_e73488_d_n8;
        locals.var_xmax2_dn9 = assign49000_e73488_d_n9;
        locals.var_xmax2_dn10 = assign49000_e73488_d_n10;
        locals.var_xmax2_dn13 = assign49000_e73488_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign49010_e73506, assign49010_e73506_d_n0, assign49010_e73506_d_n2, assign49010_e73506_d_n4, assign49010_e73506_d_n5, assign49010_e73506_d_n6, assign49010_e73506_d_n7, assign49010_e73506_d_n8, assign49010_e73506_d_n9, assign49010_e73506_d_n10, assign49010_e73506_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) && (locals.var_guard1278 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign49010_e73506;
        locals.var_xp_dn0 = assign49010_e73506_d_n0;
        locals.var_xp_dn2 = assign49010_e73506_d_n2;
        locals.var_xp_dn4 = assign49010_e73506_d_n4;
        locals.var_xp_dn5 = assign49010_e73506_d_n5;
        locals.var_xp_dn6 = assign49010_e73506_d_n6;
        locals.var_xp_dn7 = assign49010_e73506_d_n7;
        locals.var_xp_dn8 = assign49010_e73506_d_n8;
        locals.var_xp_dn9 = assign49010_e73506_d_n9;
        locals.var_xp_dn10 = assign49010_e73506_d_n10;
        locals.var_xp_dn13 = assign49010_e73506_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign49020_e73524, assign49020_e73524_d_n0, assign49020_e73524_d_n2, assign49020_e73524_d_n4, assign49020_e73524_d_n5, assign49020_e73524_d_n6, assign49020_e73524_d_n7, assign49020_e73524_d_n8, assign49020_e73524_d_n9, assign49020_e73524_d_n10, assign49020_e73524_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) && (locals.var_guard1278 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign49020_e73524;
        locals.var_xmp_dn0 = assign49020_e73524_d_n0;
        locals.var_xmp_dn2 = assign49020_e73524_d_n2;
        locals.var_xmp_dn4 = assign49020_e73524_d_n4;
        locals.var_xmp_dn5 = assign49020_e73524_d_n5;
        locals.var_xmp_dn6 = assign49020_e73524_d_n6;
        locals.var_xmp_dn7 = assign49020_e73524_d_n7;
        locals.var_xmp_dn8 = assign49020_e73524_d_n8;
        locals.var_xmp_dn9 = assign49020_e73524_d_n9;
        locals.var_xmp_dn10 = assign49020_e73524_d_n10;
        locals.var_xmp_dn13 = assign49020_e73524_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign49030_e73542,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) && (locals.var_guard1278 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign49030_e73542;
        locals.var_m0_rv = 0.0;

        let (assign49040_e73560,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) && (locals.var_guard1278 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign49040_e73560;
        locals.var_mm_rv = 0.0;

        let (assign49050_e73578, assign49050_e73578_d_n0, assign49050_e73578_d_n2, assign49050_e73578_d_n4, assign49050_e73578_d_n5, assign49050_e73578_d_n6, assign49050_e73578_d_n7, assign49050_e73578_d_n8, assign49050_e73578_d_n9, assign49050_e73578_d_n10, assign49050_e73578_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) && (locals.var_guard1278 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign49050_e73578;
        locals.var_arg_dn0 = assign49050_e73578_d_n0;
        locals.var_arg_dn2 = assign49050_e73578_d_n2;
        locals.var_arg_dn4 = assign49050_e73578_d_n4;
        locals.var_arg_dn5 = assign49050_e73578_d_n5;
        locals.var_arg_dn6 = assign49050_e73578_d_n6;
        locals.var_arg_dn7 = assign49050_e73578_d_n7;
        locals.var_arg_dn8 = assign49050_e73578_d_n8;
        locals.var_arg_dn9 = assign49050_e73578_d_n9;
        locals.var_arg_dn10 = assign49050_e73578_d_n10;
        locals.var_arg_dn13 = assign49050_e73578_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign49060_e73596, assign49060_e73596_d_n0, assign49060_e73596_d_n2, assign49060_e73596_d_n4, assign49060_e73596_d_n5, assign49060_e73596_d_n6, assign49060_e73596_d_n7, assign49060_e73596_d_n8, assign49060_e73596_d_n9, assign49060_e73596_d_n10, assign49060_e73596_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) && (locals.var_guard1278 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign49060_e73596;
        locals.var_dnm_dn0 = assign49060_e73596_d_n0;
        locals.var_dnm_dn2 = assign49060_e73596_d_n2;
        locals.var_dnm_dn4 = assign49060_e73596_d_n4;
        locals.var_dnm_dn5 = assign49060_e73596_d_n5;
        locals.var_dnm_dn6 = assign49060_e73596_d_n6;
        locals.var_dnm_dn7 = assign49060_e73596_d_n7;
        locals.var_dnm_dn8 = assign49060_e73596_d_n8;
        locals.var_dnm_dn9 = assign49060_e73596_d_n9;
        locals.var_dnm_dn10 = assign49060_e73596_d_n10;
        locals.var_dnm_dn13 = assign49060_e73596_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign49070_e73616, assign49070_e73616_d_n0, assign49070_e73616_d_n2, assign49070_e73616_d_n4, assign49070_e73616_d_n5, assign49070_e73616_d_n6, assign49070_e73616_d_n7, assign49070_e73616_d_n8, assign49070_e73616_d_n9, assign49070_e73616_d_n10, assign49070_e73616_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) && (locals.var_guard1278 != 0.0)) {
        let assign49070_e73614: f64 = (locals.var_xp * locals.var_x2);
        (assign49070_e73614, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign49070_e73616;
        locals.var_xp_dn0 = assign49070_e73616_d_n0;
        locals.var_xp_dn2 = assign49070_e73616_d_n2;
        locals.var_xp_dn4 = assign49070_e73616_d_n4;
        locals.var_xp_dn5 = assign49070_e73616_d_n5;
        locals.var_xp_dn6 = assign49070_e73616_d_n6;
        locals.var_xp_dn7 = assign49070_e73616_d_n7;
        locals.var_xp_dn8 = assign49070_e73616_d_n8;
        locals.var_xp_dn9 = assign49070_e73616_d_n9;
        locals.var_xp_dn10 = assign49070_e73616_d_n10;
        locals.var_xp_dn13 = assign49070_e73616_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign49080_e73636, assign49080_e73636_d_n0, assign49080_e73636_d_n2, assign49080_e73636_d_n4, assign49080_e73636_d_n5, assign49080_e73636_d_n6, assign49080_e73636_d_n7, assign49080_e73636_d_n8, assign49080_e73636_d_n9, assign49080_e73636_d_n10, assign49080_e73636_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) && (locals.var_guard1278 != 0.0)) {
        let assign49080_e73634: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign49080_e73634, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign49080_e73636;
        locals.var_xmp_dn0 = assign49080_e73636_d_n0;
        locals.var_xmp_dn2 = assign49080_e73636_d_n2;
        locals.var_xmp_dn4 = assign49080_e73636_d_n4;
        locals.var_xmp_dn5 = assign49080_e73636_d_n5;
        locals.var_xmp_dn6 = assign49080_e73636_d_n6;
        locals.var_xmp_dn7 = assign49080_e73636_d_n7;
        locals.var_xmp_dn8 = assign49080_e73636_d_n8;
        locals.var_xmp_dn9 = assign49080_e73636_d_n9;
        locals.var_xmp_dn10 = assign49080_e73636_d_n10;
        locals.var_xmp_dn13 = assign49080_e73636_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign49090_e73656, assign49090_e73656_d_n0, assign49090_e73656_d_n2, assign49090_e73656_d_n4, assign49090_e73656_d_n5, assign49090_e73656_d_n6, assign49090_e73656_d_n7, assign49090_e73656_d_n8, assign49090_e73656_d_n9, assign49090_e73656_d_n10, assign49090_e73656_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) && (locals.var_guard1278 != 0.0)) {
        let assign49090_e73654: f64 = (locals.var_xp * locals.var_x2);
        (assign49090_e73654, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign49090_e73656;
        locals.var_xp_dn0 = assign49090_e73656_d_n0;
        locals.var_xp_dn2 = assign49090_e73656_d_n2;
        locals.var_xp_dn4 = assign49090_e73656_d_n4;
        locals.var_xp_dn5 = assign49090_e73656_d_n5;
        locals.var_xp_dn6 = assign49090_e73656_d_n6;
        locals.var_xp_dn7 = assign49090_e73656_d_n7;
        locals.var_xp_dn8 = assign49090_e73656_d_n8;
        locals.var_xp_dn9 = assign49090_e73656_d_n9;
        locals.var_xp_dn10 = assign49090_e73656_d_n10;
        locals.var_xp_dn13 = assign49090_e73656_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign49100_e73676, assign49100_e73676_d_n0, assign49100_e73676_d_n2, assign49100_e73676_d_n4, assign49100_e73676_d_n5, assign49100_e73676_d_n6, assign49100_e73676_d_n7, assign49100_e73676_d_n8, assign49100_e73676_d_n9, assign49100_e73676_d_n10, assign49100_e73676_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) && (locals.var_guard1278 != 0.0)) {
        let assign49100_e73674: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign49100_e73674, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign49100_e73676;
        locals.var_xmp_dn0 = assign49100_e73676_d_n0;
        locals.var_xmp_dn2 = assign49100_e73676_d_n2;
        locals.var_xmp_dn4 = assign49100_e73676_d_n4;
        locals.var_xmp_dn5 = assign49100_e73676_d_n5;
        locals.var_xmp_dn6 = assign49100_e73676_d_n6;
        locals.var_xmp_dn7 = assign49100_e73676_d_n7;
        locals.var_xmp_dn8 = assign49100_e73676_d_n8;
        locals.var_xmp_dn9 = assign49100_e73676_d_n9;
        locals.var_xmp_dn10 = assign49100_e73676_d_n10;
        locals.var_xmp_dn13 = assign49100_e73676_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign49110_e73696, assign49110_e73696_d_n0, assign49110_e73696_d_n2, assign49110_e73696_d_n4, assign49110_e73696_d_n5, assign49110_e73696_d_n6, assign49110_e73696_d_n7, assign49110_e73696_d_n8, assign49110_e73696_d_n9, assign49110_e73696_d_n10, assign49110_e73696_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) && (locals.var_guard1278 != 0.0)) {
        let assign49110_e73694: f64 = (locals.var_xp * locals.var_x2);
        (assign49110_e73694, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign49110_e73696;
        locals.var_xp_dn0 = assign49110_e73696_d_n0;
        locals.var_xp_dn2 = assign49110_e73696_d_n2;
        locals.var_xp_dn4 = assign49110_e73696_d_n4;
        locals.var_xp_dn5 = assign49110_e73696_d_n5;
        locals.var_xp_dn6 = assign49110_e73696_d_n6;
        locals.var_xp_dn7 = assign49110_e73696_d_n7;
        locals.var_xp_dn8 = assign49110_e73696_d_n8;
        locals.var_xp_dn9 = assign49110_e73696_d_n9;
        locals.var_xp_dn10 = assign49110_e73696_d_n10;
        locals.var_xp_dn13 = assign49110_e73696_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign49120_e73716, assign49120_e73716_d_n0, assign49120_e73716_d_n2, assign49120_e73716_d_n4, assign49120_e73716_d_n5, assign49120_e73716_d_n6, assign49120_e73716_d_n7, assign49120_e73716_d_n8, assign49120_e73716_d_n9, assign49120_e73716_d_n10, assign49120_e73716_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) && (locals.var_guard1278 != 0.0)) {
        let assign49120_e73714: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign49120_e73714, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign49120_e73716;
        locals.var_xmp_dn0 = assign49120_e73716_d_n0;
        locals.var_xmp_dn2 = assign49120_e73716_d_n2;
        locals.var_xmp_dn4 = assign49120_e73716_d_n4;
        locals.var_xmp_dn5 = assign49120_e73716_d_n5;
        locals.var_xmp_dn6 = assign49120_e73716_d_n6;
        locals.var_xmp_dn7 = assign49120_e73716_d_n7;
        locals.var_xmp_dn8 = assign49120_e73716_d_n8;
        locals.var_xmp_dn9 = assign49120_e73716_d_n9;
        locals.var_xmp_dn10 = assign49120_e73716_d_n10;
        locals.var_xmp_dn13 = assign49120_e73716_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign49130_e73736, assign49130_e73736_d_n0, assign49130_e73736_d_n2, assign49130_e73736_d_n4, assign49130_e73736_d_n5, assign49130_e73736_d_n6, assign49130_e73736_d_n7, assign49130_e73736_d_n8, assign49130_e73736_d_n9, assign49130_e73736_d_n10, assign49130_e73736_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) && (locals.var_guard1278 != 0.0)) {
        let assign49130_e73734: f64 = (locals.var_xp * locals.var_x2);
        (assign49130_e73734, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign49130_e73736;
        locals.var_xp_dn0 = assign49130_e73736_d_n0;
        locals.var_xp_dn2 = assign49130_e73736_d_n2;
        locals.var_xp_dn4 = assign49130_e73736_d_n4;
        locals.var_xp_dn5 = assign49130_e73736_d_n5;
        locals.var_xp_dn6 = assign49130_e73736_d_n6;
        locals.var_xp_dn7 = assign49130_e73736_d_n7;
        locals.var_xp_dn8 = assign49130_e73736_d_n8;
        locals.var_xp_dn9 = assign49130_e73736_d_n9;
        locals.var_xp_dn10 = assign49130_e73736_d_n10;
        locals.var_xp_dn13 = assign49130_e73736_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign49140_e73756, assign49140_e73756_d_n0, assign49140_e73756_d_n2, assign49140_e73756_d_n4, assign49140_e73756_d_n5, assign49140_e73756_d_n6, assign49140_e73756_d_n7, assign49140_e73756_d_n8, assign49140_e73756_d_n9, assign49140_e73756_d_n10, assign49140_e73756_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) && (locals.var_guard1278 != 0.0)) {
        let assign49140_e73754: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign49140_e73754, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign49140_e73756;
        locals.var_xmp_dn0 = assign49140_e73756_d_n0;
        locals.var_xmp_dn2 = assign49140_e73756_d_n2;
        locals.var_xmp_dn4 = assign49140_e73756_d_n4;
        locals.var_xmp_dn5 = assign49140_e73756_d_n5;
        locals.var_xmp_dn6 = assign49140_e73756_d_n6;
        locals.var_xmp_dn7 = assign49140_e73756_d_n7;
        locals.var_xmp_dn8 = assign49140_e73756_d_n8;
        locals.var_xmp_dn9 = assign49140_e73756_d_n9;
        locals.var_xmp_dn10 = assign49140_e73756_d_n10;
        locals.var_xmp_dn13 = assign49140_e73756_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign49150_e73776, assign49150_e73776_d_n0, assign49150_e73776_d_n2, assign49150_e73776_d_n4, assign49150_e73776_d_n5, assign49150_e73776_d_n6, assign49150_e73776_d_n7, assign49150_e73776_d_n8, assign49150_e73776_d_n9, assign49150_e73776_d_n10, assign49150_e73776_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) && (locals.var_guard1278 != 0.0)) {
        let assign49150_e73774: f64 = (locals.var_xp * locals.var_x2);
        (assign49150_e73774, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign49150_e73776;
        locals.var_xp_dn0 = assign49150_e73776_d_n0;
        locals.var_xp_dn2 = assign49150_e73776_d_n2;
        locals.var_xp_dn4 = assign49150_e73776_d_n4;
        locals.var_xp_dn5 = assign49150_e73776_d_n5;
        locals.var_xp_dn6 = assign49150_e73776_d_n6;
        locals.var_xp_dn7 = assign49150_e73776_d_n7;
        locals.var_xp_dn8 = assign49150_e73776_d_n8;
        locals.var_xp_dn9 = assign49150_e73776_d_n9;
        locals.var_xp_dn10 = assign49150_e73776_d_n10;
        locals.var_xp_dn13 = assign49150_e73776_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign49160_e73796, assign49160_e73796_d_n0, assign49160_e73796_d_n2, assign49160_e73796_d_n4, assign49160_e73796_d_n5, assign49160_e73796_d_n6, assign49160_e73796_d_n7, assign49160_e73796_d_n8, assign49160_e73796_d_n9, assign49160_e73796_d_n10, assign49160_e73796_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) && (locals.var_guard1278 != 0.0)) {
        let assign49160_e73794: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign49160_e73794, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign49160_e73796;
        locals.var_xmp_dn0 = assign49160_e73796_d_n0;
        locals.var_xmp_dn2 = assign49160_e73796_d_n2;
        locals.var_xmp_dn4 = assign49160_e73796_d_n4;
        locals.var_xmp_dn5 = assign49160_e73796_d_n5;
        locals.var_xmp_dn6 = assign49160_e73796_d_n6;
        locals.var_xmp_dn7 = assign49160_e73796_d_n7;
        locals.var_xmp_dn8 = assign49160_e73796_d_n8;
        locals.var_xmp_dn9 = assign49160_e73796_d_n9;
        locals.var_xmp_dn10 = assign49160_e73796_d_n10;
        locals.var_xmp_dn13 = assign49160_e73796_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign49170_e73816, assign49170_e73816_d_n0, assign49170_e73816_d_n2, assign49170_e73816_d_n4, assign49170_e73816_d_n5, assign49170_e73816_d_n6, assign49170_e73816_d_n7, assign49170_e73816_d_n8, assign49170_e73816_d_n9, assign49170_e73816_d_n10, assign49170_e73816_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) && (locals.var_guard1278 != 0.0)) {
        let assign49170_e73814: f64 = (locals.var_xp * locals.var_x2);
        (assign49170_e73814, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign49170_e73816;
        locals.var_xp_dn0 = assign49170_e73816_d_n0;
        locals.var_xp_dn2 = assign49170_e73816_d_n2;
        locals.var_xp_dn4 = assign49170_e73816_d_n4;
        locals.var_xp_dn5 = assign49170_e73816_d_n5;
        locals.var_xp_dn6 = assign49170_e73816_d_n6;
        locals.var_xp_dn7 = assign49170_e73816_d_n7;
        locals.var_xp_dn8 = assign49170_e73816_d_n8;
        locals.var_xp_dn9 = assign49170_e73816_d_n9;
        locals.var_xp_dn10 = assign49170_e73816_d_n10;
        locals.var_xp_dn13 = assign49170_e73816_d_n13;
        locals.var_xp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_170(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign49180_e73836, assign49180_e73836_d_n0, assign49180_e73836_d_n2, assign49180_e73836_d_n4, assign49180_e73836_d_n5, assign49180_e73836_d_n6, assign49180_e73836_d_n7, assign49180_e73836_d_n8, assign49180_e73836_d_n9, assign49180_e73836_d_n10, assign49180_e73836_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) && (locals.var_guard1278 != 0.0)) {
        let assign49180_e73834: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign49180_e73834, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign49180_e73836;
        locals.var_xmp_dn0 = assign49180_e73836_d_n0;
        locals.var_xmp_dn2 = assign49180_e73836_d_n2;
        locals.var_xmp_dn4 = assign49180_e73836_d_n4;
        locals.var_xmp_dn5 = assign49180_e73836_d_n5;
        locals.var_xmp_dn6 = assign49180_e73836_d_n6;
        locals.var_xmp_dn7 = assign49180_e73836_d_n7;
        locals.var_xmp_dn8 = assign49180_e73836_d_n8;
        locals.var_xmp_dn9 = assign49180_e73836_d_n9;
        locals.var_xmp_dn10 = assign49180_e73836_d_n10;
        locals.var_xmp_dn13 = assign49180_e73836_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign49190_e73856, assign49190_e73856_d_n0, assign49190_e73856_d_n2, assign49190_e73856_d_n4, assign49190_e73856_d_n5, assign49190_e73856_d_n6, assign49190_e73856_d_n7, assign49190_e73856_d_n8, assign49190_e73856_d_n9, assign49190_e73856_d_n10, assign49190_e73856_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) && (locals.var_guard1278 != 0.0)) {
        let assign49190_e73854: f64 = (locals.var_xp + locals.var_xmp);
        (assign49190_e73854, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign49190_e73856;
        locals.var_arg_dn0 = assign49190_e73856_d_n0;
        locals.var_arg_dn2 = assign49190_e73856_d_n2;
        locals.var_arg_dn4 = assign49190_e73856_d_n4;
        locals.var_arg_dn5 = assign49190_e73856_d_n5;
        locals.var_arg_dn6 = assign49190_e73856_d_n6;
        locals.var_arg_dn7 = assign49190_e73856_d_n7;
        locals.var_arg_dn8 = assign49190_e73856_d_n8;
        locals.var_arg_dn9 = assign49190_e73856_d_n9;
        locals.var_arg_dn10 = assign49190_e73856_d_n10;
        locals.var_arg_dn13 = assign49190_e73856_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign49200_e73874, assign49200_e73874_d_n0, assign49200_e73874_d_n2, assign49200_e73874_d_n4, assign49200_e73874_d_n5, assign49200_e73874_d_n6, assign49200_e73874_d_n7, assign49200_e73874_d_n8, assign49200_e73874_d_n9, assign49200_e73874_d_n10, assign49200_e73874_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) && (locals.var_guard1278 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign49200_e73874;
        locals.var_dnm_dn0 = assign49200_e73874_d_n0;
        locals.var_dnm_dn2 = assign49200_e73874_d_n2;
        locals.var_dnm_dn4 = assign49200_e73874_d_n4;
        locals.var_dnm_dn5 = assign49200_e73874_d_n5;
        locals.var_dnm_dn6 = assign49200_e73874_d_n6;
        locals.var_dnm_dn7 = assign49200_e73874_d_n7;
        locals.var_dnm_dn8 = assign49200_e73874_d_n8;
        locals.var_dnm_dn9 = assign49200_e73874_d_n9;
        locals.var_dnm_dn10 = assign49200_e73874_d_n10;
        locals.var_dnm_dn13 = assign49200_e73874_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign49210_e73889: f64 = if ((((6.0 == 1.0) || (6.0 == 2.0)) || (6.0 == 4.0)) || (6.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1279 = assign49210_e73889;
        locals.var_guard1279_rv = 0.0;

        let assign49220_e73892: f64 = if 6.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1280 = assign49220_e73892;
        locals.var_guard1280_rv = 0.0;

        let (assign49230_e73914,) = {
    if (((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) && (locals.var_guard1278 != 0.0)) && (locals.var_guard1279 != 0.0)) && (locals.var_guard1280 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign49230_e73914;
        locals.var_mm_rv = 0.0;

        let assign49240_e73917: f64 = if 6.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1281 = assign49240_e73917;
        locals.var_guard1281_rv = 0.0;

        let (assign49250_e73942,) = {
    if ((((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) && (locals.var_guard1278 != 0.0)) && (locals.var_guard1279 != 0.0)) && (locals.var_guard1280 == 0.0)) && (locals.var_guard1281 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign49250_e73942;
        locals.var_mm_rv = 0.0;

        let assign49260_e73945: f64 = if 6.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1282 = assign49260_e73945;
        locals.var_guard1282_rv = 0.0;

        let (assign49270_e73973,) = {
    if (((((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) && (locals.var_guard1278 != 0.0)) && (locals.var_guard1279 != 0.0)) && (locals.var_guard1280 == 0.0)) && (locals.var_guard1281 == 0.0)) && (locals.var_guard1282 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign49270_e73973;
        locals.var_mm_rv = 0.0;

        let assign49280_e73976: f64 = if 6.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1283 = assign49280_e73976;
        locals.var_guard1283_rv = 0.0;

        let (assign49290_e74007,) = {
    if ((((((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) && (locals.var_guard1278 != 0.0)) && (locals.var_guard1279 != 0.0)) && (locals.var_guard1280 == 0.0)) && (locals.var_guard1281 == 0.0)) && (locals.var_guard1282 == 0.0)) && (locals.var_guard1283 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign49290_e74007;
        locals.var_mm_rv = 0.0;

        let (assign49300_e74027,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) && (locals.var_guard1278 != 0.0)) && (locals.var_guard1279 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign49300_e74027;
        locals.var_m0_rv = 0.0;

        let mut assign49310_loop_guard: usize = 0;
        while {
            let assign49310_cond_e74048: f64 = if (((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) && (locals.var_guard1278 != 0.0)) && (locals.var_guard1279 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign49310_cond_e74048 != 0.0
        } {
            assign49310_loop_guard += 1;
            assert!(assign49310_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign49310_body0_e74069, assign49310_body0_e74069_d_n0, assign49310_body0_e74069_d_n2, assign49310_body0_e74069_d_n4, assign49310_body0_e74069_d_n5, assign49310_body0_e74069_d_n6, assign49310_body0_e74069_d_n7, assign49310_body0_e74069_d_n8, assign49310_body0_e74069_d_n9, assign49310_body0_e74069_d_n10, assign49310_body0_e74069_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) && (locals.var_guard1278 != 0.0)) && (locals.var_guard1279 != 0.0)) {
        let assign49310_body0_e74067: f64 = (locals.var_dnm).sqrt();
        (assign49310_body0_e74067, (locals.var_dnm_dn0 / (2.0 * assign49310_body0_e74067)), (locals.var_dnm_dn2 / (2.0 * assign49310_body0_e74067)), (locals.var_dnm_dn4 / (2.0 * assign49310_body0_e74067)), (locals.var_dnm_dn5 / (2.0 * assign49310_body0_e74067)), (locals.var_dnm_dn6 / (2.0 * assign49310_body0_e74067)), (locals.var_dnm_dn7 / (2.0 * assign49310_body0_e74067)), (locals.var_dnm_dn8 / (2.0 * assign49310_body0_e74067)), (locals.var_dnm_dn9 / (2.0 * assign49310_body0_e74067)), (locals.var_dnm_dn10 / (2.0 * assign49310_body0_e74067)), (locals.var_dnm_dn13 / (2.0 * assign49310_body0_e74067)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign49310_body0_e74069;
            locals.var_dnm_dn0 = assign49310_body0_e74069_d_n0;
            locals.var_dnm_dn2 = assign49310_body0_e74069_d_n2;
            locals.var_dnm_dn4 = assign49310_body0_e74069_d_n4;
            locals.var_dnm_dn5 = assign49310_body0_e74069_d_n5;
            locals.var_dnm_dn6 = assign49310_body0_e74069_d_n6;
            locals.var_dnm_dn7 = assign49310_body0_e74069_d_n7;
            locals.var_dnm_dn8 = assign49310_body0_e74069_d_n8;
            locals.var_dnm_dn9 = assign49310_body0_e74069_d_n9;
            locals.var_dnm_dn10 = assign49310_body0_e74069_d_n10;
            locals.var_dnm_dn13 = assign49310_body0_e74069_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign49310_body1_e74091,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) && (locals.var_guard1278 != 0.0)) && (locals.var_guard1279 != 0.0)) {
        let assign49310_body1_e74089: f64 = (locals.var_m0 + 1.0);
        (assign49310_body1_e74089,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign49310_body1_e74091;
            locals.var_m0_rv = 0.0;
        }

        let (assign49320_e74123, assign49320_e74123_d_n0, assign49320_e74123_d_n2, assign49320_e74123_d_n4, assign49320_e74123_d_n5, assign49320_e74123_d_n6, assign49320_e74123_d_n7, assign49320_e74123_d_n8, assign49320_e74123_d_n9, assign49320_e74123_d_n10, assign49320_e74123_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) && (locals.var_guard1278 != 0.0)) && (locals.var_guard1279 == 0.0)) {
        let (assign49320_e74121, assign49320_e74121_d_n0, assign49320_e74121_d_n2, assign49320_e74121_d_n4, assign49320_e74121_d_n5, assign49320_e74121_d_n6, assign49320_e74121_d_n7, assign49320_e74121_d_n8, assign49320_e74121_d_n9, assign49320_e74121_d_n10, assign49320_e74121_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign49320_e74118: f64 = (2.0 * 6.0);
                let assign49320_e74119: f64 = (1.0 / assign49320_e74118);
                let assign49320_e74120: f64 = (locals.var_dnm).powf(assign49320_e74119);
                (assign49320_e74120, if 0.0 == 0.0 && ((assign49320_e74119) as f64).is_finite() && ((assign49320_e74119) as f64).fract() == 0.0 { if assign49320_e74119 == 0.0 { 0.0 } else { (assign49320_e74119 * ((locals.var_dnm).powf(assign49320_e74119 - 1.0) * locals.var_dnm_dn0)) } } else { (assign49320_e74120 * (assign49320_e74119 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign49320_e74119) as f64).is_finite() && ((assign49320_e74119) as f64).fract() == 0.0 { if assign49320_e74119 == 0.0 { 0.0 } else { (assign49320_e74119 * ((locals.var_dnm).powf(assign49320_e74119 - 1.0) * locals.var_dnm_dn2)) } } else { (assign49320_e74120 * (assign49320_e74119 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign49320_e74119) as f64).is_finite() && ((assign49320_e74119) as f64).fract() == 0.0 { if assign49320_e74119 == 0.0 { 0.0 } else { (assign49320_e74119 * ((locals.var_dnm).powf(assign49320_e74119 - 1.0) * locals.var_dnm_dn4)) } } else { (assign49320_e74120 * (assign49320_e74119 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign49320_e74119) as f64).is_finite() && ((assign49320_e74119) as f64).fract() == 0.0 { if assign49320_e74119 == 0.0 { 0.0 } else { (assign49320_e74119 * ((locals.var_dnm).powf(assign49320_e74119 - 1.0) * locals.var_dnm_dn5)) } } else { (assign49320_e74120 * (assign49320_e74119 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign49320_e74119) as f64).is_finite() && ((assign49320_e74119) as f64).fract() == 0.0 { if assign49320_e74119 == 0.0 { 0.0 } else { (assign49320_e74119 * ((locals.var_dnm).powf(assign49320_e74119 - 1.0) * locals.var_dnm_dn6)) } } else { (assign49320_e74120 * (assign49320_e74119 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign49320_e74119) as f64).is_finite() && ((assign49320_e74119) as f64).fract() == 0.0 { if assign49320_e74119 == 0.0 { 0.0 } else { (assign49320_e74119 * ((locals.var_dnm).powf(assign49320_e74119 - 1.0) * locals.var_dnm_dn7)) } } else { (assign49320_e74120 * (assign49320_e74119 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign49320_e74119) as f64).is_finite() && ((assign49320_e74119) as f64).fract() == 0.0 { if assign49320_e74119 == 0.0 { 0.0 } else { (assign49320_e74119 * ((locals.var_dnm).powf(assign49320_e74119 - 1.0) * locals.var_dnm_dn8)) } } else { (assign49320_e74120 * (assign49320_e74119 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign49320_e74119) as f64).is_finite() && ((assign49320_e74119) as f64).fract() == 0.0 { if assign49320_e74119 == 0.0 { 0.0 } else { (assign49320_e74119 * ((locals.var_dnm).powf(assign49320_e74119 - 1.0) * locals.var_dnm_dn9)) } } else { (assign49320_e74120 * (assign49320_e74119 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign49320_e74119) as f64).is_finite() && ((assign49320_e74119) as f64).fract() == 0.0 { if assign49320_e74119 == 0.0 { 0.0 } else { (assign49320_e74119 * ((locals.var_dnm).powf(assign49320_e74119 - 1.0) * locals.var_dnm_dn10)) } } else { (assign49320_e74120 * (assign49320_e74119 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign49320_e74119) as f64).is_finite() && ((assign49320_e74119) as f64).fract() == 0.0 { if assign49320_e74119 == 0.0 { 0.0 } else { (assign49320_e74119 * ((locals.var_dnm).powf(assign49320_e74119 - 1.0) * locals.var_dnm_dn13)) } } else { (assign49320_e74120 * (assign49320_e74119 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign49320_e74121, assign49320_e74121_d_n0, assign49320_e74121_d_n2, assign49320_e74121_d_n4, assign49320_e74121_d_n5, assign49320_e74121_d_n6, assign49320_e74121_d_n7, assign49320_e74121_d_n8, assign49320_e74121_d_n9, assign49320_e74121_d_n10, assign49320_e74121_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign49320_e74123;
        locals.var_dnm_dn0 = assign49320_e74123_d_n0;
        locals.var_dnm_dn2 = assign49320_e74123_d_n2;
        locals.var_dnm_dn4 = assign49320_e74123_d_n4;
        locals.var_dnm_dn5 = assign49320_e74123_d_n5;
        locals.var_dnm_dn6 = assign49320_e74123_d_n6;
        locals.var_dnm_dn7 = assign49320_e74123_d_n7;
        locals.var_dnm_dn8 = assign49320_e74123_d_n8;
        locals.var_dnm_dn9 = assign49320_e74123_d_n9;
        locals.var_dnm_dn10 = assign49320_e74123_d_n10;
        locals.var_dnm_dn13 = assign49320_e74123_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign49330_e74143, assign49330_e74143_d_n0, assign49330_e74143_d_n2, assign49330_e74143_d_n4, assign49330_e74143_d_n5, assign49330_e74143_d_n6, assign49330_e74143_d_n7, assign49330_e74143_d_n8, assign49330_e74143_d_n9, assign49330_e74143_d_n10, assign49330_e74143_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) && (locals.var_guard1278 != 0.0)) {
        let assign49330_e74141: f64 = (1.0 / locals.var_dnm);
        (assign49330_e74141, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign49330_e74143;
        locals.var_dnm_dn0 = assign49330_e74143_d_n0;
        locals.var_dnm_dn2 = assign49330_e74143_d_n2;
        locals.var_dnm_dn4 = assign49330_e74143_d_n4;
        locals.var_dnm_dn5 = assign49330_e74143_d_n5;
        locals.var_dnm_dn6 = assign49330_e74143_d_n6;
        locals.var_dnm_dn7 = assign49330_e74143_d_n7;
        locals.var_dnm_dn8 = assign49330_e74143_d_n8;
        locals.var_dnm_dn9 = assign49330_e74143_d_n9;
        locals.var_dnm_dn10 = assign49330_e74143_d_n10;
        locals.var_dnm_dn13 = assign49330_e74143_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign49340_e74165, assign49340_e74165_d_n0, assign49340_e74165_d_n2, assign49340_e74165_d_n4, assign49340_e74165_d_n5, assign49340_e74165_d_n6, assign49340_e74165_d_n7, assign49340_e74165_d_n8, assign49340_e74165_d_n9, assign49340_e74165_d_n10, assign49340_e74165_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) && (locals.var_guard1278 != 0.0)) {
        let assign49340_e74161: f64 = (locals.var_tmf1 * p.p403);
        let assign49340_e74163: f64 = (assign49340_e74161 * locals.var_dnm);
        (assign49340_e74163, (((locals.var_tmf1_dn0 * p.p403) * locals.var_dnm) + (assign49340_e74161 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * p.p403) * locals.var_dnm) + (assign49340_e74161 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * p.p403) * locals.var_dnm) + (assign49340_e74161 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * p.p403) * locals.var_dnm) + (assign49340_e74161 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * p.p403) * locals.var_dnm) + (assign49340_e74161 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * p.p403) * locals.var_dnm) + (assign49340_e74161 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * p.p403) * locals.var_dnm) + (assign49340_e74161 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * p.p403) * locals.var_dnm) + (assign49340_e74161 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * p.p403) * locals.var_dnm) + (assign49340_e74161 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * p.p403) * locals.var_dnm) + (assign49340_e74161 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign49340_e74165;
        locals.var_tmf0_dn0 = assign49340_e74165_d_n0;
        locals.var_tmf0_dn2 = assign49340_e74165_d_n2;
        locals.var_tmf0_dn4 = assign49340_e74165_d_n4;
        locals.var_tmf0_dn5 = assign49340_e74165_d_n5;
        locals.var_tmf0_dn6 = assign49340_e74165_d_n6;
        locals.var_tmf0_dn7 = assign49340_e74165_d_n7;
        locals.var_tmf0_dn8 = assign49340_e74165_d_n8;
        locals.var_tmf0_dn9 = assign49340_e74165_d_n9;
        locals.var_tmf0_dn10 = assign49340_e74165_d_n10;
        locals.var_tmf0_dn13 = assign49340_e74165_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign49350_e74189, assign49350_e74189_d_n0, assign49350_e74189_d_n2, assign49350_e74189_d_n4, assign49350_e74189_d_n5, assign49350_e74189_d_n6, assign49350_e74189_d_n7, assign49350_e74189_d_n8, assign49350_e74189_d_n9, assign49350_e74189_d_n10, assign49350_e74189_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) && (locals.var_guard1278 != 0.0)) {
        let assign49350_e74183: f64 = (p.p403 * locals.var_xmp);
        let assign49350_e74185: f64 = (assign49350_e74183 * locals.var_dnm);
        let assign49350_e74187: f64 = (assign49350_e74185 / locals.var_arg);
        (assign49350_e74187, ((((((p.p403 * locals.var_xmp_dn0) * locals.var_dnm) + (assign49350_e74183 * locals.var_dnm_dn0)) * locals.var_arg) - (assign49350_e74185 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((p.p403 * locals.var_xmp_dn2) * locals.var_dnm) + (assign49350_e74183 * locals.var_dnm_dn2)) * locals.var_arg) - (assign49350_e74185 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((p.p403 * locals.var_xmp_dn4) * locals.var_dnm) + (assign49350_e74183 * locals.var_dnm_dn4)) * locals.var_arg) - (assign49350_e74185 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((p.p403 * locals.var_xmp_dn5) * locals.var_dnm) + (assign49350_e74183 * locals.var_dnm_dn5)) * locals.var_arg) - (assign49350_e74185 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((p.p403 * locals.var_xmp_dn6) * locals.var_dnm) + (assign49350_e74183 * locals.var_dnm_dn6)) * locals.var_arg) - (assign49350_e74185 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((p.p403 * locals.var_xmp_dn7) * locals.var_dnm) + (assign49350_e74183 * locals.var_dnm_dn7)) * locals.var_arg) - (assign49350_e74185 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((p.p403 * locals.var_xmp_dn8) * locals.var_dnm) + (assign49350_e74183 * locals.var_dnm_dn8)) * locals.var_arg) - (assign49350_e74185 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((p.p403 * locals.var_xmp_dn9) * locals.var_dnm) + (assign49350_e74183 * locals.var_dnm_dn9)) * locals.var_arg) - (assign49350_e74185 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((p.p403 * locals.var_xmp_dn10) * locals.var_dnm) + (assign49350_e74183 * locals.var_dnm_dn10)) * locals.var_arg) - (assign49350_e74185 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((p.p403 * locals.var_xmp_dn13) * locals.var_dnm) + (assign49350_e74183 * locals.var_dnm_dn13)) * locals.var_arg) - (assign49350_e74185 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign49350_e74189;
        locals.var_t0_dn0 = assign49350_e74189_d_n0;
        locals.var_t0_dn2 = assign49350_e74189_d_n2;
        locals.var_t0_dn4 = assign49350_e74189_d_n4;
        locals.var_t0_dn5 = assign49350_e74189_d_n5;
        locals.var_t0_dn6 = assign49350_e74189_d_n6;
        locals.var_t0_dn7 = assign49350_e74189_d_n7;
        locals.var_t0_dn8 = assign49350_e74189_d_n8;
        locals.var_t0_dn9 = assign49350_e74189_d_n9;
        locals.var_t0_dn10 = assign49350_e74189_d_n10;
        locals.var_t0_dn13 = assign49350_e74189_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign49360_e74211, assign49360_e74211_d_n0, assign49360_e74211_d_n2, assign49360_e74211_d_n4, assign49360_e74211_d_n5, assign49360_e74211_d_n6, assign49360_e74211_d_n7, assign49360_e74211_d_n8, assign49360_e74211_d_n9, assign49360_e74211_d_n10, assign49360_e74211_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) && (locals.var_guard1278 != 0.0)) {
        let assign49360_e74207: f64 = p.p403;
        let assign49360_e74209: f64 = (assign49360_e74207 - locals.var_tmf0);
        (assign49360_e74209, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign49360_e74211;
        locals.var_t2_dn0 = assign49360_e74211_d_n0;
        locals.var_t2_dn2 = assign49360_e74211_d_n2;
        locals.var_t2_dn4 = assign49360_e74211_d_n4;
        locals.var_t2_dn5 = assign49360_e74211_d_n5;
        locals.var_t2_dn6 = assign49360_e74211_d_n6;
        locals.var_t2_dn7 = assign49360_e74211_d_n7;
        locals.var_t2_dn8 = assign49360_e74211_d_n8;
        locals.var_t2_dn9 = assign49360_e74211_d_n9;
        locals.var_t2_dn10 = assign49360_e74211_d_n10;
        locals.var_t2_dn13 = assign49360_e74211_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign49370_e74229, assign49370_e74229_d_n0, assign49370_e74229_d_n2, assign49370_e74229_d_n4, assign49370_e74229_d_n5, assign49370_e74229_d_n6, assign49370_e74229_d_n7, assign49370_e74229_d_n8, assign49370_e74229_d_n9, assign49370_e74229_d_n10, assign49370_e74229_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) && (locals.var_guard1278 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign49370_e74229;
        locals.var_t0_dn0 = assign49370_e74229_d_n0;
        locals.var_t0_dn2 = assign49370_e74229_d_n2;
        locals.var_t0_dn4 = assign49370_e74229_d_n4;
        locals.var_t0_dn5 = assign49370_e74229_d_n5;
        locals.var_t0_dn6 = assign49370_e74229_d_n6;
        locals.var_t0_dn7 = assign49370_e74229_d_n7;
        locals.var_t0_dn8 = assign49370_e74229_d_n8;
        locals.var_t0_dn9 = assign49370_e74229_d_n9;
        locals.var_t0_dn10 = assign49370_e74229_d_n10;
        locals.var_t0_dn13 = assign49370_e74229_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign49380_e74250, assign49380_e74250_d_n0, assign49380_e74250_d_n2, assign49380_e74250_d_n4, assign49380_e74250_d_n5, assign49380_e74250_d_n6, assign49380_e74250_d_n7, assign49380_e74250_d_n8, assign49380_e74250_d_n9, assign49380_e74250_d_n10, assign49380_e74250_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) && (locals.var_guard1278 == 0.0)) {
        let assign49380_e74248: f64 = (locals.var_phi_sl_dep__blk1090 - locals.var_vds_maxbl__blk1088);
        (assign49380_e74248, (locals.var_phi_sl_dep__blk1090_dn0 - locals.var_vds_maxbl__blk1088_dn0), (locals.var_phi_sl_dep__blk1090_dn2 - locals.var_vds_maxbl__blk1088_dn2), (locals.var_phi_sl_dep__blk1090_dn4 - locals.var_vds_maxbl__blk1088_dn4), (locals.var_phi_sl_dep__blk1090_dn5 - locals.var_vds_maxbl__blk1088_dn5), (locals.var_phi_sl_dep__blk1090_dn6 - locals.var_vds_maxbl__blk1088_dn6), (locals.var_phi_sl_dep__blk1090_dn7 - locals.var_vds_maxbl__blk1088_dn7), (locals.var_phi_sl_dep__blk1090_dn8 - locals.var_vds_maxbl__blk1088_dn8), (locals.var_phi_sl_dep__blk1090_dn9 - locals.var_vds_maxbl__blk1088_dn9), (locals.var_phi_sl_dep__blk1090_dn10 - locals.var_vds_maxbl__blk1088_dn10), (locals.var_phi_sl_dep__blk1090_dn13 - locals.var_vds_maxbl__blk1088_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign49380_e74250;
        locals.var_t2_dn0 = assign49380_e74250_d_n0;
        locals.var_t2_dn2 = assign49380_e74250_d_n2;
        locals.var_t2_dn4 = assign49380_e74250_d_n4;
        locals.var_t2_dn5 = assign49380_e74250_d_n5;
        locals.var_t2_dn6 = assign49380_e74250_d_n6;
        locals.var_t2_dn7 = assign49380_e74250_d_n7;
        locals.var_t2_dn8 = assign49380_e74250_d_n8;
        locals.var_t2_dn9 = assign49380_e74250_d_n9;
        locals.var_t2_dn10 = assign49380_e74250_d_n10;
        locals.var_t2_dn13 = assign49380_e74250_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign49390_e74269, assign49390_e74269_d_n0, assign49390_e74269_d_n2, assign49390_e74269_d_n4, assign49390_e74269_d_n5, assign49390_e74269_d_n6, assign49390_e74269_d_n7, assign49390_e74269_d_n8, assign49390_e74269_d_n9, assign49390_e74269_d_n10, assign49390_e74269_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) && (locals.var_guard1278 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign49390_e74269;
        locals.var_t0_dn0 = assign49390_e74269_d_n0;
        locals.var_t0_dn2 = assign49390_e74269_d_n2;
        locals.var_t0_dn4 = assign49390_e74269_d_n4;
        locals.var_t0_dn5 = assign49390_e74269_d_n5;
        locals.var_t0_dn6 = assign49390_e74269_d_n6;
        locals.var_t0_dn7 = assign49390_e74269_d_n7;
        locals.var_t0_dn8 = assign49390_e74269_d_n8;
        locals.var_t0_dn9 = assign49390_e74269_d_n9;
        locals.var_t0_dn10 = assign49390_e74269_d_n10;
        locals.var_t0_dn13 = assign49390_e74269_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign49400_e74296, assign49400_e74296_d_n0, assign49400_e74296_d_n2, assign49400_e74296_d_n4, assign49400_e74296_d_n5, assign49400_e74296_d_n6, assign49400_e74296_d_n7, assign49400_e74296_d_n8, assign49400_e74296_d_n9, assign49400_e74296_d_n10, assign49400_e74296_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) {
        let assign49400_e74285: f64 = (locals.var_beta * locals.var_t2);
        let assign49400_e74286: f64 = (assign49400_e74285).exp();
        let assign49400_e74288: f64 = (assign49400_e74286 - 1.0);
        let assign49400_e74291: f64 = (locals.var_beta * locals.var_t2);
        let assign49400_e74292: f64 = (assign49400_e74288 - assign49400_e74291);
        let assign49400_e74294: f64 = (assign49400_e74292 + 1e-15);
        (assign49400_e74294, ((assign49400_e74286 * ((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0))) - ((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0))), ((assign49400_e74286 * ((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2))) - ((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2))), ((assign49400_e74286 * ((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4))) - ((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4))), ((assign49400_e74286 * ((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5))) - ((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5))), ((assign49400_e74286 * ((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6))) - ((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6))), ((assign49400_e74286 * ((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7))) - ((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7))), ((assign49400_e74286 * ((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8))) - ((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8))), ((assign49400_e74286 * ((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9))) - ((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9))), ((assign49400_e74286 * ((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10))) - ((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10))), ((assign49400_e74286 * ((locals.var_beta_dn13 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn13))) - ((locals.var_beta_dn13 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn13))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign49400_e74296;
        locals.var_t4_dn0 = assign49400_e74296_d_n0;
        locals.var_t4_dn2 = assign49400_e74296_d_n2;
        locals.var_t4_dn4 = assign49400_e74296_d_n4;
        locals.var_t4_dn5 = assign49400_e74296_d_n5;
        locals.var_t4_dn6 = assign49400_e74296_d_n6;
        locals.var_t4_dn7 = assign49400_e74296_d_n7;
        locals.var_t4_dn8 = assign49400_e74296_d_n8;
        locals.var_t4_dn9 = assign49400_e74296_d_n9;
        locals.var_t4_dn10 = assign49400_e74296_d_n10;
        locals.var_t4_dn13 = assign49400_e74296_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign49410_e74316, assign49410_e74316_d_n0, assign49410_e74316_d_n2, assign49410_e74316_d_n4, assign49410_e74316_d_n5, assign49410_e74316_d_n6, assign49410_e74316_d_n7, assign49410_e74316_d_n8, assign49410_e74316_d_n9, assign49410_e74316_d_n10, assign49410_e74316_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 != 0.0)) {
        let assign49410_e74311: f64 = (-locals.var_cnst0);
        let assign49410_e74313: f64 = (locals.var_t4).sqrt();
        let assign49410_e74314: f64 = (assign49410_e74311 * assign49410_e74313);
        (assign49410_e74314, (((-locals.var_cnst0_dn0) * assign49410_e74313) + (assign49410_e74311 * (locals.var_t4_dn0 / (2.0 * assign49410_e74313)))), (((-locals.var_cnst0_dn2) * assign49410_e74313) + (assign49410_e74311 * (locals.var_t4_dn2 / (2.0 * assign49410_e74313)))), (((-locals.var_cnst0_dn4) * assign49410_e74313) + (assign49410_e74311 * (locals.var_t4_dn4 / (2.0 * assign49410_e74313)))), (((-locals.var_cnst0_dn5) * assign49410_e74313) + (assign49410_e74311 * (locals.var_t4_dn5 / (2.0 * assign49410_e74313)))), (((-locals.var_cnst0_dn6) * assign49410_e74313) + (assign49410_e74311 * (locals.var_t4_dn6 / (2.0 * assign49410_e74313)))), (((-locals.var_cnst0_dn7) * assign49410_e74313) + (assign49410_e74311 * (locals.var_t4_dn7 / (2.0 * assign49410_e74313)))), (((-locals.var_cnst0_dn8) * assign49410_e74313) + (assign49410_e74311 * (locals.var_t4_dn8 / (2.0 * assign49410_e74313)))), (((-locals.var_cnst0_dn9) * assign49410_e74313) + (assign49410_e74311 * (locals.var_t4_dn9 / (2.0 * assign49410_e74313)))), (((-locals.var_cnst0_dn10) * assign49410_e74313) + (assign49410_e74311 * (locals.var_t4_dn10 / (2.0 * assign49410_e74313)))), (((-locals.var_cnst0_dn13) * assign49410_e74313) + (assign49410_e74311 * (locals.var_t4_dn13 / (2.0 * assign49410_e74313)))),)
    } else {
        (locals.var_q_nl_cur__blk1119, locals.var_q_nl_cur__blk1119_dn0, locals.var_q_nl_cur__blk1119_dn2, locals.var_q_nl_cur__blk1119_dn4, locals.var_q_nl_cur__blk1119_dn5, locals.var_q_nl_cur__blk1119_dn6, locals.var_q_nl_cur__blk1119_dn7, locals.var_q_nl_cur__blk1119_dn8, locals.var_q_nl_cur__blk1119_dn9, locals.var_q_nl_cur__blk1119_dn10, locals.var_q_nl_cur__blk1119_dn13,)
    }
};
        locals.var_q_nl_cur__blk1119 = assign49410_e74316;
        locals.var_q_nl_cur__blk1119_dn0 = assign49410_e74316_d_n0;
        locals.var_q_nl_cur__blk1119_dn2 = assign49410_e74316_d_n2;
        locals.var_q_nl_cur__blk1119_dn4 = assign49410_e74316_d_n4;
        locals.var_q_nl_cur__blk1119_dn5 = assign49410_e74316_d_n5;
        locals.var_q_nl_cur__blk1119_dn6 = assign49410_e74316_d_n6;
        locals.var_q_nl_cur__blk1119_dn7 = assign49410_e74316_d_n7;
        locals.var_q_nl_cur__blk1119_dn8 = assign49410_e74316_d_n8;
        locals.var_q_nl_cur__blk1119_dn9 = assign49410_e74316_d_n9;
        locals.var_q_nl_cur__blk1119_dn10 = assign49410_e74316_d_n10;
        locals.var_q_nl_cur__blk1119_dn13 = assign49410_e74316_d_n13;
        locals.var_q_nl_cur__blk1119_rv = 0.0;

        let (assign49420_e74333, assign49420_e74333_d_n0, assign49420_e74333_d_n2, assign49420_e74333_d_n4, assign49420_e74333_d_n5, assign49420_e74333_d_n6, assign49420_e74333_d_n7, assign49420_e74333_d_n8, assign49420_e74333_d_n9, assign49420_e74333_d_n10, assign49420_e74333_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1252 == 0.0)) && (locals.var_guard1277 == 0.0)) {
        (locals.var_q_nl__blk1123, locals.var_q_nl__blk1123_dn0, locals.var_q_nl__blk1123_dn2, locals.var_q_nl__blk1123_dn4, locals.var_q_nl__blk1123_dn5, locals.var_q_nl__blk1123_dn6, locals.var_q_nl__blk1123_dn7, locals.var_q_nl__blk1123_dn8, locals.var_q_nl__blk1123_dn9, locals.var_q_nl__blk1123_dn10, locals.var_q_nl__blk1123_dn13,)
    } else {
        (locals.var_q_nl_cur__blk1119, locals.var_q_nl_cur__blk1119_dn0, locals.var_q_nl_cur__blk1119_dn2, locals.var_q_nl_cur__blk1119_dn4, locals.var_q_nl_cur__blk1119_dn5, locals.var_q_nl_cur__blk1119_dn6, locals.var_q_nl_cur__blk1119_dn7, locals.var_q_nl_cur__blk1119_dn8, locals.var_q_nl_cur__blk1119_dn9, locals.var_q_nl_cur__blk1119_dn10, locals.var_q_nl_cur__blk1119_dn13,)
    }
};
        locals.var_q_nl_cur__blk1119 = assign49420_e74333;
        locals.var_q_nl_cur__blk1119_dn0 = assign49420_e74333_d_n0;
        locals.var_q_nl_cur__blk1119_dn2 = assign49420_e74333_d_n2;
        locals.var_q_nl_cur__blk1119_dn4 = assign49420_e74333_d_n4;
        locals.var_q_nl_cur__blk1119_dn5 = assign49420_e74333_d_n5;
        locals.var_q_nl_cur__blk1119_dn6 = assign49420_e74333_d_n6;
        locals.var_q_nl_cur__blk1119_dn7 = assign49420_e74333_d_n7;
        locals.var_q_nl_cur__blk1119_dn8 = assign49420_e74333_d_n8;
        locals.var_q_nl_cur__blk1119_dn9 = assign49420_e74333_d_n9;
        locals.var_q_nl_cur__blk1119_dn10 = assign49420_e74333_d_n10;
        locals.var_q_nl_cur__blk1119_dn13 = assign49420_e74333_d_n13;
        locals.var_q_nl_cur__blk1119_rv = 0.0;

        let (assign49430_e74344, assign49430_e74344_d_n0, assign49430_e74344_d_n2, assign49430_e74344_d_n4, assign49430_e74344_d_n5, assign49430_e74344_d_n6, assign49430_e74344_d_n7, assign49430_e74344_d_n8, assign49430_e74344_d_n9, assign49430_e74344_d_n10, assign49430_e74344_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        (locals.var_phi_s0_dep__blk1089, locals.var_phi_s0_dep__blk1089_dn0, locals.var_phi_s0_dep__blk1089_dn2, locals.var_phi_s0_dep__blk1089_dn4, locals.var_phi_s0_dep__blk1089_dn5, locals.var_phi_s0_dep__blk1089_dn6, locals.var_phi_s0_dep__blk1089_dn7, locals.var_phi_s0_dep__blk1089_dn8, locals.var_phi_s0_dep__blk1089_dn9, locals.var_phi_s0_dep__blk1089_dn10, locals.var_phi_s0_dep__blk1089_dn13,)
    } else {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn4, locals.var_ps0_dn5, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn8, locals.var_ps0_dn9, locals.var_ps0_dn10, locals.var_ps0_dn13,)
    }
};
        locals.var_ps0 = assign49430_e74344;
        locals.var_ps0_dn0 = assign49430_e74344_d_n0;
        locals.var_ps0_dn2 = assign49430_e74344_d_n2;
        locals.var_ps0_dn4 = assign49430_e74344_d_n4;
        locals.var_ps0_dn5 = assign49430_e74344_d_n5;
        locals.var_ps0_dn6 = assign49430_e74344_d_n6;
        locals.var_ps0_dn7 = assign49430_e74344_d_n7;
        locals.var_ps0_dn8 = assign49430_e74344_d_n8;
        locals.var_ps0_dn9 = assign49430_e74344_d_n9;
        locals.var_ps0_dn10 = assign49430_e74344_d_n10;
        locals.var_ps0_dn13 = assign49430_e74344_d_n13;
        locals.var_ps0_rv = 0.0;

        let (assign49440_e74355, assign49440_e74355_d_n0, assign49440_e74355_d_n2, assign49440_e74355_d_n4, assign49440_e74355_d_n5, assign49440_e74355_d_n6, assign49440_e74355_d_n7, assign49440_e74355_d_n8, assign49440_e74355_d_n9, assign49440_e74355_d_n10, assign49440_e74355_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        (locals.var_phi_sl_dep__blk1090, locals.var_phi_sl_dep__blk1090_dn0, locals.var_phi_sl_dep__blk1090_dn2, locals.var_phi_sl_dep__blk1090_dn4, locals.var_phi_sl_dep__blk1090_dn5, locals.var_phi_sl_dep__blk1090_dn6, locals.var_phi_sl_dep__blk1090_dn7, locals.var_phi_sl_dep__blk1090_dn8, locals.var_phi_sl_dep__blk1090_dn9, locals.var_phi_sl_dep__blk1090_dn10, locals.var_phi_sl_dep__blk1090_dn13,)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn4, locals.var_psl_dn5, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn8, locals.var_psl_dn9, locals.var_psl_dn10, locals.var_psl_dn13,)
    }
};
        locals.var_psl = assign49440_e74355;
        locals.var_psl_dn0 = assign49440_e74355_d_n0;
        locals.var_psl_dn2 = assign49440_e74355_d_n2;
        locals.var_psl_dn4 = assign49440_e74355_d_n4;
        locals.var_psl_dn5 = assign49440_e74355_d_n5;
        locals.var_psl_dn6 = assign49440_e74355_d_n6;
        locals.var_psl_dn7 = assign49440_e74355_d_n7;
        locals.var_psl_dn8 = assign49440_e74355_d_n8;
        locals.var_psl_dn9 = assign49440_e74355_d_n9;
        locals.var_psl_dn10 = assign49440_e74355_d_n10;
        locals.var_psl_dn13 = assign49440_e74355_d_n13;
        locals.var_psl_rv = 0.0;

        let (assign49450_e74368, assign49450_e74368_d_n0, assign49450_e74368_d_n2, assign49450_e74368_d_n4, assign49450_e74368_d_n5, assign49450_e74368_d_n6, assign49450_e74368_d_n7, assign49450_e74368_d_n8, assign49450_e74368_d_n9, assign49450_e74368_d_n10, assign49450_e74368_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign49450_e74366: f64 = (locals.var_phi_sl_dep__blk1090 - locals.var_phi_s0_dep__blk1089);
        (assign49450_e74366, (locals.var_phi_sl_dep__blk1090_dn0 - locals.var_phi_s0_dep__blk1089_dn0), (locals.var_phi_sl_dep__blk1090_dn2 - locals.var_phi_s0_dep__blk1089_dn2), (locals.var_phi_sl_dep__blk1090_dn4 - locals.var_phi_s0_dep__blk1089_dn4), (locals.var_phi_sl_dep__blk1090_dn5 - locals.var_phi_s0_dep__blk1089_dn5), (locals.var_phi_sl_dep__blk1090_dn6 - locals.var_phi_s0_dep__blk1089_dn6), (locals.var_phi_sl_dep__blk1090_dn7 - locals.var_phi_s0_dep__blk1089_dn7), (locals.var_phi_sl_dep__blk1090_dn8 - locals.var_phi_s0_dep__blk1089_dn8), (locals.var_phi_sl_dep__blk1090_dn9 - locals.var_phi_s0_dep__blk1089_dn9), (locals.var_phi_sl_dep__blk1090_dn10 - locals.var_phi_s0_dep__blk1089_dn10), (locals.var_phi_sl_dep__blk1090_dn13 - locals.var_phi_s0_dep__blk1089_dn13),)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn4, locals.var_pds_dn5, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn8, locals.var_pds_dn9, locals.var_pds_dn10, locals.var_pds_dn13,)
    }
};
        locals.var_pds = assign49450_e74368;
        locals.var_pds_dn0 = assign49450_e74368_d_n0;
        locals.var_pds_dn2 = assign49450_e74368_d_n2;
        locals.var_pds_dn4 = assign49450_e74368_d_n4;
        locals.var_pds_dn5 = assign49450_e74368_d_n5;
        locals.var_pds_dn6 = assign49450_e74368_d_n6;
        locals.var_pds_dn7 = assign49450_e74368_d_n7;
        locals.var_pds_dn8 = assign49450_e74368_d_n8;
        locals.var_pds_dn9 = assign49450_e74368_d_n9;
        locals.var_pds_dn10 = assign49450_e74368_d_n10;
        locals.var_pds_dn13 = assign49450_e74368_d_n13;
        locals.var_pds_rv = 0.0;

        let (assign49460_e74383, assign49460_e74383_d_n0, assign49460_e74383_d_n2, assign49460_e74383_d_n4, assign49460_e74383_d_n5, assign49460_e74383_d_n6, assign49460_e74383_d_n7, assign49460_e74383_d_n8, assign49460_e74383_d_n9, assign49460_e74383_d_n10, assign49460_e74383_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign49460_e74379: f64 = (locals.var_vds - locals.var_pds);
        let assign49460_e74381: f64 = (assign49460_e74379 / 2.0);
        (assign49460_e74381, ((locals.var_vds_dn0 - locals.var_pds_dn0) / 2.0), ((locals.var_vds_dn2 - locals.var_pds_dn2) / 2.0), ((locals.var_vds_dn4 - locals.var_pds_dn4) / 2.0), ((locals.var_vds_dn5 - locals.var_pds_dn5) / 2.0), ((locals.var_vds_dn6 - locals.var_pds_dn6) / 2.0), ((locals.var_vds_dn7 - locals.var_pds_dn7) / 2.0), ((locals.var_vds_dn8 - locals.var_pds_dn8) / 2.0), ((locals.var_vds_dn9 - locals.var_pds_dn9) / 2.0), ((locals.var_vds_dn10 - locals.var_pds_dn10) / 2.0), ((locals.var_vds_dn13 - locals.var_pds_dn13) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign49460_e74383;
        locals.var_t1_dn0 = assign49460_e74383_d_n0;
        locals.var_t1_dn2 = assign49460_e74383_d_n2;
        locals.var_t1_dn4 = assign49460_e74383_d_n4;
        locals.var_t1_dn5 = assign49460_e74383_d_n5;
        locals.var_t1_dn6 = assign49460_e74383_d_n6;
        locals.var_t1_dn7 = assign49460_e74383_d_n7;
        locals.var_t1_dn8 = assign49460_e74383_d_n8;
        locals.var_t1_dn9 = assign49460_e74383_d_n9;
        locals.var_t1_dn10 = assign49460_e74383_d_n10;
        locals.var_t1_dn13 = assign49460_e74383_d_n13;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_171(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign49470_e74400, assign49470_e74400_d_n0, assign49470_e74400_d_n2, assign49470_e74400_d_n4, assign49470_e74400_d_n5, assign49470_e74400_d_n6, assign49470_e74400_d_n7, assign49470_e74400_d_n8, assign49470_e74400_d_n9, assign49470_e74400_d_n10, assign49470_e74400_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign49470_e74394: f64 = (2.0 * locals.var_t1);
        let assign49470_e74397: f64 = (p.p263 * 0.1);
        let assign49470_e74398: f64 = (assign49470_e74394 / assign49470_e74397);
        (assign49470_e74398, ((2.0 * locals.var_t1_dn0) / assign49470_e74397), ((2.0 * locals.var_t1_dn2) / assign49470_e74397), ((2.0 * locals.var_t1_dn4) / assign49470_e74397), ((2.0 * locals.var_t1_dn5) / assign49470_e74397), ((2.0 * locals.var_t1_dn6) / assign49470_e74397), ((2.0 * locals.var_t1_dn7) / assign49470_e74397), ((2.0 * locals.var_t1_dn8) / assign49470_e74397), ((2.0 * locals.var_t1_dn9) / assign49470_e74397), ((2.0 * locals.var_t1_dn10) / assign49470_e74397), ((2.0 * locals.var_t1_dn13) / assign49470_e74397),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign49470_e74400;
        locals.var_tmf1_dn0 = assign49470_e74400_d_n0;
        locals.var_tmf1_dn2 = assign49470_e74400_d_n2;
        locals.var_tmf1_dn4 = assign49470_e74400_d_n4;
        locals.var_tmf1_dn5 = assign49470_e74400_d_n5;
        locals.var_tmf1_dn6 = assign49470_e74400_d_n6;
        locals.var_tmf1_dn7 = assign49470_e74400_d_n7;
        locals.var_tmf1_dn8 = assign49470_e74400_d_n8;
        locals.var_tmf1_dn9 = assign49470_e74400_d_n9;
        locals.var_tmf1_dn10 = assign49470_e74400_d_n10;
        locals.var_tmf1_dn13 = assign49470_e74400_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign49480_e74447, assign49480_e74447_d_n0, assign49480_e74447_d_n2, assign49480_e74447_d_n4, assign49480_e74447_d_n5, assign49480_e74447_d_n6, assign49480_e74447_d_n7, assign49480_e74447_d_n8, assign49480_e74447_d_n9, assign49480_e74447_d_n10, assign49480_e74447_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign49480_e74413: f64 = (1.0 / 2.0);
        let assign49480_e74417: f64 = (1.0 / 6.0);
        let assign49480_e74421: f64 = (1.0 / 24.0);
        let assign49480_e74425: f64 = (1.0 / 120.0);
        let assign49480_e74429: f64 = (1.0 / 720.0);
        let assign49480_e74433: f64 = (1.0 / 5040.0);
        let assign49480_e74434: f64 = (locals.var_tmf1 * assign49480_e74433);
        let assign49480_e74435: f64 = (assign49480_e74429 + assign49480_e74434);
        let assign49480_e74436: f64 = (locals.var_tmf1 * assign49480_e74435);
        let assign49480_e74437: f64 = (assign49480_e74425 + assign49480_e74436);
        let assign49480_e74438: f64 = (locals.var_tmf1 * assign49480_e74437);
        let assign49480_e74439: f64 = (assign49480_e74421 + assign49480_e74438);
        let assign49480_e74440: f64 = (locals.var_tmf1 * assign49480_e74439);
        let assign49480_e74441: f64 = (assign49480_e74417 + assign49480_e74440);
        let assign49480_e74442: f64 = (locals.var_tmf1 * assign49480_e74441);
        let assign49480_e74443: f64 = (assign49480_e74413 + assign49480_e74442);
        let assign49480_e74444: f64 = (locals.var_tmf1 * assign49480_e74443);
        let assign49480_e74445: f64 = (1.0 + assign49480_e74444);
        (assign49480_e74445, ((locals.var_tmf1_dn0 * assign49480_e74443) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign49480_e74441) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign49480_e74439) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign49480_e74437) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign49480_e74435) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign49480_e74433))))))))))), ((locals.var_tmf1_dn2 * assign49480_e74443) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign49480_e74441) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign49480_e74439) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign49480_e74437) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign49480_e74435) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign49480_e74433))))))))))), ((locals.var_tmf1_dn4 * assign49480_e74443) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign49480_e74441) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign49480_e74439) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign49480_e74437) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign49480_e74435) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign49480_e74433))))))))))), ((locals.var_tmf1_dn5 * assign49480_e74443) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign49480_e74441) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign49480_e74439) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign49480_e74437) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign49480_e74435) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign49480_e74433))))))))))), ((locals.var_tmf1_dn6 * assign49480_e74443) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign49480_e74441) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign49480_e74439) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign49480_e74437) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign49480_e74435) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign49480_e74433))))))))))), ((locals.var_tmf1_dn7 * assign49480_e74443) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign49480_e74441) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign49480_e74439) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign49480_e74437) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign49480_e74435) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign49480_e74433))))))))))), ((locals.var_tmf1_dn8 * assign49480_e74443) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign49480_e74441) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign49480_e74439) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign49480_e74437) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign49480_e74435) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign49480_e74433))))))))))), ((locals.var_tmf1_dn9 * assign49480_e74443) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign49480_e74441) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign49480_e74439) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign49480_e74437) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign49480_e74435) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign49480_e74433))))))))))), ((locals.var_tmf1_dn10 * assign49480_e74443) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign49480_e74441) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign49480_e74439) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign49480_e74437) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign49480_e74435) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign49480_e74433))))))))))), ((locals.var_tmf1_dn13 * assign49480_e74443) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign49480_e74441) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign49480_e74439) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign49480_e74437) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign49480_e74435) + (locals.var_tmf1 * (locals.var_tmf1_dn13 * assign49480_e74433))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign49480_e74447;
        locals.var_tmf2_dn0 = assign49480_e74447_d_n0;
        locals.var_tmf2_dn2 = assign49480_e74447_d_n2;
        locals.var_tmf2_dn4 = assign49480_e74447_d_n4;
        locals.var_tmf2_dn5 = assign49480_e74447_d_n5;
        locals.var_tmf2_dn6 = assign49480_e74447_d_n6;
        locals.var_tmf2_dn7 = assign49480_e74447_d_n7;
        locals.var_tmf2_dn8 = assign49480_e74447_d_n8;
        locals.var_tmf2_dn9 = assign49480_e74447_d_n9;
        locals.var_tmf2_dn10 = assign49480_e74447_d_n10;
        locals.var_tmf2_dn13 = assign49480_e74447_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign49490_e74490, assign49490_e74490_d_n0, assign49490_e74490_d_n2, assign49490_e74490_d_n4, assign49490_e74490_d_n5, assign49490_e74490_d_n6, assign49490_e74490_d_n7, assign49490_e74490_d_n8, assign49490_e74490_d_n9, assign49490_e74490_d_n10, assign49490_e74490_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign49490_e74458: f64 = (1.0 / 2.0);
        let assign49490_e74462: f64 = (1.0 / 3.0);
        let assign49490_e74466: f64 = (1.0 / 8.0);
        let assign49490_e74470: f64 = (1.0 / 30.0);
        let assign49490_e74474: f64 = (1.0 / 144.0);
        let assign49490_e74478: f64 = (1.0 / 840.0);
        let assign49490_e74479: f64 = (locals.var_tmf1 * assign49490_e74478);
        let assign49490_e74480: f64 = (assign49490_e74474 + assign49490_e74479);
        let assign49490_e74481: f64 = (locals.var_tmf1 * assign49490_e74480);
        let assign49490_e74482: f64 = (assign49490_e74470 + assign49490_e74481);
        let assign49490_e74483: f64 = (locals.var_tmf1 * assign49490_e74482);
        let assign49490_e74484: f64 = (assign49490_e74466 + assign49490_e74483);
        let assign49490_e74485: f64 = (locals.var_tmf1 * assign49490_e74484);
        let assign49490_e74486: f64 = (assign49490_e74462 + assign49490_e74485);
        let assign49490_e74487: f64 = (locals.var_tmf1 * assign49490_e74486);
        let assign49490_e74488: f64 = (assign49490_e74458 + assign49490_e74487);
        (assign49490_e74488, ((locals.var_tmf1_dn0 * assign49490_e74486) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign49490_e74484) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign49490_e74482) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign49490_e74480) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign49490_e74478))))))))), ((locals.var_tmf1_dn2 * assign49490_e74486) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign49490_e74484) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign49490_e74482) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign49490_e74480) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign49490_e74478))))))))), ((locals.var_tmf1_dn4 * assign49490_e74486) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign49490_e74484) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign49490_e74482) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign49490_e74480) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign49490_e74478))))))))), ((locals.var_tmf1_dn5 * assign49490_e74486) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign49490_e74484) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign49490_e74482) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign49490_e74480) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign49490_e74478))))))))), ((locals.var_tmf1_dn6 * assign49490_e74486) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign49490_e74484) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign49490_e74482) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign49490_e74480) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign49490_e74478))))))))), ((locals.var_tmf1_dn7 * assign49490_e74486) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign49490_e74484) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign49490_e74482) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign49490_e74480) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign49490_e74478))))))))), ((locals.var_tmf1_dn8 * assign49490_e74486) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign49490_e74484) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign49490_e74482) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign49490_e74480) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign49490_e74478))))))))), ((locals.var_tmf1_dn9 * assign49490_e74486) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign49490_e74484) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign49490_e74482) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign49490_e74480) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign49490_e74478))))))))), ((locals.var_tmf1_dn10 * assign49490_e74486) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign49490_e74484) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign49490_e74482) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign49490_e74480) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign49490_e74478))))))))), ((locals.var_tmf1_dn13 * assign49490_e74486) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign49490_e74484) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign49490_e74482) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign49490_e74480) + (locals.var_tmf1 * (locals.var_tmf1_dn13 * assign49490_e74478))))))))),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn13,)
    }
};
        locals.var_tmf3 = assign49490_e74490;
        locals.var_tmf3_dn0 = assign49490_e74490_d_n0;
        locals.var_tmf3_dn2 = assign49490_e74490_d_n2;
        locals.var_tmf3_dn4 = assign49490_e74490_d_n4;
        locals.var_tmf3_dn5 = assign49490_e74490_d_n5;
        locals.var_tmf3_dn6 = assign49490_e74490_d_n6;
        locals.var_tmf3_dn7 = assign49490_e74490_d_n7;
        locals.var_tmf3_dn8 = assign49490_e74490_d_n8;
        locals.var_tmf3_dn9 = assign49490_e74490_d_n9;
        locals.var_tmf3_dn10 = assign49490_e74490_d_n10;
        locals.var_tmf3_dn13 = assign49490_e74490_d_n13;
        locals.var_tmf3_rv = 0.0;

        let (assign49500_e74505, assign49500_e74505_d_n0, assign49500_e74505_d_n2, assign49500_e74505_d_n4, assign49500_e74505_d_n5, assign49500_e74505_d_n6, assign49500_e74505_d_n7, assign49500_e74505_d_n8, assign49500_e74505_d_n9, assign49500_e74505_d_n10, assign49500_e74505_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign49500_e74501: f64 = (p.p263 * 0.1);
        let assign49500_e74503: f64 = (assign49500_e74501 / locals.var_tmf2);
        (assign49500_e74503, (-((assign49500_e74501 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((assign49500_e74501 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((assign49500_e74501 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2))), (-((assign49500_e74501 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2))), (-((assign49500_e74501 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((assign49500_e74501 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((assign49500_e74501 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2))), (-((assign49500_e74501 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2))), (-((assign49500_e74501 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((assign49500_e74501 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_pzadd, locals.var_pzadd_dn0, locals.var_pzadd_dn2, locals.var_pzadd_dn4, locals.var_pzadd_dn5, locals.var_pzadd_dn6, locals.var_pzadd_dn7, locals.var_pzadd_dn8, locals.var_pzadd_dn9, locals.var_pzadd_dn10, locals.var_pzadd_dn13,)
    }
};
        locals.var_pzadd = assign49500_e74505;
        locals.var_pzadd_dn0 = assign49500_e74505_d_n0;
        locals.var_pzadd_dn2 = assign49500_e74505_d_n2;
        locals.var_pzadd_dn4 = assign49500_e74505_d_n4;
        locals.var_pzadd_dn5 = assign49500_e74505_d_n5;
        locals.var_pzadd_dn6 = assign49500_e74505_d_n6;
        locals.var_pzadd_dn7 = assign49500_e74505_d_n7;
        locals.var_pzadd_dn8 = assign49500_e74505_d_n8;
        locals.var_pzadd_dn9 = assign49500_e74505_d_n9;
        locals.var_pzadd_dn10 = assign49500_e74505_d_n10;
        locals.var_pzadd_dn13 = assign49500_e74505_d_n13;
        locals.var_pzadd_rv = 0.0;

        let (assign49510_e74523, assign49510_e74523_d_n0, assign49510_e74523_d_n2, assign49510_e74523_d_n4, assign49510_e74523_d_n5, assign49510_e74523_d_n6, assign49510_e74523_d_n7, assign49510_e74523_d_n8, assign49510_e74523_d_n9, assign49510_e74523_d_n10, assign49510_e74523_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign49510_e74515: f64 = (-2.0);
        let assign49510_e74517: f64 = (assign49510_e74515 * locals.var_tmf3);
        let assign49510_e74520: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign49510_e74521: f64 = (assign49510_e74517 / assign49510_e74520);
        (assign49510_e74521, ((((assign49510_e74515 * locals.var_tmf3_dn0) * assign49510_e74520) - (assign49510_e74517 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign49510_e74520 * assign49510_e74520)), ((((assign49510_e74515 * locals.var_tmf3_dn2) * assign49510_e74520) - (assign49510_e74517 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign49510_e74520 * assign49510_e74520)), ((((assign49510_e74515 * locals.var_tmf3_dn4) * assign49510_e74520) - (assign49510_e74517 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign49510_e74520 * assign49510_e74520)), ((((assign49510_e74515 * locals.var_tmf3_dn5) * assign49510_e74520) - (assign49510_e74517 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign49510_e74520 * assign49510_e74520)), ((((assign49510_e74515 * locals.var_tmf3_dn6) * assign49510_e74520) - (assign49510_e74517 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign49510_e74520 * assign49510_e74520)), ((((assign49510_e74515 * locals.var_tmf3_dn7) * assign49510_e74520) - (assign49510_e74517 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign49510_e74520 * assign49510_e74520)), ((((assign49510_e74515 * locals.var_tmf3_dn8) * assign49510_e74520) - (assign49510_e74517 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign49510_e74520 * assign49510_e74520)), ((((assign49510_e74515 * locals.var_tmf3_dn9) * assign49510_e74520) - (assign49510_e74517 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign49510_e74520 * assign49510_e74520)), ((((assign49510_e74515 * locals.var_tmf3_dn10) * assign49510_e74520) - (assign49510_e74517 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign49510_e74520 * assign49510_e74520)), ((((assign49510_e74515 * locals.var_tmf3_dn13) * assign49510_e74520) - (assign49510_e74517 * ((locals.var_tmf2_dn13 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn13)))) / (assign49510_e74520 * assign49510_e74520)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign49510_e74523;
        locals.var_t2_dn0 = assign49510_e74523_d_n0;
        locals.var_t2_dn2 = assign49510_e74523_d_n2;
        locals.var_t2_dn4 = assign49510_e74523_d_n4;
        locals.var_t2_dn5 = assign49510_e74523_d_n5;
        locals.var_t2_dn6 = assign49510_e74523_d_n6;
        locals.var_t2_dn7 = assign49510_e74523_d_n7;
        locals.var_t2_dn8 = assign49510_e74523_d_n8;
        locals.var_t2_dn9 = assign49510_e74523_d_n9;
        locals.var_t2_dn10 = assign49510_e74523_d_n10;
        locals.var_t2_dn13 = assign49510_e74523_d_n13;
        locals.var_t2_rv = 0.0;

        let assign49520_e74527: f64 = (10.0 * 2.220446049250313e-16);
        let assign49520_e74530: f64 = (10.0 * 2.220446049250313e-16);
        let assign49520_e74531: f64 = (assign49520_e74527 + assign49520_e74530);
        let assign49520_e74535: f64 = (10.0 * 2.220446049250313e-16);
        let assign49520_e74538: f64 = if ((locals.var_pzadd < assign49520_e74531) && (assign49520_e74535 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1284 = assign49520_e74538;
        locals.var_guard1284_rv = 0.0;

        let (assign49530_e74559, assign49530_e74559_d_n0, assign49530_e74559_d_n2, assign49530_e74559_d_n4, assign49530_e74559_d_n5, assign49530_e74559_d_n6, assign49530_e74559_d_n7, assign49530_e74559_d_n8, assign49530_e74559_d_n9, assign49530_e74559_d_n10, assign49530_e74559_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1284 != 0.0)) {
        let assign49530_e74551: f64 = (10.0 * 2.220446049250313e-16);
        let assign49530_e74554: f64 = (10.0 * 2.220446049250313e-16);
        let assign49530_e74555: f64 = (assign49530_e74551 + assign49530_e74554);
        let assign49530_e74557: f64 = (assign49530_e74555 - locals.var_pzadd);
        (assign49530_e74557, (-locals.var_pzadd_dn0), (-locals.var_pzadd_dn2), (-locals.var_pzadd_dn4), (-locals.var_pzadd_dn5), (-locals.var_pzadd_dn6), (-locals.var_pzadd_dn7), (-locals.var_pzadd_dn8), (-locals.var_pzadd_dn9), (-locals.var_pzadd_dn10), (-locals.var_pzadd_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign49530_e74559;
        locals.var_tmf1_dn0 = assign49530_e74559_d_n0;
        locals.var_tmf1_dn2 = assign49530_e74559_d_n2;
        locals.var_tmf1_dn4 = assign49530_e74559_d_n4;
        locals.var_tmf1_dn5 = assign49530_e74559_d_n5;
        locals.var_tmf1_dn6 = assign49530_e74559_d_n6;
        locals.var_tmf1_dn7 = assign49530_e74559_d_n7;
        locals.var_tmf1_dn8 = assign49530_e74559_d_n8;
        locals.var_tmf1_dn9 = assign49530_e74559_d_n9;
        locals.var_tmf1_dn10 = assign49530_e74559_d_n10;
        locals.var_tmf1_dn13 = assign49530_e74559_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign49540_e74574, assign49540_e74574_d_n0, assign49540_e74574_d_n2, assign49540_e74574_d_n4, assign49540_e74574_d_n5, assign49540_e74574_d_n6, assign49540_e74574_d_n7, assign49540_e74574_d_n8, assign49540_e74574_d_n9, assign49540_e74574_d_n10, assign49540_e74574_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1284 != 0.0)) {
        let assign49540_e74572: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign49540_e74572, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign49540_e74574;
        locals.var_x2_dn0 = assign49540_e74574_d_n0;
        locals.var_x2_dn2 = assign49540_e74574_d_n2;
        locals.var_x2_dn4 = assign49540_e74574_d_n4;
        locals.var_x2_dn5 = assign49540_e74574_d_n5;
        locals.var_x2_dn6 = assign49540_e74574_d_n6;
        locals.var_x2_dn7 = assign49540_e74574_d_n7;
        locals.var_x2_dn8 = assign49540_e74574_d_n8;
        locals.var_x2_dn9 = assign49540_e74574_d_n9;
        locals.var_x2_dn10 = assign49540_e74574_d_n10;
        locals.var_x2_dn13 = assign49540_e74574_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign49550_e74593, assign49550_e74593_d_n0, assign49550_e74593_d_n2, assign49550_e74593_d_n4, assign49550_e74593_d_n5, assign49550_e74593_d_n6, assign49550_e74593_d_n7, assign49550_e74593_d_n8, assign49550_e74593_d_n9, assign49550_e74593_d_n10, assign49550_e74593_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1284 != 0.0)) {
        let assign49550_e74587: f64 = (10.0 * 2.220446049250313e-16);
        let assign49550_e74590: f64 = (10.0 * 2.220446049250313e-16);
        let assign49550_e74591: f64 = (assign49550_e74587 * assign49550_e74590);
        (assign49550_e74591, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign49550_e74593;
        locals.var_xmax2_dn0 = assign49550_e74593_d_n0;
        locals.var_xmax2_dn2 = assign49550_e74593_d_n2;
        locals.var_xmax2_dn4 = assign49550_e74593_d_n4;
        locals.var_xmax2_dn5 = assign49550_e74593_d_n5;
        locals.var_xmax2_dn6 = assign49550_e74593_d_n6;
        locals.var_xmax2_dn7 = assign49550_e74593_d_n7;
        locals.var_xmax2_dn8 = assign49550_e74593_d_n8;
        locals.var_xmax2_dn9 = assign49550_e74593_d_n9;
        locals.var_xmax2_dn10 = assign49550_e74593_d_n10;
        locals.var_xmax2_dn13 = assign49550_e74593_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign49560_e74606, assign49560_e74606_d_n0, assign49560_e74606_d_n2, assign49560_e74606_d_n4, assign49560_e74606_d_n5, assign49560_e74606_d_n6, assign49560_e74606_d_n7, assign49560_e74606_d_n8, assign49560_e74606_d_n9, assign49560_e74606_d_n10, assign49560_e74606_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1284 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign49560_e74606;
        locals.var_xp_dn0 = assign49560_e74606_d_n0;
        locals.var_xp_dn2 = assign49560_e74606_d_n2;
        locals.var_xp_dn4 = assign49560_e74606_d_n4;
        locals.var_xp_dn5 = assign49560_e74606_d_n5;
        locals.var_xp_dn6 = assign49560_e74606_d_n6;
        locals.var_xp_dn7 = assign49560_e74606_d_n7;
        locals.var_xp_dn8 = assign49560_e74606_d_n8;
        locals.var_xp_dn9 = assign49560_e74606_d_n9;
        locals.var_xp_dn10 = assign49560_e74606_d_n10;
        locals.var_xp_dn13 = assign49560_e74606_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign49570_e74619, assign49570_e74619_d_n0, assign49570_e74619_d_n2, assign49570_e74619_d_n4, assign49570_e74619_d_n5, assign49570_e74619_d_n6, assign49570_e74619_d_n7, assign49570_e74619_d_n8, assign49570_e74619_d_n9, assign49570_e74619_d_n10, assign49570_e74619_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1284 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign49570_e74619;
        locals.var_xmp_dn0 = assign49570_e74619_d_n0;
        locals.var_xmp_dn2 = assign49570_e74619_d_n2;
        locals.var_xmp_dn4 = assign49570_e74619_d_n4;
        locals.var_xmp_dn5 = assign49570_e74619_d_n5;
        locals.var_xmp_dn6 = assign49570_e74619_d_n6;
        locals.var_xmp_dn7 = assign49570_e74619_d_n7;
        locals.var_xmp_dn8 = assign49570_e74619_d_n8;
        locals.var_xmp_dn9 = assign49570_e74619_d_n9;
        locals.var_xmp_dn10 = assign49570_e74619_d_n10;
        locals.var_xmp_dn13 = assign49570_e74619_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign49580_e74632,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1284 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign49580_e74632;
        locals.var_m0_rv = 0.0;

        let (assign49590_e74645,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1284 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign49590_e74645;
        locals.var_mm_rv = 0.0;

        let (assign49600_e74658, assign49600_e74658_d_n0, assign49600_e74658_d_n2, assign49600_e74658_d_n4, assign49600_e74658_d_n5, assign49600_e74658_d_n6, assign49600_e74658_d_n7, assign49600_e74658_d_n8, assign49600_e74658_d_n9, assign49600_e74658_d_n10, assign49600_e74658_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1284 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign49600_e74658;
        locals.var_arg_dn0 = assign49600_e74658_d_n0;
        locals.var_arg_dn2 = assign49600_e74658_d_n2;
        locals.var_arg_dn4 = assign49600_e74658_d_n4;
        locals.var_arg_dn5 = assign49600_e74658_d_n5;
        locals.var_arg_dn6 = assign49600_e74658_d_n6;
        locals.var_arg_dn7 = assign49600_e74658_d_n7;
        locals.var_arg_dn8 = assign49600_e74658_d_n8;
        locals.var_arg_dn9 = assign49600_e74658_d_n9;
        locals.var_arg_dn10 = assign49600_e74658_d_n10;
        locals.var_arg_dn13 = assign49600_e74658_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign49610_e74671, assign49610_e74671_d_n0, assign49610_e74671_d_n2, assign49610_e74671_d_n4, assign49610_e74671_d_n5, assign49610_e74671_d_n6, assign49610_e74671_d_n7, assign49610_e74671_d_n8, assign49610_e74671_d_n9, assign49610_e74671_d_n10, assign49610_e74671_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1284 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign49610_e74671;
        locals.var_dnm_dn0 = assign49610_e74671_d_n0;
        locals.var_dnm_dn2 = assign49610_e74671_d_n2;
        locals.var_dnm_dn4 = assign49610_e74671_d_n4;
        locals.var_dnm_dn5 = assign49610_e74671_d_n5;
        locals.var_dnm_dn6 = assign49610_e74671_d_n6;
        locals.var_dnm_dn7 = assign49610_e74671_d_n7;
        locals.var_dnm_dn8 = assign49610_e74671_d_n8;
        locals.var_dnm_dn9 = assign49610_e74671_d_n9;
        locals.var_dnm_dn10 = assign49610_e74671_d_n10;
        locals.var_dnm_dn13 = assign49610_e74671_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign49620_e74686, assign49620_e74686_d_n0, assign49620_e74686_d_n2, assign49620_e74686_d_n4, assign49620_e74686_d_n5, assign49620_e74686_d_n6, assign49620_e74686_d_n7, assign49620_e74686_d_n8, assign49620_e74686_d_n9, assign49620_e74686_d_n10, assign49620_e74686_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1284 != 0.0)) {
        let assign49620_e74684: f64 = (locals.var_xp * locals.var_x2);
        (assign49620_e74684, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign49620_e74686;
        locals.var_xp_dn0 = assign49620_e74686_d_n0;
        locals.var_xp_dn2 = assign49620_e74686_d_n2;
        locals.var_xp_dn4 = assign49620_e74686_d_n4;
        locals.var_xp_dn5 = assign49620_e74686_d_n5;
        locals.var_xp_dn6 = assign49620_e74686_d_n6;
        locals.var_xp_dn7 = assign49620_e74686_d_n7;
        locals.var_xp_dn8 = assign49620_e74686_d_n8;
        locals.var_xp_dn9 = assign49620_e74686_d_n9;
        locals.var_xp_dn10 = assign49620_e74686_d_n10;
        locals.var_xp_dn13 = assign49620_e74686_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign49630_e74701, assign49630_e74701_d_n0, assign49630_e74701_d_n2, assign49630_e74701_d_n4, assign49630_e74701_d_n5, assign49630_e74701_d_n6, assign49630_e74701_d_n7, assign49630_e74701_d_n8, assign49630_e74701_d_n9, assign49630_e74701_d_n10, assign49630_e74701_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1284 != 0.0)) {
        let assign49630_e74699: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign49630_e74699, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign49630_e74701;
        locals.var_xmp_dn0 = assign49630_e74701_d_n0;
        locals.var_xmp_dn2 = assign49630_e74701_d_n2;
        locals.var_xmp_dn4 = assign49630_e74701_d_n4;
        locals.var_xmp_dn5 = assign49630_e74701_d_n5;
        locals.var_xmp_dn6 = assign49630_e74701_d_n6;
        locals.var_xmp_dn7 = assign49630_e74701_d_n7;
        locals.var_xmp_dn8 = assign49630_e74701_d_n8;
        locals.var_xmp_dn9 = assign49630_e74701_d_n9;
        locals.var_xmp_dn10 = assign49630_e74701_d_n10;
        locals.var_xmp_dn13 = assign49630_e74701_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign49640_e74716, assign49640_e74716_d_n0, assign49640_e74716_d_n2, assign49640_e74716_d_n4, assign49640_e74716_d_n5, assign49640_e74716_d_n6, assign49640_e74716_d_n7, assign49640_e74716_d_n8, assign49640_e74716_d_n9, assign49640_e74716_d_n10, assign49640_e74716_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1284 != 0.0)) {
        let assign49640_e74714: f64 = (locals.var_xp * locals.var_x2);
        (assign49640_e74714, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign49640_e74716;
        locals.var_xp_dn0 = assign49640_e74716_d_n0;
        locals.var_xp_dn2 = assign49640_e74716_d_n2;
        locals.var_xp_dn4 = assign49640_e74716_d_n4;
        locals.var_xp_dn5 = assign49640_e74716_d_n5;
        locals.var_xp_dn6 = assign49640_e74716_d_n6;
        locals.var_xp_dn7 = assign49640_e74716_d_n7;
        locals.var_xp_dn8 = assign49640_e74716_d_n8;
        locals.var_xp_dn9 = assign49640_e74716_d_n9;
        locals.var_xp_dn10 = assign49640_e74716_d_n10;
        locals.var_xp_dn13 = assign49640_e74716_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign49650_e74731, assign49650_e74731_d_n0, assign49650_e74731_d_n2, assign49650_e74731_d_n4, assign49650_e74731_d_n5, assign49650_e74731_d_n6, assign49650_e74731_d_n7, assign49650_e74731_d_n8, assign49650_e74731_d_n9, assign49650_e74731_d_n10, assign49650_e74731_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1284 != 0.0)) {
        let assign49650_e74729: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign49650_e74729, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign49650_e74731;
        locals.var_xmp_dn0 = assign49650_e74731_d_n0;
        locals.var_xmp_dn2 = assign49650_e74731_d_n2;
        locals.var_xmp_dn4 = assign49650_e74731_d_n4;
        locals.var_xmp_dn5 = assign49650_e74731_d_n5;
        locals.var_xmp_dn6 = assign49650_e74731_d_n6;
        locals.var_xmp_dn7 = assign49650_e74731_d_n7;
        locals.var_xmp_dn8 = assign49650_e74731_d_n8;
        locals.var_xmp_dn9 = assign49650_e74731_d_n9;
        locals.var_xmp_dn10 = assign49650_e74731_d_n10;
        locals.var_xmp_dn13 = assign49650_e74731_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign49660_e74746, assign49660_e74746_d_n0, assign49660_e74746_d_n2, assign49660_e74746_d_n4, assign49660_e74746_d_n5, assign49660_e74746_d_n6, assign49660_e74746_d_n7, assign49660_e74746_d_n8, assign49660_e74746_d_n9, assign49660_e74746_d_n10, assign49660_e74746_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1284 != 0.0)) {
        let assign49660_e74744: f64 = (locals.var_xp + locals.var_xmp);
        (assign49660_e74744, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign49660_e74746;
        locals.var_arg_dn0 = assign49660_e74746_d_n0;
        locals.var_arg_dn2 = assign49660_e74746_d_n2;
        locals.var_arg_dn4 = assign49660_e74746_d_n4;
        locals.var_arg_dn5 = assign49660_e74746_d_n5;
        locals.var_arg_dn6 = assign49660_e74746_d_n6;
        locals.var_arg_dn7 = assign49660_e74746_d_n7;
        locals.var_arg_dn8 = assign49660_e74746_d_n8;
        locals.var_arg_dn9 = assign49660_e74746_d_n9;
        locals.var_arg_dn10 = assign49660_e74746_d_n10;
        locals.var_arg_dn13 = assign49660_e74746_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign49670_e74759, assign49670_e74759_d_n0, assign49670_e74759_d_n2, assign49670_e74759_d_n4, assign49670_e74759_d_n5, assign49670_e74759_d_n6, assign49670_e74759_d_n7, assign49670_e74759_d_n8, assign49670_e74759_d_n9, assign49670_e74759_d_n10, assign49670_e74759_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1284 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign49670_e74759;
        locals.var_dnm_dn0 = assign49670_e74759_d_n0;
        locals.var_dnm_dn2 = assign49670_e74759_d_n2;
        locals.var_dnm_dn4 = assign49670_e74759_d_n4;
        locals.var_dnm_dn5 = assign49670_e74759_d_n5;
        locals.var_dnm_dn6 = assign49670_e74759_d_n6;
        locals.var_dnm_dn7 = assign49670_e74759_d_n7;
        locals.var_dnm_dn8 = assign49670_e74759_d_n8;
        locals.var_dnm_dn9 = assign49670_e74759_d_n9;
        locals.var_dnm_dn10 = assign49670_e74759_d_n10;
        locals.var_dnm_dn13 = assign49670_e74759_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign49680_e74774: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1285 = assign49680_e74774;
        locals.var_guard1285_rv = 0.0;

        let assign49690_e74777: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1286 = assign49690_e74777;
        locals.var_guard1286_rv = 0.0;

        let (assign49700_e74794,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1284 != 0.0)) && (locals.var_guard1285 != 0.0)) && (locals.var_guard1286 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign49700_e74794;
        locals.var_mm_rv = 0.0;

        let assign49710_e74797: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1287 = assign49710_e74797;
        locals.var_guard1287_rv = 0.0;

        let (assign49720_e74817,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1284 != 0.0)) && (locals.var_guard1285 != 0.0)) && (locals.var_guard1286 == 0.0)) && (locals.var_guard1287 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign49720_e74817;
        locals.var_mm_rv = 0.0;

        let assign49730_e74820: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1288 = assign49730_e74820;
        locals.var_guard1288_rv = 0.0;

        let (assign49740_e74843,) = {
    if (((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1284 != 0.0)) && (locals.var_guard1285 != 0.0)) && (locals.var_guard1286 == 0.0)) && (locals.var_guard1287 == 0.0)) && (locals.var_guard1288 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign49740_e74843;
        locals.var_mm_rv = 0.0;

        let assign49750_e74846: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1289 = assign49750_e74846;
        locals.var_guard1289_rv = 0.0;

        let (assign49760_e74872,) = {
    if ((((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1284 != 0.0)) && (locals.var_guard1285 != 0.0)) && (locals.var_guard1286 == 0.0)) && (locals.var_guard1287 == 0.0)) && (locals.var_guard1288 == 0.0)) && (locals.var_guard1289 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign49760_e74872;
        locals.var_mm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_172(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign49770_e74887,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1284 != 0.0)) && (locals.var_guard1285 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign49770_e74887;
        locals.var_m0_rv = 0.0;

        let mut assign49780_loop_guard: usize = 0;
        while {
            let assign49780_cond_e74903: f64 = if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1284 != 0.0)) && (locals.var_guard1285 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign49780_cond_e74903 != 0.0
        } {
            assign49780_loop_guard += 1;
            assert!(assign49780_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign49780_body0_e74919, assign49780_body0_e74919_d_n0, assign49780_body0_e74919_d_n2, assign49780_body0_e74919_d_n4, assign49780_body0_e74919_d_n5, assign49780_body0_e74919_d_n6, assign49780_body0_e74919_d_n7, assign49780_body0_e74919_d_n8, assign49780_body0_e74919_d_n9, assign49780_body0_e74919_d_n10, assign49780_body0_e74919_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1284 != 0.0)) && (locals.var_guard1285 != 0.0)) {
        let assign49780_body0_e74917: f64 = (locals.var_dnm).sqrt();
        (assign49780_body0_e74917, (locals.var_dnm_dn0 / (2.0 * assign49780_body0_e74917)), (locals.var_dnm_dn2 / (2.0 * assign49780_body0_e74917)), (locals.var_dnm_dn4 / (2.0 * assign49780_body0_e74917)), (locals.var_dnm_dn5 / (2.0 * assign49780_body0_e74917)), (locals.var_dnm_dn6 / (2.0 * assign49780_body0_e74917)), (locals.var_dnm_dn7 / (2.0 * assign49780_body0_e74917)), (locals.var_dnm_dn8 / (2.0 * assign49780_body0_e74917)), (locals.var_dnm_dn9 / (2.0 * assign49780_body0_e74917)), (locals.var_dnm_dn10 / (2.0 * assign49780_body0_e74917)), (locals.var_dnm_dn13 / (2.0 * assign49780_body0_e74917)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign49780_body0_e74919;
            locals.var_dnm_dn0 = assign49780_body0_e74919_d_n0;
            locals.var_dnm_dn2 = assign49780_body0_e74919_d_n2;
            locals.var_dnm_dn4 = assign49780_body0_e74919_d_n4;
            locals.var_dnm_dn5 = assign49780_body0_e74919_d_n5;
            locals.var_dnm_dn6 = assign49780_body0_e74919_d_n6;
            locals.var_dnm_dn7 = assign49780_body0_e74919_d_n7;
            locals.var_dnm_dn8 = assign49780_body0_e74919_d_n8;
            locals.var_dnm_dn9 = assign49780_body0_e74919_d_n9;
            locals.var_dnm_dn10 = assign49780_body0_e74919_d_n10;
            locals.var_dnm_dn13 = assign49780_body0_e74919_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign49780_body1_e74936,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1284 != 0.0)) && (locals.var_guard1285 != 0.0)) {
        let assign49780_body1_e74934: f64 = (locals.var_m0 + 1.0);
        (assign49780_body1_e74934,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign49780_body1_e74936;
            locals.var_m0_rv = 0.0;
        }

        let (assign49790_e74963, assign49790_e74963_d_n0, assign49790_e74963_d_n2, assign49790_e74963_d_n4, assign49790_e74963_d_n5, assign49790_e74963_d_n6, assign49790_e74963_d_n7, assign49790_e74963_d_n8, assign49790_e74963_d_n9, assign49790_e74963_d_n10, assign49790_e74963_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1284 != 0.0)) && (locals.var_guard1285 == 0.0)) {
        let (assign49790_e74961, assign49790_e74961_d_n0, assign49790_e74961_d_n2, assign49790_e74961_d_n4, assign49790_e74961_d_n5, assign49790_e74961_d_n6, assign49790_e74961_d_n7, assign49790_e74961_d_n8, assign49790_e74961_d_n9, assign49790_e74961_d_n10, assign49790_e74961_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign49790_e74958: f64 = (2.0 * 2.0);
                let assign49790_e74959: f64 = (1.0 / assign49790_e74958);
                let assign49790_e74960: f64 = (locals.var_dnm).powf(assign49790_e74959);
                (assign49790_e74960, if 0.0 == 0.0 && ((assign49790_e74959) as f64).is_finite() && ((assign49790_e74959) as f64).fract() == 0.0 { if assign49790_e74959 == 0.0 { 0.0 } else { (assign49790_e74959 * ((locals.var_dnm).powf(assign49790_e74959 - 1.0) * locals.var_dnm_dn0)) } } else { (assign49790_e74960 * (assign49790_e74959 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign49790_e74959) as f64).is_finite() && ((assign49790_e74959) as f64).fract() == 0.0 { if assign49790_e74959 == 0.0 { 0.0 } else { (assign49790_e74959 * ((locals.var_dnm).powf(assign49790_e74959 - 1.0) * locals.var_dnm_dn2)) } } else { (assign49790_e74960 * (assign49790_e74959 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign49790_e74959) as f64).is_finite() && ((assign49790_e74959) as f64).fract() == 0.0 { if assign49790_e74959 == 0.0 { 0.0 } else { (assign49790_e74959 * ((locals.var_dnm).powf(assign49790_e74959 - 1.0) * locals.var_dnm_dn4)) } } else { (assign49790_e74960 * (assign49790_e74959 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign49790_e74959) as f64).is_finite() && ((assign49790_e74959) as f64).fract() == 0.0 { if assign49790_e74959 == 0.0 { 0.0 } else { (assign49790_e74959 * ((locals.var_dnm).powf(assign49790_e74959 - 1.0) * locals.var_dnm_dn5)) } } else { (assign49790_e74960 * (assign49790_e74959 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign49790_e74959) as f64).is_finite() && ((assign49790_e74959) as f64).fract() == 0.0 { if assign49790_e74959 == 0.0 { 0.0 } else { (assign49790_e74959 * ((locals.var_dnm).powf(assign49790_e74959 - 1.0) * locals.var_dnm_dn6)) } } else { (assign49790_e74960 * (assign49790_e74959 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign49790_e74959) as f64).is_finite() && ((assign49790_e74959) as f64).fract() == 0.0 { if assign49790_e74959 == 0.0 { 0.0 } else { (assign49790_e74959 * ((locals.var_dnm).powf(assign49790_e74959 - 1.0) * locals.var_dnm_dn7)) } } else { (assign49790_e74960 * (assign49790_e74959 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign49790_e74959) as f64).is_finite() && ((assign49790_e74959) as f64).fract() == 0.0 { if assign49790_e74959 == 0.0 { 0.0 } else { (assign49790_e74959 * ((locals.var_dnm).powf(assign49790_e74959 - 1.0) * locals.var_dnm_dn8)) } } else { (assign49790_e74960 * (assign49790_e74959 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign49790_e74959) as f64).is_finite() && ((assign49790_e74959) as f64).fract() == 0.0 { if assign49790_e74959 == 0.0 { 0.0 } else { (assign49790_e74959 * ((locals.var_dnm).powf(assign49790_e74959 - 1.0) * locals.var_dnm_dn9)) } } else { (assign49790_e74960 * (assign49790_e74959 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign49790_e74959) as f64).is_finite() && ((assign49790_e74959) as f64).fract() == 0.0 { if assign49790_e74959 == 0.0 { 0.0 } else { (assign49790_e74959 * ((locals.var_dnm).powf(assign49790_e74959 - 1.0) * locals.var_dnm_dn10)) } } else { (assign49790_e74960 * (assign49790_e74959 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign49790_e74959) as f64).is_finite() && ((assign49790_e74959) as f64).fract() == 0.0 { if assign49790_e74959 == 0.0 { 0.0 } else { (assign49790_e74959 * ((locals.var_dnm).powf(assign49790_e74959 - 1.0) * locals.var_dnm_dn13)) } } else { (assign49790_e74960 * (assign49790_e74959 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign49790_e74961, assign49790_e74961_d_n0, assign49790_e74961_d_n2, assign49790_e74961_d_n4, assign49790_e74961_d_n5, assign49790_e74961_d_n6, assign49790_e74961_d_n7, assign49790_e74961_d_n8, assign49790_e74961_d_n9, assign49790_e74961_d_n10, assign49790_e74961_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign49790_e74963;
        locals.var_dnm_dn0 = assign49790_e74963_d_n0;
        locals.var_dnm_dn2 = assign49790_e74963_d_n2;
        locals.var_dnm_dn4 = assign49790_e74963_d_n4;
        locals.var_dnm_dn5 = assign49790_e74963_d_n5;
        locals.var_dnm_dn6 = assign49790_e74963_d_n6;
        locals.var_dnm_dn7 = assign49790_e74963_d_n7;
        locals.var_dnm_dn8 = assign49790_e74963_d_n8;
        locals.var_dnm_dn9 = assign49790_e74963_d_n9;
        locals.var_dnm_dn10 = assign49790_e74963_d_n10;
        locals.var_dnm_dn13 = assign49790_e74963_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign49800_e74978, assign49800_e74978_d_n0, assign49800_e74978_d_n2, assign49800_e74978_d_n4, assign49800_e74978_d_n5, assign49800_e74978_d_n6, assign49800_e74978_d_n7, assign49800_e74978_d_n8, assign49800_e74978_d_n9, assign49800_e74978_d_n10, assign49800_e74978_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1284 != 0.0)) {
        let assign49800_e74976: f64 = (1.0 / locals.var_dnm);
        (assign49800_e74976, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign49800_e74978;
        locals.var_dnm_dn0 = assign49800_e74978_d_n0;
        locals.var_dnm_dn2 = assign49800_e74978_d_n2;
        locals.var_dnm_dn4 = assign49800_e74978_d_n4;
        locals.var_dnm_dn5 = assign49800_e74978_d_n5;
        locals.var_dnm_dn6 = assign49800_e74978_d_n6;
        locals.var_dnm_dn7 = assign49800_e74978_d_n7;
        locals.var_dnm_dn8 = assign49800_e74978_d_n8;
        locals.var_dnm_dn9 = assign49800_e74978_d_n9;
        locals.var_dnm_dn10 = assign49800_e74978_d_n10;
        locals.var_dnm_dn13 = assign49800_e74978_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign49810_e74997, assign49810_e74997_d_n0, assign49810_e74997_d_n2, assign49810_e74997_d_n4, assign49810_e74997_d_n5, assign49810_e74997_d_n6, assign49810_e74997_d_n7, assign49810_e74997_d_n8, assign49810_e74997_d_n9, assign49810_e74997_d_n10, assign49810_e74997_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1284 != 0.0)) {
        let assign49810_e74992: f64 = (10.0 * 2.220446049250313e-16);
        let assign49810_e74993: f64 = (locals.var_tmf1 * assign49810_e74992);
        let assign49810_e74995: f64 = (assign49810_e74993 * locals.var_dnm);
        (assign49810_e74995, (((locals.var_tmf1_dn0 * assign49810_e74992) * locals.var_dnm) + (assign49810_e74993 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * assign49810_e74992) * locals.var_dnm) + (assign49810_e74993 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * assign49810_e74992) * locals.var_dnm) + (assign49810_e74993 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * assign49810_e74992) * locals.var_dnm) + (assign49810_e74993 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * assign49810_e74992) * locals.var_dnm) + (assign49810_e74993 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * assign49810_e74992) * locals.var_dnm) + (assign49810_e74993 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * assign49810_e74992) * locals.var_dnm) + (assign49810_e74993 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * assign49810_e74992) * locals.var_dnm) + (assign49810_e74993 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * assign49810_e74992) * locals.var_dnm) + (assign49810_e74993 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * assign49810_e74992) * locals.var_dnm) + (assign49810_e74993 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign49810_e74997;
        locals.var_tmf0_dn0 = assign49810_e74997_d_n0;
        locals.var_tmf0_dn2 = assign49810_e74997_d_n2;
        locals.var_tmf0_dn4 = assign49810_e74997_d_n4;
        locals.var_tmf0_dn5 = assign49810_e74997_d_n5;
        locals.var_tmf0_dn6 = assign49810_e74997_d_n6;
        locals.var_tmf0_dn7 = assign49810_e74997_d_n7;
        locals.var_tmf0_dn8 = assign49810_e74997_d_n8;
        locals.var_tmf0_dn9 = assign49810_e74997_d_n9;
        locals.var_tmf0_dn10 = assign49810_e74997_d_n10;
        locals.var_tmf0_dn13 = assign49810_e74997_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign49820_e75018, assign49820_e75018_d_n0, assign49820_e75018_d_n2, assign49820_e75018_d_n4, assign49820_e75018_d_n5, assign49820_e75018_d_n6, assign49820_e75018_d_n7, assign49820_e75018_d_n8, assign49820_e75018_d_n9, assign49820_e75018_d_n10, assign49820_e75018_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1284 != 0.0)) {
        let assign49820_e75010: f64 = (10.0 * 2.220446049250313e-16);
        let assign49820_e75012: f64 = (assign49820_e75010 * locals.var_xmp);
        let assign49820_e75014: f64 = (assign49820_e75012 * locals.var_dnm);
        let assign49820_e75016: f64 = (assign49820_e75014 / locals.var_arg);
        (assign49820_e75016, ((((((assign49820_e75010 * locals.var_xmp_dn0) * locals.var_dnm) + (assign49820_e75012 * locals.var_dnm_dn0)) * locals.var_arg) - (assign49820_e75014 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((assign49820_e75010 * locals.var_xmp_dn2) * locals.var_dnm) + (assign49820_e75012 * locals.var_dnm_dn2)) * locals.var_arg) - (assign49820_e75014 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((assign49820_e75010 * locals.var_xmp_dn4) * locals.var_dnm) + (assign49820_e75012 * locals.var_dnm_dn4)) * locals.var_arg) - (assign49820_e75014 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((assign49820_e75010 * locals.var_xmp_dn5) * locals.var_dnm) + (assign49820_e75012 * locals.var_dnm_dn5)) * locals.var_arg) - (assign49820_e75014 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((assign49820_e75010 * locals.var_xmp_dn6) * locals.var_dnm) + (assign49820_e75012 * locals.var_dnm_dn6)) * locals.var_arg) - (assign49820_e75014 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((assign49820_e75010 * locals.var_xmp_dn7) * locals.var_dnm) + (assign49820_e75012 * locals.var_dnm_dn7)) * locals.var_arg) - (assign49820_e75014 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((assign49820_e75010 * locals.var_xmp_dn8) * locals.var_dnm) + (assign49820_e75012 * locals.var_dnm_dn8)) * locals.var_arg) - (assign49820_e75014 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((assign49820_e75010 * locals.var_xmp_dn9) * locals.var_dnm) + (assign49820_e75012 * locals.var_dnm_dn9)) * locals.var_arg) - (assign49820_e75014 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((assign49820_e75010 * locals.var_xmp_dn10) * locals.var_dnm) + (assign49820_e75012 * locals.var_dnm_dn10)) * locals.var_arg) - (assign49820_e75014 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((assign49820_e75010 * locals.var_xmp_dn13) * locals.var_dnm) + (assign49820_e75012 * locals.var_dnm_dn13)) * locals.var_arg) - (assign49820_e75014 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign49820_e75018;
        locals.var_t0_dn0 = assign49820_e75018_d_n0;
        locals.var_t0_dn2 = assign49820_e75018_d_n2;
        locals.var_t0_dn4 = assign49820_e75018_d_n4;
        locals.var_t0_dn5 = assign49820_e75018_d_n5;
        locals.var_t0_dn6 = assign49820_e75018_d_n6;
        locals.var_t0_dn7 = assign49820_e75018_d_n7;
        locals.var_t0_dn8 = assign49820_e75018_d_n8;
        locals.var_t0_dn9 = assign49820_e75018_d_n9;
        locals.var_t0_dn10 = assign49820_e75018_d_n10;
        locals.var_t0_dn13 = assign49820_e75018_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign49830_e75039, assign49830_e75039_d_n0, assign49830_e75039_d_n2, assign49830_e75039_d_n4, assign49830_e75039_d_n5, assign49830_e75039_d_n6, assign49830_e75039_d_n7, assign49830_e75039_d_n8, assign49830_e75039_d_n9, assign49830_e75039_d_n10, assign49830_e75039_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1284 != 0.0)) {
        let assign49830_e75031: f64 = (10.0 * 2.220446049250313e-16);
        let assign49830_e75034: f64 = (10.0 * 2.220446049250313e-16);
        let assign49830_e75035: f64 = (assign49830_e75031 + assign49830_e75034);
        let assign49830_e75037: f64 = (assign49830_e75035 - locals.var_tmf0);
        (assign49830_e75037, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn13),)
    } else {
        (locals.var_pzadd, locals.var_pzadd_dn0, locals.var_pzadd_dn2, locals.var_pzadd_dn4, locals.var_pzadd_dn5, locals.var_pzadd_dn6, locals.var_pzadd_dn7, locals.var_pzadd_dn8, locals.var_pzadd_dn9, locals.var_pzadd_dn10, locals.var_pzadd_dn13,)
    }
};
        locals.var_pzadd = assign49830_e75039;
        locals.var_pzadd_dn0 = assign49830_e75039_d_n0;
        locals.var_pzadd_dn2 = assign49830_e75039_d_n2;
        locals.var_pzadd_dn4 = assign49830_e75039_d_n4;
        locals.var_pzadd_dn5 = assign49830_e75039_d_n5;
        locals.var_pzadd_dn6 = assign49830_e75039_d_n6;
        locals.var_pzadd_dn7 = assign49830_e75039_d_n7;
        locals.var_pzadd_dn8 = assign49830_e75039_d_n8;
        locals.var_pzadd_dn9 = assign49830_e75039_d_n9;
        locals.var_pzadd_dn10 = assign49830_e75039_d_n10;
        locals.var_pzadd_dn13 = assign49830_e75039_d_n13;
        locals.var_pzadd_rv = 0.0;

        let (assign49840_e75052, assign49840_e75052_d_n0, assign49840_e75052_d_n2, assign49840_e75052_d_n4, assign49840_e75052_d_n5, assign49840_e75052_d_n6, assign49840_e75052_d_n7, assign49840_e75052_d_n8, assign49840_e75052_d_n9, assign49840_e75052_d_n10, assign49840_e75052_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1284 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign49840_e75052;
        locals.var_t0_dn0 = assign49840_e75052_d_n0;
        locals.var_t0_dn2 = assign49840_e75052_d_n2;
        locals.var_t0_dn4 = assign49840_e75052_d_n4;
        locals.var_t0_dn5 = assign49840_e75052_d_n5;
        locals.var_t0_dn6 = assign49840_e75052_d_n6;
        locals.var_t0_dn7 = assign49840_e75052_d_n7;
        locals.var_t0_dn8 = assign49840_e75052_d_n8;
        locals.var_t0_dn9 = assign49840_e75052_d_n9;
        locals.var_t0_dn10 = assign49840_e75052_d_n10;
        locals.var_t0_dn13 = assign49840_e75052_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign49850_e75066, assign49850_e75066_d_n0, assign49850_e75066_d_n2, assign49850_e75066_d_n4, assign49850_e75066_d_n5, assign49850_e75066_d_n6, assign49850_e75066_d_n7, assign49850_e75066_d_n8, assign49850_e75066_d_n9, assign49850_e75066_d_n10, assign49850_e75066_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1284 == 0.0)) {
        (locals.var_pzadd, locals.var_pzadd_dn0, locals.var_pzadd_dn2, locals.var_pzadd_dn4, locals.var_pzadd_dn5, locals.var_pzadd_dn6, locals.var_pzadd_dn7, locals.var_pzadd_dn8, locals.var_pzadd_dn9, locals.var_pzadd_dn10, locals.var_pzadd_dn13,)
    } else {
        (locals.var_pzadd, locals.var_pzadd_dn0, locals.var_pzadd_dn2, locals.var_pzadd_dn4, locals.var_pzadd_dn5, locals.var_pzadd_dn6, locals.var_pzadd_dn7, locals.var_pzadd_dn8, locals.var_pzadd_dn9, locals.var_pzadd_dn10, locals.var_pzadd_dn13,)
    }
};
        locals.var_pzadd = assign49850_e75066;
        locals.var_pzadd_dn0 = assign49850_e75066_d_n0;
        locals.var_pzadd_dn2 = assign49850_e75066_d_n2;
        locals.var_pzadd_dn4 = assign49850_e75066_d_n4;
        locals.var_pzadd_dn5 = assign49850_e75066_d_n5;
        locals.var_pzadd_dn6 = assign49850_e75066_d_n6;
        locals.var_pzadd_dn7 = assign49850_e75066_d_n7;
        locals.var_pzadd_dn8 = assign49850_e75066_d_n8;
        locals.var_pzadd_dn9 = assign49850_e75066_d_n9;
        locals.var_pzadd_dn10 = assign49850_e75066_d_n10;
        locals.var_pzadd_dn13 = assign49850_e75066_d_n13;
        locals.var_pzadd_rv = 0.0;

        let (assign49860_e75080, assign49860_e75080_d_n0, assign49860_e75080_d_n2, assign49860_e75080_d_n4, assign49860_e75080_d_n5, assign49860_e75080_d_n6, assign49860_e75080_d_n7, assign49860_e75080_d_n8, assign49860_e75080_d_n9, assign49860_e75080_d_n10, assign49860_e75080_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1284 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign49860_e75080;
        locals.var_t0_dn0 = assign49860_e75080_d_n0;
        locals.var_t0_dn2 = assign49860_e75080_d_n2;
        locals.var_t0_dn4 = assign49860_e75080_d_n4;
        locals.var_t0_dn5 = assign49860_e75080_d_n5;
        locals.var_t0_dn6 = assign49860_e75080_d_n6;
        locals.var_t0_dn7 = assign49860_e75080_d_n7;
        locals.var_t0_dn8 = assign49860_e75080_d_n8;
        locals.var_t0_dn9 = assign49860_e75080_d_n9;
        locals.var_t0_dn10 = assign49860_e75080_d_n10;
        locals.var_t0_dn13 = assign49860_e75080_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign49870_e75093, assign49870_e75093_d_n0, assign49870_e75093_d_n2, assign49870_e75093_d_n4, assign49870_e75093_d_n5, assign49870_e75093_d_n6, assign49870_e75093_d_n7, assign49870_e75093_d_n8, assign49870_e75093_d_n9, assign49870_e75093_d_n10, assign49870_e75093_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign49870_e75091: f64 = (locals.var_ps0 + locals.var_pzadd);
        (assign49870_e75091, (locals.var_ps0_dn0 + locals.var_pzadd_dn0), (locals.var_ps0_dn2 + locals.var_pzadd_dn2), (locals.var_ps0_dn4 + locals.var_pzadd_dn4), (locals.var_ps0_dn5 + locals.var_pzadd_dn5), (locals.var_ps0_dn6 + locals.var_pzadd_dn6), (locals.var_ps0_dn7 + locals.var_pzadd_dn7), (locals.var_ps0_dn8 + locals.var_pzadd_dn8), (locals.var_ps0_dn9 + locals.var_pzadd_dn9), (locals.var_ps0_dn10 + locals.var_pzadd_dn10), (locals.var_ps0_dn13 + locals.var_pzadd_dn13),)
    } else {
        (locals.var_ps0z, locals.var_ps0z_dn0, locals.var_ps0z_dn2, locals.var_ps0z_dn4, locals.var_ps0z_dn5, locals.var_ps0z_dn6, locals.var_ps0z_dn7, locals.var_ps0z_dn8, locals.var_ps0z_dn9, locals.var_ps0z_dn10, locals.var_ps0z_dn13,)
    }
};
        locals.var_ps0z = assign49870_e75093;
        locals.var_ps0z_dn0 = assign49870_e75093_d_n0;
        locals.var_ps0z_dn2 = assign49870_e75093_d_n2;
        locals.var_ps0z_dn4 = assign49870_e75093_d_n4;
        locals.var_ps0z_dn5 = assign49870_e75093_d_n5;
        locals.var_ps0z_dn6 = assign49870_e75093_d_n6;
        locals.var_ps0z_dn7 = assign49870_e75093_d_n7;
        locals.var_ps0z_dn8 = assign49870_e75093_d_n8;
        locals.var_ps0z_dn9 = assign49870_e75093_d_n9;
        locals.var_ps0z_dn10 = assign49870_e75093_d_n10;
        locals.var_ps0z_dn13 = assign49870_e75093_d_n13;
        locals.var_ps0z_rv = 0.0;

        let assign49880_e75096: f64 = (locals.var_ps0z - locals.var_vds_maxb0__blk1087);
        let assign49880_e75099: f64 = p.p403;
        let assign49880_e75104: f64 = if ((assign49880_e75096 < assign49880_e75099) && (p.p403 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1290 = assign49880_e75104;
        locals.var_guard1290_rv = 0.0;

        let (assign49890_e75123, assign49890_e75123_d_n0, assign49890_e75123_d_n2, assign49890_e75123_d_n4, assign49890_e75123_d_n5, assign49890_e75123_d_n6, assign49890_e75123_d_n7, assign49890_e75123_d_n8, assign49890_e75123_d_n9, assign49890_e75123_d_n10, assign49890_e75123_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1290 != 0.0)) {
        let assign49890_e75117: f64 = p.p403;
        let assign49890_e75120: f64 = (locals.var_ps0z - locals.var_vds_maxb0__blk1087);
        let assign49890_e75121: f64 = (assign49890_e75117 - assign49890_e75120);
        (assign49890_e75121, (-locals.var_ps0z_dn0), (-locals.var_ps0z_dn2), (-locals.var_ps0z_dn4), (-locals.var_ps0z_dn5), (-locals.var_ps0z_dn6), (-locals.var_ps0z_dn7), (-locals.var_ps0z_dn8), (-locals.var_ps0z_dn9), (-locals.var_ps0z_dn10), (-locals.var_ps0z_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign49890_e75123;
        locals.var_tmf1_dn0 = assign49890_e75123_d_n0;
        locals.var_tmf1_dn2 = assign49890_e75123_d_n2;
        locals.var_tmf1_dn4 = assign49890_e75123_d_n4;
        locals.var_tmf1_dn5 = assign49890_e75123_d_n5;
        locals.var_tmf1_dn6 = assign49890_e75123_d_n6;
        locals.var_tmf1_dn7 = assign49890_e75123_d_n7;
        locals.var_tmf1_dn8 = assign49890_e75123_d_n8;
        locals.var_tmf1_dn9 = assign49890_e75123_d_n9;
        locals.var_tmf1_dn10 = assign49890_e75123_d_n10;
        locals.var_tmf1_dn13 = assign49890_e75123_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign49900_e75138, assign49900_e75138_d_n0, assign49900_e75138_d_n2, assign49900_e75138_d_n4, assign49900_e75138_d_n5, assign49900_e75138_d_n6, assign49900_e75138_d_n7, assign49900_e75138_d_n8, assign49900_e75138_d_n9, assign49900_e75138_d_n10, assign49900_e75138_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1290 != 0.0)) {
        let assign49900_e75136: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign49900_e75136, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign49900_e75138;
        locals.var_x2_dn0 = assign49900_e75138_d_n0;
        locals.var_x2_dn2 = assign49900_e75138_d_n2;
        locals.var_x2_dn4 = assign49900_e75138_d_n4;
        locals.var_x2_dn5 = assign49900_e75138_d_n5;
        locals.var_x2_dn6 = assign49900_e75138_d_n6;
        locals.var_x2_dn7 = assign49900_e75138_d_n7;
        locals.var_x2_dn8 = assign49900_e75138_d_n8;
        locals.var_x2_dn9 = assign49900_e75138_d_n9;
        locals.var_x2_dn10 = assign49900_e75138_d_n10;
        locals.var_x2_dn13 = assign49900_e75138_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign49910_e75153, assign49910_e75153_d_n0, assign49910_e75153_d_n2, assign49910_e75153_d_n4, assign49910_e75153_d_n5, assign49910_e75153_d_n6, assign49910_e75153_d_n7, assign49910_e75153_d_n8, assign49910_e75153_d_n9, assign49910_e75153_d_n10, assign49910_e75153_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1290 != 0.0)) {
        let assign49910_e75151: f64 = (p.p403 * p.p403);
        (assign49910_e75151, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign49910_e75153;
        locals.var_xmax2_dn0 = assign49910_e75153_d_n0;
        locals.var_xmax2_dn2 = assign49910_e75153_d_n2;
        locals.var_xmax2_dn4 = assign49910_e75153_d_n4;
        locals.var_xmax2_dn5 = assign49910_e75153_d_n5;
        locals.var_xmax2_dn6 = assign49910_e75153_d_n6;
        locals.var_xmax2_dn7 = assign49910_e75153_d_n7;
        locals.var_xmax2_dn8 = assign49910_e75153_d_n8;
        locals.var_xmax2_dn9 = assign49910_e75153_d_n9;
        locals.var_xmax2_dn10 = assign49910_e75153_d_n10;
        locals.var_xmax2_dn13 = assign49910_e75153_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign49920_e75166, assign49920_e75166_d_n0, assign49920_e75166_d_n2, assign49920_e75166_d_n4, assign49920_e75166_d_n5, assign49920_e75166_d_n6, assign49920_e75166_d_n7, assign49920_e75166_d_n8, assign49920_e75166_d_n9, assign49920_e75166_d_n10, assign49920_e75166_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1290 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign49920_e75166;
        locals.var_xp_dn0 = assign49920_e75166_d_n0;
        locals.var_xp_dn2 = assign49920_e75166_d_n2;
        locals.var_xp_dn4 = assign49920_e75166_d_n4;
        locals.var_xp_dn5 = assign49920_e75166_d_n5;
        locals.var_xp_dn6 = assign49920_e75166_d_n6;
        locals.var_xp_dn7 = assign49920_e75166_d_n7;
        locals.var_xp_dn8 = assign49920_e75166_d_n8;
        locals.var_xp_dn9 = assign49920_e75166_d_n9;
        locals.var_xp_dn10 = assign49920_e75166_d_n10;
        locals.var_xp_dn13 = assign49920_e75166_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign49930_e75179, assign49930_e75179_d_n0, assign49930_e75179_d_n2, assign49930_e75179_d_n4, assign49930_e75179_d_n5, assign49930_e75179_d_n6, assign49930_e75179_d_n7, assign49930_e75179_d_n8, assign49930_e75179_d_n9, assign49930_e75179_d_n10, assign49930_e75179_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1290 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign49930_e75179;
        locals.var_xmp_dn0 = assign49930_e75179_d_n0;
        locals.var_xmp_dn2 = assign49930_e75179_d_n2;
        locals.var_xmp_dn4 = assign49930_e75179_d_n4;
        locals.var_xmp_dn5 = assign49930_e75179_d_n5;
        locals.var_xmp_dn6 = assign49930_e75179_d_n6;
        locals.var_xmp_dn7 = assign49930_e75179_d_n7;
        locals.var_xmp_dn8 = assign49930_e75179_d_n8;
        locals.var_xmp_dn9 = assign49930_e75179_d_n9;
        locals.var_xmp_dn10 = assign49930_e75179_d_n10;
        locals.var_xmp_dn13 = assign49930_e75179_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign49940_e75192,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1290 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign49940_e75192;
        locals.var_m0_rv = 0.0;

        let (assign49950_e75205,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1290 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign49950_e75205;
        locals.var_mm_rv = 0.0;

        let (assign49960_e75218, assign49960_e75218_d_n0, assign49960_e75218_d_n2, assign49960_e75218_d_n4, assign49960_e75218_d_n5, assign49960_e75218_d_n6, assign49960_e75218_d_n7, assign49960_e75218_d_n8, assign49960_e75218_d_n9, assign49960_e75218_d_n10, assign49960_e75218_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1290 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign49960_e75218;
        locals.var_arg_dn0 = assign49960_e75218_d_n0;
        locals.var_arg_dn2 = assign49960_e75218_d_n2;
        locals.var_arg_dn4 = assign49960_e75218_d_n4;
        locals.var_arg_dn5 = assign49960_e75218_d_n5;
        locals.var_arg_dn6 = assign49960_e75218_d_n6;
        locals.var_arg_dn7 = assign49960_e75218_d_n7;
        locals.var_arg_dn8 = assign49960_e75218_d_n8;
        locals.var_arg_dn9 = assign49960_e75218_d_n9;
        locals.var_arg_dn10 = assign49960_e75218_d_n10;
        locals.var_arg_dn13 = assign49960_e75218_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign49970_e75231, assign49970_e75231_d_n0, assign49970_e75231_d_n2, assign49970_e75231_d_n4, assign49970_e75231_d_n5, assign49970_e75231_d_n6, assign49970_e75231_d_n7, assign49970_e75231_d_n8, assign49970_e75231_d_n9, assign49970_e75231_d_n10, assign49970_e75231_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1290 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign49970_e75231;
        locals.var_dnm_dn0 = assign49970_e75231_d_n0;
        locals.var_dnm_dn2 = assign49970_e75231_d_n2;
        locals.var_dnm_dn4 = assign49970_e75231_d_n4;
        locals.var_dnm_dn5 = assign49970_e75231_d_n5;
        locals.var_dnm_dn6 = assign49970_e75231_d_n6;
        locals.var_dnm_dn7 = assign49970_e75231_d_n7;
        locals.var_dnm_dn8 = assign49970_e75231_d_n8;
        locals.var_dnm_dn9 = assign49970_e75231_d_n9;
        locals.var_dnm_dn10 = assign49970_e75231_d_n10;
        locals.var_dnm_dn13 = assign49970_e75231_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign49980_e75246, assign49980_e75246_d_n0, assign49980_e75246_d_n2, assign49980_e75246_d_n4, assign49980_e75246_d_n5, assign49980_e75246_d_n6, assign49980_e75246_d_n7, assign49980_e75246_d_n8, assign49980_e75246_d_n9, assign49980_e75246_d_n10, assign49980_e75246_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1290 != 0.0)) {
        let assign49980_e75244: f64 = (locals.var_xp * locals.var_x2);
        (assign49980_e75244, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign49980_e75246;
        locals.var_xp_dn0 = assign49980_e75246_d_n0;
        locals.var_xp_dn2 = assign49980_e75246_d_n2;
        locals.var_xp_dn4 = assign49980_e75246_d_n4;
        locals.var_xp_dn5 = assign49980_e75246_d_n5;
        locals.var_xp_dn6 = assign49980_e75246_d_n6;
        locals.var_xp_dn7 = assign49980_e75246_d_n7;
        locals.var_xp_dn8 = assign49980_e75246_d_n8;
        locals.var_xp_dn9 = assign49980_e75246_d_n9;
        locals.var_xp_dn10 = assign49980_e75246_d_n10;
        locals.var_xp_dn13 = assign49980_e75246_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign49990_e75261, assign49990_e75261_d_n0, assign49990_e75261_d_n2, assign49990_e75261_d_n4, assign49990_e75261_d_n5, assign49990_e75261_d_n6, assign49990_e75261_d_n7, assign49990_e75261_d_n8, assign49990_e75261_d_n9, assign49990_e75261_d_n10, assign49990_e75261_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1290 != 0.0)) {
        let assign49990_e75259: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign49990_e75259, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign49990_e75261;
        locals.var_xmp_dn0 = assign49990_e75261_d_n0;
        locals.var_xmp_dn2 = assign49990_e75261_d_n2;
        locals.var_xmp_dn4 = assign49990_e75261_d_n4;
        locals.var_xmp_dn5 = assign49990_e75261_d_n5;
        locals.var_xmp_dn6 = assign49990_e75261_d_n6;
        locals.var_xmp_dn7 = assign49990_e75261_d_n7;
        locals.var_xmp_dn8 = assign49990_e75261_d_n8;
        locals.var_xmp_dn9 = assign49990_e75261_d_n9;
        locals.var_xmp_dn10 = assign49990_e75261_d_n10;
        locals.var_xmp_dn13 = assign49990_e75261_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign50000_e75276, assign50000_e75276_d_n0, assign50000_e75276_d_n2, assign50000_e75276_d_n4, assign50000_e75276_d_n5, assign50000_e75276_d_n6, assign50000_e75276_d_n7, assign50000_e75276_d_n8, assign50000_e75276_d_n9, assign50000_e75276_d_n10, assign50000_e75276_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1290 != 0.0)) {
        let assign50000_e75274: f64 = (locals.var_xp * locals.var_x2);
        (assign50000_e75274, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign50000_e75276;
        locals.var_xp_dn0 = assign50000_e75276_d_n0;
        locals.var_xp_dn2 = assign50000_e75276_d_n2;
        locals.var_xp_dn4 = assign50000_e75276_d_n4;
        locals.var_xp_dn5 = assign50000_e75276_d_n5;
        locals.var_xp_dn6 = assign50000_e75276_d_n6;
        locals.var_xp_dn7 = assign50000_e75276_d_n7;
        locals.var_xp_dn8 = assign50000_e75276_d_n8;
        locals.var_xp_dn9 = assign50000_e75276_d_n9;
        locals.var_xp_dn10 = assign50000_e75276_d_n10;
        locals.var_xp_dn13 = assign50000_e75276_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign50010_e75291, assign50010_e75291_d_n0, assign50010_e75291_d_n2, assign50010_e75291_d_n4, assign50010_e75291_d_n5, assign50010_e75291_d_n6, assign50010_e75291_d_n7, assign50010_e75291_d_n8, assign50010_e75291_d_n9, assign50010_e75291_d_n10, assign50010_e75291_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1290 != 0.0)) {
        let assign50010_e75289: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign50010_e75289, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign50010_e75291;
        locals.var_xmp_dn0 = assign50010_e75291_d_n0;
        locals.var_xmp_dn2 = assign50010_e75291_d_n2;
        locals.var_xmp_dn4 = assign50010_e75291_d_n4;
        locals.var_xmp_dn5 = assign50010_e75291_d_n5;
        locals.var_xmp_dn6 = assign50010_e75291_d_n6;
        locals.var_xmp_dn7 = assign50010_e75291_d_n7;
        locals.var_xmp_dn8 = assign50010_e75291_d_n8;
        locals.var_xmp_dn9 = assign50010_e75291_d_n9;
        locals.var_xmp_dn10 = assign50010_e75291_d_n10;
        locals.var_xmp_dn13 = assign50010_e75291_d_n13;
        locals.var_xmp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_173(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign50020_e75306, assign50020_e75306_d_n0, assign50020_e75306_d_n2, assign50020_e75306_d_n4, assign50020_e75306_d_n5, assign50020_e75306_d_n6, assign50020_e75306_d_n7, assign50020_e75306_d_n8, assign50020_e75306_d_n9, assign50020_e75306_d_n10, assign50020_e75306_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1290 != 0.0)) {
        let assign50020_e75304: f64 = (locals.var_xp * locals.var_x2);
        (assign50020_e75304, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign50020_e75306;
        locals.var_xp_dn0 = assign50020_e75306_d_n0;
        locals.var_xp_dn2 = assign50020_e75306_d_n2;
        locals.var_xp_dn4 = assign50020_e75306_d_n4;
        locals.var_xp_dn5 = assign50020_e75306_d_n5;
        locals.var_xp_dn6 = assign50020_e75306_d_n6;
        locals.var_xp_dn7 = assign50020_e75306_d_n7;
        locals.var_xp_dn8 = assign50020_e75306_d_n8;
        locals.var_xp_dn9 = assign50020_e75306_d_n9;
        locals.var_xp_dn10 = assign50020_e75306_d_n10;
        locals.var_xp_dn13 = assign50020_e75306_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign50030_e75321, assign50030_e75321_d_n0, assign50030_e75321_d_n2, assign50030_e75321_d_n4, assign50030_e75321_d_n5, assign50030_e75321_d_n6, assign50030_e75321_d_n7, assign50030_e75321_d_n8, assign50030_e75321_d_n9, assign50030_e75321_d_n10, assign50030_e75321_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1290 != 0.0)) {
        let assign50030_e75319: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign50030_e75319, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign50030_e75321;
        locals.var_xmp_dn0 = assign50030_e75321_d_n0;
        locals.var_xmp_dn2 = assign50030_e75321_d_n2;
        locals.var_xmp_dn4 = assign50030_e75321_d_n4;
        locals.var_xmp_dn5 = assign50030_e75321_d_n5;
        locals.var_xmp_dn6 = assign50030_e75321_d_n6;
        locals.var_xmp_dn7 = assign50030_e75321_d_n7;
        locals.var_xmp_dn8 = assign50030_e75321_d_n8;
        locals.var_xmp_dn9 = assign50030_e75321_d_n9;
        locals.var_xmp_dn10 = assign50030_e75321_d_n10;
        locals.var_xmp_dn13 = assign50030_e75321_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign50040_e75336, assign50040_e75336_d_n0, assign50040_e75336_d_n2, assign50040_e75336_d_n4, assign50040_e75336_d_n5, assign50040_e75336_d_n6, assign50040_e75336_d_n7, assign50040_e75336_d_n8, assign50040_e75336_d_n9, assign50040_e75336_d_n10, assign50040_e75336_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1290 != 0.0)) {
        let assign50040_e75334: f64 = (locals.var_xp * locals.var_x2);
        (assign50040_e75334, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign50040_e75336;
        locals.var_xp_dn0 = assign50040_e75336_d_n0;
        locals.var_xp_dn2 = assign50040_e75336_d_n2;
        locals.var_xp_dn4 = assign50040_e75336_d_n4;
        locals.var_xp_dn5 = assign50040_e75336_d_n5;
        locals.var_xp_dn6 = assign50040_e75336_d_n6;
        locals.var_xp_dn7 = assign50040_e75336_d_n7;
        locals.var_xp_dn8 = assign50040_e75336_d_n8;
        locals.var_xp_dn9 = assign50040_e75336_d_n9;
        locals.var_xp_dn10 = assign50040_e75336_d_n10;
        locals.var_xp_dn13 = assign50040_e75336_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign50050_e75351, assign50050_e75351_d_n0, assign50050_e75351_d_n2, assign50050_e75351_d_n4, assign50050_e75351_d_n5, assign50050_e75351_d_n6, assign50050_e75351_d_n7, assign50050_e75351_d_n8, assign50050_e75351_d_n9, assign50050_e75351_d_n10, assign50050_e75351_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1290 != 0.0)) {
        let assign50050_e75349: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign50050_e75349, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign50050_e75351;
        locals.var_xmp_dn0 = assign50050_e75351_d_n0;
        locals.var_xmp_dn2 = assign50050_e75351_d_n2;
        locals.var_xmp_dn4 = assign50050_e75351_d_n4;
        locals.var_xmp_dn5 = assign50050_e75351_d_n5;
        locals.var_xmp_dn6 = assign50050_e75351_d_n6;
        locals.var_xmp_dn7 = assign50050_e75351_d_n7;
        locals.var_xmp_dn8 = assign50050_e75351_d_n8;
        locals.var_xmp_dn9 = assign50050_e75351_d_n9;
        locals.var_xmp_dn10 = assign50050_e75351_d_n10;
        locals.var_xmp_dn13 = assign50050_e75351_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign50060_e75366, assign50060_e75366_d_n0, assign50060_e75366_d_n2, assign50060_e75366_d_n4, assign50060_e75366_d_n5, assign50060_e75366_d_n6, assign50060_e75366_d_n7, assign50060_e75366_d_n8, assign50060_e75366_d_n9, assign50060_e75366_d_n10, assign50060_e75366_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1290 != 0.0)) {
        let assign50060_e75364: f64 = (locals.var_xp * locals.var_x2);
        (assign50060_e75364, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign50060_e75366;
        locals.var_xp_dn0 = assign50060_e75366_d_n0;
        locals.var_xp_dn2 = assign50060_e75366_d_n2;
        locals.var_xp_dn4 = assign50060_e75366_d_n4;
        locals.var_xp_dn5 = assign50060_e75366_d_n5;
        locals.var_xp_dn6 = assign50060_e75366_d_n6;
        locals.var_xp_dn7 = assign50060_e75366_d_n7;
        locals.var_xp_dn8 = assign50060_e75366_d_n8;
        locals.var_xp_dn9 = assign50060_e75366_d_n9;
        locals.var_xp_dn10 = assign50060_e75366_d_n10;
        locals.var_xp_dn13 = assign50060_e75366_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign50070_e75381, assign50070_e75381_d_n0, assign50070_e75381_d_n2, assign50070_e75381_d_n4, assign50070_e75381_d_n5, assign50070_e75381_d_n6, assign50070_e75381_d_n7, assign50070_e75381_d_n8, assign50070_e75381_d_n9, assign50070_e75381_d_n10, assign50070_e75381_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1290 != 0.0)) {
        let assign50070_e75379: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign50070_e75379, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign50070_e75381;
        locals.var_xmp_dn0 = assign50070_e75381_d_n0;
        locals.var_xmp_dn2 = assign50070_e75381_d_n2;
        locals.var_xmp_dn4 = assign50070_e75381_d_n4;
        locals.var_xmp_dn5 = assign50070_e75381_d_n5;
        locals.var_xmp_dn6 = assign50070_e75381_d_n6;
        locals.var_xmp_dn7 = assign50070_e75381_d_n7;
        locals.var_xmp_dn8 = assign50070_e75381_d_n8;
        locals.var_xmp_dn9 = assign50070_e75381_d_n9;
        locals.var_xmp_dn10 = assign50070_e75381_d_n10;
        locals.var_xmp_dn13 = assign50070_e75381_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign50080_e75396, assign50080_e75396_d_n0, assign50080_e75396_d_n2, assign50080_e75396_d_n4, assign50080_e75396_d_n5, assign50080_e75396_d_n6, assign50080_e75396_d_n7, assign50080_e75396_d_n8, assign50080_e75396_d_n9, assign50080_e75396_d_n10, assign50080_e75396_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1290 != 0.0)) {
        let assign50080_e75394: f64 = (locals.var_xp * locals.var_x2);
        (assign50080_e75394, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign50080_e75396;
        locals.var_xp_dn0 = assign50080_e75396_d_n0;
        locals.var_xp_dn2 = assign50080_e75396_d_n2;
        locals.var_xp_dn4 = assign50080_e75396_d_n4;
        locals.var_xp_dn5 = assign50080_e75396_d_n5;
        locals.var_xp_dn6 = assign50080_e75396_d_n6;
        locals.var_xp_dn7 = assign50080_e75396_d_n7;
        locals.var_xp_dn8 = assign50080_e75396_d_n8;
        locals.var_xp_dn9 = assign50080_e75396_d_n9;
        locals.var_xp_dn10 = assign50080_e75396_d_n10;
        locals.var_xp_dn13 = assign50080_e75396_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign50090_e75411, assign50090_e75411_d_n0, assign50090_e75411_d_n2, assign50090_e75411_d_n4, assign50090_e75411_d_n5, assign50090_e75411_d_n6, assign50090_e75411_d_n7, assign50090_e75411_d_n8, assign50090_e75411_d_n9, assign50090_e75411_d_n10, assign50090_e75411_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1290 != 0.0)) {
        let assign50090_e75409: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign50090_e75409, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign50090_e75411;
        locals.var_xmp_dn0 = assign50090_e75411_d_n0;
        locals.var_xmp_dn2 = assign50090_e75411_d_n2;
        locals.var_xmp_dn4 = assign50090_e75411_d_n4;
        locals.var_xmp_dn5 = assign50090_e75411_d_n5;
        locals.var_xmp_dn6 = assign50090_e75411_d_n6;
        locals.var_xmp_dn7 = assign50090_e75411_d_n7;
        locals.var_xmp_dn8 = assign50090_e75411_d_n8;
        locals.var_xmp_dn9 = assign50090_e75411_d_n9;
        locals.var_xmp_dn10 = assign50090_e75411_d_n10;
        locals.var_xmp_dn13 = assign50090_e75411_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign50100_e75426, assign50100_e75426_d_n0, assign50100_e75426_d_n2, assign50100_e75426_d_n4, assign50100_e75426_d_n5, assign50100_e75426_d_n6, assign50100_e75426_d_n7, assign50100_e75426_d_n8, assign50100_e75426_d_n9, assign50100_e75426_d_n10, assign50100_e75426_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1290 != 0.0)) {
        let assign50100_e75424: f64 = (locals.var_xp + locals.var_xmp);
        (assign50100_e75424, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign50100_e75426;
        locals.var_arg_dn0 = assign50100_e75426_d_n0;
        locals.var_arg_dn2 = assign50100_e75426_d_n2;
        locals.var_arg_dn4 = assign50100_e75426_d_n4;
        locals.var_arg_dn5 = assign50100_e75426_d_n5;
        locals.var_arg_dn6 = assign50100_e75426_d_n6;
        locals.var_arg_dn7 = assign50100_e75426_d_n7;
        locals.var_arg_dn8 = assign50100_e75426_d_n8;
        locals.var_arg_dn9 = assign50100_e75426_d_n9;
        locals.var_arg_dn10 = assign50100_e75426_d_n10;
        locals.var_arg_dn13 = assign50100_e75426_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign50110_e75439, assign50110_e75439_d_n0, assign50110_e75439_d_n2, assign50110_e75439_d_n4, assign50110_e75439_d_n5, assign50110_e75439_d_n6, assign50110_e75439_d_n7, assign50110_e75439_d_n8, assign50110_e75439_d_n9, assign50110_e75439_d_n10, assign50110_e75439_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1290 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign50110_e75439;
        locals.var_dnm_dn0 = assign50110_e75439_d_n0;
        locals.var_dnm_dn2 = assign50110_e75439_d_n2;
        locals.var_dnm_dn4 = assign50110_e75439_d_n4;
        locals.var_dnm_dn5 = assign50110_e75439_d_n5;
        locals.var_dnm_dn6 = assign50110_e75439_d_n6;
        locals.var_dnm_dn7 = assign50110_e75439_d_n7;
        locals.var_dnm_dn8 = assign50110_e75439_d_n8;
        locals.var_dnm_dn9 = assign50110_e75439_d_n9;
        locals.var_dnm_dn10 = assign50110_e75439_d_n10;
        locals.var_dnm_dn13 = assign50110_e75439_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign50120_e75454: f64 = if ((((6.0 == 1.0) || (6.0 == 2.0)) || (6.0 == 4.0)) || (6.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1291 = assign50120_e75454;
        locals.var_guard1291_rv = 0.0;

        let assign50130_e75457: f64 = if 6.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1292 = assign50130_e75457;
        locals.var_guard1292_rv = 0.0;

        let (assign50140_e75474,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1290 != 0.0)) && (locals.var_guard1291 != 0.0)) && (locals.var_guard1292 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign50140_e75474;
        locals.var_mm_rv = 0.0;

        let assign50150_e75477: f64 = if 6.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1293 = assign50150_e75477;
        locals.var_guard1293_rv = 0.0;

        let (assign50160_e75497,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1290 != 0.0)) && (locals.var_guard1291 != 0.0)) && (locals.var_guard1292 == 0.0)) && (locals.var_guard1293 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign50160_e75497;
        locals.var_mm_rv = 0.0;

        let assign50170_e75500: f64 = if 6.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1294 = assign50170_e75500;
        locals.var_guard1294_rv = 0.0;

        let (assign50180_e75523,) = {
    if (((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1290 != 0.0)) && (locals.var_guard1291 != 0.0)) && (locals.var_guard1292 == 0.0)) && (locals.var_guard1293 == 0.0)) && (locals.var_guard1294 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign50180_e75523;
        locals.var_mm_rv = 0.0;

        let assign50190_e75526: f64 = if 6.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1295 = assign50190_e75526;
        locals.var_guard1295_rv = 0.0;

        let (assign50200_e75552,) = {
    if ((((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1290 != 0.0)) && (locals.var_guard1291 != 0.0)) && (locals.var_guard1292 == 0.0)) && (locals.var_guard1293 == 0.0)) && (locals.var_guard1294 == 0.0)) && (locals.var_guard1295 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign50200_e75552;
        locals.var_mm_rv = 0.0;

        let (assign50210_e75567,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1290 != 0.0)) && (locals.var_guard1291 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign50210_e75567;
        locals.var_m0_rv = 0.0;

        let mut assign50220_loop_guard: usize = 0;
        while {
            let assign50220_cond_e75583: f64 = if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1290 != 0.0)) && (locals.var_guard1291 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign50220_cond_e75583 != 0.0
        } {
            assign50220_loop_guard += 1;
            assert!(assign50220_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign50220_body0_e75599, assign50220_body0_e75599_d_n0, assign50220_body0_e75599_d_n2, assign50220_body0_e75599_d_n4, assign50220_body0_e75599_d_n5, assign50220_body0_e75599_d_n6, assign50220_body0_e75599_d_n7, assign50220_body0_e75599_d_n8, assign50220_body0_e75599_d_n9, assign50220_body0_e75599_d_n10, assign50220_body0_e75599_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1290 != 0.0)) && (locals.var_guard1291 != 0.0)) {
        let assign50220_body0_e75597: f64 = (locals.var_dnm).sqrt();
        (assign50220_body0_e75597, (locals.var_dnm_dn0 / (2.0 * assign50220_body0_e75597)), (locals.var_dnm_dn2 / (2.0 * assign50220_body0_e75597)), (locals.var_dnm_dn4 / (2.0 * assign50220_body0_e75597)), (locals.var_dnm_dn5 / (2.0 * assign50220_body0_e75597)), (locals.var_dnm_dn6 / (2.0 * assign50220_body0_e75597)), (locals.var_dnm_dn7 / (2.0 * assign50220_body0_e75597)), (locals.var_dnm_dn8 / (2.0 * assign50220_body0_e75597)), (locals.var_dnm_dn9 / (2.0 * assign50220_body0_e75597)), (locals.var_dnm_dn10 / (2.0 * assign50220_body0_e75597)), (locals.var_dnm_dn13 / (2.0 * assign50220_body0_e75597)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign50220_body0_e75599;
            locals.var_dnm_dn0 = assign50220_body0_e75599_d_n0;
            locals.var_dnm_dn2 = assign50220_body0_e75599_d_n2;
            locals.var_dnm_dn4 = assign50220_body0_e75599_d_n4;
            locals.var_dnm_dn5 = assign50220_body0_e75599_d_n5;
            locals.var_dnm_dn6 = assign50220_body0_e75599_d_n6;
            locals.var_dnm_dn7 = assign50220_body0_e75599_d_n7;
            locals.var_dnm_dn8 = assign50220_body0_e75599_d_n8;
            locals.var_dnm_dn9 = assign50220_body0_e75599_d_n9;
            locals.var_dnm_dn10 = assign50220_body0_e75599_d_n10;
            locals.var_dnm_dn13 = assign50220_body0_e75599_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign50220_body1_e75616,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1290 != 0.0)) && (locals.var_guard1291 != 0.0)) {
        let assign50220_body1_e75614: f64 = (locals.var_m0 + 1.0);
        (assign50220_body1_e75614,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign50220_body1_e75616;
            locals.var_m0_rv = 0.0;
        }

        let (assign50230_e75643, assign50230_e75643_d_n0, assign50230_e75643_d_n2, assign50230_e75643_d_n4, assign50230_e75643_d_n5, assign50230_e75643_d_n6, assign50230_e75643_d_n7, assign50230_e75643_d_n8, assign50230_e75643_d_n9, assign50230_e75643_d_n10, assign50230_e75643_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1290 != 0.0)) && (locals.var_guard1291 == 0.0)) {
        let (assign50230_e75641, assign50230_e75641_d_n0, assign50230_e75641_d_n2, assign50230_e75641_d_n4, assign50230_e75641_d_n5, assign50230_e75641_d_n6, assign50230_e75641_d_n7, assign50230_e75641_d_n8, assign50230_e75641_d_n9, assign50230_e75641_d_n10, assign50230_e75641_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign50230_e75638: f64 = (2.0 * 6.0);
                let assign50230_e75639: f64 = (1.0 / assign50230_e75638);
                let assign50230_e75640: f64 = (locals.var_dnm).powf(assign50230_e75639);
                (assign50230_e75640, if 0.0 == 0.0 && ((assign50230_e75639) as f64).is_finite() && ((assign50230_e75639) as f64).fract() == 0.0 { if assign50230_e75639 == 0.0 { 0.0 } else { (assign50230_e75639 * ((locals.var_dnm).powf(assign50230_e75639 - 1.0) * locals.var_dnm_dn0)) } } else { (assign50230_e75640 * (assign50230_e75639 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign50230_e75639) as f64).is_finite() && ((assign50230_e75639) as f64).fract() == 0.0 { if assign50230_e75639 == 0.0 { 0.0 } else { (assign50230_e75639 * ((locals.var_dnm).powf(assign50230_e75639 - 1.0) * locals.var_dnm_dn2)) } } else { (assign50230_e75640 * (assign50230_e75639 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign50230_e75639) as f64).is_finite() && ((assign50230_e75639) as f64).fract() == 0.0 { if assign50230_e75639 == 0.0 { 0.0 } else { (assign50230_e75639 * ((locals.var_dnm).powf(assign50230_e75639 - 1.0) * locals.var_dnm_dn4)) } } else { (assign50230_e75640 * (assign50230_e75639 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign50230_e75639) as f64).is_finite() && ((assign50230_e75639) as f64).fract() == 0.0 { if assign50230_e75639 == 0.0 { 0.0 } else { (assign50230_e75639 * ((locals.var_dnm).powf(assign50230_e75639 - 1.0) * locals.var_dnm_dn5)) } } else { (assign50230_e75640 * (assign50230_e75639 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign50230_e75639) as f64).is_finite() && ((assign50230_e75639) as f64).fract() == 0.0 { if assign50230_e75639 == 0.0 { 0.0 } else { (assign50230_e75639 * ((locals.var_dnm).powf(assign50230_e75639 - 1.0) * locals.var_dnm_dn6)) } } else { (assign50230_e75640 * (assign50230_e75639 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign50230_e75639) as f64).is_finite() && ((assign50230_e75639) as f64).fract() == 0.0 { if assign50230_e75639 == 0.0 { 0.0 } else { (assign50230_e75639 * ((locals.var_dnm).powf(assign50230_e75639 - 1.0) * locals.var_dnm_dn7)) } } else { (assign50230_e75640 * (assign50230_e75639 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign50230_e75639) as f64).is_finite() && ((assign50230_e75639) as f64).fract() == 0.0 { if assign50230_e75639 == 0.0 { 0.0 } else { (assign50230_e75639 * ((locals.var_dnm).powf(assign50230_e75639 - 1.0) * locals.var_dnm_dn8)) } } else { (assign50230_e75640 * (assign50230_e75639 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign50230_e75639) as f64).is_finite() && ((assign50230_e75639) as f64).fract() == 0.0 { if assign50230_e75639 == 0.0 { 0.0 } else { (assign50230_e75639 * ((locals.var_dnm).powf(assign50230_e75639 - 1.0) * locals.var_dnm_dn9)) } } else { (assign50230_e75640 * (assign50230_e75639 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign50230_e75639) as f64).is_finite() && ((assign50230_e75639) as f64).fract() == 0.0 { if assign50230_e75639 == 0.0 { 0.0 } else { (assign50230_e75639 * ((locals.var_dnm).powf(assign50230_e75639 - 1.0) * locals.var_dnm_dn10)) } } else { (assign50230_e75640 * (assign50230_e75639 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign50230_e75639) as f64).is_finite() && ((assign50230_e75639) as f64).fract() == 0.0 { if assign50230_e75639 == 0.0 { 0.0 } else { (assign50230_e75639 * ((locals.var_dnm).powf(assign50230_e75639 - 1.0) * locals.var_dnm_dn13)) } } else { (assign50230_e75640 * (assign50230_e75639 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign50230_e75641, assign50230_e75641_d_n0, assign50230_e75641_d_n2, assign50230_e75641_d_n4, assign50230_e75641_d_n5, assign50230_e75641_d_n6, assign50230_e75641_d_n7, assign50230_e75641_d_n8, assign50230_e75641_d_n9, assign50230_e75641_d_n10, assign50230_e75641_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign50230_e75643;
        locals.var_dnm_dn0 = assign50230_e75643_d_n0;
        locals.var_dnm_dn2 = assign50230_e75643_d_n2;
        locals.var_dnm_dn4 = assign50230_e75643_d_n4;
        locals.var_dnm_dn5 = assign50230_e75643_d_n5;
        locals.var_dnm_dn6 = assign50230_e75643_d_n6;
        locals.var_dnm_dn7 = assign50230_e75643_d_n7;
        locals.var_dnm_dn8 = assign50230_e75643_d_n8;
        locals.var_dnm_dn9 = assign50230_e75643_d_n9;
        locals.var_dnm_dn10 = assign50230_e75643_d_n10;
        locals.var_dnm_dn13 = assign50230_e75643_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign50240_e75658, assign50240_e75658_d_n0, assign50240_e75658_d_n2, assign50240_e75658_d_n4, assign50240_e75658_d_n5, assign50240_e75658_d_n6, assign50240_e75658_d_n7, assign50240_e75658_d_n8, assign50240_e75658_d_n9, assign50240_e75658_d_n10, assign50240_e75658_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1290 != 0.0)) {
        let assign50240_e75656: f64 = (1.0 / locals.var_dnm);
        (assign50240_e75656, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign50240_e75658;
        locals.var_dnm_dn0 = assign50240_e75658_d_n0;
        locals.var_dnm_dn2 = assign50240_e75658_d_n2;
        locals.var_dnm_dn4 = assign50240_e75658_d_n4;
        locals.var_dnm_dn5 = assign50240_e75658_d_n5;
        locals.var_dnm_dn6 = assign50240_e75658_d_n6;
        locals.var_dnm_dn7 = assign50240_e75658_d_n7;
        locals.var_dnm_dn8 = assign50240_e75658_d_n8;
        locals.var_dnm_dn9 = assign50240_e75658_d_n9;
        locals.var_dnm_dn10 = assign50240_e75658_d_n10;
        locals.var_dnm_dn13 = assign50240_e75658_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign50250_e75675, assign50250_e75675_d_n0, assign50250_e75675_d_n2, assign50250_e75675_d_n4, assign50250_e75675_d_n5, assign50250_e75675_d_n6, assign50250_e75675_d_n7, assign50250_e75675_d_n8, assign50250_e75675_d_n9, assign50250_e75675_d_n10, assign50250_e75675_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1290 != 0.0)) {
        let assign50250_e75671: f64 = (locals.var_tmf1 * p.p403);
        let assign50250_e75673: f64 = (assign50250_e75671 * locals.var_dnm);
        (assign50250_e75673, (((locals.var_tmf1_dn0 * p.p403) * locals.var_dnm) + (assign50250_e75671 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * p.p403) * locals.var_dnm) + (assign50250_e75671 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * p.p403) * locals.var_dnm) + (assign50250_e75671 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * p.p403) * locals.var_dnm) + (assign50250_e75671 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * p.p403) * locals.var_dnm) + (assign50250_e75671 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * p.p403) * locals.var_dnm) + (assign50250_e75671 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * p.p403) * locals.var_dnm) + (assign50250_e75671 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * p.p403) * locals.var_dnm) + (assign50250_e75671 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * p.p403) * locals.var_dnm) + (assign50250_e75671 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * p.p403) * locals.var_dnm) + (assign50250_e75671 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign50250_e75675;
        locals.var_tmf0_dn0 = assign50250_e75675_d_n0;
        locals.var_tmf0_dn2 = assign50250_e75675_d_n2;
        locals.var_tmf0_dn4 = assign50250_e75675_d_n4;
        locals.var_tmf0_dn5 = assign50250_e75675_d_n5;
        locals.var_tmf0_dn6 = assign50250_e75675_d_n6;
        locals.var_tmf0_dn7 = assign50250_e75675_d_n7;
        locals.var_tmf0_dn8 = assign50250_e75675_d_n8;
        locals.var_tmf0_dn9 = assign50250_e75675_d_n9;
        locals.var_tmf0_dn10 = assign50250_e75675_d_n10;
        locals.var_tmf0_dn13 = assign50250_e75675_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign50260_e75694, assign50260_e75694_d_n0, assign50260_e75694_d_n2, assign50260_e75694_d_n4, assign50260_e75694_d_n5, assign50260_e75694_d_n6, assign50260_e75694_d_n7, assign50260_e75694_d_n8, assign50260_e75694_d_n9, assign50260_e75694_d_n10, assign50260_e75694_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1290 != 0.0)) {
        let assign50260_e75688: f64 = (p.p403 * locals.var_xmp);
        let assign50260_e75690: f64 = (assign50260_e75688 * locals.var_dnm);
        let assign50260_e75692: f64 = (assign50260_e75690 / locals.var_arg);
        (assign50260_e75692, ((((((p.p403 * locals.var_xmp_dn0) * locals.var_dnm) + (assign50260_e75688 * locals.var_dnm_dn0)) * locals.var_arg) - (assign50260_e75690 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((p.p403 * locals.var_xmp_dn2) * locals.var_dnm) + (assign50260_e75688 * locals.var_dnm_dn2)) * locals.var_arg) - (assign50260_e75690 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((p.p403 * locals.var_xmp_dn4) * locals.var_dnm) + (assign50260_e75688 * locals.var_dnm_dn4)) * locals.var_arg) - (assign50260_e75690 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((p.p403 * locals.var_xmp_dn5) * locals.var_dnm) + (assign50260_e75688 * locals.var_dnm_dn5)) * locals.var_arg) - (assign50260_e75690 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((p.p403 * locals.var_xmp_dn6) * locals.var_dnm) + (assign50260_e75688 * locals.var_dnm_dn6)) * locals.var_arg) - (assign50260_e75690 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((p.p403 * locals.var_xmp_dn7) * locals.var_dnm) + (assign50260_e75688 * locals.var_dnm_dn7)) * locals.var_arg) - (assign50260_e75690 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((p.p403 * locals.var_xmp_dn8) * locals.var_dnm) + (assign50260_e75688 * locals.var_dnm_dn8)) * locals.var_arg) - (assign50260_e75690 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((p.p403 * locals.var_xmp_dn9) * locals.var_dnm) + (assign50260_e75688 * locals.var_dnm_dn9)) * locals.var_arg) - (assign50260_e75690 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((p.p403 * locals.var_xmp_dn10) * locals.var_dnm) + (assign50260_e75688 * locals.var_dnm_dn10)) * locals.var_arg) - (assign50260_e75690 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((p.p403 * locals.var_xmp_dn13) * locals.var_dnm) + (assign50260_e75688 * locals.var_dnm_dn13)) * locals.var_arg) - (assign50260_e75690 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign50260_e75694;
        locals.var_t0_dn0 = assign50260_e75694_d_n0;
        locals.var_t0_dn2 = assign50260_e75694_d_n2;
        locals.var_t0_dn4 = assign50260_e75694_d_n4;
        locals.var_t0_dn5 = assign50260_e75694_d_n5;
        locals.var_t0_dn6 = assign50260_e75694_d_n6;
        locals.var_t0_dn7 = assign50260_e75694_d_n7;
        locals.var_t0_dn8 = assign50260_e75694_d_n8;
        locals.var_t0_dn9 = assign50260_e75694_d_n9;
        locals.var_t0_dn10 = assign50260_e75694_d_n10;
        locals.var_t0_dn13 = assign50260_e75694_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign50270_e75711, assign50270_e75711_d_n0, assign50270_e75711_d_n2, assign50270_e75711_d_n4, assign50270_e75711_d_n5, assign50270_e75711_d_n6, assign50270_e75711_d_n7, assign50270_e75711_d_n8, assign50270_e75711_d_n9, assign50270_e75711_d_n10, assign50270_e75711_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1290 != 0.0)) {
        let assign50270_e75707: f64 = p.p403;
        let assign50270_e75709: f64 = (assign50270_e75707 - locals.var_tmf0);
        (assign50270_e75709, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign50270_e75711;
        locals.var_t2_dn0 = assign50270_e75711_d_n0;
        locals.var_t2_dn2 = assign50270_e75711_d_n2;
        locals.var_t2_dn4 = assign50270_e75711_d_n4;
        locals.var_t2_dn5 = assign50270_e75711_d_n5;
        locals.var_t2_dn6 = assign50270_e75711_d_n6;
        locals.var_t2_dn7 = assign50270_e75711_d_n7;
        locals.var_t2_dn8 = assign50270_e75711_d_n8;
        locals.var_t2_dn9 = assign50270_e75711_d_n9;
        locals.var_t2_dn10 = assign50270_e75711_d_n10;
        locals.var_t2_dn13 = assign50270_e75711_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign50280_e75724, assign50280_e75724_d_n0, assign50280_e75724_d_n2, assign50280_e75724_d_n4, assign50280_e75724_d_n5, assign50280_e75724_d_n6, assign50280_e75724_d_n7, assign50280_e75724_d_n8, assign50280_e75724_d_n9, assign50280_e75724_d_n10, assign50280_e75724_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1290 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign50280_e75724;
        locals.var_t0_dn0 = assign50280_e75724_d_n0;
        locals.var_t0_dn2 = assign50280_e75724_d_n2;
        locals.var_t0_dn4 = assign50280_e75724_d_n4;
        locals.var_t0_dn5 = assign50280_e75724_d_n5;
        locals.var_t0_dn6 = assign50280_e75724_d_n6;
        locals.var_t0_dn7 = assign50280_e75724_d_n7;
        locals.var_t0_dn8 = assign50280_e75724_d_n8;
        locals.var_t0_dn9 = assign50280_e75724_d_n9;
        locals.var_t0_dn10 = assign50280_e75724_d_n10;
        locals.var_t0_dn13 = assign50280_e75724_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign50290_e75740, assign50290_e75740_d_n0, assign50290_e75740_d_n2, assign50290_e75740_d_n4, assign50290_e75740_d_n5, assign50290_e75740_d_n6, assign50290_e75740_d_n7, assign50290_e75740_d_n8, assign50290_e75740_d_n9, assign50290_e75740_d_n10, assign50290_e75740_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1290 == 0.0)) {
        let assign50290_e75738: f64 = (locals.var_ps0z - locals.var_vds_maxb0__blk1087);
        (assign50290_e75738, locals.var_ps0z_dn0, locals.var_ps0z_dn2, locals.var_ps0z_dn4, locals.var_ps0z_dn5, locals.var_ps0z_dn6, locals.var_ps0z_dn7, locals.var_ps0z_dn8, locals.var_ps0z_dn9, locals.var_ps0z_dn10, locals.var_ps0z_dn13,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign50290_e75740;
        locals.var_t2_dn0 = assign50290_e75740_d_n0;
        locals.var_t2_dn2 = assign50290_e75740_d_n2;
        locals.var_t2_dn4 = assign50290_e75740_d_n4;
        locals.var_t2_dn5 = assign50290_e75740_d_n5;
        locals.var_t2_dn6 = assign50290_e75740_d_n6;
        locals.var_t2_dn7 = assign50290_e75740_d_n7;
        locals.var_t2_dn8 = assign50290_e75740_d_n8;
        locals.var_t2_dn9 = assign50290_e75740_d_n9;
        locals.var_t2_dn10 = assign50290_e75740_d_n10;
        locals.var_t2_dn13 = assign50290_e75740_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign50300_e75754, assign50300_e75754_d_n0, assign50300_e75754_d_n2, assign50300_e75754_d_n4, assign50300_e75754_d_n5, assign50300_e75754_d_n6, assign50300_e75754_d_n7, assign50300_e75754_d_n8, assign50300_e75754_d_n9, assign50300_e75754_d_n10, assign50300_e75754_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1290 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign50300_e75754;
        locals.var_t0_dn0 = assign50300_e75754_d_n0;
        locals.var_t0_dn2 = assign50300_e75754_d_n2;
        locals.var_t0_dn4 = assign50300_e75754_d_n4;
        locals.var_t0_dn5 = assign50300_e75754_d_n5;
        locals.var_t0_dn6 = assign50300_e75754_d_n6;
        locals.var_t0_dn7 = assign50300_e75754_d_n7;
        locals.var_t0_dn8 = assign50300_e75754_d_n8;
        locals.var_t0_dn9 = assign50300_e75754_d_n9;
        locals.var_t0_dn10 = assign50300_e75754_d_n10;
        locals.var_t0_dn13 = assign50300_e75754_d_n13;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_174(
        locals: &mut StampLocals,
    ) {
        let (assign50310_e75776, assign50310_e75776_d_n0, assign50310_e75776_d_n2, assign50310_e75776_d_n4, assign50310_e75776_d_n5, assign50310_e75776_d_n6, assign50310_e75776_d_n7, assign50310_e75776_d_n8, assign50310_e75776_d_n9, assign50310_e75776_d_n10, assign50310_e75776_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign50310_e75765: f64 = (locals.var_beta * locals.var_t2);
        let assign50310_e75766: f64 = (assign50310_e75765).exp();
        let assign50310_e75768: f64 = (assign50310_e75766 - 1.0);
        let assign50310_e75771: f64 = (locals.var_beta * locals.var_t2);
        let assign50310_e75772: f64 = (assign50310_e75768 - assign50310_e75771);
        let assign50310_e75774: f64 = (assign50310_e75772 + 1e-15);
        (assign50310_e75774, ((assign50310_e75766 * ((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0))) - ((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0))), ((assign50310_e75766 * ((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2))) - ((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2))), ((assign50310_e75766 * ((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4))) - ((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4))), ((assign50310_e75766 * ((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5))) - ((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5))), ((assign50310_e75766 * ((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6))) - ((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6))), ((assign50310_e75766 * ((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7))) - ((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7))), ((assign50310_e75766 * ((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8))) - ((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8))), ((assign50310_e75766 * ((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9))) - ((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9))), ((assign50310_e75766 * ((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10))) - ((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10))), ((assign50310_e75766 * ((locals.var_beta_dn13 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn13))) - ((locals.var_beta_dn13 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn13))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign50310_e75776;
        locals.var_t4_dn0 = assign50310_e75776_d_n0;
        locals.var_t4_dn2 = assign50310_e75776_d_n2;
        locals.var_t4_dn4 = assign50310_e75776_d_n4;
        locals.var_t4_dn5 = assign50310_e75776_d_n5;
        locals.var_t4_dn6 = assign50310_e75776_d_n6;
        locals.var_t4_dn7 = assign50310_e75776_d_n7;
        locals.var_t4_dn8 = assign50310_e75776_d_n8;
        locals.var_t4_dn9 = assign50310_e75776_d_n9;
        locals.var_t4_dn10 = assign50310_e75776_d_n10;
        locals.var_t4_dn13 = assign50310_e75776_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign50320_e75791, assign50320_e75791_d_n0, assign50320_e75791_d_n2, assign50320_e75791_d_n4, assign50320_e75791_d_n5, assign50320_e75791_d_n6, assign50320_e75791_d_n7, assign50320_e75791_d_n8, assign50320_e75791_d_n9, assign50320_e75791_d_n10, assign50320_e75791_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign50320_e75786: f64 = (-locals.var_cnst0);
        let assign50320_e75788: f64 = (locals.var_t4).sqrt();
        let assign50320_e75789: f64 = (assign50320_e75786 * assign50320_e75788);
        (assign50320_e75789, (((-locals.var_cnst0_dn0) * assign50320_e75788) + (assign50320_e75786 * (locals.var_t4_dn0 / (2.0 * assign50320_e75788)))), (((-locals.var_cnst0_dn2) * assign50320_e75788) + (assign50320_e75786 * (locals.var_t4_dn2 / (2.0 * assign50320_e75788)))), (((-locals.var_cnst0_dn4) * assign50320_e75788) + (assign50320_e75786 * (locals.var_t4_dn4 / (2.0 * assign50320_e75788)))), (((-locals.var_cnst0_dn5) * assign50320_e75788) + (assign50320_e75786 * (locals.var_t4_dn5 / (2.0 * assign50320_e75788)))), (((-locals.var_cnst0_dn6) * assign50320_e75788) + (assign50320_e75786 * (locals.var_t4_dn6 / (2.0 * assign50320_e75788)))), (((-locals.var_cnst0_dn7) * assign50320_e75788) + (assign50320_e75786 * (locals.var_t4_dn7 / (2.0 * assign50320_e75788)))), (((-locals.var_cnst0_dn8) * assign50320_e75788) + (assign50320_e75786 * (locals.var_t4_dn8 / (2.0 * assign50320_e75788)))), (((-locals.var_cnst0_dn9) * assign50320_e75788) + (assign50320_e75786 * (locals.var_t4_dn9 / (2.0 * assign50320_e75788)))), (((-locals.var_cnst0_dn10) * assign50320_e75788) + (assign50320_e75786 * (locals.var_t4_dn10 / (2.0 * assign50320_e75788)))), (((-locals.var_cnst0_dn13) * assign50320_e75788) + (assign50320_e75786 * (locals.var_t4_dn13 / (2.0 * assign50320_e75788)))),)
    } else {
        (locals.var_q_n0_sym__blk1120, locals.var_q_n0_sym__blk1120_dn0, locals.var_q_n0_sym__blk1120_dn2, locals.var_q_n0_sym__blk1120_dn4, locals.var_q_n0_sym__blk1120_dn5, locals.var_q_n0_sym__blk1120_dn6, locals.var_q_n0_sym__blk1120_dn7, locals.var_q_n0_sym__blk1120_dn8, locals.var_q_n0_sym__blk1120_dn9, locals.var_q_n0_sym__blk1120_dn10, locals.var_q_n0_sym__blk1120_dn13,)
    }
};
        locals.var_q_n0_sym__blk1120 = assign50320_e75791;
        locals.var_q_n0_sym__blk1120_dn0 = assign50320_e75791_d_n0;
        locals.var_q_n0_sym__blk1120_dn2 = assign50320_e75791_d_n2;
        locals.var_q_n0_sym__blk1120_dn4 = assign50320_e75791_d_n4;
        locals.var_q_n0_sym__blk1120_dn5 = assign50320_e75791_d_n5;
        locals.var_q_n0_sym__blk1120_dn6 = assign50320_e75791_d_n6;
        locals.var_q_n0_sym__blk1120_dn7 = assign50320_e75791_d_n7;
        locals.var_q_n0_sym__blk1120_dn8 = assign50320_e75791_d_n8;
        locals.var_q_n0_sym__blk1120_dn9 = assign50320_e75791_d_n9;
        locals.var_q_n0_sym__blk1120_dn10 = assign50320_e75791_d_n10;
        locals.var_q_n0_sym__blk1120_dn13 = assign50320_e75791_d_n13;
        locals.var_q_n0_sym__blk1120_rv = 0.0;

        let (assign50330_e75821, assign50330_e75821_d_n0, assign50330_e75821_d_n2, assign50330_e75821_d_n4, assign50330_e75821_d_n5, assign50330_e75821_d_n6, assign50330_e75821_d_n7, assign50330_e75821_d_n8, assign50330_e75821_d_n9, assign50330_e75821_d_n10, assign50330_e75821_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign50330_e75802: f64 = (locals.var_q_nl_cur__blk1119 + locals.var_q_n0_cur__blk1118);
        let assign50330_e75803: f64 = (-assign50330_e75802);
        let assign50330_e75806: f64 = (locals.var_q_nl_cur__blk1119 + locals.var_q_n0_cur__blk1118);
        let assign50330_e75807: f64 = (-assign50330_e75806);
        let assign50330_e75808: f64 = (assign50330_e75803 * assign50330_e75807);
        let assign50330_e75812: f64 = (1e-12 * 1e-6);
        let assign50330_e75813: f64 = (4.0 * assign50330_e75812);
        let assign50330_e75816: f64 = (1e-12 * 1e-6);
        let assign50330_e75817: f64 = (assign50330_e75813 * assign50330_e75816);
        let assign50330_e75818: f64 = (assign50330_e75808 + assign50330_e75817);
        let assign50330_e75819: f64 = (assign50330_e75818).sqrt();
        (assign50330_e75819, ((((-(locals.var_q_nl_cur__blk1119_dn0 + locals.var_q_n0_cur__blk1118_dn0)) * assign50330_e75807) + (assign50330_e75803 * (-(locals.var_q_nl_cur__blk1119_dn0 + locals.var_q_n0_cur__blk1118_dn0)))) / (2.0 * assign50330_e75819)), ((((-(locals.var_q_nl_cur__blk1119_dn2 + locals.var_q_n0_cur__blk1118_dn2)) * assign50330_e75807) + (assign50330_e75803 * (-(locals.var_q_nl_cur__blk1119_dn2 + locals.var_q_n0_cur__blk1118_dn2)))) / (2.0 * assign50330_e75819)), ((((-(locals.var_q_nl_cur__blk1119_dn4 + locals.var_q_n0_cur__blk1118_dn4)) * assign50330_e75807) + (assign50330_e75803 * (-(locals.var_q_nl_cur__blk1119_dn4 + locals.var_q_n0_cur__blk1118_dn4)))) / (2.0 * assign50330_e75819)), ((((-(locals.var_q_nl_cur__blk1119_dn5 + locals.var_q_n0_cur__blk1118_dn5)) * assign50330_e75807) + (assign50330_e75803 * (-(locals.var_q_nl_cur__blk1119_dn5 + locals.var_q_n0_cur__blk1118_dn5)))) / (2.0 * assign50330_e75819)), ((((-(locals.var_q_nl_cur__blk1119_dn6 + locals.var_q_n0_cur__blk1118_dn6)) * assign50330_e75807) + (assign50330_e75803 * (-(locals.var_q_nl_cur__blk1119_dn6 + locals.var_q_n0_cur__blk1118_dn6)))) / (2.0 * assign50330_e75819)), ((((-(locals.var_q_nl_cur__blk1119_dn7 + locals.var_q_n0_cur__blk1118_dn7)) * assign50330_e75807) + (assign50330_e75803 * (-(locals.var_q_nl_cur__blk1119_dn7 + locals.var_q_n0_cur__blk1118_dn7)))) / (2.0 * assign50330_e75819)), ((((-(locals.var_q_nl_cur__blk1119_dn8 + locals.var_q_n0_cur__blk1118_dn8)) * assign50330_e75807) + (assign50330_e75803 * (-(locals.var_q_nl_cur__blk1119_dn8 + locals.var_q_n0_cur__blk1118_dn8)))) / (2.0 * assign50330_e75819)), ((((-(locals.var_q_nl_cur__blk1119_dn9 + locals.var_q_n0_cur__blk1118_dn9)) * assign50330_e75807) + (assign50330_e75803 * (-(locals.var_q_nl_cur__blk1119_dn9 + locals.var_q_n0_cur__blk1118_dn9)))) / (2.0 * assign50330_e75819)), ((((-(locals.var_q_nl_cur__blk1119_dn10 + locals.var_q_n0_cur__blk1118_dn10)) * assign50330_e75807) + (assign50330_e75803 * (-(locals.var_q_nl_cur__blk1119_dn10 + locals.var_q_n0_cur__blk1118_dn10)))) / (2.0 * assign50330_e75819)), ((((-(locals.var_q_nl_cur__blk1119_dn13 + locals.var_q_n0_cur__blk1118_dn13)) * assign50330_e75807) + (assign50330_e75803 * (-(locals.var_q_nl_cur__blk1119_dn13 + locals.var_q_n0_cur__blk1118_dn13)))) / (2.0 * assign50330_e75819)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign50330_e75821;
        locals.var_tmf2_dn0 = assign50330_e75821_d_n0;
        locals.var_tmf2_dn2 = assign50330_e75821_d_n2;
        locals.var_tmf2_dn4 = assign50330_e75821_d_n4;
        locals.var_tmf2_dn5 = assign50330_e75821_d_n5;
        locals.var_tmf2_dn6 = assign50330_e75821_d_n6;
        locals.var_tmf2_dn7 = assign50330_e75821_d_n7;
        locals.var_tmf2_dn8 = assign50330_e75821_d_n8;
        locals.var_tmf2_dn9 = assign50330_e75821_d_n9;
        locals.var_tmf2_dn10 = assign50330_e75821_d_n10;
        locals.var_tmf2_dn13 = assign50330_e75821_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign50340_e75841, assign50340_e75841_d_n0, assign50340_e75841_d_n2, assign50340_e75841_d_n4, assign50340_e75841_d_n5, assign50340_e75841_d_n6, assign50340_e75841_d_n7, assign50340_e75841_d_n8, assign50340_e75841_d_n9, assign50340_e75841_d_n10, assign50340_e75841_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign50340_e75834: f64 = (locals.var_q_nl_cur__blk1119 + locals.var_q_n0_cur__blk1118);
        let assign50340_e75835: f64 = (-assign50340_e75834);
        let assign50340_e75837: f64 = (assign50340_e75835 / locals.var_tmf2);
        let assign50340_e75838: f64 = (1.0 + assign50340_e75837);
        let assign50340_e75839: f64 = (0.5 * assign50340_e75838);
        (assign50340_e75839, (0.5 * ((((-(locals.var_q_nl_cur__blk1119_dn0 + locals.var_q_n0_cur__blk1118_dn0)) * locals.var_tmf2) - (assign50340_e75835 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((-(locals.var_q_nl_cur__blk1119_dn2 + locals.var_q_n0_cur__blk1118_dn2)) * locals.var_tmf2) - (assign50340_e75835 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((-(locals.var_q_nl_cur__blk1119_dn4 + locals.var_q_n0_cur__blk1118_dn4)) * locals.var_tmf2) - (assign50340_e75835 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((-(locals.var_q_nl_cur__blk1119_dn5 + locals.var_q_n0_cur__blk1118_dn5)) * locals.var_tmf2) - (assign50340_e75835 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((-(locals.var_q_nl_cur__blk1119_dn6 + locals.var_q_n0_cur__blk1118_dn6)) * locals.var_tmf2) - (assign50340_e75835 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((-(locals.var_q_nl_cur__blk1119_dn7 + locals.var_q_n0_cur__blk1118_dn7)) * locals.var_tmf2) - (assign50340_e75835 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((-(locals.var_q_nl_cur__blk1119_dn8 + locals.var_q_n0_cur__blk1118_dn8)) * locals.var_tmf2) - (assign50340_e75835 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((-(locals.var_q_nl_cur__blk1119_dn9 + locals.var_q_n0_cur__blk1118_dn9)) * locals.var_tmf2) - (assign50340_e75835 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((-(locals.var_q_nl_cur__blk1119_dn10 + locals.var_q_n0_cur__blk1118_dn10)) * locals.var_tmf2) - (assign50340_e75835 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((-(locals.var_q_nl_cur__blk1119_dn13 + locals.var_q_n0_cur__blk1118_dn13)) * locals.var_tmf2) - (assign50340_e75835 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign50340_e75841;
        locals.var_t1_dn0 = assign50340_e75841_d_n0;
        locals.var_t1_dn2 = assign50340_e75841_d_n2;
        locals.var_t1_dn4 = assign50340_e75841_d_n4;
        locals.var_t1_dn5 = assign50340_e75841_d_n5;
        locals.var_t1_dn6 = assign50340_e75841_d_n6;
        locals.var_t1_dn7 = assign50340_e75841_d_n7;
        locals.var_t1_dn8 = assign50340_e75841_d_n8;
        locals.var_t1_dn9 = assign50340_e75841_d_n9;
        locals.var_t1_dn10 = assign50340_e75841_d_n10;
        locals.var_t1_dn13 = assign50340_e75841_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign50350_e75859, assign50350_e75859_d_n0, assign50350_e75859_d_n2, assign50350_e75859_d_n4, assign50350_e75859_d_n5, assign50350_e75859_d_n6, assign50350_e75859_d_n7, assign50350_e75859_d_n8, assign50350_e75859_d_n9, assign50350_e75859_d_n10, assign50350_e75859_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign50350_e75853: f64 = (locals.var_q_nl_cur__blk1119 + locals.var_q_n0_cur__blk1118);
        let assign50350_e75854: f64 = (-assign50350_e75853);
        let assign50350_e75856: f64 = (assign50350_e75854 + locals.var_tmf2);
        let assign50350_e75857: f64 = (0.5 * assign50350_e75856);
        (assign50350_e75857, (0.5 * ((-(locals.var_q_nl_cur__blk1119_dn0 + locals.var_q_n0_cur__blk1118_dn0)) + locals.var_tmf2_dn0)), (0.5 * ((-(locals.var_q_nl_cur__blk1119_dn2 + locals.var_q_n0_cur__blk1118_dn2)) + locals.var_tmf2_dn2)), (0.5 * ((-(locals.var_q_nl_cur__blk1119_dn4 + locals.var_q_n0_cur__blk1118_dn4)) + locals.var_tmf2_dn4)), (0.5 * ((-(locals.var_q_nl_cur__blk1119_dn5 + locals.var_q_n0_cur__blk1118_dn5)) + locals.var_tmf2_dn5)), (0.5 * ((-(locals.var_q_nl_cur__blk1119_dn6 + locals.var_q_n0_cur__blk1118_dn6)) + locals.var_tmf2_dn6)), (0.5 * ((-(locals.var_q_nl_cur__blk1119_dn7 + locals.var_q_n0_cur__blk1118_dn7)) + locals.var_tmf2_dn7)), (0.5 * ((-(locals.var_q_nl_cur__blk1119_dn8 + locals.var_q_n0_cur__blk1118_dn8)) + locals.var_tmf2_dn8)), (0.5 * ((-(locals.var_q_nl_cur__blk1119_dn9 + locals.var_q_n0_cur__blk1118_dn9)) + locals.var_tmf2_dn9)), (0.5 * ((-(locals.var_q_nl_cur__blk1119_dn10 + locals.var_q_n0_cur__blk1118_dn10)) + locals.var_tmf2_dn10)), (0.5 * ((-(locals.var_q_nl_cur__blk1119_dn13 + locals.var_q_n0_cur__blk1118_dn13)) + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_sum_q_nx_cur, locals.var_sum_q_nx_cur_dn0, locals.var_sum_q_nx_cur_dn2, locals.var_sum_q_nx_cur_dn4, locals.var_sum_q_nx_cur_dn5, locals.var_sum_q_nx_cur_dn6, locals.var_sum_q_nx_cur_dn7, locals.var_sum_q_nx_cur_dn8, locals.var_sum_q_nx_cur_dn9, locals.var_sum_q_nx_cur_dn10, locals.var_sum_q_nx_cur_dn13,)
    }
};
        locals.var_sum_q_nx_cur = assign50350_e75859;
        locals.var_sum_q_nx_cur_dn0 = assign50350_e75859_d_n0;
        locals.var_sum_q_nx_cur_dn2 = assign50350_e75859_d_n2;
        locals.var_sum_q_nx_cur_dn4 = assign50350_e75859_d_n4;
        locals.var_sum_q_nx_cur_dn5 = assign50350_e75859_d_n5;
        locals.var_sum_q_nx_cur_dn6 = assign50350_e75859_d_n6;
        locals.var_sum_q_nx_cur_dn7 = assign50350_e75859_d_n7;
        locals.var_sum_q_nx_cur_dn8 = assign50350_e75859_d_n8;
        locals.var_sum_q_nx_cur_dn9 = assign50350_e75859_d_n9;
        locals.var_sum_q_nx_cur_dn10 = assign50350_e75859_d_n10;
        locals.var_sum_q_nx_cur_dn13 = assign50350_e75859_d_n13;
        locals.var_sum_q_nx_cur_rv = 0.0;

        let assign50360_e75862: f64 = if locals.var_sum_q_nx_cur < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1296 = assign50360_e75862;
        locals.var_guard1296_rv = 0.0;

        let (assign50370_e75875, assign50370_e75875_d_n0, assign50370_e75875_d_n2, assign50370_e75875_d_n4, assign50370_e75875_d_n5, assign50370_e75875_d_n6, assign50370_e75875_d_n7, assign50370_e75875_d_n8, assign50370_e75875_d_n9, assign50370_e75875_d_n10, assign50370_e75875_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1296 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_sum_q_nx_cur, locals.var_sum_q_nx_cur_dn0, locals.var_sum_q_nx_cur_dn2, locals.var_sum_q_nx_cur_dn4, locals.var_sum_q_nx_cur_dn5, locals.var_sum_q_nx_cur_dn6, locals.var_sum_q_nx_cur_dn7, locals.var_sum_q_nx_cur_dn8, locals.var_sum_q_nx_cur_dn9, locals.var_sum_q_nx_cur_dn10, locals.var_sum_q_nx_cur_dn13,)
    }
};
        locals.var_sum_q_nx_cur = assign50370_e75875;
        locals.var_sum_q_nx_cur_dn0 = assign50370_e75875_d_n0;
        locals.var_sum_q_nx_cur_dn2 = assign50370_e75875_d_n2;
        locals.var_sum_q_nx_cur_dn4 = assign50370_e75875_d_n4;
        locals.var_sum_q_nx_cur_dn5 = assign50370_e75875_d_n5;
        locals.var_sum_q_nx_cur_dn6 = assign50370_e75875_d_n6;
        locals.var_sum_q_nx_cur_dn7 = assign50370_e75875_d_n7;
        locals.var_sum_q_nx_cur_dn8 = assign50370_e75875_d_n8;
        locals.var_sum_q_nx_cur_dn9 = assign50370_e75875_d_n9;
        locals.var_sum_q_nx_cur_dn10 = assign50370_e75875_d_n10;
        locals.var_sum_q_nx_cur_dn13 = assign50370_e75875_d_n13;
        locals.var_sum_q_nx_cur_rv = 0.0;

        let (assign50380_e75888, assign50380_e75888_d_n0, assign50380_e75888_d_n2, assign50380_e75888_d_n4, assign50380_e75888_d_n5, assign50380_e75888_d_n6, assign50380_e75888_d_n7, assign50380_e75888_d_n8, assign50380_e75888_d_n9, assign50380_e75888_d_n10, assign50380_e75888_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1296 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign50380_e75888;
        locals.var_t1_dn0 = assign50380_e75888_d_n0;
        locals.var_t1_dn2 = assign50380_e75888_d_n2;
        locals.var_t1_dn4 = assign50380_e75888_d_n4;
        locals.var_t1_dn5 = assign50380_e75888_d_n5;
        locals.var_t1_dn6 = assign50380_e75888_d_n6;
        locals.var_t1_dn7 = assign50380_e75888_d_n7;
        locals.var_t1_dn8 = assign50380_e75888_d_n8;
        locals.var_t1_dn9 = assign50380_e75888_d_n9;
        locals.var_t1_dn10 = assign50380_e75888_d_n10;
        locals.var_t1_dn13 = assign50380_e75888_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign50390_e75900, assign50390_e75900_d_n0, assign50390_e75900_d_n2, assign50390_e75900_d_n4, assign50390_e75900_d_n5, assign50390_e75900_d_n6, assign50390_e75900_d_n7, assign50390_e75900_d_n8, assign50390_e75900_d_n9, assign50390_e75900_d_n10, assign50390_e75900_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign50390_e75898: f64 = (-locals.var_sum_q_nx_cur);
        (assign50390_e75898, (-locals.var_sum_q_nx_cur_dn0), (-locals.var_sum_q_nx_cur_dn2), (-locals.var_sum_q_nx_cur_dn4), (-locals.var_sum_q_nx_cur_dn5), (-locals.var_sum_q_nx_cur_dn6), (-locals.var_sum_q_nx_cur_dn7), (-locals.var_sum_q_nx_cur_dn8), (-locals.var_sum_q_nx_cur_dn9), (-locals.var_sum_q_nx_cur_dn10), (-locals.var_sum_q_nx_cur_dn13),)
    } else {
        (locals.var_sum_q_nx_cur, locals.var_sum_q_nx_cur_dn0, locals.var_sum_q_nx_cur_dn2, locals.var_sum_q_nx_cur_dn4, locals.var_sum_q_nx_cur_dn5, locals.var_sum_q_nx_cur_dn6, locals.var_sum_q_nx_cur_dn7, locals.var_sum_q_nx_cur_dn8, locals.var_sum_q_nx_cur_dn9, locals.var_sum_q_nx_cur_dn10, locals.var_sum_q_nx_cur_dn13,)
    }
};
        locals.var_sum_q_nx_cur = assign50390_e75900;
        locals.var_sum_q_nx_cur_dn0 = assign50390_e75900_d_n0;
        locals.var_sum_q_nx_cur_dn2 = assign50390_e75900_d_n2;
        locals.var_sum_q_nx_cur_dn4 = assign50390_e75900_d_n4;
        locals.var_sum_q_nx_cur_dn5 = assign50390_e75900_d_n5;
        locals.var_sum_q_nx_cur_dn6 = assign50390_e75900_d_n6;
        locals.var_sum_q_nx_cur_dn7 = assign50390_e75900_d_n7;
        locals.var_sum_q_nx_cur_dn8 = assign50390_e75900_d_n8;
        locals.var_sum_q_nx_cur_dn9 = assign50390_e75900_d_n9;
        locals.var_sum_q_nx_cur_dn10 = assign50390_e75900_d_n10;
        locals.var_sum_q_nx_cur_dn13 = assign50390_e75900_d_n13;
        locals.var_sum_q_nx_cur_rv = 0.0;

        let (assign50400_e75918, assign50400_e75918_d_n0, assign50400_e75918_d_n2, assign50400_e75918_d_n4, assign50400_e75918_d_n5, assign50400_e75918_d_n6, assign50400_e75918_d_n7, assign50400_e75918_d_n8, assign50400_e75918_d_n9, assign50400_e75918_d_n10, assign50400_e75918_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign50400_e75910: f64 = (-locals.var_beta);
        let assign50400_e75912: f64 = (assign50400_e75910 * locals.var_sum_q_nx_cur);
        let assign50400_e75914: f64 = (assign50400_e75912 / 2.0);
        let assign50400_e75916: f64 = (assign50400_e75914 * locals.var_pds);
        (assign50400_e75916, ((((((-locals.var_beta_dn0) * locals.var_sum_q_nx_cur) + (assign50400_e75910 * locals.var_sum_q_nx_cur_dn0)) / 2.0) * locals.var_pds) + (assign50400_e75914 * locals.var_pds_dn0)), ((((((-locals.var_beta_dn2) * locals.var_sum_q_nx_cur) + (assign50400_e75910 * locals.var_sum_q_nx_cur_dn2)) / 2.0) * locals.var_pds) + (assign50400_e75914 * locals.var_pds_dn2)), ((((((-locals.var_beta_dn4) * locals.var_sum_q_nx_cur) + (assign50400_e75910 * locals.var_sum_q_nx_cur_dn4)) / 2.0) * locals.var_pds) + (assign50400_e75914 * locals.var_pds_dn4)), ((((((-locals.var_beta_dn5) * locals.var_sum_q_nx_cur) + (assign50400_e75910 * locals.var_sum_q_nx_cur_dn5)) / 2.0) * locals.var_pds) + (assign50400_e75914 * locals.var_pds_dn5)), ((((((-locals.var_beta_dn6) * locals.var_sum_q_nx_cur) + (assign50400_e75910 * locals.var_sum_q_nx_cur_dn6)) / 2.0) * locals.var_pds) + (assign50400_e75914 * locals.var_pds_dn6)), ((((((-locals.var_beta_dn7) * locals.var_sum_q_nx_cur) + (assign50400_e75910 * locals.var_sum_q_nx_cur_dn7)) / 2.0) * locals.var_pds) + (assign50400_e75914 * locals.var_pds_dn7)), ((((((-locals.var_beta_dn8) * locals.var_sum_q_nx_cur) + (assign50400_e75910 * locals.var_sum_q_nx_cur_dn8)) / 2.0) * locals.var_pds) + (assign50400_e75914 * locals.var_pds_dn8)), ((((((-locals.var_beta_dn9) * locals.var_sum_q_nx_cur) + (assign50400_e75910 * locals.var_sum_q_nx_cur_dn9)) / 2.0) * locals.var_pds) + (assign50400_e75914 * locals.var_pds_dn9)), ((((((-locals.var_beta_dn10) * locals.var_sum_q_nx_cur) + (assign50400_e75910 * locals.var_sum_q_nx_cur_dn10)) / 2.0) * locals.var_pds) + (assign50400_e75914 * locals.var_pds_dn10)), ((((((-locals.var_beta_dn13) * locals.var_sum_q_nx_cur) + (assign50400_e75910 * locals.var_sum_q_nx_cur_dn13)) / 2.0) * locals.var_pds) + (assign50400_e75914 * locals.var_pds_dn13)),)
    } else {
        (locals.var_idd, locals.var_idd_dn0, locals.var_idd_dn2, locals.var_idd_dn4, locals.var_idd_dn5, locals.var_idd_dn6, locals.var_idd_dn7, locals.var_idd_dn8, locals.var_idd_dn9, locals.var_idd_dn10, locals.var_idd_dn13,)
    }
};
        locals.var_idd = assign50400_e75918;
        locals.var_idd_dn0 = assign50400_e75918_d_n0;
        locals.var_idd_dn2 = assign50400_e75918_d_n2;
        locals.var_idd_dn4 = assign50400_e75918_d_n4;
        locals.var_idd_dn5 = assign50400_e75918_d_n5;
        locals.var_idd_dn6 = assign50400_e75918_d_n6;
        locals.var_idd_dn7 = assign50400_e75918_d_n7;
        locals.var_idd_dn8 = assign50400_e75918_d_n8;
        locals.var_idd_dn9 = assign50400_e75918_d_n9;
        locals.var_idd_dn10 = assign50400_e75918_d_n10;
        locals.var_idd_dn13 = assign50400_e75918_d_n13;
        locals.var_idd_rv = 0.0;

        let (assign50410_e75930, assign50410_e75930_d_n0, assign50410_e75930_d_n2, assign50410_e75930_d_n4, assign50410_e75930_d_n5, assign50410_e75930_d_n6, assign50410_e75930_d_n7, assign50410_e75930_d_n8, assign50410_e75930_d_n9, assign50410_e75930_d_n10, assign50410_e75930_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign50410_e75928: f64 = (-locals.var_q_n0_sym__blk1120);
        (assign50410_e75928, (-locals.var_q_n0_sym__blk1120_dn0), (-locals.var_q_n0_sym__blk1120_dn2), (-locals.var_q_n0_sym__blk1120_dn4), (-locals.var_q_n0_sym__blk1120_dn5), (-locals.var_q_n0_sym__blk1120_dn6), (-locals.var_q_n0_sym__blk1120_dn7), (-locals.var_q_n0_sym__blk1120_dn8), (-locals.var_q_n0_sym__blk1120_dn9), (-locals.var_q_n0_sym__blk1120_dn10), (-locals.var_q_n0_sym__blk1120_dn13),)
    } else {
        (locals.var_qn0, locals.var_qn0_dn0, locals.var_qn0_dn2, locals.var_qn0_dn4, locals.var_qn0_dn5, locals.var_qn0_dn6, locals.var_qn0_dn7, locals.var_qn0_dn8, locals.var_qn0_dn9, locals.var_qn0_dn10, locals.var_qn0_dn13,)
    }
};
        locals.var_qn0 = assign50410_e75930;
        locals.var_qn0_dn0 = assign50410_e75930_d_n0;
        locals.var_qn0_dn2 = assign50410_e75930_d_n2;
        locals.var_qn0_dn4 = assign50410_e75930_d_n4;
        locals.var_qn0_dn5 = assign50410_e75930_d_n5;
        locals.var_qn0_dn6 = assign50410_e75930_d_n6;
        locals.var_qn0_dn7 = assign50410_e75930_d_n7;
        locals.var_qn0_dn8 = assign50410_e75930_d_n8;
        locals.var_qn0_dn9 = assign50410_e75930_d_n9;
        locals.var_qn0_dn10 = assign50410_e75930_d_n10;
        locals.var_qn0_dn13 = assign50410_e75930_d_n13;
        locals.var_qn0_rv = 0.0;

        let (assign50420_e75941, assign50420_e75941_d_n0, assign50420_e75941_d_n2, assign50420_e75941_d_n4, assign50420_e75941_d_n5, assign50420_e75941_d_n6, assign50420_e75941_d_n7, assign50420_e75941_d_n8, assign50420_e75941_d_n9, assign50420_e75941_d_n10, assign50420_e75941_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        (locals.var_leff, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lch, locals.var_lch_dn0, locals.var_lch_dn2, locals.var_lch_dn4, locals.var_lch_dn5, locals.var_lch_dn6, locals.var_lch_dn7, locals.var_lch_dn8, locals.var_lch_dn9, locals.var_lch_dn10, locals.var_lch_dn13,)
    }
};
        locals.var_lch = assign50420_e75941;
        locals.var_lch_dn0 = assign50420_e75941_d_n0;
        locals.var_lch_dn2 = assign50420_e75941_d_n2;
        locals.var_lch_dn4 = assign50420_e75941_d_n4;
        locals.var_lch_dn5 = assign50420_e75941_d_n5;
        locals.var_lch_dn6 = assign50420_e75941_d_n6;
        locals.var_lch_dn7 = assign50420_e75941_d_n7;
        locals.var_lch_dn8 = assign50420_e75941_d_n8;
        locals.var_lch_dn9 = assign50420_e75941_d_n9;
        locals.var_lch_dn10 = assign50420_e75941_d_n10;
        locals.var_lch_dn13 = assign50420_e75941_d_n13;
        locals.var_lch_rv = 0.0;

        let (assign50430_e75952, assign50430_e75952_d_n0, assign50430_e75952_d_n2, assign50430_e75952_d_n4, assign50430_e75952_d_n5, assign50430_e75952_d_n6, assign50430_e75952_d_n7, assign50430_e75952_d_n8, assign50430_e75952_d_n9, assign50430_e75952_d_n10, assign50430_e75952_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn4, locals.var_vdsorg_dn5, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn8, locals.var_vdsorg_dn9, locals.var_vdsorg_dn10, locals.var_vdsorg_dn13,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn13,)
    }
};
        locals.var_vds = assign50430_e75952;
        locals.var_vds_dn0 = assign50430_e75952_d_n0;
        locals.var_vds_dn2 = assign50430_e75952_d_n2;
        locals.var_vds_dn4 = assign50430_e75952_d_n4;
        locals.var_vds_dn5 = assign50430_e75952_d_n5;
        locals.var_vds_dn6 = assign50430_e75952_d_n6;
        locals.var_vds_dn7 = assign50430_e75952_d_n7;
        locals.var_vds_dn8 = assign50430_e75952_d_n8;
        locals.var_vds_dn9 = assign50430_e75952_d_n9;
        locals.var_vds_dn10 = assign50430_e75952_d_n10;
        locals.var_vds_dn13 = assign50430_e75952_d_n13;
        locals.var_vds_rv = 0.0;

        let assign50440_e75956: f64 = (10.0 * 2.220446049250313e-16);
        let assign50440_e75961: f64 = (10.0 * 2.220446049250313e-16);
        let assign50440_e75963: f64 = if ((locals.var_uc_clm2 < assign50440_e75956) && (locals.var_uc_clm3 < assign50440_e75961)) { 1.0 } else { 0.0 };
        locals.var_guard1297 = assign50440_e75963;
        locals.var_guard1297_rv = 0.0;

        let (assign50450_e75976, assign50450_e75976_d_n0, assign50450_e75976_d_n2, assign50450_e75976_d_n4, assign50450_e75976_d_n5, assign50450_e75976_d_n6, assign50450_e75976_d_n7, assign50450_e75976_d_n8, assign50450_e75976_d_n9, assign50450_e75976_d_n10, assign50450_e75976_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn4, locals.var_lred_dn5, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn8, locals.var_lred_dn9, locals.var_lred_dn10, locals.var_lred_dn13,)
    }
};
        locals.var_lred = assign50450_e75976;
        locals.var_lred_dn0 = assign50450_e75976_d_n0;
        locals.var_lred_dn2 = assign50450_e75976_d_n2;
        locals.var_lred_dn4 = assign50450_e75976_d_n4;
        locals.var_lred_dn5 = assign50450_e75976_d_n5;
        locals.var_lred_dn6 = assign50450_e75976_d_n6;
        locals.var_lred_dn7 = assign50450_e75976_d_n7;
        locals.var_lred_dn8 = assign50450_e75976_d_n8;
        locals.var_lred_dn9 = assign50450_e75976_d_n9;
        locals.var_lred_dn10 = assign50450_e75976_d_n10;
        locals.var_lred_dn13 = assign50450_e75976_d_n13;
        locals.var_lred_rv = 0.0;

        let (assign50460_e75989, assign50460_e75989_d_n0, assign50460_e75989_d_n2, assign50460_e75989_d_n4, assign50460_e75989_d_n5, assign50460_e75989_d_n6, assign50460_e75989_d_n7, assign50460_e75989_d_n8, assign50460_e75989_d_n9, assign50460_e75989_d_n10, assign50460_e75989_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 != 0.0)) {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn4, locals.var_psl_dn5, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn8, locals.var_psl_dn9, locals.var_psl_dn10, locals.var_psl_dn13,)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn13,)
    }
};
        locals.var_psdl = assign50460_e75989;
        locals.var_psdl_dn0 = assign50460_e75989_d_n0;
        locals.var_psdl_dn2 = assign50460_e75989_d_n2;
        locals.var_psdl_dn4 = assign50460_e75989_d_n4;
        locals.var_psdl_dn5 = assign50460_e75989_d_n5;
        locals.var_psdl_dn6 = assign50460_e75989_d_n6;
        locals.var_psdl_dn7 = assign50460_e75989_d_n7;
        locals.var_psdl_dn8 = assign50460_e75989_d_n8;
        locals.var_psdl_dn9 = assign50460_e75989_d_n9;
        locals.var_psdl_dn10 = assign50460_e75989_d_n10;
        locals.var_psdl_dn13 = assign50460_e75989_d_n13;
        locals.var_psdl_rv = 0.0;

        let assign50470_e75993: f64 = (locals.var_ps0 + locals.var_vds);
        let assign50470_e75996: f64 = (10.0 * 2.220446049250313e-16);
        let assign50470_e75997: f64 = (assign50470_e75993 - assign50470_e75996);
        let assign50470_e76000: f64 = (10.0 * 2.220446049250313e-16);
        let assign50470_e76001: f64 = (assign50470_e75997 - assign50470_e76000);
        let assign50470_e76005: f64 = (10.0 * 2.220446049250313e-16);
        let assign50470_e76008: f64 = if ((locals.var_psdl > assign50470_e76001) && (assign50470_e76005 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1298 = assign50470_e76008;
        locals.var_guard1298_rv = 0.0;

        let (assign50480_e76035, assign50480_e76035_d_n0, assign50480_e76035_d_n2, assign50480_e76035_d_n4, assign50480_e76035_d_n5, assign50480_e76035_d_n6, assign50480_e76035_d_n7, assign50480_e76035_d_n8, assign50480_e76035_d_n9, assign50480_e76035_d_n10, assign50480_e76035_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 != 0.0)) && (locals.var_guard1298 != 0.0)) {
        let assign50480_e76024: f64 = (locals.var_ps0 + locals.var_vds);
        let assign50480_e76027: f64 = (10.0 * 2.220446049250313e-16);
        let assign50480_e76028: f64 = (assign50480_e76024 - assign50480_e76027);
        let assign50480_e76029: f64 = (locals.var_psdl - assign50480_e76028);
        let assign50480_e76032: f64 = (10.0 * 2.220446049250313e-16);
        let assign50480_e76033: f64 = (assign50480_e76029 + assign50480_e76032);
        (assign50480_e76033, (locals.var_psdl_dn0 - (locals.var_ps0_dn0 + locals.var_vds_dn0)), (locals.var_psdl_dn2 - (locals.var_ps0_dn2 + locals.var_vds_dn2)), (locals.var_psdl_dn4 - (locals.var_ps0_dn4 + locals.var_vds_dn4)), (locals.var_psdl_dn5 - (locals.var_ps0_dn5 + locals.var_vds_dn5)), (locals.var_psdl_dn6 - (locals.var_ps0_dn6 + locals.var_vds_dn6)), (locals.var_psdl_dn7 - (locals.var_ps0_dn7 + locals.var_vds_dn7)), (locals.var_psdl_dn8 - (locals.var_ps0_dn8 + locals.var_vds_dn8)), (locals.var_psdl_dn9 - (locals.var_ps0_dn9 + locals.var_vds_dn9)), (locals.var_psdl_dn10 - (locals.var_ps0_dn10 + locals.var_vds_dn10)), (locals.var_psdl_dn13 - (locals.var_ps0_dn13 + locals.var_vds_dn13)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign50480_e76035;
        locals.var_tmf1_dn0 = assign50480_e76035_d_n0;
        locals.var_tmf1_dn2 = assign50480_e76035_d_n2;
        locals.var_tmf1_dn4 = assign50480_e76035_d_n4;
        locals.var_tmf1_dn5 = assign50480_e76035_d_n5;
        locals.var_tmf1_dn6 = assign50480_e76035_d_n6;
        locals.var_tmf1_dn7 = assign50480_e76035_d_n7;
        locals.var_tmf1_dn8 = assign50480_e76035_d_n8;
        locals.var_tmf1_dn9 = assign50480_e76035_d_n9;
        locals.var_tmf1_dn10 = assign50480_e76035_d_n10;
        locals.var_tmf1_dn13 = assign50480_e76035_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign50490_e76052, assign50490_e76052_d_n0, assign50490_e76052_d_n2, assign50490_e76052_d_n4, assign50490_e76052_d_n5, assign50490_e76052_d_n6, assign50490_e76052_d_n7, assign50490_e76052_d_n8, assign50490_e76052_d_n9, assign50490_e76052_d_n10, assign50490_e76052_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 != 0.0)) && (locals.var_guard1298 != 0.0)) {
        let assign50490_e76050: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign50490_e76050, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign50490_e76052;
        locals.var_x2_dn0 = assign50490_e76052_d_n0;
        locals.var_x2_dn2 = assign50490_e76052_d_n2;
        locals.var_x2_dn4 = assign50490_e76052_d_n4;
        locals.var_x2_dn5 = assign50490_e76052_d_n5;
        locals.var_x2_dn6 = assign50490_e76052_d_n6;
        locals.var_x2_dn7 = assign50490_e76052_d_n7;
        locals.var_x2_dn8 = assign50490_e76052_d_n8;
        locals.var_x2_dn9 = assign50490_e76052_d_n9;
        locals.var_x2_dn10 = assign50490_e76052_d_n10;
        locals.var_x2_dn13 = assign50490_e76052_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign50500_e76073, assign50500_e76073_d_n0, assign50500_e76073_d_n2, assign50500_e76073_d_n4, assign50500_e76073_d_n5, assign50500_e76073_d_n6, assign50500_e76073_d_n7, assign50500_e76073_d_n8, assign50500_e76073_d_n9, assign50500_e76073_d_n10, assign50500_e76073_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 != 0.0)) && (locals.var_guard1298 != 0.0)) {
        let assign50500_e76067: f64 = (10.0 * 2.220446049250313e-16);
        let assign50500_e76070: f64 = (10.0 * 2.220446049250313e-16);
        let assign50500_e76071: f64 = (assign50500_e76067 * assign50500_e76070);
        (assign50500_e76071, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign50500_e76073;
        locals.var_xmax2_dn0 = assign50500_e76073_d_n0;
        locals.var_xmax2_dn2 = assign50500_e76073_d_n2;
        locals.var_xmax2_dn4 = assign50500_e76073_d_n4;
        locals.var_xmax2_dn5 = assign50500_e76073_d_n5;
        locals.var_xmax2_dn6 = assign50500_e76073_d_n6;
        locals.var_xmax2_dn7 = assign50500_e76073_d_n7;
        locals.var_xmax2_dn8 = assign50500_e76073_d_n8;
        locals.var_xmax2_dn9 = assign50500_e76073_d_n9;
        locals.var_xmax2_dn10 = assign50500_e76073_d_n10;
        locals.var_xmax2_dn13 = assign50500_e76073_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign50510_e76088, assign50510_e76088_d_n0, assign50510_e76088_d_n2, assign50510_e76088_d_n4, assign50510_e76088_d_n5, assign50510_e76088_d_n6, assign50510_e76088_d_n7, assign50510_e76088_d_n8, assign50510_e76088_d_n9, assign50510_e76088_d_n10, assign50510_e76088_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 != 0.0)) && (locals.var_guard1298 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign50510_e76088;
        locals.var_xp_dn0 = assign50510_e76088_d_n0;
        locals.var_xp_dn2 = assign50510_e76088_d_n2;
        locals.var_xp_dn4 = assign50510_e76088_d_n4;
        locals.var_xp_dn5 = assign50510_e76088_d_n5;
        locals.var_xp_dn6 = assign50510_e76088_d_n6;
        locals.var_xp_dn7 = assign50510_e76088_d_n7;
        locals.var_xp_dn8 = assign50510_e76088_d_n8;
        locals.var_xp_dn9 = assign50510_e76088_d_n9;
        locals.var_xp_dn10 = assign50510_e76088_d_n10;
        locals.var_xp_dn13 = assign50510_e76088_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign50520_e76103, assign50520_e76103_d_n0, assign50520_e76103_d_n2, assign50520_e76103_d_n4, assign50520_e76103_d_n5, assign50520_e76103_d_n6, assign50520_e76103_d_n7, assign50520_e76103_d_n8, assign50520_e76103_d_n9, assign50520_e76103_d_n10, assign50520_e76103_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 != 0.0)) && (locals.var_guard1298 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign50520_e76103;
        locals.var_xmp_dn0 = assign50520_e76103_d_n0;
        locals.var_xmp_dn2 = assign50520_e76103_d_n2;
        locals.var_xmp_dn4 = assign50520_e76103_d_n4;
        locals.var_xmp_dn5 = assign50520_e76103_d_n5;
        locals.var_xmp_dn6 = assign50520_e76103_d_n6;
        locals.var_xmp_dn7 = assign50520_e76103_d_n7;
        locals.var_xmp_dn8 = assign50520_e76103_d_n8;
        locals.var_xmp_dn9 = assign50520_e76103_d_n9;
        locals.var_xmp_dn10 = assign50520_e76103_d_n10;
        locals.var_xmp_dn13 = assign50520_e76103_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign50530_e76118,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 != 0.0)) && (locals.var_guard1298 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign50530_e76118;
        locals.var_m0_rv = 0.0;

        let (assign50540_e76133,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 != 0.0)) && (locals.var_guard1298 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign50540_e76133;
        locals.var_mm_rv = 0.0;

        let (assign50550_e76148, assign50550_e76148_d_n0, assign50550_e76148_d_n2, assign50550_e76148_d_n4, assign50550_e76148_d_n5, assign50550_e76148_d_n6, assign50550_e76148_d_n7, assign50550_e76148_d_n8, assign50550_e76148_d_n9, assign50550_e76148_d_n10, assign50550_e76148_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 != 0.0)) && (locals.var_guard1298 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign50550_e76148;
        locals.var_arg_dn0 = assign50550_e76148_d_n0;
        locals.var_arg_dn2 = assign50550_e76148_d_n2;
        locals.var_arg_dn4 = assign50550_e76148_d_n4;
        locals.var_arg_dn5 = assign50550_e76148_d_n5;
        locals.var_arg_dn6 = assign50550_e76148_d_n6;
        locals.var_arg_dn7 = assign50550_e76148_d_n7;
        locals.var_arg_dn8 = assign50550_e76148_d_n8;
        locals.var_arg_dn9 = assign50550_e76148_d_n9;
        locals.var_arg_dn10 = assign50550_e76148_d_n10;
        locals.var_arg_dn13 = assign50550_e76148_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign50560_e76163, assign50560_e76163_d_n0, assign50560_e76163_d_n2, assign50560_e76163_d_n4, assign50560_e76163_d_n5, assign50560_e76163_d_n6, assign50560_e76163_d_n7, assign50560_e76163_d_n8, assign50560_e76163_d_n9, assign50560_e76163_d_n10, assign50560_e76163_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 != 0.0)) && (locals.var_guard1298 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign50560_e76163;
        locals.var_dnm_dn0 = assign50560_e76163_d_n0;
        locals.var_dnm_dn2 = assign50560_e76163_d_n2;
        locals.var_dnm_dn4 = assign50560_e76163_d_n4;
        locals.var_dnm_dn5 = assign50560_e76163_d_n5;
        locals.var_dnm_dn6 = assign50560_e76163_d_n6;
        locals.var_dnm_dn7 = assign50560_e76163_d_n7;
        locals.var_dnm_dn8 = assign50560_e76163_d_n8;
        locals.var_dnm_dn9 = assign50560_e76163_d_n9;
        locals.var_dnm_dn10 = assign50560_e76163_d_n10;
        locals.var_dnm_dn13 = assign50560_e76163_d_n13;
        locals.var_dnm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_175(
        locals: &mut StampLocals,
    ) {
        let (assign50570_e76180, assign50570_e76180_d_n0, assign50570_e76180_d_n2, assign50570_e76180_d_n4, assign50570_e76180_d_n5, assign50570_e76180_d_n6, assign50570_e76180_d_n7, assign50570_e76180_d_n8, assign50570_e76180_d_n9, assign50570_e76180_d_n10, assign50570_e76180_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 != 0.0)) && (locals.var_guard1298 != 0.0)) {
        let assign50570_e76178: f64 = (locals.var_xp * locals.var_x2);
        (assign50570_e76178, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign50570_e76180;
        locals.var_xp_dn0 = assign50570_e76180_d_n0;
        locals.var_xp_dn2 = assign50570_e76180_d_n2;
        locals.var_xp_dn4 = assign50570_e76180_d_n4;
        locals.var_xp_dn5 = assign50570_e76180_d_n5;
        locals.var_xp_dn6 = assign50570_e76180_d_n6;
        locals.var_xp_dn7 = assign50570_e76180_d_n7;
        locals.var_xp_dn8 = assign50570_e76180_d_n8;
        locals.var_xp_dn9 = assign50570_e76180_d_n9;
        locals.var_xp_dn10 = assign50570_e76180_d_n10;
        locals.var_xp_dn13 = assign50570_e76180_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign50580_e76197, assign50580_e76197_d_n0, assign50580_e76197_d_n2, assign50580_e76197_d_n4, assign50580_e76197_d_n5, assign50580_e76197_d_n6, assign50580_e76197_d_n7, assign50580_e76197_d_n8, assign50580_e76197_d_n9, assign50580_e76197_d_n10, assign50580_e76197_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 != 0.0)) && (locals.var_guard1298 != 0.0)) {
        let assign50580_e76195: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign50580_e76195, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign50580_e76197;
        locals.var_xmp_dn0 = assign50580_e76197_d_n0;
        locals.var_xmp_dn2 = assign50580_e76197_d_n2;
        locals.var_xmp_dn4 = assign50580_e76197_d_n4;
        locals.var_xmp_dn5 = assign50580_e76197_d_n5;
        locals.var_xmp_dn6 = assign50580_e76197_d_n6;
        locals.var_xmp_dn7 = assign50580_e76197_d_n7;
        locals.var_xmp_dn8 = assign50580_e76197_d_n8;
        locals.var_xmp_dn9 = assign50580_e76197_d_n9;
        locals.var_xmp_dn10 = assign50580_e76197_d_n10;
        locals.var_xmp_dn13 = assign50580_e76197_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign50590_e76214, assign50590_e76214_d_n0, assign50590_e76214_d_n2, assign50590_e76214_d_n4, assign50590_e76214_d_n5, assign50590_e76214_d_n6, assign50590_e76214_d_n7, assign50590_e76214_d_n8, assign50590_e76214_d_n9, assign50590_e76214_d_n10, assign50590_e76214_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 != 0.0)) && (locals.var_guard1298 != 0.0)) {
        let assign50590_e76212: f64 = (locals.var_xp * locals.var_x2);
        (assign50590_e76212, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign50590_e76214;
        locals.var_xp_dn0 = assign50590_e76214_d_n0;
        locals.var_xp_dn2 = assign50590_e76214_d_n2;
        locals.var_xp_dn4 = assign50590_e76214_d_n4;
        locals.var_xp_dn5 = assign50590_e76214_d_n5;
        locals.var_xp_dn6 = assign50590_e76214_d_n6;
        locals.var_xp_dn7 = assign50590_e76214_d_n7;
        locals.var_xp_dn8 = assign50590_e76214_d_n8;
        locals.var_xp_dn9 = assign50590_e76214_d_n9;
        locals.var_xp_dn10 = assign50590_e76214_d_n10;
        locals.var_xp_dn13 = assign50590_e76214_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign50600_e76231, assign50600_e76231_d_n0, assign50600_e76231_d_n2, assign50600_e76231_d_n4, assign50600_e76231_d_n5, assign50600_e76231_d_n6, assign50600_e76231_d_n7, assign50600_e76231_d_n8, assign50600_e76231_d_n9, assign50600_e76231_d_n10, assign50600_e76231_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 != 0.0)) && (locals.var_guard1298 != 0.0)) {
        let assign50600_e76229: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign50600_e76229, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign50600_e76231;
        locals.var_xmp_dn0 = assign50600_e76231_d_n0;
        locals.var_xmp_dn2 = assign50600_e76231_d_n2;
        locals.var_xmp_dn4 = assign50600_e76231_d_n4;
        locals.var_xmp_dn5 = assign50600_e76231_d_n5;
        locals.var_xmp_dn6 = assign50600_e76231_d_n6;
        locals.var_xmp_dn7 = assign50600_e76231_d_n7;
        locals.var_xmp_dn8 = assign50600_e76231_d_n8;
        locals.var_xmp_dn9 = assign50600_e76231_d_n9;
        locals.var_xmp_dn10 = assign50600_e76231_d_n10;
        locals.var_xmp_dn13 = assign50600_e76231_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign50610_e76248, assign50610_e76248_d_n0, assign50610_e76248_d_n2, assign50610_e76248_d_n4, assign50610_e76248_d_n5, assign50610_e76248_d_n6, assign50610_e76248_d_n7, assign50610_e76248_d_n8, assign50610_e76248_d_n9, assign50610_e76248_d_n10, assign50610_e76248_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 != 0.0)) && (locals.var_guard1298 != 0.0)) {
        let assign50610_e76246: f64 = (locals.var_xp + locals.var_xmp);
        (assign50610_e76246, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign50610_e76248;
        locals.var_arg_dn0 = assign50610_e76248_d_n0;
        locals.var_arg_dn2 = assign50610_e76248_d_n2;
        locals.var_arg_dn4 = assign50610_e76248_d_n4;
        locals.var_arg_dn5 = assign50610_e76248_d_n5;
        locals.var_arg_dn6 = assign50610_e76248_d_n6;
        locals.var_arg_dn7 = assign50610_e76248_d_n7;
        locals.var_arg_dn8 = assign50610_e76248_d_n8;
        locals.var_arg_dn9 = assign50610_e76248_d_n9;
        locals.var_arg_dn10 = assign50610_e76248_d_n10;
        locals.var_arg_dn13 = assign50610_e76248_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign50620_e76263, assign50620_e76263_d_n0, assign50620_e76263_d_n2, assign50620_e76263_d_n4, assign50620_e76263_d_n5, assign50620_e76263_d_n6, assign50620_e76263_d_n7, assign50620_e76263_d_n8, assign50620_e76263_d_n9, assign50620_e76263_d_n10, assign50620_e76263_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 != 0.0)) && (locals.var_guard1298 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign50620_e76263;
        locals.var_dnm_dn0 = assign50620_e76263_d_n0;
        locals.var_dnm_dn2 = assign50620_e76263_d_n2;
        locals.var_dnm_dn4 = assign50620_e76263_d_n4;
        locals.var_dnm_dn5 = assign50620_e76263_d_n5;
        locals.var_dnm_dn6 = assign50620_e76263_d_n6;
        locals.var_dnm_dn7 = assign50620_e76263_d_n7;
        locals.var_dnm_dn8 = assign50620_e76263_d_n8;
        locals.var_dnm_dn9 = assign50620_e76263_d_n9;
        locals.var_dnm_dn10 = assign50620_e76263_d_n10;
        locals.var_dnm_dn13 = assign50620_e76263_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign50630_e76278: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1299 = assign50630_e76278;
        locals.var_guard1299_rv = 0.0;

        let assign50640_e76281: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1300 = assign50640_e76281;
        locals.var_guard1300_rv = 0.0;

        let (assign50650_e76300,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 != 0.0)) && (locals.var_guard1298 != 0.0)) && (locals.var_guard1299 != 0.0)) && (locals.var_guard1300 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign50650_e76300;
        locals.var_mm_rv = 0.0;

        let assign50660_e76303: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1301 = assign50660_e76303;
        locals.var_guard1301_rv = 0.0;

        let (assign50670_e76325,) = {
    if (((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 != 0.0)) && (locals.var_guard1298 != 0.0)) && (locals.var_guard1299 != 0.0)) && (locals.var_guard1300 == 0.0)) && (locals.var_guard1301 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign50670_e76325;
        locals.var_mm_rv = 0.0;

        let assign50680_e76328: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1302 = assign50680_e76328;
        locals.var_guard1302_rv = 0.0;

        let (assign50690_e76353,) = {
    if ((((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 != 0.0)) && (locals.var_guard1298 != 0.0)) && (locals.var_guard1299 != 0.0)) && (locals.var_guard1300 == 0.0)) && (locals.var_guard1301 == 0.0)) && (locals.var_guard1302 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign50690_e76353;
        locals.var_mm_rv = 0.0;

        let assign50700_e76356: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1303 = assign50700_e76356;
        locals.var_guard1303_rv = 0.0;

        let (assign50710_e76384,) = {
    if (((((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 != 0.0)) && (locals.var_guard1298 != 0.0)) && (locals.var_guard1299 != 0.0)) && (locals.var_guard1300 == 0.0)) && (locals.var_guard1301 == 0.0)) && (locals.var_guard1302 == 0.0)) && (locals.var_guard1303 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign50710_e76384;
        locals.var_mm_rv = 0.0;

        let (assign50720_e76401,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 != 0.0)) && (locals.var_guard1298 != 0.0)) && (locals.var_guard1299 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign50720_e76401;
        locals.var_m0_rv = 0.0;

        let mut assign50730_loop_guard: usize = 0;
        while {
            let assign50730_cond_e76419: f64 = if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 != 0.0)) && (locals.var_guard1298 != 0.0)) && (locals.var_guard1299 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign50730_cond_e76419 != 0.0
        } {
            assign50730_loop_guard += 1;
            assert!(assign50730_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign50730_body0_e76437, assign50730_body0_e76437_d_n0, assign50730_body0_e76437_d_n2, assign50730_body0_e76437_d_n4, assign50730_body0_e76437_d_n5, assign50730_body0_e76437_d_n6, assign50730_body0_e76437_d_n7, assign50730_body0_e76437_d_n8, assign50730_body0_e76437_d_n9, assign50730_body0_e76437_d_n10, assign50730_body0_e76437_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 != 0.0)) && (locals.var_guard1298 != 0.0)) && (locals.var_guard1299 != 0.0)) {
        let assign50730_body0_e76435: f64 = (locals.var_dnm).sqrt();
        (assign50730_body0_e76435, (locals.var_dnm_dn0 / (2.0 * assign50730_body0_e76435)), (locals.var_dnm_dn2 / (2.0 * assign50730_body0_e76435)), (locals.var_dnm_dn4 / (2.0 * assign50730_body0_e76435)), (locals.var_dnm_dn5 / (2.0 * assign50730_body0_e76435)), (locals.var_dnm_dn6 / (2.0 * assign50730_body0_e76435)), (locals.var_dnm_dn7 / (2.0 * assign50730_body0_e76435)), (locals.var_dnm_dn8 / (2.0 * assign50730_body0_e76435)), (locals.var_dnm_dn9 / (2.0 * assign50730_body0_e76435)), (locals.var_dnm_dn10 / (2.0 * assign50730_body0_e76435)), (locals.var_dnm_dn13 / (2.0 * assign50730_body0_e76435)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign50730_body0_e76437;
            locals.var_dnm_dn0 = assign50730_body0_e76437_d_n0;
            locals.var_dnm_dn2 = assign50730_body0_e76437_d_n2;
            locals.var_dnm_dn4 = assign50730_body0_e76437_d_n4;
            locals.var_dnm_dn5 = assign50730_body0_e76437_d_n5;
            locals.var_dnm_dn6 = assign50730_body0_e76437_d_n6;
            locals.var_dnm_dn7 = assign50730_body0_e76437_d_n7;
            locals.var_dnm_dn8 = assign50730_body0_e76437_d_n8;
            locals.var_dnm_dn9 = assign50730_body0_e76437_d_n9;
            locals.var_dnm_dn10 = assign50730_body0_e76437_d_n10;
            locals.var_dnm_dn13 = assign50730_body0_e76437_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign50730_body1_e76456,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 != 0.0)) && (locals.var_guard1298 != 0.0)) && (locals.var_guard1299 != 0.0)) {
        let assign50730_body1_e76454: f64 = (locals.var_m0 + 1.0);
        (assign50730_body1_e76454,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign50730_body1_e76456;
            locals.var_m0_rv = 0.0;
        }

        let (assign50740_e76485, assign50740_e76485_d_n0, assign50740_e76485_d_n2, assign50740_e76485_d_n4, assign50740_e76485_d_n5, assign50740_e76485_d_n6, assign50740_e76485_d_n7, assign50740_e76485_d_n8, assign50740_e76485_d_n9, assign50740_e76485_d_n10, assign50740_e76485_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 != 0.0)) && (locals.var_guard1298 != 0.0)) && (locals.var_guard1299 == 0.0)) {
        let (assign50740_e76483, assign50740_e76483_d_n0, assign50740_e76483_d_n2, assign50740_e76483_d_n4, assign50740_e76483_d_n5, assign50740_e76483_d_n6, assign50740_e76483_d_n7, assign50740_e76483_d_n8, assign50740_e76483_d_n9, assign50740_e76483_d_n10, assign50740_e76483_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign50740_e76480: f64 = (2.0 * 2.0);
                let assign50740_e76481: f64 = (1.0 / assign50740_e76480);
                let assign50740_e76482: f64 = (locals.var_dnm).powf(assign50740_e76481);
                (assign50740_e76482, if 0.0 == 0.0 && ((assign50740_e76481) as f64).is_finite() && ((assign50740_e76481) as f64).fract() == 0.0 { if assign50740_e76481 == 0.0 { 0.0 } else { (assign50740_e76481 * ((locals.var_dnm).powf(assign50740_e76481 - 1.0) * locals.var_dnm_dn0)) } } else { (assign50740_e76482 * (assign50740_e76481 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign50740_e76481) as f64).is_finite() && ((assign50740_e76481) as f64).fract() == 0.0 { if assign50740_e76481 == 0.0 { 0.0 } else { (assign50740_e76481 * ((locals.var_dnm).powf(assign50740_e76481 - 1.0) * locals.var_dnm_dn2)) } } else { (assign50740_e76482 * (assign50740_e76481 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign50740_e76481) as f64).is_finite() && ((assign50740_e76481) as f64).fract() == 0.0 { if assign50740_e76481 == 0.0 { 0.0 } else { (assign50740_e76481 * ((locals.var_dnm).powf(assign50740_e76481 - 1.0) * locals.var_dnm_dn4)) } } else { (assign50740_e76482 * (assign50740_e76481 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign50740_e76481) as f64).is_finite() && ((assign50740_e76481) as f64).fract() == 0.0 { if assign50740_e76481 == 0.0 { 0.0 } else { (assign50740_e76481 * ((locals.var_dnm).powf(assign50740_e76481 - 1.0) * locals.var_dnm_dn5)) } } else { (assign50740_e76482 * (assign50740_e76481 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign50740_e76481) as f64).is_finite() && ((assign50740_e76481) as f64).fract() == 0.0 { if assign50740_e76481 == 0.0 { 0.0 } else { (assign50740_e76481 * ((locals.var_dnm).powf(assign50740_e76481 - 1.0) * locals.var_dnm_dn6)) } } else { (assign50740_e76482 * (assign50740_e76481 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign50740_e76481) as f64).is_finite() && ((assign50740_e76481) as f64).fract() == 0.0 { if assign50740_e76481 == 0.0 { 0.0 } else { (assign50740_e76481 * ((locals.var_dnm).powf(assign50740_e76481 - 1.0) * locals.var_dnm_dn7)) } } else { (assign50740_e76482 * (assign50740_e76481 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign50740_e76481) as f64).is_finite() && ((assign50740_e76481) as f64).fract() == 0.0 { if assign50740_e76481 == 0.0 { 0.0 } else { (assign50740_e76481 * ((locals.var_dnm).powf(assign50740_e76481 - 1.0) * locals.var_dnm_dn8)) } } else { (assign50740_e76482 * (assign50740_e76481 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign50740_e76481) as f64).is_finite() && ((assign50740_e76481) as f64).fract() == 0.0 { if assign50740_e76481 == 0.0 { 0.0 } else { (assign50740_e76481 * ((locals.var_dnm).powf(assign50740_e76481 - 1.0) * locals.var_dnm_dn9)) } } else { (assign50740_e76482 * (assign50740_e76481 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign50740_e76481) as f64).is_finite() && ((assign50740_e76481) as f64).fract() == 0.0 { if assign50740_e76481 == 0.0 { 0.0 } else { (assign50740_e76481 * ((locals.var_dnm).powf(assign50740_e76481 - 1.0) * locals.var_dnm_dn10)) } } else { (assign50740_e76482 * (assign50740_e76481 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign50740_e76481) as f64).is_finite() && ((assign50740_e76481) as f64).fract() == 0.0 { if assign50740_e76481 == 0.0 { 0.0 } else { (assign50740_e76481 * ((locals.var_dnm).powf(assign50740_e76481 - 1.0) * locals.var_dnm_dn13)) } } else { (assign50740_e76482 * (assign50740_e76481 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign50740_e76483, assign50740_e76483_d_n0, assign50740_e76483_d_n2, assign50740_e76483_d_n4, assign50740_e76483_d_n5, assign50740_e76483_d_n6, assign50740_e76483_d_n7, assign50740_e76483_d_n8, assign50740_e76483_d_n9, assign50740_e76483_d_n10, assign50740_e76483_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign50740_e76485;
        locals.var_dnm_dn0 = assign50740_e76485_d_n0;
        locals.var_dnm_dn2 = assign50740_e76485_d_n2;
        locals.var_dnm_dn4 = assign50740_e76485_d_n4;
        locals.var_dnm_dn5 = assign50740_e76485_d_n5;
        locals.var_dnm_dn6 = assign50740_e76485_d_n6;
        locals.var_dnm_dn7 = assign50740_e76485_d_n7;
        locals.var_dnm_dn8 = assign50740_e76485_d_n8;
        locals.var_dnm_dn9 = assign50740_e76485_d_n9;
        locals.var_dnm_dn10 = assign50740_e76485_d_n10;
        locals.var_dnm_dn13 = assign50740_e76485_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign50750_e76502, assign50750_e76502_d_n0, assign50750_e76502_d_n2, assign50750_e76502_d_n4, assign50750_e76502_d_n5, assign50750_e76502_d_n6, assign50750_e76502_d_n7, assign50750_e76502_d_n8, assign50750_e76502_d_n9, assign50750_e76502_d_n10, assign50750_e76502_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 != 0.0)) && (locals.var_guard1298 != 0.0)) {
        let assign50750_e76500: f64 = (1.0 / locals.var_dnm);
        (assign50750_e76500, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign50750_e76502;
        locals.var_dnm_dn0 = assign50750_e76502_d_n0;
        locals.var_dnm_dn2 = assign50750_e76502_d_n2;
        locals.var_dnm_dn4 = assign50750_e76502_d_n4;
        locals.var_dnm_dn5 = assign50750_e76502_d_n5;
        locals.var_dnm_dn6 = assign50750_e76502_d_n6;
        locals.var_dnm_dn7 = assign50750_e76502_d_n7;
        locals.var_dnm_dn8 = assign50750_e76502_d_n8;
        locals.var_dnm_dn9 = assign50750_e76502_d_n9;
        locals.var_dnm_dn10 = assign50750_e76502_d_n10;
        locals.var_dnm_dn13 = assign50750_e76502_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign50760_e76523, assign50760_e76523_d_n0, assign50760_e76523_d_n2, assign50760_e76523_d_n4, assign50760_e76523_d_n5, assign50760_e76523_d_n6, assign50760_e76523_d_n7, assign50760_e76523_d_n8, assign50760_e76523_d_n9, assign50760_e76523_d_n10, assign50760_e76523_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 != 0.0)) && (locals.var_guard1298 != 0.0)) {
        let assign50760_e76518: f64 = (10.0 * 2.220446049250313e-16);
        let assign50760_e76519: f64 = (locals.var_tmf1 * assign50760_e76518);
        let assign50760_e76521: f64 = (assign50760_e76519 * locals.var_dnm);
        (assign50760_e76521, (((locals.var_tmf1_dn0 * assign50760_e76518) * locals.var_dnm) + (assign50760_e76519 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * assign50760_e76518) * locals.var_dnm) + (assign50760_e76519 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * assign50760_e76518) * locals.var_dnm) + (assign50760_e76519 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * assign50760_e76518) * locals.var_dnm) + (assign50760_e76519 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * assign50760_e76518) * locals.var_dnm) + (assign50760_e76519 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * assign50760_e76518) * locals.var_dnm) + (assign50760_e76519 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * assign50760_e76518) * locals.var_dnm) + (assign50760_e76519 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * assign50760_e76518) * locals.var_dnm) + (assign50760_e76519 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * assign50760_e76518) * locals.var_dnm) + (assign50760_e76519 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * assign50760_e76518) * locals.var_dnm) + (assign50760_e76519 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign50760_e76523;
        locals.var_tmf0_dn0 = assign50760_e76523_d_n0;
        locals.var_tmf0_dn2 = assign50760_e76523_d_n2;
        locals.var_tmf0_dn4 = assign50760_e76523_d_n4;
        locals.var_tmf0_dn5 = assign50760_e76523_d_n5;
        locals.var_tmf0_dn6 = assign50760_e76523_d_n6;
        locals.var_tmf0_dn7 = assign50760_e76523_d_n7;
        locals.var_tmf0_dn8 = assign50760_e76523_d_n8;
        locals.var_tmf0_dn9 = assign50760_e76523_d_n9;
        locals.var_tmf0_dn10 = assign50760_e76523_d_n10;
        locals.var_tmf0_dn13 = assign50760_e76523_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign50770_e76546, assign50770_e76546_d_n0, assign50770_e76546_d_n2, assign50770_e76546_d_n4, assign50770_e76546_d_n5, assign50770_e76546_d_n6, assign50770_e76546_d_n7, assign50770_e76546_d_n8, assign50770_e76546_d_n9, assign50770_e76546_d_n10, assign50770_e76546_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 != 0.0)) && (locals.var_guard1298 != 0.0)) {
        let assign50770_e76538: f64 = (10.0 * 2.220446049250313e-16);
        let assign50770_e76540: f64 = (assign50770_e76538 * locals.var_xmp);
        let assign50770_e76542: f64 = (assign50770_e76540 * locals.var_dnm);
        let assign50770_e76544: f64 = (assign50770_e76542 / locals.var_arg);
        (assign50770_e76544, ((((((assign50770_e76538 * locals.var_xmp_dn0) * locals.var_dnm) + (assign50770_e76540 * locals.var_dnm_dn0)) * locals.var_arg) - (assign50770_e76542 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((assign50770_e76538 * locals.var_xmp_dn2) * locals.var_dnm) + (assign50770_e76540 * locals.var_dnm_dn2)) * locals.var_arg) - (assign50770_e76542 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((assign50770_e76538 * locals.var_xmp_dn4) * locals.var_dnm) + (assign50770_e76540 * locals.var_dnm_dn4)) * locals.var_arg) - (assign50770_e76542 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((assign50770_e76538 * locals.var_xmp_dn5) * locals.var_dnm) + (assign50770_e76540 * locals.var_dnm_dn5)) * locals.var_arg) - (assign50770_e76542 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((assign50770_e76538 * locals.var_xmp_dn6) * locals.var_dnm) + (assign50770_e76540 * locals.var_dnm_dn6)) * locals.var_arg) - (assign50770_e76542 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((assign50770_e76538 * locals.var_xmp_dn7) * locals.var_dnm) + (assign50770_e76540 * locals.var_dnm_dn7)) * locals.var_arg) - (assign50770_e76542 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((assign50770_e76538 * locals.var_xmp_dn8) * locals.var_dnm) + (assign50770_e76540 * locals.var_dnm_dn8)) * locals.var_arg) - (assign50770_e76542 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((assign50770_e76538 * locals.var_xmp_dn9) * locals.var_dnm) + (assign50770_e76540 * locals.var_dnm_dn9)) * locals.var_arg) - (assign50770_e76542 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((assign50770_e76538 * locals.var_xmp_dn10) * locals.var_dnm) + (assign50770_e76540 * locals.var_dnm_dn10)) * locals.var_arg) - (assign50770_e76542 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((assign50770_e76538 * locals.var_xmp_dn13) * locals.var_dnm) + (assign50770_e76540 * locals.var_dnm_dn13)) * locals.var_arg) - (assign50770_e76542 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign50770_e76546;
        locals.var_t0_dn0 = assign50770_e76546_d_n0;
        locals.var_t0_dn2 = assign50770_e76546_d_n2;
        locals.var_t0_dn4 = assign50770_e76546_d_n4;
        locals.var_t0_dn5 = assign50770_e76546_d_n5;
        locals.var_t0_dn6 = assign50770_e76546_d_n6;
        locals.var_t0_dn7 = assign50770_e76546_d_n7;
        locals.var_t0_dn8 = assign50770_e76546_d_n8;
        locals.var_t0_dn9 = assign50770_e76546_d_n9;
        locals.var_t0_dn10 = assign50770_e76546_d_n10;
        locals.var_t0_dn13 = assign50770_e76546_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign50780_e76573, assign50780_e76573_d_n0, assign50780_e76573_d_n2, assign50780_e76573_d_n4, assign50780_e76573_d_n5, assign50780_e76573_d_n6, assign50780_e76573_d_n7, assign50780_e76573_d_n8, assign50780_e76573_d_n9, assign50780_e76573_d_n10, assign50780_e76573_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 != 0.0)) && (locals.var_guard1298 != 0.0)) {
        let assign50780_e76561: f64 = (locals.var_ps0 + locals.var_vds);
        let assign50780_e76564: f64 = (10.0 * 2.220446049250313e-16);
        let assign50780_e76565: f64 = (assign50780_e76561 - assign50780_e76564);
        let assign50780_e76568: f64 = (10.0 * 2.220446049250313e-16);
        let assign50780_e76569: f64 = (assign50780_e76565 - assign50780_e76568);
        let assign50780_e76571: f64 = (assign50780_e76569 + locals.var_tmf0);
        (assign50780_e76571, ((locals.var_ps0_dn0 + locals.var_vds_dn0) + locals.var_tmf0_dn0), ((locals.var_ps0_dn2 + locals.var_vds_dn2) + locals.var_tmf0_dn2), ((locals.var_ps0_dn4 + locals.var_vds_dn4) + locals.var_tmf0_dn4), ((locals.var_ps0_dn5 + locals.var_vds_dn5) + locals.var_tmf0_dn5), ((locals.var_ps0_dn6 + locals.var_vds_dn6) + locals.var_tmf0_dn6), ((locals.var_ps0_dn7 + locals.var_vds_dn7) + locals.var_tmf0_dn7), ((locals.var_ps0_dn8 + locals.var_vds_dn8) + locals.var_tmf0_dn8), ((locals.var_ps0_dn9 + locals.var_vds_dn9) + locals.var_tmf0_dn9), ((locals.var_ps0_dn10 + locals.var_vds_dn10) + locals.var_tmf0_dn10), ((locals.var_ps0_dn13 + locals.var_vds_dn13) + locals.var_tmf0_dn13),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn13,)
    }
};
        locals.var_psdl = assign50780_e76573;
        locals.var_psdl_dn0 = assign50780_e76573_d_n0;
        locals.var_psdl_dn2 = assign50780_e76573_d_n2;
        locals.var_psdl_dn4 = assign50780_e76573_d_n4;
        locals.var_psdl_dn5 = assign50780_e76573_d_n5;
        locals.var_psdl_dn6 = assign50780_e76573_d_n6;
        locals.var_psdl_dn7 = assign50780_e76573_d_n7;
        locals.var_psdl_dn8 = assign50780_e76573_d_n8;
        locals.var_psdl_dn9 = assign50780_e76573_d_n9;
        locals.var_psdl_dn10 = assign50780_e76573_d_n10;
        locals.var_psdl_dn13 = assign50780_e76573_d_n13;
        locals.var_psdl_rv = 0.0;

        let (assign50790_e76588, assign50790_e76588_d_n0, assign50790_e76588_d_n2, assign50790_e76588_d_n4, assign50790_e76588_d_n5, assign50790_e76588_d_n6, assign50790_e76588_d_n7, assign50790_e76588_d_n8, assign50790_e76588_d_n9, assign50790_e76588_d_n10, assign50790_e76588_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 != 0.0)) && (locals.var_guard1298 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign50790_e76588;
        locals.var_t0_dn0 = assign50790_e76588_d_n0;
        locals.var_t0_dn2 = assign50790_e76588_d_n2;
        locals.var_t0_dn4 = assign50790_e76588_d_n4;
        locals.var_t0_dn5 = assign50790_e76588_d_n5;
        locals.var_t0_dn6 = assign50790_e76588_d_n6;
        locals.var_t0_dn7 = assign50790_e76588_d_n7;
        locals.var_t0_dn8 = assign50790_e76588_d_n8;
        locals.var_t0_dn9 = assign50790_e76588_d_n9;
        locals.var_t0_dn10 = assign50790_e76588_d_n10;
        locals.var_t0_dn13 = assign50790_e76588_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign50800_e76604, assign50800_e76604_d_n0, assign50800_e76604_d_n2, assign50800_e76604_d_n4, assign50800_e76604_d_n5, assign50800_e76604_d_n6, assign50800_e76604_d_n7, assign50800_e76604_d_n8, assign50800_e76604_d_n9, assign50800_e76604_d_n10, assign50800_e76604_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 != 0.0)) && (locals.var_guard1298 == 0.0)) {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn13,)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn13,)
    }
};
        locals.var_psdl = assign50800_e76604;
        locals.var_psdl_dn0 = assign50800_e76604_d_n0;
        locals.var_psdl_dn2 = assign50800_e76604_d_n2;
        locals.var_psdl_dn4 = assign50800_e76604_d_n4;
        locals.var_psdl_dn5 = assign50800_e76604_d_n5;
        locals.var_psdl_dn6 = assign50800_e76604_d_n6;
        locals.var_psdl_dn7 = assign50800_e76604_d_n7;
        locals.var_psdl_dn8 = assign50800_e76604_d_n8;
        locals.var_psdl_dn9 = assign50800_e76604_d_n9;
        locals.var_psdl_dn10 = assign50800_e76604_d_n10;
        locals.var_psdl_dn13 = assign50800_e76604_d_n13;
        locals.var_psdl_rv = 0.0;

        let (assign50810_e76620, assign50810_e76620_d_n0, assign50810_e76620_d_n2, assign50810_e76620_d_n4, assign50810_e76620_d_n5, assign50810_e76620_d_n6, assign50810_e76620_d_n7, assign50810_e76620_d_n8, assign50810_e76620_d_n9, assign50810_e76620_d_n10, assign50810_e76620_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 != 0.0)) && (locals.var_guard1298 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign50810_e76620;
        locals.var_t0_dn0 = assign50810_e76620_d_n0;
        locals.var_t0_dn2 = assign50810_e76620_d_n2;
        locals.var_t0_dn4 = assign50810_e76620_d_n4;
        locals.var_t0_dn5 = assign50810_e76620_d_n5;
        locals.var_t0_dn6 = assign50810_e76620_d_n6;
        locals.var_t0_dn7 = assign50810_e76620_d_n7;
        locals.var_t0_dn8 = assign50810_e76620_d_n8;
        locals.var_t0_dn9 = assign50810_e76620_d_n9;
        locals.var_t0_dn10 = assign50810_e76620_d_n10;
        locals.var_t0_dn13 = assign50810_e76620_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign50820_e76636, assign50820_e76636_d_n0, assign50820_e76636_d_n2, assign50820_e76636_d_n4, assign50820_e76636_d_n5, assign50820_e76636_d_n6, assign50820_e76636_d_n7, assign50820_e76636_d_n8, assign50820_e76636_d_n9, assign50820_e76636_d_n10, assign50820_e76636_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) {
        let assign50820_e76634: f64 = (locals.var_psl - locals.var_vbsc__blk1117);
        (assign50820_e76634, (locals.var_psl_dn0 - locals.var_vbsc__blk1117_dn0), (locals.var_psl_dn2 - locals.var_vbsc__blk1117_dn2), (locals.var_psl_dn4 - locals.var_vbsc__blk1117_dn4), (locals.var_psl_dn5 - locals.var_vbsc__blk1117_dn5), (locals.var_psl_dn6 - locals.var_vbsc__blk1117_dn6), (locals.var_psl_dn7 - locals.var_vbsc__blk1117_dn7), (locals.var_psl_dn8 - locals.var_vbsc__blk1117_dn8), (locals.var_psl_dn9 - locals.var_vbsc__blk1117_dn9), (locals.var_psl_dn10 - locals.var_vbsc__blk1117_dn10), (locals.var_psl_dn13 - locals.var_vbsc__blk1117_dn13),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn13,)
    }
};
        locals.var_t8 = assign50820_e76636;
        locals.var_t8_dn0 = assign50820_e76636_d_n0;
        locals.var_t8_dn2 = assign50820_e76636_d_n2;
        locals.var_t8_dn4 = assign50820_e76636_d_n4;
        locals.var_t8_dn5 = assign50820_e76636_d_n5;
        locals.var_t8_dn6 = assign50820_e76636_d_n6;
        locals.var_t8_dn7 = assign50820_e76636_d_n7;
        locals.var_t8_dn8 = assign50820_e76636_d_n8;
        locals.var_t8_dn9 = assign50820_e76636_d_n9;
        locals.var_t8_dn10 = assign50820_e76636_d_n10;
        locals.var_t8_dn13 = assign50820_e76636_d_n13;
        locals.var_t8_rv = 0.0;

        let assign50830_e76640: f64 = (-locals.var_vbsc__blk1117);
        let assign50830_e76642: f64 = (assign50830_e76640 + 0.8);
        let assign50830_e76643: f64 = (0.2 + assign50830_e76642);
        let assign50830_e76646: f64 = (-locals.var_vbsc__blk1117);
        let assign50830_e76648: f64 = (assign50830_e76646 + 0.8);
        let assign50830_e76651: f64 = if ((locals.var_t8 < assign50830_e76643) && (assign50830_e76648 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1304 = assign50830_e76651;
        locals.var_guard1304_rv = 0.0;

        let (assign50840_e76674, assign50840_e76674_d_n0, assign50840_e76674_d_n2, assign50840_e76674_d_n4, assign50840_e76674_d_n5, assign50840_e76674_d_n6, assign50840_e76674_d_n7, assign50840_e76674_d_n8, assign50840_e76674_d_n9, assign50840_e76674_d_n10, assign50840_e76674_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1304 != 0.0)) {
        let assign50840_e76667: f64 = (-locals.var_vbsc__blk1117);
        let assign50840_e76669: f64 = (assign50840_e76667 + 0.8);
        let assign50840_e76670: f64 = (0.2 + assign50840_e76669);
        let assign50840_e76672: f64 = (assign50840_e76670 - locals.var_t8);
        (assign50840_e76672, ((-locals.var_vbsc__blk1117_dn0) - locals.var_t8_dn0), ((-locals.var_vbsc__blk1117_dn2) - locals.var_t8_dn2), ((-locals.var_vbsc__blk1117_dn4) - locals.var_t8_dn4), ((-locals.var_vbsc__blk1117_dn5) - locals.var_t8_dn5), ((-locals.var_vbsc__blk1117_dn6) - locals.var_t8_dn6), ((-locals.var_vbsc__blk1117_dn7) - locals.var_t8_dn7), ((-locals.var_vbsc__blk1117_dn8) - locals.var_t8_dn8), ((-locals.var_vbsc__blk1117_dn9) - locals.var_t8_dn9), ((-locals.var_vbsc__blk1117_dn10) - locals.var_t8_dn10), ((-locals.var_vbsc__blk1117_dn13) - locals.var_t8_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign50840_e76674;
        locals.var_tmf1_dn0 = assign50840_e76674_d_n0;
        locals.var_tmf1_dn2 = assign50840_e76674_d_n2;
        locals.var_tmf1_dn4 = assign50840_e76674_d_n4;
        locals.var_tmf1_dn5 = assign50840_e76674_d_n5;
        locals.var_tmf1_dn6 = assign50840_e76674_d_n6;
        locals.var_tmf1_dn7 = assign50840_e76674_d_n7;
        locals.var_tmf1_dn8 = assign50840_e76674_d_n8;
        locals.var_tmf1_dn9 = assign50840_e76674_d_n9;
        locals.var_tmf1_dn10 = assign50840_e76674_d_n10;
        locals.var_tmf1_dn13 = assign50840_e76674_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign50850_e76692, assign50850_e76692_d_n0, assign50850_e76692_d_n2, assign50850_e76692_d_n4, assign50850_e76692_d_n5, assign50850_e76692_d_n6, assign50850_e76692_d_n7, assign50850_e76692_d_n8, assign50850_e76692_d_n9, assign50850_e76692_d_n10, assign50850_e76692_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1304 != 0.0)) {
        let assign50850_e76690: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign50850_e76690, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign50850_e76692;
        locals.var_x2_dn0 = assign50850_e76692_d_n0;
        locals.var_x2_dn2 = assign50850_e76692_d_n2;
        locals.var_x2_dn4 = assign50850_e76692_d_n4;
        locals.var_x2_dn5 = assign50850_e76692_d_n5;
        locals.var_x2_dn6 = assign50850_e76692_d_n6;
        locals.var_x2_dn7 = assign50850_e76692_d_n7;
        locals.var_x2_dn8 = assign50850_e76692_d_n8;
        locals.var_x2_dn9 = assign50850_e76692_d_n9;
        locals.var_x2_dn10 = assign50850_e76692_d_n10;
        locals.var_x2_dn13 = assign50850_e76692_d_n13;
        locals.var_x2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_176(
        locals: &mut StampLocals,
    ) {
        let (assign50860_e76716, assign50860_e76716_d_n0, assign50860_e76716_d_n2, assign50860_e76716_d_n4, assign50860_e76716_d_n5, assign50860_e76716_d_n6, assign50860_e76716_d_n7, assign50860_e76716_d_n8, assign50860_e76716_d_n9, assign50860_e76716_d_n10, assign50860_e76716_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1304 != 0.0)) {
        let assign50860_e76707: f64 = (-locals.var_vbsc__blk1117);
        let assign50860_e76709: f64 = (assign50860_e76707 + 0.8);
        let assign50860_e76711: f64 = (-locals.var_vbsc__blk1117);
        let assign50860_e76713: f64 = (assign50860_e76711 + 0.8);
        let assign50860_e76714: f64 = (assign50860_e76709 * assign50860_e76713);
        (assign50860_e76714, (((-locals.var_vbsc__blk1117_dn0) * assign50860_e76713) + (assign50860_e76709 * (-locals.var_vbsc__blk1117_dn0))), (((-locals.var_vbsc__blk1117_dn2) * assign50860_e76713) + (assign50860_e76709 * (-locals.var_vbsc__blk1117_dn2))), (((-locals.var_vbsc__blk1117_dn4) * assign50860_e76713) + (assign50860_e76709 * (-locals.var_vbsc__blk1117_dn4))), (((-locals.var_vbsc__blk1117_dn5) * assign50860_e76713) + (assign50860_e76709 * (-locals.var_vbsc__blk1117_dn5))), (((-locals.var_vbsc__blk1117_dn6) * assign50860_e76713) + (assign50860_e76709 * (-locals.var_vbsc__blk1117_dn6))), (((-locals.var_vbsc__blk1117_dn7) * assign50860_e76713) + (assign50860_e76709 * (-locals.var_vbsc__blk1117_dn7))), (((-locals.var_vbsc__blk1117_dn8) * assign50860_e76713) + (assign50860_e76709 * (-locals.var_vbsc__blk1117_dn8))), (((-locals.var_vbsc__blk1117_dn9) * assign50860_e76713) + (assign50860_e76709 * (-locals.var_vbsc__blk1117_dn9))), (((-locals.var_vbsc__blk1117_dn10) * assign50860_e76713) + (assign50860_e76709 * (-locals.var_vbsc__blk1117_dn10))), (((-locals.var_vbsc__blk1117_dn13) * assign50860_e76713) + (assign50860_e76709 * (-locals.var_vbsc__blk1117_dn13))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign50860_e76716;
        locals.var_xmax2_dn0 = assign50860_e76716_d_n0;
        locals.var_xmax2_dn2 = assign50860_e76716_d_n2;
        locals.var_xmax2_dn4 = assign50860_e76716_d_n4;
        locals.var_xmax2_dn5 = assign50860_e76716_d_n5;
        locals.var_xmax2_dn6 = assign50860_e76716_d_n6;
        locals.var_xmax2_dn7 = assign50860_e76716_d_n7;
        locals.var_xmax2_dn8 = assign50860_e76716_d_n8;
        locals.var_xmax2_dn9 = assign50860_e76716_d_n9;
        locals.var_xmax2_dn10 = assign50860_e76716_d_n10;
        locals.var_xmax2_dn13 = assign50860_e76716_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign50870_e76732, assign50870_e76732_d_n0, assign50870_e76732_d_n2, assign50870_e76732_d_n4, assign50870_e76732_d_n5, assign50870_e76732_d_n6, assign50870_e76732_d_n7, assign50870_e76732_d_n8, assign50870_e76732_d_n9, assign50870_e76732_d_n10, assign50870_e76732_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1304 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign50870_e76732;
        locals.var_xp_dn0 = assign50870_e76732_d_n0;
        locals.var_xp_dn2 = assign50870_e76732_d_n2;
        locals.var_xp_dn4 = assign50870_e76732_d_n4;
        locals.var_xp_dn5 = assign50870_e76732_d_n5;
        locals.var_xp_dn6 = assign50870_e76732_d_n6;
        locals.var_xp_dn7 = assign50870_e76732_d_n7;
        locals.var_xp_dn8 = assign50870_e76732_d_n8;
        locals.var_xp_dn9 = assign50870_e76732_d_n9;
        locals.var_xp_dn10 = assign50870_e76732_d_n10;
        locals.var_xp_dn13 = assign50870_e76732_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign50880_e76748, assign50880_e76748_d_n0, assign50880_e76748_d_n2, assign50880_e76748_d_n4, assign50880_e76748_d_n5, assign50880_e76748_d_n6, assign50880_e76748_d_n7, assign50880_e76748_d_n8, assign50880_e76748_d_n9, assign50880_e76748_d_n10, assign50880_e76748_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1304 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign50880_e76748;
        locals.var_xmp_dn0 = assign50880_e76748_d_n0;
        locals.var_xmp_dn2 = assign50880_e76748_d_n2;
        locals.var_xmp_dn4 = assign50880_e76748_d_n4;
        locals.var_xmp_dn5 = assign50880_e76748_d_n5;
        locals.var_xmp_dn6 = assign50880_e76748_d_n6;
        locals.var_xmp_dn7 = assign50880_e76748_d_n7;
        locals.var_xmp_dn8 = assign50880_e76748_d_n8;
        locals.var_xmp_dn9 = assign50880_e76748_d_n9;
        locals.var_xmp_dn10 = assign50880_e76748_d_n10;
        locals.var_xmp_dn13 = assign50880_e76748_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign50890_e76764,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1304 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign50890_e76764;
        locals.var_m0_rv = 0.0;

        let (assign50900_e76780,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1304 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign50900_e76780;
        locals.var_mm_rv = 0.0;

        let (assign50910_e76796, assign50910_e76796_d_n0, assign50910_e76796_d_n2, assign50910_e76796_d_n4, assign50910_e76796_d_n5, assign50910_e76796_d_n6, assign50910_e76796_d_n7, assign50910_e76796_d_n8, assign50910_e76796_d_n9, assign50910_e76796_d_n10, assign50910_e76796_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1304 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign50910_e76796;
        locals.var_arg_dn0 = assign50910_e76796_d_n0;
        locals.var_arg_dn2 = assign50910_e76796_d_n2;
        locals.var_arg_dn4 = assign50910_e76796_d_n4;
        locals.var_arg_dn5 = assign50910_e76796_d_n5;
        locals.var_arg_dn6 = assign50910_e76796_d_n6;
        locals.var_arg_dn7 = assign50910_e76796_d_n7;
        locals.var_arg_dn8 = assign50910_e76796_d_n8;
        locals.var_arg_dn9 = assign50910_e76796_d_n9;
        locals.var_arg_dn10 = assign50910_e76796_d_n10;
        locals.var_arg_dn13 = assign50910_e76796_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign50920_e76812, assign50920_e76812_d_n0, assign50920_e76812_d_n2, assign50920_e76812_d_n4, assign50920_e76812_d_n5, assign50920_e76812_d_n6, assign50920_e76812_d_n7, assign50920_e76812_d_n8, assign50920_e76812_d_n9, assign50920_e76812_d_n10, assign50920_e76812_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1304 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign50920_e76812;
        locals.var_dnm_dn0 = assign50920_e76812_d_n0;
        locals.var_dnm_dn2 = assign50920_e76812_d_n2;
        locals.var_dnm_dn4 = assign50920_e76812_d_n4;
        locals.var_dnm_dn5 = assign50920_e76812_d_n5;
        locals.var_dnm_dn6 = assign50920_e76812_d_n6;
        locals.var_dnm_dn7 = assign50920_e76812_d_n7;
        locals.var_dnm_dn8 = assign50920_e76812_d_n8;
        locals.var_dnm_dn9 = assign50920_e76812_d_n9;
        locals.var_dnm_dn10 = assign50920_e76812_d_n10;
        locals.var_dnm_dn13 = assign50920_e76812_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign50930_e76830, assign50930_e76830_d_n0, assign50930_e76830_d_n2, assign50930_e76830_d_n4, assign50930_e76830_d_n5, assign50930_e76830_d_n6, assign50930_e76830_d_n7, assign50930_e76830_d_n8, assign50930_e76830_d_n9, assign50930_e76830_d_n10, assign50930_e76830_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1304 != 0.0)) {
        let assign50930_e76828: f64 = (locals.var_xp * locals.var_x2);
        (assign50930_e76828, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign50930_e76830;
        locals.var_xp_dn0 = assign50930_e76830_d_n0;
        locals.var_xp_dn2 = assign50930_e76830_d_n2;
        locals.var_xp_dn4 = assign50930_e76830_d_n4;
        locals.var_xp_dn5 = assign50930_e76830_d_n5;
        locals.var_xp_dn6 = assign50930_e76830_d_n6;
        locals.var_xp_dn7 = assign50930_e76830_d_n7;
        locals.var_xp_dn8 = assign50930_e76830_d_n8;
        locals.var_xp_dn9 = assign50930_e76830_d_n9;
        locals.var_xp_dn10 = assign50930_e76830_d_n10;
        locals.var_xp_dn13 = assign50930_e76830_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign50940_e76848, assign50940_e76848_d_n0, assign50940_e76848_d_n2, assign50940_e76848_d_n4, assign50940_e76848_d_n5, assign50940_e76848_d_n6, assign50940_e76848_d_n7, assign50940_e76848_d_n8, assign50940_e76848_d_n9, assign50940_e76848_d_n10, assign50940_e76848_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1304 != 0.0)) {
        let assign50940_e76846: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign50940_e76846, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign50940_e76848;
        locals.var_xmp_dn0 = assign50940_e76848_d_n0;
        locals.var_xmp_dn2 = assign50940_e76848_d_n2;
        locals.var_xmp_dn4 = assign50940_e76848_d_n4;
        locals.var_xmp_dn5 = assign50940_e76848_d_n5;
        locals.var_xmp_dn6 = assign50940_e76848_d_n6;
        locals.var_xmp_dn7 = assign50940_e76848_d_n7;
        locals.var_xmp_dn8 = assign50940_e76848_d_n8;
        locals.var_xmp_dn9 = assign50940_e76848_d_n9;
        locals.var_xmp_dn10 = assign50940_e76848_d_n10;
        locals.var_xmp_dn13 = assign50940_e76848_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign50950_e76866, assign50950_e76866_d_n0, assign50950_e76866_d_n2, assign50950_e76866_d_n4, assign50950_e76866_d_n5, assign50950_e76866_d_n6, assign50950_e76866_d_n7, assign50950_e76866_d_n8, assign50950_e76866_d_n9, assign50950_e76866_d_n10, assign50950_e76866_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1304 != 0.0)) {
        let assign50950_e76864: f64 = (locals.var_xp + locals.var_xmp);
        (assign50950_e76864, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign50950_e76866;
        locals.var_arg_dn0 = assign50950_e76866_d_n0;
        locals.var_arg_dn2 = assign50950_e76866_d_n2;
        locals.var_arg_dn4 = assign50950_e76866_d_n4;
        locals.var_arg_dn5 = assign50950_e76866_d_n5;
        locals.var_arg_dn6 = assign50950_e76866_d_n6;
        locals.var_arg_dn7 = assign50950_e76866_d_n7;
        locals.var_arg_dn8 = assign50950_e76866_d_n8;
        locals.var_arg_dn9 = assign50950_e76866_d_n9;
        locals.var_arg_dn10 = assign50950_e76866_d_n10;
        locals.var_arg_dn13 = assign50950_e76866_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign50960_e76882, assign50960_e76882_d_n0, assign50960_e76882_d_n2, assign50960_e76882_d_n4, assign50960_e76882_d_n5, assign50960_e76882_d_n6, assign50960_e76882_d_n7, assign50960_e76882_d_n8, assign50960_e76882_d_n9, assign50960_e76882_d_n10, assign50960_e76882_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1304 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign50960_e76882;
        locals.var_dnm_dn0 = assign50960_e76882_d_n0;
        locals.var_dnm_dn2 = assign50960_e76882_d_n2;
        locals.var_dnm_dn4 = assign50960_e76882_d_n4;
        locals.var_dnm_dn5 = assign50960_e76882_d_n5;
        locals.var_dnm_dn6 = assign50960_e76882_d_n6;
        locals.var_dnm_dn7 = assign50960_e76882_d_n7;
        locals.var_dnm_dn8 = assign50960_e76882_d_n8;
        locals.var_dnm_dn9 = assign50960_e76882_d_n9;
        locals.var_dnm_dn10 = assign50960_e76882_d_n10;
        locals.var_dnm_dn13 = assign50960_e76882_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign50970_e76897: f64 = if ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1305 = assign50970_e76897;
        locals.var_guard1305_rv = 0.0;

        let assign50980_e76900: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1306 = assign50980_e76900;
        locals.var_guard1306_rv = 0.0;

        let (assign50990_e76920,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1304 != 0.0)) && (locals.var_guard1305 != 0.0)) && (locals.var_guard1306 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign50990_e76920;
        locals.var_mm_rv = 0.0;

        let assign51000_e76923: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1307 = assign51000_e76923;
        locals.var_guard1307_rv = 0.0;

        let (assign51010_e76946,) = {
    if (((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1304 != 0.0)) && (locals.var_guard1305 != 0.0)) && (locals.var_guard1306 == 0.0)) && (locals.var_guard1307 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign51010_e76946;
        locals.var_mm_rv = 0.0;

        let assign51020_e76949: f64 = if 1.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1308 = assign51020_e76949;
        locals.var_guard1308_rv = 0.0;

        let (assign51030_e76975,) = {
    if ((((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1304 != 0.0)) && (locals.var_guard1305 != 0.0)) && (locals.var_guard1306 == 0.0)) && (locals.var_guard1307 == 0.0)) && (locals.var_guard1308 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign51030_e76975;
        locals.var_mm_rv = 0.0;

        let assign51040_e76978: f64 = if 1.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1309 = assign51040_e76978;
        locals.var_guard1309_rv = 0.0;

        let (assign51050_e77007,) = {
    if (((((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1304 != 0.0)) && (locals.var_guard1305 != 0.0)) && (locals.var_guard1306 == 0.0)) && (locals.var_guard1307 == 0.0)) && (locals.var_guard1308 == 0.0)) && (locals.var_guard1309 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign51050_e77007;
        locals.var_mm_rv = 0.0;

        let (assign51060_e77025,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1304 != 0.0)) && (locals.var_guard1305 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign51060_e77025;
        locals.var_m0_rv = 0.0;

        let mut assign51070_loop_guard: usize = 0;
        while {
            let assign51070_cond_e77044: f64 = if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1304 != 0.0)) && (locals.var_guard1305 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign51070_cond_e77044 != 0.0
        } {
            assign51070_loop_guard += 1;
            assert!(assign51070_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign51070_body0_e77063, assign51070_body0_e77063_d_n0, assign51070_body0_e77063_d_n2, assign51070_body0_e77063_d_n4, assign51070_body0_e77063_d_n5, assign51070_body0_e77063_d_n6, assign51070_body0_e77063_d_n7, assign51070_body0_e77063_d_n8, assign51070_body0_e77063_d_n9, assign51070_body0_e77063_d_n10, assign51070_body0_e77063_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1304 != 0.0)) && (locals.var_guard1305 != 0.0)) {
        let assign51070_body0_e77061: f64 = (locals.var_dnm).sqrt();
        (assign51070_body0_e77061, (locals.var_dnm_dn0 / (2.0 * assign51070_body0_e77061)), (locals.var_dnm_dn2 / (2.0 * assign51070_body0_e77061)), (locals.var_dnm_dn4 / (2.0 * assign51070_body0_e77061)), (locals.var_dnm_dn5 / (2.0 * assign51070_body0_e77061)), (locals.var_dnm_dn6 / (2.0 * assign51070_body0_e77061)), (locals.var_dnm_dn7 / (2.0 * assign51070_body0_e77061)), (locals.var_dnm_dn8 / (2.0 * assign51070_body0_e77061)), (locals.var_dnm_dn9 / (2.0 * assign51070_body0_e77061)), (locals.var_dnm_dn10 / (2.0 * assign51070_body0_e77061)), (locals.var_dnm_dn13 / (2.0 * assign51070_body0_e77061)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign51070_body0_e77063;
            locals.var_dnm_dn0 = assign51070_body0_e77063_d_n0;
            locals.var_dnm_dn2 = assign51070_body0_e77063_d_n2;
            locals.var_dnm_dn4 = assign51070_body0_e77063_d_n4;
            locals.var_dnm_dn5 = assign51070_body0_e77063_d_n5;
            locals.var_dnm_dn6 = assign51070_body0_e77063_d_n6;
            locals.var_dnm_dn7 = assign51070_body0_e77063_d_n7;
            locals.var_dnm_dn8 = assign51070_body0_e77063_d_n8;
            locals.var_dnm_dn9 = assign51070_body0_e77063_d_n9;
            locals.var_dnm_dn10 = assign51070_body0_e77063_d_n10;
            locals.var_dnm_dn13 = assign51070_body0_e77063_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign51070_body1_e77083,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1304 != 0.0)) && (locals.var_guard1305 != 0.0)) {
        let assign51070_body1_e77081: f64 = (locals.var_m0 + 1.0);
        (assign51070_body1_e77081,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign51070_body1_e77083;
            locals.var_m0_rv = 0.0;
        }

        let (assign51080_e77113, assign51080_e77113_d_n0, assign51080_e77113_d_n2, assign51080_e77113_d_n4, assign51080_e77113_d_n5, assign51080_e77113_d_n6, assign51080_e77113_d_n7, assign51080_e77113_d_n8, assign51080_e77113_d_n9, assign51080_e77113_d_n10, assign51080_e77113_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1304 != 0.0)) && (locals.var_guard1305 == 0.0)) {
        let (assign51080_e77111, assign51080_e77111_d_n0, assign51080_e77111_d_n2, assign51080_e77111_d_n4, assign51080_e77111_d_n5, assign51080_e77111_d_n6, assign51080_e77111_d_n7, assign51080_e77111_d_n8, assign51080_e77111_d_n9, assign51080_e77111_d_n10, assign51080_e77111_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign51080_e77108: f64 = 2.0;
                let assign51080_e77109: f64 = (1.0 / assign51080_e77108);
                let assign51080_e77110: f64 = (locals.var_dnm).powf(assign51080_e77109);
                (assign51080_e77110, if 0.0 == 0.0 && ((assign51080_e77109) as f64).is_finite() && ((assign51080_e77109) as f64).fract() == 0.0 { if assign51080_e77109 == 0.0 { 0.0 } else { (assign51080_e77109 * ((locals.var_dnm).powf(assign51080_e77109 - 1.0) * locals.var_dnm_dn0)) } } else { (assign51080_e77110 * (assign51080_e77109 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign51080_e77109) as f64).is_finite() && ((assign51080_e77109) as f64).fract() == 0.0 { if assign51080_e77109 == 0.0 { 0.0 } else { (assign51080_e77109 * ((locals.var_dnm).powf(assign51080_e77109 - 1.0) * locals.var_dnm_dn2)) } } else { (assign51080_e77110 * (assign51080_e77109 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign51080_e77109) as f64).is_finite() && ((assign51080_e77109) as f64).fract() == 0.0 { if assign51080_e77109 == 0.0 { 0.0 } else { (assign51080_e77109 * ((locals.var_dnm).powf(assign51080_e77109 - 1.0) * locals.var_dnm_dn4)) } } else { (assign51080_e77110 * (assign51080_e77109 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign51080_e77109) as f64).is_finite() && ((assign51080_e77109) as f64).fract() == 0.0 { if assign51080_e77109 == 0.0 { 0.0 } else { (assign51080_e77109 * ((locals.var_dnm).powf(assign51080_e77109 - 1.0) * locals.var_dnm_dn5)) } } else { (assign51080_e77110 * (assign51080_e77109 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign51080_e77109) as f64).is_finite() && ((assign51080_e77109) as f64).fract() == 0.0 { if assign51080_e77109 == 0.0 { 0.0 } else { (assign51080_e77109 * ((locals.var_dnm).powf(assign51080_e77109 - 1.0) * locals.var_dnm_dn6)) } } else { (assign51080_e77110 * (assign51080_e77109 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign51080_e77109) as f64).is_finite() && ((assign51080_e77109) as f64).fract() == 0.0 { if assign51080_e77109 == 0.0 { 0.0 } else { (assign51080_e77109 * ((locals.var_dnm).powf(assign51080_e77109 - 1.0) * locals.var_dnm_dn7)) } } else { (assign51080_e77110 * (assign51080_e77109 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign51080_e77109) as f64).is_finite() && ((assign51080_e77109) as f64).fract() == 0.0 { if assign51080_e77109 == 0.0 { 0.0 } else { (assign51080_e77109 * ((locals.var_dnm).powf(assign51080_e77109 - 1.0) * locals.var_dnm_dn8)) } } else { (assign51080_e77110 * (assign51080_e77109 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign51080_e77109) as f64).is_finite() && ((assign51080_e77109) as f64).fract() == 0.0 { if assign51080_e77109 == 0.0 { 0.0 } else { (assign51080_e77109 * ((locals.var_dnm).powf(assign51080_e77109 - 1.0) * locals.var_dnm_dn9)) } } else { (assign51080_e77110 * (assign51080_e77109 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign51080_e77109) as f64).is_finite() && ((assign51080_e77109) as f64).fract() == 0.0 { if assign51080_e77109 == 0.0 { 0.0 } else { (assign51080_e77109 * ((locals.var_dnm).powf(assign51080_e77109 - 1.0) * locals.var_dnm_dn10)) } } else { (assign51080_e77110 * (assign51080_e77109 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign51080_e77109) as f64).is_finite() && ((assign51080_e77109) as f64).fract() == 0.0 { if assign51080_e77109 == 0.0 { 0.0 } else { (assign51080_e77109 * ((locals.var_dnm).powf(assign51080_e77109 - 1.0) * locals.var_dnm_dn13)) } } else { (assign51080_e77110 * (assign51080_e77109 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign51080_e77111, assign51080_e77111_d_n0, assign51080_e77111_d_n2, assign51080_e77111_d_n4, assign51080_e77111_d_n5, assign51080_e77111_d_n6, assign51080_e77111_d_n7, assign51080_e77111_d_n8, assign51080_e77111_d_n9, assign51080_e77111_d_n10, assign51080_e77111_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign51080_e77113;
        locals.var_dnm_dn0 = assign51080_e77113_d_n0;
        locals.var_dnm_dn2 = assign51080_e77113_d_n2;
        locals.var_dnm_dn4 = assign51080_e77113_d_n4;
        locals.var_dnm_dn5 = assign51080_e77113_d_n5;
        locals.var_dnm_dn6 = assign51080_e77113_d_n6;
        locals.var_dnm_dn7 = assign51080_e77113_d_n7;
        locals.var_dnm_dn8 = assign51080_e77113_d_n8;
        locals.var_dnm_dn9 = assign51080_e77113_d_n9;
        locals.var_dnm_dn10 = assign51080_e77113_d_n10;
        locals.var_dnm_dn13 = assign51080_e77113_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign51090_e77131, assign51090_e77131_d_n0, assign51090_e77131_d_n2, assign51090_e77131_d_n4, assign51090_e77131_d_n5, assign51090_e77131_d_n6, assign51090_e77131_d_n7, assign51090_e77131_d_n8, assign51090_e77131_d_n9, assign51090_e77131_d_n10, assign51090_e77131_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1304 != 0.0)) {
        let assign51090_e77129: f64 = (1.0 / locals.var_dnm);
        (assign51090_e77129, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign51090_e77131;
        locals.var_dnm_dn0 = assign51090_e77131_d_n0;
        locals.var_dnm_dn2 = assign51090_e77131_d_n2;
        locals.var_dnm_dn4 = assign51090_e77131_d_n4;
        locals.var_dnm_dn5 = assign51090_e77131_d_n5;
        locals.var_dnm_dn6 = assign51090_e77131_d_n6;
        locals.var_dnm_dn7 = assign51090_e77131_d_n7;
        locals.var_dnm_dn8 = assign51090_e77131_d_n8;
        locals.var_dnm_dn9 = assign51090_e77131_d_n9;
        locals.var_dnm_dn10 = assign51090_e77131_d_n10;
        locals.var_dnm_dn13 = assign51090_e77131_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign51100_e77154, assign51100_e77154_d_n0, assign51100_e77154_d_n2, assign51100_e77154_d_n4, assign51100_e77154_d_n5, assign51100_e77154_d_n6, assign51100_e77154_d_n7, assign51100_e77154_d_n8, assign51100_e77154_d_n9, assign51100_e77154_d_n10, assign51100_e77154_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1304 != 0.0)) {
        let assign51100_e77147: f64 = (-locals.var_vbsc__blk1117);
        let assign51100_e77149: f64 = (assign51100_e77147 + 0.8);
        let assign51100_e77150: f64 = (locals.var_tmf1 * assign51100_e77149);
        let assign51100_e77152: f64 = (assign51100_e77150 * locals.var_dnm);
        (assign51100_e77152, ((((locals.var_tmf1_dn0 * assign51100_e77149) + (locals.var_tmf1 * (-locals.var_vbsc__blk1117_dn0))) * locals.var_dnm) + (assign51100_e77150 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign51100_e77149) + (locals.var_tmf1 * (-locals.var_vbsc__blk1117_dn2))) * locals.var_dnm) + (assign51100_e77150 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * assign51100_e77149) + (locals.var_tmf1 * (-locals.var_vbsc__blk1117_dn4))) * locals.var_dnm) + (assign51100_e77150 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * assign51100_e77149) + (locals.var_tmf1 * (-locals.var_vbsc__blk1117_dn5))) * locals.var_dnm) + (assign51100_e77150 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * assign51100_e77149) + (locals.var_tmf1 * (-locals.var_vbsc__blk1117_dn6))) * locals.var_dnm) + (assign51100_e77150 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign51100_e77149) + (locals.var_tmf1 * (-locals.var_vbsc__blk1117_dn7))) * locals.var_dnm) + (assign51100_e77150 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * assign51100_e77149) + (locals.var_tmf1 * (-locals.var_vbsc__blk1117_dn8))) * locals.var_dnm) + (assign51100_e77150 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * assign51100_e77149) + (locals.var_tmf1 * (-locals.var_vbsc__blk1117_dn9))) * locals.var_dnm) + (assign51100_e77150 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * assign51100_e77149) + (locals.var_tmf1 * (-locals.var_vbsc__blk1117_dn10))) * locals.var_dnm) + (assign51100_e77150 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn13 * assign51100_e77149) + (locals.var_tmf1 * (-locals.var_vbsc__blk1117_dn13))) * locals.var_dnm) + (assign51100_e77150 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign51100_e77154;
        locals.var_tmf0_dn0 = assign51100_e77154_d_n0;
        locals.var_tmf0_dn2 = assign51100_e77154_d_n2;
        locals.var_tmf0_dn4 = assign51100_e77154_d_n4;
        locals.var_tmf0_dn5 = assign51100_e77154_d_n5;
        locals.var_tmf0_dn6 = assign51100_e77154_d_n6;
        locals.var_tmf0_dn7 = assign51100_e77154_d_n7;
        locals.var_tmf0_dn8 = assign51100_e77154_d_n8;
        locals.var_tmf0_dn9 = assign51100_e77154_d_n9;
        locals.var_tmf0_dn10 = assign51100_e77154_d_n10;
        locals.var_tmf0_dn13 = assign51100_e77154_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign51110_e77179, assign51110_e77179_d_n0, assign51110_e77179_d_n2, assign51110_e77179_d_n4, assign51110_e77179_d_n5, assign51110_e77179_d_n6, assign51110_e77179_d_n7, assign51110_e77179_d_n8, assign51110_e77179_d_n9, assign51110_e77179_d_n10, assign51110_e77179_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1304 != 0.0)) {
        let assign51110_e77169: f64 = (-locals.var_vbsc__blk1117);
        let assign51110_e77171: f64 = (assign51110_e77169 + 0.8);
        let assign51110_e77173: f64 = (assign51110_e77171 * locals.var_xmp);
        let assign51110_e77175: f64 = (assign51110_e77173 * locals.var_dnm);
        let assign51110_e77177: f64 = (assign51110_e77175 / locals.var_arg);
        (assign51110_e77177, ((((((((-locals.var_vbsc__blk1117_dn0) * locals.var_xmp) + (assign51110_e77171 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign51110_e77173 * locals.var_dnm_dn0)) * locals.var_arg) - (assign51110_e77175 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((-locals.var_vbsc__blk1117_dn2) * locals.var_xmp) + (assign51110_e77171 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign51110_e77173 * locals.var_dnm_dn2)) * locals.var_arg) - (assign51110_e77175 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((-locals.var_vbsc__blk1117_dn4) * locals.var_xmp) + (assign51110_e77171 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign51110_e77173 * locals.var_dnm_dn4)) * locals.var_arg) - (assign51110_e77175 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((((-locals.var_vbsc__blk1117_dn5) * locals.var_xmp) + (assign51110_e77171 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign51110_e77173 * locals.var_dnm_dn5)) * locals.var_arg) - (assign51110_e77175 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((((-locals.var_vbsc__blk1117_dn6) * locals.var_xmp) + (assign51110_e77171 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign51110_e77173 * locals.var_dnm_dn6)) * locals.var_arg) - (assign51110_e77175 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((-locals.var_vbsc__blk1117_dn7) * locals.var_xmp) + (assign51110_e77171 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign51110_e77173 * locals.var_dnm_dn7)) * locals.var_arg) - (assign51110_e77175 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((-locals.var_vbsc__blk1117_dn8) * locals.var_xmp) + (assign51110_e77171 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign51110_e77173 * locals.var_dnm_dn8)) * locals.var_arg) - (assign51110_e77175 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((((-locals.var_vbsc__blk1117_dn9) * locals.var_xmp) + (assign51110_e77171 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign51110_e77173 * locals.var_dnm_dn9)) * locals.var_arg) - (assign51110_e77175 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((((-locals.var_vbsc__blk1117_dn10) * locals.var_xmp) + (assign51110_e77171 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign51110_e77173 * locals.var_dnm_dn10)) * locals.var_arg) - (assign51110_e77175 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((-locals.var_vbsc__blk1117_dn13) * locals.var_xmp) + (assign51110_e77171 * locals.var_xmp_dn13)) * locals.var_dnm) + (assign51110_e77173 * locals.var_dnm_dn13)) * locals.var_arg) - (assign51110_e77175 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign51110_e77179;
        locals.var_t0_dn0 = assign51110_e77179_d_n0;
        locals.var_t0_dn2 = assign51110_e77179_d_n2;
        locals.var_t0_dn4 = assign51110_e77179_d_n4;
        locals.var_t0_dn5 = assign51110_e77179_d_n5;
        locals.var_t0_dn6 = assign51110_e77179_d_n6;
        locals.var_t0_dn7 = assign51110_e77179_d_n7;
        locals.var_t0_dn8 = assign51110_e77179_d_n8;
        locals.var_t0_dn9 = assign51110_e77179_d_n9;
        locals.var_t0_dn10 = assign51110_e77179_d_n10;
        locals.var_t0_dn13 = assign51110_e77179_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign51120_e77202, assign51120_e77202_d_n0, assign51120_e77202_d_n2, assign51120_e77202_d_n4, assign51120_e77202_d_n5, assign51120_e77202_d_n6, assign51120_e77202_d_n7, assign51120_e77202_d_n8, assign51120_e77202_d_n9, assign51120_e77202_d_n10, assign51120_e77202_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1304 != 0.0)) {
        let assign51120_e77195: f64 = (-locals.var_vbsc__blk1117);
        let assign51120_e77197: f64 = (assign51120_e77195 + 0.8);
        let assign51120_e77198: f64 = (0.2 + assign51120_e77197);
        let assign51120_e77200: f64 = (assign51120_e77198 - locals.var_tmf0);
        (assign51120_e77200, ((-locals.var_vbsc__blk1117_dn0) - locals.var_tmf0_dn0), ((-locals.var_vbsc__blk1117_dn2) - locals.var_tmf0_dn2), ((-locals.var_vbsc__blk1117_dn4) - locals.var_tmf0_dn4), ((-locals.var_vbsc__blk1117_dn5) - locals.var_tmf0_dn5), ((-locals.var_vbsc__blk1117_dn6) - locals.var_tmf0_dn6), ((-locals.var_vbsc__blk1117_dn7) - locals.var_tmf0_dn7), ((-locals.var_vbsc__blk1117_dn8) - locals.var_tmf0_dn8), ((-locals.var_vbsc__blk1117_dn9) - locals.var_tmf0_dn9), ((-locals.var_vbsc__blk1117_dn10) - locals.var_tmf0_dn10), ((-locals.var_vbsc__blk1117_dn13) - locals.var_tmf0_dn13),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn13,)
    }
};
        locals.var_t8 = assign51120_e77202;
        locals.var_t8_dn0 = assign51120_e77202_d_n0;
        locals.var_t8_dn2 = assign51120_e77202_d_n2;
        locals.var_t8_dn4 = assign51120_e77202_d_n4;
        locals.var_t8_dn5 = assign51120_e77202_d_n5;
        locals.var_t8_dn6 = assign51120_e77202_d_n6;
        locals.var_t8_dn7 = assign51120_e77202_d_n7;
        locals.var_t8_dn8 = assign51120_e77202_d_n8;
        locals.var_t8_dn9 = assign51120_e77202_d_n9;
        locals.var_t8_dn10 = assign51120_e77202_d_n10;
        locals.var_t8_dn13 = assign51120_e77202_d_n13;
        locals.var_t8_rv = 0.0;

        let (assign51130_e77218, assign51130_e77218_d_n0, assign51130_e77218_d_n2, assign51130_e77218_d_n4, assign51130_e77218_d_n5, assign51130_e77218_d_n6, assign51130_e77218_d_n7, assign51130_e77218_d_n8, assign51130_e77218_d_n9, assign51130_e77218_d_n10, assign51130_e77218_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1304 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign51130_e77218;
        locals.var_t0_dn0 = assign51130_e77218_d_n0;
        locals.var_t0_dn2 = assign51130_e77218_d_n2;
        locals.var_t0_dn4 = assign51130_e77218_d_n4;
        locals.var_t0_dn5 = assign51130_e77218_d_n5;
        locals.var_t0_dn6 = assign51130_e77218_d_n6;
        locals.var_t0_dn7 = assign51130_e77218_d_n7;
        locals.var_t0_dn8 = assign51130_e77218_d_n8;
        locals.var_t0_dn9 = assign51130_e77218_d_n9;
        locals.var_t0_dn10 = assign51130_e77218_d_n10;
        locals.var_t0_dn13 = assign51130_e77218_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign51140_e77235, assign51140_e77235_d_n0, assign51140_e77235_d_n2, assign51140_e77235_d_n4, assign51140_e77235_d_n5, assign51140_e77235_d_n6, assign51140_e77235_d_n7, assign51140_e77235_d_n8, assign51140_e77235_d_n9, assign51140_e77235_d_n10, assign51140_e77235_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1304 == 0.0)) {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn13,)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn13,)
    }
};
        locals.var_t8 = assign51140_e77235;
        locals.var_t8_dn0 = assign51140_e77235_d_n0;
        locals.var_t8_dn2 = assign51140_e77235_d_n2;
        locals.var_t8_dn4 = assign51140_e77235_d_n4;
        locals.var_t8_dn5 = assign51140_e77235_d_n5;
        locals.var_t8_dn6 = assign51140_e77235_d_n6;
        locals.var_t8_dn7 = assign51140_e77235_d_n7;
        locals.var_t8_dn8 = assign51140_e77235_d_n8;
        locals.var_t8_dn9 = assign51140_e77235_d_n9;
        locals.var_t8_dn10 = assign51140_e77235_d_n10;
        locals.var_t8_dn13 = assign51140_e77235_d_n13;
        locals.var_t8_rv = 0.0;

        let (assign51150_e77252, assign51150_e77252_d_n0, assign51150_e77252_d_n2, assign51150_e77252_d_n4, assign51150_e77252_d_n5, assign51150_e77252_d_n6, assign51150_e77252_d_n7, assign51150_e77252_d_n8, assign51150_e77252_d_n9, assign51150_e77252_d_n10, assign51150_e77252_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1304 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign51150_e77252;
        locals.var_t0_dn0 = assign51150_e77252_d_n0;
        locals.var_t0_dn2 = assign51150_e77252_d_n2;
        locals.var_t0_dn4 = assign51150_e77252_d_n4;
        locals.var_t0_dn5 = assign51150_e77252_d_n5;
        locals.var_t0_dn6 = assign51150_e77252_d_n6;
        locals.var_t0_dn7 = assign51150_e77252_d_n7;
        locals.var_t0_dn8 = assign51150_e77252_d_n8;
        locals.var_t0_dn9 = assign51150_e77252_d_n9;
        locals.var_t0_dn10 = assign51150_e77252_d_n10;
        locals.var_t0_dn13 = assign51150_e77252_d_n13;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_177(
        locals: &mut StampLocals,
    ) {
        let (assign51160_e77268, assign51160_e77268_d_n0, assign51160_e77268_d_n2, assign51160_e77268_d_n4, assign51160_e77268_d_n5, assign51160_e77268_d_n6, assign51160_e77268_d_n7, assign51160_e77268_d_n8, assign51160_e77268_d_n9, assign51160_e77268_d_n10, assign51160_e77268_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) {
        let assign51160_e77266: f64 = (locals.var_c_2esipq_ndepm__blk1136 * locals.var_t8);
        (assign51160_e77266, ((locals.var_c_2esipq_ndepm__blk1136_dn0 * locals.var_t8) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_t8_dn0)), ((locals.var_c_2esipq_ndepm__blk1136_dn2 * locals.var_t8) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_t8_dn2)), ((locals.var_c_2esipq_ndepm__blk1136_dn4 * locals.var_t8) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_t8_dn4)), ((locals.var_c_2esipq_ndepm__blk1136_dn5 * locals.var_t8) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_t8_dn5)), ((locals.var_c_2esipq_ndepm__blk1136_dn6 * locals.var_t8) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_t8_dn6)), ((locals.var_c_2esipq_ndepm__blk1136_dn7 * locals.var_t8) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_t8_dn7)), ((locals.var_c_2esipq_ndepm__blk1136_dn8 * locals.var_t8) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_t8_dn8)), ((locals.var_c_2esipq_ndepm__blk1136_dn9 * locals.var_t8) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_t8_dn9)), ((locals.var_c_2esipq_ndepm__blk1136_dn10 * locals.var_t8) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_t8_dn10)), ((locals.var_c_2esipq_ndepm__blk1136_dn13 * locals.var_t8) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_t8_dn13)),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign51160_e77268;
        locals.var_t9_dn0 = assign51160_e77268_d_n0;
        locals.var_t9_dn2 = assign51160_e77268_d_n2;
        locals.var_t9_dn4 = assign51160_e77268_d_n4;
        locals.var_t9_dn5 = assign51160_e77268_d_n5;
        locals.var_t9_dn6 = assign51160_e77268_d_n6;
        locals.var_t9_dn7 = assign51160_e77268_d_n7;
        locals.var_t9_dn8 = assign51160_e77268_d_n8;
        locals.var_t9_dn9 = assign51160_e77268_d_n9;
        locals.var_t9_dn10 = assign51160_e77268_d_n10;
        locals.var_t9_dn13 = assign51160_e77268_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign51170_e77283, assign51170_e77283_d_n0, assign51170_e77283_d_n2, assign51170_e77283_d_n4, assign51170_e77283_d_n5, assign51170_e77283_d_n6, assign51170_e77283_d_n7, assign51170_e77283_d_n8, assign51170_e77283_d_n9, assign51170_e77283_d_n10, assign51170_e77283_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) {
        let assign51170_e77281: f64 = (locals.var_t9).sqrt();
        (assign51170_e77281, (locals.var_t9_dn0 / (2.0 * assign51170_e77281)), (locals.var_t9_dn2 / (2.0 * assign51170_e77281)), (locals.var_t9_dn4 / (2.0 * assign51170_e77281)), (locals.var_t9_dn5 / (2.0 * assign51170_e77281)), (locals.var_t9_dn6 / (2.0 * assign51170_e77281)), (locals.var_t9_dn7 / (2.0 * assign51170_e77281)), (locals.var_t9_dn8 / (2.0 * assign51170_e77281)), (locals.var_t9_dn9 / (2.0 * assign51170_e77281)), (locals.var_t9_dn10 / (2.0 * assign51170_e77281)), (locals.var_t9_dn13 / (2.0 * assign51170_e77281)),)
    } else {
        (locals.var_wd, locals.var_wd_dn0, locals.var_wd_dn2, locals.var_wd_dn4, locals.var_wd_dn5, locals.var_wd_dn6, locals.var_wd_dn7, locals.var_wd_dn8, locals.var_wd_dn9, locals.var_wd_dn10, locals.var_wd_dn13,)
    }
};
        locals.var_wd = assign51170_e77283;
        locals.var_wd_dn0 = assign51170_e77283_d_n0;
        locals.var_wd_dn2 = assign51170_e77283_d_n2;
        locals.var_wd_dn4 = assign51170_e77283_d_n4;
        locals.var_wd_dn5 = assign51170_e77283_d_n5;
        locals.var_wd_dn6 = assign51170_e77283_d_n6;
        locals.var_wd_dn7 = assign51170_e77283_d_n7;
        locals.var_wd_dn8 = assign51170_e77283_d_n8;
        locals.var_wd_dn9 = assign51170_e77283_d_n9;
        locals.var_wd_dn10 = assign51170_e77283_d_n10;
        locals.var_wd_dn13 = assign51170_e77283_d_n13;
        locals.var_wd_rv = 0.0;

        let (assign51180_e77299, assign51180_e77299_d_n0, assign51180_e77299_d_n2, assign51180_e77299_d_n4, assign51180_e77299_d_n5, assign51180_e77299_d_n6, assign51180_e77299_d_n7, assign51180_e77299_d_n8, assign51180_e77299_d_n9, assign51180_e77299_d_n10, assign51180_e77299_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) {
        let assign51180_e77297: f64 = (1.0 / locals.var_wd);
        (assign51180_e77297, (-(locals.var_wd_dn0 / (locals.var_wd * locals.var_wd))), (-(locals.var_wd_dn2 / (locals.var_wd * locals.var_wd))), (-(locals.var_wd_dn4 / (locals.var_wd * locals.var_wd))), (-(locals.var_wd_dn5 / (locals.var_wd * locals.var_wd))), (-(locals.var_wd_dn6 / (locals.var_wd * locals.var_wd))), (-(locals.var_wd_dn7 / (locals.var_wd * locals.var_wd))), (-(locals.var_wd_dn8 / (locals.var_wd * locals.var_wd))), (-(locals.var_wd_dn9 / (locals.var_wd * locals.var_wd))), (-(locals.var_wd_dn10 / (locals.var_wd * locals.var_wd))), (-(locals.var_wd_dn13 / (locals.var_wd * locals.var_wd))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign51180_e77299;
        locals.var_t0_dn0 = assign51180_e77299_d_n0;
        locals.var_t0_dn2 = assign51180_e77299_d_n2;
        locals.var_t0_dn4 = assign51180_e77299_d_n4;
        locals.var_t0_dn5 = assign51180_e77299_d_n5;
        locals.var_t0_dn6 = assign51180_e77299_d_n6;
        locals.var_t0_dn7 = assign51180_e77299_d_n7;
        locals.var_t0_dn8 = assign51180_e77299_d_n8;
        locals.var_t0_dn9 = assign51180_e77299_d_n9;
        locals.var_t0_dn10 = assign51180_e77299_d_n10;
        locals.var_t0_dn13 = assign51180_e77299_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign51190_e77315, assign51190_e77315_d_n0, assign51190_e77315_d_n2, assign51190_e77315_d_n4, assign51190_e77315_d_n5, assign51190_e77315_d_n6, assign51190_e77315_d_n7, assign51190_e77315_d_n8, assign51190_e77315_d_n9, assign51190_e77315_d_n10, assign51190_e77315_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) {
        let assign51190_e77313: f64 = (locals.var_qn0 * locals.var_t0);
        (assign51190_e77313, ((locals.var_qn0_dn0 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn0)), ((locals.var_qn0_dn2 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn2)), ((locals.var_qn0_dn4 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn4)), ((locals.var_qn0_dn5 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn5)), ((locals.var_qn0_dn6 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn6)), ((locals.var_qn0_dn7 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn7)), ((locals.var_qn0_dn8 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn8)), ((locals.var_qn0_dn9 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn9)), ((locals.var_qn0_dn10 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn10)), ((locals.var_qn0_dn13 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign51190_e77315;
        locals.var_t1_dn0 = assign51190_e77315_d_n0;
        locals.var_t1_dn2 = assign51190_e77315_d_n2;
        locals.var_t1_dn4 = assign51190_e77315_d_n4;
        locals.var_t1_dn5 = assign51190_e77315_d_n5;
        locals.var_t1_dn6 = assign51190_e77315_d_n6;
        locals.var_t1_dn7 = assign51190_e77315_d_n7;
        locals.var_t1_dn8 = assign51190_e77315_d_n8;
        locals.var_t1_dn9 = assign51190_e77315_d_n9;
        locals.var_t1_dn10 = assign51190_e77315_d_n10;
        locals.var_t1_dn13 = assign51190_e77315_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign51200_e77331, assign51200_e77331_d_n0, assign51200_e77331_d_n2, assign51200_e77331_d_n4, assign51200_e77331_d_n5, assign51200_e77331_d_n6, assign51200_e77331_d_n7, assign51200_e77331_d_n8, assign51200_e77331_d_n9, assign51200_e77331_d_n10, assign51200_e77331_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) {
        let assign51200_e77329: f64 = (locals.var_uc_clm3 * locals.var_t1);
        (assign51200_e77329, (locals.var_uc_clm3 * locals.var_t1_dn0), (locals.var_uc_clm3 * locals.var_t1_dn2), (locals.var_uc_clm3 * locals.var_t1_dn4), (locals.var_uc_clm3 * locals.var_t1_dn5), (locals.var_uc_clm3 * locals.var_t1_dn6), (locals.var_uc_clm3 * locals.var_t1_dn7), (locals.var_uc_clm3 * locals.var_t1_dn8), (locals.var_uc_clm3 * locals.var_t1_dn9), (locals.var_uc_clm3 * locals.var_t1_dn10), (locals.var_uc_clm3 * locals.var_t1_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign51200_e77331;
        locals.var_t2_dn0 = assign51200_e77331_d_n0;
        locals.var_t2_dn2 = assign51200_e77331_d_n2;
        locals.var_t2_dn4 = assign51200_e77331_d_n4;
        locals.var_t2_dn5 = assign51200_e77331_d_n5;
        locals.var_t2_dn6 = assign51200_e77331_d_n6;
        locals.var_t2_dn7 = assign51200_e77331_d_n7;
        locals.var_t2_dn8 = assign51200_e77331_d_n8;
        locals.var_t2_dn9 = assign51200_e77331_d_n9;
        locals.var_t2_dn10 = assign51200_e77331_d_n10;
        locals.var_t2_dn13 = assign51200_e77331_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign51210_e77347, assign51210_e77347_d_n0, assign51210_e77347_d_n2, assign51210_e77347_d_n4, assign51210_e77347_d_n5, assign51210_e77347_d_n6, assign51210_e77347_d_n7, assign51210_e77347_d_n8, assign51210_e77347_d_n9, assign51210_e77347_d_n10, assign51210_e77347_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) {
        let assign51210_e77345: f64 = (locals.var_uc_clm3 * locals.var_t0);
        (assign51210_e77345, (locals.var_uc_clm3 * locals.var_t0_dn0), (locals.var_uc_clm3 * locals.var_t0_dn2), (locals.var_uc_clm3 * locals.var_t0_dn4), (locals.var_uc_clm3 * locals.var_t0_dn5), (locals.var_uc_clm3 * locals.var_t0_dn6), (locals.var_uc_clm3 * locals.var_t0_dn7), (locals.var_uc_clm3 * locals.var_t0_dn8), (locals.var_uc_clm3 * locals.var_t0_dn9), (locals.var_uc_clm3 * locals.var_t0_dn10), (locals.var_uc_clm3 * locals.var_t0_dn13),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign51210_e77347;
        locals.var_t3_dn0 = assign51210_e77347_d_n0;
        locals.var_t3_dn2 = assign51210_e77347_d_n2;
        locals.var_t3_dn4 = assign51210_e77347_d_n4;
        locals.var_t3_dn5 = assign51210_e77347_d_n5;
        locals.var_t3_dn6 = assign51210_e77347_d_n6;
        locals.var_t3_dn7 = assign51210_e77347_d_n7;
        locals.var_t3_dn8 = assign51210_e77347_d_n8;
        locals.var_t3_dn9 = assign51210_e77347_d_n9;
        locals.var_t3_dn10 = assign51210_e77347_d_n10;
        locals.var_t3_dn13 = assign51210_e77347_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign51220_e77365, assign51220_e77365_d_n0, assign51220_e77365_d_n2, assign51220_e77365_d_n4, assign51220_e77365_d_n5, assign51220_e77365_d_n6, assign51220_e77365_d_n7, assign51220_e77365_d_n8, assign51220_e77365_d_n9, assign51220_e77365_d_n10, assign51220_e77365_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) {
        let assign51220_e77361: f64 = (locals.var_uc_clm2 * locals.var_q_ndepm__blk1133);
        let assign51220_e77363: f64 = (assign51220_e77361 + locals.var_t2);
        (assign51220_e77363, (((locals.var_uc_clm2_dn0 * locals.var_q_ndepm__blk1133) + (locals.var_uc_clm2 * locals.var_q_ndepm__blk1133_dn0)) + locals.var_t2_dn0), (((locals.var_uc_clm2_dn2 * locals.var_q_ndepm__blk1133) + (locals.var_uc_clm2 * locals.var_q_ndepm__blk1133_dn2)) + locals.var_t2_dn2), (((locals.var_uc_clm2_dn4 * locals.var_q_ndepm__blk1133) + (locals.var_uc_clm2 * locals.var_q_ndepm__blk1133_dn4)) + locals.var_t2_dn4), (((locals.var_uc_clm2_dn5 * locals.var_q_ndepm__blk1133) + (locals.var_uc_clm2 * locals.var_q_ndepm__blk1133_dn5)) + locals.var_t2_dn5), (((locals.var_uc_clm2_dn6 * locals.var_q_ndepm__blk1133) + (locals.var_uc_clm2 * locals.var_q_ndepm__blk1133_dn6)) + locals.var_t2_dn6), (((locals.var_uc_clm2_dn7 * locals.var_q_ndepm__blk1133) + (locals.var_uc_clm2 * locals.var_q_ndepm__blk1133_dn7)) + locals.var_t2_dn7), (((locals.var_uc_clm2_dn8 * locals.var_q_ndepm__blk1133) + (locals.var_uc_clm2 * locals.var_q_ndepm__blk1133_dn8)) + locals.var_t2_dn8), (((locals.var_uc_clm2_dn9 * locals.var_q_ndepm__blk1133) + (locals.var_uc_clm2 * locals.var_q_ndepm__blk1133_dn9)) + locals.var_t2_dn9), (((locals.var_uc_clm2_dn10 * locals.var_q_ndepm__blk1133) + (locals.var_uc_clm2 * locals.var_q_ndepm__blk1133_dn10)) + locals.var_t2_dn10), (((locals.var_uc_clm2_dn13 * locals.var_q_ndepm__blk1133) + (locals.var_uc_clm2 * locals.var_q_ndepm__blk1133_dn13)) + locals.var_t2_dn13),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign51220_e77365;
        locals.var_t5_dn0 = assign51220_e77365_d_n0;
        locals.var_t5_dn2 = assign51220_e77365_d_n2;
        locals.var_t5_dn4 = assign51220_e77365_d_n4;
        locals.var_t5_dn5 = assign51220_e77365_d_n5;
        locals.var_t5_dn6 = assign51220_e77365_d_n6;
        locals.var_t5_dn7 = assign51220_e77365_d_n7;
        locals.var_t5_dn8 = assign51220_e77365_d_n8;
        locals.var_t5_dn9 = assign51220_e77365_d_n9;
        locals.var_t5_dn10 = assign51220_e77365_d_n10;
        locals.var_t5_dn13 = assign51220_e77365_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign51230_e77381, assign51230_e77381_d_n0, assign51230_e77381_d_n2, assign51230_e77381_d_n4, assign51230_e77381_d_n5, assign51230_e77381_d_n6, assign51230_e77381_d_n7, assign51230_e77381_d_n8, assign51230_e77381_d_n9, assign51230_e77381_d_n10, assign51230_e77381_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) {
        let assign51230_e77379: f64 = (1.0 / locals.var_t5);
        (assign51230_e77379, (-(locals.var_t5_dn0 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn2 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn4 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn5 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn6 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn7 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn8 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn9 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn10 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn13 / (locals.var_t5 * locals.var_t5))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign51230_e77381;
        locals.var_t1_dn0 = assign51230_e77381_d_n0;
        locals.var_t1_dn2 = assign51230_e77381_d_n2;
        locals.var_t1_dn4 = assign51230_e77381_d_n4;
        locals.var_t1_dn5 = assign51230_e77381_d_n5;
        locals.var_t1_dn6 = assign51230_e77381_d_n6;
        locals.var_t1_dn7 = assign51230_e77381_d_n7;
        locals.var_t1_dn8 = assign51230_e77381_d_n8;
        locals.var_t1_dn9 = assign51230_e77381_d_n9;
        locals.var_t1_dn10 = assign51230_e77381_d_n10;
        locals.var_t1_dn13 = assign51230_e77381_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign51240_e77397, assign51240_e77397_d_n0, assign51240_e77397_d_n2, assign51240_e77397_d_n4, assign51240_e77397_d_n5, assign51240_e77397_d_n6, assign51240_e77397_d_n7, assign51240_e77397_d_n8, assign51240_e77397_d_n9, assign51240_e77397_d_n10, assign51240_e77397_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) {
        let assign51240_e77395: f64 = (1.034943e-10 * locals.var_t1);
        (assign51240_e77395, (1.034943e-10 * locals.var_t1_dn0), (1.034943e-10 * locals.var_t1_dn2), (1.034943e-10 * locals.var_t1_dn4), (1.034943e-10 * locals.var_t1_dn5), (1.034943e-10 * locals.var_t1_dn6), (1.034943e-10 * locals.var_t1_dn7), (1.034943e-10 * locals.var_t1_dn8), (1.034943e-10 * locals.var_t1_dn9), (1.034943e-10 * locals.var_t1_dn10), (1.034943e-10 * locals.var_t1_dn13),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign51240_e77397;
        locals.var_t4_dn0 = assign51240_e77397_d_n0;
        locals.var_t4_dn2 = assign51240_e77397_d_n2;
        locals.var_t4_dn4 = assign51240_e77397_d_n4;
        locals.var_t4_dn5 = assign51240_e77397_d_n5;
        locals.var_t4_dn6 = assign51240_e77397_d_n6;
        locals.var_t4_dn7 = assign51240_e77397_d_n7;
        locals.var_t4_dn8 = assign51240_e77397_d_n8;
        locals.var_t4_dn9 = assign51240_e77397_d_n9;
        locals.var_t4_dn10 = assign51240_e77397_d_n10;
        locals.var_t4_dn13 = assign51240_e77397_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign51250_e77413, assign51250_e77413_d_n0, assign51250_e77413_d_n2, assign51250_e77413_d_n4, assign51250_e77413_d_n5, assign51250_e77413_d_n6, assign51250_e77413_d_n7, assign51250_e77413_d_n8, assign51250_e77413_d_n9, assign51250_e77413_d_n10, assign51250_e77413_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) {
        let assign51250_e77411: f64 = (1.0 - locals.var_uc_clm1);
        (assign51250_e77411, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign51250_e77413;
        locals.var_t1_dn0 = assign51250_e77413_d_n0;
        locals.var_t1_dn2 = assign51250_e77413_d_n2;
        locals.var_t1_dn4 = assign51250_e77413_d_n4;
        locals.var_t1_dn5 = assign51250_e77413_d_n5;
        locals.var_t1_dn6 = assign51250_e77413_d_n6;
        locals.var_t1_dn7 = assign51250_e77413_d_n7;
        locals.var_t1_dn8 = assign51250_e77413_d_n8;
        locals.var_t1_dn9 = assign51250_e77413_d_n9;
        locals.var_t1_dn10 = assign51250_e77413_d_n10;
        locals.var_t1_dn13 = assign51250_e77413_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign51260_e77435, assign51260_e77435_d_n0, assign51260_e77435_d_n2, assign51260_e77435_d_n4, assign51260_e77435_d_n5, assign51260_e77435_d_n6, assign51260_e77435_d_n7, assign51260_e77435_d_n8, assign51260_e77435_d_n9, assign51260_e77435_d_n10, assign51260_e77435_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) {
        let assign51260_e77428: f64 = (locals.var_vds + locals.var_ps0z);
        let assign51260_e77429: f64 = (locals.var_uc_clm1 * assign51260_e77428);
        let assign51260_e77432: f64 = (locals.var_t1 * locals.var_psl);
        let assign51260_e77433: f64 = (assign51260_e77429 + assign51260_e77432);
        (assign51260_e77433, ((locals.var_uc_clm1 * (locals.var_vds_dn0 + locals.var_ps0z_dn0)) + ((locals.var_t1_dn0 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn0))), ((locals.var_uc_clm1 * (locals.var_vds_dn2 + locals.var_ps0z_dn2)) + ((locals.var_t1_dn2 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn2))), ((locals.var_uc_clm1 * (locals.var_vds_dn4 + locals.var_ps0z_dn4)) + ((locals.var_t1_dn4 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn4))), ((locals.var_uc_clm1 * (locals.var_vds_dn5 + locals.var_ps0z_dn5)) + ((locals.var_t1_dn5 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn5))), ((locals.var_uc_clm1 * (locals.var_vds_dn6 + locals.var_ps0z_dn6)) + ((locals.var_t1_dn6 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn6))), ((locals.var_uc_clm1 * (locals.var_vds_dn7 + locals.var_ps0z_dn7)) + ((locals.var_t1_dn7 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn7))), ((locals.var_uc_clm1 * (locals.var_vds_dn8 + locals.var_ps0z_dn8)) + ((locals.var_t1_dn8 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn8))), ((locals.var_uc_clm1 * (locals.var_vds_dn9 + locals.var_ps0z_dn9)) + ((locals.var_t1_dn9 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn9))), ((locals.var_uc_clm1 * (locals.var_vds_dn10 + locals.var_ps0z_dn10)) + ((locals.var_t1_dn10 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn10))), ((locals.var_uc_clm1 * (locals.var_vds_dn13 + locals.var_ps0z_dn13)) + ((locals.var_t1_dn13 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn13))),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn13,)
    }
};
        locals.var_psdl = assign51260_e77435;
        locals.var_psdl_dn0 = assign51260_e77435_d_n0;
        locals.var_psdl_dn2 = assign51260_e77435_d_n2;
        locals.var_psdl_dn4 = assign51260_e77435_d_n4;
        locals.var_psdl_dn5 = assign51260_e77435_d_n5;
        locals.var_psdl_dn6 = assign51260_e77435_d_n6;
        locals.var_psdl_dn7 = assign51260_e77435_d_n7;
        locals.var_psdl_dn8 = assign51260_e77435_d_n8;
        locals.var_psdl_dn9 = assign51260_e77435_d_n9;
        locals.var_psdl_dn10 = assign51260_e77435_d_n10;
        locals.var_psdl_dn13 = assign51260_e77435_d_n13;
        locals.var_psdl_rv = 0.0;

        let assign51270_e77439: f64 = (locals.var_ps0z + locals.var_vds);
        let assign51270_e77442: f64 = (10.0 * 2.220446049250313e-16);
        let assign51270_e77443: f64 = (assign51270_e77439 - assign51270_e77442);
        let assign51270_e77446: f64 = (10.0 * 2.220446049250313e-16);
        let assign51270_e77447: f64 = (assign51270_e77443 - assign51270_e77446);
        let assign51270_e77451: f64 = (10.0 * 2.220446049250313e-16);
        let assign51270_e77454: f64 = if ((locals.var_psdl > assign51270_e77447) && (assign51270_e77451 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1310 = assign51270_e77454;
        locals.var_guard1310_rv = 0.0;

        let (assign51280_e77482, assign51280_e77482_d_n0, assign51280_e77482_d_n2, assign51280_e77482_d_n4, assign51280_e77482_d_n5, assign51280_e77482_d_n6, assign51280_e77482_d_n7, assign51280_e77482_d_n8, assign51280_e77482_d_n9, assign51280_e77482_d_n10, assign51280_e77482_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1310 != 0.0)) {
        let assign51280_e77471: f64 = (locals.var_ps0z + locals.var_vds);
        let assign51280_e77474: f64 = (10.0 * 2.220446049250313e-16);
        let assign51280_e77475: f64 = (assign51280_e77471 - assign51280_e77474);
        let assign51280_e77476: f64 = (locals.var_psdl - assign51280_e77475);
        let assign51280_e77479: f64 = (10.0 * 2.220446049250313e-16);
        let assign51280_e77480: f64 = (assign51280_e77476 + assign51280_e77479);
        (assign51280_e77480, (locals.var_psdl_dn0 - (locals.var_ps0z_dn0 + locals.var_vds_dn0)), (locals.var_psdl_dn2 - (locals.var_ps0z_dn2 + locals.var_vds_dn2)), (locals.var_psdl_dn4 - (locals.var_ps0z_dn4 + locals.var_vds_dn4)), (locals.var_psdl_dn5 - (locals.var_ps0z_dn5 + locals.var_vds_dn5)), (locals.var_psdl_dn6 - (locals.var_ps0z_dn6 + locals.var_vds_dn6)), (locals.var_psdl_dn7 - (locals.var_ps0z_dn7 + locals.var_vds_dn7)), (locals.var_psdl_dn8 - (locals.var_ps0z_dn8 + locals.var_vds_dn8)), (locals.var_psdl_dn9 - (locals.var_ps0z_dn9 + locals.var_vds_dn9)), (locals.var_psdl_dn10 - (locals.var_ps0z_dn10 + locals.var_vds_dn10)), (locals.var_psdl_dn13 - (locals.var_ps0z_dn13 + locals.var_vds_dn13)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign51280_e77482;
        locals.var_tmf1_dn0 = assign51280_e77482_d_n0;
        locals.var_tmf1_dn2 = assign51280_e77482_d_n2;
        locals.var_tmf1_dn4 = assign51280_e77482_d_n4;
        locals.var_tmf1_dn5 = assign51280_e77482_d_n5;
        locals.var_tmf1_dn6 = assign51280_e77482_d_n6;
        locals.var_tmf1_dn7 = assign51280_e77482_d_n7;
        locals.var_tmf1_dn8 = assign51280_e77482_d_n8;
        locals.var_tmf1_dn9 = assign51280_e77482_d_n9;
        locals.var_tmf1_dn10 = assign51280_e77482_d_n10;
        locals.var_tmf1_dn13 = assign51280_e77482_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign51290_e77500, assign51290_e77500_d_n0, assign51290_e77500_d_n2, assign51290_e77500_d_n4, assign51290_e77500_d_n5, assign51290_e77500_d_n6, assign51290_e77500_d_n7, assign51290_e77500_d_n8, assign51290_e77500_d_n9, assign51290_e77500_d_n10, assign51290_e77500_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1310 != 0.0)) {
        let assign51290_e77498: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign51290_e77498, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign51290_e77500;
        locals.var_x2_dn0 = assign51290_e77500_d_n0;
        locals.var_x2_dn2 = assign51290_e77500_d_n2;
        locals.var_x2_dn4 = assign51290_e77500_d_n4;
        locals.var_x2_dn5 = assign51290_e77500_d_n5;
        locals.var_x2_dn6 = assign51290_e77500_d_n6;
        locals.var_x2_dn7 = assign51290_e77500_d_n7;
        locals.var_x2_dn8 = assign51290_e77500_d_n8;
        locals.var_x2_dn9 = assign51290_e77500_d_n9;
        locals.var_x2_dn10 = assign51290_e77500_d_n10;
        locals.var_x2_dn13 = assign51290_e77500_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign51300_e77522, assign51300_e77522_d_n0, assign51300_e77522_d_n2, assign51300_e77522_d_n4, assign51300_e77522_d_n5, assign51300_e77522_d_n6, assign51300_e77522_d_n7, assign51300_e77522_d_n8, assign51300_e77522_d_n9, assign51300_e77522_d_n10, assign51300_e77522_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1310 != 0.0)) {
        let assign51300_e77516: f64 = (10.0 * 2.220446049250313e-16);
        let assign51300_e77519: f64 = (10.0 * 2.220446049250313e-16);
        let assign51300_e77520: f64 = (assign51300_e77516 * assign51300_e77519);
        (assign51300_e77520, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign51300_e77522;
        locals.var_xmax2_dn0 = assign51300_e77522_d_n0;
        locals.var_xmax2_dn2 = assign51300_e77522_d_n2;
        locals.var_xmax2_dn4 = assign51300_e77522_d_n4;
        locals.var_xmax2_dn5 = assign51300_e77522_d_n5;
        locals.var_xmax2_dn6 = assign51300_e77522_d_n6;
        locals.var_xmax2_dn7 = assign51300_e77522_d_n7;
        locals.var_xmax2_dn8 = assign51300_e77522_d_n8;
        locals.var_xmax2_dn9 = assign51300_e77522_d_n9;
        locals.var_xmax2_dn10 = assign51300_e77522_d_n10;
        locals.var_xmax2_dn13 = assign51300_e77522_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign51310_e77538, assign51310_e77538_d_n0, assign51310_e77538_d_n2, assign51310_e77538_d_n4, assign51310_e77538_d_n5, assign51310_e77538_d_n6, assign51310_e77538_d_n7, assign51310_e77538_d_n8, assign51310_e77538_d_n9, assign51310_e77538_d_n10, assign51310_e77538_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1310 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign51310_e77538;
        locals.var_xp_dn0 = assign51310_e77538_d_n0;
        locals.var_xp_dn2 = assign51310_e77538_d_n2;
        locals.var_xp_dn4 = assign51310_e77538_d_n4;
        locals.var_xp_dn5 = assign51310_e77538_d_n5;
        locals.var_xp_dn6 = assign51310_e77538_d_n6;
        locals.var_xp_dn7 = assign51310_e77538_d_n7;
        locals.var_xp_dn8 = assign51310_e77538_d_n8;
        locals.var_xp_dn9 = assign51310_e77538_d_n9;
        locals.var_xp_dn10 = assign51310_e77538_d_n10;
        locals.var_xp_dn13 = assign51310_e77538_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign51320_e77554, assign51320_e77554_d_n0, assign51320_e77554_d_n2, assign51320_e77554_d_n4, assign51320_e77554_d_n5, assign51320_e77554_d_n6, assign51320_e77554_d_n7, assign51320_e77554_d_n8, assign51320_e77554_d_n9, assign51320_e77554_d_n10, assign51320_e77554_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1310 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign51320_e77554;
        locals.var_xmp_dn0 = assign51320_e77554_d_n0;
        locals.var_xmp_dn2 = assign51320_e77554_d_n2;
        locals.var_xmp_dn4 = assign51320_e77554_d_n4;
        locals.var_xmp_dn5 = assign51320_e77554_d_n5;
        locals.var_xmp_dn6 = assign51320_e77554_d_n6;
        locals.var_xmp_dn7 = assign51320_e77554_d_n7;
        locals.var_xmp_dn8 = assign51320_e77554_d_n8;
        locals.var_xmp_dn9 = assign51320_e77554_d_n9;
        locals.var_xmp_dn10 = assign51320_e77554_d_n10;
        locals.var_xmp_dn13 = assign51320_e77554_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign51330_e77570,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1310 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign51330_e77570;
        locals.var_m0_rv = 0.0;

        let (assign51340_e77586,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1310 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign51340_e77586;
        locals.var_mm_rv = 0.0;

        let (assign51350_e77602, assign51350_e77602_d_n0, assign51350_e77602_d_n2, assign51350_e77602_d_n4, assign51350_e77602_d_n5, assign51350_e77602_d_n6, assign51350_e77602_d_n7, assign51350_e77602_d_n8, assign51350_e77602_d_n9, assign51350_e77602_d_n10, assign51350_e77602_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1310 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign51350_e77602;
        locals.var_arg_dn0 = assign51350_e77602_d_n0;
        locals.var_arg_dn2 = assign51350_e77602_d_n2;
        locals.var_arg_dn4 = assign51350_e77602_d_n4;
        locals.var_arg_dn5 = assign51350_e77602_d_n5;
        locals.var_arg_dn6 = assign51350_e77602_d_n6;
        locals.var_arg_dn7 = assign51350_e77602_d_n7;
        locals.var_arg_dn8 = assign51350_e77602_d_n8;
        locals.var_arg_dn9 = assign51350_e77602_d_n9;
        locals.var_arg_dn10 = assign51350_e77602_d_n10;
        locals.var_arg_dn13 = assign51350_e77602_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign51360_e77618, assign51360_e77618_d_n0, assign51360_e77618_d_n2, assign51360_e77618_d_n4, assign51360_e77618_d_n5, assign51360_e77618_d_n6, assign51360_e77618_d_n7, assign51360_e77618_d_n8, assign51360_e77618_d_n9, assign51360_e77618_d_n10, assign51360_e77618_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1310 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign51360_e77618;
        locals.var_dnm_dn0 = assign51360_e77618_d_n0;
        locals.var_dnm_dn2 = assign51360_e77618_d_n2;
        locals.var_dnm_dn4 = assign51360_e77618_d_n4;
        locals.var_dnm_dn5 = assign51360_e77618_d_n5;
        locals.var_dnm_dn6 = assign51360_e77618_d_n6;
        locals.var_dnm_dn7 = assign51360_e77618_d_n7;
        locals.var_dnm_dn8 = assign51360_e77618_d_n8;
        locals.var_dnm_dn9 = assign51360_e77618_d_n9;
        locals.var_dnm_dn10 = assign51360_e77618_d_n10;
        locals.var_dnm_dn13 = assign51360_e77618_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign51370_e77636, assign51370_e77636_d_n0, assign51370_e77636_d_n2, assign51370_e77636_d_n4, assign51370_e77636_d_n5, assign51370_e77636_d_n6, assign51370_e77636_d_n7, assign51370_e77636_d_n8, assign51370_e77636_d_n9, assign51370_e77636_d_n10, assign51370_e77636_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1310 != 0.0)) {
        let assign51370_e77634: f64 = (locals.var_xp * locals.var_x2);
        (assign51370_e77634, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign51370_e77636;
        locals.var_xp_dn0 = assign51370_e77636_d_n0;
        locals.var_xp_dn2 = assign51370_e77636_d_n2;
        locals.var_xp_dn4 = assign51370_e77636_d_n4;
        locals.var_xp_dn5 = assign51370_e77636_d_n5;
        locals.var_xp_dn6 = assign51370_e77636_d_n6;
        locals.var_xp_dn7 = assign51370_e77636_d_n7;
        locals.var_xp_dn8 = assign51370_e77636_d_n8;
        locals.var_xp_dn9 = assign51370_e77636_d_n9;
        locals.var_xp_dn10 = assign51370_e77636_d_n10;
        locals.var_xp_dn13 = assign51370_e77636_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign51380_e77654, assign51380_e77654_d_n0, assign51380_e77654_d_n2, assign51380_e77654_d_n4, assign51380_e77654_d_n5, assign51380_e77654_d_n6, assign51380_e77654_d_n7, assign51380_e77654_d_n8, assign51380_e77654_d_n9, assign51380_e77654_d_n10, assign51380_e77654_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1310 != 0.0)) {
        let assign51380_e77652: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign51380_e77652, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign51380_e77654;
        locals.var_xmp_dn0 = assign51380_e77654_d_n0;
        locals.var_xmp_dn2 = assign51380_e77654_d_n2;
        locals.var_xmp_dn4 = assign51380_e77654_d_n4;
        locals.var_xmp_dn5 = assign51380_e77654_d_n5;
        locals.var_xmp_dn6 = assign51380_e77654_d_n6;
        locals.var_xmp_dn7 = assign51380_e77654_d_n7;
        locals.var_xmp_dn8 = assign51380_e77654_d_n8;
        locals.var_xmp_dn9 = assign51380_e77654_d_n9;
        locals.var_xmp_dn10 = assign51380_e77654_d_n10;
        locals.var_xmp_dn13 = assign51380_e77654_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign51390_e77672, assign51390_e77672_d_n0, assign51390_e77672_d_n2, assign51390_e77672_d_n4, assign51390_e77672_d_n5, assign51390_e77672_d_n6, assign51390_e77672_d_n7, assign51390_e77672_d_n8, assign51390_e77672_d_n9, assign51390_e77672_d_n10, assign51390_e77672_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1310 != 0.0)) {
        let assign51390_e77670: f64 = (locals.var_xp * locals.var_x2);
        (assign51390_e77670, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign51390_e77672;
        locals.var_xp_dn0 = assign51390_e77672_d_n0;
        locals.var_xp_dn2 = assign51390_e77672_d_n2;
        locals.var_xp_dn4 = assign51390_e77672_d_n4;
        locals.var_xp_dn5 = assign51390_e77672_d_n5;
        locals.var_xp_dn6 = assign51390_e77672_d_n6;
        locals.var_xp_dn7 = assign51390_e77672_d_n7;
        locals.var_xp_dn8 = assign51390_e77672_d_n8;
        locals.var_xp_dn9 = assign51390_e77672_d_n9;
        locals.var_xp_dn10 = assign51390_e77672_d_n10;
        locals.var_xp_dn13 = assign51390_e77672_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign51400_e77690, assign51400_e77690_d_n0, assign51400_e77690_d_n2, assign51400_e77690_d_n4, assign51400_e77690_d_n5, assign51400_e77690_d_n6, assign51400_e77690_d_n7, assign51400_e77690_d_n8, assign51400_e77690_d_n9, assign51400_e77690_d_n10, assign51400_e77690_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1310 != 0.0)) {
        let assign51400_e77688: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign51400_e77688, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign51400_e77690;
        locals.var_xmp_dn0 = assign51400_e77690_d_n0;
        locals.var_xmp_dn2 = assign51400_e77690_d_n2;
        locals.var_xmp_dn4 = assign51400_e77690_d_n4;
        locals.var_xmp_dn5 = assign51400_e77690_d_n5;
        locals.var_xmp_dn6 = assign51400_e77690_d_n6;
        locals.var_xmp_dn7 = assign51400_e77690_d_n7;
        locals.var_xmp_dn8 = assign51400_e77690_d_n8;
        locals.var_xmp_dn9 = assign51400_e77690_d_n9;
        locals.var_xmp_dn10 = assign51400_e77690_d_n10;
        locals.var_xmp_dn13 = assign51400_e77690_d_n13;
        locals.var_xmp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_178(
        locals: &mut StampLocals,
    ) {
        let (assign51410_e77708, assign51410_e77708_d_n0, assign51410_e77708_d_n2, assign51410_e77708_d_n4, assign51410_e77708_d_n5, assign51410_e77708_d_n6, assign51410_e77708_d_n7, assign51410_e77708_d_n8, assign51410_e77708_d_n9, assign51410_e77708_d_n10, assign51410_e77708_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1310 != 0.0)) {
        let assign51410_e77706: f64 = (locals.var_xp + locals.var_xmp);
        (assign51410_e77706, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign51410_e77708;
        locals.var_arg_dn0 = assign51410_e77708_d_n0;
        locals.var_arg_dn2 = assign51410_e77708_d_n2;
        locals.var_arg_dn4 = assign51410_e77708_d_n4;
        locals.var_arg_dn5 = assign51410_e77708_d_n5;
        locals.var_arg_dn6 = assign51410_e77708_d_n6;
        locals.var_arg_dn7 = assign51410_e77708_d_n7;
        locals.var_arg_dn8 = assign51410_e77708_d_n8;
        locals.var_arg_dn9 = assign51410_e77708_d_n9;
        locals.var_arg_dn10 = assign51410_e77708_d_n10;
        locals.var_arg_dn13 = assign51410_e77708_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign51420_e77724, assign51420_e77724_d_n0, assign51420_e77724_d_n2, assign51420_e77724_d_n4, assign51420_e77724_d_n5, assign51420_e77724_d_n6, assign51420_e77724_d_n7, assign51420_e77724_d_n8, assign51420_e77724_d_n9, assign51420_e77724_d_n10, assign51420_e77724_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1310 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign51420_e77724;
        locals.var_dnm_dn0 = assign51420_e77724_d_n0;
        locals.var_dnm_dn2 = assign51420_e77724_d_n2;
        locals.var_dnm_dn4 = assign51420_e77724_d_n4;
        locals.var_dnm_dn5 = assign51420_e77724_d_n5;
        locals.var_dnm_dn6 = assign51420_e77724_d_n6;
        locals.var_dnm_dn7 = assign51420_e77724_d_n7;
        locals.var_dnm_dn8 = assign51420_e77724_d_n8;
        locals.var_dnm_dn9 = assign51420_e77724_d_n9;
        locals.var_dnm_dn10 = assign51420_e77724_d_n10;
        locals.var_dnm_dn13 = assign51420_e77724_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign51430_e77739: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1311 = assign51430_e77739;
        locals.var_guard1311_rv = 0.0;

        let assign51440_e77742: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1312 = assign51440_e77742;
        locals.var_guard1312_rv = 0.0;

        let (assign51450_e77762,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1310 != 0.0)) && (locals.var_guard1311 != 0.0)) && (locals.var_guard1312 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign51450_e77762;
        locals.var_mm_rv = 0.0;

        let assign51460_e77765: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1313 = assign51460_e77765;
        locals.var_guard1313_rv = 0.0;

        let (assign51470_e77788,) = {
    if (((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1310 != 0.0)) && (locals.var_guard1311 != 0.0)) && (locals.var_guard1312 == 0.0)) && (locals.var_guard1313 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign51470_e77788;
        locals.var_mm_rv = 0.0;

        let assign51480_e77791: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1314 = assign51480_e77791;
        locals.var_guard1314_rv = 0.0;

        let (assign51490_e77817,) = {
    if ((((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1310 != 0.0)) && (locals.var_guard1311 != 0.0)) && (locals.var_guard1312 == 0.0)) && (locals.var_guard1313 == 0.0)) && (locals.var_guard1314 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign51490_e77817;
        locals.var_mm_rv = 0.0;

        let assign51500_e77820: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1315 = assign51500_e77820;
        locals.var_guard1315_rv = 0.0;

        let (assign51510_e77849,) = {
    if (((((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1310 != 0.0)) && (locals.var_guard1311 != 0.0)) && (locals.var_guard1312 == 0.0)) && (locals.var_guard1313 == 0.0)) && (locals.var_guard1314 == 0.0)) && (locals.var_guard1315 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign51510_e77849;
        locals.var_mm_rv = 0.0;

        let (assign51520_e77867,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1310 != 0.0)) && (locals.var_guard1311 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign51520_e77867;
        locals.var_m0_rv = 0.0;

        let mut assign51530_loop_guard: usize = 0;
        while {
            let assign51530_cond_e77886: f64 = if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1310 != 0.0)) && (locals.var_guard1311 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign51530_cond_e77886 != 0.0
        } {
            assign51530_loop_guard += 1;
            assert!(assign51530_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign51530_body0_e77905, assign51530_body0_e77905_d_n0, assign51530_body0_e77905_d_n2, assign51530_body0_e77905_d_n4, assign51530_body0_e77905_d_n5, assign51530_body0_e77905_d_n6, assign51530_body0_e77905_d_n7, assign51530_body0_e77905_d_n8, assign51530_body0_e77905_d_n9, assign51530_body0_e77905_d_n10, assign51530_body0_e77905_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1310 != 0.0)) && (locals.var_guard1311 != 0.0)) {
        let assign51530_body0_e77903: f64 = (locals.var_dnm).sqrt();
        (assign51530_body0_e77903, (locals.var_dnm_dn0 / (2.0 * assign51530_body0_e77903)), (locals.var_dnm_dn2 / (2.0 * assign51530_body0_e77903)), (locals.var_dnm_dn4 / (2.0 * assign51530_body0_e77903)), (locals.var_dnm_dn5 / (2.0 * assign51530_body0_e77903)), (locals.var_dnm_dn6 / (2.0 * assign51530_body0_e77903)), (locals.var_dnm_dn7 / (2.0 * assign51530_body0_e77903)), (locals.var_dnm_dn8 / (2.0 * assign51530_body0_e77903)), (locals.var_dnm_dn9 / (2.0 * assign51530_body0_e77903)), (locals.var_dnm_dn10 / (2.0 * assign51530_body0_e77903)), (locals.var_dnm_dn13 / (2.0 * assign51530_body0_e77903)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign51530_body0_e77905;
            locals.var_dnm_dn0 = assign51530_body0_e77905_d_n0;
            locals.var_dnm_dn2 = assign51530_body0_e77905_d_n2;
            locals.var_dnm_dn4 = assign51530_body0_e77905_d_n4;
            locals.var_dnm_dn5 = assign51530_body0_e77905_d_n5;
            locals.var_dnm_dn6 = assign51530_body0_e77905_d_n6;
            locals.var_dnm_dn7 = assign51530_body0_e77905_d_n7;
            locals.var_dnm_dn8 = assign51530_body0_e77905_d_n8;
            locals.var_dnm_dn9 = assign51530_body0_e77905_d_n9;
            locals.var_dnm_dn10 = assign51530_body0_e77905_d_n10;
            locals.var_dnm_dn13 = assign51530_body0_e77905_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign51530_body1_e77925,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1310 != 0.0)) && (locals.var_guard1311 != 0.0)) {
        let assign51530_body1_e77923: f64 = (locals.var_m0 + 1.0);
        (assign51530_body1_e77923,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign51530_body1_e77925;
            locals.var_m0_rv = 0.0;
        }

        let (assign51540_e77955, assign51540_e77955_d_n0, assign51540_e77955_d_n2, assign51540_e77955_d_n4, assign51540_e77955_d_n5, assign51540_e77955_d_n6, assign51540_e77955_d_n7, assign51540_e77955_d_n8, assign51540_e77955_d_n9, assign51540_e77955_d_n10, assign51540_e77955_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1310 != 0.0)) && (locals.var_guard1311 == 0.0)) {
        let (assign51540_e77953, assign51540_e77953_d_n0, assign51540_e77953_d_n2, assign51540_e77953_d_n4, assign51540_e77953_d_n5, assign51540_e77953_d_n6, assign51540_e77953_d_n7, assign51540_e77953_d_n8, assign51540_e77953_d_n9, assign51540_e77953_d_n10, assign51540_e77953_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign51540_e77950: f64 = (2.0 * 2.0);
                let assign51540_e77951: f64 = (1.0 / assign51540_e77950);
                let assign51540_e77952: f64 = (locals.var_dnm).powf(assign51540_e77951);
                (assign51540_e77952, if 0.0 == 0.0 && ((assign51540_e77951) as f64).is_finite() && ((assign51540_e77951) as f64).fract() == 0.0 { if assign51540_e77951 == 0.0 { 0.0 } else { (assign51540_e77951 * ((locals.var_dnm).powf(assign51540_e77951 - 1.0) * locals.var_dnm_dn0)) } } else { (assign51540_e77952 * (assign51540_e77951 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign51540_e77951) as f64).is_finite() && ((assign51540_e77951) as f64).fract() == 0.0 { if assign51540_e77951 == 0.0 { 0.0 } else { (assign51540_e77951 * ((locals.var_dnm).powf(assign51540_e77951 - 1.0) * locals.var_dnm_dn2)) } } else { (assign51540_e77952 * (assign51540_e77951 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign51540_e77951) as f64).is_finite() && ((assign51540_e77951) as f64).fract() == 0.0 { if assign51540_e77951 == 0.0 { 0.0 } else { (assign51540_e77951 * ((locals.var_dnm).powf(assign51540_e77951 - 1.0) * locals.var_dnm_dn4)) } } else { (assign51540_e77952 * (assign51540_e77951 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign51540_e77951) as f64).is_finite() && ((assign51540_e77951) as f64).fract() == 0.0 { if assign51540_e77951 == 0.0 { 0.0 } else { (assign51540_e77951 * ((locals.var_dnm).powf(assign51540_e77951 - 1.0) * locals.var_dnm_dn5)) } } else { (assign51540_e77952 * (assign51540_e77951 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign51540_e77951) as f64).is_finite() && ((assign51540_e77951) as f64).fract() == 0.0 { if assign51540_e77951 == 0.0 { 0.0 } else { (assign51540_e77951 * ((locals.var_dnm).powf(assign51540_e77951 - 1.0) * locals.var_dnm_dn6)) } } else { (assign51540_e77952 * (assign51540_e77951 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign51540_e77951) as f64).is_finite() && ((assign51540_e77951) as f64).fract() == 0.0 { if assign51540_e77951 == 0.0 { 0.0 } else { (assign51540_e77951 * ((locals.var_dnm).powf(assign51540_e77951 - 1.0) * locals.var_dnm_dn7)) } } else { (assign51540_e77952 * (assign51540_e77951 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign51540_e77951) as f64).is_finite() && ((assign51540_e77951) as f64).fract() == 0.0 { if assign51540_e77951 == 0.0 { 0.0 } else { (assign51540_e77951 * ((locals.var_dnm).powf(assign51540_e77951 - 1.0) * locals.var_dnm_dn8)) } } else { (assign51540_e77952 * (assign51540_e77951 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign51540_e77951) as f64).is_finite() && ((assign51540_e77951) as f64).fract() == 0.0 { if assign51540_e77951 == 0.0 { 0.0 } else { (assign51540_e77951 * ((locals.var_dnm).powf(assign51540_e77951 - 1.0) * locals.var_dnm_dn9)) } } else { (assign51540_e77952 * (assign51540_e77951 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign51540_e77951) as f64).is_finite() && ((assign51540_e77951) as f64).fract() == 0.0 { if assign51540_e77951 == 0.0 { 0.0 } else { (assign51540_e77951 * ((locals.var_dnm).powf(assign51540_e77951 - 1.0) * locals.var_dnm_dn10)) } } else { (assign51540_e77952 * (assign51540_e77951 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign51540_e77951) as f64).is_finite() && ((assign51540_e77951) as f64).fract() == 0.0 { if assign51540_e77951 == 0.0 { 0.0 } else { (assign51540_e77951 * ((locals.var_dnm).powf(assign51540_e77951 - 1.0) * locals.var_dnm_dn13)) } } else { (assign51540_e77952 * (assign51540_e77951 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign51540_e77953, assign51540_e77953_d_n0, assign51540_e77953_d_n2, assign51540_e77953_d_n4, assign51540_e77953_d_n5, assign51540_e77953_d_n6, assign51540_e77953_d_n7, assign51540_e77953_d_n8, assign51540_e77953_d_n9, assign51540_e77953_d_n10, assign51540_e77953_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign51540_e77955;
        locals.var_dnm_dn0 = assign51540_e77955_d_n0;
        locals.var_dnm_dn2 = assign51540_e77955_d_n2;
        locals.var_dnm_dn4 = assign51540_e77955_d_n4;
        locals.var_dnm_dn5 = assign51540_e77955_d_n5;
        locals.var_dnm_dn6 = assign51540_e77955_d_n6;
        locals.var_dnm_dn7 = assign51540_e77955_d_n7;
        locals.var_dnm_dn8 = assign51540_e77955_d_n8;
        locals.var_dnm_dn9 = assign51540_e77955_d_n9;
        locals.var_dnm_dn10 = assign51540_e77955_d_n10;
        locals.var_dnm_dn13 = assign51540_e77955_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign51550_e77973, assign51550_e77973_d_n0, assign51550_e77973_d_n2, assign51550_e77973_d_n4, assign51550_e77973_d_n5, assign51550_e77973_d_n6, assign51550_e77973_d_n7, assign51550_e77973_d_n8, assign51550_e77973_d_n9, assign51550_e77973_d_n10, assign51550_e77973_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1310 != 0.0)) {
        let assign51550_e77971: f64 = (1.0 / locals.var_dnm);
        (assign51550_e77971, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign51550_e77973;
        locals.var_dnm_dn0 = assign51550_e77973_d_n0;
        locals.var_dnm_dn2 = assign51550_e77973_d_n2;
        locals.var_dnm_dn4 = assign51550_e77973_d_n4;
        locals.var_dnm_dn5 = assign51550_e77973_d_n5;
        locals.var_dnm_dn6 = assign51550_e77973_d_n6;
        locals.var_dnm_dn7 = assign51550_e77973_d_n7;
        locals.var_dnm_dn8 = assign51550_e77973_d_n8;
        locals.var_dnm_dn9 = assign51550_e77973_d_n9;
        locals.var_dnm_dn10 = assign51550_e77973_d_n10;
        locals.var_dnm_dn13 = assign51550_e77973_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign51560_e77995, assign51560_e77995_d_n0, assign51560_e77995_d_n2, assign51560_e77995_d_n4, assign51560_e77995_d_n5, assign51560_e77995_d_n6, assign51560_e77995_d_n7, assign51560_e77995_d_n8, assign51560_e77995_d_n9, assign51560_e77995_d_n10, assign51560_e77995_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1310 != 0.0)) {
        let assign51560_e77990: f64 = (10.0 * 2.220446049250313e-16);
        let assign51560_e77991: f64 = (locals.var_tmf1 * assign51560_e77990);
        let assign51560_e77993: f64 = (assign51560_e77991 * locals.var_dnm);
        (assign51560_e77993, (((locals.var_tmf1_dn0 * assign51560_e77990) * locals.var_dnm) + (assign51560_e77991 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * assign51560_e77990) * locals.var_dnm) + (assign51560_e77991 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * assign51560_e77990) * locals.var_dnm) + (assign51560_e77991 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * assign51560_e77990) * locals.var_dnm) + (assign51560_e77991 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * assign51560_e77990) * locals.var_dnm) + (assign51560_e77991 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * assign51560_e77990) * locals.var_dnm) + (assign51560_e77991 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * assign51560_e77990) * locals.var_dnm) + (assign51560_e77991 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * assign51560_e77990) * locals.var_dnm) + (assign51560_e77991 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * assign51560_e77990) * locals.var_dnm) + (assign51560_e77991 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * assign51560_e77990) * locals.var_dnm) + (assign51560_e77991 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign51560_e77995;
        locals.var_tmf0_dn0 = assign51560_e77995_d_n0;
        locals.var_tmf0_dn2 = assign51560_e77995_d_n2;
        locals.var_tmf0_dn4 = assign51560_e77995_d_n4;
        locals.var_tmf0_dn5 = assign51560_e77995_d_n5;
        locals.var_tmf0_dn6 = assign51560_e77995_d_n6;
        locals.var_tmf0_dn7 = assign51560_e77995_d_n7;
        locals.var_tmf0_dn8 = assign51560_e77995_d_n8;
        locals.var_tmf0_dn9 = assign51560_e77995_d_n9;
        locals.var_tmf0_dn10 = assign51560_e77995_d_n10;
        locals.var_tmf0_dn13 = assign51560_e77995_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign51570_e78019, assign51570_e78019_d_n0, assign51570_e78019_d_n2, assign51570_e78019_d_n4, assign51570_e78019_d_n5, assign51570_e78019_d_n6, assign51570_e78019_d_n7, assign51570_e78019_d_n8, assign51570_e78019_d_n9, assign51570_e78019_d_n10, assign51570_e78019_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1310 != 0.0)) {
        let assign51570_e78011: f64 = (10.0 * 2.220446049250313e-16);
        let assign51570_e78013: f64 = (assign51570_e78011 * locals.var_xmp);
        let assign51570_e78015: f64 = (assign51570_e78013 * locals.var_dnm);
        let assign51570_e78017: f64 = (assign51570_e78015 / locals.var_arg);
        (assign51570_e78017, ((((((assign51570_e78011 * locals.var_xmp_dn0) * locals.var_dnm) + (assign51570_e78013 * locals.var_dnm_dn0)) * locals.var_arg) - (assign51570_e78015 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((assign51570_e78011 * locals.var_xmp_dn2) * locals.var_dnm) + (assign51570_e78013 * locals.var_dnm_dn2)) * locals.var_arg) - (assign51570_e78015 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((assign51570_e78011 * locals.var_xmp_dn4) * locals.var_dnm) + (assign51570_e78013 * locals.var_dnm_dn4)) * locals.var_arg) - (assign51570_e78015 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((assign51570_e78011 * locals.var_xmp_dn5) * locals.var_dnm) + (assign51570_e78013 * locals.var_dnm_dn5)) * locals.var_arg) - (assign51570_e78015 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((assign51570_e78011 * locals.var_xmp_dn6) * locals.var_dnm) + (assign51570_e78013 * locals.var_dnm_dn6)) * locals.var_arg) - (assign51570_e78015 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((assign51570_e78011 * locals.var_xmp_dn7) * locals.var_dnm) + (assign51570_e78013 * locals.var_dnm_dn7)) * locals.var_arg) - (assign51570_e78015 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((assign51570_e78011 * locals.var_xmp_dn8) * locals.var_dnm) + (assign51570_e78013 * locals.var_dnm_dn8)) * locals.var_arg) - (assign51570_e78015 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((assign51570_e78011 * locals.var_xmp_dn9) * locals.var_dnm) + (assign51570_e78013 * locals.var_dnm_dn9)) * locals.var_arg) - (assign51570_e78015 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((assign51570_e78011 * locals.var_xmp_dn10) * locals.var_dnm) + (assign51570_e78013 * locals.var_dnm_dn10)) * locals.var_arg) - (assign51570_e78015 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((assign51570_e78011 * locals.var_xmp_dn13) * locals.var_dnm) + (assign51570_e78013 * locals.var_dnm_dn13)) * locals.var_arg) - (assign51570_e78015 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign51570_e78019;
        locals.var_t0_dn0 = assign51570_e78019_d_n0;
        locals.var_t0_dn2 = assign51570_e78019_d_n2;
        locals.var_t0_dn4 = assign51570_e78019_d_n4;
        locals.var_t0_dn5 = assign51570_e78019_d_n5;
        locals.var_t0_dn6 = assign51570_e78019_d_n6;
        locals.var_t0_dn7 = assign51570_e78019_d_n7;
        locals.var_t0_dn8 = assign51570_e78019_d_n8;
        locals.var_t0_dn9 = assign51570_e78019_d_n9;
        locals.var_t0_dn10 = assign51570_e78019_d_n10;
        locals.var_t0_dn13 = assign51570_e78019_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign51580_e78047, assign51580_e78047_d_n0, assign51580_e78047_d_n2, assign51580_e78047_d_n4, assign51580_e78047_d_n5, assign51580_e78047_d_n6, assign51580_e78047_d_n7, assign51580_e78047_d_n8, assign51580_e78047_d_n9, assign51580_e78047_d_n10, assign51580_e78047_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1310 != 0.0)) {
        let assign51580_e78035: f64 = (locals.var_ps0z + locals.var_vds);
        let assign51580_e78038: f64 = (10.0 * 2.220446049250313e-16);
        let assign51580_e78039: f64 = (assign51580_e78035 - assign51580_e78038);
        let assign51580_e78042: f64 = (10.0 * 2.220446049250313e-16);
        let assign51580_e78043: f64 = (assign51580_e78039 - assign51580_e78042);
        let assign51580_e78045: f64 = (assign51580_e78043 + locals.var_tmf0);
        (assign51580_e78045, ((locals.var_ps0z_dn0 + locals.var_vds_dn0) + locals.var_tmf0_dn0), ((locals.var_ps0z_dn2 + locals.var_vds_dn2) + locals.var_tmf0_dn2), ((locals.var_ps0z_dn4 + locals.var_vds_dn4) + locals.var_tmf0_dn4), ((locals.var_ps0z_dn5 + locals.var_vds_dn5) + locals.var_tmf0_dn5), ((locals.var_ps0z_dn6 + locals.var_vds_dn6) + locals.var_tmf0_dn6), ((locals.var_ps0z_dn7 + locals.var_vds_dn7) + locals.var_tmf0_dn7), ((locals.var_ps0z_dn8 + locals.var_vds_dn8) + locals.var_tmf0_dn8), ((locals.var_ps0z_dn9 + locals.var_vds_dn9) + locals.var_tmf0_dn9), ((locals.var_ps0z_dn10 + locals.var_vds_dn10) + locals.var_tmf0_dn10), ((locals.var_ps0z_dn13 + locals.var_vds_dn13) + locals.var_tmf0_dn13),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn13,)
    }
};
        locals.var_psdl = assign51580_e78047;
        locals.var_psdl_dn0 = assign51580_e78047_d_n0;
        locals.var_psdl_dn2 = assign51580_e78047_d_n2;
        locals.var_psdl_dn4 = assign51580_e78047_d_n4;
        locals.var_psdl_dn5 = assign51580_e78047_d_n5;
        locals.var_psdl_dn6 = assign51580_e78047_d_n6;
        locals.var_psdl_dn7 = assign51580_e78047_d_n7;
        locals.var_psdl_dn8 = assign51580_e78047_d_n8;
        locals.var_psdl_dn9 = assign51580_e78047_d_n9;
        locals.var_psdl_dn10 = assign51580_e78047_d_n10;
        locals.var_psdl_dn13 = assign51580_e78047_d_n13;
        locals.var_psdl_rv = 0.0;

        let (assign51590_e78063, assign51590_e78063_d_n0, assign51590_e78063_d_n2, assign51590_e78063_d_n4, assign51590_e78063_d_n5, assign51590_e78063_d_n6, assign51590_e78063_d_n7, assign51590_e78063_d_n8, assign51590_e78063_d_n9, assign51590_e78063_d_n10, assign51590_e78063_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1310 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign51590_e78063;
        locals.var_t0_dn0 = assign51590_e78063_d_n0;
        locals.var_t0_dn2 = assign51590_e78063_d_n2;
        locals.var_t0_dn4 = assign51590_e78063_d_n4;
        locals.var_t0_dn5 = assign51590_e78063_d_n5;
        locals.var_t0_dn6 = assign51590_e78063_d_n6;
        locals.var_t0_dn7 = assign51590_e78063_d_n7;
        locals.var_t0_dn8 = assign51590_e78063_d_n8;
        locals.var_t0_dn9 = assign51590_e78063_d_n9;
        locals.var_t0_dn10 = assign51590_e78063_d_n10;
        locals.var_t0_dn13 = assign51590_e78063_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign51600_e78080, assign51600_e78080_d_n0, assign51600_e78080_d_n2, assign51600_e78080_d_n4, assign51600_e78080_d_n5, assign51600_e78080_d_n6, assign51600_e78080_d_n7, assign51600_e78080_d_n8, assign51600_e78080_d_n9, assign51600_e78080_d_n10, assign51600_e78080_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1310 == 0.0)) {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn13,)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn13,)
    }
};
        locals.var_psdl = assign51600_e78080;
        locals.var_psdl_dn0 = assign51600_e78080_d_n0;
        locals.var_psdl_dn2 = assign51600_e78080_d_n2;
        locals.var_psdl_dn4 = assign51600_e78080_d_n4;
        locals.var_psdl_dn5 = assign51600_e78080_d_n5;
        locals.var_psdl_dn6 = assign51600_e78080_d_n6;
        locals.var_psdl_dn7 = assign51600_e78080_d_n7;
        locals.var_psdl_dn8 = assign51600_e78080_d_n8;
        locals.var_psdl_dn9 = assign51600_e78080_d_n9;
        locals.var_psdl_dn10 = assign51600_e78080_d_n10;
        locals.var_psdl_dn13 = assign51600_e78080_d_n13;
        locals.var_psdl_rv = 0.0;

        let (assign51610_e78097, assign51610_e78097_d_n0, assign51610_e78097_d_n2, assign51610_e78097_d_n4, assign51610_e78097_d_n5, assign51610_e78097_d_n6, assign51610_e78097_d_n7, assign51610_e78097_d_n8, assign51610_e78097_d_n9, assign51610_e78097_d_n10, assign51610_e78097_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) && (locals.var_guard1310 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign51610_e78097;
        locals.var_t0_dn0 = assign51610_e78097_d_n0;
        locals.var_t0_dn2 = assign51610_e78097_d_n2;
        locals.var_t0_dn4 = assign51610_e78097_d_n4;
        locals.var_t0_dn5 = assign51610_e78097_d_n5;
        locals.var_t0_dn6 = assign51610_e78097_d_n6;
        locals.var_t0_dn7 = assign51610_e78097_d_n7;
        locals.var_t0_dn8 = assign51610_e78097_d_n8;
        locals.var_t0_dn9 = assign51610_e78097_d_n9;
        locals.var_t0_dn10 = assign51610_e78097_d_n10;
        locals.var_t0_dn13 = assign51610_e78097_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign51620_e78113, assign51620_e78113_d_n0, assign51620_e78113_d_n2, assign51620_e78113_d_n4, assign51620_e78113_d_n5, assign51620_e78113_d_n6, assign51620_e78113_d_n7, assign51620_e78113_d_n8, assign51620_e78113_d_n9, assign51620_e78113_d_n10, assign51620_e78113_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) {
        let assign51620_e78111: f64 = (locals.var_psdl - locals.var_psl);
        (assign51620_e78111, (locals.var_psdl_dn0 - locals.var_psl_dn0), (locals.var_psdl_dn2 - locals.var_psl_dn2), (locals.var_psdl_dn4 - locals.var_psl_dn4), (locals.var_psdl_dn5 - locals.var_psl_dn5), (locals.var_psdl_dn6 - locals.var_psl_dn6), (locals.var_psdl_dn7 - locals.var_psl_dn7), (locals.var_psdl_dn8 - locals.var_psl_dn8), (locals.var_psdl_dn9 - locals.var_psl_dn9), (locals.var_psdl_dn10 - locals.var_psl_dn10), (locals.var_psdl_dn13 - locals.var_psl_dn13),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign51620_e78113;
        locals.var_t6_dn0 = assign51620_e78113_d_n0;
        locals.var_t6_dn2 = assign51620_e78113_d_n2;
        locals.var_t6_dn4 = assign51620_e78113_d_n4;
        locals.var_t6_dn5 = assign51620_e78113_d_n5;
        locals.var_t6_dn6 = assign51620_e78113_d_n6;
        locals.var_t6_dn7 = assign51620_e78113_d_n7;
        locals.var_t6_dn8 = assign51620_e78113_d_n8;
        locals.var_t6_dn9 = assign51620_e78113_d_n9;
        locals.var_t6_dn10 = assign51620_e78113_d_n10;
        locals.var_t6_dn13 = assign51620_e78113_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign51630_e78129, assign51630_e78129_d_n0, assign51630_e78129_d_n2, assign51630_e78129_d_n4, assign51630_e78129_d_n5, assign51630_e78129_d_n6, assign51630_e78129_d_n7, assign51630_e78129_d_n8, assign51630_e78129_d_n9, assign51630_e78129_d_n10, assign51630_e78129_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) {
        let assign51630_e78127: f64 = (locals.var_beta * locals.var_qn0);
        (assign51630_e78127, ((locals.var_beta_dn0 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn0)), ((locals.var_beta_dn2 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn2)), ((locals.var_beta_dn4 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn4)), ((locals.var_beta_dn5 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn5)), ((locals.var_beta_dn6 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn6)), ((locals.var_beta_dn7 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn7)), ((locals.var_beta_dn8 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn8)), ((locals.var_beta_dn9 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn9)), ((locals.var_beta_dn10 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn10)), ((locals.var_beta_dn13 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn13)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign51630_e78129;
        locals.var_t3_dn0 = assign51630_e78129_d_n0;
        locals.var_t3_dn2 = assign51630_e78129_d_n2;
        locals.var_t3_dn4 = assign51630_e78129_d_n4;
        locals.var_t3_dn5 = assign51630_e78129_d_n5;
        locals.var_t3_dn6 = assign51630_e78129_d_n6;
        locals.var_t3_dn7 = assign51630_e78129_d_n7;
        locals.var_t3_dn8 = assign51630_e78129_d_n8;
        locals.var_t3_dn9 = assign51630_e78129_d_n9;
        locals.var_t3_dn10 = assign51630_e78129_d_n10;
        locals.var_t3_dn13 = assign51630_e78129_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign51640_e78145, assign51640_e78145_d_n0, assign51640_e78145_d_n2, assign51640_e78145_d_n4, assign51640_e78145_d_n5, assign51640_e78145_d_n6, assign51640_e78145_d_n7, assign51640_e78145_d_n8, assign51640_e78145_d_n9, assign51640_e78145_d_n10, assign51640_e78145_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) {
        let assign51640_e78143: f64 = (1.0 / locals.var_t3);
        (assign51640_e78143, (-(locals.var_t3_dn0 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn2 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn13 / (locals.var_t3 * locals.var_t3))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign51640_e78145;
        locals.var_t1_dn0 = assign51640_e78145_d_n0;
        locals.var_t1_dn2 = assign51640_e78145_d_n2;
        locals.var_t1_dn4 = assign51640_e78145_d_n4;
        locals.var_t1_dn5 = assign51640_e78145_d_n5;
        locals.var_t1_dn6 = assign51640_e78145_d_n6;
        locals.var_t1_dn7 = assign51640_e78145_d_n7;
        locals.var_t1_dn8 = assign51640_e78145_d_n8;
        locals.var_t1_dn9 = assign51640_e78145_d_n9;
        locals.var_t1_dn10 = assign51640_e78145_d_n10;
        locals.var_t1_dn13 = assign51640_e78145_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign51650_e78161, assign51650_e78161_d_n0, assign51650_e78161_d_n2, assign51650_e78161_d_n4, assign51650_e78161_d_n5, assign51650_e78161_d_n6, assign51650_e78161_d_n7, assign51650_e78161_d_n8, assign51650_e78161_d_n9, assign51650_e78161_d_n10, assign51650_e78161_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) {
        let assign51650_e78159: f64 = (locals.var_idd * locals.var_t1);
        (assign51650_e78159, ((locals.var_idd_dn0 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn0)), ((locals.var_idd_dn2 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn2)), ((locals.var_idd_dn4 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn4)), ((locals.var_idd_dn5 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn5)), ((locals.var_idd_dn6 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn6)), ((locals.var_idd_dn7 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn7)), ((locals.var_idd_dn8 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn8)), ((locals.var_idd_dn9 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn9)), ((locals.var_idd_dn10 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn10)), ((locals.var_idd_dn13 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn13)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign51650_e78161;
        locals.var_t5_dn0 = assign51650_e78161_d_n0;
        locals.var_t5_dn2 = assign51650_e78161_d_n2;
        locals.var_t5_dn4 = assign51650_e78161_d_n4;
        locals.var_t5_dn5 = assign51650_e78161_d_n5;
        locals.var_t5_dn6 = assign51650_e78161_d_n6;
        locals.var_t5_dn7 = assign51650_e78161_d_n7;
        locals.var_t5_dn8 = assign51650_e78161_d_n8;
        locals.var_t5_dn9 = assign51650_e78161_d_n9;
        locals.var_t5_dn10 = assign51650_e78161_d_n10;
        locals.var_t5_dn13 = assign51650_e78161_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign51660_e78177, assign51660_e78177_d_n0, assign51660_e78177_d_n2, assign51660_e78177_d_n4, assign51660_e78177_d_n5, assign51660_e78177_d_n6, assign51660_e78177_d_n7, assign51660_e78177_d_n8, assign51660_e78177_d_n9, assign51660_e78177_d_n10, assign51660_e78177_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) {
        let assign51660_e78175: f64 = (locals.var_q_ndepm__blk1133 / 1.034943e-10);
        (assign51660_e78175, (locals.var_q_ndepm__blk1133_dn0 / 1.034943e-10), (locals.var_q_ndepm__blk1133_dn2 / 1.034943e-10), (locals.var_q_ndepm__blk1133_dn4 / 1.034943e-10), (locals.var_q_ndepm__blk1133_dn5 / 1.034943e-10), (locals.var_q_ndepm__blk1133_dn6 / 1.034943e-10), (locals.var_q_ndepm__blk1133_dn7 / 1.034943e-10), (locals.var_q_ndepm__blk1133_dn8 / 1.034943e-10), (locals.var_q_ndepm__blk1133_dn9 / 1.034943e-10), (locals.var_q_ndepm__blk1133_dn10 / 1.034943e-10), (locals.var_q_ndepm__blk1133_dn13 / 1.034943e-10),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign51660_e78177;
        locals.var_t10_dn0 = assign51660_e78177_d_n0;
        locals.var_t10_dn2 = assign51660_e78177_d_n2;
        locals.var_t10_dn4 = assign51660_e78177_d_n4;
        locals.var_t10_dn5 = assign51660_e78177_d_n5;
        locals.var_t10_dn6 = assign51660_e78177_d_n6;
        locals.var_t10_dn7 = assign51660_e78177_d_n7;
        locals.var_t10_dn8 = assign51660_e78177_d_n8;
        locals.var_t10_dn9 = assign51660_e78177_d_n9;
        locals.var_t10_dn10 = assign51660_e78177_d_n10;
        locals.var_t10_dn13 = assign51660_e78177_d_n13;
        locals.var_t10_rv = 0.0;

        let (assign51670_e78191, assign51670_e78191_d_n0, assign51670_e78191_d_n2, assign51670_e78191_d_n4, assign51670_e78191_d_n5, assign51670_e78191_d_n6, assign51670_e78191_d_n7, assign51670_e78191_d_n8, assign51670_e78191_d_n9, assign51670_e78191_d_n10, assign51670_e78191_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) {
        (100000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign51670_e78191;
        locals.var_t1_dn0 = assign51670_e78191_d_n0;
        locals.var_t1_dn2 = assign51670_e78191_d_n2;
        locals.var_t1_dn4 = assign51670_e78191_d_n4;
        locals.var_t1_dn5 = assign51670_e78191_d_n5;
        locals.var_t1_dn6 = assign51670_e78191_d_n6;
        locals.var_t1_dn7 = assign51670_e78191_d_n7;
        locals.var_t1_dn8 = assign51670_e78191_d_n8;
        locals.var_t1_dn9 = assign51670_e78191_d_n9;
        locals.var_t1_dn10 = assign51670_e78191_d_n10;
        locals.var_t1_dn13 = assign51670_e78191_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign51680_e78207, assign51680_e78207_d_n0, assign51680_e78207_d_n2, assign51680_e78207_d_n4, assign51680_e78207_d_n5, assign51680_e78207_d_n6, assign51680_e78207_d_n7, assign51680_e78207_d_n8, assign51680_e78207_d_n9, assign51680_e78207_d_n10, assign51680_e78207_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) {
        let assign51680_e78205: f64 = (1.0 / locals.var_leff);
        (assign51680_e78205, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign51680_e78207;
        locals.var_t2_dn0 = assign51680_e78207_d_n0;
        locals.var_t2_dn2 = assign51680_e78207_d_n2;
        locals.var_t2_dn4 = assign51680_e78207_d_n4;
        locals.var_t2_dn5 = assign51680_e78207_d_n5;
        locals.var_t2_dn6 = assign51680_e78207_d_n6;
        locals.var_t2_dn7 = assign51680_e78207_d_n7;
        locals.var_t2_dn8 = assign51680_e78207_d_n8;
        locals.var_t2_dn9 = assign51680_e78207_d_n9;
        locals.var_t2_dn10 = assign51680_e78207_d_n10;
        locals.var_t2_dn13 = assign51680_e78207_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign51690_e78237, assign51690_e78237_d_n0, assign51690_e78237_d_n2, assign51690_e78237_d_n4, assign51690_e78237_d_n5, assign51690_e78237_d_n6, assign51690_e78237_d_n7, assign51690_e78237_d_n8, assign51690_e78237_d_n9, assign51690_e78237_d_n10, assign51690_e78237_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) {
        let assign51690_e78221: f64 = (2.0 * locals.var_t5);
        let assign51690_e78224: f64 = (2.0 * locals.var_t10);
        let assign51690_e78226: f64 = (assign51690_e78224 * locals.var_t6);
        let assign51690_e78228: f64 = (assign51690_e78226 * locals.var_t4);
        let assign51690_e78229: f64 = (assign51690_e78221 + assign51690_e78228);
        let assign51690_e78232: f64 = (locals.var_t1 * locals.var_t4);
        let assign51690_e78233: f64 = (assign51690_e78229 + assign51690_e78232);
        let assign51690_e78235: f64 = (assign51690_e78233 * locals.var_t2);
        (assign51690_e78235, (((((2.0 * locals.var_t5_dn0) + (((((2.0 * locals.var_t10_dn0) * locals.var_t6) + (assign51690_e78224 * locals.var_t6_dn0)) * locals.var_t4) + (assign51690_e78226 * locals.var_t4_dn0))) + ((locals.var_t1_dn0 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn0))) * locals.var_t2) + (assign51690_e78233 * locals.var_t2_dn0)), (((((2.0 * locals.var_t5_dn2) + (((((2.0 * locals.var_t10_dn2) * locals.var_t6) + (assign51690_e78224 * locals.var_t6_dn2)) * locals.var_t4) + (assign51690_e78226 * locals.var_t4_dn2))) + ((locals.var_t1_dn2 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn2))) * locals.var_t2) + (assign51690_e78233 * locals.var_t2_dn2)), (((((2.0 * locals.var_t5_dn4) + (((((2.0 * locals.var_t10_dn4) * locals.var_t6) + (assign51690_e78224 * locals.var_t6_dn4)) * locals.var_t4) + (assign51690_e78226 * locals.var_t4_dn4))) + ((locals.var_t1_dn4 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn4))) * locals.var_t2) + (assign51690_e78233 * locals.var_t2_dn4)), (((((2.0 * locals.var_t5_dn5) + (((((2.0 * locals.var_t10_dn5) * locals.var_t6) + (assign51690_e78224 * locals.var_t6_dn5)) * locals.var_t4) + (assign51690_e78226 * locals.var_t4_dn5))) + ((locals.var_t1_dn5 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn5))) * locals.var_t2) + (assign51690_e78233 * locals.var_t2_dn5)), (((((2.0 * locals.var_t5_dn6) + (((((2.0 * locals.var_t10_dn6) * locals.var_t6) + (assign51690_e78224 * locals.var_t6_dn6)) * locals.var_t4) + (assign51690_e78226 * locals.var_t4_dn6))) + ((locals.var_t1_dn6 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn6))) * locals.var_t2) + (assign51690_e78233 * locals.var_t2_dn6)), (((((2.0 * locals.var_t5_dn7) + (((((2.0 * locals.var_t10_dn7) * locals.var_t6) + (assign51690_e78224 * locals.var_t6_dn7)) * locals.var_t4) + (assign51690_e78226 * locals.var_t4_dn7))) + ((locals.var_t1_dn7 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn7))) * locals.var_t2) + (assign51690_e78233 * locals.var_t2_dn7)), (((((2.0 * locals.var_t5_dn8) + (((((2.0 * locals.var_t10_dn8) * locals.var_t6) + (assign51690_e78224 * locals.var_t6_dn8)) * locals.var_t4) + (assign51690_e78226 * locals.var_t4_dn8))) + ((locals.var_t1_dn8 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn8))) * locals.var_t2) + (assign51690_e78233 * locals.var_t2_dn8)), (((((2.0 * locals.var_t5_dn9) + (((((2.0 * locals.var_t10_dn9) * locals.var_t6) + (assign51690_e78224 * locals.var_t6_dn9)) * locals.var_t4) + (assign51690_e78226 * locals.var_t4_dn9))) + ((locals.var_t1_dn9 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn9))) * locals.var_t2) + (assign51690_e78233 * locals.var_t2_dn9)), (((((2.0 * locals.var_t5_dn10) + (((((2.0 * locals.var_t10_dn10) * locals.var_t6) + (assign51690_e78224 * locals.var_t6_dn10)) * locals.var_t4) + (assign51690_e78226 * locals.var_t4_dn10))) + ((locals.var_t1_dn10 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn10))) * locals.var_t2) + (assign51690_e78233 * locals.var_t2_dn10)), (((((2.0 * locals.var_t5_dn13) + (((((2.0 * locals.var_t10_dn13) * locals.var_t6) + (assign51690_e78224 * locals.var_t6_dn13)) * locals.var_t4) + (assign51690_e78226 * locals.var_t4_dn13))) + ((locals.var_t1_dn13 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn13))) * locals.var_t2) + (assign51690_e78233 * locals.var_t2_dn13)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn13,)
    }
};
        locals.var_t11 = assign51690_e78237;
        locals.var_t11_dn0 = assign51690_e78237_d_n0;
        locals.var_t11_dn2 = assign51690_e78237_d_n2;
        locals.var_t11_dn4 = assign51690_e78237_d_n4;
        locals.var_t11_dn5 = assign51690_e78237_d_n5;
        locals.var_t11_dn6 = assign51690_e78237_d_n6;
        locals.var_t11_dn7 = assign51690_e78237_d_n7;
        locals.var_t11_dn8 = assign51690_e78237_d_n8;
        locals.var_t11_dn9 = assign51690_e78237_d_n9;
        locals.var_t11_dn10 = assign51690_e78237_d_n10;
        locals.var_t11_dn13 = assign51690_e78237_d_n13;
        locals.var_t11_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_179(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign51700_e78253, assign51700_e78253_d_n0, assign51700_e78253_d_n2, assign51700_e78253_d_n4, assign51700_e78253_d_n5, assign51700_e78253_d_n6, assign51700_e78253_d_n7, assign51700_e78253_d_n8, assign51700_e78253_d_n9, assign51700_e78253_d_n10, assign51700_e78253_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) {
        let assign51700_e78251: f64 = (locals.var_t11 * locals.var_t4);
        (assign51700_e78251, ((locals.var_t11_dn0 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn0)), ((locals.var_t11_dn2 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn2)), ((locals.var_t11_dn4 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn4)), ((locals.var_t11_dn5 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn5)), ((locals.var_t11_dn6 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn6)), ((locals.var_t11_dn7 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn7)), ((locals.var_t11_dn8 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn8)), ((locals.var_t11_dn9 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn9)), ((locals.var_t11_dn10 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn10)), ((locals.var_t11_dn13 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn13)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign51700_e78253;
        locals.var_t7_dn0 = assign51700_e78253_d_n0;
        locals.var_t7_dn2 = assign51700_e78253_d_n2;
        locals.var_t7_dn4 = assign51700_e78253_d_n4;
        locals.var_t7_dn5 = assign51700_e78253_d_n5;
        locals.var_t7_dn6 = assign51700_e78253_d_n6;
        locals.var_t7_dn7 = assign51700_e78253_d_n7;
        locals.var_t7_dn8 = assign51700_e78253_d_n8;
        locals.var_t7_dn9 = assign51700_e78253_d_n9;
        locals.var_t7_dn10 = assign51700_e78253_d_n10;
        locals.var_t7_dn13 = assign51700_e78253_d_n13;
        locals.var_t7_rv = 0.0;

        let (assign51710_e78275, assign51710_e78275_d_n0, assign51710_e78275_d_n2, assign51710_e78275_d_n4, assign51710_e78275_d_n5, assign51710_e78275_d_n6, assign51710_e78275_d_n7, assign51710_e78275_d_n8, assign51710_e78275_d_n9, assign51710_e78275_d_n10, assign51710_e78275_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) {
        let assign51710_e78268: f64 = (2.0 * locals.var_t10);
        let assign51710_e78270: f64 = (assign51710_e78268 * locals.var_t6);
        let assign51710_e78272: f64 = (assign51710_e78270 + locals.var_t1);
        let assign51710_e78273: f64 = (4.0 * assign51710_e78272);
        (assign51710_e78273, (4.0 * ((((2.0 * locals.var_t10_dn0) * locals.var_t6) + (assign51710_e78268 * locals.var_t6_dn0)) + locals.var_t1_dn0)), (4.0 * ((((2.0 * locals.var_t10_dn2) * locals.var_t6) + (assign51710_e78268 * locals.var_t6_dn2)) + locals.var_t1_dn2)), (4.0 * ((((2.0 * locals.var_t10_dn4) * locals.var_t6) + (assign51710_e78268 * locals.var_t6_dn4)) + locals.var_t1_dn4)), (4.0 * ((((2.0 * locals.var_t10_dn5) * locals.var_t6) + (assign51710_e78268 * locals.var_t6_dn5)) + locals.var_t1_dn5)), (4.0 * ((((2.0 * locals.var_t10_dn6) * locals.var_t6) + (assign51710_e78268 * locals.var_t6_dn6)) + locals.var_t1_dn6)), (4.0 * ((((2.0 * locals.var_t10_dn7) * locals.var_t6) + (assign51710_e78268 * locals.var_t6_dn7)) + locals.var_t1_dn7)), (4.0 * ((((2.0 * locals.var_t10_dn8) * locals.var_t6) + (assign51710_e78268 * locals.var_t6_dn8)) + locals.var_t1_dn8)), (4.0 * ((((2.0 * locals.var_t10_dn9) * locals.var_t6) + (assign51710_e78268 * locals.var_t6_dn9)) + locals.var_t1_dn9)), (4.0 * ((((2.0 * locals.var_t10_dn10) * locals.var_t6) + (assign51710_e78268 * locals.var_t6_dn10)) + locals.var_t1_dn10)), (4.0 * ((((2.0 * locals.var_t10_dn13) * locals.var_t6) + (assign51710_e78268 * locals.var_t6_dn13)) + locals.var_t1_dn13)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn13,)
    }
};
        locals.var_t11 = assign51710_e78275;
        locals.var_t11_dn0 = assign51710_e78275_d_n0;
        locals.var_t11_dn2 = assign51710_e78275_d_n2;
        locals.var_t11_dn4 = assign51710_e78275_d_n4;
        locals.var_t11_dn5 = assign51710_e78275_d_n5;
        locals.var_t11_dn6 = assign51710_e78275_d_n6;
        locals.var_t11_dn7 = assign51710_e78275_d_n7;
        locals.var_t11_dn8 = assign51710_e78275_d_n8;
        locals.var_t11_dn9 = assign51710_e78275_d_n9;
        locals.var_t11_dn10 = assign51710_e78275_d_n10;
        locals.var_t11_dn13 = assign51710_e78275_d_n13;
        locals.var_t11_rv = 0.0;

        let (assign51720_e78293, assign51720_e78293_d_n0, assign51720_e78293_d_n2, assign51720_e78293_d_n4, assign51720_e78293_d_n5, assign51720_e78293_d_n6, assign51720_e78293_d_n7, assign51720_e78293_d_n8, assign51720_e78293_d_n9, assign51720_e78293_d_n10, assign51720_e78293_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) {
        let assign51720_e78289: f64 = (locals.var_t11 * locals.var_t4);
        let assign51720_e78291: f64 = (assign51720_e78289 * locals.var_t4);
        (assign51720_e78291, ((((locals.var_t11_dn0 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn0)) * locals.var_t4) + (assign51720_e78289 * locals.var_t4_dn0)), ((((locals.var_t11_dn2 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn2)) * locals.var_t4) + (assign51720_e78289 * locals.var_t4_dn2)), ((((locals.var_t11_dn4 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn4)) * locals.var_t4) + (assign51720_e78289 * locals.var_t4_dn4)), ((((locals.var_t11_dn5 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn5)) * locals.var_t4) + (assign51720_e78289 * locals.var_t4_dn5)), ((((locals.var_t11_dn6 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn6)) * locals.var_t4) + (assign51720_e78289 * locals.var_t4_dn6)), ((((locals.var_t11_dn7 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn7)) * locals.var_t4) + (assign51720_e78289 * locals.var_t4_dn7)), ((((locals.var_t11_dn8 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn8)) * locals.var_t4) + (assign51720_e78289 * locals.var_t4_dn8)), ((((locals.var_t11_dn9 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn9)) * locals.var_t4) + (assign51720_e78289 * locals.var_t4_dn9)), ((((locals.var_t11_dn10 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn10)) * locals.var_t4) + (assign51720_e78289 * locals.var_t4_dn10)), ((((locals.var_t11_dn13 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn13)) * locals.var_t4) + (assign51720_e78289 * locals.var_t4_dn13)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn13,)
    }
};
        locals.var_t8 = assign51720_e78293;
        locals.var_t8_dn0 = assign51720_e78293_d_n0;
        locals.var_t8_dn2 = assign51720_e78293_d_n2;
        locals.var_t8_dn4 = assign51720_e78293_d_n4;
        locals.var_t8_dn5 = assign51720_e78293_d_n5;
        locals.var_t8_dn6 = assign51720_e78293_d_n6;
        locals.var_t8_dn7 = assign51720_e78293_d_n7;
        locals.var_t8_dn8 = assign51720_e78293_d_n8;
        locals.var_t8_dn9 = assign51720_e78293_d_n9;
        locals.var_t8_dn10 = assign51720_e78293_d_n10;
        locals.var_t8_dn13 = assign51720_e78293_d_n13;
        locals.var_t8_rv = 0.0;

        let (assign51730_e78312, assign51730_e78312_d_n0, assign51730_e78312_d_n2, assign51730_e78312_d_n4, assign51730_e78312_d_n5, assign51730_e78312_d_n6, assign51730_e78312_d_n7, assign51730_e78312_d_n8, assign51730_e78312_d_n9, assign51730_e78312_d_n10, assign51730_e78312_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) {
        let assign51730_e78307: f64 = (locals.var_t7 * locals.var_t7);
        let assign51730_e78309: f64 = (assign51730_e78307 + locals.var_t8);
        let assign51730_e78310: f64 = (assign51730_e78309).sqrt();
        (assign51730_e78310, ((((locals.var_t7_dn0 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn0)) + locals.var_t8_dn0) / (2.0 * assign51730_e78310)), ((((locals.var_t7_dn2 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn2)) + locals.var_t8_dn2) / (2.0 * assign51730_e78310)), ((((locals.var_t7_dn4 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn4)) + locals.var_t8_dn4) / (2.0 * assign51730_e78310)), ((((locals.var_t7_dn5 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn5)) + locals.var_t8_dn5) / (2.0 * assign51730_e78310)), ((((locals.var_t7_dn6 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn6)) + locals.var_t8_dn6) / (2.0 * assign51730_e78310)), ((((locals.var_t7_dn7 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn7)) + locals.var_t8_dn7) / (2.0 * assign51730_e78310)), ((((locals.var_t7_dn8 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn8)) + locals.var_t8_dn8) / (2.0 * assign51730_e78310)), ((((locals.var_t7_dn9 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn9)) + locals.var_t8_dn9) / (2.0 * assign51730_e78310)), ((((locals.var_t7_dn10 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn10)) + locals.var_t8_dn10) / (2.0 * assign51730_e78310)), ((((locals.var_t7_dn13 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn13)) + locals.var_t8_dn13) / (2.0 * assign51730_e78310)),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign51730_e78312;
        locals.var_t9_dn0 = assign51730_e78312_d_n0;
        locals.var_t9_dn2 = assign51730_e78312_d_n2;
        locals.var_t9_dn4 = assign51730_e78312_d_n4;
        locals.var_t9_dn5 = assign51730_e78312_d_n5;
        locals.var_t9_dn6 = assign51730_e78312_d_n6;
        locals.var_t9_dn7 = assign51730_e78312_d_n7;
        locals.var_t9_dn8 = assign51730_e78312_d_n8;
        locals.var_t9_dn9 = assign51730_e78312_d_n9;
        locals.var_t9_dn10 = assign51730_e78312_d_n10;
        locals.var_t9_dn13 = assign51730_e78312_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign51740_e78331, assign51740_e78331_d_n0, assign51740_e78331_d_n2, assign51740_e78331_d_n4, assign51740_e78331_d_n5, assign51740_e78331_d_n6, assign51740_e78331_d_n7, assign51740_e78331_d_n8, assign51740_e78331_d_n9, assign51740_e78331_d_n10, assign51740_e78331_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) {
        let assign51740_e78326: f64 = (-locals.var_t7);
        let assign51740_e78328: f64 = (assign51740_e78326 + locals.var_t9);
        let assign51740_e78329: f64 = (0.5 * assign51740_e78328);
        (assign51740_e78329, (0.5 * ((-locals.var_t7_dn0) + locals.var_t9_dn0)), (0.5 * ((-locals.var_t7_dn2) + locals.var_t9_dn2)), (0.5 * ((-locals.var_t7_dn4) + locals.var_t9_dn4)), (0.5 * ((-locals.var_t7_dn5) + locals.var_t9_dn5)), (0.5 * ((-locals.var_t7_dn6) + locals.var_t9_dn6)), (0.5 * ((-locals.var_t7_dn7) + locals.var_t9_dn7)), (0.5 * ((-locals.var_t7_dn8) + locals.var_t9_dn8)), (0.5 * ((-locals.var_t7_dn9) + locals.var_t9_dn9)), (0.5 * ((-locals.var_t7_dn10) + locals.var_t9_dn10)), (0.5 * ((-locals.var_t7_dn13) + locals.var_t9_dn13)),)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn4, locals.var_lred_dn5, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn8, locals.var_lred_dn9, locals.var_lred_dn10, locals.var_lred_dn13,)
    }
};
        locals.var_lred = assign51740_e78331;
        locals.var_lred_dn0 = assign51740_e78331_d_n0;
        locals.var_lred_dn2 = assign51740_e78331_d_n2;
        locals.var_lred_dn4 = assign51740_e78331_d_n4;
        locals.var_lred_dn5 = assign51740_e78331_d_n5;
        locals.var_lred_dn6 = assign51740_e78331_d_n6;
        locals.var_lred_dn7 = assign51740_e78331_d_n7;
        locals.var_lred_dn8 = assign51740_e78331_d_n8;
        locals.var_lred_dn9 = assign51740_e78331_d_n9;
        locals.var_lred_dn10 = assign51740_e78331_d_n10;
        locals.var_lred_dn13 = assign51740_e78331_d_n13;
        locals.var_lred_rv = 0.0;

        let (assign51750_e78345, assign51750_e78345_d_n0, assign51750_e78345_d_n2, assign51750_e78345_d_n4, assign51750_e78345_d_n5, assign51750_e78345_d_n6, assign51750_e78345_d_n7, assign51750_e78345_d_n8, assign51750_e78345_d_n9, assign51750_e78345_d_n10, assign51750_e78345_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn4, locals.var_lred_dn5, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn8, locals.var_lred_dn9, locals.var_lred_dn10, locals.var_lred_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign51750_e78345;
        locals.var_t1_dn0 = assign51750_e78345_d_n0;
        locals.var_t1_dn2 = assign51750_e78345_d_n2;
        locals.var_t1_dn4 = assign51750_e78345_d_n4;
        locals.var_t1_dn5 = assign51750_e78345_d_n5;
        locals.var_t1_dn6 = assign51750_e78345_d_n6;
        locals.var_t1_dn7 = assign51750_e78345_d_n7;
        locals.var_t1_dn8 = assign51750_e78345_d_n8;
        locals.var_t1_dn9 = assign51750_e78345_d_n9;
        locals.var_t1_dn10 = assign51750_e78345_d_n10;
        locals.var_t1_dn13 = assign51750_e78345_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign51760_e78361, assign51760_e78361_d_n0, assign51760_e78361_d_n2, assign51760_e78361_d_n4, assign51760_e78361_d_n5, assign51760_e78361_d_n6, assign51760_e78361_d_n7, assign51760_e78361_d_n8, assign51760_e78361_d_n9, assign51760_e78361_d_n10, assign51760_e78361_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1297 == 0.0)) {
        let assign51760_e78359: f64 = (locals.var_fmdvds * locals.var_t1);
        (assign51760_e78359, ((locals.var_fmdvds_dn0 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn0)), ((locals.var_fmdvds_dn2 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn2)), ((locals.var_fmdvds_dn4 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn4)), ((locals.var_fmdvds_dn5 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn5)), ((locals.var_fmdvds_dn6 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn6)), ((locals.var_fmdvds_dn7 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn7)), ((locals.var_fmdvds_dn8 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn8)), ((locals.var_fmdvds_dn9 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn9)), ((locals.var_fmdvds_dn10 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn10)), ((locals.var_fmdvds_dn13 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn13)),)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn4, locals.var_lred_dn5, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn8, locals.var_lred_dn9, locals.var_lred_dn10, locals.var_lred_dn13,)
    }
};
        locals.var_lred = assign51760_e78361;
        locals.var_lred_dn0 = assign51760_e78361_d_n0;
        locals.var_lred_dn2 = assign51760_e78361_d_n2;
        locals.var_lred_dn4 = assign51760_e78361_d_n4;
        locals.var_lred_dn5 = assign51760_e78361_d_n5;
        locals.var_lred_dn6 = assign51760_e78361_d_n6;
        locals.var_lred_dn7 = assign51760_e78361_d_n7;
        locals.var_lred_dn8 = assign51760_e78361_d_n8;
        locals.var_lred_dn9 = assign51760_e78361_d_n9;
        locals.var_lred_dn10 = assign51760_e78361_d_n10;
        locals.var_lred_dn13 = assign51760_e78361_d_n13;
        locals.var_lred_rv = 0.0;

        let (assign51770_e78374, assign51770_e78374_d_n0, assign51770_e78374_d_n2, assign51770_e78374_d_n4, assign51770_e78374_d_n5, assign51770_e78374_d_n6, assign51770_e78374_d_n7, assign51770_e78374_d_n8, assign51770_e78374_d_n9, assign51770_e78374_d_n10, assign51770_e78374_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign51770_e78372: f64 = (locals.var_lred * locals.var_clmmod);
        (assign51770_e78372, (locals.var_lred_dn0 * locals.var_clmmod), (locals.var_lred_dn2 * locals.var_clmmod), (locals.var_lred_dn4 * locals.var_clmmod), (locals.var_lred_dn5 * locals.var_clmmod), (locals.var_lred_dn6 * locals.var_clmmod), (locals.var_lred_dn7 * locals.var_clmmod), (locals.var_lred_dn8 * locals.var_clmmod), (locals.var_lred_dn9 * locals.var_clmmod), (locals.var_lred_dn10 * locals.var_clmmod), (locals.var_lred_dn13 * locals.var_clmmod),)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn4, locals.var_lred_dn5, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn8, locals.var_lred_dn9, locals.var_lred_dn10, locals.var_lred_dn13,)
    }
};
        locals.var_lred = assign51770_e78374;
        locals.var_lred_dn0 = assign51770_e78374_d_n0;
        locals.var_lred_dn2 = assign51770_e78374_d_n2;
        locals.var_lred_dn4 = assign51770_e78374_d_n4;
        locals.var_lred_dn5 = assign51770_e78374_d_n5;
        locals.var_lred_dn6 = assign51770_e78374_d_n6;
        locals.var_lred_dn7 = assign51770_e78374_d_n7;
        locals.var_lred_dn8 = assign51770_e78374_d_n8;
        locals.var_lred_dn9 = assign51770_e78374_d_n9;
        locals.var_lred_dn10 = assign51770_e78374_d_n10;
        locals.var_lred_dn13 = assign51770_e78374_d_n13;
        locals.var_lred_rv = 0.0;

        let (assign51780_e78387, assign51780_e78387_d_n0, assign51780_e78387_d_n2, assign51780_e78387_d_n4, assign51780_e78387_d_n5, assign51780_e78387_d_n6, assign51780_e78387_d_n7, assign51780_e78387_d_n8, assign51780_e78387_d_n9, assign51780_e78387_d_n10, assign51780_e78387_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign51780_e78385: f64 = (locals.var_lch - locals.var_lred);
        (assign51780_e78385, (locals.var_lch_dn0 - locals.var_lred_dn0), (locals.var_lch_dn2 - locals.var_lred_dn2), (locals.var_lch_dn4 - locals.var_lred_dn4), (locals.var_lch_dn5 - locals.var_lred_dn5), (locals.var_lch_dn6 - locals.var_lred_dn6), (locals.var_lch_dn7 - locals.var_lred_dn7), (locals.var_lch_dn8 - locals.var_lred_dn8), (locals.var_lch_dn9 - locals.var_lred_dn9), (locals.var_lch_dn10 - locals.var_lred_dn10), (locals.var_lch_dn13 - locals.var_lred_dn13),)
    } else {
        (locals.var_lch, locals.var_lch_dn0, locals.var_lch_dn2, locals.var_lch_dn4, locals.var_lch_dn5, locals.var_lch_dn6, locals.var_lch_dn7, locals.var_lch_dn8, locals.var_lch_dn9, locals.var_lch_dn10, locals.var_lch_dn13,)
    }
};
        locals.var_lch = assign51780_e78387;
        locals.var_lch_dn0 = assign51780_e78387_d_n0;
        locals.var_lch_dn2 = assign51780_e78387_d_n2;
        locals.var_lch_dn4 = assign51780_e78387_d_n4;
        locals.var_lch_dn5 = assign51780_e78387_d_n5;
        locals.var_lch_dn6 = assign51780_e78387_d_n6;
        locals.var_lch_dn7 = assign51780_e78387_d_n7;
        locals.var_lch_dn8 = assign51780_e78387_d_n8;
        locals.var_lch_dn9 = assign51780_e78387_d_n9;
        locals.var_lch_dn10 = assign51780_e78387_d_n10;
        locals.var_lch_dn13 = assign51780_e78387_d_n13;
        locals.var_lch_rv = 0.0;

        let (assign51790_e78400, assign51790_e78400_d_n0, assign51790_e78400_d_n2, assign51790_e78400_d_n4, assign51790_e78400_d_n5, assign51790_e78400_d_n6, assign51790_e78400_d_n7, assign51790_e78400_d_n8, assign51790_e78400_d_n9, assign51790_e78400_d_n10, assign51790_e78400_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign51790_e78398: f64 = (locals.var_ninv_o_esi / 100.0);
        (assign51790_e78398, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign51790_e78400;
        locals.var_t2_dn0 = assign51790_e78400_d_n0;
        locals.var_t2_dn2 = assign51790_e78400_d_n2;
        locals.var_t2_dn4 = assign51790_e78400_d_n4;
        locals.var_t2_dn5 = assign51790_e78400_d_n5;
        locals.var_t2_dn6 = assign51790_e78400_d_n6;
        locals.var_t2_dn7 = assign51790_e78400_d_n7;
        locals.var_t2_dn8 = assign51790_e78400_d_n8;
        locals.var_t2_dn9 = assign51790_e78400_d_n9;
        locals.var_t2_dn10 = assign51790_e78400_d_n10;
        locals.var_t2_dn13 = assign51790_e78400_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign51800_e78411, assign51800_e78411_d_n0, assign51800_e78411_d_n2, assign51800_e78411_d_n4, assign51800_e78411_d_n5, assign51800_e78411_d_n6, assign51800_e78411_d_n7, assign51800_e78411_d_n8, assign51800_e78411_d_n9, assign51800_e78411_d_n10, assign51800_e78411_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        (locals.var_ninvde, locals.var_ninvde_dn0, locals.var_ninvde_dn2, locals.var_ninvde_dn4, locals.var_ninvde_dn5, locals.var_ninvde_dn6, locals.var_ninvde_dn7, locals.var_ninvde_dn8, locals.var_ninvde_dn9, locals.var_ninvde_dn10, locals.var_ninvde_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign51800_e78411;
        locals.var_t0_dn0 = assign51800_e78411_d_n0;
        locals.var_t0_dn2 = assign51800_e78411_d_n2;
        locals.var_t0_dn4 = assign51800_e78411_d_n4;
        locals.var_t0_dn5 = assign51800_e78411_d_n5;
        locals.var_t0_dn6 = assign51800_e78411_d_n6;
        locals.var_t0_dn7 = assign51800_e78411_d_n7;
        locals.var_t0_dn8 = assign51800_e78411_d_n8;
        locals.var_t0_dn9 = assign51800_e78411_d_n9;
        locals.var_t0_dn10 = assign51800_e78411_d_n10;
        locals.var_t0_dn13 = assign51800_e78411_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign51810_e78430, assign51810_e78430_d_n0, assign51810_e78430_d_n2, assign51810_e78430_d_n4, assign51810_e78430_d_n5, assign51810_e78430_d_n6, assign51810_e78430_d_n7, assign51810_e78430_d_n8, assign51810_e78430_d_n9, assign51810_e78430_d_n10, assign51810_e78430_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign51810_e78422: f64 = (locals.var_pds * locals.var_pds);
        let assign51810_e78424: f64 = (assign51810_e78422 + p.p262);
        let assign51810_e78425: f64 = (assign51810_e78424).sqrt();
        let assign51810_e78427: f64 = (p.p262).sqrt();
        let assign51810_e78428: f64 = (assign51810_e78425 - assign51810_e78427);
        (assign51810_e78428, (((locals.var_pds_dn0 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn0)) / (2.0 * assign51810_e78425)), (((locals.var_pds_dn2 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn2)) / (2.0 * assign51810_e78425)), (((locals.var_pds_dn4 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn4)) / (2.0 * assign51810_e78425)), (((locals.var_pds_dn5 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn5)) / (2.0 * assign51810_e78425)), (((locals.var_pds_dn6 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn6)) / (2.0 * assign51810_e78425)), (((locals.var_pds_dn7 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn7)) / (2.0 * assign51810_e78425)), (((locals.var_pds_dn8 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn8)) / (2.0 * assign51810_e78425)), (((locals.var_pds_dn9 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn9)) / (2.0 * assign51810_e78425)), (((locals.var_pds_dn10 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn10)) / (2.0 * assign51810_e78425)), (((locals.var_pds_dn13 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn13)) / (2.0 * assign51810_e78425)),)
    } else {
        (locals.var_pdsz, locals.var_pdsz_dn0, locals.var_pdsz_dn2, locals.var_pdsz_dn4, locals.var_pdsz_dn5, locals.var_pdsz_dn6, locals.var_pdsz_dn7, locals.var_pdsz_dn8, locals.var_pdsz_dn9, locals.var_pdsz_dn10, locals.var_pdsz_dn13,)
    }
};
        locals.var_pdsz = assign51810_e78430;
        locals.var_pdsz_dn0 = assign51810_e78430_d_n0;
        locals.var_pdsz_dn2 = assign51810_e78430_d_n2;
        locals.var_pdsz_dn4 = assign51810_e78430_d_n4;
        locals.var_pdsz_dn5 = assign51810_e78430_d_n5;
        locals.var_pdsz_dn6 = assign51810_e78430_d_n6;
        locals.var_pdsz_dn7 = assign51810_e78430_d_n7;
        locals.var_pdsz_dn8 = assign51810_e78430_d_n8;
        locals.var_pdsz_dn9 = assign51810_e78430_d_n9;
        locals.var_pdsz_dn10 = assign51810_e78430_d_n10;
        locals.var_pdsz_dn13 = assign51810_e78430_d_n13;
        locals.var_pdsz_rv = 0.0;

        let (assign51820_e78445, assign51820_e78445_d_n0, assign51820_e78445_d_n2, assign51820_e78445_d_n4, assign51820_e78445_d_n5, assign51820_e78445_d_n6, assign51820_e78445_d_n7, assign51820_e78445_d_n8, assign51820_e78445_d_n9, assign51820_e78445_d_n10, assign51820_e78445_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign51820_e78442: f64 = (locals.var_pdsz * locals.var_t0);
        let assign51820_e78443: f64 = (1.0 + assign51820_e78442);
        (assign51820_e78443, ((locals.var_pdsz_dn0 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn0)), ((locals.var_pdsz_dn2 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn2)), ((locals.var_pdsz_dn4 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn4)), ((locals.var_pdsz_dn5 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn5)), ((locals.var_pdsz_dn6 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn6)), ((locals.var_pdsz_dn7 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn7)), ((locals.var_pdsz_dn8 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn8)), ((locals.var_pdsz_dn9 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn9)), ((locals.var_pdsz_dn10 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn10)), ((locals.var_pdsz_dn13 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign51820_e78445;
        locals.var_t4_dn0 = assign51820_e78445_d_n0;
        locals.var_t4_dn2 = assign51820_e78445_d_n2;
        locals.var_t4_dn4 = assign51820_e78445_d_n4;
        locals.var_t4_dn5 = assign51820_e78445_d_n5;
        locals.var_t4_dn6 = assign51820_e78445_d_n6;
        locals.var_t4_dn7 = assign51820_e78445_d_n7;
        locals.var_t4_dn8 = assign51820_e78445_d_n8;
        locals.var_t4_dn9 = assign51820_e78445_d_n9;
        locals.var_t4_dn10 = assign51820_e78445_d_n10;
        locals.var_t4_dn13 = assign51820_e78445_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign51830_e78458, assign51830_e78458_d_n0, assign51830_e78458_d_n2, assign51830_e78458_d_n4, assign51830_e78458_d_n5, assign51830_e78458_d_n6, assign51830_e78458_d_n7, assign51830_e78458_d_n8, assign51830_e78458_d_n9, assign51830_e78458_d_n10, assign51830_e78458_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign51830_e78456: f64 = (locals.var_t2 * locals.var_qn0);
        (assign51830_e78456, ((locals.var_t2_dn0 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn0)), ((locals.var_t2_dn2 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn2)), ((locals.var_t2_dn4 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn4)), ((locals.var_t2_dn5 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn5)), ((locals.var_t2_dn6 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn6)), ((locals.var_t2_dn7 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn7)), ((locals.var_t2_dn8 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn8)), ((locals.var_t2_dn9 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn9)), ((locals.var_t2_dn10 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn10)), ((locals.var_t2_dn13 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn13)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign51830_e78458;
        locals.var_t5_dn0 = assign51830_e78458_d_n0;
        locals.var_t5_dn2 = assign51830_e78458_d_n2;
        locals.var_t5_dn4 = assign51830_e78458_d_n4;
        locals.var_t5_dn5 = assign51830_e78458_d_n5;
        locals.var_t5_dn6 = assign51830_e78458_d_n6;
        locals.var_t5_dn7 = assign51830_e78458_d_n7;
        locals.var_t5_dn8 = assign51830_e78458_d_n8;
        locals.var_t5_dn9 = assign51830_e78458_d_n9;
        locals.var_t5_dn10 = assign51830_e78458_d_n10;
        locals.var_t5_dn13 = assign51830_e78458_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign51840_e78471, assign51840_e78471_d_n0, assign51840_e78471_d_n2, assign51840_e78471_d_n4, assign51840_e78471_d_n5, assign51840_e78471_d_n6, assign51840_e78471_d_n7, assign51840_e78471_d_n8, assign51840_e78471_d_n9, assign51840_e78471_d_n10, assign51840_e78471_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign51840_e78469: f64 = (locals.var_t5 / locals.var_t4);
        (assign51840_e78469, (((locals.var_t5_dn0 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn2 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn4 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn5 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn6 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn7 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn8 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn9 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn10 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn13 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn13)) / (locals.var_t4 * locals.var_t4)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign51840_e78471;
        locals.var_t3_dn0 = assign51840_e78471_d_n0;
        locals.var_t3_dn2 = assign51840_e78471_d_n2;
        locals.var_t3_dn4 = assign51840_e78471_d_n4;
        locals.var_t3_dn5 = assign51840_e78471_d_n5;
        locals.var_t3_dn6 = assign51840_e78471_d_n6;
        locals.var_t3_dn7 = assign51840_e78471_d_n7;
        locals.var_t3_dn8 = assign51840_e78471_d_n8;
        locals.var_t3_dn9 = assign51840_e78471_d_n9;
        locals.var_t3_dn10 = assign51840_e78471_d_n10;
        locals.var_t3_dn13 = assign51840_e78471_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign51850_e78482, assign51850_e78482_d_n0, assign51850_e78482_d_n2, assign51850_e78482_d_n4, assign51850_e78482_d_n5, assign51850_e78482_d_n6, assign51850_e78482_d_n7, assign51850_e78482_d_n8, assign51850_e78482_d_n9, assign51850_e78482_d_n10, assign51850_e78482_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    } else {
        (locals.var_eeff, locals.var_eeff_dn0, locals.var_eeff_dn2, locals.var_eeff_dn4, locals.var_eeff_dn5, locals.var_eeff_dn6, locals.var_eeff_dn7, locals.var_eeff_dn8, locals.var_eeff_dn9, locals.var_eeff_dn10, locals.var_eeff_dn13,)
    }
};
        locals.var_eeff = assign51850_e78482;
        locals.var_eeff_dn0 = assign51850_e78482_d_n0;
        locals.var_eeff_dn2 = assign51850_e78482_d_n2;
        locals.var_eeff_dn4 = assign51850_e78482_d_n4;
        locals.var_eeff_dn5 = assign51850_e78482_d_n5;
        locals.var_eeff_dn6 = assign51850_e78482_d_n6;
        locals.var_eeff_dn7 = assign51850_e78482_d_n7;
        locals.var_eeff_dn8 = assign51850_e78482_d_n8;
        locals.var_eeff_dn9 = assign51850_e78482_d_n9;
        locals.var_eeff_dn10 = assign51850_e78482_d_n10;
        locals.var_eeff_dn13 = assign51850_e78482_d_n13;
        locals.var_eeff_rv = 0.0;

        let (assign51860_e78500, assign51860_e78500_d_n0, assign51860_e78500_d_n2, assign51860_e78500_d_n4, assign51860_e78500_d_n5, assign51860_e78500_d_n6, assign51860_e78500_d_n7, assign51860_e78500_d_n8, assign51860_e78500_d_n9, assign51860_e78500_d_n10, assign51860_e78500_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let (assign51860_e78498, assign51860_e78498_d_n0, assign51860_e78498_d_n2, assign51860_e78498_d_n4, assign51860_e78498_d_n5, assign51860_e78498_d_n6, assign51860_e78498_d_n7, assign51860_e78498_d_n8, assign51860_e78498_d_n9, assign51860_e78498_d_n10, assign51860_e78498_d_n13,) = {
            if (locals.var_eeff == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign51860_e78497: f64 = (locals.var_eeff).powf(p.p160);
                (assign51860_e78497, if 0.0 == 0.0 && ((p.p160) as f64).is_finite() && ((p.p160) as f64).fract() == 0.0 { if p.p160 == 0.0 { 0.0 } else { (p.p160 * ((locals.var_eeff).powf(p.p160 - 1.0) * locals.var_eeff_dn0)) } } else { (assign51860_e78497 * (p.p160 * (locals.var_eeff_dn0 / locals.var_eeff))) }, if 0.0 == 0.0 && ((p.p160) as f64).is_finite() && ((p.p160) as f64).fract() == 0.0 { if p.p160 == 0.0 { 0.0 } else { (p.p160 * ((locals.var_eeff).powf(p.p160 - 1.0) * locals.var_eeff_dn2)) } } else { (assign51860_e78497 * (p.p160 * (locals.var_eeff_dn2 / locals.var_eeff))) }, if 0.0 == 0.0 && ((p.p160) as f64).is_finite() && ((p.p160) as f64).fract() == 0.0 { if p.p160 == 0.0 { 0.0 } else { (p.p160 * ((locals.var_eeff).powf(p.p160 - 1.0) * locals.var_eeff_dn4)) } } else { (assign51860_e78497 * (p.p160 * (locals.var_eeff_dn4 / locals.var_eeff))) }, if 0.0 == 0.0 && ((p.p160) as f64).is_finite() && ((p.p160) as f64).fract() == 0.0 { if p.p160 == 0.0 { 0.0 } else { (p.p160 * ((locals.var_eeff).powf(p.p160 - 1.0) * locals.var_eeff_dn5)) } } else { (assign51860_e78497 * (p.p160 * (locals.var_eeff_dn5 / locals.var_eeff))) }, if 0.0 == 0.0 && ((p.p160) as f64).is_finite() && ((p.p160) as f64).fract() == 0.0 { if p.p160 == 0.0 { 0.0 } else { (p.p160 * ((locals.var_eeff).powf(p.p160 - 1.0) * locals.var_eeff_dn6)) } } else { (assign51860_e78497 * (p.p160 * (locals.var_eeff_dn6 / locals.var_eeff))) }, if 0.0 == 0.0 && ((p.p160) as f64).is_finite() && ((p.p160) as f64).fract() == 0.0 { if p.p160 == 0.0 { 0.0 } else { (p.p160 * ((locals.var_eeff).powf(p.p160 - 1.0) * locals.var_eeff_dn7)) } } else { (assign51860_e78497 * (p.p160 * (locals.var_eeff_dn7 / locals.var_eeff))) }, if 0.0 == 0.0 && ((p.p160) as f64).is_finite() && ((p.p160) as f64).fract() == 0.0 { if p.p160 == 0.0 { 0.0 } else { (p.p160 * ((locals.var_eeff).powf(p.p160 - 1.0) * locals.var_eeff_dn8)) } } else { (assign51860_e78497 * (p.p160 * (locals.var_eeff_dn8 / locals.var_eeff))) }, if 0.0 == 0.0 && ((p.p160) as f64).is_finite() && ((p.p160) as f64).fract() == 0.0 { if p.p160 == 0.0 { 0.0 } else { (p.p160 * ((locals.var_eeff).powf(p.p160 - 1.0) * locals.var_eeff_dn9)) } } else { (assign51860_e78497 * (p.p160 * (locals.var_eeff_dn9 / locals.var_eeff))) }, if 0.0 == 0.0 && ((p.p160) as f64).is_finite() && ((p.p160) as f64).fract() == 0.0 { if p.p160 == 0.0 { 0.0 } else { (p.p160 * ((locals.var_eeff).powf(p.p160 - 1.0) * locals.var_eeff_dn10)) } } else { (assign51860_e78497 * (p.p160 * (locals.var_eeff_dn10 / locals.var_eeff))) }, if 0.0 == 0.0 && ((p.p160) as f64).is_finite() && ((p.p160) as f64).fract() == 0.0 { if p.p160 == 0.0 { 0.0 } else { (p.p160 * ((locals.var_eeff).powf(p.p160 - 1.0) * locals.var_eeff_dn13)) } } else { (assign51860_e78497 * (p.p160 * (locals.var_eeff_dn13 / locals.var_eeff))) },)
            }
        };
        (assign51860_e78498, assign51860_e78498_d_n0, assign51860_e78498_d_n2, assign51860_e78498_d_n4, assign51860_e78498_d_n5, assign51860_e78498_d_n6, assign51860_e78498_d_n7, assign51860_e78498_d_n8, assign51860_e78498_d_n9, assign51860_e78498_d_n10, assign51860_e78498_d_n13,)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn13,)
    }
};
        locals.var_t8 = assign51860_e78500;
        locals.var_t8_dn0 = assign51860_e78500_d_n0;
        locals.var_t8_dn2 = assign51860_e78500_d_n2;
        locals.var_t8_dn4 = assign51860_e78500_d_n4;
        locals.var_t8_dn5 = assign51860_e78500_d_n5;
        locals.var_t8_dn6 = assign51860_e78500_d_n6;
        locals.var_t8_dn7 = assign51860_e78500_d_n7;
        locals.var_t8_dn8 = assign51860_e78500_d_n8;
        locals.var_t8_dn9 = assign51860_e78500_d_n9;
        locals.var_t8_dn10 = assign51860_e78500_d_n10;
        locals.var_t8_dn13 = assign51860_e78500_d_n13;
        locals.var_t8_rv = 0.0;

        let (assign51870_e78518, assign51870_e78518_d_n0, assign51870_e78518_d_n2, assign51870_e78518_d_n4, assign51870_e78518_d_n5, assign51870_e78518_d_n6, assign51870_e78518_d_n7, assign51870_e78518_d_n8, assign51870_e78518_d_n9, assign51870_e78518_d_n10, assign51870_e78518_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let (assign51870_e78516, assign51870_e78516_d_n0, assign51870_e78516_d_n2, assign51870_e78516_d_n4, assign51870_e78516_d_n5, assign51870_e78516_d_n6, assign51870_e78516_d_n7, assign51870_e78516_d_n8, assign51870_e78516_d_n9, assign51870_e78516_d_n10, assign51870_e78516_d_n13,) = {
            if (locals.var_eeff == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign51870_e78515: f64 = (locals.var_eeff).powf(locals.var_muesr);
                (assign51870_e78515, if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn0)) } } else { (assign51870_e78515 * (locals.var_muesr * (locals.var_eeff_dn0 / locals.var_eeff))) }, if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn2)) } } else { (assign51870_e78515 * (locals.var_muesr * (locals.var_eeff_dn2 / locals.var_eeff))) }, if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn4)) } } else { (assign51870_e78515 * (locals.var_muesr * (locals.var_eeff_dn4 / locals.var_eeff))) }, if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn5)) } } else { (assign51870_e78515 * (locals.var_muesr * (locals.var_eeff_dn5 / locals.var_eeff))) }, if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn6)) } } else { (assign51870_e78515 * (locals.var_muesr * (locals.var_eeff_dn6 / locals.var_eeff))) }, if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn7)) } } else { (assign51870_e78515 * (locals.var_muesr * (locals.var_eeff_dn7 / locals.var_eeff))) }, if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn8)) } } else { (assign51870_e78515 * (locals.var_muesr * (locals.var_eeff_dn8 / locals.var_eeff))) }, if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn9)) } } else { (assign51870_e78515 * (locals.var_muesr * (locals.var_eeff_dn9 / locals.var_eeff))) }, if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn10)) } } else { (assign51870_e78515 * (locals.var_muesr * (locals.var_eeff_dn10 / locals.var_eeff))) }, if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn13)) } } else { (assign51870_e78515 * (locals.var_muesr * (locals.var_eeff_dn13 / locals.var_eeff))) },)
            }
        };
        (assign51870_e78516, assign51870_e78516_d_n0, assign51870_e78516_d_n2, assign51870_e78516_d_n4, assign51870_e78516_d_n5, assign51870_e78516_d_n6, assign51870_e78516_d_n7, assign51870_e78516_d_n8, assign51870_e78516_d_n9, assign51870_e78516_d_n10, assign51870_e78516_d_n13,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign51870_e78518;
        locals.var_t6_dn0 = assign51870_e78518_d_n0;
        locals.var_t6_dn2 = assign51870_e78518_d_n2;
        locals.var_t6_dn4 = assign51870_e78518_d_n4;
        locals.var_t6_dn5 = assign51870_e78518_d_n5;
        locals.var_t6_dn6 = assign51870_e78518_d_n6;
        locals.var_t6_dn7 = assign51870_e78518_d_n7;
        locals.var_t6_dn8 = assign51870_e78518_d_n8;
        locals.var_t6_dn9 = assign51870_e78518_d_n9;
        locals.var_t6_dn10 = assign51870_e78518_d_n10;
        locals.var_t6_dn13 = assign51870_e78518_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign51880_e78531, assign51880_e78531_d_n0, assign51880_e78531_d_n2, assign51880_e78531_d_n4, assign51880_e78531_d_n5, assign51880_e78531_d_n6, assign51880_e78531_d_n7, assign51880_e78531_d_n8, assign51880_e78531_d_n9, assign51880_e78531_d_n10, assign51880_e78531_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign51880_e78529: f64 = (1.6021918e-19 * 10000.0);
        (assign51880_e78529, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign51880_e78531;
        locals.var_t9_dn0 = assign51880_e78531_d_n0;
        locals.var_t9_dn2 = assign51880_e78531_d_n2;
        locals.var_t9_dn4 = assign51880_e78531_d_n4;
        locals.var_t9_dn5 = assign51880_e78531_d_n5;
        locals.var_t9_dn6 = assign51880_e78531_d_n6;
        locals.var_t9_dn7 = assign51880_e78531_d_n7;
        locals.var_t9_dn8 = assign51880_e78531_d_n8;
        locals.var_t9_dn9 = assign51880_e78531_d_n9;
        locals.var_t9_dn10 = assign51880_e78531_d_n10;
        locals.var_t9_dn13 = assign51880_e78531_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign51890_e78544, assign51890_e78544_d_n0, assign51890_e78544_d_n2, assign51890_e78544_d_n4, assign51890_e78544_d_n5, assign51890_e78544_d_n6, assign51890_e78544_d_n7, assign51890_e78544_d_n8, assign51890_e78544_d_n9, assign51890_e78544_d_n10, assign51890_e78544_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign51890_e78542: f64 = (locals.var_qn0 / locals.var_t9);
        (assign51890_e78542, (((locals.var_qn0_dn0 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn0)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn2 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn2)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn4 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn4)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn5 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn5)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn6 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn6)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn7 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn7)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn8 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn8)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn9 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn9)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn10 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn10)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn13 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn13)) / (locals.var_t9 * locals.var_t9)),)
    } else {
        (locals.var_rns, locals.var_rns_dn0, locals.var_rns_dn2, locals.var_rns_dn4, locals.var_rns_dn5, locals.var_rns_dn6, locals.var_rns_dn7, locals.var_rns_dn8, locals.var_rns_dn9, locals.var_rns_dn10, locals.var_rns_dn13,)
    }
};
        locals.var_rns = assign51890_e78544;
        locals.var_rns_dn0 = assign51890_e78544_d_n0;
        locals.var_rns_dn2 = assign51890_e78544_d_n2;
        locals.var_rns_dn4 = assign51890_e78544_d_n4;
        locals.var_rns_dn5 = assign51890_e78544_d_n5;
        locals.var_rns_dn6 = assign51890_e78544_d_n6;
        locals.var_rns_dn7 = assign51890_e78544_d_n7;
        locals.var_rns_dn8 = assign51890_e78544_d_n8;
        locals.var_rns_dn9 = assign51890_e78544_d_n9;
        locals.var_rns_dn10 = assign51890_e78544_d_n10;
        locals.var_rns_dn13 = assign51890_e78544_d_n13;
        locals.var_rns_rv = 0.0;

        let (assign51900_e78555, assign51900_e78555_d_n0, assign51900_e78555_d_n2, assign51900_e78555_d_n4, assign51900_e78555_d_n5, assign51900_e78555_d_n6, assign51900_e78555_d_n7, assign51900_e78555_d_n8, assign51900_e78555_d_n9, assign51900_e78555_d_n10, assign51900_e78555_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        (locals.var_uc_muecb0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign51900_e78555;
        locals.var_t2_dn0 = assign51900_e78555_d_n0;
        locals.var_t2_dn2 = assign51900_e78555_d_n2;
        locals.var_t2_dn4 = assign51900_e78555_d_n4;
        locals.var_t2_dn5 = assign51900_e78555_d_n5;
        locals.var_t2_dn6 = assign51900_e78555_d_n6;
        locals.var_t2_dn7 = assign51900_e78555_d_n7;
        locals.var_t2_dn8 = assign51900_e78555_d_n8;
        locals.var_t2_dn9 = assign51900_e78555_d_n9;
        locals.var_t2_dn10 = assign51900_e78555_d_n10;
        locals.var_t2_dn13 = assign51900_e78555_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign51910_e78590, assign51910_e78590_d_n0, assign51910_e78590_d_n2, assign51910_e78590_d_n4, assign51910_e78590_d_n5, assign51910_e78590_d_n6, assign51910_e78590_d_n7, assign51910_e78590_d_n8, assign51910_e78590_d_n9, assign51910_e78590_d_n10, assign51910_e78590_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign51910_e78566: f64 = 1.0;
        let assign51910_e78570: f64 = (locals.var_uc_muecb1 * locals.var_t4);
        let assign51910_e78572: f64 = (assign51910_e78570 * locals.var_rns);
        let assign51910_e78574: f64 = (assign51910_e78572 / 100000000000.0);
        let assign51910_e78575: f64 = (locals.var_t2 + assign51910_e78574);
        let assign51910_e78576: f64 = (assign51910_e78566 / assign51910_e78575);
        let assign51910_e78579: f64 = locals.var_mphn0;
        let assign51910_e78581: f64 = (assign51910_e78579 * locals.var_t8);
        let assign51910_e78582: f64 = (assign51910_e78576 + assign51910_e78581);
        let assign51910_e78585: f64 = locals.var_t6;
        let assign51910_e78587: f64 = (assign51910_e78585 / locals.var_uc_muesr1);
        let assign51910_e78588: f64 = (assign51910_e78582 + assign51910_e78587);
        (assign51910_e78588, (((-((assign51910_e78566 * (locals.var_t2_dn0 + ((((locals.var_uc_muecb1 * locals.var_t4_dn0) * locals.var_rns) + (assign51910_e78570 * locals.var_rns_dn0)) / 100000000000.0))) / (assign51910_e78575 * assign51910_e78575))) + ((locals.var_mphn0_dn0 * locals.var_t8) + (assign51910_e78579 * locals.var_t8_dn0))) + (locals.var_t6_dn0 / locals.var_uc_muesr1)), (((-((assign51910_e78566 * (locals.var_t2_dn2 + ((((locals.var_uc_muecb1 * locals.var_t4_dn2) * locals.var_rns) + (assign51910_e78570 * locals.var_rns_dn2)) / 100000000000.0))) / (assign51910_e78575 * assign51910_e78575))) + ((locals.var_mphn0_dn2 * locals.var_t8) + (assign51910_e78579 * locals.var_t8_dn2))) + (locals.var_t6_dn2 / locals.var_uc_muesr1)), (((-((assign51910_e78566 * (locals.var_t2_dn4 + ((((locals.var_uc_muecb1 * locals.var_t4_dn4) * locals.var_rns) + (assign51910_e78570 * locals.var_rns_dn4)) / 100000000000.0))) / (assign51910_e78575 * assign51910_e78575))) + ((locals.var_mphn0_dn4 * locals.var_t8) + (assign51910_e78579 * locals.var_t8_dn4))) + (locals.var_t6_dn4 / locals.var_uc_muesr1)), (((-((assign51910_e78566 * (locals.var_t2_dn5 + ((((locals.var_uc_muecb1 * locals.var_t4_dn5) * locals.var_rns) + (assign51910_e78570 * locals.var_rns_dn5)) / 100000000000.0))) / (assign51910_e78575 * assign51910_e78575))) + ((locals.var_mphn0_dn5 * locals.var_t8) + (assign51910_e78579 * locals.var_t8_dn5))) + (locals.var_t6_dn5 / locals.var_uc_muesr1)), (((-((assign51910_e78566 * (locals.var_t2_dn6 + ((((locals.var_uc_muecb1 * locals.var_t4_dn6) * locals.var_rns) + (assign51910_e78570 * locals.var_rns_dn6)) / 100000000000.0))) / (assign51910_e78575 * assign51910_e78575))) + ((locals.var_mphn0_dn6 * locals.var_t8) + (assign51910_e78579 * locals.var_t8_dn6))) + (locals.var_t6_dn6 / locals.var_uc_muesr1)), (((-((assign51910_e78566 * (locals.var_t2_dn7 + ((((locals.var_uc_muecb1 * locals.var_t4_dn7) * locals.var_rns) + (assign51910_e78570 * locals.var_rns_dn7)) / 100000000000.0))) / (assign51910_e78575 * assign51910_e78575))) + ((locals.var_mphn0_dn7 * locals.var_t8) + (assign51910_e78579 * locals.var_t8_dn7))) + (locals.var_t6_dn7 / locals.var_uc_muesr1)), (((-((assign51910_e78566 * (locals.var_t2_dn8 + ((((locals.var_uc_muecb1 * locals.var_t4_dn8) * locals.var_rns) + (assign51910_e78570 * locals.var_rns_dn8)) / 100000000000.0))) / (assign51910_e78575 * assign51910_e78575))) + ((locals.var_mphn0_dn8 * locals.var_t8) + (assign51910_e78579 * locals.var_t8_dn8))) + (locals.var_t6_dn8 / locals.var_uc_muesr1)), (((-((assign51910_e78566 * (locals.var_t2_dn9 + ((((locals.var_uc_muecb1 * locals.var_t4_dn9) * locals.var_rns) + (assign51910_e78570 * locals.var_rns_dn9)) / 100000000000.0))) / (assign51910_e78575 * assign51910_e78575))) + ((locals.var_mphn0_dn9 * locals.var_t8) + (assign51910_e78579 * locals.var_t8_dn9))) + (locals.var_t6_dn9 / locals.var_uc_muesr1)), (((-((assign51910_e78566 * (locals.var_t2_dn10 + ((((locals.var_uc_muecb1 * locals.var_t4_dn10) * locals.var_rns) + (assign51910_e78570 * locals.var_rns_dn10)) / 100000000000.0))) / (assign51910_e78575 * assign51910_e78575))) + ((locals.var_mphn0_dn10 * locals.var_t8) + (assign51910_e78579 * locals.var_t8_dn10))) + (locals.var_t6_dn10 / locals.var_uc_muesr1)), (((-((assign51910_e78566 * (locals.var_t2_dn13 + ((((locals.var_uc_muecb1 * locals.var_t4_dn13) * locals.var_rns) + (assign51910_e78570 * locals.var_rns_dn13)) / 100000000000.0))) / (assign51910_e78575 * assign51910_e78575))) + ((locals.var_mphn0_dn13 * locals.var_t8) + (assign51910_e78579 * locals.var_t8_dn13))) + (locals.var_t6_dn13 / locals.var_uc_muesr1)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign51910_e78590;
        locals.var_t1_dn0 = assign51910_e78590_d_n0;
        locals.var_t1_dn2 = assign51910_e78590_d_n2;
        locals.var_t1_dn4 = assign51910_e78590_d_n4;
        locals.var_t1_dn5 = assign51910_e78590_d_n5;
        locals.var_t1_dn6 = assign51910_e78590_d_n6;
        locals.var_t1_dn7 = assign51910_e78590_d_n7;
        locals.var_t1_dn8 = assign51910_e78590_d_n8;
        locals.var_t1_dn9 = assign51910_e78590_d_n9;
        locals.var_t1_dn10 = assign51910_e78590_d_n10;
        locals.var_t1_dn13 = assign51910_e78590_d_n13;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_180(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign51920_e78603, assign51920_e78603_d_n0, assign51920_e78603_d_n2, assign51920_e78603_d_n4, assign51920_e78603_d_n5, assign51920_e78603_d_n6, assign51920_e78603_d_n7, assign51920_e78603_d_n8, assign51920_e78603_d_n9, assign51920_e78603_d_n10, assign51920_e78603_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign51920_e78601: f64 = (1.0 / locals.var_t1);
        (assign51920_e78601, (-(locals.var_t1_dn0 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn2 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn4 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn5 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn6 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn7 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn8 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn9 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn10 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn13 / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn13,)
    }
};
        locals.var_muun = assign51920_e78603;
        locals.var_muun_dn0 = assign51920_e78603_d_n0;
        locals.var_muun_dn2 = assign51920_e78603_d_n2;
        locals.var_muun_dn4 = assign51920_e78603_d_n4;
        locals.var_muun_dn5 = assign51920_e78603_d_n5;
        locals.var_muun_dn6 = assign51920_e78603_d_n6;
        locals.var_muun_dn7 = assign51920_e78603_d_n7;
        locals.var_muun_dn8 = assign51920_e78603_d_n8;
        locals.var_muun_dn9 = assign51920_e78603_d_n9;
        locals.var_muun_dn10 = assign51920_e78603_d_n10;
        locals.var_muun_dn13 = assign51920_e78603_d_n13;
        locals.var_muun_rv = 0.0;

        let (assign51930_e78616, assign51930_e78616_d_n0, assign51930_e78616_d_n2, assign51930_e78616_d_n4, assign51930_e78616_d_n5, assign51930_e78616_d_n6, assign51930_e78616_d_n7, assign51930_e78616_d_n8, assign51930_e78616_d_n9, assign51930_e78616_d_n10, assign51930_e78616_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign51930_e78614: f64 = (locals.var_muun / 10000.0);
        (assign51930_e78614, (locals.var_muun_dn0 / 10000.0), (locals.var_muun_dn2 / 10000.0), (locals.var_muun_dn4 / 10000.0), (locals.var_muun_dn5 / 10000.0), (locals.var_muun_dn6 / 10000.0), (locals.var_muun_dn7 / 10000.0), (locals.var_muun_dn8 / 10000.0), (locals.var_muun_dn9 / 10000.0), (locals.var_muun_dn10 / 10000.0), (locals.var_muun_dn13 / 10000.0),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn13,)
    }
};
        locals.var_muun = assign51930_e78616;
        locals.var_muun_dn0 = assign51930_e78616_d_n0;
        locals.var_muun_dn2 = assign51930_e78616_d_n2;
        locals.var_muun_dn4 = assign51930_e78616_d_n4;
        locals.var_muun_dn5 = assign51930_e78616_d_n5;
        locals.var_muun_dn6 = assign51930_e78616_d_n6;
        locals.var_muun_dn7 = assign51930_e78616_d_n7;
        locals.var_muun_dn8 = assign51930_e78616_d_n8;
        locals.var_muun_dn9 = assign51930_e78616_d_n9;
        locals.var_muun_dn10 = assign51930_e78616_d_n10;
        locals.var_muun_dn13 = assign51930_e78616_d_n13;
        locals.var_muun_rv = 0.0;

        let (assign51940_e78633, assign51940_e78633_d_n0, assign51940_e78633_d_n2, assign51940_e78633_d_n4, assign51940_e78633_d_n5, assign51940_e78633_d_n6, assign51940_e78633_d_n7, assign51940_e78633_d_n8, assign51940_e78633_d_n9, assign51940_e78633_d_n10, assign51940_e78633_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign51940_e78628: f64 = (locals.var_qn0 + 1e-25);
        let assign51940_e78629: f64 = (locals.var_beta * assign51940_e78628);
        let assign51940_e78631: f64 = (assign51940_e78629 * locals.var_lch);
        (assign51940_e78631, ((((locals.var_beta_dn0 * assign51940_e78628) + (locals.var_beta * locals.var_qn0_dn0)) * locals.var_lch) + (assign51940_e78629 * locals.var_lch_dn0)), ((((locals.var_beta_dn2 * assign51940_e78628) + (locals.var_beta * locals.var_qn0_dn2)) * locals.var_lch) + (assign51940_e78629 * locals.var_lch_dn2)), ((((locals.var_beta_dn4 * assign51940_e78628) + (locals.var_beta * locals.var_qn0_dn4)) * locals.var_lch) + (assign51940_e78629 * locals.var_lch_dn4)), ((((locals.var_beta_dn5 * assign51940_e78628) + (locals.var_beta * locals.var_qn0_dn5)) * locals.var_lch) + (assign51940_e78629 * locals.var_lch_dn5)), ((((locals.var_beta_dn6 * assign51940_e78628) + (locals.var_beta * locals.var_qn0_dn6)) * locals.var_lch) + (assign51940_e78629 * locals.var_lch_dn6)), ((((locals.var_beta_dn7 * assign51940_e78628) + (locals.var_beta * locals.var_qn0_dn7)) * locals.var_lch) + (assign51940_e78629 * locals.var_lch_dn7)), ((((locals.var_beta_dn8 * assign51940_e78628) + (locals.var_beta * locals.var_qn0_dn8)) * locals.var_lch) + (assign51940_e78629 * locals.var_lch_dn8)), ((((locals.var_beta_dn9 * assign51940_e78628) + (locals.var_beta * locals.var_qn0_dn9)) * locals.var_lch) + (assign51940_e78629 * locals.var_lch_dn9)), ((((locals.var_beta_dn10 * assign51940_e78628) + (locals.var_beta * locals.var_qn0_dn10)) * locals.var_lch) + (assign51940_e78629 * locals.var_lch_dn10)), ((((locals.var_beta_dn13 * assign51940_e78628) + (locals.var_beta * locals.var_qn0_dn13)) * locals.var_lch) + (assign51940_e78629 * locals.var_lch_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign51940_e78633;
        locals.var_t2_dn0 = assign51940_e78633_d_n0;
        locals.var_t2_dn2 = assign51940_e78633_d_n2;
        locals.var_t2_dn4 = assign51940_e78633_d_n4;
        locals.var_t2_dn5 = assign51940_e78633_d_n5;
        locals.var_t2_dn6 = assign51940_e78633_d_n6;
        locals.var_t2_dn7 = assign51940_e78633_d_n7;
        locals.var_t2_dn8 = assign51940_e78633_d_n8;
        locals.var_t2_dn9 = assign51940_e78633_d_n9;
        locals.var_t2_dn10 = assign51940_e78633_d_n10;
        locals.var_t2_dn13 = assign51940_e78633_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign51950_e78646, assign51950_e78646_d_n0, assign51950_e78646_d_n2, assign51950_e78646_d_n4, assign51950_e78646_d_n5, assign51950_e78646_d_n6, assign51950_e78646_d_n7, assign51950_e78646_d_n8, assign51950_e78646_d_n9, assign51950_e78646_d_n10, assign51950_e78646_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign51950_e78644: f64 = (1.0 / locals.var_t2);
        (assign51950_e78644, (-(locals.var_t2_dn0 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn2 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn9 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn10 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn13 / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign51950_e78646;
        locals.var_t1_dn0 = assign51950_e78646_d_n0;
        locals.var_t1_dn2 = assign51950_e78646_d_n2;
        locals.var_t1_dn4 = assign51950_e78646_d_n4;
        locals.var_t1_dn5 = assign51950_e78646_d_n5;
        locals.var_t1_dn6 = assign51950_e78646_d_n6;
        locals.var_t1_dn7 = assign51950_e78646_d_n7;
        locals.var_t1_dn8 = assign51950_e78646_d_n8;
        locals.var_t1_dn9 = assign51950_e78646_d_n9;
        locals.var_t1_dn10 = assign51950_e78646_d_n10;
        locals.var_t1_dn13 = assign51950_e78646_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign51960_e78659, assign51960_e78659_d_n0, assign51960_e78659_d_n2, assign51960_e78659_d_n4, assign51960_e78659_d_n5, assign51960_e78659_d_n6, assign51960_e78659_d_n7, assign51960_e78659_d_n8, assign51960_e78659_d_n9, assign51960_e78659_d_n10, assign51960_e78659_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign51960_e78657: f64 = (locals.var_idd * locals.var_t1);
        (assign51960_e78657, ((locals.var_idd_dn0 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn0)), ((locals.var_idd_dn2 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn2)), ((locals.var_idd_dn4 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn4)), ((locals.var_idd_dn5 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn5)), ((locals.var_idd_dn6 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn6)), ((locals.var_idd_dn7 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn7)), ((locals.var_idd_dn8 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn8)), ((locals.var_idd_dn9 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn9)), ((locals.var_idd_dn10 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn10)), ((locals.var_idd_dn13 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn13)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign51960_e78659;
        locals.var_ty_dn0 = assign51960_e78659_d_n0;
        locals.var_ty_dn2 = assign51960_e78659_d_n2;
        locals.var_ty_dn4 = assign51960_e78659_d_n4;
        locals.var_ty_dn5 = assign51960_e78659_d_n5;
        locals.var_ty_dn6 = assign51960_e78659_d_n6;
        locals.var_ty_dn7 = assign51960_e78659_d_n7;
        locals.var_ty_dn8 = assign51960_e78659_d_n8;
        locals.var_ty_dn9 = assign51960_e78659_d_n9;
        locals.var_ty_dn10 = assign51960_e78659_d_n10;
        locals.var_ty_dn13 = assign51960_e78659_d_n13;
        locals.var_ty_rv = 0.0;

        let (assign51970_e78674, assign51970_e78674_d_n0, assign51970_e78674_d_n2, assign51970_e78674_d_n4, assign51970_e78674_d_n5, assign51970_e78674_d_n6, assign51970_e78674_d_n7, assign51970_e78674_d_n8, assign51970_e78674_d_n9, assign51970_e78674_d_n10, assign51970_e78674_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign51970_e78670: f64 = (0.2 * locals.var_vmaxe);
        let assign51970_e78672: f64 = (assign51970_e78670 / locals.var_muun);
        (assign51970_e78672, ((((0.2 * locals.var_vmaxe_dn0) * locals.var_muun) - (assign51970_e78670 * locals.var_muun_dn0)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn2) * locals.var_muun) - (assign51970_e78670 * locals.var_muun_dn2)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn4) * locals.var_muun) - (assign51970_e78670 * locals.var_muun_dn4)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn5) * locals.var_muun) - (assign51970_e78670 * locals.var_muun_dn5)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn6) * locals.var_muun) - (assign51970_e78670 * locals.var_muun_dn6)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn7) * locals.var_muun) - (assign51970_e78670 * locals.var_muun_dn7)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn8) * locals.var_muun) - (assign51970_e78670 * locals.var_muun_dn8)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn9) * locals.var_muun) - (assign51970_e78670 * locals.var_muun_dn9)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn10) * locals.var_muun) - (assign51970_e78670 * locals.var_muun_dn10)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn13) * locals.var_muun) - (assign51970_e78670 * locals.var_muun_dn13)) / (locals.var_muun * locals.var_muun)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign51970_e78674;
        locals.var_t2_dn0 = assign51970_e78674_d_n0;
        locals.var_t2_dn2 = assign51970_e78674_d_n2;
        locals.var_t2_dn4 = assign51970_e78674_d_n4;
        locals.var_t2_dn5 = assign51970_e78674_d_n5;
        locals.var_t2_dn6 = assign51970_e78674_d_n6;
        locals.var_t2_dn7 = assign51970_e78674_d_n7;
        locals.var_t2_dn8 = assign51970_e78674_d_n8;
        locals.var_t2_dn9 = assign51970_e78674_d_n9;
        locals.var_t2_dn10 = assign51970_e78674_d_n10;
        locals.var_t2_dn13 = assign51970_e78674_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign51980_e78692, assign51980_e78692_d_n0, assign51980_e78692_d_n2, assign51980_e78692_d_n4, assign51980_e78692_d_n5, assign51980_e78692_d_n6, assign51980_e78692_d_n7, assign51980_e78692_d_n8, assign51980_e78692_d_n9, assign51980_e78692_d_n10, assign51980_e78692_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign51980_e78685: f64 = (locals.var_ty * locals.var_ty);
        let assign51980_e78688: f64 = (locals.var_t2 * locals.var_t2);
        let assign51980_e78689: f64 = (assign51980_e78685 + assign51980_e78688);
        let assign51980_e78690: f64 = (assign51980_e78689).sqrt();
        (assign51980_e78690, ((((locals.var_ty_dn0 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn0)) + ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0))) / (2.0 * assign51980_e78690)), ((((locals.var_ty_dn2 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn2)) + ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2))) / (2.0 * assign51980_e78690)), ((((locals.var_ty_dn4 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn4)) + ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4))) / (2.0 * assign51980_e78690)), ((((locals.var_ty_dn5 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn5)) + ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5))) / (2.0 * assign51980_e78690)), ((((locals.var_ty_dn6 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn6)) + ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6))) / (2.0 * assign51980_e78690)), ((((locals.var_ty_dn7 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn7)) + ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7))) / (2.0 * assign51980_e78690)), ((((locals.var_ty_dn8 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn8)) + ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8))) / (2.0 * assign51980_e78690)), ((((locals.var_ty_dn9 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn9)) + ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9))) / (2.0 * assign51980_e78690)), ((((locals.var_ty_dn10 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn10)) + ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10))) / (2.0 * assign51980_e78690)), ((((locals.var_ty_dn13 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn13)) + ((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13))) / (2.0 * assign51980_e78690)),)
    } else {
        (locals.var_ey, locals.var_ey_dn0, locals.var_ey_dn2, locals.var_ey_dn4, locals.var_ey_dn5, locals.var_ey_dn6, locals.var_ey_dn7, locals.var_ey_dn8, locals.var_ey_dn9, locals.var_ey_dn10, locals.var_ey_dn13,)
    }
};
        locals.var_ey = assign51980_e78692;
        locals.var_ey_dn0 = assign51980_e78692_d_n0;
        locals.var_ey_dn2 = assign51980_e78692_d_n2;
        locals.var_ey_dn4 = assign51980_e78692_d_n4;
        locals.var_ey_dn5 = assign51980_e78692_d_n5;
        locals.var_ey_dn6 = assign51980_e78692_d_n6;
        locals.var_ey_dn7 = assign51980_e78692_d_n7;
        locals.var_ey_dn8 = assign51980_e78692_d_n8;
        locals.var_ey_dn9 = assign51980_e78692_d_n9;
        locals.var_ey_dn10 = assign51980_e78692_d_n10;
        locals.var_ey_dn13 = assign51980_e78692_d_n13;
        locals.var_ey_rv = 0.0;

        let (assign51990_e78705, assign51990_e78705_d_n0, assign51990_e78705_d_n2, assign51990_e78705_d_n4, assign51990_e78705_d_n5, assign51990_e78705_d_n6, assign51990_e78705_d_n7, assign51990_e78705_d_n8, assign51990_e78705_d_n9, assign51990_e78705_d_n10, assign51990_e78705_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign51990_e78703: f64 = (1.0 / locals.var_ey);
        (assign51990_e78703, (-(locals.var_ey_dn0 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn2 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn4 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn5 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn6 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn7 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn8 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn9 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn10 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn13 / (locals.var_ey * locals.var_ey))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign51990_e78705;
        locals.var_t4_dn0 = assign51990_e78705_d_n0;
        locals.var_t4_dn2 = assign51990_e78705_d_n2;
        locals.var_t4_dn4 = assign51990_e78705_d_n4;
        locals.var_t4_dn5 = assign51990_e78705_d_n5;
        locals.var_t4_dn6 = assign51990_e78705_d_n6;
        locals.var_t4_dn7 = assign51990_e78705_d_n7;
        locals.var_t4_dn8 = assign51990_e78705_d_n8;
        locals.var_t4_dn9 = assign51990_e78705_d_n9;
        locals.var_t4_dn10 = assign51990_e78705_d_n10;
        locals.var_t4_dn13 = assign51990_e78705_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign52000_e78718, assign52000_e78718_d_n0, assign52000_e78718_d_n2, assign52000_e78718_d_n4, assign52000_e78718_d_n5, assign52000_e78718_d_n6, assign52000_e78718_d_n7, assign52000_e78718_d_n8, assign52000_e78718_d_n9, assign52000_e78718_d_n10, assign52000_e78718_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign52000_e78716: f64 = (locals.var_muun * locals.var_ey);
        (assign52000_e78716, ((locals.var_muun_dn0 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn0)), ((locals.var_muun_dn2 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn2)), ((locals.var_muun_dn4 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn4)), ((locals.var_muun_dn5 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn5)), ((locals.var_muun_dn6 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn6)), ((locals.var_muun_dn7 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn7)), ((locals.var_muun_dn8 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn8)), ((locals.var_muun_dn9 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn9)), ((locals.var_muun_dn10 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn10)), ((locals.var_muun_dn13 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn13)),)
    } else {
        (locals.var_em, locals.var_em_dn0, locals.var_em_dn2, locals.var_em_dn4, locals.var_em_dn5, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn8, locals.var_em_dn9, locals.var_em_dn10, locals.var_em_dn13,)
    }
};
        locals.var_em = assign52000_e78718;
        locals.var_em_dn0 = assign52000_e78718_d_n0;
        locals.var_em_dn2 = assign52000_e78718_d_n2;
        locals.var_em_dn4 = assign52000_e78718_d_n4;
        locals.var_em_dn5 = assign52000_e78718_d_n5;
        locals.var_em_dn6 = assign52000_e78718_d_n6;
        locals.var_em_dn7 = assign52000_e78718_d_n7;
        locals.var_em_dn8 = assign52000_e78718_d_n8;
        locals.var_em_dn9 = assign52000_e78718_d_n9;
        locals.var_em_dn10 = assign52000_e78718_d_n10;
        locals.var_em_dn13 = assign52000_e78718_d_n13;
        locals.var_em_rv = 0.0;

        let (assign52010_e78731, assign52010_e78731_d_n0, assign52010_e78731_d_n2, assign52010_e78731_d_n4, assign52010_e78731_d_n5, assign52010_e78731_d_n6, assign52010_e78731_d_n7, assign52010_e78731_d_n8, assign52010_e78731_d_n9, assign52010_e78731_d_n10, assign52010_e78731_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign52010_e78729: f64 = (locals.var_em / locals.var_vmaxe);
        (assign52010_e78729, (((locals.var_em_dn0 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn0)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn2 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn2)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn4 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn4)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn5 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn5)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn6 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn6)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn7 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn7)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn8 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn8)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn9 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn9)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn10 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn10)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn13 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn13)) / (locals.var_vmaxe * locals.var_vmaxe)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign52010_e78731;
        locals.var_t1_dn0 = assign52010_e78731_d_n0;
        locals.var_t1_dn2 = assign52010_e78731_d_n2;
        locals.var_t1_dn4 = assign52010_e78731_d_n4;
        locals.var_t1_dn5 = assign52010_e78731_d_n5;
        locals.var_t1_dn6 = assign52010_e78731_d_n6;
        locals.var_t1_dn7 = assign52010_e78731_d_n7;
        locals.var_t1_dn8 = assign52010_e78731_d_n8;
        locals.var_t1_dn9 = assign52010_e78731_d_n9;
        locals.var_t1_dn10 = assign52010_e78731_d_n10;
        locals.var_t1_dn13 = assign52010_e78731_d_n13;
        locals.var_t1_rv = 0.0;

        let assign52020_e78735: f64 = (10.0 * 2.220446049250313e-16);
        let assign52020_e78736: f64 = (1.0 - assign52020_e78735);
        let assign52020_e78743: f64 = (10.0 * 2.220446049250313e-16);
        let assign52020_e78744: f64 = (1.0 + assign52020_e78743);
        let assign52020_e78746: f64 = if ((assign52020_e78736 <= p.p178) && (p.p178 <= assign52020_e78744)) { 1.0 } else { 0.0 };
        locals.var_guard1316 = assign52020_e78746;
        locals.var_guard1316_rv = 0.0;

        let (assign52030_e78759, assign52030_e78759_d_n0, assign52030_e78759_d_n2, assign52030_e78759_d_n4, assign52030_e78759_d_n5, assign52030_e78759_d_n6, assign52030_e78759_d_n7, assign52030_e78759_d_n8, assign52030_e78759_d_n9, assign52030_e78759_d_n10, assign52030_e78759_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1316 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign52030_e78759;
        locals.var_t2_dn0 = assign52030_e78759_d_n0;
        locals.var_t2_dn2 = assign52030_e78759_d_n2;
        locals.var_t2_dn4 = assign52030_e78759_d_n4;
        locals.var_t2_dn5 = assign52030_e78759_d_n5;
        locals.var_t2_dn6 = assign52030_e78759_d_n6;
        locals.var_t2_dn7 = assign52030_e78759_d_n7;
        locals.var_t2_dn8 = assign52030_e78759_d_n8;
        locals.var_t2_dn9 = assign52030_e78759_d_n9;
        locals.var_t2_dn10 = assign52030_e78759_d_n10;
        locals.var_t2_dn13 = assign52030_e78759_d_n13;
        locals.var_t2_rv = 0.0;

        let assign52040_e78763: f64 = (10.0 * 2.220446049250313e-16);
        let assign52040_e78764: f64 = (2.0 - assign52040_e78763);
        let assign52040_e78771: f64 = (10.0 * 2.220446049250313e-16);
        let assign52040_e78772: f64 = (2.0 + assign52040_e78771);
        let assign52040_e78774: f64 = if ((assign52040_e78764 <= p.p178) && (p.p178 <= assign52040_e78772)) { 1.0 } else { 0.0 };
        locals.var_guard1317 = assign52040_e78774;
        locals.var_guard1317_rv = 0.0;

        let (assign52050_e78792, assign52050_e78792_d_n0, assign52050_e78792_d_n2, assign52050_e78792_d_n4, assign52050_e78792_d_n5, assign52050_e78792_d_n6, assign52050_e78792_d_n7, assign52050_e78792_d_n8, assign52050_e78792_d_n9, assign52050_e78792_d_n10, assign52050_e78792_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1316 == 0.0)) && (locals.var_guard1317 != 0.0)) {
        let assign52050_e78790: f64 = (locals.var_t1 * locals.var_t1);
        (assign52050_e78790, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign52050_e78792;
        locals.var_t2_dn0 = assign52050_e78792_d_n0;
        locals.var_t2_dn2 = assign52050_e78792_d_n2;
        locals.var_t2_dn4 = assign52050_e78792_d_n4;
        locals.var_t2_dn5 = assign52050_e78792_d_n5;
        locals.var_t2_dn6 = assign52050_e78792_d_n6;
        locals.var_t2_dn7 = assign52050_e78792_d_n7;
        locals.var_t2_dn8 = assign52050_e78792_d_n8;
        locals.var_t2_dn9 = assign52050_e78792_d_n9;
        locals.var_t2_dn10 = assign52050_e78792_d_n10;
        locals.var_t2_dn13 = assign52050_e78792_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign52060_e78816, assign52060_e78816_d_n0, assign52060_e78816_d_n2, assign52060_e78816_d_n4, assign52060_e78816_d_n5, assign52060_e78816_d_n6, assign52060_e78816_d_n7, assign52060_e78816_d_n8, assign52060_e78816_d_n9, assign52060_e78816_d_n10, assign52060_e78816_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1316 == 0.0)) && (locals.var_guard1317 == 0.0)) {
        let (assign52060_e78814, assign52060_e78814_d_n0, assign52060_e78814_d_n2, assign52060_e78814_d_n4, assign52060_e78814_d_n5, assign52060_e78814_d_n6, assign52060_e78814_d_n7, assign52060_e78814_d_n8, assign52060_e78814_d_n9, assign52060_e78814_d_n10, assign52060_e78814_d_n13,) = {
            if (locals.var_t1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign52060_e78813: f64 = (locals.var_t1).powf(p.p178);
                (assign52060_e78813, if 0.0 == 0.0 && ((p.p178) as f64).is_finite() && ((p.p178) as f64).fract() == 0.0 { if p.p178 == 0.0 { 0.0 } else { (p.p178 * ((locals.var_t1).powf(p.p178 - 1.0) * locals.var_t1_dn0)) } } else { (assign52060_e78813 * (p.p178 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p178) as f64).is_finite() && ((p.p178) as f64).fract() == 0.0 { if p.p178 == 0.0 { 0.0 } else { (p.p178 * ((locals.var_t1).powf(p.p178 - 1.0) * locals.var_t1_dn2)) } } else { (assign52060_e78813 * (p.p178 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p178) as f64).is_finite() && ((p.p178) as f64).fract() == 0.0 { if p.p178 == 0.0 { 0.0 } else { (p.p178 * ((locals.var_t1).powf(p.p178 - 1.0) * locals.var_t1_dn4)) } } else { (assign52060_e78813 * (p.p178 * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p178) as f64).is_finite() && ((p.p178) as f64).fract() == 0.0 { if p.p178 == 0.0 { 0.0 } else { (p.p178 * ((locals.var_t1).powf(p.p178 - 1.0) * locals.var_t1_dn5)) } } else { (assign52060_e78813 * (p.p178 * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p178) as f64).is_finite() && ((p.p178) as f64).fract() == 0.0 { if p.p178 == 0.0 { 0.0 } else { (p.p178 * ((locals.var_t1).powf(p.p178 - 1.0) * locals.var_t1_dn6)) } } else { (assign52060_e78813 * (p.p178 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p178) as f64).is_finite() && ((p.p178) as f64).fract() == 0.0 { if p.p178 == 0.0 { 0.0 } else { (p.p178 * ((locals.var_t1).powf(p.p178 - 1.0) * locals.var_t1_dn7)) } } else { (assign52060_e78813 * (p.p178 * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p178) as f64).is_finite() && ((p.p178) as f64).fract() == 0.0 { if p.p178 == 0.0 { 0.0 } else { (p.p178 * ((locals.var_t1).powf(p.p178 - 1.0) * locals.var_t1_dn8)) } } else { (assign52060_e78813 * (p.p178 * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p178) as f64).is_finite() && ((p.p178) as f64).fract() == 0.0 { if p.p178 == 0.0 { 0.0 } else { (p.p178 * ((locals.var_t1).powf(p.p178 - 1.0) * locals.var_t1_dn9)) } } else { (assign52060_e78813 * (p.p178 * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p178) as f64).is_finite() && ((p.p178) as f64).fract() == 0.0 { if p.p178 == 0.0 { 0.0 } else { (p.p178 * ((locals.var_t1).powf(p.p178 - 1.0) * locals.var_t1_dn10)) } } else { (assign52060_e78813 * (p.p178 * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p178) as f64).is_finite() && ((p.p178) as f64).fract() == 0.0 { if p.p178 == 0.0 { 0.0 } else { (p.p178 * ((locals.var_t1).powf(p.p178 - 1.0) * locals.var_t1_dn13)) } } else { (assign52060_e78813 * (p.p178 * (locals.var_t1_dn13 / locals.var_t1))) },)
            }
        };
        (assign52060_e78814, assign52060_e78814_d_n0, assign52060_e78814_d_n2, assign52060_e78814_d_n4, assign52060_e78814_d_n5, assign52060_e78814_d_n6, assign52060_e78814_d_n7, assign52060_e78814_d_n8, assign52060_e78814_d_n9, assign52060_e78814_d_n10, assign52060_e78814_d_n13,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign52060_e78816;
        locals.var_t2_dn0 = assign52060_e78816_d_n0;
        locals.var_t2_dn2 = assign52060_e78816_d_n2;
        locals.var_t2_dn4 = assign52060_e78816_d_n4;
        locals.var_t2_dn5 = assign52060_e78816_d_n5;
        locals.var_t2_dn6 = assign52060_e78816_d_n6;
        locals.var_t2_dn7 = assign52060_e78816_d_n7;
        locals.var_t2_dn8 = assign52060_e78816_d_n8;
        locals.var_t2_dn9 = assign52060_e78816_d_n9;
        locals.var_t2_dn10 = assign52060_e78816_d_n10;
        locals.var_t2_dn13 = assign52060_e78816_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign52070_e78829, assign52070_e78829_d_n0, assign52070_e78829_d_n2, assign52070_e78829_d_n4, assign52070_e78829_d_n5, assign52070_e78829_d_n6, assign52070_e78829_d_n7, assign52070_e78829_d_n8, assign52070_e78829_d_n9, assign52070_e78829_d_n10, assign52070_e78829_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign52070_e78827: f64 = (1.0 + locals.var_t2);
        (assign52070_e78827, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign52070_e78829;
        locals.var_t4_dn0 = assign52070_e78829_d_n0;
        locals.var_t4_dn2 = assign52070_e78829_d_n2;
        locals.var_t4_dn4 = assign52070_e78829_d_n4;
        locals.var_t4_dn5 = assign52070_e78829_d_n5;
        locals.var_t4_dn6 = assign52070_e78829_d_n6;
        locals.var_t4_dn7 = assign52070_e78829_d_n7;
        locals.var_t4_dn8 = assign52070_e78829_d_n8;
        locals.var_t4_dn9 = assign52070_e78829_d_n9;
        locals.var_t4_dn10 = assign52070_e78829_d_n10;
        locals.var_t4_dn13 = assign52070_e78829_d_n13;
        locals.var_t4_rv = 0.0;

        let assign52080_e78833: f64 = (10.0 * 2.220446049250313e-16);
        let assign52080_e78834: f64 = (1.0 - assign52080_e78833);
        let assign52080_e78841: f64 = (10.0 * 2.220446049250313e-16);
        let assign52080_e78842: f64 = (1.0 + assign52080_e78841);
        let assign52080_e78844: f64 = if ((assign52080_e78834 <= p.p178) && (p.p178 <= assign52080_e78842)) { 1.0 } else { 0.0 };
        locals.var_guard1318 = assign52080_e78844;
        locals.var_guard1318_rv = 0.0;

        let (assign52090_e78859, assign52090_e78859_d_n0, assign52090_e78859_d_n2, assign52090_e78859_d_n4, assign52090_e78859_d_n5, assign52090_e78859_d_n6, assign52090_e78859_d_n7, assign52090_e78859_d_n8, assign52090_e78859_d_n9, assign52090_e78859_d_n10, assign52090_e78859_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1318 != 0.0)) {
        let assign52090_e78857: f64 = (1.0 / locals.var_t4);
        (assign52090_e78857, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn13 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign52090_e78859;
        locals.var_t5_dn0 = assign52090_e78859_d_n0;
        locals.var_t5_dn2 = assign52090_e78859_d_n2;
        locals.var_t5_dn4 = assign52090_e78859_d_n4;
        locals.var_t5_dn5 = assign52090_e78859_d_n5;
        locals.var_t5_dn6 = assign52090_e78859_d_n6;
        locals.var_t5_dn7 = assign52090_e78859_d_n7;
        locals.var_t5_dn8 = assign52090_e78859_d_n8;
        locals.var_t5_dn9 = assign52090_e78859_d_n9;
        locals.var_t5_dn10 = assign52090_e78859_d_n10;
        locals.var_t5_dn13 = assign52090_e78859_d_n13;
        locals.var_t5_rv = 0.0;

        let assign52100_e78863: f64 = (10.0 * 2.220446049250313e-16);
        let assign52100_e78864: f64 = (2.0 - assign52100_e78863);
        let assign52100_e78871: f64 = (10.0 * 2.220446049250313e-16);
        let assign52100_e78872: f64 = (2.0 + assign52100_e78871);
        let assign52100_e78874: f64 = if ((assign52100_e78864 <= p.p178) && (p.p178 <= assign52100_e78872)) { 1.0 } else { 0.0 };
        locals.var_guard1319 = assign52100_e78874;
        locals.var_guard1319_rv = 0.0;

        let (assign52110_e78893, assign52110_e78893_d_n0, assign52110_e78893_d_n2, assign52110_e78893_d_n4, assign52110_e78893_d_n5, assign52110_e78893_d_n6, assign52110_e78893_d_n7, assign52110_e78893_d_n8, assign52110_e78893_d_n9, assign52110_e78893_d_n10, assign52110_e78893_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1318 == 0.0)) && (locals.var_guard1319 != 0.0)) {
        let assign52110_e78890: f64 = (locals.var_t4).sqrt();
        let assign52110_e78891: f64 = (1.0 / assign52110_e78890);
        (assign52110_e78891, (-((locals.var_t4_dn0 / (2.0 * assign52110_e78890)) / (assign52110_e78890 * assign52110_e78890))), (-((locals.var_t4_dn2 / (2.0 * assign52110_e78890)) / (assign52110_e78890 * assign52110_e78890))), (-((locals.var_t4_dn4 / (2.0 * assign52110_e78890)) / (assign52110_e78890 * assign52110_e78890))), (-((locals.var_t4_dn5 / (2.0 * assign52110_e78890)) / (assign52110_e78890 * assign52110_e78890))), (-((locals.var_t4_dn6 / (2.0 * assign52110_e78890)) / (assign52110_e78890 * assign52110_e78890))), (-((locals.var_t4_dn7 / (2.0 * assign52110_e78890)) / (assign52110_e78890 * assign52110_e78890))), (-((locals.var_t4_dn8 / (2.0 * assign52110_e78890)) / (assign52110_e78890 * assign52110_e78890))), (-((locals.var_t4_dn9 / (2.0 * assign52110_e78890)) / (assign52110_e78890 * assign52110_e78890))), (-((locals.var_t4_dn10 / (2.0 * assign52110_e78890)) / (assign52110_e78890 * assign52110_e78890))), (-((locals.var_t4_dn13 / (2.0 * assign52110_e78890)) / (assign52110_e78890 * assign52110_e78890))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign52110_e78893;
        locals.var_t5_dn0 = assign52110_e78893_d_n0;
        locals.var_t5_dn2 = assign52110_e78893_d_n2;
        locals.var_t5_dn4 = assign52110_e78893_d_n4;
        locals.var_t5_dn5 = assign52110_e78893_d_n5;
        locals.var_t5_dn6 = assign52110_e78893_d_n6;
        locals.var_t5_dn7 = assign52110_e78893_d_n7;
        locals.var_t5_dn8 = assign52110_e78893_d_n8;
        locals.var_t5_dn9 = assign52110_e78893_d_n9;
        locals.var_t5_dn10 = assign52110_e78893_d_n10;
        locals.var_t5_dn13 = assign52110_e78893_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign52120_e78920, assign52120_e78920_d_n0, assign52120_e78920_d_n2, assign52120_e78920_d_n4, assign52120_e78920_d_n5, assign52120_e78920_d_n6, assign52120_e78920_d_n7, assign52120_e78920_d_n8, assign52120_e78920_d_n9, assign52120_e78920_d_n10, assign52120_e78920_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1318 == 0.0)) && (locals.var_guard1319 == 0.0)) {
        let (assign52120_e78918, assign52120_e78918_d_n0, assign52120_e78918_d_n2, assign52120_e78918_d_n4, assign52120_e78918_d_n5, assign52120_e78918_d_n6, assign52120_e78918_d_n7, assign52120_e78918_d_n8, assign52120_e78918_d_n9, assign52120_e78918_d_n10, assign52120_e78918_d_n13,) = {
            if (locals.var_t4 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign52120_e78914: f64 = (-1.0);
                let assign52120_e78916: f64 = (assign52120_e78914 / p.p178);
                let assign52120_e78917: f64 = (locals.var_t4).powf(assign52120_e78916);
                (assign52120_e78917, if 0.0 == 0.0 && ((assign52120_e78916) as f64).is_finite() && ((assign52120_e78916) as f64).fract() == 0.0 { if assign52120_e78916 == 0.0 { 0.0 } else { (assign52120_e78916 * ((locals.var_t4).powf(assign52120_e78916 - 1.0) * locals.var_t4_dn0)) } } else { (assign52120_e78917 * (assign52120_e78916 * (locals.var_t4_dn0 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign52120_e78916) as f64).is_finite() && ((assign52120_e78916) as f64).fract() == 0.0 { if assign52120_e78916 == 0.0 { 0.0 } else { (assign52120_e78916 * ((locals.var_t4).powf(assign52120_e78916 - 1.0) * locals.var_t4_dn2)) } } else { (assign52120_e78917 * (assign52120_e78916 * (locals.var_t4_dn2 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign52120_e78916) as f64).is_finite() && ((assign52120_e78916) as f64).fract() == 0.0 { if assign52120_e78916 == 0.0 { 0.0 } else { (assign52120_e78916 * ((locals.var_t4).powf(assign52120_e78916 - 1.0) * locals.var_t4_dn4)) } } else { (assign52120_e78917 * (assign52120_e78916 * (locals.var_t4_dn4 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign52120_e78916) as f64).is_finite() && ((assign52120_e78916) as f64).fract() == 0.0 { if assign52120_e78916 == 0.0 { 0.0 } else { (assign52120_e78916 * ((locals.var_t4).powf(assign52120_e78916 - 1.0) * locals.var_t4_dn5)) } } else { (assign52120_e78917 * (assign52120_e78916 * (locals.var_t4_dn5 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign52120_e78916) as f64).is_finite() && ((assign52120_e78916) as f64).fract() == 0.0 { if assign52120_e78916 == 0.0 { 0.0 } else { (assign52120_e78916 * ((locals.var_t4).powf(assign52120_e78916 - 1.0) * locals.var_t4_dn6)) } } else { (assign52120_e78917 * (assign52120_e78916 * (locals.var_t4_dn6 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign52120_e78916) as f64).is_finite() && ((assign52120_e78916) as f64).fract() == 0.0 { if assign52120_e78916 == 0.0 { 0.0 } else { (assign52120_e78916 * ((locals.var_t4).powf(assign52120_e78916 - 1.0) * locals.var_t4_dn7)) } } else { (assign52120_e78917 * (assign52120_e78916 * (locals.var_t4_dn7 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign52120_e78916) as f64).is_finite() && ((assign52120_e78916) as f64).fract() == 0.0 { if assign52120_e78916 == 0.0 { 0.0 } else { (assign52120_e78916 * ((locals.var_t4).powf(assign52120_e78916 - 1.0) * locals.var_t4_dn8)) } } else { (assign52120_e78917 * (assign52120_e78916 * (locals.var_t4_dn8 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign52120_e78916) as f64).is_finite() && ((assign52120_e78916) as f64).fract() == 0.0 { if assign52120_e78916 == 0.0 { 0.0 } else { (assign52120_e78916 * ((locals.var_t4).powf(assign52120_e78916 - 1.0) * locals.var_t4_dn9)) } } else { (assign52120_e78917 * (assign52120_e78916 * (locals.var_t4_dn9 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign52120_e78916) as f64).is_finite() && ((assign52120_e78916) as f64).fract() == 0.0 { if assign52120_e78916 == 0.0 { 0.0 } else { (assign52120_e78916 * ((locals.var_t4).powf(assign52120_e78916 - 1.0) * locals.var_t4_dn10)) } } else { (assign52120_e78917 * (assign52120_e78916 * (locals.var_t4_dn10 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign52120_e78916) as f64).is_finite() && ((assign52120_e78916) as f64).fract() == 0.0 { if assign52120_e78916 == 0.0 { 0.0 } else { (assign52120_e78916 * ((locals.var_t4).powf(assign52120_e78916 - 1.0) * locals.var_t4_dn13)) } } else { (assign52120_e78917 * (assign52120_e78916 * (locals.var_t4_dn13 / locals.var_t4))) },)
            }
        };
        (assign52120_e78918, assign52120_e78918_d_n0, assign52120_e78918_d_n2, assign52120_e78918_d_n4, assign52120_e78918_d_n5, assign52120_e78918_d_n6, assign52120_e78918_d_n7, assign52120_e78918_d_n8, assign52120_e78918_d_n9, assign52120_e78918_d_n10, assign52120_e78918_d_n13,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign52120_e78920;
        locals.var_t5_dn0 = assign52120_e78920_d_n0;
        locals.var_t5_dn2 = assign52120_e78920_d_n2;
        locals.var_t5_dn4 = assign52120_e78920_d_n4;
        locals.var_t5_dn5 = assign52120_e78920_d_n5;
        locals.var_t5_dn6 = assign52120_e78920_d_n6;
        locals.var_t5_dn7 = assign52120_e78920_d_n7;
        locals.var_t5_dn8 = assign52120_e78920_d_n8;
        locals.var_t5_dn9 = assign52120_e78920_d_n9;
        locals.var_t5_dn10 = assign52120_e78920_d_n10;
        locals.var_t5_dn13 = assign52120_e78920_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign52130_e78933, assign52130_e78933_d_n0, assign52130_e78933_d_n2, assign52130_e78933_d_n4, assign52130_e78933_d_n5, assign52130_e78933_d_n6, assign52130_e78933_d_n7, assign52130_e78933_d_n8, assign52130_e78933_d_n9, assign52130_e78933_d_n10, assign52130_e78933_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign52130_e78931: f64 = (locals.var_muun * locals.var_t5);
        (assign52130_e78931, ((locals.var_muun_dn0 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn0)), ((locals.var_muun_dn2 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn2)), ((locals.var_muun_dn4 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn4)), ((locals.var_muun_dn5 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn5)), ((locals.var_muun_dn6 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn6)), ((locals.var_muun_dn7 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn7)), ((locals.var_muun_dn8 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn8)), ((locals.var_muun_dn9 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn9)), ((locals.var_muun_dn10 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn10)), ((locals.var_muun_dn13 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn13)),)
    } else {
        (locals.var_mu, locals.var_mu_dn0, locals.var_mu_dn2, locals.var_mu_dn4, locals.var_mu_dn5, locals.var_mu_dn6, locals.var_mu_dn7, locals.var_mu_dn8, locals.var_mu_dn9, locals.var_mu_dn10, locals.var_mu_dn13,)
    }
};
        locals.var_mu = assign52130_e78933;
        locals.var_mu_dn0 = assign52130_e78933_d_n0;
        locals.var_mu_dn2 = assign52130_e78933_d_n2;
        locals.var_mu_dn4 = assign52130_e78933_d_n4;
        locals.var_mu_dn5 = assign52130_e78933_d_n5;
        locals.var_mu_dn6 = assign52130_e78933_d_n6;
        locals.var_mu_dn7 = assign52130_e78933_d_n7;
        locals.var_mu_dn8 = assign52130_e78933_d_n8;
        locals.var_mu_dn9 = assign52130_e78933_d_n9;
        locals.var_mu_dn10 = assign52130_e78933_d_n10;
        locals.var_mu_dn13 = assign52130_e78933_d_n13;
        locals.var_mu_rv = 0.0;

        let (assign52140_e78944, assign52140_e78944_d_n0, assign52140_e78944_d_n2, assign52140_e78944_d_n4, assign52140_e78944_d_n5, assign52140_e78944_d_n6, assign52140_e78944_d_n7, assign52140_e78944_d_n8, assign52140_e78944_d_n9, assign52140_e78944_d_n10, assign52140_e78944_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        (locals.var_mu, locals.var_mu_dn0, locals.var_mu_dn2, locals.var_mu_dn4, locals.var_mu_dn5, locals.var_mu_dn6, locals.var_mu_dn7, locals.var_mu_dn8, locals.var_mu_dn9, locals.var_mu_dn10, locals.var_mu_dn13,)
    } else {
        (locals.var_mu_acc, locals.var_mu_acc_dn0, locals.var_mu_acc_dn2, locals.var_mu_acc_dn4, locals.var_mu_acc_dn5, locals.var_mu_acc_dn6, locals.var_mu_acc_dn7, locals.var_mu_acc_dn8, locals.var_mu_acc_dn9, locals.var_mu_acc_dn10, locals.var_mu_acc_dn13,)
    }
};
        locals.var_mu_acc = assign52140_e78944;
        locals.var_mu_acc_dn0 = assign52140_e78944_d_n0;
        locals.var_mu_acc_dn2 = assign52140_e78944_d_n2;
        locals.var_mu_acc_dn4 = assign52140_e78944_d_n4;
        locals.var_mu_acc_dn5 = assign52140_e78944_d_n5;
        locals.var_mu_acc_dn6 = assign52140_e78944_d_n6;
        locals.var_mu_acc_dn7 = assign52140_e78944_d_n7;
        locals.var_mu_acc_dn8 = assign52140_e78944_d_n8;
        locals.var_mu_acc_dn9 = assign52140_e78944_d_n9;
        locals.var_mu_acc_dn10 = assign52140_e78944_d_n10;
        locals.var_mu_acc_dn13 = assign52140_e78944_d_n13;
        locals.var_mu_acc_rv = 0.0;

        let (assign52150_e78955, assign52150_e78955_d_n0, assign52150_e78955_d_n2, assign52150_e78955_d_n4, assign52150_e78955_d_n5, assign52150_e78955_d_n6, assign52150_e78955_d_n7, assign52150_e78955_d_n8, assign52150_e78955_d_n9, assign52150_e78955_d_n10, assign52150_e78955_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        (locals.var_ey, locals.var_ey_dn0, locals.var_ey_dn2, locals.var_ey_dn4, locals.var_ey_dn5, locals.var_ey_dn6, locals.var_ey_dn7, locals.var_ey_dn8, locals.var_ey_dn9, locals.var_ey_dn10, locals.var_ey_dn13,)
    } else {
        (locals.var_ey_acc__blk1116, locals.var_ey_acc__blk1116_dn0, locals.var_ey_acc__blk1116_dn2, locals.var_ey_acc__blk1116_dn4, locals.var_ey_acc__blk1116_dn5, locals.var_ey_acc__blk1116_dn6, locals.var_ey_acc__blk1116_dn7, locals.var_ey_acc__blk1116_dn8, locals.var_ey_acc__blk1116_dn9, locals.var_ey_acc__blk1116_dn10, locals.var_ey_acc__blk1116_dn13,)
    }
};
        locals.var_ey_acc__blk1116 = assign52150_e78955;
        locals.var_ey_acc__blk1116_dn0 = assign52150_e78955_d_n0;
        locals.var_ey_acc__blk1116_dn2 = assign52150_e78955_d_n2;
        locals.var_ey_acc__blk1116_dn4 = assign52150_e78955_d_n4;
        locals.var_ey_acc__blk1116_dn5 = assign52150_e78955_d_n5;
        locals.var_ey_acc__blk1116_dn6 = assign52150_e78955_d_n6;
        locals.var_ey_acc__blk1116_dn7 = assign52150_e78955_d_n7;
        locals.var_ey_acc__blk1116_dn8 = assign52150_e78955_d_n8;
        locals.var_ey_acc__blk1116_dn9 = assign52150_e78955_d_n9;
        locals.var_ey_acc__blk1116_dn10 = assign52150_e78955_d_n10;
        locals.var_ey_acc__blk1116_dn13 = assign52150_e78955_d_n13;
        locals.var_ey_acc__blk1116_rv = 0.0;

        let (assign52160_e78966, assign52160_e78966_d_n0, assign52160_e78966_d_n2, assign52160_e78966_d_n4, assign52160_e78966_d_n5, assign52160_e78966_d_n6, assign52160_e78966_d_n7, assign52160_e78966_d_n8, assign52160_e78966_d_n9, assign52160_e78966_d_n10, assign52160_e78966_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgp_ws, locals.var_vgp_ws_dn0, locals.var_vgp_ws_dn2, locals.var_vgp_ws_dn4, locals.var_vgp_ws_dn5, locals.var_vgp_ws_dn6, locals.var_vgp_ws_dn7, locals.var_vgp_ws_dn8, locals.var_vgp_ws_dn9, locals.var_vgp_ws_dn10, locals.var_vgp_ws_dn13,)
    }
};
        locals.var_vgp_ws = assign52160_e78966;
        locals.var_vgp_ws_dn0 = assign52160_e78966_d_n0;
        locals.var_vgp_ws_dn2 = assign52160_e78966_d_n2;
        locals.var_vgp_ws_dn4 = assign52160_e78966_d_n4;
        locals.var_vgp_ws_dn5 = assign52160_e78966_d_n5;
        locals.var_vgp_ws_dn6 = assign52160_e78966_d_n6;
        locals.var_vgp_ws_dn7 = assign52160_e78966_d_n7;
        locals.var_vgp_ws_dn8 = assign52160_e78966_d_n8;
        locals.var_vgp_ws_dn9 = assign52160_e78966_d_n9;
        locals.var_vgp_ws_dn10 = assign52160_e78966_d_n10;
        locals.var_vgp_ws_dn13 = assign52160_e78966_d_n13;
        locals.var_vgp_ws_rv = 0.0;

        let (assign52170_e78977, assign52170_e78977_d_n0, assign52170_e78977_d_n2, assign52170_e78977_d_n4, assign52170_e78977_d_n5, assign52170_e78977_d_n6, assign52170_e78977_d_n7, assign52170_e78977_d_n8, assign52170_e78977_d_n9, assign52170_e78977_d_n10, assign52170_e78977_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_w_res_leak, locals.var_w_res_leak_dn0, locals.var_w_res_leak_dn2, locals.var_w_res_leak_dn4, locals.var_w_res_leak_dn5, locals.var_w_res_leak_dn6, locals.var_w_res_leak_dn7, locals.var_w_res_leak_dn8, locals.var_w_res_leak_dn9, locals.var_w_res_leak_dn10, locals.var_w_res_leak_dn13,)
    }
};
        locals.var_w_res_leak = assign52170_e78977;
        locals.var_w_res_leak_dn0 = assign52170_e78977_d_n0;
        locals.var_w_res_leak_dn2 = assign52170_e78977_d_n2;
        locals.var_w_res_leak_dn4 = assign52170_e78977_d_n4;
        locals.var_w_res_leak_dn5 = assign52170_e78977_d_n5;
        locals.var_w_res_leak_dn6 = assign52170_e78977_d_n6;
        locals.var_w_res_leak_dn7 = assign52170_e78977_d_n7;
        locals.var_w_res_leak_dn8 = assign52170_e78977_d_n8;
        locals.var_w_res_leak_dn9 = assign52170_e78977_d_n9;
        locals.var_w_res_leak_dn10 = assign52170_e78977_d_n10;
        locals.var_w_res_leak_dn13 = assign52170_e78977_d_n13;
        locals.var_w_res_leak_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_181(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign52180_e78988, assign52180_e78988_d_n0, assign52180_e78988_d_n2, assign52180_e78988_d_n4, assign52180_e78988_d_n5, assign52180_e78988_d_n6, assign52180_e78988_d_n7, assign52180_e78988_d_n8, assign52180_e78988_d_n9, assign52180_e78988_d_n10, assign52180_e78988_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_w_res, locals.var_w_res_dn0, locals.var_w_res_dn2, locals.var_w_res_dn4, locals.var_w_res_dn5, locals.var_w_res_dn6, locals.var_w_res_dn7, locals.var_w_res_dn8, locals.var_w_res_dn9, locals.var_w_res_dn10, locals.var_w_res_dn13,)
    }
};
        locals.var_w_res = assign52180_e78988;
        locals.var_w_res_dn0 = assign52180_e78988_d_n0;
        locals.var_w_res_dn2 = assign52180_e78988_d_n2;
        locals.var_w_res_dn4 = assign52180_e78988_d_n4;
        locals.var_w_res_dn5 = assign52180_e78988_d_n5;
        locals.var_w_res_dn6 = assign52180_e78988_d_n6;
        locals.var_w_res_dn7 = assign52180_e78988_d_n7;
        locals.var_w_res_dn8 = assign52180_e78988_d_n8;
        locals.var_w_res_dn9 = assign52180_e78988_d_n9;
        locals.var_w_res_dn10 = assign52180_e78988_d_n10;
        locals.var_w_res_dn13 = assign52180_e78988_d_n13;
        locals.var_w_res_rv = 0.0;

        let (assign52190_e78999, assign52190_e78999_d_n0, assign52190_e78999_d_n2, assign52190_e78999_d_n4, assign52190_e78999_d_n5, assign52190_e78999_d_n6, assign52190_e78999_d_n7, assign52190_e78999_d_n8, assign52190_e78999_d_n9, assign52190_e78999_d_n10, assign52190_e78999_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ws__blk1147, locals.var_ws__blk1147_dn0, locals.var_ws__blk1147_dn2, locals.var_ws__blk1147_dn4, locals.var_ws__blk1147_dn5, locals.var_ws__blk1147_dn6, locals.var_ws__blk1147_dn7, locals.var_ws__blk1147_dn8, locals.var_ws__blk1147_dn9, locals.var_ws__blk1147_dn10, locals.var_ws__blk1147_dn13,)
    }
};
        locals.var_ws__blk1147 = assign52190_e78999;
        locals.var_ws__blk1147_dn0 = assign52190_e78999_d_n0;
        locals.var_ws__blk1147_dn2 = assign52190_e78999_d_n2;
        locals.var_ws__blk1147_dn4 = assign52190_e78999_d_n4;
        locals.var_ws__blk1147_dn5 = assign52190_e78999_d_n5;
        locals.var_ws__blk1147_dn6 = assign52190_e78999_d_n6;
        locals.var_ws__blk1147_dn7 = assign52190_e78999_d_n7;
        locals.var_ws__blk1147_dn8 = assign52190_e78999_d_n8;
        locals.var_ws__blk1147_dn9 = assign52190_e78999_d_n9;
        locals.var_ws__blk1147_dn10 = assign52190_e78999_d_n10;
        locals.var_ws__blk1147_dn13 = assign52190_e78999_d_n13;
        locals.var_ws__blk1147_rv = 0.0;

        let (assign52200_e79010, assign52200_e79010_d_n0, assign52200_e79010_d_n2, assign52200_e79010_d_n4, assign52200_e79010_d_n5, assign52200_e79010_d_n6, assign52200_e79010_d_n7, assign52200_e79010_d_n8, assign52200_e79010_d_n9, assign52200_e79010_d_n10, assign52200_e79010_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_s0__blk1322, locals.var_q_s0__blk1322_dn0, locals.var_q_s0__blk1322_dn2, locals.var_q_s0__blk1322_dn4, locals.var_q_s0__blk1322_dn5, locals.var_q_s0__blk1322_dn6, locals.var_q_s0__blk1322_dn7, locals.var_q_s0__blk1322_dn8, locals.var_q_s0__blk1322_dn9, locals.var_q_s0__blk1322_dn10, locals.var_q_s0__blk1322_dn13,)
    }
};
        locals.var_q_s0__blk1322 = assign52200_e79010;
        locals.var_q_s0__blk1322_dn0 = assign52200_e79010_d_n0;
        locals.var_q_s0__blk1322_dn2 = assign52200_e79010_d_n2;
        locals.var_q_s0__blk1322_dn4 = assign52200_e79010_d_n4;
        locals.var_q_s0__blk1322_dn5 = assign52200_e79010_d_n5;
        locals.var_q_s0__blk1322_dn6 = assign52200_e79010_d_n6;
        locals.var_q_s0__blk1322_dn7 = assign52200_e79010_d_n7;
        locals.var_q_s0__blk1322_dn8 = assign52200_e79010_d_n8;
        locals.var_q_s0__blk1322_dn9 = assign52200_e79010_d_n9;
        locals.var_q_s0__blk1322_dn10 = assign52200_e79010_d_n10;
        locals.var_q_s0__blk1322_dn13 = assign52200_e79010_d_n13;
        locals.var_q_s0__blk1322_rv = 0.0;

        let (assign52210_e79027, assign52210_e79027_d_n0, assign52210_e79027_d_n2, assign52210_e79027_d_n4, assign52210_e79027_d_n5, assign52210_e79027_d_n6, assign52210_e79027_d_n7, assign52210_e79027_d_n8, assign52210_e79027_d_n9, assign52210_e79027_d_n10, assign52210_e79027_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign52210_e79021: f64 = (locals.var_vgsz__blk440 - locals.var_vfb);
        let assign52210_e79023: f64 = (assign52210_e79021 + locals.var_dvth);
        let assign52210_e79025: f64 = (assign52210_e79023 - locals.var_dppg);
        (assign52210_e79025, ((locals.var_vgsz__blk440_dn0 + locals.var_dvth_dn0) - locals.var_dppg_dn0), ((locals.var_vgsz__blk440_dn2 + locals.var_dvth_dn2) - locals.var_dppg_dn2), ((locals.var_vgsz__blk440_dn4 + locals.var_dvth_dn4) - locals.var_dppg_dn4), ((locals.var_vgsz__blk440_dn5 + locals.var_dvth_dn5) - locals.var_dppg_dn5), ((locals.var_vgsz__blk440_dn6 + locals.var_dvth_dn6) - locals.var_dppg_dn6), ((locals.var_vgsz__blk440_dn7 + locals.var_dvth_dn7) - locals.var_dppg_dn7), ((locals.var_vgsz__blk440_dn8 + locals.var_dvth_dn8) - locals.var_dppg_dn8), ((locals.var_vgsz__blk440_dn9 + locals.var_dvth_dn9) - locals.var_dppg_dn9), ((locals.var_vgsz__blk440_dn10 + locals.var_dvth_dn10) - locals.var_dppg_dn10), ((locals.var_vgsz__blk440_dn13 + locals.var_dvth_dn13) - locals.var_dppg_dn13),)
    } else {
        (locals.var_vgpz, locals.var_vgpz_dn0, locals.var_vgpz_dn2, locals.var_vgpz_dn4, locals.var_vgpz_dn5, locals.var_vgpz_dn6, locals.var_vgpz_dn7, locals.var_vgpz_dn8, locals.var_vgpz_dn9, locals.var_vgpz_dn10, locals.var_vgpz_dn13,)
    }
};
        locals.var_vgpz = assign52210_e79027;
        locals.var_vgpz_dn0 = assign52210_e79027_d_n0;
        locals.var_vgpz_dn2 = assign52210_e79027_d_n2;
        locals.var_vgpz_dn4 = assign52210_e79027_d_n4;
        locals.var_vgpz_dn5 = assign52210_e79027_d_n5;
        locals.var_vgpz_dn6 = assign52210_e79027_d_n6;
        locals.var_vgpz_dn7 = assign52210_e79027_d_n7;
        locals.var_vgpz_dn8 = assign52210_e79027_d_n8;
        locals.var_vgpz_dn9 = assign52210_e79027_d_n9;
        locals.var_vgpz_dn10 = assign52210_e79027_d_n10;
        locals.var_vgpz_dn13 = assign52210_e79027_d_n13;
        locals.var_vgpz_rv = 0.0;

        let assign52220_e79030: f64 = if 0.0 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1326 = assign52220_e79030;
        locals.var_guard1326_rv = 0.0;

        let (assign52230_e79045, assign52230_e79045_d_n0, assign52230_e79045_d_n2, assign52230_e79045_d_n4, assign52230_e79045_d_n5, assign52230_e79045_d_n6, assign52230_e79045_d_n7, assign52230_e79045_d_n8, assign52230_e79045_d_n9, assign52230_e79045_d_n10, assign52230_e79045_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1326 != 0.0)) {
        let assign52230_e79043: f64 = (locals.var_vgpz - p.p393);
        (assign52230_e79043, locals.var_vgpz_dn0, locals.var_vgpz_dn2, locals.var_vgpz_dn4, locals.var_vgpz_dn5, locals.var_vgpz_dn6, locals.var_vgpz_dn7, locals.var_vgpz_dn8, locals.var_vgpz_dn9, locals.var_vgpz_dn10, locals.var_vgpz_dn13,)
    } else {
        (locals.var_vgp_res__blk1145, locals.var_vgp_res__blk1145_dn0, locals.var_vgp_res__blk1145_dn2, locals.var_vgp_res__blk1145_dn4, locals.var_vgp_res__blk1145_dn5, locals.var_vgp_res__blk1145_dn6, locals.var_vgp_res__blk1145_dn7, locals.var_vgp_res__blk1145_dn8, locals.var_vgp_res__blk1145_dn9, locals.var_vgp_res__blk1145_dn10, locals.var_vgp_res__blk1145_dn13,)
    }
};
        locals.var_vgp_res__blk1145 = assign52230_e79045;
        locals.var_vgp_res__blk1145_dn0 = assign52230_e79045_d_n0;
        locals.var_vgp_res__blk1145_dn2 = assign52230_e79045_d_n2;
        locals.var_vgp_res__blk1145_dn4 = assign52230_e79045_d_n4;
        locals.var_vgp_res__blk1145_dn5 = assign52230_e79045_d_n5;
        locals.var_vgp_res__blk1145_dn6 = assign52230_e79045_d_n6;
        locals.var_vgp_res__blk1145_dn7 = assign52230_e79045_d_n7;
        locals.var_vgp_res__blk1145_dn8 = assign52230_e79045_d_n8;
        locals.var_vgp_res__blk1145_dn9 = assign52230_e79045_d_n9;
        locals.var_vgp_res__blk1145_dn10 = assign52230_e79045_d_n10;
        locals.var_vgp_res__blk1145_dn13 = assign52230_e79045_d_n13;
        locals.var_vgp_res__blk1145_rv = 0.0;

        let assign52240_e79048: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1327 = assign52240_e79048;
        locals.var_guard1327_rv = 0.0;

        let (assign52250_e79068, assign52250_e79068_d_n0, assign52250_e79068_d_n2, assign52250_e79068_d_n4, assign52250_e79068_d_n5, assign52250_e79068_d_n6, assign52250_e79068_d_n7, assign52250_e79068_d_n8, assign52250_e79068_d_n9, assign52250_e79068_d_n10, assign52250_e79068_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1326 == 0.0)) && (locals.var_guard1327 != 0.0)) {
        let assign52250_e79064: f64 = (locals.var_vgsz__blk440 - locals.var_vfb);
        let assign52250_e79066: f64 = (assign52250_e79064 - p.p393);
        (assign52250_e79066, locals.var_vgsz__blk440_dn0, locals.var_vgsz__blk440_dn2, locals.var_vgsz__blk440_dn4, locals.var_vgsz__blk440_dn5, locals.var_vgsz__blk440_dn6, locals.var_vgsz__blk440_dn7, locals.var_vgsz__blk440_dn8, locals.var_vgsz__blk440_dn9, locals.var_vgsz__blk440_dn10, locals.var_vgsz__blk440_dn13,)
    } else {
        (locals.var_vgp_res__blk1145, locals.var_vgp_res__blk1145_dn0, locals.var_vgp_res__blk1145_dn2, locals.var_vgp_res__blk1145_dn4, locals.var_vgp_res__blk1145_dn5, locals.var_vgp_res__blk1145_dn6, locals.var_vgp_res__blk1145_dn7, locals.var_vgp_res__blk1145_dn8, locals.var_vgp_res__blk1145_dn9, locals.var_vgp_res__blk1145_dn10, locals.var_vgp_res__blk1145_dn13,)
    }
};
        locals.var_vgp_res__blk1145 = assign52250_e79068;
        locals.var_vgp_res__blk1145_dn0 = assign52250_e79068_d_n0;
        locals.var_vgp_res__blk1145_dn2 = assign52250_e79068_d_n2;
        locals.var_vgp_res__blk1145_dn4 = assign52250_e79068_d_n4;
        locals.var_vgp_res__blk1145_dn5 = assign52250_e79068_d_n5;
        locals.var_vgp_res__blk1145_dn6 = assign52250_e79068_d_n6;
        locals.var_vgp_res__blk1145_dn7 = assign52250_e79068_d_n7;
        locals.var_vgp_res__blk1145_dn8 = assign52250_e79068_d_n8;
        locals.var_vgp_res__blk1145_dn9 = assign52250_e79068_d_n9;
        locals.var_vgp_res__blk1145_dn10 = assign52250_e79068_d_n10;
        locals.var_vgp_res__blk1145_dn13 = assign52250_e79068_d_n13;
        locals.var_vgp_res__blk1145_rv = 0.0;

        let (assign52260_e79087, assign52260_e79087_d_n0, assign52260_e79087_d_n2, assign52260_e79087_d_n4, assign52260_e79087_d_n5, assign52260_e79087_d_n6, assign52260_e79087_d_n7, assign52260_e79087_d_n8, assign52260_e79087_d_n9, assign52260_e79087_d_n10, assign52260_e79087_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1326 == 0.0)) && (locals.var_guard1327 == 0.0)) {
        let assign52260_e79085: f64 = (locals.var_vgp - p.p393);
        (assign52260_e79085, locals.var_vgp_dn0, locals.var_vgp_dn2, locals.var_vgp_dn4, locals.var_vgp_dn5, locals.var_vgp_dn6, locals.var_vgp_dn7, locals.var_vgp_dn8, locals.var_vgp_dn9, locals.var_vgp_dn10, locals.var_vgp_dn13,)
    } else {
        (locals.var_vgp_res__blk1145, locals.var_vgp_res__blk1145_dn0, locals.var_vgp_res__blk1145_dn2, locals.var_vgp_res__blk1145_dn4, locals.var_vgp_res__blk1145_dn5, locals.var_vgp_res__blk1145_dn6, locals.var_vgp_res__blk1145_dn7, locals.var_vgp_res__blk1145_dn8, locals.var_vgp_res__blk1145_dn9, locals.var_vgp_res__blk1145_dn10, locals.var_vgp_res__blk1145_dn13,)
    }
};
        locals.var_vgp_res__blk1145 = assign52260_e79087;
        locals.var_vgp_res__blk1145_dn0 = assign52260_e79087_d_n0;
        locals.var_vgp_res__blk1145_dn2 = assign52260_e79087_d_n2;
        locals.var_vgp_res__blk1145_dn4 = assign52260_e79087_d_n4;
        locals.var_vgp_res__blk1145_dn5 = assign52260_e79087_d_n5;
        locals.var_vgp_res__blk1145_dn6 = assign52260_e79087_d_n6;
        locals.var_vgp_res__blk1145_dn7 = assign52260_e79087_d_n7;
        locals.var_vgp_res__blk1145_dn8 = assign52260_e79087_d_n8;
        locals.var_vgp_res__blk1145_dn9 = assign52260_e79087_d_n9;
        locals.var_vgp_res__blk1145_dn10 = assign52260_e79087_d_n10;
        locals.var_vgp_res__blk1145_dn13 = assign52260_e79087_d_n13;
        locals.var_vgp_res__blk1145_rv = 0.0;

        let assign52270_e79089: f64 = (locals.var_tnp__blk1148).abs();
        let assign52270_e79091: f64 = if assign52270_e79089 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1328 = assign52270_e79091;
        locals.var_guard1328_rv = 0.0;

        let (assign52280_e79104, assign52280_e79104_d_n0, assign52280_e79104_d_n2, assign52280_e79104_d_n4, assign52280_e79104_d_n5, assign52280_e79104_d_n6, assign52280_e79104_d_n7, assign52280_e79104_d_n8, assign52280_e79104_d_n9, assign52280_e79104_d_n10, assign52280_e79104_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ps0_res, locals.var_ps0_res_dn0, locals.var_ps0_res_dn2, locals.var_ps0_res_dn4, locals.var_ps0_res_dn5, locals.var_ps0_res_dn6, locals.var_ps0_res_dn7, locals.var_ps0_res_dn8, locals.var_ps0_res_dn9, locals.var_ps0_res_dn10, locals.var_ps0_res_dn13,)
    }
};
        locals.var_ps0_res = assign52280_e79104;
        locals.var_ps0_res_dn0 = assign52280_e79104_d_n0;
        locals.var_ps0_res_dn2 = assign52280_e79104_d_n2;
        locals.var_ps0_res_dn4 = assign52280_e79104_d_n4;
        locals.var_ps0_res_dn5 = assign52280_e79104_d_n5;
        locals.var_ps0_res_dn6 = assign52280_e79104_d_n6;
        locals.var_ps0_res_dn7 = assign52280_e79104_d_n7;
        locals.var_ps0_res_dn8 = assign52280_e79104_d_n8;
        locals.var_ps0_res_dn9 = assign52280_e79104_d_n9;
        locals.var_ps0_res_dn10 = assign52280_e79104_d_n10;
        locals.var_ps0_res_dn13 = assign52280_e79104_d_n13;
        locals.var_ps0_res_rv = 0.0;

        let (assign52290_e79118, assign52290_e79118_d_n0, assign52290_e79118_d_n2, assign52290_e79118_d_n4, assign52290_e79118_d_n5, assign52290_e79118_d_n6, assign52290_e79118_d_n7, assign52290_e79118_d_n8, assign52290_e79118_d_n9, assign52290_e79118_d_n10, assign52290_e79118_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn4, locals.var_ps0_dn5, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn8, locals.var_ps0_dn9, locals.var_ps0_dn10, locals.var_ps0_dn13,)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn13,)
    }
};
        locals.var_ps0dep = assign52290_e79118;
        locals.var_ps0dep_dn0 = assign52290_e79118_d_n0;
        locals.var_ps0dep_dn2 = assign52290_e79118_d_n2;
        locals.var_ps0dep_dn4 = assign52290_e79118_d_n4;
        locals.var_ps0dep_dn5 = assign52290_e79118_d_n5;
        locals.var_ps0dep_dn6 = assign52290_e79118_d_n6;
        locals.var_ps0dep_dn7 = assign52290_e79118_d_n7;
        locals.var_ps0dep_dn8 = assign52290_e79118_d_n8;
        locals.var_ps0dep_dn9 = assign52290_e79118_d_n9;
        locals.var_ps0dep_dn10 = assign52290_e79118_d_n10;
        locals.var_ps0dep_dn13 = assign52290_e79118_d_n13;
        locals.var_ps0dep_rv = 0.0;

        let (assign52300_e79134, assign52300_e79134_d_n0, assign52300_e79134_d_n2, assign52300_e79134_d_n4, assign52300_e79134_d_n5, assign52300_e79134_d_n6, assign52300_e79134_d_n7, assign52300_e79134_d_n8, assign52300_e79134_d_n9, assign52300_e79134_d_n10, assign52300_e79134_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) {
        let assign52300_e79132: f64 = (p.p399 * locals.var_vbsc__blk1117);
        (assign52300_e79132, (p.p399 * locals.var_vbsc__blk1117_dn0), (p.p399 * locals.var_vbsc__blk1117_dn2), (p.p399 * locals.var_vbsc__blk1117_dn4), (p.p399 * locals.var_vbsc__blk1117_dn5), (p.p399 * locals.var_vbsc__blk1117_dn6), (p.p399 * locals.var_vbsc__blk1117_dn7), (p.p399 * locals.var_vbsc__blk1117_dn8), (p.p399 * locals.var_vbsc__blk1117_dn9), (p.p399 * locals.var_vbsc__blk1117_dn10), (p.p399 * locals.var_vbsc__blk1117_dn13),)
    } else {
        (locals.var_depvbs, locals.var_depvbs_dn0, locals.var_depvbs_dn2, locals.var_depvbs_dn4, locals.var_depvbs_dn5, locals.var_depvbs_dn6, locals.var_depvbs_dn7, locals.var_depvbs_dn8, locals.var_depvbs_dn9, locals.var_depvbs_dn10, locals.var_depvbs_dn13,)
    }
};
        locals.var_depvbs = assign52300_e79134;
        locals.var_depvbs_dn0 = assign52300_e79134_d_n0;
        locals.var_depvbs_dn2 = assign52300_e79134_d_n2;
        locals.var_depvbs_dn4 = assign52300_e79134_d_n4;
        locals.var_depvbs_dn5 = assign52300_e79134_d_n5;
        locals.var_depvbs_dn6 = assign52300_e79134_d_n6;
        locals.var_depvbs_dn7 = assign52300_e79134_d_n7;
        locals.var_depvbs_dn8 = assign52300_e79134_d_n8;
        locals.var_depvbs_dn9 = assign52300_e79134_d_n9;
        locals.var_depvbs_dn10 = assign52300_e79134_d_n10;
        locals.var_depvbs_dn13 = assign52300_e79134_d_n13;
        locals.var_depvbs_rv = 0.0;

        let (assign52310_e79152,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) {
        let assign52310_e79148: f64 = (locals.var_vfb + p.p393);
        let assign52310_e79150: f64 = (assign52310_e79148 - 3.0);
        (assign52310_e79150,)
    } else {
        (locals.var_vgp_leak,)
    }
};
        locals.var_vgp_leak = assign52310_e79152;
        locals.var_vgp_leak_rv = 0.0;

        let assign52320_e79155: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1329 = assign52320_e79155;
        locals.var_guard1329_rv = 0.0;

        let (assign52330_e79173, assign52330_e79173_d_n0, assign52330_e79173_d_n2, assign52330_e79173_d_n4, assign52330_e79173_d_n5, assign52330_e79173_d_n6, assign52330_e79173_d_n7, assign52330_e79173_d_n8, assign52330_e79173_d_n9, assign52330_e79173_d_n10, assign52330_e79173_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1329 != 0.0)) {
        let assign52330_e79171: f64 = (p.p399 * locals.var_vbsc__blk1117);
        (assign52330_e79171, (p.p399 * locals.var_vbsc__blk1117_dn0), (p.p399 * locals.var_vbsc__blk1117_dn2), (p.p399 * locals.var_vbsc__blk1117_dn4), (p.p399 * locals.var_vbsc__blk1117_dn5), (p.p399 * locals.var_vbsc__blk1117_dn6), (p.p399 * locals.var_vbsc__blk1117_dn7), (p.p399 * locals.var_vbsc__blk1117_dn8), (p.p399 * locals.var_vbsc__blk1117_dn9), (p.p399 * locals.var_vbsc__blk1117_dn10), (p.p399 * locals.var_vbsc__blk1117_dn13),)
    } else {
        (locals.var_depvbs, locals.var_depvbs_dn0, locals.var_depvbs_dn2, locals.var_depvbs_dn4, locals.var_depvbs_dn5, locals.var_depvbs_dn6, locals.var_depvbs_dn7, locals.var_depvbs_dn8, locals.var_depvbs_dn9, locals.var_depvbs_dn10, locals.var_depvbs_dn13,)
    }
};
        locals.var_depvbs = assign52330_e79173;
        locals.var_depvbs_dn0 = assign52330_e79173_d_n0;
        locals.var_depvbs_dn2 = assign52330_e79173_d_n2;
        locals.var_depvbs_dn4 = assign52330_e79173_d_n4;
        locals.var_depvbs_dn5 = assign52330_e79173_d_n5;
        locals.var_depvbs_dn6 = assign52330_e79173_d_n6;
        locals.var_depvbs_dn7 = assign52330_e79173_d_n7;
        locals.var_depvbs_dn8 = assign52330_e79173_d_n8;
        locals.var_depvbs_dn9 = assign52330_e79173_d_n9;
        locals.var_depvbs_dn10 = assign52330_e79173_d_n10;
        locals.var_depvbs_dn13 = assign52330_e79173_d_n13;
        locals.var_depvbs_rv = 0.0;

        let (assign52340_e79191, assign52340_e79191_d_n0, assign52340_e79191_d_n2, assign52340_e79191_d_n4, assign52340_e79191_d_n5, assign52340_e79191_d_n6, assign52340_e79191_d_n7, assign52340_e79191_d_n8, assign52340_e79191_d_n9, assign52340_e79191_d_n10, assign52340_e79191_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1329 != 0.0)) {
        let assign52340_e79189: f64 = (locals.var_depvbs - 1.0);
        (assign52340_e79189, locals.var_depvbs_dn0, locals.var_depvbs_dn2, locals.var_depvbs_dn4, locals.var_depvbs_dn5, locals.var_depvbs_dn6, locals.var_depvbs_dn7, locals.var_depvbs_dn8, locals.var_depvbs_dn9, locals.var_depvbs_dn10, locals.var_depvbs_dn13,)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn13,)
    }
};
        locals.var_ps0dep = assign52340_e79191;
        locals.var_ps0dep_dn0 = assign52340_e79191_d_n0;
        locals.var_ps0dep_dn2 = assign52340_e79191_d_n2;
        locals.var_ps0dep_dn4 = assign52340_e79191_d_n4;
        locals.var_ps0dep_dn5 = assign52340_e79191_d_n5;
        locals.var_ps0dep_dn6 = assign52340_e79191_d_n6;
        locals.var_ps0dep_dn7 = assign52340_e79191_d_n7;
        locals.var_ps0dep_dn8 = assign52340_e79191_d_n8;
        locals.var_ps0dep_dn9 = assign52340_e79191_d_n9;
        locals.var_ps0dep_dn10 = assign52340_e79191_d_n10;
        locals.var_ps0dep_dn13 = assign52340_e79191_d_n13;
        locals.var_ps0dep_rv = 0.0;

        let (assign52350_e79207, assign52350_e79207_d_n0, assign52350_e79207_d_n2, assign52350_e79207_d_n4, assign52350_e79207_d_n5, assign52350_e79207_d_n6, assign52350_e79207_d_n7, assign52350_e79207_d_n8, assign52350_e79207_d_n9, assign52350_e79207_d_n10, assign52350_e79207_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1329 != 0.0)) {
        (locals.var_vgp_leak, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgp_ws, locals.var_vgp_ws_dn0, locals.var_vgp_ws_dn2, locals.var_vgp_ws_dn4, locals.var_vgp_ws_dn5, locals.var_vgp_ws_dn6, locals.var_vgp_ws_dn7, locals.var_vgp_ws_dn8, locals.var_vgp_ws_dn9, locals.var_vgp_ws_dn10, locals.var_vgp_ws_dn13,)
    }
};
        locals.var_vgp_ws = assign52350_e79207;
        locals.var_vgp_ws_dn0 = assign52350_e79207_d_n0;
        locals.var_vgp_ws_dn2 = assign52350_e79207_d_n2;
        locals.var_vgp_ws_dn4 = assign52350_e79207_d_n4;
        locals.var_vgp_ws_dn5 = assign52350_e79207_d_n5;
        locals.var_vgp_ws_dn6 = assign52350_e79207_d_n6;
        locals.var_vgp_ws_dn7 = assign52350_e79207_d_n7;
        locals.var_vgp_ws_dn8 = assign52350_e79207_d_n8;
        locals.var_vgp_ws_dn9 = assign52350_e79207_d_n9;
        locals.var_vgp_ws_dn10 = assign52350_e79207_d_n10;
        locals.var_vgp_ws_dn13 = assign52350_e79207_d_n13;
        locals.var_vgp_ws_rv = 0.0;

        let (assign52360_e79223, assign52360_e79223_d_n0, assign52360_e79223_d_n2, assign52360_e79223_d_n4, assign52360_e79223_d_n5, assign52360_e79223_d_n6, assign52360_e79223_d_n7, assign52360_e79223_d_n8, assign52360_e79223_d_n9, assign52360_e79223_d_n10, assign52360_e79223_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1329 != 0.0)) {
        (locals.var_vgp_leak, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgp_res_raw, locals.var_vgp_res_raw_dn0, locals.var_vgp_res_raw_dn2, locals.var_vgp_res_raw_dn4, locals.var_vgp_res_raw_dn5, locals.var_vgp_res_raw_dn6, locals.var_vgp_res_raw_dn7, locals.var_vgp_res_raw_dn8, locals.var_vgp_res_raw_dn9, locals.var_vgp_res_raw_dn10, locals.var_vgp_res_raw_dn13,)
    }
};
        locals.var_vgp_res_raw = assign52360_e79223;
        locals.var_vgp_res_raw_dn0 = assign52360_e79223_d_n0;
        locals.var_vgp_res_raw_dn2 = assign52360_e79223_d_n2;
        locals.var_vgp_res_raw_dn4 = assign52360_e79223_d_n4;
        locals.var_vgp_res_raw_dn5 = assign52360_e79223_d_n5;
        locals.var_vgp_res_raw_dn6 = assign52360_e79223_d_n6;
        locals.var_vgp_res_raw_dn7 = assign52360_e79223_d_n7;
        locals.var_vgp_res_raw_dn8 = assign52360_e79223_d_n8;
        locals.var_vgp_res_raw_dn9 = assign52360_e79223_d_n9;
        locals.var_vgp_res_raw_dn10 = assign52360_e79223_d_n10;
        locals.var_vgp_res_raw_dn13 = assign52360_e79223_d_n13;
        locals.var_vgp_res_raw_rv = 0.0;

        let (assign52370_e79244, assign52370_e79244_d_n0, assign52370_e79244_d_n2, assign52370_e79244_d_n4, assign52370_e79244_d_n5, assign52370_e79244_d_n6, assign52370_e79244_d_n7, assign52370_e79244_d_n8, assign52370_e79244_d_n9, assign52370_e79244_d_n10, assign52370_e79244_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1329 == 0.0)) {
        let assign52370_e79240: f64 = (p.p399 * locals.var_vbsc__blk1117);
        let assign52370_e79242: f64 = (assign52370_e79240 - 0.1);
        (assign52370_e79242, (p.p399 * locals.var_vbsc__blk1117_dn0), (p.p399 * locals.var_vbsc__blk1117_dn2), (p.p399 * locals.var_vbsc__blk1117_dn4), (p.p399 * locals.var_vbsc__blk1117_dn5), (p.p399 * locals.var_vbsc__blk1117_dn6), (p.p399 * locals.var_vbsc__blk1117_dn7), (p.p399 * locals.var_vbsc__blk1117_dn8), (p.p399 * locals.var_vbsc__blk1117_dn9), (p.p399 * locals.var_vbsc__blk1117_dn10), (p.p399 * locals.var_vbsc__blk1117_dn13),)
    } else {
        (locals.var_depvbs, locals.var_depvbs_dn0, locals.var_depvbs_dn2, locals.var_depvbs_dn4, locals.var_depvbs_dn5, locals.var_depvbs_dn6, locals.var_depvbs_dn7, locals.var_depvbs_dn8, locals.var_depvbs_dn9, locals.var_depvbs_dn10, locals.var_depvbs_dn13,)
    }
};
        locals.var_depvbs = assign52370_e79244;
        locals.var_depvbs_dn0 = assign52370_e79244_d_n0;
        locals.var_depvbs_dn2 = assign52370_e79244_d_n2;
        locals.var_depvbs_dn4 = assign52370_e79244_d_n4;
        locals.var_depvbs_dn5 = assign52370_e79244_d_n5;
        locals.var_depvbs_dn6 = assign52370_e79244_d_n6;
        locals.var_depvbs_dn7 = assign52370_e79244_d_n7;
        locals.var_depvbs_dn8 = assign52370_e79244_d_n8;
        locals.var_depvbs_dn9 = assign52370_e79244_d_n9;
        locals.var_depvbs_dn10 = assign52370_e79244_d_n10;
        locals.var_depvbs_dn13 = assign52370_e79244_d_n13;
        locals.var_depvbs_rv = 0.0;

        let (assign52380_e79261, assign52380_e79261_d_n0, assign52380_e79261_d_n2, assign52380_e79261_d_n4, assign52380_e79261_d_n5, assign52380_e79261_d_n6, assign52380_e79261_d_n7, assign52380_e79261_d_n8, assign52380_e79261_d_n9, assign52380_e79261_d_n10, assign52380_e79261_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1329 == 0.0)) {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn4, locals.var_ps0_dn5, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn8, locals.var_ps0_dn9, locals.var_ps0_dn10, locals.var_ps0_dn13,)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn13,)
    }
};
        locals.var_ps0dep = assign52380_e79261;
        locals.var_ps0dep_dn0 = assign52380_e79261_d_n0;
        locals.var_ps0dep_dn2 = assign52380_e79261_d_n2;
        locals.var_ps0dep_dn4 = assign52380_e79261_d_n4;
        locals.var_ps0dep_dn5 = assign52380_e79261_d_n5;
        locals.var_ps0dep_dn6 = assign52380_e79261_d_n6;
        locals.var_ps0dep_dn7 = assign52380_e79261_d_n7;
        locals.var_ps0dep_dn8 = assign52380_e79261_d_n8;
        locals.var_ps0dep_dn9 = assign52380_e79261_d_n9;
        locals.var_ps0dep_dn10 = assign52380_e79261_d_n10;
        locals.var_ps0dep_dn13 = assign52380_e79261_d_n13;
        locals.var_ps0dep_rv = 0.0;

        let (assign52390_e79278, assign52390_e79278_d_n0, assign52390_e79278_d_n2, assign52390_e79278_d_n4, assign52390_e79278_d_n5, assign52390_e79278_d_n6, assign52390_e79278_d_n7, assign52390_e79278_d_n8, assign52390_e79278_d_n9, assign52390_e79278_d_n10, assign52390_e79278_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1329 == 0.0)) {
        (locals.var_vgp_res__blk1145, locals.var_vgp_res__blk1145_dn0, locals.var_vgp_res__blk1145_dn2, locals.var_vgp_res__blk1145_dn4, locals.var_vgp_res__blk1145_dn5, locals.var_vgp_res__blk1145_dn6, locals.var_vgp_res__blk1145_dn7, locals.var_vgp_res__blk1145_dn8, locals.var_vgp_res__blk1145_dn9, locals.var_vgp_res__blk1145_dn10, locals.var_vgp_res__blk1145_dn13,)
    } else {
        (locals.var_vgp_ws, locals.var_vgp_ws_dn0, locals.var_vgp_ws_dn2, locals.var_vgp_ws_dn4, locals.var_vgp_ws_dn5, locals.var_vgp_ws_dn6, locals.var_vgp_ws_dn7, locals.var_vgp_ws_dn8, locals.var_vgp_ws_dn9, locals.var_vgp_ws_dn10, locals.var_vgp_ws_dn13,)
    }
};
        locals.var_vgp_ws = assign52390_e79278;
        locals.var_vgp_ws_dn0 = assign52390_e79278_d_n0;
        locals.var_vgp_ws_dn2 = assign52390_e79278_d_n2;
        locals.var_vgp_ws_dn4 = assign52390_e79278_d_n4;
        locals.var_vgp_ws_dn5 = assign52390_e79278_d_n5;
        locals.var_vgp_ws_dn6 = assign52390_e79278_d_n6;
        locals.var_vgp_ws_dn7 = assign52390_e79278_d_n7;
        locals.var_vgp_ws_dn8 = assign52390_e79278_d_n8;
        locals.var_vgp_ws_dn9 = assign52390_e79278_d_n9;
        locals.var_vgp_ws_dn10 = assign52390_e79278_d_n10;
        locals.var_vgp_ws_dn13 = assign52390_e79278_d_n13;
        locals.var_vgp_ws_rv = 0.0;

        let (assign52400_e79295, assign52400_e79295_d_n0, assign52400_e79295_d_n2, assign52400_e79295_d_n4, assign52400_e79295_d_n5, assign52400_e79295_d_n6, assign52400_e79295_d_n7, assign52400_e79295_d_n8, assign52400_e79295_d_n9, assign52400_e79295_d_n10, assign52400_e79295_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1329 == 0.0)) {
        (locals.var_vgp_res__blk1145, locals.var_vgp_res__blk1145_dn0, locals.var_vgp_res__blk1145_dn2, locals.var_vgp_res__blk1145_dn4, locals.var_vgp_res__blk1145_dn5, locals.var_vgp_res__blk1145_dn6, locals.var_vgp_res__blk1145_dn7, locals.var_vgp_res__blk1145_dn8, locals.var_vgp_res__blk1145_dn9, locals.var_vgp_res__blk1145_dn10, locals.var_vgp_res__blk1145_dn13,)
    } else {
        (locals.var_vgp_res_raw, locals.var_vgp_res_raw_dn0, locals.var_vgp_res_raw_dn2, locals.var_vgp_res_raw_dn4, locals.var_vgp_res_raw_dn5, locals.var_vgp_res_raw_dn6, locals.var_vgp_res_raw_dn7, locals.var_vgp_res_raw_dn8, locals.var_vgp_res_raw_dn9, locals.var_vgp_res_raw_dn10, locals.var_vgp_res_raw_dn13,)
    }
};
        locals.var_vgp_res_raw = assign52400_e79295;
        locals.var_vgp_res_raw_dn0 = assign52400_e79295_d_n0;
        locals.var_vgp_res_raw_dn2 = assign52400_e79295_d_n2;
        locals.var_vgp_res_raw_dn4 = assign52400_e79295_d_n4;
        locals.var_vgp_res_raw_dn5 = assign52400_e79295_d_n5;
        locals.var_vgp_res_raw_dn6 = assign52400_e79295_d_n6;
        locals.var_vgp_res_raw_dn7 = assign52400_e79295_d_n7;
        locals.var_vgp_res_raw_dn8 = assign52400_e79295_d_n8;
        locals.var_vgp_res_raw_dn9 = assign52400_e79295_d_n9;
        locals.var_vgp_res_raw_dn10 = assign52400_e79295_d_n10;
        locals.var_vgp_res_raw_dn13 = assign52400_e79295_d_n13;
        locals.var_vgp_res_raw_rv = 0.0;

        let (assign52410_e79309,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign52410_e79309;
        locals.var_flg_conv_rv = 0.0;

        let (assign52420_e79323,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign52420_e79323;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_182(
        locals: &mut StampLocals,
    ) {
        let mut assign52430_loop_guard: usize = 0;
        while {
            let assign52430_cond_e79338: f64 = if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_lp_s0 <= 150.0)) { 1.0 } else { 0.0 };
            assign52430_cond_e79338 != 0.0
        } {
            assign52430_loop_guard += 1;
            assert!(assign52430_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign52430_body0_e79354, assign52430_body0_e79354_d_n0, assign52430_body0_e79354_d_n2, assign52430_body0_e79354_d_n4, assign52430_body0_e79354_d_n5, assign52430_body0_e79354_d_n6, assign52430_body0_e79354_d_n7, assign52430_body0_e79354_d_n8, assign52430_body0_e79354_d_n9, assign52430_body0_e79354_d_n10, assign52430_body0_e79354_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) {
        let assign52430_body0_e79352: f64 = (locals.var_beta * locals.var_ps0dep);
        (assign52430_body0_e79352, ((locals.var_beta_dn0 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn0)), ((locals.var_beta_dn2 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn2)), ((locals.var_beta_dn4 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn4)), ((locals.var_beta_dn5 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn5)), ((locals.var_beta_dn6 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn6)), ((locals.var_beta_dn7 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn7)), ((locals.var_beta_dn8 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn8)), ((locals.var_beta_dn9 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn9)), ((locals.var_beta_dn10 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn10)), ((locals.var_beta_dn13 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign52430_body0_e79354;
            locals.var_t1_dn0 = assign52430_body0_e79354_d_n0;
            locals.var_t1_dn2 = assign52430_body0_e79354_d_n2;
            locals.var_t1_dn4 = assign52430_body0_e79354_d_n4;
            locals.var_t1_dn5 = assign52430_body0_e79354_d_n5;
            locals.var_t1_dn6 = assign52430_body0_e79354_d_n6;
            locals.var_t1_dn7 = assign52430_body0_e79354_d_n7;
            locals.var_t1_dn8 = assign52430_body0_e79354_d_n8;
            locals.var_t1_dn9 = assign52430_body0_e79354_d_n9;
            locals.var_t1_dn10 = assign52430_body0_e79354_d_n10;
            locals.var_t1_dn13 = assign52430_body0_e79354_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign52430_body1_e79369, assign52430_body1_e79369_d_n0, assign52430_body1_e79369_d_n2, assign52430_body1_e79369_d_n4, assign52430_body1_e79369_d_n5, assign52430_body1_e79369_d_n6, assign52430_body1_e79369_d_n7, assign52430_body1_e79369_d_n8, assign52430_body1_e79369_d_n9, assign52430_body1_e79369_d_n10, assign52430_body1_e79369_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) {
        let assign52430_body1_e79367: f64 = (locals.var_t1).exp();
        (assign52430_body1_e79367, (assign52430_body1_e79367 * locals.var_t1_dn0), (assign52430_body1_e79367 * locals.var_t1_dn2), (assign52430_body1_e79367 * locals.var_t1_dn4), (assign52430_body1_e79367 * locals.var_t1_dn5), (assign52430_body1_e79367 * locals.var_t1_dn6), (assign52430_body1_e79367 * locals.var_t1_dn7), (assign52430_body1_e79367 * locals.var_t1_dn8), (assign52430_body1_e79367 * locals.var_t1_dn9), (assign52430_body1_e79367 * locals.var_t1_dn10), (assign52430_body1_e79367 * locals.var_t1_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign52430_body1_e79369;
            locals.var_t2_dn0 = assign52430_body1_e79369_d_n0;
            locals.var_t2_dn2 = assign52430_body1_e79369_d_n2;
            locals.var_t2_dn4 = assign52430_body1_e79369_d_n4;
            locals.var_t2_dn5 = assign52430_body1_e79369_d_n5;
            locals.var_t2_dn6 = assign52430_body1_e79369_d_n6;
            locals.var_t2_dn7 = assign52430_body1_e79369_d_n7;
            locals.var_t2_dn8 = assign52430_body1_e79369_d_n8;
            locals.var_t2_dn9 = assign52430_body1_e79369_d_n9;
            locals.var_t2_dn10 = assign52430_body1_e79369_d_n10;
            locals.var_t2_dn13 = assign52430_body1_e79369_d_n13;
            locals.var_t2_rv = 0.0;
            let assign52430_body2_e79372: f64 = if locals.var_ps0dep >= 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1330 = assign52430_body2_e79372;
            locals.var_guard1330_rv = 0.0;
            let (assign52430_body3_e79398, assign52430_body3_e79398_d_n0, assign52430_body3_e79398_d_n2, assign52430_body3_e79398_d_n4, assign52430_body3_e79398_d_n5, assign52430_body3_e79398_d_n6, assign52430_body3_e79398_d_n7, assign52430_body3_e79398_d_n8, assign52430_body3_e79398_d_n9, assign52430_body3_e79398_d_n10, assign52430_body3_e79398_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1330 != 0.0)) {
        let assign52430_body3_e79387: f64 = (-locals.var_cnst0);
        let assign52430_body3_e79390: f64 = (locals.var_t2 - 1.0);
        let assign52430_body3_e79392: f64 = (assign52430_body3_e79390 - locals.var_t1);
        let assign52430_body3_e79394: f64 = (assign52430_body3_e79392 + 1e-15);
        let assign52430_body3_e79395: f64 = (assign52430_body3_e79394).sqrt();
        let assign52430_body3_e79396: f64 = (assign52430_body3_e79387 * assign52430_body3_e79395);
        (assign52430_body3_e79396, (((-locals.var_cnst0_dn0) * assign52430_body3_e79395) + (assign52430_body3_e79387 * ((locals.var_t2_dn0 - locals.var_t1_dn0) / (2.0 * assign52430_body3_e79395)))), (((-locals.var_cnst0_dn2) * assign52430_body3_e79395) + (assign52430_body3_e79387 * ((locals.var_t2_dn2 - locals.var_t1_dn2) / (2.0 * assign52430_body3_e79395)))), (((-locals.var_cnst0_dn4) * assign52430_body3_e79395) + (assign52430_body3_e79387 * ((locals.var_t2_dn4 - locals.var_t1_dn4) / (2.0 * assign52430_body3_e79395)))), (((-locals.var_cnst0_dn5) * assign52430_body3_e79395) + (assign52430_body3_e79387 * ((locals.var_t2_dn5 - locals.var_t1_dn5) / (2.0 * assign52430_body3_e79395)))), (((-locals.var_cnst0_dn6) * assign52430_body3_e79395) + (assign52430_body3_e79387 * ((locals.var_t2_dn6 - locals.var_t1_dn6) / (2.0 * assign52430_body3_e79395)))), (((-locals.var_cnst0_dn7) * assign52430_body3_e79395) + (assign52430_body3_e79387 * ((locals.var_t2_dn7 - locals.var_t1_dn7) / (2.0 * assign52430_body3_e79395)))), (((-locals.var_cnst0_dn8) * assign52430_body3_e79395) + (assign52430_body3_e79387 * ((locals.var_t2_dn8 - locals.var_t1_dn8) / (2.0 * assign52430_body3_e79395)))), (((-locals.var_cnst0_dn9) * assign52430_body3_e79395) + (assign52430_body3_e79387 * ((locals.var_t2_dn9 - locals.var_t1_dn9) / (2.0 * assign52430_body3_e79395)))), (((-locals.var_cnst0_dn10) * assign52430_body3_e79395) + (assign52430_body3_e79387 * ((locals.var_t2_dn10 - locals.var_t1_dn10) / (2.0 * assign52430_body3_e79395)))), (((-locals.var_cnst0_dn13) * assign52430_body3_e79395) + (assign52430_body3_e79387 * ((locals.var_t2_dn13 - locals.var_t1_dn13) / (2.0 * assign52430_body3_e79395)))),)
    } else {
        (locals.var_q_s0__blk1322, locals.var_q_s0__blk1322_dn0, locals.var_q_s0__blk1322_dn2, locals.var_q_s0__blk1322_dn4, locals.var_q_s0__blk1322_dn5, locals.var_q_s0__blk1322_dn6, locals.var_q_s0__blk1322_dn7, locals.var_q_s0__blk1322_dn8, locals.var_q_s0__blk1322_dn9, locals.var_q_s0__blk1322_dn10, locals.var_q_s0__blk1322_dn13,)
    }
};
            locals.var_q_s0__blk1322 = assign52430_body3_e79398;
            locals.var_q_s0__blk1322_dn0 = assign52430_body3_e79398_d_n0;
            locals.var_q_s0__blk1322_dn2 = assign52430_body3_e79398_d_n2;
            locals.var_q_s0__blk1322_dn4 = assign52430_body3_e79398_d_n4;
            locals.var_q_s0__blk1322_dn5 = assign52430_body3_e79398_d_n5;
            locals.var_q_s0__blk1322_dn6 = assign52430_body3_e79398_d_n6;
            locals.var_q_s0__blk1322_dn7 = assign52430_body3_e79398_d_n7;
            locals.var_q_s0__blk1322_dn8 = assign52430_body3_e79398_d_n8;
            locals.var_q_s0__blk1322_dn9 = assign52430_body3_e79398_d_n9;
            locals.var_q_s0__blk1322_dn10 = assign52430_body3_e79398_d_n10;
            locals.var_q_s0__blk1322_dn13 = assign52430_body3_e79398_d_n13;
            locals.var_q_s0__blk1322_rv = 0.0;
            let (assign52430_body4_e79426, assign52430_body4_e79426_d_n0, assign52430_body4_e79426_d_n2, assign52430_body4_e79426_d_n4, assign52430_body4_e79426_d_n5, assign52430_body4_e79426_d_n6, assign52430_body4_e79426_d_n7, assign52430_body4_e79426_d_n8, assign52430_body4_e79426_d_n9, assign52430_body4_e79426_d_n10, assign52430_body4_e79426_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1330 != 0.0)) {
        let assign52430_body4_e79414: f64 = (0.5 * locals.var_cnst0);
        let assign52430_body4_e79416: f64 = (assign52430_body4_e79414 * locals.var_cnst0);
        let assign52430_body4_e79418: f64 = (assign52430_body4_e79416 / locals.var_q_s0__blk1322);
        let assign52430_body4_e79421: f64 = (locals.var_beta * locals.var_t2);
        let assign52430_body4_e79423: f64 = (assign52430_body4_e79421 - locals.var_beta);
        let assign52430_body4_e79424: f64 = (assign52430_body4_e79418 * assign52430_body4_e79423);
        (assign52430_body4_e79424, ((((((((0.5 * locals.var_cnst0_dn0) * locals.var_cnst0) + (assign52430_body4_e79414 * locals.var_cnst0_dn0)) * locals.var_q_s0__blk1322) - (assign52430_body4_e79416 * locals.var_q_s0__blk1322_dn0)) / (locals.var_q_s0__blk1322 * locals.var_q_s0__blk1322)) * assign52430_body4_e79423) + (assign52430_body4_e79418 * (((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0)) - locals.var_beta_dn0))), ((((((((0.5 * locals.var_cnst0_dn2) * locals.var_cnst0) + (assign52430_body4_e79414 * locals.var_cnst0_dn2)) * locals.var_q_s0__blk1322) - (assign52430_body4_e79416 * locals.var_q_s0__blk1322_dn2)) / (locals.var_q_s0__blk1322 * locals.var_q_s0__blk1322)) * assign52430_body4_e79423) + (assign52430_body4_e79418 * (((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2)) - locals.var_beta_dn2))), ((((((((0.5 * locals.var_cnst0_dn4) * locals.var_cnst0) + (assign52430_body4_e79414 * locals.var_cnst0_dn4)) * locals.var_q_s0__blk1322) - (assign52430_body4_e79416 * locals.var_q_s0__blk1322_dn4)) / (locals.var_q_s0__blk1322 * locals.var_q_s0__blk1322)) * assign52430_body4_e79423) + (assign52430_body4_e79418 * (((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4)) - locals.var_beta_dn4))), ((((((((0.5 * locals.var_cnst0_dn5) * locals.var_cnst0) + (assign52430_body4_e79414 * locals.var_cnst0_dn5)) * locals.var_q_s0__blk1322) - (assign52430_body4_e79416 * locals.var_q_s0__blk1322_dn5)) / (locals.var_q_s0__blk1322 * locals.var_q_s0__blk1322)) * assign52430_body4_e79423) + (assign52430_body4_e79418 * (((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5)) - locals.var_beta_dn5))), ((((((((0.5 * locals.var_cnst0_dn6) * locals.var_cnst0) + (assign52430_body4_e79414 * locals.var_cnst0_dn6)) * locals.var_q_s0__blk1322) - (assign52430_body4_e79416 * locals.var_q_s0__blk1322_dn6)) / (locals.var_q_s0__blk1322 * locals.var_q_s0__blk1322)) * assign52430_body4_e79423) + (assign52430_body4_e79418 * (((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6)) - locals.var_beta_dn6))), ((((((((0.5 * locals.var_cnst0_dn7) * locals.var_cnst0) + (assign52430_body4_e79414 * locals.var_cnst0_dn7)) * locals.var_q_s0__blk1322) - (assign52430_body4_e79416 * locals.var_q_s0__blk1322_dn7)) / (locals.var_q_s0__blk1322 * locals.var_q_s0__blk1322)) * assign52430_body4_e79423) + (assign52430_body4_e79418 * (((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7)) - locals.var_beta_dn7))), ((((((((0.5 * locals.var_cnst0_dn8) * locals.var_cnst0) + (assign52430_body4_e79414 * locals.var_cnst0_dn8)) * locals.var_q_s0__blk1322) - (assign52430_body4_e79416 * locals.var_q_s0__blk1322_dn8)) / (locals.var_q_s0__blk1322 * locals.var_q_s0__blk1322)) * assign52430_body4_e79423) + (assign52430_body4_e79418 * (((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8)) - locals.var_beta_dn8))), ((((((((0.5 * locals.var_cnst0_dn9) * locals.var_cnst0) + (assign52430_body4_e79414 * locals.var_cnst0_dn9)) * locals.var_q_s0__blk1322) - (assign52430_body4_e79416 * locals.var_q_s0__blk1322_dn9)) / (locals.var_q_s0__blk1322 * locals.var_q_s0__blk1322)) * assign52430_body4_e79423) + (assign52430_body4_e79418 * (((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9)) - locals.var_beta_dn9))), ((((((((0.5 * locals.var_cnst0_dn10) * locals.var_cnst0) + (assign52430_body4_e79414 * locals.var_cnst0_dn10)) * locals.var_q_s0__blk1322) - (assign52430_body4_e79416 * locals.var_q_s0__blk1322_dn10)) / (locals.var_q_s0__blk1322 * locals.var_q_s0__blk1322)) * assign52430_body4_e79423) + (assign52430_body4_e79418 * (((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10)) - locals.var_beta_dn10))), ((((((((0.5 * locals.var_cnst0_dn13) * locals.var_cnst0) + (assign52430_body4_e79414 * locals.var_cnst0_dn13)) * locals.var_q_s0__blk1322) - (assign52430_body4_e79416 * locals.var_q_s0__blk1322_dn13)) / (locals.var_q_s0__blk1322 * locals.var_q_s0__blk1322)) * assign52430_body4_e79423) + (assign52430_body4_e79418 * (((locals.var_beta_dn13 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn13)) - locals.var_beta_dn13))),)
    } else {
        (locals.var_q_s0_dps__blk1125, locals.var_q_s0_dps__blk1125_dn0, locals.var_q_s0_dps__blk1125_dn2, locals.var_q_s0_dps__blk1125_dn4, locals.var_q_s0_dps__blk1125_dn5, locals.var_q_s0_dps__blk1125_dn6, locals.var_q_s0_dps__blk1125_dn7, locals.var_q_s0_dps__blk1125_dn8, locals.var_q_s0_dps__blk1125_dn9, locals.var_q_s0_dps__blk1125_dn10, locals.var_q_s0_dps__blk1125_dn13,)
    }
};
            locals.var_q_s0_dps__blk1125 = assign52430_body4_e79426;
            locals.var_q_s0_dps__blk1125_dn0 = assign52430_body4_e79426_d_n0;
            locals.var_q_s0_dps__blk1125_dn2 = assign52430_body4_e79426_d_n2;
            locals.var_q_s0_dps__blk1125_dn4 = assign52430_body4_e79426_d_n4;
            locals.var_q_s0_dps__blk1125_dn5 = assign52430_body4_e79426_d_n5;
            locals.var_q_s0_dps__blk1125_dn6 = assign52430_body4_e79426_d_n6;
            locals.var_q_s0_dps__blk1125_dn7 = assign52430_body4_e79426_d_n7;
            locals.var_q_s0_dps__blk1125_dn8 = assign52430_body4_e79426_d_n8;
            locals.var_q_s0_dps__blk1125_dn9 = assign52430_body4_e79426_d_n9;
            locals.var_q_s0_dps__blk1125_dn10 = assign52430_body4_e79426_d_n10;
            locals.var_q_s0_dps__blk1125_dn13 = assign52430_body4_e79426_d_n13;
            locals.var_q_s0_dps__blk1125_rv = 0.0;
            let (assign52430_body5_e79449, assign52430_body5_e79449_d_n0, assign52430_body5_e79449_d_n2, assign52430_body5_e79449_d_n4, assign52430_body5_e79449_d_n5, assign52430_body5_e79449_d_n6, assign52430_body5_e79449_d_n7, assign52430_body5_e79449_d_n8, assign52430_body5_e79449_d_n9, assign52430_body5_e79449_d_n10, assign52430_body5_e79449_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1330 == 0.0)) {
        let assign52430_body5_e79442: f64 = (-locals.var_beta);
        let assign52430_body5_e79445: f64 = (locals.var_ps0dep - locals.var_depvbs);
        let assign52430_body5_e79446: f64 = (assign52430_body5_e79442 * assign52430_body5_e79445);
        let assign52430_body5_e79447: f64 = (assign52430_body5_e79446).exp();
        (assign52430_body5_e79447, (assign52430_body5_e79447 * (((-locals.var_beta_dn0) * assign52430_body5_e79445) + (assign52430_body5_e79442 * (locals.var_ps0dep_dn0 - locals.var_depvbs_dn0)))), (assign52430_body5_e79447 * (((-locals.var_beta_dn2) * assign52430_body5_e79445) + (assign52430_body5_e79442 * (locals.var_ps0dep_dn2 - locals.var_depvbs_dn2)))), (assign52430_body5_e79447 * (((-locals.var_beta_dn4) * assign52430_body5_e79445) + (assign52430_body5_e79442 * (locals.var_ps0dep_dn4 - locals.var_depvbs_dn4)))), (assign52430_body5_e79447 * (((-locals.var_beta_dn5) * assign52430_body5_e79445) + (assign52430_body5_e79442 * (locals.var_ps0dep_dn5 - locals.var_depvbs_dn5)))), (assign52430_body5_e79447 * (((-locals.var_beta_dn6) * assign52430_body5_e79445) + (assign52430_body5_e79442 * (locals.var_ps0dep_dn6 - locals.var_depvbs_dn6)))), (assign52430_body5_e79447 * (((-locals.var_beta_dn7) * assign52430_body5_e79445) + (assign52430_body5_e79442 * (locals.var_ps0dep_dn7 - locals.var_depvbs_dn7)))), (assign52430_body5_e79447 * (((-locals.var_beta_dn8) * assign52430_body5_e79445) + (assign52430_body5_e79442 * (locals.var_ps0dep_dn8 - locals.var_depvbs_dn8)))), (assign52430_body5_e79447 * (((-locals.var_beta_dn9) * assign52430_body5_e79445) + (assign52430_body5_e79442 * (locals.var_ps0dep_dn9 - locals.var_depvbs_dn9)))), (assign52430_body5_e79447 * (((-locals.var_beta_dn10) * assign52430_body5_e79445) + (assign52430_body5_e79442 * (locals.var_ps0dep_dn10 - locals.var_depvbs_dn10)))), (assign52430_body5_e79447 * (((-locals.var_beta_dn13) * assign52430_body5_e79445) + (assign52430_body5_e79442 * (locals.var_ps0dep_dn13 - locals.var_depvbs_dn13)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
            locals.var_t3 = assign52430_body5_e79449;
            locals.var_t3_dn0 = assign52430_body5_e79449_d_n0;
            locals.var_t3_dn2 = assign52430_body5_e79449_d_n2;
            locals.var_t3_dn4 = assign52430_body5_e79449_d_n4;
            locals.var_t3_dn5 = assign52430_body5_e79449_d_n5;
            locals.var_t3_dn6 = assign52430_body5_e79449_d_n6;
            locals.var_t3_dn7 = assign52430_body5_e79449_d_n7;
            locals.var_t3_dn8 = assign52430_body5_e79449_d_n8;
            locals.var_t3_dn9 = assign52430_body5_e79449_d_n9;
            locals.var_t3_dn10 = assign52430_body5_e79449_d_n10;
            locals.var_t3_dn13 = assign52430_body5_e79449_d_n13;
            locals.var_t3_rv = 0.0;
            let (assign52430_body6_e79469, assign52430_body6_e79469_d_n0, assign52430_body6_e79469_d_n2, assign52430_body6_e79469_d_n4, assign52430_body6_e79469_d_n5, assign52430_body6_e79469_d_n6, assign52430_body6_e79469_d_n7, assign52430_body6_e79469_d_n8, assign52430_body6_e79469_d_n9, assign52430_body6_e79469_d_n10, assign52430_body6_e79469_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1330 == 0.0)) {
        let assign52430_body6_e79466: f64 = (locals.var_beta * locals.var_depvbs);
        let assign52430_body6_e79467: f64 = (assign52430_body6_e79466).exp();
        (assign52430_body6_e79467, (assign52430_body6_e79467 * ((locals.var_beta_dn0 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn0))), (assign52430_body6_e79467 * ((locals.var_beta_dn2 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn2))), (assign52430_body6_e79467 * ((locals.var_beta_dn4 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn4))), (assign52430_body6_e79467 * ((locals.var_beta_dn5 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn5))), (assign52430_body6_e79467 * ((locals.var_beta_dn6 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn6))), (assign52430_body6_e79467 * ((locals.var_beta_dn7 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn7))), (assign52430_body6_e79467 * ((locals.var_beta_dn8 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn8))), (assign52430_body6_e79467 * ((locals.var_beta_dn9 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn9))), (assign52430_body6_e79467 * ((locals.var_beta_dn10 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn10))), (assign52430_body6_e79467 * ((locals.var_beta_dn13 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn13))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
            locals.var_t4 = assign52430_body6_e79469;
            locals.var_t4_dn0 = assign52430_body6_e79469_d_n0;
            locals.var_t4_dn2 = assign52430_body6_e79469_d_n2;
            locals.var_t4_dn4 = assign52430_body6_e79469_d_n4;
            locals.var_t4_dn5 = assign52430_body6_e79469_d_n5;
            locals.var_t4_dn6 = assign52430_body6_e79469_d_n6;
            locals.var_t4_dn7 = assign52430_body6_e79469_d_n7;
            locals.var_t4_dn8 = assign52430_body6_e79469_d_n8;
            locals.var_t4_dn9 = assign52430_body6_e79469_d_n9;
            locals.var_t4_dn10 = assign52430_body6_e79469_d_n10;
            locals.var_t4_dn13 = assign52430_body6_e79469_d_n13;
            locals.var_t4_rv = 0.0;
            let (assign52430_body7_e79501, assign52430_body7_e79501_d_n0, assign52430_body7_e79501_d_n2, assign52430_body7_e79501_d_n4, assign52430_body7_e79501_d_n5, assign52430_body7_e79501_d_n6, assign52430_body7_e79501_d_n7, assign52430_body7_e79501_d_n8, assign52430_body7_e79501_d_n9, assign52430_body7_e79501_d_n10, assign52430_body7_e79501_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1330 == 0.0)) {
        let assign52430_body7_e79487: f64 = (locals.var_t2 - 1.0);
        let assign52430_body7_e79489: f64 = (assign52430_body7_e79487 - locals.var_t1);
        let assign52430_body7_e79493: f64 = (locals.var_t3 - locals.var_t4);
        let assign52430_body7_e79494: f64 = (locals.var_cnst1 * assign52430_body7_e79493);
        let assign52430_body7_e79495: f64 = (assign52430_body7_e79489 + assign52430_body7_e79494);
        let assign52430_body7_e79497: f64 = (assign52430_body7_e79495 + 1e-15);
        let assign52430_body7_e79498: f64 = (assign52430_body7_e79497).sqrt();
        let assign52430_body7_e79499: f64 = (locals.var_cnst0 * assign52430_body7_e79498);
        (assign52430_body7_e79499, ((locals.var_cnst0_dn0 * assign52430_body7_e79498) + (locals.var_cnst0 * (((locals.var_t2_dn0 - locals.var_t1_dn0) + ((locals.var_cnst1_dn0 * assign52430_body7_e79493) + (locals.var_cnst1 * (locals.var_t3_dn0 - locals.var_t4_dn0)))) / (2.0 * assign52430_body7_e79498)))), ((locals.var_cnst0_dn2 * assign52430_body7_e79498) + (locals.var_cnst0 * (((locals.var_t2_dn2 - locals.var_t1_dn2) + ((locals.var_cnst1_dn2 * assign52430_body7_e79493) + (locals.var_cnst1 * (locals.var_t3_dn2 - locals.var_t4_dn2)))) / (2.0 * assign52430_body7_e79498)))), ((locals.var_cnst0_dn4 * assign52430_body7_e79498) + (locals.var_cnst0 * (((locals.var_t2_dn4 - locals.var_t1_dn4) + ((locals.var_cnst1_dn4 * assign52430_body7_e79493) + (locals.var_cnst1 * (locals.var_t3_dn4 - locals.var_t4_dn4)))) / (2.0 * assign52430_body7_e79498)))), ((locals.var_cnst0_dn5 * assign52430_body7_e79498) + (locals.var_cnst0 * (((locals.var_t2_dn5 - locals.var_t1_dn5) + ((locals.var_cnst1_dn5 * assign52430_body7_e79493) + (locals.var_cnst1 * (locals.var_t3_dn5 - locals.var_t4_dn5)))) / (2.0 * assign52430_body7_e79498)))), ((locals.var_cnst0_dn6 * assign52430_body7_e79498) + (locals.var_cnst0 * (((locals.var_t2_dn6 - locals.var_t1_dn6) + ((locals.var_cnst1_dn6 * assign52430_body7_e79493) + (locals.var_cnst1 * (locals.var_t3_dn6 - locals.var_t4_dn6)))) / (2.0 * assign52430_body7_e79498)))), ((locals.var_cnst0_dn7 * assign52430_body7_e79498) + (locals.var_cnst0 * (((locals.var_t2_dn7 - locals.var_t1_dn7) + ((locals.var_cnst1_dn7 * assign52430_body7_e79493) + (locals.var_cnst1 * (locals.var_t3_dn7 - locals.var_t4_dn7)))) / (2.0 * assign52430_body7_e79498)))), ((locals.var_cnst0_dn8 * assign52430_body7_e79498) + (locals.var_cnst0 * (((locals.var_t2_dn8 - locals.var_t1_dn8) + ((locals.var_cnst1_dn8 * assign52430_body7_e79493) + (locals.var_cnst1 * (locals.var_t3_dn8 - locals.var_t4_dn8)))) / (2.0 * assign52430_body7_e79498)))), ((locals.var_cnst0_dn9 * assign52430_body7_e79498) + (locals.var_cnst0 * (((locals.var_t2_dn9 - locals.var_t1_dn9) + ((locals.var_cnst1_dn9 * assign52430_body7_e79493) + (locals.var_cnst1 * (locals.var_t3_dn9 - locals.var_t4_dn9)))) / (2.0 * assign52430_body7_e79498)))), ((locals.var_cnst0_dn10 * assign52430_body7_e79498) + (locals.var_cnst0 * (((locals.var_t2_dn10 - locals.var_t1_dn10) + ((locals.var_cnst1_dn10 * assign52430_body7_e79493) + (locals.var_cnst1 * (locals.var_t3_dn10 - locals.var_t4_dn10)))) / (2.0 * assign52430_body7_e79498)))), ((locals.var_cnst0_dn13 * assign52430_body7_e79498) + (locals.var_cnst0 * (((locals.var_t2_dn13 - locals.var_t1_dn13) + ((locals.var_cnst1_dn13 * assign52430_body7_e79493) + (locals.var_cnst1 * (locals.var_t3_dn13 - locals.var_t4_dn13)))) / (2.0 * assign52430_body7_e79498)))),)
    } else {
        (locals.var_q_s0__blk1322, locals.var_q_s0__blk1322_dn0, locals.var_q_s0__blk1322_dn2, locals.var_q_s0__blk1322_dn4, locals.var_q_s0__blk1322_dn5, locals.var_q_s0__blk1322_dn6, locals.var_q_s0__blk1322_dn7, locals.var_q_s0__blk1322_dn8, locals.var_q_s0__blk1322_dn9, locals.var_q_s0__blk1322_dn10, locals.var_q_s0__blk1322_dn13,)
    }
};
            locals.var_q_s0__blk1322 = assign52430_body7_e79501;
            locals.var_q_s0__blk1322_dn0 = assign52430_body7_e79501_d_n0;
            locals.var_q_s0__blk1322_dn2 = assign52430_body7_e79501_d_n2;
            locals.var_q_s0__blk1322_dn4 = assign52430_body7_e79501_d_n4;
            locals.var_q_s0__blk1322_dn5 = assign52430_body7_e79501_d_n5;
            locals.var_q_s0__blk1322_dn6 = assign52430_body7_e79501_d_n6;
            locals.var_q_s0__blk1322_dn7 = assign52430_body7_e79501_d_n7;
            locals.var_q_s0__blk1322_dn8 = assign52430_body7_e79501_d_n8;
            locals.var_q_s0__blk1322_dn9 = assign52430_body7_e79501_d_n9;
            locals.var_q_s0__blk1322_dn10 = assign52430_body7_e79501_d_n10;
            locals.var_q_s0__blk1322_dn13 = assign52430_body7_e79501_d_n13;
            locals.var_q_s0__blk1322_rv = 0.0;
            let (assign52430_body8_e79524, assign52430_body8_e79524_d_n0, assign52430_body8_e79524_d_n2, assign52430_body8_e79524_d_n4, assign52430_body8_e79524_d_n5, assign52430_body8_e79524_d_n6, assign52430_body8_e79524_d_n7, assign52430_body8_e79524_d_n8, assign52430_body8_e79524_d_n9, assign52430_body8_e79524_d_n10, assign52430_body8_e79524_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1330 == 0.0)) {
        let assign52430_body8_e79518: f64 = (0.5 * locals.var_cnst0);
        let assign52430_body8_e79520: f64 = (assign52430_body8_e79518 * locals.var_cnst0);
        let assign52430_body8_e79522: f64 = (assign52430_body8_e79520 / locals.var_q_s0__blk1322);
        (assign52430_body8_e79522, ((((((0.5 * locals.var_cnst0_dn0) * locals.var_cnst0) + (assign52430_body8_e79518 * locals.var_cnst0_dn0)) * locals.var_q_s0__blk1322) - (assign52430_body8_e79520 * locals.var_q_s0__blk1322_dn0)) / (locals.var_q_s0__blk1322 * locals.var_q_s0__blk1322)), ((((((0.5 * locals.var_cnst0_dn2) * locals.var_cnst0) + (assign52430_body8_e79518 * locals.var_cnst0_dn2)) * locals.var_q_s0__blk1322) - (assign52430_body8_e79520 * locals.var_q_s0__blk1322_dn2)) / (locals.var_q_s0__blk1322 * locals.var_q_s0__blk1322)), ((((((0.5 * locals.var_cnst0_dn4) * locals.var_cnst0) + (assign52430_body8_e79518 * locals.var_cnst0_dn4)) * locals.var_q_s0__blk1322) - (assign52430_body8_e79520 * locals.var_q_s0__blk1322_dn4)) / (locals.var_q_s0__blk1322 * locals.var_q_s0__blk1322)), ((((((0.5 * locals.var_cnst0_dn5) * locals.var_cnst0) + (assign52430_body8_e79518 * locals.var_cnst0_dn5)) * locals.var_q_s0__blk1322) - (assign52430_body8_e79520 * locals.var_q_s0__blk1322_dn5)) / (locals.var_q_s0__blk1322 * locals.var_q_s0__blk1322)), ((((((0.5 * locals.var_cnst0_dn6) * locals.var_cnst0) + (assign52430_body8_e79518 * locals.var_cnst0_dn6)) * locals.var_q_s0__blk1322) - (assign52430_body8_e79520 * locals.var_q_s0__blk1322_dn6)) / (locals.var_q_s0__blk1322 * locals.var_q_s0__blk1322)), ((((((0.5 * locals.var_cnst0_dn7) * locals.var_cnst0) + (assign52430_body8_e79518 * locals.var_cnst0_dn7)) * locals.var_q_s0__blk1322) - (assign52430_body8_e79520 * locals.var_q_s0__blk1322_dn7)) / (locals.var_q_s0__blk1322 * locals.var_q_s0__blk1322)), ((((((0.5 * locals.var_cnst0_dn8) * locals.var_cnst0) + (assign52430_body8_e79518 * locals.var_cnst0_dn8)) * locals.var_q_s0__blk1322) - (assign52430_body8_e79520 * locals.var_q_s0__blk1322_dn8)) / (locals.var_q_s0__blk1322 * locals.var_q_s0__blk1322)), ((((((0.5 * locals.var_cnst0_dn9) * locals.var_cnst0) + (assign52430_body8_e79518 * locals.var_cnst0_dn9)) * locals.var_q_s0__blk1322) - (assign52430_body8_e79520 * locals.var_q_s0__blk1322_dn9)) / (locals.var_q_s0__blk1322 * locals.var_q_s0__blk1322)), ((((((0.5 * locals.var_cnst0_dn10) * locals.var_cnst0) + (assign52430_body8_e79518 * locals.var_cnst0_dn10)) * locals.var_q_s0__blk1322) - (assign52430_body8_e79520 * locals.var_q_s0__blk1322_dn10)) / (locals.var_q_s0__blk1322 * locals.var_q_s0__blk1322)), ((((((0.5 * locals.var_cnst0_dn13) * locals.var_cnst0) + (assign52430_body8_e79518 * locals.var_cnst0_dn13)) * locals.var_q_s0__blk1322) - (assign52430_body8_e79520 * locals.var_q_s0__blk1322_dn13)) / (locals.var_q_s0__blk1322 * locals.var_q_s0__blk1322)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
            locals.var_t5 = assign52430_body8_e79524;
            locals.var_t5_dn0 = assign52430_body8_e79524_d_n0;
            locals.var_t5_dn2 = assign52430_body8_e79524_d_n2;
            locals.var_t5_dn4 = assign52430_body8_e79524_d_n4;
            locals.var_t5_dn5 = assign52430_body8_e79524_d_n5;
            locals.var_t5_dn6 = assign52430_body8_e79524_d_n6;
            locals.var_t5_dn7 = assign52430_body8_e79524_d_n7;
            locals.var_t5_dn8 = assign52430_body8_e79524_d_n8;
            locals.var_t5_dn9 = assign52430_body8_e79524_d_n9;
            locals.var_t5_dn10 = assign52430_body8_e79524_d_n10;
            locals.var_t5_dn13 = assign52430_body8_e79524_d_n13;
            locals.var_t5_rv = 0.0;
            let (assign52430_body9_e79554, assign52430_body9_e79554_d_n0, assign52430_body9_e79554_d_n2, assign52430_body9_e79554_d_n4, assign52430_body9_e79554_d_n5, assign52430_body9_e79554_d_n6, assign52430_body9_e79554_d_n7, assign52430_body9_e79554_d_n8, assign52430_body9_e79554_d_n9, assign52430_body9_e79554_d_n10, assign52430_body9_e79554_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1330 == 0.0)) {
        let assign52430_body9_e79542: f64 = (locals.var_beta * locals.var_t2);
        let assign52430_body9_e79544: f64 = (assign52430_body9_e79542 - locals.var_beta);
        let assign52430_body9_e79547: f64 = (-locals.var_beta);
        let assign52430_body9_e79549: f64 = (assign52430_body9_e79547 * locals.var_t3);
        let assign52430_body9_e79550: f64 = (locals.var_cnst1 * assign52430_body9_e79549);
        let assign52430_body9_e79551: f64 = (assign52430_body9_e79544 + assign52430_body9_e79550);
        let assign52430_body9_e79552: f64 = (locals.var_t5 * assign52430_body9_e79551);
        (assign52430_body9_e79552, ((locals.var_t5_dn0 * assign52430_body9_e79551) + (locals.var_t5 * ((((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0)) - locals.var_beta_dn0) + ((locals.var_cnst1_dn0 * assign52430_body9_e79549) + (locals.var_cnst1 * (((-locals.var_beta_dn0) * locals.var_t3) + (assign52430_body9_e79547 * locals.var_t3_dn0))))))), ((locals.var_t5_dn2 * assign52430_body9_e79551) + (locals.var_t5 * ((((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2)) - locals.var_beta_dn2) + ((locals.var_cnst1_dn2 * assign52430_body9_e79549) + (locals.var_cnst1 * (((-locals.var_beta_dn2) * locals.var_t3) + (assign52430_body9_e79547 * locals.var_t3_dn2))))))), ((locals.var_t5_dn4 * assign52430_body9_e79551) + (locals.var_t5 * ((((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4)) - locals.var_beta_dn4) + ((locals.var_cnst1_dn4 * assign52430_body9_e79549) + (locals.var_cnst1 * (((-locals.var_beta_dn4) * locals.var_t3) + (assign52430_body9_e79547 * locals.var_t3_dn4))))))), ((locals.var_t5_dn5 * assign52430_body9_e79551) + (locals.var_t5 * ((((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5)) - locals.var_beta_dn5) + ((locals.var_cnst1_dn5 * assign52430_body9_e79549) + (locals.var_cnst1 * (((-locals.var_beta_dn5) * locals.var_t3) + (assign52430_body9_e79547 * locals.var_t3_dn5))))))), ((locals.var_t5_dn6 * assign52430_body9_e79551) + (locals.var_t5 * ((((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6)) - locals.var_beta_dn6) + ((locals.var_cnst1_dn6 * assign52430_body9_e79549) + (locals.var_cnst1 * (((-locals.var_beta_dn6) * locals.var_t3) + (assign52430_body9_e79547 * locals.var_t3_dn6))))))), ((locals.var_t5_dn7 * assign52430_body9_e79551) + (locals.var_t5 * ((((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7)) - locals.var_beta_dn7) + ((locals.var_cnst1_dn7 * assign52430_body9_e79549) + (locals.var_cnst1 * (((-locals.var_beta_dn7) * locals.var_t3) + (assign52430_body9_e79547 * locals.var_t3_dn7))))))), ((locals.var_t5_dn8 * assign52430_body9_e79551) + (locals.var_t5 * ((((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8)) - locals.var_beta_dn8) + ((locals.var_cnst1_dn8 * assign52430_body9_e79549) + (locals.var_cnst1 * (((-locals.var_beta_dn8) * locals.var_t3) + (assign52430_body9_e79547 * locals.var_t3_dn8))))))), ((locals.var_t5_dn9 * assign52430_body9_e79551) + (locals.var_t5 * ((((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9)) - locals.var_beta_dn9) + ((locals.var_cnst1_dn9 * assign52430_body9_e79549) + (locals.var_cnst1 * (((-locals.var_beta_dn9) * locals.var_t3) + (assign52430_body9_e79547 * locals.var_t3_dn9))))))), ((locals.var_t5_dn10 * assign52430_body9_e79551) + (locals.var_t5 * ((((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10)) - locals.var_beta_dn10) + ((locals.var_cnst1_dn10 * assign52430_body9_e79549) + (locals.var_cnst1 * (((-locals.var_beta_dn10) * locals.var_t3) + (assign52430_body9_e79547 * locals.var_t3_dn10))))))), ((locals.var_t5_dn13 * assign52430_body9_e79551) + (locals.var_t5 * ((((locals.var_beta_dn13 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn13)) - locals.var_beta_dn13) + ((locals.var_cnst1_dn13 * assign52430_body9_e79549) + (locals.var_cnst1 * (((-locals.var_beta_dn13) * locals.var_t3) + (assign52430_body9_e79547 * locals.var_t3_dn13))))))),)
    } else {
        (locals.var_q_s0_dps__blk1125, locals.var_q_s0_dps__blk1125_dn0, locals.var_q_s0_dps__blk1125_dn2, locals.var_q_s0_dps__blk1125_dn4, locals.var_q_s0_dps__blk1125_dn5, locals.var_q_s0_dps__blk1125_dn6, locals.var_q_s0_dps__blk1125_dn7, locals.var_q_s0_dps__blk1125_dn8, locals.var_q_s0_dps__blk1125_dn9, locals.var_q_s0_dps__blk1125_dn10, locals.var_q_s0_dps__blk1125_dn13,)
    }
};
            locals.var_q_s0_dps__blk1125 = assign52430_body9_e79554;
            locals.var_q_s0_dps__blk1125_dn0 = assign52430_body9_e79554_d_n0;
            locals.var_q_s0_dps__blk1125_dn2 = assign52430_body9_e79554_d_n2;
            locals.var_q_s0_dps__blk1125_dn4 = assign52430_body9_e79554_d_n4;
            locals.var_q_s0_dps__blk1125_dn5 = assign52430_body9_e79554_d_n5;
            locals.var_q_s0_dps__blk1125_dn6 = assign52430_body9_e79554_d_n6;
            locals.var_q_s0_dps__blk1125_dn7 = assign52430_body9_e79554_d_n7;
            locals.var_q_s0_dps__blk1125_dn8 = assign52430_body9_e79554_d_n8;
            locals.var_q_s0_dps__blk1125_dn9 = assign52430_body9_e79554_d_n9;
            locals.var_q_s0_dps__blk1125_dn10 = assign52430_body9_e79554_d_n10;
            locals.var_q_s0_dps__blk1125_dn13 = assign52430_body9_e79554_d_n13;
            locals.var_q_s0_dps__blk1125_rv = 0.0;
            let (assign52430_body10_e79572,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_flg_conv != 0.0)) {
        let assign52430_body10_e79570: f64 = (150.0 + 1.0);
        (assign52430_body10_e79570,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign52430_body10_e79572;
            locals.var_lp_s0_rv = 0.0;
            let (assign52430_body11_e79595, assign52430_body11_e79595_d_n0, assign52430_body11_e79595_d_n2, assign52430_body11_e79595_d_n4, assign52430_body11_e79595_d_n5, assign52430_body11_e79595_d_n6, assign52430_body11_e79595_d_n7, assign52430_body11_e79595_d_n8, assign52430_body11_e79595_d_n9, assign52430_body11_e79595_d_n10, assign52430_body11_e79595_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign52430_body11_e79590: f64 = (locals.var_vgp_ws - locals.var_ps0dep);
        let assign52430_body11_e79591: f64 = (locals.var_cox * assign52430_body11_e79590);
        let assign52430_body11_e79593: f64 = (assign52430_body11_e79591 + locals.var_q_s0__blk1322);
        (assign52430_body11_e79593, (((locals.var_cox_dn0 * assign52430_body11_e79590) + (locals.var_cox * (locals.var_vgp_ws_dn0 - locals.var_ps0dep_dn0))) + locals.var_q_s0__blk1322_dn0), (((locals.var_cox_dn2 * assign52430_body11_e79590) + (locals.var_cox * (locals.var_vgp_ws_dn2 - locals.var_ps0dep_dn2))) + locals.var_q_s0__blk1322_dn2), (((locals.var_cox_dn4 * assign52430_body11_e79590) + (locals.var_cox * (locals.var_vgp_ws_dn4 - locals.var_ps0dep_dn4))) + locals.var_q_s0__blk1322_dn4), (((locals.var_cox_dn5 * assign52430_body11_e79590) + (locals.var_cox * (locals.var_vgp_ws_dn5 - locals.var_ps0dep_dn5))) + locals.var_q_s0__blk1322_dn5), (((locals.var_cox_dn6 * assign52430_body11_e79590) + (locals.var_cox * (locals.var_vgp_ws_dn6 - locals.var_ps0dep_dn6))) + locals.var_q_s0__blk1322_dn6), (((locals.var_cox_dn7 * assign52430_body11_e79590) + (locals.var_cox * (locals.var_vgp_ws_dn7 - locals.var_ps0dep_dn7))) + locals.var_q_s0__blk1322_dn7), (((locals.var_cox_dn8 * assign52430_body11_e79590) + (locals.var_cox * (locals.var_vgp_ws_dn8 - locals.var_ps0dep_dn8))) + locals.var_q_s0__blk1322_dn8), (((locals.var_cox_dn9 * assign52430_body11_e79590) + (locals.var_cox * (locals.var_vgp_ws_dn9 - locals.var_ps0dep_dn9))) + locals.var_q_s0__blk1322_dn9), (((locals.var_cox_dn10 * assign52430_body11_e79590) + (locals.var_cox * (locals.var_vgp_ws_dn10 - locals.var_ps0dep_dn10))) + locals.var_q_s0__blk1322_dn10), (((locals.var_cox_dn13 * assign52430_body11_e79590) + (locals.var_cox * (locals.var_vgp_ws_dn13 - locals.var_ps0dep_dn13))) + locals.var_q_s0__blk1322_dn13),)
    } else {
        (locals.var_pf1__blk1100, locals.var_pf1__blk1100_dn0, locals.var_pf1__blk1100_dn2, locals.var_pf1__blk1100_dn4, locals.var_pf1__blk1100_dn5, locals.var_pf1__blk1100_dn6, locals.var_pf1__blk1100_dn7, locals.var_pf1__blk1100_dn8, locals.var_pf1__blk1100_dn9, locals.var_pf1__blk1100_dn10, locals.var_pf1__blk1100_dn13,)
    }
};
            locals.var_pf1__blk1100 = assign52430_body11_e79595;
            locals.var_pf1__blk1100_dn0 = assign52430_body11_e79595_d_n0;
            locals.var_pf1__blk1100_dn2 = assign52430_body11_e79595_d_n2;
            locals.var_pf1__blk1100_dn4 = assign52430_body11_e79595_d_n4;
            locals.var_pf1__blk1100_dn5 = assign52430_body11_e79595_d_n5;
            locals.var_pf1__blk1100_dn6 = assign52430_body11_e79595_d_n6;
            locals.var_pf1__blk1100_dn7 = assign52430_body11_e79595_d_n7;
            locals.var_pf1__blk1100_dn8 = assign52430_body11_e79595_d_n8;
            locals.var_pf1__blk1100_dn9 = assign52430_body11_e79595_d_n9;
            locals.var_pf1__blk1100_dn10 = assign52430_body11_e79595_d_n10;
            locals.var_pf1__blk1100_dn13 = assign52430_body11_e79595_d_n13;
            locals.var_pf1__blk1100_rv = 0.0;
            let (assign52430_body12_e79615, assign52430_body12_e79615_d_n0, assign52430_body12_e79615_d_n2, assign52430_body12_e79615_d_n4, assign52430_body12_e79615_d_n5, assign52430_body12_e79615_d_n6, assign52430_body12_e79615_d_n7, assign52430_body12_e79615_d_n8, assign52430_body12_e79615_d_n9, assign52430_body12_e79615_d_n10, assign52430_body12_e79615_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign52430_body12_e79611: f64 = (-locals.var_cox);
        let assign52430_body12_e79613: f64 = (assign52430_body12_e79611 + locals.var_q_s0_dps__blk1125);
        (assign52430_body12_e79613, ((-locals.var_cox_dn0) + locals.var_q_s0_dps__blk1125_dn0), ((-locals.var_cox_dn2) + locals.var_q_s0_dps__blk1125_dn2), ((-locals.var_cox_dn4) + locals.var_q_s0_dps__blk1125_dn4), ((-locals.var_cox_dn5) + locals.var_q_s0_dps__blk1125_dn5), ((-locals.var_cox_dn6) + locals.var_q_s0_dps__blk1125_dn6), ((-locals.var_cox_dn7) + locals.var_q_s0_dps__blk1125_dn7), ((-locals.var_cox_dn8) + locals.var_q_s0_dps__blk1125_dn8), ((-locals.var_cox_dn9) + locals.var_q_s0_dps__blk1125_dn9), ((-locals.var_cox_dn10) + locals.var_q_s0_dps__blk1125_dn10), ((-locals.var_cox_dn13) + locals.var_q_s0_dps__blk1125_dn13),)
    } else {
        (locals.var_pf11__blk1101, locals.var_pf11__blk1101_dn0, locals.var_pf11__blk1101_dn2, locals.var_pf11__blk1101_dn4, locals.var_pf11__blk1101_dn5, locals.var_pf11__blk1101_dn6, locals.var_pf11__blk1101_dn7, locals.var_pf11__blk1101_dn8, locals.var_pf11__blk1101_dn9, locals.var_pf11__blk1101_dn10, locals.var_pf11__blk1101_dn13,)
    }
};
            locals.var_pf11__blk1101 = assign52430_body12_e79615;
            locals.var_pf11__blk1101_dn0 = assign52430_body12_e79615_d_n0;
            locals.var_pf11__blk1101_dn2 = assign52430_body12_e79615_d_n2;
            locals.var_pf11__blk1101_dn4 = assign52430_body12_e79615_d_n4;
            locals.var_pf11__blk1101_dn5 = assign52430_body12_e79615_d_n5;
            locals.var_pf11__blk1101_dn6 = assign52430_body12_e79615_d_n6;
            locals.var_pf11__blk1101_dn7 = assign52430_body12_e79615_d_n7;
            locals.var_pf11__blk1101_dn8 = assign52430_body12_e79615_d_n8;
            locals.var_pf11__blk1101_dn9 = assign52430_body12_e79615_d_n9;
            locals.var_pf11__blk1101_dn10 = assign52430_body12_e79615_d_n10;
            locals.var_pf11__blk1101_dn13 = assign52430_body12_e79615_d_n13;
            locals.var_pf11__blk1101_rv = 0.0;
            let (assign52430_body13_e79635, assign52430_body13_e79635_d_n0, assign52430_body13_e79635_d_n2, assign52430_body13_e79635_d_n4, assign52430_body13_e79635_d_n5, assign52430_body13_e79635_d_n6, assign52430_body13_e79635_d_n7, assign52430_body13_e79635_d_n8, assign52430_body13_e79635_d_n9, assign52430_body13_e79635_d_n10, assign52430_body13_e79635_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign52430_body13_e79631: f64 = (-locals.var_pf1__blk1100);
        let assign52430_body13_e79633: f64 = (assign52430_body13_e79631 / locals.var_pf11__blk1101);
        (assign52430_body13_e79633, ((((-locals.var_pf1__blk1100_dn0) * locals.var_pf11__blk1101) - (assign52430_body13_e79631 * locals.var_pf11__blk1101_dn0)) / (locals.var_pf11__blk1101 * locals.var_pf11__blk1101)), ((((-locals.var_pf1__blk1100_dn2) * locals.var_pf11__blk1101) - (assign52430_body13_e79631 * locals.var_pf11__blk1101_dn2)) / (locals.var_pf11__blk1101 * locals.var_pf11__blk1101)), ((((-locals.var_pf1__blk1100_dn4) * locals.var_pf11__blk1101) - (assign52430_body13_e79631 * locals.var_pf11__blk1101_dn4)) / (locals.var_pf11__blk1101 * locals.var_pf11__blk1101)), ((((-locals.var_pf1__blk1100_dn5) * locals.var_pf11__blk1101) - (assign52430_body13_e79631 * locals.var_pf11__blk1101_dn5)) / (locals.var_pf11__blk1101 * locals.var_pf11__blk1101)), ((((-locals.var_pf1__blk1100_dn6) * locals.var_pf11__blk1101) - (assign52430_body13_e79631 * locals.var_pf11__blk1101_dn6)) / (locals.var_pf11__blk1101 * locals.var_pf11__blk1101)), ((((-locals.var_pf1__blk1100_dn7) * locals.var_pf11__blk1101) - (assign52430_body13_e79631 * locals.var_pf11__blk1101_dn7)) / (locals.var_pf11__blk1101 * locals.var_pf11__blk1101)), ((((-locals.var_pf1__blk1100_dn8) * locals.var_pf11__blk1101) - (assign52430_body13_e79631 * locals.var_pf11__blk1101_dn8)) / (locals.var_pf11__blk1101 * locals.var_pf11__blk1101)), ((((-locals.var_pf1__blk1100_dn9) * locals.var_pf11__blk1101) - (assign52430_body13_e79631 * locals.var_pf11__blk1101_dn9)) / (locals.var_pf11__blk1101 * locals.var_pf11__blk1101)), ((((-locals.var_pf1__blk1100_dn10) * locals.var_pf11__blk1101) - (assign52430_body13_e79631 * locals.var_pf11__blk1101_dn10)) / (locals.var_pf11__blk1101 * locals.var_pf11__blk1101)), ((((-locals.var_pf1__blk1100_dn13) * locals.var_pf11__blk1101) - (assign52430_body13_e79631 * locals.var_pf11__blk1101_dn13)) / (locals.var_pf11__blk1101 * locals.var_pf11__blk1101)),)
    } else {
        (locals.var_dps__blk1112, locals.var_dps__blk1112_dn0, locals.var_dps__blk1112_dn2, locals.var_dps__blk1112_dn4, locals.var_dps__blk1112_dn5, locals.var_dps__blk1112_dn6, locals.var_dps__blk1112_dn7, locals.var_dps__blk1112_dn8, locals.var_dps__blk1112_dn9, locals.var_dps__blk1112_dn10, locals.var_dps__blk1112_dn13,)
    }
};
            locals.var_dps__blk1112 = assign52430_body13_e79635;
            locals.var_dps__blk1112_dn0 = assign52430_body13_e79635_d_n0;
            locals.var_dps__blk1112_dn2 = assign52430_body13_e79635_d_n2;
            locals.var_dps__blk1112_dn4 = assign52430_body13_e79635_d_n4;
            locals.var_dps__blk1112_dn5 = assign52430_body13_e79635_d_n5;
            locals.var_dps__blk1112_dn6 = assign52430_body13_e79635_d_n6;
            locals.var_dps__blk1112_dn7 = assign52430_body13_e79635_d_n7;
            locals.var_dps__blk1112_dn8 = assign52430_body13_e79635_d_n8;
            locals.var_dps__blk1112_dn9 = assign52430_body13_e79635_d_n9;
            locals.var_dps__blk1112_dn10 = assign52430_body13_e79635_d_n10;
            locals.var_dps__blk1112_dn13 = assign52430_body13_e79635_d_n13;
            locals.var_dps__blk1112_rv = 0.0;
            let assign52430_body14_e79637: f64 = (locals.var_dps__blk1112).abs();
            let assign52430_body14_e79640: f64 = (1e-10 * 100.0);
            let assign52430_body14_e79641: f64 = if assign52430_body14_e79637 < assign52430_body14_e79640 { 1.0 } else { 0.0 };
            locals.var_guard1331 = assign52430_body14_e79641;
            locals.var_guard1331_rv = 0.0;
            let (assign52430_body15_e79660,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_flg_conv == 0.0)) && (locals.var_guard1331 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign52430_body15_e79660;
            locals.var_flg_conv_rv = 0.0;
            let assign52430_body16_e79663: f64 = if locals.var_dps__blk1112 > 0.1 { 1.0 } else { 0.0 };
            locals.var_guard1332 = assign52430_body16_e79663;
            locals.var_guard1332_rv = 0.0;
            let (assign52430_body17_e79685, assign52430_body17_e79685_d_n0, assign52430_body17_e79685_d_n2, assign52430_body17_e79685_d_n4, assign52430_body17_e79685_d_n5, assign52430_body17_e79685_d_n6, assign52430_body17_e79685_d_n7, assign52430_body17_e79685_d_n8, assign52430_body17_e79685_d_n9, assign52430_body17_e79685_d_n10, assign52430_body17_e79685_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_flg_conv == 0.0)) && (locals.var_guard1331 == 0.0)) && (locals.var_guard1332 != 0.0)) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps__blk1112, locals.var_dps__blk1112_dn0, locals.var_dps__blk1112_dn2, locals.var_dps__blk1112_dn4, locals.var_dps__blk1112_dn5, locals.var_dps__blk1112_dn6, locals.var_dps__blk1112_dn7, locals.var_dps__blk1112_dn8, locals.var_dps__blk1112_dn9, locals.var_dps__blk1112_dn10, locals.var_dps__blk1112_dn13,)
    }
};
            locals.var_dps__blk1112 = assign52430_body17_e79685;
            locals.var_dps__blk1112_dn0 = assign52430_body17_e79685_d_n0;
            locals.var_dps__blk1112_dn2 = assign52430_body17_e79685_d_n2;
            locals.var_dps__blk1112_dn4 = assign52430_body17_e79685_d_n4;
            locals.var_dps__blk1112_dn5 = assign52430_body17_e79685_d_n5;
            locals.var_dps__blk1112_dn6 = assign52430_body17_e79685_d_n6;
            locals.var_dps__blk1112_dn7 = assign52430_body17_e79685_d_n7;
            locals.var_dps__blk1112_dn8 = assign52430_body17_e79685_d_n8;
            locals.var_dps__blk1112_dn9 = assign52430_body17_e79685_d_n9;
            locals.var_dps__blk1112_dn10 = assign52430_body17_e79685_d_n10;
            locals.var_dps__blk1112_dn13 = assign52430_body17_e79685_d_n13;
            locals.var_dps__blk1112_rv = 0.0;
            let assign52430_body18_e79688: f64 = (-0.1);
            let assign52430_body18_e79689: f64 = if locals.var_dps__blk1112 < assign52430_body18_e79688 { 1.0 } else { 0.0 };
            locals.var_guard1333 = assign52430_body18_e79689;
            locals.var_guard1333_rv = 0.0;
            let (assign52430_body19_e79715, assign52430_body19_e79715_d_n0, assign52430_body19_e79715_d_n2, assign52430_body19_e79715_d_n4, assign52430_body19_e79715_d_n5, assign52430_body19_e79715_d_n6, assign52430_body19_e79715_d_n7, assign52430_body19_e79715_d_n8, assign52430_body19_e79715_d_n9, assign52430_body19_e79715_d_n10, assign52430_body19_e79715_d_n13,) = {
    if (((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_flg_conv == 0.0)) && (locals.var_guard1331 == 0.0)) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1333 != 0.0)) {
        let assign52430_body19_e79713: f64 = (-0.1);
        (assign52430_body19_e79713, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps__blk1112, locals.var_dps__blk1112_dn0, locals.var_dps__blk1112_dn2, locals.var_dps__blk1112_dn4, locals.var_dps__blk1112_dn5, locals.var_dps__blk1112_dn6, locals.var_dps__blk1112_dn7, locals.var_dps__blk1112_dn8, locals.var_dps__blk1112_dn9, locals.var_dps__blk1112_dn10, locals.var_dps__blk1112_dn13,)
    }
};
            locals.var_dps__blk1112 = assign52430_body19_e79715;
            locals.var_dps__blk1112_dn0 = assign52430_body19_e79715_d_n0;
            locals.var_dps__blk1112_dn2 = assign52430_body19_e79715_d_n2;
            locals.var_dps__blk1112_dn4 = assign52430_body19_e79715_d_n4;
            locals.var_dps__blk1112_dn5 = assign52430_body19_e79715_d_n5;
            locals.var_dps__blk1112_dn6 = assign52430_body19_e79715_d_n6;
            locals.var_dps__blk1112_dn7 = assign52430_body19_e79715_d_n7;
            locals.var_dps__blk1112_dn8 = assign52430_body19_e79715_d_n8;
            locals.var_dps__blk1112_dn9 = assign52430_body19_e79715_d_n9;
            locals.var_dps__blk1112_dn10 = assign52430_body19_e79715_d_n10;
            locals.var_dps__blk1112_dn13 = assign52430_body19_e79715_d_n13;
            locals.var_dps__blk1112_rv = 0.0;
            let (assign52430_body20_e79734, assign52430_body20_e79734_d_n0, assign52430_body20_e79734_d_n2, assign52430_body20_e79734_d_n4, assign52430_body20_e79734_d_n5, assign52430_body20_e79734_d_n6, assign52430_body20_e79734_d_n7, assign52430_body20_e79734_d_n8, assign52430_body20_e79734_d_n9, assign52430_body20_e79734_d_n10, assign52430_body20_e79734_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign52430_body20_e79732: f64 = (locals.var_ps0dep + locals.var_dps__blk1112);
        (assign52430_body20_e79732, (locals.var_ps0dep_dn0 + locals.var_dps__blk1112_dn0), (locals.var_ps0dep_dn2 + locals.var_dps__blk1112_dn2), (locals.var_ps0dep_dn4 + locals.var_dps__blk1112_dn4), (locals.var_ps0dep_dn5 + locals.var_dps__blk1112_dn5), (locals.var_ps0dep_dn6 + locals.var_dps__blk1112_dn6), (locals.var_ps0dep_dn7 + locals.var_dps__blk1112_dn7), (locals.var_ps0dep_dn8 + locals.var_dps__blk1112_dn8), (locals.var_ps0dep_dn9 + locals.var_dps__blk1112_dn9), (locals.var_ps0dep_dn10 + locals.var_dps__blk1112_dn10), (locals.var_ps0dep_dn13 + locals.var_dps__blk1112_dn13),)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn13,)
    }
};
            locals.var_ps0dep = assign52430_body20_e79734;
            locals.var_ps0dep_dn0 = assign52430_body20_e79734_d_n0;
            locals.var_ps0dep_dn2 = assign52430_body20_e79734_d_n2;
            locals.var_ps0dep_dn4 = assign52430_body20_e79734_d_n4;
            locals.var_ps0dep_dn5 = assign52430_body20_e79734_d_n5;
            locals.var_ps0dep_dn6 = assign52430_body20_e79734_d_n6;
            locals.var_ps0dep_dn7 = assign52430_body20_e79734_d_n7;
            locals.var_ps0dep_dn8 = assign52430_body20_e79734_d_n8;
            locals.var_ps0dep_dn9 = assign52430_body20_e79734_d_n9;
            locals.var_ps0dep_dn10 = assign52430_body20_e79734_d_n10;
            locals.var_ps0dep_dn13 = assign52430_body20_e79734_d_n13;
            locals.var_ps0dep_rv = 0.0;
            let (assign52430_body21_e79750,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) {
        let assign52430_body21_e79748: f64 = (locals.var_lp_s0 + 1.0);
        (assign52430_body21_e79748,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign52430_body21_e79750;
            locals.var_lp_s0_rv = 0.0;
        }

        let assign52450_e79756: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1335 = assign52450_e79756;
        locals.var_guard1335_rv = 0.0;

        let (assign52460_e79772, assign52460_e79772_d_n0, assign52460_e79772_d_n2, assign52460_e79772_d_n4, assign52460_e79772_d_n5, assign52460_e79772_d_n6, assign52460_e79772_d_n7, assign52460_e79772_d_n8, assign52460_e79772_d_n9, assign52460_e79772_d_n10, assign52460_e79772_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 != 0.0)) {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn13,)
    } else {
        (locals.var_ps0dep0, locals.var_ps0dep0_dn0, locals.var_ps0dep0_dn2, locals.var_ps0dep0_dn4, locals.var_ps0dep0_dn5, locals.var_ps0dep0_dn6, locals.var_ps0dep0_dn7, locals.var_ps0dep0_dn8, locals.var_ps0dep0_dn9, locals.var_ps0dep0_dn10, locals.var_ps0dep0_dn13,)
    }
};
        locals.var_ps0dep0 = assign52460_e79772;
        locals.var_ps0dep0_dn0 = assign52460_e79772_d_n0;
        locals.var_ps0dep0_dn2 = assign52460_e79772_d_n2;
        locals.var_ps0dep0_dn4 = assign52460_e79772_d_n4;
        locals.var_ps0dep0_dn5 = assign52460_e79772_d_n5;
        locals.var_ps0dep0_dn6 = assign52460_e79772_d_n6;
        locals.var_ps0dep0_dn7 = assign52460_e79772_d_n7;
        locals.var_ps0dep0_dn8 = assign52460_e79772_d_n8;
        locals.var_ps0dep0_dn9 = assign52460_e79772_d_n9;
        locals.var_ps0dep0_dn10 = assign52460_e79772_d_n10;
        locals.var_ps0dep0_dn13 = assign52460_e79772_d_n13;
        locals.var_ps0dep0_rv = 0.0;

        let assign52470_e79776: f64 = (locals.var_ps0dep0 + 0.2);
        let assign52470_e79781: f64 = if ((locals.var_ps0dep < assign52470_e79776) && (0.2 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1336 = assign52470_e79781;
        locals.var_guard1336_rv = 0.0;

        let (assign52480_e79804, assign52480_e79804_d_n0, assign52480_e79804_d_n2, assign52480_e79804_d_n4, assign52480_e79804_d_n5, assign52480_e79804_d_n6, assign52480_e79804_d_n7, assign52480_e79804_d_n8, assign52480_e79804_d_n9, assign52480_e79804_d_n10, assign52480_e79804_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) {
        let assign52480_e79800: f64 = (locals.var_ps0dep0 + 0.2);
        let assign52480_e79802: f64 = (assign52480_e79800 - locals.var_ps0dep);
        (assign52480_e79802, (locals.var_ps0dep0_dn0 - locals.var_ps0dep_dn0), (locals.var_ps0dep0_dn2 - locals.var_ps0dep_dn2), (locals.var_ps0dep0_dn4 - locals.var_ps0dep_dn4), (locals.var_ps0dep0_dn5 - locals.var_ps0dep_dn5), (locals.var_ps0dep0_dn6 - locals.var_ps0dep_dn6), (locals.var_ps0dep0_dn7 - locals.var_ps0dep_dn7), (locals.var_ps0dep0_dn8 - locals.var_ps0dep_dn8), (locals.var_ps0dep0_dn9 - locals.var_ps0dep_dn9), (locals.var_ps0dep0_dn10 - locals.var_ps0dep_dn10), (locals.var_ps0dep0_dn13 - locals.var_ps0dep_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign52480_e79804;
        locals.var_tmf1_dn0 = assign52480_e79804_d_n0;
        locals.var_tmf1_dn2 = assign52480_e79804_d_n2;
        locals.var_tmf1_dn4 = assign52480_e79804_d_n4;
        locals.var_tmf1_dn5 = assign52480_e79804_d_n5;
        locals.var_tmf1_dn6 = assign52480_e79804_d_n6;
        locals.var_tmf1_dn7 = assign52480_e79804_d_n7;
        locals.var_tmf1_dn8 = assign52480_e79804_d_n8;
        locals.var_tmf1_dn9 = assign52480_e79804_d_n9;
        locals.var_tmf1_dn10 = assign52480_e79804_d_n10;
        locals.var_tmf1_dn13 = assign52480_e79804_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign52490_e79825, assign52490_e79825_d_n0, assign52490_e79825_d_n2, assign52490_e79825_d_n4, assign52490_e79825_d_n5, assign52490_e79825_d_n6, assign52490_e79825_d_n7, assign52490_e79825_d_n8, assign52490_e79825_d_n9, assign52490_e79825_d_n10, assign52490_e79825_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) {
        let assign52490_e79823: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign52490_e79823, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign52490_e79825;
        locals.var_x2_dn0 = assign52490_e79825_d_n0;
        locals.var_x2_dn2 = assign52490_e79825_d_n2;
        locals.var_x2_dn4 = assign52490_e79825_d_n4;
        locals.var_x2_dn5 = assign52490_e79825_d_n5;
        locals.var_x2_dn6 = assign52490_e79825_d_n6;
        locals.var_x2_dn7 = assign52490_e79825_d_n7;
        locals.var_x2_dn8 = assign52490_e79825_d_n8;
        locals.var_x2_dn9 = assign52490_e79825_d_n9;
        locals.var_x2_dn10 = assign52490_e79825_d_n10;
        locals.var_x2_dn13 = assign52490_e79825_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign52500_e79846, assign52500_e79846_d_n0, assign52500_e79846_d_n2, assign52500_e79846_d_n4, assign52500_e79846_d_n5, assign52500_e79846_d_n6, assign52500_e79846_d_n7, assign52500_e79846_d_n8, assign52500_e79846_d_n9, assign52500_e79846_d_n10, assign52500_e79846_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) {
        let assign52500_e79844: f64 = (0.2 * 0.2);
        (assign52500_e79844, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign52500_e79846;
        locals.var_xmax2_dn0 = assign52500_e79846_d_n0;
        locals.var_xmax2_dn2 = assign52500_e79846_d_n2;
        locals.var_xmax2_dn4 = assign52500_e79846_d_n4;
        locals.var_xmax2_dn5 = assign52500_e79846_d_n5;
        locals.var_xmax2_dn6 = assign52500_e79846_d_n6;
        locals.var_xmax2_dn7 = assign52500_e79846_d_n7;
        locals.var_xmax2_dn8 = assign52500_e79846_d_n8;
        locals.var_xmax2_dn9 = assign52500_e79846_d_n9;
        locals.var_xmax2_dn10 = assign52500_e79846_d_n10;
        locals.var_xmax2_dn13 = assign52500_e79846_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign52510_e79865, assign52510_e79865_d_n0, assign52510_e79865_d_n2, assign52510_e79865_d_n4, assign52510_e79865_d_n5, assign52510_e79865_d_n6, assign52510_e79865_d_n7, assign52510_e79865_d_n8, assign52510_e79865_d_n9, assign52510_e79865_d_n10, assign52510_e79865_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign52510_e79865;
        locals.var_xp_dn0 = assign52510_e79865_d_n0;
        locals.var_xp_dn2 = assign52510_e79865_d_n2;
        locals.var_xp_dn4 = assign52510_e79865_d_n4;
        locals.var_xp_dn5 = assign52510_e79865_d_n5;
        locals.var_xp_dn6 = assign52510_e79865_d_n6;
        locals.var_xp_dn7 = assign52510_e79865_d_n7;
        locals.var_xp_dn8 = assign52510_e79865_d_n8;
        locals.var_xp_dn9 = assign52510_e79865_d_n9;
        locals.var_xp_dn10 = assign52510_e79865_d_n10;
        locals.var_xp_dn13 = assign52510_e79865_d_n13;
        locals.var_xp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_183(
        locals: &mut StampLocals,
    ) {
        let (assign52520_e79884, assign52520_e79884_d_n0, assign52520_e79884_d_n2, assign52520_e79884_d_n4, assign52520_e79884_d_n5, assign52520_e79884_d_n6, assign52520_e79884_d_n7, assign52520_e79884_d_n8, assign52520_e79884_d_n9, assign52520_e79884_d_n10, assign52520_e79884_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign52520_e79884;
        locals.var_xmp_dn0 = assign52520_e79884_d_n0;
        locals.var_xmp_dn2 = assign52520_e79884_d_n2;
        locals.var_xmp_dn4 = assign52520_e79884_d_n4;
        locals.var_xmp_dn5 = assign52520_e79884_d_n5;
        locals.var_xmp_dn6 = assign52520_e79884_d_n6;
        locals.var_xmp_dn7 = assign52520_e79884_d_n7;
        locals.var_xmp_dn8 = assign52520_e79884_d_n8;
        locals.var_xmp_dn9 = assign52520_e79884_d_n9;
        locals.var_xmp_dn10 = assign52520_e79884_d_n10;
        locals.var_xmp_dn13 = assign52520_e79884_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign52530_e79903,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign52530_e79903;
        locals.var_m0_rv = 0.0;

        let (assign52540_e79922,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign52540_e79922;
        locals.var_mm_rv = 0.0;

        let (assign52550_e79941, assign52550_e79941_d_n0, assign52550_e79941_d_n2, assign52550_e79941_d_n4, assign52550_e79941_d_n5, assign52550_e79941_d_n6, assign52550_e79941_d_n7, assign52550_e79941_d_n8, assign52550_e79941_d_n9, assign52550_e79941_d_n10, assign52550_e79941_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign52550_e79941;
        locals.var_arg_dn0 = assign52550_e79941_d_n0;
        locals.var_arg_dn2 = assign52550_e79941_d_n2;
        locals.var_arg_dn4 = assign52550_e79941_d_n4;
        locals.var_arg_dn5 = assign52550_e79941_d_n5;
        locals.var_arg_dn6 = assign52550_e79941_d_n6;
        locals.var_arg_dn7 = assign52550_e79941_d_n7;
        locals.var_arg_dn8 = assign52550_e79941_d_n8;
        locals.var_arg_dn9 = assign52550_e79941_d_n9;
        locals.var_arg_dn10 = assign52550_e79941_d_n10;
        locals.var_arg_dn13 = assign52550_e79941_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign52560_e79960, assign52560_e79960_d_n0, assign52560_e79960_d_n2, assign52560_e79960_d_n4, assign52560_e79960_d_n5, assign52560_e79960_d_n6, assign52560_e79960_d_n7, assign52560_e79960_d_n8, assign52560_e79960_d_n9, assign52560_e79960_d_n10, assign52560_e79960_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign52560_e79960;
        locals.var_dnm_dn0 = assign52560_e79960_d_n0;
        locals.var_dnm_dn2 = assign52560_e79960_d_n2;
        locals.var_dnm_dn4 = assign52560_e79960_d_n4;
        locals.var_dnm_dn5 = assign52560_e79960_d_n5;
        locals.var_dnm_dn6 = assign52560_e79960_d_n6;
        locals.var_dnm_dn7 = assign52560_e79960_d_n7;
        locals.var_dnm_dn8 = assign52560_e79960_d_n8;
        locals.var_dnm_dn9 = assign52560_e79960_d_n9;
        locals.var_dnm_dn10 = assign52560_e79960_d_n10;
        locals.var_dnm_dn13 = assign52560_e79960_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign52570_e79981, assign52570_e79981_d_n0, assign52570_e79981_d_n2, assign52570_e79981_d_n4, assign52570_e79981_d_n5, assign52570_e79981_d_n6, assign52570_e79981_d_n7, assign52570_e79981_d_n8, assign52570_e79981_d_n9, assign52570_e79981_d_n10, assign52570_e79981_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) {
        let assign52570_e79979: f64 = (locals.var_xp * locals.var_x2);
        (assign52570_e79979, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign52570_e79981;
        locals.var_xp_dn0 = assign52570_e79981_d_n0;
        locals.var_xp_dn2 = assign52570_e79981_d_n2;
        locals.var_xp_dn4 = assign52570_e79981_d_n4;
        locals.var_xp_dn5 = assign52570_e79981_d_n5;
        locals.var_xp_dn6 = assign52570_e79981_d_n6;
        locals.var_xp_dn7 = assign52570_e79981_d_n7;
        locals.var_xp_dn8 = assign52570_e79981_d_n8;
        locals.var_xp_dn9 = assign52570_e79981_d_n9;
        locals.var_xp_dn10 = assign52570_e79981_d_n10;
        locals.var_xp_dn13 = assign52570_e79981_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign52580_e80002, assign52580_e80002_d_n0, assign52580_e80002_d_n2, assign52580_e80002_d_n4, assign52580_e80002_d_n5, assign52580_e80002_d_n6, assign52580_e80002_d_n7, assign52580_e80002_d_n8, assign52580_e80002_d_n9, assign52580_e80002_d_n10, assign52580_e80002_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) {
        let assign52580_e80000: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign52580_e80000, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign52580_e80002;
        locals.var_xmp_dn0 = assign52580_e80002_d_n0;
        locals.var_xmp_dn2 = assign52580_e80002_d_n2;
        locals.var_xmp_dn4 = assign52580_e80002_d_n4;
        locals.var_xmp_dn5 = assign52580_e80002_d_n5;
        locals.var_xmp_dn6 = assign52580_e80002_d_n6;
        locals.var_xmp_dn7 = assign52580_e80002_d_n7;
        locals.var_xmp_dn8 = assign52580_e80002_d_n8;
        locals.var_xmp_dn9 = assign52580_e80002_d_n9;
        locals.var_xmp_dn10 = assign52580_e80002_d_n10;
        locals.var_xmp_dn13 = assign52580_e80002_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign52590_e80023, assign52590_e80023_d_n0, assign52590_e80023_d_n2, assign52590_e80023_d_n4, assign52590_e80023_d_n5, assign52590_e80023_d_n6, assign52590_e80023_d_n7, assign52590_e80023_d_n8, assign52590_e80023_d_n9, assign52590_e80023_d_n10, assign52590_e80023_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) {
        let assign52590_e80021: f64 = (locals.var_xp * locals.var_x2);
        (assign52590_e80021, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign52590_e80023;
        locals.var_xp_dn0 = assign52590_e80023_d_n0;
        locals.var_xp_dn2 = assign52590_e80023_d_n2;
        locals.var_xp_dn4 = assign52590_e80023_d_n4;
        locals.var_xp_dn5 = assign52590_e80023_d_n5;
        locals.var_xp_dn6 = assign52590_e80023_d_n6;
        locals.var_xp_dn7 = assign52590_e80023_d_n7;
        locals.var_xp_dn8 = assign52590_e80023_d_n8;
        locals.var_xp_dn9 = assign52590_e80023_d_n9;
        locals.var_xp_dn10 = assign52590_e80023_d_n10;
        locals.var_xp_dn13 = assign52590_e80023_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign52600_e80044, assign52600_e80044_d_n0, assign52600_e80044_d_n2, assign52600_e80044_d_n4, assign52600_e80044_d_n5, assign52600_e80044_d_n6, assign52600_e80044_d_n7, assign52600_e80044_d_n8, assign52600_e80044_d_n9, assign52600_e80044_d_n10, assign52600_e80044_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) {
        let assign52600_e80042: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign52600_e80042, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign52600_e80044;
        locals.var_xmp_dn0 = assign52600_e80044_d_n0;
        locals.var_xmp_dn2 = assign52600_e80044_d_n2;
        locals.var_xmp_dn4 = assign52600_e80044_d_n4;
        locals.var_xmp_dn5 = assign52600_e80044_d_n5;
        locals.var_xmp_dn6 = assign52600_e80044_d_n6;
        locals.var_xmp_dn7 = assign52600_e80044_d_n7;
        locals.var_xmp_dn8 = assign52600_e80044_d_n8;
        locals.var_xmp_dn9 = assign52600_e80044_d_n9;
        locals.var_xmp_dn10 = assign52600_e80044_d_n10;
        locals.var_xmp_dn13 = assign52600_e80044_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign52610_e80065, assign52610_e80065_d_n0, assign52610_e80065_d_n2, assign52610_e80065_d_n4, assign52610_e80065_d_n5, assign52610_e80065_d_n6, assign52610_e80065_d_n7, assign52610_e80065_d_n8, assign52610_e80065_d_n9, assign52610_e80065_d_n10, assign52610_e80065_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) {
        let assign52610_e80063: f64 = (locals.var_xp + locals.var_xmp);
        (assign52610_e80063, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign52610_e80065;
        locals.var_arg_dn0 = assign52610_e80065_d_n0;
        locals.var_arg_dn2 = assign52610_e80065_d_n2;
        locals.var_arg_dn4 = assign52610_e80065_d_n4;
        locals.var_arg_dn5 = assign52610_e80065_d_n5;
        locals.var_arg_dn6 = assign52610_e80065_d_n6;
        locals.var_arg_dn7 = assign52610_e80065_d_n7;
        locals.var_arg_dn8 = assign52610_e80065_d_n8;
        locals.var_arg_dn9 = assign52610_e80065_d_n9;
        locals.var_arg_dn10 = assign52610_e80065_d_n10;
        locals.var_arg_dn13 = assign52610_e80065_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign52620_e80084, assign52620_e80084_d_n0, assign52620_e80084_d_n2, assign52620_e80084_d_n4, assign52620_e80084_d_n5, assign52620_e80084_d_n6, assign52620_e80084_d_n7, assign52620_e80084_d_n8, assign52620_e80084_d_n9, assign52620_e80084_d_n10, assign52620_e80084_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign52620_e80084;
        locals.var_dnm_dn0 = assign52620_e80084_d_n0;
        locals.var_dnm_dn2 = assign52620_e80084_d_n2;
        locals.var_dnm_dn4 = assign52620_e80084_d_n4;
        locals.var_dnm_dn5 = assign52620_e80084_d_n5;
        locals.var_dnm_dn6 = assign52620_e80084_d_n6;
        locals.var_dnm_dn7 = assign52620_e80084_d_n7;
        locals.var_dnm_dn8 = assign52620_e80084_d_n8;
        locals.var_dnm_dn9 = assign52620_e80084_d_n9;
        locals.var_dnm_dn10 = assign52620_e80084_d_n10;
        locals.var_dnm_dn13 = assign52620_e80084_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign52630_e80099: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1337 = assign52630_e80099;
        locals.var_guard1337_rv = 0.0;

        let assign52640_e80102: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1338 = assign52640_e80102;
        locals.var_guard1338_rv = 0.0;

        let (assign52650_e80125,) = {
    if (((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) && (locals.var_guard1337 != 0.0)) && (locals.var_guard1338 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign52650_e80125;
        locals.var_mm_rv = 0.0;

        let assign52660_e80128: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1339 = assign52660_e80128;
        locals.var_guard1339_rv = 0.0;

        let (assign52670_e80154,) = {
    if ((((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) && (locals.var_guard1337 != 0.0)) && (locals.var_guard1338 == 0.0)) && (locals.var_guard1339 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign52670_e80154;
        locals.var_mm_rv = 0.0;

        let assign52680_e80157: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1340 = assign52680_e80157;
        locals.var_guard1340_rv = 0.0;

        let (assign52690_e80186,) = {
    if (((((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) && (locals.var_guard1337 != 0.0)) && (locals.var_guard1338 == 0.0)) && (locals.var_guard1339 == 0.0)) && (locals.var_guard1340 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign52690_e80186;
        locals.var_mm_rv = 0.0;

        let assign52700_e80189: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1341 = assign52700_e80189;
        locals.var_guard1341_rv = 0.0;

        let (assign52710_e80221,) = {
    if ((((((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) && (locals.var_guard1337 != 0.0)) && (locals.var_guard1338 == 0.0)) && (locals.var_guard1339 == 0.0)) && (locals.var_guard1340 == 0.0)) && (locals.var_guard1341 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign52710_e80221;
        locals.var_mm_rv = 0.0;

        let (assign52720_e80242,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) && (locals.var_guard1337 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign52720_e80242;
        locals.var_m0_rv = 0.0;

        let mut assign52730_loop_guard: usize = 0;
        while {
            let assign52730_cond_e80264: f64 = if (((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) && (locals.var_guard1337 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign52730_cond_e80264 != 0.0
        } {
            assign52730_loop_guard += 1;
            assert!(assign52730_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign52730_body0_e80286, assign52730_body0_e80286_d_n0, assign52730_body0_e80286_d_n2, assign52730_body0_e80286_d_n4, assign52730_body0_e80286_d_n5, assign52730_body0_e80286_d_n6, assign52730_body0_e80286_d_n7, assign52730_body0_e80286_d_n8, assign52730_body0_e80286_d_n9, assign52730_body0_e80286_d_n10, assign52730_body0_e80286_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) && (locals.var_guard1337 != 0.0)) {
        let assign52730_body0_e80284: f64 = (locals.var_dnm).sqrt();
        (assign52730_body0_e80284, (locals.var_dnm_dn0 / (2.0 * assign52730_body0_e80284)), (locals.var_dnm_dn2 / (2.0 * assign52730_body0_e80284)), (locals.var_dnm_dn4 / (2.0 * assign52730_body0_e80284)), (locals.var_dnm_dn5 / (2.0 * assign52730_body0_e80284)), (locals.var_dnm_dn6 / (2.0 * assign52730_body0_e80284)), (locals.var_dnm_dn7 / (2.0 * assign52730_body0_e80284)), (locals.var_dnm_dn8 / (2.0 * assign52730_body0_e80284)), (locals.var_dnm_dn9 / (2.0 * assign52730_body0_e80284)), (locals.var_dnm_dn10 / (2.0 * assign52730_body0_e80284)), (locals.var_dnm_dn13 / (2.0 * assign52730_body0_e80284)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign52730_body0_e80286;
            locals.var_dnm_dn0 = assign52730_body0_e80286_d_n0;
            locals.var_dnm_dn2 = assign52730_body0_e80286_d_n2;
            locals.var_dnm_dn4 = assign52730_body0_e80286_d_n4;
            locals.var_dnm_dn5 = assign52730_body0_e80286_d_n5;
            locals.var_dnm_dn6 = assign52730_body0_e80286_d_n6;
            locals.var_dnm_dn7 = assign52730_body0_e80286_d_n7;
            locals.var_dnm_dn8 = assign52730_body0_e80286_d_n8;
            locals.var_dnm_dn9 = assign52730_body0_e80286_d_n9;
            locals.var_dnm_dn10 = assign52730_body0_e80286_d_n10;
            locals.var_dnm_dn13 = assign52730_body0_e80286_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign52730_body1_e80309,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) && (locals.var_guard1337 != 0.0)) {
        let assign52730_body1_e80307: f64 = (locals.var_m0 + 1.0);
        (assign52730_body1_e80307,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign52730_body1_e80309;
            locals.var_m0_rv = 0.0;
        }

        let (assign52740_e80342, assign52740_e80342_d_n0, assign52740_e80342_d_n2, assign52740_e80342_d_n4, assign52740_e80342_d_n5, assign52740_e80342_d_n6, assign52740_e80342_d_n7, assign52740_e80342_d_n8, assign52740_e80342_d_n9, assign52740_e80342_d_n10, assign52740_e80342_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) && (locals.var_guard1337 == 0.0)) {
        let (assign52740_e80340, assign52740_e80340_d_n0, assign52740_e80340_d_n2, assign52740_e80340_d_n4, assign52740_e80340_d_n5, assign52740_e80340_d_n6, assign52740_e80340_d_n7, assign52740_e80340_d_n8, assign52740_e80340_d_n9, assign52740_e80340_d_n10, assign52740_e80340_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign52740_e80337: f64 = (2.0 * 2.0);
                let assign52740_e80338: f64 = (1.0 / assign52740_e80337);
                let assign52740_e80339: f64 = (locals.var_dnm).powf(assign52740_e80338);
                (assign52740_e80339, if 0.0 == 0.0 && ((assign52740_e80338) as f64).is_finite() && ((assign52740_e80338) as f64).fract() == 0.0 { if assign52740_e80338 == 0.0 { 0.0 } else { (assign52740_e80338 * ((locals.var_dnm).powf(assign52740_e80338 - 1.0) * locals.var_dnm_dn0)) } } else { (assign52740_e80339 * (assign52740_e80338 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign52740_e80338) as f64).is_finite() && ((assign52740_e80338) as f64).fract() == 0.0 { if assign52740_e80338 == 0.0 { 0.0 } else { (assign52740_e80338 * ((locals.var_dnm).powf(assign52740_e80338 - 1.0) * locals.var_dnm_dn2)) } } else { (assign52740_e80339 * (assign52740_e80338 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign52740_e80338) as f64).is_finite() && ((assign52740_e80338) as f64).fract() == 0.0 { if assign52740_e80338 == 0.0 { 0.0 } else { (assign52740_e80338 * ((locals.var_dnm).powf(assign52740_e80338 - 1.0) * locals.var_dnm_dn4)) } } else { (assign52740_e80339 * (assign52740_e80338 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign52740_e80338) as f64).is_finite() && ((assign52740_e80338) as f64).fract() == 0.0 { if assign52740_e80338 == 0.0 { 0.0 } else { (assign52740_e80338 * ((locals.var_dnm).powf(assign52740_e80338 - 1.0) * locals.var_dnm_dn5)) } } else { (assign52740_e80339 * (assign52740_e80338 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign52740_e80338) as f64).is_finite() && ((assign52740_e80338) as f64).fract() == 0.0 { if assign52740_e80338 == 0.0 { 0.0 } else { (assign52740_e80338 * ((locals.var_dnm).powf(assign52740_e80338 - 1.0) * locals.var_dnm_dn6)) } } else { (assign52740_e80339 * (assign52740_e80338 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign52740_e80338) as f64).is_finite() && ((assign52740_e80338) as f64).fract() == 0.0 { if assign52740_e80338 == 0.0 { 0.0 } else { (assign52740_e80338 * ((locals.var_dnm).powf(assign52740_e80338 - 1.0) * locals.var_dnm_dn7)) } } else { (assign52740_e80339 * (assign52740_e80338 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign52740_e80338) as f64).is_finite() && ((assign52740_e80338) as f64).fract() == 0.0 { if assign52740_e80338 == 0.0 { 0.0 } else { (assign52740_e80338 * ((locals.var_dnm).powf(assign52740_e80338 - 1.0) * locals.var_dnm_dn8)) } } else { (assign52740_e80339 * (assign52740_e80338 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign52740_e80338) as f64).is_finite() && ((assign52740_e80338) as f64).fract() == 0.0 { if assign52740_e80338 == 0.0 { 0.0 } else { (assign52740_e80338 * ((locals.var_dnm).powf(assign52740_e80338 - 1.0) * locals.var_dnm_dn9)) } } else { (assign52740_e80339 * (assign52740_e80338 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign52740_e80338) as f64).is_finite() && ((assign52740_e80338) as f64).fract() == 0.0 { if assign52740_e80338 == 0.0 { 0.0 } else { (assign52740_e80338 * ((locals.var_dnm).powf(assign52740_e80338 - 1.0) * locals.var_dnm_dn10)) } } else { (assign52740_e80339 * (assign52740_e80338 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign52740_e80338) as f64).is_finite() && ((assign52740_e80338) as f64).fract() == 0.0 { if assign52740_e80338 == 0.0 { 0.0 } else { (assign52740_e80338 * ((locals.var_dnm).powf(assign52740_e80338 - 1.0) * locals.var_dnm_dn13)) } } else { (assign52740_e80339 * (assign52740_e80338 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign52740_e80340, assign52740_e80340_d_n0, assign52740_e80340_d_n2, assign52740_e80340_d_n4, assign52740_e80340_d_n5, assign52740_e80340_d_n6, assign52740_e80340_d_n7, assign52740_e80340_d_n8, assign52740_e80340_d_n9, assign52740_e80340_d_n10, assign52740_e80340_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign52740_e80342;
        locals.var_dnm_dn0 = assign52740_e80342_d_n0;
        locals.var_dnm_dn2 = assign52740_e80342_d_n2;
        locals.var_dnm_dn4 = assign52740_e80342_d_n4;
        locals.var_dnm_dn5 = assign52740_e80342_d_n5;
        locals.var_dnm_dn6 = assign52740_e80342_d_n6;
        locals.var_dnm_dn7 = assign52740_e80342_d_n7;
        locals.var_dnm_dn8 = assign52740_e80342_d_n8;
        locals.var_dnm_dn9 = assign52740_e80342_d_n9;
        locals.var_dnm_dn10 = assign52740_e80342_d_n10;
        locals.var_dnm_dn13 = assign52740_e80342_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign52750_e80363, assign52750_e80363_d_n0, assign52750_e80363_d_n2, assign52750_e80363_d_n4, assign52750_e80363_d_n5, assign52750_e80363_d_n6, assign52750_e80363_d_n7, assign52750_e80363_d_n8, assign52750_e80363_d_n9, assign52750_e80363_d_n10, assign52750_e80363_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) {
        let assign52750_e80361: f64 = (1.0 / locals.var_dnm);
        (assign52750_e80361, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign52750_e80363;
        locals.var_dnm_dn0 = assign52750_e80363_d_n0;
        locals.var_dnm_dn2 = assign52750_e80363_d_n2;
        locals.var_dnm_dn4 = assign52750_e80363_d_n4;
        locals.var_dnm_dn5 = assign52750_e80363_d_n5;
        locals.var_dnm_dn6 = assign52750_e80363_d_n6;
        locals.var_dnm_dn7 = assign52750_e80363_d_n7;
        locals.var_dnm_dn8 = assign52750_e80363_d_n8;
        locals.var_dnm_dn9 = assign52750_e80363_d_n9;
        locals.var_dnm_dn10 = assign52750_e80363_d_n10;
        locals.var_dnm_dn13 = assign52750_e80363_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign52760_e80386, assign52760_e80386_d_n0, assign52760_e80386_d_n2, assign52760_e80386_d_n4, assign52760_e80386_d_n5, assign52760_e80386_d_n6, assign52760_e80386_d_n7, assign52760_e80386_d_n8, assign52760_e80386_d_n9, assign52760_e80386_d_n10, assign52760_e80386_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) {
        let assign52760_e80382: f64 = (locals.var_tmf1 * 0.2);
        let assign52760_e80384: f64 = (assign52760_e80382 * locals.var_dnm);
        (assign52760_e80384, (((locals.var_tmf1_dn0 * 0.2) * locals.var_dnm) + (assign52760_e80382 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.2) * locals.var_dnm) + (assign52760_e80382 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.2) * locals.var_dnm) + (assign52760_e80382 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.2) * locals.var_dnm) + (assign52760_e80382 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.2) * locals.var_dnm) + (assign52760_e80382 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.2) * locals.var_dnm) + (assign52760_e80382 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.2) * locals.var_dnm) + (assign52760_e80382 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.2) * locals.var_dnm) + (assign52760_e80382 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.2) * locals.var_dnm) + (assign52760_e80382 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 0.2) * locals.var_dnm) + (assign52760_e80382 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign52760_e80386;
        locals.var_tmf0_dn0 = assign52760_e80386_d_n0;
        locals.var_tmf0_dn2 = assign52760_e80386_d_n2;
        locals.var_tmf0_dn4 = assign52760_e80386_d_n4;
        locals.var_tmf0_dn5 = assign52760_e80386_d_n5;
        locals.var_tmf0_dn6 = assign52760_e80386_d_n6;
        locals.var_tmf0_dn7 = assign52760_e80386_d_n7;
        locals.var_tmf0_dn8 = assign52760_e80386_d_n8;
        locals.var_tmf0_dn9 = assign52760_e80386_d_n9;
        locals.var_tmf0_dn10 = assign52760_e80386_d_n10;
        locals.var_tmf0_dn13 = assign52760_e80386_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign52770_e80411, assign52770_e80411_d_n0, assign52770_e80411_d_n2, assign52770_e80411_d_n4, assign52770_e80411_d_n5, assign52770_e80411_d_n6, assign52770_e80411_d_n7, assign52770_e80411_d_n8, assign52770_e80411_d_n9, assign52770_e80411_d_n10, assign52770_e80411_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) {
        let assign52770_e80405: f64 = (0.2 * locals.var_xmp);
        let assign52770_e80407: f64 = (assign52770_e80405 * locals.var_dnm);
        let assign52770_e80409: f64 = (assign52770_e80407 / locals.var_arg);
        (assign52770_e80409, ((((((0.2 * locals.var_xmp_dn0) * locals.var_dnm) + (assign52770_e80405 * locals.var_dnm_dn0)) * locals.var_arg) - (assign52770_e80407 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn2) * locals.var_dnm) + (assign52770_e80405 * locals.var_dnm_dn2)) * locals.var_arg) - (assign52770_e80407 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn4) * locals.var_dnm) + (assign52770_e80405 * locals.var_dnm_dn4)) * locals.var_arg) - (assign52770_e80407 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn5) * locals.var_dnm) + (assign52770_e80405 * locals.var_dnm_dn5)) * locals.var_arg) - (assign52770_e80407 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn6) * locals.var_dnm) + (assign52770_e80405 * locals.var_dnm_dn6)) * locals.var_arg) - (assign52770_e80407 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn7) * locals.var_dnm) + (assign52770_e80405 * locals.var_dnm_dn7)) * locals.var_arg) - (assign52770_e80407 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn8) * locals.var_dnm) + (assign52770_e80405 * locals.var_dnm_dn8)) * locals.var_arg) - (assign52770_e80407 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn9) * locals.var_dnm) + (assign52770_e80405 * locals.var_dnm_dn9)) * locals.var_arg) - (assign52770_e80407 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn10) * locals.var_dnm) + (assign52770_e80405 * locals.var_dnm_dn10)) * locals.var_arg) - (assign52770_e80407 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn13) * locals.var_dnm) + (assign52770_e80405 * locals.var_dnm_dn13)) * locals.var_arg) - (assign52770_e80407 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign52770_e80411;
        locals.var_t0_dn0 = assign52770_e80411_d_n0;
        locals.var_t0_dn2 = assign52770_e80411_d_n2;
        locals.var_t0_dn4 = assign52770_e80411_d_n4;
        locals.var_t0_dn5 = assign52770_e80411_d_n5;
        locals.var_t0_dn6 = assign52770_e80411_d_n6;
        locals.var_t0_dn7 = assign52770_e80411_d_n7;
        locals.var_t0_dn8 = assign52770_e80411_d_n8;
        locals.var_t0_dn9 = assign52770_e80411_d_n9;
        locals.var_t0_dn10 = assign52770_e80411_d_n10;
        locals.var_t0_dn13 = assign52770_e80411_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign52780_e80434, assign52780_e80434_d_n0, assign52780_e80434_d_n2, assign52780_e80434_d_n4, assign52780_e80434_d_n5, assign52780_e80434_d_n6, assign52780_e80434_d_n7, assign52780_e80434_d_n8, assign52780_e80434_d_n9, assign52780_e80434_d_n10, assign52780_e80434_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) {
        let assign52780_e80430: f64 = (locals.var_ps0dep0 + 0.2);
        let assign52780_e80432: f64 = (assign52780_e80430 - locals.var_tmf0);
        (assign52780_e80432, (locals.var_ps0dep0_dn0 - locals.var_tmf0_dn0), (locals.var_ps0dep0_dn2 - locals.var_tmf0_dn2), (locals.var_ps0dep0_dn4 - locals.var_tmf0_dn4), (locals.var_ps0dep0_dn5 - locals.var_tmf0_dn5), (locals.var_ps0dep0_dn6 - locals.var_tmf0_dn6), (locals.var_ps0dep0_dn7 - locals.var_tmf0_dn7), (locals.var_ps0dep0_dn8 - locals.var_tmf0_dn8), (locals.var_ps0dep0_dn9 - locals.var_tmf0_dn9), (locals.var_ps0dep0_dn10 - locals.var_tmf0_dn10), (locals.var_ps0dep0_dn13 - locals.var_tmf0_dn13),)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn13,)
    }
};
        locals.var_ps0dep = assign52780_e80434;
        locals.var_ps0dep_dn0 = assign52780_e80434_d_n0;
        locals.var_ps0dep_dn2 = assign52780_e80434_d_n2;
        locals.var_ps0dep_dn4 = assign52780_e80434_d_n4;
        locals.var_ps0dep_dn5 = assign52780_e80434_d_n5;
        locals.var_ps0dep_dn6 = assign52780_e80434_d_n6;
        locals.var_ps0dep_dn7 = assign52780_e80434_d_n7;
        locals.var_ps0dep_dn8 = assign52780_e80434_d_n8;
        locals.var_ps0dep_dn9 = assign52780_e80434_d_n9;
        locals.var_ps0dep_dn10 = assign52780_e80434_d_n10;
        locals.var_ps0dep_dn13 = assign52780_e80434_d_n13;
        locals.var_ps0dep_rv = 0.0;

        let (assign52790_e80453, assign52790_e80453_d_n0, assign52790_e80453_d_n2, assign52790_e80453_d_n4, assign52790_e80453_d_n5, assign52790_e80453_d_n6, assign52790_e80453_d_n7, assign52790_e80453_d_n8, assign52790_e80453_d_n9, assign52790_e80453_d_n10, assign52790_e80453_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign52790_e80453;
        locals.var_t0_dn0 = assign52790_e80453_d_n0;
        locals.var_t0_dn2 = assign52790_e80453_d_n2;
        locals.var_t0_dn4 = assign52790_e80453_d_n4;
        locals.var_t0_dn5 = assign52790_e80453_d_n5;
        locals.var_t0_dn6 = assign52790_e80453_d_n6;
        locals.var_t0_dn7 = assign52790_e80453_d_n7;
        locals.var_t0_dn8 = assign52790_e80453_d_n8;
        locals.var_t0_dn9 = assign52790_e80453_d_n9;
        locals.var_t0_dn10 = assign52790_e80453_d_n10;
        locals.var_t0_dn13 = assign52790_e80453_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign52800_e80473, assign52800_e80473_d_n0, assign52800_e80473_d_n2, assign52800_e80473_d_n4, assign52800_e80473_d_n5, assign52800_e80473_d_n6, assign52800_e80473_d_n7, assign52800_e80473_d_n8, assign52800_e80473_d_n9, assign52800_e80473_d_n10, assign52800_e80473_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 == 0.0)) {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn13,)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn13,)
    }
};
        locals.var_ps0dep = assign52800_e80473;
        locals.var_ps0dep_dn0 = assign52800_e80473_d_n0;
        locals.var_ps0dep_dn2 = assign52800_e80473_d_n2;
        locals.var_ps0dep_dn4 = assign52800_e80473_d_n4;
        locals.var_ps0dep_dn5 = assign52800_e80473_d_n5;
        locals.var_ps0dep_dn6 = assign52800_e80473_d_n6;
        locals.var_ps0dep_dn7 = assign52800_e80473_d_n7;
        locals.var_ps0dep_dn8 = assign52800_e80473_d_n8;
        locals.var_ps0dep_dn9 = assign52800_e80473_d_n9;
        locals.var_ps0dep_dn10 = assign52800_e80473_d_n10;
        locals.var_ps0dep_dn13 = assign52800_e80473_d_n13;
        locals.var_ps0dep_rv = 0.0;

        let (assign52810_e80493, assign52810_e80493_d_n0, assign52810_e80493_d_n2, assign52810_e80493_d_n4, assign52810_e80493_d_n5, assign52810_e80493_d_n6, assign52810_e80493_d_n7, assign52810_e80493_d_n8, assign52810_e80493_d_n9, assign52810_e80493_d_n10, assign52810_e80493_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign52810_e80493;
        locals.var_t0_dn0 = assign52810_e80493_d_n0;
        locals.var_t0_dn2 = assign52810_e80493_d_n2;
        locals.var_t0_dn4 = assign52810_e80493_d_n4;
        locals.var_t0_dn5 = assign52810_e80493_d_n5;
        locals.var_t0_dn6 = assign52810_e80493_d_n6;
        locals.var_t0_dn7 = assign52810_e80493_d_n7;
        locals.var_t0_dn8 = assign52810_e80493_d_n8;
        locals.var_t0_dn9 = assign52810_e80493_d_n9;
        locals.var_t0_dn10 = assign52810_e80493_d_n10;
        locals.var_t0_dn13 = assign52810_e80493_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign52820_e80507, assign52820_e80507_d_n0, assign52820_e80507_d_n2, assign52820_e80507_d_n4, assign52820_e80507_d_n5, assign52820_e80507_d_n6, assign52820_e80507_d_n7, assign52820_e80507_d_n8, assign52820_e80507_d_n9, assign52820_e80507_d_n10, assign52820_e80507_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn13,)
    } else {
        (locals.var_ps0_res, locals.var_ps0_res_dn0, locals.var_ps0_res_dn2, locals.var_ps0_res_dn4, locals.var_ps0_res_dn5, locals.var_ps0_res_dn6, locals.var_ps0_res_dn7, locals.var_ps0_res_dn8, locals.var_ps0_res_dn9, locals.var_ps0_res_dn10, locals.var_ps0_res_dn13,)
    }
};
        locals.var_ps0_res = assign52820_e80507;
        locals.var_ps0_res_dn0 = assign52820_e80507_d_n0;
        locals.var_ps0_res_dn2 = assign52820_e80507_d_n2;
        locals.var_ps0_res_dn4 = assign52820_e80507_d_n4;
        locals.var_ps0_res_dn5 = assign52820_e80507_d_n5;
        locals.var_ps0_res_dn6 = assign52820_e80507_d_n6;
        locals.var_ps0_res_dn7 = assign52820_e80507_d_n7;
        locals.var_ps0_res_dn8 = assign52820_e80507_d_n8;
        locals.var_ps0_res_dn9 = assign52820_e80507_d_n9;
        locals.var_ps0_res_dn10 = assign52820_e80507_d_n10;
        locals.var_ps0_res_dn13 = assign52820_e80507_d_n13;
        locals.var_ps0_res_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_184(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign52830_e80526,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) {
        let (assign52830_e80524,) = {
            if (1e-6 >= p.p407) {
                (1e-6,)
            } else {
                (p.p407,)
            }
        };
        (assign52830_e80524,)
    } else {
        (locals.var_vgpdep_dlt__blk1142,)
    }
};
        locals.var_vgpdep_dlt__blk1142 = assign52830_e80526;
        locals.var_vgpdep_dlt__blk1142_rv = 0.0;

        let assign52840_e80530: f64 = (-locals.var_vgpdep_dlt__blk1142);
        let assign52840_e80535: f64 = if ((locals.var_ps0_res > assign52840_e80530) && (locals.var_vgpdep_dlt__blk1142 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1342 = assign52840_e80535;
        locals.var_guard1342_rv = 0.0;

        let (assign52850_e80555, assign52850_e80555_d_n0, assign52850_e80555_d_n2, assign52850_e80555_d_n4, assign52850_e80555_d_n5, assign52850_e80555_d_n6, assign52850_e80555_d_n7, assign52850_e80555_d_n8, assign52850_e80555_d_n9, assign52850_e80555_d_n10, assign52850_e80555_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) {
        let assign52850_e80551: f64 = locals.var_ps0_res;
        let assign52850_e80553: f64 = (assign52850_e80551 + locals.var_vgpdep_dlt__blk1142);
        (assign52850_e80553, locals.var_ps0_res_dn0, locals.var_ps0_res_dn2, locals.var_ps0_res_dn4, locals.var_ps0_res_dn5, locals.var_ps0_res_dn6, locals.var_ps0_res_dn7, locals.var_ps0_res_dn8, locals.var_ps0_res_dn9, locals.var_ps0_res_dn10, locals.var_ps0_res_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign52850_e80555;
        locals.var_tmf1_dn0 = assign52850_e80555_d_n0;
        locals.var_tmf1_dn2 = assign52850_e80555_d_n2;
        locals.var_tmf1_dn4 = assign52850_e80555_d_n4;
        locals.var_tmf1_dn5 = assign52850_e80555_d_n5;
        locals.var_tmf1_dn6 = assign52850_e80555_d_n6;
        locals.var_tmf1_dn7 = assign52850_e80555_d_n7;
        locals.var_tmf1_dn8 = assign52850_e80555_d_n8;
        locals.var_tmf1_dn9 = assign52850_e80555_d_n9;
        locals.var_tmf1_dn10 = assign52850_e80555_d_n10;
        locals.var_tmf1_dn13 = assign52850_e80555_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign52860_e80573, assign52860_e80573_d_n0, assign52860_e80573_d_n2, assign52860_e80573_d_n4, assign52860_e80573_d_n5, assign52860_e80573_d_n6, assign52860_e80573_d_n7, assign52860_e80573_d_n8, assign52860_e80573_d_n9, assign52860_e80573_d_n10, assign52860_e80573_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) {
        let assign52860_e80571: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign52860_e80571, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign52860_e80573;
        locals.var_x2_dn0 = assign52860_e80573_d_n0;
        locals.var_x2_dn2 = assign52860_e80573_d_n2;
        locals.var_x2_dn4 = assign52860_e80573_d_n4;
        locals.var_x2_dn5 = assign52860_e80573_d_n5;
        locals.var_x2_dn6 = assign52860_e80573_d_n6;
        locals.var_x2_dn7 = assign52860_e80573_d_n7;
        locals.var_x2_dn8 = assign52860_e80573_d_n8;
        locals.var_x2_dn9 = assign52860_e80573_d_n9;
        locals.var_x2_dn10 = assign52860_e80573_d_n10;
        locals.var_x2_dn13 = assign52860_e80573_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign52870_e80591, assign52870_e80591_d_n0, assign52870_e80591_d_n2, assign52870_e80591_d_n4, assign52870_e80591_d_n5, assign52870_e80591_d_n6, assign52870_e80591_d_n7, assign52870_e80591_d_n8, assign52870_e80591_d_n9, assign52870_e80591_d_n10, assign52870_e80591_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) {
        let assign52870_e80589: f64 = (locals.var_vgpdep_dlt__blk1142 * locals.var_vgpdep_dlt__blk1142);
        (assign52870_e80589, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign52870_e80591;
        locals.var_xmax2_dn0 = assign52870_e80591_d_n0;
        locals.var_xmax2_dn2 = assign52870_e80591_d_n2;
        locals.var_xmax2_dn4 = assign52870_e80591_d_n4;
        locals.var_xmax2_dn5 = assign52870_e80591_d_n5;
        locals.var_xmax2_dn6 = assign52870_e80591_d_n6;
        locals.var_xmax2_dn7 = assign52870_e80591_d_n7;
        locals.var_xmax2_dn8 = assign52870_e80591_d_n8;
        locals.var_xmax2_dn9 = assign52870_e80591_d_n9;
        locals.var_xmax2_dn10 = assign52870_e80591_d_n10;
        locals.var_xmax2_dn13 = assign52870_e80591_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign52880_e80607, assign52880_e80607_d_n0, assign52880_e80607_d_n2, assign52880_e80607_d_n4, assign52880_e80607_d_n5, assign52880_e80607_d_n6, assign52880_e80607_d_n7, assign52880_e80607_d_n8, assign52880_e80607_d_n9, assign52880_e80607_d_n10, assign52880_e80607_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign52880_e80607;
        locals.var_xp_dn0 = assign52880_e80607_d_n0;
        locals.var_xp_dn2 = assign52880_e80607_d_n2;
        locals.var_xp_dn4 = assign52880_e80607_d_n4;
        locals.var_xp_dn5 = assign52880_e80607_d_n5;
        locals.var_xp_dn6 = assign52880_e80607_d_n6;
        locals.var_xp_dn7 = assign52880_e80607_d_n7;
        locals.var_xp_dn8 = assign52880_e80607_d_n8;
        locals.var_xp_dn9 = assign52880_e80607_d_n9;
        locals.var_xp_dn10 = assign52880_e80607_d_n10;
        locals.var_xp_dn13 = assign52880_e80607_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign52890_e80623, assign52890_e80623_d_n0, assign52890_e80623_d_n2, assign52890_e80623_d_n4, assign52890_e80623_d_n5, assign52890_e80623_d_n6, assign52890_e80623_d_n7, assign52890_e80623_d_n8, assign52890_e80623_d_n9, assign52890_e80623_d_n10, assign52890_e80623_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign52890_e80623;
        locals.var_xmp_dn0 = assign52890_e80623_d_n0;
        locals.var_xmp_dn2 = assign52890_e80623_d_n2;
        locals.var_xmp_dn4 = assign52890_e80623_d_n4;
        locals.var_xmp_dn5 = assign52890_e80623_d_n5;
        locals.var_xmp_dn6 = assign52890_e80623_d_n6;
        locals.var_xmp_dn7 = assign52890_e80623_d_n7;
        locals.var_xmp_dn8 = assign52890_e80623_d_n8;
        locals.var_xmp_dn9 = assign52890_e80623_d_n9;
        locals.var_xmp_dn10 = assign52890_e80623_d_n10;
        locals.var_xmp_dn13 = assign52890_e80623_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign52900_e80639,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign52900_e80639;
        locals.var_m0_rv = 0.0;

        let (assign52910_e80655,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign52910_e80655;
        locals.var_mm_rv = 0.0;

        let (assign52920_e80671, assign52920_e80671_d_n0, assign52920_e80671_d_n2, assign52920_e80671_d_n4, assign52920_e80671_d_n5, assign52920_e80671_d_n6, assign52920_e80671_d_n7, assign52920_e80671_d_n8, assign52920_e80671_d_n9, assign52920_e80671_d_n10, assign52920_e80671_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign52920_e80671;
        locals.var_arg_dn0 = assign52920_e80671_d_n0;
        locals.var_arg_dn2 = assign52920_e80671_d_n2;
        locals.var_arg_dn4 = assign52920_e80671_d_n4;
        locals.var_arg_dn5 = assign52920_e80671_d_n5;
        locals.var_arg_dn6 = assign52920_e80671_d_n6;
        locals.var_arg_dn7 = assign52920_e80671_d_n7;
        locals.var_arg_dn8 = assign52920_e80671_d_n8;
        locals.var_arg_dn9 = assign52920_e80671_d_n9;
        locals.var_arg_dn10 = assign52920_e80671_d_n10;
        locals.var_arg_dn13 = assign52920_e80671_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign52930_e80687, assign52930_e80687_d_n0, assign52930_e80687_d_n2, assign52930_e80687_d_n4, assign52930_e80687_d_n5, assign52930_e80687_d_n6, assign52930_e80687_d_n7, assign52930_e80687_d_n8, assign52930_e80687_d_n9, assign52930_e80687_d_n10, assign52930_e80687_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign52930_e80687;
        locals.var_dnm_dn0 = assign52930_e80687_d_n0;
        locals.var_dnm_dn2 = assign52930_e80687_d_n2;
        locals.var_dnm_dn4 = assign52930_e80687_d_n4;
        locals.var_dnm_dn5 = assign52930_e80687_d_n5;
        locals.var_dnm_dn6 = assign52930_e80687_d_n6;
        locals.var_dnm_dn7 = assign52930_e80687_d_n7;
        locals.var_dnm_dn8 = assign52930_e80687_d_n8;
        locals.var_dnm_dn9 = assign52930_e80687_d_n9;
        locals.var_dnm_dn10 = assign52930_e80687_d_n10;
        locals.var_dnm_dn13 = assign52930_e80687_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign52940_e80703,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign52940_e80703;
        locals.var_m0_rv = 0.0;

        let mut assign52950_loop_guard: usize = 0;
        while {
            let assign52950_cond_e80720: f64 = if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) && (locals.var_m0 < locals.var_vgpdep_pw__blk1143)) { 1.0 } else { 0.0 };
            assign52950_cond_e80720 != 0.0
        } {
            assign52950_loop_guard += 1;
            assert!(assign52950_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign52950_body0_e80738, assign52950_body0_e80738_d_n0, assign52950_body0_e80738_d_n2, assign52950_body0_e80738_d_n4, assign52950_body0_e80738_d_n5, assign52950_body0_e80738_d_n6, assign52950_body0_e80738_d_n7, assign52950_body0_e80738_d_n8, assign52950_body0_e80738_d_n9, assign52950_body0_e80738_d_n10, assign52950_body0_e80738_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) {
        let assign52950_body0_e80736: f64 = (locals.var_xp * locals.var_x2);
        (assign52950_body0_e80736, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
            locals.var_xp = assign52950_body0_e80738;
            locals.var_xp_dn0 = assign52950_body0_e80738_d_n0;
            locals.var_xp_dn2 = assign52950_body0_e80738_d_n2;
            locals.var_xp_dn4 = assign52950_body0_e80738_d_n4;
            locals.var_xp_dn5 = assign52950_body0_e80738_d_n5;
            locals.var_xp_dn6 = assign52950_body0_e80738_d_n6;
            locals.var_xp_dn7 = assign52950_body0_e80738_d_n7;
            locals.var_xp_dn8 = assign52950_body0_e80738_d_n8;
            locals.var_xp_dn9 = assign52950_body0_e80738_d_n9;
            locals.var_xp_dn10 = assign52950_body0_e80738_d_n10;
            locals.var_xp_dn13 = assign52950_body0_e80738_d_n13;
            locals.var_xp_rv = 0.0;
            let (assign52950_body1_e80756, assign52950_body1_e80756_d_n0, assign52950_body1_e80756_d_n2, assign52950_body1_e80756_d_n4, assign52950_body1_e80756_d_n5, assign52950_body1_e80756_d_n6, assign52950_body1_e80756_d_n7, assign52950_body1_e80756_d_n8, assign52950_body1_e80756_d_n9, assign52950_body1_e80756_d_n10, assign52950_body1_e80756_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) {
        let assign52950_body1_e80754: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign52950_body1_e80754, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
            locals.var_xmp = assign52950_body1_e80756;
            locals.var_xmp_dn0 = assign52950_body1_e80756_d_n0;
            locals.var_xmp_dn2 = assign52950_body1_e80756_d_n2;
            locals.var_xmp_dn4 = assign52950_body1_e80756_d_n4;
            locals.var_xmp_dn5 = assign52950_body1_e80756_d_n5;
            locals.var_xmp_dn6 = assign52950_body1_e80756_d_n6;
            locals.var_xmp_dn7 = assign52950_body1_e80756_d_n7;
            locals.var_xmp_dn8 = assign52950_body1_e80756_d_n8;
            locals.var_xmp_dn9 = assign52950_body1_e80756_d_n9;
            locals.var_xmp_dn10 = assign52950_body1_e80756_d_n10;
            locals.var_xmp_dn13 = assign52950_body1_e80756_d_n13;
            locals.var_xmp_rv = 0.0;
            let (assign52950_body2_e80774,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) {
        let assign52950_body2_e80772: f64 = (locals.var_m0 + 1.0);
        (assign52950_body2_e80772,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign52950_body2_e80774;
            locals.var_m0_rv = 0.0;
        }

        let (assign52960_e80792, assign52960_e80792_d_n0, assign52960_e80792_d_n2, assign52960_e80792_d_n4, assign52960_e80792_d_n5, assign52960_e80792_d_n6, assign52960_e80792_d_n7, assign52960_e80792_d_n8, assign52960_e80792_d_n9, assign52960_e80792_d_n10, assign52960_e80792_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) {
        let assign52960_e80790: f64 = (locals.var_xp + locals.var_xmp);
        (assign52960_e80790, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign52960_e80792;
        locals.var_arg_dn0 = assign52960_e80792_d_n0;
        locals.var_arg_dn2 = assign52960_e80792_d_n2;
        locals.var_arg_dn4 = assign52960_e80792_d_n4;
        locals.var_arg_dn5 = assign52960_e80792_d_n5;
        locals.var_arg_dn6 = assign52960_e80792_d_n6;
        locals.var_arg_dn7 = assign52960_e80792_d_n7;
        locals.var_arg_dn8 = assign52960_e80792_d_n8;
        locals.var_arg_dn9 = assign52960_e80792_d_n9;
        locals.var_arg_dn10 = assign52960_e80792_d_n10;
        locals.var_arg_dn13 = assign52960_e80792_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign52970_e80808, assign52970_e80808_d_n0, assign52970_e80808_d_n2, assign52970_e80808_d_n4, assign52970_e80808_d_n5, assign52970_e80808_d_n6, assign52970_e80808_d_n7, assign52970_e80808_d_n8, assign52970_e80808_d_n9, assign52970_e80808_d_n10, assign52970_e80808_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign52970_e80808;
        locals.var_dnm_dn0 = assign52970_e80808_d_n0;
        locals.var_dnm_dn2 = assign52970_e80808_d_n2;
        locals.var_dnm_dn4 = assign52970_e80808_d_n4;
        locals.var_dnm_dn5 = assign52970_e80808_d_n5;
        locals.var_dnm_dn6 = assign52970_e80808_d_n6;
        locals.var_dnm_dn7 = assign52970_e80808_d_n7;
        locals.var_dnm_dn8 = assign52970_e80808_d_n8;
        locals.var_dnm_dn9 = assign52970_e80808_d_n9;
        locals.var_dnm_dn10 = assign52970_e80808_d_n10;
        locals.var_dnm_dn13 = assign52970_e80808_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign52980_e80823: f64 = if ((((locals.var_vgpdep_pw__blk1143 == 1.0) || (locals.var_vgpdep_pw__blk1143 == 2.0)) || (locals.var_vgpdep_pw__blk1143 == 4.0)) || (locals.var_vgpdep_pw__blk1143 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1343 = assign52980_e80823;
        locals.var_guard1343_rv = 0.0;

        let assign52990_e80826: f64 = if locals.var_vgpdep_pw__blk1143 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1344 = assign52990_e80826;
        locals.var_guard1344_rv = 0.0;

        let (assign53000_e80846,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) && (locals.var_guard1343 != 0.0)) && (locals.var_guard1344 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign53000_e80846;
        locals.var_mm_rv = 0.0;

        let assign53010_e80849: f64 = if locals.var_vgpdep_pw__blk1143 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1345 = assign53010_e80849;
        locals.var_guard1345_rv = 0.0;

        let (assign53020_e80872,) = {
    if (((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) && (locals.var_guard1343 != 0.0)) && (locals.var_guard1344 == 0.0)) && (locals.var_guard1345 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign53020_e80872;
        locals.var_mm_rv = 0.0;

        let assign53030_e80875: f64 = if locals.var_vgpdep_pw__blk1143 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1346 = assign53030_e80875;
        locals.var_guard1346_rv = 0.0;

        let (assign53040_e80901,) = {
    if ((((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) && (locals.var_guard1343 != 0.0)) && (locals.var_guard1344 == 0.0)) && (locals.var_guard1345 == 0.0)) && (locals.var_guard1346 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign53040_e80901;
        locals.var_mm_rv = 0.0;

        let assign53050_e80904: f64 = if locals.var_vgpdep_pw__blk1143 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1347 = assign53050_e80904;
        locals.var_guard1347_rv = 0.0;

        let (assign53060_e80933,) = {
    if (((((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) && (locals.var_guard1343 != 0.0)) && (locals.var_guard1344 == 0.0)) && (locals.var_guard1345 == 0.0)) && (locals.var_guard1346 == 0.0)) && (locals.var_guard1347 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign53060_e80933;
        locals.var_mm_rv = 0.0;

        let (assign53070_e80951,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) && (locals.var_guard1343 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign53070_e80951;
        locals.var_m0_rv = 0.0;

        let mut assign53080_loop_guard: usize = 0;
        while {
            let assign53080_cond_e80970: f64 = if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) && (locals.var_guard1343 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign53080_cond_e80970 != 0.0
        } {
            assign53080_loop_guard += 1;
            assert!(assign53080_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign53080_body0_e80989, assign53080_body0_e80989_d_n0, assign53080_body0_e80989_d_n2, assign53080_body0_e80989_d_n4, assign53080_body0_e80989_d_n5, assign53080_body0_e80989_d_n6, assign53080_body0_e80989_d_n7, assign53080_body0_e80989_d_n8, assign53080_body0_e80989_d_n9, assign53080_body0_e80989_d_n10, assign53080_body0_e80989_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) && (locals.var_guard1343 != 0.0)) {
        let assign53080_body0_e80987: f64 = (locals.var_dnm).sqrt();
        (assign53080_body0_e80987, (locals.var_dnm_dn0 / (2.0 * assign53080_body0_e80987)), (locals.var_dnm_dn2 / (2.0 * assign53080_body0_e80987)), (locals.var_dnm_dn4 / (2.0 * assign53080_body0_e80987)), (locals.var_dnm_dn5 / (2.0 * assign53080_body0_e80987)), (locals.var_dnm_dn6 / (2.0 * assign53080_body0_e80987)), (locals.var_dnm_dn7 / (2.0 * assign53080_body0_e80987)), (locals.var_dnm_dn8 / (2.0 * assign53080_body0_e80987)), (locals.var_dnm_dn9 / (2.0 * assign53080_body0_e80987)), (locals.var_dnm_dn10 / (2.0 * assign53080_body0_e80987)), (locals.var_dnm_dn13 / (2.0 * assign53080_body0_e80987)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign53080_body0_e80989;
            locals.var_dnm_dn0 = assign53080_body0_e80989_d_n0;
            locals.var_dnm_dn2 = assign53080_body0_e80989_d_n2;
            locals.var_dnm_dn4 = assign53080_body0_e80989_d_n4;
            locals.var_dnm_dn5 = assign53080_body0_e80989_d_n5;
            locals.var_dnm_dn6 = assign53080_body0_e80989_d_n6;
            locals.var_dnm_dn7 = assign53080_body0_e80989_d_n7;
            locals.var_dnm_dn8 = assign53080_body0_e80989_d_n8;
            locals.var_dnm_dn9 = assign53080_body0_e80989_d_n9;
            locals.var_dnm_dn10 = assign53080_body0_e80989_d_n10;
            locals.var_dnm_dn13 = assign53080_body0_e80989_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign53080_body1_e81009,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) && (locals.var_guard1343 != 0.0)) {
        let assign53080_body1_e81007: f64 = (locals.var_m0 + 1.0);
        (assign53080_body1_e81007,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign53080_body1_e81009;
            locals.var_m0_rv = 0.0;
        }

        let (assign53090_e81039, assign53090_e81039_d_n0, assign53090_e81039_d_n2, assign53090_e81039_d_n4, assign53090_e81039_d_n5, assign53090_e81039_d_n6, assign53090_e81039_d_n7, assign53090_e81039_d_n8, assign53090_e81039_d_n9, assign53090_e81039_d_n10, assign53090_e81039_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) && (locals.var_guard1343 == 0.0)) {
        let (assign53090_e81037, assign53090_e81037_d_n0, assign53090_e81037_d_n2, assign53090_e81037_d_n4, assign53090_e81037_d_n5, assign53090_e81037_d_n6, assign53090_e81037_d_n7, assign53090_e81037_d_n8, assign53090_e81037_d_n9, assign53090_e81037_d_n10, assign53090_e81037_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign53090_e81034: f64 = (2.0 * locals.var_vgpdep_pw__blk1143);
                let assign53090_e81035: f64 = (1.0 / assign53090_e81034);
                let assign53090_e81036: f64 = (locals.var_dnm).powf(assign53090_e81035);
                (assign53090_e81036, if 0.0 == 0.0 && ((assign53090_e81035) as f64).is_finite() && ((assign53090_e81035) as f64).fract() == 0.0 { if assign53090_e81035 == 0.0 { 0.0 } else { (assign53090_e81035 * ((locals.var_dnm).powf(assign53090_e81035 - 1.0) * locals.var_dnm_dn0)) } } else { (assign53090_e81036 * (assign53090_e81035 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53090_e81035) as f64).is_finite() && ((assign53090_e81035) as f64).fract() == 0.0 { if assign53090_e81035 == 0.0 { 0.0 } else { (assign53090_e81035 * ((locals.var_dnm).powf(assign53090_e81035 - 1.0) * locals.var_dnm_dn2)) } } else { (assign53090_e81036 * (assign53090_e81035 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53090_e81035) as f64).is_finite() && ((assign53090_e81035) as f64).fract() == 0.0 { if assign53090_e81035 == 0.0 { 0.0 } else { (assign53090_e81035 * ((locals.var_dnm).powf(assign53090_e81035 - 1.0) * locals.var_dnm_dn4)) } } else { (assign53090_e81036 * (assign53090_e81035 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53090_e81035) as f64).is_finite() && ((assign53090_e81035) as f64).fract() == 0.0 { if assign53090_e81035 == 0.0 { 0.0 } else { (assign53090_e81035 * ((locals.var_dnm).powf(assign53090_e81035 - 1.0) * locals.var_dnm_dn5)) } } else { (assign53090_e81036 * (assign53090_e81035 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53090_e81035) as f64).is_finite() && ((assign53090_e81035) as f64).fract() == 0.0 { if assign53090_e81035 == 0.0 { 0.0 } else { (assign53090_e81035 * ((locals.var_dnm).powf(assign53090_e81035 - 1.0) * locals.var_dnm_dn6)) } } else { (assign53090_e81036 * (assign53090_e81035 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53090_e81035) as f64).is_finite() && ((assign53090_e81035) as f64).fract() == 0.0 { if assign53090_e81035 == 0.0 { 0.0 } else { (assign53090_e81035 * ((locals.var_dnm).powf(assign53090_e81035 - 1.0) * locals.var_dnm_dn7)) } } else { (assign53090_e81036 * (assign53090_e81035 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53090_e81035) as f64).is_finite() && ((assign53090_e81035) as f64).fract() == 0.0 { if assign53090_e81035 == 0.0 { 0.0 } else { (assign53090_e81035 * ((locals.var_dnm).powf(assign53090_e81035 - 1.0) * locals.var_dnm_dn8)) } } else { (assign53090_e81036 * (assign53090_e81035 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53090_e81035) as f64).is_finite() && ((assign53090_e81035) as f64).fract() == 0.0 { if assign53090_e81035 == 0.0 { 0.0 } else { (assign53090_e81035 * ((locals.var_dnm).powf(assign53090_e81035 - 1.0) * locals.var_dnm_dn9)) } } else { (assign53090_e81036 * (assign53090_e81035 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53090_e81035) as f64).is_finite() && ((assign53090_e81035) as f64).fract() == 0.0 { if assign53090_e81035 == 0.0 { 0.0 } else { (assign53090_e81035 * ((locals.var_dnm).powf(assign53090_e81035 - 1.0) * locals.var_dnm_dn10)) } } else { (assign53090_e81036 * (assign53090_e81035 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53090_e81035) as f64).is_finite() && ((assign53090_e81035) as f64).fract() == 0.0 { if assign53090_e81035 == 0.0 { 0.0 } else { (assign53090_e81035 * ((locals.var_dnm).powf(assign53090_e81035 - 1.0) * locals.var_dnm_dn13)) } } else { (assign53090_e81036 * (assign53090_e81035 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign53090_e81037, assign53090_e81037_d_n0, assign53090_e81037_d_n2, assign53090_e81037_d_n4, assign53090_e81037_d_n5, assign53090_e81037_d_n6, assign53090_e81037_d_n7, assign53090_e81037_d_n8, assign53090_e81037_d_n9, assign53090_e81037_d_n10, assign53090_e81037_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign53090_e81039;
        locals.var_dnm_dn0 = assign53090_e81039_d_n0;
        locals.var_dnm_dn2 = assign53090_e81039_d_n2;
        locals.var_dnm_dn4 = assign53090_e81039_d_n4;
        locals.var_dnm_dn5 = assign53090_e81039_d_n5;
        locals.var_dnm_dn6 = assign53090_e81039_d_n6;
        locals.var_dnm_dn7 = assign53090_e81039_d_n7;
        locals.var_dnm_dn8 = assign53090_e81039_d_n8;
        locals.var_dnm_dn9 = assign53090_e81039_d_n9;
        locals.var_dnm_dn10 = assign53090_e81039_d_n10;
        locals.var_dnm_dn13 = assign53090_e81039_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign53100_e81057, assign53100_e81057_d_n0, assign53100_e81057_d_n2, assign53100_e81057_d_n4, assign53100_e81057_d_n5, assign53100_e81057_d_n6, assign53100_e81057_d_n7, assign53100_e81057_d_n8, assign53100_e81057_d_n9, assign53100_e81057_d_n10, assign53100_e81057_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) {
        let assign53100_e81055: f64 = (1.0 / locals.var_dnm);
        (assign53100_e81055, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign53100_e81057;
        locals.var_dnm_dn0 = assign53100_e81057_d_n0;
        locals.var_dnm_dn2 = assign53100_e81057_d_n2;
        locals.var_dnm_dn4 = assign53100_e81057_d_n4;
        locals.var_dnm_dn5 = assign53100_e81057_d_n5;
        locals.var_dnm_dn6 = assign53100_e81057_d_n6;
        locals.var_dnm_dn7 = assign53100_e81057_d_n7;
        locals.var_dnm_dn8 = assign53100_e81057_d_n8;
        locals.var_dnm_dn9 = assign53100_e81057_d_n9;
        locals.var_dnm_dn10 = assign53100_e81057_d_n10;
        locals.var_dnm_dn13 = assign53100_e81057_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign53110_e81077, assign53110_e81077_d_n0, assign53110_e81077_d_n2, assign53110_e81077_d_n4, assign53110_e81077_d_n5, assign53110_e81077_d_n6, assign53110_e81077_d_n7, assign53110_e81077_d_n8, assign53110_e81077_d_n9, assign53110_e81077_d_n10, assign53110_e81077_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) {
        let assign53110_e81073: f64 = (locals.var_tmf1 * locals.var_vgpdep_dlt__blk1142);
        let assign53110_e81075: f64 = (assign53110_e81073 * locals.var_dnm);
        (assign53110_e81075, (((locals.var_tmf1_dn0 * locals.var_vgpdep_dlt__blk1142) * locals.var_dnm) + (assign53110_e81073 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * locals.var_vgpdep_dlt__blk1142) * locals.var_dnm) + (assign53110_e81073 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * locals.var_vgpdep_dlt__blk1142) * locals.var_dnm) + (assign53110_e81073 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * locals.var_vgpdep_dlt__blk1142) * locals.var_dnm) + (assign53110_e81073 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * locals.var_vgpdep_dlt__blk1142) * locals.var_dnm) + (assign53110_e81073 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * locals.var_vgpdep_dlt__blk1142) * locals.var_dnm) + (assign53110_e81073 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * locals.var_vgpdep_dlt__blk1142) * locals.var_dnm) + (assign53110_e81073 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * locals.var_vgpdep_dlt__blk1142) * locals.var_dnm) + (assign53110_e81073 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * locals.var_vgpdep_dlt__blk1142) * locals.var_dnm) + (assign53110_e81073 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * locals.var_vgpdep_dlt__blk1142) * locals.var_dnm) + (assign53110_e81073 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign53110_e81077;
        locals.var_tmf0_dn0 = assign53110_e81077_d_n0;
        locals.var_tmf0_dn2 = assign53110_e81077_d_n2;
        locals.var_tmf0_dn4 = assign53110_e81077_d_n4;
        locals.var_tmf0_dn5 = assign53110_e81077_d_n5;
        locals.var_tmf0_dn6 = assign53110_e81077_d_n6;
        locals.var_tmf0_dn7 = assign53110_e81077_d_n7;
        locals.var_tmf0_dn8 = assign53110_e81077_d_n8;
        locals.var_tmf0_dn9 = assign53110_e81077_d_n9;
        locals.var_tmf0_dn10 = assign53110_e81077_d_n10;
        locals.var_tmf0_dn13 = assign53110_e81077_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign53120_e81099, assign53120_e81099_d_n0, assign53120_e81099_d_n2, assign53120_e81099_d_n4, assign53120_e81099_d_n5, assign53120_e81099_d_n6, assign53120_e81099_d_n7, assign53120_e81099_d_n8, assign53120_e81099_d_n9, assign53120_e81099_d_n10, assign53120_e81099_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) {
        let assign53120_e81093: f64 = (locals.var_vgpdep_dlt__blk1142 * locals.var_xmp);
        let assign53120_e81095: f64 = (assign53120_e81093 * locals.var_dnm);
        let assign53120_e81097: f64 = (assign53120_e81095 / locals.var_arg);
        (assign53120_e81097, ((((((locals.var_vgpdep_dlt__blk1142 * locals.var_xmp_dn0) * locals.var_dnm) + (assign53120_e81093 * locals.var_dnm_dn0)) * locals.var_arg) - (assign53120_e81095 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1142 * locals.var_xmp_dn2) * locals.var_dnm) + (assign53120_e81093 * locals.var_dnm_dn2)) * locals.var_arg) - (assign53120_e81095 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1142 * locals.var_xmp_dn4) * locals.var_dnm) + (assign53120_e81093 * locals.var_dnm_dn4)) * locals.var_arg) - (assign53120_e81095 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1142 * locals.var_xmp_dn5) * locals.var_dnm) + (assign53120_e81093 * locals.var_dnm_dn5)) * locals.var_arg) - (assign53120_e81095 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1142 * locals.var_xmp_dn6) * locals.var_dnm) + (assign53120_e81093 * locals.var_dnm_dn6)) * locals.var_arg) - (assign53120_e81095 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1142 * locals.var_xmp_dn7) * locals.var_dnm) + (assign53120_e81093 * locals.var_dnm_dn7)) * locals.var_arg) - (assign53120_e81095 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1142 * locals.var_xmp_dn8) * locals.var_dnm) + (assign53120_e81093 * locals.var_dnm_dn8)) * locals.var_arg) - (assign53120_e81095 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1142 * locals.var_xmp_dn9) * locals.var_dnm) + (assign53120_e81093 * locals.var_dnm_dn9)) * locals.var_arg) - (assign53120_e81095 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1142 * locals.var_xmp_dn10) * locals.var_dnm) + (assign53120_e81093 * locals.var_dnm_dn10)) * locals.var_arg) - (assign53120_e81095 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1142 * locals.var_xmp_dn13) * locals.var_dnm) + (assign53120_e81093 * locals.var_dnm_dn13)) * locals.var_arg) - (assign53120_e81095 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign53120_e81099;
        locals.var_t0_dn0 = assign53120_e81099_d_n0;
        locals.var_t0_dn2 = assign53120_e81099_d_n2;
        locals.var_t0_dn4 = assign53120_e81099_d_n4;
        locals.var_t0_dn5 = assign53120_e81099_d_n5;
        locals.var_t0_dn6 = assign53120_e81099_d_n6;
        locals.var_t0_dn7 = assign53120_e81099_d_n7;
        locals.var_t0_dn8 = assign53120_e81099_d_n8;
        locals.var_t0_dn9 = assign53120_e81099_d_n9;
        locals.var_t0_dn10 = assign53120_e81099_d_n10;
        locals.var_t0_dn13 = assign53120_e81099_d_n13;
        locals.var_t0_rv = 0.0;

    }
}
