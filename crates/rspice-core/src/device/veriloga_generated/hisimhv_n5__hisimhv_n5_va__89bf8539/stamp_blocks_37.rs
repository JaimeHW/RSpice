#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_200(
        locals: &mut StampLocals,
    ) {
        let (assign54950_e84951, assign54950_e84951_d_n0, assign54950_e84951_d_n2, assign54950_e84951_d_n4, assign54950_e84951_d_n5, assign54950_e84951_d_n6, assign54950_e84951_d_n7, assign54950_e84951_d_n8, assign54950_e84951_d_n9, assign54950_e84951_d_n10, assign54950_e84951_d_n11, assign54950_e84951_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1384 == 0.0)) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1386 == 0.0)) {
        let assign54950_e84948: f64 = (locals.var_tmf1).exp();
        let assign54950_e84949: f64 = (locals.var_t1 * assign54950_e84948);
        (assign54950_e84949, ((locals.var_t1_dn0 * assign54950_e84948) + (locals.var_t1 * (assign54950_e84948 * locals.var_tmf1_dn0))), ((locals.var_t1_dn2 * assign54950_e84948) + (locals.var_t1 * (assign54950_e84948 * locals.var_tmf1_dn2))), ((locals.var_t1_dn4 * assign54950_e84948) + (locals.var_t1 * (assign54950_e84948 * locals.var_tmf1_dn4))), ((locals.var_t1_dn5 * assign54950_e84948) + (locals.var_t1 * (assign54950_e84948 * locals.var_tmf1_dn5))), ((locals.var_t1_dn6 * assign54950_e84948) + (locals.var_t1 * (assign54950_e84948 * locals.var_tmf1_dn6))), ((locals.var_t1_dn7 * assign54950_e84948) + (locals.var_t1 * (assign54950_e84948 * locals.var_tmf1_dn7))), ((locals.var_t1_dn8 * assign54950_e84948) + (locals.var_t1 * (assign54950_e84948 * locals.var_tmf1_dn8))), ((locals.var_t1_dn9 * assign54950_e84948) + (locals.var_t1 * (assign54950_e84948 * locals.var_tmf1_dn9))), ((locals.var_t1_dn10 * assign54950_e84948) + (locals.var_t1 * (assign54950_e84948 * locals.var_tmf1_dn10))), ((locals.var_t1_dn11 * assign54950_e84948) + (locals.var_t1 * (assign54950_e84948 * locals.var_tmf1_dn11))), ((locals.var_t1_dn14 * assign54950_e84948) + (locals.var_t1 * (assign54950_e84948 * locals.var_tmf1_dn14))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign54950_e84951;
        locals.var_t1_dn0 = assign54950_e84951_d_n0;
        locals.var_t1_dn2 = assign54950_e84951_d_n2;
        locals.var_t1_dn4 = assign54950_e84951_d_n4;
        locals.var_t1_dn5 = assign54950_e84951_d_n5;
        locals.var_t1_dn6 = assign54950_e84951_d_n6;
        locals.var_t1_dn7 = assign54950_e84951_d_n7;
        locals.var_t1_dn8 = assign54950_e84951_d_n8;
        locals.var_t1_dn9 = assign54950_e84951_d_n9;
        locals.var_t1_dn10 = assign54950_e84951_d_n10;
        locals.var_t1_dn11 = assign54950_e84951_d_n11;
        locals.var_t1_dn14 = assign54950_e84951_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign54960_e84973, assign54960_e84973_d_n0, assign54960_e84973_d_n2, assign54960_e84973_d_n4, assign54960_e84973_d_n5, assign54960_e84973_d_n6, assign54960_e84973_d_n7, assign54960_e84973_d_n8, assign54960_e84973_d_n9, assign54960_e84973_d_n10, assign54960_e84973_d_n11, assign54960_e84973_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1384 == 0.0)) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1386 == 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign54960_e84973;
        locals.var_t3_dn0 = assign54960_e84973_d_n0;
        locals.var_t3_dn2 = assign54960_e84973_d_n2;
        locals.var_t3_dn4 = assign54960_e84973_d_n4;
        locals.var_t3_dn5 = assign54960_e84973_d_n5;
        locals.var_t3_dn6 = assign54960_e84973_d_n6;
        locals.var_t3_dn7 = assign54960_e84973_d_n7;
        locals.var_t3_dn8 = assign54960_e84973_d_n8;
        locals.var_t3_dn9 = assign54960_e84973_d_n9;
        locals.var_t3_dn10 = assign54960_e84973_d_n10;
        locals.var_t3_dn11 = assign54960_e84973_d_n11;
        locals.var_t3_dn14 = assign54960_e84973_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign54970_e84994, assign54970_e84994_d_n0, assign54970_e84994_d_n2, assign54970_e84994_d_n4, assign54970_e84994_d_n5, assign54970_e84994_d_n6, assign54970_e84994_d_n7, assign54970_e84994_d_n8, assign54970_e84994_d_n9, assign54970_e84994_d_n10, assign54970_e84994_d_n11, assign54970_e84994_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1384 == 0.0)) && (locals.var_guard1385 != 0.0)) {
        let assign54970_e84992: f64 = (locals.var_t1 * locals.var_t0);
        (assign54970_e84992, ((locals.var_t1_dn0 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn0)), ((locals.var_t1_dn2 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn2)), ((locals.var_t1_dn4 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn4)), ((locals.var_t1_dn5 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn5)), ((locals.var_t1_dn6 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn6)), ((locals.var_t1_dn7 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn7)), ((locals.var_t1_dn8 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn8)), ((locals.var_t1_dn9 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn9)), ((locals.var_t1_dn10 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn10)), ((locals.var_t1_dn11 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn11)), ((locals.var_t1_dn14 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign54970_e84994;
        locals.var_t1_dn0 = assign54970_e84994_d_n0;
        locals.var_t1_dn2 = assign54970_e84994_d_n2;
        locals.var_t1_dn4 = assign54970_e84994_d_n4;
        locals.var_t1_dn5 = assign54970_e84994_d_n5;
        locals.var_t1_dn6 = assign54970_e84994_d_n6;
        locals.var_t1_dn7 = assign54970_e84994_d_n7;
        locals.var_t1_dn8 = assign54970_e84994_d_n8;
        locals.var_t1_dn9 = assign54970_e84994_d_n9;
        locals.var_t1_dn10 = assign54970_e84994_d_n10;
        locals.var_t1_dn11 = assign54970_e84994_d_n11;
        locals.var_t1_dn14 = assign54970_e84994_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign54980_e85015, assign54980_e85015_d_n0, assign54980_e85015_d_n2, assign54980_e85015_d_n4, assign54980_e85015_d_n5, assign54980_e85015_d_n6, assign54980_e85015_d_n7, assign54980_e85015_d_n8, assign54980_e85015_d_n9, assign54980_e85015_d_n10, assign54980_e85015_d_n11, assign54980_e85015_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1384 == 0.0)) && (locals.var_guard1385 != 0.0)) {
        let assign54980_e85013: f64 = (locals.var_t1 - locals.var_t0);
        (assign54980_e85013, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn11 - locals.var_t0_dn11), (locals.var_t1_dn14 - locals.var_t0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign54980_e85015;
        locals.var_t2_dn0 = assign54980_e85015_d_n0;
        locals.var_t2_dn2 = assign54980_e85015_d_n2;
        locals.var_t2_dn4 = assign54980_e85015_d_n4;
        locals.var_t2_dn5 = assign54980_e85015_d_n5;
        locals.var_t2_dn6 = assign54980_e85015_d_n6;
        locals.var_t2_dn7 = assign54980_e85015_d_n7;
        locals.var_t2_dn8 = assign54980_e85015_d_n8;
        locals.var_t2_dn9 = assign54980_e85015_d_n9;
        locals.var_t2_dn10 = assign54980_e85015_d_n10;
        locals.var_t2_dn11 = assign54980_e85015_d_n11;
        locals.var_t2_dn14 = assign54980_e85015_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign54990_e85039, assign54990_e85039_d_n0, assign54990_e85039_d_n2, assign54990_e85039_d_n4, assign54990_e85039_d_n5, assign54990_e85039_d_n6, assign54990_e85039_d_n7, assign54990_e85039_d_n8, assign54990_e85039_d_n9, assign54990_e85039_d_n10, assign54990_e85039_d_n11, assign54990_e85039_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1384 == 0.0)) && (locals.var_guard1385 == 0.0)) {
        let assign54990_e85035: f64 = (1.0 + locals.var_tx);
        let assign54990_e85037: f64 = (assign54990_e85035 * locals.var_t0);
        (assign54990_e85037, ((locals.var_tx_dn0 * locals.var_t0) + (assign54990_e85035 * locals.var_t0_dn0)), ((locals.var_tx_dn2 * locals.var_t0) + (assign54990_e85035 * locals.var_t0_dn2)), ((locals.var_tx_dn4 * locals.var_t0) + (assign54990_e85035 * locals.var_t0_dn4)), ((locals.var_tx_dn5 * locals.var_t0) + (assign54990_e85035 * locals.var_t0_dn5)), ((locals.var_tx_dn6 * locals.var_t0) + (assign54990_e85035 * locals.var_t0_dn6)), ((locals.var_tx_dn7 * locals.var_t0) + (assign54990_e85035 * locals.var_t0_dn7)), ((locals.var_tx_dn8 * locals.var_t0) + (assign54990_e85035 * locals.var_t0_dn8)), ((locals.var_tx_dn9 * locals.var_t0) + (assign54990_e85035 * locals.var_t0_dn9)), ((locals.var_tx_dn10 * locals.var_t0) + (assign54990_e85035 * locals.var_t0_dn10)), ((locals.var_tx_dn11 * locals.var_t0) + (assign54990_e85035 * locals.var_t0_dn11)), ((locals.var_tx_dn14 * locals.var_t0) + (assign54990_e85035 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign54990_e85039;
        locals.var_t1_dn0 = assign54990_e85039_d_n0;
        locals.var_t1_dn2 = assign54990_e85039_d_n2;
        locals.var_t1_dn4 = assign54990_e85039_d_n4;
        locals.var_t1_dn5 = assign54990_e85039_d_n5;
        locals.var_t1_dn6 = assign54990_e85039_d_n6;
        locals.var_t1_dn7 = assign54990_e85039_d_n7;
        locals.var_t1_dn8 = assign54990_e85039_d_n8;
        locals.var_t1_dn9 = assign54990_e85039_d_n9;
        locals.var_t1_dn10 = assign54990_e85039_d_n10;
        locals.var_t1_dn11 = assign54990_e85039_d_n11;
        locals.var_t1_dn14 = assign54990_e85039_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign55000_e85067, assign55000_e85067_d_n0, assign55000_e85067_d_n2, assign55000_e85067_d_n4, assign55000_e85067_d_n5, assign55000_e85067_d_n6, assign55000_e85067_d_n7, assign55000_e85067_d_n8, assign55000_e85067_d_n9, assign55000_e85067_d_n10, assign55000_e85067_d_n11, assign55000_e85067_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1384 == 0.0)) && (locals.var_guard1385 == 0.0)) {
        let assign55000_e85061: f64 = (locals.var_tx / 2.0);
        let assign55000_e85062: f64 = (1.0 + assign55000_e85061);
        let assign55000_e85063: f64 = (locals.var_tx * assign55000_e85062);
        let assign55000_e85065: f64 = (assign55000_e85063 * locals.var_t0);
        (assign55000_e85065, ((((locals.var_tx_dn0 * assign55000_e85062) + (locals.var_tx * (locals.var_tx_dn0 / 2.0))) * locals.var_t0) + (assign55000_e85063 * locals.var_t0_dn0)), ((((locals.var_tx_dn2 * assign55000_e85062) + (locals.var_tx * (locals.var_tx_dn2 / 2.0))) * locals.var_t0) + (assign55000_e85063 * locals.var_t0_dn2)), ((((locals.var_tx_dn4 * assign55000_e85062) + (locals.var_tx * (locals.var_tx_dn4 / 2.0))) * locals.var_t0) + (assign55000_e85063 * locals.var_t0_dn4)), ((((locals.var_tx_dn5 * assign55000_e85062) + (locals.var_tx * (locals.var_tx_dn5 / 2.0))) * locals.var_t0) + (assign55000_e85063 * locals.var_t0_dn5)), ((((locals.var_tx_dn6 * assign55000_e85062) + (locals.var_tx * (locals.var_tx_dn6 / 2.0))) * locals.var_t0) + (assign55000_e85063 * locals.var_t0_dn6)), ((((locals.var_tx_dn7 * assign55000_e85062) + (locals.var_tx * (locals.var_tx_dn7 / 2.0))) * locals.var_t0) + (assign55000_e85063 * locals.var_t0_dn7)), ((((locals.var_tx_dn8 * assign55000_e85062) + (locals.var_tx * (locals.var_tx_dn8 / 2.0))) * locals.var_t0) + (assign55000_e85063 * locals.var_t0_dn8)), ((((locals.var_tx_dn9 * assign55000_e85062) + (locals.var_tx * (locals.var_tx_dn9 / 2.0))) * locals.var_t0) + (assign55000_e85063 * locals.var_t0_dn9)), ((((locals.var_tx_dn10 * assign55000_e85062) + (locals.var_tx * (locals.var_tx_dn10 / 2.0))) * locals.var_t0) + (assign55000_e85063 * locals.var_t0_dn10)), ((((locals.var_tx_dn11 * assign55000_e85062) + (locals.var_tx * (locals.var_tx_dn11 / 2.0))) * locals.var_t0) + (assign55000_e85063 * locals.var_t0_dn11)), ((((locals.var_tx_dn14 * assign55000_e85062) + (locals.var_tx * (locals.var_tx_dn14 / 2.0))) * locals.var_t0) + (assign55000_e85063 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign55000_e85067;
        locals.var_t2_dn0 = assign55000_e85067_d_n0;
        locals.var_t2_dn2 = assign55000_e85067_d_n2;
        locals.var_t2_dn4 = assign55000_e85067_d_n4;
        locals.var_t2_dn5 = assign55000_e85067_d_n5;
        locals.var_t2_dn6 = assign55000_e85067_d_n6;
        locals.var_t2_dn7 = assign55000_e85067_d_n7;
        locals.var_t2_dn8 = assign55000_e85067_d_n8;
        locals.var_t2_dn9 = assign55000_e85067_d_n9;
        locals.var_t2_dn10 = assign55000_e85067_d_n10;
        locals.var_t2_dn11 = assign55000_e85067_d_n11;
        locals.var_t2_dn14 = assign55000_e85067_d_n14;
        locals.var_t2_rv = 0.0;

        let assign55010_e85069: f64 = (locals.var_t2).abs();
        let assign55010_e85071: f64 = if assign55010_e85069 > 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard1387 = assign55010_e85071;
        locals.var_guard1387_rv = 0.0;

        let (assign55020_e85095, assign55020_e85095_d_n0, assign55020_e85095_d_n2, assign55020_e85095_d_n4, assign55020_e85095_d_n5, assign55020_e85095_d_n6, assign55020_e85095_d_n7, assign55020_e85095_d_n8, assign55020_e85095_d_n9, assign55020_e85095_d_n10, assign55020_e85095_d_n11, assign55020_e85095_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1384 == 0.0)) && (locals.var_guard1387 != 0.0)) {
        let assign55020_e85090: f64 = (1.0 + locals.var_t2);
        let assign55020_e85091: f64 = (assign55020_e85090).ln();
        let assign55020_e85093: f64 = (assign55020_e85091 / locals.var_c_sb__blk1323);
        (assign55020_e85093, ((((locals.var_t2_dn0 / assign55020_e85090) * locals.var_c_sb__blk1323) - (assign55020_e85091 * locals.var_c_sb__blk1323_dn0)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), ((((locals.var_t2_dn2 / assign55020_e85090) * locals.var_c_sb__blk1323) - (assign55020_e85091 * locals.var_c_sb__blk1323_dn2)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), ((((locals.var_t2_dn4 / assign55020_e85090) * locals.var_c_sb__blk1323) - (assign55020_e85091 * locals.var_c_sb__blk1323_dn4)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), ((((locals.var_t2_dn5 / assign55020_e85090) * locals.var_c_sb__blk1323) - (assign55020_e85091 * locals.var_c_sb__blk1323_dn5)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), ((((locals.var_t2_dn6 / assign55020_e85090) * locals.var_c_sb__blk1323) - (assign55020_e85091 * locals.var_c_sb__blk1323_dn6)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), ((((locals.var_t2_dn7 / assign55020_e85090) * locals.var_c_sb__blk1323) - (assign55020_e85091 * locals.var_c_sb__blk1323_dn7)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), ((((locals.var_t2_dn8 / assign55020_e85090) * locals.var_c_sb__blk1323) - (assign55020_e85091 * locals.var_c_sb__blk1323_dn8)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), ((((locals.var_t2_dn9 / assign55020_e85090) * locals.var_c_sb__blk1323) - (assign55020_e85091 * locals.var_c_sb__blk1323_dn9)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), ((((locals.var_t2_dn10 / assign55020_e85090) * locals.var_c_sb__blk1323) - (assign55020_e85091 * locals.var_c_sb__blk1323_dn10)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), ((((locals.var_t2_dn11 / assign55020_e85090) * locals.var_c_sb__blk1323) - (assign55020_e85091 * locals.var_c_sb__blk1323_dn11)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), ((((locals.var_t2_dn14 / assign55020_e85090) * locals.var_c_sb__blk1323) - (assign55020_e85091 * locals.var_c_sb__blk1323_dn14)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)),)
    } else {
        (locals.var_pb0dep__blk1167, locals.var_pb0dep__blk1167_dn0, locals.var_pb0dep__blk1167_dn2, locals.var_pb0dep__blk1167_dn4, locals.var_pb0dep__blk1167_dn5, locals.var_pb0dep__blk1167_dn6, locals.var_pb0dep__blk1167_dn7, locals.var_pb0dep__blk1167_dn8, locals.var_pb0dep__blk1167_dn9, locals.var_pb0dep__blk1167_dn10, locals.var_pb0dep__blk1167_dn11, locals.var_pb0dep__blk1167_dn14,)
    }
};
        locals.var_pb0dep__blk1167 = assign55020_e85095;
        locals.var_pb0dep__blk1167_dn0 = assign55020_e85095_d_n0;
        locals.var_pb0dep__blk1167_dn2 = assign55020_e85095_d_n2;
        locals.var_pb0dep__blk1167_dn4 = assign55020_e85095_d_n4;
        locals.var_pb0dep__blk1167_dn5 = assign55020_e85095_d_n5;
        locals.var_pb0dep__blk1167_dn6 = assign55020_e85095_d_n6;
        locals.var_pb0dep__blk1167_dn7 = assign55020_e85095_d_n7;
        locals.var_pb0dep__blk1167_dn8 = assign55020_e85095_d_n8;
        locals.var_pb0dep__blk1167_dn9 = assign55020_e85095_d_n9;
        locals.var_pb0dep__blk1167_dn10 = assign55020_e85095_d_n10;
        locals.var_pb0dep__blk1167_dn11 = assign55020_e85095_d_n11;
        locals.var_pb0dep__blk1167_dn14 = assign55020_e85095_d_n14;
        locals.var_pb0dep__blk1167_rv = 0.0;

        let (assign55030_e85117, assign55030_e85117_d_n0, assign55030_e85117_d_n2, assign55030_e85117_d_n4, assign55030_e85117_d_n5, assign55030_e85117_d_n6, assign55030_e85117_d_n7, assign55030_e85117_d_n8, assign55030_e85117_d_n9, assign55030_e85117_d_n10, assign55030_e85117_d_n11, assign55030_e85117_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1384 == 0.0)) && (locals.var_guard1387 == 0.0)) {
        let assign55030_e85115: f64 = (locals.var_t2 / locals.var_c_sb__blk1323);
        (assign55030_e85115, (((locals.var_t2_dn0 * locals.var_c_sb__blk1323) - (locals.var_t2 * locals.var_c_sb__blk1323_dn0)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), (((locals.var_t2_dn2 * locals.var_c_sb__blk1323) - (locals.var_t2 * locals.var_c_sb__blk1323_dn2)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), (((locals.var_t2_dn4 * locals.var_c_sb__blk1323) - (locals.var_t2 * locals.var_c_sb__blk1323_dn4)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), (((locals.var_t2_dn5 * locals.var_c_sb__blk1323) - (locals.var_t2 * locals.var_c_sb__blk1323_dn5)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), (((locals.var_t2_dn6 * locals.var_c_sb__blk1323) - (locals.var_t2 * locals.var_c_sb__blk1323_dn6)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), (((locals.var_t2_dn7 * locals.var_c_sb__blk1323) - (locals.var_t2 * locals.var_c_sb__blk1323_dn7)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), (((locals.var_t2_dn8 * locals.var_c_sb__blk1323) - (locals.var_t2 * locals.var_c_sb__blk1323_dn8)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), (((locals.var_t2_dn9 * locals.var_c_sb__blk1323) - (locals.var_t2 * locals.var_c_sb__blk1323_dn9)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), (((locals.var_t2_dn10 * locals.var_c_sb__blk1323) - (locals.var_t2 * locals.var_c_sb__blk1323_dn10)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), (((locals.var_t2_dn11 * locals.var_c_sb__blk1323) - (locals.var_t2 * locals.var_c_sb__blk1323_dn11)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)), (((locals.var_t2_dn14 * locals.var_c_sb__blk1323) - (locals.var_t2 * locals.var_c_sb__blk1323_dn14)) / (locals.var_c_sb__blk1323 * locals.var_c_sb__blk1323)),)
    } else {
        (locals.var_pb0dep__blk1167, locals.var_pb0dep__blk1167_dn0, locals.var_pb0dep__blk1167_dn2, locals.var_pb0dep__blk1167_dn4, locals.var_pb0dep__blk1167_dn5, locals.var_pb0dep__blk1167_dn6, locals.var_pb0dep__blk1167_dn7, locals.var_pb0dep__blk1167_dn8, locals.var_pb0dep__blk1167_dn9, locals.var_pb0dep__blk1167_dn10, locals.var_pb0dep__blk1167_dn11, locals.var_pb0dep__blk1167_dn14,)
    }
};
        locals.var_pb0dep__blk1167 = assign55030_e85117;
        locals.var_pb0dep__blk1167_dn0 = assign55030_e85117_d_n0;
        locals.var_pb0dep__blk1167_dn2 = assign55030_e85117_d_n2;
        locals.var_pb0dep__blk1167_dn4 = assign55030_e85117_d_n4;
        locals.var_pb0dep__blk1167_dn5 = assign55030_e85117_d_n5;
        locals.var_pb0dep__blk1167_dn6 = assign55030_e85117_d_n6;
        locals.var_pb0dep__blk1167_dn7 = assign55030_e85117_d_n7;
        locals.var_pb0dep__blk1167_dn8 = assign55030_e85117_d_n8;
        locals.var_pb0dep__blk1167_dn9 = assign55030_e85117_d_n9;
        locals.var_pb0dep__blk1167_dn10 = assign55030_e85117_d_n10;
        locals.var_pb0dep__blk1167_dn11 = assign55030_e85117_d_n11;
        locals.var_pb0dep__blk1167_dn14 = assign55030_e85117_d_n14;
        locals.var_pb0dep__blk1167_rv = 0.0;

        let (assign55040_e85133, assign55040_e85133_d_n0, assign55040_e85133_d_n2, assign55040_e85133_d_n4, assign55040_e85133_d_n5, assign55040_e85133_d_n6, assign55040_e85133_d_n7, assign55040_e85133_d_n8, assign55040_e85133_d_n9, assign55040_e85133_d_n10, assign55040_e85133_d_n11, assign55040_e85133_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) {
        let assign55040_e85131: f64 = (locals.var_ps0dep - locals.var_pb0dep__blk1167);
        (assign55040_e85131, (locals.var_ps0dep_dn0 - locals.var_pb0dep__blk1167_dn0), (locals.var_ps0dep_dn2 - locals.var_pb0dep__blk1167_dn2), (locals.var_ps0dep_dn4 - locals.var_pb0dep__blk1167_dn4), (locals.var_ps0dep_dn5 - locals.var_pb0dep__blk1167_dn5), (locals.var_ps0dep_dn6 - locals.var_pb0dep__blk1167_dn6), (locals.var_ps0dep_dn7 - locals.var_pb0dep__blk1167_dn7), (locals.var_ps0dep_dn8 - locals.var_pb0dep__blk1167_dn8), (locals.var_ps0dep_dn9 - locals.var_pb0dep__blk1167_dn9), (locals.var_ps0dep_dn10 - locals.var_pb0dep__blk1167_dn10), (locals.var_ps0dep_dn11 - locals.var_pb0dep__blk1167_dn11), (locals.var_ps0dep_dn14 - locals.var_pb0dep__blk1167_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign55040_e85133;
        locals.var_t2_dn0 = assign55040_e85133_d_n0;
        locals.var_t2_dn2 = assign55040_e85133_d_n2;
        locals.var_t2_dn4 = assign55040_e85133_d_n4;
        locals.var_t2_dn5 = assign55040_e85133_d_n5;
        locals.var_t2_dn6 = assign55040_e85133_d_n6;
        locals.var_t2_dn7 = assign55040_e85133_d_n7;
        locals.var_t2_dn8 = assign55040_e85133_d_n8;
        locals.var_t2_dn9 = assign55040_e85133_d_n9;
        locals.var_t2_dn10 = assign55040_e85133_d_n10;
        locals.var_t2_dn11 = assign55040_e85133_d_n11;
        locals.var_t2_dn14 = assign55040_e85133_d_n14;
        locals.var_t2_rv = 0.0;

        let assign55050_e85136: f64 = if 0.0 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1388 = assign55050_e85136;
        locals.var_guard1388_rv = 0.0;

        let (assign55060_e85165, assign55060_e85165_d_n0, assign55060_e85165_d_n2, assign55060_e85165_d_n4, assign55060_e85165_d_n5, assign55060_e85165_d_n6, assign55060_e85165_d_n7, assign55060_e85165_d_n8, assign55060_e85165_d_n9, assign55060_e85165_d_n10, assign55060_e85165_d_n11, assign55060_e85165_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1388 != 0.0)) {
        let (assign55060_e85163, assign55060_e85163_d_n0, assign55060_e85163_d_n2, assign55060_e85163_d_n4, assign55060_e85163_d_n5, assign55060_e85163_d_n6, assign55060_e85163_d_n7, assign55060_e85163_d_n8, assign55060_e85163_d_n9, assign55060_e85163_d_n10, assign55060_e85163_d_n11, assign55060_e85163_d_n14,) = {
            if (locals.var_t2 < 0.0) {
                let assign55060_e85154: f64 = (-locals.var_c_2esipq_ndepm__blk1138);
                let assign55060_e85156: f64 = (assign55060_e85154 * locals.var_t2);
                let assign55060_e85157: f64 = (assign55060_e85156).sqrt();
                let assign55060_e85158: f64 = (-assign55060_e85157);
                (assign55060_e85158, (-((((-locals.var_c_2esipq_ndepm__blk1138_dn0) * locals.var_t2) + (assign55060_e85154 * locals.var_t2_dn0)) / (2.0 * assign55060_e85157))), (-((((-locals.var_c_2esipq_ndepm__blk1138_dn2) * locals.var_t2) + (assign55060_e85154 * locals.var_t2_dn2)) / (2.0 * assign55060_e85157))), (-((((-locals.var_c_2esipq_ndepm__blk1138_dn4) * locals.var_t2) + (assign55060_e85154 * locals.var_t2_dn4)) / (2.0 * assign55060_e85157))), (-((((-locals.var_c_2esipq_ndepm__blk1138_dn5) * locals.var_t2) + (assign55060_e85154 * locals.var_t2_dn5)) / (2.0 * assign55060_e85157))), (-((((-locals.var_c_2esipq_ndepm__blk1138_dn6) * locals.var_t2) + (assign55060_e85154 * locals.var_t2_dn6)) / (2.0 * assign55060_e85157))), (-((((-locals.var_c_2esipq_ndepm__blk1138_dn7) * locals.var_t2) + (assign55060_e85154 * locals.var_t2_dn7)) / (2.0 * assign55060_e85157))), (-((((-locals.var_c_2esipq_ndepm__blk1138_dn8) * locals.var_t2) + (assign55060_e85154 * locals.var_t2_dn8)) / (2.0 * assign55060_e85157))), (-((((-locals.var_c_2esipq_ndepm__blk1138_dn9) * locals.var_t2) + (assign55060_e85154 * locals.var_t2_dn9)) / (2.0 * assign55060_e85157))), (-((((-locals.var_c_2esipq_ndepm__blk1138_dn10) * locals.var_t2) + (assign55060_e85154 * locals.var_t2_dn10)) / (2.0 * assign55060_e85157))), (-((((-locals.var_c_2esipq_ndepm__blk1138_dn11) * locals.var_t2) + (assign55060_e85154 * locals.var_t2_dn11)) / (2.0 * assign55060_e85157))), (-((((-locals.var_c_2esipq_ndepm__blk1138_dn14) * locals.var_t2) + (assign55060_e85154 * locals.var_t2_dn14)) / (2.0 * assign55060_e85157))),)
            } else {
                let assign55060_e85161: f64 = (locals.var_c_2esipq_ndepm__blk1138 * locals.var_t2);
                let assign55060_e85162: f64 = (assign55060_e85161).sqrt();
                (assign55060_e85162, (((locals.var_c_2esipq_ndepm__blk1138_dn0 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_t2_dn0)) / (2.0 * assign55060_e85162)), (((locals.var_c_2esipq_ndepm__blk1138_dn2 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_t2_dn2)) / (2.0 * assign55060_e85162)), (((locals.var_c_2esipq_ndepm__blk1138_dn4 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_t2_dn4)) / (2.0 * assign55060_e85162)), (((locals.var_c_2esipq_ndepm__blk1138_dn5 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_t2_dn5)) / (2.0 * assign55060_e85162)), (((locals.var_c_2esipq_ndepm__blk1138_dn6 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_t2_dn6)) / (2.0 * assign55060_e85162)), (((locals.var_c_2esipq_ndepm__blk1138_dn7 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_t2_dn7)) / (2.0 * assign55060_e85162)), (((locals.var_c_2esipq_ndepm__blk1138_dn8 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_t2_dn8)) / (2.0 * assign55060_e85162)), (((locals.var_c_2esipq_ndepm__blk1138_dn9 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_t2_dn9)) / (2.0 * assign55060_e85162)), (((locals.var_c_2esipq_ndepm__blk1138_dn10 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_t2_dn10)) / (2.0 * assign55060_e85162)), (((locals.var_c_2esipq_ndepm__blk1138_dn11 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_t2_dn11)) / (2.0 * assign55060_e85162)), (((locals.var_c_2esipq_ndepm__blk1138_dn14 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_t2_dn14)) / (2.0 * assign55060_e85162)),)
            }
        };
        (assign55060_e85163, assign55060_e85163_d_n0, assign55060_e85163_d_n2, assign55060_e85163_d_n4, assign55060_e85163_d_n5, assign55060_e85163_d_n6, assign55060_e85163_d_n7, assign55060_e85163_d_n8, assign55060_e85163_d_n9, assign55060_e85163_d_n10, assign55060_e85163_d_n11, assign55060_e85163_d_n14,)
    } else {
        (locals.var_ws__blk1149, locals.var_ws__blk1149_dn0, locals.var_ws__blk1149_dn2, locals.var_ws__blk1149_dn4, locals.var_ws__blk1149_dn5, locals.var_ws__blk1149_dn6, locals.var_ws__blk1149_dn7, locals.var_ws__blk1149_dn8, locals.var_ws__blk1149_dn9, locals.var_ws__blk1149_dn10, locals.var_ws__blk1149_dn11, locals.var_ws__blk1149_dn14,)
    }
};
        locals.var_ws__blk1149 = assign55060_e85165;
        locals.var_ws__blk1149_dn0 = assign55060_e85165_d_n0;
        locals.var_ws__blk1149_dn2 = assign55060_e85165_d_n2;
        locals.var_ws__blk1149_dn4 = assign55060_e85165_d_n4;
        locals.var_ws__blk1149_dn5 = assign55060_e85165_d_n5;
        locals.var_ws__blk1149_dn6 = assign55060_e85165_d_n6;
        locals.var_ws__blk1149_dn7 = assign55060_e85165_d_n7;
        locals.var_ws__blk1149_dn8 = assign55060_e85165_d_n8;
        locals.var_ws__blk1149_dn9 = assign55060_e85165_d_n9;
        locals.var_ws__blk1149_dn10 = assign55060_e85165_d_n10;
        locals.var_ws__blk1149_dn11 = assign55060_e85165_d_n11;
        locals.var_ws__blk1149_dn14 = assign55060_e85165_d_n14;
        locals.var_ws__blk1149_rv = 0.0;

        let assign55070_e85168: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1389 = assign55070_e85168;
        locals.var_guard1389_rv = 0.0;

        let (assign55080_e85189, assign55080_e85189_d_n0, assign55080_e85189_d_n2, assign55080_e85189_d_n4, assign55080_e85189_d_n5, assign55080_e85189_d_n6, assign55080_e85189_d_n7, assign55080_e85189_d_n8, assign55080_e85189_d_n9, assign55080_e85189_d_n10, assign55080_e85189_d_n11, assign55080_e85189_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1389 != 0.0)) {
        let assign55080_e85187: f64 = (locals.var_beta * locals.var_t2);
        (assign55080_e85187, ((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0)), ((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2)), ((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4)), ((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5)), ((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6)), ((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7)), ((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8)), ((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9)), ((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10)), ((locals.var_beta_dn11 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn11)), ((locals.var_beta_dn14 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign55080_e85189;
        locals.var_t3_dn0 = assign55080_e85189_d_n0;
        locals.var_t3_dn2 = assign55080_e85189_d_n2;
        locals.var_t3_dn4 = assign55080_e85189_d_n4;
        locals.var_t3_dn5 = assign55080_e85189_d_n5;
        locals.var_t3_dn6 = assign55080_e85189_d_n6;
        locals.var_t3_dn7 = assign55080_e85189_d_n7;
        locals.var_t3_dn8 = assign55080_e85189_d_n8;
        locals.var_t3_dn9 = assign55080_e85189_d_n9;
        locals.var_t3_dn10 = assign55080_e85189_d_n10;
        locals.var_t3_dn11 = assign55080_e85189_d_n11;
        locals.var_t3_dn14 = assign55080_e85189_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign55090_e85219, assign55090_e85219_d_n0, assign55090_e85219_d_n2, assign55090_e85219_d_n4, assign55090_e85219_d_n5, assign55090_e85219_d_n6, assign55090_e85219_d_n7, assign55090_e85219_d_n8, assign55090_e85219_d_n9, assign55090_e85219_d_n10, assign55090_e85219_d_n11, assign55090_e85219_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1389 != 0.0)) {
        let assign55090_e85208: f64 = (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv);
        let assign55090_e85210: f64 = (locals.var_t3).exp();
        let assign55090_e85212: f64 = (assign55090_e85210 - locals.var_t3);
        let assign55090_e85214: f64 = (assign55090_e85212 - 1.0);
        let assign55090_e85215: f64 = (assign55090_e85208 * assign55090_e85214);
        let assign55090_e85216: f64 = (assign55090_e85215).sqrt();
        let assign55090_e85217: f64 = (-assign55090_e85216);
        (assign55090_e85217, (-(((((locals.var_c_2esipq_ndepm__blk1138_dn0 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn0)) * assign55090_e85214) + (assign55090_e85208 * ((assign55090_e85210 * locals.var_t3_dn0) - locals.var_t3_dn0))) / (2.0 * assign55090_e85216))), (-(((((locals.var_c_2esipq_ndepm__blk1138_dn2 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn2)) * assign55090_e85214) + (assign55090_e85208 * ((assign55090_e85210 * locals.var_t3_dn2) - locals.var_t3_dn2))) / (2.0 * assign55090_e85216))), (-(((((locals.var_c_2esipq_ndepm__blk1138_dn4 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn4)) * assign55090_e85214) + (assign55090_e85208 * ((assign55090_e85210 * locals.var_t3_dn4) - locals.var_t3_dn4))) / (2.0 * assign55090_e85216))), (-(((((locals.var_c_2esipq_ndepm__blk1138_dn5 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn5)) * assign55090_e85214) + (assign55090_e85208 * ((assign55090_e85210 * locals.var_t3_dn5) - locals.var_t3_dn5))) / (2.0 * assign55090_e85216))), (-(((((locals.var_c_2esipq_ndepm__blk1138_dn6 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn6)) * assign55090_e85214) + (assign55090_e85208 * ((assign55090_e85210 * locals.var_t3_dn6) - locals.var_t3_dn6))) / (2.0 * assign55090_e85216))), (-(((((locals.var_c_2esipq_ndepm__blk1138_dn7 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn7)) * assign55090_e85214) + (assign55090_e85208 * ((assign55090_e85210 * locals.var_t3_dn7) - locals.var_t3_dn7))) / (2.0 * assign55090_e85216))), (-(((((locals.var_c_2esipq_ndepm__blk1138_dn8 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn8)) * assign55090_e85214) + (assign55090_e85208 * ((assign55090_e85210 * locals.var_t3_dn8) - locals.var_t3_dn8))) / (2.0 * assign55090_e85216))), (-(((((locals.var_c_2esipq_ndepm__blk1138_dn9 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn9)) * assign55090_e85214) + (assign55090_e85208 * ((assign55090_e85210 * locals.var_t3_dn9) - locals.var_t3_dn9))) / (2.0 * assign55090_e85216))), (-(((((locals.var_c_2esipq_ndepm__blk1138_dn10 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn10)) * assign55090_e85214) + (assign55090_e85208 * ((assign55090_e85210 * locals.var_t3_dn10) - locals.var_t3_dn10))) / (2.0 * assign55090_e85216))), (-(((((locals.var_c_2esipq_ndepm__blk1138_dn11 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn11)) * assign55090_e85214) + (assign55090_e85208 * ((assign55090_e85210 * locals.var_t3_dn11) - locals.var_t3_dn11))) / (2.0 * assign55090_e85216))), (-(((((locals.var_c_2esipq_ndepm__blk1138_dn14 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn14)) * assign55090_e85214) + (assign55090_e85208 * ((assign55090_e85210 * locals.var_t3_dn14) - locals.var_t3_dn14))) / (2.0 * assign55090_e85216))),)
    } else {
        (locals.var_ws__blk1149, locals.var_ws__blk1149_dn0, locals.var_ws__blk1149_dn2, locals.var_ws__blk1149_dn4, locals.var_ws__blk1149_dn5, locals.var_ws__blk1149_dn6, locals.var_ws__blk1149_dn7, locals.var_ws__blk1149_dn8, locals.var_ws__blk1149_dn9, locals.var_ws__blk1149_dn10, locals.var_ws__blk1149_dn11, locals.var_ws__blk1149_dn14,)
    }
};
        locals.var_ws__blk1149 = assign55090_e85219;
        locals.var_ws__blk1149_dn0 = assign55090_e85219_d_n0;
        locals.var_ws__blk1149_dn2 = assign55090_e85219_d_n2;
        locals.var_ws__blk1149_dn4 = assign55090_e85219_d_n4;
        locals.var_ws__blk1149_dn5 = assign55090_e85219_d_n5;
        locals.var_ws__blk1149_dn6 = assign55090_e85219_d_n6;
        locals.var_ws__blk1149_dn7 = assign55090_e85219_d_n7;
        locals.var_ws__blk1149_dn8 = assign55090_e85219_d_n8;
        locals.var_ws__blk1149_dn9 = assign55090_e85219_d_n9;
        locals.var_ws__blk1149_dn10 = assign55090_e85219_d_n10;
        locals.var_ws__blk1149_dn11 = assign55090_e85219_d_n11;
        locals.var_ws__blk1149_dn14 = assign55090_e85219_d_n14;
        locals.var_ws__blk1149_rv = 0.0;

        let (assign55100_e85242, assign55100_e85242_d_n0, assign55100_e85242_d_n2, assign55100_e85242_d_n4, assign55100_e85242_d_n5, assign55100_e85242_d_n6, assign55100_e85242_d_n7, assign55100_e85242_d_n8, assign55100_e85242_d_n9, assign55100_e85242_d_n10, assign55100_e85242_d_n11, assign55100_e85242_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1389 == 0.0)) {
        let assign55100_e85238: f64 = (-locals.var_beta);
        let assign55100_e85240: f64 = (assign55100_e85238 * locals.var_t2);
        (assign55100_e85240, (((-locals.var_beta_dn0) * locals.var_t2) + (assign55100_e85238 * locals.var_t2_dn0)), (((-locals.var_beta_dn2) * locals.var_t2) + (assign55100_e85238 * locals.var_t2_dn2)), (((-locals.var_beta_dn4) * locals.var_t2) + (assign55100_e85238 * locals.var_t2_dn4)), (((-locals.var_beta_dn5) * locals.var_t2) + (assign55100_e85238 * locals.var_t2_dn5)), (((-locals.var_beta_dn6) * locals.var_t2) + (assign55100_e85238 * locals.var_t2_dn6)), (((-locals.var_beta_dn7) * locals.var_t2) + (assign55100_e85238 * locals.var_t2_dn7)), (((-locals.var_beta_dn8) * locals.var_t2) + (assign55100_e85238 * locals.var_t2_dn8)), (((-locals.var_beta_dn9) * locals.var_t2) + (assign55100_e85238 * locals.var_t2_dn9)), (((-locals.var_beta_dn10) * locals.var_t2) + (assign55100_e85238 * locals.var_t2_dn10)), (((-locals.var_beta_dn11) * locals.var_t2) + (assign55100_e85238 * locals.var_t2_dn11)), (((-locals.var_beta_dn14) * locals.var_t2) + (assign55100_e85238 * locals.var_t2_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign55100_e85242;
        locals.var_t3_dn0 = assign55100_e85242_d_n0;
        locals.var_t3_dn2 = assign55100_e85242_d_n2;
        locals.var_t3_dn4 = assign55100_e85242_d_n4;
        locals.var_t3_dn5 = assign55100_e85242_d_n5;
        locals.var_t3_dn6 = assign55100_e85242_d_n6;
        locals.var_t3_dn7 = assign55100_e85242_d_n7;
        locals.var_t3_dn8 = assign55100_e85242_d_n8;
        locals.var_t3_dn9 = assign55100_e85242_d_n9;
        locals.var_t3_dn10 = assign55100_e85242_d_n10;
        locals.var_t3_dn11 = assign55100_e85242_d_n11;
        locals.var_t3_dn14 = assign55100_e85242_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign55110_e85272, assign55110_e85272_d_n0, assign55110_e85272_d_n2, assign55110_e85272_d_n4, assign55110_e85272_d_n5, assign55110_e85272_d_n6, assign55110_e85272_d_n7, assign55110_e85272_d_n8, assign55110_e85272_d_n9, assign55110_e85272_d_n10, assign55110_e85272_d_n11, assign55110_e85272_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1389 == 0.0)) {
        let assign55110_e85262: f64 = (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv);
        let assign55110_e85264: f64 = (locals.var_t3).exp();
        let assign55110_e85266: f64 = (assign55110_e85264 - locals.var_t3);
        let assign55110_e85268: f64 = (assign55110_e85266 - 1.0);
        let assign55110_e85269: f64 = (assign55110_e85262 * assign55110_e85268);
        let assign55110_e85270: f64 = (assign55110_e85269).sqrt();
        (assign55110_e85270, (((((locals.var_c_2esipq_ndepm__blk1138_dn0 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn0)) * assign55110_e85268) + (assign55110_e85262 * ((assign55110_e85264 * locals.var_t3_dn0) - locals.var_t3_dn0))) / (2.0 * assign55110_e85270)), (((((locals.var_c_2esipq_ndepm__blk1138_dn2 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn2)) * assign55110_e85268) + (assign55110_e85262 * ((assign55110_e85264 * locals.var_t3_dn2) - locals.var_t3_dn2))) / (2.0 * assign55110_e85270)), (((((locals.var_c_2esipq_ndepm__blk1138_dn4 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn4)) * assign55110_e85268) + (assign55110_e85262 * ((assign55110_e85264 * locals.var_t3_dn4) - locals.var_t3_dn4))) / (2.0 * assign55110_e85270)), (((((locals.var_c_2esipq_ndepm__blk1138_dn5 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn5)) * assign55110_e85268) + (assign55110_e85262 * ((assign55110_e85264 * locals.var_t3_dn5) - locals.var_t3_dn5))) / (2.0 * assign55110_e85270)), (((((locals.var_c_2esipq_ndepm__blk1138_dn6 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn6)) * assign55110_e85268) + (assign55110_e85262 * ((assign55110_e85264 * locals.var_t3_dn6) - locals.var_t3_dn6))) / (2.0 * assign55110_e85270)), (((((locals.var_c_2esipq_ndepm__blk1138_dn7 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn7)) * assign55110_e85268) + (assign55110_e85262 * ((assign55110_e85264 * locals.var_t3_dn7) - locals.var_t3_dn7))) / (2.0 * assign55110_e85270)), (((((locals.var_c_2esipq_ndepm__blk1138_dn8 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn8)) * assign55110_e85268) + (assign55110_e85262 * ((assign55110_e85264 * locals.var_t3_dn8) - locals.var_t3_dn8))) / (2.0 * assign55110_e85270)), (((((locals.var_c_2esipq_ndepm__blk1138_dn9 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn9)) * assign55110_e85268) + (assign55110_e85262 * ((assign55110_e85264 * locals.var_t3_dn9) - locals.var_t3_dn9))) / (2.0 * assign55110_e85270)), (((((locals.var_c_2esipq_ndepm__blk1138_dn10 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn10)) * assign55110_e85268) + (assign55110_e85262 * ((assign55110_e85264 * locals.var_t3_dn10) - locals.var_t3_dn10))) / (2.0 * assign55110_e85270)), (((((locals.var_c_2esipq_ndepm__blk1138_dn11 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn11)) * assign55110_e85268) + (assign55110_e85262 * ((assign55110_e85264 * locals.var_t3_dn11) - locals.var_t3_dn11))) / (2.0 * assign55110_e85270)), (((((locals.var_c_2esipq_ndepm__blk1138_dn14 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1138 * locals.var_beta_inv_dn14)) * assign55110_e85268) + (assign55110_e85262 * ((assign55110_e85264 * locals.var_t3_dn14) - locals.var_t3_dn14))) / (2.0 * assign55110_e85270)),)
    } else {
        (locals.var_ws__blk1149, locals.var_ws__blk1149_dn0, locals.var_ws__blk1149_dn2, locals.var_ws__blk1149_dn4, locals.var_ws__blk1149_dn5, locals.var_ws__blk1149_dn6, locals.var_ws__blk1149_dn7, locals.var_ws__blk1149_dn8, locals.var_ws__blk1149_dn9, locals.var_ws__blk1149_dn10, locals.var_ws__blk1149_dn11, locals.var_ws__blk1149_dn14,)
    }
};
        locals.var_ws__blk1149 = assign55110_e85272;
        locals.var_ws__blk1149_dn0 = assign55110_e85272_d_n0;
        locals.var_ws__blk1149_dn2 = assign55110_e85272_d_n2;
        locals.var_ws__blk1149_dn4 = assign55110_e85272_d_n4;
        locals.var_ws__blk1149_dn5 = assign55110_e85272_d_n5;
        locals.var_ws__blk1149_dn6 = assign55110_e85272_d_n6;
        locals.var_ws__blk1149_dn7 = assign55110_e85272_d_n7;
        locals.var_ws__blk1149_dn8 = assign55110_e85272_d_n8;
        locals.var_ws__blk1149_dn9 = assign55110_e85272_d_n9;
        locals.var_ws__blk1149_dn10 = assign55110_e85272_d_n10;
        locals.var_ws__blk1149_dn11 = assign55110_e85272_d_n11;
        locals.var_ws__blk1149_dn14 = assign55110_e85272_d_n14;
        locals.var_ws__blk1149_rv = 0.0;

        let (assign55120_e85288, assign55120_e85288_d_n0, assign55120_e85288_d_n2, assign55120_e85288_d_n4, assign55120_e85288_d_n5, assign55120_e85288_d_n6, assign55120_e85288_d_n7, assign55120_e85288_d_n8, assign55120_e85288_d_n9, assign55120_e85288_d_n10, assign55120_e85288_d_n11, assign55120_e85288_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) {
        let assign55120_e85286: f64 = (locals.var_tnp__blk1150 - locals.var_ws__blk1149);
        (assign55120_e85286, (locals.var_tnp__blk1150_dn0 - locals.var_ws__blk1149_dn0), (locals.var_tnp__blk1150_dn2 - locals.var_ws__blk1149_dn2), (locals.var_tnp__blk1150_dn4 - locals.var_ws__blk1149_dn4), (locals.var_tnp__blk1150_dn5 - locals.var_ws__blk1149_dn5), (locals.var_tnp__blk1150_dn6 - locals.var_ws__blk1149_dn6), (locals.var_tnp__blk1150_dn7 - locals.var_ws__blk1149_dn7), (locals.var_tnp__blk1150_dn8 - locals.var_ws__blk1149_dn8), (locals.var_tnp__blk1150_dn9 - locals.var_ws__blk1149_dn9), (locals.var_tnp__blk1150_dn10 - locals.var_ws__blk1149_dn10), (locals.var_tnp__blk1150_dn11 - locals.var_ws__blk1149_dn11), (locals.var_tnp__blk1150_dn14 - locals.var_ws__blk1149_dn14),)
    } else {
        (locals.var_w_res, locals.var_w_res_dn0, locals.var_w_res_dn2, locals.var_w_res_dn4, locals.var_w_res_dn5, locals.var_w_res_dn6, locals.var_w_res_dn7, locals.var_w_res_dn8, locals.var_w_res_dn9, locals.var_w_res_dn10, locals.var_w_res_dn11, locals.var_w_res_dn14,)
    }
};
        locals.var_w_res = assign55120_e85288;
        locals.var_w_res_dn0 = assign55120_e85288_d_n0;
        locals.var_w_res_dn2 = assign55120_e85288_d_n2;
        locals.var_w_res_dn4 = assign55120_e85288_d_n4;
        locals.var_w_res_dn5 = assign55120_e85288_d_n5;
        locals.var_w_res_dn6 = assign55120_e85288_d_n6;
        locals.var_w_res_dn7 = assign55120_e85288_d_n7;
        locals.var_w_res_dn8 = assign55120_e85288_d_n8;
        locals.var_w_res_dn9 = assign55120_e85288_d_n9;
        locals.var_w_res_dn10 = assign55120_e85288_d_n10;
        locals.var_w_res_dn11 = assign55120_e85288_d_n11;
        locals.var_w_res_dn14 = assign55120_e85288_d_n14;
        locals.var_w_res_rv = 0.0;

        let assign55130_e85292: f64 = 1e-16;
        let assign55130_e85297: f64 = if ((locals.var_w_res < assign55130_e85292) && (1e-16 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1390 = assign55130_e85297;
        locals.var_guard1390_rv = 0.0;

        let (assign55140_e85317, assign55140_e85317_d_n0, assign55140_e85317_d_n2, assign55140_e85317_d_n4, assign55140_e85317_d_n5, assign55140_e85317_d_n6, assign55140_e85317_d_n7, assign55140_e85317_d_n8, assign55140_e85317_d_n9, assign55140_e85317_d_n10, assign55140_e85317_d_n11, assign55140_e85317_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1390 != 0.0)) {
        let assign55140_e85313: f64 = 1e-16;
        let assign55140_e85315: f64 = (assign55140_e85313 - locals.var_w_res);
        (assign55140_e85315, (-locals.var_w_res_dn0), (-locals.var_w_res_dn2), (-locals.var_w_res_dn4), (-locals.var_w_res_dn5), (-locals.var_w_res_dn6), (-locals.var_w_res_dn7), (-locals.var_w_res_dn8), (-locals.var_w_res_dn9), (-locals.var_w_res_dn10), (-locals.var_w_res_dn11), (-locals.var_w_res_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign55140_e85317;
        locals.var_tmf1_dn0 = assign55140_e85317_d_n0;
        locals.var_tmf1_dn2 = assign55140_e85317_d_n2;
        locals.var_tmf1_dn4 = assign55140_e85317_d_n4;
        locals.var_tmf1_dn5 = assign55140_e85317_d_n5;
        locals.var_tmf1_dn6 = assign55140_e85317_d_n6;
        locals.var_tmf1_dn7 = assign55140_e85317_d_n7;
        locals.var_tmf1_dn8 = assign55140_e85317_d_n8;
        locals.var_tmf1_dn9 = assign55140_e85317_d_n9;
        locals.var_tmf1_dn10 = assign55140_e85317_d_n10;
        locals.var_tmf1_dn11 = assign55140_e85317_d_n11;
        locals.var_tmf1_dn14 = assign55140_e85317_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign55150_e85335, assign55150_e85335_d_n0, assign55150_e85335_d_n2, assign55150_e85335_d_n4, assign55150_e85335_d_n5, assign55150_e85335_d_n6, assign55150_e85335_d_n7, assign55150_e85335_d_n8, assign55150_e85335_d_n9, assign55150_e85335_d_n10, assign55150_e85335_d_n11, assign55150_e85335_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1390 != 0.0)) {
        let assign55150_e85333: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign55150_e85333, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign55150_e85335;
        locals.var_x2_dn0 = assign55150_e85335_d_n0;
        locals.var_x2_dn2 = assign55150_e85335_d_n2;
        locals.var_x2_dn4 = assign55150_e85335_d_n4;
        locals.var_x2_dn5 = assign55150_e85335_d_n5;
        locals.var_x2_dn6 = assign55150_e85335_d_n6;
        locals.var_x2_dn7 = assign55150_e85335_d_n7;
        locals.var_x2_dn8 = assign55150_e85335_d_n8;
        locals.var_x2_dn9 = assign55150_e85335_d_n9;
        locals.var_x2_dn10 = assign55150_e85335_d_n10;
        locals.var_x2_dn11 = assign55150_e85335_d_n11;
        locals.var_x2_dn14 = assign55150_e85335_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign55160_e85353, assign55160_e85353_d_n0, assign55160_e85353_d_n2, assign55160_e85353_d_n4, assign55160_e85353_d_n5, assign55160_e85353_d_n6, assign55160_e85353_d_n7, assign55160_e85353_d_n8, assign55160_e85353_d_n9, assign55160_e85353_d_n10, assign55160_e85353_d_n11, assign55160_e85353_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1390 != 0.0)) {
        let assign55160_e85351: f64 = (1e-16 * 1e-16);
        (assign55160_e85351, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign55160_e85353;
        locals.var_xmax2_dn0 = assign55160_e85353_d_n0;
        locals.var_xmax2_dn2 = assign55160_e85353_d_n2;
        locals.var_xmax2_dn4 = assign55160_e85353_d_n4;
        locals.var_xmax2_dn5 = assign55160_e85353_d_n5;
        locals.var_xmax2_dn6 = assign55160_e85353_d_n6;
        locals.var_xmax2_dn7 = assign55160_e85353_d_n7;
        locals.var_xmax2_dn8 = assign55160_e85353_d_n8;
        locals.var_xmax2_dn9 = assign55160_e85353_d_n9;
        locals.var_xmax2_dn10 = assign55160_e85353_d_n10;
        locals.var_xmax2_dn11 = assign55160_e85353_d_n11;
        locals.var_xmax2_dn14 = assign55160_e85353_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign55170_e85369, assign55170_e85369_d_n0, assign55170_e85369_d_n2, assign55170_e85369_d_n4, assign55170_e85369_d_n5, assign55170_e85369_d_n6, assign55170_e85369_d_n7, assign55170_e85369_d_n8, assign55170_e85369_d_n9, assign55170_e85369_d_n10, assign55170_e85369_d_n11, assign55170_e85369_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1390 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign55170_e85369;
        locals.var_xp_dn0 = assign55170_e85369_d_n0;
        locals.var_xp_dn2 = assign55170_e85369_d_n2;
        locals.var_xp_dn4 = assign55170_e85369_d_n4;
        locals.var_xp_dn5 = assign55170_e85369_d_n5;
        locals.var_xp_dn6 = assign55170_e85369_d_n6;
        locals.var_xp_dn7 = assign55170_e85369_d_n7;
        locals.var_xp_dn8 = assign55170_e85369_d_n8;
        locals.var_xp_dn9 = assign55170_e85369_d_n9;
        locals.var_xp_dn10 = assign55170_e85369_d_n10;
        locals.var_xp_dn11 = assign55170_e85369_d_n11;
        locals.var_xp_dn14 = assign55170_e85369_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign55180_e85385, assign55180_e85385_d_n0, assign55180_e85385_d_n2, assign55180_e85385_d_n4, assign55180_e85385_d_n5, assign55180_e85385_d_n6, assign55180_e85385_d_n7, assign55180_e85385_d_n8, assign55180_e85385_d_n9, assign55180_e85385_d_n10, assign55180_e85385_d_n11, assign55180_e85385_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1390 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign55180_e85385;
        locals.var_xmp_dn0 = assign55180_e85385_d_n0;
        locals.var_xmp_dn2 = assign55180_e85385_d_n2;
        locals.var_xmp_dn4 = assign55180_e85385_d_n4;
        locals.var_xmp_dn5 = assign55180_e85385_d_n5;
        locals.var_xmp_dn6 = assign55180_e85385_d_n6;
        locals.var_xmp_dn7 = assign55180_e85385_d_n7;
        locals.var_xmp_dn8 = assign55180_e85385_d_n8;
        locals.var_xmp_dn9 = assign55180_e85385_d_n9;
        locals.var_xmp_dn10 = assign55180_e85385_d_n10;
        locals.var_xmp_dn11 = assign55180_e85385_d_n11;
        locals.var_xmp_dn14 = assign55180_e85385_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign55190_e85401,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1390 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign55190_e85401;
        locals.var_m0_rv = 0.0;

        let (assign55200_e85417,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1390 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign55200_e85417;
        locals.var_mm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_201(
        locals: &mut StampLocals,
    ) {
        let (assign55210_e85433, assign55210_e85433_d_n0, assign55210_e85433_d_n2, assign55210_e85433_d_n4, assign55210_e85433_d_n5, assign55210_e85433_d_n6, assign55210_e85433_d_n7, assign55210_e85433_d_n8, assign55210_e85433_d_n9, assign55210_e85433_d_n10, assign55210_e85433_d_n11, assign55210_e85433_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1390 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign55210_e85433;
        locals.var_arg_dn0 = assign55210_e85433_d_n0;
        locals.var_arg_dn2 = assign55210_e85433_d_n2;
        locals.var_arg_dn4 = assign55210_e85433_d_n4;
        locals.var_arg_dn5 = assign55210_e85433_d_n5;
        locals.var_arg_dn6 = assign55210_e85433_d_n6;
        locals.var_arg_dn7 = assign55210_e85433_d_n7;
        locals.var_arg_dn8 = assign55210_e85433_d_n8;
        locals.var_arg_dn9 = assign55210_e85433_d_n9;
        locals.var_arg_dn10 = assign55210_e85433_d_n10;
        locals.var_arg_dn11 = assign55210_e85433_d_n11;
        locals.var_arg_dn14 = assign55210_e85433_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign55220_e85449, assign55220_e85449_d_n0, assign55220_e85449_d_n2, assign55220_e85449_d_n4, assign55220_e85449_d_n5, assign55220_e85449_d_n6, assign55220_e85449_d_n7, assign55220_e85449_d_n8, assign55220_e85449_d_n9, assign55220_e85449_d_n10, assign55220_e85449_d_n11, assign55220_e85449_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1390 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign55220_e85449;
        locals.var_dnm_dn0 = assign55220_e85449_d_n0;
        locals.var_dnm_dn2 = assign55220_e85449_d_n2;
        locals.var_dnm_dn4 = assign55220_e85449_d_n4;
        locals.var_dnm_dn5 = assign55220_e85449_d_n5;
        locals.var_dnm_dn6 = assign55220_e85449_d_n6;
        locals.var_dnm_dn7 = assign55220_e85449_d_n7;
        locals.var_dnm_dn8 = assign55220_e85449_d_n8;
        locals.var_dnm_dn9 = assign55220_e85449_d_n9;
        locals.var_dnm_dn10 = assign55220_e85449_d_n10;
        locals.var_dnm_dn11 = assign55220_e85449_d_n11;
        locals.var_dnm_dn14 = assign55220_e85449_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign55230_e85467, assign55230_e85467_d_n0, assign55230_e85467_d_n2, assign55230_e85467_d_n4, assign55230_e85467_d_n5, assign55230_e85467_d_n6, assign55230_e85467_d_n7, assign55230_e85467_d_n8, assign55230_e85467_d_n9, assign55230_e85467_d_n10, assign55230_e85467_d_n11, assign55230_e85467_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1390 != 0.0)) {
        let assign55230_e85465: f64 = (locals.var_xp * locals.var_x2);
        (assign55230_e85465, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign55230_e85467;
        locals.var_xp_dn0 = assign55230_e85467_d_n0;
        locals.var_xp_dn2 = assign55230_e85467_d_n2;
        locals.var_xp_dn4 = assign55230_e85467_d_n4;
        locals.var_xp_dn5 = assign55230_e85467_d_n5;
        locals.var_xp_dn6 = assign55230_e85467_d_n6;
        locals.var_xp_dn7 = assign55230_e85467_d_n7;
        locals.var_xp_dn8 = assign55230_e85467_d_n8;
        locals.var_xp_dn9 = assign55230_e85467_d_n9;
        locals.var_xp_dn10 = assign55230_e85467_d_n10;
        locals.var_xp_dn11 = assign55230_e85467_d_n11;
        locals.var_xp_dn14 = assign55230_e85467_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign55240_e85485, assign55240_e85485_d_n0, assign55240_e85485_d_n2, assign55240_e85485_d_n4, assign55240_e85485_d_n5, assign55240_e85485_d_n6, assign55240_e85485_d_n7, assign55240_e85485_d_n8, assign55240_e85485_d_n9, assign55240_e85485_d_n10, assign55240_e85485_d_n11, assign55240_e85485_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1390 != 0.0)) {
        let assign55240_e85483: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign55240_e85483, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign55240_e85485;
        locals.var_xmp_dn0 = assign55240_e85485_d_n0;
        locals.var_xmp_dn2 = assign55240_e85485_d_n2;
        locals.var_xmp_dn4 = assign55240_e85485_d_n4;
        locals.var_xmp_dn5 = assign55240_e85485_d_n5;
        locals.var_xmp_dn6 = assign55240_e85485_d_n6;
        locals.var_xmp_dn7 = assign55240_e85485_d_n7;
        locals.var_xmp_dn8 = assign55240_e85485_d_n8;
        locals.var_xmp_dn9 = assign55240_e85485_d_n9;
        locals.var_xmp_dn10 = assign55240_e85485_d_n10;
        locals.var_xmp_dn11 = assign55240_e85485_d_n11;
        locals.var_xmp_dn14 = assign55240_e85485_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign55250_e85503, assign55250_e85503_d_n0, assign55250_e85503_d_n2, assign55250_e85503_d_n4, assign55250_e85503_d_n5, assign55250_e85503_d_n6, assign55250_e85503_d_n7, assign55250_e85503_d_n8, assign55250_e85503_d_n9, assign55250_e85503_d_n10, assign55250_e85503_d_n11, assign55250_e85503_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1390 != 0.0)) {
        let assign55250_e85501: f64 = (locals.var_xp * locals.var_x2);
        (assign55250_e85501, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign55250_e85503;
        locals.var_xp_dn0 = assign55250_e85503_d_n0;
        locals.var_xp_dn2 = assign55250_e85503_d_n2;
        locals.var_xp_dn4 = assign55250_e85503_d_n4;
        locals.var_xp_dn5 = assign55250_e85503_d_n5;
        locals.var_xp_dn6 = assign55250_e85503_d_n6;
        locals.var_xp_dn7 = assign55250_e85503_d_n7;
        locals.var_xp_dn8 = assign55250_e85503_d_n8;
        locals.var_xp_dn9 = assign55250_e85503_d_n9;
        locals.var_xp_dn10 = assign55250_e85503_d_n10;
        locals.var_xp_dn11 = assign55250_e85503_d_n11;
        locals.var_xp_dn14 = assign55250_e85503_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign55260_e85521, assign55260_e85521_d_n0, assign55260_e85521_d_n2, assign55260_e85521_d_n4, assign55260_e85521_d_n5, assign55260_e85521_d_n6, assign55260_e85521_d_n7, assign55260_e85521_d_n8, assign55260_e85521_d_n9, assign55260_e85521_d_n10, assign55260_e85521_d_n11, assign55260_e85521_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1390 != 0.0)) {
        let assign55260_e85519: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign55260_e85519, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign55260_e85521;
        locals.var_xmp_dn0 = assign55260_e85521_d_n0;
        locals.var_xmp_dn2 = assign55260_e85521_d_n2;
        locals.var_xmp_dn4 = assign55260_e85521_d_n4;
        locals.var_xmp_dn5 = assign55260_e85521_d_n5;
        locals.var_xmp_dn6 = assign55260_e85521_d_n6;
        locals.var_xmp_dn7 = assign55260_e85521_d_n7;
        locals.var_xmp_dn8 = assign55260_e85521_d_n8;
        locals.var_xmp_dn9 = assign55260_e85521_d_n9;
        locals.var_xmp_dn10 = assign55260_e85521_d_n10;
        locals.var_xmp_dn11 = assign55260_e85521_d_n11;
        locals.var_xmp_dn14 = assign55260_e85521_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign55270_e85539, assign55270_e85539_d_n0, assign55270_e85539_d_n2, assign55270_e85539_d_n4, assign55270_e85539_d_n5, assign55270_e85539_d_n6, assign55270_e85539_d_n7, assign55270_e85539_d_n8, assign55270_e85539_d_n9, assign55270_e85539_d_n10, assign55270_e85539_d_n11, assign55270_e85539_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1390 != 0.0)) {
        let assign55270_e85537: f64 = (locals.var_xp + locals.var_xmp);
        (assign55270_e85537, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign55270_e85539;
        locals.var_arg_dn0 = assign55270_e85539_d_n0;
        locals.var_arg_dn2 = assign55270_e85539_d_n2;
        locals.var_arg_dn4 = assign55270_e85539_d_n4;
        locals.var_arg_dn5 = assign55270_e85539_d_n5;
        locals.var_arg_dn6 = assign55270_e85539_d_n6;
        locals.var_arg_dn7 = assign55270_e85539_d_n7;
        locals.var_arg_dn8 = assign55270_e85539_d_n8;
        locals.var_arg_dn9 = assign55270_e85539_d_n9;
        locals.var_arg_dn10 = assign55270_e85539_d_n10;
        locals.var_arg_dn11 = assign55270_e85539_d_n11;
        locals.var_arg_dn14 = assign55270_e85539_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign55280_e85555, assign55280_e85555_d_n0, assign55280_e85555_d_n2, assign55280_e85555_d_n4, assign55280_e85555_d_n5, assign55280_e85555_d_n6, assign55280_e85555_d_n7, assign55280_e85555_d_n8, assign55280_e85555_d_n9, assign55280_e85555_d_n10, assign55280_e85555_d_n11, assign55280_e85555_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1390 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign55280_e85555;
        locals.var_dnm_dn0 = assign55280_e85555_d_n0;
        locals.var_dnm_dn2 = assign55280_e85555_d_n2;
        locals.var_dnm_dn4 = assign55280_e85555_d_n4;
        locals.var_dnm_dn5 = assign55280_e85555_d_n5;
        locals.var_dnm_dn6 = assign55280_e85555_d_n6;
        locals.var_dnm_dn7 = assign55280_e85555_d_n7;
        locals.var_dnm_dn8 = assign55280_e85555_d_n8;
        locals.var_dnm_dn9 = assign55280_e85555_d_n9;
        locals.var_dnm_dn10 = assign55280_e85555_d_n10;
        locals.var_dnm_dn11 = assign55280_e85555_d_n11;
        locals.var_dnm_dn14 = assign55280_e85555_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign55290_e85570: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1391 = assign55290_e85570;
        locals.var_guard1391_rv = 0.0;

        let assign55300_e85573: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1392 = assign55300_e85573;
        locals.var_guard1392_rv = 0.0;

        let (assign55310_e85593,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1390 != 0.0)) && (locals.var_guard1391 != 0.0)) && (locals.var_guard1392 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign55310_e85593;
        locals.var_mm_rv = 0.0;

        let assign55320_e85596: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1393 = assign55320_e85596;
        locals.var_guard1393_rv = 0.0;

        let (assign55330_e85619,) = {
    if (((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1390 != 0.0)) && (locals.var_guard1391 != 0.0)) && (locals.var_guard1392 == 0.0)) && (locals.var_guard1393 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign55330_e85619;
        locals.var_mm_rv = 0.0;

        let assign55340_e85622: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1394 = assign55340_e85622;
        locals.var_guard1394_rv = 0.0;

        let (assign55350_e85648,) = {
    if ((((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1390 != 0.0)) && (locals.var_guard1391 != 0.0)) && (locals.var_guard1392 == 0.0)) && (locals.var_guard1393 == 0.0)) && (locals.var_guard1394 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign55350_e85648;
        locals.var_mm_rv = 0.0;

        let assign55360_e85651: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1395 = assign55360_e85651;
        locals.var_guard1395_rv = 0.0;

        let (assign55370_e85680,) = {
    if (((((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1390 != 0.0)) && (locals.var_guard1391 != 0.0)) && (locals.var_guard1392 == 0.0)) && (locals.var_guard1393 == 0.0)) && (locals.var_guard1394 == 0.0)) && (locals.var_guard1395 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign55370_e85680;
        locals.var_mm_rv = 0.0;

        let (assign55380_e85698,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1390 != 0.0)) && (locals.var_guard1391 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign55380_e85698;
        locals.var_m0_rv = 0.0;

        let mut assign55390_loop_guard: usize = 0;
        while {
            let assign55390_cond_e85717: f64 = if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1390 != 0.0)) && (locals.var_guard1391 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign55390_cond_e85717 != 0.0
        } {
            assign55390_loop_guard += 1;
            assert!(assign55390_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign55390_body0_e85736, assign55390_body0_e85736_d_n0, assign55390_body0_e85736_d_n2, assign55390_body0_e85736_d_n4, assign55390_body0_e85736_d_n5, assign55390_body0_e85736_d_n6, assign55390_body0_e85736_d_n7, assign55390_body0_e85736_d_n8, assign55390_body0_e85736_d_n9, assign55390_body0_e85736_d_n10, assign55390_body0_e85736_d_n11, assign55390_body0_e85736_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1390 != 0.0)) && (locals.var_guard1391 != 0.0)) {
        let assign55390_body0_e85734: f64 = (locals.var_dnm).sqrt();
        (assign55390_body0_e85734, (locals.var_dnm_dn0 / (2.0 * assign55390_body0_e85734)), (locals.var_dnm_dn2 / (2.0 * assign55390_body0_e85734)), (locals.var_dnm_dn4 / (2.0 * assign55390_body0_e85734)), (locals.var_dnm_dn5 / (2.0 * assign55390_body0_e85734)), (locals.var_dnm_dn6 / (2.0 * assign55390_body0_e85734)), (locals.var_dnm_dn7 / (2.0 * assign55390_body0_e85734)), (locals.var_dnm_dn8 / (2.0 * assign55390_body0_e85734)), (locals.var_dnm_dn9 / (2.0 * assign55390_body0_e85734)), (locals.var_dnm_dn10 / (2.0 * assign55390_body0_e85734)), (locals.var_dnm_dn11 / (2.0 * assign55390_body0_e85734)), (locals.var_dnm_dn14 / (2.0 * assign55390_body0_e85734)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign55390_body0_e85736;
            locals.var_dnm_dn0 = assign55390_body0_e85736_d_n0;
            locals.var_dnm_dn2 = assign55390_body0_e85736_d_n2;
            locals.var_dnm_dn4 = assign55390_body0_e85736_d_n4;
            locals.var_dnm_dn5 = assign55390_body0_e85736_d_n5;
            locals.var_dnm_dn6 = assign55390_body0_e85736_d_n6;
            locals.var_dnm_dn7 = assign55390_body0_e85736_d_n7;
            locals.var_dnm_dn8 = assign55390_body0_e85736_d_n8;
            locals.var_dnm_dn9 = assign55390_body0_e85736_d_n9;
            locals.var_dnm_dn10 = assign55390_body0_e85736_d_n10;
            locals.var_dnm_dn11 = assign55390_body0_e85736_d_n11;
            locals.var_dnm_dn14 = assign55390_body0_e85736_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign55390_body1_e85756,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1390 != 0.0)) && (locals.var_guard1391 != 0.0)) {
        let assign55390_body1_e85754: f64 = (locals.var_m0 + 1.0);
        (assign55390_body1_e85754,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign55390_body1_e85756;
            locals.var_m0_rv = 0.0;
        }

        let (assign55400_e85786, assign55400_e85786_d_n0, assign55400_e85786_d_n2, assign55400_e85786_d_n4, assign55400_e85786_d_n5, assign55400_e85786_d_n6, assign55400_e85786_d_n7, assign55400_e85786_d_n8, assign55400_e85786_d_n9, assign55400_e85786_d_n10, assign55400_e85786_d_n11, assign55400_e85786_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1390 != 0.0)) && (locals.var_guard1391 == 0.0)) {
        let (assign55400_e85784, assign55400_e85784_d_n0, assign55400_e85784_d_n2, assign55400_e85784_d_n4, assign55400_e85784_d_n5, assign55400_e85784_d_n6, assign55400_e85784_d_n7, assign55400_e85784_d_n8, assign55400_e85784_d_n9, assign55400_e85784_d_n10, assign55400_e85784_d_n11, assign55400_e85784_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign55400_e85781: f64 = (2.0 * 2.0);
                let assign55400_e85782: f64 = (1.0 / assign55400_e85781);
                let assign55400_e85783: f64 = (locals.var_dnm).powf(assign55400_e85782);
                (assign55400_e85783, if 0.0 == 0.0 && ((assign55400_e85782) as f64).is_finite() && ((assign55400_e85782) as f64).fract() == 0.0 { if assign55400_e85782 == 0.0 { 0.0 } else { (assign55400_e85782 * ((locals.var_dnm).powf(assign55400_e85782 - 1.0) * locals.var_dnm_dn0)) } } else { (assign55400_e85783 * (assign55400_e85782 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55400_e85782) as f64).is_finite() && ((assign55400_e85782) as f64).fract() == 0.0 { if assign55400_e85782 == 0.0 { 0.0 } else { (assign55400_e85782 * ((locals.var_dnm).powf(assign55400_e85782 - 1.0) * locals.var_dnm_dn2)) } } else { (assign55400_e85783 * (assign55400_e85782 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55400_e85782) as f64).is_finite() && ((assign55400_e85782) as f64).fract() == 0.0 { if assign55400_e85782 == 0.0 { 0.0 } else { (assign55400_e85782 * ((locals.var_dnm).powf(assign55400_e85782 - 1.0) * locals.var_dnm_dn4)) } } else { (assign55400_e85783 * (assign55400_e85782 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55400_e85782) as f64).is_finite() && ((assign55400_e85782) as f64).fract() == 0.0 { if assign55400_e85782 == 0.0 { 0.0 } else { (assign55400_e85782 * ((locals.var_dnm).powf(assign55400_e85782 - 1.0) * locals.var_dnm_dn5)) } } else { (assign55400_e85783 * (assign55400_e85782 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55400_e85782) as f64).is_finite() && ((assign55400_e85782) as f64).fract() == 0.0 { if assign55400_e85782 == 0.0 { 0.0 } else { (assign55400_e85782 * ((locals.var_dnm).powf(assign55400_e85782 - 1.0) * locals.var_dnm_dn6)) } } else { (assign55400_e85783 * (assign55400_e85782 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55400_e85782) as f64).is_finite() && ((assign55400_e85782) as f64).fract() == 0.0 { if assign55400_e85782 == 0.0 { 0.0 } else { (assign55400_e85782 * ((locals.var_dnm).powf(assign55400_e85782 - 1.0) * locals.var_dnm_dn7)) } } else { (assign55400_e85783 * (assign55400_e85782 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55400_e85782) as f64).is_finite() && ((assign55400_e85782) as f64).fract() == 0.0 { if assign55400_e85782 == 0.0 { 0.0 } else { (assign55400_e85782 * ((locals.var_dnm).powf(assign55400_e85782 - 1.0) * locals.var_dnm_dn8)) } } else { (assign55400_e85783 * (assign55400_e85782 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55400_e85782) as f64).is_finite() && ((assign55400_e85782) as f64).fract() == 0.0 { if assign55400_e85782 == 0.0 { 0.0 } else { (assign55400_e85782 * ((locals.var_dnm).powf(assign55400_e85782 - 1.0) * locals.var_dnm_dn9)) } } else { (assign55400_e85783 * (assign55400_e85782 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55400_e85782) as f64).is_finite() && ((assign55400_e85782) as f64).fract() == 0.0 { if assign55400_e85782 == 0.0 { 0.0 } else { (assign55400_e85782 * ((locals.var_dnm).powf(assign55400_e85782 - 1.0) * locals.var_dnm_dn10)) } } else { (assign55400_e85783 * (assign55400_e85782 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55400_e85782) as f64).is_finite() && ((assign55400_e85782) as f64).fract() == 0.0 { if assign55400_e85782 == 0.0 { 0.0 } else { (assign55400_e85782 * ((locals.var_dnm).powf(assign55400_e85782 - 1.0) * locals.var_dnm_dn11)) } } else { (assign55400_e85783 * (assign55400_e85782 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55400_e85782) as f64).is_finite() && ((assign55400_e85782) as f64).fract() == 0.0 { if assign55400_e85782 == 0.0 { 0.0 } else { (assign55400_e85782 * ((locals.var_dnm).powf(assign55400_e85782 - 1.0) * locals.var_dnm_dn14)) } } else { (assign55400_e85783 * (assign55400_e85782 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign55400_e85784, assign55400_e85784_d_n0, assign55400_e85784_d_n2, assign55400_e85784_d_n4, assign55400_e85784_d_n5, assign55400_e85784_d_n6, assign55400_e85784_d_n7, assign55400_e85784_d_n8, assign55400_e85784_d_n9, assign55400_e85784_d_n10, assign55400_e85784_d_n11, assign55400_e85784_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign55400_e85786;
        locals.var_dnm_dn0 = assign55400_e85786_d_n0;
        locals.var_dnm_dn2 = assign55400_e85786_d_n2;
        locals.var_dnm_dn4 = assign55400_e85786_d_n4;
        locals.var_dnm_dn5 = assign55400_e85786_d_n5;
        locals.var_dnm_dn6 = assign55400_e85786_d_n6;
        locals.var_dnm_dn7 = assign55400_e85786_d_n7;
        locals.var_dnm_dn8 = assign55400_e85786_d_n8;
        locals.var_dnm_dn9 = assign55400_e85786_d_n9;
        locals.var_dnm_dn10 = assign55400_e85786_d_n10;
        locals.var_dnm_dn11 = assign55400_e85786_d_n11;
        locals.var_dnm_dn14 = assign55400_e85786_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign55410_e85804, assign55410_e85804_d_n0, assign55410_e85804_d_n2, assign55410_e85804_d_n4, assign55410_e85804_d_n5, assign55410_e85804_d_n6, assign55410_e85804_d_n7, assign55410_e85804_d_n8, assign55410_e85804_d_n9, assign55410_e85804_d_n10, assign55410_e85804_d_n11, assign55410_e85804_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1390 != 0.0)) {
        let assign55410_e85802: f64 = (1.0 / locals.var_dnm);
        (assign55410_e85802, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign55410_e85804;
        locals.var_dnm_dn0 = assign55410_e85804_d_n0;
        locals.var_dnm_dn2 = assign55410_e85804_d_n2;
        locals.var_dnm_dn4 = assign55410_e85804_d_n4;
        locals.var_dnm_dn5 = assign55410_e85804_d_n5;
        locals.var_dnm_dn6 = assign55410_e85804_d_n6;
        locals.var_dnm_dn7 = assign55410_e85804_d_n7;
        locals.var_dnm_dn8 = assign55410_e85804_d_n8;
        locals.var_dnm_dn9 = assign55410_e85804_d_n9;
        locals.var_dnm_dn10 = assign55410_e85804_d_n10;
        locals.var_dnm_dn11 = assign55410_e85804_d_n11;
        locals.var_dnm_dn14 = assign55410_e85804_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign55420_e85824, assign55420_e85824_d_n0, assign55420_e85824_d_n2, assign55420_e85824_d_n4, assign55420_e85824_d_n5, assign55420_e85824_d_n6, assign55420_e85824_d_n7, assign55420_e85824_d_n8, assign55420_e85824_d_n9, assign55420_e85824_d_n10, assign55420_e85824_d_n11, assign55420_e85824_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1390 != 0.0)) {
        let assign55420_e85820: f64 = (locals.var_tmf1 * 1e-16);
        let assign55420_e85822: f64 = (assign55420_e85820 * locals.var_dnm);
        (assign55420_e85822, (((locals.var_tmf1_dn0 * 1e-16) * locals.var_dnm) + (assign55420_e85820 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-16) * locals.var_dnm) + (assign55420_e85820 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1e-16) * locals.var_dnm) + (assign55420_e85820 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1e-16) * locals.var_dnm) + (assign55420_e85820 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1e-16) * locals.var_dnm) + (assign55420_e85820 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-16) * locals.var_dnm) + (assign55420_e85820 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1e-16) * locals.var_dnm) + (assign55420_e85820 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1e-16) * locals.var_dnm) + (assign55420_e85820 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1e-16) * locals.var_dnm) + (assign55420_e85820 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 1e-16) * locals.var_dnm) + (assign55420_e85820 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 1e-16) * locals.var_dnm) + (assign55420_e85820 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign55420_e85824;
        locals.var_tmf0_dn0 = assign55420_e85824_d_n0;
        locals.var_tmf0_dn2 = assign55420_e85824_d_n2;
        locals.var_tmf0_dn4 = assign55420_e85824_d_n4;
        locals.var_tmf0_dn5 = assign55420_e85824_d_n5;
        locals.var_tmf0_dn6 = assign55420_e85824_d_n6;
        locals.var_tmf0_dn7 = assign55420_e85824_d_n7;
        locals.var_tmf0_dn8 = assign55420_e85824_d_n8;
        locals.var_tmf0_dn9 = assign55420_e85824_d_n9;
        locals.var_tmf0_dn10 = assign55420_e85824_d_n10;
        locals.var_tmf0_dn11 = assign55420_e85824_d_n11;
        locals.var_tmf0_dn14 = assign55420_e85824_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign55430_e85846, assign55430_e85846_d_n0, assign55430_e85846_d_n2, assign55430_e85846_d_n4, assign55430_e85846_d_n5, assign55430_e85846_d_n6, assign55430_e85846_d_n7, assign55430_e85846_d_n8, assign55430_e85846_d_n9, assign55430_e85846_d_n10, assign55430_e85846_d_n11, assign55430_e85846_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1390 != 0.0)) {
        let assign55430_e85840: f64 = (1e-16 * locals.var_xmp);
        let assign55430_e85842: f64 = (assign55430_e85840 * locals.var_dnm);
        let assign55430_e85844: f64 = (assign55430_e85842 / locals.var_arg);
        (assign55430_e85844, ((((((1e-16 * locals.var_xmp_dn0) * locals.var_dnm) + (assign55430_e85840 * locals.var_dnm_dn0)) * locals.var_arg) - (assign55430_e85842 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn2) * locals.var_dnm) + (assign55430_e85840 * locals.var_dnm_dn2)) * locals.var_arg) - (assign55430_e85842 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn4) * locals.var_dnm) + (assign55430_e85840 * locals.var_dnm_dn4)) * locals.var_arg) - (assign55430_e85842 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn5) * locals.var_dnm) + (assign55430_e85840 * locals.var_dnm_dn5)) * locals.var_arg) - (assign55430_e85842 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn6) * locals.var_dnm) + (assign55430_e85840 * locals.var_dnm_dn6)) * locals.var_arg) - (assign55430_e85842 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn7) * locals.var_dnm) + (assign55430_e85840 * locals.var_dnm_dn7)) * locals.var_arg) - (assign55430_e85842 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn8) * locals.var_dnm) + (assign55430_e85840 * locals.var_dnm_dn8)) * locals.var_arg) - (assign55430_e85842 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn9) * locals.var_dnm) + (assign55430_e85840 * locals.var_dnm_dn9)) * locals.var_arg) - (assign55430_e85842 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn10) * locals.var_dnm) + (assign55430_e85840 * locals.var_dnm_dn10)) * locals.var_arg) - (assign55430_e85842 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn11) * locals.var_dnm) + (assign55430_e85840 * locals.var_dnm_dn11)) * locals.var_arg) - (assign55430_e85842 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn14) * locals.var_dnm) + (assign55430_e85840 * locals.var_dnm_dn14)) * locals.var_arg) - (assign55430_e85842 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign55430_e85846;
        locals.var_t0_dn0 = assign55430_e85846_d_n0;
        locals.var_t0_dn2 = assign55430_e85846_d_n2;
        locals.var_t0_dn4 = assign55430_e85846_d_n4;
        locals.var_t0_dn5 = assign55430_e85846_d_n5;
        locals.var_t0_dn6 = assign55430_e85846_d_n6;
        locals.var_t0_dn7 = assign55430_e85846_d_n7;
        locals.var_t0_dn8 = assign55430_e85846_d_n8;
        locals.var_t0_dn9 = assign55430_e85846_d_n9;
        locals.var_t0_dn10 = assign55430_e85846_d_n10;
        locals.var_t0_dn11 = assign55430_e85846_d_n11;
        locals.var_t0_dn14 = assign55430_e85846_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign55440_e85866, assign55440_e85866_d_n0, assign55440_e85866_d_n2, assign55440_e85866_d_n4, assign55440_e85866_d_n5, assign55440_e85866_d_n6, assign55440_e85866_d_n7, assign55440_e85866_d_n8, assign55440_e85866_d_n9, assign55440_e85866_d_n10, assign55440_e85866_d_n11, assign55440_e85866_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1390 != 0.0)) {
        let assign55440_e85862: f64 = 1e-16;
        let assign55440_e85864: f64 = (assign55440_e85862 - locals.var_tmf0);
        (assign55440_e85864, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_w_res, locals.var_w_res_dn0, locals.var_w_res_dn2, locals.var_w_res_dn4, locals.var_w_res_dn5, locals.var_w_res_dn6, locals.var_w_res_dn7, locals.var_w_res_dn8, locals.var_w_res_dn9, locals.var_w_res_dn10, locals.var_w_res_dn11, locals.var_w_res_dn14,)
    }
};
        locals.var_w_res = assign55440_e85866;
        locals.var_w_res_dn0 = assign55440_e85866_d_n0;
        locals.var_w_res_dn2 = assign55440_e85866_d_n2;
        locals.var_w_res_dn4 = assign55440_e85866_d_n4;
        locals.var_w_res_dn5 = assign55440_e85866_d_n5;
        locals.var_w_res_dn6 = assign55440_e85866_d_n6;
        locals.var_w_res_dn7 = assign55440_e85866_d_n7;
        locals.var_w_res_dn8 = assign55440_e85866_d_n8;
        locals.var_w_res_dn9 = assign55440_e85866_d_n9;
        locals.var_w_res_dn10 = assign55440_e85866_d_n10;
        locals.var_w_res_dn11 = assign55440_e85866_d_n11;
        locals.var_w_res_dn14 = assign55440_e85866_d_n14;
        locals.var_w_res_rv = 0.0;

        let (assign55450_e85882, assign55450_e85882_d_n0, assign55450_e85882_d_n2, assign55450_e85882_d_n4, assign55450_e85882_d_n5, assign55450_e85882_d_n6, assign55450_e85882_d_n7, assign55450_e85882_d_n8, assign55450_e85882_d_n9, assign55450_e85882_d_n10, assign55450_e85882_d_n11, assign55450_e85882_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1390 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign55450_e85882;
        locals.var_t0_dn0 = assign55450_e85882_d_n0;
        locals.var_t0_dn2 = assign55450_e85882_d_n2;
        locals.var_t0_dn4 = assign55450_e85882_d_n4;
        locals.var_t0_dn5 = assign55450_e85882_d_n5;
        locals.var_t0_dn6 = assign55450_e85882_d_n6;
        locals.var_t0_dn7 = assign55450_e85882_d_n7;
        locals.var_t0_dn8 = assign55450_e85882_d_n8;
        locals.var_t0_dn9 = assign55450_e85882_d_n9;
        locals.var_t0_dn10 = assign55450_e85882_d_n10;
        locals.var_t0_dn11 = assign55450_e85882_d_n11;
        locals.var_t0_dn14 = assign55450_e85882_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign55460_e85899, assign55460_e85899_d_n0, assign55460_e85899_d_n2, assign55460_e85899_d_n4, assign55460_e85899_d_n5, assign55460_e85899_d_n6, assign55460_e85899_d_n7, assign55460_e85899_d_n8, assign55460_e85899_d_n9, assign55460_e85899_d_n10, assign55460_e85899_d_n11, assign55460_e85899_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1390 == 0.0)) {
        (locals.var_w_res, locals.var_w_res_dn0, locals.var_w_res_dn2, locals.var_w_res_dn4, locals.var_w_res_dn5, locals.var_w_res_dn6, locals.var_w_res_dn7, locals.var_w_res_dn8, locals.var_w_res_dn9, locals.var_w_res_dn10, locals.var_w_res_dn11, locals.var_w_res_dn14,)
    } else {
        (locals.var_w_res, locals.var_w_res_dn0, locals.var_w_res_dn2, locals.var_w_res_dn4, locals.var_w_res_dn5, locals.var_w_res_dn6, locals.var_w_res_dn7, locals.var_w_res_dn8, locals.var_w_res_dn9, locals.var_w_res_dn10, locals.var_w_res_dn11, locals.var_w_res_dn14,)
    }
};
        locals.var_w_res = assign55460_e85899;
        locals.var_w_res_dn0 = assign55460_e85899_d_n0;
        locals.var_w_res_dn2 = assign55460_e85899_d_n2;
        locals.var_w_res_dn4 = assign55460_e85899_d_n4;
        locals.var_w_res_dn5 = assign55460_e85899_d_n5;
        locals.var_w_res_dn6 = assign55460_e85899_d_n6;
        locals.var_w_res_dn7 = assign55460_e85899_d_n7;
        locals.var_w_res_dn8 = assign55460_e85899_d_n8;
        locals.var_w_res_dn9 = assign55460_e85899_d_n9;
        locals.var_w_res_dn10 = assign55460_e85899_d_n10;
        locals.var_w_res_dn11 = assign55460_e85899_d_n11;
        locals.var_w_res_dn14 = assign55460_e85899_d_n14;
        locals.var_w_res_rv = 0.0;

        let (assign55470_e85916, assign55470_e85916_d_n0, assign55470_e85916_d_n2, assign55470_e85916_d_n4, assign55470_e85916_d_n5, assign55470_e85916_d_n6, assign55470_e85916_d_n7, assign55470_e85916_d_n8, assign55470_e85916_d_n9, assign55470_e85916_d_n10, assign55470_e85916_d_n11, assign55470_e85916_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1390 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign55470_e85916;
        locals.var_t0_dn0 = assign55470_e85916_d_n0;
        locals.var_t0_dn2 = assign55470_e85916_d_n2;
        locals.var_t0_dn4 = assign55470_e85916_d_n4;
        locals.var_t0_dn5 = assign55470_e85916_d_n5;
        locals.var_t0_dn6 = assign55470_e85916_d_n6;
        locals.var_t0_dn7 = assign55470_e85916_d_n7;
        locals.var_t0_dn8 = assign55470_e85916_d_n8;
        locals.var_t0_dn9 = assign55470_e85916_d_n9;
        locals.var_t0_dn10 = assign55470_e85916_d_n10;
        locals.var_t0_dn11 = assign55470_e85916_d_n11;
        locals.var_t0_dn14 = assign55470_e85916_d_n14;
        locals.var_t0_rv = 0.0;

        let assign55480_e85919: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1396 = assign55480_e85919;
        locals.var_guard1396_rv = 0.0;

        let (assign55490_e85935, assign55490_e85935_d_n0, assign55490_e85935_d_n2, assign55490_e85935_d_n4, assign55490_e85935_d_n5, assign55490_e85935_d_n6, assign55490_e85935_d_n7, assign55490_e85935_d_n8, assign55490_e85935_d_n9, assign55490_e85935_d_n10, assign55490_e85935_d_n11, assign55490_e85935_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1396 != 0.0)) {
        (locals.var_w_res, locals.var_w_res_dn0, locals.var_w_res_dn2, locals.var_w_res_dn4, locals.var_w_res_dn5, locals.var_w_res_dn6, locals.var_w_res_dn7, locals.var_w_res_dn8, locals.var_w_res_dn9, locals.var_w_res_dn10, locals.var_w_res_dn11, locals.var_w_res_dn14,)
    } else {
        (locals.var_w_res_leak, locals.var_w_res_leak_dn0, locals.var_w_res_leak_dn2, locals.var_w_res_leak_dn4, locals.var_w_res_leak_dn5, locals.var_w_res_leak_dn6, locals.var_w_res_leak_dn7, locals.var_w_res_leak_dn8, locals.var_w_res_leak_dn9, locals.var_w_res_leak_dn10, locals.var_w_res_leak_dn11, locals.var_w_res_leak_dn14,)
    }
};
        locals.var_w_res_leak = assign55490_e85935;
        locals.var_w_res_leak_dn0 = assign55490_e85935_d_n0;
        locals.var_w_res_leak_dn2 = assign55490_e85935_d_n2;
        locals.var_w_res_leak_dn4 = assign55490_e85935_d_n4;
        locals.var_w_res_leak_dn5 = assign55490_e85935_d_n5;
        locals.var_w_res_leak_dn6 = assign55490_e85935_d_n6;
        locals.var_w_res_leak_dn7 = assign55490_e85935_d_n7;
        locals.var_w_res_leak_dn8 = assign55490_e85935_d_n8;
        locals.var_w_res_leak_dn9 = assign55490_e85935_d_n9;
        locals.var_w_res_leak_dn10 = assign55490_e85935_d_n10;
        locals.var_w_res_leak_dn11 = assign55490_e85935_d_n11;
        locals.var_w_res_leak_dn14 = assign55490_e85935_d_n14;
        locals.var_w_res_leak_rv = 0.0;

        let assign55500_e85938: f64 = if 0.0 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1397 = assign55500_e85938;
        locals.var_guard1397_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_202(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign55510_e85951, assign55510_e85951_d_n0, assign55510_e85951_d_n2, assign55510_e85951_d_n4, assign55510_e85951_d_n5, assign55510_e85951_d_n6, assign55510_e85951_d_n7, assign55510_e85951_d_n8, assign55510_e85951_d_n9, assign55510_e85951_d_n10, assign55510_e85951_d_n11, assign55510_e85951_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn4, locals.var_vdsorg_dn5, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn8, locals.var_vdsorg_dn9, locals.var_vdsorg_dn10, locals.var_vdsorg_dn11, locals.var_vdsorg_dn14,)
    } else {
        (locals.var_vds_res, locals.var_vds_res_dn0, locals.var_vds_res_dn2, locals.var_vds_res_dn4, locals.var_vds_res_dn5, locals.var_vds_res_dn6, locals.var_vds_res_dn7, locals.var_vds_res_dn8, locals.var_vds_res_dn9, locals.var_vds_res_dn10, locals.var_vds_res_dn11, locals.var_vds_res_dn14,)
    }
};
        locals.var_vds_res = assign55510_e85951;
        locals.var_vds_res_dn0 = assign55510_e85951_d_n0;
        locals.var_vds_res_dn2 = assign55510_e85951_d_n2;
        locals.var_vds_res_dn4 = assign55510_e85951_d_n4;
        locals.var_vds_res_dn5 = assign55510_e85951_d_n5;
        locals.var_vds_res_dn6 = assign55510_e85951_d_n6;
        locals.var_vds_res_dn7 = assign55510_e85951_d_n7;
        locals.var_vds_res_dn8 = assign55510_e85951_d_n8;
        locals.var_vds_res_dn9 = assign55510_e85951_d_n9;
        locals.var_vds_res_dn10 = assign55510_e85951_d_n10;
        locals.var_vds_res_dn11 = assign55510_e85951_d_n11;
        locals.var_vds_res_dn14 = assign55510_e85951_d_n14;
        locals.var_vds_res_rv = 0.0;

        let (assign55520_e85968, assign55520_e85968_d_n0, assign55520_e85968_d_n2, assign55520_e85968_d_n4, assign55520_e85968_d_n5, assign55520_e85968_d_n6, assign55520_e85968_d_n7, assign55520_e85968_d_n8, assign55520_e85968_d_n9, assign55520_e85968_d_n10, assign55520_e85968_d_n11, assign55520_e85968_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) {
        let assign55520_e85964: f64 = (locals.var_vbsc__blk1119 + locals.var_beta_inv);
        let assign55520_e85966: f64 = (assign55520_e85964 * p.p396);
        (assign55520_e85966, ((locals.var_vbsc__blk1119_dn0 + locals.var_beta_inv_dn0) * p.p396), ((locals.var_vbsc__blk1119_dn2 + locals.var_beta_inv_dn2) * p.p396), ((locals.var_vbsc__blk1119_dn4 + locals.var_beta_inv_dn4) * p.p396), ((locals.var_vbsc__blk1119_dn5 + locals.var_beta_inv_dn5) * p.p396), ((locals.var_vbsc__blk1119_dn6 + locals.var_beta_inv_dn6) * p.p396), ((locals.var_vbsc__blk1119_dn7 + locals.var_beta_inv_dn7) * p.p396), ((locals.var_vbsc__blk1119_dn8 + locals.var_beta_inv_dn8) * p.p396), ((locals.var_vbsc__blk1119_dn9 + locals.var_beta_inv_dn9) * p.p396), ((locals.var_vbsc__blk1119_dn10 + locals.var_beta_inv_dn10) * p.p396), ((locals.var_vbsc__blk1119_dn11 + locals.var_beta_inv_dn11) * p.p396), ((locals.var_vbsc__blk1119_dn14 + locals.var_beta_inv_dn14) * p.p396),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign55520_e85968;
        locals.var_t10_dn0 = assign55520_e85968_d_n0;
        locals.var_t10_dn2 = assign55520_e85968_d_n2;
        locals.var_t10_dn4 = assign55520_e85968_d_n4;
        locals.var_t10_dn5 = assign55520_e85968_d_n5;
        locals.var_t10_dn6 = assign55520_e85968_d_n6;
        locals.var_t10_dn7 = assign55520_e85968_d_n7;
        locals.var_t10_dn8 = assign55520_e85968_d_n8;
        locals.var_t10_dn9 = assign55520_e85968_d_n9;
        locals.var_t10_dn10 = assign55520_e85968_d_n10;
        locals.var_t10_dn11 = assign55520_e85968_d_n11;
        locals.var_t10_dn14 = assign55520_e85968_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign55530_e85987, assign55530_e85987_d_n0, assign55530_e85987_d_n2, assign55530_e85987_d_n4, assign55530_e85987_d_n5, assign55530_e85987_d_n6, assign55530_e85987_d_n7, assign55530_e85987_d_n8, assign55530_e85987_d_n9, assign55530_e85987_d_n10, assign55530_e85987_d_n11, assign55530_e85987_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) {
        let assign55530_e85983: f64 = (locals.var_vgp - locals.var_t10);
        let assign55530_e85984: f64 = (locals.var_c2_q_ndepm_esi_cox_inv2__blk1137 * assign55530_e85983);
        let assign55530_e85985: f64 = (1.0 + assign55530_e85984);
        (assign55530_e85985, ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1137_dn0 * assign55530_e85983) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1137 * (locals.var_vgp_dn0 - locals.var_t10_dn0))), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1137_dn2 * assign55530_e85983) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1137 * (locals.var_vgp_dn2 - locals.var_t10_dn2))), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1137_dn4 * assign55530_e85983) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1137 * (locals.var_vgp_dn4 - locals.var_t10_dn4))), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1137_dn5 * assign55530_e85983) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1137 * (locals.var_vgp_dn5 - locals.var_t10_dn5))), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1137_dn6 * assign55530_e85983) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1137 * (locals.var_vgp_dn6 - locals.var_t10_dn6))), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1137_dn7 * assign55530_e85983) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1137 * (locals.var_vgp_dn7 - locals.var_t10_dn7))), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1137_dn8 * assign55530_e85983) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1137 * (locals.var_vgp_dn8 - locals.var_t10_dn8))), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1137_dn9 * assign55530_e85983) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1137 * (locals.var_vgp_dn9 - locals.var_t10_dn9))), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1137_dn10 * assign55530_e85983) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1137 * (locals.var_vgp_dn10 - locals.var_t10_dn10))), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1137_dn11 * assign55530_e85983) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1137 * (locals.var_vgp_dn11 - locals.var_t10_dn11))), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1137_dn14 * assign55530_e85983) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1137 * (locals.var_vgp_dn14 - locals.var_t10_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign55530_e85987;
        locals.var_t4_dn0 = assign55530_e85987_d_n0;
        locals.var_t4_dn2 = assign55530_e85987_d_n2;
        locals.var_t4_dn4 = assign55530_e85987_d_n4;
        locals.var_t4_dn5 = assign55530_e85987_d_n5;
        locals.var_t4_dn6 = assign55530_e85987_d_n6;
        locals.var_t4_dn7 = assign55530_e85987_d_n7;
        locals.var_t4_dn8 = assign55530_e85987_d_n8;
        locals.var_t4_dn9 = assign55530_e85987_d_n9;
        locals.var_t4_dn10 = assign55530_e85987_d_n10;
        locals.var_t4_dn11 = assign55530_e85987_d_n11;
        locals.var_t4_dn14 = assign55530_e85987_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign55540_e86002, assign55540_e86002_d_n0, assign55540_e86002_d_n2, assign55540_e86002_d_n4, assign55540_e86002_d_n5, assign55540_e86002_d_n6, assign55540_e86002_d_n7, assign55540_e86002_d_n8, assign55540_e86002_d_n9, assign55540_e86002_d_n10, assign55540_e86002_d_n11, assign55540_e86002_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) {
        let assign55540_e86000: f64 = (1.0 + locals.var_c2_q_ndepm_esi_cox_inv2__blk1137);
        (assign55540_e86000, locals.var_c2_q_ndepm_esi_cox_inv2__blk1137_dn0, locals.var_c2_q_ndepm_esi_cox_inv2__blk1137_dn2, locals.var_c2_q_ndepm_esi_cox_inv2__blk1137_dn4, locals.var_c2_q_ndepm_esi_cox_inv2__blk1137_dn5, locals.var_c2_q_ndepm_esi_cox_inv2__blk1137_dn6, locals.var_c2_q_ndepm_esi_cox_inv2__blk1137_dn7, locals.var_c2_q_ndepm_esi_cox_inv2__blk1137_dn8, locals.var_c2_q_ndepm_esi_cox_inv2__blk1137_dn9, locals.var_c2_q_ndepm_esi_cox_inv2__blk1137_dn10, locals.var_c2_q_ndepm_esi_cox_inv2__blk1137_dn11, locals.var_c2_q_ndepm_esi_cox_inv2__blk1137_dn14,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign55540_e86002;
        locals.var_t5_dn0 = assign55540_e86002_d_n0;
        locals.var_t5_dn2 = assign55540_e86002_d_n2;
        locals.var_t5_dn4 = assign55540_e86002_d_n4;
        locals.var_t5_dn5 = assign55540_e86002_d_n5;
        locals.var_t5_dn6 = assign55540_e86002_d_n6;
        locals.var_t5_dn7 = assign55540_e86002_d_n7;
        locals.var_t5_dn8 = assign55540_e86002_d_n8;
        locals.var_t5_dn9 = assign55540_e86002_d_n9;
        locals.var_t5_dn10 = assign55540_e86002_d_n10;
        locals.var_t5_dn11 = assign55540_e86002_d_n11;
        locals.var_t5_dn14 = assign55540_e86002_d_n14;
        locals.var_t5_rv = 0.0;

        let assign55550_e86006: f64 = locals.var_t5;
        let assign55550_e86011: f64 = if ((locals.var_t4 < assign55550_e86006) && (locals.var_t5 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1398 = assign55550_e86011;
        locals.var_guard1398_rv = 0.0;

        let (assign55560_e86030, assign55560_e86030_d_n0, assign55560_e86030_d_n2, assign55560_e86030_d_n4, assign55560_e86030_d_n5, assign55560_e86030_d_n6, assign55560_e86030_d_n7, assign55560_e86030_d_n8, assign55560_e86030_d_n9, assign55560_e86030_d_n10, assign55560_e86030_d_n11, assign55560_e86030_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1398 != 0.0)) {
        let assign55560_e86026: f64 = locals.var_t5;
        let assign55560_e86028: f64 = (assign55560_e86026 - locals.var_t4);
        (assign55560_e86028, (locals.var_t5_dn0 - locals.var_t4_dn0), (locals.var_t5_dn2 - locals.var_t4_dn2), (locals.var_t5_dn4 - locals.var_t4_dn4), (locals.var_t5_dn5 - locals.var_t4_dn5), (locals.var_t5_dn6 - locals.var_t4_dn6), (locals.var_t5_dn7 - locals.var_t4_dn7), (locals.var_t5_dn8 - locals.var_t4_dn8), (locals.var_t5_dn9 - locals.var_t4_dn9), (locals.var_t5_dn10 - locals.var_t4_dn10), (locals.var_t5_dn11 - locals.var_t4_dn11), (locals.var_t5_dn14 - locals.var_t4_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign55560_e86030;
        locals.var_tmf1_dn0 = assign55560_e86030_d_n0;
        locals.var_tmf1_dn2 = assign55560_e86030_d_n2;
        locals.var_tmf1_dn4 = assign55560_e86030_d_n4;
        locals.var_tmf1_dn5 = assign55560_e86030_d_n5;
        locals.var_tmf1_dn6 = assign55560_e86030_d_n6;
        locals.var_tmf1_dn7 = assign55560_e86030_d_n7;
        locals.var_tmf1_dn8 = assign55560_e86030_d_n8;
        locals.var_tmf1_dn9 = assign55560_e86030_d_n9;
        locals.var_tmf1_dn10 = assign55560_e86030_d_n10;
        locals.var_tmf1_dn11 = assign55560_e86030_d_n11;
        locals.var_tmf1_dn14 = assign55560_e86030_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign55570_e86047, assign55570_e86047_d_n0, assign55570_e86047_d_n2, assign55570_e86047_d_n4, assign55570_e86047_d_n5, assign55570_e86047_d_n6, assign55570_e86047_d_n7, assign55570_e86047_d_n8, assign55570_e86047_d_n9, assign55570_e86047_d_n10, assign55570_e86047_d_n11, assign55570_e86047_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1398 != 0.0)) {
        let assign55570_e86045: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign55570_e86045, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign55570_e86047;
        locals.var_x2_dn0 = assign55570_e86047_d_n0;
        locals.var_x2_dn2 = assign55570_e86047_d_n2;
        locals.var_x2_dn4 = assign55570_e86047_d_n4;
        locals.var_x2_dn5 = assign55570_e86047_d_n5;
        locals.var_x2_dn6 = assign55570_e86047_d_n6;
        locals.var_x2_dn7 = assign55570_e86047_d_n7;
        locals.var_x2_dn8 = assign55570_e86047_d_n8;
        locals.var_x2_dn9 = assign55570_e86047_d_n9;
        locals.var_x2_dn10 = assign55570_e86047_d_n10;
        locals.var_x2_dn11 = assign55570_e86047_d_n11;
        locals.var_x2_dn14 = assign55570_e86047_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign55580_e86064, assign55580_e86064_d_n0, assign55580_e86064_d_n2, assign55580_e86064_d_n4, assign55580_e86064_d_n5, assign55580_e86064_d_n6, assign55580_e86064_d_n7, assign55580_e86064_d_n8, assign55580_e86064_d_n9, assign55580_e86064_d_n10, assign55580_e86064_d_n11, assign55580_e86064_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1398 != 0.0)) {
        let assign55580_e86062: f64 = (locals.var_t5 * locals.var_t5);
        (assign55580_e86062, ((locals.var_t5_dn0 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn0)), ((locals.var_t5_dn2 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn2)), ((locals.var_t5_dn4 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn4)), ((locals.var_t5_dn5 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn5)), ((locals.var_t5_dn6 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn6)), ((locals.var_t5_dn7 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn7)), ((locals.var_t5_dn8 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn8)), ((locals.var_t5_dn9 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn9)), ((locals.var_t5_dn10 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn10)), ((locals.var_t5_dn11 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn11)), ((locals.var_t5_dn14 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn14)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign55580_e86064;
        locals.var_xmax2_dn0 = assign55580_e86064_d_n0;
        locals.var_xmax2_dn2 = assign55580_e86064_d_n2;
        locals.var_xmax2_dn4 = assign55580_e86064_d_n4;
        locals.var_xmax2_dn5 = assign55580_e86064_d_n5;
        locals.var_xmax2_dn6 = assign55580_e86064_d_n6;
        locals.var_xmax2_dn7 = assign55580_e86064_d_n7;
        locals.var_xmax2_dn8 = assign55580_e86064_d_n8;
        locals.var_xmax2_dn9 = assign55580_e86064_d_n9;
        locals.var_xmax2_dn10 = assign55580_e86064_d_n10;
        locals.var_xmax2_dn11 = assign55580_e86064_d_n11;
        locals.var_xmax2_dn14 = assign55580_e86064_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign55590_e86079, assign55590_e86079_d_n0, assign55590_e86079_d_n2, assign55590_e86079_d_n4, assign55590_e86079_d_n5, assign55590_e86079_d_n6, assign55590_e86079_d_n7, assign55590_e86079_d_n8, assign55590_e86079_d_n9, assign55590_e86079_d_n10, assign55590_e86079_d_n11, assign55590_e86079_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1398 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign55590_e86079;
        locals.var_xp_dn0 = assign55590_e86079_d_n0;
        locals.var_xp_dn2 = assign55590_e86079_d_n2;
        locals.var_xp_dn4 = assign55590_e86079_d_n4;
        locals.var_xp_dn5 = assign55590_e86079_d_n5;
        locals.var_xp_dn6 = assign55590_e86079_d_n6;
        locals.var_xp_dn7 = assign55590_e86079_d_n7;
        locals.var_xp_dn8 = assign55590_e86079_d_n8;
        locals.var_xp_dn9 = assign55590_e86079_d_n9;
        locals.var_xp_dn10 = assign55590_e86079_d_n10;
        locals.var_xp_dn11 = assign55590_e86079_d_n11;
        locals.var_xp_dn14 = assign55590_e86079_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign55600_e86094, assign55600_e86094_d_n0, assign55600_e86094_d_n2, assign55600_e86094_d_n4, assign55600_e86094_d_n5, assign55600_e86094_d_n6, assign55600_e86094_d_n7, assign55600_e86094_d_n8, assign55600_e86094_d_n9, assign55600_e86094_d_n10, assign55600_e86094_d_n11, assign55600_e86094_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1398 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign55600_e86094;
        locals.var_xmp_dn0 = assign55600_e86094_d_n0;
        locals.var_xmp_dn2 = assign55600_e86094_d_n2;
        locals.var_xmp_dn4 = assign55600_e86094_d_n4;
        locals.var_xmp_dn5 = assign55600_e86094_d_n5;
        locals.var_xmp_dn6 = assign55600_e86094_d_n6;
        locals.var_xmp_dn7 = assign55600_e86094_d_n7;
        locals.var_xmp_dn8 = assign55600_e86094_d_n8;
        locals.var_xmp_dn9 = assign55600_e86094_d_n9;
        locals.var_xmp_dn10 = assign55600_e86094_d_n10;
        locals.var_xmp_dn11 = assign55600_e86094_d_n11;
        locals.var_xmp_dn14 = assign55600_e86094_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign55610_e86109,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1398 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign55610_e86109;
        locals.var_m0_rv = 0.0;

        let (assign55620_e86124,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1398 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign55620_e86124;
        locals.var_mm_rv = 0.0;

        let (assign55630_e86139, assign55630_e86139_d_n0, assign55630_e86139_d_n2, assign55630_e86139_d_n4, assign55630_e86139_d_n5, assign55630_e86139_d_n6, assign55630_e86139_d_n7, assign55630_e86139_d_n8, assign55630_e86139_d_n9, assign55630_e86139_d_n10, assign55630_e86139_d_n11, assign55630_e86139_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1398 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign55630_e86139;
        locals.var_arg_dn0 = assign55630_e86139_d_n0;
        locals.var_arg_dn2 = assign55630_e86139_d_n2;
        locals.var_arg_dn4 = assign55630_e86139_d_n4;
        locals.var_arg_dn5 = assign55630_e86139_d_n5;
        locals.var_arg_dn6 = assign55630_e86139_d_n6;
        locals.var_arg_dn7 = assign55630_e86139_d_n7;
        locals.var_arg_dn8 = assign55630_e86139_d_n8;
        locals.var_arg_dn9 = assign55630_e86139_d_n9;
        locals.var_arg_dn10 = assign55630_e86139_d_n10;
        locals.var_arg_dn11 = assign55630_e86139_d_n11;
        locals.var_arg_dn14 = assign55630_e86139_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign55640_e86154, assign55640_e86154_d_n0, assign55640_e86154_d_n2, assign55640_e86154_d_n4, assign55640_e86154_d_n5, assign55640_e86154_d_n6, assign55640_e86154_d_n7, assign55640_e86154_d_n8, assign55640_e86154_d_n9, assign55640_e86154_d_n10, assign55640_e86154_d_n11, assign55640_e86154_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1398 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign55640_e86154;
        locals.var_dnm_dn0 = assign55640_e86154_d_n0;
        locals.var_dnm_dn2 = assign55640_e86154_d_n2;
        locals.var_dnm_dn4 = assign55640_e86154_d_n4;
        locals.var_dnm_dn5 = assign55640_e86154_d_n5;
        locals.var_dnm_dn6 = assign55640_e86154_d_n6;
        locals.var_dnm_dn7 = assign55640_e86154_d_n7;
        locals.var_dnm_dn8 = assign55640_e86154_d_n8;
        locals.var_dnm_dn9 = assign55640_e86154_d_n9;
        locals.var_dnm_dn10 = assign55640_e86154_d_n10;
        locals.var_dnm_dn11 = assign55640_e86154_d_n11;
        locals.var_dnm_dn14 = assign55640_e86154_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign55650_e86171, assign55650_e86171_d_n0, assign55650_e86171_d_n2, assign55650_e86171_d_n4, assign55650_e86171_d_n5, assign55650_e86171_d_n6, assign55650_e86171_d_n7, assign55650_e86171_d_n8, assign55650_e86171_d_n9, assign55650_e86171_d_n10, assign55650_e86171_d_n11, assign55650_e86171_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1398 != 0.0)) {
        let assign55650_e86169: f64 = (locals.var_xp * locals.var_x2);
        (assign55650_e86169, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign55650_e86171;
        locals.var_xp_dn0 = assign55650_e86171_d_n0;
        locals.var_xp_dn2 = assign55650_e86171_d_n2;
        locals.var_xp_dn4 = assign55650_e86171_d_n4;
        locals.var_xp_dn5 = assign55650_e86171_d_n5;
        locals.var_xp_dn6 = assign55650_e86171_d_n6;
        locals.var_xp_dn7 = assign55650_e86171_d_n7;
        locals.var_xp_dn8 = assign55650_e86171_d_n8;
        locals.var_xp_dn9 = assign55650_e86171_d_n9;
        locals.var_xp_dn10 = assign55650_e86171_d_n10;
        locals.var_xp_dn11 = assign55650_e86171_d_n11;
        locals.var_xp_dn14 = assign55650_e86171_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign55660_e86188, assign55660_e86188_d_n0, assign55660_e86188_d_n2, assign55660_e86188_d_n4, assign55660_e86188_d_n5, assign55660_e86188_d_n6, assign55660_e86188_d_n7, assign55660_e86188_d_n8, assign55660_e86188_d_n9, assign55660_e86188_d_n10, assign55660_e86188_d_n11, assign55660_e86188_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1398 != 0.0)) {
        let assign55660_e86186: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign55660_e86186, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign55660_e86188;
        locals.var_xmp_dn0 = assign55660_e86188_d_n0;
        locals.var_xmp_dn2 = assign55660_e86188_d_n2;
        locals.var_xmp_dn4 = assign55660_e86188_d_n4;
        locals.var_xmp_dn5 = assign55660_e86188_d_n5;
        locals.var_xmp_dn6 = assign55660_e86188_d_n6;
        locals.var_xmp_dn7 = assign55660_e86188_d_n7;
        locals.var_xmp_dn8 = assign55660_e86188_d_n8;
        locals.var_xmp_dn9 = assign55660_e86188_d_n9;
        locals.var_xmp_dn10 = assign55660_e86188_d_n10;
        locals.var_xmp_dn11 = assign55660_e86188_d_n11;
        locals.var_xmp_dn14 = assign55660_e86188_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign55670_e86205, assign55670_e86205_d_n0, assign55670_e86205_d_n2, assign55670_e86205_d_n4, assign55670_e86205_d_n5, assign55670_e86205_d_n6, assign55670_e86205_d_n7, assign55670_e86205_d_n8, assign55670_e86205_d_n9, assign55670_e86205_d_n10, assign55670_e86205_d_n11, assign55670_e86205_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1398 != 0.0)) {
        let assign55670_e86203: f64 = (locals.var_xp * locals.var_x2);
        (assign55670_e86203, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign55670_e86205;
        locals.var_xp_dn0 = assign55670_e86205_d_n0;
        locals.var_xp_dn2 = assign55670_e86205_d_n2;
        locals.var_xp_dn4 = assign55670_e86205_d_n4;
        locals.var_xp_dn5 = assign55670_e86205_d_n5;
        locals.var_xp_dn6 = assign55670_e86205_d_n6;
        locals.var_xp_dn7 = assign55670_e86205_d_n7;
        locals.var_xp_dn8 = assign55670_e86205_d_n8;
        locals.var_xp_dn9 = assign55670_e86205_d_n9;
        locals.var_xp_dn10 = assign55670_e86205_d_n10;
        locals.var_xp_dn11 = assign55670_e86205_d_n11;
        locals.var_xp_dn14 = assign55670_e86205_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign55680_e86222, assign55680_e86222_d_n0, assign55680_e86222_d_n2, assign55680_e86222_d_n4, assign55680_e86222_d_n5, assign55680_e86222_d_n6, assign55680_e86222_d_n7, assign55680_e86222_d_n8, assign55680_e86222_d_n9, assign55680_e86222_d_n10, assign55680_e86222_d_n11, assign55680_e86222_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1398 != 0.0)) {
        let assign55680_e86220: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign55680_e86220, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign55680_e86222;
        locals.var_xmp_dn0 = assign55680_e86222_d_n0;
        locals.var_xmp_dn2 = assign55680_e86222_d_n2;
        locals.var_xmp_dn4 = assign55680_e86222_d_n4;
        locals.var_xmp_dn5 = assign55680_e86222_d_n5;
        locals.var_xmp_dn6 = assign55680_e86222_d_n6;
        locals.var_xmp_dn7 = assign55680_e86222_d_n7;
        locals.var_xmp_dn8 = assign55680_e86222_d_n8;
        locals.var_xmp_dn9 = assign55680_e86222_d_n9;
        locals.var_xmp_dn10 = assign55680_e86222_d_n10;
        locals.var_xmp_dn11 = assign55680_e86222_d_n11;
        locals.var_xmp_dn14 = assign55680_e86222_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign55690_e86239, assign55690_e86239_d_n0, assign55690_e86239_d_n2, assign55690_e86239_d_n4, assign55690_e86239_d_n5, assign55690_e86239_d_n6, assign55690_e86239_d_n7, assign55690_e86239_d_n8, assign55690_e86239_d_n9, assign55690_e86239_d_n10, assign55690_e86239_d_n11, assign55690_e86239_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1398 != 0.0)) {
        let assign55690_e86237: f64 = (locals.var_xp + locals.var_xmp);
        (assign55690_e86237, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign55690_e86239;
        locals.var_arg_dn0 = assign55690_e86239_d_n0;
        locals.var_arg_dn2 = assign55690_e86239_d_n2;
        locals.var_arg_dn4 = assign55690_e86239_d_n4;
        locals.var_arg_dn5 = assign55690_e86239_d_n5;
        locals.var_arg_dn6 = assign55690_e86239_d_n6;
        locals.var_arg_dn7 = assign55690_e86239_d_n7;
        locals.var_arg_dn8 = assign55690_e86239_d_n8;
        locals.var_arg_dn9 = assign55690_e86239_d_n9;
        locals.var_arg_dn10 = assign55690_e86239_d_n10;
        locals.var_arg_dn11 = assign55690_e86239_d_n11;
        locals.var_arg_dn14 = assign55690_e86239_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign55700_e86254, assign55700_e86254_d_n0, assign55700_e86254_d_n2, assign55700_e86254_d_n4, assign55700_e86254_d_n5, assign55700_e86254_d_n6, assign55700_e86254_d_n7, assign55700_e86254_d_n8, assign55700_e86254_d_n9, assign55700_e86254_d_n10, assign55700_e86254_d_n11, assign55700_e86254_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1398 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign55700_e86254;
        locals.var_dnm_dn0 = assign55700_e86254_d_n0;
        locals.var_dnm_dn2 = assign55700_e86254_d_n2;
        locals.var_dnm_dn4 = assign55700_e86254_d_n4;
        locals.var_dnm_dn5 = assign55700_e86254_d_n5;
        locals.var_dnm_dn6 = assign55700_e86254_d_n6;
        locals.var_dnm_dn7 = assign55700_e86254_d_n7;
        locals.var_dnm_dn8 = assign55700_e86254_d_n8;
        locals.var_dnm_dn9 = assign55700_e86254_d_n9;
        locals.var_dnm_dn10 = assign55700_e86254_d_n10;
        locals.var_dnm_dn11 = assign55700_e86254_d_n11;
        locals.var_dnm_dn14 = assign55700_e86254_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign55710_e86269: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1399 = assign55710_e86269;
        locals.var_guard1399_rv = 0.0;

        let assign55720_e86272: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1400 = assign55720_e86272;
        locals.var_guard1400_rv = 0.0;

        let (assign55730_e86291,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1398 != 0.0)) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign55730_e86291;
        locals.var_mm_rv = 0.0;

        let assign55740_e86294: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1401 = assign55740_e86294;
        locals.var_guard1401_rv = 0.0;

        let (assign55750_e86316,) = {
    if (((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1398 != 0.0)) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 == 0.0)) && (locals.var_guard1401 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign55750_e86316;
        locals.var_mm_rv = 0.0;

        let assign55760_e86319: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1402 = assign55760_e86319;
        locals.var_guard1402_rv = 0.0;

        let (assign55770_e86344,) = {
    if ((((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1398 != 0.0)) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 == 0.0)) && (locals.var_guard1401 == 0.0)) && (locals.var_guard1402 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign55770_e86344;
        locals.var_mm_rv = 0.0;

        let assign55780_e86347: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1403 = assign55780_e86347;
        locals.var_guard1403_rv = 0.0;

        let (assign55790_e86375,) = {
    if (((((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1398 != 0.0)) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 == 0.0)) && (locals.var_guard1401 == 0.0)) && (locals.var_guard1402 == 0.0)) && (locals.var_guard1403 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign55790_e86375;
        locals.var_mm_rv = 0.0;

        let (assign55800_e86392,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1398 != 0.0)) && (locals.var_guard1399 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign55800_e86392;
        locals.var_m0_rv = 0.0;

        let mut assign55810_loop_guard: usize = 0;
        while {
            let assign55810_cond_e86410: f64 = if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1398 != 0.0)) && (locals.var_guard1399 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign55810_cond_e86410 != 0.0
        } {
            assign55810_loop_guard += 1;
            assert!(assign55810_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign55810_body0_e86428, assign55810_body0_e86428_d_n0, assign55810_body0_e86428_d_n2, assign55810_body0_e86428_d_n4, assign55810_body0_e86428_d_n5, assign55810_body0_e86428_d_n6, assign55810_body0_e86428_d_n7, assign55810_body0_e86428_d_n8, assign55810_body0_e86428_d_n9, assign55810_body0_e86428_d_n10, assign55810_body0_e86428_d_n11, assign55810_body0_e86428_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1398 != 0.0)) && (locals.var_guard1399 != 0.0)) {
        let assign55810_body0_e86426: f64 = (locals.var_dnm).sqrt();
        (assign55810_body0_e86426, (locals.var_dnm_dn0 / (2.0 * assign55810_body0_e86426)), (locals.var_dnm_dn2 / (2.0 * assign55810_body0_e86426)), (locals.var_dnm_dn4 / (2.0 * assign55810_body0_e86426)), (locals.var_dnm_dn5 / (2.0 * assign55810_body0_e86426)), (locals.var_dnm_dn6 / (2.0 * assign55810_body0_e86426)), (locals.var_dnm_dn7 / (2.0 * assign55810_body0_e86426)), (locals.var_dnm_dn8 / (2.0 * assign55810_body0_e86426)), (locals.var_dnm_dn9 / (2.0 * assign55810_body0_e86426)), (locals.var_dnm_dn10 / (2.0 * assign55810_body0_e86426)), (locals.var_dnm_dn11 / (2.0 * assign55810_body0_e86426)), (locals.var_dnm_dn14 / (2.0 * assign55810_body0_e86426)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign55810_body0_e86428;
            locals.var_dnm_dn0 = assign55810_body0_e86428_d_n0;
            locals.var_dnm_dn2 = assign55810_body0_e86428_d_n2;
            locals.var_dnm_dn4 = assign55810_body0_e86428_d_n4;
            locals.var_dnm_dn5 = assign55810_body0_e86428_d_n5;
            locals.var_dnm_dn6 = assign55810_body0_e86428_d_n6;
            locals.var_dnm_dn7 = assign55810_body0_e86428_d_n7;
            locals.var_dnm_dn8 = assign55810_body0_e86428_d_n8;
            locals.var_dnm_dn9 = assign55810_body0_e86428_d_n9;
            locals.var_dnm_dn10 = assign55810_body0_e86428_d_n10;
            locals.var_dnm_dn11 = assign55810_body0_e86428_d_n11;
            locals.var_dnm_dn14 = assign55810_body0_e86428_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign55810_body1_e86447,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1398 != 0.0)) && (locals.var_guard1399 != 0.0)) {
        let assign55810_body1_e86445: f64 = (locals.var_m0 + 1.0);
        (assign55810_body1_e86445,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign55810_body1_e86447;
            locals.var_m0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_203(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign55820_e86476, assign55820_e86476_d_n0, assign55820_e86476_d_n2, assign55820_e86476_d_n4, assign55820_e86476_d_n5, assign55820_e86476_d_n6, assign55820_e86476_d_n7, assign55820_e86476_d_n8, assign55820_e86476_d_n9, assign55820_e86476_d_n10, assign55820_e86476_d_n11, assign55820_e86476_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1398 != 0.0)) && (locals.var_guard1399 == 0.0)) {
        let (assign55820_e86474, assign55820_e86474_d_n0, assign55820_e86474_d_n2, assign55820_e86474_d_n4, assign55820_e86474_d_n5, assign55820_e86474_d_n6, assign55820_e86474_d_n7, assign55820_e86474_d_n8, assign55820_e86474_d_n9, assign55820_e86474_d_n10, assign55820_e86474_d_n11, assign55820_e86474_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign55820_e86471: f64 = (2.0 * 2.0);
                let assign55820_e86472: f64 = (1.0 / assign55820_e86471);
                let assign55820_e86473: f64 = (locals.var_dnm).powf(assign55820_e86472);
                (assign55820_e86473, if 0.0 == 0.0 && ((assign55820_e86472) as f64).is_finite() && ((assign55820_e86472) as f64).fract() == 0.0 { if assign55820_e86472 == 0.0 { 0.0 } else { (assign55820_e86472 * ((locals.var_dnm).powf(assign55820_e86472 - 1.0) * locals.var_dnm_dn0)) } } else { (assign55820_e86473 * (assign55820_e86472 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55820_e86472) as f64).is_finite() && ((assign55820_e86472) as f64).fract() == 0.0 { if assign55820_e86472 == 0.0 { 0.0 } else { (assign55820_e86472 * ((locals.var_dnm).powf(assign55820_e86472 - 1.0) * locals.var_dnm_dn2)) } } else { (assign55820_e86473 * (assign55820_e86472 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55820_e86472) as f64).is_finite() && ((assign55820_e86472) as f64).fract() == 0.0 { if assign55820_e86472 == 0.0 { 0.0 } else { (assign55820_e86472 * ((locals.var_dnm).powf(assign55820_e86472 - 1.0) * locals.var_dnm_dn4)) } } else { (assign55820_e86473 * (assign55820_e86472 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55820_e86472) as f64).is_finite() && ((assign55820_e86472) as f64).fract() == 0.0 { if assign55820_e86472 == 0.0 { 0.0 } else { (assign55820_e86472 * ((locals.var_dnm).powf(assign55820_e86472 - 1.0) * locals.var_dnm_dn5)) } } else { (assign55820_e86473 * (assign55820_e86472 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55820_e86472) as f64).is_finite() && ((assign55820_e86472) as f64).fract() == 0.0 { if assign55820_e86472 == 0.0 { 0.0 } else { (assign55820_e86472 * ((locals.var_dnm).powf(assign55820_e86472 - 1.0) * locals.var_dnm_dn6)) } } else { (assign55820_e86473 * (assign55820_e86472 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55820_e86472) as f64).is_finite() && ((assign55820_e86472) as f64).fract() == 0.0 { if assign55820_e86472 == 0.0 { 0.0 } else { (assign55820_e86472 * ((locals.var_dnm).powf(assign55820_e86472 - 1.0) * locals.var_dnm_dn7)) } } else { (assign55820_e86473 * (assign55820_e86472 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55820_e86472) as f64).is_finite() && ((assign55820_e86472) as f64).fract() == 0.0 { if assign55820_e86472 == 0.0 { 0.0 } else { (assign55820_e86472 * ((locals.var_dnm).powf(assign55820_e86472 - 1.0) * locals.var_dnm_dn8)) } } else { (assign55820_e86473 * (assign55820_e86472 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55820_e86472) as f64).is_finite() && ((assign55820_e86472) as f64).fract() == 0.0 { if assign55820_e86472 == 0.0 { 0.0 } else { (assign55820_e86472 * ((locals.var_dnm).powf(assign55820_e86472 - 1.0) * locals.var_dnm_dn9)) } } else { (assign55820_e86473 * (assign55820_e86472 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55820_e86472) as f64).is_finite() && ((assign55820_e86472) as f64).fract() == 0.0 { if assign55820_e86472 == 0.0 { 0.0 } else { (assign55820_e86472 * ((locals.var_dnm).powf(assign55820_e86472 - 1.0) * locals.var_dnm_dn10)) } } else { (assign55820_e86473 * (assign55820_e86472 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55820_e86472) as f64).is_finite() && ((assign55820_e86472) as f64).fract() == 0.0 { if assign55820_e86472 == 0.0 { 0.0 } else { (assign55820_e86472 * ((locals.var_dnm).powf(assign55820_e86472 - 1.0) * locals.var_dnm_dn11)) } } else { (assign55820_e86473 * (assign55820_e86472 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55820_e86472) as f64).is_finite() && ((assign55820_e86472) as f64).fract() == 0.0 { if assign55820_e86472 == 0.0 { 0.0 } else { (assign55820_e86472 * ((locals.var_dnm).powf(assign55820_e86472 - 1.0) * locals.var_dnm_dn14)) } } else { (assign55820_e86473 * (assign55820_e86472 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign55820_e86474, assign55820_e86474_d_n0, assign55820_e86474_d_n2, assign55820_e86474_d_n4, assign55820_e86474_d_n5, assign55820_e86474_d_n6, assign55820_e86474_d_n7, assign55820_e86474_d_n8, assign55820_e86474_d_n9, assign55820_e86474_d_n10, assign55820_e86474_d_n11, assign55820_e86474_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign55820_e86476;
        locals.var_dnm_dn0 = assign55820_e86476_d_n0;
        locals.var_dnm_dn2 = assign55820_e86476_d_n2;
        locals.var_dnm_dn4 = assign55820_e86476_d_n4;
        locals.var_dnm_dn5 = assign55820_e86476_d_n5;
        locals.var_dnm_dn6 = assign55820_e86476_d_n6;
        locals.var_dnm_dn7 = assign55820_e86476_d_n7;
        locals.var_dnm_dn8 = assign55820_e86476_d_n8;
        locals.var_dnm_dn9 = assign55820_e86476_d_n9;
        locals.var_dnm_dn10 = assign55820_e86476_d_n10;
        locals.var_dnm_dn11 = assign55820_e86476_d_n11;
        locals.var_dnm_dn14 = assign55820_e86476_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign55830_e86493, assign55830_e86493_d_n0, assign55830_e86493_d_n2, assign55830_e86493_d_n4, assign55830_e86493_d_n5, assign55830_e86493_d_n6, assign55830_e86493_d_n7, assign55830_e86493_d_n8, assign55830_e86493_d_n9, assign55830_e86493_d_n10, assign55830_e86493_d_n11, assign55830_e86493_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1398 != 0.0)) {
        let assign55830_e86491: f64 = (1.0 / locals.var_dnm);
        (assign55830_e86491, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign55830_e86493;
        locals.var_dnm_dn0 = assign55830_e86493_d_n0;
        locals.var_dnm_dn2 = assign55830_e86493_d_n2;
        locals.var_dnm_dn4 = assign55830_e86493_d_n4;
        locals.var_dnm_dn5 = assign55830_e86493_d_n5;
        locals.var_dnm_dn6 = assign55830_e86493_d_n6;
        locals.var_dnm_dn7 = assign55830_e86493_d_n7;
        locals.var_dnm_dn8 = assign55830_e86493_d_n8;
        locals.var_dnm_dn9 = assign55830_e86493_d_n9;
        locals.var_dnm_dn10 = assign55830_e86493_d_n10;
        locals.var_dnm_dn11 = assign55830_e86493_d_n11;
        locals.var_dnm_dn14 = assign55830_e86493_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign55840_e86512, assign55840_e86512_d_n0, assign55840_e86512_d_n2, assign55840_e86512_d_n4, assign55840_e86512_d_n5, assign55840_e86512_d_n6, assign55840_e86512_d_n7, assign55840_e86512_d_n8, assign55840_e86512_d_n9, assign55840_e86512_d_n10, assign55840_e86512_d_n11, assign55840_e86512_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1398 != 0.0)) {
        let assign55840_e86508: f64 = (locals.var_tmf1 * locals.var_t5);
        let assign55840_e86510: f64 = (assign55840_e86508 * locals.var_dnm);
        (assign55840_e86510, ((((locals.var_tmf1_dn0 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn0)) * locals.var_dnm) + (assign55840_e86508 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn2)) * locals.var_dnm) + (assign55840_e86508 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn4)) * locals.var_dnm) + (assign55840_e86508 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn5)) * locals.var_dnm) + (assign55840_e86508 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn6)) * locals.var_dnm) + (assign55840_e86508 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn7)) * locals.var_dnm) + (assign55840_e86508 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn8)) * locals.var_dnm) + (assign55840_e86508 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn9)) * locals.var_dnm) + (assign55840_e86508 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn10)) * locals.var_dnm) + (assign55840_e86508 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn11)) * locals.var_dnm) + (assign55840_e86508 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn14)) * locals.var_dnm) + (assign55840_e86508 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign55840_e86512;
        locals.var_tmf0_dn0 = assign55840_e86512_d_n0;
        locals.var_tmf0_dn2 = assign55840_e86512_d_n2;
        locals.var_tmf0_dn4 = assign55840_e86512_d_n4;
        locals.var_tmf0_dn5 = assign55840_e86512_d_n5;
        locals.var_tmf0_dn6 = assign55840_e86512_d_n6;
        locals.var_tmf0_dn7 = assign55840_e86512_d_n7;
        locals.var_tmf0_dn8 = assign55840_e86512_d_n8;
        locals.var_tmf0_dn9 = assign55840_e86512_d_n9;
        locals.var_tmf0_dn10 = assign55840_e86512_d_n10;
        locals.var_tmf0_dn11 = assign55840_e86512_d_n11;
        locals.var_tmf0_dn14 = assign55840_e86512_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign55850_e86533, assign55850_e86533_d_n0, assign55850_e86533_d_n2, assign55850_e86533_d_n4, assign55850_e86533_d_n5, assign55850_e86533_d_n6, assign55850_e86533_d_n7, assign55850_e86533_d_n8, assign55850_e86533_d_n9, assign55850_e86533_d_n10, assign55850_e86533_d_n11, assign55850_e86533_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1398 != 0.0)) {
        let assign55850_e86527: f64 = (locals.var_t5 * locals.var_xmp);
        let assign55850_e86529: f64 = (assign55850_e86527 * locals.var_dnm);
        let assign55850_e86531: f64 = (assign55850_e86529 / locals.var_arg);
        (assign55850_e86531, (((((((locals.var_t5_dn0 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign55850_e86527 * locals.var_dnm_dn0)) * locals.var_arg) - (assign55850_e86529 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn2 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign55850_e86527 * locals.var_dnm_dn2)) * locals.var_arg) - (assign55850_e86529 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn4 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign55850_e86527 * locals.var_dnm_dn4)) * locals.var_arg) - (assign55850_e86529 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn5 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign55850_e86527 * locals.var_dnm_dn5)) * locals.var_arg) - (assign55850_e86529 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn6 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign55850_e86527 * locals.var_dnm_dn6)) * locals.var_arg) - (assign55850_e86529 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn7 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign55850_e86527 * locals.var_dnm_dn7)) * locals.var_arg) - (assign55850_e86529 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn8 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign55850_e86527 * locals.var_dnm_dn8)) * locals.var_arg) - (assign55850_e86529 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn9 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign55850_e86527 * locals.var_dnm_dn9)) * locals.var_arg) - (assign55850_e86529 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn10 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign55850_e86527 * locals.var_dnm_dn10)) * locals.var_arg) - (assign55850_e86529 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn11 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign55850_e86527 * locals.var_dnm_dn11)) * locals.var_arg) - (assign55850_e86529 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn14 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign55850_e86527 * locals.var_dnm_dn14)) * locals.var_arg) - (assign55850_e86529 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign55850_e86533;
        locals.var_t0_dn0 = assign55850_e86533_d_n0;
        locals.var_t0_dn2 = assign55850_e86533_d_n2;
        locals.var_t0_dn4 = assign55850_e86533_d_n4;
        locals.var_t0_dn5 = assign55850_e86533_d_n5;
        locals.var_t0_dn6 = assign55850_e86533_d_n6;
        locals.var_t0_dn7 = assign55850_e86533_d_n7;
        locals.var_t0_dn8 = assign55850_e86533_d_n8;
        locals.var_t0_dn9 = assign55850_e86533_d_n9;
        locals.var_t0_dn10 = assign55850_e86533_d_n10;
        locals.var_t0_dn11 = assign55850_e86533_d_n11;
        locals.var_t0_dn14 = assign55850_e86533_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign55860_e86552, assign55860_e86552_d_n0, assign55860_e86552_d_n2, assign55860_e86552_d_n4, assign55860_e86552_d_n5, assign55860_e86552_d_n6, assign55860_e86552_d_n7, assign55860_e86552_d_n8, assign55860_e86552_d_n9, assign55860_e86552_d_n10, assign55860_e86552_d_n11, assign55860_e86552_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1398 != 0.0)) {
        let assign55860_e86548: f64 = locals.var_t5;
        let assign55860_e86550: f64 = (assign55860_e86548 - locals.var_tmf0);
        (assign55860_e86550, (locals.var_t5_dn0 - locals.var_tmf0_dn0), (locals.var_t5_dn2 - locals.var_tmf0_dn2), (locals.var_t5_dn4 - locals.var_tmf0_dn4), (locals.var_t5_dn5 - locals.var_tmf0_dn5), (locals.var_t5_dn6 - locals.var_tmf0_dn6), (locals.var_t5_dn7 - locals.var_tmf0_dn7), (locals.var_t5_dn8 - locals.var_tmf0_dn8), (locals.var_t5_dn9 - locals.var_tmf0_dn9), (locals.var_t5_dn10 - locals.var_tmf0_dn10), (locals.var_t5_dn11 - locals.var_tmf0_dn11), (locals.var_t5_dn14 - locals.var_tmf0_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign55860_e86552;
        locals.var_t4_dn0 = assign55860_e86552_d_n0;
        locals.var_t4_dn2 = assign55860_e86552_d_n2;
        locals.var_t4_dn4 = assign55860_e86552_d_n4;
        locals.var_t4_dn5 = assign55860_e86552_d_n5;
        locals.var_t4_dn6 = assign55860_e86552_d_n6;
        locals.var_t4_dn7 = assign55860_e86552_d_n7;
        locals.var_t4_dn8 = assign55860_e86552_d_n8;
        locals.var_t4_dn9 = assign55860_e86552_d_n9;
        locals.var_t4_dn10 = assign55860_e86552_d_n10;
        locals.var_t4_dn11 = assign55860_e86552_d_n11;
        locals.var_t4_dn14 = assign55860_e86552_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign55870_e86567, assign55870_e86567_d_n0, assign55870_e86567_d_n2, assign55870_e86567_d_n4, assign55870_e86567_d_n5, assign55870_e86567_d_n6, assign55870_e86567_d_n7, assign55870_e86567_d_n8, assign55870_e86567_d_n9, assign55870_e86567_d_n10, assign55870_e86567_d_n11, assign55870_e86567_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1398 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign55870_e86567;
        locals.var_t0_dn0 = assign55870_e86567_d_n0;
        locals.var_t0_dn2 = assign55870_e86567_d_n2;
        locals.var_t0_dn4 = assign55870_e86567_d_n4;
        locals.var_t0_dn5 = assign55870_e86567_d_n5;
        locals.var_t0_dn6 = assign55870_e86567_d_n6;
        locals.var_t0_dn7 = assign55870_e86567_d_n7;
        locals.var_t0_dn8 = assign55870_e86567_d_n8;
        locals.var_t0_dn9 = assign55870_e86567_d_n9;
        locals.var_t0_dn10 = assign55870_e86567_d_n10;
        locals.var_t0_dn11 = assign55870_e86567_d_n11;
        locals.var_t0_dn14 = assign55870_e86567_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign55880_e86583, assign55880_e86583_d_n0, assign55880_e86583_d_n2, assign55880_e86583_d_n4, assign55880_e86583_d_n5, assign55880_e86583_d_n6, assign55880_e86583_d_n7, assign55880_e86583_d_n8, assign55880_e86583_d_n9, assign55880_e86583_d_n10, assign55880_e86583_d_n11, assign55880_e86583_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1398 == 0.0)) {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign55880_e86583;
        locals.var_t4_dn0 = assign55880_e86583_d_n0;
        locals.var_t4_dn2 = assign55880_e86583_d_n2;
        locals.var_t4_dn4 = assign55880_e86583_d_n4;
        locals.var_t4_dn5 = assign55880_e86583_d_n5;
        locals.var_t4_dn6 = assign55880_e86583_d_n6;
        locals.var_t4_dn7 = assign55880_e86583_d_n7;
        locals.var_t4_dn8 = assign55880_e86583_d_n8;
        locals.var_t4_dn9 = assign55880_e86583_d_n9;
        locals.var_t4_dn10 = assign55880_e86583_d_n10;
        locals.var_t4_dn11 = assign55880_e86583_d_n11;
        locals.var_t4_dn14 = assign55880_e86583_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign55890_e86599, assign55890_e86599_d_n0, assign55890_e86599_d_n2, assign55890_e86599_d_n4, assign55890_e86599_d_n5, assign55890_e86599_d_n6, assign55890_e86599_d_n7, assign55890_e86599_d_n8, assign55890_e86599_d_n9, assign55890_e86599_d_n10, assign55890_e86599_d_n11, assign55890_e86599_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1398 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign55890_e86599;
        locals.var_t0_dn0 = assign55890_e86599_d_n0;
        locals.var_t0_dn2 = assign55890_e86599_d_n2;
        locals.var_t0_dn4 = assign55890_e86599_d_n4;
        locals.var_t0_dn5 = assign55890_e86599_d_n5;
        locals.var_t0_dn6 = assign55890_e86599_d_n6;
        locals.var_t0_dn7 = assign55890_e86599_d_n7;
        locals.var_t0_dn8 = assign55890_e86599_d_n8;
        locals.var_t0_dn9 = assign55890_e86599_d_n9;
        locals.var_t0_dn10 = assign55890_e86599_d_n10;
        locals.var_t0_dn11 = assign55890_e86599_d_n11;
        locals.var_t0_dn14 = assign55890_e86599_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign55900_e86613, assign55900_e86613_d_n0, assign55900_e86613_d_n2, assign55900_e86613_d_n4, assign55900_e86613_d_n5, assign55900_e86613_d_n6, assign55900_e86613_d_n7, assign55900_e86613_d_n8, assign55900_e86613_d_n9, assign55900_e86613_d_n10, assign55900_e86613_d_n11, assign55900_e86613_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) {
        let assign55900_e86611: f64 = (locals.var_t4).sqrt();
        (assign55900_e86611, (locals.var_t4_dn0 / (2.0 * assign55900_e86611)), (locals.var_t4_dn2 / (2.0 * assign55900_e86611)), (locals.var_t4_dn4 / (2.0 * assign55900_e86611)), (locals.var_t4_dn5 / (2.0 * assign55900_e86611)), (locals.var_t4_dn6 / (2.0 * assign55900_e86611)), (locals.var_t4_dn7 / (2.0 * assign55900_e86611)), (locals.var_t4_dn8 / (2.0 * assign55900_e86611)), (locals.var_t4_dn9 / (2.0 * assign55900_e86611)), (locals.var_t4_dn10 / (2.0 * assign55900_e86611)), (locals.var_t4_dn11 / (2.0 * assign55900_e86611)), (locals.var_t4_dn14 / (2.0 * assign55900_e86611)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign55900_e86613;
        locals.var_t3_dn0 = assign55900_e86613_d_n0;
        locals.var_t3_dn2 = assign55900_e86613_d_n2;
        locals.var_t3_dn4 = assign55900_e86613_d_n4;
        locals.var_t3_dn5 = assign55900_e86613_d_n5;
        locals.var_t3_dn6 = assign55900_e86613_d_n6;
        locals.var_t3_dn7 = assign55900_e86613_d_n7;
        locals.var_t3_dn8 = assign55900_e86613_d_n8;
        locals.var_t3_dn9 = assign55900_e86613_d_n9;
        locals.var_t3_dn10 = assign55900_e86613_d_n10;
        locals.var_t3_dn11 = assign55900_e86613_d_n11;
        locals.var_t3_dn14 = assign55900_e86613_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign55910_e86632, assign55910_e86632_d_n0, assign55910_e86632_d_n2, assign55910_e86632_d_n4, assign55910_e86632_d_n5, assign55910_e86632_d_n6, assign55910_e86632_d_n7, assign55910_e86632_d_n8, assign55910_e86632_d_n9, assign55910_e86632_d_n10, assign55910_e86632_d_n11, assign55910_e86632_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) {
        let assign55910_e86628: f64 = (1.0 - locals.var_t3);
        let assign55910_e86629: f64 = (locals.var_q_ndepm_esi_cox_inv2__blk1136 * assign55910_e86628);
        let assign55910_e86630: f64 = (locals.var_vgp + assign55910_e86629);
        (assign55910_e86630, (locals.var_vgp_dn0 + ((locals.var_q_ndepm_esi_cox_inv2__blk1136_dn0 * assign55910_e86628) + (locals.var_q_ndepm_esi_cox_inv2__blk1136 * (-locals.var_t3_dn0)))), (locals.var_vgp_dn2 + ((locals.var_q_ndepm_esi_cox_inv2__blk1136_dn2 * assign55910_e86628) + (locals.var_q_ndepm_esi_cox_inv2__blk1136 * (-locals.var_t3_dn2)))), (locals.var_vgp_dn4 + ((locals.var_q_ndepm_esi_cox_inv2__blk1136_dn4 * assign55910_e86628) + (locals.var_q_ndepm_esi_cox_inv2__blk1136 * (-locals.var_t3_dn4)))), (locals.var_vgp_dn5 + ((locals.var_q_ndepm_esi_cox_inv2__blk1136_dn5 * assign55910_e86628) + (locals.var_q_ndepm_esi_cox_inv2__blk1136 * (-locals.var_t3_dn5)))), (locals.var_vgp_dn6 + ((locals.var_q_ndepm_esi_cox_inv2__blk1136_dn6 * assign55910_e86628) + (locals.var_q_ndepm_esi_cox_inv2__blk1136 * (-locals.var_t3_dn6)))), (locals.var_vgp_dn7 + ((locals.var_q_ndepm_esi_cox_inv2__blk1136_dn7 * assign55910_e86628) + (locals.var_q_ndepm_esi_cox_inv2__blk1136 * (-locals.var_t3_dn7)))), (locals.var_vgp_dn8 + ((locals.var_q_ndepm_esi_cox_inv2__blk1136_dn8 * assign55910_e86628) + (locals.var_q_ndepm_esi_cox_inv2__blk1136 * (-locals.var_t3_dn8)))), (locals.var_vgp_dn9 + ((locals.var_q_ndepm_esi_cox_inv2__blk1136_dn9 * assign55910_e86628) + (locals.var_q_ndepm_esi_cox_inv2__blk1136 * (-locals.var_t3_dn9)))), (locals.var_vgp_dn10 + ((locals.var_q_ndepm_esi_cox_inv2__blk1136_dn10 * assign55910_e86628) + (locals.var_q_ndepm_esi_cox_inv2__blk1136 * (-locals.var_t3_dn10)))), (locals.var_vgp_dn11 + ((locals.var_q_ndepm_esi_cox_inv2__blk1136_dn11 * assign55910_e86628) + (locals.var_q_ndepm_esi_cox_inv2__blk1136 * (-locals.var_t3_dn11)))), (locals.var_vgp_dn14 + ((locals.var_q_ndepm_esi_cox_inv2__blk1136_dn14 * assign55910_e86628) + (locals.var_q_ndepm_esi_cox_inv2__blk1136 * (-locals.var_t3_dn14)))),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign55910_e86632;
        locals.var_t10_dn0 = assign55910_e86632_d_n0;
        locals.var_t10_dn2 = assign55910_e86632_d_n2;
        locals.var_t10_dn4 = assign55910_e86632_d_n4;
        locals.var_t10_dn5 = assign55910_e86632_d_n5;
        locals.var_t10_dn6 = assign55910_e86632_d_n6;
        locals.var_t10_dn7 = assign55910_e86632_d_n7;
        locals.var_t10_dn8 = assign55910_e86632_d_n8;
        locals.var_t10_dn9 = assign55910_e86632_d_n9;
        locals.var_t10_dn10 = assign55910_e86632_d_n10;
        locals.var_t10_dn11 = assign55910_e86632_d_n11;
        locals.var_t10_dn14 = assign55910_e86632_d_n14;
        locals.var_t10_rv = 0.0;

        let assign55920_e86636: f64 = (locals.var_uc_depleak + p.p405);
        let assign55920_e86641: f64 = if ((locals.var_t10 < assign55920_e86636) && (p.p405 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1404 = assign55920_e86641;
        locals.var_guard1404_rv = 0.0;

        let (assign55930_e86660, assign55930_e86660_d_n0, assign55930_e86660_d_n2, assign55930_e86660_d_n4, assign55930_e86660_d_n5, assign55930_e86660_d_n6, assign55930_e86660_d_n7, assign55930_e86660_d_n8, assign55930_e86660_d_n9, assign55930_e86660_d_n10, assign55930_e86660_d_n11, assign55930_e86660_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1404 != 0.0)) {
        let assign55930_e86656: f64 = (locals.var_uc_depleak + p.p405);
        let assign55930_e86658: f64 = (assign55930_e86656 - locals.var_t10);
        (assign55930_e86658, (locals.var_uc_depleak_dn0 - locals.var_t10_dn0), (locals.var_uc_depleak_dn2 - locals.var_t10_dn2), (locals.var_uc_depleak_dn4 - locals.var_t10_dn4), (locals.var_uc_depleak_dn5 - locals.var_t10_dn5), (locals.var_uc_depleak_dn6 - locals.var_t10_dn6), (locals.var_uc_depleak_dn7 - locals.var_t10_dn7), (locals.var_uc_depleak_dn8 - locals.var_t10_dn8), (locals.var_uc_depleak_dn9 - locals.var_t10_dn9), (locals.var_uc_depleak_dn10 - locals.var_t10_dn10), (locals.var_uc_depleak_dn11 - locals.var_t10_dn11), (locals.var_uc_depleak_dn14 - locals.var_t10_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign55930_e86660;
        locals.var_tmf1_dn0 = assign55930_e86660_d_n0;
        locals.var_tmf1_dn2 = assign55930_e86660_d_n2;
        locals.var_tmf1_dn4 = assign55930_e86660_d_n4;
        locals.var_tmf1_dn5 = assign55930_e86660_d_n5;
        locals.var_tmf1_dn6 = assign55930_e86660_d_n6;
        locals.var_tmf1_dn7 = assign55930_e86660_d_n7;
        locals.var_tmf1_dn8 = assign55930_e86660_d_n8;
        locals.var_tmf1_dn9 = assign55930_e86660_d_n9;
        locals.var_tmf1_dn10 = assign55930_e86660_d_n10;
        locals.var_tmf1_dn11 = assign55930_e86660_d_n11;
        locals.var_tmf1_dn14 = assign55930_e86660_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign55940_e86677, assign55940_e86677_d_n0, assign55940_e86677_d_n2, assign55940_e86677_d_n4, assign55940_e86677_d_n5, assign55940_e86677_d_n6, assign55940_e86677_d_n7, assign55940_e86677_d_n8, assign55940_e86677_d_n9, assign55940_e86677_d_n10, assign55940_e86677_d_n11, assign55940_e86677_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1404 != 0.0)) {
        let assign55940_e86675: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign55940_e86675, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign55940_e86677;
        locals.var_x2_dn0 = assign55940_e86677_d_n0;
        locals.var_x2_dn2 = assign55940_e86677_d_n2;
        locals.var_x2_dn4 = assign55940_e86677_d_n4;
        locals.var_x2_dn5 = assign55940_e86677_d_n5;
        locals.var_x2_dn6 = assign55940_e86677_d_n6;
        locals.var_x2_dn7 = assign55940_e86677_d_n7;
        locals.var_x2_dn8 = assign55940_e86677_d_n8;
        locals.var_x2_dn9 = assign55940_e86677_d_n9;
        locals.var_x2_dn10 = assign55940_e86677_d_n10;
        locals.var_x2_dn11 = assign55940_e86677_d_n11;
        locals.var_x2_dn14 = assign55940_e86677_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign55950_e86694, assign55950_e86694_d_n0, assign55950_e86694_d_n2, assign55950_e86694_d_n4, assign55950_e86694_d_n5, assign55950_e86694_d_n6, assign55950_e86694_d_n7, assign55950_e86694_d_n8, assign55950_e86694_d_n9, assign55950_e86694_d_n10, assign55950_e86694_d_n11, assign55950_e86694_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1404 != 0.0)) {
        let assign55950_e86692: f64 = (p.p405 * p.p405);
        (assign55950_e86692, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign55950_e86694;
        locals.var_xmax2_dn0 = assign55950_e86694_d_n0;
        locals.var_xmax2_dn2 = assign55950_e86694_d_n2;
        locals.var_xmax2_dn4 = assign55950_e86694_d_n4;
        locals.var_xmax2_dn5 = assign55950_e86694_d_n5;
        locals.var_xmax2_dn6 = assign55950_e86694_d_n6;
        locals.var_xmax2_dn7 = assign55950_e86694_d_n7;
        locals.var_xmax2_dn8 = assign55950_e86694_d_n8;
        locals.var_xmax2_dn9 = assign55950_e86694_d_n9;
        locals.var_xmax2_dn10 = assign55950_e86694_d_n10;
        locals.var_xmax2_dn11 = assign55950_e86694_d_n11;
        locals.var_xmax2_dn14 = assign55950_e86694_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign55960_e86709, assign55960_e86709_d_n0, assign55960_e86709_d_n2, assign55960_e86709_d_n4, assign55960_e86709_d_n5, assign55960_e86709_d_n6, assign55960_e86709_d_n7, assign55960_e86709_d_n8, assign55960_e86709_d_n9, assign55960_e86709_d_n10, assign55960_e86709_d_n11, assign55960_e86709_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1404 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign55960_e86709;
        locals.var_xp_dn0 = assign55960_e86709_d_n0;
        locals.var_xp_dn2 = assign55960_e86709_d_n2;
        locals.var_xp_dn4 = assign55960_e86709_d_n4;
        locals.var_xp_dn5 = assign55960_e86709_d_n5;
        locals.var_xp_dn6 = assign55960_e86709_d_n6;
        locals.var_xp_dn7 = assign55960_e86709_d_n7;
        locals.var_xp_dn8 = assign55960_e86709_d_n8;
        locals.var_xp_dn9 = assign55960_e86709_d_n9;
        locals.var_xp_dn10 = assign55960_e86709_d_n10;
        locals.var_xp_dn11 = assign55960_e86709_d_n11;
        locals.var_xp_dn14 = assign55960_e86709_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign55970_e86724, assign55970_e86724_d_n0, assign55970_e86724_d_n2, assign55970_e86724_d_n4, assign55970_e86724_d_n5, assign55970_e86724_d_n6, assign55970_e86724_d_n7, assign55970_e86724_d_n8, assign55970_e86724_d_n9, assign55970_e86724_d_n10, assign55970_e86724_d_n11, assign55970_e86724_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1404 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign55970_e86724;
        locals.var_xmp_dn0 = assign55970_e86724_d_n0;
        locals.var_xmp_dn2 = assign55970_e86724_d_n2;
        locals.var_xmp_dn4 = assign55970_e86724_d_n4;
        locals.var_xmp_dn5 = assign55970_e86724_d_n5;
        locals.var_xmp_dn6 = assign55970_e86724_d_n6;
        locals.var_xmp_dn7 = assign55970_e86724_d_n7;
        locals.var_xmp_dn8 = assign55970_e86724_d_n8;
        locals.var_xmp_dn9 = assign55970_e86724_d_n9;
        locals.var_xmp_dn10 = assign55970_e86724_d_n10;
        locals.var_xmp_dn11 = assign55970_e86724_d_n11;
        locals.var_xmp_dn14 = assign55970_e86724_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign55980_e86739,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1404 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign55980_e86739;
        locals.var_m0_rv = 0.0;

        let (assign55990_e86754,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1404 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign55990_e86754;
        locals.var_mm_rv = 0.0;

        let (assign56000_e86769, assign56000_e86769_d_n0, assign56000_e86769_d_n2, assign56000_e86769_d_n4, assign56000_e86769_d_n5, assign56000_e86769_d_n6, assign56000_e86769_d_n7, assign56000_e86769_d_n8, assign56000_e86769_d_n9, assign56000_e86769_d_n10, assign56000_e86769_d_n11, assign56000_e86769_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1404 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign56000_e86769;
        locals.var_arg_dn0 = assign56000_e86769_d_n0;
        locals.var_arg_dn2 = assign56000_e86769_d_n2;
        locals.var_arg_dn4 = assign56000_e86769_d_n4;
        locals.var_arg_dn5 = assign56000_e86769_d_n5;
        locals.var_arg_dn6 = assign56000_e86769_d_n6;
        locals.var_arg_dn7 = assign56000_e86769_d_n7;
        locals.var_arg_dn8 = assign56000_e86769_d_n8;
        locals.var_arg_dn9 = assign56000_e86769_d_n9;
        locals.var_arg_dn10 = assign56000_e86769_d_n10;
        locals.var_arg_dn11 = assign56000_e86769_d_n11;
        locals.var_arg_dn14 = assign56000_e86769_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign56010_e86784, assign56010_e86784_d_n0, assign56010_e86784_d_n2, assign56010_e86784_d_n4, assign56010_e86784_d_n5, assign56010_e86784_d_n6, assign56010_e86784_d_n7, assign56010_e86784_d_n8, assign56010_e86784_d_n9, assign56010_e86784_d_n10, assign56010_e86784_d_n11, assign56010_e86784_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1404 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign56010_e86784;
        locals.var_dnm_dn0 = assign56010_e86784_d_n0;
        locals.var_dnm_dn2 = assign56010_e86784_d_n2;
        locals.var_dnm_dn4 = assign56010_e86784_d_n4;
        locals.var_dnm_dn5 = assign56010_e86784_d_n5;
        locals.var_dnm_dn6 = assign56010_e86784_d_n6;
        locals.var_dnm_dn7 = assign56010_e86784_d_n7;
        locals.var_dnm_dn8 = assign56010_e86784_d_n8;
        locals.var_dnm_dn9 = assign56010_e86784_d_n9;
        locals.var_dnm_dn10 = assign56010_e86784_d_n10;
        locals.var_dnm_dn11 = assign56010_e86784_d_n11;
        locals.var_dnm_dn14 = assign56010_e86784_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign56020_e86801, assign56020_e86801_d_n0, assign56020_e86801_d_n2, assign56020_e86801_d_n4, assign56020_e86801_d_n5, assign56020_e86801_d_n6, assign56020_e86801_d_n7, assign56020_e86801_d_n8, assign56020_e86801_d_n9, assign56020_e86801_d_n10, assign56020_e86801_d_n11, assign56020_e86801_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1404 != 0.0)) {
        let assign56020_e86799: f64 = (locals.var_xp * locals.var_x2);
        (assign56020_e86799, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign56020_e86801;
        locals.var_xp_dn0 = assign56020_e86801_d_n0;
        locals.var_xp_dn2 = assign56020_e86801_d_n2;
        locals.var_xp_dn4 = assign56020_e86801_d_n4;
        locals.var_xp_dn5 = assign56020_e86801_d_n5;
        locals.var_xp_dn6 = assign56020_e86801_d_n6;
        locals.var_xp_dn7 = assign56020_e86801_d_n7;
        locals.var_xp_dn8 = assign56020_e86801_d_n8;
        locals.var_xp_dn9 = assign56020_e86801_d_n9;
        locals.var_xp_dn10 = assign56020_e86801_d_n10;
        locals.var_xp_dn11 = assign56020_e86801_d_n11;
        locals.var_xp_dn14 = assign56020_e86801_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign56030_e86818, assign56030_e86818_d_n0, assign56030_e86818_d_n2, assign56030_e86818_d_n4, assign56030_e86818_d_n5, assign56030_e86818_d_n6, assign56030_e86818_d_n7, assign56030_e86818_d_n8, assign56030_e86818_d_n9, assign56030_e86818_d_n10, assign56030_e86818_d_n11, assign56030_e86818_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1404 != 0.0)) {
        let assign56030_e86816: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign56030_e86816, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign56030_e86818;
        locals.var_xmp_dn0 = assign56030_e86818_d_n0;
        locals.var_xmp_dn2 = assign56030_e86818_d_n2;
        locals.var_xmp_dn4 = assign56030_e86818_d_n4;
        locals.var_xmp_dn5 = assign56030_e86818_d_n5;
        locals.var_xmp_dn6 = assign56030_e86818_d_n6;
        locals.var_xmp_dn7 = assign56030_e86818_d_n7;
        locals.var_xmp_dn8 = assign56030_e86818_d_n8;
        locals.var_xmp_dn9 = assign56030_e86818_d_n9;
        locals.var_xmp_dn10 = assign56030_e86818_d_n10;
        locals.var_xmp_dn11 = assign56030_e86818_d_n11;
        locals.var_xmp_dn14 = assign56030_e86818_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign56040_e86835, assign56040_e86835_d_n0, assign56040_e86835_d_n2, assign56040_e86835_d_n4, assign56040_e86835_d_n5, assign56040_e86835_d_n6, assign56040_e86835_d_n7, assign56040_e86835_d_n8, assign56040_e86835_d_n9, assign56040_e86835_d_n10, assign56040_e86835_d_n11, assign56040_e86835_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1404 != 0.0)) {
        let assign56040_e86833: f64 = (locals.var_xp * locals.var_x2);
        (assign56040_e86833, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign56040_e86835;
        locals.var_xp_dn0 = assign56040_e86835_d_n0;
        locals.var_xp_dn2 = assign56040_e86835_d_n2;
        locals.var_xp_dn4 = assign56040_e86835_d_n4;
        locals.var_xp_dn5 = assign56040_e86835_d_n5;
        locals.var_xp_dn6 = assign56040_e86835_d_n6;
        locals.var_xp_dn7 = assign56040_e86835_d_n7;
        locals.var_xp_dn8 = assign56040_e86835_d_n8;
        locals.var_xp_dn9 = assign56040_e86835_d_n9;
        locals.var_xp_dn10 = assign56040_e86835_d_n10;
        locals.var_xp_dn11 = assign56040_e86835_d_n11;
        locals.var_xp_dn14 = assign56040_e86835_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign56050_e86852, assign56050_e86852_d_n0, assign56050_e86852_d_n2, assign56050_e86852_d_n4, assign56050_e86852_d_n5, assign56050_e86852_d_n6, assign56050_e86852_d_n7, assign56050_e86852_d_n8, assign56050_e86852_d_n9, assign56050_e86852_d_n10, assign56050_e86852_d_n11, assign56050_e86852_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1404 != 0.0)) {
        let assign56050_e86850: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign56050_e86850, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign56050_e86852;
        locals.var_xmp_dn0 = assign56050_e86852_d_n0;
        locals.var_xmp_dn2 = assign56050_e86852_d_n2;
        locals.var_xmp_dn4 = assign56050_e86852_d_n4;
        locals.var_xmp_dn5 = assign56050_e86852_d_n5;
        locals.var_xmp_dn6 = assign56050_e86852_d_n6;
        locals.var_xmp_dn7 = assign56050_e86852_d_n7;
        locals.var_xmp_dn8 = assign56050_e86852_d_n8;
        locals.var_xmp_dn9 = assign56050_e86852_d_n9;
        locals.var_xmp_dn10 = assign56050_e86852_d_n10;
        locals.var_xmp_dn11 = assign56050_e86852_d_n11;
        locals.var_xmp_dn14 = assign56050_e86852_d_n14;
        locals.var_xmp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_204(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign56060_e86869, assign56060_e86869_d_n0, assign56060_e86869_d_n2, assign56060_e86869_d_n4, assign56060_e86869_d_n5, assign56060_e86869_d_n6, assign56060_e86869_d_n7, assign56060_e86869_d_n8, assign56060_e86869_d_n9, assign56060_e86869_d_n10, assign56060_e86869_d_n11, assign56060_e86869_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1404 != 0.0)) {
        let assign56060_e86867: f64 = (locals.var_xp + locals.var_xmp);
        (assign56060_e86867, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign56060_e86869;
        locals.var_arg_dn0 = assign56060_e86869_d_n0;
        locals.var_arg_dn2 = assign56060_e86869_d_n2;
        locals.var_arg_dn4 = assign56060_e86869_d_n4;
        locals.var_arg_dn5 = assign56060_e86869_d_n5;
        locals.var_arg_dn6 = assign56060_e86869_d_n6;
        locals.var_arg_dn7 = assign56060_e86869_d_n7;
        locals.var_arg_dn8 = assign56060_e86869_d_n8;
        locals.var_arg_dn9 = assign56060_e86869_d_n9;
        locals.var_arg_dn10 = assign56060_e86869_d_n10;
        locals.var_arg_dn11 = assign56060_e86869_d_n11;
        locals.var_arg_dn14 = assign56060_e86869_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign56070_e86884, assign56070_e86884_d_n0, assign56070_e86884_d_n2, assign56070_e86884_d_n4, assign56070_e86884_d_n5, assign56070_e86884_d_n6, assign56070_e86884_d_n7, assign56070_e86884_d_n8, assign56070_e86884_d_n9, assign56070_e86884_d_n10, assign56070_e86884_d_n11, assign56070_e86884_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1404 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign56070_e86884;
        locals.var_dnm_dn0 = assign56070_e86884_d_n0;
        locals.var_dnm_dn2 = assign56070_e86884_d_n2;
        locals.var_dnm_dn4 = assign56070_e86884_d_n4;
        locals.var_dnm_dn5 = assign56070_e86884_d_n5;
        locals.var_dnm_dn6 = assign56070_e86884_d_n6;
        locals.var_dnm_dn7 = assign56070_e86884_d_n7;
        locals.var_dnm_dn8 = assign56070_e86884_d_n8;
        locals.var_dnm_dn9 = assign56070_e86884_d_n9;
        locals.var_dnm_dn10 = assign56070_e86884_d_n10;
        locals.var_dnm_dn11 = assign56070_e86884_d_n11;
        locals.var_dnm_dn14 = assign56070_e86884_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign56080_e86899: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1405 = assign56080_e86899;
        locals.var_guard1405_rv = 0.0;

        let assign56090_e86902: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1406 = assign56090_e86902;
        locals.var_guard1406_rv = 0.0;

        let (assign56100_e86921,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1404 != 0.0)) && (locals.var_guard1405 != 0.0)) && (locals.var_guard1406 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign56100_e86921;
        locals.var_mm_rv = 0.0;

        let assign56110_e86924: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1407 = assign56110_e86924;
        locals.var_guard1407_rv = 0.0;

        let (assign56120_e86946,) = {
    if (((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1404 != 0.0)) && (locals.var_guard1405 != 0.0)) && (locals.var_guard1406 == 0.0)) && (locals.var_guard1407 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign56120_e86946;
        locals.var_mm_rv = 0.0;

        let assign56130_e86949: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1408 = assign56130_e86949;
        locals.var_guard1408_rv = 0.0;

        let (assign56140_e86974,) = {
    if ((((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1404 != 0.0)) && (locals.var_guard1405 != 0.0)) && (locals.var_guard1406 == 0.0)) && (locals.var_guard1407 == 0.0)) && (locals.var_guard1408 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign56140_e86974;
        locals.var_mm_rv = 0.0;

        let assign56150_e86977: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1409 = assign56150_e86977;
        locals.var_guard1409_rv = 0.0;

        let (assign56160_e87005,) = {
    if (((((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1404 != 0.0)) && (locals.var_guard1405 != 0.0)) && (locals.var_guard1406 == 0.0)) && (locals.var_guard1407 == 0.0)) && (locals.var_guard1408 == 0.0)) && (locals.var_guard1409 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign56160_e87005;
        locals.var_mm_rv = 0.0;

        let (assign56170_e87022,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1404 != 0.0)) && (locals.var_guard1405 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign56170_e87022;
        locals.var_m0_rv = 0.0;

        let mut assign56180_loop_guard: usize = 0;
        while {
            let assign56180_cond_e87040: f64 = if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1404 != 0.0)) && (locals.var_guard1405 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign56180_cond_e87040 != 0.0
        } {
            assign56180_loop_guard += 1;
            assert!(assign56180_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign56180_body0_e87058, assign56180_body0_e87058_d_n0, assign56180_body0_e87058_d_n2, assign56180_body0_e87058_d_n4, assign56180_body0_e87058_d_n5, assign56180_body0_e87058_d_n6, assign56180_body0_e87058_d_n7, assign56180_body0_e87058_d_n8, assign56180_body0_e87058_d_n9, assign56180_body0_e87058_d_n10, assign56180_body0_e87058_d_n11, assign56180_body0_e87058_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1404 != 0.0)) && (locals.var_guard1405 != 0.0)) {
        let assign56180_body0_e87056: f64 = (locals.var_dnm).sqrt();
        (assign56180_body0_e87056, (locals.var_dnm_dn0 / (2.0 * assign56180_body0_e87056)), (locals.var_dnm_dn2 / (2.0 * assign56180_body0_e87056)), (locals.var_dnm_dn4 / (2.0 * assign56180_body0_e87056)), (locals.var_dnm_dn5 / (2.0 * assign56180_body0_e87056)), (locals.var_dnm_dn6 / (2.0 * assign56180_body0_e87056)), (locals.var_dnm_dn7 / (2.0 * assign56180_body0_e87056)), (locals.var_dnm_dn8 / (2.0 * assign56180_body0_e87056)), (locals.var_dnm_dn9 / (2.0 * assign56180_body0_e87056)), (locals.var_dnm_dn10 / (2.0 * assign56180_body0_e87056)), (locals.var_dnm_dn11 / (2.0 * assign56180_body0_e87056)), (locals.var_dnm_dn14 / (2.0 * assign56180_body0_e87056)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign56180_body0_e87058;
            locals.var_dnm_dn0 = assign56180_body0_e87058_d_n0;
            locals.var_dnm_dn2 = assign56180_body0_e87058_d_n2;
            locals.var_dnm_dn4 = assign56180_body0_e87058_d_n4;
            locals.var_dnm_dn5 = assign56180_body0_e87058_d_n5;
            locals.var_dnm_dn6 = assign56180_body0_e87058_d_n6;
            locals.var_dnm_dn7 = assign56180_body0_e87058_d_n7;
            locals.var_dnm_dn8 = assign56180_body0_e87058_d_n8;
            locals.var_dnm_dn9 = assign56180_body0_e87058_d_n9;
            locals.var_dnm_dn10 = assign56180_body0_e87058_d_n10;
            locals.var_dnm_dn11 = assign56180_body0_e87058_d_n11;
            locals.var_dnm_dn14 = assign56180_body0_e87058_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign56180_body1_e87077,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1404 != 0.0)) && (locals.var_guard1405 != 0.0)) {
        let assign56180_body1_e87075: f64 = (locals.var_m0 + 1.0);
        (assign56180_body1_e87075,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign56180_body1_e87077;
            locals.var_m0_rv = 0.0;
        }

        let (assign56190_e87106, assign56190_e87106_d_n0, assign56190_e87106_d_n2, assign56190_e87106_d_n4, assign56190_e87106_d_n5, assign56190_e87106_d_n6, assign56190_e87106_d_n7, assign56190_e87106_d_n8, assign56190_e87106_d_n9, assign56190_e87106_d_n10, assign56190_e87106_d_n11, assign56190_e87106_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1404 != 0.0)) && (locals.var_guard1405 == 0.0)) {
        let (assign56190_e87104, assign56190_e87104_d_n0, assign56190_e87104_d_n2, assign56190_e87104_d_n4, assign56190_e87104_d_n5, assign56190_e87104_d_n6, assign56190_e87104_d_n7, assign56190_e87104_d_n8, assign56190_e87104_d_n9, assign56190_e87104_d_n10, assign56190_e87104_d_n11, assign56190_e87104_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign56190_e87101: f64 = (2.0 * 2.0);
                let assign56190_e87102: f64 = (1.0 / assign56190_e87101);
                let assign56190_e87103: f64 = (locals.var_dnm).powf(assign56190_e87102);
                (assign56190_e87103, if 0.0 == 0.0 && ((assign56190_e87102) as f64).is_finite() && ((assign56190_e87102) as f64).fract() == 0.0 { if assign56190_e87102 == 0.0 { 0.0 } else { (assign56190_e87102 * ((locals.var_dnm).powf(assign56190_e87102 - 1.0) * locals.var_dnm_dn0)) } } else { (assign56190_e87103 * (assign56190_e87102 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56190_e87102) as f64).is_finite() && ((assign56190_e87102) as f64).fract() == 0.0 { if assign56190_e87102 == 0.0 { 0.0 } else { (assign56190_e87102 * ((locals.var_dnm).powf(assign56190_e87102 - 1.0) * locals.var_dnm_dn2)) } } else { (assign56190_e87103 * (assign56190_e87102 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56190_e87102) as f64).is_finite() && ((assign56190_e87102) as f64).fract() == 0.0 { if assign56190_e87102 == 0.0 { 0.0 } else { (assign56190_e87102 * ((locals.var_dnm).powf(assign56190_e87102 - 1.0) * locals.var_dnm_dn4)) } } else { (assign56190_e87103 * (assign56190_e87102 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56190_e87102) as f64).is_finite() && ((assign56190_e87102) as f64).fract() == 0.0 { if assign56190_e87102 == 0.0 { 0.0 } else { (assign56190_e87102 * ((locals.var_dnm).powf(assign56190_e87102 - 1.0) * locals.var_dnm_dn5)) } } else { (assign56190_e87103 * (assign56190_e87102 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56190_e87102) as f64).is_finite() && ((assign56190_e87102) as f64).fract() == 0.0 { if assign56190_e87102 == 0.0 { 0.0 } else { (assign56190_e87102 * ((locals.var_dnm).powf(assign56190_e87102 - 1.0) * locals.var_dnm_dn6)) } } else { (assign56190_e87103 * (assign56190_e87102 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56190_e87102) as f64).is_finite() && ((assign56190_e87102) as f64).fract() == 0.0 { if assign56190_e87102 == 0.0 { 0.0 } else { (assign56190_e87102 * ((locals.var_dnm).powf(assign56190_e87102 - 1.0) * locals.var_dnm_dn7)) } } else { (assign56190_e87103 * (assign56190_e87102 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56190_e87102) as f64).is_finite() && ((assign56190_e87102) as f64).fract() == 0.0 { if assign56190_e87102 == 0.0 { 0.0 } else { (assign56190_e87102 * ((locals.var_dnm).powf(assign56190_e87102 - 1.0) * locals.var_dnm_dn8)) } } else { (assign56190_e87103 * (assign56190_e87102 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56190_e87102) as f64).is_finite() && ((assign56190_e87102) as f64).fract() == 0.0 { if assign56190_e87102 == 0.0 { 0.0 } else { (assign56190_e87102 * ((locals.var_dnm).powf(assign56190_e87102 - 1.0) * locals.var_dnm_dn9)) } } else { (assign56190_e87103 * (assign56190_e87102 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56190_e87102) as f64).is_finite() && ((assign56190_e87102) as f64).fract() == 0.0 { if assign56190_e87102 == 0.0 { 0.0 } else { (assign56190_e87102 * ((locals.var_dnm).powf(assign56190_e87102 - 1.0) * locals.var_dnm_dn10)) } } else { (assign56190_e87103 * (assign56190_e87102 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56190_e87102) as f64).is_finite() && ((assign56190_e87102) as f64).fract() == 0.0 { if assign56190_e87102 == 0.0 { 0.0 } else { (assign56190_e87102 * ((locals.var_dnm).powf(assign56190_e87102 - 1.0) * locals.var_dnm_dn11)) } } else { (assign56190_e87103 * (assign56190_e87102 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56190_e87102) as f64).is_finite() && ((assign56190_e87102) as f64).fract() == 0.0 { if assign56190_e87102 == 0.0 { 0.0 } else { (assign56190_e87102 * ((locals.var_dnm).powf(assign56190_e87102 - 1.0) * locals.var_dnm_dn14)) } } else { (assign56190_e87103 * (assign56190_e87102 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign56190_e87104, assign56190_e87104_d_n0, assign56190_e87104_d_n2, assign56190_e87104_d_n4, assign56190_e87104_d_n5, assign56190_e87104_d_n6, assign56190_e87104_d_n7, assign56190_e87104_d_n8, assign56190_e87104_d_n9, assign56190_e87104_d_n10, assign56190_e87104_d_n11, assign56190_e87104_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign56190_e87106;
        locals.var_dnm_dn0 = assign56190_e87106_d_n0;
        locals.var_dnm_dn2 = assign56190_e87106_d_n2;
        locals.var_dnm_dn4 = assign56190_e87106_d_n4;
        locals.var_dnm_dn5 = assign56190_e87106_d_n5;
        locals.var_dnm_dn6 = assign56190_e87106_d_n6;
        locals.var_dnm_dn7 = assign56190_e87106_d_n7;
        locals.var_dnm_dn8 = assign56190_e87106_d_n8;
        locals.var_dnm_dn9 = assign56190_e87106_d_n9;
        locals.var_dnm_dn10 = assign56190_e87106_d_n10;
        locals.var_dnm_dn11 = assign56190_e87106_d_n11;
        locals.var_dnm_dn14 = assign56190_e87106_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign56200_e87123, assign56200_e87123_d_n0, assign56200_e87123_d_n2, assign56200_e87123_d_n4, assign56200_e87123_d_n5, assign56200_e87123_d_n6, assign56200_e87123_d_n7, assign56200_e87123_d_n8, assign56200_e87123_d_n9, assign56200_e87123_d_n10, assign56200_e87123_d_n11, assign56200_e87123_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1404 != 0.0)) {
        let assign56200_e87121: f64 = (1.0 / locals.var_dnm);
        (assign56200_e87121, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign56200_e87123;
        locals.var_dnm_dn0 = assign56200_e87123_d_n0;
        locals.var_dnm_dn2 = assign56200_e87123_d_n2;
        locals.var_dnm_dn4 = assign56200_e87123_d_n4;
        locals.var_dnm_dn5 = assign56200_e87123_d_n5;
        locals.var_dnm_dn6 = assign56200_e87123_d_n6;
        locals.var_dnm_dn7 = assign56200_e87123_d_n7;
        locals.var_dnm_dn8 = assign56200_e87123_d_n8;
        locals.var_dnm_dn9 = assign56200_e87123_d_n9;
        locals.var_dnm_dn10 = assign56200_e87123_d_n10;
        locals.var_dnm_dn11 = assign56200_e87123_d_n11;
        locals.var_dnm_dn14 = assign56200_e87123_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign56210_e87142, assign56210_e87142_d_n0, assign56210_e87142_d_n2, assign56210_e87142_d_n4, assign56210_e87142_d_n5, assign56210_e87142_d_n6, assign56210_e87142_d_n7, assign56210_e87142_d_n8, assign56210_e87142_d_n9, assign56210_e87142_d_n10, assign56210_e87142_d_n11, assign56210_e87142_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1404 != 0.0)) {
        let assign56210_e87138: f64 = (locals.var_tmf1 * p.p405);
        let assign56210_e87140: f64 = (assign56210_e87138 * locals.var_dnm);
        (assign56210_e87140, (((locals.var_tmf1_dn0 * p.p405) * locals.var_dnm) + (assign56210_e87138 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * p.p405) * locals.var_dnm) + (assign56210_e87138 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * p.p405) * locals.var_dnm) + (assign56210_e87138 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * p.p405) * locals.var_dnm) + (assign56210_e87138 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * p.p405) * locals.var_dnm) + (assign56210_e87138 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * p.p405) * locals.var_dnm) + (assign56210_e87138 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * p.p405) * locals.var_dnm) + (assign56210_e87138 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * p.p405) * locals.var_dnm) + (assign56210_e87138 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * p.p405) * locals.var_dnm) + (assign56210_e87138 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * p.p405) * locals.var_dnm) + (assign56210_e87138 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * p.p405) * locals.var_dnm) + (assign56210_e87138 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign56210_e87142;
        locals.var_tmf0_dn0 = assign56210_e87142_d_n0;
        locals.var_tmf0_dn2 = assign56210_e87142_d_n2;
        locals.var_tmf0_dn4 = assign56210_e87142_d_n4;
        locals.var_tmf0_dn5 = assign56210_e87142_d_n5;
        locals.var_tmf0_dn6 = assign56210_e87142_d_n6;
        locals.var_tmf0_dn7 = assign56210_e87142_d_n7;
        locals.var_tmf0_dn8 = assign56210_e87142_d_n8;
        locals.var_tmf0_dn9 = assign56210_e87142_d_n9;
        locals.var_tmf0_dn10 = assign56210_e87142_d_n10;
        locals.var_tmf0_dn11 = assign56210_e87142_d_n11;
        locals.var_tmf0_dn14 = assign56210_e87142_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign56220_e87163, assign56220_e87163_d_n0, assign56220_e87163_d_n2, assign56220_e87163_d_n4, assign56220_e87163_d_n5, assign56220_e87163_d_n6, assign56220_e87163_d_n7, assign56220_e87163_d_n8, assign56220_e87163_d_n9, assign56220_e87163_d_n10, assign56220_e87163_d_n11, assign56220_e87163_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1404 != 0.0)) {
        let assign56220_e87157: f64 = (p.p405 * locals.var_xmp);
        let assign56220_e87159: f64 = (assign56220_e87157 * locals.var_dnm);
        let assign56220_e87161: f64 = (assign56220_e87159 / locals.var_arg);
        (assign56220_e87161, ((((((p.p405 * locals.var_xmp_dn0) * locals.var_dnm) + (assign56220_e87157 * locals.var_dnm_dn0)) * locals.var_arg) - (assign56220_e87159 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((p.p405 * locals.var_xmp_dn2) * locals.var_dnm) + (assign56220_e87157 * locals.var_dnm_dn2)) * locals.var_arg) - (assign56220_e87159 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((p.p405 * locals.var_xmp_dn4) * locals.var_dnm) + (assign56220_e87157 * locals.var_dnm_dn4)) * locals.var_arg) - (assign56220_e87159 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((p.p405 * locals.var_xmp_dn5) * locals.var_dnm) + (assign56220_e87157 * locals.var_dnm_dn5)) * locals.var_arg) - (assign56220_e87159 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((p.p405 * locals.var_xmp_dn6) * locals.var_dnm) + (assign56220_e87157 * locals.var_dnm_dn6)) * locals.var_arg) - (assign56220_e87159 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((p.p405 * locals.var_xmp_dn7) * locals.var_dnm) + (assign56220_e87157 * locals.var_dnm_dn7)) * locals.var_arg) - (assign56220_e87159 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((p.p405 * locals.var_xmp_dn8) * locals.var_dnm) + (assign56220_e87157 * locals.var_dnm_dn8)) * locals.var_arg) - (assign56220_e87159 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((p.p405 * locals.var_xmp_dn9) * locals.var_dnm) + (assign56220_e87157 * locals.var_dnm_dn9)) * locals.var_arg) - (assign56220_e87159 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((p.p405 * locals.var_xmp_dn10) * locals.var_dnm) + (assign56220_e87157 * locals.var_dnm_dn10)) * locals.var_arg) - (assign56220_e87159 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((p.p405 * locals.var_xmp_dn11) * locals.var_dnm) + (assign56220_e87157 * locals.var_dnm_dn11)) * locals.var_arg) - (assign56220_e87159 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((p.p405 * locals.var_xmp_dn14) * locals.var_dnm) + (assign56220_e87157 * locals.var_dnm_dn14)) * locals.var_arg) - (assign56220_e87159 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign56220_e87163;
        locals.var_t0_dn0 = assign56220_e87163_d_n0;
        locals.var_t0_dn2 = assign56220_e87163_d_n2;
        locals.var_t0_dn4 = assign56220_e87163_d_n4;
        locals.var_t0_dn5 = assign56220_e87163_d_n5;
        locals.var_t0_dn6 = assign56220_e87163_d_n6;
        locals.var_t0_dn7 = assign56220_e87163_d_n7;
        locals.var_t0_dn8 = assign56220_e87163_d_n8;
        locals.var_t0_dn9 = assign56220_e87163_d_n9;
        locals.var_t0_dn10 = assign56220_e87163_d_n10;
        locals.var_t0_dn11 = assign56220_e87163_d_n11;
        locals.var_t0_dn14 = assign56220_e87163_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign56230_e87182, assign56230_e87182_d_n0, assign56230_e87182_d_n2, assign56230_e87182_d_n4, assign56230_e87182_d_n5, assign56230_e87182_d_n6, assign56230_e87182_d_n7, assign56230_e87182_d_n8, assign56230_e87182_d_n9, assign56230_e87182_d_n10, assign56230_e87182_d_n11, assign56230_e87182_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1404 != 0.0)) {
        let assign56230_e87178: f64 = (locals.var_uc_depleak + p.p405);
        let assign56230_e87180: f64 = (assign56230_e87178 - locals.var_tmf0);
        (assign56230_e87180, (locals.var_uc_depleak_dn0 - locals.var_tmf0_dn0), (locals.var_uc_depleak_dn2 - locals.var_tmf0_dn2), (locals.var_uc_depleak_dn4 - locals.var_tmf0_dn4), (locals.var_uc_depleak_dn5 - locals.var_tmf0_dn5), (locals.var_uc_depleak_dn6 - locals.var_tmf0_dn6), (locals.var_uc_depleak_dn7 - locals.var_tmf0_dn7), (locals.var_uc_depleak_dn8 - locals.var_tmf0_dn8), (locals.var_uc_depleak_dn9 - locals.var_tmf0_dn9), (locals.var_uc_depleak_dn10 - locals.var_tmf0_dn10), (locals.var_uc_depleak_dn11 - locals.var_tmf0_dn11), (locals.var_uc_depleak_dn14 - locals.var_tmf0_dn14),)
    } else {
        (locals.var_vdssat_res, locals.var_vdssat_res_dn0, locals.var_vdssat_res_dn2, locals.var_vdssat_res_dn4, locals.var_vdssat_res_dn5, locals.var_vdssat_res_dn6, locals.var_vdssat_res_dn7, locals.var_vdssat_res_dn8, locals.var_vdssat_res_dn9, locals.var_vdssat_res_dn10, locals.var_vdssat_res_dn11, locals.var_vdssat_res_dn14,)
    }
};
        locals.var_vdssat_res = assign56230_e87182;
        locals.var_vdssat_res_dn0 = assign56230_e87182_d_n0;
        locals.var_vdssat_res_dn2 = assign56230_e87182_d_n2;
        locals.var_vdssat_res_dn4 = assign56230_e87182_d_n4;
        locals.var_vdssat_res_dn5 = assign56230_e87182_d_n5;
        locals.var_vdssat_res_dn6 = assign56230_e87182_d_n6;
        locals.var_vdssat_res_dn7 = assign56230_e87182_d_n7;
        locals.var_vdssat_res_dn8 = assign56230_e87182_d_n8;
        locals.var_vdssat_res_dn9 = assign56230_e87182_d_n9;
        locals.var_vdssat_res_dn10 = assign56230_e87182_d_n10;
        locals.var_vdssat_res_dn11 = assign56230_e87182_d_n11;
        locals.var_vdssat_res_dn14 = assign56230_e87182_d_n14;
        locals.var_vdssat_res_rv = 0.0;

        let (assign56240_e87197, assign56240_e87197_d_n0, assign56240_e87197_d_n2, assign56240_e87197_d_n4, assign56240_e87197_d_n5, assign56240_e87197_d_n6, assign56240_e87197_d_n7, assign56240_e87197_d_n8, assign56240_e87197_d_n9, assign56240_e87197_d_n10, assign56240_e87197_d_n11, assign56240_e87197_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1404 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign56240_e87197;
        locals.var_t0_dn0 = assign56240_e87197_d_n0;
        locals.var_t0_dn2 = assign56240_e87197_d_n2;
        locals.var_t0_dn4 = assign56240_e87197_d_n4;
        locals.var_t0_dn5 = assign56240_e87197_d_n5;
        locals.var_t0_dn6 = assign56240_e87197_d_n6;
        locals.var_t0_dn7 = assign56240_e87197_d_n7;
        locals.var_t0_dn8 = assign56240_e87197_d_n8;
        locals.var_t0_dn9 = assign56240_e87197_d_n9;
        locals.var_t0_dn10 = assign56240_e87197_d_n10;
        locals.var_t0_dn11 = assign56240_e87197_d_n11;
        locals.var_t0_dn14 = assign56240_e87197_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign56250_e87213, assign56250_e87213_d_n0, assign56250_e87213_d_n2, assign56250_e87213_d_n4, assign56250_e87213_d_n5, assign56250_e87213_d_n6, assign56250_e87213_d_n7, assign56250_e87213_d_n8, assign56250_e87213_d_n9, assign56250_e87213_d_n10, assign56250_e87213_d_n11, assign56250_e87213_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1404 == 0.0)) {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    } else {
        (locals.var_vdssat_res, locals.var_vdssat_res_dn0, locals.var_vdssat_res_dn2, locals.var_vdssat_res_dn4, locals.var_vdssat_res_dn5, locals.var_vdssat_res_dn6, locals.var_vdssat_res_dn7, locals.var_vdssat_res_dn8, locals.var_vdssat_res_dn9, locals.var_vdssat_res_dn10, locals.var_vdssat_res_dn11, locals.var_vdssat_res_dn14,)
    }
};
        locals.var_vdssat_res = assign56250_e87213;
        locals.var_vdssat_res_dn0 = assign56250_e87213_d_n0;
        locals.var_vdssat_res_dn2 = assign56250_e87213_d_n2;
        locals.var_vdssat_res_dn4 = assign56250_e87213_d_n4;
        locals.var_vdssat_res_dn5 = assign56250_e87213_d_n5;
        locals.var_vdssat_res_dn6 = assign56250_e87213_d_n6;
        locals.var_vdssat_res_dn7 = assign56250_e87213_d_n7;
        locals.var_vdssat_res_dn8 = assign56250_e87213_d_n8;
        locals.var_vdssat_res_dn9 = assign56250_e87213_d_n9;
        locals.var_vdssat_res_dn10 = assign56250_e87213_d_n10;
        locals.var_vdssat_res_dn11 = assign56250_e87213_d_n11;
        locals.var_vdssat_res_dn14 = assign56250_e87213_d_n14;
        locals.var_vdssat_res_rv = 0.0;

        let (assign56260_e87229, assign56260_e87229_d_n0, assign56260_e87229_d_n2, assign56260_e87229_d_n4, assign56260_e87229_d_n5, assign56260_e87229_d_n6, assign56260_e87229_d_n7, assign56260_e87229_d_n8, assign56260_e87229_d_n9, assign56260_e87229_d_n10, assign56260_e87229_d_n11, assign56260_e87229_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1404 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign56260_e87229;
        locals.var_t0_dn0 = assign56260_e87229_d_n0;
        locals.var_t0_dn2 = assign56260_e87229_d_n2;
        locals.var_t0_dn4 = assign56260_e87229_d_n4;
        locals.var_t0_dn5 = assign56260_e87229_d_n5;
        locals.var_t0_dn6 = assign56260_e87229_d_n6;
        locals.var_t0_dn7 = assign56260_e87229_d_n7;
        locals.var_t0_dn8 = assign56260_e87229_d_n8;
        locals.var_t0_dn9 = assign56260_e87229_d_n9;
        locals.var_t0_dn10 = assign56260_e87229_d_n10;
        locals.var_t0_dn11 = assign56260_e87229_d_n11;
        locals.var_t0_dn14 = assign56260_e87229_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign56270_e87243, assign56270_e87243_d_n0, assign56270_e87243_d_n2, assign56270_e87243_d_n4, assign56270_e87243_d_n5, assign56270_e87243_d_n6, assign56270_e87243_d_n7, assign56270_e87243_d_n8, assign56270_e87243_d_n9, assign56270_e87243_d_n10, assign56270_e87243_d_n11, assign56270_e87243_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 == 0.0)) {
        (locals.var_vgp_res__blk1147, locals.var_vgp_res__blk1147_dn0, locals.var_vgp_res__blk1147_dn2, locals.var_vgp_res__blk1147_dn4, locals.var_vgp_res__blk1147_dn5, locals.var_vgp_res__blk1147_dn6, locals.var_vgp_res__blk1147_dn7, locals.var_vgp_res__blk1147_dn8, locals.var_vgp_res__blk1147_dn9, locals.var_vgp_res__blk1147_dn10, locals.var_vgp_res__blk1147_dn11, locals.var_vgp_res__blk1147_dn14,)
    } else {
        (locals.var_vgpsat, locals.var_vgpsat_dn0, locals.var_vgpsat_dn2, locals.var_vgpsat_dn4, locals.var_vgpsat_dn5, locals.var_vgpsat_dn6, locals.var_vgpsat_dn7, locals.var_vgpsat_dn8, locals.var_vgpsat_dn9, locals.var_vgpsat_dn10, locals.var_vgpsat_dn11, locals.var_vgpsat_dn14,)
    }
};
        locals.var_vgpsat = assign56270_e87243;
        locals.var_vgpsat_dn0 = assign56270_e87243_d_n0;
        locals.var_vgpsat_dn2 = assign56270_e87243_d_n2;
        locals.var_vgpsat_dn4 = assign56270_e87243_d_n4;
        locals.var_vgpsat_dn5 = assign56270_e87243_d_n5;
        locals.var_vgpsat_dn6 = assign56270_e87243_d_n6;
        locals.var_vgpsat_dn7 = assign56270_e87243_d_n7;
        locals.var_vgpsat_dn8 = assign56270_e87243_d_n8;
        locals.var_vgpsat_dn9 = assign56270_e87243_d_n9;
        locals.var_vgpsat_dn10 = assign56270_e87243_d_n10;
        locals.var_vgpsat_dn11 = assign56270_e87243_d_n11;
        locals.var_vgpsat_dn14 = assign56270_e87243_d_n14;
        locals.var_vgpsat_rv = 0.0;

        let (assign56280_e87261, assign56280_e87261_d_n0, assign56280_e87261_d_n2, assign56280_e87261_d_n4, assign56280_e87261_d_n5, assign56280_e87261_d_n6, assign56280_e87261_d_n7, assign56280_e87261_d_n8, assign56280_e87261_d_n9, assign56280_e87261_d_n10, assign56280_e87261_d_n11, assign56280_e87261_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 == 0.0)) {
        let assign56280_e87258: f64 = (locals.var_c2_q_ndepm_esi_cox_inv2__blk1137 * locals.var_vgpsat);
        let assign56280_e87259: f64 = (1.0 + assign56280_e87258);
        (assign56280_e87259, ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1137_dn0 * locals.var_vgpsat) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1137 * locals.var_vgpsat_dn0)), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1137_dn2 * locals.var_vgpsat) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1137 * locals.var_vgpsat_dn2)), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1137_dn4 * locals.var_vgpsat) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1137 * locals.var_vgpsat_dn4)), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1137_dn5 * locals.var_vgpsat) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1137 * locals.var_vgpsat_dn5)), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1137_dn6 * locals.var_vgpsat) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1137 * locals.var_vgpsat_dn6)), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1137_dn7 * locals.var_vgpsat) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1137 * locals.var_vgpsat_dn7)), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1137_dn8 * locals.var_vgpsat) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1137 * locals.var_vgpsat_dn8)), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1137_dn9 * locals.var_vgpsat) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1137 * locals.var_vgpsat_dn9)), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1137_dn10 * locals.var_vgpsat) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1137 * locals.var_vgpsat_dn10)), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1137_dn11 * locals.var_vgpsat) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1137 * locals.var_vgpsat_dn11)), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1137_dn14 * locals.var_vgpsat) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1137 * locals.var_vgpsat_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign56280_e87261;
        locals.var_t4_dn0 = assign56280_e87261_d_n0;
        locals.var_t4_dn2 = assign56280_e87261_d_n2;
        locals.var_t4_dn4 = assign56280_e87261_d_n4;
        locals.var_t4_dn5 = assign56280_e87261_d_n5;
        locals.var_t4_dn6 = assign56280_e87261_d_n6;
        locals.var_t4_dn7 = assign56280_e87261_d_n7;
        locals.var_t4_dn8 = assign56280_e87261_d_n8;
        locals.var_t4_dn9 = assign56280_e87261_d_n9;
        locals.var_t4_dn10 = assign56280_e87261_d_n10;
        locals.var_t4_dn11 = assign56280_e87261_d_n11;
        locals.var_t4_dn14 = assign56280_e87261_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign56290_e87284, assign56290_e87284_d_n0, assign56290_e87284_d_n2, assign56290_e87284_d_n4, assign56290_e87284_d_n5, assign56290_e87284_d_n6, assign56290_e87284_d_n7, assign56290_e87284_d_n8, assign56290_e87284_d_n9, assign56290_e87284_d_n10, assign56290_e87284_d_n11, assign56290_e87284_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 == 0.0)) {
        let (assign56290_e87282, assign56290_e87282_d_n0, assign56290_e87282_d_n2, assign56290_e87282_d_n4, assign56290_e87282_d_n5, assign56290_e87282_d_n6, assign56290_e87282_d_n7, assign56290_e87282_d_n8, assign56290_e87282_d_n9, assign56290_e87282_d_n10, assign56290_e87282_d_n11, assign56290_e87282_d_n14,) = {
            if (locals.var_t4 > 0.0) {
                let assign56290_e87277: f64 = (locals.var_t4).sqrt();
                (assign56290_e87277, (locals.var_t4_dn0 / (2.0 * assign56290_e87277)), (locals.var_t4_dn2 / (2.0 * assign56290_e87277)), (locals.var_t4_dn4 / (2.0 * assign56290_e87277)), (locals.var_t4_dn5 / (2.0 * assign56290_e87277)), (locals.var_t4_dn6 / (2.0 * assign56290_e87277)), (locals.var_t4_dn7 / (2.0 * assign56290_e87277)), (locals.var_t4_dn8 / (2.0 * assign56290_e87277)), (locals.var_t4_dn9 / (2.0 * assign56290_e87277)), (locals.var_t4_dn10 / (2.0 * assign56290_e87277)), (locals.var_t4_dn11 / (2.0 * assign56290_e87277)), (locals.var_t4_dn14 / (2.0 * assign56290_e87277)),)
            } else {
                let assign56290_e87279: f64 = (-locals.var_t4);
                let assign56290_e87280: f64 = (assign56290_e87279).sqrt();
                let assign56290_e87281: f64 = (-assign56290_e87280);
                (assign56290_e87281, (-((-locals.var_t4_dn0) / (2.0 * assign56290_e87280))), (-((-locals.var_t4_dn2) / (2.0 * assign56290_e87280))), (-((-locals.var_t4_dn4) / (2.0 * assign56290_e87280))), (-((-locals.var_t4_dn5) / (2.0 * assign56290_e87280))), (-((-locals.var_t4_dn6) / (2.0 * assign56290_e87280))), (-((-locals.var_t4_dn7) / (2.0 * assign56290_e87280))), (-((-locals.var_t4_dn8) / (2.0 * assign56290_e87280))), (-((-locals.var_t4_dn9) / (2.0 * assign56290_e87280))), (-((-locals.var_t4_dn10) / (2.0 * assign56290_e87280))), (-((-locals.var_t4_dn11) / (2.0 * assign56290_e87280))), (-((-locals.var_t4_dn14) / (2.0 * assign56290_e87280))),)
            }
        };
        (assign56290_e87282, assign56290_e87282_d_n0, assign56290_e87282_d_n2, assign56290_e87282_d_n4, assign56290_e87282_d_n5, assign56290_e87282_d_n6, assign56290_e87282_d_n7, assign56290_e87282_d_n8, assign56290_e87282_d_n9, assign56290_e87282_d_n10, assign56290_e87282_d_n11, assign56290_e87282_d_n14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign56290_e87284;
        locals.var_t3_dn0 = assign56290_e87284_d_n0;
        locals.var_t3_dn2 = assign56290_e87284_d_n2;
        locals.var_t3_dn4 = assign56290_e87284_d_n4;
        locals.var_t3_dn5 = assign56290_e87284_d_n5;
        locals.var_t3_dn6 = assign56290_e87284_d_n6;
        locals.var_t3_dn7 = assign56290_e87284_d_n7;
        locals.var_t3_dn8 = assign56290_e87284_d_n8;
        locals.var_t3_dn9 = assign56290_e87284_d_n9;
        locals.var_t3_dn10 = assign56290_e87284_d_n10;
        locals.var_t3_dn11 = assign56290_e87284_d_n11;
        locals.var_t3_dn14 = assign56290_e87284_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign56300_e87304, assign56300_e87304_d_n0, assign56300_e87304_d_n2, assign56300_e87304_d_n4, assign56300_e87304_d_n5, assign56300_e87304_d_n6, assign56300_e87304_d_n7, assign56300_e87304_d_n8, assign56300_e87304_d_n9, assign56300_e87304_d_n10, assign56300_e87304_d_n11, assign56300_e87304_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 == 0.0)) {
        let assign56300_e87300: f64 = (1.0 - locals.var_t3);
        let assign56300_e87301: f64 = (locals.var_q_ndepm_esi_cox_inv2__blk1136 * assign56300_e87300);
        let assign56300_e87302: f64 = (locals.var_vgpsat + assign56300_e87301);
        (assign56300_e87302, (locals.var_vgpsat_dn0 + ((locals.var_q_ndepm_esi_cox_inv2__blk1136_dn0 * assign56300_e87300) + (locals.var_q_ndepm_esi_cox_inv2__blk1136 * (-locals.var_t3_dn0)))), (locals.var_vgpsat_dn2 + ((locals.var_q_ndepm_esi_cox_inv2__blk1136_dn2 * assign56300_e87300) + (locals.var_q_ndepm_esi_cox_inv2__blk1136 * (-locals.var_t3_dn2)))), (locals.var_vgpsat_dn4 + ((locals.var_q_ndepm_esi_cox_inv2__blk1136_dn4 * assign56300_e87300) + (locals.var_q_ndepm_esi_cox_inv2__blk1136 * (-locals.var_t3_dn4)))), (locals.var_vgpsat_dn5 + ((locals.var_q_ndepm_esi_cox_inv2__blk1136_dn5 * assign56300_e87300) + (locals.var_q_ndepm_esi_cox_inv2__blk1136 * (-locals.var_t3_dn5)))), (locals.var_vgpsat_dn6 + ((locals.var_q_ndepm_esi_cox_inv2__blk1136_dn6 * assign56300_e87300) + (locals.var_q_ndepm_esi_cox_inv2__blk1136 * (-locals.var_t3_dn6)))), (locals.var_vgpsat_dn7 + ((locals.var_q_ndepm_esi_cox_inv2__blk1136_dn7 * assign56300_e87300) + (locals.var_q_ndepm_esi_cox_inv2__blk1136 * (-locals.var_t3_dn7)))), (locals.var_vgpsat_dn8 + ((locals.var_q_ndepm_esi_cox_inv2__blk1136_dn8 * assign56300_e87300) + (locals.var_q_ndepm_esi_cox_inv2__blk1136 * (-locals.var_t3_dn8)))), (locals.var_vgpsat_dn9 + ((locals.var_q_ndepm_esi_cox_inv2__blk1136_dn9 * assign56300_e87300) + (locals.var_q_ndepm_esi_cox_inv2__blk1136 * (-locals.var_t3_dn9)))), (locals.var_vgpsat_dn10 + ((locals.var_q_ndepm_esi_cox_inv2__blk1136_dn10 * assign56300_e87300) + (locals.var_q_ndepm_esi_cox_inv2__blk1136 * (-locals.var_t3_dn10)))), (locals.var_vgpsat_dn11 + ((locals.var_q_ndepm_esi_cox_inv2__blk1136_dn11 * assign56300_e87300) + (locals.var_q_ndepm_esi_cox_inv2__blk1136 * (-locals.var_t3_dn11)))), (locals.var_vgpsat_dn14 + ((locals.var_q_ndepm_esi_cox_inv2__blk1136_dn14 * assign56300_e87300) + (locals.var_q_ndepm_esi_cox_inv2__blk1136 * (-locals.var_t3_dn14)))),)
    } else {
        (locals.var_vdssat_ini, locals.var_vdssat_ini_dn0, locals.var_vdssat_ini_dn2, locals.var_vdssat_ini_dn4, locals.var_vdssat_ini_dn5, locals.var_vdssat_ini_dn6, locals.var_vdssat_ini_dn7, locals.var_vdssat_ini_dn8, locals.var_vdssat_ini_dn9, locals.var_vdssat_ini_dn10, locals.var_vdssat_ini_dn11, locals.var_vdssat_ini_dn14,)
    }
};
        locals.var_vdssat_ini = assign56300_e87304;
        locals.var_vdssat_ini_dn0 = assign56300_e87304_d_n0;
        locals.var_vdssat_ini_dn2 = assign56300_e87304_d_n2;
        locals.var_vdssat_ini_dn4 = assign56300_e87304_d_n4;
        locals.var_vdssat_ini_dn5 = assign56300_e87304_d_n5;
        locals.var_vdssat_ini_dn6 = assign56300_e87304_d_n6;
        locals.var_vdssat_ini_dn7 = assign56300_e87304_d_n7;
        locals.var_vdssat_ini_dn8 = assign56300_e87304_d_n8;
        locals.var_vdssat_ini_dn9 = assign56300_e87304_d_n9;
        locals.var_vdssat_ini_dn10 = assign56300_e87304_d_n10;
        locals.var_vdssat_ini_dn11 = assign56300_e87304_d_n11;
        locals.var_vdssat_ini_dn14 = assign56300_e87304_d_n14;
        locals.var_vdssat_ini_rv = 0.0;

        let (assign56310_e87318, assign56310_e87318_d_n0, assign56310_e87318_d_n2, assign56310_e87318_d_n4, assign56310_e87318_d_n5, assign56310_e87318_d_n6, assign56310_e87318_d_n7, assign56310_e87318_d_n8, assign56310_e87318_d_n9, assign56310_e87318_d_n10, assign56310_e87318_d_n11, assign56310_e87318_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 == 0.0)) {
        (locals.var_vdssat_ini, locals.var_vdssat_ini_dn0, locals.var_vdssat_ini_dn2, locals.var_vdssat_ini_dn4, locals.var_vdssat_ini_dn5, locals.var_vdssat_ini_dn6, locals.var_vdssat_ini_dn7, locals.var_vdssat_ini_dn8, locals.var_vdssat_ini_dn9, locals.var_vdssat_ini_dn10, locals.var_vdssat_ini_dn11, locals.var_vdssat_ini_dn14,)
    } else {
        (locals.var_phi_vsat, locals.var_phi_vsat_dn0, locals.var_phi_vsat_dn2, locals.var_phi_vsat_dn4, locals.var_phi_vsat_dn5, locals.var_phi_vsat_dn6, locals.var_phi_vsat_dn7, locals.var_phi_vsat_dn8, locals.var_phi_vsat_dn9, locals.var_phi_vsat_dn10, locals.var_phi_vsat_dn11, locals.var_phi_vsat_dn14,)
    }
};
        locals.var_phi_vsat = assign56310_e87318;
        locals.var_phi_vsat_dn0 = assign56310_e87318_d_n0;
        locals.var_phi_vsat_dn2 = assign56310_e87318_d_n2;
        locals.var_phi_vsat_dn4 = assign56310_e87318_d_n4;
        locals.var_phi_vsat_dn5 = assign56310_e87318_d_n5;
        locals.var_phi_vsat_dn6 = assign56310_e87318_d_n6;
        locals.var_phi_vsat_dn7 = assign56310_e87318_d_n7;
        locals.var_phi_vsat_dn8 = assign56310_e87318_d_n8;
        locals.var_phi_vsat_dn9 = assign56310_e87318_d_n9;
        locals.var_phi_vsat_dn10 = assign56310_e87318_d_n10;
        locals.var_phi_vsat_dn11 = assign56310_e87318_d_n11;
        locals.var_phi_vsat_dn14 = assign56310_e87318_d_n14;
        locals.var_phi_vsat_rv = 0.0;

        let (assign56320_e87332,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign56320_e87332;
        locals.var_flg_conv_rv = 0.0;

        let (assign56330_e87346,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign56330_e87346;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_205(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign56340_loop_guard: usize = 0;
        while {
            let assign56340_cond_e87361: f64 = if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 == 0.0)) && (locals.var_lp_s0 <= 150.0)) { 1.0 } else { 0.0 };
            assign56340_cond_e87361 != 0.0
        } {
            assign56340_loop_guard += 1;
            assert!(assign56340_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign56340_body0_e87378, assign56340_body0_e87378_d_n0, assign56340_body0_e87378_d_n2, assign56340_body0_e87378_d_n4, assign56340_body0_e87378_d_n5, assign56340_body0_e87378_d_n6, assign56340_body0_e87378_d_n7, assign56340_body0_e87378_d_n8, assign56340_body0_e87378_d_n9, assign56340_body0_e87378_d_n10, assign56340_body0_e87378_d_n11, assign56340_body0_e87378_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 == 0.0)) {
        let assign56340_body0_e87374: f64 = (-locals.var_beta);
        let assign56340_body0_e87376: f64 = (assign56340_body0_e87374 * locals.var_phi_vsat);
        (assign56340_body0_e87376, (((-locals.var_beta_dn0) * locals.var_phi_vsat) + (assign56340_body0_e87374 * locals.var_phi_vsat_dn0)), (((-locals.var_beta_dn2) * locals.var_phi_vsat) + (assign56340_body0_e87374 * locals.var_phi_vsat_dn2)), (((-locals.var_beta_dn4) * locals.var_phi_vsat) + (assign56340_body0_e87374 * locals.var_phi_vsat_dn4)), (((-locals.var_beta_dn5) * locals.var_phi_vsat) + (assign56340_body0_e87374 * locals.var_phi_vsat_dn5)), (((-locals.var_beta_dn6) * locals.var_phi_vsat) + (assign56340_body0_e87374 * locals.var_phi_vsat_dn6)), (((-locals.var_beta_dn7) * locals.var_phi_vsat) + (assign56340_body0_e87374 * locals.var_phi_vsat_dn7)), (((-locals.var_beta_dn8) * locals.var_phi_vsat) + (assign56340_body0_e87374 * locals.var_phi_vsat_dn8)), (((-locals.var_beta_dn9) * locals.var_phi_vsat) + (assign56340_body0_e87374 * locals.var_phi_vsat_dn9)), (((-locals.var_beta_dn10) * locals.var_phi_vsat) + (assign56340_body0_e87374 * locals.var_phi_vsat_dn10)), (((-locals.var_beta_dn11) * locals.var_phi_vsat) + (assign56340_body0_e87374 * locals.var_phi_vsat_dn11)), (((-locals.var_beta_dn14) * locals.var_phi_vsat) + (assign56340_body0_e87374 * locals.var_phi_vsat_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign56340_body0_e87378;
            locals.var_t1_dn0 = assign56340_body0_e87378_d_n0;
            locals.var_t1_dn2 = assign56340_body0_e87378_d_n2;
            locals.var_t1_dn4 = assign56340_body0_e87378_d_n4;
            locals.var_t1_dn5 = assign56340_body0_e87378_d_n5;
            locals.var_t1_dn6 = assign56340_body0_e87378_d_n6;
            locals.var_t1_dn7 = assign56340_body0_e87378_d_n7;
            locals.var_t1_dn8 = assign56340_body0_e87378_d_n8;
            locals.var_t1_dn9 = assign56340_body0_e87378_d_n9;
            locals.var_t1_dn10 = assign56340_body0_e87378_d_n10;
            locals.var_t1_dn11 = assign56340_body0_e87378_d_n11;
            locals.var_t1_dn14 = assign56340_body0_e87378_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign56340_body1_e87393, assign56340_body1_e87393_d_n0, assign56340_body1_e87393_d_n2, assign56340_body1_e87393_d_n4, assign56340_body1_e87393_d_n5, assign56340_body1_e87393_d_n6, assign56340_body1_e87393_d_n7, assign56340_body1_e87393_d_n8, assign56340_body1_e87393_d_n9, assign56340_body1_e87393_d_n10, assign56340_body1_e87393_d_n11, assign56340_body1_e87393_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 == 0.0)) {
        let assign56340_body1_e87391: f64 = (locals.var_t1).exp();
        (assign56340_body1_e87391, (assign56340_body1_e87391 * locals.var_t1_dn0), (assign56340_body1_e87391 * locals.var_t1_dn2), (assign56340_body1_e87391 * locals.var_t1_dn4), (assign56340_body1_e87391 * locals.var_t1_dn5), (assign56340_body1_e87391 * locals.var_t1_dn6), (assign56340_body1_e87391 * locals.var_t1_dn7), (assign56340_body1_e87391 * locals.var_t1_dn8), (assign56340_body1_e87391 * locals.var_t1_dn9), (assign56340_body1_e87391 * locals.var_t1_dn10), (assign56340_body1_e87391 * locals.var_t1_dn11), (assign56340_body1_e87391 * locals.var_t1_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
            locals.var_t2 = assign56340_body1_e87393;
            locals.var_t2_dn0 = assign56340_body1_e87393_d_n0;
            locals.var_t2_dn2 = assign56340_body1_e87393_d_n2;
            locals.var_t2_dn4 = assign56340_body1_e87393_d_n4;
            locals.var_t2_dn5 = assign56340_body1_e87393_d_n5;
            locals.var_t2_dn6 = assign56340_body1_e87393_d_n6;
            locals.var_t2_dn7 = assign56340_body1_e87393_d_n7;
            locals.var_t2_dn8 = assign56340_body1_e87393_d_n8;
            locals.var_t2_dn9 = assign56340_body1_e87393_d_n9;
            locals.var_t2_dn10 = assign56340_body1_e87393_d_n10;
            locals.var_t2_dn11 = assign56340_body1_e87393_d_n11;
            locals.var_t2_dn14 = assign56340_body1_e87393_d_n14;
            locals.var_t2_rv = 0.0;
            let (assign56340_body2_e87412, assign56340_body2_e87412_d_n0, assign56340_body2_e87412_d_n2, assign56340_body2_e87412_d_n4, assign56340_body2_e87412_d_n5, assign56340_body2_e87412_d_n6, assign56340_body2_e87412_d_n7, assign56340_body2_e87412_d_n8, assign56340_body2_e87412_d_n9, assign56340_body2_e87412_d_n10, assign56340_body2_e87412_d_n11, assign56340_body2_e87412_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 == 0.0)) {
        let assign56340_body2_e87407: f64 = (2.0 * locals.var_q_ndepm_esi__blk1116);
        let assign56340_body2_e87409: f64 = (assign56340_body2_e87407 / locals.var_beta);
        let assign56340_body2_e87410: f64 = (assign56340_body2_e87409).sqrt();
        (assign56340_body2_e87410, (((((2.0 * locals.var_q_ndepm_esi__blk1116_dn0) * locals.var_beta) - (assign56340_body2_e87407 * locals.var_beta_dn0)) / (locals.var_beta * locals.var_beta)) / (2.0 * assign56340_body2_e87410)), (((((2.0 * locals.var_q_ndepm_esi__blk1116_dn2) * locals.var_beta) - (assign56340_body2_e87407 * locals.var_beta_dn2)) / (locals.var_beta * locals.var_beta)) / (2.0 * assign56340_body2_e87410)), (((((2.0 * locals.var_q_ndepm_esi__blk1116_dn4) * locals.var_beta) - (assign56340_body2_e87407 * locals.var_beta_dn4)) / (locals.var_beta * locals.var_beta)) / (2.0 * assign56340_body2_e87410)), (((((2.0 * locals.var_q_ndepm_esi__blk1116_dn5) * locals.var_beta) - (assign56340_body2_e87407 * locals.var_beta_dn5)) / (locals.var_beta * locals.var_beta)) / (2.0 * assign56340_body2_e87410)), (((((2.0 * locals.var_q_ndepm_esi__blk1116_dn6) * locals.var_beta) - (assign56340_body2_e87407 * locals.var_beta_dn6)) / (locals.var_beta * locals.var_beta)) / (2.0 * assign56340_body2_e87410)), (((((2.0 * locals.var_q_ndepm_esi__blk1116_dn7) * locals.var_beta) - (assign56340_body2_e87407 * locals.var_beta_dn7)) / (locals.var_beta * locals.var_beta)) / (2.0 * assign56340_body2_e87410)), (((((2.0 * locals.var_q_ndepm_esi__blk1116_dn8) * locals.var_beta) - (assign56340_body2_e87407 * locals.var_beta_dn8)) / (locals.var_beta * locals.var_beta)) / (2.0 * assign56340_body2_e87410)), (((((2.0 * locals.var_q_ndepm_esi__blk1116_dn9) * locals.var_beta) - (assign56340_body2_e87407 * locals.var_beta_dn9)) / (locals.var_beta * locals.var_beta)) / (2.0 * assign56340_body2_e87410)), (((((2.0 * locals.var_q_ndepm_esi__blk1116_dn10) * locals.var_beta) - (assign56340_body2_e87407 * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta)) / (2.0 * assign56340_body2_e87410)), (((((2.0 * locals.var_q_ndepm_esi__blk1116_dn11) * locals.var_beta) - (assign56340_body2_e87407 * locals.var_beta_dn11)) / (locals.var_beta * locals.var_beta)) / (2.0 * assign56340_body2_e87410)), (((((2.0 * locals.var_q_ndepm_esi__blk1116_dn14) * locals.var_beta) - (assign56340_body2_e87407 * locals.var_beta_dn14)) / (locals.var_beta * locals.var_beta)) / (2.0 * assign56340_body2_e87410)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
            locals.var_t4 = assign56340_body2_e87412;
            locals.var_t4_dn0 = assign56340_body2_e87412_d_n0;
            locals.var_t4_dn2 = assign56340_body2_e87412_d_n2;
            locals.var_t4_dn4 = assign56340_body2_e87412_d_n4;
            locals.var_t4_dn5 = assign56340_body2_e87412_d_n5;
            locals.var_t4_dn6 = assign56340_body2_e87412_d_n6;
            locals.var_t4_dn7 = assign56340_body2_e87412_d_n7;
            locals.var_t4_dn8 = assign56340_body2_e87412_d_n8;
            locals.var_t4_dn9 = assign56340_body2_e87412_d_n9;
            locals.var_t4_dn10 = assign56340_body2_e87412_d_n10;
            locals.var_t4_dn11 = assign56340_body2_e87412_d_n11;
            locals.var_t4_dn14 = assign56340_body2_e87412_d_n14;
            locals.var_t4_rv = 0.0;
            let (assign56340_body3_e87430, assign56340_body3_e87430_d_n0, assign56340_body3_e87430_d_n2, assign56340_body3_e87430_d_n4, assign56340_body3_e87430_d_n5, assign56340_body3_e87430_d_n6, assign56340_body3_e87430_d_n7, assign56340_body3_e87430_d_n8, assign56340_body3_e87430_d_n9, assign56340_body3_e87430_d_n10, assign56340_body3_e87430_d_n11, assign56340_body3_e87430_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 == 0.0)) {
        let assign56340_body3_e87426: f64 = (locals.var_t2 - locals.var_t1);
        let assign56340_body3_e87428: f64 = (assign56340_body3_e87426 - 1.0);
        (assign56340_body3_e87428, (locals.var_t2_dn0 - locals.var_t1_dn0), (locals.var_t2_dn2 - locals.var_t1_dn2), (locals.var_t2_dn4 - locals.var_t1_dn4), (locals.var_t2_dn5 - locals.var_t1_dn5), (locals.var_t2_dn6 - locals.var_t1_dn6), (locals.var_t2_dn7 - locals.var_t1_dn7), (locals.var_t2_dn8 - locals.var_t1_dn8), (locals.var_t2_dn9 - locals.var_t1_dn9), (locals.var_t2_dn10 - locals.var_t1_dn10), (locals.var_t2_dn11 - locals.var_t1_dn11), (locals.var_t2_dn14 - locals.var_t1_dn14),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
            locals.var_t10 = assign56340_body3_e87430;
            locals.var_t10_dn0 = assign56340_body3_e87430_d_n0;
            locals.var_t10_dn2 = assign56340_body3_e87430_d_n2;
            locals.var_t10_dn4 = assign56340_body3_e87430_d_n4;
            locals.var_t10_dn5 = assign56340_body3_e87430_d_n5;
            locals.var_t10_dn6 = assign56340_body3_e87430_d_n6;
            locals.var_t10_dn7 = assign56340_body3_e87430_d_n7;
            locals.var_t10_dn8 = assign56340_body3_e87430_d_n8;
            locals.var_t10_dn9 = assign56340_body3_e87430_d_n9;
            locals.var_t10_dn10 = assign56340_body3_e87430_d_n10;
            locals.var_t10_dn11 = assign56340_body3_e87430_d_n11;
            locals.var_t10_dn14 = assign56340_body3_e87430_d_n14;
            locals.var_t10_rv = 0.0;
            let (assign56340_body4_e87449, assign56340_body4_e87449_d_n0, assign56340_body4_e87449_d_n2, assign56340_body4_e87449_d_n4, assign56340_body4_e87449_d_n5, assign56340_body4_e87449_d_n6, assign56340_body4_e87449_d_n7, assign56340_body4_e87449_d_n8, assign56340_body4_e87449_d_n9, assign56340_body4_e87449_d_n10, assign56340_body4_e87449_d_n11, assign56340_body4_e87449_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 == 0.0)) {
        let assign56340_body4_e87445: f64 = (locals.var_t10 + 1e-15);
        let assign56340_body4_e87446: f64 = (assign56340_body4_e87445).sqrt();
        let assign56340_body4_e87447: f64 = (locals.var_t4 * assign56340_body4_e87446);
        (assign56340_body4_e87447, ((locals.var_t4_dn0 * assign56340_body4_e87446) + (locals.var_t4 * (locals.var_t10_dn0 / (2.0 * assign56340_body4_e87446)))), ((locals.var_t4_dn2 * assign56340_body4_e87446) + (locals.var_t4 * (locals.var_t10_dn2 / (2.0 * assign56340_body4_e87446)))), ((locals.var_t4_dn4 * assign56340_body4_e87446) + (locals.var_t4 * (locals.var_t10_dn4 / (2.0 * assign56340_body4_e87446)))), ((locals.var_t4_dn5 * assign56340_body4_e87446) + (locals.var_t4 * (locals.var_t10_dn5 / (2.0 * assign56340_body4_e87446)))), ((locals.var_t4_dn6 * assign56340_body4_e87446) + (locals.var_t4 * (locals.var_t10_dn6 / (2.0 * assign56340_body4_e87446)))), ((locals.var_t4_dn7 * assign56340_body4_e87446) + (locals.var_t4 * (locals.var_t10_dn7 / (2.0 * assign56340_body4_e87446)))), ((locals.var_t4_dn8 * assign56340_body4_e87446) + (locals.var_t4 * (locals.var_t10_dn8 / (2.0 * assign56340_body4_e87446)))), ((locals.var_t4_dn9 * assign56340_body4_e87446) + (locals.var_t4 * (locals.var_t10_dn9 / (2.0 * assign56340_body4_e87446)))), ((locals.var_t4_dn10 * assign56340_body4_e87446) + (locals.var_t4 * (locals.var_t10_dn10 / (2.0 * assign56340_body4_e87446)))), ((locals.var_t4_dn11 * assign56340_body4_e87446) + (locals.var_t4 * (locals.var_t10_dn11 / (2.0 * assign56340_body4_e87446)))), ((locals.var_t4_dn14 * assign56340_body4_e87446) + (locals.var_t4 * (locals.var_t10_dn14 / (2.0 * assign56340_body4_e87446)))),)
    } else {
        (locals.var_q_sat, locals.var_q_sat_dn0, locals.var_q_sat_dn2, locals.var_q_sat_dn4, locals.var_q_sat_dn5, locals.var_q_sat_dn6, locals.var_q_sat_dn7, locals.var_q_sat_dn8, locals.var_q_sat_dn9, locals.var_q_sat_dn10, locals.var_q_sat_dn11, locals.var_q_sat_dn14,)
    }
};
            locals.var_q_sat = assign56340_body4_e87449;
            locals.var_q_sat_dn0 = assign56340_body4_e87449_d_n0;
            locals.var_q_sat_dn2 = assign56340_body4_e87449_d_n2;
            locals.var_q_sat_dn4 = assign56340_body4_e87449_d_n4;
            locals.var_q_sat_dn5 = assign56340_body4_e87449_d_n5;
            locals.var_q_sat_dn6 = assign56340_body4_e87449_d_n6;
            locals.var_q_sat_dn7 = assign56340_body4_e87449_d_n7;
            locals.var_q_sat_dn8 = assign56340_body4_e87449_d_n8;
            locals.var_q_sat_dn9 = assign56340_body4_e87449_d_n9;
            locals.var_q_sat_dn10 = assign56340_body4_e87449_d_n10;
            locals.var_q_sat_dn11 = assign56340_body4_e87449_d_n11;
            locals.var_q_sat_dn14 = assign56340_body4_e87449_d_n14;
            locals.var_q_sat_rv = 0.0;
            let assign56340_body5_e87452: f64 = if locals.var_t1 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1410 = assign56340_body5_e87452;
            locals.var_guard1410_rv = 0.0;
            let (assign56340_body6_e87469, assign56340_body6_e87469_d_n0, assign56340_body6_e87469_d_n2, assign56340_body6_e87469_d_n4, assign56340_body6_e87469_d_n5, assign56340_body6_e87469_d_n6, assign56340_body6_e87469_d_n7, assign56340_body6_e87469_d_n8, assign56340_body6_e87469_d_n9, assign56340_body6_e87469_d_n10, assign56340_body6_e87469_d_n11, assign56340_body6_e87469_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 == 0.0)) && (locals.var_guard1410 != 0.0)) {
        let assign56340_body6_e87467: f64 = (-locals.var_q_sat);
        (assign56340_body6_e87467, (-locals.var_q_sat_dn0), (-locals.var_q_sat_dn2), (-locals.var_q_sat_dn4), (-locals.var_q_sat_dn5), (-locals.var_q_sat_dn6), (-locals.var_q_sat_dn7), (-locals.var_q_sat_dn8), (-locals.var_q_sat_dn9), (-locals.var_q_sat_dn10), (-locals.var_q_sat_dn11), (-locals.var_q_sat_dn14),)
    } else {
        (locals.var_q_sat, locals.var_q_sat_dn0, locals.var_q_sat_dn2, locals.var_q_sat_dn4, locals.var_q_sat_dn5, locals.var_q_sat_dn6, locals.var_q_sat_dn7, locals.var_q_sat_dn8, locals.var_q_sat_dn9, locals.var_q_sat_dn10, locals.var_q_sat_dn11, locals.var_q_sat_dn14,)
    }
};
            locals.var_q_sat = assign56340_body6_e87469;
            locals.var_q_sat_dn0 = assign56340_body6_e87469_d_n0;
            locals.var_q_sat_dn2 = assign56340_body6_e87469_d_n2;
            locals.var_q_sat_dn4 = assign56340_body6_e87469_d_n4;
            locals.var_q_sat_dn5 = assign56340_body6_e87469_d_n5;
            locals.var_q_sat_dn6 = assign56340_body6_e87469_d_n6;
            locals.var_q_sat_dn7 = assign56340_body6_e87469_d_n7;
            locals.var_q_sat_dn8 = assign56340_body6_e87469_d_n8;
            locals.var_q_sat_dn9 = assign56340_body6_e87469_d_n9;
            locals.var_q_sat_dn10 = assign56340_body6_e87469_d_n10;
            locals.var_q_sat_dn11 = assign56340_body6_e87469_d_n11;
            locals.var_q_sat_dn14 = assign56340_body6_e87469_d_n14;
            locals.var_q_sat_rv = 0.0;
            let (assign56340_body7_e87491, assign56340_body7_e87491_d_n0, assign56340_body7_e87491_d_n2, assign56340_body7_e87491_d_n4, assign56340_body7_e87491_d_n5, assign56340_body7_e87491_d_n6, assign56340_body7_e87491_d_n7, assign56340_body7_e87491_d_n8, assign56340_body7_e87491_d_n9, assign56340_body7_e87491_d_n10, assign56340_body7_e87491_d_n11, assign56340_body7_e87491_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 == 0.0)) {
        let assign56340_body7_e87483: f64 = (0.5 * locals.var_t4);
        let assign56340_body7_e87485: f64 = (assign56340_body7_e87483 * locals.var_t4);
        let assign56340_body7_e87487: f64 = (assign56340_body7_e87485 * locals.var_beta);
        let assign56340_body7_e87489: f64 = (assign56340_body7_e87487 / locals.var_q_sat);
        (assign56340_body7_e87489, ((((((((0.5 * locals.var_t4_dn0) * locals.var_t4) + (assign56340_body7_e87483 * locals.var_t4_dn0)) * locals.var_beta) + (assign56340_body7_e87485 * locals.var_beta_dn0)) * locals.var_q_sat) - (assign56340_body7_e87487 * locals.var_q_sat_dn0)) / (locals.var_q_sat * locals.var_q_sat)), ((((((((0.5 * locals.var_t4_dn2) * locals.var_t4) + (assign56340_body7_e87483 * locals.var_t4_dn2)) * locals.var_beta) + (assign56340_body7_e87485 * locals.var_beta_dn2)) * locals.var_q_sat) - (assign56340_body7_e87487 * locals.var_q_sat_dn2)) / (locals.var_q_sat * locals.var_q_sat)), ((((((((0.5 * locals.var_t4_dn4) * locals.var_t4) + (assign56340_body7_e87483 * locals.var_t4_dn4)) * locals.var_beta) + (assign56340_body7_e87485 * locals.var_beta_dn4)) * locals.var_q_sat) - (assign56340_body7_e87487 * locals.var_q_sat_dn4)) / (locals.var_q_sat * locals.var_q_sat)), ((((((((0.5 * locals.var_t4_dn5) * locals.var_t4) + (assign56340_body7_e87483 * locals.var_t4_dn5)) * locals.var_beta) + (assign56340_body7_e87485 * locals.var_beta_dn5)) * locals.var_q_sat) - (assign56340_body7_e87487 * locals.var_q_sat_dn5)) / (locals.var_q_sat * locals.var_q_sat)), ((((((((0.5 * locals.var_t4_dn6) * locals.var_t4) + (assign56340_body7_e87483 * locals.var_t4_dn6)) * locals.var_beta) + (assign56340_body7_e87485 * locals.var_beta_dn6)) * locals.var_q_sat) - (assign56340_body7_e87487 * locals.var_q_sat_dn6)) / (locals.var_q_sat * locals.var_q_sat)), ((((((((0.5 * locals.var_t4_dn7) * locals.var_t4) + (assign56340_body7_e87483 * locals.var_t4_dn7)) * locals.var_beta) + (assign56340_body7_e87485 * locals.var_beta_dn7)) * locals.var_q_sat) - (assign56340_body7_e87487 * locals.var_q_sat_dn7)) / (locals.var_q_sat * locals.var_q_sat)), ((((((((0.5 * locals.var_t4_dn8) * locals.var_t4) + (assign56340_body7_e87483 * locals.var_t4_dn8)) * locals.var_beta) + (assign56340_body7_e87485 * locals.var_beta_dn8)) * locals.var_q_sat) - (assign56340_body7_e87487 * locals.var_q_sat_dn8)) / (locals.var_q_sat * locals.var_q_sat)), ((((((((0.5 * locals.var_t4_dn9) * locals.var_t4) + (assign56340_body7_e87483 * locals.var_t4_dn9)) * locals.var_beta) + (assign56340_body7_e87485 * locals.var_beta_dn9)) * locals.var_q_sat) - (assign56340_body7_e87487 * locals.var_q_sat_dn9)) / (locals.var_q_sat * locals.var_q_sat)), ((((((((0.5 * locals.var_t4_dn10) * locals.var_t4) + (assign56340_body7_e87483 * locals.var_t4_dn10)) * locals.var_beta) + (assign56340_body7_e87485 * locals.var_beta_dn10)) * locals.var_q_sat) - (assign56340_body7_e87487 * locals.var_q_sat_dn10)) / (locals.var_q_sat * locals.var_q_sat)), ((((((((0.5 * locals.var_t4_dn11) * locals.var_t4) + (assign56340_body7_e87483 * locals.var_t4_dn11)) * locals.var_beta) + (assign56340_body7_e87485 * locals.var_beta_dn11)) * locals.var_q_sat) - (assign56340_body7_e87487 * locals.var_q_sat_dn11)) / (locals.var_q_sat * locals.var_q_sat)), ((((((((0.5 * locals.var_t4_dn14) * locals.var_t4) + (assign56340_body7_e87483 * locals.var_t4_dn14)) * locals.var_beta) + (assign56340_body7_e87485 * locals.var_beta_dn14)) * locals.var_q_sat) - (assign56340_body7_e87487 * locals.var_q_sat_dn14)) / (locals.var_q_sat * locals.var_q_sat)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
            locals.var_t11 = assign56340_body7_e87491;
            locals.var_t11_dn0 = assign56340_body7_e87491_d_n0;
            locals.var_t11_dn2 = assign56340_body7_e87491_d_n2;
            locals.var_t11_dn4 = assign56340_body7_e87491_d_n4;
            locals.var_t11_dn5 = assign56340_body7_e87491_d_n5;
            locals.var_t11_dn6 = assign56340_body7_e87491_d_n6;
            locals.var_t11_dn7 = assign56340_body7_e87491_d_n7;
            locals.var_t11_dn8 = assign56340_body7_e87491_d_n8;
            locals.var_t11_dn9 = assign56340_body7_e87491_d_n9;
            locals.var_t11_dn10 = assign56340_body7_e87491_d_n10;
            locals.var_t11_dn11 = assign56340_body7_e87491_d_n11;
            locals.var_t11_dn14 = assign56340_body7_e87491_d_n14;
            locals.var_t11_rv = 0.0;
            let (assign56340_body8_e87510, assign56340_body8_e87510_d_n0, assign56340_body8_e87510_d_n2, assign56340_body8_e87510_d_n4, assign56340_body8_e87510_d_n5, assign56340_body8_e87510_d_n6, assign56340_body8_e87510_d_n7, assign56340_body8_e87510_d_n8, assign56340_body8_e87510_d_n9, assign56340_body8_e87510_d_n10, assign56340_body8_e87510_d_n11, assign56340_body8_e87510_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 == 0.0)) {
        let assign56340_body8_e87505: f64 = (-locals.var_t2);
        let assign56340_body8_e87507: f64 = (assign56340_body8_e87505 + 1.0);
        let assign56340_body8_e87508: f64 = (locals.var_t11 * assign56340_body8_e87507);
        (assign56340_body8_e87508, ((locals.var_t11_dn0 * assign56340_body8_e87507) + (locals.var_t11 * (-locals.var_t2_dn0))), ((locals.var_t11_dn2 * assign56340_body8_e87507) + (locals.var_t11 * (-locals.var_t2_dn2))), ((locals.var_t11_dn4 * assign56340_body8_e87507) + (locals.var_t11 * (-locals.var_t2_dn4))), ((locals.var_t11_dn5 * assign56340_body8_e87507) + (locals.var_t11 * (-locals.var_t2_dn5))), ((locals.var_t11_dn6 * assign56340_body8_e87507) + (locals.var_t11 * (-locals.var_t2_dn6))), ((locals.var_t11_dn7 * assign56340_body8_e87507) + (locals.var_t11 * (-locals.var_t2_dn7))), ((locals.var_t11_dn8 * assign56340_body8_e87507) + (locals.var_t11 * (-locals.var_t2_dn8))), ((locals.var_t11_dn9 * assign56340_body8_e87507) + (locals.var_t11 * (-locals.var_t2_dn9))), ((locals.var_t11_dn10 * assign56340_body8_e87507) + (locals.var_t11 * (-locals.var_t2_dn10))), ((locals.var_t11_dn11 * assign56340_body8_e87507) + (locals.var_t11 * (-locals.var_t2_dn11))), ((locals.var_t11_dn14 * assign56340_body8_e87507) + (locals.var_t11 * (-locals.var_t2_dn14))),)
    } else {
        (locals.var_q_sat_dps, locals.var_q_sat_dps_dn0, locals.var_q_sat_dps_dn2, locals.var_q_sat_dps_dn4, locals.var_q_sat_dps_dn5, locals.var_q_sat_dps_dn6, locals.var_q_sat_dps_dn7, locals.var_q_sat_dps_dn8, locals.var_q_sat_dps_dn9, locals.var_q_sat_dps_dn10, locals.var_q_sat_dps_dn11, locals.var_q_sat_dps_dn14,)
    }
};
            locals.var_q_sat_dps = assign56340_body8_e87510;
            locals.var_q_sat_dps_dn0 = assign56340_body8_e87510_d_n0;
            locals.var_q_sat_dps_dn2 = assign56340_body8_e87510_d_n2;
            locals.var_q_sat_dps_dn4 = assign56340_body8_e87510_d_n4;
            locals.var_q_sat_dps_dn5 = assign56340_body8_e87510_d_n5;
            locals.var_q_sat_dps_dn6 = assign56340_body8_e87510_d_n6;
            locals.var_q_sat_dps_dn7 = assign56340_body8_e87510_d_n7;
            locals.var_q_sat_dps_dn8 = assign56340_body8_e87510_d_n8;
            locals.var_q_sat_dps_dn9 = assign56340_body8_e87510_d_n9;
            locals.var_q_sat_dps_dn10 = assign56340_body8_e87510_d_n10;
            locals.var_q_sat_dps_dn11 = assign56340_body8_e87510_d_n11;
            locals.var_q_sat_dps_dn14 = assign56340_body8_e87510_d_n14;
            locals.var_q_sat_dps_rv = 0.0;
            let (assign56340_body9_e87528,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 == 0.0)) && (locals.var_flg_conv != 0.0)) {
        let assign56340_body9_e87526: f64 = (150.0 + 1.0);
        (assign56340_body9_e87526,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign56340_body9_e87528;
            locals.var_lp_s0_rv = 0.0;
            let (assign56340_body10_e87552, assign56340_body10_e87552_d_n0, assign56340_body10_e87552_d_n2, assign56340_body10_e87552_d_n4, assign56340_body10_e87552_d_n5, assign56340_body10_e87552_d_n6, assign56340_body10_e87552_d_n7, assign56340_body10_e87552_d_n8, assign56340_body10_e87552_d_n9, assign56340_body10_e87552_d_n10, assign56340_body10_e87552_d_n11, assign56340_body10_e87552_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign56340_body10_e87544: f64 = (-locals.var_cox);
        let assign56340_body10_e87547: f64 = (locals.var_vgpsat - locals.var_phi_vsat);
        let assign56340_body10_e87548: f64 = (assign56340_body10_e87544 * assign56340_body10_e87547);
        let assign56340_body10_e87550: f64 = (assign56340_body10_e87548 + locals.var_q_sat);
        (assign56340_body10_e87550, ((((-locals.var_cox_dn0) * assign56340_body10_e87547) + (assign56340_body10_e87544 * (locals.var_vgpsat_dn0 - locals.var_phi_vsat_dn0))) + locals.var_q_sat_dn0), ((((-locals.var_cox_dn2) * assign56340_body10_e87547) + (assign56340_body10_e87544 * (locals.var_vgpsat_dn2 - locals.var_phi_vsat_dn2))) + locals.var_q_sat_dn2), ((((-locals.var_cox_dn4) * assign56340_body10_e87547) + (assign56340_body10_e87544 * (locals.var_vgpsat_dn4 - locals.var_phi_vsat_dn4))) + locals.var_q_sat_dn4), ((((-locals.var_cox_dn5) * assign56340_body10_e87547) + (assign56340_body10_e87544 * (locals.var_vgpsat_dn5 - locals.var_phi_vsat_dn5))) + locals.var_q_sat_dn5), ((((-locals.var_cox_dn6) * assign56340_body10_e87547) + (assign56340_body10_e87544 * (locals.var_vgpsat_dn6 - locals.var_phi_vsat_dn6))) + locals.var_q_sat_dn6), ((((-locals.var_cox_dn7) * assign56340_body10_e87547) + (assign56340_body10_e87544 * (locals.var_vgpsat_dn7 - locals.var_phi_vsat_dn7))) + locals.var_q_sat_dn7), ((((-locals.var_cox_dn8) * assign56340_body10_e87547) + (assign56340_body10_e87544 * (locals.var_vgpsat_dn8 - locals.var_phi_vsat_dn8))) + locals.var_q_sat_dn8), ((((-locals.var_cox_dn9) * assign56340_body10_e87547) + (assign56340_body10_e87544 * (locals.var_vgpsat_dn9 - locals.var_phi_vsat_dn9))) + locals.var_q_sat_dn9), ((((-locals.var_cox_dn10) * assign56340_body10_e87547) + (assign56340_body10_e87544 * (locals.var_vgpsat_dn10 - locals.var_phi_vsat_dn10))) + locals.var_q_sat_dn10), ((((-locals.var_cox_dn11) * assign56340_body10_e87547) + (assign56340_body10_e87544 * (locals.var_vgpsat_dn11 - locals.var_phi_vsat_dn11))) + locals.var_q_sat_dn11), ((((-locals.var_cox_dn14) * assign56340_body10_e87547) + (assign56340_body10_e87544 * (locals.var_vgpsat_dn14 - locals.var_phi_vsat_dn14))) + locals.var_q_sat_dn14),)
    } else {
        (locals.var_pf1__blk1102, locals.var_pf1__blk1102_dn0, locals.var_pf1__blk1102_dn2, locals.var_pf1__blk1102_dn4, locals.var_pf1__blk1102_dn5, locals.var_pf1__blk1102_dn6, locals.var_pf1__blk1102_dn7, locals.var_pf1__blk1102_dn8, locals.var_pf1__blk1102_dn9, locals.var_pf1__blk1102_dn10, locals.var_pf1__blk1102_dn11, locals.var_pf1__blk1102_dn14,)
    }
};
            locals.var_pf1__blk1102 = assign56340_body10_e87552;
            locals.var_pf1__blk1102_dn0 = assign56340_body10_e87552_d_n0;
            locals.var_pf1__blk1102_dn2 = assign56340_body10_e87552_d_n2;
            locals.var_pf1__blk1102_dn4 = assign56340_body10_e87552_d_n4;
            locals.var_pf1__blk1102_dn5 = assign56340_body10_e87552_d_n5;
            locals.var_pf1__blk1102_dn6 = assign56340_body10_e87552_d_n6;
            locals.var_pf1__blk1102_dn7 = assign56340_body10_e87552_d_n7;
            locals.var_pf1__blk1102_dn8 = assign56340_body10_e87552_d_n8;
            locals.var_pf1__blk1102_dn9 = assign56340_body10_e87552_d_n9;
            locals.var_pf1__blk1102_dn10 = assign56340_body10_e87552_d_n10;
            locals.var_pf1__blk1102_dn11 = assign56340_body10_e87552_d_n11;
            locals.var_pf1__blk1102_dn14 = assign56340_body10_e87552_d_n14;
            locals.var_pf1__blk1102_rv = 0.0;
            let (assign56340_body11_e87571, assign56340_body11_e87571_d_n0, assign56340_body11_e87571_d_n2, assign56340_body11_e87571_d_n4, assign56340_body11_e87571_d_n5, assign56340_body11_e87571_d_n6, assign56340_body11_e87571_d_n7, assign56340_body11_e87571_d_n8, assign56340_body11_e87571_d_n9, assign56340_body11_e87571_d_n10, assign56340_body11_e87571_d_n11, assign56340_body11_e87571_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign56340_body11_e87569: f64 = (locals.var_cox + locals.var_q_sat_dps);
        (assign56340_body11_e87569, (locals.var_cox_dn0 + locals.var_q_sat_dps_dn0), (locals.var_cox_dn2 + locals.var_q_sat_dps_dn2), (locals.var_cox_dn4 + locals.var_q_sat_dps_dn4), (locals.var_cox_dn5 + locals.var_q_sat_dps_dn5), (locals.var_cox_dn6 + locals.var_q_sat_dps_dn6), (locals.var_cox_dn7 + locals.var_q_sat_dps_dn7), (locals.var_cox_dn8 + locals.var_q_sat_dps_dn8), (locals.var_cox_dn9 + locals.var_q_sat_dps_dn9), (locals.var_cox_dn10 + locals.var_q_sat_dps_dn10), (locals.var_cox_dn11 + locals.var_q_sat_dps_dn11), (locals.var_cox_dn14 + locals.var_q_sat_dps_dn14),)
    } else {
        (locals.var_pf11__blk1103, locals.var_pf11__blk1103_dn0, locals.var_pf11__blk1103_dn2, locals.var_pf11__blk1103_dn4, locals.var_pf11__blk1103_dn5, locals.var_pf11__blk1103_dn6, locals.var_pf11__blk1103_dn7, locals.var_pf11__blk1103_dn8, locals.var_pf11__blk1103_dn9, locals.var_pf11__blk1103_dn10, locals.var_pf11__blk1103_dn11, locals.var_pf11__blk1103_dn14,)
    }
};
            locals.var_pf11__blk1103 = assign56340_body11_e87571;
            locals.var_pf11__blk1103_dn0 = assign56340_body11_e87571_d_n0;
            locals.var_pf11__blk1103_dn2 = assign56340_body11_e87571_d_n2;
            locals.var_pf11__blk1103_dn4 = assign56340_body11_e87571_d_n4;
            locals.var_pf11__blk1103_dn5 = assign56340_body11_e87571_d_n5;
            locals.var_pf11__blk1103_dn6 = assign56340_body11_e87571_d_n6;
            locals.var_pf11__blk1103_dn7 = assign56340_body11_e87571_d_n7;
            locals.var_pf11__blk1103_dn8 = assign56340_body11_e87571_d_n8;
            locals.var_pf11__blk1103_dn9 = assign56340_body11_e87571_d_n9;
            locals.var_pf11__blk1103_dn10 = assign56340_body11_e87571_d_n10;
            locals.var_pf11__blk1103_dn11 = assign56340_body11_e87571_d_n11;
            locals.var_pf11__blk1103_dn14 = assign56340_body11_e87571_d_n14;
            locals.var_pf11__blk1103_rv = 0.0;
            let (assign56340_body12_e87591, assign56340_body12_e87591_d_n0, assign56340_body12_e87591_d_n2, assign56340_body12_e87591_d_n4, assign56340_body12_e87591_d_n5, assign56340_body12_e87591_d_n6, assign56340_body12_e87591_d_n7, assign56340_body12_e87591_d_n8, assign56340_body12_e87591_d_n9, assign56340_body12_e87591_d_n10, assign56340_body12_e87591_d_n11, assign56340_body12_e87591_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign56340_body12_e87587: f64 = (-locals.var_pf1__blk1102);
        let assign56340_body12_e87589: f64 = (assign56340_body12_e87587 / locals.var_pf11__blk1103);
        (assign56340_body12_e87589, ((((-locals.var_pf1__blk1102_dn0) * locals.var_pf11__blk1103) - (assign56340_body12_e87587 * locals.var_pf11__blk1103_dn0)) / (locals.var_pf11__blk1103 * locals.var_pf11__blk1103)), ((((-locals.var_pf1__blk1102_dn2) * locals.var_pf11__blk1103) - (assign56340_body12_e87587 * locals.var_pf11__blk1103_dn2)) / (locals.var_pf11__blk1103 * locals.var_pf11__blk1103)), ((((-locals.var_pf1__blk1102_dn4) * locals.var_pf11__blk1103) - (assign56340_body12_e87587 * locals.var_pf11__blk1103_dn4)) / (locals.var_pf11__blk1103 * locals.var_pf11__blk1103)), ((((-locals.var_pf1__blk1102_dn5) * locals.var_pf11__blk1103) - (assign56340_body12_e87587 * locals.var_pf11__blk1103_dn5)) / (locals.var_pf11__blk1103 * locals.var_pf11__blk1103)), ((((-locals.var_pf1__blk1102_dn6) * locals.var_pf11__blk1103) - (assign56340_body12_e87587 * locals.var_pf11__blk1103_dn6)) / (locals.var_pf11__blk1103 * locals.var_pf11__blk1103)), ((((-locals.var_pf1__blk1102_dn7) * locals.var_pf11__blk1103) - (assign56340_body12_e87587 * locals.var_pf11__blk1103_dn7)) / (locals.var_pf11__blk1103 * locals.var_pf11__blk1103)), ((((-locals.var_pf1__blk1102_dn8) * locals.var_pf11__blk1103) - (assign56340_body12_e87587 * locals.var_pf11__blk1103_dn8)) / (locals.var_pf11__blk1103 * locals.var_pf11__blk1103)), ((((-locals.var_pf1__blk1102_dn9) * locals.var_pf11__blk1103) - (assign56340_body12_e87587 * locals.var_pf11__blk1103_dn9)) / (locals.var_pf11__blk1103 * locals.var_pf11__blk1103)), ((((-locals.var_pf1__blk1102_dn10) * locals.var_pf11__blk1103) - (assign56340_body12_e87587 * locals.var_pf11__blk1103_dn10)) / (locals.var_pf11__blk1103 * locals.var_pf11__blk1103)), ((((-locals.var_pf1__blk1102_dn11) * locals.var_pf11__blk1103) - (assign56340_body12_e87587 * locals.var_pf11__blk1103_dn11)) / (locals.var_pf11__blk1103 * locals.var_pf11__blk1103)), ((((-locals.var_pf1__blk1102_dn14) * locals.var_pf11__blk1103) - (assign56340_body12_e87587 * locals.var_pf11__blk1103_dn14)) / (locals.var_pf11__blk1103 * locals.var_pf11__blk1103)),)
    } else {
        (locals.var_dps__blk1114, locals.var_dps__blk1114_dn0, locals.var_dps__blk1114_dn2, locals.var_dps__blk1114_dn4, locals.var_dps__blk1114_dn5, locals.var_dps__blk1114_dn6, locals.var_dps__blk1114_dn7, locals.var_dps__blk1114_dn8, locals.var_dps__blk1114_dn9, locals.var_dps__blk1114_dn10, locals.var_dps__blk1114_dn11, locals.var_dps__blk1114_dn14,)
    }
};
            locals.var_dps__blk1114 = assign56340_body12_e87591;
            locals.var_dps__blk1114_dn0 = assign56340_body12_e87591_d_n0;
            locals.var_dps__blk1114_dn2 = assign56340_body12_e87591_d_n2;
            locals.var_dps__blk1114_dn4 = assign56340_body12_e87591_d_n4;
            locals.var_dps__blk1114_dn5 = assign56340_body12_e87591_d_n5;
            locals.var_dps__blk1114_dn6 = assign56340_body12_e87591_d_n6;
            locals.var_dps__blk1114_dn7 = assign56340_body12_e87591_d_n7;
            locals.var_dps__blk1114_dn8 = assign56340_body12_e87591_d_n8;
            locals.var_dps__blk1114_dn9 = assign56340_body12_e87591_d_n9;
            locals.var_dps__blk1114_dn10 = assign56340_body12_e87591_d_n10;
            locals.var_dps__blk1114_dn11 = assign56340_body12_e87591_d_n11;
            locals.var_dps__blk1114_dn14 = assign56340_body12_e87591_d_n14;
            locals.var_dps__blk1114_rv = 0.0;
            let assign56340_body13_e87593: f64 = (locals.var_dps__blk1114).abs();
            let assign56340_body13_e87595: f64 = if assign56340_body13_e87593 < 1e-10 { 1.0 } else { 0.0 };
            locals.var_guard1411 = assign56340_body13_e87595;
            locals.var_guard1411_rv = 0.0;
            let (assign56340_body14_e87614,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 == 0.0)) && (locals.var_flg_conv == 0.0)) && (locals.var_guard1411 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign56340_body14_e87614;
            locals.var_flg_conv_rv = 0.0;
            let assign56340_body15_e87617: f64 = if locals.var_dps__blk1114 > 0.1 { 1.0 } else { 0.0 };
            locals.var_guard1412 = assign56340_body15_e87617;
            locals.var_guard1412_rv = 0.0;
            let (assign56340_body16_e87639, assign56340_body16_e87639_d_n0, assign56340_body16_e87639_d_n2, assign56340_body16_e87639_d_n4, assign56340_body16_e87639_d_n5, assign56340_body16_e87639_d_n6, assign56340_body16_e87639_d_n7, assign56340_body16_e87639_d_n8, assign56340_body16_e87639_d_n9, assign56340_body16_e87639_d_n10, assign56340_body16_e87639_d_n11, assign56340_body16_e87639_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 == 0.0)) && (locals.var_flg_conv == 0.0)) && (locals.var_guard1411 == 0.0)) && (locals.var_guard1412 != 0.0)) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps__blk1114, locals.var_dps__blk1114_dn0, locals.var_dps__blk1114_dn2, locals.var_dps__blk1114_dn4, locals.var_dps__blk1114_dn5, locals.var_dps__blk1114_dn6, locals.var_dps__blk1114_dn7, locals.var_dps__blk1114_dn8, locals.var_dps__blk1114_dn9, locals.var_dps__blk1114_dn10, locals.var_dps__blk1114_dn11, locals.var_dps__blk1114_dn14,)
    }
};
            locals.var_dps__blk1114 = assign56340_body16_e87639;
            locals.var_dps__blk1114_dn0 = assign56340_body16_e87639_d_n0;
            locals.var_dps__blk1114_dn2 = assign56340_body16_e87639_d_n2;
            locals.var_dps__blk1114_dn4 = assign56340_body16_e87639_d_n4;
            locals.var_dps__blk1114_dn5 = assign56340_body16_e87639_d_n5;
            locals.var_dps__blk1114_dn6 = assign56340_body16_e87639_d_n6;
            locals.var_dps__blk1114_dn7 = assign56340_body16_e87639_d_n7;
            locals.var_dps__blk1114_dn8 = assign56340_body16_e87639_d_n8;
            locals.var_dps__blk1114_dn9 = assign56340_body16_e87639_d_n9;
            locals.var_dps__blk1114_dn10 = assign56340_body16_e87639_d_n10;
            locals.var_dps__blk1114_dn11 = assign56340_body16_e87639_d_n11;
            locals.var_dps__blk1114_dn14 = assign56340_body16_e87639_d_n14;
            locals.var_dps__blk1114_rv = 0.0;
            let assign56340_body17_e87642: f64 = (-0.1);
            let assign56340_body17_e87643: f64 = if locals.var_dps__blk1114 < assign56340_body17_e87642 { 1.0 } else { 0.0 };
            locals.var_guard1413 = assign56340_body17_e87643;
            locals.var_guard1413_rv = 0.0;
            let (assign56340_body18_e87669, assign56340_body18_e87669_d_n0, assign56340_body18_e87669_d_n2, assign56340_body18_e87669_d_n4, assign56340_body18_e87669_d_n5, assign56340_body18_e87669_d_n6, assign56340_body18_e87669_d_n7, assign56340_body18_e87669_d_n8, assign56340_body18_e87669_d_n9, assign56340_body18_e87669_d_n10, assign56340_body18_e87669_d_n11, assign56340_body18_e87669_d_n14,) = {
    if (((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 == 0.0)) && (locals.var_flg_conv == 0.0)) && (locals.var_guard1411 == 0.0)) && (locals.var_guard1412 == 0.0)) && (locals.var_guard1413 != 0.0)) {
        let assign56340_body18_e87667: f64 = (-0.1);
        (assign56340_body18_e87667, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps__blk1114, locals.var_dps__blk1114_dn0, locals.var_dps__blk1114_dn2, locals.var_dps__blk1114_dn4, locals.var_dps__blk1114_dn5, locals.var_dps__blk1114_dn6, locals.var_dps__blk1114_dn7, locals.var_dps__blk1114_dn8, locals.var_dps__blk1114_dn9, locals.var_dps__blk1114_dn10, locals.var_dps__blk1114_dn11, locals.var_dps__blk1114_dn14,)
    }
};
            locals.var_dps__blk1114 = assign56340_body18_e87669;
            locals.var_dps__blk1114_dn0 = assign56340_body18_e87669_d_n0;
            locals.var_dps__blk1114_dn2 = assign56340_body18_e87669_d_n2;
            locals.var_dps__blk1114_dn4 = assign56340_body18_e87669_d_n4;
            locals.var_dps__blk1114_dn5 = assign56340_body18_e87669_d_n5;
            locals.var_dps__blk1114_dn6 = assign56340_body18_e87669_d_n6;
            locals.var_dps__blk1114_dn7 = assign56340_body18_e87669_d_n7;
            locals.var_dps__blk1114_dn8 = assign56340_body18_e87669_d_n8;
            locals.var_dps__blk1114_dn9 = assign56340_body18_e87669_d_n9;
            locals.var_dps__blk1114_dn10 = assign56340_body18_e87669_d_n10;
            locals.var_dps__blk1114_dn11 = assign56340_body18_e87669_d_n11;
            locals.var_dps__blk1114_dn14 = assign56340_body18_e87669_d_n14;
            locals.var_dps__blk1114_rv = 0.0;
            let (assign56340_body19_e87688, assign56340_body19_e87688_d_n0, assign56340_body19_e87688_d_n2, assign56340_body19_e87688_d_n4, assign56340_body19_e87688_d_n5, assign56340_body19_e87688_d_n6, assign56340_body19_e87688_d_n7, assign56340_body19_e87688_d_n8, assign56340_body19_e87688_d_n9, assign56340_body19_e87688_d_n10, assign56340_body19_e87688_d_n11, assign56340_body19_e87688_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign56340_body19_e87686: f64 = (locals.var_phi_vsat + locals.var_dps__blk1114);
        (assign56340_body19_e87686, (locals.var_phi_vsat_dn0 + locals.var_dps__blk1114_dn0), (locals.var_phi_vsat_dn2 + locals.var_dps__blk1114_dn2), (locals.var_phi_vsat_dn4 + locals.var_dps__blk1114_dn4), (locals.var_phi_vsat_dn5 + locals.var_dps__blk1114_dn5), (locals.var_phi_vsat_dn6 + locals.var_dps__blk1114_dn6), (locals.var_phi_vsat_dn7 + locals.var_dps__blk1114_dn7), (locals.var_phi_vsat_dn8 + locals.var_dps__blk1114_dn8), (locals.var_phi_vsat_dn9 + locals.var_dps__blk1114_dn9), (locals.var_phi_vsat_dn10 + locals.var_dps__blk1114_dn10), (locals.var_phi_vsat_dn11 + locals.var_dps__blk1114_dn11), (locals.var_phi_vsat_dn14 + locals.var_dps__blk1114_dn14),)
    } else {
        (locals.var_phi_vsat, locals.var_phi_vsat_dn0, locals.var_phi_vsat_dn2, locals.var_phi_vsat_dn4, locals.var_phi_vsat_dn5, locals.var_phi_vsat_dn6, locals.var_phi_vsat_dn7, locals.var_phi_vsat_dn8, locals.var_phi_vsat_dn9, locals.var_phi_vsat_dn10, locals.var_phi_vsat_dn11, locals.var_phi_vsat_dn14,)
    }
};
            locals.var_phi_vsat = assign56340_body19_e87688;
            locals.var_phi_vsat_dn0 = assign56340_body19_e87688_d_n0;
            locals.var_phi_vsat_dn2 = assign56340_body19_e87688_d_n2;
            locals.var_phi_vsat_dn4 = assign56340_body19_e87688_d_n4;
            locals.var_phi_vsat_dn5 = assign56340_body19_e87688_d_n5;
            locals.var_phi_vsat_dn6 = assign56340_body19_e87688_d_n6;
            locals.var_phi_vsat_dn7 = assign56340_body19_e87688_d_n7;
            locals.var_phi_vsat_dn8 = assign56340_body19_e87688_d_n8;
            locals.var_phi_vsat_dn9 = assign56340_body19_e87688_d_n9;
            locals.var_phi_vsat_dn10 = assign56340_body19_e87688_d_n10;
            locals.var_phi_vsat_dn11 = assign56340_body19_e87688_d_n11;
            locals.var_phi_vsat_dn14 = assign56340_body19_e87688_d_n14;
            locals.var_phi_vsat_rv = 0.0;
            let (assign56340_body20_e87704,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 == 0.0)) {
        let assign56340_body20_e87702: f64 = (locals.var_lp_s0 + 1.0);
        (assign56340_body20_e87702,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign56340_body20_e87704;
            locals.var_lp_s0_rv = 0.0;
        }

        let (assign56350_e87718, assign56350_e87718_d_n0, assign56350_e87718_d_n2, assign56350_e87718_d_n4, assign56350_e87718_d_n5, assign56350_e87718_d_n6, assign56350_e87718_d_n7, assign56350_e87718_d_n8, assign56350_e87718_d_n9, assign56350_e87718_d_n10, assign56350_e87718_d_n11, assign56350_e87718_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 == 0.0)) {
        (locals.var_phi_vsat, locals.var_phi_vsat_dn0, locals.var_phi_vsat_dn2, locals.var_phi_vsat_dn4, locals.var_phi_vsat_dn5, locals.var_phi_vsat_dn6, locals.var_phi_vsat_dn7, locals.var_phi_vsat_dn8, locals.var_phi_vsat_dn9, locals.var_phi_vsat_dn10, locals.var_phi_vsat_dn11, locals.var_phi_vsat_dn14,)
    } else {
        (locals.var_ps0_res, locals.var_ps0_res_dn0, locals.var_ps0_res_dn2, locals.var_ps0_res_dn4, locals.var_ps0_res_dn5, locals.var_ps0_res_dn6, locals.var_ps0_res_dn7, locals.var_ps0_res_dn8, locals.var_ps0_res_dn9, locals.var_ps0_res_dn10, locals.var_ps0_res_dn11, locals.var_ps0_res_dn14,)
    }
};
        locals.var_ps0_res = assign56350_e87718;
        locals.var_ps0_res_dn0 = assign56350_e87718_d_n0;
        locals.var_ps0_res_dn2 = assign56350_e87718_d_n2;
        locals.var_ps0_res_dn4 = assign56350_e87718_d_n4;
        locals.var_ps0_res_dn5 = assign56350_e87718_d_n5;
        locals.var_ps0_res_dn6 = assign56350_e87718_d_n6;
        locals.var_ps0_res_dn7 = assign56350_e87718_d_n7;
        locals.var_ps0_res_dn8 = assign56350_e87718_d_n8;
        locals.var_ps0_res_dn9 = assign56350_e87718_d_n9;
        locals.var_ps0_res_dn10 = assign56350_e87718_d_n10;
        locals.var_ps0_res_dn11 = assign56350_e87718_d_n11;
        locals.var_ps0_res_dn14 = assign56350_e87718_d_n14;
        locals.var_ps0_res_rv = 0.0;

        let (assign56360_e87732, assign56360_e87732_d_n0, assign56360_e87732_d_n2, assign56360_e87732_d_n4, assign56360_e87732_d_n5, assign56360_e87732_d_n6, assign56360_e87732_d_n7, assign56360_e87732_d_n8, assign56360_e87732_d_n9, assign56360_e87732_d_n10, assign56360_e87732_d_n11, assign56360_e87732_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 == 0.0)) {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn4, locals.var_vdsorg_dn5, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn8, locals.var_vdsorg_dn9, locals.var_vdsorg_dn10, locals.var_vdsorg_dn11, locals.var_vdsorg_dn14,)
    } else {
        (locals.var_vds_res, locals.var_vds_res_dn0, locals.var_vds_res_dn2, locals.var_vds_res_dn4, locals.var_vds_res_dn5, locals.var_vds_res_dn6, locals.var_vds_res_dn7, locals.var_vds_res_dn8, locals.var_vds_res_dn9, locals.var_vds_res_dn10, locals.var_vds_res_dn11, locals.var_vds_res_dn14,)
    }
};
        locals.var_vds_res = assign56360_e87732;
        locals.var_vds_res_dn0 = assign56360_e87732_d_n0;
        locals.var_vds_res_dn2 = assign56360_e87732_d_n2;
        locals.var_vds_res_dn4 = assign56360_e87732_d_n4;
        locals.var_vds_res_dn5 = assign56360_e87732_d_n5;
        locals.var_vds_res_dn6 = assign56360_e87732_d_n6;
        locals.var_vds_res_dn7 = assign56360_e87732_d_n7;
        locals.var_vds_res_dn8 = assign56360_e87732_d_n8;
        locals.var_vds_res_dn9 = assign56360_e87732_d_n9;
        locals.var_vds_res_dn10 = assign56360_e87732_d_n10;
        locals.var_vds_res_dn11 = assign56360_e87732_d_n11;
        locals.var_vds_res_dn14 = assign56360_e87732_d_n14;
        locals.var_vds_res_rv = 0.0;

        let (assign56370_e87755, assign56370_e87755_d_n0, assign56370_e87755_d_n2, assign56370_e87755_d_n4, assign56370_e87755_d_n5, assign56370_e87755_d_n6, assign56370_e87755_d_n7, assign56370_e87755_d_n8, assign56370_e87755_d_n9, assign56370_e87755_d_n10, assign56370_e87755_d_n11, assign56370_e87755_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 == 0.0)) {
        let assign56370_e87746: f64 = (locals.var_ps0_res * locals.var_ps0_res);
        let assign56370_e87749: f64 = (4.0 * p.p405);
        let assign56370_e87751: f64 = (assign56370_e87749 * p.p405);
        let assign56370_e87752: f64 = (assign56370_e87746 + assign56370_e87751);
        let assign56370_e87753: f64 = (assign56370_e87752).sqrt();
        (assign56370_e87753, (((locals.var_ps0_res_dn0 * locals.var_ps0_res) + (locals.var_ps0_res * locals.var_ps0_res_dn0)) / (2.0 * assign56370_e87753)), (((locals.var_ps0_res_dn2 * locals.var_ps0_res) + (locals.var_ps0_res * locals.var_ps0_res_dn2)) / (2.0 * assign56370_e87753)), (((locals.var_ps0_res_dn4 * locals.var_ps0_res) + (locals.var_ps0_res * locals.var_ps0_res_dn4)) / (2.0 * assign56370_e87753)), (((locals.var_ps0_res_dn5 * locals.var_ps0_res) + (locals.var_ps0_res * locals.var_ps0_res_dn5)) / (2.0 * assign56370_e87753)), (((locals.var_ps0_res_dn6 * locals.var_ps0_res) + (locals.var_ps0_res * locals.var_ps0_res_dn6)) / (2.0 * assign56370_e87753)), (((locals.var_ps0_res_dn7 * locals.var_ps0_res) + (locals.var_ps0_res * locals.var_ps0_res_dn7)) / (2.0 * assign56370_e87753)), (((locals.var_ps0_res_dn8 * locals.var_ps0_res) + (locals.var_ps0_res * locals.var_ps0_res_dn8)) / (2.0 * assign56370_e87753)), (((locals.var_ps0_res_dn9 * locals.var_ps0_res) + (locals.var_ps0_res * locals.var_ps0_res_dn9)) / (2.0 * assign56370_e87753)), (((locals.var_ps0_res_dn10 * locals.var_ps0_res) + (locals.var_ps0_res * locals.var_ps0_res_dn10)) / (2.0 * assign56370_e87753)), (((locals.var_ps0_res_dn11 * locals.var_ps0_res) + (locals.var_ps0_res * locals.var_ps0_res_dn11)) / (2.0 * assign56370_e87753)), (((locals.var_ps0_res_dn14 * locals.var_ps0_res) + (locals.var_ps0_res * locals.var_ps0_res_dn14)) / (2.0 * assign56370_e87753)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign56370_e87755;
        locals.var_tmf2_dn0 = assign56370_e87755_d_n0;
        locals.var_tmf2_dn2 = assign56370_e87755_d_n2;
        locals.var_tmf2_dn4 = assign56370_e87755_d_n4;
        locals.var_tmf2_dn5 = assign56370_e87755_d_n5;
        locals.var_tmf2_dn6 = assign56370_e87755_d_n6;
        locals.var_tmf2_dn7 = assign56370_e87755_d_n7;
        locals.var_tmf2_dn8 = assign56370_e87755_d_n8;
        locals.var_tmf2_dn9 = assign56370_e87755_d_n9;
        locals.var_tmf2_dn10 = assign56370_e87755_d_n10;
        locals.var_tmf2_dn11 = assign56370_e87755_d_n11;
        locals.var_tmf2_dn14 = assign56370_e87755_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign56380_e87775, assign56380_e87775_d_n0, assign56380_e87775_d_n2, assign56380_e87775_d_n4, assign56380_e87775_d_n5, assign56380_e87775_d_n6, assign56380_e87775_d_n7, assign56380_e87775_d_n8, assign56380_e87775_d_n9, assign56380_e87775_d_n10, assign56380_e87775_d_n11, assign56380_e87775_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 == 0.0)) {
        let assign56380_e87771: f64 = (locals.var_ps0_res / locals.var_tmf2);
        let assign56380_e87772: f64 = (1.0 + assign56380_e87771);
        let assign56380_e87773: f64 = (0.5 * assign56380_e87772);
        (assign56380_e87773, (0.5 * (((locals.var_ps0_res_dn0 * locals.var_tmf2) - (locals.var_ps0_res * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_ps0_res_dn2 * locals.var_tmf2) - (locals.var_ps0_res * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_ps0_res_dn4 * locals.var_tmf2) - (locals.var_ps0_res * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_ps0_res_dn5 * locals.var_tmf2) - (locals.var_ps0_res * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_ps0_res_dn6 * locals.var_tmf2) - (locals.var_ps0_res * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_ps0_res_dn7 * locals.var_tmf2) - (locals.var_ps0_res * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_ps0_res_dn8 * locals.var_tmf2) - (locals.var_ps0_res * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_ps0_res_dn9 * locals.var_tmf2) - (locals.var_ps0_res * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_ps0_res_dn10 * locals.var_tmf2) - (locals.var_ps0_res * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_ps0_res_dn11 * locals.var_tmf2) - (locals.var_ps0_res * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_ps0_res_dn14 * locals.var_tmf2) - (locals.var_ps0_res * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign56380_e87775;
        locals.var_t0_dn0 = assign56380_e87775_d_n0;
        locals.var_t0_dn2 = assign56380_e87775_d_n2;
        locals.var_t0_dn4 = assign56380_e87775_d_n4;
        locals.var_t0_dn5 = assign56380_e87775_d_n5;
        locals.var_t0_dn6 = assign56380_e87775_d_n6;
        locals.var_t0_dn7 = assign56380_e87775_d_n7;
        locals.var_t0_dn8 = assign56380_e87775_d_n8;
        locals.var_t0_dn9 = assign56380_e87775_d_n9;
        locals.var_t0_dn10 = assign56380_e87775_d_n10;
        locals.var_t0_dn11 = assign56380_e87775_d_n11;
        locals.var_t0_dn14 = assign56380_e87775_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign56390_e87793, assign56390_e87793_d_n0, assign56390_e87793_d_n2, assign56390_e87793_d_n4, assign56390_e87793_d_n5, assign56390_e87793_d_n6, assign56390_e87793_d_n7, assign56390_e87793_d_n8, assign56390_e87793_d_n9, assign56390_e87793_d_n10, assign56390_e87793_d_n11, assign56390_e87793_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 == 0.0)) {
        let assign56390_e87790: f64 = (locals.var_ps0_res + locals.var_tmf2);
        let assign56390_e87791: f64 = (0.5 * assign56390_e87790);
        (assign56390_e87791, (0.5 * (locals.var_ps0_res_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_ps0_res_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_ps0_res_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_ps0_res_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_ps0_res_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_ps0_res_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_ps0_res_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_ps0_res_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_ps0_res_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_ps0_res_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_ps0_res_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_vdssat_res, locals.var_vdssat_res_dn0, locals.var_vdssat_res_dn2, locals.var_vdssat_res_dn4, locals.var_vdssat_res_dn5, locals.var_vdssat_res_dn6, locals.var_vdssat_res_dn7, locals.var_vdssat_res_dn8, locals.var_vdssat_res_dn9, locals.var_vdssat_res_dn10, locals.var_vdssat_res_dn11, locals.var_vdssat_res_dn14,)
    }
};
        locals.var_vdssat_res = assign56390_e87793;
        locals.var_vdssat_res_dn0 = assign56390_e87793_d_n0;
        locals.var_vdssat_res_dn2 = assign56390_e87793_d_n2;
        locals.var_vdssat_res_dn4 = assign56390_e87793_d_n4;
        locals.var_vdssat_res_dn5 = assign56390_e87793_d_n5;
        locals.var_vdssat_res_dn6 = assign56390_e87793_d_n6;
        locals.var_vdssat_res_dn7 = assign56390_e87793_d_n7;
        locals.var_vdssat_res_dn8 = assign56390_e87793_d_n8;
        locals.var_vdssat_res_dn9 = assign56390_e87793_d_n9;
        locals.var_vdssat_res_dn10 = assign56390_e87793_d_n10;
        locals.var_vdssat_res_dn11 = assign56390_e87793_d_n11;
        locals.var_vdssat_res_dn14 = assign56390_e87793_d_n14;
        locals.var_vdssat_res_rv = 0.0;

        let assign56400_e87796: f64 = if locals.var_vdssat_res < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1414 = assign56400_e87796;
        locals.var_guard1414_rv = 0.0;

        let (assign56410_e87812, assign56410_e87812_d_n0, assign56410_e87812_d_n2, assign56410_e87812_d_n4, assign56410_e87812_d_n5, assign56410_e87812_d_n6, assign56410_e87812_d_n7, assign56410_e87812_d_n8, assign56410_e87812_d_n9, assign56410_e87812_d_n10, assign56410_e87812_d_n11, assign56410_e87812_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 == 0.0)) && (locals.var_guard1414 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vdssat_res, locals.var_vdssat_res_dn0, locals.var_vdssat_res_dn2, locals.var_vdssat_res_dn4, locals.var_vdssat_res_dn5, locals.var_vdssat_res_dn6, locals.var_vdssat_res_dn7, locals.var_vdssat_res_dn8, locals.var_vdssat_res_dn9, locals.var_vdssat_res_dn10, locals.var_vdssat_res_dn11, locals.var_vdssat_res_dn14,)
    }
};
        locals.var_vdssat_res = assign56410_e87812;
        locals.var_vdssat_res_dn0 = assign56410_e87812_d_n0;
        locals.var_vdssat_res_dn2 = assign56410_e87812_d_n2;
        locals.var_vdssat_res_dn4 = assign56410_e87812_d_n4;
        locals.var_vdssat_res_dn5 = assign56410_e87812_d_n5;
        locals.var_vdssat_res_dn6 = assign56410_e87812_d_n6;
        locals.var_vdssat_res_dn7 = assign56410_e87812_d_n7;
        locals.var_vdssat_res_dn8 = assign56410_e87812_d_n8;
        locals.var_vdssat_res_dn9 = assign56410_e87812_d_n9;
        locals.var_vdssat_res_dn10 = assign56410_e87812_d_n10;
        locals.var_vdssat_res_dn11 = assign56410_e87812_d_n11;
        locals.var_vdssat_res_dn14 = assign56410_e87812_d_n14;
        locals.var_vdssat_res_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_206(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign56420_e87828, assign56420_e87828_d_n0, assign56420_e87828_d_n2, assign56420_e87828_d_n4, assign56420_e87828_d_n5, assign56420_e87828_d_n6, assign56420_e87828_d_n7, assign56420_e87828_d_n8, assign56420_e87828_d_n9, assign56420_e87828_d_n10, assign56420_e87828_d_n11, assign56420_e87828_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1397 == 0.0)) && (locals.var_guard1414 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign56420_e87828;
        locals.var_t0_dn0 = assign56420_e87828_d_n0;
        locals.var_t0_dn2 = assign56420_e87828_d_n2;
        locals.var_t0_dn4 = assign56420_e87828_d_n4;
        locals.var_t0_dn5 = assign56420_e87828_d_n5;
        locals.var_t0_dn6 = assign56420_e87828_d_n6;
        locals.var_t0_dn7 = assign56420_e87828_d_n7;
        locals.var_t0_dn8 = assign56420_e87828_d_n8;
        locals.var_t0_dn9 = assign56420_e87828_d_n9;
        locals.var_t0_dn10 = assign56420_e87828_d_n10;
        locals.var_t0_dn11 = assign56420_e87828_d_n11;
        locals.var_t0_dn14 = assign56420_e87828_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign56430_e87841, assign56430_e87841_d_n0, assign56430_e87841_d_n2, assign56430_e87841_d_n4, assign56430_e87841_d_n5, assign56430_e87841_d_n6, assign56430_e87841_d_n7, assign56430_e87841_d_n8, assign56430_e87841_d_n9, assign56430_e87841_d_n10, assign56430_e87841_d_n11, assign56430_e87841_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign56430_e87839: f64 = (locals.var_vds_res / locals.var_vdssat_res);
        (assign56430_e87839, (((locals.var_vds_res_dn0 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn0)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn2 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn2)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn4 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn4)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn5 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn5)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn6 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn6)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn7 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn7)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn8 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn8)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn9 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn9)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn10 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn10)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn11 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn11)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn14 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn14)) / (locals.var_vdssat_res * locals.var_vdssat_res)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign56430_e87841;
        locals.var_t1_dn0 = assign56430_e87841_d_n0;
        locals.var_t1_dn2 = assign56430_e87841_d_n2;
        locals.var_t1_dn4 = assign56430_e87841_d_n4;
        locals.var_t1_dn5 = assign56430_e87841_d_n5;
        locals.var_t1_dn6 = assign56430_e87841_d_n6;
        locals.var_t1_dn7 = assign56430_e87841_d_n7;
        locals.var_t1_dn8 = assign56430_e87841_d_n8;
        locals.var_t1_dn9 = assign56430_e87841_d_n9;
        locals.var_t1_dn10 = assign56430_e87841_d_n10;
        locals.var_t1_dn11 = assign56430_e87841_d_n11;
        locals.var_t1_dn14 = assign56430_e87841_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign56440_e87861, assign56440_e87861_d_n0, assign56440_e87861_d_n2, assign56440_e87861_d_n4, assign56440_e87861_d_n5, assign56440_e87861_d_n6, assign56440_e87861_d_n7, assign56440_e87861_d_n8, assign56440_e87861_d_n9, assign56440_e87861_d_n10, assign56440_e87861_d_n11, assign56440_e87861_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let (assign56440_e87859, assign56440_e87859_d_n0, assign56440_e87859_d_n2, assign56440_e87859_d_n4, assign56440_e87859_d_n5, assign56440_e87859_d_n6, assign56440_e87859_d_n7, assign56440_e87859_d_n8, assign56440_e87859_d_n9, assign56440_e87859_d_n10, assign56440_e87859_d_n11, assign56440_e87859_d_n14,) = {
            if (locals.var_t1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign56440_e87857: f64 = (p.p383 - 1.0);
                let assign56440_e87858: f64 = (locals.var_t1).powf(assign56440_e87857);
                (assign56440_e87858, if 0.0 == 0.0 && ((assign56440_e87857) as f64).is_finite() && ((assign56440_e87857) as f64).fract() == 0.0 { if assign56440_e87857 == 0.0 { 0.0 } else { (assign56440_e87857 * ((locals.var_t1).powf(assign56440_e87857 - 1.0) * locals.var_t1_dn0)) } } else { (assign56440_e87858 * (assign56440_e87857 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign56440_e87857) as f64).is_finite() && ((assign56440_e87857) as f64).fract() == 0.0 { if assign56440_e87857 == 0.0 { 0.0 } else { (assign56440_e87857 * ((locals.var_t1).powf(assign56440_e87857 - 1.0) * locals.var_t1_dn2)) } } else { (assign56440_e87858 * (assign56440_e87857 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign56440_e87857) as f64).is_finite() && ((assign56440_e87857) as f64).fract() == 0.0 { if assign56440_e87857 == 0.0 { 0.0 } else { (assign56440_e87857 * ((locals.var_t1).powf(assign56440_e87857 - 1.0) * locals.var_t1_dn4)) } } else { (assign56440_e87858 * (assign56440_e87857 * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign56440_e87857) as f64).is_finite() && ((assign56440_e87857) as f64).fract() == 0.0 { if assign56440_e87857 == 0.0 { 0.0 } else { (assign56440_e87857 * ((locals.var_t1).powf(assign56440_e87857 - 1.0) * locals.var_t1_dn5)) } } else { (assign56440_e87858 * (assign56440_e87857 * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign56440_e87857) as f64).is_finite() && ((assign56440_e87857) as f64).fract() == 0.0 { if assign56440_e87857 == 0.0 { 0.0 } else { (assign56440_e87857 * ((locals.var_t1).powf(assign56440_e87857 - 1.0) * locals.var_t1_dn6)) } } else { (assign56440_e87858 * (assign56440_e87857 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign56440_e87857) as f64).is_finite() && ((assign56440_e87857) as f64).fract() == 0.0 { if assign56440_e87857 == 0.0 { 0.0 } else { (assign56440_e87857 * ((locals.var_t1).powf(assign56440_e87857 - 1.0) * locals.var_t1_dn7)) } } else { (assign56440_e87858 * (assign56440_e87857 * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign56440_e87857) as f64).is_finite() && ((assign56440_e87857) as f64).fract() == 0.0 { if assign56440_e87857 == 0.0 { 0.0 } else { (assign56440_e87857 * ((locals.var_t1).powf(assign56440_e87857 - 1.0) * locals.var_t1_dn8)) } } else { (assign56440_e87858 * (assign56440_e87857 * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign56440_e87857) as f64).is_finite() && ((assign56440_e87857) as f64).fract() == 0.0 { if assign56440_e87857 == 0.0 { 0.0 } else { (assign56440_e87857 * ((locals.var_t1).powf(assign56440_e87857 - 1.0) * locals.var_t1_dn9)) } } else { (assign56440_e87858 * (assign56440_e87857 * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign56440_e87857) as f64).is_finite() && ((assign56440_e87857) as f64).fract() == 0.0 { if assign56440_e87857 == 0.0 { 0.0 } else { (assign56440_e87857 * ((locals.var_t1).powf(assign56440_e87857 - 1.0) * locals.var_t1_dn10)) } } else { (assign56440_e87858 * (assign56440_e87857 * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign56440_e87857) as f64).is_finite() && ((assign56440_e87857) as f64).fract() == 0.0 { if assign56440_e87857 == 0.0 { 0.0 } else { (assign56440_e87857 * ((locals.var_t1).powf(assign56440_e87857 - 1.0) * locals.var_t1_dn11)) } } else { (assign56440_e87858 * (assign56440_e87857 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign56440_e87857) as f64).is_finite() && ((assign56440_e87857) as f64).fract() == 0.0 { if assign56440_e87857 == 0.0 { 0.0 } else { (assign56440_e87857 * ((locals.var_t1).powf(assign56440_e87857 - 1.0) * locals.var_t1_dn14)) } } else { (assign56440_e87858 * (assign56440_e87857 * (locals.var_t1_dn14 / locals.var_t1))) },)
            }
        };
        (assign56440_e87859, assign56440_e87859_d_n0, assign56440_e87859_d_n2, assign56440_e87859_d_n4, assign56440_e87859_d_n5, assign56440_e87859_d_n6, assign56440_e87859_d_n7, assign56440_e87859_d_n8, assign56440_e87859_d_n9, assign56440_e87859_d_n10, assign56440_e87859_d_n11, assign56440_e87859_d_n14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign56440_e87861;
        locals.var_t2_dn0 = assign56440_e87861_d_n0;
        locals.var_t2_dn2 = assign56440_e87861_d_n2;
        locals.var_t2_dn4 = assign56440_e87861_d_n4;
        locals.var_t2_dn5 = assign56440_e87861_d_n5;
        locals.var_t2_dn6 = assign56440_e87861_d_n6;
        locals.var_t2_dn7 = assign56440_e87861_d_n7;
        locals.var_t2_dn8 = assign56440_e87861_d_n8;
        locals.var_t2_dn9 = assign56440_e87861_d_n9;
        locals.var_t2_dn10 = assign56440_e87861_d_n10;
        locals.var_t2_dn11 = assign56440_e87861_d_n11;
        locals.var_t2_dn14 = assign56440_e87861_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign56450_e87876, assign56450_e87876_d_n0, assign56450_e87876_d_n2, assign56450_e87876_d_n4, assign56450_e87876_d_n5, assign56450_e87876_d_n6, assign56450_e87876_d_n7, assign56450_e87876_d_n8, assign56450_e87876_d_n9, assign56450_e87876_d_n10, assign56450_e87876_d_n11, assign56450_e87876_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign56450_e87873: f64 = (locals.var_t2 * locals.var_t1);
        let assign56450_e87874: f64 = (1.0 + assign56450_e87873);
        (assign56450_e87874, ((locals.var_t2_dn0 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn0)), ((locals.var_t2_dn2 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn2)), ((locals.var_t2_dn4 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn4)), ((locals.var_t2_dn5 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn5)), ((locals.var_t2_dn6 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn6)), ((locals.var_t2_dn7 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn7)), ((locals.var_t2_dn8 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn8)), ((locals.var_t2_dn9 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn9)), ((locals.var_t2_dn10 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn10)), ((locals.var_t2_dn11 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn11)), ((locals.var_t2_dn14 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign56450_e87876;
        locals.var_t3_dn0 = assign56450_e87876_d_n0;
        locals.var_t3_dn2 = assign56450_e87876_d_n2;
        locals.var_t3_dn4 = assign56450_e87876_d_n4;
        locals.var_t3_dn5 = assign56450_e87876_d_n5;
        locals.var_t3_dn6 = assign56450_e87876_d_n6;
        locals.var_t3_dn7 = assign56450_e87876_d_n7;
        locals.var_t3_dn8 = assign56450_e87876_d_n8;
        locals.var_t3_dn9 = assign56450_e87876_d_n9;
        locals.var_t3_dn10 = assign56450_e87876_d_n10;
        locals.var_t3_dn11 = assign56450_e87876_d_n11;
        locals.var_t3_dn14 = assign56450_e87876_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign56460_e87898, assign56460_e87898_d_n0, assign56460_e87898_d_n2, assign56460_e87898_d_n4, assign56460_e87898_d_n5, assign56460_e87898_d_n6, assign56460_e87898_d_n7, assign56460_e87898_d_n8, assign56460_e87898_d_n9, assign56460_e87898_d_n10, assign56460_e87898_d_n11, assign56460_e87898_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let (assign56460_e87896, assign56460_e87896_d_n0, assign56460_e87896_d_n2, assign56460_e87896_d_n4, assign56460_e87896_d_n5, assign56460_e87896_d_n6, assign56460_e87896_d_n7, assign56460_e87896_d_n8, assign56460_e87896_d_n9, assign56460_e87896_d_n10, assign56460_e87896_d_n11, assign56460_e87896_d_n14,) = {
            if (locals.var_t3 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign56460_e87892: f64 = (1.0 / p.p383);
                let assign56460_e87894: f64 = (assign56460_e87892 - 1.0);
                let assign56460_e87895: f64 = (locals.var_t3).powf(assign56460_e87894);
                (assign56460_e87895, if 0.0 == 0.0 && ((assign56460_e87894) as f64).is_finite() && ((assign56460_e87894) as f64).fract() == 0.0 { if assign56460_e87894 == 0.0 { 0.0 } else { (assign56460_e87894 * ((locals.var_t3).powf(assign56460_e87894 - 1.0) * locals.var_t3_dn0)) } } else { (assign56460_e87895 * (assign56460_e87894 * (locals.var_t3_dn0 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign56460_e87894) as f64).is_finite() && ((assign56460_e87894) as f64).fract() == 0.0 { if assign56460_e87894 == 0.0 { 0.0 } else { (assign56460_e87894 * ((locals.var_t3).powf(assign56460_e87894 - 1.0) * locals.var_t3_dn2)) } } else { (assign56460_e87895 * (assign56460_e87894 * (locals.var_t3_dn2 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign56460_e87894) as f64).is_finite() && ((assign56460_e87894) as f64).fract() == 0.0 { if assign56460_e87894 == 0.0 { 0.0 } else { (assign56460_e87894 * ((locals.var_t3).powf(assign56460_e87894 - 1.0) * locals.var_t3_dn4)) } } else { (assign56460_e87895 * (assign56460_e87894 * (locals.var_t3_dn4 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign56460_e87894) as f64).is_finite() && ((assign56460_e87894) as f64).fract() == 0.0 { if assign56460_e87894 == 0.0 { 0.0 } else { (assign56460_e87894 * ((locals.var_t3).powf(assign56460_e87894 - 1.0) * locals.var_t3_dn5)) } } else { (assign56460_e87895 * (assign56460_e87894 * (locals.var_t3_dn5 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign56460_e87894) as f64).is_finite() && ((assign56460_e87894) as f64).fract() == 0.0 { if assign56460_e87894 == 0.0 { 0.0 } else { (assign56460_e87894 * ((locals.var_t3).powf(assign56460_e87894 - 1.0) * locals.var_t3_dn6)) } } else { (assign56460_e87895 * (assign56460_e87894 * (locals.var_t3_dn6 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign56460_e87894) as f64).is_finite() && ((assign56460_e87894) as f64).fract() == 0.0 { if assign56460_e87894 == 0.0 { 0.0 } else { (assign56460_e87894 * ((locals.var_t3).powf(assign56460_e87894 - 1.0) * locals.var_t3_dn7)) } } else { (assign56460_e87895 * (assign56460_e87894 * (locals.var_t3_dn7 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign56460_e87894) as f64).is_finite() && ((assign56460_e87894) as f64).fract() == 0.0 { if assign56460_e87894 == 0.0 { 0.0 } else { (assign56460_e87894 * ((locals.var_t3).powf(assign56460_e87894 - 1.0) * locals.var_t3_dn8)) } } else { (assign56460_e87895 * (assign56460_e87894 * (locals.var_t3_dn8 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign56460_e87894) as f64).is_finite() && ((assign56460_e87894) as f64).fract() == 0.0 { if assign56460_e87894 == 0.0 { 0.0 } else { (assign56460_e87894 * ((locals.var_t3).powf(assign56460_e87894 - 1.0) * locals.var_t3_dn9)) } } else { (assign56460_e87895 * (assign56460_e87894 * (locals.var_t3_dn9 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign56460_e87894) as f64).is_finite() && ((assign56460_e87894) as f64).fract() == 0.0 { if assign56460_e87894 == 0.0 { 0.0 } else { (assign56460_e87894 * ((locals.var_t3).powf(assign56460_e87894 - 1.0) * locals.var_t3_dn10)) } } else { (assign56460_e87895 * (assign56460_e87894 * (locals.var_t3_dn10 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign56460_e87894) as f64).is_finite() && ((assign56460_e87894) as f64).fract() == 0.0 { if assign56460_e87894 == 0.0 { 0.0 } else { (assign56460_e87894 * ((locals.var_t3).powf(assign56460_e87894 - 1.0) * locals.var_t3_dn11)) } } else { (assign56460_e87895 * (assign56460_e87894 * (locals.var_t3_dn11 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign56460_e87894) as f64).is_finite() && ((assign56460_e87894) as f64).fract() == 0.0 { if assign56460_e87894 == 0.0 { 0.0 } else { (assign56460_e87894 * ((locals.var_t3).powf(assign56460_e87894 - 1.0) * locals.var_t3_dn14)) } } else { (assign56460_e87895 * (assign56460_e87894 * (locals.var_t3_dn14 / locals.var_t3))) },)
            }
        };
        (assign56460_e87896, assign56460_e87896_d_n0, assign56460_e87896_d_n2, assign56460_e87896_d_n4, assign56460_e87896_d_n5, assign56460_e87896_d_n6, assign56460_e87896_d_n7, assign56460_e87896_d_n8, assign56460_e87896_d_n9, assign56460_e87896_d_n10, assign56460_e87896_d_n11, assign56460_e87896_d_n14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign56460_e87898;
        locals.var_t4_dn0 = assign56460_e87898_d_n0;
        locals.var_t4_dn2 = assign56460_e87898_d_n2;
        locals.var_t4_dn4 = assign56460_e87898_d_n4;
        locals.var_t4_dn5 = assign56460_e87898_d_n5;
        locals.var_t4_dn6 = assign56460_e87898_d_n6;
        locals.var_t4_dn7 = assign56460_e87898_d_n7;
        locals.var_t4_dn8 = assign56460_e87898_d_n8;
        locals.var_t4_dn9 = assign56460_e87898_d_n9;
        locals.var_t4_dn10 = assign56460_e87898_d_n10;
        locals.var_t4_dn11 = assign56460_e87898_d_n11;
        locals.var_t4_dn14 = assign56460_e87898_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign56470_e87911, assign56470_e87911_d_n0, assign56470_e87911_d_n2, assign56470_e87911_d_n4, assign56470_e87911_d_n5, assign56470_e87911_d_n6, assign56470_e87911_d_n7, assign56470_e87911_d_n8, assign56470_e87911_d_n9, assign56470_e87911_d_n10, assign56470_e87911_d_n11, assign56470_e87911_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign56470_e87909: f64 = (locals.var_t4 * locals.var_t3);
        (assign56470_e87909, ((locals.var_t4_dn0 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn0)), ((locals.var_t4_dn2 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn2)), ((locals.var_t4_dn4 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn4)), ((locals.var_t4_dn5 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn5)), ((locals.var_t4_dn6 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn6)), ((locals.var_t4_dn7 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn7)), ((locals.var_t4_dn8 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn8)), ((locals.var_t4_dn9 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn9)), ((locals.var_t4_dn10 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn10)), ((locals.var_t4_dn11 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn11)), ((locals.var_t4_dn14 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign56470_e87911;
        locals.var_t6_dn0 = assign56470_e87911_d_n0;
        locals.var_t6_dn2 = assign56470_e87911_d_n2;
        locals.var_t6_dn4 = assign56470_e87911_d_n4;
        locals.var_t6_dn5 = assign56470_e87911_d_n5;
        locals.var_t6_dn6 = assign56470_e87911_d_n6;
        locals.var_t6_dn7 = assign56470_e87911_d_n7;
        locals.var_t6_dn8 = assign56470_e87911_d_n8;
        locals.var_t6_dn9 = assign56470_e87911_d_n9;
        locals.var_t6_dn10 = assign56470_e87911_d_n10;
        locals.var_t6_dn11 = assign56470_e87911_d_n11;
        locals.var_t6_dn14 = assign56470_e87911_d_n14;
        locals.var_t6_rv = 0.0;

        let assign56480_e87916: f64 = (locals.var_uc_depleak * 0.5);
        let assign56480_e87917: f64 = (locals.var_uc_depleak - assign56480_e87916);
        let assign56480_e87921: f64 = (locals.var_uc_depleak * 0.5);
        let assign56480_e87924: f64 = if ((locals.var_vdsorg > assign56480_e87917) && (assign56480_e87921 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1415 = assign56480_e87924;
        locals.var_guard1415_rv = 0.0;

        let (assign56490_e87943, assign56490_e87943_d_n0, assign56490_e87943_d_n2, assign56490_e87943_d_n4, assign56490_e87943_d_n5, assign56490_e87943_d_n6, assign56490_e87943_d_n7, assign56490_e87943_d_n8, assign56490_e87943_d_n9, assign56490_e87943_d_n10, assign56490_e87943_d_n11, assign56490_e87943_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1415 != 0.0)) {
        let assign56490_e87937: f64 = (locals.var_vdsorg - locals.var_uc_depleak);
        let assign56490_e87940: f64 = (locals.var_uc_depleak * 0.5);
        let assign56490_e87941: f64 = (assign56490_e87937 + assign56490_e87940);
        (assign56490_e87941, ((locals.var_vdsorg_dn0 - locals.var_uc_depleak_dn0) + (locals.var_uc_depleak_dn0 * 0.5)), ((locals.var_vdsorg_dn2 - locals.var_uc_depleak_dn2) + (locals.var_uc_depleak_dn2 * 0.5)), ((locals.var_vdsorg_dn4 - locals.var_uc_depleak_dn4) + (locals.var_uc_depleak_dn4 * 0.5)), ((locals.var_vdsorg_dn5 - locals.var_uc_depleak_dn5) + (locals.var_uc_depleak_dn5 * 0.5)), ((locals.var_vdsorg_dn6 - locals.var_uc_depleak_dn6) + (locals.var_uc_depleak_dn6 * 0.5)), ((locals.var_vdsorg_dn7 - locals.var_uc_depleak_dn7) + (locals.var_uc_depleak_dn7 * 0.5)), ((locals.var_vdsorg_dn8 - locals.var_uc_depleak_dn8) + (locals.var_uc_depleak_dn8 * 0.5)), ((locals.var_vdsorg_dn9 - locals.var_uc_depleak_dn9) + (locals.var_uc_depleak_dn9 * 0.5)), ((locals.var_vdsorg_dn10 - locals.var_uc_depleak_dn10) + (locals.var_uc_depleak_dn10 * 0.5)), ((locals.var_vdsorg_dn11 - locals.var_uc_depleak_dn11) + (locals.var_uc_depleak_dn11 * 0.5)), ((locals.var_vdsorg_dn14 - locals.var_uc_depleak_dn14) + (locals.var_uc_depleak_dn14 * 0.5)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign56490_e87943;
        locals.var_tmf1_dn0 = assign56490_e87943_d_n0;
        locals.var_tmf1_dn2 = assign56490_e87943_d_n2;
        locals.var_tmf1_dn4 = assign56490_e87943_d_n4;
        locals.var_tmf1_dn5 = assign56490_e87943_d_n5;
        locals.var_tmf1_dn6 = assign56490_e87943_d_n6;
        locals.var_tmf1_dn7 = assign56490_e87943_d_n7;
        locals.var_tmf1_dn8 = assign56490_e87943_d_n8;
        locals.var_tmf1_dn9 = assign56490_e87943_d_n9;
        locals.var_tmf1_dn10 = assign56490_e87943_d_n10;
        locals.var_tmf1_dn11 = assign56490_e87943_d_n11;
        locals.var_tmf1_dn14 = assign56490_e87943_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign56500_e87958, assign56500_e87958_d_n0, assign56500_e87958_d_n2, assign56500_e87958_d_n4, assign56500_e87958_d_n5, assign56500_e87958_d_n6, assign56500_e87958_d_n7, assign56500_e87958_d_n8, assign56500_e87958_d_n9, assign56500_e87958_d_n10, assign56500_e87958_d_n11, assign56500_e87958_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1415 != 0.0)) {
        let assign56500_e87956: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign56500_e87956, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign56500_e87958;
        locals.var_x2_dn0 = assign56500_e87958_d_n0;
        locals.var_x2_dn2 = assign56500_e87958_d_n2;
        locals.var_x2_dn4 = assign56500_e87958_d_n4;
        locals.var_x2_dn5 = assign56500_e87958_d_n5;
        locals.var_x2_dn6 = assign56500_e87958_d_n6;
        locals.var_x2_dn7 = assign56500_e87958_d_n7;
        locals.var_x2_dn8 = assign56500_e87958_d_n8;
        locals.var_x2_dn9 = assign56500_e87958_d_n9;
        locals.var_x2_dn10 = assign56500_e87958_d_n10;
        locals.var_x2_dn11 = assign56500_e87958_d_n11;
        locals.var_x2_dn14 = assign56500_e87958_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign56510_e87977, assign56510_e87977_d_n0, assign56510_e87977_d_n2, assign56510_e87977_d_n4, assign56510_e87977_d_n5, assign56510_e87977_d_n6, assign56510_e87977_d_n7, assign56510_e87977_d_n8, assign56510_e87977_d_n9, assign56510_e87977_d_n10, assign56510_e87977_d_n11, assign56510_e87977_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1415 != 0.0)) {
        let assign56510_e87971: f64 = (locals.var_uc_depleak * 0.5);
        let assign56510_e87974: f64 = (locals.var_uc_depleak * 0.5);
        let assign56510_e87975: f64 = (assign56510_e87971 * assign56510_e87974);
        (assign56510_e87975, (((locals.var_uc_depleak_dn0 * 0.5) * assign56510_e87974) + (assign56510_e87971 * (locals.var_uc_depleak_dn0 * 0.5))), (((locals.var_uc_depleak_dn2 * 0.5) * assign56510_e87974) + (assign56510_e87971 * (locals.var_uc_depleak_dn2 * 0.5))), (((locals.var_uc_depleak_dn4 * 0.5) * assign56510_e87974) + (assign56510_e87971 * (locals.var_uc_depleak_dn4 * 0.5))), (((locals.var_uc_depleak_dn5 * 0.5) * assign56510_e87974) + (assign56510_e87971 * (locals.var_uc_depleak_dn5 * 0.5))), (((locals.var_uc_depleak_dn6 * 0.5) * assign56510_e87974) + (assign56510_e87971 * (locals.var_uc_depleak_dn6 * 0.5))), (((locals.var_uc_depleak_dn7 * 0.5) * assign56510_e87974) + (assign56510_e87971 * (locals.var_uc_depleak_dn7 * 0.5))), (((locals.var_uc_depleak_dn8 * 0.5) * assign56510_e87974) + (assign56510_e87971 * (locals.var_uc_depleak_dn8 * 0.5))), (((locals.var_uc_depleak_dn9 * 0.5) * assign56510_e87974) + (assign56510_e87971 * (locals.var_uc_depleak_dn9 * 0.5))), (((locals.var_uc_depleak_dn10 * 0.5) * assign56510_e87974) + (assign56510_e87971 * (locals.var_uc_depleak_dn10 * 0.5))), (((locals.var_uc_depleak_dn11 * 0.5) * assign56510_e87974) + (assign56510_e87971 * (locals.var_uc_depleak_dn11 * 0.5))), (((locals.var_uc_depleak_dn14 * 0.5) * assign56510_e87974) + (assign56510_e87971 * (locals.var_uc_depleak_dn14 * 0.5))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign56510_e87977;
        locals.var_xmax2_dn0 = assign56510_e87977_d_n0;
        locals.var_xmax2_dn2 = assign56510_e87977_d_n2;
        locals.var_xmax2_dn4 = assign56510_e87977_d_n4;
        locals.var_xmax2_dn5 = assign56510_e87977_d_n5;
        locals.var_xmax2_dn6 = assign56510_e87977_d_n6;
        locals.var_xmax2_dn7 = assign56510_e87977_d_n7;
        locals.var_xmax2_dn8 = assign56510_e87977_d_n8;
        locals.var_xmax2_dn9 = assign56510_e87977_d_n9;
        locals.var_xmax2_dn10 = assign56510_e87977_d_n10;
        locals.var_xmax2_dn11 = assign56510_e87977_d_n11;
        locals.var_xmax2_dn14 = assign56510_e87977_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign56520_e87990, assign56520_e87990_d_n0, assign56520_e87990_d_n2, assign56520_e87990_d_n4, assign56520_e87990_d_n5, assign56520_e87990_d_n6, assign56520_e87990_d_n7, assign56520_e87990_d_n8, assign56520_e87990_d_n9, assign56520_e87990_d_n10, assign56520_e87990_d_n11, assign56520_e87990_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1415 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign56520_e87990;
        locals.var_xp_dn0 = assign56520_e87990_d_n0;
        locals.var_xp_dn2 = assign56520_e87990_d_n2;
        locals.var_xp_dn4 = assign56520_e87990_d_n4;
        locals.var_xp_dn5 = assign56520_e87990_d_n5;
        locals.var_xp_dn6 = assign56520_e87990_d_n6;
        locals.var_xp_dn7 = assign56520_e87990_d_n7;
        locals.var_xp_dn8 = assign56520_e87990_d_n8;
        locals.var_xp_dn9 = assign56520_e87990_d_n9;
        locals.var_xp_dn10 = assign56520_e87990_d_n10;
        locals.var_xp_dn11 = assign56520_e87990_d_n11;
        locals.var_xp_dn14 = assign56520_e87990_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign56530_e88003, assign56530_e88003_d_n0, assign56530_e88003_d_n2, assign56530_e88003_d_n4, assign56530_e88003_d_n5, assign56530_e88003_d_n6, assign56530_e88003_d_n7, assign56530_e88003_d_n8, assign56530_e88003_d_n9, assign56530_e88003_d_n10, assign56530_e88003_d_n11, assign56530_e88003_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1415 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign56530_e88003;
        locals.var_xmp_dn0 = assign56530_e88003_d_n0;
        locals.var_xmp_dn2 = assign56530_e88003_d_n2;
        locals.var_xmp_dn4 = assign56530_e88003_d_n4;
        locals.var_xmp_dn5 = assign56530_e88003_d_n5;
        locals.var_xmp_dn6 = assign56530_e88003_d_n6;
        locals.var_xmp_dn7 = assign56530_e88003_d_n7;
        locals.var_xmp_dn8 = assign56530_e88003_d_n8;
        locals.var_xmp_dn9 = assign56530_e88003_d_n9;
        locals.var_xmp_dn10 = assign56530_e88003_d_n10;
        locals.var_xmp_dn11 = assign56530_e88003_d_n11;
        locals.var_xmp_dn14 = assign56530_e88003_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign56540_e88016,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1415 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign56540_e88016;
        locals.var_m0_rv = 0.0;

        let (assign56550_e88029,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1415 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign56550_e88029;
        locals.var_mm_rv = 0.0;

        let (assign56560_e88042, assign56560_e88042_d_n0, assign56560_e88042_d_n2, assign56560_e88042_d_n4, assign56560_e88042_d_n5, assign56560_e88042_d_n6, assign56560_e88042_d_n7, assign56560_e88042_d_n8, assign56560_e88042_d_n9, assign56560_e88042_d_n10, assign56560_e88042_d_n11, assign56560_e88042_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1415 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign56560_e88042;
        locals.var_arg_dn0 = assign56560_e88042_d_n0;
        locals.var_arg_dn2 = assign56560_e88042_d_n2;
        locals.var_arg_dn4 = assign56560_e88042_d_n4;
        locals.var_arg_dn5 = assign56560_e88042_d_n5;
        locals.var_arg_dn6 = assign56560_e88042_d_n6;
        locals.var_arg_dn7 = assign56560_e88042_d_n7;
        locals.var_arg_dn8 = assign56560_e88042_d_n8;
        locals.var_arg_dn9 = assign56560_e88042_d_n9;
        locals.var_arg_dn10 = assign56560_e88042_d_n10;
        locals.var_arg_dn11 = assign56560_e88042_d_n11;
        locals.var_arg_dn14 = assign56560_e88042_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign56570_e88055, assign56570_e88055_d_n0, assign56570_e88055_d_n2, assign56570_e88055_d_n4, assign56570_e88055_d_n5, assign56570_e88055_d_n6, assign56570_e88055_d_n7, assign56570_e88055_d_n8, assign56570_e88055_d_n9, assign56570_e88055_d_n10, assign56570_e88055_d_n11, assign56570_e88055_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1415 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign56570_e88055;
        locals.var_dnm_dn0 = assign56570_e88055_d_n0;
        locals.var_dnm_dn2 = assign56570_e88055_d_n2;
        locals.var_dnm_dn4 = assign56570_e88055_d_n4;
        locals.var_dnm_dn5 = assign56570_e88055_d_n5;
        locals.var_dnm_dn6 = assign56570_e88055_d_n6;
        locals.var_dnm_dn7 = assign56570_e88055_d_n7;
        locals.var_dnm_dn8 = assign56570_e88055_d_n8;
        locals.var_dnm_dn9 = assign56570_e88055_d_n9;
        locals.var_dnm_dn10 = assign56570_e88055_d_n10;
        locals.var_dnm_dn11 = assign56570_e88055_d_n11;
        locals.var_dnm_dn14 = assign56570_e88055_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign56580_e88070, assign56580_e88070_d_n0, assign56580_e88070_d_n2, assign56580_e88070_d_n4, assign56580_e88070_d_n5, assign56580_e88070_d_n6, assign56580_e88070_d_n7, assign56580_e88070_d_n8, assign56580_e88070_d_n9, assign56580_e88070_d_n10, assign56580_e88070_d_n11, assign56580_e88070_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1415 != 0.0)) {
        let assign56580_e88068: f64 = (locals.var_xp * locals.var_x2);
        (assign56580_e88068, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign56580_e88070;
        locals.var_xp_dn0 = assign56580_e88070_d_n0;
        locals.var_xp_dn2 = assign56580_e88070_d_n2;
        locals.var_xp_dn4 = assign56580_e88070_d_n4;
        locals.var_xp_dn5 = assign56580_e88070_d_n5;
        locals.var_xp_dn6 = assign56580_e88070_d_n6;
        locals.var_xp_dn7 = assign56580_e88070_d_n7;
        locals.var_xp_dn8 = assign56580_e88070_d_n8;
        locals.var_xp_dn9 = assign56580_e88070_d_n9;
        locals.var_xp_dn10 = assign56580_e88070_d_n10;
        locals.var_xp_dn11 = assign56580_e88070_d_n11;
        locals.var_xp_dn14 = assign56580_e88070_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign56590_e88085, assign56590_e88085_d_n0, assign56590_e88085_d_n2, assign56590_e88085_d_n4, assign56590_e88085_d_n5, assign56590_e88085_d_n6, assign56590_e88085_d_n7, assign56590_e88085_d_n8, assign56590_e88085_d_n9, assign56590_e88085_d_n10, assign56590_e88085_d_n11, assign56590_e88085_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1415 != 0.0)) {
        let assign56590_e88083: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign56590_e88083, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign56590_e88085;
        locals.var_xmp_dn0 = assign56590_e88085_d_n0;
        locals.var_xmp_dn2 = assign56590_e88085_d_n2;
        locals.var_xmp_dn4 = assign56590_e88085_d_n4;
        locals.var_xmp_dn5 = assign56590_e88085_d_n5;
        locals.var_xmp_dn6 = assign56590_e88085_d_n6;
        locals.var_xmp_dn7 = assign56590_e88085_d_n7;
        locals.var_xmp_dn8 = assign56590_e88085_d_n8;
        locals.var_xmp_dn9 = assign56590_e88085_d_n9;
        locals.var_xmp_dn10 = assign56590_e88085_d_n10;
        locals.var_xmp_dn11 = assign56590_e88085_d_n11;
        locals.var_xmp_dn14 = assign56590_e88085_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign56600_e88100, assign56600_e88100_d_n0, assign56600_e88100_d_n2, assign56600_e88100_d_n4, assign56600_e88100_d_n5, assign56600_e88100_d_n6, assign56600_e88100_d_n7, assign56600_e88100_d_n8, assign56600_e88100_d_n9, assign56600_e88100_d_n10, assign56600_e88100_d_n11, assign56600_e88100_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1415 != 0.0)) {
        let assign56600_e88098: f64 = (locals.var_xp * locals.var_x2);
        (assign56600_e88098, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign56600_e88100;
        locals.var_xp_dn0 = assign56600_e88100_d_n0;
        locals.var_xp_dn2 = assign56600_e88100_d_n2;
        locals.var_xp_dn4 = assign56600_e88100_d_n4;
        locals.var_xp_dn5 = assign56600_e88100_d_n5;
        locals.var_xp_dn6 = assign56600_e88100_d_n6;
        locals.var_xp_dn7 = assign56600_e88100_d_n7;
        locals.var_xp_dn8 = assign56600_e88100_d_n8;
        locals.var_xp_dn9 = assign56600_e88100_d_n9;
        locals.var_xp_dn10 = assign56600_e88100_d_n10;
        locals.var_xp_dn11 = assign56600_e88100_d_n11;
        locals.var_xp_dn14 = assign56600_e88100_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign56610_e88115, assign56610_e88115_d_n0, assign56610_e88115_d_n2, assign56610_e88115_d_n4, assign56610_e88115_d_n5, assign56610_e88115_d_n6, assign56610_e88115_d_n7, assign56610_e88115_d_n8, assign56610_e88115_d_n9, assign56610_e88115_d_n10, assign56610_e88115_d_n11, assign56610_e88115_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1415 != 0.0)) {
        let assign56610_e88113: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign56610_e88113, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign56610_e88115;
        locals.var_xmp_dn0 = assign56610_e88115_d_n0;
        locals.var_xmp_dn2 = assign56610_e88115_d_n2;
        locals.var_xmp_dn4 = assign56610_e88115_d_n4;
        locals.var_xmp_dn5 = assign56610_e88115_d_n5;
        locals.var_xmp_dn6 = assign56610_e88115_d_n6;
        locals.var_xmp_dn7 = assign56610_e88115_d_n7;
        locals.var_xmp_dn8 = assign56610_e88115_d_n8;
        locals.var_xmp_dn9 = assign56610_e88115_d_n9;
        locals.var_xmp_dn10 = assign56610_e88115_d_n10;
        locals.var_xmp_dn11 = assign56610_e88115_d_n11;
        locals.var_xmp_dn14 = assign56610_e88115_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign56620_e88130, assign56620_e88130_d_n0, assign56620_e88130_d_n2, assign56620_e88130_d_n4, assign56620_e88130_d_n5, assign56620_e88130_d_n6, assign56620_e88130_d_n7, assign56620_e88130_d_n8, assign56620_e88130_d_n9, assign56620_e88130_d_n10, assign56620_e88130_d_n11, assign56620_e88130_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1415 != 0.0)) {
        let assign56620_e88128: f64 = (locals.var_xp + locals.var_xmp);
        (assign56620_e88128, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign56620_e88130;
        locals.var_arg_dn0 = assign56620_e88130_d_n0;
        locals.var_arg_dn2 = assign56620_e88130_d_n2;
        locals.var_arg_dn4 = assign56620_e88130_d_n4;
        locals.var_arg_dn5 = assign56620_e88130_d_n5;
        locals.var_arg_dn6 = assign56620_e88130_d_n6;
        locals.var_arg_dn7 = assign56620_e88130_d_n7;
        locals.var_arg_dn8 = assign56620_e88130_d_n8;
        locals.var_arg_dn9 = assign56620_e88130_d_n9;
        locals.var_arg_dn10 = assign56620_e88130_d_n10;
        locals.var_arg_dn11 = assign56620_e88130_d_n11;
        locals.var_arg_dn14 = assign56620_e88130_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign56630_e88143, assign56630_e88143_d_n0, assign56630_e88143_d_n2, assign56630_e88143_d_n4, assign56630_e88143_d_n5, assign56630_e88143_d_n6, assign56630_e88143_d_n7, assign56630_e88143_d_n8, assign56630_e88143_d_n9, assign56630_e88143_d_n10, assign56630_e88143_d_n11, assign56630_e88143_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1415 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign56630_e88143;
        locals.var_dnm_dn0 = assign56630_e88143_d_n0;
        locals.var_dnm_dn2 = assign56630_e88143_d_n2;
        locals.var_dnm_dn4 = assign56630_e88143_d_n4;
        locals.var_dnm_dn5 = assign56630_e88143_d_n5;
        locals.var_dnm_dn6 = assign56630_e88143_d_n6;
        locals.var_dnm_dn7 = assign56630_e88143_d_n7;
        locals.var_dnm_dn8 = assign56630_e88143_d_n8;
        locals.var_dnm_dn9 = assign56630_e88143_d_n9;
        locals.var_dnm_dn10 = assign56630_e88143_d_n10;
        locals.var_dnm_dn11 = assign56630_e88143_d_n11;
        locals.var_dnm_dn14 = assign56630_e88143_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign56640_e88158: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1416 = assign56640_e88158;
        locals.var_guard1416_rv = 0.0;

        let assign56650_e88161: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1417 = assign56650_e88161;
        locals.var_guard1417_rv = 0.0;

        let (assign56660_e88178,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1415 != 0.0)) && (locals.var_guard1416 != 0.0)) && (locals.var_guard1417 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign56660_e88178;
        locals.var_mm_rv = 0.0;

        let assign56670_e88181: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1418 = assign56670_e88181;
        locals.var_guard1418_rv = 0.0;

        let (assign56680_e88201,) = {
    if ((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1415 != 0.0)) && (locals.var_guard1416 != 0.0)) && (locals.var_guard1417 == 0.0)) && (locals.var_guard1418 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign56680_e88201;
        locals.var_mm_rv = 0.0;

        let assign56690_e88204: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1419 = assign56690_e88204;
        locals.var_guard1419_rv = 0.0;

        let (assign56700_e88227,) = {
    if (((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1415 != 0.0)) && (locals.var_guard1416 != 0.0)) && (locals.var_guard1417 == 0.0)) && (locals.var_guard1418 == 0.0)) && (locals.var_guard1419 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign56700_e88227;
        locals.var_mm_rv = 0.0;

        let assign56710_e88230: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1420 = assign56710_e88230;
        locals.var_guard1420_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_207(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign56720_e88256,) = {
    if ((((((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1415 != 0.0)) && (locals.var_guard1416 != 0.0)) && (locals.var_guard1417 == 0.0)) && (locals.var_guard1418 == 0.0)) && (locals.var_guard1419 == 0.0)) && (locals.var_guard1420 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign56720_e88256;
        locals.var_mm_rv = 0.0;

        let (assign56730_e88271,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1415 != 0.0)) && (locals.var_guard1416 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign56730_e88271;
        locals.var_m0_rv = 0.0;

        let mut assign56740_loop_guard: usize = 0;
        while {
            let assign56740_cond_e88287: f64 = if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1415 != 0.0)) && (locals.var_guard1416 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign56740_cond_e88287 != 0.0
        } {
            assign56740_loop_guard += 1;
            assert!(assign56740_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign56740_body0_e88303, assign56740_body0_e88303_d_n0, assign56740_body0_e88303_d_n2, assign56740_body0_e88303_d_n4, assign56740_body0_e88303_d_n5, assign56740_body0_e88303_d_n6, assign56740_body0_e88303_d_n7, assign56740_body0_e88303_d_n8, assign56740_body0_e88303_d_n9, assign56740_body0_e88303_d_n10, assign56740_body0_e88303_d_n11, assign56740_body0_e88303_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1415 != 0.0)) && (locals.var_guard1416 != 0.0)) {
        let assign56740_body0_e88301: f64 = (locals.var_dnm).sqrt();
        (assign56740_body0_e88301, (locals.var_dnm_dn0 / (2.0 * assign56740_body0_e88301)), (locals.var_dnm_dn2 / (2.0 * assign56740_body0_e88301)), (locals.var_dnm_dn4 / (2.0 * assign56740_body0_e88301)), (locals.var_dnm_dn5 / (2.0 * assign56740_body0_e88301)), (locals.var_dnm_dn6 / (2.0 * assign56740_body0_e88301)), (locals.var_dnm_dn7 / (2.0 * assign56740_body0_e88301)), (locals.var_dnm_dn8 / (2.0 * assign56740_body0_e88301)), (locals.var_dnm_dn9 / (2.0 * assign56740_body0_e88301)), (locals.var_dnm_dn10 / (2.0 * assign56740_body0_e88301)), (locals.var_dnm_dn11 / (2.0 * assign56740_body0_e88301)), (locals.var_dnm_dn14 / (2.0 * assign56740_body0_e88301)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign56740_body0_e88303;
            locals.var_dnm_dn0 = assign56740_body0_e88303_d_n0;
            locals.var_dnm_dn2 = assign56740_body0_e88303_d_n2;
            locals.var_dnm_dn4 = assign56740_body0_e88303_d_n4;
            locals.var_dnm_dn5 = assign56740_body0_e88303_d_n5;
            locals.var_dnm_dn6 = assign56740_body0_e88303_d_n6;
            locals.var_dnm_dn7 = assign56740_body0_e88303_d_n7;
            locals.var_dnm_dn8 = assign56740_body0_e88303_d_n8;
            locals.var_dnm_dn9 = assign56740_body0_e88303_d_n9;
            locals.var_dnm_dn10 = assign56740_body0_e88303_d_n10;
            locals.var_dnm_dn11 = assign56740_body0_e88303_d_n11;
            locals.var_dnm_dn14 = assign56740_body0_e88303_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign56740_body1_e88320,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1415 != 0.0)) && (locals.var_guard1416 != 0.0)) {
        let assign56740_body1_e88318: f64 = (locals.var_m0 + 1.0);
        (assign56740_body1_e88318,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign56740_body1_e88320;
            locals.var_m0_rv = 0.0;
        }

        let (assign56750_e88347, assign56750_e88347_d_n0, assign56750_e88347_d_n2, assign56750_e88347_d_n4, assign56750_e88347_d_n5, assign56750_e88347_d_n6, assign56750_e88347_d_n7, assign56750_e88347_d_n8, assign56750_e88347_d_n9, assign56750_e88347_d_n10, assign56750_e88347_d_n11, assign56750_e88347_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1415 != 0.0)) && (locals.var_guard1416 == 0.0)) {
        let (assign56750_e88345, assign56750_e88345_d_n0, assign56750_e88345_d_n2, assign56750_e88345_d_n4, assign56750_e88345_d_n5, assign56750_e88345_d_n6, assign56750_e88345_d_n7, assign56750_e88345_d_n8, assign56750_e88345_d_n9, assign56750_e88345_d_n10, assign56750_e88345_d_n11, assign56750_e88345_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign56750_e88342: f64 = (2.0 * 2.0);
                let assign56750_e88343: f64 = (1.0 / assign56750_e88342);
                let assign56750_e88344: f64 = (locals.var_dnm).powf(assign56750_e88343);
                (assign56750_e88344, if 0.0 == 0.0 && ((assign56750_e88343) as f64).is_finite() && ((assign56750_e88343) as f64).fract() == 0.0 { if assign56750_e88343 == 0.0 { 0.0 } else { (assign56750_e88343 * ((locals.var_dnm).powf(assign56750_e88343 - 1.0) * locals.var_dnm_dn0)) } } else { (assign56750_e88344 * (assign56750_e88343 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56750_e88343) as f64).is_finite() && ((assign56750_e88343) as f64).fract() == 0.0 { if assign56750_e88343 == 0.0 { 0.0 } else { (assign56750_e88343 * ((locals.var_dnm).powf(assign56750_e88343 - 1.0) * locals.var_dnm_dn2)) } } else { (assign56750_e88344 * (assign56750_e88343 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56750_e88343) as f64).is_finite() && ((assign56750_e88343) as f64).fract() == 0.0 { if assign56750_e88343 == 0.0 { 0.0 } else { (assign56750_e88343 * ((locals.var_dnm).powf(assign56750_e88343 - 1.0) * locals.var_dnm_dn4)) } } else { (assign56750_e88344 * (assign56750_e88343 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56750_e88343) as f64).is_finite() && ((assign56750_e88343) as f64).fract() == 0.0 { if assign56750_e88343 == 0.0 { 0.0 } else { (assign56750_e88343 * ((locals.var_dnm).powf(assign56750_e88343 - 1.0) * locals.var_dnm_dn5)) } } else { (assign56750_e88344 * (assign56750_e88343 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56750_e88343) as f64).is_finite() && ((assign56750_e88343) as f64).fract() == 0.0 { if assign56750_e88343 == 0.0 { 0.0 } else { (assign56750_e88343 * ((locals.var_dnm).powf(assign56750_e88343 - 1.0) * locals.var_dnm_dn6)) } } else { (assign56750_e88344 * (assign56750_e88343 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56750_e88343) as f64).is_finite() && ((assign56750_e88343) as f64).fract() == 0.0 { if assign56750_e88343 == 0.0 { 0.0 } else { (assign56750_e88343 * ((locals.var_dnm).powf(assign56750_e88343 - 1.0) * locals.var_dnm_dn7)) } } else { (assign56750_e88344 * (assign56750_e88343 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56750_e88343) as f64).is_finite() && ((assign56750_e88343) as f64).fract() == 0.0 { if assign56750_e88343 == 0.0 { 0.0 } else { (assign56750_e88343 * ((locals.var_dnm).powf(assign56750_e88343 - 1.0) * locals.var_dnm_dn8)) } } else { (assign56750_e88344 * (assign56750_e88343 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56750_e88343) as f64).is_finite() && ((assign56750_e88343) as f64).fract() == 0.0 { if assign56750_e88343 == 0.0 { 0.0 } else { (assign56750_e88343 * ((locals.var_dnm).powf(assign56750_e88343 - 1.0) * locals.var_dnm_dn9)) } } else { (assign56750_e88344 * (assign56750_e88343 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56750_e88343) as f64).is_finite() && ((assign56750_e88343) as f64).fract() == 0.0 { if assign56750_e88343 == 0.0 { 0.0 } else { (assign56750_e88343 * ((locals.var_dnm).powf(assign56750_e88343 - 1.0) * locals.var_dnm_dn10)) } } else { (assign56750_e88344 * (assign56750_e88343 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56750_e88343) as f64).is_finite() && ((assign56750_e88343) as f64).fract() == 0.0 { if assign56750_e88343 == 0.0 { 0.0 } else { (assign56750_e88343 * ((locals.var_dnm).powf(assign56750_e88343 - 1.0) * locals.var_dnm_dn11)) } } else { (assign56750_e88344 * (assign56750_e88343 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56750_e88343) as f64).is_finite() && ((assign56750_e88343) as f64).fract() == 0.0 { if assign56750_e88343 == 0.0 { 0.0 } else { (assign56750_e88343 * ((locals.var_dnm).powf(assign56750_e88343 - 1.0) * locals.var_dnm_dn14)) } } else { (assign56750_e88344 * (assign56750_e88343 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign56750_e88345, assign56750_e88345_d_n0, assign56750_e88345_d_n2, assign56750_e88345_d_n4, assign56750_e88345_d_n5, assign56750_e88345_d_n6, assign56750_e88345_d_n7, assign56750_e88345_d_n8, assign56750_e88345_d_n9, assign56750_e88345_d_n10, assign56750_e88345_d_n11, assign56750_e88345_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign56750_e88347;
        locals.var_dnm_dn0 = assign56750_e88347_d_n0;
        locals.var_dnm_dn2 = assign56750_e88347_d_n2;
        locals.var_dnm_dn4 = assign56750_e88347_d_n4;
        locals.var_dnm_dn5 = assign56750_e88347_d_n5;
        locals.var_dnm_dn6 = assign56750_e88347_d_n6;
        locals.var_dnm_dn7 = assign56750_e88347_d_n7;
        locals.var_dnm_dn8 = assign56750_e88347_d_n8;
        locals.var_dnm_dn9 = assign56750_e88347_d_n9;
        locals.var_dnm_dn10 = assign56750_e88347_d_n10;
        locals.var_dnm_dn11 = assign56750_e88347_d_n11;
        locals.var_dnm_dn14 = assign56750_e88347_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign56760_e88362, assign56760_e88362_d_n0, assign56760_e88362_d_n2, assign56760_e88362_d_n4, assign56760_e88362_d_n5, assign56760_e88362_d_n6, assign56760_e88362_d_n7, assign56760_e88362_d_n8, assign56760_e88362_d_n9, assign56760_e88362_d_n10, assign56760_e88362_d_n11, assign56760_e88362_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1415 != 0.0)) {
        let assign56760_e88360: f64 = (1.0 / locals.var_dnm);
        (assign56760_e88360, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign56760_e88362;
        locals.var_dnm_dn0 = assign56760_e88362_d_n0;
        locals.var_dnm_dn2 = assign56760_e88362_d_n2;
        locals.var_dnm_dn4 = assign56760_e88362_d_n4;
        locals.var_dnm_dn5 = assign56760_e88362_d_n5;
        locals.var_dnm_dn6 = assign56760_e88362_d_n6;
        locals.var_dnm_dn7 = assign56760_e88362_d_n7;
        locals.var_dnm_dn8 = assign56760_e88362_d_n8;
        locals.var_dnm_dn9 = assign56760_e88362_d_n9;
        locals.var_dnm_dn10 = assign56760_e88362_d_n10;
        locals.var_dnm_dn11 = assign56760_e88362_d_n11;
        locals.var_dnm_dn14 = assign56760_e88362_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign56770_e88381, assign56770_e88381_d_n0, assign56770_e88381_d_n2, assign56770_e88381_d_n4, assign56770_e88381_d_n5, assign56770_e88381_d_n6, assign56770_e88381_d_n7, assign56770_e88381_d_n8, assign56770_e88381_d_n9, assign56770_e88381_d_n10, assign56770_e88381_d_n11, assign56770_e88381_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1415 != 0.0)) {
        let assign56770_e88376: f64 = (locals.var_uc_depleak * 0.5);
        let assign56770_e88377: f64 = (locals.var_tmf1 * assign56770_e88376);
        let assign56770_e88379: f64 = (assign56770_e88377 * locals.var_dnm);
        (assign56770_e88379, ((((locals.var_tmf1_dn0 * assign56770_e88376) + (locals.var_tmf1 * (locals.var_uc_depleak_dn0 * 0.5))) * locals.var_dnm) + (assign56770_e88377 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign56770_e88376) + (locals.var_tmf1 * (locals.var_uc_depleak_dn2 * 0.5))) * locals.var_dnm) + (assign56770_e88377 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * assign56770_e88376) + (locals.var_tmf1 * (locals.var_uc_depleak_dn4 * 0.5))) * locals.var_dnm) + (assign56770_e88377 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * assign56770_e88376) + (locals.var_tmf1 * (locals.var_uc_depleak_dn5 * 0.5))) * locals.var_dnm) + (assign56770_e88377 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * assign56770_e88376) + (locals.var_tmf1 * (locals.var_uc_depleak_dn6 * 0.5))) * locals.var_dnm) + (assign56770_e88377 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign56770_e88376) + (locals.var_tmf1 * (locals.var_uc_depleak_dn7 * 0.5))) * locals.var_dnm) + (assign56770_e88377 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * assign56770_e88376) + (locals.var_tmf1 * (locals.var_uc_depleak_dn8 * 0.5))) * locals.var_dnm) + (assign56770_e88377 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * assign56770_e88376) + (locals.var_tmf1 * (locals.var_uc_depleak_dn9 * 0.5))) * locals.var_dnm) + (assign56770_e88377 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * assign56770_e88376) + (locals.var_tmf1 * (locals.var_uc_depleak_dn10 * 0.5))) * locals.var_dnm) + (assign56770_e88377 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * assign56770_e88376) + (locals.var_tmf1 * (locals.var_uc_depleak_dn11 * 0.5))) * locals.var_dnm) + (assign56770_e88377 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * assign56770_e88376) + (locals.var_tmf1 * (locals.var_uc_depleak_dn14 * 0.5))) * locals.var_dnm) + (assign56770_e88377 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign56770_e88381;
        locals.var_tmf0_dn0 = assign56770_e88381_d_n0;
        locals.var_tmf0_dn2 = assign56770_e88381_d_n2;
        locals.var_tmf0_dn4 = assign56770_e88381_d_n4;
        locals.var_tmf0_dn5 = assign56770_e88381_d_n5;
        locals.var_tmf0_dn6 = assign56770_e88381_d_n6;
        locals.var_tmf0_dn7 = assign56770_e88381_d_n7;
        locals.var_tmf0_dn8 = assign56770_e88381_d_n8;
        locals.var_tmf0_dn9 = assign56770_e88381_d_n9;
        locals.var_tmf0_dn10 = assign56770_e88381_d_n10;
        locals.var_tmf0_dn11 = assign56770_e88381_d_n11;
        locals.var_tmf0_dn14 = assign56770_e88381_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign56780_e88402, assign56780_e88402_d_n0, assign56780_e88402_d_n2, assign56780_e88402_d_n4, assign56780_e88402_d_n5, assign56780_e88402_d_n6, assign56780_e88402_d_n7, assign56780_e88402_d_n8, assign56780_e88402_d_n9, assign56780_e88402_d_n10, assign56780_e88402_d_n11, assign56780_e88402_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1415 != 0.0)) {
        let assign56780_e88394: f64 = (locals.var_uc_depleak * 0.5);
        let assign56780_e88396: f64 = (assign56780_e88394 * locals.var_xmp);
        let assign56780_e88398: f64 = (assign56780_e88396 * locals.var_dnm);
        let assign56780_e88400: f64 = (assign56780_e88398 / locals.var_arg);
        (assign56780_e88400, ((((((((locals.var_uc_depleak_dn0 * 0.5) * locals.var_xmp) + (assign56780_e88394 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign56780_e88396 * locals.var_dnm_dn0)) * locals.var_arg) - (assign56780_e88398 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_uc_depleak_dn2 * 0.5) * locals.var_xmp) + (assign56780_e88394 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign56780_e88396 * locals.var_dnm_dn2)) * locals.var_arg) - (assign56780_e88398 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_uc_depleak_dn4 * 0.5) * locals.var_xmp) + (assign56780_e88394 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign56780_e88396 * locals.var_dnm_dn4)) * locals.var_arg) - (assign56780_e88398 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_uc_depleak_dn5 * 0.5) * locals.var_xmp) + (assign56780_e88394 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign56780_e88396 * locals.var_dnm_dn5)) * locals.var_arg) - (assign56780_e88398 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_uc_depleak_dn6 * 0.5) * locals.var_xmp) + (assign56780_e88394 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign56780_e88396 * locals.var_dnm_dn6)) * locals.var_arg) - (assign56780_e88398 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_uc_depleak_dn7 * 0.5) * locals.var_xmp) + (assign56780_e88394 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign56780_e88396 * locals.var_dnm_dn7)) * locals.var_arg) - (assign56780_e88398 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_uc_depleak_dn8 * 0.5) * locals.var_xmp) + (assign56780_e88394 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign56780_e88396 * locals.var_dnm_dn8)) * locals.var_arg) - (assign56780_e88398 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_uc_depleak_dn9 * 0.5) * locals.var_xmp) + (assign56780_e88394 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign56780_e88396 * locals.var_dnm_dn9)) * locals.var_arg) - (assign56780_e88398 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_uc_depleak_dn10 * 0.5) * locals.var_xmp) + (assign56780_e88394 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign56780_e88396 * locals.var_dnm_dn10)) * locals.var_arg) - (assign56780_e88398 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_uc_depleak_dn11 * 0.5) * locals.var_xmp) + (assign56780_e88394 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign56780_e88396 * locals.var_dnm_dn11)) * locals.var_arg) - (assign56780_e88398 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_uc_depleak_dn14 * 0.5) * locals.var_xmp) + (assign56780_e88394 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign56780_e88396 * locals.var_dnm_dn14)) * locals.var_arg) - (assign56780_e88398 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign56780_e88402;
        locals.var_t0_dn0 = assign56780_e88402_d_n0;
        locals.var_t0_dn2 = assign56780_e88402_d_n2;
        locals.var_t0_dn4 = assign56780_e88402_d_n4;
        locals.var_t0_dn5 = assign56780_e88402_d_n5;
        locals.var_t0_dn6 = assign56780_e88402_d_n6;
        locals.var_t0_dn7 = assign56780_e88402_d_n7;
        locals.var_t0_dn8 = assign56780_e88402_d_n8;
        locals.var_t0_dn9 = assign56780_e88402_d_n9;
        locals.var_t0_dn10 = assign56780_e88402_d_n10;
        locals.var_t0_dn11 = assign56780_e88402_d_n11;
        locals.var_t0_dn14 = assign56780_e88402_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign56790_e88421, assign56790_e88421_d_n0, assign56790_e88421_d_n2, assign56790_e88421_d_n4, assign56790_e88421_d_n5, assign56790_e88421_d_n6, assign56790_e88421_d_n7, assign56790_e88421_d_n8, assign56790_e88421_d_n9, assign56790_e88421_d_n10, assign56790_e88421_d_n11, assign56790_e88421_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1415 != 0.0)) {
        let assign56790_e88416: f64 = (locals.var_uc_depleak * 0.5);
        let assign56790_e88417: f64 = (locals.var_uc_depleak - assign56790_e88416);
        let assign56790_e88419: f64 = (assign56790_e88417 + locals.var_tmf0);
        (assign56790_e88419, ((locals.var_uc_depleak_dn0 - (locals.var_uc_depleak_dn0 * 0.5)) + locals.var_tmf0_dn0), ((locals.var_uc_depleak_dn2 - (locals.var_uc_depleak_dn2 * 0.5)) + locals.var_tmf0_dn2), ((locals.var_uc_depleak_dn4 - (locals.var_uc_depleak_dn4 * 0.5)) + locals.var_tmf0_dn4), ((locals.var_uc_depleak_dn5 - (locals.var_uc_depleak_dn5 * 0.5)) + locals.var_tmf0_dn5), ((locals.var_uc_depleak_dn6 - (locals.var_uc_depleak_dn6 * 0.5)) + locals.var_tmf0_dn6), ((locals.var_uc_depleak_dn7 - (locals.var_uc_depleak_dn7 * 0.5)) + locals.var_tmf0_dn7), ((locals.var_uc_depleak_dn8 - (locals.var_uc_depleak_dn8 * 0.5)) + locals.var_tmf0_dn8), ((locals.var_uc_depleak_dn9 - (locals.var_uc_depleak_dn9 * 0.5)) + locals.var_tmf0_dn9), ((locals.var_uc_depleak_dn10 - (locals.var_uc_depleak_dn10 * 0.5)) + locals.var_tmf0_dn10), ((locals.var_uc_depleak_dn11 - (locals.var_uc_depleak_dn11 * 0.5)) + locals.var_tmf0_dn11), ((locals.var_uc_depleak_dn14 - (locals.var_uc_depleak_dn14 * 0.5)) + locals.var_tmf0_dn14),)
    } else {
        (locals.var_vds_res0, locals.var_vds_res0_dn0, locals.var_vds_res0_dn2, locals.var_vds_res0_dn4, locals.var_vds_res0_dn5, locals.var_vds_res0_dn6, locals.var_vds_res0_dn7, locals.var_vds_res0_dn8, locals.var_vds_res0_dn9, locals.var_vds_res0_dn10, locals.var_vds_res0_dn11, locals.var_vds_res0_dn14,)
    }
};
        locals.var_vds_res0 = assign56790_e88421;
        locals.var_vds_res0_dn0 = assign56790_e88421_d_n0;
        locals.var_vds_res0_dn2 = assign56790_e88421_d_n2;
        locals.var_vds_res0_dn4 = assign56790_e88421_d_n4;
        locals.var_vds_res0_dn5 = assign56790_e88421_d_n5;
        locals.var_vds_res0_dn6 = assign56790_e88421_d_n6;
        locals.var_vds_res0_dn7 = assign56790_e88421_d_n7;
        locals.var_vds_res0_dn8 = assign56790_e88421_d_n8;
        locals.var_vds_res0_dn9 = assign56790_e88421_d_n9;
        locals.var_vds_res0_dn10 = assign56790_e88421_d_n10;
        locals.var_vds_res0_dn11 = assign56790_e88421_d_n11;
        locals.var_vds_res0_dn14 = assign56790_e88421_d_n14;
        locals.var_vds_res0_rv = 0.0;

        let (assign56800_e88434, assign56800_e88434_d_n0, assign56800_e88434_d_n2, assign56800_e88434_d_n4, assign56800_e88434_d_n5, assign56800_e88434_d_n6, assign56800_e88434_d_n7, assign56800_e88434_d_n8, assign56800_e88434_d_n9, assign56800_e88434_d_n10, assign56800_e88434_d_n11, assign56800_e88434_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1415 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign56800_e88434;
        locals.var_t0_dn0 = assign56800_e88434_d_n0;
        locals.var_t0_dn2 = assign56800_e88434_d_n2;
        locals.var_t0_dn4 = assign56800_e88434_d_n4;
        locals.var_t0_dn5 = assign56800_e88434_d_n5;
        locals.var_t0_dn6 = assign56800_e88434_d_n6;
        locals.var_t0_dn7 = assign56800_e88434_d_n7;
        locals.var_t0_dn8 = assign56800_e88434_d_n8;
        locals.var_t0_dn9 = assign56800_e88434_d_n9;
        locals.var_t0_dn10 = assign56800_e88434_d_n10;
        locals.var_t0_dn11 = assign56800_e88434_d_n11;
        locals.var_t0_dn14 = assign56800_e88434_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign56810_e88448, assign56810_e88448_d_n0, assign56810_e88448_d_n2, assign56810_e88448_d_n4, assign56810_e88448_d_n5, assign56810_e88448_d_n6, assign56810_e88448_d_n7, assign56810_e88448_d_n8, assign56810_e88448_d_n9, assign56810_e88448_d_n10, assign56810_e88448_d_n11, assign56810_e88448_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1415 == 0.0)) {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn4, locals.var_vdsorg_dn5, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn8, locals.var_vdsorg_dn9, locals.var_vdsorg_dn10, locals.var_vdsorg_dn11, locals.var_vdsorg_dn14,)
    } else {
        (locals.var_vds_res0, locals.var_vds_res0_dn0, locals.var_vds_res0_dn2, locals.var_vds_res0_dn4, locals.var_vds_res0_dn5, locals.var_vds_res0_dn6, locals.var_vds_res0_dn7, locals.var_vds_res0_dn8, locals.var_vds_res0_dn9, locals.var_vds_res0_dn10, locals.var_vds_res0_dn11, locals.var_vds_res0_dn14,)
    }
};
        locals.var_vds_res0 = assign56810_e88448;
        locals.var_vds_res0_dn0 = assign56810_e88448_d_n0;
        locals.var_vds_res0_dn2 = assign56810_e88448_d_n2;
        locals.var_vds_res0_dn4 = assign56810_e88448_d_n4;
        locals.var_vds_res0_dn5 = assign56810_e88448_d_n5;
        locals.var_vds_res0_dn6 = assign56810_e88448_d_n6;
        locals.var_vds_res0_dn7 = assign56810_e88448_d_n7;
        locals.var_vds_res0_dn8 = assign56810_e88448_d_n8;
        locals.var_vds_res0_dn9 = assign56810_e88448_d_n9;
        locals.var_vds_res0_dn10 = assign56810_e88448_d_n10;
        locals.var_vds_res0_dn11 = assign56810_e88448_d_n11;
        locals.var_vds_res0_dn14 = assign56810_e88448_d_n14;
        locals.var_vds_res0_rv = 0.0;

        let (assign56820_e88462, assign56820_e88462_d_n0, assign56820_e88462_d_n2, assign56820_e88462_d_n4, assign56820_e88462_d_n5, assign56820_e88462_d_n6, assign56820_e88462_d_n7, assign56820_e88462_d_n8, assign56820_e88462_d_n9, assign56820_e88462_d_n10, assign56820_e88462_d_n11, assign56820_e88462_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1415 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign56820_e88462;
        locals.var_t0_dn0 = assign56820_e88462_d_n0;
        locals.var_t0_dn2 = assign56820_e88462_d_n2;
        locals.var_t0_dn4 = assign56820_e88462_d_n4;
        locals.var_t0_dn5 = assign56820_e88462_d_n5;
        locals.var_t0_dn6 = assign56820_e88462_d_n6;
        locals.var_t0_dn7 = assign56820_e88462_d_n7;
        locals.var_t0_dn8 = assign56820_e88462_d_n8;
        locals.var_t0_dn9 = assign56820_e88462_d_n9;
        locals.var_t0_dn10 = assign56820_e88462_d_n10;
        locals.var_t0_dn11 = assign56820_e88462_d_n11;
        locals.var_t0_dn14 = assign56820_e88462_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign56830_e88477, assign56830_e88477_d_n0, assign56830_e88477_d_n2, assign56830_e88477_d_n4, assign56830_e88477_d_n5, assign56830_e88477_d_n6, assign56830_e88477_d_n7, assign56830_e88477_d_n8, assign56830_e88477_d_n9, assign56830_e88477_d_n10, assign56830_e88477_d_n11, assign56830_e88477_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign56830_e88473: f64 = (locals.var_vds_res / locals.var_t6);
        let assign56830_e88475: f64 = (assign56830_e88473 + locals.var_vds_res0);
        (assign56830_e88475, ((((locals.var_vds_res_dn0 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn0)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn0), ((((locals.var_vds_res_dn2 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn2)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn2), ((((locals.var_vds_res_dn4 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn4)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn4), ((((locals.var_vds_res_dn5 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn5)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn5), ((((locals.var_vds_res_dn6 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn6)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn6), ((((locals.var_vds_res_dn7 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn7)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn7), ((((locals.var_vds_res_dn8 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn8)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn8), ((((locals.var_vds_res_dn9 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn9)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn9), ((((locals.var_vds_res_dn10 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn10)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn10), ((((locals.var_vds_res_dn11 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn11)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn11), ((((locals.var_vds_res_dn14 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn14)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn14),)
    } else {
        (locals.var_vds_res, locals.var_vds_res_dn0, locals.var_vds_res_dn2, locals.var_vds_res_dn4, locals.var_vds_res_dn5, locals.var_vds_res_dn6, locals.var_vds_res_dn7, locals.var_vds_res_dn8, locals.var_vds_res_dn9, locals.var_vds_res_dn10, locals.var_vds_res_dn11, locals.var_vds_res_dn14,)
    }
};
        locals.var_vds_res = assign56830_e88477;
        locals.var_vds_res_dn0 = assign56830_e88477_d_n0;
        locals.var_vds_res_dn2 = assign56830_e88477_d_n2;
        locals.var_vds_res_dn4 = assign56830_e88477_d_n4;
        locals.var_vds_res_dn5 = assign56830_e88477_d_n5;
        locals.var_vds_res_dn6 = assign56830_e88477_d_n6;
        locals.var_vds_res_dn7 = assign56830_e88477_d_n7;
        locals.var_vds_res_dn8 = assign56830_e88477_d_n8;
        locals.var_vds_res_dn9 = assign56830_e88477_d_n9;
        locals.var_vds_res_dn10 = assign56830_e88477_d_n10;
        locals.var_vds_res_dn11 = assign56830_e88477_d_n11;
        locals.var_vds_res_dn14 = assign56830_e88477_d_n14;
        locals.var_vds_res_rv = 0.0;

        let (assign56840_e88492, assign56840_e88492_d_n0, assign56840_e88492_d_n2, assign56840_e88492_d_n4, assign56840_e88492_d_n5, assign56840_e88492_d_n6, assign56840_e88492_d_n7, assign56840_e88492_d_n8, assign56840_e88492_d_n9, assign56840_e88492_d_n10, assign56840_e88492_d_n11, assign56840_e88492_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign56840_e88488: f64 = (locals.var_vds_res0 * locals.var_vds_res0);
        let assign56840_e88490: f64 = (assign56840_e88488 * locals.var_vds_res0);
        (assign56840_e88490, ((((locals.var_vds_res0_dn0 * locals.var_vds_res0) + (locals.var_vds_res0 * locals.var_vds_res0_dn0)) * locals.var_vds_res0) + (assign56840_e88488 * locals.var_vds_res0_dn0)), ((((locals.var_vds_res0_dn2 * locals.var_vds_res0) + (locals.var_vds_res0 * locals.var_vds_res0_dn2)) * locals.var_vds_res0) + (assign56840_e88488 * locals.var_vds_res0_dn2)), ((((locals.var_vds_res0_dn4 * locals.var_vds_res0) + (locals.var_vds_res0 * locals.var_vds_res0_dn4)) * locals.var_vds_res0) + (assign56840_e88488 * locals.var_vds_res0_dn4)), ((((locals.var_vds_res0_dn5 * locals.var_vds_res0) + (locals.var_vds_res0 * locals.var_vds_res0_dn5)) * locals.var_vds_res0) + (assign56840_e88488 * locals.var_vds_res0_dn5)), ((((locals.var_vds_res0_dn6 * locals.var_vds_res0) + (locals.var_vds_res0 * locals.var_vds_res0_dn6)) * locals.var_vds_res0) + (assign56840_e88488 * locals.var_vds_res0_dn6)), ((((locals.var_vds_res0_dn7 * locals.var_vds_res0) + (locals.var_vds_res0 * locals.var_vds_res0_dn7)) * locals.var_vds_res0) + (assign56840_e88488 * locals.var_vds_res0_dn7)), ((((locals.var_vds_res0_dn8 * locals.var_vds_res0) + (locals.var_vds_res0 * locals.var_vds_res0_dn8)) * locals.var_vds_res0) + (assign56840_e88488 * locals.var_vds_res0_dn8)), ((((locals.var_vds_res0_dn9 * locals.var_vds_res0) + (locals.var_vds_res0 * locals.var_vds_res0_dn9)) * locals.var_vds_res0) + (assign56840_e88488 * locals.var_vds_res0_dn9)), ((((locals.var_vds_res0_dn10 * locals.var_vds_res0) + (locals.var_vds_res0 * locals.var_vds_res0_dn10)) * locals.var_vds_res0) + (assign56840_e88488 * locals.var_vds_res0_dn10)), ((((locals.var_vds_res0_dn11 * locals.var_vds_res0) + (locals.var_vds_res0 * locals.var_vds_res0_dn11)) * locals.var_vds_res0) + (assign56840_e88488 * locals.var_vds_res0_dn11)), ((((locals.var_vds_res0_dn14 * locals.var_vds_res0) + (locals.var_vds_res0 * locals.var_vds_res0_dn14)) * locals.var_vds_res0) + (assign56840_e88488 * locals.var_vds_res0_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign56840_e88492;
        locals.var_t4_dn0 = assign56840_e88492_d_n0;
        locals.var_t4_dn2 = assign56840_e88492_d_n2;
        locals.var_t4_dn4 = assign56840_e88492_d_n4;
        locals.var_t4_dn5 = assign56840_e88492_d_n5;
        locals.var_t4_dn6 = assign56840_e88492_d_n6;
        locals.var_t4_dn7 = assign56840_e88492_d_n7;
        locals.var_t4_dn8 = assign56840_e88492_d_n8;
        locals.var_t4_dn9 = assign56840_e88492_d_n9;
        locals.var_t4_dn10 = assign56840_e88492_d_n10;
        locals.var_t4_dn11 = assign56840_e88492_d_n11;
        locals.var_t4_dn14 = assign56840_e88492_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign56850_e88505, assign56850_e88505_d_n0, assign56850_e88505_d_n2, assign56850_e88505_d_n4, assign56850_e88505_d_n5, assign56850_e88505_d_n6, assign56850_e88505_d_n7, assign56850_e88505_d_n8, assign56850_e88505_d_n9, assign56850_e88505_d_n10, assign56850_e88505_d_n11, assign56850_e88505_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign56850_e88503: f64 = (locals.var_t4 + 0.0001);
        (assign56850_e88503, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign56850_e88505;
        locals.var_t0_dn0 = assign56850_e88505_d_n0;
        locals.var_t0_dn2 = assign56850_e88505_d_n2;
        locals.var_t0_dn4 = assign56850_e88505_d_n4;
        locals.var_t0_dn5 = assign56850_e88505_d_n5;
        locals.var_t0_dn6 = assign56850_e88505_d_n6;
        locals.var_t0_dn7 = assign56850_e88505_d_n7;
        locals.var_t0_dn8 = assign56850_e88505_d_n8;
        locals.var_t0_dn9 = assign56850_e88505_d_n9;
        locals.var_t0_dn10 = assign56850_e88505_d_n10;
        locals.var_t0_dn11 = assign56850_e88505_d_n11;
        locals.var_t0_dn14 = assign56850_e88505_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign56860_e88518, assign56860_e88518_d_n0, assign56860_e88518_d_n2, assign56860_e88518_d_n4, assign56860_e88518_d_n5, assign56860_e88518_d_n6, assign56860_e88518_d_n7, assign56860_e88518_d_n8, assign56860_e88518_d_n9, assign56860_e88518_d_n10, assign56860_e88518_d_n11, assign56860_e88518_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign56860_e88516: f64 = (locals.var_t4 / locals.var_t0);
        (assign56860_e88516, (((locals.var_t4_dn0 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn2 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn4 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn5 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn6 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn7 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn8 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn9 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn10 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn11 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn14 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn14)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_vds_res0_sym, locals.var_vds_res0_sym_dn0, locals.var_vds_res0_sym_dn2, locals.var_vds_res0_sym_dn4, locals.var_vds_res0_sym_dn5, locals.var_vds_res0_sym_dn6, locals.var_vds_res0_sym_dn7, locals.var_vds_res0_sym_dn8, locals.var_vds_res0_sym_dn9, locals.var_vds_res0_sym_dn10, locals.var_vds_res0_sym_dn11, locals.var_vds_res0_sym_dn14,)
    }
};
        locals.var_vds_res0_sym = assign56860_e88518;
        locals.var_vds_res0_sym_dn0 = assign56860_e88518_d_n0;
        locals.var_vds_res0_sym_dn2 = assign56860_e88518_d_n2;
        locals.var_vds_res0_sym_dn4 = assign56860_e88518_d_n4;
        locals.var_vds_res0_sym_dn5 = assign56860_e88518_d_n5;
        locals.var_vds_res0_sym_dn6 = assign56860_e88518_d_n6;
        locals.var_vds_res0_sym_dn7 = assign56860_e88518_d_n7;
        locals.var_vds_res0_sym_dn8 = assign56860_e88518_d_n8;
        locals.var_vds_res0_sym_dn9 = assign56860_e88518_d_n9;
        locals.var_vds_res0_sym_dn10 = assign56860_e88518_d_n10;
        locals.var_vds_res0_sym_dn11 = assign56860_e88518_d_n11;
        locals.var_vds_res0_sym_dn14 = assign56860_e88518_d_n14;
        locals.var_vds_res0_sym_rv = 0.0;

        let assign56870_e88521: f64 = (-1.0);
        let assign56870_e88522: f64 = if p.p43 == assign56870_e88521 { 1.0 } else { 0.0 };
        locals.var_guard1421 = assign56870_e88522;
        locals.var_guard1421_rv = 0.0;

        let (assign56880_e88535, assign56880_e88535_d_n0, assign56880_e88535_d_n2, assign56880_e88535_d_n4, assign56880_e88535_d_n5, assign56880_e88535_d_n6, assign56880_e88535_d_n7, assign56880_e88535_d_n8, assign56880_e88535_d_n9, assign56880_e88535_d_n10, assign56880_e88535_d_n11, assign56880_e88535_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1421 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vds_res0_sym, locals.var_vds_res0_sym_dn0, locals.var_vds_res0_sym_dn2, locals.var_vds_res0_sym_dn4, locals.var_vds_res0_sym_dn5, locals.var_vds_res0_sym_dn6, locals.var_vds_res0_sym_dn7, locals.var_vds_res0_sym_dn8, locals.var_vds_res0_sym_dn9, locals.var_vds_res0_sym_dn10, locals.var_vds_res0_sym_dn11, locals.var_vds_res0_sym_dn14,)
    }
};
        locals.var_vds_res0_sym = assign56880_e88535;
        locals.var_vds_res0_sym_dn0 = assign56880_e88535_d_n0;
        locals.var_vds_res0_sym_dn2 = assign56880_e88535_d_n2;
        locals.var_vds_res0_sym_dn4 = assign56880_e88535_d_n4;
        locals.var_vds_res0_sym_dn5 = assign56880_e88535_d_n5;
        locals.var_vds_res0_sym_dn6 = assign56880_e88535_d_n6;
        locals.var_vds_res0_sym_dn7 = assign56880_e88535_d_n7;
        locals.var_vds_res0_sym_dn8 = assign56880_e88535_d_n8;
        locals.var_vds_res0_sym_dn9 = assign56880_e88535_d_n9;
        locals.var_vds_res0_sym_dn10 = assign56880_e88535_d_n10;
        locals.var_vds_res0_sym_dn11 = assign56880_e88535_d_n11;
        locals.var_vds_res0_sym_dn14 = assign56880_e88535_d_n14;
        locals.var_vds_res0_sym_rv = 0.0;

        let (assign56890_e88548, assign56890_e88548_d_n0, assign56890_e88548_d_n2, assign56890_e88548_d_n4, assign56890_e88548_d_n5, assign56890_e88548_d_n6, assign56890_e88548_d_n7, assign56890_e88548_d_n8, assign56890_e88548_d_n9, assign56890_e88548_d_n10, assign56890_e88548_d_n11, assign56890_e88548_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1421 != 0.0)) {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn4, locals.var_vdsorg_dn5, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn8, locals.var_vdsorg_dn9, locals.var_vdsorg_dn10, locals.var_vdsorg_dn11, locals.var_vdsorg_dn14,)
    } else {
        (locals.var_vds_res, locals.var_vds_res_dn0, locals.var_vds_res_dn2, locals.var_vds_res_dn4, locals.var_vds_res_dn5, locals.var_vds_res_dn6, locals.var_vds_res_dn7, locals.var_vds_res_dn8, locals.var_vds_res_dn9, locals.var_vds_res_dn10, locals.var_vds_res_dn11, locals.var_vds_res_dn14,)
    }
};
        locals.var_vds_res = assign56890_e88548;
        locals.var_vds_res_dn0 = assign56890_e88548_d_n0;
        locals.var_vds_res_dn2 = assign56890_e88548_d_n2;
        locals.var_vds_res_dn4 = assign56890_e88548_d_n4;
        locals.var_vds_res_dn5 = assign56890_e88548_d_n5;
        locals.var_vds_res_dn6 = assign56890_e88548_d_n6;
        locals.var_vds_res_dn7 = assign56890_e88548_d_n7;
        locals.var_vds_res_dn8 = assign56890_e88548_d_n8;
        locals.var_vds_res_dn9 = assign56890_e88548_d_n9;
        locals.var_vds_res_dn10 = assign56890_e88548_d_n10;
        locals.var_vds_res_dn11 = assign56890_e88548_d_n11;
        locals.var_vds_res_dn14 = assign56890_e88548_d_n14;
        locals.var_vds_res_rv = 0.0;

        let assign56900_e88551: f64 = if p.p43 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1422 = assign56900_e88551;
        locals.var_guard1422_rv = 0.0;

        let (assign56910_e88567, assign56910_e88567_d_n0, assign56910_e88567_d_n2, assign56910_e88567_d_n4, assign56910_e88567_d_n5, assign56910_e88567_d_n6, assign56910_e88567_d_n7, assign56910_e88567_d_n8, assign56910_e88567_d_n9, assign56910_e88567_d_n10, assign56910_e88567_d_n11, assign56910_e88567_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1421 == 0.0)) && (locals.var_guard1422 != 0.0)) {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn4, locals.var_vdsorg_dn5, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn8, locals.var_vdsorg_dn9, locals.var_vdsorg_dn10, locals.var_vdsorg_dn11, locals.var_vdsorg_dn14,)
    } else {
        (locals.var_vds_res, locals.var_vds_res_dn0, locals.var_vds_res_dn2, locals.var_vds_res_dn4, locals.var_vds_res_dn5, locals.var_vds_res_dn6, locals.var_vds_res_dn7, locals.var_vds_res_dn8, locals.var_vds_res_dn9, locals.var_vds_res_dn10, locals.var_vds_res_dn11, locals.var_vds_res_dn14,)
    }
};
        locals.var_vds_res = assign56910_e88567;
        locals.var_vds_res_dn0 = assign56910_e88567_d_n0;
        locals.var_vds_res_dn2 = assign56910_e88567_d_n2;
        locals.var_vds_res_dn4 = assign56910_e88567_d_n4;
        locals.var_vds_res_dn5 = assign56910_e88567_d_n5;
        locals.var_vds_res_dn6 = assign56910_e88567_d_n6;
        locals.var_vds_res_dn7 = assign56910_e88567_d_n7;
        locals.var_vds_res_dn8 = assign56910_e88567_d_n8;
        locals.var_vds_res_dn9 = assign56910_e88567_d_n9;
        locals.var_vds_res_dn10 = assign56910_e88567_d_n10;
        locals.var_vds_res_dn11 = assign56910_e88567_d_n11;
        locals.var_vds_res_dn14 = assign56910_e88567_d_n14;
        locals.var_vds_res_rv = 0.0;

        let (assign56920_e88583, assign56920_e88583_d_n0, assign56920_e88583_d_n2, assign56920_e88583_d_n4, assign56920_e88583_d_n5, assign56920_e88583_d_n6, assign56920_e88583_d_n7, assign56920_e88583_d_n8, assign56920_e88583_d_n9, assign56920_e88583_d_n10, assign56920_e88583_d_n11, assign56920_e88583_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1421 == 0.0)) && (locals.var_guard1422 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vds_res0, locals.var_vds_res0_dn0, locals.var_vds_res0_dn2, locals.var_vds_res0_dn4, locals.var_vds_res0_dn5, locals.var_vds_res0_dn6, locals.var_vds_res0_dn7, locals.var_vds_res0_dn8, locals.var_vds_res0_dn9, locals.var_vds_res0_dn10, locals.var_vds_res0_dn11, locals.var_vds_res0_dn14,)
    }
};
        locals.var_vds_res0 = assign56920_e88583;
        locals.var_vds_res0_dn0 = assign56920_e88583_d_n0;
        locals.var_vds_res0_dn2 = assign56920_e88583_d_n2;
        locals.var_vds_res0_dn4 = assign56920_e88583_d_n4;
        locals.var_vds_res0_dn5 = assign56920_e88583_d_n5;
        locals.var_vds_res0_dn6 = assign56920_e88583_d_n6;
        locals.var_vds_res0_dn7 = assign56920_e88583_d_n7;
        locals.var_vds_res0_dn8 = assign56920_e88583_d_n8;
        locals.var_vds_res0_dn9 = assign56920_e88583_d_n9;
        locals.var_vds_res0_dn10 = assign56920_e88583_d_n10;
        locals.var_vds_res0_dn11 = assign56920_e88583_d_n11;
        locals.var_vds_res0_dn14 = assign56920_e88583_d_n14;
        locals.var_vds_res0_rv = 0.0;

        let (assign56930_e88599, assign56930_e88599_d_n0, assign56930_e88599_d_n2, assign56930_e88599_d_n4, assign56930_e88599_d_n5, assign56930_e88599_d_n6, assign56930_e88599_d_n7, assign56930_e88599_d_n8, assign56930_e88599_d_n9, assign56930_e88599_d_n10, assign56930_e88599_d_n11, assign56930_e88599_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1421 == 0.0)) && (locals.var_guard1422 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vds_res0_sym, locals.var_vds_res0_sym_dn0, locals.var_vds_res0_sym_dn2, locals.var_vds_res0_sym_dn4, locals.var_vds_res0_sym_dn5, locals.var_vds_res0_sym_dn6, locals.var_vds_res0_sym_dn7, locals.var_vds_res0_sym_dn8, locals.var_vds_res0_sym_dn9, locals.var_vds_res0_sym_dn10, locals.var_vds_res0_sym_dn11, locals.var_vds_res0_sym_dn14,)
    }
};
        locals.var_vds_res0_sym = assign56930_e88599;
        locals.var_vds_res0_sym_dn0 = assign56930_e88599_d_n0;
        locals.var_vds_res0_sym_dn2 = assign56930_e88599_d_n2;
        locals.var_vds_res0_sym_dn4 = assign56930_e88599_d_n4;
        locals.var_vds_res0_sym_dn5 = assign56930_e88599_d_n5;
        locals.var_vds_res0_sym_dn6 = assign56930_e88599_d_n6;
        locals.var_vds_res0_sym_dn7 = assign56930_e88599_d_n7;
        locals.var_vds_res0_sym_dn8 = assign56930_e88599_d_n8;
        locals.var_vds_res0_sym_dn9 = assign56930_e88599_d_n9;
        locals.var_vds_res0_sym_dn10 = assign56930_e88599_d_n10;
        locals.var_vds_res0_sym_dn11 = assign56930_e88599_d_n11;
        locals.var_vds_res0_sym_dn14 = assign56930_e88599_d_n14;
        locals.var_vds_res0_sym_rv = 0.0;

        let (assign56940_e88617, assign56940_e88617_d_n0, assign56940_e88617_d_n2, assign56940_e88617_d_n4, assign56940_e88617_d_n5, assign56940_e88617_d_n6, assign56940_e88617_d_n7, assign56940_e88617_d_n8, assign56940_e88617_d_n9, assign56940_e88617_d_n10, assign56940_e88617_d_n11, assign56940_e88617_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1421 == 0.0)) && (locals.var_guard1422 != 0.0)) {
        let assign56940_e88615: f64 = (locals.var_vgp_res_raw - locals.var_uc_depleak);
        (assign56940_e88615, (locals.var_vgp_res_raw_dn0 - locals.var_uc_depleak_dn0), (locals.var_vgp_res_raw_dn2 - locals.var_uc_depleak_dn2), (locals.var_vgp_res_raw_dn4 - locals.var_uc_depleak_dn4), (locals.var_vgp_res_raw_dn5 - locals.var_uc_depleak_dn5), (locals.var_vgp_res_raw_dn6 - locals.var_uc_depleak_dn6), (locals.var_vgp_res_raw_dn7 - locals.var_uc_depleak_dn7), (locals.var_vgp_res_raw_dn8 - locals.var_uc_depleak_dn8), (locals.var_vgp_res_raw_dn9 - locals.var_uc_depleak_dn9), (locals.var_vgp_res_raw_dn10 - locals.var_uc_depleak_dn10), (locals.var_vgp_res_raw_dn11 - locals.var_uc_depleak_dn11), (locals.var_vgp_res_raw_dn14 - locals.var_uc_depleak_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign56940_e88617;
        locals.var_t1_dn0 = assign56940_e88617_d_n0;
        locals.var_t1_dn2 = assign56940_e88617_d_n2;
        locals.var_t1_dn4 = assign56940_e88617_d_n4;
        locals.var_t1_dn5 = assign56940_e88617_d_n5;
        locals.var_t1_dn6 = assign56940_e88617_d_n6;
        locals.var_t1_dn7 = assign56940_e88617_d_n7;
        locals.var_t1_dn8 = assign56940_e88617_d_n8;
        locals.var_t1_dn9 = assign56940_e88617_d_n9;
        locals.var_t1_dn10 = assign56940_e88617_d_n10;
        locals.var_t1_dn11 = assign56940_e88617_d_n11;
        locals.var_t1_dn14 = assign56940_e88617_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign56950_e88644, assign56950_e88644_d_n0, assign56950_e88644_d_n2, assign56950_e88644_d_n4, assign56950_e88644_d_n5, assign56950_e88644_d_n6, assign56950_e88644_d_n7, assign56950_e88644_d_n8, assign56950_e88644_d_n9, assign56950_e88644_d_n10, assign56950_e88644_d_n11, assign56950_e88644_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1421 == 0.0)) && (locals.var_guard1422 != 0.0)) {
        let assign56950_e88634: f64 = (locals.var_t1).cosh();
        let assign56950_e88635: f64 = (assign56950_e88634).ln();
        let assign56950_e88636: f64 = (locals.var_t1 + assign56950_e88635);
        let assign56950_e88638: f64 = (2.0_f64).ln();
        let assign56950_e88639: f64 = (assign56950_e88636 + assign56950_e88638);
        let assign56950_e88640: f64 = (0.5 * assign56950_e88639);
        let assign56950_e88642: f64 = (assign56950_e88640 + locals.var_uc_depleak);
        (assign56950_e88642, ((0.5 * (locals.var_t1_dn0 + (((locals.var_t1).sinh() * locals.var_t1_dn0) / assign56950_e88634))) + locals.var_uc_depleak_dn0), ((0.5 * (locals.var_t1_dn2 + (((locals.var_t1).sinh() * locals.var_t1_dn2) / assign56950_e88634))) + locals.var_uc_depleak_dn2), ((0.5 * (locals.var_t1_dn4 + (((locals.var_t1).sinh() * locals.var_t1_dn4) / assign56950_e88634))) + locals.var_uc_depleak_dn4), ((0.5 * (locals.var_t1_dn5 + (((locals.var_t1).sinh() * locals.var_t1_dn5) / assign56950_e88634))) + locals.var_uc_depleak_dn5), ((0.5 * (locals.var_t1_dn6 + (((locals.var_t1).sinh() * locals.var_t1_dn6) / assign56950_e88634))) + locals.var_uc_depleak_dn6), ((0.5 * (locals.var_t1_dn7 + (((locals.var_t1).sinh() * locals.var_t1_dn7) / assign56950_e88634))) + locals.var_uc_depleak_dn7), ((0.5 * (locals.var_t1_dn8 + (((locals.var_t1).sinh() * locals.var_t1_dn8) / assign56950_e88634))) + locals.var_uc_depleak_dn8), ((0.5 * (locals.var_t1_dn9 + (((locals.var_t1).sinh() * locals.var_t1_dn9) / assign56950_e88634))) + locals.var_uc_depleak_dn9), ((0.5 * (locals.var_t1_dn10 + (((locals.var_t1).sinh() * locals.var_t1_dn10) / assign56950_e88634))) + locals.var_uc_depleak_dn10), ((0.5 * (locals.var_t1_dn11 + (((locals.var_t1).sinh() * locals.var_t1_dn11) / assign56950_e88634))) + locals.var_uc_depleak_dn11), ((0.5 * (locals.var_t1_dn14 + (((locals.var_t1).sinh() * locals.var_t1_dn14) / assign56950_e88634))) + locals.var_uc_depleak_dn14),)
    } else {
        (locals.var_vdssat_res, locals.var_vdssat_res_dn0, locals.var_vdssat_res_dn2, locals.var_vdssat_res_dn4, locals.var_vdssat_res_dn5, locals.var_vdssat_res_dn6, locals.var_vdssat_res_dn7, locals.var_vdssat_res_dn8, locals.var_vdssat_res_dn9, locals.var_vdssat_res_dn10, locals.var_vdssat_res_dn11, locals.var_vdssat_res_dn14,)
    }
};
        locals.var_vdssat_res = assign56950_e88644;
        locals.var_vdssat_res_dn0 = assign56950_e88644_d_n0;
        locals.var_vdssat_res_dn2 = assign56950_e88644_d_n2;
        locals.var_vdssat_res_dn4 = assign56950_e88644_d_n4;
        locals.var_vdssat_res_dn5 = assign56950_e88644_d_n5;
        locals.var_vdssat_res_dn6 = assign56950_e88644_d_n6;
        locals.var_vdssat_res_dn7 = assign56950_e88644_d_n7;
        locals.var_vdssat_res_dn8 = assign56950_e88644_d_n8;
        locals.var_vdssat_res_dn9 = assign56950_e88644_d_n9;
        locals.var_vdssat_res_dn10 = assign56950_e88644_d_n10;
        locals.var_vdssat_res_dn11 = assign56950_e88644_d_n11;
        locals.var_vdssat_res_dn14 = assign56950_e88644_d_n14;
        locals.var_vdssat_res_rv = 0.0;

        let assign56960_e88647: f64 = if p.p43 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1423 = assign56960_e88647;
        locals.var_guard1423_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_208(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign56970_e88674, assign56970_e88674_d_n0, assign56970_e88674_d_n2, assign56970_e88674_d_n4, assign56970_e88674_d_n5, assign56970_e88674_d_n6, assign56970_e88674_d_n7, assign56970_e88674_d_n8, assign56970_e88674_d_n9, assign56970_e88674_d_n10, assign56970_e88674_d_n11, assign56970_e88674_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1421 == 0.0)) && (locals.var_guard1422 == 0.0)) && (locals.var_guard1423 != 0.0)) {
        let assign56970_e88667: f64 = (locals.var_vgp_res_raw - locals.var_uc_depleak);
        let assign56970_e88668: f64 = (assign56970_e88667).exp();
        let assign56970_e88669: f64 = (1.0 + assign56970_e88668);
        let assign56970_e88670: f64 = (assign56970_e88669).ln();
        let assign56970_e88672: f64 = (assign56970_e88670 + locals.var_uc_depleak);
        (assign56970_e88672, (((assign56970_e88668 * (locals.var_vgp_res_raw_dn0 - locals.var_uc_depleak_dn0)) / assign56970_e88669) + locals.var_uc_depleak_dn0), (((assign56970_e88668 * (locals.var_vgp_res_raw_dn2 - locals.var_uc_depleak_dn2)) / assign56970_e88669) + locals.var_uc_depleak_dn2), (((assign56970_e88668 * (locals.var_vgp_res_raw_dn4 - locals.var_uc_depleak_dn4)) / assign56970_e88669) + locals.var_uc_depleak_dn4), (((assign56970_e88668 * (locals.var_vgp_res_raw_dn5 - locals.var_uc_depleak_dn5)) / assign56970_e88669) + locals.var_uc_depleak_dn5), (((assign56970_e88668 * (locals.var_vgp_res_raw_dn6 - locals.var_uc_depleak_dn6)) / assign56970_e88669) + locals.var_uc_depleak_dn6), (((assign56970_e88668 * (locals.var_vgp_res_raw_dn7 - locals.var_uc_depleak_dn7)) / assign56970_e88669) + locals.var_uc_depleak_dn7), (((assign56970_e88668 * (locals.var_vgp_res_raw_dn8 - locals.var_uc_depleak_dn8)) / assign56970_e88669) + locals.var_uc_depleak_dn8), (((assign56970_e88668 * (locals.var_vgp_res_raw_dn9 - locals.var_uc_depleak_dn9)) / assign56970_e88669) + locals.var_uc_depleak_dn9), (((assign56970_e88668 * (locals.var_vgp_res_raw_dn10 - locals.var_uc_depleak_dn10)) / assign56970_e88669) + locals.var_uc_depleak_dn10), (((assign56970_e88668 * (locals.var_vgp_res_raw_dn11 - locals.var_uc_depleak_dn11)) / assign56970_e88669) + locals.var_uc_depleak_dn11), (((assign56970_e88668 * (locals.var_vgp_res_raw_dn14 - locals.var_uc_depleak_dn14)) / assign56970_e88669) + locals.var_uc_depleak_dn14),)
    } else {
        (locals.var_vdssat_res, locals.var_vdssat_res_dn0, locals.var_vdssat_res_dn2, locals.var_vdssat_res_dn4, locals.var_vdssat_res_dn5, locals.var_vdssat_res_dn6, locals.var_vdssat_res_dn7, locals.var_vdssat_res_dn8, locals.var_vdssat_res_dn9, locals.var_vdssat_res_dn10, locals.var_vdssat_res_dn11, locals.var_vdssat_res_dn14,)
    }
};
        locals.var_vdssat_res = assign56970_e88674;
        locals.var_vdssat_res_dn0 = assign56970_e88674_d_n0;
        locals.var_vdssat_res_dn2 = assign56970_e88674_d_n2;
        locals.var_vdssat_res_dn4 = assign56970_e88674_d_n4;
        locals.var_vdssat_res_dn5 = assign56970_e88674_d_n5;
        locals.var_vdssat_res_dn6 = assign56970_e88674_d_n6;
        locals.var_vdssat_res_dn7 = assign56970_e88674_d_n7;
        locals.var_vdssat_res_dn8 = assign56970_e88674_d_n8;
        locals.var_vdssat_res_dn9 = assign56970_e88674_d_n9;
        locals.var_vdssat_res_dn10 = assign56970_e88674_d_n10;
        locals.var_vdssat_res_dn11 = assign56970_e88674_d_n11;
        locals.var_vdssat_res_dn14 = assign56970_e88674_d_n14;
        locals.var_vdssat_res_rv = 0.0;

        let (assign56980_e88690, assign56980_e88690_d_n0, assign56980_e88690_d_n2, assign56980_e88690_d_n4, assign56980_e88690_d_n5, assign56980_e88690_d_n6, assign56980_e88690_d_n7, assign56980_e88690_d_n8, assign56980_e88690_d_n9, assign56980_e88690_d_n10, assign56980_e88690_d_n11, assign56980_e88690_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1421 == 0.0)) {
        let assign56980_e88688: f64 = (locals.var_vds_res / locals.var_vdssat_res);
        (assign56980_e88688, (((locals.var_vds_res_dn0 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn0)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn2 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn2)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn4 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn4)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn5 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn5)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn6 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn6)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn7 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn7)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn8 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn8)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn9 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn9)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn10 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn10)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn11 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn11)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn14 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn14)) / (locals.var_vdssat_res * locals.var_vdssat_res)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign56980_e88690;
        locals.var_t1_dn0 = assign56980_e88690_d_n0;
        locals.var_t1_dn2 = assign56980_e88690_d_n2;
        locals.var_t1_dn4 = assign56980_e88690_d_n4;
        locals.var_t1_dn5 = assign56980_e88690_d_n5;
        locals.var_t1_dn6 = assign56980_e88690_d_n6;
        locals.var_t1_dn7 = assign56980_e88690_d_n7;
        locals.var_t1_dn8 = assign56980_e88690_d_n8;
        locals.var_t1_dn9 = assign56980_e88690_d_n9;
        locals.var_t1_dn10 = assign56980_e88690_d_n10;
        locals.var_t1_dn11 = assign56980_e88690_d_n11;
        locals.var_t1_dn14 = assign56980_e88690_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign56990_e88713, assign56990_e88713_d_n0, assign56990_e88713_d_n2, assign56990_e88713_d_n4, assign56990_e88713_d_n5, assign56990_e88713_d_n6, assign56990_e88713_d_n7, assign56990_e88713_d_n8, assign56990_e88713_d_n9, assign56990_e88713_d_n10, assign56990_e88713_d_n11, assign56990_e88713_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1421 == 0.0)) {
        let (assign56990_e88711, assign56990_e88711_d_n0, assign56990_e88711_d_n2, assign56990_e88711_d_n4, assign56990_e88711_d_n5, assign56990_e88711_d_n6, assign56990_e88711_d_n7, assign56990_e88711_d_n8, assign56990_e88711_d_n9, assign56990_e88711_d_n10, assign56990_e88711_d_n11, assign56990_e88711_d_n14,) = {
            if (locals.var_t1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign56990_e88709: f64 = (p.p383 - 1.0);
                let assign56990_e88710: f64 = (locals.var_t1).powf(assign56990_e88709);
                (assign56990_e88710, if 0.0 == 0.0 && ((assign56990_e88709) as f64).is_finite() && ((assign56990_e88709) as f64).fract() == 0.0 { if assign56990_e88709 == 0.0 { 0.0 } else { (assign56990_e88709 * ((locals.var_t1).powf(assign56990_e88709 - 1.0) * locals.var_t1_dn0)) } } else { (assign56990_e88710 * (assign56990_e88709 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign56990_e88709) as f64).is_finite() && ((assign56990_e88709) as f64).fract() == 0.0 { if assign56990_e88709 == 0.0 { 0.0 } else { (assign56990_e88709 * ((locals.var_t1).powf(assign56990_e88709 - 1.0) * locals.var_t1_dn2)) } } else { (assign56990_e88710 * (assign56990_e88709 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign56990_e88709) as f64).is_finite() && ((assign56990_e88709) as f64).fract() == 0.0 { if assign56990_e88709 == 0.0 { 0.0 } else { (assign56990_e88709 * ((locals.var_t1).powf(assign56990_e88709 - 1.0) * locals.var_t1_dn4)) } } else { (assign56990_e88710 * (assign56990_e88709 * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign56990_e88709) as f64).is_finite() && ((assign56990_e88709) as f64).fract() == 0.0 { if assign56990_e88709 == 0.0 { 0.0 } else { (assign56990_e88709 * ((locals.var_t1).powf(assign56990_e88709 - 1.0) * locals.var_t1_dn5)) } } else { (assign56990_e88710 * (assign56990_e88709 * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign56990_e88709) as f64).is_finite() && ((assign56990_e88709) as f64).fract() == 0.0 { if assign56990_e88709 == 0.0 { 0.0 } else { (assign56990_e88709 * ((locals.var_t1).powf(assign56990_e88709 - 1.0) * locals.var_t1_dn6)) } } else { (assign56990_e88710 * (assign56990_e88709 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign56990_e88709) as f64).is_finite() && ((assign56990_e88709) as f64).fract() == 0.0 { if assign56990_e88709 == 0.0 { 0.0 } else { (assign56990_e88709 * ((locals.var_t1).powf(assign56990_e88709 - 1.0) * locals.var_t1_dn7)) } } else { (assign56990_e88710 * (assign56990_e88709 * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign56990_e88709) as f64).is_finite() && ((assign56990_e88709) as f64).fract() == 0.0 { if assign56990_e88709 == 0.0 { 0.0 } else { (assign56990_e88709 * ((locals.var_t1).powf(assign56990_e88709 - 1.0) * locals.var_t1_dn8)) } } else { (assign56990_e88710 * (assign56990_e88709 * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign56990_e88709) as f64).is_finite() && ((assign56990_e88709) as f64).fract() == 0.0 { if assign56990_e88709 == 0.0 { 0.0 } else { (assign56990_e88709 * ((locals.var_t1).powf(assign56990_e88709 - 1.0) * locals.var_t1_dn9)) } } else { (assign56990_e88710 * (assign56990_e88709 * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign56990_e88709) as f64).is_finite() && ((assign56990_e88709) as f64).fract() == 0.0 { if assign56990_e88709 == 0.0 { 0.0 } else { (assign56990_e88709 * ((locals.var_t1).powf(assign56990_e88709 - 1.0) * locals.var_t1_dn10)) } } else { (assign56990_e88710 * (assign56990_e88709 * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign56990_e88709) as f64).is_finite() && ((assign56990_e88709) as f64).fract() == 0.0 { if assign56990_e88709 == 0.0 { 0.0 } else { (assign56990_e88709 * ((locals.var_t1).powf(assign56990_e88709 - 1.0) * locals.var_t1_dn11)) } } else { (assign56990_e88710 * (assign56990_e88709 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign56990_e88709) as f64).is_finite() && ((assign56990_e88709) as f64).fract() == 0.0 { if assign56990_e88709 == 0.0 { 0.0 } else { (assign56990_e88709 * ((locals.var_t1).powf(assign56990_e88709 - 1.0) * locals.var_t1_dn14)) } } else { (assign56990_e88710 * (assign56990_e88709 * (locals.var_t1_dn14 / locals.var_t1))) },)
            }
        };
        (assign56990_e88711, assign56990_e88711_d_n0, assign56990_e88711_d_n2, assign56990_e88711_d_n4, assign56990_e88711_d_n5, assign56990_e88711_d_n6, assign56990_e88711_d_n7, assign56990_e88711_d_n8, assign56990_e88711_d_n9, assign56990_e88711_d_n10, assign56990_e88711_d_n11, assign56990_e88711_d_n14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign56990_e88713;
        locals.var_t2_dn0 = assign56990_e88713_d_n0;
        locals.var_t2_dn2 = assign56990_e88713_d_n2;
        locals.var_t2_dn4 = assign56990_e88713_d_n4;
        locals.var_t2_dn5 = assign56990_e88713_d_n5;
        locals.var_t2_dn6 = assign56990_e88713_d_n6;
        locals.var_t2_dn7 = assign56990_e88713_d_n7;
        locals.var_t2_dn8 = assign56990_e88713_d_n8;
        locals.var_t2_dn9 = assign56990_e88713_d_n9;
        locals.var_t2_dn10 = assign56990_e88713_d_n10;
        locals.var_t2_dn11 = assign56990_e88713_d_n11;
        locals.var_t2_dn14 = assign56990_e88713_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign57000_e88731, assign57000_e88731_d_n0, assign57000_e88731_d_n2, assign57000_e88731_d_n4, assign57000_e88731_d_n5, assign57000_e88731_d_n6, assign57000_e88731_d_n7, assign57000_e88731_d_n8, assign57000_e88731_d_n9, assign57000_e88731_d_n10, assign57000_e88731_d_n11, assign57000_e88731_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1421 == 0.0)) {
        let assign57000_e88728: f64 = (locals.var_t2 * locals.var_t1);
        let assign57000_e88729: f64 = (1.0 + assign57000_e88728);
        (assign57000_e88729, ((locals.var_t2_dn0 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn0)), ((locals.var_t2_dn2 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn2)), ((locals.var_t2_dn4 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn4)), ((locals.var_t2_dn5 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn5)), ((locals.var_t2_dn6 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn6)), ((locals.var_t2_dn7 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn7)), ((locals.var_t2_dn8 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn8)), ((locals.var_t2_dn9 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn9)), ((locals.var_t2_dn10 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn10)), ((locals.var_t2_dn11 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn11)), ((locals.var_t2_dn14 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign57000_e88731;
        locals.var_t3_dn0 = assign57000_e88731_d_n0;
        locals.var_t3_dn2 = assign57000_e88731_d_n2;
        locals.var_t3_dn4 = assign57000_e88731_d_n4;
        locals.var_t3_dn5 = assign57000_e88731_d_n5;
        locals.var_t3_dn6 = assign57000_e88731_d_n6;
        locals.var_t3_dn7 = assign57000_e88731_d_n7;
        locals.var_t3_dn8 = assign57000_e88731_d_n8;
        locals.var_t3_dn9 = assign57000_e88731_d_n9;
        locals.var_t3_dn10 = assign57000_e88731_d_n10;
        locals.var_t3_dn11 = assign57000_e88731_d_n11;
        locals.var_t3_dn14 = assign57000_e88731_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign57010_e88756, assign57010_e88756_d_n0, assign57010_e88756_d_n2, assign57010_e88756_d_n4, assign57010_e88756_d_n5, assign57010_e88756_d_n6, assign57010_e88756_d_n7, assign57010_e88756_d_n8, assign57010_e88756_d_n9, assign57010_e88756_d_n10, assign57010_e88756_d_n11, assign57010_e88756_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1421 == 0.0)) {
        let (assign57010_e88754, assign57010_e88754_d_n0, assign57010_e88754_d_n2, assign57010_e88754_d_n4, assign57010_e88754_d_n5, assign57010_e88754_d_n6, assign57010_e88754_d_n7, assign57010_e88754_d_n8, assign57010_e88754_d_n9, assign57010_e88754_d_n10, assign57010_e88754_d_n11, assign57010_e88754_d_n14,) = {
            if (locals.var_t3 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign57010_e88750: f64 = (1.0 / p.p383);
                let assign57010_e88752: f64 = (assign57010_e88750 - 1.0);
                let assign57010_e88753: f64 = (locals.var_t3).powf(assign57010_e88752);
                (assign57010_e88753, if 0.0 == 0.0 && ((assign57010_e88752) as f64).is_finite() && ((assign57010_e88752) as f64).fract() == 0.0 { if assign57010_e88752 == 0.0 { 0.0 } else { (assign57010_e88752 * ((locals.var_t3).powf(assign57010_e88752 - 1.0) * locals.var_t3_dn0)) } } else { (assign57010_e88753 * (assign57010_e88752 * (locals.var_t3_dn0 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57010_e88752) as f64).is_finite() && ((assign57010_e88752) as f64).fract() == 0.0 { if assign57010_e88752 == 0.0 { 0.0 } else { (assign57010_e88752 * ((locals.var_t3).powf(assign57010_e88752 - 1.0) * locals.var_t3_dn2)) } } else { (assign57010_e88753 * (assign57010_e88752 * (locals.var_t3_dn2 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57010_e88752) as f64).is_finite() && ((assign57010_e88752) as f64).fract() == 0.0 { if assign57010_e88752 == 0.0 { 0.0 } else { (assign57010_e88752 * ((locals.var_t3).powf(assign57010_e88752 - 1.0) * locals.var_t3_dn4)) } } else { (assign57010_e88753 * (assign57010_e88752 * (locals.var_t3_dn4 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57010_e88752) as f64).is_finite() && ((assign57010_e88752) as f64).fract() == 0.0 { if assign57010_e88752 == 0.0 { 0.0 } else { (assign57010_e88752 * ((locals.var_t3).powf(assign57010_e88752 - 1.0) * locals.var_t3_dn5)) } } else { (assign57010_e88753 * (assign57010_e88752 * (locals.var_t3_dn5 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57010_e88752) as f64).is_finite() && ((assign57010_e88752) as f64).fract() == 0.0 { if assign57010_e88752 == 0.0 { 0.0 } else { (assign57010_e88752 * ((locals.var_t3).powf(assign57010_e88752 - 1.0) * locals.var_t3_dn6)) } } else { (assign57010_e88753 * (assign57010_e88752 * (locals.var_t3_dn6 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57010_e88752) as f64).is_finite() && ((assign57010_e88752) as f64).fract() == 0.0 { if assign57010_e88752 == 0.0 { 0.0 } else { (assign57010_e88752 * ((locals.var_t3).powf(assign57010_e88752 - 1.0) * locals.var_t3_dn7)) } } else { (assign57010_e88753 * (assign57010_e88752 * (locals.var_t3_dn7 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57010_e88752) as f64).is_finite() && ((assign57010_e88752) as f64).fract() == 0.0 { if assign57010_e88752 == 0.0 { 0.0 } else { (assign57010_e88752 * ((locals.var_t3).powf(assign57010_e88752 - 1.0) * locals.var_t3_dn8)) } } else { (assign57010_e88753 * (assign57010_e88752 * (locals.var_t3_dn8 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57010_e88752) as f64).is_finite() && ((assign57010_e88752) as f64).fract() == 0.0 { if assign57010_e88752 == 0.0 { 0.0 } else { (assign57010_e88752 * ((locals.var_t3).powf(assign57010_e88752 - 1.0) * locals.var_t3_dn9)) } } else { (assign57010_e88753 * (assign57010_e88752 * (locals.var_t3_dn9 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57010_e88752) as f64).is_finite() && ((assign57010_e88752) as f64).fract() == 0.0 { if assign57010_e88752 == 0.0 { 0.0 } else { (assign57010_e88752 * ((locals.var_t3).powf(assign57010_e88752 - 1.0) * locals.var_t3_dn10)) } } else { (assign57010_e88753 * (assign57010_e88752 * (locals.var_t3_dn10 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57010_e88752) as f64).is_finite() && ((assign57010_e88752) as f64).fract() == 0.0 { if assign57010_e88752 == 0.0 { 0.0 } else { (assign57010_e88752 * ((locals.var_t3).powf(assign57010_e88752 - 1.0) * locals.var_t3_dn11)) } } else { (assign57010_e88753 * (assign57010_e88752 * (locals.var_t3_dn11 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57010_e88752) as f64).is_finite() && ((assign57010_e88752) as f64).fract() == 0.0 { if assign57010_e88752 == 0.0 { 0.0 } else { (assign57010_e88752 * ((locals.var_t3).powf(assign57010_e88752 - 1.0) * locals.var_t3_dn14)) } } else { (assign57010_e88753 * (assign57010_e88752 * (locals.var_t3_dn14 / locals.var_t3))) },)
            }
        };
        (assign57010_e88754, assign57010_e88754_d_n0, assign57010_e88754_d_n2, assign57010_e88754_d_n4, assign57010_e88754_d_n5, assign57010_e88754_d_n6, assign57010_e88754_d_n7, assign57010_e88754_d_n8, assign57010_e88754_d_n9, assign57010_e88754_d_n10, assign57010_e88754_d_n11, assign57010_e88754_d_n14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign57010_e88756;
        locals.var_t4_dn0 = assign57010_e88756_d_n0;
        locals.var_t4_dn2 = assign57010_e88756_d_n2;
        locals.var_t4_dn4 = assign57010_e88756_d_n4;
        locals.var_t4_dn5 = assign57010_e88756_d_n5;
        locals.var_t4_dn6 = assign57010_e88756_d_n6;
        locals.var_t4_dn7 = assign57010_e88756_d_n7;
        locals.var_t4_dn8 = assign57010_e88756_d_n8;
        locals.var_t4_dn9 = assign57010_e88756_d_n9;
        locals.var_t4_dn10 = assign57010_e88756_d_n10;
        locals.var_t4_dn11 = assign57010_e88756_d_n11;
        locals.var_t4_dn14 = assign57010_e88756_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign57020_e88772, assign57020_e88772_d_n0, assign57020_e88772_d_n2, assign57020_e88772_d_n4, assign57020_e88772_d_n5, assign57020_e88772_d_n6, assign57020_e88772_d_n7, assign57020_e88772_d_n8, assign57020_e88772_d_n9, assign57020_e88772_d_n10, assign57020_e88772_d_n11, assign57020_e88772_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1421 == 0.0)) {
        let assign57020_e88770: f64 = (locals.var_t4 * locals.var_t3);
        (assign57020_e88770, ((locals.var_t4_dn0 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn0)), ((locals.var_t4_dn2 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn2)), ((locals.var_t4_dn4 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn4)), ((locals.var_t4_dn5 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn5)), ((locals.var_t4_dn6 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn6)), ((locals.var_t4_dn7 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn7)), ((locals.var_t4_dn8 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn8)), ((locals.var_t4_dn9 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn9)), ((locals.var_t4_dn10 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn10)), ((locals.var_t4_dn11 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn11)), ((locals.var_t4_dn14 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign57020_e88772;
        locals.var_t6_dn0 = assign57020_e88772_d_n0;
        locals.var_t6_dn2 = assign57020_e88772_d_n2;
        locals.var_t6_dn4 = assign57020_e88772_d_n4;
        locals.var_t6_dn5 = assign57020_e88772_d_n5;
        locals.var_t6_dn6 = assign57020_e88772_d_n6;
        locals.var_t6_dn7 = assign57020_e88772_d_n7;
        locals.var_t6_dn8 = assign57020_e88772_d_n8;
        locals.var_t6_dn9 = assign57020_e88772_d_n9;
        locals.var_t6_dn10 = assign57020_e88772_d_n10;
        locals.var_t6_dn11 = assign57020_e88772_d_n11;
        locals.var_t6_dn14 = assign57020_e88772_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign57030_e88790, assign57030_e88790_d_n0, assign57030_e88790_d_n2, assign57030_e88790_d_n4, assign57030_e88790_d_n5, assign57030_e88790_d_n6, assign57030_e88790_d_n7, assign57030_e88790_d_n8, assign57030_e88790_d_n9, assign57030_e88790_d_n10, assign57030_e88790_d_n11, assign57030_e88790_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1421 == 0.0)) {
        let assign57030_e88786: f64 = (locals.var_vds_res / locals.var_t6);
        let assign57030_e88788: f64 = (assign57030_e88786 + locals.var_vds_res0);
        (assign57030_e88788, ((((locals.var_vds_res_dn0 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn0)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn0), ((((locals.var_vds_res_dn2 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn2)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn2), ((((locals.var_vds_res_dn4 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn4)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn4), ((((locals.var_vds_res_dn5 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn5)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn5), ((((locals.var_vds_res_dn6 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn6)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn6), ((((locals.var_vds_res_dn7 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn7)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn7), ((((locals.var_vds_res_dn8 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn8)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn8), ((((locals.var_vds_res_dn9 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn9)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn9), ((((locals.var_vds_res_dn10 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn10)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn10), ((((locals.var_vds_res_dn11 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn11)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn11), ((((locals.var_vds_res_dn14 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn14)) / (locals.var_t6 * locals.var_t6)) + locals.var_vds_res0_dn14),)
    } else {
        (locals.var_vds_res, locals.var_vds_res_dn0, locals.var_vds_res_dn2, locals.var_vds_res_dn4, locals.var_vds_res_dn5, locals.var_vds_res_dn6, locals.var_vds_res_dn7, locals.var_vds_res_dn8, locals.var_vds_res_dn9, locals.var_vds_res_dn10, locals.var_vds_res_dn11, locals.var_vds_res_dn14,)
    }
};
        locals.var_vds_res = assign57030_e88790;
        locals.var_vds_res_dn0 = assign57030_e88790_d_n0;
        locals.var_vds_res_dn2 = assign57030_e88790_d_n2;
        locals.var_vds_res_dn4 = assign57030_e88790_d_n4;
        locals.var_vds_res_dn5 = assign57030_e88790_d_n5;
        locals.var_vds_res_dn6 = assign57030_e88790_d_n6;
        locals.var_vds_res_dn7 = assign57030_e88790_d_n7;
        locals.var_vds_res_dn8 = assign57030_e88790_d_n8;
        locals.var_vds_res_dn9 = assign57030_e88790_d_n9;
        locals.var_vds_res_dn10 = assign57030_e88790_d_n10;
        locals.var_vds_res_dn11 = assign57030_e88790_d_n11;
        locals.var_vds_res_dn14 = assign57030_e88790_d_n14;
        locals.var_vds_res_rv = 0.0;

        let (assign57040_e88803, assign57040_e88803_d_n0, assign57040_e88803_d_n2, assign57040_e88803_d_n4, assign57040_e88803_d_n5, assign57040_e88803_d_n6, assign57040_e88803_d_n7, assign57040_e88803_d_n8, assign57040_e88803_d_n9, assign57040_e88803_d_n10, assign57040_e88803_d_n11, assign57040_e88803_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign57040_e88801: f64 = (locals.var_w_res * locals.var_q_ndepm__blk1135);
        (assign57040_e88801, ((locals.var_w_res_dn0 * locals.var_q_ndepm__blk1135) + (locals.var_w_res * locals.var_q_ndepm__blk1135_dn0)), ((locals.var_w_res_dn2 * locals.var_q_ndepm__blk1135) + (locals.var_w_res * locals.var_q_ndepm__blk1135_dn2)), ((locals.var_w_res_dn4 * locals.var_q_ndepm__blk1135) + (locals.var_w_res * locals.var_q_ndepm__blk1135_dn4)), ((locals.var_w_res_dn5 * locals.var_q_ndepm__blk1135) + (locals.var_w_res * locals.var_q_ndepm__blk1135_dn5)), ((locals.var_w_res_dn6 * locals.var_q_ndepm__blk1135) + (locals.var_w_res * locals.var_q_ndepm__blk1135_dn6)), ((locals.var_w_res_dn7 * locals.var_q_ndepm__blk1135) + (locals.var_w_res * locals.var_q_ndepm__blk1135_dn7)), ((locals.var_w_res_dn8 * locals.var_q_ndepm__blk1135) + (locals.var_w_res * locals.var_q_ndepm__blk1135_dn8)), ((locals.var_w_res_dn9 * locals.var_q_ndepm__blk1135) + (locals.var_w_res * locals.var_q_ndepm__blk1135_dn9)), ((locals.var_w_res_dn10 * locals.var_q_ndepm__blk1135) + (locals.var_w_res * locals.var_q_ndepm__blk1135_dn10)), ((locals.var_w_res_dn11 * locals.var_q_ndepm__blk1135) + (locals.var_w_res * locals.var_q_ndepm__blk1135_dn11)), ((locals.var_w_res_dn14 * locals.var_q_ndepm__blk1135) + (locals.var_w_res * locals.var_q_ndepm__blk1135_dn14)),)
    } else {
        (locals.var_qn_res__blk1126, locals.var_qn_res__blk1126_dn0, locals.var_qn_res__blk1126_dn2, locals.var_qn_res__blk1126_dn4, locals.var_qn_res__blk1126_dn5, locals.var_qn_res__blk1126_dn6, locals.var_qn_res__blk1126_dn7, locals.var_qn_res__blk1126_dn8, locals.var_qn_res__blk1126_dn9, locals.var_qn_res__blk1126_dn10, locals.var_qn_res__blk1126_dn11, locals.var_qn_res__blk1126_dn14,)
    }
};
        locals.var_qn_res__blk1126 = assign57040_e88803;
        locals.var_qn_res__blk1126_dn0 = assign57040_e88803_d_n0;
        locals.var_qn_res__blk1126_dn2 = assign57040_e88803_d_n2;
        locals.var_qn_res__blk1126_dn4 = assign57040_e88803_d_n4;
        locals.var_qn_res__blk1126_dn5 = assign57040_e88803_d_n5;
        locals.var_qn_res__blk1126_dn6 = assign57040_e88803_d_n6;
        locals.var_qn_res__blk1126_dn7 = assign57040_e88803_d_n7;
        locals.var_qn_res__blk1126_dn8 = assign57040_e88803_d_n8;
        locals.var_qn_res__blk1126_dn9 = assign57040_e88803_d_n9;
        locals.var_qn_res__blk1126_dn10 = assign57040_e88803_d_n10;
        locals.var_qn_res__blk1126_dn11 = assign57040_e88803_d_n11;
        locals.var_qn_res__blk1126_dn14 = assign57040_e88803_d_n14;
        locals.var_qn_res__blk1126_rv = 0.0;

        let (assign57050_e88816, assign57050_e88816_d_n0, assign57050_e88816_d_n2, assign57050_e88816_d_n4, assign57050_e88816_d_n5, assign57050_e88816_d_n6, assign57050_e88816_d_n7, assign57050_e88816_d_n8, assign57050_e88816_d_n9, assign57050_e88816_d_n10, assign57050_e88816_d_n11, assign57050_e88816_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign57050_e88814: f64 = (1.6021918e-19 * 10000.0);
        (assign57050_e88814, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign57050_e88816;
        locals.var_t9_dn0 = assign57050_e88816_d_n0;
        locals.var_t9_dn2 = assign57050_e88816_d_n2;
        locals.var_t9_dn4 = assign57050_e88816_d_n4;
        locals.var_t9_dn5 = assign57050_e88816_d_n5;
        locals.var_t9_dn6 = assign57050_e88816_d_n6;
        locals.var_t9_dn7 = assign57050_e88816_d_n7;
        locals.var_t9_dn8 = assign57050_e88816_d_n8;
        locals.var_t9_dn9 = assign57050_e88816_d_n9;
        locals.var_t9_dn10 = assign57050_e88816_d_n10;
        locals.var_t9_dn11 = assign57050_e88816_d_n11;
        locals.var_t9_dn14 = assign57050_e88816_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign57060_e88829, assign57060_e88829_d_n0, assign57060_e88829_d_n2, assign57060_e88829_d_n4, assign57060_e88829_d_n5, assign57060_e88829_d_n6, assign57060_e88829_d_n7, assign57060_e88829_d_n8, assign57060_e88829_d_n9, assign57060_e88829_d_n10, assign57060_e88829_d_n11, assign57060_e88829_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign57060_e88827: f64 = (locals.var_qn_res__blk1126 / locals.var_t9);
        (assign57060_e88827, (((locals.var_qn_res__blk1126_dn0 * locals.var_t9) - (locals.var_qn_res__blk1126 * locals.var_t9_dn0)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn_res__blk1126_dn2 * locals.var_t9) - (locals.var_qn_res__blk1126 * locals.var_t9_dn2)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn_res__blk1126_dn4 * locals.var_t9) - (locals.var_qn_res__blk1126 * locals.var_t9_dn4)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn_res__blk1126_dn5 * locals.var_t9) - (locals.var_qn_res__blk1126 * locals.var_t9_dn5)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn_res__blk1126_dn6 * locals.var_t9) - (locals.var_qn_res__blk1126 * locals.var_t9_dn6)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn_res__blk1126_dn7 * locals.var_t9) - (locals.var_qn_res__blk1126 * locals.var_t9_dn7)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn_res__blk1126_dn8 * locals.var_t9) - (locals.var_qn_res__blk1126 * locals.var_t9_dn8)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn_res__blk1126_dn9 * locals.var_t9) - (locals.var_qn_res__blk1126 * locals.var_t9_dn9)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn_res__blk1126_dn10 * locals.var_t9) - (locals.var_qn_res__blk1126 * locals.var_t9_dn10)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn_res__blk1126_dn11 * locals.var_t9) - (locals.var_qn_res__blk1126 * locals.var_t9_dn11)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn_res__blk1126_dn14 * locals.var_t9) - (locals.var_qn_res__blk1126 * locals.var_t9_dn14)) / (locals.var_t9 * locals.var_t9)),)
    } else {
        (locals.var_rns, locals.var_rns_dn0, locals.var_rns_dn2, locals.var_rns_dn4, locals.var_rns_dn5, locals.var_rns_dn6, locals.var_rns_dn7, locals.var_rns_dn8, locals.var_rns_dn9, locals.var_rns_dn10, locals.var_rns_dn11, locals.var_rns_dn14,)
    }
};
        locals.var_rns = assign57060_e88829;
        locals.var_rns_dn0 = assign57060_e88829_d_n0;
        locals.var_rns_dn2 = assign57060_e88829_d_n2;
        locals.var_rns_dn4 = assign57060_e88829_d_n4;
        locals.var_rns_dn5 = assign57060_e88829_d_n5;
        locals.var_rns_dn6 = assign57060_e88829_d_n6;
        locals.var_rns_dn7 = assign57060_e88829_d_n7;
        locals.var_rns_dn8 = assign57060_e88829_d_n8;
        locals.var_rns_dn9 = assign57060_e88829_d_n9;
        locals.var_rns_dn10 = assign57060_e88829_d_n10;
        locals.var_rns_dn11 = assign57060_e88829_d_n11;
        locals.var_rns_dn14 = assign57060_e88829_d_n14;
        locals.var_rns_rv = 0.0;

        let (assign57070_e88848, assign57070_e88848_d_n0, assign57070_e88848_d_n2, assign57070_e88848_d_n4, assign57070_e88848_d_n5, assign57070_e88848_d_n6, assign57070_e88848_d_n7, assign57070_e88848_d_n8, assign57070_e88848_d_n9, assign57070_e88848_d_n10, assign57070_e88848_d_n11, assign57070_e88848_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign57070_e88840: f64 = (locals.var_vds_res * locals.var_vds_res);
        let assign57070_e88842: f64 = (assign57070_e88840 + p.p262);
        let assign57070_e88843: f64 = (assign57070_e88842).sqrt();
        let assign57070_e88845: f64 = (p.p262).sqrt();
        let assign57070_e88846: f64 = (assign57070_e88843 - assign57070_e88845);
        (assign57070_e88846, (((locals.var_vds_res_dn0 * locals.var_vds_res) + (locals.var_vds_res * locals.var_vds_res_dn0)) / (2.0 * assign57070_e88843)), (((locals.var_vds_res_dn2 * locals.var_vds_res) + (locals.var_vds_res * locals.var_vds_res_dn2)) / (2.0 * assign57070_e88843)), (((locals.var_vds_res_dn4 * locals.var_vds_res) + (locals.var_vds_res * locals.var_vds_res_dn4)) / (2.0 * assign57070_e88843)), (((locals.var_vds_res_dn5 * locals.var_vds_res) + (locals.var_vds_res * locals.var_vds_res_dn5)) / (2.0 * assign57070_e88843)), (((locals.var_vds_res_dn6 * locals.var_vds_res) + (locals.var_vds_res * locals.var_vds_res_dn6)) / (2.0 * assign57070_e88843)), (((locals.var_vds_res_dn7 * locals.var_vds_res) + (locals.var_vds_res * locals.var_vds_res_dn7)) / (2.0 * assign57070_e88843)), (((locals.var_vds_res_dn8 * locals.var_vds_res) + (locals.var_vds_res * locals.var_vds_res_dn8)) / (2.0 * assign57070_e88843)), (((locals.var_vds_res_dn9 * locals.var_vds_res) + (locals.var_vds_res * locals.var_vds_res_dn9)) / (2.0 * assign57070_e88843)), (((locals.var_vds_res_dn10 * locals.var_vds_res) + (locals.var_vds_res * locals.var_vds_res_dn10)) / (2.0 * assign57070_e88843)), (((locals.var_vds_res_dn11 * locals.var_vds_res) + (locals.var_vds_res * locals.var_vds_res_dn11)) / (2.0 * assign57070_e88843)), (((locals.var_vds_res_dn14 * locals.var_vds_res) + (locals.var_vds_res * locals.var_vds_res_dn14)) / (2.0 * assign57070_e88843)),)
    } else {
        (locals.var_vds_resz, locals.var_vds_resz_dn0, locals.var_vds_resz_dn2, locals.var_vds_resz_dn4, locals.var_vds_resz_dn5, locals.var_vds_resz_dn6, locals.var_vds_resz_dn7, locals.var_vds_resz_dn8, locals.var_vds_resz_dn9, locals.var_vds_resz_dn10, locals.var_vds_resz_dn11, locals.var_vds_resz_dn14,)
    }
};
        locals.var_vds_resz = assign57070_e88848;
        locals.var_vds_resz_dn0 = assign57070_e88848_d_n0;
        locals.var_vds_resz_dn2 = assign57070_e88848_d_n2;
        locals.var_vds_resz_dn4 = assign57070_e88848_d_n4;
        locals.var_vds_resz_dn5 = assign57070_e88848_d_n5;
        locals.var_vds_resz_dn6 = assign57070_e88848_d_n6;
        locals.var_vds_resz_dn7 = assign57070_e88848_d_n7;
        locals.var_vds_resz_dn8 = assign57070_e88848_d_n8;
        locals.var_vds_resz_dn9 = assign57070_e88848_d_n9;
        locals.var_vds_resz_dn10 = assign57070_e88848_d_n10;
        locals.var_vds_resz_dn11 = assign57070_e88848_d_n11;
        locals.var_vds_resz_dn14 = assign57070_e88848_d_n14;
        locals.var_vds_resz_rv = 0.0;

        let (assign57080_e88863, assign57080_e88863_d_n0, assign57080_e88863_d_n2, assign57080_e88863_d_n4, assign57080_e88863_d_n5, assign57080_e88863_d_n6, assign57080_e88863_d_n7, assign57080_e88863_d_n8, assign57080_e88863_d_n9, assign57080_e88863_d_n10, assign57080_e88863_d_n11, assign57080_e88863_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign57080_e88860: f64 = (locals.var_vds_resz * locals.var_ninvdecres);
        let assign57080_e88861: f64 = (1.0 + assign57080_e88860);
        (assign57080_e88861, ((locals.var_vds_resz_dn0 * locals.var_ninvdecres) + (locals.var_vds_resz * locals.var_ninvdecres_dn0)), ((locals.var_vds_resz_dn2 * locals.var_ninvdecres) + (locals.var_vds_resz * locals.var_ninvdecres_dn2)), ((locals.var_vds_resz_dn4 * locals.var_ninvdecres) + (locals.var_vds_resz * locals.var_ninvdecres_dn4)), ((locals.var_vds_resz_dn5 * locals.var_ninvdecres) + (locals.var_vds_resz * locals.var_ninvdecres_dn5)), ((locals.var_vds_resz_dn6 * locals.var_ninvdecres) + (locals.var_vds_resz * locals.var_ninvdecres_dn6)), ((locals.var_vds_resz_dn7 * locals.var_ninvdecres) + (locals.var_vds_resz * locals.var_ninvdecres_dn7)), ((locals.var_vds_resz_dn8 * locals.var_ninvdecres) + (locals.var_vds_resz * locals.var_ninvdecres_dn8)), ((locals.var_vds_resz_dn9 * locals.var_ninvdecres) + (locals.var_vds_resz * locals.var_ninvdecres_dn9)), ((locals.var_vds_resz_dn10 * locals.var_ninvdecres) + (locals.var_vds_resz * locals.var_ninvdecres_dn10)), ((locals.var_vds_resz_dn11 * locals.var_ninvdecres) + (locals.var_vds_resz * locals.var_ninvdecres_dn11)), ((locals.var_vds_resz_dn14 * locals.var_ninvdecres) + (locals.var_vds_resz * locals.var_ninvdecres_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign57080_e88863;
        locals.var_t4_dn0 = assign57080_e88863_d_n0;
        locals.var_t4_dn2 = assign57080_e88863_d_n2;
        locals.var_t4_dn4 = assign57080_e88863_d_n4;
        locals.var_t4_dn5 = assign57080_e88863_d_n5;
        locals.var_t4_dn6 = assign57080_e88863_d_n6;
        locals.var_t4_dn7 = assign57080_e88863_d_n7;
        locals.var_t4_dn8 = assign57080_e88863_d_n8;
        locals.var_t4_dn9 = assign57080_e88863_d_n9;
        locals.var_t4_dn10 = assign57080_e88863_d_n10;
        locals.var_t4_dn11 = assign57080_e88863_d_n11;
        locals.var_t4_dn14 = assign57080_e88863_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign57090_e88878, assign57090_e88878_d_n0, assign57090_e88878_d_n2, assign57090_e88878_d_n4, assign57090_e88878_d_n5, assign57090_e88878_d_n6, assign57090_e88878_d_n7, assign57090_e88878_d_n8, assign57090_e88878_d_n9, assign57090_e88878_d_n10, assign57090_e88878_d_n11, assign57090_e88878_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign57090_e88875: f64 = (locals.var_vds_resz * locals.var_ninvdehres);
        let assign57090_e88876: f64 = (1.0 + assign57090_e88875);
        (assign57090_e88876, ((locals.var_vds_resz_dn0 * locals.var_ninvdehres) + (locals.var_vds_resz * locals.var_ninvdehres_dn0)), ((locals.var_vds_resz_dn2 * locals.var_ninvdehres) + (locals.var_vds_resz * locals.var_ninvdehres_dn2)), ((locals.var_vds_resz_dn4 * locals.var_ninvdehres) + (locals.var_vds_resz * locals.var_ninvdehres_dn4)), ((locals.var_vds_resz_dn5 * locals.var_ninvdehres) + (locals.var_vds_resz * locals.var_ninvdehres_dn5)), ((locals.var_vds_resz_dn6 * locals.var_ninvdehres) + (locals.var_vds_resz * locals.var_ninvdehres_dn6)), ((locals.var_vds_resz_dn7 * locals.var_ninvdehres) + (locals.var_vds_resz * locals.var_ninvdehres_dn7)), ((locals.var_vds_resz_dn8 * locals.var_ninvdehres) + (locals.var_vds_resz * locals.var_ninvdehres_dn8)), ((locals.var_vds_resz_dn9 * locals.var_ninvdehres) + (locals.var_vds_resz * locals.var_ninvdehres_dn9)), ((locals.var_vds_resz_dn10 * locals.var_ninvdehres) + (locals.var_vds_resz * locals.var_ninvdehres_dn10)), ((locals.var_vds_resz_dn11 * locals.var_ninvdehres) + (locals.var_vds_resz * locals.var_ninvdehres_dn11)), ((locals.var_vds_resz_dn14 * locals.var_ninvdehres) + (locals.var_vds_resz * locals.var_ninvdehres_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign57090_e88878;
        locals.var_t5_dn0 = assign57090_e88878_d_n0;
        locals.var_t5_dn2 = assign57090_e88878_d_n2;
        locals.var_t5_dn4 = assign57090_e88878_d_n4;
        locals.var_t5_dn5 = assign57090_e88878_d_n5;
        locals.var_t5_dn6 = assign57090_e88878_d_n6;
        locals.var_t5_dn7 = assign57090_e88878_d_n7;
        locals.var_t5_dn8 = assign57090_e88878_d_n8;
        locals.var_t5_dn9 = assign57090_e88878_d_n9;
        locals.var_t5_dn10 = assign57090_e88878_d_n10;
        locals.var_t5_dn11 = assign57090_e88878_d_n11;
        locals.var_t5_dn14 = assign57090_e88878_d_n14;
        locals.var_t5_rv = 0.0;

        let assign57100_e88880: f64 = if param_given[408] { 1.0 } else { 0.0 };
        locals.var_guard1424 = assign57100_e88880;
        locals.var_guard1424_rv = 0.0;

        let (assign57110_e88901, assign57110_e88901_d_n0, assign57110_e88901_d_n2, assign57110_e88901_d_n4, assign57110_e88901_d_n5, assign57110_e88901_d_n6, assign57110_e88901_d_n7, assign57110_e88901_d_n8, assign57110_e88901_d_n9, assign57110_e88901_d_n10, assign57110_e88901_d_n11, assign57110_e88901_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1424 != 0.0)) {
        let assign57110_e88893: f64 = (p.p408 - locals.var_phi_b0_dep__blk1094);
        let assign57110_e88896: f64 = (100.0 * locals.var_uc_depthn);
        let assign57110_e88897: f64 = (assign57110_e88893 / assign57110_e88896);
        let assign57110_e88899: f64 = (assign57110_e88897 / locals.var_t5);
        (assign57110_e88899, (((((((-locals.var_phi_b0_dep__blk1094_dn0) * assign57110_e88896) - (assign57110_e88893 * (100.0 * locals.var_uc_depthn_dn0))) / (assign57110_e88896 * assign57110_e88896)) * locals.var_t5) - (assign57110_e88897 * locals.var_t5_dn0)) / (locals.var_t5 * locals.var_t5)), (((((((-locals.var_phi_b0_dep__blk1094_dn2) * assign57110_e88896) - (assign57110_e88893 * (100.0 * locals.var_uc_depthn_dn2))) / (assign57110_e88896 * assign57110_e88896)) * locals.var_t5) - (assign57110_e88897 * locals.var_t5_dn2)) / (locals.var_t5 * locals.var_t5)), (((((((-locals.var_phi_b0_dep__blk1094_dn4) * assign57110_e88896) - (assign57110_e88893 * (100.0 * locals.var_uc_depthn_dn4))) / (assign57110_e88896 * assign57110_e88896)) * locals.var_t5) - (assign57110_e88897 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)), (((((((-locals.var_phi_b0_dep__blk1094_dn5) * assign57110_e88896) - (assign57110_e88893 * (100.0 * locals.var_uc_depthn_dn5))) / (assign57110_e88896 * assign57110_e88896)) * locals.var_t5) - (assign57110_e88897 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)), (((((((-locals.var_phi_b0_dep__blk1094_dn6) * assign57110_e88896) - (assign57110_e88893 * (100.0 * locals.var_uc_depthn_dn6))) / (assign57110_e88896 * assign57110_e88896)) * locals.var_t5) - (assign57110_e88897 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)), (((((((-locals.var_phi_b0_dep__blk1094_dn7) * assign57110_e88896) - (assign57110_e88893 * (100.0 * locals.var_uc_depthn_dn7))) / (assign57110_e88896 * assign57110_e88896)) * locals.var_t5) - (assign57110_e88897 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)), (((((((-locals.var_phi_b0_dep__blk1094_dn8) * assign57110_e88896) - (assign57110_e88893 * (100.0 * locals.var_uc_depthn_dn8))) / (assign57110_e88896 * assign57110_e88896)) * locals.var_t5) - (assign57110_e88897 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)), (((((((-locals.var_phi_b0_dep__blk1094_dn9) * assign57110_e88896) - (assign57110_e88893 * (100.0 * locals.var_uc_depthn_dn9))) / (assign57110_e88896 * assign57110_e88896)) * locals.var_t5) - (assign57110_e88897 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)), (((((((-locals.var_phi_b0_dep__blk1094_dn10) * assign57110_e88896) - (assign57110_e88893 * (100.0 * locals.var_uc_depthn_dn10))) / (assign57110_e88896 * assign57110_e88896)) * locals.var_t5) - (assign57110_e88897 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)), (((((((-locals.var_phi_b0_dep__blk1094_dn11) * assign57110_e88896) - (assign57110_e88893 * (100.0 * locals.var_uc_depthn_dn11))) / (assign57110_e88896 * assign57110_e88896)) * locals.var_t5) - (assign57110_e88897 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)), (((((((-locals.var_phi_b0_dep__blk1094_dn14) * assign57110_e88896) - (assign57110_e88893 * (100.0 * locals.var_uc_depthn_dn14))) / (assign57110_e88896 * assign57110_e88896)) * locals.var_t5) - (assign57110_e88897 * locals.var_t5_dn14)) / (locals.var_t5 * locals.var_t5)),)
    } else {
        (locals.var_eeff_res, locals.var_eeff_res_dn0, locals.var_eeff_res_dn2, locals.var_eeff_res_dn4, locals.var_eeff_res_dn5, locals.var_eeff_res_dn6, locals.var_eeff_res_dn7, locals.var_eeff_res_dn8, locals.var_eeff_res_dn9, locals.var_eeff_res_dn10, locals.var_eeff_res_dn11, locals.var_eeff_res_dn14,)
    }
};
        locals.var_eeff_res = assign57110_e88901;
        locals.var_eeff_res_dn0 = assign57110_e88901_d_n0;
        locals.var_eeff_res_dn2 = assign57110_e88901_d_n2;
        locals.var_eeff_res_dn4 = assign57110_e88901_d_n4;
        locals.var_eeff_res_dn5 = assign57110_e88901_d_n5;
        locals.var_eeff_res_dn6 = assign57110_e88901_d_n6;
        locals.var_eeff_res_dn7 = assign57110_e88901_d_n7;
        locals.var_eeff_res_dn8 = assign57110_e88901_d_n8;
        locals.var_eeff_res_dn9 = assign57110_e88901_d_n9;
        locals.var_eeff_res_dn10 = assign57110_e88901_d_n10;
        locals.var_eeff_res_dn11 = assign57110_e88901_d_n11;
        locals.var_eeff_res_dn14 = assign57110_e88901_d_n14;
        locals.var_eeff_res_rv = 0.0;

        let (assign57120_e88919, assign57120_e88919_d_n0, assign57120_e88919_d_n2, assign57120_e88919_d_n4, assign57120_e88919_d_n5, assign57120_e88919_d_n6, assign57120_e88919_d_n7, assign57120_e88919_d_n8, assign57120_e88919_d_n9, assign57120_e88919_d_n10, assign57120_e88919_d_n11, assign57120_e88919_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1424 == 0.0)) {
        let assign57120_e88915: f64 = (locals.var_qn_res__blk1126 / 1.034943e-10);
        let assign57120_e88917: f64 = (assign57120_e88915 / locals.var_t5);
        (assign57120_e88917, ((((locals.var_qn_res__blk1126_dn0 / 1.034943e-10) * locals.var_t5) - (assign57120_e88915 * locals.var_t5_dn0)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_qn_res__blk1126_dn2 / 1.034943e-10) * locals.var_t5) - (assign57120_e88915 * locals.var_t5_dn2)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_qn_res__blk1126_dn4 / 1.034943e-10) * locals.var_t5) - (assign57120_e88915 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_qn_res__blk1126_dn5 / 1.034943e-10) * locals.var_t5) - (assign57120_e88915 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_qn_res__blk1126_dn6 / 1.034943e-10) * locals.var_t5) - (assign57120_e88915 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_qn_res__blk1126_dn7 / 1.034943e-10) * locals.var_t5) - (assign57120_e88915 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_qn_res__blk1126_dn8 / 1.034943e-10) * locals.var_t5) - (assign57120_e88915 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_qn_res__blk1126_dn9 / 1.034943e-10) * locals.var_t5) - (assign57120_e88915 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_qn_res__blk1126_dn10 / 1.034943e-10) * locals.var_t5) - (assign57120_e88915 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_qn_res__blk1126_dn11 / 1.034943e-10) * locals.var_t5) - (assign57120_e88915 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_qn_res__blk1126_dn14 / 1.034943e-10) * locals.var_t5) - (assign57120_e88915 * locals.var_t5_dn14)) / (locals.var_t5 * locals.var_t5)),)
    } else {
        (locals.var_eeff_res, locals.var_eeff_res_dn0, locals.var_eeff_res_dn2, locals.var_eeff_res_dn4, locals.var_eeff_res_dn5, locals.var_eeff_res_dn6, locals.var_eeff_res_dn7, locals.var_eeff_res_dn8, locals.var_eeff_res_dn9, locals.var_eeff_res_dn10, locals.var_eeff_res_dn11, locals.var_eeff_res_dn14,)
    }
};
        locals.var_eeff_res = assign57120_e88919;
        locals.var_eeff_res_dn0 = assign57120_e88919_d_n0;
        locals.var_eeff_res_dn2 = assign57120_e88919_d_n2;
        locals.var_eeff_res_dn4 = assign57120_e88919_d_n4;
        locals.var_eeff_res_dn5 = assign57120_e88919_d_n5;
        locals.var_eeff_res_dn6 = assign57120_e88919_d_n6;
        locals.var_eeff_res_dn7 = assign57120_e88919_d_n7;
        locals.var_eeff_res_dn8 = assign57120_e88919_d_n8;
        locals.var_eeff_res_dn9 = assign57120_e88919_d_n9;
        locals.var_eeff_res_dn10 = assign57120_e88919_d_n10;
        locals.var_eeff_res_dn11 = assign57120_e88919_d_n11;
        locals.var_eeff_res_dn14 = assign57120_e88919_d_n14;
        locals.var_eeff_res_rv = 0.0;

        let (assign57130_e88937, assign57130_e88937_d_n0, assign57130_e88937_d_n2, assign57130_e88937_d_n4, assign57130_e88937_d_n5, assign57130_e88937_d_n6, assign57130_e88937_d_n7, assign57130_e88937_d_n8, assign57130_e88937_d_n9, assign57130_e88937_d_n10, assign57130_e88937_d_n11, assign57130_e88937_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let (assign57130_e88935, assign57130_e88935_d_n0, assign57130_e88935_d_n2, assign57130_e88935_d_n4, assign57130_e88935_d_n5, assign57130_e88935_d_n6, assign57130_e88935_d_n7, assign57130_e88935_d_n8, assign57130_e88935_d_n9, assign57130_e88935_d_n10, assign57130_e88935_d_n11, assign57130_e88935_d_n14,) = {
            if (locals.var_eeff_res == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign57130_e88934: f64 = (locals.var_eeff_res).powf(p.p376);
                (assign57130_e88934, if 0.0 == 0.0 && ((p.p376) as f64).is_finite() && ((p.p376) as f64).fract() == 0.0 { if p.p376 == 0.0 { 0.0 } else { (p.p376 * ((locals.var_eeff_res).powf(p.p376 - 1.0) * locals.var_eeff_res_dn0)) } } else { (assign57130_e88934 * (p.p376 * (locals.var_eeff_res_dn0 / locals.var_eeff_res))) }, if 0.0 == 0.0 && ((p.p376) as f64).is_finite() && ((p.p376) as f64).fract() == 0.0 { if p.p376 == 0.0 { 0.0 } else { (p.p376 * ((locals.var_eeff_res).powf(p.p376 - 1.0) * locals.var_eeff_res_dn2)) } } else { (assign57130_e88934 * (p.p376 * (locals.var_eeff_res_dn2 / locals.var_eeff_res))) }, if 0.0 == 0.0 && ((p.p376) as f64).is_finite() && ((p.p376) as f64).fract() == 0.0 { if p.p376 == 0.0 { 0.0 } else { (p.p376 * ((locals.var_eeff_res).powf(p.p376 - 1.0) * locals.var_eeff_res_dn4)) } } else { (assign57130_e88934 * (p.p376 * (locals.var_eeff_res_dn4 / locals.var_eeff_res))) }, if 0.0 == 0.0 && ((p.p376) as f64).is_finite() && ((p.p376) as f64).fract() == 0.0 { if p.p376 == 0.0 { 0.0 } else { (p.p376 * ((locals.var_eeff_res).powf(p.p376 - 1.0) * locals.var_eeff_res_dn5)) } } else { (assign57130_e88934 * (p.p376 * (locals.var_eeff_res_dn5 / locals.var_eeff_res))) }, if 0.0 == 0.0 && ((p.p376) as f64).is_finite() && ((p.p376) as f64).fract() == 0.0 { if p.p376 == 0.0 { 0.0 } else { (p.p376 * ((locals.var_eeff_res).powf(p.p376 - 1.0) * locals.var_eeff_res_dn6)) } } else { (assign57130_e88934 * (p.p376 * (locals.var_eeff_res_dn6 / locals.var_eeff_res))) }, if 0.0 == 0.0 && ((p.p376) as f64).is_finite() && ((p.p376) as f64).fract() == 0.0 { if p.p376 == 0.0 { 0.0 } else { (p.p376 * ((locals.var_eeff_res).powf(p.p376 - 1.0) * locals.var_eeff_res_dn7)) } } else { (assign57130_e88934 * (p.p376 * (locals.var_eeff_res_dn7 / locals.var_eeff_res))) }, if 0.0 == 0.0 && ((p.p376) as f64).is_finite() && ((p.p376) as f64).fract() == 0.0 { if p.p376 == 0.0 { 0.0 } else { (p.p376 * ((locals.var_eeff_res).powf(p.p376 - 1.0) * locals.var_eeff_res_dn8)) } } else { (assign57130_e88934 * (p.p376 * (locals.var_eeff_res_dn8 / locals.var_eeff_res))) }, if 0.0 == 0.0 && ((p.p376) as f64).is_finite() && ((p.p376) as f64).fract() == 0.0 { if p.p376 == 0.0 { 0.0 } else { (p.p376 * ((locals.var_eeff_res).powf(p.p376 - 1.0) * locals.var_eeff_res_dn9)) } } else { (assign57130_e88934 * (p.p376 * (locals.var_eeff_res_dn9 / locals.var_eeff_res))) }, if 0.0 == 0.0 && ((p.p376) as f64).is_finite() && ((p.p376) as f64).fract() == 0.0 { if p.p376 == 0.0 { 0.0 } else { (p.p376 * ((locals.var_eeff_res).powf(p.p376 - 1.0) * locals.var_eeff_res_dn10)) } } else { (assign57130_e88934 * (p.p376 * (locals.var_eeff_res_dn10 / locals.var_eeff_res))) }, if 0.0 == 0.0 && ((p.p376) as f64).is_finite() && ((p.p376) as f64).fract() == 0.0 { if p.p376 == 0.0 { 0.0 } else { (p.p376 * ((locals.var_eeff_res).powf(p.p376 - 1.0) * locals.var_eeff_res_dn11)) } } else { (assign57130_e88934 * (p.p376 * (locals.var_eeff_res_dn11 / locals.var_eeff_res))) }, if 0.0 == 0.0 && ((p.p376) as f64).is_finite() && ((p.p376) as f64).fract() == 0.0 { if p.p376 == 0.0 { 0.0 } else { (p.p376 * ((locals.var_eeff_res).powf(p.p376 - 1.0) * locals.var_eeff_res_dn14)) } } else { (assign57130_e88934 * (p.p376 * (locals.var_eeff_res_dn14 / locals.var_eeff_res))) },)
            }
        };
        (assign57130_e88935, assign57130_e88935_d_n0, assign57130_e88935_d_n2, assign57130_e88935_d_n4, assign57130_e88935_d_n5, assign57130_e88935_d_n6, assign57130_e88935_d_n7, assign57130_e88935_d_n8, assign57130_e88935_d_n9, assign57130_e88935_d_n10, assign57130_e88935_d_n11, assign57130_e88935_d_n14,)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign57130_e88937;
        locals.var_t8_dn0 = assign57130_e88937_d_n0;
        locals.var_t8_dn2 = assign57130_e88937_d_n2;
        locals.var_t8_dn4 = assign57130_e88937_d_n4;
        locals.var_t8_dn5 = assign57130_e88937_d_n5;
        locals.var_t8_dn6 = assign57130_e88937_d_n6;
        locals.var_t8_dn7 = assign57130_e88937_d_n7;
        locals.var_t8_dn8 = assign57130_e88937_d_n8;
        locals.var_t8_dn9 = assign57130_e88937_d_n9;
        locals.var_t8_dn10 = assign57130_e88937_d_n10;
        locals.var_t8_dn11 = assign57130_e88937_d_n11;
        locals.var_t8_dn14 = assign57130_e88937_d_n14;
        locals.var_t8_rv = 0.0;

        let (assign57140_e88968, assign57140_e88968_d_n0, assign57140_e88968_d_n2, assign57140_e88968_d_n4, assign57140_e88968_d_n5, assign57140_e88968_d_n6, assign57140_e88968_d_n7, assign57140_e88968_d_n8, assign57140_e88968_d_n9, assign57140_e88968_d_n10, assign57140_e88968_d_n11, assign57140_e88968_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign57140_e88948: f64 = 1.0;
        let assign57140_e88952: f64 = (locals.var_uc_depmue1 * locals.var_t4);
        let assign57140_e88954: f64 = (assign57140_e88952 * locals.var_rns);
        let assign57140_e88956: f64 = (assign57140_e88954 / 10000000000.0);
        let assign57140_e88957: f64 = (locals.var_uc_depmue0 + assign57140_e88956);
        let assign57140_e88959: f64 = (assign57140_e88957 + 1e-25);
        let assign57140_e88960: f64 = (assign57140_e88948 / assign57140_e88959);
        let assign57140_e88963: f64 = locals.var_depmphn0;
        let assign57140_e88965: f64 = (assign57140_e88963 * locals.var_t8);
        let assign57140_e88966: f64 = (assign57140_e88960 + assign57140_e88965);
        (assign57140_e88966, ((-((assign57140_e88948 * (locals.var_uc_depmue0_dn0 + (((((locals.var_uc_depmue1_dn0 * locals.var_t4) + (locals.var_uc_depmue1 * locals.var_t4_dn0)) * locals.var_rns) + (assign57140_e88952 * locals.var_rns_dn0)) / 10000000000.0))) / (assign57140_e88959 * assign57140_e88959))) + ((locals.var_depmphn0_dn0 * locals.var_t8) + (assign57140_e88963 * locals.var_t8_dn0))), ((-((assign57140_e88948 * (locals.var_uc_depmue0_dn2 + (((((locals.var_uc_depmue1_dn2 * locals.var_t4) + (locals.var_uc_depmue1 * locals.var_t4_dn2)) * locals.var_rns) + (assign57140_e88952 * locals.var_rns_dn2)) / 10000000000.0))) / (assign57140_e88959 * assign57140_e88959))) + ((locals.var_depmphn0_dn2 * locals.var_t8) + (assign57140_e88963 * locals.var_t8_dn2))), ((-((assign57140_e88948 * (locals.var_uc_depmue0_dn4 + (((((locals.var_uc_depmue1_dn4 * locals.var_t4) + (locals.var_uc_depmue1 * locals.var_t4_dn4)) * locals.var_rns) + (assign57140_e88952 * locals.var_rns_dn4)) / 10000000000.0))) / (assign57140_e88959 * assign57140_e88959))) + ((locals.var_depmphn0_dn4 * locals.var_t8) + (assign57140_e88963 * locals.var_t8_dn4))), ((-((assign57140_e88948 * (locals.var_uc_depmue0_dn5 + (((((locals.var_uc_depmue1_dn5 * locals.var_t4) + (locals.var_uc_depmue1 * locals.var_t4_dn5)) * locals.var_rns) + (assign57140_e88952 * locals.var_rns_dn5)) / 10000000000.0))) / (assign57140_e88959 * assign57140_e88959))) + ((locals.var_depmphn0_dn5 * locals.var_t8) + (assign57140_e88963 * locals.var_t8_dn5))), ((-((assign57140_e88948 * (locals.var_uc_depmue0_dn6 + (((((locals.var_uc_depmue1_dn6 * locals.var_t4) + (locals.var_uc_depmue1 * locals.var_t4_dn6)) * locals.var_rns) + (assign57140_e88952 * locals.var_rns_dn6)) / 10000000000.0))) / (assign57140_e88959 * assign57140_e88959))) + ((locals.var_depmphn0_dn6 * locals.var_t8) + (assign57140_e88963 * locals.var_t8_dn6))), ((-((assign57140_e88948 * (locals.var_uc_depmue0_dn7 + (((((locals.var_uc_depmue1_dn7 * locals.var_t4) + (locals.var_uc_depmue1 * locals.var_t4_dn7)) * locals.var_rns) + (assign57140_e88952 * locals.var_rns_dn7)) / 10000000000.0))) / (assign57140_e88959 * assign57140_e88959))) + ((locals.var_depmphn0_dn7 * locals.var_t8) + (assign57140_e88963 * locals.var_t8_dn7))), ((-((assign57140_e88948 * (locals.var_uc_depmue0_dn8 + (((((locals.var_uc_depmue1_dn8 * locals.var_t4) + (locals.var_uc_depmue1 * locals.var_t4_dn8)) * locals.var_rns) + (assign57140_e88952 * locals.var_rns_dn8)) / 10000000000.0))) / (assign57140_e88959 * assign57140_e88959))) + ((locals.var_depmphn0_dn8 * locals.var_t8) + (assign57140_e88963 * locals.var_t8_dn8))), ((-((assign57140_e88948 * (locals.var_uc_depmue0_dn9 + (((((locals.var_uc_depmue1_dn9 * locals.var_t4) + (locals.var_uc_depmue1 * locals.var_t4_dn9)) * locals.var_rns) + (assign57140_e88952 * locals.var_rns_dn9)) / 10000000000.0))) / (assign57140_e88959 * assign57140_e88959))) + ((locals.var_depmphn0_dn9 * locals.var_t8) + (assign57140_e88963 * locals.var_t8_dn9))), ((-((assign57140_e88948 * (locals.var_uc_depmue0_dn10 + (((((locals.var_uc_depmue1_dn10 * locals.var_t4) + (locals.var_uc_depmue1 * locals.var_t4_dn10)) * locals.var_rns) + (assign57140_e88952 * locals.var_rns_dn10)) / 10000000000.0))) / (assign57140_e88959 * assign57140_e88959))) + ((locals.var_depmphn0_dn10 * locals.var_t8) + (assign57140_e88963 * locals.var_t8_dn10))), ((-((assign57140_e88948 * (locals.var_uc_depmue0_dn11 + (((((locals.var_uc_depmue1_dn11 * locals.var_t4) + (locals.var_uc_depmue1 * locals.var_t4_dn11)) * locals.var_rns) + (assign57140_e88952 * locals.var_rns_dn11)) / 10000000000.0))) / (assign57140_e88959 * assign57140_e88959))) + ((locals.var_depmphn0_dn11 * locals.var_t8) + (assign57140_e88963 * locals.var_t8_dn11))), ((-((assign57140_e88948 * (locals.var_uc_depmue0_dn14 + (((((locals.var_uc_depmue1_dn14 * locals.var_t4) + (locals.var_uc_depmue1 * locals.var_t4_dn14)) * locals.var_rns) + (assign57140_e88952 * locals.var_rns_dn14)) / 10000000000.0))) / (assign57140_e88959 * assign57140_e88959))) + ((locals.var_depmphn0_dn14 * locals.var_t8) + (assign57140_e88963 * locals.var_t8_dn14))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign57140_e88968;
        locals.var_t1_dn0 = assign57140_e88968_d_n0;
        locals.var_t1_dn2 = assign57140_e88968_d_n2;
        locals.var_t1_dn4 = assign57140_e88968_d_n4;
        locals.var_t1_dn5 = assign57140_e88968_d_n5;
        locals.var_t1_dn6 = assign57140_e88968_d_n6;
        locals.var_t1_dn7 = assign57140_e88968_d_n7;
        locals.var_t1_dn8 = assign57140_e88968_d_n8;
        locals.var_t1_dn9 = assign57140_e88968_d_n9;
        locals.var_t1_dn10 = assign57140_e88968_d_n10;
        locals.var_t1_dn11 = assign57140_e88968_d_n11;
        locals.var_t1_dn14 = assign57140_e88968_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign57150_e88981, assign57150_e88981_d_n0, assign57150_e88981_d_n2, assign57150_e88981_d_n4, assign57150_e88981_d_n5, assign57150_e88981_d_n6, assign57150_e88981_d_n7, assign57150_e88981_d_n8, assign57150_e88981_d_n9, assign57150_e88981_d_n10, assign57150_e88981_d_n11, assign57150_e88981_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign57150_e88979: f64 = (1.0 / locals.var_t1);
        (assign57150_e88979, (-(locals.var_t1_dn0 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn2 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn4 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn5 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn6 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn7 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn8 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn9 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn10 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn11 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn14 / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn11, locals.var_muun_dn14,)
    }
};
        locals.var_muun = assign57150_e88981;
        locals.var_muun_dn0 = assign57150_e88981_d_n0;
        locals.var_muun_dn2 = assign57150_e88981_d_n2;
        locals.var_muun_dn4 = assign57150_e88981_d_n4;
        locals.var_muun_dn5 = assign57150_e88981_d_n5;
        locals.var_muun_dn6 = assign57150_e88981_d_n6;
        locals.var_muun_dn7 = assign57150_e88981_d_n7;
        locals.var_muun_dn8 = assign57150_e88981_d_n8;
        locals.var_muun_dn9 = assign57150_e88981_d_n9;
        locals.var_muun_dn10 = assign57150_e88981_d_n10;
        locals.var_muun_dn11 = assign57150_e88981_d_n11;
        locals.var_muun_dn14 = assign57150_e88981_d_n14;
        locals.var_muun_rv = 0.0;

        let (assign57160_e88994, assign57160_e88994_d_n0, assign57160_e88994_d_n2, assign57160_e88994_d_n4, assign57160_e88994_d_n5, assign57160_e88994_d_n6, assign57160_e88994_d_n7, assign57160_e88994_d_n8, assign57160_e88994_d_n9, assign57160_e88994_d_n10, assign57160_e88994_d_n11, assign57160_e88994_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign57160_e88992: f64 = (locals.var_muun / 10000.0);
        (assign57160_e88992, (locals.var_muun_dn0 / 10000.0), (locals.var_muun_dn2 / 10000.0), (locals.var_muun_dn4 / 10000.0), (locals.var_muun_dn5 / 10000.0), (locals.var_muun_dn6 / 10000.0), (locals.var_muun_dn7 / 10000.0), (locals.var_muun_dn8 / 10000.0), (locals.var_muun_dn9 / 10000.0), (locals.var_muun_dn10 / 10000.0), (locals.var_muun_dn11 / 10000.0), (locals.var_muun_dn14 / 10000.0),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn11, locals.var_muun_dn14,)
    }
};
        locals.var_muun = assign57160_e88994;
        locals.var_muun_dn0 = assign57160_e88994_d_n0;
        locals.var_muun_dn2 = assign57160_e88994_d_n2;
        locals.var_muun_dn4 = assign57160_e88994_d_n4;
        locals.var_muun_dn5 = assign57160_e88994_d_n5;
        locals.var_muun_dn6 = assign57160_e88994_d_n6;
        locals.var_muun_dn7 = assign57160_e88994_d_n7;
        locals.var_muun_dn8 = assign57160_e88994_d_n8;
        locals.var_muun_dn9 = assign57160_e88994_d_n9;
        locals.var_muun_dn10 = assign57160_e88994_d_n10;
        locals.var_muun_dn11 = assign57160_e88994_d_n11;
        locals.var_muun_dn14 = assign57160_e88994_d_n14;
        locals.var_muun_rv = 0.0;

        let (assign57170_e89009, assign57170_e89009_d_n0, assign57170_e89009_d_n2, assign57170_e89009_d_n4, assign57170_e89009_d_n5, assign57170_e89009_d_n6, assign57170_e89009_d_n7, assign57170_e89009_d_n8, assign57170_e89009_d_n9, assign57170_e89009_d_n10, assign57170_e89009_d_n11, assign57170_e89009_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign57170_e89006: f64 = (locals.var_leff + p.p401);
        let assign57170_e89007: f64 = (locals.var_vds_res / assign57170_e89006);
        (assign57170_e89007, (locals.var_vds_res_dn0 / assign57170_e89006), (locals.var_vds_res_dn2 / assign57170_e89006), (locals.var_vds_res_dn4 / assign57170_e89006), (locals.var_vds_res_dn5 / assign57170_e89006), (locals.var_vds_res_dn6 / assign57170_e89006), (locals.var_vds_res_dn7 / assign57170_e89006), (locals.var_vds_res_dn8 / assign57170_e89006), (locals.var_vds_res_dn9 / assign57170_e89006), (locals.var_vds_res_dn10 / assign57170_e89006), (locals.var_vds_res_dn11 / assign57170_e89006), (locals.var_vds_res_dn14 / assign57170_e89006),)
    } else {
        (locals.var_edri__blk1117, locals.var_edri__blk1117_dn0, locals.var_edri__blk1117_dn2, locals.var_edri__blk1117_dn4, locals.var_edri__blk1117_dn5, locals.var_edri__blk1117_dn6, locals.var_edri__blk1117_dn7, locals.var_edri__blk1117_dn8, locals.var_edri__blk1117_dn9, locals.var_edri__blk1117_dn10, locals.var_edri__blk1117_dn11, locals.var_edri__blk1117_dn14,)
    }
};
        locals.var_edri__blk1117 = assign57170_e89009;
        locals.var_edri__blk1117_dn0 = assign57170_e89009_d_n0;
        locals.var_edri__blk1117_dn2 = assign57170_e89009_d_n2;
        locals.var_edri__blk1117_dn4 = assign57170_e89009_d_n4;
        locals.var_edri__blk1117_dn5 = assign57170_e89009_d_n5;
        locals.var_edri__blk1117_dn6 = assign57170_e89009_d_n6;
        locals.var_edri__blk1117_dn7 = assign57170_e89009_d_n7;
        locals.var_edri__blk1117_dn8 = assign57170_e89009_d_n8;
        locals.var_edri__blk1117_dn9 = assign57170_e89009_d_n9;
        locals.var_edri__blk1117_dn10 = assign57170_e89009_d_n10;
        locals.var_edri__blk1117_dn11 = assign57170_e89009_d_n11;
        locals.var_edri__blk1117_dn14 = assign57170_e89009_d_n14;
        locals.var_edri__blk1117_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_209(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign57180_e89022, assign57180_e89022_d_n0, assign57180_e89022_d_n2, assign57180_e89022_d_n4, assign57180_e89022_d_n5, assign57180_e89022_d_n6, assign57180_e89022_d_n7, assign57180_e89022_d_n8, assign57180_e89022_d_n9, assign57180_e89022_d_n10, assign57180_e89022_d_n11, assign57180_e89022_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign57180_e89020: f64 = (locals.var_vds_res).powf(2.0);
        (assign57180_e89020, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn0)) } } else { (assign57180_e89020 * (2.0 * (locals.var_vds_res_dn0 / locals.var_vds_res))) }, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn2)) } } else { (assign57180_e89020 * (2.0 * (locals.var_vds_res_dn2 / locals.var_vds_res))) }, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn4)) } } else { (assign57180_e89020 * (2.0 * (locals.var_vds_res_dn4 / locals.var_vds_res))) }, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn5)) } } else { (assign57180_e89020 * (2.0 * (locals.var_vds_res_dn5 / locals.var_vds_res))) }, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn6)) } } else { (assign57180_e89020 * (2.0 * (locals.var_vds_res_dn6 / locals.var_vds_res))) }, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn7)) } } else { (assign57180_e89020 * (2.0 * (locals.var_vds_res_dn7 / locals.var_vds_res))) }, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn8)) } } else { (assign57180_e89020 * (2.0 * (locals.var_vds_res_dn8 / locals.var_vds_res))) }, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn9)) } } else { (assign57180_e89020 * (2.0 * (locals.var_vds_res_dn9 / locals.var_vds_res))) }, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn10)) } } else { (assign57180_e89020 * (2.0 * (locals.var_vds_res_dn10 / locals.var_vds_res))) }, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn11)) } } else { (assign57180_e89020 * (2.0 * (locals.var_vds_res_dn11 / locals.var_vds_res))) }, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn14)) } } else { (assign57180_e89020 * (2.0 * (locals.var_vds_res_dn14 / locals.var_vds_res))) },)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign57180_e89022;
        locals.var_tmf1_dn0 = assign57180_e89022_d_n0;
        locals.var_tmf1_dn2 = assign57180_e89022_d_n2;
        locals.var_tmf1_dn4 = assign57180_e89022_d_n4;
        locals.var_tmf1_dn5 = assign57180_e89022_d_n5;
        locals.var_tmf1_dn6 = assign57180_e89022_d_n6;
        locals.var_tmf1_dn7 = assign57180_e89022_d_n7;
        locals.var_tmf1_dn8 = assign57180_e89022_d_n8;
        locals.var_tmf1_dn9 = assign57180_e89022_d_n9;
        locals.var_tmf1_dn10 = assign57180_e89022_d_n10;
        locals.var_tmf1_dn11 = assign57180_e89022_d_n11;
        locals.var_tmf1_dn14 = assign57180_e89022_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign57190_e89035, assign57190_e89035_d_n0, assign57190_e89035_d_n2, assign57190_e89035_d_n4, assign57190_e89035_d_n5, assign57190_e89035_d_n6, assign57190_e89035_d_n7, assign57190_e89035_d_n8, assign57190_e89035_d_n9, assign57190_e89035_d_n10, assign57190_e89035_d_n11, assign57190_e89035_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign57190_e89033: f64 = (0.01_f64).powf(2.0);
        (assign57190_e89033, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign57190_e89035;
        locals.var_tmf2_dn0 = assign57190_e89035_d_n0;
        locals.var_tmf2_dn2 = assign57190_e89035_d_n2;
        locals.var_tmf2_dn4 = assign57190_e89035_d_n4;
        locals.var_tmf2_dn5 = assign57190_e89035_d_n5;
        locals.var_tmf2_dn6 = assign57190_e89035_d_n6;
        locals.var_tmf2_dn7 = assign57190_e89035_d_n7;
        locals.var_tmf2_dn8 = assign57190_e89035_d_n8;
        locals.var_tmf2_dn9 = assign57190_e89035_d_n9;
        locals.var_tmf2_dn10 = assign57190_e89035_d_n10;
        locals.var_tmf2_dn11 = assign57190_e89035_d_n11;
        locals.var_tmf2_dn14 = assign57190_e89035_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign57200_e89058, assign57200_e89058_d_n0, assign57200_e89058_d_n2, assign57200_e89058_d_n4, assign57200_e89058_d_n5, assign57200_e89058_d_n6, assign57200_e89058_d_n7, assign57200_e89058_d_n8, assign57200_e89058_d_n9, assign57200_e89058_d_n10, assign57200_e89058_d_n11, assign57200_e89058_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign57200_e89046: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign57200_e89049: f64 = (1.0 / 2.0);
        let assign57200_e89050: f64 = (assign57200_e89046).powf(assign57200_e89049);
        let assign57200_e89054: f64 = (1.0 / 2.0);
        let assign57200_e89055: f64 = (locals.var_tmf2).powf(assign57200_e89054);
        let assign57200_e89056: f64 = (assign57200_e89050 - assign57200_e89055);
        (assign57200_e89056, (if 0.0 == 0.0 && ((assign57200_e89049) as f64).is_finite() && ((assign57200_e89049) as f64).fract() == 0.0 { if assign57200_e89049 == 0.0 { 0.0 } else { (assign57200_e89049 * ((assign57200_e89046).powf(assign57200_e89049 - 1.0) * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))) } } else { (assign57200_e89050 * (assign57200_e89049 * ((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) / assign57200_e89046))) } - if 0.0 == 0.0 && ((assign57200_e89054) as f64).is_finite() && ((assign57200_e89054) as f64).fract() == 0.0 { if assign57200_e89054 == 0.0 { 0.0 } else { (assign57200_e89054 * ((locals.var_tmf2).powf(assign57200_e89054 - 1.0) * locals.var_tmf2_dn0)) } } else { (assign57200_e89055 * (assign57200_e89054 * (locals.var_tmf2_dn0 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign57200_e89049) as f64).is_finite() && ((assign57200_e89049) as f64).fract() == 0.0 { if assign57200_e89049 == 0.0 { 0.0 } else { (assign57200_e89049 * ((assign57200_e89046).powf(assign57200_e89049 - 1.0) * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))) } } else { (assign57200_e89050 * (assign57200_e89049 * ((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) / assign57200_e89046))) } - if 0.0 == 0.0 && ((assign57200_e89054) as f64).is_finite() && ((assign57200_e89054) as f64).fract() == 0.0 { if assign57200_e89054 == 0.0 { 0.0 } else { (assign57200_e89054 * ((locals.var_tmf2).powf(assign57200_e89054 - 1.0) * locals.var_tmf2_dn2)) } } else { (assign57200_e89055 * (assign57200_e89054 * (locals.var_tmf2_dn2 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign57200_e89049) as f64).is_finite() && ((assign57200_e89049) as f64).fract() == 0.0 { if assign57200_e89049 == 0.0 { 0.0 } else { (assign57200_e89049 * ((assign57200_e89046).powf(assign57200_e89049 - 1.0) * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))) } } else { (assign57200_e89050 * (assign57200_e89049 * ((locals.var_tmf1_dn4 + locals.var_tmf2_dn4) / assign57200_e89046))) } - if 0.0 == 0.0 && ((assign57200_e89054) as f64).is_finite() && ((assign57200_e89054) as f64).fract() == 0.0 { if assign57200_e89054 == 0.0 { 0.0 } else { (assign57200_e89054 * ((locals.var_tmf2).powf(assign57200_e89054 - 1.0) * locals.var_tmf2_dn4)) } } else { (assign57200_e89055 * (assign57200_e89054 * (locals.var_tmf2_dn4 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign57200_e89049) as f64).is_finite() && ((assign57200_e89049) as f64).fract() == 0.0 { if assign57200_e89049 == 0.0 { 0.0 } else { (assign57200_e89049 * ((assign57200_e89046).powf(assign57200_e89049 - 1.0) * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))) } } else { (assign57200_e89050 * (assign57200_e89049 * ((locals.var_tmf1_dn5 + locals.var_tmf2_dn5) / assign57200_e89046))) } - if 0.0 == 0.0 && ((assign57200_e89054) as f64).is_finite() && ((assign57200_e89054) as f64).fract() == 0.0 { if assign57200_e89054 == 0.0 { 0.0 } else { (assign57200_e89054 * ((locals.var_tmf2).powf(assign57200_e89054 - 1.0) * locals.var_tmf2_dn5)) } } else { (assign57200_e89055 * (assign57200_e89054 * (locals.var_tmf2_dn5 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign57200_e89049) as f64).is_finite() && ((assign57200_e89049) as f64).fract() == 0.0 { if assign57200_e89049 == 0.0 { 0.0 } else { (assign57200_e89049 * ((assign57200_e89046).powf(assign57200_e89049 - 1.0) * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))) } } else { (assign57200_e89050 * (assign57200_e89049 * ((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) / assign57200_e89046))) } - if 0.0 == 0.0 && ((assign57200_e89054) as f64).is_finite() && ((assign57200_e89054) as f64).fract() == 0.0 { if assign57200_e89054 == 0.0 { 0.0 } else { (assign57200_e89054 * ((locals.var_tmf2).powf(assign57200_e89054 - 1.0) * locals.var_tmf2_dn6)) } } else { (assign57200_e89055 * (assign57200_e89054 * (locals.var_tmf2_dn6 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign57200_e89049) as f64).is_finite() && ((assign57200_e89049) as f64).fract() == 0.0 { if assign57200_e89049 == 0.0 { 0.0 } else { (assign57200_e89049 * ((assign57200_e89046).powf(assign57200_e89049 - 1.0) * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))) } } else { (assign57200_e89050 * (assign57200_e89049 * ((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) / assign57200_e89046))) } - if 0.0 == 0.0 && ((assign57200_e89054) as f64).is_finite() && ((assign57200_e89054) as f64).fract() == 0.0 { if assign57200_e89054 == 0.0 { 0.0 } else { (assign57200_e89054 * ((locals.var_tmf2).powf(assign57200_e89054 - 1.0) * locals.var_tmf2_dn7)) } } else { (assign57200_e89055 * (assign57200_e89054 * (locals.var_tmf2_dn7 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign57200_e89049) as f64).is_finite() && ((assign57200_e89049) as f64).fract() == 0.0 { if assign57200_e89049 == 0.0 { 0.0 } else { (assign57200_e89049 * ((assign57200_e89046).powf(assign57200_e89049 - 1.0) * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))) } } else { (assign57200_e89050 * (assign57200_e89049 * ((locals.var_tmf1_dn8 + locals.var_tmf2_dn8) / assign57200_e89046))) } - if 0.0 == 0.0 && ((assign57200_e89054) as f64).is_finite() && ((assign57200_e89054) as f64).fract() == 0.0 { if assign57200_e89054 == 0.0 { 0.0 } else { (assign57200_e89054 * ((locals.var_tmf2).powf(assign57200_e89054 - 1.0) * locals.var_tmf2_dn8)) } } else { (assign57200_e89055 * (assign57200_e89054 * (locals.var_tmf2_dn8 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign57200_e89049) as f64).is_finite() && ((assign57200_e89049) as f64).fract() == 0.0 { if assign57200_e89049 == 0.0 { 0.0 } else { (assign57200_e89049 * ((assign57200_e89046).powf(assign57200_e89049 - 1.0) * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))) } } else { (assign57200_e89050 * (assign57200_e89049 * ((locals.var_tmf1_dn9 + locals.var_tmf2_dn9) / assign57200_e89046))) } - if 0.0 == 0.0 && ((assign57200_e89054) as f64).is_finite() && ((assign57200_e89054) as f64).fract() == 0.0 { if assign57200_e89054 == 0.0 { 0.0 } else { (assign57200_e89054 * ((locals.var_tmf2).powf(assign57200_e89054 - 1.0) * locals.var_tmf2_dn9)) } } else { (assign57200_e89055 * (assign57200_e89054 * (locals.var_tmf2_dn9 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign57200_e89049) as f64).is_finite() && ((assign57200_e89049) as f64).fract() == 0.0 { if assign57200_e89049 == 0.0 { 0.0 } else { (assign57200_e89049 * ((assign57200_e89046).powf(assign57200_e89049 - 1.0) * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))) } } else { (assign57200_e89050 * (assign57200_e89049 * ((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) / assign57200_e89046))) } - if 0.0 == 0.0 && ((assign57200_e89054) as f64).is_finite() && ((assign57200_e89054) as f64).fract() == 0.0 { if assign57200_e89054 == 0.0 { 0.0 } else { (assign57200_e89054 * ((locals.var_tmf2).powf(assign57200_e89054 - 1.0) * locals.var_tmf2_dn10)) } } else { (assign57200_e89055 * (assign57200_e89054 * (locals.var_tmf2_dn10 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign57200_e89049) as f64).is_finite() && ((assign57200_e89049) as f64).fract() == 0.0 { if assign57200_e89049 == 0.0 { 0.0 } else { (assign57200_e89049 * ((assign57200_e89046).powf(assign57200_e89049 - 1.0) * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))) } } else { (assign57200_e89050 * (assign57200_e89049 * ((locals.var_tmf1_dn11 + locals.var_tmf2_dn11) / assign57200_e89046))) } - if 0.0 == 0.0 && ((assign57200_e89054) as f64).is_finite() && ((assign57200_e89054) as f64).fract() == 0.0 { if assign57200_e89054 == 0.0 { 0.0 } else { (assign57200_e89054 * ((locals.var_tmf2).powf(assign57200_e89054 - 1.0) * locals.var_tmf2_dn11)) } } else { (assign57200_e89055 * (assign57200_e89054 * (locals.var_tmf2_dn11 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign57200_e89049) as f64).is_finite() && ((assign57200_e89049) as f64).fract() == 0.0 { if assign57200_e89049 == 0.0 { 0.0 } else { (assign57200_e89049 * ((assign57200_e89046).powf(assign57200_e89049 - 1.0) * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))) } } else { (assign57200_e89050 * (assign57200_e89049 * ((locals.var_tmf1_dn14 + locals.var_tmf2_dn14) / assign57200_e89046))) } - if 0.0 == 0.0 && ((assign57200_e89054) as f64).is_finite() && ((assign57200_e89054) as f64).fract() == 0.0 { if assign57200_e89054 == 0.0 { 0.0 } else { (assign57200_e89054 * ((locals.var_tmf2).powf(assign57200_e89054 - 1.0) * locals.var_tmf2_dn14)) } } else { (assign57200_e89055 * (assign57200_e89054 * (locals.var_tmf2_dn14 / locals.var_tmf2))) }),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign57200_e89058;
        locals.var_t0_dn0 = assign57200_e89058_d_n0;
        locals.var_t0_dn2 = assign57200_e89058_d_n2;
        locals.var_t0_dn4 = assign57200_e89058_d_n4;
        locals.var_t0_dn5 = assign57200_e89058_d_n5;
        locals.var_t0_dn6 = assign57200_e89058_d_n6;
        locals.var_t0_dn7 = assign57200_e89058_d_n7;
        locals.var_t0_dn8 = assign57200_e89058_d_n8;
        locals.var_t0_dn9 = assign57200_e89058_d_n9;
        locals.var_t0_dn10 = assign57200_e89058_d_n10;
        locals.var_t0_dn11 = assign57200_e89058_d_n11;
        locals.var_t0_dn14 = assign57200_e89058_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign57210_e89073, assign57210_e89073_d_n0, assign57210_e89073_d_n2, assign57210_e89073_d_n4, assign57210_e89073_d_n5, assign57210_e89073_d_n6, assign57210_e89073_d_n7, assign57210_e89073_d_n8, assign57210_e89073_d_n9, assign57210_e89073_d_n10, assign57210_e89073_d_n11, assign57210_e89073_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign57210_e89070: f64 = (locals.var_leff - p.p402);
        let assign57210_e89071: f64 = (locals.var_t0 / assign57210_e89070);
        (assign57210_e89071, (locals.var_t0_dn0 / assign57210_e89070), (locals.var_t0_dn2 / assign57210_e89070), (locals.var_t0_dn4 / assign57210_e89070), (locals.var_t0_dn5 / assign57210_e89070), (locals.var_t0_dn6 / assign57210_e89070), (locals.var_t0_dn7 / assign57210_e89070), (locals.var_t0_dn8 / assign57210_e89070), (locals.var_t0_dn9 / assign57210_e89070), (locals.var_t0_dn10 / assign57210_e89070), (locals.var_t0_dn11 / assign57210_e89070), (locals.var_t0_dn14 / assign57210_e89070),)
    } else {
        (locals.var_edri2, locals.var_edri2_dn0, locals.var_edri2_dn2, locals.var_edri2_dn4, locals.var_edri2_dn5, locals.var_edri2_dn6, locals.var_edri2_dn7, locals.var_edri2_dn8, locals.var_edri2_dn9, locals.var_edri2_dn10, locals.var_edri2_dn11, locals.var_edri2_dn14,)
    }
};
        locals.var_edri2 = assign57210_e89073;
        locals.var_edri2_dn0 = assign57210_e89073_d_n0;
        locals.var_edri2_dn2 = assign57210_e89073_d_n2;
        locals.var_edri2_dn4 = assign57210_e89073_d_n4;
        locals.var_edri2_dn5 = assign57210_e89073_d_n5;
        locals.var_edri2_dn6 = assign57210_e89073_d_n6;
        locals.var_edri2_dn7 = assign57210_e89073_d_n7;
        locals.var_edri2_dn8 = assign57210_e89073_d_n8;
        locals.var_edri2_dn9 = assign57210_e89073_d_n9;
        locals.var_edri2_dn10 = assign57210_e89073_d_n10;
        locals.var_edri2_dn11 = assign57210_e89073_d_n11;
        locals.var_edri2_dn14 = assign57210_e89073_d_n14;
        locals.var_edri2_rv = 0.0;

        let (assign57220_e89088, assign57220_e89088_d_n0, assign57220_e89088_d_n2, assign57220_e89088_d_n4, assign57220_e89088_d_n5, assign57220_e89088_d_n6, assign57220_e89088_d_n7, assign57220_e89088_d_n8, assign57220_e89088_d_n9, assign57220_e89088_d_n10, assign57220_e89088_d_n11, assign57220_e89088_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign57220_e89084: f64 = (locals.var_muun * locals.var_edri2);
        let assign57220_e89086: f64 = (assign57220_e89084 / locals.var_uc_depvmax);
        (assign57220_e89086, (((((locals.var_muun_dn0 * locals.var_edri2) + (locals.var_muun * locals.var_edri2_dn0)) * locals.var_uc_depvmax) - (assign57220_e89084 * locals.var_uc_depvmax_dn0)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn2 * locals.var_edri2) + (locals.var_muun * locals.var_edri2_dn2)) * locals.var_uc_depvmax) - (assign57220_e89084 * locals.var_uc_depvmax_dn2)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn4 * locals.var_edri2) + (locals.var_muun * locals.var_edri2_dn4)) * locals.var_uc_depvmax) - (assign57220_e89084 * locals.var_uc_depvmax_dn4)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn5 * locals.var_edri2) + (locals.var_muun * locals.var_edri2_dn5)) * locals.var_uc_depvmax) - (assign57220_e89084 * locals.var_uc_depvmax_dn5)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn6 * locals.var_edri2) + (locals.var_muun * locals.var_edri2_dn6)) * locals.var_uc_depvmax) - (assign57220_e89084 * locals.var_uc_depvmax_dn6)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn7 * locals.var_edri2) + (locals.var_muun * locals.var_edri2_dn7)) * locals.var_uc_depvmax) - (assign57220_e89084 * locals.var_uc_depvmax_dn7)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn8 * locals.var_edri2) + (locals.var_muun * locals.var_edri2_dn8)) * locals.var_uc_depvmax) - (assign57220_e89084 * locals.var_uc_depvmax_dn8)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn9 * locals.var_edri2) + (locals.var_muun * locals.var_edri2_dn9)) * locals.var_uc_depvmax) - (assign57220_e89084 * locals.var_uc_depvmax_dn9)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn10 * locals.var_edri2) + (locals.var_muun * locals.var_edri2_dn10)) * locals.var_uc_depvmax) - (assign57220_e89084 * locals.var_uc_depvmax_dn10)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn11 * locals.var_edri2) + (locals.var_muun * locals.var_edri2_dn11)) * locals.var_uc_depvmax) - (assign57220_e89084 * locals.var_uc_depvmax_dn11)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn14 * locals.var_edri2) + (locals.var_muun * locals.var_edri2_dn14)) * locals.var_uc_depvmax) - (assign57220_e89084 * locals.var_uc_depvmax_dn14)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign57220_e89088;
        locals.var_t1_dn0 = assign57220_e89088_d_n0;
        locals.var_t1_dn2 = assign57220_e89088_d_n2;
        locals.var_t1_dn4 = assign57220_e89088_d_n4;
        locals.var_t1_dn5 = assign57220_e89088_d_n5;
        locals.var_t1_dn6 = assign57220_e89088_d_n6;
        locals.var_t1_dn7 = assign57220_e89088_d_n7;
        locals.var_t1_dn8 = assign57220_e89088_d_n8;
        locals.var_t1_dn9 = assign57220_e89088_d_n9;
        locals.var_t1_dn10 = assign57220_e89088_d_n10;
        locals.var_t1_dn11 = assign57220_e89088_d_n11;
        locals.var_t1_dn14 = assign57220_e89088_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign57230_e89106, assign57230_e89106_d_n0, assign57230_e89106_d_n2, assign57230_e89106_d_n4, assign57230_e89106_d_n5, assign57230_e89106_d_n6, assign57230_e89106_d_n7, assign57230_e89106_d_n8, assign57230_e89106_d_n9, assign57230_e89106_d_n10, assign57230_e89106_d_n11, assign57230_e89106_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let (assign57230_e89104, assign57230_e89104_d_n0, assign57230_e89104_d_n2, assign57230_e89104_d_n4, assign57230_e89104_d_n5, assign57230_e89104_d_n6, assign57230_e89104_d_n7, assign57230_e89104_d_n8, assign57230_e89104_d_n9, assign57230_e89104_d_n10, assign57230_e89104_d_n11, assign57230_e89104_d_n14,) = {
            if (locals.var_t1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign57230_e89103: f64 = (locals.var_t1).powf(p.p378);
                (assign57230_e89103, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn0)) } } else { (assign57230_e89103 * (p.p378 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn2)) } } else { (assign57230_e89103 * (p.p378 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn4)) } } else { (assign57230_e89103 * (p.p378 * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn5)) } } else { (assign57230_e89103 * (p.p378 * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn6)) } } else { (assign57230_e89103 * (p.p378 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn7)) } } else { (assign57230_e89103 * (p.p378 * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn8)) } } else { (assign57230_e89103 * (p.p378 * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn9)) } } else { (assign57230_e89103 * (p.p378 * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn10)) } } else { (assign57230_e89103 * (p.p378 * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn11)) } } else { (assign57230_e89103 * (p.p378 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn14)) } } else { (assign57230_e89103 * (p.p378 * (locals.var_t1_dn14 / locals.var_t1))) },)
            }
        };
        (assign57230_e89104, assign57230_e89104_d_n0, assign57230_e89104_d_n2, assign57230_e89104_d_n4, assign57230_e89104_d_n5, assign57230_e89104_d_n6, assign57230_e89104_d_n7, assign57230_e89104_d_n8, assign57230_e89104_d_n9, assign57230_e89104_d_n10, assign57230_e89104_d_n11, assign57230_e89104_d_n14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign57230_e89106;
        locals.var_t2_dn0 = assign57230_e89106_d_n0;
        locals.var_t2_dn2 = assign57230_e89106_d_n2;
        locals.var_t2_dn4 = assign57230_e89106_d_n4;
        locals.var_t2_dn5 = assign57230_e89106_d_n5;
        locals.var_t2_dn6 = assign57230_e89106_d_n6;
        locals.var_t2_dn7 = assign57230_e89106_d_n7;
        locals.var_t2_dn8 = assign57230_e89106_d_n8;
        locals.var_t2_dn9 = assign57230_e89106_d_n9;
        locals.var_t2_dn10 = assign57230_e89106_d_n10;
        locals.var_t2_dn11 = assign57230_e89106_d_n11;
        locals.var_t2_dn14 = assign57230_e89106_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign57240_e89119, assign57240_e89119_d_n0, assign57240_e89119_d_n2, assign57240_e89119_d_n4, assign57240_e89119_d_n5, assign57240_e89119_d_n6, assign57240_e89119_d_n7, assign57240_e89119_d_n8, assign57240_e89119_d_n9, assign57240_e89119_d_n10, assign57240_e89119_d_n11, assign57240_e89119_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign57240_e89117: f64 = (1.0 + locals.var_t2);
        (assign57240_e89117, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign57240_e89119;
        locals.var_t3_dn0 = assign57240_e89119_d_n0;
        locals.var_t3_dn2 = assign57240_e89119_d_n2;
        locals.var_t3_dn4 = assign57240_e89119_d_n4;
        locals.var_t3_dn5 = assign57240_e89119_d_n5;
        locals.var_t3_dn6 = assign57240_e89119_d_n6;
        locals.var_t3_dn7 = assign57240_e89119_d_n7;
        locals.var_t3_dn8 = assign57240_e89119_d_n8;
        locals.var_t3_dn9 = assign57240_e89119_d_n9;
        locals.var_t3_dn10 = assign57240_e89119_d_n10;
        locals.var_t3_dn11 = assign57240_e89119_d_n11;
        locals.var_t3_dn14 = assign57240_e89119_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign57250_e89139, assign57250_e89139_d_n0, assign57250_e89139_d_n2, assign57250_e89139_d_n4, assign57250_e89139_d_n5, assign57250_e89139_d_n6, assign57250_e89139_d_n7, assign57250_e89139_d_n8, assign57250_e89139_d_n9, assign57250_e89139_d_n10, assign57250_e89139_d_n11, assign57250_e89139_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let (assign57250_e89137, assign57250_e89137_d_n0, assign57250_e89137_d_n2, assign57250_e89137_d_n4, assign57250_e89137_d_n5, assign57250_e89137_d_n6, assign57250_e89137_d_n7, assign57250_e89137_d_n8, assign57250_e89137_d_n9, assign57250_e89137_d_n10, assign57250_e89137_d_n11, assign57250_e89137_d_n14,) = {
            if (locals.var_t3 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign57250_e89135: f64 = (1.0 / p.p378);
                let assign57250_e89136: f64 = (locals.var_t3).powf(assign57250_e89135);
                (assign57250_e89136, if 0.0 == 0.0 && ((assign57250_e89135) as f64).is_finite() && ((assign57250_e89135) as f64).fract() == 0.0 { if assign57250_e89135 == 0.0 { 0.0 } else { (assign57250_e89135 * ((locals.var_t3).powf(assign57250_e89135 - 1.0) * locals.var_t3_dn0)) } } else { (assign57250_e89136 * (assign57250_e89135 * (locals.var_t3_dn0 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57250_e89135) as f64).is_finite() && ((assign57250_e89135) as f64).fract() == 0.0 { if assign57250_e89135 == 0.0 { 0.0 } else { (assign57250_e89135 * ((locals.var_t3).powf(assign57250_e89135 - 1.0) * locals.var_t3_dn2)) } } else { (assign57250_e89136 * (assign57250_e89135 * (locals.var_t3_dn2 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57250_e89135) as f64).is_finite() && ((assign57250_e89135) as f64).fract() == 0.0 { if assign57250_e89135 == 0.0 { 0.0 } else { (assign57250_e89135 * ((locals.var_t3).powf(assign57250_e89135 - 1.0) * locals.var_t3_dn4)) } } else { (assign57250_e89136 * (assign57250_e89135 * (locals.var_t3_dn4 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57250_e89135) as f64).is_finite() && ((assign57250_e89135) as f64).fract() == 0.0 { if assign57250_e89135 == 0.0 { 0.0 } else { (assign57250_e89135 * ((locals.var_t3).powf(assign57250_e89135 - 1.0) * locals.var_t3_dn5)) } } else { (assign57250_e89136 * (assign57250_e89135 * (locals.var_t3_dn5 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57250_e89135) as f64).is_finite() && ((assign57250_e89135) as f64).fract() == 0.0 { if assign57250_e89135 == 0.0 { 0.0 } else { (assign57250_e89135 * ((locals.var_t3).powf(assign57250_e89135 - 1.0) * locals.var_t3_dn6)) } } else { (assign57250_e89136 * (assign57250_e89135 * (locals.var_t3_dn6 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57250_e89135) as f64).is_finite() && ((assign57250_e89135) as f64).fract() == 0.0 { if assign57250_e89135 == 0.0 { 0.0 } else { (assign57250_e89135 * ((locals.var_t3).powf(assign57250_e89135 - 1.0) * locals.var_t3_dn7)) } } else { (assign57250_e89136 * (assign57250_e89135 * (locals.var_t3_dn7 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57250_e89135) as f64).is_finite() && ((assign57250_e89135) as f64).fract() == 0.0 { if assign57250_e89135 == 0.0 { 0.0 } else { (assign57250_e89135 * ((locals.var_t3).powf(assign57250_e89135 - 1.0) * locals.var_t3_dn8)) } } else { (assign57250_e89136 * (assign57250_e89135 * (locals.var_t3_dn8 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57250_e89135) as f64).is_finite() && ((assign57250_e89135) as f64).fract() == 0.0 { if assign57250_e89135 == 0.0 { 0.0 } else { (assign57250_e89135 * ((locals.var_t3).powf(assign57250_e89135 - 1.0) * locals.var_t3_dn9)) } } else { (assign57250_e89136 * (assign57250_e89135 * (locals.var_t3_dn9 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57250_e89135) as f64).is_finite() && ((assign57250_e89135) as f64).fract() == 0.0 { if assign57250_e89135 == 0.0 { 0.0 } else { (assign57250_e89135 * ((locals.var_t3).powf(assign57250_e89135 - 1.0) * locals.var_t3_dn10)) } } else { (assign57250_e89136 * (assign57250_e89135 * (locals.var_t3_dn10 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57250_e89135) as f64).is_finite() && ((assign57250_e89135) as f64).fract() == 0.0 { if assign57250_e89135 == 0.0 { 0.0 } else { (assign57250_e89135 * ((locals.var_t3).powf(assign57250_e89135 - 1.0) * locals.var_t3_dn11)) } } else { (assign57250_e89136 * (assign57250_e89135 * (locals.var_t3_dn11 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign57250_e89135) as f64).is_finite() && ((assign57250_e89135) as f64).fract() == 0.0 { if assign57250_e89135 == 0.0 { 0.0 } else { (assign57250_e89135 * ((locals.var_t3).powf(assign57250_e89135 - 1.0) * locals.var_t3_dn14)) } } else { (assign57250_e89136 * (assign57250_e89135 * (locals.var_t3_dn14 / locals.var_t3))) },)
            }
        };
        (assign57250_e89137, assign57250_e89137_d_n0, assign57250_e89137_d_n2, assign57250_e89137_d_n4, assign57250_e89137_d_n5, assign57250_e89137_d_n6, assign57250_e89137_d_n7, assign57250_e89137_d_n8, assign57250_e89137_d_n9, assign57250_e89137_d_n10, assign57250_e89137_d_n11, assign57250_e89137_d_n14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign57250_e89139;
        locals.var_t4_dn0 = assign57250_e89139_d_n0;
        locals.var_t4_dn2 = assign57250_e89139_d_n2;
        locals.var_t4_dn4 = assign57250_e89139_d_n4;
        locals.var_t4_dn5 = assign57250_e89139_d_n5;
        locals.var_t4_dn6 = assign57250_e89139_d_n6;
        locals.var_t4_dn7 = assign57250_e89139_d_n7;
        locals.var_t4_dn8 = assign57250_e89139_d_n8;
        locals.var_t4_dn9 = assign57250_e89139_d_n9;
        locals.var_t4_dn10 = assign57250_e89139_d_n10;
        locals.var_t4_dn11 = assign57250_e89139_d_n11;
        locals.var_t4_dn14 = assign57250_e89139_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign57260_e89152, assign57260_e89152_d_n0, assign57260_e89152_d_n2, assign57260_e89152_d_n4, assign57260_e89152_d_n5, assign57260_e89152_d_n6, assign57260_e89152_d_n7, assign57260_e89152_d_n8, assign57260_e89152_d_n9, assign57260_e89152_d_n10, assign57260_e89152_d_n11, assign57260_e89152_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign57260_e89150: f64 = (locals.var_muun / locals.var_t4);
        (assign57260_e89150, (((locals.var_muun_dn0 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn2 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn4 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn5 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn6 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn7 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn8 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn9 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn10 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn11 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn14 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn14)) / (locals.var_t4 * locals.var_t4)),)
    } else {
        (locals.var_mu_res, locals.var_mu_res_dn0, locals.var_mu_res_dn2, locals.var_mu_res_dn4, locals.var_mu_res_dn5, locals.var_mu_res_dn6, locals.var_mu_res_dn7, locals.var_mu_res_dn8, locals.var_mu_res_dn9, locals.var_mu_res_dn10, locals.var_mu_res_dn11, locals.var_mu_res_dn14,)
    }
};
        locals.var_mu_res = assign57260_e89152;
        locals.var_mu_res_dn0 = assign57260_e89152_d_n0;
        locals.var_mu_res_dn2 = assign57260_e89152_d_n2;
        locals.var_mu_res_dn4 = assign57260_e89152_d_n4;
        locals.var_mu_res_dn5 = assign57260_e89152_d_n5;
        locals.var_mu_res_dn6 = assign57260_e89152_d_n6;
        locals.var_mu_res_dn7 = assign57260_e89152_d_n7;
        locals.var_mu_res_dn8 = assign57260_e89152_d_n8;
        locals.var_mu_res_dn9 = assign57260_e89152_d_n9;
        locals.var_mu_res_dn10 = assign57260_e89152_d_n10;
        locals.var_mu_res_dn11 = assign57260_e89152_d_n11;
        locals.var_mu_res_dn14 = assign57260_e89152_d_n14;
        locals.var_mu_res_rv = 0.0;

        let (assign57270_e89181, assign57270_e89181_d_n0, assign57270_e89181_d_n2, assign57270_e89181_d_n4, assign57270_e89181_d_n5, assign57270_e89181_d_n6, assign57270_e89181_d_n7, assign57270_e89181_d_n8, assign57270_e89181_d_n9, assign57270_e89181_d_n10, assign57270_e89181_d_n11, assign57270_e89181_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign57270_e89165: f64 = (p.p400 * locals.var_edri__blk1117);
        let assign57270_e89171: f64 = (locals.var_muun * locals.var_edri__blk1117);
        let assign57270_e89173: f64 = (assign57270_e89171 / locals.var_uc_depvmax);
        let assign57270_e89174: f64 = (1.0 + assign57270_e89173);
        let assign57270_e89175: f64 = (1.0 / assign57270_e89174);
        let assign57270_e89176: f64 = (1.0 - assign57270_e89175);
        let assign57270_e89177: f64 = (assign57270_e89165 * assign57270_e89176);
        let assign57270_e89178: f64 = (1.0 + assign57270_e89177);
        let assign57270_e89179: f64 = (locals.var_uc_ndepm * assign57270_e89178);
        (assign57270_e89179, ((locals.var_uc_ndepm_dn0 * assign57270_e89178) + (locals.var_uc_ndepm * (((p.p400 * locals.var_edri__blk1117_dn0) * assign57270_e89176) + (assign57270_e89165 * (-(-((((((locals.var_muun_dn0 * locals.var_edri__blk1117) + (locals.var_muun * locals.var_edri__blk1117_dn0)) * locals.var_uc_depvmax) - (assign57270_e89171 * locals.var_uc_depvmax_dn0)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)) / (assign57270_e89174 * assign57270_e89174)))))))), ((locals.var_uc_ndepm_dn2 * assign57270_e89178) + (locals.var_uc_ndepm * (((p.p400 * locals.var_edri__blk1117_dn2) * assign57270_e89176) + (assign57270_e89165 * (-(-((((((locals.var_muun_dn2 * locals.var_edri__blk1117) + (locals.var_muun * locals.var_edri__blk1117_dn2)) * locals.var_uc_depvmax) - (assign57270_e89171 * locals.var_uc_depvmax_dn2)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)) / (assign57270_e89174 * assign57270_e89174)))))))), ((locals.var_uc_ndepm_dn4 * assign57270_e89178) + (locals.var_uc_ndepm * (((p.p400 * locals.var_edri__blk1117_dn4) * assign57270_e89176) + (assign57270_e89165 * (-(-((((((locals.var_muun_dn4 * locals.var_edri__blk1117) + (locals.var_muun * locals.var_edri__blk1117_dn4)) * locals.var_uc_depvmax) - (assign57270_e89171 * locals.var_uc_depvmax_dn4)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)) / (assign57270_e89174 * assign57270_e89174)))))))), ((locals.var_uc_ndepm_dn5 * assign57270_e89178) + (locals.var_uc_ndepm * (((p.p400 * locals.var_edri__blk1117_dn5) * assign57270_e89176) + (assign57270_e89165 * (-(-((((((locals.var_muun_dn5 * locals.var_edri__blk1117) + (locals.var_muun * locals.var_edri__blk1117_dn5)) * locals.var_uc_depvmax) - (assign57270_e89171 * locals.var_uc_depvmax_dn5)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)) / (assign57270_e89174 * assign57270_e89174)))))))), ((locals.var_uc_ndepm_dn6 * assign57270_e89178) + (locals.var_uc_ndepm * (((p.p400 * locals.var_edri__blk1117_dn6) * assign57270_e89176) + (assign57270_e89165 * (-(-((((((locals.var_muun_dn6 * locals.var_edri__blk1117) + (locals.var_muun * locals.var_edri__blk1117_dn6)) * locals.var_uc_depvmax) - (assign57270_e89171 * locals.var_uc_depvmax_dn6)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)) / (assign57270_e89174 * assign57270_e89174)))))))), ((locals.var_uc_ndepm_dn7 * assign57270_e89178) + (locals.var_uc_ndepm * (((p.p400 * locals.var_edri__blk1117_dn7) * assign57270_e89176) + (assign57270_e89165 * (-(-((((((locals.var_muun_dn7 * locals.var_edri__blk1117) + (locals.var_muun * locals.var_edri__blk1117_dn7)) * locals.var_uc_depvmax) - (assign57270_e89171 * locals.var_uc_depvmax_dn7)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)) / (assign57270_e89174 * assign57270_e89174)))))))), ((locals.var_uc_ndepm_dn8 * assign57270_e89178) + (locals.var_uc_ndepm * (((p.p400 * locals.var_edri__blk1117_dn8) * assign57270_e89176) + (assign57270_e89165 * (-(-((((((locals.var_muun_dn8 * locals.var_edri__blk1117) + (locals.var_muun * locals.var_edri__blk1117_dn8)) * locals.var_uc_depvmax) - (assign57270_e89171 * locals.var_uc_depvmax_dn8)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)) / (assign57270_e89174 * assign57270_e89174)))))))), ((locals.var_uc_ndepm_dn9 * assign57270_e89178) + (locals.var_uc_ndepm * (((p.p400 * locals.var_edri__blk1117_dn9) * assign57270_e89176) + (assign57270_e89165 * (-(-((((((locals.var_muun_dn9 * locals.var_edri__blk1117) + (locals.var_muun * locals.var_edri__blk1117_dn9)) * locals.var_uc_depvmax) - (assign57270_e89171 * locals.var_uc_depvmax_dn9)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)) / (assign57270_e89174 * assign57270_e89174)))))))), ((locals.var_uc_ndepm_dn10 * assign57270_e89178) + (locals.var_uc_ndepm * (((p.p400 * locals.var_edri__blk1117_dn10) * assign57270_e89176) + (assign57270_e89165 * (-(-((((((locals.var_muun_dn10 * locals.var_edri__blk1117) + (locals.var_muun * locals.var_edri__blk1117_dn10)) * locals.var_uc_depvmax) - (assign57270_e89171 * locals.var_uc_depvmax_dn10)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)) / (assign57270_e89174 * assign57270_e89174)))))))), ((locals.var_uc_ndepm_dn11 * assign57270_e89178) + (locals.var_uc_ndepm * (((p.p400 * locals.var_edri__blk1117_dn11) * assign57270_e89176) + (assign57270_e89165 * (-(-((((((locals.var_muun_dn11 * locals.var_edri__blk1117) + (locals.var_muun * locals.var_edri__blk1117_dn11)) * locals.var_uc_depvmax) - (assign57270_e89171 * locals.var_uc_depvmax_dn11)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)) / (assign57270_e89174 * assign57270_e89174)))))))), ((locals.var_uc_ndepm_dn14 * assign57270_e89178) + (locals.var_uc_ndepm * (((p.p400 * locals.var_edri__blk1117_dn14) * assign57270_e89176) + (assign57270_e89165 * (-(-((((((locals.var_muun_dn14 * locals.var_edri__blk1117) + (locals.var_muun * locals.var_edri__blk1117_dn14)) * locals.var_uc_depvmax) - (assign57270_e89171 * locals.var_uc_depvmax_dn14)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)) / (assign57270_e89174 * assign57270_e89174)))))))),)
    } else {
        (locals.var_n_res, locals.var_n_res_dn0, locals.var_n_res_dn2, locals.var_n_res_dn4, locals.var_n_res_dn5, locals.var_n_res_dn6, locals.var_n_res_dn7, locals.var_n_res_dn8, locals.var_n_res_dn9, locals.var_n_res_dn10, locals.var_n_res_dn11, locals.var_n_res_dn14,)
    }
};
        locals.var_n_res = assign57270_e89181;
        locals.var_n_res_dn0 = assign57270_e89181_d_n0;
        locals.var_n_res_dn2 = assign57270_e89181_d_n2;
        locals.var_n_res_dn4 = assign57270_e89181_d_n4;
        locals.var_n_res_dn5 = assign57270_e89181_d_n5;
        locals.var_n_res_dn6 = assign57270_e89181_d_n6;
        locals.var_n_res_dn7 = assign57270_e89181_d_n7;
        locals.var_n_res_dn8 = assign57270_e89181_d_n8;
        locals.var_n_res_dn9 = assign57270_e89181_d_n9;
        locals.var_n_res_dn10 = assign57270_e89181_d_n10;
        locals.var_n_res_dn11 = assign57270_e89181_d_n11;
        locals.var_n_res_dn14 = assign57270_e89181_d_n14;
        locals.var_n_res_rv = 0.0;

        let (assign57280_e89196, assign57280_e89196_d_n0, assign57280_e89196_d_n2, assign57280_e89196_d_n4, assign57280_e89196_d_n5, assign57280_e89196_d_n6, assign57280_e89196_d_n7, assign57280_e89196_d_n8, assign57280_e89196_d_n9, assign57280_e89196_d_n10, assign57280_e89196_d_n11, assign57280_e89196_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign57280_e89192: f64 = (locals.var_w_res * 1.6021918e-19);
        let assign57280_e89194: f64 = (assign57280_e89192 * locals.var_n_res);
        (assign57280_e89194, (((locals.var_w_res_dn0 * 1.6021918e-19) * locals.var_n_res) + (assign57280_e89192 * locals.var_n_res_dn0)), (((locals.var_w_res_dn2 * 1.6021918e-19) * locals.var_n_res) + (assign57280_e89192 * locals.var_n_res_dn2)), (((locals.var_w_res_dn4 * 1.6021918e-19) * locals.var_n_res) + (assign57280_e89192 * locals.var_n_res_dn4)), (((locals.var_w_res_dn5 * 1.6021918e-19) * locals.var_n_res) + (assign57280_e89192 * locals.var_n_res_dn5)), (((locals.var_w_res_dn6 * 1.6021918e-19) * locals.var_n_res) + (assign57280_e89192 * locals.var_n_res_dn6)), (((locals.var_w_res_dn7 * 1.6021918e-19) * locals.var_n_res) + (assign57280_e89192 * locals.var_n_res_dn7)), (((locals.var_w_res_dn8 * 1.6021918e-19) * locals.var_n_res) + (assign57280_e89192 * locals.var_n_res_dn8)), (((locals.var_w_res_dn9 * 1.6021918e-19) * locals.var_n_res) + (assign57280_e89192 * locals.var_n_res_dn9)), (((locals.var_w_res_dn10 * 1.6021918e-19) * locals.var_n_res) + (assign57280_e89192 * locals.var_n_res_dn10)), (((locals.var_w_res_dn11 * 1.6021918e-19) * locals.var_n_res) + (assign57280_e89192 * locals.var_n_res_dn11)), (((locals.var_w_res_dn14 * 1.6021918e-19) * locals.var_n_res) + (assign57280_e89192 * locals.var_n_res_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign57280_e89196;
        locals.var_t1_dn0 = assign57280_e89196_d_n0;
        locals.var_t1_dn2 = assign57280_e89196_d_n2;
        locals.var_t1_dn4 = assign57280_e89196_d_n4;
        locals.var_t1_dn5 = assign57280_e89196_d_n5;
        locals.var_t1_dn6 = assign57280_e89196_d_n6;
        locals.var_t1_dn7 = assign57280_e89196_d_n7;
        locals.var_t1_dn8 = assign57280_e89196_d_n8;
        locals.var_t1_dn9 = assign57280_e89196_d_n9;
        locals.var_t1_dn10 = assign57280_e89196_d_n10;
        locals.var_t1_dn11 = assign57280_e89196_d_n11;
        locals.var_t1_dn14 = assign57280_e89196_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign57290_e89213, assign57290_e89213_d_n0, assign57290_e89213_d_n2, assign57290_e89213_d_n4, assign57290_e89213_d_n5, assign57290_e89213_d_n6, assign57290_e89213_d_n7, assign57290_e89213_d_n8, assign57290_e89213_d_n9, assign57290_e89213_d_n10, assign57290_e89213_d_n11, assign57290_e89213_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign57290_e89207: f64 = (locals.var_weff / locals.var_leff);
        let assign57290_e89209: f64 = (assign57290_e89207).powf(locals.var_uc_depwlp);
        let assign57290_e89211: f64 = (assign57290_e89209 * p.p7);
        (assign57290_e89211, (if locals.var_uc_depwlp_dn0 == 0.0 && ((locals.var_uc_depwlp) as f64).is_finite() && ((locals.var_uc_depwlp) as f64).fract() == 0.0 { 0.0 } else { (assign57290_e89209 * (locals.var_uc_depwlp_dn0 * (assign57290_e89207).ln())) } * p.p7), (if locals.var_uc_depwlp_dn2 == 0.0 && ((locals.var_uc_depwlp) as f64).is_finite() && ((locals.var_uc_depwlp) as f64).fract() == 0.0 { 0.0 } else { (assign57290_e89209 * (locals.var_uc_depwlp_dn2 * (assign57290_e89207).ln())) } * p.p7), (if locals.var_uc_depwlp_dn4 == 0.0 && ((locals.var_uc_depwlp) as f64).is_finite() && ((locals.var_uc_depwlp) as f64).fract() == 0.0 { 0.0 } else { (assign57290_e89209 * (locals.var_uc_depwlp_dn4 * (assign57290_e89207).ln())) } * p.p7), (if locals.var_uc_depwlp_dn5 == 0.0 && ((locals.var_uc_depwlp) as f64).is_finite() && ((locals.var_uc_depwlp) as f64).fract() == 0.0 { 0.0 } else { (assign57290_e89209 * (locals.var_uc_depwlp_dn5 * (assign57290_e89207).ln())) } * p.p7), (if locals.var_uc_depwlp_dn6 == 0.0 && ((locals.var_uc_depwlp) as f64).is_finite() && ((locals.var_uc_depwlp) as f64).fract() == 0.0 { 0.0 } else { (assign57290_e89209 * (locals.var_uc_depwlp_dn6 * (assign57290_e89207).ln())) } * p.p7), (if locals.var_uc_depwlp_dn7 == 0.0 && ((locals.var_uc_depwlp) as f64).is_finite() && ((locals.var_uc_depwlp) as f64).fract() == 0.0 { 0.0 } else { (assign57290_e89209 * (locals.var_uc_depwlp_dn7 * (assign57290_e89207).ln())) } * p.p7), (if locals.var_uc_depwlp_dn8 == 0.0 && ((locals.var_uc_depwlp) as f64).is_finite() && ((locals.var_uc_depwlp) as f64).fract() == 0.0 { 0.0 } else { (assign57290_e89209 * (locals.var_uc_depwlp_dn8 * (assign57290_e89207).ln())) } * p.p7), (if locals.var_uc_depwlp_dn9 == 0.0 && ((locals.var_uc_depwlp) as f64).is_finite() && ((locals.var_uc_depwlp) as f64).fract() == 0.0 { 0.0 } else { (assign57290_e89209 * (locals.var_uc_depwlp_dn9 * (assign57290_e89207).ln())) } * p.p7), (if locals.var_uc_depwlp_dn10 == 0.0 && ((locals.var_uc_depwlp) as f64).is_finite() && ((locals.var_uc_depwlp) as f64).fract() == 0.0 { 0.0 } else { (assign57290_e89209 * (locals.var_uc_depwlp_dn10 * (assign57290_e89207).ln())) } * p.p7), (if locals.var_uc_depwlp_dn11 == 0.0 && ((locals.var_uc_depwlp) as f64).is_finite() && ((locals.var_uc_depwlp) as f64).fract() == 0.0 { 0.0 } else { (assign57290_e89209 * (locals.var_uc_depwlp_dn11 * (assign57290_e89207).ln())) } * p.p7), (if locals.var_uc_depwlp_dn14 == 0.0 && ((locals.var_uc_depwlp) as f64).is_finite() && ((locals.var_uc_depwlp) as f64).fract() == 0.0 { 0.0 } else { (assign57290_e89209 * (locals.var_uc_depwlp_dn14 * (assign57290_e89207).ln())) } * p.p7),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign57290_e89213;
        locals.var_t2_dn0 = assign57290_e89213_d_n0;
        locals.var_t2_dn2 = assign57290_e89213_d_n2;
        locals.var_t2_dn4 = assign57290_e89213_d_n4;
        locals.var_t2_dn5 = assign57290_e89213_d_n5;
        locals.var_t2_dn6 = assign57290_e89213_d_n6;
        locals.var_t2_dn7 = assign57290_e89213_d_n7;
        locals.var_t2_dn8 = assign57290_e89213_d_n8;
        locals.var_t2_dn9 = assign57290_e89213_d_n9;
        locals.var_t2_dn10 = assign57290_e89213_d_n10;
        locals.var_t2_dn11 = assign57290_e89213_d_n11;
        locals.var_t2_dn14 = assign57290_e89213_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign57300_e89230, assign57300_e89230_d_n0, assign57300_e89230_d_n2, assign57300_e89230_d_n4, assign57300_e89230_d_n5, assign57300_e89230_d_n6, assign57300_e89230_d_n7, assign57300_e89230_d_n8, assign57300_e89230_d_n9, assign57300_e89230_d_n10, assign57300_e89230_d_n11, assign57300_e89230_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign57300_e89224: f64 = (locals.var_weff_nf * locals.var_t1);
        let assign57300_e89226: f64 = (assign57300_e89224 * locals.var_mu_res);
        let assign57300_e89228: f64 = (assign57300_e89226 * locals.var_edri__blk1117);
        (assign57300_e89228, (((((locals.var_weff_nf * locals.var_t1_dn0) * locals.var_mu_res) + (assign57300_e89224 * locals.var_mu_res_dn0)) * locals.var_edri__blk1117) + (assign57300_e89226 * locals.var_edri__blk1117_dn0)), (((((locals.var_weff_nf * locals.var_t1_dn2) * locals.var_mu_res) + (assign57300_e89224 * locals.var_mu_res_dn2)) * locals.var_edri__blk1117) + (assign57300_e89226 * locals.var_edri__blk1117_dn2)), (((((locals.var_weff_nf * locals.var_t1_dn4) * locals.var_mu_res) + (assign57300_e89224 * locals.var_mu_res_dn4)) * locals.var_edri__blk1117) + (assign57300_e89226 * locals.var_edri__blk1117_dn4)), (((((locals.var_weff_nf * locals.var_t1_dn5) * locals.var_mu_res) + (assign57300_e89224 * locals.var_mu_res_dn5)) * locals.var_edri__blk1117) + (assign57300_e89226 * locals.var_edri__blk1117_dn5)), (((((locals.var_weff_nf * locals.var_t1_dn6) * locals.var_mu_res) + (assign57300_e89224 * locals.var_mu_res_dn6)) * locals.var_edri__blk1117) + (assign57300_e89226 * locals.var_edri__blk1117_dn6)), (((((locals.var_weff_nf * locals.var_t1_dn7) * locals.var_mu_res) + (assign57300_e89224 * locals.var_mu_res_dn7)) * locals.var_edri__blk1117) + (assign57300_e89226 * locals.var_edri__blk1117_dn7)), (((((locals.var_weff_nf * locals.var_t1_dn8) * locals.var_mu_res) + (assign57300_e89224 * locals.var_mu_res_dn8)) * locals.var_edri__blk1117) + (assign57300_e89226 * locals.var_edri__blk1117_dn8)), (((((locals.var_weff_nf * locals.var_t1_dn9) * locals.var_mu_res) + (assign57300_e89224 * locals.var_mu_res_dn9)) * locals.var_edri__blk1117) + (assign57300_e89226 * locals.var_edri__blk1117_dn9)), (((((locals.var_weff_nf * locals.var_t1_dn10) * locals.var_mu_res) + (assign57300_e89224 * locals.var_mu_res_dn10)) * locals.var_edri__blk1117) + (assign57300_e89226 * locals.var_edri__blk1117_dn10)), (((((locals.var_weff_nf * locals.var_t1_dn11) * locals.var_mu_res) + (assign57300_e89224 * locals.var_mu_res_dn11)) * locals.var_edri__blk1117) + (assign57300_e89226 * locals.var_edri__blk1117_dn11)), (((((locals.var_weff_nf * locals.var_t1_dn14) * locals.var_mu_res) + (assign57300_e89224 * locals.var_mu_res_dn14)) * locals.var_edri__blk1117) + (assign57300_e89226 * locals.var_edri__blk1117_dn14)),)
    } else {
        (locals.var_ids_res, locals.var_ids_res_dn0, locals.var_ids_res_dn2, locals.var_ids_res_dn4, locals.var_ids_res_dn5, locals.var_ids_res_dn6, locals.var_ids_res_dn7, locals.var_ids_res_dn8, locals.var_ids_res_dn9, locals.var_ids_res_dn10, locals.var_ids_res_dn11, locals.var_ids_res_dn14,)
    }
};
        locals.var_ids_res = assign57300_e89230;
        locals.var_ids_res_dn0 = assign57300_e89230_d_n0;
        locals.var_ids_res_dn2 = assign57300_e89230_d_n2;
        locals.var_ids_res_dn4 = assign57300_e89230_d_n4;
        locals.var_ids_res_dn5 = assign57300_e89230_d_n5;
        locals.var_ids_res_dn6 = assign57300_e89230_d_n6;
        locals.var_ids_res_dn7 = assign57300_e89230_d_n7;
        locals.var_ids_res_dn8 = assign57300_e89230_d_n8;
        locals.var_ids_res_dn9 = assign57300_e89230_d_n9;
        locals.var_ids_res_dn10 = assign57300_e89230_d_n10;
        locals.var_ids_res_dn11 = assign57300_e89230_d_n11;
        locals.var_ids_res_dn14 = assign57300_e89230_d_n14;
        locals.var_ids_res_rv = 0.0;

        let (assign57310_e89247, assign57310_e89247_d_n0, assign57310_e89247_d_n2, assign57310_e89247_d_n4, assign57310_e89247_d_n5, assign57310_e89247_d_n6, assign57310_e89247_d_n7, assign57310_e89247_d_n8, assign57310_e89247_d_n9, assign57310_e89247_d_n10, assign57310_e89247_d_n11, assign57310_e89247_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign57310_e89241: f64 = (locals.var_t2 * locals.var_w_res_leak);
        let assign57310_e89243: f64 = (assign57310_e89241 * p.p363);
        let assign57310_e89245: f64 = (assign57310_e89243 * locals.var_vds_res0_sym);
        (assign57310_e89245, (((((locals.var_t2_dn0 * locals.var_w_res_leak) + (locals.var_t2 * locals.var_w_res_leak_dn0)) * p.p363) * locals.var_vds_res0_sym) + (assign57310_e89243 * locals.var_vds_res0_sym_dn0)), (((((locals.var_t2_dn2 * locals.var_w_res_leak) + (locals.var_t2 * locals.var_w_res_leak_dn2)) * p.p363) * locals.var_vds_res0_sym) + (assign57310_e89243 * locals.var_vds_res0_sym_dn2)), (((((locals.var_t2_dn4 * locals.var_w_res_leak) + (locals.var_t2 * locals.var_w_res_leak_dn4)) * p.p363) * locals.var_vds_res0_sym) + (assign57310_e89243 * locals.var_vds_res0_sym_dn4)), (((((locals.var_t2_dn5 * locals.var_w_res_leak) + (locals.var_t2 * locals.var_w_res_leak_dn5)) * p.p363) * locals.var_vds_res0_sym) + (assign57310_e89243 * locals.var_vds_res0_sym_dn5)), (((((locals.var_t2_dn6 * locals.var_w_res_leak) + (locals.var_t2 * locals.var_w_res_leak_dn6)) * p.p363) * locals.var_vds_res0_sym) + (assign57310_e89243 * locals.var_vds_res0_sym_dn6)), (((((locals.var_t2_dn7 * locals.var_w_res_leak) + (locals.var_t2 * locals.var_w_res_leak_dn7)) * p.p363) * locals.var_vds_res0_sym) + (assign57310_e89243 * locals.var_vds_res0_sym_dn7)), (((((locals.var_t2_dn8 * locals.var_w_res_leak) + (locals.var_t2 * locals.var_w_res_leak_dn8)) * p.p363) * locals.var_vds_res0_sym) + (assign57310_e89243 * locals.var_vds_res0_sym_dn8)), (((((locals.var_t2_dn9 * locals.var_w_res_leak) + (locals.var_t2 * locals.var_w_res_leak_dn9)) * p.p363) * locals.var_vds_res0_sym) + (assign57310_e89243 * locals.var_vds_res0_sym_dn9)), (((((locals.var_t2_dn10 * locals.var_w_res_leak) + (locals.var_t2 * locals.var_w_res_leak_dn10)) * p.p363) * locals.var_vds_res0_sym) + (assign57310_e89243 * locals.var_vds_res0_sym_dn10)), (((((locals.var_t2_dn11 * locals.var_w_res_leak) + (locals.var_t2 * locals.var_w_res_leak_dn11)) * p.p363) * locals.var_vds_res0_sym) + (assign57310_e89243 * locals.var_vds_res0_sym_dn11)), (((((locals.var_t2_dn14 * locals.var_w_res_leak) + (locals.var_t2 * locals.var_w_res_leak_dn14)) * p.p363) * locals.var_vds_res0_sym) + (assign57310_e89243 * locals.var_vds_res0_sym_dn14)),)
    } else {
        (locals.var_ires_leak, locals.var_ires_leak_dn0, locals.var_ires_leak_dn2, locals.var_ires_leak_dn4, locals.var_ires_leak_dn5, locals.var_ires_leak_dn6, locals.var_ires_leak_dn7, locals.var_ires_leak_dn8, locals.var_ires_leak_dn9, locals.var_ires_leak_dn10, locals.var_ires_leak_dn11, locals.var_ires_leak_dn14,)
    }
};
        locals.var_ires_leak = assign57310_e89247;
        locals.var_ires_leak_dn0 = assign57310_e89247_d_n0;
        locals.var_ires_leak_dn2 = assign57310_e89247_d_n2;
        locals.var_ires_leak_dn4 = assign57310_e89247_d_n4;
        locals.var_ires_leak_dn5 = assign57310_e89247_d_n5;
        locals.var_ires_leak_dn6 = assign57310_e89247_d_n6;
        locals.var_ires_leak_dn7 = assign57310_e89247_d_n7;
        locals.var_ires_leak_dn8 = assign57310_e89247_d_n8;
        locals.var_ires_leak_dn9 = assign57310_e89247_d_n9;
        locals.var_ires_leak_dn10 = assign57310_e89247_d_n10;
        locals.var_ires_leak_dn11 = assign57310_e89247_d_n11;
        locals.var_ires_leak_dn14 = assign57310_e89247_d_n14;
        locals.var_ires_leak_rv = 0.0;

        let (assign57320_e89262, assign57320_e89262_d_n0, assign57320_e89262_d_n2, assign57320_e89262_d_n4, assign57320_e89262_d_n5, assign57320_e89262_d_n6, assign57320_e89262_d_n7, assign57320_e89262_d_n8, assign57320_e89262_d_n9, assign57320_e89262_d_n10, assign57320_e89262_d_n11, assign57320_e89262_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign57320_e89258: f64 = (locals.var_weff_nf * locals.var_beta_inv);
        let assign57320_e89260: f64 = (assign57320_e89258 / locals.var_lch);
        (assign57320_e89260, ((((locals.var_weff_nf * locals.var_beta_inv_dn0) * locals.var_lch) - (assign57320_e89258 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn2) * locals.var_lch) - (assign57320_e89258 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn4) * locals.var_lch) - (assign57320_e89258 * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn5) * locals.var_lch) - (assign57320_e89258 * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn6) * locals.var_lch) - (assign57320_e89258 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn7) * locals.var_lch) - (assign57320_e89258 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn8) * locals.var_lch) - (assign57320_e89258 * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn9) * locals.var_lch) - (assign57320_e89258 * locals.var_lch_dn9)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn10) * locals.var_lch) - (assign57320_e89258 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn11) * locals.var_lch) - (assign57320_e89258 * locals.var_lch_dn11)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn14) * locals.var_lch) - (assign57320_e89258 * locals.var_lch_dn14)) / (locals.var_lch * locals.var_lch)),)
    } else {
        (locals.var_betawl, locals.var_betawl_dn0, locals.var_betawl_dn2, locals.var_betawl_dn4, locals.var_betawl_dn5, locals.var_betawl_dn6, locals.var_betawl_dn7, locals.var_betawl_dn8, locals.var_betawl_dn9, locals.var_betawl_dn10, locals.var_betawl_dn11, locals.var_betawl_dn14,)
    }
};
        locals.var_betawl = assign57320_e89262;
        locals.var_betawl_dn0 = assign57320_e89262_d_n0;
        locals.var_betawl_dn2 = assign57320_e89262_d_n2;
        locals.var_betawl_dn4 = assign57320_e89262_d_n4;
        locals.var_betawl_dn5 = assign57320_e89262_d_n5;
        locals.var_betawl_dn6 = assign57320_e89262_d_n6;
        locals.var_betawl_dn7 = assign57320_e89262_d_n7;
        locals.var_betawl_dn8 = assign57320_e89262_d_n8;
        locals.var_betawl_dn9 = assign57320_e89262_d_n9;
        locals.var_betawl_dn10 = assign57320_e89262_d_n10;
        locals.var_betawl_dn11 = assign57320_e89262_d_n11;
        locals.var_betawl_dn14 = assign57320_e89262_d_n14;
        locals.var_betawl_rv = 0.0;

        let (assign57330_e89277, assign57330_e89277_d_n0, assign57330_e89277_d_n2, assign57330_e89277_d_n4, assign57330_e89277_d_n5, assign57330_e89277_d_n6, assign57330_e89277_d_n7, assign57330_e89277_d_n8, assign57330_e89277_d_n9, assign57330_e89277_d_n10, assign57330_e89277_d_n11, assign57330_e89277_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign57330_e89273: f64 = (locals.var_betawl * locals.var_idd);
        let assign57330_e89275: f64 = (assign57330_e89273 * locals.var_mu_acc);
        (assign57330_e89275, ((((locals.var_betawl_dn0 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn0)) * locals.var_mu_acc) + (assign57330_e89273 * locals.var_mu_acc_dn0)), ((((locals.var_betawl_dn2 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn2)) * locals.var_mu_acc) + (assign57330_e89273 * locals.var_mu_acc_dn2)), ((((locals.var_betawl_dn4 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn4)) * locals.var_mu_acc) + (assign57330_e89273 * locals.var_mu_acc_dn4)), ((((locals.var_betawl_dn5 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn5)) * locals.var_mu_acc) + (assign57330_e89273 * locals.var_mu_acc_dn5)), ((((locals.var_betawl_dn6 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn6)) * locals.var_mu_acc) + (assign57330_e89273 * locals.var_mu_acc_dn6)), ((((locals.var_betawl_dn7 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn7)) * locals.var_mu_acc) + (assign57330_e89273 * locals.var_mu_acc_dn7)), ((((locals.var_betawl_dn8 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn8)) * locals.var_mu_acc) + (assign57330_e89273 * locals.var_mu_acc_dn8)), ((((locals.var_betawl_dn9 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn9)) * locals.var_mu_acc) + (assign57330_e89273 * locals.var_mu_acc_dn9)), ((((locals.var_betawl_dn10 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn10)) * locals.var_mu_acc) + (assign57330_e89273 * locals.var_mu_acc_dn10)), ((((locals.var_betawl_dn11 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn11)) * locals.var_mu_acc) + (assign57330_e89273 * locals.var_mu_acc_dn11)), ((((locals.var_betawl_dn14 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn14)) * locals.var_mu_acc) + (assign57330_e89273 * locals.var_mu_acc_dn14)),)
    } else {
        (locals.var_ids_acc, locals.var_ids_acc_dn0, locals.var_ids_acc_dn2, locals.var_ids_acc_dn4, locals.var_ids_acc_dn5, locals.var_ids_acc_dn6, locals.var_ids_acc_dn7, locals.var_ids_acc_dn8, locals.var_ids_acc_dn9, locals.var_ids_acc_dn10, locals.var_ids_acc_dn11, locals.var_ids_acc_dn14,)
    }
};
        locals.var_ids_acc = assign57330_e89277;
        locals.var_ids_acc_dn0 = assign57330_e89277_d_n0;
        locals.var_ids_acc_dn2 = assign57330_e89277_d_n2;
        locals.var_ids_acc_dn4 = assign57330_e89277_d_n4;
        locals.var_ids_acc_dn5 = assign57330_e89277_d_n5;
        locals.var_ids_acc_dn6 = assign57330_e89277_d_n6;
        locals.var_ids_acc_dn7 = assign57330_e89277_d_n7;
        locals.var_ids_acc_dn8 = assign57330_e89277_d_n8;
        locals.var_ids_acc_dn9 = assign57330_e89277_d_n9;
        locals.var_ids_acc_dn10 = assign57330_e89277_d_n10;
        locals.var_ids_acc_dn11 = assign57330_e89277_d_n11;
        locals.var_ids_acc_dn14 = assign57330_e89277_d_n14;
        locals.var_ids_acc_rv = 0.0;

        let (assign57340_e89298, assign57340_e89298_d_n0, assign57340_e89298_d_n2, assign57340_e89298_d_n4, assign57340_e89298_d_n5, assign57340_e89298_d_n6, assign57340_e89298_d_n7, assign57340_e89298_d_n8, assign57340_e89298_d_n9, assign57340_e89298_d_n10, assign57340_e89298_d_n11, assign57340_e89298_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign57340_e89288: f64 = locals.var_ids_acc;
        let assign57340_e89291: f64 = locals.var_ids_res;
        let assign57340_e89292: f64 = (assign57340_e89288 + assign57340_e89291);
        let assign57340_e89295: f64 = locals.var_ires_leak;
        let assign57340_e89296: f64 = (assign57340_e89292 + assign57340_e89295);
        (assign57340_e89296, ((locals.var_ids_acc_dn0 + locals.var_ids_res_dn0) + locals.var_ires_leak_dn0), ((locals.var_ids_acc_dn2 + locals.var_ids_res_dn2) + locals.var_ires_leak_dn2), ((locals.var_ids_acc_dn4 + locals.var_ids_res_dn4) + locals.var_ires_leak_dn4), ((locals.var_ids_acc_dn5 + locals.var_ids_res_dn5) + locals.var_ires_leak_dn5), ((locals.var_ids_acc_dn6 + locals.var_ids_res_dn6) + locals.var_ires_leak_dn6), ((locals.var_ids_acc_dn7 + locals.var_ids_res_dn7) + locals.var_ires_leak_dn7), ((locals.var_ids_acc_dn8 + locals.var_ids_res_dn8) + locals.var_ires_leak_dn8), ((locals.var_ids_acc_dn9 + locals.var_ids_res_dn9) + locals.var_ires_leak_dn9), ((locals.var_ids_acc_dn10 + locals.var_ids_res_dn10) + locals.var_ires_leak_dn10), ((locals.var_ids_acc_dn11 + locals.var_ids_res_dn11) + locals.var_ires_leak_dn11), ((locals.var_ids_acc_dn14 + locals.var_ids_res_dn14) + locals.var_ires_leak_dn14),)
    } else {
        (locals.var_ids0, locals.var_ids0_dn0, locals.var_ids0_dn2, locals.var_ids0_dn4, locals.var_ids0_dn5, locals.var_ids0_dn6, locals.var_ids0_dn7, locals.var_ids0_dn8, locals.var_ids0_dn9, locals.var_ids0_dn10, locals.var_ids0_dn11, locals.var_ids0_dn14,)
    }
};
        locals.var_ids0 = assign57340_e89298;
        locals.var_ids0_dn0 = assign57340_e89298_d_n0;
        locals.var_ids0_dn2 = assign57340_e89298_d_n2;
        locals.var_ids0_dn4 = assign57340_e89298_d_n4;
        locals.var_ids0_dn5 = assign57340_e89298_d_n5;
        locals.var_ids0_dn6 = assign57340_e89298_d_n6;
        locals.var_ids0_dn7 = assign57340_e89298_d_n7;
        locals.var_ids0_dn8 = assign57340_e89298_d_n8;
        locals.var_ids0_dn9 = assign57340_e89298_d_n9;
        locals.var_ids0_dn10 = assign57340_e89298_d_n10;
        locals.var_ids0_dn11 = assign57340_e89298_d_n11;
        locals.var_ids0_dn14 = assign57340_e89298_d_n14;
        locals.var_ids0_rv = 0.0;

        let (assign57350_e89309, assign57350_e89309_d_n0, assign57350_e89309_d_n2, assign57350_e89309_d_n4, assign57350_e89309_d_n5, assign57350_e89309_d_n6, assign57350_e89309_d_n7, assign57350_e89309_d_n8, assign57350_e89309_d_n9, assign57350_e89309_d_n10, assign57350_e89309_d_n11, assign57350_e89309_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn4, locals.var_vdsorg_dn5, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn8, locals.var_vdsorg_dn9, locals.var_vdsorg_dn10, locals.var_vdsorg_dn11, locals.var_vdsorg_dn14,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn14,)
    }
};
        locals.var_vds = assign57350_e89309;
        locals.var_vds_dn0 = assign57350_e89309_d_n0;
        locals.var_vds_dn2 = assign57350_e89309_d_n2;
        locals.var_vds_dn4 = assign57350_e89309_d_n4;
        locals.var_vds_dn5 = assign57350_e89309_d_n5;
        locals.var_vds_dn6 = assign57350_e89309_d_n6;
        locals.var_vds_dn7 = assign57350_e89309_d_n7;
        locals.var_vds_dn8 = assign57350_e89309_d_n8;
        locals.var_vds_dn9 = assign57350_e89309_d_n9;
        locals.var_vds_dn10 = assign57350_e89309_d_n10;
        locals.var_vds_dn11 = assign57350_e89309_d_n11;
        locals.var_vds_dn14 = assign57350_e89309_d_n14;
        locals.var_vds_rv = 0.0;

        let assign57360_e89312: f64 = if p.p283 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1425 = assign57360_e89312;
        locals.var_guard1425_rv = 0.0;

        let (assign57370_e89329, assign57370_e89329_d_n0, assign57370_e89329_d_n2, assign57370_e89329_d_n4, assign57370_e89329_d_n5, assign57370_e89329_d_n6, assign57370_e89329_d_n7, assign57370_e89329_d_n8, assign57370_e89329_d_n9, assign57370_e89329_d_n10, assign57370_e89329_d_n11, assign57370_e89329_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1425 != 0.0)) {
        let assign57370_e89326: f64 = (locals.var_vds - locals.var_pds);
        let assign57370_e89327: f64 = (0.5 * assign57370_e89326);
        (assign57370_e89327, (0.5 * (locals.var_vds_dn0 - locals.var_pds_dn0)), (0.5 * (locals.var_vds_dn2 - locals.var_pds_dn2)), (0.5 * (locals.var_vds_dn4 - locals.var_pds_dn4)), (0.5 * (locals.var_vds_dn5 - locals.var_pds_dn5)), (0.5 * (locals.var_vds_dn6 - locals.var_pds_dn6)), (0.5 * (locals.var_vds_dn7 - locals.var_pds_dn7)), (0.5 * (locals.var_vds_dn8 - locals.var_pds_dn8)), (0.5 * (locals.var_vds_dn9 - locals.var_pds_dn9)), (0.5 * (locals.var_vds_dn10 - locals.var_pds_dn10)), (0.5 * (locals.var_vds_dn11 - locals.var_pds_dn11)), (0.5 * (locals.var_vds_dn14 - locals.var_pds_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign57370_e89329;
        locals.var_t1_dn0 = assign57370_e89329_d_n0;
        locals.var_t1_dn2 = assign57370_e89329_d_n2;
        locals.var_t1_dn4 = assign57370_e89329_d_n4;
        locals.var_t1_dn5 = assign57370_e89329_d_n5;
        locals.var_t1_dn6 = assign57370_e89329_d_n6;
        locals.var_t1_dn7 = assign57370_e89329_d_n7;
        locals.var_t1_dn8 = assign57370_e89329_d_n8;
        locals.var_t1_dn9 = assign57370_e89329_d_n9;
        locals.var_t1_dn10 = assign57370_e89329_d_n10;
        locals.var_t1_dn11 = assign57370_e89329_d_n11;
        locals.var_t1_dn14 = assign57370_e89329_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign57380_e89346, assign57380_e89346_d_n0, assign57380_e89346_d_n2, assign57380_e89346_d_n4, assign57380_e89346_d_n5, assign57380_e89346_d_n6, assign57380_e89346_d_n7, assign57380_e89346_d_n8, assign57380_e89346_d_n9, assign57380_e89346_d_n10, assign57380_e89346_d_n11, assign57380_e89346_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1425 != 0.0)) {
        let assign57380_e89342: f64 = (2.0 * locals.var_t1);
        let assign57380_e89344: f64 = (assign57380_e89342 / 0.01);
        (assign57380_e89344, ((2.0 * locals.var_t1_dn0) / 0.01), ((2.0 * locals.var_t1_dn2) / 0.01), ((2.0 * locals.var_t1_dn4) / 0.01), ((2.0 * locals.var_t1_dn5) / 0.01), ((2.0 * locals.var_t1_dn6) / 0.01), ((2.0 * locals.var_t1_dn7) / 0.01), ((2.0 * locals.var_t1_dn8) / 0.01), ((2.0 * locals.var_t1_dn9) / 0.01), ((2.0 * locals.var_t1_dn10) / 0.01), ((2.0 * locals.var_t1_dn11) / 0.01), ((2.0 * locals.var_t1_dn14) / 0.01),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign57380_e89346;
        locals.var_tmf1_dn0 = assign57380_e89346_d_n0;
        locals.var_tmf1_dn2 = assign57380_e89346_d_n2;
        locals.var_tmf1_dn4 = assign57380_e89346_d_n4;
        locals.var_tmf1_dn5 = assign57380_e89346_d_n5;
        locals.var_tmf1_dn6 = assign57380_e89346_d_n6;
        locals.var_tmf1_dn7 = assign57380_e89346_d_n7;
        locals.var_tmf1_dn8 = assign57380_e89346_d_n8;
        locals.var_tmf1_dn9 = assign57380_e89346_d_n9;
        locals.var_tmf1_dn10 = assign57380_e89346_d_n10;
        locals.var_tmf1_dn11 = assign57380_e89346_d_n11;
        locals.var_tmf1_dn14 = assign57380_e89346_d_n14;
        locals.var_tmf1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_210(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign57390_e89395, assign57390_e89395_d_n0, assign57390_e89395_d_n2, assign57390_e89395_d_n4, assign57390_e89395_d_n5, assign57390_e89395_d_n6, assign57390_e89395_d_n7, assign57390_e89395_d_n8, assign57390_e89395_d_n9, assign57390_e89395_d_n10, assign57390_e89395_d_n11, assign57390_e89395_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1425 != 0.0)) {
        let assign57390_e89361: f64 = (1.0 / 2.0);
        let assign57390_e89365: f64 = (1.0 / 6.0);
        let assign57390_e89369: f64 = (1.0 / 24.0);
        let assign57390_e89373: f64 = (1.0 / 120.0);
        let assign57390_e89377: f64 = (1.0 / 720.0);
        let assign57390_e89381: f64 = (1.0 / 5040.0);
        let assign57390_e89382: f64 = (locals.var_tmf1 * assign57390_e89381);
        let assign57390_e89383: f64 = (assign57390_e89377 + assign57390_e89382);
        let assign57390_e89384: f64 = (locals.var_tmf1 * assign57390_e89383);
        let assign57390_e89385: f64 = (assign57390_e89373 + assign57390_e89384);
        let assign57390_e89386: f64 = (locals.var_tmf1 * assign57390_e89385);
        let assign57390_e89387: f64 = (assign57390_e89369 + assign57390_e89386);
        let assign57390_e89388: f64 = (locals.var_tmf1 * assign57390_e89387);
        let assign57390_e89389: f64 = (assign57390_e89365 + assign57390_e89388);
        let assign57390_e89390: f64 = (locals.var_tmf1 * assign57390_e89389);
        let assign57390_e89391: f64 = (assign57390_e89361 + assign57390_e89390);
        let assign57390_e89392: f64 = (locals.var_tmf1 * assign57390_e89391);
        let assign57390_e89393: f64 = (1.0 + assign57390_e89392);
        (assign57390_e89393, ((locals.var_tmf1_dn0 * assign57390_e89391) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign57390_e89389) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign57390_e89387) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign57390_e89385) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign57390_e89383) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign57390_e89381))))))))))), ((locals.var_tmf1_dn2 * assign57390_e89391) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign57390_e89389) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign57390_e89387) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign57390_e89385) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign57390_e89383) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign57390_e89381))))))))))), ((locals.var_tmf1_dn4 * assign57390_e89391) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign57390_e89389) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign57390_e89387) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign57390_e89385) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign57390_e89383) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign57390_e89381))))))))))), ((locals.var_tmf1_dn5 * assign57390_e89391) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign57390_e89389) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign57390_e89387) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign57390_e89385) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign57390_e89383) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign57390_e89381))))))))))), ((locals.var_tmf1_dn6 * assign57390_e89391) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign57390_e89389) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign57390_e89387) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign57390_e89385) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign57390_e89383) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign57390_e89381))))))))))), ((locals.var_tmf1_dn7 * assign57390_e89391) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign57390_e89389) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign57390_e89387) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign57390_e89385) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign57390_e89383) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign57390_e89381))))))))))), ((locals.var_tmf1_dn8 * assign57390_e89391) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign57390_e89389) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign57390_e89387) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign57390_e89385) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign57390_e89383) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign57390_e89381))))))))))), ((locals.var_tmf1_dn9 * assign57390_e89391) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign57390_e89389) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign57390_e89387) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign57390_e89385) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign57390_e89383) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign57390_e89381))))))))))), ((locals.var_tmf1_dn10 * assign57390_e89391) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign57390_e89389) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign57390_e89387) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign57390_e89385) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign57390_e89383) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign57390_e89381))))))))))), ((locals.var_tmf1_dn11 * assign57390_e89391) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign57390_e89389) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign57390_e89387) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign57390_e89385) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign57390_e89383) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign57390_e89381))))))))))), ((locals.var_tmf1_dn14 * assign57390_e89391) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign57390_e89389) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign57390_e89387) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign57390_e89385) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign57390_e89383) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign57390_e89381))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign57390_e89395;
        locals.var_tmf2_dn0 = assign57390_e89395_d_n0;
        locals.var_tmf2_dn2 = assign57390_e89395_d_n2;
        locals.var_tmf2_dn4 = assign57390_e89395_d_n4;
        locals.var_tmf2_dn5 = assign57390_e89395_d_n5;
        locals.var_tmf2_dn6 = assign57390_e89395_d_n6;
        locals.var_tmf2_dn7 = assign57390_e89395_d_n7;
        locals.var_tmf2_dn8 = assign57390_e89395_d_n8;
        locals.var_tmf2_dn9 = assign57390_e89395_d_n9;
        locals.var_tmf2_dn10 = assign57390_e89395_d_n10;
        locals.var_tmf2_dn11 = assign57390_e89395_d_n11;
        locals.var_tmf2_dn14 = assign57390_e89395_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign57400_e89440, assign57400_e89440_d_n0, assign57400_e89440_d_n2, assign57400_e89440_d_n4, assign57400_e89440_d_n5, assign57400_e89440_d_n6, assign57400_e89440_d_n7, assign57400_e89440_d_n8, assign57400_e89440_d_n9, assign57400_e89440_d_n10, assign57400_e89440_d_n11, assign57400_e89440_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1425 != 0.0)) {
        let assign57400_e89408: f64 = (1.0 / 2.0);
        let assign57400_e89412: f64 = (1.0 / 3.0);
        let assign57400_e89416: f64 = (1.0 / 8.0);
        let assign57400_e89420: f64 = (1.0 / 30.0);
        let assign57400_e89424: f64 = (1.0 / 144.0);
        let assign57400_e89428: f64 = (1.0 / 840.0);
        let assign57400_e89429: f64 = (locals.var_tmf1 * assign57400_e89428);
        let assign57400_e89430: f64 = (assign57400_e89424 + assign57400_e89429);
        let assign57400_e89431: f64 = (locals.var_tmf1 * assign57400_e89430);
        let assign57400_e89432: f64 = (assign57400_e89420 + assign57400_e89431);
        let assign57400_e89433: f64 = (locals.var_tmf1 * assign57400_e89432);
        let assign57400_e89434: f64 = (assign57400_e89416 + assign57400_e89433);
        let assign57400_e89435: f64 = (locals.var_tmf1 * assign57400_e89434);
        let assign57400_e89436: f64 = (assign57400_e89412 + assign57400_e89435);
        let assign57400_e89437: f64 = (locals.var_tmf1 * assign57400_e89436);
        let assign57400_e89438: f64 = (assign57400_e89408 + assign57400_e89437);
        (assign57400_e89438, ((locals.var_tmf1_dn0 * assign57400_e89436) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign57400_e89434) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign57400_e89432) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign57400_e89430) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign57400_e89428))))))))), ((locals.var_tmf1_dn2 * assign57400_e89436) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign57400_e89434) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign57400_e89432) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign57400_e89430) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign57400_e89428))))))))), ((locals.var_tmf1_dn4 * assign57400_e89436) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign57400_e89434) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign57400_e89432) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign57400_e89430) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign57400_e89428))))))))), ((locals.var_tmf1_dn5 * assign57400_e89436) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign57400_e89434) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign57400_e89432) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign57400_e89430) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign57400_e89428))))))))), ((locals.var_tmf1_dn6 * assign57400_e89436) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign57400_e89434) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign57400_e89432) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign57400_e89430) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign57400_e89428))))))))), ((locals.var_tmf1_dn7 * assign57400_e89436) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign57400_e89434) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign57400_e89432) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign57400_e89430) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign57400_e89428))))))))), ((locals.var_tmf1_dn8 * assign57400_e89436) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign57400_e89434) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign57400_e89432) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign57400_e89430) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign57400_e89428))))))))), ((locals.var_tmf1_dn9 * assign57400_e89436) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign57400_e89434) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign57400_e89432) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign57400_e89430) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign57400_e89428))))))))), ((locals.var_tmf1_dn10 * assign57400_e89436) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign57400_e89434) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign57400_e89432) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign57400_e89430) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign57400_e89428))))))))), ((locals.var_tmf1_dn11 * assign57400_e89436) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign57400_e89434) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign57400_e89432) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign57400_e89430) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign57400_e89428))))))))), ((locals.var_tmf1_dn14 * assign57400_e89436) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign57400_e89434) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign57400_e89432) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign57400_e89430) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign57400_e89428))))))))),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign57400_e89440;
        locals.var_tmf3_dn0 = assign57400_e89440_d_n0;
        locals.var_tmf3_dn2 = assign57400_e89440_d_n2;
        locals.var_tmf3_dn4 = assign57400_e89440_d_n4;
        locals.var_tmf3_dn5 = assign57400_e89440_d_n5;
        locals.var_tmf3_dn6 = assign57400_e89440_d_n6;
        locals.var_tmf3_dn7 = assign57400_e89440_d_n7;
        locals.var_tmf3_dn8 = assign57400_e89440_d_n8;
        locals.var_tmf3_dn9 = assign57400_e89440_d_n9;
        locals.var_tmf3_dn10 = assign57400_e89440_d_n10;
        locals.var_tmf3_dn11 = assign57400_e89440_d_n11;
        locals.var_tmf3_dn14 = assign57400_e89440_d_n14;
        locals.var_tmf3_rv = 0.0;

        let (assign57410_e89455, assign57410_e89455_d_n0, assign57410_e89455_d_n2, assign57410_e89455_d_n4, assign57410_e89455_d_n5, assign57410_e89455_d_n6, assign57410_e89455_d_n7, assign57410_e89455_d_n8, assign57410_e89455_d_n9, assign57410_e89455_d_n10, assign57410_e89455_d_n11, assign57410_e89455_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1425 != 0.0)) {
        let assign57410_e89453: f64 = (0.01 / locals.var_tmf2);
        (assign57410_e89453, (-((0.01 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign57410_e89455;
        locals.var_t6_dn0 = assign57410_e89455_d_n0;
        locals.var_t6_dn2 = assign57410_e89455_d_n2;
        locals.var_t6_dn4 = assign57410_e89455_d_n4;
        locals.var_t6_dn5 = assign57410_e89455_d_n5;
        locals.var_t6_dn6 = assign57410_e89455_d_n6;
        locals.var_t6_dn7 = assign57410_e89455_d_n7;
        locals.var_t6_dn8 = assign57410_e89455_d_n8;
        locals.var_t6_dn9 = assign57410_e89455_d_n9;
        locals.var_t6_dn10 = assign57410_e89455_d_n10;
        locals.var_t6_dn11 = assign57410_e89455_d_n11;
        locals.var_t6_dn14 = assign57410_e89455_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign57420_e89475, assign57420_e89475_d_n0, assign57420_e89475_d_n2, assign57420_e89475_d_n4, assign57420_e89475_d_n5, assign57420_e89475_d_n6, assign57420_e89475_d_n7, assign57420_e89475_d_n8, assign57420_e89475_d_n9, assign57420_e89475_d_n10, assign57420_e89475_d_n11, assign57420_e89475_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1425 != 0.0)) {
        let assign57420_e89467: f64 = (-2.0);
        let assign57420_e89469: f64 = (assign57420_e89467 * locals.var_tmf3);
        let assign57420_e89472: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign57420_e89473: f64 = (assign57420_e89469 / assign57420_e89472);
        (assign57420_e89473, ((((assign57420_e89467 * locals.var_tmf3_dn0) * assign57420_e89472) - (assign57420_e89469 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign57420_e89472 * assign57420_e89472)), ((((assign57420_e89467 * locals.var_tmf3_dn2) * assign57420_e89472) - (assign57420_e89469 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign57420_e89472 * assign57420_e89472)), ((((assign57420_e89467 * locals.var_tmf3_dn4) * assign57420_e89472) - (assign57420_e89469 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign57420_e89472 * assign57420_e89472)), ((((assign57420_e89467 * locals.var_tmf3_dn5) * assign57420_e89472) - (assign57420_e89469 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign57420_e89472 * assign57420_e89472)), ((((assign57420_e89467 * locals.var_tmf3_dn6) * assign57420_e89472) - (assign57420_e89469 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign57420_e89472 * assign57420_e89472)), ((((assign57420_e89467 * locals.var_tmf3_dn7) * assign57420_e89472) - (assign57420_e89469 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign57420_e89472 * assign57420_e89472)), ((((assign57420_e89467 * locals.var_tmf3_dn8) * assign57420_e89472) - (assign57420_e89469 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign57420_e89472 * assign57420_e89472)), ((((assign57420_e89467 * locals.var_tmf3_dn9) * assign57420_e89472) - (assign57420_e89469 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign57420_e89472 * assign57420_e89472)), ((((assign57420_e89467 * locals.var_tmf3_dn10) * assign57420_e89472) - (assign57420_e89469 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign57420_e89472 * assign57420_e89472)), ((((assign57420_e89467 * locals.var_tmf3_dn11) * assign57420_e89472) - (assign57420_e89469 * ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)))) / (assign57420_e89472 * assign57420_e89472)), ((((assign57420_e89467 * locals.var_tmf3_dn14) * assign57420_e89472) - (assign57420_e89469 * ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)))) / (assign57420_e89472 * assign57420_e89472)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign57420_e89475;
        locals.var_t2_dn0 = assign57420_e89475_d_n0;
        locals.var_t2_dn2 = assign57420_e89475_d_n2;
        locals.var_t2_dn4 = assign57420_e89475_d_n4;
        locals.var_t2_dn5 = assign57420_e89475_d_n5;
        locals.var_t2_dn6 = assign57420_e89475_d_n6;
        locals.var_t2_dn7 = assign57420_e89475_d_n7;
        locals.var_t2_dn8 = assign57420_e89475_d_n8;
        locals.var_t2_dn9 = assign57420_e89475_d_n9;
        locals.var_t2_dn10 = assign57420_e89475_d_n10;
        locals.var_t2_dn11 = assign57420_e89475_d_n11;
        locals.var_t2_dn14 = assign57420_e89475_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign57430_e89492, assign57430_e89492_d_n0, assign57430_e89492_d_n2, assign57430_e89492_d_n4, assign57430_e89492_d_n5, assign57430_e89492_d_n6, assign57430_e89492_d_n7, assign57430_e89492_d_n8, assign57430_e89492_d_n9, assign57430_e89492_d_n10, assign57430_e89492_d_n11, assign57430_e89492_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1425 != 0.0)) {
        let assign57430_e89489: f64 = (locals.var_phi_s0_dep__blk1091 + locals.var_t6);
        let assign57430_e89490: f64 = (1.1 - assign57430_e89489);
        (assign57430_e89490, (-(locals.var_phi_s0_dep__blk1091_dn0 + locals.var_t6_dn0)), (-(locals.var_phi_s0_dep__blk1091_dn2 + locals.var_t6_dn2)), (-(locals.var_phi_s0_dep__blk1091_dn4 + locals.var_t6_dn4)), (-(locals.var_phi_s0_dep__blk1091_dn5 + locals.var_t6_dn5)), (-(locals.var_phi_s0_dep__blk1091_dn6 + locals.var_t6_dn6)), (-(locals.var_phi_s0_dep__blk1091_dn7 + locals.var_t6_dn7)), (-(locals.var_phi_s0_dep__blk1091_dn8 + locals.var_t6_dn8)), (-(locals.var_phi_s0_dep__blk1091_dn9 + locals.var_t6_dn9)), (-(locals.var_phi_s0_dep__blk1091_dn10 + locals.var_t6_dn10)), (-(locals.var_phi_s0_dep__blk1091_dn11 + locals.var_t6_dn11)), (-(locals.var_phi_s0_dep__blk1091_dn14 + locals.var_t6_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign57430_e89492;
        locals.var_t1_dn0 = assign57430_e89492_d_n0;
        locals.var_t1_dn2 = assign57430_e89492_d_n2;
        locals.var_t1_dn4 = assign57430_e89492_d_n4;
        locals.var_t1_dn5 = assign57430_e89492_d_n5;
        locals.var_t1_dn6 = assign57430_e89492_d_n6;
        locals.var_t1_dn7 = assign57430_e89492_d_n7;
        locals.var_t1_dn8 = assign57430_e89492_d_n8;
        locals.var_t1_dn9 = assign57430_e89492_d_n9;
        locals.var_t1_dn10 = assign57430_e89492_d_n10;
        locals.var_t1_dn11 = assign57430_e89492_d_n11;
        locals.var_t1_dn14 = assign57430_e89492_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign57440_e89514, assign57440_e89514_d_n0, assign57440_e89514_d_n2, assign57440_e89514_d_n4, assign57440_e89514_d_n5, assign57440_e89514_d_n6, assign57440_e89514_d_n7, assign57440_e89514_d_n8, assign57440_e89514_d_n9, assign57440_e89514_d_n10, assign57440_e89514_d_n11, assign57440_e89514_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1425 != 0.0)) {
        let assign57440_e89505: f64 = (locals.var_t1 * locals.var_t1);
        let assign57440_e89508: f64 = (4.0 * 0.05);
        let assign57440_e89510: f64 = (assign57440_e89508 * 0.05);
        let assign57440_e89511: f64 = (assign57440_e89505 + assign57440_e89510);
        let assign57440_e89512: f64 = (assign57440_e89511).sqrt();
        (assign57440_e89512, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign57440_e89512)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign57440_e89512)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign57440_e89512)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign57440_e89512)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign57440_e89512)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign57440_e89512)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign57440_e89512)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign57440_e89512)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign57440_e89512)), (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign57440_e89512)), (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) / (2.0 * assign57440_e89512)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign57440_e89514;
        locals.var_tmf2_dn0 = assign57440_e89514_d_n0;
        locals.var_tmf2_dn2 = assign57440_e89514_d_n2;
        locals.var_tmf2_dn4 = assign57440_e89514_d_n4;
        locals.var_tmf2_dn5 = assign57440_e89514_d_n5;
        locals.var_tmf2_dn6 = assign57440_e89514_d_n6;
        locals.var_tmf2_dn7 = assign57440_e89514_d_n7;
        locals.var_tmf2_dn8 = assign57440_e89514_d_n8;
        locals.var_tmf2_dn9 = assign57440_e89514_d_n9;
        locals.var_tmf2_dn10 = assign57440_e89514_d_n10;
        locals.var_tmf2_dn11 = assign57440_e89514_d_n11;
        locals.var_tmf2_dn14 = assign57440_e89514_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign57450_e89533, assign57450_e89533_d_n0, assign57450_e89533_d_n2, assign57450_e89533_d_n4, assign57450_e89533_d_n5, assign57450_e89533_d_n6, assign57450_e89533_d_n7, assign57450_e89533_d_n8, assign57450_e89533_d_n9, assign57450_e89533_d_n10, assign57450_e89533_d_n11, assign57450_e89533_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1425 != 0.0)) {
        let assign57450_e89529: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign57450_e89530: f64 = (1.0 + assign57450_e89529);
        let assign57450_e89531: f64 = (0.5 * assign57450_e89530);
        (assign57450_e89531, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn7 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn9 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn11 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn14 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign57450_e89533;
        locals.var_t0_dn0 = assign57450_e89533_d_n0;
        locals.var_t0_dn2 = assign57450_e89533_d_n2;
        locals.var_t0_dn4 = assign57450_e89533_d_n4;
        locals.var_t0_dn5 = assign57450_e89533_d_n5;
        locals.var_t0_dn6 = assign57450_e89533_d_n6;
        locals.var_t0_dn7 = assign57450_e89533_d_n7;
        locals.var_t0_dn8 = assign57450_e89533_d_n8;
        locals.var_t0_dn9 = assign57450_e89533_d_n9;
        locals.var_t0_dn10 = assign57450_e89533_d_n10;
        locals.var_t0_dn11 = assign57450_e89533_d_n11;
        locals.var_t0_dn14 = assign57450_e89533_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign57460_e89550, assign57460_e89550_d_n0, assign57460_e89550_d_n2, assign57460_e89550_d_n4, assign57460_e89550_d_n5, assign57460_e89550_d_n6, assign57460_e89550_d_n7, assign57460_e89550_d_n8, assign57460_e89550_d_n9, assign57460_e89550_d_n10, assign57460_e89550_d_n11, assign57460_e89550_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1425 != 0.0)) {
        let assign57460_e89547: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign57460_e89548: f64 = (0.5 * assign57460_e89547);
        (assign57460_e89548, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign57460_e89550;
        locals.var_t2_dn0 = assign57460_e89550_d_n0;
        locals.var_t2_dn2 = assign57460_e89550_d_n2;
        locals.var_t2_dn4 = assign57460_e89550_d_n4;
        locals.var_t2_dn5 = assign57460_e89550_d_n5;
        locals.var_t2_dn6 = assign57460_e89550_d_n6;
        locals.var_t2_dn7 = assign57460_e89550_d_n7;
        locals.var_t2_dn8 = assign57460_e89550_d_n8;
        locals.var_t2_dn9 = assign57460_e89550_d_n9;
        locals.var_t2_dn10 = assign57460_e89550_d_n10;
        locals.var_t2_dn11 = assign57460_e89550_d_n11;
        locals.var_t2_dn14 = assign57460_e89550_d_n14;
        locals.var_t2_rv = 0.0;

        let assign57470_e89553: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1426 = assign57470_e89553;
        locals.var_guard1426_rv = 0.0;

        let (assign57480_e89568, assign57480_e89568_d_n0, assign57480_e89568_d_n2, assign57480_e89568_d_n4, assign57480_e89568_d_n5, assign57480_e89568_d_n6, assign57480_e89568_d_n7, assign57480_e89568_d_n8, assign57480_e89568_d_n9, assign57480_e89568_d_n10, assign57480_e89568_d_n11, assign57480_e89568_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1425 != 0.0)) && (locals.var_guard1426 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign57480_e89568;
        locals.var_t2_dn0 = assign57480_e89568_d_n0;
        locals.var_t2_dn2 = assign57480_e89568_d_n2;
        locals.var_t2_dn4 = assign57480_e89568_d_n4;
        locals.var_t2_dn5 = assign57480_e89568_d_n5;
        locals.var_t2_dn6 = assign57480_e89568_d_n6;
        locals.var_t2_dn7 = assign57480_e89568_d_n7;
        locals.var_t2_dn8 = assign57480_e89568_d_n8;
        locals.var_t2_dn9 = assign57480_e89568_d_n9;
        locals.var_t2_dn10 = assign57480_e89568_d_n10;
        locals.var_t2_dn11 = assign57480_e89568_d_n11;
        locals.var_t2_dn14 = assign57480_e89568_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign57490_e89583, assign57490_e89583_d_n0, assign57490_e89583_d_n2, assign57490_e89583_d_n4, assign57490_e89583_d_n5, assign57490_e89583_d_n6, assign57490_e89583_d_n7, assign57490_e89583_d_n8, assign57490_e89583_d_n9, assign57490_e89583_d_n10, assign57490_e89583_d_n11, assign57490_e89583_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1425 != 0.0)) && (locals.var_guard1426 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign57490_e89583;
        locals.var_t0_dn0 = assign57490_e89583_d_n0;
        locals.var_t0_dn2 = assign57490_e89583_d_n2;
        locals.var_t0_dn4 = assign57490_e89583_d_n4;
        locals.var_t0_dn5 = assign57490_e89583_d_n5;
        locals.var_t0_dn6 = assign57490_e89583_d_n6;
        locals.var_t0_dn7 = assign57490_e89583_d_n7;
        locals.var_t0_dn8 = assign57490_e89583_d_n8;
        locals.var_t0_dn9 = assign57490_e89583_d_n9;
        locals.var_t0_dn10 = assign57490_e89583_d_n10;
        locals.var_t0_dn11 = assign57490_e89583_d_n11;
        locals.var_t0_dn14 = assign57490_e89583_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign57500_e89598, assign57500_e89598_d_n0, assign57500_e89598_d_n2, assign57500_e89598_d_n4, assign57500_e89598_d_n5, assign57500_e89598_d_n6, assign57500_e89598_d_n7, assign57500_e89598_d_n8, assign57500_e89598_d_n9, assign57500_e89598_d_n10, assign57500_e89598_d_n11, assign57500_e89598_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1425 != 0.0)) {
        let assign57500_e89596: f64 = (locals.var_t2 + 1e-25);
        (assign57500_e89596, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign57500_e89598;
        locals.var_t2_dn0 = assign57500_e89598_d_n0;
        locals.var_t2_dn2 = assign57500_e89598_d_n2;
        locals.var_t2_dn4 = assign57500_e89598_d_n4;
        locals.var_t2_dn5 = assign57500_e89598_d_n5;
        locals.var_t2_dn6 = assign57500_e89598_d_n6;
        locals.var_t2_dn7 = assign57500_e89598_d_n7;
        locals.var_t2_dn8 = assign57500_e89598_d_n8;
        locals.var_t2_dn9 = assign57500_e89598_d_n9;
        locals.var_t2_dn10 = assign57500_e89598_d_n10;
        locals.var_t2_dn11 = assign57500_e89598_d_n11;
        locals.var_t2_dn14 = assign57500_e89598_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign57510_e89613, assign57510_e89613_d_n0, assign57510_e89613_d_n2, assign57510_e89613_d_n4, assign57510_e89613_d_n5, assign57510_e89613_d_n6, assign57510_e89613_d_n7, assign57510_e89613_d_n8, assign57510_e89613_d_n9, assign57510_e89613_d_n10, assign57510_e89613_d_n11, assign57510_e89613_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1425 != 0.0)) {
        let assign57510_e89611: f64 = (locals.var_beta * locals.var_ptl0);
        (assign57510_e89611, (locals.var_beta_dn0 * locals.var_ptl0), (locals.var_beta_dn2 * locals.var_ptl0), (locals.var_beta_dn4 * locals.var_ptl0), (locals.var_beta_dn5 * locals.var_ptl0), (locals.var_beta_dn6 * locals.var_ptl0), (locals.var_beta_dn7 * locals.var_ptl0), (locals.var_beta_dn8 * locals.var_ptl0), (locals.var_beta_dn9 * locals.var_ptl0), (locals.var_beta_dn10 * locals.var_ptl0), (locals.var_beta_dn11 * locals.var_ptl0), (locals.var_beta_dn14 * locals.var_ptl0),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign57510_e89613;
        locals.var_t0_dn0 = assign57510_e89613_d_n0;
        locals.var_t0_dn2 = assign57510_e89613_d_n2;
        locals.var_t0_dn4 = assign57510_e89613_d_n4;
        locals.var_t0_dn5 = assign57510_e89613_d_n5;
        locals.var_t0_dn6 = assign57510_e89613_d_n6;
        locals.var_t0_dn7 = assign57510_e89613_d_n7;
        locals.var_t0_dn8 = assign57510_e89613_d_n8;
        locals.var_t0_dn9 = assign57510_e89613_d_n9;
        locals.var_t0_dn10 = assign57510_e89613_d_n10;
        locals.var_t0_dn11 = assign57510_e89613_d_n11;
        locals.var_t0_dn14 = assign57510_e89613_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign57520_e89628, assign57520_e89628_d_n0, assign57520_e89628_d_n2, assign57520_e89628_d_n4, assign57520_e89628_d_n5, assign57520_e89628_d_n6, assign57520_e89628_d_n7, assign57520_e89628_d_n8, assign57520_e89628_d_n9, assign57520_e89628_d_n10, assign57520_e89628_d_n11, assign57520_e89628_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1425 != 0.0)) {
        let assign57520_e89626: f64 = (locals.var_cox * locals.var_t0);
        (assign57520_e89626, ((locals.var_cox_dn0 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn0)), ((locals.var_cox_dn2 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn2)), ((locals.var_cox_dn4 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn4)), ((locals.var_cox_dn5 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn5)), ((locals.var_cox_dn6 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn6)), ((locals.var_cox_dn7 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn7)), ((locals.var_cox_dn8 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn8)), ((locals.var_cox_dn9 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn9)), ((locals.var_cox_dn10 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn10)), ((locals.var_cox_dn11 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn11)), ((locals.var_cox_dn14 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign57520_e89628;
        locals.var_t3_dn0 = assign57520_e89628_d_n0;
        locals.var_t3_dn2 = assign57520_e89628_d_n2;
        locals.var_t3_dn4 = assign57520_e89628_d_n4;
        locals.var_t3_dn5 = assign57520_e89628_d_n5;
        locals.var_t3_dn6 = assign57520_e89628_d_n6;
        locals.var_t3_dn7 = assign57520_e89628_d_n7;
        locals.var_t3_dn8 = assign57520_e89628_d_n8;
        locals.var_t3_dn9 = assign57520_e89628_d_n9;
        locals.var_t3_dn10 = assign57520_e89628_d_n10;
        locals.var_t3_dn11 = assign57520_e89628_d_n11;
        locals.var_t3_dn14 = assign57520_e89628_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign57530_e89643, assign57530_e89643_d_n0, assign57530_e89643_d_n2, assign57530_e89643_d_n4, assign57530_e89643_d_n5, assign57530_e89643_d_n6, assign57530_e89643_d_n7, assign57530_e89643_d_n8, assign57530_e89643_d_n9, assign57530_e89643_d_n10, assign57530_e89643_d_n11, assign57530_e89643_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1425 != 0.0)) {
        let assign57530_e89641: f64 = (locals.var_t2).powf(p.p284);
        (assign57530_e89641, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn0)) } } else { (assign57530_e89641 * (p.p284 * (locals.var_t2_dn0 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn2)) } } else { (assign57530_e89641 * (p.p284 * (locals.var_t2_dn2 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn4)) } } else { (assign57530_e89641 * (p.p284 * (locals.var_t2_dn4 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn5)) } } else { (assign57530_e89641 * (p.p284 * (locals.var_t2_dn5 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn6)) } } else { (assign57530_e89641 * (p.p284 * (locals.var_t2_dn6 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn7)) } } else { (assign57530_e89641 * (p.p284 * (locals.var_t2_dn7 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn8)) } } else { (assign57530_e89641 * (p.p284 * (locals.var_t2_dn8 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn9)) } } else { (assign57530_e89641 * (p.p284 * (locals.var_t2_dn9 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn10)) } } else { (assign57530_e89641 * (p.p284 * (locals.var_t2_dn10 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn11)) } } else { (assign57530_e89641 * (p.p284 * (locals.var_t2_dn11 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn14)) } } else { (assign57530_e89641 * (p.p284 * (locals.var_t2_dn14 / locals.var_t2))) },)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign57530_e89643;
        locals.var_t0_dn0 = assign57530_e89643_d_n0;
        locals.var_t0_dn2 = assign57530_e89643_d_n2;
        locals.var_t0_dn4 = assign57530_e89643_d_n4;
        locals.var_t0_dn5 = assign57530_e89643_d_n5;
        locals.var_t0_dn6 = assign57530_e89643_d_n6;
        locals.var_t0_dn7 = assign57530_e89643_d_n7;
        locals.var_t0_dn8 = assign57530_e89643_d_n8;
        locals.var_t0_dn9 = assign57530_e89643_d_n9;
        locals.var_t0_dn10 = assign57530_e89643_d_n10;
        locals.var_t0_dn11 = assign57530_e89643_d_n11;
        locals.var_t0_dn14 = assign57530_e89643_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign57540_e89658, assign57540_e89658_d_n0, assign57540_e89658_d_n2, assign57540_e89658_d_n4, assign57540_e89658_d_n5, assign57540_e89658_d_n6, assign57540_e89658_d_n7, assign57540_e89658_d_n8, assign57540_e89658_d_n9, assign57540_e89658_d_n10, assign57540_e89658_d_n11, assign57540_e89658_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1425 != 0.0)) {
        let assign57540_e89656: f64 = (locals.var_t3 * locals.var_t0);
        (assign57540_e89656, ((locals.var_t3_dn0 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn0)), ((locals.var_t3_dn2 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn2)), ((locals.var_t3_dn4 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn4)), ((locals.var_t3_dn5 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn5)), ((locals.var_t3_dn6 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn6)), ((locals.var_t3_dn7 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn7)), ((locals.var_t3_dn8 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn8)), ((locals.var_t3_dn9 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn9)), ((locals.var_t3_dn10 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn10)), ((locals.var_t3_dn11 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn11)), ((locals.var_t3_dn14 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign57540_e89658;
        locals.var_t9_dn0 = assign57540_e89658_d_n0;
        locals.var_t9_dn2 = assign57540_e89658_d_n2;
        locals.var_t9_dn4 = assign57540_e89658_d_n4;
        locals.var_t9_dn5 = assign57540_e89658_d_n5;
        locals.var_t9_dn6 = assign57540_e89658_d_n6;
        locals.var_t9_dn7 = assign57540_e89658_d_n7;
        locals.var_t9_dn8 = assign57540_e89658_d_n8;
        locals.var_t9_dn9 = assign57540_e89658_d_n9;
        locals.var_t9_dn10 = assign57540_e89658_d_n10;
        locals.var_t9_dn11 = assign57540_e89658_d_n11;
        locals.var_t9_dn14 = assign57540_e89658_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign57550_e89675, assign57550_e89675_d_n0, assign57550_e89675_d_n2, assign57550_e89675_d_n4, assign57550_e89675_d_n5, assign57550_e89675_d_n6, assign57550_e89675_d_n7, assign57550_e89675_d_n8, assign57550_e89675_d_n9, assign57550_e89675_d_n10, assign57550_e89675_d_n11, assign57550_e89675_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1425 != 0.0)) {
        let assign57550_e89672: f64 = (locals.var_vdsz__blk441 * p.p285);
        let assign57550_e89673: f64 = (1.0 + assign57550_e89672);
        (assign57550_e89673, (locals.var_vdsz__blk441_dn0 * p.p285), (locals.var_vdsz__blk441_dn2 * p.p285), (locals.var_vdsz__blk441_dn4 * p.p285), (locals.var_vdsz__blk441_dn5 * p.p285), (locals.var_vdsz__blk441_dn6 * p.p285), (locals.var_vdsz__blk441_dn7 * p.p285), (locals.var_vdsz__blk441_dn8 * p.p285), (locals.var_vdsz__blk441_dn9 * p.p285), (locals.var_vdsz__blk441_dn10 * p.p285), (locals.var_vdsz__blk441_dn11 * p.p285), (locals.var_vdsz__blk441_dn14 * p.p285),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign57550_e89675;
        locals.var_t4_dn0 = assign57550_e89675_d_n0;
        locals.var_t4_dn2 = assign57550_e89675_d_n2;
        locals.var_t4_dn4 = assign57550_e89675_d_n4;
        locals.var_t4_dn5 = assign57550_e89675_d_n5;
        locals.var_t4_dn6 = assign57550_e89675_d_n6;
        locals.var_t4_dn7 = assign57550_e89675_d_n7;
        locals.var_t4_dn8 = assign57550_e89675_d_n8;
        locals.var_t4_dn9 = assign57550_e89675_d_n9;
        locals.var_t4_dn10 = assign57550_e89675_d_n10;
        locals.var_t4_dn11 = assign57550_e89675_d_n11;
        locals.var_t4_dn14 = assign57550_e89675_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign57560_e89688, assign57560_e89688_d_n0, assign57560_e89688_d_n2, assign57560_e89688_d_n4, assign57560_e89688_d_n5, assign57560_e89688_d_n6, assign57560_e89688_d_n7, assign57560_e89688_d_n8, assign57560_e89688_d_n9, assign57560_e89688_d_n10, assign57560_e89688_d_n11, assign57560_e89688_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1425 != 0.0)) {
        (locals.var_pt40, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign57560_e89688;
        locals.var_t0_dn0 = assign57560_e89688_d_n0;
        locals.var_t0_dn2 = assign57560_e89688_d_n2;
        locals.var_t0_dn4 = assign57560_e89688_d_n4;
        locals.var_t0_dn5 = assign57560_e89688_d_n5;
        locals.var_t0_dn6 = assign57560_e89688_d_n6;
        locals.var_t0_dn7 = assign57560_e89688_d_n7;
        locals.var_t0_dn8 = assign57560_e89688_d_n8;
        locals.var_t0_dn9 = assign57560_e89688_d_n9;
        locals.var_t0_dn10 = assign57560_e89688_d_n10;
        locals.var_t0_dn11 = assign57560_e89688_d_n11;
        locals.var_t0_dn14 = assign57560_e89688_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign57570_e89705, assign57570_e89705_d_n0, assign57570_e89705_d_n2, assign57570_e89705_d_n4, assign57570_e89705_d_n5, assign57570_e89705_d_n6, assign57570_e89705_d_n7, assign57570_e89705_d_n8, assign57570_e89705_d_n9, assign57570_e89705_d_n10, assign57570_e89705_d_n11, assign57570_e89705_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1425 != 0.0)) {
        let assign57570_e89701: f64 = (locals.var_phi_s0_dep__blk1091 + locals.var_t6);
        let assign57570_e89703: f64 = (assign57570_e89701 - locals.var_vbsz__blk440);
        (assign57570_e89703, ((locals.var_phi_s0_dep__blk1091_dn0 + locals.var_t6_dn0) - locals.var_vbsz__blk440_dn0), ((locals.var_phi_s0_dep__blk1091_dn2 + locals.var_t6_dn2) - locals.var_vbsz__blk440_dn2), ((locals.var_phi_s0_dep__blk1091_dn4 + locals.var_t6_dn4) - locals.var_vbsz__blk440_dn4), ((locals.var_phi_s0_dep__blk1091_dn5 + locals.var_t6_dn5) - locals.var_vbsz__blk440_dn5), ((locals.var_phi_s0_dep__blk1091_dn6 + locals.var_t6_dn6) - locals.var_vbsz__blk440_dn6), ((locals.var_phi_s0_dep__blk1091_dn7 + locals.var_t6_dn7) - locals.var_vbsz__blk440_dn7), ((locals.var_phi_s0_dep__blk1091_dn8 + locals.var_t6_dn8) - locals.var_vbsz__blk440_dn8), ((locals.var_phi_s0_dep__blk1091_dn9 + locals.var_t6_dn9) - locals.var_vbsz__blk440_dn9), ((locals.var_phi_s0_dep__blk1091_dn10 + locals.var_t6_dn10) - locals.var_vbsz__blk440_dn10), ((locals.var_phi_s0_dep__blk1091_dn11 + locals.var_t6_dn11) - locals.var_vbsz__blk440_dn11), ((locals.var_phi_s0_dep__blk1091_dn14 + locals.var_t6_dn14) - locals.var_vbsz__blk440_dn14),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign57570_e89705;
        locals.var_t5_dn0 = assign57570_e89705_d_n0;
        locals.var_t5_dn2 = assign57570_e89705_d_n2;
        locals.var_t5_dn4 = assign57570_e89705_d_n4;
        locals.var_t5_dn5 = assign57570_e89705_d_n5;
        locals.var_t5_dn6 = assign57570_e89705_d_n6;
        locals.var_t5_dn7 = assign57570_e89705_d_n7;
        locals.var_t5_dn8 = assign57570_e89705_d_n8;
        locals.var_t5_dn9 = assign57570_e89705_d_n9;
        locals.var_t5_dn10 = assign57570_e89705_d_n10;
        locals.var_t5_dn11 = assign57570_e89705_d_n11;
        locals.var_t5_dn14 = assign57570_e89705_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign57580_e89724, assign57580_e89724_d_n0, assign57580_e89724_d_n2, assign57580_e89724_d_n4, assign57580_e89724_d_n5, assign57580_e89724_d_n6, assign57580_e89724_d_n7, assign57580_e89724_d_n8, assign57580_e89724_d_n9, assign57580_e89724_d_n10, assign57580_e89724_d_n11, assign57580_e89724_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1425 != 0.0)) {
        let assign57580_e89719: f64 = (locals.var_vdsz__blk441 * locals.var_t0);
        let assign57580_e89721: f64 = (assign57580_e89719 * locals.var_t5);
        let assign57580_e89722: f64 = (locals.var_t4 + assign57580_e89721);
        (assign57580_e89722, (locals.var_t4_dn0 + ((((locals.var_vdsz__blk441_dn0 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn0)) * locals.var_t5) + (assign57580_e89719 * locals.var_t5_dn0))), (locals.var_t4_dn2 + ((((locals.var_vdsz__blk441_dn2 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn2)) * locals.var_t5) + (assign57580_e89719 * locals.var_t5_dn2))), (locals.var_t4_dn4 + ((((locals.var_vdsz__blk441_dn4 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn4)) * locals.var_t5) + (assign57580_e89719 * locals.var_t5_dn4))), (locals.var_t4_dn5 + ((((locals.var_vdsz__blk441_dn5 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn5)) * locals.var_t5) + (assign57580_e89719 * locals.var_t5_dn5))), (locals.var_t4_dn6 + ((((locals.var_vdsz__blk441_dn6 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn6)) * locals.var_t5) + (assign57580_e89719 * locals.var_t5_dn6))), (locals.var_t4_dn7 + ((((locals.var_vdsz__blk441_dn7 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn7)) * locals.var_t5) + (assign57580_e89719 * locals.var_t5_dn7))), (locals.var_t4_dn8 + ((((locals.var_vdsz__blk441_dn8 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn8)) * locals.var_t5) + (assign57580_e89719 * locals.var_t5_dn8))), (locals.var_t4_dn9 + ((((locals.var_vdsz__blk441_dn9 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn9)) * locals.var_t5) + (assign57580_e89719 * locals.var_t5_dn9))), (locals.var_t4_dn10 + ((((locals.var_vdsz__blk441_dn10 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn10)) * locals.var_t5) + (assign57580_e89719 * locals.var_t5_dn10))), (locals.var_t4_dn11 + ((((locals.var_vdsz__blk441_dn11 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn11)) * locals.var_t5) + (assign57580_e89719 * locals.var_t5_dn11))), (locals.var_t4_dn14 + ((((locals.var_vdsz__blk441_dn14 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn14)) * locals.var_t5) + (assign57580_e89719 * locals.var_t5_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign57580_e89724;
        locals.var_t4_dn0 = assign57580_e89724_d_n0;
        locals.var_t4_dn2 = assign57580_e89724_d_n2;
        locals.var_t4_dn4 = assign57580_e89724_d_n4;
        locals.var_t4_dn5 = assign57580_e89724_d_n5;
        locals.var_t4_dn6 = assign57580_e89724_d_n6;
        locals.var_t4_dn7 = assign57580_e89724_d_n7;
        locals.var_t4_dn8 = assign57580_e89724_d_n8;
        locals.var_t4_dn9 = assign57580_e89724_d_n9;
        locals.var_t4_dn10 = assign57580_e89724_d_n10;
        locals.var_t4_dn11 = assign57580_e89724_d_n11;
        locals.var_t4_dn14 = assign57580_e89724_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign57590_e89739, assign57590_e89739_d_n0, assign57590_e89739_d_n2, assign57590_e89739_d_n4, assign57590_e89739_d_n5, assign57590_e89739_d_n6, assign57590_e89739_d_n7, assign57590_e89739_d_n8, assign57590_e89739_d_n9, assign57590_e89739_d_n10, assign57590_e89739_d_n11, assign57590_e89739_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1425 != 0.0)) {
        let assign57590_e89737: f64 = (locals.var_t9 * locals.var_t4);
        (assign57590_e89737, ((locals.var_t9_dn0 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn0)), ((locals.var_t9_dn2 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn2)), ((locals.var_t9_dn4 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn4)), ((locals.var_t9_dn5 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn5)), ((locals.var_t9_dn6 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn6)), ((locals.var_t9_dn7 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn7)), ((locals.var_t9_dn8 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn8)), ((locals.var_t9_dn9 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn9)), ((locals.var_t9_dn10 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn10)), ((locals.var_t9_dn11 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn11)), ((locals.var_t9_dn14 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign57590_e89739;
        locals.var_t6_dn0 = assign57590_e89739_d_n0;
        locals.var_t6_dn2 = assign57590_e89739_d_n2;
        locals.var_t6_dn4 = assign57590_e89739_d_n4;
        locals.var_t6_dn5 = assign57590_e89739_d_n5;
        locals.var_t6_dn6 = assign57590_e89739_d_n6;
        locals.var_t6_dn7 = assign57590_e89739_d_n7;
        locals.var_t6_dn8 = assign57590_e89739_d_n8;
        locals.var_t6_dn9 = assign57590_e89739_d_n9;
        locals.var_t6_dn10 = assign57590_e89739_d_n10;
        locals.var_t6_dn11 = assign57590_e89739_d_n11;
        locals.var_t6_dn14 = assign57590_e89739_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign57600_e89752, assign57600_e89752_d_n0, assign57600_e89752_d_n2, assign57600_e89752_d_n4, assign57600_e89752_d_n5, assign57600_e89752_d_n6, assign57600_e89752_d_n7, assign57600_e89752_d_n8, assign57600_e89752_d_n9, assign57600_e89752_d_n10, assign57600_e89752_d_n11, assign57600_e89752_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1425 != 0.0)) {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign57600_e89752;
        locals.var_t9_dn0 = assign57600_e89752_d_n0;
        locals.var_t9_dn2 = assign57600_e89752_d_n2;
        locals.var_t9_dn4 = assign57600_e89752_d_n4;
        locals.var_t9_dn5 = assign57600_e89752_d_n5;
        locals.var_t9_dn6 = assign57600_e89752_d_n6;
        locals.var_t9_dn7 = assign57600_e89752_d_n7;
        locals.var_t9_dn8 = assign57600_e89752_d_n8;
        locals.var_t9_dn9 = assign57600_e89752_d_n9;
        locals.var_t9_dn10 = assign57600_e89752_d_n10;
        locals.var_t9_dn11 = assign57600_e89752_d_n11;
        locals.var_t9_dn14 = assign57600_e89752_d_n14;
        locals.var_t9_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_211(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign57610_e89766, assign57610_e89766_d_n0, assign57610_e89766_d_n2, assign57610_e89766_d_n4, assign57610_e89766_d_n5, assign57610_e89766_d_n6, assign57610_e89766_d_n7, assign57610_e89766_d_n8, assign57610_e89766_d_n9, assign57610_e89766_d_n10, assign57610_e89766_d_n11, assign57610_e89766_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1425 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign57610_e89766;
        locals.var_t9_dn0 = assign57610_e89766_d_n0;
        locals.var_t9_dn2 = assign57610_e89766_d_n2;
        locals.var_t9_dn4 = assign57610_e89766_d_n4;
        locals.var_t9_dn5 = assign57610_e89766_d_n5;
        locals.var_t9_dn6 = assign57610_e89766_d_n6;
        locals.var_t9_dn7 = assign57610_e89766_d_n7;
        locals.var_t9_dn8 = assign57610_e89766_d_n8;
        locals.var_t9_dn9 = assign57610_e89766_d_n9;
        locals.var_t9_dn10 = assign57610_e89766_d_n10;
        locals.var_t9_dn11 = assign57610_e89766_d_n11;
        locals.var_t9_dn14 = assign57610_e89766_d_n14;
        locals.var_t9_rv = 0.0;

        let assign57620_e89769: f64 = if p.p287 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1427 = assign57620_e89769;
        locals.var_guard1427_rv = 0.0;

        let (assign57630_e89784, assign57630_e89784_d_n0, assign57630_e89784_d_n2, assign57630_e89784_d_n4, assign57630_e89784_d_n5, assign57630_e89784_d_n6, assign57630_e89784_d_n7, assign57630_e89784_d_n8, assign57630_e89784_d_n9, assign57630_e89784_d_n10, assign57630_e89784_d_n11, assign57630_e89784_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1427 != 0.0)) {
        let assign57630_e89782: f64 = (locals.var_beta * locals.var_gdl0);
        (assign57630_e89782, (locals.var_beta_dn0 * locals.var_gdl0), (locals.var_beta_dn2 * locals.var_gdl0), (locals.var_beta_dn4 * locals.var_gdl0), (locals.var_beta_dn5 * locals.var_gdl0), (locals.var_beta_dn6 * locals.var_gdl0), (locals.var_beta_dn7 * locals.var_gdl0), (locals.var_beta_dn8 * locals.var_gdl0), (locals.var_beta_dn9 * locals.var_gdl0), (locals.var_beta_dn10 * locals.var_gdl0), (locals.var_beta_dn11 * locals.var_gdl0), (locals.var_beta_dn14 * locals.var_gdl0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign57630_e89784;
        locals.var_t1_dn0 = assign57630_e89784_d_n0;
        locals.var_t1_dn2 = assign57630_e89784_d_n2;
        locals.var_t1_dn4 = assign57630_e89784_d_n4;
        locals.var_t1_dn5 = assign57630_e89784_d_n5;
        locals.var_t1_dn6 = assign57630_e89784_d_n6;
        locals.var_t1_dn7 = assign57630_e89784_d_n7;
        locals.var_t1_dn8 = assign57630_e89784_d_n8;
        locals.var_t1_dn9 = assign57630_e89784_d_n9;
        locals.var_t1_dn10 = assign57630_e89784_d_n10;
        locals.var_t1_dn11 = assign57630_e89784_d_n11;
        locals.var_t1_dn14 = assign57630_e89784_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign57640_e89799, assign57640_e89799_d_n0, assign57640_e89799_d_n2, assign57640_e89799_d_n4, assign57640_e89799_d_n5, assign57640_e89799_d_n6, assign57640_e89799_d_n7, assign57640_e89799_d_n8, assign57640_e89799_d_n9, assign57640_e89799_d_n10, assign57640_e89799_d_n11, assign57640_e89799_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1427 != 0.0)) {
        let assign57640_e89797: f64 = (locals.var_cox * locals.var_t1);
        (assign57640_e89797, ((locals.var_cox_dn0 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn0)), ((locals.var_cox_dn2 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn2)), ((locals.var_cox_dn4 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn4)), ((locals.var_cox_dn5 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn5)), ((locals.var_cox_dn6 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn6)), ((locals.var_cox_dn7 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn7)), ((locals.var_cox_dn8 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn8)), ((locals.var_cox_dn9 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn9)), ((locals.var_cox_dn10 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn10)), ((locals.var_cox_dn11 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn11)), ((locals.var_cox_dn14 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign57640_e89799;
        locals.var_t2_dn0 = assign57640_e89799_d_n0;
        locals.var_t2_dn2 = assign57640_e89799_d_n2;
        locals.var_t2_dn4 = assign57640_e89799_d_n4;
        locals.var_t2_dn5 = assign57640_e89799_d_n5;
        locals.var_t2_dn6 = assign57640_e89799_d_n6;
        locals.var_t2_dn7 = assign57640_e89799_d_n7;
        locals.var_t2_dn8 = assign57640_e89799_d_n8;
        locals.var_t2_dn9 = assign57640_e89799_d_n9;
        locals.var_t2_dn10 = assign57640_e89799_d_n10;
        locals.var_t2_dn11 = assign57640_e89799_d_n11;
        locals.var_t2_dn14 = assign57640_e89799_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign57650_e89814, assign57650_e89814_d_n0, assign57650_e89814_d_n2, assign57650_e89814_d_n4, assign57650_e89814_d_n5, assign57650_e89814_d_n6, assign57650_e89814_d_n7, assign57650_e89814_d_n8, assign57650_e89814_d_n9, assign57650_e89814_d_n10, assign57650_e89814_d_n11, assign57650_e89814_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1427 != 0.0)) {
        let assign57650_e89812: f64 = (locals.var_t2 * locals.var_vdsz__blk441);
        (assign57650_e89812, ((locals.var_t2_dn0 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn0)), ((locals.var_t2_dn2 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn2)), ((locals.var_t2_dn4 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn4)), ((locals.var_t2_dn5 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn5)), ((locals.var_t2_dn6 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn6)), ((locals.var_t2_dn7 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn7)), ((locals.var_t2_dn8 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn8)), ((locals.var_t2_dn9 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn9)), ((locals.var_t2_dn10 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn10)), ((locals.var_t2_dn11 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn11)), ((locals.var_t2_dn14 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn14)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign57650_e89814;
        locals.var_t8_dn0 = assign57650_e89814_d_n0;
        locals.var_t8_dn2 = assign57650_e89814_d_n2;
        locals.var_t8_dn4 = assign57650_e89814_d_n4;
        locals.var_t8_dn5 = assign57650_e89814_d_n5;
        locals.var_t8_dn6 = assign57650_e89814_d_n6;
        locals.var_t8_dn7 = assign57650_e89814_d_n7;
        locals.var_t8_dn8 = assign57650_e89814_d_n8;
        locals.var_t8_dn9 = assign57650_e89814_d_n9;
        locals.var_t8_dn10 = assign57650_e89814_d_n10;
        locals.var_t8_dn11 = assign57650_e89814_d_n11;
        locals.var_t8_dn14 = assign57650_e89814_d_n14;
        locals.var_t8_rv = 0.0;

        let (assign57660_e89828, assign57660_e89828_d_n0, assign57660_e89828_d_n2, assign57660_e89828_d_n4, assign57660_e89828_d_n5, assign57660_e89828_d_n6, assign57660_e89828_d_n7, assign57660_e89828_d_n8, assign57660_e89828_d_n9, assign57660_e89828_d_n10, assign57660_e89828_d_n11, assign57660_e89828_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1427 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign57660_e89828;
        locals.var_t8_dn0 = assign57660_e89828_d_n0;
        locals.var_t8_dn2 = assign57660_e89828_d_n2;
        locals.var_t8_dn4 = assign57660_e89828_d_n4;
        locals.var_t8_dn5 = assign57660_e89828_d_n5;
        locals.var_t8_dn6 = assign57660_e89828_d_n6;
        locals.var_t8_dn7 = assign57660_e89828_d_n7;
        locals.var_t8_dn8 = assign57660_e89828_d_n8;
        locals.var_t8_dn9 = assign57660_e89828_d_n9;
        locals.var_t8_dn10 = assign57660_e89828_d_n10;
        locals.var_t8_dn11 = assign57660_e89828_d_n11;
        locals.var_t8_dn14 = assign57660_e89828_d_n14;
        locals.var_t8_rv = 0.0;

        let assign57670_e89831: f64 = (locals.var_t9 + locals.var_t8);
        let assign57670_e89833: f64 = if assign57670_e89831 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1428 = assign57670_e89833;
        locals.var_guard1428_rv = 0.0;

        let (assign57680_e89850, assign57680_e89850_d_n0, assign57680_e89850_d_n2, assign57680_e89850_d_n4, assign57680_e89850_d_n5, assign57680_e89850_d_n6, assign57680_e89850_d_n7, assign57680_e89850_d_n8, assign57680_e89850_d_n9, assign57680_e89850_d_n10, assign57680_e89850_d_n11, assign57680_e89850_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1428 != 0.0)) {
        let assign57680_e89847: f64 = (locals.var_t9 + locals.var_t8);
        let assign57680_e89848: f64 = (locals.var_pds * assign57680_e89847);
        (assign57680_e89848, ((locals.var_pds_dn0 * assign57680_e89847) + (locals.var_pds * (locals.var_t9_dn0 + locals.var_t8_dn0))), ((locals.var_pds_dn2 * assign57680_e89847) + (locals.var_pds * (locals.var_t9_dn2 + locals.var_t8_dn2))), ((locals.var_pds_dn4 * assign57680_e89847) + (locals.var_pds * (locals.var_t9_dn4 + locals.var_t8_dn4))), ((locals.var_pds_dn5 * assign57680_e89847) + (locals.var_pds * (locals.var_t9_dn5 + locals.var_t8_dn5))), ((locals.var_pds_dn6 * assign57680_e89847) + (locals.var_pds * (locals.var_t9_dn6 + locals.var_t8_dn6))), ((locals.var_pds_dn7 * assign57680_e89847) + (locals.var_pds * (locals.var_t9_dn7 + locals.var_t8_dn7))), ((locals.var_pds_dn8 * assign57680_e89847) + (locals.var_pds * (locals.var_t9_dn8 + locals.var_t8_dn8))), ((locals.var_pds_dn9 * assign57680_e89847) + (locals.var_pds * (locals.var_t9_dn9 + locals.var_t8_dn9))), ((locals.var_pds_dn10 * assign57680_e89847) + (locals.var_pds * (locals.var_t9_dn10 + locals.var_t8_dn10))), ((locals.var_pds_dn11 * assign57680_e89847) + (locals.var_pds * (locals.var_t9_dn11 + locals.var_t8_dn11))), ((locals.var_pds_dn14 * assign57680_e89847) + (locals.var_pds * (locals.var_t9_dn14 + locals.var_t8_dn14))),)
    } else {
        (locals.var_idd1, locals.var_idd1_dn0, locals.var_idd1_dn2, locals.var_idd1_dn4, locals.var_idd1_dn5, locals.var_idd1_dn6, locals.var_idd1_dn7, locals.var_idd1_dn8, locals.var_idd1_dn9, locals.var_idd1_dn10, locals.var_idd1_dn11, locals.var_idd1_dn14,)
    }
};
        locals.var_idd1 = assign57680_e89850;
        locals.var_idd1_dn0 = assign57680_e89850_d_n0;
        locals.var_idd1_dn2 = assign57680_e89850_d_n2;
        locals.var_idd1_dn4 = assign57680_e89850_d_n4;
        locals.var_idd1_dn5 = assign57680_e89850_d_n5;
        locals.var_idd1_dn6 = assign57680_e89850_d_n6;
        locals.var_idd1_dn7 = assign57680_e89850_d_n7;
        locals.var_idd1_dn8 = assign57680_e89850_d_n8;
        locals.var_idd1_dn9 = assign57680_e89850_d_n9;
        locals.var_idd1_dn10 = assign57680_e89850_d_n10;
        locals.var_idd1_dn11 = assign57680_e89850_d_n11;
        locals.var_idd1_dn14 = assign57680_e89850_d_n14;
        locals.var_idd1_rv = 0.0;

        let (assign57690_e89869, assign57690_e89869_d_n0, assign57690_e89869_d_n2, assign57690_e89869_d_n4, assign57690_e89869_d_n5, assign57690_e89869_d_n6, assign57690_e89869_d_n7, assign57690_e89869_d_n8, assign57690_e89869_d_n9, assign57690_e89869_d_n10, assign57690_e89869_d_n11, assign57690_e89869_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1428 != 0.0)) {
        let assign57690_e89864: f64 = (locals.var_betawl * locals.var_idd1);
        let assign57690_e89866: f64 = (assign57690_e89864 * locals.var_mu);
        let assign57690_e89867: f64 = (locals.var_ids0 + assign57690_e89866);
        (assign57690_e89867, (locals.var_ids0_dn0 + ((((locals.var_betawl_dn0 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn0)) * locals.var_mu) + (assign57690_e89864 * locals.var_mu_dn0))), (locals.var_ids0_dn2 + ((((locals.var_betawl_dn2 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn2)) * locals.var_mu) + (assign57690_e89864 * locals.var_mu_dn2))), (locals.var_ids0_dn4 + ((((locals.var_betawl_dn4 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn4)) * locals.var_mu) + (assign57690_e89864 * locals.var_mu_dn4))), (locals.var_ids0_dn5 + ((((locals.var_betawl_dn5 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn5)) * locals.var_mu) + (assign57690_e89864 * locals.var_mu_dn5))), (locals.var_ids0_dn6 + ((((locals.var_betawl_dn6 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn6)) * locals.var_mu) + (assign57690_e89864 * locals.var_mu_dn6))), (locals.var_ids0_dn7 + ((((locals.var_betawl_dn7 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn7)) * locals.var_mu) + (assign57690_e89864 * locals.var_mu_dn7))), (locals.var_ids0_dn8 + ((((locals.var_betawl_dn8 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn8)) * locals.var_mu) + (assign57690_e89864 * locals.var_mu_dn8))), (locals.var_ids0_dn9 + ((((locals.var_betawl_dn9 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn9)) * locals.var_mu) + (assign57690_e89864 * locals.var_mu_dn9))), (locals.var_ids0_dn10 + ((((locals.var_betawl_dn10 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn10)) * locals.var_mu) + (assign57690_e89864 * locals.var_mu_dn10))), (locals.var_ids0_dn11 + ((((locals.var_betawl_dn11 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn11)) * locals.var_mu) + (assign57690_e89864 * locals.var_mu_dn11))), (locals.var_ids0_dn14 + ((((locals.var_betawl_dn14 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn14)) * locals.var_mu) + (assign57690_e89864 * locals.var_mu_dn14))),)
    } else {
        (locals.var_ids0, locals.var_ids0_dn0, locals.var_ids0_dn2, locals.var_ids0_dn4, locals.var_ids0_dn5, locals.var_ids0_dn6, locals.var_ids0_dn7, locals.var_ids0_dn8, locals.var_ids0_dn9, locals.var_ids0_dn10, locals.var_ids0_dn11, locals.var_ids0_dn14,)
    }
};
        locals.var_ids0 = assign57690_e89869;
        locals.var_ids0_dn0 = assign57690_e89869_d_n0;
        locals.var_ids0_dn2 = assign57690_e89869_d_n2;
        locals.var_ids0_dn4 = assign57690_e89869_d_n4;
        locals.var_ids0_dn5 = assign57690_e89869_d_n5;
        locals.var_ids0_dn6 = assign57690_e89869_d_n6;
        locals.var_ids0_dn7 = assign57690_e89869_d_n7;
        locals.var_ids0_dn8 = assign57690_e89869_d_n8;
        locals.var_ids0_dn9 = assign57690_e89869_d_n9;
        locals.var_ids0_dn10 = assign57690_e89869_d_n10;
        locals.var_ids0_dn11 = assign57690_e89869_d_n11;
        locals.var_ids0_dn14 = assign57690_e89869_d_n14;
        locals.var_ids0_rv = 0.0;

        let (assign57700_e89880, assign57700_e89880_d_n0, assign57700_e89880_d_n2, assign57700_e89880_d_n4, assign57700_e89880_d_n5, assign57700_e89880_d_n6, assign57700_e89880_d_n7, assign57700_e89880_d_n8, assign57700_e89880_d_n9, assign57700_e89880_d_n10, assign57700_e89880_d_n11, assign57700_e89880_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        (locals.var_ids0, locals.var_ids0_dn0, locals.var_ids0_dn2, locals.var_ids0_dn4, locals.var_ids0_dn5, locals.var_ids0_dn6, locals.var_ids0_dn7, locals.var_ids0_dn8, locals.var_ids0_dn9, locals.var_ids0_dn10, locals.var_ids0_dn11, locals.var_ids0_dn14,)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn14,)
    }
};
        locals.var_ids = assign57700_e89880;
        locals.var_ids_dn0 = assign57700_e89880_d_n0;
        locals.var_ids_dn2 = assign57700_e89880_d_n2;
        locals.var_ids_dn4 = assign57700_e89880_d_n4;
        locals.var_ids_dn5 = assign57700_e89880_d_n5;
        locals.var_ids_dn6 = assign57700_e89880_d_n6;
        locals.var_ids_dn7 = assign57700_e89880_d_n7;
        locals.var_ids_dn8 = assign57700_e89880_d_n8;
        locals.var_ids_dn9 = assign57700_e89880_d_n9;
        locals.var_ids_dn10 = assign57700_e89880_d_n10;
        locals.var_ids_dn11 = assign57700_e89880_d_n11;
        locals.var_ids_dn14 = assign57700_e89880_d_n14;
        locals.var_ids_rv = 0.0;

        let (assign57710_e89900, assign57710_e89900_d_n0, assign57710_e89900_d_n2, assign57710_e89900_d_n4, assign57710_e89900_d_n5, assign57710_e89900_d_n6, assign57710_e89900_d_n7, assign57710_e89900_d_n8, assign57710_e89900_d_n9, assign57710_e89900_d_n10, assign57710_e89900_d_n11, assign57710_e89900_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign57710_e89890: f64 = (-0.5);
        let assign57710_e89893: f64 = (locals.var_q_s0__blk1100 - locals.var_q_n0__blk1124);
        let assign57710_e89895: f64 = (assign57710_e89893 + locals.var_q_sl__blk1101);
        let assign57710_e89897: f64 = (assign57710_e89895 - locals.var_q_nl__blk1125);
        let assign57710_e89898: f64 = (assign57710_e89890 * assign57710_e89897);
        (assign57710_e89898, (assign57710_e89890 * (((locals.var_q_s0__blk1100_dn0 - locals.var_q_n0__blk1124_dn0) + locals.var_q_sl__blk1101_dn0) - locals.var_q_nl__blk1125_dn0)), (assign57710_e89890 * (((locals.var_q_s0__blk1100_dn2 - locals.var_q_n0__blk1124_dn2) + locals.var_q_sl__blk1101_dn2) - locals.var_q_nl__blk1125_dn2)), (assign57710_e89890 * (((locals.var_q_s0__blk1100_dn4 - locals.var_q_n0__blk1124_dn4) + locals.var_q_sl__blk1101_dn4) - locals.var_q_nl__blk1125_dn4)), (assign57710_e89890 * (((locals.var_q_s0__blk1100_dn5 - locals.var_q_n0__blk1124_dn5) + locals.var_q_sl__blk1101_dn5) - locals.var_q_nl__blk1125_dn5)), (assign57710_e89890 * (((locals.var_q_s0__blk1100_dn6 - locals.var_q_n0__blk1124_dn6) + locals.var_q_sl__blk1101_dn6) - locals.var_q_nl__blk1125_dn6)), (assign57710_e89890 * (((locals.var_q_s0__blk1100_dn7 - locals.var_q_n0__blk1124_dn7) + locals.var_q_sl__blk1101_dn7) - locals.var_q_nl__blk1125_dn7)), (assign57710_e89890 * (((locals.var_q_s0__blk1100_dn8 - locals.var_q_n0__blk1124_dn8) + locals.var_q_sl__blk1101_dn8) - locals.var_q_nl__blk1125_dn8)), (assign57710_e89890 * (((locals.var_q_s0__blk1100_dn9 - locals.var_q_n0__blk1124_dn9) + locals.var_q_sl__blk1101_dn9) - locals.var_q_nl__blk1125_dn9)), (assign57710_e89890 * (((locals.var_q_s0__blk1100_dn10 - locals.var_q_n0__blk1124_dn10) + locals.var_q_sl__blk1101_dn10) - locals.var_q_nl__blk1125_dn10)), (assign57710_e89890 * (((locals.var_q_s0__blk1100_dn11 - locals.var_q_n0__blk1124_dn11) + locals.var_q_sl__blk1101_dn11) - locals.var_q_nl__blk1125_dn11)), (assign57710_e89890 * (((locals.var_q_s0__blk1100_dn14 - locals.var_q_n0__blk1124_dn14) + locals.var_q_sl__blk1101_dn14) - locals.var_q_nl__blk1125_dn14)),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn4, locals.var_qbu_dn5, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn8, locals.var_qbu_dn9, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn14,)
    }
};
        locals.var_qbu = assign57710_e89900;
        locals.var_qbu_dn0 = assign57710_e89900_d_n0;
        locals.var_qbu_dn2 = assign57710_e89900_d_n2;
        locals.var_qbu_dn4 = assign57710_e89900_d_n4;
        locals.var_qbu_dn5 = assign57710_e89900_d_n5;
        locals.var_qbu_dn6 = assign57710_e89900_d_n6;
        locals.var_qbu_dn7 = assign57710_e89900_d_n7;
        locals.var_qbu_dn8 = assign57710_e89900_d_n8;
        locals.var_qbu_dn9 = assign57710_e89900_d_n9;
        locals.var_qbu_dn10 = assign57710_e89900_d_n10;
        locals.var_qbu_dn11 = assign57710_e89900_d_n11;
        locals.var_qbu_dn14 = assign57710_e89900_d_n14;
        locals.var_qbu_rv = 0.0;

        let (assign57720_e89916, assign57720_e89916_d_n0, assign57720_e89916_d_n2, assign57720_e89916_d_n4, assign57720_e89916_d_n5, assign57720_e89916_d_n6, assign57720_e89916_d_n7, assign57720_e89916_d_n8, assign57720_e89916_d_n9, assign57720_e89916_d_n10, assign57720_e89916_d_n11, assign57720_e89916_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign57720_e89910: f64 = (-0.5);
        let assign57720_e89913: f64 = (locals.var_q_n0__blk1124 + locals.var_q_nl__blk1125);
        let assign57720_e89914: f64 = (assign57720_e89910 * assign57720_e89913);
        (assign57720_e89914, (assign57720_e89910 * (locals.var_q_n0__blk1124_dn0 + locals.var_q_nl__blk1125_dn0)), (assign57720_e89910 * (locals.var_q_n0__blk1124_dn2 + locals.var_q_nl__blk1125_dn2)), (assign57720_e89910 * (locals.var_q_n0__blk1124_dn4 + locals.var_q_nl__blk1125_dn4)), (assign57720_e89910 * (locals.var_q_n0__blk1124_dn5 + locals.var_q_nl__blk1125_dn5)), (assign57720_e89910 * (locals.var_q_n0__blk1124_dn6 + locals.var_q_nl__blk1125_dn6)), (assign57720_e89910 * (locals.var_q_n0__blk1124_dn7 + locals.var_q_nl__blk1125_dn7)), (assign57720_e89910 * (locals.var_q_n0__blk1124_dn8 + locals.var_q_nl__blk1125_dn8)), (assign57720_e89910 * (locals.var_q_n0__blk1124_dn9 + locals.var_q_nl__blk1125_dn9)), (assign57720_e89910 * (locals.var_q_n0__blk1124_dn10 + locals.var_q_nl__blk1125_dn10)), (assign57720_e89910 * (locals.var_q_n0__blk1124_dn11 + locals.var_q_nl__blk1125_dn11)), (assign57720_e89910 * (locals.var_q_n0__blk1124_dn14 + locals.var_q_nl__blk1125_dn14)),)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn4, locals.var_qiu_dn5, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn8, locals.var_qiu_dn9, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn14,)
    }
};
        locals.var_qiu = assign57720_e89916;
        locals.var_qiu_dn0 = assign57720_e89916_d_n0;
        locals.var_qiu_dn2 = assign57720_e89916_d_n2;
        locals.var_qiu_dn4 = assign57720_e89916_d_n4;
        locals.var_qiu_dn5 = assign57720_e89916_d_n5;
        locals.var_qiu_dn6 = assign57720_e89916_d_n6;
        locals.var_qiu_dn7 = assign57720_e89916_d_n7;
        locals.var_qiu_dn8 = assign57720_e89916_d_n8;
        locals.var_qiu_dn9 = assign57720_e89916_d_n9;
        locals.var_qiu_dn10 = assign57720_e89916_d_n10;
        locals.var_qiu_dn11 = assign57720_e89916_d_n11;
        locals.var_qiu_dn14 = assign57720_e89916_d_n14;
        locals.var_qiu_rv = 0.0;

        let (assign57730_e89927, assign57730_e89927_d_n0, assign57730_e89927_d_n2, assign57730_e89927_d_n4, assign57730_e89927_d_n5, assign57730_e89927_d_n6, assign57730_e89927_d_n7, assign57730_e89927_d_n8, assign57730_e89927_d_n9, assign57730_e89927_d_n10, assign57730_e89927_d_n11, assign57730_e89927_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        (0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn4, locals.var_qdrat_dn5, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn8, locals.var_qdrat_dn9, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn14,)
    }
};
        locals.var_qdrat = assign57730_e89927;
        locals.var_qdrat_dn0 = assign57730_e89927_d_n0;
        locals.var_qdrat_dn2 = assign57730_e89927_d_n2;
        locals.var_qdrat_dn4 = assign57730_e89927_d_n4;
        locals.var_qdrat_dn5 = assign57730_e89927_d_n5;
        locals.var_qdrat_dn6 = assign57730_e89927_d_n6;
        locals.var_qdrat_dn7 = assign57730_e89927_d_n7;
        locals.var_qdrat_dn8 = assign57730_e89927_d_n8;
        locals.var_qdrat_dn9 = assign57730_e89927_d_n9;
        locals.var_qdrat_dn10 = assign57730_e89927_d_n10;
        locals.var_qdrat_dn11 = assign57730_e89927_d_n11;
        locals.var_qdrat_dn14 = assign57730_e89927_d_n14;
        locals.var_qdrat_rv = 0.0;

        let (assign57740_e89943, assign57740_e89943_d_n0, assign57740_e89943_d_n2, assign57740_e89943_d_n4, assign57740_e89943_d_n5, assign57740_e89943_d_n6, assign57740_e89943_d_n7, assign57740_e89943_d_n8, assign57740_e89943_d_n9, assign57740_e89943_d_n10, assign57740_e89943_d_n11, assign57740_e89943_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign57740_e89937: f64 = (-0.5);
        let assign57740_e89940: f64 = (locals.var_q_n0__blk1124 + locals.var_q_nl__blk1125);
        let assign57740_e89941: f64 = (assign57740_e89937 * assign57740_e89940);
        (assign57740_e89941, (assign57740_e89937 * (locals.var_q_n0__blk1124_dn0 + locals.var_q_nl__blk1125_dn0)), (assign57740_e89937 * (locals.var_q_n0__blk1124_dn2 + locals.var_q_nl__blk1125_dn2)), (assign57740_e89937 * (locals.var_q_n0__blk1124_dn4 + locals.var_q_nl__blk1125_dn4)), (assign57740_e89937 * (locals.var_q_n0__blk1124_dn5 + locals.var_q_nl__blk1125_dn5)), (assign57740_e89937 * (locals.var_q_n0__blk1124_dn6 + locals.var_q_nl__blk1125_dn6)), (assign57740_e89937 * (locals.var_q_n0__blk1124_dn7 + locals.var_q_nl__blk1125_dn7)), (assign57740_e89937 * (locals.var_q_n0__blk1124_dn8 + locals.var_q_nl__blk1125_dn8)), (assign57740_e89937 * (locals.var_q_n0__blk1124_dn9 + locals.var_q_nl__blk1125_dn9)), (assign57740_e89937 * (locals.var_q_n0__blk1124_dn10 + locals.var_q_nl__blk1125_dn10)), (assign57740_e89937 * (locals.var_q_n0__blk1124_dn11 + locals.var_q_nl__blk1125_dn11)), (assign57740_e89937 * (locals.var_q_n0__blk1124_dn14 + locals.var_q_nl__blk1125_dn14)),)
    } else {
        (locals.var_qiu_noi, locals.var_qiu_noi_dn0, locals.var_qiu_noi_dn2, locals.var_qiu_noi_dn4, locals.var_qiu_noi_dn5, locals.var_qiu_noi_dn6, locals.var_qiu_noi_dn7, locals.var_qiu_noi_dn8, locals.var_qiu_noi_dn9, locals.var_qiu_noi_dn10, locals.var_qiu_noi_dn11, locals.var_qiu_noi_dn14,)
    }
};
        locals.var_qiu_noi = assign57740_e89943;
        locals.var_qiu_noi_dn0 = assign57740_e89943_d_n0;
        locals.var_qiu_noi_dn2 = assign57740_e89943_d_n2;
        locals.var_qiu_noi_dn4 = assign57740_e89943_d_n4;
        locals.var_qiu_noi_dn5 = assign57740_e89943_d_n5;
        locals.var_qiu_noi_dn6 = assign57740_e89943_d_n6;
        locals.var_qiu_noi_dn7 = assign57740_e89943_d_n7;
        locals.var_qiu_noi_dn8 = assign57740_e89943_d_n8;
        locals.var_qiu_noi_dn9 = assign57740_e89943_d_n9;
        locals.var_qiu_noi_dn10 = assign57740_e89943_d_n10;
        locals.var_qiu_noi_dn11 = assign57740_e89943_d_n11;
        locals.var_qiu_noi_dn14 = assign57740_e89943_d_n14;
        locals.var_qiu_noi_rv = 0.0;

        let (assign57750_e89955, assign57750_e89955_d_n0, assign57750_e89955_d_n2, assign57750_e89955_d_n4, assign57750_e89955_d_n5, assign57750_e89955_d_n6, assign57750_e89955_d_n7, assign57750_e89955_d_n8, assign57750_e89955_d_n9, assign57750_e89955_d_n10, assign57750_e89955_d_n11, assign57750_e89955_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        let assign57750_e89953: f64 = (-locals.var_q_n0__blk1124);
        (assign57750_e89953, (-locals.var_q_n0__blk1124_dn0), (-locals.var_q_n0__blk1124_dn2), (-locals.var_q_n0__blk1124_dn4), (-locals.var_q_n0__blk1124_dn5), (-locals.var_q_n0__blk1124_dn6), (-locals.var_q_n0__blk1124_dn7), (-locals.var_q_n0__blk1124_dn8), (-locals.var_q_n0__blk1124_dn9), (-locals.var_q_n0__blk1124_dn10), (-locals.var_q_n0__blk1124_dn11), (-locals.var_q_n0__blk1124_dn14),)
    } else {
        (locals.var_qn0, locals.var_qn0_dn0, locals.var_qn0_dn2, locals.var_qn0_dn4, locals.var_qn0_dn5, locals.var_qn0_dn6, locals.var_qn0_dn7, locals.var_qn0_dn8, locals.var_qn0_dn9, locals.var_qn0_dn10, locals.var_qn0_dn11, locals.var_qn0_dn14,)
    }
};
        locals.var_qn0 = assign57750_e89955;
        locals.var_qn0_dn0 = assign57750_e89955_d_n0;
        locals.var_qn0_dn2 = assign57750_e89955_d_n2;
        locals.var_qn0_dn4 = assign57750_e89955_d_n4;
        locals.var_qn0_dn5 = assign57750_e89955_d_n5;
        locals.var_qn0_dn6 = assign57750_e89955_d_n6;
        locals.var_qn0_dn7 = assign57750_e89955_d_n7;
        locals.var_qn0_dn8 = assign57750_e89955_d_n8;
        locals.var_qn0_dn9 = assign57750_e89955_d_n9;
        locals.var_qn0_dn10 = assign57750_e89955_d_n10;
        locals.var_qn0_dn11 = assign57750_e89955_d_n11;
        locals.var_qn0_dn14 = assign57750_e89955_d_n14;
        locals.var_qn0_rv = 0.0;

        let (assign57760_e89966, assign57760_e89966_d_n0, assign57760_e89966_d_n2, assign57760_e89966_d_n4, assign57760_e89966_d_n5, assign57760_e89966_d_n6, assign57760_e89966_d_n7, assign57760_e89966_d_n8, assign57760_e89966_d_n9, assign57760_e89966_d_n10, assign57760_e89966_d_n11, assign57760_e89966_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) {
        (locals.var_ey_acc__blk1118, locals.var_ey_acc__blk1118_dn0, locals.var_ey_acc__blk1118_dn2, locals.var_ey_acc__blk1118_dn4, locals.var_ey_acc__blk1118_dn5, locals.var_ey_acc__blk1118_dn6, locals.var_ey_acc__blk1118_dn7, locals.var_ey_acc__blk1118_dn8, locals.var_ey_acc__blk1118_dn9, locals.var_ey_acc__blk1118_dn10, locals.var_ey_acc__blk1118_dn11, locals.var_ey_acc__blk1118_dn14,)
    } else {
        (locals.var_ey, locals.var_ey_dn0, locals.var_ey_dn2, locals.var_ey_dn4, locals.var_ey_dn5, locals.var_ey_dn6, locals.var_ey_dn7, locals.var_ey_dn8, locals.var_ey_dn9, locals.var_ey_dn10, locals.var_ey_dn11, locals.var_ey_dn14,)
    }
};
        locals.var_ey = assign57760_e89966;
        locals.var_ey_dn0 = assign57760_e89966_d_n0;
        locals.var_ey_dn2 = assign57760_e89966_d_n2;
        locals.var_ey_dn4 = assign57760_e89966_d_n4;
        locals.var_ey_dn5 = assign57760_e89966_d_n5;
        locals.var_ey_dn6 = assign57760_e89966_d_n6;
        locals.var_ey_dn7 = assign57760_e89966_d_n7;
        locals.var_ey_dn8 = assign57760_e89966_d_n8;
        locals.var_ey_dn9 = assign57760_e89966_d_n9;
        locals.var_ey_dn10 = assign57760_e89966_d_n10;
        locals.var_ey_dn11 = assign57760_e89966_d_n11;
        locals.var_ey_dn14 = assign57760_e89966_d_n14;
        locals.var_ey_rv = 0.0;

        let assign57770_e89973: f64 = if ((locals.var_qn0 < 1e-25) || (locals.var_qiu < 1e-25)) { 1.0 } else { 0.0 };
        locals.var_guard1429 = assign57770_e89973;
        locals.var_guard1429_rv = 0.0;

        let (assign57780_e89986,) = {
    if (((locals.var_guard445 != 0.0) && ((locals.var_guard448 != 0.0) && (!((locals.var_guard446 != 0.0) || (locals.var_guard447 != 0.0))))) && (locals.var_guard1429 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_noqi,)
    }
};
        locals.var_flg_noqi = assign57780_e89986;
        locals.var_flg_noqi_rv = 0.0;

        let assign57790_e89989: f64 = if locals.var_vgs < locals.var_vgs_fb { 1.0 } else { 0.0 };
        locals.var_guard1430 = assign57790_e89989;
        locals.var_guard1430_rv = 0.0;

        let (assign57800_e89997,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign57800_e89995: f64 = (-1.0);
        (assign57800_e89995,)
    } else {
        (locals.var_flg_zone,)
    }
};
        locals.var_flg_zone = assign57800_e89997;
        locals.var_flg_zone_rv = 0.0;

        let (assign57810_e90012, assign57810_e90012_d_n0, assign57810_e90012_d_n2, assign57810_e90012_d_n4, assign57810_e90012_d_n5, assign57810_e90012_d_n6, assign57810_e90012_d_n7, assign57810_e90012_d_n8, assign57810_e90012_d_n9, assign57810_e90012_d_n10, assign57810_e90012_d_n11, assign57810_e90012_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign57810_e90004: f64 = (2.0 * locals.var_beta_inv);
        let assign57810_e90006: f64 = (-locals.var_vgs_min);
        let assign57810_e90008: f64 = (assign57810_e90006 / locals.var_fac1);
        let assign57810_e90009: f64 = (assign57810_e90008).ln();
        let assign57810_e90010: f64 = (assign57810_e90004 * assign57810_e90009);
        (assign57810_e90010, (((2.0 * locals.var_beta_inv_dn0) * assign57810_e90009) + (assign57810_e90004 * ((-((assign57810_e90006 * locals.var_fac1_dn0) / (locals.var_fac1 * locals.var_fac1))) / assign57810_e90008))), (((2.0 * locals.var_beta_inv_dn2) * assign57810_e90009) + (assign57810_e90004 * ((-((assign57810_e90006 * locals.var_fac1_dn2) / (locals.var_fac1 * locals.var_fac1))) / assign57810_e90008))), (((2.0 * locals.var_beta_inv_dn4) * assign57810_e90009) + (assign57810_e90004 * ((-((assign57810_e90006 * locals.var_fac1_dn4) / (locals.var_fac1 * locals.var_fac1))) / assign57810_e90008))), (((2.0 * locals.var_beta_inv_dn5) * assign57810_e90009) + (assign57810_e90004 * ((-((assign57810_e90006 * locals.var_fac1_dn5) / (locals.var_fac1 * locals.var_fac1))) / assign57810_e90008))), (((2.0 * locals.var_beta_inv_dn6) * assign57810_e90009) + (assign57810_e90004 * ((-((assign57810_e90006 * locals.var_fac1_dn6) / (locals.var_fac1 * locals.var_fac1))) / assign57810_e90008))), (((2.0 * locals.var_beta_inv_dn7) * assign57810_e90009) + (assign57810_e90004 * ((-((assign57810_e90006 * locals.var_fac1_dn7) / (locals.var_fac1 * locals.var_fac1))) / assign57810_e90008))), (((2.0 * locals.var_beta_inv_dn8) * assign57810_e90009) + (assign57810_e90004 * ((-((assign57810_e90006 * locals.var_fac1_dn8) / (locals.var_fac1 * locals.var_fac1))) / assign57810_e90008))), (((2.0 * locals.var_beta_inv_dn9) * assign57810_e90009) + (assign57810_e90004 * ((-((assign57810_e90006 * locals.var_fac1_dn9) / (locals.var_fac1 * locals.var_fac1))) / assign57810_e90008))), (((2.0 * locals.var_beta_inv_dn10) * assign57810_e90009) + (assign57810_e90004 * ((-((assign57810_e90006 * locals.var_fac1_dn10) / (locals.var_fac1 * locals.var_fac1))) / assign57810_e90008))), (((2.0 * locals.var_beta_inv_dn11) * assign57810_e90009) + (assign57810_e90004 * ((-((assign57810_e90006 * locals.var_fac1_dn11) / (locals.var_fac1 * locals.var_fac1))) / assign57810_e90008))), (((2.0 * locals.var_beta_inv_dn14) * assign57810_e90009) + (assign57810_e90004 * ((-((assign57810_e90006 * locals.var_fac1_dn14) / (locals.var_fac1 * locals.var_fac1))) / assign57810_e90008))),)
    } else {
        (locals.var_ps0_min, locals.var_ps0_min_dn0, locals.var_ps0_min_dn2, locals.var_ps0_min_dn4, locals.var_ps0_min_dn5, locals.var_ps0_min_dn6, locals.var_ps0_min_dn7, locals.var_ps0_min_dn8, locals.var_ps0_min_dn9, locals.var_ps0_min_dn10, locals.var_ps0_min_dn11, locals.var_ps0_min_dn14,)
    }
};
        locals.var_ps0_min = assign57810_e90012;
        locals.var_ps0_min_dn0 = assign57810_e90012_d_n0;
        locals.var_ps0_min_dn2 = assign57810_e90012_d_n2;
        locals.var_ps0_min_dn4 = assign57810_e90012_d_n4;
        locals.var_ps0_min_dn5 = assign57810_e90012_d_n5;
        locals.var_ps0_min_dn6 = assign57810_e90012_d_n6;
        locals.var_ps0_min_dn7 = assign57810_e90012_d_n7;
        locals.var_ps0_min_dn8 = assign57810_e90012_d_n8;
        locals.var_ps0_min_dn9 = assign57810_e90012_d_n9;
        locals.var_ps0_min_dn10 = assign57810_e90012_d_n10;
        locals.var_ps0_min_dn11 = assign57810_e90012_d_n11;
        locals.var_ps0_min_dn14 = assign57810_e90012_d_n14;
        locals.var_ps0_min_rv = 0.0;

        let (assign57820_e90023, assign57820_e90023_d_n0, assign57820_e90023_d_n2, assign57820_e90023_d_n4, assign57820_e90023_d_n5, assign57820_e90023_d_n6, assign57820_e90023_d_n7, assign57820_e90023_d_n8, assign57820_e90023_d_n9, assign57820_e90023_d_n10, assign57820_e90023_d_n11, assign57820_e90023_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign57820_e90020: f64 = (locals.var_vgp - locals.var_vbscl__blk437);
        let assign57820_e90021: f64 = (locals.var_beta * assign57820_e90020);
        (assign57820_e90021, ((locals.var_beta_dn0 * assign57820_e90020) + (locals.var_beta * (locals.var_vgp_dn0 - locals.var_vbscl__blk437_dn0))), ((locals.var_beta_dn2 * assign57820_e90020) + (locals.var_beta * (locals.var_vgp_dn2 - locals.var_vbscl__blk437_dn2))), ((locals.var_beta_dn4 * assign57820_e90020) + (locals.var_beta * (locals.var_vgp_dn4 - locals.var_vbscl__blk437_dn4))), ((locals.var_beta_dn5 * assign57820_e90020) + (locals.var_beta * (locals.var_vgp_dn5 - locals.var_vbscl__blk437_dn5))), ((locals.var_beta_dn6 * assign57820_e90020) + (locals.var_beta * (locals.var_vgp_dn6 - locals.var_vbscl__blk437_dn6))), ((locals.var_beta_dn7 * assign57820_e90020) + (locals.var_beta * (locals.var_vgp_dn7 - locals.var_vbscl__blk437_dn7))), ((locals.var_beta_dn8 * assign57820_e90020) + (locals.var_beta * (locals.var_vgp_dn8 - locals.var_vbscl__blk437_dn8))), ((locals.var_beta_dn9 * assign57820_e90020) + (locals.var_beta * (locals.var_vgp_dn9 - locals.var_vbscl__blk437_dn9))), ((locals.var_beta_dn10 * assign57820_e90020) + (locals.var_beta * (locals.var_vgp_dn10 - locals.var_vbscl__blk437_dn10))), ((locals.var_beta_dn11 * assign57820_e90020) + (locals.var_beta * (locals.var_vgp_dn11 - locals.var_vbscl__blk437_dn11))), ((locals.var_beta_dn14 * assign57820_e90020) + (locals.var_beta * (locals.var_vgp_dn14 - locals.var_vbscl__blk437_dn14))),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign57820_e90023;
        locals.var_tx_dn0 = assign57820_e90023_d_n0;
        locals.var_tx_dn2 = assign57820_e90023_d_n2;
        locals.var_tx_dn4 = assign57820_e90023_d_n4;
        locals.var_tx_dn5 = assign57820_e90023_d_n5;
        locals.var_tx_dn6 = assign57820_e90023_d_n6;
        locals.var_tx_dn7 = assign57820_e90023_d_n7;
        locals.var_tx_dn8 = assign57820_e90023_d_n8;
        locals.var_tx_dn9 = assign57820_e90023_d_n9;
        locals.var_tx_dn10 = assign57820_e90023_d_n10;
        locals.var_tx_dn11 = assign57820_e90023_d_n11;
        locals.var_tx_dn14 = assign57820_e90023_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign57830_e90034, assign57830_e90034_d_n0, assign57830_e90034_d_n2, assign57830_e90034_d_n4, assign57830_e90034_d_n5, assign57830_e90034_d_n6, assign57830_e90034_d_n7, assign57830_e90034_d_n8, assign57830_e90034_d_n9, assign57830_e90034_d_n10, assign57830_e90034_d_n11, assign57830_e90034_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign57830_e90031: f64 = (locals.var_beta * locals.var_cnst0);
        let assign57830_e90032: f64 = (1.0 / assign57830_e90031);
        (assign57830_e90032, (-(((locals.var_beta_dn0 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn0)) / (assign57830_e90031 * assign57830_e90031))), (-(((locals.var_beta_dn2 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn2)) / (assign57830_e90031 * assign57830_e90031))), (-(((locals.var_beta_dn4 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn4)) / (assign57830_e90031 * assign57830_e90031))), (-(((locals.var_beta_dn5 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn5)) / (assign57830_e90031 * assign57830_e90031))), (-(((locals.var_beta_dn6 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn6)) / (assign57830_e90031 * assign57830_e90031))), (-(((locals.var_beta_dn7 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn7)) / (assign57830_e90031 * assign57830_e90031))), (-(((locals.var_beta_dn8 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn8)) / (assign57830_e90031 * assign57830_e90031))), (-(((locals.var_beta_dn9 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn9)) / (assign57830_e90031 * assign57830_e90031))), (-(((locals.var_beta_dn10 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn10)) / (assign57830_e90031 * assign57830_e90031))), (-(((locals.var_beta_dn11 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn11)) / (assign57830_e90031 * assign57830_e90031))), (-(((locals.var_beta_dn14 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn14)) / (assign57830_e90031 * assign57830_e90031))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign57830_e90034;
        locals.var_t1_dn0 = assign57830_e90034_d_n0;
        locals.var_t1_dn2 = assign57830_e90034_d_n2;
        locals.var_t1_dn4 = assign57830_e90034_d_n4;
        locals.var_t1_dn5 = assign57830_e90034_d_n5;
        locals.var_t1_dn6 = assign57830_e90034_d_n6;
        locals.var_t1_dn7 = assign57830_e90034_d_n7;
        locals.var_t1_dn8 = assign57830_e90034_d_n8;
        locals.var_t1_dn9 = assign57830_e90034_d_n9;
        locals.var_t1_dn10 = assign57830_e90034_d_n10;
        locals.var_t1_dn11 = assign57830_e90034_d_n11;
        locals.var_t1_dn14 = assign57830_e90034_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign57840_e90043, assign57840_e90043_d_n0, assign57840_e90043_d_n2, assign57840_e90043_d_n4, assign57840_e90043_d_n5, assign57840_e90043_d_n6, assign57840_e90043_d_n7, assign57840_e90043_d_n8, assign57840_e90043_d_n9, assign57840_e90043_d_n10, assign57840_e90043_d_n11, assign57840_e90043_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign57840_e90041: f64 = (locals.var_t1 * locals.var_cox);
        (assign57840_e90041, ((locals.var_t1_dn0 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn0)), ((locals.var_t1_dn2 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn2)), ((locals.var_t1_dn4 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn4)), ((locals.var_t1_dn5 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn5)), ((locals.var_t1_dn6 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn6)), ((locals.var_t1_dn7 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn7)), ((locals.var_t1_dn8 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn8)), ((locals.var_t1_dn9 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn9)), ((locals.var_t1_dn10 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn10)), ((locals.var_t1_dn11 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn11)), ((locals.var_t1_dn14 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn14)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign57840_e90043;
        locals.var_ty_dn0 = assign57840_e90043_d_n0;
        locals.var_ty_dn2 = assign57840_e90043_d_n2;
        locals.var_ty_dn4 = assign57840_e90043_d_n4;
        locals.var_ty_dn5 = assign57840_e90043_d_n5;
        locals.var_ty_dn6 = assign57840_e90043_d_n6;
        locals.var_ty_dn7 = assign57840_e90043_d_n7;
        locals.var_ty_dn8 = assign57840_e90043_d_n8;
        locals.var_ty_dn9 = assign57840_e90043_d_n9;
        locals.var_ty_dn10 = assign57840_e90043_d_n10;
        locals.var_ty_dn11 = assign57840_e90043_d_n11;
        locals.var_ty_dn14 = assign57840_e90043_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign57850_e90056, assign57850_e90056_d_n0, assign57850_e90056_d_n2, assign57850_e90056_d_n4, assign57850_e90056_d_n5, assign57850_e90056_d_n6, assign57850_e90056_d_n7, assign57850_e90056_d_n8, assign57850_e90056_d_n9, assign57850_e90056_d_n10, assign57850_e90056_d_n11, assign57850_e90056_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign57850_e90051: f64 = (3.0 * 1.414213562373095);
        let assign57850_e90053: f64 = (assign57850_e90051 * locals.var_ty);
        let assign57850_e90054: f64 = (2.0 + assign57850_e90053);
        (assign57850_e90054, (assign57850_e90051 * locals.var_ty_dn0), (assign57850_e90051 * locals.var_ty_dn2), (assign57850_e90051 * locals.var_ty_dn4), (assign57850_e90051 * locals.var_ty_dn5), (assign57850_e90051 * locals.var_ty_dn6), (assign57850_e90051 * locals.var_ty_dn7), (assign57850_e90051 * locals.var_ty_dn8), (assign57850_e90051 * locals.var_ty_dn9), (assign57850_e90051 * locals.var_ty_dn10), (assign57850_e90051 * locals.var_ty_dn11), (assign57850_e90051 * locals.var_ty_dn14),)
    } else {
        (locals.var_ac41, locals.var_ac41_dn0, locals.var_ac41_dn2, locals.var_ac41_dn4, locals.var_ac41_dn5, locals.var_ac41_dn6, locals.var_ac41_dn7, locals.var_ac41_dn8, locals.var_ac41_dn9, locals.var_ac41_dn10, locals.var_ac41_dn11, locals.var_ac41_dn14,)
    }
};
        locals.var_ac41 = assign57850_e90056;
        locals.var_ac41_dn0 = assign57850_e90056_d_n0;
        locals.var_ac41_dn2 = assign57850_e90056_d_n2;
        locals.var_ac41_dn4 = assign57850_e90056_d_n4;
        locals.var_ac41_dn5 = assign57850_e90056_d_n5;
        locals.var_ac41_dn6 = assign57850_e90056_d_n6;
        locals.var_ac41_dn7 = assign57850_e90056_d_n7;
        locals.var_ac41_dn8 = assign57850_e90056_d_n8;
        locals.var_ac41_dn9 = assign57850_e90056_d_n9;
        locals.var_ac41_dn10 = assign57850_e90056_d_n10;
        locals.var_ac41_dn11 = assign57850_e90056_d_n11;
        locals.var_ac41_dn14 = assign57850_e90056_d_n14;
        locals.var_ac41_rv = 0.0;

        let (assign57860_e90069, assign57860_e90069_d_n0, assign57860_e90069_d_n2, assign57860_e90069_d_n4, assign57860_e90069_d_n5, assign57860_e90069_d_n6, assign57860_e90069_d_n7, assign57860_e90069_d_n8, assign57860_e90069_d_n9, assign57860_e90069_d_n10, assign57860_e90069_d_n11, assign57860_e90069_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign57860_e90063: f64 = (8.0 * locals.var_ac41);
        let assign57860_e90065: f64 = (assign57860_e90063 * locals.var_ac41);
        let assign57860_e90067: f64 = (assign57860_e90065 * locals.var_ac41);
        (assign57860_e90067, (((((8.0 * locals.var_ac41_dn0) * locals.var_ac41) + (assign57860_e90063 * locals.var_ac41_dn0)) * locals.var_ac41) + (assign57860_e90065 * locals.var_ac41_dn0)), (((((8.0 * locals.var_ac41_dn2) * locals.var_ac41) + (assign57860_e90063 * locals.var_ac41_dn2)) * locals.var_ac41) + (assign57860_e90065 * locals.var_ac41_dn2)), (((((8.0 * locals.var_ac41_dn4) * locals.var_ac41) + (assign57860_e90063 * locals.var_ac41_dn4)) * locals.var_ac41) + (assign57860_e90065 * locals.var_ac41_dn4)), (((((8.0 * locals.var_ac41_dn5) * locals.var_ac41) + (assign57860_e90063 * locals.var_ac41_dn5)) * locals.var_ac41) + (assign57860_e90065 * locals.var_ac41_dn5)), (((((8.0 * locals.var_ac41_dn6) * locals.var_ac41) + (assign57860_e90063 * locals.var_ac41_dn6)) * locals.var_ac41) + (assign57860_e90065 * locals.var_ac41_dn6)), (((((8.0 * locals.var_ac41_dn7) * locals.var_ac41) + (assign57860_e90063 * locals.var_ac41_dn7)) * locals.var_ac41) + (assign57860_e90065 * locals.var_ac41_dn7)), (((((8.0 * locals.var_ac41_dn8) * locals.var_ac41) + (assign57860_e90063 * locals.var_ac41_dn8)) * locals.var_ac41) + (assign57860_e90065 * locals.var_ac41_dn8)), (((((8.0 * locals.var_ac41_dn9) * locals.var_ac41) + (assign57860_e90063 * locals.var_ac41_dn9)) * locals.var_ac41) + (assign57860_e90065 * locals.var_ac41_dn9)), (((((8.0 * locals.var_ac41_dn10) * locals.var_ac41) + (assign57860_e90063 * locals.var_ac41_dn10)) * locals.var_ac41) + (assign57860_e90065 * locals.var_ac41_dn10)), (((((8.0 * locals.var_ac41_dn11) * locals.var_ac41) + (assign57860_e90063 * locals.var_ac41_dn11)) * locals.var_ac41) + (assign57860_e90065 * locals.var_ac41_dn11)), (((((8.0 * locals.var_ac41_dn14) * locals.var_ac41) + (assign57860_e90063 * locals.var_ac41_dn14)) * locals.var_ac41) + (assign57860_e90065 * locals.var_ac41_dn14)),)
    } else {
        (locals.var_ac4, locals.var_ac4_dn0, locals.var_ac4_dn2, locals.var_ac4_dn4, locals.var_ac4_dn5, locals.var_ac4_dn6, locals.var_ac4_dn7, locals.var_ac4_dn8, locals.var_ac4_dn9, locals.var_ac4_dn10, locals.var_ac4_dn11, locals.var_ac4_dn14,)
    }
};
        locals.var_ac4 = assign57860_e90069;
        locals.var_ac4_dn0 = assign57860_e90069_d_n0;
        locals.var_ac4_dn2 = assign57860_e90069_d_n2;
        locals.var_ac4_dn4 = assign57860_e90069_d_n4;
        locals.var_ac4_dn5 = assign57860_e90069_d_n5;
        locals.var_ac4_dn6 = assign57860_e90069_d_n6;
        locals.var_ac4_dn7 = assign57860_e90069_d_n7;
        locals.var_ac4_dn8 = assign57860_e90069_d_n8;
        locals.var_ac4_dn9 = assign57860_e90069_d_n9;
        locals.var_ac4_dn10 = assign57860_e90069_d_n10;
        locals.var_ac4_dn11 = assign57860_e90069_d_n11;
        locals.var_ac4_dn14 = assign57860_e90069_d_n14;
        locals.var_ac4_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_212(
        locals: &mut StampLocals,
    ) {
        let (assign57870_e90078, assign57870_e90078_d_n0, assign57870_e90078_d_n2, assign57870_e90078_d_n4, assign57870_e90078_d_n5, assign57870_e90078_d_n6, assign57870_e90078_d_n7, assign57870_e90078_d_n8, assign57870_e90078_d_n9, assign57870_e90078_d_n10, assign57870_e90078_d_n11, assign57870_e90078_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign57870_e90076: f64 = (locals.var_tx - 2.0);
        (assign57870_e90076, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign57870_e90078;
        locals.var_t4_dn0 = assign57870_e90078_d_n0;
        locals.var_t4_dn2 = assign57870_e90078_d_n2;
        locals.var_t4_dn4 = assign57870_e90078_d_n4;
        locals.var_t4_dn5 = assign57870_e90078_d_n5;
        locals.var_t4_dn6 = assign57870_e90078_d_n6;
        locals.var_t4_dn7 = assign57870_e90078_d_n7;
        locals.var_t4_dn8 = assign57870_e90078_d_n8;
        locals.var_t4_dn9 = assign57870_e90078_d_n9;
        locals.var_t4_dn10 = assign57870_e90078_d_n10;
        locals.var_t4_dn11 = assign57870_e90078_d_n11;
        locals.var_t4_dn14 = assign57870_e90078_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign57880_e90089, assign57880_e90089_d_n0, assign57880_e90089_d_n2, assign57880_e90089_d_n4, assign57880_e90089_d_n5, assign57880_e90089_d_n6, assign57880_e90089_d_n7, assign57880_e90089_d_n8, assign57880_e90089_d_n9, assign57880_e90089_d_n10, assign57880_e90089_d_n11, assign57880_e90089_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign57880_e90085: f64 = (9.0 * locals.var_ty);
        let assign57880_e90087: f64 = (assign57880_e90085 * locals.var_t4);
        (assign57880_e90087, (((9.0 * locals.var_ty_dn0) * locals.var_t4) + (assign57880_e90085 * locals.var_t4_dn0)), (((9.0 * locals.var_ty_dn2) * locals.var_t4) + (assign57880_e90085 * locals.var_t4_dn2)), (((9.0 * locals.var_ty_dn4) * locals.var_t4) + (assign57880_e90085 * locals.var_t4_dn4)), (((9.0 * locals.var_ty_dn5) * locals.var_t4) + (assign57880_e90085 * locals.var_t4_dn5)), (((9.0 * locals.var_ty_dn6) * locals.var_t4) + (assign57880_e90085 * locals.var_t4_dn6)), (((9.0 * locals.var_ty_dn7) * locals.var_t4) + (assign57880_e90085 * locals.var_t4_dn7)), (((9.0 * locals.var_ty_dn8) * locals.var_t4) + (assign57880_e90085 * locals.var_t4_dn8)), (((9.0 * locals.var_ty_dn9) * locals.var_t4) + (assign57880_e90085 * locals.var_t4_dn9)), (((9.0 * locals.var_ty_dn10) * locals.var_t4) + (assign57880_e90085 * locals.var_t4_dn10)), (((9.0 * locals.var_ty_dn11) * locals.var_t4) + (assign57880_e90085 * locals.var_t4_dn11)), (((9.0 * locals.var_ty_dn14) * locals.var_t4) + (assign57880_e90085 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign57880_e90089;
        locals.var_t5_dn0 = assign57880_e90089_d_n0;
        locals.var_t5_dn2 = assign57880_e90089_d_n2;
        locals.var_t5_dn4 = assign57880_e90089_d_n4;
        locals.var_t5_dn5 = assign57880_e90089_d_n5;
        locals.var_t5_dn6 = assign57880_e90089_d_n6;
        locals.var_t5_dn7 = assign57880_e90089_d_n7;
        locals.var_t5_dn8 = assign57880_e90089_d_n8;
        locals.var_t5_dn9 = assign57880_e90089_d_n9;
        locals.var_t5_dn10 = assign57880_e90089_d_n10;
        locals.var_t5_dn11 = assign57880_e90089_d_n11;
        locals.var_t5_dn14 = assign57880_e90089_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign57890_e90100, assign57890_e90100_d_n0, assign57890_e90100_d_n2, assign57890_e90100_d_n4, assign57890_e90100_d_n5, assign57890_e90100_d_n6, assign57890_e90100_d_n7, assign57890_e90100_d_n8, assign57890_e90100_d_n9, assign57890_e90100_d_n10, assign57890_e90100_d_n11, assign57890_e90100_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign57890_e90096: f64 = (7.0 * 1.414213562373095);
        let assign57890_e90098: f64 = (assign57890_e90096 - locals.var_t5);
        (assign57890_e90098, (-locals.var_t5_dn0), (-locals.var_t5_dn2), (-locals.var_t5_dn4), (-locals.var_t5_dn5), (-locals.var_t5_dn6), (-locals.var_t5_dn7), (-locals.var_t5_dn8), (-locals.var_t5_dn9), (-locals.var_t5_dn10), (-locals.var_t5_dn11), (-locals.var_t5_dn14),)
    } else {
        (locals.var_ac31, locals.var_ac31_dn0, locals.var_ac31_dn2, locals.var_ac31_dn4, locals.var_ac31_dn5, locals.var_ac31_dn6, locals.var_ac31_dn7, locals.var_ac31_dn8, locals.var_ac31_dn9, locals.var_ac31_dn10, locals.var_ac31_dn11, locals.var_ac31_dn14,)
    }
};
        locals.var_ac31 = assign57890_e90100;
        locals.var_ac31_dn0 = assign57890_e90100_d_n0;
        locals.var_ac31_dn2 = assign57890_e90100_d_n2;
        locals.var_ac31_dn4 = assign57890_e90100_d_n4;
        locals.var_ac31_dn5 = assign57890_e90100_d_n5;
        locals.var_ac31_dn6 = assign57890_e90100_d_n6;
        locals.var_ac31_dn7 = assign57890_e90100_d_n7;
        locals.var_ac31_dn8 = assign57890_e90100_d_n8;
        locals.var_ac31_dn9 = assign57890_e90100_d_n9;
        locals.var_ac31_dn10 = assign57890_e90100_d_n10;
        locals.var_ac31_dn11 = assign57890_e90100_d_n11;
        locals.var_ac31_dn14 = assign57890_e90100_d_n14;
        locals.var_ac31_rv = 0.0;

        let (assign57900_e90109, assign57900_e90109_d_n0, assign57900_e90109_d_n2, assign57900_e90109_d_n4, assign57900_e90109_d_n5, assign57900_e90109_d_n6, assign57900_e90109_d_n7, assign57900_e90109_d_n8, assign57900_e90109_d_n9, assign57900_e90109_d_n10, assign57900_e90109_d_n11, assign57900_e90109_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign57900_e90107: f64 = (locals.var_ac31 * locals.var_ac31);
        (assign57900_e90107, ((locals.var_ac31_dn0 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn0)), ((locals.var_ac31_dn2 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn2)), ((locals.var_ac31_dn4 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn4)), ((locals.var_ac31_dn5 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn5)), ((locals.var_ac31_dn6 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn6)), ((locals.var_ac31_dn7 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn7)), ((locals.var_ac31_dn8 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn8)), ((locals.var_ac31_dn9 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn9)), ((locals.var_ac31_dn10 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn10)), ((locals.var_ac31_dn11 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn11)), ((locals.var_ac31_dn14 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn14)),)
    } else {
        (locals.var_ac3, locals.var_ac3_dn0, locals.var_ac3_dn2, locals.var_ac3_dn4, locals.var_ac3_dn5, locals.var_ac3_dn6, locals.var_ac3_dn7, locals.var_ac3_dn8, locals.var_ac3_dn9, locals.var_ac3_dn10, locals.var_ac3_dn11, locals.var_ac3_dn14,)
    }
};
        locals.var_ac3 = assign57900_e90109;
        locals.var_ac3_dn0 = assign57900_e90109_d_n0;
        locals.var_ac3_dn2 = assign57900_e90109_d_n2;
        locals.var_ac3_dn4 = assign57900_e90109_d_n4;
        locals.var_ac3_dn5 = assign57900_e90109_d_n5;
        locals.var_ac3_dn6 = assign57900_e90109_d_n6;
        locals.var_ac3_dn7 = assign57900_e90109_d_n7;
        locals.var_ac3_dn8 = assign57900_e90109_d_n8;
        locals.var_ac3_dn9 = assign57900_e90109_d_n9;
        locals.var_ac3_dn10 = assign57900_e90109_d_n10;
        locals.var_ac3_dn11 = assign57900_e90109_d_n11;
        locals.var_ac3_dn14 = assign57900_e90109_d_n14;
        locals.var_ac3_rv = 0.0;

        let assign57910_e90113: f64 = (locals.var_ac3 * 1e-8);
        let assign57910_e90114: f64 = if locals.var_ac4 < assign57910_e90113 { 1.0 } else { 0.0 };
        locals.var_guard1431 = assign57910_e90114;
        locals.var_guard1431_rv = 0.0;

        let (assign57920_e90127, assign57920_e90127_d_n0, assign57920_e90127_d_n2, assign57920_e90127_d_n4, assign57920_e90127_d_n5, assign57920_e90127_d_n6, assign57920_e90127_d_n7, assign57920_e90127_d_n8, assign57920_e90127_d_n9, assign57920_e90127_d_n10, assign57920_e90127_d_n11, assign57920_e90127_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1431 != 0.0)) {
        let assign57920_e90123: f64 = (0.5 * locals.var_ac4);
        let assign57920_e90125: f64 = (assign57920_e90123 / locals.var_ac31);
        (assign57920_e90125, ((((0.5 * locals.var_ac4_dn0) * locals.var_ac31) - (assign57920_e90123 * locals.var_ac31_dn0)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn2) * locals.var_ac31) - (assign57920_e90123 * locals.var_ac31_dn2)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn4) * locals.var_ac31) - (assign57920_e90123 * locals.var_ac31_dn4)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn5) * locals.var_ac31) - (assign57920_e90123 * locals.var_ac31_dn5)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn6) * locals.var_ac31) - (assign57920_e90123 * locals.var_ac31_dn6)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn7) * locals.var_ac31) - (assign57920_e90123 * locals.var_ac31_dn7)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn8) * locals.var_ac31) - (assign57920_e90123 * locals.var_ac31_dn8)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn9) * locals.var_ac31) - (assign57920_e90123 * locals.var_ac31_dn9)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn10) * locals.var_ac31) - (assign57920_e90123 * locals.var_ac31_dn10)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn11) * locals.var_ac31) - (assign57920_e90123 * locals.var_ac31_dn11)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn14) * locals.var_ac31) - (assign57920_e90123 * locals.var_ac31_dn14)) / (locals.var_ac31 * locals.var_ac31)),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn14,)
    }
};
        locals.var_ac1 = assign57920_e90127;
        locals.var_ac1_dn0 = assign57920_e90127_d_n0;
        locals.var_ac1_dn2 = assign57920_e90127_d_n2;
        locals.var_ac1_dn4 = assign57920_e90127_d_n4;
        locals.var_ac1_dn5 = assign57920_e90127_d_n5;
        locals.var_ac1_dn6 = assign57920_e90127_d_n6;
        locals.var_ac1_dn7 = assign57920_e90127_d_n7;
        locals.var_ac1_dn8 = assign57920_e90127_d_n8;
        locals.var_ac1_dn9 = assign57920_e90127_d_n9;
        locals.var_ac1_dn10 = assign57920_e90127_d_n10;
        locals.var_ac1_dn11 = assign57920_e90127_d_n11;
        locals.var_ac1_dn14 = assign57920_e90127_d_n14;
        locals.var_ac1_rv = 0.0;

        let (assign57930_e90140, assign57930_e90140_d_n0, assign57930_e90140_d_n2, assign57930_e90140_d_n4, assign57930_e90140_d_n5, assign57930_e90140_d_n6, assign57930_e90140_d_n7, assign57930_e90140_d_n8, assign57930_e90140_d_n9, assign57930_e90140_d_n10, assign57930_e90140_d_n11, assign57930_e90140_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1431 == 0.0)) {
        let assign57930_e90137: f64 = (locals.var_ac4 + locals.var_ac3);
        let assign57930_e90138: f64 = (assign57930_e90137).sqrt();
        (assign57930_e90138, ((locals.var_ac4_dn0 + locals.var_ac3_dn0) / (2.0 * assign57930_e90138)), ((locals.var_ac4_dn2 + locals.var_ac3_dn2) / (2.0 * assign57930_e90138)), ((locals.var_ac4_dn4 + locals.var_ac3_dn4) / (2.0 * assign57930_e90138)), ((locals.var_ac4_dn5 + locals.var_ac3_dn5) / (2.0 * assign57930_e90138)), ((locals.var_ac4_dn6 + locals.var_ac3_dn6) / (2.0 * assign57930_e90138)), ((locals.var_ac4_dn7 + locals.var_ac3_dn7) / (2.0 * assign57930_e90138)), ((locals.var_ac4_dn8 + locals.var_ac3_dn8) / (2.0 * assign57930_e90138)), ((locals.var_ac4_dn9 + locals.var_ac3_dn9) / (2.0 * assign57930_e90138)), ((locals.var_ac4_dn10 + locals.var_ac3_dn10) / (2.0 * assign57930_e90138)), ((locals.var_ac4_dn11 + locals.var_ac3_dn11) / (2.0 * assign57930_e90138)), ((locals.var_ac4_dn14 + locals.var_ac3_dn14) / (2.0 * assign57930_e90138)),)
    } else {
        (locals.var_ac2, locals.var_ac2_dn0, locals.var_ac2_dn2, locals.var_ac2_dn4, locals.var_ac2_dn5, locals.var_ac2_dn6, locals.var_ac2_dn7, locals.var_ac2_dn8, locals.var_ac2_dn9, locals.var_ac2_dn10, locals.var_ac2_dn11, locals.var_ac2_dn14,)
    }
};
        locals.var_ac2 = assign57930_e90140;
        locals.var_ac2_dn0 = assign57930_e90140_d_n0;
        locals.var_ac2_dn2 = assign57930_e90140_d_n2;
        locals.var_ac2_dn4 = assign57930_e90140_d_n4;
        locals.var_ac2_dn5 = assign57930_e90140_d_n5;
        locals.var_ac2_dn6 = assign57930_e90140_d_n6;
        locals.var_ac2_dn7 = assign57930_e90140_d_n7;
        locals.var_ac2_dn8 = assign57930_e90140_d_n8;
        locals.var_ac2_dn9 = assign57930_e90140_d_n9;
        locals.var_ac2_dn10 = assign57930_e90140_d_n10;
        locals.var_ac2_dn11 = assign57930_e90140_d_n11;
        locals.var_ac2_dn14 = assign57930_e90140_d_n14;
        locals.var_ac2_rv = 0.0;

        let (assign57940_e90153, assign57940_e90153_d_n0, assign57940_e90153_d_n2, assign57940_e90153_d_n4, assign57940_e90153_d_n5, assign57940_e90153_d_n6, assign57940_e90153_d_n7, assign57940_e90153_d_n8, assign57940_e90153_d_n9, assign57940_e90153_d_n10, assign57940_e90153_d_n11, assign57940_e90153_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1430 != 0.0)) && (locals.var_guard1431 == 0.0)) {
        let assign57940_e90149: f64 = (-locals.var_ac31);
        let assign57940_e90151: f64 = (assign57940_e90149 + locals.var_ac2);
        (assign57940_e90151, ((-locals.var_ac31_dn0) + locals.var_ac2_dn0), ((-locals.var_ac31_dn2) + locals.var_ac2_dn2), ((-locals.var_ac31_dn4) + locals.var_ac2_dn4), ((-locals.var_ac31_dn5) + locals.var_ac2_dn5), ((-locals.var_ac31_dn6) + locals.var_ac2_dn6), ((-locals.var_ac31_dn7) + locals.var_ac2_dn7), ((-locals.var_ac31_dn8) + locals.var_ac2_dn8), ((-locals.var_ac31_dn9) + locals.var_ac2_dn9), ((-locals.var_ac31_dn10) + locals.var_ac2_dn10), ((-locals.var_ac31_dn11) + locals.var_ac2_dn11), ((-locals.var_ac31_dn14) + locals.var_ac2_dn14),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn14,)
    }
};
        locals.var_ac1 = assign57940_e90153;
        locals.var_ac1_dn0 = assign57940_e90153_d_n0;
        locals.var_ac1_dn2 = assign57940_e90153_d_n2;
        locals.var_ac1_dn4 = assign57940_e90153_d_n4;
        locals.var_ac1_dn5 = assign57940_e90153_d_n5;
        locals.var_ac1_dn6 = assign57940_e90153_d_n6;
        locals.var_ac1_dn7 = assign57940_e90153_d_n7;
        locals.var_ac1_dn8 = assign57940_e90153_d_n8;
        locals.var_ac1_dn9 = assign57940_e90153_d_n9;
        locals.var_ac1_dn10 = assign57940_e90153_d_n10;
        locals.var_ac1_dn11 = assign57940_e90153_d_n11;
        locals.var_ac1_dn14 = assign57940_e90153_d_n14;
        locals.var_ac1_rv = 0.0;

        let (assign57950_e90167, assign57950_e90167_d_n0, assign57950_e90167_d_n2, assign57950_e90167_d_n4, assign57950_e90167_d_n5, assign57950_e90167_d_n6, assign57950_e90167_d_n7, assign57950_e90167_d_n8, assign57950_e90167_d_n9, assign57950_e90167_d_n10, assign57950_e90167_d_n11, assign57950_e90167_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let (assign57950_e90165, assign57950_e90165_d_n0, assign57950_e90165_d_n2, assign57950_e90165_d_n4, assign57950_e90165_d_n5, assign57950_e90165_d_n6, assign57950_e90165_d_n7, assign57950_e90165_d_n8, assign57950_e90165_d_n9, assign57950_e90165_d_n10, assign57950_e90165_d_n11, assign57950_e90165_d_n14,) = {
            if (locals.var_ac1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign57950_e90164: f64 = (locals.var_ac1).powf(0.3333333333333333);
                (assign57950_e90164, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn0)) } } else { (assign57950_e90164 * (0.3333333333333333 * (locals.var_ac1_dn0 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn2)) } } else { (assign57950_e90164 * (0.3333333333333333 * (locals.var_ac1_dn2 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn4)) } } else { (assign57950_e90164 * (0.3333333333333333 * (locals.var_ac1_dn4 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn5)) } } else { (assign57950_e90164 * (0.3333333333333333 * (locals.var_ac1_dn5 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn6)) } } else { (assign57950_e90164 * (0.3333333333333333 * (locals.var_ac1_dn6 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn7)) } } else { (assign57950_e90164 * (0.3333333333333333 * (locals.var_ac1_dn7 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn8)) } } else { (assign57950_e90164 * (0.3333333333333333 * (locals.var_ac1_dn8 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn9)) } } else { (assign57950_e90164 * (0.3333333333333333 * (locals.var_ac1_dn9 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn10)) } } else { (assign57950_e90164 * (0.3333333333333333 * (locals.var_ac1_dn10 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn11)) } } else { (assign57950_e90164 * (0.3333333333333333 * (locals.var_ac1_dn11 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn14)) } } else { (assign57950_e90164 * (0.3333333333333333 * (locals.var_ac1_dn14 / locals.var_ac1))) },)
            }
        };
        (assign57950_e90165, assign57950_e90165_d_n0, assign57950_e90165_d_n2, assign57950_e90165_d_n4, assign57950_e90165_d_n5, assign57950_e90165_d_n6, assign57950_e90165_d_n7, assign57950_e90165_d_n8, assign57950_e90165_d_n9, assign57950_e90165_d_n10, assign57950_e90165_d_n11, assign57950_e90165_d_n14,)
    } else {
        (locals.var_acd, locals.var_acd_dn0, locals.var_acd_dn2, locals.var_acd_dn4, locals.var_acd_dn5, locals.var_acd_dn6, locals.var_acd_dn7, locals.var_acd_dn8, locals.var_acd_dn9, locals.var_acd_dn10, locals.var_acd_dn11, locals.var_acd_dn14,)
    }
};
        locals.var_acd = assign57950_e90167;
        locals.var_acd_dn0 = assign57950_e90167_d_n0;
        locals.var_acd_dn2 = assign57950_e90167_d_n2;
        locals.var_acd_dn4 = assign57950_e90167_d_n4;
        locals.var_acd_dn5 = assign57950_e90167_d_n5;
        locals.var_acd_dn6 = assign57950_e90167_d_n6;
        locals.var_acd_dn7 = assign57950_e90167_d_n7;
        locals.var_acd_dn8 = assign57950_e90167_d_n8;
        locals.var_acd_dn9 = assign57950_e90167_d_n9;
        locals.var_acd_dn10 = assign57950_e90167_d_n10;
        locals.var_acd_dn11 = assign57950_e90167_d_n11;
        locals.var_acd_dn14 = assign57950_e90167_d_n14;
        locals.var_acd_rv = 0.0;

        let (assign57960_e90191, assign57960_e90191_d_n0, assign57960_e90191_d_n2, assign57960_e90191_d_n4, assign57960_e90191_d_n5, assign57960_e90191_d_n6, assign57960_e90191_d_n7, assign57960_e90191_d_n8, assign57960_e90191_d_n9, assign57960_e90191_d_n10, assign57960_e90191_d_n11, assign57960_e90191_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign57960_e90173: f64 = (-4.0);
        let assign57960_e90175: f64 = (assign57960_e90173 * 1.414213562373095);
        let assign57960_e90178: f64 = (12.0 * locals.var_ty);
        let assign57960_e90179: f64 = (assign57960_e90175 - assign57960_e90178);
        let assign57960_e90182: f64 = (2.0 * locals.var_acd);
        let assign57960_e90183: f64 = (assign57960_e90179 + assign57960_e90182);
        let assign57960_e90186: f64 = (1.414213562373095 * locals.var_acd);
        let assign57960_e90188: f64 = (assign57960_e90186 * locals.var_acd);
        let assign57960_e90189: f64 = (assign57960_e90183 + assign57960_e90188);
        (assign57960_e90189, (((-(12.0 * locals.var_ty_dn0)) + (2.0 * locals.var_acd_dn0)) + (((1.414213562373095 * locals.var_acd_dn0) * locals.var_acd) + (assign57960_e90186 * locals.var_acd_dn0))), (((-(12.0 * locals.var_ty_dn2)) + (2.0 * locals.var_acd_dn2)) + (((1.414213562373095 * locals.var_acd_dn2) * locals.var_acd) + (assign57960_e90186 * locals.var_acd_dn2))), (((-(12.0 * locals.var_ty_dn4)) + (2.0 * locals.var_acd_dn4)) + (((1.414213562373095 * locals.var_acd_dn4) * locals.var_acd) + (assign57960_e90186 * locals.var_acd_dn4))), (((-(12.0 * locals.var_ty_dn5)) + (2.0 * locals.var_acd_dn5)) + (((1.414213562373095 * locals.var_acd_dn5) * locals.var_acd) + (assign57960_e90186 * locals.var_acd_dn5))), (((-(12.0 * locals.var_ty_dn6)) + (2.0 * locals.var_acd_dn6)) + (((1.414213562373095 * locals.var_acd_dn6) * locals.var_acd) + (assign57960_e90186 * locals.var_acd_dn6))), (((-(12.0 * locals.var_ty_dn7)) + (2.0 * locals.var_acd_dn7)) + (((1.414213562373095 * locals.var_acd_dn7) * locals.var_acd) + (assign57960_e90186 * locals.var_acd_dn7))), (((-(12.0 * locals.var_ty_dn8)) + (2.0 * locals.var_acd_dn8)) + (((1.414213562373095 * locals.var_acd_dn8) * locals.var_acd) + (assign57960_e90186 * locals.var_acd_dn8))), (((-(12.0 * locals.var_ty_dn9)) + (2.0 * locals.var_acd_dn9)) + (((1.414213562373095 * locals.var_acd_dn9) * locals.var_acd) + (assign57960_e90186 * locals.var_acd_dn9))), (((-(12.0 * locals.var_ty_dn10)) + (2.0 * locals.var_acd_dn10)) + (((1.414213562373095 * locals.var_acd_dn10) * locals.var_acd) + (assign57960_e90186 * locals.var_acd_dn10))), (((-(12.0 * locals.var_ty_dn11)) + (2.0 * locals.var_acd_dn11)) + (((1.414213562373095 * locals.var_acd_dn11) * locals.var_acd) + (assign57960_e90186 * locals.var_acd_dn11))), (((-(12.0 * locals.var_ty_dn14)) + (2.0 * locals.var_acd_dn14)) + (((1.414213562373095 * locals.var_acd_dn14) * locals.var_acd) + (assign57960_e90186 * locals.var_acd_dn14))),)
    } else {
        (locals.var_acn, locals.var_acn_dn0, locals.var_acn_dn2, locals.var_acn_dn4, locals.var_acn_dn5, locals.var_acn_dn6, locals.var_acn_dn7, locals.var_acn_dn8, locals.var_acn_dn9, locals.var_acn_dn10, locals.var_acn_dn11, locals.var_acn_dn14,)
    }
};
        locals.var_acn = assign57960_e90191;
        locals.var_acn_dn0 = assign57960_e90191_d_n0;
        locals.var_acn_dn2 = assign57960_e90191_d_n2;
        locals.var_acn_dn4 = assign57960_e90191_d_n4;
        locals.var_acn_dn5 = assign57960_e90191_d_n5;
        locals.var_acn_dn6 = assign57960_e90191_d_n6;
        locals.var_acn_dn7 = assign57960_e90191_d_n7;
        locals.var_acn_dn8 = assign57960_e90191_d_n8;
        locals.var_acn_dn9 = assign57960_e90191_d_n9;
        locals.var_acn_dn10 = assign57960_e90191_d_n10;
        locals.var_acn_dn11 = assign57960_e90191_d_n11;
        locals.var_acn_dn14 = assign57960_e90191_d_n14;
        locals.var_acn_rv = 0.0;

        let (assign57970_e90200, assign57970_e90200_d_n0, assign57970_e90200_d_n2, assign57970_e90200_d_n4, assign57970_e90200_d_n5, assign57970_e90200_d_n6, assign57970_e90200_d_n7, assign57970_e90200_d_n8, assign57970_e90200_d_n9, assign57970_e90200_d_n10, assign57970_e90200_d_n11, assign57970_e90200_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign57970_e90198: f64 = (1.0 / locals.var_acd);
        (assign57970_e90198, (-(locals.var_acd_dn0 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn2 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn4 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn5 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn6 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn7 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn8 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn9 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn10 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn11 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn14 / (locals.var_acd * locals.var_acd))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign57970_e90200;
        locals.var_t1_dn0 = assign57970_e90200_d_n0;
        locals.var_t1_dn2 = assign57970_e90200_d_n2;
        locals.var_t1_dn4 = assign57970_e90200_d_n4;
        locals.var_t1_dn5 = assign57970_e90200_d_n5;
        locals.var_t1_dn6 = assign57970_e90200_d_n6;
        locals.var_t1_dn7 = assign57970_e90200_d_n7;
        locals.var_t1_dn8 = assign57970_e90200_d_n8;
        locals.var_t1_dn9 = assign57970_e90200_d_n9;
        locals.var_t1_dn10 = assign57970_e90200_d_n10;
        locals.var_t1_dn11 = assign57970_e90200_d_n11;
        locals.var_t1_dn14 = assign57970_e90200_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign57980_e90209, assign57980_e90209_d_n0, assign57980_e90209_d_n2, assign57980_e90209_d_n4, assign57980_e90209_d_n5, assign57980_e90209_d_n6, assign57980_e90209_d_n7, assign57980_e90209_d_n8, assign57980_e90209_d_n9, assign57980_e90209_d_n10, assign57980_e90209_d_n11, assign57980_e90209_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign57980_e90207: f64 = (locals.var_acn * locals.var_t1);
        (assign57980_e90207, ((locals.var_acn_dn0 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn0)), ((locals.var_acn_dn2 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn2)), ((locals.var_acn_dn4 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn4)), ((locals.var_acn_dn5 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn5)), ((locals.var_acn_dn6 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn6)), ((locals.var_acn_dn7 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn7)), ((locals.var_acn_dn8 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn8)), ((locals.var_acn_dn9 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn9)), ((locals.var_acn_dn10 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn10)), ((locals.var_acn_dn11 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn11)), ((locals.var_acn_dn14 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn14)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign57980_e90209;
        locals.var_chi_dn0 = assign57980_e90209_d_n0;
        locals.var_chi_dn2 = assign57980_e90209_d_n2;
        locals.var_chi_dn4 = assign57980_e90209_d_n4;
        locals.var_chi_dn5 = assign57980_e90209_d_n5;
        locals.var_chi_dn6 = assign57980_e90209_d_n6;
        locals.var_chi_dn7 = assign57980_e90209_d_n7;
        locals.var_chi_dn8 = assign57980_e90209_d_n8;
        locals.var_chi_dn9 = assign57980_e90209_d_n9;
        locals.var_chi_dn10 = assign57980_e90209_d_n10;
        locals.var_chi_dn11 = assign57980_e90209_d_n11;
        locals.var_chi_dn14 = assign57980_e90209_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign57990_e90220, assign57990_e90220_d_n0, assign57990_e90220_d_n2, assign57990_e90220_d_n4, assign57990_e90220_d_n5, assign57990_e90220_d_n6, assign57990_e90220_d_n7, assign57990_e90220_d_n8, assign57990_e90220_d_n9, assign57990_e90220_d_n10, assign57990_e90220_d_n11, assign57990_e90220_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign57990_e90216: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign57990_e90218: f64 = (assign57990_e90216 + locals.var_vbscl__blk437);
        (assign57990_e90218, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) + locals.var_vbscl__blk437_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) + locals.var_vbscl__blk437_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) + locals.var_vbscl__blk437_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) + locals.var_vbscl__blk437_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) + locals.var_vbscl__blk437_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) + locals.var_vbscl__blk437_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) + locals.var_vbscl__blk437_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) + locals.var_vbscl__blk437_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) + locals.var_vbscl__blk437_dn10), (((locals.var_chi_dn11 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn11)) + locals.var_vbscl__blk437_dn11), (((locals.var_chi_dn14 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn14)) + locals.var_vbscl__blk437_dn14),)
    } else {
        (locals.var_psa, locals.var_psa_dn0, locals.var_psa_dn2, locals.var_psa_dn4, locals.var_psa_dn5, locals.var_psa_dn6, locals.var_psa_dn7, locals.var_psa_dn8, locals.var_psa_dn9, locals.var_psa_dn10, locals.var_psa_dn11, locals.var_psa_dn14,)
    }
};
        locals.var_psa = assign57990_e90220;
        locals.var_psa_dn0 = assign57990_e90220_d_n0;
        locals.var_psa_dn2 = assign57990_e90220_d_n2;
        locals.var_psa_dn4 = assign57990_e90220_d_n4;
        locals.var_psa_dn5 = assign57990_e90220_d_n5;
        locals.var_psa_dn6 = assign57990_e90220_d_n6;
        locals.var_psa_dn7 = assign57990_e90220_d_n7;
        locals.var_psa_dn8 = assign57990_e90220_d_n8;
        locals.var_psa_dn9 = assign57990_e90220_d_n9;
        locals.var_psa_dn10 = assign57990_e90220_d_n10;
        locals.var_psa_dn11 = assign57990_e90220_d_n11;
        locals.var_psa_dn14 = assign57990_e90220_d_n14;
        locals.var_psa_rv = 0.0;

        let (assign58000_e90229, assign58000_e90229_d_n0, assign58000_e90229_d_n2, assign58000_e90229_d_n4, assign58000_e90229_d_n5, assign58000_e90229_d_n6, assign58000_e90229_d_n7, assign58000_e90229_d_n8, assign58000_e90229_d_n9, assign58000_e90229_d_n10, assign58000_e90229_d_n11, assign58000_e90229_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign58000_e90227: f64 = (locals.var_psa - locals.var_vbscl__blk437);
        (assign58000_e90227, (locals.var_psa_dn0 - locals.var_vbscl__blk437_dn0), (locals.var_psa_dn2 - locals.var_vbscl__blk437_dn2), (locals.var_psa_dn4 - locals.var_vbscl__blk437_dn4), (locals.var_psa_dn5 - locals.var_vbscl__blk437_dn5), (locals.var_psa_dn6 - locals.var_vbscl__blk437_dn6), (locals.var_psa_dn7 - locals.var_vbscl__blk437_dn7), (locals.var_psa_dn8 - locals.var_vbscl__blk437_dn8), (locals.var_psa_dn9 - locals.var_vbscl__blk437_dn9), (locals.var_psa_dn10 - locals.var_vbscl__blk437_dn10), (locals.var_psa_dn11 - locals.var_vbscl__blk437_dn11), (locals.var_psa_dn14 - locals.var_vbscl__blk437_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign58000_e90229;
        locals.var_t1_dn0 = assign58000_e90229_d_n0;
        locals.var_t1_dn2 = assign58000_e90229_d_n2;
        locals.var_t1_dn4 = assign58000_e90229_d_n4;
        locals.var_t1_dn5 = assign58000_e90229_d_n5;
        locals.var_t1_dn6 = assign58000_e90229_d_n6;
        locals.var_t1_dn7 = assign58000_e90229_d_n7;
        locals.var_t1_dn8 = assign58000_e90229_d_n8;
        locals.var_t1_dn9 = assign58000_e90229_d_n9;
        locals.var_t1_dn10 = assign58000_e90229_d_n10;
        locals.var_t1_dn11 = assign58000_e90229_d_n11;
        locals.var_t1_dn14 = assign58000_e90229_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign58010_e90238, assign58010_e90238_d_n0, assign58010_e90238_d_n2, assign58010_e90238_d_n4, assign58010_e90238_d_n5, assign58010_e90238_d_n6, assign58010_e90238_d_n7, assign58010_e90238_d_n8, assign58010_e90238_d_n9, assign58010_e90238_d_n10, assign58010_e90238_d_n11, assign58010_e90238_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign58010_e90236: f64 = (locals.var_t1 / locals.var_ps0_min);
        (assign58010_e90236, (((locals.var_t1_dn0 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn0)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn2 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn2)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn4 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn4)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn5 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn5)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn6 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn6)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn7 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn7)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn8 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn8)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn9 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn9)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn10 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn10)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn11 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn11)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn14 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn14)) / (locals.var_ps0_min * locals.var_ps0_min)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign58010_e90238;
        locals.var_t2_dn0 = assign58010_e90238_d_n0;
        locals.var_t2_dn2 = assign58010_e90238_d_n2;
        locals.var_t2_dn4 = assign58010_e90238_d_n4;
        locals.var_t2_dn5 = assign58010_e90238_d_n5;
        locals.var_t2_dn6 = assign58010_e90238_d_n6;
        locals.var_t2_dn7 = assign58010_e90238_d_n7;
        locals.var_t2_dn8 = assign58010_e90238_d_n8;
        locals.var_t2_dn9 = assign58010_e90238_d_n9;
        locals.var_t2_dn10 = assign58010_e90238_d_n10;
        locals.var_t2_dn11 = assign58010_e90238_d_n11;
        locals.var_t2_dn14 = assign58010_e90238_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign58020_e90250, assign58020_e90250_d_n0, assign58020_e90250_d_n2, assign58020_e90250_d_n4, assign58020_e90250_d_n5, assign58020_e90250_d_n6, assign58020_e90250_d_n7, assign58020_e90250_d_n8, assign58020_e90250_d_n9, assign58020_e90250_d_n10, assign58020_e90250_d_n11, assign58020_e90250_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign58020_e90246: f64 = (locals.var_t2 * locals.var_t2);
        let assign58020_e90247: f64 = (1.0 + assign58020_e90246);
        let assign58020_e90248: f64 = (assign58020_e90247).sqrt();
        (assign58020_e90248, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign58020_e90248)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign58020_e90248)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign58020_e90248)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign58020_e90248)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign58020_e90248)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign58020_e90248)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign58020_e90248)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign58020_e90248)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign58020_e90248)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign58020_e90248)), (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign58020_e90248)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign58020_e90250;
        locals.var_t3_dn0 = assign58020_e90250_d_n0;
        locals.var_t3_dn2 = assign58020_e90250_d_n2;
        locals.var_t3_dn4 = assign58020_e90250_d_n4;
        locals.var_t3_dn5 = assign58020_e90250_d_n5;
        locals.var_t3_dn6 = assign58020_e90250_d_n6;
        locals.var_t3_dn7 = assign58020_e90250_d_n7;
        locals.var_t3_dn8 = assign58020_e90250_d_n8;
        locals.var_t3_dn9 = assign58020_e90250_d_n9;
        locals.var_t3_dn10 = assign58020_e90250_d_n10;
        locals.var_t3_dn11 = assign58020_e90250_d_n11;
        locals.var_t3_dn14 = assign58020_e90250_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign58030_e90261, assign58030_e90261_d_n0, assign58030_e90261_d_n2, assign58030_e90261_d_n4, assign58030_e90261_d_n5, assign58030_e90261_d_n6, assign58030_e90261_d_n7, assign58030_e90261_d_n8, assign58030_e90261_d_n9, assign58030_e90261_d_n10, assign58030_e90261_d_n11, assign58030_e90261_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign58030_e90257: f64 = (locals.var_t1 / locals.var_t3);
        let assign58030_e90259: f64 = (assign58030_e90257 + locals.var_vbscl__blk437);
        (assign58030_e90259, ((((locals.var_t1_dn0 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk437_dn0), ((((locals.var_t1_dn2 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk437_dn2), ((((locals.var_t1_dn4 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn4)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk437_dn4), ((((locals.var_t1_dn5 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn5)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk437_dn5), ((((locals.var_t1_dn6 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk437_dn6), ((((locals.var_t1_dn7 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk437_dn7), ((((locals.var_t1_dn8 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn8)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk437_dn8), ((((locals.var_t1_dn9 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn9)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk437_dn9), ((((locals.var_t1_dn10 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk437_dn10), ((((locals.var_t1_dn11 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk437_dn11), ((((locals.var_t1_dn14 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn14)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk437_dn14),)
    } else {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn4, locals.var_ps0_dn5, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn8, locals.var_ps0_dn9, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn14,)
    }
};
        locals.var_ps0 = assign58030_e90261;
        locals.var_ps0_dn0 = assign58030_e90261_d_n0;
        locals.var_ps0_dn2 = assign58030_e90261_d_n2;
        locals.var_ps0_dn4 = assign58030_e90261_d_n4;
        locals.var_ps0_dn5 = assign58030_e90261_d_n5;
        locals.var_ps0_dn6 = assign58030_e90261_d_n6;
        locals.var_ps0_dn7 = assign58030_e90261_d_n7;
        locals.var_ps0_dn8 = assign58030_e90261_d_n8;
        locals.var_ps0_dn9 = assign58030_e90261_d_n9;
        locals.var_ps0_dn10 = assign58030_e90261_d_n10;
        locals.var_ps0_dn11 = assign58030_e90261_d_n11;
        locals.var_ps0_dn14 = assign58030_e90261_d_n14;
        locals.var_ps0_rv = 0.0;

        let (assign58040_e90268, assign58040_e90268_d_n0, assign58040_e90268_d_n2, assign58040_e90268_d_n4, assign58040_e90268_d_n5, assign58040_e90268_d_n6, assign58040_e90268_d_n7, assign58040_e90268_d_n8, assign58040_e90268_d_n9, assign58040_e90268_d_n10, assign58040_e90268_d_n11, assign58040_e90268_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1430 != 0.0)) {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn4, locals.var_ps0_dn5, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn8, locals.var_ps0_dn9, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn14,)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn4, locals.var_psl_dn5, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn8, locals.var_psl_dn9, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn14,)
    }
};
        locals.var_psl = assign58040_e90268;
        locals.var_psl_dn0 = assign58040_e90268_d_n0;
        locals.var_psl_dn2 = assign58040_e90268_d_n2;
        locals.var_psl_dn4 = assign58040_e90268_d_n4;
        locals.var_psl_dn5 = assign58040_e90268_d_n5;
        locals.var_psl_dn6 = assign58040_e90268_d_n6;
        locals.var_psl_dn7 = assign58040_e90268_d_n7;
        locals.var_psl_dn8 = assign58040_e90268_d_n8;
        locals.var_psl_dn9 = assign58040_e90268_d_n9;
        locals.var_psl_dn10 = assign58040_e90268_d_n10;
        locals.var_psl_dn11 = assign58040_e90268_d_n11;
        locals.var_psl_dn14 = assign58040_e90268_d_n14;
        locals.var_psl_rv = 0.0;

        let (assign58050_e90275, assign58050_e90275_d_n0, assign58050_e90275_d_n2, assign58050_e90275_d_n4, assign58050_e90275_d_n5, assign58050_e90275_d_n6, assign58050_e90275_d_n7, assign58050_e90275_d_n8, assign58050_e90275_d_n9, assign58050_e90275_d_n10, assign58050_e90275_d_n11, assign58050_e90275_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1430 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn4, locals.var_pds_dn5, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn8, locals.var_pds_dn9, locals.var_pds_dn10, locals.var_pds_dn11, locals.var_pds_dn14,)
    }
};
        locals.var_pds = assign58050_e90275;
        locals.var_pds_dn0 = assign58050_e90275_d_n0;
        locals.var_pds_dn2 = assign58050_e90275_d_n2;
        locals.var_pds_dn4 = assign58050_e90275_d_n4;
        locals.var_pds_dn5 = assign58050_e90275_d_n5;
        locals.var_pds_dn6 = assign58050_e90275_d_n6;
        locals.var_pds_dn7 = assign58050_e90275_d_n7;
        locals.var_pds_dn8 = assign58050_e90275_d_n8;
        locals.var_pds_dn9 = assign58050_e90275_d_n9;
        locals.var_pds_dn10 = assign58050_e90275_d_n10;
        locals.var_pds_dn11 = assign58050_e90275_d_n11;
        locals.var_pds_dn14 = assign58050_e90275_d_n14;
        locals.var_pds_rv = 0.0;

        let (assign58060_e90284, assign58060_e90284_d_n0, assign58060_e90284_d_n2, assign58060_e90284_d_n4, assign58060_e90284_d_n5, assign58060_e90284_d_n6, assign58060_e90284_d_n7, assign58060_e90284_d_n8, assign58060_e90284_d_n9, assign58060_e90284_d_n10, assign58060_e90284_d_n11, assign58060_e90284_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign58060_e90282: f64 = (locals.var_vgp - locals.var_ps0);
        (assign58060_e90282, (locals.var_vgp_dn0 - locals.var_ps0_dn0), (locals.var_vgp_dn2 - locals.var_ps0_dn2), (locals.var_vgp_dn4 - locals.var_ps0_dn4), (locals.var_vgp_dn5 - locals.var_ps0_dn5), (locals.var_vgp_dn6 - locals.var_ps0_dn6), (locals.var_vgp_dn7 - locals.var_ps0_dn7), (locals.var_vgp_dn8 - locals.var_ps0_dn8), (locals.var_vgp_dn9 - locals.var_ps0_dn9), (locals.var_vgp_dn10 - locals.var_ps0_dn10), (locals.var_vgp_dn11 - locals.var_ps0_dn11), (locals.var_vgp_dn14 - locals.var_ps0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign58060_e90284;
        locals.var_t2_dn0 = assign58060_e90284_d_n0;
        locals.var_t2_dn2 = assign58060_e90284_d_n2;
        locals.var_t2_dn4 = assign58060_e90284_d_n4;
        locals.var_t2_dn5 = assign58060_e90284_d_n5;
        locals.var_t2_dn6 = assign58060_e90284_d_n6;
        locals.var_t2_dn7 = assign58060_e90284_d_n7;
        locals.var_t2_dn8 = assign58060_e90284_d_n8;
        locals.var_t2_dn9 = assign58060_e90284_d_n9;
        locals.var_t2_dn10 = assign58060_e90284_d_n10;
        locals.var_t2_dn11 = assign58060_e90284_d_n11;
        locals.var_t2_dn14 = assign58060_e90284_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign58070_e90293, assign58070_e90293_d_n0, assign58070_e90293_d_n2, assign58070_e90293_d_n4, assign58070_e90293_d_n5, assign58070_e90293_d_n6, assign58070_e90293_d_n7, assign58070_e90293_d_n8, assign58070_e90293_d_n9, assign58070_e90293_d_n10, assign58070_e90293_d_n11, assign58070_e90293_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1430 != 0.0)) {
        let assign58070_e90291: f64 = (locals.var_cox * locals.var_t2);
        (assign58070_e90291, ((locals.var_cox_dn0 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn0)), ((locals.var_cox_dn2 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn2)), ((locals.var_cox_dn4 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn4)), ((locals.var_cox_dn5 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn5)), ((locals.var_cox_dn6 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn6)), ((locals.var_cox_dn7 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn7)), ((locals.var_cox_dn8 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn8)), ((locals.var_cox_dn9 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn9)), ((locals.var_cox_dn10 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn10)), ((locals.var_cox_dn11 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn11)), ((locals.var_cox_dn14 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn14)),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn4, locals.var_qbu_dn5, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn8, locals.var_qbu_dn9, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn14,)
    }
};
        locals.var_qbu = assign58070_e90293;
        locals.var_qbu_dn0 = assign58070_e90293_d_n0;
        locals.var_qbu_dn2 = assign58070_e90293_d_n2;
        locals.var_qbu_dn4 = assign58070_e90293_d_n4;
        locals.var_qbu_dn5 = assign58070_e90293_d_n5;
        locals.var_qbu_dn6 = assign58070_e90293_d_n6;
        locals.var_qbu_dn7 = assign58070_e90293_d_n7;
        locals.var_qbu_dn8 = assign58070_e90293_d_n8;
        locals.var_qbu_dn9 = assign58070_e90293_d_n9;
        locals.var_qbu_dn10 = assign58070_e90293_d_n10;
        locals.var_qbu_dn11 = assign58070_e90293_d_n11;
        locals.var_qbu_dn14 = assign58070_e90293_d_n14;
        locals.var_qbu_rv = 0.0;

        let (assign58080_e90300, assign58080_e90300_d_n0, assign58080_e90300_d_n2, assign58080_e90300_d_n4, assign58080_e90300_d_n5, assign58080_e90300_d_n6, assign58080_e90300_d_n7, assign58080_e90300_d_n8, assign58080_e90300_d_n9, assign58080_e90300_d_n10, assign58080_e90300_d_n11, assign58080_e90300_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1430 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn4, locals.var_qiu_dn5, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn8, locals.var_qiu_dn9, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn14,)
    }
};
        locals.var_qiu = assign58080_e90300;
        locals.var_qiu_dn0 = assign58080_e90300_d_n0;
        locals.var_qiu_dn2 = assign58080_e90300_d_n2;
        locals.var_qiu_dn4 = assign58080_e90300_d_n4;
        locals.var_qiu_dn5 = assign58080_e90300_d_n5;
        locals.var_qiu_dn6 = assign58080_e90300_d_n6;
        locals.var_qiu_dn7 = assign58080_e90300_d_n7;
        locals.var_qiu_dn8 = assign58080_e90300_d_n8;
        locals.var_qiu_dn9 = assign58080_e90300_d_n9;
        locals.var_qiu_dn10 = assign58080_e90300_d_n10;
        locals.var_qiu_dn11 = assign58080_e90300_d_n11;
        locals.var_qiu_dn14 = assign58080_e90300_d_n14;
        locals.var_qiu_rv = 0.0;

        let (assign58090_e90307, assign58090_e90307_d_n0, assign58090_e90307_d_n2, assign58090_e90307_d_n4, assign58090_e90307_d_n5, assign58090_e90307_d_n6, assign58090_e90307_d_n7, assign58090_e90307_d_n8, assign58090_e90307_d_n9, assign58090_e90307_d_n10, assign58090_e90307_d_n11, assign58090_e90307_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1430 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn4, locals.var_qdrat_dn5, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn8, locals.var_qdrat_dn9, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn14,)
    }
};
        locals.var_qdrat = assign58090_e90307;
        locals.var_qdrat_dn0 = assign58090_e90307_d_n0;
        locals.var_qdrat_dn2 = assign58090_e90307_d_n2;
        locals.var_qdrat_dn4 = assign58090_e90307_d_n4;
        locals.var_qdrat_dn5 = assign58090_e90307_d_n5;
        locals.var_qdrat_dn6 = assign58090_e90307_d_n6;
        locals.var_qdrat_dn7 = assign58090_e90307_d_n7;
        locals.var_qdrat_dn8 = assign58090_e90307_d_n8;
        locals.var_qdrat_dn9 = assign58090_e90307_d_n9;
        locals.var_qdrat_dn10 = assign58090_e90307_d_n10;
        locals.var_qdrat_dn11 = assign58090_e90307_d_n11;
        locals.var_qdrat_dn14 = assign58090_e90307_d_n14;
        locals.var_qdrat_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_213(
        locals: &mut StampLocals,
    ) {
        let (assign58100_e90314, assign58100_e90314_d_n0, assign58100_e90314_d_n2, assign58100_e90314_d_n4, assign58100_e90314_d_n5, assign58100_e90314_d_n6, assign58100_e90314_d_n7, assign58100_e90314_d_n8, assign58100_e90314_d_n9, assign58100_e90314_d_n10, assign58100_e90314_d_n11, assign58100_e90314_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1430 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn4, locals.var_lred_dn5, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn8, locals.var_lred_dn9, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn14,)
    }
};
        locals.var_lred = assign58100_e90314;
        locals.var_lred_dn0 = assign58100_e90314_d_n0;
        locals.var_lred_dn2 = assign58100_e90314_d_n2;
        locals.var_lred_dn4 = assign58100_e90314_d_n4;
        locals.var_lred_dn5 = assign58100_e90314_d_n5;
        locals.var_lred_dn6 = assign58100_e90314_d_n6;
        locals.var_lred_dn7 = assign58100_e90314_d_n7;
        locals.var_lred_dn8 = assign58100_e90314_d_n8;
        locals.var_lred_dn9 = assign58100_e90314_d_n9;
        locals.var_lred_dn10 = assign58100_e90314_d_n10;
        locals.var_lred_dn11 = assign58100_e90314_d_n11;
        locals.var_lred_dn14 = assign58100_e90314_d_n14;
        locals.var_lred_rv = 0.0;

        let (assign58110_e90321, assign58110_e90321_d_n0, assign58110_e90321_d_n2, assign58110_e90321_d_n4, assign58110_e90321_d_n5, assign58110_e90321_d_n6, assign58110_e90321_d_n7, assign58110_e90321_d_n8, assign58110_e90321_d_n9, assign58110_e90321_d_n10, assign58110_e90321_d_n11, assign58110_e90321_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1430 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn14,)
    }
};
        locals.var_ids = assign58110_e90321;
        locals.var_ids_dn0 = assign58110_e90321_d_n0;
        locals.var_ids_dn2 = assign58110_e90321_d_n2;
        locals.var_ids_dn4 = assign58110_e90321_d_n4;
        locals.var_ids_dn5 = assign58110_e90321_d_n5;
        locals.var_ids_dn6 = assign58110_e90321_d_n6;
        locals.var_ids_dn7 = assign58110_e90321_d_n7;
        locals.var_ids_dn8 = assign58110_e90321_d_n8;
        locals.var_ids_dn9 = assign58110_e90321_d_n9;
        locals.var_ids_dn10 = assign58110_e90321_d_n10;
        locals.var_ids_dn11 = assign58110_e90321_d_n11;
        locals.var_ids_dn14 = assign58110_e90321_d_n14;
        locals.var_ids_rv = 0.0;

        let (assign58120_e90328, assign58120_e90328_d_n0, assign58120_e90328_d_n2, assign58120_e90328_d_n4, assign58120_e90328_d_n5, assign58120_e90328_d_n6, assign58120_e90328_d_n7, assign58120_e90328_d_n8, assign58120_e90328_d_n9, assign58120_e90328_d_n10, assign58120_e90328_d_n11, assign58120_e90328_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1430 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgvt, locals.var_vgvt_dn0, locals.var_vgvt_dn2, locals.var_vgvt_dn4, locals.var_vgvt_dn5, locals.var_vgvt_dn6, locals.var_vgvt_dn7, locals.var_vgvt_dn8, locals.var_vgvt_dn9, locals.var_vgvt_dn10, locals.var_vgvt_dn11, locals.var_vgvt_dn14,)
    }
};
        locals.var_vgvt = assign58120_e90328;
        locals.var_vgvt_dn0 = assign58120_e90328_d_n0;
        locals.var_vgvt_dn2 = assign58120_e90328_d_n2;
        locals.var_vgvt_dn4 = assign58120_e90328_d_n4;
        locals.var_vgvt_dn5 = assign58120_e90328_d_n5;
        locals.var_vgvt_dn6 = assign58120_e90328_d_n6;
        locals.var_vgvt_dn7 = assign58120_e90328_d_n7;
        locals.var_vgvt_dn8 = assign58120_e90328_d_n8;
        locals.var_vgvt_dn9 = assign58120_e90328_d_n9;
        locals.var_vgvt_dn10 = assign58120_e90328_d_n10;
        locals.var_vgvt_dn11 = assign58120_e90328_d_n11;
        locals.var_vgvt_dn14 = assign58120_e90328_d_n14;
        locals.var_vgvt_rv = 0.0;

        let (assign58130_e90335,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1430 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_noqi,)
    }
};
        locals.var_flg_noqi = assign58130_e90335;
        locals.var_flg_noqi_rv = 0.0;

        let (assign58140_e90342,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1430 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_end_of_part_1,)
    }
};
        locals.var_end_of_part_1 = assign58140_e90342;
        locals.var_end_of_part_1_rv = 0.0;

        let assign58150_e90345: f64 = if locals.var_end_of_part_1 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1432 = assign58150_e90345;
        locals.var_guard1432_rv = 0.0;

        let (assign58160_e90366, assign58160_e90366_d_n0, assign58160_e90366_d_n2, assign58160_e90366_d_n4, assign58160_e90366_d_n5, assign58160_e90366_d_n6, assign58160_e90366_d_n7, assign58160_e90366_d_n8, assign58160_e90366_d_n9, assign58160_e90366_d_n10, assign58160_e90366_d_n11, assign58160_e90366_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign58160_e90355: f64 = (locals.var_vgp - locals.var_vbscl__blk437);
        let assign58160_e90356: f64 = (locals.var_beta * assign58160_e90355);
        let assign58160_e90358: f64 = (assign58160_e90356 - 1.0);
        let assign58160_e90359: f64 = (4.0 * assign58160_e90358);
        let assign58160_e90362: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign58160_e90363: f64 = (assign58160_e90359 / assign58160_e90362);
        let assign58160_e90364: f64 = (1.0 + assign58160_e90363);
        (assign58160_e90364, ((((4.0 * ((locals.var_beta_dn0 * assign58160_e90355) + (locals.var_beta * (locals.var_vgp_dn0 - locals.var_vbscl__blk437_dn0)))) * assign58160_e90362) - (assign58160_e90359 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign58160_e90362 * assign58160_e90362)), ((((4.0 * ((locals.var_beta_dn2 * assign58160_e90355) + (locals.var_beta * (locals.var_vgp_dn2 - locals.var_vbscl__blk437_dn2)))) * assign58160_e90362) - (assign58160_e90359 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign58160_e90362 * assign58160_e90362)), ((((4.0 * ((locals.var_beta_dn4 * assign58160_e90355) + (locals.var_beta * (locals.var_vgp_dn4 - locals.var_vbscl__blk437_dn4)))) * assign58160_e90362) - (assign58160_e90359 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign58160_e90362 * assign58160_e90362)), ((((4.0 * ((locals.var_beta_dn5 * assign58160_e90355) + (locals.var_beta * (locals.var_vgp_dn5 - locals.var_vbscl__blk437_dn5)))) * assign58160_e90362) - (assign58160_e90359 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign58160_e90362 * assign58160_e90362)), ((((4.0 * ((locals.var_beta_dn6 * assign58160_e90355) + (locals.var_beta * (locals.var_vgp_dn6 - locals.var_vbscl__blk437_dn6)))) * assign58160_e90362) - (assign58160_e90359 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign58160_e90362 * assign58160_e90362)), ((((4.0 * ((locals.var_beta_dn7 * assign58160_e90355) + (locals.var_beta * (locals.var_vgp_dn7 - locals.var_vbscl__blk437_dn7)))) * assign58160_e90362) - (assign58160_e90359 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign58160_e90362 * assign58160_e90362)), ((((4.0 * ((locals.var_beta_dn8 * assign58160_e90355) + (locals.var_beta * (locals.var_vgp_dn8 - locals.var_vbscl__blk437_dn8)))) * assign58160_e90362) - (assign58160_e90359 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign58160_e90362 * assign58160_e90362)), ((((4.0 * ((locals.var_beta_dn9 * assign58160_e90355) + (locals.var_beta * (locals.var_vgp_dn9 - locals.var_vbscl__blk437_dn9)))) * assign58160_e90362) - (assign58160_e90359 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign58160_e90362 * assign58160_e90362)), ((((4.0 * ((locals.var_beta_dn10 * assign58160_e90355) + (locals.var_beta * (locals.var_vgp_dn10 - locals.var_vbscl__blk437_dn10)))) * assign58160_e90362) - (assign58160_e90359 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign58160_e90362 * assign58160_e90362)), ((((4.0 * ((locals.var_beta_dn11 * assign58160_e90355) + (locals.var_beta * (locals.var_vgp_dn11 - locals.var_vbscl__blk437_dn11)))) * assign58160_e90362) - (assign58160_e90359 * ((locals.var_fac1p2_dn11 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn11)))) / (assign58160_e90362 * assign58160_e90362)), ((((4.0 * ((locals.var_beta_dn14 * assign58160_e90355) + (locals.var_beta * (locals.var_vgp_dn14 - locals.var_vbscl__blk437_dn14)))) * assign58160_e90362) - (assign58160_e90359 * ((locals.var_fac1p2_dn14 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn14)))) / (assign58160_e90362 * assign58160_e90362)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign58160_e90366;
        locals.var_tx_dn0 = assign58160_e90366_d_n0;
        locals.var_tx_dn2 = assign58160_e90366_d_n2;
        locals.var_tx_dn4 = assign58160_e90366_d_n4;
        locals.var_tx_dn5 = assign58160_e90366_d_n5;
        locals.var_tx_dn6 = assign58160_e90366_d_n6;
        locals.var_tx_dn7 = assign58160_e90366_d_n7;
        locals.var_tx_dn8 = assign58160_e90366_d_n8;
        locals.var_tx_dn9 = assign58160_e90366_d_n9;
        locals.var_tx_dn10 = assign58160_e90366_d_n10;
        locals.var_tx_dn11 = assign58160_e90366_d_n11;
        locals.var_tx_dn14 = assign58160_e90366_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign58170_e90382, assign58170_e90382_d_n0, assign58170_e90382_d_n2, assign58170_e90382_d_n4, assign58170_e90382_d_n5, assign58170_e90382_d_n6, assign58170_e90382_d_n7, assign58170_e90382_d_n8, assign58170_e90382_d_n9, assign58170_e90382_d_n10, assign58170_e90382_d_n11, assign58170_e90382_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign58170_e90374: f64 = (10.0 * 2.220446049250313e-16);
        let (assign58170_e90380, assign58170_e90380_d_n0, assign58170_e90380_d_n2, assign58170_e90380_d_n4, assign58170_e90380_d_n5, assign58170_e90380_d_n6, assign58170_e90380_d_n7, assign58170_e90380_d_n8, assign58170_e90380_d_n9, assign58170_e90380_d_n10, assign58170_e90380_d_n11, assign58170_e90380_d_n14,) = {
            if (locals.var_tx >= assign58170_e90374) {
                (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
            } else {
                let assign58170_e90379: f64 = (10.0 * 2.220446049250313e-16);
                (assign58170_e90379, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign58170_e90380, assign58170_e90380_d_n0, assign58170_e90380_d_n2, assign58170_e90380_d_n4, assign58170_e90380_d_n5, assign58170_e90380_d_n6, assign58170_e90380_d_n7, assign58170_e90380_d_n8, assign58170_e90380_d_n9, assign58170_e90380_d_n10, assign58170_e90380_d_n11, assign58170_e90380_d_n14,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign58170_e90382;
        locals.var_tx_dn0 = assign58170_e90382_d_n0;
        locals.var_tx_dn2 = assign58170_e90382_d_n2;
        locals.var_tx_dn4 = assign58170_e90382_d_n4;
        locals.var_tx_dn5 = assign58170_e90382_d_n5;
        locals.var_tx_dn6 = assign58170_e90382_d_n6;
        locals.var_tx_dn7 = assign58170_e90382_d_n7;
        locals.var_tx_dn8 = assign58170_e90382_d_n8;
        locals.var_tx_dn9 = assign58170_e90382_d_n9;
        locals.var_tx_dn10 = assign58170_e90382_d_n10;
        locals.var_tx_dn11 = assign58170_e90382_d_n11;
        locals.var_tx_dn14 = assign58170_e90382_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign58180_e90400, assign58180_e90400_d_n0, assign58180_e90400_d_n2, assign58180_e90400_d_n4, assign58180_e90400_d_n5, assign58180_e90400_d_n6, assign58180_e90400_d_n7, assign58180_e90400_d_n8, assign58180_e90400_d_n9, assign58180_e90400_d_n10, assign58180_e90400_d_n11, assign58180_e90400_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign58180_e90390: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign58180_e90392: f64 = (assign58180_e90390 * 0.5);
        let assign58180_e90395: f64 = (locals.var_tx).sqrt();
        let assign58180_e90396: f64 = (1.0 - assign58180_e90395);
        let assign58180_e90397: f64 = (assign58180_e90392 * assign58180_e90396);
        let assign58180_e90398: f64 = (locals.var_vgp + assign58180_e90397);
        (assign58180_e90398, (locals.var_vgp_dn0 + (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) * 0.5) * assign58180_e90396) + (assign58180_e90392 * (-(locals.var_tx_dn0 / (2.0 * assign58180_e90395)))))), (locals.var_vgp_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) * 0.5) * assign58180_e90396) + (assign58180_e90392 * (-(locals.var_tx_dn2 / (2.0 * assign58180_e90395)))))), (locals.var_vgp_dn4 + (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) * 0.5) * assign58180_e90396) + (assign58180_e90392 * (-(locals.var_tx_dn4 / (2.0 * assign58180_e90395)))))), (locals.var_vgp_dn5 + (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) * 0.5) * assign58180_e90396) + (assign58180_e90392 * (-(locals.var_tx_dn5 / (2.0 * assign58180_e90395)))))), (locals.var_vgp_dn6 + (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) * 0.5) * assign58180_e90396) + (assign58180_e90392 * (-(locals.var_tx_dn6 / (2.0 * assign58180_e90395)))))), (locals.var_vgp_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) * 0.5) * assign58180_e90396) + (assign58180_e90392 * (-(locals.var_tx_dn7 / (2.0 * assign58180_e90395)))))), (locals.var_vgp_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) * 0.5) * assign58180_e90396) + (assign58180_e90392 * (-(locals.var_tx_dn8 / (2.0 * assign58180_e90395)))))), (locals.var_vgp_dn9 + (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) * 0.5) * assign58180_e90396) + (assign58180_e90392 * (-(locals.var_tx_dn9 / (2.0 * assign58180_e90395)))))), (locals.var_vgp_dn10 + (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) * 0.5) * assign58180_e90396) + (assign58180_e90392 * (-(locals.var_tx_dn10 / (2.0 * assign58180_e90395)))))), (locals.var_vgp_dn11 + (((((locals.var_fac1p2_dn11 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn11)) * 0.5) * assign58180_e90396) + (assign58180_e90392 * (-(locals.var_tx_dn11 / (2.0 * assign58180_e90395)))))), (locals.var_vgp_dn14 + (((((locals.var_fac1p2_dn14 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn14)) * 0.5) * assign58180_e90396) + (assign58180_e90392 * (-(locals.var_tx_dn14 / (2.0 * assign58180_e90395)))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign58180_e90400;
        locals.var_ps0_inia_dn0 = assign58180_e90400_d_n0;
        locals.var_ps0_inia_dn2 = assign58180_e90400_d_n2;
        locals.var_ps0_inia_dn4 = assign58180_e90400_d_n4;
        locals.var_ps0_inia_dn5 = assign58180_e90400_d_n5;
        locals.var_ps0_inia_dn6 = assign58180_e90400_d_n6;
        locals.var_ps0_inia_dn7 = assign58180_e90400_d_n7;
        locals.var_ps0_inia_dn8 = assign58180_e90400_d_n8;
        locals.var_ps0_inia_dn9 = assign58180_e90400_d_n9;
        locals.var_ps0_inia_dn10 = assign58180_e90400_d_n10;
        locals.var_ps0_inia_dn11 = assign58180_e90400_d_n11;
        locals.var_ps0_inia_dn14 = assign58180_e90400_d_n14;
        locals.var_ps0_inia_rv = 0.0;

        let assign58190_e90403: f64 = if locals.var_flg_pprv == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1433 = assign58190_e90403;
        locals.var_guard1433_rv = 0.0;

        let (assign58200_e90416, assign58200_e90416_d_n0, assign58200_e90416_d_n2, assign58200_e90416_d_n4, assign58200_e90416_d_n5, assign58200_e90416_d_n6, assign58200_e90416_d_n7, assign58200_e90416_d_n8, assign58200_e90416_d_n9, assign58200_e90416_d_n10, assign58200_e90416_d_n11, assign58200_e90416_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1433 != 0.0)) {
        let assign58200_e90413: f64 = (locals.var_ps0_inia - locals.var_vbscl__blk437);
        let assign58200_e90414: f64 = (locals.var_beta * assign58200_e90413);
        (assign58200_e90414, ((locals.var_beta_dn0 * assign58200_e90413) + (locals.var_beta * (locals.var_ps0_inia_dn0 - locals.var_vbscl__blk437_dn0))), ((locals.var_beta_dn2 * assign58200_e90413) + (locals.var_beta * (locals.var_ps0_inia_dn2 - locals.var_vbscl__blk437_dn2))), ((locals.var_beta_dn4 * assign58200_e90413) + (locals.var_beta * (locals.var_ps0_inia_dn4 - locals.var_vbscl__blk437_dn4))), ((locals.var_beta_dn5 * assign58200_e90413) + (locals.var_beta * (locals.var_ps0_inia_dn5 - locals.var_vbscl__blk437_dn5))), ((locals.var_beta_dn6 * assign58200_e90413) + (locals.var_beta * (locals.var_ps0_inia_dn6 - locals.var_vbscl__blk437_dn6))), ((locals.var_beta_dn7 * assign58200_e90413) + (locals.var_beta * (locals.var_ps0_inia_dn7 - locals.var_vbscl__blk437_dn7))), ((locals.var_beta_dn8 * assign58200_e90413) + (locals.var_beta * (locals.var_ps0_inia_dn8 - locals.var_vbscl__blk437_dn8))), ((locals.var_beta_dn9 * assign58200_e90413) + (locals.var_beta * (locals.var_ps0_inia_dn9 - locals.var_vbscl__blk437_dn9))), ((locals.var_beta_dn10 * assign58200_e90413) + (locals.var_beta * (locals.var_ps0_inia_dn10 - locals.var_vbscl__blk437_dn10))), ((locals.var_beta_dn11 * assign58200_e90413) + (locals.var_beta * (locals.var_ps0_inia_dn11 - locals.var_vbscl__blk437_dn11))), ((locals.var_beta_dn14 * assign58200_e90413) + (locals.var_beta * (locals.var_ps0_inia_dn14 - locals.var_vbscl__blk437_dn14))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign58200_e90416;
        locals.var_chi_dn0 = assign58200_e90416_d_n0;
        locals.var_chi_dn2 = assign58200_e90416_d_n2;
        locals.var_chi_dn4 = assign58200_e90416_d_n4;
        locals.var_chi_dn5 = assign58200_e90416_d_n5;
        locals.var_chi_dn6 = assign58200_e90416_d_n6;
        locals.var_chi_dn7 = assign58200_e90416_d_n7;
        locals.var_chi_dn8 = assign58200_e90416_d_n8;
        locals.var_chi_dn9 = assign58200_e90416_d_n9;
        locals.var_chi_dn10 = assign58200_e90416_d_n10;
        locals.var_chi_dn11 = assign58200_e90416_d_n11;
        locals.var_chi_dn14 = assign58200_e90416_d_n14;
        locals.var_chi_rv = 0.0;

        let assign58210_e90419: f64 = if locals.var_chi < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1434 = assign58210_e90419;
        locals.var_guard1434_rv = 0.0;

        let (assign58220_e90434, assign58220_e90434_d_n0, assign58220_e90434_d_n2, assign58220_e90434_d_n4, assign58220_e90434_d_n5, assign58220_e90434_d_n6, assign58220_e90434_d_n7, assign58220_e90434_d_n8, assign58220_e90434_d_n9, assign58220_e90434_d_n10, assign58220_e90434_d_n11, assign58220_e90434_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1433 != 0.0)) && (locals.var_guard1434 != 0.0)) {
        let assign58220_e90431: f64 = (locals.var_vgp - locals.var_vbscl__blk437);
        let assign58220_e90432: f64 = (locals.var_beta * assign58220_e90431);
        (assign58220_e90432, ((locals.var_beta_dn0 * assign58220_e90431) + (locals.var_beta * (locals.var_vgp_dn0 - locals.var_vbscl__blk437_dn0))), ((locals.var_beta_dn2 * assign58220_e90431) + (locals.var_beta * (locals.var_vgp_dn2 - locals.var_vbscl__blk437_dn2))), ((locals.var_beta_dn4 * assign58220_e90431) + (locals.var_beta * (locals.var_vgp_dn4 - locals.var_vbscl__blk437_dn4))), ((locals.var_beta_dn5 * assign58220_e90431) + (locals.var_beta * (locals.var_vgp_dn5 - locals.var_vbscl__blk437_dn5))), ((locals.var_beta_dn6 * assign58220_e90431) + (locals.var_beta * (locals.var_vgp_dn6 - locals.var_vbscl__blk437_dn6))), ((locals.var_beta_dn7 * assign58220_e90431) + (locals.var_beta * (locals.var_vgp_dn7 - locals.var_vbscl__blk437_dn7))), ((locals.var_beta_dn8 * assign58220_e90431) + (locals.var_beta * (locals.var_vgp_dn8 - locals.var_vbscl__blk437_dn8))), ((locals.var_beta_dn9 * assign58220_e90431) + (locals.var_beta * (locals.var_vgp_dn9 - locals.var_vbscl__blk437_dn9))), ((locals.var_beta_dn10 * assign58220_e90431) + (locals.var_beta * (locals.var_vgp_dn10 - locals.var_vbscl__blk437_dn10))), ((locals.var_beta_dn11 * assign58220_e90431) + (locals.var_beta * (locals.var_vgp_dn11 - locals.var_vbscl__blk437_dn11))), ((locals.var_beta_dn14 * assign58220_e90431) + (locals.var_beta * (locals.var_vgp_dn14 - locals.var_vbscl__blk437_dn14))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign58220_e90434;
        locals.var_ty_dn0 = assign58220_e90434_d_n0;
        locals.var_ty_dn2 = assign58220_e90434_d_n2;
        locals.var_ty_dn4 = assign58220_e90434_d_n4;
        locals.var_ty_dn5 = assign58220_e90434_d_n5;
        locals.var_ty_dn6 = assign58220_e90434_d_n6;
        locals.var_ty_dn7 = assign58220_e90434_d_n7;
        locals.var_ty_dn8 = assign58220_e90434_d_n8;
        locals.var_ty_dn9 = assign58220_e90434_d_n9;
        locals.var_ty_dn10 = assign58220_e90434_d_n10;
        locals.var_ty_dn11 = assign58220_e90434_d_n11;
        locals.var_ty_dn14 = assign58220_e90434_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign58230_e90453, assign58230_e90453_d_n0, assign58230_e90453_d_n2, assign58230_e90453_d_n4, assign58230_e90453_d_n5, assign58230_e90453_d_n6, assign58230_e90453_d_n7, assign58230_e90453_d_n8, assign58230_e90453_d_n9, assign58230_e90453_d_n10, assign58230_e90453_d_n11, assign58230_e90453_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1433 != 0.0)) && (locals.var_guard1434 != 0.0)) {
        let assign58230_e90446: f64 = (1.414213562373095 / 108.0);
        let assign58230_e90448: f64 = (assign58230_e90446 * locals.var_beta);
        let assign58230_e90450: f64 = (assign58230_e90448 * locals.var_fac1);
        let assign58230_e90451: f64 = (1.0 / assign58230_e90450);
        (assign58230_e90451, (-((((assign58230_e90446 * locals.var_beta_dn0) * locals.var_fac1) + (assign58230_e90448 * locals.var_fac1_dn0)) / (assign58230_e90450 * assign58230_e90450))), (-((((assign58230_e90446 * locals.var_beta_dn2) * locals.var_fac1) + (assign58230_e90448 * locals.var_fac1_dn2)) / (assign58230_e90450 * assign58230_e90450))), (-((((assign58230_e90446 * locals.var_beta_dn4) * locals.var_fac1) + (assign58230_e90448 * locals.var_fac1_dn4)) / (assign58230_e90450 * assign58230_e90450))), (-((((assign58230_e90446 * locals.var_beta_dn5) * locals.var_fac1) + (assign58230_e90448 * locals.var_fac1_dn5)) / (assign58230_e90450 * assign58230_e90450))), (-((((assign58230_e90446 * locals.var_beta_dn6) * locals.var_fac1) + (assign58230_e90448 * locals.var_fac1_dn6)) / (assign58230_e90450 * assign58230_e90450))), (-((((assign58230_e90446 * locals.var_beta_dn7) * locals.var_fac1) + (assign58230_e90448 * locals.var_fac1_dn7)) / (assign58230_e90450 * assign58230_e90450))), (-((((assign58230_e90446 * locals.var_beta_dn8) * locals.var_fac1) + (assign58230_e90448 * locals.var_fac1_dn8)) / (assign58230_e90450 * assign58230_e90450))), (-((((assign58230_e90446 * locals.var_beta_dn9) * locals.var_fac1) + (assign58230_e90448 * locals.var_fac1_dn9)) / (assign58230_e90450 * assign58230_e90450))), (-((((assign58230_e90446 * locals.var_beta_dn10) * locals.var_fac1) + (assign58230_e90448 * locals.var_fac1_dn10)) / (assign58230_e90450 * assign58230_e90450))), (-((((assign58230_e90446 * locals.var_beta_dn11) * locals.var_fac1) + (assign58230_e90448 * locals.var_fac1_dn11)) / (assign58230_e90450 * assign58230_e90450))), (-((((assign58230_e90446 * locals.var_beta_dn14) * locals.var_fac1) + (assign58230_e90448 * locals.var_fac1_dn14)) / (assign58230_e90450 * assign58230_e90450))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign58230_e90453;
        locals.var_t1_dn0 = assign58230_e90453_d_n0;
        locals.var_t1_dn2 = assign58230_e90453_d_n2;
        locals.var_t1_dn4 = assign58230_e90453_d_n4;
        locals.var_t1_dn5 = assign58230_e90453_d_n5;
        locals.var_t1_dn6 = assign58230_e90453_d_n6;
        locals.var_t1_dn7 = assign58230_e90453_d_n7;
        locals.var_t1_dn8 = assign58230_e90453_d_n8;
        locals.var_t1_dn9 = assign58230_e90453_d_n9;
        locals.var_t1_dn10 = assign58230_e90453_d_n10;
        locals.var_t1_dn11 = assign58230_e90453_d_n11;
        locals.var_t1_dn14 = assign58230_e90453_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign58240_e90468, assign58240_e90468_d_n0, assign58240_e90468_d_n2, assign58240_e90468_d_n4, assign58240_e90468_d_n5, assign58240_e90468_d_n6, assign58240_e90468_d_n7, assign58240_e90468_d_n8, assign58240_e90468_d_n9, assign58240_e90468_d_n10, assign58240_e90468_d_n11, assign58240_e90468_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1433 != 0.0)) && (locals.var_guard1434 != 0.0)) {
        let assign58240_e90465: f64 = (3.0 * locals.var_t1);
        let assign58240_e90466: f64 = (81.0 + assign58240_e90465);
        (assign58240_e90466, (3.0 * locals.var_t1_dn0), (3.0 * locals.var_t1_dn2), (3.0 * locals.var_t1_dn4), (3.0 * locals.var_t1_dn5), (3.0 * locals.var_t1_dn6), (3.0 * locals.var_t1_dn7), (3.0 * locals.var_t1_dn8), (3.0 * locals.var_t1_dn9), (3.0 * locals.var_t1_dn10), (3.0 * locals.var_t1_dn11), (3.0 * locals.var_t1_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign58240_e90468;
        locals.var_t2_dn0 = assign58240_e90468_d_n0;
        locals.var_t2_dn2 = assign58240_e90468_d_n2;
        locals.var_t2_dn4 = assign58240_e90468_d_n4;
        locals.var_t2_dn5 = assign58240_e90468_d_n5;
        locals.var_t2_dn6 = assign58240_e90468_d_n6;
        locals.var_t2_dn7 = assign58240_e90468_d_n7;
        locals.var_t2_dn8 = assign58240_e90468_d_n8;
        locals.var_t2_dn9 = assign58240_e90468_d_n9;
        locals.var_t2_dn10 = assign58240_e90468_d_n10;
        locals.var_t2_dn11 = assign58240_e90468_d_n11;
        locals.var_t2_dn14 = assign58240_e90468_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign58250_e90490, assign58250_e90490_d_n0, assign58250_e90490_d_n2, assign58250_e90490_d_n4, assign58250_e90490_d_n5, assign58250_e90490_d_n6, assign58250_e90490_d_n7, assign58250_e90490_d_n8, assign58250_e90490_d_n9, assign58250_e90490_d_n10, assign58250_e90490_d_n11, assign58250_e90490_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1433 != 0.0)) && (locals.var_guard1434 != 0.0)) {
        let assign58250_e90478: f64 = (-2916.0);
        let assign58250_e90481: f64 = (81.0 * locals.var_t1);
        let assign58250_e90482: f64 = (assign58250_e90478 - assign58250_e90481);
        let assign58250_e90485: f64 = (27.0 * locals.var_t1);
        let assign58250_e90487: f64 = (assign58250_e90485 * locals.var_ty);
        let assign58250_e90488: f64 = (assign58250_e90482 + assign58250_e90487);
        (assign58250_e90488, ((-(81.0 * locals.var_t1_dn0)) + (((27.0 * locals.var_t1_dn0) * locals.var_ty) + (assign58250_e90485 * locals.var_ty_dn0))), ((-(81.0 * locals.var_t1_dn2)) + (((27.0 * locals.var_t1_dn2) * locals.var_ty) + (assign58250_e90485 * locals.var_ty_dn2))), ((-(81.0 * locals.var_t1_dn4)) + (((27.0 * locals.var_t1_dn4) * locals.var_ty) + (assign58250_e90485 * locals.var_ty_dn4))), ((-(81.0 * locals.var_t1_dn5)) + (((27.0 * locals.var_t1_dn5) * locals.var_ty) + (assign58250_e90485 * locals.var_ty_dn5))), ((-(81.0 * locals.var_t1_dn6)) + (((27.0 * locals.var_t1_dn6) * locals.var_ty) + (assign58250_e90485 * locals.var_ty_dn6))), ((-(81.0 * locals.var_t1_dn7)) + (((27.0 * locals.var_t1_dn7) * locals.var_ty) + (assign58250_e90485 * locals.var_ty_dn7))), ((-(81.0 * locals.var_t1_dn8)) + (((27.0 * locals.var_t1_dn8) * locals.var_ty) + (assign58250_e90485 * locals.var_ty_dn8))), ((-(81.0 * locals.var_t1_dn9)) + (((27.0 * locals.var_t1_dn9) * locals.var_ty) + (assign58250_e90485 * locals.var_ty_dn9))), ((-(81.0 * locals.var_t1_dn10)) + (((27.0 * locals.var_t1_dn10) * locals.var_ty) + (assign58250_e90485 * locals.var_ty_dn10))), ((-(81.0 * locals.var_t1_dn11)) + (((27.0 * locals.var_t1_dn11) * locals.var_ty) + (assign58250_e90485 * locals.var_ty_dn11))), ((-(81.0 * locals.var_t1_dn14)) + (((27.0 * locals.var_t1_dn14) * locals.var_ty) + (assign58250_e90485 * locals.var_ty_dn14))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign58250_e90490;
        locals.var_t3_dn0 = assign58250_e90490_d_n0;
        locals.var_t3_dn2 = assign58250_e90490_d_n2;
        locals.var_t3_dn4 = assign58250_e90490_d_n4;
        locals.var_t3_dn5 = assign58250_e90490_d_n5;
        locals.var_t3_dn6 = assign58250_e90490_d_n6;
        locals.var_t3_dn7 = assign58250_e90490_d_n7;
        locals.var_t3_dn8 = assign58250_e90490_d_n8;
        locals.var_t3_dn9 = assign58250_e90490_d_n9;
        locals.var_t3_dn10 = assign58250_e90490_d_n10;
        locals.var_t3_dn11 = assign58250_e90490_d_n11;
        locals.var_t3_dn14 = assign58250_e90490_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign58260_e90513, assign58260_e90513_d_n0, assign58260_e90513_d_n2, assign58260_e90513_d_n4, assign58260_e90513_d_n5, assign58260_e90513_d_n6, assign58260_e90513_d_n7, assign58260_e90513_d_n8, assign58260_e90513_d_n9, assign58260_e90513_d_n10, assign58260_e90513_d_n11, assign58260_e90513_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1433 != 0.0)) && (locals.var_guard1434 != 0.0)) {
        let assign58260_e90503: f64 = (54.0 + locals.var_t1);
        let assign58260_e90504: f64 = (81.0 * assign58260_e90503);
        let assign58260_e90505: f64 = (1458.0 - assign58260_e90504);
        let assign58260_e90508: f64 = (27.0 * locals.var_t1);
        let assign58260_e90510: f64 = (assign58260_e90508 * locals.var_ty);
        let assign58260_e90511: f64 = (assign58260_e90505 + assign58260_e90510);
        (assign58260_e90511, ((-(81.0 * locals.var_t1_dn0)) + (((27.0 * locals.var_t1_dn0) * locals.var_ty) + (assign58260_e90508 * locals.var_ty_dn0))), ((-(81.0 * locals.var_t1_dn2)) + (((27.0 * locals.var_t1_dn2) * locals.var_ty) + (assign58260_e90508 * locals.var_ty_dn2))), ((-(81.0 * locals.var_t1_dn4)) + (((27.0 * locals.var_t1_dn4) * locals.var_ty) + (assign58260_e90508 * locals.var_ty_dn4))), ((-(81.0 * locals.var_t1_dn5)) + (((27.0 * locals.var_t1_dn5) * locals.var_ty) + (assign58260_e90508 * locals.var_ty_dn5))), ((-(81.0 * locals.var_t1_dn6)) + (((27.0 * locals.var_t1_dn6) * locals.var_ty) + (assign58260_e90508 * locals.var_ty_dn6))), ((-(81.0 * locals.var_t1_dn7)) + (((27.0 * locals.var_t1_dn7) * locals.var_ty) + (assign58260_e90508 * locals.var_ty_dn7))), ((-(81.0 * locals.var_t1_dn8)) + (((27.0 * locals.var_t1_dn8) * locals.var_ty) + (assign58260_e90508 * locals.var_ty_dn8))), ((-(81.0 * locals.var_t1_dn9)) + (((27.0 * locals.var_t1_dn9) * locals.var_ty) + (assign58260_e90508 * locals.var_ty_dn9))), ((-(81.0 * locals.var_t1_dn10)) + (((27.0 * locals.var_t1_dn10) * locals.var_ty) + (assign58260_e90508 * locals.var_ty_dn10))), ((-(81.0 * locals.var_t1_dn11)) + (((27.0 * locals.var_t1_dn11) * locals.var_ty) + (assign58260_e90508 * locals.var_ty_dn11))), ((-(81.0 * locals.var_t1_dn14)) + (((27.0 * locals.var_t1_dn14) * locals.var_ty) + (assign58260_e90508 * locals.var_ty_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign58260_e90513;
        locals.var_t4_dn0 = assign58260_e90513_d_n0;
        locals.var_t4_dn2 = assign58260_e90513_d_n2;
        locals.var_t4_dn4 = assign58260_e90513_d_n4;
        locals.var_t4_dn5 = assign58260_e90513_d_n5;
        locals.var_t4_dn6 = assign58260_e90513_d_n6;
        locals.var_t4_dn7 = assign58260_e90513_d_n7;
        locals.var_t4_dn8 = assign58260_e90513_d_n8;
        locals.var_t4_dn9 = assign58260_e90513_d_n9;
        locals.var_t4_dn10 = assign58260_e90513_d_n10;
        locals.var_t4_dn11 = assign58260_e90513_d_n11;
        locals.var_t4_dn14 = assign58260_e90513_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign58270_e90526, assign58270_e90526_d_n0, assign58270_e90526_d_n2, assign58270_e90526_d_n4, assign58270_e90526_d_n5, assign58270_e90526_d_n6, assign58270_e90526_d_n7, assign58270_e90526_d_n8, assign58270_e90526_d_n9, assign58270_e90526_d_n10, assign58270_e90526_d_n11, assign58270_e90526_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1433 != 0.0)) && (locals.var_guard1434 != 0.0)) {
        let assign58270_e90524: f64 = (locals.var_t4 * locals.var_t4);
        (assign58270_e90524, ((locals.var_t4_dn0 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn0)), ((locals.var_t4_dn2 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn2)), ((locals.var_t4_dn4 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn4)), ((locals.var_t4_dn5 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn5)), ((locals.var_t4_dn6 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn6)), ((locals.var_t4_dn7 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn7)), ((locals.var_t4_dn8 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn8)), ((locals.var_t4_dn9 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn9)), ((locals.var_t4_dn10 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn10)), ((locals.var_t4_dn11 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn11)), ((locals.var_t4_dn14 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign58270_e90526;
        locals.var_t4_dn0 = assign58270_e90526_d_n0;
        locals.var_t4_dn2 = assign58270_e90526_d_n2;
        locals.var_t4_dn4 = assign58270_e90526_d_n4;
        locals.var_t4_dn5 = assign58270_e90526_d_n5;
        locals.var_t4_dn6 = assign58270_e90526_d_n6;
        locals.var_t4_dn7 = assign58270_e90526_d_n7;
        locals.var_t4_dn8 = assign58270_e90526_d_n8;
        locals.var_t4_dn9 = assign58270_e90526_d_n9;
        locals.var_t4_dn10 = assign58270_e90526_d_n10;
        locals.var_t4_dn11 = assign58270_e90526_d_n11;
        locals.var_t4_dn14 = assign58270_e90526_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign58280_e90566, assign58280_e90566_d_n0, assign58280_e90566_d_n2, assign58280_e90566_d_n4, assign58280_e90566_d_n5, assign58280_e90566_d_n6, assign58280_e90566_d_n7, assign58280_e90566_d_n8, assign58280_e90566_d_n9, assign58280_e90566_d_n10, assign58280_e90566_d_n11, assign58280_e90566_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1433 != 0.0)) && (locals.var_guard1434 != 0.0)) {
        let assign58280_e90538: f64 = (4.0 * locals.var_t2);
        let assign58280_e90540: f64 = (assign58280_e90538 * locals.var_t2);
        let assign58280_e90542: f64 = (assign58280_e90540 * locals.var_t2);
        let assign58280_e90544: f64 = (assign58280_e90542 + locals.var_t4);
        let assign58280_e90545: f64 = (assign58280_e90544).sqrt();
        let assign58280_e90546: f64 = (locals.var_t3 + assign58280_e90545);
        let (assign58280_e90564, assign58280_e90564_d_n0, assign58280_e90564_d_n2, assign58280_e90564_d_n4, assign58280_e90564_d_n5, assign58280_e90564_d_n6, assign58280_e90564_d_n7, assign58280_e90564_d_n8, assign58280_e90564_d_n9, assign58280_e90564_d_n10, assign58280_e90564_d_n11, assign58280_e90564_d_n14,) = {
            if (assign58280_e90546 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign58280_e90553: f64 = (4.0 * locals.var_t2);
                let assign58280_e90555: f64 = (assign58280_e90553 * locals.var_t2);
                let assign58280_e90557: f64 = (assign58280_e90555 * locals.var_t2);
                let assign58280_e90559: f64 = (assign58280_e90557 + locals.var_t4);
                let assign58280_e90560: f64 = (assign58280_e90559).sqrt();
                let assign58280_e90561: f64 = (locals.var_t3 + assign58280_e90560);
                let assign58280_e90563: f64 = (assign58280_e90561).powf(0.3333333333333333);
                (assign58280_e90563, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign58280_e90561).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn0 + (((((((4.0 * locals.var_t2_dn0) * locals.var_t2) + (assign58280_e90553 * locals.var_t2_dn0)) * locals.var_t2) + (assign58280_e90555 * locals.var_t2_dn0)) + locals.var_t4_dn0) / (2.0 * assign58280_e90560))))) } } else { (assign58280_e90563 * (0.3333333333333333 * ((locals.var_t3_dn0 + (((((((4.0 * locals.var_t2_dn0) * locals.var_t2) + (assign58280_e90553 * locals.var_t2_dn0)) * locals.var_t2) + (assign58280_e90555 * locals.var_t2_dn0)) + locals.var_t4_dn0) / (2.0 * assign58280_e90560))) / assign58280_e90561))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign58280_e90561).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn2 + (((((((4.0 * locals.var_t2_dn2) * locals.var_t2) + (assign58280_e90553 * locals.var_t2_dn2)) * locals.var_t2) + (assign58280_e90555 * locals.var_t2_dn2)) + locals.var_t4_dn2) / (2.0 * assign58280_e90560))))) } } else { (assign58280_e90563 * (0.3333333333333333 * ((locals.var_t3_dn2 + (((((((4.0 * locals.var_t2_dn2) * locals.var_t2) + (assign58280_e90553 * locals.var_t2_dn2)) * locals.var_t2) + (assign58280_e90555 * locals.var_t2_dn2)) + locals.var_t4_dn2) / (2.0 * assign58280_e90560))) / assign58280_e90561))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign58280_e90561).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn4 + (((((((4.0 * locals.var_t2_dn4) * locals.var_t2) + (assign58280_e90553 * locals.var_t2_dn4)) * locals.var_t2) + (assign58280_e90555 * locals.var_t2_dn4)) + locals.var_t4_dn4) / (2.0 * assign58280_e90560))))) } } else { (assign58280_e90563 * (0.3333333333333333 * ((locals.var_t3_dn4 + (((((((4.0 * locals.var_t2_dn4) * locals.var_t2) + (assign58280_e90553 * locals.var_t2_dn4)) * locals.var_t2) + (assign58280_e90555 * locals.var_t2_dn4)) + locals.var_t4_dn4) / (2.0 * assign58280_e90560))) / assign58280_e90561))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign58280_e90561).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn5 + (((((((4.0 * locals.var_t2_dn5) * locals.var_t2) + (assign58280_e90553 * locals.var_t2_dn5)) * locals.var_t2) + (assign58280_e90555 * locals.var_t2_dn5)) + locals.var_t4_dn5) / (2.0 * assign58280_e90560))))) } } else { (assign58280_e90563 * (0.3333333333333333 * ((locals.var_t3_dn5 + (((((((4.0 * locals.var_t2_dn5) * locals.var_t2) + (assign58280_e90553 * locals.var_t2_dn5)) * locals.var_t2) + (assign58280_e90555 * locals.var_t2_dn5)) + locals.var_t4_dn5) / (2.0 * assign58280_e90560))) / assign58280_e90561))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign58280_e90561).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn6 + (((((((4.0 * locals.var_t2_dn6) * locals.var_t2) + (assign58280_e90553 * locals.var_t2_dn6)) * locals.var_t2) + (assign58280_e90555 * locals.var_t2_dn6)) + locals.var_t4_dn6) / (2.0 * assign58280_e90560))))) } } else { (assign58280_e90563 * (0.3333333333333333 * ((locals.var_t3_dn6 + (((((((4.0 * locals.var_t2_dn6) * locals.var_t2) + (assign58280_e90553 * locals.var_t2_dn6)) * locals.var_t2) + (assign58280_e90555 * locals.var_t2_dn6)) + locals.var_t4_dn6) / (2.0 * assign58280_e90560))) / assign58280_e90561))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign58280_e90561).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn7 + (((((((4.0 * locals.var_t2_dn7) * locals.var_t2) + (assign58280_e90553 * locals.var_t2_dn7)) * locals.var_t2) + (assign58280_e90555 * locals.var_t2_dn7)) + locals.var_t4_dn7) / (2.0 * assign58280_e90560))))) } } else { (assign58280_e90563 * (0.3333333333333333 * ((locals.var_t3_dn7 + (((((((4.0 * locals.var_t2_dn7) * locals.var_t2) + (assign58280_e90553 * locals.var_t2_dn7)) * locals.var_t2) + (assign58280_e90555 * locals.var_t2_dn7)) + locals.var_t4_dn7) / (2.0 * assign58280_e90560))) / assign58280_e90561))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign58280_e90561).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn8 + (((((((4.0 * locals.var_t2_dn8) * locals.var_t2) + (assign58280_e90553 * locals.var_t2_dn8)) * locals.var_t2) + (assign58280_e90555 * locals.var_t2_dn8)) + locals.var_t4_dn8) / (2.0 * assign58280_e90560))))) } } else { (assign58280_e90563 * (0.3333333333333333 * ((locals.var_t3_dn8 + (((((((4.0 * locals.var_t2_dn8) * locals.var_t2) + (assign58280_e90553 * locals.var_t2_dn8)) * locals.var_t2) + (assign58280_e90555 * locals.var_t2_dn8)) + locals.var_t4_dn8) / (2.0 * assign58280_e90560))) / assign58280_e90561))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign58280_e90561).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn9 + (((((((4.0 * locals.var_t2_dn9) * locals.var_t2) + (assign58280_e90553 * locals.var_t2_dn9)) * locals.var_t2) + (assign58280_e90555 * locals.var_t2_dn9)) + locals.var_t4_dn9) / (2.0 * assign58280_e90560))))) } } else { (assign58280_e90563 * (0.3333333333333333 * ((locals.var_t3_dn9 + (((((((4.0 * locals.var_t2_dn9) * locals.var_t2) + (assign58280_e90553 * locals.var_t2_dn9)) * locals.var_t2) + (assign58280_e90555 * locals.var_t2_dn9)) + locals.var_t4_dn9) / (2.0 * assign58280_e90560))) / assign58280_e90561))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign58280_e90561).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn10 + (((((((4.0 * locals.var_t2_dn10) * locals.var_t2) + (assign58280_e90553 * locals.var_t2_dn10)) * locals.var_t2) + (assign58280_e90555 * locals.var_t2_dn10)) + locals.var_t4_dn10) / (2.0 * assign58280_e90560))))) } } else { (assign58280_e90563 * (0.3333333333333333 * ((locals.var_t3_dn10 + (((((((4.0 * locals.var_t2_dn10) * locals.var_t2) + (assign58280_e90553 * locals.var_t2_dn10)) * locals.var_t2) + (assign58280_e90555 * locals.var_t2_dn10)) + locals.var_t4_dn10) / (2.0 * assign58280_e90560))) / assign58280_e90561))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign58280_e90561).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn11 + (((((((4.0 * locals.var_t2_dn11) * locals.var_t2) + (assign58280_e90553 * locals.var_t2_dn11)) * locals.var_t2) + (assign58280_e90555 * locals.var_t2_dn11)) + locals.var_t4_dn11) / (2.0 * assign58280_e90560))))) } } else { (assign58280_e90563 * (0.3333333333333333 * ((locals.var_t3_dn11 + (((((((4.0 * locals.var_t2_dn11) * locals.var_t2) + (assign58280_e90553 * locals.var_t2_dn11)) * locals.var_t2) + (assign58280_e90555 * locals.var_t2_dn11)) + locals.var_t4_dn11) / (2.0 * assign58280_e90560))) / assign58280_e90561))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign58280_e90561).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn14 + (((((((4.0 * locals.var_t2_dn14) * locals.var_t2) + (assign58280_e90553 * locals.var_t2_dn14)) * locals.var_t2) + (assign58280_e90555 * locals.var_t2_dn14)) + locals.var_t4_dn14) / (2.0 * assign58280_e90560))))) } } else { (assign58280_e90563 * (0.3333333333333333 * ((locals.var_t3_dn14 + (((((((4.0 * locals.var_t2_dn14) * locals.var_t2) + (assign58280_e90553 * locals.var_t2_dn14)) * locals.var_t2) + (assign58280_e90555 * locals.var_t2_dn14)) + locals.var_t4_dn14) / (2.0 * assign58280_e90560))) / assign58280_e90561))) },)
            }
        };
        (assign58280_e90564, assign58280_e90564_d_n0, assign58280_e90564_d_n2, assign58280_e90564_d_n4, assign58280_e90564_d_n5, assign58280_e90564_d_n6, assign58280_e90564_d_n7, assign58280_e90564_d_n8, assign58280_e90564_d_n9, assign58280_e90564_d_n10, assign58280_e90564_d_n11, assign58280_e90564_d_n14,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign58280_e90566;
        locals.var_t5_dn0 = assign58280_e90566_d_n0;
        locals.var_t5_dn2 = assign58280_e90566_d_n2;
        locals.var_t5_dn4 = assign58280_e90566_d_n4;
        locals.var_t5_dn5 = assign58280_e90566_d_n5;
        locals.var_t5_dn6 = assign58280_e90566_d_n6;
        locals.var_t5_dn7 = assign58280_e90566_d_n7;
        locals.var_t5_dn8 = assign58280_e90566_d_n8;
        locals.var_t5_dn9 = assign58280_e90566_d_n9;
        locals.var_t5_dn10 = assign58280_e90566_d_n10;
        locals.var_t5_dn11 = assign58280_e90566_d_n11;
        locals.var_t5_dn14 = assign58280_e90566_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign58290_e90593, assign58290_e90593_d_n0, assign58290_e90593_d_n2, assign58290_e90593_d_n4, assign58290_e90593_d_n5, assign58290_e90593_d_n6, assign58290_e90593_d_n7, assign58290_e90593_d_n8, assign58290_e90593_d_n9, assign58290_e90593_d_n10, assign58290_e90593_d_n11, assign58290_e90593_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1433 != 0.0)) && (locals.var_guard1434 != 0.0)) {
        let assign58290_e90578: f64 = (1.259921049894873 * locals.var_t2);
        let assign58290_e90581: f64 = (3.0 * locals.var_t5);
        let assign58290_e90582: f64 = (assign58290_e90578 / assign58290_e90581);
        let assign58290_e90583: f64 = (3.0 - assign58290_e90582);
        let assign58290_e90587: f64 = (3.0 * 1.259921049894873);
        let assign58290_e90588: f64 = (1.0 / assign58290_e90587);
        let assign58290_e90590: f64 = (assign58290_e90588 * locals.var_t5);
        let assign58290_e90591: f64 = (assign58290_e90583 + assign58290_e90590);
        (assign58290_e90591, ((-((((1.259921049894873 * locals.var_t2_dn0) * assign58290_e90581) - (assign58290_e90578 * (3.0 * locals.var_t5_dn0))) / (assign58290_e90581 * assign58290_e90581))) + (assign58290_e90588 * locals.var_t5_dn0)), ((-((((1.259921049894873 * locals.var_t2_dn2) * assign58290_e90581) - (assign58290_e90578 * (3.0 * locals.var_t5_dn2))) / (assign58290_e90581 * assign58290_e90581))) + (assign58290_e90588 * locals.var_t5_dn2)), ((-((((1.259921049894873 * locals.var_t2_dn4) * assign58290_e90581) - (assign58290_e90578 * (3.0 * locals.var_t5_dn4))) / (assign58290_e90581 * assign58290_e90581))) + (assign58290_e90588 * locals.var_t5_dn4)), ((-((((1.259921049894873 * locals.var_t2_dn5) * assign58290_e90581) - (assign58290_e90578 * (3.0 * locals.var_t5_dn5))) / (assign58290_e90581 * assign58290_e90581))) + (assign58290_e90588 * locals.var_t5_dn5)), ((-((((1.259921049894873 * locals.var_t2_dn6) * assign58290_e90581) - (assign58290_e90578 * (3.0 * locals.var_t5_dn6))) / (assign58290_e90581 * assign58290_e90581))) + (assign58290_e90588 * locals.var_t5_dn6)), ((-((((1.259921049894873 * locals.var_t2_dn7) * assign58290_e90581) - (assign58290_e90578 * (3.0 * locals.var_t5_dn7))) / (assign58290_e90581 * assign58290_e90581))) + (assign58290_e90588 * locals.var_t5_dn7)), ((-((((1.259921049894873 * locals.var_t2_dn8) * assign58290_e90581) - (assign58290_e90578 * (3.0 * locals.var_t5_dn8))) / (assign58290_e90581 * assign58290_e90581))) + (assign58290_e90588 * locals.var_t5_dn8)), ((-((((1.259921049894873 * locals.var_t2_dn9) * assign58290_e90581) - (assign58290_e90578 * (3.0 * locals.var_t5_dn9))) / (assign58290_e90581 * assign58290_e90581))) + (assign58290_e90588 * locals.var_t5_dn9)), ((-((((1.259921049894873 * locals.var_t2_dn10) * assign58290_e90581) - (assign58290_e90578 * (3.0 * locals.var_t5_dn10))) / (assign58290_e90581 * assign58290_e90581))) + (assign58290_e90588 * locals.var_t5_dn10)), ((-((((1.259921049894873 * locals.var_t2_dn11) * assign58290_e90581) - (assign58290_e90578 * (3.0 * locals.var_t5_dn11))) / (assign58290_e90581 * assign58290_e90581))) + (assign58290_e90588 * locals.var_t5_dn11)), ((-((((1.259921049894873 * locals.var_t2_dn14) * assign58290_e90581) - (assign58290_e90578 * (3.0 * locals.var_t5_dn14))) / (assign58290_e90581 * assign58290_e90581))) + (assign58290_e90588 * locals.var_t5_dn14)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign58290_e90593;
        locals.var_tx_dn0 = assign58290_e90593_d_n0;
        locals.var_tx_dn2 = assign58290_e90593_d_n2;
        locals.var_tx_dn4 = assign58290_e90593_d_n4;
        locals.var_tx_dn5 = assign58290_e90593_d_n5;
        locals.var_tx_dn6 = assign58290_e90593_d_n6;
        locals.var_tx_dn7 = assign58290_e90593_d_n7;
        locals.var_tx_dn8 = assign58290_e90593_d_n8;
        locals.var_tx_dn9 = assign58290_e90593_d_n9;
        locals.var_tx_dn10 = assign58290_e90593_d_n10;
        locals.var_tx_dn11 = assign58290_e90593_d_n11;
        locals.var_tx_dn14 = assign58290_e90593_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign58300_e90608, assign58300_e90608_d_n0, assign58300_e90608_d_n2, assign58300_e90608_d_n4, assign58300_e90608_d_n5, assign58300_e90608_d_n6, assign58300_e90608_d_n7, assign58300_e90608_d_n8, assign58300_e90608_d_n9, assign58300_e90608_d_n10, assign58300_e90608_d_n11, assign58300_e90608_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1433 != 0.0)) && (locals.var_guard1434 != 0.0)) {
        let assign58300_e90604: f64 = (locals.var_tx * locals.var_beta_inv);
        let assign58300_e90606: f64 = (assign58300_e90604 + locals.var_vbscl__blk437);
        (assign58300_e90606, (((locals.var_tx_dn0 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn0)) + locals.var_vbscl__blk437_dn0), (((locals.var_tx_dn2 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn2)) + locals.var_vbscl__blk437_dn2), (((locals.var_tx_dn4 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn4)) + locals.var_vbscl__blk437_dn4), (((locals.var_tx_dn5 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn5)) + locals.var_vbscl__blk437_dn5), (((locals.var_tx_dn6 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn6)) + locals.var_vbscl__blk437_dn6), (((locals.var_tx_dn7 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn7)) + locals.var_vbscl__blk437_dn7), (((locals.var_tx_dn8 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn8)) + locals.var_vbscl__blk437_dn8), (((locals.var_tx_dn9 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn9)) + locals.var_vbscl__blk437_dn9), (((locals.var_tx_dn10 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn10)) + locals.var_vbscl__blk437_dn10), (((locals.var_tx_dn11 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn11)) + locals.var_vbscl__blk437_dn11), (((locals.var_tx_dn14 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn14)) + locals.var_vbscl__blk437_dn14),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign58300_e90608;
        locals.var_ps0_inia_dn0 = assign58300_e90608_d_n0;
        locals.var_ps0_inia_dn2 = assign58300_e90608_d_n2;
        locals.var_ps0_inia_dn4 = assign58300_e90608_d_n4;
        locals.var_ps0_inia_dn5 = assign58300_e90608_d_n5;
        locals.var_ps0_inia_dn6 = assign58300_e90608_d_n6;
        locals.var_ps0_inia_dn7 = assign58300_e90608_d_n7;
        locals.var_ps0_inia_dn8 = assign58300_e90608_d_n8;
        locals.var_ps0_inia_dn9 = assign58300_e90608_d_n9;
        locals.var_ps0_inia_dn10 = assign58300_e90608_d_n10;
        locals.var_ps0_inia_dn11 = assign58300_e90608_d_n11;
        locals.var_ps0_inia_dn14 = assign58300_e90608_d_n14;
        locals.var_ps0_inia_rv = 0.0;

        let (assign58310_e90619, assign58310_e90619_d_n0, assign58310_e90619_d_n2, assign58310_e90619_d_n4, assign58310_e90619_d_n5, assign58310_e90619_d_n6, assign58310_e90619_d_n7, assign58310_e90619_d_n8, assign58310_e90619_d_n9, assign58310_e90619_d_n10, assign58310_e90619_d_n11, assign58310_e90619_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1433 != 0.0)) && (locals.var_guard1434 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn14,)
    }
};
        locals.var_ps0_ini = assign58310_e90619;
        locals.var_ps0_ini_dn0 = assign58310_e90619_d_n0;
        locals.var_ps0_ini_dn2 = assign58310_e90619_d_n2;
        locals.var_ps0_ini_dn4 = assign58310_e90619_d_n4;
        locals.var_ps0_ini_dn5 = assign58310_e90619_d_n5;
        locals.var_ps0_ini_dn6 = assign58310_e90619_d_n6;
        locals.var_ps0_ini_dn7 = assign58310_e90619_d_n7;
        locals.var_ps0_ini_dn8 = assign58310_e90619_d_n8;
        locals.var_ps0_ini_dn9 = assign58310_e90619_d_n9;
        locals.var_ps0_ini_dn10 = assign58310_e90619_d_n10;
        locals.var_ps0_ini_dn11 = assign58310_e90619_d_n11;
        locals.var_ps0_ini_dn14 = assign58310_e90619_d_n14;
        locals.var_ps0_ini_rv = 0.0;

        let assign58320_e90622: f64 = if locals.var_vgs <= locals.var_vth { 1.0 } else { 0.0 };
        locals.var_guard1435 = assign58320_e90622;
        locals.var_guard1435_rv = 0.0;

        let (assign58330_e90636, assign58330_e90636_d_n0, assign58330_e90636_d_n2, assign58330_e90636_d_n4, assign58330_e90636_d_n5, assign58330_e90636_d_n6, assign58330_e90636_d_n7, assign58330_e90636_d_n8, assign58330_e90636_d_n9, assign58330_e90636_d_n10, assign58330_e90636_d_n11, assign58330_e90636_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1433 != 0.0)) && (locals.var_guard1434 == 0.0)) && (locals.var_guard1435 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn14,)
    }
};
        locals.var_ps0_ini = assign58330_e90636;
        locals.var_ps0_ini_dn0 = assign58330_e90636_d_n0;
        locals.var_ps0_ini_dn2 = assign58330_e90636_d_n2;
        locals.var_ps0_ini_dn4 = assign58330_e90636_d_n4;
        locals.var_ps0_ini_dn5 = assign58330_e90636_d_n5;
        locals.var_ps0_ini_dn6 = assign58330_e90636_d_n6;
        locals.var_ps0_ini_dn7 = assign58330_e90636_d_n7;
        locals.var_ps0_ini_dn8 = assign58330_e90636_d_n8;
        locals.var_ps0_ini_dn9 = assign58330_e90636_d_n9;
        locals.var_ps0_ini_dn10 = assign58330_e90636_d_n10;
        locals.var_ps0_ini_dn11 = assign58330_e90636_d_n11;
        locals.var_ps0_ini_dn14 = assign58330_e90636_d_n14;
        locals.var_ps0_ini_rv = 0.0;

        let (assign58340_e90655, assign58340_e90655_d_n0, assign58340_e90655_d_n2, assign58340_e90655_d_n4, assign58340_e90655_d_n5, assign58340_e90655_d_n6, assign58340_e90655_d_n7, assign58340_e90655_d_n8, assign58340_e90655_d_n9, assign58340_e90655_d_n10, assign58340_e90655_d_n11, assign58340_e90655_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1433 != 0.0)) && (locals.var_guard1434 == 0.0)) && (locals.var_guard1435 == 0.0)) {
        let assign58340_e90651: f64 = (1.0 / locals.var_cnst1);
        let assign58340_e90653: f64 = (assign58340_e90651 / locals.var_cnstcoxi);
        (assign58340_e90653, ((((-(locals.var_cnst1_dn0 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign58340_e90651 * locals.var_cnstcoxi_dn0)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn2 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign58340_e90651 * locals.var_cnstcoxi_dn2)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn4 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign58340_e90651 * locals.var_cnstcoxi_dn4)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn5 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign58340_e90651 * locals.var_cnstcoxi_dn5)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn6 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign58340_e90651 * locals.var_cnstcoxi_dn6)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn7 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign58340_e90651 * locals.var_cnstcoxi_dn7)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn8 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign58340_e90651 * locals.var_cnstcoxi_dn8)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn9 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign58340_e90651 * locals.var_cnstcoxi_dn9)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn10 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign58340_e90651 * locals.var_cnstcoxi_dn10)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn11 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign58340_e90651 * locals.var_cnstcoxi_dn11)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn14 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign58340_e90651 * locals.var_cnstcoxi_dn14)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign58340_e90655;
        locals.var_t1_dn0 = assign58340_e90655_d_n0;
        locals.var_t1_dn2 = assign58340_e90655_d_n2;
        locals.var_t1_dn4 = assign58340_e90655_d_n4;
        locals.var_t1_dn5 = assign58340_e90655_d_n5;
        locals.var_t1_dn6 = assign58340_e90655_d_n6;
        locals.var_t1_dn7 = assign58340_e90655_d_n7;
        locals.var_t1_dn8 = assign58340_e90655_d_n8;
        locals.var_t1_dn9 = assign58340_e90655_d_n9;
        locals.var_t1_dn10 = assign58340_e90655_d_n10;
        locals.var_t1_dn11 = assign58340_e90655_d_n11;
        locals.var_t1_dn14 = assign58340_e90655_d_n14;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_214(
        locals: &mut StampLocals,
    ) {
        let (assign58350_e90674, assign58350_e90674_d_n0, assign58350_e90674_d_n2, assign58350_e90674_d_n4, assign58350_e90674_d_n5, assign58350_e90674_d_n6, assign58350_e90674_d_n7, assign58350_e90674_d_n8, assign58350_e90674_d_n9, assign58350_e90674_d_n10, assign58350_e90674_d_n11, assign58350_e90674_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1433 != 0.0)) && (locals.var_guard1434 == 0.0)) && (locals.var_guard1435 == 0.0)) {
        let assign58350_e90670: f64 = (locals.var_t1 * locals.var_vgp);
        let assign58350_e90672: f64 = (assign58350_e90670 * locals.var_vgp);
        (assign58350_e90672, ((((locals.var_t1_dn0 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn0)) * locals.var_vgp) + (assign58350_e90670 * locals.var_vgp_dn0)), ((((locals.var_t1_dn2 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn2)) * locals.var_vgp) + (assign58350_e90670 * locals.var_vgp_dn2)), ((((locals.var_t1_dn4 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn4)) * locals.var_vgp) + (assign58350_e90670 * locals.var_vgp_dn4)), ((((locals.var_t1_dn5 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn5)) * locals.var_vgp) + (assign58350_e90670 * locals.var_vgp_dn5)), ((((locals.var_t1_dn6 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn6)) * locals.var_vgp) + (assign58350_e90670 * locals.var_vgp_dn6)), ((((locals.var_t1_dn7 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn7)) * locals.var_vgp) + (assign58350_e90670 * locals.var_vgp_dn7)), ((((locals.var_t1_dn8 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn8)) * locals.var_vgp) + (assign58350_e90670 * locals.var_vgp_dn8)), ((((locals.var_t1_dn9 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn9)) * locals.var_vgp) + (assign58350_e90670 * locals.var_vgp_dn9)), ((((locals.var_t1_dn10 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn10)) * locals.var_vgp) + (assign58350_e90670 * locals.var_vgp_dn10)), ((((locals.var_t1_dn11 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn11)) * locals.var_vgp) + (assign58350_e90670 * locals.var_vgp_dn11)), ((((locals.var_t1_dn14 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn14)) * locals.var_vgp) + (assign58350_e90670 * locals.var_vgp_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign58350_e90674;
        locals.var_t2_dn0 = assign58350_e90674_d_n0;
        locals.var_t2_dn2 = assign58350_e90674_d_n2;
        locals.var_t2_dn4 = assign58350_e90674_d_n4;
        locals.var_t2_dn5 = assign58350_e90674_d_n5;
        locals.var_t2_dn6 = assign58350_e90674_d_n6;
        locals.var_t2_dn7 = assign58350_e90674_d_n7;
        locals.var_t2_dn8 = assign58350_e90674_d_n8;
        locals.var_t2_dn9 = assign58350_e90674_d_n9;
        locals.var_t2_dn10 = assign58350_e90674_d_n10;
        locals.var_t2_dn11 = assign58350_e90674_d_n11;
        locals.var_t2_dn14 = assign58350_e90674_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign58360_e90693, assign58360_e90693_d_n0, assign58360_e90693_d_n2, assign58360_e90693_d_n4, assign58360_e90693_d_n5, assign58360_e90693_d_n6, assign58360_e90693_d_n7, assign58360_e90693_d_n8, assign58360_e90693_d_n9, assign58360_e90693_d_n10, assign58360_e90693_d_n11, assign58360_e90693_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1433 != 0.0)) && (locals.var_guard1434 == 0.0)) && (locals.var_guard1435 == 0.0)) {
        let assign58360_e90690: f64 = (2.0 / locals.var_vgp);
        let assign58360_e90691: f64 = (locals.var_beta + assign58360_e90690);
        (assign58360_e90691, (locals.var_beta_dn0 + (-((2.0 * locals.var_vgp_dn0) / (locals.var_vgp * locals.var_vgp)))), (locals.var_beta_dn2 + (-((2.0 * locals.var_vgp_dn2) / (locals.var_vgp * locals.var_vgp)))), (locals.var_beta_dn4 + (-((2.0 * locals.var_vgp_dn4) / (locals.var_vgp * locals.var_vgp)))), (locals.var_beta_dn5 + (-((2.0 * locals.var_vgp_dn5) / (locals.var_vgp * locals.var_vgp)))), (locals.var_beta_dn6 + (-((2.0 * locals.var_vgp_dn6) / (locals.var_vgp * locals.var_vgp)))), (locals.var_beta_dn7 + (-((2.0 * locals.var_vgp_dn7) / (locals.var_vgp * locals.var_vgp)))), (locals.var_beta_dn8 + (-((2.0 * locals.var_vgp_dn8) / (locals.var_vgp * locals.var_vgp)))), (locals.var_beta_dn9 + (-((2.0 * locals.var_vgp_dn9) / (locals.var_vgp * locals.var_vgp)))), (locals.var_beta_dn10 + (-((2.0 * locals.var_vgp_dn10) / (locals.var_vgp * locals.var_vgp)))), (locals.var_beta_dn11 + (-((2.0 * locals.var_vgp_dn11) / (locals.var_vgp * locals.var_vgp)))), (locals.var_beta_dn14 + (-((2.0 * locals.var_vgp_dn14) / (locals.var_vgp * locals.var_vgp)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign58360_e90693;
        locals.var_t3_dn0 = assign58360_e90693_d_n0;
        locals.var_t3_dn2 = assign58360_e90693_d_n2;
        locals.var_t3_dn4 = assign58360_e90693_d_n4;
        locals.var_t3_dn5 = assign58360_e90693_d_n5;
        locals.var_t3_dn6 = assign58360_e90693_d_n6;
        locals.var_t3_dn7 = assign58360_e90693_d_n7;
        locals.var_t3_dn8 = assign58360_e90693_d_n8;
        locals.var_t3_dn9 = assign58360_e90693_d_n9;
        locals.var_t3_dn10 = assign58360_e90693_d_n10;
        locals.var_t3_dn11 = assign58360_e90693_d_n11;
        locals.var_t3_dn14 = assign58360_e90693_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign58370_e90711, assign58370_e90711_d_n0, assign58370_e90711_d_n2, assign58370_e90711_d_n4, assign58370_e90711_d_n5, assign58370_e90711_d_n6, assign58370_e90711_d_n7, assign58370_e90711_d_n8, assign58370_e90711_d_n9, assign58370_e90711_d_n10, assign58370_e90711_d_n11, assign58370_e90711_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1433 != 0.0)) && (locals.var_guard1434 == 0.0)) && (locals.var_guard1435 == 0.0)) {
        let assign58370_e90707: f64 = (locals.var_t2).ln();
        let assign58370_e90709: f64 = (assign58370_e90707 / locals.var_t3);
        (assign58370_e90709, ((((locals.var_t2_dn0 / locals.var_t2) * locals.var_t3) - (assign58370_e90707 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn2 / locals.var_t2) * locals.var_t3) - (assign58370_e90707 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn4 / locals.var_t2) * locals.var_t3) - (assign58370_e90707 * locals.var_t3_dn4)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn5 / locals.var_t2) * locals.var_t3) - (assign58370_e90707 * locals.var_t3_dn5)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn6 / locals.var_t2) * locals.var_t3) - (assign58370_e90707 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn7 / locals.var_t2) * locals.var_t3) - (assign58370_e90707 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn8 / locals.var_t2) * locals.var_t3) - (assign58370_e90707 * locals.var_t3_dn8)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn9 / locals.var_t2) * locals.var_t3) - (assign58370_e90707 * locals.var_t3_dn9)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn10 / locals.var_t2) * locals.var_t3) - (assign58370_e90707 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn11 / locals.var_t2) * locals.var_t3) - (assign58370_e90707 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn14 / locals.var_t2) * locals.var_t3) - (assign58370_e90707 * locals.var_t3_dn14)) / (locals.var_t3 * locals.var_t3)),)
    } else {
        (locals.var_ps0_inib, locals.var_ps0_inib_dn0, locals.var_ps0_inib_dn2, locals.var_ps0_inib_dn4, locals.var_ps0_inib_dn5, locals.var_ps0_inib_dn6, locals.var_ps0_inib_dn7, locals.var_ps0_inib_dn8, locals.var_ps0_inib_dn9, locals.var_ps0_inib_dn10, locals.var_ps0_inib_dn11, locals.var_ps0_inib_dn14,)
    }
};
        locals.var_ps0_inib = assign58370_e90711;
        locals.var_ps0_inib_dn0 = assign58370_e90711_d_n0;
        locals.var_ps0_inib_dn2 = assign58370_e90711_d_n2;
        locals.var_ps0_inib_dn4 = assign58370_e90711_d_n4;
        locals.var_ps0_inib_dn5 = assign58370_e90711_d_n5;
        locals.var_ps0_inib_dn6 = assign58370_e90711_d_n6;
        locals.var_ps0_inib_dn7 = assign58370_e90711_d_n7;
        locals.var_ps0_inib_dn8 = assign58370_e90711_d_n8;
        locals.var_ps0_inib_dn9 = assign58370_e90711_d_n9;
        locals.var_ps0_inib_dn10 = assign58370_e90711_d_n10;
        locals.var_ps0_inib_dn11 = assign58370_e90711_d_n11;
        locals.var_ps0_inib_dn14 = assign58370_e90711_d_n14;
        locals.var_ps0_inib_rv = 0.0;

        let (assign58380_e90730, assign58380_e90730_d_n0, assign58380_e90730_d_n2, assign58380_e90730_d_n4, assign58380_e90730_d_n5, assign58380_e90730_d_n6, assign58380_e90730_d_n7, assign58380_e90730_d_n8, assign58380_e90730_d_n9, assign58380_e90730_d_n10, assign58380_e90730_d_n11, assign58380_e90730_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1433 != 0.0)) && (locals.var_guard1434 == 0.0)) && (locals.var_guard1435 == 0.0)) {
        let assign58380_e90726: f64 = (locals.var_ps0_inib - locals.var_ps0_inia);
        let assign58380_e90728: f64 = (assign58380_e90726 - 0.0008);
        (assign58380_e90728, (locals.var_ps0_inib_dn0 - locals.var_ps0_inia_dn0), (locals.var_ps0_inib_dn2 - locals.var_ps0_inia_dn2), (locals.var_ps0_inib_dn4 - locals.var_ps0_inia_dn4), (locals.var_ps0_inib_dn5 - locals.var_ps0_inia_dn5), (locals.var_ps0_inib_dn6 - locals.var_ps0_inia_dn6), (locals.var_ps0_inib_dn7 - locals.var_ps0_inia_dn7), (locals.var_ps0_inib_dn8 - locals.var_ps0_inia_dn8), (locals.var_ps0_inib_dn9 - locals.var_ps0_inia_dn9), (locals.var_ps0_inib_dn10 - locals.var_ps0_inia_dn10), (locals.var_ps0_inib_dn11 - locals.var_ps0_inia_dn11), (locals.var_ps0_inib_dn14 - locals.var_ps0_inia_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign58380_e90730;
        locals.var_tmf1_dn0 = assign58380_e90730_d_n0;
        locals.var_tmf1_dn2 = assign58380_e90730_d_n2;
        locals.var_tmf1_dn4 = assign58380_e90730_d_n4;
        locals.var_tmf1_dn5 = assign58380_e90730_d_n5;
        locals.var_tmf1_dn6 = assign58380_e90730_d_n6;
        locals.var_tmf1_dn7 = assign58380_e90730_d_n7;
        locals.var_tmf1_dn8 = assign58380_e90730_d_n8;
        locals.var_tmf1_dn9 = assign58380_e90730_d_n9;
        locals.var_tmf1_dn10 = assign58380_e90730_d_n10;
        locals.var_tmf1_dn11 = assign58380_e90730_d_n11;
        locals.var_tmf1_dn14 = assign58380_e90730_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign58390_e90749, assign58390_e90749_d_n0, assign58390_e90749_d_n2, assign58390_e90749_d_n4, assign58390_e90749_d_n5, assign58390_e90749_d_n6, assign58390_e90749_d_n7, assign58390_e90749_d_n8, assign58390_e90749_d_n9, assign58390_e90749_d_n10, assign58390_e90749_d_n11, assign58390_e90749_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1433 != 0.0)) && (locals.var_guard1434 == 0.0)) && (locals.var_guard1435 == 0.0)) {
        let assign58390_e90745: f64 = (4.0 * locals.var_ps0_inib);
        let assign58390_e90747: f64 = (assign58390_e90745 * 0.0008);
        (assign58390_e90747, ((4.0 * locals.var_ps0_inib_dn0) * 0.0008), ((4.0 * locals.var_ps0_inib_dn2) * 0.0008), ((4.0 * locals.var_ps0_inib_dn4) * 0.0008), ((4.0 * locals.var_ps0_inib_dn5) * 0.0008), ((4.0 * locals.var_ps0_inib_dn6) * 0.0008), ((4.0 * locals.var_ps0_inib_dn7) * 0.0008), ((4.0 * locals.var_ps0_inib_dn8) * 0.0008), ((4.0 * locals.var_ps0_inib_dn9) * 0.0008), ((4.0 * locals.var_ps0_inib_dn10) * 0.0008), ((4.0 * locals.var_ps0_inib_dn11) * 0.0008), ((4.0 * locals.var_ps0_inib_dn14) * 0.0008),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign58390_e90749;
        locals.var_tmf2_dn0 = assign58390_e90749_d_n0;
        locals.var_tmf2_dn2 = assign58390_e90749_d_n2;
        locals.var_tmf2_dn4 = assign58390_e90749_d_n4;
        locals.var_tmf2_dn5 = assign58390_e90749_d_n5;
        locals.var_tmf2_dn6 = assign58390_e90749_d_n6;
        locals.var_tmf2_dn7 = assign58390_e90749_d_n7;
        locals.var_tmf2_dn8 = assign58390_e90749_d_n8;
        locals.var_tmf2_dn9 = assign58390_e90749_d_n9;
        locals.var_tmf2_dn10 = assign58390_e90749_d_n10;
        locals.var_tmf2_dn11 = assign58390_e90749_d_n11;
        locals.var_tmf2_dn14 = assign58390_e90749_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign58400_e90770, assign58400_e90770_d_n0, assign58400_e90770_d_n2, assign58400_e90770_d_n4, assign58400_e90770_d_n5, assign58400_e90770_d_n6, assign58400_e90770_d_n7, assign58400_e90770_d_n8, assign58400_e90770_d_n9, assign58400_e90770_d_n10, assign58400_e90770_d_n11, assign58400_e90770_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1433 != 0.0)) && (locals.var_guard1434 == 0.0)) && (locals.var_guard1435 == 0.0)) {
        let (assign58400_e90768, assign58400_e90768_d_n0, assign58400_e90768_d_n2, assign58400_e90768_d_n4, assign58400_e90768_d_n5, assign58400_e90768_d_n6, assign58400_e90768_d_n7, assign58400_e90768_d_n8, assign58400_e90768_d_n9, assign58400_e90768_d_n10, assign58400_e90768_d_n11, assign58400_e90768_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign58400_e90767: f64 = (-locals.var_tmf2);
                (assign58400_e90767, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign58400_e90768, assign58400_e90768_d_n0, assign58400_e90768_d_n2, assign58400_e90768_d_n4, assign58400_e90768_d_n5, assign58400_e90768_d_n6, assign58400_e90768_d_n7, assign58400_e90768_d_n8, assign58400_e90768_d_n9, assign58400_e90768_d_n10, assign58400_e90768_d_n11, assign58400_e90768_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign58400_e90770;
        locals.var_tmf2_dn0 = assign58400_e90770_d_n0;
        locals.var_tmf2_dn2 = assign58400_e90770_d_n2;
        locals.var_tmf2_dn4 = assign58400_e90770_d_n4;
        locals.var_tmf2_dn5 = assign58400_e90770_d_n5;
        locals.var_tmf2_dn6 = assign58400_e90770_d_n6;
        locals.var_tmf2_dn7 = assign58400_e90770_d_n7;
        locals.var_tmf2_dn8 = assign58400_e90770_d_n8;
        locals.var_tmf2_dn9 = assign58400_e90770_d_n9;
        locals.var_tmf2_dn10 = assign58400_e90770_d_n10;
        locals.var_tmf2_dn11 = assign58400_e90770_d_n11;
        locals.var_tmf2_dn14 = assign58400_e90770_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign58410_e90790, assign58410_e90790_d_n0, assign58410_e90790_d_n2, assign58410_e90790_d_n4, assign58410_e90790_d_n5, assign58410_e90790_d_n6, assign58410_e90790_d_n7, assign58410_e90790_d_n8, assign58410_e90790_d_n9, assign58410_e90790_d_n10, assign58410_e90790_d_n11, assign58410_e90790_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1433 != 0.0)) && (locals.var_guard1434 == 0.0)) && (locals.var_guard1435 == 0.0)) {
        let assign58410_e90785: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign58410_e90787: f64 = (assign58410_e90785 + locals.var_tmf2);
        let assign58410_e90788: f64 = (assign58410_e90787).sqrt();
        (assign58410_e90788, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign58410_e90788)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign58410_e90788)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign58410_e90788)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign58410_e90788)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign58410_e90788)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign58410_e90788)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign58410_e90788)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign58410_e90788)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign58410_e90788)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign58410_e90788)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign58410_e90788)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign58410_e90790;
        locals.var_tmf2_dn0 = assign58410_e90790_d_n0;
        locals.var_tmf2_dn2 = assign58410_e90790_d_n2;
        locals.var_tmf2_dn4 = assign58410_e90790_d_n4;
        locals.var_tmf2_dn5 = assign58410_e90790_d_n5;
        locals.var_tmf2_dn6 = assign58410_e90790_d_n6;
        locals.var_tmf2_dn7 = assign58410_e90790_d_n7;
        locals.var_tmf2_dn8 = assign58410_e90790_d_n8;
        locals.var_tmf2_dn9 = assign58410_e90790_d_n9;
        locals.var_tmf2_dn10 = assign58410_e90790_d_n10;
        locals.var_tmf2_dn11 = assign58410_e90790_d_n11;
        locals.var_tmf2_dn14 = assign58410_e90790_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign58420_e90811, assign58420_e90811_d_n0, assign58420_e90811_d_n2, assign58420_e90811_d_n4, assign58420_e90811_d_n5, assign58420_e90811_d_n6, assign58420_e90811_d_n7, assign58420_e90811_d_n8, assign58420_e90811_d_n9, assign58420_e90811_d_n10, assign58420_e90811_d_n11, assign58420_e90811_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1433 != 0.0)) && (locals.var_guard1434 == 0.0)) && (locals.var_guard1435 == 0.0)) {
        let assign58420_e90807: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign58420_e90808: f64 = (1.0 + assign58420_e90807);
        let assign58420_e90809: f64 = (0.5 * assign58420_e90808);
        (assign58420_e90809, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign58420_e90811;
        locals.var_t1_dn0 = assign58420_e90811_d_n0;
        locals.var_t1_dn2 = assign58420_e90811_d_n2;
        locals.var_t1_dn4 = assign58420_e90811_d_n4;
        locals.var_t1_dn5 = assign58420_e90811_d_n5;
        locals.var_t1_dn6 = assign58420_e90811_d_n6;
        locals.var_t1_dn7 = assign58420_e90811_d_n7;
        locals.var_t1_dn8 = assign58420_e90811_d_n8;
        locals.var_t1_dn9 = assign58420_e90811_d_n9;
        locals.var_t1_dn10 = assign58420_e90811_d_n10;
        locals.var_t1_dn11 = assign58420_e90811_d_n11;
        locals.var_t1_dn14 = assign58420_e90811_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign58430_e90832, assign58430_e90832_d_n0, assign58430_e90832_d_n2, assign58430_e90832_d_n4, assign58430_e90832_d_n5, assign58430_e90832_d_n6, assign58430_e90832_d_n7, assign58430_e90832_d_n8, assign58430_e90832_d_n9, assign58430_e90832_d_n10, assign58430_e90832_d_n11, assign58430_e90832_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1433 != 0.0)) && (locals.var_guard1434 == 0.0)) && (locals.var_guard1435 == 0.0)) {
        let assign58430_e90828: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign58430_e90829: f64 = (0.5 * assign58430_e90828);
        let assign58430_e90830: f64 = (locals.var_ps0_inib - assign58430_e90829);
        (assign58430_e90830, (locals.var_ps0_inib_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_ps0_inib_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_ps0_inib_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_ps0_inib_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_ps0_inib_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_ps0_inib_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_ps0_inib_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_ps0_inib_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_ps0_inib_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_ps0_inib_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_ps0_inib_dn14 - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn14,)
    }
};
        locals.var_ps0_ini = assign58430_e90832;
        locals.var_ps0_ini_dn0 = assign58430_e90832_d_n0;
        locals.var_ps0_ini_dn2 = assign58430_e90832_d_n2;
        locals.var_ps0_ini_dn4 = assign58430_e90832_d_n4;
        locals.var_ps0_ini_dn5 = assign58430_e90832_d_n5;
        locals.var_ps0_ini_dn6 = assign58430_e90832_d_n6;
        locals.var_ps0_ini_dn7 = assign58430_e90832_d_n7;
        locals.var_ps0_ini_dn8 = assign58430_e90832_d_n8;
        locals.var_ps0_ini_dn9 = assign58430_e90832_d_n9;
        locals.var_ps0_ini_dn10 = assign58430_e90832_d_n10;
        locals.var_ps0_ini_dn11 = assign58430_e90832_d_n11;
        locals.var_ps0_ini_dn14 = assign58430_e90832_d_n14;
        locals.var_ps0_ini_rv = 0.0;

        let (assign58440_e90843, assign58440_e90843_d_n0, assign58440_e90843_d_n2, assign58440_e90843_d_n4, assign58440_e90843_d_n5, assign58440_e90843_d_n6, assign58440_e90843_d_n7, assign58440_e90843_d_n8, assign58440_e90843_d_n9, assign58440_e90843_d_n10, assign58440_e90843_d_n11, assign58440_e90843_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign58440_e90840: f64 = (1e-12 / 2.0);
        let assign58440_e90841: f64 = (locals.var_vbscl__blk437 + assign58440_e90840);
        (assign58440_e90841, locals.var_vbscl__blk437_dn0, locals.var_vbscl__blk437_dn2, locals.var_vbscl__blk437_dn4, locals.var_vbscl__blk437_dn5, locals.var_vbscl__blk437_dn6, locals.var_vbscl__blk437_dn7, locals.var_vbscl__blk437_dn8, locals.var_vbscl__blk437_dn9, locals.var_vbscl__blk437_dn10, locals.var_vbscl__blk437_dn11, locals.var_vbscl__blk437_dn14,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign58440_e90843;
        locals.var_tx_dn0 = assign58440_e90843_d_n0;
        locals.var_tx_dn2 = assign58440_e90843_d_n2;
        locals.var_tx_dn4 = assign58440_e90843_d_n4;
        locals.var_tx_dn5 = assign58440_e90843_d_n5;
        locals.var_tx_dn6 = assign58440_e90843_d_n6;
        locals.var_tx_dn7 = assign58440_e90843_d_n7;
        locals.var_tx_dn8 = assign58440_e90843_d_n8;
        locals.var_tx_dn9 = assign58440_e90843_d_n9;
        locals.var_tx_dn10 = assign58440_e90843_d_n10;
        locals.var_tx_dn11 = assign58440_e90843_d_n11;
        locals.var_tx_dn14 = assign58440_e90843_d_n14;
        locals.var_tx_rv = 0.0;

        let assign58450_e90846: f64 = if locals.var_ps0_ini < locals.var_tx { 1.0 } else { 0.0 };
        locals.var_guard1436 = assign58450_e90846;
        locals.var_guard1436_rv = 0.0;

        let (assign58460_e90855, assign58460_e90855_d_n0, assign58460_e90855_d_n2, assign58460_e90855_d_n4, assign58460_e90855_d_n5, assign58460_e90855_d_n6, assign58460_e90855_d_n7, assign58460_e90855_d_n8, assign58460_e90855_d_n9, assign58460_e90855_d_n10, assign58460_e90855_d_n11, assign58460_e90855_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1436 != 0.0)) {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn14,)
    }
};
        locals.var_ps0_ini = assign58460_e90855;
        locals.var_ps0_ini_dn0 = assign58460_e90855_d_n0;
        locals.var_ps0_ini_dn2 = assign58460_e90855_d_n2;
        locals.var_ps0_ini_dn4 = assign58460_e90855_d_n4;
        locals.var_ps0_ini_dn5 = assign58460_e90855_d_n5;
        locals.var_ps0_ini_dn6 = assign58460_e90855_d_n6;
        locals.var_ps0_ini_dn7 = assign58460_e90855_d_n7;
        locals.var_ps0_ini_dn8 = assign58460_e90855_d_n8;
        locals.var_ps0_ini_dn9 = assign58460_e90855_d_n9;
        locals.var_ps0_ini_dn10 = assign58460_e90855_d_n10;
        locals.var_ps0_ini_dn11 = assign58460_e90855_d_n11;
        locals.var_ps0_ini_dn14 = assign58460_e90855_d_n14;
        locals.var_ps0_ini_rv = 0.0;

        let (assign58470_e90862, assign58470_e90862_d_n0, assign58470_e90862_d_n2, assign58470_e90862_d_n4, assign58470_e90862_d_n5, assign58470_e90862_d_n6, assign58470_e90862_d_n7, assign58470_e90862_d_n8, assign58470_e90862_d_n9, assign58470_e90862_d_n10, assign58470_e90862_d_n11, assign58470_e90862_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn14,)
    } else {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn4, locals.var_ps0_dn5, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn8, locals.var_ps0_dn9, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn14,)
    }
};
        locals.var_ps0 = assign58470_e90862;
        locals.var_ps0_dn0 = assign58470_e90862_d_n0;
        locals.var_ps0_dn2 = assign58470_e90862_d_n2;
        locals.var_ps0_dn4 = assign58470_e90862_d_n4;
        locals.var_ps0_dn5 = assign58470_e90862_d_n5;
        locals.var_ps0_dn6 = assign58470_e90862_d_n6;
        locals.var_ps0_dn7 = assign58470_e90862_d_n7;
        locals.var_ps0_dn8 = assign58470_e90862_d_n8;
        locals.var_ps0_dn9 = assign58470_e90862_d_n9;
        locals.var_ps0_dn10 = assign58470_e90862_d_n10;
        locals.var_ps0_dn11 = assign58470_e90862_d_n11;
        locals.var_ps0_dn14 = assign58470_e90862_d_n14;
        locals.var_ps0_rv = 0.0;

        let (assign58480_e90869, assign58480_e90869_d_n0, assign58480_e90869_d_n2, assign58480_e90869_d_n4, assign58480_e90869_d_n5, assign58480_e90869_d_n6, assign58480_e90869_d_n7, assign58480_e90869_d_n8, assign58480_e90869_d_n9, assign58480_e90869_d_n10, assign58480_e90869_d_n11, assign58480_e90869_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    } else {
        (locals.var_psl_lim, locals.var_psl_lim_dn0, locals.var_psl_lim_dn2, locals.var_psl_lim_dn4, locals.var_psl_lim_dn5, locals.var_psl_lim_dn6, locals.var_psl_lim_dn7, locals.var_psl_lim_dn8, locals.var_psl_lim_dn9, locals.var_psl_lim_dn10, locals.var_psl_lim_dn11, locals.var_psl_lim_dn14,)
    }
};
        locals.var_psl_lim = assign58480_e90869;
        locals.var_psl_lim_dn0 = assign58480_e90869_d_n0;
        locals.var_psl_lim_dn2 = assign58480_e90869_d_n2;
        locals.var_psl_lim_dn4 = assign58480_e90869_d_n4;
        locals.var_psl_lim_dn5 = assign58480_e90869_d_n5;
        locals.var_psl_lim_dn6 = assign58480_e90869_d_n6;
        locals.var_psl_lim_dn7 = assign58480_e90869_d_n7;
        locals.var_psl_lim_dn8 = assign58480_e90869_d_n8;
        locals.var_psl_lim_dn9 = assign58480_e90869_d_n9;
        locals.var_psl_lim_dn10 = assign58480_e90869_d_n10;
        locals.var_psl_lim_dn11 = assign58480_e90869_d_n11;
        locals.var_psl_lim_dn14 = assign58480_e90869_d_n14;
        locals.var_psl_lim_rv = 0.0;

        let (assign58490_e90879, assign58490_e90879_d_n0, assign58490_e90879_d_n2, assign58490_e90879_d_n4, assign58490_e90879_d_n5, assign58490_e90879_d_n6, assign58490_e90879_d_n7, assign58490_e90879_d_n8, assign58490_e90879_d_n9, assign58490_e90879_d_n10, assign58490_e90879_d_n11, assign58490_e90879_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign58490_e90876: f64 = (locals.var_beta * locals.var_vbscl__blk437);
        let assign58490_e90877: f64 = (assign58490_e90876).exp();
        (assign58490_e90877, (assign58490_e90877 * ((locals.var_beta_dn0 * locals.var_vbscl__blk437) + (locals.var_beta * locals.var_vbscl__blk437_dn0))), (assign58490_e90877 * ((locals.var_beta_dn2 * locals.var_vbscl__blk437) + (locals.var_beta * locals.var_vbscl__blk437_dn2))), (assign58490_e90877 * ((locals.var_beta_dn4 * locals.var_vbscl__blk437) + (locals.var_beta * locals.var_vbscl__blk437_dn4))), (assign58490_e90877 * ((locals.var_beta_dn5 * locals.var_vbscl__blk437) + (locals.var_beta * locals.var_vbscl__blk437_dn5))), (assign58490_e90877 * ((locals.var_beta_dn6 * locals.var_vbscl__blk437) + (locals.var_beta * locals.var_vbscl__blk437_dn6))), (assign58490_e90877 * ((locals.var_beta_dn7 * locals.var_vbscl__blk437) + (locals.var_beta * locals.var_vbscl__blk437_dn7))), (assign58490_e90877 * ((locals.var_beta_dn8 * locals.var_vbscl__blk437) + (locals.var_beta * locals.var_vbscl__blk437_dn8))), (assign58490_e90877 * ((locals.var_beta_dn9 * locals.var_vbscl__blk437) + (locals.var_beta * locals.var_vbscl__blk437_dn9))), (assign58490_e90877 * ((locals.var_beta_dn10 * locals.var_vbscl__blk437) + (locals.var_beta * locals.var_vbscl__blk437_dn10))), (assign58490_e90877 * ((locals.var_beta_dn11 * locals.var_vbscl__blk437) + (locals.var_beta * locals.var_vbscl__blk437_dn11))), (assign58490_e90877 * ((locals.var_beta_dn14 * locals.var_vbscl__blk437) + (locals.var_beta * locals.var_vbscl__blk437_dn14))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
        locals.var_exp_bvbs = assign58490_e90879;
        locals.var_exp_bvbs_dn0 = assign58490_e90879_d_n0;
        locals.var_exp_bvbs_dn2 = assign58490_e90879_d_n2;
        locals.var_exp_bvbs_dn4 = assign58490_e90879_d_n4;
        locals.var_exp_bvbs_dn5 = assign58490_e90879_d_n5;
        locals.var_exp_bvbs_dn6 = assign58490_e90879_d_n6;
        locals.var_exp_bvbs_dn7 = assign58490_e90879_d_n7;
        locals.var_exp_bvbs_dn8 = assign58490_e90879_d_n8;
        locals.var_exp_bvbs_dn9 = assign58490_e90879_d_n9;
        locals.var_exp_bvbs_dn10 = assign58490_e90879_d_n10;
        locals.var_exp_bvbs_dn11 = assign58490_e90879_d_n11;
        locals.var_exp_bvbs_dn14 = assign58490_e90879_d_n14;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign58500_e90888, assign58500_e90888_d_n0, assign58500_e90888_d_n2, assign58500_e90888_d_n4, assign58500_e90888_d_n5, assign58500_e90888_d_n6, assign58500_e90888_d_n7, assign58500_e90888_d_n8, assign58500_e90888_d_n9, assign58500_e90888_d_n10, assign58500_e90888_d_n11, assign58500_e90888_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign58500_e90886: f64 = (locals.var_cnst1 * locals.var_exp_bvbs);
        (assign58500_e90886, ((locals.var_cnst1_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1 * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1 * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1 * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1 * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1 * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1 * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1 * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1 * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1 * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1_dn11 * locals.var_exp_bvbs) + (locals.var_cnst1 * locals.var_exp_bvbs_dn11)), ((locals.var_cnst1_dn14 * locals.var_exp_bvbs) + (locals.var_cnst1 * locals.var_exp_bvbs_dn14)),)
    } else {
        (locals.var_cfs1, locals.var_cfs1_dn0, locals.var_cfs1_dn2, locals.var_cfs1_dn4, locals.var_cfs1_dn5, locals.var_cfs1_dn6, locals.var_cfs1_dn7, locals.var_cfs1_dn8, locals.var_cfs1_dn9, locals.var_cfs1_dn10, locals.var_cfs1_dn11, locals.var_cfs1_dn14,)
    }
};
        locals.var_cfs1 = assign58500_e90888;
        locals.var_cfs1_dn0 = assign58500_e90888_d_n0;
        locals.var_cfs1_dn2 = assign58500_e90888_d_n2;
        locals.var_cfs1_dn4 = assign58500_e90888_d_n4;
        locals.var_cfs1_dn5 = assign58500_e90888_d_n5;
        locals.var_cfs1_dn6 = assign58500_e90888_d_n6;
        locals.var_cfs1_dn7 = assign58500_e90888_d_n7;
        locals.var_cfs1_dn8 = assign58500_e90888_d_n8;
        locals.var_cfs1_dn9 = assign58500_e90888_d_n9;
        locals.var_cfs1_dn10 = assign58500_e90888_d_n10;
        locals.var_cfs1_dn11 = assign58500_e90888_d_n11;
        locals.var_cfs1_dn14 = assign58500_e90888_d_n14;
        locals.var_cfs1_rv = 0.0;

        let (assign58510_e90895,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign58510_e90895;
        locals.var_flg_conv_rv = 0.0;

        let (assign58520_e90902,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign58520_e90902;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_215(
        locals: &mut StampLocals,
    ) {
        let mut assign58530_loop_guard: usize = 0;
        while {
            let assign58530_cond_e90910: f64 = (locals.var_lp_s0_max + 1.0);
            let assign58530_cond_e90912: f64 = if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_lp_s0 <= assign58530_cond_e90910)) { 1.0 } else { 0.0 };
            assign58530_cond_e90912 != 0.0
        } {
            assign58530_loop_guard += 1;
            assert!(assign58530_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign58530_body1_e90932, assign58530_body1_e90932_d_n0, assign58530_body1_e90932_d_n2, assign58530_body1_e90932_d_n4, assign58530_body1_e90932_d_n5, assign58530_body1_e90932_d_n6, assign58530_body1_e90932_d_n7, assign58530_body1_e90932_d_n8, assign58530_body1_e90932_d_n9, assign58530_body1_e90932_d_n10, assign58530_body1_e90932_d_n11, assign58530_body1_e90932_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign58530_body1_e90929: f64 = (locals.var_ps0 - locals.var_vbscl__blk437);
        let assign58530_body1_e90930: f64 = (locals.var_beta * assign58530_body1_e90929);
        (assign58530_body1_e90930, ((locals.var_beta_dn0 * assign58530_body1_e90929) + (locals.var_beta * (locals.var_ps0_dn0 - locals.var_vbscl__blk437_dn0))), ((locals.var_beta_dn2 * assign58530_body1_e90929) + (locals.var_beta * (locals.var_ps0_dn2 - locals.var_vbscl__blk437_dn2))), ((locals.var_beta_dn4 * assign58530_body1_e90929) + (locals.var_beta * (locals.var_ps0_dn4 - locals.var_vbscl__blk437_dn4))), ((locals.var_beta_dn5 * assign58530_body1_e90929) + (locals.var_beta * (locals.var_ps0_dn5 - locals.var_vbscl__blk437_dn5))), ((locals.var_beta_dn6 * assign58530_body1_e90929) + (locals.var_beta * (locals.var_ps0_dn6 - locals.var_vbscl__blk437_dn6))), ((locals.var_beta_dn7 * assign58530_body1_e90929) + (locals.var_beta * (locals.var_ps0_dn7 - locals.var_vbscl__blk437_dn7))), ((locals.var_beta_dn8 * assign58530_body1_e90929) + (locals.var_beta * (locals.var_ps0_dn8 - locals.var_vbscl__blk437_dn8))), ((locals.var_beta_dn9 * assign58530_body1_e90929) + (locals.var_beta * (locals.var_ps0_dn9 - locals.var_vbscl__blk437_dn9))), ((locals.var_beta_dn10 * assign58530_body1_e90929) + (locals.var_beta * (locals.var_ps0_dn10 - locals.var_vbscl__blk437_dn10))), ((locals.var_beta_dn11 * assign58530_body1_e90929) + (locals.var_beta * (locals.var_ps0_dn11 - locals.var_vbscl__blk437_dn11))), ((locals.var_beta_dn14 * assign58530_body1_e90929) + (locals.var_beta * (locals.var_ps0_dn14 - locals.var_vbscl__blk437_dn14))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
            locals.var_chi = assign58530_body1_e90932;
            locals.var_chi_dn0 = assign58530_body1_e90932_d_n0;
            locals.var_chi_dn2 = assign58530_body1_e90932_d_n2;
            locals.var_chi_dn4 = assign58530_body1_e90932_d_n4;
            locals.var_chi_dn5 = assign58530_body1_e90932_d_n5;
            locals.var_chi_dn6 = assign58530_body1_e90932_d_n6;
            locals.var_chi_dn7 = assign58530_body1_e90932_d_n7;
            locals.var_chi_dn8 = assign58530_body1_e90932_d_n8;
            locals.var_chi_dn9 = assign58530_body1_e90932_d_n9;
            locals.var_chi_dn10 = assign58530_body1_e90932_d_n10;
            locals.var_chi_dn11 = assign58530_body1_e90932_d_n11;
            locals.var_chi_dn14 = assign58530_body1_e90932_d_n14;
            locals.var_chi_rv = 0.0;
            let assign58530_body2_e90935: f64 = if locals.var_chi < 5.0 { 1.0 } else { 0.0 };
            locals.var_guard1437 = assign58530_body2_e90935;
            locals.var_guard1437_rv = 0.0;
            let (assign58530_body3_e90959, assign58530_body3_e90959_d_n0, assign58530_body3_e90959_d_n2, assign58530_body3_e90959_d_n4, assign58530_body3_e90959_d_n5, assign58530_body3_e90959_d_n6, assign58530_body3_e90959_d_n7, assign58530_body3_e90959_d_n8, assign58530_body3_e90959_d_n9, assign58530_body3_e90959_d_n10, assign58530_body3_e90959_d_n11, assign58530_body3_e90959_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1437 != 0.0)) {
        let assign58530_body3_e90944: f64 = (locals.var_chi * locals.var_chi);
        let assign58530_body3_e90946: f64 = (assign58530_body3_e90944 * locals.var_chi);
        let assign58530_body3_e90950: f64 = (-0.07053654284009761);
        let assign58530_body3_e90953: f64 = (locals.var_chi * 0.006115288895133179);
        let assign58530_body3_e90954: f64 = (assign58530_body3_e90950 + assign58530_body3_e90953);
        let assign58530_body3_e90955: f64 = (locals.var_chi * assign58530_body3_e90954);
        let assign58530_body3_e90956: f64 = (0.29693154855771 + assign58530_body3_e90955);
        let assign58530_body3_e90957: f64 = (assign58530_body3_e90946 * assign58530_body3_e90956);
        (assign58530_body3_e90957, ((((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) * locals.var_chi) + (assign58530_body3_e90944 * locals.var_chi_dn0)) * assign58530_body3_e90956) + (assign58530_body3_e90946 * ((locals.var_chi_dn0 * assign58530_body3_e90954) + (locals.var_chi * (locals.var_chi_dn0 * 0.006115288895133179))))), ((((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) * locals.var_chi) + (assign58530_body3_e90944 * locals.var_chi_dn2)) * assign58530_body3_e90956) + (assign58530_body3_e90946 * ((locals.var_chi_dn2 * assign58530_body3_e90954) + (locals.var_chi * (locals.var_chi_dn2 * 0.006115288895133179))))), ((((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) * locals.var_chi) + (assign58530_body3_e90944 * locals.var_chi_dn4)) * assign58530_body3_e90956) + (assign58530_body3_e90946 * ((locals.var_chi_dn4 * assign58530_body3_e90954) + (locals.var_chi * (locals.var_chi_dn4 * 0.006115288895133179))))), ((((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) * locals.var_chi) + (assign58530_body3_e90944 * locals.var_chi_dn5)) * assign58530_body3_e90956) + (assign58530_body3_e90946 * ((locals.var_chi_dn5 * assign58530_body3_e90954) + (locals.var_chi * (locals.var_chi_dn5 * 0.006115288895133179))))), ((((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) * locals.var_chi) + (assign58530_body3_e90944 * locals.var_chi_dn6)) * assign58530_body3_e90956) + (assign58530_body3_e90946 * ((locals.var_chi_dn6 * assign58530_body3_e90954) + (locals.var_chi * (locals.var_chi_dn6 * 0.006115288895133179))))), ((((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) * locals.var_chi) + (assign58530_body3_e90944 * locals.var_chi_dn7)) * assign58530_body3_e90956) + (assign58530_body3_e90946 * ((locals.var_chi_dn7 * assign58530_body3_e90954) + (locals.var_chi * (locals.var_chi_dn7 * 0.006115288895133179))))), ((((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) * locals.var_chi) + (assign58530_body3_e90944 * locals.var_chi_dn8)) * assign58530_body3_e90956) + (assign58530_body3_e90946 * ((locals.var_chi_dn8 * assign58530_body3_e90954) + (locals.var_chi * (locals.var_chi_dn8 * 0.006115288895133179))))), ((((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) * locals.var_chi) + (assign58530_body3_e90944 * locals.var_chi_dn9)) * assign58530_body3_e90956) + (assign58530_body3_e90946 * ((locals.var_chi_dn9 * assign58530_body3_e90954) + (locals.var_chi * (locals.var_chi_dn9 * 0.006115288895133179))))), ((((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) * locals.var_chi) + (assign58530_body3_e90944 * locals.var_chi_dn10)) * assign58530_body3_e90956) + (assign58530_body3_e90946 * ((locals.var_chi_dn10 * assign58530_body3_e90954) + (locals.var_chi * (locals.var_chi_dn10 * 0.006115288895133179))))), ((((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) * locals.var_chi) + (assign58530_body3_e90944 * locals.var_chi_dn11)) * assign58530_body3_e90956) + (assign58530_body3_e90946 * ((locals.var_chi_dn11 * assign58530_body3_e90954) + (locals.var_chi * (locals.var_chi_dn11 * 0.006115288895133179))))), ((((((locals.var_chi_dn14 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn14)) * locals.var_chi) + (assign58530_body3_e90944 * locals.var_chi_dn14)) * assign58530_body3_e90956) + (assign58530_body3_e90946 * ((locals.var_chi_dn14 * assign58530_body3_e90954) + (locals.var_chi * (locals.var_chi_dn14 * 0.006115288895133179))))),)
    } else {
        (locals.var_fi, locals.var_fi_dn0, locals.var_fi_dn2, locals.var_fi_dn4, locals.var_fi_dn5, locals.var_fi_dn6, locals.var_fi_dn7, locals.var_fi_dn8, locals.var_fi_dn9, locals.var_fi_dn10, locals.var_fi_dn11, locals.var_fi_dn14,)
    }
};
            locals.var_fi = assign58530_body3_e90959;
            locals.var_fi_dn0 = assign58530_body3_e90959_d_n0;
            locals.var_fi_dn2 = assign58530_body3_e90959_d_n2;
            locals.var_fi_dn4 = assign58530_body3_e90959_d_n4;
            locals.var_fi_dn5 = assign58530_body3_e90959_d_n5;
            locals.var_fi_dn6 = assign58530_body3_e90959_d_n6;
            locals.var_fi_dn7 = assign58530_body3_e90959_d_n7;
            locals.var_fi_dn8 = assign58530_body3_e90959_d_n8;
            locals.var_fi_dn9 = assign58530_body3_e90959_d_n9;
            locals.var_fi_dn10 = assign58530_body3_e90959_d_n10;
            locals.var_fi_dn11 = assign58530_body3_e90959_d_n11;
            locals.var_fi_dn14 = assign58530_body3_e90959_d_n14;
            locals.var_fi_rv = 0.0;
            let (assign58530_body4_e90987, assign58530_body4_e90987_d_n0, assign58530_body4_e90987_d_n2, assign58530_body4_e90987_d_n4, assign58530_body4_e90987_d_n5, assign58530_body4_e90987_d_n6, assign58530_body4_e90987_d_n7, assign58530_body4_e90987_d_n8, assign58530_body4_e90987_d_n9, assign58530_body4_e90987_d_n10, assign58530_body4_e90987_d_n11, assign58530_body4_e90987_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1437 != 0.0)) {
        let assign58530_body4_e90968: f64 = (locals.var_chi * locals.var_chi);
        let assign58530_body4_e90971: f64 = (3.0 * 0.29693154855771);
        let assign58530_body4_e90975: f64 = (-0.07053654284009761);
        let assign58530_body4_e90976: f64 = (4.0 * assign58530_body4_e90975);
        let assign58530_body4_e90979: f64 = (locals.var_chi * 5.0);
        let assign58530_body4_e90981: f64 = (assign58530_body4_e90979 * 0.006115288895133179);
        let assign58530_body4_e90982: f64 = (assign58530_body4_e90976 + assign58530_body4_e90981);
        let assign58530_body4_e90983: f64 = (locals.var_chi * assign58530_body4_e90982);
        let assign58530_body4_e90984: f64 = (assign58530_body4_e90971 + assign58530_body4_e90983);
        let assign58530_body4_e90985: f64 = (assign58530_body4_e90968 * assign58530_body4_e90984);
        (assign58530_body4_e90985, ((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) * assign58530_body4_e90984) + (assign58530_body4_e90968 * ((locals.var_chi_dn0 * assign58530_body4_e90982) + (locals.var_chi * ((locals.var_chi_dn0 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) * assign58530_body4_e90984) + (assign58530_body4_e90968 * ((locals.var_chi_dn2 * assign58530_body4_e90982) + (locals.var_chi * ((locals.var_chi_dn2 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) * assign58530_body4_e90984) + (assign58530_body4_e90968 * ((locals.var_chi_dn4 * assign58530_body4_e90982) + (locals.var_chi * ((locals.var_chi_dn4 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) * assign58530_body4_e90984) + (assign58530_body4_e90968 * ((locals.var_chi_dn5 * assign58530_body4_e90982) + (locals.var_chi * ((locals.var_chi_dn5 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) * assign58530_body4_e90984) + (assign58530_body4_e90968 * ((locals.var_chi_dn6 * assign58530_body4_e90982) + (locals.var_chi * ((locals.var_chi_dn6 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) * assign58530_body4_e90984) + (assign58530_body4_e90968 * ((locals.var_chi_dn7 * assign58530_body4_e90982) + (locals.var_chi * ((locals.var_chi_dn7 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) * assign58530_body4_e90984) + (assign58530_body4_e90968 * ((locals.var_chi_dn8 * assign58530_body4_e90982) + (locals.var_chi * ((locals.var_chi_dn8 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) * assign58530_body4_e90984) + (assign58530_body4_e90968 * ((locals.var_chi_dn9 * assign58530_body4_e90982) + (locals.var_chi * ((locals.var_chi_dn9 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) * assign58530_body4_e90984) + (assign58530_body4_e90968 * ((locals.var_chi_dn10 * assign58530_body4_e90982) + (locals.var_chi * ((locals.var_chi_dn10 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) * assign58530_body4_e90984) + (assign58530_body4_e90968 * ((locals.var_chi_dn11 * assign58530_body4_e90982) + (locals.var_chi * ((locals.var_chi_dn11 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn14 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn14)) * assign58530_body4_e90984) + (assign58530_body4_e90968 * ((locals.var_chi_dn14 * assign58530_body4_e90982) + (locals.var_chi * ((locals.var_chi_dn14 * 5.0) * 0.006115288895133179))))),)
    } else {
        (locals.var_fi_dchi, locals.var_fi_dchi_dn0, locals.var_fi_dchi_dn2, locals.var_fi_dchi_dn4, locals.var_fi_dchi_dn5, locals.var_fi_dchi_dn6, locals.var_fi_dchi_dn7, locals.var_fi_dchi_dn8, locals.var_fi_dchi_dn9, locals.var_fi_dchi_dn10, locals.var_fi_dchi_dn11, locals.var_fi_dchi_dn14,)
    }
};
            locals.var_fi_dchi = assign58530_body4_e90987;
            locals.var_fi_dchi_dn0 = assign58530_body4_e90987_d_n0;
            locals.var_fi_dchi_dn2 = assign58530_body4_e90987_d_n2;
            locals.var_fi_dchi_dn4 = assign58530_body4_e90987_d_n4;
            locals.var_fi_dchi_dn5 = assign58530_body4_e90987_d_n5;
            locals.var_fi_dchi_dn6 = assign58530_body4_e90987_d_n6;
            locals.var_fi_dchi_dn7 = assign58530_body4_e90987_d_n7;
            locals.var_fi_dchi_dn8 = assign58530_body4_e90987_d_n8;
            locals.var_fi_dchi_dn9 = assign58530_body4_e90987_d_n9;
            locals.var_fi_dchi_dn10 = assign58530_body4_e90987_d_n10;
            locals.var_fi_dchi_dn11 = assign58530_body4_e90987_d_n11;
            locals.var_fi_dchi_dn14 = assign58530_body4_e90987_d_n14;
            locals.var_fi_dchi_rv = 0.0;
            let (assign58530_body5_e91000, assign58530_body5_e91000_d_n0, assign58530_body5_e91000_d_n2, assign58530_body5_e91000_d_n4, assign58530_body5_e91000_d_n5, assign58530_body5_e91000_d_n6, assign58530_body5_e91000_d_n7, assign58530_body5_e91000_d_n8, assign58530_body5_e91000_d_n9, assign58530_body5_e91000_d_n10, assign58530_body5_e91000_d_n11, assign58530_body5_e91000_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1437 != 0.0)) {
        let assign58530_body5_e90996: f64 = (locals.var_cfs1 * locals.var_fi);
        let assign58530_body5_e90998: f64 = (assign58530_body5_e90996 * locals.var_fi);
        (assign58530_body5_e90998, ((((locals.var_cfs1_dn0 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn0)) * locals.var_fi) + (assign58530_body5_e90996 * locals.var_fi_dn0)), ((((locals.var_cfs1_dn2 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn2)) * locals.var_fi) + (assign58530_body5_e90996 * locals.var_fi_dn2)), ((((locals.var_cfs1_dn4 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn4)) * locals.var_fi) + (assign58530_body5_e90996 * locals.var_fi_dn4)), ((((locals.var_cfs1_dn5 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn5)) * locals.var_fi) + (assign58530_body5_e90996 * locals.var_fi_dn5)), ((((locals.var_cfs1_dn6 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn6)) * locals.var_fi) + (assign58530_body5_e90996 * locals.var_fi_dn6)), ((((locals.var_cfs1_dn7 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn7)) * locals.var_fi) + (assign58530_body5_e90996 * locals.var_fi_dn7)), ((((locals.var_cfs1_dn8 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn8)) * locals.var_fi) + (assign58530_body5_e90996 * locals.var_fi_dn8)), ((((locals.var_cfs1_dn9 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn9)) * locals.var_fi) + (assign58530_body5_e90996 * locals.var_fi_dn9)), ((((locals.var_cfs1_dn10 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn10)) * locals.var_fi) + (assign58530_body5_e90996 * locals.var_fi_dn10)), ((((locals.var_cfs1_dn11 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn11)) * locals.var_fi) + (assign58530_body5_e90996 * locals.var_fi_dn11)), ((((locals.var_cfs1_dn14 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn14)) * locals.var_fi) + (assign58530_body5_e90996 * locals.var_fi_dn14)),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign58530_body5_e91000;
            locals.var_fs01_dn0 = assign58530_body5_e91000_d_n0;
            locals.var_fs01_dn2 = assign58530_body5_e91000_d_n2;
            locals.var_fs01_dn4 = assign58530_body5_e91000_d_n4;
            locals.var_fs01_dn5 = assign58530_body5_e91000_d_n5;
            locals.var_fs01_dn6 = assign58530_body5_e91000_d_n6;
            locals.var_fs01_dn7 = assign58530_body5_e91000_d_n7;
            locals.var_fs01_dn8 = assign58530_body5_e91000_d_n8;
            locals.var_fs01_dn9 = assign58530_body5_e91000_d_n9;
            locals.var_fs01_dn10 = assign58530_body5_e91000_d_n10;
            locals.var_fs01_dn11 = assign58530_body5_e91000_d_n11;
            locals.var_fs01_dn14 = assign58530_body5_e91000_d_n14;
            locals.var_fs01_rv = 0.0;
            let (assign58530_body6_e91017, assign58530_body6_e91017_d_n0, assign58530_body6_e91017_d_n2, assign58530_body6_e91017_d_n4, assign58530_body6_e91017_d_n5, assign58530_body6_e91017_d_n6, assign58530_body6_e91017_d_n7, assign58530_body6_e91017_d_n8, assign58530_body6_e91017_d_n9, assign58530_body6_e91017_d_n10, assign58530_body6_e91017_d_n11, assign58530_body6_e91017_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1437 != 0.0)) {
        let assign58530_body6_e91009: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign58530_body6_e91011: f64 = (assign58530_body6_e91009 * 2.0);
        let assign58530_body6_e91013: f64 = (assign58530_body6_e91011 * locals.var_fi);
        let assign58530_body6_e91015: f64 = (assign58530_body6_e91013 * locals.var_fi_dchi);
        (assign58530_body6_e91015, (((((((locals.var_cfs1_dn0 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn0)) * 2.0) * locals.var_fi) + (assign58530_body6_e91011 * locals.var_fi_dn0)) * locals.var_fi_dchi) + (assign58530_body6_e91013 * locals.var_fi_dchi_dn0)), (((((((locals.var_cfs1_dn2 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn2)) * 2.0) * locals.var_fi) + (assign58530_body6_e91011 * locals.var_fi_dn2)) * locals.var_fi_dchi) + (assign58530_body6_e91013 * locals.var_fi_dchi_dn2)), (((((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * 2.0) * locals.var_fi) + (assign58530_body6_e91011 * locals.var_fi_dn4)) * locals.var_fi_dchi) + (assign58530_body6_e91013 * locals.var_fi_dchi_dn4)), (((((((locals.var_cfs1_dn5 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn5)) * 2.0) * locals.var_fi) + (assign58530_body6_e91011 * locals.var_fi_dn5)) * locals.var_fi_dchi) + (assign58530_body6_e91013 * locals.var_fi_dchi_dn5)), (((((((locals.var_cfs1_dn6 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn6)) * 2.0) * locals.var_fi) + (assign58530_body6_e91011 * locals.var_fi_dn6)) * locals.var_fi_dchi) + (assign58530_body6_e91013 * locals.var_fi_dchi_dn6)), (((((((locals.var_cfs1_dn7 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn7)) * 2.0) * locals.var_fi) + (assign58530_body6_e91011 * locals.var_fi_dn7)) * locals.var_fi_dchi) + (assign58530_body6_e91013 * locals.var_fi_dchi_dn7)), (((((((locals.var_cfs1_dn8 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn8)) * 2.0) * locals.var_fi) + (assign58530_body6_e91011 * locals.var_fi_dn8)) * locals.var_fi_dchi) + (assign58530_body6_e91013 * locals.var_fi_dchi_dn8)), (((((((locals.var_cfs1_dn9 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn9)) * 2.0) * locals.var_fi) + (assign58530_body6_e91011 * locals.var_fi_dn9)) * locals.var_fi_dchi) + (assign58530_body6_e91013 * locals.var_fi_dchi_dn9)), (((((((locals.var_cfs1_dn10 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn10)) * 2.0) * locals.var_fi) + (assign58530_body6_e91011 * locals.var_fi_dn10)) * locals.var_fi_dchi) + (assign58530_body6_e91013 * locals.var_fi_dchi_dn10)), (((((((locals.var_cfs1_dn11 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn11)) * 2.0) * locals.var_fi) + (assign58530_body6_e91011 * locals.var_fi_dn11)) * locals.var_fi_dchi) + (assign58530_body6_e91013 * locals.var_fi_dchi_dn11)), (((((((locals.var_cfs1_dn14 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn14)) * 2.0) * locals.var_fi) + (assign58530_body6_e91011 * locals.var_fi_dn14)) * locals.var_fi_dchi) + (assign58530_body6_e91013 * locals.var_fi_dchi_dn14)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign58530_body6_e91017;
            locals.var_fs01_dps0_dn0 = assign58530_body6_e91017_d_n0;
            locals.var_fs01_dps0_dn2 = assign58530_body6_e91017_d_n2;
            locals.var_fs01_dps0_dn4 = assign58530_body6_e91017_d_n4;
            locals.var_fs01_dps0_dn5 = assign58530_body6_e91017_d_n5;
            locals.var_fs01_dps0_dn6 = assign58530_body6_e91017_d_n6;
            locals.var_fs01_dps0_dn7 = assign58530_body6_e91017_d_n7;
            locals.var_fs01_dps0_dn8 = assign58530_body6_e91017_d_n8;
            locals.var_fs01_dps0_dn9 = assign58530_body6_e91017_d_n9;
            locals.var_fs01_dps0_dn10 = assign58530_body6_e91017_d_n10;
            locals.var_fs01_dps0_dn11 = assign58530_body6_e91017_d_n11;
            locals.var_fs01_dps0_dn14 = assign58530_body6_e91017_d_n14;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign58530_body7_e91046, assign58530_body7_e91046_d_n0, assign58530_body7_e91046_d_n2, assign58530_body7_e91046_d_n4, assign58530_body7_e91046_d_n5, assign58530_body7_e91046_d_n6, assign58530_body7_e91046_d_n7, assign58530_body7_e91046_d_n8, assign58530_body7_e91046_d_n9, assign58530_body7_e91046_d_n10, assign58530_body7_e91046_d_n11, assign58530_body7_e91046_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1437 != 0.0)) {
        let assign58530_body7_e91028: f64 = (-0.117851130197758);
        let assign58530_body7_e91033: f64 = (-0.00163730162779191);
        let assign58530_body7_e91036: f64 = (locals.var_chi * 6.36964918866352e-5);
        let assign58530_body7_e91037: f64 = (assign58530_body7_e91033 + assign58530_body7_e91036);
        let assign58530_body7_e91038: f64 = (locals.var_chi * assign58530_body7_e91037);
        let assign58530_body7_e91039: f64 = (0.0178800506338833 + assign58530_body7_e91038);
        let assign58530_body7_e91040: f64 = (locals.var_chi * assign58530_body7_e91039);
        let assign58530_body7_e91041: f64 = (assign58530_body7_e91028 + assign58530_body7_e91040);
        let assign58530_body7_e91042: f64 = (locals.var_chi * assign58530_body7_e91041);
        let assign58530_body7_e91043: f64 = (0.707106781186548 + assign58530_body7_e91042);
        let assign58530_body7_e91044: f64 = (locals.var_chi * assign58530_body7_e91043);
        (assign58530_body7_e91044, ((locals.var_chi_dn0 * assign58530_body7_e91043) + (locals.var_chi * ((locals.var_chi_dn0 * assign58530_body7_e91041) + (locals.var_chi * ((locals.var_chi_dn0 * assign58530_body7_e91039) + (locals.var_chi * ((locals.var_chi_dn0 * assign58530_body7_e91037) + (locals.var_chi * (locals.var_chi_dn0 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn2 * assign58530_body7_e91043) + (locals.var_chi * ((locals.var_chi_dn2 * assign58530_body7_e91041) + (locals.var_chi * ((locals.var_chi_dn2 * assign58530_body7_e91039) + (locals.var_chi * ((locals.var_chi_dn2 * assign58530_body7_e91037) + (locals.var_chi * (locals.var_chi_dn2 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn4 * assign58530_body7_e91043) + (locals.var_chi * ((locals.var_chi_dn4 * assign58530_body7_e91041) + (locals.var_chi * ((locals.var_chi_dn4 * assign58530_body7_e91039) + (locals.var_chi * ((locals.var_chi_dn4 * assign58530_body7_e91037) + (locals.var_chi * (locals.var_chi_dn4 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn5 * assign58530_body7_e91043) + (locals.var_chi * ((locals.var_chi_dn5 * assign58530_body7_e91041) + (locals.var_chi * ((locals.var_chi_dn5 * assign58530_body7_e91039) + (locals.var_chi * ((locals.var_chi_dn5 * assign58530_body7_e91037) + (locals.var_chi * (locals.var_chi_dn5 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn6 * assign58530_body7_e91043) + (locals.var_chi * ((locals.var_chi_dn6 * assign58530_body7_e91041) + (locals.var_chi * ((locals.var_chi_dn6 * assign58530_body7_e91039) + (locals.var_chi * ((locals.var_chi_dn6 * assign58530_body7_e91037) + (locals.var_chi * (locals.var_chi_dn6 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn7 * assign58530_body7_e91043) + (locals.var_chi * ((locals.var_chi_dn7 * assign58530_body7_e91041) + (locals.var_chi * ((locals.var_chi_dn7 * assign58530_body7_e91039) + (locals.var_chi * ((locals.var_chi_dn7 * assign58530_body7_e91037) + (locals.var_chi * (locals.var_chi_dn7 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn8 * assign58530_body7_e91043) + (locals.var_chi * ((locals.var_chi_dn8 * assign58530_body7_e91041) + (locals.var_chi * ((locals.var_chi_dn8 * assign58530_body7_e91039) + (locals.var_chi * ((locals.var_chi_dn8 * assign58530_body7_e91037) + (locals.var_chi * (locals.var_chi_dn8 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn9 * assign58530_body7_e91043) + (locals.var_chi * ((locals.var_chi_dn9 * assign58530_body7_e91041) + (locals.var_chi * ((locals.var_chi_dn9 * assign58530_body7_e91039) + (locals.var_chi * ((locals.var_chi_dn9 * assign58530_body7_e91037) + (locals.var_chi * (locals.var_chi_dn9 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn10 * assign58530_body7_e91043) + (locals.var_chi * ((locals.var_chi_dn10 * assign58530_body7_e91041) + (locals.var_chi * ((locals.var_chi_dn10 * assign58530_body7_e91039) + (locals.var_chi * ((locals.var_chi_dn10 * assign58530_body7_e91037) + (locals.var_chi * (locals.var_chi_dn10 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn11 * assign58530_body7_e91043) + (locals.var_chi * ((locals.var_chi_dn11 * assign58530_body7_e91041) + (locals.var_chi * ((locals.var_chi_dn11 * assign58530_body7_e91039) + (locals.var_chi * ((locals.var_chi_dn11 * assign58530_body7_e91037) + (locals.var_chi * (locals.var_chi_dn11 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn14 * assign58530_body7_e91043) + (locals.var_chi * ((locals.var_chi_dn14 * assign58530_body7_e91041) + (locals.var_chi * ((locals.var_chi_dn14 * assign58530_body7_e91039) + (locals.var_chi * ((locals.var_chi_dn14 * assign58530_body7_e91037) + (locals.var_chi * (locals.var_chi_dn14 * 6.36964918866352e-5))))))))),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
            locals.var_fb = assign58530_body7_e91046;
            locals.var_fb_dn0 = assign58530_body7_e91046_d_n0;
            locals.var_fb_dn2 = assign58530_body7_e91046_d_n2;
            locals.var_fb_dn4 = assign58530_body7_e91046_d_n4;
            locals.var_fb_dn5 = assign58530_body7_e91046_d_n5;
            locals.var_fb_dn6 = assign58530_body7_e91046_d_n6;
            locals.var_fb_dn7 = assign58530_body7_e91046_d_n7;
            locals.var_fb_dn8 = assign58530_body7_e91046_d_n8;
            locals.var_fb_dn9 = assign58530_body7_e91046_d_n9;
            locals.var_fb_dn10 = assign58530_body7_e91046_d_n10;
            locals.var_fb_dn11 = assign58530_body7_e91046_d_n11;
            locals.var_fb_dn14 = assign58530_body7_e91046_d_n14;
            locals.var_fb_rv = 0.0;
            let (assign58530_body8_e91081, assign58530_body8_e91081_d_n0, assign58530_body8_e91081_d_n2, assign58530_body8_e91081_d_n4, assign58530_body8_e91081_d_n5, assign58530_body8_e91081_d_n6, assign58530_body8_e91081_d_n7, assign58530_body8_e91081_d_n8, assign58530_body8_e91081_d_n9, assign58530_body8_e91081_d_n10, assign58530_body8_e91081_d_n11, assign58530_body8_e91081_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1437 != 0.0)) {
        let assign58530_body8_e91057: f64 = (-0.117851130197758);
        let assign58530_body8_e91058: f64 = (2.0 * assign58530_body8_e91057);
        let assign58530_body8_e91062: f64 = (3.0 * 0.0178800506338833);
        let assign58530_body8_e91066: f64 = (-0.00163730162779191);
        let assign58530_body8_e91067: f64 = (4.0 * assign58530_body8_e91066);
        let assign58530_body8_e91070: f64 = (locals.var_chi * 5.0);
        let assign58530_body8_e91072: f64 = (assign58530_body8_e91070 * 6.36964918866352e-5);
        let assign58530_body8_e91073: f64 = (assign58530_body8_e91067 + assign58530_body8_e91072);
        let assign58530_body8_e91074: f64 = (locals.var_chi * assign58530_body8_e91073);
        let assign58530_body8_e91075: f64 = (assign58530_body8_e91062 + assign58530_body8_e91074);
        let assign58530_body8_e91076: f64 = (locals.var_chi * assign58530_body8_e91075);
        let assign58530_body8_e91077: f64 = (assign58530_body8_e91058 + assign58530_body8_e91076);
        let assign58530_body8_e91078: f64 = (locals.var_chi * assign58530_body8_e91077);
        let assign58530_body8_e91079: f64 = (0.707106781186548 + assign58530_body8_e91078);
        (assign58530_body8_e91079, ((locals.var_chi_dn0 * assign58530_body8_e91077) + (locals.var_chi * ((locals.var_chi_dn0 * assign58530_body8_e91075) + (locals.var_chi * ((locals.var_chi_dn0 * assign58530_body8_e91073) + (locals.var_chi * ((locals.var_chi_dn0 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn2 * assign58530_body8_e91077) + (locals.var_chi * ((locals.var_chi_dn2 * assign58530_body8_e91075) + (locals.var_chi * ((locals.var_chi_dn2 * assign58530_body8_e91073) + (locals.var_chi * ((locals.var_chi_dn2 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn4 * assign58530_body8_e91077) + (locals.var_chi * ((locals.var_chi_dn4 * assign58530_body8_e91075) + (locals.var_chi * ((locals.var_chi_dn4 * assign58530_body8_e91073) + (locals.var_chi * ((locals.var_chi_dn4 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn5 * assign58530_body8_e91077) + (locals.var_chi * ((locals.var_chi_dn5 * assign58530_body8_e91075) + (locals.var_chi * ((locals.var_chi_dn5 * assign58530_body8_e91073) + (locals.var_chi * ((locals.var_chi_dn5 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn6 * assign58530_body8_e91077) + (locals.var_chi * ((locals.var_chi_dn6 * assign58530_body8_e91075) + (locals.var_chi * ((locals.var_chi_dn6 * assign58530_body8_e91073) + (locals.var_chi * ((locals.var_chi_dn6 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn7 * assign58530_body8_e91077) + (locals.var_chi * ((locals.var_chi_dn7 * assign58530_body8_e91075) + (locals.var_chi * ((locals.var_chi_dn7 * assign58530_body8_e91073) + (locals.var_chi * ((locals.var_chi_dn7 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn8 * assign58530_body8_e91077) + (locals.var_chi * ((locals.var_chi_dn8 * assign58530_body8_e91075) + (locals.var_chi * ((locals.var_chi_dn8 * assign58530_body8_e91073) + (locals.var_chi * ((locals.var_chi_dn8 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn9 * assign58530_body8_e91077) + (locals.var_chi * ((locals.var_chi_dn9 * assign58530_body8_e91075) + (locals.var_chi * ((locals.var_chi_dn9 * assign58530_body8_e91073) + (locals.var_chi * ((locals.var_chi_dn9 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn10 * assign58530_body8_e91077) + (locals.var_chi * ((locals.var_chi_dn10 * assign58530_body8_e91075) + (locals.var_chi * ((locals.var_chi_dn10 * assign58530_body8_e91073) + (locals.var_chi * ((locals.var_chi_dn10 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn11 * assign58530_body8_e91077) + (locals.var_chi * ((locals.var_chi_dn11 * assign58530_body8_e91075) + (locals.var_chi * ((locals.var_chi_dn11 * assign58530_body8_e91073) + (locals.var_chi * ((locals.var_chi_dn11 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn14 * assign58530_body8_e91077) + (locals.var_chi * ((locals.var_chi_dn14 * assign58530_body8_e91075) + (locals.var_chi * ((locals.var_chi_dn14 * assign58530_body8_e91073) + (locals.var_chi * ((locals.var_chi_dn14 * 5.0) * 6.36964918866352e-5))))))),)
    } else {
        (locals.var_fb_dchi, locals.var_fb_dchi_dn0, locals.var_fb_dchi_dn2, locals.var_fb_dchi_dn4, locals.var_fb_dchi_dn5, locals.var_fb_dchi_dn6, locals.var_fb_dchi_dn7, locals.var_fb_dchi_dn8, locals.var_fb_dchi_dn9, locals.var_fb_dchi_dn10, locals.var_fb_dchi_dn11, locals.var_fb_dchi_dn14,)
    }
};
            locals.var_fb_dchi = assign58530_body8_e91081;
            locals.var_fb_dchi_dn0 = assign58530_body8_e91081_d_n0;
            locals.var_fb_dchi_dn2 = assign58530_body8_e91081_d_n2;
            locals.var_fb_dchi_dn4 = assign58530_body8_e91081_d_n4;
            locals.var_fb_dchi_dn5 = assign58530_body8_e91081_d_n5;
            locals.var_fb_dchi_dn6 = assign58530_body8_e91081_d_n6;
            locals.var_fb_dchi_dn7 = assign58530_body8_e91081_d_n7;
            locals.var_fb_dchi_dn8 = assign58530_body8_e91081_d_n8;
            locals.var_fb_dchi_dn9 = assign58530_body8_e91081_d_n9;
            locals.var_fb_dchi_dn10 = assign58530_body8_e91081_d_n10;
            locals.var_fb_dchi_dn11 = assign58530_body8_e91081_d_n11;
            locals.var_fb_dchi_dn14 = assign58530_body8_e91081_d_n14;
            locals.var_fb_dchi_rv = 0.0;
            let (assign58530_body9_e91095, assign58530_body9_e91095_d_n0, assign58530_body9_e91095_d_n2, assign58530_body9_e91095_d_n4, assign58530_body9_e91095_d_n5, assign58530_body9_e91095_d_n6, assign58530_body9_e91095_d_n7, assign58530_body9_e91095_d_n8, assign58530_body9_e91095_d_n9, assign58530_body9_e91095_d_n10, assign58530_body9_e91095_d_n11, assign58530_body9_e91095_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1437 != 0.0)) {
        let assign58530_body9_e91090: f64 = (locals.var_fb * locals.var_fb);
        let assign58530_body9_e91092: f64 = (assign58530_body9_e91090 + locals.var_fs01);
        let assign58530_body9_e91093: f64 = (assign58530_body9_e91092).sqrt();
        (assign58530_body9_e91093, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) + locals.var_fs01_dn0) / (2.0 * assign58530_body9_e91093)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) + locals.var_fs01_dn2) / (2.0 * assign58530_body9_e91093)), ((((locals.var_fb_dn4 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn4)) + locals.var_fs01_dn4) / (2.0 * assign58530_body9_e91093)), ((((locals.var_fb_dn5 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn5)) + locals.var_fs01_dn5) / (2.0 * assign58530_body9_e91093)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) + locals.var_fs01_dn6) / (2.0 * assign58530_body9_e91093)), ((((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)) + locals.var_fs01_dn7) / (2.0 * assign58530_body9_e91093)), ((((locals.var_fb_dn8 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn8)) + locals.var_fs01_dn8) / (2.0 * assign58530_body9_e91093)), ((((locals.var_fb_dn9 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn9)) + locals.var_fs01_dn9) / (2.0 * assign58530_body9_e91093)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) + locals.var_fs01_dn10) / (2.0 * assign58530_body9_e91093)), ((((locals.var_fb_dn11 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn11)) + locals.var_fs01_dn11) / (2.0 * assign58530_body9_e91093)), ((((locals.var_fb_dn14 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn14)) + locals.var_fs01_dn14) / (2.0 * assign58530_body9_e91093)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign58530_body9_e91095;
            locals.var_fs02_dn0 = assign58530_body9_e91095_d_n0;
            locals.var_fs02_dn2 = assign58530_body9_e91095_d_n2;
            locals.var_fs02_dn4 = assign58530_body9_e91095_d_n4;
            locals.var_fs02_dn5 = assign58530_body9_e91095_d_n5;
            locals.var_fs02_dn6 = assign58530_body9_e91095_d_n6;
            locals.var_fs02_dn7 = assign58530_body9_e91095_d_n7;
            locals.var_fs02_dn8 = assign58530_body9_e91095_d_n8;
            locals.var_fs02_dn9 = assign58530_body9_e91095_d_n9;
            locals.var_fs02_dn10 = assign58530_body9_e91095_d_n10;
            locals.var_fs02_dn11 = assign58530_body9_e91095_d_n11;
            locals.var_fs02_dn14 = assign58530_body9_e91095_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign58530_body10_e91116, assign58530_body10_e91116_d_n0, assign58530_body10_e91116_d_n2, assign58530_body10_e91116_d_n4, assign58530_body10_e91116_d_n5, assign58530_body10_e91116_d_n6, assign58530_body10_e91116_d_n7, assign58530_body10_e91116_d_n8, assign58530_body10_e91116_d_n9, assign58530_body10_e91116_d_n10, assign58530_body10_e91116_d_n11, assign58530_body10_e91116_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1437 != 0.0)) {
        let assign58530_body10_e91104: f64 = (locals.var_beta * locals.var_fb_dchi);
        let assign58530_body10_e91106: f64 = (assign58530_body10_e91104 * 2.0);
        let assign58530_body10_e91108: f64 = (assign58530_body10_e91106 * locals.var_fb);
        let assign58530_body10_e91110: f64 = (assign58530_body10_e91108 + locals.var_fs01_dps0);
        let assign58530_body10_e91113: f64 = (locals.var_fs02 + locals.var_fs02);
        let assign58530_body10_e91114: f64 = (assign58530_body10_e91110 / assign58530_body10_e91113);
        (assign58530_body10_e91114, (((((((((locals.var_beta_dn0 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn0)) * 2.0) * locals.var_fb) + (assign58530_body10_e91106 * locals.var_fb_dn0)) + locals.var_fs01_dps0_dn0) * assign58530_body10_e91113) - (assign58530_body10_e91110 * (locals.var_fs02_dn0 + locals.var_fs02_dn0))) / (assign58530_body10_e91113 * assign58530_body10_e91113)), (((((((((locals.var_beta_dn2 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn2)) * 2.0) * locals.var_fb) + (assign58530_body10_e91106 * locals.var_fb_dn2)) + locals.var_fs01_dps0_dn2) * assign58530_body10_e91113) - (assign58530_body10_e91110 * (locals.var_fs02_dn2 + locals.var_fs02_dn2))) / (assign58530_body10_e91113 * assign58530_body10_e91113)), (((((((((locals.var_beta_dn4 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn4)) * 2.0) * locals.var_fb) + (assign58530_body10_e91106 * locals.var_fb_dn4)) + locals.var_fs01_dps0_dn4) * assign58530_body10_e91113) - (assign58530_body10_e91110 * (locals.var_fs02_dn4 + locals.var_fs02_dn4))) / (assign58530_body10_e91113 * assign58530_body10_e91113)), (((((((((locals.var_beta_dn5 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn5)) * 2.0) * locals.var_fb) + (assign58530_body10_e91106 * locals.var_fb_dn5)) + locals.var_fs01_dps0_dn5) * assign58530_body10_e91113) - (assign58530_body10_e91110 * (locals.var_fs02_dn5 + locals.var_fs02_dn5))) / (assign58530_body10_e91113 * assign58530_body10_e91113)), (((((((((locals.var_beta_dn6 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn6)) * 2.0) * locals.var_fb) + (assign58530_body10_e91106 * locals.var_fb_dn6)) + locals.var_fs01_dps0_dn6) * assign58530_body10_e91113) - (assign58530_body10_e91110 * (locals.var_fs02_dn6 + locals.var_fs02_dn6))) / (assign58530_body10_e91113 * assign58530_body10_e91113)), (((((((((locals.var_beta_dn7 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn7)) * 2.0) * locals.var_fb) + (assign58530_body10_e91106 * locals.var_fb_dn7)) + locals.var_fs01_dps0_dn7) * assign58530_body10_e91113) - (assign58530_body10_e91110 * (locals.var_fs02_dn7 + locals.var_fs02_dn7))) / (assign58530_body10_e91113 * assign58530_body10_e91113)), (((((((((locals.var_beta_dn8 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn8)) * 2.0) * locals.var_fb) + (assign58530_body10_e91106 * locals.var_fb_dn8)) + locals.var_fs01_dps0_dn8) * assign58530_body10_e91113) - (assign58530_body10_e91110 * (locals.var_fs02_dn8 + locals.var_fs02_dn8))) / (assign58530_body10_e91113 * assign58530_body10_e91113)), (((((((((locals.var_beta_dn9 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn9)) * 2.0) * locals.var_fb) + (assign58530_body10_e91106 * locals.var_fb_dn9)) + locals.var_fs01_dps0_dn9) * assign58530_body10_e91113) - (assign58530_body10_e91110 * (locals.var_fs02_dn9 + locals.var_fs02_dn9))) / (assign58530_body10_e91113 * assign58530_body10_e91113)), (((((((((locals.var_beta_dn10 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn10)) * 2.0) * locals.var_fb) + (assign58530_body10_e91106 * locals.var_fb_dn10)) + locals.var_fs01_dps0_dn10) * assign58530_body10_e91113) - (assign58530_body10_e91110 * (locals.var_fs02_dn10 + locals.var_fs02_dn10))) / (assign58530_body10_e91113 * assign58530_body10_e91113)), (((((((((locals.var_beta_dn11 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn11)) * 2.0) * locals.var_fb) + (assign58530_body10_e91106 * locals.var_fb_dn11)) + locals.var_fs01_dps0_dn11) * assign58530_body10_e91113) - (assign58530_body10_e91110 * (locals.var_fs02_dn11 + locals.var_fs02_dn11))) / (assign58530_body10_e91113 * assign58530_body10_e91113)), (((((((((locals.var_beta_dn14 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn14)) * 2.0) * locals.var_fb) + (assign58530_body10_e91106 * locals.var_fb_dn14)) + locals.var_fs01_dps0_dn14) * assign58530_body10_e91113) - (assign58530_body10_e91110 * (locals.var_fs02_dn14 + locals.var_fs02_dn14))) / (assign58530_body10_e91113 * assign58530_body10_e91113)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign58530_body10_e91116;
            locals.var_fs02_dps0_dn0 = assign58530_body10_e91116_d_n0;
            locals.var_fs02_dps0_dn2 = assign58530_body10_e91116_d_n2;
            locals.var_fs02_dps0_dn4 = assign58530_body10_e91116_d_n4;
            locals.var_fs02_dps0_dn5 = assign58530_body10_e91116_d_n5;
            locals.var_fs02_dps0_dn6 = assign58530_body10_e91116_d_n6;
            locals.var_fs02_dps0_dn7 = assign58530_body10_e91116_d_n7;
            locals.var_fs02_dps0_dn8 = assign58530_body10_e91116_d_n8;
            locals.var_fs02_dps0_dn9 = assign58530_body10_e91116_d_n9;
            locals.var_fs02_dps0_dn10 = assign58530_body10_e91116_d_n10;
            locals.var_fs02_dps0_dn11 = assign58530_body10_e91116_d_n11;
            locals.var_fs02_dps0_dn14 = assign58530_body10_e91116_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let assign58530_body11_e91119: f64 = if locals.var_chi < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard1438 = assign58530_body11_e91119;
            locals.var_guard1438_rv = 0.0;
            let (assign58530_body12_e91132, assign58530_body12_e91132_d_n0, assign58530_body12_e91132_d_n2, assign58530_body12_e91132_d_n4, assign58530_body12_e91132_d_n5, assign58530_body12_e91132_d_n6, assign58530_body12_e91132_d_n7, assign58530_body12_e91132_d_n8, assign58530_body12_e91132_d_n9, assign58530_body12_e91132_d_n10, assign58530_body12_e91132_d_n11, assign58530_body12_e91132_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1437 == 0.0)) && (locals.var_guard1438 != 0.0)) {
        let assign58530_body12_e91130: f64 = (locals.var_chi).exp();
        (assign58530_body12_e91130, (assign58530_body12_e91130 * locals.var_chi_dn0), (assign58530_body12_e91130 * locals.var_chi_dn2), (assign58530_body12_e91130 * locals.var_chi_dn4), (assign58530_body12_e91130 * locals.var_chi_dn5), (assign58530_body12_e91130 * locals.var_chi_dn6), (assign58530_body12_e91130 * locals.var_chi_dn7), (assign58530_body12_e91130 * locals.var_chi_dn8), (assign58530_body12_e91130 * locals.var_chi_dn9), (assign58530_body12_e91130 * locals.var_chi_dn10), (assign58530_body12_e91130 * locals.var_chi_dn11), (assign58530_body12_e91130 * locals.var_chi_dn14),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn14,)
    }
};
            locals.var_exp_chi = assign58530_body12_e91132;
            locals.var_exp_chi_dn0 = assign58530_body12_e91132_d_n0;
            locals.var_exp_chi_dn2 = assign58530_body12_e91132_d_n2;
            locals.var_exp_chi_dn4 = assign58530_body12_e91132_d_n4;
            locals.var_exp_chi_dn5 = assign58530_body12_e91132_d_n5;
            locals.var_exp_chi_dn6 = assign58530_body12_e91132_d_n6;
            locals.var_exp_chi_dn7 = assign58530_body12_e91132_d_n7;
            locals.var_exp_chi_dn8 = assign58530_body12_e91132_d_n8;
            locals.var_exp_chi_dn9 = assign58530_body12_e91132_d_n9;
            locals.var_exp_chi_dn10 = assign58530_body12_e91132_d_n10;
            locals.var_exp_chi_dn11 = assign58530_body12_e91132_d_n11;
            locals.var_exp_chi_dn14 = assign58530_body12_e91132_d_n14;
            locals.var_exp_chi_rv = 0.0;
            let (assign58530_body13_e91148, assign58530_body13_e91148_d_n0, assign58530_body13_e91148_d_n2, assign58530_body13_e91148_d_n4, assign58530_body13_e91148_d_n5, assign58530_body13_e91148_d_n6, assign58530_body13_e91148_d_n7, assign58530_body13_e91148_d_n8, assign58530_body13_e91148_d_n9, assign58530_body13_e91148_d_n10, assign58530_body13_e91148_d_n11, assign58530_body13_e91148_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1437 == 0.0)) && (locals.var_guard1438 != 0.0)) {
        let assign58530_body13_e91145: f64 = (locals.var_exp_chi - 1.0);
        let assign58530_body13_e91146: f64 = (locals.var_cfs1 * assign58530_body13_e91145);
        (assign58530_body13_e91146, ((locals.var_cfs1_dn0 * assign58530_body13_e91145) + (locals.var_cfs1 * locals.var_exp_chi_dn0)), ((locals.var_cfs1_dn2 * assign58530_body13_e91145) + (locals.var_cfs1 * locals.var_exp_chi_dn2)), ((locals.var_cfs1_dn4 * assign58530_body13_e91145) + (locals.var_cfs1 * locals.var_exp_chi_dn4)), ((locals.var_cfs1_dn5 * assign58530_body13_e91145) + (locals.var_cfs1 * locals.var_exp_chi_dn5)), ((locals.var_cfs1_dn6 * assign58530_body13_e91145) + (locals.var_cfs1 * locals.var_exp_chi_dn6)), ((locals.var_cfs1_dn7 * assign58530_body13_e91145) + (locals.var_cfs1 * locals.var_exp_chi_dn7)), ((locals.var_cfs1_dn8 * assign58530_body13_e91145) + (locals.var_cfs1 * locals.var_exp_chi_dn8)), ((locals.var_cfs1_dn9 * assign58530_body13_e91145) + (locals.var_cfs1 * locals.var_exp_chi_dn9)), ((locals.var_cfs1_dn10 * assign58530_body13_e91145) + (locals.var_cfs1 * locals.var_exp_chi_dn10)), ((locals.var_cfs1_dn11 * assign58530_body13_e91145) + (locals.var_cfs1 * locals.var_exp_chi_dn11)), ((locals.var_cfs1_dn14 * assign58530_body13_e91145) + (locals.var_cfs1 * locals.var_exp_chi_dn14)),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign58530_body13_e91148;
            locals.var_fs01_dn0 = assign58530_body13_e91148_d_n0;
            locals.var_fs01_dn2 = assign58530_body13_e91148_d_n2;
            locals.var_fs01_dn4 = assign58530_body13_e91148_d_n4;
            locals.var_fs01_dn5 = assign58530_body13_e91148_d_n5;
            locals.var_fs01_dn6 = assign58530_body13_e91148_d_n6;
            locals.var_fs01_dn7 = assign58530_body13_e91148_d_n7;
            locals.var_fs01_dn8 = assign58530_body13_e91148_d_n8;
            locals.var_fs01_dn9 = assign58530_body13_e91148_d_n9;
            locals.var_fs01_dn10 = assign58530_body13_e91148_d_n10;
            locals.var_fs01_dn11 = assign58530_body13_e91148_d_n11;
            locals.var_fs01_dn14 = assign58530_body13_e91148_d_n14;
            locals.var_fs01_rv = 0.0;
            let (assign58530_body14_e91164, assign58530_body14_e91164_d_n0, assign58530_body14_e91164_d_n2, assign58530_body14_e91164_d_n4, assign58530_body14_e91164_d_n5, assign58530_body14_e91164_d_n6, assign58530_body14_e91164_d_n7, assign58530_body14_e91164_d_n8, assign58530_body14_e91164_d_n9, assign58530_body14_e91164_d_n10, assign58530_body14_e91164_d_n11, assign58530_body14_e91164_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1437 == 0.0)) && (locals.var_guard1438 != 0.0)) {
        let assign58530_body14_e91160: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign58530_body14_e91162: f64 = (assign58530_body14_e91160 * locals.var_exp_chi);
        (assign58530_body14_e91162, ((((locals.var_cfs1_dn0 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn0)) * locals.var_exp_chi) + (assign58530_body14_e91160 * locals.var_exp_chi_dn0)), ((((locals.var_cfs1_dn2 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn2)) * locals.var_exp_chi) + (assign58530_body14_e91160 * locals.var_exp_chi_dn2)), ((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * locals.var_exp_chi) + (assign58530_body14_e91160 * locals.var_exp_chi_dn4)), ((((locals.var_cfs1_dn5 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn5)) * locals.var_exp_chi) + (assign58530_body14_e91160 * locals.var_exp_chi_dn5)), ((((locals.var_cfs1_dn6 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn6)) * locals.var_exp_chi) + (assign58530_body14_e91160 * locals.var_exp_chi_dn6)), ((((locals.var_cfs1_dn7 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn7)) * locals.var_exp_chi) + (assign58530_body14_e91160 * locals.var_exp_chi_dn7)), ((((locals.var_cfs1_dn8 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn8)) * locals.var_exp_chi) + (assign58530_body14_e91160 * locals.var_exp_chi_dn8)), ((((locals.var_cfs1_dn9 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn9)) * locals.var_exp_chi) + (assign58530_body14_e91160 * locals.var_exp_chi_dn9)), ((((locals.var_cfs1_dn10 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn10)) * locals.var_exp_chi) + (assign58530_body14_e91160 * locals.var_exp_chi_dn10)), ((((locals.var_cfs1_dn11 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn11)) * locals.var_exp_chi) + (assign58530_body14_e91160 * locals.var_exp_chi_dn11)), ((((locals.var_cfs1_dn14 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn14)) * locals.var_exp_chi) + (assign58530_body14_e91160 * locals.var_exp_chi_dn14)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign58530_body14_e91164;
            locals.var_fs01_dps0_dn0 = assign58530_body14_e91164_d_n0;
            locals.var_fs01_dps0_dn2 = assign58530_body14_e91164_d_n2;
            locals.var_fs01_dps0_dn4 = assign58530_body14_e91164_d_n4;
            locals.var_fs01_dps0_dn5 = assign58530_body14_e91164_d_n5;
            locals.var_fs01_dps0_dn6 = assign58530_body14_e91164_d_n6;
            locals.var_fs01_dps0_dn7 = assign58530_body14_e91164_d_n7;
            locals.var_fs01_dps0_dn8 = assign58530_body14_e91164_d_n8;
            locals.var_fs01_dps0_dn9 = assign58530_body14_e91164_d_n9;
            locals.var_fs01_dps0_dn10 = assign58530_body14_e91164_d_n10;
            locals.var_fs01_dps0_dn11 = assign58530_body14_e91164_d_n11;
            locals.var_fs01_dps0_dn14 = assign58530_body14_e91164_d_n14;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign58530_body15_e91180, assign58530_body15_e91180_d_n0, assign58530_body15_e91180_d_n2, assign58530_body15_e91180_d_n4, assign58530_body15_e91180_d_n5, assign58530_body15_e91180_d_n6, assign58530_body15_e91180_d_n7, assign58530_body15_e91180_d_n8, assign58530_body15_e91180_d_n9, assign58530_body15_e91180_d_n10, assign58530_body15_e91180_d_n11, assign58530_body15_e91180_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1437 == 0.0)) && (locals.var_guard1438 == 0.0)) {
        let assign58530_body15_e91177: f64 = (locals.var_beta * locals.var_ps0);
        let assign58530_body15_e91178: f64 = (assign58530_body15_e91177).exp();
        (assign58530_body15_e91178, (assign58530_body15_e91178 * ((locals.var_beta_dn0 * locals.var_ps0) + (locals.var_beta * locals.var_ps0_dn0))), (assign58530_body15_e91178 * ((locals.var_beta_dn2 * locals.var_ps0) + (locals.var_beta * locals.var_ps0_dn2))), (assign58530_body15_e91178 * ((locals.var_beta_dn4 * locals.var_ps0) + (locals.var_beta * locals.var_ps0_dn4))), (assign58530_body15_e91178 * ((locals.var_beta_dn5 * locals.var_ps0) + (locals.var_beta * locals.var_ps0_dn5))), (assign58530_body15_e91178 * ((locals.var_beta_dn6 * locals.var_ps0) + (locals.var_beta * locals.var_ps0_dn6))), (assign58530_body15_e91178 * ((locals.var_beta_dn7 * locals.var_ps0) + (locals.var_beta * locals.var_ps0_dn7))), (assign58530_body15_e91178 * ((locals.var_beta_dn8 * locals.var_ps0) + (locals.var_beta * locals.var_ps0_dn8))), (assign58530_body15_e91178 * ((locals.var_beta_dn9 * locals.var_ps0) + (locals.var_beta * locals.var_ps0_dn9))), (assign58530_body15_e91178 * ((locals.var_beta_dn10 * locals.var_ps0) + (locals.var_beta * locals.var_ps0_dn10))), (assign58530_body15_e91178 * ((locals.var_beta_dn11 * locals.var_ps0) + (locals.var_beta * locals.var_ps0_dn11))), (assign58530_body15_e91178 * ((locals.var_beta_dn14 * locals.var_ps0) + (locals.var_beta * locals.var_ps0_dn14))),)
    } else {
        (locals.var_exp_bps0, locals.var_exp_bps0_dn0, locals.var_exp_bps0_dn2, locals.var_exp_bps0_dn4, locals.var_exp_bps0_dn5, locals.var_exp_bps0_dn6, locals.var_exp_bps0_dn7, locals.var_exp_bps0_dn8, locals.var_exp_bps0_dn9, locals.var_exp_bps0_dn10, locals.var_exp_bps0_dn11, locals.var_exp_bps0_dn14,)
    }
};
            locals.var_exp_bps0 = assign58530_body15_e91180;
            locals.var_exp_bps0_dn0 = assign58530_body15_e91180_d_n0;
            locals.var_exp_bps0_dn2 = assign58530_body15_e91180_d_n2;
            locals.var_exp_bps0_dn4 = assign58530_body15_e91180_d_n4;
            locals.var_exp_bps0_dn5 = assign58530_body15_e91180_d_n5;
            locals.var_exp_bps0_dn6 = assign58530_body15_e91180_d_n6;
            locals.var_exp_bps0_dn7 = assign58530_body15_e91180_d_n7;
            locals.var_exp_bps0_dn8 = assign58530_body15_e91180_d_n8;
            locals.var_exp_bps0_dn9 = assign58530_body15_e91180_d_n9;
            locals.var_exp_bps0_dn10 = assign58530_body15_e91180_d_n10;
            locals.var_exp_bps0_dn11 = assign58530_body15_e91180_d_n11;
            locals.var_exp_bps0_dn14 = assign58530_body15_e91180_d_n14;
            locals.var_exp_bps0_rv = 0.0;
            let (assign58530_body16_e91197, assign58530_body16_e91197_d_n0, assign58530_body16_e91197_d_n2, assign58530_body16_e91197_d_n4, assign58530_body16_e91197_d_n5, assign58530_body16_e91197_d_n6, assign58530_body16_e91197_d_n7, assign58530_body16_e91197_d_n8, assign58530_body16_e91197_d_n9, assign58530_body16_e91197_d_n10, assign58530_body16_e91197_d_n11, assign58530_body16_e91197_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1437 == 0.0)) && (locals.var_guard1438 == 0.0)) {
        let assign58530_body16_e91194: f64 = (locals.var_exp_bps0 - locals.var_exp_bvbs);
        let assign58530_body16_e91195: f64 = (locals.var_cnst1 * assign58530_body16_e91194);
        (assign58530_body16_e91195, ((locals.var_cnst1_dn0 * assign58530_body16_e91194) + (locals.var_cnst1 * (locals.var_exp_bps0_dn0 - locals.var_exp_bvbs_dn0))), ((locals.var_cnst1_dn2 * assign58530_body16_e91194) + (locals.var_cnst1 * (locals.var_exp_bps0_dn2 - locals.var_exp_bvbs_dn2))), ((locals.var_cnst1_dn4 * assign58530_body16_e91194) + (locals.var_cnst1 * (locals.var_exp_bps0_dn4 - locals.var_exp_bvbs_dn4))), ((locals.var_cnst1_dn5 * assign58530_body16_e91194) + (locals.var_cnst1 * (locals.var_exp_bps0_dn5 - locals.var_exp_bvbs_dn5))), ((locals.var_cnst1_dn6 * assign58530_body16_e91194) + (locals.var_cnst1 * (locals.var_exp_bps0_dn6 - locals.var_exp_bvbs_dn6))), ((locals.var_cnst1_dn7 * assign58530_body16_e91194) + (locals.var_cnst1 * (locals.var_exp_bps0_dn7 - locals.var_exp_bvbs_dn7))), ((locals.var_cnst1_dn8 * assign58530_body16_e91194) + (locals.var_cnst1 * (locals.var_exp_bps0_dn8 - locals.var_exp_bvbs_dn8))), ((locals.var_cnst1_dn9 * assign58530_body16_e91194) + (locals.var_cnst1 * (locals.var_exp_bps0_dn9 - locals.var_exp_bvbs_dn9))), ((locals.var_cnst1_dn10 * assign58530_body16_e91194) + (locals.var_cnst1 * (locals.var_exp_bps0_dn10 - locals.var_exp_bvbs_dn10))), ((locals.var_cnst1_dn11 * assign58530_body16_e91194) + (locals.var_cnst1 * (locals.var_exp_bps0_dn11 - locals.var_exp_bvbs_dn11))), ((locals.var_cnst1_dn14 * assign58530_body16_e91194) + (locals.var_cnst1 * (locals.var_exp_bps0_dn14 - locals.var_exp_bvbs_dn14))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign58530_body16_e91197;
            locals.var_fs01_dn0 = assign58530_body16_e91197_d_n0;
            locals.var_fs01_dn2 = assign58530_body16_e91197_d_n2;
            locals.var_fs01_dn4 = assign58530_body16_e91197_d_n4;
            locals.var_fs01_dn5 = assign58530_body16_e91197_d_n5;
            locals.var_fs01_dn6 = assign58530_body16_e91197_d_n6;
            locals.var_fs01_dn7 = assign58530_body16_e91197_d_n7;
            locals.var_fs01_dn8 = assign58530_body16_e91197_d_n8;
            locals.var_fs01_dn9 = assign58530_body16_e91197_d_n9;
            locals.var_fs01_dn10 = assign58530_body16_e91197_d_n10;
            locals.var_fs01_dn11 = assign58530_body16_e91197_d_n11;
            locals.var_fs01_dn14 = assign58530_body16_e91197_d_n14;
            locals.var_fs01_rv = 0.0;
            let (assign58530_body17_e91214, assign58530_body17_e91214_d_n0, assign58530_body17_e91214_d_n2, assign58530_body17_e91214_d_n4, assign58530_body17_e91214_d_n5, assign58530_body17_e91214_d_n6, assign58530_body17_e91214_d_n7, assign58530_body17_e91214_d_n8, assign58530_body17_e91214_d_n9, assign58530_body17_e91214_d_n10, assign58530_body17_e91214_d_n11, assign58530_body17_e91214_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1437 == 0.0)) && (locals.var_guard1438 == 0.0)) {
        let assign58530_body17_e91210: f64 = (locals.var_cnst1 * locals.var_beta);
        let assign58530_body17_e91212: f64 = (assign58530_body17_e91210 * locals.var_exp_bps0);
        (assign58530_body17_e91212, ((((locals.var_cnst1_dn0 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn0)) * locals.var_exp_bps0) + (assign58530_body17_e91210 * locals.var_exp_bps0_dn0)), ((((locals.var_cnst1_dn2 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn2)) * locals.var_exp_bps0) + (assign58530_body17_e91210 * locals.var_exp_bps0_dn2)), ((((locals.var_cnst1_dn4 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn4)) * locals.var_exp_bps0) + (assign58530_body17_e91210 * locals.var_exp_bps0_dn4)), ((((locals.var_cnst1_dn5 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn5)) * locals.var_exp_bps0) + (assign58530_body17_e91210 * locals.var_exp_bps0_dn5)), ((((locals.var_cnst1_dn6 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn6)) * locals.var_exp_bps0) + (assign58530_body17_e91210 * locals.var_exp_bps0_dn6)), ((((locals.var_cnst1_dn7 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn7)) * locals.var_exp_bps0) + (assign58530_body17_e91210 * locals.var_exp_bps0_dn7)), ((((locals.var_cnst1_dn8 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn8)) * locals.var_exp_bps0) + (assign58530_body17_e91210 * locals.var_exp_bps0_dn8)), ((((locals.var_cnst1_dn9 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn9)) * locals.var_exp_bps0) + (assign58530_body17_e91210 * locals.var_exp_bps0_dn9)), ((((locals.var_cnst1_dn10 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn10)) * locals.var_exp_bps0) + (assign58530_body17_e91210 * locals.var_exp_bps0_dn10)), ((((locals.var_cnst1_dn11 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn11)) * locals.var_exp_bps0) + (assign58530_body17_e91210 * locals.var_exp_bps0_dn11)), ((((locals.var_cnst1_dn14 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn14)) * locals.var_exp_bps0) + (assign58530_body17_e91210 * locals.var_exp_bps0_dn14)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign58530_body17_e91214;
            locals.var_fs01_dps0_dn0 = assign58530_body17_e91214_d_n0;
            locals.var_fs01_dps0_dn2 = assign58530_body17_e91214_d_n2;
            locals.var_fs01_dps0_dn4 = assign58530_body17_e91214_d_n4;
            locals.var_fs01_dps0_dn5 = assign58530_body17_e91214_d_n5;
            locals.var_fs01_dps0_dn6 = assign58530_body17_e91214_d_n6;
            locals.var_fs01_dps0_dn7 = assign58530_body17_e91214_d_n7;
            locals.var_fs01_dps0_dn8 = assign58530_body17_e91214_d_n8;
            locals.var_fs01_dps0_dn9 = assign58530_body17_e91214_d_n9;
            locals.var_fs01_dps0_dn10 = assign58530_body17_e91214_d_n10;
            locals.var_fs01_dps0_dn11 = assign58530_body17_e91214_d_n11;
            locals.var_fs01_dps0_dn14 = assign58530_body17_e91214_d_n14;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign58530_body18_e91229, assign58530_body18_e91229_d_n0, assign58530_body18_e91229_d_n2, assign58530_body18_e91229_d_n4, assign58530_body18_e91229_d_n5, assign58530_body18_e91229_d_n6, assign58530_body18_e91229_d_n7, assign58530_body18_e91229_d_n8, assign58530_body18_e91229_d_n9, assign58530_body18_e91229_d_n10, assign58530_body18_e91229_d_n11, assign58530_body18_e91229_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1437 == 0.0)) {
        let assign58530_body18_e91224: f64 = (locals.var_chi - 1.0);
        let assign58530_body18_e91226: f64 = (assign58530_body18_e91224 + locals.var_fs01);
        let assign58530_body18_e91227: f64 = (assign58530_body18_e91226).sqrt();
        (assign58530_body18_e91227, ((locals.var_chi_dn0 + locals.var_fs01_dn0) / (2.0 * assign58530_body18_e91227)), ((locals.var_chi_dn2 + locals.var_fs01_dn2) / (2.0 * assign58530_body18_e91227)), ((locals.var_chi_dn4 + locals.var_fs01_dn4) / (2.0 * assign58530_body18_e91227)), ((locals.var_chi_dn5 + locals.var_fs01_dn5) / (2.0 * assign58530_body18_e91227)), ((locals.var_chi_dn6 + locals.var_fs01_dn6) / (2.0 * assign58530_body18_e91227)), ((locals.var_chi_dn7 + locals.var_fs01_dn7) / (2.0 * assign58530_body18_e91227)), ((locals.var_chi_dn8 + locals.var_fs01_dn8) / (2.0 * assign58530_body18_e91227)), ((locals.var_chi_dn9 + locals.var_fs01_dn9) / (2.0 * assign58530_body18_e91227)), ((locals.var_chi_dn10 + locals.var_fs01_dn10) / (2.0 * assign58530_body18_e91227)), ((locals.var_chi_dn11 + locals.var_fs01_dn11) / (2.0 * assign58530_body18_e91227)), ((locals.var_chi_dn14 + locals.var_fs01_dn14) / (2.0 * assign58530_body18_e91227)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign58530_body18_e91229;
            locals.var_fs02_dn0 = assign58530_body18_e91229_d_n0;
            locals.var_fs02_dn2 = assign58530_body18_e91229_d_n2;
            locals.var_fs02_dn4 = assign58530_body18_e91229_d_n4;
            locals.var_fs02_dn5 = assign58530_body18_e91229_d_n5;
            locals.var_fs02_dn6 = assign58530_body18_e91229_d_n6;
            locals.var_fs02_dn7 = assign58530_body18_e91229_d_n7;
            locals.var_fs02_dn8 = assign58530_body18_e91229_d_n8;
            locals.var_fs02_dn9 = assign58530_body18_e91229_d_n9;
            locals.var_fs02_dn10 = assign58530_body18_e91229_d_n10;
            locals.var_fs02_dn11 = assign58530_body18_e91229_d_n11;
            locals.var_fs02_dn14 = assign58530_body18_e91229_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign58530_body19_e91245, assign58530_body19_e91245_d_n0, assign58530_body19_e91245_d_n2, assign58530_body19_e91245_d_n4, assign58530_body19_e91245_d_n5, assign58530_body19_e91245_d_n6, assign58530_body19_e91245_d_n7, assign58530_body19_e91245_d_n8, assign58530_body19_e91245_d_n9, assign58530_body19_e91245_d_n10, assign58530_body19_e91245_d_n11, assign58530_body19_e91245_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1437 == 0.0)) {
        let assign58530_body19_e91239: f64 = (locals.var_beta + locals.var_fs01_dps0);
        let assign58530_body19_e91242: f64 = (locals.var_fs02 + locals.var_fs02);
        let assign58530_body19_e91243: f64 = (assign58530_body19_e91239 / assign58530_body19_e91242);
        (assign58530_body19_e91243, ((((locals.var_beta_dn0 + locals.var_fs01_dps0_dn0) * assign58530_body19_e91242) - (assign58530_body19_e91239 * (locals.var_fs02_dn0 + locals.var_fs02_dn0))) / (assign58530_body19_e91242 * assign58530_body19_e91242)), ((((locals.var_beta_dn2 + locals.var_fs01_dps0_dn2) * assign58530_body19_e91242) - (assign58530_body19_e91239 * (locals.var_fs02_dn2 + locals.var_fs02_dn2))) / (assign58530_body19_e91242 * assign58530_body19_e91242)), ((((locals.var_beta_dn4 + locals.var_fs01_dps0_dn4) * assign58530_body19_e91242) - (assign58530_body19_e91239 * (locals.var_fs02_dn4 + locals.var_fs02_dn4))) / (assign58530_body19_e91242 * assign58530_body19_e91242)), ((((locals.var_beta_dn5 + locals.var_fs01_dps0_dn5) * assign58530_body19_e91242) - (assign58530_body19_e91239 * (locals.var_fs02_dn5 + locals.var_fs02_dn5))) / (assign58530_body19_e91242 * assign58530_body19_e91242)), ((((locals.var_beta_dn6 + locals.var_fs01_dps0_dn6) * assign58530_body19_e91242) - (assign58530_body19_e91239 * (locals.var_fs02_dn6 + locals.var_fs02_dn6))) / (assign58530_body19_e91242 * assign58530_body19_e91242)), ((((locals.var_beta_dn7 + locals.var_fs01_dps0_dn7) * assign58530_body19_e91242) - (assign58530_body19_e91239 * (locals.var_fs02_dn7 + locals.var_fs02_dn7))) / (assign58530_body19_e91242 * assign58530_body19_e91242)), ((((locals.var_beta_dn8 + locals.var_fs01_dps0_dn8) * assign58530_body19_e91242) - (assign58530_body19_e91239 * (locals.var_fs02_dn8 + locals.var_fs02_dn8))) / (assign58530_body19_e91242 * assign58530_body19_e91242)), ((((locals.var_beta_dn9 + locals.var_fs01_dps0_dn9) * assign58530_body19_e91242) - (assign58530_body19_e91239 * (locals.var_fs02_dn9 + locals.var_fs02_dn9))) / (assign58530_body19_e91242 * assign58530_body19_e91242)), ((((locals.var_beta_dn10 + locals.var_fs01_dps0_dn10) * assign58530_body19_e91242) - (assign58530_body19_e91239 * (locals.var_fs02_dn10 + locals.var_fs02_dn10))) / (assign58530_body19_e91242 * assign58530_body19_e91242)), ((((locals.var_beta_dn11 + locals.var_fs01_dps0_dn11) * assign58530_body19_e91242) - (assign58530_body19_e91239 * (locals.var_fs02_dn11 + locals.var_fs02_dn11))) / (assign58530_body19_e91242 * assign58530_body19_e91242)), ((((locals.var_beta_dn14 + locals.var_fs01_dps0_dn14) * assign58530_body19_e91242) - (assign58530_body19_e91239 * (locals.var_fs02_dn14 + locals.var_fs02_dn14))) / (assign58530_body19_e91242 * assign58530_body19_e91242)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign58530_body19_e91245;
            locals.var_fs02_dps0_dn0 = assign58530_body19_e91245_d_n0;
            locals.var_fs02_dps0_dn2 = assign58530_body19_e91245_d_n2;
            locals.var_fs02_dps0_dn4 = assign58530_body19_e91245_d_n4;
            locals.var_fs02_dps0_dn5 = assign58530_body19_e91245_d_n5;
            locals.var_fs02_dps0_dn6 = assign58530_body19_e91245_d_n6;
            locals.var_fs02_dps0_dn7 = assign58530_body19_e91245_d_n7;
            locals.var_fs02_dps0_dn8 = assign58530_body19_e91245_d_n8;
            locals.var_fs02_dps0_dn9 = assign58530_body19_e91245_d_n9;
            locals.var_fs02_dps0_dn10 = assign58530_body19_e91245_d_n10;
            locals.var_fs02_dps0_dn11 = assign58530_body19_e91245_d_n11;
            locals.var_fs02_dps0_dn14 = assign58530_body19_e91245_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign58530_body20_e91258, assign58530_body20_e91258_d_n0, assign58530_body20_e91258_d_n2, assign58530_body20_e91258_d_n4, assign58530_body20_e91258_d_n5, assign58530_body20_e91258_d_n6, assign58530_body20_e91258_d_n7, assign58530_body20_e91258_d_n8, assign58530_body20_e91258_d_n9, assign58530_body20_e91258_d_n10, assign58530_body20_e91258_d_n11, assign58530_body20_e91258_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign58530_body20_e91252: f64 = (locals.var_vgp - locals.var_ps0);
        let assign58530_body20_e91255: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign58530_body20_e91256: f64 = (assign58530_body20_e91252 - assign58530_body20_e91255);
        (assign58530_body20_e91256, ((locals.var_vgp_dn0 - locals.var_ps0_dn0) - ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))), ((locals.var_vgp_dn2 - locals.var_ps0_dn2) - ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))), ((locals.var_vgp_dn4 - locals.var_ps0_dn4) - ((locals.var_fac1_dn4 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn4))), ((locals.var_vgp_dn5 - locals.var_ps0_dn5) - ((locals.var_fac1_dn5 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn5))), ((locals.var_vgp_dn6 - locals.var_ps0_dn6) - ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))), ((locals.var_vgp_dn7 - locals.var_ps0_dn7) - ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))), ((locals.var_vgp_dn8 - locals.var_ps0_dn8) - ((locals.var_fac1_dn8 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn8))), ((locals.var_vgp_dn9 - locals.var_ps0_dn9) - ((locals.var_fac1_dn9 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn9))), ((locals.var_vgp_dn10 - locals.var_ps0_dn10) - ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))), ((locals.var_vgp_dn11 - locals.var_ps0_dn11) - ((locals.var_fac1_dn11 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn11))), ((locals.var_vgp_dn14 - locals.var_ps0_dn14) - ((locals.var_fac1_dn14 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn14))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn11, locals.var_fs0_dn14,)
    }
};
            locals.var_fs0 = assign58530_body20_e91258;
            locals.var_fs0_dn0 = assign58530_body20_e91258_d_n0;
            locals.var_fs0_dn2 = assign58530_body20_e91258_d_n2;
            locals.var_fs0_dn4 = assign58530_body20_e91258_d_n4;
            locals.var_fs0_dn5 = assign58530_body20_e91258_d_n5;
            locals.var_fs0_dn6 = assign58530_body20_e91258_d_n6;
            locals.var_fs0_dn7 = assign58530_body20_e91258_d_n7;
            locals.var_fs0_dn8 = assign58530_body20_e91258_d_n8;
            locals.var_fs0_dn9 = assign58530_body20_e91258_d_n9;
            locals.var_fs0_dn10 = assign58530_body20_e91258_d_n10;
            locals.var_fs0_dn11 = assign58530_body20_e91258_d_n11;
            locals.var_fs0_dn14 = assign58530_body20_e91258_d_n14;
            locals.var_fs0_rv = 0.0;
            let (assign58530_body21_e91270, assign58530_body21_e91270_d_n0, assign58530_body21_e91270_d_n2, assign58530_body21_e91270_d_n4, assign58530_body21_e91270_d_n5, assign58530_body21_e91270_d_n6, assign58530_body21_e91270_d_n7, assign58530_body21_e91270_d_n8, assign58530_body21_e91270_d_n9, assign58530_body21_e91270_d_n10, assign58530_body21_e91270_d_n11, assign58530_body21_e91270_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign58530_body21_e91264: f64 = (-1.0);
        let assign58530_body21_e91267: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign58530_body21_e91268: f64 = (assign58530_body21_e91264 - assign58530_body21_e91267);
        (assign58530_body21_e91268, (-((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0))), (-((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2))), (-((locals.var_fac1_dn4 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn4))), (-((locals.var_fac1_dn5 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn5))), (-((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6))), (-((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7))), (-((locals.var_fac1_dn8 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn8))), (-((locals.var_fac1_dn9 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn9))), (-((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10))), (-((locals.var_fac1_dn11 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn11))), (-((locals.var_fac1_dn14 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn14))),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn11, locals.var_fs0_dps0_dn14,)
    }
};
            locals.var_fs0_dps0 = assign58530_body21_e91270;
            locals.var_fs0_dps0_dn0 = assign58530_body21_e91270_d_n0;
            locals.var_fs0_dps0_dn2 = assign58530_body21_e91270_d_n2;
            locals.var_fs0_dps0_dn4 = assign58530_body21_e91270_d_n4;
            locals.var_fs0_dps0_dn5 = assign58530_body21_e91270_d_n5;
            locals.var_fs0_dps0_dn6 = assign58530_body21_e91270_d_n6;
            locals.var_fs0_dps0_dn7 = assign58530_body21_e91270_d_n7;
            locals.var_fs0_dps0_dn8 = assign58530_body21_e91270_d_n8;
            locals.var_fs0_dps0_dn9 = assign58530_body21_e91270_d_n9;
            locals.var_fs0_dps0_dn10 = assign58530_body21_e91270_d_n10;
            locals.var_fs0_dps0_dn11 = assign58530_body21_e91270_d_n11;
            locals.var_fs0_dps0_dn14 = assign58530_body21_e91270_d_n14;
            locals.var_fs0_dps0_rv = 0.0;
            let assign58530_body22_e91273: f64 = if locals.var_flg_conv == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard1439 = assign58530_body22_e91273;
            locals.var_guard1439_rv = 0.0;
            let (assign58530_body23_e91282,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1439 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_brk1,)
    }
};
            locals.var_flg_brk1 = assign58530_body23_e91282;
            locals.var_flg_brk1_rv = 0.0;
            let assign58530_body24_e91285: f64 = if locals.var_flg_brk1 == 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1440 = assign58530_body24_e91285;
            locals.var_guard1440_rv = 0.0;
            let (assign58530_body25_e91297, assign58530_body25_e91297_d_n0, assign58530_body25_e91297_d_n2, assign58530_body25_e91297_d_n4, assign58530_body25_e91297_d_n5, assign58530_body25_e91297_d_n6, assign58530_body25_e91297_d_n7, assign58530_body25_e91297_d_n8, assign58530_body25_e91297_d_n9, assign58530_body25_e91297_d_n10, assign58530_body25_e91297_d_n11, assign58530_body25_e91297_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1440 != 0.0)) {
        let assign58530_body25_e91293: f64 = (-locals.var_fs0);
        let assign58530_body25_e91295: f64 = (assign58530_body25_e91293 / locals.var_fs0_dps0);
        (assign58530_body25_e91295, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign58530_body25_e91293 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign58530_body25_e91293 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn4) * locals.var_fs0_dps0) - (assign58530_body25_e91293 * locals.var_fs0_dps0_dn4)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn5) * locals.var_fs0_dps0) - (assign58530_body25_e91293 * locals.var_fs0_dps0_dn5)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign58530_body25_e91293 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign58530_body25_e91293 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn8) * locals.var_fs0_dps0) - (assign58530_body25_e91293 * locals.var_fs0_dps0_dn8)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn9) * locals.var_fs0_dps0) - (assign58530_body25_e91293 * locals.var_fs0_dps0_dn9)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign58530_body25_e91293 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn11) * locals.var_fs0_dps0) - (assign58530_body25_e91293 * locals.var_fs0_dps0_dn11)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn14) * locals.var_fs0_dps0) - (assign58530_body25_e91293 * locals.var_fs0_dps0_dn14)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn14,)
    }
};
            locals.var_dps0 = assign58530_body25_e91297;
            locals.var_dps0_dn0 = assign58530_body25_e91297_d_n0;
            locals.var_dps0_dn2 = assign58530_body25_e91297_d_n2;
            locals.var_dps0_dn4 = assign58530_body25_e91297_d_n4;
            locals.var_dps0_dn5 = assign58530_body25_e91297_d_n5;
            locals.var_dps0_dn6 = assign58530_body25_e91297_d_n6;
            locals.var_dps0_dn7 = assign58530_body25_e91297_d_n7;
            locals.var_dps0_dn8 = assign58530_body25_e91297_d_n8;
            locals.var_dps0_dn9 = assign58530_body25_e91297_d_n9;
            locals.var_dps0_dn10 = assign58530_body25_e91297_d_n10;
            locals.var_dps0_dn11 = assign58530_body25_e91297_d_n11;
            locals.var_dps0_dn14 = assign58530_body25_e91297_d_n14;
            locals.var_dps0_rv = 0.0;
            let (assign58530_body26_e91319, assign58530_body26_e91319_d_n0, assign58530_body26_e91319_d_n2, assign58530_body26_e91319_d_n4, assign58530_body26_e91319_d_n5, assign58530_body26_e91319_d_n6, assign58530_body26_e91319_d_n7, assign58530_body26_e91319_d_n8, assign58530_body26_e91319_d_n9, assign58530_body26_e91319_d_n10, assign58530_body26_e91319_d_n11, assign58530_body26_e91319_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1440 != 0.0)) {
        let assign58530_body26_e91306: f64 = (0.5 * 0.1);
        let assign58530_body26_e91310: f64 = (locals.var_ps0).abs();
        let (assign58530_body26_e91315, assign58530_body26_e91315_d_n0, assign58530_body26_e91315_d_n2, assign58530_body26_e91315_d_n4, assign58530_body26_e91315_d_n5, assign58530_body26_e91315_d_n6, assign58530_body26_e91315_d_n7, assign58530_body26_e91315_d_n8, assign58530_body26_e91315_d_n9, assign58530_body26_e91315_d_n10, assign58530_body26_e91315_d_n11, assign58530_body26_e91315_d_n14,) = {
            if (1.0 >= assign58530_body26_e91310) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign58530_body26_e91314: f64 = (locals.var_ps0).abs();
                (assign58530_body26_e91314, if locals.var_ps0 >= 0.0 { locals.var_ps0_dn0 } else { (-locals.var_ps0_dn0) }, if locals.var_ps0 >= 0.0 { locals.var_ps0_dn2 } else { (-locals.var_ps0_dn2) }, if locals.var_ps0 >= 0.0 { locals.var_ps0_dn4 } else { (-locals.var_ps0_dn4) }, if locals.var_ps0 >= 0.0 { locals.var_ps0_dn5 } else { (-locals.var_ps0_dn5) }, if locals.var_ps0 >= 0.0 { locals.var_ps0_dn6 } else { (-locals.var_ps0_dn6) }, if locals.var_ps0 >= 0.0 { locals.var_ps0_dn7 } else { (-locals.var_ps0_dn7) }, if locals.var_ps0 >= 0.0 { locals.var_ps0_dn8 } else { (-locals.var_ps0_dn8) }, if locals.var_ps0 >= 0.0 { locals.var_ps0_dn9 } else { (-locals.var_ps0_dn9) }, if locals.var_ps0 >= 0.0 { locals.var_ps0_dn10 } else { (-locals.var_ps0_dn10) }, if locals.var_ps0 >= 0.0 { locals.var_ps0_dn11 } else { (-locals.var_ps0_dn11) }, if locals.var_ps0 >= 0.0 { locals.var_ps0_dn14 } else { (-locals.var_ps0_dn14) },)
            }
        };
        let assign58530_body26_e91316: f64 = (1.0 + assign58530_body26_e91315);
        let assign58530_body26_e91317: f64 = (assign58530_body26_e91306 * assign58530_body26_e91316);
        (assign58530_body26_e91317, (assign58530_body26_e91306 * assign58530_body26_e91315_d_n0), (assign58530_body26_e91306 * assign58530_body26_e91315_d_n2), (assign58530_body26_e91306 * assign58530_body26_e91315_d_n4), (assign58530_body26_e91306 * assign58530_body26_e91315_d_n5), (assign58530_body26_e91306 * assign58530_body26_e91315_d_n6), (assign58530_body26_e91306 * assign58530_body26_e91315_d_n7), (assign58530_body26_e91306 * assign58530_body26_e91315_d_n8), (assign58530_body26_e91306 * assign58530_body26_e91315_d_n9), (assign58530_body26_e91306 * assign58530_body26_e91315_d_n10), (assign58530_body26_e91306 * assign58530_body26_e91315_d_n11), (assign58530_body26_e91306 * assign58530_body26_e91315_d_n14),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn4, locals.var_dplim_dn5, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn8, locals.var_dplim_dn9, locals.var_dplim_dn10, locals.var_dplim_dn11, locals.var_dplim_dn14,)
    }
};
            locals.var_dplim = assign58530_body26_e91319;
            locals.var_dplim_dn0 = assign58530_body26_e91319_d_n0;
            locals.var_dplim_dn2 = assign58530_body26_e91319_d_n2;
            locals.var_dplim_dn4 = assign58530_body26_e91319_d_n4;
            locals.var_dplim_dn5 = assign58530_body26_e91319_d_n5;
            locals.var_dplim_dn6 = assign58530_body26_e91319_d_n6;
            locals.var_dplim_dn7 = assign58530_body26_e91319_d_n7;
            locals.var_dplim_dn8 = assign58530_body26_e91319_d_n8;
            locals.var_dplim_dn9 = assign58530_body26_e91319_d_n9;
            locals.var_dplim_dn10 = assign58530_body26_e91319_d_n10;
            locals.var_dplim_dn11 = assign58530_body26_e91319_d_n11;
            locals.var_dplim_dn14 = assign58530_body26_e91319_d_n14;
            locals.var_dplim_rv = 0.0;
            let assign58530_body27_e91321: f64 = (locals.var_dps0).abs();
            let assign58530_body27_e91323: f64 = if assign58530_body27_e91321 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard1441 = assign58530_body27_e91323;
            locals.var_guard1441_rv = 0.0;
            let (assign58530_body28_e91342, assign58530_body28_e91342_d_n0, assign58530_body28_e91342_d_n2, assign58530_body28_e91342_d_n4, assign58530_body28_e91342_d_n5, assign58530_body28_e91342_d_n6, assign58530_body28_e91342_d_n7, assign58530_body28_e91342_d_n8, assign58530_body28_e91342_d_n9, assign58530_body28_e91342_d_n10, assign58530_body28_e91342_d_n11, assign58530_body28_e91342_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1440 != 0.0)) && (locals.var_guard1441 != 0.0)) {
        let (assign58530_body28_e91339,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign58530_body28_e91338: f64 = (-1.0);
                (assign58530_body28_e91338,)
            }
        };
        let assign58530_body28_e91340: f64 = (locals.var_dplim * assign58530_body28_e91339);
        (assign58530_body28_e91340, (locals.var_dplim_dn0 * assign58530_body28_e91339), (locals.var_dplim_dn2 * assign58530_body28_e91339), (locals.var_dplim_dn4 * assign58530_body28_e91339), (locals.var_dplim_dn5 * assign58530_body28_e91339), (locals.var_dplim_dn6 * assign58530_body28_e91339), (locals.var_dplim_dn7 * assign58530_body28_e91339), (locals.var_dplim_dn8 * assign58530_body28_e91339), (locals.var_dplim_dn9 * assign58530_body28_e91339), (locals.var_dplim_dn10 * assign58530_body28_e91339), (locals.var_dplim_dn11 * assign58530_body28_e91339), (locals.var_dplim_dn14 * assign58530_body28_e91339),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn14,)
    }
};
            locals.var_dps0 = assign58530_body28_e91342;
            locals.var_dps0_dn0 = assign58530_body28_e91342_d_n0;
            locals.var_dps0_dn2 = assign58530_body28_e91342_d_n2;
            locals.var_dps0_dn4 = assign58530_body28_e91342_d_n4;
            locals.var_dps0_dn5 = assign58530_body28_e91342_d_n5;
            locals.var_dps0_dn6 = assign58530_body28_e91342_d_n6;
            locals.var_dps0_dn7 = assign58530_body28_e91342_d_n7;
            locals.var_dps0_dn8 = assign58530_body28_e91342_d_n8;
            locals.var_dps0_dn9 = assign58530_body28_e91342_d_n9;
            locals.var_dps0_dn10 = assign58530_body28_e91342_d_n10;
            locals.var_dps0_dn11 = assign58530_body28_e91342_d_n11;
            locals.var_dps0_dn14 = assign58530_body28_e91342_d_n14;
            locals.var_dps0_rv = 0.0;
            let (assign58530_body29_e91353, assign58530_body29_e91353_d_n0, assign58530_body29_e91353_d_n2, assign58530_body29_e91353_d_n4, assign58530_body29_e91353_d_n5, assign58530_body29_e91353_d_n6, assign58530_body29_e91353_d_n7, assign58530_body29_e91353_d_n8, assign58530_body29_e91353_d_n9, assign58530_body29_e91353_d_n10, assign58530_body29_e91353_d_n11, assign58530_body29_e91353_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1440 != 0.0)) {
        let assign58530_body29_e91351: f64 = (locals.var_ps0 + locals.var_dps0);
        (assign58530_body29_e91351, (locals.var_ps0_dn0 + locals.var_dps0_dn0), (locals.var_ps0_dn2 + locals.var_dps0_dn2), (locals.var_ps0_dn4 + locals.var_dps0_dn4), (locals.var_ps0_dn5 + locals.var_dps0_dn5), (locals.var_ps0_dn6 + locals.var_dps0_dn6), (locals.var_ps0_dn7 + locals.var_dps0_dn7), (locals.var_ps0_dn8 + locals.var_dps0_dn8), (locals.var_ps0_dn9 + locals.var_dps0_dn9), (locals.var_ps0_dn10 + locals.var_dps0_dn10), (locals.var_ps0_dn11 + locals.var_dps0_dn11), (locals.var_ps0_dn14 + locals.var_dps0_dn14),)
    } else {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn4, locals.var_ps0_dn5, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn8, locals.var_ps0_dn9, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn14,)
    }
};
            locals.var_ps0 = assign58530_body29_e91353;
            locals.var_ps0_dn0 = assign58530_body29_e91353_d_n0;
            locals.var_ps0_dn2 = assign58530_body29_e91353_d_n2;
            locals.var_ps0_dn4 = assign58530_body29_e91353_d_n4;
            locals.var_ps0_dn5 = assign58530_body29_e91353_d_n5;
            locals.var_ps0_dn6 = assign58530_body29_e91353_d_n6;
            locals.var_ps0_dn7 = assign58530_body29_e91353_d_n7;
            locals.var_ps0_dn8 = assign58530_body29_e91353_d_n8;
            locals.var_ps0_dn9 = assign58530_body29_e91353_d_n9;
            locals.var_ps0_dn10 = assign58530_body29_e91353_d_n10;
            locals.var_ps0_dn11 = assign58530_body29_e91353_d_n11;
            locals.var_ps0_dn14 = assign58530_body29_e91353_d_n14;
            locals.var_ps0_rv = 0.0;
            let assign58530_body30_e91355: f64 = (locals.var_dps0).abs();
            let assign58530_body30_e91359: f64 = (locals.var_fs0).abs();
            let assign58530_body30_e91362: f64 = if ((assign58530_body30_e91355 <= 1e-12) && (assign58530_body30_e91359 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard1442 = assign58530_body30_e91362;
            locals.var_guard1442_rv = 0.0;
            let (assign58530_body31_e91373,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1440 != 0.0)) && (locals.var_guard1442 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign58530_body31_e91373;
            locals.var_flg_conv_rv = 0.0;
            let (assign58530_body32_e91384,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_flg_brk1 != 0.0)) {
        let assign58530_body32_e91382: f64 = (locals.var_lp_s0_max + 1.0);
        (assign58530_body32_e91382,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign58530_body32_e91384;
            locals.var_lp_s0_rv = 0.0;
            let (assign58530_body33_e91391,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_brk1,)
    }
};
            locals.var_flg_brk1 = assign58530_body33_e91391;
            locals.var_flg_brk1_rv = 0.0;
            let (assign58530_body34_e91400,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign58530_body34_e91398: f64 = (locals.var_lp_s0 + 1.0);
        (assign58530_body34_e91398,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign58530_body34_e91400;
            locals.var_lp_s0_rv = 0.0;
        }

    }
}
