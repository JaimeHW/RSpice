#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_56(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign21410_e16400, assign21410_e16400_d_n0, assign21410_e16400_d_n2, assign21410_e16400_d_n4, assign21410_e16400_d_n5, assign21410_e16400_d_n6, assign21410_e16400_d_n7, assign21410_e16400_d_n8, assign21410_e16400_d_n9, assign21410_e16400_d_n10, assign21410_e16400_d_n11, assign21410_e16400_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) {
        let assign21410_e16397: f64 = (locals.var_t0 + locals.var_tmf2);
        let assign21410_e16398: f64 = (0.5 * assign21410_e16397);
        (assign21410_e16398, (0.5 * (locals.var_t0_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t0_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t0_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t0_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t0_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t0_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t0_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t0_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t0_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t0_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t0_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign21410_e16400;
        locals.var_t1_dn0 = assign21410_e16400_d_n0;
        locals.var_t1_dn2 = assign21410_e16400_d_n2;
        locals.var_t1_dn4 = assign21410_e16400_d_n4;
        locals.var_t1_dn5 = assign21410_e16400_d_n5;
        locals.var_t1_dn6 = assign21410_e16400_d_n6;
        locals.var_t1_dn7 = assign21410_e16400_d_n7;
        locals.var_t1_dn8 = assign21410_e16400_d_n8;
        locals.var_t1_dn9 = assign21410_e16400_d_n9;
        locals.var_t1_dn10 = assign21410_e16400_d_n10;
        locals.var_t1_dn11 = assign21410_e16400_d_n11;
        locals.var_t1_dn14 = assign21410_e16400_d_n14;
        locals.var_t1_rv = 0.0;

        let assign21420_e16403: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard423 = assign21420_e16403;
        locals.var_guard423_rv = 0.0;

        let (assign21430_e16413, assign21430_e16413_d_n0, assign21430_e16413_d_n2, assign21430_e16413_d_n4, assign21430_e16413_d_n5, assign21430_e16413_d_n6, assign21430_e16413_d_n7, assign21430_e16413_d_n8, assign21430_e16413_d_n9, assign21430_e16413_d_n10, assign21430_e16413_d_n11, assign21430_e16413_d_n14,) = {
    if ((((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) && (locals.var_guard423 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign21430_e16413;
        locals.var_t1_dn0 = assign21430_e16413_d_n0;
        locals.var_t1_dn2 = assign21430_e16413_d_n2;
        locals.var_t1_dn4 = assign21430_e16413_d_n4;
        locals.var_t1_dn5 = assign21430_e16413_d_n5;
        locals.var_t1_dn6 = assign21430_e16413_d_n6;
        locals.var_t1_dn7 = assign21430_e16413_d_n7;
        locals.var_t1_dn8 = assign21430_e16413_d_n8;
        locals.var_t1_dn9 = assign21430_e16413_d_n9;
        locals.var_t1_dn10 = assign21430_e16413_d_n10;
        locals.var_t1_dn11 = assign21430_e16413_d_n11;
        locals.var_t1_dn14 = assign21430_e16413_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign21440_e16423, assign21440_e16423_d_n0, assign21440_e16423_d_n2, assign21440_e16423_d_n4, assign21440_e16423_d_n5, assign21440_e16423_d_n6, assign21440_e16423_d_n7, assign21440_e16423_d_n8, assign21440_e16423_d_n9, assign21440_e16423_d_n10, assign21440_e16423_d_n11, assign21440_e16423_d_n14,) = {
    if ((((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) && (locals.var_guard423 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign21440_e16423;
        locals.var_t2_dn0 = assign21440_e16423_d_n0;
        locals.var_t2_dn2 = assign21440_e16423_d_n2;
        locals.var_t2_dn4 = assign21440_e16423_d_n4;
        locals.var_t2_dn5 = assign21440_e16423_d_n5;
        locals.var_t2_dn6 = assign21440_e16423_d_n6;
        locals.var_t2_dn7 = assign21440_e16423_d_n7;
        locals.var_t2_dn8 = assign21440_e16423_d_n8;
        locals.var_t2_dn9 = assign21440_e16423_d_n9;
        locals.var_t2_dn10 = assign21440_e16423_d_n10;
        locals.var_t2_dn11 = assign21440_e16423_d_n11;
        locals.var_t2_dn14 = assign21440_e16423_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign21450_e16435, assign21450_e16435_d_n0, assign21450_e16435_d_n2, assign21450_e16435_d_n4, assign21450_e16435_d_n5, assign21450_e16435_d_n6, assign21450_e16435_d_n7, assign21450_e16435_d_n8, assign21450_e16435_d_n9, assign21450_e16435_d_n10, assign21450_e16435_d_n11, assign21450_e16435_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) {
        let assign21450_e16432: f64 = (10.0 * 2.220446049250313e-16);
        let assign21450_e16433: f64 = (locals.var_t1 + assign21450_e16432);
        (assign21450_e16433, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign21450_e16435;
        locals.var_t1_dn0 = assign21450_e16435_d_n0;
        locals.var_t1_dn2 = assign21450_e16435_d_n2;
        locals.var_t1_dn4 = assign21450_e16435_d_n4;
        locals.var_t1_dn5 = assign21450_e16435_d_n5;
        locals.var_t1_dn6 = assign21450_e16435_d_n6;
        locals.var_t1_dn7 = assign21450_e16435_d_n7;
        locals.var_t1_dn8 = assign21450_e16435_d_n8;
        locals.var_t1_dn9 = assign21450_e16435_d_n9;
        locals.var_t1_dn10 = assign21450_e16435_d_n10;
        locals.var_t1_dn11 = assign21450_e16435_d_n11;
        locals.var_t1_dn14 = assign21450_e16435_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign21460_e16449, assign21460_e16449_d_n0, assign21460_e16449_d_n2, assign21460_e16449_d_n4, assign21460_e16449_d_n5, assign21460_e16449_d_n6, assign21460_e16449_d_n7, assign21460_e16449_d_n8, assign21460_e16449_d_n9, assign21460_e16449_d_n10, assign21460_e16449_d_n11, assign21460_e16449_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) {
        let assign21460_e16445: f64 = (locals.var_mks_nsubsub + locals.var_uc_nover);
        let assign21460_e16446: f64 = (locals.var_uc_nover * assign21460_e16445);
        let assign21460_e16447: f64 = (locals.var_mks_nsubsub / assign21460_e16446);
        (assign21460_e16447, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21460_e16449;
        locals.var_t0_dn0 = assign21460_e16449_d_n0;
        locals.var_t0_dn2 = assign21460_e16449_d_n2;
        locals.var_t0_dn4 = assign21460_e16449_d_n4;
        locals.var_t0_dn5 = assign21460_e16449_d_n5;
        locals.var_t0_dn6 = assign21460_e16449_d_n6;
        locals.var_t0_dn7 = assign21460_e16449_d_n7;
        locals.var_t0_dn8 = assign21460_e16449_d_n8;
        locals.var_t0_dn9 = assign21460_e16449_d_n9;
        locals.var_t0_dn10 = assign21460_e16449_d_n10;
        locals.var_t0_dn11 = assign21460_e16449_d_n11;
        locals.var_t0_dn14 = assign21460_e16449_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign21470_e16463, assign21470_e16463_d_n0, assign21470_e16463_d_n2, assign21470_e16463_d_n4, assign21470_e16463_d_n5, assign21470_e16463_d_n6, assign21470_e16463_d_n7, assign21470_e16463_d_n8, assign21470_e16463_d_n9, assign21470_e16463_d_n10, assign21470_e16463_d_n11, assign21470_e16463_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) {
        let assign21470_e16457: f64 = (2.0 * 1.034943e-10);
        let assign21470_e16459: f64 = (assign21470_e16457 / 1.6021918e-19);
        let assign21470_e16461: f64 = (assign21470_e16459 * locals.var_t0);
        (assign21470_e16461, (assign21470_e16459 * locals.var_t0_dn0), (assign21470_e16459 * locals.var_t0_dn2), (assign21470_e16459 * locals.var_t0_dn4), (assign21470_e16459 * locals.var_t0_dn5), (assign21470_e16459 * locals.var_t0_dn6), (assign21470_e16459 * locals.var_t0_dn7), (assign21470_e16459 * locals.var_t0_dn8), (assign21470_e16459 * locals.var_t0_dn9), (assign21470_e16459 * locals.var_t0_dn10), (assign21470_e16459 * locals.var_t0_dn11), (assign21470_e16459 * locals.var_t0_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign21470_e16463;
        locals.var_t4_dn0 = assign21470_e16463_d_n0;
        locals.var_t4_dn2 = assign21470_e16463_d_n2;
        locals.var_t4_dn4 = assign21470_e16463_d_n4;
        locals.var_t4_dn5 = assign21470_e16463_d_n5;
        locals.var_t4_dn6 = assign21470_e16463_d_n6;
        locals.var_t4_dn7 = assign21470_e16463_d_n7;
        locals.var_t4_dn8 = assign21470_e16463_d_n8;
        locals.var_t4_dn9 = assign21470_e16463_d_n9;
        locals.var_t4_dn10 = assign21470_e16463_d_n10;
        locals.var_t4_dn11 = assign21470_e16463_d_n11;
        locals.var_t4_dn14 = assign21470_e16463_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign21480_e16476, assign21480_e16476_d_n0, assign21480_e16476_d_n2, assign21480_e16476_d_n4, assign21480_e16476_d_n5, assign21480_e16476_d_n6, assign21480_e16476_d_n7, assign21480_e16476_d_n8, assign21480_e16476_d_n9, assign21480_e16476_d_n10, assign21480_e16476_d_n11, assign21480_e16476_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) {
        let assign21480_e16471: f64 = (locals.var_t4 * locals.var_t1);
        let assign21480_e16472: f64 = (assign21480_e16471).sqrt();
        let assign21480_e16474: f64 = (assign21480_e16472 + 1e-25);
        (assign21480_e16474, (((locals.var_t4_dn0 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn0)) / (2.0 * assign21480_e16472)), (((locals.var_t4_dn2 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn2)) / (2.0 * assign21480_e16472)), (((locals.var_t4_dn4 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn4)) / (2.0 * assign21480_e16472)), (((locals.var_t4_dn5 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn5)) / (2.0 * assign21480_e16472)), (((locals.var_t4_dn6 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn6)) / (2.0 * assign21480_e16472)), (((locals.var_t4_dn7 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn7)) / (2.0 * assign21480_e16472)), (((locals.var_t4_dn8 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn8)) / (2.0 * assign21480_e16472)), (((locals.var_t4_dn9 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn9)) / (2.0 * assign21480_e16472)), (((locals.var_t4_dn10 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn10)) / (2.0 * assign21480_e16472)), (((locals.var_t4_dn11 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn11)) / (2.0 * assign21480_e16472)), (((locals.var_t4_dn14 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn14)) / (2.0 * assign21480_e16472)),)
    } else {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn4, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, locals.var_wdep_dn9, locals.var_wdep_dn10, locals.var_wdep_dn11, locals.var_wdep_dn14,)
    }
};
        locals.var_wdep = assign21480_e16476;
        locals.var_wdep_dn0 = assign21480_e16476_d_n0;
        locals.var_wdep_dn2 = assign21480_e16476_d_n2;
        locals.var_wdep_dn4 = assign21480_e16476_d_n4;
        locals.var_wdep_dn5 = assign21480_e16476_d_n5;
        locals.var_wdep_dn6 = assign21480_e16476_d_n6;
        locals.var_wdep_dn7 = assign21480_e16476_d_n7;
        locals.var_wdep_dn8 = assign21480_e16476_d_n8;
        locals.var_wdep_dn9 = assign21480_e16476_d_n9;
        locals.var_wdep_dn10 = assign21480_e16476_d_n10;
        locals.var_wdep_dn11 = assign21480_e16476_d_n11;
        locals.var_wdep_dn14 = assign21480_e16476_d_n14;
        locals.var_wdep_rv = 0.0;

        let (assign21490_e16490, assign21490_e16490_d_n0, assign21490_e16490_d_n2, assign21490_e16490_d_n4, assign21490_e16490_d_n5, assign21490_e16490_d_n6, assign21490_e16490_d_n7, assign21490_e16490_d_n8, assign21490_e16490_d_n9, assign21490_e16490_d_n10, assign21490_e16490_d_n11, assign21490_e16490_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) {
        let assign21490_e16484: f64 = (p.p334 - locals.var_wdep);
        let assign21490_e16487: f64 = (0.1 * p.p334);
        let assign21490_e16488: f64 = (assign21490_e16484 - assign21490_e16487);
        (assign21490_e16488, (-locals.var_wdep_dn0), (-locals.var_wdep_dn2), (-locals.var_wdep_dn4), (-locals.var_wdep_dn5), (-locals.var_wdep_dn6), (-locals.var_wdep_dn7), (-locals.var_wdep_dn8), (-locals.var_wdep_dn9), (-locals.var_wdep_dn10), (-locals.var_wdep_dn11), (-locals.var_wdep_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign21490_e16490;
        locals.var_tmf1_dn0 = assign21490_e16490_d_n0;
        locals.var_tmf1_dn2 = assign21490_e16490_d_n2;
        locals.var_tmf1_dn4 = assign21490_e16490_d_n4;
        locals.var_tmf1_dn5 = assign21490_e16490_d_n5;
        locals.var_tmf1_dn6 = assign21490_e16490_d_n6;
        locals.var_tmf1_dn7 = assign21490_e16490_d_n7;
        locals.var_tmf1_dn8 = assign21490_e16490_d_n8;
        locals.var_tmf1_dn9 = assign21490_e16490_d_n9;
        locals.var_tmf1_dn10 = assign21490_e16490_d_n10;
        locals.var_tmf1_dn11 = assign21490_e16490_d_n11;
        locals.var_tmf1_dn14 = assign21490_e16490_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign21500_e16504, assign21500_e16504_d_n0, assign21500_e16504_d_n2, assign21500_e16504_d_n4, assign21500_e16504_d_n5, assign21500_e16504_d_n6, assign21500_e16504_d_n7, assign21500_e16504_d_n8, assign21500_e16504_d_n9, assign21500_e16504_d_n10, assign21500_e16504_d_n11, assign21500_e16504_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) {
        let assign21500_e16498: f64 = (4.0 * p.p334);
        let assign21500_e16501: f64 = (0.1 * p.p334);
        let assign21500_e16502: f64 = (assign21500_e16498 * assign21500_e16501);
        (assign21500_e16502, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign21500_e16504;
        locals.var_tmf2_dn0 = assign21500_e16504_d_n0;
        locals.var_tmf2_dn2 = assign21500_e16504_d_n2;
        locals.var_tmf2_dn4 = assign21500_e16504_d_n4;
        locals.var_tmf2_dn5 = assign21500_e16504_d_n5;
        locals.var_tmf2_dn6 = assign21500_e16504_d_n6;
        locals.var_tmf2_dn7 = assign21500_e16504_d_n7;
        locals.var_tmf2_dn8 = assign21500_e16504_d_n8;
        locals.var_tmf2_dn9 = assign21500_e16504_d_n9;
        locals.var_tmf2_dn10 = assign21500_e16504_d_n10;
        locals.var_tmf2_dn11 = assign21500_e16504_d_n11;
        locals.var_tmf2_dn14 = assign21500_e16504_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign21510_e16518, assign21510_e16518_d_n0, assign21510_e16518_d_n2, assign21510_e16518_d_n4, assign21510_e16518_d_n5, assign21510_e16518_d_n6, assign21510_e16518_d_n7, assign21510_e16518_d_n8, assign21510_e16518_d_n9, assign21510_e16518_d_n10, assign21510_e16518_d_n11, assign21510_e16518_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) {
        let (assign21510_e16516, assign21510_e16516_d_n0, assign21510_e16516_d_n2, assign21510_e16516_d_n4, assign21510_e16516_d_n5, assign21510_e16516_d_n6, assign21510_e16516_d_n7, assign21510_e16516_d_n8, assign21510_e16516_d_n9, assign21510_e16516_d_n10, assign21510_e16516_d_n11, assign21510_e16516_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign21510_e16515: f64 = (-locals.var_tmf2);
                (assign21510_e16515, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign21510_e16516, assign21510_e16516_d_n0, assign21510_e16516_d_n2, assign21510_e16516_d_n4, assign21510_e16516_d_n5, assign21510_e16516_d_n6, assign21510_e16516_d_n7, assign21510_e16516_d_n8, assign21510_e16516_d_n9, assign21510_e16516_d_n10, assign21510_e16516_d_n11, assign21510_e16516_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign21510_e16518;
        locals.var_tmf2_dn0 = assign21510_e16518_d_n0;
        locals.var_tmf2_dn2 = assign21510_e16518_d_n2;
        locals.var_tmf2_dn4 = assign21510_e16518_d_n4;
        locals.var_tmf2_dn5 = assign21510_e16518_d_n5;
        locals.var_tmf2_dn6 = assign21510_e16518_d_n6;
        locals.var_tmf2_dn7 = assign21510_e16518_d_n7;
        locals.var_tmf2_dn8 = assign21510_e16518_d_n8;
        locals.var_tmf2_dn9 = assign21510_e16518_d_n9;
        locals.var_tmf2_dn10 = assign21510_e16518_d_n10;
        locals.var_tmf2_dn11 = assign21510_e16518_d_n11;
        locals.var_tmf2_dn14 = assign21510_e16518_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign21520_e16531, assign21520_e16531_d_n0, assign21520_e16531_d_n2, assign21520_e16531_d_n4, assign21520_e16531_d_n5, assign21520_e16531_d_n6, assign21520_e16531_d_n7, assign21520_e16531_d_n8, assign21520_e16531_d_n9, assign21520_e16531_d_n10, assign21520_e16531_d_n11, assign21520_e16531_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) {
        let assign21520_e16526: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign21520_e16528: f64 = (assign21520_e16526 + locals.var_tmf2);
        let assign21520_e16529: f64 = (assign21520_e16528).sqrt();
        (assign21520_e16529, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign21520_e16529)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign21520_e16529)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign21520_e16529)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign21520_e16529)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign21520_e16529)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign21520_e16529)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign21520_e16529)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign21520_e16529)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign21520_e16529)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign21520_e16529)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign21520_e16529)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign21520_e16531;
        locals.var_tmf2_dn0 = assign21520_e16531_d_n0;
        locals.var_tmf2_dn2 = assign21520_e16531_d_n2;
        locals.var_tmf2_dn4 = assign21520_e16531_d_n4;
        locals.var_tmf2_dn5 = assign21520_e16531_d_n5;
        locals.var_tmf2_dn6 = assign21520_e16531_d_n6;
        locals.var_tmf2_dn7 = assign21520_e16531_d_n7;
        locals.var_tmf2_dn8 = assign21520_e16531_d_n8;
        locals.var_tmf2_dn9 = assign21520_e16531_d_n9;
        locals.var_tmf2_dn10 = assign21520_e16531_d_n10;
        locals.var_tmf2_dn11 = assign21520_e16531_d_n11;
        locals.var_tmf2_dn14 = assign21520_e16531_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign21530_e16545, assign21530_e16545_d_n0, assign21530_e16545_d_n2, assign21530_e16545_d_n4, assign21530_e16545_d_n5, assign21530_e16545_d_n6, assign21530_e16545_d_n7, assign21530_e16545_d_n8, assign21530_e16545_d_n9, assign21530_e16545_d_n10, assign21530_e16545_d_n11, assign21530_e16545_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) {
        let assign21530_e16541: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign21530_e16542: f64 = (1.0 + assign21530_e16541);
        let assign21530_e16543: f64 = (0.5 * assign21530_e16542);
        (assign21530_e16543, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21530_e16545;
        locals.var_t0_dn0 = assign21530_e16545_d_n0;
        locals.var_t0_dn2 = assign21530_e16545_d_n2;
        locals.var_t0_dn4 = assign21530_e16545_d_n4;
        locals.var_t0_dn5 = assign21530_e16545_d_n5;
        locals.var_t0_dn6 = assign21530_e16545_d_n6;
        locals.var_t0_dn7 = assign21530_e16545_d_n7;
        locals.var_t0_dn8 = assign21530_e16545_d_n8;
        locals.var_t0_dn9 = assign21530_e16545_d_n9;
        locals.var_t0_dn10 = assign21530_e16545_d_n10;
        locals.var_t0_dn11 = assign21530_e16545_d_n11;
        locals.var_t0_dn14 = assign21530_e16545_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign21540_e16559, assign21540_e16559_d_n0, assign21540_e16559_d_n2, assign21540_e16559_d_n4, assign21540_e16559_d_n5, assign21540_e16559_d_n6, assign21540_e16559_d_n7, assign21540_e16559_d_n8, assign21540_e16559_d_n9, assign21540_e16559_d_n10, assign21540_e16559_d_n11, assign21540_e16559_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) {
        let assign21540_e16555: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign21540_e16556: f64 = (0.5 * assign21540_e16555);
        let assign21540_e16557: f64 = (p.p334 - assign21540_e16556);
        (assign21540_e16557, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn4, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, locals.var_wdep_dn9, locals.var_wdep_dn10, locals.var_wdep_dn11, locals.var_wdep_dn14,)
    }
};
        locals.var_wdep = assign21540_e16559;
        locals.var_wdep_dn0 = assign21540_e16559_d_n0;
        locals.var_wdep_dn2 = assign21540_e16559_d_n2;
        locals.var_wdep_dn4 = assign21540_e16559_d_n4;
        locals.var_wdep_dn5 = assign21540_e16559_d_n5;
        locals.var_wdep_dn6 = assign21540_e16559_d_n6;
        locals.var_wdep_dn7 = assign21540_e16559_d_n7;
        locals.var_wdep_dn8 = assign21540_e16559_d_n8;
        locals.var_wdep_dn9 = assign21540_e16559_d_n9;
        locals.var_wdep_dn10 = assign21540_e16559_d_n10;
        locals.var_wdep_dn11 = assign21540_e16559_d_n11;
        locals.var_wdep_dn14 = assign21540_e16559_d_n14;
        locals.var_wdep_rv = 0.0;

        let (assign21550_e16571, assign21550_e16571_d_n0, assign21550_e16571_d_n2, assign21550_e16571_d_n4, assign21550_e16571_d_n5, assign21550_e16571_d_n6, assign21550_e16571_d_n7, assign21550_e16571_d_n8, assign21550_e16571_d_n9, assign21550_e16571_d_n10, assign21550_e16571_d_n11, assign21550_e16571_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) {
        let assign21550_e16568: f64 = (p.p334 - locals.var_wdep);
        let assign21550_e16569: f64 = (locals.var_ldrift0 / assign21550_e16568);
        (assign21550_e16569, (-((locals.var_ldrift0 * (-locals.var_wdep_dn0)) / (assign21550_e16568 * assign21550_e16568))), (-((locals.var_ldrift0 * (-locals.var_wdep_dn2)) / (assign21550_e16568 * assign21550_e16568))), (-((locals.var_ldrift0 * (-locals.var_wdep_dn4)) / (assign21550_e16568 * assign21550_e16568))), (-((locals.var_ldrift0 * (-locals.var_wdep_dn5)) / (assign21550_e16568 * assign21550_e16568))), (-((locals.var_ldrift0 * (-locals.var_wdep_dn6)) / (assign21550_e16568 * assign21550_e16568))), (-((locals.var_ldrift0 * (-locals.var_wdep_dn7)) / (assign21550_e16568 * assign21550_e16568))), (-((locals.var_ldrift0 * (-locals.var_wdep_dn8)) / (assign21550_e16568 * assign21550_e16568))), (-((locals.var_ldrift0 * (-locals.var_wdep_dn9)) / (assign21550_e16568 * assign21550_e16568))), (-((locals.var_ldrift0 * (-locals.var_wdep_dn10)) / (assign21550_e16568 * assign21550_e16568))), (-((locals.var_ldrift0 * (-locals.var_wdep_dn11)) / (assign21550_e16568 * assign21550_e16568))), (-((locals.var_ldrift0 * (-locals.var_wdep_dn14)) / (assign21550_e16568 * assign21550_e16568))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign21550_e16571;
        locals.var_t6_dn0 = assign21550_e16571_d_n0;
        locals.var_t6_dn2 = assign21550_e16571_d_n2;
        locals.var_t6_dn4 = assign21550_e16571_d_n4;
        locals.var_t6_dn5 = assign21550_e16571_d_n5;
        locals.var_t6_dn6 = assign21550_e16571_d_n6;
        locals.var_t6_dn7 = assign21550_e16571_d_n7;
        locals.var_t6_dn8 = assign21550_e16571_d_n8;
        locals.var_t6_dn9 = assign21550_e16571_d_n9;
        locals.var_t6_dn10 = assign21550_e16571_d_n10;
        locals.var_t6_dn11 = assign21550_e16571_d_n11;
        locals.var_t6_dn14 = assign21550_e16571_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign21560_e16581, assign21560_e16581_d_n0, assign21560_e16581_d_n2, assign21560_e16581_d_n4, assign21560_e16581_d_n5, assign21560_e16581_d_n6, assign21560_e16581_d_n7, assign21560_e16581_d_n8, assign21560_e16581_d_n9, assign21560_e16581_d_n10, assign21560_e16581_d_n11, assign21560_e16581_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) {
        let assign21560_e16579: f64 = (locals.var_rdrift * locals.var_t6);
        (assign21560_e16579, ((locals.var_rdrift_dn0 * locals.var_t6) + (locals.var_rdrift * locals.var_t6_dn0)), ((locals.var_rdrift_dn2 * locals.var_t6) + (locals.var_rdrift * locals.var_t6_dn2)), ((locals.var_rdrift_dn4 * locals.var_t6) + (locals.var_rdrift * locals.var_t6_dn4)), ((locals.var_rdrift_dn5 * locals.var_t6) + (locals.var_rdrift * locals.var_t6_dn5)), ((locals.var_rdrift_dn6 * locals.var_t6) + (locals.var_rdrift * locals.var_t6_dn6)), ((locals.var_rdrift_dn7 * locals.var_t6) + (locals.var_rdrift * locals.var_t6_dn7)), ((locals.var_rdrift_dn8 * locals.var_t6) + (locals.var_rdrift * locals.var_t6_dn8)), ((locals.var_rdrift_dn9 * locals.var_t6) + (locals.var_rdrift * locals.var_t6_dn9)), ((locals.var_rdrift_dn10 * locals.var_t6) + (locals.var_rdrift * locals.var_t6_dn10)), ((locals.var_rdrift_dn11 * locals.var_t6) + (locals.var_rdrift * locals.var_t6_dn11)), ((locals.var_rdrift_dn14 * locals.var_t6) + (locals.var_rdrift * locals.var_t6_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21560_e16581;
        locals.var_t0_dn0 = assign21560_e16581_d_n0;
        locals.var_t0_dn2 = assign21560_e16581_d_n2;
        locals.var_t0_dn4 = assign21560_e16581_d_n4;
        locals.var_t0_dn5 = assign21560_e16581_d_n5;
        locals.var_t0_dn6 = assign21560_e16581_d_n6;
        locals.var_t0_dn7 = assign21560_e16581_d_n7;
        locals.var_t0_dn8 = assign21560_e16581_d_n8;
        locals.var_t0_dn9 = assign21560_e16581_d_n9;
        locals.var_t0_dn10 = assign21560_e16581_d_n10;
        locals.var_t0_dn11 = assign21560_e16581_d_n11;
        locals.var_t0_dn14 = assign21560_e16581_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign21570_e16591, assign21570_e16591_d_n0, assign21570_e16591_d_n2, assign21570_e16591_d_n4, assign21570_e16591_d_n5, assign21570_e16591_d_n6, assign21570_e16591_d_n7, assign21570_e16591_d_n8, assign21570_e16591_d_n9, assign21570_e16591_d_n10, assign21570_e16591_d_n11, assign21570_e16591_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) {
        let assign21570_e16589: f64 = (locals.var_rsdrift * locals.var_t6);
        (assign21570_e16589, ((locals.var_rsdrift_dn0 * locals.var_t6) + (locals.var_rsdrift * locals.var_t6_dn0)), ((locals.var_rsdrift_dn2 * locals.var_t6) + (locals.var_rsdrift * locals.var_t6_dn2)), ((locals.var_rsdrift_dn4 * locals.var_t6) + (locals.var_rsdrift * locals.var_t6_dn4)), ((locals.var_rsdrift_dn5 * locals.var_t6) + (locals.var_rsdrift * locals.var_t6_dn5)), ((locals.var_rsdrift_dn6 * locals.var_t6) + (locals.var_rsdrift * locals.var_t6_dn6)), ((locals.var_rsdrift_dn7 * locals.var_t6) + (locals.var_rsdrift * locals.var_t6_dn7)), ((locals.var_rsdrift_dn8 * locals.var_t6) + (locals.var_rsdrift * locals.var_t6_dn8)), ((locals.var_rsdrift_dn9 * locals.var_t6) + (locals.var_rsdrift * locals.var_t6_dn9)), ((locals.var_rsdrift_dn10 * locals.var_t6) + (locals.var_rsdrift * locals.var_t6_dn10)), ((locals.var_rsdrift_dn11 * locals.var_t6) + (locals.var_rsdrift * locals.var_t6_dn11)), ((locals.var_rsdrift_dn14 * locals.var_t6) + (locals.var_rsdrift * locals.var_t6_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign21570_e16591;
        locals.var_t1_dn0 = assign21570_e16591_d_n0;
        locals.var_t1_dn2 = assign21570_e16591_d_n2;
        locals.var_t1_dn4 = assign21570_e16591_d_n4;
        locals.var_t1_dn5 = assign21570_e16591_d_n5;
        locals.var_t1_dn6 = assign21570_e16591_d_n6;
        locals.var_t1_dn7 = assign21570_e16591_d_n7;
        locals.var_t1_dn8 = assign21570_e16591_d_n8;
        locals.var_t1_dn9 = assign21570_e16591_d_n9;
        locals.var_t1_dn10 = assign21570_e16591_d_n10;
        locals.var_t1_dn11 = assign21570_e16591_d_n11;
        locals.var_t1_dn14 = assign21570_e16591_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign21580_e16605, assign21580_e16605_d_n0, assign21580_e16605_d_n2, assign21580_e16605_d_n4, assign21580_e16605_d_n5, assign21580_e16605_d_n6, assign21580_e16605_d_n7, assign21580_e16605_d_n8, assign21580_e16605_d_n9, assign21580_e16605_d_n10, assign21580_e16605_d_n11, assign21580_e16605_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) {
        let assign21580_e16599: f64 = (locals.var_t0 * locals.var_vdsemodenml);
        let assign21580_e16602: f64 = (locals.var_rdrift * locals.var_vdsemodervs);
        let assign21580_e16603: f64 = (assign21580_e16599 + assign21580_e16602);
        (assign21580_e16603, ((locals.var_t0_dn0 * locals.var_vdsemodenml) + (locals.var_rdrift_dn0 * locals.var_vdsemodervs)), ((locals.var_t0_dn2 * locals.var_vdsemodenml) + (locals.var_rdrift_dn2 * locals.var_vdsemodervs)), ((locals.var_t0_dn4 * locals.var_vdsemodenml) + (locals.var_rdrift_dn4 * locals.var_vdsemodervs)), ((locals.var_t0_dn5 * locals.var_vdsemodenml) + (locals.var_rdrift_dn5 * locals.var_vdsemodervs)), ((locals.var_t0_dn6 * locals.var_vdsemodenml) + (locals.var_rdrift_dn6 * locals.var_vdsemodervs)), ((locals.var_t0_dn7 * locals.var_vdsemodenml) + (locals.var_rdrift_dn7 * locals.var_vdsemodervs)), ((locals.var_t0_dn8 * locals.var_vdsemodenml) + (locals.var_rdrift_dn8 * locals.var_vdsemodervs)), ((locals.var_t0_dn9 * locals.var_vdsemodenml) + (locals.var_rdrift_dn9 * locals.var_vdsemodervs)), ((locals.var_t0_dn10 * locals.var_vdsemodenml) + (locals.var_rdrift_dn10 * locals.var_vdsemodervs)), ((locals.var_t0_dn11 * locals.var_vdsemodenml) + (locals.var_rdrift_dn11 * locals.var_vdsemodervs)), ((locals.var_t0_dn14 * locals.var_vdsemodenml) + (locals.var_rdrift_dn14 * locals.var_vdsemodervs)),)
    } else {
        (locals.var_rdrift, locals.var_rdrift_dn0, locals.var_rdrift_dn2, locals.var_rdrift_dn4, locals.var_rdrift_dn5, locals.var_rdrift_dn6, locals.var_rdrift_dn7, locals.var_rdrift_dn8, locals.var_rdrift_dn9, locals.var_rdrift_dn10, locals.var_rdrift_dn11, locals.var_rdrift_dn14,)
    }
};
        locals.var_rdrift = assign21580_e16605;
        locals.var_rdrift_dn0 = assign21580_e16605_d_n0;
        locals.var_rdrift_dn2 = assign21580_e16605_d_n2;
        locals.var_rdrift_dn4 = assign21580_e16605_d_n4;
        locals.var_rdrift_dn5 = assign21580_e16605_d_n5;
        locals.var_rdrift_dn6 = assign21580_e16605_d_n6;
        locals.var_rdrift_dn7 = assign21580_e16605_d_n7;
        locals.var_rdrift_dn8 = assign21580_e16605_d_n8;
        locals.var_rdrift_dn9 = assign21580_e16605_d_n9;
        locals.var_rdrift_dn10 = assign21580_e16605_d_n10;
        locals.var_rdrift_dn11 = assign21580_e16605_d_n11;
        locals.var_rdrift_dn14 = assign21580_e16605_d_n14;
        locals.var_rdrift_rv = 0.0;

        let (assign21590_e16619, assign21590_e16619_d_n0, assign21590_e16619_d_n2, assign21590_e16619_d_n4, assign21590_e16619_d_n5, assign21590_e16619_d_n6, assign21590_e16619_d_n7, assign21590_e16619_d_n8, assign21590_e16619_d_n9, assign21590_e16619_d_n10, assign21590_e16619_d_n11, assign21590_e16619_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) {
        let assign21590_e16613: f64 = (locals.var_t1 * locals.var_vdsemodervs);
        let assign21590_e16616: f64 = (locals.var_rsdrift * locals.var_vdsemodenml);
        let assign21590_e16617: f64 = (assign21590_e16613 + assign21590_e16616);
        (assign21590_e16617, ((locals.var_t1_dn0 * locals.var_vdsemodervs) + (locals.var_rsdrift_dn0 * locals.var_vdsemodenml)), ((locals.var_t1_dn2 * locals.var_vdsemodervs) + (locals.var_rsdrift_dn2 * locals.var_vdsemodenml)), ((locals.var_t1_dn4 * locals.var_vdsemodervs) + (locals.var_rsdrift_dn4 * locals.var_vdsemodenml)), ((locals.var_t1_dn5 * locals.var_vdsemodervs) + (locals.var_rsdrift_dn5 * locals.var_vdsemodenml)), ((locals.var_t1_dn6 * locals.var_vdsemodervs) + (locals.var_rsdrift_dn6 * locals.var_vdsemodenml)), ((locals.var_t1_dn7 * locals.var_vdsemodervs) + (locals.var_rsdrift_dn7 * locals.var_vdsemodenml)), ((locals.var_t1_dn8 * locals.var_vdsemodervs) + (locals.var_rsdrift_dn8 * locals.var_vdsemodenml)), ((locals.var_t1_dn9 * locals.var_vdsemodervs) + (locals.var_rsdrift_dn9 * locals.var_vdsemodenml)), ((locals.var_t1_dn10 * locals.var_vdsemodervs) + (locals.var_rsdrift_dn10 * locals.var_vdsemodenml)), ((locals.var_t1_dn11 * locals.var_vdsemodervs) + (locals.var_rsdrift_dn11 * locals.var_vdsemodenml)), ((locals.var_t1_dn14 * locals.var_vdsemodervs) + (locals.var_rsdrift_dn14 * locals.var_vdsemodenml)),)
    } else {
        (locals.var_rsdrift, locals.var_rsdrift_dn0, locals.var_rsdrift_dn2, locals.var_rsdrift_dn4, locals.var_rsdrift_dn5, locals.var_rsdrift_dn6, locals.var_rsdrift_dn7, locals.var_rsdrift_dn8, locals.var_rsdrift_dn9, locals.var_rsdrift_dn10, locals.var_rsdrift_dn11, locals.var_rsdrift_dn14,)
    }
};
        locals.var_rsdrift = assign21590_e16619;
        locals.var_rsdrift_dn0 = assign21590_e16619_d_n0;
        locals.var_rsdrift_dn2 = assign21590_e16619_d_n2;
        locals.var_rsdrift_dn4 = assign21590_e16619_d_n4;
        locals.var_rsdrift_dn5 = assign21590_e16619_d_n5;
        locals.var_rsdrift_dn6 = assign21590_e16619_d_n6;
        locals.var_rsdrift_dn7 = assign21590_e16619_d_n7;
        locals.var_rsdrift_dn8 = assign21590_e16619_d_n8;
        locals.var_rsdrift_dn9 = assign21590_e16619_d_n9;
        locals.var_rsdrift_dn10 = assign21590_e16619_d_n10;
        locals.var_rsdrift_dn11 = assign21590_e16619_d_n11;
        locals.var_rsdrift_dn14 = assign21590_e16619_d_n14;
        locals.var_rsdrift_rv = 0.0;

        let (assign21600_e16628, assign21600_e16628_d_n0, assign21600_e16628_d_n2, assign21600_e16628_d_n4, assign21600_e16628_d_n5, assign21600_e16628_d_n6, assign21600_e16628_d_n7, assign21600_e16628_d_n8, assign21600_e16628_d_n9, assign21600_e16628_d_n10, assign21600_e16628_d_n11, assign21600_e16628_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn4, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, locals.var_wdep_dn9, locals.var_wdep_dn10, locals.var_wdep_dn11, locals.var_wdep_dn14,)
    }
};
        locals.var_wdep = assign21600_e16628;
        locals.var_wdep_dn0 = assign21600_e16628_d_n0;
        locals.var_wdep_dn2 = assign21600_e16628_d_n2;
        locals.var_wdep_dn4 = assign21600_e16628_d_n4;
        locals.var_wdep_dn5 = assign21600_e16628_d_n5;
        locals.var_wdep_dn6 = assign21600_e16628_d_n6;
        locals.var_wdep_dn7 = assign21600_e16628_d_n7;
        locals.var_wdep_dn8 = assign21600_e16628_d_n8;
        locals.var_wdep_dn9 = assign21600_e16628_d_n9;
        locals.var_wdep_dn10 = assign21600_e16628_d_n10;
        locals.var_wdep_dn11 = assign21600_e16628_d_n11;
        locals.var_wdep_dn14 = assign21600_e16628_d_n14;
        locals.var_wdep_rv = 0.0;

        let (assign21610_e16634, assign21610_e16634_d_n0, assign21610_e16634_d_n2, assign21610_e16634_d_n4, assign21610_e16634_d_n5, assign21610_e16634_d_n6, assign21610_e16634_d_n7, assign21610_e16634_d_n8, assign21610_e16634_d_n9, assign21610_e16634_d_n10, assign21610_e16634_d_n11, assign21610_e16634_d_n14,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) {
        (locals.var_rdrift, locals.var_rdrift_dn0, locals.var_rdrift_dn2, locals.var_rdrift_dn4, locals.var_rdrift_dn5, locals.var_rdrift_dn6, locals.var_rdrift_dn7, locals.var_rdrift_dn8, locals.var_rdrift_dn9, locals.var_rdrift_dn10, locals.var_rdrift_dn11, locals.var_rdrift_dn14,)
    } else {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14,)
    }
};
        locals.var_rdd = assign21610_e16634;
        locals.var_rdd_dn0 = assign21610_e16634_d_n0;
        locals.var_rdd_dn2 = assign21610_e16634_d_n2;
        locals.var_rdd_dn4 = assign21610_e16634_d_n4;
        locals.var_rdd_dn5 = assign21610_e16634_d_n5;
        locals.var_rdd_dn6 = assign21610_e16634_d_n6;
        locals.var_rdd_dn7 = assign21610_e16634_d_n7;
        locals.var_rdd_dn8 = assign21610_e16634_d_n8;
        locals.var_rdd_dn9 = assign21610_e16634_d_n9;
        locals.var_rdd_dn10 = assign21610_e16634_d_n10;
        locals.var_rdd_dn11 = assign21610_e16634_d_n11;
        locals.var_rdd_dn14 = assign21610_e16634_d_n14;
        locals.var_rdd_rv = 0.0;

        let (assign21620_e16640, assign21620_e16640_d_n0, assign21620_e16640_d_n2, assign21620_e16640_d_n4, assign21620_e16640_d_n5, assign21620_e16640_d_n6, assign21620_e16640_d_n7, assign21620_e16640_d_n8, assign21620_e16640_d_n9, assign21620_e16640_d_n10, assign21620_e16640_d_n11, assign21620_e16640_d_n14,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) {
        (locals.var_rsdrift, locals.var_rsdrift_dn0, locals.var_rsdrift_dn2, locals.var_rsdrift_dn4, locals.var_rsdrift_dn5, locals.var_rsdrift_dn6, locals.var_rsdrift_dn7, locals.var_rsdrift_dn8, locals.var_rsdrift_dn9, locals.var_rsdrift_dn10, locals.var_rsdrift_dn11, locals.var_rsdrift_dn14,)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn14,)
    }
};
        locals.var_rsd = assign21620_e16640;
        locals.var_rsd_dn0 = assign21620_e16640_d_n0;
        locals.var_rsd_dn2 = assign21620_e16640_d_n2;
        locals.var_rsd_dn4 = assign21620_e16640_d_n4;
        locals.var_rsd_dn5 = assign21620_e16640_d_n5;
        locals.var_rsd_dn6 = assign21620_e16640_d_n6;
        locals.var_rsd_dn7 = assign21620_e16640_d_n7;
        locals.var_rsd_dn8 = assign21620_e16640_d_n8;
        locals.var_rsd_dn9 = assign21620_e16640_d_n9;
        locals.var_rsd_dn10 = assign21620_e16640_d_n10;
        locals.var_rsd_dn11 = assign21620_e16640_d_n11;
        locals.var_rsd_dn14 = assign21620_e16640_d_n14;
        locals.var_rsd_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_57(
        locals: &mut StampLocals,
    ) {
        let (assign21630_e16653, assign21630_e16653_d_n0, assign21630_e16653_d_n2, assign21630_e16653_d_n4, assign21630_e16653_d_n5, assign21630_e16653_d_n6, assign21630_e16653_d_n7, assign21630_e16653_d_n8, assign21630_e16653_d_n9, assign21630_e16653_d_n10, assign21630_e16653_d_n11, assign21630_e16653_d_n14,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard415 == 0.0)) {
        let assign21630_e16647: f64 = (locals.var_vdsemodenml * locals.var_rde);
        let assign21630_e16650: f64 = (locals.var_vdsemodervs * locals.var_rse);
        let assign21630_e16651: f64 = (assign21630_e16647 + assign21630_e16650);
        (assign21630_e16651, ((locals.var_vdsemodenml * locals.var_rde_dn0) + (locals.var_vdsemodervs * locals.var_rse_dn0)), ((locals.var_vdsemodenml * locals.var_rde_dn2) + (locals.var_vdsemodervs * locals.var_rse_dn2)), ((locals.var_vdsemodenml * locals.var_rde_dn4) + (locals.var_vdsemodervs * locals.var_rse_dn4)), ((locals.var_vdsemodenml * locals.var_rde_dn5) + (locals.var_vdsemodervs * locals.var_rse_dn5)), ((locals.var_vdsemodenml * locals.var_rde_dn6) + (locals.var_vdsemodervs * locals.var_rse_dn6)), ((locals.var_vdsemodenml * locals.var_rde_dn7) + (locals.var_vdsemodervs * locals.var_rse_dn7)), ((locals.var_vdsemodenml * locals.var_rde_dn8) + (locals.var_vdsemodervs * locals.var_rse_dn8)), ((locals.var_vdsemodenml * locals.var_rde_dn9) + (locals.var_vdsemodervs * locals.var_rse_dn9)), ((locals.var_vdsemodenml * locals.var_rde_dn10) + (locals.var_vdsemodervs * locals.var_rse_dn10)), ((locals.var_vdsemodenml * locals.var_rde_dn11) + (locals.var_vdsemodervs * locals.var_rse_dn11)), ((locals.var_vdsemodenml * locals.var_rde_dn14) + (locals.var_vdsemodervs * locals.var_rse_dn14)),)
    } else {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14,)
    }
};
        locals.var_rdd = assign21630_e16653;
        locals.var_rdd_dn0 = assign21630_e16653_d_n0;
        locals.var_rdd_dn2 = assign21630_e16653_d_n2;
        locals.var_rdd_dn4 = assign21630_e16653_d_n4;
        locals.var_rdd_dn5 = assign21630_e16653_d_n5;
        locals.var_rdd_dn6 = assign21630_e16653_d_n6;
        locals.var_rdd_dn7 = assign21630_e16653_d_n7;
        locals.var_rdd_dn8 = assign21630_e16653_d_n8;
        locals.var_rdd_dn9 = assign21630_e16653_d_n9;
        locals.var_rdd_dn10 = assign21630_e16653_d_n10;
        locals.var_rdd_dn11 = assign21630_e16653_d_n11;
        locals.var_rdd_dn14 = assign21630_e16653_d_n14;
        locals.var_rdd_rv = 0.0;

        let (assign21640_e16666, assign21640_e16666_d_n0, assign21640_e16666_d_n2, assign21640_e16666_d_n4, assign21640_e16666_d_n5, assign21640_e16666_d_n6, assign21640_e16666_d_n7, assign21640_e16666_d_n8, assign21640_e16666_d_n9, assign21640_e16666_d_n10, assign21640_e16666_d_n11, assign21640_e16666_d_n14,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard415 == 0.0)) {
        let assign21640_e16660: f64 = (locals.var_vdsemodenml * locals.var_rse);
        let assign21640_e16663: f64 = (locals.var_vdsemodervs * locals.var_rde);
        let assign21640_e16664: f64 = (assign21640_e16660 + assign21640_e16663);
        (assign21640_e16664, ((locals.var_vdsemodenml * locals.var_rse_dn0) + (locals.var_vdsemodervs * locals.var_rde_dn0)), ((locals.var_vdsemodenml * locals.var_rse_dn2) + (locals.var_vdsemodervs * locals.var_rde_dn2)), ((locals.var_vdsemodenml * locals.var_rse_dn4) + (locals.var_vdsemodervs * locals.var_rde_dn4)), ((locals.var_vdsemodenml * locals.var_rse_dn5) + (locals.var_vdsemodervs * locals.var_rde_dn5)), ((locals.var_vdsemodenml * locals.var_rse_dn6) + (locals.var_vdsemodervs * locals.var_rde_dn6)), ((locals.var_vdsemodenml * locals.var_rse_dn7) + (locals.var_vdsemodervs * locals.var_rde_dn7)), ((locals.var_vdsemodenml * locals.var_rse_dn8) + (locals.var_vdsemodervs * locals.var_rde_dn8)), ((locals.var_vdsemodenml * locals.var_rse_dn9) + (locals.var_vdsemodervs * locals.var_rde_dn9)), ((locals.var_vdsemodenml * locals.var_rse_dn10) + (locals.var_vdsemodervs * locals.var_rde_dn10)), ((locals.var_vdsemodenml * locals.var_rse_dn11) + (locals.var_vdsemodervs * locals.var_rde_dn11)), ((locals.var_vdsemodenml * locals.var_rse_dn14) + (locals.var_vdsemodervs * locals.var_rde_dn14)),)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn14,)
    }
};
        locals.var_rsd = assign21640_e16666;
        locals.var_rsd_dn0 = assign21640_e16666_d_n0;
        locals.var_rsd_dn2 = assign21640_e16666_d_n2;
        locals.var_rsd_dn4 = assign21640_e16666_d_n4;
        locals.var_rsd_dn5 = assign21640_e16666_d_n5;
        locals.var_rsd_dn6 = assign21640_e16666_d_n6;
        locals.var_rsd_dn7 = assign21640_e16666_d_n7;
        locals.var_rsd_dn8 = assign21640_e16666_d_n8;
        locals.var_rsd_dn9 = assign21640_e16666_d_n9;
        locals.var_rsd_dn10 = assign21640_e16666_d_n10;
        locals.var_rsd_dn11 = assign21640_e16666_d_n11;
        locals.var_rsd_dn14 = assign21640_e16666_d_n14;
        locals.var_rsd_rv = 0.0;

        let (assign21650_e16672, assign21650_e16672_d_n0, assign21650_e16672_d_n2, assign21650_e16672_d_n4, assign21650_e16672_d_n5, assign21650_e16672_d_n6, assign21650_e16672_d_n7, assign21650_e16672_d_n8, assign21650_e16672_d_n9, assign21650_e16672_d_n10, assign21650_e16672_d_n11, assign21650_e16672_d_n14,) = {
    if (locals.var_guard413 != 0.0) {
        let assign21650_e16670: f64 = (locals.var_rdd / locals.var_weffld_nf);
        (assign21650_e16670, (locals.var_rdd_dn0 / locals.var_weffld_nf), (locals.var_rdd_dn2 / locals.var_weffld_nf), (locals.var_rdd_dn4 / locals.var_weffld_nf), (locals.var_rdd_dn5 / locals.var_weffld_nf), (locals.var_rdd_dn6 / locals.var_weffld_nf), (locals.var_rdd_dn7 / locals.var_weffld_nf), (locals.var_rdd_dn8 / locals.var_weffld_nf), (locals.var_rdd_dn9 / locals.var_weffld_nf), (locals.var_rdd_dn10 / locals.var_weffld_nf), (locals.var_rdd_dn11 / locals.var_weffld_nf), (locals.var_rdd_dn14 / locals.var_weffld_nf),)
    } else {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14,)
    }
};
        locals.var_rdd = assign21650_e16672;
        locals.var_rdd_dn0 = assign21650_e16672_d_n0;
        locals.var_rdd_dn2 = assign21650_e16672_d_n2;
        locals.var_rdd_dn4 = assign21650_e16672_d_n4;
        locals.var_rdd_dn5 = assign21650_e16672_d_n5;
        locals.var_rdd_dn6 = assign21650_e16672_d_n6;
        locals.var_rdd_dn7 = assign21650_e16672_d_n7;
        locals.var_rdd_dn8 = assign21650_e16672_d_n8;
        locals.var_rdd_dn9 = assign21650_e16672_d_n9;
        locals.var_rdd_dn10 = assign21650_e16672_d_n10;
        locals.var_rdd_dn11 = assign21650_e16672_d_n11;
        locals.var_rdd_dn14 = assign21650_e16672_d_n14;
        locals.var_rdd_rv = 0.0;

        let (assign21660_e16678, assign21660_e16678_d_n0, assign21660_e16678_d_n2, assign21660_e16678_d_n4, assign21660_e16678_d_n5, assign21660_e16678_d_n6, assign21660_e16678_d_n7, assign21660_e16678_d_n8, assign21660_e16678_d_n9, assign21660_e16678_d_n10, assign21660_e16678_d_n11, assign21660_e16678_d_n14,) = {
    if (locals.var_guard413 != 0.0) {
        let assign21660_e16676: f64 = (locals.var_rsd / locals.var_weffld_nf);
        (assign21660_e16676, (locals.var_rsd_dn0 / locals.var_weffld_nf), (locals.var_rsd_dn2 / locals.var_weffld_nf), (locals.var_rsd_dn4 / locals.var_weffld_nf), (locals.var_rsd_dn5 / locals.var_weffld_nf), (locals.var_rsd_dn6 / locals.var_weffld_nf), (locals.var_rsd_dn7 / locals.var_weffld_nf), (locals.var_rsd_dn8 / locals.var_weffld_nf), (locals.var_rsd_dn9 / locals.var_weffld_nf), (locals.var_rsd_dn10 / locals.var_weffld_nf), (locals.var_rsd_dn11 / locals.var_weffld_nf), (locals.var_rsd_dn14 / locals.var_weffld_nf),)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn14,)
    }
};
        locals.var_rsd = assign21660_e16678;
        locals.var_rsd_dn0 = assign21660_e16678_d_n0;
        locals.var_rsd_dn2 = assign21660_e16678_d_n2;
        locals.var_rsd_dn4 = assign21660_e16678_d_n4;
        locals.var_rsd_dn5 = assign21660_e16678_d_n5;
        locals.var_rsd_dn6 = assign21660_e16678_d_n6;
        locals.var_rsd_dn7 = assign21660_e16678_d_n7;
        locals.var_rsd_dn8 = assign21660_e16678_d_n8;
        locals.var_rsd_dn9 = assign21660_e16678_d_n9;
        locals.var_rsd_dn10 = assign21660_e16678_d_n10;
        locals.var_rsd_dn11 = assign21660_e16678_d_n11;
        locals.var_rsd_dn14 = assign21660_e16678_d_n14;
        locals.var_rsd_rv = 0.0;

        let (assign21670_e16690, assign21670_e16690_d_n0, assign21670_e16690_d_n2, assign21670_e16690_d_n4, assign21670_e16690_d_n5, assign21670_e16690_d_n6, assign21670_e16690_d_n7, assign21670_e16690_d_n8, assign21670_e16690_d_n9, assign21670_e16690_d_n10, assign21670_e16690_d_n11, assign21670_e16690_d_n14,) = {
    if (locals.var_guard413 != 0.0) {
        let assign21670_e16683: f64 = (locals.var_vdsemodenml * locals.var_rd0);
        let assign21670_e16684: f64 = (locals.var_rdd + assign21670_e16683);
        let assign21670_e16687: f64 = (locals.var_vdsemodervs * locals.var_rs0);
        let assign21670_e16688: f64 = (assign21670_e16684 + assign21670_e16687);
        (assign21670_e16688, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14,)
    } else {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14,)
    }
};
        locals.var_rdd = assign21670_e16690;
        locals.var_rdd_dn0 = assign21670_e16690_d_n0;
        locals.var_rdd_dn2 = assign21670_e16690_d_n2;
        locals.var_rdd_dn4 = assign21670_e16690_d_n4;
        locals.var_rdd_dn5 = assign21670_e16690_d_n5;
        locals.var_rdd_dn6 = assign21670_e16690_d_n6;
        locals.var_rdd_dn7 = assign21670_e16690_d_n7;
        locals.var_rdd_dn8 = assign21670_e16690_d_n8;
        locals.var_rdd_dn9 = assign21670_e16690_d_n9;
        locals.var_rdd_dn10 = assign21670_e16690_d_n10;
        locals.var_rdd_dn11 = assign21670_e16690_d_n11;
        locals.var_rdd_dn14 = assign21670_e16690_d_n14;
        locals.var_rdd_rv = 0.0;

        let (assign21680_e16702, assign21680_e16702_d_n0, assign21680_e16702_d_n2, assign21680_e16702_d_n4, assign21680_e16702_d_n5, assign21680_e16702_d_n6, assign21680_e16702_d_n7, assign21680_e16702_d_n8, assign21680_e16702_d_n9, assign21680_e16702_d_n10, assign21680_e16702_d_n11, assign21680_e16702_d_n14,) = {
    if (locals.var_guard413 != 0.0) {
        let assign21680_e16695: f64 = (locals.var_vdsemodenml * locals.var_rs0);
        let assign21680_e16696: f64 = (locals.var_rsd + assign21680_e16695);
        let assign21680_e16699: f64 = (locals.var_vdsemodervs * locals.var_rd0);
        let assign21680_e16700: f64 = (assign21680_e16696 + assign21680_e16699);
        (assign21680_e16700, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn14,)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn14,)
    }
};
        locals.var_rsd = assign21680_e16702;
        locals.var_rsd_dn0 = assign21680_e16702_d_n0;
        locals.var_rsd_dn2 = assign21680_e16702_d_n2;
        locals.var_rsd_dn4 = assign21680_e16702_d_n4;
        locals.var_rsd_dn5 = assign21680_e16702_d_n5;
        locals.var_rsd_dn6 = assign21680_e16702_d_n6;
        locals.var_rsd_dn7 = assign21680_e16702_d_n7;
        locals.var_rsd_dn8 = assign21680_e16702_d_n8;
        locals.var_rsd_dn9 = assign21680_e16702_d_n9;
        locals.var_rsd_dn10 = assign21680_e16702_d_n10;
        locals.var_rsd_dn11 = assign21680_e16702_d_n11;
        locals.var_rsd_dn14 = assign21680_e16702_d_n14;
        locals.var_rsd_rv = 0.0;

        let (assign21690_e16712, assign21690_e16712_d_n0, assign21690_e16712_d_n2, assign21690_e16712_d_n4, assign21690_e16712_d_n5, assign21690_e16712_d_n6, assign21690_e16712_d_n7, assign21690_e16712_d_n8, assign21690_e16712_d_n9, assign21690_e16712_d_n10, assign21690_e16712_d_n11, assign21690_e16712_d_n14,) = {
    if (locals.var_guard413 != 0.0) {
        let assign21690_e16706: f64 = (locals.var_vdsemodenml * locals.var_rdd);
        let assign21690_e16709: f64 = (locals.var_vdsemodervs * locals.var_rsd);
        let assign21690_e16710: f64 = (assign21690_e16706 + assign21690_e16709);
        (assign21690_e16710, ((locals.var_vdsemodenml * locals.var_rdd_dn0) + (locals.var_vdsemodervs * locals.var_rsd_dn0)), ((locals.var_vdsemodenml * locals.var_rdd_dn2) + (locals.var_vdsemodervs * locals.var_rsd_dn2)), ((locals.var_vdsemodenml * locals.var_rdd_dn4) + (locals.var_vdsemodervs * locals.var_rsd_dn4)), ((locals.var_vdsemodenml * locals.var_rdd_dn5) + (locals.var_vdsemodervs * locals.var_rsd_dn5)), ((locals.var_vdsemodenml * locals.var_rdd_dn6) + (locals.var_vdsemodervs * locals.var_rsd_dn6)), ((locals.var_vdsemodenml * locals.var_rdd_dn7) + (locals.var_vdsemodervs * locals.var_rsd_dn7)), ((locals.var_vdsemodenml * locals.var_rdd_dn8) + (locals.var_vdsemodervs * locals.var_rsd_dn8)), ((locals.var_vdsemodenml * locals.var_rdd_dn9) + (locals.var_vdsemodervs * locals.var_rsd_dn9)), ((locals.var_vdsemodenml * locals.var_rdd_dn10) + (locals.var_vdsemodervs * locals.var_rsd_dn10)), ((locals.var_vdsemodenml * locals.var_rdd_dn11) + (locals.var_vdsemodervs * locals.var_rsd_dn11)), ((locals.var_vdsemodenml * locals.var_rdd_dn14) + (locals.var_vdsemodervs * locals.var_rsd_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21690_e16712;
        locals.var_t0_dn0 = assign21690_e16712_d_n0;
        locals.var_t0_dn2 = assign21690_e16712_d_n2;
        locals.var_t0_dn4 = assign21690_e16712_d_n4;
        locals.var_t0_dn5 = assign21690_e16712_d_n5;
        locals.var_t0_dn6 = assign21690_e16712_d_n6;
        locals.var_t0_dn7 = assign21690_e16712_d_n7;
        locals.var_t0_dn8 = assign21690_e16712_d_n8;
        locals.var_t0_dn9 = assign21690_e16712_d_n9;
        locals.var_t0_dn10 = assign21690_e16712_d_n10;
        locals.var_t0_dn11 = assign21690_e16712_d_n11;
        locals.var_t0_dn14 = assign21690_e16712_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign21730_e16744, assign21730_e16744_d_n0, assign21730_e16744_d_n2, assign21730_e16744_d_n4, assign21730_e16744_d_n5, assign21730_e16744_d_n6, assign21730_e16744_d_n7, assign21730_e16744_d_n8, assign21730_e16744_d_n9, assign21730_e16744_d_n10, assign21730_e16744_d_n11, assign21730_e16744_d_n14,) = {
    if (locals.var_guard413 != 0.0) {
        let assign21730_e16738: f64 = (locals.var_vdsemodenml * locals.var_rsd);
        let assign21730_e16741: f64 = (locals.var_vdsemodervs * locals.var_rdd);
        let assign21730_e16742: f64 = (assign21730_e16738 + assign21730_e16741);
        (assign21730_e16742, ((locals.var_vdsemodenml * locals.var_rsd_dn0) + (locals.var_vdsemodervs * locals.var_rdd_dn0)), ((locals.var_vdsemodenml * locals.var_rsd_dn2) + (locals.var_vdsemodervs * locals.var_rdd_dn2)), ((locals.var_vdsemodenml * locals.var_rsd_dn4) + (locals.var_vdsemodervs * locals.var_rdd_dn4)), ((locals.var_vdsemodenml * locals.var_rsd_dn5) + (locals.var_vdsemodervs * locals.var_rdd_dn5)), ((locals.var_vdsemodenml * locals.var_rsd_dn6) + (locals.var_vdsemodervs * locals.var_rdd_dn6)), ((locals.var_vdsemodenml * locals.var_rsd_dn7) + (locals.var_vdsemodervs * locals.var_rdd_dn7)), ((locals.var_vdsemodenml * locals.var_rsd_dn8) + (locals.var_vdsemodervs * locals.var_rdd_dn8)), ((locals.var_vdsemodenml * locals.var_rsd_dn9) + (locals.var_vdsemodervs * locals.var_rdd_dn9)), ((locals.var_vdsemodenml * locals.var_rsd_dn10) + (locals.var_vdsemodervs * locals.var_rdd_dn10)), ((locals.var_vdsemodenml * locals.var_rsd_dn11) + (locals.var_vdsemodervs * locals.var_rdd_dn11)), ((locals.var_vdsemodenml * locals.var_rsd_dn14) + (locals.var_vdsemodervs * locals.var_rdd_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21730_e16744;
        locals.var_t0_dn0 = assign21730_e16744_d_n0;
        locals.var_t0_dn2 = assign21730_e16744_d_n2;
        locals.var_t0_dn4 = assign21730_e16744_d_n4;
        locals.var_t0_dn5 = assign21730_e16744_d_n5;
        locals.var_t0_dn6 = assign21730_e16744_d_n6;
        locals.var_t0_dn7 = assign21730_e16744_d_n7;
        locals.var_t0_dn8 = assign21730_e16744_d_n8;
        locals.var_t0_dn9 = assign21730_e16744_d_n9;
        locals.var_t0_dn10 = assign21730_e16744_d_n10;
        locals.var_t0_dn11 = assign21730_e16744_d_n11;
        locals.var_t0_dn14 = assign21730_e16744_d_n14;
        locals.var_t0_rv = 0.0;

        let assign21770_e16769: f64 = if locals.var_vbs > locals.var_vbs_bnd { 1.0 } else { 0.0 };
        locals.var_guard426 = assign21770_e16769;
        locals.var_guard426_rv = 0.0;

        let (assign21780_e16775, assign21780_e16775_d_n0, assign21780_e16775_d_n2, assign21780_e16775_d_n4, assign21780_e16775_d_n5, assign21780_e16775_d_n6, assign21780_e16775_d_n7, assign21780_e16775_d_n8, assign21780_e16775_d_n9, assign21780_e16775_d_n10, assign21780_e16775_d_n11, assign21780_e16775_d_n14,) = {
    if (locals.var_guard426 != 0.0) {
        let assign21780_e16773: f64 = (locals.var_vbs - locals.var_vbs_bnd);
        (assign21780_e16773, (-locals.var_vbs_bnd_dn0), (-locals.var_vbs_bnd_dn2), (-locals.var_vbs_bnd_dn4), (-locals.var_vbs_bnd_dn5), (locals.var_vbs_dn6 - locals.var_vbs_bnd_dn6), (-locals.var_vbs_bnd_dn7), (locals.var_vbs_dn8 - locals.var_vbs_bnd_dn8), (locals.var_vbs_dn9 - locals.var_vbs_bnd_dn9), (-locals.var_vbs_bnd_dn10), (-locals.var_vbs_bnd_dn11), (-locals.var_vbs_bnd_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign21780_e16775;
        locals.var_t1_dn0 = assign21780_e16775_d_n0;
        locals.var_t1_dn2 = assign21780_e16775_d_n2;
        locals.var_t1_dn4 = assign21780_e16775_d_n4;
        locals.var_t1_dn5 = assign21780_e16775_d_n5;
        locals.var_t1_dn6 = assign21780_e16775_d_n6;
        locals.var_t1_dn7 = assign21780_e16775_d_n7;
        locals.var_t1_dn8 = assign21780_e16775_d_n8;
        locals.var_t1_dn9 = assign21780_e16775_d_n9;
        locals.var_t1_dn10 = assign21780_e16775_d_n10;
        locals.var_t1_dn11 = assign21780_e16775_d_n11;
        locals.var_t1_dn14 = assign21780_e16775_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign21790_e16781, assign21790_e16781_d_n0, assign21790_e16781_d_n2, assign21790_e16781_d_n4, assign21790_e16781_d_n5, assign21790_e16781_d_n6, assign21790_e16781_d_n7, assign21790_e16781_d_n8, assign21790_e16781_d_n9, assign21790_e16781_d_n10, assign21790_e16781_d_n11, assign21790_e16781_d_n14,) = {
    if (locals.var_guard426 != 0.0) {
        let assign21790_e16779: f64 = (locals.var_vbs_max - locals.var_vbs_bnd);
        (assign21790_e16779, (locals.var_vbs_max_dn0 - locals.var_vbs_bnd_dn0), (locals.var_vbs_max_dn2 - locals.var_vbs_bnd_dn2), (locals.var_vbs_max_dn4 - locals.var_vbs_bnd_dn4), (locals.var_vbs_max_dn5 - locals.var_vbs_bnd_dn5), (locals.var_vbs_max_dn6 - locals.var_vbs_bnd_dn6), (locals.var_vbs_max_dn7 - locals.var_vbs_bnd_dn7), (locals.var_vbs_max_dn8 - locals.var_vbs_bnd_dn8), (locals.var_vbs_max_dn9 - locals.var_vbs_bnd_dn9), (locals.var_vbs_max_dn10 - locals.var_vbs_bnd_dn10), (locals.var_vbs_max_dn11 - locals.var_vbs_bnd_dn11), (locals.var_vbs_max_dn14 - locals.var_vbs_bnd_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign21790_e16781;
        locals.var_t2_dn0 = assign21790_e16781_d_n0;
        locals.var_t2_dn2 = assign21790_e16781_d_n2;
        locals.var_t2_dn4 = assign21790_e16781_d_n4;
        locals.var_t2_dn5 = assign21790_e16781_d_n5;
        locals.var_t2_dn6 = assign21790_e16781_d_n6;
        locals.var_t2_dn7 = assign21790_e16781_d_n7;
        locals.var_t2_dn8 = assign21790_e16781_d_n8;
        locals.var_t2_dn9 = assign21790_e16781_d_n9;
        locals.var_t2_dn10 = assign21790_e16781_d_n10;
        locals.var_t2_dn11 = assign21790_e16781_d_n11;
        locals.var_t2_dn14 = assign21790_e16781_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign21800_e16787, assign21800_e16787_d_n0, assign21800_e16787_d_n2, assign21800_e16787_d_n4, assign21800_e16787_d_n5, assign21800_e16787_d_n6, assign21800_e16787_d_n7, assign21800_e16787_d_n8, assign21800_e16787_d_n9, assign21800_e16787_d_n10, assign21800_e16787_d_n11, assign21800_e16787_d_n14,) = {
    if (locals.var_guard426 != 0.0) {
        let assign21800_e16785: f64 = (locals.var_t1 / locals.var_t2);
        (assign21800_e16785, (((locals.var_t1_dn0 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn2 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn7 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn9 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn10 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn11 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn14 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn14)) / (locals.var_t2 * locals.var_t2)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign21800_e16787;
        locals.var_tmf1_dn0 = assign21800_e16787_d_n0;
        locals.var_tmf1_dn2 = assign21800_e16787_d_n2;
        locals.var_tmf1_dn4 = assign21800_e16787_d_n4;
        locals.var_tmf1_dn5 = assign21800_e16787_d_n5;
        locals.var_tmf1_dn6 = assign21800_e16787_d_n6;
        locals.var_tmf1_dn7 = assign21800_e16787_d_n7;
        locals.var_tmf1_dn8 = assign21800_e16787_d_n8;
        locals.var_tmf1_dn9 = assign21800_e16787_d_n9;
        locals.var_tmf1_dn10 = assign21800_e16787_d_n10;
        locals.var_tmf1_dn11 = assign21800_e16787_d_n11;
        locals.var_tmf1_dn14 = assign21800_e16787_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign21810_e16793, assign21810_e16793_d_n0, assign21810_e16793_d_n2, assign21810_e16793_d_n4, assign21810_e16793_d_n5, assign21810_e16793_d_n6, assign21810_e16793_d_n7, assign21810_e16793_d_n8, assign21810_e16793_d_n9, assign21810_e16793_d_n10, assign21810_e16793_d_n11, assign21810_e16793_d_n14,) = {
    if (locals.var_guard426 != 0.0) {
        let assign21810_e16791: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign21810_e16791, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign21810_e16793;
        locals.var_tmf2_dn0 = assign21810_e16793_d_n0;
        locals.var_tmf2_dn2 = assign21810_e16793_d_n2;
        locals.var_tmf2_dn4 = assign21810_e16793_d_n4;
        locals.var_tmf2_dn5 = assign21810_e16793_d_n5;
        locals.var_tmf2_dn6 = assign21810_e16793_d_n6;
        locals.var_tmf2_dn7 = assign21810_e16793_d_n7;
        locals.var_tmf2_dn8 = assign21810_e16793_d_n8;
        locals.var_tmf2_dn9 = assign21810_e16793_d_n9;
        locals.var_tmf2_dn10 = assign21810_e16793_d_n10;
        locals.var_tmf2_dn11 = assign21810_e16793_d_n11;
        locals.var_tmf2_dn14 = assign21810_e16793_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign21820_e16799, assign21820_e16799_d_n0, assign21820_e16799_d_n2, assign21820_e16799_d_n4, assign21820_e16799_d_n5, assign21820_e16799_d_n6, assign21820_e16799_d_n7, assign21820_e16799_d_n8, assign21820_e16799_d_n9, assign21820_e16799_d_n10, assign21820_e16799_d_n11, assign21820_e16799_d_n14,) = {
    if (locals.var_guard426 != 0.0) {
        let assign21820_e16797: f64 = (locals.var_tmf2 * locals.var_tmf1);
        (assign21820_e16797, ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn11)), ((locals.var_tmf2_dn14 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign21820_e16799;
        locals.var_tmf3_dn0 = assign21820_e16799_d_n0;
        locals.var_tmf3_dn2 = assign21820_e16799_d_n2;
        locals.var_tmf3_dn4 = assign21820_e16799_d_n4;
        locals.var_tmf3_dn5 = assign21820_e16799_d_n5;
        locals.var_tmf3_dn6 = assign21820_e16799_d_n6;
        locals.var_tmf3_dn7 = assign21820_e16799_d_n7;
        locals.var_tmf3_dn8 = assign21820_e16799_d_n8;
        locals.var_tmf3_dn9 = assign21820_e16799_d_n9;
        locals.var_tmf3_dn10 = assign21820_e16799_d_n10;
        locals.var_tmf3_dn11 = assign21820_e16799_d_n11;
        locals.var_tmf3_dn14 = assign21820_e16799_d_n14;
        locals.var_tmf3_rv = 0.0;

        let (assign21830_e16805, assign21830_e16805_d_n0, assign21830_e16805_d_n2, assign21830_e16805_d_n4, assign21830_e16805_d_n5, assign21830_e16805_d_n6, assign21830_e16805_d_n7, assign21830_e16805_d_n8, assign21830_e16805_d_n9, assign21830_e16805_d_n10, assign21830_e16805_d_n11, assign21830_e16805_d_n14,) = {
    if (locals.var_guard426 != 0.0) {
        let assign21830_e16803: f64 = (locals.var_tmf2 * locals.var_tmf2);
        (assign21830_e16803, ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)), ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn8, locals.var_tmf4_dn9, locals.var_tmf4_dn10, locals.var_tmf4_dn11, locals.var_tmf4_dn14,)
    }
};
        locals.var_tmf4 = assign21830_e16805;
        locals.var_tmf4_dn0 = assign21830_e16805_d_n0;
        locals.var_tmf4_dn2 = assign21830_e16805_d_n2;
        locals.var_tmf4_dn4 = assign21830_e16805_d_n4;
        locals.var_tmf4_dn5 = assign21830_e16805_d_n5;
        locals.var_tmf4_dn6 = assign21830_e16805_d_n6;
        locals.var_tmf4_dn7 = assign21830_e16805_d_n7;
        locals.var_tmf4_dn8 = assign21830_e16805_d_n8;
        locals.var_tmf4_dn9 = assign21830_e16805_d_n9;
        locals.var_tmf4_dn10 = assign21830_e16805_d_n10;
        locals.var_tmf4_dn11 = assign21830_e16805_d_n11;
        locals.var_tmf4_dn14 = assign21830_e16805_d_n14;
        locals.var_tmf4_rv = 0.0;

        let (assign21840_e16819, assign21840_e16819_d_n0, assign21840_e16819_d_n2, assign21840_e16819_d_n4, assign21840_e16819_d_n5, assign21840_e16819_d_n6, assign21840_e16819_d_n7, assign21840_e16819_d_n8, assign21840_e16819_d_n9, assign21840_e16819_d_n10, assign21840_e16819_d_n11, assign21840_e16819_d_n14,) = {
    if (locals.var_guard426 != 0.0) {
        let assign21840_e16810: f64 = (1.0 + locals.var_tmf1);
        let assign21840_e16812: f64 = (assign21840_e16810 + locals.var_tmf2);
        let assign21840_e16814: f64 = (assign21840_e16812 + locals.var_tmf3);
        let assign21840_e16816: f64 = (assign21840_e16814 + locals.var_tmf4);
        let assign21840_e16817: f64 = (1.0 / assign21840_e16816);
        (assign21840_e16817, (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign21840_e16816 * assign21840_e16816))), (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign21840_e16816 * assign21840_e16816))), (-((((locals.var_tmf1_dn4 + locals.var_tmf2_dn4) + locals.var_tmf3_dn4) + locals.var_tmf4_dn4) / (assign21840_e16816 * assign21840_e16816))), (-((((locals.var_tmf1_dn5 + locals.var_tmf2_dn5) + locals.var_tmf3_dn5) + locals.var_tmf4_dn5) / (assign21840_e16816 * assign21840_e16816))), (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign21840_e16816 * assign21840_e16816))), (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign21840_e16816 * assign21840_e16816))), (-((((locals.var_tmf1_dn8 + locals.var_tmf2_dn8) + locals.var_tmf3_dn8) + locals.var_tmf4_dn8) / (assign21840_e16816 * assign21840_e16816))), (-((((locals.var_tmf1_dn9 + locals.var_tmf2_dn9) + locals.var_tmf3_dn9) + locals.var_tmf4_dn9) / (assign21840_e16816 * assign21840_e16816))), (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign21840_e16816 * assign21840_e16816))), (-((((locals.var_tmf1_dn11 + locals.var_tmf2_dn11) + locals.var_tmf3_dn11) + locals.var_tmf4_dn11) / (assign21840_e16816 * assign21840_e16816))), (-((((locals.var_tmf1_dn14 + locals.var_tmf2_dn14) + locals.var_tmf3_dn14) + locals.var_tmf4_dn14) / (assign21840_e16816 * assign21840_e16816))),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign21840_e16819;
        locals.var_tmf0_dn0 = assign21840_e16819_d_n0;
        locals.var_tmf0_dn2 = assign21840_e16819_d_n2;
        locals.var_tmf0_dn4 = assign21840_e16819_d_n4;
        locals.var_tmf0_dn5 = assign21840_e16819_d_n5;
        locals.var_tmf0_dn6 = assign21840_e16819_d_n6;
        locals.var_tmf0_dn7 = assign21840_e16819_d_n7;
        locals.var_tmf0_dn8 = assign21840_e16819_d_n8;
        locals.var_tmf0_dn9 = assign21840_e16819_d_n9;
        locals.var_tmf0_dn10 = assign21840_e16819_d_n10;
        locals.var_tmf0_dn11 = assign21840_e16819_d_n11;
        locals.var_tmf0_dn14 = assign21840_e16819_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign21850_e16840, assign21850_e16840_d_n0, assign21850_e16840_d_n2, assign21850_e16840_d_n4, assign21850_e16840_d_n5, assign21850_e16840_d_n6, assign21850_e16840_d_n7, assign21850_e16840_d_n8, assign21850_e16840_d_n9, assign21850_e16840_d_n10, assign21850_e16840_d_n11, assign21850_e16840_d_n14,) = {
    if (locals.var_guard426 != 0.0) {
        let assign21850_e16824: f64 = (2.0 * locals.var_tmf1);
        let assign21850_e16825: f64 = (1.0 + assign21850_e16824);
        let assign21850_e16828: f64 = (3.0 * locals.var_tmf2);
        let assign21850_e16829: f64 = (assign21850_e16825 + assign21850_e16828);
        let assign21850_e16832: f64 = (4.0 * locals.var_tmf3);
        let assign21850_e16833: f64 = (assign21850_e16829 + assign21850_e16832);
        let assign21850_e16834: f64 = (-assign21850_e16833);
        let assign21850_e16836: f64 = (assign21850_e16834 * locals.var_tmf0);
        let assign21850_e16838: f64 = (assign21850_e16836 * locals.var_tmf0);
        (assign21850_e16838, (((((-(((2.0 * locals.var_tmf1_dn0) + (3.0 * locals.var_tmf2_dn0)) + (4.0 * locals.var_tmf3_dn0))) * locals.var_tmf0) + (assign21850_e16834 * locals.var_tmf0_dn0)) * locals.var_tmf0) + (assign21850_e16836 * locals.var_tmf0_dn0)), (((((-(((2.0 * locals.var_tmf1_dn2) + (3.0 * locals.var_tmf2_dn2)) + (4.0 * locals.var_tmf3_dn2))) * locals.var_tmf0) + (assign21850_e16834 * locals.var_tmf0_dn2)) * locals.var_tmf0) + (assign21850_e16836 * locals.var_tmf0_dn2)), (((((-(((2.0 * locals.var_tmf1_dn4) + (3.0 * locals.var_tmf2_dn4)) + (4.0 * locals.var_tmf3_dn4))) * locals.var_tmf0) + (assign21850_e16834 * locals.var_tmf0_dn4)) * locals.var_tmf0) + (assign21850_e16836 * locals.var_tmf0_dn4)), (((((-(((2.0 * locals.var_tmf1_dn5) + (3.0 * locals.var_tmf2_dn5)) + (4.0 * locals.var_tmf3_dn5))) * locals.var_tmf0) + (assign21850_e16834 * locals.var_tmf0_dn5)) * locals.var_tmf0) + (assign21850_e16836 * locals.var_tmf0_dn5)), (((((-(((2.0 * locals.var_tmf1_dn6) + (3.0 * locals.var_tmf2_dn6)) + (4.0 * locals.var_tmf3_dn6))) * locals.var_tmf0) + (assign21850_e16834 * locals.var_tmf0_dn6)) * locals.var_tmf0) + (assign21850_e16836 * locals.var_tmf0_dn6)), (((((-(((2.0 * locals.var_tmf1_dn7) + (3.0 * locals.var_tmf2_dn7)) + (4.0 * locals.var_tmf3_dn7))) * locals.var_tmf0) + (assign21850_e16834 * locals.var_tmf0_dn7)) * locals.var_tmf0) + (assign21850_e16836 * locals.var_tmf0_dn7)), (((((-(((2.0 * locals.var_tmf1_dn8) + (3.0 * locals.var_tmf2_dn8)) + (4.0 * locals.var_tmf3_dn8))) * locals.var_tmf0) + (assign21850_e16834 * locals.var_tmf0_dn8)) * locals.var_tmf0) + (assign21850_e16836 * locals.var_tmf0_dn8)), (((((-(((2.0 * locals.var_tmf1_dn9) + (3.0 * locals.var_tmf2_dn9)) + (4.0 * locals.var_tmf3_dn9))) * locals.var_tmf0) + (assign21850_e16834 * locals.var_tmf0_dn9)) * locals.var_tmf0) + (assign21850_e16836 * locals.var_tmf0_dn9)), (((((-(((2.0 * locals.var_tmf1_dn10) + (3.0 * locals.var_tmf2_dn10)) + (4.0 * locals.var_tmf3_dn10))) * locals.var_tmf0) + (assign21850_e16834 * locals.var_tmf0_dn10)) * locals.var_tmf0) + (assign21850_e16836 * locals.var_tmf0_dn10)), (((((-(((2.0 * locals.var_tmf1_dn11) + (3.0 * locals.var_tmf2_dn11)) + (4.0 * locals.var_tmf3_dn11))) * locals.var_tmf0) + (assign21850_e16834 * locals.var_tmf0_dn11)) * locals.var_tmf0) + (assign21850_e16836 * locals.var_tmf0_dn11)), (((((-(((2.0 * locals.var_tmf1_dn14) + (3.0 * locals.var_tmf2_dn14)) + (4.0 * locals.var_tmf3_dn14))) * locals.var_tmf0) + (assign21850_e16834 * locals.var_tmf0_dn14)) * locals.var_tmf0) + (assign21850_e16836 * locals.var_tmf0_dn14)),)
    } else {
        (locals.var_vbscldvbs, locals.var_vbscldvbs_dn0, locals.var_vbscldvbs_dn2, locals.var_vbscldvbs_dn4, locals.var_vbscldvbs_dn5, locals.var_vbscldvbs_dn6, locals.var_vbscldvbs_dn7, locals.var_vbscldvbs_dn8, locals.var_vbscldvbs_dn9, locals.var_vbscldvbs_dn10, locals.var_vbscldvbs_dn11, locals.var_vbscldvbs_dn14,)
    }
};
        locals.var_vbscldvbs = assign21850_e16840;
        locals.var_vbscldvbs_dn0 = assign21850_e16840_d_n0;
        locals.var_vbscldvbs_dn2 = assign21850_e16840_d_n2;
        locals.var_vbscldvbs_dn4 = assign21850_e16840_d_n4;
        locals.var_vbscldvbs_dn5 = assign21850_e16840_d_n5;
        locals.var_vbscldvbs_dn6 = assign21850_e16840_d_n6;
        locals.var_vbscldvbs_dn7 = assign21850_e16840_d_n7;
        locals.var_vbscldvbs_dn8 = assign21850_e16840_d_n8;
        locals.var_vbscldvbs_dn9 = assign21850_e16840_d_n9;
        locals.var_vbscldvbs_dn10 = assign21850_e16840_d_n10;
        locals.var_vbscldvbs_dn11 = assign21850_e16840_d_n11;
        locals.var_vbscldvbs_dn14 = assign21850_e16840_d_n14;
        locals.var_vbscldvbs_rv = 0.0;

        let (assign21860_e16848, assign21860_e16848_d_n0, assign21860_e16848_d_n2, assign21860_e16848_d_n4, assign21860_e16848_d_n5, assign21860_e16848_d_n6, assign21860_e16848_d_n7, assign21860_e16848_d_n8, assign21860_e16848_d_n9, assign21860_e16848_d_n10, assign21860_e16848_d_n11, assign21860_e16848_d_n14,) = {
    if (locals.var_guard426 != 0.0) {
        let assign21860_e16845: f64 = (1.0 - locals.var_tmf0);
        let assign21860_e16846: f64 = (locals.var_t2 * assign21860_e16845);
        (assign21860_e16846, ((locals.var_t2_dn0 * assign21860_e16845) + (locals.var_t2 * (-locals.var_tmf0_dn0))), ((locals.var_t2_dn2 * assign21860_e16845) + (locals.var_t2 * (-locals.var_tmf0_dn2))), ((locals.var_t2_dn4 * assign21860_e16845) + (locals.var_t2 * (-locals.var_tmf0_dn4))), ((locals.var_t2_dn5 * assign21860_e16845) + (locals.var_t2 * (-locals.var_tmf0_dn5))), ((locals.var_t2_dn6 * assign21860_e16845) + (locals.var_t2 * (-locals.var_tmf0_dn6))), ((locals.var_t2_dn7 * assign21860_e16845) + (locals.var_t2 * (-locals.var_tmf0_dn7))), ((locals.var_t2_dn8 * assign21860_e16845) + (locals.var_t2 * (-locals.var_tmf0_dn8))), ((locals.var_t2_dn9 * assign21860_e16845) + (locals.var_t2 * (-locals.var_tmf0_dn9))), ((locals.var_t2_dn10 * assign21860_e16845) + (locals.var_t2 * (-locals.var_tmf0_dn10))), ((locals.var_t2_dn11 * assign21860_e16845) + (locals.var_t2 * (-locals.var_tmf0_dn11))), ((locals.var_t2_dn14 * assign21860_e16845) + (locals.var_t2 * (-locals.var_tmf0_dn14))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign21860_e16848;
        locals.var_ty_dn0 = assign21860_e16848_d_n0;
        locals.var_ty_dn2 = assign21860_e16848_d_n2;
        locals.var_ty_dn4 = assign21860_e16848_d_n4;
        locals.var_ty_dn5 = assign21860_e16848_d_n5;
        locals.var_ty_dn6 = assign21860_e16848_d_n6;
        locals.var_ty_dn7 = assign21860_e16848_d_n7;
        locals.var_ty_dn8 = assign21860_e16848_d_n8;
        locals.var_ty_dn9 = assign21860_e16848_d_n9;
        locals.var_ty_dn10 = assign21860_e16848_d_n10;
        locals.var_ty_dn11 = assign21860_e16848_d_n11;
        locals.var_ty_dn14 = assign21860_e16848_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign21870_e16858, assign21870_e16858_d_n0, assign21870_e16858_d_n2, assign21870_e16858_d_n4, assign21870_e16858_d_n5, assign21870_e16858_d_n6, assign21870_e16858_d_n7, assign21870_e16858_d_n8, assign21870_e16858_d_n9, assign21870_e16858_d_n10, assign21870_e16858_d_n11, assign21870_e16858_d_n14,) = {
    if (locals.var_guard426 != 0.0) {
        let assign21870_e16852: f64 = (1.0 - locals.var_tmf0);
        let assign21870_e16855: f64 = (locals.var_tmf1 * locals.var_vbscldvbs);
        let assign21870_e16856: f64 = (assign21870_e16852 + assign21870_e16855);
        (assign21870_e16856, ((-locals.var_tmf0_dn0) + ((locals.var_tmf1_dn0 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn0))), ((-locals.var_tmf0_dn2) + ((locals.var_tmf1_dn2 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn2))), ((-locals.var_tmf0_dn4) + ((locals.var_tmf1_dn4 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn4))), ((-locals.var_tmf0_dn5) + ((locals.var_tmf1_dn5 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn5))), ((-locals.var_tmf0_dn6) + ((locals.var_tmf1_dn6 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn6))), ((-locals.var_tmf0_dn7) + ((locals.var_tmf1_dn7 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn7))), ((-locals.var_tmf0_dn8) + ((locals.var_tmf1_dn8 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn8))), ((-locals.var_tmf0_dn9) + ((locals.var_tmf1_dn9 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn9))), ((-locals.var_tmf0_dn10) + ((locals.var_tmf1_dn10 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn10))), ((-locals.var_tmf0_dn11) + ((locals.var_tmf1_dn11 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn11))), ((-locals.var_tmf0_dn14) + ((locals.var_tmf1_dn14 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21870_e16858;
        locals.var_t0_dn0 = assign21870_e16858_d_n0;
        locals.var_t0_dn2 = assign21870_e16858_d_n2;
        locals.var_t0_dn4 = assign21870_e16858_d_n4;
        locals.var_t0_dn5 = assign21870_e16858_d_n5;
        locals.var_t0_dn6 = assign21870_e16858_d_n6;
        locals.var_t0_dn7 = assign21870_e16858_d_n7;
        locals.var_t0_dn8 = assign21870_e16858_d_n8;
        locals.var_t0_dn9 = assign21870_e16858_d_n9;
        locals.var_t0_dn10 = assign21870_e16858_d_n10;
        locals.var_t0_dn11 = assign21870_e16858_d_n11;
        locals.var_t0_dn14 = assign21870_e16858_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign21880_e16863, assign21880_e16863_d_n0, assign21880_e16863_d_n2, assign21880_e16863_d_n4, assign21880_e16863_d_n5, assign21880_e16863_d_n6, assign21880_e16863_d_n7, assign21880_e16863_d_n8, assign21880_e16863_d_n9, assign21880_e16863_d_n10, assign21880_e16863_d_n11, assign21880_e16863_d_n14,) = {
    if (locals.var_guard426 != 0.0) {
        let assign21880_e16861: f64 = (-locals.var_vbscldvbs);
        (assign21880_e16861, (-locals.var_vbscldvbs_dn0), (-locals.var_vbscldvbs_dn2), (-locals.var_vbscldvbs_dn4), (-locals.var_vbscldvbs_dn5), (-locals.var_vbscldvbs_dn6), (-locals.var_vbscldvbs_dn7), (-locals.var_vbscldvbs_dn8), (-locals.var_vbscldvbs_dn9), (-locals.var_vbscldvbs_dn10), (-locals.var_vbscldvbs_dn11), (-locals.var_vbscldvbs_dn14),)
    } else {
        (locals.var_vbscldvbs, locals.var_vbscldvbs_dn0, locals.var_vbscldvbs_dn2, locals.var_vbscldvbs_dn4, locals.var_vbscldvbs_dn5, locals.var_vbscldvbs_dn6, locals.var_vbscldvbs_dn7, locals.var_vbscldvbs_dn8, locals.var_vbscldvbs_dn9, locals.var_vbscldvbs_dn10, locals.var_vbscldvbs_dn11, locals.var_vbscldvbs_dn14,)
    }
};
        locals.var_vbscldvbs = assign21880_e16863;
        locals.var_vbscldvbs_dn0 = assign21880_e16863_d_n0;
        locals.var_vbscldvbs_dn2 = assign21880_e16863_d_n2;
        locals.var_vbscldvbs_dn4 = assign21880_e16863_d_n4;
        locals.var_vbscldvbs_dn5 = assign21880_e16863_d_n5;
        locals.var_vbscldvbs_dn6 = assign21880_e16863_d_n6;
        locals.var_vbscldvbs_dn7 = assign21880_e16863_d_n7;
        locals.var_vbscldvbs_dn8 = assign21880_e16863_d_n8;
        locals.var_vbscldvbs_dn9 = assign21880_e16863_d_n9;
        locals.var_vbscldvbs_dn10 = assign21880_e16863_d_n10;
        locals.var_vbscldvbs_dn11 = assign21880_e16863_d_n11;
        locals.var_vbscldvbs_dn14 = assign21880_e16863_d_n14;
        locals.var_vbscldvbs_rv = 0.0;

        let (assign21890_e16869, assign21890_e16869_d_n0, assign21890_e16869_d_n2, assign21890_e16869_d_n4, assign21890_e16869_d_n5, assign21890_e16869_d_n6, assign21890_e16869_d_n7, assign21890_e16869_d_n8, assign21890_e16869_d_n9, assign21890_e16869_d_n10, assign21890_e16869_d_n11, assign21890_e16869_d_n14,) = {
    if (locals.var_guard426 != 0.0) {
        let assign21890_e16867: f64 = (locals.var_vbs_bnd + locals.var_ty);
        (assign21890_e16867, (locals.var_vbs_bnd_dn0 + locals.var_ty_dn0), (locals.var_vbs_bnd_dn2 + locals.var_ty_dn2), (locals.var_vbs_bnd_dn4 + locals.var_ty_dn4), (locals.var_vbs_bnd_dn5 + locals.var_ty_dn5), (locals.var_vbs_bnd_dn6 + locals.var_ty_dn6), (locals.var_vbs_bnd_dn7 + locals.var_ty_dn7), (locals.var_vbs_bnd_dn8 + locals.var_ty_dn8), (locals.var_vbs_bnd_dn9 + locals.var_ty_dn9), (locals.var_vbs_bnd_dn10 + locals.var_ty_dn10), (locals.var_vbs_bnd_dn11 + locals.var_ty_dn11), (locals.var_vbs_bnd_dn14 + locals.var_ty_dn14),)
    } else {
        (locals.var_vbscl, locals.var_vbscl_dn0, locals.var_vbscl_dn2, locals.var_vbscl_dn4, locals.var_vbscl_dn5, locals.var_vbscl_dn6, locals.var_vbscl_dn7, locals.var_vbscl_dn8, locals.var_vbscl_dn9, locals.var_vbscl_dn10, locals.var_vbscl_dn11, locals.var_vbscl_dn14,)
    }
};
        locals.var_vbscl = assign21890_e16869;
        locals.var_vbscl_dn0 = assign21890_e16869_d_n0;
        locals.var_vbscl_dn2 = assign21890_e16869_d_n2;
        locals.var_vbscl_dn4 = assign21890_e16869_d_n4;
        locals.var_vbscl_dn5 = assign21890_e16869_d_n5;
        locals.var_vbscl_dn6 = assign21890_e16869_d_n6;
        locals.var_vbscl_dn7 = assign21890_e16869_d_n7;
        locals.var_vbscl_dn8 = assign21890_e16869_d_n8;
        locals.var_vbscl_dn9 = assign21890_e16869_d_n9;
        locals.var_vbscl_dn10 = assign21890_e16869_d_n10;
        locals.var_vbscl_dn11 = assign21890_e16869_d_n11;
        locals.var_vbscl_dn14 = assign21890_e16869_d_n14;
        locals.var_vbscl_rv = 0.0;

        let (assign21900_e16875, assign21900_e16875_d_n0, assign21900_e16875_d_n2, assign21900_e16875_d_n4, assign21900_e16875_d_n5, assign21900_e16875_d_n6, assign21900_e16875_d_n7, assign21900_e16875_d_n8, assign21900_e16875_d_n9, assign21900_e16875_d_n10, assign21900_e16875_d_n11, assign21900_e16875_d_n14,) = {
    if (locals.var_guard426 != 0.0) {
        let assign21900_e16873: f64 = (1.0 / locals.var_t2);
        (assign21900_e16873, (-(locals.var_t2_dn0 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn2 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn9 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn10 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn11 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn14 / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign21900_e16875;
        locals.var_t3_dn0 = assign21900_e16875_d_n0;
        locals.var_t3_dn2 = assign21900_e16875_d_n2;
        locals.var_t3_dn4 = assign21900_e16875_d_n4;
        locals.var_t3_dn5 = assign21900_e16875_d_n5;
        locals.var_t3_dn6 = assign21900_e16875_d_n6;
        locals.var_t3_dn7 = assign21900_e16875_d_n7;
        locals.var_t3_dn8 = assign21900_e16875_d_n8;
        locals.var_t3_dn9 = assign21900_e16875_d_n9;
        locals.var_t3_dn10 = assign21900_e16875_d_n10;
        locals.var_t3_dn11 = assign21900_e16875_d_n11;
        locals.var_t3_dn14 = assign21900_e16875_d_n14;
        locals.var_t3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_58(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign21910_e16881, assign21910_e16881_d_n0, assign21910_e16881_d_n2, assign21910_e16881_d_n4, assign21910_e16881_d_n5, assign21910_e16881_d_n6, assign21910_e16881_d_n7, assign21910_e16881_d_n8, assign21910_e16881_d_n9, assign21910_e16881_d_n10, assign21910_e16881_d_n11, assign21910_e16881_d_n14,) = {
    if (locals.var_guard426 != 0.0) {
        let assign21910_e16879: f64 = (locals.var_t1 * locals.var_t3);
        (assign21910_e16879, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)), ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)), ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)), ((locals.var_t1_dn14 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign21910_e16881;
        locals.var_t4_dn0 = assign21910_e16881_d_n0;
        locals.var_t4_dn2 = assign21910_e16881_d_n2;
        locals.var_t4_dn4 = assign21910_e16881_d_n4;
        locals.var_t4_dn5 = assign21910_e16881_d_n5;
        locals.var_t4_dn6 = assign21910_e16881_d_n6;
        locals.var_t4_dn7 = assign21910_e16881_d_n7;
        locals.var_t4_dn8 = assign21910_e16881_d_n8;
        locals.var_t4_dn9 = assign21910_e16881_d_n9;
        locals.var_t4_dn10 = assign21910_e16881_d_n10;
        locals.var_t4_dn11 = assign21910_e16881_d_n11;
        locals.var_t4_dn14 = assign21910_e16881_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign21920_e16887, assign21920_e16887_d_n0, assign21920_e16887_d_n2, assign21920_e16887_d_n4, assign21920_e16887_d_n5, assign21920_e16887_d_n6, assign21920_e16887_d_n7, assign21920_e16887_d_n8, assign21920_e16887_d_n9, assign21920_e16887_d_n10, assign21920_e16887_d_n11, assign21920_e16887_d_n14,) = {
    if (locals.var_guard426 != 0.0) {
        let assign21920_e16885: f64 = (locals.var_t4 * locals.var_t4);
        (assign21920_e16885, ((locals.var_t4_dn0 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn0)), ((locals.var_t4_dn2 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn2)), ((locals.var_t4_dn4 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn4)), ((locals.var_t4_dn5 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn5)), ((locals.var_t4_dn6 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn6)), ((locals.var_t4_dn7 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn7)), ((locals.var_t4_dn8 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn8)), ((locals.var_t4_dn9 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn9)), ((locals.var_t4_dn10 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn10)), ((locals.var_t4_dn11 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn11)), ((locals.var_t4_dn14 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign21920_e16887;
        locals.var_t5_dn0 = assign21920_e16887_d_n0;
        locals.var_t5_dn2 = assign21920_e16887_d_n2;
        locals.var_t5_dn4 = assign21920_e16887_d_n4;
        locals.var_t5_dn5 = assign21920_e16887_d_n5;
        locals.var_t5_dn6 = assign21920_e16887_d_n6;
        locals.var_t5_dn7 = assign21920_e16887_d_n7;
        locals.var_t5_dn8 = assign21920_e16887_d_n8;
        locals.var_t5_dn9 = assign21920_e16887_d_n9;
        locals.var_t5_dn10 = assign21920_e16887_d_n10;
        locals.var_t5_dn11 = assign21920_e16887_d_n11;
        locals.var_t5_dn14 = assign21920_e16887_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign21930_e16901, assign21930_e16901_d_n0, assign21930_e16901_d_n2, assign21930_e16901_d_n4, assign21930_e16901_d_n5, assign21930_e16901_d_n6, assign21930_e16901_d_n7, assign21930_e16901_d_n8, assign21930_e16901_d_n9, assign21930_e16901_d_n10, assign21930_e16901_d_n11, assign21930_e16901_d_n14,) = {
    if (locals.var_guard426 != 0.0) {
        let assign21930_e16891: f64 = (1.0 + locals.var_t4);
        let assign21930_e16895: f64 = (1.0 + locals.var_t4);
        let assign21930_e16897: f64 = (assign21930_e16895 + locals.var_t5);
        let assign21930_e16898: f64 = (locals.var_t5 * assign21930_e16897);
        let assign21930_e16899: f64 = (assign21930_e16891 + assign21930_e16898);
        (assign21930_e16899, (locals.var_t4_dn0 + ((locals.var_t5_dn0 * assign21930_e16897) + (locals.var_t5 * (locals.var_t4_dn0 + locals.var_t5_dn0)))), (locals.var_t4_dn2 + ((locals.var_t5_dn2 * assign21930_e16897) + (locals.var_t5 * (locals.var_t4_dn2 + locals.var_t5_dn2)))), (locals.var_t4_dn4 + ((locals.var_t5_dn4 * assign21930_e16897) + (locals.var_t5 * (locals.var_t4_dn4 + locals.var_t5_dn4)))), (locals.var_t4_dn5 + ((locals.var_t5_dn5 * assign21930_e16897) + (locals.var_t5 * (locals.var_t4_dn5 + locals.var_t5_dn5)))), (locals.var_t4_dn6 + ((locals.var_t5_dn6 * assign21930_e16897) + (locals.var_t5 * (locals.var_t4_dn6 + locals.var_t5_dn6)))), (locals.var_t4_dn7 + ((locals.var_t5_dn7 * assign21930_e16897) + (locals.var_t5 * (locals.var_t4_dn7 + locals.var_t5_dn7)))), (locals.var_t4_dn8 + ((locals.var_t5_dn8 * assign21930_e16897) + (locals.var_t5 * (locals.var_t4_dn8 + locals.var_t5_dn8)))), (locals.var_t4_dn9 + ((locals.var_t5_dn9 * assign21930_e16897) + (locals.var_t5 * (locals.var_t4_dn9 + locals.var_t5_dn9)))), (locals.var_t4_dn10 + ((locals.var_t5_dn10 * assign21930_e16897) + (locals.var_t5 * (locals.var_t4_dn10 + locals.var_t5_dn10)))), (locals.var_t4_dn11 + ((locals.var_t5_dn11 * assign21930_e16897) + (locals.var_t5 * (locals.var_t4_dn11 + locals.var_t5_dn11)))), (locals.var_t4_dn14 + ((locals.var_t5_dn14 * assign21930_e16897) + (locals.var_t5 * (locals.var_t4_dn14 + locals.var_t5_dn14)))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign21930_e16901;
        locals.var_t7_dn0 = assign21930_e16901_d_n0;
        locals.var_t7_dn2 = assign21930_e16901_d_n2;
        locals.var_t7_dn4 = assign21930_e16901_d_n4;
        locals.var_t7_dn5 = assign21930_e16901_d_n5;
        locals.var_t7_dn6 = assign21930_e16901_d_n6;
        locals.var_t7_dn7 = assign21930_e16901_d_n7;
        locals.var_t7_dn8 = assign21930_e16901_d_n8;
        locals.var_t7_dn9 = assign21930_e16901_d_n9;
        locals.var_t7_dn10 = assign21930_e16901_d_n10;
        locals.var_t7_dn11 = assign21930_e16901_d_n11;
        locals.var_t7_dn14 = assign21930_e16901_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign21940_e16923, assign21940_e16923_d_n0, assign21940_e16923_d_n2, assign21940_e16923_d_n4, assign21940_e16923_d_n5, assign21940_e16923_d_n6, assign21940_e16923_d_n7, assign21940_e16923_d_n8, assign21940_e16923_d_n9, assign21940_e16923_d_n10, assign21940_e16923_d_n11, assign21940_e16923_d_n14,) = {
    if (locals.var_guard426 != 0.0) {
        let assign21940_e16906: f64 = (2.0 * locals.var_t4);
        let assign21940_e16907: f64 = (1.0 + assign21940_e16906);
        let assign21940_e16910: f64 = (3.0 * locals.var_t5);
        let assign21940_e16911: f64 = (assign21940_e16907 + assign21940_e16910);
        let assign21940_e16914: f64 = (4.0 * locals.var_t4);
        let assign21940_e16916: f64 = (assign21940_e16914 * locals.var_t5);
        let assign21940_e16917: f64 = (assign21940_e16911 + assign21940_e16916);
        let assign21940_e16920: f64 = (locals.var_t7 * locals.var_t7);
        let assign21940_e16921: f64 = (assign21940_e16917 / assign21940_e16920);
        (assign21940_e16921, ((((((2.0 * locals.var_t4_dn0) + (3.0 * locals.var_t5_dn0)) + (((4.0 * locals.var_t4_dn0) * locals.var_t5) + (assign21940_e16914 * locals.var_t5_dn0))) * assign21940_e16920) - (assign21940_e16917 * ((locals.var_t7_dn0 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn0)))) / (assign21940_e16920 * assign21940_e16920)), ((((((2.0 * locals.var_t4_dn2) + (3.0 * locals.var_t5_dn2)) + (((4.0 * locals.var_t4_dn2) * locals.var_t5) + (assign21940_e16914 * locals.var_t5_dn2))) * assign21940_e16920) - (assign21940_e16917 * ((locals.var_t7_dn2 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn2)))) / (assign21940_e16920 * assign21940_e16920)), ((((((2.0 * locals.var_t4_dn4) + (3.0 * locals.var_t5_dn4)) + (((4.0 * locals.var_t4_dn4) * locals.var_t5) + (assign21940_e16914 * locals.var_t5_dn4))) * assign21940_e16920) - (assign21940_e16917 * ((locals.var_t7_dn4 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn4)))) / (assign21940_e16920 * assign21940_e16920)), ((((((2.0 * locals.var_t4_dn5) + (3.0 * locals.var_t5_dn5)) + (((4.0 * locals.var_t4_dn5) * locals.var_t5) + (assign21940_e16914 * locals.var_t5_dn5))) * assign21940_e16920) - (assign21940_e16917 * ((locals.var_t7_dn5 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn5)))) / (assign21940_e16920 * assign21940_e16920)), ((((((2.0 * locals.var_t4_dn6) + (3.0 * locals.var_t5_dn6)) + (((4.0 * locals.var_t4_dn6) * locals.var_t5) + (assign21940_e16914 * locals.var_t5_dn6))) * assign21940_e16920) - (assign21940_e16917 * ((locals.var_t7_dn6 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn6)))) / (assign21940_e16920 * assign21940_e16920)), ((((((2.0 * locals.var_t4_dn7) + (3.0 * locals.var_t5_dn7)) + (((4.0 * locals.var_t4_dn7) * locals.var_t5) + (assign21940_e16914 * locals.var_t5_dn7))) * assign21940_e16920) - (assign21940_e16917 * ((locals.var_t7_dn7 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn7)))) / (assign21940_e16920 * assign21940_e16920)), ((((((2.0 * locals.var_t4_dn8) + (3.0 * locals.var_t5_dn8)) + (((4.0 * locals.var_t4_dn8) * locals.var_t5) + (assign21940_e16914 * locals.var_t5_dn8))) * assign21940_e16920) - (assign21940_e16917 * ((locals.var_t7_dn8 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn8)))) / (assign21940_e16920 * assign21940_e16920)), ((((((2.0 * locals.var_t4_dn9) + (3.0 * locals.var_t5_dn9)) + (((4.0 * locals.var_t4_dn9) * locals.var_t5) + (assign21940_e16914 * locals.var_t5_dn9))) * assign21940_e16920) - (assign21940_e16917 * ((locals.var_t7_dn9 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn9)))) / (assign21940_e16920 * assign21940_e16920)), ((((((2.0 * locals.var_t4_dn10) + (3.0 * locals.var_t5_dn10)) + (((4.0 * locals.var_t4_dn10) * locals.var_t5) + (assign21940_e16914 * locals.var_t5_dn10))) * assign21940_e16920) - (assign21940_e16917 * ((locals.var_t7_dn10 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn10)))) / (assign21940_e16920 * assign21940_e16920)), ((((((2.0 * locals.var_t4_dn11) + (3.0 * locals.var_t5_dn11)) + (((4.0 * locals.var_t4_dn11) * locals.var_t5) + (assign21940_e16914 * locals.var_t5_dn11))) * assign21940_e16920) - (assign21940_e16917 * ((locals.var_t7_dn11 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn11)))) / (assign21940_e16920 * assign21940_e16920)), ((((((2.0 * locals.var_t4_dn14) + (3.0 * locals.var_t5_dn14)) + (((4.0 * locals.var_t4_dn14) * locals.var_t5) + (assign21940_e16914 * locals.var_t5_dn14))) * assign21940_e16920) - (assign21940_e16917 * ((locals.var_t7_dn14 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn14)))) / (assign21940_e16920 * assign21940_e16920)),)
    } else {
        (locals.var_vbscldvbs, locals.var_vbscldvbs_dn0, locals.var_vbscldvbs_dn2, locals.var_vbscldvbs_dn4, locals.var_vbscldvbs_dn5, locals.var_vbscldvbs_dn6, locals.var_vbscldvbs_dn7, locals.var_vbscldvbs_dn8, locals.var_vbscldvbs_dn9, locals.var_vbscldvbs_dn10, locals.var_vbscldvbs_dn11, locals.var_vbscldvbs_dn14,)
    }
};
        locals.var_vbscldvbs = assign21940_e16923;
        locals.var_vbscldvbs_dn0 = assign21940_e16923_d_n0;
        locals.var_vbscldvbs_dn2 = assign21940_e16923_d_n2;
        locals.var_vbscldvbs_dn4 = assign21940_e16923_d_n4;
        locals.var_vbscldvbs_dn5 = assign21940_e16923_d_n5;
        locals.var_vbscldvbs_dn6 = assign21940_e16923_d_n6;
        locals.var_vbscldvbs_dn7 = assign21940_e16923_d_n7;
        locals.var_vbscldvbs_dn8 = assign21940_e16923_d_n8;
        locals.var_vbscldvbs_dn9 = assign21940_e16923_d_n9;
        locals.var_vbscldvbs_dn10 = assign21940_e16923_d_n10;
        locals.var_vbscldvbs_dn11 = assign21940_e16923_d_n11;
        locals.var_vbscldvbs_dn14 = assign21940_e16923_d_n14;
        locals.var_vbscldvbs_rv = 0.0;

        let (assign21950_e16928, assign21950_e16928_d_n0, assign21950_e16928_d_n2, assign21950_e16928_d_n4, assign21950_e16928_d_n5, assign21950_e16928_d_n6, assign21950_e16928_d_n7, assign21950_e16928_d_n8, assign21950_e16928_d_n9, assign21950_e16928_d_n10, assign21950_e16928_d_n11, assign21950_e16928_d_n14,) = {
    if (locals.var_guard426 == 0.0) {
        (locals.var_vbs, 0.0, 0.0, 0.0, 0.0, locals.var_vbs_dn6, 0.0, locals.var_vbs_dn8, locals.var_vbs_dn9, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbscl, locals.var_vbscl_dn0, locals.var_vbscl_dn2, locals.var_vbscl_dn4, locals.var_vbscl_dn5, locals.var_vbscl_dn6, locals.var_vbscl_dn7, locals.var_vbscl_dn8, locals.var_vbscl_dn9, locals.var_vbscl_dn10, locals.var_vbscl_dn11, locals.var_vbscl_dn14,)
    }
};
        locals.var_vbscl = assign21950_e16928;
        locals.var_vbscl_dn0 = assign21950_e16928_d_n0;
        locals.var_vbscl_dn2 = assign21950_e16928_d_n2;
        locals.var_vbscl_dn4 = assign21950_e16928_d_n4;
        locals.var_vbscl_dn5 = assign21950_e16928_d_n5;
        locals.var_vbscl_dn6 = assign21950_e16928_d_n6;
        locals.var_vbscl_dn7 = assign21950_e16928_d_n7;
        locals.var_vbscl_dn8 = assign21950_e16928_d_n8;
        locals.var_vbscl_dn9 = assign21950_e16928_d_n9;
        locals.var_vbscl_dn10 = assign21950_e16928_d_n10;
        locals.var_vbscl_dn11 = assign21950_e16928_d_n11;
        locals.var_vbscl_dn14 = assign21950_e16928_d_n14;
        locals.var_vbscl_rv = 0.0;

        let (assign21960_e16933, assign21960_e16933_d_n0, assign21960_e16933_d_n2, assign21960_e16933_d_n4, assign21960_e16933_d_n5, assign21960_e16933_d_n6, assign21960_e16933_d_n7, assign21960_e16933_d_n8, assign21960_e16933_d_n9, assign21960_e16933_d_n10, assign21960_e16933_d_n11, assign21960_e16933_d_n14,) = {
    if (locals.var_guard426 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbscldvbs, locals.var_vbscldvbs_dn0, locals.var_vbscldvbs_dn2, locals.var_vbscldvbs_dn4, locals.var_vbscldvbs_dn5, locals.var_vbscldvbs_dn6, locals.var_vbscldvbs_dn7, locals.var_vbscldvbs_dn8, locals.var_vbscldvbs_dn9, locals.var_vbscldvbs_dn10, locals.var_vbscldvbs_dn11, locals.var_vbscldvbs_dn14,)
    }
};
        locals.var_vbscldvbs = assign21960_e16933;
        locals.var_vbscldvbs_dn0 = assign21960_e16933_d_n0;
        locals.var_vbscldvbs_dn2 = assign21960_e16933_d_n2;
        locals.var_vbscldvbs_dn4 = assign21960_e16933_d_n4;
        locals.var_vbscldvbs_dn5 = assign21960_e16933_d_n5;
        locals.var_vbscldvbs_dn6 = assign21960_e16933_d_n6;
        locals.var_vbscldvbs_dn7 = assign21960_e16933_d_n7;
        locals.var_vbscldvbs_dn8 = assign21960_e16933_d_n8;
        locals.var_vbscldvbs_dn9 = assign21960_e16933_d_n9;
        locals.var_vbscldvbs_dn10 = assign21960_e16933_d_n10;
        locals.var_vbscldvbs_dn11 = assign21960_e16933_d_n11;
        locals.var_vbscldvbs_dn14 = assign21960_e16933_d_n14;
        locals.var_vbscldvbs_rv = 0.0;

        let assign21970_e16936: f64 = (locals.var_vbscldvbs * locals.var_vds);
        let assign21970_e16938: f64 = (assign21970_e16936 / 2.0);
        locals.var_t1 = assign21970_e16938;
        locals.var_t1_dn0 = (((locals.var_vbscldvbs_dn0 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn0)) / 2.0);
        locals.var_t1_dn2 = (((locals.var_vbscldvbs_dn2 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn2)) / 2.0);
        locals.var_t1_dn4 = (((locals.var_vbscldvbs_dn4 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn4)) / 2.0);
        locals.var_t1_dn5 = (((locals.var_vbscldvbs_dn5 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn5)) / 2.0);
        locals.var_t1_dn6 = (((locals.var_vbscldvbs_dn6 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn6)) / 2.0);
        locals.var_t1_dn7 = (((locals.var_vbscldvbs_dn7 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn7)) / 2.0);
        locals.var_t1_dn8 = (((locals.var_vbscldvbs_dn8 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn8)) / 2.0);
        locals.var_t1_dn9 = (((locals.var_vbscldvbs_dn9 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn9)) / 2.0);
        locals.var_t1_dn10 = (((locals.var_vbscldvbs_dn10 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn10)) / 2.0);
        locals.var_t1_dn11 = (((locals.var_vbscldvbs_dn11 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn11)) / 2.0);
        locals.var_t1_dn14 = (((locals.var_vbscldvbs_dn14 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn14)) / 2.0);
        locals.var_t1_rv = 0.0;

        let assign21980_e16941: f64 = (2.0 * locals.var_t1);
        let assign21980_e16943: f64 = (assign21980_e16941 / p.p262);
        locals.var_tmf1 = assign21980_e16943;
        locals.var_tmf1_dn0 = ((2.0 * locals.var_t1_dn0) / p.p262);
        locals.var_tmf1_dn2 = ((2.0 * locals.var_t1_dn2) / p.p262);
        locals.var_tmf1_dn4 = ((2.0 * locals.var_t1_dn4) / p.p262);
        locals.var_tmf1_dn5 = ((2.0 * locals.var_t1_dn5) / p.p262);
        locals.var_tmf1_dn6 = ((2.0 * locals.var_t1_dn6) / p.p262);
        locals.var_tmf1_dn7 = ((2.0 * locals.var_t1_dn7) / p.p262);
        locals.var_tmf1_dn8 = ((2.0 * locals.var_t1_dn8) / p.p262);
        locals.var_tmf1_dn9 = ((2.0 * locals.var_t1_dn9) / p.p262);
        locals.var_tmf1_dn10 = ((2.0 * locals.var_t1_dn10) / p.p262);
        locals.var_tmf1_dn11 = ((2.0 * locals.var_t1_dn11) / p.p262);
        locals.var_tmf1_dn14 = ((2.0 * locals.var_t1_dn14) / p.p262);
        locals.var_tmf1_rv = 0.0;

        let assign21990_e16948: f64 = (1.0 / 2.0);
        let assign21990_e16952: f64 = (1.0 / 6.0);
        let assign21990_e16956: f64 = (1.0 / 24.0);
        let assign21990_e16960: f64 = (1.0 / 120.0);
        let assign21990_e16964: f64 = (1.0 / 720.0);
        let assign21990_e16968: f64 = (1.0 / 5040.0);
        let assign21990_e16969: f64 = (locals.var_tmf1 * assign21990_e16968);
        let assign21990_e16970: f64 = (assign21990_e16964 + assign21990_e16969);
        let assign21990_e16971: f64 = (locals.var_tmf1 * assign21990_e16970);
        let assign21990_e16972: f64 = (assign21990_e16960 + assign21990_e16971);
        let assign21990_e16973: f64 = (locals.var_tmf1 * assign21990_e16972);
        let assign21990_e16974: f64 = (assign21990_e16956 + assign21990_e16973);
        let assign21990_e16975: f64 = (locals.var_tmf1 * assign21990_e16974);
        let assign21990_e16976: f64 = (assign21990_e16952 + assign21990_e16975);
        let assign21990_e16977: f64 = (locals.var_tmf1 * assign21990_e16976);
        let assign21990_e16978: f64 = (assign21990_e16948 + assign21990_e16977);
        let assign21990_e16979: f64 = (locals.var_tmf1 * assign21990_e16978);
        let assign21990_e16980: f64 = (1.0 + assign21990_e16979);
        locals.var_tmf2 = assign21990_e16980;
        locals.var_tmf2_dn0 = ((locals.var_tmf1_dn0 * assign21990_e16978) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign21990_e16976) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign21990_e16974) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign21990_e16972) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign21990_e16970) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign21990_e16968)))))))))));
        locals.var_tmf2_dn2 = ((locals.var_tmf1_dn2 * assign21990_e16978) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign21990_e16976) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign21990_e16974) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign21990_e16972) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign21990_e16970) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign21990_e16968)))))))))));
        locals.var_tmf2_dn4 = ((locals.var_tmf1_dn4 * assign21990_e16978) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign21990_e16976) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign21990_e16974) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign21990_e16972) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign21990_e16970) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign21990_e16968)))))))))));
        locals.var_tmf2_dn5 = ((locals.var_tmf1_dn5 * assign21990_e16978) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign21990_e16976) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign21990_e16974) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign21990_e16972) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign21990_e16970) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign21990_e16968)))))))))));
        locals.var_tmf2_dn6 = ((locals.var_tmf1_dn6 * assign21990_e16978) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign21990_e16976) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign21990_e16974) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign21990_e16972) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign21990_e16970) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign21990_e16968)))))))))));
        locals.var_tmf2_dn7 = ((locals.var_tmf1_dn7 * assign21990_e16978) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign21990_e16976) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign21990_e16974) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign21990_e16972) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign21990_e16970) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign21990_e16968)))))))))));
        locals.var_tmf2_dn8 = ((locals.var_tmf1_dn8 * assign21990_e16978) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign21990_e16976) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign21990_e16974) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign21990_e16972) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign21990_e16970) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign21990_e16968)))))))))));
        locals.var_tmf2_dn9 = ((locals.var_tmf1_dn9 * assign21990_e16978) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign21990_e16976) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign21990_e16974) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign21990_e16972) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign21990_e16970) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign21990_e16968)))))))))));
        locals.var_tmf2_dn10 = ((locals.var_tmf1_dn10 * assign21990_e16978) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign21990_e16976) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign21990_e16974) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign21990_e16972) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign21990_e16970) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign21990_e16968)))))))))));
        locals.var_tmf2_dn11 = ((locals.var_tmf1_dn11 * assign21990_e16978) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign21990_e16976) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign21990_e16974) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign21990_e16972) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign21990_e16970) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign21990_e16968)))))))))));
        locals.var_tmf2_dn14 = ((locals.var_tmf1_dn14 * assign21990_e16978) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign21990_e16976) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign21990_e16974) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign21990_e16972) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign21990_e16970) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign21990_e16968)))))))))));
        locals.var_tmf2_rv = 0.0;

        let assign22000_e16983: f64 = (1.0 / 2.0);
        let assign22000_e16987: f64 = (1.0 / 3.0);
        let assign22000_e16991: f64 = (1.0 / 8.0);
        let assign22000_e16995: f64 = (1.0 / 30.0);
        let assign22000_e16999: f64 = (1.0 / 144.0);
        let assign22000_e17003: f64 = (1.0 / 840.0);
        let assign22000_e17004: f64 = (locals.var_tmf1 * assign22000_e17003);
        let assign22000_e17005: f64 = (assign22000_e16999 + assign22000_e17004);
        let assign22000_e17006: f64 = (locals.var_tmf1 * assign22000_e17005);
        let assign22000_e17007: f64 = (assign22000_e16995 + assign22000_e17006);
        let assign22000_e17008: f64 = (locals.var_tmf1 * assign22000_e17007);
        let assign22000_e17009: f64 = (assign22000_e16991 + assign22000_e17008);
        let assign22000_e17010: f64 = (locals.var_tmf1 * assign22000_e17009);
        let assign22000_e17011: f64 = (assign22000_e16987 + assign22000_e17010);
        let assign22000_e17012: f64 = (locals.var_tmf1 * assign22000_e17011);
        let assign22000_e17013: f64 = (assign22000_e16983 + assign22000_e17012);
        locals.var_tmf3 = assign22000_e17013;
        locals.var_tmf3_dn0 = ((locals.var_tmf1_dn0 * assign22000_e17011) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign22000_e17009) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign22000_e17007) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign22000_e17005) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign22000_e17003)))))))));
        locals.var_tmf3_dn2 = ((locals.var_tmf1_dn2 * assign22000_e17011) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign22000_e17009) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign22000_e17007) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign22000_e17005) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign22000_e17003)))))))));
        locals.var_tmf3_dn4 = ((locals.var_tmf1_dn4 * assign22000_e17011) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign22000_e17009) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign22000_e17007) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign22000_e17005) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign22000_e17003)))))))));
        locals.var_tmf3_dn5 = ((locals.var_tmf1_dn5 * assign22000_e17011) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign22000_e17009) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign22000_e17007) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign22000_e17005) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign22000_e17003)))))))));
        locals.var_tmf3_dn6 = ((locals.var_tmf1_dn6 * assign22000_e17011) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign22000_e17009) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign22000_e17007) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign22000_e17005) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign22000_e17003)))))))));
        locals.var_tmf3_dn7 = ((locals.var_tmf1_dn7 * assign22000_e17011) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign22000_e17009) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign22000_e17007) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign22000_e17005) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign22000_e17003)))))))));
        locals.var_tmf3_dn8 = ((locals.var_tmf1_dn8 * assign22000_e17011) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign22000_e17009) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign22000_e17007) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign22000_e17005) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign22000_e17003)))))))));
        locals.var_tmf3_dn9 = ((locals.var_tmf1_dn9 * assign22000_e17011) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign22000_e17009) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign22000_e17007) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign22000_e17005) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign22000_e17003)))))))));
        locals.var_tmf3_dn10 = ((locals.var_tmf1_dn10 * assign22000_e17011) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign22000_e17009) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign22000_e17007) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign22000_e17005) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign22000_e17003)))))))));
        locals.var_tmf3_dn11 = ((locals.var_tmf1_dn11 * assign22000_e17011) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign22000_e17009) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign22000_e17007) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign22000_e17005) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign22000_e17003)))))))));
        locals.var_tmf3_dn14 = ((locals.var_tmf1_dn14 * assign22000_e17011) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign22000_e17009) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign22000_e17007) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign22000_e17005) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign22000_e17003)))))))));
        locals.var_tmf3_rv = 0.0;

        let assign22010_e17016: f64 = (p.p262 / locals.var_tmf2);
        locals.var_vzadd = assign22010_e17016;
        locals.var_vzadd_dn0 = (-((p.p262 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn2 = (-((p.p262 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn4 = (-((p.p262 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn5 = (-((p.p262 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn6 = (-((p.p262 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn7 = (-((p.p262 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn8 = (-((p.p262 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn9 = (-((p.p262 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn10 = (-((p.p262 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn11 = (-((p.p262 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn14 = (-((p.p262 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_rv = 0.0;

        let assign22020_e17018: f64 = (-2.0);
        let assign22020_e17020: f64 = (assign22020_e17018 * locals.var_tmf3);
        let assign22020_e17023: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign22020_e17024: f64 = (assign22020_e17020 / assign22020_e17023);
        locals.var_t2 = assign22020_e17024;
        locals.var_t2_dn0 = ((((assign22020_e17018 * locals.var_tmf3_dn0) * assign22020_e17023) - (assign22020_e17020 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign22020_e17023 * assign22020_e17023));
        locals.var_t2_dn2 = ((((assign22020_e17018 * locals.var_tmf3_dn2) * assign22020_e17023) - (assign22020_e17020 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign22020_e17023 * assign22020_e17023));
        locals.var_t2_dn4 = ((((assign22020_e17018 * locals.var_tmf3_dn4) * assign22020_e17023) - (assign22020_e17020 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign22020_e17023 * assign22020_e17023));
        locals.var_t2_dn5 = ((((assign22020_e17018 * locals.var_tmf3_dn5) * assign22020_e17023) - (assign22020_e17020 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign22020_e17023 * assign22020_e17023));
        locals.var_t2_dn6 = ((((assign22020_e17018 * locals.var_tmf3_dn6) * assign22020_e17023) - (assign22020_e17020 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign22020_e17023 * assign22020_e17023));
        locals.var_t2_dn7 = ((((assign22020_e17018 * locals.var_tmf3_dn7) * assign22020_e17023) - (assign22020_e17020 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign22020_e17023 * assign22020_e17023));
        locals.var_t2_dn8 = ((((assign22020_e17018 * locals.var_tmf3_dn8) * assign22020_e17023) - (assign22020_e17020 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign22020_e17023 * assign22020_e17023));
        locals.var_t2_dn9 = ((((assign22020_e17018 * locals.var_tmf3_dn9) * assign22020_e17023) - (assign22020_e17020 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign22020_e17023 * assign22020_e17023));
        locals.var_t2_dn10 = ((((assign22020_e17018 * locals.var_tmf3_dn10) * assign22020_e17023) - (assign22020_e17020 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign22020_e17023 * assign22020_e17023));
        locals.var_t2_dn11 = ((((assign22020_e17018 * locals.var_tmf3_dn11) * assign22020_e17023) - (assign22020_e17020 * ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)))) / (assign22020_e17023 * assign22020_e17023));
        locals.var_t2_dn14 = ((((assign22020_e17018 * locals.var_tmf3_dn14) * assign22020_e17023) - (assign22020_e17020 * ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)))) / (assign22020_e17023 * assign22020_e17023));
        locals.var_t2_rv = 0.0;

        let assign22030_e17027: f64 = if locals.var_vzadd < 1e-12 { 1.0 } else { 0.0 };
        locals.var_guard427 = assign22030_e17027;
        locals.var_guard427_rv = 0.0;

        let (assign22040_e17031, assign22040_e17031_d_n0, assign22040_e17031_d_n2, assign22040_e17031_d_n4, assign22040_e17031_d_n5, assign22040_e17031_d_n6, assign22040_e17031_d_n7, assign22040_e17031_d_n8, assign22040_e17031_d_n9, assign22040_e17031_d_n10, assign22040_e17031_d_n11, assign22040_e17031_d_n14,) = {
    if (locals.var_guard427 != 0.0) {
        (1e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn11, locals.var_vzadd_dn14,)
    }
};
        locals.var_vzadd = assign22040_e17031;
        locals.var_vzadd_dn0 = assign22040_e17031_d_n0;
        locals.var_vzadd_dn2 = assign22040_e17031_d_n2;
        locals.var_vzadd_dn4 = assign22040_e17031_d_n4;
        locals.var_vzadd_dn5 = assign22040_e17031_d_n5;
        locals.var_vzadd_dn6 = assign22040_e17031_d_n6;
        locals.var_vzadd_dn7 = assign22040_e17031_d_n7;
        locals.var_vzadd_dn8 = assign22040_e17031_d_n8;
        locals.var_vzadd_dn9 = assign22040_e17031_d_n9;
        locals.var_vzadd_dn10 = assign22040_e17031_d_n10;
        locals.var_vzadd_dn11 = assign22040_e17031_d_n11;
        locals.var_vzadd_dn14 = assign22040_e17031_d_n14;
        locals.var_vzadd_rv = 0.0;

        let assign22050_e17034: f64 = (locals.var_vbscl + locals.var_vzadd);
        locals.var_vbsz = assign22050_e17034;
        locals.var_vbsz_dn0 = (locals.var_vbscl_dn0 + locals.var_vzadd_dn0);
        locals.var_vbsz_dn2 = (locals.var_vbscl_dn2 + locals.var_vzadd_dn2);
        locals.var_vbsz_dn4 = (locals.var_vbscl_dn4 + locals.var_vzadd_dn4);
        locals.var_vbsz_dn5 = (locals.var_vbscl_dn5 + locals.var_vzadd_dn5);
        locals.var_vbsz_dn6 = (locals.var_vbscl_dn6 + locals.var_vzadd_dn6);
        locals.var_vbsz_dn7 = (locals.var_vbscl_dn7 + locals.var_vzadd_dn7);
        locals.var_vbsz_dn8 = (locals.var_vbscl_dn8 + locals.var_vzadd_dn8);
        locals.var_vbsz_dn9 = (locals.var_vbscl_dn9 + locals.var_vzadd_dn9);
        locals.var_vbsz_dn10 = (locals.var_vbscl_dn10 + locals.var_vzadd_dn10);
        locals.var_vbsz_dn11 = (locals.var_vbscl_dn11 + locals.var_vzadd_dn11);
        locals.var_vbsz_dn14 = (locals.var_vbscl_dn14 + locals.var_vzadd_dn14);
        locals.var_vbsz_rv = 0.0;

        let assign22060_e17038: f64 = (2.0 * locals.var_vzadd);
        let assign22060_e17039: f64 = (locals.var_vds + assign22060_e17038);
        locals.var_vdsz = assign22060_e17039;
        locals.var_vdsz_dn0 = (locals.var_vds_dn0 + (2.0 * locals.var_vzadd_dn0));
        locals.var_vdsz_dn2 = (locals.var_vds_dn2 + (2.0 * locals.var_vzadd_dn2));
        locals.var_vdsz_dn4 = (locals.var_vds_dn4 + (2.0 * locals.var_vzadd_dn4));
        locals.var_vdsz_dn5 = (locals.var_vds_dn5 + (2.0 * locals.var_vzadd_dn5));
        locals.var_vdsz_dn6 = (locals.var_vds_dn6 + (2.0 * locals.var_vzadd_dn6));
        locals.var_vdsz_dn7 = (locals.var_vds_dn7 + (2.0 * locals.var_vzadd_dn7));
        locals.var_vdsz_dn8 = (locals.var_vds_dn8 + (2.0 * locals.var_vzadd_dn8));
        locals.var_vdsz_dn9 = (locals.var_vds_dn9 + (2.0 * locals.var_vzadd_dn9));
        locals.var_vdsz_dn10 = (locals.var_vds_dn10 + (2.0 * locals.var_vzadd_dn10));
        locals.var_vdsz_dn11 = (locals.var_vds_dn11 + (2.0 * locals.var_vzadd_dn11));
        locals.var_vdsz_dn14 = (locals.var_vds_dn14 + (2.0 * locals.var_vzadd_dn14));
        locals.var_vdsz_rv = 0.0;

        let assign22070_e17042: f64 = (locals.var_vgs + locals.var_vzadd);
        locals.var_vgsz = assign22070_e17042;
        locals.var_vgsz_dn0 = locals.var_vzadd_dn0;
        locals.var_vgsz_dn2 = locals.var_vzadd_dn2;
        locals.var_vgsz_dn4 = locals.var_vzadd_dn4;
        locals.var_vgsz_dn5 = locals.var_vzadd_dn5;
        locals.var_vgsz_dn6 = (locals.var_vgs_dn6 + locals.var_vzadd_dn6);
        locals.var_vgsz_dn7 = (locals.var_vgs_dn7 + locals.var_vzadd_dn7);
        locals.var_vgsz_dn8 = (locals.var_vgs_dn8 + locals.var_vzadd_dn8);
        locals.var_vgsz_dn9 = locals.var_vzadd_dn9;
        locals.var_vgsz_dn10 = locals.var_vzadd_dn10;
        locals.var_vgsz_dn11 = locals.var_vzadd_dn11;
        locals.var_vgsz_dn14 = locals.var_vzadd_dn14;
        locals.var_vgsz_rv = 0.0;

        let assign22080_e17045: f64 = (locals.var_qnsub_esi * locals.var_cox0_inv);
        let assign22080_e17047: f64 = (assign22080_e17045 * locals.var_cox0_inv);
        locals.var_t1 = assign22080_e17047;
        locals.var_t1_dn0 = ((locals.var_qnsub_esi_dn0 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_dn2 = ((locals.var_qnsub_esi_dn2 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_dn4 = ((locals.var_qnsub_esi_dn4 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_dn5 = ((locals.var_qnsub_esi_dn5 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_dn6 = ((locals.var_qnsub_esi_dn6 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_dn7 = ((locals.var_qnsub_esi_dn7 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_dn8 = ((locals.var_qnsub_esi_dn8 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_dn9 = ((locals.var_qnsub_esi_dn9 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_dn10 = ((locals.var_qnsub_esi_dn10 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_dn11 = ((locals.var_qnsub_esi_dn11 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_dn14 = ((locals.var_qnsub_esi_dn14 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_rv = 0.0;

        let assign22090_e17050: f64 = (locals.var_vgs - locals.var_vfb);
        locals.var_t2 = assign22090_e17050;
        locals.var_t2_dn0 = 0.0;
        locals.var_t2_dn2 = 0.0;
        locals.var_t2_dn4 = 0.0;
        locals.var_t2_dn5 = 0.0;
        locals.var_t2_dn6 = locals.var_vgs_dn6;
        locals.var_t2_dn7 = locals.var_vgs_dn7;
        locals.var_t2_dn8 = locals.var_vgs_dn8;
        locals.var_t2_dn9 = 0.0;
        locals.var_t2_dn10 = 0.0;
        locals.var_t2_dn11 = 0.0;
        locals.var_t2_dn14 = 0.0;
        locals.var_t2_rv = 0.0;

        let assign22100_e17054: f64 = (2.0 / locals.var_t1);
        let assign22100_e17058: f64 = (1.0 / locals.var_betatnom);
        let assign22100_e17059: f64 = (locals.var_t2 - assign22100_e17058);
        let assign22100_e17061: f64 = (assign22100_e17059 - locals.var_vbscl);
        let assign22100_e17062: f64 = (assign22100_e17054 * assign22100_e17061);
        let assign22100_e17063: f64 = (1.0 + assign22100_e17062);
        locals.var_t3 = assign22100_e17063;
        locals.var_t3_dn0 = (((-((2.0 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))) * assign22100_e17061) + (assign22100_e17054 * (locals.var_t2_dn0 - locals.var_vbscl_dn0)));
        locals.var_t3_dn2 = (((-((2.0 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))) * assign22100_e17061) + (assign22100_e17054 * (locals.var_t2_dn2 - locals.var_vbscl_dn2)));
        locals.var_t3_dn4 = (((-((2.0 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))) * assign22100_e17061) + (assign22100_e17054 * (locals.var_t2_dn4 - locals.var_vbscl_dn4)));
        locals.var_t3_dn5 = (((-((2.0 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))) * assign22100_e17061) + (assign22100_e17054 * (locals.var_t2_dn5 - locals.var_vbscl_dn5)));
        locals.var_t3_dn6 = (((-((2.0 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))) * assign22100_e17061) + (assign22100_e17054 * (locals.var_t2_dn6 - locals.var_vbscl_dn6)));
        locals.var_t3_dn7 = (((-((2.0 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))) * assign22100_e17061) + (assign22100_e17054 * (locals.var_t2_dn7 - locals.var_vbscl_dn7)));
        locals.var_t3_dn8 = (((-((2.0 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))) * assign22100_e17061) + (assign22100_e17054 * (locals.var_t2_dn8 - locals.var_vbscl_dn8)));
        locals.var_t3_dn9 = (((-((2.0 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))) * assign22100_e17061) + (assign22100_e17054 * (locals.var_t2_dn9 - locals.var_vbscl_dn9)));
        locals.var_t3_dn10 = (((-((2.0 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))) * assign22100_e17061) + (assign22100_e17054 * (locals.var_t2_dn10 - locals.var_vbscl_dn10)));
        locals.var_t3_dn11 = (((-((2.0 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))) * assign22100_e17061) + (assign22100_e17054 * (locals.var_t2_dn11 - locals.var_vbscl_dn11)));
        locals.var_t3_dn14 = (((-((2.0 * locals.var_t1_dn14) / (locals.var_t1 * locals.var_t1))) * assign22100_e17061) + (assign22100_e17054 * (locals.var_t2_dn14 - locals.var_vbscl_dn14)));
        locals.var_t3_rv = 0.0;

        let assign22110_e17066: f64 = (locals.var_t3 * locals.var_t3);
        let assign22110_e17069: f64 = (4.0 * 0.001);
        let assign22110_e17071: f64 = (assign22110_e17069 * 0.001);
        let assign22110_e17072: f64 = (assign22110_e17066 + assign22110_e17071);
        let assign22110_e17073: f64 = (assign22110_e17072).sqrt();
        locals.var_tmf2 = assign22110_e17073;
        locals.var_tmf2_dn0 = (((locals.var_t3_dn0 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn0)) / (2.0 * assign22110_e17073));
        locals.var_tmf2_dn2 = (((locals.var_t3_dn2 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn2)) / (2.0 * assign22110_e17073));
        locals.var_tmf2_dn4 = (((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4)) / (2.0 * assign22110_e17073));
        locals.var_tmf2_dn5 = (((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5)) / (2.0 * assign22110_e17073));
        locals.var_tmf2_dn6 = (((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6)) / (2.0 * assign22110_e17073));
        locals.var_tmf2_dn7 = (((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7)) / (2.0 * assign22110_e17073));
        locals.var_tmf2_dn8 = (((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8)) / (2.0 * assign22110_e17073));
        locals.var_tmf2_dn9 = (((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9)) / (2.0 * assign22110_e17073));
        locals.var_tmf2_dn10 = (((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10)) / (2.0 * assign22110_e17073));
        locals.var_tmf2_dn11 = (((locals.var_t3_dn11 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn11)) / (2.0 * assign22110_e17073));
        locals.var_tmf2_dn14 = (((locals.var_t3_dn14 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn14)) / (2.0 * assign22110_e17073));
        locals.var_tmf2_rv = 0.0;

        let assign22120_e17078: f64 = (locals.var_t3 / locals.var_tmf2);
        let assign22120_e17079: f64 = (1.0 + assign22120_e17078);
        let assign22120_e17080: f64 = (0.5 * assign22120_e17079);
        locals.var_t5 = assign22120_e17080;
        locals.var_t5_dn0 = (0.5 * (((locals.var_t3_dn0 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn2 = (0.5 * (((locals.var_t3_dn2 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn4 = (0.5 * (((locals.var_t3_dn4 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn5 = (0.5 * (((locals.var_t3_dn5 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn6 = (0.5 * (((locals.var_t3_dn6 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn7 = (0.5 * (((locals.var_t3_dn7 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn8 = (0.5 * (((locals.var_t3_dn8 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn9 = (0.5 * (((locals.var_t3_dn9 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn10 = (0.5 * (((locals.var_t3_dn10 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn11 = (0.5 * (((locals.var_t3_dn11 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn14 = (0.5 * (((locals.var_t3_dn14 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_rv = 0.0;

        let assign22130_e17084: f64 = (locals.var_t3 + locals.var_tmf2);
        let assign22130_e17085: f64 = (0.5 * assign22130_e17084);
        locals.var_t4 = assign22130_e17085;
        locals.var_t4_dn0 = (0.5 * (locals.var_t3_dn0 + locals.var_tmf2_dn0));
        locals.var_t4_dn2 = (0.5 * (locals.var_t3_dn2 + locals.var_tmf2_dn2));
        locals.var_t4_dn4 = (0.5 * (locals.var_t3_dn4 + locals.var_tmf2_dn4));
        locals.var_t4_dn5 = (0.5 * (locals.var_t3_dn5 + locals.var_tmf2_dn5));
        locals.var_t4_dn6 = (0.5 * (locals.var_t3_dn6 + locals.var_tmf2_dn6));
        locals.var_t4_dn7 = (0.5 * (locals.var_t3_dn7 + locals.var_tmf2_dn7));
        locals.var_t4_dn8 = (0.5 * (locals.var_t3_dn8 + locals.var_tmf2_dn8));
        locals.var_t4_dn9 = (0.5 * (locals.var_t3_dn9 + locals.var_tmf2_dn9));
        locals.var_t4_dn10 = (0.5 * (locals.var_t3_dn10 + locals.var_tmf2_dn10));
        locals.var_t4_dn11 = (0.5 * (locals.var_t3_dn11 + locals.var_tmf2_dn11));
        locals.var_t4_dn14 = (0.5 * (locals.var_t3_dn14 + locals.var_tmf2_dn14));
        locals.var_t4_rv = 0.0;

        let assign22140_e17088: f64 = if locals.var_t4 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard428 = assign22140_e17088;
        locals.var_guard428_rv = 0.0;

        let (assign22150_e17092, assign22150_e17092_d_n0, assign22150_e17092_d_n2, assign22150_e17092_d_n4, assign22150_e17092_d_n5, assign22150_e17092_d_n6, assign22150_e17092_d_n7, assign22150_e17092_d_n8, assign22150_e17092_d_n9, assign22150_e17092_d_n10, assign22150_e17092_d_n11, assign22150_e17092_d_n14,) = {
    if (locals.var_guard428 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign22150_e17092;
        locals.var_t4_dn0 = assign22150_e17092_d_n0;
        locals.var_t4_dn2 = assign22150_e17092_d_n2;
        locals.var_t4_dn4 = assign22150_e17092_d_n4;
        locals.var_t4_dn5 = assign22150_e17092_d_n5;
        locals.var_t4_dn6 = assign22150_e17092_d_n6;
        locals.var_t4_dn7 = assign22150_e17092_d_n7;
        locals.var_t4_dn8 = assign22150_e17092_d_n8;
        locals.var_t4_dn9 = assign22150_e17092_d_n9;
        locals.var_t4_dn10 = assign22150_e17092_d_n10;
        locals.var_t4_dn11 = assign22150_e17092_d_n11;
        locals.var_t4_dn14 = assign22150_e17092_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign22160_e17096, assign22160_e17096_d_n0, assign22160_e17096_d_n2, assign22160_e17096_d_n4, assign22160_e17096_d_n5, assign22160_e17096_d_n6, assign22160_e17096_d_n7, assign22160_e17096_d_n8, assign22160_e17096_d_n9, assign22160_e17096_d_n10, assign22160_e17096_d_n11, assign22160_e17096_d_n14,) = {
    if (locals.var_guard428 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign22160_e17096;
        locals.var_t5_dn0 = assign22160_e17096_d_n0;
        locals.var_t5_dn2 = assign22160_e17096_d_n2;
        locals.var_t5_dn4 = assign22160_e17096_d_n4;
        locals.var_t5_dn5 = assign22160_e17096_d_n5;
        locals.var_t5_dn6 = assign22160_e17096_d_n6;
        locals.var_t5_dn7 = assign22160_e17096_d_n7;
        locals.var_t5_dn8 = assign22160_e17096_d_n8;
        locals.var_t5_dn9 = assign22160_e17096_d_n9;
        locals.var_t5_dn10 = assign22160_e17096_d_n10;
        locals.var_t5_dn11 = assign22160_e17096_d_n11;
        locals.var_t5_dn14 = assign22160_e17096_d_n14;
        locals.var_t5_rv = 0.0;

        let assign22170_e17099: f64 = (locals.var_t4 + 1e-25);
        locals.var_t4 = assign22170_e17099;
        locals.var_t4_dn0 = locals.var_t4_dn0;
        locals.var_t4_dn2 = locals.var_t4_dn2;
        locals.var_t4_dn4 = locals.var_t4_dn4;
        locals.var_t4_dn5 = locals.var_t4_dn5;
        locals.var_t4_dn6 = locals.var_t4_dn6;
        locals.var_t4_dn7 = locals.var_t4_dn7;
        locals.var_t4_dn8 = locals.var_t4_dn8;
        locals.var_t4_dn9 = locals.var_t4_dn9;
        locals.var_t4_dn10 = locals.var_t4_dn10;
        locals.var_t4_dn11 = locals.var_t4_dn11;
        locals.var_t4_dn14 = locals.var_t4_dn14;
        locals.var_t4_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_59(
        locals: &mut StampLocals,
    ) {
        let assign22180_e17101: f64 = (locals.var_t4).sqrt();
        locals.var_tx = assign22180_e17101;
        locals.var_tx_dn0 = (locals.var_t4_dn0 / (2.0 * assign22180_e17101));
        locals.var_tx_dn2 = (locals.var_t4_dn2 / (2.0 * assign22180_e17101));
        locals.var_tx_dn4 = (locals.var_t4_dn4 / (2.0 * assign22180_e17101));
        locals.var_tx_dn5 = (locals.var_t4_dn5 / (2.0 * assign22180_e17101));
        locals.var_tx_dn6 = (locals.var_t4_dn6 / (2.0 * assign22180_e17101));
        locals.var_tx_dn7 = (locals.var_t4_dn7 / (2.0 * assign22180_e17101));
        locals.var_tx_dn8 = (locals.var_t4_dn8 / (2.0 * assign22180_e17101));
        locals.var_tx_dn9 = (locals.var_t4_dn9 / (2.0 * assign22180_e17101));
        locals.var_tx_dn10 = (locals.var_t4_dn10 / (2.0 * assign22180_e17101));
        locals.var_tx_dn11 = (locals.var_t4_dn11 / (2.0 * assign22180_e17101));
        locals.var_tx_dn14 = (locals.var_t4_dn14 / (2.0 * assign22180_e17101));
        locals.var_tx_rv = 0.0;

        let assign22190_e17106: f64 = (1.0 - locals.var_tx);
        let assign22190_e17107: f64 = (locals.var_t1 * assign22190_e17106);
        let assign22190_e17108: f64 = (locals.var_t2 + assign22190_e17107);
        locals.var_pslsat = assign22190_e17108;
        locals.var_pslsat_dn0 = (locals.var_t2_dn0 + ((locals.var_t1_dn0 * assign22190_e17106) + (locals.var_t1 * (-locals.var_tx_dn0))));
        locals.var_pslsat_dn2 = (locals.var_t2_dn2 + ((locals.var_t1_dn2 * assign22190_e17106) + (locals.var_t1 * (-locals.var_tx_dn2))));
        locals.var_pslsat_dn4 = (locals.var_t2_dn4 + ((locals.var_t1_dn4 * assign22190_e17106) + (locals.var_t1 * (-locals.var_tx_dn4))));
        locals.var_pslsat_dn5 = (locals.var_t2_dn5 + ((locals.var_t1_dn5 * assign22190_e17106) + (locals.var_t1 * (-locals.var_tx_dn5))));
        locals.var_pslsat_dn6 = (locals.var_t2_dn6 + ((locals.var_t1_dn6 * assign22190_e17106) + (locals.var_t1 * (-locals.var_tx_dn6))));
        locals.var_pslsat_dn7 = (locals.var_t2_dn7 + ((locals.var_t1_dn7 * assign22190_e17106) + (locals.var_t1 * (-locals.var_tx_dn7))));
        locals.var_pslsat_dn8 = (locals.var_t2_dn8 + ((locals.var_t1_dn8 * assign22190_e17106) + (locals.var_t1 * (-locals.var_tx_dn8))));
        locals.var_pslsat_dn9 = (locals.var_t2_dn9 + ((locals.var_t1_dn9 * assign22190_e17106) + (locals.var_t1 * (-locals.var_tx_dn9))));
        locals.var_pslsat_dn10 = (locals.var_t2_dn10 + ((locals.var_t1_dn10 * assign22190_e17106) + (locals.var_t1 * (-locals.var_tx_dn10))));
        locals.var_pslsat_dn11 = (locals.var_t2_dn11 + ((locals.var_t1_dn11 * assign22190_e17106) + (locals.var_t1 * (-locals.var_tx_dn11))));
        locals.var_pslsat_dn14 = (locals.var_t2_dn14 + ((locals.var_t1_dn14 * assign22190_e17106) + (locals.var_t1 * (-locals.var_tx_dn14))));
        locals.var_pslsat_rv = 0.0;

        let assign22200_e17111: f64 = (locals.var_pslsat - locals.var_pb2c);
        locals.var_vdsats = assign22200_e17111;
        locals.var_vdsats_dn0 = (locals.var_pslsat_dn0 - locals.var_pb2c_dn0);
        locals.var_vdsats_dn2 = (locals.var_pslsat_dn2 - locals.var_pb2c_dn2);
        locals.var_vdsats_dn4 = (locals.var_pslsat_dn4 - locals.var_pb2c_dn4);
        locals.var_vdsats_dn5 = (locals.var_pslsat_dn5 - locals.var_pb2c_dn5);
        locals.var_vdsats_dn6 = (locals.var_pslsat_dn6 - locals.var_pb2c_dn6);
        locals.var_vdsats_dn7 = (locals.var_pslsat_dn7 - locals.var_pb2c_dn7);
        locals.var_vdsats_dn8 = (locals.var_pslsat_dn8 - locals.var_pb2c_dn8);
        locals.var_vdsats_dn9 = (locals.var_pslsat_dn9 - locals.var_pb2c_dn9);
        locals.var_vdsats_dn10 = (locals.var_pslsat_dn10 - locals.var_pb2c_dn10);
        locals.var_vdsats_dn11 = (locals.var_pslsat_dn11 - locals.var_pb2c_dn11);
        locals.var_vdsats_dn14 = (locals.var_pslsat_dn14 - locals.var_pb2c_dn14);
        locals.var_vdsats_rv = 0.0;

        let assign22210_e17114: f64 = (locals.var_vdsats - 0.1);
        let assign22210_e17116: f64 = (assign22210_e17114 - 0.05);
        locals.var_tmf1 = assign22210_e17116;
        locals.var_tmf1_dn0 = locals.var_vdsats_dn0;
        locals.var_tmf1_dn2 = locals.var_vdsats_dn2;
        locals.var_tmf1_dn4 = locals.var_vdsats_dn4;
        locals.var_tmf1_dn5 = locals.var_vdsats_dn5;
        locals.var_tmf1_dn6 = locals.var_vdsats_dn6;
        locals.var_tmf1_dn7 = locals.var_vdsats_dn7;
        locals.var_tmf1_dn8 = locals.var_vdsats_dn8;
        locals.var_tmf1_dn9 = locals.var_vdsats_dn9;
        locals.var_tmf1_dn10 = locals.var_vdsats_dn10;
        locals.var_tmf1_dn11 = locals.var_vdsats_dn11;
        locals.var_tmf1_dn14 = locals.var_vdsats_dn14;
        locals.var_tmf1_rv = 0.0;

        let assign22220_e17119: f64 = (4.0 * 0.1);
        let assign22220_e17121: f64 = (assign22220_e17119 * 0.05);
        locals.var_tmf2 = assign22220_e17121;
        locals.var_tmf2_dn0 = 0.0;
        locals.var_tmf2_dn2 = 0.0;
        locals.var_tmf2_dn4 = 0.0;
        locals.var_tmf2_dn5 = 0.0;
        locals.var_tmf2_dn6 = 0.0;
        locals.var_tmf2_dn7 = 0.0;
        locals.var_tmf2_dn8 = 0.0;
        locals.var_tmf2_dn9 = 0.0;
        locals.var_tmf2_dn10 = 0.0;
        locals.var_tmf2_dn11 = 0.0;
        locals.var_tmf2_dn14 = 0.0;
        locals.var_tmf2_rv = 0.0;

        let (assign22230_e17128, assign22230_e17128_d_n0, assign22230_e17128_d_n2, assign22230_e17128_d_n4, assign22230_e17128_d_n5, assign22230_e17128_d_n6, assign22230_e17128_d_n7, assign22230_e17128_d_n8, assign22230_e17128_d_n9, assign22230_e17128_d_n10, assign22230_e17128_d_n11, assign22230_e17128_d_n14,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    } else {
        let assign22230_e17127: f64 = (-locals.var_tmf2);
        (assign22230_e17127, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
    }
};
        locals.var_tmf2 = assign22230_e17128;
        locals.var_tmf2_dn0 = assign22230_e17128_d_n0;
        locals.var_tmf2_dn2 = assign22230_e17128_d_n2;
        locals.var_tmf2_dn4 = assign22230_e17128_d_n4;
        locals.var_tmf2_dn5 = assign22230_e17128_d_n5;
        locals.var_tmf2_dn6 = assign22230_e17128_d_n6;
        locals.var_tmf2_dn7 = assign22230_e17128_d_n7;
        locals.var_tmf2_dn8 = assign22230_e17128_d_n8;
        locals.var_tmf2_dn9 = assign22230_e17128_d_n9;
        locals.var_tmf2_dn10 = assign22230_e17128_d_n10;
        locals.var_tmf2_dn11 = assign22230_e17128_d_n11;
        locals.var_tmf2_dn14 = assign22230_e17128_d_n14;
        locals.var_tmf2_rv = 0.0;

        let assign22240_e17131: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign22240_e17133: f64 = (assign22240_e17131 + locals.var_tmf2);
        let assign22240_e17134: f64 = (assign22240_e17133).sqrt();
        locals.var_tmf2 = assign22240_e17134;
        locals.var_tmf2_dn0 = ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign22240_e17134));
        locals.var_tmf2_dn2 = ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign22240_e17134));
        locals.var_tmf2_dn4 = ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign22240_e17134));
        locals.var_tmf2_dn5 = ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign22240_e17134));
        locals.var_tmf2_dn6 = ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign22240_e17134));
        locals.var_tmf2_dn7 = ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign22240_e17134));
        locals.var_tmf2_dn8 = ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign22240_e17134));
        locals.var_tmf2_dn9 = ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign22240_e17134));
        locals.var_tmf2_dn10 = ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign22240_e17134));
        locals.var_tmf2_dn11 = ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign22240_e17134));
        locals.var_tmf2_dn14 = ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign22240_e17134));
        locals.var_tmf2_rv = 0.0;

        let assign22250_e17139: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign22250_e17140: f64 = (1.0 + assign22250_e17139);
        let assign22250_e17141: f64 = (0.5 * assign22250_e17140);
        locals.var_t6 = assign22250_e17141;
        locals.var_t6_dn0 = (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn2 = (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn4 = (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn5 = (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn6 = (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn7 = (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn8 = (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn9 = (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn10 = (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn11 = (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn14 = (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_rv = 0.0;

        let assign22260_e17146: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign22260_e17147: f64 = (0.5 * assign22260_e17146);
        let assign22260_e17148: f64 = (0.1 + assign22260_e17147);
        locals.var_vdsats = assign22260_e17148;
        locals.var_vdsats_dn0 = (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0));
        locals.var_vdsats_dn2 = (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2));
        locals.var_vdsats_dn4 = (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4));
        locals.var_vdsats_dn5 = (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5));
        locals.var_vdsats_dn6 = (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6));
        locals.var_vdsats_dn7 = (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7));
        locals.var_vdsats_dn8 = (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8));
        locals.var_vdsats_dn9 = (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9));
        locals.var_vdsats_dn10 = (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10));
        locals.var_vdsats_dn11 = (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11));
        locals.var_vdsats_dn14 = (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14));
        locals.var_vdsats_rv = 0.0;

        let assign22270_e17151: f64 = (locals.var_vds / locals.var_vdsats);
        locals.var_t1 = assign22270_e17151;
        locals.var_t1_dn0 = (((locals.var_vds_dn0 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn0)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn2 = (((locals.var_vds_dn2 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn2)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn4 = (((locals.var_vds_dn4 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn4)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn5 = (((locals.var_vds_dn5 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn5)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn6 = (((locals.var_vds_dn6 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn6)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn7 = (((locals.var_vds_dn7 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn7)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn8 = (((locals.var_vds_dn8 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn8)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn9 = (((locals.var_vds_dn9 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn9)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn10 = (((locals.var_vds_dn10 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn10)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn11 = (((locals.var_vds_dn11 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn11)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn14 = (((locals.var_vds_dn14 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn14)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_rv = 0.0;

        let assign22280_e17154: f64 = locals.var_t1;
        locals.var_tmf1 = assign22280_e17154;
        locals.var_tmf1_dn0 = locals.var_t1_dn0;
        locals.var_tmf1_dn2 = locals.var_t1_dn2;
        locals.var_tmf1_dn4 = locals.var_t1_dn4;
        locals.var_tmf1_dn5 = locals.var_t1_dn5;
        locals.var_tmf1_dn6 = locals.var_t1_dn6;
        locals.var_tmf1_dn7 = locals.var_t1_dn7;
        locals.var_tmf1_dn8 = locals.var_t1_dn8;
        locals.var_tmf1_dn9 = locals.var_t1_dn9;
        locals.var_tmf1_dn10 = locals.var_t1_dn10;
        locals.var_tmf1_dn11 = locals.var_t1_dn11;
        locals.var_tmf1_dn14 = locals.var_t1_dn14;
        locals.var_tmf1_rv = 0.0;

        let assign22290_e17157: f64 = (locals.var_tmf1 * locals.var_tmf1);
        locals.var_tmf2 = assign22290_e17157;
        locals.var_tmf2_dn0 = ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0));
        locals.var_tmf2_dn2 = ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2));
        locals.var_tmf2_dn4 = ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4));
        locals.var_tmf2_dn5 = ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5));
        locals.var_tmf2_dn6 = ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6));
        locals.var_tmf2_dn7 = ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7));
        locals.var_tmf2_dn8 = ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8));
        locals.var_tmf2_dn9 = ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9));
        locals.var_tmf2_dn10 = ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10));
        locals.var_tmf2_dn11 = ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11));
        locals.var_tmf2_dn14 = ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14));
        locals.var_tmf2_rv = 0.0;

        let assign22300_e17160: f64 = (locals.var_tmf2 * locals.var_tmf1);
        locals.var_tmf3 = assign22300_e17160;
        locals.var_tmf3_dn0 = ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0));
        locals.var_tmf3_dn2 = ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2));
        locals.var_tmf3_dn4 = ((locals.var_tmf2_dn4 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn4));
        locals.var_tmf3_dn5 = ((locals.var_tmf2_dn5 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn5));
        locals.var_tmf3_dn6 = ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6));
        locals.var_tmf3_dn7 = ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7));
        locals.var_tmf3_dn8 = ((locals.var_tmf2_dn8 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn8));
        locals.var_tmf3_dn9 = ((locals.var_tmf2_dn9 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn9));
        locals.var_tmf3_dn10 = ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10));
        locals.var_tmf3_dn11 = ((locals.var_tmf2_dn11 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn11));
        locals.var_tmf3_dn14 = ((locals.var_tmf2_dn14 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn14));
        locals.var_tmf3_rv = 0.0;

        let assign22310_e17163: f64 = (locals.var_tmf2 * locals.var_tmf2);
        locals.var_tmf4 = assign22310_e17163;
        locals.var_tmf4_dn0 = ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0));
        locals.var_tmf4_dn2 = ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2));
        locals.var_tmf4_dn4 = ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4));
        locals.var_tmf4_dn5 = ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5));
        locals.var_tmf4_dn6 = ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6));
        locals.var_tmf4_dn7 = ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7));
        locals.var_tmf4_dn8 = ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8));
        locals.var_tmf4_dn9 = ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9));
        locals.var_tmf4_dn10 = ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10));
        locals.var_tmf4_dn11 = ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11));
        locals.var_tmf4_dn14 = ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14));
        locals.var_tmf4_rv = 0.0;

        let assign22320_e17167: f64 = (1.0 + locals.var_tmf1);
        let assign22320_e17169: f64 = (assign22320_e17167 + locals.var_tmf2);
        let assign22320_e17171: f64 = (assign22320_e17169 + locals.var_tmf3);
        let assign22320_e17173: f64 = (assign22320_e17171 + locals.var_tmf4);
        let assign22320_e17174: f64 = (1.0 / assign22320_e17173);
        locals.var_tx = assign22320_e17174;
        locals.var_tx_dn0 = (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign22320_e17173 * assign22320_e17173)));
        locals.var_tx_dn2 = (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign22320_e17173 * assign22320_e17173)));
        locals.var_tx_dn4 = (-((((locals.var_tmf1_dn4 + locals.var_tmf2_dn4) + locals.var_tmf3_dn4) + locals.var_tmf4_dn4) / (assign22320_e17173 * assign22320_e17173)));
        locals.var_tx_dn5 = (-((((locals.var_tmf1_dn5 + locals.var_tmf2_dn5) + locals.var_tmf3_dn5) + locals.var_tmf4_dn5) / (assign22320_e17173 * assign22320_e17173)));
        locals.var_tx_dn6 = (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign22320_e17173 * assign22320_e17173)));
        locals.var_tx_dn7 = (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign22320_e17173 * assign22320_e17173)));
        locals.var_tx_dn8 = (-((((locals.var_tmf1_dn8 + locals.var_tmf2_dn8) + locals.var_tmf3_dn8) + locals.var_tmf4_dn8) / (assign22320_e17173 * assign22320_e17173)));
        locals.var_tx_dn9 = (-((((locals.var_tmf1_dn9 + locals.var_tmf2_dn9) + locals.var_tmf3_dn9) + locals.var_tmf4_dn9) / (assign22320_e17173 * assign22320_e17173)));
        locals.var_tx_dn10 = (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign22320_e17173 * assign22320_e17173)));
        locals.var_tx_dn11 = (-((((locals.var_tmf1_dn11 + locals.var_tmf2_dn11) + locals.var_tmf3_dn11) + locals.var_tmf4_dn11) / (assign22320_e17173 * assign22320_e17173)));
        locals.var_tx_dn14 = (-((((locals.var_tmf1_dn14 + locals.var_tmf2_dn14) + locals.var_tmf3_dn14) + locals.var_tmf4_dn14) / (assign22320_e17173 * assign22320_e17173)));
        locals.var_tx_rv = 0.0;

        let assign22330_e17178: f64 = (2.0 * locals.var_tmf1);
        let assign22330_e17179: f64 = (1.0 + assign22330_e17178);
        let assign22330_e17182: f64 = (3.0 * locals.var_tmf2);
        let assign22330_e17183: f64 = (assign22330_e17179 + assign22330_e17182);
        let assign22330_e17186: f64 = (4.0 * locals.var_tmf3);
        let assign22330_e17187: f64 = (assign22330_e17183 + assign22330_e17186);
        let assign22330_e17188: f64 = (-assign22330_e17187);
        let assign22330_e17190: f64 = (assign22330_e17188 * locals.var_tx);
        let assign22330_e17192: f64 = (assign22330_e17190 * locals.var_tx);
        locals.var_t0 = assign22330_e17192;
        locals.var_t0_dn0 = (((((-(((2.0 * locals.var_tmf1_dn0) + (3.0 * locals.var_tmf2_dn0)) + (4.0 * locals.var_tmf3_dn0))) * locals.var_tx) + (assign22330_e17188 * locals.var_tx_dn0)) * locals.var_tx) + (assign22330_e17190 * locals.var_tx_dn0));
        locals.var_t0_dn2 = (((((-(((2.0 * locals.var_tmf1_dn2) + (3.0 * locals.var_tmf2_dn2)) + (4.0 * locals.var_tmf3_dn2))) * locals.var_tx) + (assign22330_e17188 * locals.var_tx_dn2)) * locals.var_tx) + (assign22330_e17190 * locals.var_tx_dn2));
        locals.var_t0_dn4 = (((((-(((2.0 * locals.var_tmf1_dn4) + (3.0 * locals.var_tmf2_dn4)) + (4.0 * locals.var_tmf3_dn4))) * locals.var_tx) + (assign22330_e17188 * locals.var_tx_dn4)) * locals.var_tx) + (assign22330_e17190 * locals.var_tx_dn4));
        locals.var_t0_dn5 = (((((-(((2.0 * locals.var_tmf1_dn5) + (3.0 * locals.var_tmf2_dn5)) + (4.0 * locals.var_tmf3_dn5))) * locals.var_tx) + (assign22330_e17188 * locals.var_tx_dn5)) * locals.var_tx) + (assign22330_e17190 * locals.var_tx_dn5));
        locals.var_t0_dn6 = (((((-(((2.0 * locals.var_tmf1_dn6) + (3.0 * locals.var_tmf2_dn6)) + (4.0 * locals.var_tmf3_dn6))) * locals.var_tx) + (assign22330_e17188 * locals.var_tx_dn6)) * locals.var_tx) + (assign22330_e17190 * locals.var_tx_dn6));
        locals.var_t0_dn7 = (((((-(((2.0 * locals.var_tmf1_dn7) + (3.0 * locals.var_tmf2_dn7)) + (4.0 * locals.var_tmf3_dn7))) * locals.var_tx) + (assign22330_e17188 * locals.var_tx_dn7)) * locals.var_tx) + (assign22330_e17190 * locals.var_tx_dn7));
        locals.var_t0_dn8 = (((((-(((2.0 * locals.var_tmf1_dn8) + (3.0 * locals.var_tmf2_dn8)) + (4.0 * locals.var_tmf3_dn8))) * locals.var_tx) + (assign22330_e17188 * locals.var_tx_dn8)) * locals.var_tx) + (assign22330_e17190 * locals.var_tx_dn8));
        locals.var_t0_dn9 = (((((-(((2.0 * locals.var_tmf1_dn9) + (3.0 * locals.var_tmf2_dn9)) + (4.0 * locals.var_tmf3_dn9))) * locals.var_tx) + (assign22330_e17188 * locals.var_tx_dn9)) * locals.var_tx) + (assign22330_e17190 * locals.var_tx_dn9));
        locals.var_t0_dn10 = (((((-(((2.0 * locals.var_tmf1_dn10) + (3.0 * locals.var_tmf2_dn10)) + (4.0 * locals.var_tmf3_dn10))) * locals.var_tx) + (assign22330_e17188 * locals.var_tx_dn10)) * locals.var_tx) + (assign22330_e17190 * locals.var_tx_dn10));
        locals.var_t0_dn11 = (((((-(((2.0 * locals.var_tmf1_dn11) + (3.0 * locals.var_tmf2_dn11)) + (4.0 * locals.var_tmf3_dn11))) * locals.var_tx) + (assign22330_e17188 * locals.var_tx_dn11)) * locals.var_tx) + (assign22330_e17190 * locals.var_tx_dn11));
        locals.var_t0_dn14 = (((((-(((2.0 * locals.var_tmf1_dn14) + (3.0 * locals.var_tmf2_dn14)) + (4.0 * locals.var_tmf3_dn14))) * locals.var_tx) + (assign22330_e17188 * locals.var_tx_dn14)) * locals.var_tx) + (assign22330_e17190 * locals.var_tx_dn14));
        locals.var_t0_rv = 0.0;

        let assign22340_e17196: f64 = (1.0 - locals.var_tx);
        let assign22340_e17197: f64 = assign22340_e17196;
        locals.var_tx = assign22340_e17197;
        locals.var_tx_dn0 = (-locals.var_tx_dn0);
        locals.var_tx_dn2 = (-locals.var_tx_dn2);
        locals.var_tx_dn4 = (-locals.var_tx_dn4);
        locals.var_tx_dn5 = (-locals.var_tx_dn5);
        locals.var_tx_dn6 = (-locals.var_tx_dn6);
        locals.var_tx_dn7 = (-locals.var_tx_dn7);
        locals.var_tx_dn8 = (-locals.var_tx_dn8);
        locals.var_tx_dn9 = (-locals.var_tx_dn9);
        locals.var_tx_dn10 = (-locals.var_tx_dn10);
        locals.var_tx_dn11 = (-locals.var_tx_dn11);
        locals.var_tx_dn14 = (-locals.var_tx_dn14);
        locals.var_tx_rv = 0.0;

        let assign22350_e17199: f64 = (-locals.var_t0);
        locals.var_t0 = assign22350_e17199;
        locals.var_t0_dn0 = (-locals.var_t0_dn0);
        locals.var_t0_dn2 = (-locals.var_t0_dn2);
        locals.var_t0_dn4 = (-locals.var_t0_dn4);
        locals.var_t0_dn5 = (-locals.var_t0_dn5);
        locals.var_t0_dn6 = (-locals.var_t0_dn6);
        locals.var_t0_dn7 = (-locals.var_t0_dn7);
        locals.var_t0_dn8 = (-locals.var_t0_dn8);
        locals.var_t0_dn9 = (-locals.var_t0_dn9);
        locals.var_t0_dn10 = (-locals.var_t0_dn10);
        locals.var_t0_dn11 = (-locals.var_t0_dn11);
        locals.var_t0_dn14 = (-locals.var_t0_dn14);
        locals.var_t0_rv = 0.0;

        let assign22360_e17202: f64 = (locals.var_tx * locals.var_tx);
        locals.var_fmdvds = assign22360_e17202;
        locals.var_fmdvds_dn0 = ((locals.var_tx_dn0 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn0));
        locals.var_fmdvds_dn2 = ((locals.var_tx_dn2 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn2));
        locals.var_fmdvds_dn4 = ((locals.var_tx_dn4 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn4));
        locals.var_fmdvds_dn5 = ((locals.var_tx_dn5 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn5));
        locals.var_fmdvds_dn6 = ((locals.var_tx_dn6 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn6));
        locals.var_fmdvds_dn7 = ((locals.var_tx_dn7 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn7));
        locals.var_fmdvds_dn8 = ((locals.var_tx_dn8 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn8));
        locals.var_fmdvds_dn9 = ((locals.var_tx_dn9 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn9));
        locals.var_fmdvds_dn10 = ((locals.var_tx_dn10 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn10));
        locals.var_fmdvds_dn11 = ((locals.var_tx_dn11 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn11));
        locals.var_fmdvds_dn14 = ((locals.var_tx_dn14 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn14));
        locals.var_fmdvds_rv = 0.0;

        let assign22370_e17205: f64 = if locals.var_flg_qmetemp == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard429 = assign22370_e17205;
        locals.var_guard429_rv = 0.0;

        let (assign22380_e17209,) = {
    if (locals.var_guard429 != 0.0) {
        (0.0,)
    } else {
        (locals.var_flg_qme,)
    }
};
        locals.var_flg_qme = assign22380_e17209;
        locals.var_flg_qme_rv = 0.0;

        let (assign22390_e17214,) = {
    if (locals.var_guard429 == 0.0) {
        (1.0,)
    } else {
        (locals.var_flg_qme,)
    }
};
        locals.var_flg_qme = assign22390_e17214;
        locals.var_flg_qme_rv = 0.0;

        locals.var_t1 = locals.var_qnsub_esi2;
        locals.var_t1_dn0 = locals.var_qnsub_esi2_dn0;
        locals.var_t1_dn2 = locals.var_qnsub_esi2_dn2;
        locals.var_t1_dn4 = locals.var_qnsub_esi2_dn4;
        locals.var_t1_dn5 = locals.var_qnsub_esi2_dn5;
        locals.var_t1_dn6 = locals.var_qnsub_esi2_dn6;
        locals.var_t1_dn7 = locals.var_qnsub_esi2_dn7;
        locals.var_t1_dn8 = locals.var_qnsub_esi2_dn8;
        locals.var_t1_dn9 = locals.var_qnsub_esi2_dn9;
        locals.var_t1_dn10 = locals.var_qnsub_esi2_dn10;
        locals.var_t1_dn11 = locals.var_qnsub_esi2_dn11;
        locals.var_t1_dn14 = locals.var_qnsub_esi2_dn14;
        locals.var_t1_rv = 0.0;

        let assign22410_e17218: f64 = (locals.var_t1 * locals.var_pb20);
        let assign22410_e17219: f64 = (assign22410_e17218).sqrt();
        locals.var_t2 = assign22410_e17219;
        locals.var_t2_dn0 = (((locals.var_t1_dn0 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn0)) / (2.0 * assign22410_e17219));
        locals.var_t2_dn2 = (((locals.var_t1_dn2 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn2)) / (2.0 * assign22410_e17219));
        locals.var_t2_dn4 = (((locals.var_t1_dn4 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn4)) / (2.0 * assign22410_e17219));
        locals.var_t2_dn5 = (((locals.var_t1_dn5 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn5)) / (2.0 * assign22410_e17219));
        locals.var_t2_dn6 = (((locals.var_t1_dn6 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn6)) / (2.0 * assign22410_e17219));
        locals.var_t2_dn7 = (((locals.var_t1_dn7 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn7)) / (2.0 * assign22410_e17219));
        locals.var_t2_dn8 = (((locals.var_t1_dn8 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn8)) / (2.0 * assign22410_e17219));
        locals.var_t2_dn9 = (((locals.var_t1_dn9 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn9)) / (2.0 * assign22410_e17219));
        locals.var_t2_dn10 = (((locals.var_t1_dn10 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn10)) / (2.0 * assign22410_e17219));
        locals.var_t2_dn11 = (((locals.var_t1_dn11 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn11)) / (2.0 * assign22410_e17219));
        locals.var_t2_dn14 = (((locals.var_t1_dn14 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn14)) / (2.0 * assign22410_e17219));
        locals.var_t2_rv = 0.0;

        let assign22420_e17222: f64 = (locals.var_pb20 + locals.var_vfb);
        let assign22420_e17225: f64 = (locals.var_t2 * locals.var_cox0_inv);
        let assign22420_e17226: f64 = (assign22420_e17222 + assign22420_e17225);
        locals.var_vthq = assign22420_e17226;
        locals.var_vthq_dn0 = (locals.var_pb20_dn0 + (locals.var_t2_dn0 * locals.var_cox0_inv));
        locals.var_vthq_dn2 = (locals.var_pb20_dn2 + (locals.var_t2_dn2 * locals.var_cox0_inv));
        locals.var_vthq_dn4 = (locals.var_pb20_dn4 + (locals.var_t2_dn4 * locals.var_cox0_inv));
        locals.var_vthq_dn5 = (locals.var_pb20_dn5 + (locals.var_t2_dn5 * locals.var_cox0_inv));
        locals.var_vthq_dn6 = (locals.var_pb20_dn6 + (locals.var_t2_dn6 * locals.var_cox0_inv));
        locals.var_vthq_dn7 = (locals.var_pb20_dn7 + (locals.var_t2_dn7 * locals.var_cox0_inv));
        locals.var_vthq_dn8 = (locals.var_pb20_dn8 + (locals.var_t2_dn8 * locals.var_cox0_inv));
        locals.var_vthq_dn9 = (locals.var_pb20_dn9 + (locals.var_t2_dn9 * locals.var_cox0_inv));
        locals.var_vthq_dn10 = (locals.var_pb20_dn10 + (locals.var_t2_dn10 * locals.var_cox0_inv));
        locals.var_vthq_dn11 = (locals.var_pb20_dn11 + (locals.var_t2_dn11 * locals.var_cox0_inv));
        locals.var_vthq_dn14 = (locals.var_pb20_dn14 + (locals.var_t2_dn14 * locals.var_cox0_inv));
        locals.var_vthq_rv = 0.0;

        let assign22430_e17229: f64 = if locals.var_flg_qme == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard430 = assign22430_e17229;
        locals.var_guard430_rv = 0.0;

        let (assign22440_e17233, assign22440_e17233_d_n0, assign22440_e17233_d_n2, assign22440_e17233_d_n4, assign22440_e17233_d_n5, assign22440_e17233_d_n6, assign22440_e17233_d_n7, assign22440_e17233_d_n8, assign22440_e17233_d_n9, assign22440_e17233_d_n10, assign22440_e17233_d_n11, assign22440_e17233_d_n14,) = {
    if (locals.var_guard430 != 0.0) {
        (locals.var_tox0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_toxe, locals.var_toxe_dn0, locals.var_toxe_dn2, locals.var_toxe_dn4, locals.var_toxe_dn5, locals.var_toxe_dn6, locals.var_toxe_dn7, locals.var_toxe_dn8, locals.var_toxe_dn9, locals.var_toxe_dn10, locals.var_toxe_dn11, locals.var_toxe_dn14,)
    }
};
        locals.var_toxe = assign22440_e17233;
        locals.var_toxe_dn0 = assign22440_e17233_d_n0;
        locals.var_toxe_dn2 = assign22440_e17233_d_n2;
        locals.var_toxe_dn4 = assign22440_e17233_d_n4;
        locals.var_toxe_dn5 = assign22440_e17233_d_n5;
        locals.var_toxe_dn6 = assign22440_e17233_d_n6;
        locals.var_toxe_dn7 = assign22440_e17233_d_n7;
        locals.var_toxe_dn8 = assign22440_e17233_d_n8;
        locals.var_toxe_dn9 = assign22440_e17233_d_n9;
        locals.var_toxe_dn10 = assign22440_e17233_d_n10;
        locals.var_toxe_dn11 = assign22440_e17233_d_n11;
        locals.var_toxe_dn14 = assign22440_e17233_d_n14;
        locals.var_toxe_rv = 0.0;

        let (assign22450_e17237, assign22450_e17237_d_n0, assign22450_e17237_d_n2, assign22450_e17237_d_n4, assign22450_e17237_d_n5, assign22450_e17237_d_n6, assign22450_e17237_d_n7, assign22450_e17237_d_n8, assign22450_e17237_d_n9, assign22450_e17237_d_n10, assign22450_e17237_d_n11, assign22450_e17237_d_n14,) = {
    if (locals.var_guard430 != 0.0) {
        (locals.var_cox0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cox, locals.var_cox_dn0, locals.var_cox_dn2, locals.var_cox_dn4, locals.var_cox_dn5, locals.var_cox_dn6, locals.var_cox_dn7, locals.var_cox_dn8, locals.var_cox_dn9, locals.var_cox_dn10, locals.var_cox_dn11, locals.var_cox_dn14,)
    }
};
        locals.var_cox = assign22450_e17237;
        locals.var_cox_dn0 = assign22450_e17237_d_n0;
        locals.var_cox_dn2 = assign22450_e17237_d_n2;
        locals.var_cox_dn4 = assign22450_e17237_d_n4;
        locals.var_cox_dn5 = assign22450_e17237_d_n5;
        locals.var_cox_dn6 = assign22450_e17237_d_n6;
        locals.var_cox_dn7 = assign22450_e17237_d_n7;
        locals.var_cox_dn8 = assign22450_e17237_d_n8;
        locals.var_cox_dn9 = assign22450_e17237_d_n9;
        locals.var_cox_dn10 = assign22450_e17237_d_n10;
        locals.var_cox_dn11 = assign22450_e17237_d_n11;
        locals.var_cox_dn14 = assign22450_e17237_d_n14;
        locals.var_cox_rv = 0.0;

        let (assign22460_e17241, assign22460_e17241_d_n0, assign22460_e17241_d_n2, assign22460_e17241_d_n4, assign22460_e17241_d_n5, assign22460_e17241_d_n6, assign22460_e17241_d_n7, assign22460_e17241_d_n8, assign22460_e17241_d_n9, assign22460_e17241_d_n10, assign22460_e17241_d_n11, assign22460_e17241_d_n14,) = {
    if (locals.var_guard430 != 0.0) {
        (locals.var_cox0_inv, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cox_inv, locals.var_cox_inv_dn0, locals.var_cox_inv_dn2, locals.var_cox_inv_dn4, locals.var_cox_inv_dn5, locals.var_cox_inv_dn6, locals.var_cox_inv_dn7, locals.var_cox_inv_dn8, locals.var_cox_inv_dn9, locals.var_cox_inv_dn10, locals.var_cox_inv_dn11, locals.var_cox_inv_dn14,)
    }
};
        locals.var_cox_inv = assign22460_e17241;
        locals.var_cox_inv_dn0 = assign22460_e17241_d_n0;
        locals.var_cox_inv_dn2 = assign22460_e17241_d_n2;
        locals.var_cox_inv_dn4 = assign22460_e17241_d_n4;
        locals.var_cox_inv_dn5 = assign22460_e17241_d_n5;
        locals.var_cox_inv_dn6 = assign22460_e17241_d_n6;
        locals.var_cox_inv_dn7 = assign22460_e17241_d_n7;
        locals.var_cox_inv_dn8 = assign22460_e17241_d_n8;
        locals.var_cox_inv_dn9 = assign22460_e17241_d_n9;
        locals.var_cox_inv_dn10 = assign22460_e17241_d_n10;
        locals.var_cox_inv_dn11 = assign22460_e17241_d_n11;
        locals.var_cox_inv_dn14 = assign22460_e17241_d_n14;
        locals.var_cox_inv_rv = 0.0;

        let (assign22470_e17249, assign22470_e17249_d_n0, assign22470_e17249_d_n2, assign22470_e17249_d_n4, assign22470_e17249_d_n5, assign22470_e17249_d_n6, assign22470_e17249_d_n7, assign22470_e17249_d_n8, assign22470_e17249_d_n9, assign22470_e17249_d_n10, assign22470_e17249_d_n11, assign22470_e17249_d_n14,) = {
    if (locals.var_guard430 != 0.0) {
        let assign22470_e17245: f64 = (locals.var_cnst0 * locals.var_cnst0);
        let assign22470_e17247: f64 = (assign22470_e17245 * locals.var_cox_inv);
        (assign22470_e17247, ((((locals.var_cnst0_dn0 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn0)) * locals.var_cox_inv) + (assign22470_e17245 * locals.var_cox_inv_dn0)), ((((locals.var_cnst0_dn2 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn2)) * locals.var_cox_inv) + (assign22470_e17245 * locals.var_cox_inv_dn2)), ((((locals.var_cnst0_dn4 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn4)) * locals.var_cox_inv) + (assign22470_e17245 * locals.var_cox_inv_dn4)), ((((locals.var_cnst0_dn5 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn5)) * locals.var_cox_inv) + (assign22470_e17245 * locals.var_cox_inv_dn5)), ((((locals.var_cnst0_dn6 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn6)) * locals.var_cox_inv) + (assign22470_e17245 * locals.var_cox_inv_dn6)), ((((locals.var_cnst0_dn7 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn7)) * locals.var_cox_inv) + (assign22470_e17245 * locals.var_cox_inv_dn7)), ((((locals.var_cnst0_dn8 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn8)) * locals.var_cox_inv) + (assign22470_e17245 * locals.var_cox_inv_dn8)), ((((locals.var_cnst0_dn9 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn9)) * locals.var_cox_inv) + (assign22470_e17245 * locals.var_cox_inv_dn9)), ((((locals.var_cnst0_dn10 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn10)) * locals.var_cox_inv) + (assign22470_e17245 * locals.var_cox_inv_dn10)), ((((locals.var_cnst0_dn11 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn11)) * locals.var_cox_inv) + (assign22470_e17245 * locals.var_cox_inv_dn11)), ((((locals.var_cnst0_dn14 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn14)) * locals.var_cox_inv) + (assign22470_e17245 * locals.var_cox_inv_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign22470_e17249;
        locals.var_t0_dn0 = assign22470_e17249_d_n0;
        locals.var_t0_dn2 = assign22470_e17249_d_n2;
        locals.var_t0_dn4 = assign22470_e17249_d_n4;
        locals.var_t0_dn5 = assign22470_e17249_d_n5;
        locals.var_t0_dn6 = assign22470_e17249_d_n6;
        locals.var_t0_dn7 = assign22470_e17249_d_n7;
        locals.var_t0_dn8 = assign22470_e17249_d_n8;
        locals.var_t0_dn9 = assign22470_e17249_d_n9;
        locals.var_t0_dn10 = assign22470_e17249_d_n10;
        locals.var_t0_dn11 = assign22470_e17249_d_n11;
        locals.var_t0_dn14 = assign22470_e17249_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign22480_e17255, assign22480_e17255_d_n0, assign22480_e17255_d_n2, assign22480_e17255_d_n4, assign22480_e17255_d_n5, assign22480_e17255_d_n6, assign22480_e17255_d_n7, assign22480_e17255_d_n8, assign22480_e17255_d_n9, assign22480_e17255_d_n10, assign22480_e17255_d_n11, assign22480_e17255_d_n14,) = {
    if (locals.var_guard430 != 0.0) {
        let assign22480_e17253: f64 = (locals.var_t0 * locals.var_cox_inv);
        (assign22480_e17253, ((locals.var_t0_dn0 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn0)), ((locals.var_t0_dn2 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn2)), ((locals.var_t0_dn4 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn4)), ((locals.var_t0_dn5 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn5)), ((locals.var_t0_dn6 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn6)), ((locals.var_t0_dn7 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn7)), ((locals.var_t0_dn8 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn8)), ((locals.var_t0_dn9 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn9)), ((locals.var_t0_dn10 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn10)), ((locals.var_t0_dn11 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn11)), ((locals.var_t0_dn14 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn14)),)
    } else {
        (locals.var_cnstcoxi, locals.var_cnstcoxi_dn0, locals.var_cnstcoxi_dn2, locals.var_cnstcoxi_dn4, locals.var_cnstcoxi_dn5, locals.var_cnstcoxi_dn6, locals.var_cnstcoxi_dn7, locals.var_cnstcoxi_dn8, locals.var_cnstcoxi_dn9, locals.var_cnstcoxi_dn10, locals.var_cnstcoxi_dn11, locals.var_cnstcoxi_dn14,)
    }
};
        locals.var_cnstcoxi = assign22480_e17255;
        locals.var_cnstcoxi_dn0 = assign22480_e17255_d_n0;
        locals.var_cnstcoxi_dn2 = assign22480_e17255_d_n2;
        locals.var_cnstcoxi_dn4 = assign22480_e17255_d_n4;
        locals.var_cnstcoxi_dn5 = assign22480_e17255_d_n5;
        locals.var_cnstcoxi_dn6 = assign22480_e17255_d_n6;
        locals.var_cnstcoxi_dn7 = assign22480_e17255_d_n7;
        locals.var_cnstcoxi_dn8 = assign22480_e17255_d_n8;
        locals.var_cnstcoxi_dn9 = assign22480_e17255_d_n9;
        locals.var_cnstcoxi_dn10 = assign22480_e17255_d_n10;
        locals.var_cnstcoxi_dn11 = assign22480_e17255_d_n11;
        locals.var_cnstcoxi_dn14 = assign22480_e17255_d_n14;
        locals.var_cnstcoxi_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_60(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign22490_e17266, assign22490_e17266_d_n0, assign22490_e17266_d_n2, assign22490_e17266_d_n4, assign22490_e17266_d_n5, assign22490_e17266_d_n6, assign22490_e17266_d_n7, assign22490_e17266_d_n8, assign22490_e17266_d_n9, assign22490_e17266_d_n10, assign22490_e17266_d_n11, assign22490_e17266_d_n14,) = {
    if (locals.var_guard430 == 0.0) {
        let assign22490_e17260: f64 = (locals.var_vgs - locals.var_vbs);
        let assign22490_e17262: f64 = (assign22490_e17260 - locals.var_vthq);
        let assign22490_e17264: f64 = (assign22490_e17262 + p.p236);
        (assign22490_e17264, (-locals.var_vthq_dn0), (-locals.var_vthq_dn2), (-locals.var_vthq_dn4), (-locals.var_vthq_dn5), ((locals.var_vgs_dn6 - locals.var_vbs_dn6) - locals.var_vthq_dn6), (locals.var_vgs_dn7 - locals.var_vthq_dn7), ((locals.var_vgs_dn8 - locals.var_vbs_dn8) - locals.var_vthq_dn8), ((-locals.var_vbs_dn9) - locals.var_vthq_dn9), (-locals.var_vthq_dn10), (-locals.var_vthq_dn11), (-locals.var_vthq_dn14),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign22490_e17266;
        locals.var_t5_dn0 = assign22490_e17266_d_n0;
        locals.var_t5_dn2 = assign22490_e17266_d_n2;
        locals.var_t5_dn4 = assign22490_e17266_d_n4;
        locals.var_t5_dn5 = assign22490_e17266_d_n5;
        locals.var_t5_dn6 = assign22490_e17266_d_n6;
        locals.var_t5_dn7 = assign22490_e17266_d_n7;
        locals.var_t5_dn8 = assign22490_e17266_d_n8;
        locals.var_t5_dn9 = assign22490_e17266_d_n9;
        locals.var_t5_dn10 = assign22490_e17266_d_n10;
        locals.var_t5_dn11 = assign22490_e17266_d_n11;
        locals.var_t5_dn14 = assign22490_e17266_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign22500_e17284, assign22500_e17284_d_n0, assign22500_e17284_d_n2, assign22500_e17284_d_n4, assign22500_e17284_d_n5, assign22500_e17284_d_n6, assign22500_e17284_d_n7, assign22500_e17284_d_n8, assign22500_e17284_d_n9, assign22500_e17284_d_n10, assign22500_e17284_d_n11, assign22500_e17284_d_n14,) = {
    if (locals.var_guard430 == 0.0) {
        let assign22500_e17271: f64 = (locals.var_t5 * locals.var_t5);
        let assign22500_e17275: f64 = (1e-9 * 0.01);
        let assign22500_e17276: f64 = (4.0 * assign22500_e17275);
        let assign22500_e17279: f64 = (1e-9 * 0.01);
        let assign22500_e17280: f64 = (assign22500_e17276 * assign22500_e17279);
        let assign22500_e17281: f64 = (assign22500_e17271 + assign22500_e17280);
        let assign22500_e17282: f64 = (assign22500_e17281).sqrt();
        (assign22500_e17282, (((locals.var_t5_dn0 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn0)) / (2.0 * assign22500_e17282)), (((locals.var_t5_dn2 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn2)) / (2.0 * assign22500_e17282)), (((locals.var_t5_dn4 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn4)) / (2.0 * assign22500_e17282)), (((locals.var_t5_dn5 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn5)) / (2.0 * assign22500_e17282)), (((locals.var_t5_dn6 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn6)) / (2.0 * assign22500_e17282)), (((locals.var_t5_dn7 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn7)) / (2.0 * assign22500_e17282)), (((locals.var_t5_dn8 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn8)) / (2.0 * assign22500_e17282)), (((locals.var_t5_dn9 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn9)) / (2.0 * assign22500_e17282)), (((locals.var_t5_dn10 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn10)) / (2.0 * assign22500_e17282)), (((locals.var_t5_dn11 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn11)) / (2.0 * assign22500_e17282)), (((locals.var_t5_dn14 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn14)) / (2.0 * assign22500_e17282)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign22500_e17284;
        locals.var_tmf2_dn0 = assign22500_e17284_d_n0;
        locals.var_tmf2_dn2 = assign22500_e17284_d_n2;
        locals.var_tmf2_dn4 = assign22500_e17284_d_n4;
        locals.var_tmf2_dn5 = assign22500_e17284_d_n5;
        locals.var_tmf2_dn6 = assign22500_e17284_d_n6;
        locals.var_tmf2_dn7 = assign22500_e17284_d_n7;
        locals.var_tmf2_dn8 = assign22500_e17284_d_n8;
        locals.var_tmf2_dn9 = assign22500_e17284_d_n9;
        locals.var_tmf2_dn10 = assign22500_e17284_d_n10;
        locals.var_tmf2_dn11 = assign22500_e17284_d_n11;
        locals.var_tmf2_dn14 = assign22500_e17284_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign22510_e17295, assign22510_e17295_d_n0, assign22510_e17295_d_n2, assign22510_e17295_d_n4, assign22510_e17295_d_n5, assign22510_e17295_d_n6, assign22510_e17295_d_n7, assign22510_e17295_d_n8, assign22510_e17295_d_n9, assign22510_e17295_d_n10, assign22510_e17295_d_n11, assign22510_e17295_d_n14,) = {
    if (locals.var_guard430 == 0.0) {
        let assign22510_e17291: f64 = (locals.var_t5 / locals.var_tmf2);
        let assign22510_e17292: f64 = (1.0 + assign22510_e17291);
        let assign22510_e17293: f64 = (0.5 * assign22510_e17292);
        (assign22510_e17293, (0.5 * (((locals.var_t5_dn0 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn2 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn4 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn5 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn6 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn7 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn8 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn9 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn10 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn11 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn14 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign22510_e17295;
        locals.var_t3_dn0 = assign22510_e17295_d_n0;
        locals.var_t3_dn2 = assign22510_e17295_d_n2;
        locals.var_t3_dn4 = assign22510_e17295_d_n4;
        locals.var_t3_dn5 = assign22510_e17295_d_n5;
        locals.var_t3_dn6 = assign22510_e17295_d_n6;
        locals.var_t3_dn7 = assign22510_e17295_d_n7;
        locals.var_t3_dn8 = assign22510_e17295_d_n8;
        locals.var_t3_dn9 = assign22510_e17295_d_n9;
        locals.var_t3_dn10 = assign22510_e17295_d_n10;
        locals.var_t3_dn11 = assign22510_e17295_d_n11;
        locals.var_t3_dn14 = assign22510_e17295_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign22520_e17304, assign22520_e17304_d_n0, assign22520_e17304_d_n2, assign22520_e17304_d_n4, assign22520_e17304_d_n5, assign22520_e17304_d_n6, assign22520_e17304_d_n7, assign22520_e17304_d_n8, assign22520_e17304_d_n9, assign22520_e17304_d_n10, assign22520_e17304_d_n11, assign22520_e17304_d_n14,) = {
    if (locals.var_guard430 == 0.0) {
        let assign22520_e17301: f64 = (locals.var_t5 + locals.var_tmf2);
        let assign22520_e17302: f64 = (0.5 * assign22520_e17301);
        (assign22520_e17302, (0.5 * (locals.var_t5_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t5_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t5_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t5_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t5_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t5_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t5_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t5_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t5_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t5_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t5_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign22520_e17304;
        locals.var_t2_dn0 = assign22520_e17304_d_n0;
        locals.var_t2_dn2 = assign22520_e17304_d_n2;
        locals.var_t2_dn4 = assign22520_e17304_d_n4;
        locals.var_t2_dn5 = assign22520_e17304_d_n5;
        locals.var_t2_dn6 = assign22520_e17304_d_n6;
        locals.var_t2_dn7 = assign22520_e17304_d_n7;
        locals.var_t2_dn8 = assign22520_e17304_d_n8;
        locals.var_t2_dn9 = assign22520_e17304_d_n9;
        locals.var_t2_dn10 = assign22520_e17304_d_n10;
        locals.var_t2_dn11 = assign22520_e17304_d_n11;
        locals.var_t2_dn14 = assign22520_e17304_d_n14;
        locals.var_t2_rv = 0.0;

        let assign22530_e17307: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard431 = assign22530_e17307;
        locals.var_guard431_rv = 0.0;

        let (assign22540_e17314, assign22540_e17314_d_n0, assign22540_e17314_d_n2, assign22540_e17314_d_n4, assign22540_e17314_d_n5, assign22540_e17314_d_n6, assign22540_e17314_d_n7, assign22540_e17314_d_n8, assign22540_e17314_d_n9, assign22540_e17314_d_n10, assign22540_e17314_d_n11, assign22540_e17314_d_n14,) = {
    if ((locals.var_guard430 == 0.0) && (locals.var_guard431 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign22540_e17314;
        locals.var_t2_dn0 = assign22540_e17314_d_n0;
        locals.var_t2_dn2 = assign22540_e17314_d_n2;
        locals.var_t2_dn4 = assign22540_e17314_d_n4;
        locals.var_t2_dn5 = assign22540_e17314_d_n5;
        locals.var_t2_dn6 = assign22540_e17314_d_n6;
        locals.var_t2_dn7 = assign22540_e17314_d_n7;
        locals.var_t2_dn8 = assign22540_e17314_d_n8;
        locals.var_t2_dn9 = assign22540_e17314_d_n9;
        locals.var_t2_dn10 = assign22540_e17314_d_n10;
        locals.var_t2_dn11 = assign22540_e17314_d_n11;
        locals.var_t2_dn14 = assign22540_e17314_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign22550_e17321, assign22550_e17321_d_n0, assign22550_e17321_d_n2, assign22550_e17321_d_n4, assign22550_e17321_d_n5, assign22550_e17321_d_n6, assign22550_e17321_d_n7, assign22550_e17321_d_n8, assign22550_e17321_d_n9, assign22550_e17321_d_n10, assign22550_e17321_d_n11, assign22550_e17321_d_n14,) = {
    if ((locals.var_guard430 == 0.0) && (locals.var_guard431 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign22550_e17321;
        locals.var_t3_dn0 = assign22550_e17321_d_n0;
        locals.var_t3_dn2 = assign22550_e17321_d_n2;
        locals.var_t3_dn4 = assign22550_e17321_d_n4;
        locals.var_t3_dn5 = assign22550_e17321_d_n5;
        locals.var_t3_dn6 = assign22550_e17321_d_n6;
        locals.var_t3_dn7 = assign22550_e17321_d_n7;
        locals.var_t3_dn8 = assign22550_e17321_d_n8;
        locals.var_t3_dn9 = assign22550_e17321_d_n9;
        locals.var_t3_dn10 = assign22550_e17321_d_n10;
        locals.var_t3_dn11 = assign22550_e17321_d_n11;
        locals.var_t3_dn14 = assign22550_e17321_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign22560_e17328, assign22560_e17328_d_n0, assign22560_e17328_d_n2, assign22560_e17328_d_n4, assign22560_e17328_d_n5, assign22560_e17328_d_n6, assign22560_e17328_d_n7, assign22560_e17328_d_n8, assign22560_e17328_d_n9, assign22560_e17328_d_n10, assign22560_e17328_d_n11, assign22560_e17328_d_n14,) = {
    if (locals.var_guard430 == 0.0) {
        let assign22560_e17326: f64 = (locals.var_t2 + 1e-25);
        (assign22560_e17326, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign22560_e17328;
        locals.var_t2_dn0 = assign22560_e17328_d_n0;
        locals.var_t2_dn2 = assign22560_e17328_d_n2;
        locals.var_t2_dn4 = assign22560_e17328_d_n4;
        locals.var_t2_dn5 = assign22560_e17328_d_n5;
        locals.var_t2_dn6 = assign22560_e17328_d_n6;
        locals.var_t2_dn7 = assign22560_e17328_d_n7;
        locals.var_t2_dn8 = assign22560_e17328_d_n8;
        locals.var_t2_dn9 = assign22560_e17328_d_n9;
        locals.var_t2_dn10 = assign22560_e17328_d_n10;
        locals.var_t2_dn11 = assign22560_e17328_d_n11;
        locals.var_t2_dn14 = assign22560_e17328_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign22570_e17335, assign22570_e17335_d_n0, assign22570_e17335_d_n2, assign22570_e17335_d_n4, assign22570_e17335_d_n5, assign22570_e17335_d_n6, assign22570_e17335_d_n7, assign22570_e17335_d_n8, assign22570_e17335_d_n9, assign22570_e17335_d_n10, assign22570_e17335_d_n11, assign22570_e17335_d_n14,) = {
    if (locals.var_guard430 == 0.0) {
        let assign22570_e17333: f64 = (1.0 / locals.var_t2);
        (assign22570_e17333, (-(locals.var_t2_dn0 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn2 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn9 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn10 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn11 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn14 / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign22570_e17335;
        locals.var_t3_dn0 = assign22570_e17335_d_n0;
        locals.var_t3_dn2 = assign22570_e17335_d_n2;
        locals.var_t3_dn4 = assign22570_e17335_d_n4;
        locals.var_t3_dn5 = assign22570_e17335_d_n5;
        locals.var_t3_dn6 = assign22570_e17335_d_n6;
        locals.var_t3_dn7 = assign22570_e17335_d_n7;
        locals.var_t3_dn8 = assign22570_e17335_d_n8;
        locals.var_t3_dn9 = assign22570_e17335_d_n9;
        locals.var_t3_dn10 = assign22570_e17335_d_n10;
        locals.var_t3_dn11 = assign22570_e17335_d_n11;
        locals.var_t3_dn14 = assign22570_e17335_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign22580_e17345, assign22580_e17345_d_n0, assign22580_e17345_d_n2, assign22580_e17345_d_n4, assign22580_e17345_d_n5, assign22580_e17345_d_n6, assign22580_e17345_d_n7, assign22580_e17345_d_n8, assign22580_e17345_d_n9, assign22580_e17345_d_n10, assign22580_e17345_d_n11, assign22580_e17345_d_n14,) = {
    if (locals.var_guard430 == 0.0) {
        let assign22580_e17339: f64 = (-1.0);
        let assign22580_e17342: f64 = (locals.var_t2 * locals.var_t2);
        let assign22580_e17343: f64 = (assign22580_e17339 / assign22580_e17342);
        (assign22580_e17343, (-((assign22580_e17339 * ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0))) / (assign22580_e17342 * assign22580_e17342))), (-((assign22580_e17339 * ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2))) / (assign22580_e17342 * assign22580_e17342))), (-((assign22580_e17339 * ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4))) / (assign22580_e17342 * assign22580_e17342))), (-((assign22580_e17339 * ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5))) / (assign22580_e17342 * assign22580_e17342))), (-((assign22580_e17339 * ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6))) / (assign22580_e17342 * assign22580_e17342))), (-((assign22580_e17339 * ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7))) / (assign22580_e17342 * assign22580_e17342))), (-((assign22580_e17339 * ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8))) / (assign22580_e17342 * assign22580_e17342))), (-((assign22580_e17339 * ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9))) / (assign22580_e17342 * assign22580_e17342))), (-((assign22580_e17339 * ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10))) / (assign22580_e17342 * assign22580_e17342))), (-((assign22580_e17339 * ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11))) / (assign22580_e17342 * assign22580_e17342))), (-((assign22580_e17339 * ((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14))) / (assign22580_e17342 * assign22580_e17342))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign22580_e17345;
        locals.var_t7_dn0 = assign22580_e17345_d_n0;
        locals.var_t7_dn2 = assign22580_e17345_d_n2;
        locals.var_t7_dn4 = assign22580_e17345_d_n4;
        locals.var_t7_dn5 = assign22580_e17345_d_n5;
        locals.var_t7_dn6 = assign22580_e17345_d_n6;
        locals.var_t7_dn7 = assign22580_e17345_d_n7;
        locals.var_t7_dn8 = assign22580_e17345_d_n8;
        locals.var_t7_dn9 = assign22580_e17345_d_n9;
        locals.var_t7_dn10 = assign22580_e17345_d_n10;
        locals.var_t7_dn11 = assign22580_e17345_d_n11;
        locals.var_t7_dn14 = assign22580_e17345_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign22590_e17353, assign22590_e17353_d_n0, assign22590_e17353_d_n2, assign22590_e17353_d_n4, assign22590_e17353_d_n5, assign22590_e17353_d_n6, assign22590_e17353_d_n7, assign22590_e17353_d_n8, assign22590_e17353_d_n9, assign22590_e17353_d_n10, assign22590_e17353_d_n11, assign22590_e17353_d_n14,) = {
    if (locals.var_guard430 == 0.0) {
        let assign22590_e17350: f64 = (locals.var_vthq).abs();
        let assign22590_e17351: f64 = (2.0 * assign22590_e17350);
        (assign22590_e17351, (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn0 } else { (-locals.var_vthq_dn0) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn2 } else { (-locals.var_vthq_dn2) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn4 } else { (-locals.var_vthq_dn4) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn5 } else { (-locals.var_vthq_dn5) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn6 } else { (-locals.var_vthq_dn6) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn7 } else { (-locals.var_vthq_dn7) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn8 } else { (-locals.var_vthq_dn8) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn9 } else { (-locals.var_vthq_dn9) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn10 } else { (-locals.var_vthq_dn10) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn11 } else { (-locals.var_vthq_dn11) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn14 } else { (-locals.var_vthq_dn14) }),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign22590_e17353;
        locals.var_t4_dn0 = assign22590_e17353_d_n0;
        locals.var_t4_dn2 = assign22590_e17353_d_n2;
        locals.var_t4_dn4 = assign22590_e17353_d_n4;
        locals.var_t4_dn5 = assign22590_e17353_d_n5;
        locals.var_t4_dn6 = assign22590_e17353_d_n6;
        locals.var_t4_dn7 = assign22590_e17353_d_n7;
        locals.var_t4_dn8 = assign22590_e17353_d_n8;
        locals.var_t4_dn9 = assign22590_e17353_d_n9;
        locals.var_t4_dn10 = assign22590_e17353_d_n10;
        locals.var_t4_dn11 = assign22590_e17353_d_n11;
        locals.var_t4_dn14 = assign22590_e17353_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign22600_e17362, assign22600_e17362_d_n0, assign22600_e17362_d_n2, assign22600_e17362_d_n4, assign22600_e17362_d_n5, assign22600_e17362_d_n6, assign22600_e17362_d_n7, assign22600_e17362_d_n8, assign22600_e17362_d_n9, assign22600_e17362_d_n10, assign22600_e17362_d_n11, assign22600_e17362_d_n14,) = {
    if (locals.var_guard430 == 0.0) {
        let assign22600_e17358: f64 = (locals.var_t5 - locals.var_vgs);
        let assign22600_e17360: f64 = (assign22600_e17358 + locals.var_vfb);
        (assign22600_e17360, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, (locals.var_t5_dn6 - locals.var_vgs_dn6), (locals.var_t5_dn7 - locals.var_vgs_dn7), (locals.var_t5_dn8 - locals.var_vgs_dn8), locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign22600_e17362;
        locals.var_t6_dn0 = assign22600_e17362_d_n0;
        locals.var_t6_dn2 = assign22600_e17362_d_n2;
        locals.var_t6_dn4 = assign22600_e17362_d_n4;
        locals.var_t6_dn5 = assign22600_e17362_d_n5;
        locals.var_t6_dn6 = assign22600_e17362_d_n6;
        locals.var_t6_dn7 = assign22600_e17362_d_n7;
        locals.var_t6_dn8 = assign22600_e17362_d_n8;
        locals.var_t6_dn9 = assign22600_e17362_d_n9;
        locals.var_t6_dn10 = assign22600_e17362_d_n10;
        locals.var_t6_dn11 = assign22600_e17362_d_n11;
        locals.var_t6_dn14 = assign22600_e17362_d_n14;
        locals.var_t6_rv = 0.0;

        let assign22610_e17365: f64 = if locals.var_t6 > locals.var_t4 { 1.0 } else { 0.0 };
        locals.var_guard432 = assign22610_e17365;
        locals.var_guard432_rv = 0.0;

        let (assign22620_e17372, assign22620_e17372_d_n0, assign22620_e17372_d_n2, assign22620_e17372_d_n4, assign22620_e17372_d_n5, assign22620_e17372_d_n6, assign22620_e17372_d_n7, assign22620_e17372_d_n8, assign22620_e17372_d_n9, assign22620_e17372_d_n10, assign22620_e17372_d_n11, assign22620_e17372_d_n14,) = {
    if ((locals.var_guard430 == 0.0) && (locals.var_guard432 != 0.0)) {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign22620_e17372;
        locals.var_t4_dn0 = assign22620_e17372_d_n0;
        locals.var_t4_dn2 = assign22620_e17372_d_n2;
        locals.var_t4_dn4 = assign22620_e17372_d_n4;
        locals.var_t4_dn5 = assign22620_e17372_d_n5;
        locals.var_t4_dn6 = assign22620_e17372_d_n6;
        locals.var_t4_dn7 = assign22620_e17372_d_n7;
        locals.var_t4_dn8 = assign22620_e17372_d_n8;
        locals.var_t4_dn9 = assign22620_e17372_d_n9;
        locals.var_t4_dn10 = assign22620_e17372_d_n10;
        locals.var_t4_dn11 = assign22620_e17372_d_n11;
        locals.var_t4_dn14 = assign22620_e17372_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign22630_e17385, assign22630_e17385_d_n0, assign22630_e17385_d_n2, assign22630_e17385_d_n4, assign22630_e17385_d_n5, assign22630_e17385_d_n6, assign22630_e17385_d_n7, assign22630_e17385_d_n8, assign22630_e17385_d_n9, assign22630_e17385_d_n10, assign22630_e17385_d_n11, assign22630_e17385_d_n14,) = {
    if (locals.var_guard430 == 0.0) {
        let assign22630_e17377: f64 = (1.0 / locals.var_t4);
        let assign22630_e17379: f64 = (assign22630_e17377 - locals.var_t3);
        let assign22630_e17382: f64 = (1e-9 * 0.01);
        let assign22630_e17383: f64 = (assign22630_e17379 - assign22630_e17382);
        (assign22630_e17383, ((-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn0), ((-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn2), ((-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn4), ((-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn5), ((-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn6), ((-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn7), ((-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn8), ((-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn9), ((-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn10), ((-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn11), ((-(locals.var_t4_dn14 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign22630_e17385;
        locals.var_tmf1_dn0 = assign22630_e17385_d_n0;
        locals.var_tmf1_dn2 = assign22630_e17385_d_n2;
        locals.var_tmf1_dn4 = assign22630_e17385_d_n4;
        locals.var_tmf1_dn5 = assign22630_e17385_d_n5;
        locals.var_tmf1_dn6 = assign22630_e17385_d_n6;
        locals.var_tmf1_dn7 = assign22630_e17385_d_n7;
        locals.var_tmf1_dn8 = assign22630_e17385_d_n8;
        locals.var_tmf1_dn9 = assign22630_e17385_d_n9;
        locals.var_tmf1_dn10 = assign22630_e17385_d_n10;
        locals.var_tmf1_dn11 = assign22630_e17385_d_n11;
        locals.var_tmf1_dn14 = assign22630_e17385_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign22640_e17398, assign22640_e17398_d_n0, assign22640_e17398_d_n2, assign22640_e17398_d_n4, assign22640_e17398_d_n5, assign22640_e17398_d_n6, assign22640_e17398_d_n7, assign22640_e17398_d_n8, assign22640_e17398_d_n9, assign22640_e17398_d_n10, assign22640_e17398_d_n11, assign22640_e17398_d_n14,) = {
    if (locals.var_guard430 == 0.0) {
        let assign22640_e17391: f64 = (1.0 / locals.var_t4);
        let assign22640_e17392: f64 = (4.0 * assign22640_e17391);
        let assign22640_e17395: f64 = (1e-9 * 0.01);
        let assign22640_e17396: f64 = (assign22640_e17392 * assign22640_e17395);
        (assign22640_e17396, ((4.0 * (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4)))) * assign22640_e17395), ((4.0 * (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4)))) * assign22640_e17395), ((4.0 * (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4)))) * assign22640_e17395), ((4.0 * (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4)))) * assign22640_e17395), ((4.0 * (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4)))) * assign22640_e17395), ((4.0 * (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4)))) * assign22640_e17395), ((4.0 * (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4)))) * assign22640_e17395), ((4.0 * (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4)))) * assign22640_e17395), ((4.0 * (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4)))) * assign22640_e17395), ((4.0 * (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4)))) * assign22640_e17395), ((4.0 * (-(locals.var_t4_dn14 / (locals.var_t4 * locals.var_t4)))) * assign22640_e17395),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign22640_e17398;
        locals.var_tmf2_dn0 = assign22640_e17398_d_n0;
        locals.var_tmf2_dn2 = assign22640_e17398_d_n2;
        locals.var_tmf2_dn4 = assign22640_e17398_d_n4;
        locals.var_tmf2_dn5 = assign22640_e17398_d_n5;
        locals.var_tmf2_dn6 = assign22640_e17398_d_n6;
        locals.var_tmf2_dn7 = assign22640_e17398_d_n7;
        locals.var_tmf2_dn8 = assign22640_e17398_d_n8;
        locals.var_tmf2_dn9 = assign22640_e17398_d_n9;
        locals.var_tmf2_dn10 = assign22640_e17398_d_n10;
        locals.var_tmf2_dn11 = assign22640_e17398_d_n11;
        locals.var_tmf2_dn14 = assign22640_e17398_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign22650_e17409, assign22650_e17409_d_n0, assign22650_e17409_d_n2, assign22650_e17409_d_n4, assign22650_e17409_d_n5, assign22650_e17409_d_n6, assign22650_e17409_d_n7, assign22650_e17409_d_n8, assign22650_e17409_d_n9, assign22650_e17409_d_n10, assign22650_e17409_d_n11, assign22650_e17409_d_n14,) = {
    if (locals.var_guard430 == 0.0) {
        let (assign22650_e17407, assign22650_e17407_d_n0, assign22650_e17407_d_n2, assign22650_e17407_d_n4, assign22650_e17407_d_n5, assign22650_e17407_d_n6, assign22650_e17407_d_n7, assign22650_e17407_d_n8, assign22650_e17407_d_n9, assign22650_e17407_d_n10, assign22650_e17407_d_n11, assign22650_e17407_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign22650_e17406: f64 = (-locals.var_tmf2);
                (assign22650_e17406, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign22650_e17407, assign22650_e17407_d_n0, assign22650_e17407_d_n2, assign22650_e17407_d_n4, assign22650_e17407_d_n5, assign22650_e17407_d_n6, assign22650_e17407_d_n7, assign22650_e17407_d_n8, assign22650_e17407_d_n9, assign22650_e17407_d_n10, assign22650_e17407_d_n11, assign22650_e17407_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign22650_e17409;
        locals.var_tmf2_dn0 = assign22650_e17409_d_n0;
        locals.var_tmf2_dn2 = assign22650_e17409_d_n2;
        locals.var_tmf2_dn4 = assign22650_e17409_d_n4;
        locals.var_tmf2_dn5 = assign22650_e17409_d_n5;
        locals.var_tmf2_dn6 = assign22650_e17409_d_n6;
        locals.var_tmf2_dn7 = assign22650_e17409_d_n7;
        locals.var_tmf2_dn8 = assign22650_e17409_d_n8;
        locals.var_tmf2_dn9 = assign22650_e17409_d_n9;
        locals.var_tmf2_dn10 = assign22650_e17409_d_n10;
        locals.var_tmf2_dn11 = assign22650_e17409_d_n11;
        locals.var_tmf2_dn14 = assign22650_e17409_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign22660_e17419, assign22660_e17419_d_n0, assign22660_e17419_d_n2, assign22660_e17419_d_n4, assign22660_e17419_d_n5, assign22660_e17419_d_n6, assign22660_e17419_d_n7, assign22660_e17419_d_n8, assign22660_e17419_d_n9, assign22660_e17419_d_n10, assign22660_e17419_d_n11, assign22660_e17419_d_n14,) = {
    if (locals.var_guard430 == 0.0) {
        let assign22660_e17414: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign22660_e17416: f64 = (assign22660_e17414 + locals.var_tmf2);
        let assign22660_e17417: f64 = (assign22660_e17416).sqrt();
        (assign22660_e17417, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign22660_e17417)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign22660_e17417)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign22660_e17417)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign22660_e17417)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign22660_e17417)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign22660_e17417)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign22660_e17417)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign22660_e17417)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign22660_e17417)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign22660_e17417)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign22660_e17417)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign22660_e17419;
        locals.var_tmf2_dn0 = assign22660_e17419_d_n0;
        locals.var_tmf2_dn2 = assign22660_e17419_d_n2;
        locals.var_tmf2_dn4 = assign22660_e17419_d_n4;
        locals.var_tmf2_dn5 = assign22660_e17419_d_n5;
        locals.var_tmf2_dn6 = assign22660_e17419_d_n6;
        locals.var_tmf2_dn7 = assign22660_e17419_d_n7;
        locals.var_tmf2_dn8 = assign22660_e17419_d_n8;
        locals.var_tmf2_dn9 = assign22660_e17419_d_n9;
        locals.var_tmf2_dn10 = assign22660_e17419_d_n10;
        locals.var_tmf2_dn11 = assign22660_e17419_d_n11;
        locals.var_tmf2_dn14 = assign22660_e17419_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign22670_e17430, assign22670_e17430_d_n0, assign22670_e17430_d_n2, assign22670_e17430_d_n4, assign22670_e17430_d_n5, assign22670_e17430_d_n6, assign22670_e17430_d_n7, assign22670_e17430_d_n8, assign22670_e17430_d_n9, assign22670_e17430_d_n10, assign22670_e17430_d_n11, assign22670_e17430_d_n14,) = {
    if (locals.var_guard430 == 0.0) {
        let assign22670_e17426: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign22670_e17427: f64 = (1.0 + assign22670_e17426);
        let assign22670_e17428: f64 = (0.5 * assign22670_e17427);
        (assign22670_e17428, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign22670_e17430;
        locals.var_t6_dn0 = assign22670_e17430_d_n0;
        locals.var_t6_dn2 = assign22670_e17430_d_n2;
        locals.var_t6_dn4 = assign22670_e17430_d_n4;
        locals.var_t6_dn5 = assign22670_e17430_d_n5;
        locals.var_t6_dn6 = assign22670_e17430_d_n6;
        locals.var_t6_dn7 = assign22670_e17430_d_n7;
        locals.var_t6_dn8 = assign22670_e17430_d_n8;
        locals.var_t6_dn9 = assign22670_e17430_d_n9;
        locals.var_t6_dn10 = assign22670_e17430_d_n10;
        locals.var_t6_dn11 = assign22670_e17430_d_n11;
        locals.var_t6_dn14 = assign22670_e17430_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign22680_e17443, assign22680_e17443_d_n0, assign22680_e17443_d_n2, assign22680_e17443_d_n4, assign22680_e17443_d_n5, assign22680_e17443_d_n6, assign22680_e17443_d_n7, assign22680_e17443_d_n8, assign22680_e17443_d_n9, assign22680_e17443_d_n10, assign22680_e17443_d_n11, assign22680_e17443_d_n14,) = {
    if (locals.var_guard430 == 0.0) {
        let assign22680_e17435: f64 = (1.0 / locals.var_t4);
        let assign22680_e17439: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign22680_e17440: f64 = (0.5 * assign22680_e17439);
        let assign22680_e17441: f64 = (assign22680_e17435 - assign22680_e17440);
        (assign22680_e17441, ((-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), ((-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), ((-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), ((-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), ((-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((-(locals.var_t4_dn14 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign22680_e17443;
        locals.var_t2_dn0 = assign22680_e17443_d_n0;
        locals.var_t2_dn2 = assign22680_e17443_d_n2;
        locals.var_t2_dn4 = assign22680_e17443_d_n4;
        locals.var_t2_dn5 = assign22680_e17443_d_n5;
        locals.var_t2_dn6 = assign22680_e17443_d_n6;
        locals.var_t2_dn7 = assign22680_e17443_d_n7;
        locals.var_t2_dn8 = assign22680_e17443_d_n8;
        locals.var_t2_dn9 = assign22680_e17443_d_n9;
        locals.var_t2_dn10 = assign22680_e17443_d_n10;
        locals.var_t2_dn11 = assign22680_e17443_d_n11;
        locals.var_t2_dn14 = assign22680_e17443_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign22690_e17452, assign22690_e17452_d_n0, assign22690_e17452_d_n2, assign22690_e17452_d_n4, assign22690_e17452_d_n5, assign22690_e17452_d_n6, assign22690_e17452_d_n7, assign22690_e17452_d_n8, assign22690_e17452_d_n9, assign22690_e17452_d_n10, assign22690_e17452_d_n11, assign22690_e17452_d_n14,) = {
    if (locals.var_guard430 == 0.0) {
        let assign22690_e17448: f64 = (p.p235 * locals.var_t2);
        let assign22690_e17450: f64 = (assign22690_e17448 + p.p237);
        (assign22690_e17450, (p.p235 * locals.var_t2_dn0), (p.p235 * locals.var_t2_dn2), (p.p235 * locals.var_t2_dn4), (p.p235 * locals.var_t2_dn5), (p.p235 * locals.var_t2_dn6), (p.p235 * locals.var_t2_dn7), (p.p235 * locals.var_t2_dn8), (p.p235 * locals.var_t2_dn9), (p.p235 * locals.var_t2_dn10), (p.p235 * locals.var_t2_dn11), (p.p235 * locals.var_t2_dn14),)
    } else {
        (locals.var_dtox, locals.var_dtox_dn0, locals.var_dtox_dn2, locals.var_dtox_dn4, locals.var_dtox_dn5, locals.var_dtox_dn6, locals.var_dtox_dn7, locals.var_dtox_dn8, locals.var_dtox_dn9, locals.var_dtox_dn10, locals.var_dtox_dn11, locals.var_dtox_dn14,)
    }
};
        locals.var_dtox = assign22690_e17452;
        locals.var_dtox_dn0 = assign22690_e17452_d_n0;
        locals.var_dtox_dn2 = assign22690_e17452_d_n2;
        locals.var_dtox_dn4 = assign22690_e17452_d_n4;
        locals.var_dtox_dn5 = assign22690_e17452_d_n5;
        locals.var_dtox_dn6 = assign22690_e17452_d_n6;
        locals.var_dtox_dn7 = assign22690_e17452_d_n7;
        locals.var_dtox_dn8 = assign22690_e17452_d_n8;
        locals.var_dtox_dn9 = assign22690_e17452_d_n9;
        locals.var_dtox_dn10 = assign22690_e17452_d_n10;
        locals.var_dtox_dn11 = assign22690_e17452_d_n11;
        locals.var_dtox_dn14 = assign22690_e17452_d_n14;
        locals.var_dtox_rv = 0.0;

        let (assign22700_e17457, assign22700_e17457_d_n0, assign22700_e17457_d_n2, assign22700_e17457_d_n4, assign22700_e17457_d_n5, assign22700_e17457_d_n6, assign22700_e17457_d_n7, assign22700_e17457_d_n8, assign22700_e17457_d_n9, assign22700_e17457_d_n10, assign22700_e17457_d_n11, assign22700_e17457_d_n14,) = {
    if (locals.var_guard430 == 0.0) {
        (p.p235, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign22700_e17457;
        locals.var_t7_dn0 = assign22700_e17457_d_n0;
        locals.var_t7_dn2 = assign22700_e17457_d_n2;
        locals.var_t7_dn4 = assign22700_e17457_d_n4;
        locals.var_t7_dn5 = assign22700_e17457_d_n5;
        locals.var_t7_dn6 = assign22700_e17457_d_n6;
        locals.var_t7_dn7 = assign22700_e17457_d_n7;
        locals.var_t7_dn8 = assign22700_e17457_d_n8;
        locals.var_t7_dn9 = assign22700_e17457_d_n9;
        locals.var_t7_dn10 = assign22700_e17457_d_n10;
        locals.var_t7_dn11 = assign22700_e17457_d_n11;
        locals.var_t7_dn14 = assign22700_e17457_d_n14;
        locals.var_t7_rv = 0.0;

        let assign22710_e17460: f64 = (locals.var_dtox * 1000000000000.0);
        let assign22710_e17462: f64 = if assign22710_e17460 < locals.var_tox0 { 1.0 } else { 0.0 };
        locals.var_guard433 = assign22710_e17462;
        locals.var_guard433_rv = 0.0;

        let (assign22720_e17469, assign22720_e17469_d_n0, assign22720_e17469_d_n2, assign22720_e17469_d_n4, assign22720_e17469_d_n5, assign22720_e17469_d_n6, assign22720_e17469_d_n7, assign22720_e17469_d_n8, assign22720_e17469_d_n9, assign22720_e17469_d_n10, assign22720_e17469_d_n11, assign22720_e17469_d_n14,) = {
    if ((locals.var_guard430 == 0.0) && (locals.var_guard433 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dtox, locals.var_dtox_dn0, locals.var_dtox_dn2, locals.var_dtox_dn4, locals.var_dtox_dn5, locals.var_dtox_dn6, locals.var_dtox_dn7, locals.var_dtox_dn8, locals.var_dtox_dn9, locals.var_dtox_dn10, locals.var_dtox_dn11, locals.var_dtox_dn14,)
    }
};
        locals.var_dtox = assign22720_e17469;
        locals.var_dtox_dn0 = assign22720_e17469_d_n0;
        locals.var_dtox_dn2 = assign22720_e17469_d_n2;
        locals.var_dtox_dn4 = assign22720_e17469_d_n4;
        locals.var_dtox_dn5 = assign22720_e17469_d_n5;
        locals.var_dtox_dn6 = assign22720_e17469_d_n6;
        locals.var_dtox_dn7 = assign22720_e17469_d_n7;
        locals.var_dtox_dn8 = assign22720_e17469_d_n8;
        locals.var_dtox_dn9 = assign22720_e17469_d_n9;
        locals.var_dtox_dn10 = assign22720_e17469_d_n10;
        locals.var_dtox_dn11 = assign22720_e17469_d_n11;
        locals.var_dtox_dn14 = assign22720_e17469_d_n14;
        locals.var_dtox_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_61(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign22730_e17476,) = {
    if ((locals.var_guard430 == 0.0) && (locals.var_guard433 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_qme,)
    }
};
        locals.var_flg_qme = assign22730_e17476;
        locals.var_flg_qme_rv = 0.0;

        let (assign22740_e17483, assign22740_e17483_d_n0, assign22740_e17483_d_n2, assign22740_e17483_d_n4, assign22740_e17483_d_n5, assign22740_e17483_d_n6, assign22740_e17483_d_n7, assign22740_e17483_d_n8, assign22740_e17483_d_n9, assign22740_e17483_d_n10, assign22740_e17483_d_n11, assign22740_e17483_d_n14,) = {
    if (locals.var_guard430 == 0.0) {
        let assign22740_e17481: f64 = (locals.var_tox0 + locals.var_dtox);
        (assign22740_e17481, locals.var_dtox_dn0, locals.var_dtox_dn2, locals.var_dtox_dn4, locals.var_dtox_dn5, locals.var_dtox_dn6, locals.var_dtox_dn7, locals.var_dtox_dn8, locals.var_dtox_dn9, locals.var_dtox_dn10, locals.var_dtox_dn11, locals.var_dtox_dn14,)
    } else {
        (locals.var_toxe, locals.var_toxe_dn0, locals.var_toxe_dn2, locals.var_toxe_dn4, locals.var_toxe_dn5, locals.var_toxe_dn6, locals.var_toxe_dn7, locals.var_toxe_dn8, locals.var_toxe_dn9, locals.var_toxe_dn10, locals.var_toxe_dn11, locals.var_toxe_dn14,)
    }
};
        locals.var_toxe = assign22740_e17483;
        locals.var_toxe_dn0 = assign22740_e17483_d_n0;
        locals.var_toxe_dn2 = assign22740_e17483_d_n2;
        locals.var_toxe_dn4 = assign22740_e17483_d_n4;
        locals.var_toxe_dn5 = assign22740_e17483_d_n5;
        locals.var_toxe_dn6 = assign22740_e17483_d_n6;
        locals.var_toxe_dn7 = assign22740_e17483_d_n7;
        locals.var_toxe_dn8 = assign22740_e17483_d_n8;
        locals.var_toxe_dn9 = assign22740_e17483_d_n9;
        locals.var_toxe_dn10 = assign22740_e17483_d_n10;
        locals.var_toxe_dn11 = assign22740_e17483_d_n11;
        locals.var_toxe_dn14 = assign22740_e17483_d_n14;
        locals.var_toxe_rv = 0.0;

        let (assign22750_e17490, assign22750_e17490_d_n0, assign22750_e17490_d_n2, assign22750_e17490_d_n4, assign22750_e17490_d_n5, assign22750_e17490_d_n6, assign22750_e17490_d_n7, assign22750_e17490_d_n8, assign22750_e17490_d_n9, assign22750_e17490_d_n10, assign22750_e17490_d_n11, assign22750_e17490_d_n14,) = {
    if (locals.var_guard430 == 0.0) {
        let assign22750_e17488: f64 = (locals.var_c_eox / locals.var_toxe);
        (assign22750_e17488, (-((locals.var_c_eox * locals.var_toxe_dn0) / (locals.var_toxe * locals.var_toxe))), (-((locals.var_c_eox * locals.var_toxe_dn2) / (locals.var_toxe * locals.var_toxe))), (-((locals.var_c_eox * locals.var_toxe_dn4) / (locals.var_toxe * locals.var_toxe))), (-((locals.var_c_eox * locals.var_toxe_dn5) / (locals.var_toxe * locals.var_toxe))), (-((locals.var_c_eox * locals.var_toxe_dn6) / (locals.var_toxe * locals.var_toxe))), (-((locals.var_c_eox * locals.var_toxe_dn7) / (locals.var_toxe * locals.var_toxe))), (-((locals.var_c_eox * locals.var_toxe_dn8) / (locals.var_toxe * locals.var_toxe))), (-((locals.var_c_eox * locals.var_toxe_dn9) / (locals.var_toxe * locals.var_toxe))), (-((locals.var_c_eox * locals.var_toxe_dn10) / (locals.var_toxe * locals.var_toxe))), (-((locals.var_c_eox * locals.var_toxe_dn11) / (locals.var_toxe * locals.var_toxe))), (-((locals.var_c_eox * locals.var_toxe_dn14) / (locals.var_toxe * locals.var_toxe))),)
    } else {
        (locals.var_cox, locals.var_cox_dn0, locals.var_cox_dn2, locals.var_cox_dn4, locals.var_cox_dn5, locals.var_cox_dn6, locals.var_cox_dn7, locals.var_cox_dn8, locals.var_cox_dn9, locals.var_cox_dn10, locals.var_cox_dn11, locals.var_cox_dn14,)
    }
};
        locals.var_cox = assign22750_e17490;
        locals.var_cox_dn0 = assign22750_e17490_d_n0;
        locals.var_cox_dn2 = assign22750_e17490_d_n2;
        locals.var_cox_dn4 = assign22750_e17490_d_n4;
        locals.var_cox_dn5 = assign22750_e17490_d_n5;
        locals.var_cox_dn6 = assign22750_e17490_d_n6;
        locals.var_cox_dn7 = assign22750_e17490_d_n7;
        locals.var_cox_dn8 = assign22750_e17490_d_n8;
        locals.var_cox_dn9 = assign22750_e17490_d_n9;
        locals.var_cox_dn10 = assign22750_e17490_d_n10;
        locals.var_cox_dn11 = assign22750_e17490_d_n11;
        locals.var_cox_dn14 = assign22750_e17490_d_n14;
        locals.var_cox_rv = 0.0;

        let (assign22760_e17500, assign22760_e17500_d_n0, assign22760_e17500_d_n2, assign22760_e17500_d_n4, assign22760_e17500_d_n5, assign22760_e17500_d_n6, assign22760_e17500_d_n7, assign22760_e17500_d_n8, assign22760_e17500_d_n9, assign22760_e17500_d_n10, assign22760_e17500_d_n11, assign22760_e17500_d_n14,) = {
    if (locals.var_guard430 == 0.0) {
        let assign22760_e17494: f64 = (-locals.var_c_eox);
        let assign22760_e17497: f64 = (locals.var_toxe * locals.var_toxe);
        let assign22760_e17498: f64 = (assign22760_e17494 / assign22760_e17497);
        (assign22760_e17498, (-((assign22760_e17494 * ((locals.var_toxe_dn0 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn0))) / (assign22760_e17497 * assign22760_e17497))), (-((assign22760_e17494 * ((locals.var_toxe_dn2 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn2))) / (assign22760_e17497 * assign22760_e17497))), (-((assign22760_e17494 * ((locals.var_toxe_dn4 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn4))) / (assign22760_e17497 * assign22760_e17497))), (-((assign22760_e17494 * ((locals.var_toxe_dn5 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn5))) / (assign22760_e17497 * assign22760_e17497))), (-((assign22760_e17494 * ((locals.var_toxe_dn6 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn6))) / (assign22760_e17497 * assign22760_e17497))), (-((assign22760_e17494 * ((locals.var_toxe_dn7 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn7))) / (assign22760_e17497 * assign22760_e17497))), (-((assign22760_e17494 * ((locals.var_toxe_dn8 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn8))) / (assign22760_e17497 * assign22760_e17497))), (-((assign22760_e17494 * ((locals.var_toxe_dn9 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn9))) / (assign22760_e17497 * assign22760_e17497))), (-((assign22760_e17494 * ((locals.var_toxe_dn10 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn10))) / (assign22760_e17497 * assign22760_e17497))), (-((assign22760_e17494 * ((locals.var_toxe_dn11 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn11))) / (assign22760_e17497 * assign22760_e17497))), (-((assign22760_e17494 * ((locals.var_toxe_dn14 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn14))) / (assign22760_e17497 * assign22760_e17497))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign22760_e17500;
        locals.var_t1_dn0 = assign22760_e17500_d_n0;
        locals.var_t1_dn2 = assign22760_e17500_d_n2;
        locals.var_t1_dn4 = assign22760_e17500_d_n4;
        locals.var_t1_dn5 = assign22760_e17500_d_n5;
        locals.var_t1_dn6 = assign22760_e17500_d_n6;
        locals.var_t1_dn7 = assign22760_e17500_d_n7;
        locals.var_t1_dn8 = assign22760_e17500_d_n8;
        locals.var_t1_dn9 = assign22760_e17500_d_n9;
        locals.var_t1_dn10 = assign22760_e17500_d_n10;
        locals.var_t1_dn11 = assign22760_e17500_d_n11;
        locals.var_t1_dn14 = assign22760_e17500_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign22770_e17507, assign22770_e17507_d_n0, assign22770_e17507_d_n2, assign22770_e17507_d_n4, assign22770_e17507_d_n5, assign22770_e17507_d_n6, assign22770_e17507_d_n7, assign22770_e17507_d_n8, assign22770_e17507_d_n9, assign22770_e17507_d_n10, assign22770_e17507_d_n11, assign22770_e17507_d_n14,) = {
    if (locals.var_guard430 == 0.0) {
        let assign22770_e17505: f64 = (locals.var_toxe / locals.var_c_eox);
        (assign22770_e17505, (locals.var_toxe_dn0 / locals.var_c_eox), (locals.var_toxe_dn2 / locals.var_c_eox), (locals.var_toxe_dn4 / locals.var_c_eox), (locals.var_toxe_dn5 / locals.var_c_eox), (locals.var_toxe_dn6 / locals.var_c_eox), (locals.var_toxe_dn7 / locals.var_c_eox), (locals.var_toxe_dn8 / locals.var_c_eox), (locals.var_toxe_dn9 / locals.var_c_eox), (locals.var_toxe_dn10 / locals.var_c_eox), (locals.var_toxe_dn11 / locals.var_c_eox), (locals.var_toxe_dn14 / locals.var_c_eox),)
    } else {
        (locals.var_cox_inv, locals.var_cox_inv_dn0, locals.var_cox_inv_dn2, locals.var_cox_inv_dn4, locals.var_cox_inv_dn5, locals.var_cox_inv_dn6, locals.var_cox_inv_dn7, locals.var_cox_inv_dn8, locals.var_cox_inv_dn9, locals.var_cox_inv_dn10, locals.var_cox_inv_dn11, locals.var_cox_inv_dn14,)
    }
};
        locals.var_cox_inv = assign22770_e17507;
        locals.var_cox_inv_dn0 = assign22770_e17507_d_n0;
        locals.var_cox_inv_dn2 = assign22770_e17507_d_n2;
        locals.var_cox_inv_dn4 = assign22770_e17507_d_n4;
        locals.var_cox_inv_dn5 = assign22770_e17507_d_n5;
        locals.var_cox_inv_dn6 = assign22770_e17507_d_n6;
        locals.var_cox_inv_dn7 = assign22770_e17507_d_n7;
        locals.var_cox_inv_dn8 = assign22770_e17507_d_n8;
        locals.var_cox_inv_dn9 = assign22770_e17507_d_n9;
        locals.var_cox_inv_dn10 = assign22770_e17507_d_n10;
        locals.var_cox_inv_dn11 = assign22770_e17507_d_n11;
        locals.var_cox_inv_dn14 = assign22770_e17507_d_n14;
        locals.var_cox_inv_rv = 0.0;

        let (assign22780_e17514, assign22780_e17514_d_n0, assign22780_e17514_d_n2, assign22780_e17514_d_n4, assign22780_e17514_d_n5, assign22780_e17514_d_n6, assign22780_e17514_d_n7, assign22780_e17514_d_n8, assign22780_e17514_d_n9, assign22780_e17514_d_n10, assign22780_e17514_d_n11, assign22780_e17514_d_n14,) = {
    if (locals.var_guard430 == 0.0) {
        let assign22780_e17512: f64 = (1.0 / locals.var_c_eox);
        (assign22780_e17512, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign22780_e17514;
        locals.var_t1_dn0 = assign22780_e17514_d_n0;
        locals.var_t1_dn2 = assign22780_e17514_d_n2;
        locals.var_t1_dn4 = assign22780_e17514_d_n4;
        locals.var_t1_dn5 = assign22780_e17514_d_n5;
        locals.var_t1_dn6 = assign22780_e17514_d_n6;
        locals.var_t1_dn7 = assign22780_e17514_d_n7;
        locals.var_t1_dn8 = assign22780_e17514_d_n8;
        locals.var_t1_dn9 = assign22780_e17514_d_n9;
        locals.var_t1_dn10 = assign22780_e17514_d_n10;
        locals.var_t1_dn11 = assign22780_e17514_d_n11;
        locals.var_t1_dn14 = assign22780_e17514_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign22790_e17523, assign22790_e17523_d_n0, assign22790_e17523_d_n2, assign22790_e17523_d_n4, assign22790_e17523_d_n5, assign22790_e17523_d_n6, assign22790_e17523_d_n7, assign22790_e17523_d_n8, assign22790_e17523_d_n9, assign22790_e17523_d_n10, assign22790_e17523_d_n11, assign22790_e17523_d_n14,) = {
    if (locals.var_guard430 == 0.0) {
        let assign22790_e17519: f64 = (locals.var_cnst0 * locals.var_cnst0);
        let assign22790_e17521: f64 = (assign22790_e17519 * locals.var_cox_inv);
        (assign22790_e17521, ((((locals.var_cnst0_dn0 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn0)) * locals.var_cox_inv) + (assign22790_e17519 * locals.var_cox_inv_dn0)), ((((locals.var_cnst0_dn2 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn2)) * locals.var_cox_inv) + (assign22790_e17519 * locals.var_cox_inv_dn2)), ((((locals.var_cnst0_dn4 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn4)) * locals.var_cox_inv) + (assign22790_e17519 * locals.var_cox_inv_dn4)), ((((locals.var_cnst0_dn5 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn5)) * locals.var_cox_inv) + (assign22790_e17519 * locals.var_cox_inv_dn5)), ((((locals.var_cnst0_dn6 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn6)) * locals.var_cox_inv) + (assign22790_e17519 * locals.var_cox_inv_dn6)), ((((locals.var_cnst0_dn7 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn7)) * locals.var_cox_inv) + (assign22790_e17519 * locals.var_cox_inv_dn7)), ((((locals.var_cnst0_dn8 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn8)) * locals.var_cox_inv) + (assign22790_e17519 * locals.var_cox_inv_dn8)), ((((locals.var_cnst0_dn9 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn9)) * locals.var_cox_inv) + (assign22790_e17519 * locals.var_cox_inv_dn9)), ((((locals.var_cnst0_dn10 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn10)) * locals.var_cox_inv) + (assign22790_e17519 * locals.var_cox_inv_dn10)), ((((locals.var_cnst0_dn11 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn11)) * locals.var_cox_inv) + (assign22790_e17519 * locals.var_cox_inv_dn11)), ((((locals.var_cnst0_dn14 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn14)) * locals.var_cox_inv) + (assign22790_e17519 * locals.var_cox_inv_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign22790_e17523;
        locals.var_t0_dn0 = assign22790_e17523_d_n0;
        locals.var_t0_dn2 = assign22790_e17523_d_n2;
        locals.var_t0_dn4 = assign22790_e17523_d_n4;
        locals.var_t0_dn5 = assign22790_e17523_d_n5;
        locals.var_t0_dn6 = assign22790_e17523_d_n6;
        locals.var_t0_dn7 = assign22790_e17523_d_n7;
        locals.var_t0_dn8 = assign22790_e17523_d_n8;
        locals.var_t0_dn9 = assign22790_e17523_d_n9;
        locals.var_t0_dn10 = assign22790_e17523_d_n10;
        locals.var_t0_dn11 = assign22790_e17523_d_n11;
        locals.var_t0_dn14 = assign22790_e17523_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign22800_e17530, assign22800_e17530_d_n0, assign22800_e17530_d_n2, assign22800_e17530_d_n4, assign22800_e17530_d_n5, assign22800_e17530_d_n6, assign22800_e17530_d_n7, assign22800_e17530_d_n8, assign22800_e17530_d_n9, assign22800_e17530_d_n10, assign22800_e17530_d_n11, assign22800_e17530_d_n14,) = {
    if (locals.var_guard430 == 0.0) {
        let assign22800_e17528: f64 = (locals.var_t0 * locals.var_cox_inv);
        (assign22800_e17528, ((locals.var_t0_dn0 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn0)), ((locals.var_t0_dn2 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn2)), ((locals.var_t0_dn4 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn4)), ((locals.var_t0_dn5 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn5)), ((locals.var_t0_dn6 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn6)), ((locals.var_t0_dn7 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn7)), ((locals.var_t0_dn8 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn8)), ((locals.var_t0_dn9 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn9)), ((locals.var_t0_dn10 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn10)), ((locals.var_t0_dn11 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn11)), ((locals.var_t0_dn14 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn14)),)
    } else {
        (locals.var_cnstcoxi, locals.var_cnstcoxi_dn0, locals.var_cnstcoxi_dn2, locals.var_cnstcoxi_dn4, locals.var_cnstcoxi_dn5, locals.var_cnstcoxi_dn6, locals.var_cnstcoxi_dn7, locals.var_cnstcoxi_dn8, locals.var_cnstcoxi_dn9, locals.var_cnstcoxi_dn10, locals.var_cnstcoxi_dn11, locals.var_cnstcoxi_dn14,)
    }
};
        locals.var_cnstcoxi = assign22800_e17530;
        locals.var_cnstcoxi_dn0 = assign22800_e17530_d_n0;
        locals.var_cnstcoxi_dn2 = assign22800_e17530_d_n2;
        locals.var_cnstcoxi_dn4 = assign22800_e17530_d_n4;
        locals.var_cnstcoxi_dn5 = assign22800_e17530_d_n5;
        locals.var_cnstcoxi_dn6 = assign22800_e17530_d_n6;
        locals.var_cnstcoxi_dn7 = assign22800_e17530_d_n7;
        locals.var_cnstcoxi_dn8 = assign22800_e17530_d_n8;
        locals.var_cnstcoxi_dn9 = assign22800_e17530_d_n9;
        locals.var_cnstcoxi_dn10 = assign22800_e17530_d_n10;
        locals.var_cnstcoxi_dn11 = assign22800_e17530_d_n11;
        locals.var_cnstcoxi_dn14 = assign22800_e17530_d_n14;
        locals.var_cnstcoxi_rv = 0.0;

        locals.var_vbsz2 = locals.var_vbsz;
        locals.var_vbsz2_dn0 = locals.var_vbsz_dn0;
        locals.var_vbsz2_dn2 = locals.var_vbsz_dn2;
        locals.var_vbsz2_dn4 = locals.var_vbsz_dn4;
        locals.var_vbsz2_dn5 = locals.var_vbsz_dn5;
        locals.var_vbsz2_dn6 = locals.var_vbsz_dn6;
        locals.var_vbsz2_dn7 = locals.var_vbsz_dn7;
        locals.var_vbsz2_dn8 = locals.var_vbsz_dn8;
        locals.var_vbsz2_dn9 = locals.var_vbsz_dn9;
        locals.var_vbsz2_dn10 = locals.var_vbsz_dn10;
        locals.var_vbsz2_dn11 = locals.var_vbsz_dn11;
        locals.var_vbsz2_dn14 = locals.var_vbsz_dn14;
        locals.var_vbsz2_rv = 0.0;

        locals.var_t1 = locals.var_qnsub_esi2;
        locals.var_t1_dn0 = locals.var_qnsub_esi2_dn0;
        locals.var_t1_dn2 = locals.var_qnsub_esi2_dn2;
        locals.var_t1_dn4 = locals.var_qnsub_esi2_dn4;
        locals.var_t1_dn5 = locals.var_qnsub_esi2_dn5;
        locals.var_t1_dn6 = locals.var_qnsub_esi2_dn6;
        locals.var_t1_dn7 = locals.var_qnsub_esi2_dn7;
        locals.var_t1_dn8 = locals.var_qnsub_esi2_dn8;
        locals.var_t1_dn9 = locals.var_qnsub_esi2_dn9;
        locals.var_t1_dn10 = locals.var_qnsub_esi2_dn10;
        locals.var_t1_dn11 = locals.var_qnsub_esi2_dn11;
        locals.var_t1_dn14 = locals.var_qnsub_esi2_dn14;
        locals.var_t1_rv = 0.0;

        let assign22830_e17536: f64 = (locals.var_pb20 - locals.var_vbsz2);
        let assign22830_e17537: f64 = (locals.var_t1 * assign22830_e17536);
        let assign22830_e17538: f64 = (assign22830_e17537).sqrt();
        locals.var_qb0 = assign22830_e17538;
        locals.var_qb0_dn0 = (((locals.var_t1_dn0 * assign22830_e17536) + (locals.var_t1 * (locals.var_pb20_dn0 - locals.var_vbsz2_dn0))) / (2.0 * assign22830_e17538));
        locals.var_qb0_dn2 = (((locals.var_t1_dn2 * assign22830_e17536) + (locals.var_t1 * (locals.var_pb20_dn2 - locals.var_vbsz2_dn2))) / (2.0 * assign22830_e17538));
        locals.var_qb0_dn4 = (((locals.var_t1_dn4 * assign22830_e17536) + (locals.var_t1 * (locals.var_pb20_dn4 - locals.var_vbsz2_dn4))) / (2.0 * assign22830_e17538));
        locals.var_qb0_dn5 = (((locals.var_t1_dn5 * assign22830_e17536) + (locals.var_t1 * (locals.var_pb20_dn5 - locals.var_vbsz2_dn5))) / (2.0 * assign22830_e17538));
        locals.var_qb0_dn6 = (((locals.var_t1_dn6 * assign22830_e17536) + (locals.var_t1 * (locals.var_pb20_dn6 - locals.var_vbsz2_dn6))) / (2.0 * assign22830_e17538));
        locals.var_qb0_dn7 = (((locals.var_t1_dn7 * assign22830_e17536) + (locals.var_t1 * (locals.var_pb20_dn7 - locals.var_vbsz2_dn7))) / (2.0 * assign22830_e17538));
        locals.var_qb0_dn8 = (((locals.var_t1_dn8 * assign22830_e17536) + (locals.var_t1 * (locals.var_pb20_dn8 - locals.var_vbsz2_dn8))) / (2.0 * assign22830_e17538));
        locals.var_qb0_dn9 = (((locals.var_t1_dn9 * assign22830_e17536) + (locals.var_t1 * (locals.var_pb20_dn9 - locals.var_vbsz2_dn9))) / (2.0 * assign22830_e17538));
        locals.var_qb0_dn10 = (((locals.var_t1_dn10 * assign22830_e17536) + (locals.var_t1 * (locals.var_pb20_dn10 - locals.var_vbsz2_dn10))) / (2.0 * assign22830_e17538));
        locals.var_qb0_dn11 = (((locals.var_t1_dn11 * assign22830_e17536) + (locals.var_t1 * (locals.var_pb20_dn11 - locals.var_vbsz2_dn11))) / (2.0 * assign22830_e17538));
        locals.var_qb0_dn14 = (((locals.var_t1_dn14 * assign22830_e17536) + (locals.var_t1 * (locals.var_pb20_dn14 - locals.var_vbsz2_dn14))) / (2.0 * assign22830_e17538));
        locals.var_qb0_rv = 0.0;

        let assign22840_e17541: f64 = (0.5 * locals.var_t1);
        let assign22840_e17543: f64 = (assign22840_e17541 / locals.var_qb0);
        locals.var_t2 = assign22840_e17543;
        locals.var_t2_dn0 = ((((0.5 * locals.var_t1_dn0) * locals.var_qb0) - (assign22840_e17541 * locals.var_qb0_dn0)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_dn2 = ((((0.5 * locals.var_t1_dn2) * locals.var_qb0) - (assign22840_e17541 * locals.var_qb0_dn2)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_dn4 = ((((0.5 * locals.var_t1_dn4) * locals.var_qb0) - (assign22840_e17541 * locals.var_qb0_dn4)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_dn5 = ((((0.5 * locals.var_t1_dn5) * locals.var_qb0) - (assign22840_e17541 * locals.var_qb0_dn5)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_dn6 = ((((0.5 * locals.var_t1_dn6) * locals.var_qb0) - (assign22840_e17541 * locals.var_qb0_dn6)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_dn7 = ((((0.5 * locals.var_t1_dn7) * locals.var_qb0) - (assign22840_e17541 * locals.var_qb0_dn7)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_dn8 = ((((0.5 * locals.var_t1_dn8) * locals.var_qb0) - (assign22840_e17541 * locals.var_qb0_dn8)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_dn9 = ((((0.5 * locals.var_t1_dn9) * locals.var_qb0) - (assign22840_e17541 * locals.var_qb0_dn9)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_dn10 = ((((0.5 * locals.var_t1_dn10) * locals.var_qb0) - (assign22840_e17541 * locals.var_qb0_dn10)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_dn11 = ((((0.5 * locals.var_t1_dn11) * locals.var_qb0) - (assign22840_e17541 * locals.var_qb0_dn11)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_dn14 = ((((0.5 * locals.var_t1_dn14) * locals.var_qb0) - (assign22840_e17541 * locals.var_qb0_dn14)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_rv = 0.0;

        let assign22850_e17546: f64 = (locals.var_pb20 + locals.var_vfb);
        let assign22850_e17549: f64 = (locals.var_qb0 * locals.var_cox_inv);
        let assign22850_e17550: f64 = (assign22850_e17546 + assign22850_e17549);
        let assign22850_e17552: f64 = (assign22850_e17550 + locals.var_ptovr);
        locals.var_vthp = assign22850_e17552;
        locals.var_vthp_dn0 = ((locals.var_pb20_dn0 + ((locals.var_qb0_dn0 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn0))) + locals.var_ptovr_dn0);
        locals.var_vthp_dn2 = ((locals.var_pb20_dn2 + ((locals.var_qb0_dn2 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn2))) + locals.var_ptovr_dn2);
        locals.var_vthp_dn4 = ((locals.var_pb20_dn4 + ((locals.var_qb0_dn4 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn4))) + locals.var_ptovr_dn4);
        locals.var_vthp_dn5 = ((locals.var_pb20_dn5 + ((locals.var_qb0_dn5 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn5))) + locals.var_ptovr_dn5);
        locals.var_vthp_dn6 = ((locals.var_pb20_dn6 + ((locals.var_qb0_dn6 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn6))) + locals.var_ptovr_dn6);
        locals.var_vthp_dn7 = ((locals.var_pb20_dn7 + ((locals.var_qb0_dn7 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn7))) + locals.var_ptovr_dn7);
        locals.var_vthp_dn8 = ((locals.var_pb20_dn8 + ((locals.var_qb0_dn8 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn8))) + locals.var_ptovr_dn8);
        locals.var_vthp_dn9 = ((locals.var_pb20_dn9 + ((locals.var_qb0_dn9 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn9))) + locals.var_ptovr_dn9);
        locals.var_vthp_dn10 = ((locals.var_pb20_dn10 + ((locals.var_qb0_dn10 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn10))) + locals.var_ptovr_dn10);
        locals.var_vthp_dn11 = ((locals.var_pb20_dn11 + ((locals.var_qb0_dn11 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn11))) + locals.var_ptovr_dn11);
        locals.var_vthp_dn14 = ((locals.var_pb20_dn14 + ((locals.var_qb0_dn14 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn14))) + locals.var_ptovr_dn14);
        locals.var_vthp_rv = 0.0;

        locals.var_pb20b = locals.var_pb20;
        locals.var_pb20b_dn0 = locals.var_pb20_dn0;
        locals.var_pb20b_dn2 = locals.var_pb20_dn2;
        locals.var_pb20b_dn4 = locals.var_pb20_dn4;
        locals.var_pb20b_dn5 = locals.var_pb20_dn5;
        locals.var_pb20b_dn6 = locals.var_pb20_dn6;
        locals.var_pb20b_dn7 = locals.var_pb20_dn7;
        locals.var_pb20b_dn8 = locals.var_pb20_dn8;
        locals.var_pb20b_dn9 = locals.var_pb20_dn9;
        locals.var_pb20b_dn10 = locals.var_pb20_dn10;
        locals.var_pb20b_dn11 = locals.var_pb20_dn11;
        locals.var_pb20b_dn14 = locals.var_pb20_dn14;
        locals.var_pb20b_rv = 0.0;

        locals.var_t0 = 0.95;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn14 = 0.0;
        locals.var_t0_rv = 0.0;

        let (assign22880_e17560,) = {
    if (locals.var_uc_codep > 1.0) {
        (0.0,)
    } else {
        (1.0,)
    }
};
        locals.var_t4 = assign22880_e17560;
        locals.var_t4_dn0 = 0.0;
        locals.var_t4_dn2 = 0.0;
        locals.var_t4_dn4 = 0.0;
        locals.var_t4_dn5 = 0.0;
        locals.var_t4_dn6 = 0.0;
        locals.var_t4_dn7 = 0.0;
        locals.var_t4_dn8 = 0.0;
        locals.var_t4_dn9 = 0.0;
        locals.var_t4_dn10 = 0.0;
        locals.var_t4_dn11 = 0.0;
        locals.var_t4_dn14 = 0.0;
        locals.var_t4_rv = 0.0;

        let assign22890_e17563: f64 = (locals.var_t0 * locals.var_pb20b);
        let assign22890_e17566: f64 = (locals.var_t4 * locals.var_vbsz2);
        let assign22890_e17567: f64 = (assign22890_e17563 - assign22890_e17566);
        let assign22890_e17569: f64 = (assign22890_e17567 - 0.001);
        locals.var_t1 = assign22890_e17569;
        locals.var_t1_dn0 = (((locals.var_t0_dn0 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn0)) - ((locals.var_t4_dn0 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn0)));
        locals.var_t1_dn2 = (((locals.var_t0_dn2 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn2)) - ((locals.var_t4_dn2 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn2)));
        locals.var_t1_dn4 = (((locals.var_t0_dn4 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn4)) - ((locals.var_t4_dn4 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn4)));
        locals.var_t1_dn5 = (((locals.var_t0_dn5 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn5)) - ((locals.var_t4_dn5 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn5)));
        locals.var_t1_dn6 = (((locals.var_t0_dn6 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn6)) - ((locals.var_t4_dn6 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn6)));
        locals.var_t1_dn7 = (((locals.var_t0_dn7 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn7)) - ((locals.var_t4_dn7 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn7)));
        locals.var_t1_dn8 = (((locals.var_t0_dn8 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn8)) - ((locals.var_t4_dn8 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn8)));
        locals.var_t1_dn9 = (((locals.var_t0_dn9 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn9)) - ((locals.var_t4_dn9 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn9)));
        locals.var_t1_dn10 = (((locals.var_t0_dn10 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn10)) - ((locals.var_t4_dn10 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn10)));
        locals.var_t1_dn11 = (((locals.var_t0_dn11 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn11)) - ((locals.var_t4_dn11 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn11)));
        locals.var_t1_dn14 = (((locals.var_t0_dn14 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn14)) - ((locals.var_t4_dn14 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn14)));
        locals.var_t1_rv = 0.0;

        let assign22900_e17572: f64 = (locals.var_t1 * locals.var_t1);
        let assign22900_e17575: f64 = (4.0 * locals.var_t0);
        let assign22900_e17577: f64 = (assign22900_e17575 * locals.var_pb20b);
        let assign22900_e17579: f64 = (assign22900_e17577 * 0.001);
        let assign22900_e17580: f64 = (assign22900_e17572 + assign22900_e17579);
        let assign22900_e17581: f64 = (assign22900_e17580).sqrt();
        locals.var_t2 = assign22900_e17581;
        locals.var_t2_dn0 = ((((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) + ((((4.0 * locals.var_t0_dn0) * locals.var_pb20b) + (assign22900_e17575 * locals.var_pb20b_dn0)) * 0.001)) / (2.0 * assign22900_e17581));
        locals.var_t2_dn2 = ((((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) + ((((4.0 * locals.var_t0_dn2) * locals.var_pb20b) + (assign22900_e17575 * locals.var_pb20b_dn2)) * 0.001)) / (2.0 * assign22900_e17581));
        locals.var_t2_dn4 = ((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) + ((((4.0 * locals.var_t0_dn4) * locals.var_pb20b) + (assign22900_e17575 * locals.var_pb20b_dn4)) * 0.001)) / (2.0 * assign22900_e17581));
        locals.var_t2_dn5 = ((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) + ((((4.0 * locals.var_t0_dn5) * locals.var_pb20b) + (assign22900_e17575 * locals.var_pb20b_dn5)) * 0.001)) / (2.0 * assign22900_e17581));
        locals.var_t2_dn6 = ((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) + ((((4.0 * locals.var_t0_dn6) * locals.var_pb20b) + (assign22900_e17575 * locals.var_pb20b_dn6)) * 0.001)) / (2.0 * assign22900_e17581));
        locals.var_t2_dn7 = ((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) + ((((4.0 * locals.var_t0_dn7) * locals.var_pb20b) + (assign22900_e17575 * locals.var_pb20b_dn7)) * 0.001)) / (2.0 * assign22900_e17581));
        locals.var_t2_dn8 = ((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) + ((((4.0 * locals.var_t0_dn8) * locals.var_pb20b) + (assign22900_e17575 * locals.var_pb20b_dn8)) * 0.001)) / (2.0 * assign22900_e17581));
        locals.var_t2_dn9 = ((((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) + ((((4.0 * locals.var_t0_dn9) * locals.var_pb20b) + (assign22900_e17575 * locals.var_pb20b_dn9)) * 0.001)) / (2.0 * assign22900_e17581));
        locals.var_t2_dn10 = ((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) + ((((4.0 * locals.var_t0_dn10) * locals.var_pb20b) + (assign22900_e17575 * locals.var_pb20b_dn10)) * 0.001)) / (2.0 * assign22900_e17581));
        locals.var_t2_dn11 = ((((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) + ((((4.0 * locals.var_t0_dn11) * locals.var_pb20b) + (assign22900_e17575 * locals.var_pb20b_dn11)) * 0.001)) / (2.0 * assign22900_e17581));
        locals.var_t2_dn14 = ((((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) + ((((4.0 * locals.var_t0_dn14) * locals.var_pb20b) + (assign22900_e17575 * locals.var_pb20b_dn14)) * 0.001)) / (2.0 * assign22900_e17581));
        locals.var_t2_rv = 0.0;

        let assign22910_e17584: f64 = (locals.var_t0 * locals.var_pb20b);
        let assign22910_e17588: f64 = (locals.var_t1 + locals.var_t2);
        let assign22910_e17589: f64 = (0.5 * assign22910_e17588);
        let assign22910_e17590: f64 = (assign22910_e17584 - assign22910_e17589);
        locals.var_t3 = assign22910_e17590;
        locals.var_t3_dn0 = (((locals.var_t0_dn0 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn0)) - (0.5 * (locals.var_t1_dn0 + locals.var_t2_dn0)));
        locals.var_t3_dn2 = (((locals.var_t0_dn2 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn2)) - (0.5 * (locals.var_t1_dn2 + locals.var_t2_dn2)));
        locals.var_t3_dn4 = (((locals.var_t0_dn4 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn4)) - (0.5 * (locals.var_t1_dn4 + locals.var_t2_dn4)));
        locals.var_t3_dn5 = (((locals.var_t0_dn5 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn5)) - (0.5 * (locals.var_t1_dn5 + locals.var_t2_dn5)));
        locals.var_t3_dn6 = (((locals.var_t0_dn6 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn6)) - (0.5 * (locals.var_t1_dn6 + locals.var_t2_dn6)));
        locals.var_t3_dn7 = (((locals.var_t0_dn7 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn7)) - (0.5 * (locals.var_t1_dn7 + locals.var_t2_dn7)));
        locals.var_t3_dn8 = (((locals.var_t0_dn8 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn8)) - (0.5 * (locals.var_t1_dn8 + locals.var_t2_dn8)));
        locals.var_t3_dn9 = (((locals.var_t0_dn9 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn9)) - (0.5 * (locals.var_t1_dn9 + locals.var_t2_dn9)));
        locals.var_t3_dn10 = (((locals.var_t0_dn10 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn10)) - (0.5 * (locals.var_t1_dn10 + locals.var_t2_dn10)));
        locals.var_t3_dn11 = (((locals.var_t0_dn11 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn11)) - (0.5 * (locals.var_t1_dn11 + locals.var_t2_dn11)));
        locals.var_t3_dn14 = (((locals.var_t0_dn14 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn14)) - (0.5 * (locals.var_t1_dn14 + locals.var_t2_dn14)));
        locals.var_t3_rv = 0.0;

        let (assign22920_e17598, assign22920_e17598_d_n0, assign22920_e17598_d_n2, assign22920_e17598_d_n4, assign22920_e17598_d_n5, assign22920_e17598_d_n6, assign22920_e17598_d_n7, assign22920_e17598_d_n8, assign22920_e17598_d_n9, assign22920_e17598_d_n10, assign22920_e17598_d_n11, assign22920_e17598_d_n14,) = {
    if (locals.var_uc_codep == 1.0) {
        let assign22920_e17596: f64 = (p.p366 * locals.var_vdsz);
        (assign22920_e17596, (p.p366 * locals.var_vdsz_dn0), (p.p366 * locals.var_vdsz_dn2), (p.p366 * locals.var_vdsz_dn4), (p.p366 * locals.var_vdsz_dn5), (p.p366 * locals.var_vdsz_dn6), (p.p366 * locals.var_vdsz_dn7), (p.p366 * locals.var_vdsz_dn8), (p.p366 * locals.var_vdsz_dn9), (p.p366 * locals.var_vdsz_dn10), (p.p366 * locals.var_vdsz_dn11), (p.p366 * locals.var_vdsz_dn14),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        locals.var_t5 = assign22920_e17598;
        locals.var_t5_dn0 = assign22920_e17598_d_n0;
        locals.var_t5_dn2 = assign22920_e17598_d_n2;
        locals.var_t5_dn4 = assign22920_e17598_d_n4;
        locals.var_t5_dn5 = assign22920_e17598_d_n5;
        locals.var_t5_dn6 = assign22920_e17598_d_n6;
        locals.var_t5_dn7 = assign22920_e17598_d_n7;
        locals.var_t5_dn8 = assign22920_e17598_d_n8;
        locals.var_t5_dn9 = assign22920_e17598_d_n9;
        locals.var_t5_dn10 = assign22920_e17598_d_n10;
        locals.var_t5_dn11 = assign22920_e17598_d_n11;
        locals.var_t5_dn14 = assign22920_e17598_d_n14;
        locals.var_t5_rv = 0.0;

        let assign22930_e17601: f64 = (locals.var_pb20b - locals.var_t3);
        let assign22930_e17603: f64 = (assign22930_e17601 + locals.var_t5);
        locals.var_pbsum = assign22930_e17603;
        locals.var_pbsum_dn0 = ((locals.var_pb20b_dn0 - locals.var_t3_dn0) + locals.var_t5_dn0);
        locals.var_pbsum_dn2 = ((locals.var_pb20b_dn2 - locals.var_t3_dn2) + locals.var_t5_dn2);
        locals.var_pbsum_dn4 = ((locals.var_pb20b_dn4 - locals.var_t3_dn4) + locals.var_t5_dn4);
        locals.var_pbsum_dn5 = ((locals.var_pb20b_dn5 - locals.var_t3_dn5) + locals.var_t5_dn5);
        locals.var_pbsum_dn6 = ((locals.var_pb20b_dn6 - locals.var_t3_dn6) + locals.var_t5_dn6);
        locals.var_pbsum_dn7 = ((locals.var_pb20b_dn7 - locals.var_t3_dn7) + locals.var_t5_dn7);
        locals.var_pbsum_dn8 = ((locals.var_pb20b_dn8 - locals.var_t3_dn8) + locals.var_t5_dn8);
        locals.var_pbsum_dn9 = ((locals.var_pb20b_dn9 - locals.var_t3_dn9) + locals.var_t5_dn9);
        locals.var_pbsum_dn10 = ((locals.var_pb20b_dn10 - locals.var_t3_dn10) + locals.var_t5_dn10);
        locals.var_pbsum_dn11 = ((locals.var_pb20b_dn11 - locals.var_t3_dn11) + locals.var_t5_dn11);
        locals.var_pbsum_dn14 = ((locals.var_pb20b_dn14 - locals.var_t3_dn14) + locals.var_t5_dn14);
        locals.var_pbsum_rv = 0.0;

        let assign22940_e17605: f64 = (locals.var_pbsum).sqrt();
        locals.var_sqrt_pbsum = assign22940_e17605;
        locals.var_sqrt_pbsum_dn0 = (locals.var_pbsum_dn0 / (2.0 * assign22940_e17605));
        locals.var_sqrt_pbsum_dn2 = (locals.var_pbsum_dn2 / (2.0 * assign22940_e17605));
        locals.var_sqrt_pbsum_dn4 = (locals.var_pbsum_dn4 / (2.0 * assign22940_e17605));
        locals.var_sqrt_pbsum_dn5 = (locals.var_pbsum_dn5 / (2.0 * assign22940_e17605));
        locals.var_sqrt_pbsum_dn6 = (locals.var_pbsum_dn6 / (2.0 * assign22940_e17605));
        locals.var_sqrt_pbsum_dn7 = (locals.var_pbsum_dn7 / (2.0 * assign22940_e17605));
        locals.var_sqrt_pbsum_dn8 = (locals.var_pbsum_dn8 / (2.0 * assign22940_e17605));
        locals.var_sqrt_pbsum_dn9 = (locals.var_pbsum_dn9 / (2.0 * assign22940_e17605));
        locals.var_sqrt_pbsum_dn10 = (locals.var_pbsum_dn10 / (2.0 * assign22940_e17605));
        locals.var_sqrt_pbsum_dn11 = (locals.var_pbsum_dn11 / (2.0 * assign22940_e17605));
        locals.var_sqrt_pbsum_dn14 = (locals.var_pbsum_dn14 / (2.0 * assign22940_e17605));
        locals.var_sqrt_pbsum_rv = 0.0;

        let assign22950_e17608: f64 = if p.p140 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard434 = assign22950_e17608;
        locals.var_guard434_rv = 0.0;

        let (assign22960_e17612, assign22960_e17612_d_n0, assign22960_e17612_d_n2, assign22960_e17612_d_n4, assign22960_e17612_d_n5, assign22960_e17612_d_n6, assign22960_e17612_d_n7, assign22960_e17612_d_n8, assign22960_e17612_d_n9, assign22960_e17612_d_n10, assign22960_e17612_d_n11, assign22960_e17612_d_n14,) = {
    if (locals.var_guard434 != 0.0) {
        (locals.var_qnsub_esi2, locals.var_qnsub_esi2_dn0, locals.var_qnsub_esi2_dn2, locals.var_qnsub_esi2_dn4, locals.var_qnsub_esi2_dn5, locals.var_qnsub_esi2_dn6, locals.var_qnsub_esi2_dn7, locals.var_qnsub_esi2_dn8, locals.var_qnsub_esi2_dn9, locals.var_qnsub_esi2_dn10, locals.var_qnsub_esi2_dn11, locals.var_qnsub_esi2_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign22960_e17612;
        locals.var_t1_dn0 = assign22960_e17612_d_n0;
        locals.var_t1_dn2 = assign22960_e17612_d_n2;
        locals.var_t1_dn4 = assign22960_e17612_d_n4;
        locals.var_t1_dn5 = assign22960_e17612_d_n5;
        locals.var_t1_dn6 = assign22960_e17612_d_n6;
        locals.var_t1_dn7 = assign22960_e17612_d_n7;
        locals.var_t1_dn8 = assign22960_e17612_d_n8;
        locals.var_t1_dn9 = assign22960_e17612_d_n9;
        locals.var_t1_dn10 = assign22960_e17612_d_n10;
        locals.var_t1_dn11 = assign22960_e17612_d_n11;
        locals.var_t1_dn14 = assign22960_e17612_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign22970_e17618, assign22970_e17618_d_n0, assign22970_e17618_d_n2, assign22970_e17618_d_n4, assign22970_e17618_d_n5, assign22970_e17618_d_n6, assign22970_e17618_d_n7, assign22970_e17618_d_n8, assign22970_e17618_d_n9, assign22970_e17618_d_n10, assign22970_e17618_d_n11, assign22970_e17618_d_n14,) = {
    if (locals.var_guard434 != 0.0) {
        let assign22970_e17616: f64 = (p.p224 - locals.var_vbsz2);
        (assign22970_e17616, (-locals.var_vbsz2_dn0), (-locals.var_vbsz2_dn2), (-locals.var_vbsz2_dn4), (-locals.var_vbsz2_dn5), (-locals.var_vbsz2_dn6), (-locals.var_vbsz2_dn7), (-locals.var_vbsz2_dn8), (-locals.var_vbsz2_dn9), (-locals.var_vbsz2_dn10), (-locals.var_vbsz2_dn11), (-locals.var_vbsz2_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign22970_e17618;
        locals.var_t2_dn0 = assign22970_e17618_d_n0;
        locals.var_t2_dn2 = assign22970_e17618_d_n2;
        locals.var_t2_dn4 = assign22970_e17618_d_n4;
        locals.var_t2_dn5 = assign22970_e17618_d_n5;
        locals.var_t2_dn6 = assign22970_e17618_d_n6;
        locals.var_t2_dn7 = assign22970_e17618_d_n7;
        locals.var_t2_dn8 = assign22970_e17618_d_n8;
        locals.var_t2_dn9 = assign22970_e17618_d_n9;
        locals.var_t2_dn10 = assign22970_e17618_d_n10;
        locals.var_t2_dn11 = assign22970_e17618_d_n11;
        locals.var_t2_dn14 = assign22970_e17618_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign22980_e17624, assign22980_e17624_d_n0, assign22980_e17624_d_n2, assign22980_e17624_d_n4, assign22980_e17624_d_n5, assign22980_e17624_d_n6, assign22980_e17624_d_n7, assign22980_e17624_d_n8, assign22980_e17624_d_n9, assign22980_e17624_d_n10, assign22980_e17624_d_n11, assign22980_e17624_d_n14,) = {
    if (locals.var_guard434 != 0.0) {
        let assign22980_e17622: f64 = (locals.var_t2 + 1e-25);
        (assign22980_e17622, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign22980_e17624;
        locals.var_t3_dn0 = assign22980_e17624_d_n0;
        locals.var_t3_dn2 = assign22980_e17624_d_n2;
        locals.var_t3_dn4 = assign22980_e17624_d_n4;
        locals.var_t3_dn5 = assign22980_e17624_d_n5;
        locals.var_t3_dn6 = assign22980_e17624_d_n6;
        locals.var_t3_dn7 = assign22980_e17624_d_n7;
        locals.var_t3_dn8 = assign22980_e17624_d_n8;
        locals.var_t3_dn9 = assign22980_e17624_d_n9;
        locals.var_t3_dn10 = assign22980_e17624_d_n10;
        locals.var_t3_dn11 = assign22980_e17624_d_n11;
        locals.var_t3_dn14 = assign22980_e17624_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign22990_e17635, assign22990_e17635_d_n0, assign22990_e17635_d_n2, assign22990_e17635_d_n4, assign22990_e17635_d_n5, assign22990_e17635_d_n6, assign22990_e17635_d_n7, assign22990_e17635_d_n8, assign22990_e17635_d_n9, assign22990_e17635_d_n10, assign22990_e17635_d_n11, assign22990_e17635_d_n14,) = {
    if (locals.var_guard434 != 0.0) {
        let assign22990_e17628: f64 = (locals.var_t3 * locals.var_t3);
        let assign22990_e17631: f64 = (4.0 * 0.001);
        let assign22990_e17632: f64 = (assign22990_e17628 + assign22990_e17631);
        let assign22990_e17633: f64 = (assign22990_e17632).sqrt();
        (assign22990_e17633, (((locals.var_t3_dn0 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn0)) / (2.0 * assign22990_e17633)), (((locals.var_t3_dn2 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn2)) / (2.0 * assign22990_e17633)), (((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4)) / (2.0 * assign22990_e17633)), (((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5)) / (2.0 * assign22990_e17633)), (((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6)) / (2.0 * assign22990_e17633)), (((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7)) / (2.0 * assign22990_e17633)), (((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8)) / (2.0 * assign22990_e17633)), (((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9)) / (2.0 * assign22990_e17633)), (((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10)) / (2.0 * assign22990_e17633)), (((locals.var_t3_dn11 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn11)) / (2.0 * assign22990_e17633)), (((locals.var_t3_dn14 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn14)) / (2.0 * assign22990_e17633)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign22990_e17635;
        locals.var_t4_dn0 = assign22990_e17635_d_n0;
        locals.var_t4_dn2 = assign22990_e17635_d_n2;
        locals.var_t4_dn4 = assign22990_e17635_d_n4;
        locals.var_t4_dn5 = assign22990_e17635_d_n5;
        locals.var_t4_dn6 = assign22990_e17635_d_n6;
        locals.var_t4_dn7 = assign22990_e17635_d_n7;
        locals.var_t4_dn8 = assign22990_e17635_d_n8;
        locals.var_t4_dn9 = assign22990_e17635_d_n9;
        locals.var_t4_dn10 = assign22990_e17635_d_n10;
        locals.var_t4_dn11 = assign22990_e17635_d_n11;
        locals.var_t4_dn14 = assign22990_e17635_d_n14;
        locals.var_t4_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_62(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23000_e17643, assign23000_e17643_d_n0, assign23000_e17643_d_n2, assign23000_e17643_d_n4, assign23000_e17643_d_n5, assign23000_e17643_d_n6, assign23000_e17643_d_n7, assign23000_e17643_d_n8, assign23000_e17643_d_n9, assign23000_e17643_d_n10, assign23000_e17643_d_n11, assign23000_e17643_d_n14,) = {
    if (locals.var_guard434 != 0.0) {
        let assign23000_e17640: f64 = (locals.var_t3 + locals.var_t4);
        let assign23000_e17641: f64 = (0.5 * assign23000_e17640);
        (assign23000_e17641, (0.5 * (locals.var_t3_dn0 + locals.var_t4_dn0)), (0.5 * (locals.var_t3_dn2 + locals.var_t4_dn2)), (0.5 * (locals.var_t3_dn4 + locals.var_t4_dn4)), (0.5 * (locals.var_t3_dn5 + locals.var_t4_dn5)), (0.5 * (locals.var_t3_dn6 + locals.var_t4_dn6)), (0.5 * (locals.var_t3_dn7 + locals.var_t4_dn7)), (0.5 * (locals.var_t3_dn8 + locals.var_t4_dn8)), (0.5 * (locals.var_t3_dn9 + locals.var_t4_dn9)), (0.5 * (locals.var_t3_dn10 + locals.var_t4_dn10)), (0.5 * (locals.var_t3_dn11 + locals.var_t4_dn11)), (0.5 * (locals.var_t3_dn14 + locals.var_t4_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign23000_e17643;
        locals.var_t5_dn0 = assign23000_e17643_d_n0;
        locals.var_t5_dn2 = assign23000_e17643_d_n2;
        locals.var_t5_dn4 = assign23000_e17643_d_n4;
        locals.var_t5_dn5 = assign23000_e17643_d_n5;
        locals.var_t5_dn6 = assign23000_e17643_d_n6;
        locals.var_t5_dn7 = assign23000_e17643_d_n7;
        locals.var_t5_dn8 = assign23000_e17643_d_n8;
        locals.var_t5_dn9 = assign23000_e17643_d_n9;
        locals.var_t5_dn10 = assign23000_e17643_d_n10;
        locals.var_t5_dn11 = assign23000_e17643_d_n11;
        locals.var_t5_dn14 = assign23000_e17643_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign23010_e17653, assign23010_e17653_d_n0, assign23010_e17653_d_n2, assign23010_e17653_d_n4, assign23010_e17653_d_n5, assign23010_e17653_d_n6, assign23010_e17653_d_n7, assign23010_e17653_d_n8, assign23010_e17653_d_n9, assign23010_e17653_d_n10, assign23010_e17653_d_n11, assign23010_e17653_d_n14,) = {
    if (locals.var_guard434 != 0.0) {
        let assign23010_e17649: f64 = (locals.var_t3 / locals.var_t4);
        let assign23010_e17650: f64 = (1.0 + assign23010_e17649);
        let assign23010_e17651: f64 = (0.5 * assign23010_e17650);
        (assign23010_e17651, (0.5 * (((locals.var_t3_dn0 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4))), (0.5 * (((locals.var_t3_dn2 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4))), (0.5 * (((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4))), (0.5 * (((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4))), (0.5 * (((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4))), (0.5 * (((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4))), (0.5 * (((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4))), (0.5 * (((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4))), (0.5 * (((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4))), (0.5 * (((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4))), (0.5 * (((locals.var_t3_dn14 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn14)) / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign23010_e17653;
        locals.var_t6_dn0 = assign23010_e17653_d_n0;
        locals.var_t6_dn2 = assign23010_e17653_d_n2;
        locals.var_t6_dn4 = assign23010_e17653_d_n4;
        locals.var_t6_dn5 = assign23010_e17653_d_n5;
        locals.var_t6_dn6 = assign23010_e17653_d_n6;
        locals.var_t6_dn7 = assign23010_e17653_d_n7;
        locals.var_t6_dn8 = assign23010_e17653_d_n8;
        locals.var_t6_dn9 = assign23010_e17653_d_n9;
        locals.var_t6_dn10 = assign23010_e17653_d_n10;
        locals.var_t6_dn11 = assign23010_e17653_d_n11;
        locals.var_t6_dn14 = assign23010_e17653_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign23020_e17659, assign23020_e17659_d_n0, assign23020_e17659_d_n2, assign23020_e17659_d_n4, assign23020_e17659_d_n5, assign23020_e17659_d_n6, assign23020_e17659_d_n7, assign23020_e17659_d_n8, assign23020_e17659_d_n9, assign23020_e17659_d_n10, assign23020_e17659_d_n11, assign23020_e17659_d_n14,) = {
    if (locals.var_guard434 != 0.0) {
        let assign23020_e17657: f64 = (1.0 / locals.var_t5);
        (assign23020_e17657, (-(locals.var_t5_dn0 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn2 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn4 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn5 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn6 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn7 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn8 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn9 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn10 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn11 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn14 / (locals.var_t5 * locals.var_t5))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign23020_e17659;
        locals.var_t7_dn0 = assign23020_e17659_d_n0;
        locals.var_t7_dn2 = assign23020_e17659_d_n2;
        locals.var_t7_dn4 = assign23020_e17659_d_n4;
        locals.var_t7_dn5 = assign23020_e17659_d_n5;
        locals.var_t7_dn6 = assign23020_e17659_d_n6;
        locals.var_t7_dn7 = assign23020_e17659_d_n7;
        locals.var_t7_dn8 = assign23020_e17659_d_n8;
        locals.var_t7_dn9 = assign23020_e17659_d_n9;
        locals.var_t7_dn10 = assign23020_e17659_d_n10;
        locals.var_t7_dn11 = assign23020_e17659_d_n11;
        locals.var_t7_dn14 = assign23020_e17659_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign23030_e17665, assign23030_e17665_d_n0, assign23030_e17665_d_n2, assign23030_e17665_d_n4, assign23030_e17665_d_n5, assign23030_e17665_d_n6, assign23030_e17665_d_n7, assign23030_e17665_d_n8, assign23030_e17665_d_n9, assign23030_e17665_d_n10, assign23030_e17665_d_n11, assign23030_e17665_d_n14,) = {
    if (locals.var_guard434 != 0.0) {
        let assign23030_e17663: f64 = (p.p223 * locals.var_t7);
        (assign23030_e17663, (p.p223 * locals.var_t7_dn0), (p.p223 * locals.var_t7_dn2), (p.p223 * locals.var_t7_dn4), (p.p223 * locals.var_t7_dn5), (p.p223 * locals.var_t7_dn6), (p.p223 * locals.var_t7_dn7), (p.p223 * locals.var_t7_dn8), (p.p223 * locals.var_t7_dn9), (p.p223 * locals.var_t7_dn10), (p.p223 * locals.var_t7_dn11), (p.p223 * locals.var_t7_dn14),)
    } else {
        (locals.var_bs12, locals.var_bs12_dn0, locals.var_bs12_dn2, locals.var_bs12_dn4, locals.var_bs12_dn5, locals.var_bs12_dn6, locals.var_bs12_dn7, locals.var_bs12_dn8, locals.var_bs12_dn9, locals.var_bs12_dn10, locals.var_bs12_dn11, locals.var_bs12_dn14,)
    }
};
        locals.var_bs12 = assign23030_e17665;
        locals.var_bs12_dn0 = assign23030_e17665_d_n0;
        locals.var_bs12_dn2 = assign23030_e17665_d_n2;
        locals.var_bs12_dn4 = assign23030_e17665_d_n4;
        locals.var_bs12_dn5 = assign23030_e17665_d_n5;
        locals.var_bs12_dn6 = assign23030_e17665_d_n6;
        locals.var_bs12_dn7 = assign23030_e17665_d_n7;
        locals.var_bs12_dn8 = assign23030_e17665_d_n8;
        locals.var_bs12_dn9 = assign23030_e17665_d_n9;
        locals.var_bs12_dn10 = assign23030_e17665_d_n10;
        locals.var_bs12_dn11 = assign23030_e17665_d_n11;
        locals.var_bs12_dn14 = assign23030_e17665_d_n14;
        locals.var_bs12_rv = 0.0;

        let (assign23040_e17672, assign23040_e17672_d_n0, assign23040_e17672_d_n2, assign23040_e17672_d_n4, assign23040_e17672_d_n5, assign23040_e17672_d_n6, assign23040_e17672_d_n7, assign23040_e17672_d_n8, assign23040_e17672_d_n9, assign23040_e17672_d_n10, assign23040_e17672_d_n11, assign23040_e17672_d_n14,) = {
    if (locals.var_guard434 != 0.0) {
        let assign23040_e17668: f64 = (-locals.var_bs12);
        let assign23040_e17670: f64 = (assign23040_e17668 * locals.var_t7);
        (assign23040_e17670, (((-locals.var_bs12_dn0) * locals.var_t7) + (assign23040_e17668 * locals.var_t7_dn0)), (((-locals.var_bs12_dn2) * locals.var_t7) + (assign23040_e17668 * locals.var_t7_dn2)), (((-locals.var_bs12_dn4) * locals.var_t7) + (assign23040_e17668 * locals.var_t7_dn4)), (((-locals.var_bs12_dn5) * locals.var_t7) + (assign23040_e17668 * locals.var_t7_dn5)), (((-locals.var_bs12_dn6) * locals.var_t7) + (assign23040_e17668 * locals.var_t7_dn6)), (((-locals.var_bs12_dn7) * locals.var_t7) + (assign23040_e17668 * locals.var_t7_dn7)), (((-locals.var_bs12_dn8) * locals.var_t7) + (assign23040_e17668 * locals.var_t7_dn8)), (((-locals.var_bs12_dn9) * locals.var_t7) + (assign23040_e17668 * locals.var_t7_dn9)), (((-locals.var_bs12_dn10) * locals.var_t7) + (assign23040_e17668 * locals.var_t7_dn10)), (((-locals.var_bs12_dn11) * locals.var_t7) + (assign23040_e17668 * locals.var_t7_dn11)), (((-locals.var_bs12_dn14) * locals.var_t7) + (assign23040_e17668 * locals.var_t7_dn14)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign23040_e17672;
        locals.var_t8_dn0 = assign23040_e17672_d_n0;
        locals.var_t8_dn2 = assign23040_e17672_d_n2;
        locals.var_t8_dn4 = assign23040_e17672_d_n4;
        locals.var_t8_dn5 = assign23040_e17672_d_n5;
        locals.var_t8_dn6 = assign23040_e17672_d_n6;
        locals.var_t8_dn7 = assign23040_e17672_d_n7;
        locals.var_t8_dn8 = assign23040_e17672_d_n8;
        locals.var_t8_dn9 = assign23040_e17672_d_n9;
        locals.var_t8_dn10 = assign23040_e17672_d_n10;
        locals.var_t8_dn11 = assign23040_e17672_d_n11;
        locals.var_t8_dn14 = assign23040_e17672_d_n14;
        locals.var_t8_rv = 0.0;

        let (assign23050_e17684, assign23050_e17684_d_n0, assign23050_e17684_d_n2, assign23050_e17684_d_n4, assign23050_e17684_d_n5, assign23050_e17684_d_n6, assign23050_e17684_d_n7, assign23050_e17684_d_n8, assign23050_e17684_d_n9, assign23050_e17684_d_n10, assign23050_e17684_d_n11, assign23050_e17684_d_n14,) = {
    if (locals.var_guard434 != 0.0) {
        let assign23050_e17676: f64 = (0.93 * locals.var_pb20);
        let assign23050_e17679: f64 = (locals.var_vbsz2 + locals.var_bs12);
        let assign23050_e17680: f64 = (assign23050_e17676 - assign23050_e17679);
        let assign23050_e17682: f64 = (assign23050_e17680 - 0.001);
        (assign23050_e17682, ((0.93 * locals.var_pb20_dn0) - (locals.var_vbsz2_dn0 + locals.var_bs12_dn0)), ((0.93 * locals.var_pb20_dn2) - (locals.var_vbsz2_dn2 + locals.var_bs12_dn2)), ((0.93 * locals.var_pb20_dn4) - (locals.var_vbsz2_dn4 + locals.var_bs12_dn4)), ((0.93 * locals.var_pb20_dn5) - (locals.var_vbsz2_dn5 + locals.var_bs12_dn5)), ((0.93 * locals.var_pb20_dn6) - (locals.var_vbsz2_dn6 + locals.var_bs12_dn6)), ((0.93 * locals.var_pb20_dn7) - (locals.var_vbsz2_dn7 + locals.var_bs12_dn7)), ((0.93 * locals.var_pb20_dn8) - (locals.var_vbsz2_dn8 + locals.var_bs12_dn8)), ((0.93 * locals.var_pb20_dn9) - (locals.var_vbsz2_dn9 + locals.var_bs12_dn9)), ((0.93 * locals.var_pb20_dn10) - (locals.var_vbsz2_dn10 + locals.var_bs12_dn10)), ((0.93 * locals.var_pb20_dn11) - (locals.var_vbsz2_dn11 + locals.var_bs12_dn11)), ((0.93 * locals.var_pb20_dn14) - (locals.var_vbsz2_dn14 + locals.var_bs12_dn14)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign23050_e17684;
        locals.var_tmf1_dn0 = assign23050_e17684_d_n0;
        locals.var_tmf1_dn2 = assign23050_e17684_d_n2;
        locals.var_tmf1_dn4 = assign23050_e17684_d_n4;
        locals.var_tmf1_dn5 = assign23050_e17684_d_n5;
        locals.var_tmf1_dn6 = assign23050_e17684_d_n6;
        locals.var_tmf1_dn7 = assign23050_e17684_d_n7;
        locals.var_tmf1_dn8 = assign23050_e17684_d_n8;
        locals.var_tmf1_dn9 = assign23050_e17684_d_n9;
        locals.var_tmf1_dn10 = assign23050_e17684_d_n10;
        locals.var_tmf1_dn11 = assign23050_e17684_d_n11;
        locals.var_tmf1_dn14 = assign23050_e17684_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign23060_e17694, assign23060_e17694_d_n0, assign23060_e17694_d_n2, assign23060_e17694_d_n4, assign23060_e17694_d_n5, assign23060_e17694_d_n6, assign23060_e17694_d_n7, assign23060_e17694_d_n8, assign23060_e17694_d_n9, assign23060_e17694_d_n10, assign23060_e17694_d_n11, assign23060_e17694_d_n14,) = {
    if (locals.var_guard434 != 0.0) {
        let assign23060_e17689: f64 = (0.93 * locals.var_pb20);
        let assign23060_e17690: f64 = (4.0 * assign23060_e17689);
        let assign23060_e17692: f64 = (assign23060_e17690 * 0.001);
        (assign23060_e17692, ((4.0 * (0.93 * locals.var_pb20_dn0)) * 0.001), ((4.0 * (0.93 * locals.var_pb20_dn2)) * 0.001), ((4.0 * (0.93 * locals.var_pb20_dn4)) * 0.001), ((4.0 * (0.93 * locals.var_pb20_dn5)) * 0.001), ((4.0 * (0.93 * locals.var_pb20_dn6)) * 0.001), ((4.0 * (0.93 * locals.var_pb20_dn7)) * 0.001), ((4.0 * (0.93 * locals.var_pb20_dn8)) * 0.001), ((4.0 * (0.93 * locals.var_pb20_dn9)) * 0.001), ((4.0 * (0.93 * locals.var_pb20_dn10)) * 0.001), ((4.0 * (0.93 * locals.var_pb20_dn11)) * 0.001), ((4.0 * (0.93 * locals.var_pb20_dn14)) * 0.001),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign23060_e17694;
        locals.var_tmf2_dn0 = assign23060_e17694_d_n0;
        locals.var_tmf2_dn2 = assign23060_e17694_d_n2;
        locals.var_tmf2_dn4 = assign23060_e17694_d_n4;
        locals.var_tmf2_dn5 = assign23060_e17694_d_n5;
        locals.var_tmf2_dn6 = assign23060_e17694_d_n6;
        locals.var_tmf2_dn7 = assign23060_e17694_d_n7;
        locals.var_tmf2_dn8 = assign23060_e17694_d_n8;
        locals.var_tmf2_dn9 = assign23060_e17694_d_n9;
        locals.var_tmf2_dn10 = assign23060_e17694_d_n10;
        locals.var_tmf2_dn11 = assign23060_e17694_d_n11;
        locals.var_tmf2_dn14 = assign23060_e17694_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign23070_e17704, assign23070_e17704_d_n0, assign23070_e17704_d_n2, assign23070_e17704_d_n4, assign23070_e17704_d_n5, assign23070_e17704_d_n6, assign23070_e17704_d_n7, assign23070_e17704_d_n8, assign23070_e17704_d_n9, assign23070_e17704_d_n10, assign23070_e17704_d_n11, assign23070_e17704_d_n14,) = {
    if (locals.var_guard434 != 0.0) {
        let (assign23070_e17702, assign23070_e17702_d_n0, assign23070_e17702_d_n2, assign23070_e17702_d_n4, assign23070_e17702_d_n5, assign23070_e17702_d_n6, assign23070_e17702_d_n7, assign23070_e17702_d_n8, assign23070_e17702_d_n9, assign23070_e17702_d_n10, assign23070_e17702_d_n11, assign23070_e17702_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign23070_e17701: f64 = (-locals.var_tmf2);
                (assign23070_e17701, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign23070_e17702, assign23070_e17702_d_n0, assign23070_e17702_d_n2, assign23070_e17702_d_n4, assign23070_e17702_d_n5, assign23070_e17702_d_n6, assign23070_e17702_d_n7, assign23070_e17702_d_n8, assign23070_e17702_d_n9, assign23070_e17702_d_n10, assign23070_e17702_d_n11, assign23070_e17702_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign23070_e17704;
        locals.var_tmf2_dn0 = assign23070_e17704_d_n0;
        locals.var_tmf2_dn2 = assign23070_e17704_d_n2;
        locals.var_tmf2_dn4 = assign23070_e17704_d_n4;
        locals.var_tmf2_dn5 = assign23070_e17704_d_n5;
        locals.var_tmf2_dn6 = assign23070_e17704_d_n6;
        locals.var_tmf2_dn7 = assign23070_e17704_d_n7;
        locals.var_tmf2_dn8 = assign23070_e17704_d_n8;
        locals.var_tmf2_dn9 = assign23070_e17704_d_n9;
        locals.var_tmf2_dn10 = assign23070_e17704_d_n10;
        locals.var_tmf2_dn11 = assign23070_e17704_d_n11;
        locals.var_tmf2_dn14 = assign23070_e17704_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign23080_e17713, assign23080_e17713_d_n0, assign23080_e17713_d_n2, assign23080_e17713_d_n4, assign23080_e17713_d_n5, assign23080_e17713_d_n6, assign23080_e17713_d_n7, assign23080_e17713_d_n8, assign23080_e17713_d_n9, assign23080_e17713_d_n10, assign23080_e17713_d_n11, assign23080_e17713_d_n14,) = {
    if (locals.var_guard434 != 0.0) {
        let assign23080_e17708: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign23080_e17710: f64 = (assign23080_e17708 + locals.var_tmf2);
        let assign23080_e17711: f64 = (assign23080_e17710).sqrt();
        (assign23080_e17711, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign23080_e17711)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign23080_e17711)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign23080_e17711)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign23080_e17711)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign23080_e17711)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign23080_e17711)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign23080_e17711)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign23080_e17711)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign23080_e17711)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign23080_e17711)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign23080_e17711)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign23080_e17713;
        locals.var_tmf2_dn0 = assign23080_e17713_d_n0;
        locals.var_tmf2_dn2 = assign23080_e17713_d_n2;
        locals.var_tmf2_dn4 = assign23080_e17713_d_n4;
        locals.var_tmf2_dn5 = assign23080_e17713_d_n5;
        locals.var_tmf2_dn6 = assign23080_e17713_d_n6;
        locals.var_tmf2_dn7 = assign23080_e17713_d_n7;
        locals.var_tmf2_dn8 = assign23080_e17713_d_n8;
        locals.var_tmf2_dn9 = assign23080_e17713_d_n9;
        locals.var_tmf2_dn10 = assign23080_e17713_d_n10;
        locals.var_tmf2_dn11 = assign23080_e17713_d_n11;
        locals.var_tmf2_dn14 = assign23080_e17713_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign23090_e17723, assign23090_e17723_d_n0, assign23090_e17723_d_n2, assign23090_e17723_d_n4, assign23090_e17723_d_n5, assign23090_e17723_d_n6, assign23090_e17723_d_n7, assign23090_e17723_d_n8, assign23090_e17723_d_n9, assign23090_e17723_d_n10, assign23090_e17723_d_n11, assign23090_e17723_d_n14,) = {
    if (locals.var_guard434 != 0.0) {
        let assign23090_e17719: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign23090_e17720: f64 = (1.0 + assign23090_e17719);
        let assign23090_e17721: f64 = (0.5 * assign23090_e17720);
        (assign23090_e17721, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign23090_e17723;
        locals.var_t0_dn0 = assign23090_e17723_d_n0;
        locals.var_t0_dn2 = assign23090_e17723_d_n2;
        locals.var_t0_dn4 = assign23090_e17723_d_n4;
        locals.var_t0_dn5 = assign23090_e17723_d_n5;
        locals.var_t0_dn6 = assign23090_e17723_d_n6;
        locals.var_t0_dn7 = assign23090_e17723_d_n7;
        locals.var_t0_dn8 = assign23090_e17723_d_n8;
        locals.var_t0_dn9 = assign23090_e17723_d_n9;
        locals.var_t0_dn10 = assign23090_e17723_d_n10;
        locals.var_t0_dn11 = assign23090_e17723_d_n11;
        locals.var_t0_dn14 = assign23090_e17723_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign23100_e17735, assign23100_e17735_d_n0, assign23100_e17735_d_n2, assign23100_e17735_d_n4, assign23100_e17735_d_n5, assign23100_e17735_d_n6, assign23100_e17735_d_n7, assign23100_e17735_d_n8, assign23100_e17735_d_n9, assign23100_e17735_d_n10, assign23100_e17735_d_n11, assign23100_e17735_d_n14,) = {
    if (locals.var_guard434 != 0.0) {
        let assign23100_e17727: f64 = (0.93 * locals.var_pb20);
        let assign23100_e17731: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign23100_e17732: f64 = (0.5 * assign23100_e17731);
        let assign23100_e17733: f64 = (assign23100_e17727 - assign23100_e17732);
        (assign23100_e17733, ((0.93 * locals.var_pb20_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((0.93 * locals.var_pb20_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((0.93 * locals.var_pb20_dn4) - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), ((0.93 * locals.var_pb20_dn5) - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), ((0.93 * locals.var_pb20_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((0.93 * locals.var_pb20_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((0.93 * locals.var_pb20_dn8) - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), ((0.93 * locals.var_pb20_dn9) - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), ((0.93 * locals.var_pb20_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((0.93 * locals.var_pb20_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((0.93 * locals.var_pb20_dn14) - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign23100_e17735;
        locals.var_t10_dn0 = assign23100_e17735_d_n0;
        locals.var_t10_dn2 = assign23100_e17735_d_n2;
        locals.var_t10_dn4 = assign23100_e17735_d_n4;
        locals.var_t10_dn5 = assign23100_e17735_d_n5;
        locals.var_t10_dn6 = assign23100_e17735_d_n6;
        locals.var_t10_dn7 = assign23100_e17735_d_n7;
        locals.var_t10_dn8 = assign23100_e17735_d_n8;
        locals.var_t10_dn9 = assign23100_e17735_d_n9;
        locals.var_t10_dn10 = assign23100_e17735_d_n10;
        locals.var_t10_dn11 = assign23100_e17735_d_n11;
        locals.var_t10_dn14 = assign23100_e17735_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign23110_e17744, assign23110_e17744_d_n0, assign23110_e17744_d_n2, assign23110_e17744_d_n4, assign23110_e17744_d_n5, assign23110_e17744_d_n6, assign23110_e17744_d_n7, assign23110_e17744_d_n8, assign23110_e17744_d_n9, assign23110_e17744_d_n10, assign23110_e17744_d_n11, assign23110_e17744_d_n14,) = {
    if (locals.var_guard434 != 0.0) {
        let assign23110_e17740: f64 = (locals.var_pb20 - locals.var_t10);
        let assign23110_e17741: f64 = (locals.var_t1 * assign23110_e17740);
        let assign23110_e17742: f64 = (assign23110_e17741).sqrt();
        (assign23110_e17742, (((locals.var_t1_dn0 * assign23110_e17740) + (locals.var_t1 * (locals.var_pb20_dn0 - locals.var_t10_dn0))) / (2.0 * assign23110_e17742)), (((locals.var_t1_dn2 * assign23110_e17740) + (locals.var_t1 * (locals.var_pb20_dn2 - locals.var_t10_dn2))) / (2.0 * assign23110_e17742)), (((locals.var_t1_dn4 * assign23110_e17740) + (locals.var_t1 * (locals.var_pb20_dn4 - locals.var_t10_dn4))) / (2.0 * assign23110_e17742)), (((locals.var_t1_dn5 * assign23110_e17740) + (locals.var_t1 * (locals.var_pb20_dn5 - locals.var_t10_dn5))) / (2.0 * assign23110_e17742)), (((locals.var_t1_dn6 * assign23110_e17740) + (locals.var_t1 * (locals.var_pb20_dn6 - locals.var_t10_dn6))) / (2.0 * assign23110_e17742)), (((locals.var_t1_dn7 * assign23110_e17740) + (locals.var_t1 * (locals.var_pb20_dn7 - locals.var_t10_dn7))) / (2.0 * assign23110_e17742)), (((locals.var_t1_dn8 * assign23110_e17740) + (locals.var_t1 * (locals.var_pb20_dn8 - locals.var_t10_dn8))) / (2.0 * assign23110_e17742)), (((locals.var_t1_dn9 * assign23110_e17740) + (locals.var_t1 * (locals.var_pb20_dn9 - locals.var_t10_dn9))) / (2.0 * assign23110_e17742)), (((locals.var_t1_dn10 * assign23110_e17740) + (locals.var_t1 * (locals.var_pb20_dn10 - locals.var_t10_dn10))) / (2.0 * assign23110_e17742)), (((locals.var_t1_dn11 * assign23110_e17740) + (locals.var_t1 * (locals.var_pb20_dn11 - locals.var_t10_dn11))) / (2.0 * assign23110_e17742)), (((locals.var_t1_dn14 * assign23110_e17740) + (locals.var_t1 * (locals.var_pb20_dn14 - locals.var_t10_dn14))) / (2.0 * assign23110_e17742)),)
    } else {
        (locals.var_qbmm, locals.var_qbmm_dn0, locals.var_qbmm_dn2, locals.var_qbmm_dn4, locals.var_qbmm_dn5, locals.var_qbmm_dn6, locals.var_qbmm_dn7, locals.var_qbmm_dn8, locals.var_qbmm_dn9, locals.var_qbmm_dn10, locals.var_qbmm_dn11, locals.var_qbmm_dn14,)
    }
};
        locals.var_qbmm = assign23110_e17744;
        locals.var_qbmm_dn0 = assign23110_e17744_d_n0;
        locals.var_qbmm_dn2 = assign23110_e17744_d_n2;
        locals.var_qbmm_dn4 = assign23110_e17744_d_n4;
        locals.var_qbmm_dn5 = assign23110_e17744_d_n5;
        locals.var_qbmm_dn6 = assign23110_e17744_d_n6;
        locals.var_qbmm_dn7 = assign23110_e17744_d_n7;
        locals.var_qbmm_dn8 = assign23110_e17744_d_n8;
        locals.var_qbmm_dn9 = assign23110_e17744_d_n9;
        locals.var_qbmm_dn10 = assign23110_e17744_d_n10;
        locals.var_qbmm_dn11 = assign23110_e17744_d_n11;
        locals.var_qbmm_dn14 = assign23110_e17744_d_n14;
        locals.var_qbmm_rv = 0.0;

        let (assign23120_e17750, assign23120_e17750_d_n0, assign23120_e17750_d_n2, assign23120_e17750_d_n4, assign23120_e17750_d_n5, assign23120_e17750_d_n6, assign23120_e17750_d_n7, assign23120_e17750_d_n8, assign23120_e17750_d_n9, assign23120_e17750_d_n10, assign23120_e17750_d_n11, assign23120_e17750_d_n14,) = {
    if (locals.var_guard434 != 0.0) {
        let assign23120_e17748: f64 = (locals.var_t0 / locals.var_qbmm);
        (assign23120_e17748, (((locals.var_t0_dn0 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn0)) / (locals.var_qbmm * locals.var_qbmm)), (((locals.var_t0_dn2 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn2)) / (locals.var_qbmm * locals.var_qbmm)), (((locals.var_t0_dn4 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn4)) / (locals.var_qbmm * locals.var_qbmm)), (((locals.var_t0_dn5 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn5)) / (locals.var_qbmm * locals.var_qbmm)), (((locals.var_t0_dn6 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn6)) / (locals.var_qbmm * locals.var_qbmm)), (((locals.var_t0_dn7 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn7)) / (locals.var_qbmm * locals.var_qbmm)), (((locals.var_t0_dn8 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn8)) / (locals.var_qbmm * locals.var_qbmm)), (((locals.var_t0_dn9 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn9)) / (locals.var_qbmm * locals.var_qbmm)), (((locals.var_t0_dn10 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn10)) / (locals.var_qbmm * locals.var_qbmm)), (((locals.var_t0_dn11 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn11)) / (locals.var_qbmm * locals.var_qbmm)), (((locals.var_t0_dn14 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn14)) / (locals.var_qbmm * locals.var_qbmm)),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign23120_e17750;
        locals.var_t9_dn0 = assign23120_e17750_d_n0;
        locals.var_t9_dn2 = assign23120_e17750_d_n2;
        locals.var_t9_dn4 = assign23120_e17750_d_n4;
        locals.var_t9_dn5 = assign23120_e17750_d_n5;
        locals.var_t9_dn6 = assign23120_e17750_d_n6;
        locals.var_t9_dn7 = assign23120_e17750_d_n7;
        locals.var_t9_dn8 = assign23120_e17750_d_n8;
        locals.var_t9_dn9 = assign23120_e17750_d_n9;
        locals.var_t9_dn10 = assign23120_e17750_d_n10;
        locals.var_t9_dn11 = assign23120_e17750_d_n11;
        locals.var_t9_dn14 = assign23120_e17750_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign23130_e17758, assign23130_e17758_d_n0, assign23130_e17758_d_n2, assign23130_e17758_d_n4, assign23130_e17758_d_n5, assign23130_e17758_d_n6, assign23130_e17758_d_n7, assign23130_e17758_d_n8, assign23130_e17758_d_n9, assign23130_e17758_d_n10, assign23130_e17758_d_n11, assign23130_e17758_d_n14,) = {
    if (locals.var_guard434 != 0.0) {
        let assign23130_e17754: f64 = (locals.var_qb0 - locals.var_qbmm);
        let assign23130_e17756: f64 = (assign23130_e17754 * locals.var_cox_inv);
        (assign23130_e17756, (((locals.var_qb0_dn0 - locals.var_qbmm_dn0) * locals.var_cox_inv) + (assign23130_e17754 * locals.var_cox_inv_dn0)), (((locals.var_qb0_dn2 - locals.var_qbmm_dn2) * locals.var_cox_inv) + (assign23130_e17754 * locals.var_cox_inv_dn2)), (((locals.var_qb0_dn4 - locals.var_qbmm_dn4) * locals.var_cox_inv) + (assign23130_e17754 * locals.var_cox_inv_dn4)), (((locals.var_qb0_dn5 - locals.var_qbmm_dn5) * locals.var_cox_inv) + (assign23130_e17754 * locals.var_cox_inv_dn5)), (((locals.var_qb0_dn6 - locals.var_qbmm_dn6) * locals.var_cox_inv) + (assign23130_e17754 * locals.var_cox_inv_dn6)), (((locals.var_qb0_dn7 - locals.var_qbmm_dn7) * locals.var_cox_inv) + (assign23130_e17754 * locals.var_cox_inv_dn7)), (((locals.var_qb0_dn8 - locals.var_qbmm_dn8) * locals.var_cox_inv) + (assign23130_e17754 * locals.var_cox_inv_dn8)), (((locals.var_qb0_dn9 - locals.var_qbmm_dn9) * locals.var_cox_inv) + (assign23130_e17754 * locals.var_cox_inv_dn9)), (((locals.var_qb0_dn10 - locals.var_qbmm_dn10) * locals.var_cox_inv) + (assign23130_e17754 * locals.var_cox_inv_dn10)), (((locals.var_qb0_dn11 - locals.var_qbmm_dn11) * locals.var_cox_inv) + (assign23130_e17754 * locals.var_cox_inv_dn11)), (((locals.var_qb0_dn14 - locals.var_qbmm_dn14) * locals.var_cox_inv) + (assign23130_e17754 * locals.var_cox_inv_dn14)),)
    } else {
        (locals.var_dqb, locals.var_dqb_dn0, locals.var_dqb_dn2, locals.var_dqb_dn4, locals.var_dqb_dn5, locals.var_dqb_dn6, locals.var_dqb_dn7, locals.var_dqb_dn8, locals.var_dqb_dn9, locals.var_dqb_dn10, locals.var_dqb_dn11, locals.var_dqb_dn14,)
    }
};
        locals.var_dqb = assign23130_e17758;
        locals.var_dqb_dn0 = assign23130_e17758_d_n0;
        locals.var_dqb_dn2 = assign23130_e17758_d_n2;
        locals.var_dqb_dn4 = assign23130_e17758_d_n4;
        locals.var_dqb_dn5 = assign23130_e17758_d_n5;
        locals.var_dqb_dn6 = assign23130_e17758_d_n6;
        locals.var_dqb_dn7 = assign23130_e17758_d_n7;
        locals.var_dqb_dn8 = assign23130_e17758_d_n8;
        locals.var_dqb_dn9 = assign23130_e17758_d_n9;
        locals.var_dqb_dn10 = assign23130_e17758_d_n10;
        locals.var_dqb_dn11 = assign23130_e17758_d_n11;
        locals.var_dqb_dn14 = assign23130_e17758_d_n14;
        locals.var_dqb_rv = 0.0;

        let (assign23140_e17768, assign23140_e17768_d_n0, assign23140_e17768_d_n2, assign23140_e17768_d_n4, assign23140_e17768_d_n5, assign23140_e17768_d_n6, assign23140_e17768_d_n7, assign23140_e17768_d_n8, assign23140_e17768_d_n9, assign23140_e17768_d_n10, assign23140_e17768_d_n11, assign23140_e17768_d_n14,) = {
    if (locals.var_guard434 != 0.0) {
        let assign23140_e17762: f64 = (2.0 * 1.6021918e-19);
        let assign23140_e17764: f64 = (assign23140_e17762 * locals.var_ef_nsubc);
        let assign23140_e17766: f64 = (assign23140_e17764 * 1.034943e-10);
        (assign23140_e17766, ((assign23140_e17762 * locals.var_ef_nsubc_dn0) * 1.034943e-10), ((assign23140_e17762 * locals.var_ef_nsubc_dn2) * 1.034943e-10), ((assign23140_e17762 * locals.var_ef_nsubc_dn4) * 1.034943e-10), ((assign23140_e17762 * locals.var_ef_nsubc_dn5) * 1.034943e-10), ((assign23140_e17762 * locals.var_ef_nsubc_dn6) * 1.034943e-10), ((assign23140_e17762 * locals.var_ef_nsubc_dn7) * 1.034943e-10), ((assign23140_e17762 * locals.var_ef_nsubc_dn8) * 1.034943e-10), ((assign23140_e17762 * locals.var_ef_nsubc_dn9) * 1.034943e-10), ((assign23140_e17762 * locals.var_ef_nsubc_dn10) * 1.034943e-10), ((assign23140_e17762 * locals.var_ef_nsubc_dn11) * 1.034943e-10), ((assign23140_e17762 * locals.var_ef_nsubc_dn14) * 1.034943e-10),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign23140_e17768;
        locals.var_t1_dn0 = assign23140_e17768_d_n0;
        locals.var_t1_dn2 = assign23140_e17768_d_n2;
        locals.var_t1_dn4 = assign23140_e17768_d_n4;
        locals.var_t1_dn5 = assign23140_e17768_d_n5;
        locals.var_t1_dn6 = assign23140_e17768_d_n6;
        locals.var_t1_dn7 = assign23140_e17768_d_n7;
        locals.var_t1_dn8 = assign23140_e17768_d_n8;
        locals.var_t1_dn9 = assign23140_e17768_d_n9;
        locals.var_t1_dn10 = assign23140_e17768_d_n10;
        locals.var_t1_dn11 = assign23140_e17768_d_n11;
        locals.var_t1_dn14 = assign23140_e17768_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign23150_e17777, assign23150_e17777_d_n0, assign23150_e17777_d_n2, assign23150_e17777_d_n4, assign23150_e17777_d_n5, assign23150_e17777_d_n6, assign23150_e17777_d_n7, assign23150_e17777_d_n8, assign23150_e17777_d_n9, assign23150_e17777_d_n10, assign23150_e17777_d_n11, assign23150_e17777_d_n14,) = {
    if (locals.var_guard434 != 0.0) {
        let assign23150_e17773: f64 = (locals.var_pb2c - locals.var_vbsz2);
        let assign23150_e17774: f64 = (locals.var_t1 * assign23150_e17773);
        let assign23150_e17775: f64 = (assign23150_e17774).sqrt();
        (assign23150_e17775, (((locals.var_t1_dn0 * assign23150_e17773) + (locals.var_t1 * (locals.var_pb2c_dn0 - locals.var_vbsz2_dn0))) / (2.0 * assign23150_e17775)), (((locals.var_t1_dn2 * assign23150_e17773) + (locals.var_t1 * (locals.var_pb2c_dn2 - locals.var_vbsz2_dn2))) / (2.0 * assign23150_e17775)), (((locals.var_t1_dn4 * assign23150_e17773) + (locals.var_t1 * (locals.var_pb2c_dn4 - locals.var_vbsz2_dn4))) / (2.0 * assign23150_e17775)), (((locals.var_t1_dn5 * assign23150_e17773) + (locals.var_t1 * (locals.var_pb2c_dn5 - locals.var_vbsz2_dn5))) / (2.0 * assign23150_e17775)), (((locals.var_t1_dn6 * assign23150_e17773) + (locals.var_t1 * (locals.var_pb2c_dn6 - locals.var_vbsz2_dn6))) / (2.0 * assign23150_e17775)), (((locals.var_t1_dn7 * assign23150_e17773) + (locals.var_t1 * (locals.var_pb2c_dn7 - locals.var_vbsz2_dn7))) / (2.0 * assign23150_e17775)), (((locals.var_t1_dn8 * assign23150_e17773) + (locals.var_t1 * (locals.var_pb2c_dn8 - locals.var_vbsz2_dn8))) / (2.0 * assign23150_e17775)), (((locals.var_t1_dn9 * assign23150_e17773) + (locals.var_t1 * (locals.var_pb2c_dn9 - locals.var_vbsz2_dn9))) / (2.0 * assign23150_e17775)), (((locals.var_t1_dn10 * assign23150_e17773) + (locals.var_t1 * (locals.var_pb2c_dn10 - locals.var_vbsz2_dn10))) / (2.0 * assign23150_e17775)), (((locals.var_t1_dn11 * assign23150_e17773) + (locals.var_t1 * (locals.var_pb2c_dn11 - locals.var_vbsz2_dn11))) / (2.0 * assign23150_e17775)), (((locals.var_t1_dn14 * assign23150_e17773) + (locals.var_t1 * (locals.var_pb2c_dn14 - locals.var_vbsz2_dn14))) / (2.0 * assign23150_e17775)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign23150_e17777;
        locals.var_t2_dn0 = assign23150_e17777_d_n0;
        locals.var_t2_dn2 = assign23150_e17777_d_n2;
        locals.var_t2_dn4 = assign23150_e17777_d_n4;
        locals.var_t2_dn5 = assign23150_e17777_d_n5;
        locals.var_t2_dn6 = assign23150_e17777_d_n6;
        locals.var_t2_dn7 = assign23150_e17777_d_n7;
        locals.var_t2_dn8 = assign23150_e17777_d_n8;
        locals.var_t2_dn9 = assign23150_e17777_d_n9;
        locals.var_t2_dn10 = assign23150_e17777_d_n10;
        locals.var_t2_dn11 = assign23150_e17777_d_n11;
        locals.var_t2_dn14 = assign23150_e17777_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign23160_e17787, assign23160_e17787_d_n0, assign23160_e17787_d_n2, assign23160_e17787_d_n4, assign23160_e17787_d_n5, assign23160_e17787_d_n6, assign23160_e17787_d_n7, assign23160_e17787_d_n8, assign23160_e17787_d_n9, assign23160_e17787_d_n10, assign23160_e17787_d_n11, assign23160_e17787_d_n14,) = {
    if (locals.var_guard434 != 0.0) {
        let assign23160_e17781: f64 = (locals.var_pb2c + locals.var_vfb);
        let assign23160_e17784: f64 = (locals.var_t2 * locals.var_cox_inv);
        let assign23160_e17785: f64 = (assign23160_e17781 + assign23160_e17784);
        (assign23160_e17785, (locals.var_pb2c_dn0 + ((locals.var_t2_dn0 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn0))), (locals.var_pb2c_dn2 + ((locals.var_t2_dn2 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn2))), (locals.var_pb2c_dn4 + ((locals.var_t2_dn4 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn4))), (locals.var_pb2c_dn5 + ((locals.var_t2_dn5 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn5))), (locals.var_pb2c_dn6 + ((locals.var_t2_dn6 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn6))), (locals.var_pb2c_dn7 + ((locals.var_t2_dn7 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn7))), (locals.var_pb2c_dn8 + ((locals.var_t2_dn8 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn8))), (locals.var_pb2c_dn9 + ((locals.var_t2_dn9 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn9))), (locals.var_pb2c_dn10 + ((locals.var_t2_dn10 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn10))), (locals.var_pb2c_dn11 + ((locals.var_t2_dn11 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn11))), (locals.var_pb2c_dn14 + ((locals.var_t2_dn14 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn14))),)
    } else {
        (locals.var_vth0, locals.var_vth0_dn0, locals.var_vth0_dn2, locals.var_vth0_dn4, locals.var_vth0_dn5, locals.var_vth0_dn6, locals.var_vth0_dn7, locals.var_vth0_dn8, locals.var_vth0_dn9, locals.var_vth0_dn10, locals.var_vth0_dn11, locals.var_vth0_dn14,)
    }
};
        locals.var_vth0 = assign23160_e17787;
        locals.var_vth0_dn0 = assign23160_e17787_d_n0;
        locals.var_vth0_dn2 = assign23160_e17787_d_n2;
        locals.var_vth0_dn4 = assign23160_e17787_d_n4;
        locals.var_vth0_dn5 = assign23160_e17787_d_n5;
        locals.var_vth0_dn6 = assign23160_e17787_d_n6;
        locals.var_vth0_dn7 = assign23160_e17787_d_n7;
        locals.var_vth0_dn8 = assign23160_e17787_d_n8;
        locals.var_vth0_dn9 = assign23160_e17787_d_n9;
        locals.var_vth0_dn10 = assign23160_e17787_d_n10;
        locals.var_vth0_dn11 = assign23160_e17787_d_n11;
        locals.var_vth0_dn14 = assign23160_e17787_d_n14;
        locals.var_vth0_rv = 0.0;

        let (assign23170_e17797, assign23170_e17797_d_n0, assign23170_e17797_d_n2, assign23170_e17797_d_n4, assign23170_e17797_d_n5, assign23170_e17797_d_n6, assign23170_e17797_d_n7, assign23170_e17797_d_n8, assign23170_e17797_d_n9, assign23170_e17797_d_n10, assign23170_e17797_d_n11, assign23170_e17797_d_n14,) = {
    if (locals.var_guard434 != 0.0) {
        let assign23170_e17791: f64 = (0.5 * locals.var_t1);
        let assign23170_e17793: f64 = (assign23170_e17791 / locals.var_t2);
        let assign23170_e17795: f64 = (assign23170_e17793 * locals.var_cox_inv);
        (assign23170_e17795, ((((((0.5 * locals.var_t1_dn0) * locals.var_t2) - (assign23170_e17791 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23170_e17793 * locals.var_cox_inv_dn0)), ((((((0.5 * locals.var_t1_dn2) * locals.var_t2) - (assign23170_e17791 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23170_e17793 * locals.var_cox_inv_dn2)), ((((((0.5 * locals.var_t1_dn4) * locals.var_t2) - (assign23170_e17791 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23170_e17793 * locals.var_cox_inv_dn4)), ((((((0.5 * locals.var_t1_dn5) * locals.var_t2) - (assign23170_e17791 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23170_e17793 * locals.var_cox_inv_dn5)), ((((((0.5 * locals.var_t1_dn6) * locals.var_t2) - (assign23170_e17791 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23170_e17793 * locals.var_cox_inv_dn6)), ((((((0.5 * locals.var_t1_dn7) * locals.var_t2) - (assign23170_e17791 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23170_e17793 * locals.var_cox_inv_dn7)), ((((((0.5 * locals.var_t1_dn8) * locals.var_t2) - (assign23170_e17791 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23170_e17793 * locals.var_cox_inv_dn8)), ((((((0.5 * locals.var_t1_dn9) * locals.var_t2) - (assign23170_e17791 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23170_e17793 * locals.var_cox_inv_dn9)), ((((((0.5 * locals.var_t1_dn10) * locals.var_t2) - (assign23170_e17791 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23170_e17793 * locals.var_cox_inv_dn10)), ((((((0.5 * locals.var_t1_dn11) * locals.var_t2) - (assign23170_e17791 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23170_e17793 * locals.var_cox_inv_dn11)), ((((((0.5 * locals.var_t1_dn14) * locals.var_t2) - (assign23170_e17791 * locals.var_t2_dn14)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23170_e17793 * locals.var_cox_inv_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign23170_e17797;
        locals.var_t3_dn0 = assign23170_e17797_d_n0;
        locals.var_t3_dn2 = assign23170_e17797_d_n2;
        locals.var_t3_dn4 = assign23170_e17797_d_n4;
        locals.var_t3_dn5 = assign23170_e17797_d_n5;
        locals.var_t3_dn6 = assign23170_e17797_d_n6;
        locals.var_t3_dn7 = assign23170_e17797_d_n7;
        locals.var_t3_dn8 = assign23170_e17797_d_n8;
        locals.var_t3_dn9 = assign23170_e17797_d_n9;
        locals.var_t3_dn10 = assign23170_e17797_d_n10;
        locals.var_t3_dn11 = assign23170_e17797_d_n11;
        locals.var_t3_dn14 = assign23170_e17797_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign23180_e17803, assign23180_e17803_d_n0, assign23180_e17803_d_n2, assign23180_e17803_d_n4, assign23180_e17803_d_n5, assign23180_e17803_d_n6, assign23180_e17803_d_n7, assign23180_e17803_d_n8, assign23180_e17803_d_n9, assign23180_e17803_d_n10, assign23180_e17803_d_n11, assign23180_e17803_d_n14,) = {
    if (locals.var_guard434 != 0.0) {
        let assign23180_e17801: f64 = (1.034943e-10 * locals.var_cox_inv);
        (assign23180_e17801, (1.034943e-10 * locals.var_cox_inv_dn0), (1.034943e-10 * locals.var_cox_inv_dn2), (1.034943e-10 * locals.var_cox_inv_dn4), (1.034943e-10 * locals.var_cox_inv_dn5), (1.034943e-10 * locals.var_cox_inv_dn6), (1.034943e-10 * locals.var_cox_inv_dn7), (1.034943e-10 * locals.var_cox_inv_dn8), (1.034943e-10 * locals.var_cox_inv_dn9), (1.034943e-10 * locals.var_cox_inv_dn10), (1.034943e-10 * locals.var_cox_inv_dn11), (1.034943e-10 * locals.var_cox_inv_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign23180_e17803;
        locals.var_t1_dn0 = assign23180_e17803_d_n0;
        locals.var_t1_dn2 = assign23180_e17803_d_n2;
        locals.var_t1_dn4 = assign23180_e17803_d_n4;
        locals.var_t1_dn5 = assign23180_e17803_d_n5;
        locals.var_t1_dn6 = assign23180_e17803_d_n6;
        locals.var_t1_dn7 = assign23180_e17803_d_n7;
        locals.var_t1_dn8 = assign23180_e17803_d_n8;
        locals.var_t1_dn9 = assign23180_e17803_d_n9;
        locals.var_t1_dn10 = assign23180_e17803_d_n10;
        locals.var_t1_dn11 = assign23180_e17803_d_n11;
        locals.var_t1_dn14 = assign23180_e17803_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign23190_e17807, assign23190_e17807_d_n0, assign23190_e17807_d_n2, assign23190_e17807_d_n4, assign23190_e17807_d_n5, assign23190_e17807_d_n6, assign23190_e17807_d_n7, assign23190_e17807_d_n8, assign23190_e17807_d_n9, assign23190_e17807_d_n10, assign23190_e17807_d_n11, assign23190_e17807_d_n14,) = {
    if (locals.var_guard434 != 0.0) {
        (locals.var_wdplp, locals.var_wdplp_dn0, locals.var_wdplp_dn2, locals.var_wdplp_dn4, locals.var_wdplp_dn5, locals.var_wdplp_dn6, locals.var_wdplp_dn7, locals.var_wdplp_dn8, locals.var_wdplp_dn9, locals.var_wdplp_dn10, locals.var_wdplp_dn11, locals.var_wdplp_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign23190_e17807;
        locals.var_t2_dn0 = assign23190_e17807_d_n0;
        locals.var_t2_dn2 = assign23190_e17807_d_n2;
        locals.var_t2_dn4 = assign23190_e17807_d_n4;
        locals.var_t2_dn5 = assign23190_e17807_d_n5;
        locals.var_t2_dn6 = assign23190_e17807_d_n6;
        locals.var_t2_dn7 = assign23190_e17807_d_n7;
        locals.var_t2_dn8 = assign23190_e17807_d_n8;
        locals.var_t2_dn9 = assign23190_e17807_d_n9;
        locals.var_t2_dn10 = assign23190_e17807_d_n10;
        locals.var_t2_dn11 = assign23190_e17807_d_n11;
        locals.var_t2_dn14 = assign23190_e17807_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign23200_e17815, assign23200_e17815_d_n0, assign23200_e17815_d_n2, assign23200_e17815_d_n4, assign23200_e17815_d_n5, assign23200_e17815_d_n6, assign23200_e17815_d_n7, assign23200_e17815_d_n8, assign23200_e17815_d_n9, assign23200_e17815_d_n10, assign23200_e17815_d_n11, assign23200_e17815_d_n14,) = {
    if (locals.var_guard434 != 0.0) {
        let assign23200_e17812: f64 = (p.p140 * p.p140);
        let assign23200_e17813: f64 = (1.0 / assign23200_e17812);
        (assign23200_e17813, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign23200_e17815;
        locals.var_t4_dn0 = assign23200_e17815_d_n0;
        locals.var_t4_dn2 = assign23200_e17815_d_n2;
        locals.var_t4_dn4 = assign23200_e17815_d_n4;
        locals.var_t4_dn5 = assign23200_e17815_d_n5;
        locals.var_t4_dn6 = assign23200_e17815_d_n6;
        locals.var_t4_dn7 = assign23200_e17815_d_n7;
        locals.var_t4_dn8 = assign23200_e17815_d_n8;
        locals.var_t4_dn9 = assign23200_e17815_d_n9;
        locals.var_t4_dn10 = assign23200_e17815_d_n10;
        locals.var_t4_dn11 = assign23200_e17815_d_n11;
        locals.var_t4_dn14 = assign23200_e17815_d_n14;
        locals.var_t4_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_63(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23210_e17829, assign23210_e17829_d_n0, assign23210_e17829_d_n2, assign23210_e17829_d_n4, assign23210_e17829_d_n5, assign23210_e17829_d_n6, assign23210_e17829_d_n7, assign23210_e17829_d_n8, assign23210_e17829_d_n9, assign23210_e17829_d_n10, assign23210_e17829_d_n11, assign23210_e17829_d_n14,) = {
    if (locals.var_guard434 != 0.0) {
        let assign23210_e17820: f64 = (p.p137 - locals.var_pb20b);
        let assign23210_e17821: f64 = (2.0 * assign23210_e17820);
        let assign23210_e17823: f64 = (assign23210_e17821 * locals.var_t1);
        let assign23210_e17825: f64 = (assign23210_e17823 * locals.var_t2);
        let assign23210_e17827: f64 = (assign23210_e17825 * locals.var_t4);
        (assign23210_e17827, (((((((2.0 * (-locals.var_pb20b_dn0)) * locals.var_t1) + (assign23210_e17821 * locals.var_t1_dn0)) * locals.var_t2) + (assign23210_e17823 * locals.var_t2_dn0)) * locals.var_t4) + (assign23210_e17825 * locals.var_t4_dn0)), (((((((2.0 * (-locals.var_pb20b_dn2)) * locals.var_t1) + (assign23210_e17821 * locals.var_t1_dn2)) * locals.var_t2) + (assign23210_e17823 * locals.var_t2_dn2)) * locals.var_t4) + (assign23210_e17825 * locals.var_t4_dn2)), (((((((2.0 * (-locals.var_pb20b_dn4)) * locals.var_t1) + (assign23210_e17821 * locals.var_t1_dn4)) * locals.var_t2) + (assign23210_e17823 * locals.var_t2_dn4)) * locals.var_t4) + (assign23210_e17825 * locals.var_t4_dn4)), (((((((2.0 * (-locals.var_pb20b_dn5)) * locals.var_t1) + (assign23210_e17821 * locals.var_t1_dn5)) * locals.var_t2) + (assign23210_e17823 * locals.var_t2_dn5)) * locals.var_t4) + (assign23210_e17825 * locals.var_t4_dn5)), (((((((2.0 * (-locals.var_pb20b_dn6)) * locals.var_t1) + (assign23210_e17821 * locals.var_t1_dn6)) * locals.var_t2) + (assign23210_e17823 * locals.var_t2_dn6)) * locals.var_t4) + (assign23210_e17825 * locals.var_t4_dn6)), (((((((2.0 * (-locals.var_pb20b_dn7)) * locals.var_t1) + (assign23210_e17821 * locals.var_t1_dn7)) * locals.var_t2) + (assign23210_e17823 * locals.var_t2_dn7)) * locals.var_t4) + (assign23210_e17825 * locals.var_t4_dn7)), (((((((2.0 * (-locals.var_pb20b_dn8)) * locals.var_t1) + (assign23210_e17821 * locals.var_t1_dn8)) * locals.var_t2) + (assign23210_e17823 * locals.var_t2_dn8)) * locals.var_t4) + (assign23210_e17825 * locals.var_t4_dn8)), (((((((2.0 * (-locals.var_pb20b_dn9)) * locals.var_t1) + (assign23210_e17821 * locals.var_t1_dn9)) * locals.var_t2) + (assign23210_e17823 * locals.var_t2_dn9)) * locals.var_t4) + (assign23210_e17825 * locals.var_t4_dn9)), (((((((2.0 * (-locals.var_pb20b_dn10)) * locals.var_t1) + (assign23210_e17821 * locals.var_t1_dn10)) * locals.var_t2) + (assign23210_e17823 * locals.var_t2_dn10)) * locals.var_t4) + (assign23210_e17825 * locals.var_t4_dn10)), (((((((2.0 * (-locals.var_pb20b_dn11)) * locals.var_t1) + (assign23210_e17821 * locals.var_t1_dn11)) * locals.var_t2) + (assign23210_e17823 * locals.var_t2_dn11)) * locals.var_t4) + (assign23210_e17825 * locals.var_t4_dn11)), (((((((2.0 * (-locals.var_pb20b_dn14)) * locals.var_t1) + (assign23210_e17821 * locals.var_t1_dn14)) * locals.var_t2) + (assign23210_e17823 * locals.var_t2_dn14)) * locals.var_t4) + (assign23210_e17825 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign23210_e17829;
        locals.var_t5_dn0 = assign23210_e17829_d_n0;
        locals.var_t5_dn2 = assign23210_e17829_d_n2;
        locals.var_t5_dn4 = assign23210_e17829_d_n4;
        locals.var_t5_dn5 = assign23210_e17829_d_n5;
        locals.var_t5_dn6 = assign23210_e17829_d_n6;
        locals.var_t5_dn7 = assign23210_e17829_d_n7;
        locals.var_t5_dn8 = assign23210_e17829_d_n8;
        locals.var_t5_dn9 = assign23210_e17829_d_n9;
        locals.var_t5_dn10 = assign23210_e17829_d_n10;
        locals.var_t5_dn11 = assign23210_e17829_d_n11;
        locals.var_t5_dn14 = assign23210_e17829_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign23220_e17835, assign23220_e17835_d_n0, assign23220_e17835_d_n2, assign23220_e17835_d_n4, assign23220_e17835_d_n5, assign23220_e17835_d_n6, assign23220_e17835_d_n7, assign23220_e17835_d_n8, assign23220_e17835_d_n9, assign23220_e17835_d_n10, assign23220_e17835_d_n11, assign23220_e17835_d_n14,) = {
    if (locals.var_guard434 != 0.0) {
        let assign23220_e17833: f64 = (locals.var_t5 * locals.var_sqrt_pbsum);
        (assign23220_e17833, ((locals.var_t5_dn0 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn0)), ((locals.var_t5_dn2 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn2)), ((locals.var_t5_dn4 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn4)), ((locals.var_t5_dn5 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn5)), ((locals.var_t5_dn6 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn6)), ((locals.var_t5_dn7 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn7)), ((locals.var_t5_dn8 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn8)), ((locals.var_t5_dn9 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn9)), ((locals.var_t5_dn10 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn10)), ((locals.var_t5_dn11 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn11)), ((locals.var_t5_dn14 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn14)),)
    } else {
        (locals.var_dvth0, locals.var_dvth0_dn0, locals.var_dvth0_dn2, locals.var_dvth0_dn4, locals.var_dvth0_dn5, locals.var_dvth0_dn6, locals.var_dvth0_dn7, locals.var_dvth0_dn8, locals.var_dvth0_dn9, locals.var_dvth0_dn10, locals.var_dvth0_dn11, locals.var_dvth0_dn14,)
    }
};
        locals.var_dvth0 = assign23220_e17835;
        locals.var_dvth0_dn0 = assign23220_e17835_d_n0;
        locals.var_dvth0_dn2 = assign23220_e17835_d_n2;
        locals.var_dvth0_dn4 = assign23220_e17835_d_n4;
        locals.var_dvth0_dn5 = assign23220_e17835_d_n5;
        locals.var_dvth0_dn6 = assign23220_e17835_d_n6;
        locals.var_dvth0_dn7 = assign23220_e17835_d_n7;
        locals.var_dvth0_dn8 = assign23220_e17835_d_n8;
        locals.var_dvth0_dn9 = assign23220_e17835_d_n9;
        locals.var_dvth0_dn10 = assign23220_e17835_d_n10;
        locals.var_dvth0_dn11 = assign23220_e17835_d_n11;
        locals.var_dvth0_dn14 = assign23220_e17835_d_n14;
        locals.var_dvth0_rv = 0.0;

        let (assign23230_e17843, assign23230_e17843_d_n0, assign23230_e17843_d_n2, assign23230_e17843_d_n4, assign23230_e17843_d_n5, assign23230_e17843_d_n6, assign23230_e17843_d_n7, assign23230_e17843_d_n8, assign23230_e17843_d_n9, assign23230_e17843_d_n10, assign23230_e17843_d_n11, assign23230_e17843_d_n14,) = {
    if (locals.var_guard434 != 0.0) {
        let assign23230_e17839: f64 = (0.5 * locals.var_t5);
        let assign23230_e17841: f64 = (assign23230_e17839 / locals.var_sqrt_pbsum);
        (assign23230_e17841, ((((0.5 * locals.var_t5_dn0) * locals.var_sqrt_pbsum) - (assign23230_e17839 * locals.var_sqrt_pbsum_dn0)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((0.5 * locals.var_t5_dn2) * locals.var_sqrt_pbsum) - (assign23230_e17839 * locals.var_sqrt_pbsum_dn2)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((0.5 * locals.var_t5_dn4) * locals.var_sqrt_pbsum) - (assign23230_e17839 * locals.var_sqrt_pbsum_dn4)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((0.5 * locals.var_t5_dn5) * locals.var_sqrt_pbsum) - (assign23230_e17839 * locals.var_sqrt_pbsum_dn5)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((0.5 * locals.var_t5_dn6) * locals.var_sqrt_pbsum) - (assign23230_e17839 * locals.var_sqrt_pbsum_dn6)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((0.5 * locals.var_t5_dn7) * locals.var_sqrt_pbsum) - (assign23230_e17839 * locals.var_sqrt_pbsum_dn7)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((0.5 * locals.var_t5_dn8) * locals.var_sqrt_pbsum) - (assign23230_e17839 * locals.var_sqrt_pbsum_dn8)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((0.5 * locals.var_t5_dn9) * locals.var_sqrt_pbsum) - (assign23230_e17839 * locals.var_sqrt_pbsum_dn9)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((0.5 * locals.var_t5_dn10) * locals.var_sqrt_pbsum) - (assign23230_e17839 * locals.var_sqrt_pbsum_dn10)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((0.5 * locals.var_t5_dn11) * locals.var_sqrt_pbsum) - (assign23230_e17839 * locals.var_sqrt_pbsum_dn11)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((0.5 * locals.var_t5_dn14) * locals.var_sqrt_pbsum) - (assign23230_e17839 * locals.var_sqrt_pbsum_dn14)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign23230_e17843;
        locals.var_t6_dn0 = assign23230_e17843_d_n0;
        locals.var_t6_dn2 = assign23230_e17843_d_n2;
        locals.var_t6_dn4 = assign23230_e17843_d_n4;
        locals.var_t6_dn5 = assign23230_e17843_d_n5;
        locals.var_t6_dn6 = assign23230_e17843_d_n6;
        locals.var_t6_dn7 = assign23230_e17843_d_n7;
        locals.var_t6_dn8 = assign23230_e17843_d_n8;
        locals.var_t6_dn9 = assign23230_e17843_d_n9;
        locals.var_t6_dn10 = assign23230_e17843_d_n10;
        locals.var_t6_dn11 = assign23230_e17843_d_n11;
        locals.var_t6_dn14 = assign23230_e17843_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign23240_e17859, assign23240_e17859_d_n0, assign23240_e17859_d_n2, assign23240_e17859_d_n4, assign23240_e17859_d_n5, assign23240_e17859_d_n6, assign23240_e17859_d_n7, assign23240_e17859_d_n8, assign23240_e17859_d_n9, assign23240_e17859_d_n10, assign23240_e17859_d_n11, assign23240_e17859_d_n14,) = {
    if (locals.var_guard434 != 0.0) {
        let assign23240_e17848: f64 = (p.p137 - locals.var_pb20b);
        let assign23240_e17849: f64 = (2.0 * assign23240_e17848);
        let assign23240_e17851: f64 = (assign23240_e17849 * 1.034943e-10);
        let assign23240_e17853: f64 = (assign23240_e17851 * locals.var_t2);
        let assign23240_e17855: f64 = (assign23240_e17853 * locals.var_t4);
        let assign23240_e17857: f64 = (assign23240_e17855 * locals.var_sqrt_pbsum);
        (assign23240_e17857, ((((((((2.0 * (-locals.var_pb20b_dn0)) * 1.034943e-10) * locals.var_t2) + (assign23240_e17851 * locals.var_t2_dn0)) * locals.var_t4) + (assign23240_e17853 * locals.var_t4_dn0)) * locals.var_sqrt_pbsum) + (assign23240_e17855 * locals.var_sqrt_pbsum_dn0)), ((((((((2.0 * (-locals.var_pb20b_dn2)) * 1.034943e-10) * locals.var_t2) + (assign23240_e17851 * locals.var_t2_dn2)) * locals.var_t4) + (assign23240_e17853 * locals.var_t4_dn2)) * locals.var_sqrt_pbsum) + (assign23240_e17855 * locals.var_sqrt_pbsum_dn2)), ((((((((2.0 * (-locals.var_pb20b_dn4)) * 1.034943e-10) * locals.var_t2) + (assign23240_e17851 * locals.var_t2_dn4)) * locals.var_t4) + (assign23240_e17853 * locals.var_t4_dn4)) * locals.var_sqrt_pbsum) + (assign23240_e17855 * locals.var_sqrt_pbsum_dn4)), ((((((((2.0 * (-locals.var_pb20b_dn5)) * 1.034943e-10) * locals.var_t2) + (assign23240_e17851 * locals.var_t2_dn5)) * locals.var_t4) + (assign23240_e17853 * locals.var_t4_dn5)) * locals.var_sqrt_pbsum) + (assign23240_e17855 * locals.var_sqrt_pbsum_dn5)), ((((((((2.0 * (-locals.var_pb20b_dn6)) * 1.034943e-10) * locals.var_t2) + (assign23240_e17851 * locals.var_t2_dn6)) * locals.var_t4) + (assign23240_e17853 * locals.var_t4_dn6)) * locals.var_sqrt_pbsum) + (assign23240_e17855 * locals.var_sqrt_pbsum_dn6)), ((((((((2.0 * (-locals.var_pb20b_dn7)) * 1.034943e-10) * locals.var_t2) + (assign23240_e17851 * locals.var_t2_dn7)) * locals.var_t4) + (assign23240_e17853 * locals.var_t4_dn7)) * locals.var_sqrt_pbsum) + (assign23240_e17855 * locals.var_sqrt_pbsum_dn7)), ((((((((2.0 * (-locals.var_pb20b_dn8)) * 1.034943e-10) * locals.var_t2) + (assign23240_e17851 * locals.var_t2_dn8)) * locals.var_t4) + (assign23240_e17853 * locals.var_t4_dn8)) * locals.var_sqrt_pbsum) + (assign23240_e17855 * locals.var_sqrt_pbsum_dn8)), ((((((((2.0 * (-locals.var_pb20b_dn9)) * 1.034943e-10) * locals.var_t2) + (assign23240_e17851 * locals.var_t2_dn9)) * locals.var_t4) + (assign23240_e17853 * locals.var_t4_dn9)) * locals.var_sqrt_pbsum) + (assign23240_e17855 * locals.var_sqrt_pbsum_dn9)), ((((((((2.0 * (-locals.var_pb20b_dn10)) * 1.034943e-10) * locals.var_t2) + (assign23240_e17851 * locals.var_t2_dn10)) * locals.var_t4) + (assign23240_e17853 * locals.var_t4_dn10)) * locals.var_sqrt_pbsum) + (assign23240_e17855 * locals.var_sqrt_pbsum_dn10)), ((((((((2.0 * (-locals.var_pb20b_dn11)) * 1.034943e-10) * locals.var_t2) + (assign23240_e17851 * locals.var_t2_dn11)) * locals.var_t4) + (assign23240_e17853 * locals.var_t4_dn11)) * locals.var_sqrt_pbsum) + (assign23240_e17855 * locals.var_sqrt_pbsum_dn11)), ((((((((2.0 * (-locals.var_pb20b_dn14)) * 1.034943e-10) * locals.var_t2) + (assign23240_e17851 * locals.var_t2_dn14)) * locals.var_t4) + (assign23240_e17853 * locals.var_t4_dn14)) * locals.var_sqrt_pbsum) + (assign23240_e17855 * locals.var_sqrt_pbsum_dn14)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign23240_e17859;
        locals.var_t7_dn0 = assign23240_e17859_d_n0;
        locals.var_t7_dn2 = assign23240_e17859_d_n2;
        locals.var_t7_dn4 = assign23240_e17859_d_n4;
        locals.var_t7_dn5 = assign23240_e17859_d_n5;
        locals.var_t7_dn6 = assign23240_e17859_d_n6;
        locals.var_t7_dn7 = assign23240_e17859_d_n7;
        locals.var_t7_dn8 = assign23240_e17859_d_n8;
        locals.var_t7_dn9 = assign23240_e17859_d_n9;
        locals.var_t7_dn10 = assign23240_e17859_d_n10;
        locals.var_t7_dn11 = assign23240_e17859_d_n11;
        locals.var_t7_dn14 = assign23240_e17859_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign23250_e17872, assign23250_e17872_d_n0, assign23250_e17872_d_n2, assign23250_e17872_d_n4, assign23250_e17872_d_n5, assign23250_e17872_d_n6, assign23250_e17872_d_n7, assign23250_e17872_d_n8, assign23250_e17872_d_n9, assign23250_e17872_d_n10, assign23250_e17872_d_n11, assign23250_e17872_d_n14,) = {
    if (locals.var_guard434 != 0.0) {
        let assign23250_e17862: f64 = (-2.0);
        let assign23250_e17864: f64 = (assign23250_e17862 * locals.var_t1);
        let assign23250_e17866: f64 = (assign23250_e17864 * locals.var_t2);
        let assign23250_e17868: f64 = (assign23250_e17866 * locals.var_t4);
        let assign23250_e17870: f64 = (assign23250_e17868 * locals.var_sqrt_pbsum);
        (assign23250_e17870, (((((((assign23250_e17862 * locals.var_t1_dn0) * locals.var_t2) + (assign23250_e17864 * locals.var_t2_dn0)) * locals.var_t4) + (assign23250_e17866 * locals.var_t4_dn0)) * locals.var_sqrt_pbsum) + (assign23250_e17868 * locals.var_sqrt_pbsum_dn0)), (((((((assign23250_e17862 * locals.var_t1_dn2) * locals.var_t2) + (assign23250_e17864 * locals.var_t2_dn2)) * locals.var_t4) + (assign23250_e17866 * locals.var_t4_dn2)) * locals.var_sqrt_pbsum) + (assign23250_e17868 * locals.var_sqrt_pbsum_dn2)), (((((((assign23250_e17862 * locals.var_t1_dn4) * locals.var_t2) + (assign23250_e17864 * locals.var_t2_dn4)) * locals.var_t4) + (assign23250_e17866 * locals.var_t4_dn4)) * locals.var_sqrt_pbsum) + (assign23250_e17868 * locals.var_sqrt_pbsum_dn4)), (((((((assign23250_e17862 * locals.var_t1_dn5) * locals.var_t2) + (assign23250_e17864 * locals.var_t2_dn5)) * locals.var_t4) + (assign23250_e17866 * locals.var_t4_dn5)) * locals.var_sqrt_pbsum) + (assign23250_e17868 * locals.var_sqrt_pbsum_dn5)), (((((((assign23250_e17862 * locals.var_t1_dn6) * locals.var_t2) + (assign23250_e17864 * locals.var_t2_dn6)) * locals.var_t4) + (assign23250_e17866 * locals.var_t4_dn6)) * locals.var_sqrt_pbsum) + (assign23250_e17868 * locals.var_sqrt_pbsum_dn6)), (((((((assign23250_e17862 * locals.var_t1_dn7) * locals.var_t2) + (assign23250_e17864 * locals.var_t2_dn7)) * locals.var_t4) + (assign23250_e17866 * locals.var_t4_dn7)) * locals.var_sqrt_pbsum) + (assign23250_e17868 * locals.var_sqrt_pbsum_dn7)), (((((((assign23250_e17862 * locals.var_t1_dn8) * locals.var_t2) + (assign23250_e17864 * locals.var_t2_dn8)) * locals.var_t4) + (assign23250_e17866 * locals.var_t4_dn8)) * locals.var_sqrt_pbsum) + (assign23250_e17868 * locals.var_sqrt_pbsum_dn8)), (((((((assign23250_e17862 * locals.var_t1_dn9) * locals.var_t2) + (assign23250_e17864 * locals.var_t2_dn9)) * locals.var_t4) + (assign23250_e17866 * locals.var_t4_dn9)) * locals.var_sqrt_pbsum) + (assign23250_e17868 * locals.var_sqrt_pbsum_dn9)), (((((((assign23250_e17862 * locals.var_t1_dn10) * locals.var_t2) + (assign23250_e17864 * locals.var_t2_dn10)) * locals.var_t4) + (assign23250_e17866 * locals.var_t4_dn10)) * locals.var_sqrt_pbsum) + (assign23250_e17868 * locals.var_sqrt_pbsum_dn10)), (((((((assign23250_e17862 * locals.var_t1_dn11) * locals.var_t2) + (assign23250_e17864 * locals.var_t2_dn11)) * locals.var_t4) + (assign23250_e17866 * locals.var_t4_dn11)) * locals.var_sqrt_pbsum) + (assign23250_e17868 * locals.var_sqrt_pbsum_dn11)), (((((((assign23250_e17862 * locals.var_t1_dn14) * locals.var_t2) + (assign23250_e17864 * locals.var_t2_dn14)) * locals.var_t4) + (assign23250_e17866 * locals.var_t4_dn14)) * locals.var_sqrt_pbsum) + (assign23250_e17868 * locals.var_sqrt_pbsum_dn14)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign23250_e17872;
        locals.var_t8_dn0 = assign23250_e17872_d_n0;
        locals.var_t8_dn2 = assign23250_e17872_d_n2;
        locals.var_t8_dn4 = assign23250_e17872_d_n4;
        locals.var_t8_dn5 = assign23250_e17872_d_n5;
        locals.var_t8_dn6 = assign23250_e17872_d_n6;
        locals.var_t8_dn7 = assign23250_e17872_d_n7;
        locals.var_t8_dn8 = assign23250_e17872_d_n8;
        locals.var_t8_dn9 = assign23250_e17872_d_n9;
        locals.var_t8_dn10 = assign23250_e17872_d_n10;
        locals.var_t8_dn11 = assign23250_e17872_d_n11;
        locals.var_t8_dn14 = assign23250_e17872_d_n14;
        locals.var_t8_rv = 0.0;

        let (assign23260_e17878, assign23260_e17878_d_n0, assign23260_e17878_d_n2, assign23260_e17878_d_n4, assign23260_e17878_d_n5, assign23260_e17878_d_n6, assign23260_e17878_d_n7, assign23260_e17878_d_n8, assign23260_e17878_d_n9, assign23260_e17878_d_n10, assign23260_e17878_d_n11, assign23260_e17878_d_n14,) = {
    if (locals.var_guard434 != 0.0) {
        let assign23260_e17876: f64 = (locals.var_vthp - locals.var_vth0);
        (assign23260_e17876, (locals.var_vthp_dn0 - locals.var_vth0_dn0), (locals.var_vthp_dn2 - locals.var_vth0_dn2), (locals.var_vthp_dn4 - locals.var_vth0_dn4), (locals.var_vthp_dn5 - locals.var_vth0_dn5), (locals.var_vthp_dn6 - locals.var_vth0_dn6), (locals.var_vthp_dn7 - locals.var_vth0_dn7), (locals.var_vthp_dn8 - locals.var_vth0_dn8), (locals.var_vthp_dn9 - locals.var_vth0_dn9), (locals.var_vthp_dn10 - locals.var_vth0_dn10), (locals.var_vthp_dn11 - locals.var_vth0_dn11), (locals.var_vthp_dn14 - locals.var_vth0_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign23260_e17878;
        locals.var_t1_dn0 = assign23260_e17878_d_n0;
        locals.var_t1_dn2 = assign23260_e17878_d_n2;
        locals.var_t1_dn4 = assign23260_e17878_d_n4;
        locals.var_t1_dn5 = assign23260_e17878_d_n5;
        locals.var_t1_dn6 = assign23260_e17878_d_n6;
        locals.var_t1_dn7 = assign23260_e17878_d_n7;
        locals.var_t1_dn8 = assign23260_e17878_d_n8;
        locals.var_t1_dn9 = assign23260_e17878_d_n9;
        locals.var_t1_dn10 = assign23260_e17878_d_n10;
        locals.var_t1_dn11 = assign23260_e17878_d_n11;
        locals.var_t1_dn14 = assign23260_e17878_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign23270_e17888, assign23270_e17888_d_n0, assign23270_e17888_d_n2, assign23270_e17888_d_n4, assign23270_e17888_d_n5, assign23270_e17888_d_n6, assign23270_e17888_d_n7, assign23270_e17888_d_n8, assign23270_e17888_d_n9, assign23270_e17888_d_n10, assign23270_e17888_d_n11, assign23270_e17888_d_n14,) = {
    if (locals.var_guard434 != 0.0) {
        let assign23270_e17883: f64 = (locals.var_uc_scp3 * locals.var_pbsum);
        let assign23270_e17885: f64 = (assign23270_e17883 / p.p140);
        let assign23270_e17886: f64 = (locals.var_uc_scp1 + assign23270_e17885);
        (assign23270_e17886, ((locals.var_uc_scp3 * locals.var_pbsum_dn0) / p.p140), ((locals.var_uc_scp3 * locals.var_pbsum_dn2) / p.p140), ((locals.var_uc_scp3 * locals.var_pbsum_dn4) / p.p140), ((locals.var_uc_scp3 * locals.var_pbsum_dn5) / p.p140), ((locals.var_uc_scp3 * locals.var_pbsum_dn6) / p.p140), ((locals.var_uc_scp3 * locals.var_pbsum_dn7) / p.p140), ((locals.var_uc_scp3 * locals.var_pbsum_dn8) / p.p140), ((locals.var_uc_scp3 * locals.var_pbsum_dn9) / p.p140), ((locals.var_uc_scp3 * locals.var_pbsum_dn10) / p.p140), ((locals.var_uc_scp3 * locals.var_pbsum_dn11) / p.p140), ((locals.var_uc_scp3 * locals.var_pbsum_dn14) / p.p140),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign23270_e17888;
        locals.var_t2_dn0 = assign23270_e17888_d_n0;
        locals.var_t2_dn2 = assign23270_e17888_d_n2;
        locals.var_t2_dn4 = assign23270_e17888_d_n4;
        locals.var_t2_dn5 = assign23270_e17888_d_n5;
        locals.var_t2_dn6 = assign23270_e17888_d_n6;
        locals.var_t2_dn7 = assign23270_e17888_d_n7;
        locals.var_t2_dn8 = assign23270_e17888_d_n8;
        locals.var_t2_dn9 = assign23270_e17888_d_n9;
        locals.var_t2_dn10 = assign23270_e17888_d_n10;
        locals.var_t2_dn11 = assign23270_e17888_d_n11;
        locals.var_t2_dn14 = assign23270_e17888_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign23280_e17896, assign23280_e17896_d_n0, assign23280_e17896_d_n2, assign23280_e17896_d_n4, assign23280_e17896_d_n5, assign23280_e17896_d_n6, assign23280_e17896_d_n7, assign23280_e17896_d_n8, assign23280_e17896_d_n9, assign23280_e17896_d_n10, assign23280_e17896_d_n11, assign23280_e17896_d_n14,) = {
    if (locals.var_guard434 != 0.0) {
        let assign23280_e17893: f64 = (locals.var_uc_scp2 * locals.var_vdsz);
        let assign23280_e17894: f64 = (locals.var_t2 + assign23280_e17893);
        (assign23280_e17894, (locals.var_t2_dn0 + (locals.var_uc_scp2 * locals.var_vdsz_dn0)), (locals.var_t2_dn2 + (locals.var_uc_scp2 * locals.var_vdsz_dn2)), (locals.var_t2_dn4 + (locals.var_uc_scp2 * locals.var_vdsz_dn4)), (locals.var_t2_dn5 + (locals.var_uc_scp2 * locals.var_vdsz_dn5)), (locals.var_t2_dn6 + (locals.var_uc_scp2 * locals.var_vdsz_dn6)), (locals.var_t2_dn7 + (locals.var_uc_scp2 * locals.var_vdsz_dn7)), (locals.var_t2_dn8 + (locals.var_uc_scp2 * locals.var_vdsz_dn8)), (locals.var_t2_dn9 + (locals.var_uc_scp2 * locals.var_vdsz_dn9)), (locals.var_t2_dn10 + (locals.var_uc_scp2 * locals.var_vdsz_dn10)), (locals.var_t2_dn11 + (locals.var_uc_scp2 * locals.var_vdsz_dn11)), (locals.var_t2_dn14 + (locals.var_uc_scp2 * locals.var_vdsz_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign23280_e17896;
        locals.var_t3_dn0 = assign23280_e17896_d_n0;
        locals.var_t3_dn2 = assign23280_e17896_d_n2;
        locals.var_t3_dn4 = assign23280_e17896_d_n4;
        locals.var_t3_dn5 = assign23280_e17896_d_n5;
        locals.var_t3_dn6 = assign23280_e17896_d_n6;
        locals.var_t3_dn7 = assign23280_e17896_d_n7;
        locals.var_t3_dn8 = assign23280_e17896_d_n8;
        locals.var_t3_dn9 = assign23280_e17896_d_n9;
        locals.var_t3_dn10 = assign23280_e17896_d_n10;
        locals.var_t3_dn11 = assign23280_e17896_d_n11;
        locals.var_t3_dn14 = assign23280_e17896_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign23290_e17902, assign23290_e17902_d_n0, assign23290_e17902_d_n2, assign23290_e17902_d_n4, assign23290_e17902_d_n5, assign23290_e17902_d_n6, assign23290_e17902_d_n7, assign23290_e17902_d_n8, assign23290_e17902_d_n9, assign23290_e17902_d_n10, assign23290_e17902_d_n11, assign23290_e17902_d_n14,) = {
    if (locals.var_guard434 != 0.0) {
        let assign23290_e17900: f64 = (p.p221 + locals.var_vdsz);
        (assign23290_e17900, locals.var_vdsz_dn0, locals.var_vdsz_dn2, locals.var_vdsz_dn4, locals.var_vdsz_dn5, locals.var_vdsz_dn6, locals.var_vdsz_dn7, locals.var_vdsz_dn8, locals.var_vdsz_dn9, locals.var_vdsz_dn10, locals.var_vdsz_dn11, locals.var_vdsz_dn14,)
    } else {
        (locals.var_vdx, locals.var_vdx_dn0, locals.var_vdx_dn2, locals.var_vdx_dn4, locals.var_vdx_dn5, locals.var_vdx_dn6, locals.var_vdx_dn7, locals.var_vdx_dn8, locals.var_vdx_dn9, locals.var_vdx_dn10, locals.var_vdx_dn11, locals.var_vdx_dn14,)
    }
};
        locals.var_vdx = assign23290_e17902;
        locals.var_vdx_dn0 = assign23290_e17902_d_n0;
        locals.var_vdx_dn2 = assign23290_e17902_d_n2;
        locals.var_vdx_dn4 = assign23290_e17902_d_n4;
        locals.var_vdx_dn5 = assign23290_e17902_d_n5;
        locals.var_vdx_dn6 = assign23290_e17902_d_n6;
        locals.var_vdx_dn7 = assign23290_e17902_d_n7;
        locals.var_vdx_dn8 = assign23290_e17902_d_n8;
        locals.var_vdx_dn9 = assign23290_e17902_d_n9;
        locals.var_vdx_dn10 = assign23290_e17902_d_n10;
        locals.var_vdx_dn11 = assign23290_e17902_d_n11;
        locals.var_vdx_dn14 = assign23290_e17902_d_n14;
        locals.var_vdx_rv = 0.0;

        let (assign23300_e17908, assign23300_e17908_d_n0, assign23300_e17908_d_n2, assign23300_e17908_d_n4, assign23300_e17908_d_n5, assign23300_e17908_d_n6, assign23300_e17908_d_n7, assign23300_e17908_d_n8, assign23300_e17908_d_n9, assign23300_e17908_d_n10, assign23300_e17908_d_n11, assign23300_e17908_d_n14,) = {
    if (locals.var_guard434 != 0.0) {
        let assign23300_e17906: f64 = (locals.var_vdx * locals.var_vdx);
        (assign23300_e17906, ((locals.var_vdx_dn0 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn0)), ((locals.var_vdx_dn2 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn2)), ((locals.var_vdx_dn4 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn4)), ((locals.var_vdx_dn5 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn5)), ((locals.var_vdx_dn6 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn6)), ((locals.var_vdx_dn7 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn7)), ((locals.var_vdx_dn8 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn8)), ((locals.var_vdx_dn9 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn9)), ((locals.var_vdx_dn10 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn10)), ((locals.var_vdx_dn11 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn11)), ((locals.var_vdx_dn14 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn14)),)
    } else {
        (locals.var_vdx2, locals.var_vdx2_dn0, locals.var_vdx2_dn2, locals.var_vdx2_dn4, locals.var_vdx2_dn5, locals.var_vdx2_dn6, locals.var_vdx2_dn7, locals.var_vdx2_dn8, locals.var_vdx2_dn9, locals.var_vdx2_dn10, locals.var_vdx2_dn11, locals.var_vdx2_dn14,)
    }
};
        locals.var_vdx2 = assign23300_e17908;
        locals.var_vdx2_dn0 = assign23300_e17908_d_n0;
        locals.var_vdx2_dn2 = assign23300_e17908_d_n2;
        locals.var_vdx2_dn4 = assign23300_e17908_d_n4;
        locals.var_vdx2_dn5 = assign23300_e17908_d_n5;
        locals.var_vdx2_dn6 = assign23300_e17908_d_n6;
        locals.var_vdx2_dn7 = assign23300_e17908_d_n7;
        locals.var_vdx2_dn8 = assign23300_e17908_d_n8;
        locals.var_vdx2_dn9 = assign23300_e17908_d_n9;
        locals.var_vdx2_dn10 = assign23300_e17908_d_n10;
        locals.var_vdx2_dn11 = assign23300_e17908_d_n11;
        locals.var_vdx2_dn14 = assign23300_e17908_d_n14;
        locals.var_vdx2_rv = 0.0;

        let (assign23310_e17922, assign23310_e17922_d_n0, assign23310_e17922_d_n2, assign23310_e17922_d_n4, assign23310_e17922_d_n5, assign23310_e17922_d_n6, assign23310_e17922_d_n7, assign23310_e17922_d_n8, assign23310_e17922_d_n9, assign23310_e17922_d_n10, assign23310_e17922_d_n11, assign23310_e17922_d_n14,) = {
    if (locals.var_guard434 != 0.0) {
        let assign23310_e17912: f64 = (locals.var_t1 * locals.var_dvth0);
        let assign23310_e17914: f64 = (assign23310_e17912 * locals.var_t3);
        let assign23310_e17916: f64 = (assign23310_e17914 + locals.var_dqb);
        let assign23310_e17919: f64 = (locals.var_msc / locals.var_vdx2);
        let assign23310_e17920: f64 = (assign23310_e17916 - assign23310_e17919);
        (assign23310_e17920, ((((((locals.var_t1_dn0 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn0)) * locals.var_t3) + (assign23310_e17912 * locals.var_t3_dn0)) + locals.var_dqb_dn0) - (-((locals.var_msc * locals.var_vdx2_dn0) / (locals.var_vdx2 * locals.var_vdx2)))), ((((((locals.var_t1_dn2 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn2)) * locals.var_t3) + (assign23310_e17912 * locals.var_t3_dn2)) + locals.var_dqb_dn2) - (-((locals.var_msc * locals.var_vdx2_dn2) / (locals.var_vdx2 * locals.var_vdx2)))), ((((((locals.var_t1_dn4 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn4)) * locals.var_t3) + (assign23310_e17912 * locals.var_t3_dn4)) + locals.var_dqb_dn4) - (-((locals.var_msc * locals.var_vdx2_dn4) / (locals.var_vdx2 * locals.var_vdx2)))), ((((((locals.var_t1_dn5 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn5)) * locals.var_t3) + (assign23310_e17912 * locals.var_t3_dn5)) + locals.var_dqb_dn5) - (-((locals.var_msc * locals.var_vdx2_dn5) / (locals.var_vdx2 * locals.var_vdx2)))), ((((((locals.var_t1_dn6 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn6)) * locals.var_t3) + (assign23310_e17912 * locals.var_t3_dn6)) + locals.var_dqb_dn6) - (-((locals.var_msc * locals.var_vdx2_dn6) / (locals.var_vdx2 * locals.var_vdx2)))), ((((((locals.var_t1_dn7 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn7)) * locals.var_t3) + (assign23310_e17912 * locals.var_t3_dn7)) + locals.var_dqb_dn7) - (-((locals.var_msc * locals.var_vdx2_dn7) / (locals.var_vdx2 * locals.var_vdx2)))), ((((((locals.var_t1_dn8 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn8)) * locals.var_t3) + (assign23310_e17912 * locals.var_t3_dn8)) + locals.var_dqb_dn8) - (-((locals.var_msc * locals.var_vdx2_dn8) / (locals.var_vdx2 * locals.var_vdx2)))), ((((((locals.var_t1_dn9 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn9)) * locals.var_t3) + (assign23310_e17912 * locals.var_t3_dn9)) + locals.var_dqb_dn9) - (-((locals.var_msc * locals.var_vdx2_dn9) / (locals.var_vdx2 * locals.var_vdx2)))), ((((((locals.var_t1_dn10 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn10)) * locals.var_t3) + (assign23310_e17912 * locals.var_t3_dn10)) + locals.var_dqb_dn10) - (-((locals.var_msc * locals.var_vdx2_dn10) / (locals.var_vdx2 * locals.var_vdx2)))), ((((((locals.var_t1_dn11 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn11)) * locals.var_t3) + (assign23310_e17912 * locals.var_t3_dn11)) + locals.var_dqb_dn11) - (-((locals.var_msc * locals.var_vdx2_dn11) / (locals.var_vdx2 * locals.var_vdx2)))), ((((((locals.var_t1_dn14 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn14)) * locals.var_t3) + (assign23310_e17912 * locals.var_t3_dn14)) + locals.var_dqb_dn14) - (-((locals.var_msc * locals.var_vdx2_dn14) / (locals.var_vdx2 * locals.var_vdx2)))),)
    } else {
        (locals.var_dvthlp, locals.var_dvthlp_dn0, locals.var_dvthlp_dn2, locals.var_dvthlp_dn4, locals.var_dvthlp_dn5, locals.var_dvthlp_dn6, locals.var_dvthlp_dn7, locals.var_dvthlp_dn8, locals.var_dvthlp_dn9, locals.var_dvthlp_dn10, locals.var_dvthlp_dn11, locals.var_dvthlp_dn14,)
    }
};
        locals.var_dvthlp = assign23310_e17922;
        locals.var_dvthlp_dn0 = assign23310_e17922_d_n0;
        locals.var_dvthlp_dn2 = assign23310_e17922_d_n2;
        locals.var_dvthlp_dn4 = assign23310_e17922_d_n4;
        locals.var_dvthlp_dn5 = assign23310_e17922_d_n5;
        locals.var_dvthlp_dn6 = assign23310_e17922_d_n6;
        locals.var_dvthlp_dn7 = assign23310_e17922_d_n7;
        locals.var_dvthlp_dn8 = assign23310_e17922_d_n8;
        locals.var_dvthlp_dn9 = assign23310_e17922_d_n9;
        locals.var_dvthlp_dn10 = assign23310_e17922_d_n10;
        locals.var_dvthlp_dn11 = assign23310_e17922_d_n11;
        locals.var_dvthlp_dn14 = assign23310_e17922_d_n14;
        locals.var_dvthlp_rv = 0.0;

        let (assign23320_e17927, assign23320_e17927_d_n0, assign23320_e17927_d_n2, assign23320_e17927_d_n4, assign23320_e17927_d_n5, assign23320_e17927_d_n6, assign23320_e17927_d_n7, assign23320_e17927_d_n8, assign23320_e17927_d_n9, assign23320_e17927_d_n10, assign23320_e17927_d_n11, assign23320_e17927_d_n14,) = {
    if (locals.var_guard434 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dvthlp, locals.var_dvthlp_dn0, locals.var_dvthlp_dn2, locals.var_dvthlp_dn4, locals.var_dvthlp_dn5, locals.var_dvthlp_dn6, locals.var_dvthlp_dn7, locals.var_dvthlp_dn8, locals.var_dvthlp_dn9, locals.var_dvthlp_dn10, locals.var_dvthlp_dn11, locals.var_dvthlp_dn14,)
    }
};
        locals.var_dvthlp = assign23320_e17927;
        locals.var_dvthlp_dn0 = assign23320_e17927_d_n0;
        locals.var_dvthlp_dn2 = assign23320_e17927_d_n2;
        locals.var_dvthlp_dn4 = assign23320_e17927_d_n4;
        locals.var_dvthlp_dn5 = assign23320_e17927_d_n5;
        locals.var_dvthlp_dn6 = assign23320_e17927_d_n6;
        locals.var_dvthlp_dn7 = assign23320_e17927_d_n7;
        locals.var_dvthlp_dn8 = assign23320_e17927_d_n8;
        locals.var_dvthlp_dn9 = assign23320_e17927_d_n9;
        locals.var_dvthlp_dn10 = assign23320_e17927_d_n10;
        locals.var_dvthlp_dn11 = assign23320_e17927_d_n11;
        locals.var_dvthlp_dn14 = assign23320_e17927_d_n14;
        locals.var_dvthlp_rv = 0.0;

        let assign23330_e17930: f64 = (1.034943e-10 * locals.var_cox_inv);
        locals.var_t1 = assign23330_e17930;
        locals.var_t1_dn0 = (1.034943e-10 * locals.var_cox_inv_dn0);
        locals.var_t1_dn2 = (1.034943e-10 * locals.var_cox_inv_dn2);
        locals.var_t1_dn4 = (1.034943e-10 * locals.var_cox_inv_dn4);
        locals.var_t1_dn5 = (1.034943e-10 * locals.var_cox_inv_dn5);
        locals.var_t1_dn6 = (1.034943e-10 * locals.var_cox_inv_dn6);
        locals.var_t1_dn7 = (1.034943e-10 * locals.var_cox_inv_dn7);
        locals.var_t1_dn8 = (1.034943e-10 * locals.var_cox_inv_dn8);
        locals.var_t1_dn9 = (1.034943e-10 * locals.var_cox_inv_dn9);
        locals.var_t1_dn10 = (1.034943e-10 * locals.var_cox_inv_dn10);
        locals.var_t1_dn11 = (1.034943e-10 * locals.var_cox_inv_dn11);
        locals.var_t1_dn14 = (1.034943e-10 * locals.var_cox_inv_dn14);
        locals.var_t1_rv = 0.0;

        locals.var_t2 = locals.var_wdpl;
        locals.var_t2_dn0 = locals.var_wdpl_dn0;
        locals.var_t2_dn2 = locals.var_wdpl_dn2;
        locals.var_t2_dn4 = locals.var_wdpl_dn4;
        locals.var_t2_dn5 = locals.var_wdpl_dn5;
        locals.var_t2_dn6 = locals.var_wdpl_dn6;
        locals.var_t2_dn7 = locals.var_wdpl_dn7;
        locals.var_t2_dn8 = locals.var_wdpl_dn8;
        locals.var_t2_dn9 = locals.var_wdpl_dn9;
        locals.var_t2_dn10 = locals.var_wdpl_dn10;
        locals.var_t2_dn11 = locals.var_wdpl_dn11;
        locals.var_t2_dn14 = locals.var_wdpl_dn14;
        locals.var_t2_rv = 0.0;

        let assign23350_e17934: f64 = (locals.var_lgate - p.p139);
        locals.var_t3 = assign23350_e17934;
        locals.var_t3_dn0 = 0.0;
        locals.var_t3_dn2 = 0.0;
        locals.var_t3_dn4 = 0.0;
        locals.var_t3_dn5 = 0.0;
        locals.var_t3_dn6 = 0.0;
        locals.var_t3_dn7 = 0.0;
        locals.var_t3_dn8 = 0.0;
        locals.var_t3_dn9 = 0.0;
        locals.var_t3_dn10 = 0.0;
        locals.var_t3_dn11 = 0.0;
        locals.var_t3_dn14 = 0.0;
        locals.var_t3_rv = 0.0;

        let assign23360_e17938: f64 = (locals.var_t3 * locals.var_t3);
        let assign23360_e17939: f64 = (1.0 / assign23360_e17938);
        locals.var_t4 = assign23360_e17939;
        locals.var_t4_dn0 = (-(((locals.var_t3_dn0 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn0)) / (assign23360_e17938 * assign23360_e17938)));
        locals.var_t4_dn2 = (-(((locals.var_t3_dn2 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn2)) / (assign23360_e17938 * assign23360_e17938)));
        locals.var_t4_dn4 = (-(((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4)) / (assign23360_e17938 * assign23360_e17938)));
        locals.var_t4_dn5 = (-(((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5)) / (assign23360_e17938 * assign23360_e17938)));
        locals.var_t4_dn6 = (-(((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6)) / (assign23360_e17938 * assign23360_e17938)));
        locals.var_t4_dn7 = (-(((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7)) / (assign23360_e17938 * assign23360_e17938)));
        locals.var_t4_dn8 = (-(((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8)) / (assign23360_e17938 * assign23360_e17938)));
        locals.var_t4_dn9 = (-(((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9)) / (assign23360_e17938 * assign23360_e17938)));
        locals.var_t4_dn10 = (-(((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10)) / (assign23360_e17938 * assign23360_e17938)));
        locals.var_t4_dn11 = (-(((locals.var_t3_dn11 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn11)) / (assign23360_e17938 * assign23360_e17938)));
        locals.var_t4_dn14 = (-(((locals.var_t3_dn14 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn14)) / (assign23360_e17938 * assign23360_e17938)));
        locals.var_t4_rv = 0.0;

        let assign23370_e17943: f64 = (p.p137 - locals.var_pb20b);
        let assign23370_e17944: f64 = (2.0 * assign23370_e17943);
        let assign23370_e17946: f64 = (assign23370_e17944 * locals.var_t1);
        let assign23370_e17948: f64 = (assign23370_e17946 * locals.var_t2);
        let assign23370_e17950: f64 = (assign23370_e17948 * locals.var_t4);
        locals.var_t5 = assign23370_e17950;
        locals.var_t5_dn0 = (((((((2.0 * (-locals.var_pb20b_dn0)) * locals.var_t1) + (assign23370_e17944 * locals.var_t1_dn0)) * locals.var_t2) + (assign23370_e17946 * locals.var_t2_dn0)) * locals.var_t4) + (assign23370_e17948 * locals.var_t4_dn0));
        locals.var_t5_dn2 = (((((((2.0 * (-locals.var_pb20b_dn2)) * locals.var_t1) + (assign23370_e17944 * locals.var_t1_dn2)) * locals.var_t2) + (assign23370_e17946 * locals.var_t2_dn2)) * locals.var_t4) + (assign23370_e17948 * locals.var_t4_dn2));
        locals.var_t5_dn4 = (((((((2.0 * (-locals.var_pb20b_dn4)) * locals.var_t1) + (assign23370_e17944 * locals.var_t1_dn4)) * locals.var_t2) + (assign23370_e17946 * locals.var_t2_dn4)) * locals.var_t4) + (assign23370_e17948 * locals.var_t4_dn4));
        locals.var_t5_dn5 = (((((((2.0 * (-locals.var_pb20b_dn5)) * locals.var_t1) + (assign23370_e17944 * locals.var_t1_dn5)) * locals.var_t2) + (assign23370_e17946 * locals.var_t2_dn5)) * locals.var_t4) + (assign23370_e17948 * locals.var_t4_dn5));
        locals.var_t5_dn6 = (((((((2.0 * (-locals.var_pb20b_dn6)) * locals.var_t1) + (assign23370_e17944 * locals.var_t1_dn6)) * locals.var_t2) + (assign23370_e17946 * locals.var_t2_dn6)) * locals.var_t4) + (assign23370_e17948 * locals.var_t4_dn6));
        locals.var_t5_dn7 = (((((((2.0 * (-locals.var_pb20b_dn7)) * locals.var_t1) + (assign23370_e17944 * locals.var_t1_dn7)) * locals.var_t2) + (assign23370_e17946 * locals.var_t2_dn7)) * locals.var_t4) + (assign23370_e17948 * locals.var_t4_dn7));
        locals.var_t5_dn8 = (((((((2.0 * (-locals.var_pb20b_dn8)) * locals.var_t1) + (assign23370_e17944 * locals.var_t1_dn8)) * locals.var_t2) + (assign23370_e17946 * locals.var_t2_dn8)) * locals.var_t4) + (assign23370_e17948 * locals.var_t4_dn8));
        locals.var_t5_dn9 = (((((((2.0 * (-locals.var_pb20b_dn9)) * locals.var_t1) + (assign23370_e17944 * locals.var_t1_dn9)) * locals.var_t2) + (assign23370_e17946 * locals.var_t2_dn9)) * locals.var_t4) + (assign23370_e17948 * locals.var_t4_dn9));
        locals.var_t5_dn10 = (((((((2.0 * (-locals.var_pb20b_dn10)) * locals.var_t1) + (assign23370_e17944 * locals.var_t1_dn10)) * locals.var_t2) + (assign23370_e17946 * locals.var_t2_dn10)) * locals.var_t4) + (assign23370_e17948 * locals.var_t4_dn10));
        locals.var_t5_dn11 = (((((((2.0 * (-locals.var_pb20b_dn11)) * locals.var_t1) + (assign23370_e17944 * locals.var_t1_dn11)) * locals.var_t2) + (assign23370_e17946 * locals.var_t2_dn11)) * locals.var_t4) + (assign23370_e17948 * locals.var_t4_dn11));
        locals.var_t5_dn14 = (((((((2.0 * (-locals.var_pb20b_dn14)) * locals.var_t1) + (assign23370_e17944 * locals.var_t1_dn14)) * locals.var_t2) + (assign23370_e17946 * locals.var_t2_dn14)) * locals.var_t4) + (assign23370_e17948 * locals.var_t4_dn14));
        locals.var_t5_rv = 0.0;

        let assign23380_e17953: f64 = (locals.var_t5 * locals.var_sqrt_pbsum);
        locals.var_dvth0 = assign23380_e17953;
        locals.var_dvth0_dn0 = ((locals.var_t5_dn0 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn0));
        locals.var_dvth0_dn2 = ((locals.var_t5_dn2 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn2));
        locals.var_dvth0_dn4 = ((locals.var_t5_dn4 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn4));
        locals.var_dvth0_dn5 = ((locals.var_t5_dn5 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn5));
        locals.var_dvth0_dn6 = ((locals.var_t5_dn6 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn6));
        locals.var_dvth0_dn7 = ((locals.var_t5_dn7 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn7));
        locals.var_dvth0_dn8 = ((locals.var_t5_dn8 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn8));
        locals.var_dvth0_dn9 = ((locals.var_t5_dn9 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn9));
        locals.var_dvth0_dn10 = ((locals.var_t5_dn10 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn10));
        locals.var_dvth0_dn11 = ((locals.var_t5_dn11 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn11));
        locals.var_dvth0_dn14 = ((locals.var_t5_dn14 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn14));
        locals.var_dvth0_rv = 0.0;

        let assign23390_e17956: f64 = (locals.var_t5 / 2.0);
        let assign23390_e17958: f64 = (assign23390_e17956 / locals.var_sqrt_pbsum);
        locals.var_t6 = assign23390_e17958;
        locals.var_t6_dn0 = ((((locals.var_t5_dn0 / 2.0) * locals.var_sqrt_pbsum) - (assign23390_e17956 * locals.var_sqrt_pbsum_dn0)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_dn2 = ((((locals.var_t5_dn2 / 2.0) * locals.var_sqrt_pbsum) - (assign23390_e17956 * locals.var_sqrt_pbsum_dn2)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_dn4 = ((((locals.var_t5_dn4 / 2.0) * locals.var_sqrt_pbsum) - (assign23390_e17956 * locals.var_sqrt_pbsum_dn4)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_dn5 = ((((locals.var_t5_dn5 / 2.0) * locals.var_sqrt_pbsum) - (assign23390_e17956 * locals.var_sqrt_pbsum_dn5)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_dn6 = ((((locals.var_t5_dn6 / 2.0) * locals.var_sqrt_pbsum) - (assign23390_e17956 * locals.var_sqrt_pbsum_dn6)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_dn7 = ((((locals.var_t5_dn7 / 2.0) * locals.var_sqrt_pbsum) - (assign23390_e17956 * locals.var_sqrt_pbsum_dn7)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_dn8 = ((((locals.var_t5_dn8 / 2.0) * locals.var_sqrt_pbsum) - (assign23390_e17956 * locals.var_sqrt_pbsum_dn8)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_dn9 = ((((locals.var_t5_dn9 / 2.0) * locals.var_sqrt_pbsum) - (assign23390_e17956 * locals.var_sqrt_pbsum_dn9)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_dn10 = ((((locals.var_t5_dn10 / 2.0) * locals.var_sqrt_pbsum) - (assign23390_e17956 * locals.var_sqrt_pbsum_dn10)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_dn11 = ((((locals.var_t5_dn11 / 2.0) * locals.var_sqrt_pbsum) - (assign23390_e17956 * locals.var_sqrt_pbsum_dn11)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_dn14 = ((((locals.var_t5_dn14 / 2.0) * locals.var_sqrt_pbsum) - (assign23390_e17956 * locals.var_sqrt_pbsum_dn14)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_rv = 0.0;

        let assign23400_e17962: f64 = (p.p137 - locals.var_pb20b);
        let assign23400_e17963: f64 = (2.0 * assign23400_e17962);
        let assign23400_e17965: f64 = (assign23400_e17963 * 1.034943e-10);
        let assign23400_e17967: f64 = (assign23400_e17965 * locals.var_t2);
        let assign23400_e17969: f64 = (assign23400_e17967 * locals.var_t4);
        let assign23400_e17971: f64 = (assign23400_e17969 * locals.var_sqrt_pbsum);
        locals.var_t7 = assign23400_e17971;
        locals.var_t7_dn0 = ((((((((2.0 * (-locals.var_pb20b_dn0)) * 1.034943e-10) * locals.var_t2) + (assign23400_e17965 * locals.var_t2_dn0)) * locals.var_t4) + (assign23400_e17967 * locals.var_t4_dn0)) * locals.var_sqrt_pbsum) + (assign23400_e17969 * locals.var_sqrt_pbsum_dn0));
        locals.var_t7_dn2 = ((((((((2.0 * (-locals.var_pb20b_dn2)) * 1.034943e-10) * locals.var_t2) + (assign23400_e17965 * locals.var_t2_dn2)) * locals.var_t4) + (assign23400_e17967 * locals.var_t4_dn2)) * locals.var_sqrt_pbsum) + (assign23400_e17969 * locals.var_sqrt_pbsum_dn2));
        locals.var_t7_dn4 = ((((((((2.0 * (-locals.var_pb20b_dn4)) * 1.034943e-10) * locals.var_t2) + (assign23400_e17965 * locals.var_t2_dn4)) * locals.var_t4) + (assign23400_e17967 * locals.var_t4_dn4)) * locals.var_sqrt_pbsum) + (assign23400_e17969 * locals.var_sqrt_pbsum_dn4));
        locals.var_t7_dn5 = ((((((((2.0 * (-locals.var_pb20b_dn5)) * 1.034943e-10) * locals.var_t2) + (assign23400_e17965 * locals.var_t2_dn5)) * locals.var_t4) + (assign23400_e17967 * locals.var_t4_dn5)) * locals.var_sqrt_pbsum) + (assign23400_e17969 * locals.var_sqrt_pbsum_dn5));
        locals.var_t7_dn6 = ((((((((2.0 * (-locals.var_pb20b_dn6)) * 1.034943e-10) * locals.var_t2) + (assign23400_e17965 * locals.var_t2_dn6)) * locals.var_t4) + (assign23400_e17967 * locals.var_t4_dn6)) * locals.var_sqrt_pbsum) + (assign23400_e17969 * locals.var_sqrt_pbsum_dn6));
        locals.var_t7_dn7 = ((((((((2.0 * (-locals.var_pb20b_dn7)) * 1.034943e-10) * locals.var_t2) + (assign23400_e17965 * locals.var_t2_dn7)) * locals.var_t4) + (assign23400_e17967 * locals.var_t4_dn7)) * locals.var_sqrt_pbsum) + (assign23400_e17969 * locals.var_sqrt_pbsum_dn7));
        locals.var_t7_dn8 = ((((((((2.0 * (-locals.var_pb20b_dn8)) * 1.034943e-10) * locals.var_t2) + (assign23400_e17965 * locals.var_t2_dn8)) * locals.var_t4) + (assign23400_e17967 * locals.var_t4_dn8)) * locals.var_sqrt_pbsum) + (assign23400_e17969 * locals.var_sqrt_pbsum_dn8));
        locals.var_t7_dn9 = ((((((((2.0 * (-locals.var_pb20b_dn9)) * 1.034943e-10) * locals.var_t2) + (assign23400_e17965 * locals.var_t2_dn9)) * locals.var_t4) + (assign23400_e17967 * locals.var_t4_dn9)) * locals.var_sqrt_pbsum) + (assign23400_e17969 * locals.var_sqrt_pbsum_dn9));
        locals.var_t7_dn10 = ((((((((2.0 * (-locals.var_pb20b_dn10)) * 1.034943e-10) * locals.var_t2) + (assign23400_e17965 * locals.var_t2_dn10)) * locals.var_t4) + (assign23400_e17967 * locals.var_t4_dn10)) * locals.var_sqrt_pbsum) + (assign23400_e17969 * locals.var_sqrt_pbsum_dn10));
        locals.var_t7_dn11 = ((((((((2.0 * (-locals.var_pb20b_dn11)) * 1.034943e-10) * locals.var_t2) + (assign23400_e17965 * locals.var_t2_dn11)) * locals.var_t4) + (assign23400_e17967 * locals.var_t4_dn11)) * locals.var_sqrt_pbsum) + (assign23400_e17969 * locals.var_sqrt_pbsum_dn11));
        locals.var_t7_dn14 = ((((((((2.0 * (-locals.var_pb20b_dn14)) * 1.034943e-10) * locals.var_t2) + (assign23400_e17965 * locals.var_t2_dn14)) * locals.var_t4) + (assign23400_e17967 * locals.var_t4_dn14)) * locals.var_sqrt_pbsum) + (assign23400_e17969 * locals.var_sqrt_pbsum_dn14));
        locals.var_t7_rv = 0.0;

        let assign23410_e17973: f64 = (-2.0);
        let assign23410_e17975: f64 = (assign23410_e17973 * locals.var_t1);
        let assign23410_e17977: f64 = (assign23410_e17975 * locals.var_t2);
        let assign23410_e17979: f64 = (assign23410_e17977 * locals.var_t4);
        let assign23410_e17981: f64 = (assign23410_e17979 * locals.var_sqrt_pbsum);
        locals.var_t8 = assign23410_e17981;
        locals.var_t8_dn0 = (((((((assign23410_e17973 * locals.var_t1_dn0) * locals.var_t2) + (assign23410_e17975 * locals.var_t2_dn0)) * locals.var_t4) + (assign23410_e17977 * locals.var_t4_dn0)) * locals.var_sqrt_pbsum) + (assign23410_e17979 * locals.var_sqrt_pbsum_dn0));
        locals.var_t8_dn2 = (((((((assign23410_e17973 * locals.var_t1_dn2) * locals.var_t2) + (assign23410_e17975 * locals.var_t2_dn2)) * locals.var_t4) + (assign23410_e17977 * locals.var_t4_dn2)) * locals.var_sqrt_pbsum) + (assign23410_e17979 * locals.var_sqrt_pbsum_dn2));
        locals.var_t8_dn4 = (((((((assign23410_e17973 * locals.var_t1_dn4) * locals.var_t2) + (assign23410_e17975 * locals.var_t2_dn4)) * locals.var_t4) + (assign23410_e17977 * locals.var_t4_dn4)) * locals.var_sqrt_pbsum) + (assign23410_e17979 * locals.var_sqrt_pbsum_dn4));
        locals.var_t8_dn5 = (((((((assign23410_e17973 * locals.var_t1_dn5) * locals.var_t2) + (assign23410_e17975 * locals.var_t2_dn5)) * locals.var_t4) + (assign23410_e17977 * locals.var_t4_dn5)) * locals.var_sqrt_pbsum) + (assign23410_e17979 * locals.var_sqrt_pbsum_dn5));
        locals.var_t8_dn6 = (((((((assign23410_e17973 * locals.var_t1_dn6) * locals.var_t2) + (assign23410_e17975 * locals.var_t2_dn6)) * locals.var_t4) + (assign23410_e17977 * locals.var_t4_dn6)) * locals.var_sqrt_pbsum) + (assign23410_e17979 * locals.var_sqrt_pbsum_dn6));
        locals.var_t8_dn7 = (((((((assign23410_e17973 * locals.var_t1_dn7) * locals.var_t2) + (assign23410_e17975 * locals.var_t2_dn7)) * locals.var_t4) + (assign23410_e17977 * locals.var_t4_dn7)) * locals.var_sqrt_pbsum) + (assign23410_e17979 * locals.var_sqrt_pbsum_dn7));
        locals.var_t8_dn8 = (((((((assign23410_e17973 * locals.var_t1_dn8) * locals.var_t2) + (assign23410_e17975 * locals.var_t2_dn8)) * locals.var_t4) + (assign23410_e17977 * locals.var_t4_dn8)) * locals.var_sqrt_pbsum) + (assign23410_e17979 * locals.var_sqrt_pbsum_dn8));
        locals.var_t8_dn9 = (((((((assign23410_e17973 * locals.var_t1_dn9) * locals.var_t2) + (assign23410_e17975 * locals.var_t2_dn9)) * locals.var_t4) + (assign23410_e17977 * locals.var_t4_dn9)) * locals.var_sqrt_pbsum) + (assign23410_e17979 * locals.var_sqrt_pbsum_dn9));
        locals.var_t8_dn10 = (((((((assign23410_e17973 * locals.var_t1_dn10) * locals.var_t2) + (assign23410_e17975 * locals.var_t2_dn10)) * locals.var_t4) + (assign23410_e17977 * locals.var_t4_dn10)) * locals.var_sqrt_pbsum) + (assign23410_e17979 * locals.var_sqrt_pbsum_dn10));
        locals.var_t8_dn11 = (((((((assign23410_e17973 * locals.var_t1_dn11) * locals.var_t2) + (assign23410_e17975 * locals.var_t2_dn11)) * locals.var_t4) + (assign23410_e17977 * locals.var_t4_dn11)) * locals.var_sqrt_pbsum) + (assign23410_e17979 * locals.var_sqrt_pbsum_dn11));
        locals.var_t8_dn14 = (((((((assign23410_e17973 * locals.var_t1_dn14) * locals.var_t2) + (assign23410_e17975 * locals.var_t2_dn14)) * locals.var_t4) + (assign23410_e17977 * locals.var_t4_dn14)) * locals.var_sqrt_pbsum) + (assign23410_e17979 * locals.var_sqrt_pbsum_dn14));
        locals.var_t8_rv = 0.0;

        let assign23420_e17984: f64 = (locals.var_uc_sc3 / locals.var_lgate);
        locals.var_t1 = assign23420_e17984;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign23430_e17988: f64 = (locals.var_t1 * locals.var_pbsum);
        let assign23430_e17989: f64 = (locals.var_uc_sc1 + assign23430_e17988);
        locals.var_t4 = assign23430_e17989;
        locals.var_t4_dn0 = ((locals.var_t1_dn0 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn0));
        locals.var_t4_dn2 = ((locals.var_t1_dn2 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn2));
        locals.var_t4_dn4 = ((locals.var_t1_dn4 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn4));
        locals.var_t4_dn5 = ((locals.var_t1_dn5 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn5));
        locals.var_t4_dn6 = ((locals.var_t1_dn6 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn6));
        locals.var_t4_dn7 = ((locals.var_t1_dn7 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn7));
        locals.var_t4_dn8 = ((locals.var_t1_dn8 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn8));
        locals.var_t4_dn9 = ((locals.var_t1_dn9 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn9));
        locals.var_t4_dn10 = ((locals.var_t1_dn10 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn10));
        locals.var_t4_dn11 = ((locals.var_t1_dn11 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn11));
        locals.var_t4_dn14 = ((locals.var_t1_dn14 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn14));
        locals.var_t4_rv = 0.0;

        let assign23440_e17993: f64 = (locals.var_uc_sc2 * locals.var_vdsz);
        let assign23440_e17997: f64 = (p.p150 * locals.var_pbsum);
        let assign23440_e17998: f64 = (1.0 + assign23440_e17997);
        let assign23440_e17999: f64 = (assign23440_e17993 * assign23440_e17998);
        let assign23440_e18000: f64 = (locals.var_t4 + assign23440_e17999);
        locals.var_t5 = assign23440_e18000;
        locals.var_t5_dn0 = (locals.var_t4_dn0 + (((locals.var_uc_sc2 * locals.var_vdsz_dn0) * assign23440_e17998) + (assign23440_e17993 * (p.p150 * locals.var_pbsum_dn0))));
        locals.var_t5_dn2 = (locals.var_t4_dn2 + (((locals.var_uc_sc2 * locals.var_vdsz_dn2) * assign23440_e17998) + (assign23440_e17993 * (p.p150 * locals.var_pbsum_dn2))));
        locals.var_t5_dn4 = (locals.var_t4_dn4 + (((locals.var_uc_sc2 * locals.var_vdsz_dn4) * assign23440_e17998) + (assign23440_e17993 * (p.p150 * locals.var_pbsum_dn4))));
        locals.var_t5_dn5 = (locals.var_t4_dn5 + (((locals.var_uc_sc2 * locals.var_vdsz_dn5) * assign23440_e17998) + (assign23440_e17993 * (p.p150 * locals.var_pbsum_dn5))));
        locals.var_t5_dn6 = (locals.var_t4_dn6 + (((locals.var_uc_sc2 * locals.var_vdsz_dn6) * assign23440_e17998) + (assign23440_e17993 * (p.p150 * locals.var_pbsum_dn6))));
        locals.var_t5_dn7 = (locals.var_t4_dn7 + (((locals.var_uc_sc2 * locals.var_vdsz_dn7) * assign23440_e17998) + (assign23440_e17993 * (p.p150 * locals.var_pbsum_dn7))));
        locals.var_t5_dn8 = (locals.var_t4_dn8 + (((locals.var_uc_sc2 * locals.var_vdsz_dn8) * assign23440_e17998) + (assign23440_e17993 * (p.p150 * locals.var_pbsum_dn8))));
        locals.var_t5_dn9 = (locals.var_t4_dn9 + (((locals.var_uc_sc2 * locals.var_vdsz_dn9) * assign23440_e17998) + (assign23440_e17993 * (p.p150 * locals.var_pbsum_dn9))));
        locals.var_t5_dn10 = (locals.var_t4_dn10 + (((locals.var_uc_sc2 * locals.var_vdsz_dn10) * assign23440_e17998) + (assign23440_e17993 * (p.p150 * locals.var_pbsum_dn10))));
        locals.var_t5_dn11 = (locals.var_t4_dn11 + (((locals.var_uc_sc2 * locals.var_vdsz_dn11) * assign23440_e17998) + (assign23440_e17993 * (p.p150 * locals.var_pbsum_dn11))));
        locals.var_t5_dn14 = (locals.var_t4_dn14 + (((locals.var_uc_sc2 * locals.var_vdsz_dn14) * assign23440_e17998) + (assign23440_e17993 * (p.p150 * locals.var_pbsum_dn14))));
        locals.var_t5_rv = 0.0;

        let assign23450_e18003: f64 = (locals.var_dvth0 * locals.var_t5);
        locals.var_dvthsc = assign23450_e18003;
        locals.var_dvthsc_dn0 = ((locals.var_dvth0_dn0 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn0));
        locals.var_dvthsc_dn2 = ((locals.var_dvth0_dn2 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn2));
        locals.var_dvthsc_dn4 = ((locals.var_dvth0_dn4 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn4));
        locals.var_dvthsc_dn5 = ((locals.var_dvth0_dn5 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn5));
        locals.var_dvthsc_dn6 = ((locals.var_dvth0_dn6 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn6));
        locals.var_dvthsc_dn7 = ((locals.var_dvth0_dn7 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn7));
        locals.var_dvthsc_dn8 = ((locals.var_dvth0_dn8 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn8));
        locals.var_dvthsc_dn9 = ((locals.var_dvth0_dn9 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn9));
        locals.var_dvthsc_dn10 = ((locals.var_dvth0_dn10 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn10));
        locals.var_dvthsc_dn11 = ((locals.var_dvth0_dn11 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn11));
        locals.var_dvthsc_dn14 = ((locals.var_dvth0_dn14 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn14));
        locals.var_dvthsc_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_64(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign23460_e18006: f64 = (1.0 / locals.var_cox);
        locals.var_t1 = assign23460_e18006;
        locals.var_t1_dn0 = (-(locals.var_cox_dn0 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_dn2 = (-(locals.var_cox_dn2 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_dn4 = (-(locals.var_cox_dn4 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_dn5 = (-(locals.var_cox_dn5 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_dn6 = (-(locals.var_cox_dn6 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_dn7 = (-(locals.var_cox_dn7 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_dn8 = (-(locals.var_cox_dn8 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_dn9 = (-(locals.var_cox_dn9 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_dn10 = (-(locals.var_cox_dn10 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_dn11 = (-(locals.var_cox_dn11 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_dn14 = (-(locals.var_cox_dn14 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_rv = 0.0;

        let assign23470_e18009: f64 = (locals.var_t1 * locals.var_t1);
        locals.var_t2 = assign23470_e18009;
        locals.var_t2_dn0 = ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0));
        locals.var_t2_dn2 = ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2));
        locals.var_t2_dn4 = ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4));
        locals.var_t2_dn5 = ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5));
        locals.var_t2_dn6 = ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6));
        locals.var_t2_dn7 = ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7));
        locals.var_t2_dn8 = ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8));
        locals.var_t2_dn9 = ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9));
        locals.var_t2_dn10 = ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10));
        locals.var_t2_dn11 = ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11));
        locals.var_t2_dn14 = ((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14));
        locals.var_t2_rv = 0.0;

        let assign23480_e18014: f64 = (locals.var_uc_wfc / locals.var_weff);
        let assign23480_e18015: f64 = (locals.var_cox + assign23480_e18014);
        let assign23480_e18016: f64 = (1.0 / assign23480_e18015);
        locals.var_t3 = assign23480_e18016;
        locals.var_t3_dn0 = (-(locals.var_cox_dn0 / (assign23480_e18015 * assign23480_e18015)));
        locals.var_t3_dn2 = (-(locals.var_cox_dn2 / (assign23480_e18015 * assign23480_e18015)));
        locals.var_t3_dn4 = (-(locals.var_cox_dn4 / (assign23480_e18015 * assign23480_e18015)));
        locals.var_t3_dn5 = (-(locals.var_cox_dn5 / (assign23480_e18015 * assign23480_e18015)));
        locals.var_t3_dn6 = (-(locals.var_cox_dn6 / (assign23480_e18015 * assign23480_e18015)));
        locals.var_t3_dn7 = (-(locals.var_cox_dn7 / (assign23480_e18015 * assign23480_e18015)));
        locals.var_t3_dn8 = (-(locals.var_cox_dn8 / (assign23480_e18015 * assign23480_e18015)));
        locals.var_t3_dn9 = (-(locals.var_cox_dn9 / (assign23480_e18015 * assign23480_e18015)));
        locals.var_t3_dn10 = (-(locals.var_cox_dn10 / (assign23480_e18015 * assign23480_e18015)));
        locals.var_t3_dn11 = (-(locals.var_cox_dn11 / (assign23480_e18015 * assign23480_e18015)));
        locals.var_t3_dn14 = (-(locals.var_cox_dn14 / (assign23480_e18015 * assign23480_e18015)));
        locals.var_t3_rv = 0.0;

        let assign23490_e18019: f64 = (locals.var_t3 * locals.var_t3);
        locals.var_t4 = assign23490_e18019;
        locals.var_t4_dn0 = ((locals.var_t3_dn0 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn0));
        locals.var_t4_dn2 = ((locals.var_t3_dn2 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn2));
        locals.var_t4_dn4 = ((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4));
        locals.var_t4_dn5 = ((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5));
        locals.var_t4_dn6 = ((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6));
        locals.var_t4_dn7 = ((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7));
        locals.var_t4_dn8 = ((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8));
        locals.var_t4_dn9 = ((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9));
        locals.var_t4_dn10 = ((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10));
        locals.var_t4_dn11 = ((locals.var_t3_dn11 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn11));
        locals.var_t4_dn14 = ((locals.var_t3_dn14 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn14));
        locals.var_t4_rv = 0.0;

        let assign23500_e18022: f64 = (locals.var_t1 - locals.var_t3);
        locals.var_t5 = assign23500_e18022;
        locals.var_t5_dn0 = (locals.var_t1_dn0 - locals.var_t3_dn0);
        locals.var_t5_dn2 = (locals.var_t1_dn2 - locals.var_t3_dn2);
        locals.var_t5_dn4 = (locals.var_t1_dn4 - locals.var_t3_dn4);
        locals.var_t5_dn5 = (locals.var_t1_dn5 - locals.var_t3_dn5);
        locals.var_t5_dn6 = (locals.var_t1_dn6 - locals.var_t3_dn6);
        locals.var_t5_dn7 = (locals.var_t1_dn7 - locals.var_t3_dn7);
        locals.var_t5_dn8 = (locals.var_t1_dn8 - locals.var_t3_dn8);
        locals.var_t5_dn9 = (locals.var_t1_dn9 - locals.var_t3_dn9);
        locals.var_t5_dn10 = (locals.var_t1_dn10 - locals.var_t3_dn10);
        locals.var_t5_dn11 = (locals.var_t1_dn11 - locals.var_t3_dn11);
        locals.var_t5_dn14 = (locals.var_t1_dn14 - locals.var_t3_dn14);
        locals.var_t5_rv = 0.0;

        let assign23510_e18026: f64 = (locals.var_t2 - locals.var_t4);
        let assign23510_e18027: f64 = (locals.var_qb0 * assign23510_e18026);
        locals.var_t6 = assign23510_e18027;
        locals.var_t6_dn0 = ((locals.var_qb0_dn0 * assign23510_e18026) + (locals.var_qb0 * (locals.var_t2_dn0 - locals.var_t4_dn0)));
        locals.var_t6_dn2 = ((locals.var_qb0_dn2 * assign23510_e18026) + (locals.var_qb0 * (locals.var_t2_dn2 - locals.var_t4_dn2)));
        locals.var_t6_dn4 = ((locals.var_qb0_dn4 * assign23510_e18026) + (locals.var_qb0 * (locals.var_t2_dn4 - locals.var_t4_dn4)));
        locals.var_t6_dn5 = ((locals.var_qb0_dn5 * assign23510_e18026) + (locals.var_qb0 * (locals.var_t2_dn5 - locals.var_t4_dn5)));
        locals.var_t6_dn6 = ((locals.var_qb0_dn6 * assign23510_e18026) + (locals.var_qb0 * (locals.var_t2_dn6 - locals.var_t4_dn6)));
        locals.var_t6_dn7 = ((locals.var_qb0_dn7 * assign23510_e18026) + (locals.var_qb0 * (locals.var_t2_dn7 - locals.var_t4_dn7)));
        locals.var_t6_dn8 = ((locals.var_qb0_dn8 * assign23510_e18026) + (locals.var_qb0 * (locals.var_t2_dn8 - locals.var_t4_dn8)));
        locals.var_t6_dn9 = ((locals.var_qb0_dn9 * assign23510_e18026) + (locals.var_qb0 * (locals.var_t2_dn9 - locals.var_t4_dn9)));
        locals.var_t6_dn10 = ((locals.var_qb0_dn10 * assign23510_e18026) + (locals.var_qb0 * (locals.var_t2_dn10 - locals.var_t4_dn10)));
        locals.var_t6_dn11 = ((locals.var_qb0_dn11 * assign23510_e18026) + (locals.var_qb0 * (locals.var_t2_dn11 - locals.var_t4_dn11)));
        locals.var_t6_dn14 = ((locals.var_qb0_dn14 * assign23510_e18026) + (locals.var_qb0 * (locals.var_t2_dn14 - locals.var_t4_dn14)));
        locals.var_t6_rv = 0.0;

        let assign23520_e18030: f64 = (locals.var_qb0 * locals.var_t5);
        let assign23520_e18033: f64 = (locals.var_uc_wvth0 / locals.var_wg);
        let assign23520_e18034: f64 = (assign23520_e18030 + assign23520_e18033);
        locals.var_dvthw = assign23520_e18034;
        locals.var_dvthw_dn0 = ((locals.var_qb0_dn0 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn0));
        locals.var_dvthw_dn2 = ((locals.var_qb0_dn2 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn2));
        locals.var_dvthw_dn4 = ((locals.var_qb0_dn4 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn4));
        locals.var_dvthw_dn5 = ((locals.var_qb0_dn5 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn5));
        locals.var_dvthw_dn6 = ((locals.var_qb0_dn6 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn6));
        locals.var_dvthw_dn7 = ((locals.var_qb0_dn7 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn7));
        locals.var_dvthw_dn8 = ((locals.var_qb0_dn8 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn8));
        locals.var_dvthw_dn9 = ((locals.var_qb0_dn9 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn9));
        locals.var_dvthw_dn10 = ((locals.var_qb0_dn10 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn10));
        locals.var_dvthw_dn11 = ((locals.var_qb0_dn11 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn11));
        locals.var_dvthw_dn14 = ((locals.var_qb0_dn14 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn14));
        locals.var_dvthw_rv = 0.0;

        let assign23530_e18037: f64 = (locals.var_dvthsc + locals.var_dvthlp);
        let assign23530_e18039: f64 = (assign23530_e18037 + locals.var_dvthw);
        let assign23530_e18041: f64 = (assign23530_e18039 + locals.var_dvthsm);
        locals.var_dvth = assign23530_e18041;
        locals.var_dvth_dn0 = ((locals.var_dvthsc_dn0 + locals.var_dvthlp_dn0) + locals.var_dvthw_dn0);
        locals.var_dvth_dn2 = ((locals.var_dvthsc_dn2 + locals.var_dvthlp_dn2) + locals.var_dvthw_dn2);
        locals.var_dvth_dn4 = ((locals.var_dvthsc_dn4 + locals.var_dvthlp_dn4) + locals.var_dvthw_dn4);
        locals.var_dvth_dn5 = ((locals.var_dvthsc_dn5 + locals.var_dvthlp_dn5) + locals.var_dvthw_dn5);
        locals.var_dvth_dn6 = ((locals.var_dvthsc_dn6 + locals.var_dvthlp_dn6) + locals.var_dvthw_dn6);
        locals.var_dvth_dn7 = ((locals.var_dvthsc_dn7 + locals.var_dvthlp_dn7) + locals.var_dvthw_dn7);
        locals.var_dvth_dn8 = ((locals.var_dvthsc_dn8 + locals.var_dvthlp_dn8) + locals.var_dvthw_dn8);
        locals.var_dvth_dn9 = ((locals.var_dvthsc_dn9 + locals.var_dvthlp_dn9) + locals.var_dvthw_dn9);
        locals.var_dvth_dn10 = ((locals.var_dvthsc_dn10 + locals.var_dvthlp_dn10) + locals.var_dvthw_dn10);
        locals.var_dvth_dn11 = ((locals.var_dvthsc_dn11 + locals.var_dvthlp_dn11) + locals.var_dvthw_dn11);
        locals.var_dvth_dn14 = ((locals.var_dvthsc_dn14 + locals.var_dvthlp_dn14) + locals.var_dvthw_dn14);
        locals.var_dvth_rv = 0.0;

        let assign23540_e18045: f64 = (locals.var_pb2 - locals.var_vbsz);
        let assign23540_e18046: f64 = (locals.var_qnsub_esi2 * assign23540_e18045);
        let assign23540_e18047: f64 = (assign23540_e18046).sqrt();
        locals.var_t2 = assign23540_e18047;
        locals.var_t2_dn0 = (((locals.var_qnsub_esi2_dn0 * assign23540_e18045) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn0 - locals.var_vbsz_dn0))) / (2.0 * assign23540_e18047));
        locals.var_t2_dn2 = (((locals.var_qnsub_esi2_dn2 * assign23540_e18045) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn2 - locals.var_vbsz_dn2))) / (2.0 * assign23540_e18047));
        locals.var_t2_dn4 = (((locals.var_qnsub_esi2_dn4 * assign23540_e18045) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn4 - locals.var_vbsz_dn4))) / (2.0 * assign23540_e18047));
        locals.var_t2_dn5 = (((locals.var_qnsub_esi2_dn5 * assign23540_e18045) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn5 - locals.var_vbsz_dn5))) / (2.0 * assign23540_e18047));
        locals.var_t2_dn6 = (((locals.var_qnsub_esi2_dn6 * assign23540_e18045) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn6 - locals.var_vbsz_dn6))) / (2.0 * assign23540_e18047));
        locals.var_t2_dn7 = (((locals.var_qnsub_esi2_dn7 * assign23540_e18045) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn7 - locals.var_vbsz_dn7))) / (2.0 * assign23540_e18047));
        locals.var_t2_dn8 = (((locals.var_qnsub_esi2_dn8 * assign23540_e18045) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn8 - locals.var_vbsz_dn8))) / (2.0 * assign23540_e18047));
        locals.var_t2_dn9 = (((locals.var_qnsub_esi2_dn9 * assign23540_e18045) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn9 - locals.var_vbsz_dn9))) / (2.0 * assign23540_e18047));
        locals.var_t2_dn10 = (((locals.var_qnsub_esi2_dn10 * assign23540_e18045) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn10 - locals.var_vbsz_dn10))) / (2.0 * assign23540_e18047));
        locals.var_t2_dn11 = (((locals.var_qnsub_esi2_dn11 * assign23540_e18045) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn11 - locals.var_vbsz_dn11))) / (2.0 * assign23540_e18047));
        locals.var_t2_dn14 = (((locals.var_qnsub_esi2_dn14 * assign23540_e18045) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn14 - locals.var_vbsz_dn14))) / (2.0 * assign23540_e18047));
        locals.var_t2_rv = 0.0;

        let assign23550_e18050: f64 = (locals.var_pb2 + locals.var_vfb);
        let assign23550_e18053: f64 = (locals.var_t2 * locals.var_cox0_inv);
        let assign23550_e18054: f64 = (assign23550_e18050 + assign23550_e18053);
        let assign23550_e18056: f64 = (assign23550_e18054 - locals.var_dvth);
        locals.var_vth = assign23550_e18056;
        locals.var_vth_dn0 = ((locals.var_pb2_dn0 + (locals.var_t2_dn0 * locals.var_cox0_inv)) - locals.var_dvth_dn0);
        locals.var_vth_dn2 = ((locals.var_pb2_dn2 + (locals.var_t2_dn2 * locals.var_cox0_inv)) - locals.var_dvth_dn2);
        locals.var_vth_dn4 = ((locals.var_pb2_dn4 + (locals.var_t2_dn4 * locals.var_cox0_inv)) - locals.var_dvth_dn4);
        locals.var_vth_dn5 = ((locals.var_pb2_dn5 + (locals.var_t2_dn5 * locals.var_cox0_inv)) - locals.var_dvth_dn5);
        locals.var_vth_dn6 = ((locals.var_pb2_dn6 + (locals.var_t2_dn6 * locals.var_cox0_inv)) - locals.var_dvth_dn6);
        locals.var_vth_dn7 = ((locals.var_pb2_dn7 + (locals.var_t2_dn7 * locals.var_cox0_inv)) - locals.var_dvth_dn7);
        locals.var_vth_dn8 = ((locals.var_pb2_dn8 + (locals.var_t2_dn8 * locals.var_cox0_inv)) - locals.var_dvth_dn8);
        locals.var_vth_dn9 = ((locals.var_pb2_dn9 + (locals.var_t2_dn9 * locals.var_cox0_inv)) - locals.var_dvth_dn9);
        locals.var_vth_dn10 = ((locals.var_pb2_dn10 + (locals.var_t2_dn10 * locals.var_cox0_inv)) - locals.var_dvth_dn10);
        locals.var_vth_dn11 = ((locals.var_pb2_dn11 + (locals.var_t2_dn11 * locals.var_cox0_inv)) - locals.var_dvth_dn11);
        locals.var_vth_dn14 = ((locals.var_pb2_dn14 + (locals.var_t2_dn14 * locals.var_cox0_inv)) - locals.var_dvth_dn14);
        locals.var_vth_rv = 0.0;

        let assign23560_e18059: f64 = (locals.var_cnst0 * locals.var_cox_inv);
        locals.var_fac1 = assign23560_e18059;
        locals.var_fac1_dn0 = ((locals.var_cnst0_dn0 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn0));
        locals.var_fac1_dn2 = ((locals.var_cnst0_dn2 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn2));
        locals.var_fac1_dn4 = ((locals.var_cnst0_dn4 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn4));
        locals.var_fac1_dn5 = ((locals.var_cnst0_dn5 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn5));
        locals.var_fac1_dn6 = ((locals.var_cnst0_dn6 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn6));
        locals.var_fac1_dn7 = ((locals.var_cnst0_dn7 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn7));
        locals.var_fac1_dn8 = ((locals.var_cnst0_dn8 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn8));
        locals.var_fac1_dn9 = ((locals.var_cnst0_dn9 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn9));
        locals.var_fac1_dn10 = ((locals.var_cnst0_dn10 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn10));
        locals.var_fac1_dn11 = ((locals.var_cnst0_dn11 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn11));
        locals.var_fac1_dn14 = ((locals.var_cnst0_dn14 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn14));
        locals.var_fac1_rv = 0.0;

        let assign23570_e18062: f64 = (locals.var_fac1 * locals.var_fac1);
        locals.var_fac1p2 = assign23570_e18062;
        locals.var_fac1p2_dn0 = ((locals.var_fac1_dn0 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn0));
        locals.var_fac1p2_dn2 = ((locals.var_fac1_dn2 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn2));
        locals.var_fac1p2_dn4 = ((locals.var_fac1_dn4 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn4));
        locals.var_fac1p2_dn5 = ((locals.var_fac1_dn5 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn5));
        locals.var_fac1p2_dn6 = ((locals.var_fac1_dn6 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn6));
        locals.var_fac1p2_dn7 = ((locals.var_fac1_dn7 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn7));
        locals.var_fac1p2_dn8 = ((locals.var_fac1_dn8 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn8));
        locals.var_fac1p2_dn9 = ((locals.var_fac1_dn9 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn9));
        locals.var_fac1p2_dn10 = ((locals.var_fac1_dn10 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn10));
        locals.var_fac1p2_dn11 = ((locals.var_fac1_dn11 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn11));
        locals.var_fac1p2_dn14 = ((locals.var_fac1_dn14 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn14));
        locals.var_fac1p2_rv = 0.0;

        locals.var_dppg = 0.0;
        locals.var_dppg_dn0 = 0.0;
        locals.var_dppg_dn2 = 0.0;
        locals.var_dppg_dn4 = 0.0;
        locals.var_dppg_dn5 = 0.0;
        locals.var_dppg_dn6 = 0.0;
        locals.var_dppg_dn7 = 0.0;
        locals.var_dppg_dn8 = 0.0;
        locals.var_dppg_dn9 = 0.0;
        locals.var_dppg_dn10 = 0.0;
        locals.var_dppg_dn11 = 0.0;
        locals.var_dppg_dn14 = 0.0;
        locals.var_dppg_rv = 0.0;

        let assign23590_e18066: f64 = if locals.var_flg_pgd == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard435 = assign23590_e18066;
        locals.var_guard435_rv = 0.0;

        let (assign23600_e18070, assign23600_e18070_d_n0, assign23600_e18070_d_n2, assign23600_e18070_d_n4, assign23600_e18070_d_n5, assign23600_e18070_d_n6, assign23600_e18070_d_n7, assign23600_e18070_d_n8, assign23600_e18070_d_n9, assign23600_e18070_d_n10, assign23600_e18070_d_n11, assign23600_e18070_d_n14,) = {
    if (locals.var_guard435 != 0.0) {
        (locals.var_vgsz, locals.var_vgsz_dn0, locals.var_vgsz_dn2, locals.var_vgsz_dn4, locals.var_vgsz_dn5, locals.var_vgsz_dn6, locals.var_vgsz_dn7, locals.var_vgsz_dn8, locals.var_vgsz_dn9, locals.var_vgsz_dn10, locals.var_vgsz_dn11, locals.var_vgsz_dn14,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign23600_e18070;
        locals.var_t7_dn0 = assign23600_e18070_d_n0;
        locals.var_t7_dn2 = assign23600_e18070_d_n2;
        locals.var_t7_dn4 = assign23600_e18070_d_n4;
        locals.var_t7_dn5 = assign23600_e18070_d_n5;
        locals.var_t7_dn6 = assign23600_e18070_d_n6;
        locals.var_t7_dn7 = assign23600_e18070_d_n7;
        locals.var_t7_dn8 = assign23600_e18070_d_n8;
        locals.var_t7_dn9 = assign23600_e18070_d_n9;
        locals.var_t7_dn10 = assign23600_e18070_d_n10;
        locals.var_t7_dn11 = assign23600_e18070_d_n11;
        locals.var_t7_dn14 = assign23600_e18070_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign23610_e18074, assign23610_e18074_d_n0, assign23610_e18074_d_n2, assign23610_e18074_d_n4, assign23610_e18074_d_n5, assign23610_e18074_d_n6, assign23610_e18074_d_n7, assign23610_e18074_d_n8, assign23610_e18074_d_n9, assign23610_e18074_d_n10, assign23610_e18074_d_n11, assign23610_e18074_d_n14,) = {
    if (locals.var_guard435 != 0.0) {
        (locals.var_cnstpgd, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign23610_e18074;
        locals.var_t0_dn0 = assign23610_e18074_d_n0;
        locals.var_t0_dn2 = assign23610_e18074_d_n2;
        locals.var_t0_dn4 = assign23610_e18074_d_n4;
        locals.var_t0_dn5 = assign23610_e18074_d_n5;
        locals.var_t0_dn6 = assign23610_e18074_d_n6;
        locals.var_t0_dn7 = assign23610_e18074_d_n7;
        locals.var_t0_dn8 = assign23610_e18074_d_n8;
        locals.var_t0_dn9 = assign23610_e18074_d_n9;
        locals.var_t0_dn10 = assign23610_e18074_d_n10;
        locals.var_t0_dn11 = assign23610_e18074_d_n11;
        locals.var_t0_dn14 = assign23610_e18074_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign23620_e18080, assign23620_e18080_d_n0, assign23620_e18080_d_n2, assign23620_e18080_d_n4, assign23620_e18080_d_n5, assign23620_e18080_d_n6, assign23620_e18080_d_n7, assign23620_e18080_d_n8, assign23620_e18080_d_n9, assign23620_e18080_d_n10, assign23620_e18080_d_n11, assign23620_e18080_d_n14,) = {
    if (locals.var_guard435 != 0.0) {
        let assign23620_e18078: f64 = (locals.var_t7 - p.p152);
        (assign23620_e18078, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign23620_e18080;
        locals.var_t3_dn0 = assign23620_e18080_d_n0;
        locals.var_t3_dn2 = assign23620_e18080_d_n2;
        locals.var_t3_dn4 = assign23620_e18080_d_n4;
        locals.var_t3_dn5 = assign23620_e18080_d_n5;
        locals.var_t3_dn6 = assign23620_e18080_d_n6;
        locals.var_t3_dn7 = assign23620_e18080_d_n7;
        locals.var_t3_dn8 = assign23620_e18080_d_n8;
        locals.var_t3_dn9 = assign23620_e18080_d_n9;
        locals.var_t3_dn10 = assign23620_e18080_d_n10;
        locals.var_t3_dn11 = assign23620_e18080_d_n11;
        locals.var_t3_dn14 = assign23620_e18080_d_n14;
        locals.var_t3_rv = 0.0;

        let assign23630_e18083: f64 = (-3.0);
        let assign23630_e18084: f64 = if locals.var_t3 < assign23630_e18083 { 1.0 } else { 0.0 };
        locals.var_guard436 = assign23630_e18084;
        locals.var_guard436_rv = 0.0;

        let (assign23640_e18090, assign23640_e18090_d_n0, assign23640_e18090_d_n2, assign23640_e18090_d_n4, assign23640_e18090_d_n5, assign23640_e18090_d_n6, assign23640_e18090_d_n7, assign23640_e18090_d_n8, assign23640_e18090_d_n9, assign23640_e18090_d_n10, assign23640_e18090_d_n11, assign23640_e18090_d_n14,) = {
    if ((locals.var_guard435 != 0.0) && (locals.var_guard436 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign23640_e18090;
        locals.var_t6_dn0 = assign23640_e18090_d_n0;
        locals.var_t6_dn2 = assign23640_e18090_d_n2;
        locals.var_t6_dn4 = assign23640_e18090_d_n4;
        locals.var_t6_dn5 = assign23640_e18090_d_n5;
        locals.var_t6_dn6 = assign23640_e18090_d_n6;
        locals.var_t6_dn7 = assign23640_e18090_d_n7;
        locals.var_t6_dn8 = assign23640_e18090_d_n8;
        locals.var_t6_dn9 = assign23640_e18090_d_n9;
        locals.var_t6_dn10 = assign23640_e18090_d_n10;
        locals.var_t6_dn11 = assign23640_e18090_d_n11;
        locals.var_t6_dn14 = assign23640_e18090_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign23650_e18096, assign23650_e18096_d_n0, assign23650_e18096_d_n2, assign23650_e18096_d_n4, assign23650_e18096_d_n5, assign23650_e18096_d_n6, assign23650_e18096_d_n7, assign23650_e18096_d_n8, assign23650_e18096_d_n9, assign23650_e18096_d_n10, assign23650_e18096_d_n11, assign23650_e18096_d_n14,) = {
    if ((locals.var_guard435 != 0.0) && (locals.var_guard436 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn4, locals.var_dppg_dn5, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn8, locals.var_dppg_dn9, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn14,)
    }
};
        locals.var_dppg = assign23650_e18096;
        locals.var_dppg_dn0 = assign23650_e18096_d_n0;
        locals.var_dppg_dn2 = assign23650_e18096_d_n2;
        locals.var_dppg_dn4 = assign23650_e18096_d_n4;
        locals.var_dppg_dn5 = assign23650_e18096_d_n5;
        locals.var_dppg_dn6 = assign23650_e18096_d_n6;
        locals.var_dppg_dn7 = assign23650_e18096_d_n7;
        locals.var_dppg_dn8 = assign23650_e18096_d_n8;
        locals.var_dppg_dn9 = assign23650_e18096_d_n9;
        locals.var_dppg_dn10 = assign23650_e18096_d_n10;
        locals.var_dppg_dn11 = assign23650_e18096_d_n11;
        locals.var_dppg_dn14 = assign23650_e18096_d_n14;
        locals.var_dppg_rv = 0.0;

        let assign23660_e18099: f64 = if locals.var_t3 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard437 = assign23660_e18099;
        locals.var_guard437_rv = 0.0;

        let (assign23670_e18124, assign23670_e18124_d_n0, assign23670_e18124_d_n2, assign23670_e18124_d_n4, assign23670_e18124_d_n5, assign23670_e18124_d_n6, assign23670_e18124_d_n7, assign23670_e18124_d_n8, assign23670_e18124_d_n9, assign23670_e18124_d_n10, assign23670_e18124_d_n11, assign23670_e18124_d_n14,) = {
    if (((locals.var_guard435 != 0.0) && (locals.var_guard436 == 0.0)) && (locals.var_guard437 != 0.0)) {
        let assign23670_e18111: f64 = (1.0 / 3.0);
        let assign23670_e18112: f64 = (2.0 * assign23670_e18111);
        let assign23670_e18115: f64 = (locals.var_t3 * 3.0);
        let assign23670_e18118: f64 = (1.0 / 27.0);
        let assign23670_e18119: f64 = (assign23670_e18115 * assign23670_e18118);
        let assign23670_e18120: f64 = (assign23670_e18112 + assign23670_e18119);
        let assign23670_e18121: f64 = (locals.var_t3 * assign23670_e18120);
        let assign23670_e18122: f64 = (1.0 + assign23670_e18121);
        (assign23670_e18122, ((locals.var_t3_dn0 * assign23670_e18120) + (locals.var_t3 * ((locals.var_t3_dn0 * 3.0) * assign23670_e18118))), ((locals.var_t3_dn2 * assign23670_e18120) + (locals.var_t3 * ((locals.var_t3_dn2 * 3.0) * assign23670_e18118))), ((locals.var_t3_dn4 * assign23670_e18120) + (locals.var_t3 * ((locals.var_t3_dn4 * 3.0) * assign23670_e18118))), ((locals.var_t3_dn5 * assign23670_e18120) + (locals.var_t3 * ((locals.var_t3_dn5 * 3.0) * assign23670_e18118))), ((locals.var_t3_dn6 * assign23670_e18120) + (locals.var_t3 * ((locals.var_t3_dn6 * 3.0) * assign23670_e18118))), ((locals.var_t3_dn7 * assign23670_e18120) + (locals.var_t3 * ((locals.var_t3_dn7 * 3.0) * assign23670_e18118))), ((locals.var_t3_dn8 * assign23670_e18120) + (locals.var_t3 * ((locals.var_t3_dn8 * 3.0) * assign23670_e18118))), ((locals.var_t3_dn9 * assign23670_e18120) + (locals.var_t3 * ((locals.var_t3_dn9 * 3.0) * assign23670_e18118))), ((locals.var_t3_dn10 * assign23670_e18120) + (locals.var_t3 * ((locals.var_t3_dn10 * 3.0) * assign23670_e18118))), ((locals.var_t3_dn11 * assign23670_e18120) + (locals.var_t3 * ((locals.var_t3_dn11 * 3.0) * assign23670_e18118))), ((locals.var_t3_dn14 * assign23670_e18120) + (locals.var_t3 * ((locals.var_t3_dn14 * 3.0) * assign23670_e18118))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign23670_e18124;
        locals.var_t6_dn0 = assign23670_e18124_d_n0;
        locals.var_t6_dn2 = assign23670_e18124_d_n2;
        locals.var_t6_dn4 = assign23670_e18124_d_n4;
        locals.var_t6_dn5 = assign23670_e18124_d_n5;
        locals.var_t6_dn6 = assign23670_e18124_d_n6;
        locals.var_t6_dn7 = assign23670_e18124_d_n7;
        locals.var_t6_dn8 = assign23670_e18124_d_n8;
        locals.var_t6_dn9 = assign23670_e18124_d_n9;
        locals.var_t6_dn10 = assign23670_e18124_d_n10;
        locals.var_t6_dn11 = assign23670_e18124_d_n11;
        locals.var_t6_dn14 = assign23670_e18124_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign23680_e18149, assign23680_e18149_d_n0, assign23680_e18149_d_n2, assign23680_e18149_d_n4, assign23680_e18149_d_n5, assign23680_e18149_d_n6, assign23680_e18149_d_n7, assign23680_e18149_d_n8, assign23680_e18149_d_n9, assign23680_e18149_d_n10, assign23680_e18149_d_n11, assign23680_e18149_d_n14,) = {
    if (((locals.var_guard435 != 0.0) && (locals.var_guard436 == 0.0)) && (locals.var_guard437 != 0.0)) {
        let assign23680_e18137: f64 = (1.0 / 3.0);
        let assign23680_e18141: f64 = (1.0 / 27.0);
        let assign23680_e18142: f64 = (locals.var_t3 * assign23680_e18141);
        let assign23680_e18143: f64 = (assign23680_e18137 + assign23680_e18142);
        let assign23680_e18144: f64 = (locals.var_t3 * assign23680_e18143);
        let assign23680_e18145: f64 = (1.0 + assign23680_e18144);
        let assign23680_e18146: f64 = (locals.var_t3 * assign23680_e18145);
        let assign23680_e18147: f64 = (1.0 + assign23680_e18146);
        (assign23680_e18147, ((locals.var_t3_dn0 * assign23680_e18145) + (locals.var_t3 * ((locals.var_t3_dn0 * assign23680_e18143) + (locals.var_t3 * (locals.var_t3_dn0 * assign23680_e18141))))), ((locals.var_t3_dn2 * assign23680_e18145) + (locals.var_t3 * ((locals.var_t3_dn2 * assign23680_e18143) + (locals.var_t3 * (locals.var_t3_dn2 * assign23680_e18141))))), ((locals.var_t3_dn4 * assign23680_e18145) + (locals.var_t3 * ((locals.var_t3_dn4 * assign23680_e18143) + (locals.var_t3 * (locals.var_t3_dn4 * assign23680_e18141))))), ((locals.var_t3_dn5 * assign23680_e18145) + (locals.var_t3 * ((locals.var_t3_dn5 * assign23680_e18143) + (locals.var_t3 * (locals.var_t3_dn5 * assign23680_e18141))))), ((locals.var_t3_dn6 * assign23680_e18145) + (locals.var_t3 * ((locals.var_t3_dn6 * assign23680_e18143) + (locals.var_t3 * (locals.var_t3_dn6 * assign23680_e18141))))), ((locals.var_t3_dn7 * assign23680_e18145) + (locals.var_t3 * ((locals.var_t3_dn7 * assign23680_e18143) + (locals.var_t3 * (locals.var_t3_dn7 * assign23680_e18141))))), ((locals.var_t3_dn8 * assign23680_e18145) + (locals.var_t3 * ((locals.var_t3_dn8 * assign23680_e18143) + (locals.var_t3 * (locals.var_t3_dn8 * assign23680_e18141))))), ((locals.var_t3_dn9 * assign23680_e18145) + (locals.var_t3 * ((locals.var_t3_dn9 * assign23680_e18143) + (locals.var_t3 * (locals.var_t3_dn9 * assign23680_e18141))))), ((locals.var_t3_dn10 * assign23680_e18145) + (locals.var_t3 * ((locals.var_t3_dn10 * assign23680_e18143) + (locals.var_t3 * (locals.var_t3_dn10 * assign23680_e18141))))), ((locals.var_t3_dn11 * assign23680_e18145) + (locals.var_t3 * ((locals.var_t3_dn11 * assign23680_e18143) + (locals.var_t3 * (locals.var_t3_dn11 * assign23680_e18141))))), ((locals.var_t3_dn14 * assign23680_e18145) + (locals.var_t3 * ((locals.var_t3_dn14 * assign23680_e18143) + (locals.var_t3 * (locals.var_t3_dn14 * assign23680_e18141))))),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn4, locals.var_dppg_dn5, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn8, locals.var_dppg_dn9, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn14,)
    }
};
        locals.var_dppg = assign23680_e18149;
        locals.var_dppg_dn0 = assign23680_e18149_d_n0;
        locals.var_dppg_dn2 = assign23680_e18149_d_n2;
        locals.var_dppg_dn4 = assign23680_e18149_d_n4;
        locals.var_dppg_dn5 = assign23680_e18149_d_n5;
        locals.var_dppg_dn6 = assign23680_e18149_d_n6;
        locals.var_dppg_dn7 = assign23680_e18149_d_n7;
        locals.var_dppg_dn8 = assign23680_e18149_d_n8;
        locals.var_dppg_dn9 = assign23680_e18149_d_n9;
        locals.var_dppg_dn10 = assign23680_e18149_d_n10;
        locals.var_dppg_dn11 = assign23680_e18149_d_n11;
        locals.var_dppg_dn14 = assign23680_e18149_d_n14;
        locals.var_dppg_rv = 0.0;

        let (assign23690_e18179, assign23690_e18179_d_n0, assign23690_e18179_d_n2, assign23690_e18179_d_n4, assign23690_e18179_d_n5, assign23690_e18179_d_n6, assign23690_e18179_d_n7, assign23690_e18179_d_n8, assign23690_e18179_d_n9, assign23690_e18179_d_n10, assign23690_e18179_d_n11, assign23690_e18179_d_n14,) = {
    if (((locals.var_guard435 != 0.0) && (locals.var_guard436 == 0.0)) && (locals.var_guard437 == 0.0)) {
        let assign23690_e18162: f64 = (1.0 / 3.0);
        let assign23690_e18163: f64 = (2.0 * assign23690_e18162);
        let assign23690_e18167: f64 = (3.0 * 0.0402052934513951);
        let assign23690_e18170: f64 = (locals.var_t3 * 4.0);
        let assign23690_e18172: f64 = (assign23690_e18170 * 0.148148111111111);
        let assign23690_e18173: f64 = (assign23690_e18167 + assign23690_e18172);
        let assign23690_e18174: f64 = (locals.var_t3 * assign23690_e18173);
        let assign23690_e18175: f64 = (assign23690_e18163 + assign23690_e18174);
        let assign23690_e18176: f64 = (locals.var_t3 * assign23690_e18175);
        let assign23690_e18177: f64 = (1.0 + assign23690_e18176);
        (assign23690_e18177, ((locals.var_t3_dn0 * assign23690_e18175) + (locals.var_t3 * ((locals.var_t3_dn0 * assign23690_e18173) + (locals.var_t3 * ((locals.var_t3_dn0 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn2 * assign23690_e18175) + (locals.var_t3 * ((locals.var_t3_dn2 * assign23690_e18173) + (locals.var_t3 * ((locals.var_t3_dn2 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn4 * assign23690_e18175) + (locals.var_t3 * ((locals.var_t3_dn4 * assign23690_e18173) + (locals.var_t3 * ((locals.var_t3_dn4 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn5 * assign23690_e18175) + (locals.var_t3 * ((locals.var_t3_dn5 * assign23690_e18173) + (locals.var_t3 * ((locals.var_t3_dn5 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn6 * assign23690_e18175) + (locals.var_t3 * ((locals.var_t3_dn6 * assign23690_e18173) + (locals.var_t3 * ((locals.var_t3_dn6 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn7 * assign23690_e18175) + (locals.var_t3 * ((locals.var_t3_dn7 * assign23690_e18173) + (locals.var_t3 * ((locals.var_t3_dn7 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn8 * assign23690_e18175) + (locals.var_t3 * ((locals.var_t3_dn8 * assign23690_e18173) + (locals.var_t3 * ((locals.var_t3_dn8 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn9 * assign23690_e18175) + (locals.var_t3 * ((locals.var_t3_dn9 * assign23690_e18173) + (locals.var_t3 * ((locals.var_t3_dn9 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn10 * assign23690_e18175) + (locals.var_t3 * ((locals.var_t3_dn10 * assign23690_e18173) + (locals.var_t3 * ((locals.var_t3_dn10 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn11 * assign23690_e18175) + (locals.var_t3 * ((locals.var_t3_dn11 * assign23690_e18173) + (locals.var_t3 * ((locals.var_t3_dn11 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn14 * assign23690_e18175) + (locals.var_t3 * ((locals.var_t3_dn14 * assign23690_e18173) + (locals.var_t3 * ((locals.var_t3_dn14 * 4.0) * 0.148148111111111))))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign23690_e18179;
        locals.var_t6_dn0 = assign23690_e18179_d_n0;
        locals.var_t6_dn2 = assign23690_e18179_d_n2;
        locals.var_t6_dn4 = assign23690_e18179_d_n4;
        locals.var_t6_dn5 = assign23690_e18179_d_n5;
        locals.var_t6_dn6 = assign23690_e18179_d_n6;
        locals.var_t6_dn7 = assign23690_e18179_d_n7;
        locals.var_t6_dn8 = assign23690_e18179_d_n8;
        locals.var_t6_dn9 = assign23690_e18179_d_n9;
        locals.var_t6_dn10 = assign23690_e18179_d_n10;
        locals.var_t6_dn11 = assign23690_e18179_d_n11;
        locals.var_t6_dn14 = assign23690_e18179_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign23700_e18207, assign23700_e18207_d_n0, assign23700_e18207_d_n2, assign23700_e18207_d_n4, assign23700_e18207_d_n5, assign23700_e18207_d_n6, assign23700_e18207_d_n7, assign23700_e18207_d_n8, assign23700_e18207_d_n9, assign23700_e18207_d_n10, assign23700_e18207_d_n11, assign23700_e18207_d_n14,) = {
    if (((locals.var_guard435 != 0.0) && (locals.var_guard436 == 0.0)) && (locals.var_guard437 == 0.0)) {
        let assign23700_e18193: f64 = (1.0 / 3.0);
        let assign23700_e18198: f64 = (locals.var_t3 * 0.148148111111111);
        let assign23700_e18199: f64 = (0.0402052934513951 + assign23700_e18198);
        let assign23700_e18200: f64 = (locals.var_t3 * assign23700_e18199);
        let assign23700_e18201: f64 = (assign23700_e18193 + assign23700_e18200);
        let assign23700_e18202: f64 = (locals.var_t3 * assign23700_e18201);
        let assign23700_e18203: f64 = (1.0 + assign23700_e18202);
        let assign23700_e18204: f64 = (locals.var_t3 * assign23700_e18203);
        let assign23700_e18205: f64 = (1.0 + assign23700_e18204);
        (assign23700_e18205, ((locals.var_t3_dn0 * assign23700_e18203) + (locals.var_t3 * ((locals.var_t3_dn0 * assign23700_e18201) + (locals.var_t3 * ((locals.var_t3_dn0 * assign23700_e18199) + (locals.var_t3 * (locals.var_t3_dn0 * 0.148148111111111))))))), ((locals.var_t3_dn2 * assign23700_e18203) + (locals.var_t3 * ((locals.var_t3_dn2 * assign23700_e18201) + (locals.var_t3 * ((locals.var_t3_dn2 * assign23700_e18199) + (locals.var_t3 * (locals.var_t3_dn2 * 0.148148111111111))))))), ((locals.var_t3_dn4 * assign23700_e18203) + (locals.var_t3 * ((locals.var_t3_dn4 * assign23700_e18201) + (locals.var_t3 * ((locals.var_t3_dn4 * assign23700_e18199) + (locals.var_t3 * (locals.var_t3_dn4 * 0.148148111111111))))))), ((locals.var_t3_dn5 * assign23700_e18203) + (locals.var_t3 * ((locals.var_t3_dn5 * assign23700_e18201) + (locals.var_t3 * ((locals.var_t3_dn5 * assign23700_e18199) + (locals.var_t3 * (locals.var_t3_dn5 * 0.148148111111111))))))), ((locals.var_t3_dn6 * assign23700_e18203) + (locals.var_t3 * ((locals.var_t3_dn6 * assign23700_e18201) + (locals.var_t3 * ((locals.var_t3_dn6 * assign23700_e18199) + (locals.var_t3 * (locals.var_t3_dn6 * 0.148148111111111))))))), ((locals.var_t3_dn7 * assign23700_e18203) + (locals.var_t3 * ((locals.var_t3_dn7 * assign23700_e18201) + (locals.var_t3 * ((locals.var_t3_dn7 * assign23700_e18199) + (locals.var_t3 * (locals.var_t3_dn7 * 0.148148111111111))))))), ((locals.var_t3_dn8 * assign23700_e18203) + (locals.var_t3 * ((locals.var_t3_dn8 * assign23700_e18201) + (locals.var_t3 * ((locals.var_t3_dn8 * assign23700_e18199) + (locals.var_t3 * (locals.var_t3_dn8 * 0.148148111111111))))))), ((locals.var_t3_dn9 * assign23700_e18203) + (locals.var_t3 * ((locals.var_t3_dn9 * assign23700_e18201) + (locals.var_t3 * ((locals.var_t3_dn9 * assign23700_e18199) + (locals.var_t3 * (locals.var_t3_dn9 * 0.148148111111111))))))), ((locals.var_t3_dn10 * assign23700_e18203) + (locals.var_t3 * ((locals.var_t3_dn10 * assign23700_e18201) + (locals.var_t3 * ((locals.var_t3_dn10 * assign23700_e18199) + (locals.var_t3 * (locals.var_t3_dn10 * 0.148148111111111))))))), ((locals.var_t3_dn11 * assign23700_e18203) + (locals.var_t3 * ((locals.var_t3_dn11 * assign23700_e18201) + (locals.var_t3 * ((locals.var_t3_dn11 * assign23700_e18199) + (locals.var_t3 * (locals.var_t3_dn11 * 0.148148111111111))))))), ((locals.var_t3_dn14 * assign23700_e18203) + (locals.var_t3 * ((locals.var_t3_dn14 * assign23700_e18201) + (locals.var_t3 * ((locals.var_t3_dn14 * assign23700_e18199) + (locals.var_t3 * (locals.var_t3_dn14 * 0.148148111111111))))))),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn4, locals.var_dppg_dn5, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn8, locals.var_dppg_dn9, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn14,)
    }
};
        locals.var_dppg = assign23700_e18207;
        locals.var_dppg_dn0 = assign23700_e18207_d_n0;
        locals.var_dppg_dn2 = assign23700_e18207_d_n2;
        locals.var_dppg_dn4 = assign23700_e18207_d_n4;
        locals.var_dppg_dn5 = assign23700_e18207_d_n5;
        locals.var_dppg_dn6 = assign23700_e18207_d_n6;
        locals.var_dppg_dn7 = assign23700_e18207_d_n7;
        locals.var_dppg_dn8 = assign23700_e18207_d_n8;
        locals.var_dppg_dn9 = assign23700_e18207_d_n9;
        locals.var_dppg_dn10 = assign23700_e18207_d_n10;
        locals.var_dppg_dn11 = assign23700_e18207_d_n11;
        locals.var_dppg_dn14 = assign23700_e18207_d_n14;
        locals.var_dppg_rv = 0.0;

        let (assign23710_e18224, assign23710_e18224_d_n0, assign23710_e18224_d_n2, assign23710_e18224_d_n4, assign23710_e18224_d_n5, assign23710_e18224_d_n6, assign23710_e18224_d_n7, assign23710_e18224_d_n8, assign23710_e18224_d_n9, assign23710_e18224_d_n10, assign23710_e18224_d_n11, assign23710_e18224_d_n14,) = {
    if (locals.var_guard435 != 0.0) {
        let assign23710_e18211: f64 = (locals.var_dppg - 1.0);
        let assign23710_e18214: f64 = (locals.var_dppg - 1.0);
        let assign23710_e18215: f64 = (assign23710_e18211 * assign23710_e18214);
        let assign23710_e18218: f64 = (4.0 * 0.05);
        let assign23710_e18220: f64 = (assign23710_e18218 * 0.05);
        let assign23710_e18221: f64 = (assign23710_e18215 + assign23710_e18220);
        let assign23710_e18222: f64 = (assign23710_e18221).sqrt();
        (assign23710_e18222, (((locals.var_dppg_dn0 * assign23710_e18214) + (assign23710_e18211 * locals.var_dppg_dn0)) / (2.0 * assign23710_e18222)), (((locals.var_dppg_dn2 * assign23710_e18214) + (assign23710_e18211 * locals.var_dppg_dn2)) / (2.0 * assign23710_e18222)), (((locals.var_dppg_dn4 * assign23710_e18214) + (assign23710_e18211 * locals.var_dppg_dn4)) / (2.0 * assign23710_e18222)), (((locals.var_dppg_dn5 * assign23710_e18214) + (assign23710_e18211 * locals.var_dppg_dn5)) / (2.0 * assign23710_e18222)), (((locals.var_dppg_dn6 * assign23710_e18214) + (assign23710_e18211 * locals.var_dppg_dn6)) / (2.0 * assign23710_e18222)), (((locals.var_dppg_dn7 * assign23710_e18214) + (assign23710_e18211 * locals.var_dppg_dn7)) / (2.0 * assign23710_e18222)), (((locals.var_dppg_dn8 * assign23710_e18214) + (assign23710_e18211 * locals.var_dppg_dn8)) / (2.0 * assign23710_e18222)), (((locals.var_dppg_dn9 * assign23710_e18214) + (assign23710_e18211 * locals.var_dppg_dn9)) / (2.0 * assign23710_e18222)), (((locals.var_dppg_dn10 * assign23710_e18214) + (assign23710_e18211 * locals.var_dppg_dn10)) / (2.0 * assign23710_e18222)), (((locals.var_dppg_dn11 * assign23710_e18214) + (assign23710_e18211 * locals.var_dppg_dn11)) / (2.0 * assign23710_e18222)), (((locals.var_dppg_dn14 * assign23710_e18214) + (assign23710_e18211 * locals.var_dppg_dn14)) / (2.0 * assign23710_e18222)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign23710_e18224;
        locals.var_tmf2_dn0 = assign23710_e18224_d_n0;
        locals.var_tmf2_dn2 = assign23710_e18224_d_n2;
        locals.var_tmf2_dn4 = assign23710_e18224_d_n4;
        locals.var_tmf2_dn5 = assign23710_e18224_d_n5;
        locals.var_tmf2_dn6 = assign23710_e18224_d_n6;
        locals.var_tmf2_dn7 = assign23710_e18224_d_n7;
        locals.var_tmf2_dn8 = assign23710_e18224_d_n8;
        locals.var_tmf2_dn9 = assign23710_e18224_d_n9;
        locals.var_tmf2_dn10 = assign23710_e18224_d_n10;
        locals.var_tmf2_dn11 = assign23710_e18224_d_n11;
        locals.var_tmf2_dn14 = assign23710_e18224_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign23720_e18236, assign23720_e18236_d_n0, assign23720_e18236_d_n2, assign23720_e18236_d_n4, assign23720_e18236_d_n5, assign23720_e18236_d_n6, assign23720_e18236_d_n7, assign23720_e18236_d_n8, assign23720_e18236_d_n9, assign23720_e18236_d_n10, assign23720_e18236_d_n11, assign23720_e18236_d_n14,) = {
    if (locals.var_guard435 != 0.0) {
        let assign23720_e18230: f64 = (locals.var_dppg - 1.0);
        let assign23720_e18232: f64 = (assign23720_e18230 / locals.var_tmf2);
        let assign23720_e18233: f64 = (1.0 + assign23720_e18232);
        let assign23720_e18234: f64 = (0.5 * assign23720_e18233);
        (assign23720_e18234, (0.5 * (((locals.var_dppg_dn0 * locals.var_tmf2) - (assign23720_e18230 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn2 * locals.var_tmf2) - (assign23720_e18230 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn4 * locals.var_tmf2) - (assign23720_e18230 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn5 * locals.var_tmf2) - (assign23720_e18230 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn6 * locals.var_tmf2) - (assign23720_e18230 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn7 * locals.var_tmf2) - (assign23720_e18230 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn8 * locals.var_tmf2) - (assign23720_e18230 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn9 * locals.var_tmf2) - (assign23720_e18230 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn10 * locals.var_tmf2) - (assign23720_e18230 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn11 * locals.var_tmf2) - (assign23720_e18230 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn14 * locals.var_tmf2) - (assign23720_e18230 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign23720_e18236;
        locals.var_t6_dn0 = assign23720_e18236_d_n0;
        locals.var_t6_dn2 = assign23720_e18236_d_n2;
        locals.var_t6_dn4 = assign23720_e18236_d_n4;
        locals.var_t6_dn5 = assign23720_e18236_d_n5;
        locals.var_t6_dn6 = assign23720_e18236_d_n6;
        locals.var_t6_dn7 = assign23720_e18236_d_n7;
        locals.var_t6_dn8 = assign23720_e18236_d_n8;
        locals.var_t6_dn9 = assign23720_e18236_d_n9;
        locals.var_t6_dn10 = assign23720_e18236_d_n10;
        locals.var_t6_dn11 = assign23720_e18236_d_n11;
        locals.var_t6_dn14 = assign23720_e18236_d_n14;
        locals.var_t6_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_65(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23730_e18246, assign23730_e18246_d_n0, assign23730_e18246_d_n2, assign23730_e18246_d_n4, assign23730_e18246_d_n5, assign23730_e18246_d_n6, assign23730_e18246_d_n7, assign23730_e18246_d_n8, assign23730_e18246_d_n9, assign23730_e18246_d_n10, assign23730_e18246_d_n11, assign23730_e18246_d_n14,) = {
    if (locals.var_guard435 != 0.0) {
        let assign23730_e18241: f64 = (locals.var_dppg - 1.0);
        let assign23730_e18243: f64 = (assign23730_e18241 + locals.var_tmf2);
        let assign23730_e18244: f64 = (0.5 * assign23730_e18243);
        (assign23730_e18244, (0.5 * (locals.var_dppg_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_dppg_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_dppg_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_dppg_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_dppg_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_dppg_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_dppg_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_dppg_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_dppg_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_dppg_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_dppg_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn4, locals.var_dppg_dn5, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn8, locals.var_dppg_dn9, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn14,)
    }
};
        locals.var_dppg = assign23730_e18246;
        locals.var_dppg_dn0 = assign23730_e18246_d_n0;
        locals.var_dppg_dn2 = assign23730_e18246_d_n2;
        locals.var_dppg_dn4 = assign23730_e18246_d_n4;
        locals.var_dppg_dn5 = assign23730_e18246_d_n5;
        locals.var_dppg_dn6 = assign23730_e18246_d_n6;
        locals.var_dppg_dn7 = assign23730_e18246_d_n7;
        locals.var_dppg_dn8 = assign23730_e18246_d_n8;
        locals.var_dppg_dn9 = assign23730_e18246_d_n9;
        locals.var_dppg_dn10 = assign23730_e18246_d_n10;
        locals.var_dppg_dn11 = assign23730_e18246_d_n11;
        locals.var_dppg_dn14 = assign23730_e18246_d_n14;
        locals.var_dppg_rv = 0.0;

        let assign23740_e18249: f64 = if locals.var_dppg < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard438 = assign23740_e18249;
        locals.var_guard438_rv = 0.0;

        let (assign23750_e18255, assign23750_e18255_d_n0, assign23750_e18255_d_n2, assign23750_e18255_d_n4, assign23750_e18255_d_n5, assign23750_e18255_d_n6, assign23750_e18255_d_n7, assign23750_e18255_d_n8, assign23750_e18255_d_n9, assign23750_e18255_d_n10, assign23750_e18255_d_n11, assign23750_e18255_d_n14,) = {
    if ((locals.var_guard435 != 0.0) && (locals.var_guard438 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn4, locals.var_dppg_dn5, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn8, locals.var_dppg_dn9, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn14,)
    }
};
        locals.var_dppg = assign23750_e18255;
        locals.var_dppg_dn0 = assign23750_e18255_d_n0;
        locals.var_dppg_dn2 = assign23750_e18255_d_n2;
        locals.var_dppg_dn4 = assign23750_e18255_d_n4;
        locals.var_dppg_dn5 = assign23750_e18255_d_n5;
        locals.var_dppg_dn6 = assign23750_e18255_d_n6;
        locals.var_dppg_dn7 = assign23750_e18255_d_n7;
        locals.var_dppg_dn8 = assign23750_e18255_d_n8;
        locals.var_dppg_dn9 = assign23750_e18255_d_n9;
        locals.var_dppg_dn10 = assign23750_e18255_d_n10;
        locals.var_dppg_dn11 = assign23750_e18255_d_n11;
        locals.var_dppg_dn14 = assign23750_e18255_d_n14;
        locals.var_dppg_rv = 0.0;

        let (assign23760_e18261, assign23760_e18261_d_n0, assign23760_e18261_d_n2, assign23760_e18261_d_n4, assign23760_e18261_d_n5, assign23760_e18261_d_n6, assign23760_e18261_d_n7, assign23760_e18261_d_n8, assign23760_e18261_d_n9, assign23760_e18261_d_n10, assign23760_e18261_d_n11, assign23760_e18261_d_n14,) = {
    if ((locals.var_guard435 != 0.0) && (locals.var_guard438 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign23760_e18261;
        locals.var_t6_dn0 = assign23760_e18261_d_n0;
        locals.var_t6_dn2 = assign23760_e18261_d_n2;
        locals.var_t6_dn4 = assign23760_e18261_d_n4;
        locals.var_t6_dn5 = assign23760_e18261_d_n5;
        locals.var_t6_dn6 = assign23760_e18261_d_n6;
        locals.var_t6_dn7 = assign23760_e18261_d_n7;
        locals.var_t6_dn8 = assign23760_e18261_d_n8;
        locals.var_t6_dn9 = assign23760_e18261_d_n9;
        locals.var_t6_dn10 = assign23760_e18261_d_n10;
        locals.var_t6_dn11 = assign23760_e18261_d_n11;
        locals.var_t6_dn14 = assign23760_e18261_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign23770_e18267, assign23770_e18267_d_n0, assign23770_e18267_d_n2, assign23770_e18267_d_n4, assign23770_e18267_d_n5, assign23770_e18267_d_n6, assign23770_e18267_d_n7, assign23770_e18267_d_n8, assign23770_e18267_d_n9, assign23770_e18267_d_n10, assign23770_e18267_d_n11, assign23770_e18267_d_n14,) = {
    if (locals.var_guard435 != 0.0) {
        let assign23770_e18265: f64 = (locals.var_dppg * locals.var_t0);
        (assign23770_e18265, ((locals.var_dppg_dn0 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn0)), ((locals.var_dppg_dn2 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn2)), ((locals.var_dppg_dn4 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn4)), ((locals.var_dppg_dn5 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn5)), ((locals.var_dppg_dn6 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn6)), ((locals.var_dppg_dn7 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn7)), ((locals.var_dppg_dn8 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn8)), ((locals.var_dppg_dn9 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn9)), ((locals.var_dppg_dn10 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn10)), ((locals.var_dppg_dn11 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn11)), ((locals.var_dppg_dn14 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn14)),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn4, locals.var_dppg_dn5, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn8, locals.var_dppg_dn9, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn14,)
    }
};
        locals.var_dppg = assign23770_e18267;
        locals.var_dppg_dn0 = assign23770_e18267_d_n0;
        locals.var_dppg_dn2 = assign23770_e18267_d_n2;
        locals.var_dppg_dn4 = assign23770_e18267_d_n4;
        locals.var_dppg_dn5 = assign23770_e18267_d_n5;
        locals.var_dppg_dn6 = assign23770_e18267_d_n6;
        locals.var_dppg_dn7 = assign23770_e18267_d_n7;
        locals.var_dppg_dn8 = assign23770_e18267_d_n8;
        locals.var_dppg_dn9 = assign23770_e18267_d_n9;
        locals.var_dppg_dn10 = assign23770_e18267_d_n10;
        locals.var_dppg_dn11 = assign23770_e18267_d_n11;
        locals.var_dppg_dn14 = assign23770_e18267_d_n14;
        locals.var_dppg_rv = 0.0;

        let (assign23780_e18275, assign23780_e18275_d_n0, assign23780_e18275_d_n2, assign23780_e18275_d_n4, assign23780_e18275_d_n5, assign23780_e18275_d_n6, assign23780_e18275_d_n7, assign23780_e18275_d_n8, assign23780_e18275_d_n9, assign23780_e18275_d_n10, assign23780_e18275_d_n11, assign23780_e18275_d_n14,) = {
    if (locals.var_guard435 != 0.0) {
        let assign23780_e18271: f64 = (1.0 - locals.var_dppg);
        let assign23780_e18273: f64 = (assign23780_e18271 - 0.05);
        (assign23780_e18273, (-locals.var_dppg_dn0), (-locals.var_dppg_dn2), (-locals.var_dppg_dn4), (-locals.var_dppg_dn5), (-locals.var_dppg_dn6), (-locals.var_dppg_dn7), (-locals.var_dppg_dn8), (-locals.var_dppg_dn9), (-locals.var_dppg_dn10), (-locals.var_dppg_dn11), (-locals.var_dppg_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign23780_e18275;
        locals.var_tmf1_dn0 = assign23780_e18275_d_n0;
        locals.var_tmf1_dn2 = assign23780_e18275_d_n2;
        locals.var_tmf1_dn4 = assign23780_e18275_d_n4;
        locals.var_tmf1_dn5 = assign23780_e18275_d_n5;
        locals.var_tmf1_dn6 = assign23780_e18275_d_n6;
        locals.var_tmf1_dn7 = assign23780_e18275_d_n7;
        locals.var_tmf1_dn8 = assign23780_e18275_d_n8;
        locals.var_tmf1_dn9 = assign23780_e18275_d_n9;
        locals.var_tmf1_dn10 = assign23780_e18275_d_n10;
        locals.var_tmf1_dn11 = assign23780_e18275_d_n11;
        locals.var_tmf1_dn14 = assign23780_e18275_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign23790_e18283, assign23790_e18283_d_n0, assign23790_e18283_d_n2, assign23790_e18283_d_n4, assign23790_e18283_d_n5, assign23790_e18283_d_n6, assign23790_e18283_d_n7, assign23790_e18283_d_n8, assign23790_e18283_d_n9, assign23790_e18283_d_n10, assign23790_e18283_d_n11, assign23790_e18283_d_n14,) = {
    if (locals.var_guard435 != 0.0) {
        let assign23790_e18279: f64 = 4.0;
        let assign23790_e18281: f64 = (assign23790_e18279 * 0.05);
        (assign23790_e18281, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign23790_e18283;
        locals.var_tmf2_dn0 = assign23790_e18283_d_n0;
        locals.var_tmf2_dn2 = assign23790_e18283_d_n2;
        locals.var_tmf2_dn4 = assign23790_e18283_d_n4;
        locals.var_tmf2_dn5 = assign23790_e18283_d_n5;
        locals.var_tmf2_dn6 = assign23790_e18283_d_n6;
        locals.var_tmf2_dn7 = assign23790_e18283_d_n7;
        locals.var_tmf2_dn8 = assign23790_e18283_d_n8;
        locals.var_tmf2_dn9 = assign23790_e18283_d_n9;
        locals.var_tmf2_dn10 = assign23790_e18283_d_n10;
        locals.var_tmf2_dn11 = assign23790_e18283_d_n11;
        locals.var_tmf2_dn14 = assign23790_e18283_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign23800_e18293, assign23800_e18293_d_n0, assign23800_e18293_d_n2, assign23800_e18293_d_n4, assign23800_e18293_d_n5, assign23800_e18293_d_n6, assign23800_e18293_d_n7, assign23800_e18293_d_n8, assign23800_e18293_d_n9, assign23800_e18293_d_n10, assign23800_e18293_d_n11, assign23800_e18293_d_n14,) = {
    if (locals.var_guard435 != 0.0) {
        let (assign23800_e18291, assign23800_e18291_d_n0, assign23800_e18291_d_n2, assign23800_e18291_d_n4, assign23800_e18291_d_n5, assign23800_e18291_d_n6, assign23800_e18291_d_n7, assign23800_e18291_d_n8, assign23800_e18291_d_n9, assign23800_e18291_d_n10, assign23800_e18291_d_n11, assign23800_e18291_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign23800_e18290: f64 = (-locals.var_tmf2);
                (assign23800_e18290, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign23800_e18291, assign23800_e18291_d_n0, assign23800_e18291_d_n2, assign23800_e18291_d_n4, assign23800_e18291_d_n5, assign23800_e18291_d_n6, assign23800_e18291_d_n7, assign23800_e18291_d_n8, assign23800_e18291_d_n9, assign23800_e18291_d_n10, assign23800_e18291_d_n11, assign23800_e18291_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign23800_e18293;
        locals.var_tmf2_dn0 = assign23800_e18293_d_n0;
        locals.var_tmf2_dn2 = assign23800_e18293_d_n2;
        locals.var_tmf2_dn4 = assign23800_e18293_d_n4;
        locals.var_tmf2_dn5 = assign23800_e18293_d_n5;
        locals.var_tmf2_dn6 = assign23800_e18293_d_n6;
        locals.var_tmf2_dn7 = assign23800_e18293_d_n7;
        locals.var_tmf2_dn8 = assign23800_e18293_d_n8;
        locals.var_tmf2_dn9 = assign23800_e18293_d_n9;
        locals.var_tmf2_dn10 = assign23800_e18293_d_n10;
        locals.var_tmf2_dn11 = assign23800_e18293_d_n11;
        locals.var_tmf2_dn14 = assign23800_e18293_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign23810_e18302, assign23810_e18302_d_n0, assign23810_e18302_d_n2, assign23810_e18302_d_n4, assign23810_e18302_d_n5, assign23810_e18302_d_n6, assign23810_e18302_d_n7, assign23810_e18302_d_n8, assign23810_e18302_d_n9, assign23810_e18302_d_n10, assign23810_e18302_d_n11, assign23810_e18302_d_n14,) = {
    if (locals.var_guard435 != 0.0) {
        let assign23810_e18297: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign23810_e18299: f64 = (assign23810_e18297 + locals.var_tmf2);
        let assign23810_e18300: f64 = (assign23810_e18299).sqrt();
        (assign23810_e18300, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign23810_e18300)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign23810_e18300)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign23810_e18300)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign23810_e18300)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign23810_e18300)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign23810_e18300)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign23810_e18300)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign23810_e18300)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign23810_e18300)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign23810_e18300)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign23810_e18300)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign23810_e18302;
        locals.var_tmf2_dn0 = assign23810_e18302_d_n0;
        locals.var_tmf2_dn2 = assign23810_e18302_d_n2;
        locals.var_tmf2_dn4 = assign23810_e18302_d_n4;
        locals.var_tmf2_dn5 = assign23810_e18302_d_n5;
        locals.var_tmf2_dn6 = assign23810_e18302_d_n6;
        locals.var_tmf2_dn7 = assign23810_e18302_d_n7;
        locals.var_tmf2_dn8 = assign23810_e18302_d_n8;
        locals.var_tmf2_dn9 = assign23810_e18302_d_n9;
        locals.var_tmf2_dn10 = assign23810_e18302_d_n10;
        locals.var_tmf2_dn11 = assign23810_e18302_d_n11;
        locals.var_tmf2_dn14 = assign23810_e18302_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign23820_e18312, assign23820_e18312_d_n0, assign23820_e18312_d_n2, assign23820_e18312_d_n4, assign23820_e18312_d_n5, assign23820_e18312_d_n6, assign23820_e18312_d_n7, assign23820_e18312_d_n8, assign23820_e18312_d_n9, assign23820_e18312_d_n10, assign23820_e18312_d_n11, assign23820_e18312_d_n14,) = {
    if (locals.var_guard435 != 0.0) {
        let assign23820_e18308: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign23820_e18309: f64 = (1.0 + assign23820_e18308);
        let assign23820_e18310: f64 = (0.5 * assign23820_e18309);
        (assign23820_e18310, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign23820_e18312;
        locals.var_t9_dn0 = assign23820_e18312_d_n0;
        locals.var_t9_dn2 = assign23820_e18312_d_n2;
        locals.var_t9_dn4 = assign23820_e18312_d_n4;
        locals.var_t9_dn5 = assign23820_e18312_d_n5;
        locals.var_t9_dn6 = assign23820_e18312_d_n6;
        locals.var_t9_dn7 = assign23820_e18312_d_n7;
        locals.var_t9_dn8 = assign23820_e18312_d_n8;
        locals.var_t9_dn9 = assign23820_e18312_d_n9;
        locals.var_t9_dn10 = assign23820_e18312_d_n10;
        locals.var_t9_dn11 = assign23820_e18312_d_n11;
        locals.var_t9_dn14 = assign23820_e18312_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign23830_e18322, assign23830_e18322_d_n0, assign23830_e18322_d_n2, assign23830_e18322_d_n4, assign23830_e18322_d_n5, assign23830_e18322_d_n6, assign23830_e18322_d_n7, assign23830_e18322_d_n8, assign23830_e18322_d_n9, assign23830_e18322_d_n10, assign23830_e18322_d_n11, assign23830_e18322_d_n14,) = {
    if (locals.var_guard435 != 0.0) {
        let assign23830_e18318: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign23830_e18319: f64 = (0.5 * assign23830_e18318);
        let assign23830_e18320: f64 = (1.0 - assign23830_e18319);
        (assign23830_e18320, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn4, locals.var_dppg_dn5, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn8, locals.var_dppg_dn9, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn14,)
    }
};
        locals.var_dppg = assign23830_e18322;
        locals.var_dppg_dn0 = assign23830_e18322_d_n0;
        locals.var_dppg_dn2 = assign23830_e18322_d_n2;
        locals.var_dppg_dn4 = assign23830_e18322_d_n4;
        locals.var_dppg_dn5 = assign23830_e18322_d_n5;
        locals.var_dppg_dn6 = assign23830_e18322_d_n6;
        locals.var_dppg_dn7 = assign23830_e18322_d_n7;
        locals.var_dppg_dn8 = assign23830_e18322_d_n8;
        locals.var_dppg_dn9 = assign23830_e18322_d_n9;
        locals.var_dppg_dn10 = assign23830_e18322_d_n10;
        locals.var_dppg_dn11 = assign23830_e18322_d_n11;
        locals.var_dppg_dn14 = assign23830_e18322_d_n14;
        locals.var_dppg_rv = 0.0;

        let assign23840_e18325: f64 = if locals.var_vbs > locals.var_vbs_bnd_local { 1.0 } else { 0.0 };
        locals.var_guard445 = assign23840_e18325;
        locals.var_guard445_rv = 0.0;

        let (assign23850_e18333, assign23850_e18333_d_n0, assign23850_e18333_d_n2, assign23850_e18333_d_n4, assign23850_e18333_d_n5, assign23850_e18333_d_n6, assign23850_e18333_d_n7, assign23850_e18333_d_n8, assign23850_e18333_d_n9, assign23850_e18333_d_n10, assign23850_e18333_d_n11, assign23850_e18333_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard445 != 0.0)) {
        let assign23850_e18331: f64 = (locals.var_vbs - locals.var_vbs_bnd_local);
        (assign23850_e18331, (-locals.var_vbs_bnd_local_dn0), (-locals.var_vbs_bnd_local_dn2), (-locals.var_vbs_bnd_local_dn4), (-locals.var_vbs_bnd_local_dn5), (locals.var_vbs_dn6 - locals.var_vbs_bnd_local_dn6), (-locals.var_vbs_bnd_local_dn7), (locals.var_vbs_dn8 - locals.var_vbs_bnd_local_dn8), (locals.var_vbs_dn9 - locals.var_vbs_bnd_local_dn9), (-locals.var_vbs_bnd_local_dn10), (-locals.var_vbs_bnd_local_dn11), (-locals.var_vbs_bnd_local_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign23850_e18333;
        locals.var_t1_dn0 = assign23850_e18333_d_n0;
        locals.var_t1_dn2 = assign23850_e18333_d_n2;
        locals.var_t1_dn4 = assign23850_e18333_d_n4;
        locals.var_t1_dn5 = assign23850_e18333_d_n5;
        locals.var_t1_dn6 = assign23850_e18333_d_n6;
        locals.var_t1_dn7 = assign23850_e18333_d_n7;
        locals.var_t1_dn8 = assign23850_e18333_d_n8;
        locals.var_t1_dn9 = assign23850_e18333_d_n9;
        locals.var_t1_dn10 = assign23850_e18333_d_n10;
        locals.var_t1_dn11 = assign23850_e18333_d_n11;
        locals.var_t1_dn14 = assign23850_e18333_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign23860_e18341, assign23860_e18341_d_n0, assign23860_e18341_d_n2, assign23860_e18341_d_n4, assign23860_e18341_d_n5, assign23860_e18341_d_n6, assign23860_e18341_d_n7, assign23860_e18341_d_n8, assign23860_e18341_d_n9, assign23860_e18341_d_n10, assign23860_e18341_d_n11, assign23860_e18341_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard445 != 0.0)) {
        let assign23860_e18339: f64 = (locals.var_vbs_max_local - locals.var_vbs_bnd_local);
        (assign23860_e18339, (locals.var_vbs_max_local_dn0 - locals.var_vbs_bnd_local_dn0), (locals.var_vbs_max_local_dn2 - locals.var_vbs_bnd_local_dn2), (locals.var_vbs_max_local_dn4 - locals.var_vbs_bnd_local_dn4), (locals.var_vbs_max_local_dn5 - locals.var_vbs_bnd_local_dn5), (locals.var_vbs_max_local_dn6 - locals.var_vbs_bnd_local_dn6), (locals.var_vbs_max_local_dn7 - locals.var_vbs_bnd_local_dn7), (locals.var_vbs_max_local_dn8 - locals.var_vbs_bnd_local_dn8), (locals.var_vbs_max_local_dn9 - locals.var_vbs_bnd_local_dn9), (locals.var_vbs_max_local_dn10 - locals.var_vbs_bnd_local_dn10), (locals.var_vbs_max_local_dn11 - locals.var_vbs_bnd_local_dn11), (locals.var_vbs_max_local_dn14 - locals.var_vbs_bnd_local_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign23860_e18341;
        locals.var_t2_dn0 = assign23860_e18341_d_n0;
        locals.var_t2_dn2 = assign23860_e18341_d_n2;
        locals.var_t2_dn4 = assign23860_e18341_d_n4;
        locals.var_t2_dn5 = assign23860_e18341_d_n5;
        locals.var_t2_dn6 = assign23860_e18341_d_n6;
        locals.var_t2_dn7 = assign23860_e18341_d_n7;
        locals.var_t2_dn8 = assign23860_e18341_d_n8;
        locals.var_t2_dn9 = assign23860_e18341_d_n9;
        locals.var_t2_dn10 = assign23860_e18341_d_n10;
        locals.var_t2_dn11 = assign23860_e18341_d_n11;
        locals.var_t2_dn14 = assign23860_e18341_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign23870_e18349, assign23870_e18349_d_n0, assign23870_e18349_d_n2, assign23870_e18349_d_n4, assign23870_e18349_d_n5, assign23870_e18349_d_n6, assign23870_e18349_d_n7, assign23870_e18349_d_n8, assign23870_e18349_d_n9, assign23870_e18349_d_n10, assign23870_e18349_d_n11, assign23870_e18349_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard445 != 0.0)) {
        let assign23870_e18347: f64 = (locals.var_t1 / locals.var_t2);
        (assign23870_e18347, (((locals.var_t1_dn0 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn2 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn7 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn9 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn10 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn11 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn14 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn14)) / (locals.var_t2 * locals.var_t2)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign23870_e18349;
        locals.var_tmf1_dn0 = assign23870_e18349_d_n0;
        locals.var_tmf1_dn2 = assign23870_e18349_d_n2;
        locals.var_tmf1_dn4 = assign23870_e18349_d_n4;
        locals.var_tmf1_dn5 = assign23870_e18349_d_n5;
        locals.var_tmf1_dn6 = assign23870_e18349_d_n6;
        locals.var_tmf1_dn7 = assign23870_e18349_d_n7;
        locals.var_tmf1_dn8 = assign23870_e18349_d_n8;
        locals.var_tmf1_dn9 = assign23870_e18349_d_n9;
        locals.var_tmf1_dn10 = assign23870_e18349_d_n10;
        locals.var_tmf1_dn11 = assign23870_e18349_d_n11;
        locals.var_tmf1_dn14 = assign23870_e18349_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign23880_e18357, assign23880_e18357_d_n0, assign23880_e18357_d_n2, assign23880_e18357_d_n4, assign23880_e18357_d_n5, assign23880_e18357_d_n6, assign23880_e18357_d_n7, assign23880_e18357_d_n8, assign23880_e18357_d_n9, assign23880_e18357_d_n10, assign23880_e18357_d_n11, assign23880_e18357_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard445 != 0.0)) {
        let assign23880_e18355: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign23880_e18355, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign23880_e18357;
        locals.var_tmf2_dn0 = assign23880_e18357_d_n0;
        locals.var_tmf2_dn2 = assign23880_e18357_d_n2;
        locals.var_tmf2_dn4 = assign23880_e18357_d_n4;
        locals.var_tmf2_dn5 = assign23880_e18357_d_n5;
        locals.var_tmf2_dn6 = assign23880_e18357_d_n6;
        locals.var_tmf2_dn7 = assign23880_e18357_d_n7;
        locals.var_tmf2_dn8 = assign23880_e18357_d_n8;
        locals.var_tmf2_dn9 = assign23880_e18357_d_n9;
        locals.var_tmf2_dn10 = assign23880_e18357_d_n10;
        locals.var_tmf2_dn11 = assign23880_e18357_d_n11;
        locals.var_tmf2_dn14 = assign23880_e18357_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign23890_e18365, assign23890_e18365_d_n0, assign23890_e18365_d_n2, assign23890_e18365_d_n4, assign23890_e18365_d_n5, assign23890_e18365_d_n6, assign23890_e18365_d_n7, assign23890_e18365_d_n8, assign23890_e18365_d_n9, assign23890_e18365_d_n10, assign23890_e18365_d_n11, assign23890_e18365_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard445 != 0.0)) {
        let assign23890_e18363: f64 = (locals.var_tmf2 * locals.var_tmf1);
        (assign23890_e18363, ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn11)), ((locals.var_tmf2_dn14 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign23890_e18365;
        locals.var_tmf3_dn0 = assign23890_e18365_d_n0;
        locals.var_tmf3_dn2 = assign23890_e18365_d_n2;
        locals.var_tmf3_dn4 = assign23890_e18365_d_n4;
        locals.var_tmf3_dn5 = assign23890_e18365_d_n5;
        locals.var_tmf3_dn6 = assign23890_e18365_d_n6;
        locals.var_tmf3_dn7 = assign23890_e18365_d_n7;
        locals.var_tmf3_dn8 = assign23890_e18365_d_n8;
        locals.var_tmf3_dn9 = assign23890_e18365_d_n9;
        locals.var_tmf3_dn10 = assign23890_e18365_d_n10;
        locals.var_tmf3_dn11 = assign23890_e18365_d_n11;
        locals.var_tmf3_dn14 = assign23890_e18365_d_n14;
        locals.var_tmf3_rv = 0.0;

        let (assign23900_e18373, assign23900_e18373_d_n0, assign23900_e18373_d_n2, assign23900_e18373_d_n4, assign23900_e18373_d_n5, assign23900_e18373_d_n6, assign23900_e18373_d_n7, assign23900_e18373_d_n8, assign23900_e18373_d_n9, assign23900_e18373_d_n10, assign23900_e18373_d_n11, assign23900_e18373_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard445 != 0.0)) {
        let assign23900_e18371: f64 = (locals.var_tmf2 * locals.var_tmf2);
        (assign23900_e18371, ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)), ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn8, locals.var_tmf4_dn9, locals.var_tmf4_dn10, locals.var_tmf4_dn11, locals.var_tmf4_dn14,)
    }
};
        locals.var_tmf4 = assign23900_e18373;
        locals.var_tmf4_dn0 = assign23900_e18373_d_n0;
        locals.var_tmf4_dn2 = assign23900_e18373_d_n2;
        locals.var_tmf4_dn4 = assign23900_e18373_d_n4;
        locals.var_tmf4_dn5 = assign23900_e18373_d_n5;
        locals.var_tmf4_dn6 = assign23900_e18373_d_n6;
        locals.var_tmf4_dn7 = assign23900_e18373_d_n7;
        locals.var_tmf4_dn8 = assign23900_e18373_d_n8;
        locals.var_tmf4_dn9 = assign23900_e18373_d_n9;
        locals.var_tmf4_dn10 = assign23900_e18373_d_n10;
        locals.var_tmf4_dn11 = assign23900_e18373_d_n11;
        locals.var_tmf4_dn14 = assign23900_e18373_d_n14;
        locals.var_tmf4_rv = 0.0;

        let (assign23910_e18389, assign23910_e18389_d_n0, assign23910_e18389_d_n2, assign23910_e18389_d_n4, assign23910_e18389_d_n5, assign23910_e18389_d_n6, assign23910_e18389_d_n7, assign23910_e18389_d_n8, assign23910_e18389_d_n9, assign23910_e18389_d_n10, assign23910_e18389_d_n11, assign23910_e18389_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard445 != 0.0)) {
        let assign23910_e18380: f64 = (1.0 + locals.var_tmf1);
        let assign23910_e18382: f64 = (assign23910_e18380 + locals.var_tmf2);
        let assign23910_e18384: f64 = (assign23910_e18382 + locals.var_tmf3);
        let assign23910_e18386: f64 = (assign23910_e18384 + locals.var_tmf4);
        let assign23910_e18387: f64 = (1.0 / assign23910_e18386);
        (assign23910_e18387, (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign23910_e18386 * assign23910_e18386))), (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign23910_e18386 * assign23910_e18386))), (-((((locals.var_tmf1_dn4 + locals.var_tmf2_dn4) + locals.var_tmf3_dn4) + locals.var_tmf4_dn4) / (assign23910_e18386 * assign23910_e18386))), (-((((locals.var_tmf1_dn5 + locals.var_tmf2_dn5) + locals.var_tmf3_dn5) + locals.var_tmf4_dn5) / (assign23910_e18386 * assign23910_e18386))), (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign23910_e18386 * assign23910_e18386))), (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign23910_e18386 * assign23910_e18386))), (-((((locals.var_tmf1_dn8 + locals.var_tmf2_dn8) + locals.var_tmf3_dn8) + locals.var_tmf4_dn8) / (assign23910_e18386 * assign23910_e18386))), (-((((locals.var_tmf1_dn9 + locals.var_tmf2_dn9) + locals.var_tmf3_dn9) + locals.var_tmf4_dn9) / (assign23910_e18386 * assign23910_e18386))), (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign23910_e18386 * assign23910_e18386))), (-((((locals.var_tmf1_dn11 + locals.var_tmf2_dn11) + locals.var_tmf3_dn11) + locals.var_tmf4_dn11) / (assign23910_e18386 * assign23910_e18386))), (-((((locals.var_tmf1_dn14 + locals.var_tmf2_dn14) + locals.var_tmf3_dn14) + locals.var_tmf4_dn14) / (assign23910_e18386 * assign23910_e18386))),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign23910_e18389;
        locals.var_tmf0_dn0 = assign23910_e18389_d_n0;
        locals.var_tmf0_dn2 = assign23910_e18389_d_n2;
        locals.var_tmf0_dn4 = assign23910_e18389_d_n4;
        locals.var_tmf0_dn5 = assign23910_e18389_d_n5;
        locals.var_tmf0_dn6 = assign23910_e18389_d_n6;
        locals.var_tmf0_dn7 = assign23910_e18389_d_n7;
        locals.var_tmf0_dn8 = assign23910_e18389_d_n8;
        locals.var_tmf0_dn9 = assign23910_e18389_d_n9;
        locals.var_tmf0_dn10 = assign23910_e18389_d_n10;
        locals.var_tmf0_dn11 = assign23910_e18389_d_n11;
        locals.var_tmf0_dn14 = assign23910_e18389_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign23920_e18412, assign23920_e18412_d_n0, assign23920_e18412_d_n2, assign23920_e18412_d_n4, assign23920_e18412_d_n5, assign23920_e18412_d_n6, assign23920_e18412_d_n7, assign23920_e18412_d_n8, assign23920_e18412_d_n9, assign23920_e18412_d_n10, assign23920_e18412_d_n11, assign23920_e18412_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard445 != 0.0)) {
        let assign23920_e18396: f64 = (2.0 * locals.var_tmf1);
        let assign23920_e18397: f64 = (1.0 + assign23920_e18396);
        let assign23920_e18400: f64 = (3.0 * locals.var_tmf2);
        let assign23920_e18401: f64 = (assign23920_e18397 + assign23920_e18400);
        let assign23920_e18404: f64 = (4.0 * locals.var_tmf3);
        let assign23920_e18405: f64 = (assign23920_e18401 + assign23920_e18404);
        let assign23920_e18406: f64 = (-assign23920_e18405);
        let assign23920_e18408: f64 = (assign23920_e18406 * locals.var_tmf0);
        let assign23920_e18410: f64 = (assign23920_e18408 * locals.var_tmf0);
        (assign23920_e18410, (((((-(((2.0 * locals.var_tmf1_dn0) + (3.0 * locals.var_tmf2_dn0)) + (4.0 * locals.var_tmf3_dn0))) * locals.var_tmf0) + (assign23920_e18406 * locals.var_tmf0_dn0)) * locals.var_tmf0) + (assign23920_e18408 * locals.var_tmf0_dn0)), (((((-(((2.0 * locals.var_tmf1_dn2) + (3.0 * locals.var_tmf2_dn2)) + (4.0 * locals.var_tmf3_dn2))) * locals.var_tmf0) + (assign23920_e18406 * locals.var_tmf0_dn2)) * locals.var_tmf0) + (assign23920_e18408 * locals.var_tmf0_dn2)), (((((-(((2.0 * locals.var_tmf1_dn4) + (3.0 * locals.var_tmf2_dn4)) + (4.0 * locals.var_tmf3_dn4))) * locals.var_tmf0) + (assign23920_e18406 * locals.var_tmf0_dn4)) * locals.var_tmf0) + (assign23920_e18408 * locals.var_tmf0_dn4)), (((((-(((2.0 * locals.var_tmf1_dn5) + (3.0 * locals.var_tmf2_dn5)) + (4.0 * locals.var_tmf3_dn5))) * locals.var_tmf0) + (assign23920_e18406 * locals.var_tmf0_dn5)) * locals.var_tmf0) + (assign23920_e18408 * locals.var_tmf0_dn5)), (((((-(((2.0 * locals.var_tmf1_dn6) + (3.0 * locals.var_tmf2_dn6)) + (4.0 * locals.var_tmf3_dn6))) * locals.var_tmf0) + (assign23920_e18406 * locals.var_tmf0_dn6)) * locals.var_tmf0) + (assign23920_e18408 * locals.var_tmf0_dn6)), (((((-(((2.0 * locals.var_tmf1_dn7) + (3.0 * locals.var_tmf2_dn7)) + (4.0 * locals.var_tmf3_dn7))) * locals.var_tmf0) + (assign23920_e18406 * locals.var_tmf0_dn7)) * locals.var_tmf0) + (assign23920_e18408 * locals.var_tmf0_dn7)), (((((-(((2.0 * locals.var_tmf1_dn8) + (3.0 * locals.var_tmf2_dn8)) + (4.0 * locals.var_tmf3_dn8))) * locals.var_tmf0) + (assign23920_e18406 * locals.var_tmf0_dn8)) * locals.var_tmf0) + (assign23920_e18408 * locals.var_tmf0_dn8)), (((((-(((2.0 * locals.var_tmf1_dn9) + (3.0 * locals.var_tmf2_dn9)) + (4.0 * locals.var_tmf3_dn9))) * locals.var_tmf0) + (assign23920_e18406 * locals.var_tmf0_dn9)) * locals.var_tmf0) + (assign23920_e18408 * locals.var_tmf0_dn9)), (((((-(((2.0 * locals.var_tmf1_dn10) + (3.0 * locals.var_tmf2_dn10)) + (4.0 * locals.var_tmf3_dn10))) * locals.var_tmf0) + (assign23920_e18406 * locals.var_tmf0_dn10)) * locals.var_tmf0) + (assign23920_e18408 * locals.var_tmf0_dn10)), (((((-(((2.0 * locals.var_tmf1_dn11) + (3.0 * locals.var_tmf2_dn11)) + (4.0 * locals.var_tmf3_dn11))) * locals.var_tmf0) + (assign23920_e18406 * locals.var_tmf0_dn11)) * locals.var_tmf0) + (assign23920_e18408 * locals.var_tmf0_dn11)), (((((-(((2.0 * locals.var_tmf1_dn14) + (3.0 * locals.var_tmf2_dn14)) + (4.0 * locals.var_tmf3_dn14))) * locals.var_tmf0) + (assign23920_e18406 * locals.var_tmf0_dn14)) * locals.var_tmf0) + (assign23920_e18408 * locals.var_tmf0_dn14)),)
    } else {
        (locals.var_vbscldvbs__blk440, locals.var_vbscldvbs__blk440_dn0, locals.var_vbscldvbs__blk440_dn2, locals.var_vbscldvbs__blk440_dn4, locals.var_vbscldvbs__blk440_dn5, locals.var_vbscldvbs__blk440_dn6, locals.var_vbscldvbs__blk440_dn7, locals.var_vbscldvbs__blk440_dn8, locals.var_vbscldvbs__blk440_dn9, locals.var_vbscldvbs__blk440_dn10, locals.var_vbscldvbs__blk440_dn11, locals.var_vbscldvbs__blk440_dn14,)
    }
};
        locals.var_vbscldvbs__blk440 = assign23920_e18412;
        locals.var_vbscldvbs__blk440_dn0 = assign23920_e18412_d_n0;
        locals.var_vbscldvbs__blk440_dn2 = assign23920_e18412_d_n2;
        locals.var_vbscldvbs__blk440_dn4 = assign23920_e18412_d_n4;
        locals.var_vbscldvbs__blk440_dn5 = assign23920_e18412_d_n5;
        locals.var_vbscldvbs__blk440_dn6 = assign23920_e18412_d_n6;
        locals.var_vbscldvbs__blk440_dn7 = assign23920_e18412_d_n7;
        locals.var_vbscldvbs__blk440_dn8 = assign23920_e18412_d_n8;
        locals.var_vbscldvbs__blk440_dn9 = assign23920_e18412_d_n9;
        locals.var_vbscldvbs__blk440_dn10 = assign23920_e18412_d_n10;
        locals.var_vbscldvbs__blk440_dn11 = assign23920_e18412_d_n11;
        locals.var_vbscldvbs__blk440_dn14 = assign23920_e18412_d_n14;
        locals.var_vbscldvbs__blk440_rv = 0.0;

        let (assign23930_e18422, assign23930_e18422_d_n0, assign23930_e18422_d_n2, assign23930_e18422_d_n4, assign23930_e18422_d_n5, assign23930_e18422_d_n6, assign23930_e18422_d_n7, assign23930_e18422_d_n8, assign23930_e18422_d_n9, assign23930_e18422_d_n10, assign23930_e18422_d_n11, assign23930_e18422_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard445 != 0.0)) {
        let assign23930_e18419: f64 = (1.0 - locals.var_tmf0);
        let assign23930_e18420: f64 = (locals.var_t2 * assign23930_e18419);
        (assign23930_e18420, ((locals.var_t2_dn0 * assign23930_e18419) + (locals.var_t2 * (-locals.var_tmf0_dn0))), ((locals.var_t2_dn2 * assign23930_e18419) + (locals.var_t2 * (-locals.var_tmf0_dn2))), ((locals.var_t2_dn4 * assign23930_e18419) + (locals.var_t2 * (-locals.var_tmf0_dn4))), ((locals.var_t2_dn5 * assign23930_e18419) + (locals.var_t2 * (-locals.var_tmf0_dn5))), ((locals.var_t2_dn6 * assign23930_e18419) + (locals.var_t2 * (-locals.var_tmf0_dn6))), ((locals.var_t2_dn7 * assign23930_e18419) + (locals.var_t2 * (-locals.var_tmf0_dn7))), ((locals.var_t2_dn8 * assign23930_e18419) + (locals.var_t2 * (-locals.var_tmf0_dn8))), ((locals.var_t2_dn9 * assign23930_e18419) + (locals.var_t2 * (-locals.var_tmf0_dn9))), ((locals.var_t2_dn10 * assign23930_e18419) + (locals.var_t2 * (-locals.var_tmf0_dn10))), ((locals.var_t2_dn11 * assign23930_e18419) + (locals.var_t2 * (-locals.var_tmf0_dn11))), ((locals.var_t2_dn14 * assign23930_e18419) + (locals.var_t2 * (-locals.var_tmf0_dn14))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign23930_e18422;
        locals.var_ty_dn0 = assign23930_e18422_d_n0;
        locals.var_ty_dn2 = assign23930_e18422_d_n2;
        locals.var_ty_dn4 = assign23930_e18422_d_n4;
        locals.var_ty_dn5 = assign23930_e18422_d_n5;
        locals.var_ty_dn6 = assign23930_e18422_d_n6;
        locals.var_ty_dn7 = assign23930_e18422_d_n7;
        locals.var_ty_dn8 = assign23930_e18422_d_n8;
        locals.var_ty_dn9 = assign23930_e18422_d_n9;
        locals.var_ty_dn10 = assign23930_e18422_d_n10;
        locals.var_ty_dn11 = assign23930_e18422_d_n11;
        locals.var_ty_dn14 = assign23930_e18422_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign23940_e18434, assign23940_e18434_d_n0, assign23940_e18434_d_n2, assign23940_e18434_d_n4, assign23940_e18434_d_n5, assign23940_e18434_d_n6, assign23940_e18434_d_n7, assign23940_e18434_d_n8, assign23940_e18434_d_n9, assign23940_e18434_d_n10, assign23940_e18434_d_n11, assign23940_e18434_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard445 != 0.0)) {
        let assign23940_e18428: f64 = (1.0 - locals.var_tmf0);
        let assign23940_e18431: f64 = (locals.var_tmf1 * locals.var_vbscldvbs__blk440);
        let assign23940_e18432: f64 = (assign23940_e18428 + assign23940_e18431);
        (assign23940_e18432, ((-locals.var_tmf0_dn0) + ((locals.var_tmf1_dn0 * locals.var_vbscldvbs__blk440) + (locals.var_tmf1 * locals.var_vbscldvbs__blk440_dn0))), ((-locals.var_tmf0_dn2) + ((locals.var_tmf1_dn2 * locals.var_vbscldvbs__blk440) + (locals.var_tmf1 * locals.var_vbscldvbs__blk440_dn2))), ((-locals.var_tmf0_dn4) + ((locals.var_tmf1_dn4 * locals.var_vbscldvbs__blk440) + (locals.var_tmf1 * locals.var_vbscldvbs__blk440_dn4))), ((-locals.var_tmf0_dn5) + ((locals.var_tmf1_dn5 * locals.var_vbscldvbs__blk440) + (locals.var_tmf1 * locals.var_vbscldvbs__blk440_dn5))), ((-locals.var_tmf0_dn6) + ((locals.var_tmf1_dn6 * locals.var_vbscldvbs__blk440) + (locals.var_tmf1 * locals.var_vbscldvbs__blk440_dn6))), ((-locals.var_tmf0_dn7) + ((locals.var_tmf1_dn7 * locals.var_vbscldvbs__blk440) + (locals.var_tmf1 * locals.var_vbscldvbs__blk440_dn7))), ((-locals.var_tmf0_dn8) + ((locals.var_tmf1_dn8 * locals.var_vbscldvbs__blk440) + (locals.var_tmf1 * locals.var_vbscldvbs__blk440_dn8))), ((-locals.var_tmf0_dn9) + ((locals.var_tmf1_dn9 * locals.var_vbscldvbs__blk440) + (locals.var_tmf1 * locals.var_vbscldvbs__blk440_dn9))), ((-locals.var_tmf0_dn10) + ((locals.var_tmf1_dn10 * locals.var_vbscldvbs__blk440) + (locals.var_tmf1 * locals.var_vbscldvbs__blk440_dn10))), ((-locals.var_tmf0_dn11) + ((locals.var_tmf1_dn11 * locals.var_vbscldvbs__blk440) + (locals.var_tmf1 * locals.var_vbscldvbs__blk440_dn11))), ((-locals.var_tmf0_dn14) + ((locals.var_tmf1_dn14 * locals.var_vbscldvbs__blk440) + (locals.var_tmf1 * locals.var_vbscldvbs__blk440_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign23940_e18434;
        locals.var_t0_dn0 = assign23940_e18434_d_n0;
        locals.var_t0_dn2 = assign23940_e18434_d_n2;
        locals.var_t0_dn4 = assign23940_e18434_d_n4;
        locals.var_t0_dn5 = assign23940_e18434_d_n5;
        locals.var_t0_dn6 = assign23940_e18434_d_n6;
        locals.var_t0_dn7 = assign23940_e18434_d_n7;
        locals.var_t0_dn8 = assign23940_e18434_d_n8;
        locals.var_t0_dn9 = assign23940_e18434_d_n9;
        locals.var_t0_dn10 = assign23940_e18434_d_n10;
        locals.var_t0_dn11 = assign23940_e18434_d_n11;
        locals.var_t0_dn14 = assign23940_e18434_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign23950_e18441, assign23950_e18441_d_n0, assign23950_e18441_d_n2, assign23950_e18441_d_n4, assign23950_e18441_d_n5, assign23950_e18441_d_n6, assign23950_e18441_d_n7, assign23950_e18441_d_n8, assign23950_e18441_d_n9, assign23950_e18441_d_n10, assign23950_e18441_d_n11, assign23950_e18441_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard445 != 0.0)) {
        let assign23950_e18439: f64 = (-locals.var_vbscldvbs__blk440);
        (assign23950_e18439, (-locals.var_vbscldvbs__blk440_dn0), (-locals.var_vbscldvbs__blk440_dn2), (-locals.var_vbscldvbs__blk440_dn4), (-locals.var_vbscldvbs__blk440_dn5), (-locals.var_vbscldvbs__blk440_dn6), (-locals.var_vbscldvbs__blk440_dn7), (-locals.var_vbscldvbs__blk440_dn8), (-locals.var_vbscldvbs__blk440_dn9), (-locals.var_vbscldvbs__blk440_dn10), (-locals.var_vbscldvbs__blk440_dn11), (-locals.var_vbscldvbs__blk440_dn14),)
    } else {
        (locals.var_vbscldvbs__blk440, locals.var_vbscldvbs__blk440_dn0, locals.var_vbscldvbs__blk440_dn2, locals.var_vbscldvbs__blk440_dn4, locals.var_vbscldvbs__blk440_dn5, locals.var_vbscldvbs__blk440_dn6, locals.var_vbscldvbs__blk440_dn7, locals.var_vbscldvbs__blk440_dn8, locals.var_vbscldvbs__blk440_dn9, locals.var_vbscldvbs__blk440_dn10, locals.var_vbscldvbs__blk440_dn11, locals.var_vbscldvbs__blk440_dn14,)
    }
};
        locals.var_vbscldvbs__blk440 = assign23950_e18441;
        locals.var_vbscldvbs__blk440_dn0 = assign23950_e18441_d_n0;
        locals.var_vbscldvbs__blk440_dn2 = assign23950_e18441_d_n2;
        locals.var_vbscldvbs__blk440_dn4 = assign23950_e18441_d_n4;
        locals.var_vbscldvbs__blk440_dn5 = assign23950_e18441_d_n5;
        locals.var_vbscldvbs__blk440_dn6 = assign23950_e18441_d_n6;
        locals.var_vbscldvbs__blk440_dn7 = assign23950_e18441_d_n7;
        locals.var_vbscldvbs__blk440_dn8 = assign23950_e18441_d_n8;
        locals.var_vbscldvbs__blk440_dn9 = assign23950_e18441_d_n9;
        locals.var_vbscldvbs__blk440_dn10 = assign23950_e18441_d_n10;
        locals.var_vbscldvbs__blk440_dn11 = assign23950_e18441_d_n11;
        locals.var_vbscldvbs__blk440_dn14 = assign23950_e18441_d_n14;
        locals.var_vbscldvbs__blk440_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_66(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23960_e18449, assign23960_e18449_d_n0, assign23960_e18449_d_n2, assign23960_e18449_d_n4, assign23960_e18449_d_n5, assign23960_e18449_d_n6, assign23960_e18449_d_n7, assign23960_e18449_d_n8, assign23960_e18449_d_n9, assign23960_e18449_d_n10, assign23960_e18449_d_n11, assign23960_e18449_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard445 != 0.0)) {
        let assign23960_e18447: f64 = (locals.var_vbs_bnd_local + locals.var_ty);
        (assign23960_e18447, (locals.var_vbs_bnd_local_dn0 + locals.var_ty_dn0), (locals.var_vbs_bnd_local_dn2 + locals.var_ty_dn2), (locals.var_vbs_bnd_local_dn4 + locals.var_ty_dn4), (locals.var_vbs_bnd_local_dn5 + locals.var_ty_dn5), (locals.var_vbs_bnd_local_dn6 + locals.var_ty_dn6), (locals.var_vbs_bnd_local_dn7 + locals.var_ty_dn7), (locals.var_vbs_bnd_local_dn8 + locals.var_ty_dn8), (locals.var_vbs_bnd_local_dn9 + locals.var_ty_dn9), (locals.var_vbs_bnd_local_dn10 + locals.var_ty_dn10), (locals.var_vbs_bnd_local_dn11 + locals.var_ty_dn11), (locals.var_vbs_bnd_local_dn14 + locals.var_ty_dn14),)
    } else {
        (locals.var_vbscl__blk439, locals.var_vbscl__blk439_dn0, locals.var_vbscl__blk439_dn2, locals.var_vbscl__blk439_dn4, locals.var_vbscl__blk439_dn5, locals.var_vbscl__blk439_dn6, locals.var_vbscl__blk439_dn7, locals.var_vbscl__blk439_dn8, locals.var_vbscl__blk439_dn9, locals.var_vbscl__blk439_dn10, locals.var_vbscl__blk439_dn11, locals.var_vbscl__blk439_dn14,)
    }
};
        locals.var_vbscl__blk439 = assign23960_e18449;
        locals.var_vbscl__blk439_dn0 = assign23960_e18449_d_n0;
        locals.var_vbscl__blk439_dn2 = assign23960_e18449_d_n2;
        locals.var_vbscl__blk439_dn4 = assign23960_e18449_d_n4;
        locals.var_vbscl__blk439_dn5 = assign23960_e18449_d_n5;
        locals.var_vbscl__blk439_dn6 = assign23960_e18449_d_n6;
        locals.var_vbscl__blk439_dn7 = assign23960_e18449_d_n7;
        locals.var_vbscl__blk439_dn8 = assign23960_e18449_d_n8;
        locals.var_vbscl__blk439_dn9 = assign23960_e18449_d_n9;
        locals.var_vbscl__blk439_dn10 = assign23960_e18449_d_n10;
        locals.var_vbscl__blk439_dn11 = assign23960_e18449_d_n11;
        locals.var_vbscl__blk439_dn14 = assign23960_e18449_d_n14;
        locals.var_vbscl__blk439_rv = 0.0;

        let (assign23970_e18457, assign23970_e18457_d_n0, assign23970_e18457_d_n2, assign23970_e18457_d_n4, assign23970_e18457_d_n5, assign23970_e18457_d_n6, assign23970_e18457_d_n7, assign23970_e18457_d_n8, assign23970_e18457_d_n9, assign23970_e18457_d_n10, assign23970_e18457_d_n11, assign23970_e18457_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard445 != 0.0)) {
        let assign23970_e18455: f64 = (1.0 / locals.var_t2);
        (assign23970_e18455, (-(locals.var_t2_dn0 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn2 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn9 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn10 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn11 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn14 / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign23970_e18457;
        locals.var_t3_dn0 = assign23970_e18457_d_n0;
        locals.var_t3_dn2 = assign23970_e18457_d_n2;
        locals.var_t3_dn4 = assign23970_e18457_d_n4;
        locals.var_t3_dn5 = assign23970_e18457_d_n5;
        locals.var_t3_dn6 = assign23970_e18457_d_n6;
        locals.var_t3_dn7 = assign23970_e18457_d_n7;
        locals.var_t3_dn8 = assign23970_e18457_d_n8;
        locals.var_t3_dn9 = assign23970_e18457_d_n9;
        locals.var_t3_dn10 = assign23970_e18457_d_n10;
        locals.var_t3_dn11 = assign23970_e18457_d_n11;
        locals.var_t3_dn14 = assign23970_e18457_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign23980_e18465, assign23980_e18465_d_n0, assign23980_e18465_d_n2, assign23980_e18465_d_n4, assign23980_e18465_d_n5, assign23980_e18465_d_n6, assign23980_e18465_d_n7, assign23980_e18465_d_n8, assign23980_e18465_d_n9, assign23980_e18465_d_n10, assign23980_e18465_d_n11, assign23980_e18465_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard445 != 0.0)) {
        let assign23980_e18463: f64 = (locals.var_t1 * locals.var_t3);
        (assign23980_e18463, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)), ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)), ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)), ((locals.var_t1_dn14 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign23980_e18465;
        locals.var_t4_dn0 = assign23980_e18465_d_n0;
        locals.var_t4_dn2 = assign23980_e18465_d_n2;
        locals.var_t4_dn4 = assign23980_e18465_d_n4;
        locals.var_t4_dn5 = assign23980_e18465_d_n5;
        locals.var_t4_dn6 = assign23980_e18465_d_n6;
        locals.var_t4_dn7 = assign23980_e18465_d_n7;
        locals.var_t4_dn8 = assign23980_e18465_d_n8;
        locals.var_t4_dn9 = assign23980_e18465_d_n9;
        locals.var_t4_dn10 = assign23980_e18465_d_n10;
        locals.var_t4_dn11 = assign23980_e18465_d_n11;
        locals.var_t4_dn14 = assign23980_e18465_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign23990_e18473, assign23990_e18473_d_n0, assign23990_e18473_d_n2, assign23990_e18473_d_n4, assign23990_e18473_d_n5, assign23990_e18473_d_n6, assign23990_e18473_d_n7, assign23990_e18473_d_n8, assign23990_e18473_d_n9, assign23990_e18473_d_n10, assign23990_e18473_d_n11, assign23990_e18473_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard445 != 0.0)) {
        let assign23990_e18471: f64 = (locals.var_t4 * locals.var_t4);
        (assign23990_e18471, ((locals.var_t4_dn0 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn0)), ((locals.var_t4_dn2 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn2)), ((locals.var_t4_dn4 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn4)), ((locals.var_t4_dn5 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn5)), ((locals.var_t4_dn6 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn6)), ((locals.var_t4_dn7 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn7)), ((locals.var_t4_dn8 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn8)), ((locals.var_t4_dn9 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn9)), ((locals.var_t4_dn10 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn10)), ((locals.var_t4_dn11 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn11)), ((locals.var_t4_dn14 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign23990_e18473;
        locals.var_t5_dn0 = assign23990_e18473_d_n0;
        locals.var_t5_dn2 = assign23990_e18473_d_n2;
        locals.var_t5_dn4 = assign23990_e18473_d_n4;
        locals.var_t5_dn5 = assign23990_e18473_d_n5;
        locals.var_t5_dn6 = assign23990_e18473_d_n6;
        locals.var_t5_dn7 = assign23990_e18473_d_n7;
        locals.var_t5_dn8 = assign23990_e18473_d_n8;
        locals.var_t5_dn9 = assign23990_e18473_d_n9;
        locals.var_t5_dn10 = assign23990_e18473_d_n10;
        locals.var_t5_dn11 = assign23990_e18473_d_n11;
        locals.var_t5_dn14 = assign23990_e18473_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign24000_e18489, assign24000_e18489_d_n0, assign24000_e18489_d_n2, assign24000_e18489_d_n4, assign24000_e18489_d_n5, assign24000_e18489_d_n6, assign24000_e18489_d_n7, assign24000_e18489_d_n8, assign24000_e18489_d_n9, assign24000_e18489_d_n10, assign24000_e18489_d_n11, assign24000_e18489_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard445 != 0.0)) {
        let assign24000_e18479: f64 = (1.0 + locals.var_t4);
        let assign24000_e18483: f64 = (1.0 + locals.var_t4);
        let assign24000_e18485: f64 = (assign24000_e18483 + locals.var_t5);
        let assign24000_e18486: f64 = (locals.var_t5 * assign24000_e18485);
        let assign24000_e18487: f64 = (assign24000_e18479 + assign24000_e18486);
        (assign24000_e18487, (locals.var_t4_dn0 + ((locals.var_t5_dn0 * assign24000_e18485) + (locals.var_t5 * (locals.var_t4_dn0 + locals.var_t5_dn0)))), (locals.var_t4_dn2 + ((locals.var_t5_dn2 * assign24000_e18485) + (locals.var_t5 * (locals.var_t4_dn2 + locals.var_t5_dn2)))), (locals.var_t4_dn4 + ((locals.var_t5_dn4 * assign24000_e18485) + (locals.var_t5 * (locals.var_t4_dn4 + locals.var_t5_dn4)))), (locals.var_t4_dn5 + ((locals.var_t5_dn5 * assign24000_e18485) + (locals.var_t5 * (locals.var_t4_dn5 + locals.var_t5_dn5)))), (locals.var_t4_dn6 + ((locals.var_t5_dn6 * assign24000_e18485) + (locals.var_t5 * (locals.var_t4_dn6 + locals.var_t5_dn6)))), (locals.var_t4_dn7 + ((locals.var_t5_dn7 * assign24000_e18485) + (locals.var_t5 * (locals.var_t4_dn7 + locals.var_t5_dn7)))), (locals.var_t4_dn8 + ((locals.var_t5_dn8 * assign24000_e18485) + (locals.var_t5 * (locals.var_t4_dn8 + locals.var_t5_dn8)))), (locals.var_t4_dn9 + ((locals.var_t5_dn9 * assign24000_e18485) + (locals.var_t5 * (locals.var_t4_dn9 + locals.var_t5_dn9)))), (locals.var_t4_dn10 + ((locals.var_t5_dn10 * assign24000_e18485) + (locals.var_t5 * (locals.var_t4_dn10 + locals.var_t5_dn10)))), (locals.var_t4_dn11 + ((locals.var_t5_dn11 * assign24000_e18485) + (locals.var_t5 * (locals.var_t4_dn11 + locals.var_t5_dn11)))), (locals.var_t4_dn14 + ((locals.var_t5_dn14 * assign24000_e18485) + (locals.var_t5 * (locals.var_t4_dn14 + locals.var_t5_dn14)))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign24000_e18489;
        locals.var_t7_dn0 = assign24000_e18489_d_n0;
        locals.var_t7_dn2 = assign24000_e18489_d_n2;
        locals.var_t7_dn4 = assign24000_e18489_d_n4;
        locals.var_t7_dn5 = assign24000_e18489_d_n5;
        locals.var_t7_dn6 = assign24000_e18489_d_n6;
        locals.var_t7_dn7 = assign24000_e18489_d_n7;
        locals.var_t7_dn8 = assign24000_e18489_d_n8;
        locals.var_t7_dn9 = assign24000_e18489_d_n9;
        locals.var_t7_dn10 = assign24000_e18489_d_n10;
        locals.var_t7_dn11 = assign24000_e18489_d_n11;
        locals.var_t7_dn14 = assign24000_e18489_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign24010_e18513, assign24010_e18513_d_n0, assign24010_e18513_d_n2, assign24010_e18513_d_n4, assign24010_e18513_d_n5, assign24010_e18513_d_n6, assign24010_e18513_d_n7, assign24010_e18513_d_n8, assign24010_e18513_d_n9, assign24010_e18513_d_n10, assign24010_e18513_d_n11, assign24010_e18513_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard445 != 0.0)) {
        let assign24010_e18496: f64 = (2.0 * locals.var_t4);
        let assign24010_e18497: f64 = (1.0 + assign24010_e18496);
        let assign24010_e18500: f64 = (3.0 * locals.var_t5);
        let assign24010_e18501: f64 = (assign24010_e18497 + assign24010_e18500);
        let assign24010_e18504: f64 = (4.0 * locals.var_t4);
        let assign24010_e18506: f64 = (assign24010_e18504 * locals.var_t5);
        let assign24010_e18507: f64 = (assign24010_e18501 + assign24010_e18506);
        let assign24010_e18510: f64 = (locals.var_t7 * locals.var_t7);
        let assign24010_e18511: f64 = (assign24010_e18507 / assign24010_e18510);
        (assign24010_e18511, ((((((2.0 * locals.var_t4_dn0) + (3.0 * locals.var_t5_dn0)) + (((4.0 * locals.var_t4_dn0) * locals.var_t5) + (assign24010_e18504 * locals.var_t5_dn0))) * assign24010_e18510) - (assign24010_e18507 * ((locals.var_t7_dn0 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn0)))) / (assign24010_e18510 * assign24010_e18510)), ((((((2.0 * locals.var_t4_dn2) + (3.0 * locals.var_t5_dn2)) + (((4.0 * locals.var_t4_dn2) * locals.var_t5) + (assign24010_e18504 * locals.var_t5_dn2))) * assign24010_e18510) - (assign24010_e18507 * ((locals.var_t7_dn2 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn2)))) / (assign24010_e18510 * assign24010_e18510)), ((((((2.0 * locals.var_t4_dn4) + (3.0 * locals.var_t5_dn4)) + (((4.0 * locals.var_t4_dn4) * locals.var_t5) + (assign24010_e18504 * locals.var_t5_dn4))) * assign24010_e18510) - (assign24010_e18507 * ((locals.var_t7_dn4 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn4)))) / (assign24010_e18510 * assign24010_e18510)), ((((((2.0 * locals.var_t4_dn5) + (3.0 * locals.var_t5_dn5)) + (((4.0 * locals.var_t4_dn5) * locals.var_t5) + (assign24010_e18504 * locals.var_t5_dn5))) * assign24010_e18510) - (assign24010_e18507 * ((locals.var_t7_dn5 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn5)))) / (assign24010_e18510 * assign24010_e18510)), ((((((2.0 * locals.var_t4_dn6) + (3.0 * locals.var_t5_dn6)) + (((4.0 * locals.var_t4_dn6) * locals.var_t5) + (assign24010_e18504 * locals.var_t5_dn6))) * assign24010_e18510) - (assign24010_e18507 * ((locals.var_t7_dn6 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn6)))) / (assign24010_e18510 * assign24010_e18510)), ((((((2.0 * locals.var_t4_dn7) + (3.0 * locals.var_t5_dn7)) + (((4.0 * locals.var_t4_dn7) * locals.var_t5) + (assign24010_e18504 * locals.var_t5_dn7))) * assign24010_e18510) - (assign24010_e18507 * ((locals.var_t7_dn7 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn7)))) / (assign24010_e18510 * assign24010_e18510)), ((((((2.0 * locals.var_t4_dn8) + (3.0 * locals.var_t5_dn8)) + (((4.0 * locals.var_t4_dn8) * locals.var_t5) + (assign24010_e18504 * locals.var_t5_dn8))) * assign24010_e18510) - (assign24010_e18507 * ((locals.var_t7_dn8 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn8)))) / (assign24010_e18510 * assign24010_e18510)), ((((((2.0 * locals.var_t4_dn9) + (3.0 * locals.var_t5_dn9)) + (((4.0 * locals.var_t4_dn9) * locals.var_t5) + (assign24010_e18504 * locals.var_t5_dn9))) * assign24010_e18510) - (assign24010_e18507 * ((locals.var_t7_dn9 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn9)))) / (assign24010_e18510 * assign24010_e18510)), ((((((2.0 * locals.var_t4_dn10) + (3.0 * locals.var_t5_dn10)) + (((4.0 * locals.var_t4_dn10) * locals.var_t5) + (assign24010_e18504 * locals.var_t5_dn10))) * assign24010_e18510) - (assign24010_e18507 * ((locals.var_t7_dn10 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn10)))) / (assign24010_e18510 * assign24010_e18510)), ((((((2.0 * locals.var_t4_dn11) + (3.0 * locals.var_t5_dn11)) + (((4.0 * locals.var_t4_dn11) * locals.var_t5) + (assign24010_e18504 * locals.var_t5_dn11))) * assign24010_e18510) - (assign24010_e18507 * ((locals.var_t7_dn11 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn11)))) / (assign24010_e18510 * assign24010_e18510)), ((((((2.0 * locals.var_t4_dn14) + (3.0 * locals.var_t5_dn14)) + (((4.0 * locals.var_t4_dn14) * locals.var_t5) + (assign24010_e18504 * locals.var_t5_dn14))) * assign24010_e18510) - (assign24010_e18507 * ((locals.var_t7_dn14 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn14)))) / (assign24010_e18510 * assign24010_e18510)),)
    } else {
        (locals.var_vbscldvbs__blk440, locals.var_vbscldvbs__blk440_dn0, locals.var_vbscldvbs__blk440_dn2, locals.var_vbscldvbs__blk440_dn4, locals.var_vbscldvbs__blk440_dn5, locals.var_vbscldvbs__blk440_dn6, locals.var_vbscldvbs__blk440_dn7, locals.var_vbscldvbs__blk440_dn8, locals.var_vbscldvbs__blk440_dn9, locals.var_vbscldvbs__blk440_dn10, locals.var_vbscldvbs__blk440_dn11, locals.var_vbscldvbs__blk440_dn14,)
    }
};
        locals.var_vbscldvbs__blk440 = assign24010_e18513;
        locals.var_vbscldvbs__blk440_dn0 = assign24010_e18513_d_n0;
        locals.var_vbscldvbs__blk440_dn2 = assign24010_e18513_d_n2;
        locals.var_vbscldvbs__blk440_dn4 = assign24010_e18513_d_n4;
        locals.var_vbscldvbs__blk440_dn5 = assign24010_e18513_d_n5;
        locals.var_vbscldvbs__blk440_dn6 = assign24010_e18513_d_n6;
        locals.var_vbscldvbs__blk440_dn7 = assign24010_e18513_d_n7;
        locals.var_vbscldvbs__blk440_dn8 = assign24010_e18513_d_n8;
        locals.var_vbscldvbs__blk440_dn9 = assign24010_e18513_d_n9;
        locals.var_vbscldvbs__blk440_dn10 = assign24010_e18513_d_n10;
        locals.var_vbscldvbs__blk440_dn11 = assign24010_e18513_d_n11;
        locals.var_vbscldvbs__blk440_dn14 = assign24010_e18513_d_n14;
        locals.var_vbscldvbs__blk440_rv = 0.0;

        let (assign24020_e18520, assign24020_e18520_d_n0, assign24020_e18520_d_n2, assign24020_e18520_d_n4, assign24020_e18520_d_n5, assign24020_e18520_d_n6, assign24020_e18520_d_n7, assign24020_e18520_d_n8, assign24020_e18520_d_n9, assign24020_e18520_d_n10, assign24020_e18520_d_n11, assign24020_e18520_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard445 == 0.0)) {
        (locals.var_vbs, 0.0, 0.0, 0.0, 0.0, locals.var_vbs_dn6, 0.0, locals.var_vbs_dn8, locals.var_vbs_dn9, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbscl__blk439, locals.var_vbscl__blk439_dn0, locals.var_vbscl__blk439_dn2, locals.var_vbscl__blk439_dn4, locals.var_vbscl__blk439_dn5, locals.var_vbscl__blk439_dn6, locals.var_vbscl__blk439_dn7, locals.var_vbscl__blk439_dn8, locals.var_vbscl__blk439_dn9, locals.var_vbscl__blk439_dn10, locals.var_vbscl__blk439_dn11, locals.var_vbscl__blk439_dn14,)
    }
};
        locals.var_vbscl__blk439 = assign24020_e18520;
        locals.var_vbscl__blk439_dn0 = assign24020_e18520_d_n0;
        locals.var_vbscl__blk439_dn2 = assign24020_e18520_d_n2;
        locals.var_vbscl__blk439_dn4 = assign24020_e18520_d_n4;
        locals.var_vbscl__blk439_dn5 = assign24020_e18520_d_n5;
        locals.var_vbscl__blk439_dn6 = assign24020_e18520_d_n6;
        locals.var_vbscl__blk439_dn7 = assign24020_e18520_d_n7;
        locals.var_vbscl__blk439_dn8 = assign24020_e18520_d_n8;
        locals.var_vbscl__blk439_dn9 = assign24020_e18520_d_n9;
        locals.var_vbscl__blk439_dn10 = assign24020_e18520_d_n10;
        locals.var_vbscl__blk439_dn11 = assign24020_e18520_d_n11;
        locals.var_vbscl__blk439_dn14 = assign24020_e18520_d_n14;
        locals.var_vbscl__blk439_rv = 0.0;

        let (assign24030_e18527, assign24030_e18527_d_n0, assign24030_e18527_d_n2, assign24030_e18527_d_n4, assign24030_e18527_d_n5, assign24030_e18527_d_n6, assign24030_e18527_d_n7, assign24030_e18527_d_n8, assign24030_e18527_d_n9, assign24030_e18527_d_n10, assign24030_e18527_d_n11, assign24030_e18527_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard445 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbscldvbs__blk440, locals.var_vbscldvbs__blk440_dn0, locals.var_vbscldvbs__blk440_dn2, locals.var_vbscldvbs__blk440_dn4, locals.var_vbscldvbs__blk440_dn5, locals.var_vbscldvbs__blk440_dn6, locals.var_vbscldvbs__blk440_dn7, locals.var_vbscldvbs__blk440_dn8, locals.var_vbscldvbs__blk440_dn9, locals.var_vbscldvbs__blk440_dn10, locals.var_vbscldvbs__blk440_dn11, locals.var_vbscldvbs__blk440_dn14,)
    }
};
        locals.var_vbscldvbs__blk440 = assign24030_e18527;
        locals.var_vbscldvbs__blk440_dn0 = assign24030_e18527_d_n0;
        locals.var_vbscldvbs__blk440_dn2 = assign24030_e18527_d_n2;
        locals.var_vbscldvbs__blk440_dn4 = assign24030_e18527_d_n4;
        locals.var_vbscldvbs__blk440_dn5 = assign24030_e18527_d_n5;
        locals.var_vbscldvbs__blk440_dn6 = assign24030_e18527_d_n6;
        locals.var_vbscldvbs__blk440_dn7 = assign24030_e18527_d_n7;
        locals.var_vbscldvbs__blk440_dn8 = assign24030_e18527_d_n8;
        locals.var_vbscldvbs__blk440_dn9 = assign24030_e18527_d_n9;
        locals.var_vbscldvbs__blk440_dn10 = assign24030_e18527_d_n10;
        locals.var_vbscldvbs__blk440_dn11 = assign24030_e18527_d_n11;
        locals.var_vbscldvbs__blk440_dn14 = assign24030_e18527_d_n14;
        locals.var_vbscldvbs__blk440_rv = 0.0;

        let (assign24040_e18532, assign24040_e18532_d_n0, assign24040_e18532_d_n2, assign24040_e18532_d_n4, assign24040_e18532_d_n5, assign24040_e18532_d_n6, assign24040_e18532_d_n7, assign24040_e18532_d_n8, assign24040_e18532_d_n9, assign24040_e18532_d_n10, assign24040_e18532_d_n11, assign24040_e18532_d_n14,) = {
    if (p.p37 == 0.0) {
        (locals.var_vbs, 0.0, 0.0, 0.0, 0.0, locals.var_vbs_dn6, 0.0, locals.var_vbs_dn8, locals.var_vbs_dn9, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbscl__blk439, locals.var_vbscl__blk439_dn0, locals.var_vbscl__blk439_dn2, locals.var_vbscl__blk439_dn4, locals.var_vbscl__blk439_dn5, locals.var_vbscl__blk439_dn6, locals.var_vbscl__blk439_dn7, locals.var_vbscl__blk439_dn8, locals.var_vbscl__blk439_dn9, locals.var_vbscl__blk439_dn10, locals.var_vbscl__blk439_dn11, locals.var_vbscl__blk439_dn14,)
    }
};
        locals.var_vbscl__blk439 = assign24040_e18532;
        locals.var_vbscl__blk439_dn0 = assign24040_e18532_d_n0;
        locals.var_vbscl__blk439_dn2 = assign24040_e18532_d_n2;
        locals.var_vbscl__blk439_dn4 = assign24040_e18532_d_n4;
        locals.var_vbscl__blk439_dn5 = assign24040_e18532_d_n5;
        locals.var_vbscl__blk439_dn6 = assign24040_e18532_d_n6;
        locals.var_vbscl__blk439_dn7 = assign24040_e18532_d_n7;
        locals.var_vbscl__blk439_dn8 = assign24040_e18532_d_n8;
        locals.var_vbscl__blk439_dn9 = assign24040_e18532_d_n9;
        locals.var_vbscl__blk439_dn10 = assign24040_e18532_d_n10;
        locals.var_vbscl__blk439_dn11 = assign24040_e18532_d_n11;
        locals.var_vbscl__blk439_dn14 = assign24040_e18532_d_n14;
        locals.var_vbscl__blk439_rv = 0.0;

        let (assign24050_e18537, assign24050_e18537_d_n0, assign24050_e18537_d_n2, assign24050_e18537_d_n4, assign24050_e18537_d_n5, assign24050_e18537_d_n6, assign24050_e18537_d_n7, assign24050_e18537_d_n8, assign24050_e18537_d_n9, assign24050_e18537_d_n10, assign24050_e18537_d_n11, assign24050_e18537_d_n14,) = {
    if (p.p37 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbscldvbs__blk440, locals.var_vbscldvbs__blk440_dn0, locals.var_vbscldvbs__blk440_dn2, locals.var_vbscldvbs__blk440_dn4, locals.var_vbscldvbs__blk440_dn5, locals.var_vbscldvbs__blk440_dn6, locals.var_vbscldvbs__blk440_dn7, locals.var_vbscldvbs__blk440_dn8, locals.var_vbscldvbs__blk440_dn9, locals.var_vbscldvbs__blk440_dn10, locals.var_vbscldvbs__blk440_dn11, locals.var_vbscldvbs__blk440_dn14,)
    }
};
        locals.var_vbscldvbs__blk440 = assign24050_e18537;
        locals.var_vbscldvbs__blk440_dn0 = assign24050_e18537_d_n0;
        locals.var_vbscldvbs__blk440_dn2 = assign24050_e18537_d_n2;
        locals.var_vbscldvbs__blk440_dn4 = assign24050_e18537_d_n4;
        locals.var_vbscldvbs__blk440_dn5 = assign24050_e18537_d_n5;
        locals.var_vbscldvbs__blk440_dn6 = assign24050_e18537_d_n6;
        locals.var_vbscldvbs__blk440_dn7 = assign24050_e18537_d_n7;
        locals.var_vbscldvbs__blk440_dn8 = assign24050_e18537_d_n8;
        locals.var_vbscldvbs__blk440_dn9 = assign24050_e18537_d_n9;
        locals.var_vbscldvbs__blk440_dn10 = assign24050_e18537_d_n10;
        locals.var_vbscldvbs__blk440_dn11 = assign24050_e18537_d_n11;
        locals.var_vbscldvbs__blk440_dn14 = assign24050_e18537_d_n14;
        locals.var_vbscldvbs__blk440_rv = 0.0;

        let assign24060_e18540: f64 = (locals.var_vbscldvbs__blk440 * locals.var_vds);
        let assign24060_e18542: f64 = (assign24060_e18540 / 2.0);
        locals.var_t1 = assign24060_e18542;
        locals.var_t1_dn0 = (((locals.var_vbscldvbs__blk440_dn0 * locals.var_vds) + (locals.var_vbscldvbs__blk440 * locals.var_vds_dn0)) / 2.0);
        locals.var_t1_dn2 = (((locals.var_vbscldvbs__blk440_dn2 * locals.var_vds) + (locals.var_vbscldvbs__blk440 * locals.var_vds_dn2)) / 2.0);
        locals.var_t1_dn4 = (((locals.var_vbscldvbs__blk440_dn4 * locals.var_vds) + (locals.var_vbscldvbs__blk440 * locals.var_vds_dn4)) / 2.0);
        locals.var_t1_dn5 = (((locals.var_vbscldvbs__blk440_dn5 * locals.var_vds) + (locals.var_vbscldvbs__blk440 * locals.var_vds_dn5)) / 2.0);
        locals.var_t1_dn6 = (((locals.var_vbscldvbs__blk440_dn6 * locals.var_vds) + (locals.var_vbscldvbs__blk440 * locals.var_vds_dn6)) / 2.0);
        locals.var_t1_dn7 = (((locals.var_vbscldvbs__blk440_dn7 * locals.var_vds) + (locals.var_vbscldvbs__blk440 * locals.var_vds_dn7)) / 2.0);
        locals.var_t1_dn8 = (((locals.var_vbscldvbs__blk440_dn8 * locals.var_vds) + (locals.var_vbscldvbs__blk440 * locals.var_vds_dn8)) / 2.0);
        locals.var_t1_dn9 = (((locals.var_vbscldvbs__blk440_dn9 * locals.var_vds) + (locals.var_vbscldvbs__blk440 * locals.var_vds_dn9)) / 2.0);
        locals.var_t1_dn10 = (((locals.var_vbscldvbs__blk440_dn10 * locals.var_vds) + (locals.var_vbscldvbs__blk440 * locals.var_vds_dn10)) / 2.0);
        locals.var_t1_dn11 = (((locals.var_vbscldvbs__blk440_dn11 * locals.var_vds) + (locals.var_vbscldvbs__blk440 * locals.var_vds_dn11)) / 2.0);
        locals.var_t1_dn14 = (((locals.var_vbscldvbs__blk440_dn14 * locals.var_vds) + (locals.var_vbscldvbs__blk440 * locals.var_vds_dn14)) / 2.0);
        locals.var_t1_rv = 0.0;

        let assign24070_e18545: f64 = (2.0 * locals.var_t1);
        let assign24070_e18547: f64 = (assign24070_e18545 / p.p262);
        locals.var_tmf1 = assign24070_e18547;
        locals.var_tmf1_dn0 = ((2.0 * locals.var_t1_dn0) / p.p262);
        locals.var_tmf1_dn2 = ((2.0 * locals.var_t1_dn2) / p.p262);
        locals.var_tmf1_dn4 = ((2.0 * locals.var_t1_dn4) / p.p262);
        locals.var_tmf1_dn5 = ((2.0 * locals.var_t1_dn5) / p.p262);
        locals.var_tmf1_dn6 = ((2.0 * locals.var_t1_dn6) / p.p262);
        locals.var_tmf1_dn7 = ((2.0 * locals.var_t1_dn7) / p.p262);
        locals.var_tmf1_dn8 = ((2.0 * locals.var_t1_dn8) / p.p262);
        locals.var_tmf1_dn9 = ((2.0 * locals.var_t1_dn9) / p.p262);
        locals.var_tmf1_dn10 = ((2.0 * locals.var_t1_dn10) / p.p262);
        locals.var_tmf1_dn11 = ((2.0 * locals.var_t1_dn11) / p.p262);
        locals.var_tmf1_dn14 = ((2.0 * locals.var_t1_dn14) / p.p262);
        locals.var_tmf1_rv = 0.0;

        let assign24080_e18552: f64 = (1.0 / 2.0);
        let assign24080_e18556: f64 = (1.0 / 6.0);
        let assign24080_e18560: f64 = (1.0 / 24.0);
        let assign24080_e18564: f64 = (1.0 / 120.0);
        let assign24080_e18568: f64 = (1.0 / 720.0);
        let assign24080_e18572: f64 = (1.0 / 5040.0);
        let assign24080_e18573: f64 = (locals.var_tmf1 * assign24080_e18572);
        let assign24080_e18574: f64 = (assign24080_e18568 + assign24080_e18573);
        let assign24080_e18575: f64 = (locals.var_tmf1 * assign24080_e18574);
        let assign24080_e18576: f64 = (assign24080_e18564 + assign24080_e18575);
        let assign24080_e18577: f64 = (locals.var_tmf1 * assign24080_e18576);
        let assign24080_e18578: f64 = (assign24080_e18560 + assign24080_e18577);
        let assign24080_e18579: f64 = (locals.var_tmf1 * assign24080_e18578);
        let assign24080_e18580: f64 = (assign24080_e18556 + assign24080_e18579);
        let assign24080_e18581: f64 = (locals.var_tmf1 * assign24080_e18580);
        let assign24080_e18582: f64 = (assign24080_e18552 + assign24080_e18581);
        let assign24080_e18583: f64 = (locals.var_tmf1 * assign24080_e18582);
        let assign24080_e18584: f64 = (1.0 + assign24080_e18583);
        locals.var_tmf2 = assign24080_e18584;
        locals.var_tmf2_dn0 = ((locals.var_tmf1_dn0 * assign24080_e18582) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign24080_e18580) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign24080_e18578) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign24080_e18576) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign24080_e18574) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign24080_e18572)))))))))));
        locals.var_tmf2_dn2 = ((locals.var_tmf1_dn2 * assign24080_e18582) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign24080_e18580) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign24080_e18578) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign24080_e18576) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign24080_e18574) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign24080_e18572)))))))))));
        locals.var_tmf2_dn4 = ((locals.var_tmf1_dn4 * assign24080_e18582) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign24080_e18580) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign24080_e18578) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign24080_e18576) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign24080_e18574) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign24080_e18572)))))))))));
        locals.var_tmf2_dn5 = ((locals.var_tmf1_dn5 * assign24080_e18582) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign24080_e18580) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign24080_e18578) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign24080_e18576) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign24080_e18574) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign24080_e18572)))))))))));
        locals.var_tmf2_dn6 = ((locals.var_tmf1_dn6 * assign24080_e18582) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign24080_e18580) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign24080_e18578) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign24080_e18576) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign24080_e18574) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign24080_e18572)))))))))));
        locals.var_tmf2_dn7 = ((locals.var_tmf1_dn7 * assign24080_e18582) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign24080_e18580) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign24080_e18578) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign24080_e18576) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign24080_e18574) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign24080_e18572)))))))))));
        locals.var_tmf2_dn8 = ((locals.var_tmf1_dn8 * assign24080_e18582) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign24080_e18580) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign24080_e18578) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign24080_e18576) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign24080_e18574) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign24080_e18572)))))))))));
        locals.var_tmf2_dn9 = ((locals.var_tmf1_dn9 * assign24080_e18582) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign24080_e18580) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign24080_e18578) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign24080_e18576) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign24080_e18574) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign24080_e18572)))))))))));
        locals.var_tmf2_dn10 = ((locals.var_tmf1_dn10 * assign24080_e18582) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign24080_e18580) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign24080_e18578) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign24080_e18576) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign24080_e18574) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign24080_e18572)))))))))));
        locals.var_tmf2_dn11 = ((locals.var_tmf1_dn11 * assign24080_e18582) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign24080_e18580) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign24080_e18578) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign24080_e18576) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign24080_e18574) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign24080_e18572)))))))))));
        locals.var_tmf2_dn14 = ((locals.var_tmf1_dn14 * assign24080_e18582) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign24080_e18580) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign24080_e18578) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign24080_e18576) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign24080_e18574) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign24080_e18572)))))))))));
        locals.var_tmf2_rv = 0.0;

        let assign24090_e18587: f64 = (1.0 / 2.0);
        let assign24090_e18591: f64 = (1.0 / 3.0);
        let assign24090_e18595: f64 = (1.0 / 8.0);
        let assign24090_e18599: f64 = (1.0 / 30.0);
        let assign24090_e18603: f64 = (1.0 / 144.0);
        let assign24090_e18607: f64 = (1.0 / 840.0);
        let assign24090_e18608: f64 = (locals.var_tmf1 * assign24090_e18607);
        let assign24090_e18609: f64 = (assign24090_e18603 + assign24090_e18608);
        let assign24090_e18610: f64 = (locals.var_tmf1 * assign24090_e18609);
        let assign24090_e18611: f64 = (assign24090_e18599 + assign24090_e18610);
        let assign24090_e18612: f64 = (locals.var_tmf1 * assign24090_e18611);
        let assign24090_e18613: f64 = (assign24090_e18595 + assign24090_e18612);
        let assign24090_e18614: f64 = (locals.var_tmf1 * assign24090_e18613);
        let assign24090_e18615: f64 = (assign24090_e18591 + assign24090_e18614);
        let assign24090_e18616: f64 = (locals.var_tmf1 * assign24090_e18615);
        let assign24090_e18617: f64 = (assign24090_e18587 + assign24090_e18616);
        locals.var_tmf3 = assign24090_e18617;
        locals.var_tmf3_dn0 = ((locals.var_tmf1_dn0 * assign24090_e18615) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign24090_e18613) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign24090_e18611) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign24090_e18609) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign24090_e18607)))))))));
        locals.var_tmf3_dn2 = ((locals.var_tmf1_dn2 * assign24090_e18615) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign24090_e18613) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign24090_e18611) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign24090_e18609) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign24090_e18607)))))))));
        locals.var_tmf3_dn4 = ((locals.var_tmf1_dn4 * assign24090_e18615) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign24090_e18613) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign24090_e18611) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign24090_e18609) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign24090_e18607)))))))));
        locals.var_tmf3_dn5 = ((locals.var_tmf1_dn5 * assign24090_e18615) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign24090_e18613) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign24090_e18611) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign24090_e18609) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign24090_e18607)))))))));
        locals.var_tmf3_dn6 = ((locals.var_tmf1_dn6 * assign24090_e18615) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign24090_e18613) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign24090_e18611) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign24090_e18609) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign24090_e18607)))))))));
        locals.var_tmf3_dn7 = ((locals.var_tmf1_dn7 * assign24090_e18615) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign24090_e18613) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign24090_e18611) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign24090_e18609) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign24090_e18607)))))))));
        locals.var_tmf3_dn8 = ((locals.var_tmf1_dn8 * assign24090_e18615) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign24090_e18613) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign24090_e18611) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign24090_e18609) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign24090_e18607)))))))));
        locals.var_tmf3_dn9 = ((locals.var_tmf1_dn9 * assign24090_e18615) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign24090_e18613) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign24090_e18611) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign24090_e18609) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign24090_e18607)))))))));
        locals.var_tmf3_dn10 = ((locals.var_tmf1_dn10 * assign24090_e18615) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign24090_e18613) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign24090_e18611) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign24090_e18609) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign24090_e18607)))))))));
        locals.var_tmf3_dn11 = ((locals.var_tmf1_dn11 * assign24090_e18615) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign24090_e18613) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign24090_e18611) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign24090_e18609) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign24090_e18607)))))))));
        locals.var_tmf3_dn14 = ((locals.var_tmf1_dn14 * assign24090_e18615) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign24090_e18613) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign24090_e18611) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign24090_e18609) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign24090_e18607)))))))));
        locals.var_tmf3_rv = 0.0;

        let assign24100_e18620: f64 = (p.p262 / locals.var_tmf2);
        locals.var_vzadd__blk441 = assign24100_e18620;
        locals.var_vzadd__blk441_dn0 = (-((p.p262 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk441_dn2 = (-((p.p262 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk441_dn4 = (-((p.p262 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk441_dn5 = (-((p.p262 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk441_dn6 = (-((p.p262 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk441_dn7 = (-((p.p262 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk441_dn8 = (-((p.p262 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk441_dn9 = (-((p.p262 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk441_dn10 = (-((p.p262 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk441_dn11 = (-((p.p262 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk441_dn14 = (-((p.p262 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk441_rv = 0.0;

        let assign24110_e18622: f64 = (-2.0);
        let assign24110_e18624: f64 = (assign24110_e18622 * locals.var_tmf3);
        let assign24110_e18627: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign24110_e18628: f64 = (assign24110_e18624 / assign24110_e18627);
        locals.var_t2 = assign24110_e18628;
        locals.var_t2_dn0 = ((((assign24110_e18622 * locals.var_tmf3_dn0) * assign24110_e18627) - (assign24110_e18624 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign24110_e18627 * assign24110_e18627));
        locals.var_t2_dn2 = ((((assign24110_e18622 * locals.var_tmf3_dn2) * assign24110_e18627) - (assign24110_e18624 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign24110_e18627 * assign24110_e18627));
        locals.var_t2_dn4 = ((((assign24110_e18622 * locals.var_tmf3_dn4) * assign24110_e18627) - (assign24110_e18624 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign24110_e18627 * assign24110_e18627));
        locals.var_t2_dn5 = ((((assign24110_e18622 * locals.var_tmf3_dn5) * assign24110_e18627) - (assign24110_e18624 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign24110_e18627 * assign24110_e18627));
        locals.var_t2_dn6 = ((((assign24110_e18622 * locals.var_tmf3_dn6) * assign24110_e18627) - (assign24110_e18624 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign24110_e18627 * assign24110_e18627));
        locals.var_t2_dn7 = ((((assign24110_e18622 * locals.var_tmf3_dn7) * assign24110_e18627) - (assign24110_e18624 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign24110_e18627 * assign24110_e18627));
        locals.var_t2_dn8 = ((((assign24110_e18622 * locals.var_tmf3_dn8) * assign24110_e18627) - (assign24110_e18624 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign24110_e18627 * assign24110_e18627));
        locals.var_t2_dn9 = ((((assign24110_e18622 * locals.var_tmf3_dn9) * assign24110_e18627) - (assign24110_e18624 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign24110_e18627 * assign24110_e18627));
        locals.var_t2_dn10 = ((((assign24110_e18622 * locals.var_tmf3_dn10) * assign24110_e18627) - (assign24110_e18624 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign24110_e18627 * assign24110_e18627));
        locals.var_t2_dn11 = ((((assign24110_e18622 * locals.var_tmf3_dn11) * assign24110_e18627) - (assign24110_e18624 * ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)))) / (assign24110_e18627 * assign24110_e18627));
        locals.var_t2_dn14 = ((((assign24110_e18622 * locals.var_tmf3_dn14) * assign24110_e18627) - (assign24110_e18624 * ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)))) / (assign24110_e18627 * assign24110_e18627));
        locals.var_t2_rv = 0.0;

        let assign24120_e18631: f64 = if locals.var_vzadd__blk441 < 1e-12 { 1.0 } else { 0.0 };
        locals.var_guard446 = assign24120_e18631;
        locals.var_guard446_rv = 0.0;

        let (assign24130_e18635, assign24130_e18635_d_n0, assign24130_e18635_d_n2, assign24130_e18635_d_n4, assign24130_e18635_d_n5, assign24130_e18635_d_n6, assign24130_e18635_d_n7, assign24130_e18635_d_n8, assign24130_e18635_d_n9, assign24130_e18635_d_n10, assign24130_e18635_d_n11, assign24130_e18635_d_n14,) = {
    if (locals.var_guard446 != 0.0) {
        (1e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vzadd__blk441, locals.var_vzadd__blk441_dn0, locals.var_vzadd__blk441_dn2, locals.var_vzadd__blk441_dn4, locals.var_vzadd__blk441_dn5, locals.var_vzadd__blk441_dn6, locals.var_vzadd__blk441_dn7, locals.var_vzadd__blk441_dn8, locals.var_vzadd__blk441_dn9, locals.var_vzadd__blk441_dn10, locals.var_vzadd__blk441_dn11, locals.var_vzadd__blk441_dn14,)
    }
};
        locals.var_vzadd__blk441 = assign24130_e18635;
        locals.var_vzadd__blk441_dn0 = assign24130_e18635_d_n0;
        locals.var_vzadd__blk441_dn2 = assign24130_e18635_d_n2;
        locals.var_vzadd__blk441_dn4 = assign24130_e18635_d_n4;
        locals.var_vzadd__blk441_dn5 = assign24130_e18635_d_n5;
        locals.var_vzadd__blk441_dn6 = assign24130_e18635_d_n6;
        locals.var_vzadd__blk441_dn7 = assign24130_e18635_d_n7;
        locals.var_vzadd__blk441_dn8 = assign24130_e18635_d_n8;
        locals.var_vzadd__blk441_dn9 = assign24130_e18635_d_n9;
        locals.var_vzadd__blk441_dn10 = assign24130_e18635_d_n10;
        locals.var_vzadd__blk441_dn11 = assign24130_e18635_d_n11;
        locals.var_vzadd__blk441_dn14 = assign24130_e18635_d_n14;
        locals.var_vzadd__blk441_rv = 0.0;

        let assign24140_e18638: f64 = (locals.var_vbscl__blk439 + locals.var_vzadd__blk441);
        locals.var_vbsz__blk442 = assign24140_e18638;
        locals.var_vbsz__blk442_dn0 = (locals.var_vbscl__blk439_dn0 + locals.var_vzadd__blk441_dn0);
        locals.var_vbsz__blk442_dn2 = (locals.var_vbscl__blk439_dn2 + locals.var_vzadd__blk441_dn2);
        locals.var_vbsz__blk442_dn4 = (locals.var_vbscl__blk439_dn4 + locals.var_vzadd__blk441_dn4);
        locals.var_vbsz__blk442_dn5 = (locals.var_vbscl__blk439_dn5 + locals.var_vzadd__blk441_dn5);
        locals.var_vbsz__blk442_dn6 = (locals.var_vbscl__blk439_dn6 + locals.var_vzadd__blk441_dn6);
        locals.var_vbsz__blk442_dn7 = (locals.var_vbscl__blk439_dn7 + locals.var_vzadd__blk441_dn7);
        locals.var_vbsz__blk442_dn8 = (locals.var_vbscl__blk439_dn8 + locals.var_vzadd__blk441_dn8);
        locals.var_vbsz__blk442_dn9 = (locals.var_vbscl__blk439_dn9 + locals.var_vzadd__blk441_dn9);
        locals.var_vbsz__blk442_dn10 = (locals.var_vbscl__blk439_dn10 + locals.var_vzadd__blk441_dn10);
        locals.var_vbsz__blk442_dn11 = (locals.var_vbscl__blk439_dn11 + locals.var_vzadd__blk441_dn11);
        locals.var_vbsz__blk442_dn14 = (locals.var_vbscl__blk439_dn14 + locals.var_vzadd__blk441_dn14);
        locals.var_vbsz__blk442_rv = 0.0;

        let assign24150_e18642: f64 = (2.0 * locals.var_vzadd__blk441);
        let assign24150_e18643: f64 = (locals.var_vds + assign24150_e18642);
        locals.var_vdsz__blk443 = assign24150_e18643;
        locals.var_vdsz__blk443_dn0 = (locals.var_vds_dn0 + (2.0 * locals.var_vzadd__blk441_dn0));
        locals.var_vdsz__blk443_dn2 = (locals.var_vds_dn2 + (2.0 * locals.var_vzadd__blk441_dn2));
        locals.var_vdsz__blk443_dn4 = (locals.var_vds_dn4 + (2.0 * locals.var_vzadd__blk441_dn4));
        locals.var_vdsz__blk443_dn5 = (locals.var_vds_dn5 + (2.0 * locals.var_vzadd__blk441_dn5));
        locals.var_vdsz__blk443_dn6 = (locals.var_vds_dn6 + (2.0 * locals.var_vzadd__blk441_dn6));
        locals.var_vdsz__blk443_dn7 = (locals.var_vds_dn7 + (2.0 * locals.var_vzadd__blk441_dn7));
        locals.var_vdsz__blk443_dn8 = (locals.var_vds_dn8 + (2.0 * locals.var_vzadd__blk441_dn8));
        locals.var_vdsz__blk443_dn9 = (locals.var_vds_dn9 + (2.0 * locals.var_vzadd__blk441_dn9));
        locals.var_vdsz__blk443_dn10 = (locals.var_vds_dn10 + (2.0 * locals.var_vzadd__blk441_dn10));
        locals.var_vdsz__blk443_dn11 = (locals.var_vds_dn11 + (2.0 * locals.var_vzadd__blk441_dn11));
        locals.var_vdsz__blk443_dn14 = (locals.var_vds_dn14 + (2.0 * locals.var_vzadd__blk441_dn14));
        locals.var_vdsz__blk443_rv = 0.0;

        let assign24160_e18646: f64 = (locals.var_vgs + locals.var_vzadd__blk441);
        locals.var_vgsz__blk444 = assign24160_e18646;
        locals.var_vgsz__blk444_dn0 = locals.var_vzadd__blk441_dn0;
        locals.var_vgsz__blk444_dn2 = locals.var_vzadd__blk441_dn2;
        locals.var_vgsz__blk444_dn4 = locals.var_vzadd__blk441_dn4;
        locals.var_vgsz__blk444_dn5 = locals.var_vzadd__blk441_dn5;
        locals.var_vgsz__blk444_dn6 = (locals.var_vgs_dn6 + locals.var_vzadd__blk441_dn6);
        locals.var_vgsz__blk444_dn7 = (locals.var_vgs_dn7 + locals.var_vzadd__blk441_dn7);
        locals.var_vgsz__blk444_dn8 = (locals.var_vgs_dn8 + locals.var_vzadd__blk441_dn8);
        locals.var_vgsz__blk444_dn9 = locals.var_vzadd__blk441_dn9;
        locals.var_vgsz__blk444_dn10 = locals.var_vzadd__blk441_dn10;
        locals.var_vgsz__blk444_dn11 = locals.var_vzadd__blk441_dn11;
        locals.var_vgsz__blk444_dn14 = locals.var_vzadd__blk441_dn14;
        locals.var_vgsz__blk444_rv = 0.0;

        let assign24170_e18649: f64 = (locals.var_vgs - locals.var_vfb);
        let assign24170_e18651: f64 = (assign24170_e18649 + locals.var_dvth);
        let assign24170_e18653: f64 = (assign24170_e18651 - locals.var_dppg);
        locals.var_vgp = assign24170_e18653;
        locals.var_vgp_dn0 = (locals.var_dvth_dn0 - locals.var_dppg_dn0);
        locals.var_vgp_dn2 = (locals.var_dvth_dn2 - locals.var_dppg_dn2);
        locals.var_vgp_dn4 = (locals.var_dvth_dn4 - locals.var_dppg_dn4);
        locals.var_vgp_dn5 = (locals.var_dvth_dn5 - locals.var_dppg_dn5);
        locals.var_vgp_dn6 = ((locals.var_vgs_dn6 + locals.var_dvth_dn6) - locals.var_dppg_dn6);
        locals.var_vgp_dn7 = ((locals.var_vgs_dn7 + locals.var_dvth_dn7) - locals.var_dppg_dn7);
        locals.var_vgp_dn8 = ((locals.var_vgs_dn8 + locals.var_dvth_dn8) - locals.var_dppg_dn8);
        locals.var_vgp_dn9 = (locals.var_dvth_dn9 - locals.var_dppg_dn9);
        locals.var_vgp_dn10 = (locals.var_dvth_dn10 - locals.var_dppg_dn10);
        locals.var_vgp_dn11 = (locals.var_dvth_dn11 - locals.var_dppg_dn11);
        locals.var_vgp_dn14 = (locals.var_dvth_dn14 - locals.var_dppg_dn14);
        locals.var_vgp_rv = 0.0;

        let assign24180_e18656: f64 = (locals.var_vfb - locals.var_dvth);
        let assign24180_e18658: f64 = (assign24180_e18656 + locals.var_dppg);
        let assign24180_e18660: f64 = (assign24180_e18658 + locals.var_vbscl__blk439);
        locals.var_vgs_fb = assign24180_e18660;
        locals.var_vgs_fb_dn0 = (((-locals.var_dvth_dn0) + locals.var_dppg_dn0) + locals.var_vbscl__blk439_dn0);
        locals.var_vgs_fb_dn2 = (((-locals.var_dvth_dn2) + locals.var_dppg_dn2) + locals.var_vbscl__blk439_dn2);
        locals.var_vgs_fb_dn4 = (((-locals.var_dvth_dn4) + locals.var_dppg_dn4) + locals.var_vbscl__blk439_dn4);
        locals.var_vgs_fb_dn5 = (((-locals.var_dvth_dn5) + locals.var_dppg_dn5) + locals.var_vbscl__blk439_dn5);
        locals.var_vgs_fb_dn6 = (((-locals.var_dvth_dn6) + locals.var_dppg_dn6) + locals.var_vbscl__blk439_dn6);
        locals.var_vgs_fb_dn7 = (((-locals.var_dvth_dn7) + locals.var_dppg_dn7) + locals.var_vbscl__blk439_dn7);
        locals.var_vgs_fb_dn8 = (((-locals.var_dvth_dn8) + locals.var_dppg_dn8) + locals.var_vbscl__blk439_dn8);
        locals.var_vgs_fb_dn9 = (((-locals.var_dvth_dn9) + locals.var_dppg_dn9) + locals.var_vbscl__blk439_dn9);
        locals.var_vgs_fb_dn10 = (((-locals.var_dvth_dn10) + locals.var_dppg_dn10) + locals.var_vbscl__blk439_dn10);
        locals.var_vgs_fb_dn11 = (((-locals.var_dvth_dn11) + locals.var_dppg_dn11) + locals.var_vbscl__blk439_dn11);
        locals.var_vgs_fb_dn14 = (((-locals.var_dvth_dn14) + locals.var_dppg_dn14) + locals.var_vbscl__blk439_dn14);
        locals.var_vgs_fb_rv = 0.0;

        let assign24190_e18663: f64 = if locals.var_uc_codep != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard447 = assign24190_e18663;
        locals.var_guard447_rv = 0.0;

        let assign24200_e18666: f64 = if p.p42 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard448 = assign24200_e18666;
        locals.var_guard448_rv = 0.0;

        let assign24210_e18669: f64 = if p.p42 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard449 = assign24210_e18669;
        locals.var_guard449_rv = 0.0;

        let assign24220_e18672: f64 = if p.p42 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard450 = assign24220_e18672;
        locals.var_guard450_rv = 0.0;

        let (assign24230_e18678, assign24230_e18678_d_n0, assign24230_e18678_d_n2, assign24230_e18678_d_n4, assign24230_e18678_d_n5, assign24230_e18678_d_n6, assign24230_e18678_d_n7, assign24230_e18678_d_n8, assign24230_e18678_d_n9, assign24230_e18678_d_n10, assign24230_e18678_d_n11, assign24230_e18678_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        (locals.var_vbipn, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn4, locals.var_vbipn_dn5, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn8, locals.var_vbipn_dn9, locals.var_vbipn_dn10, locals.var_vbipn_dn11, locals.var_vbipn_dn14,)
    } else {
        (locals.var_vbi_dep, locals.var_vbi_dep_dn0, locals.var_vbi_dep_dn2, locals.var_vbi_dep_dn4, locals.var_vbi_dep_dn5, locals.var_vbi_dep_dn6, locals.var_vbi_dep_dn7, locals.var_vbi_dep_dn8, locals.var_vbi_dep_dn9, locals.var_vbi_dep_dn10, locals.var_vbi_dep_dn11, locals.var_vbi_dep_dn14,)
    }
};
        locals.var_vbi_dep = assign24230_e18678;
        locals.var_vbi_dep_dn0 = assign24230_e18678_d_n0;
        locals.var_vbi_dep_dn2 = assign24230_e18678_d_n2;
        locals.var_vbi_dep_dn4 = assign24230_e18678_d_n4;
        locals.var_vbi_dep_dn5 = assign24230_e18678_d_n5;
        locals.var_vbi_dep_dn6 = assign24230_e18678_d_n6;
        locals.var_vbi_dep_dn7 = assign24230_e18678_d_n7;
        locals.var_vbi_dep_dn8 = assign24230_e18678_d_n8;
        locals.var_vbi_dep_dn9 = assign24230_e18678_d_n9;
        locals.var_vbi_dep_dn10 = assign24230_e18678_d_n10;
        locals.var_vbi_dep_dn11 = assign24230_e18678_d_n11;
        locals.var_vbi_dep_dn14 = assign24230_e18678_d_n14;
        locals.var_vbi_dep_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_67(
        locals: &mut StampLocals,
    ) {
        let (assign24240_e18686, assign24240_e18686_d_n0, assign24240_e18686_d_n2, assign24240_e18686_d_n4, assign24240_e18686_d_n5, assign24240_e18686_d_n6, assign24240_e18686_d_n7, assign24240_e18686_d_n8, assign24240_e18686_d_n9, assign24240_e18686_d_n10, assign24240_e18686_d_n11, assign24240_e18686_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign24240_e18684: f64 = (1.6021918e-19 * locals.var_uc_ndepm);
        (assign24240_e18684, (1.6021918e-19 * locals.var_uc_ndepm_dn0), (1.6021918e-19 * locals.var_uc_ndepm_dn2), (1.6021918e-19 * locals.var_uc_ndepm_dn4), (1.6021918e-19 * locals.var_uc_ndepm_dn5), (1.6021918e-19 * locals.var_uc_ndepm_dn6), (1.6021918e-19 * locals.var_uc_ndepm_dn7), (1.6021918e-19 * locals.var_uc_ndepm_dn8), (1.6021918e-19 * locals.var_uc_ndepm_dn9), (1.6021918e-19 * locals.var_uc_ndepm_dn10), (1.6021918e-19 * locals.var_uc_ndepm_dn11), (1.6021918e-19 * locals.var_uc_ndepm_dn14),)
    } else {
        (locals.var_q_ndepm, locals.var_q_ndepm_dn0, locals.var_q_ndepm_dn2, locals.var_q_ndepm_dn4, locals.var_q_ndepm_dn5, locals.var_q_ndepm_dn6, locals.var_q_ndepm_dn7, locals.var_q_ndepm_dn8, locals.var_q_ndepm_dn9, locals.var_q_ndepm_dn10, locals.var_q_ndepm_dn11, locals.var_q_ndepm_dn14,)
    }
};
        locals.var_q_ndepm = assign24240_e18686;
        locals.var_q_ndepm_dn0 = assign24240_e18686_d_n0;
        locals.var_q_ndepm_dn2 = assign24240_e18686_d_n2;
        locals.var_q_ndepm_dn4 = assign24240_e18686_d_n4;
        locals.var_q_ndepm_dn5 = assign24240_e18686_d_n5;
        locals.var_q_ndepm_dn6 = assign24240_e18686_d_n6;
        locals.var_q_ndepm_dn7 = assign24240_e18686_d_n7;
        locals.var_q_ndepm_dn8 = assign24240_e18686_d_n8;
        locals.var_q_ndepm_dn9 = assign24240_e18686_d_n9;
        locals.var_q_ndepm_dn10 = assign24240_e18686_d_n10;
        locals.var_q_ndepm_dn11 = assign24240_e18686_d_n11;
        locals.var_q_ndepm_dn14 = assign24240_e18686_d_n14;
        locals.var_q_ndepm_rv = 0.0;

        let (assign24250_e18694, assign24250_e18694_d_n0, assign24250_e18694_d_n2, assign24250_e18694_d_n4, assign24250_e18694_d_n5, assign24250_e18694_d_n6, assign24250_e18694_d_n7, assign24250_e18694_d_n8, assign24250_e18694_d_n9, assign24250_e18694_d_n10, assign24250_e18694_d_n11, assign24250_e18694_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign24250_e18692: f64 = (locals.var_uc_ndepm * locals.var_uc_ndepm);
        (assign24250_e18692, ((locals.var_uc_ndepm_dn0 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn0)), ((locals.var_uc_ndepm_dn2 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn2)), ((locals.var_uc_ndepm_dn4 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn4)), ((locals.var_uc_ndepm_dn5 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn5)), ((locals.var_uc_ndepm_dn6 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn6)), ((locals.var_uc_ndepm_dn7 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn7)), ((locals.var_uc_ndepm_dn8 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn8)), ((locals.var_uc_ndepm_dn9 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn9)), ((locals.var_uc_ndepm_dn10 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn10)), ((locals.var_uc_ndepm_dn11 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn11)), ((locals.var_uc_ndepm_dn14 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn14)),)
    } else {
        (locals.var_ndepm2, locals.var_ndepm2_dn0, locals.var_ndepm2_dn2, locals.var_ndepm2_dn4, locals.var_ndepm2_dn5, locals.var_ndepm2_dn6, locals.var_ndepm2_dn7, locals.var_ndepm2_dn8, locals.var_ndepm2_dn9, locals.var_ndepm2_dn10, locals.var_ndepm2_dn11, locals.var_ndepm2_dn14,)
    }
};
        locals.var_ndepm2 = assign24250_e18694;
        locals.var_ndepm2_dn0 = assign24250_e18694_d_n0;
        locals.var_ndepm2_dn2 = assign24250_e18694_d_n2;
        locals.var_ndepm2_dn4 = assign24250_e18694_d_n4;
        locals.var_ndepm2_dn5 = assign24250_e18694_d_n5;
        locals.var_ndepm2_dn6 = assign24250_e18694_d_n6;
        locals.var_ndepm2_dn7 = assign24250_e18694_d_n7;
        locals.var_ndepm2_dn8 = assign24250_e18694_d_n8;
        locals.var_ndepm2_dn9 = assign24250_e18694_d_n9;
        locals.var_ndepm2_dn10 = assign24250_e18694_d_n10;
        locals.var_ndepm2_dn11 = assign24250_e18694_d_n11;
        locals.var_ndepm2_dn14 = assign24250_e18694_d_n14;
        locals.var_ndepm2_rv = 0.0;

        let (assign24260_e18704, assign24260_e18704_d_n0, assign24260_e18704_d_n2, assign24260_e18704_d_n4, assign24260_e18704_d_n5, assign24260_e18704_d_n6, assign24260_e18704_d_n7, assign24260_e18704_d_n8, assign24260_e18704_d_n9, assign24260_e18704_d_n10, assign24260_e18704_d_n11, assign24260_e18704_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign24260_e18700: f64 = (1.6021918e-19 * locals.var_uc_ndepm);
        let assign24260_e18702: f64 = (assign24260_e18700 * 1.034943e-10);
        (assign24260_e18702, ((1.6021918e-19 * locals.var_uc_ndepm_dn0) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn2) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn4) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn5) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn6) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn7) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn8) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn9) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn10) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn11) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn14) * 1.034943e-10),)
    } else {
        (locals.var_q_ndepm_esi, locals.var_q_ndepm_esi_dn0, locals.var_q_ndepm_esi_dn2, locals.var_q_ndepm_esi_dn4, locals.var_q_ndepm_esi_dn5, locals.var_q_ndepm_esi_dn6, locals.var_q_ndepm_esi_dn7, locals.var_q_ndepm_esi_dn8, locals.var_q_ndepm_esi_dn9, locals.var_q_ndepm_esi_dn10, locals.var_q_ndepm_esi_dn11, locals.var_q_ndepm_esi_dn14,)
    }
};
        locals.var_q_ndepm_esi = assign24260_e18704;
        locals.var_q_ndepm_esi_dn0 = assign24260_e18704_d_n0;
        locals.var_q_ndepm_esi_dn2 = assign24260_e18704_d_n2;
        locals.var_q_ndepm_esi_dn4 = assign24260_e18704_d_n4;
        locals.var_q_ndepm_esi_dn5 = assign24260_e18704_d_n5;
        locals.var_q_ndepm_esi_dn6 = assign24260_e18704_d_n6;
        locals.var_q_ndepm_esi_dn7 = assign24260_e18704_d_n7;
        locals.var_q_ndepm_esi_dn8 = assign24260_e18704_d_n8;
        locals.var_q_ndepm_esi_dn9 = assign24260_e18704_d_n9;
        locals.var_q_ndepm_esi_dn10 = assign24260_e18704_d_n10;
        locals.var_q_ndepm_esi_dn11 = assign24260_e18704_d_n11;
        locals.var_q_ndepm_esi_dn14 = assign24260_e18704_d_n14;
        locals.var_q_ndepm_esi_rv = 0.0;

        let (assign24270_e18712, assign24270_e18712_d_n0, assign24270_e18712_d_n2, assign24270_e18712_d_n4, assign24270_e18712_d_n5, assign24270_e18712_d_n6, assign24270_e18712_d_n7, assign24270_e18712_d_n8, assign24270_e18712_d_n9, assign24270_e18712_d_n10, assign24270_e18712_d_n11, assign24270_e18712_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign24270_e18710: f64 = (1.6021918e-19 * locals.var_ef_nsubc);
        (assign24270_e18710, (1.6021918e-19 * locals.var_ef_nsubc_dn0), (1.6021918e-19 * locals.var_ef_nsubc_dn2), (1.6021918e-19 * locals.var_ef_nsubc_dn4), (1.6021918e-19 * locals.var_ef_nsubc_dn5), (1.6021918e-19 * locals.var_ef_nsubc_dn6), (1.6021918e-19 * locals.var_ef_nsubc_dn7), (1.6021918e-19 * locals.var_ef_nsubc_dn8), (1.6021918e-19 * locals.var_ef_nsubc_dn9), (1.6021918e-19 * locals.var_ef_nsubc_dn10), (1.6021918e-19 * locals.var_ef_nsubc_dn11), (1.6021918e-19 * locals.var_ef_nsubc_dn14),)
    } else {
        (locals.var_q_nsub__blk548, locals.var_q_nsub__blk548_dn0, locals.var_q_nsub__blk548_dn2, locals.var_q_nsub__blk548_dn4, locals.var_q_nsub__blk548_dn5, locals.var_q_nsub__blk548_dn6, locals.var_q_nsub__blk548_dn7, locals.var_q_nsub__blk548_dn8, locals.var_q_nsub__blk548_dn9, locals.var_q_nsub__blk548_dn10, locals.var_q_nsub__blk548_dn11, locals.var_q_nsub__blk548_dn14,)
    }
};
        locals.var_q_nsub__blk548 = assign24270_e18712;
        locals.var_q_nsub__blk548_dn0 = assign24270_e18712_d_n0;
        locals.var_q_nsub__blk548_dn2 = assign24270_e18712_d_n2;
        locals.var_q_nsub__blk548_dn4 = assign24270_e18712_d_n4;
        locals.var_q_nsub__blk548_dn5 = assign24270_e18712_d_n5;
        locals.var_q_nsub__blk548_dn6 = assign24270_e18712_d_n6;
        locals.var_q_nsub__blk548_dn7 = assign24270_e18712_d_n7;
        locals.var_q_nsub__blk548_dn8 = assign24270_e18712_d_n8;
        locals.var_q_nsub__blk548_dn9 = assign24270_e18712_d_n9;
        locals.var_q_nsub__blk548_dn10 = assign24270_e18712_d_n10;
        locals.var_q_nsub__blk548_dn11 = assign24270_e18712_d_n11;
        locals.var_q_nsub__blk548_dn14 = assign24270_e18712_d_n14;
        locals.var_q_nsub__blk548_rv = 0.0;

        let (assign24280_e18720,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign24280_e18718: f64 = (1.6021918e-19 * 1.6021918e-19);
        (assign24280_e18718,)
    } else {
        (locals.var_c_qe2,)
    }
};
        locals.var_c_qe2 = assign24280_e18720;
        locals.var_c_qe2_rv = 0.0;

        let (assign24290_e18728,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign24290_e18726: f64 = (1.034943e-10 * 1.034943e-10);
        (assign24290_e18726,)
    } else {
        (locals.var_c_esi2,)
    }
};
        locals.var_c_esi2 = assign24290_e18728;
        locals.var_c_esi2_rv = 0.0;

        let (assign24300_e18736, assign24300_e18736_d_n0, assign24300_e18736_d_n2, assign24300_e18736_d_n4, assign24300_e18736_d_n5, assign24300_e18736_d_n6, assign24300_e18736_d_n7, assign24300_e18736_d_n8, assign24300_e18736_d_n9, assign24300_e18736_d_n10, assign24300_e18736_d_n11, assign24300_e18736_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign24300_e18734: f64 = (locals.var_uc_depthn * locals.var_uc_depthn);
        (assign24300_e18734, ((locals.var_uc_depthn_dn0 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn0)), ((locals.var_uc_depthn_dn2 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn2)), ((locals.var_uc_depthn_dn4 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn4)), ((locals.var_uc_depthn_dn5 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn5)), ((locals.var_uc_depthn_dn6 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn6)), ((locals.var_uc_depthn_dn7 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn7)), ((locals.var_uc_depthn_dn8 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn8)), ((locals.var_uc_depthn_dn9 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn9)), ((locals.var_uc_depthn_dn10 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn10)), ((locals.var_uc_depthn_dn11 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn11)), ((locals.var_uc_depthn_dn14 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn14)),)
    } else {
        (locals.var_tn2, locals.var_tn2_dn0, locals.var_tn2_dn2, locals.var_tn2_dn4, locals.var_tn2_dn5, locals.var_tn2_dn6, locals.var_tn2_dn7, locals.var_tn2_dn8, locals.var_tn2_dn9, locals.var_tn2_dn10, locals.var_tn2_dn11, locals.var_tn2_dn14,)
    }
};
        locals.var_tn2 = assign24300_e18736;
        locals.var_tn2_dn0 = assign24300_e18736_d_n0;
        locals.var_tn2_dn2 = assign24300_e18736_d_n2;
        locals.var_tn2_dn4 = assign24300_e18736_d_n4;
        locals.var_tn2_dn5 = assign24300_e18736_d_n5;
        locals.var_tn2_dn6 = assign24300_e18736_d_n6;
        locals.var_tn2_dn7 = assign24300_e18736_d_n7;
        locals.var_tn2_dn8 = assign24300_e18736_d_n8;
        locals.var_tn2_dn9 = assign24300_e18736_d_n9;
        locals.var_tn2_dn10 = assign24300_e18736_d_n10;
        locals.var_tn2_dn11 = assign24300_e18736_d_n11;
        locals.var_tn2_dn14 = assign24300_e18736_d_n14;
        locals.var_tn2_rv = 0.0;

        let (assign24310_e18746, assign24310_e18746_d_n0, assign24310_e18746_d_n2, assign24310_e18746_d_n4, assign24310_e18746_d_n5, assign24310_e18746_d_n6, assign24310_e18746_d_n7, assign24310_e18746_d_n8, assign24310_e18746_d_n9, assign24310_e18746_d_n10, assign24310_e18746_d_n11, assign24310_e18746_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign24310_e18742: f64 = (2.0 * 1.034943e-10);
        let assign24310_e18744: f64 = (assign24310_e18742 / locals.var_q_ndepm);
        (assign24310_e18744, (-((assign24310_e18742 * locals.var_q_ndepm_dn0) / (locals.var_q_ndepm * locals.var_q_ndepm))), (-((assign24310_e18742 * locals.var_q_ndepm_dn2) / (locals.var_q_ndepm * locals.var_q_ndepm))), (-((assign24310_e18742 * locals.var_q_ndepm_dn4) / (locals.var_q_ndepm * locals.var_q_ndepm))), (-((assign24310_e18742 * locals.var_q_ndepm_dn5) / (locals.var_q_ndepm * locals.var_q_ndepm))), (-((assign24310_e18742 * locals.var_q_ndepm_dn6) / (locals.var_q_ndepm * locals.var_q_ndepm))), (-((assign24310_e18742 * locals.var_q_ndepm_dn7) / (locals.var_q_ndepm * locals.var_q_ndepm))), (-((assign24310_e18742 * locals.var_q_ndepm_dn8) / (locals.var_q_ndepm * locals.var_q_ndepm))), (-((assign24310_e18742 * locals.var_q_ndepm_dn9) / (locals.var_q_ndepm * locals.var_q_ndepm))), (-((assign24310_e18742 * locals.var_q_ndepm_dn10) / (locals.var_q_ndepm * locals.var_q_ndepm))), (-((assign24310_e18742 * locals.var_q_ndepm_dn11) / (locals.var_q_ndepm * locals.var_q_ndepm))), (-((assign24310_e18742 * locals.var_q_ndepm_dn14) / (locals.var_q_ndepm * locals.var_q_ndepm))),)
    } else {
        (locals.var_c_2esipq_ndepm, locals.var_c_2esipq_ndepm_dn0, locals.var_c_2esipq_ndepm_dn2, locals.var_c_2esipq_ndepm_dn4, locals.var_c_2esipq_ndepm_dn5, locals.var_c_2esipq_ndepm_dn6, locals.var_c_2esipq_ndepm_dn7, locals.var_c_2esipq_ndepm_dn8, locals.var_c_2esipq_ndepm_dn9, locals.var_c_2esipq_ndepm_dn10, locals.var_c_2esipq_ndepm_dn11, locals.var_c_2esipq_ndepm_dn14,)
    }
};
        locals.var_c_2esipq_ndepm = assign24310_e18746;
        locals.var_c_2esipq_ndepm_dn0 = assign24310_e18746_d_n0;
        locals.var_c_2esipq_ndepm_dn2 = assign24310_e18746_d_n2;
        locals.var_c_2esipq_ndepm_dn4 = assign24310_e18746_d_n4;
        locals.var_c_2esipq_ndepm_dn5 = assign24310_e18746_d_n5;
        locals.var_c_2esipq_ndepm_dn6 = assign24310_e18746_d_n6;
        locals.var_c_2esipq_ndepm_dn7 = assign24310_e18746_d_n7;
        locals.var_c_2esipq_ndepm_dn8 = assign24310_e18746_d_n8;
        locals.var_c_2esipq_ndepm_dn9 = assign24310_e18746_d_n9;
        locals.var_c_2esipq_ndepm_dn10 = assign24310_e18746_d_n10;
        locals.var_c_2esipq_ndepm_dn11 = assign24310_e18746_d_n11;
        locals.var_c_2esipq_ndepm_dn14 = assign24310_e18746_d_n14;
        locals.var_c_2esipq_ndepm_rv = 0.0;

        let (assign24320_e18756, assign24320_e18756_d_n0, assign24320_e18756_d_n2, assign24320_e18756_d_n4, assign24320_e18756_d_n5, assign24320_e18756_d_n6, assign24320_e18756_d_n7, assign24320_e18756_d_n8, assign24320_e18756_d_n9, assign24320_e18756_d_n10, assign24320_e18756_d_n11, assign24320_e18756_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign24320_e18753: f64 = (2.0 * 1.034943e-10);
        let assign24320_e18754: f64 = (locals.var_q_ndepm / assign24320_e18753);
        (assign24320_e18754, (locals.var_q_ndepm_dn0 / assign24320_e18753), (locals.var_q_ndepm_dn2 / assign24320_e18753), (locals.var_q_ndepm_dn4 / assign24320_e18753), (locals.var_q_ndepm_dn5 / assign24320_e18753), (locals.var_q_ndepm_dn6 / assign24320_e18753), (locals.var_q_ndepm_dn7 / assign24320_e18753), (locals.var_q_ndepm_dn8 / assign24320_e18753), (locals.var_q_ndepm_dn9 / assign24320_e18753), (locals.var_q_ndepm_dn10 / assign24320_e18753), (locals.var_q_ndepm_dn11 / assign24320_e18753), (locals.var_q_ndepm_dn14 / assign24320_e18753),)
    } else {
        (locals.var_c_2esipq_ndepm_inv, locals.var_c_2esipq_ndepm_inv_dn0, locals.var_c_2esipq_ndepm_inv_dn2, locals.var_c_2esipq_ndepm_inv_dn4, locals.var_c_2esipq_ndepm_inv_dn5, locals.var_c_2esipq_ndepm_inv_dn6, locals.var_c_2esipq_ndepm_inv_dn7, locals.var_c_2esipq_ndepm_inv_dn8, locals.var_c_2esipq_ndepm_inv_dn9, locals.var_c_2esipq_ndepm_inv_dn10, locals.var_c_2esipq_ndepm_inv_dn11, locals.var_c_2esipq_ndepm_inv_dn14,)
    }
};
        locals.var_c_2esipq_ndepm_inv = assign24320_e18756;
        locals.var_c_2esipq_ndepm_inv_dn0 = assign24320_e18756_d_n0;
        locals.var_c_2esipq_ndepm_inv_dn2 = assign24320_e18756_d_n2;
        locals.var_c_2esipq_ndepm_inv_dn4 = assign24320_e18756_d_n4;
        locals.var_c_2esipq_ndepm_inv_dn5 = assign24320_e18756_d_n5;
        locals.var_c_2esipq_ndepm_inv_dn6 = assign24320_e18756_d_n6;
        locals.var_c_2esipq_ndepm_inv_dn7 = assign24320_e18756_d_n7;
        locals.var_c_2esipq_ndepm_inv_dn8 = assign24320_e18756_d_n8;
        locals.var_c_2esipq_ndepm_inv_dn9 = assign24320_e18756_d_n9;
        locals.var_c_2esipq_ndepm_inv_dn10 = assign24320_e18756_d_n10;
        locals.var_c_2esipq_ndepm_inv_dn11 = assign24320_e18756_d_n11;
        locals.var_c_2esipq_ndepm_inv_dn14 = assign24320_e18756_d_n14;
        locals.var_c_2esipq_ndepm_inv_rv = 0.0;

        let (assign24330_e18766, assign24330_e18766_d_n0, assign24330_e18766_d_n2, assign24330_e18766_d_n4, assign24330_e18766_d_n5, assign24330_e18766_d_n6, assign24330_e18766_d_n7, assign24330_e18766_d_n8, assign24330_e18766_d_n9, assign24330_e18766_d_n10, assign24330_e18766_d_n11, assign24330_e18766_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign24330_e18762: f64 = (2.0 * 1.034943e-10);
        let assign24330_e18764: f64 = (assign24330_e18762 * locals.var_q_ndepm);
        (assign24330_e18764, (assign24330_e18762 * locals.var_q_ndepm_dn0), (assign24330_e18762 * locals.var_q_ndepm_dn2), (assign24330_e18762 * locals.var_q_ndepm_dn4), (assign24330_e18762 * locals.var_q_ndepm_dn5), (assign24330_e18762 * locals.var_q_ndepm_dn6), (assign24330_e18762 * locals.var_q_ndepm_dn7), (assign24330_e18762 * locals.var_q_ndepm_dn8), (assign24330_e18762 * locals.var_q_ndepm_dn9), (assign24330_e18762 * locals.var_q_ndepm_dn10), (assign24330_e18762 * locals.var_q_ndepm_dn11), (assign24330_e18762 * locals.var_q_ndepm_dn14),)
    } else {
        (locals.var_c_2esi_q_ndepm, locals.var_c_2esi_q_ndepm_dn0, locals.var_c_2esi_q_ndepm_dn2, locals.var_c_2esi_q_ndepm_dn4, locals.var_c_2esi_q_ndepm_dn5, locals.var_c_2esi_q_ndepm_dn6, locals.var_c_2esi_q_ndepm_dn7, locals.var_c_2esi_q_ndepm_dn8, locals.var_c_2esi_q_ndepm_dn9, locals.var_c_2esi_q_ndepm_dn10, locals.var_c_2esi_q_ndepm_dn11, locals.var_c_2esi_q_ndepm_dn14,)
    }
};
        locals.var_c_2esi_q_ndepm = assign24330_e18766;
        locals.var_c_2esi_q_ndepm_dn0 = assign24330_e18766_d_n0;
        locals.var_c_2esi_q_ndepm_dn2 = assign24330_e18766_d_n2;
        locals.var_c_2esi_q_ndepm_dn4 = assign24330_e18766_d_n4;
        locals.var_c_2esi_q_ndepm_dn5 = assign24330_e18766_d_n5;
        locals.var_c_2esi_q_ndepm_dn6 = assign24330_e18766_d_n6;
        locals.var_c_2esi_q_ndepm_dn7 = assign24330_e18766_d_n7;
        locals.var_c_2esi_q_ndepm_dn8 = assign24330_e18766_d_n8;
        locals.var_c_2esi_q_ndepm_dn9 = assign24330_e18766_d_n9;
        locals.var_c_2esi_q_ndepm_dn10 = assign24330_e18766_d_n10;
        locals.var_c_2esi_q_ndepm_dn11 = assign24330_e18766_d_n11;
        locals.var_c_2esi_q_ndepm_dn14 = assign24330_e18766_d_n14;
        locals.var_c_2esi_q_ndepm_rv = 0.0;

        let (assign24340_e18776, assign24340_e18776_d_n0, assign24340_e18776_d_n2, assign24340_e18776_d_n4, assign24340_e18776_d_n5, assign24340_e18776_d_n6, assign24340_e18776_d_n7, assign24340_e18776_d_n8, assign24340_e18776_d_n9, assign24340_e18776_d_n10, assign24340_e18776_d_n11, assign24340_e18776_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign24340_e18772: f64 = (2.0 * 1.034943e-10);
        let assign24340_e18774: f64 = (assign24340_e18772 / locals.var_q_nsub__blk548);
        (assign24340_e18774, (-((assign24340_e18772 * locals.var_q_nsub__blk548_dn0) / (locals.var_q_nsub__blk548 * locals.var_q_nsub__blk548))), (-((assign24340_e18772 * locals.var_q_nsub__blk548_dn2) / (locals.var_q_nsub__blk548 * locals.var_q_nsub__blk548))), (-((assign24340_e18772 * locals.var_q_nsub__blk548_dn4) / (locals.var_q_nsub__blk548 * locals.var_q_nsub__blk548))), (-((assign24340_e18772 * locals.var_q_nsub__blk548_dn5) / (locals.var_q_nsub__blk548 * locals.var_q_nsub__blk548))), (-((assign24340_e18772 * locals.var_q_nsub__blk548_dn6) / (locals.var_q_nsub__blk548 * locals.var_q_nsub__blk548))), (-((assign24340_e18772 * locals.var_q_nsub__blk548_dn7) / (locals.var_q_nsub__blk548 * locals.var_q_nsub__blk548))), (-((assign24340_e18772 * locals.var_q_nsub__blk548_dn8) / (locals.var_q_nsub__blk548 * locals.var_q_nsub__blk548))), (-((assign24340_e18772 * locals.var_q_nsub__blk548_dn9) / (locals.var_q_nsub__blk548 * locals.var_q_nsub__blk548))), (-((assign24340_e18772 * locals.var_q_nsub__blk548_dn10) / (locals.var_q_nsub__blk548 * locals.var_q_nsub__blk548))), (-((assign24340_e18772 * locals.var_q_nsub__blk548_dn11) / (locals.var_q_nsub__blk548 * locals.var_q_nsub__blk548))), (-((assign24340_e18772 * locals.var_q_nsub__blk548_dn14) / (locals.var_q_nsub__blk548 * locals.var_q_nsub__blk548))),)
    } else {
        (locals.var_c_2esipq_nsub, locals.var_c_2esipq_nsub_dn0, locals.var_c_2esipq_nsub_dn2, locals.var_c_2esipq_nsub_dn4, locals.var_c_2esipq_nsub_dn5, locals.var_c_2esipq_nsub_dn6, locals.var_c_2esipq_nsub_dn7, locals.var_c_2esipq_nsub_dn8, locals.var_c_2esipq_nsub_dn9, locals.var_c_2esipq_nsub_dn10, locals.var_c_2esipq_nsub_dn11, locals.var_c_2esipq_nsub_dn14,)
    }
};
        locals.var_c_2esipq_nsub = assign24340_e18776;
        locals.var_c_2esipq_nsub_dn0 = assign24340_e18776_d_n0;
        locals.var_c_2esipq_nsub_dn2 = assign24340_e18776_d_n2;
        locals.var_c_2esipq_nsub_dn4 = assign24340_e18776_d_n4;
        locals.var_c_2esipq_nsub_dn5 = assign24340_e18776_d_n5;
        locals.var_c_2esipq_nsub_dn6 = assign24340_e18776_d_n6;
        locals.var_c_2esipq_nsub_dn7 = assign24340_e18776_d_n7;
        locals.var_c_2esipq_nsub_dn8 = assign24340_e18776_d_n8;
        locals.var_c_2esipq_nsub_dn9 = assign24340_e18776_d_n9;
        locals.var_c_2esipq_nsub_dn10 = assign24340_e18776_d_n10;
        locals.var_c_2esipq_nsub_dn11 = assign24340_e18776_d_n11;
        locals.var_c_2esipq_nsub_dn14 = assign24340_e18776_d_n14;
        locals.var_c_2esipq_nsub_rv = 0.0;

        let (assign24350_e18786, assign24350_e18786_d_n0, assign24350_e18786_d_n2, assign24350_e18786_d_n4, assign24350_e18786_d_n5, assign24350_e18786_d_n6, assign24350_e18786_d_n7, assign24350_e18786_d_n8, assign24350_e18786_d_n9, assign24350_e18786_d_n10, assign24350_e18786_d_n11, assign24350_e18786_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign24350_e18783: f64 = (2.0 * 1.034943e-10);
        let assign24350_e18784: f64 = (locals.var_q_nsub__blk548 / assign24350_e18783);
        (assign24350_e18784, (locals.var_q_nsub__blk548_dn0 / assign24350_e18783), (locals.var_q_nsub__blk548_dn2 / assign24350_e18783), (locals.var_q_nsub__blk548_dn4 / assign24350_e18783), (locals.var_q_nsub__blk548_dn5 / assign24350_e18783), (locals.var_q_nsub__blk548_dn6 / assign24350_e18783), (locals.var_q_nsub__blk548_dn7 / assign24350_e18783), (locals.var_q_nsub__blk548_dn8 / assign24350_e18783), (locals.var_q_nsub__blk548_dn9 / assign24350_e18783), (locals.var_q_nsub__blk548_dn10 / assign24350_e18783), (locals.var_q_nsub__blk548_dn11 / assign24350_e18783), (locals.var_q_nsub__blk548_dn14 / assign24350_e18783),)
    } else {
        (locals.var_c_2esipq_nsub_inv, locals.var_c_2esipq_nsub_inv_dn0, locals.var_c_2esipq_nsub_inv_dn2, locals.var_c_2esipq_nsub_inv_dn4, locals.var_c_2esipq_nsub_inv_dn5, locals.var_c_2esipq_nsub_inv_dn6, locals.var_c_2esipq_nsub_inv_dn7, locals.var_c_2esipq_nsub_inv_dn8, locals.var_c_2esipq_nsub_inv_dn9, locals.var_c_2esipq_nsub_inv_dn10, locals.var_c_2esipq_nsub_inv_dn11, locals.var_c_2esipq_nsub_inv_dn14,)
    }
};
        locals.var_c_2esipq_nsub_inv = assign24350_e18786;
        locals.var_c_2esipq_nsub_inv_dn0 = assign24350_e18786_d_n0;
        locals.var_c_2esipq_nsub_inv_dn2 = assign24350_e18786_d_n2;
        locals.var_c_2esipq_nsub_inv_dn4 = assign24350_e18786_d_n4;
        locals.var_c_2esipq_nsub_inv_dn5 = assign24350_e18786_d_n5;
        locals.var_c_2esipq_nsub_inv_dn6 = assign24350_e18786_d_n6;
        locals.var_c_2esipq_nsub_inv_dn7 = assign24350_e18786_d_n7;
        locals.var_c_2esipq_nsub_inv_dn8 = assign24350_e18786_d_n8;
        locals.var_c_2esipq_nsub_inv_dn9 = assign24350_e18786_d_n9;
        locals.var_c_2esipq_nsub_inv_dn10 = assign24350_e18786_d_n10;
        locals.var_c_2esipq_nsub_inv_dn11 = assign24350_e18786_d_n11;
        locals.var_c_2esipq_nsub_inv_dn14 = assign24350_e18786_d_n14;
        locals.var_c_2esipq_nsub_inv_rv = 0.0;

        let (assign24360_e18794, assign24360_e18794_d_n0, assign24360_e18794_d_n2, assign24360_e18794_d_n4, assign24360_e18794_d_n5, assign24360_e18794_d_n6, assign24360_e18794_d_n7, assign24360_e18794_d_n8, assign24360_e18794_d_n9, assign24360_e18794_d_n10, assign24360_e18794_d_n11, assign24360_e18794_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign24360_e18792: f64 = (locals.var_uc_ndepm / locals.var_ef_nsubc);
        (assign24360_e18792, (((locals.var_uc_ndepm_dn0 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn0)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn2 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn2)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn4 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn4)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn5 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn5)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn6 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn6)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn7 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn7)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn8 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn8)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn9 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn9)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn10 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn10)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn11 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn11)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn14 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn14)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)),)
    } else {
        (locals.var_ndepmpnsub, locals.var_ndepmpnsub_dn0, locals.var_ndepmpnsub_dn2, locals.var_ndepmpnsub_dn4, locals.var_ndepmpnsub_dn5, locals.var_ndepmpnsub_dn6, locals.var_ndepmpnsub_dn7, locals.var_ndepmpnsub_dn8, locals.var_ndepmpnsub_dn9, locals.var_ndepmpnsub_dn10, locals.var_ndepmpnsub_dn11, locals.var_ndepmpnsub_dn14,)
    }
};
        locals.var_ndepmpnsub = assign24360_e18794;
        locals.var_ndepmpnsub_dn0 = assign24360_e18794_d_n0;
        locals.var_ndepmpnsub_dn2 = assign24360_e18794_d_n2;
        locals.var_ndepmpnsub_dn4 = assign24360_e18794_d_n4;
        locals.var_ndepmpnsub_dn5 = assign24360_e18794_d_n5;
        locals.var_ndepmpnsub_dn6 = assign24360_e18794_d_n6;
        locals.var_ndepmpnsub_dn7 = assign24360_e18794_d_n7;
        locals.var_ndepmpnsub_dn8 = assign24360_e18794_d_n8;
        locals.var_ndepmpnsub_dn9 = assign24360_e18794_d_n9;
        locals.var_ndepmpnsub_dn10 = assign24360_e18794_d_n10;
        locals.var_ndepmpnsub_dn11 = assign24360_e18794_d_n11;
        locals.var_ndepmpnsub_dn14 = assign24360_e18794_d_n14;
        locals.var_ndepmpnsub_rv = 0.0;

        let (assign24370_e18804, assign24370_e18804_d_n0, assign24370_e18804_d_n2, assign24370_e18804_d_n4, assign24370_e18804_d_n5, assign24370_e18804_d_n6, assign24370_e18804_d_n7, assign24370_e18804_d_n8, assign24370_e18804_d_n9, assign24370_e18804_d_n10, assign24370_e18804_d_n11, assign24370_e18804_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign24370_e18801: f64 = (1.0 + locals.var_ndepmpnsub);
        let assign24370_e18802: f64 = (1.0 / assign24370_e18801);
        (assign24370_e18802, (-(locals.var_ndepmpnsub_dn0 / (assign24370_e18801 * assign24370_e18801))), (-(locals.var_ndepmpnsub_dn2 / (assign24370_e18801 * assign24370_e18801))), (-(locals.var_ndepmpnsub_dn4 / (assign24370_e18801 * assign24370_e18801))), (-(locals.var_ndepmpnsub_dn5 / (assign24370_e18801 * assign24370_e18801))), (-(locals.var_ndepmpnsub_dn6 / (assign24370_e18801 * assign24370_e18801))), (-(locals.var_ndepmpnsub_dn7 / (assign24370_e18801 * assign24370_e18801))), (-(locals.var_ndepmpnsub_dn8 / (assign24370_e18801 * assign24370_e18801))), (-(locals.var_ndepmpnsub_dn9 / (assign24370_e18801 * assign24370_e18801))), (-(locals.var_ndepmpnsub_dn10 / (assign24370_e18801 * assign24370_e18801))), (-(locals.var_ndepmpnsub_dn11 / (assign24370_e18801 * assign24370_e18801))), (-(locals.var_ndepmpnsub_dn14 / (assign24370_e18801 * assign24370_e18801))),)
    } else {
        (locals.var_ndepmpnsub_inv1, locals.var_ndepmpnsub_inv1_dn0, locals.var_ndepmpnsub_inv1_dn2, locals.var_ndepmpnsub_inv1_dn4, locals.var_ndepmpnsub_inv1_dn5, locals.var_ndepmpnsub_inv1_dn6, locals.var_ndepmpnsub_inv1_dn7, locals.var_ndepmpnsub_inv1_dn8, locals.var_ndepmpnsub_inv1_dn9, locals.var_ndepmpnsub_inv1_dn10, locals.var_ndepmpnsub_inv1_dn11, locals.var_ndepmpnsub_inv1_dn14,)
    }
};
        locals.var_ndepmpnsub_inv1 = assign24370_e18804;
        locals.var_ndepmpnsub_inv1_dn0 = assign24370_e18804_d_n0;
        locals.var_ndepmpnsub_inv1_dn2 = assign24370_e18804_d_n2;
        locals.var_ndepmpnsub_inv1_dn4 = assign24370_e18804_d_n4;
        locals.var_ndepmpnsub_inv1_dn5 = assign24370_e18804_d_n5;
        locals.var_ndepmpnsub_inv1_dn6 = assign24370_e18804_d_n6;
        locals.var_ndepmpnsub_inv1_dn7 = assign24370_e18804_d_n7;
        locals.var_ndepmpnsub_inv1_dn8 = assign24370_e18804_d_n8;
        locals.var_ndepmpnsub_inv1_dn9 = assign24370_e18804_d_n9;
        locals.var_ndepmpnsub_inv1_dn10 = assign24370_e18804_d_n10;
        locals.var_ndepmpnsub_inv1_dn11 = assign24370_e18804_d_n11;
        locals.var_ndepmpnsub_inv1_dn14 = assign24370_e18804_d_n14;
        locals.var_ndepmpnsub_inv1_rv = 0.0;

        let (assign24380_e18812,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign24380_e18810: f64 = (1e-12 * 1000.0);
        (assign24380_e18810,)
    } else {
        (locals.var_ps_conv3,)
    }
};
        locals.var_ps_conv3 = assign24380_e18812;
        locals.var_ps_conv3_rv = 0.0;

        let (assign24390_e18820,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign24390_e18818: f64 = (1e-10 * 1000.0);
        (assign24390_e18818,)
    } else {
        (locals.var_ps_conv23,)
    }
};
        locals.var_ps_conv23 = assign24390_e18820;
        locals.var_ps_conv23_rv = 0.0;

        let (assign24400_e18826, assign24400_e18826_d_n0, assign24400_e18826_d_n2, assign24400_e18826_d_n4, assign24400_e18826_d_n5, assign24400_e18826_d_n6, assign24400_e18826_d_n7, assign24400_e18826_d_n8, assign24400_e18826_d_n9, assign24400_e18826_d_n10, assign24400_e18826_d_n11, assign24400_e18826_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_s0_dep, locals.var_phi_s0_dep_dn0, locals.var_phi_s0_dep_dn2, locals.var_phi_s0_dep_dn4, locals.var_phi_s0_dep_dn5, locals.var_phi_s0_dep_dn6, locals.var_phi_s0_dep_dn7, locals.var_phi_s0_dep_dn8, locals.var_phi_s0_dep_dn9, locals.var_phi_s0_dep_dn10, locals.var_phi_s0_dep_dn11, locals.var_phi_s0_dep_dn14,)
    }
};
        locals.var_phi_s0_dep = assign24400_e18826;
        locals.var_phi_s0_dep_dn0 = assign24400_e18826_d_n0;
        locals.var_phi_s0_dep_dn2 = assign24400_e18826_d_n2;
        locals.var_phi_s0_dep_dn4 = assign24400_e18826_d_n4;
        locals.var_phi_s0_dep_dn5 = assign24400_e18826_d_n5;
        locals.var_phi_s0_dep_dn6 = assign24400_e18826_d_n6;
        locals.var_phi_s0_dep_dn7 = assign24400_e18826_d_n7;
        locals.var_phi_s0_dep_dn8 = assign24400_e18826_d_n8;
        locals.var_phi_s0_dep_dn9 = assign24400_e18826_d_n9;
        locals.var_phi_s0_dep_dn10 = assign24400_e18826_d_n10;
        locals.var_phi_s0_dep_dn11 = assign24400_e18826_d_n11;
        locals.var_phi_s0_dep_dn14 = assign24400_e18826_d_n14;
        locals.var_phi_s0_dep_rv = 0.0;

        let (assign24410_e18832, assign24410_e18832_d_n0, assign24410_e18832_d_n2, assign24410_e18832_d_n4, assign24410_e18832_d_n5, assign24410_e18832_d_n6, assign24410_e18832_d_n7, assign24410_e18832_d_n8, assign24410_e18832_d_n9, assign24410_e18832_d_n10, assign24410_e18832_d_n11, assign24410_e18832_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_sl_dep, locals.var_phi_sl_dep_dn0, locals.var_phi_sl_dep_dn2, locals.var_phi_sl_dep_dn4, locals.var_phi_sl_dep_dn5, locals.var_phi_sl_dep_dn6, locals.var_phi_sl_dep_dn7, locals.var_phi_sl_dep_dn8, locals.var_phi_sl_dep_dn9, locals.var_phi_sl_dep_dn10, locals.var_phi_sl_dep_dn11, locals.var_phi_sl_dep_dn14,)
    }
};
        locals.var_phi_sl_dep = assign24410_e18832;
        locals.var_phi_sl_dep_dn0 = assign24410_e18832_d_n0;
        locals.var_phi_sl_dep_dn2 = assign24410_e18832_d_n2;
        locals.var_phi_sl_dep_dn4 = assign24410_e18832_d_n4;
        locals.var_phi_sl_dep_dn5 = assign24410_e18832_d_n5;
        locals.var_phi_sl_dep_dn6 = assign24410_e18832_d_n6;
        locals.var_phi_sl_dep_dn7 = assign24410_e18832_d_n7;
        locals.var_phi_sl_dep_dn8 = assign24410_e18832_d_n8;
        locals.var_phi_sl_dep_dn9 = assign24410_e18832_d_n9;
        locals.var_phi_sl_dep_dn10 = assign24410_e18832_d_n10;
        locals.var_phi_sl_dep_dn11 = assign24410_e18832_d_n11;
        locals.var_phi_sl_dep_dn14 = assign24410_e18832_d_n14;
        locals.var_phi_sl_dep_rv = 0.0;

        let (assign24420_e18838, assign24420_e18838_d_n0, assign24420_e18838_d_n2, assign24420_e18838_d_n4, assign24420_e18838_d_n5, assign24420_e18838_d_n6, assign24420_e18838_d_n7, assign24420_e18838_d_n8, assign24420_e18838_d_n9, assign24420_e18838_d_n10, assign24420_e18838_d_n11, assign24420_e18838_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_s0, locals.var_q_s0_dn0, locals.var_q_s0_dn2, locals.var_q_s0_dn4, locals.var_q_s0_dn5, locals.var_q_s0_dn6, locals.var_q_s0_dn7, locals.var_q_s0_dn8, locals.var_q_s0_dn9, locals.var_q_s0_dn10, locals.var_q_s0_dn11, locals.var_q_s0_dn14,)
    }
};
        locals.var_q_s0 = assign24420_e18838;
        locals.var_q_s0_dn0 = assign24420_e18838_d_n0;
        locals.var_q_s0_dn2 = assign24420_e18838_d_n2;
        locals.var_q_s0_dn4 = assign24420_e18838_d_n4;
        locals.var_q_s0_dn5 = assign24420_e18838_d_n5;
        locals.var_q_s0_dn6 = assign24420_e18838_d_n6;
        locals.var_q_s0_dn7 = assign24420_e18838_d_n7;
        locals.var_q_s0_dn8 = assign24420_e18838_d_n8;
        locals.var_q_s0_dn9 = assign24420_e18838_d_n9;
        locals.var_q_s0_dn10 = assign24420_e18838_d_n10;
        locals.var_q_s0_dn11 = assign24420_e18838_d_n11;
        locals.var_q_s0_dn14 = assign24420_e18838_d_n14;
        locals.var_q_s0_rv = 0.0;

        let (assign24430_e18844, assign24430_e18844_d_n0, assign24430_e18844_d_n2, assign24430_e18844_d_n4, assign24430_e18844_d_n5, assign24430_e18844_d_n6, assign24430_e18844_d_n7, assign24430_e18844_d_n8, assign24430_e18844_d_n9, assign24430_e18844_d_n10, assign24430_e18844_d_n11, assign24430_e18844_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_sl, locals.var_q_sl_dn0, locals.var_q_sl_dn2, locals.var_q_sl_dn4, locals.var_q_sl_dn5, locals.var_q_sl_dn6, locals.var_q_sl_dn7, locals.var_q_sl_dn8, locals.var_q_sl_dn9, locals.var_q_sl_dn10, locals.var_q_sl_dn11, locals.var_q_sl_dn14,)
    }
};
        locals.var_q_sl = assign24430_e18844;
        locals.var_q_sl_dn0 = assign24430_e18844_d_n0;
        locals.var_q_sl_dn2 = assign24430_e18844_d_n2;
        locals.var_q_sl_dn4 = assign24430_e18844_d_n4;
        locals.var_q_sl_dn5 = assign24430_e18844_d_n5;
        locals.var_q_sl_dn6 = assign24430_e18844_d_n6;
        locals.var_q_sl_dn7 = assign24430_e18844_d_n7;
        locals.var_q_sl_dn8 = assign24430_e18844_d_n8;
        locals.var_q_sl_dn9 = assign24430_e18844_d_n9;
        locals.var_q_sl_dn10 = assign24430_e18844_d_n10;
        locals.var_q_sl_dn11 = assign24430_e18844_d_n11;
        locals.var_q_sl_dn14 = assign24430_e18844_d_n14;
        locals.var_q_sl_rv = 0.0;

        let (assign24440_e18850, assign24440_e18850_d_n0, assign24440_e18850_d_n2, assign24440_e18850_d_n4, assign24440_e18850_d_n5, assign24440_e18850_d_n6, assign24440_e18850_d_n7, assign24440_e18850_d_n8, assign24440_e18850_d_n9, assign24440_e18850_d_n10, assign24440_e18850_d_n11, assign24440_e18850_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_s0_dep, locals.var_q_s0_dep_dn0, locals.var_q_s0_dep_dn2, locals.var_q_s0_dep_dn4, locals.var_q_s0_dep_dn5, locals.var_q_s0_dep_dn6, locals.var_q_s0_dep_dn7, locals.var_q_s0_dep_dn8, locals.var_q_s0_dep_dn9, locals.var_q_s0_dep_dn10, locals.var_q_s0_dep_dn11, locals.var_q_s0_dep_dn14,)
    }
};
        locals.var_q_s0_dep = assign24440_e18850;
        locals.var_q_s0_dep_dn0 = assign24440_e18850_d_n0;
        locals.var_q_s0_dep_dn2 = assign24440_e18850_d_n2;
        locals.var_q_s0_dep_dn4 = assign24440_e18850_d_n4;
        locals.var_q_s0_dep_dn5 = assign24440_e18850_d_n5;
        locals.var_q_s0_dep_dn6 = assign24440_e18850_d_n6;
        locals.var_q_s0_dep_dn7 = assign24440_e18850_d_n7;
        locals.var_q_s0_dep_dn8 = assign24440_e18850_d_n8;
        locals.var_q_s0_dep_dn9 = assign24440_e18850_d_n9;
        locals.var_q_s0_dep_dn10 = assign24440_e18850_d_n10;
        locals.var_q_s0_dep_dn11 = assign24440_e18850_d_n11;
        locals.var_q_s0_dep_dn14 = assign24440_e18850_d_n14;
        locals.var_q_s0_dep_rv = 0.0;

        let (assign24450_e18856, assign24450_e18856_d_n0, assign24450_e18856_d_n2, assign24450_e18856_d_n4, assign24450_e18856_d_n5, assign24450_e18856_d_n6, assign24450_e18856_d_n7, assign24450_e18856_d_n8, assign24450_e18856_d_n9, assign24450_e18856_d_n10, assign24450_e18856_d_n11, assign24450_e18856_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_sl_dep, locals.var_q_sl_dep_dn0, locals.var_q_sl_dep_dn2, locals.var_q_sl_dep_dn4, locals.var_q_sl_dep_dn5, locals.var_q_sl_dep_dn6, locals.var_q_sl_dep_dn7, locals.var_q_sl_dep_dn8, locals.var_q_sl_dep_dn9, locals.var_q_sl_dep_dn10, locals.var_q_sl_dep_dn11, locals.var_q_sl_dep_dn14,)
    }
};
        locals.var_q_sl_dep = assign24450_e18856;
        locals.var_q_sl_dep_dn0 = assign24450_e18856_d_n0;
        locals.var_q_sl_dep_dn2 = assign24450_e18856_d_n2;
        locals.var_q_sl_dep_dn4 = assign24450_e18856_d_n4;
        locals.var_q_sl_dep_dn5 = assign24450_e18856_d_n5;
        locals.var_q_sl_dep_dn6 = assign24450_e18856_d_n6;
        locals.var_q_sl_dep_dn7 = assign24450_e18856_d_n7;
        locals.var_q_sl_dep_dn8 = assign24450_e18856_d_n8;
        locals.var_q_sl_dep_dn9 = assign24450_e18856_d_n9;
        locals.var_q_sl_dep_dn10 = assign24450_e18856_d_n10;
        locals.var_q_sl_dep_dn11 = assign24450_e18856_d_n11;
        locals.var_q_sl_dep_dn14 = assign24450_e18856_d_n14;
        locals.var_q_sl_dep_rv = 0.0;

        let (assign24460_e18862, assign24460_e18862_d_n0, assign24460_e18862_d_n2, assign24460_e18862_d_n4, assign24460_e18862_d_n5, assign24460_e18862_d_n6, assign24460_e18862_d_n7, assign24460_e18862_d_n8, assign24460_e18862_d_n9, assign24460_e18862_d_n10, assign24460_e18862_d_n11, assign24460_e18862_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_b0_dep, locals.var_q_b0_dep_dn0, locals.var_q_b0_dep_dn2, locals.var_q_b0_dep_dn4, locals.var_q_b0_dep_dn5, locals.var_q_b0_dep_dn6, locals.var_q_b0_dep_dn7, locals.var_q_b0_dep_dn8, locals.var_q_b0_dep_dn9, locals.var_q_b0_dep_dn10, locals.var_q_b0_dep_dn11, locals.var_q_b0_dep_dn14,)
    }
};
        locals.var_q_b0_dep = assign24460_e18862;
        locals.var_q_b0_dep_dn0 = assign24460_e18862_d_n0;
        locals.var_q_b0_dep_dn2 = assign24460_e18862_d_n2;
        locals.var_q_b0_dep_dn4 = assign24460_e18862_d_n4;
        locals.var_q_b0_dep_dn5 = assign24460_e18862_d_n5;
        locals.var_q_b0_dep_dn6 = assign24460_e18862_d_n6;
        locals.var_q_b0_dep_dn7 = assign24460_e18862_d_n7;
        locals.var_q_b0_dep_dn8 = assign24460_e18862_d_n8;
        locals.var_q_b0_dep_dn9 = assign24460_e18862_d_n9;
        locals.var_q_b0_dep_dn10 = assign24460_e18862_d_n10;
        locals.var_q_b0_dep_dn11 = assign24460_e18862_d_n11;
        locals.var_q_b0_dep_dn14 = assign24460_e18862_d_n14;
        locals.var_q_b0_dep_rv = 0.0;

        let (assign24470_e18868, assign24470_e18868_d_n0, assign24470_e18868_d_n2, assign24470_e18868_d_n4, assign24470_e18868_d_n5, assign24470_e18868_d_n6, assign24470_e18868_d_n7, assign24470_e18868_d_n8, assign24470_e18868_d_n9, assign24470_e18868_d_n10, assign24470_e18868_d_n11, assign24470_e18868_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn4, locals.var_q_bl_dep_dn5, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn8, locals.var_q_bl_dep_dn9, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn14,)
    }
};
        locals.var_q_bl_dep = assign24470_e18868;
        locals.var_q_bl_dep_dn0 = assign24470_e18868_d_n0;
        locals.var_q_bl_dep_dn2 = assign24470_e18868_d_n2;
        locals.var_q_bl_dep_dn4 = assign24470_e18868_d_n4;
        locals.var_q_bl_dep_dn5 = assign24470_e18868_d_n5;
        locals.var_q_bl_dep_dn6 = assign24470_e18868_d_n6;
        locals.var_q_bl_dep_dn7 = assign24470_e18868_d_n7;
        locals.var_q_bl_dep_dn8 = assign24470_e18868_d_n8;
        locals.var_q_bl_dep_dn9 = assign24470_e18868_d_n9;
        locals.var_q_bl_dep_dn10 = assign24470_e18868_d_n10;
        locals.var_q_bl_dep_dn11 = assign24470_e18868_d_n11;
        locals.var_q_bl_dep_dn14 = assign24470_e18868_d_n14;
        locals.var_q_bl_dep_rv = 0.0;

        let (assign24480_e18874, assign24480_e18874_d_n0, assign24480_e18874_d_n2, assign24480_e18874_d_n4, assign24480_e18874_d_n5, assign24480_e18874_d_n6, assign24480_e18874_d_n7, assign24480_e18874_d_n8, assign24480_e18874_d_n9, assign24480_e18874_d_n10, assign24480_e18874_d_n11, assign24480_e18874_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_sub0_dep, locals.var_q_sub0_dep_dn0, locals.var_q_sub0_dep_dn2, locals.var_q_sub0_dep_dn4, locals.var_q_sub0_dep_dn5, locals.var_q_sub0_dep_dn6, locals.var_q_sub0_dep_dn7, locals.var_q_sub0_dep_dn8, locals.var_q_sub0_dep_dn9, locals.var_q_sub0_dep_dn10, locals.var_q_sub0_dep_dn11, locals.var_q_sub0_dep_dn14,)
    }
};
        locals.var_q_sub0_dep = assign24480_e18874;
        locals.var_q_sub0_dep_dn0 = assign24480_e18874_d_n0;
        locals.var_q_sub0_dep_dn2 = assign24480_e18874_d_n2;
        locals.var_q_sub0_dep_dn4 = assign24480_e18874_d_n4;
        locals.var_q_sub0_dep_dn5 = assign24480_e18874_d_n5;
        locals.var_q_sub0_dep_dn6 = assign24480_e18874_d_n6;
        locals.var_q_sub0_dep_dn7 = assign24480_e18874_d_n7;
        locals.var_q_sub0_dep_dn8 = assign24480_e18874_d_n8;
        locals.var_q_sub0_dep_dn9 = assign24480_e18874_d_n9;
        locals.var_q_sub0_dep_dn10 = assign24480_e18874_d_n10;
        locals.var_q_sub0_dep_dn11 = assign24480_e18874_d_n11;
        locals.var_q_sub0_dep_dn14 = assign24480_e18874_d_n14;
        locals.var_q_sub0_dep_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_68(
        locals: &mut StampLocals,
    ) {
        let (assign24490_e18880, assign24490_e18880_d_n0, assign24490_e18880_d_n2, assign24490_e18880_d_n4, assign24490_e18880_d_n5, assign24490_e18880_d_n6, assign24490_e18880_d_n7, assign24490_e18880_d_n8, assign24490_e18880_d_n9, assign24490_e18880_d_n10, assign24490_e18880_d_n11, assign24490_e18880_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_subl_dep, locals.var_q_subl_dep_dn0, locals.var_q_subl_dep_dn2, locals.var_q_subl_dep_dn4, locals.var_q_subl_dep_dn5, locals.var_q_subl_dep_dn6, locals.var_q_subl_dep_dn7, locals.var_q_subl_dep_dn8, locals.var_q_subl_dep_dn9, locals.var_q_subl_dep_dn10, locals.var_q_subl_dep_dn11, locals.var_q_subl_dep_dn14,)
    }
};
        locals.var_q_subl_dep = assign24490_e18880;
        locals.var_q_subl_dep_dn0 = assign24490_e18880_d_n0;
        locals.var_q_subl_dep_dn2 = assign24490_e18880_d_n2;
        locals.var_q_subl_dep_dn4 = assign24490_e18880_d_n4;
        locals.var_q_subl_dep_dn5 = assign24490_e18880_d_n5;
        locals.var_q_subl_dep_dn6 = assign24490_e18880_d_n6;
        locals.var_q_subl_dep_dn7 = assign24490_e18880_d_n7;
        locals.var_q_subl_dep_dn8 = assign24490_e18880_d_n8;
        locals.var_q_subl_dep_dn9 = assign24490_e18880_d_n9;
        locals.var_q_subl_dep_dn10 = assign24490_e18880_d_n10;
        locals.var_q_subl_dep_dn11 = assign24490_e18880_d_n11;
        locals.var_q_subl_dep_dn14 = assign24490_e18880_d_n14;
        locals.var_q_subl_dep_rv = 0.0;

        let (assign24500_e18886, assign24500_e18886_d_n0, assign24500_e18886_d_n2, assign24500_e18886_d_n4, assign24500_e18886_d_n5, assign24500_e18886_d_n6, assign24500_e18886_d_n7, assign24500_e18886_d_n8, assign24500_e18886_d_n9, assign24500_e18886_d_n10, assign24500_e18886_d_n11, assign24500_e18886_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phib_ref, locals.var_phib_ref_dn0, locals.var_phib_ref_dn2, locals.var_phib_ref_dn4, locals.var_phib_ref_dn5, locals.var_phib_ref_dn6, locals.var_phib_ref_dn7, locals.var_phib_ref_dn8, locals.var_phib_ref_dn9, locals.var_phib_ref_dn10, locals.var_phib_ref_dn11, locals.var_phib_ref_dn14,)
    }
};
        locals.var_phib_ref = assign24500_e18886;
        locals.var_phib_ref_dn0 = assign24500_e18886_d_n0;
        locals.var_phib_ref_dn2 = assign24500_e18886_d_n2;
        locals.var_phib_ref_dn4 = assign24500_e18886_d_n4;
        locals.var_phib_ref_dn5 = assign24500_e18886_d_n5;
        locals.var_phib_ref_dn6 = assign24500_e18886_d_n6;
        locals.var_phib_ref_dn7 = assign24500_e18886_d_n7;
        locals.var_phib_ref_dn8 = assign24500_e18886_d_n8;
        locals.var_phib_ref_dn9 = assign24500_e18886_d_n9;
        locals.var_phib_ref_dn10 = assign24500_e18886_d_n10;
        locals.var_phib_ref_dn11 = assign24500_e18886_d_n11;
        locals.var_phib_ref_dn14 = assign24500_e18886_d_n14;
        locals.var_phib_ref_rv = 0.0;

        let (assign24510_e18898, assign24510_e18898_d_n0, assign24510_e18898_d_n2, assign24510_e18898_d_n4, assign24510_e18898_d_n5, assign24510_e18898_d_n6, assign24510_e18898_d_n7, assign24510_e18898_d_n8, assign24510_e18898_d_n9, assign24510_e18898_d_n10, assign24510_e18898_d_n11, assign24510_e18898_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign24510_e18893: f64 = (10.0 * 2.220446049250313e-16);
        let assign24510_e18895: f64 = (assign24510_e18893 * 10000000.0);
        let assign24510_e18896: f64 = (locals.var_vgp + assign24510_e18895);
        (assign24510_e18896, locals.var_vgp_dn0, locals.var_vgp_dn2, locals.var_vgp_dn4, locals.var_vgp_dn5, locals.var_vgp_dn6, locals.var_vgp_dn7, locals.var_vgp_dn8, locals.var_vgp_dn9, locals.var_vgp_dn10, locals.var_vgp_dn11, locals.var_vgp_dn14,)
    } else {
        (locals.var_vgp, locals.var_vgp_dn0, locals.var_vgp_dn2, locals.var_vgp_dn4, locals.var_vgp_dn5, locals.var_vgp_dn6, locals.var_vgp_dn7, locals.var_vgp_dn8, locals.var_vgp_dn9, locals.var_vgp_dn10, locals.var_vgp_dn11, locals.var_vgp_dn14,)
    }
};
        locals.var_vgp = assign24510_e18898;
        locals.var_vgp_dn0 = assign24510_e18898_d_n0;
        locals.var_vgp_dn2 = assign24510_e18898_d_n2;
        locals.var_vgp_dn4 = assign24510_e18898_d_n4;
        locals.var_vgp_dn5 = assign24510_e18898_d_n5;
        locals.var_vgp_dn6 = assign24510_e18898_d_n6;
        locals.var_vgp_dn7 = assign24510_e18898_d_n7;
        locals.var_vgp_dn8 = assign24510_e18898_d_n8;
        locals.var_vgp_dn9 = assign24510_e18898_d_n9;
        locals.var_vgp_dn10 = assign24510_e18898_d_n10;
        locals.var_vgp_dn11 = assign24510_e18898_d_n11;
        locals.var_vgp_dn14 = assign24510_e18898_d_n14;
        locals.var_vgp_rv = 0.0;

        let (assign24520_e18910, assign24520_e18910_d_n0, assign24520_e18910_d_n2, assign24520_e18910_d_n4, assign24520_e18910_d_n5, assign24520_e18910_d_n6, assign24520_e18910_d_n7, assign24520_e18910_d_n8, assign24520_e18910_d_n9, assign24520_e18910_d_n10, assign24520_e18910_d_n11, assign24520_e18910_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign24520_e18904: f64 = (locals.var_cox * locals.var_cox);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_cnst0;
        let assign24520_e18906: f64 = (assign24520_e18904 * __rspice_inv_cse_0);
        let assign24520_e18908: f64 = (assign24520_e18906 * __rspice_inv_cse_0);
        (assign24520_e18908, ((((((((locals.var_cox_dn0 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn0)) * locals.var_cnst0) - (assign24520_e18904 * locals.var_cnst0_dn0)) / (locals.var_cnst0 * locals.var_cnst0)) * locals.var_cnst0) - (assign24520_e18906 * locals.var_cnst0_dn0)) / (locals.var_cnst0 * locals.var_cnst0)), ((((((((locals.var_cox_dn2 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn2)) * locals.var_cnst0) - (assign24520_e18904 * locals.var_cnst0_dn2)) / (locals.var_cnst0 * locals.var_cnst0)) * locals.var_cnst0) - (assign24520_e18906 * locals.var_cnst0_dn2)) / (locals.var_cnst0 * locals.var_cnst0)), ((((((((locals.var_cox_dn4 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn4)) * locals.var_cnst0) - (assign24520_e18904 * locals.var_cnst0_dn4)) / (locals.var_cnst0 * locals.var_cnst0)) * locals.var_cnst0) - (assign24520_e18906 * locals.var_cnst0_dn4)) / (locals.var_cnst0 * locals.var_cnst0)), ((((((((locals.var_cox_dn5 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn5)) * locals.var_cnst0) - (assign24520_e18904 * locals.var_cnst0_dn5)) / (locals.var_cnst0 * locals.var_cnst0)) * locals.var_cnst0) - (assign24520_e18906 * locals.var_cnst0_dn5)) / (locals.var_cnst0 * locals.var_cnst0)), ((((((((locals.var_cox_dn6 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn6)) * locals.var_cnst0) - (assign24520_e18904 * locals.var_cnst0_dn6)) / (locals.var_cnst0 * locals.var_cnst0)) * locals.var_cnst0) - (assign24520_e18906 * locals.var_cnst0_dn6)) / (locals.var_cnst0 * locals.var_cnst0)), ((((((((locals.var_cox_dn7 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn7)) * locals.var_cnst0) - (assign24520_e18904 * locals.var_cnst0_dn7)) / (locals.var_cnst0 * locals.var_cnst0)) * locals.var_cnst0) - (assign24520_e18906 * locals.var_cnst0_dn7)) / (locals.var_cnst0 * locals.var_cnst0)), ((((((((locals.var_cox_dn8 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn8)) * locals.var_cnst0) - (assign24520_e18904 * locals.var_cnst0_dn8)) / (locals.var_cnst0 * locals.var_cnst0)) * locals.var_cnst0) - (assign24520_e18906 * locals.var_cnst0_dn8)) / (locals.var_cnst0 * locals.var_cnst0)), ((((((((locals.var_cox_dn9 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn9)) * locals.var_cnst0) - (assign24520_e18904 * locals.var_cnst0_dn9)) / (locals.var_cnst0 * locals.var_cnst0)) * locals.var_cnst0) - (assign24520_e18906 * locals.var_cnst0_dn9)) / (locals.var_cnst0 * locals.var_cnst0)), ((((((((locals.var_cox_dn10 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn10)) * locals.var_cnst0) - (assign24520_e18904 * locals.var_cnst0_dn10)) / (locals.var_cnst0 * locals.var_cnst0)) * locals.var_cnst0) - (assign24520_e18906 * locals.var_cnst0_dn10)) / (locals.var_cnst0 * locals.var_cnst0)), ((((((((locals.var_cox_dn11 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn11)) * locals.var_cnst0) - (assign24520_e18904 * locals.var_cnst0_dn11)) / (locals.var_cnst0 * locals.var_cnst0)) * locals.var_cnst0) - (assign24520_e18906 * locals.var_cnst0_dn11)) / (locals.var_cnst0 * locals.var_cnst0)), ((((((((locals.var_cox_dn14 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn14)) * locals.var_cnst0) - (assign24520_e18904 * locals.var_cnst0_dn14)) / (locals.var_cnst0 * locals.var_cnst0)) * locals.var_cnst0) - (assign24520_e18906 * locals.var_cnst0_dn14)) / (locals.var_cnst0 * locals.var_cnst0)),)
    } else {
        (locals.var_afact, locals.var_afact_dn0, locals.var_afact_dn2, locals.var_afact_dn4, locals.var_afact_dn5, locals.var_afact_dn6, locals.var_afact_dn7, locals.var_afact_dn8, locals.var_afact_dn9, locals.var_afact_dn10, locals.var_afact_dn11, locals.var_afact_dn14,)
    }
};
        locals.var_afact = assign24520_e18910;
        locals.var_afact_dn0 = assign24520_e18910_d_n0;
        locals.var_afact_dn2 = assign24520_e18910_d_n2;
        locals.var_afact_dn4 = assign24520_e18910_d_n4;
        locals.var_afact_dn5 = assign24520_e18910_d_n5;
        locals.var_afact_dn6 = assign24520_e18910_d_n6;
        locals.var_afact_dn7 = assign24520_e18910_d_n7;
        locals.var_afact_dn8 = assign24520_e18910_d_n8;
        locals.var_afact_dn9 = assign24520_e18910_d_n9;
        locals.var_afact_dn10 = assign24520_e18910_d_n10;
        locals.var_afact_dn11 = assign24520_e18910_d_n11;
        locals.var_afact_dn14 = assign24520_e18910_d_n14;
        locals.var_afact_rv = 0.0;

        let (assign24530_e18922, assign24530_e18922_d_n0, assign24530_e18922_d_n2, assign24530_e18922_d_n4, assign24530_e18922_d_n5, assign24530_e18922_d_n6, assign24530_e18922_d_n7, assign24530_e18922_d_n8, assign24530_e18922_d_n9, assign24530_e18922_d_n10, assign24530_e18922_d_n11, assign24530_e18922_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_nin;
        let assign24530_e18916: f64 = (locals.var_afact * __rspice_inv_cse_1);
        let assign24530_e18918: f64 = (assign24530_e18916 * __rspice_inv_cse_1);
        let assign24530_e18920: f64 = (assign24530_e18918 * locals.var_ndepm2);
        (assign24530_e18920, ((((((((locals.var_afact_dn0 * locals.var_nin) - (locals.var_afact * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign24530_e18916 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) * locals.var_ndepm2) + (assign24530_e18918 * locals.var_ndepm2_dn0)), ((((((((locals.var_afact_dn2 * locals.var_nin) - (locals.var_afact * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign24530_e18916 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) * locals.var_ndepm2) + (assign24530_e18918 * locals.var_ndepm2_dn2)), ((((((((locals.var_afact_dn4 * locals.var_nin) - (locals.var_afact * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign24530_e18916 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) * locals.var_ndepm2) + (assign24530_e18918 * locals.var_ndepm2_dn4)), ((((((((locals.var_afact_dn5 * locals.var_nin) - (locals.var_afact * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign24530_e18916 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) * locals.var_ndepm2) + (assign24530_e18918 * locals.var_ndepm2_dn5)), ((((((((locals.var_afact_dn6 * locals.var_nin) - (locals.var_afact * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign24530_e18916 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) * locals.var_ndepm2) + (assign24530_e18918 * locals.var_ndepm2_dn6)), ((((((((locals.var_afact_dn7 * locals.var_nin) - (locals.var_afact * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign24530_e18916 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) * locals.var_ndepm2) + (assign24530_e18918 * locals.var_ndepm2_dn7)), ((((((((locals.var_afact_dn8 * locals.var_nin) - (locals.var_afact * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign24530_e18916 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) * locals.var_ndepm2) + (assign24530_e18918 * locals.var_ndepm2_dn8)), ((((((((locals.var_afact_dn9 * locals.var_nin) - (locals.var_afact * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign24530_e18916 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) * locals.var_ndepm2) + (assign24530_e18918 * locals.var_ndepm2_dn9)), ((((((((locals.var_afact_dn10 * locals.var_nin) - (locals.var_afact * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign24530_e18916 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) * locals.var_ndepm2) + (assign24530_e18918 * locals.var_ndepm2_dn10)), ((((((((locals.var_afact_dn11 * locals.var_nin) - (locals.var_afact * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign24530_e18916 * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) * locals.var_ndepm2) + (assign24530_e18918 * locals.var_ndepm2_dn11)), ((((((((locals.var_afact_dn14 * locals.var_nin) - (locals.var_afact * locals.var_nin_dn14)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign24530_e18916 * locals.var_nin_dn14)) / (locals.var_nin * locals.var_nin)) * locals.var_ndepm2) + (assign24530_e18918 * locals.var_ndepm2_dn14)),)
    } else {
        (locals.var_afact2, locals.var_afact2_dn0, locals.var_afact2_dn2, locals.var_afact2_dn4, locals.var_afact2_dn5, locals.var_afact2_dn6, locals.var_afact2_dn7, locals.var_afact2_dn8, locals.var_afact2_dn9, locals.var_afact2_dn10, locals.var_afact2_dn11, locals.var_afact2_dn14,)
    }
};
        locals.var_afact2 = assign24530_e18922;
        locals.var_afact2_dn0 = assign24530_e18922_d_n0;
        locals.var_afact2_dn2 = assign24530_e18922_d_n2;
        locals.var_afact2_dn4 = assign24530_e18922_d_n4;
        locals.var_afact2_dn5 = assign24530_e18922_d_n5;
        locals.var_afact2_dn6 = assign24530_e18922_d_n6;
        locals.var_afact2_dn7 = assign24530_e18922_d_n7;
        locals.var_afact2_dn8 = assign24530_e18922_d_n8;
        locals.var_afact2_dn9 = assign24530_e18922_d_n9;
        locals.var_afact2_dn10 = assign24530_e18922_d_n10;
        locals.var_afact2_dn11 = assign24530_e18922_d_n11;
        locals.var_afact2_dn14 = assign24530_e18922_d_n14;
        locals.var_afact2_rv = 0.0;

        let (assign24540_e18940, assign24540_e18940_d_n0, assign24540_e18940_d_n2, assign24540_e18940_d_n4, assign24540_e18940_d_n5, assign24540_e18940_d_n6, assign24540_e18940_d_n7, assign24540_e18940_d_n8, assign24540_e18940_d_n9, assign24540_e18940_d_n10, assign24540_e18940_d_n11, assign24540_e18940_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign24540_e18928: f64 = (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc);
        let assign24540_e18931: f64 = (locals.var_ef_nsubc + locals.var_uc_ndepm);
        let assign24540_e18932: f64 = (assign24540_e18928 / assign24540_e18931);
        let assign24540_e18934: f64 = (-locals.var_vbscl__blk439);
        let assign24540_e18936: f64 = (assign24540_e18934 + locals.var_vbi_dep);
        let assign24540_e18937: f64 = (assign24540_e18932 * assign24540_e18936);
        let assign24540_e18938: f64 = (assign24540_e18937).sqrt();
        (assign24540_e18938, ((((((((locals.var_c_2esipq_ndepm_dn0 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn0)) * assign24540_e18931) - (assign24540_e18928 * (locals.var_ef_nsubc_dn0 + locals.var_uc_ndepm_dn0))) / (assign24540_e18931 * assign24540_e18931)) * assign24540_e18936) + (assign24540_e18932 * ((-locals.var_vbscl__blk439_dn0) + locals.var_vbi_dep_dn0))) / (2.0 * assign24540_e18938)), ((((((((locals.var_c_2esipq_ndepm_dn2 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn2)) * assign24540_e18931) - (assign24540_e18928 * (locals.var_ef_nsubc_dn2 + locals.var_uc_ndepm_dn2))) / (assign24540_e18931 * assign24540_e18931)) * assign24540_e18936) + (assign24540_e18932 * ((-locals.var_vbscl__blk439_dn2) + locals.var_vbi_dep_dn2))) / (2.0 * assign24540_e18938)), ((((((((locals.var_c_2esipq_ndepm_dn4 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn4)) * assign24540_e18931) - (assign24540_e18928 * (locals.var_ef_nsubc_dn4 + locals.var_uc_ndepm_dn4))) / (assign24540_e18931 * assign24540_e18931)) * assign24540_e18936) + (assign24540_e18932 * ((-locals.var_vbscl__blk439_dn4) + locals.var_vbi_dep_dn4))) / (2.0 * assign24540_e18938)), ((((((((locals.var_c_2esipq_ndepm_dn5 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn5)) * assign24540_e18931) - (assign24540_e18928 * (locals.var_ef_nsubc_dn5 + locals.var_uc_ndepm_dn5))) / (assign24540_e18931 * assign24540_e18931)) * assign24540_e18936) + (assign24540_e18932 * ((-locals.var_vbscl__blk439_dn5) + locals.var_vbi_dep_dn5))) / (2.0 * assign24540_e18938)), ((((((((locals.var_c_2esipq_ndepm_dn6 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn6)) * assign24540_e18931) - (assign24540_e18928 * (locals.var_ef_nsubc_dn6 + locals.var_uc_ndepm_dn6))) / (assign24540_e18931 * assign24540_e18931)) * assign24540_e18936) + (assign24540_e18932 * ((-locals.var_vbscl__blk439_dn6) + locals.var_vbi_dep_dn6))) / (2.0 * assign24540_e18938)), ((((((((locals.var_c_2esipq_ndepm_dn7 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn7)) * assign24540_e18931) - (assign24540_e18928 * (locals.var_ef_nsubc_dn7 + locals.var_uc_ndepm_dn7))) / (assign24540_e18931 * assign24540_e18931)) * assign24540_e18936) + (assign24540_e18932 * ((-locals.var_vbscl__blk439_dn7) + locals.var_vbi_dep_dn7))) / (2.0 * assign24540_e18938)), ((((((((locals.var_c_2esipq_ndepm_dn8 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn8)) * assign24540_e18931) - (assign24540_e18928 * (locals.var_ef_nsubc_dn8 + locals.var_uc_ndepm_dn8))) / (assign24540_e18931 * assign24540_e18931)) * assign24540_e18936) + (assign24540_e18932 * ((-locals.var_vbscl__blk439_dn8) + locals.var_vbi_dep_dn8))) / (2.0 * assign24540_e18938)), ((((((((locals.var_c_2esipq_ndepm_dn9 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn9)) * assign24540_e18931) - (assign24540_e18928 * (locals.var_ef_nsubc_dn9 + locals.var_uc_ndepm_dn9))) / (assign24540_e18931 * assign24540_e18931)) * assign24540_e18936) + (assign24540_e18932 * ((-locals.var_vbscl__blk439_dn9) + locals.var_vbi_dep_dn9))) / (2.0 * assign24540_e18938)), ((((((((locals.var_c_2esipq_ndepm_dn10 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn10)) * assign24540_e18931) - (assign24540_e18928 * (locals.var_ef_nsubc_dn10 + locals.var_uc_ndepm_dn10))) / (assign24540_e18931 * assign24540_e18931)) * assign24540_e18936) + (assign24540_e18932 * ((-locals.var_vbscl__blk439_dn10) + locals.var_vbi_dep_dn10))) / (2.0 * assign24540_e18938)), ((((((((locals.var_c_2esipq_ndepm_dn11 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn11)) * assign24540_e18931) - (assign24540_e18928 * (locals.var_ef_nsubc_dn11 + locals.var_uc_ndepm_dn11))) / (assign24540_e18931 * assign24540_e18931)) * assign24540_e18936) + (assign24540_e18932 * ((-locals.var_vbscl__blk439_dn11) + locals.var_vbi_dep_dn11))) / (2.0 * assign24540_e18938)), ((((((((locals.var_c_2esipq_ndepm_dn14 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn14)) * assign24540_e18931) - (assign24540_e18928 * (locals.var_ef_nsubc_dn14 + locals.var_uc_ndepm_dn14))) / (assign24540_e18931 * assign24540_e18931)) * assign24540_e18936) + (assign24540_e18932 * ((-locals.var_vbscl__blk439_dn14) + locals.var_vbi_dep_dn14))) / (2.0 * assign24540_e18938)),)
    } else {
        (locals.var_w_bsub0, locals.var_w_bsub0_dn0, locals.var_w_bsub0_dn2, locals.var_w_bsub0_dn4, locals.var_w_bsub0_dn5, locals.var_w_bsub0_dn6, locals.var_w_bsub0_dn7, locals.var_w_bsub0_dn8, locals.var_w_bsub0_dn9, locals.var_w_bsub0_dn10, locals.var_w_bsub0_dn11, locals.var_w_bsub0_dn14,)
    }
};
        locals.var_w_bsub0 = assign24540_e18940;
        locals.var_w_bsub0_dn0 = assign24540_e18940_d_n0;
        locals.var_w_bsub0_dn2 = assign24540_e18940_d_n2;
        locals.var_w_bsub0_dn4 = assign24540_e18940_d_n4;
        locals.var_w_bsub0_dn5 = assign24540_e18940_d_n5;
        locals.var_w_bsub0_dn6 = assign24540_e18940_d_n6;
        locals.var_w_bsub0_dn7 = assign24540_e18940_d_n7;
        locals.var_w_bsub0_dn8 = assign24540_e18940_d_n8;
        locals.var_w_bsub0_dn9 = assign24540_e18940_d_n9;
        locals.var_w_bsub0_dn10 = assign24540_e18940_d_n10;
        locals.var_w_bsub0_dn11 = assign24540_e18940_d_n11;
        locals.var_w_bsub0_dn14 = assign24540_e18940_d_n14;
        locals.var_w_bsub0_rv = 0.0;

        let assign24550_e18943: f64 = if locals.var_w_bsub0 > locals.var_uc_depthn { 1.0 } else { 0.0 };
        locals.var_guard563 = assign24550_e18943;
        locals.var_guard563_rv = 0.0;

        let (assign24560_e18951, assign24560_e18951_d_n0, assign24560_e18951_d_n2, assign24560_e18951_d_n4, assign24560_e18951_d_n5, assign24560_e18951_d_n6, assign24560_e18951_d_n7, assign24560_e18951_d_n8, assign24560_e18951_d_n9, assign24560_e18951_d_n10, assign24560_e18951_d_n11, assign24560_e18951_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgp0, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn11, locals.var_vgp0_dn14,)
    }
};
        locals.var_vgp0 = assign24560_e18951;
        locals.var_vgp0_dn0 = assign24560_e18951_d_n0;
        locals.var_vgp0_dn2 = assign24560_e18951_d_n2;
        locals.var_vgp0_dn4 = assign24560_e18951_d_n4;
        locals.var_vgp0_dn5 = assign24560_e18951_d_n5;
        locals.var_vgp0_dn6 = assign24560_e18951_d_n6;
        locals.var_vgp0_dn7 = assign24560_e18951_d_n7;
        locals.var_vgp0_dn8 = assign24560_e18951_d_n8;
        locals.var_vgp0_dn9 = assign24560_e18951_d_n9;
        locals.var_vgp0_dn10 = assign24560_e18951_d_n10;
        locals.var_vgp0_dn11 = assign24560_e18951_d_n11;
        locals.var_vgp0_dn14 = assign24560_e18951_d_n14;
        locals.var_vgp0_rv = 0.0;

        let (assign24570_e18959, assign24570_e18959_d_n0, assign24570_e18959_d_n2, assign24570_e18959_d_n4, assign24570_e18959_d_n5, assign24570_e18959_d_n6, assign24570_e18959_d_n7, assign24570_e18959_d_n8, assign24570_e18959_d_n9, assign24570_e18959_d_n10, assign24570_e18959_d_n11, assign24570_e18959_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) {
        (locals.var_uc_depthn, locals.var_uc_depthn_dn0, locals.var_uc_depthn_dn2, locals.var_uc_depthn_dn4, locals.var_uc_depthn_dn5, locals.var_uc_depthn_dn6, locals.var_uc_depthn_dn7, locals.var_uc_depthn_dn8, locals.var_uc_depthn_dn9, locals.var_uc_depthn_dn10, locals.var_uc_depthn_dn11, locals.var_uc_depthn_dn14,)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn11, locals.var_w_b0_dn14,)
    }
};
        locals.var_w_b0 = assign24570_e18959;
        locals.var_w_b0_dn0 = assign24570_e18959_d_n0;
        locals.var_w_b0_dn2 = assign24570_e18959_d_n2;
        locals.var_w_b0_dn4 = assign24570_e18959_d_n4;
        locals.var_w_b0_dn5 = assign24570_e18959_d_n5;
        locals.var_w_b0_dn6 = assign24570_e18959_d_n6;
        locals.var_w_b0_dn7 = assign24570_e18959_d_n7;
        locals.var_w_b0_dn8 = assign24570_e18959_d_n8;
        locals.var_w_b0_dn9 = assign24570_e18959_d_n9;
        locals.var_w_b0_dn10 = assign24570_e18959_d_n10;
        locals.var_w_b0_dn11 = assign24570_e18959_d_n11;
        locals.var_w_b0_dn14 = assign24570_e18959_d_n14;
        locals.var_w_b0_rv = 0.0;

        let (assign24580_e18967, assign24580_e18967_d_n0, assign24580_e18967_d_n2, assign24580_e18967_d_n4, assign24580_e18967_d_n5, assign24580_e18967_d_n6, assign24580_e18967_d_n7, assign24580_e18967_d_n8, assign24580_e18967_d_n9, assign24580_e18967_d_n10, assign24580_e18967_d_n11, assign24580_e18967_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn11, locals.var_phi_b0_dep_dn14,)
    }
};
        locals.var_phi_b0_dep = assign24580_e18967;
        locals.var_phi_b0_dep_dn0 = assign24580_e18967_d_n0;
        locals.var_phi_b0_dep_dn2 = assign24580_e18967_d_n2;
        locals.var_phi_b0_dep_dn4 = assign24580_e18967_d_n4;
        locals.var_phi_b0_dep_dn5 = assign24580_e18967_d_n5;
        locals.var_phi_b0_dep_dn6 = assign24580_e18967_d_n6;
        locals.var_phi_b0_dep_dn7 = assign24580_e18967_d_n7;
        locals.var_phi_b0_dep_dn8 = assign24580_e18967_d_n8;
        locals.var_phi_b0_dep_dn9 = assign24580_e18967_d_n9;
        locals.var_phi_b0_dep_dn10 = assign24580_e18967_d_n10;
        locals.var_phi_b0_dep_dn11 = assign24580_e18967_d_n11;
        locals.var_phi_b0_dep_dn14 = assign24580_e18967_d_n14;
        locals.var_phi_b0_dep_rv = 0.0;

        let (assign24590_e18981, assign24590_e18981_d_n0, assign24590_e18981_d_n2, assign24590_e18981_d_n4, assign24590_e18981_d_n5, assign24590_e18981_d_n6, assign24590_e18981_d_n7, assign24590_e18981_d_n8, assign24590_e18981_d_n9, assign24590_e18981_d_n10, assign24590_e18981_d_n11, assign24590_e18981_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) {
        let assign24590_e18976: f64 = (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0);
        let assign24590_e18978: f64 = (assign24590_e18976 * locals.var_w_b0);
        let assign24590_e18979: f64 = (locals.var_phi_b0_dep - assign24590_e18978);
        (assign24590_e18979, (locals.var_phi_b0_dep_dn0 - ((((locals.var_c_2esipq_ndepm_inv_dn0 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn0)) * locals.var_w_b0) + (assign24590_e18976 * locals.var_w_b0_dn0))), (locals.var_phi_b0_dep_dn2 - ((((locals.var_c_2esipq_ndepm_inv_dn2 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn2)) * locals.var_w_b0) + (assign24590_e18976 * locals.var_w_b0_dn2))), (locals.var_phi_b0_dep_dn4 - ((((locals.var_c_2esipq_ndepm_inv_dn4 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn4)) * locals.var_w_b0) + (assign24590_e18976 * locals.var_w_b0_dn4))), (locals.var_phi_b0_dep_dn5 - ((((locals.var_c_2esipq_ndepm_inv_dn5 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn5)) * locals.var_w_b0) + (assign24590_e18976 * locals.var_w_b0_dn5))), (locals.var_phi_b0_dep_dn6 - ((((locals.var_c_2esipq_ndepm_inv_dn6 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn6)) * locals.var_w_b0) + (assign24590_e18976 * locals.var_w_b0_dn6))), (locals.var_phi_b0_dep_dn7 - ((((locals.var_c_2esipq_ndepm_inv_dn7 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn7)) * locals.var_w_b0) + (assign24590_e18976 * locals.var_w_b0_dn7))), (locals.var_phi_b0_dep_dn8 - ((((locals.var_c_2esipq_ndepm_inv_dn8 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn8)) * locals.var_w_b0) + (assign24590_e18976 * locals.var_w_b0_dn8))), (locals.var_phi_b0_dep_dn9 - ((((locals.var_c_2esipq_ndepm_inv_dn9 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn9)) * locals.var_w_b0) + (assign24590_e18976 * locals.var_w_b0_dn9))), (locals.var_phi_b0_dep_dn10 - ((((locals.var_c_2esipq_ndepm_inv_dn10 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn10)) * locals.var_w_b0) + (assign24590_e18976 * locals.var_w_b0_dn10))), (locals.var_phi_b0_dep_dn11 - ((((locals.var_c_2esipq_ndepm_inv_dn11 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn11)) * locals.var_w_b0) + (assign24590_e18976 * locals.var_w_b0_dn11))), (locals.var_phi_b0_dep_dn14 - ((((locals.var_c_2esipq_ndepm_inv_dn14 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn14)) * locals.var_w_b0) + (assign24590_e18976 * locals.var_w_b0_dn14))),)
    } else {
        (locals.var_phi_j0_dep, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn11, locals.var_phi_j0_dep_dn14,)
    }
};
        locals.var_phi_j0_dep = assign24590_e18981;
        locals.var_phi_j0_dep_dn0 = assign24590_e18981_d_n0;
        locals.var_phi_j0_dep_dn2 = assign24590_e18981_d_n2;
        locals.var_phi_j0_dep_dn4 = assign24590_e18981_d_n4;
        locals.var_phi_j0_dep_dn5 = assign24590_e18981_d_n5;
        locals.var_phi_j0_dep_dn6 = assign24590_e18981_d_n6;
        locals.var_phi_j0_dep_dn7 = assign24590_e18981_d_n7;
        locals.var_phi_j0_dep_dn8 = assign24590_e18981_d_n8;
        locals.var_phi_j0_dep_dn9 = assign24590_e18981_d_n9;
        locals.var_phi_j0_dep_dn10 = assign24590_e18981_d_n10;
        locals.var_phi_j0_dep_dn11 = assign24590_e18981_d_n11;
        locals.var_phi_j0_dep_dn14 = assign24590_e18981_d_n14;
        locals.var_phi_j0_dep_rv = 0.0;

        let (assign24600_e18989, assign24600_e18989_d_n0, assign24600_e18989_d_n2, assign24600_e18989_d_n4, assign24600_e18989_d_n5, assign24600_e18989_d_n6, assign24600_e18989_d_n7, assign24600_e18989_d_n8, assign24600_e18989_d_n9, assign24600_e18989_d_n10, assign24600_e18989_d_n11, assign24600_e18989_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vds_maxb0, locals.var_vds_maxb0_dn0, locals.var_vds_maxb0_dn2, locals.var_vds_maxb0_dn4, locals.var_vds_maxb0_dn5, locals.var_vds_maxb0_dn6, locals.var_vds_maxb0_dn7, locals.var_vds_maxb0_dn8, locals.var_vds_maxb0_dn9, locals.var_vds_maxb0_dn10, locals.var_vds_maxb0_dn11, locals.var_vds_maxb0_dn14,)
    }
};
        locals.var_vds_maxb0 = assign24600_e18989;
        locals.var_vds_maxb0_dn0 = assign24600_e18989_d_n0;
        locals.var_vds_maxb0_dn2 = assign24600_e18989_d_n2;
        locals.var_vds_maxb0_dn4 = assign24600_e18989_d_n4;
        locals.var_vds_maxb0_dn5 = assign24600_e18989_d_n5;
        locals.var_vds_maxb0_dn6 = assign24600_e18989_d_n6;
        locals.var_vds_maxb0_dn7 = assign24600_e18989_d_n7;
        locals.var_vds_maxb0_dn8 = assign24600_e18989_d_n8;
        locals.var_vds_maxb0_dn9 = assign24600_e18989_d_n9;
        locals.var_vds_maxb0_dn10 = assign24600_e18989_d_n10;
        locals.var_vds_maxb0_dn11 = assign24600_e18989_d_n11;
        locals.var_vds_maxb0_dn14 = assign24600_e18989_d_n14;
        locals.var_vds_maxb0_rv = 0.0;

        let (assign24610_e18997, assign24610_e18997_d_n0, assign24610_e18997_d_n2, assign24610_e18997_d_n4, assign24610_e18997_d_n5, assign24610_e18997_d_n6, assign24610_e18997_d_n7, assign24610_e18997_d_n8, assign24610_e18997_d_n9, assign24610_e18997_d_n10, assign24610_e18997_d_n11, assign24610_e18997_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) {
        (locals.var_vgp0, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn11, locals.var_vgp0_dn14,)
    } else {
        (locals.var_vgp0old, locals.var_vgp0old_dn0, locals.var_vgp0old_dn2, locals.var_vgp0old_dn4, locals.var_vgp0old_dn5, locals.var_vgp0old_dn6, locals.var_vgp0old_dn7, locals.var_vgp0old_dn8, locals.var_vgp0old_dn9, locals.var_vgp0old_dn10, locals.var_vgp0old_dn11, locals.var_vgp0old_dn14,)
    }
};
        locals.var_vgp0old = assign24610_e18997;
        locals.var_vgp0old_dn0 = assign24610_e18997_d_n0;
        locals.var_vgp0old_dn2 = assign24610_e18997_d_n2;
        locals.var_vgp0old_dn4 = assign24610_e18997_d_n4;
        locals.var_vgp0old_dn5 = assign24610_e18997_d_n5;
        locals.var_vgp0old_dn6 = assign24610_e18997_d_n6;
        locals.var_vgp0old_dn7 = assign24610_e18997_d_n7;
        locals.var_vgp0old_dn8 = assign24610_e18997_d_n8;
        locals.var_vgp0old_dn9 = assign24610_e18997_d_n9;
        locals.var_vgp0old_dn10 = assign24610_e18997_d_n10;
        locals.var_vgp0old_dn11 = assign24610_e18997_d_n11;
        locals.var_vgp0old_dn14 = assign24610_e18997_d_n14;
        locals.var_vgp0old_rv = 0.0;

        let (assign24620_e19005, assign24620_e19005_d_n0, assign24620_e19005_d_n2, assign24620_e19005_d_n4, assign24620_e19005_d_n5, assign24620_e19005_d_n6, assign24620_e19005_d_n7, assign24620_e19005_d_n8, assign24620_e19005_d_n9, assign24620_e19005_d_n10, assign24620_e19005_d_n11, assign24620_e19005_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) {
        (locals.var_phi_j0_dep, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn11, locals.var_phi_j0_dep_dn14,)
    } else {
        (locals.var_phi_j0_dep_old, locals.var_phi_j0_dep_old_dn0, locals.var_phi_j0_dep_old_dn2, locals.var_phi_j0_dep_old_dn4, locals.var_phi_j0_dep_old_dn5, locals.var_phi_j0_dep_old_dn6, locals.var_phi_j0_dep_old_dn7, locals.var_phi_j0_dep_old_dn8, locals.var_phi_j0_dep_old_dn9, locals.var_phi_j0_dep_old_dn10, locals.var_phi_j0_dep_old_dn11, locals.var_phi_j0_dep_old_dn14,)
    }
};
        locals.var_phi_j0_dep_old = assign24620_e19005;
        locals.var_phi_j0_dep_old_dn0 = assign24620_e19005_d_n0;
        locals.var_phi_j0_dep_old_dn2 = assign24620_e19005_d_n2;
        locals.var_phi_j0_dep_old_dn4 = assign24620_e19005_d_n4;
        locals.var_phi_j0_dep_old_dn5 = assign24620_e19005_d_n5;
        locals.var_phi_j0_dep_old_dn6 = assign24620_e19005_d_n6;
        locals.var_phi_j0_dep_old_dn7 = assign24620_e19005_d_n7;
        locals.var_phi_j0_dep_old_dn8 = assign24620_e19005_d_n8;
        locals.var_phi_j0_dep_old_dn9 = assign24620_e19005_d_n9;
        locals.var_phi_j0_dep_old_dn10 = assign24620_e19005_d_n10;
        locals.var_phi_j0_dep_old_dn11 = assign24620_e19005_d_n11;
        locals.var_phi_j0_dep_old_dn14 = assign24620_e19005_d_n14;
        locals.var_phi_j0_dep_old_rv = 0.0;

        let (assign24630_e19013,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign24630_e19013;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_69(
        locals: &mut StampLocals,
    ) {
        let mut assign24640_loop_guard: usize = 0;
        while {
            let assign24640_cond_e19022: f64 = (150.0 + 1.0);
            let assign24640_cond_e19024: f64 = if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_lp_s0 <= assign24640_cond_e19022)) { 1.0 } else { 0.0 };
            assign24640_cond_e19024 != 0.0
        } {
            assign24640_loop_guard += 1;
            assert!(assign24640_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign24640_body0_e19037, assign24640_body0_e19037_d_n0, assign24640_body0_e19037_d_n2, assign24640_body0_e19037_d_n4, assign24640_body0_e19037_d_n5, assign24640_body0_e19037_d_n6, assign24640_body0_e19037_d_n7, assign24640_body0_e19037_d_n8, assign24640_body0_e19037_d_n9, assign24640_body0_e19037_d_n10, assign24640_body0_e19037_d_n11, assign24640_body0_e19037_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) {
        let assign24640_body0_e19033: f64 = (locals.var_phi_b0_dep - locals.var_phi_j0_dep);
        let assign24640_body0_e19034: f64 = (locals.var_c_2esipq_ndepm * assign24640_body0_e19033);
        let assign24640_body0_e19035: f64 = (assign24640_body0_e19034).sqrt();
        (assign24640_body0_e19035, (((locals.var_c_2esipq_ndepm_dn0 * assign24640_body0_e19033) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn0 - locals.var_phi_j0_dep_dn0))) / (2.0 * assign24640_body0_e19035)), (((locals.var_c_2esipq_ndepm_dn2 * assign24640_body0_e19033) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn2 - locals.var_phi_j0_dep_dn2))) / (2.0 * assign24640_body0_e19035)), (((locals.var_c_2esipq_ndepm_dn4 * assign24640_body0_e19033) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn4 - locals.var_phi_j0_dep_dn4))) / (2.0 * assign24640_body0_e19035)), (((locals.var_c_2esipq_ndepm_dn5 * assign24640_body0_e19033) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn5 - locals.var_phi_j0_dep_dn5))) / (2.0 * assign24640_body0_e19035)), (((locals.var_c_2esipq_ndepm_dn6 * assign24640_body0_e19033) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn6 - locals.var_phi_j0_dep_dn6))) / (2.0 * assign24640_body0_e19035)), (((locals.var_c_2esipq_ndepm_dn7 * assign24640_body0_e19033) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn7 - locals.var_phi_j0_dep_dn7))) / (2.0 * assign24640_body0_e19035)), (((locals.var_c_2esipq_ndepm_dn8 * assign24640_body0_e19033) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn8 - locals.var_phi_j0_dep_dn8))) / (2.0 * assign24640_body0_e19035)), (((locals.var_c_2esipq_ndepm_dn9 * assign24640_body0_e19033) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn9 - locals.var_phi_j0_dep_dn9))) / (2.0 * assign24640_body0_e19035)), (((locals.var_c_2esipq_ndepm_dn10 * assign24640_body0_e19033) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn10 - locals.var_phi_j0_dep_dn10))) / (2.0 * assign24640_body0_e19035)), (((locals.var_c_2esipq_ndepm_dn11 * assign24640_body0_e19033) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn11 - locals.var_phi_j0_dep_dn11))) / (2.0 * assign24640_body0_e19035)), (((locals.var_c_2esipq_ndepm_dn14 * assign24640_body0_e19033) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn14 - locals.var_phi_j0_dep_dn14))) / (2.0 * assign24640_body0_e19035)),)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn11, locals.var_w_b0_dn14,)
    }
};
            locals.var_w_b0 = assign24640_body0_e19037;
            locals.var_w_b0_dn0 = assign24640_body0_e19037_d_n0;
            locals.var_w_b0_dn2 = assign24640_body0_e19037_d_n2;
            locals.var_w_b0_dn4 = assign24640_body0_e19037_d_n4;
            locals.var_w_b0_dn5 = assign24640_body0_e19037_d_n5;
            locals.var_w_b0_dn6 = assign24640_body0_e19037_d_n6;
            locals.var_w_b0_dn7 = assign24640_body0_e19037_d_n7;
            locals.var_w_b0_dn8 = assign24640_body0_e19037_d_n8;
            locals.var_w_b0_dn9 = assign24640_body0_e19037_d_n9;
            locals.var_w_b0_dn10 = assign24640_body0_e19037_d_n10;
            locals.var_w_b0_dn11 = assign24640_body0_e19037_d_n11;
            locals.var_w_b0_dn14 = assign24640_body0_e19037_d_n14;
            locals.var_w_b0_rv = 0.0;
            let assign24640_body1_e19041: f64 = (locals.var_uc_depthn - 1e-8);
            let assign24640_body1_e19046: f64 = if ((locals.var_w_b0 > assign24640_body1_e19041) && (1e-8 >= 0.0)) { 1.0 } else { 0.0 };
            locals.var_guard564 = assign24640_body1_e19046;
            locals.var_guard564_rv = 0.0;
            let (assign24640_body2_e19060, assign24640_body2_e19060_d_n0, assign24640_body2_e19060_d_n2, assign24640_body2_e19060_d_n4, assign24640_body2_e19060_d_n5, assign24640_body2_e19060_d_n6, assign24640_body2_e19060_d_n7, assign24640_body2_e19060_d_n8, assign24640_body2_e19060_d_n9, assign24640_body2_e19060_d_n10, assign24640_body2_e19060_d_n11, assign24640_body2_e19060_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) {
        let assign24640_body2_e19056: f64 = (locals.var_w_b0 - locals.var_uc_depthn);
        let assign24640_body2_e19058: f64 = (assign24640_body2_e19056 + 1e-8);
        (assign24640_body2_e19058, (locals.var_w_b0_dn0 - locals.var_uc_depthn_dn0), (locals.var_w_b0_dn2 - locals.var_uc_depthn_dn2), (locals.var_w_b0_dn4 - locals.var_uc_depthn_dn4), (locals.var_w_b0_dn5 - locals.var_uc_depthn_dn5), (locals.var_w_b0_dn6 - locals.var_uc_depthn_dn6), (locals.var_w_b0_dn7 - locals.var_uc_depthn_dn7), (locals.var_w_b0_dn8 - locals.var_uc_depthn_dn8), (locals.var_w_b0_dn9 - locals.var_uc_depthn_dn9), (locals.var_w_b0_dn10 - locals.var_uc_depthn_dn10), (locals.var_w_b0_dn11 - locals.var_uc_depthn_dn11), (locals.var_w_b0_dn14 - locals.var_uc_depthn_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
            locals.var_tmf1 = assign24640_body2_e19060;
            locals.var_tmf1_dn0 = assign24640_body2_e19060_d_n0;
            locals.var_tmf1_dn2 = assign24640_body2_e19060_d_n2;
            locals.var_tmf1_dn4 = assign24640_body2_e19060_d_n4;
            locals.var_tmf1_dn5 = assign24640_body2_e19060_d_n5;
            locals.var_tmf1_dn6 = assign24640_body2_e19060_d_n6;
            locals.var_tmf1_dn7 = assign24640_body2_e19060_d_n7;
            locals.var_tmf1_dn8 = assign24640_body2_e19060_d_n8;
            locals.var_tmf1_dn9 = assign24640_body2_e19060_d_n9;
            locals.var_tmf1_dn10 = assign24640_body2_e19060_d_n10;
            locals.var_tmf1_dn11 = assign24640_body2_e19060_d_n11;
            locals.var_tmf1_dn14 = assign24640_body2_e19060_d_n14;
            locals.var_tmf1_rv = 0.0;
            let (assign24640_body3_e19072, assign24640_body3_e19072_d_n0, assign24640_body3_e19072_d_n2, assign24640_body3_e19072_d_n4, assign24640_body3_e19072_d_n5, assign24640_body3_e19072_d_n6, assign24640_body3_e19072_d_n7, assign24640_body3_e19072_d_n8, assign24640_body3_e19072_d_n9, assign24640_body3_e19072_d_n10, assign24640_body3_e19072_d_n11, assign24640_body3_e19072_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) {
        let assign24640_body3_e19070: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign24640_body3_e19070, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
            locals.var_x2 = assign24640_body3_e19072;
            locals.var_x2_dn0 = assign24640_body3_e19072_d_n0;
            locals.var_x2_dn2 = assign24640_body3_e19072_d_n2;
            locals.var_x2_dn4 = assign24640_body3_e19072_d_n4;
            locals.var_x2_dn5 = assign24640_body3_e19072_d_n5;
            locals.var_x2_dn6 = assign24640_body3_e19072_d_n6;
            locals.var_x2_dn7 = assign24640_body3_e19072_d_n7;
            locals.var_x2_dn8 = assign24640_body3_e19072_d_n8;
            locals.var_x2_dn9 = assign24640_body3_e19072_d_n9;
            locals.var_x2_dn10 = assign24640_body3_e19072_d_n10;
            locals.var_x2_dn11 = assign24640_body3_e19072_d_n11;
            locals.var_x2_dn14 = assign24640_body3_e19072_d_n14;
            locals.var_x2_rv = 0.0;
            let (assign24640_body4_e19084, assign24640_body4_e19084_d_n0, assign24640_body4_e19084_d_n2, assign24640_body4_e19084_d_n4, assign24640_body4_e19084_d_n5, assign24640_body4_e19084_d_n6, assign24640_body4_e19084_d_n7, assign24640_body4_e19084_d_n8, assign24640_body4_e19084_d_n9, assign24640_body4_e19084_d_n10, assign24640_body4_e19084_d_n11, assign24640_body4_e19084_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) {
        let assign24640_body4_e19082: f64 = (1e-8 * 1e-8);
        (assign24640_body4_e19082, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
            locals.var_xmax2 = assign24640_body4_e19084;
            locals.var_xmax2_dn0 = assign24640_body4_e19084_d_n0;
            locals.var_xmax2_dn2 = assign24640_body4_e19084_d_n2;
            locals.var_xmax2_dn4 = assign24640_body4_e19084_d_n4;
            locals.var_xmax2_dn5 = assign24640_body4_e19084_d_n5;
            locals.var_xmax2_dn6 = assign24640_body4_e19084_d_n6;
            locals.var_xmax2_dn7 = assign24640_body4_e19084_d_n7;
            locals.var_xmax2_dn8 = assign24640_body4_e19084_d_n8;
            locals.var_xmax2_dn9 = assign24640_body4_e19084_d_n9;
            locals.var_xmax2_dn10 = assign24640_body4_e19084_d_n10;
            locals.var_xmax2_dn11 = assign24640_body4_e19084_d_n11;
            locals.var_xmax2_dn14 = assign24640_body4_e19084_d_n14;
            locals.var_xmax2_rv = 0.0;
            let (assign24640_body5_e19094, assign24640_body5_e19094_d_n0, assign24640_body5_e19094_d_n2, assign24640_body5_e19094_d_n4, assign24640_body5_e19094_d_n5, assign24640_body5_e19094_d_n6, assign24640_body5_e19094_d_n7, assign24640_body5_e19094_d_n8, assign24640_body5_e19094_d_n9, assign24640_body5_e19094_d_n10, assign24640_body5_e19094_d_n11, assign24640_body5_e19094_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
            locals.var_xp = assign24640_body5_e19094;
            locals.var_xp_dn0 = assign24640_body5_e19094_d_n0;
            locals.var_xp_dn2 = assign24640_body5_e19094_d_n2;
            locals.var_xp_dn4 = assign24640_body5_e19094_d_n4;
            locals.var_xp_dn5 = assign24640_body5_e19094_d_n5;
            locals.var_xp_dn6 = assign24640_body5_e19094_d_n6;
            locals.var_xp_dn7 = assign24640_body5_e19094_d_n7;
            locals.var_xp_dn8 = assign24640_body5_e19094_d_n8;
            locals.var_xp_dn9 = assign24640_body5_e19094_d_n9;
            locals.var_xp_dn10 = assign24640_body5_e19094_d_n10;
            locals.var_xp_dn11 = assign24640_body5_e19094_d_n11;
            locals.var_xp_dn14 = assign24640_body5_e19094_d_n14;
            locals.var_xp_rv = 0.0;
            let (assign24640_body6_e19104, assign24640_body6_e19104_d_n0, assign24640_body6_e19104_d_n2, assign24640_body6_e19104_d_n4, assign24640_body6_e19104_d_n5, assign24640_body6_e19104_d_n6, assign24640_body6_e19104_d_n7, assign24640_body6_e19104_d_n8, assign24640_body6_e19104_d_n9, assign24640_body6_e19104_d_n10, assign24640_body6_e19104_d_n11, assign24640_body6_e19104_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
            locals.var_xmp = assign24640_body6_e19104;
            locals.var_xmp_dn0 = assign24640_body6_e19104_d_n0;
            locals.var_xmp_dn2 = assign24640_body6_e19104_d_n2;
            locals.var_xmp_dn4 = assign24640_body6_e19104_d_n4;
            locals.var_xmp_dn5 = assign24640_body6_e19104_d_n5;
            locals.var_xmp_dn6 = assign24640_body6_e19104_d_n6;
            locals.var_xmp_dn7 = assign24640_body6_e19104_d_n7;
            locals.var_xmp_dn8 = assign24640_body6_e19104_d_n8;
            locals.var_xmp_dn9 = assign24640_body6_e19104_d_n9;
            locals.var_xmp_dn10 = assign24640_body6_e19104_d_n10;
            locals.var_xmp_dn11 = assign24640_body6_e19104_d_n11;
            locals.var_xmp_dn14 = assign24640_body6_e19104_d_n14;
            locals.var_xmp_rv = 0.0;
            let (assign24640_body7_e19114,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign24640_body7_e19114;
            locals.var_m0_rv = 0.0;
            let (assign24640_body8_e19124,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign24640_body8_e19124;
            locals.var_mm_rv = 0.0;
            let (assign24640_body9_e19134, assign24640_body9_e19134_d_n0, assign24640_body9_e19134_d_n2, assign24640_body9_e19134_d_n4, assign24640_body9_e19134_d_n5, assign24640_body9_e19134_d_n6, assign24640_body9_e19134_d_n7, assign24640_body9_e19134_d_n8, assign24640_body9_e19134_d_n9, assign24640_body9_e19134_d_n10, assign24640_body9_e19134_d_n11, assign24640_body9_e19134_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
            locals.var_arg = assign24640_body9_e19134;
            locals.var_arg_dn0 = assign24640_body9_e19134_d_n0;
            locals.var_arg_dn2 = assign24640_body9_e19134_d_n2;
            locals.var_arg_dn4 = assign24640_body9_e19134_d_n4;
            locals.var_arg_dn5 = assign24640_body9_e19134_d_n5;
            locals.var_arg_dn6 = assign24640_body9_e19134_d_n6;
            locals.var_arg_dn7 = assign24640_body9_e19134_d_n7;
            locals.var_arg_dn8 = assign24640_body9_e19134_d_n8;
            locals.var_arg_dn9 = assign24640_body9_e19134_d_n9;
            locals.var_arg_dn10 = assign24640_body9_e19134_d_n10;
            locals.var_arg_dn11 = assign24640_body9_e19134_d_n11;
            locals.var_arg_dn14 = assign24640_body9_e19134_d_n14;
            locals.var_arg_rv = 0.0;
            let (assign24640_body10_e19144, assign24640_body10_e19144_d_n0, assign24640_body10_e19144_d_n2, assign24640_body10_e19144_d_n4, assign24640_body10_e19144_d_n5, assign24640_body10_e19144_d_n6, assign24640_body10_e19144_d_n7, assign24640_body10_e19144_d_n8, assign24640_body10_e19144_d_n9, assign24640_body10_e19144_d_n10, assign24640_body10_e19144_d_n11, assign24640_body10_e19144_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign24640_body10_e19144;
            locals.var_dnm_dn0 = assign24640_body10_e19144_d_n0;
            locals.var_dnm_dn2 = assign24640_body10_e19144_d_n2;
            locals.var_dnm_dn4 = assign24640_body10_e19144_d_n4;
            locals.var_dnm_dn5 = assign24640_body10_e19144_d_n5;
            locals.var_dnm_dn6 = assign24640_body10_e19144_d_n6;
            locals.var_dnm_dn7 = assign24640_body10_e19144_d_n7;
            locals.var_dnm_dn8 = assign24640_body10_e19144_d_n8;
            locals.var_dnm_dn9 = assign24640_body10_e19144_d_n9;
            locals.var_dnm_dn10 = assign24640_body10_e19144_d_n10;
            locals.var_dnm_dn11 = assign24640_body10_e19144_d_n11;
            locals.var_dnm_dn14 = assign24640_body10_e19144_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign24640_body11_e19156, assign24640_body11_e19156_d_n0, assign24640_body11_e19156_d_n2, assign24640_body11_e19156_d_n4, assign24640_body11_e19156_d_n5, assign24640_body11_e19156_d_n6, assign24640_body11_e19156_d_n7, assign24640_body11_e19156_d_n8, assign24640_body11_e19156_d_n9, assign24640_body11_e19156_d_n10, assign24640_body11_e19156_d_n11, assign24640_body11_e19156_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) {
        let assign24640_body11_e19154: f64 = (locals.var_xp * locals.var_x2);
        (assign24640_body11_e19154, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
            locals.var_xp = assign24640_body11_e19156;
            locals.var_xp_dn0 = assign24640_body11_e19156_d_n0;
            locals.var_xp_dn2 = assign24640_body11_e19156_d_n2;
            locals.var_xp_dn4 = assign24640_body11_e19156_d_n4;
            locals.var_xp_dn5 = assign24640_body11_e19156_d_n5;
            locals.var_xp_dn6 = assign24640_body11_e19156_d_n6;
            locals.var_xp_dn7 = assign24640_body11_e19156_d_n7;
            locals.var_xp_dn8 = assign24640_body11_e19156_d_n8;
            locals.var_xp_dn9 = assign24640_body11_e19156_d_n9;
            locals.var_xp_dn10 = assign24640_body11_e19156_d_n10;
            locals.var_xp_dn11 = assign24640_body11_e19156_d_n11;
            locals.var_xp_dn14 = assign24640_body11_e19156_d_n14;
            locals.var_xp_rv = 0.0;
            let (assign24640_body12_e19168, assign24640_body12_e19168_d_n0, assign24640_body12_e19168_d_n2, assign24640_body12_e19168_d_n4, assign24640_body12_e19168_d_n5, assign24640_body12_e19168_d_n6, assign24640_body12_e19168_d_n7, assign24640_body12_e19168_d_n8, assign24640_body12_e19168_d_n9, assign24640_body12_e19168_d_n10, assign24640_body12_e19168_d_n11, assign24640_body12_e19168_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) {
        let assign24640_body12_e19166: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign24640_body12_e19166, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
            locals.var_xmp = assign24640_body12_e19168;
            locals.var_xmp_dn0 = assign24640_body12_e19168_d_n0;
            locals.var_xmp_dn2 = assign24640_body12_e19168_d_n2;
            locals.var_xmp_dn4 = assign24640_body12_e19168_d_n4;
            locals.var_xmp_dn5 = assign24640_body12_e19168_d_n5;
            locals.var_xmp_dn6 = assign24640_body12_e19168_d_n6;
            locals.var_xmp_dn7 = assign24640_body12_e19168_d_n7;
            locals.var_xmp_dn8 = assign24640_body12_e19168_d_n8;
            locals.var_xmp_dn9 = assign24640_body12_e19168_d_n9;
            locals.var_xmp_dn10 = assign24640_body12_e19168_d_n10;
            locals.var_xmp_dn11 = assign24640_body12_e19168_d_n11;
            locals.var_xmp_dn14 = assign24640_body12_e19168_d_n14;
            locals.var_xmp_rv = 0.0;
            let (assign24640_body13_e19180, assign24640_body13_e19180_d_n0, assign24640_body13_e19180_d_n2, assign24640_body13_e19180_d_n4, assign24640_body13_e19180_d_n5, assign24640_body13_e19180_d_n6, assign24640_body13_e19180_d_n7, assign24640_body13_e19180_d_n8, assign24640_body13_e19180_d_n9, assign24640_body13_e19180_d_n10, assign24640_body13_e19180_d_n11, assign24640_body13_e19180_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) {
        let assign24640_body13_e19178: f64 = (locals.var_xp * locals.var_x2);
        (assign24640_body13_e19178, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
            locals.var_xp = assign24640_body13_e19180;
            locals.var_xp_dn0 = assign24640_body13_e19180_d_n0;
            locals.var_xp_dn2 = assign24640_body13_e19180_d_n2;
            locals.var_xp_dn4 = assign24640_body13_e19180_d_n4;
            locals.var_xp_dn5 = assign24640_body13_e19180_d_n5;
            locals.var_xp_dn6 = assign24640_body13_e19180_d_n6;
            locals.var_xp_dn7 = assign24640_body13_e19180_d_n7;
            locals.var_xp_dn8 = assign24640_body13_e19180_d_n8;
            locals.var_xp_dn9 = assign24640_body13_e19180_d_n9;
            locals.var_xp_dn10 = assign24640_body13_e19180_d_n10;
            locals.var_xp_dn11 = assign24640_body13_e19180_d_n11;
            locals.var_xp_dn14 = assign24640_body13_e19180_d_n14;
            locals.var_xp_rv = 0.0;
            let (assign24640_body14_e19192, assign24640_body14_e19192_d_n0, assign24640_body14_e19192_d_n2, assign24640_body14_e19192_d_n4, assign24640_body14_e19192_d_n5, assign24640_body14_e19192_d_n6, assign24640_body14_e19192_d_n7, assign24640_body14_e19192_d_n8, assign24640_body14_e19192_d_n9, assign24640_body14_e19192_d_n10, assign24640_body14_e19192_d_n11, assign24640_body14_e19192_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) {
        let assign24640_body14_e19190: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign24640_body14_e19190, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
            locals.var_xmp = assign24640_body14_e19192;
            locals.var_xmp_dn0 = assign24640_body14_e19192_d_n0;
            locals.var_xmp_dn2 = assign24640_body14_e19192_d_n2;
            locals.var_xmp_dn4 = assign24640_body14_e19192_d_n4;
            locals.var_xmp_dn5 = assign24640_body14_e19192_d_n5;
            locals.var_xmp_dn6 = assign24640_body14_e19192_d_n6;
            locals.var_xmp_dn7 = assign24640_body14_e19192_d_n7;
            locals.var_xmp_dn8 = assign24640_body14_e19192_d_n8;
            locals.var_xmp_dn9 = assign24640_body14_e19192_d_n9;
            locals.var_xmp_dn10 = assign24640_body14_e19192_d_n10;
            locals.var_xmp_dn11 = assign24640_body14_e19192_d_n11;
            locals.var_xmp_dn14 = assign24640_body14_e19192_d_n14;
            locals.var_xmp_rv = 0.0;
            let (assign24640_body15_e19204, assign24640_body15_e19204_d_n0, assign24640_body15_e19204_d_n2, assign24640_body15_e19204_d_n4, assign24640_body15_e19204_d_n5, assign24640_body15_e19204_d_n6, assign24640_body15_e19204_d_n7, assign24640_body15_e19204_d_n8, assign24640_body15_e19204_d_n9, assign24640_body15_e19204_d_n10, assign24640_body15_e19204_d_n11, assign24640_body15_e19204_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) {
        let assign24640_body15_e19202: f64 = (locals.var_xp + locals.var_xmp);
        (assign24640_body15_e19202, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
            locals.var_arg = assign24640_body15_e19204;
            locals.var_arg_dn0 = assign24640_body15_e19204_d_n0;
            locals.var_arg_dn2 = assign24640_body15_e19204_d_n2;
            locals.var_arg_dn4 = assign24640_body15_e19204_d_n4;
            locals.var_arg_dn5 = assign24640_body15_e19204_d_n5;
            locals.var_arg_dn6 = assign24640_body15_e19204_d_n6;
            locals.var_arg_dn7 = assign24640_body15_e19204_d_n7;
            locals.var_arg_dn8 = assign24640_body15_e19204_d_n8;
            locals.var_arg_dn9 = assign24640_body15_e19204_d_n9;
            locals.var_arg_dn10 = assign24640_body15_e19204_d_n10;
            locals.var_arg_dn11 = assign24640_body15_e19204_d_n11;
            locals.var_arg_dn14 = assign24640_body15_e19204_d_n14;
            locals.var_arg_rv = 0.0;
            let (assign24640_body16_e19214, assign24640_body16_e19214_d_n0, assign24640_body16_e19214_d_n2, assign24640_body16_e19214_d_n4, assign24640_body16_e19214_d_n5, assign24640_body16_e19214_d_n6, assign24640_body16_e19214_d_n7, assign24640_body16_e19214_d_n8, assign24640_body16_e19214_d_n9, assign24640_body16_e19214_d_n10, assign24640_body16_e19214_d_n11, assign24640_body16_e19214_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign24640_body16_e19214;
            locals.var_dnm_dn0 = assign24640_body16_e19214_d_n0;
            locals.var_dnm_dn2 = assign24640_body16_e19214_d_n2;
            locals.var_dnm_dn4 = assign24640_body16_e19214_d_n4;
            locals.var_dnm_dn5 = assign24640_body16_e19214_d_n5;
            locals.var_dnm_dn6 = assign24640_body16_e19214_d_n6;
            locals.var_dnm_dn7 = assign24640_body16_e19214_d_n7;
            locals.var_dnm_dn8 = assign24640_body16_e19214_d_n8;
            locals.var_dnm_dn9 = assign24640_body16_e19214_d_n9;
            locals.var_dnm_dn10 = assign24640_body16_e19214_d_n10;
            locals.var_dnm_dn11 = assign24640_body16_e19214_d_n11;
            locals.var_dnm_dn14 = assign24640_body16_e19214_d_n14;
            locals.var_dnm_rv = 0.0;
            let assign24640_body17_e19229: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
            locals.var_guard565 = assign24640_body17_e19229;
            locals.var_guard565_rv = 0.0;
            let assign24640_body18_e19232: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard566 = assign24640_body18_e19232;
            locals.var_guard566_rv = 0.0;
            let (assign24640_body19_e19246,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) && (locals.var_guard565 != 0.0)) && (locals.var_guard566 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign24640_body19_e19246;
            locals.var_mm_rv = 0.0;
            let assign24640_body20_e19249: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
            locals.var_guard567 = assign24640_body20_e19249;
            locals.var_guard567_rv = 0.0;
            let (assign24640_body21_e19266,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) && (locals.var_guard565 != 0.0)) && (locals.var_guard566 == 0.0)) && (locals.var_guard567 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign24640_body21_e19266;
            locals.var_mm_rv = 0.0;
            let assign24640_body22_e19269: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
            locals.var_guard568 = assign24640_body22_e19269;
            locals.var_guard568_rv = 0.0;
            let (assign24640_body23_e19289,) = {
    if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) && (locals.var_guard565 != 0.0)) && (locals.var_guard566 == 0.0)) && (locals.var_guard567 == 0.0)) && (locals.var_guard568 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign24640_body23_e19289;
            locals.var_mm_rv = 0.0;
            let assign24640_body24_e19292: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
            locals.var_guard569 = assign24640_body24_e19292;
            locals.var_guard569_rv = 0.0;
            let (assign24640_body25_e19315,) = {
    if (((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) && (locals.var_guard565 != 0.0)) && (locals.var_guard566 == 0.0)) && (locals.var_guard567 == 0.0)) && (locals.var_guard568 == 0.0)) && (locals.var_guard569 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign24640_body25_e19315;
            locals.var_mm_rv = 0.0;
            let (assign24640_body26_e19327,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) && (locals.var_guard565 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign24640_body26_e19327;
            locals.var_m0_rv = 0.0;
            let mut assign24640_body27_loop_guard: usize = 0;
            while {
                let assign24640_body27_cond_e19340: f64 = if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) && (locals.var_guard565 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
                assign24640_body27_cond_e19340 != 0.0
            } {
                assign24640_body27_loop_guard += 1;
                assert!(assign24640_body27_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                let (assign24640_body27_body0_e19353, assign24640_body27_body0_e19353_d_n0, assign24640_body27_body0_e19353_d_n2, assign24640_body27_body0_e19353_d_n4, assign24640_body27_body0_e19353_d_n5, assign24640_body27_body0_e19353_d_n6, assign24640_body27_body0_e19353_d_n7, assign24640_body27_body0_e19353_d_n8, assign24640_body27_body0_e19353_d_n9, assign24640_body27_body0_e19353_d_n10, assign24640_body27_body0_e19353_d_n11, assign24640_body27_body0_e19353_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) && (locals.var_guard565 != 0.0)) {
        let assign24640_body27_body0_e19351: f64 = (locals.var_dnm).sqrt();
        (assign24640_body27_body0_e19351, (locals.var_dnm_dn0 / (2.0 * assign24640_body27_body0_e19351)), (locals.var_dnm_dn2 / (2.0 * assign24640_body27_body0_e19351)), (locals.var_dnm_dn4 / (2.0 * assign24640_body27_body0_e19351)), (locals.var_dnm_dn5 / (2.0 * assign24640_body27_body0_e19351)), (locals.var_dnm_dn6 / (2.0 * assign24640_body27_body0_e19351)), (locals.var_dnm_dn7 / (2.0 * assign24640_body27_body0_e19351)), (locals.var_dnm_dn8 / (2.0 * assign24640_body27_body0_e19351)), (locals.var_dnm_dn9 / (2.0 * assign24640_body27_body0_e19351)), (locals.var_dnm_dn10 / (2.0 * assign24640_body27_body0_e19351)), (locals.var_dnm_dn11 / (2.0 * assign24640_body27_body0_e19351)), (locals.var_dnm_dn14 / (2.0 * assign24640_body27_body0_e19351)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
                locals.var_dnm = assign24640_body27_body0_e19353;
                locals.var_dnm_dn0 = assign24640_body27_body0_e19353_d_n0;
                locals.var_dnm_dn2 = assign24640_body27_body0_e19353_d_n2;
                locals.var_dnm_dn4 = assign24640_body27_body0_e19353_d_n4;
                locals.var_dnm_dn5 = assign24640_body27_body0_e19353_d_n5;
                locals.var_dnm_dn6 = assign24640_body27_body0_e19353_d_n6;
                locals.var_dnm_dn7 = assign24640_body27_body0_e19353_d_n7;
                locals.var_dnm_dn8 = assign24640_body27_body0_e19353_d_n8;
                locals.var_dnm_dn9 = assign24640_body27_body0_e19353_d_n9;
                locals.var_dnm_dn10 = assign24640_body27_body0_e19353_d_n10;
                locals.var_dnm_dn11 = assign24640_body27_body0_e19353_d_n11;
                locals.var_dnm_dn14 = assign24640_body27_body0_e19353_d_n14;
                locals.var_dnm_rv = 0.0;
                let (assign24640_body27_body1_e19367,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) && (locals.var_guard565 != 0.0)) {
        let assign24640_body27_body1_e19365: f64 = (locals.var_m0 + 1.0);
        (assign24640_body27_body1_e19365,)
    } else {
        (locals.var_m0,)
    }
};
                locals.var_m0 = assign24640_body27_body1_e19367;
                locals.var_m0_rv = 0.0;
            }
            let (assign24640_body28_e19391, assign24640_body28_e19391_d_n0, assign24640_body28_e19391_d_n2, assign24640_body28_e19391_d_n4, assign24640_body28_e19391_d_n5, assign24640_body28_e19391_d_n6, assign24640_body28_e19391_d_n7, assign24640_body28_e19391_d_n8, assign24640_body28_e19391_d_n9, assign24640_body28_e19391_d_n10, assign24640_body28_e19391_d_n11, assign24640_body28_e19391_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) && (locals.var_guard565 == 0.0)) {
        let (assign24640_body28_e19389, assign24640_body28_e19389_d_n0, assign24640_body28_e19389_d_n2, assign24640_body28_e19389_d_n4, assign24640_body28_e19389_d_n5, assign24640_body28_e19389_d_n6, assign24640_body28_e19389_d_n7, assign24640_body28_e19389_d_n8, assign24640_body28_e19389_d_n9, assign24640_body28_e19389_d_n10, assign24640_body28_e19389_d_n11, assign24640_body28_e19389_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign24640_body28_e19386: f64 = (2.0 * 2.0);
                let assign24640_body28_e19387: f64 = (1.0 / assign24640_body28_e19386);
                let assign24640_body28_e19388: f64 = (locals.var_dnm).powf(assign24640_body28_e19387);
                (assign24640_body28_e19388, if 0.0 == 0.0 && ((assign24640_body28_e19387) as f64).is_finite() && ((assign24640_body28_e19387) as f64).fract() == 0.0 { if assign24640_body28_e19387 == 0.0 { 0.0 } else { (assign24640_body28_e19387 * ((locals.var_dnm).powf(assign24640_body28_e19387 - 1.0) * locals.var_dnm_dn0)) } } else { (assign24640_body28_e19388 * (assign24640_body28_e19387 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24640_body28_e19387) as f64).is_finite() && ((assign24640_body28_e19387) as f64).fract() == 0.0 { if assign24640_body28_e19387 == 0.0 { 0.0 } else { (assign24640_body28_e19387 * ((locals.var_dnm).powf(assign24640_body28_e19387 - 1.0) * locals.var_dnm_dn2)) } } else { (assign24640_body28_e19388 * (assign24640_body28_e19387 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24640_body28_e19387) as f64).is_finite() && ((assign24640_body28_e19387) as f64).fract() == 0.0 { if assign24640_body28_e19387 == 0.0 { 0.0 } else { (assign24640_body28_e19387 * ((locals.var_dnm).powf(assign24640_body28_e19387 - 1.0) * locals.var_dnm_dn4)) } } else { (assign24640_body28_e19388 * (assign24640_body28_e19387 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24640_body28_e19387) as f64).is_finite() && ((assign24640_body28_e19387) as f64).fract() == 0.0 { if assign24640_body28_e19387 == 0.0 { 0.0 } else { (assign24640_body28_e19387 * ((locals.var_dnm).powf(assign24640_body28_e19387 - 1.0) * locals.var_dnm_dn5)) } } else { (assign24640_body28_e19388 * (assign24640_body28_e19387 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24640_body28_e19387) as f64).is_finite() && ((assign24640_body28_e19387) as f64).fract() == 0.0 { if assign24640_body28_e19387 == 0.0 { 0.0 } else { (assign24640_body28_e19387 * ((locals.var_dnm).powf(assign24640_body28_e19387 - 1.0) * locals.var_dnm_dn6)) } } else { (assign24640_body28_e19388 * (assign24640_body28_e19387 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24640_body28_e19387) as f64).is_finite() && ((assign24640_body28_e19387) as f64).fract() == 0.0 { if assign24640_body28_e19387 == 0.0 { 0.0 } else { (assign24640_body28_e19387 * ((locals.var_dnm).powf(assign24640_body28_e19387 - 1.0) * locals.var_dnm_dn7)) } } else { (assign24640_body28_e19388 * (assign24640_body28_e19387 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24640_body28_e19387) as f64).is_finite() && ((assign24640_body28_e19387) as f64).fract() == 0.0 { if assign24640_body28_e19387 == 0.0 { 0.0 } else { (assign24640_body28_e19387 * ((locals.var_dnm).powf(assign24640_body28_e19387 - 1.0) * locals.var_dnm_dn8)) } } else { (assign24640_body28_e19388 * (assign24640_body28_e19387 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24640_body28_e19387) as f64).is_finite() && ((assign24640_body28_e19387) as f64).fract() == 0.0 { if assign24640_body28_e19387 == 0.0 { 0.0 } else { (assign24640_body28_e19387 * ((locals.var_dnm).powf(assign24640_body28_e19387 - 1.0) * locals.var_dnm_dn9)) } } else { (assign24640_body28_e19388 * (assign24640_body28_e19387 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24640_body28_e19387) as f64).is_finite() && ((assign24640_body28_e19387) as f64).fract() == 0.0 { if assign24640_body28_e19387 == 0.0 { 0.0 } else { (assign24640_body28_e19387 * ((locals.var_dnm).powf(assign24640_body28_e19387 - 1.0) * locals.var_dnm_dn10)) } } else { (assign24640_body28_e19388 * (assign24640_body28_e19387 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24640_body28_e19387) as f64).is_finite() && ((assign24640_body28_e19387) as f64).fract() == 0.0 { if assign24640_body28_e19387 == 0.0 { 0.0 } else { (assign24640_body28_e19387 * ((locals.var_dnm).powf(assign24640_body28_e19387 - 1.0) * locals.var_dnm_dn11)) } } else { (assign24640_body28_e19388 * (assign24640_body28_e19387 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24640_body28_e19387) as f64).is_finite() && ((assign24640_body28_e19387) as f64).fract() == 0.0 { if assign24640_body28_e19387 == 0.0 { 0.0 } else { (assign24640_body28_e19387 * ((locals.var_dnm).powf(assign24640_body28_e19387 - 1.0) * locals.var_dnm_dn14)) } } else { (assign24640_body28_e19388 * (assign24640_body28_e19387 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign24640_body28_e19389, assign24640_body28_e19389_d_n0, assign24640_body28_e19389_d_n2, assign24640_body28_e19389_d_n4, assign24640_body28_e19389_d_n5, assign24640_body28_e19389_d_n6, assign24640_body28_e19389_d_n7, assign24640_body28_e19389_d_n8, assign24640_body28_e19389_d_n9, assign24640_body28_e19389_d_n10, assign24640_body28_e19389_d_n11, assign24640_body28_e19389_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign24640_body28_e19391;
            locals.var_dnm_dn0 = assign24640_body28_e19391_d_n0;
            locals.var_dnm_dn2 = assign24640_body28_e19391_d_n2;
            locals.var_dnm_dn4 = assign24640_body28_e19391_d_n4;
            locals.var_dnm_dn5 = assign24640_body28_e19391_d_n5;
            locals.var_dnm_dn6 = assign24640_body28_e19391_d_n6;
            locals.var_dnm_dn7 = assign24640_body28_e19391_d_n7;
            locals.var_dnm_dn8 = assign24640_body28_e19391_d_n8;
            locals.var_dnm_dn9 = assign24640_body28_e19391_d_n9;
            locals.var_dnm_dn10 = assign24640_body28_e19391_d_n10;
            locals.var_dnm_dn11 = assign24640_body28_e19391_d_n11;
            locals.var_dnm_dn14 = assign24640_body28_e19391_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign24640_body29_e19403, assign24640_body29_e19403_d_n0, assign24640_body29_e19403_d_n2, assign24640_body29_e19403_d_n4, assign24640_body29_e19403_d_n5, assign24640_body29_e19403_d_n6, assign24640_body29_e19403_d_n7, assign24640_body29_e19403_d_n8, assign24640_body29_e19403_d_n9, assign24640_body29_e19403_d_n10, assign24640_body29_e19403_d_n11, assign24640_body29_e19403_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) {
        let assign24640_body29_e19401: f64 = (1.0 / locals.var_dnm);
        (assign24640_body29_e19401, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign24640_body29_e19403;
            locals.var_dnm_dn0 = assign24640_body29_e19403_d_n0;
            locals.var_dnm_dn2 = assign24640_body29_e19403_d_n2;
            locals.var_dnm_dn4 = assign24640_body29_e19403_d_n4;
            locals.var_dnm_dn5 = assign24640_body29_e19403_d_n5;
            locals.var_dnm_dn6 = assign24640_body29_e19403_d_n6;
            locals.var_dnm_dn7 = assign24640_body29_e19403_d_n7;
            locals.var_dnm_dn8 = assign24640_body29_e19403_d_n8;
            locals.var_dnm_dn9 = assign24640_body29_e19403_d_n9;
            locals.var_dnm_dn10 = assign24640_body29_e19403_d_n10;
            locals.var_dnm_dn11 = assign24640_body29_e19403_d_n11;
            locals.var_dnm_dn14 = assign24640_body29_e19403_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign24640_body30_e19417, assign24640_body30_e19417_d_n0, assign24640_body30_e19417_d_n2, assign24640_body30_e19417_d_n4, assign24640_body30_e19417_d_n5, assign24640_body30_e19417_d_n6, assign24640_body30_e19417_d_n7, assign24640_body30_e19417_d_n8, assign24640_body30_e19417_d_n9, assign24640_body30_e19417_d_n10, assign24640_body30_e19417_d_n11, assign24640_body30_e19417_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) {
        let assign24640_body30_e19413: f64 = (locals.var_tmf1 * 1e-8);
        let assign24640_body30_e19415: f64 = (assign24640_body30_e19413 * locals.var_dnm);
        (assign24640_body30_e19415, (((locals.var_tmf1_dn0 * 1e-8) * locals.var_dnm) + (assign24640_body30_e19413 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-8) * locals.var_dnm) + (assign24640_body30_e19413 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1e-8) * locals.var_dnm) + (assign24640_body30_e19413 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1e-8) * locals.var_dnm) + (assign24640_body30_e19413 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1e-8) * locals.var_dnm) + (assign24640_body30_e19413 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-8) * locals.var_dnm) + (assign24640_body30_e19413 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1e-8) * locals.var_dnm) + (assign24640_body30_e19413 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1e-8) * locals.var_dnm) + (assign24640_body30_e19413 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1e-8) * locals.var_dnm) + (assign24640_body30_e19413 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 1e-8) * locals.var_dnm) + (assign24640_body30_e19413 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 1e-8) * locals.var_dnm) + (assign24640_body30_e19413 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
            locals.var_tmf0 = assign24640_body30_e19417;
            locals.var_tmf0_dn0 = assign24640_body30_e19417_d_n0;
            locals.var_tmf0_dn2 = assign24640_body30_e19417_d_n2;
            locals.var_tmf0_dn4 = assign24640_body30_e19417_d_n4;
            locals.var_tmf0_dn5 = assign24640_body30_e19417_d_n5;
            locals.var_tmf0_dn6 = assign24640_body30_e19417_d_n6;
            locals.var_tmf0_dn7 = assign24640_body30_e19417_d_n7;
            locals.var_tmf0_dn8 = assign24640_body30_e19417_d_n8;
            locals.var_tmf0_dn9 = assign24640_body30_e19417_d_n9;
            locals.var_tmf0_dn10 = assign24640_body30_e19417_d_n10;
            locals.var_tmf0_dn11 = assign24640_body30_e19417_d_n11;
            locals.var_tmf0_dn14 = assign24640_body30_e19417_d_n14;
            locals.var_tmf0_rv = 0.0;
            let (assign24640_body31_e19433, assign24640_body31_e19433_d_n0, assign24640_body31_e19433_d_n2, assign24640_body31_e19433_d_n4, assign24640_body31_e19433_d_n5, assign24640_body31_e19433_d_n6, assign24640_body31_e19433_d_n7, assign24640_body31_e19433_d_n8, assign24640_body31_e19433_d_n9, assign24640_body31_e19433_d_n10, assign24640_body31_e19433_d_n11, assign24640_body31_e19433_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) {
        let assign24640_body31_e19427: f64 = (1e-8 * locals.var_xmp);
        let assign24640_body31_e19429: f64 = (assign24640_body31_e19427 * locals.var_dnm);
        let assign24640_body31_e19431: f64 = (assign24640_body31_e19429 / locals.var_arg);
        (assign24640_body31_e19431, ((((((1e-8 * locals.var_xmp_dn0) * locals.var_dnm) + (assign24640_body31_e19427 * locals.var_dnm_dn0)) * locals.var_arg) - (assign24640_body31_e19429 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn2) * locals.var_dnm) + (assign24640_body31_e19427 * locals.var_dnm_dn2)) * locals.var_arg) - (assign24640_body31_e19429 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn4) * locals.var_dnm) + (assign24640_body31_e19427 * locals.var_dnm_dn4)) * locals.var_arg) - (assign24640_body31_e19429 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn5) * locals.var_dnm) + (assign24640_body31_e19427 * locals.var_dnm_dn5)) * locals.var_arg) - (assign24640_body31_e19429 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn6) * locals.var_dnm) + (assign24640_body31_e19427 * locals.var_dnm_dn6)) * locals.var_arg) - (assign24640_body31_e19429 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn7) * locals.var_dnm) + (assign24640_body31_e19427 * locals.var_dnm_dn7)) * locals.var_arg) - (assign24640_body31_e19429 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn8) * locals.var_dnm) + (assign24640_body31_e19427 * locals.var_dnm_dn8)) * locals.var_arg) - (assign24640_body31_e19429 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn9) * locals.var_dnm) + (assign24640_body31_e19427 * locals.var_dnm_dn9)) * locals.var_arg) - (assign24640_body31_e19429 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn10) * locals.var_dnm) + (assign24640_body31_e19427 * locals.var_dnm_dn10)) * locals.var_arg) - (assign24640_body31_e19429 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn11) * locals.var_dnm) + (assign24640_body31_e19427 * locals.var_dnm_dn11)) * locals.var_arg) - (assign24640_body31_e19429 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn14) * locals.var_dnm) + (assign24640_body31_e19427 * locals.var_dnm_dn14)) * locals.var_arg) - (assign24640_body31_e19429 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign24640_body31_e19433;
            locals.var_t0_dn0 = assign24640_body31_e19433_d_n0;
            locals.var_t0_dn2 = assign24640_body31_e19433_d_n2;
            locals.var_t0_dn4 = assign24640_body31_e19433_d_n4;
            locals.var_t0_dn5 = assign24640_body31_e19433_d_n5;
            locals.var_t0_dn6 = assign24640_body31_e19433_d_n6;
            locals.var_t0_dn7 = assign24640_body31_e19433_d_n7;
            locals.var_t0_dn8 = assign24640_body31_e19433_d_n8;
            locals.var_t0_dn9 = assign24640_body31_e19433_d_n9;
            locals.var_t0_dn10 = assign24640_body31_e19433_d_n10;
            locals.var_t0_dn11 = assign24640_body31_e19433_d_n11;
            locals.var_t0_dn14 = assign24640_body31_e19433_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign24640_body32_e19447, assign24640_body32_e19447_d_n0, assign24640_body32_e19447_d_n2, assign24640_body32_e19447_d_n4, assign24640_body32_e19447_d_n5, assign24640_body32_e19447_d_n6, assign24640_body32_e19447_d_n7, assign24640_body32_e19447_d_n8, assign24640_body32_e19447_d_n9, assign24640_body32_e19447_d_n10, assign24640_body32_e19447_d_n11, assign24640_body32_e19447_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) {
        let assign24640_body32_e19443: f64 = (locals.var_uc_depthn - 1e-8);
        let assign24640_body32_e19445: f64 = (assign24640_body32_e19443 + locals.var_tmf0);
        (assign24640_body32_e19445, (locals.var_uc_depthn_dn0 + locals.var_tmf0_dn0), (locals.var_uc_depthn_dn2 + locals.var_tmf0_dn2), (locals.var_uc_depthn_dn4 + locals.var_tmf0_dn4), (locals.var_uc_depthn_dn5 + locals.var_tmf0_dn5), (locals.var_uc_depthn_dn6 + locals.var_tmf0_dn6), (locals.var_uc_depthn_dn7 + locals.var_tmf0_dn7), (locals.var_uc_depthn_dn8 + locals.var_tmf0_dn8), (locals.var_uc_depthn_dn9 + locals.var_tmf0_dn9), (locals.var_uc_depthn_dn10 + locals.var_tmf0_dn10), (locals.var_uc_depthn_dn11 + locals.var_tmf0_dn11), (locals.var_uc_depthn_dn14 + locals.var_tmf0_dn14),)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn11, locals.var_w_b0_dn14,)
    }
};
            locals.var_w_b0 = assign24640_body32_e19447;
            locals.var_w_b0_dn0 = assign24640_body32_e19447_d_n0;
            locals.var_w_b0_dn2 = assign24640_body32_e19447_d_n2;
            locals.var_w_b0_dn4 = assign24640_body32_e19447_d_n4;
            locals.var_w_b0_dn5 = assign24640_body32_e19447_d_n5;
            locals.var_w_b0_dn6 = assign24640_body32_e19447_d_n6;
            locals.var_w_b0_dn7 = assign24640_body32_e19447_d_n7;
            locals.var_w_b0_dn8 = assign24640_body32_e19447_d_n8;
            locals.var_w_b0_dn9 = assign24640_body32_e19447_d_n9;
            locals.var_w_b0_dn10 = assign24640_body32_e19447_d_n10;
            locals.var_w_b0_dn11 = assign24640_body32_e19447_d_n11;
            locals.var_w_b0_dn14 = assign24640_body32_e19447_d_n14;
            locals.var_w_b0_rv = 0.0;
            let (assign24640_body33_e19457, assign24640_body33_e19457_d_n0, assign24640_body33_e19457_d_n2, assign24640_body33_e19457_d_n4, assign24640_body33_e19457_d_n5, assign24640_body33_e19457_d_n6, assign24640_body33_e19457_d_n7, assign24640_body33_e19457_d_n8, assign24640_body33_e19457_d_n9, assign24640_body33_e19457_d_n10, assign24640_body33_e19457_d_n11, assign24640_body33_e19457_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign24640_body33_e19457;
            locals.var_t0_dn0 = assign24640_body33_e19457_d_n0;
            locals.var_t0_dn2 = assign24640_body33_e19457_d_n2;
            locals.var_t0_dn4 = assign24640_body33_e19457_d_n4;
            locals.var_t0_dn5 = assign24640_body33_e19457_d_n5;
            locals.var_t0_dn6 = assign24640_body33_e19457_d_n6;
            locals.var_t0_dn7 = assign24640_body33_e19457_d_n7;
            locals.var_t0_dn8 = assign24640_body33_e19457_d_n8;
            locals.var_t0_dn9 = assign24640_body33_e19457_d_n9;
            locals.var_t0_dn10 = assign24640_body33_e19457_d_n10;
            locals.var_t0_dn11 = assign24640_body33_e19457_d_n11;
            locals.var_t0_dn14 = assign24640_body33_e19457_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign24640_body34_e19468, assign24640_body34_e19468_d_n0, assign24640_body34_e19468_d_n2, assign24640_body34_e19468_d_n4, assign24640_body34_e19468_d_n5, assign24640_body34_e19468_d_n6, assign24640_body34_e19468_d_n7, assign24640_body34_e19468_d_n8, assign24640_body34_e19468_d_n9, assign24640_body34_e19468_d_n10, assign24640_body34_e19468_d_n11, assign24640_body34_e19468_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 == 0.0)) {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn11, locals.var_w_b0_dn14,)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn11, locals.var_w_b0_dn14,)
    }
};
            locals.var_w_b0 = assign24640_body34_e19468;
            locals.var_w_b0_dn0 = assign24640_body34_e19468_d_n0;
            locals.var_w_b0_dn2 = assign24640_body34_e19468_d_n2;
            locals.var_w_b0_dn4 = assign24640_body34_e19468_d_n4;
            locals.var_w_b0_dn5 = assign24640_body34_e19468_d_n5;
            locals.var_w_b0_dn6 = assign24640_body34_e19468_d_n6;
            locals.var_w_b0_dn7 = assign24640_body34_e19468_d_n7;
            locals.var_w_b0_dn8 = assign24640_body34_e19468_d_n8;
            locals.var_w_b0_dn9 = assign24640_body34_e19468_d_n9;
            locals.var_w_b0_dn10 = assign24640_body34_e19468_d_n10;
            locals.var_w_b0_dn11 = assign24640_body34_e19468_d_n11;
            locals.var_w_b0_dn14 = assign24640_body34_e19468_d_n14;
            locals.var_w_b0_rv = 0.0;
            let (assign24640_body35_e19479, assign24640_body35_e19479_d_n0, assign24640_body35_e19479_d_n2, assign24640_body35_e19479_d_n4, assign24640_body35_e19479_d_n5, assign24640_body35_e19479_d_n6, assign24640_body35_e19479_d_n7, assign24640_body35_e19479_d_n8, assign24640_body35_e19479_d_n9, assign24640_body35_e19479_d_n10, assign24640_body35_e19479_d_n11, assign24640_body35_e19479_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign24640_body35_e19479;
            locals.var_t0_dn0 = assign24640_body35_e19479_d_n0;
            locals.var_t0_dn2 = assign24640_body35_e19479_d_n2;
            locals.var_t0_dn4 = assign24640_body35_e19479_d_n4;
            locals.var_t0_dn5 = assign24640_body35_e19479_d_n5;
            locals.var_t0_dn6 = assign24640_body35_e19479_d_n6;
            locals.var_t0_dn7 = assign24640_body35_e19479_d_n7;
            locals.var_t0_dn8 = assign24640_body35_e19479_d_n8;
            locals.var_t0_dn9 = assign24640_body35_e19479_d_n9;
            locals.var_t0_dn10 = assign24640_body35_e19479_d_n10;
            locals.var_t0_dn11 = assign24640_body35_e19479_d_n11;
            locals.var_t0_dn14 = assign24640_body35_e19479_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign24640_body36_e19491, assign24640_body36_e19491_d_n0, assign24640_body36_e19491_d_n2, assign24640_body36_e19491_d_n4, assign24640_body36_e19491_d_n5, assign24640_body36_e19491_d_n6, assign24640_body36_e19491_d_n7, assign24640_body36_e19491_d_n8, assign24640_body36_e19491_d_n9, assign24640_body36_e19491_d_n10, assign24640_body36_e19491_d_n11, assign24640_body36_e19491_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) {
        let assign24640_body36_e19487: f64 = (locals.var_phi_j0_dep - locals.var_vbscl__blk439);
        let assign24640_body36_e19489: f64 = (assign24640_body36_e19487 + locals.var_vbi_dep);
        (assign24640_body36_e19489, ((locals.var_phi_j0_dep_dn0 - locals.var_vbscl__blk439_dn0) + locals.var_vbi_dep_dn0), ((locals.var_phi_j0_dep_dn2 - locals.var_vbscl__blk439_dn2) + locals.var_vbi_dep_dn2), ((locals.var_phi_j0_dep_dn4 - locals.var_vbscl__blk439_dn4) + locals.var_vbi_dep_dn4), ((locals.var_phi_j0_dep_dn5 - locals.var_vbscl__blk439_dn5) + locals.var_vbi_dep_dn5), ((locals.var_phi_j0_dep_dn6 - locals.var_vbscl__blk439_dn6) + locals.var_vbi_dep_dn6), ((locals.var_phi_j0_dep_dn7 - locals.var_vbscl__blk439_dn7) + locals.var_vbi_dep_dn7), ((locals.var_phi_j0_dep_dn8 - locals.var_vbscl__blk439_dn8) + locals.var_vbi_dep_dn8), ((locals.var_phi_j0_dep_dn9 - locals.var_vbscl__blk439_dn9) + locals.var_vbi_dep_dn9), ((locals.var_phi_j0_dep_dn10 - locals.var_vbscl__blk439_dn10) + locals.var_vbi_dep_dn10), ((locals.var_phi_j0_dep_dn11 - locals.var_vbscl__blk439_dn11) + locals.var_vbi_dep_dn11), ((locals.var_phi_j0_dep_dn14 - locals.var_vbscl__blk439_dn14) + locals.var_vbi_dep_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign24640_body36_e19491;
            locals.var_t1_dn0 = assign24640_body36_e19491_d_n0;
            locals.var_t1_dn2 = assign24640_body36_e19491_d_n2;
            locals.var_t1_dn4 = assign24640_body36_e19491_d_n4;
            locals.var_t1_dn5 = assign24640_body36_e19491_d_n5;
            locals.var_t1_dn6 = assign24640_body36_e19491_d_n6;
            locals.var_t1_dn7 = assign24640_body36_e19491_d_n7;
            locals.var_t1_dn8 = assign24640_body36_e19491_d_n8;
            locals.var_t1_dn9 = assign24640_body36_e19491_d_n9;
            locals.var_t1_dn10 = assign24640_body36_e19491_d_n10;
            locals.var_t1_dn11 = assign24640_body36_e19491_d_n11;
            locals.var_t1_dn14 = assign24640_body36_e19491_d_n14;
            locals.var_t1_rv = 0.0;
            let assign24640_body37_e19495: f64 = 0.1;
            let assign24640_body37_e19500: f64 = if ((locals.var_t1 < assign24640_body37_e19495) && (0.1 >= 0.0)) { 1.0 } else { 0.0 };
            locals.var_guard570 = assign24640_body37_e19500;
            locals.var_guard570_rv = 0.0;
            let (assign24640_body38_e19514, assign24640_body38_e19514_d_n0, assign24640_body38_e19514_d_n2, assign24640_body38_e19514_d_n4, assign24640_body38_e19514_d_n5, assign24640_body38_e19514_d_n6, assign24640_body38_e19514_d_n7, assign24640_body38_e19514_d_n8, assign24640_body38_e19514_d_n9, assign24640_body38_e19514_d_n10, assign24640_body38_e19514_d_n11, assign24640_body38_e19514_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard570 != 0.0)) {
        let assign24640_body38_e19510: f64 = 0.1;
        let assign24640_body38_e19512: f64 = (assign24640_body38_e19510 - locals.var_t1);
        (assign24640_body38_e19512, (-locals.var_t1_dn0), (-locals.var_t1_dn2), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11), (-locals.var_t1_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
            locals.var_tmf1 = assign24640_body38_e19514;
            locals.var_tmf1_dn0 = assign24640_body38_e19514_d_n0;
            locals.var_tmf1_dn2 = assign24640_body38_e19514_d_n2;
            locals.var_tmf1_dn4 = assign24640_body38_e19514_d_n4;
            locals.var_tmf1_dn5 = assign24640_body38_e19514_d_n5;
            locals.var_tmf1_dn6 = assign24640_body38_e19514_d_n6;
            locals.var_tmf1_dn7 = assign24640_body38_e19514_d_n7;
            locals.var_tmf1_dn8 = assign24640_body38_e19514_d_n8;
            locals.var_tmf1_dn9 = assign24640_body38_e19514_d_n9;
            locals.var_tmf1_dn10 = assign24640_body38_e19514_d_n10;
            locals.var_tmf1_dn11 = assign24640_body38_e19514_d_n11;
            locals.var_tmf1_dn14 = assign24640_body38_e19514_d_n14;
            locals.var_tmf1_rv = 0.0;
            let (assign24640_body39_e19526, assign24640_body39_e19526_d_n0, assign24640_body39_e19526_d_n2, assign24640_body39_e19526_d_n4, assign24640_body39_e19526_d_n5, assign24640_body39_e19526_d_n6, assign24640_body39_e19526_d_n7, assign24640_body39_e19526_d_n8, assign24640_body39_e19526_d_n9, assign24640_body39_e19526_d_n10, assign24640_body39_e19526_d_n11, assign24640_body39_e19526_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard570 != 0.0)) {
        let assign24640_body39_e19524: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign24640_body39_e19524, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
            locals.var_x2 = assign24640_body39_e19526;
            locals.var_x2_dn0 = assign24640_body39_e19526_d_n0;
            locals.var_x2_dn2 = assign24640_body39_e19526_d_n2;
            locals.var_x2_dn4 = assign24640_body39_e19526_d_n4;
            locals.var_x2_dn5 = assign24640_body39_e19526_d_n5;
            locals.var_x2_dn6 = assign24640_body39_e19526_d_n6;
            locals.var_x2_dn7 = assign24640_body39_e19526_d_n7;
            locals.var_x2_dn8 = assign24640_body39_e19526_d_n8;
            locals.var_x2_dn9 = assign24640_body39_e19526_d_n9;
            locals.var_x2_dn10 = assign24640_body39_e19526_d_n10;
            locals.var_x2_dn11 = assign24640_body39_e19526_d_n11;
            locals.var_x2_dn14 = assign24640_body39_e19526_d_n14;
            locals.var_x2_rv = 0.0;
            let (assign24640_body40_e19538, assign24640_body40_e19538_d_n0, assign24640_body40_e19538_d_n2, assign24640_body40_e19538_d_n4, assign24640_body40_e19538_d_n5, assign24640_body40_e19538_d_n6, assign24640_body40_e19538_d_n7, assign24640_body40_e19538_d_n8, assign24640_body40_e19538_d_n9, assign24640_body40_e19538_d_n10, assign24640_body40_e19538_d_n11, assign24640_body40_e19538_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard570 != 0.0)) {
        let assign24640_body40_e19536: f64 = (0.1 * 0.1);
        (assign24640_body40_e19536, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
            locals.var_xmax2 = assign24640_body40_e19538;
            locals.var_xmax2_dn0 = assign24640_body40_e19538_d_n0;
            locals.var_xmax2_dn2 = assign24640_body40_e19538_d_n2;
            locals.var_xmax2_dn4 = assign24640_body40_e19538_d_n4;
            locals.var_xmax2_dn5 = assign24640_body40_e19538_d_n5;
            locals.var_xmax2_dn6 = assign24640_body40_e19538_d_n6;
            locals.var_xmax2_dn7 = assign24640_body40_e19538_d_n7;
            locals.var_xmax2_dn8 = assign24640_body40_e19538_d_n8;
            locals.var_xmax2_dn9 = assign24640_body40_e19538_d_n9;
            locals.var_xmax2_dn10 = assign24640_body40_e19538_d_n10;
            locals.var_xmax2_dn11 = assign24640_body40_e19538_d_n11;
            locals.var_xmax2_dn14 = assign24640_body40_e19538_d_n14;
            locals.var_xmax2_rv = 0.0;
            let (assign24640_body41_e19548, assign24640_body41_e19548_d_n0, assign24640_body41_e19548_d_n2, assign24640_body41_e19548_d_n4, assign24640_body41_e19548_d_n5, assign24640_body41_e19548_d_n6, assign24640_body41_e19548_d_n7, assign24640_body41_e19548_d_n8, assign24640_body41_e19548_d_n9, assign24640_body41_e19548_d_n10, assign24640_body41_e19548_d_n11, assign24640_body41_e19548_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard570 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
            locals.var_xp = assign24640_body41_e19548;
            locals.var_xp_dn0 = assign24640_body41_e19548_d_n0;
            locals.var_xp_dn2 = assign24640_body41_e19548_d_n2;
            locals.var_xp_dn4 = assign24640_body41_e19548_d_n4;
            locals.var_xp_dn5 = assign24640_body41_e19548_d_n5;
            locals.var_xp_dn6 = assign24640_body41_e19548_d_n6;
            locals.var_xp_dn7 = assign24640_body41_e19548_d_n7;
            locals.var_xp_dn8 = assign24640_body41_e19548_d_n8;
            locals.var_xp_dn9 = assign24640_body41_e19548_d_n9;
            locals.var_xp_dn10 = assign24640_body41_e19548_d_n10;
            locals.var_xp_dn11 = assign24640_body41_e19548_d_n11;
            locals.var_xp_dn14 = assign24640_body41_e19548_d_n14;
            locals.var_xp_rv = 0.0;
            let (assign24640_body42_e19558, assign24640_body42_e19558_d_n0, assign24640_body42_e19558_d_n2, assign24640_body42_e19558_d_n4, assign24640_body42_e19558_d_n5, assign24640_body42_e19558_d_n6, assign24640_body42_e19558_d_n7, assign24640_body42_e19558_d_n8, assign24640_body42_e19558_d_n9, assign24640_body42_e19558_d_n10, assign24640_body42_e19558_d_n11, assign24640_body42_e19558_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard570 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
            locals.var_xmp = assign24640_body42_e19558;
            locals.var_xmp_dn0 = assign24640_body42_e19558_d_n0;
            locals.var_xmp_dn2 = assign24640_body42_e19558_d_n2;
            locals.var_xmp_dn4 = assign24640_body42_e19558_d_n4;
            locals.var_xmp_dn5 = assign24640_body42_e19558_d_n5;
            locals.var_xmp_dn6 = assign24640_body42_e19558_d_n6;
            locals.var_xmp_dn7 = assign24640_body42_e19558_d_n7;
            locals.var_xmp_dn8 = assign24640_body42_e19558_d_n8;
            locals.var_xmp_dn9 = assign24640_body42_e19558_d_n9;
            locals.var_xmp_dn10 = assign24640_body42_e19558_d_n10;
            locals.var_xmp_dn11 = assign24640_body42_e19558_d_n11;
            locals.var_xmp_dn14 = assign24640_body42_e19558_d_n14;
            locals.var_xmp_rv = 0.0;
            let (assign24640_body43_e19568,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard570 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign24640_body43_e19568;
            locals.var_m0_rv = 0.0;
            let (assign24640_body44_e19578,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard570 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign24640_body44_e19578;
            locals.var_mm_rv = 0.0;
            let (assign24640_body45_e19588, assign24640_body45_e19588_d_n0, assign24640_body45_e19588_d_n2, assign24640_body45_e19588_d_n4, assign24640_body45_e19588_d_n5, assign24640_body45_e19588_d_n6, assign24640_body45_e19588_d_n7, assign24640_body45_e19588_d_n8, assign24640_body45_e19588_d_n9, assign24640_body45_e19588_d_n10, assign24640_body45_e19588_d_n11, assign24640_body45_e19588_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard570 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
            locals.var_arg = assign24640_body45_e19588;
            locals.var_arg_dn0 = assign24640_body45_e19588_d_n0;
            locals.var_arg_dn2 = assign24640_body45_e19588_d_n2;
            locals.var_arg_dn4 = assign24640_body45_e19588_d_n4;
            locals.var_arg_dn5 = assign24640_body45_e19588_d_n5;
            locals.var_arg_dn6 = assign24640_body45_e19588_d_n6;
            locals.var_arg_dn7 = assign24640_body45_e19588_d_n7;
            locals.var_arg_dn8 = assign24640_body45_e19588_d_n8;
            locals.var_arg_dn9 = assign24640_body45_e19588_d_n9;
            locals.var_arg_dn10 = assign24640_body45_e19588_d_n10;
            locals.var_arg_dn11 = assign24640_body45_e19588_d_n11;
            locals.var_arg_dn14 = assign24640_body45_e19588_d_n14;
            locals.var_arg_rv = 0.0;
            let (assign24640_body46_e19598, assign24640_body46_e19598_d_n0, assign24640_body46_e19598_d_n2, assign24640_body46_e19598_d_n4, assign24640_body46_e19598_d_n5, assign24640_body46_e19598_d_n6, assign24640_body46_e19598_d_n7, assign24640_body46_e19598_d_n8, assign24640_body46_e19598_d_n9, assign24640_body46_e19598_d_n10, assign24640_body46_e19598_d_n11, assign24640_body46_e19598_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard570 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign24640_body46_e19598;
            locals.var_dnm_dn0 = assign24640_body46_e19598_d_n0;
            locals.var_dnm_dn2 = assign24640_body46_e19598_d_n2;
            locals.var_dnm_dn4 = assign24640_body46_e19598_d_n4;
            locals.var_dnm_dn5 = assign24640_body46_e19598_d_n5;
            locals.var_dnm_dn6 = assign24640_body46_e19598_d_n6;
            locals.var_dnm_dn7 = assign24640_body46_e19598_d_n7;
            locals.var_dnm_dn8 = assign24640_body46_e19598_d_n8;
            locals.var_dnm_dn9 = assign24640_body46_e19598_d_n9;
            locals.var_dnm_dn10 = assign24640_body46_e19598_d_n10;
            locals.var_dnm_dn11 = assign24640_body46_e19598_d_n11;
            locals.var_dnm_dn14 = assign24640_body46_e19598_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign24640_body47_e19610, assign24640_body47_e19610_d_n0, assign24640_body47_e19610_d_n2, assign24640_body47_e19610_d_n4, assign24640_body47_e19610_d_n5, assign24640_body47_e19610_d_n6, assign24640_body47_e19610_d_n7, assign24640_body47_e19610_d_n8, assign24640_body47_e19610_d_n9, assign24640_body47_e19610_d_n10, assign24640_body47_e19610_d_n11, assign24640_body47_e19610_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard570 != 0.0)) {
        let assign24640_body47_e19608: f64 = (locals.var_xp * locals.var_x2);
        (assign24640_body47_e19608, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
            locals.var_xp = assign24640_body47_e19610;
            locals.var_xp_dn0 = assign24640_body47_e19610_d_n0;
            locals.var_xp_dn2 = assign24640_body47_e19610_d_n2;
            locals.var_xp_dn4 = assign24640_body47_e19610_d_n4;
            locals.var_xp_dn5 = assign24640_body47_e19610_d_n5;
            locals.var_xp_dn6 = assign24640_body47_e19610_d_n6;
            locals.var_xp_dn7 = assign24640_body47_e19610_d_n7;
            locals.var_xp_dn8 = assign24640_body47_e19610_d_n8;
            locals.var_xp_dn9 = assign24640_body47_e19610_d_n9;
            locals.var_xp_dn10 = assign24640_body47_e19610_d_n10;
            locals.var_xp_dn11 = assign24640_body47_e19610_d_n11;
            locals.var_xp_dn14 = assign24640_body47_e19610_d_n14;
            locals.var_xp_rv = 0.0;
            let (assign24640_body48_e19622, assign24640_body48_e19622_d_n0, assign24640_body48_e19622_d_n2, assign24640_body48_e19622_d_n4, assign24640_body48_e19622_d_n5, assign24640_body48_e19622_d_n6, assign24640_body48_e19622_d_n7, assign24640_body48_e19622_d_n8, assign24640_body48_e19622_d_n9, assign24640_body48_e19622_d_n10, assign24640_body48_e19622_d_n11, assign24640_body48_e19622_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard570 != 0.0)) {
        let assign24640_body48_e19620: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign24640_body48_e19620, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
            locals.var_xmp = assign24640_body48_e19622;
            locals.var_xmp_dn0 = assign24640_body48_e19622_d_n0;
            locals.var_xmp_dn2 = assign24640_body48_e19622_d_n2;
            locals.var_xmp_dn4 = assign24640_body48_e19622_d_n4;
            locals.var_xmp_dn5 = assign24640_body48_e19622_d_n5;
            locals.var_xmp_dn6 = assign24640_body48_e19622_d_n6;
            locals.var_xmp_dn7 = assign24640_body48_e19622_d_n7;
            locals.var_xmp_dn8 = assign24640_body48_e19622_d_n8;
            locals.var_xmp_dn9 = assign24640_body48_e19622_d_n9;
            locals.var_xmp_dn10 = assign24640_body48_e19622_d_n10;
            locals.var_xmp_dn11 = assign24640_body48_e19622_d_n11;
            locals.var_xmp_dn14 = assign24640_body48_e19622_d_n14;
            locals.var_xmp_rv = 0.0;
            let (assign24640_body49_e19634, assign24640_body49_e19634_d_n0, assign24640_body49_e19634_d_n2, assign24640_body49_e19634_d_n4, assign24640_body49_e19634_d_n5, assign24640_body49_e19634_d_n6, assign24640_body49_e19634_d_n7, assign24640_body49_e19634_d_n8, assign24640_body49_e19634_d_n9, assign24640_body49_e19634_d_n10, assign24640_body49_e19634_d_n11, assign24640_body49_e19634_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard570 != 0.0)) {
        let assign24640_body49_e19632: f64 = (locals.var_xp * locals.var_x2);
        (assign24640_body49_e19632, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
            locals.var_xp = assign24640_body49_e19634;
            locals.var_xp_dn0 = assign24640_body49_e19634_d_n0;
            locals.var_xp_dn2 = assign24640_body49_e19634_d_n2;
            locals.var_xp_dn4 = assign24640_body49_e19634_d_n4;
            locals.var_xp_dn5 = assign24640_body49_e19634_d_n5;
            locals.var_xp_dn6 = assign24640_body49_e19634_d_n6;
            locals.var_xp_dn7 = assign24640_body49_e19634_d_n7;
            locals.var_xp_dn8 = assign24640_body49_e19634_d_n8;
            locals.var_xp_dn9 = assign24640_body49_e19634_d_n9;
            locals.var_xp_dn10 = assign24640_body49_e19634_d_n10;
            locals.var_xp_dn11 = assign24640_body49_e19634_d_n11;
            locals.var_xp_dn14 = assign24640_body49_e19634_d_n14;
            locals.var_xp_rv = 0.0;
            let (assign24640_body50_e19646, assign24640_body50_e19646_d_n0, assign24640_body50_e19646_d_n2, assign24640_body50_e19646_d_n4, assign24640_body50_e19646_d_n5, assign24640_body50_e19646_d_n6, assign24640_body50_e19646_d_n7, assign24640_body50_e19646_d_n8, assign24640_body50_e19646_d_n9, assign24640_body50_e19646_d_n10, assign24640_body50_e19646_d_n11, assign24640_body50_e19646_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard570 != 0.0)) {
        let assign24640_body50_e19644: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign24640_body50_e19644, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
            locals.var_xmp = assign24640_body50_e19646;
            locals.var_xmp_dn0 = assign24640_body50_e19646_d_n0;
            locals.var_xmp_dn2 = assign24640_body50_e19646_d_n2;
            locals.var_xmp_dn4 = assign24640_body50_e19646_d_n4;
            locals.var_xmp_dn5 = assign24640_body50_e19646_d_n5;
            locals.var_xmp_dn6 = assign24640_body50_e19646_d_n6;
            locals.var_xmp_dn7 = assign24640_body50_e19646_d_n7;
            locals.var_xmp_dn8 = assign24640_body50_e19646_d_n8;
            locals.var_xmp_dn9 = assign24640_body50_e19646_d_n9;
            locals.var_xmp_dn10 = assign24640_body50_e19646_d_n10;
            locals.var_xmp_dn11 = assign24640_body50_e19646_d_n11;
            locals.var_xmp_dn14 = assign24640_body50_e19646_d_n14;
            locals.var_xmp_rv = 0.0;
            let (assign24640_body51_e19658, assign24640_body51_e19658_d_n0, assign24640_body51_e19658_d_n2, assign24640_body51_e19658_d_n4, assign24640_body51_e19658_d_n5, assign24640_body51_e19658_d_n6, assign24640_body51_e19658_d_n7, assign24640_body51_e19658_d_n8, assign24640_body51_e19658_d_n9, assign24640_body51_e19658_d_n10, assign24640_body51_e19658_d_n11, assign24640_body51_e19658_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard570 != 0.0)) {
        let assign24640_body51_e19656: f64 = (locals.var_xp + locals.var_xmp);
        (assign24640_body51_e19656, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
            locals.var_arg = assign24640_body51_e19658;
            locals.var_arg_dn0 = assign24640_body51_e19658_d_n0;
            locals.var_arg_dn2 = assign24640_body51_e19658_d_n2;
            locals.var_arg_dn4 = assign24640_body51_e19658_d_n4;
            locals.var_arg_dn5 = assign24640_body51_e19658_d_n5;
            locals.var_arg_dn6 = assign24640_body51_e19658_d_n6;
            locals.var_arg_dn7 = assign24640_body51_e19658_d_n7;
            locals.var_arg_dn8 = assign24640_body51_e19658_d_n8;
            locals.var_arg_dn9 = assign24640_body51_e19658_d_n9;
            locals.var_arg_dn10 = assign24640_body51_e19658_d_n10;
            locals.var_arg_dn11 = assign24640_body51_e19658_d_n11;
            locals.var_arg_dn14 = assign24640_body51_e19658_d_n14;
            locals.var_arg_rv = 0.0;
            let (assign24640_body52_e19668, assign24640_body52_e19668_d_n0, assign24640_body52_e19668_d_n2, assign24640_body52_e19668_d_n4, assign24640_body52_e19668_d_n5, assign24640_body52_e19668_d_n6, assign24640_body52_e19668_d_n7, assign24640_body52_e19668_d_n8, assign24640_body52_e19668_d_n9, assign24640_body52_e19668_d_n10, assign24640_body52_e19668_d_n11, assign24640_body52_e19668_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard570 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign24640_body52_e19668;
            locals.var_dnm_dn0 = assign24640_body52_e19668_d_n0;
            locals.var_dnm_dn2 = assign24640_body52_e19668_d_n2;
            locals.var_dnm_dn4 = assign24640_body52_e19668_d_n4;
            locals.var_dnm_dn5 = assign24640_body52_e19668_d_n5;
            locals.var_dnm_dn6 = assign24640_body52_e19668_d_n6;
            locals.var_dnm_dn7 = assign24640_body52_e19668_d_n7;
            locals.var_dnm_dn8 = assign24640_body52_e19668_d_n8;
            locals.var_dnm_dn9 = assign24640_body52_e19668_d_n9;
            locals.var_dnm_dn10 = assign24640_body52_e19668_d_n10;
            locals.var_dnm_dn11 = assign24640_body52_e19668_d_n11;
            locals.var_dnm_dn14 = assign24640_body52_e19668_d_n14;
            locals.var_dnm_rv = 0.0;
            let assign24640_body53_e19683: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
            locals.var_guard571 = assign24640_body53_e19683;
            locals.var_guard571_rv = 0.0;
            let assign24640_body54_e19686: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard572 = assign24640_body54_e19686;
            locals.var_guard572_rv = 0.0;
            let (assign24640_body55_e19700,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard570 != 0.0)) && (locals.var_guard571 != 0.0)) && (locals.var_guard572 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign24640_body55_e19700;
            locals.var_mm_rv = 0.0;
            let assign24640_body56_e19703: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
            locals.var_guard573 = assign24640_body56_e19703;
            locals.var_guard573_rv = 0.0;
            let (assign24640_body57_e19720,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard570 != 0.0)) && (locals.var_guard571 != 0.0)) && (locals.var_guard572 == 0.0)) && (locals.var_guard573 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign24640_body57_e19720;
            locals.var_mm_rv = 0.0;
            let assign24640_body58_e19723: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
            locals.var_guard574 = assign24640_body58_e19723;
            locals.var_guard574_rv = 0.0;
            let (assign24640_body59_e19743,) = {
    if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard570 != 0.0)) && (locals.var_guard571 != 0.0)) && (locals.var_guard572 == 0.0)) && (locals.var_guard573 == 0.0)) && (locals.var_guard574 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign24640_body59_e19743;
            locals.var_mm_rv = 0.0;
            let assign24640_body60_e19746: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
            locals.var_guard575 = assign24640_body60_e19746;
            locals.var_guard575_rv = 0.0;
            let (assign24640_body61_e19769,) = {
    if (((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard570 != 0.0)) && (locals.var_guard571 != 0.0)) && (locals.var_guard572 == 0.0)) && (locals.var_guard573 == 0.0)) && (locals.var_guard574 == 0.0)) && (locals.var_guard575 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign24640_body61_e19769;
            locals.var_mm_rv = 0.0;
            let (assign24640_body62_e19781,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard570 != 0.0)) && (locals.var_guard571 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign24640_body62_e19781;
            locals.var_m0_rv = 0.0;
            let mut assign24640_body63_loop_guard: usize = 0;
            while {
                let assign24640_body63_cond_e19794: f64 = if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard570 != 0.0)) && (locals.var_guard571 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
                assign24640_body63_cond_e19794 != 0.0
            } {
                assign24640_body63_loop_guard += 1;
                assert!(assign24640_body63_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                let (assign24640_body63_body0_e19807, assign24640_body63_body0_e19807_d_n0, assign24640_body63_body0_e19807_d_n2, assign24640_body63_body0_e19807_d_n4, assign24640_body63_body0_e19807_d_n5, assign24640_body63_body0_e19807_d_n6, assign24640_body63_body0_e19807_d_n7, assign24640_body63_body0_e19807_d_n8, assign24640_body63_body0_e19807_d_n9, assign24640_body63_body0_e19807_d_n10, assign24640_body63_body0_e19807_d_n11, assign24640_body63_body0_e19807_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard570 != 0.0)) && (locals.var_guard571 != 0.0)) {
        let assign24640_body63_body0_e19805: f64 = (locals.var_dnm).sqrt();
        (assign24640_body63_body0_e19805, (locals.var_dnm_dn0 / (2.0 * assign24640_body63_body0_e19805)), (locals.var_dnm_dn2 / (2.0 * assign24640_body63_body0_e19805)), (locals.var_dnm_dn4 / (2.0 * assign24640_body63_body0_e19805)), (locals.var_dnm_dn5 / (2.0 * assign24640_body63_body0_e19805)), (locals.var_dnm_dn6 / (2.0 * assign24640_body63_body0_e19805)), (locals.var_dnm_dn7 / (2.0 * assign24640_body63_body0_e19805)), (locals.var_dnm_dn8 / (2.0 * assign24640_body63_body0_e19805)), (locals.var_dnm_dn9 / (2.0 * assign24640_body63_body0_e19805)), (locals.var_dnm_dn10 / (2.0 * assign24640_body63_body0_e19805)), (locals.var_dnm_dn11 / (2.0 * assign24640_body63_body0_e19805)), (locals.var_dnm_dn14 / (2.0 * assign24640_body63_body0_e19805)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
                locals.var_dnm = assign24640_body63_body0_e19807;
                locals.var_dnm_dn0 = assign24640_body63_body0_e19807_d_n0;
                locals.var_dnm_dn2 = assign24640_body63_body0_e19807_d_n2;
                locals.var_dnm_dn4 = assign24640_body63_body0_e19807_d_n4;
                locals.var_dnm_dn5 = assign24640_body63_body0_e19807_d_n5;
                locals.var_dnm_dn6 = assign24640_body63_body0_e19807_d_n6;
                locals.var_dnm_dn7 = assign24640_body63_body0_e19807_d_n7;
                locals.var_dnm_dn8 = assign24640_body63_body0_e19807_d_n8;
                locals.var_dnm_dn9 = assign24640_body63_body0_e19807_d_n9;
                locals.var_dnm_dn10 = assign24640_body63_body0_e19807_d_n10;
                locals.var_dnm_dn11 = assign24640_body63_body0_e19807_d_n11;
                locals.var_dnm_dn14 = assign24640_body63_body0_e19807_d_n14;
                locals.var_dnm_rv = 0.0;
                let (assign24640_body63_body1_e19821,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard570 != 0.0)) && (locals.var_guard571 != 0.0)) {
        let assign24640_body63_body1_e19819: f64 = (locals.var_m0 + 1.0);
        (assign24640_body63_body1_e19819,)
    } else {
        (locals.var_m0,)
    }
};
                locals.var_m0 = assign24640_body63_body1_e19821;
                locals.var_m0_rv = 0.0;
            }
            let (assign24640_body64_e19845, assign24640_body64_e19845_d_n0, assign24640_body64_e19845_d_n2, assign24640_body64_e19845_d_n4, assign24640_body64_e19845_d_n5, assign24640_body64_e19845_d_n6, assign24640_body64_e19845_d_n7, assign24640_body64_e19845_d_n8, assign24640_body64_e19845_d_n9, assign24640_body64_e19845_d_n10, assign24640_body64_e19845_d_n11, assign24640_body64_e19845_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard570 != 0.0)) && (locals.var_guard571 == 0.0)) {
        let (assign24640_body64_e19843, assign24640_body64_e19843_d_n0, assign24640_body64_e19843_d_n2, assign24640_body64_e19843_d_n4, assign24640_body64_e19843_d_n5, assign24640_body64_e19843_d_n6, assign24640_body64_e19843_d_n7, assign24640_body64_e19843_d_n8, assign24640_body64_e19843_d_n9, assign24640_body64_e19843_d_n10, assign24640_body64_e19843_d_n11, assign24640_body64_e19843_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign24640_body64_e19840: f64 = (2.0 * 2.0);
                let assign24640_body64_e19841: f64 = (1.0 / assign24640_body64_e19840);
                let assign24640_body64_e19842: f64 = (locals.var_dnm).powf(assign24640_body64_e19841);
                (assign24640_body64_e19842, if 0.0 == 0.0 && ((assign24640_body64_e19841) as f64).is_finite() && ((assign24640_body64_e19841) as f64).fract() == 0.0 { if assign24640_body64_e19841 == 0.0 { 0.0 } else { (assign24640_body64_e19841 * ((locals.var_dnm).powf(assign24640_body64_e19841 - 1.0) * locals.var_dnm_dn0)) } } else { (assign24640_body64_e19842 * (assign24640_body64_e19841 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24640_body64_e19841) as f64).is_finite() && ((assign24640_body64_e19841) as f64).fract() == 0.0 { if assign24640_body64_e19841 == 0.0 { 0.0 } else { (assign24640_body64_e19841 * ((locals.var_dnm).powf(assign24640_body64_e19841 - 1.0) * locals.var_dnm_dn2)) } } else { (assign24640_body64_e19842 * (assign24640_body64_e19841 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24640_body64_e19841) as f64).is_finite() && ((assign24640_body64_e19841) as f64).fract() == 0.0 { if assign24640_body64_e19841 == 0.0 { 0.0 } else { (assign24640_body64_e19841 * ((locals.var_dnm).powf(assign24640_body64_e19841 - 1.0) * locals.var_dnm_dn4)) } } else { (assign24640_body64_e19842 * (assign24640_body64_e19841 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24640_body64_e19841) as f64).is_finite() && ((assign24640_body64_e19841) as f64).fract() == 0.0 { if assign24640_body64_e19841 == 0.0 { 0.0 } else { (assign24640_body64_e19841 * ((locals.var_dnm).powf(assign24640_body64_e19841 - 1.0) * locals.var_dnm_dn5)) } } else { (assign24640_body64_e19842 * (assign24640_body64_e19841 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24640_body64_e19841) as f64).is_finite() && ((assign24640_body64_e19841) as f64).fract() == 0.0 { if assign24640_body64_e19841 == 0.0 { 0.0 } else { (assign24640_body64_e19841 * ((locals.var_dnm).powf(assign24640_body64_e19841 - 1.0) * locals.var_dnm_dn6)) } } else { (assign24640_body64_e19842 * (assign24640_body64_e19841 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24640_body64_e19841) as f64).is_finite() && ((assign24640_body64_e19841) as f64).fract() == 0.0 { if assign24640_body64_e19841 == 0.0 { 0.0 } else { (assign24640_body64_e19841 * ((locals.var_dnm).powf(assign24640_body64_e19841 - 1.0) * locals.var_dnm_dn7)) } } else { (assign24640_body64_e19842 * (assign24640_body64_e19841 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24640_body64_e19841) as f64).is_finite() && ((assign24640_body64_e19841) as f64).fract() == 0.0 { if assign24640_body64_e19841 == 0.0 { 0.0 } else { (assign24640_body64_e19841 * ((locals.var_dnm).powf(assign24640_body64_e19841 - 1.0) * locals.var_dnm_dn8)) } } else { (assign24640_body64_e19842 * (assign24640_body64_e19841 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24640_body64_e19841) as f64).is_finite() && ((assign24640_body64_e19841) as f64).fract() == 0.0 { if assign24640_body64_e19841 == 0.0 { 0.0 } else { (assign24640_body64_e19841 * ((locals.var_dnm).powf(assign24640_body64_e19841 - 1.0) * locals.var_dnm_dn9)) } } else { (assign24640_body64_e19842 * (assign24640_body64_e19841 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24640_body64_e19841) as f64).is_finite() && ((assign24640_body64_e19841) as f64).fract() == 0.0 { if assign24640_body64_e19841 == 0.0 { 0.0 } else { (assign24640_body64_e19841 * ((locals.var_dnm).powf(assign24640_body64_e19841 - 1.0) * locals.var_dnm_dn10)) } } else { (assign24640_body64_e19842 * (assign24640_body64_e19841 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24640_body64_e19841) as f64).is_finite() && ((assign24640_body64_e19841) as f64).fract() == 0.0 { if assign24640_body64_e19841 == 0.0 { 0.0 } else { (assign24640_body64_e19841 * ((locals.var_dnm).powf(assign24640_body64_e19841 - 1.0) * locals.var_dnm_dn11)) } } else { (assign24640_body64_e19842 * (assign24640_body64_e19841 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24640_body64_e19841) as f64).is_finite() && ((assign24640_body64_e19841) as f64).fract() == 0.0 { if assign24640_body64_e19841 == 0.0 { 0.0 } else { (assign24640_body64_e19841 * ((locals.var_dnm).powf(assign24640_body64_e19841 - 1.0) * locals.var_dnm_dn14)) } } else { (assign24640_body64_e19842 * (assign24640_body64_e19841 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign24640_body64_e19843, assign24640_body64_e19843_d_n0, assign24640_body64_e19843_d_n2, assign24640_body64_e19843_d_n4, assign24640_body64_e19843_d_n5, assign24640_body64_e19843_d_n6, assign24640_body64_e19843_d_n7, assign24640_body64_e19843_d_n8, assign24640_body64_e19843_d_n9, assign24640_body64_e19843_d_n10, assign24640_body64_e19843_d_n11, assign24640_body64_e19843_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign24640_body64_e19845;
            locals.var_dnm_dn0 = assign24640_body64_e19845_d_n0;
            locals.var_dnm_dn2 = assign24640_body64_e19845_d_n2;
            locals.var_dnm_dn4 = assign24640_body64_e19845_d_n4;
            locals.var_dnm_dn5 = assign24640_body64_e19845_d_n5;
            locals.var_dnm_dn6 = assign24640_body64_e19845_d_n6;
            locals.var_dnm_dn7 = assign24640_body64_e19845_d_n7;
            locals.var_dnm_dn8 = assign24640_body64_e19845_d_n8;
            locals.var_dnm_dn9 = assign24640_body64_e19845_d_n9;
            locals.var_dnm_dn10 = assign24640_body64_e19845_d_n10;
            locals.var_dnm_dn11 = assign24640_body64_e19845_d_n11;
            locals.var_dnm_dn14 = assign24640_body64_e19845_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign24640_body65_e19857, assign24640_body65_e19857_d_n0, assign24640_body65_e19857_d_n2, assign24640_body65_e19857_d_n4, assign24640_body65_e19857_d_n5, assign24640_body65_e19857_d_n6, assign24640_body65_e19857_d_n7, assign24640_body65_e19857_d_n8, assign24640_body65_e19857_d_n9, assign24640_body65_e19857_d_n10, assign24640_body65_e19857_d_n11, assign24640_body65_e19857_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard570 != 0.0)) {
        let assign24640_body65_e19855: f64 = (1.0 / locals.var_dnm);
        (assign24640_body65_e19855, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign24640_body65_e19857;
            locals.var_dnm_dn0 = assign24640_body65_e19857_d_n0;
            locals.var_dnm_dn2 = assign24640_body65_e19857_d_n2;
            locals.var_dnm_dn4 = assign24640_body65_e19857_d_n4;
            locals.var_dnm_dn5 = assign24640_body65_e19857_d_n5;
            locals.var_dnm_dn6 = assign24640_body65_e19857_d_n6;
            locals.var_dnm_dn7 = assign24640_body65_e19857_d_n7;
            locals.var_dnm_dn8 = assign24640_body65_e19857_d_n8;
            locals.var_dnm_dn9 = assign24640_body65_e19857_d_n9;
            locals.var_dnm_dn10 = assign24640_body65_e19857_d_n10;
            locals.var_dnm_dn11 = assign24640_body65_e19857_d_n11;
            locals.var_dnm_dn14 = assign24640_body65_e19857_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign24640_body66_e19871, assign24640_body66_e19871_d_n0, assign24640_body66_e19871_d_n2, assign24640_body66_e19871_d_n4, assign24640_body66_e19871_d_n5, assign24640_body66_e19871_d_n6, assign24640_body66_e19871_d_n7, assign24640_body66_e19871_d_n8, assign24640_body66_e19871_d_n9, assign24640_body66_e19871_d_n10, assign24640_body66_e19871_d_n11, assign24640_body66_e19871_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard570 != 0.0)) {
        let assign24640_body66_e19867: f64 = (locals.var_tmf1 * 0.1);
        let assign24640_body66_e19869: f64 = (assign24640_body66_e19867 * locals.var_dnm);
        (assign24640_body66_e19869, (((locals.var_tmf1_dn0 * 0.1) * locals.var_dnm) + (assign24640_body66_e19867 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.1) * locals.var_dnm) + (assign24640_body66_e19867 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.1) * locals.var_dnm) + (assign24640_body66_e19867 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.1) * locals.var_dnm) + (assign24640_body66_e19867 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.1) * locals.var_dnm) + (assign24640_body66_e19867 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.1) * locals.var_dnm) + (assign24640_body66_e19867 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.1) * locals.var_dnm) + (assign24640_body66_e19867 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.1) * locals.var_dnm) + (assign24640_body66_e19867 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.1) * locals.var_dnm) + (assign24640_body66_e19867 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.1) * locals.var_dnm) + (assign24640_body66_e19867 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 0.1) * locals.var_dnm) + (assign24640_body66_e19867 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
            locals.var_tmf0 = assign24640_body66_e19871;
            locals.var_tmf0_dn0 = assign24640_body66_e19871_d_n0;
            locals.var_tmf0_dn2 = assign24640_body66_e19871_d_n2;
            locals.var_tmf0_dn4 = assign24640_body66_e19871_d_n4;
            locals.var_tmf0_dn5 = assign24640_body66_e19871_d_n5;
            locals.var_tmf0_dn6 = assign24640_body66_e19871_d_n6;
            locals.var_tmf0_dn7 = assign24640_body66_e19871_d_n7;
            locals.var_tmf0_dn8 = assign24640_body66_e19871_d_n8;
            locals.var_tmf0_dn9 = assign24640_body66_e19871_d_n9;
            locals.var_tmf0_dn10 = assign24640_body66_e19871_d_n10;
            locals.var_tmf0_dn11 = assign24640_body66_e19871_d_n11;
            locals.var_tmf0_dn14 = assign24640_body66_e19871_d_n14;
            locals.var_tmf0_rv = 0.0;
            let (assign24640_body67_e19887, assign24640_body67_e19887_d_n0, assign24640_body67_e19887_d_n2, assign24640_body67_e19887_d_n4, assign24640_body67_e19887_d_n5, assign24640_body67_e19887_d_n6, assign24640_body67_e19887_d_n7, assign24640_body67_e19887_d_n8, assign24640_body67_e19887_d_n9, assign24640_body67_e19887_d_n10, assign24640_body67_e19887_d_n11, assign24640_body67_e19887_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard570 != 0.0)) {
        let assign24640_body67_e19881: f64 = (0.1 * locals.var_xmp);
        let assign24640_body67_e19883: f64 = (assign24640_body67_e19881 * locals.var_dnm);
        let assign24640_body67_e19885: f64 = (assign24640_body67_e19883 / locals.var_arg);
        (assign24640_body67_e19885, ((((((0.1 * locals.var_xmp_dn0) * locals.var_dnm) + (assign24640_body67_e19881 * locals.var_dnm_dn0)) * locals.var_arg) - (assign24640_body67_e19883 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn2) * locals.var_dnm) + (assign24640_body67_e19881 * locals.var_dnm_dn2)) * locals.var_arg) - (assign24640_body67_e19883 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn4) * locals.var_dnm) + (assign24640_body67_e19881 * locals.var_dnm_dn4)) * locals.var_arg) - (assign24640_body67_e19883 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn5) * locals.var_dnm) + (assign24640_body67_e19881 * locals.var_dnm_dn5)) * locals.var_arg) - (assign24640_body67_e19883 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn6) * locals.var_dnm) + (assign24640_body67_e19881 * locals.var_dnm_dn6)) * locals.var_arg) - (assign24640_body67_e19883 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn7) * locals.var_dnm) + (assign24640_body67_e19881 * locals.var_dnm_dn7)) * locals.var_arg) - (assign24640_body67_e19883 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn8) * locals.var_dnm) + (assign24640_body67_e19881 * locals.var_dnm_dn8)) * locals.var_arg) - (assign24640_body67_e19883 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn9) * locals.var_dnm) + (assign24640_body67_e19881 * locals.var_dnm_dn9)) * locals.var_arg) - (assign24640_body67_e19883 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn10) * locals.var_dnm) + (assign24640_body67_e19881 * locals.var_dnm_dn10)) * locals.var_arg) - (assign24640_body67_e19883 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn11) * locals.var_dnm) + (assign24640_body67_e19881 * locals.var_dnm_dn11)) * locals.var_arg) - (assign24640_body67_e19883 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn14) * locals.var_dnm) + (assign24640_body67_e19881 * locals.var_dnm_dn14)) * locals.var_arg) - (assign24640_body67_e19883 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
            locals.var_t7 = assign24640_body67_e19887;
            locals.var_t7_dn0 = assign24640_body67_e19887_d_n0;
            locals.var_t7_dn2 = assign24640_body67_e19887_d_n2;
            locals.var_t7_dn4 = assign24640_body67_e19887_d_n4;
            locals.var_t7_dn5 = assign24640_body67_e19887_d_n5;
            locals.var_t7_dn6 = assign24640_body67_e19887_d_n6;
            locals.var_t7_dn7 = assign24640_body67_e19887_d_n7;
            locals.var_t7_dn8 = assign24640_body67_e19887_d_n8;
            locals.var_t7_dn9 = assign24640_body67_e19887_d_n9;
            locals.var_t7_dn10 = assign24640_body67_e19887_d_n10;
            locals.var_t7_dn11 = assign24640_body67_e19887_d_n11;
            locals.var_t7_dn14 = assign24640_body67_e19887_d_n14;
            locals.var_t7_rv = 0.0;
            let (assign24640_body68_e19901, assign24640_body68_e19901_d_n0, assign24640_body68_e19901_d_n2, assign24640_body68_e19901_d_n4, assign24640_body68_e19901_d_n5, assign24640_body68_e19901_d_n6, assign24640_body68_e19901_d_n7, assign24640_body68_e19901_d_n8, assign24640_body68_e19901_d_n9, assign24640_body68_e19901_d_n10, assign24640_body68_e19901_d_n11, assign24640_body68_e19901_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard570 != 0.0)) {
        let assign24640_body68_e19897: f64 = 0.1;
        let assign24640_body68_e19899: f64 = (assign24640_body68_e19897 - locals.var_tmf0);
        (assign24640_body68_e19899, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
            locals.var_t2 = assign24640_body68_e19901;
            locals.var_t2_dn0 = assign24640_body68_e19901_d_n0;
            locals.var_t2_dn2 = assign24640_body68_e19901_d_n2;
            locals.var_t2_dn4 = assign24640_body68_e19901_d_n4;
            locals.var_t2_dn5 = assign24640_body68_e19901_d_n5;
            locals.var_t2_dn6 = assign24640_body68_e19901_d_n6;
            locals.var_t2_dn7 = assign24640_body68_e19901_d_n7;
            locals.var_t2_dn8 = assign24640_body68_e19901_d_n8;
            locals.var_t2_dn9 = assign24640_body68_e19901_d_n9;
            locals.var_t2_dn10 = assign24640_body68_e19901_d_n10;
            locals.var_t2_dn11 = assign24640_body68_e19901_d_n11;
            locals.var_t2_dn14 = assign24640_body68_e19901_d_n14;
            locals.var_t2_rv = 0.0;
            let (assign24640_body69_e19911, assign24640_body69_e19911_d_n0, assign24640_body69_e19911_d_n2, assign24640_body69_e19911_d_n4, assign24640_body69_e19911_d_n5, assign24640_body69_e19911_d_n6, assign24640_body69_e19911_d_n7, assign24640_body69_e19911_d_n8, assign24640_body69_e19911_d_n9, assign24640_body69_e19911_d_n10, assign24640_body69_e19911_d_n11, assign24640_body69_e19911_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard570 != 0.0)) {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
            locals.var_t7 = assign24640_body69_e19911;
            locals.var_t7_dn0 = assign24640_body69_e19911_d_n0;
            locals.var_t7_dn2 = assign24640_body69_e19911_d_n2;
            locals.var_t7_dn4 = assign24640_body69_e19911_d_n4;
            locals.var_t7_dn5 = assign24640_body69_e19911_d_n5;
            locals.var_t7_dn6 = assign24640_body69_e19911_d_n6;
            locals.var_t7_dn7 = assign24640_body69_e19911_d_n7;
            locals.var_t7_dn8 = assign24640_body69_e19911_d_n8;
            locals.var_t7_dn9 = assign24640_body69_e19911_d_n9;
            locals.var_t7_dn10 = assign24640_body69_e19911_d_n10;
            locals.var_t7_dn11 = assign24640_body69_e19911_d_n11;
            locals.var_t7_dn14 = assign24640_body69_e19911_d_n14;
            locals.var_t7_rv = 0.0;
            let (assign24640_body70_e19922, assign24640_body70_e19922_d_n0, assign24640_body70_e19922_d_n2, assign24640_body70_e19922_d_n4, assign24640_body70_e19922_d_n5, assign24640_body70_e19922_d_n6, assign24640_body70_e19922_d_n7, assign24640_body70_e19922_d_n8, assign24640_body70_e19922_d_n9, assign24640_body70_e19922_d_n10, assign24640_body70_e19922_d_n11, assign24640_body70_e19922_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard570 == 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
            locals.var_t2 = assign24640_body70_e19922;
            locals.var_t2_dn0 = assign24640_body70_e19922_d_n0;
            locals.var_t2_dn2 = assign24640_body70_e19922_d_n2;
            locals.var_t2_dn4 = assign24640_body70_e19922_d_n4;
            locals.var_t2_dn5 = assign24640_body70_e19922_d_n5;
            locals.var_t2_dn6 = assign24640_body70_e19922_d_n6;
            locals.var_t2_dn7 = assign24640_body70_e19922_d_n7;
            locals.var_t2_dn8 = assign24640_body70_e19922_d_n8;
            locals.var_t2_dn9 = assign24640_body70_e19922_d_n9;
            locals.var_t2_dn10 = assign24640_body70_e19922_d_n10;
            locals.var_t2_dn11 = assign24640_body70_e19922_d_n11;
            locals.var_t2_dn14 = assign24640_body70_e19922_d_n14;
            locals.var_t2_rv = 0.0;
            let (assign24640_body71_e19933, assign24640_body71_e19933_d_n0, assign24640_body71_e19933_d_n2, assign24640_body71_e19933_d_n4, assign24640_body71_e19933_d_n5, assign24640_body71_e19933_d_n6, assign24640_body71_e19933_d_n7, assign24640_body71_e19933_d_n8, assign24640_body71_e19933_d_n9, assign24640_body71_e19933_d_n10, assign24640_body71_e19933_d_n11, assign24640_body71_e19933_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard570 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
            locals.var_t7 = assign24640_body71_e19933;
            locals.var_t7_dn0 = assign24640_body71_e19933_d_n0;
            locals.var_t7_dn2 = assign24640_body71_e19933_d_n2;
            locals.var_t7_dn4 = assign24640_body71_e19933_d_n4;
            locals.var_t7_dn5 = assign24640_body71_e19933_d_n5;
            locals.var_t7_dn6 = assign24640_body71_e19933_d_n6;
            locals.var_t7_dn7 = assign24640_body71_e19933_d_n7;
            locals.var_t7_dn8 = assign24640_body71_e19933_d_n8;
            locals.var_t7_dn9 = assign24640_body71_e19933_d_n9;
            locals.var_t7_dn10 = assign24640_body71_e19933_d_n10;
            locals.var_t7_dn11 = assign24640_body71_e19933_d_n11;
            locals.var_t7_dn14 = assign24640_body71_e19933_d_n14;
            locals.var_t7_rv = 0.0;
            let (assign24640_body72_e19944, assign24640_body72_e19944_d_n0, assign24640_body72_e19944_d_n2, assign24640_body72_e19944_d_n4, assign24640_body72_e19944_d_n5, assign24640_body72_e19944_d_n6, assign24640_body72_e19944_d_n7, assign24640_body72_e19944_d_n8, assign24640_body72_e19944_d_n9, assign24640_body72_e19944_d_n10, assign24640_body72_e19944_d_n11, assign24640_body72_e19944_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) {
        let assign24640_body72_e19941: f64 = (locals.var_c_2esipq_nsub * locals.var_t2);
        let assign24640_body72_e19942: f64 = (assign24640_body72_e19941).sqrt();
        (assign24640_body72_e19942, (((locals.var_c_2esipq_nsub_dn0 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn0)) / (2.0 * assign24640_body72_e19942)), (((locals.var_c_2esipq_nsub_dn2 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn2)) / (2.0 * assign24640_body72_e19942)), (((locals.var_c_2esipq_nsub_dn4 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn4)) / (2.0 * assign24640_body72_e19942)), (((locals.var_c_2esipq_nsub_dn5 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn5)) / (2.0 * assign24640_body72_e19942)), (((locals.var_c_2esipq_nsub_dn6 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn6)) / (2.0 * assign24640_body72_e19942)), (((locals.var_c_2esipq_nsub_dn7 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn7)) / (2.0 * assign24640_body72_e19942)), (((locals.var_c_2esipq_nsub_dn8 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn8)) / (2.0 * assign24640_body72_e19942)), (((locals.var_c_2esipq_nsub_dn9 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn9)) / (2.0 * assign24640_body72_e19942)), (((locals.var_c_2esipq_nsub_dn10 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn10)) / (2.0 * assign24640_body72_e19942)), (((locals.var_c_2esipq_nsub_dn11 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn11)) / (2.0 * assign24640_body72_e19942)), (((locals.var_c_2esipq_nsub_dn14 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn14)) / (2.0 * assign24640_body72_e19942)),)
    } else {
        (locals.var_w_sub0, locals.var_w_sub0_dn0, locals.var_w_sub0_dn2, locals.var_w_sub0_dn4, locals.var_w_sub0_dn5, locals.var_w_sub0_dn6, locals.var_w_sub0_dn7, locals.var_w_sub0_dn8, locals.var_w_sub0_dn9, locals.var_w_sub0_dn10, locals.var_w_sub0_dn11, locals.var_w_sub0_dn14,)
    }
};
            locals.var_w_sub0 = assign24640_body72_e19944;
            locals.var_w_sub0_dn0 = assign24640_body72_e19944_d_n0;
            locals.var_w_sub0_dn2 = assign24640_body72_e19944_d_n2;
            locals.var_w_sub0_dn4 = assign24640_body72_e19944_d_n4;
            locals.var_w_sub0_dn5 = assign24640_body72_e19944_d_n5;
            locals.var_w_sub0_dn6 = assign24640_body72_e19944_d_n6;
            locals.var_w_sub0_dn7 = assign24640_body72_e19944_d_n7;
            locals.var_w_sub0_dn8 = assign24640_body72_e19944_d_n8;
            locals.var_w_sub0_dn9 = assign24640_body72_e19944_d_n9;
            locals.var_w_sub0_dn10 = assign24640_body72_e19944_d_n10;
            locals.var_w_sub0_dn11 = assign24640_body72_e19944_d_n11;
            locals.var_w_sub0_dn14 = assign24640_body72_e19944_d_n14;
            locals.var_w_sub0_rv = 0.0;
            let (assign24640_body73_e19954, assign24640_body73_e19954_d_n0, assign24640_body73_e19954_d_n2, assign24640_body73_e19954_d_n4, assign24640_body73_e19954_d_n5, assign24640_body73_e19954_d_n6, assign24640_body73_e19954_d_n7, assign24640_body73_e19954_d_n8, assign24640_body73_e19954_d_n9, assign24640_body73_e19954_d_n10, assign24640_body73_e19954_d_n11, assign24640_body73_e19954_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) {
        let assign24640_body73_e19952: f64 = (locals.var_w_b0 * locals.var_q_ndepm);
        (assign24640_body73_e19952, ((locals.var_w_b0_dn0 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn0)), ((locals.var_w_b0_dn2 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn2)), ((locals.var_w_b0_dn4 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn4)), ((locals.var_w_b0_dn5 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn5)), ((locals.var_w_b0_dn6 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn6)), ((locals.var_w_b0_dn7 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn7)), ((locals.var_w_b0_dn8 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn8)), ((locals.var_w_b0_dn9 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn9)), ((locals.var_w_b0_dn10 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn10)), ((locals.var_w_b0_dn11 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn11)), ((locals.var_w_b0_dn14 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn14)),)
    } else {
        (locals.var_q_b0_dep, locals.var_q_b0_dep_dn0, locals.var_q_b0_dep_dn2, locals.var_q_b0_dep_dn4, locals.var_q_b0_dep_dn5, locals.var_q_b0_dep_dn6, locals.var_q_b0_dep_dn7, locals.var_q_b0_dep_dn8, locals.var_q_b0_dep_dn9, locals.var_q_b0_dep_dn10, locals.var_q_b0_dep_dn11, locals.var_q_b0_dep_dn14,)
    }
};
            locals.var_q_b0_dep = assign24640_body73_e19954;
            locals.var_q_b0_dep_dn0 = assign24640_body73_e19954_d_n0;
            locals.var_q_b0_dep_dn2 = assign24640_body73_e19954_d_n2;
            locals.var_q_b0_dep_dn4 = assign24640_body73_e19954_d_n4;
            locals.var_q_b0_dep_dn5 = assign24640_body73_e19954_d_n5;
            locals.var_q_b0_dep_dn6 = assign24640_body73_e19954_d_n6;
            locals.var_q_b0_dep_dn7 = assign24640_body73_e19954_d_n7;
            locals.var_q_b0_dep_dn8 = assign24640_body73_e19954_d_n8;
            locals.var_q_b0_dep_dn9 = assign24640_body73_e19954_d_n9;
            locals.var_q_b0_dep_dn10 = assign24640_body73_e19954_d_n10;
            locals.var_q_b0_dep_dn11 = assign24640_body73_e19954_d_n11;
            locals.var_q_b0_dep_dn14 = assign24640_body73_e19954_d_n14;
            locals.var_q_b0_dep_rv = 0.0;
            let (assign24640_body74_e19967, assign24640_body74_e19967_d_n0, assign24640_body74_e19967_d_n2, assign24640_body74_e19967_d_n4, assign24640_body74_e19967_d_n5, assign24640_body74_e19967_d_n6, assign24640_body74_e19967_d_n7, assign24640_body74_e19967_d_n8, assign24640_body74_e19967_d_n9, assign24640_body74_e19967_d_n10, assign24640_body74_e19967_d_n11, assign24640_body74_e19967_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) {
        let assign24640_body74_e19961: f64 = (-1.034943e-10);
        let assign24640_body74_e19963: f64 = (assign24640_body74_e19961 / locals.var_w_b0);
        let assign24640_body74_e19965: f64 = (assign24640_body74_e19963 * locals.var_t0);
        (assign24640_body74_e19965, (((-((assign24640_body74_e19961 * locals.var_w_b0_dn0) / (locals.var_w_b0 * locals.var_w_b0))) * locals.var_t0) + (assign24640_body74_e19963 * locals.var_t0_dn0)), (((-((assign24640_body74_e19961 * locals.var_w_b0_dn2) / (locals.var_w_b0 * locals.var_w_b0))) * locals.var_t0) + (assign24640_body74_e19963 * locals.var_t0_dn2)), (((-((assign24640_body74_e19961 * locals.var_w_b0_dn4) / (locals.var_w_b0 * locals.var_w_b0))) * locals.var_t0) + (assign24640_body74_e19963 * locals.var_t0_dn4)), (((-((assign24640_body74_e19961 * locals.var_w_b0_dn5) / (locals.var_w_b0 * locals.var_w_b0))) * locals.var_t0) + (assign24640_body74_e19963 * locals.var_t0_dn5)), (((-((assign24640_body74_e19961 * locals.var_w_b0_dn6) / (locals.var_w_b0 * locals.var_w_b0))) * locals.var_t0) + (assign24640_body74_e19963 * locals.var_t0_dn6)), (((-((assign24640_body74_e19961 * locals.var_w_b0_dn7) / (locals.var_w_b0 * locals.var_w_b0))) * locals.var_t0) + (assign24640_body74_e19963 * locals.var_t0_dn7)), (((-((assign24640_body74_e19961 * locals.var_w_b0_dn8) / (locals.var_w_b0 * locals.var_w_b0))) * locals.var_t0) + (assign24640_body74_e19963 * locals.var_t0_dn8)), (((-((assign24640_body74_e19961 * locals.var_w_b0_dn9) / (locals.var_w_b0 * locals.var_w_b0))) * locals.var_t0) + (assign24640_body74_e19963 * locals.var_t0_dn9)), (((-((assign24640_body74_e19961 * locals.var_w_b0_dn10) / (locals.var_w_b0 * locals.var_w_b0))) * locals.var_t0) + (assign24640_body74_e19963 * locals.var_t0_dn10)), (((-((assign24640_body74_e19961 * locals.var_w_b0_dn11) / (locals.var_w_b0 * locals.var_w_b0))) * locals.var_t0) + (assign24640_body74_e19963 * locals.var_t0_dn11)), (((-((assign24640_body74_e19961 * locals.var_w_b0_dn14) / (locals.var_w_b0 * locals.var_w_b0))) * locals.var_t0) + (assign24640_body74_e19963 * locals.var_t0_dn14)),)
    } else {
        (locals.var_q_b0_dep_dpd, locals.var_q_b0_dep_dpd_dn0, locals.var_q_b0_dep_dpd_dn2, locals.var_q_b0_dep_dpd_dn4, locals.var_q_b0_dep_dpd_dn5, locals.var_q_b0_dep_dpd_dn6, locals.var_q_b0_dep_dpd_dn7, locals.var_q_b0_dep_dpd_dn8, locals.var_q_b0_dep_dpd_dn9, locals.var_q_b0_dep_dpd_dn10, locals.var_q_b0_dep_dpd_dn11, locals.var_q_b0_dep_dpd_dn14,)
    }
};
            locals.var_q_b0_dep_dpd = assign24640_body74_e19967;
            locals.var_q_b0_dep_dpd_dn0 = assign24640_body74_e19967_d_n0;
            locals.var_q_b0_dep_dpd_dn2 = assign24640_body74_e19967_d_n2;
            locals.var_q_b0_dep_dpd_dn4 = assign24640_body74_e19967_d_n4;
            locals.var_q_b0_dep_dpd_dn5 = assign24640_body74_e19967_d_n5;
            locals.var_q_b0_dep_dpd_dn6 = assign24640_body74_e19967_d_n6;
            locals.var_q_b0_dep_dpd_dn7 = assign24640_body74_e19967_d_n7;
            locals.var_q_b0_dep_dpd_dn8 = assign24640_body74_e19967_d_n8;
            locals.var_q_b0_dep_dpd_dn9 = assign24640_body74_e19967_d_n9;
            locals.var_q_b0_dep_dpd_dn10 = assign24640_body74_e19967_d_n10;
            locals.var_q_b0_dep_dpd_dn11 = assign24640_body74_e19967_d_n11;
            locals.var_q_b0_dep_dpd_dn14 = assign24640_body74_e19967_d_n14;
            locals.var_q_b0_dep_dpd_rv = 0.0;
            let (assign24640_body75_e19978, assign24640_body75_e19978_d_n0, assign24640_body75_e19978_d_n2, assign24640_body75_e19978_d_n4, assign24640_body75_e19978_d_n5, assign24640_body75_e19978_d_n6, assign24640_body75_e19978_d_n7, assign24640_body75_e19978_d_n8, assign24640_body75_e19978_d_n9, assign24640_body75_e19978_d_n10, assign24640_body75_e19978_d_n11, assign24640_body75_e19978_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) {
        let assign24640_body75_e19974: f64 = (-locals.var_w_sub0);
        let assign24640_body75_e19976: f64 = (assign24640_body75_e19974 * locals.var_q_nsub__blk548);
        (assign24640_body75_e19976, (((-locals.var_w_sub0_dn0) * locals.var_q_nsub__blk548) + (assign24640_body75_e19974 * locals.var_q_nsub__blk548_dn0)), (((-locals.var_w_sub0_dn2) * locals.var_q_nsub__blk548) + (assign24640_body75_e19974 * locals.var_q_nsub__blk548_dn2)), (((-locals.var_w_sub0_dn4) * locals.var_q_nsub__blk548) + (assign24640_body75_e19974 * locals.var_q_nsub__blk548_dn4)), (((-locals.var_w_sub0_dn5) * locals.var_q_nsub__blk548) + (assign24640_body75_e19974 * locals.var_q_nsub__blk548_dn5)), (((-locals.var_w_sub0_dn6) * locals.var_q_nsub__blk548) + (assign24640_body75_e19974 * locals.var_q_nsub__blk548_dn6)), (((-locals.var_w_sub0_dn7) * locals.var_q_nsub__blk548) + (assign24640_body75_e19974 * locals.var_q_nsub__blk548_dn7)), (((-locals.var_w_sub0_dn8) * locals.var_q_nsub__blk548) + (assign24640_body75_e19974 * locals.var_q_nsub__blk548_dn8)), (((-locals.var_w_sub0_dn9) * locals.var_q_nsub__blk548) + (assign24640_body75_e19974 * locals.var_q_nsub__blk548_dn9)), (((-locals.var_w_sub0_dn10) * locals.var_q_nsub__blk548) + (assign24640_body75_e19974 * locals.var_q_nsub__blk548_dn10)), (((-locals.var_w_sub0_dn11) * locals.var_q_nsub__blk548) + (assign24640_body75_e19974 * locals.var_q_nsub__blk548_dn11)), (((-locals.var_w_sub0_dn14) * locals.var_q_nsub__blk548) + (assign24640_body75_e19974 * locals.var_q_nsub__blk548_dn14)),)
    } else {
        (locals.var_q_sub0_dep, locals.var_q_sub0_dep_dn0, locals.var_q_sub0_dep_dn2, locals.var_q_sub0_dep_dn4, locals.var_q_sub0_dep_dn5, locals.var_q_sub0_dep_dn6, locals.var_q_sub0_dep_dn7, locals.var_q_sub0_dep_dn8, locals.var_q_sub0_dep_dn9, locals.var_q_sub0_dep_dn10, locals.var_q_sub0_dep_dn11, locals.var_q_sub0_dep_dn14,)
    }
};
            locals.var_q_sub0_dep = assign24640_body75_e19978;
            locals.var_q_sub0_dep_dn0 = assign24640_body75_e19978_d_n0;
            locals.var_q_sub0_dep_dn2 = assign24640_body75_e19978_d_n2;
            locals.var_q_sub0_dep_dn4 = assign24640_body75_e19978_d_n4;
            locals.var_q_sub0_dep_dn5 = assign24640_body75_e19978_d_n5;
            locals.var_q_sub0_dep_dn6 = assign24640_body75_e19978_d_n6;
            locals.var_q_sub0_dep_dn7 = assign24640_body75_e19978_d_n7;
            locals.var_q_sub0_dep_dn8 = assign24640_body75_e19978_d_n8;
            locals.var_q_sub0_dep_dn9 = assign24640_body75_e19978_d_n9;
            locals.var_q_sub0_dep_dn10 = assign24640_body75_e19978_d_n10;
            locals.var_q_sub0_dep_dn11 = assign24640_body75_e19978_d_n11;
            locals.var_q_sub0_dep_dn14 = assign24640_body75_e19978_d_n14;
            locals.var_q_sub0_dep_rv = 0.0;
            let (assign24640_body76_e19991, assign24640_body76_e19991_d_n0, assign24640_body76_e19991_d_n2, assign24640_body76_e19991_d_n4, assign24640_body76_e19991_d_n5, assign24640_body76_e19991_d_n6, assign24640_body76_e19991_d_n7, assign24640_body76_e19991_d_n8, assign24640_body76_e19991_d_n9, assign24640_body76_e19991_d_n10, assign24640_body76_e19991_d_n11, assign24640_body76_e19991_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) {
        let assign24640_body76_e19985: f64 = (-1.034943e-10);
        let assign24640_body76_e19987: f64 = (assign24640_body76_e19985 / locals.var_w_sub0);
        let assign24640_body76_e19989: f64 = (assign24640_body76_e19987 * locals.var_t7);
        (assign24640_body76_e19989, (((-((assign24640_body76_e19985 * locals.var_w_sub0_dn0) / (locals.var_w_sub0 * locals.var_w_sub0))) * locals.var_t7) + (assign24640_body76_e19987 * locals.var_t7_dn0)), (((-((assign24640_body76_e19985 * locals.var_w_sub0_dn2) / (locals.var_w_sub0 * locals.var_w_sub0))) * locals.var_t7) + (assign24640_body76_e19987 * locals.var_t7_dn2)), (((-((assign24640_body76_e19985 * locals.var_w_sub0_dn4) / (locals.var_w_sub0 * locals.var_w_sub0))) * locals.var_t7) + (assign24640_body76_e19987 * locals.var_t7_dn4)), (((-((assign24640_body76_e19985 * locals.var_w_sub0_dn5) / (locals.var_w_sub0 * locals.var_w_sub0))) * locals.var_t7) + (assign24640_body76_e19987 * locals.var_t7_dn5)), (((-((assign24640_body76_e19985 * locals.var_w_sub0_dn6) / (locals.var_w_sub0 * locals.var_w_sub0))) * locals.var_t7) + (assign24640_body76_e19987 * locals.var_t7_dn6)), (((-((assign24640_body76_e19985 * locals.var_w_sub0_dn7) / (locals.var_w_sub0 * locals.var_w_sub0))) * locals.var_t7) + (assign24640_body76_e19987 * locals.var_t7_dn7)), (((-((assign24640_body76_e19985 * locals.var_w_sub0_dn8) / (locals.var_w_sub0 * locals.var_w_sub0))) * locals.var_t7) + (assign24640_body76_e19987 * locals.var_t7_dn8)), (((-((assign24640_body76_e19985 * locals.var_w_sub0_dn9) / (locals.var_w_sub0 * locals.var_w_sub0))) * locals.var_t7) + (assign24640_body76_e19987 * locals.var_t7_dn9)), (((-((assign24640_body76_e19985 * locals.var_w_sub0_dn10) / (locals.var_w_sub0 * locals.var_w_sub0))) * locals.var_t7) + (assign24640_body76_e19987 * locals.var_t7_dn10)), (((-((assign24640_body76_e19985 * locals.var_w_sub0_dn11) / (locals.var_w_sub0 * locals.var_w_sub0))) * locals.var_t7) + (assign24640_body76_e19987 * locals.var_t7_dn11)), (((-((assign24640_body76_e19985 * locals.var_w_sub0_dn14) / (locals.var_w_sub0 * locals.var_w_sub0))) * locals.var_t7) + (assign24640_body76_e19987 * locals.var_t7_dn14)),)
    } else {
        (locals.var_q_sub0_dep_dpd, locals.var_q_sub0_dep_dpd_dn0, locals.var_q_sub0_dep_dpd_dn2, locals.var_q_sub0_dep_dpd_dn4, locals.var_q_sub0_dep_dpd_dn5, locals.var_q_sub0_dep_dpd_dn6, locals.var_q_sub0_dep_dpd_dn7, locals.var_q_sub0_dep_dpd_dn8, locals.var_q_sub0_dep_dpd_dn9, locals.var_q_sub0_dep_dpd_dn10, locals.var_q_sub0_dep_dpd_dn11, locals.var_q_sub0_dep_dpd_dn14,)
    }
};
            locals.var_q_sub0_dep_dpd = assign24640_body76_e19991;
            locals.var_q_sub0_dep_dpd_dn0 = assign24640_body76_e19991_d_n0;
            locals.var_q_sub0_dep_dpd_dn2 = assign24640_body76_e19991_d_n2;
            locals.var_q_sub0_dep_dpd_dn4 = assign24640_body76_e19991_d_n4;
            locals.var_q_sub0_dep_dpd_dn5 = assign24640_body76_e19991_d_n5;
            locals.var_q_sub0_dep_dpd_dn6 = assign24640_body76_e19991_d_n6;
            locals.var_q_sub0_dep_dpd_dn7 = assign24640_body76_e19991_d_n7;
            locals.var_q_sub0_dep_dpd_dn8 = assign24640_body76_e19991_d_n8;
            locals.var_q_sub0_dep_dpd_dn9 = assign24640_body76_e19991_d_n9;
            locals.var_q_sub0_dep_dpd_dn10 = assign24640_body76_e19991_d_n10;
            locals.var_q_sub0_dep_dpd_dn11 = assign24640_body76_e19991_d_n11;
            locals.var_q_sub0_dep_dpd_dn14 = assign24640_body76_e19991_d_n14;
            locals.var_q_sub0_dep_dpd_rv = 0.0;
            let (assign24640_body77_e20007, assign24640_body77_e20007_d_n0, assign24640_body77_e20007_d_n2, assign24640_body77_e20007_d_n4, assign24640_body77_e20007_d_n5, assign24640_body77_e20007_d_n6, assign24640_body77_e20007_d_n7, assign24640_body77_e20007_d_n8, assign24640_body77_e20007_d_n9, assign24640_body77_e20007_d_n10, assign24640_body77_e20007_d_n11, assign24640_body77_e20007_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) {
        let assign24640_body77_e20000: f64 = (locals.var_vgp0 - locals.var_phi_b0_dep);
        let assign24640_body77_e20001: f64 = (locals.var_cox * assign24640_body77_e20000);
        let assign24640_body77_e20003: f64 = (assign24640_body77_e20001 + locals.var_q_b0_dep);
        let assign24640_body77_e20005: f64 = (assign24640_body77_e20003 + locals.var_q_sub0_dep);
        (assign24640_body77_e20005, ((((locals.var_cox_dn0 * assign24640_body77_e20000) + (locals.var_cox * (locals.var_vgp0_dn0 - locals.var_phi_b0_dep_dn0))) + locals.var_q_b0_dep_dn0) + locals.var_q_sub0_dep_dn0), ((((locals.var_cox_dn2 * assign24640_body77_e20000) + (locals.var_cox * (locals.var_vgp0_dn2 - locals.var_phi_b0_dep_dn2))) + locals.var_q_b0_dep_dn2) + locals.var_q_sub0_dep_dn2), ((((locals.var_cox_dn4 * assign24640_body77_e20000) + (locals.var_cox * (locals.var_vgp0_dn4 - locals.var_phi_b0_dep_dn4))) + locals.var_q_b0_dep_dn4) + locals.var_q_sub0_dep_dn4), ((((locals.var_cox_dn5 * assign24640_body77_e20000) + (locals.var_cox * (locals.var_vgp0_dn5 - locals.var_phi_b0_dep_dn5))) + locals.var_q_b0_dep_dn5) + locals.var_q_sub0_dep_dn5), ((((locals.var_cox_dn6 * assign24640_body77_e20000) + (locals.var_cox * (locals.var_vgp0_dn6 - locals.var_phi_b0_dep_dn6))) + locals.var_q_b0_dep_dn6) + locals.var_q_sub0_dep_dn6), ((((locals.var_cox_dn7 * assign24640_body77_e20000) + (locals.var_cox * (locals.var_vgp0_dn7 - locals.var_phi_b0_dep_dn7))) + locals.var_q_b0_dep_dn7) + locals.var_q_sub0_dep_dn7), ((((locals.var_cox_dn8 * assign24640_body77_e20000) + (locals.var_cox * (locals.var_vgp0_dn8 - locals.var_phi_b0_dep_dn8))) + locals.var_q_b0_dep_dn8) + locals.var_q_sub0_dep_dn8), ((((locals.var_cox_dn9 * assign24640_body77_e20000) + (locals.var_cox * (locals.var_vgp0_dn9 - locals.var_phi_b0_dep_dn9))) + locals.var_q_b0_dep_dn9) + locals.var_q_sub0_dep_dn9), ((((locals.var_cox_dn10 * assign24640_body77_e20000) + (locals.var_cox * (locals.var_vgp0_dn10 - locals.var_phi_b0_dep_dn10))) + locals.var_q_b0_dep_dn10) + locals.var_q_sub0_dep_dn10), ((((locals.var_cox_dn11 * assign24640_body77_e20000) + (locals.var_cox * (locals.var_vgp0_dn11 - locals.var_phi_b0_dep_dn11))) + locals.var_q_b0_dep_dn11) + locals.var_q_sub0_dep_dn11), ((((locals.var_cox_dn14 * assign24640_body77_e20000) + (locals.var_cox * (locals.var_vgp0_dn14 - locals.var_phi_b0_dep_dn14))) + locals.var_q_b0_dep_dn14) + locals.var_q_sub0_dep_dn14),)
    } else {
        (locals.var_y1, locals.var_y1_dn0, locals.var_y1_dn2, locals.var_y1_dn4, locals.var_y1_dn5, locals.var_y1_dn6, locals.var_y1_dn7, locals.var_y1_dn8, locals.var_y1_dn9, locals.var_y1_dn10, locals.var_y1_dn11, locals.var_y1_dn14,)
    }
};
            locals.var_y1 = assign24640_body77_e20007;
            locals.var_y1_dn0 = assign24640_body77_e20007_d_n0;
            locals.var_y1_dn2 = assign24640_body77_e20007_d_n2;
            locals.var_y1_dn4 = assign24640_body77_e20007_d_n4;
            locals.var_y1_dn5 = assign24640_body77_e20007_d_n5;
            locals.var_y1_dn6 = assign24640_body77_e20007_d_n6;
            locals.var_y1_dn7 = assign24640_body77_e20007_d_n7;
            locals.var_y1_dn8 = assign24640_body77_e20007_d_n8;
            locals.var_y1_dn9 = assign24640_body77_e20007_d_n9;
            locals.var_y1_dn10 = assign24640_body77_e20007_d_n10;
            locals.var_y1_dn11 = assign24640_body77_e20007_d_n11;
            locals.var_y1_dn14 = assign24640_body77_e20007_d_n14;
            locals.var_y1_rv = 0.0;
            let (assign24640_body78_e20015, assign24640_body78_e20015_d_n0, assign24640_body78_e20015_d_n2, assign24640_body78_e20015_d_n4, assign24640_body78_e20015_d_n5, assign24640_body78_e20015_d_n6, assign24640_body78_e20015_d_n7, assign24640_body78_e20015_d_n8, assign24640_body78_e20015_d_n9, assign24640_body78_e20015_d_n10, assign24640_body78_e20015_d_n11, assign24640_body78_e20015_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) {
        (locals.var_cox, locals.var_cox_dn0, locals.var_cox_dn2, locals.var_cox_dn4, locals.var_cox_dn5, locals.var_cox_dn6, locals.var_cox_dn7, locals.var_cox_dn8, locals.var_cox_dn9, locals.var_cox_dn10, locals.var_cox_dn11, locals.var_cox_dn14,)
    } else {
        (locals.var_y11, locals.var_y11_dn0, locals.var_y11_dn2, locals.var_y11_dn4, locals.var_y11_dn5, locals.var_y11_dn6, locals.var_y11_dn7, locals.var_y11_dn8, locals.var_y11_dn9, locals.var_y11_dn10, locals.var_y11_dn11, locals.var_y11_dn14,)
    }
};
            locals.var_y11 = assign24640_body78_e20015;
            locals.var_y11_dn0 = assign24640_body78_e20015_d_n0;
            locals.var_y11_dn2 = assign24640_body78_e20015_d_n2;
            locals.var_y11_dn4 = assign24640_body78_e20015_d_n4;
            locals.var_y11_dn5 = assign24640_body78_e20015_d_n5;
            locals.var_y11_dn6 = assign24640_body78_e20015_d_n6;
            locals.var_y11_dn7 = assign24640_body78_e20015_d_n7;
            locals.var_y11_dn8 = assign24640_body78_e20015_d_n8;
            locals.var_y11_dn9 = assign24640_body78_e20015_d_n9;
            locals.var_y11_dn10 = assign24640_body78_e20015_d_n10;
            locals.var_y11_dn11 = assign24640_body78_e20015_d_n11;
            locals.var_y11_dn14 = assign24640_body78_e20015_d_n14;
            locals.var_y11_rv = 0.0;
            let (assign24640_body79_e20025, assign24640_body79_e20025_d_n0, assign24640_body79_e20025_d_n2, assign24640_body79_e20025_d_n4, assign24640_body79_e20025_d_n5, assign24640_body79_e20025_d_n6, assign24640_body79_e20025_d_n7, assign24640_body79_e20025_d_n8, assign24640_body79_e20025_d_n9, assign24640_body79_e20025_d_n10, assign24640_body79_e20025_d_n11, assign24640_body79_e20025_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) {
        let assign24640_body79_e20023: f64 = (locals.var_q_b0_dep_dpd + locals.var_q_sub0_dep_dpd);
        (assign24640_body79_e20023, (locals.var_q_b0_dep_dpd_dn0 + locals.var_q_sub0_dep_dpd_dn0), (locals.var_q_b0_dep_dpd_dn2 + locals.var_q_sub0_dep_dpd_dn2), (locals.var_q_b0_dep_dpd_dn4 + locals.var_q_sub0_dep_dpd_dn4), (locals.var_q_b0_dep_dpd_dn5 + locals.var_q_sub0_dep_dpd_dn5), (locals.var_q_b0_dep_dpd_dn6 + locals.var_q_sub0_dep_dpd_dn6), (locals.var_q_b0_dep_dpd_dn7 + locals.var_q_sub0_dep_dpd_dn7), (locals.var_q_b0_dep_dpd_dn8 + locals.var_q_sub0_dep_dpd_dn8), (locals.var_q_b0_dep_dpd_dn9 + locals.var_q_sub0_dep_dpd_dn9), (locals.var_q_b0_dep_dpd_dn10 + locals.var_q_sub0_dep_dpd_dn10), (locals.var_q_b0_dep_dpd_dn11 + locals.var_q_sub0_dep_dpd_dn11), (locals.var_q_b0_dep_dpd_dn14 + locals.var_q_sub0_dep_dpd_dn14),)
    } else {
        (locals.var_y12, locals.var_y12_dn0, locals.var_y12_dn2, locals.var_y12_dn4, locals.var_y12_dn5, locals.var_y12_dn6, locals.var_y12_dn7, locals.var_y12_dn8, locals.var_y12_dn9, locals.var_y12_dn10, locals.var_y12_dn11, locals.var_y12_dn14,)
    }
};
            locals.var_y12 = assign24640_body79_e20025;
            locals.var_y12_dn0 = assign24640_body79_e20025_d_n0;
            locals.var_y12_dn2 = assign24640_body79_e20025_d_n2;
            locals.var_y12_dn4 = assign24640_body79_e20025_d_n4;
            locals.var_y12_dn5 = assign24640_body79_e20025_d_n5;
            locals.var_y12_dn6 = assign24640_body79_e20025_d_n6;
            locals.var_y12_dn7 = assign24640_body79_e20025_d_n7;
            locals.var_y12_dn8 = assign24640_body79_e20025_d_n8;
            locals.var_y12_dn9 = assign24640_body79_e20025_d_n9;
            locals.var_y12_dn10 = assign24640_body79_e20025_d_n10;
            locals.var_y12_dn11 = assign24640_body79_e20025_d_n11;
            locals.var_y12_dn14 = assign24640_body79_e20025_d_n14;
            locals.var_y12_rv = 0.0;
            let (assign24640_body80_e20043, assign24640_body80_e20043_d_n0, assign24640_body80_e20043_d_n2, assign24640_body80_e20043_d_n4, assign24640_body80_e20043_d_n5, assign24640_body80_e20043_d_n6, assign24640_body80_e20043_d_n7, assign24640_body80_e20043_d_n8, assign24640_body80_e20043_d_n9, assign24640_body80_e20043_d_n10, assign24640_body80_e20043_d_n11, assign24640_body80_e20043_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) {
        let assign24640_body80_e20035: f64 = (locals.var_ndepmpnsub * locals.var_phi_b0_dep);
        let assign24640_body80_e20037: f64 = (assign24640_body80_e20035 + locals.var_vbscl__blk439);
        let assign24640_body80_e20039: f64 = (assign24640_body80_e20037 - locals.var_vbi_dep);
        let assign24640_body80_e20040: f64 = (locals.var_ndepmpnsub_inv1 * assign24640_body80_e20039);
        let assign24640_body80_e20041: f64 = (locals.var_phi_j0_dep - assign24640_body80_e20040);
        (assign24640_body80_e20041, (locals.var_phi_j0_dep_dn0 - ((locals.var_ndepmpnsub_inv1_dn0 * assign24640_body80_e20039) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn0 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn0)) + locals.var_vbscl__blk439_dn0) - locals.var_vbi_dep_dn0)))), (locals.var_phi_j0_dep_dn2 - ((locals.var_ndepmpnsub_inv1_dn2 * assign24640_body80_e20039) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn2 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn2)) + locals.var_vbscl__blk439_dn2) - locals.var_vbi_dep_dn2)))), (locals.var_phi_j0_dep_dn4 - ((locals.var_ndepmpnsub_inv1_dn4 * assign24640_body80_e20039) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn4 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn4)) + locals.var_vbscl__blk439_dn4) - locals.var_vbi_dep_dn4)))), (locals.var_phi_j0_dep_dn5 - ((locals.var_ndepmpnsub_inv1_dn5 * assign24640_body80_e20039) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn5 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn5)) + locals.var_vbscl__blk439_dn5) - locals.var_vbi_dep_dn5)))), (locals.var_phi_j0_dep_dn6 - ((locals.var_ndepmpnsub_inv1_dn6 * assign24640_body80_e20039) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn6 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn6)) + locals.var_vbscl__blk439_dn6) - locals.var_vbi_dep_dn6)))), (locals.var_phi_j0_dep_dn7 - ((locals.var_ndepmpnsub_inv1_dn7 * assign24640_body80_e20039) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn7 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn7)) + locals.var_vbscl__blk439_dn7) - locals.var_vbi_dep_dn7)))), (locals.var_phi_j0_dep_dn8 - ((locals.var_ndepmpnsub_inv1_dn8 * assign24640_body80_e20039) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn8 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn8)) + locals.var_vbscl__blk439_dn8) - locals.var_vbi_dep_dn8)))), (locals.var_phi_j0_dep_dn9 - ((locals.var_ndepmpnsub_inv1_dn9 * assign24640_body80_e20039) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn9 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn9)) + locals.var_vbscl__blk439_dn9) - locals.var_vbi_dep_dn9)))), (locals.var_phi_j0_dep_dn10 - ((locals.var_ndepmpnsub_inv1_dn10 * assign24640_body80_e20039) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn10 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn10)) + locals.var_vbscl__blk439_dn10) - locals.var_vbi_dep_dn10)))), (locals.var_phi_j0_dep_dn11 - ((locals.var_ndepmpnsub_inv1_dn11 * assign24640_body80_e20039) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn11 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn11)) + locals.var_vbscl__blk439_dn11) - locals.var_vbi_dep_dn11)))), (locals.var_phi_j0_dep_dn14 - ((locals.var_ndepmpnsub_inv1_dn14 * assign24640_body80_e20039) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn14 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn14)) + locals.var_vbscl__blk439_dn14) - locals.var_vbi_dep_dn14)))),)
    } else {
        (locals.var_y2, locals.var_y2_dn0, locals.var_y2_dn2, locals.var_y2_dn4, locals.var_y2_dn5, locals.var_y2_dn6, locals.var_y2_dn7, locals.var_y2_dn8, locals.var_y2_dn9, locals.var_y2_dn10, locals.var_y2_dn11, locals.var_y2_dn14,)
    }
};
            locals.var_y2 = assign24640_body80_e20043;
            locals.var_y2_dn0 = assign24640_body80_e20043_d_n0;
            locals.var_y2_dn2 = assign24640_body80_e20043_d_n2;
            locals.var_y2_dn4 = assign24640_body80_e20043_d_n4;
            locals.var_y2_dn5 = assign24640_body80_e20043_d_n5;
            locals.var_y2_dn6 = assign24640_body80_e20043_d_n6;
            locals.var_y2_dn7 = assign24640_body80_e20043_d_n7;
            locals.var_y2_dn8 = assign24640_body80_e20043_d_n8;
            locals.var_y2_dn9 = assign24640_body80_e20043_d_n9;
            locals.var_y2_dn10 = assign24640_body80_e20043_d_n10;
            locals.var_y2_dn11 = assign24640_body80_e20043_d_n11;
            locals.var_y2_dn14 = assign24640_body80_e20043_d_n14;
            locals.var_y2_rv = 0.0;
            let (assign24640_body81_e20051, assign24640_body81_e20051_d_n0, assign24640_body81_e20051_d_n2, assign24640_body81_e20051_d_n4, assign24640_body81_e20051_d_n5, assign24640_body81_e20051_d_n6, assign24640_body81_e20051_d_n7, assign24640_body81_e20051_d_n8, assign24640_body81_e20051_d_n9, assign24640_body81_e20051_d_n10, assign24640_body81_e20051_d_n11, assign24640_body81_e20051_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_y21, locals.var_y21_dn0, locals.var_y21_dn2, locals.var_y21_dn4, locals.var_y21_dn5, locals.var_y21_dn6, locals.var_y21_dn7, locals.var_y21_dn8, locals.var_y21_dn9, locals.var_y21_dn10, locals.var_y21_dn11, locals.var_y21_dn14,)
    }
};
            locals.var_y21 = assign24640_body81_e20051;
            locals.var_y21_dn0 = assign24640_body81_e20051_d_n0;
            locals.var_y21_dn2 = assign24640_body81_e20051_d_n2;
            locals.var_y21_dn4 = assign24640_body81_e20051_d_n4;
            locals.var_y21_dn5 = assign24640_body81_e20051_d_n5;
            locals.var_y21_dn6 = assign24640_body81_e20051_d_n6;
            locals.var_y21_dn7 = assign24640_body81_e20051_d_n7;
            locals.var_y21_dn8 = assign24640_body81_e20051_d_n8;
            locals.var_y21_dn9 = assign24640_body81_e20051_d_n9;
            locals.var_y21_dn10 = assign24640_body81_e20051_d_n10;
            locals.var_y21_dn11 = assign24640_body81_e20051_d_n11;
            locals.var_y21_dn14 = assign24640_body81_e20051_d_n14;
            locals.var_y21_rv = 0.0;
            let (assign24640_body82_e20059, assign24640_body82_e20059_d_n0, assign24640_body82_e20059_d_n2, assign24640_body82_e20059_d_n4, assign24640_body82_e20059_d_n5, assign24640_body82_e20059_d_n6, assign24640_body82_e20059_d_n7, assign24640_body82_e20059_d_n8, assign24640_body82_e20059_d_n9, assign24640_body82_e20059_d_n10, assign24640_body82_e20059_d_n11, assign24640_body82_e20059_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_y22, locals.var_y22_dn0, locals.var_y22_dn2, locals.var_y22_dn4, locals.var_y22_dn5, locals.var_y22_dn6, locals.var_y22_dn7, locals.var_y22_dn8, locals.var_y22_dn9, locals.var_y22_dn10, locals.var_y22_dn11, locals.var_y22_dn14,)
    }
};
            locals.var_y22 = assign24640_body82_e20059;
            locals.var_y22_dn0 = assign24640_body82_e20059_d_n0;
            locals.var_y22_dn2 = assign24640_body82_e20059_d_n2;
            locals.var_y22_dn4 = assign24640_body82_e20059_d_n4;
            locals.var_y22_dn5 = assign24640_body82_e20059_d_n5;
            locals.var_y22_dn6 = assign24640_body82_e20059_d_n6;
            locals.var_y22_dn7 = assign24640_body82_e20059_d_n7;
            locals.var_y22_dn8 = assign24640_body82_e20059_d_n8;
            locals.var_y22_dn9 = assign24640_body82_e20059_d_n9;
            locals.var_y22_dn10 = assign24640_body82_e20059_d_n10;
            locals.var_y22_dn11 = assign24640_body82_e20059_d_n11;
            locals.var_y22_dn14 = assign24640_body82_e20059_d_n14;
            locals.var_y22_rv = 0.0;
            let (assign24640_body83_e20073, assign24640_body83_e20073_d_n0, assign24640_body83_e20073_d_n2, assign24640_body83_e20073_d_n4, assign24640_body83_e20073_d_n5, assign24640_body83_e20073_d_n6, assign24640_body83_e20073_d_n7, assign24640_body83_e20073_d_n8, assign24640_body83_e20073_d_n9, assign24640_body83_e20073_d_n10, assign24640_body83_e20073_d_n11, assign24640_body83_e20073_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) {
        let assign24640_body83_e20067: f64 = (locals.var_y11 * locals.var_y22);
        let assign24640_body83_e20070: f64 = (locals.var_y21 * locals.var_y12);
        let assign24640_body83_e20071: f64 = (assign24640_body83_e20067 - assign24640_body83_e20070);
        (assign24640_body83_e20071, (((locals.var_y11_dn0 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn0)) - ((locals.var_y21_dn0 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn0))), (((locals.var_y11_dn2 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn2)) - ((locals.var_y21_dn2 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn2))), (((locals.var_y11_dn4 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn4)) - ((locals.var_y21_dn4 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn4))), (((locals.var_y11_dn5 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn5)) - ((locals.var_y21_dn5 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn5))), (((locals.var_y11_dn6 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn6)) - ((locals.var_y21_dn6 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn6))), (((locals.var_y11_dn7 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn7)) - ((locals.var_y21_dn7 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn7))), (((locals.var_y11_dn8 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn8)) - ((locals.var_y21_dn8 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn8))), (((locals.var_y11_dn9 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn9)) - ((locals.var_y21_dn9 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn9))), (((locals.var_y11_dn10 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn10)) - ((locals.var_y21_dn10 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn10))), (((locals.var_y11_dn11 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn11)) - ((locals.var_y21_dn11 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn11))), (((locals.var_y11_dn14 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn14)) - ((locals.var_y21_dn14 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn14))),)
    } else {
        (locals.var_dety, locals.var_dety_dn0, locals.var_dety_dn2, locals.var_dety_dn4, locals.var_dety_dn5, locals.var_dety_dn6, locals.var_dety_dn7, locals.var_dety_dn8, locals.var_dety_dn9, locals.var_dety_dn10, locals.var_dety_dn11, locals.var_dety_dn14,)
    }
};
            locals.var_dety = assign24640_body83_e20073;
            locals.var_dety_dn0 = assign24640_body83_e20073_d_n0;
            locals.var_dety_dn2 = assign24640_body83_e20073_d_n2;
            locals.var_dety_dn4 = assign24640_body83_e20073_d_n4;
            locals.var_dety_dn5 = assign24640_body83_e20073_d_n5;
            locals.var_dety_dn6 = assign24640_body83_e20073_d_n6;
            locals.var_dety_dn7 = assign24640_body83_e20073_d_n7;
            locals.var_dety_dn8 = assign24640_body83_e20073_d_n8;
            locals.var_dety_dn9 = assign24640_body83_e20073_d_n9;
            locals.var_dety_dn10 = assign24640_body83_e20073_d_n10;
            locals.var_dety_dn11 = assign24640_body83_e20073_d_n11;
            locals.var_dety_dn14 = assign24640_body83_e20073_d_n14;
            locals.var_dety_rv = 0.0;
            let (assign24640_body84_e20083, assign24640_body84_e20083_d_n0, assign24640_body84_e20083_d_n2, assign24640_body84_e20083_d_n4, assign24640_body84_e20083_d_n5, assign24640_body84_e20083_d_n6, assign24640_body84_e20083_d_n7, assign24640_body84_e20083_d_n8, assign24640_body84_e20083_d_n9, assign24640_body84_e20083_d_n10, assign24640_body84_e20083_d_n11, assign24640_body84_e20083_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) {
        let assign24640_body84_e20081: f64 = (locals.var_y22 / locals.var_dety);
        (assign24640_body84_e20081, (((locals.var_y22_dn0 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn0)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn2 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn2)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn4 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn4)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn5 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn5)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn6 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn6)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn7 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn7)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn8 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn8)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn9 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn9)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn10 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn10)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn11 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn11)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn14 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn14)) / (locals.var_dety * locals.var_dety)),)
    } else {
        (locals.var_rev11, locals.var_rev11_dn0, locals.var_rev11_dn2, locals.var_rev11_dn4, locals.var_rev11_dn5, locals.var_rev11_dn6, locals.var_rev11_dn7, locals.var_rev11_dn8, locals.var_rev11_dn9, locals.var_rev11_dn10, locals.var_rev11_dn11, locals.var_rev11_dn14,)
    }
};
            locals.var_rev11 = assign24640_body84_e20083;
            locals.var_rev11_dn0 = assign24640_body84_e20083_d_n0;
            locals.var_rev11_dn2 = assign24640_body84_e20083_d_n2;
            locals.var_rev11_dn4 = assign24640_body84_e20083_d_n4;
            locals.var_rev11_dn5 = assign24640_body84_e20083_d_n5;
            locals.var_rev11_dn6 = assign24640_body84_e20083_d_n6;
            locals.var_rev11_dn7 = assign24640_body84_e20083_d_n7;
            locals.var_rev11_dn8 = assign24640_body84_e20083_d_n8;
            locals.var_rev11_dn9 = assign24640_body84_e20083_d_n9;
            locals.var_rev11_dn10 = assign24640_body84_e20083_d_n10;
            locals.var_rev11_dn11 = assign24640_body84_e20083_d_n11;
            locals.var_rev11_dn14 = assign24640_body84_e20083_d_n14;
            locals.var_rev11_rv = 0.0;
            let (assign24640_body85_e20094, assign24640_body85_e20094_d_n0, assign24640_body85_e20094_d_n2, assign24640_body85_e20094_d_n4, assign24640_body85_e20094_d_n5, assign24640_body85_e20094_d_n6, assign24640_body85_e20094_d_n7, assign24640_body85_e20094_d_n8, assign24640_body85_e20094_d_n9, assign24640_body85_e20094_d_n10, assign24640_body85_e20094_d_n11, assign24640_body85_e20094_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) {
        let assign24640_body85_e20090: f64 = (-locals.var_y12);
        let assign24640_body85_e20092: f64 = (assign24640_body85_e20090 / locals.var_dety);
        (assign24640_body85_e20092, ((((-locals.var_y12_dn0) * locals.var_dety) - (assign24640_body85_e20090 * locals.var_dety_dn0)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn2) * locals.var_dety) - (assign24640_body85_e20090 * locals.var_dety_dn2)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn4) * locals.var_dety) - (assign24640_body85_e20090 * locals.var_dety_dn4)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn5) * locals.var_dety) - (assign24640_body85_e20090 * locals.var_dety_dn5)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn6) * locals.var_dety) - (assign24640_body85_e20090 * locals.var_dety_dn6)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn7) * locals.var_dety) - (assign24640_body85_e20090 * locals.var_dety_dn7)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn8) * locals.var_dety) - (assign24640_body85_e20090 * locals.var_dety_dn8)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn9) * locals.var_dety) - (assign24640_body85_e20090 * locals.var_dety_dn9)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn10) * locals.var_dety) - (assign24640_body85_e20090 * locals.var_dety_dn10)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn11) * locals.var_dety) - (assign24640_body85_e20090 * locals.var_dety_dn11)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn14) * locals.var_dety) - (assign24640_body85_e20090 * locals.var_dety_dn14)) / (locals.var_dety * locals.var_dety)),)
    } else {
        (locals.var_rev12, locals.var_rev12_dn0, locals.var_rev12_dn2, locals.var_rev12_dn4, locals.var_rev12_dn5, locals.var_rev12_dn6, locals.var_rev12_dn7, locals.var_rev12_dn8, locals.var_rev12_dn9, locals.var_rev12_dn10, locals.var_rev12_dn11, locals.var_rev12_dn14,)
    }
};
            locals.var_rev12 = assign24640_body85_e20094;
            locals.var_rev12_dn0 = assign24640_body85_e20094_d_n0;
            locals.var_rev12_dn2 = assign24640_body85_e20094_d_n2;
            locals.var_rev12_dn4 = assign24640_body85_e20094_d_n4;
            locals.var_rev12_dn5 = assign24640_body85_e20094_d_n5;
            locals.var_rev12_dn6 = assign24640_body85_e20094_d_n6;
            locals.var_rev12_dn7 = assign24640_body85_e20094_d_n7;
            locals.var_rev12_dn8 = assign24640_body85_e20094_d_n8;
            locals.var_rev12_dn9 = assign24640_body85_e20094_d_n9;
            locals.var_rev12_dn10 = assign24640_body85_e20094_d_n10;
            locals.var_rev12_dn11 = assign24640_body85_e20094_d_n11;
            locals.var_rev12_dn14 = assign24640_body85_e20094_d_n14;
            locals.var_rev12_rv = 0.0;
            let (assign24640_body86_e20105, assign24640_body86_e20105_d_n0, assign24640_body86_e20105_d_n2, assign24640_body86_e20105_d_n4, assign24640_body86_e20105_d_n5, assign24640_body86_e20105_d_n6, assign24640_body86_e20105_d_n7, assign24640_body86_e20105_d_n8, assign24640_body86_e20105_d_n9, assign24640_body86_e20105_d_n10, assign24640_body86_e20105_d_n11, assign24640_body86_e20105_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) {
        let assign24640_body86_e20101: f64 = (-locals.var_y21);
        let assign24640_body86_e20103: f64 = (assign24640_body86_e20101 / locals.var_dety);
        (assign24640_body86_e20103, ((((-locals.var_y21_dn0) * locals.var_dety) - (assign24640_body86_e20101 * locals.var_dety_dn0)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn2) * locals.var_dety) - (assign24640_body86_e20101 * locals.var_dety_dn2)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn4) * locals.var_dety) - (assign24640_body86_e20101 * locals.var_dety_dn4)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn5) * locals.var_dety) - (assign24640_body86_e20101 * locals.var_dety_dn5)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn6) * locals.var_dety) - (assign24640_body86_e20101 * locals.var_dety_dn6)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn7) * locals.var_dety) - (assign24640_body86_e20101 * locals.var_dety_dn7)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn8) * locals.var_dety) - (assign24640_body86_e20101 * locals.var_dety_dn8)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn9) * locals.var_dety) - (assign24640_body86_e20101 * locals.var_dety_dn9)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn10) * locals.var_dety) - (assign24640_body86_e20101 * locals.var_dety_dn10)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn11) * locals.var_dety) - (assign24640_body86_e20101 * locals.var_dety_dn11)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn14) * locals.var_dety) - (assign24640_body86_e20101 * locals.var_dety_dn14)) / (locals.var_dety * locals.var_dety)),)
    } else {
        (locals.var_rev21, locals.var_rev21_dn0, locals.var_rev21_dn2, locals.var_rev21_dn4, locals.var_rev21_dn5, locals.var_rev21_dn6, locals.var_rev21_dn7, locals.var_rev21_dn8, locals.var_rev21_dn9, locals.var_rev21_dn10, locals.var_rev21_dn11, locals.var_rev21_dn14,)
    }
};
            locals.var_rev21 = assign24640_body86_e20105;
            locals.var_rev21_dn0 = assign24640_body86_e20105_d_n0;
            locals.var_rev21_dn2 = assign24640_body86_e20105_d_n2;
            locals.var_rev21_dn4 = assign24640_body86_e20105_d_n4;
            locals.var_rev21_dn5 = assign24640_body86_e20105_d_n5;
            locals.var_rev21_dn6 = assign24640_body86_e20105_d_n6;
            locals.var_rev21_dn7 = assign24640_body86_e20105_d_n7;
            locals.var_rev21_dn8 = assign24640_body86_e20105_d_n8;
            locals.var_rev21_dn9 = assign24640_body86_e20105_d_n9;
            locals.var_rev21_dn10 = assign24640_body86_e20105_d_n10;
            locals.var_rev21_dn11 = assign24640_body86_e20105_d_n11;
            locals.var_rev21_dn14 = assign24640_body86_e20105_d_n14;
            locals.var_rev21_rv = 0.0;
            let (assign24640_body87_e20115, assign24640_body87_e20115_d_n0, assign24640_body87_e20115_d_n2, assign24640_body87_e20115_d_n4, assign24640_body87_e20115_d_n5, assign24640_body87_e20115_d_n6, assign24640_body87_e20115_d_n7, assign24640_body87_e20115_d_n8, assign24640_body87_e20115_d_n9, assign24640_body87_e20115_d_n10, assign24640_body87_e20115_d_n11, assign24640_body87_e20115_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) {
        let assign24640_body87_e20113: f64 = (locals.var_y11 / locals.var_dety);
        (assign24640_body87_e20113, (((locals.var_y11_dn0 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn0)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn2 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn2)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn4 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn4)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn5 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn5)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn6 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn6)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn7 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn7)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn8 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn8)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn9 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn9)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn10 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn10)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn11 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn11)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn14 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn14)) / (locals.var_dety * locals.var_dety)),)
    } else {
        (locals.var_rev22, locals.var_rev22_dn0, locals.var_rev22_dn2, locals.var_rev22_dn4, locals.var_rev22_dn5, locals.var_rev22_dn6, locals.var_rev22_dn7, locals.var_rev22_dn8, locals.var_rev22_dn9, locals.var_rev22_dn10, locals.var_rev22_dn11, locals.var_rev22_dn14,)
    }
};
            locals.var_rev22 = assign24640_body87_e20115;
            locals.var_rev22_dn0 = assign24640_body87_e20115_d_n0;
            locals.var_rev22_dn2 = assign24640_body87_e20115_d_n2;
            locals.var_rev22_dn4 = assign24640_body87_e20115_d_n4;
            locals.var_rev22_dn5 = assign24640_body87_e20115_d_n5;
            locals.var_rev22_dn6 = assign24640_body87_e20115_d_n6;
            locals.var_rev22_dn7 = assign24640_body87_e20115_d_n7;
            locals.var_rev22_dn8 = assign24640_body87_e20115_d_n8;
            locals.var_rev22_dn9 = assign24640_body87_e20115_d_n9;
            locals.var_rev22_dn10 = assign24640_body87_e20115_d_n10;
            locals.var_rev22_dn11 = assign24640_body87_e20115_d_n11;
            locals.var_rev22_dn14 = assign24640_body87_e20115_d_n14;
            locals.var_rev22_rv = 0.0;
            let assign24640_body88_e20118: f64 = (locals.var_rev11 * locals.var_y1);
            let assign24640_body88_e20121: f64 = (locals.var_rev12 * locals.var_y2);
            let assign24640_body88_e20122: f64 = (assign24640_body88_e20118 + assign24640_body88_e20121);
            let assign24640_body88_e20123: f64 = (assign24640_body88_e20122).abs();
            let assign24640_body88_e20125: f64 = if assign24640_body88_e20123 > 0.5 { 1.0 } else { 0.0 };
            locals.var_guard576 = assign24640_body88_e20125;
            locals.var_guard576_rv = 0.0;
            let (assign24640_body89_e20151, assign24640_body89_e20151_d_n0, assign24640_body89_e20151_d_n2, assign24640_body89_e20151_d_n4, assign24640_body89_e20151_d_n5, assign24640_body89_e20151_d_n6, assign24640_body89_e20151_d_n7, assign24640_body89_e20151_d_n8, assign24640_body89_e20151_d_n9, assign24640_body89_e20151_d_n10, assign24640_body89_e20151_d_n11, assign24640_body89_e20151_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard576 != 0.0)) {
        let assign24640_body89_e20137: f64 = (locals.var_rev11 * locals.var_y1);
        let assign24640_body89_e20140: f64 = (locals.var_rev12 * locals.var_y2);
        let assign24640_body89_e20141: f64 = (assign24640_body89_e20137 + assign24640_body89_e20140);
        let (assign24640_body89_e20147,) = {
            if (assign24640_body89_e20141 >= 0.0) {
                (1.0,)
            } else {
                let assign24640_body89_e20146: f64 = (-1.0);
                (assign24640_body89_e20146,)
            }
        };
        let assign24640_body89_e20148: f64 = (0.5 * assign24640_body89_e20147);
        let assign24640_body89_e20149: f64 = (locals.var_vgp0 - assign24640_body89_e20148);
        (assign24640_body89_e20149, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn11, locals.var_vgp0_dn14,)
    } else {
        (locals.var_vgp0, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn11, locals.var_vgp0_dn14,)
    }
};
            locals.var_vgp0 = assign24640_body89_e20151;
            locals.var_vgp0_dn0 = assign24640_body89_e20151_d_n0;
            locals.var_vgp0_dn2 = assign24640_body89_e20151_d_n2;
            locals.var_vgp0_dn4 = assign24640_body89_e20151_d_n4;
            locals.var_vgp0_dn5 = assign24640_body89_e20151_d_n5;
            locals.var_vgp0_dn6 = assign24640_body89_e20151_d_n6;
            locals.var_vgp0_dn7 = assign24640_body89_e20151_d_n7;
            locals.var_vgp0_dn8 = assign24640_body89_e20151_d_n8;
            locals.var_vgp0_dn9 = assign24640_body89_e20151_d_n9;
            locals.var_vgp0_dn10 = assign24640_body89_e20151_d_n10;
            locals.var_vgp0_dn11 = assign24640_body89_e20151_d_n11;
            locals.var_vgp0_dn14 = assign24640_body89_e20151_d_n14;
            locals.var_vgp0_rv = 0.0;
            let (assign24640_body90_e20177, assign24640_body90_e20177_d_n0, assign24640_body90_e20177_d_n2, assign24640_body90_e20177_d_n4, assign24640_body90_e20177_d_n5, assign24640_body90_e20177_d_n6, assign24640_body90_e20177_d_n7, assign24640_body90_e20177_d_n8, assign24640_body90_e20177_d_n9, assign24640_body90_e20177_d_n10, assign24640_body90_e20177_d_n11, assign24640_body90_e20177_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard576 != 0.0)) {
        let assign24640_body90_e20163: f64 = (locals.var_rev21 * locals.var_y1);
        let assign24640_body90_e20166: f64 = (locals.var_rev22 * locals.var_y2);
        let assign24640_body90_e20167: f64 = (assign24640_body90_e20163 + assign24640_body90_e20166);
        let (assign24640_body90_e20173,) = {
            if (assign24640_body90_e20167 >= 0.0) {
                (1.0,)
            } else {
                let assign24640_body90_e20172: f64 = (-1.0);
                (assign24640_body90_e20172,)
            }
        };
        let assign24640_body90_e20174: f64 = (0.5 * assign24640_body90_e20173);
        let assign24640_body90_e20175: f64 = (locals.var_phi_j0_dep - assign24640_body90_e20174);
        (assign24640_body90_e20175, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn11, locals.var_phi_j0_dep_dn14,)
    } else {
        (locals.var_phi_j0_dep, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn11, locals.var_phi_j0_dep_dn14,)
    }
};
            locals.var_phi_j0_dep = assign24640_body90_e20177;
            locals.var_phi_j0_dep_dn0 = assign24640_body90_e20177_d_n0;
            locals.var_phi_j0_dep_dn2 = assign24640_body90_e20177_d_n2;
            locals.var_phi_j0_dep_dn4 = assign24640_body90_e20177_d_n4;
            locals.var_phi_j0_dep_dn5 = assign24640_body90_e20177_d_n5;
            locals.var_phi_j0_dep_dn6 = assign24640_body90_e20177_d_n6;
            locals.var_phi_j0_dep_dn7 = assign24640_body90_e20177_d_n7;
            locals.var_phi_j0_dep_dn8 = assign24640_body90_e20177_d_n8;
            locals.var_phi_j0_dep_dn9 = assign24640_body90_e20177_d_n9;
            locals.var_phi_j0_dep_dn10 = assign24640_body90_e20177_d_n10;
            locals.var_phi_j0_dep_dn11 = assign24640_body90_e20177_d_n11;
            locals.var_phi_j0_dep_dn14 = assign24640_body90_e20177_d_n14;
            locals.var_phi_j0_dep_rv = 0.0;
            let (assign24640_body91_e20196, assign24640_body91_e20196_d_n0, assign24640_body91_e20196_d_n2, assign24640_body91_e20196_d_n4, assign24640_body91_e20196_d_n5, assign24640_body91_e20196_d_n6, assign24640_body91_e20196_d_n7, assign24640_body91_e20196_d_n8, assign24640_body91_e20196_d_n9, assign24640_body91_e20196_d_n10, assign24640_body91_e20196_d_n11, assign24640_body91_e20196_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard576 == 0.0)) {
        let assign24640_body91_e20189: f64 = (locals.var_rev11 * locals.var_y1);
        let assign24640_body91_e20192: f64 = (locals.var_rev12 * locals.var_y2);
        let assign24640_body91_e20193: f64 = (assign24640_body91_e20189 + assign24640_body91_e20192);
        let assign24640_body91_e20194: f64 = (locals.var_vgp0 - assign24640_body91_e20193);
        (assign24640_body91_e20194, (locals.var_vgp0_dn0 - (((locals.var_rev11_dn0 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn0)) + ((locals.var_rev12_dn0 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn0)))), (locals.var_vgp0_dn2 - (((locals.var_rev11_dn2 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn2)) + ((locals.var_rev12_dn2 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn2)))), (locals.var_vgp0_dn4 - (((locals.var_rev11_dn4 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn4)) + ((locals.var_rev12_dn4 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn4)))), (locals.var_vgp0_dn5 - (((locals.var_rev11_dn5 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn5)) + ((locals.var_rev12_dn5 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn5)))), (locals.var_vgp0_dn6 - (((locals.var_rev11_dn6 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn6)) + ((locals.var_rev12_dn6 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn6)))), (locals.var_vgp0_dn7 - (((locals.var_rev11_dn7 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn7)) + ((locals.var_rev12_dn7 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn7)))), (locals.var_vgp0_dn8 - (((locals.var_rev11_dn8 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn8)) + ((locals.var_rev12_dn8 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn8)))), (locals.var_vgp0_dn9 - (((locals.var_rev11_dn9 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn9)) + ((locals.var_rev12_dn9 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn9)))), (locals.var_vgp0_dn10 - (((locals.var_rev11_dn10 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn10)) + ((locals.var_rev12_dn10 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn10)))), (locals.var_vgp0_dn11 - (((locals.var_rev11_dn11 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn11)) + ((locals.var_rev12_dn11 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn11)))), (locals.var_vgp0_dn14 - (((locals.var_rev11_dn14 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn14)) + ((locals.var_rev12_dn14 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn14)))),)
    } else {
        (locals.var_vgp0, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn11, locals.var_vgp0_dn14,)
    }
};
            locals.var_vgp0 = assign24640_body91_e20196;
            locals.var_vgp0_dn0 = assign24640_body91_e20196_d_n0;
            locals.var_vgp0_dn2 = assign24640_body91_e20196_d_n2;
            locals.var_vgp0_dn4 = assign24640_body91_e20196_d_n4;
            locals.var_vgp0_dn5 = assign24640_body91_e20196_d_n5;
            locals.var_vgp0_dn6 = assign24640_body91_e20196_d_n6;
            locals.var_vgp0_dn7 = assign24640_body91_e20196_d_n7;
            locals.var_vgp0_dn8 = assign24640_body91_e20196_d_n8;
            locals.var_vgp0_dn9 = assign24640_body91_e20196_d_n9;
            locals.var_vgp0_dn10 = assign24640_body91_e20196_d_n10;
            locals.var_vgp0_dn11 = assign24640_body91_e20196_d_n11;
            locals.var_vgp0_dn14 = assign24640_body91_e20196_d_n14;
            locals.var_vgp0_rv = 0.0;
            let (assign24640_body92_e20215, assign24640_body92_e20215_d_n0, assign24640_body92_e20215_d_n2, assign24640_body92_e20215_d_n4, assign24640_body92_e20215_d_n5, assign24640_body92_e20215_d_n6, assign24640_body92_e20215_d_n7, assign24640_body92_e20215_d_n8, assign24640_body92_e20215_d_n9, assign24640_body92_e20215_d_n10, assign24640_body92_e20215_d_n11, assign24640_body92_e20215_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard576 == 0.0)) {
        let assign24640_body92_e20208: f64 = (locals.var_rev21 * locals.var_y1);
        let assign24640_body92_e20211: f64 = (locals.var_rev22 * locals.var_y2);
        let assign24640_body92_e20212: f64 = (assign24640_body92_e20208 + assign24640_body92_e20211);
        let assign24640_body92_e20213: f64 = (locals.var_phi_j0_dep - assign24640_body92_e20212);
        (assign24640_body92_e20213, (locals.var_phi_j0_dep_dn0 - (((locals.var_rev21_dn0 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn0)) + ((locals.var_rev22_dn0 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn0)))), (locals.var_phi_j0_dep_dn2 - (((locals.var_rev21_dn2 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn2)) + ((locals.var_rev22_dn2 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn2)))), (locals.var_phi_j0_dep_dn4 - (((locals.var_rev21_dn4 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn4)) + ((locals.var_rev22_dn4 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn4)))), (locals.var_phi_j0_dep_dn5 - (((locals.var_rev21_dn5 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn5)) + ((locals.var_rev22_dn5 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn5)))), (locals.var_phi_j0_dep_dn6 - (((locals.var_rev21_dn6 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn6)) + ((locals.var_rev22_dn6 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn6)))), (locals.var_phi_j0_dep_dn7 - (((locals.var_rev21_dn7 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn7)) + ((locals.var_rev22_dn7 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn7)))), (locals.var_phi_j0_dep_dn8 - (((locals.var_rev21_dn8 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn8)) + ((locals.var_rev22_dn8 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn8)))), (locals.var_phi_j0_dep_dn9 - (((locals.var_rev21_dn9 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn9)) + ((locals.var_rev22_dn9 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn9)))), (locals.var_phi_j0_dep_dn10 - (((locals.var_rev21_dn10 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn10)) + ((locals.var_rev22_dn10 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn10)))), (locals.var_phi_j0_dep_dn11 - (((locals.var_rev21_dn11 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn11)) + ((locals.var_rev22_dn11 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn11)))), (locals.var_phi_j0_dep_dn14 - (((locals.var_rev21_dn14 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn14)) + ((locals.var_rev22_dn14 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn14)))),)
    } else {
        (locals.var_phi_j0_dep, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn11, locals.var_phi_j0_dep_dn14,)
    }
};
            locals.var_phi_j0_dep = assign24640_body92_e20215;
            locals.var_phi_j0_dep_dn0 = assign24640_body92_e20215_d_n0;
            locals.var_phi_j0_dep_dn2 = assign24640_body92_e20215_d_n2;
            locals.var_phi_j0_dep_dn4 = assign24640_body92_e20215_d_n4;
            locals.var_phi_j0_dep_dn5 = assign24640_body92_e20215_d_n5;
            locals.var_phi_j0_dep_dn6 = assign24640_body92_e20215_d_n6;
            locals.var_phi_j0_dep_dn7 = assign24640_body92_e20215_d_n7;
            locals.var_phi_j0_dep_dn8 = assign24640_body92_e20215_d_n8;
            locals.var_phi_j0_dep_dn9 = assign24640_body92_e20215_d_n9;
            locals.var_phi_j0_dep_dn10 = assign24640_body92_e20215_d_n10;
            locals.var_phi_j0_dep_dn11 = assign24640_body92_e20215_d_n11;
            locals.var_phi_j0_dep_dn14 = assign24640_body92_e20215_d_n14;
            locals.var_phi_j0_dep_rv = 0.0;
            let assign24640_body93_e20218: f64 = (locals.var_vgp0 - locals.var_vgp0old);
            let assign24640_body93_e20219: f64 = (assign24640_body93_e20218).abs();
            let assign24640_body93_e20224: f64 = (locals.var_phi_j0_dep - locals.var_phi_j0_dep_old);
            let assign24640_body93_e20225: f64 = (assign24640_body93_e20224).abs();
            let assign24640_body93_e20228: f64 = if ((assign24640_body93_e20219 <= 1e-12) && (assign24640_body93_e20225 <= 1e-12)) { 1.0 } else { 0.0 };
            locals.var_guard577 = assign24640_body93_e20228;
            locals.var_guard577_rv = 0.0;
            let (assign24640_body94_e20240,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard577 != 0.0)) {
        let assign24640_body94_e20238: f64 = (150.0 + 1.0);
        (assign24640_body94_e20238,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign24640_body94_e20240;
            locals.var_lp_s0_rv = 0.0;
            let (assign24640_body95_e20248, assign24640_body95_e20248_d_n0, assign24640_body95_e20248_d_n2, assign24640_body95_e20248_d_n4, assign24640_body95_e20248_d_n5, assign24640_body95_e20248_d_n6, assign24640_body95_e20248_d_n7, assign24640_body95_e20248_d_n8, assign24640_body95_e20248_d_n9, assign24640_body95_e20248_d_n10, assign24640_body95_e20248_d_n11, assign24640_body95_e20248_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) {
        (locals.var_vgp0, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn11, locals.var_vgp0_dn14,)
    } else {
        (locals.var_vgp0old, locals.var_vgp0old_dn0, locals.var_vgp0old_dn2, locals.var_vgp0old_dn4, locals.var_vgp0old_dn5, locals.var_vgp0old_dn6, locals.var_vgp0old_dn7, locals.var_vgp0old_dn8, locals.var_vgp0old_dn9, locals.var_vgp0old_dn10, locals.var_vgp0old_dn11, locals.var_vgp0old_dn14,)
    }
};
            locals.var_vgp0old = assign24640_body95_e20248;
            locals.var_vgp0old_dn0 = assign24640_body95_e20248_d_n0;
            locals.var_vgp0old_dn2 = assign24640_body95_e20248_d_n2;
            locals.var_vgp0old_dn4 = assign24640_body95_e20248_d_n4;
            locals.var_vgp0old_dn5 = assign24640_body95_e20248_d_n5;
            locals.var_vgp0old_dn6 = assign24640_body95_e20248_d_n6;
            locals.var_vgp0old_dn7 = assign24640_body95_e20248_d_n7;
            locals.var_vgp0old_dn8 = assign24640_body95_e20248_d_n8;
            locals.var_vgp0old_dn9 = assign24640_body95_e20248_d_n9;
            locals.var_vgp0old_dn10 = assign24640_body95_e20248_d_n10;
            locals.var_vgp0old_dn11 = assign24640_body95_e20248_d_n11;
            locals.var_vgp0old_dn14 = assign24640_body95_e20248_d_n14;
            locals.var_vgp0old_rv = 0.0;
            let (assign24640_body96_e20256, assign24640_body96_e20256_d_n0, assign24640_body96_e20256_d_n2, assign24640_body96_e20256_d_n4, assign24640_body96_e20256_d_n5, assign24640_body96_e20256_d_n6, assign24640_body96_e20256_d_n7, assign24640_body96_e20256_d_n8, assign24640_body96_e20256_d_n9, assign24640_body96_e20256_d_n10, assign24640_body96_e20256_d_n11, assign24640_body96_e20256_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) {
        (locals.var_phi_j0_dep, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn11, locals.var_phi_j0_dep_dn14,)
    } else {
        (locals.var_phi_j0_dep_old, locals.var_phi_j0_dep_old_dn0, locals.var_phi_j0_dep_old_dn2, locals.var_phi_j0_dep_old_dn4, locals.var_phi_j0_dep_old_dn5, locals.var_phi_j0_dep_old_dn6, locals.var_phi_j0_dep_old_dn7, locals.var_phi_j0_dep_old_dn8, locals.var_phi_j0_dep_old_dn9, locals.var_phi_j0_dep_old_dn10, locals.var_phi_j0_dep_old_dn11, locals.var_phi_j0_dep_old_dn14,)
    }
};
            locals.var_phi_j0_dep_old = assign24640_body96_e20256;
            locals.var_phi_j0_dep_old_dn0 = assign24640_body96_e20256_d_n0;
            locals.var_phi_j0_dep_old_dn2 = assign24640_body96_e20256_d_n2;
            locals.var_phi_j0_dep_old_dn4 = assign24640_body96_e20256_d_n4;
            locals.var_phi_j0_dep_old_dn5 = assign24640_body96_e20256_d_n5;
            locals.var_phi_j0_dep_old_dn6 = assign24640_body96_e20256_d_n6;
            locals.var_phi_j0_dep_old_dn7 = assign24640_body96_e20256_d_n7;
            locals.var_phi_j0_dep_old_dn8 = assign24640_body96_e20256_d_n8;
            locals.var_phi_j0_dep_old_dn9 = assign24640_body96_e20256_d_n9;
            locals.var_phi_j0_dep_old_dn10 = assign24640_body96_e20256_d_n10;
            locals.var_phi_j0_dep_old_dn11 = assign24640_body96_e20256_d_n11;
            locals.var_phi_j0_dep_old_dn14 = assign24640_body96_e20256_d_n14;
            locals.var_phi_j0_dep_old_rv = 0.0;
            let (assign24640_body97_e20266,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) {
        let assign24640_body97_e20264: f64 = (locals.var_lp_s0 + 1.0);
        (assign24640_body97_e20264,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign24640_body97_e20266;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_70(
        locals: &mut StampLocals,
    ) {
        let (assign24650_e20274, assign24650_e20274_d_n0, assign24650_e20274_d_n2, assign24650_e20274_d_n4, assign24650_e20274_d_n5, assign24650_e20274_d_n6, assign24650_e20274_d_n7, assign24650_e20274_d_n8, assign24650_e20274_d_n9, assign24650_e20274_d_n10, assign24650_e20274_d_n11, assign24650_e20274_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) {
        (locals.var_phi_j0_dep, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn11, locals.var_phi_j0_dep_dn14,)
    } else {
        (locals.var_phi_j0_dep_acc, locals.var_phi_j0_dep_acc_dn0, locals.var_phi_j0_dep_acc_dn2, locals.var_phi_j0_dep_acc_dn4, locals.var_phi_j0_dep_acc_dn5, locals.var_phi_j0_dep_acc_dn6, locals.var_phi_j0_dep_acc_dn7, locals.var_phi_j0_dep_acc_dn8, locals.var_phi_j0_dep_acc_dn9, locals.var_phi_j0_dep_acc_dn10, locals.var_phi_j0_dep_acc_dn11, locals.var_phi_j0_dep_acc_dn14,)
    }
};
        locals.var_phi_j0_dep_acc = assign24650_e20274;
        locals.var_phi_j0_dep_acc_dn0 = assign24650_e20274_d_n0;
        locals.var_phi_j0_dep_acc_dn2 = assign24650_e20274_d_n2;
        locals.var_phi_j0_dep_acc_dn4 = assign24650_e20274_d_n4;
        locals.var_phi_j0_dep_acc_dn5 = assign24650_e20274_d_n5;
        locals.var_phi_j0_dep_acc_dn6 = assign24650_e20274_d_n6;
        locals.var_phi_j0_dep_acc_dn7 = assign24650_e20274_d_n7;
        locals.var_phi_j0_dep_acc_dn8 = assign24650_e20274_d_n8;
        locals.var_phi_j0_dep_acc_dn9 = assign24650_e20274_d_n9;
        locals.var_phi_j0_dep_acc_dn10 = assign24650_e20274_d_n10;
        locals.var_phi_j0_dep_acc_dn11 = assign24650_e20274_d_n11;
        locals.var_phi_j0_dep_acc_dn14 = assign24650_e20274_d_n14;
        locals.var_phi_j0_dep_acc_rv = 0.0;

        let (assign24660_e20284, assign24660_e20284_d_n0, assign24660_e20284_d_n2, assign24660_e20284_d_n4, assign24660_e20284_d_n5, assign24660_e20284_d_n6, assign24660_e20284_d_n7, assign24660_e20284_d_n8, assign24660_e20284_d_n9, assign24660_e20284_d_n10, assign24660_e20284_d_n11, assign24660_e20284_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) {
        let assign24660_e20282: f64 = (locals.var_uc_depthn * locals.var_ndepmpnsub);
        (assign24660_e20282, ((locals.var_uc_depthn_dn0 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn0)), ((locals.var_uc_depthn_dn2 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn2)), ((locals.var_uc_depthn_dn4 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn4)), ((locals.var_uc_depthn_dn5 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn5)), ((locals.var_uc_depthn_dn6 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn6)), ((locals.var_uc_depthn_dn7 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn7)), ((locals.var_uc_depthn_dn8 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn8)), ((locals.var_uc_depthn_dn9 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn9)), ((locals.var_uc_depthn_dn10 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn10)), ((locals.var_uc_depthn_dn11 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn11)), ((locals.var_uc_depthn_dn14 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn14)),)
    } else {
        (locals.var_w_sub0, locals.var_w_sub0_dn0, locals.var_w_sub0_dn2, locals.var_w_sub0_dn4, locals.var_w_sub0_dn5, locals.var_w_sub0_dn6, locals.var_w_sub0_dn7, locals.var_w_sub0_dn8, locals.var_w_sub0_dn9, locals.var_w_sub0_dn10, locals.var_w_sub0_dn11, locals.var_w_sub0_dn14,)
    }
};
        locals.var_w_sub0 = assign24660_e20284;
        locals.var_w_sub0_dn0 = assign24660_e20284_d_n0;
        locals.var_w_sub0_dn2 = assign24660_e20284_d_n2;
        locals.var_w_sub0_dn4 = assign24660_e20284_d_n4;
        locals.var_w_sub0_dn5 = assign24660_e20284_d_n5;
        locals.var_w_sub0_dn6 = assign24660_e20284_d_n6;
        locals.var_w_sub0_dn7 = assign24660_e20284_d_n7;
        locals.var_w_sub0_dn8 = assign24660_e20284_d_n8;
        locals.var_w_sub0_dn9 = assign24660_e20284_d_n9;
        locals.var_w_sub0_dn10 = assign24660_e20284_d_n10;
        locals.var_w_sub0_dn11 = assign24660_e20284_d_n11;
        locals.var_w_sub0_dn14 = assign24660_e20284_d_n14;
        locals.var_w_sub0_rv = 0.0;

        let (assign24670_e20300, assign24670_e20300_d_n0, assign24670_e20300_d_n2, assign24670_e20300_d_n4, assign24670_e20300_d_n5, assign24670_e20300_d_n6, assign24670_e20300_d_n7, assign24670_e20300_d_n8, assign24670_e20300_d_n9, assign24670_e20300_d_n10, assign24670_e20300_d_n11, assign24670_e20300_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) {
        let assign24670_e20292: f64 = (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0);
        let assign24670_e20294: f64 = (assign24670_e20292 * locals.var_w_sub0);
        let assign24670_e20296: f64 = (assign24670_e20294 + locals.var_vbscl__blk439);
        let assign24670_e20298: f64 = (assign24670_e20296 - locals.var_vbi_dep);
        (assign24670_e20298, ((((((locals.var_c_2esipq_nsub_inv_dn0 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn0)) * locals.var_w_sub0) + (assign24670_e20292 * locals.var_w_sub0_dn0)) + locals.var_vbscl__blk439_dn0) - locals.var_vbi_dep_dn0), ((((((locals.var_c_2esipq_nsub_inv_dn2 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn2)) * locals.var_w_sub0) + (assign24670_e20292 * locals.var_w_sub0_dn2)) + locals.var_vbscl__blk439_dn2) - locals.var_vbi_dep_dn2), ((((((locals.var_c_2esipq_nsub_inv_dn4 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn4)) * locals.var_w_sub0) + (assign24670_e20292 * locals.var_w_sub0_dn4)) + locals.var_vbscl__blk439_dn4) - locals.var_vbi_dep_dn4), ((((((locals.var_c_2esipq_nsub_inv_dn5 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn5)) * locals.var_w_sub0) + (assign24670_e20292 * locals.var_w_sub0_dn5)) + locals.var_vbscl__blk439_dn5) - locals.var_vbi_dep_dn5), ((((((locals.var_c_2esipq_nsub_inv_dn6 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn6)) * locals.var_w_sub0) + (assign24670_e20292 * locals.var_w_sub0_dn6)) + locals.var_vbscl__blk439_dn6) - locals.var_vbi_dep_dn6), ((((((locals.var_c_2esipq_nsub_inv_dn7 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn7)) * locals.var_w_sub0) + (assign24670_e20292 * locals.var_w_sub0_dn7)) + locals.var_vbscl__blk439_dn7) - locals.var_vbi_dep_dn7), ((((((locals.var_c_2esipq_nsub_inv_dn8 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn8)) * locals.var_w_sub0) + (assign24670_e20292 * locals.var_w_sub0_dn8)) + locals.var_vbscl__blk439_dn8) - locals.var_vbi_dep_dn8), ((((((locals.var_c_2esipq_nsub_inv_dn9 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn9)) * locals.var_w_sub0) + (assign24670_e20292 * locals.var_w_sub0_dn9)) + locals.var_vbscl__blk439_dn9) - locals.var_vbi_dep_dn9), ((((((locals.var_c_2esipq_nsub_inv_dn10 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn10)) * locals.var_w_sub0) + (assign24670_e20292 * locals.var_w_sub0_dn10)) + locals.var_vbscl__blk439_dn10) - locals.var_vbi_dep_dn10), ((((((locals.var_c_2esipq_nsub_inv_dn11 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn11)) * locals.var_w_sub0) + (assign24670_e20292 * locals.var_w_sub0_dn11)) + locals.var_vbscl__blk439_dn11) - locals.var_vbi_dep_dn11), ((((((locals.var_c_2esipq_nsub_inv_dn14 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn14)) * locals.var_w_sub0) + (assign24670_e20292 * locals.var_w_sub0_dn14)) + locals.var_vbscl__blk439_dn14) - locals.var_vbi_dep_dn14),)
    } else {
        (locals.var_phi_j0_dep, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn11, locals.var_phi_j0_dep_dn14,)
    }
};
        locals.var_phi_j0_dep = assign24670_e20300;
        locals.var_phi_j0_dep_dn0 = assign24670_e20300_d_n0;
        locals.var_phi_j0_dep_dn2 = assign24670_e20300_d_n2;
        locals.var_phi_j0_dep_dn4 = assign24670_e20300_d_n4;
        locals.var_phi_j0_dep_dn5 = assign24670_e20300_d_n5;
        locals.var_phi_j0_dep_dn6 = assign24670_e20300_d_n6;
        locals.var_phi_j0_dep_dn7 = assign24670_e20300_d_n7;
        locals.var_phi_j0_dep_dn8 = assign24670_e20300_d_n8;
        locals.var_phi_j0_dep_dn9 = assign24670_e20300_d_n9;
        locals.var_phi_j0_dep_dn10 = assign24670_e20300_d_n10;
        locals.var_phi_j0_dep_dn11 = assign24670_e20300_d_n11;
        locals.var_phi_j0_dep_dn14 = assign24670_e20300_d_n14;
        locals.var_phi_j0_dep_rv = 0.0;

        let (assign24680_e20312, assign24680_e20312_d_n0, assign24680_e20312_d_n2, assign24680_e20312_d_n4, assign24680_e20312_d_n5, assign24680_e20312_d_n6, assign24680_e20312_d_n7, assign24680_e20312_d_n8, assign24680_e20312_d_n9, assign24680_e20312_d_n10, assign24680_e20312_d_n11, assign24680_e20312_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) {
        let assign24680_e20309: f64 = (locals.var_c_2esipq_ndepm_inv * locals.var_tn2);
        let assign24680_e20310: f64 = (locals.var_phi_j0_dep + assign24680_e20309);
        (assign24680_e20310, (locals.var_phi_j0_dep_dn0 + ((locals.var_c_2esipq_ndepm_inv_dn0 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn0))), (locals.var_phi_j0_dep_dn2 + ((locals.var_c_2esipq_ndepm_inv_dn2 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn2))), (locals.var_phi_j0_dep_dn4 + ((locals.var_c_2esipq_ndepm_inv_dn4 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn4))), (locals.var_phi_j0_dep_dn5 + ((locals.var_c_2esipq_ndepm_inv_dn5 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn5))), (locals.var_phi_j0_dep_dn6 + ((locals.var_c_2esipq_ndepm_inv_dn6 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn6))), (locals.var_phi_j0_dep_dn7 + ((locals.var_c_2esipq_ndepm_inv_dn7 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn7))), (locals.var_phi_j0_dep_dn8 + ((locals.var_c_2esipq_ndepm_inv_dn8 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn8))), (locals.var_phi_j0_dep_dn9 + ((locals.var_c_2esipq_ndepm_inv_dn9 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn9))), (locals.var_phi_j0_dep_dn10 + ((locals.var_c_2esipq_ndepm_inv_dn10 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn10))), (locals.var_phi_j0_dep_dn11 + ((locals.var_c_2esipq_ndepm_inv_dn11 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn11))), (locals.var_phi_j0_dep_dn14 + ((locals.var_c_2esipq_ndepm_inv_dn14 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn14))),)
    } else {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn11, locals.var_phi_b0_dep_dn14,)
    }
};
        locals.var_phi_b0_dep = assign24680_e20312;
        locals.var_phi_b0_dep_dn0 = assign24680_e20312_d_n0;
        locals.var_phi_b0_dep_dn2 = assign24680_e20312_d_n2;
        locals.var_phi_b0_dep_dn4 = assign24680_e20312_d_n4;
        locals.var_phi_b0_dep_dn5 = assign24680_e20312_d_n5;
        locals.var_phi_b0_dep_dn6 = assign24680_e20312_d_n6;
        locals.var_phi_b0_dep_dn7 = assign24680_e20312_d_n7;
        locals.var_phi_b0_dep_dn8 = assign24680_e20312_d_n8;
        locals.var_phi_b0_dep_dn9 = assign24680_e20312_d_n9;
        locals.var_phi_b0_dep_dn10 = assign24680_e20312_d_n10;
        locals.var_phi_b0_dep_dn11 = assign24680_e20312_d_n11;
        locals.var_phi_b0_dep_dn14 = assign24680_e20312_d_n14;
        locals.var_phi_b0_dep_rv = 0.0;

        let (assign24690_e20320, assign24690_e20320_d_n0, assign24690_e20320_d_n2, assign24690_e20320_d_n4, assign24690_e20320_d_n5, assign24690_e20320_d_n6, assign24690_e20320_d_n7, assign24690_e20320_d_n8, assign24690_e20320_d_n9, assign24690_e20320_d_n10, assign24690_e20320_d_n11, assign24690_e20320_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn11, locals.var_phi_b0_dep_dn14,)
    } else {
        (locals.var_phi_s0_dep, locals.var_phi_s0_dep_dn0, locals.var_phi_s0_dep_dn2, locals.var_phi_s0_dep_dn4, locals.var_phi_s0_dep_dn5, locals.var_phi_s0_dep_dn6, locals.var_phi_s0_dep_dn7, locals.var_phi_s0_dep_dn8, locals.var_phi_s0_dep_dn9, locals.var_phi_s0_dep_dn10, locals.var_phi_s0_dep_dn11, locals.var_phi_s0_dep_dn14,)
    }
};
        locals.var_phi_s0_dep = assign24690_e20320;
        locals.var_phi_s0_dep_dn0 = assign24690_e20320_d_n0;
        locals.var_phi_s0_dep_dn2 = assign24690_e20320_d_n2;
        locals.var_phi_s0_dep_dn4 = assign24690_e20320_d_n4;
        locals.var_phi_s0_dep_dn5 = assign24690_e20320_d_n5;
        locals.var_phi_s0_dep_dn6 = assign24690_e20320_d_n6;
        locals.var_phi_s0_dep_dn7 = assign24690_e20320_d_n7;
        locals.var_phi_s0_dep_dn8 = assign24690_e20320_d_n8;
        locals.var_phi_s0_dep_dn9 = assign24690_e20320_d_n9;
        locals.var_phi_s0_dep_dn10 = assign24690_e20320_d_n10;
        locals.var_phi_s0_dep_dn11 = assign24690_e20320_d_n11;
        locals.var_phi_s0_dep_dn14 = assign24690_e20320_d_n14;
        locals.var_phi_s0_dep_rv = 0.0;

        let (assign24700_e20328, assign24700_e20328_d_n0, assign24700_e20328_d_n2, assign24700_e20328_d_n4, assign24700_e20328_d_n5, assign24700_e20328_d_n6, assign24700_e20328_d_n7, assign24700_e20328_d_n8, assign24700_e20328_d_n9, assign24700_e20328_d_n10, assign24700_e20328_d_n11, assign24700_e20328_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn11, locals.var_phi_b0_dep_dn14,)
    } else {
        (locals.var_psbmax, locals.var_psbmax_dn0, locals.var_psbmax_dn2, locals.var_psbmax_dn4, locals.var_psbmax_dn5, locals.var_psbmax_dn6, locals.var_psbmax_dn7, locals.var_psbmax_dn8, locals.var_psbmax_dn9, locals.var_psbmax_dn10, locals.var_psbmax_dn11, locals.var_psbmax_dn14,)
    }
};
        locals.var_psbmax = assign24700_e20328;
        locals.var_psbmax_dn0 = assign24700_e20328_d_n0;
        locals.var_psbmax_dn2 = assign24700_e20328_d_n2;
        locals.var_psbmax_dn4 = assign24700_e20328_d_n4;
        locals.var_psbmax_dn5 = assign24700_e20328_d_n5;
        locals.var_psbmax_dn6 = assign24700_e20328_d_n6;
        locals.var_psbmax_dn7 = assign24700_e20328_d_n7;
        locals.var_psbmax_dn8 = assign24700_e20328_d_n8;
        locals.var_psbmax_dn9 = assign24700_e20328_d_n9;
        locals.var_psbmax_dn10 = assign24700_e20328_d_n10;
        locals.var_psbmax_dn11 = assign24700_e20328_d_n11;
        locals.var_psbmax_dn14 = assign24700_e20328_d_n14;
        locals.var_psbmax_rv = 0.0;

        let (assign24710_e20336, assign24710_e20336_d_n0, assign24710_e20336_d_n2, assign24710_e20336_d_n4, assign24710_e20336_d_n5, assign24710_e20336_d_n6, assign24710_e20336_d_n7, assign24710_e20336_d_n8, assign24710_e20336_d_n9, assign24710_e20336_d_n10, assign24710_e20336_d_n11, assign24710_e20336_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn11, locals.var_phi_b0_dep_dn14,)
    } else {
        (locals.var_vgp1, locals.var_vgp1_dn0, locals.var_vgp1_dn2, locals.var_vgp1_dn4, locals.var_vgp1_dn5, locals.var_vgp1_dn6, locals.var_vgp1_dn7, locals.var_vgp1_dn8, locals.var_vgp1_dn9, locals.var_vgp1_dn10, locals.var_vgp1_dn11, locals.var_vgp1_dn14,)
    }
};
        locals.var_vgp1 = assign24710_e20336;
        locals.var_vgp1_dn0 = assign24710_e20336_d_n0;
        locals.var_vgp1_dn2 = assign24710_e20336_d_n2;
        locals.var_vgp1_dn4 = assign24710_e20336_d_n4;
        locals.var_vgp1_dn5 = assign24710_e20336_d_n5;
        locals.var_vgp1_dn6 = assign24710_e20336_d_n6;
        locals.var_vgp1_dn7 = assign24710_e20336_d_n7;
        locals.var_vgp1_dn8 = assign24710_e20336_d_n8;
        locals.var_vgp1_dn9 = assign24710_e20336_d_n9;
        locals.var_vgp1_dn10 = assign24710_e20336_d_n10;
        locals.var_vgp1_dn11 = assign24710_e20336_d_n11;
        locals.var_vgp1_dn14 = assign24710_e20336_d_n14;
        locals.var_vgp1_rv = 0.0;

        let assign24720_e20339: f64 = if locals.var_vgp > locals.var_vgp0 { 1.0 } else { 0.0 };
        locals.var_guard578 = assign24720_e20339;
        locals.var_guard578_rv = 0.0;

        let (assign24730_e20349,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard578 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_depmode,)
    }
};
        locals.var_depmode = assign24730_e20349;
        locals.var_depmode_rv = 0.0;

        let assign24740_e20352: f64 = if locals.var_vgp > locals.var_vgp1 { 1.0 } else { 0.0 };
        locals.var_guard579 = assign24740_e20352;
        locals.var_guard579_rv = 0.0;

        let (assign24750_e20365,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard579 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_depmode,)
    }
};
        locals.var_depmode = assign24750_e20365;
        locals.var_depmode_rv = 0.0;

        let (assign24760_e20379,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard579 == 0.0)) {
        (2.0,)
    } else {
        (locals.var_depmode,)
    }
};
        locals.var_depmode = assign24760_e20379;
        locals.var_depmode_rv = 0.0;

        let (assign24770_e20388, assign24770_e20388_d_n0, assign24770_e20388_d_n2, assign24770_e20388_d_n4, assign24770_e20388_d_n5, assign24770_e20388_d_n6, assign24770_e20388_d_n7, assign24770_e20388_d_n8, assign24770_e20388_d_n9, assign24770_e20388_d_n10, assign24770_e20388_d_n11, assign24770_e20388_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgp0, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn11, locals.var_vgp0_dn14,)
    }
};
        locals.var_vgp0 = assign24770_e20388;
        locals.var_vgp0_dn0 = assign24770_e20388_d_n0;
        locals.var_vgp0_dn2 = assign24770_e20388_d_n2;
        locals.var_vgp0_dn4 = assign24770_e20388_d_n4;
        locals.var_vgp0_dn5 = assign24770_e20388_d_n5;
        locals.var_vgp0_dn6 = assign24770_e20388_d_n6;
        locals.var_vgp0_dn7 = assign24770_e20388_d_n7;
        locals.var_vgp0_dn8 = assign24770_e20388_d_n8;
        locals.var_vgp0_dn9 = assign24770_e20388_d_n9;
        locals.var_vgp0_dn10 = assign24770_e20388_d_n10;
        locals.var_vgp0_dn11 = assign24770_e20388_d_n11;
        locals.var_vgp0_dn14 = assign24770_e20388_d_n14;
        locals.var_vgp0_rv = 0.0;

        let (assign24780_e20397, assign24780_e20397_d_n0, assign24780_e20397_d_n2, assign24780_e20397_d_n4, assign24780_e20397_d_n5, assign24780_e20397_d_n6, assign24780_e20397_d_n7, assign24780_e20397_d_n8, assign24780_e20397_d_n9, assign24780_e20397_d_n10, assign24780_e20397_d_n11, assign24780_e20397_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 == 0.0)) {
        (locals.var_vgp0, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn11, locals.var_vgp0_dn14,)
    } else {
        (locals.var_vgp1, locals.var_vgp1_dn0, locals.var_vgp1_dn2, locals.var_vgp1_dn4, locals.var_vgp1_dn5, locals.var_vgp1_dn6, locals.var_vgp1_dn7, locals.var_vgp1_dn8, locals.var_vgp1_dn9, locals.var_vgp1_dn10, locals.var_vgp1_dn11, locals.var_vgp1_dn14,)
    }
};
        locals.var_vgp1 = assign24780_e20397;
        locals.var_vgp1_dn0 = assign24780_e20397_d_n0;
        locals.var_vgp1_dn2 = assign24780_e20397_d_n2;
        locals.var_vgp1_dn4 = assign24780_e20397_d_n4;
        locals.var_vgp1_dn5 = assign24780_e20397_d_n5;
        locals.var_vgp1_dn6 = assign24780_e20397_d_n6;
        locals.var_vgp1_dn7 = assign24780_e20397_d_n7;
        locals.var_vgp1_dn8 = assign24780_e20397_d_n8;
        locals.var_vgp1_dn9 = assign24780_e20397_d_n9;
        locals.var_vgp1_dn10 = assign24780_e20397_d_n10;
        locals.var_vgp1_dn11 = assign24780_e20397_d_n11;
        locals.var_vgp1_dn14 = assign24780_e20397_d_n14;
        locals.var_vgp1_rv = 0.0;

        let (assign24790_e20406, assign24790_e20406_d_n0, assign24790_e20406_d_n2, assign24790_e20406_d_n4, assign24790_e20406_d_n5, assign24790_e20406_d_n6, assign24790_e20406_d_n7, assign24790_e20406_d_n8, assign24790_e20406_d_n9, assign24790_e20406_d_n10, assign24790_e20406_d_n11, assign24790_e20406_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_psbmax, locals.var_psbmax_dn0, locals.var_psbmax_dn2, locals.var_psbmax_dn4, locals.var_psbmax_dn5, locals.var_psbmax_dn6, locals.var_psbmax_dn7, locals.var_psbmax_dn8, locals.var_psbmax_dn9, locals.var_psbmax_dn10, locals.var_psbmax_dn11, locals.var_psbmax_dn14,)
    }
};
        locals.var_psbmax = assign24790_e20406;
        locals.var_psbmax_dn0 = assign24790_e20406_d_n0;
        locals.var_psbmax_dn2 = assign24790_e20406_d_n2;
        locals.var_psbmax_dn4 = assign24790_e20406_d_n4;
        locals.var_psbmax_dn5 = assign24790_e20406_d_n5;
        locals.var_psbmax_dn6 = assign24790_e20406_d_n6;
        locals.var_psbmax_dn7 = assign24790_e20406_d_n7;
        locals.var_psbmax_dn8 = assign24790_e20406_d_n8;
        locals.var_psbmax_dn9 = assign24790_e20406_d_n9;
        locals.var_psbmax_dn10 = assign24790_e20406_d_n10;
        locals.var_psbmax_dn11 = assign24790_e20406_d_n11;
        locals.var_psbmax_dn14 = assign24790_e20406_d_n14;
        locals.var_psbmax_rv = 0.0;

        let (assign24800_e20415, assign24800_e20415_d_n0, assign24800_e20415_d_n2, assign24800_e20415_d_n4, assign24800_e20415_d_n5, assign24800_e20415_d_n6, assign24800_e20415_d_n7, assign24800_e20415_d_n8, assign24800_e20415_d_n9, assign24800_e20415_d_n10, assign24800_e20415_d_n11, assign24800_e20415_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 == 0.0)) {
        (locals.var_vgp0, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn11, locals.var_vgp0_dn14,)
    } else {
        (locals.var_vds_maxb0, locals.var_vds_maxb0_dn0, locals.var_vds_maxb0_dn2, locals.var_vds_maxb0_dn4, locals.var_vds_maxb0_dn5, locals.var_vds_maxb0_dn6, locals.var_vds_maxb0_dn7, locals.var_vds_maxb0_dn8, locals.var_vds_maxb0_dn9, locals.var_vds_maxb0_dn10, locals.var_vds_maxb0_dn11, locals.var_vds_maxb0_dn14,)
    }
};
        locals.var_vds_maxb0 = assign24800_e20415;
        locals.var_vds_maxb0_dn0 = assign24800_e20415_d_n0;
        locals.var_vds_maxb0_dn2 = assign24800_e20415_d_n2;
        locals.var_vds_maxb0_dn4 = assign24800_e20415_d_n4;
        locals.var_vds_maxb0_dn5 = assign24800_e20415_d_n5;
        locals.var_vds_maxb0_dn6 = assign24800_e20415_d_n6;
        locals.var_vds_maxb0_dn7 = assign24800_e20415_d_n7;
        locals.var_vds_maxb0_dn8 = assign24800_e20415_d_n8;
        locals.var_vds_maxb0_dn9 = assign24800_e20415_d_n9;
        locals.var_vds_maxb0_dn10 = assign24800_e20415_d_n10;
        locals.var_vds_maxb0_dn11 = assign24800_e20415_d_n11;
        locals.var_vds_maxb0_dn14 = assign24800_e20415_d_n14;
        locals.var_vds_maxb0_rv = 0.0;

        let (assign24810_e20424, assign24810_e20424_d_n0, assign24810_e20424_d_n2, assign24810_e20424_d_n4, assign24810_e20424_d_n5, assign24810_e20424_d_n6, assign24810_e20424_d_n7, assign24810_e20424_d_n8, assign24810_e20424_d_n9, assign24810_e20424_d_n10, assign24810_e20424_d_n11, assign24810_e20424_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 == 0.0)) {
        (locals.var_w_bsub0, locals.var_w_bsub0_dn0, locals.var_w_bsub0_dn2, locals.var_w_bsub0_dn4, locals.var_w_bsub0_dn5, locals.var_w_bsub0_dn6, locals.var_w_bsub0_dn7, locals.var_w_bsub0_dn8, locals.var_w_bsub0_dn9, locals.var_w_bsub0_dn10, locals.var_w_bsub0_dn11, locals.var_w_bsub0_dn14,)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn11, locals.var_w_b0_dn14,)
    }
};
        locals.var_w_b0 = assign24810_e20424;
        locals.var_w_b0_dn0 = assign24810_e20424_d_n0;
        locals.var_w_b0_dn2 = assign24810_e20424_d_n2;
        locals.var_w_b0_dn4 = assign24810_e20424_d_n4;
        locals.var_w_b0_dn5 = assign24810_e20424_d_n5;
        locals.var_w_b0_dn6 = assign24810_e20424_d_n6;
        locals.var_w_b0_dn7 = assign24810_e20424_d_n7;
        locals.var_w_b0_dn8 = assign24810_e20424_d_n8;
        locals.var_w_b0_dn9 = assign24810_e20424_d_n9;
        locals.var_w_b0_dn10 = assign24810_e20424_d_n10;
        locals.var_w_b0_dn11 = assign24810_e20424_d_n11;
        locals.var_w_b0_dn14 = assign24810_e20424_d_n14;
        locals.var_w_b0_rv = 0.0;

        let (assign24820_e20435, assign24820_e20435_d_n0, assign24820_e20435_d_n2, assign24820_e20435_d_n4, assign24820_e20435_d_n5, assign24820_e20435_d_n6, assign24820_e20435_d_n7, assign24820_e20435_d_n8, assign24820_e20435_d_n9, assign24820_e20435_d_n10, assign24820_e20435_d_n11, assign24820_e20435_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 == 0.0)) {
        let assign24820_e20433: f64 = (locals.var_w_b0 * locals.var_ndepmpnsub);
        (assign24820_e20433, ((locals.var_w_b0_dn0 * locals.var_ndepmpnsub) + (locals.var_w_b0 * locals.var_ndepmpnsub_dn0)), ((locals.var_w_b0_dn2 * locals.var_ndepmpnsub) + (locals.var_w_b0 * locals.var_ndepmpnsub_dn2)), ((locals.var_w_b0_dn4 * locals.var_ndepmpnsub) + (locals.var_w_b0 * locals.var_ndepmpnsub_dn4)), ((locals.var_w_b0_dn5 * locals.var_ndepmpnsub) + (locals.var_w_b0 * locals.var_ndepmpnsub_dn5)), ((locals.var_w_b0_dn6 * locals.var_ndepmpnsub) + (locals.var_w_b0 * locals.var_ndepmpnsub_dn6)), ((locals.var_w_b0_dn7 * locals.var_ndepmpnsub) + (locals.var_w_b0 * locals.var_ndepmpnsub_dn7)), ((locals.var_w_b0_dn8 * locals.var_ndepmpnsub) + (locals.var_w_b0 * locals.var_ndepmpnsub_dn8)), ((locals.var_w_b0_dn9 * locals.var_ndepmpnsub) + (locals.var_w_b0 * locals.var_ndepmpnsub_dn9)), ((locals.var_w_b0_dn10 * locals.var_ndepmpnsub) + (locals.var_w_b0 * locals.var_ndepmpnsub_dn10)), ((locals.var_w_b0_dn11 * locals.var_ndepmpnsub) + (locals.var_w_b0 * locals.var_ndepmpnsub_dn11)), ((locals.var_w_b0_dn14 * locals.var_ndepmpnsub) + (locals.var_w_b0 * locals.var_ndepmpnsub_dn14)),)
    } else {
        (locals.var_w_sub0, locals.var_w_sub0_dn0, locals.var_w_sub0_dn2, locals.var_w_sub0_dn4, locals.var_w_sub0_dn5, locals.var_w_sub0_dn6, locals.var_w_sub0_dn7, locals.var_w_sub0_dn8, locals.var_w_sub0_dn9, locals.var_w_sub0_dn10, locals.var_w_sub0_dn11, locals.var_w_sub0_dn14,)
    }
};
        locals.var_w_sub0 = assign24820_e20435;
        locals.var_w_sub0_dn0 = assign24820_e20435_d_n0;
        locals.var_w_sub0_dn2 = assign24820_e20435_d_n2;
        locals.var_w_sub0_dn4 = assign24820_e20435_d_n4;
        locals.var_w_sub0_dn5 = assign24820_e20435_d_n5;
        locals.var_w_sub0_dn6 = assign24820_e20435_d_n6;
        locals.var_w_sub0_dn7 = assign24820_e20435_d_n7;
        locals.var_w_sub0_dn8 = assign24820_e20435_d_n8;
        locals.var_w_sub0_dn9 = assign24820_e20435_d_n9;
        locals.var_w_sub0_dn10 = assign24820_e20435_d_n10;
        locals.var_w_sub0_dn11 = assign24820_e20435_d_n11;
        locals.var_w_sub0_dn14 = assign24820_e20435_d_n14;
        locals.var_w_sub0_rv = 0.0;

        let (assign24830_e20452, assign24830_e20452_d_n0, assign24830_e20452_d_n2, assign24830_e20452_d_n4, assign24830_e20452_d_n5, assign24830_e20452_d_n6, assign24830_e20452_d_n7, assign24830_e20452_d_n8, assign24830_e20452_d_n9, assign24830_e20452_d_n10, assign24830_e20452_d_n11, assign24830_e20452_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 == 0.0)) {
        let assign24830_e20444: f64 = (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0);
        let assign24830_e20446: f64 = (assign24830_e20444 * locals.var_w_sub0);
        let assign24830_e20448: f64 = (assign24830_e20446 + locals.var_vbscl__blk439);
        let assign24830_e20450: f64 = (assign24830_e20448 - locals.var_vbi_dep);
        (assign24830_e20450, ((((((locals.var_c_2esipq_nsub_inv_dn0 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn0)) * locals.var_w_sub0) + (assign24830_e20444 * locals.var_w_sub0_dn0)) + locals.var_vbscl__blk439_dn0) - locals.var_vbi_dep_dn0), ((((((locals.var_c_2esipq_nsub_inv_dn2 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn2)) * locals.var_w_sub0) + (assign24830_e20444 * locals.var_w_sub0_dn2)) + locals.var_vbscl__blk439_dn2) - locals.var_vbi_dep_dn2), ((((((locals.var_c_2esipq_nsub_inv_dn4 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn4)) * locals.var_w_sub0) + (assign24830_e20444 * locals.var_w_sub0_dn4)) + locals.var_vbscl__blk439_dn4) - locals.var_vbi_dep_dn4), ((((((locals.var_c_2esipq_nsub_inv_dn5 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn5)) * locals.var_w_sub0) + (assign24830_e20444 * locals.var_w_sub0_dn5)) + locals.var_vbscl__blk439_dn5) - locals.var_vbi_dep_dn5), ((((((locals.var_c_2esipq_nsub_inv_dn6 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn6)) * locals.var_w_sub0) + (assign24830_e20444 * locals.var_w_sub0_dn6)) + locals.var_vbscl__blk439_dn6) - locals.var_vbi_dep_dn6), ((((((locals.var_c_2esipq_nsub_inv_dn7 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn7)) * locals.var_w_sub0) + (assign24830_e20444 * locals.var_w_sub0_dn7)) + locals.var_vbscl__blk439_dn7) - locals.var_vbi_dep_dn7), ((((((locals.var_c_2esipq_nsub_inv_dn8 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn8)) * locals.var_w_sub0) + (assign24830_e20444 * locals.var_w_sub0_dn8)) + locals.var_vbscl__blk439_dn8) - locals.var_vbi_dep_dn8), ((((((locals.var_c_2esipq_nsub_inv_dn9 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn9)) * locals.var_w_sub0) + (assign24830_e20444 * locals.var_w_sub0_dn9)) + locals.var_vbscl__blk439_dn9) - locals.var_vbi_dep_dn9), ((((((locals.var_c_2esipq_nsub_inv_dn10 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn10)) * locals.var_w_sub0) + (assign24830_e20444 * locals.var_w_sub0_dn10)) + locals.var_vbscl__blk439_dn10) - locals.var_vbi_dep_dn10), ((((((locals.var_c_2esipq_nsub_inv_dn11 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn11)) * locals.var_w_sub0) + (assign24830_e20444 * locals.var_w_sub0_dn11)) + locals.var_vbscl__blk439_dn11) - locals.var_vbi_dep_dn11), ((((((locals.var_c_2esipq_nsub_inv_dn14 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn14)) * locals.var_w_sub0) + (assign24830_e20444 * locals.var_w_sub0_dn14)) + locals.var_vbscl__blk439_dn14) - locals.var_vbi_dep_dn14),)
    } else {
        (locals.var_phi_j0_dep, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn11, locals.var_phi_j0_dep_dn14,)
    }
};
        locals.var_phi_j0_dep = assign24830_e20452;
        locals.var_phi_j0_dep_dn0 = assign24830_e20452_d_n0;
        locals.var_phi_j0_dep_dn2 = assign24830_e20452_d_n2;
        locals.var_phi_j0_dep_dn4 = assign24830_e20452_d_n4;
        locals.var_phi_j0_dep_dn5 = assign24830_e20452_d_n5;
        locals.var_phi_j0_dep_dn6 = assign24830_e20452_d_n6;
        locals.var_phi_j0_dep_dn7 = assign24830_e20452_d_n7;
        locals.var_phi_j0_dep_dn8 = assign24830_e20452_d_n8;
        locals.var_phi_j0_dep_dn9 = assign24830_e20452_d_n9;
        locals.var_phi_j0_dep_dn10 = assign24830_e20452_d_n10;
        locals.var_phi_j0_dep_dn11 = assign24830_e20452_d_n11;
        locals.var_phi_j0_dep_dn14 = assign24830_e20452_d_n14;
        locals.var_phi_j0_dep_rv = 0.0;

        let (assign24840_e20467, assign24840_e20467_d_n0, assign24840_e20467_d_n2, assign24840_e20467_d_n4, assign24840_e20467_d_n5, assign24840_e20467_d_n6, assign24840_e20467_d_n7, assign24840_e20467_d_n8, assign24840_e20467_d_n9, assign24840_e20467_d_n10, assign24840_e20467_d_n11, assign24840_e20467_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 == 0.0)) {
        let assign24840_e20461: f64 = (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0);
        let assign24840_e20463: f64 = (assign24840_e20461 * locals.var_w_b0);
        let assign24840_e20465: f64 = (assign24840_e20463 + locals.var_phi_j0_dep);
        (assign24840_e20465, (((((locals.var_c_2esipq_ndepm_inv_dn0 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn0)) * locals.var_w_b0) + (assign24840_e20461 * locals.var_w_b0_dn0)) + locals.var_phi_j0_dep_dn0), (((((locals.var_c_2esipq_ndepm_inv_dn2 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn2)) * locals.var_w_b0) + (assign24840_e20461 * locals.var_w_b0_dn2)) + locals.var_phi_j0_dep_dn2), (((((locals.var_c_2esipq_ndepm_inv_dn4 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn4)) * locals.var_w_b0) + (assign24840_e20461 * locals.var_w_b0_dn4)) + locals.var_phi_j0_dep_dn4), (((((locals.var_c_2esipq_ndepm_inv_dn5 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn5)) * locals.var_w_b0) + (assign24840_e20461 * locals.var_w_b0_dn5)) + locals.var_phi_j0_dep_dn5), (((((locals.var_c_2esipq_ndepm_inv_dn6 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn6)) * locals.var_w_b0) + (assign24840_e20461 * locals.var_w_b0_dn6)) + locals.var_phi_j0_dep_dn6), (((((locals.var_c_2esipq_ndepm_inv_dn7 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn7)) * locals.var_w_b0) + (assign24840_e20461 * locals.var_w_b0_dn7)) + locals.var_phi_j0_dep_dn7), (((((locals.var_c_2esipq_ndepm_inv_dn8 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn8)) * locals.var_w_b0) + (assign24840_e20461 * locals.var_w_b0_dn8)) + locals.var_phi_j0_dep_dn8), (((((locals.var_c_2esipq_ndepm_inv_dn9 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn9)) * locals.var_w_b0) + (assign24840_e20461 * locals.var_w_b0_dn9)) + locals.var_phi_j0_dep_dn9), (((((locals.var_c_2esipq_ndepm_inv_dn10 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn10)) * locals.var_w_b0) + (assign24840_e20461 * locals.var_w_b0_dn10)) + locals.var_phi_j0_dep_dn10), (((((locals.var_c_2esipq_ndepm_inv_dn11 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn11)) * locals.var_w_b0) + (assign24840_e20461 * locals.var_w_b0_dn11)) + locals.var_phi_j0_dep_dn11), (((((locals.var_c_2esipq_ndepm_inv_dn14 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn14)) * locals.var_w_b0) + (assign24840_e20461 * locals.var_w_b0_dn14)) + locals.var_phi_j0_dep_dn14),)
    } else {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn11, locals.var_phi_b0_dep_dn14,)
    }
};
        locals.var_phi_b0_dep = assign24840_e20467;
        locals.var_phi_b0_dep_dn0 = assign24840_e20467_d_n0;
        locals.var_phi_b0_dep_dn2 = assign24840_e20467_d_n2;
        locals.var_phi_b0_dep_dn4 = assign24840_e20467_d_n4;
        locals.var_phi_b0_dep_dn5 = assign24840_e20467_d_n5;
        locals.var_phi_b0_dep_dn6 = assign24840_e20467_d_n6;
        locals.var_phi_b0_dep_dn7 = assign24840_e20467_d_n7;
        locals.var_phi_b0_dep_dn8 = assign24840_e20467_d_n8;
        locals.var_phi_b0_dep_dn9 = assign24840_e20467_d_n9;
        locals.var_phi_b0_dep_dn10 = assign24840_e20467_d_n10;
        locals.var_phi_b0_dep_dn11 = assign24840_e20467_d_n11;
        locals.var_phi_b0_dep_dn14 = assign24840_e20467_d_n14;
        locals.var_phi_b0_dep_rv = 0.0;

        let (assign24850_e20476, assign24850_e20476_d_n0, assign24850_e20476_d_n2, assign24850_e20476_d_n4, assign24850_e20476_d_n5, assign24850_e20476_d_n6, assign24850_e20476_d_n7, assign24850_e20476_d_n8, assign24850_e20476_d_n9, assign24850_e20476_d_n10, assign24850_e20476_d_n11, assign24850_e20476_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 == 0.0)) {
        (locals.var_phi_j0_dep, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn11, locals.var_phi_j0_dep_dn14,)
    } else {
        (locals.var_phi_j0_dep_acc, locals.var_phi_j0_dep_acc_dn0, locals.var_phi_j0_dep_acc_dn2, locals.var_phi_j0_dep_acc_dn4, locals.var_phi_j0_dep_acc_dn5, locals.var_phi_j0_dep_acc_dn6, locals.var_phi_j0_dep_acc_dn7, locals.var_phi_j0_dep_acc_dn8, locals.var_phi_j0_dep_acc_dn9, locals.var_phi_j0_dep_acc_dn10, locals.var_phi_j0_dep_acc_dn11, locals.var_phi_j0_dep_acc_dn14,)
    }
};
        locals.var_phi_j0_dep_acc = assign24850_e20476;
        locals.var_phi_j0_dep_acc_dn0 = assign24850_e20476_d_n0;
        locals.var_phi_j0_dep_acc_dn2 = assign24850_e20476_d_n2;
        locals.var_phi_j0_dep_acc_dn4 = assign24850_e20476_d_n4;
        locals.var_phi_j0_dep_acc_dn5 = assign24850_e20476_d_n5;
        locals.var_phi_j0_dep_acc_dn6 = assign24850_e20476_d_n6;
        locals.var_phi_j0_dep_acc_dn7 = assign24850_e20476_d_n7;
        locals.var_phi_j0_dep_acc_dn8 = assign24850_e20476_d_n8;
        locals.var_phi_j0_dep_acc_dn9 = assign24850_e20476_d_n9;
        locals.var_phi_j0_dep_acc_dn10 = assign24850_e20476_d_n10;
        locals.var_phi_j0_dep_acc_dn11 = assign24850_e20476_d_n11;
        locals.var_phi_j0_dep_acc_dn14 = assign24850_e20476_d_n14;
        locals.var_phi_j0_dep_acc_rv = 0.0;

        let assign24860_e20479: f64 = if locals.var_vgp > locals.var_vgp0 { 1.0 } else { 0.0 };
        locals.var_guard580 = assign24860_e20479;
        locals.var_guard580_rv = 0.0;

        let (assign24870_e20490,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 == 0.0)) && (locals.var_guard580 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_depmode,)
    }
};
        locals.var_depmode = assign24870_e20490;
        locals.var_depmode_rv = 0.0;

        let (assign24880_e20502,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard563 == 0.0)) && (locals.var_guard580 == 0.0)) {
        (2.0,)
    } else {
        (locals.var_depmode,)
    }
};
        locals.var_depmode = assign24880_e20502;
        locals.var_depmode_rv = 0.0;

        let (assign24890_e20515, assign24890_e20515_d_n0, assign24890_e20515_d_n2, assign24890_e20515_d_n4, assign24890_e20515_d_n5, assign24890_e20515_d_n6, assign24890_e20515_d_n7, assign24890_e20515_d_n8, assign24890_e20515_d_n9, assign24890_e20515_d_n10, assign24890_e20515_d_n11, assign24890_e20515_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign24890_e20509: f64 = (-locals.var_pb2n);
        let assign24890_e20511: f64 = (assign24890_e20509 + locals.var_vbscl__blk439);
        let assign24890_e20512: f64 = (locals.var_psbmax - assign24890_e20511);
        let assign24890_e20513: f64 = (locals.var_c_2esi_q_ndepm * assign24890_e20512);
        (assign24890_e20513, ((locals.var_c_2esi_q_ndepm_dn0 * assign24890_e20512) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn0 - ((-locals.var_pb2n_dn0) + locals.var_vbscl__blk439_dn0)))), ((locals.var_c_2esi_q_ndepm_dn2 * assign24890_e20512) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn2 - ((-locals.var_pb2n_dn2) + locals.var_vbscl__blk439_dn2)))), ((locals.var_c_2esi_q_ndepm_dn4 * assign24890_e20512) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn4 - ((-locals.var_pb2n_dn4) + locals.var_vbscl__blk439_dn4)))), ((locals.var_c_2esi_q_ndepm_dn5 * assign24890_e20512) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn5 - ((-locals.var_pb2n_dn5) + locals.var_vbscl__blk439_dn5)))), ((locals.var_c_2esi_q_ndepm_dn6 * assign24890_e20512) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn6 - ((-locals.var_pb2n_dn6) + locals.var_vbscl__blk439_dn6)))), ((locals.var_c_2esi_q_ndepm_dn7 * assign24890_e20512) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn7 - ((-locals.var_pb2n_dn7) + locals.var_vbscl__blk439_dn7)))), ((locals.var_c_2esi_q_ndepm_dn8 * assign24890_e20512) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn8 - ((-locals.var_pb2n_dn8) + locals.var_vbscl__blk439_dn8)))), ((locals.var_c_2esi_q_ndepm_dn9 * assign24890_e20512) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn9 - ((-locals.var_pb2n_dn9) + locals.var_vbscl__blk439_dn9)))), ((locals.var_c_2esi_q_ndepm_dn10 * assign24890_e20512) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn10 - ((-locals.var_pb2n_dn10) + locals.var_vbscl__blk439_dn10)))), ((locals.var_c_2esi_q_ndepm_dn11 * assign24890_e20512) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn11 - ((-locals.var_pb2n_dn11) + locals.var_vbscl__blk439_dn11)))), ((locals.var_c_2esi_q_ndepm_dn14 * assign24890_e20512) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn14 - ((-locals.var_pb2n_dn14) + locals.var_vbscl__blk439_dn14)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign24890_e20515;
        locals.var_t1_dn0 = assign24890_e20515_d_n0;
        locals.var_t1_dn2 = assign24890_e20515_d_n2;
        locals.var_t1_dn4 = assign24890_e20515_d_n4;
        locals.var_t1_dn5 = assign24890_e20515_d_n5;
        locals.var_t1_dn6 = assign24890_e20515_d_n6;
        locals.var_t1_dn7 = assign24890_e20515_d_n7;
        locals.var_t1_dn8 = assign24890_e20515_d_n8;
        locals.var_t1_dn9 = assign24890_e20515_d_n9;
        locals.var_t1_dn10 = assign24890_e20515_d_n10;
        locals.var_t1_dn11 = assign24890_e20515_d_n11;
        locals.var_t1_dn14 = assign24890_e20515_d_n14;
        locals.var_t1_rv = 0.0;

        let assign24900_e20518: f64 = if locals.var_t1 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard581 = assign24900_e20518;
        locals.var_guard581_rv = 0.0;

        let (assign24910_e20534, assign24910_e20534_d_n0, assign24910_e20534_d_n2, assign24910_e20534_d_n4, assign24910_e20534_d_n5, assign24910_e20534_d_n6, assign24910_e20534_d_n7, assign24910_e20534_d_n8, assign24910_e20534_d_n9, assign24910_e20534_d_n10, assign24910_e20534_d_n11, assign24910_e20534_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard581 != 0.0)) {
        let assign24910_e20525: f64 = (-locals.var_pb2n);
        let assign24910_e20527: f64 = (assign24910_e20525 + locals.var_vbscl__blk439);
        let assign24910_e20529: f64 = (locals.var_t1).sqrt();
        let assign24910_e20531: f64 = (assign24910_e20529 / locals.var_cox);
        let assign24910_e20532: f64 = (assign24910_e20527 - assign24910_e20531);
        (assign24910_e20532, (((-locals.var_pb2n_dn0) + locals.var_vbscl__blk439_dn0) - ((((locals.var_t1_dn0 / (2.0 * assign24910_e20529)) * locals.var_cox) - (assign24910_e20529 * locals.var_cox_dn0)) / (locals.var_cox * locals.var_cox))), (((-locals.var_pb2n_dn2) + locals.var_vbscl__blk439_dn2) - ((((locals.var_t1_dn2 / (2.0 * assign24910_e20529)) * locals.var_cox) - (assign24910_e20529 * locals.var_cox_dn2)) / (locals.var_cox * locals.var_cox))), (((-locals.var_pb2n_dn4) + locals.var_vbscl__blk439_dn4) - ((((locals.var_t1_dn4 / (2.0 * assign24910_e20529)) * locals.var_cox) - (assign24910_e20529 * locals.var_cox_dn4)) / (locals.var_cox * locals.var_cox))), (((-locals.var_pb2n_dn5) + locals.var_vbscl__blk439_dn5) - ((((locals.var_t1_dn5 / (2.0 * assign24910_e20529)) * locals.var_cox) - (assign24910_e20529 * locals.var_cox_dn5)) / (locals.var_cox * locals.var_cox))), (((-locals.var_pb2n_dn6) + locals.var_vbscl__blk439_dn6) - ((((locals.var_t1_dn6 / (2.0 * assign24910_e20529)) * locals.var_cox) - (assign24910_e20529 * locals.var_cox_dn6)) / (locals.var_cox * locals.var_cox))), (((-locals.var_pb2n_dn7) + locals.var_vbscl__blk439_dn7) - ((((locals.var_t1_dn7 / (2.0 * assign24910_e20529)) * locals.var_cox) - (assign24910_e20529 * locals.var_cox_dn7)) / (locals.var_cox * locals.var_cox))), (((-locals.var_pb2n_dn8) + locals.var_vbscl__blk439_dn8) - ((((locals.var_t1_dn8 / (2.0 * assign24910_e20529)) * locals.var_cox) - (assign24910_e20529 * locals.var_cox_dn8)) / (locals.var_cox * locals.var_cox))), (((-locals.var_pb2n_dn9) + locals.var_vbscl__blk439_dn9) - ((((locals.var_t1_dn9 / (2.0 * assign24910_e20529)) * locals.var_cox) - (assign24910_e20529 * locals.var_cox_dn9)) / (locals.var_cox * locals.var_cox))), (((-locals.var_pb2n_dn10) + locals.var_vbscl__blk439_dn10) - ((((locals.var_t1_dn10 / (2.0 * assign24910_e20529)) * locals.var_cox) - (assign24910_e20529 * locals.var_cox_dn10)) / (locals.var_cox * locals.var_cox))), (((-locals.var_pb2n_dn11) + locals.var_vbscl__blk439_dn11) - ((((locals.var_t1_dn11 / (2.0 * assign24910_e20529)) * locals.var_cox) - (assign24910_e20529 * locals.var_cox_dn11)) / (locals.var_cox * locals.var_cox))), (((-locals.var_pb2n_dn14) + locals.var_vbscl__blk439_dn14) - ((((locals.var_t1_dn14 / (2.0 * assign24910_e20529)) * locals.var_cox) - (assign24910_e20529 * locals.var_cox_dn14)) / (locals.var_cox * locals.var_cox))),)
    } else {
        (locals.var_vthn, locals.var_vthn_dn0, locals.var_vthn_dn2, locals.var_vthn_dn4, locals.var_vthn_dn5, locals.var_vthn_dn6, locals.var_vthn_dn7, locals.var_vthn_dn8, locals.var_vthn_dn9, locals.var_vthn_dn10, locals.var_vthn_dn11, locals.var_vthn_dn14,)
    }
};
        locals.var_vthn = assign24910_e20534;
        locals.var_vthn_dn0 = assign24910_e20534_d_n0;
        locals.var_vthn_dn2 = assign24910_e20534_d_n2;
        locals.var_vthn_dn4 = assign24910_e20534_d_n4;
        locals.var_vthn_dn5 = assign24910_e20534_d_n5;
        locals.var_vthn_dn6 = assign24910_e20534_d_n6;
        locals.var_vthn_dn7 = assign24910_e20534_d_n7;
        locals.var_vthn_dn8 = assign24910_e20534_d_n8;
        locals.var_vthn_dn9 = assign24910_e20534_d_n9;
        locals.var_vthn_dn10 = assign24910_e20534_d_n10;
        locals.var_vthn_dn11 = assign24910_e20534_d_n11;
        locals.var_vthn_dn14 = assign24910_e20534_d_n14;
        locals.var_vthn_rv = 0.0;

        let (assign24920_e20546, assign24920_e20546_d_n0, assign24920_e20546_d_n2, assign24920_e20546_d_n4, assign24920_e20546_d_n5, assign24920_e20546_d_n6, assign24920_e20546_d_n7, assign24920_e20546_d_n8, assign24920_e20546_d_n9, assign24920_e20546_d_n10, assign24920_e20546_d_n11, assign24920_e20546_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard581 == 0.0)) {
        let assign24920_e20542: f64 = (-locals.var_pb2n);
        let assign24920_e20544: f64 = (assign24920_e20542 + locals.var_vbscl__blk439);
        (assign24920_e20544, ((-locals.var_pb2n_dn0) + locals.var_vbscl__blk439_dn0), ((-locals.var_pb2n_dn2) + locals.var_vbscl__blk439_dn2), ((-locals.var_pb2n_dn4) + locals.var_vbscl__blk439_dn4), ((-locals.var_pb2n_dn5) + locals.var_vbscl__blk439_dn5), ((-locals.var_pb2n_dn6) + locals.var_vbscl__blk439_dn6), ((-locals.var_pb2n_dn7) + locals.var_vbscl__blk439_dn7), ((-locals.var_pb2n_dn8) + locals.var_vbscl__blk439_dn8), ((-locals.var_pb2n_dn9) + locals.var_vbscl__blk439_dn9), ((-locals.var_pb2n_dn10) + locals.var_vbscl__blk439_dn10), ((-locals.var_pb2n_dn11) + locals.var_vbscl__blk439_dn11), ((-locals.var_pb2n_dn14) + locals.var_vbscl__blk439_dn14),)
    } else {
        (locals.var_vthn, locals.var_vthn_dn0, locals.var_vthn_dn2, locals.var_vthn_dn4, locals.var_vthn_dn5, locals.var_vthn_dn6, locals.var_vthn_dn7, locals.var_vthn_dn8, locals.var_vthn_dn9, locals.var_vthn_dn10, locals.var_vthn_dn11, locals.var_vthn_dn14,)
    }
};
        locals.var_vthn = assign24920_e20546;
        locals.var_vthn_dn0 = assign24920_e20546_d_n0;
        locals.var_vthn_dn2 = assign24920_e20546_d_n2;
        locals.var_vthn_dn4 = assign24920_e20546_d_n4;
        locals.var_vthn_dn5 = assign24920_e20546_d_n5;
        locals.var_vthn_dn6 = assign24920_e20546_d_n6;
        locals.var_vthn_dn7 = assign24920_e20546_d_n7;
        locals.var_vthn_dn8 = assign24920_e20546_d_n8;
        locals.var_vthn_dn9 = assign24920_e20546_d_n9;
        locals.var_vthn_dn10 = assign24920_e20546_d_n10;
        locals.var_vthn_dn11 = assign24920_e20546_d_n11;
        locals.var_vthn_dn14 = assign24920_e20546_d_n14;
        locals.var_vthn_rv = 0.0;

        let assign24930_e20549: f64 = if locals.var_vgp > locals.var_vgp0 { 1.0 } else { 0.0 };
        locals.var_guard582 = assign24930_e20549;
        locals.var_guard582_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_71(
        locals: &mut StampLocals,
    ) {
        let (assign24940_e20557, assign24940_e20557_d_n0, assign24940_e20557_d_n2, assign24940_e20557_d_n4, assign24940_e20557_d_n5, assign24940_e20557_d_n6, assign24940_e20557_d_n7, assign24940_e20557_d_n8, assign24940_e20557_d_n9, assign24940_e20557_d_n10, assign24940_e20557_d_n11, assign24940_e20557_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard582 != 0.0)) {
        (locals.var_phi_j0_dep_acc, locals.var_phi_j0_dep_acc_dn0, locals.var_phi_j0_dep_acc_dn2, locals.var_phi_j0_dep_acc_dn4, locals.var_phi_j0_dep_acc_dn5, locals.var_phi_j0_dep_acc_dn6, locals.var_phi_j0_dep_acc_dn7, locals.var_phi_j0_dep_acc_dn8, locals.var_phi_j0_dep_acc_dn9, locals.var_phi_j0_dep_acc_dn10, locals.var_phi_j0_dep_acc_dn11, locals.var_phi_j0_dep_acc_dn14,)
    } else {
        (locals.var_phi_j0_dep, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn11, locals.var_phi_j0_dep_dn14,)
    }
};
        locals.var_phi_j0_dep = assign24940_e20557;
        locals.var_phi_j0_dep_dn0 = assign24940_e20557_d_n0;
        locals.var_phi_j0_dep_dn2 = assign24940_e20557_d_n2;
        locals.var_phi_j0_dep_dn4 = assign24940_e20557_d_n4;
        locals.var_phi_j0_dep_dn5 = assign24940_e20557_d_n5;
        locals.var_phi_j0_dep_dn6 = assign24940_e20557_d_n6;
        locals.var_phi_j0_dep_dn7 = assign24940_e20557_d_n7;
        locals.var_phi_j0_dep_dn8 = assign24940_e20557_d_n8;
        locals.var_phi_j0_dep_dn9 = assign24940_e20557_d_n9;
        locals.var_phi_j0_dep_dn10 = assign24940_e20557_d_n10;
        locals.var_phi_j0_dep_dn11 = assign24940_e20557_d_n11;
        locals.var_phi_j0_dep_dn14 = assign24940_e20557_d_n14;
        locals.var_phi_j0_dep_rv = 0.0;

        let (assign24950_e20565, assign24950_e20565_d_n0, assign24950_e20565_d_n2, assign24950_e20565_d_n4, assign24950_e20565_d_n5, assign24950_e20565_d_n6, assign24950_e20565_d_n7, assign24950_e20565_d_n8, assign24950_e20565_d_n9, assign24950_e20565_d_n10, assign24950_e20565_d_n11, assign24950_e20565_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard582 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn11, locals.var_phi_b0_dep_dn14,)
    }
};
        locals.var_phi_b0_dep = assign24950_e20565;
        locals.var_phi_b0_dep_dn0 = assign24950_e20565_d_n0;
        locals.var_phi_b0_dep_dn2 = assign24950_e20565_d_n2;
        locals.var_phi_b0_dep_dn4 = assign24950_e20565_d_n4;
        locals.var_phi_b0_dep_dn5 = assign24950_e20565_d_n5;
        locals.var_phi_b0_dep_dn6 = assign24950_e20565_d_n6;
        locals.var_phi_b0_dep_dn7 = assign24950_e20565_d_n7;
        locals.var_phi_b0_dep_dn8 = assign24950_e20565_d_n8;
        locals.var_phi_b0_dep_dn9 = assign24950_e20565_d_n9;
        locals.var_phi_b0_dep_dn10 = assign24950_e20565_d_n10;
        locals.var_phi_b0_dep_dn11 = assign24950_e20565_d_n11;
        locals.var_phi_b0_dep_dn14 = assign24950_e20565_d_n14;
        locals.var_phi_b0_dep_rv = 0.0;

        let (assign24960_e20586, assign24960_e20586_d_n0, assign24960_e20586_d_n2, assign24960_e20586_d_n4, assign24960_e20586_d_n5, assign24960_e20586_d_n6, assign24960_e20586_d_n7, assign24960_e20586_d_n8, assign24960_e20586_d_n9, assign24960_e20586_d_n10, assign24960_e20586_d_n11, assign24960_e20586_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard582 != 0.0)) {
        let assign24960_e20573: f64 = (locals.var_afact * locals.var_vgp);
        let assign24960_e20575: f64 = (assign24960_e20573 * locals.var_vgp);
        let assign24960_e20576: f64 = (assign24960_e20575).ln();
        let assign24960_e20580: f64 = (2.0 / locals.var_vgp);
        let assign24960_e20581: f64 = (locals.var_beta + assign24960_e20580);
        let assign24960_e20582: f64 = (assign24960_e20576 / assign24960_e20581);
        let assign24960_e20584: f64 = (assign24960_e20582 + locals.var_phi_b0_dep);
        (assign24960_e20584, (((((((((locals.var_afact_dn0 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn0)) * locals.var_vgp) + (assign24960_e20573 * locals.var_vgp_dn0)) / assign24960_e20575) * assign24960_e20581) - (assign24960_e20576 * (locals.var_beta_dn0 + (-((2.0 * locals.var_vgp_dn0) / (locals.var_vgp * locals.var_vgp)))))) / (assign24960_e20581 * assign24960_e20581)) + locals.var_phi_b0_dep_dn0), (((((((((locals.var_afact_dn2 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn2)) * locals.var_vgp) + (assign24960_e20573 * locals.var_vgp_dn2)) / assign24960_e20575) * assign24960_e20581) - (assign24960_e20576 * (locals.var_beta_dn2 + (-((2.0 * locals.var_vgp_dn2) / (locals.var_vgp * locals.var_vgp)))))) / (assign24960_e20581 * assign24960_e20581)) + locals.var_phi_b0_dep_dn2), (((((((((locals.var_afact_dn4 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn4)) * locals.var_vgp) + (assign24960_e20573 * locals.var_vgp_dn4)) / assign24960_e20575) * assign24960_e20581) - (assign24960_e20576 * (locals.var_beta_dn4 + (-((2.0 * locals.var_vgp_dn4) / (locals.var_vgp * locals.var_vgp)))))) / (assign24960_e20581 * assign24960_e20581)) + locals.var_phi_b0_dep_dn4), (((((((((locals.var_afact_dn5 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn5)) * locals.var_vgp) + (assign24960_e20573 * locals.var_vgp_dn5)) / assign24960_e20575) * assign24960_e20581) - (assign24960_e20576 * (locals.var_beta_dn5 + (-((2.0 * locals.var_vgp_dn5) / (locals.var_vgp * locals.var_vgp)))))) / (assign24960_e20581 * assign24960_e20581)) + locals.var_phi_b0_dep_dn5), (((((((((locals.var_afact_dn6 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn6)) * locals.var_vgp) + (assign24960_e20573 * locals.var_vgp_dn6)) / assign24960_e20575) * assign24960_e20581) - (assign24960_e20576 * (locals.var_beta_dn6 + (-((2.0 * locals.var_vgp_dn6) / (locals.var_vgp * locals.var_vgp)))))) / (assign24960_e20581 * assign24960_e20581)) + locals.var_phi_b0_dep_dn6), (((((((((locals.var_afact_dn7 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn7)) * locals.var_vgp) + (assign24960_e20573 * locals.var_vgp_dn7)) / assign24960_e20575) * assign24960_e20581) - (assign24960_e20576 * (locals.var_beta_dn7 + (-((2.0 * locals.var_vgp_dn7) / (locals.var_vgp * locals.var_vgp)))))) / (assign24960_e20581 * assign24960_e20581)) + locals.var_phi_b0_dep_dn7), (((((((((locals.var_afact_dn8 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn8)) * locals.var_vgp) + (assign24960_e20573 * locals.var_vgp_dn8)) / assign24960_e20575) * assign24960_e20581) - (assign24960_e20576 * (locals.var_beta_dn8 + (-((2.0 * locals.var_vgp_dn8) / (locals.var_vgp * locals.var_vgp)))))) / (assign24960_e20581 * assign24960_e20581)) + locals.var_phi_b0_dep_dn8), (((((((((locals.var_afact_dn9 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn9)) * locals.var_vgp) + (assign24960_e20573 * locals.var_vgp_dn9)) / assign24960_e20575) * assign24960_e20581) - (assign24960_e20576 * (locals.var_beta_dn9 + (-((2.0 * locals.var_vgp_dn9) / (locals.var_vgp * locals.var_vgp)))))) / (assign24960_e20581 * assign24960_e20581)) + locals.var_phi_b0_dep_dn9), (((((((((locals.var_afact_dn10 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn10)) * locals.var_vgp) + (assign24960_e20573 * locals.var_vgp_dn10)) / assign24960_e20575) * assign24960_e20581) - (assign24960_e20576 * (locals.var_beta_dn10 + (-((2.0 * locals.var_vgp_dn10) / (locals.var_vgp * locals.var_vgp)))))) / (assign24960_e20581 * assign24960_e20581)) + locals.var_phi_b0_dep_dn10), (((((((((locals.var_afact_dn11 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn11)) * locals.var_vgp) + (assign24960_e20573 * locals.var_vgp_dn11)) / assign24960_e20575) * assign24960_e20581) - (assign24960_e20576 * (locals.var_beta_dn11 + (-((2.0 * locals.var_vgp_dn11) / (locals.var_vgp * locals.var_vgp)))))) / (assign24960_e20581 * assign24960_e20581)) + locals.var_phi_b0_dep_dn11), (((((((((locals.var_afact_dn14 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn14)) * locals.var_vgp) + (assign24960_e20573 * locals.var_vgp_dn14)) / assign24960_e20575) * assign24960_e20581) - (assign24960_e20576 * (locals.var_beta_dn14 + (-((2.0 * locals.var_vgp_dn14) / (locals.var_vgp * locals.var_vgp)))))) / (assign24960_e20581 * assign24960_e20581)) + locals.var_phi_b0_dep_dn14),)
    } else {
        (locals.var_phi_s0_dep_ini, locals.var_phi_s0_dep_ini_dn0, locals.var_phi_s0_dep_ini_dn2, locals.var_phi_s0_dep_ini_dn4, locals.var_phi_s0_dep_ini_dn5, locals.var_phi_s0_dep_ini_dn6, locals.var_phi_s0_dep_ini_dn7, locals.var_phi_s0_dep_ini_dn8, locals.var_phi_s0_dep_ini_dn9, locals.var_phi_s0_dep_ini_dn10, locals.var_phi_s0_dep_ini_dn11, locals.var_phi_s0_dep_ini_dn14,)
    }
};
        locals.var_phi_s0_dep_ini = assign24960_e20586;
        locals.var_phi_s0_dep_ini_dn0 = assign24960_e20586_d_n0;
        locals.var_phi_s0_dep_ini_dn2 = assign24960_e20586_d_n2;
        locals.var_phi_s0_dep_ini_dn4 = assign24960_e20586_d_n4;
        locals.var_phi_s0_dep_ini_dn5 = assign24960_e20586_d_n5;
        locals.var_phi_s0_dep_ini_dn6 = assign24960_e20586_d_n6;
        locals.var_phi_s0_dep_ini_dn7 = assign24960_e20586_d_n7;
        locals.var_phi_s0_dep_ini_dn8 = assign24960_e20586_d_n8;
        locals.var_phi_s0_dep_ini_dn9 = assign24960_e20586_d_n9;
        locals.var_phi_s0_dep_ini_dn10 = assign24960_e20586_d_n10;
        locals.var_phi_s0_dep_ini_dn11 = assign24960_e20586_d_n11;
        locals.var_phi_s0_dep_ini_dn14 = assign24960_e20586_d_n14;
        locals.var_phi_s0_dep_ini_rv = 0.0;

        let assign24970_e20590: f64 = (locals.var_vds_maxb0 + locals.var_ps_conv23);
        let assign24970_e20591: f64 = if locals.var_phi_s0_dep_ini < assign24970_e20590 { 1.0 } else { 0.0 };
        locals.var_guard583 = assign24970_e20591;
        locals.var_guard583_rv = 0.0;

        let (assign24980_e20603, assign24980_e20603_d_n0, assign24980_e20603_d_n2, assign24980_e20603_d_n4, assign24980_e20603_d_n5, assign24980_e20603_d_n6, assign24980_e20603_d_n7, assign24980_e20603_d_n8, assign24980_e20603_d_n9, assign24980_e20603_d_n10, assign24980_e20603_d_n11, assign24980_e20603_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard582 != 0.0)) && (locals.var_guard583 != 0.0)) {
        let assign24980_e20601: f64 = (locals.var_vds_maxb0 + locals.var_ps_conv23);
        (assign24980_e20601, locals.var_vds_maxb0_dn0, locals.var_vds_maxb0_dn2, locals.var_vds_maxb0_dn4, locals.var_vds_maxb0_dn5, locals.var_vds_maxb0_dn6, locals.var_vds_maxb0_dn7, locals.var_vds_maxb0_dn8, locals.var_vds_maxb0_dn9, locals.var_vds_maxb0_dn10, locals.var_vds_maxb0_dn11, locals.var_vds_maxb0_dn14,)
    } else {
        (locals.var_phi_s0_dep_ini, locals.var_phi_s0_dep_ini_dn0, locals.var_phi_s0_dep_ini_dn2, locals.var_phi_s0_dep_ini_dn4, locals.var_phi_s0_dep_ini_dn5, locals.var_phi_s0_dep_ini_dn6, locals.var_phi_s0_dep_ini_dn7, locals.var_phi_s0_dep_ini_dn8, locals.var_phi_s0_dep_ini_dn9, locals.var_phi_s0_dep_ini_dn10, locals.var_phi_s0_dep_ini_dn11, locals.var_phi_s0_dep_ini_dn14,)
    }
};
        locals.var_phi_s0_dep_ini = assign24980_e20603;
        locals.var_phi_s0_dep_ini_dn0 = assign24980_e20603_d_n0;
        locals.var_phi_s0_dep_ini_dn2 = assign24980_e20603_d_n2;
        locals.var_phi_s0_dep_ini_dn4 = assign24980_e20603_d_n4;
        locals.var_phi_s0_dep_ini_dn5 = assign24980_e20603_d_n5;
        locals.var_phi_s0_dep_ini_dn6 = assign24980_e20603_d_n6;
        locals.var_phi_s0_dep_ini_dn7 = assign24980_e20603_d_n7;
        locals.var_phi_s0_dep_ini_dn8 = assign24980_e20603_d_n8;
        locals.var_phi_s0_dep_ini_dn9 = assign24980_e20603_d_n9;
        locals.var_phi_s0_dep_ini_dn10 = assign24980_e20603_d_n10;
        locals.var_phi_s0_dep_ini_dn11 = assign24980_e20603_d_n11;
        locals.var_phi_s0_dep_ini_dn14 = assign24980_e20603_d_n14;
        locals.var_phi_s0_dep_ini_rv = 0.0;

        let assign24990_e20606: f64 = if locals.var_vgp > locals.var_vgp1 { 1.0 } else { 0.0 };
        locals.var_guard584 = assign24990_e20606;
        locals.var_guard584_rv = 0.0;

        let (assign25000_e20617, assign25000_e20617_d_n0, assign25000_e20617_d_n2, assign25000_e20617_d_n4, assign25000_e20617_d_n5, assign25000_e20617_d_n6, assign25000_e20617_d_n7, assign25000_e20617_d_n8, assign25000_e20617_d_n9, assign25000_e20617_d_n10, assign25000_e20617_d_n11, assign25000_e20617_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard584 != 0.0)) {
        (locals.var_phi_s0_dep, locals.var_phi_s0_dep_dn0, locals.var_phi_s0_dep_dn2, locals.var_phi_s0_dep_dn4, locals.var_phi_s0_dep_dn5, locals.var_phi_s0_dep_dn6, locals.var_phi_s0_dep_dn7, locals.var_phi_s0_dep_dn8, locals.var_phi_s0_dep_dn9, locals.var_phi_s0_dep_dn10, locals.var_phi_s0_dep_dn11, locals.var_phi_s0_dep_dn14,)
    } else {
        (locals.var_phi_s0_dep_ini, locals.var_phi_s0_dep_ini_dn0, locals.var_phi_s0_dep_ini_dn2, locals.var_phi_s0_dep_ini_dn4, locals.var_phi_s0_dep_ini_dn5, locals.var_phi_s0_dep_ini_dn6, locals.var_phi_s0_dep_ini_dn7, locals.var_phi_s0_dep_ini_dn8, locals.var_phi_s0_dep_ini_dn9, locals.var_phi_s0_dep_ini_dn10, locals.var_phi_s0_dep_ini_dn11, locals.var_phi_s0_dep_ini_dn14,)
    }
};
        locals.var_phi_s0_dep_ini = assign25000_e20617;
        locals.var_phi_s0_dep_ini_dn0 = assign25000_e20617_d_n0;
        locals.var_phi_s0_dep_ini_dn2 = assign25000_e20617_d_n2;
        locals.var_phi_s0_dep_ini_dn4 = assign25000_e20617_d_n4;
        locals.var_phi_s0_dep_ini_dn5 = assign25000_e20617_d_n5;
        locals.var_phi_s0_dep_ini_dn6 = assign25000_e20617_d_n6;
        locals.var_phi_s0_dep_ini_dn7 = assign25000_e20617_d_n7;
        locals.var_phi_s0_dep_ini_dn8 = assign25000_e20617_d_n8;
        locals.var_phi_s0_dep_ini_dn9 = assign25000_e20617_d_n9;
        locals.var_phi_s0_dep_ini_dn10 = assign25000_e20617_d_n10;
        locals.var_phi_s0_dep_ini_dn11 = assign25000_e20617_d_n11;
        locals.var_phi_s0_dep_ini_dn14 = assign25000_e20617_d_n14;
        locals.var_phi_s0_dep_ini_rv = 0.0;

        let assign25010_e20620: f64 = if locals.var_vgp > locals.var_vthn { 1.0 } else { 0.0 };
        locals.var_guard585 = assign25010_e20620;
        locals.var_guard585_rv = 0.0;

        let (assign25020_e20641, assign25020_e20641_d_n0, assign25020_e20641_d_n2, assign25020_e20641_d_n4, assign25020_e20641_d_n5, assign25020_e20641_d_n6, assign25020_e20641_d_n7, assign25020_e20641_d_n8, assign25020_e20641_d_n9, assign25020_e20641_d_n10, assign25020_e20641_d_n11, assign25020_e20641_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard584 == 0.0)) && (locals.var_guard585 != 0.0)) {
        let assign25020_e20633: f64 = (-2.0);
        let assign25020_e20635: f64 = (assign25020_e20633 * locals.var_afact);
        let assign25020_e20637: f64 = (assign25020_e20635 * locals.var_vgp);
        let assign25020_e20639: f64 = (assign25020_e20637 + locals.var_beta);
        (assign25020_e20639, ((((assign25020_e20633 * locals.var_afact_dn0) * locals.var_vgp) + (assign25020_e20635 * locals.var_vgp_dn0)) + locals.var_beta_dn0), ((((assign25020_e20633 * locals.var_afact_dn2) * locals.var_vgp) + (assign25020_e20635 * locals.var_vgp_dn2)) + locals.var_beta_dn2), ((((assign25020_e20633 * locals.var_afact_dn4) * locals.var_vgp) + (assign25020_e20635 * locals.var_vgp_dn4)) + locals.var_beta_dn4), ((((assign25020_e20633 * locals.var_afact_dn5) * locals.var_vgp) + (assign25020_e20635 * locals.var_vgp_dn5)) + locals.var_beta_dn5), ((((assign25020_e20633 * locals.var_afact_dn6) * locals.var_vgp) + (assign25020_e20635 * locals.var_vgp_dn6)) + locals.var_beta_dn6), ((((assign25020_e20633 * locals.var_afact_dn7) * locals.var_vgp) + (assign25020_e20635 * locals.var_vgp_dn7)) + locals.var_beta_dn7), ((((assign25020_e20633 * locals.var_afact_dn8) * locals.var_vgp) + (assign25020_e20635 * locals.var_vgp_dn8)) + locals.var_beta_dn8), ((((assign25020_e20633 * locals.var_afact_dn9) * locals.var_vgp) + (assign25020_e20635 * locals.var_vgp_dn9)) + locals.var_beta_dn9), ((((assign25020_e20633 * locals.var_afact_dn10) * locals.var_vgp) + (assign25020_e20635 * locals.var_vgp_dn10)) + locals.var_beta_dn10), ((((assign25020_e20633 * locals.var_afact_dn11) * locals.var_vgp) + (assign25020_e20635 * locals.var_vgp_dn11)) + locals.var_beta_dn11), ((((assign25020_e20633 * locals.var_afact_dn14) * locals.var_vgp) + (assign25020_e20635 * locals.var_vgp_dn14)) + locals.var_beta_dn14),)
    } else {
        (locals.var_bfact, locals.var_bfact_dn0, locals.var_bfact_dn2, locals.var_bfact_dn4, locals.var_bfact_dn5, locals.var_bfact_dn6, locals.var_bfact_dn7, locals.var_bfact_dn8, locals.var_bfact_dn9, locals.var_bfact_dn10, locals.var_bfact_dn11, locals.var_bfact_dn14,)
    }
};
        locals.var_bfact = assign25020_e20641;
        locals.var_bfact_dn0 = assign25020_e20641_d_n0;
        locals.var_bfact_dn2 = assign25020_e20641_d_n2;
        locals.var_bfact_dn4 = assign25020_e20641_d_n4;
        locals.var_bfact_dn5 = assign25020_e20641_d_n5;
        locals.var_bfact_dn6 = assign25020_e20641_d_n6;
        locals.var_bfact_dn7 = assign25020_e20641_d_n7;
        locals.var_bfact_dn8 = assign25020_e20641_d_n8;
        locals.var_bfact_dn9 = assign25020_e20641_d_n9;
        locals.var_bfact_dn10 = assign25020_e20641_d_n10;
        locals.var_bfact_dn11 = assign25020_e20641_d_n11;
        locals.var_bfact_dn14 = assign25020_e20641_d_n14;
        locals.var_bfact_rv = 0.0;

        let (assign25030_e20663, assign25030_e20663_d_n0, assign25030_e20663_d_n2, assign25030_e20663_d_n4, assign25030_e20663_d_n5, assign25030_e20663_d_n6, assign25030_e20663_d_n7, assign25030_e20663_d_n8, assign25030_e20663_d_n9, assign25030_e20663_d_n10, assign25030_e20663_d_n11, assign25030_e20663_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard584 == 0.0)) && (locals.var_guard585 != 0.0)) {
        let assign25030_e20655: f64 = (locals.var_afact * locals.var_vgp);
        let assign25030_e20657: f64 = (assign25030_e20655 * locals.var_vgp);
        let assign25030_e20660: f64 = (locals.var_beta * locals.var_phi_b0_dep);
        let assign25030_e20661: f64 = (assign25030_e20657 - assign25030_e20660);
        (assign25030_e20661, (((((locals.var_afact_dn0 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn0)) * locals.var_vgp) + (assign25030_e20655 * locals.var_vgp_dn0)) - ((locals.var_beta_dn0 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn0))), (((((locals.var_afact_dn2 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn2)) * locals.var_vgp) + (assign25030_e20655 * locals.var_vgp_dn2)) - ((locals.var_beta_dn2 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn2))), (((((locals.var_afact_dn4 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn4)) * locals.var_vgp) + (assign25030_e20655 * locals.var_vgp_dn4)) - ((locals.var_beta_dn4 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn4))), (((((locals.var_afact_dn5 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn5)) * locals.var_vgp) + (assign25030_e20655 * locals.var_vgp_dn5)) - ((locals.var_beta_dn5 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn5))), (((((locals.var_afact_dn6 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn6)) * locals.var_vgp) + (assign25030_e20655 * locals.var_vgp_dn6)) - ((locals.var_beta_dn6 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn6))), (((((locals.var_afact_dn7 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn7)) * locals.var_vgp) + (assign25030_e20655 * locals.var_vgp_dn7)) - ((locals.var_beta_dn7 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn7))), (((((locals.var_afact_dn8 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn8)) * locals.var_vgp) + (assign25030_e20655 * locals.var_vgp_dn8)) - ((locals.var_beta_dn8 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn8))), (((((locals.var_afact_dn9 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn9)) * locals.var_vgp) + (assign25030_e20655 * locals.var_vgp_dn9)) - ((locals.var_beta_dn9 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn9))), (((((locals.var_afact_dn10 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn10)) * locals.var_vgp) + (assign25030_e20655 * locals.var_vgp_dn10)) - ((locals.var_beta_dn10 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn10))), (((((locals.var_afact_dn11 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn11)) * locals.var_vgp) + (assign25030_e20655 * locals.var_vgp_dn11)) - ((locals.var_beta_dn11 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn11))), (((((locals.var_afact_dn14 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn14)) * locals.var_vgp) + (assign25030_e20655 * locals.var_vgp_dn14)) - ((locals.var_beta_dn14 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn14))),)
    } else {
        (locals.var_cfact, locals.var_cfact_dn0, locals.var_cfact_dn2, locals.var_cfact_dn4, locals.var_cfact_dn5, locals.var_cfact_dn6, locals.var_cfact_dn7, locals.var_cfact_dn8, locals.var_cfact_dn9, locals.var_cfact_dn10, locals.var_cfact_dn11, locals.var_cfact_dn14,)
    }
};
        locals.var_cfact = assign25030_e20663;
        locals.var_cfact_dn0 = assign25030_e20663_d_n0;
        locals.var_cfact_dn2 = assign25030_e20663_d_n2;
        locals.var_cfact_dn4 = assign25030_e20663_d_n4;
        locals.var_cfact_dn5 = assign25030_e20663_d_n5;
        locals.var_cfact_dn6 = assign25030_e20663_d_n6;
        locals.var_cfact_dn7 = assign25030_e20663_d_n7;
        locals.var_cfact_dn8 = assign25030_e20663_d_n8;
        locals.var_cfact_dn9 = assign25030_e20663_d_n9;
        locals.var_cfact_dn10 = assign25030_e20663_d_n10;
        locals.var_cfact_dn11 = assign25030_e20663_d_n11;
        locals.var_cfact_dn14 = assign25030_e20663_d_n14;
        locals.var_cfact_rv = 0.0;

        let (assign25040_e20677, assign25040_e20677_d_n0, assign25040_e20677_d_n2, assign25040_e20677_d_n4, assign25040_e20677_d_n5, assign25040_e20677_d_n6, assign25040_e20677_d_n7, assign25040_e20677_d_n8, assign25040_e20677_d_n9, assign25040_e20677_d_n10, assign25040_e20677_d_n11, assign25040_e20677_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard584 == 0.0)) && (locals.var_guard585 != 0.0)) {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn11, locals.var_phi_b0_dep_dn14,)
    } else {
        (locals.var_phi_b0_dep_old, locals.var_phi_b0_dep_old_dn0, locals.var_phi_b0_dep_old_dn2, locals.var_phi_b0_dep_old_dn4, locals.var_phi_b0_dep_old_dn5, locals.var_phi_b0_dep_old_dn6, locals.var_phi_b0_dep_old_dn7, locals.var_phi_b0_dep_old_dn8, locals.var_phi_b0_dep_old_dn9, locals.var_phi_b0_dep_old_dn10, locals.var_phi_b0_dep_old_dn11, locals.var_phi_b0_dep_old_dn14,)
    }
};
        locals.var_phi_b0_dep_old = assign25040_e20677;
        locals.var_phi_b0_dep_old_dn0 = assign25040_e20677_d_n0;
        locals.var_phi_b0_dep_old_dn2 = assign25040_e20677_d_n2;
        locals.var_phi_b0_dep_old_dn4 = assign25040_e20677_d_n4;
        locals.var_phi_b0_dep_old_dn5 = assign25040_e20677_d_n5;
        locals.var_phi_b0_dep_old_dn6 = assign25040_e20677_d_n6;
        locals.var_phi_b0_dep_old_dn7 = assign25040_e20677_d_n7;
        locals.var_phi_b0_dep_old_dn8 = assign25040_e20677_d_n8;
        locals.var_phi_b0_dep_old_dn9 = assign25040_e20677_d_n9;
        locals.var_phi_b0_dep_old_dn10 = assign25040_e20677_d_n10;
        locals.var_phi_b0_dep_old_dn11 = assign25040_e20677_d_n11;
        locals.var_phi_b0_dep_old_dn14 = assign25040_e20677_d_n14;
        locals.var_phi_b0_dep_old_rv = 0.0;

        let (assign25050_e20707, assign25050_e20707_d_n0, assign25050_e20707_d_n2, assign25050_e20707_d_n4, assign25050_e20707_d_n5, assign25050_e20707_d_n6, assign25050_e20707_d_n7, assign25050_e20707_d_n8, assign25050_e20707_d_n9, assign25050_e20707_d_n10, assign25050_e20707_d_n11, assign25050_e20707_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard584 == 0.0)) && (locals.var_guard585 != 0.0)) {
        let assign25050_e20690: f64 = (-locals.var_bfact);
        let assign25050_e20693: f64 = (locals.var_bfact * locals.var_bfact);
        let assign25050_e20696: f64 = (4.0 * locals.var_afact);
        let assign25050_e20698: f64 = (assign25050_e20696 * locals.var_cfact);
        let assign25050_e20699: f64 = (assign25050_e20693 - assign25050_e20698);
        let assign25050_e20700: f64 = (assign25050_e20699).sqrt();
        let assign25050_e20701: f64 = (assign25050_e20690 + assign25050_e20700);
        let assign25050_e20703: f64 = (assign25050_e20701 / 2.0);
        let assign25050_e20705: f64 = (assign25050_e20703 / locals.var_afact);
        (assign25050_e20705, ((((((-locals.var_bfact_dn0) + ((((locals.var_bfact_dn0 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn0)) - (((4.0 * locals.var_afact_dn0) * locals.var_cfact) + (assign25050_e20696 * locals.var_cfact_dn0))) / (2.0 * assign25050_e20700))) / 2.0) * locals.var_afact) - (assign25050_e20703 * locals.var_afact_dn0)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn2) + ((((locals.var_bfact_dn2 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn2)) - (((4.0 * locals.var_afact_dn2) * locals.var_cfact) + (assign25050_e20696 * locals.var_cfact_dn2))) / (2.0 * assign25050_e20700))) / 2.0) * locals.var_afact) - (assign25050_e20703 * locals.var_afact_dn2)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn4) + ((((locals.var_bfact_dn4 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn4)) - (((4.0 * locals.var_afact_dn4) * locals.var_cfact) + (assign25050_e20696 * locals.var_cfact_dn4))) / (2.0 * assign25050_e20700))) / 2.0) * locals.var_afact) - (assign25050_e20703 * locals.var_afact_dn4)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn5) + ((((locals.var_bfact_dn5 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn5)) - (((4.0 * locals.var_afact_dn5) * locals.var_cfact) + (assign25050_e20696 * locals.var_cfact_dn5))) / (2.0 * assign25050_e20700))) / 2.0) * locals.var_afact) - (assign25050_e20703 * locals.var_afact_dn5)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn6) + ((((locals.var_bfact_dn6 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn6)) - (((4.0 * locals.var_afact_dn6) * locals.var_cfact) + (assign25050_e20696 * locals.var_cfact_dn6))) / (2.0 * assign25050_e20700))) / 2.0) * locals.var_afact) - (assign25050_e20703 * locals.var_afact_dn6)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn7) + ((((locals.var_bfact_dn7 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn7)) - (((4.0 * locals.var_afact_dn7) * locals.var_cfact) + (assign25050_e20696 * locals.var_cfact_dn7))) / (2.0 * assign25050_e20700))) / 2.0) * locals.var_afact) - (assign25050_e20703 * locals.var_afact_dn7)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn8) + ((((locals.var_bfact_dn8 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn8)) - (((4.0 * locals.var_afact_dn8) * locals.var_cfact) + (assign25050_e20696 * locals.var_cfact_dn8))) / (2.0 * assign25050_e20700))) / 2.0) * locals.var_afact) - (assign25050_e20703 * locals.var_afact_dn8)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn9) + ((((locals.var_bfact_dn9 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn9)) - (((4.0 * locals.var_afact_dn9) * locals.var_cfact) + (assign25050_e20696 * locals.var_cfact_dn9))) / (2.0 * assign25050_e20700))) / 2.0) * locals.var_afact) - (assign25050_e20703 * locals.var_afact_dn9)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn10) + ((((locals.var_bfact_dn10 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn10)) - (((4.0 * locals.var_afact_dn10) * locals.var_cfact) + (assign25050_e20696 * locals.var_cfact_dn10))) / (2.0 * assign25050_e20700))) / 2.0) * locals.var_afact) - (assign25050_e20703 * locals.var_afact_dn10)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn11) + ((((locals.var_bfact_dn11 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn11)) - (((4.0 * locals.var_afact_dn11) * locals.var_cfact) + (assign25050_e20696 * locals.var_cfact_dn11))) / (2.0 * assign25050_e20700))) / 2.0) * locals.var_afact) - (assign25050_e20703 * locals.var_afact_dn11)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn14) + ((((locals.var_bfact_dn14 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn14)) - (((4.0 * locals.var_afact_dn14) * locals.var_cfact) + (assign25050_e20696 * locals.var_cfact_dn14))) / (2.0 * assign25050_e20700))) / 2.0) * locals.var_afact) - (assign25050_e20703 * locals.var_afact_dn14)) / (locals.var_afact * locals.var_afact)),)
    } else {
        (locals.var_phi_s0_dep_ini, locals.var_phi_s0_dep_ini_dn0, locals.var_phi_s0_dep_ini_dn2, locals.var_phi_s0_dep_ini_dn4, locals.var_phi_s0_dep_ini_dn5, locals.var_phi_s0_dep_ini_dn6, locals.var_phi_s0_dep_ini_dn7, locals.var_phi_s0_dep_ini_dn8, locals.var_phi_s0_dep_ini_dn9, locals.var_phi_s0_dep_ini_dn10, locals.var_phi_s0_dep_ini_dn11, locals.var_phi_s0_dep_ini_dn14,)
    }
};
        locals.var_phi_s0_dep_ini = assign25050_e20707;
        locals.var_phi_s0_dep_ini_dn0 = assign25050_e20707_d_n0;
        locals.var_phi_s0_dep_ini_dn2 = assign25050_e20707_d_n2;
        locals.var_phi_s0_dep_ini_dn4 = assign25050_e20707_d_n4;
        locals.var_phi_s0_dep_ini_dn5 = assign25050_e20707_d_n5;
        locals.var_phi_s0_dep_ini_dn6 = assign25050_e20707_d_n6;
        locals.var_phi_s0_dep_ini_dn7 = assign25050_e20707_d_n7;
        locals.var_phi_s0_dep_ini_dn8 = assign25050_e20707_d_n8;
        locals.var_phi_s0_dep_ini_dn9 = assign25050_e20707_d_n9;
        locals.var_phi_s0_dep_ini_dn10 = assign25050_e20707_d_n10;
        locals.var_phi_s0_dep_ini_dn11 = assign25050_e20707_d_n11;
        locals.var_phi_s0_dep_ini_dn14 = assign25050_e20707_d_n14;
        locals.var_phi_s0_dep_ini_rv = 0.0;

        let assign25060_e20711: f64 = (locals.var_psbmax - locals.var_ps_conv3);
        let assign25060_e20712: f64 = if locals.var_phi_s0_dep_ini > assign25060_e20711 { 1.0 } else { 0.0 };
        locals.var_guard586 = assign25060_e20712;
        locals.var_guard586_rv = 0.0;

        let (assign25070_e20730, assign25070_e20730_d_n0, assign25070_e20730_d_n2, assign25070_e20730_d_n4, assign25070_e20730_d_n5, assign25070_e20730_d_n6, assign25070_e20730_d_n7, assign25070_e20730_d_n8, assign25070_e20730_d_n9, assign25070_e20730_d_n10, assign25070_e20730_d_n11, assign25070_e20730_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard584 == 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 != 0.0)) {
        let assign25070_e20728: f64 = (locals.var_psbmax - locals.var_ps_conv3);
        (assign25070_e20728, locals.var_psbmax_dn0, locals.var_psbmax_dn2, locals.var_psbmax_dn4, locals.var_psbmax_dn5, locals.var_psbmax_dn6, locals.var_psbmax_dn7, locals.var_psbmax_dn8, locals.var_psbmax_dn9, locals.var_psbmax_dn10, locals.var_psbmax_dn11, locals.var_psbmax_dn14,)
    } else {
        (locals.var_phi_s0_dep_ini, locals.var_phi_s0_dep_ini_dn0, locals.var_phi_s0_dep_ini_dn2, locals.var_phi_s0_dep_ini_dn4, locals.var_phi_s0_dep_ini_dn5, locals.var_phi_s0_dep_ini_dn6, locals.var_phi_s0_dep_ini_dn7, locals.var_phi_s0_dep_ini_dn8, locals.var_phi_s0_dep_ini_dn9, locals.var_phi_s0_dep_ini_dn10, locals.var_phi_s0_dep_ini_dn11, locals.var_phi_s0_dep_ini_dn14,)
    }
};
        locals.var_phi_s0_dep_ini = assign25070_e20730;
        locals.var_phi_s0_dep_ini_dn0 = assign25070_e20730_d_n0;
        locals.var_phi_s0_dep_ini_dn2 = assign25070_e20730_d_n2;
        locals.var_phi_s0_dep_ini_dn4 = assign25070_e20730_d_n4;
        locals.var_phi_s0_dep_ini_dn5 = assign25070_e20730_d_n5;
        locals.var_phi_s0_dep_ini_dn6 = assign25070_e20730_d_n6;
        locals.var_phi_s0_dep_ini_dn7 = assign25070_e20730_d_n7;
        locals.var_phi_s0_dep_ini_dn8 = assign25070_e20730_d_n8;
        locals.var_phi_s0_dep_ini_dn9 = assign25070_e20730_d_n9;
        locals.var_phi_s0_dep_ini_dn10 = assign25070_e20730_d_n10;
        locals.var_phi_s0_dep_ini_dn11 = assign25070_e20730_d_n11;
        locals.var_phi_s0_dep_ini_dn14 = assign25070_e20730_d_n14;
        locals.var_phi_s0_dep_ini_rv = 0.0;

        let (assign25080_e20749, assign25080_e20749_d_n0, assign25080_e20749_d_n2, assign25080_e20749_d_n4, assign25080_e20749_d_n5, assign25080_e20749_d_n6, assign25080_e20749_d_n7, assign25080_e20749_d_n8, assign25080_e20749_d_n9, assign25080_e20749_d_n10, assign25080_e20749_d_n11, assign25080_e20749_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard584 == 0.0)) && (locals.var_guard585 != 0.0)) {
        let assign25080_e20745: f64 = (locals.var_phi_b0_dep - locals.var_phi_s0_dep_ini);
        let assign25080_e20746: f64 = (locals.var_c_2esipq_ndepm * assign25080_e20745);
        let assign25080_e20747: f64 = (assign25080_e20746).sqrt();
        (assign25080_e20747, (((locals.var_c_2esipq_ndepm_dn0 * assign25080_e20745) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn0 - locals.var_phi_s0_dep_ini_dn0))) / (2.0 * assign25080_e20747)), (((locals.var_c_2esipq_ndepm_dn2 * assign25080_e20745) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn2 - locals.var_phi_s0_dep_ini_dn2))) / (2.0 * assign25080_e20747)), (((locals.var_c_2esipq_ndepm_dn4 * assign25080_e20745) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn4 - locals.var_phi_s0_dep_ini_dn4))) / (2.0 * assign25080_e20747)), (((locals.var_c_2esipq_ndepm_dn5 * assign25080_e20745) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn5 - locals.var_phi_s0_dep_ini_dn5))) / (2.0 * assign25080_e20747)), (((locals.var_c_2esipq_ndepm_dn6 * assign25080_e20745) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn6 - locals.var_phi_s0_dep_ini_dn6))) / (2.0 * assign25080_e20747)), (((locals.var_c_2esipq_ndepm_dn7 * assign25080_e20745) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn7 - locals.var_phi_s0_dep_ini_dn7))) / (2.0 * assign25080_e20747)), (((locals.var_c_2esipq_ndepm_dn8 * assign25080_e20745) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn8 - locals.var_phi_s0_dep_ini_dn8))) / (2.0 * assign25080_e20747)), (((locals.var_c_2esipq_ndepm_dn9 * assign25080_e20745) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn9 - locals.var_phi_s0_dep_ini_dn9))) / (2.0 * assign25080_e20747)), (((locals.var_c_2esipq_ndepm_dn10 * assign25080_e20745) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn10 - locals.var_phi_s0_dep_ini_dn10))) / (2.0 * assign25080_e20747)), (((locals.var_c_2esipq_ndepm_dn11 * assign25080_e20745) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn11 - locals.var_phi_s0_dep_ini_dn11))) / (2.0 * assign25080_e20747)), (((locals.var_c_2esipq_ndepm_dn14 * assign25080_e20745) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn14 - locals.var_phi_s0_dep_ini_dn14))) / (2.0 * assign25080_e20747)),)
    } else {
        (locals.var_w_s0, locals.var_w_s0_dn0, locals.var_w_s0_dn2, locals.var_w_s0_dn4, locals.var_w_s0_dn5, locals.var_w_s0_dn6, locals.var_w_s0_dn7, locals.var_w_s0_dn8, locals.var_w_s0_dn9, locals.var_w_s0_dn10, locals.var_w_s0_dn11, locals.var_w_s0_dn14,)
    }
};
        locals.var_w_s0 = assign25080_e20749;
        locals.var_w_s0_dn0 = assign25080_e20749_d_n0;
        locals.var_w_s0_dn2 = assign25080_e20749_d_n2;
        locals.var_w_s0_dn4 = assign25080_e20749_d_n4;
        locals.var_w_s0_dn5 = assign25080_e20749_d_n5;
        locals.var_w_s0_dn6 = assign25080_e20749_d_n6;
        locals.var_w_s0_dn7 = assign25080_e20749_d_n7;
        locals.var_w_s0_dn8 = assign25080_e20749_d_n8;
        locals.var_w_s0_dn9 = assign25080_e20749_d_n9;
        locals.var_w_s0_dn10 = assign25080_e20749_d_n10;
        locals.var_w_s0_dn11 = assign25080_e20749_d_n11;
        locals.var_w_s0_dn14 = assign25080_e20749_d_n14;
        locals.var_w_s0_rv = 0.0;

        let (assign25090_e20768, assign25090_e20768_d_n0, assign25090_e20768_d_n2, assign25090_e20768_d_n4, assign25090_e20768_d_n5, assign25090_e20768_d_n6, assign25090_e20768_d_n7, assign25090_e20768_d_n8, assign25090_e20768_d_n9, assign25090_e20768_d_n10, assign25090_e20768_d_n11, assign25090_e20768_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard584 == 0.0)) && (locals.var_guard585 != 0.0)) {
        let assign25090_e20764: f64 = (locals.var_phi_b0_dep - locals.var_phi_j0_dep);
        let assign25090_e20765: f64 = (locals.var_c_2esipq_ndepm * assign25090_e20764);
        let assign25090_e20766: f64 = (assign25090_e20765).sqrt();
        (assign25090_e20766, (((locals.var_c_2esipq_ndepm_dn0 * assign25090_e20764) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn0 - locals.var_phi_j0_dep_dn0))) / (2.0 * assign25090_e20766)), (((locals.var_c_2esipq_ndepm_dn2 * assign25090_e20764) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn2 - locals.var_phi_j0_dep_dn2))) / (2.0 * assign25090_e20766)), (((locals.var_c_2esipq_ndepm_dn4 * assign25090_e20764) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn4 - locals.var_phi_j0_dep_dn4))) / (2.0 * assign25090_e20766)), (((locals.var_c_2esipq_ndepm_dn5 * assign25090_e20764) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn5 - locals.var_phi_j0_dep_dn5))) / (2.0 * assign25090_e20766)), (((locals.var_c_2esipq_ndepm_dn6 * assign25090_e20764) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn6 - locals.var_phi_j0_dep_dn6))) / (2.0 * assign25090_e20766)), (((locals.var_c_2esipq_ndepm_dn7 * assign25090_e20764) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn7 - locals.var_phi_j0_dep_dn7))) / (2.0 * assign25090_e20766)), (((locals.var_c_2esipq_ndepm_dn8 * assign25090_e20764) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn8 - locals.var_phi_j0_dep_dn8))) / (2.0 * assign25090_e20766)), (((locals.var_c_2esipq_ndepm_dn9 * assign25090_e20764) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn9 - locals.var_phi_j0_dep_dn9))) / (2.0 * assign25090_e20766)), (((locals.var_c_2esipq_ndepm_dn10 * assign25090_e20764) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn10 - locals.var_phi_j0_dep_dn10))) / (2.0 * assign25090_e20766)), (((locals.var_c_2esipq_ndepm_dn11 * assign25090_e20764) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn11 - locals.var_phi_j0_dep_dn11))) / (2.0 * assign25090_e20766)), (((locals.var_c_2esipq_ndepm_dn14 * assign25090_e20764) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn14 - locals.var_phi_j0_dep_dn14))) / (2.0 * assign25090_e20766)),)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn11, locals.var_w_b0_dn14,)
    }
};
        locals.var_w_b0 = assign25090_e20768;
        locals.var_w_b0_dn0 = assign25090_e20768_d_n0;
        locals.var_w_b0_dn2 = assign25090_e20768_d_n2;
        locals.var_w_b0_dn4 = assign25090_e20768_d_n4;
        locals.var_w_b0_dn5 = assign25090_e20768_d_n5;
        locals.var_w_b0_dn6 = assign25090_e20768_d_n6;
        locals.var_w_b0_dn7 = assign25090_e20768_d_n7;
        locals.var_w_b0_dn8 = assign25090_e20768_d_n8;
        locals.var_w_b0_dn9 = assign25090_e20768_d_n9;
        locals.var_w_b0_dn10 = assign25090_e20768_d_n10;
        locals.var_w_b0_dn11 = assign25090_e20768_d_n11;
        locals.var_w_b0_dn14 = assign25090_e20768_d_n14;
        locals.var_w_b0_rv = 0.0;

        let assign25100_e20771: f64 = (locals.var_w_s0 + locals.var_w_b0);
        let assign25100_e20773: f64 = if assign25100_e20771 > locals.var_uc_depthn { 1.0 } else { 0.0 };
        locals.var_guard587 = assign25100_e20773;
        locals.var_guard587_rv = 0.0;

        let (assign25110_e20789,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard584 == 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard587 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign25110_e20789;
        locals.var_lp_s0_rv = 0.0;

    }
}
