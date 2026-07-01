#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_368(
        locals: &mut StampLocals,
    ) {
        let (assign100430_e152531, assign100430_e152531_d_n0, assign100430_e152531_d_n2, assign100430_e152531_d_n4, assign100430_e152531_d_n5, assign100430_e152531_d_n6, assign100430_e152531_d_n7, assign100430_e152531_d_n8, assign100430_e152531_d_n9, assign100430_e152531_d_n10, assign100430_e152531_d_n11, assign100430_e152531_d_n14,) = {
    if ((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) {
        let assign100430_e152528: f64 = (locals.var_cox0 * locals.var_cox0);
        let assign100430_e152529: f64 = (locals.var_qnsub_esi / assign100430_e152528);
        (assign100430_e152529, (locals.var_qnsub_esi_dn0 / assign100430_e152528), (locals.var_qnsub_esi_dn2 / assign100430_e152528), (locals.var_qnsub_esi_dn4 / assign100430_e152528), (locals.var_qnsub_esi_dn5 / assign100430_e152528), (locals.var_qnsub_esi_dn6 / assign100430_e152528), (locals.var_qnsub_esi_dn7 / assign100430_e152528), (locals.var_qnsub_esi_dn8 / assign100430_e152528), (locals.var_qnsub_esi_dn9 / assign100430_e152528), (locals.var_qnsub_esi_dn10 / assign100430_e152528), (locals.var_qnsub_esi_dn11 / assign100430_e152528), (locals.var_qnsub_esi_dn14 / assign100430_e152528),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign100430_e152531;
        locals.var_t3_dn0 = assign100430_e152531_d_n0;
        locals.var_t3_dn2 = assign100430_e152531_d_n2;
        locals.var_t3_dn4 = assign100430_e152531_d_n4;
        locals.var_t3_dn5 = assign100430_e152531_d_n5;
        locals.var_t3_dn6 = assign100430_e152531_d_n6;
        locals.var_t3_dn7 = assign100430_e152531_d_n7;
        locals.var_t3_dn8 = assign100430_e152531_d_n8;
        locals.var_t3_dn9 = assign100430_e152531_d_n9;
        locals.var_t3_dn10 = assign100430_e152531_d_n10;
        locals.var_t3_dn11 = assign100430_e152531_d_n11;
        locals.var_t3_dn14 = assign100430_e152531_d_n14;

        let (assign100440_e152543, assign100440_e152543_d_n0, assign100440_e152543_d_n2, assign100440_e152543_d_n4, assign100440_e152543_d_n5, assign100440_e152543_d_n6, assign100440_e152543_d_n7, assign100440_e152543_d_n8, assign100440_e152543_d_n9, assign100440_e152543_d_n10, assign100440_e152543_d_n11, assign100440_e152543_d_n14,) = {
    if ((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) {
        let assign100440_e152537: f64 = (2.0 / locals.var_qnsub_esi);
        let assign100440_e152540: f64 = (locals.var_cox0 * locals.var_cox0);
        let assign100440_e152541: f64 = (assign100440_e152537 * assign100440_e152540);
        (assign100440_e152541, ((-((2.0 * locals.var_qnsub_esi_dn0) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100440_e152540), ((-((2.0 * locals.var_qnsub_esi_dn2) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100440_e152540), ((-((2.0 * locals.var_qnsub_esi_dn4) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100440_e152540), ((-((2.0 * locals.var_qnsub_esi_dn5) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100440_e152540), ((-((2.0 * locals.var_qnsub_esi_dn6) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100440_e152540), ((-((2.0 * locals.var_qnsub_esi_dn7) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100440_e152540), ((-((2.0 * locals.var_qnsub_esi_dn8) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100440_e152540), ((-((2.0 * locals.var_qnsub_esi_dn9) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100440_e152540), ((-((2.0 * locals.var_qnsub_esi_dn10) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100440_e152540), ((-((2.0 * locals.var_qnsub_esi_dn11) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100440_e152540), ((-((2.0 * locals.var_qnsub_esi_dn14) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100440_e152540),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign100440_e152543;
        locals.var_t4_dn0 = assign100440_e152543_d_n0;
        locals.var_t4_dn2 = assign100440_e152543_d_n2;
        locals.var_t4_dn4 = assign100440_e152543_d_n4;
        locals.var_t4_dn5 = assign100440_e152543_d_n5;
        locals.var_t4_dn6 = assign100440_e152543_d_n6;
        locals.var_t4_dn7 = assign100440_e152543_d_n7;
        locals.var_t4_dn8 = assign100440_e152543_d_n8;
        locals.var_t4_dn9 = assign100440_e152543_d_n9;
        locals.var_t4_dn10 = assign100440_e152543_d_n10;
        locals.var_t4_dn11 = assign100440_e152543_d_n11;
        locals.var_t4_dn14 = assign100440_e152543_d_n14;

        let (assign100450_e152555, assign100450_e152555_d_n0, assign100450_e152555_d_n2, assign100450_e152555_d_n4, assign100450_e152555_d_n5, assign100450_e152555_d_n6, assign100450_e152555_d_n7, assign100450_e152555_d_n8, assign100450_e152555_d_n9, assign100450_e152555_d_n10, assign100450_e152555_d_n11, assign100450_e152555_d_n14,) = {
    if ((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) {
        let assign100450_e152549: f64 = (locals.var_t1 - locals.var_beta_inv);
        let assign100450_e152552: f64 = (locals.var_xvbs_1 * locals.var_vbsz__blk440);
        let assign100450_e152553: f64 = (assign100450_e152549 - assign100450_e152552);
        (assign100450_e152553, ((locals.var_t1_dn0 - locals.var_beta_inv_dn0) - (locals.var_xvbs_1 * locals.var_vbsz__blk440_dn0)), ((locals.var_t1_dn2 - locals.var_beta_inv_dn2) - (locals.var_xvbs_1 * locals.var_vbsz__blk440_dn2)), ((locals.var_t1_dn4 - locals.var_beta_inv_dn4) - (locals.var_xvbs_1 * locals.var_vbsz__blk440_dn4)), ((locals.var_t1_dn5 - locals.var_beta_inv_dn5) - (locals.var_xvbs_1 * locals.var_vbsz__blk440_dn5)), ((locals.var_t1_dn6 - locals.var_beta_inv_dn6) - (locals.var_xvbs_1 * locals.var_vbsz__blk440_dn6)), ((locals.var_t1_dn7 - locals.var_beta_inv_dn7) - (locals.var_xvbs_1 * locals.var_vbsz__blk440_dn7)), ((locals.var_t1_dn8 - locals.var_beta_inv_dn8) - (locals.var_xvbs_1 * locals.var_vbsz__blk440_dn8)), ((locals.var_t1_dn9 - locals.var_beta_inv_dn9) - (locals.var_xvbs_1 * locals.var_vbsz__blk440_dn9)), ((locals.var_t1_dn10 - locals.var_beta_inv_dn10) - (locals.var_xvbs_1 * locals.var_vbsz__blk440_dn10)), ((locals.var_t1_dn11 - locals.var_beta_inv_dn11) - (locals.var_xvbs_1 * locals.var_vbsz__blk440_dn11)), ((locals.var_t1_dn14 - locals.var_beta_inv_dn14) - (locals.var_xvbs_1 * locals.var_vbsz__blk440_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign100450_e152555;
        locals.var_t5_dn0 = assign100450_e152555_d_n0;
        locals.var_t5_dn2 = assign100450_e152555_d_n2;
        locals.var_t5_dn4 = assign100450_e152555_d_n4;
        locals.var_t5_dn5 = assign100450_e152555_d_n5;
        locals.var_t5_dn6 = assign100450_e152555_d_n6;
        locals.var_t5_dn7 = assign100450_e152555_d_n7;
        locals.var_t5_dn8 = assign100450_e152555_d_n8;
        locals.var_t5_dn9 = assign100450_e152555_d_n9;
        locals.var_t5_dn10 = assign100450_e152555_d_n10;
        locals.var_t5_dn11 = assign100450_e152555_d_n11;
        locals.var_t5_dn14 = assign100450_e152555_d_n14;

        let (assign100460_e152565, assign100460_e152565_d_n0, assign100460_e152565_d_n2, assign100460_e152565_d_n4, assign100460_e152565_d_n5, assign100460_e152565_d_n6, assign100460_e152565_d_n7, assign100460_e152565_d_n8, assign100460_e152565_d_n9, assign100460_e152565_d_n10, assign100460_e152565_d_n11, assign100460_e152565_d_n14,) = {
    if ((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) {
        let assign100460_e152562: f64 = (locals.var_t4 * locals.var_t5);
        let assign100460_e152563: f64 = (1.0 + assign100460_e152562);
        (assign100460_e152563, ((locals.var_t4_dn0 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn0)), ((locals.var_t4_dn2 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn2)), ((locals.var_t4_dn4 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn4)), ((locals.var_t4_dn5 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn5)), ((locals.var_t4_dn6 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn6)), ((locals.var_t4_dn7 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn7)), ((locals.var_t4_dn8 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn8)), ((locals.var_t4_dn9 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn9)), ((locals.var_t4_dn10 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn10)), ((locals.var_t4_dn11 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn11)), ((locals.var_t4_dn14 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign100460_e152565;
        locals.var_t6_dn0 = assign100460_e152565_d_n0;
        locals.var_t6_dn2 = assign100460_e152565_d_n2;
        locals.var_t6_dn4 = assign100460_e152565_d_n4;
        locals.var_t6_dn5 = assign100460_e152565_d_n5;
        locals.var_t6_dn6 = assign100460_e152565_d_n6;
        locals.var_t6_dn7 = assign100460_e152565_d_n7;
        locals.var_t6_dn8 = assign100460_e152565_d_n8;
        locals.var_t6_dn9 = assign100460_e152565_d_n9;
        locals.var_t6_dn10 = assign100460_e152565_d_n10;
        locals.var_t6_dn11 = assign100460_e152565_d_n11;
        locals.var_t6_dn14 = assign100460_e152565_d_n14;

        let (assign100470_e152575, assign100470_e152575_d_n0, assign100470_e152575_d_n2, assign100470_e152575_d_n4, assign100470_e152575_d_n5, assign100470_e152575_d_n6, assign100470_e152575_d_n7, assign100470_e152575_d_n8, assign100470_e152575_d_n9, assign100470_e152575_d_n10, assign100470_e152575_d_n11, assign100470_e152575_d_n14,) = {
    if ((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) {
        let assign100470_e152572: f64 = (1.0 + locals.var_t4);
        let assign100470_e152573: f64 = (2.0 * assign100470_e152572);
        (assign100470_e152573, (2.0 * locals.var_t4_dn0), (2.0 * locals.var_t4_dn2), (2.0 * locals.var_t4_dn4), (2.0 * locals.var_t4_dn5), (2.0 * locals.var_t4_dn6), (2.0 * locals.var_t4_dn7), (2.0 * locals.var_t4_dn8), (2.0 * locals.var_t4_dn9), (2.0 * locals.var_t4_dn10), (2.0 * locals.var_t4_dn11), (2.0 * locals.var_t4_dn14),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign100470_e152575;
        locals.var_t7_dn0 = assign100470_e152575_d_n0;
        locals.var_t7_dn2 = assign100470_e152575_d_n2;
        locals.var_t7_dn4 = assign100470_e152575_d_n4;
        locals.var_t7_dn5 = assign100470_e152575_d_n5;
        locals.var_t7_dn6 = assign100470_e152575_d_n6;
        locals.var_t7_dn7 = assign100470_e152575_d_n7;
        locals.var_t7_dn8 = assign100470_e152575_d_n8;
        locals.var_t7_dn9 = assign100470_e152575_d_n9;
        locals.var_t7_dn10 = assign100470_e152575_d_n10;
        locals.var_t7_dn11 = assign100470_e152575_d_n11;
        locals.var_t7_dn14 = assign100470_e152575_d_n14;

        let assign100480_e152579: f64 = locals.var_t7;
        let assign100480_e152584: f64 = if ((locals.var_t6 < assign100480_e152579) && (locals.var_t7 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2315 = assign100480_e152584;

        let (assign100490_e152596, assign100490_e152596_d_n0, assign100490_e152596_d_n2, assign100490_e152596_d_n4, assign100490_e152596_d_n5, assign100490_e152596_d_n6, assign100490_e152596_d_n7, assign100490_e152596_d_n8, assign100490_e152596_d_n9, assign100490_e152596_d_n10, assign100490_e152596_d_n11, assign100490_e152596_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        let assign100490_e152592: f64 = locals.var_t7;
        let assign100490_e152594: f64 = (assign100490_e152592 - locals.var_t6);
        (assign100490_e152594, (locals.var_t7_dn0 - locals.var_t6_dn0), (locals.var_t7_dn2 - locals.var_t6_dn2), (locals.var_t7_dn4 - locals.var_t6_dn4), (locals.var_t7_dn5 - locals.var_t6_dn5), (locals.var_t7_dn6 - locals.var_t6_dn6), (locals.var_t7_dn7 - locals.var_t6_dn7), (locals.var_t7_dn8 - locals.var_t6_dn8), (locals.var_t7_dn9 - locals.var_t6_dn9), (locals.var_t7_dn10 - locals.var_t6_dn10), (locals.var_t7_dn11 - locals.var_t6_dn11), (locals.var_t7_dn14 - locals.var_t6_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign100490_e152596;
        locals.var_tmf1_dn0 = assign100490_e152596_d_n0;
        locals.var_tmf1_dn2 = assign100490_e152596_d_n2;
        locals.var_tmf1_dn4 = assign100490_e152596_d_n4;
        locals.var_tmf1_dn5 = assign100490_e152596_d_n5;
        locals.var_tmf1_dn6 = assign100490_e152596_d_n6;
        locals.var_tmf1_dn7 = assign100490_e152596_d_n7;
        locals.var_tmf1_dn8 = assign100490_e152596_d_n8;
        locals.var_tmf1_dn9 = assign100490_e152596_d_n9;
        locals.var_tmf1_dn10 = assign100490_e152596_d_n10;
        locals.var_tmf1_dn11 = assign100490_e152596_d_n11;
        locals.var_tmf1_dn14 = assign100490_e152596_d_n14;

        let (assign100500_e152606, assign100500_e152606_d_n0, assign100500_e152606_d_n2, assign100500_e152606_d_n4, assign100500_e152606_d_n5, assign100500_e152606_d_n6, assign100500_e152606_d_n7, assign100500_e152606_d_n8, assign100500_e152606_d_n9, assign100500_e152606_d_n10, assign100500_e152606_d_n11, assign100500_e152606_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        let assign100500_e152604: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign100500_e152604, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign100500_e152606;
        locals.var_x2_dn0 = assign100500_e152606_d_n0;
        locals.var_x2_dn2 = assign100500_e152606_d_n2;
        locals.var_x2_dn4 = assign100500_e152606_d_n4;
        locals.var_x2_dn5 = assign100500_e152606_d_n5;
        locals.var_x2_dn6 = assign100500_e152606_d_n6;
        locals.var_x2_dn7 = assign100500_e152606_d_n7;
        locals.var_x2_dn8 = assign100500_e152606_d_n8;
        locals.var_x2_dn9 = assign100500_e152606_d_n9;
        locals.var_x2_dn10 = assign100500_e152606_d_n10;
        locals.var_x2_dn11 = assign100500_e152606_d_n11;
        locals.var_x2_dn14 = assign100500_e152606_d_n14;

        let (assign100510_e152616, assign100510_e152616_d_n0, assign100510_e152616_d_n2, assign100510_e152616_d_n4, assign100510_e152616_d_n5, assign100510_e152616_d_n6, assign100510_e152616_d_n7, assign100510_e152616_d_n8, assign100510_e152616_d_n9, assign100510_e152616_d_n10, assign100510_e152616_d_n11, assign100510_e152616_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        let assign100510_e152614: f64 = (locals.var_t7 * locals.var_t7);
        (assign100510_e152614, ((locals.var_t7_dn0 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn0)), ((locals.var_t7_dn2 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn2)), ((locals.var_t7_dn4 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn4)), ((locals.var_t7_dn5 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn5)), ((locals.var_t7_dn6 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn6)), ((locals.var_t7_dn7 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn7)), ((locals.var_t7_dn8 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn8)), ((locals.var_t7_dn9 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn9)), ((locals.var_t7_dn10 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn10)), ((locals.var_t7_dn11 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn11)), ((locals.var_t7_dn14 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn14)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign100510_e152616;
        locals.var_xmax2_dn0 = assign100510_e152616_d_n0;
        locals.var_xmax2_dn2 = assign100510_e152616_d_n2;
        locals.var_xmax2_dn4 = assign100510_e152616_d_n4;
        locals.var_xmax2_dn5 = assign100510_e152616_d_n5;
        locals.var_xmax2_dn6 = assign100510_e152616_d_n6;
        locals.var_xmax2_dn7 = assign100510_e152616_d_n7;
        locals.var_xmax2_dn8 = assign100510_e152616_d_n8;
        locals.var_xmax2_dn9 = assign100510_e152616_d_n9;
        locals.var_xmax2_dn10 = assign100510_e152616_d_n10;
        locals.var_xmax2_dn11 = assign100510_e152616_d_n11;
        locals.var_xmax2_dn14 = assign100510_e152616_d_n14;

        let (assign100520_e152624, assign100520_e152624_d_n0, assign100520_e152624_d_n2, assign100520_e152624_d_n4, assign100520_e152624_d_n5, assign100520_e152624_d_n6, assign100520_e152624_d_n7, assign100520_e152624_d_n8, assign100520_e152624_d_n9, assign100520_e152624_d_n10, assign100520_e152624_d_n11, assign100520_e152624_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign100520_e152624;
        locals.var_xp_dn0 = assign100520_e152624_d_n0;
        locals.var_xp_dn2 = assign100520_e152624_d_n2;
        locals.var_xp_dn4 = assign100520_e152624_d_n4;
        locals.var_xp_dn5 = assign100520_e152624_d_n5;
        locals.var_xp_dn6 = assign100520_e152624_d_n6;
        locals.var_xp_dn7 = assign100520_e152624_d_n7;
        locals.var_xp_dn8 = assign100520_e152624_d_n8;
        locals.var_xp_dn9 = assign100520_e152624_d_n9;
        locals.var_xp_dn10 = assign100520_e152624_d_n10;
        locals.var_xp_dn11 = assign100520_e152624_d_n11;
        locals.var_xp_dn14 = assign100520_e152624_d_n14;

        let (assign100530_e152632, assign100530_e152632_d_n0, assign100530_e152632_d_n2, assign100530_e152632_d_n4, assign100530_e152632_d_n5, assign100530_e152632_d_n6, assign100530_e152632_d_n7, assign100530_e152632_d_n8, assign100530_e152632_d_n9, assign100530_e152632_d_n10, assign100530_e152632_d_n11, assign100530_e152632_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign100530_e152632;
        locals.var_xmp_dn0 = assign100530_e152632_d_n0;
        locals.var_xmp_dn2 = assign100530_e152632_d_n2;
        locals.var_xmp_dn4 = assign100530_e152632_d_n4;
        locals.var_xmp_dn5 = assign100530_e152632_d_n5;
        locals.var_xmp_dn6 = assign100530_e152632_d_n6;
        locals.var_xmp_dn7 = assign100530_e152632_d_n7;
        locals.var_xmp_dn8 = assign100530_e152632_d_n8;
        locals.var_xmp_dn9 = assign100530_e152632_d_n9;
        locals.var_xmp_dn10 = assign100530_e152632_d_n10;
        locals.var_xmp_dn11 = assign100530_e152632_d_n11;
        locals.var_xmp_dn14 = assign100530_e152632_d_n14;

        let (assign100540_e152640,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign100540_e152640;

        let (assign100550_e152648,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign100550_e152648;

        let (assign100560_e152656, assign100560_e152656_d_n0, assign100560_e152656_d_n2, assign100560_e152656_d_n4, assign100560_e152656_d_n5, assign100560_e152656_d_n6, assign100560_e152656_d_n7, assign100560_e152656_d_n8, assign100560_e152656_d_n9, assign100560_e152656_d_n10, assign100560_e152656_d_n11, assign100560_e152656_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign100560_e152656;
        locals.var_arg_dn0 = assign100560_e152656_d_n0;
        locals.var_arg_dn2 = assign100560_e152656_d_n2;
        locals.var_arg_dn4 = assign100560_e152656_d_n4;
        locals.var_arg_dn5 = assign100560_e152656_d_n5;
        locals.var_arg_dn6 = assign100560_e152656_d_n6;
        locals.var_arg_dn7 = assign100560_e152656_d_n7;
        locals.var_arg_dn8 = assign100560_e152656_d_n8;
        locals.var_arg_dn9 = assign100560_e152656_d_n9;
        locals.var_arg_dn10 = assign100560_e152656_d_n10;
        locals.var_arg_dn11 = assign100560_e152656_d_n11;
        locals.var_arg_dn14 = assign100560_e152656_d_n14;

        let (assign100570_e152664, assign100570_e152664_d_n0, assign100570_e152664_d_n2, assign100570_e152664_d_n4, assign100570_e152664_d_n5, assign100570_e152664_d_n6, assign100570_e152664_d_n7, assign100570_e152664_d_n8, assign100570_e152664_d_n9, assign100570_e152664_d_n10, assign100570_e152664_d_n11, assign100570_e152664_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign100570_e152664;
        locals.var_dnm_dn0 = assign100570_e152664_d_n0;
        locals.var_dnm_dn2 = assign100570_e152664_d_n2;
        locals.var_dnm_dn4 = assign100570_e152664_d_n4;
        locals.var_dnm_dn5 = assign100570_e152664_d_n5;
        locals.var_dnm_dn6 = assign100570_e152664_d_n6;
        locals.var_dnm_dn7 = assign100570_e152664_d_n7;
        locals.var_dnm_dn8 = assign100570_e152664_d_n8;
        locals.var_dnm_dn9 = assign100570_e152664_d_n9;
        locals.var_dnm_dn10 = assign100570_e152664_d_n10;
        locals.var_dnm_dn11 = assign100570_e152664_d_n11;
        locals.var_dnm_dn14 = assign100570_e152664_d_n14;

        let (assign100580_e152674, assign100580_e152674_d_n0, assign100580_e152674_d_n2, assign100580_e152674_d_n4, assign100580_e152674_d_n5, assign100580_e152674_d_n6, assign100580_e152674_d_n7, assign100580_e152674_d_n8, assign100580_e152674_d_n9, assign100580_e152674_d_n10, assign100580_e152674_d_n11, assign100580_e152674_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        let assign100580_e152672: f64 = (locals.var_xp * locals.var_x2);
        (assign100580_e152672, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign100580_e152674;
        locals.var_xp_dn0 = assign100580_e152674_d_n0;
        locals.var_xp_dn2 = assign100580_e152674_d_n2;
        locals.var_xp_dn4 = assign100580_e152674_d_n4;
        locals.var_xp_dn5 = assign100580_e152674_d_n5;
        locals.var_xp_dn6 = assign100580_e152674_d_n6;
        locals.var_xp_dn7 = assign100580_e152674_d_n7;
        locals.var_xp_dn8 = assign100580_e152674_d_n8;
        locals.var_xp_dn9 = assign100580_e152674_d_n9;
        locals.var_xp_dn10 = assign100580_e152674_d_n10;
        locals.var_xp_dn11 = assign100580_e152674_d_n11;
        locals.var_xp_dn14 = assign100580_e152674_d_n14;

        let (assign100590_e152684, assign100590_e152684_d_n0, assign100590_e152684_d_n2, assign100590_e152684_d_n4, assign100590_e152684_d_n5, assign100590_e152684_d_n6, assign100590_e152684_d_n7, assign100590_e152684_d_n8, assign100590_e152684_d_n9, assign100590_e152684_d_n10, assign100590_e152684_d_n11, assign100590_e152684_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        let assign100590_e152682: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign100590_e152682, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign100590_e152684;
        locals.var_xmp_dn0 = assign100590_e152684_d_n0;
        locals.var_xmp_dn2 = assign100590_e152684_d_n2;
        locals.var_xmp_dn4 = assign100590_e152684_d_n4;
        locals.var_xmp_dn5 = assign100590_e152684_d_n5;
        locals.var_xmp_dn6 = assign100590_e152684_d_n6;
        locals.var_xmp_dn7 = assign100590_e152684_d_n7;
        locals.var_xmp_dn8 = assign100590_e152684_d_n8;
        locals.var_xmp_dn9 = assign100590_e152684_d_n9;
        locals.var_xmp_dn10 = assign100590_e152684_d_n10;
        locals.var_xmp_dn11 = assign100590_e152684_d_n11;
        locals.var_xmp_dn14 = assign100590_e152684_d_n14;

        let (assign100600_e152694, assign100600_e152694_d_n0, assign100600_e152694_d_n2, assign100600_e152694_d_n4, assign100600_e152694_d_n5, assign100600_e152694_d_n6, assign100600_e152694_d_n7, assign100600_e152694_d_n8, assign100600_e152694_d_n9, assign100600_e152694_d_n10, assign100600_e152694_d_n11, assign100600_e152694_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        let assign100600_e152692: f64 = (locals.var_xp * locals.var_x2);
        (assign100600_e152692, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign100600_e152694;
        locals.var_xp_dn0 = assign100600_e152694_d_n0;
        locals.var_xp_dn2 = assign100600_e152694_d_n2;
        locals.var_xp_dn4 = assign100600_e152694_d_n4;
        locals.var_xp_dn5 = assign100600_e152694_d_n5;
        locals.var_xp_dn6 = assign100600_e152694_d_n6;
        locals.var_xp_dn7 = assign100600_e152694_d_n7;
        locals.var_xp_dn8 = assign100600_e152694_d_n8;
        locals.var_xp_dn9 = assign100600_e152694_d_n9;
        locals.var_xp_dn10 = assign100600_e152694_d_n10;
        locals.var_xp_dn11 = assign100600_e152694_d_n11;
        locals.var_xp_dn14 = assign100600_e152694_d_n14;

        let (assign100610_e152704, assign100610_e152704_d_n0, assign100610_e152704_d_n2, assign100610_e152704_d_n4, assign100610_e152704_d_n5, assign100610_e152704_d_n6, assign100610_e152704_d_n7, assign100610_e152704_d_n8, assign100610_e152704_d_n9, assign100610_e152704_d_n10, assign100610_e152704_d_n11, assign100610_e152704_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        let assign100610_e152702: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign100610_e152702, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign100610_e152704;
        locals.var_xmp_dn0 = assign100610_e152704_d_n0;
        locals.var_xmp_dn2 = assign100610_e152704_d_n2;
        locals.var_xmp_dn4 = assign100610_e152704_d_n4;
        locals.var_xmp_dn5 = assign100610_e152704_d_n5;
        locals.var_xmp_dn6 = assign100610_e152704_d_n6;
        locals.var_xmp_dn7 = assign100610_e152704_d_n7;
        locals.var_xmp_dn8 = assign100610_e152704_d_n8;
        locals.var_xmp_dn9 = assign100610_e152704_d_n9;
        locals.var_xmp_dn10 = assign100610_e152704_d_n10;
        locals.var_xmp_dn11 = assign100610_e152704_d_n11;
        locals.var_xmp_dn14 = assign100610_e152704_d_n14;

        let (assign100620_e152714, assign100620_e152714_d_n0, assign100620_e152714_d_n2, assign100620_e152714_d_n4, assign100620_e152714_d_n5, assign100620_e152714_d_n6, assign100620_e152714_d_n7, assign100620_e152714_d_n8, assign100620_e152714_d_n9, assign100620_e152714_d_n10, assign100620_e152714_d_n11, assign100620_e152714_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        let assign100620_e152712: f64 = (locals.var_xp * locals.var_x2);
        (assign100620_e152712, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign100620_e152714;
        locals.var_xp_dn0 = assign100620_e152714_d_n0;
        locals.var_xp_dn2 = assign100620_e152714_d_n2;
        locals.var_xp_dn4 = assign100620_e152714_d_n4;
        locals.var_xp_dn5 = assign100620_e152714_d_n5;
        locals.var_xp_dn6 = assign100620_e152714_d_n6;
        locals.var_xp_dn7 = assign100620_e152714_d_n7;
        locals.var_xp_dn8 = assign100620_e152714_d_n8;
        locals.var_xp_dn9 = assign100620_e152714_d_n9;
        locals.var_xp_dn10 = assign100620_e152714_d_n10;
        locals.var_xp_dn11 = assign100620_e152714_d_n11;
        locals.var_xp_dn14 = assign100620_e152714_d_n14;

        let (assign100630_e152724, assign100630_e152724_d_n0, assign100630_e152724_d_n2, assign100630_e152724_d_n4, assign100630_e152724_d_n5, assign100630_e152724_d_n6, assign100630_e152724_d_n7, assign100630_e152724_d_n8, assign100630_e152724_d_n9, assign100630_e152724_d_n10, assign100630_e152724_d_n11, assign100630_e152724_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        let assign100630_e152722: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign100630_e152722, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign100630_e152724;
        locals.var_xmp_dn0 = assign100630_e152724_d_n0;
        locals.var_xmp_dn2 = assign100630_e152724_d_n2;
        locals.var_xmp_dn4 = assign100630_e152724_d_n4;
        locals.var_xmp_dn5 = assign100630_e152724_d_n5;
        locals.var_xmp_dn6 = assign100630_e152724_d_n6;
        locals.var_xmp_dn7 = assign100630_e152724_d_n7;
        locals.var_xmp_dn8 = assign100630_e152724_d_n8;
        locals.var_xmp_dn9 = assign100630_e152724_d_n9;
        locals.var_xmp_dn10 = assign100630_e152724_d_n10;
        locals.var_xmp_dn11 = assign100630_e152724_d_n11;
        locals.var_xmp_dn14 = assign100630_e152724_d_n14;

        let (assign100640_e152734, assign100640_e152734_d_n0, assign100640_e152734_d_n2, assign100640_e152734_d_n4, assign100640_e152734_d_n5, assign100640_e152734_d_n6, assign100640_e152734_d_n7, assign100640_e152734_d_n8, assign100640_e152734_d_n9, assign100640_e152734_d_n10, assign100640_e152734_d_n11, assign100640_e152734_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        let assign100640_e152732: f64 = (locals.var_xp * locals.var_x2);
        (assign100640_e152732, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign100640_e152734;
        locals.var_xp_dn0 = assign100640_e152734_d_n0;
        locals.var_xp_dn2 = assign100640_e152734_d_n2;
        locals.var_xp_dn4 = assign100640_e152734_d_n4;
        locals.var_xp_dn5 = assign100640_e152734_d_n5;
        locals.var_xp_dn6 = assign100640_e152734_d_n6;
        locals.var_xp_dn7 = assign100640_e152734_d_n7;
        locals.var_xp_dn8 = assign100640_e152734_d_n8;
        locals.var_xp_dn9 = assign100640_e152734_d_n9;
        locals.var_xp_dn10 = assign100640_e152734_d_n10;
        locals.var_xp_dn11 = assign100640_e152734_d_n11;
        locals.var_xp_dn14 = assign100640_e152734_d_n14;

        let (assign100650_e152744, assign100650_e152744_d_n0, assign100650_e152744_d_n2, assign100650_e152744_d_n4, assign100650_e152744_d_n5, assign100650_e152744_d_n6, assign100650_e152744_d_n7, assign100650_e152744_d_n8, assign100650_e152744_d_n9, assign100650_e152744_d_n10, assign100650_e152744_d_n11, assign100650_e152744_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        let assign100650_e152742: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign100650_e152742, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign100650_e152744;
        locals.var_xmp_dn0 = assign100650_e152744_d_n0;
        locals.var_xmp_dn2 = assign100650_e152744_d_n2;
        locals.var_xmp_dn4 = assign100650_e152744_d_n4;
        locals.var_xmp_dn5 = assign100650_e152744_d_n5;
        locals.var_xmp_dn6 = assign100650_e152744_d_n6;
        locals.var_xmp_dn7 = assign100650_e152744_d_n7;
        locals.var_xmp_dn8 = assign100650_e152744_d_n8;
        locals.var_xmp_dn9 = assign100650_e152744_d_n9;
        locals.var_xmp_dn10 = assign100650_e152744_d_n10;
        locals.var_xmp_dn11 = assign100650_e152744_d_n11;
        locals.var_xmp_dn14 = assign100650_e152744_d_n14;

        let (assign100660_e152754, assign100660_e152754_d_n0, assign100660_e152754_d_n2, assign100660_e152754_d_n4, assign100660_e152754_d_n5, assign100660_e152754_d_n6, assign100660_e152754_d_n7, assign100660_e152754_d_n8, assign100660_e152754_d_n9, assign100660_e152754_d_n10, assign100660_e152754_d_n11, assign100660_e152754_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        let assign100660_e152752: f64 = (locals.var_xp + locals.var_xmp);
        (assign100660_e152752, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign100660_e152754;
        locals.var_arg_dn0 = assign100660_e152754_d_n0;
        locals.var_arg_dn2 = assign100660_e152754_d_n2;
        locals.var_arg_dn4 = assign100660_e152754_d_n4;
        locals.var_arg_dn5 = assign100660_e152754_d_n5;
        locals.var_arg_dn6 = assign100660_e152754_d_n6;
        locals.var_arg_dn7 = assign100660_e152754_d_n7;
        locals.var_arg_dn8 = assign100660_e152754_d_n8;
        locals.var_arg_dn9 = assign100660_e152754_d_n9;
        locals.var_arg_dn10 = assign100660_e152754_d_n10;
        locals.var_arg_dn11 = assign100660_e152754_d_n11;
        locals.var_arg_dn14 = assign100660_e152754_d_n14;

        let (assign100670_e152762, assign100670_e152762_d_n0, assign100670_e152762_d_n2, assign100670_e152762_d_n4, assign100670_e152762_d_n5, assign100670_e152762_d_n6, assign100670_e152762_d_n7, assign100670_e152762_d_n8, assign100670_e152762_d_n9, assign100670_e152762_d_n10, assign100670_e152762_d_n11, assign100670_e152762_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign100670_e152762;
        locals.var_dnm_dn0 = assign100670_e152762_d_n0;
        locals.var_dnm_dn2 = assign100670_e152762_d_n2;
        locals.var_dnm_dn4 = assign100670_e152762_d_n4;
        locals.var_dnm_dn5 = assign100670_e152762_d_n5;
        locals.var_dnm_dn6 = assign100670_e152762_d_n6;
        locals.var_dnm_dn7 = assign100670_e152762_d_n7;
        locals.var_dnm_dn8 = assign100670_e152762_d_n8;
        locals.var_dnm_dn9 = assign100670_e152762_d_n9;
        locals.var_dnm_dn10 = assign100670_e152762_d_n10;
        locals.var_dnm_dn11 = assign100670_e152762_d_n11;
        locals.var_dnm_dn14 = assign100670_e152762_d_n14;

        let assign100680_e152777: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2316 = assign100680_e152777;

        let assign100690_e152780: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2317 = assign100690_e152780;

        let (assign100700_e152792,) = {
    if (((((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign100700_e152792;

        let assign100710_e152795: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2318 = assign100710_e152795;

    }

    pub(super) fn stamp_transient_block_369(
        locals: &mut StampLocals,
    ) {
        let (assign100720_e152810,) = {
    if ((((((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 == 0.0)) && (locals.var_guard2318 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign100720_e152810;

        let assign100730_e152813: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2319 = assign100730_e152813;

        let (assign100740_e152831,) = {
    if (((((((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 == 0.0)) && (locals.var_guard2318 == 0.0)) && (locals.var_guard2319 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign100740_e152831;

        let assign100750_e152834: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2320 = assign100750_e152834;

        let (assign100760_e152855,) = {
    if ((((((((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 == 0.0)) && (locals.var_guard2318 == 0.0)) && (locals.var_guard2319 == 0.0)) && (locals.var_guard2320 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign100760_e152855;

        let (assign100770_e152865,) = {
    if ((((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) && (locals.var_guard2316 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign100770_e152865;

        let mut assign100780_loop_guard: usize = 0;
        while {
            let assign100780_cond_e152876: f64 = if (((((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) && (locals.var_guard2316 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign100780_cond_e152876 != 0.0
        } {
            assign100780_loop_guard += 1;
            assert!(assign100780_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign100780_body0_e152887, assign100780_body0_e152887_d_n0, assign100780_body0_e152887_d_n2, assign100780_body0_e152887_d_n4, assign100780_body0_e152887_d_n5, assign100780_body0_e152887_d_n6, assign100780_body0_e152887_d_n7, assign100780_body0_e152887_d_n8, assign100780_body0_e152887_d_n9, assign100780_body0_e152887_d_n10, assign100780_body0_e152887_d_n11, assign100780_body0_e152887_d_n14,) = {
    if ((((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) && (locals.var_guard2316 != 0.0)) {
        let assign100780_body0_e152885: f64 = (locals.var_dnm).sqrt();
        (assign100780_body0_e152885, (locals.var_dnm_dn0 / (2.0 * assign100780_body0_e152885)), (locals.var_dnm_dn2 / (2.0 * assign100780_body0_e152885)), (locals.var_dnm_dn4 / (2.0 * assign100780_body0_e152885)), (locals.var_dnm_dn5 / (2.0 * assign100780_body0_e152885)), (locals.var_dnm_dn6 / (2.0 * assign100780_body0_e152885)), (locals.var_dnm_dn7 / (2.0 * assign100780_body0_e152885)), (locals.var_dnm_dn8 / (2.0 * assign100780_body0_e152885)), (locals.var_dnm_dn9 / (2.0 * assign100780_body0_e152885)), (locals.var_dnm_dn10 / (2.0 * assign100780_body0_e152885)), (locals.var_dnm_dn11 / (2.0 * assign100780_body0_e152885)), (locals.var_dnm_dn14 / (2.0 * assign100780_body0_e152885)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign100780_body0_e152887;
            locals.var_dnm_dn0 = assign100780_body0_e152887_d_n0;
            locals.var_dnm_dn2 = assign100780_body0_e152887_d_n2;
            locals.var_dnm_dn4 = assign100780_body0_e152887_d_n4;
            locals.var_dnm_dn5 = assign100780_body0_e152887_d_n5;
            locals.var_dnm_dn6 = assign100780_body0_e152887_d_n6;
            locals.var_dnm_dn7 = assign100780_body0_e152887_d_n7;
            locals.var_dnm_dn8 = assign100780_body0_e152887_d_n8;
            locals.var_dnm_dn9 = assign100780_body0_e152887_d_n9;
            locals.var_dnm_dn10 = assign100780_body0_e152887_d_n10;
            locals.var_dnm_dn11 = assign100780_body0_e152887_d_n11;
            locals.var_dnm_dn14 = assign100780_body0_e152887_d_n14;
            let (assign100780_body1_e152899,) = {
    if ((((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) && (locals.var_guard2316 != 0.0)) {
        let assign100780_body1_e152897: f64 = (locals.var_m0 + 1.0);
        (assign100780_body1_e152897,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign100780_body1_e152899;
        }

        let (assign100790_e152921, assign100790_e152921_d_n0, assign100790_e152921_d_n2, assign100790_e152921_d_n4, assign100790_e152921_d_n5, assign100790_e152921_d_n6, assign100790_e152921_d_n7, assign100790_e152921_d_n8, assign100790_e152921_d_n9, assign100790_e152921_d_n10, assign100790_e152921_d_n11, assign100790_e152921_d_n14,) = {
    if ((((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) && (locals.var_guard2316 == 0.0)) {
        let (assign100790_e152919, assign100790_e152919_d_n0, assign100790_e152919_d_n2, assign100790_e152919_d_n4, assign100790_e152919_d_n5, assign100790_e152919_d_n6, assign100790_e152919_d_n7, assign100790_e152919_d_n8, assign100790_e152919_d_n9, assign100790_e152919_d_n10, assign100790_e152919_d_n11, assign100790_e152919_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign100790_e152916: f64 = (2.0 * 4.0);
                let assign100790_e152917: f64 = (1.0 / assign100790_e152916);
                let assign100790_e152918: f64 = (locals.var_dnm).powf(assign100790_e152917);
                (assign100790_e152918, if 0.0 == 0.0 && ((assign100790_e152917) as f64).is_finite() && ((assign100790_e152917) as f64).fract() == 0.0 { if assign100790_e152917 == 0.0 { 0.0 } else { (assign100790_e152917 * ((locals.var_dnm).powf(assign100790_e152917 - 1.0) * locals.var_dnm_dn0)) } } else { (assign100790_e152918 * (assign100790_e152917 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100790_e152917) as f64).is_finite() && ((assign100790_e152917) as f64).fract() == 0.0 { if assign100790_e152917 == 0.0 { 0.0 } else { (assign100790_e152917 * ((locals.var_dnm).powf(assign100790_e152917 - 1.0) * locals.var_dnm_dn2)) } } else { (assign100790_e152918 * (assign100790_e152917 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100790_e152917) as f64).is_finite() && ((assign100790_e152917) as f64).fract() == 0.0 { if assign100790_e152917 == 0.0 { 0.0 } else { (assign100790_e152917 * ((locals.var_dnm).powf(assign100790_e152917 - 1.0) * locals.var_dnm_dn4)) } } else { (assign100790_e152918 * (assign100790_e152917 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100790_e152917) as f64).is_finite() && ((assign100790_e152917) as f64).fract() == 0.0 { if assign100790_e152917 == 0.0 { 0.0 } else { (assign100790_e152917 * ((locals.var_dnm).powf(assign100790_e152917 - 1.0) * locals.var_dnm_dn5)) } } else { (assign100790_e152918 * (assign100790_e152917 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100790_e152917) as f64).is_finite() && ((assign100790_e152917) as f64).fract() == 0.0 { if assign100790_e152917 == 0.0 { 0.0 } else { (assign100790_e152917 * ((locals.var_dnm).powf(assign100790_e152917 - 1.0) * locals.var_dnm_dn6)) } } else { (assign100790_e152918 * (assign100790_e152917 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100790_e152917) as f64).is_finite() && ((assign100790_e152917) as f64).fract() == 0.0 { if assign100790_e152917 == 0.0 { 0.0 } else { (assign100790_e152917 * ((locals.var_dnm).powf(assign100790_e152917 - 1.0) * locals.var_dnm_dn7)) } } else { (assign100790_e152918 * (assign100790_e152917 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100790_e152917) as f64).is_finite() && ((assign100790_e152917) as f64).fract() == 0.0 { if assign100790_e152917 == 0.0 { 0.0 } else { (assign100790_e152917 * ((locals.var_dnm).powf(assign100790_e152917 - 1.0) * locals.var_dnm_dn8)) } } else { (assign100790_e152918 * (assign100790_e152917 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100790_e152917) as f64).is_finite() && ((assign100790_e152917) as f64).fract() == 0.0 { if assign100790_e152917 == 0.0 { 0.0 } else { (assign100790_e152917 * ((locals.var_dnm).powf(assign100790_e152917 - 1.0) * locals.var_dnm_dn9)) } } else { (assign100790_e152918 * (assign100790_e152917 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100790_e152917) as f64).is_finite() && ((assign100790_e152917) as f64).fract() == 0.0 { if assign100790_e152917 == 0.0 { 0.0 } else { (assign100790_e152917 * ((locals.var_dnm).powf(assign100790_e152917 - 1.0) * locals.var_dnm_dn10)) } } else { (assign100790_e152918 * (assign100790_e152917 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100790_e152917) as f64).is_finite() && ((assign100790_e152917) as f64).fract() == 0.0 { if assign100790_e152917 == 0.0 { 0.0 } else { (assign100790_e152917 * ((locals.var_dnm).powf(assign100790_e152917 - 1.0) * locals.var_dnm_dn11)) } } else { (assign100790_e152918 * (assign100790_e152917 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100790_e152917) as f64).is_finite() && ((assign100790_e152917) as f64).fract() == 0.0 { if assign100790_e152917 == 0.0 { 0.0 } else { (assign100790_e152917 * ((locals.var_dnm).powf(assign100790_e152917 - 1.0) * locals.var_dnm_dn14)) } } else { (assign100790_e152918 * (assign100790_e152917 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign100790_e152919, assign100790_e152919_d_n0, assign100790_e152919_d_n2, assign100790_e152919_d_n4, assign100790_e152919_d_n5, assign100790_e152919_d_n6, assign100790_e152919_d_n7, assign100790_e152919_d_n8, assign100790_e152919_d_n9, assign100790_e152919_d_n10, assign100790_e152919_d_n11, assign100790_e152919_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign100790_e152921;
        locals.var_dnm_dn0 = assign100790_e152921_d_n0;
        locals.var_dnm_dn2 = assign100790_e152921_d_n2;
        locals.var_dnm_dn4 = assign100790_e152921_d_n4;
        locals.var_dnm_dn5 = assign100790_e152921_d_n5;
        locals.var_dnm_dn6 = assign100790_e152921_d_n6;
        locals.var_dnm_dn7 = assign100790_e152921_d_n7;
        locals.var_dnm_dn8 = assign100790_e152921_d_n8;
        locals.var_dnm_dn9 = assign100790_e152921_d_n9;
        locals.var_dnm_dn10 = assign100790_e152921_d_n10;
        locals.var_dnm_dn11 = assign100790_e152921_d_n11;
        locals.var_dnm_dn14 = assign100790_e152921_d_n14;

        let (assign100800_e152931, assign100800_e152931_d_n0, assign100800_e152931_d_n2, assign100800_e152931_d_n4, assign100800_e152931_d_n5, assign100800_e152931_d_n6, assign100800_e152931_d_n7, assign100800_e152931_d_n8, assign100800_e152931_d_n9, assign100800_e152931_d_n10, assign100800_e152931_d_n11, assign100800_e152931_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        let assign100800_e152929: f64 = (1.0 / locals.var_dnm);
        (assign100800_e152929, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign100800_e152931;
        locals.var_dnm_dn0 = assign100800_e152931_d_n0;
        locals.var_dnm_dn2 = assign100800_e152931_d_n2;
        locals.var_dnm_dn4 = assign100800_e152931_d_n4;
        locals.var_dnm_dn5 = assign100800_e152931_d_n5;
        locals.var_dnm_dn6 = assign100800_e152931_d_n6;
        locals.var_dnm_dn7 = assign100800_e152931_d_n7;
        locals.var_dnm_dn8 = assign100800_e152931_d_n8;
        locals.var_dnm_dn9 = assign100800_e152931_d_n9;
        locals.var_dnm_dn10 = assign100800_e152931_d_n10;
        locals.var_dnm_dn11 = assign100800_e152931_d_n11;
        locals.var_dnm_dn14 = assign100800_e152931_d_n14;

        let (assign100810_e152943, assign100810_e152943_d_n0, assign100810_e152943_d_n2, assign100810_e152943_d_n4, assign100810_e152943_d_n5, assign100810_e152943_d_n6, assign100810_e152943_d_n7, assign100810_e152943_d_n8, assign100810_e152943_d_n9, assign100810_e152943_d_n10, assign100810_e152943_d_n11, assign100810_e152943_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        let assign100810_e152939: f64 = (locals.var_tmf1 * locals.var_t7);
        let assign100810_e152941: f64 = (assign100810_e152939 * locals.var_dnm);
        (assign100810_e152941, ((((locals.var_tmf1_dn0 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn0)) * locals.var_dnm) + (assign100810_e152939 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn2)) * locals.var_dnm) + (assign100810_e152939 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn4)) * locals.var_dnm) + (assign100810_e152939 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn5)) * locals.var_dnm) + (assign100810_e152939 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn6)) * locals.var_dnm) + (assign100810_e152939 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn7)) * locals.var_dnm) + (assign100810_e152939 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn8)) * locals.var_dnm) + (assign100810_e152939 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn9)) * locals.var_dnm) + (assign100810_e152939 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn10)) * locals.var_dnm) + (assign100810_e152939 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn11)) * locals.var_dnm) + (assign100810_e152939 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn14)) * locals.var_dnm) + (assign100810_e152939 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign100810_e152943;
        locals.var_tmf0_dn0 = assign100810_e152943_d_n0;
        locals.var_tmf0_dn2 = assign100810_e152943_d_n2;
        locals.var_tmf0_dn4 = assign100810_e152943_d_n4;
        locals.var_tmf0_dn5 = assign100810_e152943_d_n5;
        locals.var_tmf0_dn6 = assign100810_e152943_d_n6;
        locals.var_tmf0_dn7 = assign100810_e152943_d_n7;
        locals.var_tmf0_dn8 = assign100810_e152943_d_n8;
        locals.var_tmf0_dn9 = assign100810_e152943_d_n9;
        locals.var_tmf0_dn10 = assign100810_e152943_d_n10;
        locals.var_tmf0_dn11 = assign100810_e152943_d_n11;
        locals.var_tmf0_dn14 = assign100810_e152943_d_n14;

        let (assign100820_e152957, assign100820_e152957_d_n0, assign100820_e152957_d_n2, assign100820_e152957_d_n4, assign100820_e152957_d_n5, assign100820_e152957_d_n6, assign100820_e152957_d_n7, assign100820_e152957_d_n8, assign100820_e152957_d_n9, assign100820_e152957_d_n10, assign100820_e152957_d_n11, assign100820_e152957_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        let assign100820_e152951: f64 = (locals.var_t7 * locals.var_xmp);
        let assign100820_e152953: f64 = (assign100820_e152951 * locals.var_dnm);
        let assign100820_e152955: f64 = (assign100820_e152953 / locals.var_arg);
        (assign100820_e152955, (((((((locals.var_t7_dn0 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign100820_e152951 * locals.var_dnm_dn0)) * locals.var_arg) - (assign100820_e152953 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn2 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign100820_e152951 * locals.var_dnm_dn2)) * locals.var_arg) - (assign100820_e152953 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn4 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign100820_e152951 * locals.var_dnm_dn4)) * locals.var_arg) - (assign100820_e152953 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn5 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign100820_e152951 * locals.var_dnm_dn5)) * locals.var_arg) - (assign100820_e152953 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn6 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign100820_e152951 * locals.var_dnm_dn6)) * locals.var_arg) - (assign100820_e152953 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn7 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign100820_e152951 * locals.var_dnm_dn7)) * locals.var_arg) - (assign100820_e152953 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn8 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign100820_e152951 * locals.var_dnm_dn8)) * locals.var_arg) - (assign100820_e152953 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn9 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign100820_e152951 * locals.var_dnm_dn9)) * locals.var_arg) - (assign100820_e152953 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn10 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign100820_e152951 * locals.var_dnm_dn10)) * locals.var_arg) - (assign100820_e152953 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn11 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign100820_e152951 * locals.var_dnm_dn11)) * locals.var_arg) - (assign100820_e152953 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn14 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign100820_e152951 * locals.var_dnm_dn14)) * locals.var_arg) - (assign100820_e152953 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign100820_e152957;
        locals.var_t0_dn0 = assign100820_e152957_d_n0;
        locals.var_t0_dn2 = assign100820_e152957_d_n2;
        locals.var_t0_dn4 = assign100820_e152957_d_n4;
        locals.var_t0_dn5 = assign100820_e152957_d_n5;
        locals.var_t0_dn6 = assign100820_e152957_d_n6;
        locals.var_t0_dn7 = assign100820_e152957_d_n7;
        locals.var_t0_dn8 = assign100820_e152957_d_n8;
        locals.var_t0_dn9 = assign100820_e152957_d_n9;
        locals.var_t0_dn10 = assign100820_e152957_d_n10;
        locals.var_t0_dn11 = assign100820_e152957_d_n11;
        locals.var_t0_dn14 = assign100820_e152957_d_n14;

        let (assign100830_e152969, assign100830_e152969_d_n0, assign100830_e152969_d_n2, assign100830_e152969_d_n4, assign100830_e152969_d_n5, assign100830_e152969_d_n6, assign100830_e152969_d_n7, assign100830_e152969_d_n8, assign100830_e152969_d_n9, assign100830_e152969_d_n10, assign100830_e152969_d_n11, assign100830_e152969_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        let assign100830_e152965: f64 = locals.var_t7;
        let assign100830_e152967: f64 = (assign100830_e152965 - locals.var_tmf0);
        (assign100830_e152967, (locals.var_t7_dn0 - locals.var_tmf0_dn0), (locals.var_t7_dn2 - locals.var_tmf0_dn2), (locals.var_t7_dn4 - locals.var_tmf0_dn4), (locals.var_t7_dn5 - locals.var_tmf0_dn5), (locals.var_t7_dn6 - locals.var_tmf0_dn6), (locals.var_t7_dn7 - locals.var_tmf0_dn7), (locals.var_t7_dn8 - locals.var_tmf0_dn8), (locals.var_t7_dn9 - locals.var_tmf0_dn9), (locals.var_t7_dn10 - locals.var_tmf0_dn10), (locals.var_t7_dn11 - locals.var_tmf0_dn11), (locals.var_t7_dn14 - locals.var_tmf0_dn14),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign100830_e152969;
        locals.var_t6_dn0 = assign100830_e152969_d_n0;
        locals.var_t6_dn2 = assign100830_e152969_d_n2;
        locals.var_t6_dn4 = assign100830_e152969_d_n4;
        locals.var_t6_dn5 = assign100830_e152969_d_n5;
        locals.var_t6_dn6 = assign100830_e152969_d_n6;
        locals.var_t6_dn7 = assign100830_e152969_d_n7;
        locals.var_t6_dn8 = assign100830_e152969_d_n8;
        locals.var_t6_dn9 = assign100830_e152969_d_n9;
        locals.var_t6_dn10 = assign100830_e152969_d_n10;
        locals.var_t6_dn11 = assign100830_e152969_d_n11;
        locals.var_t6_dn14 = assign100830_e152969_d_n14;

        let (assign100840_e152977, assign100840_e152977_d_n0, assign100840_e152977_d_n2, assign100840_e152977_d_n4, assign100840_e152977_d_n5, assign100840_e152977_d_n6, assign100840_e152977_d_n7, assign100840_e152977_d_n8, assign100840_e152977_d_n9, assign100840_e152977_d_n10, assign100840_e152977_d_n11, assign100840_e152977_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign100840_e152977;
        locals.var_t0_dn0 = assign100840_e152977_d_n0;
        locals.var_t0_dn2 = assign100840_e152977_d_n2;
        locals.var_t0_dn4 = assign100840_e152977_d_n4;
        locals.var_t0_dn5 = assign100840_e152977_d_n5;
        locals.var_t0_dn6 = assign100840_e152977_d_n6;
        locals.var_t0_dn7 = assign100840_e152977_d_n7;
        locals.var_t0_dn8 = assign100840_e152977_d_n8;
        locals.var_t0_dn9 = assign100840_e152977_d_n9;
        locals.var_t0_dn10 = assign100840_e152977_d_n10;
        locals.var_t0_dn11 = assign100840_e152977_d_n11;
        locals.var_t0_dn14 = assign100840_e152977_d_n14;

        let (assign100850_e152986, assign100850_e152986_d_n0, assign100850_e152986_d_n2, assign100850_e152986_d_n4, assign100850_e152986_d_n5, assign100850_e152986_d_n6, assign100850_e152986_d_n7, assign100850_e152986_d_n8, assign100850_e152986_d_n9, assign100850_e152986_d_n10, assign100850_e152986_d_n11, assign100850_e152986_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 == 0.0)) {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign100850_e152986;
        locals.var_t6_dn0 = assign100850_e152986_d_n0;
        locals.var_t6_dn2 = assign100850_e152986_d_n2;
        locals.var_t6_dn4 = assign100850_e152986_d_n4;
        locals.var_t6_dn5 = assign100850_e152986_d_n5;
        locals.var_t6_dn6 = assign100850_e152986_d_n6;
        locals.var_t6_dn7 = assign100850_e152986_d_n7;
        locals.var_t6_dn8 = assign100850_e152986_d_n8;
        locals.var_t6_dn9 = assign100850_e152986_d_n9;
        locals.var_t6_dn10 = assign100850_e152986_d_n10;
        locals.var_t6_dn11 = assign100850_e152986_d_n11;
        locals.var_t6_dn14 = assign100850_e152986_d_n14;

        let (assign100860_e152995, assign100860_e152995_d_n0, assign100860_e152995_d_n2, assign100860_e152995_d_n4, assign100860_e152995_d_n5, assign100860_e152995_d_n6, assign100860_e152995_d_n7, assign100860_e152995_d_n8, assign100860_e152995_d_n9, assign100860_e152995_d_n10, assign100860_e152995_d_n11, assign100860_e152995_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign100860_e152995;
        locals.var_t0_dn0 = assign100860_e152995_d_n0;
        locals.var_t0_dn2 = assign100860_e152995_d_n2;
        locals.var_t0_dn4 = assign100860_e152995_d_n4;
        locals.var_t0_dn5 = assign100860_e152995_d_n5;
        locals.var_t0_dn6 = assign100860_e152995_d_n6;
        locals.var_t0_dn7 = assign100860_e152995_d_n7;
        locals.var_t0_dn8 = assign100860_e152995_d_n8;
        locals.var_t0_dn9 = assign100860_e152995_d_n9;
        locals.var_t0_dn10 = assign100860_e152995_d_n10;
        locals.var_t0_dn11 = assign100860_e152995_d_n11;
        locals.var_t0_dn14 = assign100860_e152995_d_n14;

        let (assign100870_e153002, assign100870_e153002_d_n0, assign100870_e153002_d_n2, assign100870_e153002_d_n4, assign100870_e153002_d_n5, assign100870_e153002_d_n6, assign100870_e153002_d_n7, assign100870_e153002_d_n8, assign100870_e153002_d_n9, assign100870_e153002_d_n10, assign100870_e153002_d_n11, assign100870_e153002_d_n14,) = {
    if ((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) {
        let assign100870_e153000: f64 = (locals.var_t6).sqrt();
        (assign100870_e153000, (locals.var_t6_dn0 / (2.0 * assign100870_e153000)), (locals.var_t6_dn2 / (2.0 * assign100870_e153000)), (locals.var_t6_dn4 / (2.0 * assign100870_e153000)), (locals.var_t6_dn5 / (2.0 * assign100870_e153000)), (locals.var_t6_dn6 / (2.0 * assign100870_e153000)), (locals.var_t6_dn7 / (2.0 * assign100870_e153000)), (locals.var_t6_dn8 / (2.0 * assign100870_e153000)), (locals.var_t6_dn9 / (2.0 * assign100870_e153000)), (locals.var_t6_dn10 / (2.0 * assign100870_e153000)), (locals.var_t6_dn11 / (2.0 * assign100870_e153000)), (locals.var_t6_dn14 / (2.0 * assign100870_e153000)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign100870_e153002;
        locals.var_t6_dn0 = assign100870_e153002_d_n0;
        locals.var_t6_dn2 = assign100870_e153002_d_n2;
        locals.var_t6_dn4 = assign100870_e153002_d_n4;
        locals.var_t6_dn5 = assign100870_e153002_d_n5;
        locals.var_t6_dn6 = assign100870_e153002_d_n6;
        locals.var_t6_dn7 = assign100870_e153002_d_n7;
        locals.var_t6_dn8 = assign100870_e153002_d_n8;
        locals.var_t6_dn9 = assign100870_e153002_d_n9;
        locals.var_t6_dn10 = assign100870_e153002_d_n10;
        locals.var_t6_dn11 = assign100870_e153002_d_n11;
        locals.var_t6_dn14 = assign100870_e153002_d_n14;

        let (assign100880_e153014, assign100880_e153014_d_n0, assign100880_e153014_d_n2, assign100880_e153014_d_n4, assign100880_e153014_d_n5, assign100880_e153014_d_n6, assign100880_e153014_d_n7, assign100880_e153014_d_n8, assign100880_e153014_d_n9, assign100880_e153014_d_n10, assign100880_e153014_d_n11, assign100880_e153014_d_n14,) = {
    if ((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) {
        let assign100880_e153010: f64 = (1.0 - locals.var_t6);
        let assign100880_e153011: f64 = (locals.var_t3 * assign100880_e153010);
        let assign100880_e153012: f64 = (locals.var_t1 + assign100880_e153011);
        (assign100880_e153012, (locals.var_t1_dn0 + ((locals.var_t3_dn0 * assign100880_e153010) + (locals.var_t3 * (-locals.var_t6_dn0)))), (locals.var_t1_dn2 + ((locals.var_t3_dn2 * assign100880_e153010) + (locals.var_t3 * (-locals.var_t6_dn2)))), (locals.var_t1_dn4 + ((locals.var_t3_dn4 * assign100880_e153010) + (locals.var_t3 * (-locals.var_t6_dn4)))), (locals.var_t1_dn5 + ((locals.var_t3_dn5 * assign100880_e153010) + (locals.var_t3 * (-locals.var_t6_dn5)))), (locals.var_t1_dn6 + ((locals.var_t3_dn6 * assign100880_e153010) + (locals.var_t3 * (-locals.var_t6_dn6)))), (locals.var_t1_dn7 + ((locals.var_t3_dn7 * assign100880_e153010) + (locals.var_t3 * (-locals.var_t6_dn7)))), (locals.var_t1_dn8 + ((locals.var_t3_dn8 * assign100880_e153010) + (locals.var_t3 * (-locals.var_t6_dn8)))), (locals.var_t1_dn9 + ((locals.var_t3_dn9 * assign100880_e153010) + (locals.var_t3 * (-locals.var_t6_dn9)))), (locals.var_t1_dn10 + ((locals.var_t3_dn10 * assign100880_e153010) + (locals.var_t3 * (-locals.var_t6_dn10)))), (locals.var_t1_dn11 + ((locals.var_t3_dn11 * assign100880_e153010) + (locals.var_t3 * (-locals.var_t6_dn11)))), (locals.var_t1_dn14 + ((locals.var_t3_dn14 * assign100880_e153010) + (locals.var_t3 * (-locals.var_t6_dn14)))),)
    } else {
        (locals.var_psislsat, locals.var_psislsat_dn0, locals.var_psislsat_dn2, locals.var_psislsat_dn4, locals.var_psislsat_dn5, locals.var_psislsat_dn6, locals.var_psislsat_dn7, locals.var_psislsat_dn8, locals.var_psislsat_dn9, locals.var_psislsat_dn10, locals.var_psislsat_dn11, locals.var_psislsat_dn14,)
    }
};
        locals.var_psislsat = assign100880_e153014;
        locals.var_psislsat_dn0 = assign100880_e153014_d_n0;
        locals.var_psislsat_dn2 = assign100880_e153014_d_n2;
        locals.var_psislsat_dn4 = assign100880_e153014_d_n4;
        locals.var_psislsat_dn5 = assign100880_e153014_d_n5;
        locals.var_psislsat_dn6 = assign100880_e153014_d_n6;
        locals.var_psislsat_dn7 = assign100880_e153014_d_n7;
        locals.var_psislsat_dn8 = assign100880_e153014_d_n8;
        locals.var_psislsat_dn9 = assign100880_e153014_d_n9;
        locals.var_psislsat_dn10 = assign100880_e153014_d_n10;
        locals.var_psislsat_dn11 = assign100880_e153014_d_n11;
        locals.var_psislsat_dn14 = assign100880_e153014_d_n14;

        let (assign100890_e153024, assign100890_e153024_d_n0, assign100890_e153024_d_n2, assign100890_e153024_d_n4, assign100890_e153024_d_n5, assign100890_e153024_d_n6, assign100890_e153024_d_n7, assign100890_e153024_d_n8, assign100890_e153024_d_n9, assign100890_e153024_d_n10, assign100890_e153024_d_n11, assign100890_e153024_d_n14,) = {
    if ((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) {
        let assign100890_e153021: f64 = (locals.var_xgate_1 + locals.var_lgate);
        let assign100890_e153022: f64 = (locals.var_lgate / assign100890_e153021);
        (assign100890_e153022, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign100890_e153024;
        locals.var_t2_dn0 = assign100890_e153024_d_n0;
        locals.var_t2_dn2 = assign100890_e153024_d_n2;
        locals.var_t2_dn4 = assign100890_e153024_d_n4;
        locals.var_t2_dn5 = assign100890_e153024_d_n5;
        locals.var_t2_dn6 = assign100890_e153024_d_n6;
        locals.var_t2_dn7 = assign100890_e153024_d_n7;
        locals.var_t2_dn8 = assign100890_e153024_d_n8;
        locals.var_t2_dn9 = assign100890_e153024_d_n9;
        locals.var_t2_dn10 = assign100890_e153024_d_n10;
        locals.var_t2_dn11 = assign100890_e153024_d_n11;
        locals.var_t2_dn14 = assign100890_e153024_d_n14;

        let (assign100900_e153038, assign100900_e153038_d_n0, assign100900_e153038_d_n2, assign100900_e153038_d_n4, assign100900_e153038_d_n5, assign100900_e153038_d_n6, assign100900_e153038_d_n7, assign100900_e153038_d_n8, assign100900_e153038_d_n9, assign100900_e153038_d_n10, assign100900_e153038_d_n11, assign100900_e153038_d_n14,) = {
    if ((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) {
        let assign100900_e153030: f64 = (locals.var_uc_svdssnp * locals.var_vdsz__blk441);
        let assign100900_e153032: f64 = (assign100900_e153030 + locals.var_ps0z);
        let assign100900_e153035: f64 = (locals.var_t2 * locals.var_psislsat);
        let assign100900_e153036: f64 = (assign100900_e153032 - assign100900_e153035);
        (assign100900_e153036, (((locals.var_uc_svdssnp * locals.var_vdsz__blk441_dn0) + locals.var_ps0z_dn0) - ((locals.var_t2_dn0 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn0))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk441_dn2) + locals.var_ps0z_dn2) - ((locals.var_t2_dn2 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn2))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk441_dn4) + locals.var_ps0z_dn4) - ((locals.var_t2_dn4 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn4))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk441_dn5) + locals.var_ps0z_dn5) - ((locals.var_t2_dn5 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn5))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk441_dn6) + locals.var_ps0z_dn6) - ((locals.var_t2_dn6 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn6))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk441_dn7) + locals.var_ps0z_dn7) - ((locals.var_t2_dn7 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn7))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk441_dn8) + locals.var_ps0z_dn8) - ((locals.var_t2_dn8 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn8))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk441_dn9) + locals.var_ps0z_dn9) - ((locals.var_t2_dn9 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn9))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk441_dn10) + locals.var_ps0z_dn10) - ((locals.var_t2_dn10 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn10))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk441_dn11) + locals.var_ps0z_dn11) - ((locals.var_t2_dn11 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn11))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk441_dn14) + locals.var_ps0z_dn14) - ((locals.var_t2_dn14 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn14))),)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn11, locals.var_psisubsat_dn14,)
    }
};
        locals.var_psisubsat = assign100900_e153038;
        locals.var_psisubsat_dn0 = assign100900_e153038_d_n0;
        locals.var_psisubsat_dn2 = assign100900_e153038_d_n2;
        locals.var_psisubsat_dn4 = assign100900_e153038_d_n4;
        locals.var_psisubsat_dn5 = assign100900_e153038_d_n5;
        locals.var_psisubsat_dn6 = assign100900_e153038_d_n6;
        locals.var_psisubsat_dn7 = assign100900_e153038_d_n7;
        locals.var_psisubsat_dn8 = assign100900_e153038_d_n8;
        locals.var_psisubsat_dn9 = assign100900_e153038_d_n9;
        locals.var_psisubsat_dn10 = assign100900_e153038_d_n10;
        locals.var_psisubsat_dn11 = assign100900_e153038_d_n11;
        locals.var_psisubsat_dn14 = assign100900_e153038_d_n14;

        let (assign100910_e153053, assign100910_e153053_d_n0, assign100910_e153053_d_n2, assign100910_e153053_d_n4, assign100910_e153053_d_n5, assign100910_e153053_d_n6, assign100910_e153053_d_n7, assign100910_e153053_d_n8, assign100910_e153053_d_n9, assign100910_e153053_d_n10, assign100910_e153053_d_n11, assign100910_e153053_d_n14,) = {
    if ((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) {
        let assign100910_e153044: f64 = (locals.var_psisubsat * locals.var_psisubsat);
        let assign100910_e153047: f64 = (4.0 * 0.001);
        let assign100910_e153049: f64 = (assign100910_e153047 * 0.001);
        let assign100910_e153050: f64 = (assign100910_e153044 + assign100910_e153049);
        let assign100910_e153051: f64 = (assign100910_e153050).sqrt();
        (assign100910_e153051, (((locals.var_psisubsat_dn0 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn0)) / (2.0 * assign100910_e153051)), (((locals.var_psisubsat_dn2 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn2)) / (2.0 * assign100910_e153051)), (((locals.var_psisubsat_dn4 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn4)) / (2.0 * assign100910_e153051)), (((locals.var_psisubsat_dn5 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn5)) / (2.0 * assign100910_e153051)), (((locals.var_psisubsat_dn6 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn6)) / (2.0 * assign100910_e153051)), (((locals.var_psisubsat_dn7 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn7)) / (2.0 * assign100910_e153051)), (((locals.var_psisubsat_dn8 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn8)) / (2.0 * assign100910_e153051)), (((locals.var_psisubsat_dn9 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn9)) / (2.0 * assign100910_e153051)), (((locals.var_psisubsat_dn10 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn10)) / (2.0 * assign100910_e153051)), (((locals.var_psisubsat_dn11 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn11)) / (2.0 * assign100910_e153051)), (((locals.var_psisubsat_dn14 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn14)) / (2.0 * assign100910_e153051)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign100910_e153053;
        locals.var_tmf2_dn0 = assign100910_e153053_d_n0;
        locals.var_tmf2_dn2 = assign100910_e153053_d_n2;
        locals.var_tmf2_dn4 = assign100910_e153053_d_n4;
        locals.var_tmf2_dn5 = assign100910_e153053_d_n5;
        locals.var_tmf2_dn6 = assign100910_e153053_d_n6;
        locals.var_tmf2_dn7 = assign100910_e153053_d_n7;
        locals.var_tmf2_dn8 = assign100910_e153053_d_n8;
        locals.var_tmf2_dn9 = assign100910_e153053_d_n9;
        locals.var_tmf2_dn10 = assign100910_e153053_d_n10;
        locals.var_tmf2_dn11 = assign100910_e153053_d_n11;
        locals.var_tmf2_dn14 = assign100910_e153053_d_n14;

        let (assign100920_e153065, assign100920_e153065_d_n0, assign100920_e153065_d_n2, assign100920_e153065_d_n4, assign100920_e153065_d_n5, assign100920_e153065_d_n6, assign100920_e153065_d_n7, assign100920_e153065_d_n8, assign100920_e153065_d_n9, assign100920_e153065_d_n10, assign100920_e153065_d_n11, assign100920_e153065_d_n14,) = {
    if ((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) {
        let assign100920_e153061: f64 = (locals.var_psisubsat / locals.var_tmf2);
        let assign100920_e153062: f64 = (1.0 + assign100920_e153061);
        let assign100920_e153063: f64 = (0.5 * assign100920_e153062);
        (assign100920_e153063, (0.5 * (((locals.var_psisubsat_dn0 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn2 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn4 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn5 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn6 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn7 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn8 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn9 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn10 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn11 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn14 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign100920_e153065;
        locals.var_t9_dn0 = assign100920_e153065_d_n0;
        locals.var_t9_dn2 = assign100920_e153065_d_n2;
        locals.var_t9_dn4 = assign100920_e153065_d_n4;
        locals.var_t9_dn5 = assign100920_e153065_d_n5;
        locals.var_t9_dn6 = assign100920_e153065_d_n6;
        locals.var_t9_dn7 = assign100920_e153065_d_n7;
        locals.var_t9_dn8 = assign100920_e153065_d_n8;
        locals.var_t9_dn9 = assign100920_e153065_d_n9;
        locals.var_t9_dn10 = assign100920_e153065_d_n10;
        locals.var_t9_dn11 = assign100920_e153065_d_n11;
        locals.var_t9_dn14 = assign100920_e153065_d_n14;

        let (assign100930_e153075, assign100930_e153075_d_n0, assign100930_e153075_d_n2, assign100930_e153075_d_n4, assign100930_e153075_d_n5, assign100930_e153075_d_n6, assign100930_e153075_d_n7, assign100930_e153075_d_n8, assign100930_e153075_d_n9, assign100930_e153075_d_n10, assign100930_e153075_d_n11, assign100930_e153075_d_n14,) = {
    if ((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) {
        let assign100930_e153072: f64 = (locals.var_psisubsat + locals.var_tmf2);
        let assign100930_e153073: f64 = (0.5 * assign100930_e153072);
        (assign100930_e153073, (0.5 * (locals.var_psisubsat_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_psisubsat_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_psisubsat_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_psisubsat_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_psisubsat_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_psisubsat_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_psisubsat_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_psisubsat_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_psisubsat_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_psisubsat_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_psisubsat_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn11, locals.var_psisubsat_dn14,)
    }
};
        locals.var_psisubsat = assign100930_e153075;
        locals.var_psisubsat_dn0 = assign100930_e153075_d_n0;
        locals.var_psisubsat_dn2 = assign100930_e153075_d_n2;
        locals.var_psisubsat_dn4 = assign100930_e153075_d_n4;
        locals.var_psisubsat_dn5 = assign100930_e153075_d_n5;
        locals.var_psisubsat_dn6 = assign100930_e153075_d_n6;
        locals.var_psisubsat_dn7 = assign100930_e153075_d_n7;
        locals.var_psisubsat_dn8 = assign100930_e153075_d_n8;
        locals.var_psisubsat_dn9 = assign100930_e153075_d_n9;
        locals.var_psisubsat_dn10 = assign100930_e153075_d_n10;
        locals.var_psisubsat_dn11 = assign100930_e153075_d_n11;
        locals.var_psisubsat_dn14 = assign100930_e153075_d_n14;

        let assign100940_e153078: f64 = if locals.var_psisubsat < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2321 = assign100940_e153078;

        let (assign100950_e153086, assign100950_e153086_d_n0, assign100950_e153086_d_n2, assign100950_e153086_d_n4, assign100950_e153086_d_n5, assign100950_e153086_d_n6, assign100950_e153086_d_n7, assign100950_e153086_d_n8, assign100950_e153086_d_n9, assign100950_e153086_d_n10, assign100950_e153086_d_n11, assign100950_e153086_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2321 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn11, locals.var_psisubsat_dn14,)
    }
};
        locals.var_psisubsat = assign100950_e153086;
        locals.var_psisubsat_dn0 = assign100950_e153086_d_n0;
        locals.var_psisubsat_dn2 = assign100950_e153086_d_n2;
        locals.var_psisubsat_dn4 = assign100950_e153086_d_n4;
        locals.var_psisubsat_dn5 = assign100950_e153086_d_n5;
        locals.var_psisubsat_dn6 = assign100950_e153086_d_n6;
        locals.var_psisubsat_dn7 = assign100950_e153086_d_n7;
        locals.var_psisubsat_dn8 = assign100950_e153086_d_n8;
        locals.var_psisubsat_dn9 = assign100950_e153086_d_n9;
        locals.var_psisubsat_dn10 = assign100950_e153086_d_n10;
        locals.var_psisubsat_dn11 = assign100950_e153086_d_n11;
        locals.var_psisubsat_dn14 = assign100950_e153086_d_n14;

        let (assign100960_e153094, assign100960_e153094_d_n0, assign100960_e153094_d_n2, assign100960_e153094_d_n4, assign100960_e153094_d_n5, assign100960_e153094_d_n6, assign100960_e153094_d_n7, assign100960_e153094_d_n8, assign100960_e153094_d_n9, assign100960_e153094_d_n10, assign100960_e153094_d_n11, assign100960_e153094_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2321 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign100960_e153094;
        locals.var_t9_dn0 = assign100960_e153094_d_n0;
        locals.var_t9_dn2 = assign100960_e153094_d_n2;
        locals.var_t9_dn4 = assign100960_e153094_d_n4;
        locals.var_t9_dn5 = assign100960_e153094_d_n5;
        locals.var_t9_dn6 = assign100960_e153094_d_n6;
        locals.var_t9_dn7 = assign100960_e153094_d_n7;
        locals.var_t9_dn8 = assign100960_e153094_d_n8;
        locals.var_t9_dn9 = assign100960_e153094_d_n9;
        locals.var_t9_dn10 = assign100960_e153094_d_n10;
        locals.var_t9_dn11 = assign100960_e153094_d_n11;
        locals.var_t9_dn14 = assign100960_e153094_d_n14;

        let (assign100970_e153102, assign100970_e153102_d_n0, assign100970_e153102_d_n2, assign100970_e153102_d_n4, assign100970_e153102_d_n5, assign100970_e153102_d_n6, assign100970_e153102_d_n7, assign100970_e153102_d_n8, assign100970_e153102_d_n9, assign100970_e153102_d_n10, assign100970_e153102_d_n11, assign100970_e153102_d_n14,) = {
    if ((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) {
        let assign100970_e153100: f64 = (locals.var_psisubsat + 1e-25);
        (assign100970_e153100, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn11, locals.var_psisubsat_dn14,)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn11, locals.var_psisubsat_dn14,)
    }
};
        locals.var_psisubsat = assign100970_e153102;
        locals.var_psisubsat_dn0 = assign100970_e153102_d_n0;
        locals.var_psisubsat_dn2 = assign100970_e153102_d_n2;
        locals.var_psisubsat_dn4 = assign100970_e153102_d_n4;
        locals.var_psisubsat_dn5 = assign100970_e153102_d_n5;
        locals.var_psisubsat_dn6 = assign100970_e153102_d_n6;
        locals.var_psisubsat_dn7 = assign100970_e153102_d_n7;
        locals.var_psisubsat_dn8 = assign100970_e153102_d_n8;
        locals.var_psisubsat_dn9 = assign100970_e153102_d_n9;
        locals.var_psisubsat_dn10 = assign100970_e153102_d_n10;
        locals.var_psisubsat_dn11 = assign100970_e153102_d_n11;
        locals.var_psisubsat_dn14 = assign100970_e153102_d_n14;

        let (assign100980_e153114, assign100980_e153114_d_n0, assign100980_e153114_d_n2, assign100980_e153114_d_n4, assign100980_e153114_d_n5, assign100980_e153114_d_n6, assign100980_e153114_d_n7, assign100980_e153114_d_n8, assign100980_e153114_d_n9, assign100980_e153114_d_n10, assign100980_e153114_d_n11, assign100980_e153114_d_n14,) = {
    if ((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) {
        let assign100980_e153110: f64 = (locals.var_ttemp - locals.var_ktnom);
        let assign100980_e153111: f64 = (locals.var_uc_subtmp * assign100980_e153110);
        let assign100980_e153112: f64 = (1.0 + assign100980_e153111);
        (assign100980_e153112, (locals.var_uc_subtmp * locals.var_ttemp_dn0), (locals.var_uc_subtmp * locals.var_ttemp_dn2), (locals.var_uc_subtmp * locals.var_ttemp_dn4), (locals.var_uc_subtmp * locals.var_ttemp_dn5), (locals.var_uc_subtmp * locals.var_ttemp_dn6), (locals.var_uc_subtmp * locals.var_ttemp_dn7), (locals.var_uc_subtmp * locals.var_ttemp_dn8), (locals.var_uc_subtmp * locals.var_ttemp_dn9), (locals.var_uc_subtmp * locals.var_ttemp_dn10), (locals.var_uc_subtmp * locals.var_ttemp_dn11), (locals.var_uc_subtmp * locals.var_ttemp_dn14),)
    } else {
        (locals.var_xsubtmp, locals.var_xsubtmp_dn0, locals.var_xsubtmp_dn2, locals.var_xsubtmp_dn4, locals.var_xsubtmp_dn5, locals.var_xsubtmp_dn6, locals.var_xsubtmp_dn7, locals.var_xsubtmp_dn8, locals.var_xsubtmp_dn9, locals.var_xsubtmp_dn10, locals.var_xsubtmp_dn11, locals.var_xsubtmp_dn14,)
    }
};
        locals.var_xsubtmp = assign100980_e153114;
        locals.var_xsubtmp_dn0 = assign100980_e153114_d_n0;
        locals.var_xsubtmp_dn2 = assign100980_e153114_d_n2;
        locals.var_xsubtmp_dn4 = assign100980_e153114_d_n4;
        locals.var_xsubtmp_dn5 = assign100980_e153114_d_n5;
        locals.var_xsubtmp_dn6 = assign100980_e153114_d_n6;
        locals.var_xsubtmp_dn7 = assign100980_e153114_d_n7;
        locals.var_xsubtmp_dn8 = assign100980_e153114_d_n8;
        locals.var_xsubtmp_dn9 = assign100980_e153114_d_n9;
        locals.var_xsubtmp_dn10 = assign100980_e153114_d_n10;
        locals.var_xsubtmp_dn11 = assign100980_e153114_d_n11;
        locals.var_xsubtmp_dn14 = assign100980_e153114_d_n14;

    }

    pub(super) fn stamp_transient_block_370(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign100990_e153125, assign100990_e153125_d_n0, assign100990_e153125_d_n2, assign100990_e153125_d_n4, assign100990_e153125_d_n5, assign100990_e153125_d_n6, assign100990_e153125_d_n7, assign100990_e153125_d_n8, assign100990_e153125_d_n9, assign100990_e153125_d_n10, assign100990_e153125_d_n11, assign100990_e153125_d_n14,) = {
    if ((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) {
        let (assign100990_e153123, assign100990_e153123_d_n0, assign100990_e153123_d_n2, assign100990_e153123_d_n4, assign100990_e153123_d_n5, assign100990_e153123_d_n6, assign100990_e153123_d_n7, assign100990_e153123_d_n8, assign100990_e153123_d_n9, assign100990_e153123_d_n10, assign100990_e153123_d_n11, assign100990_e153123_d_n14,) = {
            if (locals.var_xsubtmp <= 0.001) {
                (0.001, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                (locals.var_xsubtmp, locals.var_xsubtmp_dn0, locals.var_xsubtmp_dn2, locals.var_xsubtmp_dn4, locals.var_xsubtmp_dn5, locals.var_xsubtmp_dn6, locals.var_xsubtmp_dn7, locals.var_xsubtmp_dn8, locals.var_xsubtmp_dn9, locals.var_xsubtmp_dn10, locals.var_xsubtmp_dn11, locals.var_xsubtmp_dn14,)
            }
        };
        (assign100990_e153123, assign100990_e153123_d_n0, assign100990_e153123_d_n2, assign100990_e153123_d_n4, assign100990_e153123_d_n5, assign100990_e153123_d_n6, assign100990_e153123_d_n7, assign100990_e153123_d_n8, assign100990_e153123_d_n9, assign100990_e153123_d_n10, assign100990_e153123_d_n11, assign100990_e153123_d_n14,)
    } else {
        (locals.var_xsubtmp, locals.var_xsubtmp_dn0, locals.var_xsubtmp_dn2, locals.var_xsubtmp_dn4, locals.var_xsubtmp_dn5, locals.var_xsubtmp_dn6, locals.var_xsubtmp_dn7, locals.var_xsubtmp_dn8, locals.var_xsubtmp_dn9, locals.var_xsubtmp_dn10, locals.var_xsubtmp_dn11, locals.var_xsubtmp_dn14,)
    }
};
        locals.var_xsubtmp = assign100990_e153125;
        locals.var_xsubtmp_dn0 = assign100990_e153125_d_n0;
        locals.var_xsubtmp_dn2 = assign100990_e153125_d_n2;
        locals.var_xsubtmp_dn4 = assign100990_e153125_d_n4;
        locals.var_xsubtmp_dn5 = assign100990_e153125_d_n5;
        locals.var_xsubtmp_dn6 = assign100990_e153125_d_n6;
        locals.var_xsubtmp_dn7 = assign100990_e153125_d_n7;
        locals.var_xsubtmp_dn8 = assign100990_e153125_d_n8;
        locals.var_xsubtmp_dn9 = assign100990_e153125_d_n9;
        locals.var_xsubtmp_dn10 = assign100990_e153125_d_n10;
        locals.var_xsubtmp_dn11 = assign100990_e153125_d_n11;
        locals.var_xsubtmp_dn14 = assign100990_e153125_d_n14;

        let (assign101000_e153133, assign101000_e153133_d_n0, assign101000_e153133_d_n2, assign101000_e153133_d_n4, assign101000_e153133_d_n5, assign101000_e153133_d_n6, assign101000_e153133_d_n7, assign101000_e153133_d_n8, assign101000_e153133_d_n9, assign101000_e153133_d_n10, assign101000_e153133_d_n11, assign101000_e153133_d_n14,) = {
    if ((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) {
        let assign101000_e153131: f64 = (locals.var_xsub1_1 / locals.var_xsubtmp);
        (assign101000_e153131, (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn0) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn2) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn4) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn5) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn6) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn7) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn8) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn9) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn10) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn11) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn14) / (locals.var_xsubtmp * locals.var_xsubtmp))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign101000_e153133;
        locals.var_t5_dn0 = assign101000_e153133_d_n0;
        locals.var_t5_dn2 = assign101000_e153133_d_n2;
        locals.var_t5_dn4 = assign101000_e153133_d_n4;
        locals.var_t5_dn5 = assign101000_e153133_d_n5;
        locals.var_t5_dn6 = assign101000_e153133_d_n6;
        locals.var_t5_dn7 = assign101000_e153133_d_n7;
        locals.var_t5_dn8 = assign101000_e153133_d_n8;
        locals.var_t5_dn9 = assign101000_e153133_d_n9;
        locals.var_t5_dn10 = assign101000_e153133_d_n10;
        locals.var_t5_dn11 = assign101000_e153133_d_n11;
        locals.var_t5_dn14 = assign101000_e153133_d_n14;

        let (assign101010_e153141, assign101010_e153141_d_n0, assign101010_e153141_d_n2, assign101010_e153141_d_n4, assign101010_e153141_d_n5, assign101010_e153141_d_n6, assign101010_e153141_d_n7, assign101010_e153141_d_n8, assign101010_e153141_d_n9, assign101010_e153141_d_n10, assign101010_e153141_d_n11, assign101010_e153141_d_n14,) = {
    if ((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) {
        let assign101010_e153139: f64 = (locals.var_xsub2_1 * locals.var_xsubtmp);
        (assign101010_e153139, (locals.var_xsub2_1 * locals.var_xsubtmp_dn0), (locals.var_xsub2_1 * locals.var_xsubtmp_dn2), (locals.var_xsub2_1 * locals.var_xsubtmp_dn4), (locals.var_xsub2_1 * locals.var_xsubtmp_dn5), (locals.var_xsub2_1 * locals.var_xsubtmp_dn6), (locals.var_xsub2_1 * locals.var_xsubtmp_dn7), (locals.var_xsub2_1 * locals.var_xsubtmp_dn8), (locals.var_xsub2_1 * locals.var_xsubtmp_dn9), (locals.var_xsub2_1 * locals.var_xsubtmp_dn10), (locals.var_xsub2_1 * locals.var_xsubtmp_dn11), (locals.var_xsub2_1 * locals.var_xsubtmp_dn14),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign101010_e153141;
        locals.var_t6_dn0 = assign101010_e153141_d_n0;
        locals.var_t6_dn2 = assign101010_e153141_d_n2;
        locals.var_t6_dn4 = assign101010_e153141_d_n4;
        locals.var_t6_dn5 = assign101010_e153141_d_n5;
        locals.var_t6_dn6 = assign101010_e153141_d_n6;
        locals.var_t6_dn7 = assign101010_e153141_d_n7;
        locals.var_t6_dn8 = assign101010_e153141_d_n8;
        locals.var_t6_dn9 = assign101010_e153141_d_n9;
        locals.var_t6_dn10 = assign101010_e153141_d_n10;
        locals.var_t6_dn11 = assign101010_e153141_d_n11;
        locals.var_t6_dn14 = assign101010_e153141_d_n14;

        let (assign101020_e153151, assign101020_e153151_d_n0, assign101020_e153151_d_n2, assign101020_e153151_d_n4, assign101020_e153151_d_n5, assign101020_e153151_d_n6, assign101020_e153151_d_n7, assign101020_e153151_d_n8, assign101020_e153151_d_n9, assign101020_e153151_d_n10, assign101020_e153151_d_n11, assign101020_e153151_d_n14,) = {
    if ((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) {
        let assign101020_e153146: f64 = (-locals.var_t6);
        let assign101020_e153148: f64 = (assign101020_e153146 / locals.var_psisubsat);
        let assign101020_e153149: f64 = (assign101020_e153148).exp();
        (assign101020_e153149, (assign101020_e153149 * ((((-locals.var_t6_dn0) * locals.var_psisubsat) - (assign101020_e153146 * locals.var_psisubsat_dn0)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101020_e153149 * ((((-locals.var_t6_dn2) * locals.var_psisubsat) - (assign101020_e153146 * locals.var_psisubsat_dn2)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101020_e153149 * ((((-locals.var_t6_dn4) * locals.var_psisubsat) - (assign101020_e153146 * locals.var_psisubsat_dn4)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101020_e153149 * ((((-locals.var_t6_dn5) * locals.var_psisubsat) - (assign101020_e153146 * locals.var_psisubsat_dn5)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101020_e153149 * ((((-locals.var_t6_dn6) * locals.var_psisubsat) - (assign101020_e153146 * locals.var_psisubsat_dn6)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101020_e153149 * ((((-locals.var_t6_dn7) * locals.var_psisubsat) - (assign101020_e153146 * locals.var_psisubsat_dn7)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101020_e153149 * ((((-locals.var_t6_dn8) * locals.var_psisubsat) - (assign101020_e153146 * locals.var_psisubsat_dn8)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101020_e153149 * ((((-locals.var_t6_dn9) * locals.var_psisubsat) - (assign101020_e153146 * locals.var_psisubsat_dn9)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101020_e153149 * ((((-locals.var_t6_dn10) * locals.var_psisubsat) - (assign101020_e153146 * locals.var_psisubsat_dn10)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101020_e153149 * ((((-locals.var_t6_dn11) * locals.var_psisubsat) - (assign101020_e153146 * locals.var_psisubsat_dn11)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101020_e153149 * ((((-locals.var_t6_dn14) * locals.var_psisubsat) - (assign101020_e153146 * locals.var_psisubsat_dn14)) / (locals.var_psisubsat * locals.var_psisubsat))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign101020_e153151;
        locals.var_t2_dn0 = assign101020_e153151_d_n0;
        locals.var_t2_dn2 = assign101020_e153151_d_n2;
        locals.var_t2_dn4 = assign101020_e153151_d_n4;
        locals.var_t2_dn5 = assign101020_e153151_d_n5;
        locals.var_t2_dn6 = assign101020_e153151_d_n6;
        locals.var_t2_dn7 = assign101020_e153151_d_n7;
        locals.var_t2_dn8 = assign101020_e153151_d_n8;
        locals.var_t2_dn9 = assign101020_e153151_d_n9;
        locals.var_t2_dn10 = assign101020_e153151_d_n10;
        locals.var_t2_dn11 = assign101020_e153151_d_n11;
        locals.var_t2_dn14 = assign101020_e153151_d_n14;

        let assign101070_e153192: f64 = if locals.var_flg_noqi == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2323 = assign101070_e153192;

        let (assign101080_e153198, assign101080_e153198_d_n0, assign101080_e153198_d_n2, assign101080_e153198_d_n4, assign101080_e153198_d_n5, assign101080_e153198_d_n6, assign101080_e153198_d_n7, assign101080_e153198_d_n8, assign101080_e153198_d_n9, assign101080_e153198_d_n10, assign101080_e153198_d_n11, assign101080_e153198_d_n14,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard2323 != 0.0)) {
        (p.p270, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t12, locals.var_t12_dn0, locals.var_t12_dn2, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn14,)
    }
};
        locals.var_t12 = assign101080_e153198;
        locals.var_t12_dn0 = assign101080_e153198_d_n0;
        locals.var_t12_dn2 = assign101080_e153198_d_n2;
        locals.var_t12_dn4 = assign101080_e153198_d_n4;
        locals.var_t12_dn5 = assign101080_e153198_d_n5;
        locals.var_t12_dn6 = assign101080_e153198_d_n6;
        locals.var_t12_dn7 = assign101080_e153198_d_n7;
        locals.var_t12_dn8 = assign101080_e153198_d_n8;
        locals.var_t12_dn9 = assign101080_e153198_d_n9;
        locals.var_t12_dn10 = assign101080_e153198_d_n10;
        locals.var_t12_dn11 = assign101080_e153198_d_n11;
        locals.var_t12_dn14 = assign101080_e153198_d_n14;

        let (assign101090_e153204, assign101090_e153204_d_n0, assign101090_e153204_d_n2, assign101090_e153204_d_n4, assign101090_e153204_d_n5, assign101090_e153204_d_n6, assign101090_e153204_d_n7, assign101090_e153204_d_n8, assign101090_e153204_d_n9, assign101090_e153204_d_n10, assign101090_e153204_d_n11, assign101090_e153204_d_n14,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard2323 != 0.0)) {
        (p.p271, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign101090_e153204;
        locals.var_t10_dn0 = assign101090_e153204_d_n0;
        locals.var_t10_dn2 = assign101090_e153204_d_n2;
        locals.var_t10_dn4 = assign101090_e153204_d_n4;
        locals.var_t10_dn5 = assign101090_e153204_d_n5;
        locals.var_t10_dn6 = assign101090_e153204_d_n6;
        locals.var_t10_dn7 = assign101090_e153204_d_n7;
        locals.var_t10_dn8 = assign101090_e153204_d_n8;
        locals.var_t10_dn9 = assign101090_e153204_d_n9;
        locals.var_t10_dn10 = assign101090_e153204_d_n10;
        locals.var_t10_dn11 = assign101090_e153204_d_n11;
        locals.var_t10_dn14 = assign101090_e153204_d_n14;

        let (assign101100_e153210, assign101100_e153210_d_n0, assign101100_e153210_d_n2, assign101100_e153210_d_n4, assign101100_e153210_d_n5, assign101100_e153210_d_n6, assign101100_e153210_d_n7, assign101100_e153210_d_n8, assign101100_e153210_d_n9, assign101100_e153210_d_n10, assign101100_e153210_d_n11, assign101100_e153210_d_n14,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard2323 != 0.0)) {
        (locals.var_lch, locals.var_lch_dn0, locals.var_lch_dn2, locals.var_lch_dn4, locals.var_lch_dn5, locals.var_lch_dn6, locals.var_lch_dn7, locals.var_lch_dn8, locals.var_lch_dn9, locals.var_lch_dn10, locals.var_lch_dn11, locals.var_lch_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign101100_e153210;
        locals.var_t3_dn0 = assign101100_e153210_d_n0;
        locals.var_t3_dn2 = assign101100_e153210_d_n2;
        locals.var_t3_dn4 = assign101100_e153210_d_n4;
        locals.var_t3_dn5 = assign101100_e153210_d_n5;
        locals.var_t3_dn6 = assign101100_e153210_d_n6;
        locals.var_t3_dn7 = assign101100_e153210_d_n7;
        locals.var_t3_dn8 = assign101100_e153210_d_n8;
        locals.var_t3_dn9 = assign101100_e153210_d_n9;
        locals.var_t3_dn10 = assign101100_e153210_d_n10;
        locals.var_t3_dn11 = assign101100_e153210_d_n11;
        locals.var_t3_dn14 = assign101100_e153210_d_n14;

        let (assign101110_e153222, assign101110_e153222_d_n0, assign101110_e153222_d_n2, assign101110_e153222_d_n4, assign101110_e153222_d_n5, assign101110_e153222_d_n6, assign101110_e153222_d_n7, assign101110_e153222_d_n8, assign101110_e153222_d_n9, assign101110_e153222_d_n10, assign101110_e153222_d_n11, assign101110_e153222_d_n14,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard2323 != 0.0)) {
        let assign101110_e153216: f64 = (locals.var_t12 * locals.var_t10);
        let assign101110_e153218: f64 = (assign101110_e153216 * locals.var_t3);
        let assign101110_e153220: f64 = (assign101110_e153218 * locals.var_t3);
        (assign101110_e153220, ((((((locals.var_t12_dn0 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn0)) * locals.var_t3) + (assign101110_e153216 * locals.var_t3_dn0)) * locals.var_t3) + (assign101110_e153218 * locals.var_t3_dn0)), ((((((locals.var_t12_dn2 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn2)) * locals.var_t3) + (assign101110_e153216 * locals.var_t3_dn2)) * locals.var_t3) + (assign101110_e153218 * locals.var_t3_dn2)), ((((((locals.var_t12_dn4 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn4)) * locals.var_t3) + (assign101110_e153216 * locals.var_t3_dn4)) * locals.var_t3) + (assign101110_e153218 * locals.var_t3_dn4)), ((((((locals.var_t12_dn5 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn5)) * locals.var_t3) + (assign101110_e153216 * locals.var_t3_dn5)) * locals.var_t3) + (assign101110_e153218 * locals.var_t3_dn5)), ((((((locals.var_t12_dn6 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn6)) * locals.var_t3) + (assign101110_e153216 * locals.var_t3_dn6)) * locals.var_t3) + (assign101110_e153218 * locals.var_t3_dn6)), ((((((locals.var_t12_dn7 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn7)) * locals.var_t3) + (assign101110_e153216 * locals.var_t3_dn7)) * locals.var_t3) + (assign101110_e153218 * locals.var_t3_dn7)), ((((((locals.var_t12_dn8 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn8)) * locals.var_t3) + (assign101110_e153216 * locals.var_t3_dn8)) * locals.var_t3) + (assign101110_e153218 * locals.var_t3_dn8)), ((((((locals.var_t12_dn9 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn9)) * locals.var_t3) + (assign101110_e153216 * locals.var_t3_dn9)) * locals.var_t3) + (assign101110_e153218 * locals.var_t3_dn9)), ((((((locals.var_t12_dn10 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn10)) * locals.var_t3) + (assign101110_e153216 * locals.var_t3_dn10)) * locals.var_t3) + (assign101110_e153218 * locals.var_t3_dn10)), ((((((locals.var_t12_dn11 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn11)) * locals.var_t3) + (assign101110_e153216 * locals.var_t3_dn11)) * locals.var_t3) + (assign101110_e153218 * locals.var_t3_dn11)), ((((((locals.var_t12_dn14 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn14)) * locals.var_t3) + (assign101110_e153216 * locals.var_t3_dn14)) * locals.var_t3) + (assign101110_e153218 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign101110_e153222;
        locals.var_t1_dn0 = assign101110_e153222_d_n0;
        locals.var_t1_dn2 = assign101110_e153222_d_n2;
        locals.var_t1_dn4 = assign101110_e153222_d_n4;
        locals.var_t1_dn5 = assign101110_e153222_d_n5;
        locals.var_t1_dn6 = assign101110_e153222_d_n6;
        locals.var_t1_dn7 = assign101110_e153222_d_n7;
        locals.var_t1_dn8 = assign101110_e153222_d_n8;
        locals.var_t1_dn9 = assign101110_e153222_d_n9;
        locals.var_t1_dn10 = assign101110_e153222_d_n10;
        locals.var_t1_dn11 = assign101110_e153222_d_n11;
        locals.var_t1_dn14 = assign101110_e153222_d_n14;

        let (assign101120_e153240, assign101120_e153240_d_n0, assign101120_e153240_d_n2, assign101120_e153240_d_n4, assign101120_e153240_d_n5, assign101120_e153240_d_n6, assign101120_e153240_d_n7, assign101120_e153240_d_n8, assign101120_e153240_d_n9, assign101120_e153240_d_n10, assign101120_e153240_d_n11, assign101120_e153240_d_n14,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard2323 != 0.0)) {
        let assign101120_e153228: f64 = (locals.var_mu * locals.var_vgvt);
        let assign101120_e153230: f64 = (assign101120_e153228 * locals.var_t12);
        let assign101120_e153233: f64 = (locals.var_t10 * locals.var_t3);
        let assign101120_e153235: f64 = (assign101120_e153233 * locals.var_t3);
        let assign101120_e153236: f64 = (assign101120_e153230 + assign101120_e153235);
        let assign101120_e153238: f64 = (assign101120_e153236 + 1e-25);
        (assign101120_e153238, (((((locals.var_mu_dn0 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn0)) * locals.var_t12) + (assign101120_e153228 * locals.var_t12_dn0)) + ((((locals.var_t10_dn0 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn0)) * locals.var_t3) + (assign101120_e153233 * locals.var_t3_dn0))), (((((locals.var_mu_dn2 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn2)) * locals.var_t12) + (assign101120_e153228 * locals.var_t12_dn2)) + ((((locals.var_t10_dn2 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn2)) * locals.var_t3) + (assign101120_e153233 * locals.var_t3_dn2))), (((((locals.var_mu_dn4 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn4)) * locals.var_t12) + (assign101120_e153228 * locals.var_t12_dn4)) + ((((locals.var_t10_dn4 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn4)) * locals.var_t3) + (assign101120_e153233 * locals.var_t3_dn4))), (((((locals.var_mu_dn5 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn5)) * locals.var_t12) + (assign101120_e153228 * locals.var_t12_dn5)) + ((((locals.var_t10_dn5 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn5)) * locals.var_t3) + (assign101120_e153233 * locals.var_t3_dn5))), (((((locals.var_mu_dn6 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn6)) * locals.var_t12) + (assign101120_e153228 * locals.var_t12_dn6)) + ((((locals.var_t10_dn6 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn6)) * locals.var_t3) + (assign101120_e153233 * locals.var_t3_dn6))), (((((locals.var_mu_dn7 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn7)) * locals.var_t12) + (assign101120_e153228 * locals.var_t12_dn7)) + ((((locals.var_t10_dn7 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn7)) * locals.var_t3) + (assign101120_e153233 * locals.var_t3_dn7))), (((((locals.var_mu_dn8 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn8)) * locals.var_t12) + (assign101120_e153228 * locals.var_t12_dn8)) + ((((locals.var_t10_dn8 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn8)) * locals.var_t3) + (assign101120_e153233 * locals.var_t3_dn8))), (((((locals.var_mu_dn9 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn9)) * locals.var_t12) + (assign101120_e153228 * locals.var_t12_dn9)) + ((((locals.var_t10_dn9 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn9)) * locals.var_t3) + (assign101120_e153233 * locals.var_t3_dn9))), (((((locals.var_mu_dn10 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn10)) * locals.var_t12) + (assign101120_e153228 * locals.var_t12_dn10)) + ((((locals.var_t10_dn10 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn10)) * locals.var_t3) + (assign101120_e153233 * locals.var_t3_dn10))), (((((locals.var_mu_dn11 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn11)) * locals.var_t12) + (assign101120_e153228 * locals.var_t12_dn11)) + ((((locals.var_t10_dn11 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn11)) * locals.var_t3) + (assign101120_e153233 * locals.var_t3_dn11))), (((((locals.var_mu_dn14 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn14)) * locals.var_t12) + (assign101120_e153228 * locals.var_t12_dn14)) + ((((locals.var_t10_dn14 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn14)) * locals.var_t3) + (assign101120_e153233 * locals.var_t3_dn14))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign101120_e153240;
        locals.var_t2_dn0 = assign101120_e153240_d_n0;
        locals.var_t2_dn2 = assign101120_e153240_d_n2;
        locals.var_t2_dn4 = assign101120_e153240_d_n4;
        locals.var_t2_dn5 = assign101120_e153240_d_n5;
        locals.var_t2_dn6 = assign101120_e153240_d_n6;
        locals.var_t2_dn7 = assign101120_e153240_d_n7;
        locals.var_t2_dn8 = assign101120_e153240_d_n8;
        locals.var_t2_dn9 = assign101120_e153240_d_n9;
        locals.var_t2_dn10 = assign101120_e153240_d_n10;
        locals.var_t2_dn11 = assign101120_e153240_d_n11;
        locals.var_t2_dn14 = assign101120_e153240_d_n14;

        let (assign101150_e153259, assign101150_e153259_d_n0, assign101150_e153259_d_n2, assign101150_e153259_d_n4, assign101150_e153259_d_n5, assign101150_e153259_d_n6, assign101150_e153259_d_n7, assign101150_e153259_d_n8, assign101150_e153259_d_n9, assign101150_e153259_d_n10, assign101150_e153259_d_n11, assign101150_e153259_d_n14,) = {
    if (locals.var_flg_nqs != 0.0) {
        (locals.var_mks_dly3, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign101150_e153259;
        locals.var_t2_dn0 = assign101150_e153259_d_n0;
        locals.var_t2_dn2 = assign101150_e153259_d_n2;
        locals.var_t2_dn4 = assign101150_e153259_d_n4;
        locals.var_t2_dn5 = assign101150_e153259_d_n5;
        locals.var_t2_dn6 = assign101150_e153259_d_n6;
        locals.var_t2_dn7 = assign101150_e153259_d_n7;
        locals.var_t2_dn8 = assign101150_e153259_d_n8;
        locals.var_t2_dn9 = assign101150_e153259_d_n9;
        locals.var_t2_dn10 = assign101150_e153259_d_n10;
        locals.var_t2_dn11 = assign101150_e153259_d_n11;
        locals.var_t2_dn14 = assign101150_e153259_d_n14;

        let assign101170_e153271: f64 = if ((p.p26 != 0.0) && (locals.var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2324 = assign101170_e153271;

        let (assign101180_e153275,) = {
    if (locals.var_guard2324 != 0.0) {
        (locals.var_uc_nfalp,)
    } else {
        (locals.var_nfalpe,)
    }
};
        locals.var_nfalpe = assign101180_e153275;

        let (assign101200_e153283,) = {
    if (locals.var_guard2324 != 0.0) {
        (locals.var_mks_cit,)
    } else {
        (locals.var_cite,)
    }
};
        locals.var_cite = assign101200_e153283;

        let (assign101210_e153289, assign101210_e153289_d_n0, assign101210_e153289_d_n2, assign101210_e153289_d_n4, assign101210_e153289_d_n5, assign101210_e153289_d_n6, assign101210_e153289_d_n7, assign101210_e153289_d_n8, assign101210_e153289_d_n9, assign101210_e153289_d_n10, assign101210_e153289_d_n11, assign101210_e153289_d_n14,) = {
    if (locals.var_guard2324 != 0.0) {
        let assign101210_e153287: f64 = (locals.var_qn0 / 1.6021918e-19);
        (assign101210_e153287, (locals.var_qn0_dn0 / 1.6021918e-19), (locals.var_qn0_dn2 / 1.6021918e-19), (locals.var_qn0_dn4 / 1.6021918e-19), (locals.var_qn0_dn5 / 1.6021918e-19), (locals.var_qn0_dn6 / 1.6021918e-19), (locals.var_qn0_dn7 / 1.6021918e-19), (locals.var_qn0_dn8 / 1.6021918e-19), (locals.var_qn0_dn9 / 1.6021918e-19), (locals.var_qn0_dn10 / 1.6021918e-19), (locals.var_qn0_dn11 / 1.6021918e-19), (locals.var_qn0_dn14 / 1.6021918e-19),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign101210_e153289;
        locals.var_t1_dn0 = assign101210_e153289_d_n0;
        locals.var_t1_dn2 = assign101210_e153289_d_n2;
        locals.var_t1_dn4 = assign101210_e153289_d_n4;
        locals.var_t1_dn5 = assign101210_e153289_d_n5;
        locals.var_t1_dn6 = assign101210_e153289_d_n6;
        locals.var_t1_dn7 = assign101210_e153289_d_n7;
        locals.var_t1_dn8 = assign101210_e153289_d_n8;
        locals.var_t1_dn9 = assign101210_e153289_d_n9;
        locals.var_t1_dn10 = assign101210_e153289_d_n10;
        locals.var_t1_dn11 = assign101210_e153289_d_n11;
        locals.var_t1_dn14 = assign101210_e153289_d_n14;

        let (assign101220_e153306, assign101220_e153306_d_n0, assign101220_e153306_d_n2, assign101220_e153306_d_n4, assign101220_e153306_d_n5, assign101220_e153306_d_n6, assign101220_e153306_d_n7, assign101220_e153306_d_n8, assign101220_e153306_d_n9, assign101220_e153306_d_n10, assign101220_e153306_d_n11, assign101220_e153306_d_n14,) = {
    if (locals.var_guard2324 != 0.0) {
        let assign101220_e153293: f64 = (locals.var_ps0 - locals.var_vbscl__blk437);
        let assign101220_e153296: f64 = (locals.var_ps0 - locals.var_vbscl__blk437);
        let assign101220_e153297: f64 = (assign101220_e153293 * assign101220_e153296);
        let assign101220_e153300: f64 = (4.0 * 0.001);
        let assign101220_e153302: f64 = (assign101220_e153300 * 0.001);
        let assign101220_e153303: f64 = (assign101220_e153297 + assign101220_e153302);
        let assign101220_e153304: f64 = (assign101220_e153303).sqrt();
        (assign101220_e153304, ((((locals.var_ps0_dn0 - locals.var_vbscl__blk437_dn0) * assign101220_e153296) + (assign101220_e153293 * (locals.var_ps0_dn0 - locals.var_vbscl__blk437_dn0))) / (2.0 * assign101220_e153304)), ((((locals.var_ps0_dn2 - locals.var_vbscl__blk437_dn2) * assign101220_e153296) + (assign101220_e153293 * (locals.var_ps0_dn2 - locals.var_vbscl__blk437_dn2))) / (2.0 * assign101220_e153304)), ((((locals.var_ps0_dn4 - locals.var_vbscl__blk437_dn4) * assign101220_e153296) + (assign101220_e153293 * (locals.var_ps0_dn4 - locals.var_vbscl__blk437_dn4))) / (2.0 * assign101220_e153304)), ((((locals.var_ps0_dn5 - locals.var_vbscl__blk437_dn5) * assign101220_e153296) + (assign101220_e153293 * (locals.var_ps0_dn5 - locals.var_vbscl__blk437_dn5))) / (2.0 * assign101220_e153304)), ((((locals.var_ps0_dn6 - locals.var_vbscl__blk437_dn6) * assign101220_e153296) + (assign101220_e153293 * (locals.var_ps0_dn6 - locals.var_vbscl__blk437_dn6))) / (2.0 * assign101220_e153304)), ((((locals.var_ps0_dn7 - locals.var_vbscl__blk437_dn7) * assign101220_e153296) + (assign101220_e153293 * (locals.var_ps0_dn7 - locals.var_vbscl__blk437_dn7))) / (2.0 * assign101220_e153304)), ((((locals.var_ps0_dn8 - locals.var_vbscl__blk437_dn8) * assign101220_e153296) + (assign101220_e153293 * (locals.var_ps0_dn8 - locals.var_vbscl__blk437_dn8))) / (2.0 * assign101220_e153304)), ((((locals.var_ps0_dn9 - locals.var_vbscl__blk437_dn9) * assign101220_e153296) + (assign101220_e153293 * (locals.var_ps0_dn9 - locals.var_vbscl__blk437_dn9))) / (2.0 * assign101220_e153304)), ((((locals.var_ps0_dn10 - locals.var_vbscl__blk437_dn10) * assign101220_e153296) + (assign101220_e153293 * (locals.var_ps0_dn10 - locals.var_vbscl__blk437_dn10))) / (2.0 * assign101220_e153304)), ((((locals.var_ps0_dn11 - locals.var_vbscl__blk437_dn11) * assign101220_e153296) + (assign101220_e153293 * (locals.var_ps0_dn11 - locals.var_vbscl__blk437_dn11))) / (2.0 * assign101220_e153304)), ((((locals.var_ps0_dn14 - locals.var_vbscl__blk437_dn14) * assign101220_e153296) + (assign101220_e153293 * (locals.var_ps0_dn14 - locals.var_vbscl__blk437_dn14))) / (2.0 * assign101220_e153304)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign101220_e153306;
        locals.var_tmf2_dn0 = assign101220_e153306_d_n0;
        locals.var_tmf2_dn2 = assign101220_e153306_d_n2;
        locals.var_tmf2_dn4 = assign101220_e153306_d_n4;
        locals.var_tmf2_dn5 = assign101220_e153306_d_n5;
        locals.var_tmf2_dn6 = assign101220_e153306_d_n6;
        locals.var_tmf2_dn7 = assign101220_e153306_d_n7;
        locals.var_tmf2_dn8 = assign101220_e153306_d_n8;
        locals.var_tmf2_dn9 = assign101220_e153306_d_n9;
        locals.var_tmf2_dn10 = assign101220_e153306_d_n10;
        locals.var_tmf2_dn11 = assign101220_e153306_d_n11;
        locals.var_tmf2_dn14 = assign101220_e153306_d_n14;

        let (assign101230_e153318, assign101230_e153318_d_n0, assign101230_e153318_d_n2, assign101230_e153318_d_n4, assign101230_e153318_d_n5, assign101230_e153318_d_n6, assign101230_e153318_d_n7, assign101230_e153318_d_n8, assign101230_e153318_d_n9, assign101230_e153318_d_n10, assign101230_e153318_d_n11, assign101230_e153318_d_n14,) = {
    if (locals.var_guard2324 != 0.0) {
        let assign101230_e153312: f64 = (locals.var_ps0 - locals.var_vbscl__blk437);
        let assign101230_e153314: f64 = (assign101230_e153312 / locals.var_tmf2);
        let assign101230_e153315: f64 = (1.0 + assign101230_e153314);
        let assign101230_e153316: f64 = (0.5 * assign101230_e153315);
        (assign101230_e153316, (0.5 * ((((locals.var_ps0_dn0 - locals.var_vbscl__blk437_dn0) * locals.var_tmf2) - (assign101230_e153312 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn2 - locals.var_vbscl__blk437_dn2) * locals.var_tmf2) - (assign101230_e153312 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn4 - locals.var_vbscl__blk437_dn4) * locals.var_tmf2) - (assign101230_e153312 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn5 - locals.var_vbscl__blk437_dn5) * locals.var_tmf2) - (assign101230_e153312 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn6 - locals.var_vbscl__blk437_dn6) * locals.var_tmf2) - (assign101230_e153312 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn7 - locals.var_vbscl__blk437_dn7) * locals.var_tmf2) - (assign101230_e153312 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn8 - locals.var_vbscl__blk437_dn8) * locals.var_tmf2) - (assign101230_e153312 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn9 - locals.var_vbscl__blk437_dn9) * locals.var_tmf2) - (assign101230_e153312 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn10 - locals.var_vbscl__blk437_dn10) * locals.var_tmf2) - (assign101230_e153312 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn11 - locals.var_vbscl__blk437_dn11) * locals.var_tmf2) - (assign101230_e153312 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn14 - locals.var_vbscl__blk437_dn14) * locals.var_tmf2) - (assign101230_e153312 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign101230_e153318;
        locals.var_t0_dn0 = assign101230_e153318_d_n0;
        locals.var_t0_dn2 = assign101230_e153318_d_n2;
        locals.var_t0_dn4 = assign101230_e153318_d_n4;
        locals.var_t0_dn5 = assign101230_e153318_d_n5;
        locals.var_t0_dn6 = assign101230_e153318_d_n6;
        locals.var_t0_dn7 = assign101230_e153318_d_n7;
        locals.var_t0_dn8 = assign101230_e153318_d_n8;
        locals.var_t0_dn9 = assign101230_e153318_d_n9;
        locals.var_t0_dn10 = assign101230_e153318_d_n10;
        locals.var_t0_dn11 = assign101230_e153318_d_n11;
        locals.var_t0_dn14 = assign101230_e153318_d_n14;

        let (assign101240_e153328, assign101240_e153328_d_n0, assign101240_e153328_d_n2, assign101240_e153328_d_n4, assign101240_e153328_d_n5, assign101240_e153328_d_n6, assign101240_e153328_d_n7, assign101240_e153328_d_n8, assign101240_e153328_d_n9, assign101240_e153328_d_n10, assign101240_e153328_d_n11, assign101240_e153328_d_n14,) = {
    if (locals.var_guard2324 != 0.0) {
        let assign101240_e153323: f64 = (locals.var_ps0 - locals.var_vbscl__blk437);
        let assign101240_e153325: f64 = (assign101240_e153323 + locals.var_tmf2);
        let assign101240_e153326: f64 = (0.5 * assign101240_e153325);
        (assign101240_e153326, (0.5 * ((locals.var_ps0_dn0 - locals.var_vbscl__blk437_dn0) + locals.var_tmf2_dn0)), (0.5 * ((locals.var_ps0_dn2 - locals.var_vbscl__blk437_dn2) + locals.var_tmf2_dn2)), (0.5 * ((locals.var_ps0_dn4 - locals.var_vbscl__blk437_dn4) + locals.var_tmf2_dn4)), (0.5 * ((locals.var_ps0_dn5 - locals.var_vbscl__blk437_dn5) + locals.var_tmf2_dn5)), (0.5 * ((locals.var_ps0_dn6 - locals.var_vbscl__blk437_dn6) + locals.var_tmf2_dn6)), (0.5 * ((locals.var_ps0_dn7 - locals.var_vbscl__blk437_dn7) + locals.var_tmf2_dn7)), (0.5 * ((locals.var_ps0_dn8 - locals.var_vbscl__blk437_dn8) + locals.var_tmf2_dn8)), (0.5 * ((locals.var_ps0_dn9 - locals.var_vbscl__blk437_dn9) + locals.var_tmf2_dn9)), (0.5 * ((locals.var_ps0_dn10 - locals.var_vbscl__blk437_dn10) + locals.var_tmf2_dn10)), (0.5 * ((locals.var_ps0_dn11 - locals.var_vbscl__blk437_dn11) + locals.var_tmf2_dn11)), (0.5 * ((locals.var_ps0_dn14 - locals.var_vbscl__blk437_dn14) + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign101240_e153328;
        locals.var_t5_dn0 = assign101240_e153328_d_n0;
        locals.var_t5_dn2 = assign101240_e153328_d_n2;
        locals.var_t5_dn4 = assign101240_e153328_d_n4;
        locals.var_t5_dn5 = assign101240_e153328_d_n5;
        locals.var_t5_dn6 = assign101240_e153328_d_n6;
        locals.var_t5_dn7 = assign101240_e153328_d_n7;
        locals.var_t5_dn8 = assign101240_e153328_d_n8;
        locals.var_t5_dn9 = assign101240_e153328_d_n9;
        locals.var_t5_dn10 = assign101240_e153328_d_n10;
        locals.var_t5_dn11 = assign101240_e153328_d_n11;
        locals.var_t5_dn14 = assign101240_e153328_d_n14;

        let assign101250_e153331: f64 = if locals.var_t5 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2325 = assign101250_e153331;

        let (assign101260_e153337, assign101260_e153337_d_n0, assign101260_e153337_d_n2, assign101260_e153337_d_n4, assign101260_e153337_d_n5, assign101260_e153337_d_n6, assign101260_e153337_d_n7, assign101260_e153337_d_n8, assign101260_e153337_d_n9, assign101260_e153337_d_n10, assign101260_e153337_d_n11, assign101260_e153337_d_n14,) = {
    if ((locals.var_guard2324 != 0.0) && (locals.var_guard2325 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign101260_e153337;
        locals.var_t5_dn0 = assign101260_e153337_d_n0;
        locals.var_t5_dn2 = assign101260_e153337_d_n2;
        locals.var_t5_dn4 = assign101260_e153337_d_n4;
        locals.var_t5_dn5 = assign101260_e153337_d_n5;
        locals.var_t5_dn6 = assign101260_e153337_d_n6;
        locals.var_t5_dn7 = assign101260_e153337_d_n7;
        locals.var_t5_dn8 = assign101260_e153337_d_n8;
        locals.var_t5_dn9 = assign101260_e153337_d_n9;
        locals.var_t5_dn10 = assign101260_e153337_d_n10;
        locals.var_t5_dn11 = assign101260_e153337_d_n11;
        locals.var_t5_dn14 = assign101260_e153337_d_n14;

        let (assign101270_e153343, assign101270_e153343_d_n0, assign101270_e153343_d_n2, assign101270_e153343_d_n4, assign101270_e153343_d_n5, assign101270_e153343_d_n6, assign101270_e153343_d_n7, assign101270_e153343_d_n8, assign101270_e153343_d_n9, assign101270_e153343_d_n10, assign101270_e153343_d_n11, assign101270_e153343_d_n14,) = {
    if ((locals.var_guard2324 != 0.0) && (locals.var_guard2325 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign101270_e153343;
        locals.var_t0_dn0 = assign101270_e153343_d_n0;
        locals.var_t0_dn2 = assign101270_e153343_d_n2;
        locals.var_t0_dn4 = assign101270_e153343_d_n4;
        locals.var_t0_dn5 = assign101270_e153343_d_n5;
        locals.var_t0_dn6 = assign101270_e153343_d_n6;
        locals.var_t0_dn7 = assign101270_e153343_d_n7;
        locals.var_t0_dn8 = assign101270_e153343_d_n8;
        locals.var_t0_dn9 = assign101270_e153343_d_n9;
        locals.var_t0_dn10 = assign101270_e153343_d_n10;
        locals.var_t0_dn11 = assign101270_e153343_d_n11;
        locals.var_t0_dn14 = assign101270_e153343_d_n14;

        let (assign101280_e153357, assign101280_e153357_d_n0, assign101280_e153357_d_n2, assign101280_e153357_d_n4, assign101280_e153357_d_n5, assign101280_e153357_d_n6, assign101280_e153357_d_n7, assign101280_e153357_d_n8, assign101280_e153357_d_n9, assign101280_e153357_d_n10, assign101280_e153357_d_n11, assign101280_e153357_d_n14,) = {
    if (locals.var_guard2324 != 0.0) {
        let assign101280_e153348: f64 = (locals.var_qn0 / locals.var_t5);
        let assign101280_e153349: f64 = (locals.var_cox + assign101280_e153348);
        let assign101280_e153351: f64 = (assign101280_e153349 + locals.var_cite);
        let assign101280_e153353: f64 = (assign101280_e153351 * locals.var_beta_inv);
        let assign101280_e153355: f64 = (assign101280_e153353 / 1.6021918e-19);
        (assign101280_e153355, ((((locals.var_cox_dn0 + (((locals.var_qn0_dn0 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn0)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101280_e153351 * locals.var_beta_inv_dn0)) / 1.6021918e-19), ((((locals.var_cox_dn2 + (((locals.var_qn0_dn2 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn2)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101280_e153351 * locals.var_beta_inv_dn2)) / 1.6021918e-19), ((((locals.var_cox_dn4 + (((locals.var_qn0_dn4 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101280_e153351 * locals.var_beta_inv_dn4)) / 1.6021918e-19), ((((locals.var_cox_dn5 + (((locals.var_qn0_dn5 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101280_e153351 * locals.var_beta_inv_dn5)) / 1.6021918e-19), ((((locals.var_cox_dn6 + (((locals.var_qn0_dn6 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101280_e153351 * locals.var_beta_inv_dn6)) / 1.6021918e-19), ((((locals.var_cox_dn7 + (((locals.var_qn0_dn7 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101280_e153351 * locals.var_beta_inv_dn7)) / 1.6021918e-19), ((((locals.var_cox_dn8 + (((locals.var_qn0_dn8 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101280_e153351 * locals.var_beta_inv_dn8)) / 1.6021918e-19), ((((locals.var_cox_dn9 + (((locals.var_qn0_dn9 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101280_e153351 * locals.var_beta_inv_dn9)) / 1.6021918e-19), ((((locals.var_cox_dn10 + (((locals.var_qn0_dn10 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101280_e153351 * locals.var_beta_inv_dn10)) / 1.6021918e-19), ((((locals.var_cox_dn11 + (((locals.var_qn0_dn11 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101280_e153351 * locals.var_beta_inv_dn11)) / 1.6021918e-19), ((((locals.var_cox_dn14 + (((locals.var_qn0_dn14 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn14)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101280_e153351 * locals.var_beta_inv_dn14)) / 1.6021918e-19),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign101280_e153357;
        locals.var_t2_dn0 = assign101280_e153357_d_n0;
        locals.var_t2_dn2 = assign101280_e153357_d_n2;
        locals.var_t2_dn4 = assign101280_e153357_d_n4;
        locals.var_t2_dn5 = assign101280_e153357_d_n5;
        locals.var_t2_dn6 = assign101280_e153357_d_n6;
        locals.var_t2_dn7 = assign101280_e153357_d_n7;
        locals.var_t2_dn8 = assign101280_e153357_d_n8;
        locals.var_t2_dn9 = assign101280_e153357_d_n9;
        locals.var_t2_dn10 = assign101280_e153357_d_n10;
        locals.var_t2_dn11 = assign101280_e153357_d_n11;
        locals.var_t2_dn14 = assign101280_e153357_d_n14;

        let (assign101290_e153372, assign101290_e153372_d_n0, assign101290_e153372_d_n2, assign101290_e153372_d_n4, assign101290_e153372_d_n5, assign101290_e153372_d_n6, assign101290_e153372_d_n7, assign101290_e153372_d_n8, assign101290_e153372_d_n9, assign101290_e153372_d_n10, assign101290_e153372_d_n11, assign101290_e153372_d_n14,) = {
    if (locals.var_guard2324 != 0.0) {
        let assign101290_e153360: f64 = (-2.0);
        let assign101290_e153362: f64 = (assign101290_e153360 * locals.var_qi_noi);
        let assign101290_e153364: f64 = (assign101290_e153362 / 1.6021918e-19);
        let assign101290_e153366: f64 = (assign101290_e153364 / locals.var_lch);
        let assign101290_e153368: f64 = (assign101290_e153366 / locals.var_weffcv_nf);
        let assign101290_e153370: f64 = (assign101290_e153368 - locals.var_t1);
        (assign101290_e153370, (((((((assign101290_e153360 * locals.var_qi_noi_dn0) / 1.6021918e-19) * locals.var_lch) - (assign101290_e153364 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn0), (((((((assign101290_e153360 * locals.var_qi_noi_dn2) / 1.6021918e-19) * locals.var_lch) - (assign101290_e153364 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn2), (((((((assign101290_e153360 * locals.var_qi_noi_dn4) / 1.6021918e-19) * locals.var_lch) - (assign101290_e153364 * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn4), (((((((assign101290_e153360 * locals.var_qi_noi_dn5) / 1.6021918e-19) * locals.var_lch) - (assign101290_e153364 * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn5), (((((((assign101290_e153360 * locals.var_qi_noi_dn6) / 1.6021918e-19) * locals.var_lch) - (assign101290_e153364 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn6), (((((((assign101290_e153360 * locals.var_qi_noi_dn7) / 1.6021918e-19) * locals.var_lch) - (assign101290_e153364 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn7), (((((((assign101290_e153360 * locals.var_qi_noi_dn8) / 1.6021918e-19) * locals.var_lch) - (assign101290_e153364 * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn8), (((((((assign101290_e153360 * locals.var_qi_noi_dn9) / 1.6021918e-19) * locals.var_lch) - (assign101290_e153364 * locals.var_lch_dn9)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn9), (((((((assign101290_e153360 * locals.var_qi_noi_dn10) / 1.6021918e-19) * locals.var_lch) - (assign101290_e153364 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn10), (((((((assign101290_e153360 * locals.var_qi_noi_dn11) / 1.6021918e-19) * locals.var_lch) - (assign101290_e153364 * locals.var_lch_dn11)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn11), (((((((assign101290_e153360 * locals.var_qi_noi_dn14) / 1.6021918e-19) * locals.var_lch) - (assign101290_e153364 * locals.var_lch_dn14)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign101290_e153372;
        locals.var_t3_dn0 = assign101290_e153372_d_n0;
        locals.var_t3_dn2 = assign101290_e153372_d_n2;
        locals.var_t3_dn4 = assign101290_e153372_d_n4;
        locals.var_t3_dn5 = assign101290_e153372_d_n5;
        locals.var_t3_dn6 = assign101290_e153372_d_n6;
        locals.var_t3_dn7 = assign101290_e153372_d_n7;
        locals.var_t3_dn8 = assign101290_e153372_d_n8;
        locals.var_t3_dn9 = assign101290_e153372_d_n9;
        locals.var_t3_dn10 = assign101290_e153372_d_n10;
        locals.var_t3_dn11 = assign101290_e153372_d_n11;
        locals.var_t3_dn14 = assign101290_e153372_d_n14;

        let assign101300_e153375: f64 = (locals.var_t3 - locals.var_t1);
        let assign101300_e153376: f64 = (assign101300_e153375).abs();
        let assign101300_e153379: f64 = (10.0 * 2.220446049250313e-16);
        let assign101300_e153380: f64 = if assign101300_e153376 > assign101300_e153379 { 1.0 } else { 0.0 };
        locals.var_guard2326 = assign101300_e153380;

        let (assign101310_e153427, assign101310_e153427_d_n0, assign101310_e153427_d_n2, assign101310_e153427_d_n4, assign101310_e153427_d_n5, assign101310_e153427_d_n6, assign101310_e153427_d_n7, assign101310_e153427_d_n8, assign101310_e153427_d_n9, assign101310_e153427_d_n10, assign101310_e153427_d_n11, assign101310_e153427_d_n14,) = {
    if ((locals.var_guard2324 != 0.0) && (locals.var_guard2326 != 0.0)) {
        let assign101310_e153387: f64 = (locals.var_t1 + locals.var_t2);
        let assign101310_e153388: f64 = (1.0 / assign101310_e153387);
        let assign101310_e153391: f64 = (locals.var_t3 + locals.var_t2);
        let assign101310_e153392: f64 = (assign101310_e153388 / assign101310_e153391);
        let assign101310_e153395: f64 = (2.0 * locals.var_nfalpe);
        let assign101310_e153397: f64 = (assign101310_e153395 * locals.var_ey);
        let assign101310_e153399: f64 = (assign101310_e153397 * locals.var_mu);
        let assign101310_e153402: f64 = (locals.var_t3 - locals.var_t1);
        let assign101310_e153403: f64 = (assign101310_e153399 / assign101310_e153402);
        let assign101310_e153406: f64 = (locals.var_t3 + locals.var_t2);
        let assign101310_e153409: f64 = (locals.var_t1 + locals.var_t2);
        let assign101310_e153410: f64 = (assign101310_e153406 / assign101310_e153409);
        let assign101310_e153411: f64 = (assign101310_e153410).ln();
        let assign101310_e153412: f64 = (assign101310_e153403 * assign101310_e153411);
        let assign101310_e153413: f64 = (assign101310_e153392 + assign101310_e153412);
        let assign101310_e153416: f64 = (locals.var_nfalpe * locals.var_ey);
        let assign101310_e153418: f64 = (assign101310_e153416 * locals.var_mu);
        let assign101310_e153420: f64 = (assign101310_e153418 * locals.var_nfalpe);
        let assign101310_e153422: f64 = (assign101310_e153420 * locals.var_ey);
        let assign101310_e153424: f64 = (assign101310_e153422 * locals.var_mu);
        let assign101310_e153425: f64 = (assign101310_e153413 + assign101310_e153424);
        (assign101310_e153425, ((((((-((locals.var_t1_dn0 + locals.var_t2_dn0) / (assign101310_e153387 * assign101310_e153387))) * assign101310_e153391) - (assign101310_e153388 * (locals.var_t3_dn0 + locals.var_t2_dn0))) / (assign101310_e153391 * assign101310_e153391)) + ((((((((assign101310_e153395 * locals.var_ey_dn0) * locals.var_mu) + (assign101310_e153397 * locals.var_mu_dn0)) * assign101310_e153402) - (assign101310_e153399 * (locals.var_t3_dn0 - locals.var_t1_dn0))) / (assign101310_e153402 * assign101310_e153402)) * assign101310_e153411) + (assign101310_e153403 * (((((locals.var_t3_dn0 + locals.var_t2_dn0) * assign101310_e153409) - (assign101310_e153406 * (locals.var_t1_dn0 + locals.var_t2_dn0))) / (assign101310_e153409 * assign101310_e153409)) / assign101310_e153410)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn0) * locals.var_mu) + (assign101310_e153416 * locals.var_mu_dn0)) * locals.var_nfalpe) * locals.var_ey) + (assign101310_e153420 * locals.var_ey_dn0)) * locals.var_mu) + (assign101310_e153422 * locals.var_mu_dn0))), ((((((-((locals.var_t1_dn2 + locals.var_t2_dn2) / (assign101310_e153387 * assign101310_e153387))) * assign101310_e153391) - (assign101310_e153388 * (locals.var_t3_dn2 + locals.var_t2_dn2))) / (assign101310_e153391 * assign101310_e153391)) + ((((((((assign101310_e153395 * locals.var_ey_dn2) * locals.var_mu) + (assign101310_e153397 * locals.var_mu_dn2)) * assign101310_e153402) - (assign101310_e153399 * (locals.var_t3_dn2 - locals.var_t1_dn2))) / (assign101310_e153402 * assign101310_e153402)) * assign101310_e153411) + (assign101310_e153403 * (((((locals.var_t3_dn2 + locals.var_t2_dn2) * assign101310_e153409) - (assign101310_e153406 * (locals.var_t1_dn2 + locals.var_t2_dn2))) / (assign101310_e153409 * assign101310_e153409)) / assign101310_e153410)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn2) * locals.var_mu) + (assign101310_e153416 * locals.var_mu_dn2)) * locals.var_nfalpe) * locals.var_ey) + (assign101310_e153420 * locals.var_ey_dn2)) * locals.var_mu) + (assign101310_e153422 * locals.var_mu_dn2))), ((((((-((locals.var_t1_dn4 + locals.var_t2_dn4) / (assign101310_e153387 * assign101310_e153387))) * assign101310_e153391) - (assign101310_e153388 * (locals.var_t3_dn4 + locals.var_t2_dn4))) / (assign101310_e153391 * assign101310_e153391)) + ((((((((assign101310_e153395 * locals.var_ey_dn4) * locals.var_mu) + (assign101310_e153397 * locals.var_mu_dn4)) * assign101310_e153402) - (assign101310_e153399 * (locals.var_t3_dn4 - locals.var_t1_dn4))) / (assign101310_e153402 * assign101310_e153402)) * assign101310_e153411) + (assign101310_e153403 * (((((locals.var_t3_dn4 + locals.var_t2_dn4) * assign101310_e153409) - (assign101310_e153406 * (locals.var_t1_dn4 + locals.var_t2_dn4))) / (assign101310_e153409 * assign101310_e153409)) / assign101310_e153410)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn4) * locals.var_mu) + (assign101310_e153416 * locals.var_mu_dn4)) * locals.var_nfalpe) * locals.var_ey) + (assign101310_e153420 * locals.var_ey_dn4)) * locals.var_mu) + (assign101310_e153422 * locals.var_mu_dn4))), ((((((-((locals.var_t1_dn5 + locals.var_t2_dn5) / (assign101310_e153387 * assign101310_e153387))) * assign101310_e153391) - (assign101310_e153388 * (locals.var_t3_dn5 + locals.var_t2_dn5))) / (assign101310_e153391 * assign101310_e153391)) + ((((((((assign101310_e153395 * locals.var_ey_dn5) * locals.var_mu) + (assign101310_e153397 * locals.var_mu_dn5)) * assign101310_e153402) - (assign101310_e153399 * (locals.var_t3_dn5 - locals.var_t1_dn5))) / (assign101310_e153402 * assign101310_e153402)) * assign101310_e153411) + (assign101310_e153403 * (((((locals.var_t3_dn5 + locals.var_t2_dn5) * assign101310_e153409) - (assign101310_e153406 * (locals.var_t1_dn5 + locals.var_t2_dn5))) / (assign101310_e153409 * assign101310_e153409)) / assign101310_e153410)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn5) * locals.var_mu) + (assign101310_e153416 * locals.var_mu_dn5)) * locals.var_nfalpe) * locals.var_ey) + (assign101310_e153420 * locals.var_ey_dn5)) * locals.var_mu) + (assign101310_e153422 * locals.var_mu_dn5))), ((((((-((locals.var_t1_dn6 + locals.var_t2_dn6) / (assign101310_e153387 * assign101310_e153387))) * assign101310_e153391) - (assign101310_e153388 * (locals.var_t3_dn6 + locals.var_t2_dn6))) / (assign101310_e153391 * assign101310_e153391)) + ((((((((assign101310_e153395 * locals.var_ey_dn6) * locals.var_mu) + (assign101310_e153397 * locals.var_mu_dn6)) * assign101310_e153402) - (assign101310_e153399 * (locals.var_t3_dn6 - locals.var_t1_dn6))) / (assign101310_e153402 * assign101310_e153402)) * assign101310_e153411) + (assign101310_e153403 * (((((locals.var_t3_dn6 + locals.var_t2_dn6) * assign101310_e153409) - (assign101310_e153406 * (locals.var_t1_dn6 + locals.var_t2_dn6))) / (assign101310_e153409 * assign101310_e153409)) / assign101310_e153410)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn6) * locals.var_mu) + (assign101310_e153416 * locals.var_mu_dn6)) * locals.var_nfalpe) * locals.var_ey) + (assign101310_e153420 * locals.var_ey_dn6)) * locals.var_mu) + (assign101310_e153422 * locals.var_mu_dn6))), ((((((-((locals.var_t1_dn7 + locals.var_t2_dn7) / (assign101310_e153387 * assign101310_e153387))) * assign101310_e153391) - (assign101310_e153388 * (locals.var_t3_dn7 + locals.var_t2_dn7))) / (assign101310_e153391 * assign101310_e153391)) + ((((((((assign101310_e153395 * locals.var_ey_dn7) * locals.var_mu) + (assign101310_e153397 * locals.var_mu_dn7)) * assign101310_e153402) - (assign101310_e153399 * (locals.var_t3_dn7 - locals.var_t1_dn7))) / (assign101310_e153402 * assign101310_e153402)) * assign101310_e153411) + (assign101310_e153403 * (((((locals.var_t3_dn7 + locals.var_t2_dn7) * assign101310_e153409) - (assign101310_e153406 * (locals.var_t1_dn7 + locals.var_t2_dn7))) / (assign101310_e153409 * assign101310_e153409)) / assign101310_e153410)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn7) * locals.var_mu) + (assign101310_e153416 * locals.var_mu_dn7)) * locals.var_nfalpe) * locals.var_ey) + (assign101310_e153420 * locals.var_ey_dn7)) * locals.var_mu) + (assign101310_e153422 * locals.var_mu_dn7))), ((((((-((locals.var_t1_dn8 + locals.var_t2_dn8) / (assign101310_e153387 * assign101310_e153387))) * assign101310_e153391) - (assign101310_e153388 * (locals.var_t3_dn8 + locals.var_t2_dn8))) / (assign101310_e153391 * assign101310_e153391)) + ((((((((assign101310_e153395 * locals.var_ey_dn8) * locals.var_mu) + (assign101310_e153397 * locals.var_mu_dn8)) * assign101310_e153402) - (assign101310_e153399 * (locals.var_t3_dn8 - locals.var_t1_dn8))) / (assign101310_e153402 * assign101310_e153402)) * assign101310_e153411) + (assign101310_e153403 * (((((locals.var_t3_dn8 + locals.var_t2_dn8) * assign101310_e153409) - (assign101310_e153406 * (locals.var_t1_dn8 + locals.var_t2_dn8))) / (assign101310_e153409 * assign101310_e153409)) / assign101310_e153410)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn8) * locals.var_mu) + (assign101310_e153416 * locals.var_mu_dn8)) * locals.var_nfalpe) * locals.var_ey) + (assign101310_e153420 * locals.var_ey_dn8)) * locals.var_mu) + (assign101310_e153422 * locals.var_mu_dn8))), ((((((-((locals.var_t1_dn9 + locals.var_t2_dn9) / (assign101310_e153387 * assign101310_e153387))) * assign101310_e153391) - (assign101310_e153388 * (locals.var_t3_dn9 + locals.var_t2_dn9))) / (assign101310_e153391 * assign101310_e153391)) + ((((((((assign101310_e153395 * locals.var_ey_dn9) * locals.var_mu) + (assign101310_e153397 * locals.var_mu_dn9)) * assign101310_e153402) - (assign101310_e153399 * (locals.var_t3_dn9 - locals.var_t1_dn9))) / (assign101310_e153402 * assign101310_e153402)) * assign101310_e153411) + (assign101310_e153403 * (((((locals.var_t3_dn9 + locals.var_t2_dn9) * assign101310_e153409) - (assign101310_e153406 * (locals.var_t1_dn9 + locals.var_t2_dn9))) / (assign101310_e153409 * assign101310_e153409)) / assign101310_e153410)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn9) * locals.var_mu) + (assign101310_e153416 * locals.var_mu_dn9)) * locals.var_nfalpe) * locals.var_ey) + (assign101310_e153420 * locals.var_ey_dn9)) * locals.var_mu) + (assign101310_e153422 * locals.var_mu_dn9))), ((((((-((locals.var_t1_dn10 + locals.var_t2_dn10) / (assign101310_e153387 * assign101310_e153387))) * assign101310_e153391) - (assign101310_e153388 * (locals.var_t3_dn10 + locals.var_t2_dn10))) / (assign101310_e153391 * assign101310_e153391)) + ((((((((assign101310_e153395 * locals.var_ey_dn10) * locals.var_mu) + (assign101310_e153397 * locals.var_mu_dn10)) * assign101310_e153402) - (assign101310_e153399 * (locals.var_t3_dn10 - locals.var_t1_dn10))) / (assign101310_e153402 * assign101310_e153402)) * assign101310_e153411) + (assign101310_e153403 * (((((locals.var_t3_dn10 + locals.var_t2_dn10) * assign101310_e153409) - (assign101310_e153406 * (locals.var_t1_dn10 + locals.var_t2_dn10))) / (assign101310_e153409 * assign101310_e153409)) / assign101310_e153410)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn10) * locals.var_mu) + (assign101310_e153416 * locals.var_mu_dn10)) * locals.var_nfalpe) * locals.var_ey) + (assign101310_e153420 * locals.var_ey_dn10)) * locals.var_mu) + (assign101310_e153422 * locals.var_mu_dn10))), ((((((-((locals.var_t1_dn11 + locals.var_t2_dn11) / (assign101310_e153387 * assign101310_e153387))) * assign101310_e153391) - (assign101310_e153388 * (locals.var_t3_dn11 + locals.var_t2_dn11))) / (assign101310_e153391 * assign101310_e153391)) + ((((((((assign101310_e153395 * locals.var_ey_dn11) * locals.var_mu) + (assign101310_e153397 * locals.var_mu_dn11)) * assign101310_e153402) - (assign101310_e153399 * (locals.var_t3_dn11 - locals.var_t1_dn11))) / (assign101310_e153402 * assign101310_e153402)) * assign101310_e153411) + (assign101310_e153403 * (((((locals.var_t3_dn11 + locals.var_t2_dn11) * assign101310_e153409) - (assign101310_e153406 * (locals.var_t1_dn11 + locals.var_t2_dn11))) / (assign101310_e153409 * assign101310_e153409)) / assign101310_e153410)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn11) * locals.var_mu) + (assign101310_e153416 * locals.var_mu_dn11)) * locals.var_nfalpe) * locals.var_ey) + (assign101310_e153420 * locals.var_ey_dn11)) * locals.var_mu) + (assign101310_e153422 * locals.var_mu_dn11))), ((((((-((locals.var_t1_dn14 + locals.var_t2_dn14) / (assign101310_e153387 * assign101310_e153387))) * assign101310_e153391) - (assign101310_e153388 * (locals.var_t3_dn14 + locals.var_t2_dn14))) / (assign101310_e153391 * assign101310_e153391)) + ((((((((assign101310_e153395 * locals.var_ey_dn14) * locals.var_mu) + (assign101310_e153397 * locals.var_mu_dn14)) * assign101310_e153402) - (assign101310_e153399 * (locals.var_t3_dn14 - locals.var_t1_dn14))) / (assign101310_e153402 * assign101310_e153402)) * assign101310_e153411) + (assign101310_e153403 * (((((locals.var_t3_dn14 + locals.var_t2_dn14) * assign101310_e153409) - (assign101310_e153406 * (locals.var_t1_dn14 + locals.var_t2_dn14))) / (assign101310_e153409 * assign101310_e153409)) / assign101310_e153410)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn14) * locals.var_mu) + (assign101310_e153416 * locals.var_mu_dn14)) * locals.var_nfalpe) * locals.var_ey) + (assign101310_e153420 * locals.var_ey_dn14)) * locals.var_mu) + (assign101310_e153422 * locals.var_mu_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign101310_e153427;
        locals.var_t4_dn0 = assign101310_e153427_d_n0;
        locals.var_t4_dn2 = assign101310_e153427_d_n2;
        locals.var_t4_dn4 = assign101310_e153427_d_n4;
        locals.var_t4_dn5 = assign101310_e153427_d_n5;
        locals.var_t4_dn6 = assign101310_e153427_d_n6;
        locals.var_t4_dn7 = assign101310_e153427_d_n7;
        locals.var_t4_dn8 = assign101310_e153427_d_n8;
        locals.var_t4_dn9 = assign101310_e153427_d_n9;
        locals.var_t4_dn10 = assign101310_e153427_d_n10;
        locals.var_t4_dn11 = assign101310_e153427_d_n11;
        locals.var_t4_dn14 = assign101310_e153427_d_n14;

    }

    pub(super) fn stamp_transient_block_371(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign101320_e153466, assign101320_e153466_d_n0, assign101320_e153466_d_n2, assign101320_e153466_d_n4, assign101320_e153466_d_n5, assign101320_e153466_d_n6, assign101320_e153466_d_n7, assign101320_e153466_d_n8, assign101320_e153466_d_n9, assign101320_e153466_d_n10, assign101320_e153466_d_n11, assign101320_e153466_d_n14,) = {
    if ((locals.var_guard2324 != 0.0) && (locals.var_guard2326 == 0.0)) {
        let assign101320_e153435: f64 = (locals.var_t1 + locals.var_t2);
        let assign101320_e153436: f64 = (1.0 / assign101320_e153435);
        let assign101320_e153439: f64 = (locals.var_t3 + locals.var_t2);
        let assign101320_e153440: f64 = (assign101320_e153436 / assign101320_e153439);
        let assign101320_e153443: f64 = (2.0 * locals.var_nfalpe);
        let assign101320_e153445: f64 = (assign101320_e153443 * locals.var_ey);
        let assign101320_e153447: f64 = (assign101320_e153445 * locals.var_mu);
        let assign101320_e153450: f64 = (locals.var_t1 + locals.var_t2);
        let assign101320_e153451: f64 = (assign101320_e153447 / assign101320_e153450);
        let assign101320_e153452: f64 = (assign101320_e153440 + assign101320_e153451);
        let assign101320_e153455: f64 = (locals.var_nfalpe * locals.var_ey);
        let assign101320_e153457: f64 = (assign101320_e153455 * locals.var_mu);
        let assign101320_e153459: f64 = (assign101320_e153457 * locals.var_nfalpe);
        let assign101320_e153461: f64 = (assign101320_e153459 * locals.var_ey);
        let assign101320_e153463: f64 = (assign101320_e153461 * locals.var_mu);
        let assign101320_e153464: f64 = (assign101320_e153452 + assign101320_e153463);
        (assign101320_e153464, ((((((-((locals.var_t1_dn0 + locals.var_t2_dn0) / (assign101320_e153435 * assign101320_e153435))) * assign101320_e153439) - (assign101320_e153436 * (locals.var_t3_dn0 + locals.var_t2_dn0))) / (assign101320_e153439 * assign101320_e153439)) + ((((((assign101320_e153443 * locals.var_ey_dn0) * locals.var_mu) + (assign101320_e153445 * locals.var_mu_dn0)) * assign101320_e153450) - (assign101320_e153447 * (locals.var_t1_dn0 + locals.var_t2_dn0))) / (assign101320_e153450 * assign101320_e153450))) + ((((((((locals.var_nfalpe * locals.var_ey_dn0) * locals.var_mu) + (assign101320_e153455 * locals.var_mu_dn0)) * locals.var_nfalpe) * locals.var_ey) + (assign101320_e153459 * locals.var_ey_dn0)) * locals.var_mu) + (assign101320_e153461 * locals.var_mu_dn0))), ((((((-((locals.var_t1_dn2 + locals.var_t2_dn2) / (assign101320_e153435 * assign101320_e153435))) * assign101320_e153439) - (assign101320_e153436 * (locals.var_t3_dn2 + locals.var_t2_dn2))) / (assign101320_e153439 * assign101320_e153439)) + ((((((assign101320_e153443 * locals.var_ey_dn2) * locals.var_mu) + (assign101320_e153445 * locals.var_mu_dn2)) * assign101320_e153450) - (assign101320_e153447 * (locals.var_t1_dn2 + locals.var_t2_dn2))) / (assign101320_e153450 * assign101320_e153450))) + ((((((((locals.var_nfalpe * locals.var_ey_dn2) * locals.var_mu) + (assign101320_e153455 * locals.var_mu_dn2)) * locals.var_nfalpe) * locals.var_ey) + (assign101320_e153459 * locals.var_ey_dn2)) * locals.var_mu) + (assign101320_e153461 * locals.var_mu_dn2))), ((((((-((locals.var_t1_dn4 + locals.var_t2_dn4) / (assign101320_e153435 * assign101320_e153435))) * assign101320_e153439) - (assign101320_e153436 * (locals.var_t3_dn4 + locals.var_t2_dn4))) / (assign101320_e153439 * assign101320_e153439)) + ((((((assign101320_e153443 * locals.var_ey_dn4) * locals.var_mu) + (assign101320_e153445 * locals.var_mu_dn4)) * assign101320_e153450) - (assign101320_e153447 * (locals.var_t1_dn4 + locals.var_t2_dn4))) / (assign101320_e153450 * assign101320_e153450))) + ((((((((locals.var_nfalpe * locals.var_ey_dn4) * locals.var_mu) + (assign101320_e153455 * locals.var_mu_dn4)) * locals.var_nfalpe) * locals.var_ey) + (assign101320_e153459 * locals.var_ey_dn4)) * locals.var_mu) + (assign101320_e153461 * locals.var_mu_dn4))), ((((((-((locals.var_t1_dn5 + locals.var_t2_dn5) / (assign101320_e153435 * assign101320_e153435))) * assign101320_e153439) - (assign101320_e153436 * (locals.var_t3_dn5 + locals.var_t2_dn5))) / (assign101320_e153439 * assign101320_e153439)) + ((((((assign101320_e153443 * locals.var_ey_dn5) * locals.var_mu) + (assign101320_e153445 * locals.var_mu_dn5)) * assign101320_e153450) - (assign101320_e153447 * (locals.var_t1_dn5 + locals.var_t2_dn5))) / (assign101320_e153450 * assign101320_e153450))) + ((((((((locals.var_nfalpe * locals.var_ey_dn5) * locals.var_mu) + (assign101320_e153455 * locals.var_mu_dn5)) * locals.var_nfalpe) * locals.var_ey) + (assign101320_e153459 * locals.var_ey_dn5)) * locals.var_mu) + (assign101320_e153461 * locals.var_mu_dn5))), ((((((-((locals.var_t1_dn6 + locals.var_t2_dn6) / (assign101320_e153435 * assign101320_e153435))) * assign101320_e153439) - (assign101320_e153436 * (locals.var_t3_dn6 + locals.var_t2_dn6))) / (assign101320_e153439 * assign101320_e153439)) + ((((((assign101320_e153443 * locals.var_ey_dn6) * locals.var_mu) + (assign101320_e153445 * locals.var_mu_dn6)) * assign101320_e153450) - (assign101320_e153447 * (locals.var_t1_dn6 + locals.var_t2_dn6))) / (assign101320_e153450 * assign101320_e153450))) + ((((((((locals.var_nfalpe * locals.var_ey_dn6) * locals.var_mu) + (assign101320_e153455 * locals.var_mu_dn6)) * locals.var_nfalpe) * locals.var_ey) + (assign101320_e153459 * locals.var_ey_dn6)) * locals.var_mu) + (assign101320_e153461 * locals.var_mu_dn6))), ((((((-((locals.var_t1_dn7 + locals.var_t2_dn7) / (assign101320_e153435 * assign101320_e153435))) * assign101320_e153439) - (assign101320_e153436 * (locals.var_t3_dn7 + locals.var_t2_dn7))) / (assign101320_e153439 * assign101320_e153439)) + ((((((assign101320_e153443 * locals.var_ey_dn7) * locals.var_mu) + (assign101320_e153445 * locals.var_mu_dn7)) * assign101320_e153450) - (assign101320_e153447 * (locals.var_t1_dn7 + locals.var_t2_dn7))) / (assign101320_e153450 * assign101320_e153450))) + ((((((((locals.var_nfalpe * locals.var_ey_dn7) * locals.var_mu) + (assign101320_e153455 * locals.var_mu_dn7)) * locals.var_nfalpe) * locals.var_ey) + (assign101320_e153459 * locals.var_ey_dn7)) * locals.var_mu) + (assign101320_e153461 * locals.var_mu_dn7))), ((((((-((locals.var_t1_dn8 + locals.var_t2_dn8) / (assign101320_e153435 * assign101320_e153435))) * assign101320_e153439) - (assign101320_e153436 * (locals.var_t3_dn8 + locals.var_t2_dn8))) / (assign101320_e153439 * assign101320_e153439)) + ((((((assign101320_e153443 * locals.var_ey_dn8) * locals.var_mu) + (assign101320_e153445 * locals.var_mu_dn8)) * assign101320_e153450) - (assign101320_e153447 * (locals.var_t1_dn8 + locals.var_t2_dn8))) / (assign101320_e153450 * assign101320_e153450))) + ((((((((locals.var_nfalpe * locals.var_ey_dn8) * locals.var_mu) + (assign101320_e153455 * locals.var_mu_dn8)) * locals.var_nfalpe) * locals.var_ey) + (assign101320_e153459 * locals.var_ey_dn8)) * locals.var_mu) + (assign101320_e153461 * locals.var_mu_dn8))), ((((((-((locals.var_t1_dn9 + locals.var_t2_dn9) / (assign101320_e153435 * assign101320_e153435))) * assign101320_e153439) - (assign101320_e153436 * (locals.var_t3_dn9 + locals.var_t2_dn9))) / (assign101320_e153439 * assign101320_e153439)) + ((((((assign101320_e153443 * locals.var_ey_dn9) * locals.var_mu) + (assign101320_e153445 * locals.var_mu_dn9)) * assign101320_e153450) - (assign101320_e153447 * (locals.var_t1_dn9 + locals.var_t2_dn9))) / (assign101320_e153450 * assign101320_e153450))) + ((((((((locals.var_nfalpe * locals.var_ey_dn9) * locals.var_mu) + (assign101320_e153455 * locals.var_mu_dn9)) * locals.var_nfalpe) * locals.var_ey) + (assign101320_e153459 * locals.var_ey_dn9)) * locals.var_mu) + (assign101320_e153461 * locals.var_mu_dn9))), ((((((-((locals.var_t1_dn10 + locals.var_t2_dn10) / (assign101320_e153435 * assign101320_e153435))) * assign101320_e153439) - (assign101320_e153436 * (locals.var_t3_dn10 + locals.var_t2_dn10))) / (assign101320_e153439 * assign101320_e153439)) + ((((((assign101320_e153443 * locals.var_ey_dn10) * locals.var_mu) + (assign101320_e153445 * locals.var_mu_dn10)) * assign101320_e153450) - (assign101320_e153447 * (locals.var_t1_dn10 + locals.var_t2_dn10))) / (assign101320_e153450 * assign101320_e153450))) + ((((((((locals.var_nfalpe * locals.var_ey_dn10) * locals.var_mu) + (assign101320_e153455 * locals.var_mu_dn10)) * locals.var_nfalpe) * locals.var_ey) + (assign101320_e153459 * locals.var_ey_dn10)) * locals.var_mu) + (assign101320_e153461 * locals.var_mu_dn10))), ((((((-((locals.var_t1_dn11 + locals.var_t2_dn11) / (assign101320_e153435 * assign101320_e153435))) * assign101320_e153439) - (assign101320_e153436 * (locals.var_t3_dn11 + locals.var_t2_dn11))) / (assign101320_e153439 * assign101320_e153439)) + ((((((assign101320_e153443 * locals.var_ey_dn11) * locals.var_mu) + (assign101320_e153445 * locals.var_mu_dn11)) * assign101320_e153450) - (assign101320_e153447 * (locals.var_t1_dn11 + locals.var_t2_dn11))) / (assign101320_e153450 * assign101320_e153450))) + ((((((((locals.var_nfalpe * locals.var_ey_dn11) * locals.var_mu) + (assign101320_e153455 * locals.var_mu_dn11)) * locals.var_nfalpe) * locals.var_ey) + (assign101320_e153459 * locals.var_ey_dn11)) * locals.var_mu) + (assign101320_e153461 * locals.var_mu_dn11))), ((((((-((locals.var_t1_dn14 + locals.var_t2_dn14) / (assign101320_e153435 * assign101320_e153435))) * assign101320_e153439) - (assign101320_e153436 * (locals.var_t3_dn14 + locals.var_t2_dn14))) / (assign101320_e153439 * assign101320_e153439)) + ((((((assign101320_e153443 * locals.var_ey_dn14) * locals.var_mu) + (assign101320_e153445 * locals.var_mu_dn14)) * assign101320_e153450) - (assign101320_e153447 * (locals.var_t1_dn14 + locals.var_t2_dn14))) / (assign101320_e153450 * assign101320_e153450))) + ((((((((locals.var_nfalpe * locals.var_ey_dn14) * locals.var_mu) + (assign101320_e153455 * locals.var_mu_dn14)) * locals.var_nfalpe) * locals.var_ey) + (assign101320_e153459 * locals.var_ey_dn14)) * locals.var_mu) + (assign101320_e153461 * locals.var_mu_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign101320_e153466;
        locals.var_t4_dn0 = assign101320_e153466_d_n0;
        locals.var_t4_dn2 = assign101320_e153466_d_n2;
        locals.var_t4_dn4 = assign101320_e153466_d_n4;
        locals.var_t4_dn5 = assign101320_e153466_d_n5;
        locals.var_t4_dn6 = assign101320_e153466_d_n6;
        locals.var_t4_dn7 = assign101320_e153466_d_n7;
        locals.var_t4_dn8 = assign101320_e153466_d_n8;
        locals.var_t4_dn9 = assign101320_e153466_d_n9;
        locals.var_t4_dn10 = assign101320_e153466_d_n10;
        locals.var_t4_dn11 = assign101320_e153466_d_n11;
        locals.var_t4_dn14 = assign101320_e153466_d_n14;

        let assign101350_e153497: f64 = if (((p.p30 != 0.0) && (locals.var_flg_noqi == 0.0)) && (locals.var_uc_codep == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2327 = assign101350_e153497;

        let (assign101360_e153509, assign101360_e153509_d_n0, assign101360_e153509_d_n2, assign101360_e153509_d_n4, assign101360_e153509_d_n5, assign101360_e153509_d_n6, assign101360_e153509_d_n7, assign101360_e153509_d_n8, assign101360_e153509_d_n9, assign101360_e153509_d_n10, assign101360_e153509_d_n11, assign101360_e153509_d_n14,) = {
    if (locals.var_guard2327 != 0.0) {
        let assign101360_e153501: f64 = (locals.var_psdl - locals.var_ps0);
        let assign101360_e153504: f64 = (10.0 * 2.220446049250313e-16);
        let assign101360_e153505: f64 = (assign101360_e153501 + assign101360_e153504);
        let assign101360_e153507: f64 = (assign101360_e153505 / locals.var_lch);
        (assign101360_e153507, ((((locals.var_psdl_dn0 - locals.var_ps0_dn0) * locals.var_lch) - (assign101360_e153505 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn2 - locals.var_ps0_dn2) * locals.var_lch) - (assign101360_e153505 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn4 - locals.var_ps0_dn4) * locals.var_lch) - (assign101360_e153505 * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn5 - locals.var_ps0_dn5) * locals.var_lch) - (assign101360_e153505 * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn6 - locals.var_ps0_dn6) * locals.var_lch) - (assign101360_e153505 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn7 - locals.var_ps0_dn7) * locals.var_lch) - (assign101360_e153505 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn8 - locals.var_ps0_dn8) * locals.var_lch) - (assign101360_e153505 * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn9 - locals.var_ps0_dn9) * locals.var_lch) - (assign101360_e153505 * locals.var_lch_dn9)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn10 - locals.var_ps0_dn10) * locals.var_lch) - (assign101360_e153505 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn11 - locals.var_ps0_dn11) * locals.var_lch) - (assign101360_e153505 * locals.var_lch_dn11)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn14 - locals.var_ps0_dn14) * locals.var_lch) - (assign101360_e153505 * locals.var_lch_dn14)) / (locals.var_lch * locals.var_lch)),)
    } else {
        (locals.var_eyd, locals.var_eyd_dn0, locals.var_eyd_dn2, locals.var_eyd_dn4, locals.var_eyd_dn5, locals.var_eyd_dn6, locals.var_eyd_dn7, locals.var_eyd_dn8, locals.var_eyd_dn9, locals.var_eyd_dn10, locals.var_eyd_dn11, locals.var_eyd_dn14,)
    }
};
        locals.var_eyd = assign101360_e153509;
        locals.var_eyd_dn0 = assign101360_e153509_d_n0;
        locals.var_eyd_dn2 = assign101360_e153509_d_n2;
        locals.var_eyd_dn4 = assign101360_e153509_d_n4;
        locals.var_eyd_dn5 = assign101360_e153509_d_n5;
        locals.var_eyd_dn6 = assign101360_e153509_d_n6;
        locals.var_eyd_dn7 = assign101360_e153509_d_n7;
        locals.var_eyd_dn8 = assign101360_e153509_d_n8;
        locals.var_eyd_dn9 = assign101360_e153509_d_n9;
        locals.var_eyd_dn10 = assign101360_e153509_d_n10;
        locals.var_eyd_dn11 = assign101360_e153509_d_n11;
        locals.var_eyd_dn14 = assign101360_e153509_d_n14;

        let (assign101370_e153518, assign101370_e153518_d_n0, assign101370_e153518_d_n2, assign101370_e153518_d_n4, assign101370_e153518_d_n5, assign101370_e153518_d_n6, assign101370_e153518_d_n7, assign101370_e153518_d_n8, assign101370_e153518_d_n9, assign101370_e153518_d_n10, assign101370_e153518_d_n11, assign101370_e153518_d_n14,) = {
    if (locals.var_guard2327 != 0.0) {
        let (assign101370_e153516, assign101370_e153516_d_n0, assign101370_e153516_d_n2, assign101370_e153516_d_n4, assign101370_e153516_d_n5, assign101370_e153516_d_n6, assign101370_e153516_d_n7, assign101370_e153516_d_n8, assign101370_e153516_d_n9, assign101370_e153516_d_n10, assign101370_e153516_d_n11, assign101370_e153516_d_n14,) = {
            if (locals.var_eyd >= 0.0) {
                (locals.var_eyd, locals.var_eyd_dn0, locals.var_eyd_dn2, locals.var_eyd_dn4, locals.var_eyd_dn5, locals.var_eyd_dn6, locals.var_eyd_dn7, locals.var_eyd_dn8, locals.var_eyd_dn9, locals.var_eyd_dn10, locals.var_eyd_dn11, locals.var_eyd_dn14,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign101370_e153516, assign101370_e153516_d_n0, assign101370_e153516_d_n2, assign101370_e153516_d_n4, assign101370_e153516_d_n5, assign101370_e153516_d_n6, assign101370_e153516_d_n7, assign101370_e153516_d_n8, assign101370_e153516_d_n9, assign101370_e153516_d_n10, assign101370_e153516_d_n11, assign101370_e153516_d_n14,)
    } else {
        (locals.var_eyd, locals.var_eyd_dn0, locals.var_eyd_dn2, locals.var_eyd_dn4, locals.var_eyd_dn5, locals.var_eyd_dn6, locals.var_eyd_dn7, locals.var_eyd_dn8, locals.var_eyd_dn9, locals.var_eyd_dn10, locals.var_eyd_dn11, locals.var_eyd_dn14,)
    }
};
        locals.var_eyd = assign101370_e153518;
        locals.var_eyd_dn0 = assign101370_e153518_d_n0;
        locals.var_eyd_dn2 = assign101370_e153518_d_n2;
        locals.var_eyd_dn4 = assign101370_e153518_d_n4;
        locals.var_eyd_dn5 = assign101370_e153518_d_n5;
        locals.var_eyd_dn6 = assign101370_e153518_d_n6;
        locals.var_eyd_dn7 = assign101370_e153518_d_n7;
        locals.var_eyd_dn8 = assign101370_e153518_d_n8;
        locals.var_eyd_dn9 = assign101370_e153518_d_n9;
        locals.var_eyd_dn10 = assign101370_e153518_d_n10;
        locals.var_eyd_dn11 = assign101370_e153518_d_n11;
        locals.var_eyd_dn14 = assign101370_e153518_d_n14;

        let (assign101380_e153526, assign101380_e153526_d_n0, assign101380_e153526_d_n2, assign101380_e153526_d_n4, assign101380_e153526_d_n5, assign101380_e153526_d_n6, assign101380_e153526_d_n7, assign101380_e153526_d_n8, assign101380_e153526_d_n9, assign101380_e153526_d_n10, assign101380_e153526_d_n11, assign101380_e153526_d_n14,) = {
    if (locals.var_guard2327 != 0.0) {
        let assign101380_e153522: f64 = (locals.var_muun * locals.var_eyd);
        let assign101380_e153524: f64 = (assign101380_e153522 / 10000000.0);
        (assign101380_e153524, (((locals.var_muun_dn0 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn0)) / 10000000.0), (((locals.var_muun_dn2 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn2)) / 10000000.0), (((locals.var_muun_dn4 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn4)) / 10000000.0), (((locals.var_muun_dn5 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn5)) / 10000000.0), (((locals.var_muun_dn6 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn6)) / 10000000.0), (((locals.var_muun_dn7 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn7)) / 10000000.0), (((locals.var_muun_dn8 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn8)) / 10000000.0), (((locals.var_muun_dn9 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn9)) / 10000000.0), (((locals.var_muun_dn10 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn10)) / 10000000.0), (((locals.var_muun_dn11 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn11)) / 10000000.0), (((locals.var_muun_dn14 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn14)) / 10000000.0),)
    } else {
        (locals.var_t12, locals.var_t12_dn0, locals.var_t12_dn2, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn14,)
    }
};
        locals.var_t12 = assign101380_e153526;
        locals.var_t12_dn0 = assign101380_e153526_d_n0;
        locals.var_t12_dn2 = assign101380_e153526_d_n2;
        locals.var_t12_dn4 = assign101380_e153526_d_n4;
        locals.var_t12_dn5 = assign101380_e153526_d_n5;
        locals.var_t12_dn6 = assign101380_e153526_d_n6;
        locals.var_t12_dn7 = assign101380_e153526_d_n7;
        locals.var_t12_dn8 = assign101380_e153526_d_n8;
        locals.var_t12_dn9 = assign101380_e153526_d_n9;
        locals.var_t12_dn10 = assign101380_e153526_d_n10;
        locals.var_t12_dn11 = assign101380_e153526_d_n11;
        locals.var_t12_dn14 = assign101380_e153526_d_n14;

        let assign101390_e153530: f64 = (10.0 * 2.220446049250313e-16);
        let assign101390_e153531: f64 = (1.0 - assign101390_e153530);
        let assign101390_e153538: f64 = (10.0 * 2.220446049250313e-16);
        let assign101390_e153539: f64 = (1.0 + assign101390_e153538);
        let assign101390_e153541: f64 = if ((assign101390_e153531 <= p.p178) && (p.p178 <= assign101390_e153539)) { 1.0 } else { 0.0 };
        locals.var_guard2328 = assign101390_e153541;

        let (assign101400_e153547, assign101400_e153547_d_n0, assign101400_e153547_d_n2, assign101400_e153547_d_n4, assign101400_e153547_d_n5, assign101400_e153547_d_n6, assign101400_e153547_d_n7, assign101400_e153547_d_n8, assign101400_e153547_d_n9, assign101400_e153547_d_n10, assign101400_e153547_d_n11, assign101400_e153547_d_n14,) = {
    if ((locals.var_guard2327 != 0.0) && (locals.var_guard2328 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign101400_e153547;
        locals.var_t7_dn0 = assign101400_e153547_d_n0;
        locals.var_t7_dn2 = assign101400_e153547_d_n2;
        locals.var_t7_dn4 = assign101400_e153547_d_n4;
        locals.var_t7_dn5 = assign101400_e153547_d_n5;
        locals.var_t7_dn6 = assign101400_e153547_d_n6;
        locals.var_t7_dn7 = assign101400_e153547_d_n7;
        locals.var_t7_dn8 = assign101400_e153547_d_n8;
        locals.var_t7_dn9 = assign101400_e153547_d_n9;
        locals.var_t7_dn10 = assign101400_e153547_d_n10;
        locals.var_t7_dn11 = assign101400_e153547_d_n11;
        locals.var_t7_dn14 = assign101400_e153547_d_n14;

        let assign101410_e153551: f64 = (10.0 * 2.220446049250313e-16);
        let assign101410_e153552: f64 = (2.0 - assign101410_e153551);
        let assign101410_e153559: f64 = (10.0 * 2.220446049250313e-16);
        let assign101410_e153560: f64 = (2.0 + assign101410_e153559);
        let assign101410_e153562: f64 = if ((assign101410_e153552 <= p.p178) && (p.p178 <= assign101410_e153560)) { 1.0 } else { 0.0 };
        locals.var_guard2329 = assign101410_e153562;

        let (assign101420_e153571, assign101420_e153571_d_n0, assign101420_e153571_d_n2, assign101420_e153571_d_n4, assign101420_e153571_d_n5, assign101420_e153571_d_n6, assign101420_e153571_d_n7, assign101420_e153571_d_n8, assign101420_e153571_d_n9, assign101420_e153571_d_n10, assign101420_e153571_d_n11, assign101420_e153571_d_n14,) = {
    if (((locals.var_guard2327 != 0.0) && (locals.var_guard2328 == 0.0)) && (locals.var_guard2329 != 0.0)) {
        (locals.var_t12, locals.var_t12_dn0, locals.var_t12_dn2, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn14,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign101420_e153571;
        locals.var_t7_dn0 = assign101420_e153571_d_n0;
        locals.var_t7_dn2 = assign101420_e153571_d_n2;
        locals.var_t7_dn4 = assign101420_e153571_d_n4;
        locals.var_t7_dn5 = assign101420_e153571_d_n5;
        locals.var_t7_dn6 = assign101420_e153571_d_n6;
        locals.var_t7_dn7 = assign101420_e153571_d_n7;
        locals.var_t7_dn8 = assign101420_e153571_d_n8;
        locals.var_t7_dn9 = assign101420_e153571_d_n9;
        locals.var_t7_dn10 = assign101420_e153571_d_n10;
        locals.var_t7_dn11 = assign101420_e153571_d_n11;
        locals.var_t7_dn14 = assign101420_e153571_d_n14;

        let (assign101430_e153590, assign101430_e153590_d_n0, assign101430_e153590_d_n2, assign101430_e153590_d_n4, assign101430_e153590_d_n5, assign101430_e153590_d_n6, assign101430_e153590_d_n7, assign101430_e153590_d_n8, assign101430_e153590_d_n9, assign101430_e153590_d_n10, assign101430_e153590_d_n11, assign101430_e153590_d_n14,) = {
    if (((locals.var_guard2327 != 0.0) && (locals.var_guard2328 == 0.0)) && (locals.var_guard2329 == 0.0)) {
        let (assign101430_e153588, assign101430_e153588_d_n0, assign101430_e153588_d_n2, assign101430_e153588_d_n4, assign101430_e153588_d_n5, assign101430_e153588_d_n6, assign101430_e153588_d_n7, assign101430_e153588_d_n8, assign101430_e153588_d_n9, assign101430_e153588_d_n10, assign101430_e153588_d_n11, assign101430_e153588_d_n14,) = {
            if (locals.var_eyd == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign101430_e153586: f64 = (p.p178 - 1.0);
                let assign101430_e153587: f64 = (locals.var_eyd).powf(assign101430_e153586);
                (assign101430_e153587, if 0.0 == 0.0 && ((assign101430_e153586) as f64).is_finite() && ((assign101430_e153586) as f64).fract() == 0.0 { if assign101430_e153586 == 0.0 { 0.0 } else { (assign101430_e153586 * ((locals.var_eyd).powf(assign101430_e153586 - 1.0) * locals.var_eyd_dn0)) } } else { (assign101430_e153587 * (assign101430_e153586 * (locals.var_eyd_dn0 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101430_e153586) as f64).is_finite() && ((assign101430_e153586) as f64).fract() == 0.0 { if assign101430_e153586 == 0.0 { 0.0 } else { (assign101430_e153586 * ((locals.var_eyd).powf(assign101430_e153586 - 1.0) * locals.var_eyd_dn2)) } } else { (assign101430_e153587 * (assign101430_e153586 * (locals.var_eyd_dn2 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101430_e153586) as f64).is_finite() && ((assign101430_e153586) as f64).fract() == 0.0 { if assign101430_e153586 == 0.0 { 0.0 } else { (assign101430_e153586 * ((locals.var_eyd).powf(assign101430_e153586 - 1.0) * locals.var_eyd_dn4)) } } else { (assign101430_e153587 * (assign101430_e153586 * (locals.var_eyd_dn4 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101430_e153586) as f64).is_finite() && ((assign101430_e153586) as f64).fract() == 0.0 { if assign101430_e153586 == 0.0 { 0.0 } else { (assign101430_e153586 * ((locals.var_eyd).powf(assign101430_e153586 - 1.0) * locals.var_eyd_dn5)) } } else { (assign101430_e153587 * (assign101430_e153586 * (locals.var_eyd_dn5 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101430_e153586) as f64).is_finite() && ((assign101430_e153586) as f64).fract() == 0.0 { if assign101430_e153586 == 0.0 { 0.0 } else { (assign101430_e153586 * ((locals.var_eyd).powf(assign101430_e153586 - 1.0) * locals.var_eyd_dn6)) } } else { (assign101430_e153587 * (assign101430_e153586 * (locals.var_eyd_dn6 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101430_e153586) as f64).is_finite() && ((assign101430_e153586) as f64).fract() == 0.0 { if assign101430_e153586 == 0.0 { 0.0 } else { (assign101430_e153586 * ((locals.var_eyd).powf(assign101430_e153586 - 1.0) * locals.var_eyd_dn7)) } } else { (assign101430_e153587 * (assign101430_e153586 * (locals.var_eyd_dn7 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101430_e153586) as f64).is_finite() && ((assign101430_e153586) as f64).fract() == 0.0 { if assign101430_e153586 == 0.0 { 0.0 } else { (assign101430_e153586 * ((locals.var_eyd).powf(assign101430_e153586 - 1.0) * locals.var_eyd_dn8)) } } else { (assign101430_e153587 * (assign101430_e153586 * (locals.var_eyd_dn8 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101430_e153586) as f64).is_finite() && ((assign101430_e153586) as f64).fract() == 0.0 { if assign101430_e153586 == 0.0 { 0.0 } else { (assign101430_e153586 * ((locals.var_eyd).powf(assign101430_e153586 - 1.0) * locals.var_eyd_dn9)) } } else { (assign101430_e153587 * (assign101430_e153586 * (locals.var_eyd_dn9 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101430_e153586) as f64).is_finite() && ((assign101430_e153586) as f64).fract() == 0.0 { if assign101430_e153586 == 0.0 { 0.0 } else { (assign101430_e153586 * ((locals.var_eyd).powf(assign101430_e153586 - 1.0) * locals.var_eyd_dn10)) } } else { (assign101430_e153587 * (assign101430_e153586 * (locals.var_eyd_dn10 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101430_e153586) as f64).is_finite() && ((assign101430_e153586) as f64).fract() == 0.0 { if assign101430_e153586 == 0.0 { 0.0 } else { (assign101430_e153586 * ((locals.var_eyd).powf(assign101430_e153586 - 1.0) * locals.var_eyd_dn11)) } } else { (assign101430_e153587 * (assign101430_e153586 * (locals.var_eyd_dn11 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101430_e153586) as f64).is_finite() && ((assign101430_e153586) as f64).fract() == 0.0 { if assign101430_e153586 == 0.0 { 0.0 } else { (assign101430_e153586 * ((locals.var_eyd).powf(assign101430_e153586 - 1.0) * locals.var_eyd_dn14)) } } else { (assign101430_e153587 * (assign101430_e153586 * (locals.var_eyd_dn14 / locals.var_eyd))) },)
            }
        };
        (assign101430_e153588, assign101430_e153588_d_n0, assign101430_e153588_d_n2, assign101430_e153588_d_n4, assign101430_e153588_d_n5, assign101430_e153588_d_n6, assign101430_e153588_d_n7, assign101430_e153588_d_n8, assign101430_e153588_d_n9, assign101430_e153588_d_n10, assign101430_e153588_d_n11, assign101430_e153588_d_n14,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign101430_e153590;
        locals.var_t7_dn0 = assign101430_e153590_d_n0;
        locals.var_t7_dn2 = assign101430_e153590_d_n2;
        locals.var_t7_dn4 = assign101430_e153590_d_n4;
        locals.var_t7_dn5 = assign101430_e153590_d_n5;
        locals.var_t7_dn6 = assign101430_e153590_d_n6;
        locals.var_t7_dn7 = assign101430_e153590_d_n7;
        locals.var_t7_dn8 = assign101430_e153590_d_n8;
        locals.var_t7_dn9 = assign101430_e153590_d_n9;
        locals.var_t7_dn10 = assign101430_e153590_d_n10;
        locals.var_t7_dn11 = assign101430_e153590_d_n11;
        locals.var_t7_dn14 = assign101430_e153590_d_n14;

        let (assign101440_e153596, assign101440_e153596_d_n0, assign101440_e153596_d_n2, assign101440_e153596_d_n4, assign101440_e153596_d_n5, assign101440_e153596_d_n6, assign101440_e153596_d_n7, assign101440_e153596_d_n8, assign101440_e153596_d_n9, assign101440_e153596_d_n10, assign101440_e153596_d_n11, assign101440_e153596_d_n14,) = {
    if (locals.var_guard2327 != 0.0) {
        let assign101440_e153594: f64 = (locals.var_t12 * locals.var_t7);
        (assign101440_e153594, ((locals.var_t12_dn0 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn0)), ((locals.var_t12_dn2 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn2)), ((locals.var_t12_dn4 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn4)), ((locals.var_t12_dn5 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn5)), ((locals.var_t12_dn6 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn6)), ((locals.var_t12_dn7 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn7)), ((locals.var_t12_dn8 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn8)), ((locals.var_t12_dn9 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn9)), ((locals.var_t12_dn10 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn10)), ((locals.var_t12_dn11 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn11)), ((locals.var_t12_dn14 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn14)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign101440_e153596;
        locals.var_t8_dn0 = assign101440_e153596_d_n0;
        locals.var_t8_dn2 = assign101440_e153596_d_n2;
        locals.var_t8_dn4 = assign101440_e153596_d_n4;
        locals.var_t8_dn5 = assign101440_e153596_d_n5;
        locals.var_t8_dn6 = assign101440_e153596_d_n6;
        locals.var_t8_dn7 = assign101440_e153596_d_n7;
        locals.var_t8_dn8 = assign101440_e153596_d_n8;
        locals.var_t8_dn9 = assign101440_e153596_d_n9;
        locals.var_t8_dn10 = assign101440_e153596_d_n10;
        locals.var_t8_dn11 = assign101440_e153596_d_n11;
        locals.var_t8_dn14 = assign101440_e153596_d_n14;

        let (assign101450_e153602, assign101450_e153602_d_n0, assign101450_e153602_d_n2, assign101450_e153602_d_n4, assign101450_e153602_d_n5, assign101450_e153602_d_n6, assign101450_e153602_d_n7, assign101450_e153602_d_n8, assign101450_e153602_d_n9, assign101450_e153602_d_n10, assign101450_e153602_d_n11, assign101450_e153602_d_n14,) = {
    if (locals.var_guard2327 != 0.0) {
        let assign101450_e153600: f64 = (1.0 + locals.var_t8);
        (assign101450_e153600, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign101450_e153602;
        locals.var_t9_dn0 = assign101450_e153602_d_n0;
        locals.var_t9_dn2 = assign101450_e153602_d_n2;
        locals.var_t9_dn4 = assign101450_e153602_d_n4;
        locals.var_t9_dn5 = assign101450_e153602_d_n5;
        locals.var_t9_dn6 = assign101450_e153602_d_n6;
        locals.var_t9_dn7 = assign101450_e153602_d_n7;
        locals.var_t9_dn8 = assign101450_e153602_d_n8;
        locals.var_t9_dn9 = assign101450_e153602_d_n9;
        locals.var_t9_dn10 = assign101450_e153602_d_n10;
        locals.var_t9_dn11 = assign101450_e153602_d_n11;
        locals.var_t9_dn14 = assign101450_e153602_d_n14;

        let (assign101460_e153618, assign101460_e153618_d_n0, assign101460_e153618_d_n2, assign101460_e153618_d_n4, assign101460_e153618_d_n5, assign101460_e153618_d_n6, assign101460_e153618_d_n7, assign101460_e153618_d_n8, assign101460_e153618_d_n9, assign101460_e153618_d_n10, assign101460_e153618_d_n11, assign101460_e153618_d_n14,) = {
    if (locals.var_guard2327 != 0.0) {
        let (assign101460_e153616, assign101460_e153616_d_n0, assign101460_e153616_d_n2, assign101460_e153616_d_n4, assign101460_e153616_d_n5, assign101460_e153616_d_n6, assign101460_e153616_d_n7, assign101460_e153616_d_n8, assign101460_e153616_d_n9, assign101460_e153616_d_n10, assign101460_e153616_d_n11, assign101460_e153616_d_n14,) = {
            if (locals.var_t9 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign101460_e153610: f64 = (-1.0);
                let assign101460_e153612: f64 = (assign101460_e153610 / p.p178);
                let assign101460_e153614: f64 = (assign101460_e153612 - 1.0);
                let assign101460_e153615: f64 = (locals.var_t9).powf(assign101460_e153614);
                (assign101460_e153615, if 0.0 == 0.0 && ((assign101460_e153614) as f64).is_finite() && ((assign101460_e153614) as f64).fract() == 0.0 { if assign101460_e153614 == 0.0 { 0.0 } else { (assign101460_e153614 * ((locals.var_t9).powf(assign101460_e153614 - 1.0) * locals.var_t9_dn0)) } } else { (assign101460_e153615 * (assign101460_e153614 * (locals.var_t9_dn0 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101460_e153614) as f64).is_finite() && ((assign101460_e153614) as f64).fract() == 0.0 { if assign101460_e153614 == 0.0 { 0.0 } else { (assign101460_e153614 * ((locals.var_t9).powf(assign101460_e153614 - 1.0) * locals.var_t9_dn2)) } } else { (assign101460_e153615 * (assign101460_e153614 * (locals.var_t9_dn2 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101460_e153614) as f64).is_finite() && ((assign101460_e153614) as f64).fract() == 0.0 { if assign101460_e153614 == 0.0 { 0.0 } else { (assign101460_e153614 * ((locals.var_t9).powf(assign101460_e153614 - 1.0) * locals.var_t9_dn4)) } } else { (assign101460_e153615 * (assign101460_e153614 * (locals.var_t9_dn4 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101460_e153614) as f64).is_finite() && ((assign101460_e153614) as f64).fract() == 0.0 { if assign101460_e153614 == 0.0 { 0.0 } else { (assign101460_e153614 * ((locals.var_t9).powf(assign101460_e153614 - 1.0) * locals.var_t9_dn5)) } } else { (assign101460_e153615 * (assign101460_e153614 * (locals.var_t9_dn5 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101460_e153614) as f64).is_finite() && ((assign101460_e153614) as f64).fract() == 0.0 { if assign101460_e153614 == 0.0 { 0.0 } else { (assign101460_e153614 * ((locals.var_t9).powf(assign101460_e153614 - 1.0) * locals.var_t9_dn6)) } } else { (assign101460_e153615 * (assign101460_e153614 * (locals.var_t9_dn6 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101460_e153614) as f64).is_finite() && ((assign101460_e153614) as f64).fract() == 0.0 { if assign101460_e153614 == 0.0 { 0.0 } else { (assign101460_e153614 * ((locals.var_t9).powf(assign101460_e153614 - 1.0) * locals.var_t9_dn7)) } } else { (assign101460_e153615 * (assign101460_e153614 * (locals.var_t9_dn7 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101460_e153614) as f64).is_finite() && ((assign101460_e153614) as f64).fract() == 0.0 { if assign101460_e153614 == 0.0 { 0.0 } else { (assign101460_e153614 * ((locals.var_t9).powf(assign101460_e153614 - 1.0) * locals.var_t9_dn8)) } } else { (assign101460_e153615 * (assign101460_e153614 * (locals.var_t9_dn8 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101460_e153614) as f64).is_finite() && ((assign101460_e153614) as f64).fract() == 0.0 { if assign101460_e153614 == 0.0 { 0.0 } else { (assign101460_e153614 * ((locals.var_t9).powf(assign101460_e153614 - 1.0) * locals.var_t9_dn9)) } } else { (assign101460_e153615 * (assign101460_e153614 * (locals.var_t9_dn9 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101460_e153614) as f64).is_finite() && ((assign101460_e153614) as f64).fract() == 0.0 { if assign101460_e153614 == 0.0 { 0.0 } else { (assign101460_e153614 * ((locals.var_t9).powf(assign101460_e153614 - 1.0) * locals.var_t9_dn10)) } } else { (assign101460_e153615 * (assign101460_e153614 * (locals.var_t9_dn10 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101460_e153614) as f64).is_finite() && ((assign101460_e153614) as f64).fract() == 0.0 { if assign101460_e153614 == 0.0 { 0.0 } else { (assign101460_e153614 * ((locals.var_t9).powf(assign101460_e153614 - 1.0) * locals.var_t9_dn11)) } } else { (assign101460_e153615 * (assign101460_e153614 * (locals.var_t9_dn11 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101460_e153614) as f64).is_finite() && ((assign101460_e153614) as f64).fract() == 0.0 { if assign101460_e153614 == 0.0 { 0.0 } else { (assign101460_e153614 * ((locals.var_t9).powf(assign101460_e153614 - 1.0) * locals.var_t9_dn14)) } } else { (assign101460_e153615 * (assign101460_e153614 * (locals.var_t9_dn14 / locals.var_t9))) },)
            }
        };
        (assign101460_e153616, assign101460_e153616_d_n0, assign101460_e153616_d_n2, assign101460_e153616_d_n4, assign101460_e153616_d_n5, assign101460_e153616_d_n6, assign101460_e153616_d_n7, assign101460_e153616_d_n8, assign101460_e153616_d_n9, assign101460_e153616_d_n10, assign101460_e153616_d_n11, assign101460_e153616_d_n14,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign101460_e153618;
        locals.var_t10_dn0 = assign101460_e153618_d_n0;
        locals.var_t10_dn2 = assign101460_e153618_d_n2;
        locals.var_t10_dn4 = assign101460_e153618_d_n4;
        locals.var_t10_dn5 = assign101460_e153618_d_n5;
        locals.var_t10_dn6 = assign101460_e153618_d_n6;
        locals.var_t10_dn7 = assign101460_e153618_d_n7;
        locals.var_t10_dn8 = assign101460_e153618_d_n8;
        locals.var_t10_dn9 = assign101460_e153618_d_n9;
        locals.var_t10_dn10 = assign101460_e153618_d_n10;
        locals.var_t10_dn11 = assign101460_e153618_d_n11;
        locals.var_t10_dn14 = assign101460_e153618_d_n14;

        let (assign101470_e153624, assign101470_e153624_d_n0, assign101470_e153624_d_n2, assign101470_e153624_d_n4, assign101470_e153624_d_n5, assign101470_e153624_d_n6, assign101470_e153624_d_n7, assign101470_e153624_d_n8, assign101470_e153624_d_n9, assign101470_e153624_d_n10, assign101470_e153624_d_n11, assign101470_e153624_d_n14,) = {
    if (locals.var_guard2327 != 0.0) {
        let assign101470_e153622: f64 = (locals.var_t9 * locals.var_t10);
        (assign101470_e153622, ((locals.var_t9_dn0 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn0)), ((locals.var_t9_dn2 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn2)), ((locals.var_t9_dn4 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn4)), ((locals.var_t9_dn5 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn5)), ((locals.var_t9_dn6 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn6)), ((locals.var_t9_dn7 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn7)), ((locals.var_t9_dn8 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn8)), ((locals.var_t9_dn9 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn9)), ((locals.var_t9_dn10 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn10)), ((locals.var_t9_dn11 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn11)), ((locals.var_t9_dn14 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn14)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign101470_e153624;
        locals.var_t11_dn0 = assign101470_e153624_d_n0;
        locals.var_t11_dn2 = assign101470_e153624_d_n2;
        locals.var_t11_dn4 = assign101470_e153624_d_n4;
        locals.var_t11_dn5 = assign101470_e153624_d_n5;
        locals.var_t11_dn6 = assign101470_e153624_d_n6;
        locals.var_t11_dn7 = assign101470_e153624_d_n7;
        locals.var_t11_dn8 = assign101470_e153624_d_n8;
        locals.var_t11_dn9 = assign101470_e153624_d_n9;
        locals.var_t11_dn10 = assign101470_e153624_d_n10;
        locals.var_t11_dn11 = assign101470_e153624_d_n11;
        locals.var_t11_dn14 = assign101470_e153624_d_n14;

        let (assign101480_e153630, assign101480_e153630_d_n0, assign101480_e153630_d_n2, assign101480_e153630_d_n4, assign101480_e153630_d_n5, assign101480_e153630_d_n6, assign101480_e153630_d_n7, assign101480_e153630_d_n8, assign101480_e153630_d_n9, assign101480_e153630_d_n10, assign101480_e153630_d_n11, assign101480_e153630_d_n14,) = {
    if (locals.var_guard2327 != 0.0) {
        let assign101480_e153628: f64 = (locals.var_muun * locals.var_t11);
        (assign101480_e153628, ((locals.var_muun_dn0 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn0)), ((locals.var_muun_dn2 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn2)), ((locals.var_muun_dn4 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn4)), ((locals.var_muun_dn5 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn5)), ((locals.var_muun_dn6 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn6)), ((locals.var_muun_dn7 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn7)), ((locals.var_muun_dn8 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn8)), ((locals.var_muun_dn9 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn9)), ((locals.var_muun_dn10 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn10)), ((locals.var_muun_dn11 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn11)), ((locals.var_muun_dn14 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn14)),)
    } else {
        (locals.var_mud_hoso, locals.var_mud_hoso_dn0, locals.var_mud_hoso_dn2, locals.var_mud_hoso_dn4, locals.var_mud_hoso_dn5, locals.var_mud_hoso_dn6, locals.var_mud_hoso_dn7, locals.var_mud_hoso_dn8, locals.var_mud_hoso_dn9, locals.var_mud_hoso_dn10, locals.var_mud_hoso_dn11, locals.var_mud_hoso_dn14,)
    }
};
        locals.var_mud_hoso = assign101480_e153630;
        locals.var_mud_hoso_dn0 = assign101480_e153630_d_n0;
        locals.var_mud_hoso_dn2 = assign101480_e153630_d_n2;
        locals.var_mud_hoso_dn4 = assign101480_e153630_d_n4;
        locals.var_mud_hoso_dn5 = assign101480_e153630_d_n5;
        locals.var_mud_hoso_dn6 = assign101480_e153630_d_n6;
        locals.var_mud_hoso_dn7 = assign101480_e153630_d_n7;
        locals.var_mud_hoso_dn8 = assign101480_e153630_d_n8;
        locals.var_mud_hoso_dn9 = assign101480_e153630_d_n9;
        locals.var_mud_hoso_dn10 = assign101480_e153630_d_n10;
        locals.var_mud_hoso_dn11 = assign101480_e153630_d_n11;
        locals.var_mud_hoso_dn14 = assign101480_e153630_d_n14;

        let (assign101490_e153638, assign101490_e153638_d_n0, assign101490_e153638_d_n2, assign101490_e153638_d_n4, assign101490_e153638_d_n5, assign101490_e153638_d_n6, assign101490_e153638_d_n7, assign101490_e153638_d_n8, assign101490_e153638_d_n9, assign101490_e153638_d_n10, assign101490_e153638_d_n11, assign101490_e153638_d_n14,) = {
    if (locals.var_guard2327 != 0.0) {
        let assign101490_e153634: f64 = (locals.var_mu + locals.var_mud_hoso);
        let assign101490_e153636: f64 = (assign101490_e153634 / 2.0);
        (assign101490_e153636, ((locals.var_mu_dn0 + locals.var_mud_hoso_dn0) / 2.0), ((locals.var_mu_dn2 + locals.var_mud_hoso_dn2) / 2.0), ((locals.var_mu_dn4 + locals.var_mud_hoso_dn4) / 2.0), ((locals.var_mu_dn5 + locals.var_mud_hoso_dn5) / 2.0), ((locals.var_mu_dn6 + locals.var_mud_hoso_dn6) / 2.0), ((locals.var_mu_dn7 + locals.var_mud_hoso_dn7) / 2.0), ((locals.var_mu_dn8 + locals.var_mud_hoso_dn8) / 2.0), ((locals.var_mu_dn9 + locals.var_mud_hoso_dn9) / 2.0), ((locals.var_mu_dn10 + locals.var_mud_hoso_dn10) / 2.0), ((locals.var_mu_dn11 + locals.var_mud_hoso_dn11) / 2.0), ((locals.var_mu_dn14 + locals.var_mud_hoso_dn14) / 2.0),)
    } else {
        (locals.var_mu_ave, locals.var_mu_ave_dn0, locals.var_mu_ave_dn2, locals.var_mu_ave_dn4, locals.var_mu_ave_dn5, locals.var_mu_ave_dn6, locals.var_mu_ave_dn7, locals.var_mu_ave_dn8, locals.var_mu_ave_dn9, locals.var_mu_ave_dn10, locals.var_mu_ave_dn11, locals.var_mu_ave_dn14,)
    }
};
        locals.var_mu_ave = assign101490_e153638;
        locals.var_mu_ave_dn0 = assign101490_e153638_d_n0;
        locals.var_mu_ave_dn2 = assign101490_e153638_d_n2;
        locals.var_mu_ave_dn4 = assign101490_e153638_d_n4;
        locals.var_mu_ave_dn5 = assign101490_e153638_d_n5;
        locals.var_mu_ave_dn6 = assign101490_e153638_d_n6;
        locals.var_mu_ave_dn7 = assign101490_e153638_d_n7;
        locals.var_mu_ave_dn8 = assign101490_e153638_d_n8;
        locals.var_mu_ave_dn9 = assign101490_e153638_d_n9;
        locals.var_mu_ave_dn10 = assign101490_e153638_d_n10;
        locals.var_mu_ave_dn11 = assign101490_e153638_d_n11;
        locals.var_mu_ave_dn14 = assign101490_e153638_d_n14;

        let (assign101500_e153644, assign101500_e153644_d_n0, assign101500_e153644_d_n2, assign101500_e153644_d_n4, assign101500_e153644_d_n5, assign101500_e153644_d_n6, assign101500_e153644_d_n7, assign101500_e153644_d_n8, assign101500_e153644_d_n9, assign101500_e153644_d_n10, assign101500_e153644_d_n11, assign101500_e153644_d_n14,) = {
    if (locals.var_guard2327 != 0.0) {
        let assign101500_e153642: f64 = (locals.var_alpha * locals.var_alpha);
        (assign101500_e153642, ((locals.var_alpha_dn0 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn0)), ((locals.var_alpha_dn2 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn2)), ((locals.var_alpha_dn4 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn4)), ((locals.var_alpha_dn5 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn5)), ((locals.var_alpha_dn6 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn6)), ((locals.var_alpha_dn7 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn7)), ((locals.var_alpha_dn8 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn8)), ((locals.var_alpha_dn9 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn9)), ((locals.var_alpha_dn10 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn10)), ((locals.var_alpha_dn11 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn11)), ((locals.var_alpha_dn14 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign101500_e153644;
        locals.var_t0_dn0 = assign101500_e153644_d_n0;
        locals.var_t0_dn2 = assign101500_e153644_d_n2;
        locals.var_t0_dn4 = assign101500_e153644_d_n4;
        locals.var_t0_dn5 = assign101500_e153644_d_n5;
        locals.var_t0_dn6 = assign101500_e153644_d_n6;
        locals.var_t0_dn7 = assign101500_e153644_d_n7;
        locals.var_t0_dn8 = assign101500_e153644_d_n8;
        locals.var_t0_dn9 = assign101500_e153644_d_n9;
        locals.var_t0_dn10 = assign101500_e153644_d_n10;
        locals.var_t0_dn11 = assign101500_e153644_d_n11;
        locals.var_t0_dn14 = assign101500_e153644_d_n14;

        let (assign101510_e153706, assign101510_e153706_d_n0, assign101510_e153706_d_n2, assign101510_e153706_d_n4, assign101510_e153706_d_n5, assign101510_e153706_d_n6, assign101510_e153706_d_n7, assign101510_e153706_d_n8, assign101510_e153706_d_n9, assign101510_e153706_d_n10, assign101510_e153706_d_n11, assign101510_e153706_d_n14,) = {
    if (locals.var_guard2327 != 0.0) {
        let assign101510_e153648: f64 = (locals.var_weff_nf * locals.var_cox);
        let assign101510_e153650: f64 = (assign101510_e153648 * locals.var_vgvt);
        let assign101510_e153652: f64 = (assign101510_e153650 * locals.var_mu);
        let assign101510_e153656: f64 = (3.0 * locals.var_alpha);
        let assign101510_e153657: f64 = (1.0 + assign101510_e153656);
        let assign101510_e153660: f64 = (6.0 * locals.var_t0);
        let assign101510_e153661: f64 = (assign101510_e153657 + assign101510_e153660);
        let assign101510_e153663: f64 = (assign101510_e153661 * locals.var_mud_hoso);
        let assign101510_e153665: f64 = (assign101510_e153663 * locals.var_mud_hoso);
        let assign101510_e153669: f64 = (4.0 * locals.var_alpha);
        let assign101510_e153670: f64 = (3.0 + assign101510_e153669);
        let assign101510_e153673: f64 = (3.0 * locals.var_t0);
        let assign101510_e153674: f64 = (assign101510_e153670 + assign101510_e153673);
        let assign101510_e153676: f64 = (assign101510_e153674 * locals.var_mud_hoso);
        let assign101510_e153678: f64 = (assign101510_e153676 * locals.var_mu);
        let assign101510_e153679: f64 = (assign101510_e153665 + assign101510_e153678);
        let assign101510_e153683: f64 = (3.0 * locals.var_alpha);
        let assign101510_e153684: f64 = (6.0 + assign101510_e153683);
        let assign101510_e153686: f64 = (assign101510_e153684 + locals.var_t0);
        let assign101510_e153688: f64 = (assign101510_e153686 * locals.var_mu);
        let assign101510_e153690: f64 = (assign101510_e153688 * locals.var_mu);
        let assign101510_e153691: f64 = (assign101510_e153679 + assign101510_e153690);
        let assign101510_e153692: f64 = (assign101510_e153652 * assign101510_e153691);
        let assign101510_e153695: f64 = (15.0 * locals.var_lch);
        let assign101510_e153698: f64 = (1.0 + locals.var_alpha);
        let assign101510_e153699: f64 = (assign101510_e153695 * assign101510_e153698);
        let assign101510_e153701: f64 = (assign101510_e153699 * locals.var_mu_ave);
        let assign101510_e153703: f64 = (assign101510_e153701 * locals.var_mu_ave);
        let assign101510_e153704: f64 = (assign101510_e153692 / assign101510_e153703);
        (assign101510_e153704, ((((((((((locals.var_weff_nf * locals.var_cox_dn0) * locals.var_vgvt) + (assign101510_e153648 * locals.var_vgvt_dn0)) * locals.var_mu) + (assign101510_e153650 * locals.var_mu_dn0)) * assign101510_e153691) + (assign101510_e153652 * ((((((((3.0 * locals.var_alpha_dn0) + (6.0 * locals.var_t0_dn0)) * locals.var_mud_hoso) + (assign101510_e153661 * locals.var_mud_hoso_dn0)) * locals.var_mud_hoso) + (assign101510_e153663 * locals.var_mud_hoso_dn0)) + ((((((4.0 * locals.var_alpha_dn0) + (3.0 * locals.var_t0_dn0)) * locals.var_mud_hoso) + (assign101510_e153674 * locals.var_mud_hoso_dn0)) * locals.var_mu) + (assign101510_e153676 * locals.var_mu_dn0))) + ((((((3.0 * locals.var_alpha_dn0) + locals.var_t0_dn0) * locals.var_mu) + (assign101510_e153686 * locals.var_mu_dn0)) * locals.var_mu) + (assign101510_e153688 * locals.var_mu_dn0))))) * assign101510_e153703) - (assign101510_e153692 * (((((((15.0 * locals.var_lch_dn0) * assign101510_e153698) + (assign101510_e153695 * locals.var_alpha_dn0)) * locals.var_mu_ave) + (assign101510_e153699 * locals.var_mu_ave_dn0)) * locals.var_mu_ave) + (assign101510_e153701 * locals.var_mu_ave_dn0)))) / (assign101510_e153703 * assign101510_e153703)), ((((((((((locals.var_weff_nf * locals.var_cox_dn2) * locals.var_vgvt) + (assign101510_e153648 * locals.var_vgvt_dn2)) * locals.var_mu) + (assign101510_e153650 * locals.var_mu_dn2)) * assign101510_e153691) + (assign101510_e153652 * ((((((((3.0 * locals.var_alpha_dn2) + (6.0 * locals.var_t0_dn2)) * locals.var_mud_hoso) + (assign101510_e153661 * locals.var_mud_hoso_dn2)) * locals.var_mud_hoso) + (assign101510_e153663 * locals.var_mud_hoso_dn2)) + ((((((4.0 * locals.var_alpha_dn2) + (3.0 * locals.var_t0_dn2)) * locals.var_mud_hoso) + (assign101510_e153674 * locals.var_mud_hoso_dn2)) * locals.var_mu) + (assign101510_e153676 * locals.var_mu_dn2))) + ((((((3.0 * locals.var_alpha_dn2) + locals.var_t0_dn2) * locals.var_mu) + (assign101510_e153686 * locals.var_mu_dn2)) * locals.var_mu) + (assign101510_e153688 * locals.var_mu_dn2))))) * assign101510_e153703) - (assign101510_e153692 * (((((((15.0 * locals.var_lch_dn2) * assign101510_e153698) + (assign101510_e153695 * locals.var_alpha_dn2)) * locals.var_mu_ave) + (assign101510_e153699 * locals.var_mu_ave_dn2)) * locals.var_mu_ave) + (assign101510_e153701 * locals.var_mu_ave_dn2)))) / (assign101510_e153703 * assign101510_e153703)), ((((((((((locals.var_weff_nf * locals.var_cox_dn4) * locals.var_vgvt) + (assign101510_e153648 * locals.var_vgvt_dn4)) * locals.var_mu) + (assign101510_e153650 * locals.var_mu_dn4)) * assign101510_e153691) + (assign101510_e153652 * ((((((((3.0 * locals.var_alpha_dn4) + (6.0 * locals.var_t0_dn4)) * locals.var_mud_hoso) + (assign101510_e153661 * locals.var_mud_hoso_dn4)) * locals.var_mud_hoso) + (assign101510_e153663 * locals.var_mud_hoso_dn4)) + ((((((4.0 * locals.var_alpha_dn4) + (3.0 * locals.var_t0_dn4)) * locals.var_mud_hoso) + (assign101510_e153674 * locals.var_mud_hoso_dn4)) * locals.var_mu) + (assign101510_e153676 * locals.var_mu_dn4))) + ((((((3.0 * locals.var_alpha_dn4) + locals.var_t0_dn4) * locals.var_mu) + (assign101510_e153686 * locals.var_mu_dn4)) * locals.var_mu) + (assign101510_e153688 * locals.var_mu_dn4))))) * assign101510_e153703) - (assign101510_e153692 * (((((((15.0 * locals.var_lch_dn4) * assign101510_e153698) + (assign101510_e153695 * locals.var_alpha_dn4)) * locals.var_mu_ave) + (assign101510_e153699 * locals.var_mu_ave_dn4)) * locals.var_mu_ave) + (assign101510_e153701 * locals.var_mu_ave_dn4)))) / (assign101510_e153703 * assign101510_e153703)), ((((((((((locals.var_weff_nf * locals.var_cox_dn5) * locals.var_vgvt) + (assign101510_e153648 * locals.var_vgvt_dn5)) * locals.var_mu) + (assign101510_e153650 * locals.var_mu_dn5)) * assign101510_e153691) + (assign101510_e153652 * ((((((((3.0 * locals.var_alpha_dn5) + (6.0 * locals.var_t0_dn5)) * locals.var_mud_hoso) + (assign101510_e153661 * locals.var_mud_hoso_dn5)) * locals.var_mud_hoso) + (assign101510_e153663 * locals.var_mud_hoso_dn5)) + ((((((4.0 * locals.var_alpha_dn5) + (3.0 * locals.var_t0_dn5)) * locals.var_mud_hoso) + (assign101510_e153674 * locals.var_mud_hoso_dn5)) * locals.var_mu) + (assign101510_e153676 * locals.var_mu_dn5))) + ((((((3.0 * locals.var_alpha_dn5) + locals.var_t0_dn5) * locals.var_mu) + (assign101510_e153686 * locals.var_mu_dn5)) * locals.var_mu) + (assign101510_e153688 * locals.var_mu_dn5))))) * assign101510_e153703) - (assign101510_e153692 * (((((((15.0 * locals.var_lch_dn5) * assign101510_e153698) + (assign101510_e153695 * locals.var_alpha_dn5)) * locals.var_mu_ave) + (assign101510_e153699 * locals.var_mu_ave_dn5)) * locals.var_mu_ave) + (assign101510_e153701 * locals.var_mu_ave_dn5)))) / (assign101510_e153703 * assign101510_e153703)), ((((((((((locals.var_weff_nf * locals.var_cox_dn6) * locals.var_vgvt) + (assign101510_e153648 * locals.var_vgvt_dn6)) * locals.var_mu) + (assign101510_e153650 * locals.var_mu_dn6)) * assign101510_e153691) + (assign101510_e153652 * ((((((((3.0 * locals.var_alpha_dn6) + (6.0 * locals.var_t0_dn6)) * locals.var_mud_hoso) + (assign101510_e153661 * locals.var_mud_hoso_dn6)) * locals.var_mud_hoso) + (assign101510_e153663 * locals.var_mud_hoso_dn6)) + ((((((4.0 * locals.var_alpha_dn6) + (3.0 * locals.var_t0_dn6)) * locals.var_mud_hoso) + (assign101510_e153674 * locals.var_mud_hoso_dn6)) * locals.var_mu) + (assign101510_e153676 * locals.var_mu_dn6))) + ((((((3.0 * locals.var_alpha_dn6) + locals.var_t0_dn6) * locals.var_mu) + (assign101510_e153686 * locals.var_mu_dn6)) * locals.var_mu) + (assign101510_e153688 * locals.var_mu_dn6))))) * assign101510_e153703) - (assign101510_e153692 * (((((((15.0 * locals.var_lch_dn6) * assign101510_e153698) + (assign101510_e153695 * locals.var_alpha_dn6)) * locals.var_mu_ave) + (assign101510_e153699 * locals.var_mu_ave_dn6)) * locals.var_mu_ave) + (assign101510_e153701 * locals.var_mu_ave_dn6)))) / (assign101510_e153703 * assign101510_e153703)), ((((((((((locals.var_weff_nf * locals.var_cox_dn7) * locals.var_vgvt) + (assign101510_e153648 * locals.var_vgvt_dn7)) * locals.var_mu) + (assign101510_e153650 * locals.var_mu_dn7)) * assign101510_e153691) + (assign101510_e153652 * ((((((((3.0 * locals.var_alpha_dn7) + (6.0 * locals.var_t0_dn7)) * locals.var_mud_hoso) + (assign101510_e153661 * locals.var_mud_hoso_dn7)) * locals.var_mud_hoso) + (assign101510_e153663 * locals.var_mud_hoso_dn7)) + ((((((4.0 * locals.var_alpha_dn7) + (3.0 * locals.var_t0_dn7)) * locals.var_mud_hoso) + (assign101510_e153674 * locals.var_mud_hoso_dn7)) * locals.var_mu) + (assign101510_e153676 * locals.var_mu_dn7))) + ((((((3.0 * locals.var_alpha_dn7) + locals.var_t0_dn7) * locals.var_mu) + (assign101510_e153686 * locals.var_mu_dn7)) * locals.var_mu) + (assign101510_e153688 * locals.var_mu_dn7))))) * assign101510_e153703) - (assign101510_e153692 * (((((((15.0 * locals.var_lch_dn7) * assign101510_e153698) + (assign101510_e153695 * locals.var_alpha_dn7)) * locals.var_mu_ave) + (assign101510_e153699 * locals.var_mu_ave_dn7)) * locals.var_mu_ave) + (assign101510_e153701 * locals.var_mu_ave_dn7)))) / (assign101510_e153703 * assign101510_e153703)), ((((((((((locals.var_weff_nf * locals.var_cox_dn8) * locals.var_vgvt) + (assign101510_e153648 * locals.var_vgvt_dn8)) * locals.var_mu) + (assign101510_e153650 * locals.var_mu_dn8)) * assign101510_e153691) + (assign101510_e153652 * ((((((((3.0 * locals.var_alpha_dn8) + (6.0 * locals.var_t0_dn8)) * locals.var_mud_hoso) + (assign101510_e153661 * locals.var_mud_hoso_dn8)) * locals.var_mud_hoso) + (assign101510_e153663 * locals.var_mud_hoso_dn8)) + ((((((4.0 * locals.var_alpha_dn8) + (3.0 * locals.var_t0_dn8)) * locals.var_mud_hoso) + (assign101510_e153674 * locals.var_mud_hoso_dn8)) * locals.var_mu) + (assign101510_e153676 * locals.var_mu_dn8))) + ((((((3.0 * locals.var_alpha_dn8) + locals.var_t0_dn8) * locals.var_mu) + (assign101510_e153686 * locals.var_mu_dn8)) * locals.var_mu) + (assign101510_e153688 * locals.var_mu_dn8))))) * assign101510_e153703) - (assign101510_e153692 * (((((((15.0 * locals.var_lch_dn8) * assign101510_e153698) + (assign101510_e153695 * locals.var_alpha_dn8)) * locals.var_mu_ave) + (assign101510_e153699 * locals.var_mu_ave_dn8)) * locals.var_mu_ave) + (assign101510_e153701 * locals.var_mu_ave_dn8)))) / (assign101510_e153703 * assign101510_e153703)), ((((((((((locals.var_weff_nf * locals.var_cox_dn9) * locals.var_vgvt) + (assign101510_e153648 * locals.var_vgvt_dn9)) * locals.var_mu) + (assign101510_e153650 * locals.var_mu_dn9)) * assign101510_e153691) + (assign101510_e153652 * ((((((((3.0 * locals.var_alpha_dn9) + (6.0 * locals.var_t0_dn9)) * locals.var_mud_hoso) + (assign101510_e153661 * locals.var_mud_hoso_dn9)) * locals.var_mud_hoso) + (assign101510_e153663 * locals.var_mud_hoso_dn9)) + ((((((4.0 * locals.var_alpha_dn9) + (3.0 * locals.var_t0_dn9)) * locals.var_mud_hoso) + (assign101510_e153674 * locals.var_mud_hoso_dn9)) * locals.var_mu) + (assign101510_e153676 * locals.var_mu_dn9))) + ((((((3.0 * locals.var_alpha_dn9) + locals.var_t0_dn9) * locals.var_mu) + (assign101510_e153686 * locals.var_mu_dn9)) * locals.var_mu) + (assign101510_e153688 * locals.var_mu_dn9))))) * assign101510_e153703) - (assign101510_e153692 * (((((((15.0 * locals.var_lch_dn9) * assign101510_e153698) + (assign101510_e153695 * locals.var_alpha_dn9)) * locals.var_mu_ave) + (assign101510_e153699 * locals.var_mu_ave_dn9)) * locals.var_mu_ave) + (assign101510_e153701 * locals.var_mu_ave_dn9)))) / (assign101510_e153703 * assign101510_e153703)), ((((((((((locals.var_weff_nf * locals.var_cox_dn10) * locals.var_vgvt) + (assign101510_e153648 * locals.var_vgvt_dn10)) * locals.var_mu) + (assign101510_e153650 * locals.var_mu_dn10)) * assign101510_e153691) + (assign101510_e153652 * ((((((((3.0 * locals.var_alpha_dn10) + (6.0 * locals.var_t0_dn10)) * locals.var_mud_hoso) + (assign101510_e153661 * locals.var_mud_hoso_dn10)) * locals.var_mud_hoso) + (assign101510_e153663 * locals.var_mud_hoso_dn10)) + ((((((4.0 * locals.var_alpha_dn10) + (3.0 * locals.var_t0_dn10)) * locals.var_mud_hoso) + (assign101510_e153674 * locals.var_mud_hoso_dn10)) * locals.var_mu) + (assign101510_e153676 * locals.var_mu_dn10))) + ((((((3.0 * locals.var_alpha_dn10) + locals.var_t0_dn10) * locals.var_mu) + (assign101510_e153686 * locals.var_mu_dn10)) * locals.var_mu) + (assign101510_e153688 * locals.var_mu_dn10))))) * assign101510_e153703) - (assign101510_e153692 * (((((((15.0 * locals.var_lch_dn10) * assign101510_e153698) + (assign101510_e153695 * locals.var_alpha_dn10)) * locals.var_mu_ave) + (assign101510_e153699 * locals.var_mu_ave_dn10)) * locals.var_mu_ave) + (assign101510_e153701 * locals.var_mu_ave_dn10)))) / (assign101510_e153703 * assign101510_e153703)), ((((((((((locals.var_weff_nf * locals.var_cox_dn11) * locals.var_vgvt) + (assign101510_e153648 * locals.var_vgvt_dn11)) * locals.var_mu) + (assign101510_e153650 * locals.var_mu_dn11)) * assign101510_e153691) + (assign101510_e153652 * ((((((((3.0 * locals.var_alpha_dn11) + (6.0 * locals.var_t0_dn11)) * locals.var_mud_hoso) + (assign101510_e153661 * locals.var_mud_hoso_dn11)) * locals.var_mud_hoso) + (assign101510_e153663 * locals.var_mud_hoso_dn11)) + ((((((4.0 * locals.var_alpha_dn11) + (3.0 * locals.var_t0_dn11)) * locals.var_mud_hoso) + (assign101510_e153674 * locals.var_mud_hoso_dn11)) * locals.var_mu) + (assign101510_e153676 * locals.var_mu_dn11))) + ((((((3.0 * locals.var_alpha_dn11) + locals.var_t0_dn11) * locals.var_mu) + (assign101510_e153686 * locals.var_mu_dn11)) * locals.var_mu) + (assign101510_e153688 * locals.var_mu_dn11))))) * assign101510_e153703) - (assign101510_e153692 * (((((((15.0 * locals.var_lch_dn11) * assign101510_e153698) + (assign101510_e153695 * locals.var_alpha_dn11)) * locals.var_mu_ave) + (assign101510_e153699 * locals.var_mu_ave_dn11)) * locals.var_mu_ave) + (assign101510_e153701 * locals.var_mu_ave_dn11)))) / (assign101510_e153703 * assign101510_e153703)), ((((((((((locals.var_weff_nf * locals.var_cox_dn14) * locals.var_vgvt) + (assign101510_e153648 * locals.var_vgvt_dn14)) * locals.var_mu) + (assign101510_e153650 * locals.var_mu_dn14)) * assign101510_e153691) + (assign101510_e153652 * ((((((((3.0 * locals.var_alpha_dn14) + (6.0 * locals.var_t0_dn14)) * locals.var_mud_hoso) + (assign101510_e153661 * locals.var_mud_hoso_dn14)) * locals.var_mud_hoso) + (assign101510_e153663 * locals.var_mud_hoso_dn14)) + ((((((4.0 * locals.var_alpha_dn14) + (3.0 * locals.var_t0_dn14)) * locals.var_mud_hoso) + (assign101510_e153674 * locals.var_mud_hoso_dn14)) * locals.var_mu) + (assign101510_e153676 * locals.var_mu_dn14))) + ((((((3.0 * locals.var_alpha_dn14) + locals.var_t0_dn14) * locals.var_mu) + (assign101510_e153686 * locals.var_mu_dn14)) * locals.var_mu) + (assign101510_e153688 * locals.var_mu_dn14))))) * assign101510_e153703) - (assign101510_e153692 * (((((((15.0 * locals.var_lch_dn14) * assign101510_e153698) + (assign101510_e153695 * locals.var_alpha_dn14)) * locals.var_mu_ave) + (assign101510_e153699 * locals.var_mu_ave_dn14)) * locals.var_mu_ave) + (assign101510_e153701 * locals.var_mu_ave_dn14)))) / (assign101510_e153703 * assign101510_e153703)),)
    } else {
        (locals.var_nthrml, locals.var_nthrml_dn0, locals.var_nthrml_dn2, locals.var_nthrml_dn4, locals.var_nthrml_dn5, locals.var_nthrml_dn6, locals.var_nthrml_dn7, locals.var_nthrml_dn8, locals.var_nthrml_dn9, locals.var_nthrml_dn10, locals.var_nthrml_dn11, locals.var_nthrml_dn14,)
    }
};
        locals.var_nthrml = assign101510_e153706;
        locals.var_nthrml_dn0 = assign101510_e153706_d_n0;
        locals.var_nthrml_dn2 = assign101510_e153706_d_n2;
        locals.var_nthrml_dn4 = assign101510_e153706_d_n4;
        locals.var_nthrml_dn5 = assign101510_e153706_d_n5;
        locals.var_nthrml_dn6 = assign101510_e153706_d_n6;
        locals.var_nthrml_dn7 = assign101510_e153706_d_n7;
        locals.var_nthrml_dn8 = assign101510_e153706_d_n8;
        locals.var_nthrml_dn9 = assign101510_e153706_d_n9;
        locals.var_nthrml_dn10 = assign101510_e153706_d_n10;
        locals.var_nthrml_dn11 = assign101510_e153706_d_n11;
        locals.var_nthrml_dn14 = assign101510_e153706_d_n14;

        let (assign101520_e153711, assign101520_e153711_d_n0, assign101520_e153711_d_n2, assign101520_e153711_d_n4, assign101520_e153711_d_n5, assign101520_e153711_d_n6, assign101520_e153711_d_n7, assign101520_e153711_d_n8, assign101520_e153711_d_n9, assign101520_e153711_d_n10, assign101520_e153711_d_n11, assign101520_e153711_d_n14,) = {
    if (locals.var_guard2327 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_nthrml, locals.var_nthrml_dn0, locals.var_nthrml_dn2, locals.var_nthrml_dn4, locals.var_nthrml_dn5, locals.var_nthrml_dn6, locals.var_nthrml_dn7, locals.var_nthrml_dn8, locals.var_nthrml_dn9, locals.var_nthrml_dn10, locals.var_nthrml_dn11, locals.var_nthrml_dn14,)
    }
};
        locals.var_nthrml = assign101520_e153711;
        locals.var_nthrml_dn0 = assign101520_e153711_d_n0;
        locals.var_nthrml_dn2 = assign101520_e153711_d_n2;
        locals.var_nthrml_dn4 = assign101520_e153711_d_n4;
        locals.var_nthrml_dn5 = assign101520_e153711_d_n5;
        locals.var_nthrml_dn6 = assign101520_e153711_d_n6;
        locals.var_nthrml_dn7 = assign101520_e153711_d_n7;
        locals.var_nthrml_dn8 = assign101520_e153711_d_n8;
        locals.var_nthrml_dn9 = assign101520_e153711_d_n9;
        locals.var_nthrml_dn10 = assign101520_e153711_d_n10;
        locals.var_nthrml_dn11 = assign101520_e153711_d_n11;
        locals.var_nthrml_dn14 = assign101520_e153711_d_n14;

        let assign101530_e153729: f64 = if (((((p.p31 != 0.0) && (p.p30 != 0.0)) && (locals.var_flg_ign == 1.0)) && (locals.var_flg_noqi == 0.0)) && (locals.var_uc_codep == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2330 = assign101530_e153729;

        let (assign101540_e153734, assign101540_e153734_d_n0, assign101540_e153734_d_n2, assign101540_e153734_d_n4, assign101540_e153734_d_n5, assign101540_e153734_d_n6, assign101540_e153734_d_n7, assign101540_e153734_d_n8, assign101540_e153734_d_n9, assign101540_e153734_d_n10, assign101540_e153734_d_n11, assign101540_e153734_d_n14,) = {
    if (locals.var_guard2330 != 0.0) {
        let assign101540_e153732: f64 = (locals.var_kusail).sqrt();
        (assign101540_e153732, (locals.var_kusail_dn0 / (2.0 * assign101540_e153732)), (locals.var_kusail_dn2 / (2.0 * assign101540_e153732)), (locals.var_kusail_dn4 / (2.0 * assign101540_e153732)), (locals.var_kusail_dn5 / (2.0 * assign101540_e153732)), (locals.var_kusail_dn6 / (2.0 * assign101540_e153732)), (locals.var_kusail_dn7 / (2.0 * assign101540_e153732)), (locals.var_kusail_dn8 / (2.0 * assign101540_e153732)), (locals.var_kusail_dn9 / (2.0 * assign101540_e153732)), (locals.var_kusail_dn10 / (2.0 * assign101540_e153732)), (locals.var_kusail_dn11 / (2.0 * assign101540_e153732)), (locals.var_kusail_dn14 / (2.0 * assign101540_e153732)),)
    } else {
        (locals.var_sqrtkusail, locals.var_sqrtkusail_dn0, locals.var_sqrtkusail_dn2, locals.var_sqrtkusail_dn4, locals.var_sqrtkusail_dn5, locals.var_sqrtkusail_dn6, locals.var_sqrtkusail_dn7, locals.var_sqrtkusail_dn8, locals.var_sqrtkusail_dn9, locals.var_sqrtkusail_dn10, locals.var_sqrtkusail_dn11, locals.var_sqrtkusail_dn14,)
    }
};
        locals.var_sqrtkusail = assign101540_e153734;
        locals.var_sqrtkusail_dn0 = assign101540_e153734_d_n0;
        locals.var_sqrtkusail_dn2 = assign101540_e153734_d_n2;
        locals.var_sqrtkusail_dn4 = assign101540_e153734_d_n4;
        locals.var_sqrtkusail_dn5 = assign101540_e153734_d_n5;
        locals.var_sqrtkusail_dn6 = assign101540_e153734_d_n6;
        locals.var_sqrtkusail_dn7 = assign101540_e153734_d_n7;
        locals.var_sqrtkusail_dn8 = assign101540_e153734_d_n8;
        locals.var_sqrtkusail_dn9 = assign101540_e153734_d_n9;
        locals.var_sqrtkusail_dn10 = assign101540_e153734_d_n10;
        locals.var_sqrtkusail_dn11 = assign101540_e153734_d_n11;
        locals.var_sqrtkusail_dn14 = assign101540_e153734_d_n14;

        let (assign101550_e153740, assign101550_e153740_d_n0, assign101550_e153740_d_n2, assign101550_e153740_d_n4, assign101550_e153740_d_n5, assign101550_e153740_d_n6, assign101550_e153740_d_n7, assign101550_e153740_d_n8, assign101550_e153740_d_n9, assign101550_e153740_d_n10, assign101550_e153740_d_n11, assign101550_e153740_d_n14,) = {
    if (locals.var_guard2330 != 0.0) {
        let assign101550_e153738: f64 = (locals.var_vgvt + locals.var_sqrtkusail);
        (assign101550_e153738, (locals.var_vgvt_dn0 + locals.var_sqrtkusail_dn0), (locals.var_vgvt_dn2 + locals.var_sqrtkusail_dn2), (locals.var_vgvt_dn4 + locals.var_sqrtkusail_dn4), (locals.var_vgvt_dn5 + locals.var_sqrtkusail_dn5), (locals.var_vgvt_dn6 + locals.var_sqrtkusail_dn6), (locals.var_vgvt_dn7 + locals.var_sqrtkusail_dn7), (locals.var_vgvt_dn8 + locals.var_sqrtkusail_dn8), (locals.var_vgvt_dn9 + locals.var_sqrtkusail_dn9), (locals.var_vgvt_dn10 + locals.var_sqrtkusail_dn10), (locals.var_vgvt_dn11 + locals.var_sqrtkusail_dn11), (locals.var_vgvt_dn14 + locals.var_sqrtkusail_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign101550_e153740;
        locals.var_t2_dn0 = assign101550_e153740_d_n0;
        locals.var_t2_dn2 = assign101550_e153740_d_n2;
        locals.var_t2_dn4 = assign101550_e153740_d_n4;
        locals.var_t2_dn5 = assign101550_e153740_d_n5;
        locals.var_t2_dn6 = assign101550_e153740_d_n6;
        locals.var_t2_dn7 = assign101550_e153740_d_n7;
        locals.var_t2_dn8 = assign101550_e153740_d_n8;
        locals.var_t2_dn9 = assign101550_e153740_d_n9;
        locals.var_t2_dn10 = assign101550_e153740_d_n10;
        locals.var_t2_dn11 = assign101550_e153740_d_n11;
        locals.var_t2_dn14 = assign101550_e153740_d_n14;

        let (assign101560_e153746, assign101560_e153746_d_n0, assign101560_e153746_d_n2, assign101560_e153746_d_n4, assign101560_e153746_d_n5, assign101560_e153746_d_n6, assign101560_e153746_d_n7, assign101560_e153746_d_n8, assign101560_e153746_d_n9, assign101560_e153746_d_n10, assign101560_e153746_d_n11, assign101560_e153746_d_n14,) = {
    if (locals.var_guard2330 != 0.0) {
        let assign101560_e153744: f64 = (locals.var_kusai00 * locals.var_kusai00);
        (assign101560_e153744, ((locals.var_kusai00_dn0 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn0)), ((locals.var_kusai00_dn2 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn2)), ((locals.var_kusai00_dn4 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn4)), ((locals.var_kusai00_dn5 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn5)), ((locals.var_kusai00_dn6 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn6)), ((locals.var_kusai00_dn7 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn7)), ((locals.var_kusai00_dn8 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn8)), ((locals.var_kusai00_dn9 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn9)), ((locals.var_kusai00_dn10 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn10)), ((locals.var_kusai00_dn11 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn11)), ((locals.var_kusai00_dn14 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign101560_e153746;
        locals.var_t3_dn0 = assign101560_e153746_d_n0;
        locals.var_t3_dn2 = assign101560_e153746_d_n2;
        locals.var_t3_dn4 = assign101560_e153746_d_n4;
        locals.var_t3_dn5 = assign101560_e153746_d_n5;
        locals.var_t3_dn6 = assign101560_e153746_d_n6;
        locals.var_t3_dn7 = assign101560_e153746_d_n7;
        locals.var_t3_dn8 = assign101560_e153746_d_n8;
        locals.var_t3_dn9 = assign101560_e153746_d_n9;
        locals.var_t3_dn10 = assign101560_e153746_d_n10;
        locals.var_t3_dn11 = assign101560_e153746_d_n11;
        locals.var_t3_dn14 = assign101560_e153746_d_n14;

        let (assign101570_e153752, assign101570_e153752_d_n0, assign101570_e153752_d_n2, assign101570_e153752_d_n4, assign101570_e153752_d_n5, assign101570_e153752_d_n6, assign101570_e153752_d_n7, assign101570_e153752_d_n8, assign101570_e153752_d_n9, assign101570_e153752_d_n10, assign101570_e153752_d_n11, assign101570_e153752_d_n14,) = {
    if (locals.var_guard2330 != 0.0) {
        let assign101570_e153750: f64 = (locals.var_kusail * locals.var_kusail);
        (assign101570_e153750, ((locals.var_kusail_dn0 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn0)), ((locals.var_kusail_dn2 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn2)), ((locals.var_kusail_dn4 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn4)), ((locals.var_kusail_dn5 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn5)), ((locals.var_kusail_dn6 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn6)), ((locals.var_kusail_dn7 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn7)), ((locals.var_kusail_dn8 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn8)), ((locals.var_kusail_dn9 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn9)), ((locals.var_kusail_dn10 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn10)), ((locals.var_kusail_dn11 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn11)), ((locals.var_kusail_dn14 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign101570_e153752;
        locals.var_t4_dn0 = assign101570_e153752_d_n0;
        locals.var_t4_dn2 = assign101570_e153752_d_n2;
        locals.var_t4_dn4 = assign101570_e153752_d_n4;
        locals.var_t4_dn5 = assign101570_e153752_d_n5;
        locals.var_t4_dn6 = assign101570_e153752_d_n6;
        locals.var_t4_dn7 = assign101570_e153752_d_n7;
        locals.var_t4_dn8 = assign101570_e153752_d_n8;
        locals.var_t4_dn9 = assign101570_e153752_d_n9;
        locals.var_t4_dn10 = assign101570_e153752_d_n10;
        locals.var_t4_dn11 = assign101570_e153752_d_n11;
        locals.var_t4_dn14 = assign101570_e153752_d_n14;

    }

    pub(super) fn stamp_transient_block_372(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign101580_e153760, assign101580_e153760_d_n0, assign101580_e153760_d_n2, assign101580_e153760_d_n4, assign101580_e153760_d_n5, assign101580_e153760_d_n6, assign101580_e153760_d_n7, assign101580_e153760_d_n8, assign101580_e153760_d_n9, assign101580_e153760_d_n10, assign101580_e153760_d_n11, assign101580_e153760_d_n14,) = {
    if (locals.var_guard2330 != 0.0) {
        let assign101580_e153756: f64 = (42.0 * locals.var_kusai00);
        let assign101580_e153758: f64 = (assign101580_e153756 * locals.var_kusail);
        (assign101580_e153758, (((42.0 * locals.var_kusai00_dn0) * locals.var_kusail) + (assign101580_e153756 * locals.var_kusail_dn0)), (((42.0 * locals.var_kusai00_dn2) * locals.var_kusail) + (assign101580_e153756 * locals.var_kusail_dn2)), (((42.0 * locals.var_kusai00_dn4) * locals.var_kusail) + (assign101580_e153756 * locals.var_kusail_dn4)), (((42.0 * locals.var_kusai00_dn5) * locals.var_kusail) + (assign101580_e153756 * locals.var_kusail_dn5)), (((42.0 * locals.var_kusai00_dn6) * locals.var_kusail) + (assign101580_e153756 * locals.var_kusail_dn6)), (((42.0 * locals.var_kusai00_dn7) * locals.var_kusail) + (assign101580_e153756 * locals.var_kusail_dn7)), (((42.0 * locals.var_kusai00_dn8) * locals.var_kusail) + (assign101580_e153756 * locals.var_kusail_dn8)), (((42.0 * locals.var_kusai00_dn9) * locals.var_kusail) + (assign101580_e153756 * locals.var_kusail_dn9)), (((42.0 * locals.var_kusai00_dn10) * locals.var_kusail) + (assign101580_e153756 * locals.var_kusail_dn10)), (((42.0 * locals.var_kusai00_dn11) * locals.var_kusail) + (assign101580_e153756 * locals.var_kusail_dn11)), (((42.0 * locals.var_kusai00_dn14) * locals.var_kusail) + (assign101580_e153756 * locals.var_kusail_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign101580_e153760;
        locals.var_t5_dn0 = assign101580_e153760_d_n0;
        locals.var_t5_dn2 = assign101580_e153760_d_n2;
        locals.var_t5_dn4 = assign101580_e153760_d_n4;
        locals.var_t5_dn5 = assign101580_e153760_d_n5;
        locals.var_t5_dn6 = assign101580_e153760_d_n6;
        locals.var_t5_dn7 = assign101580_e153760_d_n7;
        locals.var_t5_dn8 = assign101580_e153760_d_n8;
        locals.var_t5_dn9 = assign101580_e153760_d_n9;
        locals.var_t5_dn10 = assign101580_e153760_d_n10;
        locals.var_t5_dn11 = assign101580_e153760_d_n11;
        locals.var_t5_dn14 = assign101580_e153760_d_n14;

        let (assign101590_e153770, assign101590_e153770_d_n0, assign101590_e153770_d_n2, assign101590_e153770_d_n4, assign101590_e153770_d_n5, assign101590_e153770_d_n6, assign101590_e153770_d_n7, assign101590_e153770_d_n8, assign101590_e153770_d_n9, assign101590_e153770_d_n10, assign101590_e153770_d_n11, assign101590_e153770_d_n14,) = {
    if (locals.var_guard2330 != 0.0) {
        let assign101590_e153766: f64 = (locals.var_t3 + locals.var_t4);
        let assign101590_e153767: f64 = (4.0 * assign101590_e153766);
        let assign101590_e153768: f64 = (locals.var_t5 + assign101590_e153767);
        (assign101590_e153768, (locals.var_t5_dn0 + (4.0 * (locals.var_t3_dn0 + locals.var_t4_dn0))), (locals.var_t5_dn2 + (4.0 * (locals.var_t3_dn2 + locals.var_t4_dn2))), (locals.var_t5_dn4 + (4.0 * (locals.var_t3_dn4 + locals.var_t4_dn4))), (locals.var_t5_dn5 + (4.0 * (locals.var_t3_dn5 + locals.var_t4_dn5))), (locals.var_t5_dn6 + (4.0 * (locals.var_t3_dn6 + locals.var_t4_dn6))), (locals.var_t5_dn7 + (4.0 * (locals.var_t3_dn7 + locals.var_t4_dn7))), (locals.var_t5_dn8 + (4.0 * (locals.var_t3_dn8 + locals.var_t4_dn8))), (locals.var_t5_dn9 + (4.0 * (locals.var_t3_dn9 + locals.var_t4_dn9))), (locals.var_t5_dn10 + (4.0 * (locals.var_t3_dn10 + locals.var_t4_dn10))), (locals.var_t5_dn11 + (4.0 * (locals.var_t3_dn11 + locals.var_t4_dn11))), (locals.var_t5_dn14 + (4.0 * (locals.var_t3_dn14 + locals.var_t4_dn14))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign101590_e153770;
        locals.var_t5_dn0 = assign101590_e153770_d_n0;
        locals.var_t5_dn2 = assign101590_e153770_d_n2;
        locals.var_t5_dn4 = assign101590_e153770_d_n4;
        locals.var_t5_dn5 = assign101590_e153770_d_n5;
        locals.var_t5_dn6 = assign101590_e153770_d_n6;
        locals.var_t5_dn7 = assign101590_e153770_d_n7;
        locals.var_t5_dn8 = assign101590_e153770_d_n8;
        locals.var_t5_dn9 = assign101590_e153770_d_n9;
        locals.var_t5_dn10 = assign101590_e153770_d_n10;
        locals.var_t5_dn11 = assign101590_e153770_d_n11;
        locals.var_t5_dn14 = assign101590_e153770_d_n14;

        let (assign101600_e153784, assign101600_e153784_d_n0, assign101600_e153784_d_n2, assign101600_e153784_d_n4, assign101600_e153784_d_n5, assign101600_e153784_d_n6, assign101600_e153784_d_n7, assign101600_e153784_d_n8, assign101600_e153784_d_n9, assign101600_e153784_d_n10, assign101600_e153784_d_n11, assign101600_e153784_d_n14,) = {
    if (locals.var_guard2330 != 0.0) {
        let assign101600_e153775: f64 = (20.0 * locals.var_sqrtkusail);
        let assign101600_e153777: f64 = (assign101600_e153775 * locals.var_vgvt);
        let assign101600_e153780: f64 = (locals.var_kusai00 + locals.var_kusail);
        let assign101600_e153781: f64 = (assign101600_e153777 * assign101600_e153780);
        let assign101600_e153782: f64 = (locals.var_t5 + assign101600_e153781);
        (assign101600_e153782, (locals.var_t5_dn0 + (((((20.0 * locals.var_sqrtkusail_dn0) * locals.var_vgvt) + (assign101600_e153775 * locals.var_vgvt_dn0)) * assign101600_e153780) + (assign101600_e153777 * (locals.var_kusai00_dn0 + locals.var_kusail_dn0)))), (locals.var_t5_dn2 + (((((20.0 * locals.var_sqrtkusail_dn2) * locals.var_vgvt) + (assign101600_e153775 * locals.var_vgvt_dn2)) * assign101600_e153780) + (assign101600_e153777 * (locals.var_kusai00_dn2 + locals.var_kusail_dn2)))), (locals.var_t5_dn4 + (((((20.0 * locals.var_sqrtkusail_dn4) * locals.var_vgvt) + (assign101600_e153775 * locals.var_vgvt_dn4)) * assign101600_e153780) + (assign101600_e153777 * (locals.var_kusai00_dn4 + locals.var_kusail_dn4)))), (locals.var_t5_dn5 + (((((20.0 * locals.var_sqrtkusail_dn5) * locals.var_vgvt) + (assign101600_e153775 * locals.var_vgvt_dn5)) * assign101600_e153780) + (assign101600_e153777 * (locals.var_kusai00_dn5 + locals.var_kusail_dn5)))), (locals.var_t5_dn6 + (((((20.0 * locals.var_sqrtkusail_dn6) * locals.var_vgvt) + (assign101600_e153775 * locals.var_vgvt_dn6)) * assign101600_e153780) + (assign101600_e153777 * (locals.var_kusai00_dn6 + locals.var_kusail_dn6)))), (locals.var_t5_dn7 + (((((20.0 * locals.var_sqrtkusail_dn7) * locals.var_vgvt) + (assign101600_e153775 * locals.var_vgvt_dn7)) * assign101600_e153780) + (assign101600_e153777 * (locals.var_kusai00_dn7 + locals.var_kusail_dn7)))), (locals.var_t5_dn8 + (((((20.0 * locals.var_sqrtkusail_dn8) * locals.var_vgvt) + (assign101600_e153775 * locals.var_vgvt_dn8)) * assign101600_e153780) + (assign101600_e153777 * (locals.var_kusai00_dn8 + locals.var_kusail_dn8)))), (locals.var_t5_dn9 + (((((20.0 * locals.var_sqrtkusail_dn9) * locals.var_vgvt) + (assign101600_e153775 * locals.var_vgvt_dn9)) * assign101600_e153780) + (assign101600_e153777 * (locals.var_kusai00_dn9 + locals.var_kusail_dn9)))), (locals.var_t5_dn10 + (((((20.0 * locals.var_sqrtkusail_dn10) * locals.var_vgvt) + (assign101600_e153775 * locals.var_vgvt_dn10)) * assign101600_e153780) + (assign101600_e153777 * (locals.var_kusai00_dn10 + locals.var_kusail_dn10)))), (locals.var_t5_dn11 + (((((20.0 * locals.var_sqrtkusail_dn11) * locals.var_vgvt) + (assign101600_e153775 * locals.var_vgvt_dn11)) * assign101600_e153780) + (assign101600_e153777 * (locals.var_kusai00_dn11 + locals.var_kusail_dn11)))), (locals.var_t5_dn14 + (((((20.0 * locals.var_sqrtkusail_dn14) * locals.var_vgvt) + (assign101600_e153775 * locals.var_vgvt_dn14)) * assign101600_e153780) + (assign101600_e153777 * (locals.var_kusai00_dn14 + locals.var_kusail_dn14)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign101600_e153784;
        locals.var_t5_dn0 = assign101600_e153784_d_n0;
        locals.var_t5_dn2 = assign101600_e153784_d_n2;
        locals.var_t5_dn4 = assign101600_e153784_d_n4;
        locals.var_t5_dn5 = assign101600_e153784_d_n5;
        locals.var_t5_dn6 = assign101600_e153784_d_n6;
        locals.var_t5_dn7 = assign101600_e153784_d_n7;
        locals.var_t5_dn8 = assign101600_e153784_d_n8;
        locals.var_t5_dn9 = assign101600_e153784_d_n9;
        locals.var_t5_dn10 = assign101600_e153784_d_n10;
        locals.var_t5_dn11 = assign101600_e153784_d_n11;
        locals.var_t5_dn14 = assign101600_e153784_d_n14;

        let (assign101610_e153790, assign101610_e153790_d_n0, assign101610_e153790_d_n2, assign101610_e153790_d_n4, assign101610_e153790_d_n5, assign101610_e153790_d_n6, assign101610_e153790_d_n7, assign101610_e153790_d_n8, assign101610_e153790_d_n9, assign101610_e153790_d_n10, assign101610_e153790_d_n11, assign101610_e153790_d_n14,) = {
    if (locals.var_guard2330 != 0.0) {
        let assign101610_e153788: f64 = (locals.var_t2 * locals.var_t2);
        (assign101610_e153788, ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)), ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)), ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)), ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)), ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)), ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)), ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)), ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)), ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)), ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)), ((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign101610_e153790;
        locals.var_t10_dn0 = assign101610_e153790_d_n0;
        locals.var_t10_dn2 = assign101610_e153790_d_n2;
        locals.var_t10_dn4 = assign101610_e153790_d_n4;
        locals.var_t10_dn5 = assign101610_e153790_d_n5;
        locals.var_t10_dn6 = assign101610_e153790_d_n6;
        locals.var_t10_dn7 = assign101610_e153790_d_n7;
        locals.var_t10_dn8 = assign101610_e153790_d_n8;
        locals.var_t10_dn9 = assign101610_e153790_d_n9;
        locals.var_t10_dn10 = assign101610_e153790_d_n10;
        locals.var_t10_dn11 = assign101610_e153790_d_n11;
        locals.var_t10_dn14 = assign101610_e153790_d_n14;

        let (assign101620_e153796, assign101620_e153796_d_n0, assign101620_e153796_d_n2, assign101620_e153796_d_n4, assign101620_e153796_d_n5, assign101620_e153796_d_n6, assign101620_e153796_d_n7, assign101620_e153796_d_n8, assign101620_e153796_d_n9, assign101620_e153796_d_n10, assign101620_e153796_d_n11, assign101620_e153796_d_n14,) = {
    if (locals.var_guard2330 != 0.0) {
        let assign101620_e153794: f64 = (locals.var_t10 * locals.var_t10);
        (assign101620_e153794, ((locals.var_t10_dn0 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn0)), ((locals.var_t10_dn2 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn2)), ((locals.var_t10_dn4 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn4)), ((locals.var_t10_dn5 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn5)), ((locals.var_t10_dn6 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn6)), ((locals.var_t10_dn7 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn7)), ((locals.var_t10_dn8 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn8)), ((locals.var_t10_dn9 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn9)), ((locals.var_t10_dn10 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn10)), ((locals.var_t10_dn11 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn11)), ((locals.var_t10_dn14 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn14)),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign101620_e153796;
        locals.var_t10_dn0 = assign101620_e153796_d_n0;
        locals.var_t10_dn2 = assign101620_e153796_d_n2;
        locals.var_t10_dn4 = assign101620_e153796_d_n4;
        locals.var_t10_dn5 = assign101620_e153796_d_n5;
        locals.var_t10_dn6 = assign101620_e153796_d_n6;
        locals.var_t10_dn7 = assign101620_e153796_d_n7;
        locals.var_t10_dn8 = assign101620_e153796_d_n8;
        locals.var_t10_dn9 = assign101620_e153796_d_n9;
        locals.var_t10_dn10 = assign101620_e153796_d_n10;
        locals.var_t10_dn11 = assign101620_e153796_d_n11;
        locals.var_t10_dn14 = assign101620_e153796_d_n14;

        let (assign101630_e153804, assign101630_e153804_d_n0, assign101630_e153804_d_n2, assign101630_e153804_d_n4, assign101630_e153804_d_n5, assign101630_e153804_d_n6, assign101630_e153804_d_n7, assign101630_e153804_d_n8, assign101630_e153804_d_n9, assign101630_e153804_d_n10, assign101630_e153804_d_n11, assign101630_e153804_d_n14,) = {
    if (locals.var_guard2330 != 0.0) {
        let assign101630_e153801: f64 = (locals.var_t10 * locals.var_t2);
        let assign101630_e153802: f64 = (locals.var_t5 / assign101630_e153801);
        (assign101630_e153802, (((locals.var_t5_dn0 * assign101630_e153801) - (locals.var_t5 * ((locals.var_t10_dn0 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn0)))) / (assign101630_e153801 * assign101630_e153801)), (((locals.var_t5_dn2 * assign101630_e153801) - (locals.var_t5 * ((locals.var_t10_dn2 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn2)))) / (assign101630_e153801 * assign101630_e153801)), (((locals.var_t5_dn4 * assign101630_e153801) - (locals.var_t5 * ((locals.var_t10_dn4 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn4)))) / (assign101630_e153801 * assign101630_e153801)), (((locals.var_t5_dn5 * assign101630_e153801) - (locals.var_t5 * ((locals.var_t10_dn5 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn5)))) / (assign101630_e153801 * assign101630_e153801)), (((locals.var_t5_dn6 * assign101630_e153801) - (locals.var_t5 * ((locals.var_t10_dn6 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn6)))) / (assign101630_e153801 * assign101630_e153801)), (((locals.var_t5_dn7 * assign101630_e153801) - (locals.var_t5 * ((locals.var_t10_dn7 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn7)))) / (assign101630_e153801 * assign101630_e153801)), (((locals.var_t5_dn8 * assign101630_e153801) - (locals.var_t5 * ((locals.var_t10_dn8 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn8)))) / (assign101630_e153801 * assign101630_e153801)), (((locals.var_t5_dn9 * assign101630_e153801) - (locals.var_t5 * ((locals.var_t10_dn9 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn9)))) / (assign101630_e153801 * assign101630_e153801)), (((locals.var_t5_dn10 * assign101630_e153801) - (locals.var_t5 * ((locals.var_t10_dn10 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn10)))) / (assign101630_e153801 * assign101630_e153801)), (((locals.var_t5_dn11 * assign101630_e153801) - (locals.var_t5 * ((locals.var_t10_dn11 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn11)))) / (assign101630_e153801 * assign101630_e153801)), (((locals.var_t5_dn14 * assign101630_e153801) - (locals.var_t5 * ((locals.var_t10_dn14 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn14)))) / (assign101630_e153801 * assign101630_e153801)),)
    } else {
        (locals.var_kusai_ig, locals.var_kusai_ig_dn0, locals.var_kusai_ig_dn2, locals.var_kusai_ig_dn4, locals.var_kusai_ig_dn5, locals.var_kusai_ig_dn6, locals.var_kusai_ig_dn7, locals.var_kusai_ig_dn8, locals.var_kusai_ig_dn9, locals.var_kusai_ig_dn10, locals.var_kusai_ig_dn11, locals.var_kusai_ig_dn14,)
    }
};
        locals.var_kusai_ig = assign101630_e153804;
        locals.var_kusai_ig_dn0 = assign101630_e153804_d_n0;
        locals.var_kusai_ig_dn2 = assign101630_e153804_d_n2;
        locals.var_kusai_ig_dn4 = assign101630_e153804_d_n4;
        locals.var_kusai_ig_dn5 = assign101630_e153804_d_n5;
        locals.var_kusai_ig_dn6 = assign101630_e153804_d_n6;
        locals.var_kusai_ig_dn7 = assign101630_e153804_d_n7;
        locals.var_kusai_ig_dn8 = assign101630_e153804_d_n8;
        locals.var_kusai_ig_dn9 = assign101630_e153804_d_n9;
        locals.var_kusai_ig_dn10 = assign101630_e153804_d_n10;
        locals.var_kusai_ig_dn11 = assign101630_e153804_d_n11;
        locals.var_kusai_ig_dn14 = assign101630_e153804_d_n14;

        let (assign101640_e153814, assign101640_e153814_d_n0, assign101640_e153814_d_n2, assign101640_e153814_d_n4, assign101640_e153814_d_n5, assign101640_e153814_d_n6, assign101640_e153814_d_n7, assign101640_e153814_d_n8, assign101640_e153814_d_n9, assign101640_e153814_d_n10, assign101640_e153814_d_n11, assign101640_e153814_d_n14,) = {
    if (locals.var_guard2330 != 0.0) {
        let assign101640_e153808: f64 = (locals.var_weff_nf / locals.var_lch);
        let assign101640_e153810: f64 = (assign101640_e153808 * locals.var_mu);
        let assign101640_e153812: f64 = (assign101640_e153810 * locals.var_cox);
        (assign101640_e153812, (((((-((locals.var_weff_nf * locals.var_lch_dn0) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101640_e153808 * locals.var_mu_dn0)) * locals.var_cox) + (assign101640_e153810 * locals.var_cox_dn0)), (((((-((locals.var_weff_nf * locals.var_lch_dn2) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101640_e153808 * locals.var_mu_dn2)) * locals.var_cox) + (assign101640_e153810 * locals.var_cox_dn2)), (((((-((locals.var_weff_nf * locals.var_lch_dn4) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101640_e153808 * locals.var_mu_dn4)) * locals.var_cox) + (assign101640_e153810 * locals.var_cox_dn4)), (((((-((locals.var_weff_nf * locals.var_lch_dn5) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101640_e153808 * locals.var_mu_dn5)) * locals.var_cox) + (assign101640_e153810 * locals.var_cox_dn5)), (((((-((locals.var_weff_nf * locals.var_lch_dn6) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101640_e153808 * locals.var_mu_dn6)) * locals.var_cox) + (assign101640_e153810 * locals.var_cox_dn6)), (((((-((locals.var_weff_nf * locals.var_lch_dn7) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101640_e153808 * locals.var_mu_dn7)) * locals.var_cox) + (assign101640_e153810 * locals.var_cox_dn7)), (((((-((locals.var_weff_nf * locals.var_lch_dn8) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101640_e153808 * locals.var_mu_dn8)) * locals.var_cox) + (assign101640_e153810 * locals.var_cox_dn8)), (((((-((locals.var_weff_nf * locals.var_lch_dn9) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101640_e153808 * locals.var_mu_dn9)) * locals.var_cox) + (assign101640_e153810 * locals.var_cox_dn9)), (((((-((locals.var_weff_nf * locals.var_lch_dn10) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101640_e153808 * locals.var_mu_dn10)) * locals.var_cox) + (assign101640_e153810 * locals.var_cox_dn10)), (((((-((locals.var_weff_nf * locals.var_lch_dn11) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101640_e153808 * locals.var_mu_dn11)) * locals.var_cox) + (assign101640_e153810 * locals.var_cox_dn11)), (((((-((locals.var_weff_nf * locals.var_lch_dn14) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101640_e153808 * locals.var_mu_dn14)) * locals.var_cox) + (assign101640_e153810 * locals.var_cox_dn14)),)
    } else {
        (locals.var_gds0_ign, locals.var_gds0_ign_dn0, locals.var_gds0_ign_dn2, locals.var_gds0_ign_dn4, locals.var_gds0_ign_dn5, locals.var_gds0_ign_dn6, locals.var_gds0_ign_dn7, locals.var_gds0_ign_dn8, locals.var_gds0_ign_dn9, locals.var_gds0_ign_dn10, locals.var_gds0_ign_dn11, locals.var_gds0_ign_dn14,)
    }
};
        locals.var_gds0_ign = assign101640_e153814;
        locals.var_gds0_ign_dn0 = assign101640_e153814_d_n0;
        locals.var_gds0_ign_dn2 = assign101640_e153814_d_n2;
        locals.var_gds0_ign_dn4 = assign101640_e153814_d_n4;
        locals.var_gds0_ign_dn5 = assign101640_e153814_d_n5;
        locals.var_gds0_ign_dn6 = assign101640_e153814_d_n6;
        locals.var_gds0_ign_dn7 = assign101640_e153814_d_n7;
        locals.var_gds0_ign_dn8 = assign101640_e153814_d_n8;
        locals.var_gds0_ign_dn9 = assign101640_e153814_d_n9;
        locals.var_gds0_ign_dn10 = assign101640_e153814_d_n10;
        locals.var_gds0_ign_dn11 = assign101640_e153814_d_n11;
        locals.var_gds0_ign_dn14 = assign101640_e153814_d_n14;

        let (assign101650_e153820, assign101650_e153820_d_n0, assign101650_e153820_d_n2, assign101650_e153820_d_n4, assign101650_e153820_d_n5, assign101650_e153820_d_n6, assign101650_e153820_d_n7, assign101650_e153820_d_n8, assign101650_e153820_d_n9, assign101650_e153820_d_n10, assign101650_e153820_d_n11, assign101650_e153820_d_n14,) = {
    if (locals.var_guard2330 != 0.0) {
        let assign101650_e153818: f64 = (locals.var_gds0_ign * locals.var_vgvt);
        (assign101650_e153818, ((locals.var_gds0_ign_dn0 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn0)), ((locals.var_gds0_ign_dn2 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn2)), ((locals.var_gds0_ign_dn4 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn4)), ((locals.var_gds0_ign_dn5 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn5)), ((locals.var_gds0_ign_dn6 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn6)), ((locals.var_gds0_ign_dn7 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn7)), ((locals.var_gds0_ign_dn8 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn8)), ((locals.var_gds0_ign_dn9 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn9)), ((locals.var_gds0_ign_dn10 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn10)), ((locals.var_gds0_ign_dn11 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn11)), ((locals.var_gds0_ign_dn14 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn14)),)
    } else {
        (locals.var_gds0_h2, locals.var_gds0_h2_dn0, locals.var_gds0_h2_dn2, locals.var_gds0_h2_dn4, locals.var_gds0_h2_dn5, locals.var_gds0_h2_dn6, locals.var_gds0_h2_dn7, locals.var_gds0_h2_dn8, locals.var_gds0_h2_dn9, locals.var_gds0_h2_dn10, locals.var_gds0_h2_dn11, locals.var_gds0_h2_dn14,)
    }
};
        locals.var_gds0_h2 = assign101650_e153820;
        locals.var_gds0_h2_dn0 = assign101650_e153820_d_n0;
        locals.var_gds0_h2_dn2 = assign101650_e153820_d_n2;
        locals.var_gds0_h2_dn4 = assign101650_e153820_d_n4;
        locals.var_gds0_h2_dn5 = assign101650_e153820_d_n5;
        locals.var_gds0_h2_dn6 = assign101650_e153820_d_n6;
        locals.var_gds0_h2_dn7 = assign101650_e153820_d_n7;
        locals.var_gds0_h2_dn8 = assign101650_e153820_d_n8;
        locals.var_gds0_h2_dn9 = assign101650_e153820_d_n9;
        locals.var_gds0_h2_dn10 = assign101650_e153820_d_n10;
        locals.var_gds0_h2_dn11 = assign101650_e153820_d_n11;
        locals.var_gds0_h2_dn14 = assign101650_e153820_d_n14;

        let (assign101660_e153826, assign101660_e153826_d_n0, assign101660_e153826_d_n2, assign101660_e153826_d_n4, assign101660_e153826_d_n5, assign101660_e153826_d_n6, assign101660_e153826_d_n7, assign101660_e153826_d_n8, assign101660_e153826_d_n9, assign101660_e153826_d_n10, assign101660_e153826_d_n11, assign101660_e153826_d_n14,) = {
    if (locals.var_guard2330 != 0.0) {
        let assign101660_e153824: f64 = (locals.var_nthrml / locals.var_gds0_h2);
        (assign101660_e153824, (((locals.var_nthrml_dn0 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn0)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn2 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn2)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn4 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn4)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn5 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn5)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn6 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn6)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn7 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn7)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn8 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn8)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn9 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn9)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn10 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn10)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn11 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn11)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn14 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn14)) / (locals.var_gds0_h2 * locals.var_gds0_h2)),)
    } else {
        (locals.var_gamma, locals.var_gamma_dn0, locals.var_gamma_dn2, locals.var_gamma_dn4, locals.var_gamma_dn5, locals.var_gamma_dn6, locals.var_gamma_dn7, locals.var_gamma_dn8, locals.var_gamma_dn9, locals.var_gamma_dn10, locals.var_gamma_dn11, locals.var_gamma_dn14,)
    }
};
        locals.var_gamma = assign101660_e153826;
        locals.var_gamma_dn0 = assign101660_e153826_d_n0;
        locals.var_gamma_dn2 = assign101660_e153826_d_n2;
        locals.var_gamma_dn4 = assign101660_e153826_d_n4;
        locals.var_gamma_dn5 = assign101660_e153826_d_n5;
        locals.var_gamma_dn6 = assign101660_e153826_d_n6;
        locals.var_gamma_dn7 = assign101660_e153826_d_n7;
        locals.var_gamma_dn8 = assign101660_e153826_d_n8;
        locals.var_gamma_dn9 = assign101660_e153826_d_n9;
        locals.var_gamma_dn10 = assign101660_e153826_d_n10;
        locals.var_gamma_dn11 = assign101660_e153826_d_n11;
        locals.var_gamma_dn14 = assign101660_e153826_d_n14;

        let (assign101670_e153838, assign101670_e153838_d_n0, assign101670_e153838_d_n2, assign101670_e153838_d_n4, assign101670_e153838_d_n5, assign101670_e153838_d_n6, assign101670_e153838_d_n7, assign101670_e153838_d_n8, assign101670_e153838_d_n9, assign101670_e153838_d_n10, assign101670_e153838_d_n11, assign101670_e153838_d_n14,) = {
    if (locals.var_guard2330 != 0.0) {
        let assign101670_e153831: f64 = (4.0 * locals.var_vgvt);
        let assign101670_e153833: f64 = (assign101670_e153831 * locals.var_sqrtkusail);
        let assign101670_e153834: f64 = (locals.var_kusai00 + assign101670_e153833);
        let assign101670_e153836: f64 = (assign101670_e153834 + locals.var_kusail);
        (assign101670_e153836, ((locals.var_kusai00_dn0 + (((4.0 * locals.var_vgvt_dn0) * locals.var_sqrtkusail) + (assign101670_e153831 * locals.var_sqrtkusail_dn0))) + locals.var_kusail_dn0), ((locals.var_kusai00_dn2 + (((4.0 * locals.var_vgvt_dn2) * locals.var_sqrtkusail) + (assign101670_e153831 * locals.var_sqrtkusail_dn2))) + locals.var_kusail_dn2), ((locals.var_kusai00_dn4 + (((4.0 * locals.var_vgvt_dn4) * locals.var_sqrtkusail) + (assign101670_e153831 * locals.var_sqrtkusail_dn4))) + locals.var_kusail_dn4), ((locals.var_kusai00_dn5 + (((4.0 * locals.var_vgvt_dn5) * locals.var_sqrtkusail) + (assign101670_e153831 * locals.var_sqrtkusail_dn5))) + locals.var_kusail_dn5), ((locals.var_kusai00_dn6 + (((4.0 * locals.var_vgvt_dn6) * locals.var_sqrtkusail) + (assign101670_e153831 * locals.var_sqrtkusail_dn6))) + locals.var_kusail_dn6), ((locals.var_kusai00_dn7 + (((4.0 * locals.var_vgvt_dn7) * locals.var_sqrtkusail) + (assign101670_e153831 * locals.var_sqrtkusail_dn7))) + locals.var_kusail_dn7), ((locals.var_kusai00_dn8 + (((4.0 * locals.var_vgvt_dn8) * locals.var_sqrtkusail) + (assign101670_e153831 * locals.var_sqrtkusail_dn8))) + locals.var_kusail_dn8), ((locals.var_kusai00_dn9 + (((4.0 * locals.var_vgvt_dn9) * locals.var_sqrtkusail) + (assign101670_e153831 * locals.var_sqrtkusail_dn9))) + locals.var_kusail_dn9), ((locals.var_kusai00_dn10 + (((4.0 * locals.var_vgvt_dn10) * locals.var_sqrtkusail) + (assign101670_e153831 * locals.var_sqrtkusail_dn10))) + locals.var_kusail_dn10), ((locals.var_kusai00_dn11 + (((4.0 * locals.var_vgvt_dn11) * locals.var_sqrtkusail) + (assign101670_e153831 * locals.var_sqrtkusail_dn11))) + locals.var_kusail_dn11), ((locals.var_kusai00_dn14 + (((4.0 * locals.var_vgvt_dn14) * locals.var_sqrtkusail) + (assign101670_e153831 * locals.var_sqrtkusail_dn14))) + locals.var_kusail_dn14),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign101670_e153838;
        locals.var_t7_dn0 = assign101670_e153838_d_n0;
        locals.var_t7_dn2 = assign101670_e153838_d_n2;
        locals.var_t7_dn4 = assign101670_e153838_d_n4;
        locals.var_t7_dn5 = assign101670_e153838_d_n5;
        locals.var_t7_dn6 = assign101670_e153838_d_n6;
        locals.var_t7_dn7 = assign101670_e153838_d_n7;
        locals.var_t7_dn8 = assign101670_e153838_d_n8;
        locals.var_t7_dn9 = assign101670_e153838_d_n9;
        locals.var_t7_dn10 = assign101670_e153838_d_n10;
        locals.var_t7_dn11 = assign101670_e153838_d_n11;
        locals.var_t7_dn14 = assign101670_e153838_d_n14;

        let (assign101680_e153859, assign101680_e153859_d_n0, assign101680_e153859_d_n2, assign101680_e153859_d_n4, assign101680_e153859_d_n5, assign101680_e153859_d_n6, assign101680_e153859_d_n7, assign101680_e153859_d_n8, assign101680_e153859_d_n9, assign101680_e153859_d_n10, assign101680_e153859_d_n11, assign101680_e153859_d_n14,) = {
    if (locals.var_guard2330 != 0.0) {
        let assign101680_e153842: f64 = (3.872983346207417 * locals.var_kusai00l);
        let assign101680_e153844: f64 = (assign101680_e153842 * locals.var_t7);
        let assign101680_e153847: f64 = (6.0 * locals.var_t2);
        let assign101680_e153850: f64 = (locals.var_gamma * locals.var_t2);
        let assign101680_e153852: f64 = (assign101680_e153850 * locals.var_vgvt);
        let assign101680_e153854: f64 = (assign101680_e153852 * locals.var_t5);
        let assign101680_e153855: f64 = (assign101680_e153854).sqrt();
        let assign101680_e153856: f64 = (assign101680_e153847 * assign101680_e153855);
        let assign101680_e153857: f64 = (assign101680_e153844 / assign101680_e153856);
        (assign101680_e153857, ((((((3.872983346207417 * locals.var_kusai00l_dn0) * locals.var_t7) + (assign101680_e153842 * locals.var_t7_dn0)) * assign101680_e153856) - (assign101680_e153844 * (((6.0 * locals.var_t2_dn0) * assign101680_e153855) + (assign101680_e153847 * (((((((locals.var_gamma_dn0 * locals.var_t2) + (locals.var_gamma * locals.var_t2_dn0)) * locals.var_vgvt) + (assign101680_e153850 * locals.var_vgvt_dn0)) * locals.var_t5) + (assign101680_e153852 * locals.var_t5_dn0)) / (2.0 * assign101680_e153855)))))) / (assign101680_e153856 * assign101680_e153856)), ((((((3.872983346207417 * locals.var_kusai00l_dn2) * locals.var_t7) + (assign101680_e153842 * locals.var_t7_dn2)) * assign101680_e153856) - (assign101680_e153844 * (((6.0 * locals.var_t2_dn2) * assign101680_e153855) + (assign101680_e153847 * (((((((locals.var_gamma_dn2 * locals.var_t2) + (locals.var_gamma * locals.var_t2_dn2)) * locals.var_vgvt) + (assign101680_e153850 * locals.var_vgvt_dn2)) * locals.var_t5) + (assign101680_e153852 * locals.var_t5_dn2)) / (2.0 * assign101680_e153855)))))) / (assign101680_e153856 * assign101680_e153856)), ((((((3.872983346207417 * locals.var_kusai00l_dn4) * locals.var_t7) + (assign101680_e153842 * locals.var_t7_dn4)) * assign101680_e153856) - (assign101680_e153844 * (((6.0 * locals.var_t2_dn4) * assign101680_e153855) + (assign101680_e153847 * (((((((locals.var_gamma_dn4 * locals.var_t2) + (locals.var_gamma * locals.var_t2_dn4)) * locals.var_vgvt) + (assign101680_e153850 * locals.var_vgvt_dn4)) * locals.var_t5) + (assign101680_e153852 * locals.var_t5_dn4)) / (2.0 * assign101680_e153855)))))) / (assign101680_e153856 * assign101680_e153856)), ((((((3.872983346207417 * locals.var_kusai00l_dn5) * locals.var_t7) + (assign101680_e153842 * locals.var_t7_dn5)) * assign101680_e153856) - (assign101680_e153844 * (((6.0 * locals.var_t2_dn5) * assign101680_e153855) + (assign101680_e153847 * (((((((locals.var_gamma_dn5 * locals.var_t2) + (locals.var_gamma * locals.var_t2_dn5)) * locals.var_vgvt) + (assign101680_e153850 * locals.var_vgvt_dn5)) * locals.var_t5) + (assign101680_e153852 * locals.var_t5_dn5)) / (2.0 * assign101680_e153855)))))) / (assign101680_e153856 * assign101680_e153856)), ((((((3.872983346207417 * locals.var_kusai00l_dn6) * locals.var_t7) + (assign101680_e153842 * locals.var_t7_dn6)) * assign101680_e153856) - (assign101680_e153844 * (((6.0 * locals.var_t2_dn6) * assign101680_e153855) + (assign101680_e153847 * (((((((locals.var_gamma_dn6 * locals.var_t2) + (locals.var_gamma * locals.var_t2_dn6)) * locals.var_vgvt) + (assign101680_e153850 * locals.var_vgvt_dn6)) * locals.var_t5) + (assign101680_e153852 * locals.var_t5_dn6)) / (2.0 * assign101680_e153855)))))) / (assign101680_e153856 * assign101680_e153856)), ((((((3.872983346207417 * locals.var_kusai00l_dn7) * locals.var_t7) + (assign101680_e153842 * locals.var_t7_dn7)) * assign101680_e153856) - (assign101680_e153844 * (((6.0 * locals.var_t2_dn7) * assign101680_e153855) + (assign101680_e153847 * (((((((locals.var_gamma_dn7 * locals.var_t2) + (locals.var_gamma * locals.var_t2_dn7)) * locals.var_vgvt) + (assign101680_e153850 * locals.var_vgvt_dn7)) * locals.var_t5) + (assign101680_e153852 * locals.var_t5_dn7)) / (2.0 * assign101680_e153855)))))) / (assign101680_e153856 * assign101680_e153856)), ((((((3.872983346207417 * locals.var_kusai00l_dn8) * locals.var_t7) + (assign101680_e153842 * locals.var_t7_dn8)) * assign101680_e153856) - (assign101680_e153844 * (((6.0 * locals.var_t2_dn8) * assign101680_e153855) + (assign101680_e153847 * (((((((locals.var_gamma_dn8 * locals.var_t2) + (locals.var_gamma * locals.var_t2_dn8)) * locals.var_vgvt) + (assign101680_e153850 * locals.var_vgvt_dn8)) * locals.var_t5) + (assign101680_e153852 * locals.var_t5_dn8)) / (2.0 * assign101680_e153855)))))) / (assign101680_e153856 * assign101680_e153856)), ((((((3.872983346207417 * locals.var_kusai00l_dn9) * locals.var_t7) + (assign101680_e153842 * locals.var_t7_dn9)) * assign101680_e153856) - (assign101680_e153844 * (((6.0 * locals.var_t2_dn9) * assign101680_e153855) + (assign101680_e153847 * (((((((locals.var_gamma_dn9 * locals.var_t2) + (locals.var_gamma * locals.var_t2_dn9)) * locals.var_vgvt) + (assign101680_e153850 * locals.var_vgvt_dn9)) * locals.var_t5) + (assign101680_e153852 * locals.var_t5_dn9)) / (2.0 * assign101680_e153855)))))) / (assign101680_e153856 * assign101680_e153856)), ((((((3.872983346207417 * locals.var_kusai00l_dn10) * locals.var_t7) + (assign101680_e153842 * locals.var_t7_dn10)) * assign101680_e153856) - (assign101680_e153844 * (((6.0 * locals.var_t2_dn10) * assign101680_e153855) + (assign101680_e153847 * (((((((locals.var_gamma_dn10 * locals.var_t2) + (locals.var_gamma * locals.var_t2_dn10)) * locals.var_vgvt) + (assign101680_e153850 * locals.var_vgvt_dn10)) * locals.var_t5) + (assign101680_e153852 * locals.var_t5_dn10)) / (2.0 * assign101680_e153855)))))) / (assign101680_e153856 * assign101680_e153856)), ((((((3.872983346207417 * locals.var_kusai00l_dn11) * locals.var_t7) + (assign101680_e153842 * locals.var_t7_dn11)) * assign101680_e153856) - (assign101680_e153844 * (((6.0 * locals.var_t2_dn11) * assign101680_e153855) + (assign101680_e153847 * (((((((locals.var_gamma_dn11 * locals.var_t2) + (locals.var_gamma * locals.var_t2_dn11)) * locals.var_vgvt) + (assign101680_e153850 * locals.var_vgvt_dn11)) * locals.var_t5) + (assign101680_e153852 * locals.var_t5_dn11)) / (2.0 * assign101680_e153855)))))) / (assign101680_e153856 * assign101680_e153856)), ((((((3.872983346207417 * locals.var_kusai00l_dn14) * locals.var_t7) + (assign101680_e153842 * locals.var_t7_dn14)) * assign101680_e153856) - (assign101680_e153844 * (((6.0 * locals.var_t2_dn14) * assign101680_e153855) + (assign101680_e153847 * (((((((locals.var_gamma_dn14 * locals.var_t2) + (locals.var_gamma * locals.var_t2_dn14)) * locals.var_vgvt) + (assign101680_e153850 * locals.var_vgvt_dn14)) * locals.var_t5) + (assign101680_e153852 * locals.var_t5_dn14)) / (2.0 * assign101680_e153855)))))) / (assign101680_e153856 * assign101680_e153856)),)
    } else {
        (locals.var_crl_f, locals.var_crl_f_dn0, locals.var_crl_f_dn2, locals.var_crl_f_dn4, locals.var_crl_f_dn5, locals.var_crl_f_dn6, locals.var_crl_f_dn7, locals.var_crl_f_dn8, locals.var_crl_f_dn9, locals.var_crl_f_dn10, locals.var_crl_f_dn11, locals.var_crl_f_dn14,)
    }
};
        locals.var_crl_f = assign101680_e153859;
        locals.var_crl_f_dn0 = assign101680_e153859_d_n0;
        locals.var_crl_f_dn2 = assign101680_e153859_d_n2;
        locals.var_crl_f_dn4 = assign101680_e153859_d_n4;
        locals.var_crl_f_dn5 = assign101680_e153859_d_n5;
        locals.var_crl_f_dn6 = assign101680_e153859_d_n6;
        locals.var_crl_f_dn7 = assign101680_e153859_d_n7;
        locals.var_crl_f_dn8 = assign101680_e153859_d_n8;
        locals.var_crl_f_dn9 = assign101680_e153859_d_n9;
        locals.var_crl_f_dn10 = assign101680_e153859_d_n10;
        locals.var_crl_f_dn11 = assign101680_e153859_d_n11;
        locals.var_crl_f_dn14 = assign101680_e153859_d_n14;

        let assign101690_e153862: f64 = (locals.var_mfactor * locals.var_ids);
        locals.var_idse = assign101690_e153862;
        locals.var_idse_dn0 = (locals.var_mfactor * locals.var_ids_dn0);
        locals.var_idse_dn2 = (locals.var_mfactor * locals.var_ids_dn2);
        locals.var_idse_dn4 = (locals.var_mfactor * locals.var_ids_dn4);
        locals.var_idse_dn5 = (locals.var_mfactor * locals.var_ids_dn5);
        locals.var_idse_dn6 = (locals.var_mfactor * locals.var_ids_dn6);
        locals.var_idse_dn7 = (locals.var_mfactor * locals.var_ids_dn7);
        locals.var_idse_dn8 = (locals.var_mfactor * locals.var_ids_dn8);
        locals.var_idse_dn9 = (locals.var_mfactor * locals.var_ids_dn9);
        locals.var_idse_dn10 = (locals.var_mfactor * locals.var_ids_dn10);
        locals.var_idse_dn11 = (locals.var_mfactor * locals.var_ids_dn11);
        locals.var_idse_dn14 = (locals.var_mfactor * locals.var_ids_dn14);

        let assign101730_e153874: f64 = (locals.var_mfactor * locals.var_idsibpc);
        locals.var_idsibpce = assign101730_e153874;
        locals.var_idsibpce_dn0 = (locals.var_mfactor * locals.var_idsibpc_dn0);
        locals.var_idsibpce_dn2 = (locals.var_mfactor * locals.var_idsibpc_dn2);
        locals.var_idsibpce_dn4 = (locals.var_mfactor * locals.var_idsibpc_dn4);
        locals.var_idsibpce_dn5 = (locals.var_mfactor * locals.var_idsibpc_dn5);
        locals.var_idsibpce_dn6 = (locals.var_mfactor * locals.var_idsibpc_dn6);
        locals.var_idsibpce_dn7 = (locals.var_mfactor * locals.var_idsibpc_dn7);
        locals.var_idsibpce_dn8 = (locals.var_mfactor * locals.var_idsibpc_dn8);
        locals.var_idsibpce_dn9 = (locals.var_mfactor * locals.var_idsibpc_dn9);
        locals.var_idsibpce_dn10 = (locals.var_mfactor * locals.var_idsibpc_dn10);
        locals.var_idsibpce_dn11 = (locals.var_mfactor * locals.var_idsibpc_dn11);
        locals.var_idsibpce_dn14 = (locals.var_mfactor * locals.var_idsibpc_dn14);

        locals.var_qgexte = 0.0;
        locals.var_qgexte_dn0 = 0.0;
        locals.var_qgexte_dn2 = 0.0;
        locals.var_qgexte_dn4 = 0.0;
        locals.var_qgexte_dn5 = 0.0;
        locals.var_qgexte_dn6 = 0.0;
        locals.var_qgexte_dn7 = 0.0;
        locals.var_qgexte_dn8 = 0.0;
        locals.var_qgexte_dn9 = 0.0;
        locals.var_qgexte_dn10 = 0.0;
        locals.var_qgexte_dn11 = 0.0;
        locals.var_qgexte_dn14 = 0.0;

        locals.var_qdexte = 0.0;
        locals.var_qdexte_dn0 = 0.0;
        locals.var_qdexte_dn2 = 0.0;
        locals.var_qdexte_dn4 = 0.0;
        locals.var_qdexte_dn5 = 0.0;
        locals.var_qdexte_dn6 = 0.0;
        locals.var_qdexte_dn7 = 0.0;
        locals.var_qdexte_dn8 = 0.0;
        locals.var_qdexte_dn9 = 0.0;
        locals.var_qdexte_dn10 = 0.0;
        locals.var_qdexte_dn11 = 0.0;
        locals.var_qdexte_dn14 = 0.0;

        locals.var_qsexte = 0.0;
        locals.var_qsexte_dn0 = 0.0;
        locals.var_qsexte_dn2 = 0.0;
        locals.var_qsexte_dn4 = 0.0;
        locals.var_qsexte_dn5 = 0.0;
        locals.var_qsexte_dn6 = 0.0;
        locals.var_qsexte_dn7 = 0.0;
        locals.var_qsexte_dn8 = 0.0;
        locals.var_qsexte_dn9 = 0.0;
        locals.var_qsexte_dn10 = 0.0;
        locals.var_qsexte_dn11 = 0.0;
        locals.var_qsexte_dn14 = 0.0;

        locals.var_qgov = 0.0;
        locals.var_qgov_dn0 = 0.0;
        locals.var_qgov_dn2 = 0.0;
        locals.var_qgov_dn4 = 0.0;
        locals.var_qgov_dn5 = 0.0;
        locals.var_qgov_dn6 = 0.0;
        locals.var_qgov_dn7 = 0.0;
        locals.var_qgov_dn8 = 0.0;
        locals.var_qgov_dn9 = 0.0;
        locals.var_qgov_dn10 = 0.0;
        locals.var_qgov_dn11 = 0.0;
        locals.var_qgov_dn14 = 0.0;

        locals.var_qdov = 0.0;
        locals.var_qdov_dn0 = 0.0;
        locals.var_qdov_dn2 = 0.0;
        locals.var_qdov_dn4 = 0.0;
        locals.var_qdov_dn5 = 0.0;
        locals.var_qdov_dn6 = 0.0;
        locals.var_qdov_dn7 = 0.0;
        locals.var_qdov_dn8 = 0.0;
        locals.var_qdov_dn9 = 0.0;
        locals.var_qdov_dn10 = 0.0;
        locals.var_qdov_dn11 = 0.0;
        locals.var_qdov_dn14 = 0.0;

        locals.var_qsov = 0.0;
        locals.var_qsov_dn0 = 0.0;
        locals.var_qsov_dn2 = 0.0;
        locals.var_qsov_dn4 = 0.0;
        locals.var_qsov_dn5 = 0.0;
        locals.var_qsov_dn6 = 0.0;
        locals.var_qsov_dn7 = 0.0;
        locals.var_qsov_dn8 = 0.0;
        locals.var_qsov_dn9 = 0.0;
        locals.var_qsov_dn10 = 0.0;
        locals.var_qsov_dn11 = 0.0;
        locals.var_qsov_dn14 = 0.0;

        locals.var_qdp = 0.0;
        locals.var_qdp_dn0 = 0.0;
        locals.var_qdp_dn2 = 0.0;
        locals.var_qdp_dn7 = 0.0;

        locals.var_qsp = 0.0;
        locals.var_qsp_dn2 = 0.0;
        locals.var_qsp_dn7 = 0.0;

        let assign101830_e153888: f64 = if ((locals.var_flg_nqs != 0.0) || (p.p22 == 2.0)) { 1.0 } else { 0.0 };
        locals.var_guard2331 = assign101830_e153888;

        let (assign101840_e153892, assign101840_e153892_d_n0, assign101840_e153892_d_n2, assign101840_e153892_d_n4, assign101840_e153892_d_n5, assign101840_e153892_d_n6, assign101840_e153892_d_n7, assign101840_e153892_d_n8, assign101840_e153892_d_n9, assign101840_e153892_d_n10, assign101840_e153892_d_n11, assign101840_e153892_d_n14,) = {
    if (locals.var_guard2331 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn4, locals.var_qge_dn5, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn8, locals.var_qge_dn9, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn14,)
    }
};
        locals.var_qge = assign101840_e153892;
        locals.var_qge_dn0 = assign101840_e153892_d_n0;
        locals.var_qge_dn2 = assign101840_e153892_d_n2;
        locals.var_qge_dn4 = assign101840_e153892_d_n4;
        locals.var_qge_dn5 = assign101840_e153892_d_n5;
        locals.var_qge_dn6 = assign101840_e153892_d_n6;
        locals.var_qge_dn7 = assign101840_e153892_d_n7;
        locals.var_qge_dn8 = assign101840_e153892_d_n8;
        locals.var_qge_dn9 = assign101840_e153892_d_n9;
        locals.var_qge_dn10 = assign101840_e153892_d_n10;
        locals.var_qge_dn11 = assign101840_e153892_d_n11;
        locals.var_qge_dn14 = assign101840_e153892_d_n14;

        let (assign101850_e153896, assign101850_e153896_d_n0, assign101850_e153896_d_n2, assign101850_e153896_d_n4, assign101850_e153896_d_n5, assign101850_e153896_d_n6, assign101850_e153896_d_n7, assign101850_e153896_d_n8, assign101850_e153896_d_n9, assign101850_e153896_d_n10, assign101850_e153896_d_n11, assign101850_e153896_d_n14,) = {
    if (locals.var_guard2331 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn14,)
    }
};
        locals.var_qde = assign101850_e153896;
        locals.var_qde_dn0 = assign101850_e153896_d_n0;
        locals.var_qde_dn2 = assign101850_e153896_d_n2;
        locals.var_qde_dn4 = assign101850_e153896_d_n4;
        locals.var_qde_dn5 = assign101850_e153896_d_n5;
        locals.var_qde_dn6 = assign101850_e153896_d_n6;
        locals.var_qde_dn7 = assign101850_e153896_d_n7;
        locals.var_qde_dn8 = assign101850_e153896_d_n8;
        locals.var_qde_dn9 = assign101850_e153896_d_n9;
        locals.var_qde_dn10 = assign101850_e153896_d_n10;
        locals.var_qde_dn11 = assign101850_e153896_d_n11;
        locals.var_qde_dn14 = assign101850_e153896_d_n14;

        let (assign101860_e153900, assign101860_e153900_d_n0, assign101860_e153900_d_n2, assign101860_e153900_d_n4, assign101860_e153900_d_n5, assign101860_e153900_d_n6, assign101860_e153900_d_n7, assign101860_e153900_d_n8, assign101860_e153900_d_n9, assign101860_e153900_d_n10, assign101860_e153900_d_n11, assign101860_e153900_d_n14,) = {
    if (locals.var_guard2331 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn8, locals.var_qse_dn9, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn14,)
    }
};
        locals.var_qse = assign101860_e153900;
        locals.var_qse_dn0 = assign101860_e153900_d_n0;
        locals.var_qse_dn2 = assign101860_e153900_d_n2;
        locals.var_qse_dn4 = assign101860_e153900_d_n4;
        locals.var_qse_dn5 = assign101860_e153900_d_n5;
        locals.var_qse_dn6 = assign101860_e153900_d_n6;
        locals.var_qse_dn7 = assign101860_e153900_d_n7;
        locals.var_qse_dn8 = assign101860_e153900_d_n8;
        locals.var_qse_dn9 = assign101860_e153900_d_n9;
        locals.var_qse_dn10 = assign101860_e153900_d_n10;
        locals.var_qse_dn11 = assign101860_e153900_d_n11;
        locals.var_qse_dn14 = assign101860_e153900_d_n14;

        let (assign101870_e153904, assign101870_e153904_d_n0, assign101870_e153904_d_n2, assign101870_e153904_d_n4, assign101870_e153904_d_n5, assign101870_e153904_d_n6, assign101870_e153904_d_n7, assign101870_e153904_d_n8, assign101870_e153904_d_n9, assign101870_e153904_d_n10, assign101870_e153904_d_n11, assign101870_e153904_d_n14,) = {
    if (locals.var_guard2331 != 0.0) {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn4, locals.var_qdrat_dn5, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn8, locals.var_qdrat_dn9, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn14,)
    } else {
        (locals.var_xd, locals.var_xd_dn0, locals.var_xd_dn2, locals.var_xd_dn4, locals.var_xd_dn5, locals.var_xd_dn6, locals.var_xd_dn7, locals.var_xd_dn8, locals.var_xd_dn9, locals.var_xd_dn10, locals.var_xd_dn11, locals.var_xd_dn14,)
    }
};
        locals.var_xd = assign101870_e153904;
        locals.var_xd_dn0 = assign101870_e153904_d_n0;
        locals.var_xd_dn2 = assign101870_e153904_d_n2;
        locals.var_xd_dn4 = assign101870_e153904_d_n4;
        locals.var_xd_dn5 = assign101870_e153904_d_n5;
        locals.var_xd_dn6 = assign101870_e153904_d_n6;
        locals.var_xd_dn7 = assign101870_e153904_d_n7;
        locals.var_xd_dn8 = assign101870_e153904_d_n8;
        locals.var_xd_dn9 = assign101870_e153904_d_n9;
        locals.var_xd_dn10 = assign101870_e153904_d_n10;
        locals.var_xd_dn11 = assign101870_e153904_d_n11;
        locals.var_xd_dn14 = assign101870_e153904_d_n14;

        let (assign101890_e153916, assign101890_e153916_d_n0, assign101890_e153916_d_n2, assign101890_e153916_d_n4, assign101890_e153916_d_n5, assign101890_e153916_d_n6, assign101890_e153916_d_n7, assign101890_e153916_d_n8, assign101890_e153916_d_n9, assign101890_e153916_d_n10, assign101890_e153916_d_n11, assign101890_e153916_d_n14,) = {
    if (locals.var_guard2331 != 0.0) {
        let assign101890_e153914: f64 = (locals.var_mfactor * locals.var_qi);
        (assign101890_e153914, (locals.var_mfactor * locals.var_qi_dn0), (locals.var_mfactor * locals.var_qi_dn2), (locals.var_mfactor * locals.var_qi_dn4), (locals.var_mfactor * locals.var_qi_dn5), (locals.var_mfactor * locals.var_qi_dn6), (locals.var_mfactor * locals.var_qi_dn7), (locals.var_mfactor * locals.var_qi_dn8), (locals.var_mfactor * locals.var_qi_dn9), (locals.var_mfactor * locals.var_qi_dn10), (locals.var_mfactor * locals.var_qi_dn11), (locals.var_mfactor * locals.var_qi_dn14),)
    } else {
        (locals.var_qi, locals.var_qi_dn0, locals.var_qi_dn2, locals.var_qi_dn4, locals.var_qi_dn5, locals.var_qi_dn6, locals.var_qi_dn7, locals.var_qi_dn8, locals.var_qi_dn9, locals.var_qi_dn10, locals.var_qi_dn11, locals.var_qi_dn14,)
    }
};
        locals.var_qi = assign101890_e153916;
        locals.var_qi_dn0 = assign101890_e153916_d_n0;
        locals.var_qi_dn2 = assign101890_e153916_d_n2;
        locals.var_qi_dn4 = assign101890_e153916_d_n4;
        locals.var_qi_dn5 = assign101890_e153916_d_n5;
        locals.var_qi_dn6 = assign101890_e153916_d_n6;
        locals.var_qi_dn7 = assign101890_e153916_d_n7;
        locals.var_qi_dn8 = assign101890_e153916_d_n8;
        locals.var_qi_dn9 = assign101890_e153916_d_n9;
        locals.var_qi_dn10 = assign101890_e153916_d_n10;
        locals.var_qi_dn11 = assign101890_e153916_d_n11;
        locals.var_qi_dn14 = assign101890_e153916_d_n14;

        let (assign101900_e153926, assign101900_e153926_d_n0, assign101900_e153926_d_n2, assign101900_e153926_d_n4, assign101900_e153926_d_n5, assign101900_e153926_d_n6, assign101900_e153926_d_n7, assign101900_e153926_d_n8, assign101900_e153926_d_n9, assign101900_e153926_d_n10, assign101900_e153926_d_n11, assign101900_e153926_d_n14,) = {
    if (locals.var_guard2331 == 0.0) {
        let assign101900_e153922: f64 = (locals.var_qb + locals.var_qi);
        let assign101900_e153923: f64 = (-assign101900_e153922);
        let assign101900_e153924: f64 = (locals.var_mfactor * assign101900_e153923);
        (assign101900_e153924, (locals.var_mfactor * (-(locals.var_qb_dn0 + locals.var_qi_dn0))), (locals.var_mfactor * (-(locals.var_qb_dn2 + locals.var_qi_dn2))), (locals.var_mfactor * (-(locals.var_qb_dn4 + locals.var_qi_dn4))), (locals.var_mfactor * (-(locals.var_qb_dn5 + locals.var_qi_dn5))), (locals.var_mfactor * (-(locals.var_qb_dn6 + locals.var_qi_dn6))), (locals.var_mfactor * (-(locals.var_qb_dn7 + locals.var_qi_dn7))), (locals.var_mfactor * (-(locals.var_qb_dn8 + locals.var_qi_dn8))), (locals.var_mfactor * (-(locals.var_qb_dn9 + locals.var_qi_dn9))), (locals.var_mfactor * (-(locals.var_qb_dn10 + locals.var_qi_dn10))), (locals.var_mfactor * (-(locals.var_qb_dn11 + locals.var_qi_dn11))), (locals.var_mfactor * (-(locals.var_qb_dn14 + locals.var_qi_dn14))),)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn4, locals.var_qge_dn5, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn8, locals.var_qge_dn9, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn14,)
    }
};
        locals.var_qge = assign101900_e153926;
        locals.var_qge_dn0 = assign101900_e153926_d_n0;
        locals.var_qge_dn2 = assign101900_e153926_d_n2;
        locals.var_qge_dn4 = assign101900_e153926_d_n4;
        locals.var_qge_dn5 = assign101900_e153926_d_n5;
        locals.var_qge_dn6 = assign101900_e153926_d_n6;
        locals.var_qge_dn7 = assign101900_e153926_d_n7;
        locals.var_qge_dn8 = assign101900_e153926_d_n8;
        locals.var_qge_dn9 = assign101900_e153926_d_n9;
        locals.var_qge_dn10 = assign101900_e153926_d_n10;
        locals.var_qge_dn11 = assign101900_e153926_d_n11;
        locals.var_qge_dn14 = assign101900_e153926_d_n14;

    }

    pub(super) fn stamp_transient_block_373(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv14 = ctx.node_voltage(nodes[14]);
        let (assign101910_e153933, assign101910_e153933_d_n0, assign101910_e153933_d_n2, assign101910_e153933_d_n4, assign101910_e153933_d_n5, assign101910_e153933_d_n6, assign101910_e153933_d_n7, assign101910_e153933_d_n8, assign101910_e153933_d_n9, assign101910_e153933_d_n10, assign101910_e153933_d_n11, assign101910_e153933_d_n14,) = {
    if (locals.var_guard2331 == 0.0) {
        let assign101910_e153931: f64 = (locals.var_mfactor * locals.var_qd);
        (assign101910_e153931, (locals.var_mfactor * locals.var_qd_dn0), (locals.var_mfactor * locals.var_qd_dn2), (locals.var_mfactor * locals.var_qd_dn4), (locals.var_mfactor * locals.var_qd_dn5), (locals.var_mfactor * locals.var_qd_dn6), (locals.var_mfactor * locals.var_qd_dn7), (locals.var_mfactor * locals.var_qd_dn8), (locals.var_mfactor * locals.var_qd_dn9), (locals.var_mfactor * locals.var_qd_dn10), (locals.var_mfactor * locals.var_qd_dn11), (locals.var_mfactor * locals.var_qd_dn14),)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn14,)
    }
};
        locals.var_qde = assign101910_e153933;
        locals.var_qde_dn0 = assign101910_e153933_d_n0;
        locals.var_qde_dn2 = assign101910_e153933_d_n2;
        locals.var_qde_dn4 = assign101910_e153933_d_n4;
        locals.var_qde_dn5 = assign101910_e153933_d_n5;
        locals.var_qde_dn6 = assign101910_e153933_d_n6;
        locals.var_qde_dn7 = assign101910_e153933_d_n7;
        locals.var_qde_dn8 = assign101910_e153933_d_n8;
        locals.var_qde_dn9 = assign101910_e153933_d_n9;
        locals.var_qde_dn10 = assign101910_e153933_d_n10;
        locals.var_qde_dn11 = assign101910_e153933_d_n11;
        locals.var_qde_dn14 = assign101910_e153933_d_n14;

        let (assign101920_e153942, assign101920_e153942_d_n0, assign101920_e153942_d_n2, assign101920_e153942_d_n4, assign101920_e153942_d_n5, assign101920_e153942_d_n6, assign101920_e153942_d_n7, assign101920_e153942_d_n8, assign101920_e153942_d_n9, assign101920_e153942_d_n10, assign101920_e153942_d_n11, assign101920_e153942_d_n14,) = {
    if (locals.var_guard2331 == 0.0) {
        let assign101920_e153939: f64 = (locals.var_qi - locals.var_qd);
        let assign101920_e153940: f64 = (locals.var_mfactor * assign101920_e153939);
        (assign101920_e153940, (locals.var_mfactor * (locals.var_qi_dn0 - locals.var_qd_dn0)), (locals.var_mfactor * (locals.var_qi_dn2 - locals.var_qd_dn2)), (locals.var_mfactor * (locals.var_qi_dn4 - locals.var_qd_dn4)), (locals.var_mfactor * (locals.var_qi_dn5 - locals.var_qd_dn5)), (locals.var_mfactor * (locals.var_qi_dn6 - locals.var_qd_dn6)), (locals.var_mfactor * (locals.var_qi_dn7 - locals.var_qd_dn7)), (locals.var_mfactor * (locals.var_qi_dn8 - locals.var_qd_dn8)), (locals.var_mfactor * (locals.var_qi_dn9 - locals.var_qd_dn9)), (locals.var_mfactor * (locals.var_qi_dn10 - locals.var_qd_dn10)), (locals.var_mfactor * (locals.var_qi_dn11 - locals.var_qd_dn11)), (locals.var_mfactor * (locals.var_qi_dn14 - locals.var_qd_dn14)),)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn8, locals.var_qse_dn9, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn14,)
    }
};
        locals.var_qse = assign101920_e153942;
        locals.var_qse_dn0 = assign101920_e153942_d_n0;
        locals.var_qse_dn2 = assign101920_e153942_d_n2;
        locals.var_qse_dn4 = assign101920_e153942_d_n4;
        locals.var_qse_dn5 = assign101920_e153942_d_n5;
        locals.var_qse_dn6 = assign101920_e153942_d_n6;
        locals.var_qse_dn7 = assign101920_e153942_d_n7;
        locals.var_qse_dn8 = assign101920_e153942_d_n8;
        locals.var_qse_dn9 = assign101920_e153942_d_n9;
        locals.var_qse_dn10 = assign101920_e153942_d_n10;
        locals.var_qse_dn11 = assign101920_e153942_d_n11;
        locals.var_qse_dn14 = assign101920_e153942_d_n14;

        let (assign101930_e153948, assign101930_e153948_d_n0, assign101930_e153948_d_n2, assign101930_e153948_d_n4, assign101930_e153948_d_n5, assign101930_e153948_d_n6, assign101930_e153948_d_n7, assign101930_e153948_d_n8, assign101930_e153948_d_n9, assign101930_e153948_d_n10, assign101930_e153948_d_n11, assign101930_e153948_d_n14,) = {
    if (p.p29 != 0.0) {
        let assign101930_e153946: f64 = (locals.var_mks_dlyov * locals.var_psl);
        (assign101930_e153946, ((locals.var_mks_dlyov_dn0 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn0)), ((locals.var_mks_dlyov_dn2 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn2)), ((locals.var_mks_dlyov_dn4 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn4)), ((locals.var_mks_dlyov_dn5 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn5)), ((locals.var_mks_dlyov_dn6 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn6)), ((locals.var_mks_dlyov_dn7 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn7)), ((locals.var_mks_dlyov_dn8 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn8)), ((locals.var_mks_dlyov_dn9 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn9)), ((locals.var_mks_dlyov_dn10 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn10)), ((locals.var_mks_dlyov_dn11 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn11)), ((locals.var_mks_dlyov_dn14 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn14)),)
    } else {
        (locals.var_mks_dlyov, locals.var_mks_dlyov_dn0, locals.var_mks_dlyov_dn2, locals.var_mks_dlyov_dn4, locals.var_mks_dlyov_dn5, locals.var_mks_dlyov_dn6, locals.var_mks_dlyov_dn7, locals.var_mks_dlyov_dn8, locals.var_mks_dlyov_dn9, locals.var_mks_dlyov_dn10, locals.var_mks_dlyov_dn11, locals.var_mks_dlyov_dn14,)
    }
};
        locals.var_mks_dlyov = assign101930_e153948;
        locals.var_mks_dlyov_dn0 = assign101930_e153948_d_n0;
        locals.var_mks_dlyov_dn2 = assign101930_e153948_d_n2;
        locals.var_mks_dlyov_dn4 = assign101930_e153948_d_n4;
        locals.var_mks_dlyov_dn5 = assign101930_e153948_d_n5;
        locals.var_mks_dlyov_dn6 = assign101930_e153948_d_n6;
        locals.var_mks_dlyov_dn7 = assign101930_e153948_d_n7;
        locals.var_mks_dlyov_dn8 = assign101930_e153948_d_n8;
        locals.var_mks_dlyov_dn9 = assign101930_e153948_d_n9;
        locals.var_mks_dlyov_dn10 = assign101930_e153948_d_n10;
        locals.var_mks_dlyov_dn11 = assign101930_e153948_d_n11;
        locals.var_mks_dlyov_dn14 = assign101930_e153948_d_n14;

        let (assign101940_e153961, assign101940_e153961_d_n0, assign101940_e153961_d_n2, assign101940_e153961_d_n4, assign101940_e153961_d_n5, assign101940_e153961_d_n6, assign101940_e153961_d_n7, assign101940_e153961_d_n8, assign101940_e153961_d_n9, assign101940_e153961_d_n10, assign101940_e153961_d_n11, assign101940_e153961_d_n14,) = {
    if (p.p29 != 0.0) {
        let assign101940_e153952: f64 = (locals.var_mks_dlyov * locals.var_mks_dlyov);
        let assign101940_e153955: f64 = (4.0 * 1e-12);
        let assign101940_e153957: f64 = (assign101940_e153955 * 1e-12);
        let assign101940_e153958: f64 = (assign101940_e153952 + assign101940_e153957);
        let assign101940_e153959: f64 = (assign101940_e153958).sqrt();
        (assign101940_e153959, (((locals.var_mks_dlyov_dn0 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn0)) / (2.0 * assign101940_e153959)), (((locals.var_mks_dlyov_dn2 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn2)) / (2.0 * assign101940_e153959)), (((locals.var_mks_dlyov_dn4 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn4)) / (2.0 * assign101940_e153959)), (((locals.var_mks_dlyov_dn5 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn5)) / (2.0 * assign101940_e153959)), (((locals.var_mks_dlyov_dn6 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn6)) / (2.0 * assign101940_e153959)), (((locals.var_mks_dlyov_dn7 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn7)) / (2.0 * assign101940_e153959)), (((locals.var_mks_dlyov_dn8 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn8)) / (2.0 * assign101940_e153959)), (((locals.var_mks_dlyov_dn9 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn9)) / (2.0 * assign101940_e153959)), (((locals.var_mks_dlyov_dn10 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn10)) / (2.0 * assign101940_e153959)), (((locals.var_mks_dlyov_dn11 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn11)) / (2.0 * assign101940_e153959)), (((locals.var_mks_dlyov_dn14 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn14)) / (2.0 * assign101940_e153959)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign101940_e153961;
        locals.var_tmf2_dn0 = assign101940_e153961_d_n0;
        locals.var_tmf2_dn2 = assign101940_e153961_d_n2;
        locals.var_tmf2_dn4 = assign101940_e153961_d_n4;
        locals.var_tmf2_dn5 = assign101940_e153961_d_n5;
        locals.var_tmf2_dn6 = assign101940_e153961_d_n6;
        locals.var_tmf2_dn7 = assign101940_e153961_d_n7;
        locals.var_tmf2_dn8 = assign101940_e153961_d_n8;
        locals.var_tmf2_dn9 = assign101940_e153961_d_n9;
        locals.var_tmf2_dn10 = assign101940_e153961_d_n10;
        locals.var_tmf2_dn11 = assign101940_e153961_d_n11;
        locals.var_tmf2_dn14 = assign101940_e153961_d_n14;

        let (assign101950_e153971, assign101950_e153971_d_n0, assign101950_e153971_d_n2, assign101950_e153971_d_n4, assign101950_e153971_d_n5, assign101950_e153971_d_n6, assign101950_e153971_d_n7, assign101950_e153971_d_n8, assign101950_e153971_d_n9, assign101950_e153971_d_n10, assign101950_e153971_d_n11, assign101950_e153971_d_n14,) = {
    if (p.p29 != 0.0) {
        let assign101950_e153967: f64 = (locals.var_mks_dlyov / locals.var_tmf2);
        let assign101950_e153968: f64 = (1.0 + assign101950_e153967);
        let assign101950_e153969: f64 = (0.5 * assign101950_e153968);
        (assign101950_e153969, (0.5 * (((locals.var_mks_dlyov_dn0 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn2 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn4 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn5 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn6 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn7 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn8 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn9 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn10 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn11 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn14 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign101950_e153971;
        locals.var_t0_dn0 = assign101950_e153971_d_n0;
        locals.var_t0_dn2 = assign101950_e153971_d_n2;
        locals.var_t0_dn4 = assign101950_e153971_d_n4;
        locals.var_t0_dn5 = assign101950_e153971_d_n5;
        locals.var_t0_dn6 = assign101950_e153971_d_n6;
        locals.var_t0_dn7 = assign101950_e153971_d_n7;
        locals.var_t0_dn8 = assign101950_e153971_d_n8;
        locals.var_t0_dn9 = assign101950_e153971_d_n9;
        locals.var_t0_dn10 = assign101950_e153971_d_n10;
        locals.var_t0_dn11 = assign101950_e153971_d_n11;
        locals.var_t0_dn14 = assign101950_e153971_d_n14;

        let (assign101960_e153979, assign101960_e153979_d_n0, assign101960_e153979_d_n2, assign101960_e153979_d_n4, assign101960_e153979_d_n5, assign101960_e153979_d_n6, assign101960_e153979_d_n7, assign101960_e153979_d_n8, assign101960_e153979_d_n9, assign101960_e153979_d_n10, assign101960_e153979_d_n11, assign101960_e153979_d_n14,) = {
    if (p.p29 != 0.0) {
        let assign101960_e153976: f64 = (locals.var_mks_dlyov + locals.var_tmf2);
        let assign101960_e153977: f64 = (0.5 * assign101960_e153976);
        (assign101960_e153977, (0.5 * (locals.var_mks_dlyov_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_mks_dlyov_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_mks_dlyov_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_mks_dlyov_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_mks_dlyov_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_mks_dlyov_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_mks_dlyov_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_mks_dlyov_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_mks_dlyov_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_mks_dlyov_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_mks_dlyov_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_mks_dlyov, locals.var_mks_dlyov_dn0, locals.var_mks_dlyov_dn2, locals.var_mks_dlyov_dn4, locals.var_mks_dlyov_dn5, locals.var_mks_dlyov_dn6, locals.var_mks_dlyov_dn7, locals.var_mks_dlyov_dn8, locals.var_mks_dlyov_dn9, locals.var_mks_dlyov_dn10, locals.var_mks_dlyov_dn11, locals.var_mks_dlyov_dn14,)
    }
};
        locals.var_mks_dlyov = assign101960_e153979;
        locals.var_mks_dlyov_dn0 = assign101960_e153979_d_n0;
        locals.var_mks_dlyov_dn2 = assign101960_e153979_d_n2;
        locals.var_mks_dlyov_dn4 = assign101960_e153979_d_n4;
        locals.var_mks_dlyov_dn5 = assign101960_e153979_d_n5;
        locals.var_mks_dlyov_dn6 = assign101960_e153979_d_n6;
        locals.var_mks_dlyov_dn7 = assign101960_e153979_d_n7;
        locals.var_mks_dlyov_dn8 = assign101960_e153979_d_n8;
        locals.var_mks_dlyov_dn9 = assign101960_e153979_d_n9;
        locals.var_mks_dlyov_dn10 = assign101960_e153979_d_n10;
        locals.var_mks_dlyov_dn11 = assign101960_e153979_d_n11;
        locals.var_mks_dlyov_dn14 = assign101960_e153979_d_n14;

        let assign101970_e153982: f64 = if locals.var_mks_dlyov < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2332 = assign101970_e153982;

        let (assign101980_e153988, assign101980_e153988_d_n0, assign101980_e153988_d_n2, assign101980_e153988_d_n4, assign101980_e153988_d_n5, assign101980_e153988_d_n6, assign101980_e153988_d_n7, assign101980_e153988_d_n8, assign101980_e153988_d_n9, assign101980_e153988_d_n10, assign101980_e153988_d_n11, assign101980_e153988_d_n14,) = {
    if ((p.p29 != 0.0) && (locals.var_guard2332 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_mks_dlyov, locals.var_mks_dlyov_dn0, locals.var_mks_dlyov_dn2, locals.var_mks_dlyov_dn4, locals.var_mks_dlyov_dn5, locals.var_mks_dlyov_dn6, locals.var_mks_dlyov_dn7, locals.var_mks_dlyov_dn8, locals.var_mks_dlyov_dn9, locals.var_mks_dlyov_dn10, locals.var_mks_dlyov_dn11, locals.var_mks_dlyov_dn14,)
    }
};
        locals.var_mks_dlyov = assign101980_e153988;
        locals.var_mks_dlyov_dn0 = assign101980_e153988_d_n0;
        locals.var_mks_dlyov_dn2 = assign101980_e153988_d_n2;
        locals.var_mks_dlyov_dn4 = assign101980_e153988_d_n4;
        locals.var_mks_dlyov_dn5 = assign101980_e153988_d_n5;
        locals.var_mks_dlyov_dn6 = assign101980_e153988_d_n6;
        locals.var_mks_dlyov_dn7 = assign101980_e153988_d_n7;
        locals.var_mks_dlyov_dn8 = assign101980_e153988_d_n8;
        locals.var_mks_dlyov_dn9 = assign101980_e153988_d_n9;
        locals.var_mks_dlyov_dn10 = assign101980_e153988_d_n10;
        locals.var_mks_dlyov_dn11 = assign101980_e153988_d_n11;
        locals.var_mks_dlyov_dn14 = assign101980_e153988_d_n14;

        let (assign101990_e153994, assign101990_e153994_d_n0, assign101990_e153994_d_n2, assign101990_e153994_d_n4, assign101990_e153994_d_n5, assign101990_e153994_d_n6, assign101990_e153994_d_n7, assign101990_e153994_d_n8, assign101990_e153994_d_n9, assign101990_e153994_d_n10, assign101990_e153994_d_n11, assign101990_e153994_d_n14,) = {
    if ((p.p29 != 0.0) && (locals.var_guard2332 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign101990_e153994;
        locals.var_t0_dn0 = assign101990_e153994_d_n0;
        locals.var_t0_dn2 = assign101990_e153994_d_n2;
        locals.var_t0_dn4 = assign101990_e153994_d_n4;
        locals.var_t0_dn5 = assign101990_e153994_d_n5;
        locals.var_t0_dn6 = assign101990_e153994_d_n6;
        locals.var_t0_dn7 = assign101990_e153994_d_n7;
        locals.var_t0_dn8 = assign101990_e153994_d_n8;
        locals.var_t0_dn9 = assign101990_e153994_d_n9;
        locals.var_t0_dn10 = assign101990_e153994_d_n10;
        locals.var_t0_dn11 = assign101990_e153994_d_n11;
        locals.var_t0_dn14 = assign101990_e153994_d_n14;

        let (assign102010_e154004, assign102010_e154004_d_n0, assign102010_e154004_d_n2, assign102010_e154004_d_n4, assign102010_e154004_d_n5, assign102010_e154004_d_n6, assign102010_e154004_d_n7, assign102010_e154004_d_n8, assign102010_e154004_d_n9, assign102010_e154004_d_n10, assign102010_e154004_d_n11, assign102010_e154004_d_n14,) = {
    if (p.p29 != 0.0) {
        ((nv14 - 0.0), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,)
    } else {
        (locals.var_qbd_nqs, locals.var_qbd_nqs_dn0, locals.var_qbd_nqs_dn2, locals.var_qbd_nqs_dn4, locals.var_qbd_nqs_dn5, locals.var_qbd_nqs_dn6, locals.var_qbd_nqs_dn7, locals.var_qbd_nqs_dn8, locals.var_qbd_nqs_dn9, locals.var_qbd_nqs_dn10, locals.var_qbd_nqs_dn11, locals.var_qbd_nqs_dn14,)
    }
};
        locals.var_qbd_nqs = assign102010_e154004;
        locals.var_qbd_nqs_dn0 = assign102010_e154004_d_n0;
        locals.var_qbd_nqs_dn2 = assign102010_e154004_d_n2;
        locals.var_qbd_nqs_dn4 = assign102010_e154004_d_n4;
        locals.var_qbd_nqs_dn5 = assign102010_e154004_d_n5;
        locals.var_qbd_nqs_dn6 = assign102010_e154004_d_n6;
        locals.var_qbd_nqs_dn7 = assign102010_e154004_d_n7;
        locals.var_qbd_nqs_dn8 = assign102010_e154004_d_n8;
        locals.var_qbd_nqs_dn9 = assign102010_e154004_d_n9;
        locals.var_qbd_nqs_dn10 = assign102010_e154004_d_n10;
        locals.var_qbd_nqs_dn11 = assign102010_e154004_d_n11;
        locals.var_qbd_nqs_dn14 = assign102010_e154004_d_n14;

        let (assign102030_e154020, assign102030_e154020_d_n0, assign102030_e154020_d_n2, assign102030_e154020_d_n4, assign102030_e154020_d_n5, assign102030_e154020_d_n6, assign102030_e154020_d_n7, assign102030_e154020_d_n8, assign102030_e154020_d_n9, assign102030_e154020_d_n10, assign102030_e154020_d_n11, assign102030_e154020_d_n14,) = {
    if (p.p29 != 0.0) {
        let assign102030_e154017: f64 = (locals.var_qbd_qs - locals.var_qbd_nqs);
        let assign102030_e154018: f64 = (locals.var_qovd - assign102030_e154017);
        (assign102030_e154018, (locals.var_qovd_dn0 - (locals.var_qbd_qs_dn0 - locals.var_qbd_nqs_dn0)), (locals.var_qovd_dn2 - (locals.var_qbd_qs_dn2 - locals.var_qbd_nqs_dn2)), (locals.var_qovd_dn4 - (locals.var_qbd_qs_dn4 - locals.var_qbd_nqs_dn4)), (locals.var_qovd_dn5 - (locals.var_qbd_qs_dn5 - locals.var_qbd_nqs_dn5)), (locals.var_qovd_dn6 - (locals.var_qbd_qs_dn6 - locals.var_qbd_nqs_dn6)), (locals.var_qovd_dn7 - (locals.var_qbd_qs_dn7 - locals.var_qbd_nqs_dn7)), (locals.var_qovd_dn8 - (locals.var_qbd_qs_dn8 - locals.var_qbd_nqs_dn8)), (locals.var_qovd_dn9 - (locals.var_qbd_qs_dn9 - locals.var_qbd_nqs_dn9)), (locals.var_qovd_dn10 - (locals.var_qbd_qs_dn10 - locals.var_qbd_nqs_dn10)), (locals.var_qovd_dn11 - (locals.var_qbd_qs_dn11 - locals.var_qbd_nqs_dn11)), (locals.var_qovd_dn14 - (locals.var_qbd_qs_dn14 - locals.var_qbd_nqs_dn14)),)
    } else {
        (locals.var_qovd, locals.var_qovd_dn0, locals.var_qovd_dn2, locals.var_qovd_dn4, locals.var_qovd_dn5, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn8, locals.var_qovd_dn9, locals.var_qovd_dn10, locals.var_qovd_dn11, locals.var_qovd_dn14,)
    }
};
        locals.var_qovd = assign102030_e154020;
        locals.var_qovd_dn0 = assign102030_e154020_d_n0;
        locals.var_qovd_dn2 = assign102030_e154020_d_n2;
        locals.var_qovd_dn4 = assign102030_e154020_d_n4;
        locals.var_qovd_dn5 = assign102030_e154020_d_n5;
        locals.var_qovd_dn6 = assign102030_e154020_d_n6;
        locals.var_qovd_dn7 = assign102030_e154020_d_n7;
        locals.var_qovd_dn8 = assign102030_e154020_d_n8;
        locals.var_qovd_dn9 = assign102030_e154020_d_n9;
        locals.var_qovd_dn10 = assign102030_e154020_d_n10;
        locals.var_qovd_dn11 = assign102030_e154020_d_n11;
        locals.var_qovd_dn14 = assign102030_e154020_d_n14;

        let (assign102040_e154024, assign102040_e154024_d_n0, assign102040_e154024_d_n2, assign102040_e154024_d_n4, assign102040_e154024_d_n5, assign102040_e154024_d_n6, assign102040_e154024_d_n7, assign102040_e154024_d_n8, assign102040_e154024_d_n9, assign102040_e154024_d_n10, assign102040_e154024_d_n11, assign102040_e154024_d_n14,) = {
    if (p.p29 != 0.0) {
        (locals.var_qbd_nqs, locals.var_qbd_nqs_dn0, locals.var_qbd_nqs_dn2, locals.var_qbd_nqs_dn4, locals.var_qbd_nqs_dn5, locals.var_qbd_nqs_dn6, locals.var_qbd_nqs_dn7, locals.var_qbd_nqs_dn8, locals.var_qbd_nqs_dn9, locals.var_qbd_nqs_dn10, locals.var_qbd_nqs_dn11, locals.var_qbd_nqs_dn14,)
    } else {
        (locals.var_qbdld, locals.var_qbdld_dn0, locals.var_qbdld_dn2, locals.var_qbdld_dn4, locals.var_qbdld_dn5, locals.var_qbdld_dn6, locals.var_qbdld_dn7, locals.var_qbdld_dn8, locals.var_qbdld_dn9, locals.var_qbdld_dn10, locals.var_qbdld_dn11, locals.var_qbdld_dn14,)
    }
};
        locals.var_qbdld = assign102040_e154024;
        locals.var_qbdld_dn0 = assign102040_e154024_d_n0;
        locals.var_qbdld_dn2 = assign102040_e154024_d_n2;
        locals.var_qbdld_dn4 = assign102040_e154024_d_n4;
        locals.var_qbdld_dn5 = assign102040_e154024_d_n5;
        locals.var_qbdld_dn6 = assign102040_e154024_d_n6;
        locals.var_qbdld_dn7 = assign102040_e154024_d_n7;
        locals.var_qbdld_dn8 = assign102040_e154024_d_n8;
        locals.var_qbdld_dn9 = assign102040_e154024_d_n9;
        locals.var_qbdld_dn10 = assign102040_e154024_d_n10;
        locals.var_qbdld_dn11 = assign102040_e154024_d_n11;
        locals.var_qbdld_dn14 = assign102040_e154024_d_n14;

        let (assign102050_e154029, assign102050_e154029_d_n0, assign102050_e154029_d_n2, assign102050_e154029_d_n4, assign102050_e154029_d_n5, assign102050_e154029_d_n6, assign102050_e154029_d_n7, assign102050_e154029_d_n8, assign102050_e154029_d_n9, assign102050_e154029_d_n10, assign102050_e154029_d_n11, assign102050_e154029_d_n14,) = {
    if (p.p29 == 0.0) {
        (locals.var_qbd_qs, locals.var_qbd_qs_dn0, locals.var_qbd_qs_dn2, locals.var_qbd_qs_dn4, locals.var_qbd_qs_dn5, locals.var_qbd_qs_dn6, locals.var_qbd_qs_dn7, locals.var_qbd_qs_dn8, locals.var_qbd_qs_dn9, locals.var_qbd_qs_dn10, locals.var_qbd_qs_dn11, locals.var_qbd_qs_dn14,)
    } else {
        (locals.var_qbd_nqs, locals.var_qbd_nqs_dn0, locals.var_qbd_nqs_dn2, locals.var_qbd_nqs_dn4, locals.var_qbd_nqs_dn5, locals.var_qbd_nqs_dn6, locals.var_qbd_nqs_dn7, locals.var_qbd_nqs_dn8, locals.var_qbd_nqs_dn9, locals.var_qbd_nqs_dn10, locals.var_qbd_nqs_dn11, locals.var_qbd_nqs_dn14,)
    }
};
        locals.var_qbd_nqs = assign102050_e154029;
        locals.var_qbd_nqs_dn0 = assign102050_e154029_d_n0;
        locals.var_qbd_nqs_dn2 = assign102050_e154029_d_n2;
        locals.var_qbd_nqs_dn4 = assign102050_e154029_d_n4;
        locals.var_qbd_nqs_dn5 = assign102050_e154029_d_n5;
        locals.var_qbd_nqs_dn6 = assign102050_e154029_d_n6;
        locals.var_qbd_nqs_dn7 = assign102050_e154029_d_n7;
        locals.var_qbd_nqs_dn8 = assign102050_e154029_d_n8;
        locals.var_qbd_nqs_dn9 = assign102050_e154029_d_n9;
        locals.var_qbd_nqs_dn10 = assign102050_e154029_d_n10;
        locals.var_qbd_nqs_dn11 = assign102050_e154029_d_n11;
        locals.var_qbd_nqs_dn14 = assign102050_e154029_d_n14;

        let assign102060_e154032: f64 = if p.p22 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2333 = assign102060_e154032;

        let (assign102070_e154046, assign102070_e154046_d_n0, assign102070_e154046_d_n2, assign102070_e154046_d_n4, assign102070_e154046_d_n5, assign102070_e154046_d_n6, assign102070_e154046_d_n7, assign102070_e154046_d_n8, assign102070_e154046_d_n9, assign102070_e154046_d_n10, assign102070_e154046_d_n11, assign102070_e154046_d_n14,) = {
    if (locals.var_guard2333 != 0.0) {
        let assign102070_e154037: f64 = (locals.var_qgbo - locals.var_qovd);
        let assign102070_e154039: f64 = (assign102070_e154037 - locals.var_qovs);
        let assign102070_e154041: f64 = (assign102070_e154039 + locals.var_qgos);
        let assign102070_e154043: f64 = (assign102070_e154041 + locals.var_qgod);
        let assign102070_e154044: f64 = (locals.var_mfactor * assign102070_e154043);
        (assign102070_e154044, (locals.var_mfactor * ((((-locals.var_qovd_dn0) - locals.var_qovs_dn0) + locals.var_qgos_dn0) + locals.var_qgod_dn0)), (locals.var_mfactor * ((((-locals.var_qovd_dn2) - locals.var_qovs_dn2) + locals.var_qgos_dn2) + locals.var_qgod_dn2)), (locals.var_mfactor * ((((-locals.var_qovd_dn4) - locals.var_qovs_dn4) + locals.var_qgos_dn4) + locals.var_qgod_dn4)), (locals.var_mfactor * ((((-locals.var_qovd_dn5) - locals.var_qovs_dn5) + locals.var_qgos_dn5) + locals.var_qgod_dn5)), (locals.var_mfactor * ((((-locals.var_qovd_dn6) - locals.var_qovs_dn6) + locals.var_qgos_dn6) + locals.var_qgod_dn6)), (locals.var_mfactor * ((((locals.var_qgbo_dn7 - locals.var_qovd_dn7) - locals.var_qovs_dn7) + locals.var_qgos_dn7) + locals.var_qgod_dn7)), (locals.var_mfactor * ((((locals.var_qgbo_dn8 - locals.var_qovd_dn8) - locals.var_qovs_dn8) + locals.var_qgos_dn8) + locals.var_qgod_dn8)), (locals.var_mfactor * ((((locals.var_qgbo_dn9 - locals.var_qovd_dn9) - locals.var_qovs_dn9) + locals.var_qgos_dn9) + locals.var_qgod_dn9)), (locals.var_mfactor * ((((-locals.var_qovd_dn10) - locals.var_qovs_dn10) + locals.var_qgos_dn10) + locals.var_qgod_dn10)), (locals.var_mfactor * ((((-locals.var_qovd_dn11) - locals.var_qovs_dn11) + locals.var_qgos_dn11) + locals.var_qgod_dn11)), (locals.var_mfactor * ((((-locals.var_qovd_dn14) - locals.var_qovs_dn14) + locals.var_qgos_dn14) + locals.var_qgod_dn14)),)
    } else {
        (locals.var_qgov, locals.var_qgov_dn0, locals.var_qgov_dn2, locals.var_qgov_dn4, locals.var_qgov_dn5, locals.var_qgov_dn6, locals.var_qgov_dn7, locals.var_qgov_dn8, locals.var_qgov_dn9, locals.var_qgov_dn10, locals.var_qgov_dn11, locals.var_qgov_dn14,)
    }
};
        locals.var_qgov = assign102070_e154046;
        locals.var_qgov_dn0 = assign102070_e154046_d_n0;
        locals.var_qgov_dn2 = assign102070_e154046_d_n2;
        locals.var_qgov_dn4 = assign102070_e154046_d_n4;
        locals.var_qgov_dn5 = assign102070_e154046_d_n5;
        locals.var_qgov_dn6 = assign102070_e154046_d_n6;
        locals.var_qgov_dn7 = assign102070_e154046_d_n7;
        locals.var_qgov_dn8 = assign102070_e154046_d_n8;
        locals.var_qgov_dn9 = assign102070_e154046_d_n9;
        locals.var_qgov_dn10 = assign102070_e154046_d_n10;
        locals.var_qgov_dn11 = assign102070_e154046_d_n11;
        locals.var_qgov_dn14 = assign102070_e154046_d_n14;

        let (assign102080_e154055, assign102080_e154055_d_n0, assign102080_e154055_d_n2, assign102080_e154055_d_n4, assign102080_e154055_d_n5, assign102080_e154055_d_n6, assign102080_e154055_d_n7, assign102080_e154055_d_n8, assign102080_e154055_d_n9, assign102080_e154055_d_n10, assign102080_e154055_d_n11, assign102080_e154055_d_n14,) = {
    if (locals.var_guard2333 != 0.0) {
        let assign102080_e154050: f64 = locals.var_qbdld;
        let assign102080_e154052: f64 = (assign102080_e154050 - locals.var_qgod);
        let assign102080_e154053: f64 = (locals.var_mfactor * assign102080_e154052);
        (assign102080_e154053, (locals.var_mfactor * (locals.var_qbdld_dn0 - locals.var_qgod_dn0)), (locals.var_mfactor * (locals.var_qbdld_dn2 - locals.var_qgod_dn2)), (locals.var_mfactor * (locals.var_qbdld_dn4 - locals.var_qgod_dn4)), (locals.var_mfactor * (locals.var_qbdld_dn5 - locals.var_qgod_dn5)), (locals.var_mfactor * (locals.var_qbdld_dn6 - locals.var_qgod_dn6)), (locals.var_mfactor * (locals.var_qbdld_dn7 - locals.var_qgod_dn7)), (locals.var_mfactor * (locals.var_qbdld_dn8 - locals.var_qgod_dn8)), (locals.var_mfactor * (locals.var_qbdld_dn9 - locals.var_qgod_dn9)), (locals.var_mfactor * (locals.var_qbdld_dn10 - locals.var_qgod_dn10)), (locals.var_mfactor * (locals.var_qbdld_dn11 - locals.var_qgod_dn11)), (locals.var_mfactor * (locals.var_qbdld_dn14 - locals.var_qgod_dn14)),)
    } else {
        (locals.var_qdov, locals.var_qdov_dn0, locals.var_qdov_dn2, locals.var_qdov_dn4, locals.var_qdov_dn5, locals.var_qdov_dn6, locals.var_qdov_dn7, locals.var_qdov_dn8, locals.var_qdov_dn9, locals.var_qdov_dn10, locals.var_qdov_dn11, locals.var_qdov_dn14,)
    }
};
        locals.var_qdov = assign102080_e154055;
        locals.var_qdov_dn0 = assign102080_e154055_d_n0;
        locals.var_qdov_dn2 = assign102080_e154055_d_n2;
        locals.var_qdov_dn4 = assign102080_e154055_d_n4;
        locals.var_qdov_dn5 = assign102080_e154055_d_n5;
        locals.var_qdov_dn6 = assign102080_e154055_d_n6;
        locals.var_qdov_dn7 = assign102080_e154055_d_n7;
        locals.var_qdov_dn8 = assign102080_e154055_d_n8;
        locals.var_qdov_dn9 = assign102080_e154055_d_n9;
        locals.var_qdov_dn10 = assign102080_e154055_d_n10;
        locals.var_qdov_dn11 = assign102080_e154055_d_n11;
        locals.var_qdov_dn14 = assign102080_e154055_d_n14;

        let (assign102090_e154064, assign102090_e154064_d_n0, assign102090_e154064_d_n2, assign102090_e154064_d_n4, assign102090_e154064_d_n5, assign102090_e154064_d_n6, assign102090_e154064_d_n7, assign102090_e154064_d_n8, assign102090_e154064_d_n9, assign102090_e154064_d_n10, assign102090_e154064_d_n11, assign102090_e154064_d_n14,) = {
    if (locals.var_guard2333 != 0.0) {
        let assign102090_e154059: f64 = locals.var_qbsld;
        let assign102090_e154061: f64 = (assign102090_e154059 - locals.var_qgos);
        let assign102090_e154062: f64 = (locals.var_mfactor * assign102090_e154061);
        (assign102090_e154062, (locals.var_mfactor * (locals.var_qbsld_dn0 - locals.var_qgos_dn0)), (locals.var_mfactor * (locals.var_qbsld_dn2 - locals.var_qgos_dn2)), (locals.var_mfactor * (locals.var_qbsld_dn4 - locals.var_qgos_dn4)), (locals.var_mfactor * (locals.var_qbsld_dn5 - locals.var_qgos_dn5)), (locals.var_mfactor * (locals.var_qbsld_dn6 - locals.var_qgos_dn6)), (locals.var_mfactor * (locals.var_qbsld_dn7 - locals.var_qgos_dn7)), (locals.var_mfactor * (locals.var_qbsld_dn8 - locals.var_qgos_dn8)), (locals.var_mfactor * (locals.var_qbsld_dn9 - locals.var_qgos_dn9)), (locals.var_mfactor * (locals.var_qbsld_dn10 - locals.var_qgos_dn10)), (locals.var_mfactor * (locals.var_qbsld_dn11 - locals.var_qgos_dn11)), (locals.var_mfactor * (locals.var_qbsld_dn14 - locals.var_qgos_dn14)),)
    } else {
        (locals.var_qsov, locals.var_qsov_dn0, locals.var_qsov_dn2, locals.var_qsov_dn4, locals.var_qsov_dn5, locals.var_qsov_dn6, locals.var_qsov_dn7, locals.var_qsov_dn8, locals.var_qsov_dn9, locals.var_qsov_dn10, locals.var_qsov_dn11, locals.var_qsov_dn14,)
    }
};
        locals.var_qsov = assign102090_e154064;
        locals.var_qsov_dn0 = assign102090_e154064_d_n0;
        locals.var_qsov_dn2 = assign102090_e154064_d_n2;
        locals.var_qsov_dn4 = assign102090_e154064_d_n4;
        locals.var_qsov_dn5 = assign102090_e154064_d_n5;
        locals.var_qsov_dn6 = assign102090_e154064_d_n6;
        locals.var_qsov_dn7 = assign102090_e154064_d_n7;
        locals.var_qsov_dn8 = assign102090_e154064_d_n8;
        locals.var_qsov_dn9 = assign102090_e154064_d_n9;
        locals.var_qsov_dn10 = assign102090_e154064_d_n10;
        locals.var_qsov_dn11 = assign102090_e154064_d_n11;
        locals.var_qsov_dn14 = assign102090_e154064_d_n14;

        let (assign102100_e154077, assign102100_e154077_d_n0, assign102100_e154077_d_n2, assign102100_e154077_d_n4, assign102100_e154077_d_n5, assign102100_e154077_d_n6, assign102100_e154077_d_n7, assign102100_e154077_d_n8, assign102100_e154077_d_n9, assign102100_e154077_d_n10, assign102100_e154077_d_n11, assign102100_e154077_d_n14,) = {
    if (locals.var_guard2333 != 0.0) {
        let assign102100_e154069: f64 = locals.var_qy;
        let assign102100_e154071: f64 = (assign102100_e154069 - locals.var_qovd_add);
        let assign102100_e154073: f64 = (assign102100_e154071 - locals.var_qovs_add);
        let assign102100_e154074: f64 = (locals.var_mfactor * assign102100_e154073);
        let assign102100_e154075: f64 = (locals.var_qge + assign102100_e154074);
        (assign102100_e154075, (locals.var_qge_dn0 + (locals.var_mfactor * ((locals.var_qy_dn0 - locals.var_qovd_add_dn0) - locals.var_qovs_add_dn0))), (locals.var_qge_dn2 + (locals.var_mfactor * ((locals.var_qy_dn2 - locals.var_qovd_add_dn2) - locals.var_qovs_add_dn2))), (locals.var_qge_dn4 + (locals.var_mfactor * ((locals.var_qy_dn4 - locals.var_qovd_add_dn4) - locals.var_qovs_add_dn4))), (locals.var_qge_dn5 + (locals.var_mfactor * ((locals.var_qy_dn5 - locals.var_qovd_add_dn5) - locals.var_qovs_add_dn5))), (locals.var_qge_dn6 + (locals.var_mfactor * ((locals.var_qy_dn6 - locals.var_qovd_add_dn6) - locals.var_qovs_add_dn6))), (locals.var_qge_dn7 + (locals.var_mfactor * ((locals.var_qy_dn7 - locals.var_qovd_add_dn7) - locals.var_qovs_add_dn7))), (locals.var_qge_dn8 + (locals.var_mfactor * ((locals.var_qy_dn8 - locals.var_qovd_add_dn8) - locals.var_qovs_add_dn8))), (locals.var_qge_dn9 + (locals.var_mfactor * ((locals.var_qy_dn9 - locals.var_qovd_add_dn9) - locals.var_qovs_add_dn9))), (locals.var_qge_dn10 + (locals.var_mfactor * ((locals.var_qy_dn10 - locals.var_qovd_add_dn10) - locals.var_qovs_add_dn10))), (locals.var_qge_dn11 + (locals.var_mfactor * ((locals.var_qy_dn11 - locals.var_qovd_add_dn11) - locals.var_qovs_add_dn11))), (locals.var_qge_dn14 + (locals.var_mfactor * ((locals.var_qy_dn14 - locals.var_qovd_add_dn14) - locals.var_qovs_add_dn14))),)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn4, locals.var_qge_dn5, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn8, locals.var_qge_dn9, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn14,)
    }
};
        locals.var_qge = assign102100_e154077;
        locals.var_qge_dn0 = assign102100_e154077_d_n0;
        locals.var_qge_dn2 = assign102100_e154077_d_n2;
        locals.var_qge_dn4 = assign102100_e154077_d_n4;
        locals.var_qge_dn5 = assign102100_e154077_d_n5;
        locals.var_qge_dn6 = assign102100_e154077_d_n6;
        locals.var_qge_dn7 = assign102100_e154077_d_n7;
        locals.var_qge_dn8 = assign102100_e154077_d_n8;
        locals.var_qge_dn9 = assign102100_e154077_d_n9;
        locals.var_qge_dn10 = assign102100_e154077_d_n10;
        locals.var_qge_dn11 = assign102100_e154077_d_n11;
        locals.var_qge_dn14 = assign102100_e154077_d_n14;

        let (assign102110_e154088, assign102110_e154088_d_n0, assign102110_e154088_d_n2, assign102110_e154088_d_n4, assign102110_e154088_d_n5, assign102110_e154088_d_n6, assign102110_e154088_d_n7, assign102110_e154088_d_n8, assign102110_e154088_d_n9, assign102110_e154088_d_n10, assign102110_e154088_d_n11, assign102110_e154088_d_n14,) = {
    if (locals.var_guard2333 != 0.0) {
        let assign102110_e154082: f64 = (-locals.var_qy);
        let assign102110_e154084: f64 = (assign102110_e154082 + locals.var_qbdld_add);
        let assign102110_e154085: f64 = (locals.var_mfactor * assign102110_e154084);
        let assign102110_e154086: f64 = (locals.var_qde + assign102110_e154085);
        (assign102110_e154086, (locals.var_qde_dn0 + (locals.var_mfactor * ((-locals.var_qy_dn0) + locals.var_qbdld_add_dn0))), (locals.var_qde_dn2 + (locals.var_mfactor * ((-locals.var_qy_dn2) + locals.var_qbdld_add_dn2))), (locals.var_qde_dn4 + (locals.var_mfactor * ((-locals.var_qy_dn4) + locals.var_qbdld_add_dn4))), (locals.var_qde_dn5 + (locals.var_mfactor * ((-locals.var_qy_dn5) + locals.var_qbdld_add_dn5))), (locals.var_qde_dn6 + (locals.var_mfactor * ((-locals.var_qy_dn6) + locals.var_qbdld_add_dn6))), (locals.var_qde_dn7 + (locals.var_mfactor * ((-locals.var_qy_dn7) + locals.var_qbdld_add_dn7))), (locals.var_qde_dn8 + (locals.var_mfactor * ((-locals.var_qy_dn8) + locals.var_qbdld_add_dn8))), (locals.var_qde_dn9 + (locals.var_mfactor * ((-locals.var_qy_dn9) + locals.var_qbdld_add_dn9))), (locals.var_qde_dn10 + (locals.var_mfactor * ((-locals.var_qy_dn10) + locals.var_qbdld_add_dn10))), (locals.var_qde_dn11 + (locals.var_mfactor * ((-locals.var_qy_dn11) + locals.var_qbdld_add_dn11))), (locals.var_qde_dn14 + (locals.var_mfactor * ((-locals.var_qy_dn14) + locals.var_qbdld_add_dn14))),)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn14,)
    }
};
        locals.var_qde = assign102110_e154088;
        locals.var_qde_dn0 = assign102110_e154088_d_n0;
        locals.var_qde_dn2 = assign102110_e154088_d_n2;
        locals.var_qde_dn4 = assign102110_e154088_d_n4;
        locals.var_qde_dn5 = assign102110_e154088_d_n5;
        locals.var_qde_dn6 = assign102110_e154088_d_n6;
        locals.var_qde_dn7 = assign102110_e154088_d_n7;
        locals.var_qde_dn8 = assign102110_e154088_d_n8;
        locals.var_qde_dn9 = assign102110_e154088_d_n9;
        locals.var_qde_dn10 = assign102110_e154088_d_n10;
        locals.var_qde_dn11 = assign102110_e154088_d_n11;
        locals.var_qde_dn14 = assign102110_e154088_d_n14;

        let (assign102120_e154097, assign102120_e154097_d_n0, assign102120_e154097_d_n2, assign102120_e154097_d_n4, assign102120_e154097_d_n5, assign102120_e154097_d_n6, assign102120_e154097_d_n7, assign102120_e154097_d_n8, assign102120_e154097_d_n9, assign102120_e154097_d_n10, assign102120_e154097_d_n11, assign102120_e154097_d_n14,) = {
    if (locals.var_guard2333 != 0.0) {
        let assign102120_e154093: f64 = locals.var_qbsld_add;
        let assign102120_e154094: f64 = (locals.var_mfactor * assign102120_e154093);
        let assign102120_e154095: f64 = (locals.var_qse + assign102120_e154094);
        (assign102120_e154095, (locals.var_qse_dn0 + (locals.var_mfactor * locals.var_qbsld_add_dn0)), (locals.var_qse_dn2 + (locals.var_mfactor * locals.var_qbsld_add_dn2)), (locals.var_qse_dn4 + (locals.var_mfactor * locals.var_qbsld_add_dn4)), (locals.var_qse_dn5 + (locals.var_mfactor * locals.var_qbsld_add_dn5)), (locals.var_qse_dn6 + (locals.var_mfactor * locals.var_qbsld_add_dn6)), (locals.var_qse_dn7 + (locals.var_mfactor * locals.var_qbsld_add_dn7)), (locals.var_qse_dn8 + (locals.var_mfactor * locals.var_qbsld_add_dn8)), (locals.var_qse_dn9 + (locals.var_mfactor * locals.var_qbsld_add_dn9)), (locals.var_qse_dn10 + (locals.var_mfactor * locals.var_qbsld_add_dn10)), (locals.var_qse_dn11 + (locals.var_mfactor * locals.var_qbsld_add_dn11)), (locals.var_qse_dn14 + (locals.var_mfactor * locals.var_qbsld_add_dn14)),)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn8, locals.var_qse_dn9, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn14,)
    }
};
        locals.var_qse = assign102120_e154097;
        locals.var_qse_dn0 = assign102120_e154097_d_n0;
        locals.var_qse_dn2 = assign102120_e154097_d_n2;
        locals.var_qse_dn4 = assign102120_e154097_d_n4;
        locals.var_qse_dn5 = assign102120_e154097_d_n5;
        locals.var_qse_dn6 = assign102120_e154097_d_n6;
        locals.var_qse_dn7 = assign102120_e154097_d_n7;
        locals.var_qse_dn8 = assign102120_e154097_d_n8;
        locals.var_qse_dn9 = assign102120_e154097_d_n9;
        locals.var_qse_dn10 = assign102120_e154097_d_n10;
        locals.var_qse_dn11 = assign102120_e154097_d_n11;
        locals.var_qse_dn14 = assign102120_e154097_d_n14;

        let (assign102130_e154106, assign102130_e154106_d_n0, assign102130_e154106_d_n2, assign102130_e154106_d_n4, assign102130_e154106_d_n5, assign102130_e154106_d_n6, assign102130_e154106_d_n7, assign102130_e154106_d_n8, assign102130_e154106_d_n9, assign102130_e154106_d_n10, assign102130_e154106_d_n11, assign102130_e154106_d_n14,) = {
    if (locals.var_guard2333 != 0.0) {
        let assign102130_e154101: f64 = (-locals.var_qovdext);
        let assign102130_e154103: f64 = (assign102130_e154101 - locals.var_qovsext);
        let assign102130_e154104: f64 = (locals.var_mfactor * assign102130_e154103);
        (assign102130_e154104, (locals.var_mfactor * ((-locals.var_qovdext_dn0) - locals.var_qovsext_dn0)), (locals.var_mfactor * ((-locals.var_qovdext_dn2) - locals.var_qovsext_dn2)), (locals.var_mfactor * ((-locals.var_qovdext_dn4) - locals.var_qovsext_dn4)), (locals.var_mfactor * ((-locals.var_qovdext_dn5) - locals.var_qovsext_dn5)), (locals.var_mfactor * ((-locals.var_qovdext_dn6) - locals.var_qovsext_dn6)), (locals.var_mfactor * ((-locals.var_qovdext_dn7) - locals.var_qovsext_dn7)), (locals.var_mfactor * ((-locals.var_qovdext_dn8) - locals.var_qovsext_dn8)), (locals.var_mfactor * ((-locals.var_qovdext_dn9) - locals.var_qovsext_dn9)), (locals.var_mfactor * ((-locals.var_qovdext_dn10) - locals.var_qovsext_dn10)), (locals.var_mfactor * ((-locals.var_qovdext_dn11) - locals.var_qovsext_dn11)), (locals.var_mfactor * ((-locals.var_qovdext_dn14) - locals.var_qovsext_dn14)),)
    } else {
        (locals.var_qgexte, locals.var_qgexte_dn0, locals.var_qgexte_dn2, locals.var_qgexte_dn4, locals.var_qgexte_dn5, locals.var_qgexte_dn6, locals.var_qgexte_dn7, locals.var_qgexte_dn8, locals.var_qgexte_dn9, locals.var_qgexte_dn10, locals.var_qgexte_dn11, locals.var_qgexte_dn14,)
    }
};
        locals.var_qgexte = assign102130_e154106;
        locals.var_qgexte_dn0 = assign102130_e154106_d_n0;
        locals.var_qgexte_dn2 = assign102130_e154106_d_n2;
        locals.var_qgexte_dn4 = assign102130_e154106_d_n4;
        locals.var_qgexte_dn5 = assign102130_e154106_d_n5;
        locals.var_qgexte_dn6 = assign102130_e154106_d_n6;
        locals.var_qgexte_dn7 = assign102130_e154106_d_n7;
        locals.var_qgexte_dn8 = assign102130_e154106_d_n8;
        locals.var_qgexte_dn9 = assign102130_e154106_d_n9;
        locals.var_qgexte_dn10 = assign102130_e154106_d_n10;
        locals.var_qgexte_dn11 = assign102130_e154106_d_n11;
        locals.var_qgexte_dn14 = assign102130_e154106_d_n14;

        let (assign102140_e154112, assign102140_e154112_d_n0, assign102140_e154112_d_n2, assign102140_e154112_d_n4, assign102140_e154112_d_n5, assign102140_e154112_d_n6, assign102140_e154112_d_n7, assign102140_e154112_d_n8, assign102140_e154112_d_n9, assign102140_e154112_d_n10, assign102140_e154112_d_n11, assign102140_e154112_d_n14,) = {
    if (locals.var_guard2333 != 0.0) {
        let assign102140_e154110: f64 = (locals.var_mfactor * locals.var_qbdldext);
        (assign102140_e154110, (locals.var_mfactor * locals.var_qbdldext_dn0), (locals.var_mfactor * locals.var_qbdldext_dn2), (locals.var_mfactor * locals.var_qbdldext_dn4), (locals.var_mfactor * locals.var_qbdldext_dn5), (locals.var_mfactor * locals.var_qbdldext_dn6), (locals.var_mfactor * locals.var_qbdldext_dn7), (locals.var_mfactor * locals.var_qbdldext_dn8), (locals.var_mfactor * locals.var_qbdldext_dn9), (locals.var_mfactor * locals.var_qbdldext_dn10), (locals.var_mfactor * locals.var_qbdldext_dn11), (locals.var_mfactor * locals.var_qbdldext_dn14),)
    } else {
        (locals.var_qdexte, locals.var_qdexte_dn0, locals.var_qdexte_dn2, locals.var_qdexte_dn4, locals.var_qdexte_dn5, locals.var_qdexte_dn6, locals.var_qdexte_dn7, locals.var_qdexte_dn8, locals.var_qdexte_dn9, locals.var_qdexte_dn10, locals.var_qdexte_dn11, locals.var_qdexte_dn14,)
    }
};
        locals.var_qdexte = assign102140_e154112;
        locals.var_qdexte_dn0 = assign102140_e154112_d_n0;
        locals.var_qdexte_dn2 = assign102140_e154112_d_n2;
        locals.var_qdexte_dn4 = assign102140_e154112_d_n4;
        locals.var_qdexte_dn5 = assign102140_e154112_d_n5;
        locals.var_qdexte_dn6 = assign102140_e154112_d_n6;
        locals.var_qdexte_dn7 = assign102140_e154112_d_n7;
        locals.var_qdexte_dn8 = assign102140_e154112_d_n8;
        locals.var_qdexte_dn9 = assign102140_e154112_d_n9;
        locals.var_qdexte_dn10 = assign102140_e154112_d_n10;
        locals.var_qdexte_dn11 = assign102140_e154112_d_n11;
        locals.var_qdexte_dn14 = assign102140_e154112_d_n14;

        let (assign102150_e154118, assign102150_e154118_d_n0, assign102150_e154118_d_n2, assign102150_e154118_d_n4, assign102150_e154118_d_n5, assign102150_e154118_d_n6, assign102150_e154118_d_n7, assign102150_e154118_d_n8, assign102150_e154118_d_n9, assign102150_e154118_d_n10, assign102150_e154118_d_n11, assign102150_e154118_d_n14,) = {
    if (locals.var_guard2333 != 0.0) {
        let assign102150_e154116: f64 = (locals.var_mfactor * locals.var_qbsldext);
        (assign102150_e154116, (locals.var_mfactor * locals.var_qbsldext_dn0), (locals.var_mfactor * locals.var_qbsldext_dn2), (locals.var_mfactor * locals.var_qbsldext_dn4), (locals.var_mfactor * locals.var_qbsldext_dn5), (locals.var_mfactor * locals.var_qbsldext_dn6), (locals.var_mfactor * locals.var_qbsldext_dn7), (locals.var_mfactor * locals.var_qbsldext_dn8), (locals.var_mfactor * locals.var_qbsldext_dn9), (locals.var_mfactor * locals.var_qbsldext_dn10), (locals.var_mfactor * locals.var_qbsldext_dn11), (locals.var_mfactor * locals.var_qbsldext_dn14),)
    } else {
        (locals.var_qsexte, locals.var_qsexte_dn0, locals.var_qsexte_dn2, locals.var_qsexte_dn4, locals.var_qsexte_dn5, locals.var_qsexte_dn6, locals.var_qsexte_dn7, locals.var_qsexte_dn8, locals.var_qsexte_dn9, locals.var_qsexte_dn10, locals.var_qsexte_dn11, locals.var_qsexte_dn14,)
    }
};
        locals.var_qsexte = assign102150_e154118;
        locals.var_qsexte_dn0 = assign102150_e154118_d_n0;
        locals.var_qsexte_dn2 = assign102150_e154118_d_n2;
        locals.var_qsexte_dn4 = assign102150_e154118_d_n4;
        locals.var_qsexte_dn5 = assign102150_e154118_d_n5;
        locals.var_qsexte_dn6 = assign102150_e154118_d_n6;
        locals.var_qsexte_dn7 = assign102150_e154118_d_n7;
        locals.var_qsexte_dn8 = assign102150_e154118_d_n8;
        locals.var_qsexte_dn9 = assign102150_e154118_d_n9;
        locals.var_qsexte_dn10 = assign102150_e154118_d_n10;
        locals.var_qsexte_dn11 = assign102150_e154118_d_n11;
        locals.var_qsexte_dn14 = assign102150_e154118_d_n14;

        let (assign102160_e154129, assign102160_e154129_d_n0, assign102160_e154129_d_n2, assign102160_e154129_d_n7,) = {
    if (locals.var_guard2333 != 0.0) {
        let assign102160_e154123: f64 = (-locals.var_qfd);
        let assign102160_e154125: f64 = (assign102160_e154123 - locals.var_qgdo);
        let assign102160_e154126: f64 = (locals.var_mfactor * assign102160_e154125);
        let assign102160_e154127: f64 = (locals.var_qdp + assign102160_e154126);
        (assign102160_e154127, (locals.var_qdp_dn0 + (locals.var_mfactor * ((-locals.var_qfd_dn0) - locals.var_qgdo_dn0))), (locals.var_qdp_dn2 + (locals.var_mfactor * ((-locals.var_qfd_dn2) - locals.var_qgdo_dn2))), (locals.var_qdp_dn7 + (locals.var_mfactor * ((-locals.var_qfd_dn7) - locals.var_qgdo_dn7))),)
    } else {
        (locals.var_qdp, locals.var_qdp_dn0, locals.var_qdp_dn2, locals.var_qdp_dn7,)
    }
};
        locals.var_qdp = assign102160_e154129;
        locals.var_qdp_dn0 = assign102160_e154129_d_n0;
        locals.var_qdp_dn2 = assign102160_e154129_d_n2;
        locals.var_qdp_dn7 = assign102160_e154129_d_n7;

        let (assign102170_e154140, assign102170_e154140_d_n2, assign102170_e154140_d_n7,) = {
    if (locals.var_guard2333 != 0.0) {
        let assign102170_e154134: f64 = (-locals.var_qfs);
        let assign102170_e154136: f64 = (assign102170_e154134 - locals.var_qgso);
        let assign102170_e154137: f64 = (locals.var_mfactor * assign102170_e154136);
        let assign102170_e154138: f64 = (locals.var_qsp + assign102170_e154137);
        (assign102170_e154138, (locals.var_qsp_dn2 + (locals.var_mfactor * ((-locals.var_qfs_dn2) - locals.var_qgso_dn2))), (locals.var_qsp_dn7 + (locals.var_mfactor * ((-locals.var_qfs_dn7) - locals.var_qgso_dn7))),)
    } else {
        (locals.var_qsp, locals.var_qsp_dn2, locals.var_qsp_dn7,)
    }
};
        locals.var_qsp = assign102170_e154140;
        locals.var_qsp_dn2 = assign102170_e154140_d_n2;
        locals.var_qsp_dn7 = assign102170_e154140_d_n7;

    }

    pub(super) fn stamp_transient_block_374(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign102180_e154144: f64 = (locals.var_isub + locals.var_isubibpc);
        let assign102180_e154145: f64 = (locals.var_mfactor * assign102180_e154144);
        locals.var_isube = assign102180_e154145;
        locals.var_isube_dn0 = (locals.var_mfactor * (locals.var_isub_dn0 + locals.var_isubibpc_dn0));
        locals.var_isube_dn2 = (locals.var_mfactor * (locals.var_isub_dn2 + locals.var_isubibpc_dn2));
        locals.var_isube_dn4 = (locals.var_mfactor * (locals.var_isub_dn4 + locals.var_isubibpc_dn4));
        locals.var_isube_dn5 = (locals.var_mfactor * (locals.var_isub_dn5 + locals.var_isubibpc_dn5));
        locals.var_isube_dn6 = (locals.var_mfactor * (locals.var_isub_dn6 + locals.var_isubibpc_dn6));
        locals.var_isube_dn7 = (locals.var_mfactor * (locals.var_isub_dn7 + locals.var_isubibpc_dn7));
        locals.var_isube_dn8 = (locals.var_mfactor * (locals.var_isub_dn8 + locals.var_isubibpc_dn8));
        locals.var_isube_dn9 = (locals.var_mfactor * (locals.var_isub_dn9 + locals.var_isubibpc_dn9));
        locals.var_isube_dn10 = (locals.var_mfactor * (locals.var_isub_dn10 + locals.var_isubibpc_dn10));
        locals.var_isube_dn11 = (locals.var_mfactor * (locals.var_isub_dn11 + locals.var_isubibpc_dn11));
        locals.var_isube_dn14 = (locals.var_mfactor * (locals.var_isub_dn14 + locals.var_isubibpc_dn14));

        let assign102190_e154148: f64 = (locals.var_mfactor * locals.var_isubld);
        locals.var_isublde = assign102190_e154148;
        locals.var_isublde_dn0 = (locals.var_mfactor * locals.var_isubld_dn0);
        locals.var_isublde_dn2 = (locals.var_mfactor * locals.var_isubld_dn2);
        locals.var_isublde_dn4 = (locals.var_mfactor * locals.var_isubld_dn4);
        locals.var_isublde_dn5 = (locals.var_mfactor * locals.var_isubld_dn5);
        locals.var_isublde_dn6 = (locals.var_mfactor * locals.var_isubld_dn6);
        locals.var_isublde_dn7 = (locals.var_mfactor * locals.var_isubld_dn7);
        locals.var_isublde_dn8 = (locals.var_mfactor * locals.var_isubld_dn8);
        locals.var_isublde_dn9 = (locals.var_mfactor * locals.var_isubld_dn9);
        locals.var_isublde_dn10 = (locals.var_mfactor * locals.var_isubld_dn10);
        locals.var_isublde_dn11 = (locals.var_mfactor * locals.var_isubld_dn11);
        locals.var_isublde_dn14 = (locals.var_mfactor * locals.var_isubld_dn14);

        let assign102310_e154215: f64 = (4.0 * 1.3806226e-23);
        let assign102310_e154217: f64 = (assign102310_e154215 * locals.var_ttemp);
        let assign102310_e154219: f64 = assign102310_e154217;
        locals.var_whi_noise = assign102310_e154219;
        locals.var_whi_noise_dn0 = (assign102310_e154215 * locals.var_ttemp_dn0);
        locals.var_whi_noise_dn2 = (assign102310_e154215 * locals.var_ttemp_dn2);
        locals.var_whi_noise_dn4 = (assign102310_e154215 * locals.var_ttemp_dn4);
        locals.var_whi_noise_dn5 = (assign102310_e154215 * locals.var_ttemp_dn5);
        locals.var_whi_noise_dn6 = (assign102310_e154215 * locals.var_ttemp_dn6);
        locals.var_whi_noise_dn7 = (assign102310_e154215 * locals.var_ttemp_dn7);
        locals.var_whi_noise_dn8 = (assign102310_e154215 * locals.var_ttemp_dn8);
        locals.var_whi_noise_dn9 = (assign102310_e154215 * locals.var_ttemp_dn9);
        locals.var_whi_noise_dn10 = (assign102310_e154215 * locals.var_ttemp_dn10);
        locals.var_whi_noise_dn11 = (assign102310_e154215 * locals.var_ttemp_dn11);
        locals.var_whi_noise_dn14 = (assign102310_e154215 * locals.var_ttemp_dn14);

        let assign102330_e154225: f64 = (locals.var_mfactor * locals.var_nthrml);
        locals.var_noithrml = assign102330_e154225;
        locals.var_noithrml_dn0 = (locals.var_mfactor * locals.var_nthrml_dn0);
        locals.var_noithrml_dn2 = (locals.var_mfactor * locals.var_nthrml_dn2);
        locals.var_noithrml_dn4 = (locals.var_mfactor * locals.var_nthrml_dn4);
        locals.var_noithrml_dn5 = (locals.var_mfactor * locals.var_nthrml_dn5);
        locals.var_noithrml_dn6 = (locals.var_mfactor * locals.var_nthrml_dn6);
        locals.var_noithrml_dn7 = (locals.var_mfactor * locals.var_nthrml_dn7);
        locals.var_noithrml_dn8 = (locals.var_mfactor * locals.var_nthrml_dn8);
        locals.var_noithrml_dn9 = (locals.var_mfactor * locals.var_nthrml_dn9);
        locals.var_noithrml_dn10 = (locals.var_mfactor * locals.var_nthrml_dn10);
        locals.var_noithrml_dn11 = (locals.var_mfactor * locals.var_nthrml_dn11);
        locals.var_noithrml_dn14 = (locals.var_mfactor * locals.var_nthrml_dn14);

        let assign102340_e154228: f64 = locals.var_qge_dn6;
        locals.var_cgdbd = assign102340_e154228;
        locals.var_cgdbd_dn0 = 0.0;
        locals.var_cgdbd_dn2 = 0.0;
        locals.var_cgdbd_dn4 = 0.0;
        locals.var_cgdbd_dn5 = 0.0;
        locals.var_cgdbd_dn6 = 0.0;
        locals.var_cgdbd_dn7 = 0.0;
        locals.var_cgdbd_dn8 = 0.0;
        locals.var_cgdbd_dn9 = 0.0;
        locals.var_cgdbd_dn10 = 0.0;
        locals.var_cgdbd_dn11 = 0.0;
        locals.var_cgdbd_dn14 = 0.0;

        let assign102350_e154231: f64 = (p.p87 * locals.var_cgdbd);
        locals.var_cgdbd = assign102350_e154231;
        locals.var_cgdbd_dn0 = (p.p87 * locals.var_cgdbd_dn0);
        locals.var_cgdbd_dn2 = (p.p87 * locals.var_cgdbd_dn2);
        locals.var_cgdbd_dn4 = (p.p87 * locals.var_cgdbd_dn4);
        locals.var_cgdbd_dn5 = (p.p87 * locals.var_cgdbd_dn5);
        locals.var_cgdbd_dn6 = (p.p87 * locals.var_cgdbd_dn6);
        locals.var_cgdbd_dn7 = (p.p87 * locals.var_cgdbd_dn7);
        locals.var_cgdbd_dn8 = (p.p87 * locals.var_cgdbd_dn8);
        locals.var_cgdbd_dn9 = (p.p87 * locals.var_cgdbd_dn9);
        locals.var_cgdbd_dn10 = (p.p87 * locals.var_cgdbd_dn10);
        locals.var_cgdbd_dn11 = (p.p87 * locals.var_cgdbd_dn11);
        locals.var_cgdbd_dn14 = (p.p87 * locals.var_cgdbd_dn14);

        let assign102360_e154234: f64 = locals.var_qge_dn8;
        locals.var_cgsbd = assign102360_e154234;
        locals.var_cgsbd_dn0 = 0.0;
        locals.var_cgsbd_dn2 = 0.0;
        locals.var_cgsbd_dn4 = 0.0;
        locals.var_cgsbd_dn5 = 0.0;
        locals.var_cgsbd_dn6 = 0.0;
        locals.var_cgsbd_dn7 = 0.0;
        locals.var_cgsbd_dn8 = 0.0;
        locals.var_cgsbd_dn9 = 0.0;
        locals.var_cgsbd_dn10 = 0.0;
        locals.var_cgsbd_dn11 = 0.0;
        locals.var_cgsbd_dn14 = 0.0;

        let assign102370_e154237: f64 = (p.p87 * locals.var_cgsbd);
        locals.var_cgsbd = assign102370_e154237;
        locals.var_cgsbd_dn0 = (p.p87 * locals.var_cgsbd_dn0);
        locals.var_cgsbd_dn2 = (p.p87 * locals.var_cgsbd_dn2);
        locals.var_cgsbd_dn4 = (p.p87 * locals.var_cgsbd_dn4);
        locals.var_cgsbd_dn5 = (p.p87 * locals.var_cgsbd_dn5);
        locals.var_cgsbd_dn6 = (p.p87 * locals.var_cgsbd_dn6);
        locals.var_cgsbd_dn7 = (p.p87 * locals.var_cgsbd_dn7);
        locals.var_cgsbd_dn8 = (p.p87 * locals.var_cgsbd_dn8);
        locals.var_cgsbd_dn9 = (p.p87 * locals.var_cgsbd_dn9);
        locals.var_cgsbd_dn10 = (p.p87 * locals.var_cgsbd_dn10);
        locals.var_cgsbd_dn11 = (p.p87 * locals.var_cgsbd_dn11);
        locals.var_cgsbd_dn14 = (p.p87 * locals.var_cgsbd_dn14);

        let (assign102380_e154243, assign102380_e154243_d_n0, assign102380_e154243_d_n2, assign102380_e154243_d_n4, assign102380_e154243_d_n5, assign102380_e154243_d_n6, assign102380_e154243_d_n7, assign102380_e154243_d_n8, assign102380_e154243_d_n9, assign102380_e154243_d_n10, assign102380_e154243_d_n11, assign102380_e154243_d_n14,) = {
    if (locals.var_mode > 0.0) {
        (locals.var_cgsbd, locals.var_cgsbd_dn0, locals.var_cgsbd_dn2, locals.var_cgsbd_dn4, locals.var_cgsbd_dn5, locals.var_cgsbd_dn6, locals.var_cgsbd_dn7, locals.var_cgsbd_dn8, locals.var_cgsbd_dn9, locals.var_cgsbd_dn10, locals.var_cgsbd_dn11, locals.var_cgsbd_dn14,)
    } else {
        (locals.var_cgdbd, locals.var_cgdbd_dn0, locals.var_cgdbd_dn2, locals.var_cgdbd_dn4, locals.var_cgdbd_dn5, locals.var_cgdbd_dn6, locals.var_cgdbd_dn7, locals.var_cgdbd_dn8, locals.var_cgdbd_dn9, locals.var_cgdbd_dn10, locals.var_cgdbd_dn11, locals.var_cgdbd_dn14,)
    }
};
        locals.var_cgsb = assign102380_e154243;
        locals.var_cgsb_dn0 = assign102380_e154243_d_n0;
        locals.var_cgsb_dn2 = assign102380_e154243_d_n2;
        locals.var_cgsb_dn4 = assign102380_e154243_d_n4;
        locals.var_cgsb_dn5 = assign102380_e154243_d_n5;
        locals.var_cgsb_dn6 = assign102380_e154243_d_n6;
        locals.var_cgsb_dn7 = assign102380_e154243_d_n7;
        locals.var_cgsb_dn8 = assign102380_e154243_d_n8;
        locals.var_cgsb_dn9 = assign102380_e154243_d_n9;
        locals.var_cgsb_dn10 = assign102380_e154243_d_n10;
        locals.var_cgsb_dn11 = assign102380_e154243_d_n11;
        locals.var_cgsb_dn14 = assign102380_e154243_d_n14;

        locals.var_noiigate = 0.0;
        locals.var_noiigate_dn0 = 0.0;
        locals.var_noiigate_dn2 = 0.0;
        locals.var_noiigate_dn4 = 0.0;
        locals.var_noiigate_dn5 = 0.0;
        locals.var_noiigate_dn6 = 0.0;
        locals.var_noiigate_dn7 = 0.0;
        locals.var_noiigate_dn8 = 0.0;
        locals.var_noiigate_dn9 = 0.0;
        locals.var_noiigate_dn10 = 0.0;
        locals.var_noiigate_dn11 = 0.0;
        locals.var_noiigate_dn14 = 0.0;

        locals.var_noicross = 0.0;
        locals.var_noicross_dn0 = 0.0;
        locals.var_noicross_dn2 = 0.0;
        locals.var_noicross_dn4 = 0.0;
        locals.var_noicross_dn5 = 0.0;
        locals.var_noicross_dn6 = 0.0;
        locals.var_noicross_dn7 = 0.0;
        locals.var_noicross_dn8 = 0.0;
        locals.var_noicross_dn9 = 0.0;
        locals.var_noicross_dn10 = 0.0;
        locals.var_noicross_dn11 = 0.0;
        locals.var_noicross_dn14 = 0.0;

        let assign102410_e154263: f64 = if (((((p.p31 != 0.0) && (p.p30 != 0.0)) && (locals.var_flg_ign == 1.0)) && (locals.var_flg_noqi == 0.0)) && (locals.var_uc_codep == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2336 = assign102410_e154263;

        let (assign102420_e154273, assign102420_e154273_d_n0, assign102420_e154273_d_n2, assign102420_e154273_d_n4, assign102420_e154273_d_n5, assign102420_e154273_d_n6, assign102420_e154273_d_n7, assign102420_e154273_d_n8, assign102420_e154273_d_n9, assign102420_e154273_d_n10, assign102420_e154273_d_n11, assign102420_e154273_d_n14,) = {
    if (locals.var_guard2336 != 0.0) {
        let assign102420_e154267: f64 = (1e-6 * locals.var_cox);
        let assign102420_e154269: f64 = (assign102420_e154267 * locals.var_weffcv_nf);
        let assign102420_e154271: f64 = (assign102420_e154269 * locals.var_leff);
        (assign102420_e154271, (((1e-6 * locals.var_cox_dn0) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn2) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn4) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn5) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn6) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn7) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn8) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn9) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn10) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn11) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn14) * locals.var_weffcv_nf) * locals.var_leff),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign102420_e154273;
        locals.var_t0_dn0 = assign102420_e154273_d_n0;
        locals.var_t0_dn2 = assign102420_e154273_d_n2;
        locals.var_t0_dn4 = assign102420_e154273_d_n4;
        locals.var_t0_dn5 = assign102420_e154273_d_n5;
        locals.var_t0_dn6 = assign102420_e154273_d_n6;
        locals.var_t0_dn7 = assign102420_e154273_d_n7;
        locals.var_t0_dn8 = assign102420_e154273_d_n8;
        locals.var_t0_dn9 = assign102420_e154273_d_n9;
        locals.var_t0_dn10 = assign102420_e154273_d_n10;
        locals.var_t0_dn11 = assign102420_e154273_d_n11;
        locals.var_t0_dn14 = assign102420_e154273_d_n14;

        let (assign102430_e154279, assign102430_e154279_d_n0, assign102430_e154279_d_n2, assign102430_e154279_d_n4, assign102430_e154279_d_n5, assign102430_e154279_d_n6, assign102430_e154279_d_n7, assign102430_e154279_d_n8, assign102430_e154279_d_n9, assign102430_e154279_d_n10, assign102430_e154279_d_n11, assign102430_e154279_d_n14,) = {
    if (locals.var_guard2336 != 0.0) {
        let assign102430_e154277: f64 = (locals.var_cgsb / locals.var_mfactor);
        (assign102430_e154277, (locals.var_cgsb_dn0 / locals.var_mfactor), (locals.var_cgsb_dn2 / locals.var_mfactor), (locals.var_cgsb_dn4 / locals.var_mfactor), (locals.var_cgsb_dn5 / locals.var_mfactor), (locals.var_cgsb_dn6 / locals.var_mfactor), (locals.var_cgsb_dn7 / locals.var_mfactor), (locals.var_cgsb_dn8 / locals.var_mfactor), (locals.var_cgsb_dn9 / locals.var_mfactor), (locals.var_cgsb_dn10 / locals.var_mfactor), (locals.var_cgsb_dn11 / locals.var_mfactor), (locals.var_cgsb_dn14 / locals.var_mfactor),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign102430_e154279;
        locals.var_t10_dn0 = assign102430_e154279_d_n0;
        locals.var_t10_dn2 = assign102430_e154279_d_n2;
        locals.var_t10_dn4 = assign102430_e154279_d_n4;
        locals.var_t10_dn5 = assign102430_e154279_d_n5;
        locals.var_t10_dn6 = assign102430_e154279_d_n6;
        locals.var_t10_dn7 = assign102430_e154279_d_n7;
        locals.var_t10_dn8 = assign102430_e154279_d_n8;
        locals.var_t10_dn9 = assign102430_e154279_d_n9;
        locals.var_t10_dn10 = assign102430_e154279_d_n10;
        locals.var_t10_dn11 = assign102430_e154279_d_n11;
        locals.var_t10_dn14 = assign102430_e154279_d_n14;

        let (assign102440_e154293, assign102440_e154293_d_n0, assign102440_e154293_d_n2, assign102440_e154293_d_n4, assign102440_e154293_d_n5, assign102440_e154293_d_n6, assign102440_e154293_d_n7, assign102440_e154293_d_n8, assign102440_e154293_d_n9, assign102440_e154293_d_n10, assign102440_e154293_d_n11, assign102440_e154293_d_n14,) = {
    if (locals.var_guard2336 != 0.0) {
        let assign102440_e154283: f64 = (0.1185185185185185 * 1.6021918e-19);
        let assign102440_e154285: f64 = (assign102440_e154283 * locals.var_beta_inv);
        let assign102440_e154287: f64 = (assign102440_e154285 * locals.var_t10);
        let assign102440_e154289: f64 = (assign102440_e154287 * locals.var_t10);
        let assign102440_e154291: f64 = (assign102440_e154289 / locals.var_gds0_ign);
        (assign102440_e154291, ((((((((assign102440_e154283 * locals.var_beta_inv_dn0) * locals.var_t10) + (assign102440_e154285 * locals.var_t10_dn0)) * locals.var_t10) + (assign102440_e154287 * locals.var_t10_dn0)) * locals.var_gds0_ign) - (assign102440_e154289 * locals.var_gds0_ign_dn0)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102440_e154283 * locals.var_beta_inv_dn2) * locals.var_t10) + (assign102440_e154285 * locals.var_t10_dn2)) * locals.var_t10) + (assign102440_e154287 * locals.var_t10_dn2)) * locals.var_gds0_ign) - (assign102440_e154289 * locals.var_gds0_ign_dn2)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102440_e154283 * locals.var_beta_inv_dn4) * locals.var_t10) + (assign102440_e154285 * locals.var_t10_dn4)) * locals.var_t10) + (assign102440_e154287 * locals.var_t10_dn4)) * locals.var_gds0_ign) - (assign102440_e154289 * locals.var_gds0_ign_dn4)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102440_e154283 * locals.var_beta_inv_dn5) * locals.var_t10) + (assign102440_e154285 * locals.var_t10_dn5)) * locals.var_t10) + (assign102440_e154287 * locals.var_t10_dn5)) * locals.var_gds0_ign) - (assign102440_e154289 * locals.var_gds0_ign_dn5)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102440_e154283 * locals.var_beta_inv_dn6) * locals.var_t10) + (assign102440_e154285 * locals.var_t10_dn6)) * locals.var_t10) + (assign102440_e154287 * locals.var_t10_dn6)) * locals.var_gds0_ign) - (assign102440_e154289 * locals.var_gds0_ign_dn6)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102440_e154283 * locals.var_beta_inv_dn7) * locals.var_t10) + (assign102440_e154285 * locals.var_t10_dn7)) * locals.var_t10) + (assign102440_e154287 * locals.var_t10_dn7)) * locals.var_gds0_ign) - (assign102440_e154289 * locals.var_gds0_ign_dn7)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102440_e154283 * locals.var_beta_inv_dn8) * locals.var_t10) + (assign102440_e154285 * locals.var_t10_dn8)) * locals.var_t10) + (assign102440_e154287 * locals.var_t10_dn8)) * locals.var_gds0_ign) - (assign102440_e154289 * locals.var_gds0_ign_dn8)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102440_e154283 * locals.var_beta_inv_dn9) * locals.var_t10) + (assign102440_e154285 * locals.var_t10_dn9)) * locals.var_t10) + (assign102440_e154287 * locals.var_t10_dn9)) * locals.var_gds0_ign) - (assign102440_e154289 * locals.var_gds0_ign_dn9)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102440_e154283 * locals.var_beta_inv_dn10) * locals.var_t10) + (assign102440_e154285 * locals.var_t10_dn10)) * locals.var_t10) + (assign102440_e154287 * locals.var_t10_dn10)) * locals.var_gds0_ign) - (assign102440_e154289 * locals.var_gds0_ign_dn10)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102440_e154283 * locals.var_beta_inv_dn11) * locals.var_t10) + (assign102440_e154285 * locals.var_t10_dn11)) * locals.var_t10) + (assign102440_e154287 * locals.var_t10_dn11)) * locals.var_gds0_ign) - (assign102440_e154289 * locals.var_gds0_ign_dn11)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102440_e154283 * locals.var_beta_inv_dn14) * locals.var_t10) + (assign102440_e154285 * locals.var_t10_dn14)) * locals.var_t10) + (assign102440_e154287 * locals.var_t10_dn14)) * locals.var_gds0_ign) - (assign102440_e154289 * locals.var_gds0_ign_dn14)) / (locals.var_gds0_ign * locals.var_gds0_ign)),)
    } else {
        (locals.var_nign0, locals.var_nign0_dn0, locals.var_nign0_dn2, locals.var_nign0_dn4, locals.var_nign0_dn5, locals.var_nign0_dn6, locals.var_nign0_dn7, locals.var_nign0_dn8, locals.var_nign0_dn9, locals.var_nign0_dn10, locals.var_nign0_dn11, locals.var_nign0_dn14,)
    }
};
        locals.var_nign0 = assign102440_e154293;
        locals.var_nign0_dn0 = assign102440_e154293_d_n0;
        locals.var_nign0_dn2 = assign102440_e154293_d_n2;
        locals.var_nign0_dn4 = assign102440_e154293_d_n4;
        locals.var_nign0_dn5 = assign102440_e154293_d_n5;
        locals.var_nign0_dn6 = assign102440_e154293_d_n6;
        locals.var_nign0_dn7 = assign102440_e154293_d_n7;
        locals.var_nign0_dn8 = assign102440_e154293_d_n8;
        locals.var_nign0_dn9 = assign102440_e154293_d_n9;
        locals.var_nign0_dn10 = assign102440_e154293_d_n10;
        locals.var_nign0_dn11 = assign102440_e154293_d_n11;
        locals.var_nign0_dn14 = assign102440_e154293_d_n14;

        let assign102450_e154297: f64 = (10.0 * 2.220446049250313e-16);
        let assign102450_e154302: f64 = (10.0 * 2.220446049250313e-16);
        let assign102450_e154304: f64 = if ((locals.var_kusai00l > assign102450_e154297) && (locals.var_vds > assign102450_e154302)) { 1.0 } else { 0.0 };
        locals.var_guard2337 = assign102450_e154304;

        let (assign102460_e154312, assign102460_e154312_d_n0, assign102460_e154312_d_n2, assign102460_e154312_d_n4, assign102460_e154312_d_n5, assign102460_e154312_d_n6, assign102460_e154312_d_n7, assign102460_e154312_d_n8, assign102460_e154312_d_n9, assign102460_e154312_d_n10, assign102460_e154312_d_n11, assign102460_e154312_d_n14,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 != 0.0)) {
        let assign102460_e154310: f64 = (locals.var_muun / locals.var_mu);
        (assign102460_e154310, (((locals.var_muun_dn0 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn0)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn2 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn2)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn4 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn4)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn5 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn5)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn6 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn6)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn7 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn7)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn8 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn8)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn9 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn9)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn10 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn10)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn11 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn11)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn14 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn14)) / (locals.var_mu * locals.var_mu)),)
    } else {
        (locals.var_mumoda, locals.var_mumoda_dn0, locals.var_mumoda_dn2, locals.var_mumoda_dn4, locals.var_mumoda_dn5, locals.var_mumoda_dn6, locals.var_mumoda_dn7, locals.var_mumoda_dn8, locals.var_mumoda_dn9, locals.var_mumoda_dn10, locals.var_mumoda_dn11, locals.var_mumoda_dn14,)
    }
};
        locals.var_mumoda = assign102460_e154312;
        locals.var_mumoda_dn0 = assign102460_e154312_d_n0;
        locals.var_mumoda_dn2 = assign102460_e154312_d_n2;
        locals.var_mumoda_dn4 = assign102460_e154312_d_n4;
        locals.var_mumoda_dn5 = assign102460_e154312_d_n5;
        locals.var_mumoda_dn6 = assign102460_e154312_d_n6;
        locals.var_mumoda_dn7 = assign102460_e154312_d_n7;
        locals.var_mumoda_dn8 = assign102460_e154312_d_n8;
        locals.var_mumoda_dn9 = assign102460_e154312_d_n9;
        locals.var_mumoda_dn10 = assign102460_e154312_d_n10;
        locals.var_mumoda_dn11 = assign102460_e154312_d_n11;
        locals.var_mumoda_dn14 = assign102460_e154312_d_n14;

        let (assign102470_e154324, assign102470_e154324_d_n0, assign102470_e154324_d_n2, assign102470_e154324_d_n4, assign102470_e154324_d_n5, assign102470_e154324_d_n6, assign102470_e154324_d_n7, assign102470_e154324_d_n8, assign102470_e154324_d_n9, assign102470_e154324_d_n10, assign102470_e154324_d_n11, assign102470_e154324_d_n14,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 != 0.0)) {
        let assign102470_e154318: f64 = (locals.var_muun / locals.var_mud_hoso);
        let assign102470_e154320: f64 = (assign102470_e154318 - locals.var_mumoda);
        let assign102470_e154322: f64 = (assign102470_e154320 / locals.var_vds);
        (assign102470_e154322, (((((((locals.var_muun_dn0 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn0)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn0) * locals.var_vds) - (assign102470_e154320 * locals.var_vds_dn0)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn2 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn2)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn2) * locals.var_vds) - (assign102470_e154320 * locals.var_vds_dn2)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn4 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn4)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn4) * locals.var_vds) - (assign102470_e154320 * locals.var_vds_dn4)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn5 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn5)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn5) * locals.var_vds) - (assign102470_e154320 * locals.var_vds_dn5)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn6 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn6)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn6) * locals.var_vds) - (assign102470_e154320 * locals.var_vds_dn6)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn7 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn7)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn7) * locals.var_vds) - (assign102470_e154320 * locals.var_vds_dn7)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn8 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn8)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn8) * locals.var_vds) - (assign102470_e154320 * locals.var_vds_dn8)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn9 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn9)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn9) * locals.var_vds) - (assign102470_e154320 * locals.var_vds_dn9)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn10 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn10)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn10) * locals.var_vds) - (assign102470_e154320 * locals.var_vds_dn10)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn11 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn11)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn11) * locals.var_vds) - (assign102470_e154320 * locals.var_vds_dn11)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn14 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn14)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn14) * locals.var_vds) - (assign102470_e154320 * locals.var_vds_dn14)) / (locals.var_vds * locals.var_vds)),)
    } else {
        (locals.var_mumodb, locals.var_mumodb_dn0, locals.var_mumodb_dn2, locals.var_mumodb_dn4, locals.var_mumodb_dn5, locals.var_mumodb_dn6, locals.var_mumodb_dn7, locals.var_mumodb_dn8, locals.var_mumodb_dn9, locals.var_mumodb_dn10, locals.var_mumodb_dn11, locals.var_mumodb_dn14,)
    }
};
        locals.var_mumodb = assign102470_e154324;
        locals.var_mumodb_dn0 = assign102470_e154324_d_n0;
        locals.var_mumodb_dn2 = assign102470_e154324_d_n2;
        locals.var_mumodb_dn4 = assign102470_e154324_d_n4;
        locals.var_mumodb_dn5 = assign102470_e154324_d_n5;
        locals.var_mumodb_dn6 = assign102470_e154324_d_n6;
        locals.var_mumodb_dn7 = assign102470_e154324_d_n7;
        locals.var_mumodb_dn8 = assign102470_e154324_d_n8;
        locals.var_mumodb_dn9 = assign102470_e154324_d_n9;
        locals.var_mumodb_dn10 = assign102470_e154324_d_n10;
        locals.var_mumodb_dn11 = assign102470_e154324_d_n11;
        locals.var_mumodb_dn14 = assign102470_e154324_d_n14;

        let (assign102480_e154346, assign102480_e154346_d_n0, assign102480_e154346_d_n2, assign102480_e154346_d_n4, assign102480_e154346_d_n5, assign102480_e154346_d_n6, assign102480_e154346_d_n7, assign102480_e154346_d_n8, assign102480_e154346_d_n9, assign102480_e154346_d_n10, assign102480_e154346_d_n11, assign102480_e154346_d_n14,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 != 0.0)) {
        let assign102480_e154331: f64 = (0.6666666666666667 * locals.var_mumodb);
        let assign102480_e154335: f64 = (locals.var_vgvt * locals.var_sqrtkusail);
        let assign102480_e154336: f64 = (locals.var_kusai00 + assign102480_e154335);
        let assign102480_e154338: f64 = (assign102480_e154336 + locals.var_kusail);
        let assign102480_e154339: f64 = (assign102480_e154331 * assign102480_e154338);
        let assign102480_e154342: f64 = (locals.var_vgvt + locals.var_sqrtkusail);
        let assign102480_e154343: f64 = (assign102480_e154339 / assign102480_e154342);
        let assign102480_e154344: f64 = (locals.var_mumoda + assign102480_e154343);
        (assign102480_e154344, (locals.var_mumoda_dn0 + ((((((0.6666666666666667 * locals.var_mumodb_dn0) * assign102480_e154338) + (assign102480_e154331 * ((locals.var_kusai00_dn0 + ((locals.var_vgvt_dn0 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn0))) + locals.var_kusail_dn0))) * assign102480_e154342) - (assign102480_e154339 * (locals.var_vgvt_dn0 + locals.var_sqrtkusail_dn0))) / (assign102480_e154342 * assign102480_e154342))), (locals.var_mumoda_dn2 + ((((((0.6666666666666667 * locals.var_mumodb_dn2) * assign102480_e154338) + (assign102480_e154331 * ((locals.var_kusai00_dn2 + ((locals.var_vgvt_dn2 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn2))) + locals.var_kusail_dn2))) * assign102480_e154342) - (assign102480_e154339 * (locals.var_vgvt_dn2 + locals.var_sqrtkusail_dn2))) / (assign102480_e154342 * assign102480_e154342))), (locals.var_mumoda_dn4 + ((((((0.6666666666666667 * locals.var_mumodb_dn4) * assign102480_e154338) + (assign102480_e154331 * ((locals.var_kusai00_dn4 + ((locals.var_vgvt_dn4 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn4))) + locals.var_kusail_dn4))) * assign102480_e154342) - (assign102480_e154339 * (locals.var_vgvt_dn4 + locals.var_sqrtkusail_dn4))) / (assign102480_e154342 * assign102480_e154342))), (locals.var_mumoda_dn5 + ((((((0.6666666666666667 * locals.var_mumodb_dn5) * assign102480_e154338) + (assign102480_e154331 * ((locals.var_kusai00_dn5 + ((locals.var_vgvt_dn5 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn5))) + locals.var_kusail_dn5))) * assign102480_e154342) - (assign102480_e154339 * (locals.var_vgvt_dn5 + locals.var_sqrtkusail_dn5))) / (assign102480_e154342 * assign102480_e154342))), (locals.var_mumoda_dn6 + ((((((0.6666666666666667 * locals.var_mumodb_dn6) * assign102480_e154338) + (assign102480_e154331 * ((locals.var_kusai00_dn6 + ((locals.var_vgvt_dn6 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn6))) + locals.var_kusail_dn6))) * assign102480_e154342) - (assign102480_e154339 * (locals.var_vgvt_dn6 + locals.var_sqrtkusail_dn6))) / (assign102480_e154342 * assign102480_e154342))), (locals.var_mumoda_dn7 + ((((((0.6666666666666667 * locals.var_mumodb_dn7) * assign102480_e154338) + (assign102480_e154331 * ((locals.var_kusai00_dn7 + ((locals.var_vgvt_dn7 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn7))) + locals.var_kusail_dn7))) * assign102480_e154342) - (assign102480_e154339 * (locals.var_vgvt_dn7 + locals.var_sqrtkusail_dn7))) / (assign102480_e154342 * assign102480_e154342))), (locals.var_mumoda_dn8 + ((((((0.6666666666666667 * locals.var_mumodb_dn8) * assign102480_e154338) + (assign102480_e154331 * ((locals.var_kusai00_dn8 + ((locals.var_vgvt_dn8 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn8))) + locals.var_kusail_dn8))) * assign102480_e154342) - (assign102480_e154339 * (locals.var_vgvt_dn8 + locals.var_sqrtkusail_dn8))) / (assign102480_e154342 * assign102480_e154342))), (locals.var_mumoda_dn9 + ((((((0.6666666666666667 * locals.var_mumodb_dn9) * assign102480_e154338) + (assign102480_e154331 * ((locals.var_kusai00_dn9 + ((locals.var_vgvt_dn9 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn9))) + locals.var_kusail_dn9))) * assign102480_e154342) - (assign102480_e154339 * (locals.var_vgvt_dn9 + locals.var_sqrtkusail_dn9))) / (assign102480_e154342 * assign102480_e154342))), (locals.var_mumoda_dn10 + ((((((0.6666666666666667 * locals.var_mumodb_dn10) * assign102480_e154338) + (assign102480_e154331 * ((locals.var_kusai00_dn10 + ((locals.var_vgvt_dn10 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn10))) + locals.var_kusail_dn10))) * assign102480_e154342) - (assign102480_e154339 * (locals.var_vgvt_dn10 + locals.var_sqrtkusail_dn10))) / (assign102480_e154342 * assign102480_e154342))), (locals.var_mumoda_dn11 + ((((((0.6666666666666667 * locals.var_mumodb_dn11) * assign102480_e154338) + (assign102480_e154331 * ((locals.var_kusai00_dn11 + ((locals.var_vgvt_dn11 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn11))) + locals.var_kusail_dn11))) * assign102480_e154342) - (assign102480_e154339 * (locals.var_vgvt_dn11 + locals.var_sqrtkusail_dn11))) / (assign102480_e154342 * assign102480_e154342))), (locals.var_mumoda_dn14 + ((((((0.6666666666666667 * locals.var_mumodb_dn14) * assign102480_e154338) + (assign102480_e154331 * ((locals.var_kusai00_dn14 + ((locals.var_vgvt_dn14 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn14))) + locals.var_kusail_dn14))) * assign102480_e154342) - (assign102480_e154339 * (locals.var_vgvt_dn14 + locals.var_sqrtkusail_dn14))) / (assign102480_e154342 * assign102480_e154342))),)
    } else {
        (locals.var_correct_w1, locals.var_correct_w1_dn0, locals.var_correct_w1_dn2, locals.var_correct_w1_dn4, locals.var_correct_w1_dn5, locals.var_correct_w1_dn6, locals.var_correct_w1_dn7, locals.var_correct_w1_dn8, locals.var_correct_w1_dn9, locals.var_correct_w1_dn10, locals.var_correct_w1_dn11, locals.var_correct_w1_dn14,)
    }
};
        locals.var_correct_w1 = assign102480_e154346;
        locals.var_correct_w1_dn0 = assign102480_e154346_d_n0;
        locals.var_correct_w1_dn2 = assign102480_e154346_d_n2;
        locals.var_correct_w1_dn4 = assign102480_e154346_d_n4;
        locals.var_correct_w1_dn5 = assign102480_e154346_d_n5;
        locals.var_correct_w1_dn6 = assign102480_e154346_d_n6;
        locals.var_correct_w1_dn7 = assign102480_e154346_d_n7;
        locals.var_correct_w1_dn8 = assign102480_e154346_d_n8;
        locals.var_correct_w1_dn9 = assign102480_e154346_d_n9;
        locals.var_correct_w1_dn10 = assign102480_e154346_d_n10;
        locals.var_correct_w1_dn11 = assign102480_e154346_d_n11;
        locals.var_correct_w1_dn14 = assign102480_e154346_d_n14;

        let (assign102490_e154355, assign102490_e154355_d_n0, assign102490_e154355_d_n2, assign102490_e154355_d_n4, assign102490_e154355_d_n5, assign102490_e154355_d_n6, assign102490_e154355_d_n7, assign102490_e154355_d_n8, assign102490_e154355_d_n9, assign102490_e154355_d_n10, assign102490_e154355_d_n11, assign102490_e154355_d_n14,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 == 0.0)) {
        let assign102490_e154353: f64 = (locals.var_muun / locals.var_mud_hoso);
        (assign102490_e154353, (((locals.var_muun_dn0 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn0)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn2 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn2)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn4 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn4)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn5 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn5)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn6 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn6)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn7 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn7)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn8 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn8)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn9 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn9)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn10 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn10)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn11 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn11)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn14 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn14)) / (locals.var_mud_hoso * locals.var_mud_hoso)),)
    } else {
        (locals.var_correct_w1, locals.var_correct_w1_dn0, locals.var_correct_w1_dn2, locals.var_correct_w1_dn4, locals.var_correct_w1_dn5, locals.var_correct_w1_dn6, locals.var_correct_w1_dn7, locals.var_correct_w1_dn8, locals.var_correct_w1_dn9, locals.var_correct_w1_dn10, locals.var_correct_w1_dn11, locals.var_correct_w1_dn14,)
    }
};
        locals.var_correct_w1 = assign102490_e154355;
        locals.var_correct_w1_dn0 = assign102490_e154355_d_n0;
        locals.var_correct_w1_dn2 = assign102490_e154355_d_n2;
        locals.var_correct_w1_dn4 = assign102490_e154355_d_n4;
        locals.var_correct_w1_dn5 = assign102490_e154355_d_n5;
        locals.var_correct_w1_dn6 = assign102490_e154355_d_n6;
        locals.var_correct_w1_dn7 = assign102490_e154355_d_n7;
        locals.var_correct_w1_dn8 = assign102490_e154355_d_n8;
        locals.var_correct_w1_dn9 = assign102490_e154355_d_n9;
        locals.var_correct_w1_dn10 = assign102490_e154355_d_n10;
        locals.var_correct_w1_dn11 = assign102490_e154355_d_n11;
        locals.var_correct_w1_dn14 = assign102490_e154355_d_n14;

        let (assign102500_e154365, assign102500_e154365_d_n0, assign102500_e154365_d_n2, assign102500_e154365_d_n4, assign102500_e154365_d_n5, assign102500_e154365_d_n6, assign102500_e154365_d_n7, assign102500_e154365_d_n8, assign102500_e154365_d_n9, assign102500_e154365_d_n10, assign102500_e154365_d_n11, assign102500_e154365_d_n14,) = {
    if (locals.var_guard2336 != 0.0) {
        let assign102500_e154359: f64 = (locals.var_mfactor * locals.var_nign0);
        let assign102500_e154361: f64 = (assign102500_e154359 * locals.var_kusai_ig);
        let assign102500_e154363: f64 = (assign102500_e154361 * locals.var_correct_w1);
        (assign102500_e154363, (((((locals.var_mfactor * locals.var_nign0_dn0) * locals.var_kusai_ig) + (assign102500_e154359 * locals.var_kusai_ig_dn0)) * locals.var_correct_w1) + (assign102500_e154361 * locals.var_correct_w1_dn0)), (((((locals.var_mfactor * locals.var_nign0_dn2) * locals.var_kusai_ig) + (assign102500_e154359 * locals.var_kusai_ig_dn2)) * locals.var_correct_w1) + (assign102500_e154361 * locals.var_correct_w1_dn2)), (((((locals.var_mfactor * locals.var_nign0_dn4) * locals.var_kusai_ig) + (assign102500_e154359 * locals.var_kusai_ig_dn4)) * locals.var_correct_w1) + (assign102500_e154361 * locals.var_correct_w1_dn4)), (((((locals.var_mfactor * locals.var_nign0_dn5) * locals.var_kusai_ig) + (assign102500_e154359 * locals.var_kusai_ig_dn5)) * locals.var_correct_w1) + (assign102500_e154361 * locals.var_correct_w1_dn5)), (((((locals.var_mfactor * locals.var_nign0_dn6) * locals.var_kusai_ig) + (assign102500_e154359 * locals.var_kusai_ig_dn6)) * locals.var_correct_w1) + (assign102500_e154361 * locals.var_correct_w1_dn6)), (((((locals.var_mfactor * locals.var_nign0_dn7) * locals.var_kusai_ig) + (assign102500_e154359 * locals.var_kusai_ig_dn7)) * locals.var_correct_w1) + (assign102500_e154361 * locals.var_correct_w1_dn7)), (((((locals.var_mfactor * locals.var_nign0_dn8) * locals.var_kusai_ig) + (assign102500_e154359 * locals.var_kusai_ig_dn8)) * locals.var_correct_w1) + (assign102500_e154361 * locals.var_correct_w1_dn8)), (((((locals.var_mfactor * locals.var_nign0_dn9) * locals.var_kusai_ig) + (assign102500_e154359 * locals.var_kusai_ig_dn9)) * locals.var_correct_w1) + (assign102500_e154361 * locals.var_correct_w1_dn9)), (((((locals.var_mfactor * locals.var_nign0_dn10) * locals.var_kusai_ig) + (assign102500_e154359 * locals.var_kusai_ig_dn10)) * locals.var_correct_w1) + (assign102500_e154361 * locals.var_correct_w1_dn10)), (((((locals.var_mfactor * locals.var_nign0_dn11) * locals.var_kusai_ig) + (assign102500_e154359 * locals.var_kusai_ig_dn11)) * locals.var_correct_w1) + (assign102500_e154361 * locals.var_correct_w1_dn11)), (((((locals.var_mfactor * locals.var_nign0_dn14) * locals.var_kusai_ig) + (assign102500_e154359 * locals.var_kusai_ig_dn14)) * locals.var_correct_w1) + (assign102500_e154361 * locals.var_correct_w1_dn14)),)
    } else {
        (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn4, locals.var_noiigate_dn5, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn8, locals.var_noiigate_dn9, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn14,)
    }
};
        locals.var_noiigate = assign102500_e154365;
        locals.var_noiigate_dn0 = assign102500_e154365_d_n0;
        locals.var_noiigate_dn2 = assign102500_e154365_d_n2;
        locals.var_noiigate_dn4 = assign102500_e154365_d_n4;
        locals.var_noiigate_dn5 = assign102500_e154365_d_n5;
        locals.var_noiigate_dn6 = assign102500_e154365_d_n6;
        locals.var_noiigate_dn7 = assign102500_e154365_d_n7;
        locals.var_noiigate_dn8 = assign102500_e154365_d_n8;
        locals.var_noiigate_dn9 = assign102500_e154365_d_n9;
        locals.var_noiigate_dn10 = assign102500_e154365_d_n10;
        locals.var_noiigate_dn11 = assign102500_e154365_d_n11;
        locals.var_noiigate_dn14 = assign102500_e154365_d_n14;

        let (assign102510_e154369, assign102510_e154369_d_n0, assign102510_e154369_d_n2, assign102510_e154369_d_n4, assign102510_e154369_d_n5, assign102510_e154369_d_n6, assign102510_e154369_d_n7, assign102510_e154369_d_n8, assign102510_e154369_d_n9, assign102510_e154369_d_n10, assign102510_e154369_d_n11, assign102510_e154369_d_n14,) = {
    if (locals.var_guard2336 != 0.0) {
        (locals.var_crl_f, locals.var_crl_f_dn0, locals.var_crl_f_dn2, locals.var_crl_f_dn4, locals.var_crl_f_dn5, locals.var_crl_f_dn6, locals.var_crl_f_dn7, locals.var_crl_f_dn8, locals.var_crl_f_dn9, locals.var_crl_f_dn10, locals.var_crl_f_dn11, locals.var_crl_f_dn14,)
    } else {
        (locals.var_noicross, locals.var_noicross_dn0, locals.var_noicross_dn2, locals.var_noicross_dn4, locals.var_noicross_dn5, locals.var_noicross_dn6, locals.var_noicross_dn7, locals.var_noicross_dn8, locals.var_noicross_dn9, locals.var_noicross_dn10, locals.var_noicross_dn11, locals.var_noicross_dn14,)
    }
};
        locals.var_noicross = assign102510_e154369;
        locals.var_noicross_dn0 = assign102510_e154369_d_n0;
        locals.var_noicross_dn2 = assign102510_e154369_d_n2;
        locals.var_noicross_dn4 = assign102510_e154369_d_n4;
        locals.var_noicross_dn5 = assign102510_e154369_d_n5;
        locals.var_noicross_dn6 = assign102510_e154369_d_n6;
        locals.var_noicross_dn7 = assign102510_e154369_d_n7;
        locals.var_noicross_dn8 = assign102510_e154369_d_n8;
        locals.var_noicross_dn9 = assign102510_e154369_d_n9;
        locals.var_noicross_dn10 = assign102510_e154369_d_n10;
        locals.var_noicross_dn11 = assign102510_e154369_d_n11;
        locals.var_noicross_dn14 = assign102510_e154369_d_n14;

        let (assign102520_e154378, assign102520_e154378_d_n0, assign102520_e154378_d_n2, assign102520_e154378_d_n4, assign102520_e154378_d_n5, assign102520_e154378_d_n6, assign102520_e154378_d_n7, assign102520_e154378_d_n8, assign102520_e154378_d_n9, assign102520_e154378_d_n10, assign102520_e154378_d_n11, assign102520_e154378_d_n14,) = {
    if (locals.var_guard2336 != 0.0) {
        let (assign102520_e154376, assign102520_e154376_d_n0, assign102520_e154376_d_n2, assign102520_e154376_d_n4, assign102520_e154376_d_n5, assign102520_e154376_d_n6, assign102520_e154376_d_n7, assign102520_e154376_d_n8, assign102520_e154376_d_n9, assign102520_e154376_d_n10, assign102520_e154376_d_n11, assign102520_e154376_d_n14,) = {
            if (locals.var_noiigate < 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn4, locals.var_noiigate_dn5, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn8, locals.var_noiigate_dn9, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn14,)
            }
        };
        (assign102520_e154376, assign102520_e154376_d_n0, assign102520_e154376_d_n2, assign102520_e154376_d_n4, assign102520_e154376_d_n5, assign102520_e154376_d_n6, assign102520_e154376_d_n7, assign102520_e154376_d_n8, assign102520_e154376_d_n9, assign102520_e154376_d_n10, assign102520_e154376_d_n11, assign102520_e154376_d_n14,)
    } else {
        (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn4, locals.var_noiigate_dn5, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn8, locals.var_noiigate_dn9, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn14,)
    }
};
        locals.var_noiigate = assign102520_e154378;
        locals.var_noiigate_dn0 = assign102520_e154378_d_n0;
        locals.var_noiigate_dn2 = assign102520_e154378_d_n2;
        locals.var_noiigate_dn4 = assign102520_e154378_d_n4;
        locals.var_noiigate_dn5 = assign102520_e154378_d_n5;
        locals.var_noiigate_dn6 = assign102520_e154378_d_n6;
        locals.var_noiigate_dn7 = assign102520_e154378_d_n7;
        locals.var_noiigate_dn8 = assign102520_e154378_d_n8;
        locals.var_noiigate_dn9 = assign102520_e154378_d_n9;
        locals.var_noiigate_dn10 = assign102520_e154378_d_n10;
        locals.var_noiigate_dn11 = assign102520_e154378_d_n11;
        locals.var_noiigate_dn14 = assign102520_e154378_d_n14;

        let (assign102530_e154388, assign102530_e154388_d_n0, assign102530_e154388_d_n2, assign102530_e154388_d_n4, assign102530_e154388_d_n5, assign102530_e154388_d_n6, assign102530_e154388_d_n7, assign102530_e154388_d_n8, assign102530_e154388_d_n9, assign102530_e154388_d_n10, assign102530_e154388_d_n11, assign102530_e154388_d_n14,) = {
    if (locals.var_guard2336 != 0.0) {
        let assign102530_e154381: f64 = (-locals.var_t10);
        let (assign102530_e154386, assign102530_e154386_d_n0, assign102530_e154386_d_n2, assign102530_e154386_d_n4, assign102530_e154386_d_n5, assign102530_e154386_d_n6, assign102530_e154386_d_n7, assign102530_e154386_d_n8, assign102530_e154386_d_n9, assign102530_e154386_d_n10, assign102530_e154386_d_n11, assign102530_e154386_d_n14,) = {
            if (assign102530_e154381 > locals.var_t0) {
                (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn4, locals.var_noiigate_dn5, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn8, locals.var_noiigate_dn9, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn14,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign102530_e154386, assign102530_e154386_d_n0, assign102530_e154386_d_n2, assign102530_e154386_d_n4, assign102530_e154386_d_n5, assign102530_e154386_d_n6, assign102530_e154386_d_n7, assign102530_e154386_d_n8, assign102530_e154386_d_n9, assign102530_e154386_d_n10, assign102530_e154386_d_n11, assign102530_e154386_d_n14,)
    } else {
        (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn4, locals.var_noiigate_dn5, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn8, locals.var_noiigate_dn9, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn14,)
    }
};
        locals.var_noiigate = assign102530_e154388;
        locals.var_noiigate_dn0 = assign102530_e154388_d_n0;
        locals.var_noiigate_dn2 = assign102530_e154388_d_n2;
        locals.var_noiigate_dn4 = assign102530_e154388_d_n4;
        locals.var_noiigate_dn5 = assign102530_e154388_d_n5;
        locals.var_noiigate_dn6 = assign102530_e154388_d_n6;
        locals.var_noiigate_dn7 = assign102530_e154388_d_n7;
        locals.var_noiigate_dn8 = assign102530_e154388_d_n8;
        locals.var_noiigate_dn9 = assign102530_e154388_d_n9;
        locals.var_noiigate_dn10 = assign102530_e154388_d_n10;
        locals.var_noiigate_dn11 = assign102530_e154388_d_n11;
        locals.var_noiigate_dn14 = assign102530_e154388_d_n14;

        let (assign102540_e154398, assign102540_e154398_d_n0, assign102540_e154398_d_n2, assign102540_e154398_d_n4, assign102540_e154398_d_n5, assign102540_e154398_d_n6, assign102540_e154398_d_n7, assign102540_e154398_d_n8, assign102540_e154398_d_n9, assign102540_e154398_d_n10, assign102540_e154398_d_n11, assign102540_e154398_d_n14,) = {
    if (locals.var_guard2336 != 0.0) {
        let assign102540_e154391: f64 = (-locals.var_t10);
        let (assign102540_e154396, assign102540_e154396_d_n0, assign102540_e154396_d_n2, assign102540_e154396_d_n4, assign102540_e154396_d_n5, assign102540_e154396_d_n6, assign102540_e154396_d_n7, assign102540_e154396_d_n8, assign102540_e154396_d_n9, assign102540_e154396_d_n10, assign102540_e154396_d_n11, assign102540_e154396_d_n14,) = {
            if (assign102540_e154391 > locals.var_t0) {
                (locals.var_noicross, locals.var_noicross_dn0, locals.var_noicross_dn2, locals.var_noicross_dn4, locals.var_noicross_dn5, locals.var_noicross_dn6, locals.var_noicross_dn7, locals.var_noicross_dn8, locals.var_noicross_dn9, locals.var_noicross_dn10, locals.var_noicross_dn11, locals.var_noicross_dn14,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign102540_e154396, assign102540_e154396_d_n0, assign102540_e154396_d_n2, assign102540_e154396_d_n4, assign102540_e154396_d_n5, assign102540_e154396_d_n6, assign102540_e154396_d_n7, assign102540_e154396_d_n8, assign102540_e154396_d_n9, assign102540_e154396_d_n10, assign102540_e154396_d_n11, assign102540_e154396_d_n14,)
    } else {
        (locals.var_noicross, locals.var_noicross_dn0, locals.var_noicross_dn2, locals.var_noicross_dn4, locals.var_noicross_dn5, locals.var_noicross_dn6, locals.var_noicross_dn7, locals.var_noicross_dn8, locals.var_noicross_dn9, locals.var_noicross_dn10, locals.var_noicross_dn11, locals.var_noicross_dn14,)
    }
};
        locals.var_noicross = assign102540_e154398;
        locals.var_noicross_dn0 = assign102540_e154398_d_n0;
        locals.var_noicross_dn2 = assign102540_e154398_d_n2;
        locals.var_noicross_dn4 = assign102540_e154398_d_n4;
        locals.var_noicross_dn5 = assign102540_e154398_d_n5;
        locals.var_noicross_dn6 = assign102540_e154398_d_n6;
        locals.var_noicross_dn7 = assign102540_e154398_d_n7;
        locals.var_noicross_dn8 = assign102540_e154398_d_n8;
        locals.var_noicross_dn9 = assign102540_e154398_d_n9;
        locals.var_noicross_dn10 = assign102540_e154398_d_n10;
        locals.var_noicross_dn11 = assign102540_e154398_d_n11;
        locals.var_noicross_dn14 = assign102540_e154398_d_n14;

        let assign102550_e154401: f64 = (locals.var_whi_noise * locals.var_noithrml);
        locals.var_sid = assign102550_e154401;
        locals.var_sid_dn0 = ((locals.var_whi_noise_dn0 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn0));
        locals.var_sid_dn2 = ((locals.var_whi_noise_dn2 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn2));
        locals.var_sid_dn4 = ((locals.var_whi_noise_dn4 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn4));
        locals.var_sid_dn5 = ((locals.var_whi_noise_dn5 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn5));
        locals.var_sid_dn6 = ((locals.var_whi_noise_dn6 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn6));
        locals.var_sid_dn7 = ((locals.var_whi_noise_dn7 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn7));
        locals.var_sid_dn8 = ((locals.var_whi_noise_dn8 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn8));
        locals.var_sid_dn9 = ((locals.var_whi_noise_dn9 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn9));
        locals.var_sid_dn10 = ((locals.var_whi_noise_dn10 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn10));
        locals.var_sid_dn11 = ((locals.var_whi_noise_dn11 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn11));
        locals.var_sid_dn14 = ((locals.var_whi_noise_dn14 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn14));

        locals.var_ci = locals.var_noicross;
        locals.var_ci_dn0 = locals.var_noicross_dn0;
        locals.var_ci_dn2 = locals.var_noicross_dn2;
        locals.var_ci_dn4 = locals.var_noicross_dn4;
        locals.var_ci_dn5 = locals.var_noicross_dn5;
        locals.var_ci_dn6 = locals.var_noicross_dn6;
        locals.var_ci_dn7 = locals.var_noicross_dn7;
        locals.var_ci_dn8 = locals.var_noicross_dn8;
        locals.var_ci_dn9 = locals.var_noicross_dn9;
        locals.var_ci_dn10 = locals.var_noicross_dn10;
        locals.var_ci_dn11 = locals.var_noicross_dn11;
        locals.var_ci_dn14 = locals.var_noicross_dn14;

        let (assign102570_e154415, assign102570_e154415_d_n0, assign102570_e154415_d_n2, assign102570_e154415_d_n4, assign102570_e154415_d_n5, assign102570_e154415_d_n6, assign102570_e154415_d_n7, assign102570_e154415_d_n8, assign102570_e154415_d_n9, assign102570_e154415_d_n10, assign102570_e154415_d_n11, assign102570_e154415_d_n14,) = {
    if ((locals.var_sid > 0.0) && (locals.var_noiigate > 0.0)) {
        let assign102570_e154412: f64 = (locals.var_noiigate / locals.var_sid);
        let assign102570_e154413: f64 = (assign102570_e154412).sqrt();
        (assign102570_e154413, ((((locals.var_noiigate_dn0 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn0)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102570_e154413)), ((((locals.var_noiigate_dn2 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn2)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102570_e154413)), ((((locals.var_noiigate_dn4 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn4)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102570_e154413)), ((((locals.var_noiigate_dn5 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn5)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102570_e154413)), ((((locals.var_noiigate_dn6 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn6)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102570_e154413)), ((((locals.var_noiigate_dn7 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn7)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102570_e154413)), ((((locals.var_noiigate_dn8 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn8)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102570_e154413)), ((((locals.var_noiigate_dn9 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn9)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102570_e154413)), ((((locals.var_noiigate_dn10 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn10)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102570_e154413)), ((((locals.var_noiigate_dn11 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn11)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102570_e154413)), ((((locals.var_noiigate_dn14 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn14)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102570_e154413)),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        locals.var_sigrat = assign102570_e154415;
        locals.var_sigrat_dn0 = assign102570_e154415_d_n0;
        locals.var_sigrat_dn2 = assign102570_e154415_d_n2;
        locals.var_sigrat_dn4 = assign102570_e154415_d_n4;
        locals.var_sigrat_dn5 = assign102570_e154415_d_n5;
        locals.var_sigrat_dn6 = assign102570_e154415_d_n6;
        locals.var_sigrat_dn7 = assign102570_e154415_d_n7;
        locals.var_sigrat_dn8 = assign102570_e154415_d_n8;
        locals.var_sigrat_dn9 = assign102570_e154415_d_n9;
        locals.var_sigrat_dn10 = assign102570_e154415_d_n10;
        locals.var_sigrat_dn11 = assign102570_e154415_d_n11;
        locals.var_sigrat_dn14 = assign102570_e154415_d_n14;

    }

    pub(super) fn stamp_transient_block_375(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let (assign102580_e154427, assign102580_e154427_d_n0, assign102580_e154427_d_n2, assign102580_e154427_d_n4, assign102580_e154427_d_n5, assign102580_e154427_d_n6, assign102580_e154427_d_n7, assign102580_e154427_d_n8, assign102580_e154427_d_n9, assign102580_e154427_d_n10, assign102580_e154427_d_n11, assign102580_e154427_d_n14,) = {
    if (locals.var_mode > 0.0) {
        let assign102580_e154422: f64 = (1.0 - locals.var_qdrat);
        let assign102580_e154423: f64 = (locals.var_sigrat * assign102580_e154422);
        (assign102580_e154423, ((locals.var_sigrat_dn0 * assign102580_e154422) + (locals.var_sigrat * (-locals.var_qdrat_dn0))), ((locals.var_sigrat_dn2 * assign102580_e154422) + (locals.var_sigrat * (-locals.var_qdrat_dn2))), ((locals.var_sigrat_dn4 * assign102580_e154422) + (locals.var_sigrat * (-locals.var_qdrat_dn4))), ((locals.var_sigrat_dn5 * assign102580_e154422) + (locals.var_sigrat * (-locals.var_qdrat_dn5))), ((locals.var_sigrat_dn6 * assign102580_e154422) + (locals.var_sigrat * (-locals.var_qdrat_dn6))), ((locals.var_sigrat_dn7 * assign102580_e154422) + (locals.var_sigrat * (-locals.var_qdrat_dn7))), ((locals.var_sigrat_dn8 * assign102580_e154422) + (locals.var_sigrat * (-locals.var_qdrat_dn8))), ((locals.var_sigrat_dn9 * assign102580_e154422) + (locals.var_sigrat * (-locals.var_qdrat_dn9))), ((locals.var_sigrat_dn10 * assign102580_e154422) + (locals.var_sigrat * (-locals.var_qdrat_dn10))), ((locals.var_sigrat_dn11 * assign102580_e154422) + (locals.var_sigrat * (-locals.var_qdrat_dn11))), ((locals.var_sigrat_dn14 * assign102580_e154422) + (locals.var_sigrat * (-locals.var_qdrat_dn14))),)
    } else {
        let assign102580_e154426: f64 = (locals.var_sigrat * locals.var_qdrat);
        (assign102580_e154426, ((locals.var_sigrat_dn0 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn0)), ((locals.var_sigrat_dn2 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn2)), ((locals.var_sigrat_dn4 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn4)), ((locals.var_sigrat_dn5 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn5)), ((locals.var_sigrat_dn6 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn6)), ((locals.var_sigrat_dn7 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn7)), ((locals.var_sigrat_dn8 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn8)), ((locals.var_sigrat_dn9 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn9)), ((locals.var_sigrat_dn10 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn10)), ((locals.var_sigrat_dn11 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn11)), ((locals.var_sigrat_dn14 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn14)),)
    }
};
        locals.var_sigrat_s = assign102580_e154427;
        locals.var_sigrat_s_dn0 = assign102580_e154427_d_n0;
        locals.var_sigrat_s_dn2 = assign102580_e154427_d_n2;
        locals.var_sigrat_s_dn4 = assign102580_e154427_d_n4;
        locals.var_sigrat_s_dn5 = assign102580_e154427_d_n5;
        locals.var_sigrat_s_dn6 = assign102580_e154427_d_n6;
        locals.var_sigrat_s_dn7 = assign102580_e154427_d_n7;
        locals.var_sigrat_s_dn8 = assign102580_e154427_d_n8;
        locals.var_sigrat_s_dn9 = assign102580_e154427_d_n9;
        locals.var_sigrat_s_dn10 = assign102580_e154427_d_n10;
        locals.var_sigrat_s_dn11 = assign102580_e154427_d_n11;
        locals.var_sigrat_s_dn14 = assign102580_e154427_d_n14;

        let (assign102590_e154439, assign102590_e154439_d_n0, assign102590_e154439_d_n2, assign102590_e154439_d_n4, assign102590_e154439_d_n5, assign102590_e154439_d_n6, assign102590_e154439_d_n7, assign102590_e154439_d_n8, assign102590_e154439_d_n9, assign102590_e154439_d_n10, assign102590_e154439_d_n11, assign102590_e154439_d_n14,) = {
    if (locals.var_mode > 0.0) {
        let assign102590_e154433: f64 = (locals.var_sigrat * locals.var_qdrat);
        (assign102590_e154433, ((locals.var_sigrat_dn0 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn0)), ((locals.var_sigrat_dn2 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn2)), ((locals.var_sigrat_dn4 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn4)), ((locals.var_sigrat_dn5 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn5)), ((locals.var_sigrat_dn6 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn6)), ((locals.var_sigrat_dn7 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn7)), ((locals.var_sigrat_dn8 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn8)), ((locals.var_sigrat_dn9 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn9)), ((locals.var_sigrat_dn10 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn10)), ((locals.var_sigrat_dn11 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn11)), ((locals.var_sigrat_dn14 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn14)),)
    } else {
        let assign102590_e154437: f64 = (1.0 - locals.var_qdrat);
        let assign102590_e154438: f64 = (locals.var_sigrat * assign102590_e154437);
        (assign102590_e154438, ((locals.var_sigrat_dn0 * assign102590_e154437) + (locals.var_sigrat * (-locals.var_qdrat_dn0))), ((locals.var_sigrat_dn2 * assign102590_e154437) + (locals.var_sigrat * (-locals.var_qdrat_dn2))), ((locals.var_sigrat_dn4 * assign102590_e154437) + (locals.var_sigrat * (-locals.var_qdrat_dn4))), ((locals.var_sigrat_dn5 * assign102590_e154437) + (locals.var_sigrat * (-locals.var_qdrat_dn5))), ((locals.var_sigrat_dn6 * assign102590_e154437) + (locals.var_sigrat * (-locals.var_qdrat_dn6))), ((locals.var_sigrat_dn7 * assign102590_e154437) + (locals.var_sigrat * (-locals.var_qdrat_dn7))), ((locals.var_sigrat_dn8 * assign102590_e154437) + (locals.var_sigrat * (-locals.var_qdrat_dn8))), ((locals.var_sigrat_dn9 * assign102590_e154437) + (locals.var_sigrat * (-locals.var_qdrat_dn9))), ((locals.var_sigrat_dn10 * assign102590_e154437) + (locals.var_sigrat * (-locals.var_qdrat_dn10))), ((locals.var_sigrat_dn11 * assign102590_e154437) + (locals.var_sigrat * (-locals.var_qdrat_dn11))), ((locals.var_sigrat_dn14 * assign102590_e154437) + (locals.var_sigrat * (-locals.var_qdrat_dn14))),)
    }
};
        locals.var_sigrat_d = assign102590_e154439;
        locals.var_sigrat_d_dn0 = assign102590_e154439_d_n0;
        locals.var_sigrat_d_dn2 = assign102590_e154439_d_n2;
        locals.var_sigrat_d_dn4 = assign102590_e154439_d_n4;
        locals.var_sigrat_d_dn5 = assign102590_e154439_d_n5;
        locals.var_sigrat_d_dn6 = assign102590_e154439_d_n6;
        locals.var_sigrat_d_dn7 = assign102590_e154439_d_n7;
        locals.var_sigrat_d_dn8 = assign102590_e154439_d_n8;
        locals.var_sigrat_d_dn9 = assign102590_e154439_d_n9;
        locals.var_sigrat_d_dn10 = assign102590_e154439_d_n10;
        locals.var_sigrat_d_dn11 = assign102590_e154439_d_n11;
        locals.var_sigrat_d_dn14 = assign102590_e154439_d_n14;

        locals.var_rsde = 0.0;
        locals.var_rsde_dn0 = 0.0;
        locals.var_rsde_dn2 = 0.0;
        locals.var_rsde_dn4 = 0.0;
        locals.var_rsde_dn5 = 0.0;
        locals.var_rsde_dn6 = 0.0;
        locals.var_rsde_dn7 = 0.0;
        locals.var_rsde_dn8 = 0.0;
        locals.var_rsde_dn9 = 0.0;
        locals.var_rsde_dn10 = 0.0;
        locals.var_rsde_dn11 = 0.0;
        locals.var_rsde_dn14 = 0.0;

        locals.var_rdde = 0.0;
        locals.var_rdde_dn0 = 0.0;
        locals.var_rdde_dn2 = 0.0;
        locals.var_rdde_dn4 = 0.0;
        locals.var_rdde_dn5 = 0.0;
        locals.var_rdde_dn6 = 0.0;
        locals.var_rdde_dn7 = 0.0;
        locals.var_rdde_dn8 = 0.0;
        locals.var_rdde_dn9 = 0.0;
        locals.var_rdde_dn10 = 0.0;
        locals.var_rdde_dn11 = 0.0;
        locals.var_rdde_dn14 = 0.0;

        let assign102620_e154444: f64 = if locals.var_uc_cordrift == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2338 = assign102620_e154444;

        let assign102630_e154447: f64 = if locals.var_flg_rs == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2339 = assign102630_e154447;

        let assign102640_e154454: f64 = if ((p.p53 > 0.0) && (locals.var_uc_rth0 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2340 = assign102640_e154454;

        let (assign102650_e154470, assign102650_e154470_d_n0, assign102650_e154470_d_n2, assign102650_e154470_d_n4, assign102650_e154470_d_n5, assign102650_e154470_d_n6, assign102650_e154470_d_n7, assign102650_e154470_d_n8, assign102650_e154470_d_n9, assign102650_e154470_d_n10, assign102650_e154470_d_n11, assign102650_e154470_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2340 != 0.0)) {
        let (assign102650_e154468, assign102650_e154468_d_n0, assign102650_e154468_d_n2, assign102650_e154468_d_n4, assign102650_e154468_d_n5, assign102650_e154468_d_n6, assign102650_e154468_d_n7, assign102650_e154468_d_n8, assign102650_e154468_d_n9, assign102650_e154468_d_n10, assign102650_e154468_d_n11, assign102650_e154468_d_n14,) = {
            if (locals.var_tratio == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign102650_e154467: f64 = (locals.var_tratio).powf(p.p416);
                (assign102650_e154467, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn0)) } } else { (assign102650_e154467 * (p.p416 * (locals.var_tratio_dn0 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn2)) } } else { (assign102650_e154467 * (p.p416 * (locals.var_tratio_dn2 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn4)) } } else { (assign102650_e154467 * (p.p416 * (locals.var_tratio_dn4 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn5)) } } else { (assign102650_e154467 * (p.p416 * (locals.var_tratio_dn5 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn6)) } } else { (assign102650_e154467 * (p.p416 * (locals.var_tratio_dn6 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn7)) } } else { (assign102650_e154467 * (p.p416 * (locals.var_tratio_dn7 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn8)) } } else { (assign102650_e154467 * (p.p416 * (locals.var_tratio_dn8 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn9)) } } else { (assign102650_e154467 * (p.p416 * (locals.var_tratio_dn9 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn10)) } } else { (assign102650_e154467 * (p.p416 * (locals.var_tratio_dn10 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn11)) } } else { (assign102650_e154467 * (p.p416 * (locals.var_tratio_dn11 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn14)) } } else { (assign102650_e154467 * (p.p416 * (locals.var_tratio_dn14 / locals.var_tratio))) },)
            }
        };
        (assign102650_e154468, assign102650_e154468_d_n0, assign102650_e154468_d_n2, assign102650_e154468_d_n4, assign102650_e154468_d_n5, assign102650_e154468_d_n6, assign102650_e154468_d_n7, assign102650_e154468_d_n8, assign102650_e154468_d_n9, assign102650_e154468_d_n10, assign102650_e154468_d_n11, assign102650_e154468_d_n14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign102650_e154470;
        locals.var_t1_dn0 = assign102650_e154470_d_n0;
        locals.var_t1_dn2 = assign102650_e154470_d_n2;
        locals.var_t1_dn4 = assign102650_e154470_d_n4;
        locals.var_t1_dn5 = assign102650_e154470_d_n5;
        locals.var_t1_dn6 = assign102650_e154470_d_n6;
        locals.var_t1_dn7 = assign102650_e154470_d_n7;
        locals.var_t1_dn8 = assign102650_e154470_d_n8;
        locals.var_t1_dn9 = assign102650_e154470_d_n9;
        locals.var_t1_dn10 = assign102650_e154470_d_n10;
        locals.var_t1_dn11 = assign102650_e154470_d_n11;
        locals.var_t1_dn14 = assign102650_e154470_d_n14;

        let (assign102660_e154481, assign102660_e154481_d_n0, assign102660_e154481_d_n2, assign102660_e154481_d_n4, assign102660_e154481_d_n5, assign102660_e154481_d_n6, assign102660_e154481_d_n7, assign102660_e154481_d_n8, assign102660_e154481_d_n9, assign102660_e154481_d_n10, assign102660_e154481_d_n11, assign102660_e154481_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2340 != 0.0)) {
        let assign102660_e154479: f64 = (locals.var_mks_rdrmues / locals.var_t1);
        (assign102660_e154479, (-((locals.var_mks_rdrmues * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn14) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_rrdrmues, locals.var_rrdrmues_dn0, locals.var_rrdrmues_dn2, locals.var_rrdrmues_dn4, locals.var_rrdrmues_dn5, locals.var_rrdrmues_dn6, locals.var_rrdrmues_dn7, locals.var_rrdrmues_dn8, locals.var_rrdrmues_dn9, locals.var_rrdrmues_dn10, locals.var_rrdrmues_dn11, locals.var_rrdrmues_dn14,)
    }
};
        locals.var_rrdrmues = assign102660_e154481;
        locals.var_rrdrmues_dn0 = assign102660_e154481_d_n0;
        locals.var_rrdrmues_dn2 = assign102660_e154481_d_n2;
        locals.var_rrdrmues_dn4 = assign102660_e154481_d_n4;
        locals.var_rrdrmues_dn5 = assign102660_e154481_d_n5;
        locals.var_rrdrmues_dn6 = assign102660_e154481_d_n6;
        locals.var_rrdrmues_dn7 = assign102660_e154481_d_n7;
        locals.var_rrdrmues_dn8 = assign102660_e154481_d_n8;
        locals.var_rrdrmues_dn9 = assign102660_e154481_d_n9;
        locals.var_rrdrmues_dn10 = assign102660_e154481_d_n10;
        locals.var_rrdrmues_dn11 = assign102660_e154481_d_n11;
        locals.var_rrdrmues_dn14 = assign102660_e154481_d_n14;

        let (assign102670_e154506, assign102670_e154506_d_n0, assign102670_e154506_d_n2, assign102670_e154506_d_n4, assign102670_e154506_d_n5, assign102670_e154506_d_n6, assign102670_e154506_d_n7, assign102670_e154506_d_n8, assign102670_e154506_d_n9, assign102670_e154506_d_n10, assign102670_e154506_d_n11, assign102670_e154506_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2340 != 0.0)) {
        let assign102670_e154491: f64 = (0.4 * locals.var_tratio);
        let assign102670_e154492: f64 = (1.8 + assign102670_e154491);
        let assign102670_e154495: f64 = (0.1 * locals.var_tratio);
        let assign102670_e154497: f64 = (assign102670_e154495 * locals.var_tratio);
        let assign102670_e154498: f64 = (assign102670_e154492 + assign102670_e154497);
        let assign102670_e154502: f64 = (1.0 - locals.var_tratio);
        let assign102670_e154503: f64 = (p.p418 * assign102670_e154502);
        let assign102670_e154504: f64 = (assign102670_e154498 - assign102670_e154503);
        (assign102670_e154504, (((0.4 * locals.var_tratio_dn0) + (((0.1 * locals.var_tratio_dn0) * locals.var_tratio) + (assign102670_e154495 * locals.var_tratio_dn0))) - (p.p418 * (-locals.var_tratio_dn0))), (((0.4 * locals.var_tratio_dn2) + (((0.1 * locals.var_tratio_dn2) * locals.var_tratio) + (assign102670_e154495 * locals.var_tratio_dn2))) - (p.p418 * (-locals.var_tratio_dn2))), (((0.4 * locals.var_tratio_dn4) + (((0.1 * locals.var_tratio_dn4) * locals.var_tratio) + (assign102670_e154495 * locals.var_tratio_dn4))) - (p.p418 * (-locals.var_tratio_dn4))), (((0.4 * locals.var_tratio_dn5) + (((0.1 * locals.var_tratio_dn5) * locals.var_tratio) + (assign102670_e154495 * locals.var_tratio_dn5))) - (p.p418 * (-locals.var_tratio_dn5))), (((0.4 * locals.var_tratio_dn6) + (((0.1 * locals.var_tratio_dn6) * locals.var_tratio) + (assign102670_e154495 * locals.var_tratio_dn6))) - (p.p418 * (-locals.var_tratio_dn6))), (((0.4 * locals.var_tratio_dn7) + (((0.1 * locals.var_tratio_dn7) * locals.var_tratio) + (assign102670_e154495 * locals.var_tratio_dn7))) - (p.p418 * (-locals.var_tratio_dn7))), (((0.4 * locals.var_tratio_dn8) + (((0.1 * locals.var_tratio_dn8) * locals.var_tratio) + (assign102670_e154495 * locals.var_tratio_dn8))) - (p.p418 * (-locals.var_tratio_dn8))), (((0.4 * locals.var_tratio_dn9) + (((0.1 * locals.var_tratio_dn9) * locals.var_tratio) + (assign102670_e154495 * locals.var_tratio_dn9))) - (p.p418 * (-locals.var_tratio_dn9))), (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign102670_e154495 * locals.var_tratio_dn10))) - (p.p418 * (-locals.var_tratio_dn10))), (((0.4 * locals.var_tratio_dn11) + (((0.1 * locals.var_tratio_dn11) * locals.var_tratio) + (assign102670_e154495 * locals.var_tratio_dn11))) - (p.p418 * (-locals.var_tratio_dn11))), (((0.4 * locals.var_tratio_dn14) + (((0.1 * locals.var_tratio_dn14) * locals.var_tratio) + (assign102670_e154495 * locals.var_tratio_dn14))) - (p.p418 * (-locals.var_tratio_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign102670_e154506;
        locals.var_t0_dn0 = assign102670_e154506_d_n0;
        locals.var_t0_dn2 = assign102670_e154506_d_n2;
        locals.var_t0_dn4 = assign102670_e154506_d_n4;
        locals.var_t0_dn5 = assign102670_e154506_d_n5;
        locals.var_t0_dn6 = assign102670_e154506_d_n6;
        locals.var_t0_dn7 = assign102670_e154506_d_n7;
        locals.var_t0_dn8 = assign102670_e154506_d_n8;
        locals.var_t0_dn9 = assign102670_e154506_d_n9;
        locals.var_t0_dn10 = assign102670_e154506_d_n10;
        locals.var_t0_dn11 = assign102670_e154506_d_n11;
        locals.var_t0_dn14 = assign102670_e154506_d_n14;

        let (assign102680_e154517, assign102680_e154517_d_n0, assign102680_e154517_d_n2, assign102680_e154517_d_n4, assign102680_e154517_d_n5, assign102680_e154517_d_n6, assign102680_e154517_d_n7, assign102680_e154517_d_n8, assign102680_e154517_d_n9, assign102680_e154517_d_n10, assign102680_e154517_d_n11, assign102680_e154517_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2340 != 0.0)) {
        let assign102680_e154515: f64 = (locals.var_mks_rdrvmaxs / locals.var_t0);
        (assign102680_e154515, (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_rrdrvmaxs, locals.var_rrdrvmaxs_dn0, locals.var_rrdrvmaxs_dn2, locals.var_rrdrvmaxs_dn4, locals.var_rrdrvmaxs_dn5, locals.var_rrdrvmaxs_dn6, locals.var_rrdrvmaxs_dn7, locals.var_rrdrvmaxs_dn8, locals.var_rrdrvmaxs_dn9, locals.var_rrdrvmaxs_dn10, locals.var_rrdrvmaxs_dn11, locals.var_rrdrvmaxs_dn14,)
    }
};
        locals.var_rrdrvmaxs = assign102680_e154517;
        locals.var_rrdrvmaxs_dn0 = assign102680_e154517_d_n0;
        locals.var_rrdrvmaxs_dn2 = assign102680_e154517_d_n2;
        locals.var_rrdrvmaxs_dn4 = assign102680_e154517_d_n4;
        locals.var_rrdrvmaxs_dn5 = assign102680_e154517_d_n5;
        locals.var_rrdrvmaxs_dn6 = assign102680_e154517_d_n6;
        locals.var_rrdrvmaxs_dn7 = assign102680_e154517_d_n7;
        locals.var_rrdrvmaxs_dn8 = assign102680_e154517_d_n8;
        locals.var_rrdrvmaxs_dn9 = assign102680_e154517_d_n9;
        locals.var_rrdrvmaxs_dn10 = assign102680_e154517_d_n10;
        locals.var_rrdrvmaxs_dn11 = assign102680_e154517_d_n11;
        locals.var_rrdrvmaxs_dn14 = assign102680_e154517_d_n14;

        let (assign102690_e154532, assign102690_e154532_d_n0, assign102690_e154532_d_n2, assign102690_e154532_d_n4, assign102690_e154532_d_n5, assign102690_e154532_d_n6, assign102690_e154532_d_n7, assign102690_e154532_d_n8, assign102690_e154532_d_n9, assign102690_e154532_d_n10, assign102690_e154532_d_n11, assign102690_e154532_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2340 != 0.0)) {
        let assign102690_e154528: f64 = (locals.var_ttemp - locals.var_ktnom);
        let assign102690_e154529: f64 = (p.p439 * assign102690_e154528);
        let assign102690_e154530: f64 = (locals.var_uc_rdrbb_s + assign102690_e154529);
        (assign102690_e154530, (locals.var_uc_rdrbb_s_dn0 + (p.p439 * locals.var_ttemp_dn0)), (locals.var_uc_rdrbb_s_dn2 + (p.p439 * locals.var_ttemp_dn2)), (locals.var_uc_rdrbb_s_dn4 + (p.p439 * locals.var_ttemp_dn4)), (locals.var_uc_rdrbb_s_dn5 + (p.p439 * locals.var_ttemp_dn5)), (locals.var_uc_rdrbb_s_dn6 + (p.p439 * locals.var_ttemp_dn6)), (locals.var_uc_rdrbb_s_dn7 + (p.p439 * locals.var_ttemp_dn7)), (locals.var_uc_rdrbb_s_dn8 + (p.p439 * locals.var_ttemp_dn8)), (locals.var_uc_rdrbb_s_dn9 + (p.p439 * locals.var_ttemp_dn9)), (locals.var_uc_rdrbb_s_dn10 + (p.p439 * locals.var_ttemp_dn10)), (locals.var_uc_rdrbb_s_dn11 + (p.p439 * locals.var_ttemp_dn11)), (locals.var_uc_rdrbb_s_dn14 + (p.p439 * locals.var_ttemp_dn14)),)
    } else {
        (locals.var_uc_rdrbb_s, locals.var_uc_rdrbb_s_dn0, locals.var_uc_rdrbb_s_dn2, locals.var_uc_rdrbb_s_dn4, locals.var_uc_rdrbb_s_dn5, locals.var_uc_rdrbb_s_dn6, locals.var_uc_rdrbb_s_dn7, locals.var_uc_rdrbb_s_dn8, locals.var_uc_rdrbb_s_dn9, locals.var_uc_rdrbb_s_dn10, locals.var_uc_rdrbb_s_dn11, locals.var_uc_rdrbb_s_dn14,)
    }
};
        locals.var_uc_rdrbb_s = assign102690_e154532;
        locals.var_uc_rdrbb_s_dn0 = assign102690_e154532_d_n0;
        locals.var_uc_rdrbb_s_dn2 = assign102690_e154532_d_n2;
        locals.var_uc_rdrbb_s_dn4 = assign102690_e154532_d_n4;
        locals.var_uc_rdrbb_s_dn5 = assign102690_e154532_d_n5;
        locals.var_uc_rdrbb_s_dn6 = assign102690_e154532_d_n6;
        locals.var_uc_rdrbb_s_dn7 = assign102690_e154532_d_n7;
        locals.var_uc_rdrbb_s_dn8 = assign102690_e154532_d_n8;
        locals.var_uc_rdrbb_s_dn9 = assign102690_e154532_d_n9;
        locals.var_uc_rdrbb_s_dn10 = assign102690_e154532_d_n10;
        locals.var_uc_rdrbb_s_dn11 = assign102690_e154532_d_n11;
        locals.var_uc_rdrbb_s_dn14 = assign102690_e154532_d_n14;

        let (assign102700_e154544, assign102700_e154544_d_n0, assign102700_e154544_d_n2, assign102700_e154544_d_n4, assign102700_e154544_d_n5, assign102700_e154544_d_n6, assign102700_e154544_d_n7, assign102700_e154544_d_n8, assign102700_e154544_d_n9, assign102700_e154544_d_n10, assign102700_e154544_d_n11, assign102700_e154544_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2340 == 0.0)) {
        let assign102700_e154540: f64 = ctx_temp;
        let assign102700_e154542: f64 = (assign102700_e154540 + p.p11);
        (assign102700_e154542, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn11, locals.var_ttemp_dn14,)
    }
};
        locals.var_ttemp = assign102700_e154544;
        locals.var_ttemp_dn0 = assign102700_e154544_d_n0;
        locals.var_ttemp_dn2 = assign102700_e154544_d_n2;
        locals.var_ttemp_dn4 = assign102700_e154544_d_n4;
        locals.var_ttemp_dn5 = assign102700_e154544_d_n5;
        locals.var_ttemp_dn6 = assign102700_e154544_d_n6;
        locals.var_ttemp_dn7 = assign102700_e154544_d_n7;
        locals.var_ttemp_dn8 = assign102700_e154544_d_n8;
        locals.var_ttemp_dn9 = assign102700_e154544_d_n9;
        locals.var_ttemp_dn10 = assign102700_e154544_d_n10;
        locals.var_ttemp_dn11 = assign102700_e154544_d_n11;
        locals.var_ttemp_dn14 = assign102700_e154544_d_n14;

        let (assign102710_e154553,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) {
        let assign102710_e154551: f64 = (locals.var_weff_ld * p.p7);
        (assign102710_e154551,)
    } else {
        (locals.var_weffld_nf,)
    }
};
        locals.var_weffld_nf = assign102710_e154553;

        let (assign102720_e154560,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) {
        (p.p71,)
    } else {
        (locals.var_ldrifte_s,)
    }
};
        locals.var_ldrifte_s = assign102720_e154560;

        let (assign102730_e154567,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) {
        (locals.var_uc_novers,)
    } else {
        (locals.var_novers,)
    }
};
        locals.var_novers = assign102730_e154567;

        let (assign102740_e154576, assign102740_e154576_d_n0, assign102740_e154576_d_n2, assign102740_e154576_d_n4, assign102740_e154576_d_n5, assign102740_e154576_d_n6, assign102740_e154576_d_n7, assign102740_e154576_d_n8, assign102740_e154576_d_n9, assign102740_e154576_d_n10, assign102740_e154576_d_n11, assign102740_e154576_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) {
        let assign102740_e154574: f64 = (locals.var_rrdrmues * locals.var_rdrmuele);
        (assign102740_e154574, (locals.var_rrdrmues_dn0 * locals.var_rdrmuele), (locals.var_rrdrmues_dn2 * locals.var_rdrmuele), (locals.var_rrdrmues_dn4 * locals.var_rdrmuele), (locals.var_rrdrmues_dn5 * locals.var_rdrmuele), (locals.var_rrdrmues_dn6 * locals.var_rdrmuele), (locals.var_rrdrmues_dn7 * locals.var_rdrmuele), (locals.var_rrdrmues_dn8 * locals.var_rdrmuele), (locals.var_rrdrmues_dn9 * locals.var_rdrmuele), (locals.var_rrdrmues_dn10 * locals.var_rdrmuele), (locals.var_rrdrmues_dn11 * locals.var_rdrmuele), (locals.var_rrdrmues_dn14 * locals.var_rdrmuele),)
    } else {
        (locals.var_mu0_s, locals.var_mu0_s_dn0, locals.var_mu0_s_dn2, locals.var_mu0_s_dn4, locals.var_mu0_s_dn5, locals.var_mu0_s_dn6, locals.var_mu0_s_dn7, locals.var_mu0_s_dn8, locals.var_mu0_s_dn9, locals.var_mu0_s_dn10, locals.var_mu0_s_dn11, locals.var_mu0_s_dn14,)
    }
};
        locals.var_mu0_s = assign102740_e154576;
        locals.var_mu0_s_dn0 = assign102740_e154576_d_n0;
        locals.var_mu0_s_dn2 = assign102740_e154576_d_n2;
        locals.var_mu0_s_dn4 = assign102740_e154576_d_n4;
        locals.var_mu0_s_dn5 = assign102740_e154576_d_n5;
        locals.var_mu0_s_dn6 = assign102740_e154576_d_n6;
        locals.var_mu0_s_dn7 = assign102740_e154576_d_n7;
        locals.var_mu0_s_dn8 = assign102740_e154576_d_n8;
        locals.var_mu0_s_dn9 = assign102740_e154576_d_n9;
        locals.var_mu0_s_dn10 = assign102740_e154576_d_n10;
        locals.var_mu0_s_dn11 = assign102740_e154576_d_n11;
        locals.var_mu0_s_dn14 = assign102740_e154576_d_n14;

        let (assign102750_e154589, assign102750_e154589_d_n0, assign102750_e154589_d_n2, assign102750_e154589_d_n4, assign102750_e154589_d_n5, assign102750_e154589_d_n6, assign102750_e154589_d_n7, assign102750_e154589_d_n8, assign102750_e154589_d_n9, assign102750_e154589_d_n10, assign102750_e154589_d_n11, assign102750_e154589_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) {
        let assign102750_e154583: f64 = (locals.var_rrdrvmaxs * locals.var_rdrvmaxwe);
        let assign102750_e154585: f64 = (assign102750_e154583 * locals.var_rdrvmaxle);
        let assign102750_e154587: f64 = (assign102750_e154585 + 1e-25);
        (assign102750_e154587, ((locals.var_rrdrvmaxs_dn0 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn2 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn4 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn5 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn6 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn7 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn8 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn9 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn10 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn11 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn14 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle),)
    } else {
        (locals.var_vmaxe_s, locals.var_vmaxe_s_dn0, locals.var_vmaxe_s_dn2, locals.var_vmaxe_s_dn4, locals.var_vmaxe_s_dn5, locals.var_vmaxe_s_dn6, locals.var_vmaxe_s_dn7, locals.var_vmaxe_s_dn8, locals.var_vmaxe_s_dn9, locals.var_vmaxe_s_dn10, locals.var_vmaxe_s_dn11, locals.var_vmaxe_s_dn14,)
    }
};
        locals.var_vmaxe_s = assign102750_e154589;
        locals.var_vmaxe_s_dn0 = assign102750_e154589_d_n0;
        locals.var_vmaxe_s_dn2 = assign102750_e154589_d_n2;
        locals.var_vmaxe_s_dn4 = assign102750_e154589_d_n4;
        locals.var_vmaxe_s_dn5 = assign102750_e154589_d_n5;
        locals.var_vmaxe_s_dn6 = assign102750_e154589_d_n6;
        locals.var_vmaxe_s_dn7 = assign102750_e154589_d_n7;
        locals.var_vmaxe_s_dn8 = assign102750_e154589_d_n8;
        locals.var_vmaxe_s_dn9 = assign102750_e154589_d_n9;
        locals.var_vmaxe_s_dn10 = assign102750_e154589_d_n10;
        locals.var_vmaxe_s_dn11 = assign102750_e154589_d_n11;
        locals.var_vmaxe_s_dn14 = assign102750_e154589_d_n14;

        let (assign102760_e154598, assign102760_e154598_d_n2, assign102760_e154598_d_n8,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) {
        let assign102760_e154596: f64 = (locals.var_vsps / locals.var_ldrifte_s);
        (assign102760_e154596, (locals.var_vsps_dn2 / locals.var_ldrifte_s), (locals.var_vsps_dn8 / locals.var_ldrifte_s),)
    } else {
        (locals.var_edri_s, locals.var_edri_s_dn2, locals.var_edri_s_dn8,)
    }
};
        locals.var_edri_s = assign102760_e154598;
        locals.var_edri_s_dn2 = assign102760_e154598_d_n2;
        locals.var_edri_s_dn8 = assign102760_e154598_d_n8;

        let (assign102770_e154607, assign102770_e154607_d_n0, assign102770_e154607_d_n2, assign102770_e154607_d_n4, assign102770_e154607_d_n5, assign102770_e154607_d_n6, assign102770_e154607_d_n7, assign102770_e154607_d_n8, assign102770_e154607_d_n9, assign102770_e154607_d_n10, assign102770_e154607_d_n11, assign102770_e154607_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) {
        let assign102770_e154605: f64 = (locals.var_mu0_s * locals.var_edri_s);
        (assign102770_e154605, (locals.var_mu0_s_dn0 * locals.var_edri_s), ((locals.var_mu0_s_dn2 * locals.var_edri_s) + (locals.var_mu0_s * locals.var_edri_s_dn2)), (locals.var_mu0_s_dn4 * locals.var_edri_s), (locals.var_mu0_s_dn5 * locals.var_edri_s), (locals.var_mu0_s_dn6 * locals.var_edri_s), (locals.var_mu0_s_dn7 * locals.var_edri_s), ((locals.var_mu0_s_dn8 * locals.var_edri_s) + (locals.var_mu0_s * locals.var_edri_s_dn8)), (locals.var_mu0_s_dn9 * locals.var_edri_s), (locals.var_mu0_s_dn10 * locals.var_edri_s), (locals.var_mu0_s_dn11 * locals.var_edri_s), (locals.var_mu0_s_dn14 * locals.var_edri_s),)
    } else {
        (locals.var_vdri_s, locals.var_vdri_s_dn0, locals.var_vdri_s_dn2, locals.var_vdri_s_dn4, locals.var_vdri_s_dn5, locals.var_vdri_s_dn6, locals.var_vdri_s_dn7, locals.var_vdri_s_dn8, locals.var_vdri_s_dn9, locals.var_vdri_s_dn10, locals.var_vdri_s_dn11, locals.var_vdri_s_dn14,)
    }
};
        locals.var_vdri_s = assign102770_e154607;
        locals.var_vdri_s_dn0 = assign102770_e154607_d_n0;
        locals.var_vdri_s_dn2 = assign102770_e154607_d_n2;
        locals.var_vdri_s_dn4 = assign102770_e154607_d_n4;
        locals.var_vdri_s_dn5 = assign102770_e154607_d_n5;
        locals.var_vdri_s_dn6 = assign102770_e154607_d_n6;
        locals.var_vdri_s_dn7 = assign102770_e154607_d_n7;
        locals.var_vdri_s_dn8 = assign102770_e154607_d_n8;
        locals.var_vdri_s_dn9 = assign102770_e154607_d_n9;
        locals.var_vdri_s_dn10 = assign102770_e154607_d_n10;
        locals.var_vdri_s_dn11 = assign102770_e154607_d_n11;
        locals.var_vdri_s_dn14 = assign102770_e154607_d_n14;

        let assign102780_e154610: f64 = if locals.var_vsps >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2341 = assign102780_e154610;

        let (assign102790_e154621, assign102790_e154621_d_n0, assign102790_e154621_d_n2, assign102790_e154621_d_n4, assign102790_e154621_d_n5, assign102790_e154621_d_n6, assign102790_e154621_d_n7, assign102790_e154621_d_n8, assign102790_e154621_d_n9, assign102790_e154621_d_n10, assign102790_e154621_d_n11, assign102790_e154621_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2341 != 0.0)) {
        let assign102790_e154619: f64 = (locals.var_vdri_s / locals.var_vmaxe_s);
        (assign102790_e154619, (((locals.var_vdri_s_dn0 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn0)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn2 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn2)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn4 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn4)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn5 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn5)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn6 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn6)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn7 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn7)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn8 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn8)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn9 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn9)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn10 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn10)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn11 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn11)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn14 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn14)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign102790_e154621;
        locals.var_t1_dn0 = assign102790_e154621_d_n0;
        locals.var_t1_dn2 = assign102790_e154621_d_n2;
        locals.var_t1_dn4 = assign102790_e154621_d_n4;
        locals.var_t1_dn5 = assign102790_e154621_d_n5;
        locals.var_t1_dn6 = assign102790_e154621_d_n6;
        locals.var_t1_dn7 = assign102790_e154621_d_n7;
        locals.var_t1_dn8 = assign102790_e154621_d_n8;
        locals.var_t1_dn9 = assign102790_e154621_d_n9;
        locals.var_t1_dn10 = assign102790_e154621_d_n10;
        locals.var_t1_dn11 = assign102790_e154621_d_n11;
        locals.var_t1_dn14 = assign102790_e154621_d_n14;

        let (assign102800_e154634, assign102800_e154634_d_n0, assign102800_e154634_d_n2, assign102800_e154634_d_n4, assign102800_e154634_d_n5, assign102800_e154634_d_n6, assign102800_e154634_d_n7, assign102800_e154634_d_n8, assign102800_e154634_d_n9, assign102800_e154634_d_n10, assign102800_e154634_d_n11, assign102800_e154634_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2341 == 0.0)) {
        let assign102800_e154630: f64 = (-locals.var_vdri_s);
        let assign102800_e154632: f64 = (assign102800_e154630 / locals.var_vmaxe_s);
        (assign102800_e154632, ((((-locals.var_vdri_s_dn0) * locals.var_vmaxe_s) - (assign102800_e154630 * locals.var_vmaxe_s_dn0)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn2) * locals.var_vmaxe_s) - (assign102800_e154630 * locals.var_vmaxe_s_dn2)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn4) * locals.var_vmaxe_s) - (assign102800_e154630 * locals.var_vmaxe_s_dn4)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn5) * locals.var_vmaxe_s) - (assign102800_e154630 * locals.var_vmaxe_s_dn5)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn6) * locals.var_vmaxe_s) - (assign102800_e154630 * locals.var_vmaxe_s_dn6)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn7) * locals.var_vmaxe_s) - (assign102800_e154630 * locals.var_vmaxe_s_dn7)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn8) * locals.var_vmaxe_s) - (assign102800_e154630 * locals.var_vmaxe_s_dn8)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn9) * locals.var_vmaxe_s) - (assign102800_e154630 * locals.var_vmaxe_s_dn9)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn10) * locals.var_vmaxe_s) - (assign102800_e154630 * locals.var_vmaxe_s_dn10)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn11) * locals.var_vmaxe_s) - (assign102800_e154630 * locals.var_vmaxe_s_dn11)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn14) * locals.var_vmaxe_s) - (assign102800_e154630 * locals.var_vmaxe_s_dn14)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign102800_e154634;
        locals.var_t1_dn0 = assign102800_e154634_d_n0;
        locals.var_t1_dn2 = assign102800_e154634_d_n2;
        locals.var_t1_dn4 = assign102800_e154634_d_n4;
        locals.var_t1_dn5 = assign102800_e154634_d_n5;
        locals.var_t1_dn6 = assign102800_e154634_d_n6;
        locals.var_t1_dn7 = assign102800_e154634_d_n7;
        locals.var_t1_dn8 = assign102800_e154634_d_n8;
        locals.var_t1_dn9 = assign102800_e154634_d_n9;
        locals.var_t1_dn10 = assign102800_e154634_d_n10;
        locals.var_t1_dn11 = assign102800_e154634_d_n11;
        locals.var_t1_dn14 = assign102800_e154634_d_n14;

        let assign102810_e154638: f64 = (10.0 * 2.220446049250313e-16);
        let assign102810_e154639: f64 = (1.0 - assign102810_e154638);
        let assign102810_e154646: f64 = (10.0 * 2.220446049250313e-16);
        let assign102810_e154647: f64 = (1.0 + assign102810_e154646);
        let assign102810_e154649: f64 = if ((assign102810_e154639 <= locals.var_uc_rdrbb_s) && (locals.var_uc_rdrbb_s <= assign102810_e154647)) { 1.0 } else { 0.0 };
        locals.var_guard2342 = assign102810_e154649;

        let (assign102820_e154658, assign102820_e154658_d_n0, assign102820_e154658_d_n2, assign102820_e154658_d_n4, assign102820_e154658_d_n5, assign102820_e154658_d_n6, assign102820_e154658_d_n7, assign102820_e154658_d_n8, assign102820_e154658_d_n9, assign102820_e154658_d_n10, assign102820_e154658_d_n11, assign102820_e154658_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2342 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign102820_e154658;
        locals.var_t3_dn0 = assign102820_e154658_d_n0;
        locals.var_t3_dn2 = assign102820_e154658_d_n2;
        locals.var_t3_dn4 = assign102820_e154658_d_n4;
        locals.var_t3_dn5 = assign102820_e154658_d_n5;
        locals.var_t3_dn6 = assign102820_e154658_d_n6;
        locals.var_t3_dn7 = assign102820_e154658_d_n7;
        locals.var_t3_dn8 = assign102820_e154658_d_n8;
        locals.var_t3_dn9 = assign102820_e154658_d_n9;
        locals.var_t3_dn10 = assign102820_e154658_d_n10;
        locals.var_t3_dn11 = assign102820_e154658_d_n11;
        locals.var_t3_dn14 = assign102820_e154658_d_n14;

        let assign102830_e154662: f64 = (10.0 * 2.220446049250313e-16);
        let assign102830_e154663: f64 = (2.0 - assign102830_e154662);
        let assign102830_e154670: f64 = (10.0 * 2.220446049250313e-16);
        let assign102830_e154671: f64 = (2.0 + assign102830_e154670);
        let assign102830_e154673: f64 = if ((assign102830_e154663 <= locals.var_uc_rdrbb_s) && (locals.var_uc_rdrbb_s <= assign102830_e154671)) { 1.0 } else { 0.0 };
        locals.var_guard2343 = assign102830_e154673;

        let (assign102840_e154685, assign102840_e154685_d_n0, assign102840_e154685_d_n2, assign102840_e154685_d_n4, assign102840_e154685_d_n5, assign102840_e154685_d_n6, assign102840_e154685_d_n7, assign102840_e154685_d_n8, assign102840_e154685_d_n9, assign102840_e154685_d_n10, assign102840_e154685_d_n11, assign102840_e154685_d_n14,) = {
    if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2342 == 0.0)) && (locals.var_guard2343 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign102840_e154685;
        locals.var_t3_dn0 = assign102840_e154685_d_n0;
        locals.var_t3_dn2 = assign102840_e154685_d_n2;
        locals.var_t3_dn4 = assign102840_e154685_d_n4;
        locals.var_t3_dn5 = assign102840_e154685_d_n5;
        locals.var_t3_dn6 = assign102840_e154685_d_n6;
        locals.var_t3_dn7 = assign102840_e154685_d_n7;
        locals.var_t3_dn8 = assign102840_e154685_d_n8;
        locals.var_t3_dn9 = assign102840_e154685_d_n9;
        locals.var_t3_dn10 = assign102840_e154685_d_n10;
        locals.var_t3_dn11 = assign102840_e154685_d_n11;
        locals.var_t3_dn14 = assign102840_e154685_d_n14;

        let (assign102850_e154702, assign102850_e154702_d_n0, assign102850_e154702_d_n2, assign102850_e154702_d_n4, assign102850_e154702_d_n5, assign102850_e154702_d_n6, assign102850_e154702_d_n7, assign102850_e154702_d_n8, assign102850_e154702_d_n9, assign102850_e154702_d_n10, assign102850_e154702_d_n11, assign102850_e154702_d_n14,) = {
    if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2342 == 0.0)) && (locals.var_guard2343 == 0.0)) {
        let assign102850_e154699: f64 = (locals.var_uc_rdrbb_s - 1.0);
        let assign102850_e154700: f64 = (locals.var_t1).powf(assign102850_e154699);
        (assign102850_e154700, if locals.var_uc_rdrbb_s_dn0 == 0.0 && ((assign102850_e154699) as f64).is_finite() && ((assign102850_e154699) as f64).fract() == 0.0 { if assign102850_e154699 == 0.0 { 0.0 } else { (assign102850_e154699 * ((locals.var_t1).powf(assign102850_e154699 - 1.0) * locals.var_t1_dn0)) } } else { (assign102850_e154700 * ((locals.var_uc_rdrbb_s_dn0 * (locals.var_t1).ln()) + (assign102850_e154699 * (locals.var_t1_dn0 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn2 == 0.0 && ((assign102850_e154699) as f64).is_finite() && ((assign102850_e154699) as f64).fract() == 0.0 { if assign102850_e154699 == 0.0 { 0.0 } else { (assign102850_e154699 * ((locals.var_t1).powf(assign102850_e154699 - 1.0) * locals.var_t1_dn2)) } } else { (assign102850_e154700 * ((locals.var_uc_rdrbb_s_dn2 * (locals.var_t1).ln()) + (assign102850_e154699 * (locals.var_t1_dn2 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn4 == 0.0 && ((assign102850_e154699) as f64).is_finite() && ((assign102850_e154699) as f64).fract() == 0.0 { if assign102850_e154699 == 0.0 { 0.0 } else { (assign102850_e154699 * ((locals.var_t1).powf(assign102850_e154699 - 1.0) * locals.var_t1_dn4)) } } else { (assign102850_e154700 * ((locals.var_uc_rdrbb_s_dn4 * (locals.var_t1).ln()) + (assign102850_e154699 * (locals.var_t1_dn4 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn5 == 0.0 && ((assign102850_e154699) as f64).is_finite() && ((assign102850_e154699) as f64).fract() == 0.0 { if assign102850_e154699 == 0.0 { 0.0 } else { (assign102850_e154699 * ((locals.var_t1).powf(assign102850_e154699 - 1.0) * locals.var_t1_dn5)) } } else { (assign102850_e154700 * ((locals.var_uc_rdrbb_s_dn5 * (locals.var_t1).ln()) + (assign102850_e154699 * (locals.var_t1_dn5 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn6 == 0.0 && ((assign102850_e154699) as f64).is_finite() && ((assign102850_e154699) as f64).fract() == 0.0 { if assign102850_e154699 == 0.0 { 0.0 } else { (assign102850_e154699 * ((locals.var_t1).powf(assign102850_e154699 - 1.0) * locals.var_t1_dn6)) } } else { (assign102850_e154700 * ((locals.var_uc_rdrbb_s_dn6 * (locals.var_t1).ln()) + (assign102850_e154699 * (locals.var_t1_dn6 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn7 == 0.0 && ((assign102850_e154699) as f64).is_finite() && ((assign102850_e154699) as f64).fract() == 0.0 { if assign102850_e154699 == 0.0 { 0.0 } else { (assign102850_e154699 * ((locals.var_t1).powf(assign102850_e154699 - 1.0) * locals.var_t1_dn7)) } } else { (assign102850_e154700 * ((locals.var_uc_rdrbb_s_dn7 * (locals.var_t1).ln()) + (assign102850_e154699 * (locals.var_t1_dn7 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn8 == 0.0 && ((assign102850_e154699) as f64).is_finite() && ((assign102850_e154699) as f64).fract() == 0.0 { if assign102850_e154699 == 0.0 { 0.0 } else { (assign102850_e154699 * ((locals.var_t1).powf(assign102850_e154699 - 1.0) * locals.var_t1_dn8)) } } else { (assign102850_e154700 * ((locals.var_uc_rdrbb_s_dn8 * (locals.var_t1).ln()) + (assign102850_e154699 * (locals.var_t1_dn8 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn9 == 0.0 && ((assign102850_e154699) as f64).is_finite() && ((assign102850_e154699) as f64).fract() == 0.0 { if assign102850_e154699 == 0.0 { 0.0 } else { (assign102850_e154699 * ((locals.var_t1).powf(assign102850_e154699 - 1.0) * locals.var_t1_dn9)) } } else { (assign102850_e154700 * ((locals.var_uc_rdrbb_s_dn9 * (locals.var_t1).ln()) + (assign102850_e154699 * (locals.var_t1_dn9 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn10 == 0.0 && ((assign102850_e154699) as f64).is_finite() && ((assign102850_e154699) as f64).fract() == 0.0 { if assign102850_e154699 == 0.0 { 0.0 } else { (assign102850_e154699 * ((locals.var_t1).powf(assign102850_e154699 - 1.0) * locals.var_t1_dn10)) } } else { (assign102850_e154700 * ((locals.var_uc_rdrbb_s_dn10 * (locals.var_t1).ln()) + (assign102850_e154699 * (locals.var_t1_dn10 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn11 == 0.0 && ((assign102850_e154699) as f64).is_finite() && ((assign102850_e154699) as f64).fract() == 0.0 { if assign102850_e154699 == 0.0 { 0.0 } else { (assign102850_e154699 * ((locals.var_t1).powf(assign102850_e154699 - 1.0) * locals.var_t1_dn11)) } } else { (assign102850_e154700 * ((locals.var_uc_rdrbb_s_dn11 * (locals.var_t1).ln()) + (assign102850_e154699 * (locals.var_t1_dn11 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn14 == 0.0 && ((assign102850_e154699) as f64).is_finite() && ((assign102850_e154699) as f64).fract() == 0.0 { if assign102850_e154699 == 0.0 { 0.0 } else { (assign102850_e154699 * ((locals.var_t1).powf(assign102850_e154699 - 1.0) * locals.var_t1_dn14)) } } else { (assign102850_e154700 * ((locals.var_uc_rdrbb_s_dn14 * (locals.var_t1).ln()) + (assign102850_e154699 * (locals.var_t1_dn14 / locals.var_t1)))) },)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign102850_e154702;
        locals.var_t3_dn0 = assign102850_e154702_d_n0;
        locals.var_t3_dn2 = assign102850_e154702_d_n2;
        locals.var_t3_dn4 = assign102850_e154702_d_n4;
        locals.var_t3_dn5 = assign102850_e154702_d_n5;
        locals.var_t3_dn6 = assign102850_e154702_d_n6;
        locals.var_t3_dn7 = assign102850_e154702_d_n7;
        locals.var_t3_dn8 = assign102850_e154702_d_n8;
        locals.var_t3_dn9 = assign102850_e154702_d_n9;
        locals.var_t3_dn10 = assign102850_e154702_d_n10;
        locals.var_t3_dn11 = assign102850_e154702_d_n11;
        locals.var_t3_dn14 = assign102850_e154702_d_n14;

        let (assign102860_e154711, assign102860_e154711_d_n0, assign102860_e154711_d_n2, assign102860_e154711_d_n4, assign102860_e154711_d_n5, assign102860_e154711_d_n6, assign102860_e154711_d_n7, assign102860_e154711_d_n8, assign102860_e154711_d_n9, assign102860_e154711_d_n10, assign102860_e154711_d_n11, assign102860_e154711_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) {
        let assign102860_e154709: f64 = (locals.var_t1 * locals.var_t3);
        (assign102860_e154709, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)), ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)), ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)), ((locals.var_t1_dn14 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign102860_e154711;
        locals.var_t2_dn0 = assign102860_e154711_d_n0;
        locals.var_t2_dn2 = assign102860_e154711_d_n2;
        locals.var_t2_dn4 = assign102860_e154711_d_n4;
        locals.var_t2_dn5 = assign102860_e154711_d_n5;
        locals.var_t2_dn6 = assign102860_e154711_d_n6;
        locals.var_t2_dn7 = assign102860_e154711_d_n7;
        locals.var_t2_dn8 = assign102860_e154711_d_n8;
        locals.var_t2_dn9 = assign102860_e154711_d_n9;
        locals.var_t2_dn10 = assign102860_e154711_d_n10;
        locals.var_t2_dn11 = assign102860_e154711_d_n11;
        locals.var_t2_dn14 = assign102860_e154711_d_n14;

        let (assign102870_e154720, assign102870_e154720_d_n0, assign102870_e154720_d_n2, assign102870_e154720_d_n4, assign102870_e154720_d_n5, assign102870_e154720_d_n6, assign102870_e154720_d_n7, assign102870_e154720_d_n8, assign102870_e154720_d_n9, assign102870_e154720_d_n10, assign102870_e154720_d_n11, assign102870_e154720_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) {
        let assign102870_e154718: f64 = (1.0 + locals.var_t2);
        (assign102870_e154718, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign102870_e154720;
        locals.var_t4_dn0 = assign102870_e154720_d_n0;
        locals.var_t4_dn2 = assign102870_e154720_d_n2;
        locals.var_t4_dn4 = assign102870_e154720_d_n4;
        locals.var_t4_dn5 = assign102870_e154720_d_n5;
        locals.var_t4_dn6 = assign102870_e154720_d_n6;
        locals.var_t4_dn7 = assign102870_e154720_d_n7;
        locals.var_t4_dn8 = assign102870_e154720_d_n8;
        locals.var_t4_dn9 = assign102870_e154720_d_n9;
        locals.var_t4_dn10 = assign102870_e154720_d_n10;
        locals.var_t4_dn11 = assign102870_e154720_d_n11;
        locals.var_t4_dn14 = assign102870_e154720_d_n14;

        let assign102880_e154724: f64 = (10.0 * 2.220446049250313e-16);
        let assign102880_e154725: f64 = (1.0 - assign102880_e154724);
        let assign102880_e154732: f64 = (10.0 * 2.220446049250313e-16);
        let assign102880_e154733: f64 = (1.0 + assign102880_e154732);
        let assign102880_e154735: f64 = if ((assign102880_e154725 <= locals.var_uc_rdrbb_s) && (locals.var_uc_rdrbb_s <= assign102880_e154733)) { 1.0 } else { 0.0 };
        locals.var_guard2344 = assign102880_e154735;

    }

    pub(super) fn stamp_transient_block_376(
        locals: &mut StampLocals,
    ) {
        let (assign102890_e154746, assign102890_e154746_d_n0, assign102890_e154746_d_n2, assign102890_e154746_d_n4, assign102890_e154746_d_n5, assign102890_e154746_d_n6, assign102890_e154746_d_n7, assign102890_e154746_d_n8, assign102890_e154746_d_n9, assign102890_e154746_d_n10, assign102890_e154746_d_n11, assign102890_e154746_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        let assign102890_e154744: f64 = (1.0 / locals.var_t4);
        (assign102890_e154744, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn14 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign102890_e154746;
        locals.var_t5_dn0 = assign102890_e154746_d_n0;
        locals.var_t5_dn2 = assign102890_e154746_d_n2;
        locals.var_t5_dn4 = assign102890_e154746_d_n4;
        locals.var_t5_dn5 = assign102890_e154746_d_n5;
        locals.var_t5_dn6 = assign102890_e154746_d_n6;
        locals.var_t5_dn7 = assign102890_e154746_d_n7;
        locals.var_t5_dn8 = assign102890_e154746_d_n8;
        locals.var_t5_dn9 = assign102890_e154746_d_n9;
        locals.var_t5_dn10 = assign102890_e154746_d_n10;
        locals.var_t5_dn11 = assign102890_e154746_d_n11;
        locals.var_t5_dn14 = assign102890_e154746_d_n14;

        let assign102900_e154750: f64 = (10.0 * 2.220446049250313e-16);
        let assign102900_e154751: f64 = (2.0 - assign102900_e154750);
        let assign102900_e154758: f64 = (10.0 * 2.220446049250313e-16);
        let assign102900_e154759: f64 = (2.0 + assign102900_e154758);
        let assign102900_e154761: f64 = if ((assign102900_e154751 <= locals.var_uc_rdrbb_s) && (locals.var_uc_rdrbb_s <= assign102900_e154759)) { 1.0 } else { 0.0 };
        locals.var_guard2345 = assign102900_e154761;

        let (assign102910_e154776, assign102910_e154776_d_n0, assign102910_e154776_d_n2, assign102910_e154776_d_n4, assign102910_e154776_d_n5, assign102910_e154776_d_n6, assign102910_e154776_d_n7, assign102910_e154776_d_n8, assign102910_e154776_d_n9, assign102910_e154776_d_n10, assign102910_e154776_d_n11, assign102910_e154776_d_n14,) = {
    if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2344 == 0.0)) && (locals.var_guard2345 != 0.0)) {
        let assign102910_e154773: f64 = (locals.var_t4).sqrt();
        let assign102910_e154774: f64 = (1.0 / assign102910_e154773);
        (assign102910_e154774, (-((locals.var_t4_dn0 / (2.0 * assign102910_e154773)) / (assign102910_e154773 * assign102910_e154773))), (-((locals.var_t4_dn2 / (2.0 * assign102910_e154773)) / (assign102910_e154773 * assign102910_e154773))), (-((locals.var_t4_dn4 / (2.0 * assign102910_e154773)) / (assign102910_e154773 * assign102910_e154773))), (-((locals.var_t4_dn5 / (2.0 * assign102910_e154773)) / (assign102910_e154773 * assign102910_e154773))), (-((locals.var_t4_dn6 / (2.0 * assign102910_e154773)) / (assign102910_e154773 * assign102910_e154773))), (-((locals.var_t4_dn7 / (2.0 * assign102910_e154773)) / (assign102910_e154773 * assign102910_e154773))), (-((locals.var_t4_dn8 / (2.0 * assign102910_e154773)) / (assign102910_e154773 * assign102910_e154773))), (-((locals.var_t4_dn9 / (2.0 * assign102910_e154773)) / (assign102910_e154773 * assign102910_e154773))), (-((locals.var_t4_dn10 / (2.0 * assign102910_e154773)) / (assign102910_e154773 * assign102910_e154773))), (-((locals.var_t4_dn11 / (2.0 * assign102910_e154773)) / (assign102910_e154773 * assign102910_e154773))), (-((locals.var_t4_dn14 / (2.0 * assign102910_e154773)) / (assign102910_e154773 * assign102910_e154773))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign102910_e154776;
        locals.var_t5_dn0 = assign102910_e154776_d_n0;
        locals.var_t5_dn2 = assign102910_e154776_d_n2;
        locals.var_t5_dn4 = assign102910_e154776_d_n4;
        locals.var_t5_dn5 = assign102910_e154776_d_n5;
        locals.var_t5_dn6 = assign102910_e154776_d_n6;
        locals.var_t5_dn7 = assign102910_e154776_d_n7;
        locals.var_t5_dn8 = assign102910_e154776_d_n8;
        locals.var_t5_dn9 = assign102910_e154776_d_n9;
        locals.var_t5_dn10 = assign102910_e154776_d_n10;
        locals.var_t5_dn11 = assign102910_e154776_d_n11;
        locals.var_t5_dn14 = assign102910_e154776_d_n14;

        let (assign102920_e154801, assign102920_e154801_d_n0, assign102920_e154801_d_n2, assign102920_e154801_d_n4, assign102920_e154801_d_n5, assign102920_e154801_d_n6, assign102920_e154801_d_n7, assign102920_e154801_d_n8, assign102920_e154801_d_n9, assign102920_e154801_d_n10, assign102920_e154801_d_n11, assign102920_e154801_d_n14,) = {
    if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2344 == 0.0)) && (locals.var_guard2345 == 0.0)) {
        let (assign102920_e154799, assign102920_e154799_d_n0, assign102920_e154799_d_n2, assign102920_e154799_d_n4, assign102920_e154799_d_n5, assign102920_e154799_d_n6, assign102920_e154799_d_n7, assign102920_e154799_d_n8, assign102920_e154799_d_n9, assign102920_e154799_d_n10, assign102920_e154799_d_n11, assign102920_e154799_d_n14,) = {
            if (locals.var_t4 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign102920_e154793: f64 = (-1.0);
                let assign102920_e154795: f64 = (assign102920_e154793 / locals.var_uc_rdrbb_s);
                let assign102920_e154797: f64 = (assign102920_e154795 - 1.0);
                let assign102920_e154798: f64 = (locals.var_t4).powf(assign102920_e154797);
                (assign102920_e154798, if (-((assign102920_e154793 * locals.var_uc_rdrbb_s_dn0) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102920_e154797) as f64).is_finite() && ((assign102920_e154797) as f64).fract() == 0.0 { if assign102920_e154797 == 0.0 { 0.0 } else { (assign102920_e154797 * ((locals.var_t4).powf(assign102920_e154797 - 1.0) * locals.var_t4_dn0)) } } else { (assign102920_e154798 * (((-((assign102920_e154793 * locals.var_uc_rdrbb_s_dn0) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102920_e154797 * (locals.var_t4_dn0 / locals.var_t4)))) }, if (-((assign102920_e154793 * locals.var_uc_rdrbb_s_dn2) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102920_e154797) as f64).is_finite() && ((assign102920_e154797) as f64).fract() == 0.0 { if assign102920_e154797 == 0.0 { 0.0 } else { (assign102920_e154797 * ((locals.var_t4).powf(assign102920_e154797 - 1.0) * locals.var_t4_dn2)) } } else { (assign102920_e154798 * (((-((assign102920_e154793 * locals.var_uc_rdrbb_s_dn2) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102920_e154797 * (locals.var_t4_dn2 / locals.var_t4)))) }, if (-((assign102920_e154793 * locals.var_uc_rdrbb_s_dn4) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102920_e154797) as f64).is_finite() && ((assign102920_e154797) as f64).fract() == 0.0 { if assign102920_e154797 == 0.0 { 0.0 } else { (assign102920_e154797 * ((locals.var_t4).powf(assign102920_e154797 - 1.0) * locals.var_t4_dn4)) } } else { (assign102920_e154798 * (((-((assign102920_e154793 * locals.var_uc_rdrbb_s_dn4) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102920_e154797 * (locals.var_t4_dn4 / locals.var_t4)))) }, if (-((assign102920_e154793 * locals.var_uc_rdrbb_s_dn5) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102920_e154797) as f64).is_finite() && ((assign102920_e154797) as f64).fract() == 0.0 { if assign102920_e154797 == 0.0 { 0.0 } else { (assign102920_e154797 * ((locals.var_t4).powf(assign102920_e154797 - 1.0) * locals.var_t4_dn5)) } } else { (assign102920_e154798 * (((-((assign102920_e154793 * locals.var_uc_rdrbb_s_dn5) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102920_e154797 * (locals.var_t4_dn5 / locals.var_t4)))) }, if (-((assign102920_e154793 * locals.var_uc_rdrbb_s_dn6) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102920_e154797) as f64).is_finite() && ((assign102920_e154797) as f64).fract() == 0.0 { if assign102920_e154797 == 0.0 { 0.0 } else { (assign102920_e154797 * ((locals.var_t4).powf(assign102920_e154797 - 1.0) * locals.var_t4_dn6)) } } else { (assign102920_e154798 * (((-((assign102920_e154793 * locals.var_uc_rdrbb_s_dn6) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102920_e154797 * (locals.var_t4_dn6 / locals.var_t4)))) }, if (-((assign102920_e154793 * locals.var_uc_rdrbb_s_dn7) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102920_e154797) as f64).is_finite() && ((assign102920_e154797) as f64).fract() == 0.0 { if assign102920_e154797 == 0.0 { 0.0 } else { (assign102920_e154797 * ((locals.var_t4).powf(assign102920_e154797 - 1.0) * locals.var_t4_dn7)) } } else { (assign102920_e154798 * (((-((assign102920_e154793 * locals.var_uc_rdrbb_s_dn7) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102920_e154797 * (locals.var_t4_dn7 / locals.var_t4)))) }, if (-((assign102920_e154793 * locals.var_uc_rdrbb_s_dn8) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102920_e154797) as f64).is_finite() && ((assign102920_e154797) as f64).fract() == 0.0 { if assign102920_e154797 == 0.0 { 0.0 } else { (assign102920_e154797 * ((locals.var_t4).powf(assign102920_e154797 - 1.0) * locals.var_t4_dn8)) } } else { (assign102920_e154798 * (((-((assign102920_e154793 * locals.var_uc_rdrbb_s_dn8) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102920_e154797 * (locals.var_t4_dn8 / locals.var_t4)))) }, if (-((assign102920_e154793 * locals.var_uc_rdrbb_s_dn9) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102920_e154797) as f64).is_finite() && ((assign102920_e154797) as f64).fract() == 0.0 { if assign102920_e154797 == 0.0 { 0.0 } else { (assign102920_e154797 * ((locals.var_t4).powf(assign102920_e154797 - 1.0) * locals.var_t4_dn9)) } } else { (assign102920_e154798 * (((-((assign102920_e154793 * locals.var_uc_rdrbb_s_dn9) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102920_e154797 * (locals.var_t4_dn9 / locals.var_t4)))) }, if (-((assign102920_e154793 * locals.var_uc_rdrbb_s_dn10) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102920_e154797) as f64).is_finite() && ((assign102920_e154797) as f64).fract() == 0.0 { if assign102920_e154797 == 0.0 { 0.0 } else { (assign102920_e154797 * ((locals.var_t4).powf(assign102920_e154797 - 1.0) * locals.var_t4_dn10)) } } else { (assign102920_e154798 * (((-((assign102920_e154793 * locals.var_uc_rdrbb_s_dn10) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102920_e154797 * (locals.var_t4_dn10 / locals.var_t4)))) }, if (-((assign102920_e154793 * locals.var_uc_rdrbb_s_dn11) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102920_e154797) as f64).is_finite() && ((assign102920_e154797) as f64).fract() == 0.0 { if assign102920_e154797 == 0.0 { 0.0 } else { (assign102920_e154797 * ((locals.var_t4).powf(assign102920_e154797 - 1.0) * locals.var_t4_dn11)) } } else { (assign102920_e154798 * (((-((assign102920_e154793 * locals.var_uc_rdrbb_s_dn11) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102920_e154797 * (locals.var_t4_dn11 / locals.var_t4)))) }, if (-((assign102920_e154793 * locals.var_uc_rdrbb_s_dn14) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102920_e154797) as f64).is_finite() && ((assign102920_e154797) as f64).fract() == 0.0 { if assign102920_e154797 == 0.0 { 0.0 } else { (assign102920_e154797 * ((locals.var_t4).powf(assign102920_e154797 - 1.0) * locals.var_t4_dn14)) } } else { (assign102920_e154798 * (((-((assign102920_e154793 * locals.var_uc_rdrbb_s_dn14) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102920_e154797 * (locals.var_t4_dn14 / locals.var_t4)))) },)
            }
        };
        (assign102920_e154799, assign102920_e154799_d_n0, assign102920_e154799_d_n2, assign102920_e154799_d_n4, assign102920_e154799_d_n5, assign102920_e154799_d_n6, assign102920_e154799_d_n7, assign102920_e154799_d_n8, assign102920_e154799_d_n9, assign102920_e154799_d_n10, assign102920_e154799_d_n11, assign102920_e154799_d_n14,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign102920_e154801;
        locals.var_t6_dn0 = assign102920_e154801_d_n0;
        locals.var_t6_dn2 = assign102920_e154801_d_n2;
        locals.var_t6_dn4 = assign102920_e154801_d_n4;
        locals.var_t6_dn5 = assign102920_e154801_d_n5;
        locals.var_t6_dn6 = assign102920_e154801_d_n6;
        locals.var_t6_dn7 = assign102920_e154801_d_n7;
        locals.var_t6_dn8 = assign102920_e154801_d_n8;
        locals.var_t6_dn9 = assign102920_e154801_d_n9;
        locals.var_t6_dn10 = assign102920_e154801_d_n10;
        locals.var_t6_dn11 = assign102920_e154801_d_n11;
        locals.var_t6_dn14 = assign102920_e154801_d_n14;

        let (assign102930_e154816, assign102930_e154816_d_n0, assign102930_e154816_d_n2, assign102930_e154816_d_n4, assign102930_e154816_d_n5, assign102930_e154816_d_n6, assign102930_e154816_d_n7, assign102930_e154816_d_n8, assign102930_e154816_d_n9, assign102930_e154816_d_n10, assign102930_e154816_d_n11, assign102930_e154816_d_n14,) = {
    if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2344 == 0.0)) && (locals.var_guard2345 == 0.0)) {
        let assign102930_e154814: f64 = (locals.var_t4 * locals.var_t6);
        (assign102930_e154814, ((locals.var_t4_dn0 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn0)), ((locals.var_t4_dn2 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn2)), ((locals.var_t4_dn4 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn4)), ((locals.var_t4_dn5 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn5)), ((locals.var_t4_dn6 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn6)), ((locals.var_t4_dn7 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn7)), ((locals.var_t4_dn8 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn8)), ((locals.var_t4_dn9 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn9)), ((locals.var_t4_dn10 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn10)), ((locals.var_t4_dn11 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn11)), ((locals.var_t4_dn14 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign102930_e154816;
        locals.var_t5_dn0 = assign102930_e154816_d_n0;
        locals.var_t5_dn2 = assign102930_e154816_d_n2;
        locals.var_t5_dn4 = assign102930_e154816_d_n4;
        locals.var_t5_dn5 = assign102930_e154816_d_n5;
        locals.var_t5_dn6 = assign102930_e154816_d_n6;
        locals.var_t5_dn7 = assign102930_e154816_d_n7;
        locals.var_t5_dn8 = assign102930_e154816_d_n8;
        locals.var_t5_dn9 = assign102930_e154816_d_n9;
        locals.var_t5_dn10 = assign102930_e154816_d_n10;
        locals.var_t5_dn11 = assign102930_e154816_d_n11;
        locals.var_t5_dn14 = assign102930_e154816_d_n14;

        let (assign102940_e154825, assign102940_e154825_d_n0, assign102940_e154825_d_n2, assign102940_e154825_d_n4, assign102940_e154825_d_n5, assign102940_e154825_d_n6, assign102940_e154825_d_n7, assign102940_e154825_d_n8, assign102940_e154825_d_n9, assign102940_e154825_d_n10, assign102940_e154825_d_n11, assign102940_e154825_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) {
        let assign102940_e154823: f64 = (locals.var_mu0_s * locals.var_t5);
        (assign102940_e154823, ((locals.var_mu0_s_dn0 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn0)), ((locals.var_mu0_s_dn2 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn2)), ((locals.var_mu0_s_dn4 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn4)), ((locals.var_mu0_s_dn5 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn5)), ((locals.var_mu0_s_dn6 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn6)), ((locals.var_mu0_s_dn7 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn7)), ((locals.var_mu0_s_dn8 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn8)), ((locals.var_mu0_s_dn9 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn9)), ((locals.var_mu0_s_dn10 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn10)), ((locals.var_mu0_s_dn11 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn11)), ((locals.var_mu0_s_dn14 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn14)),)
    } else {
        (locals.var_mu_s, locals.var_mu_s_dn0, locals.var_mu_s_dn2, locals.var_mu_s_dn4, locals.var_mu_s_dn5, locals.var_mu_s_dn6, locals.var_mu_s_dn7, locals.var_mu_s_dn8, locals.var_mu_s_dn9, locals.var_mu_s_dn10, locals.var_mu_s_dn11, locals.var_mu_s_dn14,)
    }
};
        locals.var_mu_s = assign102940_e154825;
        locals.var_mu_s_dn0 = assign102940_e154825_d_n0;
        locals.var_mu_s_dn2 = assign102940_e154825_d_n2;
        locals.var_mu_s_dn4 = assign102940_e154825_d_n4;
        locals.var_mu_s_dn5 = assign102940_e154825_d_n5;
        locals.var_mu_s_dn6 = assign102940_e154825_d_n6;
        locals.var_mu_s_dn7 = assign102940_e154825_d_n7;
        locals.var_mu_s_dn8 = assign102940_e154825_d_n8;
        locals.var_mu_s_dn9 = assign102940_e154825_d_n9;
        locals.var_mu_s_dn10 = assign102940_e154825_d_n10;
        locals.var_mu_s_dn11 = assign102940_e154825_d_n11;
        locals.var_mu_s_dn14 = assign102940_e154825_d_n14;

        let (assign102950_e154832,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) {
        (locals.var_novers,)
    } else {
        (locals.var_carr_s,)
    }
};
        locals.var_carr_s = assign102950_e154832;

        let (assign102960_e154839,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) {
        (locals.var_xmax_s,)
    } else {
        (locals.var_xov_s,)
    }
};
        locals.var_xov_s = assign102960_e154839;

        let (assign102970_e154848, assign102970_e154848_d_n0, assign102970_e154848_d_n2, assign102970_e154848_d_n4, assign102970_e154848_d_n5, assign102970_e154848_d_n6, assign102970_e154848_d_n7, assign102970_e154848_d_n8, assign102970_e154848_d_n9, assign102970_e154848_d_n10, assign102970_e154848_d_n11, assign102970_e154848_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) {
        let assign102970_e154846: f64 = (1.6021918e-19 / locals.var_ldrifte_s);
        (assign102970_e154846, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign102970_e154848;
        locals.var_t1_dn0 = assign102970_e154848_d_n0;
        locals.var_t1_dn2 = assign102970_e154848_d_n2;
        locals.var_t1_dn4 = assign102970_e154848_d_n4;
        locals.var_t1_dn5 = assign102970_e154848_d_n5;
        locals.var_t1_dn6 = assign102970_e154848_d_n6;
        locals.var_t1_dn7 = assign102970_e154848_d_n7;
        locals.var_t1_dn8 = assign102970_e154848_d_n8;
        locals.var_t1_dn9 = assign102970_e154848_d_n9;
        locals.var_t1_dn10 = assign102970_e154848_d_n10;
        locals.var_t1_dn11 = assign102970_e154848_d_n11;
        locals.var_t1_dn14 = assign102970_e154848_d_n14;

        let (assign102980_e154861, assign102980_e154861_d_n0, assign102980_e154861_d_n2, assign102980_e154861_d_n4, assign102980_e154861_d_n5, assign102980_e154861_d_n6, assign102980_e154861_d_n7, assign102980_e154861_d_n8, assign102980_e154861_d_n9, assign102980_e154861_d_n10, assign102980_e154861_d_n11, assign102980_e154861_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) {
        let assign102980_e154855: f64 = (locals.var_t1 * locals.var_xov_s);
        let assign102980_e154857: f64 = (assign102980_e154855 * locals.var_mu_s);
        let assign102980_e154859: f64 = (assign102980_e154857 * locals.var_carr_s);
        (assign102980_e154859, ((((locals.var_t1_dn0 * locals.var_xov_s) * locals.var_mu_s) + (assign102980_e154855 * locals.var_mu_s_dn0)) * locals.var_carr_s), ((((locals.var_t1_dn2 * locals.var_xov_s) * locals.var_mu_s) + (assign102980_e154855 * locals.var_mu_s_dn2)) * locals.var_carr_s), ((((locals.var_t1_dn4 * locals.var_xov_s) * locals.var_mu_s) + (assign102980_e154855 * locals.var_mu_s_dn4)) * locals.var_carr_s), ((((locals.var_t1_dn5 * locals.var_xov_s) * locals.var_mu_s) + (assign102980_e154855 * locals.var_mu_s_dn5)) * locals.var_carr_s), ((((locals.var_t1_dn6 * locals.var_xov_s) * locals.var_mu_s) + (assign102980_e154855 * locals.var_mu_s_dn6)) * locals.var_carr_s), ((((locals.var_t1_dn7 * locals.var_xov_s) * locals.var_mu_s) + (assign102980_e154855 * locals.var_mu_s_dn7)) * locals.var_carr_s), ((((locals.var_t1_dn8 * locals.var_xov_s) * locals.var_mu_s) + (assign102980_e154855 * locals.var_mu_s_dn8)) * locals.var_carr_s), ((((locals.var_t1_dn9 * locals.var_xov_s) * locals.var_mu_s) + (assign102980_e154855 * locals.var_mu_s_dn9)) * locals.var_carr_s), ((((locals.var_t1_dn10 * locals.var_xov_s) * locals.var_mu_s) + (assign102980_e154855 * locals.var_mu_s_dn10)) * locals.var_carr_s), ((((locals.var_t1_dn11 * locals.var_xov_s) * locals.var_mu_s) + (assign102980_e154855 * locals.var_mu_s_dn11)) * locals.var_carr_s), ((((locals.var_t1_dn14 * locals.var_xov_s) * locals.var_mu_s) + (assign102980_e154855 * locals.var_mu_s_dn14)) * locals.var_carr_s),)
    } else {
        (locals.var_gd_s, locals.var_gd_s_dn0, locals.var_gd_s_dn2, locals.var_gd_s_dn4, locals.var_gd_s_dn5, locals.var_gd_s_dn6, locals.var_gd_s_dn7, locals.var_gd_s_dn8, locals.var_gd_s_dn9, locals.var_gd_s_dn10, locals.var_gd_s_dn11, locals.var_gd_s_dn14,)
    }
};
        locals.var_gd_s = assign102980_e154861;
        locals.var_gd_s_dn0 = assign102980_e154861_d_n0;
        locals.var_gd_s_dn2 = assign102980_e154861_d_n2;
        locals.var_gd_s_dn4 = assign102980_e154861_d_n4;
        locals.var_gd_s_dn5 = assign102980_e154861_d_n5;
        locals.var_gd_s_dn6 = assign102980_e154861_d_n6;
        locals.var_gd_s_dn7 = assign102980_e154861_d_n7;
        locals.var_gd_s_dn8 = assign102980_e154861_d_n8;
        locals.var_gd_s_dn9 = assign102980_e154861_d_n9;
        locals.var_gd_s_dn10 = assign102980_e154861_d_n10;
        locals.var_gd_s_dn11 = assign102980_e154861_d_n11;
        locals.var_gd_s_dn14 = assign102980_e154861_d_n14;

        let assign102990_e154865: f64 = 1e-25;
        let assign102990_e154870: f64 = if ((locals.var_gd_s < assign102990_e154865) && (1e-25 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2346 = assign102990_e154870;

        let (assign103000_e154883, assign103000_e154883_d_n0, assign103000_e154883_d_n2, assign103000_e154883_d_n4, assign103000_e154883_d_n5, assign103000_e154883_d_n6, assign103000_e154883_d_n7, assign103000_e154883_d_n8, assign103000_e154883_d_n9, assign103000_e154883_d_n10, assign103000_e154883_d_n11, assign103000_e154883_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2346 != 0.0)) {
        let assign103000_e154879: f64 = 1e-25;
        let assign103000_e154881: f64 = (assign103000_e154879 - locals.var_gd_s);
        (assign103000_e154881, (-locals.var_gd_s_dn0), (-locals.var_gd_s_dn2), (-locals.var_gd_s_dn4), (-locals.var_gd_s_dn5), (-locals.var_gd_s_dn6), (-locals.var_gd_s_dn7), (-locals.var_gd_s_dn8), (-locals.var_gd_s_dn9), (-locals.var_gd_s_dn10), (-locals.var_gd_s_dn11), (-locals.var_gd_s_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign103000_e154883;
        locals.var_tmf1_dn0 = assign103000_e154883_d_n0;
        locals.var_tmf1_dn2 = assign103000_e154883_d_n2;
        locals.var_tmf1_dn4 = assign103000_e154883_d_n4;
        locals.var_tmf1_dn5 = assign103000_e154883_d_n5;
        locals.var_tmf1_dn6 = assign103000_e154883_d_n6;
        locals.var_tmf1_dn7 = assign103000_e154883_d_n7;
        locals.var_tmf1_dn8 = assign103000_e154883_d_n8;
        locals.var_tmf1_dn9 = assign103000_e154883_d_n9;
        locals.var_tmf1_dn10 = assign103000_e154883_d_n10;
        locals.var_tmf1_dn11 = assign103000_e154883_d_n11;
        locals.var_tmf1_dn14 = assign103000_e154883_d_n14;

        let (assign103010_e154894, assign103010_e154894_d_n0, assign103010_e154894_d_n2, assign103010_e154894_d_n4, assign103010_e154894_d_n5, assign103010_e154894_d_n6, assign103010_e154894_d_n7, assign103010_e154894_d_n8, assign103010_e154894_d_n9, assign103010_e154894_d_n10, assign103010_e154894_d_n11, assign103010_e154894_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2346 != 0.0)) {
        let assign103010_e154892: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign103010_e154892, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign103010_e154894;
        locals.var_x2_dn0 = assign103010_e154894_d_n0;
        locals.var_x2_dn2 = assign103010_e154894_d_n2;
        locals.var_x2_dn4 = assign103010_e154894_d_n4;
        locals.var_x2_dn5 = assign103010_e154894_d_n5;
        locals.var_x2_dn6 = assign103010_e154894_d_n6;
        locals.var_x2_dn7 = assign103010_e154894_d_n7;
        locals.var_x2_dn8 = assign103010_e154894_d_n8;
        locals.var_x2_dn9 = assign103010_e154894_d_n9;
        locals.var_x2_dn10 = assign103010_e154894_d_n10;
        locals.var_x2_dn11 = assign103010_e154894_d_n11;
        locals.var_x2_dn14 = assign103010_e154894_d_n14;

        let (assign103020_e154905, assign103020_e154905_d_n0, assign103020_e154905_d_n2, assign103020_e154905_d_n4, assign103020_e154905_d_n5, assign103020_e154905_d_n6, assign103020_e154905_d_n7, assign103020_e154905_d_n8, assign103020_e154905_d_n9, assign103020_e154905_d_n10, assign103020_e154905_d_n11, assign103020_e154905_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2346 != 0.0)) {
        let assign103020_e154903: f64 = (1e-25 * 1e-25);
        (assign103020_e154903, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign103020_e154905;
        locals.var_xmax2_dn0 = assign103020_e154905_d_n0;
        locals.var_xmax2_dn2 = assign103020_e154905_d_n2;
        locals.var_xmax2_dn4 = assign103020_e154905_d_n4;
        locals.var_xmax2_dn5 = assign103020_e154905_d_n5;
        locals.var_xmax2_dn6 = assign103020_e154905_d_n6;
        locals.var_xmax2_dn7 = assign103020_e154905_d_n7;
        locals.var_xmax2_dn8 = assign103020_e154905_d_n8;
        locals.var_xmax2_dn9 = assign103020_e154905_d_n9;
        locals.var_xmax2_dn10 = assign103020_e154905_d_n10;
        locals.var_xmax2_dn11 = assign103020_e154905_d_n11;
        locals.var_xmax2_dn14 = assign103020_e154905_d_n14;

        let (assign103030_e154914, assign103030_e154914_d_n0, assign103030_e154914_d_n2, assign103030_e154914_d_n4, assign103030_e154914_d_n5, assign103030_e154914_d_n6, assign103030_e154914_d_n7, assign103030_e154914_d_n8, assign103030_e154914_d_n9, assign103030_e154914_d_n10, assign103030_e154914_d_n11, assign103030_e154914_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2346 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign103030_e154914;
        locals.var_xp_dn0 = assign103030_e154914_d_n0;
        locals.var_xp_dn2 = assign103030_e154914_d_n2;
        locals.var_xp_dn4 = assign103030_e154914_d_n4;
        locals.var_xp_dn5 = assign103030_e154914_d_n5;
        locals.var_xp_dn6 = assign103030_e154914_d_n6;
        locals.var_xp_dn7 = assign103030_e154914_d_n7;
        locals.var_xp_dn8 = assign103030_e154914_d_n8;
        locals.var_xp_dn9 = assign103030_e154914_d_n9;
        locals.var_xp_dn10 = assign103030_e154914_d_n10;
        locals.var_xp_dn11 = assign103030_e154914_d_n11;
        locals.var_xp_dn14 = assign103030_e154914_d_n14;

        let (assign103040_e154923, assign103040_e154923_d_n0, assign103040_e154923_d_n2, assign103040_e154923_d_n4, assign103040_e154923_d_n5, assign103040_e154923_d_n6, assign103040_e154923_d_n7, assign103040_e154923_d_n8, assign103040_e154923_d_n9, assign103040_e154923_d_n10, assign103040_e154923_d_n11, assign103040_e154923_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2346 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign103040_e154923;
        locals.var_xmp_dn0 = assign103040_e154923_d_n0;
        locals.var_xmp_dn2 = assign103040_e154923_d_n2;
        locals.var_xmp_dn4 = assign103040_e154923_d_n4;
        locals.var_xmp_dn5 = assign103040_e154923_d_n5;
        locals.var_xmp_dn6 = assign103040_e154923_d_n6;
        locals.var_xmp_dn7 = assign103040_e154923_d_n7;
        locals.var_xmp_dn8 = assign103040_e154923_d_n8;
        locals.var_xmp_dn9 = assign103040_e154923_d_n9;
        locals.var_xmp_dn10 = assign103040_e154923_d_n10;
        locals.var_xmp_dn11 = assign103040_e154923_d_n11;
        locals.var_xmp_dn14 = assign103040_e154923_d_n14;

        let (assign103050_e154932,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2346 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign103050_e154932;

        let (assign103060_e154941,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2346 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign103060_e154941;

        let (assign103070_e154950, assign103070_e154950_d_n0, assign103070_e154950_d_n2, assign103070_e154950_d_n4, assign103070_e154950_d_n5, assign103070_e154950_d_n6, assign103070_e154950_d_n7, assign103070_e154950_d_n8, assign103070_e154950_d_n9, assign103070_e154950_d_n10, assign103070_e154950_d_n11, assign103070_e154950_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2346 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign103070_e154950;
        locals.var_arg_dn0 = assign103070_e154950_d_n0;
        locals.var_arg_dn2 = assign103070_e154950_d_n2;
        locals.var_arg_dn4 = assign103070_e154950_d_n4;
        locals.var_arg_dn5 = assign103070_e154950_d_n5;
        locals.var_arg_dn6 = assign103070_e154950_d_n6;
        locals.var_arg_dn7 = assign103070_e154950_d_n7;
        locals.var_arg_dn8 = assign103070_e154950_d_n8;
        locals.var_arg_dn9 = assign103070_e154950_d_n9;
        locals.var_arg_dn10 = assign103070_e154950_d_n10;
        locals.var_arg_dn11 = assign103070_e154950_d_n11;
        locals.var_arg_dn14 = assign103070_e154950_d_n14;

        let (assign103080_e154959, assign103080_e154959_d_n0, assign103080_e154959_d_n2, assign103080_e154959_d_n4, assign103080_e154959_d_n5, assign103080_e154959_d_n6, assign103080_e154959_d_n7, assign103080_e154959_d_n8, assign103080_e154959_d_n9, assign103080_e154959_d_n10, assign103080_e154959_d_n11, assign103080_e154959_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2346 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign103080_e154959;
        locals.var_dnm_dn0 = assign103080_e154959_d_n0;
        locals.var_dnm_dn2 = assign103080_e154959_d_n2;
        locals.var_dnm_dn4 = assign103080_e154959_d_n4;
        locals.var_dnm_dn5 = assign103080_e154959_d_n5;
        locals.var_dnm_dn6 = assign103080_e154959_d_n6;
        locals.var_dnm_dn7 = assign103080_e154959_d_n7;
        locals.var_dnm_dn8 = assign103080_e154959_d_n8;
        locals.var_dnm_dn9 = assign103080_e154959_d_n9;
        locals.var_dnm_dn10 = assign103080_e154959_d_n10;
        locals.var_dnm_dn11 = assign103080_e154959_d_n11;
        locals.var_dnm_dn14 = assign103080_e154959_d_n14;

        let (assign103090_e154970, assign103090_e154970_d_n0, assign103090_e154970_d_n2, assign103090_e154970_d_n4, assign103090_e154970_d_n5, assign103090_e154970_d_n6, assign103090_e154970_d_n7, assign103090_e154970_d_n8, assign103090_e154970_d_n9, assign103090_e154970_d_n10, assign103090_e154970_d_n11, assign103090_e154970_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2346 != 0.0)) {
        let assign103090_e154968: f64 = (locals.var_xp * locals.var_x2);
        (assign103090_e154968, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign103090_e154970;
        locals.var_xp_dn0 = assign103090_e154970_d_n0;
        locals.var_xp_dn2 = assign103090_e154970_d_n2;
        locals.var_xp_dn4 = assign103090_e154970_d_n4;
        locals.var_xp_dn5 = assign103090_e154970_d_n5;
        locals.var_xp_dn6 = assign103090_e154970_d_n6;
        locals.var_xp_dn7 = assign103090_e154970_d_n7;
        locals.var_xp_dn8 = assign103090_e154970_d_n8;
        locals.var_xp_dn9 = assign103090_e154970_d_n9;
        locals.var_xp_dn10 = assign103090_e154970_d_n10;
        locals.var_xp_dn11 = assign103090_e154970_d_n11;
        locals.var_xp_dn14 = assign103090_e154970_d_n14;

        let (assign103100_e154981, assign103100_e154981_d_n0, assign103100_e154981_d_n2, assign103100_e154981_d_n4, assign103100_e154981_d_n5, assign103100_e154981_d_n6, assign103100_e154981_d_n7, assign103100_e154981_d_n8, assign103100_e154981_d_n9, assign103100_e154981_d_n10, assign103100_e154981_d_n11, assign103100_e154981_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2346 != 0.0)) {
        let assign103100_e154979: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign103100_e154979, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign103100_e154981;
        locals.var_xmp_dn0 = assign103100_e154981_d_n0;
        locals.var_xmp_dn2 = assign103100_e154981_d_n2;
        locals.var_xmp_dn4 = assign103100_e154981_d_n4;
        locals.var_xmp_dn5 = assign103100_e154981_d_n5;
        locals.var_xmp_dn6 = assign103100_e154981_d_n6;
        locals.var_xmp_dn7 = assign103100_e154981_d_n7;
        locals.var_xmp_dn8 = assign103100_e154981_d_n8;
        locals.var_xmp_dn9 = assign103100_e154981_d_n9;
        locals.var_xmp_dn10 = assign103100_e154981_d_n10;
        locals.var_xmp_dn11 = assign103100_e154981_d_n11;
        locals.var_xmp_dn14 = assign103100_e154981_d_n14;

        let (assign103110_e154992, assign103110_e154992_d_n0, assign103110_e154992_d_n2, assign103110_e154992_d_n4, assign103110_e154992_d_n5, assign103110_e154992_d_n6, assign103110_e154992_d_n7, assign103110_e154992_d_n8, assign103110_e154992_d_n9, assign103110_e154992_d_n10, assign103110_e154992_d_n11, assign103110_e154992_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2346 != 0.0)) {
        let assign103110_e154990: f64 = (locals.var_xp * locals.var_x2);
        (assign103110_e154990, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign103110_e154992;
        locals.var_xp_dn0 = assign103110_e154992_d_n0;
        locals.var_xp_dn2 = assign103110_e154992_d_n2;
        locals.var_xp_dn4 = assign103110_e154992_d_n4;
        locals.var_xp_dn5 = assign103110_e154992_d_n5;
        locals.var_xp_dn6 = assign103110_e154992_d_n6;
        locals.var_xp_dn7 = assign103110_e154992_d_n7;
        locals.var_xp_dn8 = assign103110_e154992_d_n8;
        locals.var_xp_dn9 = assign103110_e154992_d_n9;
        locals.var_xp_dn10 = assign103110_e154992_d_n10;
        locals.var_xp_dn11 = assign103110_e154992_d_n11;
        locals.var_xp_dn14 = assign103110_e154992_d_n14;

        let (assign103120_e155003, assign103120_e155003_d_n0, assign103120_e155003_d_n2, assign103120_e155003_d_n4, assign103120_e155003_d_n5, assign103120_e155003_d_n6, assign103120_e155003_d_n7, assign103120_e155003_d_n8, assign103120_e155003_d_n9, assign103120_e155003_d_n10, assign103120_e155003_d_n11, assign103120_e155003_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2346 != 0.0)) {
        let assign103120_e155001: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign103120_e155001, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign103120_e155003;
        locals.var_xmp_dn0 = assign103120_e155003_d_n0;
        locals.var_xmp_dn2 = assign103120_e155003_d_n2;
        locals.var_xmp_dn4 = assign103120_e155003_d_n4;
        locals.var_xmp_dn5 = assign103120_e155003_d_n5;
        locals.var_xmp_dn6 = assign103120_e155003_d_n6;
        locals.var_xmp_dn7 = assign103120_e155003_d_n7;
        locals.var_xmp_dn8 = assign103120_e155003_d_n8;
        locals.var_xmp_dn9 = assign103120_e155003_d_n9;
        locals.var_xmp_dn10 = assign103120_e155003_d_n10;
        locals.var_xmp_dn11 = assign103120_e155003_d_n11;
        locals.var_xmp_dn14 = assign103120_e155003_d_n14;

        let (assign103130_e155014, assign103130_e155014_d_n0, assign103130_e155014_d_n2, assign103130_e155014_d_n4, assign103130_e155014_d_n5, assign103130_e155014_d_n6, assign103130_e155014_d_n7, assign103130_e155014_d_n8, assign103130_e155014_d_n9, assign103130_e155014_d_n10, assign103130_e155014_d_n11, assign103130_e155014_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2346 != 0.0)) {
        let assign103130_e155012: f64 = (locals.var_xp + locals.var_xmp);
        (assign103130_e155012, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign103130_e155014;
        locals.var_arg_dn0 = assign103130_e155014_d_n0;
        locals.var_arg_dn2 = assign103130_e155014_d_n2;
        locals.var_arg_dn4 = assign103130_e155014_d_n4;
        locals.var_arg_dn5 = assign103130_e155014_d_n5;
        locals.var_arg_dn6 = assign103130_e155014_d_n6;
        locals.var_arg_dn7 = assign103130_e155014_d_n7;
        locals.var_arg_dn8 = assign103130_e155014_d_n8;
        locals.var_arg_dn9 = assign103130_e155014_d_n9;
        locals.var_arg_dn10 = assign103130_e155014_d_n10;
        locals.var_arg_dn11 = assign103130_e155014_d_n11;
        locals.var_arg_dn14 = assign103130_e155014_d_n14;

        let (assign103140_e155023, assign103140_e155023_d_n0, assign103140_e155023_d_n2, assign103140_e155023_d_n4, assign103140_e155023_d_n5, assign103140_e155023_d_n6, assign103140_e155023_d_n7, assign103140_e155023_d_n8, assign103140_e155023_d_n9, assign103140_e155023_d_n10, assign103140_e155023_d_n11, assign103140_e155023_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2346 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign103140_e155023;
        locals.var_dnm_dn0 = assign103140_e155023_d_n0;
        locals.var_dnm_dn2 = assign103140_e155023_d_n2;
        locals.var_dnm_dn4 = assign103140_e155023_d_n4;
        locals.var_dnm_dn5 = assign103140_e155023_d_n5;
        locals.var_dnm_dn6 = assign103140_e155023_d_n6;
        locals.var_dnm_dn7 = assign103140_e155023_d_n7;
        locals.var_dnm_dn8 = assign103140_e155023_d_n8;
        locals.var_dnm_dn9 = assign103140_e155023_d_n9;
        locals.var_dnm_dn10 = assign103140_e155023_d_n10;
        locals.var_dnm_dn11 = assign103140_e155023_d_n11;
        locals.var_dnm_dn14 = assign103140_e155023_d_n14;

        let assign103150_e155038: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2347 = assign103150_e155038;

        let assign103160_e155041: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2348 = assign103160_e155041;

        let (assign103170_e155054,) = {
    if (((((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2346 != 0.0)) && (locals.var_guard2347 != 0.0)) && (locals.var_guard2348 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign103170_e155054;

        let assign103180_e155057: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2349 = assign103180_e155057;

        let (assign103190_e155073,) = {
    if ((((((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2346 != 0.0)) && (locals.var_guard2347 != 0.0)) && (locals.var_guard2348 == 0.0)) && (locals.var_guard2349 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign103190_e155073;

        let assign103200_e155076: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2350 = assign103200_e155076;

    }

    pub(super) fn stamp_transient_block_377(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign103210_e155095,) = {
    if (((((((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2346 != 0.0)) && (locals.var_guard2347 != 0.0)) && (locals.var_guard2348 == 0.0)) && (locals.var_guard2349 == 0.0)) && (locals.var_guard2350 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign103210_e155095;

        let assign103220_e155098: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2351 = assign103220_e155098;

        let (assign103230_e155120,) = {
    if ((((((((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2346 != 0.0)) && (locals.var_guard2347 != 0.0)) && (locals.var_guard2348 == 0.0)) && (locals.var_guard2349 == 0.0)) && (locals.var_guard2350 == 0.0)) && (locals.var_guard2351 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign103230_e155120;

        let (assign103240_e155131,) = {
    if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2346 != 0.0)) && (locals.var_guard2347 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign103240_e155131;

        let mut assign103250_loop_guard: usize = 0;
        while {
            let assign103250_cond_e155143: f64 = if (((((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2346 != 0.0)) && (locals.var_guard2347 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign103250_cond_e155143 != 0.0
        } {
            assign103250_loop_guard += 1;
            assert!(assign103250_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign103250_body0_e155155, assign103250_body0_e155155_d_n0, assign103250_body0_e155155_d_n2, assign103250_body0_e155155_d_n4, assign103250_body0_e155155_d_n5, assign103250_body0_e155155_d_n6, assign103250_body0_e155155_d_n7, assign103250_body0_e155155_d_n8, assign103250_body0_e155155_d_n9, assign103250_body0_e155155_d_n10, assign103250_body0_e155155_d_n11, assign103250_body0_e155155_d_n14,) = {
    if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2346 != 0.0)) && (locals.var_guard2347 != 0.0)) {
        let assign103250_body0_e155153: f64 = (locals.var_dnm).sqrt();
        (assign103250_body0_e155153, (locals.var_dnm_dn0 / (2.0 * assign103250_body0_e155153)), (locals.var_dnm_dn2 / (2.0 * assign103250_body0_e155153)), (locals.var_dnm_dn4 / (2.0 * assign103250_body0_e155153)), (locals.var_dnm_dn5 / (2.0 * assign103250_body0_e155153)), (locals.var_dnm_dn6 / (2.0 * assign103250_body0_e155153)), (locals.var_dnm_dn7 / (2.0 * assign103250_body0_e155153)), (locals.var_dnm_dn8 / (2.0 * assign103250_body0_e155153)), (locals.var_dnm_dn9 / (2.0 * assign103250_body0_e155153)), (locals.var_dnm_dn10 / (2.0 * assign103250_body0_e155153)), (locals.var_dnm_dn11 / (2.0 * assign103250_body0_e155153)), (locals.var_dnm_dn14 / (2.0 * assign103250_body0_e155153)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign103250_body0_e155155;
            locals.var_dnm_dn0 = assign103250_body0_e155155_d_n0;
            locals.var_dnm_dn2 = assign103250_body0_e155155_d_n2;
            locals.var_dnm_dn4 = assign103250_body0_e155155_d_n4;
            locals.var_dnm_dn5 = assign103250_body0_e155155_d_n5;
            locals.var_dnm_dn6 = assign103250_body0_e155155_d_n6;
            locals.var_dnm_dn7 = assign103250_body0_e155155_d_n7;
            locals.var_dnm_dn8 = assign103250_body0_e155155_d_n8;
            locals.var_dnm_dn9 = assign103250_body0_e155155_d_n9;
            locals.var_dnm_dn10 = assign103250_body0_e155155_d_n10;
            locals.var_dnm_dn11 = assign103250_body0_e155155_d_n11;
            locals.var_dnm_dn14 = assign103250_body0_e155155_d_n14;
            let (assign103250_body1_e155168,) = {
    if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2346 != 0.0)) && (locals.var_guard2347 != 0.0)) {
        let assign103250_body1_e155166: f64 = (locals.var_m0 + 1.0);
        (assign103250_body1_e155166,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign103250_body1_e155168;
        }

        let (assign103260_e155191, assign103260_e155191_d_n0, assign103260_e155191_d_n2, assign103260_e155191_d_n4, assign103260_e155191_d_n5, assign103260_e155191_d_n6, assign103260_e155191_d_n7, assign103260_e155191_d_n8, assign103260_e155191_d_n9, assign103260_e155191_d_n10, assign103260_e155191_d_n11, assign103260_e155191_d_n14,) = {
    if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2346 != 0.0)) && (locals.var_guard2347 == 0.0)) {
        let (assign103260_e155189, assign103260_e155189_d_n0, assign103260_e155189_d_n2, assign103260_e155189_d_n4, assign103260_e155189_d_n5, assign103260_e155189_d_n6, assign103260_e155189_d_n7, assign103260_e155189_d_n8, assign103260_e155189_d_n9, assign103260_e155189_d_n10, assign103260_e155189_d_n11, assign103260_e155189_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign103260_e155186: f64 = (2.0 * 2.0);
                let assign103260_e155187: f64 = (1.0 / assign103260_e155186);
                let assign103260_e155188: f64 = (locals.var_dnm).powf(assign103260_e155187);
                (assign103260_e155188, if 0.0 == 0.0 && ((assign103260_e155187) as f64).is_finite() && ((assign103260_e155187) as f64).fract() == 0.0 { if assign103260_e155187 == 0.0 { 0.0 } else { (assign103260_e155187 * ((locals.var_dnm).powf(assign103260_e155187 - 1.0) * locals.var_dnm_dn0)) } } else { (assign103260_e155188 * (assign103260_e155187 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103260_e155187) as f64).is_finite() && ((assign103260_e155187) as f64).fract() == 0.0 { if assign103260_e155187 == 0.0 { 0.0 } else { (assign103260_e155187 * ((locals.var_dnm).powf(assign103260_e155187 - 1.0) * locals.var_dnm_dn2)) } } else { (assign103260_e155188 * (assign103260_e155187 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103260_e155187) as f64).is_finite() && ((assign103260_e155187) as f64).fract() == 0.0 { if assign103260_e155187 == 0.0 { 0.0 } else { (assign103260_e155187 * ((locals.var_dnm).powf(assign103260_e155187 - 1.0) * locals.var_dnm_dn4)) } } else { (assign103260_e155188 * (assign103260_e155187 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103260_e155187) as f64).is_finite() && ((assign103260_e155187) as f64).fract() == 0.0 { if assign103260_e155187 == 0.0 { 0.0 } else { (assign103260_e155187 * ((locals.var_dnm).powf(assign103260_e155187 - 1.0) * locals.var_dnm_dn5)) } } else { (assign103260_e155188 * (assign103260_e155187 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103260_e155187) as f64).is_finite() && ((assign103260_e155187) as f64).fract() == 0.0 { if assign103260_e155187 == 0.0 { 0.0 } else { (assign103260_e155187 * ((locals.var_dnm).powf(assign103260_e155187 - 1.0) * locals.var_dnm_dn6)) } } else { (assign103260_e155188 * (assign103260_e155187 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103260_e155187) as f64).is_finite() && ((assign103260_e155187) as f64).fract() == 0.0 { if assign103260_e155187 == 0.0 { 0.0 } else { (assign103260_e155187 * ((locals.var_dnm).powf(assign103260_e155187 - 1.0) * locals.var_dnm_dn7)) } } else { (assign103260_e155188 * (assign103260_e155187 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103260_e155187) as f64).is_finite() && ((assign103260_e155187) as f64).fract() == 0.0 { if assign103260_e155187 == 0.0 { 0.0 } else { (assign103260_e155187 * ((locals.var_dnm).powf(assign103260_e155187 - 1.0) * locals.var_dnm_dn8)) } } else { (assign103260_e155188 * (assign103260_e155187 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103260_e155187) as f64).is_finite() && ((assign103260_e155187) as f64).fract() == 0.0 { if assign103260_e155187 == 0.0 { 0.0 } else { (assign103260_e155187 * ((locals.var_dnm).powf(assign103260_e155187 - 1.0) * locals.var_dnm_dn9)) } } else { (assign103260_e155188 * (assign103260_e155187 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103260_e155187) as f64).is_finite() && ((assign103260_e155187) as f64).fract() == 0.0 { if assign103260_e155187 == 0.0 { 0.0 } else { (assign103260_e155187 * ((locals.var_dnm).powf(assign103260_e155187 - 1.0) * locals.var_dnm_dn10)) } } else { (assign103260_e155188 * (assign103260_e155187 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103260_e155187) as f64).is_finite() && ((assign103260_e155187) as f64).fract() == 0.0 { if assign103260_e155187 == 0.0 { 0.0 } else { (assign103260_e155187 * ((locals.var_dnm).powf(assign103260_e155187 - 1.0) * locals.var_dnm_dn11)) } } else { (assign103260_e155188 * (assign103260_e155187 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103260_e155187) as f64).is_finite() && ((assign103260_e155187) as f64).fract() == 0.0 { if assign103260_e155187 == 0.0 { 0.0 } else { (assign103260_e155187 * ((locals.var_dnm).powf(assign103260_e155187 - 1.0) * locals.var_dnm_dn14)) } } else { (assign103260_e155188 * (assign103260_e155187 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign103260_e155189, assign103260_e155189_d_n0, assign103260_e155189_d_n2, assign103260_e155189_d_n4, assign103260_e155189_d_n5, assign103260_e155189_d_n6, assign103260_e155189_d_n7, assign103260_e155189_d_n8, assign103260_e155189_d_n9, assign103260_e155189_d_n10, assign103260_e155189_d_n11, assign103260_e155189_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign103260_e155191;
        locals.var_dnm_dn0 = assign103260_e155191_d_n0;
        locals.var_dnm_dn2 = assign103260_e155191_d_n2;
        locals.var_dnm_dn4 = assign103260_e155191_d_n4;
        locals.var_dnm_dn5 = assign103260_e155191_d_n5;
        locals.var_dnm_dn6 = assign103260_e155191_d_n6;
        locals.var_dnm_dn7 = assign103260_e155191_d_n7;
        locals.var_dnm_dn8 = assign103260_e155191_d_n8;
        locals.var_dnm_dn9 = assign103260_e155191_d_n9;
        locals.var_dnm_dn10 = assign103260_e155191_d_n10;
        locals.var_dnm_dn11 = assign103260_e155191_d_n11;
        locals.var_dnm_dn14 = assign103260_e155191_d_n14;

        let (assign103270_e155202, assign103270_e155202_d_n0, assign103270_e155202_d_n2, assign103270_e155202_d_n4, assign103270_e155202_d_n5, assign103270_e155202_d_n6, assign103270_e155202_d_n7, assign103270_e155202_d_n8, assign103270_e155202_d_n9, assign103270_e155202_d_n10, assign103270_e155202_d_n11, assign103270_e155202_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2346 != 0.0)) {
        let assign103270_e155200: f64 = (1.0 / locals.var_dnm);
        (assign103270_e155200, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign103270_e155202;
        locals.var_dnm_dn0 = assign103270_e155202_d_n0;
        locals.var_dnm_dn2 = assign103270_e155202_d_n2;
        locals.var_dnm_dn4 = assign103270_e155202_d_n4;
        locals.var_dnm_dn5 = assign103270_e155202_d_n5;
        locals.var_dnm_dn6 = assign103270_e155202_d_n6;
        locals.var_dnm_dn7 = assign103270_e155202_d_n7;
        locals.var_dnm_dn8 = assign103270_e155202_d_n8;
        locals.var_dnm_dn9 = assign103270_e155202_d_n9;
        locals.var_dnm_dn10 = assign103270_e155202_d_n10;
        locals.var_dnm_dn11 = assign103270_e155202_d_n11;
        locals.var_dnm_dn14 = assign103270_e155202_d_n14;

        let (assign103280_e155215, assign103280_e155215_d_n0, assign103280_e155215_d_n2, assign103280_e155215_d_n4, assign103280_e155215_d_n5, assign103280_e155215_d_n6, assign103280_e155215_d_n7, assign103280_e155215_d_n8, assign103280_e155215_d_n9, assign103280_e155215_d_n10, assign103280_e155215_d_n11, assign103280_e155215_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2346 != 0.0)) {
        let assign103280_e155211: f64 = (locals.var_tmf1 * 1e-25);
        let assign103280_e155213: f64 = (assign103280_e155211 * locals.var_dnm);
        (assign103280_e155213, (((locals.var_tmf1_dn0 * 1e-25) * locals.var_dnm) + (assign103280_e155211 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-25) * locals.var_dnm) + (assign103280_e155211 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1e-25) * locals.var_dnm) + (assign103280_e155211 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1e-25) * locals.var_dnm) + (assign103280_e155211 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1e-25) * locals.var_dnm) + (assign103280_e155211 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-25) * locals.var_dnm) + (assign103280_e155211 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1e-25) * locals.var_dnm) + (assign103280_e155211 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1e-25) * locals.var_dnm) + (assign103280_e155211 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1e-25) * locals.var_dnm) + (assign103280_e155211 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 1e-25) * locals.var_dnm) + (assign103280_e155211 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 1e-25) * locals.var_dnm) + (assign103280_e155211 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign103280_e155215;
        locals.var_tmf0_dn0 = assign103280_e155215_d_n0;
        locals.var_tmf0_dn2 = assign103280_e155215_d_n2;
        locals.var_tmf0_dn4 = assign103280_e155215_d_n4;
        locals.var_tmf0_dn5 = assign103280_e155215_d_n5;
        locals.var_tmf0_dn6 = assign103280_e155215_d_n6;
        locals.var_tmf0_dn7 = assign103280_e155215_d_n7;
        locals.var_tmf0_dn8 = assign103280_e155215_d_n8;
        locals.var_tmf0_dn9 = assign103280_e155215_d_n9;
        locals.var_tmf0_dn10 = assign103280_e155215_d_n10;
        locals.var_tmf0_dn11 = assign103280_e155215_d_n11;
        locals.var_tmf0_dn14 = assign103280_e155215_d_n14;

        let (assign103290_e155230, assign103290_e155230_d_n0, assign103290_e155230_d_n2, assign103290_e155230_d_n4, assign103290_e155230_d_n5, assign103290_e155230_d_n6, assign103290_e155230_d_n7, assign103290_e155230_d_n8, assign103290_e155230_d_n9, assign103290_e155230_d_n10, assign103290_e155230_d_n11, assign103290_e155230_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2346 != 0.0)) {
        let assign103290_e155224: f64 = (1e-25 * locals.var_xmp);
        let assign103290_e155226: f64 = (assign103290_e155224 * locals.var_dnm);
        let assign103290_e155228: f64 = (assign103290_e155226 / locals.var_arg);
        (assign103290_e155228, ((((((1e-25 * locals.var_xmp_dn0) * locals.var_dnm) + (assign103290_e155224 * locals.var_dnm_dn0)) * locals.var_arg) - (assign103290_e155226 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn2) * locals.var_dnm) + (assign103290_e155224 * locals.var_dnm_dn2)) * locals.var_arg) - (assign103290_e155226 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn4) * locals.var_dnm) + (assign103290_e155224 * locals.var_dnm_dn4)) * locals.var_arg) - (assign103290_e155226 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn5) * locals.var_dnm) + (assign103290_e155224 * locals.var_dnm_dn5)) * locals.var_arg) - (assign103290_e155226 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn6) * locals.var_dnm) + (assign103290_e155224 * locals.var_dnm_dn6)) * locals.var_arg) - (assign103290_e155226 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn7) * locals.var_dnm) + (assign103290_e155224 * locals.var_dnm_dn7)) * locals.var_arg) - (assign103290_e155226 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn8) * locals.var_dnm) + (assign103290_e155224 * locals.var_dnm_dn8)) * locals.var_arg) - (assign103290_e155226 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn9) * locals.var_dnm) + (assign103290_e155224 * locals.var_dnm_dn9)) * locals.var_arg) - (assign103290_e155226 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn10) * locals.var_dnm) + (assign103290_e155224 * locals.var_dnm_dn10)) * locals.var_arg) - (assign103290_e155226 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn11) * locals.var_dnm) + (assign103290_e155224 * locals.var_dnm_dn11)) * locals.var_arg) - (assign103290_e155226 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn14) * locals.var_dnm) + (assign103290_e155224 * locals.var_dnm_dn14)) * locals.var_arg) - (assign103290_e155226 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign103290_e155230;
        locals.var_t0_dn0 = assign103290_e155230_d_n0;
        locals.var_t0_dn2 = assign103290_e155230_d_n2;
        locals.var_t0_dn4 = assign103290_e155230_d_n4;
        locals.var_t0_dn5 = assign103290_e155230_d_n5;
        locals.var_t0_dn6 = assign103290_e155230_d_n6;
        locals.var_t0_dn7 = assign103290_e155230_d_n7;
        locals.var_t0_dn8 = assign103290_e155230_d_n8;
        locals.var_t0_dn9 = assign103290_e155230_d_n9;
        locals.var_t0_dn10 = assign103290_e155230_d_n10;
        locals.var_t0_dn11 = assign103290_e155230_d_n11;
        locals.var_t0_dn14 = assign103290_e155230_d_n14;

        let (assign103300_e155243, assign103300_e155243_d_n0, assign103300_e155243_d_n2, assign103300_e155243_d_n4, assign103300_e155243_d_n5, assign103300_e155243_d_n6, assign103300_e155243_d_n7, assign103300_e155243_d_n8, assign103300_e155243_d_n9, assign103300_e155243_d_n10, assign103300_e155243_d_n11, assign103300_e155243_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2346 != 0.0)) {
        let assign103300_e155239: f64 = 1e-25;
        let assign103300_e155241: f64 = (assign103300_e155239 - locals.var_tmf0);
        (assign103300_e155241, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_gd_s, locals.var_gd_s_dn0, locals.var_gd_s_dn2, locals.var_gd_s_dn4, locals.var_gd_s_dn5, locals.var_gd_s_dn6, locals.var_gd_s_dn7, locals.var_gd_s_dn8, locals.var_gd_s_dn9, locals.var_gd_s_dn10, locals.var_gd_s_dn11, locals.var_gd_s_dn14,)
    }
};
        locals.var_gd_s = assign103300_e155243;
        locals.var_gd_s_dn0 = assign103300_e155243_d_n0;
        locals.var_gd_s_dn2 = assign103300_e155243_d_n2;
        locals.var_gd_s_dn4 = assign103300_e155243_d_n4;
        locals.var_gd_s_dn5 = assign103300_e155243_d_n5;
        locals.var_gd_s_dn6 = assign103300_e155243_d_n6;
        locals.var_gd_s_dn7 = assign103300_e155243_d_n7;
        locals.var_gd_s_dn8 = assign103300_e155243_d_n8;
        locals.var_gd_s_dn9 = assign103300_e155243_d_n9;
        locals.var_gd_s_dn10 = assign103300_e155243_d_n10;
        locals.var_gd_s_dn11 = assign103300_e155243_d_n11;
        locals.var_gd_s_dn14 = assign103300_e155243_d_n14;

        let (assign103310_e155252, assign103310_e155252_d_n0, assign103310_e155252_d_n2, assign103310_e155252_d_n4, assign103310_e155252_d_n5, assign103310_e155252_d_n6, assign103310_e155252_d_n7, assign103310_e155252_d_n8, assign103310_e155252_d_n9, assign103310_e155252_d_n10, assign103310_e155252_d_n11, assign103310_e155252_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2346 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign103310_e155252;
        locals.var_t0_dn0 = assign103310_e155252_d_n0;
        locals.var_t0_dn2 = assign103310_e155252_d_n2;
        locals.var_t0_dn4 = assign103310_e155252_d_n4;
        locals.var_t0_dn5 = assign103310_e155252_d_n5;
        locals.var_t0_dn6 = assign103310_e155252_d_n6;
        locals.var_t0_dn7 = assign103310_e155252_d_n7;
        locals.var_t0_dn8 = assign103310_e155252_d_n8;
        locals.var_t0_dn9 = assign103310_e155252_d_n9;
        locals.var_t0_dn10 = assign103310_e155252_d_n10;
        locals.var_t0_dn11 = assign103310_e155252_d_n11;
        locals.var_t0_dn14 = assign103310_e155252_d_n14;

        let (assign103320_e155262, assign103320_e155262_d_n0, assign103320_e155262_d_n2, assign103320_e155262_d_n4, assign103320_e155262_d_n5, assign103320_e155262_d_n6, assign103320_e155262_d_n7, assign103320_e155262_d_n8, assign103320_e155262_d_n9, assign103320_e155262_d_n10, assign103320_e155262_d_n11, assign103320_e155262_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2346 == 0.0)) {
        (locals.var_gd_s, locals.var_gd_s_dn0, locals.var_gd_s_dn2, locals.var_gd_s_dn4, locals.var_gd_s_dn5, locals.var_gd_s_dn6, locals.var_gd_s_dn7, locals.var_gd_s_dn8, locals.var_gd_s_dn9, locals.var_gd_s_dn10, locals.var_gd_s_dn11, locals.var_gd_s_dn14,)
    } else {
        (locals.var_gd_s, locals.var_gd_s_dn0, locals.var_gd_s_dn2, locals.var_gd_s_dn4, locals.var_gd_s_dn5, locals.var_gd_s_dn6, locals.var_gd_s_dn7, locals.var_gd_s_dn8, locals.var_gd_s_dn9, locals.var_gd_s_dn10, locals.var_gd_s_dn11, locals.var_gd_s_dn14,)
    }
};
        locals.var_gd_s = assign103320_e155262;
        locals.var_gd_s_dn0 = assign103320_e155262_d_n0;
        locals.var_gd_s_dn2 = assign103320_e155262_d_n2;
        locals.var_gd_s_dn4 = assign103320_e155262_d_n4;
        locals.var_gd_s_dn5 = assign103320_e155262_d_n5;
        locals.var_gd_s_dn6 = assign103320_e155262_d_n6;
        locals.var_gd_s_dn7 = assign103320_e155262_d_n7;
        locals.var_gd_s_dn8 = assign103320_e155262_d_n8;
        locals.var_gd_s_dn9 = assign103320_e155262_d_n9;
        locals.var_gd_s_dn10 = assign103320_e155262_d_n10;
        locals.var_gd_s_dn11 = assign103320_e155262_d_n11;
        locals.var_gd_s_dn14 = assign103320_e155262_d_n14;

        let (assign103330_e155272, assign103330_e155272_d_n0, assign103330_e155272_d_n2, assign103330_e155272_d_n4, assign103330_e155272_d_n5, assign103330_e155272_d_n6, assign103330_e155272_d_n7, assign103330_e155272_d_n8, assign103330_e155272_d_n9, assign103330_e155272_d_n10, assign103330_e155272_d_n11, assign103330_e155272_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2346 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign103330_e155272;
        locals.var_t0_dn0 = assign103330_e155272_d_n0;
        locals.var_t0_dn2 = assign103330_e155272_d_n2;
        locals.var_t0_dn4 = assign103330_e155272_d_n4;
        locals.var_t0_dn5 = assign103330_e155272_d_n5;
        locals.var_t0_dn6 = assign103330_e155272_d_n6;
        locals.var_t0_dn7 = assign103330_e155272_d_n7;
        locals.var_t0_dn8 = assign103330_e155272_d_n8;
        locals.var_t0_dn9 = assign103330_e155272_d_n9;
        locals.var_t0_dn10 = assign103330_e155272_d_n10;
        locals.var_t0_dn11 = assign103330_e155272_d_n11;
        locals.var_t0_dn14 = assign103330_e155272_d_n14;

        let (assign103340_e155281, assign103340_e155281_d_n0, assign103340_e155281_d_n2, assign103340_e155281_d_n4, assign103340_e155281_d_n5, assign103340_e155281_d_n6, assign103340_e155281_d_n7, assign103340_e155281_d_n8, assign103340_e155281_d_n9, assign103340_e155281_d_n10, assign103340_e155281_d_n11, assign103340_e155281_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) {
        let assign103340_e155279: f64 = (1.0 / locals.var_gd_s);
        (assign103340_e155279, (-(locals.var_gd_s_dn0 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn2 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn4 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn5 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn6 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn7 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn8 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn9 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn10 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn11 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn14 / (locals.var_gd_s * locals.var_gd_s))),)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn14,)
    }
};
        locals.var_rsd = assign103340_e155281;
        locals.var_rsd_dn0 = assign103340_e155281_d_n0;
        locals.var_rsd_dn2 = assign103340_e155281_d_n2;
        locals.var_rsd_dn4 = assign103340_e155281_d_n4;
        locals.var_rsd_dn5 = assign103340_e155281_d_n5;
        locals.var_rsd_dn6 = assign103340_e155281_d_n6;
        locals.var_rsd_dn7 = assign103340_e155281_d_n7;
        locals.var_rsd_dn8 = assign103340_e155281_d_n8;
        locals.var_rsd_dn9 = assign103340_e155281_d_n9;
        locals.var_rsd_dn10 = assign103340_e155281_d_n10;
        locals.var_rsd_dn11 = assign103340_e155281_d_n11;
        locals.var_rsd_dn14 = assign103340_e155281_d_n14;

        let (assign103350_e155290, assign103350_e155290_d_n0, assign103350_e155290_d_n2, assign103350_e155290_d_n4, assign103350_e155290_d_n5, assign103350_e155290_d_n6, assign103350_e155290_d_n7, assign103350_e155290_d_n8, assign103350_e155290_d_n9, assign103350_e155290_d_n10, assign103350_e155290_d_n11, assign103350_e155290_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) {
        let assign103350_e155288: f64 = (locals.var_rsd / locals.var_weffld_nf);
        (assign103350_e155288, (locals.var_rsd_dn0 / locals.var_weffld_nf), (locals.var_rsd_dn2 / locals.var_weffld_nf), (locals.var_rsd_dn4 / locals.var_weffld_nf), (locals.var_rsd_dn5 / locals.var_weffld_nf), (locals.var_rsd_dn6 / locals.var_weffld_nf), (locals.var_rsd_dn7 / locals.var_weffld_nf), (locals.var_rsd_dn8 / locals.var_weffld_nf), (locals.var_rsd_dn9 / locals.var_weffld_nf), (locals.var_rsd_dn10 / locals.var_weffld_nf), (locals.var_rsd_dn11 / locals.var_weffld_nf), (locals.var_rsd_dn14 / locals.var_weffld_nf),)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn14,)
    }
};
        locals.var_rsd = assign103350_e155290;
        locals.var_rsd_dn0 = assign103350_e155290_d_n0;
        locals.var_rsd_dn2 = assign103350_e155290_d_n2;
        locals.var_rsd_dn4 = assign103350_e155290_d_n4;
        locals.var_rsd_dn5 = assign103350_e155290_d_n5;
        locals.var_rsd_dn6 = assign103350_e155290_d_n6;
        locals.var_rsd_dn7 = assign103350_e155290_d_n7;
        locals.var_rsd_dn8 = assign103350_e155290_d_n8;
        locals.var_rsd_dn9 = assign103350_e155290_d_n9;
        locals.var_rsd_dn10 = assign103350_e155290_d_n10;
        locals.var_rsd_dn11 = assign103350_e155290_d_n11;
        locals.var_rsd_dn14 = assign103350_e155290_d_n14;

        let (assign103360_e155299, assign103360_e155299_d_n0, assign103360_e155299_d_n2, assign103360_e155299_d_n4, assign103360_e155299_d_n5, assign103360_e155299_d_n6, assign103360_e155299_d_n7, assign103360_e155299_d_n8, assign103360_e155299_d_n9, assign103360_e155299_d_n10, assign103360_e155299_d_n11, assign103360_e155299_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) {
        let assign103360_e155297: f64 = (locals.var_rsd + locals.var_rs0);
        (assign103360_e155297, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn14,)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn14,)
    }
};
        locals.var_rsd = assign103360_e155299;
        locals.var_rsd_dn0 = assign103360_e155299_d_n0;
        locals.var_rsd_dn2 = assign103360_e155299_d_n2;
        locals.var_rsd_dn4 = assign103360_e155299_d_n4;
        locals.var_rsd_dn5 = assign103360_e155299_d_n5;
        locals.var_rsd_dn6 = assign103360_e155299_d_n6;
        locals.var_rsd_dn7 = assign103360_e155299_d_n7;
        locals.var_rsd_dn8 = assign103360_e155299_d_n8;
        locals.var_rsd_dn9 = assign103360_e155299_d_n9;
        locals.var_rsd_dn10 = assign103360_e155299_d_n10;
        locals.var_rsd_dn11 = assign103360_e155299_d_n11;
        locals.var_rsd_dn14 = assign103360_e155299_d_n14;

        let assign103400_e155330: f64 = if locals.var_rsd < p.p444 { 1.0 } else { 0.0 };
        locals.var_guard2353 = assign103400_e155330;

        let (assign103410_e155339, assign103410_e155339_d_n0, assign103410_e155339_d_n2, assign103410_e155339_d_n4, assign103410_e155339_d_n5, assign103410_e155339_d_n6, assign103410_e155339_d_n7, assign103410_e155339_d_n8, assign103410_e155339_d_n9, assign103410_e155339_d_n10, assign103410_e155339_d_n11, assign103410_e155339_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) && (locals.var_guard2353 != 0.0)) {
        (p.p444, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn14,)
    }
};
        locals.var_rsd = assign103410_e155339;
        locals.var_rsd_dn0 = assign103410_e155339_d_n0;
        locals.var_rsd_dn2 = assign103410_e155339_d_n2;
        locals.var_rsd_dn4 = assign103410_e155339_d_n4;
        locals.var_rsd_dn5 = assign103410_e155339_d_n5;
        locals.var_rsd_dn6 = assign103410_e155339_d_n6;
        locals.var_rsd_dn7 = assign103410_e155339_d_n7;
        locals.var_rsd_dn8 = assign103410_e155339_d_n8;
        locals.var_rsd_dn9 = assign103410_e155339_d_n9;
        locals.var_rsd_dn10 = assign103410_e155339_d_n10;
        locals.var_rsd_dn11 = assign103410_e155339_d_n11;
        locals.var_rsd_dn14 = assign103410_e155339_d_n14;

        let (assign103420_e155348, assign103420_e155348_d_n0, assign103420_e155348_d_n2, assign103420_e155348_d_n4, assign103420_e155348_d_n5, assign103420_e155348_d_n6, assign103420_e155348_d_n7, assign103420_e155348_d_n8, assign103420_e155348_d_n9, assign103420_e155348_d_n10, assign103420_e155348_d_n11, assign103420_e155348_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) {
        let assign103420_e155346: f64 = (locals.var_rsd / locals.var_mfactor);
        (assign103420_e155346, (locals.var_rsd_dn0 / locals.var_mfactor), (locals.var_rsd_dn2 / locals.var_mfactor), (locals.var_rsd_dn4 / locals.var_mfactor), (locals.var_rsd_dn5 / locals.var_mfactor), (locals.var_rsd_dn6 / locals.var_mfactor), (locals.var_rsd_dn7 / locals.var_mfactor), (locals.var_rsd_dn8 / locals.var_mfactor), (locals.var_rsd_dn9 / locals.var_mfactor), (locals.var_rsd_dn10 / locals.var_mfactor), (locals.var_rsd_dn11 / locals.var_mfactor), (locals.var_rsd_dn14 / locals.var_mfactor),)
    } else {
        (locals.var_rsde, locals.var_rsde_dn0, locals.var_rsde_dn2, locals.var_rsde_dn4, locals.var_rsde_dn5, locals.var_rsde_dn6, locals.var_rsde_dn7, locals.var_rsde_dn8, locals.var_rsde_dn9, locals.var_rsde_dn10, locals.var_rsde_dn11, locals.var_rsde_dn14,)
    }
};
        locals.var_rsde = assign103420_e155348;
        locals.var_rsde_dn0 = assign103420_e155348_d_n0;
        locals.var_rsde_dn2 = assign103420_e155348_d_n2;
        locals.var_rsde_dn4 = assign103420_e155348_d_n4;
        locals.var_rsde_dn5 = assign103420_e155348_d_n5;
        locals.var_rsde_dn6 = assign103420_e155348_d_n6;
        locals.var_rsde_dn7 = assign103420_e155348_d_n7;
        locals.var_rsde_dn8 = assign103420_e155348_d_n8;
        locals.var_rsde_dn9 = assign103420_e155348_d_n9;
        locals.var_rsde_dn10 = assign103420_e155348_d_n10;
        locals.var_rsde_dn11 = assign103420_e155348_d_n11;
        locals.var_rsde_dn14 = assign103420_e155348_d_n14;

        let assign103430_e155351: f64 = if locals.var_flg_rd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2358 = assign103430_e155351;

        let (assign103440_e155358, assign103440_e155358_d_n6, assign103440_e155358_d_n8,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        (locals.var_vdsi, locals.var_vdsi_dn6, locals.var_vdsi_dn8,)
    } else {
        (locals.var_vds__blk2354, locals.var_vds__blk2354_dn6, locals.var_vds__blk2354_dn8,)
    }
};
        locals.var_vds__blk2354 = assign103440_e155358;
        locals.var_vds__blk2354_dn6 = assign103440_e155358_d_n6;
        locals.var_vds__blk2354_dn8 = assign103440_e155358_d_n8;

        let (assign103450_e155365, assign103450_e155365_d_n8, assign103450_e155365_d_n9,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        (locals.var_vbsi, locals.var_vbsi_dn8, locals.var_vbsi_dn9,)
    } else {
        (locals.var_vbs__blk2355, locals.var_vbs__blk2355_dn8, locals.var_vbs__blk2355_dn9,)
    }
};
        locals.var_vbs__blk2355 = assign103450_e155365;
        locals.var_vbs__blk2355_dn8 = assign103450_e155365_d_n8;
        locals.var_vbs__blk2355_dn9 = assign103450_e155365_d_n9;

        let assign103460_e155372: f64 = if ((p.p53 > 0.0) && (locals.var_uc_rth0 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2359 = assign103460_e155372;

        let (assign103470_e155388, assign103470_e155388_d_n0, assign103470_e155388_d_n2, assign103470_e155388_d_n4, assign103470_e155388_d_n5, assign103470_e155388_d_n6, assign103470_e155388_d_n7, assign103470_e155388_d_n8, assign103470_e155388_d_n9, assign103470_e155388_d_n10, assign103470_e155388_d_n11, assign103470_e155388_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2359 != 0.0)) {
        let (assign103470_e155386, assign103470_e155386_d_n0, assign103470_e155386_d_n2, assign103470_e155386_d_n4, assign103470_e155386_d_n5, assign103470_e155386_d_n6, assign103470_e155386_d_n7, assign103470_e155386_d_n8, assign103470_e155386_d_n9, assign103470_e155386_d_n10, assign103470_e155386_d_n11, assign103470_e155386_d_n14,) = {
            if (locals.var_tratio == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign103470_e155385: f64 = (locals.var_tratio).powf(p.p415);
                (assign103470_e155385, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn0)) } } else { (assign103470_e155385 * (p.p415 * (locals.var_tratio_dn0 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn2)) } } else { (assign103470_e155385 * (p.p415 * (locals.var_tratio_dn2 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn4)) } } else { (assign103470_e155385 * (p.p415 * (locals.var_tratio_dn4 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn5)) } } else { (assign103470_e155385 * (p.p415 * (locals.var_tratio_dn5 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn6)) } } else { (assign103470_e155385 * (p.p415 * (locals.var_tratio_dn6 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn7)) } } else { (assign103470_e155385 * (p.p415 * (locals.var_tratio_dn7 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn8)) } } else { (assign103470_e155385 * (p.p415 * (locals.var_tratio_dn8 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn9)) } } else { (assign103470_e155385 * (p.p415 * (locals.var_tratio_dn9 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn10)) } } else { (assign103470_e155385 * (p.p415 * (locals.var_tratio_dn10 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn11)) } } else { (assign103470_e155385 * (p.p415 * (locals.var_tratio_dn11 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn14)) } } else { (assign103470_e155385 * (p.p415 * (locals.var_tratio_dn14 / locals.var_tratio))) },)
            }
        };
        (assign103470_e155386, assign103470_e155386_d_n0, assign103470_e155386_d_n2, assign103470_e155386_d_n4, assign103470_e155386_d_n5, assign103470_e155386_d_n6, assign103470_e155386_d_n7, assign103470_e155386_d_n8, assign103470_e155386_d_n9, assign103470_e155386_d_n10, assign103470_e155386_d_n11, assign103470_e155386_d_n14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign103470_e155388;
        locals.var_t1_dn0 = assign103470_e155388_d_n0;
        locals.var_t1_dn2 = assign103470_e155388_d_n2;
        locals.var_t1_dn4 = assign103470_e155388_d_n4;
        locals.var_t1_dn5 = assign103470_e155388_d_n5;
        locals.var_t1_dn6 = assign103470_e155388_d_n6;
        locals.var_t1_dn7 = assign103470_e155388_d_n7;
        locals.var_t1_dn8 = assign103470_e155388_d_n8;
        locals.var_t1_dn9 = assign103470_e155388_d_n9;
        locals.var_t1_dn10 = assign103470_e155388_d_n10;
        locals.var_t1_dn11 = assign103470_e155388_d_n11;
        locals.var_t1_dn14 = assign103470_e155388_d_n14;

        let (assign103480_e155399, assign103480_e155399_d_n0, assign103480_e155399_d_n2, assign103480_e155399_d_n4, assign103480_e155399_d_n5, assign103480_e155399_d_n6, assign103480_e155399_d_n7, assign103480_e155399_d_n8, assign103480_e155399_d_n9, assign103480_e155399_d_n10, assign103480_e155399_d_n11, assign103480_e155399_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2359 != 0.0)) {
        let assign103480_e155397: f64 = (locals.var_mks_rdrmue / locals.var_t1);
        (assign103480_e155397, (-((locals.var_mks_rdrmue * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn14) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_rrdrmue, locals.var_rrdrmue_dn0, locals.var_rrdrmue_dn2, locals.var_rrdrmue_dn4, locals.var_rrdrmue_dn5, locals.var_rrdrmue_dn6, locals.var_rrdrmue_dn7, locals.var_rrdrmue_dn8, locals.var_rrdrmue_dn9, locals.var_rrdrmue_dn10, locals.var_rrdrmue_dn11, locals.var_rrdrmue_dn14,)
    }
};
        locals.var_rrdrmue = assign103480_e155399;
        locals.var_rrdrmue_dn0 = assign103480_e155399_d_n0;
        locals.var_rrdrmue_dn2 = assign103480_e155399_d_n2;
        locals.var_rrdrmue_dn4 = assign103480_e155399_d_n4;
        locals.var_rrdrmue_dn5 = assign103480_e155399_d_n5;
        locals.var_rrdrmue_dn6 = assign103480_e155399_d_n6;
        locals.var_rrdrmue_dn7 = assign103480_e155399_d_n7;
        locals.var_rrdrmue_dn8 = assign103480_e155399_d_n8;
        locals.var_rrdrmue_dn9 = assign103480_e155399_d_n9;
        locals.var_rrdrmue_dn10 = assign103480_e155399_d_n10;
        locals.var_rrdrmue_dn11 = assign103480_e155399_d_n11;
        locals.var_rrdrmue_dn14 = assign103480_e155399_d_n14;

        let (assign103490_e155424, assign103490_e155424_d_n0, assign103490_e155424_d_n2, assign103490_e155424_d_n4, assign103490_e155424_d_n5, assign103490_e155424_d_n6, assign103490_e155424_d_n7, assign103490_e155424_d_n8, assign103490_e155424_d_n9, assign103490_e155424_d_n10, assign103490_e155424_d_n11, assign103490_e155424_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2359 != 0.0)) {
        let assign103490_e155409: f64 = (0.4 * locals.var_tratio);
        let assign103490_e155410: f64 = (1.8 + assign103490_e155409);
        let assign103490_e155413: f64 = (0.1 * locals.var_tratio);
        let assign103490_e155415: f64 = (assign103490_e155413 * locals.var_tratio);
        let assign103490_e155416: f64 = (assign103490_e155410 + assign103490_e155415);
        let assign103490_e155420: f64 = (1.0 - locals.var_tratio);
        let assign103490_e155421: f64 = (p.p417 * assign103490_e155420);
        let assign103490_e155422: f64 = (assign103490_e155416 - assign103490_e155421);
        (assign103490_e155422, (((0.4 * locals.var_tratio_dn0) + (((0.1 * locals.var_tratio_dn0) * locals.var_tratio) + (assign103490_e155413 * locals.var_tratio_dn0))) - (p.p417 * (-locals.var_tratio_dn0))), (((0.4 * locals.var_tratio_dn2) + (((0.1 * locals.var_tratio_dn2) * locals.var_tratio) + (assign103490_e155413 * locals.var_tratio_dn2))) - (p.p417 * (-locals.var_tratio_dn2))), (((0.4 * locals.var_tratio_dn4) + (((0.1 * locals.var_tratio_dn4) * locals.var_tratio) + (assign103490_e155413 * locals.var_tratio_dn4))) - (p.p417 * (-locals.var_tratio_dn4))), (((0.4 * locals.var_tratio_dn5) + (((0.1 * locals.var_tratio_dn5) * locals.var_tratio) + (assign103490_e155413 * locals.var_tratio_dn5))) - (p.p417 * (-locals.var_tratio_dn5))), (((0.4 * locals.var_tratio_dn6) + (((0.1 * locals.var_tratio_dn6) * locals.var_tratio) + (assign103490_e155413 * locals.var_tratio_dn6))) - (p.p417 * (-locals.var_tratio_dn6))), (((0.4 * locals.var_tratio_dn7) + (((0.1 * locals.var_tratio_dn7) * locals.var_tratio) + (assign103490_e155413 * locals.var_tratio_dn7))) - (p.p417 * (-locals.var_tratio_dn7))), (((0.4 * locals.var_tratio_dn8) + (((0.1 * locals.var_tratio_dn8) * locals.var_tratio) + (assign103490_e155413 * locals.var_tratio_dn8))) - (p.p417 * (-locals.var_tratio_dn8))), (((0.4 * locals.var_tratio_dn9) + (((0.1 * locals.var_tratio_dn9) * locals.var_tratio) + (assign103490_e155413 * locals.var_tratio_dn9))) - (p.p417 * (-locals.var_tratio_dn9))), (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign103490_e155413 * locals.var_tratio_dn10))) - (p.p417 * (-locals.var_tratio_dn10))), (((0.4 * locals.var_tratio_dn11) + (((0.1 * locals.var_tratio_dn11) * locals.var_tratio) + (assign103490_e155413 * locals.var_tratio_dn11))) - (p.p417 * (-locals.var_tratio_dn11))), (((0.4 * locals.var_tratio_dn14) + (((0.1 * locals.var_tratio_dn14) * locals.var_tratio) + (assign103490_e155413 * locals.var_tratio_dn14))) - (p.p417 * (-locals.var_tratio_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign103490_e155424;
        locals.var_t0_dn0 = assign103490_e155424_d_n0;
        locals.var_t0_dn2 = assign103490_e155424_d_n2;
        locals.var_t0_dn4 = assign103490_e155424_d_n4;
        locals.var_t0_dn5 = assign103490_e155424_d_n5;
        locals.var_t0_dn6 = assign103490_e155424_d_n6;
        locals.var_t0_dn7 = assign103490_e155424_d_n7;
        locals.var_t0_dn8 = assign103490_e155424_d_n8;
        locals.var_t0_dn9 = assign103490_e155424_d_n9;
        locals.var_t0_dn10 = assign103490_e155424_d_n10;
        locals.var_t0_dn11 = assign103490_e155424_d_n11;
        locals.var_t0_dn14 = assign103490_e155424_d_n14;

        let (assign103500_e155435, assign103500_e155435_d_n0, assign103500_e155435_d_n2, assign103500_e155435_d_n4, assign103500_e155435_d_n5, assign103500_e155435_d_n6, assign103500_e155435_d_n7, assign103500_e155435_d_n8, assign103500_e155435_d_n9, assign103500_e155435_d_n10, assign103500_e155435_d_n11, assign103500_e155435_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2359 != 0.0)) {
        let assign103500_e155433: f64 = (locals.var_mks_rdrvmax / locals.var_t0);
        (assign103500_e155433, (-((locals.var_mks_rdrvmax * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_rrdrvmax, locals.var_rrdrvmax_dn0, locals.var_rrdrvmax_dn2, locals.var_rrdrvmax_dn4, locals.var_rrdrvmax_dn5, locals.var_rrdrvmax_dn6, locals.var_rrdrvmax_dn7, locals.var_rrdrvmax_dn8, locals.var_rrdrvmax_dn9, locals.var_rrdrvmax_dn10, locals.var_rrdrvmax_dn11, locals.var_rrdrvmax_dn14,)
    }
};
        locals.var_rrdrvmax = assign103500_e155435;
        locals.var_rrdrvmax_dn0 = assign103500_e155435_d_n0;
        locals.var_rrdrvmax_dn2 = assign103500_e155435_d_n2;
        locals.var_rrdrvmax_dn4 = assign103500_e155435_d_n4;
        locals.var_rrdrvmax_dn5 = assign103500_e155435_d_n5;
        locals.var_rrdrvmax_dn6 = assign103500_e155435_d_n6;
        locals.var_rrdrvmax_dn7 = assign103500_e155435_d_n7;
        locals.var_rrdrvmax_dn8 = assign103500_e155435_d_n8;
        locals.var_rrdrvmax_dn9 = assign103500_e155435_d_n9;
        locals.var_rrdrvmax_dn10 = assign103500_e155435_d_n10;
        locals.var_rrdrvmax_dn11 = assign103500_e155435_d_n11;
        locals.var_rrdrvmax_dn14 = assign103500_e155435_d_n14;

        let (assign103510_e155450, assign103510_e155450_d_n0, assign103510_e155450_d_n2, assign103510_e155450_d_n4, assign103510_e155450_d_n5, assign103510_e155450_d_n6, assign103510_e155450_d_n7, assign103510_e155450_d_n8, assign103510_e155450_d_n9, assign103510_e155450_d_n10, assign103510_e155450_d_n11, assign103510_e155450_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2359 != 0.0)) {
        let assign103510_e155446: f64 = (locals.var_ttemp - locals.var_ktnom);
        let assign103510_e155447: f64 = (p.p438 * assign103510_e155446);
        let assign103510_e155448: f64 = (locals.var_uc_rdrbb + assign103510_e155447);
        (assign103510_e155448, (locals.var_uc_rdrbb_dn0 + (p.p438 * locals.var_ttemp_dn0)), (locals.var_uc_rdrbb_dn2 + (p.p438 * locals.var_ttemp_dn2)), (locals.var_uc_rdrbb_dn4 + (p.p438 * locals.var_ttemp_dn4)), (locals.var_uc_rdrbb_dn5 + (p.p438 * locals.var_ttemp_dn5)), (locals.var_uc_rdrbb_dn6 + (p.p438 * locals.var_ttemp_dn6)), (locals.var_uc_rdrbb_dn7 + (p.p438 * locals.var_ttemp_dn7)), (locals.var_uc_rdrbb_dn8 + (p.p438 * locals.var_ttemp_dn8)), (locals.var_uc_rdrbb_dn9 + (p.p438 * locals.var_ttemp_dn9)), (locals.var_uc_rdrbb_dn10 + (p.p438 * locals.var_ttemp_dn10)), (locals.var_uc_rdrbb_dn11 + (p.p438 * locals.var_ttemp_dn11)), (locals.var_uc_rdrbb_dn14 + (p.p438 * locals.var_ttemp_dn14)),)
    } else {
        (locals.var_uc_rdrbb, locals.var_uc_rdrbb_dn0, locals.var_uc_rdrbb_dn2, locals.var_uc_rdrbb_dn4, locals.var_uc_rdrbb_dn5, locals.var_uc_rdrbb_dn6, locals.var_uc_rdrbb_dn7, locals.var_uc_rdrbb_dn8, locals.var_uc_rdrbb_dn9, locals.var_uc_rdrbb_dn10, locals.var_uc_rdrbb_dn11, locals.var_uc_rdrbb_dn14,)
    }
};
        locals.var_uc_rdrbb = assign103510_e155450;
        locals.var_uc_rdrbb_dn0 = assign103510_e155450_d_n0;
        locals.var_uc_rdrbb_dn2 = assign103510_e155450_d_n2;
        locals.var_uc_rdrbb_dn4 = assign103510_e155450_d_n4;
        locals.var_uc_rdrbb_dn5 = assign103510_e155450_d_n5;
        locals.var_uc_rdrbb_dn6 = assign103510_e155450_d_n6;
        locals.var_uc_rdrbb_dn7 = assign103510_e155450_d_n7;
        locals.var_uc_rdrbb_dn8 = assign103510_e155450_d_n8;
        locals.var_uc_rdrbb_dn9 = assign103510_e155450_d_n9;
        locals.var_uc_rdrbb_dn10 = assign103510_e155450_d_n10;
        locals.var_uc_rdrbb_dn11 = assign103510_e155450_d_n11;
        locals.var_uc_rdrbb_dn14 = assign103510_e155450_d_n14;

        let assign103530_e155458: f64 = if locals.var_uc_rdrbb < 0.1 { 1.0 } else { 0.0 };
        locals.var_guard2361 = assign103530_e155458;

    }

    pub(super) fn stamp_transient_block_378(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let (assign103540_e155469, assign103540_e155469_d_n0, assign103540_e155469_d_n2, assign103540_e155469_d_n4, assign103540_e155469_d_n5, assign103540_e155469_d_n6, assign103540_e155469_d_n7, assign103540_e155469_d_n8, assign103540_e155469_d_n9, assign103540_e155469_d_n10, assign103540_e155469_d_n11, assign103540_e155469_d_n14,) = {
    if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2359 != 0.0)) && (locals.var_guard2361 != 0.0)) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_rdrbb, locals.var_uc_rdrbb_dn0, locals.var_uc_rdrbb_dn2, locals.var_uc_rdrbb_dn4, locals.var_uc_rdrbb_dn5, locals.var_uc_rdrbb_dn6, locals.var_uc_rdrbb_dn7, locals.var_uc_rdrbb_dn8, locals.var_uc_rdrbb_dn9, locals.var_uc_rdrbb_dn10, locals.var_uc_rdrbb_dn11, locals.var_uc_rdrbb_dn14,)
    }
};
        locals.var_uc_rdrbb = assign103540_e155469;
        locals.var_uc_rdrbb_dn0 = assign103540_e155469_d_n0;
        locals.var_uc_rdrbb_dn2 = assign103540_e155469_d_n2;
        locals.var_uc_rdrbb_dn4 = assign103540_e155469_d_n4;
        locals.var_uc_rdrbb_dn5 = assign103540_e155469_d_n5;
        locals.var_uc_rdrbb_dn6 = assign103540_e155469_d_n6;
        locals.var_uc_rdrbb_dn7 = assign103540_e155469_d_n7;
        locals.var_uc_rdrbb_dn8 = assign103540_e155469_d_n8;
        locals.var_uc_rdrbb_dn9 = assign103540_e155469_d_n9;
        locals.var_uc_rdrbb_dn10 = assign103540_e155469_d_n10;
        locals.var_uc_rdrbb_dn11 = assign103540_e155469_d_n11;
        locals.var_uc_rdrbb_dn14 = assign103540_e155469_d_n14;

        let (assign103550_e155481, assign103550_e155481_d_n0, assign103550_e155481_d_n2, assign103550_e155481_d_n4, assign103550_e155481_d_n5, assign103550_e155481_d_n6, assign103550_e155481_d_n7, assign103550_e155481_d_n8, assign103550_e155481_d_n9, assign103550_e155481_d_n10, assign103550_e155481_d_n11, assign103550_e155481_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2359 == 0.0)) {
        let assign103550_e155477: f64 = ctx_temp;
        let assign103550_e155479: f64 = (assign103550_e155477 + p.p11);
        (assign103550_e155479, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn11, locals.var_ttemp_dn14,)
    }
};
        locals.var_ttemp = assign103550_e155481;
        locals.var_ttemp_dn0 = assign103550_e155481_d_n0;
        locals.var_ttemp_dn2 = assign103550_e155481_d_n2;
        locals.var_ttemp_dn4 = assign103550_e155481_d_n4;
        locals.var_ttemp_dn5 = assign103550_e155481_d_n5;
        locals.var_ttemp_dn6 = assign103550_e155481_d_n6;
        locals.var_ttemp_dn7 = assign103550_e155481_d_n7;
        locals.var_ttemp_dn8 = assign103550_e155481_d_n8;
        locals.var_ttemp_dn9 = assign103550_e155481_d_n9;
        locals.var_ttemp_dn10 = assign103550_e155481_d_n10;
        locals.var_ttemp_dn11 = assign103550_e155481_d_n11;
        locals.var_ttemp_dn14 = assign103550_e155481_d_n14;

        let (assign103560_e155490,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign103560_e155488: f64 = (locals.var_weff_ld * p.p7);
        (assign103560_e155488,)
    } else {
        (locals.var_weffld_nf,)
    }
};
        locals.var_weffld_nf = assign103560_e155490;

        let (assign103570_e155499,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign103570_e155497: f64 = (p.p67 + p.p68);
        (assign103570_e155497,)
    } else {
        (locals.var_ldrifte,)
    }
};
        locals.var_ldrifte = assign103570_e155499;

        let (assign103580_e155508,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign103580_e155506: f64 = (locals.var_uc_xldld + 1e-12);
        (assign103580_e155506,)
    } else {
        (locals.var_rd_xldld,)
    }
};
        locals.var_rd_xldld = assign103580_e155508;

        let (assign103590_e155515,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        (locals.var_uc_nover,)
    } else {
        (locals.var_noverd,)
    }
};
        locals.var_noverd = assign103590_e155515;

        let (assign103600_e155530, assign103600_e155530_d_n0, assign103600_e155530_d_n2, assign103600_e155530_d_n4, assign103600_e155530_d_n5, assign103600_e155530_d_n6, assign103600_e155530_d_n7, assign103600_e155530_d_n8, assign103600_e155530_d_n9, assign103600_e155530_d_n10, assign103600_e155530_d_n11, assign103600_e155530_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign103600_e155525: f64 = (p.p411 * locals.var_vbs__blk2355);
        let assign103600_e155526: f64 = (p.p410 - assign103600_e155525);
        let assign103600_e155527: f64 = (locals.var_vbs__blk2355 * assign103600_e155526);
        let assign103600_e155528: f64 = (1.0 + assign103600_e155527);
        (assign103600_e155528, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, ((locals.var_vbs__blk2355_dn8 * assign103600_e155526) + (locals.var_vbs__blk2355 * (-(p.p411 * locals.var_vbs__blk2355_dn8)))), ((locals.var_vbs__blk2355_dn9 * assign103600_e155526) + (locals.var_vbs__blk2355 * (-(p.p411 * locals.var_vbs__blk2355_dn9)))), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign103600_e155530;
        locals.var_t1_dn0 = assign103600_e155530_d_n0;
        locals.var_t1_dn2 = assign103600_e155530_d_n2;
        locals.var_t1_dn4 = assign103600_e155530_d_n4;
        locals.var_t1_dn5 = assign103600_e155530_d_n5;
        locals.var_t1_dn6 = assign103600_e155530_d_n6;
        locals.var_t1_dn7 = assign103600_e155530_d_n7;
        locals.var_t1_dn8 = assign103600_e155530_d_n8;
        locals.var_t1_dn9 = assign103600_e155530_d_n9;
        locals.var_t1_dn10 = assign103600_e155530_d_n10;
        locals.var_t1_dn11 = assign103600_e155530_d_n11;
        locals.var_t1_dn14 = assign103600_e155530_d_n14;

        let (assign103610_e155546, assign103610_e155546_d_n0, assign103610_e155546_d_n2, assign103610_e155546_d_n4, assign103610_e155546_d_n5, assign103610_e155546_d_n6, assign103610_e155546_d_n7, assign103610_e155546_d_n8, assign103610_e155546_d_n9, assign103610_e155546_d_n10, assign103610_e155546_d_n11, assign103610_e155546_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign103610_e155537: f64 = (locals.var_t1 * locals.var_t1);
        let assign103610_e155540: f64 = (4.0 * 0.1);
        let assign103610_e155542: f64 = (assign103610_e155540 * 0.1);
        let assign103610_e155543: f64 = (assign103610_e155537 + assign103610_e155542);
        let assign103610_e155544: f64 = (assign103610_e155543).sqrt();
        (assign103610_e155544, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign103610_e155544)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign103610_e155544)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign103610_e155544)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign103610_e155544)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign103610_e155544)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign103610_e155544)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign103610_e155544)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign103610_e155544)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign103610_e155544)), (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign103610_e155544)), (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) / (2.0 * assign103610_e155544)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign103610_e155546;
        locals.var_tmf2_dn0 = assign103610_e155546_d_n0;
        locals.var_tmf2_dn2 = assign103610_e155546_d_n2;
        locals.var_tmf2_dn4 = assign103610_e155546_d_n4;
        locals.var_tmf2_dn5 = assign103610_e155546_d_n5;
        locals.var_tmf2_dn6 = assign103610_e155546_d_n6;
        locals.var_tmf2_dn7 = assign103610_e155546_d_n7;
        locals.var_tmf2_dn8 = assign103610_e155546_d_n8;
        locals.var_tmf2_dn9 = assign103610_e155546_d_n9;
        locals.var_tmf2_dn10 = assign103610_e155546_d_n10;
        locals.var_tmf2_dn11 = assign103610_e155546_d_n11;
        locals.var_tmf2_dn14 = assign103610_e155546_d_n14;

        let (assign103620_e155559, assign103620_e155559_d_n0, assign103620_e155559_d_n2, assign103620_e155559_d_n4, assign103620_e155559_d_n5, assign103620_e155559_d_n6, assign103620_e155559_d_n7, assign103620_e155559_d_n8, assign103620_e155559_d_n9, assign103620_e155559_d_n10, assign103620_e155559_d_n11, assign103620_e155559_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign103620_e155555: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign103620_e155556: f64 = (1.0 + assign103620_e155555);
        let assign103620_e155557: f64 = (0.5 * assign103620_e155556);
        (assign103620_e155557, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn7 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn9 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn11 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn14 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign103620_e155559;
        locals.var_t2_dn0 = assign103620_e155559_d_n0;
        locals.var_t2_dn2 = assign103620_e155559_d_n2;
        locals.var_t2_dn4 = assign103620_e155559_d_n4;
        locals.var_t2_dn5 = assign103620_e155559_d_n5;
        locals.var_t2_dn6 = assign103620_e155559_d_n6;
        locals.var_t2_dn7 = assign103620_e155559_d_n7;
        locals.var_t2_dn8 = assign103620_e155559_d_n8;
        locals.var_t2_dn9 = assign103620_e155559_d_n9;
        locals.var_t2_dn10 = assign103620_e155559_d_n10;
        locals.var_t2_dn11 = assign103620_e155559_d_n11;
        locals.var_t2_dn14 = assign103620_e155559_d_n14;

        let (assign103630_e155570, assign103630_e155570_d_n0, assign103630_e155570_d_n2, assign103630_e155570_d_n4, assign103630_e155570_d_n5, assign103630_e155570_d_n6, assign103630_e155570_d_n7, assign103630_e155570_d_n8, assign103630_e155570_d_n9, assign103630_e155570_d_n10, assign103630_e155570_d_n11, assign103630_e155570_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign103630_e155567: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign103630_e155568: f64 = (0.5 * assign103630_e155567);
        (assign103630_e155568, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rdrmuevbs, locals.var_rdrmuevbs_dn0, locals.var_rdrmuevbs_dn2, locals.var_rdrmuevbs_dn4, locals.var_rdrmuevbs_dn5, locals.var_rdrmuevbs_dn6, locals.var_rdrmuevbs_dn7, locals.var_rdrmuevbs_dn8, locals.var_rdrmuevbs_dn9, locals.var_rdrmuevbs_dn10, locals.var_rdrmuevbs_dn11, locals.var_rdrmuevbs_dn14,)
    }
};
        locals.var_rdrmuevbs = assign103630_e155570;
        locals.var_rdrmuevbs_dn0 = assign103630_e155570_d_n0;
        locals.var_rdrmuevbs_dn2 = assign103630_e155570_d_n2;
        locals.var_rdrmuevbs_dn4 = assign103630_e155570_d_n4;
        locals.var_rdrmuevbs_dn5 = assign103630_e155570_d_n5;
        locals.var_rdrmuevbs_dn6 = assign103630_e155570_d_n6;
        locals.var_rdrmuevbs_dn7 = assign103630_e155570_d_n7;
        locals.var_rdrmuevbs_dn8 = assign103630_e155570_d_n8;
        locals.var_rdrmuevbs_dn9 = assign103630_e155570_d_n9;
        locals.var_rdrmuevbs_dn10 = assign103630_e155570_d_n10;
        locals.var_rdrmuevbs_dn11 = assign103630_e155570_d_n11;
        locals.var_rdrmuevbs_dn14 = assign103630_e155570_d_n14;

        let assign103640_e155573: f64 = if locals.var_rdrmuevbs < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2362 = assign103640_e155573;

        let (assign103650_e155582, assign103650_e155582_d_n0, assign103650_e155582_d_n2, assign103650_e155582_d_n4, assign103650_e155582_d_n5, assign103650_e155582_d_n6, assign103650_e155582_d_n7, assign103650_e155582_d_n8, assign103650_e155582_d_n9, assign103650_e155582_d_n10, assign103650_e155582_d_n11, assign103650_e155582_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2362 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdrmuevbs, locals.var_rdrmuevbs_dn0, locals.var_rdrmuevbs_dn2, locals.var_rdrmuevbs_dn4, locals.var_rdrmuevbs_dn5, locals.var_rdrmuevbs_dn6, locals.var_rdrmuevbs_dn7, locals.var_rdrmuevbs_dn8, locals.var_rdrmuevbs_dn9, locals.var_rdrmuevbs_dn10, locals.var_rdrmuevbs_dn11, locals.var_rdrmuevbs_dn14,)
    }
};
        locals.var_rdrmuevbs = assign103650_e155582;
        locals.var_rdrmuevbs_dn0 = assign103650_e155582_d_n0;
        locals.var_rdrmuevbs_dn2 = assign103650_e155582_d_n2;
        locals.var_rdrmuevbs_dn4 = assign103650_e155582_d_n4;
        locals.var_rdrmuevbs_dn5 = assign103650_e155582_d_n5;
        locals.var_rdrmuevbs_dn6 = assign103650_e155582_d_n6;
        locals.var_rdrmuevbs_dn7 = assign103650_e155582_d_n7;
        locals.var_rdrmuevbs_dn8 = assign103650_e155582_d_n8;
        locals.var_rdrmuevbs_dn9 = assign103650_e155582_d_n9;
        locals.var_rdrmuevbs_dn10 = assign103650_e155582_d_n10;
        locals.var_rdrmuevbs_dn11 = assign103650_e155582_d_n11;
        locals.var_rdrmuevbs_dn14 = assign103650_e155582_d_n14;

        let (assign103660_e155591, assign103660_e155591_d_n0, assign103660_e155591_d_n2, assign103660_e155591_d_n4, assign103660_e155591_d_n5, assign103660_e155591_d_n6, assign103660_e155591_d_n7, assign103660_e155591_d_n8, assign103660_e155591_d_n9, assign103660_e155591_d_n10, assign103660_e155591_d_n11, assign103660_e155591_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2362 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign103660_e155591;
        locals.var_t2_dn0 = assign103660_e155591_d_n0;
        locals.var_t2_dn2 = assign103660_e155591_d_n2;
        locals.var_t2_dn4 = assign103660_e155591_d_n4;
        locals.var_t2_dn5 = assign103660_e155591_d_n5;
        locals.var_t2_dn6 = assign103660_e155591_d_n6;
        locals.var_t2_dn7 = assign103660_e155591_d_n7;
        locals.var_t2_dn8 = assign103660_e155591_d_n8;
        locals.var_t2_dn9 = assign103660_e155591_d_n9;
        locals.var_t2_dn10 = assign103660_e155591_d_n10;
        locals.var_t2_dn11 = assign103660_e155591_d_n11;
        locals.var_t2_dn14 = assign103660_e155591_d_n14;

        let (assign103670_e155602, assign103670_e155602_d_n0, assign103670_e155602_d_n2, assign103670_e155602_d_n4, assign103670_e155602_d_n5, assign103670_e155602_d_n6, assign103670_e155602_d_n7, assign103670_e155602_d_n8, assign103670_e155602_d_n9, assign103670_e155602_d_n10, assign103670_e155602_d_n11, assign103670_e155602_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign103670_e155598: f64 = (locals.var_rrdrmue * locals.var_rdrmuele);
        let assign103670_e155600: f64 = (assign103670_e155598 * locals.var_rdrmuevbs);
        (assign103670_e155600, (((locals.var_rrdrmue_dn0 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103670_e155598 * locals.var_rdrmuevbs_dn0)), (((locals.var_rrdrmue_dn2 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103670_e155598 * locals.var_rdrmuevbs_dn2)), (((locals.var_rrdrmue_dn4 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103670_e155598 * locals.var_rdrmuevbs_dn4)), (((locals.var_rrdrmue_dn5 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103670_e155598 * locals.var_rdrmuevbs_dn5)), (((locals.var_rrdrmue_dn6 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103670_e155598 * locals.var_rdrmuevbs_dn6)), (((locals.var_rrdrmue_dn7 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103670_e155598 * locals.var_rdrmuevbs_dn7)), (((locals.var_rrdrmue_dn8 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103670_e155598 * locals.var_rdrmuevbs_dn8)), (((locals.var_rrdrmue_dn9 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103670_e155598 * locals.var_rdrmuevbs_dn9)), (((locals.var_rrdrmue_dn10 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103670_e155598 * locals.var_rdrmuevbs_dn10)), (((locals.var_rrdrmue_dn11 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103670_e155598 * locals.var_rdrmuevbs_dn11)), (((locals.var_rrdrmue_dn14 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103670_e155598 * locals.var_rdrmuevbs_dn14)),)
    } else {
        (locals.var_mu0, locals.var_mu0_dn0, locals.var_mu0_dn2, locals.var_mu0_dn4, locals.var_mu0_dn5, locals.var_mu0_dn6, locals.var_mu0_dn7, locals.var_mu0_dn8, locals.var_mu0_dn9, locals.var_mu0_dn10, locals.var_mu0_dn11, locals.var_mu0_dn14,)
    }
};
        locals.var_mu0 = assign103670_e155602;
        locals.var_mu0_dn0 = assign103670_e155602_d_n0;
        locals.var_mu0_dn2 = assign103670_e155602_d_n2;
        locals.var_mu0_dn4 = assign103670_e155602_d_n4;
        locals.var_mu0_dn5 = assign103670_e155602_d_n5;
        locals.var_mu0_dn6 = assign103670_e155602_d_n6;
        locals.var_mu0_dn7 = assign103670_e155602_d_n7;
        locals.var_mu0_dn8 = assign103670_e155602_d_n8;
        locals.var_mu0_dn9 = assign103670_e155602_d_n9;
        locals.var_mu0_dn10 = assign103670_e155602_d_n10;
        locals.var_mu0_dn11 = assign103670_e155602_d_n11;
        locals.var_mu0_dn14 = assign103670_e155602_d_n14;

        let (assign103680_e155615, assign103680_e155615_d_n0, assign103680_e155615_d_n2, assign103680_e155615_d_n4, assign103680_e155615_d_n5, assign103680_e155615_d_n6, assign103680_e155615_d_n7, assign103680_e155615_d_n8, assign103680_e155615_d_n9, assign103680_e155615_d_n10, assign103680_e155615_d_n11, assign103680_e155615_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign103680_e155609: f64 = (locals.var_rrdrvmax * locals.var_rdrvmaxwe);
        let assign103680_e155611: f64 = (assign103680_e155609 * locals.var_rdrvmaxle);
        let assign103680_e155613: f64 = (assign103680_e155611 + 1e-25);
        (assign103680_e155613, ((locals.var_rrdrvmax_dn0 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn2 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn4 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn5 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn6 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn7 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn8 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn9 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn10 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn11 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn14 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle),)
    } else {
        (locals.var_vmaxe__blk2357, locals.var_vmaxe__blk2357_dn0, locals.var_vmaxe__blk2357_dn2, locals.var_vmaxe__blk2357_dn4, locals.var_vmaxe__blk2357_dn5, locals.var_vmaxe__blk2357_dn6, locals.var_vmaxe__blk2357_dn7, locals.var_vmaxe__blk2357_dn8, locals.var_vmaxe__blk2357_dn9, locals.var_vmaxe__blk2357_dn10, locals.var_vmaxe__blk2357_dn11, locals.var_vmaxe__blk2357_dn14,)
    }
};
        locals.var_vmaxe__blk2357 = assign103680_e155615;
        locals.var_vmaxe__blk2357_dn0 = assign103680_e155615_d_n0;
        locals.var_vmaxe__blk2357_dn2 = assign103680_e155615_d_n2;
        locals.var_vmaxe__blk2357_dn4 = assign103680_e155615_d_n4;
        locals.var_vmaxe__blk2357_dn5 = assign103680_e155615_d_n5;
        locals.var_vmaxe__blk2357_dn6 = assign103680_e155615_d_n6;
        locals.var_vmaxe__blk2357_dn7 = assign103680_e155615_d_n7;
        locals.var_vmaxe__blk2357_dn8 = assign103680_e155615_d_n8;
        locals.var_vmaxe__blk2357_dn9 = assign103680_e155615_d_n9;
        locals.var_vmaxe__blk2357_dn10 = assign103680_e155615_d_n10;
        locals.var_vmaxe__blk2357_dn11 = assign103680_e155615_d_n11;
        locals.var_vmaxe__blk2357_dn14 = assign103680_e155615_d_n14;

        let (assign103690_e155622,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        (locals.var_uc_rdrcx,)
    } else {
        (locals.var_cx,)
    }
};
        locals.var_cx = assign103690_e155622;

        let (assign103700_e155629,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        (p.p421,)
    } else {
        (locals.var_car,)
    }
};
        locals.var_car = assign103700_e155629;

        let (assign103710_e155638, assign103710_e155638_d_n0, assign103710_e155638_d_n2, assign103710_e155638_d_n4, assign103710_e155638_d_n5, assign103710_e155638_d_n6, assign103710_e155638_d_n7, assign103710_e155638_d_n8, assign103710_e155638_d_n9, assign103710_e155638_d_n10, assign103710_e155638_d_n11, assign103710_e155638_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign103710_e155636: f64 = (locals.var_mu0 * 10000.0);
        (assign103710_e155636, (locals.var_mu0_dn0 * 10000.0), (locals.var_mu0_dn2 * 10000.0), (locals.var_mu0_dn4 * 10000.0), (locals.var_mu0_dn5 * 10000.0), (locals.var_mu0_dn6 * 10000.0), (locals.var_mu0_dn7 * 10000.0), (locals.var_mu0_dn8 * 10000.0), (locals.var_mu0_dn9 * 10000.0), (locals.var_mu0_dn10 * 10000.0), (locals.var_mu0_dn11 * 10000.0), (locals.var_mu0_dn14 * 10000.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign103710_e155638;
        locals.var_t1_dn0 = assign103710_e155638_d_n0;
        locals.var_t1_dn2 = assign103710_e155638_d_n2;
        locals.var_t1_dn4 = assign103710_e155638_d_n4;
        locals.var_t1_dn5 = assign103710_e155638_d_n5;
        locals.var_t1_dn6 = assign103710_e155638_d_n6;
        locals.var_t1_dn7 = assign103710_e155638_d_n7;
        locals.var_t1_dn8 = assign103710_e155638_d_n8;
        locals.var_t1_dn9 = assign103710_e155638_d_n9;
        locals.var_t1_dn10 = assign103710_e155638_d_n10;
        locals.var_t1_dn11 = assign103710_e155638_d_n11;
        locals.var_t1_dn14 = assign103710_e155638_d_n14;

        let (assign103720_e155647, assign103720_e155647_d_n0, assign103720_e155647_d_n2, assign103720_e155647_d_n4, assign103720_e155647_d_n5, assign103720_e155647_d_n6, assign103720_e155647_d_n7, assign103720_e155647_d_n8, assign103720_e155647_d_n9, assign103720_e155647_d_n10, assign103720_e155647_d_n11, assign103720_e155647_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign103720_e155645: f64 = (locals.var_vmaxe__blk2357 * 100.0);
        (assign103720_e155645, (locals.var_vmaxe__blk2357_dn0 * 100.0), (locals.var_vmaxe__blk2357_dn2 * 100.0), (locals.var_vmaxe__blk2357_dn4 * 100.0), (locals.var_vmaxe__blk2357_dn5 * 100.0), (locals.var_vmaxe__blk2357_dn6 * 100.0), (locals.var_vmaxe__blk2357_dn7 * 100.0), (locals.var_vmaxe__blk2357_dn8 * 100.0), (locals.var_vmaxe__blk2357_dn9 * 100.0), (locals.var_vmaxe__blk2357_dn10 * 100.0), (locals.var_vmaxe__blk2357_dn11 * 100.0), (locals.var_vmaxe__blk2357_dn14 * 100.0),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign103720_e155647;
        locals.var_t2_dn0 = assign103720_e155647_d_n0;
        locals.var_t2_dn2 = assign103720_e155647_d_n2;
        locals.var_t2_dn4 = assign103720_e155647_d_n4;
        locals.var_t2_dn5 = assign103720_e155647_d_n5;
        locals.var_t2_dn6 = assign103720_e155647_d_n6;
        locals.var_t2_dn7 = assign103720_e155647_d_n7;
        locals.var_t2_dn8 = assign103720_e155647_d_n8;
        locals.var_t2_dn9 = assign103720_e155647_d_n9;
        locals.var_t2_dn10 = assign103720_e155647_d_n10;
        locals.var_t2_dn11 = assign103720_e155647_d_n11;
        locals.var_t2_dn14 = assign103720_e155647_d_n14;

        let assign103750_e155668: f64 = if locals.var_vddp < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2365 = assign103750_e155668;

        let (assign103760_e155684, assign103760_e155684_d_n0, assign103760_e155684_d_n2, assign103760_e155684_d_n4, assign103760_e155684_d_n5, assign103760_e155684_d_n6, assign103760_e155684_d_n7, assign103760_e155684_d_n8, assign103760_e155684_d_n9, assign103760_e155684_d_n10, assign103760_e155684_d_n11, assign103760_e155684_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2365 != 0.0)) {
        let assign103760_e155677: f64 = (-locals.var_vddp);
        let assign103760_e155679: f64 = (assign103760_e155677 / 2.0);
        let assign103760_e155680: f64 = (2.0 * assign103760_e155679);
        let assign103760_e155682: f64 = (assign103760_e155680 / p.p262);
        (assign103760_e155682, ((2.0 * ((-locals.var_vddp_dn0) / 2.0)) / p.p262), 0.0, 0.0, 0.0, ((2.0 * ((-locals.var_vddp_dn6) / 2.0)) / p.p262), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign103760_e155684;
        locals.var_tmf1_dn0 = assign103760_e155684_d_n0;
        locals.var_tmf1_dn2 = assign103760_e155684_d_n2;
        locals.var_tmf1_dn4 = assign103760_e155684_d_n4;
        locals.var_tmf1_dn5 = assign103760_e155684_d_n5;
        locals.var_tmf1_dn6 = assign103760_e155684_d_n6;
        locals.var_tmf1_dn7 = assign103760_e155684_d_n7;
        locals.var_tmf1_dn8 = assign103760_e155684_d_n8;
        locals.var_tmf1_dn9 = assign103760_e155684_d_n9;
        locals.var_tmf1_dn10 = assign103760_e155684_d_n10;
        locals.var_tmf1_dn11 = assign103760_e155684_d_n11;
        locals.var_tmf1_dn14 = assign103760_e155684_d_n14;

        let (assign103770_e155729, assign103770_e155729_d_n0, assign103770_e155729_d_n2, assign103770_e155729_d_n4, assign103770_e155729_d_n5, assign103770_e155729_d_n6, assign103770_e155729_d_n7, assign103770_e155729_d_n8, assign103770_e155729_d_n9, assign103770_e155729_d_n10, assign103770_e155729_d_n11, assign103770_e155729_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2365 != 0.0)) {
        let assign103770_e155695: f64 = (1.0 / 2.0);
        let assign103770_e155699: f64 = (1.0 / 6.0);
        let assign103770_e155703: f64 = (1.0 / 24.0);
        let assign103770_e155707: f64 = (1.0 / 120.0);
        let assign103770_e155711: f64 = (1.0 / 720.0);
        let assign103770_e155715: f64 = (1.0 / 5040.0);
        let assign103770_e155716: f64 = (locals.var_tmf1 * assign103770_e155715);
        let assign103770_e155717: f64 = (assign103770_e155711 + assign103770_e155716);
        let assign103770_e155718: f64 = (locals.var_tmf1 * assign103770_e155717);
        let assign103770_e155719: f64 = (assign103770_e155707 + assign103770_e155718);
        let assign103770_e155720: f64 = (locals.var_tmf1 * assign103770_e155719);
        let assign103770_e155721: f64 = (assign103770_e155703 + assign103770_e155720);
        let assign103770_e155722: f64 = (locals.var_tmf1 * assign103770_e155721);
        let assign103770_e155723: f64 = (assign103770_e155699 + assign103770_e155722);
        let assign103770_e155724: f64 = (locals.var_tmf1 * assign103770_e155723);
        let assign103770_e155725: f64 = (assign103770_e155695 + assign103770_e155724);
        let assign103770_e155726: f64 = (locals.var_tmf1 * assign103770_e155725);
        let assign103770_e155727: f64 = (1.0 + assign103770_e155726);
        (assign103770_e155727, ((locals.var_tmf1_dn0 * assign103770_e155725) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103770_e155723) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103770_e155721) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103770_e155719) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103770_e155717) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign103770_e155715))))))))))), ((locals.var_tmf1_dn2 * assign103770_e155725) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103770_e155723) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103770_e155721) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103770_e155719) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103770_e155717) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign103770_e155715))))))))))), ((locals.var_tmf1_dn4 * assign103770_e155725) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103770_e155723) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103770_e155721) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103770_e155719) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103770_e155717) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign103770_e155715))))))))))), ((locals.var_tmf1_dn5 * assign103770_e155725) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103770_e155723) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103770_e155721) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103770_e155719) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103770_e155717) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign103770_e155715))))))))))), ((locals.var_tmf1_dn6 * assign103770_e155725) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103770_e155723) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103770_e155721) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103770_e155719) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103770_e155717) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign103770_e155715))))))))))), ((locals.var_tmf1_dn7 * assign103770_e155725) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103770_e155723) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103770_e155721) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103770_e155719) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103770_e155717) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign103770_e155715))))))))))), ((locals.var_tmf1_dn8 * assign103770_e155725) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103770_e155723) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103770_e155721) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103770_e155719) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103770_e155717) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign103770_e155715))))))))))), ((locals.var_tmf1_dn9 * assign103770_e155725) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103770_e155723) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103770_e155721) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103770_e155719) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103770_e155717) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign103770_e155715))))))))))), ((locals.var_tmf1_dn10 * assign103770_e155725) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103770_e155723) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103770_e155721) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103770_e155719) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103770_e155717) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign103770_e155715))))))))))), ((locals.var_tmf1_dn11 * assign103770_e155725) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103770_e155723) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103770_e155721) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103770_e155719) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103770_e155717) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign103770_e155715))))))))))), ((locals.var_tmf1_dn14 * assign103770_e155725) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103770_e155723) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103770_e155721) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103770_e155719) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103770_e155717) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign103770_e155715))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign103770_e155729;
        locals.var_tmf2_dn0 = assign103770_e155729_d_n0;
        locals.var_tmf2_dn2 = assign103770_e155729_d_n2;
        locals.var_tmf2_dn4 = assign103770_e155729_d_n4;
        locals.var_tmf2_dn5 = assign103770_e155729_d_n5;
        locals.var_tmf2_dn6 = assign103770_e155729_d_n6;
        locals.var_tmf2_dn7 = assign103770_e155729_d_n7;
        locals.var_tmf2_dn8 = assign103770_e155729_d_n8;
        locals.var_tmf2_dn9 = assign103770_e155729_d_n9;
        locals.var_tmf2_dn10 = assign103770_e155729_d_n10;
        locals.var_tmf2_dn11 = assign103770_e155729_d_n11;
        locals.var_tmf2_dn14 = assign103770_e155729_d_n14;

        let (assign103780_e155770, assign103780_e155770_d_n0, assign103780_e155770_d_n2, assign103780_e155770_d_n4, assign103780_e155770_d_n5, assign103780_e155770_d_n6, assign103780_e155770_d_n7, assign103780_e155770_d_n8, assign103780_e155770_d_n9, assign103780_e155770_d_n10, assign103780_e155770_d_n11, assign103780_e155770_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2365 != 0.0)) {
        let assign103780_e155738: f64 = (1.0 / 2.0);
        let assign103780_e155742: f64 = (1.0 / 3.0);
        let assign103780_e155746: f64 = (1.0 / 8.0);
        let assign103780_e155750: f64 = (1.0 / 30.0);
        let assign103780_e155754: f64 = (1.0 / 144.0);
        let assign103780_e155758: f64 = (1.0 / 840.0);
        let assign103780_e155759: f64 = (locals.var_tmf1 * assign103780_e155758);
        let assign103780_e155760: f64 = (assign103780_e155754 + assign103780_e155759);
        let assign103780_e155761: f64 = (locals.var_tmf1 * assign103780_e155760);
        let assign103780_e155762: f64 = (assign103780_e155750 + assign103780_e155761);
        let assign103780_e155763: f64 = (locals.var_tmf1 * assign103780_e155762);
        let assign103780_e155764: f64 = (assign103780_e155746 + assign103780_e155763);
        let assign103780_e155765: f64 = (locals.var_tmf1 * assign103780_e155764);
        let assign103780_e155766: f64 = (assign103780_e155742 + assign103780_e155765);
        let assign103780_e155767: f64 = (locals.var_tmf1 * assign103780_e155766);
        let assign103780_e155768: f64 = (assign103780_e155738 + assign103780_e155767);
        (assign103780_e155768, ((locals.var_tmf1_dn0 * assign103780_e155766) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103780_e155764) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103780_e155762) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103780_e155760) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign103780_e155758))))))))), ((locals.var_tmf1_dn2 * assign103780_e155766) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103780_e155764) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103780_e155762) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103780_e155760) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign103780_e155758))))))))), ((locals.var_tmf1_dn4 * assign103780_e155766) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103780_e155764) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103780_e155762) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103780_e155760) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign103780_e155758))))))))), ((locals.var_tmf1_dn5 * assign103780_e155766) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103780_e155764) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103780_e155762) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103780_e155760) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign103780_e155758))))))))), ((locals.var_tmf1_dn6 * assign103780_e155766) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103780_e155764) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103780_e155762) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103780_e155760) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign103780_e155758))))))))), ((locals.var_tmf1_dn7 * assign103780_e155766) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103780_e155764) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103780_e155762) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103780_e155760) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign103780_e155758))))))))), ((locals.var_tmf1_dn8 * assign103780_e155766) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103780_e155764) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103780_e155762) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103780_e155760) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign103780_e155758))))))))), ((locals.var_tmf1_dn9 * assign103780_e155766) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103780_e155764) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103780_e155762) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103780_e155760) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign103780_e155758))))))))), ((locals.var_tmf1_dn10 * assign103780_e155766) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103780_e155764) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103780_e155762) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103780_e155760) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign103780_e155758))))))))), ((locals.var_tmf1_dn11 * assign103780_e155766) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103780_e155764) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103780_e155762) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103780_e155760) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign103780_e155758))))))))), ((locals.var_tmf1_dn14 * assign103780_e155766) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103780_e155764) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103780_e155762) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103780_e155760) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign103780_e155758))))))))),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign103780_e155770;
        locals.var_tmf3_dn0 = assign103780_e155770_d_n0;
        locals.var_tmf3_dn2 = assign103780_e155770_d_n2;
        locals.var_tmf3_dn4 = assign103780_e155770_d_n4;
        locals.var_tmf3_dn5 = assign103780_e155770_d_n5;
        locals.var_tmf3_dn6 = assign103780_e155770_d_n6;
        locals.var_tmf3_dn7 = assign103780_e155770_d_n7;
        locals.var_tmf3_dn8 = assign103780_e155770_d_n8;
        locals.var_tmf3_dn9 = assign103780_e155770_d_n9;
        locals.var_tmf3_dn10 = assign103780_e155770_d_n10;
        locals.var_tmf3_dn11 = assign103780_e155770_d_n11;
        locals.var_tmf3_dn14 = assign103780_e155770_d_n14;

        let (assign103790_e155781, assign103790_e155781_d_n0, assign103790_e155781_d_n2, assign103790_e155781_d_n4, assign103790_e155781_d_n5, assign103790_e155781_d_n6, assign103790_e155781_d_n7, assign103790_e155781_d_n8, assign103790_e155781_d_n9, assign103790_e155781_d_n10, assign103790_e155781_d_n11, assign103790_e155781_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2365 != 0.0)) {
        let assign103790_e155779: f64 = (p.p262 / locals.var_tmf2);
        (assign103790_e155779, (-((p.p262 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn11, locals.var_vzadd_dn14,)
    }
};
        locals.var_vzadd = assign103790_e155781;
        locals.var_vzadd_dn0 = assign103790_e155781_d_n0;
        locals.var_vzadd_dn2 = assign103790_e155781_d_n2;
        locals.var_vzadd_dn4 = assign103790_e155781_d_n4;
        locals.var_vzadd_dn5 = assign103790_e155781_d_n5;
        locals.var_vzadd_dn6 = assign103790_e155781_d_n6;
        locals.var_vzadd_dn7 = assign103790_e155781_d_n7;
        locals.var_vzadd_dn8 = assign103790_e155781_d_n8;
        locals.var_vzadd_dn9 = assign103790_e155781_d_n9;
        locals.var_vzadd_dn10 = assign103790_e155781_d_n10;
        locals.var_vzadd_dn11 = assign103790_e155781_d_n11;
        locals.var_vzadd_dn14 = assign103790_e155781_d_n14;

        let (assign103800_e155797, assign103800_e155797_d_n0, assign103800_e155797_d_n2, assign103800_e155797_d_n4, assign103800_e155797_d_n5, assign103800_e155797_d_n6, assign103800_e155797_d_n7, assign103800_e155797_d_n8, assign103800_e155797_d_n9, assign103800_e155797_d_n10, assign103800_e155797_d_n11, assign103800_e155797_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2365 != 0.0)) {
        let assign103800_e155789: f64 = (-2.0);
        let assign103800_e155791: f64 = (assign103800_e155789 * locals.var_tmf3);
        let assign103800_e155794: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign103800_e155795: f64 = (assign103800_e155791 / assign103800_e155794);
        (assign103800_e155795, ((((assign103800_e155789 * locals.var_tmf3_dn0) * assign103800_e155794) - (assign103800_e155791 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign103800_e155794 * assign103800_e155794)), ((((assign103800_e155789 * locals.var_tmf3_dn2) * assign103800_e155794) - (assign103800_e155791 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign103800_e155794 * assign103800_e155794)), ((((assign103800_e155789 * locals.var_tmf3_dn4) * assign103800_e155794) - (assign103800_e155791 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign103800_e155794 * assign103800_e155794)), ((((assign103800_e155789 * locals.var_tmf3_dn5) * assign103800_e155794) - (assign103800_e155791 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign103800_e155794 * assign103800_e155794)), ((((assign103800_e155789 * locals.var_tmf3_dn6) * assign103800_e155794) - (assign103800_e155791 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign103800_e155794 * assign103800_e155794)), ((((assign103800_e155789 * locals.var_tmf3_dn7) * assign103800_e155794) - (assign103800_e155791 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign103800_e155794 * assign103800_e155794)), ((((assign103800_e155789 * locals.var_tmf3_dn8) * assign103800_e155794) - (assign103800_e155791 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign103800_e155794 * assign103800_e155794)), ((((assign103800_e155789 * locals.var_tmf3_dn9) * assign103800_e155794) - (assign103800_e155791 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign103800_e155794 * assign103800_e155794)), ((((assign103800_e155789 * locals.var_tmf3_dn10) * assign103800_e155794) - (assign103800_e155791 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign103800_e155794 * assign103800_e155794)), ((((assign103800_e155789 * locals.var_tmf3_dn11) * assign103800_e155794) - (assign103800_e155791 * ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)))) / (assign103800_e155794 * assign103800_e155794)), ((((assign103800_e155789 * locals.var_tmf3_dn14) * assign103800_e155794) - (assign103800_e155791 * ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)))) / (assign103800_e155794 * assign103800_e155794)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign103800_e155797;
        locals.var_t2_dn0 = assign103800_e155797_d_n0;
        locals.var_t2_dn2 = assign103800_e155797_d_n2;
        locals.var_t2_dn4 = assign103800_e155797_d_n4;
        locals.var_t2_dn5 = assign103800_e155797_d_n5;
        locals.var_t2_dn6 = assign103800_e155797_d_n6;
        locals.var_t2_dn7 = assign103800_e155797_d_n7;
        locals.var_t2_dn8 = assign103800_e155797_d_n8;
        locals.var_t2_dn9 = assign103800_e155797_d_n9;
        locals.var_t2_dn10 = assign103800_e155797_d_n10;
        locals.var_t2_dn11 = assign103800_e155797_d_n11;
        locals.var_t2_dn14 = assign103800_e155797_d_n14;

        let assign103810_e155800: f64 = if locals.var_vzadd < 1e-12 { 1.0 } else { 0.0 };
        locals.var_guard2366 = assign103810_e155800;

        let (assign103820_e155811, assign103820_e155811_d_n0, assign103820_e155811_d_n2, assign103820_e155811_d_n4, assign103820_e155811_d_n5, assign103820_e155811_d_n6, assign103820_e155811_d_n7, assign103820_e155811_d_n8, assign103820_e155811_d_n9, assign103820_e155811_d_n10, assign103820_e155811_d_n11, assign103820_e155811_d_n14,) = {
    if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2365 != 0.0)) && (locals.var_guard2366 != 0.0)) {
        (1e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn11, locals.var_vzadd_dn14,)
    }
};
        locals.var_vzadd = assign103820_e155811;
        locals.var_vzadd_dn0 = assign103820_e155811_d_n0;
        locals.var_vzadd_dn2 = assign103820_e155811_d_n2;
        locals.var_vzadd_dn4 = assign103820_e155811_d_n4;
        locals.var_vzadd_dn5 = assign103820_e155811_d_n5;
        locals.var_vzadd_dn6 = assign103820_e155811_d_n6;
        locals.var_vzadd_dn7 = assign103820_e155811_d_n7;
        locals.var_vzadd_dn8 = assign103820_e155811_d_n8;
        locals.var_vzadd_dn9 = assign103820_e155811_d_n9;
        locals.var_vzadd_dn10 = assign103820_e155811_d_n10;
        locals.var_vzadd_dn11 = assign103820_e155811_d_n11;
        locals.var_vzadd_dn14 = assign103820_e155811_d_n14;

    }

    pub(super) fn stamp_transient_block_379(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign103830_e155824, assign103830_e155824_d_n0, assign103830_e155824_d_n2, assign103830_e155824_d_n4, assign103830_e155824_d_n5, assign103830_e155824_d_n6, assign103830_e155824_d_n7, assign103830_e155824_d_n8, assign103830_e155824_d_n9, assign103830_e155824_d_n10, assign103830_e155824_d_n11, assign103830_e155824_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2365 != 0.0)) {
        let assign103830_e155821: f64 = (2.0 * locals.var_vzadd);
        let assign103830_e155822: f64 = (locals.var_vddp - assign103830_e155821);
        (assign103830_e155822, (locals.var_vddp_dn0 - (2.0 * locals.var_vzadd_dn0)), (-(2.0 * locals.var_vzadd_dn2)), (-(2.0 * locals.var_vzadd_dn4)), (-(2.0 * locals.var_vzadd_dn5)), (locals.var_vddp_dn6 - (2.0 * locals.var_vzadd_dn6)), (-(2.0 * locals.var_vzadd_dn7)), (-(2.0 * locals.var_vzadd_dn8)), (-(2.0 * locals.var_vzadd_dn9)), (-(2.0 * locals.var_vzadd_dn10)), (-(2.0 * locals.var_vzadd_dn11)), (-(2.0 * locals.var_vzadd_dn14)),)
    } else {
        (locals.var_vddpz, locals.var_vddpz_dn0, locals.var_vddpz_dn2, locals.var_vddpz_dn4, locals.var_vddpz_dn5, locals.var_vddpz_dn6, locals.var_vddpz_dn7, locals.var_vddpz_dn8, locals.var_vddpz_dn9, locals.var_vddpz_dn10, locals.var_vddpz_dn11, locals.var_vddpz_dn14,)
    }
};
        locals.var_vddpz = assign103830_e155824;
        locals.var_vddpz_dn0 = assign103830_e155824_d_n0;
        locals.var_vddpz_dn2 = assign103830_e155824_d_n2;
        locals.var_vddpz_dn4 = assign103830_e155824_d_n4;
        locals.var_vddpz_dn5 = assign103830_e155824_d_n5;
        locals.var_vddpz_dn6 = assign103830_e155824_d_n6;
        locals.var_vddpz_dn7 = assign103830_e155824_d_n7;
        locals.var_vddpz_dn8 = assign103830_e155824_d_n8;
        locals.var_vddpz_dn9 = assign103830_e155824_d_n9;
        locals.var_vddpz_dn10 = assign103830_e155824_d_n10;
        locals.var_vddpz_dn11 = assign103830_e155824_d_n11;
        locals.var_vddpz_dn14 = assign103830_e155824_d_n14;

        let (assign103840_e155840, assign103840_e155840_d_n0, assign103840_e155840_d_n2, assign103840_e155840_d_n4, assign103840_e155840_d_n5, assign103840_e155840_d_n6, assign103840_e155840_d_n7, assign103840_e155840_d_n8, assign103840_e155840_d_n9, assign103840_e155840_d_n10, assign103840_e155840_d_n11, assign103840_e155840_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2365 == 0.0)) {
        let assign103840_e155835: f64 = (locals.var_vddp / 2.0);
        let assign103840_e155836: f64 = (2.0 * assign103840_e155835);
        let assign103840_e155838: f64 = (assign103840_e155836 / p.p262);
        (assign103840_e155838, ((2.0 * (locals.var_vddp_dn0 / 2.0)) / p.p262), 0.0, 0.0, 0.0, ((2.0 * (locals.var_vddp_dn6 / 2.0)) / p.p262), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign103840_e155840;
        locals.var_tmf1_dn0 = assign103840_e155840_d_n0;
        locals.var_tmf1_dn2 = assign103840_e155840_d_n2;
        locals.var_tmf1_dn4 = assign103840_e155840_d_n4;
        locals.var_tmf1_dn5 = assign103840_e155840_d_n5;
        locals.var_tmf1_dn6 = assign103840_e155840_d_n6;
        locals.var_tmf1_dn7 = assign103840_e155840_d_n7;
        locals.var_tmf1_dn8 = assign103840_e155840_d_n8;
        locals.var_tmf1_dn9 = assign103840_e155840_d_n9;
        locals.var_tmf1_dn10 = assign103840_e155840_d_n10;
        locals.var_tmf1_dn11 = assign103840_e155840_d_n11;
        locals.var_tmf1_dn14 = assign103840_e155840_d_n14;

        let (assign103850_e155886, assign103850_e155886_d_n0, assign103850_e155886_d_n2, assign103850_e155886_d_n4, assign103850_e155886_d_n5, assign103850_e155886_d_n6, assign103850_e155886_d_n7, assign103850_e155886_d_n8, assign103850_e155886_d_n9, assign103850_e155886_d_n10, assign103850_e155886_d_n11, assign103850_e155886_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2365 == 0.0)) {
        let assign103850_e155852: f64 = (1.0 / 2.0);
        let assign103850_e155856: f64 = (1.0 / 6.0);
        let assign103850_e155860: f64 = (1.0 / 24.0);
        let assign103850_e155864: f64 = (1.0 / 120.0);
        let assign103850_e155868: f64 = (1.0 / 720.0);
        let assign103850_e155872: f64 = (1.0 / 5040.0);
        let assign103850_e155873: f64 = (locals.var_tmf1 * assign103850_e155872);
        let assign103850_e155874: f64 = (assign103850_e155868 + assign103850_e155873);
        let assign103850_e155875: f64 = (locals.var_tmf1 * assign103850_e155874);
        let assign103850_e155876: f64 = (assign103850_e155864 + assign103850_e155875);
        let assign103850_e155877: f64 = (locals.var_tmf1 * assign103850_e155876);
        let assign103850_e155878: f64 = (assign103850_e155860 + assign103850_e155877);
        let assign103850_e155879: f64 = (locals.var_tmf1 * assign103850_e155878);
        let assign103850_e155880: f64 = (assign103850_e155856 + assign103850_e155879);
        let assign103850_e155881: f64 = (locals.var_tmf1 * assign103850_e155880);
        let assign103850_e155882: f64 = (assign103850_e155852 + assign103850_e155881);
        let assign103850_e155883: f64 = (locals.var_tmf1 * assign103850_e155882);
        let assign103850_e155884: f64 = (1.0 + assign103850_e155883);
        (assign103850_e155884, ((locals.var_tmf1_dn0 * assign103850_e155882) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103850_e155880) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103850_e155878) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103850_e155876) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103850_e155874) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign103850_e155872))))))))))), ((locals.var_tmf1_dn2 * assign103850_e155882) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103850_e155880) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103850_e155878) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103850_e155876) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103850_e155874) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign103850_e155872))))))))))), ((locals.var_tmf1_dn4 * assign103850_e155882) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103850_e155880) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103850_e155878) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103850_e155876) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103850_e155874) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign103850_e155872))))))))))), ((locals.var_tmf1_dn5 * assign103850_e155882) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103850_e155880) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103850_e155878) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103850_e155876) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103850_e155874) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign103850_e155872))))))))))), ((locals.var_tmf1_dn6 * assign103850_e155882) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103850_e155880) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103850_e155878) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103850_e155876) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103850_e155874) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign103850_e155872))))))))))), ((locals.var_tmf1_dn7 * assign103850_e155882) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103850_e155880) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103850_e155878) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103850_e155876) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103850_e155874) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign103850_e155872))))))))))), ((locals.var_tmf1_dn8 * assign103850_e155882) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103850_e155880) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103850_e155878) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103850_e155876) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103850_e155874) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign103850_e155872))))))))))), ((locals.var_tmf1_dn9 * assign103850_e155882) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103850_e155880) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103850_e155878) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103850_e155876) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103850_e155874) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign103850_e155872))))))))))), ((locals.var_tmf1_dn10 * assign103850_e155882) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103850_e155880) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103850_e155878) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103850_e155876) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103850_e155874) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign103850_e155872))))))))))), ((locals.var_tmf1_dn11 * assign103850_e155882) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103850_e155880) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103850_e155878) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103850_e155876) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103850_e155874) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign103850_e155872))))))))))), ((locals.var_tmf1_dn14 * assign103850_e155882) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103850_e155880) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103850_e155878) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103850_e155876) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103850_e155874) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign103850_e155872))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign103850_e155886;
        locals.var_tmf2_dn0 = assign103850_e155886_d_n0;
        locals.var_tmf2_dn2 = assign103850_e155886_d_n2;
        locals.var_tmf2_dn4 = assign103850_e155886_d_n4;
        locals.var_tmf2_dn5 = assign103850_e155886_d_n5;
        locals.var_tmf2_dn6 = assign103850_e155886_d_n6;
        locals.var_tmf2_dn7 = assign103850_e155886_d_n7;
        locals.var_tmf2_dn8 = assign103850_e155886_d_n8;
        locals.var_tmf2_dn9 = assign103850_e155886_d_n9;
        locals.var_tmf2_dn10 = assign103850_e155886_d_n10;
        locals.var_tmf2_dn11 = assign103850_e155886_d_n11;
        locals.var_tmf2_dn14 = assign103850_e155886_d_n14;

        let (assign103860_e155928, assign103860_e155928_d_n0, assign103860_e155928_d_n2, assign103860_e155928_d_n4, assign103860_e155928_d_n5, assign103860_e155928_d_n6, assign103860_e155928_d_n7, assign103860_e155928_d_n8, assign103860_e155928_d_n9, assign103860_e155928_d_n10, assign103860_e155928_d_n11, assign103860_e155928_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2365 == 0.0)) {
        let assign103860_e155896: f64 = (1.0 / 2.0);
        let assign103860_e155900: f64 = (1.0 / 3.0);
        let assign103860_e155904: f64 = (1.0 / 8.0);
        let assign103860_e155908: f64 = (1.0 / 30.0);
        let assign103860_e155912: f64 = (1.0 / 144.0);
        let assign103860_e155916: f64 = (1.0 / 840.0);
        let assign103860_e155917: f64 = (locals.var_tmf1 * assign103860_e155916);
        let assign103860_e155918: f64 = (assign103860_e155912 + assign103860_e155917);
        let assign103860_e155919: f64 = (locals.var_tmf1 * assign103860_e155918);
        let assign103860_e155920: f64 = (assign103860_e155908 + assign103860_e155919);
        let assign103860_e155921: f64 = (locals.var_tmf1 * assign103860_e155920);
        let assign103860_e155922: f64 = (assign103860_e155904 + assign103860_e155921);
        let assign103860_e155923: f64 = (locals.var_tmf1 * assign103860_e155922);
        let assign103860_e155924: f64 = (assign103860_e155900 + assign103860_e155923);
        let assign103860_e155925: f64 = (locals.var_tmf1 * assign103860_e155924);
        let assign103860_e155926: f64 = (assign103860_e155896 + assign103860_e155925);
        (assign103860_e155926, ((locals.var_tmf1_dn0 * assign103860_e155924) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103860_e155922) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103860_e155920) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103860_e155918) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign103860_e155916))))))))), ((locals.var_tmf1_dn2 * assign103860_e155924) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103860_e155922) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103860_e155920) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103860_e155918) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign103860_e155916))))))))), ((locals.var_tmf1_dn4 * assign103860_e155924) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103860_e155922) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103860_e155920) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103860_e155918) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign103860_e155916))))))))), ((locals.var_tmf1_dn5 * assign103860_e155924) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103860_e155922) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103860_e155920) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103860_e155918) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign103860_e155916))))))))), ((locals.var_tmf1_dn6 * assign103860_e155924) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103860_e155922) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103860_e155920) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103860_e155918) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign103860_e155916))))))))), ((locals.var_tmf1_dn7 * assign103860_e155924) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103860_e155922) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103860_e155920) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103860_e155918) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign103860_e155916))))))))), ((locals.var_tmf1_dn8 * assign103860_e155924) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103860_e155922) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103860_e155920) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103860_e155918) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign103860_e155916))))))))), ((locals.var_tmf1_dn9 * assign103860_e155924) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103860_e155922) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103860_e155920) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103860_e155918) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign103860_e155916))))))))), ((locals.var_tmf1_dn10 * assign103860_e155924) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103860_e155922) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103860_e155920) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103860_e155918) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign103860_e155916))))))))), ((locals.var_tmf1_dn11 * assign103860_e155924) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103860_e155922) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103860_e155920) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103860_e155918) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign103860_e155916))))))))), ((locals.var_tmf1_dn14 * assign103860_e155924) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103860_e155922) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103860_e155920) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103860_e155918) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign103860_e155916))))))))),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign103860_e155928;
        locals.var_tmf3_dn0 = assign103860_e155928_d_n0;
        locals.var_tmf3_dn2 = assign103860_e155928_d_n2;
        locals.var_tmf3_dn4 = assign103860_e155928_d_n4;
        locals.var_tmf3_dn5 = assign103860_e155928_d_n5;
        locals.var_tmf3_dn6 = assign103860_e155928_d_n6;
        locals.var_tmf3_dn7 = assign103860_e155928_d_n7;
        locals.var_tmf3_dn8 = assign103860_e155928_d_n8;
        locals.var_tmf3_dn9 = assign103860_e155928_d_n9;
        locals.var_tmf3_dn10 = assign103860_e155928_d_n10;
        locals.var_tmf3_dn11 = assign103860_e155928_d_n11;
        locals.var_tmf3_dn14 = assign103860_e155928_d_n14;

        let (assign103870_e155940, assign103870_e155940_d_n0, assign103870_e155940_d_n2, assign103870_e155940_d_n4, assign103870_e155940_d_n5, assign103870_e155940_d_n6, assign103870_e155940_d_n7, assign103870_e155940_d_n8, assign103870_e155940_d_n9, assign103870_e155940_d_n10, assign103870_e155940_d_n11, assign103870_e155940_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2365 == 0.0)) {
        let assign103870_e155938: f64 = (p.p262 / locals.var_tmf2);
        (assign103870_e155938, (-((p.p262 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn11, locals.var_vzadd_dn14,)
    }
};
        locals.var_vzadd = assign103870_e155940;
        locals.var_vzadd_dn0 = assign103870_e155940_d_n0;
        locals.var_vzadd_dn2 = assign103870_e155940_d_n2;
        locals.var_vzadd_dn4 = assign103870_e155940_d_n4;
        locals.var_vzadd_dn5 = assign103870_e155940_d_n5;
        locals.var_vzadd_dn6 = assign103870_e155940_d_n6;
        locals.var_vzadd_dn7 = assign103870_e155940_d_n7;
        locals.var_vzadd_dn8 = assign103870_e155940_d_n8;
        locals.var_vzadd_dn9 = assign103870_e155940_d_n9;
        locals.var_vzadd_dn10 = assign103870_e155940_d_n10;
        locals.var_vzadd_dn11 = assign103870_e155940_d_n11;
        locals.var_vzadd_dn14 = assign103870_e155940_d_n14;

        let (assign103880_e155957, assign103880_e155957_d_n0, assign103880_e155957_d_n2, assign103880_e155957_d_n4, assign103880_e155957_d_n5, assign103880_e155957_d_n6, assign103880_e155957_d_n7, assign103880_e155957_d_n8, assign103880_e155957_d_n9, assign103880_e155957_d_n10, assign103880_e155957_d_n11, assign103880_e155957_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2365 == 0.0)) {
        let assign103880_e155949: f64 = (-2.0);
        let assign103880_e155951: f64 = (assign103880_e155949 * locals.var_tmf3);
        let assign103880_e155954: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign103880_e155955: f64 = (assign103880_e155951 / assign103880_e155954);
        (assign103880_e155955, ((((assign103880_e155949 * locals.var_tmf3_dn0) * assign103880_e155954) - (assign103880_e155951 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign103880_e155954 * assign103880_e155954)), ((((assign103880_e155949 * locals.var_tmf3_dn2) * assign103880_e155954) - (assign103880_e155951 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign103880_e155954 * assign103880_e155954)), ((((assign103880_e155949 * locals.var_tmf3_dn4) * assign103880_e155954) - (assign103880_e155951 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign103880_e155954 * assign103880_e155954)), ((((assign103880_e155949 * locals.var_tmf3_dn5) * assign103880_e155954) - (assign103880_e155951 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign103880_e155954 * assign103880_e155954)), ((((assign103880_e155949 * locals.var_tmf3_dn6) * assign103880_e155954) - (assign103880_e155951 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign103880_e155954 * assign103880_e155954)), ((((assign103880_e155949 * locals.var_tmf3_dn7) * assign103880_e155954) - (assign103880_e155951 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign103880_e155954 * assign103880_e155954)), ((((assign103880_e155949 * locals.var_tmf3_dn8) * assign103880_e155954) - (assign103880_e155951 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign103880_e155954 * assign103880_e155954)), ((((assign103880_e155949 * locals.var_tmf3_dn9) * assign103880_e155954) - (assign103880_e155951 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign103880_e155954 * assign103880_e155954)), ((((assign103880_e155949 * locals.var_tmf3_dn10) * assign103880_e155954) - (assign103880_e155951 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign103880_e155954 * assign103880_e155954)), ((((assign103880_e155949 * locals.var_tmf3_dn11) * assign103880_e155954) - (assign103880_e155951 * ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)))) / (assign103880_e155954 * assign103880_e155954)), ((((assign103880_e155949 * locals.var_tmf3_dn14) * assign103880_e155954) - (assign103880_e155951 * ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)))) / (assign103880_e155954 * assign103880_e155954)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign103880_e155957;
        locals.var_t2_dn0 = assign103880_e155957_d_n0;
        locals.var_t2_dn2 = assign103880_e155957_d_n2;
        locals.var_t2_dn4 = assign103880_e155957_d_n4;
        locals.var_t2_dn5 = assign103880_e155957_d_n5;
        locals.var_t2_dn6 = assign103880_e155957_d_n6;
        locals.var_t2_dn7 = assign103880_e155957_d_n7;
        locals.var_t2_dn8 = assign103880_e155957_d_n8;
        locals.var_t2_dn9 = assign103880_e155957_d_n9;
        locals.var_t2_dn10 = assign103880_e155957_d_n10;
        locals.var_t2_dn11 = assign103880_e155957_d_n11;
        locals.var_t2_dn14 = assign103880_e155957_d_n14;

        let assign103890_e155960: f64 = if locals.var_vzadd < 1e-12 { 1.0 } else { 0.0 };
        locals.var_guard2367 = assign103890_e155960;

        let (assign103900_e155972, assign103900_e155972_d_n0, assign103900_e155972_d_n2, assign103900_e155972_d_n4, assign103900_e155972_d_n5, assign103900_e155972_d_n6, assign103900_e155972_d_n7, assign103900_e155972_d_n8, assign103900_e155972_d_n9, assign103900_e155972_d_n10, assign103900_e155972_d_n11, assign103900_e155972_d_n14,) = {
    if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2365 == 0.0)) && (locals.var_guard2367 != 0.0)) {
        (1e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn11, locals.var_vzadd_dn14,)
    }
};
        locals.var_vzadd = assign103900_e155972;
        locals.var_vzadd_dn0 = assign103900_e155972_d_n0;
        locals.var_vzadd_dn2 = assign103900_e155972_d_n2;
        locals.var_vzadd_dn4 = assign103900_e155972_d_n4;
        locals.var_vzadd_dn5 = assign103900_e155972_d_n5;
        locals.var_vzadd_dn6 = assign103900_e155972_d_n6;
        locals.var_vzadd_dn7 = assign103900_e155972_d_n7;
        locals.var_vzadd_dn8 = assign103900_e155972_d_n8;
        locals.var_vzadd_dn9 = assign103900_e155972_d_n9;
        locals.var_vzadd_dn10 = assign103900_e155972_d_n10;
        locals.var_vzadd_dn11 = assign103900_e155972_d_n11;
        locals.var_vzadd_dn14 = assign103900_e155972_d_n14;

        let (assign103910_e155986, assign103910_e155986_d_n0, assign103910_e155986_d_n2, assign103910_e155986_d_n4, assign103910_e155986_d_n5, assign103910_e155986_d_n6, assign103910_e155986_d_n7, assign103910_e155986_d_n8, assign103910_e155986_d_n9, assign103910_e155986_d_n10, assign103910_e155986_d_n11, assign103910_e155986_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2365 == 0.0)) {
        let assign103910_e155983: f64 = (2.0 * locals.var_vzadd);
        let assign103910_e155984: f64 = (locals.var_vddp + assign103910_e155983);
        (assign103910_e155984, (locals.var_vddp_dn0 + (2.0 * locals.var_vzadd_dn0)), (2.0 * locals.var_vzadd_dn2), (2.0 * locals.var_vzadd_dn4), (2.0 * locals.var_vzadd_dn5), (locals.var_vddp_dn6 + (2.0 * locals.var_vzadd_dn6)), (2.0 * locals.var_vzadd_dn7), (2.0 * locals.var_vzadd_dn8), (2.0 * locals.var_vzadd_dn9), (2.0 * locals.var_vzadd_dn10), (2.0 * locals.var_vzadd_dn11), (2.0 * locals.var_vzadd_dn14),)
    } else {
        (locals.var_vddpz, locals.var_vddpz_dn0, locals.var_vddpz_dn2, locals.var_vddpz_dn4, locals.var_vddpz_dn5, locals.var_vddpz_dn6, locals.var_vddpz_dn7, locals.var_vddpz_dn8, locals.var_vddpz_dn9, locals.var_vddpz_dn10, locals.var_vddpz_dn11, locals.var_vddpz_dn14,)
    }
};
        locals.var_vddpz = assign103910_e155986;
        locals.var_vddpz_dn0 = assign103910_e155986_d_n0;
        locals.var_vddpz_dn2 = assign103910_e155986_d_n2;
        locals.var_vddpz_dn4 = assign103910_e155986_d_n4;
        locals.var_vddpz_dn5 = assign103910_e155986_d_n5;
        locals.var_vddpz_dn6 = assign103910_e155986_d_n6;
        locals.var_vddpz_dn7 = assign103910_e155986_d_n7;
        locals.var_vddpz_dn8 = assign103910_e155986_d_n8;
        locals.var_vddpz_dn9 = assign103910_e155986_d_n9;
        locals.var_vddpz_dn10 = assign103910_e155986_d_n10;
        locals.var_vddpz_dn11 = assign103910_e155986_d_n11;
        locals.var_vddpz_dn14 = assign103910_e155986_d_n14;

        let (assign103920_e155995, assign103920_e155995_d_n0, assign103920_e155995_d_n2, assign103920_e155995_d_n4, assign103920_e155995_d_n5, assign103920_e155995_d_n6, assign103920_e155995_d_n7, assign103920_e155995_d_n8, assign103920_e155995_d_n9, assign103920_e155995_d_n10, assign103920_e155995_d_n11, assign103920_e155995_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign103920_e155993: f64 = (locals.var_vddpz / locals.var_ldrifte);
        (assign103920_e155993, (locals.var_vddpz_dn0 / locals.var_ldrifte), (locals.var_vddpz_dn2 / locals.var_ldrifte), (locals.var_vddpz_dn4 / locals.var_ldrifte), (locals.var_vddpz_dn5 / locals.var_ldrifte), (locals.var_vddpz_dn6 / locals.var_ldrifte), (locals.var_vddpz_dn7 / locals.var_ldrifte), (locals.var_vddpz_dn8 / locals.var_ldrifte), (locals.var_vddpz_dn9 / locals.var_ldrifte), (locals.var_vddpz_dn10 / locals.var_ldrifte), (locals.var_vddpz_dn11 / locals.var_ldrifte), (locals.var_vddpz_dn14 / locals.var_ldrifte),)
    } else {
        (locals.var_edri, locals.var_edri_dn0, locals.var_edri_dn2, locals.var_edri_dn4, locals.var_edri_dn5, locals.var_edri_dn6, locals.var_edri_dn7, locals.var_edri_dn8, locals.var_edri_dn9, locals.var_edri_dn10, locals.var_edri_dn11, locals.var_edri_dn14,)
    }
};
        locals.var_edri = assign103920_e155995;
        locals.var_edri_dn0 = assign103920_e155995_d_n0;
        locals.var_edri_dn2 = assign103920_e155995_d_n2;
        locals.var_edri_dn4 = assign103920_e155995_d_n4;
        locals.var_edri_dn5 = assign103920_e155995_d_n5;
        locals.var_edri_dn6 = assign103920_e155995_d_n6;
        locals.var_edri_dn7 = assign103920_e155995_d_n7;
        locals.var_edri_dn8 = assign103920_e155995_d_n8;
        locals.var_edri_dn9 = assign103920_e155995_d_n9;
        locals.var_edri_dn10 = assign103920_e155995_d_n10;
        locals.var_edri_dn11 = assign103920_e155995_d_n11;
        locals.var_edri_dn14 = assign103920_e155995_d_n14;

        let (assign103930_e156004, assign103930_e156004_d_n0, assign103930_e156004_d_n2, assign103930_e156004_d_n4, assign103930_e156004_d_n5, assign103930_e156004_d_n6, assign103930_e156004_d_n7, assign103930_e156004_d_n8, assign103930_e156004_d_n9, assign103930_e156004_d_n10, assign103930_e156004_d_n11, assign103930_e156004_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign103930_e156002: f64 = (locals.var_mu0 * locals.var_edri);
        (assign103930_e156002, ((locals.var_mu0_dn0 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn0)), ((locals.var_mu0_dn2 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn2)), ((locals.var_mu0_dn4 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn4)), ((locals.var_mu0_dn5 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn5)), ((locals.var_mu0_dn6 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn6)), ((locals.var_mu0_dn7 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn7)), ((locals.var_mu0_dn8 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn8)), ((locals.var_mu0_dn9 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn9)), ((locals.var_mu0_dn10 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn10)), ((locals.var_mu0_dn11 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn11)), ((locals.var_mu0_dn14 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn14)),)
    } else {
        (locals.var_vdri, locals.var_vdri_dn0, locals.var_vdri_dn2, locals.var_vdri_dn4, locals.var_vdri_dn5, locals.var_vdri_dn6, locals.var_vdri_dn7, locals.var_vdri_dn8, locals.var_vdri_dn9, locals.var_vdri_dn10, locals.var_vdri_dn11, locals.var_vdri_dn14,)
    }
};
        locals.var_vdri = assign103930_e156004;
        locals.var_vdri_dn0 = assign103930_e156004_d_n0;
        locals.var_vdri_dn2 = assign103930_e156004_d_n2;
        locals.var_vdri_dn4 = assign103930_e156004_d_n4;
        locals.var_vdri_dn5 = assign103930_e156004_d_n5;
        locals.var_vdri_dn6 = assign103930_e156004_d_n6;
        locals.var_vdri_dn7 = assign103930_e156004_d_n7;
        locals.var_vdri_dn8 = assign103930_e156004_d_n8;
        locals.var_vdri_dn9 = assign103930_e156004_d_n9;
        locals.var_vdri_dn10 = assign103930_e156004_d_n10;
        locals.var_vdri_dn11 = assign103930_e156004_d_n11;
        locals.var_vdri_dn14 = assign103930_e156004_d_n14;

        let assign103940_e156007: f64 = if locals.var_vddp >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2368 = assign103940_e156007;

        let (assign103950_e156018, assign103950_e156018_d_n0, assign103950_e156018_d_n2, assign103950_e156018_d_n4, assign103950_e156018_d_n5, assign103950_e156018_d_n6, assign103950_e156018_d_n7, assign103950_e156018_d_n8, assign103950_e156018_d_n9, assign103950_e156018_d_n10, assign103950_e156018_d_n11, assign103950_e156018_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2368 != 0.0)) {
        let assign103950_e156016: f64 = (locals.var_vdri / locals.var_vmaxe__blk2357);
        (assign103950_e156016, (((locals.var_vdri_dn0 * locals.var_vmaxe__blk2357) - (locals.var_vdri * locals.var_vmaxe__blk2357_dn0)) / (locals.var_vmaxe__blk2357 * locals.var_vmaxe__blk2357)), (((locals.var_vdri_dn2 * locals.var_vmaxe__blk2357) - (locals.var_vdri * locals.var_vmaxe__blk2357_dn2)) / (locals.var_vmaxe__blk2357 * locals.var_vmaxe__blk2357)), (((locals.var_vdri_dn4 * locals.var_vmaxe__blk2357) - (locals.var_vdri * locals.var_vmaxe__blk2357_dn4)) / (locals.var_vmaxe__blk2357 * locals.var_vmaxe__blk2357)), (((locals.var_vdri_dn5 * locals.var_vmaxe__blk2357) - (locals.var_vdri * locals.var_vmaxe__blk2357_dn5)) / (locals.var_vmaxe__blk2357 * locals.var_vmaxe__blk2357)), (((locals.var_vdri_dn6 * locals.var_vmaxe__blk2357) - (locals.var_vdri * locals.var_vmaxe__blk2357_dn6)) / (locals.var_vmaxe__blk2357 * locals.var_vmaxe__blk2357)), (((locals.var_vdri_dn7 * locals.var_vmaxe__blk2357) - (locals.var_vdri * locals.var_vmaxe__blk2357_dn7)) / (locals.var_vmaxe__blk2357 * locals.var_vmaxe__blk2357)), (((locals.var_vdri_dn8 * locals.var_vmaxe__blk2357) - (locals.var_vdri * locals.var_vmaxe__blk2357_dn8)) / (locals.var_vmaxe__blk2357 * locals.var_vmaxe__blk2357)), (((locals.var_vdri_dn9 * locals.var_vmaxe__blk2357) - (locals.var_vdri * locals.var_vmaxe__blk2357_dn9)) / (locals.var_vmaxe__blk2357 * locals.var_vmaxe__blk2357)), (((locals.var_vdri_dn10 * locals.var_vmaxe__blk2357) - (locals.var_vdri * locals.var_vmaxe__blk2357_dn10)) / (locals.var_vmaxe__blk2357 * locals.var_vmaxe__blk2357)), (((locals.var_vdri_dn11 * locals.var_vmaxe__blk2357) - (locals.var_vdri * locals.var_vmaxe__blk2357_dn11)) / (locals.var_vmaxe__blk2357 * locals.var_vmaxe__blk2357)), (((locals.var_vdri_dn14 * locals.var_vmaxe__blk2357) - (locals.var_vdri * locals.var_vmaxe__blk2357_dn14)) / (locals.var_vmaxe__blk2357 * locals.var_vmaxe__blk2357)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign103950_e156018;
        locals.var_t1_dn0 = assign103950_e156018_d_n0;
        locals.var_t1_dn2 = assign103950_e156018_d_n2;
        locals.var_t1_dn4 = assign103950_e156018_d_n4;
        locals.var_t1_dn5 = assign103950_e156018_d_n5;
        locals.var_t1_dn6 = assign103950_e156018_d_n6;
        locals.var_t1_dn7 = assign103950_e156018_d_n7;
        locals.var_t1_dn8 = assign103950_e156018_d_n8;
        locals.var_t1_dn9 = assign103950_e156018_d_n9;
        locals.var_t1_dn10 = assign103950_e156018_d_n10;
        locals.var_t1_dn11 = assign103950_e156018_d_n11;
        locals.var_t1_dn14 = assign103950_e156018_d_n14;

        let (assign103960_e156031, assign103960_e156031_d_n0, assign103960_e156031_d_n2, assign103960_e156031_d_n4, assign103960_e156031_d_n5, assign103960_e156031_d_n6, assign103960_e156031_d_n7, assign103960_e156031_d_n8, assign103960_e156031_d_n9, assign103960_e156031_d_n10, assign103960_e156031_d_n11, assign103960_e156031_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2368 == 0.0)) {
        let assign103960_e156027: f64 = (-locals.var_vdri);
        let assign103960_e156029: f64 = (assign103960_e156027 / locals.var_vmaxe__blk2357);
        (assign103960_e156029, ((((-locals.var_vdri_dn0) * locals.var_vmaxe__blk2357) - (assign103960_e156027 * locals.var_vmaxe__blk2357_dn0)) / (locals.var_vmaxe__blk2357 * locals.var_vmaxe__blk2357)), ((((-locals.var_vdri_dn2) * locals.var_vmaxe__blk2357) - (assign103960_e156027 * locals.var_vmaxe__blk2357_dn2)) / (locals.var_vmaxe__blk2357 * locals.var_vmaxe__blk2357)), ((((-locals.var_vdri_dn4) * locals.var_vmaxe__blk2357) - (assign103960_e156027 * locals.var_vmaxe__blk2357_dn4)) / (locals.var_vmaxe__blk2357 * locals.var_vmaxe__blk2357)), ((((-locals.var_vdri_dn5) * locals.var_vmaxe__blk2357) - (assign103960_e156027 * locals.var_vmaxe__blk2357_dn5)) / (locals.var_vmaxe__blk2357 * locals.var_vmaxe__blk2357)), ((((-locals.var_vdri_dn6) * locals.var_vmaxe__blk2357) - (assign103960_e156027 * locals.var_vmaxe__blk2357_dn6)) / (locals.var_vmaxe__blk2357 * locals.var_vmaxe__blk2357)), ((((-locals.var_vdri_dn7) * locals.var_vmaxe__blk2357) - (assign103960_e156027 * locals.var_vmaxe__blk2357_dn7)) / (locals.var_vmaxe__blk2357 * locals.var_vmaxe__blk2357)), ((((-locals.var_vdri_dn8) * locals.var_vmaxe__blk2357) - (assign103960_e156027 * locals.var_vmaxe__blk2357_dn8)) / (locals.var_vmaxe__blk2357 * locals.var_vmaxe__blk2357)), ((((-locals.var_vdri_dn9) * locals.var_vmaxe__blk2357) - (assign103960_e156027 * locals.var_vmaxe__blk2357_dn9)) / (locals.var_vmaxe__blk2357 * locals.var_vmaxe__blk2357)), ((((-locals.var_vdri_dn10) * locals.var_vmaxe__blk2357) - (assign103960_e156027 * locals.var_vmaxe__blk2357_dn10)) / (locals.var_vmaxe__blk2357 * locals.var_vmaxe__blk2357)), ((((-locals.var_vdri_dn11) * locals.var_vmaxe__blk2357) - (assign103960_e156027 * locals.var_vmaxe__blk2357_dn11)) / (locals.var_vmaxe__blk2357 * locals.var_vmaxe__blk2357)), ((((-locals.var_vdri_dn14) * locals.var_vmaxe__blk2357) - (assign103960_e156027 * locals.var_vmaxe__blk2357_dn14)) / (locals.var_vmaxe__blk2357 * locals.var_vmaxe__blk2357)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign103960_e156031;
        locals.var_t1_dn0 = assign103960_e156031_d_n0;
        locals.var_t1_dn2 = assign103960_e156031_d_n2;
        locals.var_t1_dn4 = assign103960_e156031_d_n4;
        locals.var_t1_dn5 = assign103960_e156031_d_n5;
        locals.var_t1_dn6 = assign103960_e156031_d_n6;
        locals.var_t1_dn7 = assign103960_e156031_d_n7;
        locals.var_t1_dn8 = assign103960_e156031_d_n8;
        locals.var_t1_dn9 = assign103960_e156031_d_n9;
        locals.var_t1_dn10 = assign103960_e156031_d_n10;
        locals.var_t1_dn11 = assign103960_e156031_d_n11;
        locals.var_t1_dn14 = assign103960_e156031_d_n14;

        let assign103970_e156035: f64 = (10.0 * 2.220446049250313e-16);
        let assign103970_e156036: f64 = (1.0 - assign103970_e156035);
        let assign103970_e156043: f64 = (10.0 * 2.220446049250313e-16);
        let assign103970_e156044: f64 = (1.0 + assign103970_e156043);
        let assign103970_e156046: f64 = if ((assign103970_e156036 <= locals.var_uc_rdrbb) && (locals.var_uc_rdrbb <= assign103970_e156044)) { 1.0 } else { 0.0 };
        locals.var_guard2369 = assign103970_e156046;

        let (assign103980_e156055, assign103980_e156055_d_n0, assign103980_e156055_d_n2, assign103980_e156055_d_n4, assign103980_e156055_d_n5, assign103980_e156055_d_n6, assign103980_e156055_d_n7, assign103980_e156055_d_n8, assign103980_e156055_d_n9, assign103980_e156055_d_n10, assign103980_e156055_d_n11, assign103980_e156055_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2369 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign103980_e156055;
        locals.var_t3_dn0 = assign103980_e156055_d_n0;
        locals.var_t3_dn2 = assign103980_e156055_d_n2;
        locals.var_t3_dn4 = assign103980_e156055_d_n4;
        locals.var_t3_dn5 = assign103980_e156055_d_n5;
        locals.var_t3_dn6 = assign103980_e156055_d_n6;
        locals.var_t3_dn7 = assign103980_e156055_d_n7;
        locals.var_t3_dn8 = assign103980_e156055_d_n8;
        locals.var_t3_dn9 = assign103980_e156055_d_n9;
        locals.var_t3_dn10 = assign103980_e156055_d_n10;
        locals.var_t3_dn11 = assign103980_e156055_d_n11;
        locals.var_t3_dn14 = assign103980_e156055_d_n14;

        let assign103990_e156059: f64 = (10.0 * 2.220446049250313e-16);
        let assign103990_e156060: f64 = (2.0 - assign103990_e156059);
        let assign103990_e156067: f64 = (10.0 * 2.220446049250313e-16);
        let assign103990_e156068: f64 = (2.0 + assign103990_e156067);
        let assign103990_e156070: f64 = if ((assign103990_e156060 <= locals.var_uc_rdrbb) && (locals.var_uc_rdrbb <= assign103990_e156068)) { 1.0 } else { 0.0 };
        locals.var_guard2370 = assign103990_e156070;

        let (assign104000_e156082, assign104000_e156082_d_n0, assign104000_e156082_d_n2, assign104000_e156082_d_n4, assign104000_e156082_d_n5, assign104000_e156082_d_n6, assign104000_e156082_d_n7, assign104000_e156082_d_n8, assign104000_e156082_d_n9, assign104000_e156082_d_n10, assign104000_e156082_d_n11, assign104000_e156082_d_n14,) = {
    if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2369 == 0.0)) && (locals.var_guard2370 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign104000_e156082;
        locals.var_t3_dn0 = assign104000_e156082_d_n0;
        locals.var_t3_dn2 = assign104000_e156082_d_n2;
        locals.var_t3_dn4 = assign104000_e156082_d_n4;
        locals.var_t3_dn5 = assign104000_e156082_d_n5;
        locals.var_t3_dn6 = assign104000_e156082_d_n6;
        locals.var_t3_dn7 = assign104000_e156082_d_n7;
        locals.var_t3_dn8 = assign104000_e156082_d_n8;
        locals.var_t3_dn9 = assign104000_e156082_d_n9;
        locals.var_t3_dn10 = assign104000_e156082_d_n10;
        locals.var_t3_dn11 = assign104000_e156082_d_n11;
        locals.var_t3_dn14 = assign104000_e156082_d_n14;

        let (assign104010_e156099, assign104010_e156099_d_n0, assign104010_e156099_d_n2, assign104010_e156099_d_n4, assign104010_e156099_d_n5, assign104010_e156099_d_n6, assign104010_e156099_d_n7, assign104010_e156099_d_n8, assign104010_e156099_d_n9, assign104010_e156099_d_n10, assign104010_e156099_d_n11, assign104010_e156099_d_n14,) = {
    if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2369 == 0.0)) && (locals.var_guard2370 == 0.0)) {
        let assign104010_e156096: f64 = (locals.var_uc_rdrbb - 1.0);
        let assign104010_e156097: f64 = (locals.var_t1).powf(assign104010_e156096);
        (assign104010_e156097, if locals.var_uc_rdrbb_dn0 == 0.0 && ((assign104010_e156096) as f64).is_finite() && ((assign104010_e156096) as f64).fract() == 0.0 { if assign104010_e156096 == 0.0 { 0.0 } else { (assign104010_e156096 * ((locals.var_t1).powf(assign104010_e156096 - 1.0) * locals.var_t1_dn0)) } } else { (assign104010_e156097 * ((locals.var_uc_rdrbb_dn0 * (locals.var_t1).ln()) + (assign104010_e156096 * (locals.var_t1_dn0 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn2 == 0.0 && ((assign104010_e156096) as f64).is_finite() && ((assign104010_e156096) as f64).fract() == 0.0 { if assign104010_e156096 == 0.0 { 0.0 } else { (assign104010_e156096 * ((locals.var_t1).powf(assign104010_e156096 - 1.0) * locals.var_t1_dn2)) } } else { (assign104010_e156097 * ((locals.var_uc_rdrbb_dn2 * (locals.var_t1).ln()) + (assign104010_e156096 * (locals.var_t1_dn2 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn4 == 0.0 && ((assign104010_e156096) as f64).is_finite() && ((assign104010_e156096) as f64).fract() == 0.0 { if assign104010_e156096 == 0.0 { 0.0 } else { (assign104010_e156096 * ((locals.var_t1).powf(assign104010_e156096 - 1.0) * locals.var_t1_dn4)) } } else { (assign104010_e156097 * ((locals.var_uc_rdrbb_dn4 * (locals.var_t1).ln()) + (assign104010_e156096 * (locals.var_t1_dn4 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn5 == 0.0 && ((assign104010_e156096) as f64).is_finite() && ((assign104010_e156096) as f64).fract() == 0.0 { if assign104010_e156096 == 0.0 { 0.0 } else { (assign104010_e156096 * ((locals.var_t1).powf(assign104010_e156096 - 1.0) * locals.var_t1_dn5)) } } else { (assign104010_e156097 * ((locals.var_uc_rdrbb_dn5 * (locals.var_t1).ln()) + (assign104010_e156096 * (locals.var_t1_dn5 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn6 == 0.0 && ((assign104010_e156096) as f64).is_finite() && ((assign104010_e156096) as f64).fract() == 0.0 { if assign104010_e156096 == 0.0 { 0.0 } else { (assign104010_e156096 * ((locals.var_t1).powf(assign104010_e156096 - 1.0) * locals.var_t1_dn6)) } } else { (assign104010_e156097 * ((locals.var_uc_rdrbb_dn6 * (locals.var_t1).ln()) + (assign104010_e156096 * (locals.var_t1_dn6 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn7 == 0.0 && ((assign104010_e156096) as f64).is_finite() && ((assign104010_e156096) as f64).fract() == 0.0 { if assign104010_e156096 == 0.0 { 0.0 } else { (assign104010_e156096 * ((locals.var_t1).powf(assign104010_e156096 - 1.0) * locals.var_t1_dn7)) } } else { (assign104010_e156097 * ((locals.var_uc_rdrbb_dn7 * (locals.var_t1).ln()) + (assign104010_e156096 * (locals.var_t1_dn7 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn8 == 0.0 && ((assign104010_e156096) as f64).is_finite() && ((assign104010_e156096) as f64).fract() == 0.0 { if assign104010_e156096 == 0.0 { 0.0 } else { (assign104010_e156096 * ((locals.var_t1).powf(assign104010_e156096 - 1.0) * locals.var_t1_dn8)) } } else { (assign104010_e156097 * ((locals.var_uc_rdrbb_dn8 * (locals.var_t1).ln()) + (assign104010_e156096 * (locals.var_t1_dn8 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn9 == 0.0 && ((assign104010_e156096) as f64).is_finite() && ((assign104010_e156096) as f64).fract() == 0.0 { if assign104010_e156096 == 0.0 { 0.0 } else { (assign104010_e156096 * ((locals.var_t1).powf(assign104010_e156096 - 1.0) * locals.var_t1_dn9)) } } else { (assign104010_e156097 * ((locals.var_uc_rdrbb_dn9 * (locals.var_t1).ln()) + (assign104010_e156096 * (locals.var_t1_dn9 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn10 == 0.0 && ((assign104010_e156096) as f64).is_finite() && ((assign104010_e156096) as f64).fract() == 0.0 { if assign104010_e156096 == 0.0 { 0.0 } else { (assign104010_e156096 * ((locals.var_t1).powf(assign104010_e156096 - 1.0) * locals.var_t1_dn10)) } } else { (assign104010_e156097 * ((locals.var_uc_rdrbb_dn10 * (locals.var_t1).ln()) + (assign104010_e156096 * (locals.var_t1_dn10 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn11 == 0.0 && ((assign104010_e156096) as f64).is_finite() && ((assign104010_e156096) as f64).fract() == 0.0 { if assign104010_e156096 == 0.0 { 0.0 } else { (assign104010_e156096 * ((locals.var_t1).powf(assign104010_e156096 - 1.0) * locals.var_t1_dn11)) } } else { (assign104010_e156097 * ((locals.var_uc_rdrbb_dn11 * (locals.var_t1).ln()) + (assign104010_e156096 * (locals.var_t1_dn11 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn14 == 0.0 && ((assign104010_e156096) as f64).is_finite() && ((assign104010_e156096) as f64).fract() == 0.0 { if assign104010_e156096 == 0.0 { 0.0 } else { (assign104010_e156096 * ((locals.var_t1).powf(assign104010_e156096 - 1.0) * locals.var_t1_dn14)) } } else { (assign104010_e156097 * ((locals.var_uc_rdrbb_dn14 * (locals.var_t1).ln()) + (assign104010_e156096 * (locals.var_t1_dn14 / locals.var_t1)))) },)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign104010_e156099;
        locals.var_t3_dn0 = assign104010_e156099_d_n0;
        locals.var_t3_dn2 = assign104010_e156099_d_n2;
        locals.var_t3_dn4 = assign104010_e156099_d_n4;
        locals.var_t3_dn5 = assign104010_e156099_d_n5;
        locals.var_t3_dn6 = assign104010_e156099_d_n6;
        locals.var_t3_dn7 = assign104010_e156099_d_n7;
        locals.var_t3_dn8 = assign104010_e156099_d_n8;
        locals.var_t3_dn9 = assign104010_e156099_d_n9;
        locals.var_t3_dn10 = assign104010_e156099_d_n10;
        locals.var_t3_dn11 = assign104010_e156099_d_n11;
        locals.var_t3_dn14 = assign104010_e156099_d_n14;

        let (assign104020_e156108, assign104020_e156108_d_n0, assign104020_e156108_d_n2, assign104020_e156108_d_n4, assign104020_e156108_d_n5, assign104020_e156108_d_n6, assign104020_e156108_d_n7, assign104020_e156108_d_n8, assign104020_e156108_d_n9, assign104020_e156108_d_n10, assign104020_e156108_d_n11, assign104020_e156108_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104020_e156106: f64 = (locals.var_t1 * locals.var_t3);
        (assign104020_e156106, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)), ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)), ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)), ((locals.var_t1_dn14 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign104020_e156108;
        locals.var_t2_dn0 = assign104020_e156108_d_n0;
        locals.var_t2_dn2 = assign104020_e156108_d_n2;
        locals.var_t2_dn4 = assign104020_e156108_d_n4;
        locals.var_t2_dn5 = assign104020_e156108_d_n5;
        locals.var_t2_dn6 = assign104020_e156108_d_n6;
        locals.var_t2_dn7 = assign104020_e156108_d_n7;
        locals.var_t2_dn8 = assign104020_e156108_d_n8;
        locals.var_t2_dn9 = assign104020_e156108_d_n9;
        locals.var_t2_dn10 = assign104020_e156108_d_n10;
        locals.var_t2_dn11 = assign104020_e156108_d_n11;
        locals.var_t2_dn14 = assign104020_e156108_d_n14;

        let (assign104030_e156117, assign104030_e156117_d_n0, assign104030_e156117_d_n2, assign104030_e156117_d_n4, assign104030_e156117_d_n5, assign104030_e156117_d_n6, assign104030_e156117_d_n7, assign104030_e156117_d_n8, assign104030_e156117_d_n9, assign104030_e156117_d_n10, assign104030_e156117_d_n11, assign104030_e156117_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104030_e156115: f64 = (1.0 + locals.var_t2);
        (assign104030_e156115, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign104030_e156117;
        locals.var_t4_dn0 = assign104030_e156117_d_n0;
        locals.var_t4_dn2 = assign104030_e156117_d_n2;
        locals.var_t4_dn4 = assign104030_e156117_d_n4;
        locals.var_t4_dn5 = assign104030_e156117_d_n5;
        locals.var_t4_dn6 = assign104030_e156117_d_n6;
        locals.var_t4_dn7 = assign104030_e156117_d_n7;
        locals.var_t4_dn8 = assign104030_e156117_d_n8;
        locals.var_t4_dn9 = assign104030_e156117_d_n9;
        locals.var_t4_dn10 = assign104030_e156117_d_n10;
        locals.var_t4_dn11 = assign104030_e156117_d_n11;
        locals.var_t4_dn14 = assign104030_e156117_d_n14;

        let assign104040_e156121: f64 = (10.0 * 2.220446049250313e-16);
        let assign104040_e156122: f64 = (1.0 - assign104040_e156121);
        let assign104040_e156129: f64 = (10.0 * 2.220446049250313e-16);
        let assign104040_e156130: f64 = (1.0 + assign104040_e156129);
        let assign104040_e156132: f64 = if ((assign104040_e156122 <= locals.var_uc_rdrbb) && (locals.var_uc_rdrbb <= assign104040_e156130)) { 1.0 } else { 0.0 };
        locals.var_guard2371 = assign104040_e156132;

        let (assign104050_e156143, assign104050_e156143_d_n0, assign104050_e156143_d_n2, assign104050_e156143_d_n4, assign104050_e156143_d_n5, assign104050_e156143_d_n6, assign104050_e156143_d_n7, assign104050_e156143_d_n8, assign104050_e156143_d_n9, assign104050_e156143_d_n10, assign104050_e156143_d_n11, assign104050_e156143_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2371 != 0.0)) {
        let assign104050_e156141: f64 = (1.0 / locals.var_t4);
        (assign104050_e156141, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn14 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign104050_e156143;
        locals.var_t5_dn0 = assign104050_e156143_d_n0;
        locals.var_t5_dn2 = assign104050_e156143_d_n2;
        locals.var_t5_dn4 = assign104050_e156143_d_n4;
        locals.var_t5_dn5 = assign104050_e156143_d_n5;
        locals.var_t5_dn6 = assign104050_e156143_d_n6;
        locals.var_t5_dn7 = assign104050_e156143_d_n7;
        locals.var_t5_dn8 = assign104050_e156143_d_n8;
        locals.var_t5_dn9 = assign104050_e156143_d_n9;
        locals.var_t5_dn10 = assign104050_e156143_d_n10;
        locals.var_t5_dn11 = assign104050_e156143_d_n11;
        locals.var_t5_dn14 = assign104050_e156143_d_n14;

        let assign104060_e156147: f64 = (10.0 * 2.220446049250313e-16);
        let assign104060_e156148: f64 = (2.0 - assign104060_e156147);
        let assign104060_e156155: f64 = (10.0 * 2.220446049250313e-16);
        let assign104060_e156156: f64 = (2.0 + assign104060_e156155);
        let assign104060_e156158: f64 = if ((assign104060_e156148 <= locals.var_uc_rdrbb) && (locals.var_uc_rdrbb <= assign104060_e156156)) { 1.0 } else { 0.0 };
        locals.var_guard2372 = assign104060_e156158;

        let (assign104070_e156173, assign104070_e156173_d_n0, assign104070_e156173_d_n2, assign104070_e156173_d_n4, assign104070_e156173_d_n5, assign104070_e156173_d_n6, assign104070_e156173_d_n7, assign104070_e156173_d_n8, assign104070_e156173_d_n9, assign104070_e156173_d_n10, assign104070_e156173_d_n11, assign104070_e156173_d_n14,) = {
    if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2371 == 0.0)) && (locals.var_guard2372 != 0.0)) {
        let assign104070_e156170: f64 = (locals.var_t4).sqrt();
        let assign104070_e156171: f64 = (1.0 / assign104070_e156170);
        (assign104070_e156171, (-((locals.var_t4_dn0 / (2.0 * assign104070_e156170)) / (assign104070_e156170 * assign104070_e156170))), (-((locals.var_t4_dn2 / (2.0 * assign104070_e156170)) / (assign104070_e156170 * assign104070_e156170))), (-((locals.var_t4_dn4 / (2.0 * assign104070_e156170)) / (assign104070_e156170 * assign104070_e156170))), (-((locals.var_t4_dn5 / (2.0 * assign104070_e156170)) / (assign104070_e156170 * assign104070_e156170))), (-((locals.var_t4_dn6 / (2.0 * assign104070_e156170)) / (assign104070_e156170 * assign104070_e156170))), (-((locals.var_t4_dn7 / (2.0 * assign104070_e156170)) / (assign104070_e156170 * assign104070_e156170))), (-((locals.var_t4_dn8 / (2.0 * assign104070_e156170)) / (assign104070_e156170 * assign104070_e156170))), (-((locals.var_t4_dn9 / (2.0 * assign104070_e156170)) / (assign104070_e156170 * assign104070_e156170))), (-((locals.var_t4_dn10 / (2.0 * assign104070_e156170)) / (assign104070_e156170 * assign104070_e156170))), (-((locals.var_t4_dn11 / (2.0 * assign104070_e156170)) / (assign104070_e156170 * assign104070_e156170))), (-((locals.var_t4_dn14 / (2.0 * assign104070_e156170)) / (assign104070_e156170 * assign104070_e156170))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign104070_e156173;
        locals.var_t5_dn0 = assign104070_e156173_d_n0;
        locals.var_t5_dn2 = assign104070_e156173_d_n2;
        locals.var_t5_dn4 = assign104070_e156173_d_n4;
        locals.var_t5_dn5 = assign104070_e156173_d_n5;
        locals.var_t5_dn6 = assign104070_e156173_d_n6;
        locals.var_t5_dn7 = assign104070_e156173_d_n7;
        locals.var_t5_dn8 = assign104070_e156173_d_n8;
        locals.var_t5_dn9 = assign104070_e156173_d_n9;
        locals.var_t5_dn10 = assign104070_e156173_d_n10;
        locals.var_t5_dn11 = assign104070_e156173_d_n11;
        locals.var_t5_dn14 = assign104070_e156173_d_n14;

        let (assign104080_e156198, assign104080_e156198_d_n0, assign104080_e156198_d_n2, assign104080_e156198_d_n4, assign104080_e156198_d_n5, assign104080_e156198_d_n6, assign104080_e156198_d_n7, assign104080_e156198_d_n8, assign104080_e156198_d_n9, assign104080_e156198_d_n10, assign104080_e156198_d_n11, assign104080_e156198_d_n14,) = {
    if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2371 == 0.0)) && (locals.var_guard2372 == 0.0)) {
        let (assign104080_e156196, assign104080_e156196_d_n0, assign104080_e156196_d_n2, assign104080_e156196_d_n4, assign104080_e156196_d_n5, assign104080_e156196_d_n6, assign104080_e156196_d_n7, assign104080_e156196_d_n8, assign104080_e156196_d_n9, assign104080_e156196_d_n10, assign104080_e156196_d_n11, assign104080_e156196_d_n14,) = {
            if (locals.var_t4 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign104080_e156190: f64 = (-1.0);
                let assign104080_e156192: f64 = (assign104080_e156190 / locals.var_uc_rdrbb);
                let assign104080_e156194: f64 = (assign104080_e156192 - 1.0);
                let assign104080_e156195: f64 = (locals.var_t4).powf(assign104080_e156194);
                (assign104080_e156195, if (-((assign104080_e156190 * locals.var_uc_rdrbb_dn0) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104080_e156194) as f64).is_finite() && ((assign104080_e156194) as f64).fract() == 0.0 { if assign104080_e156194 == 0.0 { 0.0 } else { (assign104080_e156194 * ((locals.var_t4).powf(assign104080_e156194 - 1.0) * locals.var_t4_dn0)) } } else { (assign104080_e156195 * (((-((assign104080_e156190 * locals.var_uc_rdrbb_dn0) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104080_e156194 * (locals.var_t4_dn0 / locals.var_t4)))) }, if (-((assign104080_e156190 * locals.var_uc_rdrbb_dn2) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104080_e156194) as f64).is_finite() && ((assign104080_e156194) as f64).fract() == 0.0 { if assign104080_e156194 == 0.0 { 0.0 } else { (assign104080_e156194 * ((locals.var_t4).powf(assign104080_e156194 - 1.0) * locals.var_t4_dn2)) } } else { (assign104080_e156195 * (((-((assign104080_e156190 * locals.var_uc_rdrbb_dn2) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104080_e156194 * (locals.var_t4_dn2 / locals.var_t4)))) }, if (-((assign104080_e156190 * locals.var_uc_rdrbb_dn4) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104080_e156194) as f64).is_finite() && ((assign104080_e156194) as f64).fract() == 0.0 { if assign104080_e156194 == 0.0 { 0.0 } else { (assign104080_e156194 * ((locals.var_t4).powf(assign104080_e156194 - 1.0) * locals.var_t4_dn4)) } } else { (assign104080_e156195 * (((-((assign104080_e156190 * locals.var_uc_rdrbb_dn4) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104080_e156194 * (locals.var_t4_dn4 / locals.var_t4)))) }, if (-((assign104080_e156190 * locals.var_uc_rdrbb_dn5) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104080_e156194) as f64).is_finite() && ((assign104080_e156194) as f64).fract() == 0.0 { if assign104080_e156194 == 0.0 { 0.0 } else { (assign104080_e156194 * ((locals.var_t4).powf(assign104080_e156194 - 1.0) * locals.var_t4_dn5)) } } else { (assign104080_e156195 * (((-((assign104080_e156190 * locals.var_uc_rdrbb_dn5) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104080_e156194 * (locals.var_t4_dn5 / locals.var_t4)))) }, if (-((assign104080_e156190 * locals.var_uc_rdrbb_dn6) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104080_e156194) as f64).is_finite() && ((assign104080_e156194) as f64).fract() == 0.0 { if assign104080_e156194 == 0.0 { 0.0 } else { (assign104080_e156194 * ((locals.var_t4).powf(assign104080_e156194 - 1.0) * locals.var_t4_dn6)) } } else { (assign104080_e156195 * (((-((assign104080_e156190 * locals.var_uc_rdrbb_dn6) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104080_e156194 * (locals.var_t4_dn6 / locals.var_t4)))) }, if (-((assign104080_e156190 * locals.var_uc_rdrbb_dn7) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104080_e156194) as f64).is_finite() && ((assign104080_e156194) as f64).fract() == 0.0 { if assign104080_e156194 == 0.0 { 0.0 } else { (assign104080_e156194 * ((locals.var_t4).powf(assign104080_e156194 - 1.0) * locals.var_t4_dn7)) } } else { (assign104080_e156195 * (((-((assign104080_e156190 * locals.var_uc_rdrbb_dn7) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104080_e156194 * (locals.var_t4_dn7 / locals.var_t4)))) }, if (-((assign104080_e156190 * locals.var_uc_rdrbb_dn8) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104080_e156194) as f64).is_finite() && ((assign104080_e156194) as f64).fract() == 0.0 { if assign104080_e156194 == 0.0 { 0.0 } else { (assign104080_e156194 * ((locals.var_t4).powf(assign104080_e156194 - 1.0) * locals.var_t4_dn8)) } } else { (assign104080_e156195 * (((-((assign104080_e156190 * locals.var_uc_rdrbb_dn8) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104080_e156194 * (locals.var_t4_dn8 / locals.var_t4)))) }, if (-((assign104080_e156190 * locals.var_uc_rdrbb_dn9) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104080_e156194) as f64).is_finite() && ((assign104080_e156194) as f64).fract() == 0.0 { if assign104080_e156194 == 0.0 { 0.0 } else { (assign104080_e156194 * ((locals.var_t4).powf(assign104080_e156194 - 1.0) * locals.var_t4_dn9)) } } else { (assign104080_e156195 * (((-((assign104080_e156190 * locals.var_uc_rdrbb_dn9) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104080_e156194 * (locals.var_t4_dn9 / locals.var_t4)))) }, if (-((assign104080_e156190 * locals.var_uc_rdrbb_dn10) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104080_e156194) as f64).is_finite() && ((assign104080_e156194) as f64).fract() == 0.0 { if assign104080_e156194 == 0.0 { 0.0 } else { (assign104080_e156194 * ((locals.var_t4).powf(assign104080_e156194 - 1.0) * locals.var_t4_dn10)) } } else { (assign104080_e156195 * (((-((assign104080_e156190 * locals.var_uc_rdrbb_dn10) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104080_e156194 * (locals.var_t4_dn10 / locals.var_t4)))) }, if (-((assign104080_e156190 * locals.var_uc_rdrbb_dn11) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104080_e156194) as f64).is_finite() && ((assign104080_e156194) as f64).fract() == 0.0 { if assign104080_e156194 == 0.0 { 0.0 } else { (assign104080_e156194 * ((locals.var_t4).powf(assign104080_e156194 - 1.0) * locals.var_t4_dn11)) } } else { (assign104080_e156195 * (((-((assign104080_e156190 * locals.var_uc_rdrbb_dn11) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104080_e156194 * (locals.var_t4_dn11 / locals.var_t4)))) }, if (-((assign104080_e156190 * locals.var_uc_rdrbb_dn14) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104080_e156194) as f64).is_finite() && ((assign104080_e156194) as f64).fract() == 0.0 { if assign104080_e156194 == 0.0 { 0.0 } else { (assign104080_e156194 * ((locals.var_t4).powf(assign104080_e156194 - 1.0) * locals.var_t4_dn14)) } } else { (assign104080_e156195 * (((-((assign104080_e156190 * locals.var_uc_rdrbb_dn14) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104080_e156194 * (locals.var_t4_dn14 / locals.var_t4)))) },)
            }
        };
        (assign104080_e156196, assign104080_e156196_d_n0, assign104080_e156196_d_n2, assign104080_e156196_d_n4, assign104080_e156196_d_n5, assign104080_e156196_d_n6, assign104080_e156196_d_n7, assign104080_e156196_d_n8, assign104080_e156196_d_n9, assign104080_e156196_d_n10, assign104080_e156196_d_n11, assign104080_e156196_d_n14,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign104080_e156198;
        locals.var_t6_dn0 = assign104080_e156198_d_n0;
        locals.var_t6_dn2 = assign104080_e156198_d_n2;
        locals.var_t6_dn4 = assign104080_e156198_d_n4;
        locals.var_t6_dn5 = assign104080_e156198_d_n5;
        locals.var_t6_dn6 = assign104080_e156198_d_n6;
        locals.var_t6_dn7 = assign104080_e156198_d_n7;
        locals.var_t6_dn8 = assign104080_e156198_d_n8;
        locals.var_t6_dn9 = assign104080_e156198_d_n9;
        locals.var_t6_dn10 = assign104080_e156198_d_n10;
        locals.var_t6_dn11 = assign104080_e156198_d_n11;
        locals.var_t6_dn14 = assign104080_e156198_d_n14;

    }

    pub(super) fn stamp_transient_block_380(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign104090_e156213, assign104090_e156213_d_n0, assign104090_e156213_d_n2, assign104090_e156213_d_n4, assign104090_e156213_d_n5, assign104090_e156213_d_n6, assign104090_e156213_d_n7, assign104090_e156213_d_n8, assign104090_e156213_d_n9, assign104090_e156213_d_n10, assign104090_e156213_d_n11, assign104090_e156213_d_n14,) = {
    if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2371 == 0.0)) && (locals.var_guard2372 == 0.0)) {
        let assign104090_e156211: f64 = (locals.var_t4 * locals.var_t6);
        (assign104090_e156211, ((locals.var_t4_dn0 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn0)), ((locals.var_t4_dn2 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn2)), ((locals.var_t4_dn4 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn4)), ((locals.var_t4_dn5 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn5)), ((locals.var_t4_dn6 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn6)), ((locals.var_t4_dn7 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn7)), ((locals.var_t4_dn8 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn8)), ((locals.var_t4_dn9 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn9)), ((locals.var_t4_dn10 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn10)), ((locals.var_t4_dn11 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn11)), ((locals.var_t4_dn14 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign104090_e156213;
        locals.var_t5_dn0 = assign104090_e156213_d_n0;
        locals.var_t5_dn2 = assign104090_e156213_d_n2;
        locals.var_t5_dn4 = assign104090_e156213_d_n4;
        locals.var_t5_dn5 = assign104090_e156213_d_n5;
        locals.var_t5_dn6 = assign104090_e156213_d_n6;
        locals.var_t5_dn7 = assign104090_e156213_d_n7;
        locals.var_t5_dn8 = assign104090_e156213_d_n8;
        locals.var_t5_dn9 = assign104090_e156213_d_n9;
        locals.var_t5_dn10 = assign104090_e156213_d_n10;
        locals.var_t5_dn11 = assign104090_e156213_d_n11;
        locals.var_t5_dn14 = assign104090_e156213_d_n14;

        let (assign104100_e156222, assign104100_e156222_d_n0, assign104100_e156222_d_n2, assign104100_e156222_d_n4, assign104100_e156222_d_n5, assign104100_e156222_d_n6, assign104100_e156222_d_n7, assign104100_e156222_d_n8, assign104100_e156222_d_n9, assign104100_e156222_d_n10, assign104100_e156222_d_n11, assign104100_e156222_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104100_e156220: f64 = (locals.var_mu0 * locals.var_t5);
        (assign104100_e156220, ((locals.var_mu0_dn0 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn0)), ((locals.var_mu0_dn2 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn2)), ((locals.var_mu0_dn4 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn4)), ((locals.var_mu0_dn5 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn5)), ((locals.var_mu0_dn6 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn6)), ((locals.var_mu0_dn7 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn7)), ((locals.var_mu0_dn8 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn8)), ((locals.var_mu0_dn9 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn9)), ((locals.var_mu0_dn10 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn10)), ((locals.var_mu0_dn11 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn11)), ((locals.var_mu0_dn14 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn14)),)
    } else {
        (locals.var_mu__blk2356, locals.var_mu__blk2356_dn0, locals.var_mu__blk2356_dn2, locals.var_mu__blk2356_dn4, locals.var_mu__blk2356_dn5, locals.var_mu__blk2356_dn6, locals.var_mu__blk2356_dn7, locals.var_mu__blk2356_dn8, locals.var_mu__blk2356_dn9, locals.var_mu__blk2356_dn10, locals.var_mu__blk2356_dn11, locals.var_mu__blk2356_dn14,)
    }
};
        locals.var_mu__blk2356 = assign104100_e156222;
        locals.var_mu__blk2356_dn0 = assign104100_e156222_d_n0;
        locals.var_mu__blk2356_dn2 = assign104100_e156222_d_n2;
        locals.var_mu__blk2356_dn4 = assign104100_e156222_d_n4;
        locals.var_mu__blk2356_dn5 = assign104100_e156222_d_n5;
        locals.var_mu__blk2356_dn6 = assign104100_e156222_d_n6;
        locals.var_mu__blk2356_dn7 = assign104100_e156222_d_n7;
        locals.var_mu__blk2356_dn8 = assign104100_e156222_d_n8;
        locals.var_mu__blk2356_dn9 = assign104100_e156222_d_n9;
        locals.var_mu__blk2356_dn10 = assign104100_e156222_d_n10;
        locals.var_mu__blk2356_dn11 = assign104100_e156222_d_n11;
        locals.var_mu__blk2356_dn14 = assign104100_e156222_d_n14;

        let (assign104110_e156231, assign104110_e156231_d_n0, assign104110_e156231_d_n2, assign104110_e156231_d_n4, assign104110_e156231_d_n5, assign104110_e156231_d_n6, assign104110_e156231_d_n7, assign104110_e156231_d_n8, assign104110_e156231_d_n9, assign104110_e156231_d_n10, assign104110_e156231_d_n11, assign104110_e156231_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104110_e156229: f64 = (1.0 + locals.var_t1);
        (assign104110_e156229, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign104110_e156231;
        locals.var_t4_dn0 = assign104110_e156231_d_n0;
        locals.var_t4_dn2 = assign104110_e156231_d_n2;
        locals.var_t4_dn4 = assign104110_e156231_d_n4;
        locals.var_t4_dn5 = assign104110_e156231_d_n5;
        locals.var_t4_dn6 = assign104110_e156231_d_n6;
        locals.var_t4_dn7 = assign104110_e156231_d_n7;
        locals.var_t4_dn8 = assign104110_e156231_d_n8;
        locals.var_t4_dn9 = assign104110_e156231_d_n9;
        locals.var_t4_dn10 = assign104110_e156231_d_n10;
        locals.var_t4_dn11 = assign104110_e156231_d_n11;
        locals.var_t4_dn14 = assign104110_e156231_d_n14;

        let (assign104120_e156240, assign104120_e156240_d_n0, assign104120_e156240_d_n2, assign104120_e156240_d_n4, assign104120_e156240_d_n5, assign104120_e156240_d_n6, assign104120_e156240_d_n7, assign104120_e156240_d_n8, assign104120_e156240_d_n9, assign104120_e156240_d_n10, assign104120_e156240_d_n11, assign104120_e156240_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104120_e156238: f64 = (1.0 / locals.var_t4);
        (assign104120_e156238, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn14 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign104120_e156240;
        locals.var_t5_dn0 = assign104120_e156240_d_n0;
        locals.var_t5_dn2 = assign104120_e156240_d_n2;
        locals.var_t5_dn4 = assign104120_e156240_d_n4;
        locals.var_t5_dn5 = assign104120_e156240_d_n5;
        locals.var_t5_dn6 = assign104120_e156240_d_n6;
        locals.var_t5_dn7 = assign104120_e156240_d_n7;
        locals.var_t5_dn8 = assign104120_e156240_d_n8;
        locals.var_t5_dn9 = assign104120_e156240_d_n9;
        locals.var_t5_dn10 = assign104120_e156240_d_n10;
        locals.var_t5_dn11 = assign104120_e156240_d_n11;
        locals.var_t5_dn14 = assign104120_e156240_d_n14;

        let (assign104130_e156259, assign104130_e156259_d_n0, assign104130_e156259_d_n2, assign104130_e156259_d_n4, assign104130_e156259_d_n5, assign104130_e156259_d_n6, assign104130_e156259_d_n7, assign104130_e156259_d_n8, assign104130_e156259_d_n9, assign104130_e156259_d_n10, assign104130_e156259_d_n11, assign104130_e156259_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104130_e156249: f64 = (1.0 - locals.var_t5);
        let assign104130_e156250: f64 = (locals.var_car * assign104130_e156249);
        let assign104130_e156252: f64 = (assign104130_e156250 * locals.var_vddpz);
        let assign104130_e156255: f64 = (locals.var_ldrifte - p.p423);
        let assign104130_e156256: f64 = (assign104130_e156252 / assign104130_e156255);
        let assign104130_e156257: f64 = (1.0 + assign104130_e156256);
        (assign104130_e156257, ((((locals.var_car * (-locals.var_t5_dn0)) * locals.var_vddpz) + (assign104130_e156250 * locals.var_vddpz_dn0)) / assign104130_e156255), ((((locals.var_car * (-locals.var_t5_dn2)) * locals.var_vddpz) + (assign104130_e156250 * locals.var_vddpz_dn2)) / assign104130_e156255), ((((locals.var_car * (-locals.var_t5_dn4)) * locals.var_vddpz) + (assign104130_e156250 * locals.var_vddpz_dn4)) / assign104130_e156255), ((((locals.var_car * (-locals.var_t5_dn5)) * locals.var_vddpz) + (assign104130_e156250 * locals.var_vddpz_dn5)) / assign104130_e156255), ((((locals.var_car * (-locals.var_t5_dn6)) * locals.var_vddpz) + (assign104130_e156250 * locals.var_vddpz_dn6)) / assign104130_e156255), ((((locals.var_car * (-locals.var_t5_dn7)) * locals.var_vddpz) + (assign104130_e156250 * locals.var_vddpz_dn7)) / assign104130_e156255), ((((locals.var_car * (-locals.var_t5_dn8)) * locals.var_vddpz) + (assign104130_e156250 * locals.var_vddpz_dn8)) / assign104130_e156255), ((((locals.var_car * (-locals.var_t5_dn9)) * locals.var_vddpz) + (assign104130_e156250 * locals.var_vddpz_dn9)) / assign104130_e156255), ((((locals.var_car * (-locals.var_t5_dn10)) * locals.var_vddpz) + (assign104130_e156250 * locals.var_vddpz_dn10)) / assign104130_e156255), ((((locals.var_car * (-locals.var_t5_dn11)) * locals.var_vddpz) + (assign104130_e156250 * locals.var_vddpz_dn11)) / assign104130_e156255), ((((locals.var_car * (-locals.var_t5_dn14)) * locals.var_vddpz) + (assign104130_e156250 * locals.var_vddpz_dn14)) / assign104130_e156255),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign104130_e156259;
        locals.var_t4_dn0 = assign104130_e156259_d_n0;
        locals.var_t4_dn2 = assign104130_e156259_d_n2;
        locals.var_t4_dn4 = assign104130_e156259_d_n4;
        locals.var_t4_dn5 = assign104130_e156259_d_n5;
        locals.var_t4_dn6 = assign104130_e156259_d_n6;
        locals.var_t4_dn7 = assign104130_e156259_d_n7;
        locals.var_t4_dn8 = assign104130_e156259_d_n8;
        locals.var_t4_dn9 = assign104130_e156259_d_n9;
        locals.var_t4_dn10 = assign104130_e156259_d_n10;
        locals.var_t4_dn11 = assign104130_e156259_d_n11;
        locals.var_t4_dn14 = assign104130_e156259_d_n14;

        let (assign104140_e156270, assign104140_e156270_d_n0, assign104140_e156270_d_n2, assign104140_e156270_d_n4, assign104140_e156270_d_n5, assign104140_e156270_d_n6, assign104140_e156270_d_n7, assign104140_e156270_d_n8, assign104140_e156270_d_n9, assign104140_e156270_d_n10, assign104140_e156270_d_n11, assign104140_e156270_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104140_e156266: f64 = locals.var_t4;
        let assign104140_e156268: f64 = (assign104140_e156266 - 0.001);
        (assign104140_e156268, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign104140_e156270;
        locals.var_tmf1_dn0 = assign104140_e156270_d_n0;
        locals.var_tmf1_dn2 = assign104140_e156270_d_n2;
        locals.var_tmf1_dn4 = assign104140_e156270_d_n4;
        locals.var_tmf1_dn5 = assign104140_e156270_d_n5;
        locals.var_tmf1_dn6 = assign104140_e156270_d_n6;
        locals.var_tmf1_dn7 = assign104140_e156270_d_n7;
        locals.var_tmf1_dn8 = assign104140_e156270_d_n8;
        locals.var_tmf1_dn9 = assign104140_e156270_d_n9;
        locals.var_tmf1_dn10 = assign104140_e156270_d_n10;
        locals.var_tmf1_dn11 = assign104140_e156270_d_n11;
        locals.var_tmf1_dn14 = assign104140_e156270_d_n14;

        let (assign104150_e156281, assign104150_e156281_d_n0, assign104150_e156281_d_n2, assign104150_e156281_d_n4, assign104150_e156281_d_n5, assign104150_e156281_d_n6, assign104150_e156281_d_n7, assign104150_e156281_d_n8, assign104150_e156281_d_n9, assign104150_e156281_d_n10, assign104150_e156281_d_n11, assign104150_e156281_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign104150_e156281;
        locals.var_tmf2_dn0 = assign104150_e156281_d_n0;
        locals.var_tmf2_dn2 = assign104150_e156281_d_n2;
        locals.var_tmf2_dn4 = assign104150_e156281_d_n4;
        locals.var_tmf2_dn5 = assign104150_e156281_d_n5;
        locals.var_tmf2_dn6 = assign104150_e156281_d_n6;
        locals.var_tmf2_dn7 = assign104150_e156281_d_n7;
        locals.var_tmf2_dn8 = assign104150_e156281_d_n8;
        locals.var_tmf2_dn9 = assign104150_e156281_d_n9;
        locals.var_tmf2_dn10 = assign104150_e156281_d_n10;
        locals.var_tmf2_dn11 = assign104150_e156281_d_n11;
        locals.var_tmf2_dn14 = assign104150_e156281_d_n14;

        let (assign104160_e156294, assign104160_e156294_d_n0, assign104160_e156294_d_n2, assign104160_e156294_d_n4, assign104160_e156294_d_n5, assign104160_e156294_d_n6, assign104160_e156294_d_n7, assign104160_e156294_d_n8, assign104160_e156294_d_n9, assign104160_e156294_d_n10, assign104160_e156294_d_n11, assign104160_e156294_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let (assign104160_e156292, assign104160_e156292_d_n0, assign104160_e156292_d_n2, assign104160_e156292_d_n4, assign104160_e156292_d_n5, assign104160_e156292_d_n6, assign104160_e156292_d_n7, assign104160_e156292_d_n8, assign104160_e156292_d_n9, assign104160_e156292_d_n10, assign104160_e156292_d_n11, assign104160_e156292_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign104160_e156291: f64 = (-locals.var_tmf2);
                (assign104160_e156291, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign104160_e156292, assign104160_e156292_d_n0, assign104160_e156292_d_n2, assign104160_e156292_d_n4, assign104160_e156292_d_n5, assign104160_e156292_d_n6, assign104160_e156292_d_n7, assign104160_e156292_d_n8, assign104160_e156292_d_n9, assign104160_e156292_d_n10, assign104160_e156292_d_n11, assign104160_e156292_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign104160_e156294;
        locals.var_tmf2_dn0 = assign104160_e156294_d_n0;
        locals.var_tmf2_dn2 = assign104160_e156294_d_n2;
        locals.var_tmf2_dn4 = assign104160_e156294_d_n4;
        locals.var_tmf2_dn5 = assign104160_e156294_d_n5;
        locals.var_tmf2_dn6 = assign104160_e156294_d_n6;
        locals.var_tmf2_dn7 = assign104160_e156294_d_n7;
        locals.var_tmf2_dn8 = assign104160_e156294_d_n8;
        locals.var_tmf2_dn9 = assign104160_e156294_d_n9;
        locals.var_tmf2_dn10 = assign104160_e156294_d_n10;
        locals.var_tmf2_dn11 = assign104160_e156294_d_n11;
        locals.var_tmf2_dn14 = assign104160_e156294_d_n14;

        let (assign104170_e156306, assign104170_e156306_d_n0, assign104170_e156306_d_n2, assign104170_e156306_d_n4, assign104170_e156306_d_n5, assign104170_e156306_d_n6, assign104170_e156306_d_n7, assign104170_e156306_d_n8, assign104170_e156306_d_n9, assign104170_e156306_d_n10, assign104170_e156306_d_n11, assign104170_e156306_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104170_e156301: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign104170_e156303: f64 = (assign104170_e156301 + locals.var_tmf2);
        let assign104170_e156304: f64 = (assign104170_e156303).sqrt();
        (assign104170_e156304, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign104170_e156304)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign104170_e156304)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign104170_e156304)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign104170_e156304)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign104170_e156304)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign104170_e156304)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign104170_e156304)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign104170_e156304)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign104170_e156304)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign104170_e156304)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign104170_e156304)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign104170_e156306;
        locals.var_tmf2_dn0 = assign104170_e156306_d_n0;
        locals.var_tmf2_dn2 = assign104170_e156306_d_n2;
        locals.var_tmf2_dn4 = assign104170_e156306_d_n4;
        locals.var_tmf2_dn5 = assign104170_e156306_d_n5;
        locals.var_tmf2_dn6 = assign104170_e156306_d_n6;
        locals.var_tmf2_dn7 = assign104170_e156306_d_n7;
        locals.var_tmf2_dn8 = assign104170_e156306_d_n8;
        locals.var_tmf2_dn9 = assign104170_e156306_d_n9;
        locals.var_tmf2_dn10 = assign104170_e156306_d_n10;
        locals.var_tmf2_dn11 = assign104170_e156306_d_n11;
        locals.var_tmf2_dn14 = assign104170_e156306_d_n14;

        let (assign104180_e156319, assign104180_e156319_d_n0, assign104180_e156319_d_n2, assign104180_e156319_d_n4, assign104180_e156319_d_n5, assign104180_e156319_d_n6, assign104180_e156319_d_n7, assign104180_e156319_d_n8, assign104180_e156319_d_n9, assign104180_e156319_d_n10, assign104180_e156319_d_n11, assign104180_e156319_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104180_e156315: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign104180_e156316: f64 = (1.0 + assign104180_e156315);
        let assign104180_e156317: f64 = (0.5 * assign104180_e156316);
        (assign104180_e156317, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign104180_e156319;
        locals.var_t0_dn0 = assign104180_e156319_d_n0;
        locals.var_t0_dn2 = assign104180_e156319_d_n2;
        locals.var_t0_dn4 = assign104180_e156319_d_n4;
        locals.var_t0_dn5 = assign104180_e156319_d_n5;
        locals.var_t0_dn6 = assign104180_e156319_d_n6;
        locals.var_t0_dn7 = assign104180_e156319_d_n7;
        locals.var_t0_dn8 = assign104180_e156319_d_n8;
        locals.var_t0_dn9 = assign104180_e156319_d_n9;
        locals.var_t0_dn10 = assign104180_e156319_d_n10;
        locals.var_t0_dn11 = assign104180_e156319_d_n11;
        locals.var_t0_dn14 = assign104180_e156319_d_n14;

        let (assign104190_e156332, assign104190_e156332_d_n0, assign104190_e156332_d_n2, assign104190_e156332_d_n4, assign104190_e156332_d_n5, assign104190_e156332_d_n6, assign104190_e156332_d_n7, assign104190_e156332_d_n8, assign104190_e156332_d_n9, assign104190_e156332_d_n10, assign104190_e156332_d_n11, assign104190_e156332_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104190_e156328: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign104190_e156329: f64 = (0.5 * assign104190_e156328);
        let assign104190_e156330: f64 = assign104190_e156329;
        (assign104190_e156330, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign104190_e156332;
        locals.var_t5_dn0 = assign104190_e156332_d_n0;
        locals.var_t5_dn2 = assign104190_e156332_d_n2;
        locals.var_t5_dn4 = assign104190_e156332_d_n4;
        locals.var_t5_dn5 = assign104190_e156332_d_n5;
        locals.var_t5_dn6 = assign104190_e156332_d_n6;
        locals.var_t5_dn7 = assign104190_e156332_d_n7;
        locals.var_t5_dn8 = assign104190_e156332_d_n8;
        locals.var_t5_dn9 = assign104190_e156332_d_n9;
        locals.var_t5_dn10 = assign104190_e156332_d_n10;
        locals.var_t5_dn11 = assign104190_e156332_d_n11;
        locals.var_t5_dn14 = assign104190_e156332_d_n14;

        let (assign104200_e156341, assign104200_e156341_d_n0, assign104200_e156341_d_n2, assign104200_e156341_d_n4, assign104200_e156341_d_n5, assign104200_e156341_d_n6, assign104200_e156341_d_n7, assign104200_e156341_d_n8, assign104200_e156341_d_n9, assign104200_e156341_d_n10, assign104200_e156341_d_n11, assign104200_e156341_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104200_e156339: f64 = (locals.var_noverd * locals.var_t5);
        (assign104200_e156339, (locals.var_noverd * locals.var_t5_dn0), (locals.var_noverd * locals.var_t5_dn2), (locals.var_noverd * locals.var_t5_dn4), (locals.var_noverd * locals.var_t5_dn5), (locals.var_noverd * locals.var_t5_dn6), (locals.var_noverd * locals.var_t5_dn7), (locals.var_noverd * locals.var_t5_dn8), (locals.var_noverd * locals.var_t5_dn9), (locals.var_noverd * locals.var_t5_dn10), (locals.var_noverd * locals.var_t5_dn11), (locals.var_noverd * locals.var_t5_dn14),)
    } else {
        (locals.var_carr1, locals.var_carr1_dn0, locals.var_carr1_dn2, locals.var_carr1_dn4, locals.var_carr1_dn5, locals.var_carr1_dn6, locals.var_carr1_dn7, locals.var_carr1_dn8, locals.var_carr1_dn9, locals.var_carr1_dn10, locals.var_carr1_dn11, locals.var_carr1_dn14,)
    }
};
        locals.var_carr1 = assign104200_e156341;
        locals.var_carr1_dn0 = assign104200_e156341_d_n0;
        locals.var_carr1_dn2 = assign104200_e156341_d_n2;
        locals.var_carr1_dn4 = assign104200_e156341_d_n4;
        locals.var_carr1_dn5 = assign104200_e156341_d_n5;
        locals.var_carr1_dn6 = assign104200_e156341_d_n6;
        locals.var_carr1_dn7 = assign104200_e156341_d_n7;
        locals.var_carr1_dn8 = assign104200_e156341_d_n8;
        locals.var_carr1_dn9 = assign104200_e156341_d_n9;
        locals.var_carr1_dn10 = assign104200_e156341_d_n10;
        locals.var_carr1_dn11 = assign104200_e156341_d_n11;
        locals.var_carr1_dn14 = assign104200_e156341_d_n14;

        let (assign104210_e156352, assign104210_e156352_d_n0, assign104210_e156352_d_n2, assign104210_e156352_d_n4, assign104210_e156352_d_n5, assign104210_e156352_d_n6, assign104210_e156352_d_n7, assign104210_e156352_d_n8, assign104210_e156352_d_n9, assign104210_e156352_d_n10, assign104210_e156352_d_n11, assign104210_e156352_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104210_e156348: f64 = (locals.var_rd_qbuld / 1.6021918e-19);
        let assign104210_e156350: f64 = (assign104210_e156348 * p.p430);
        (assign104210_e156350, ((locals.var_rd_qbuld_dn0 / 1.6021918e-19) * p.p430), ((locals.var_rd_qbuld_dn2 / 1.6021918e-19) * p.p430), ((locals.var_rd_qbuld_dn4 / 1.6021918e-19) * p.p430), ((locals.var_rd_qbuld_dn5 / 1.6021918e-19) * p.p430), ((locals.var_rd_qbuld_dn6 / 1.6021918e-19) * p.p430), ((locals.var_rd_qbuld_dn7 / 1.6021918e-19) * p.p430), ((locals.var_rd_qbuld_dn8 / 1.6021918e-19) * p.p430), ((locals.var_rd_qbuld_dn9 / 1.6021918e-19) * p.p430), ((locals.var_rd_qbuld_dn10 / 1.6021918e-19) * p.p430), ((locals.var_rd_qbuld_dn11 / 1.6021918e-19) * p.p430), ((locals.var_rd_qbuld_dn14 / 1.6021918e-19) * p.p430),)
    } else {
        (locals.var_carr2, locals.var_carr2_dn0, locals.var_carr2_dn2, locals.var_carr2_dn4, locals.var_carr2_dn5, locals.var_carr2_dn6, locals.var_carr2_dn7, locals.var_carr2_dn8, locals.var_carr2_dn9, locals.var_carr2_dn10, locals.var_carr2_dn11, locals.var_carr2_dn14,)
    }
};
        locals.var_carr2 = assign104210_e156352;
        locals.var_carr2_dn0 = assign104210_e156352_d_n0;
        locals.var_carr2_dn2 = assign104210_e156352_d_n2;
        locals.var_carr2_dn4 = assign104210_e156352_d_n4;
        locals.var_carr2_dn5 = assign104210_e156352_d_n5;
        locals.var_carr2_dn6 = assign104210_e156352_d_n6;
        locals.var_carr2_dn7 = assign104210_e156352_d_n7;
        locals.var_carr2_dn8 = assign104210_e156352_d_n8;
        locals.var_carr2_dn9 = assign104210_e156352_d_n9;
        locals.var_carr2_dn10 = assign104210_e156352_d_n10;
        locals.var_carr2_dn11 = assign104210_e156352_d_n11;
        locals.var_carr2_dn14 = assign104210_e156352_d_n14;

        let (assign104220_e156365, assign104220_e156365_d_n0, assign104220_e156365_d_n2, assign104220_e156365_d_n4, assign104220_e156365_d_n5, assign104220_e156365_d_n6, assign104220_e156365_d_n7, assign104220_e156365_d_n8, assign104220_e156365_d_n9, assign104220_e156365_d_n10, assign104220_e156365_d_n11, assign104220_e156365_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104220_e156359: f64 = (locals.var_carr1 - locals.var_carr2);
        let assign104220_e156362: f64 = (locals.var_carr1 * 0.001);
        let assign104220_e156363: f64 = (assign104220_e156359 - assign104220_e156362);
        (assign104220_e156363, ((locals.var_carr1_dn0 - locals.var_carr2_dn0) - (locals.var_carr1_dn0 * 0.001)), ((locals.var_carr1_dn2 - locals.var_carr2_dn2) - (locals.var_carr1_dn2 * 0.001)), ((locals.var_carr1_dn4 - locals.var_carr2_dn4) - (locals.var_carr1_dn4 * 0.001)), ((locals.var_carr1_dn5 - locals.var_carr2_dn5) - (locals.var_carr1_dn5 * 0.001)), ((locals.var_carr1_dn6 - locals.var_carr2_dn6) - (locals.var_carr1_dn6 * 0.001)), ((locals.var_carr1_dn7 - locals.var_carr2_dn7) - (locals.var_carr1_dn7 * 0.001)), ((locals.var_carr1_dn8 - locals.var_carr2_dn8) - (locals.var_carr1_dn8 * 0.001)), ((locals.var_carr1_dn9 - locals.var_carr2_dn9) - (locals.var_carr1_dn9 * 0.001)), ((locals.var_carr1_dn10 - locals.var_carr2_dn10) - (locals.var_carr1_dn10 * 0.001)), ((locals.var_carr1_dn11 - locals.var_carr2_dn11) - (locals.var_carr1_dn11 * 0.001)), ((locals.var_carr1_dn14 - locals.var_carr2_dn14) - (locals.var_carr1_dn14 * 0.001)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign104220_e156365;
        locals.var_tmf1_dn0 = assign104220_e156365_d_n0;
        locals.var_tmf1_dn2 = assign104220_e156365_d_n2;
        locals.var_tmf1_dn4 = assign104220_e156365_d_n4;
        locals.var_tmf1_dn5 = assign104220_e156365_d_n5;
        locals.var_tmf1_dn6 = assign104220_e156365_d_n6;
        locals.var_tmf1_dn7 = assign104220_e156365_d_n7;
        locals.var_tmf1_dn8 = assign104220_e156365_d_n8;
        locals.var_tmf1_dn9 = assign104220_e156365_d_n9;
        locals.var_tmf1_dn10 = assign104220_e156365_d_n10;
        locals.var_tmf1_dn11 = assign104220_e156365_d_n11;
        locals.var_tmf1_dn14 = assign104220_e156365_d_n14;

        let (assign104230_e156378, assign104230_e156378_d_n0, assign104230_e156378_d_n2, assign104230_e156378_d_n4, assign104230_e156378_d_n5, assign104230_e156378_d_n6, assign104230_e156378_d_n7, assign104230_e156378_d_n8, assign104230_e156378_d_n9, assign104230_e156378_d_n10, assign104230_e156378_d_n11, assign104230_e156378_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104230_e156372: f64 = (4.0 * locals.var_carr1);
        let assign104230_e156375: f64 = (locals.var_carr1 * 0.001);
        let assign104230_e156376: f64 = (assign104230_e156372 * assign104230_e156375);
        (assign104230_e156376, (((4.0 * locals.var_carr1_dn0) * assign104230_e156375) + (assign104230_e156372 * (locals.var_carr1_dn0 * 0.001))), (((4.0 * locals.var_carr1_dn2) * assign104230_e156375) + (assign104230_e156372 * (locals.var_carr1_dn2 * 0.001))), (((4.0 * locals.var_carr1_dn4) * assign104230_e156375) + (assign104230_e156372 * (locals.var_carr1_dn4 * 0.001))), (((4.0 * locals.var_carr1_dn5) * assign104230_e156375) + (assign104230_e156372 * (locals.var_carr1_dn5 * 0.001))), (((4.0 * locals.var_carr1_dn6) * assign104230_e156375) + (assign104230_e156372 * (locals.var_carr1_dn6 * 0.001))), (((4.0 * locals.var_carr1_dn7) * assign104230_e156375) + (assign104230_e156372 * (locals.var_carr1_dn7 * 0.001))), (((4.0 * locals.var_carr1_dn8) * assign104230_e156375) + (assign104230_e156372 * (locals.var_carr1_dn8 * 0.001))), (((4.0 * locals.var_carr1_dn9) * assign104230_e156375) + (assign104230_e156372 * (locals.var_carr1_dn9 * 0.001))), (((4.0 * locals.var_carr1_dn10) * assign104230_e156375) + (assign104230_e156372 * (locals.var_carr1_dn10 * 0.001))), (((4.0 * locals.var_carr1_dn11) * assign104230_e156375) + (assign104230_e156372 * (locals.var_carr1_dn11 * 0.001))), (((4.0 * locals.var_carr1_dn14) * assign104230_e156375) + (assign104230_e156372 * (locals.var_carr1_dn14 * 0.001))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign104230_e156378;
        locals.var_tmf2_dn0 = assign104230_e156378_d_n0;
        locals.var_tmf2_dn2 = assign104230_e156378_d_n2;
        locals.var_tmf2_dn4 = assign104230_e156378_d_n4;
        locals.var_tmf2_dn5 = assign104230_e156378_d_n5;
        locals.var_tmf2_dn6 = assign104230_e156378_d_n6;
        locals.var_tmf2_dn7 = assign104230_e156378_d_n7;
        locals.var_tmf2_dn8 = assign104230_e156378_d_n8;
        locals.var_tmf2_dn9 = assign104230_e156378_d_n9;
        locals.var_tmf2_dn10 = assign104230_e156378_d_n10;
        locals.var_tmf2_dn11 = assign104230_e156378_d_n11;
        locals.var_tmf2_dn14 = assign104230_e156378_d_n14;

        let (assign104240_e156391, assign104240_e156391_d_n0, assign104240_e156391_d_n2, assign104240_e156391_d_n4, assign104240_e156391_d_n5, assign104240_e156391_d_n6, assign104240_e156391_d_n7, assign104240_e156391_d_n8, assign104240_e156391_d_n9, assign104240_e156391_d_n10, assign104240_e156391_d_n11, assign104240_e156391_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let (assign104240_e156389, assign104240_e156389_d_n0, assign104240_e156389_d_n2, assign104240_e156389_d_n4, assign104240_e156389_d_n5, assign104240_e156389_d_n6, assign104240_e156389_d_n7, assign104240_e156389_d_n8, assign104240_e156389_d_n9, assign104240_e156389_d_n10, assign104240_e156389_d_n11, assign104240_e156389_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign104240_e156388: f64 = (-locals.var_tmf2);
                (assign104240_e156388, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign104240_e156389, assign104240_e156389_d_n0, assign104240_e156389_d_n2, assign104240_e156389_d_n4, assign104240_e156389_d_n5, assign104240_e156389_d_n6, assign104240_e156389_d_n7, assign104240_e156389_d_n8, assign104240_e156389_d_n9, assign104240_e156389_d_n10, assign104240_e156389_d_n11, assign104240_e156389_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign104240_e156391;
        locals.var_tmf2_dn0 = assign104240_e156391_d_n0;
        locals.var_tmf2_dn2 = assign104240_e156391_d_n2;
        locals.var_tmf2_dn4 = assign104240_e156391_d_n4;
        locals.var_tmf2_dn5 = assign104240_e156391_d_n5;
        locals.var_tmf2_dn6 = assign104240_e156391_d_n6;
        locals.var_tmf2_dn7 = assign104240_e156391_d_n7;
        locals.var_tmf2_dn8 = assign104240_e156391_d_n8;
        locals.var_tmf2_dn9 = assign104240_e156391_d_n9;
        locals.var_tmf2_dn10 = assign104240_e156391_d_n10;
        locals.var_tmf2_dn11 = assign104240_e156391_d_n11;
        locals.var_tmf2_dn14 = assign104240_e156391_d_n14;

        let (assign104250_e156403, assign104250_e156403_d_n0, assign104250_e156403_d_n2, assign104250_e156403_d_n4, assign104250_e156403_d_n5, assign104250_e156403_d_n6, assign104250_e156403_d_n7, assign104250_e156403_d_n8, assign104250_e156403_d_n9, assign104250_e156403_d_n10, assign104250_e156403_d_n11, assign104250_e156403_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104250_e156398: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign104250_e156400: f64 = (assign104250_e156398 + locals.var_tmf2);
        let assign104250_e156401: f64 = (assign104250_e156400).sqrt();
        (assign104250_e156401, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign104250_e156401)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign104250_e156401)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign104250_e156401)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign104250_e156401)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign104250_e156401)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign104250_e156401)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign104250_e156401)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign104250_e156401)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign104250_e156401)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign104250_e156401)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign104250_e156401)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign104250_e156403;
        locals.var_tmf2_dn0 = assign104250_e156403_d_n0;
        locals.var_tmf2_dn2 = assign104250_e156403_d_n2;
        locals.var_tmf2_dn4 = assign104250_e156403_d_n4;
        locals.var_tmf2_dn5 = assign104250_e156403_d_n5;
        locals.var_tmf2_dn6 = assign104250_e156403_d_n6;
        locals.var_tmf2_dn7 = assign104250_e156403_d_n7;
        locals.var_tmf2_dn8 = assign104250_e156403_d_n8;
        locals.var_tmf2_dn9 = assign104250_e156403_d_n9;
        locals.var_tmf2_dn10 = assign104250_e156403_d_n10;
        locals.var_tmf2_dn11 = assign104250_e156403_d_n11;
        locals.var_tmf2_dn14 = assign104250_e156403_d_n14;

        let (assign104260_e156416, assign104260_e156416_d_n0, assign104260_e156416_d_n2, assign104260_e156416_d_n4, assign104260_e156416_d_n5, assign104260_e156416_d_n6, assign104260_e156416_d_n7, assign104260_e156416_d_n8, assign104260_e156416_d_n9, assign104260_e156416_d_n10, assign104260_e156416_d_n11, assign104260_e156416_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104260_e156412: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign104260_e156413: f64 = (1.0 + assign104260_e156412);
        let assign104260_e156414: f64 = (0.5 * assign104260_e156413);
        (assign104260_e156414, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign104260_e156416;
        locals.var_t0_dn0 = assign104260_e156416_d_n0;
        locals.var_t0_dn2 = assign104260_e156416_d_n2;
        locals.var_t0_dn4 = assign104260_e156416_d_n4;
        locals.var_t0_dn5 = assign104260_e156416_d_n5;
        locals.var_t0_dn6 = assign104260_e156416_d_n6;
        locals.var_t0_dn7 = assign104260_e156416_d_n7;
        locals.var_t0_dn8 = assign104260_e156416_d_n8;
        locals.var_t0_dn9 = assign104260_e156416_d_n9;
        locals.var_t0_dn10 = assign104260_e156416_d_n10;
        locals.var_t0_dn11 = assign104260_e156416_d_n11;
        locals.var_t0_dn14 = assign104260_e156416_d_n14;

        let (assign104270_e156429, assign104270_e156429_d_n0, assign104270_e156429_d_n2, assign104270_e156429_d_n4, assign104270_e156429_d_n5, assign104270_e156429_d_n6, assign104270_e156429_d_n7, assign104270_e156429_d_n8, assign104270_e156429_d_n9, assign104270_e156429_d_n10, assign104270_e156429_d_n11, assign104270_e156429_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104270_e156425: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign104270_e156426: f64 = (0.5 * assign104270_e156425);
        let assign104270_e156427: f64 = (locals.var_carr1 - assign104270_e156426);
        (assign104270_e156427, (locals.var_carr1_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_carr1_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_carr1_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_carr1_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_carr1_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_carr1_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_carr1_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_carr1_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_carr1_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_carr1_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_carr1_dn14 - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_carr2, locals.var_carr2_dn0, locals.var_carr2_dn2, locals.var_carr2_dn4, locals.var_carr2_dn5, locals.var_carr2_dn6, locals.var_carr2_dn7, locals.var_carr2_dn8, locals.var_carr2_dn9, locals.var_carr2_dn10, locals.var_carr2_dn11, locals.var_carr2_dn14,)
    }
};
        locals.var_carr2 = assign104270_e156429;
        locals.var_carr2_dn0 = assign104270_e156429_d_n0;
        locals.var_carr2_dn2 = assign104270_e156429_d_n2;
        locals.var_carr2_dn4 = assign104270_e156429_d_n4;
        locals.var_carr2_dn5 = assign104270_e156429_d_n5;
        locals.var_carr2_dn6 = assign104270_e156429_d_n6;
        locals.var_carr2_dn7 = assign104270_e156429_d_n7;
        locals.var_carr2_dn8 = assign104270_e156429_d_n8;
        locals.var_carr2_dn9 = assign104270_e156429_d_n9;
        locals.var_carr2_dn10 = assign104270_e156429_d_n10;
        locals.var_carr2_dn11 = assign104270_e156429_d_n11;
        locals.var_carr2_dn14 = assign104270_e156429_d_n14;

        let (assign104280_e156438, assign104280_e156438_d_n0, assign104280_e156438_d_n2, assign104280_e156438_d_n4, assign104280_e156438_d_n5, assign104280_e156438_d_n6, assign104280_e156438_d_n7, assign104280_e156438_d_n8, assign104280_e156438_d_n9, assign104280_e156438_d_n10, assign104280_e156438_d_n11, assign104280_e156438_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104280_e156436: f64 = (locals.var_carr1 - locals.var_carr2);
        (assign104280_e156436, (locals.var_carr1_dn0 - locals.var_carr2_dn0), (locals.var_carr1_dn2 - locals.var_carr2_dn2), (locals.var_carr1_dn4 - locals.var_carr2_dn4), (locals.var_carr1_dn5 - locals.var_carr2_dn5), (locals.var_carr1_dn6 - locals.var_carr2_dn6), (locals.var_carr1_dn7 - locals.var_carr2_dn7), (locals.var_carr1_dn8 - locals.var_carr2_dn8), (locals.var_carr1_dn9 - locals.var_carr2_dn9), (locals.var_carr1_dn10 - locals.var_carr2_dn10), (locals.var_carr1_dn11 - locals.var_carr2_dn11), (locals.var_carr1_dn14 - locals.var_carr2_dn14),)
    } else {
        (locals.var_carr, locals.var_carr_dn0, locals.var_carr_dn2, locals.var_carr_dn4, locals.var_carr_dn5, locals.var_carr_dn6, locals.var_carr_dn7, locals.var_carr_dn8, locals.var_carr_dn9, locals.var_carr_dn10, locals.var_carr_dn11, locals.var_carr_dn14,)
    }
};
        locals.var_carr = assign104280_e156438;
        locals.var_carr_dn0 = assign104280_e156438_d_n0;
        locals.var_carr_dn2 = assign104280_e156438_d_n2;
        locals.var_carr_dn4 = assign104280_e156438_d_n4;
        locals.var_carr_dn5 = assign104280_e156438_d_n5;
        locals.var_carr_dn6 = assign104280_e156438_d_n6;
        locals.var_carr_dn7 = assign104280_e156438_d_n7;
        locals.var_carr_dn8 = assign104280_e156438_d_n8;
        locals.var_carr_dn9 = assign104280_e156438_d_n9;
        locals.var_carr_dn10 = assign104280_e156438_d_n10;
        locals.var_carr_dn11 = assign104280_e156438_d_n11;
        locals.var_carr_dn14 = assign104280_e156438_d_n14;

        let assign104290_e156445: f64 = if ((p.p441 > 0.0) && (p.p440 > 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard2373 = assign104290_e156445;

        let assign104300_e156449: f64 = (locals.var_noverd * p.p440);
        let assign104300_e156452: f64 = (locals.var_noverd * p.p441);
        let assign104300_e156453: f64 = (assign104300_e156449 - assign104300_e156452);
        let assign104300_e156457: f64 = (locals.var_noverd * p.p441);
        let assign104300_e156460: f64 = if ((locals.var_carr > assign104300_e156453) && (assign104300_e156457 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2374 = assign104300_e156460;

        let (assign104310_e156479, assign104310_e156479_d_n0, assign104310_e156479_d_n2, assign104310_e156479_d_n4, assign104310_e156479_d_n5, assign104310_e156479_d_n6, assign104310_e156479_d_n7, assign104310_e156479_d_n8, assign104310_e156479_d_n9, assign104310_e156479_d_n10, assign104310_e156479_d_n11, assign104310_e156479_d_n14,) = {
    if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_guard2374 != 0.0)) {
        let assign104310_e156472: f64 = (locals.var_noverd * p.p440);
        let assign104310_e156473: f64 = (locals.var_carr - assign104310_e156472);
        let assign104310_e156476: f64 = (locals.var_noverd * p.p441);
        let assign104310_e156477: f64 = (assign104310_e156473 + assign104310_e156476);
        (assign104310_e156477, locals.var_carr_dn0, locals.var_carr_dn2, locals.var_carr_dn4, locals.var_carr_dn5, locals.var_carr_dn6, locals.var_carr_dn7, locals.var_carr_dn8, locals.var_carr_dn9, locals.var_carr_dn10, locals.var_carr_dn11, locals.var_carr_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign104310_e156479;
        locals.var_tmf1_dn0 = assign104310_e156479_d_n0;
        locals.var_tmf1_dn2 = assign104310_e156479_d_n2;
        locals.var_tmf1_dn4 = assign104310_e156479_d_n4;
        locals.var_tmf1_dn5 = assign104310_e156479_d_n5;
        locals.var_tmf1_dn6 = assign104310_e156479_d_n6;
        locals.var_tmf1_dn7 = assign104310_e156479_d_n7;
        locals.var_tmf1_dn8 = assign104310_e156479_d_n8;
        locals.var_tmf1_dn9 = assign104310_e156479_d_n9;
        locals.var_tmf1_dn10 = assign104310_e156479_d_n10;
        locals.var_tmf1_dn11 = assign104310_e156479_d_n11;
        locals.var_tmf1_dn14 = assign104310_e156479_d_n14;

        let (assign104320_e156492, assign104320_e156492_d_n0, assign104320_e156492_d_n2, assign104320_e156492_d_n4, assign104320_e156492_d_n5, assign104320_e156492_d_n6, assign104320_e156492_d_n7, assign104320_e156492_d_n8, assign104320_e156492_d_n9, assign104320_e156492_d_n10, assign104320_e156492_d_n11, assign104320_e156492_d_n14,) = {
    if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_guard2374 != 0.0)) {
        let assign104320_e156490: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign104320_e156490, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign104320_e156492;
        locals.var_x2_dn0 = assign104320_e156492_d_n0;
        locals.var_x2_dn2 = assign104320_e156492_d_n2;
        locals.var_x2_dn4 = assign104320_e156492_d_n4;
        locals.var_x2_dn5 = assign104320_e156492_d_n5;
        locals.var_x2_dn6 = assign104320_e156492_d_n6;
        locals.var_x2_dn7 = assign104320_e156492_d_n7;
        locals.var_x2_dn8 = assign104320_e156492_d_n8;
        locals.var_x2_dn9 = assign104320_e156492_d_n9;
        locals.var_x2_dn10 = assign104320_e156492_d_n10;
        locals.var_x2_dn11 = assign104320_e156492_d_n11;
        locals.var_x2_dn14 = assign104320_e156492_d_n14;

    }

    pub(super) fn stamp_transient_block_381(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign104330_e156509, assign104330_e156509_d_n0, assign104330_e156509_d_n2, assign104330_e156509_d_n4, assign104330_e156509_d_n5, assign104330_e156509_d_n6, assign104330_e156509_d_n7, assign104330_e156509_d_n8, assign104330_e156509_d_n9, assign104330_e156509_d_n10, assign104330_e156509_d_n11, assign104330_e156509_d_n14,) = {
    if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_guard2374 != 0.0)) {
        let assign104330_e156503: f64 = (locals.var_noverd * p.p441);
        let assign104330_e156506: f64 = (locals.var_noverd * p.p441);
        let assign104330_e156507: f64 = (assign104330_e156503 * assign104330_e156506);
        (assign104330_e156507, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign104330_e156509;
        locals.var_xmax2_dn0 = assign104330_e156509_d_n0;
        locals.var_xmax2_dn2 = assign104330_e156509_d_n2;
        locals.var_xmax2_dn4 = assign104330_e156509_d_n4;
        locals.var_xmax2_dn5 = assign104330_e156509_d_n5;
        locals.var_xmax2_dn6 = assign104330_e156509_d_n6;
        locals.var_xmax2_dn7 = assign104330_e156509_d_n7;
        locals.var_xmax2_dn8 = assign104330_e156509_d_n8;
        locals.var_xmax2_dn9 = assign104330_e156509_d_n9;
        locals.var_xmax2_dn10 = assign104330_e156509_d_n10;
        locals.var_xmax2_dn11 = assign104330_e156509_d_n11;
        locals.var_xmax2_dn14 = assign104330_e156509_d_n14;

        let (assign104340_e156520, assign104340_e156520_d_n0, assign104340_e156520_d_n2, assign104340_e156520_d_n4, assign104340_e156520_d_n5, assign104340_e156520_d_n6, assign104340_e156520_d_n7, assign104340_e156520_d_n8, assign104340_e156520_d_n9, assign104340_e156520_d_n10, assign104340_e156520_d_n11, assign104340_e156520_d_n14,) = {
    if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_guard2374 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign104340_e156520;
        locals.var_xp_dn0 = assign104340_e156520_d_n0;
        locals.var_xp_dn2 = assign104340_e156520_d_n2;
        locals.var_xp_dn4 = assign104340_e156520_d_n4;
        locals.var_xp_dn5 = assign104340_e156520_d_n5;
        locals.var_xp_dn6 = assign104340_e156520_d_n6;
        locals.var_xp_dn7 = assign104340_e156520_d_n7;
        locals.var_xp_dn8 = assign104340_e156520_d_n8;
        locals.var_xp_dn9 = assign104340_e156520_d_n9;
        locals.var_xp_dn10 = assign104340_e156520_d_n10;
        locals.var_xp_dn11 = assign104340_e156520_d_n11;
        locals.var_xp_dn14 = assign104340_e156520_d_n14;

        let (assign104350_e156531, assign104350_e156531_d_n0, assign104350_e156531_d_n2, assign104350_e156531_d_n4, assign104350_e156531_d_n5, assign104350_e156531_d_n6, assign104350_e156531_d_n7, assign104350_e156531_d_n8, assign104350_e156531_d_n9, assign104350_e156531_d_n10, assign104350_e156531_d_n11, assign104350_e156531_d_n14,) = {
    if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_guard2374 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign104350_e156531;
        locals.var_xmp_dn0 = assign104350_e156531_d_n0;
        locals.var_xmp_dn2 = assign104350_e156531_d_n2;
        locals.var_xmp_dn4 = assign104350_e156531_d_n4;
        locals.var_xmp_dn5 = assign104350_e156531_d_n5;
        locals.var_xmp_dn6 = assign104350_e156531_d_n6;
        locals.var_xmp_dn7 = assign104350_e156531_d_n7;
        locals.var_xmp_dn8 = assign104350_e156531_d_n8;
        locals.var_xmp_dn9 = assign104350_e156531_d_n9;
        locals.var_xmp_dn10 = assign104350_e156531_d_n10;
        locals.var_xmp_dn11 = assign104350_e156531_d_n11;
        locals.var_xmp_dn14 = assign104350_e156531_d_n14;

        let (assign104360_e156542,) = {
    if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_guard2374 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign104360_e156542;

        let (assign104370_e156553,) = {
    if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_guard2374 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign104370_e156553;

        let (assign104380_e156564, assign104380_e156564_d_n0, assign104380_e156564_d_n2, assign104380_e156564_d_n4, assign104380_e156564_d_n5, assign104380_e156564_d_n6, assign104380_e156564_d_n7, assign104380_e156564_d_n8, assign104380_e156564_d_n9, assign104380_e156564_d_n10, assign104380_e156564_d_n11, assign104380_e156564_d_n14,) = {
    if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_guard2374 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign104380_e156564;
        locals.var_arg_dn0 = assign104380_e156564_d_n0;
        locals.var_arg_dn2 = assign104380_e156564_d_n2;
        locals.var_arg_dn4 = assign104380_e156564_d_n4;
        locals.var_arg_dn5 = assign104380_e156564_d_n5;
        locals.var_arg_dn6 = assign104380_e156564_d_n6;
        locals.var_arg_dn7 = assign104380_e156564_d_n7;
        locals.var_arg_dn8 = assign104380_e156564_d_n8;
        locals.var_arg_dn9 = assign104380_e156564_d_n9;
        locals.var_arg_dn10 = assign104380_e156564_d_n10;
        locals.var_arg_dn11 = assign104380_e156564_d_n11;
        locals.var_arg_dn14 = assign104380_e156564_d_n14;

        let (assign104390_e156575, assign104390_e156575_d_n0, assign104390_e156575_d_n2, assign104390_e156575_d_n4, assign104390_e156575_d_n5, assign104390_e156575_d_n6, assign104390_e156575_d_n7, assign104390_e156575_d_n8, assign104390_e156575_d_n9, assign104390_e156575_d_n10, assign104390_e156575_d_n11, assign104390_e156575_d_n14,) = {
    if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_guard2374 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign104390_e156575;
        locals.var_dnm_dn0 = assign104390_e156575_d_n0;
        locals.var_dnm_dn2 = assign104390_e156575_d_n2;
        locals.var_dnm_dn4 = assign104390_e156575_d_n4;
        locals.var_dnm_dn5 = assign104390_e156575_d_n5;
        locals.var_dnm_dn6 = assign104390_e156575_d_n6;
        locals.var_dnm_dn7 = assign104390_e156575_d_n7;
        locals.var_dnm_dn8 = assign104390_e156575_d_n8;
        locals.var_dnm_dn9 = assign104390_e156575_d_n9;
        locals.var_dnm_dn10 = assign104390_e156575_d_n10;
        locals.var_dnm_dn11 = assign104390_e156575_d_n11;
        locals.var_dnm_dn14 = assign104390_e156575_d_n14;

        let (assign104400_e156586,) = {
    if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_guard2374 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign104400_e156586;

        let mut assign104410_loop_guard: usize = 0;
        while {
            let assign104410_cond_e156598: f64 = if (((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_guard2374 != 0.0)) && (locals.var_m0 < p.p442)) { 1.0 } else { 0.0 };
            assign104410_cond_e156598 != 0.0
        } {
            assign104410_loop_guard += 1;
            assert!(assign104410_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign104410_body0_e156611, assign104410_body0_e156611_d_n0, assign104410_body0_e156611_d_n2, assign104410_body0_e156611_d_n4, assign104410_body0_e156611_d_n5, assign104410_body0_e156611_d_n6, assign104410_body0_e156611_d_n7, assign104410_body0_e156611_d_n8, assign104410_body0_e156611_d_n9, assign104410_body0_e156611_d_n10, assign104410_body0_e156611_d_n11, assign104410_body0_e156611_d_n14,) = {
    if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_guard2374 != 0.0)) {
        let assign104410_body0_e156609: f64 = (locals.var_xp * locals.var_x2);
        (assign104410_body0_e156609, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
            locals.var_xp = assign104410_body0_e156611;
            locals.var_xp_dn0 = assign104410_body0_e156611_d_n0;
            locals.var_xp_dn2 = assign104410_body0_e156611_d_n2;
            locals.var_xp_dn4 = assign104410_body0_e156611_d_n4;
            locals.var_xp_dn5 = assign104410_body0_e156611_d_n5;
            locals.var_xp_dn6 = assign104410_body0_e156611_d_n6;
            locals.var_xp_dn7 = assign104410_body0_e156611_d_n7;
            locals.var_xp_dn8 = assign104410_body0_e156611_d_n8;
            locals.var_xp_dn9 = assign104410_body0_e156611_d_n9;
            locals.var_xp_dn10 = assign104410_body0_e156611_d_n10;
            locals.var_xp_dn11 = assign104410_body0_e156611_d_n11;
            locals.var_xp_dn14 = assign104410_body0_e156611_d_n14;
            let (assign104410_body1_e156624, assign104410_body1_e156624_d_n0, assign104410_body1_e156624_d_n2, assign104410_body1_e156624_d_n4, assign104410_body1_e156624_d_n5, assign104410_body1_e156624_d_n6, assign104410_body1_e156624_d_n7, assign104410_body1_e156624_d_n8, assign104410_body1_e156624_d_n9, assign104410_body1_e156624_d_n10, assign104410_body1_e156624_d_n11, assign104410_body1_e156624_d_n14,) = {
    if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_guard2374 != 0.0)) {
        let assign104410_body1_e156622: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign104410_body1_e156622, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
            locals.var_xmp = assign104410_body1_e156624;
            locals.var_xmp_dn0 = assign104410_body1_e156624_d_n0;
            locals.var_xmp_dn2 = assign104410_body1_e156624_d_n2;
            locals.var_xmp_dn4 = assign104410_body1_e156624_d_n4;
            locals.var_xmp_dn5 = assign104410_body1_e156624_d_n5;
            locals.var_xmp_dn6 = assign104410_body1_e156624_d_n6;
            locals.var_xmp_dn7 = assign104410_body1_e156624_d_n7;
            locals.var_xmp_dn8 = assign104410_body1_e156624_d_n8;
            locals.var_xmp_dn9 = assign104410_body1_e156624_d_n9;
            locals.var_xmp_dn10 = assign104410_body1_e156624_d_n10;
            locals.var_xmp_dn11 = assign104410_body1_e156624_d_n11;
            locals.var_xmp_dn14 = assign104410_body1_e156624_d_n14;
            let (assign104410_body2_e156637,) = {
    if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_guard2374 != 0.0)) {
        let assign104410_body2_e156635: f64 = (locals.var_m0 + 1.0);
        (assign104410_body2_e156635,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign104410_body2_e156637;
        }

        let (assign104420_e156650, assign104420_e156650_d_n0, assign104420_e156650_d_n2, assign104420_e156650_d_n4, assign104420_e156650_d_n5, assign104420_e156650_d_n6, assign104420_e156650_d_n7, assign104420_e156650_d_n8, assign104420_e156650_d_n9, assign104420_e156650_d_n10, assign104420_e156650_d_n11, assign104420_e156650_d_n14,) = {
    if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_guard2374 != 0.0)) {
        let assign104420_e156648: f64 = (locals.var_xp + locals.var_xmp);
        (assign104420_e156648, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign104420_e156650;
        locals.var_arg_dn0 = assign104420_e156650_d_n0;
        locals.var_arg_dn2 = assign104420_e156650_d_n2;
        locals.var_arg_dn4 = assign104420_e156650_d_n4;
        locals.var_arg_dn5 = assign104420_e156650_d_n5;
        locals.var_arg_dn6 = assign104420_e156650_d_n6;
        locals.var_arg_dn7 = assign104420_e156650_d_n7;
        locals.var_arg_dn8 = assign104420_e156650_d_n8;
        locals.var_arg_dn9 = assign104420_e156650_d_n9;
        locals.var_arg_dn10 = assign104420_e156650_d_n10;
        locals.var_arg_dn11 = assign104420_e156650_d_n11;
        locals.var_arg_dn14 = assign104420_e156650_d_n14;

        let (assign104430_e156661, assign104430_e156661_d_n0, assign104430_e156661_d_n2, assign104430_e156661_d_n4, assign104430_e156661_d_n5, assign104430_e156661_d_n6, assign104430_e156661_d_n7, assign104430_e156661_d_n8, assign104430_e156661_d_n9, assign104430_e156661_d_n10, assign104430_e156661_d_n11, assign104430_e156661_d_n14,) = {
    if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_guard2374 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign104430_e156661;
        locals.var_dnm_dn0 = assign104430_e156661_d_n0;
        locals.var_dnm_dn2 = assign104430_e156661_d_n2;
        locals.var_dnm_dn4 = assign104430_e156661_d_n4;
        locals.var_dnm_dn5 = assign104430_e156661_d_n5;
        locals.var_dnm_dn6 = assign104430_e156661_d_n6;
        locals.var_dnm_dn7 = assign104430_e156661_d_n7;
        locals.var_dnm_dn8 = assign104430_e156661_d_n8;
        locals.var_dnm_dn9 = assign104430_e156661_d_n9;
        locals.var_dnm_dn10 = assign104430_e156661_d_n10;
        locals.var_dnm_dn11 = assign104430_e156661_d_n11;
        locals.var_dnm_dn14 = assign104430_e156661_d_n14;

        let assign104440_e156676: f64 = if ((((p.p442 == 1.0) || (p.p442 == 2.0)) || (p.p442 == 4.0)) || (p.p442 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2375 = assign104440_e156676;

        let assign104450_e156679: f64 = if p.p442 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2376 = assign104450_e156679;

        let (assign104460_e156694,) = {
    if ((((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_guard2374 != 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign104460_e156694;

        let assign104470_e156697: f64 = if p.p442 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2377 = assign104470_e156697;

        let (assign104480_e156715,) = {
    if (((((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_guard2374 != 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 == 0.0)) && (locals.var_guard2377 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign104480_e156715;

        let assign104490_e156718: f64 = if p.p442 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2378 = assign104490_e156718;

        let (assign104500_e156739,) = {
    if ((((((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_guard2374 != 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 == 0.0)) && (locals.var_guard2377 == 0.0)) && (locals.var_guard2378 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign104500_e156739;

        let assign104510_e156742: f64 = if p.p442 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2379 = assign104510_e156742;

        let (assign104520_e156766,) = {
    if (((((((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_guard2374 != 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 == 0.0)) && (locals.var_guard2377 == 0.0)) && (locals.var_guard2378 == 0.0)) && (locals.var_guard2379 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign104520_e156766;

        let (assign104530_e156779,) = {
    if (((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_guard2374 != 0.0)) && (locals.var_guard2375 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign104530_e156779;

        let mut assign104540_loop_guard: usize = 0;
        while {
            let assign104540_cond_e156793: f64 = if ((((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_guard2374 != 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign104540_cond_e156793 != 0.0
        } {
            assign104540_loop_guard += 1;
            assert!(assign104540_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign104540_body0_e156807, assign104540_body0_e156807_d_n0, assign104540_body0_e156807_d_n2, assign104540_body0_e156807_d_n4, assign104540_body0_e156807_d_n5, assign104540_body0_e156807_d_n6, assign104540_body0_e156807_d_n7, assign104540_body0_e156807_d_n8, assign104540_body0_e156807_d_n9, assign104540_body0_e156807_d_n10, assign104540_body0_e156807_d_n11, assign104540_body0_e156807_d_n14,) = {
    if (((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_guard2374 != 0.0)) && (locals.var_guard2375 != 0.0)) {
        let assign104540_body0_e156805: f64 = (locals.var_dnm).sqrt();
        (assign104540_body0_e156805, (locals.var_dnm_dn0 / (2.0 * assign104540_body0_e156805)), (locals.var_dnm_dn2 / (2.0 * assign104540_body0_e156805)), (locals.var_dnm_dn4 / (2.0 * assign104540_body0_e156805)), (locals.var_dnm_dn5 / (2.0 * assign104540_body0_e156805)), (locals.var_dnm_dn6 / (2.0 * assign104540_body0_e156805)), (locals.var_dnm_dn7 / (2.0 * assign104540_body0_e156805)), (locals.var_dnm_dn8 / (2.0 * assign104540_body0_e156805)), (locals.var_dnm_dn9 / (2.0 * assign104540_body0_e156805)), (locals.var_dnm_dn10 / (2.0 * assign104540_body0_e156805)), (locals.var_dnm_dn11 / (2.0 * assign104540_body0_e156805)), (locals.var_dnm_dn14 / (2.0 * assign104540_body0_e156805)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign104540_body0_e156807;
            locals.var_dnm_dn0 = assign104540_body0_e156807_d_n0;
            locals.var_dnm_dn2 = assign104540_body0_e156807_d_n2;
            locals.var_dnm_dn4 = assign104540_body0_e156807_d_n4;
            locals.var_dnm_dn5 = assign104540_body0_e156807_d_n5;
            locals.var_dnm_dn6 = assign104540_body0_e156807_d_n6;
            locals.var_dnm_dn7 = assign104540_body0_e156807_d_n7;
            locals.var_dnm_dn8 = assign104540_body0_e156807_d_n8;
            locals.var_dnm_dn9 = assign104540_body0_e156807_d_n9;
            locals.var_dnm_dn10 = assign104540_body0_e156807_d_n10;
            locals.var_dnm_dn11 = assign104540_body0_e156807_d_n11;
            locals.var_dnm_dn14 = assign104540_body0_e156807_d_n14;
            let (assign104540_body1_e156822,) = {
    if (((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_guard2374 != 0.0)) && (locals.var_guard2375 != 0.0)) {
        let assign104540_body1_e156820: f64 = (locals.var_m0 + 1.0);
        (assign104540_body1_e156820,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign104540_body1_e156822;
        }

        let (assign104550_e156847, assign104550_e156847_d_n0, assign104550_e156847_d_n2, assign104550_e156847_d_n4, assign104550_e156847_d_n5, assign104550_e156847_d_n6, assign104550_e156847_d_n7, assign104550_e156847_d_n8, assign104550_e156847_d_n9, assign104550_e156847_d_n10, assign104550_e156847_d_n11, assign104550_e156847_d_n14,) = {
    if (((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_guard2374 != 0.0)) && (locals.var_guard2375 == 0.0)) {
        let (assign104550_e156845, assign104550_e156845_d_n0, assign104550_e156845_d_n2, assign104550_e156845_d_n4, assign104550_e156845_d_n5, assign104550_e156845_d_n6, assign104550_e156845_d_n7, assign104550_e156845_d_n8, assign104550_e156845_d_n9, assign104550_e156845_d_n10, assign104550_e156845_d_n11, assign104550_e156845_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign104550_e156842: f64 = (2.0 * p.p442);
                let assign104550_e156843: f64 = (1.0 / assign104550_e156842);
                let assign104550_e156844: f64 = (locals.var_dnm).powf(assign104550_e156843);
                (assign104550_e156844, if 0.0 == 0.0 && ((assign104550_e156843) as f64).is_finite() && ((assign104550_e156843) as f64).fract() == 0.0 { if assign104550_e156843 == 0.0 { 0.0 } else { (assign104550_e156843 * ((locals.var_dnm).powf(assign104550_e156843 - 1.0) * locals.var_dnm_dn0)) } } else { (assign104550_e156844 * (assign104550_e156843 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign104550_e156843) as f64).is_finite() && ((assign104550_e156843) as f64).fract() == 0.0 { if assign104550_e156843 == 0.0 { 0.0 } else { (assign104550_e156843 * ((locals.var_dnm).powf(assign104550_e156843 - 1.0) * locals.var_dnm_dn2)) } } else { (assign104550_e156844 * (assign104550_e156843 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign104550_e156843) as f64).is_finite() && ((assign104550_e156843) as f64).fract() == 0.0 { if assign104550_e156843 == 0.0 { 0.0 } else { (assign104550_e156843 * ((locals.var_dnm).powf(assign104550_e156843 - 1.0) * locals.var_dnm_dn4)) } } else { (assign104550_e156844 * (assign104550_e156843 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign104550_e156843) as f64).is_finite() && ((assign104550_e156843) as f64).fract() == 0.0 { if assign104550_e156843 == 0.0 { 0.0 } else { (assign104550_e156843 * ((locals.var_dnm).powf(assign104550_e156843 - 1.0) * locals.var_dnm_dn5)) } } else { (assign104550_e156844 * (assign104550_e156843 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign104550_e156843) as f64).is_finite() && ((assign104550_e156843) as f64).fract() == 0.0 { if assign104550_e156843 == 0.0 { 0.0 } else { (assign104550_e156843 * ((locals.var_dnm).powf(assign104550_e156843 - 1.0) * locals.var_dnm_dn6)) } } else { (assign104550_e156844 * (assign104550_e156843 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign104550_e156843) as f64).is_finite() && ((assign104550_e156843) as f64).fract() == 0.0 { if assign104550_e156843 == 0.0 { 0.0 } else { (assign104550_e156843 * ((locals.var_dnm).powf(assign104550_e156843 - 1.0) * locals.var_dnm_dn7)) } } else { (assign104550_e156844 * (assign104550_e156843 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign104550_e156843) as f64).is_finite() && ((assign104550_e156843) as f64).fract() == 0.0 { if assign104550_e156843 == 0.0 { 0.0 } else { (assign104550_e156843 * ((locals.var_dnm).powf(assign104550_e156843 - 1.0) * locals.var_dnm_dn8)) } } else { (assign104550_e156844 * (assign104550_e156843 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign104550_e156843) as f64).is_finite() && ((assign104550_e156843) as f64).fract() == 0.0 { if assign104550_e156843 == 0.0 { 0.0 } else { (assign104550_e156843 * ((locals.var_dnm).powf(assign104550_e156843 - 1.0) * locals.var_dnm_dn9)) } } else { (assign104550_e156844 * (assign104550_e156843 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign104550_e156843) as f64).is_finite() && ((assign104550_e156843) as f64).fract() == 0.0 { if assign104550_e156843 == 0.0 { 0.0 } else { (assign104550_e156843 * ((locals.var_dnm).powf(assign104550_e156843 - 1.0) * locals.var_dnm_dn10)) } } else { (assign104550_e156844 * (assign104550_e156843 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign104550_e156843) as f64).is_finite() && ((assign104550_e156843) as f64).fract() == 0.0 { if assign104550_e156843 == 0.0 { 0.0 } else { (assign104550_e156843 * ((locals.var_dnm).powf(assign104550_e156843 - 1.0) * locals.var_dnm_dn11)) } } else { (assign104550_e156844 * (assign104550_e156843 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign104550_e156843) as f64).is_finite() && ((assign104550_e156843) as f64).fract() == 0.0 { if assign104550_e156843 == 0.0 { 0.0 } else { (assign104550_e156843 * ((locals.var_dnm).powf(assign104550_e156843 - 1.0) * locals.var_dnm_dn14)) } } else { (assign104550_e156844 * (assign104550_e156843 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign104550_e156845, assign104550_e156845_d_n0, assign104550_e156845_d_n2, assign104550_e156845_d_n4, assign104550_e156845_d_n5, assign104550_e156845_d_n6, assign104550_e156845_d_n7, assign104550_e156845_d_n8, assign104550_e156845_d_n9, assign104550_e156845_d_n10, assign104550_e156845_d_n11, assign104550_e156845_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign104550_e156847;
        locals.var_dnm_dn0 = assign104550_e156847_d_n0;
        locals.var_dnm_dn2 = assign104550_e156847_d_n2;
        locals.var_dnm_dn4 = assign104550_e156847_d_n4;
        locals.var_dnm_dn5 = assign104550_e156847_d_n5;
        locals.var_dnm_dn6 = assign104550_e156847_d_n6;
        locals.var_dnm_dn7 = assign104550_e156847_d_n7;
        locals.var_dnm_dn8 = assign104550_e156847_d_n8;
        locals.var_dnm_dn9 = assign104550_e156847_d_n9;
        locals.var_dnm_dn10 = assign104550_e156847_d_n10;
        locals.var_dnm_dn11 = assign104550_e156847_d_n11;
        locals.var_dnm_dn14 = assign104550_e156847_d_n14;

        let (assign104560_e156860, assign104560_e156860_d_n0, assign104560_e156860_d_n2, assign104560_e156860_d_n4, assign104560_e156860_d_n5, assign104560_e156860_d_n6, assign104560_e156860_d_n7, assign104560_e156860_d_n8, assign104560_e156860_d_n9, assign104560_e156860_d_n10, assign104560_e156860_d_n11, assign104560_e156860_d_n14,) = {
    if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_guard2374 != 0.0)) {
        let assign104560_e156858: f64 = (1.0 / locals.var_dnm);
        (assign104560_e156858, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign104560_e156860;
        locals.var_dnm_dn0 = assign104560_e156860_d_n0;
        locals.var_dnm_dn2 = assign104560_e156860_d_n2;
        locals.var_dnm_dn4 = assign104560_e156860_d_n4;
        locals.var_dnm_dn5 = assign104560_e156860_d_n5;
        locals.var_dnm_dn6 = assign104560_e156860_d_n6;
        locals.var_dnm_dn7 = assign104560_e156860_d_n7;
        locals.var_dnm_dn8 = assign104560_e156860_d_n8;
        locals.var_dnm_dn9 = assign104560_e156860_d_n9;
        locals.var_dnm_dn10 = assign104560_e156860_d_n10;
        locals.var_dnm_dn11 = assign104560_e156860_d_n11;
        locals.var_dnm_dn14 = assign104560_e156860_d_n14;

        let (assign104570_e156877, assign104570_e156877_d_n0, assign104570_e156877_d_n2, assign104570_e156877_d_n4, assign104570_e156877_d_n5, assign104570_e156877_d_n6, assign104570_e156877_d_n7, assign104570_e156877_d_n8, assign104570_e156877_d_n9, assign104570_e156877_d_n10, assign104570_e156877_d_n11, assign104570_e156877_d_n14,) = {
    if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_guard2374 != 0.0)) {
        let assign104570_e156872: f64 = (locals.var_noverd * p.p441);
        let assign104570_e156873: f64 = (locals.var_tmf1 * assign104570_e156872);
        let assign104570_e156875: f64 = (assign104570_e156873 * locals.var_dnm);
        (assign104570_e156875, (((locals.var_tmf1_dn0 * assign104570_e156872) * locals.var_dnm) + (assign104570_e156873 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * assign104570_e156872) * locals.var_dnm) + (assign104570_e156873 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * assign104570_e156872) * locals.var_dnm) + (assign104570_e156873 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * assign104570_e156872) * locals.var_dnm) + (assign104570_e156873 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * assign104570_e156872) * locals.var_dnm) + (assign104570_e156873 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * assign104570_e156872) * locals.var_dnm) + (assign104570_e156873 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * assign104570_e156872) * locals.var_dnm) + (assign104570_e156873 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * assign104570_e156872) * locals.var_dnm) + (assign104570_e156873 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * assign104570_e156872) * locals.var_dnm) + (assign104570_e156873 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * assign104570_e156872) * locals.var_dnm) + (assign104570_e156873 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * assign104570_e156872) * locals.var_dnm) + (assign104570_e156873 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign104570_e156877;
        locals.var_tmf0_dn0 = assign104570_e156877_d_n0;
        locals.var_tmf0_dn2 = assign104570_e156877_d_n2;
        locals.var_tmf0_dn4 = assign104570_e156877_d_n4;
        locals.var_tmf0_dn5 = assign104570_e156877_d_n5;
        locals.var_tmf0_dn6 = assign104570_e156877_d_n6;
        locals.var_tmf0_dn7 = assign104570_e156877_d_n7;
        locals.var_tmf0_dn8 = assign104570_e156877_d_n8;
        locals.var_tmf0_dn9 = assign104570_e156877_d_n9;
        locals.var_tmf0_dn10 = assign104570_e156877_d_n10;
        locals.var_tmf0_dn11 = assign104570_e156877_d_n11;
        locals.var_tmf0_dn14 = assign104570_e156877_d_n14;

        let (assign104580_e156896, assign104580_e156896_d_n0, assign104580_e156896_d_n2, assign104580_e156896_d_n4, assign104580_e156896_d_n5, assign104580_e156896_d_n6, assign104580_e156896_d_n7, assign104580_e156896_d_n8, assign104580_e156896_d_n9, assign104580_e156896_d_n10, assign104580_e156896_d_n11, assign104580_e156896_d_n14,) = {
    if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_guard2374 != 0.0)) {
        let assign104580_e156888: f64 = (locals.var_noverd * p.p441);
        let assign104580_e156890: f64 = (assign104580_e156888 * locals.var_xmp);
        let assign104580_e156892: f64 = (assign104580_e156890 * locals.var_dnm);
        let assign104580_e156894: f64 = (assign104580_e156892 / locals.var_arg);
        (assign104580_e156894, ((((((assign104580_e156888 * locals.var_xmp_dn0) * locals.var_dnm) + (assign104580_e156890 * locals.var_dnm_dn0)) * locals.var_arg) - (assign104580_e156892 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((assign104580_e156888 * locals.var_xmp_dn2) * locals.var_dnm) + (assign104580_e156890 * locals.var_dnm_dn2)) * locals.var_arg) - (assign104580_e156892 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((assign104580_e156888 * locals.var_xmp_dn4) * locals.var_dnm) + (assign104580_e156890 * locals.var_dnm_dn4)) * locals.var_arg) - (assign104580_e156892 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((assign104580_e156888 * locals.var_xmp_dn5) * locals.var_dnm) + (assign104580_e156890 * locals.var_dnm_dn5)) * locals.var_arg) - (assign104580_e156892 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((assign104580_e156888 * locals.var_xmp_dn6) * locals.var_dnm) + (assign104580_e156890 * locals.var_dnm_dn6)) * locals.var_arg) - (assign104580_e156892 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((assign104580_e156888 * locals.var_xmp_dn7) * locals.var_dnm) + (assign104580_e156890 * locals.var_dnm_dn7)) * locals.var_arg) - (assign104580_e156892 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((assign104580_e156888 * locals.var_xmp_dn8) * locals.var_dnm) + (assign104580_e156890 * locals.var_dnm_dn8)) * locals.var_arg) - (assign104580_e156892 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((assign104580_e156888 * locals.var_xmp_dn9) * locals.var_dnm) + (assign104580_e156890 * locals.var_dnm_dn9)) * locals.var_arg) - (assign104580_e156892 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((assign104580_e156888 * locals.var_xmp_dn10) * locals.var_dnm) + (assign104580_e156890 * locals.var_dnm_dn10)) * locals.var_arg) - (assign104580_e156892 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((assign104580_e156888 * locals.var_xmp_dn11) * locals.var_dnm) + (assign104580_e156890 * locals.var_dnm_dn11)) * locals.var_arg) - (assign104580_e156892 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((assign104580_e156888 * locals.var_xmp_dn14) * locals.var_dnm) + (assign104580_e156890 * locals.var_dnm_dn14)) * locals.var_arg) - (assign104580_e156892 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign104580_e156896;
        locals.var_t0_dn0 = assign104580_e156896_d_n0;
        locals.var_t0_dn2 = assign104580_e156896_d_n2;
        locals.var_t0_dn4 = assign104580_e156896_d_n4;
        locals.var_t0_dn5 = assign104580_e156896_d_n5;
        locals.var_t0_dn6 = assign104580_e156896_d_n6;
        locals.var_t0_dn7 = assign104580_e156896_d_n7;
        locals.var_t0_dn8 = assign104580_e156896_d_n8;
        locals.var_t0_dn9 = assign104580_e156896_d_n9;
        locals.var_t0_dn10 = assign104580_e156896_d_n10;
        locals.var_t0_dn11 = assign104580_e156896_d_n11;
        locals.var_t0_dn14 = assign104580_e156896_d_n14;

        let (assign104590_e156915, assign104590_e156915_d_n0, assign104590_e156915_d_n2, assign104590_e156915_d_n4, assign104590_e156915_d_n5, assign104590_e156915_d_n6, assign104590_e156915_d_n7, assign104590_e156915_d_n8, assign104590_e156915_d_n9, assign104590_e156915_d_n10, assign104590_e156915_d_n11, assign104590_e156915_d_n14,) = {
    if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_guard2374 != 0.0)) {
        let assign104590_e156907: f64 = (locals.var_noverd * p.p440);
        let assign104590_e156910: f64 = (locals.var_noverd * p.p441);
        let assign104590_e156911: f64 = (assign104590_e156907 - assign104590_e156910);
        let assign104590_e156913: f64 = (assign104590_e156911 + locals.var_tmf0);
        (assign104590_e156913, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign104590_e156915;
        locals.var_t2_dn0 = assign104590_e156915_d_n0;
        locals.var_t2_dn2 = assign104590_e156915_d_n2;
        locals.var_t2_dn4 = assign104590_e156915_d_n4;
        locals.var_t2_dn5 = assign104590_e156915_d_n5;
        locals.var_t2_dn6 = assign104590_e156915_d_n6;
        locals.var_t2_dn7 = assign104590_e156915_d_n7;
        locals.var_t2_dn8 = assign104590_e156915_d_n8;
        locals.var_t2_dn9 = assign104590_e156915_d_n9;
        locals.var_t2_dn10 = assign104590_e156915_d_n10;
        locals.var_t2_dn11 = assign104590_e156915_d_n11;
        locals.var_t2_dn14 = assign104590_e156915_d_n14;

        let (assign104600_e156926, assign104600_e156926_d_n0, assign104600_e156926_d_n2, assign104600_e156926_d_n4, assign104600_e156926_d_n5, assign104600_e156926_d_n6, assign104600_e156926_d_n7, assign104600_e156926_d_n8, assign104600_e156926_d_n9, assign104600_e156926_d_n10, assign104600_e156926_d_n11, assign104600_e156926_d_n14,) = {
    if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_guard2374 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign104600_e156926;
        locals.var_t0_dn0 = assign104600_e156926_d_n0;
        locals.var_t0_dn2 = assign104600_e156926_d_n2;
        locals.var_t0_dn4 = assign104600_e156926_d_n4;
        locals.var_t0_dn5 = assign104600_e156926_d_n5;
        locals.var_t0_dn6 = assign104600_e156926_d_n6;
        locals.var_t0_dn7 = assign104600_e156926_d_n7;
        locals.var_t0_dn8 = assign104600_e156926_d_n8;
        locals.var_t0_dn9 = assign104600_e156926_d_n9;
        locals.var_t0_dn10 = assign104600_e156926_d_n10;
        locals.var_t0_dn11 = assign104600_e156926_d_n11;
        locals.var_t0_dn14 = assign104600_e156926_d_n14;

        let (assign104610_e156938, assign104610_e156938_d_n0, assign104610_e156938_d_n2, assign104610_e156938_d_n4, assign104610_e156938_d_n5, assign104610_e156938_d_n6, assign104610_e156938_d_n7, assign104610_e156938_d_n8, assign104610_e156938_d_n9, assign104610_e156938_d_n10, assign104610_e156938_d_n11, assign104610_e156938_d_n14,) = {
    if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_guard2374 == 0.0)) {
        (locals.var_carr, locals.var_carr_dn0, locals.var_carr_dn2, locals.var_carr_dn4, locals.var_carr_dn5, locals.var_carr_dn6, locals.var_carr_dn7, locals.var_carr_dn8, locals.var_carr_dn9, locals.var_carr_dn10, locals.var_carr_dn11, locals.var_carr_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign104610_e156938;
        locals.var_t2_dn0 = assign104610_e156938_d_n0;
        locals.var_t2_dn2 = assign104610_e156938_d_n2;
        locals.var_t2_dn4 = assign104610_e156938_d_n4;
        locals.var_t2_dn5 = assign104610_e156938_d_n5;
        locals.var_t2_dn6 = assign104610_e156938_d_n6;
        locals.var_t2_dn7 = assign104610_e156938_d_n7;
        locals.var_t2_dn8 = assign104610_e156938_d_n8;
        locals.var_t2_dn9 = assign104610_e156938_d_n9;
        locals.var_t2_dn10 = assign104610_e156938_d_n10;
        locals.var_t2_dn11 = assign104610_e156938_d_n11;
        locals.var_t2_dn14 = assign104610_e156938_d_n14;

        let (assign104620_e156950, assign104620_e156950_d_n0, assign104620_e156950_d_n2, assign104620_e156950_d_n4, assign104620_e156950_d_n5, assign104620_e156950_d_n6, assign104620_e156950_d_n7, assign104620_e156950_d_n8, assign104620_e156950_d_n9, assign104620_e156950_d_n10, assign104620_e156950_d_n11, assign104620_e156950_d_n14,) = {
    if ((((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_guard2374 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign104620_e156950;
        locals.var_t0_dn0 = assign104620_e156950_d_n0;
        locals.var_t0_dn2 = assign104620_e156950_d_n2;
        locals.var_t0_dn4 = assign104620_e156950_d_n4;
        locals.var_t0_dn5 = assign104620_e156950_d_n5;
        locals.var_t0_dn6 = assign104620_e156950_d_n6;
        locals.var_t0_dn7 = assign104620_e156950_d_n7;
        locals.var_t0_dn8 = assign104620_e156950_d_n8;
        locals.var_t0_dn9 = assign104620_e156950_d_n9;
        locals.var_t0_dn10 = assign104620_e156950_d_n10;
        locals.var_t0_dn11 = assign104620_e156950_d_n11;
        locals.var_t0_dn14 = assign104620_e156950_d_n14;

    }

    pub(super) fn stamp_transient_block_382(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign104630_e156959, assign104630_e156959_d_n0, assign104630_e156959_d_n2, assign104630_e156959_d_n4, assign104630_e156959_d_n5, assign104630_e156959_d_n6, assign104630_e156959_d_n7, assign104630_e156959_d_n8, assign104630_e156959_d_n9, assign104630_e156959_d_n10, assign104630_e156959_d_n11, assign104630_e156959_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2373 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_carr, locals.var_carr_dn0, locals.var_carr_dn2, locals.var_carr_dn4, locals.var_carr_dn5, locals.var_carr_dn6, locals.var_carr_dn7, locals.var_carr_dn8, locals.var_carr_dn9, locals.var_carr_dn10, locals.var_carr_dn11, locals.var_carr_dn14,)
    }
};
        locals.var_carr = assign104630_e156959;
        locals.var_carr_dn0 = assign104630_e156959_d_n0;
        locals.var_carr_dn2 = assign104630_e156959_d_n2;
        locals.var_carr_dn4 = assign104630_e156959_d_n4;
        locals.var_carr_dn5 = assign104630_e156959_d_n5;
        locals.var_carr_dn6 = assign104630_e156959_d_n6;
        locals.var_carr_dn7 = assign104630_e156959_d_n7;
        locals.var_carr_dn8 = assign104630_e156959_d_n8;
        locals.var_carr_dn9 = assign104630_e156959_d_n9;
        locals.var_carr_dn10 = assign104630_e156959_d_n10;
        locals.var_carr_dn11 = assign104630_e156959_d_n11;
        locals.var_carr_dn14 = assign104630_e156959_d_n14;

        let (assign104640_e156967, assign104640_e156967_d_n0, assign104640_e156967_d_n2, assign104640_e156967_d_n4, assign104640_e156967_d_n5, assign104640_e156967_d_n6, assign104640_e156967_d_n7, assign104640_e156967_d_n8, assign104640_e156967_d_n9, assign104640_e156967_d_n10, assign104640_e156967_d_n11, assign104640_e156967_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104640_e156965: f64 = (-locals.var_rd_ps0ld);
        (assign104640_e156965, (-locals.var_rd_ps0ld_dn0), (-locals.var_rd_ps0ld_dn2), (-locals.var_rd_ps0ld_dn4), (-locals.var_rd_ps0ld_dn5), (-locals.var_rd_ps0ld_dn6), (-locals.var_rd_ps0ld_dn7), (-locals.var_rd_ps0ld_dn8), (-locals.var_rd_ps0ld_dn9), (-locals.var_rd_ps0ld_dn10), (-locals.var_rd_ps0ld_dn11), (-locals.var_rd_ps0ld_dn14),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign104640_e156967;
        locals.var_t0_dn0 = assign104640_e156967_d_n0;
        locals.var_t0_dn2 = assign104640_e156967_d_n2;
        locals.var_t0_dn4 = assign104640_e156967_d_n4;
        locals.var_t0_dn5 = assign104640_e156967_d_n5;
        locals.var_t0_dn6 = assign104640_e156967_d_n6;
        locals.var_t0_dn7 = assign104640_e156967_d_n7;
        locals.var_t0_dn8 = assign104640_e156967_d_n8;
        locals.var_t0_dn9 = assign104640_e156967_d_n9;
        locals.var_t0_dn10 = assign104640_e156967_d_n10;
        locals.var_t0_dn11 = assign104640_e156967_d_n11;
        locals.var_t0_dn14 = assign104640_e156967_d_n14;

        let (assign104650_e156983, assign104650_e156983_d_n0, assign104650_e156983_d_n2, assign104650_e156983_d_n4, assign104650_e156983_d_n5, assign104650_e156983_d_n6, assign104650_e156983_d_n7, assign104650_e156983_d_n8, assign104650_e156983_d_n9, assign104650_e156983_d_n10, assign104650_e156983_d_n11, assign104650_e156983_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104650_e156974: f64 = (locals.var_t0 * locals.var_t0);
        let assign104650_e156977: f64 = (4.0 * 0.01);
        let assign104650_e156979: f64 = (assign104650_e156977 * 0.01);
        let assign104650_e156980: f64 = (assign104650_e156974 + assign104650_e156979);
        let assign104650_e156981: f64 = (assign104650_e156980).sqrt();
        (assign104650_e156981, (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / (2.0 * assign104650_e156981)), (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / (2.0 * assign104650_e156981)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign104650_e156981)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign104650_e156981)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign104650_e156981)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign104650_e156981)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign104650_e156981)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign104650_e156981)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign104650_e156981)), (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign104650_e156981)), (((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)) / (2.0 * assign104650_e156981)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign104650_e156983;
        locals.var_tmf2_dn0 = assign104650_e156983_d_n0;
        locals.var_tmf2_dn2 = assign104650_e156983_d_n2;
        locals.var_tmf2_dn4 = assign104650_e156983_d_n4;
        locals.var_tmf2_dn5 = assign104650_e156983_d_n5;
        locals.var_tmf2_dn6 = assign104650_e156983_d_n6;
        locals.var_tmf2_dn7 = assign104650_e156983_d_n7;
        locals.var_tmf2_dn8 = assign104650_e156983_d_n8;
        locals.var_tmf2_dn9 = assign104650_e156983_d_n9;
        locals.var_tmf2_dn10 = assign104650_e156983_d_n10;
        locals.var_tmf2_dn11 = assign104650_e156983_d_n11;
        locals.var_tmf2_dn14 = assign104650_e156983_d_n14;

        let (assign104660_e156996, assign104660_e156996_d_n0, assign104660_e156996_d_n2, assign104660_e156996_d_n4, assign104660_e156996_d_n5, assign104660_e156996_d_n6, assign104660_e156996_d_n7, assign104660_e156996_d_n8, assign104660_e156996_d_n9, assign104660_e156996_d_n10, assign104660_e156996_d_n11, assign104660_e156996_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104660_e156992: f64 = (locals.var_t0 / locals.var_tmf2);
        let assign104660_e156993: f64 = (1.0 + assign104660_e156992);
        let assign104660_e156994: f64 = (0.5 * assign104660_e156993);
        (assign104660_e156994, (0.5 * (((locals.var_t0_dn0 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn2 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn4 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn5 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn6 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn7 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn8 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn9 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn10 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn11 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn14 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign104660_e156996;
        locals.var_t9_dn0 = assign104660_e156996_d_n0;
        locals.var_t9_dn2 = assign104660_e156996_d_n2;
        locals.var_t9_dn4 = assign104660_e156996_d_n4;
        locals.var_t9_dn5 = assign104660_e156996_d_n5;
        locals.var_t9_dn6 = assign104660_e156996_d_n6;
        locals.var_t9_dn7 = assign104660_e156996_d_n7;
        locals.var_t9_dn8 = assign104660_e156996_d_n8;
        locals.var_t9_dn9 = assign104660_e156996_d_n9;
        locals.var_t9_dn10 = assign104660_e156996_d_n10;
        locals.var_t9_dn11 = assign104660_e156996_d_n11;
        locals.var_t9_dn14 = assign104660_e156996_d_n14;

        let (assign104670_e157007, assign104670_e157007_d_n0, assign104670_e157007_d_n2, assign104670_e157007_d_n4, assign104670_e157007_d_n5, assign104670_e157007_d_n6, assign104670_e157007_d_n7, assign104670_e157007_d_n8, assign104670_e157007_d_n9, assign104670_e157007_d_n10, assign104670_e157007_d_n11, assign104670_e157007_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104670_e157004: f64 = (locals.var_t0 + locals.var_tmf2);
        let assign104670_e157005: f64 = (0.5 * assign104670_e157004);
        (assign104670_e157005, (0.5 * (locals.var_t0_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t0_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t0_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t0_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t0_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t0_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t0_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t0_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t0_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t0_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t0_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign104670_e157007;
        locals.var_t0_dn0 = assign104670_e157007_d_n0;
        locals.var_t0_dn2 = assign104670_e157007_d_n2;
        locals.var_t0_dn4 = assign104670_e157007_d_n4;
        locals.var_t0_dn5 = assign104670_e157007_d_n5;
        locals.var_t0_dn6 = assign104670_e157007_d_n6;
        locals.var_t0_dn7 = assign104670_e157007_d_n7;
        locals.var_t0_dn8 = assign104670_e157007_d_n8;
        locals.var_t0_dn9 = assign104670_e157007_d_n9;
        locals.var_t0_dn10 = assign104670_e157007_d_n10;
        locals.var_t0_dn11 = assign104670_e157007_d_n11;
        locals.var_t0_dn14 = assign104670_e157007_d_n14;

        let assign104680_e157010: f64 = if locals.var_t0 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2380 = assign104680_e157010;

        let (assign104690_e157019, assign104690_e157019_d_n0, assign104690_e157019_d_n2, assign104690_e157019_d_n4, assign104690_e157019_d_n5, assign104690_e157019_d_n6, assign104690_e157019_d_n7, assign104690_e157019_d_n8, assign104690_e157019_d_n9, assign104690_e157019_d_n10, assign104690_e157019_d_n11, assign104690_e157019_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2380 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign104690_e157019;
        locals.var_t0_dn0 = assign104690_e157019_d_n0;
        locals.var_t0_dn2 = assign104690_e157019_d_n2;
        locals.var_t0_dn4 = assign104690_e157019_d_n4;
        locals.var_t0_dn5 = assign104690_e157019_d_n5;
        locals.var_t0_dn6 = assign104690_e157019_d_n6;
        locals.var_t0_dn7 = assign104690_e157019_d_n7;
        locals.var_t0_dn8 = assign104690_e157019_d_n8;
        locals.var_t0_dn9 = assign104690_e157019_d_n9;
        locals.var_t0_dn10 = assign104690_e157019_d_n10;
        locals.var_t0_dn11 = assign104690_e157019_d_n11;
        locals.var_t0_dn14 = assign104690_e157019_d_n14;

        let (assign104700_e157028, assign104700_e157028_d_n0, assign104700_e157028_d_n2, assign104700_e157028_d_n4, assign104700_e157028_d_n5, assign104700_e157028_d_n6, assign104700_e157028_d_n7, assign104700_e157028_d_n8, assign104700_e157028_d_n9, assign104700_e157028_d_n10, assign104700_e157028_d_n11, assign104700_e157028_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2380 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign104700_e157028;
        locals.var_t9_dn0 = assign104700_e157028_d_n0;
        locals.var_t9_dn2 = assign104700_e157028_d_n2;
        locals.var_t9_dn4 = assign104700_e157028_d_n4;
        locals.var_t9_dn5 = assign104700_e157028_d_n5;
        locals.var_t9_dn6 = assign104700_e157028_d_n6;
        locals.var_t9_dn7 = assign104700_e157028_d_n7;
        locals.var_t9_dn8 = assign104700_e157028_d_n8;
        locals.var_t9_dn9 = assign104700_e157028_d_n9;
        locals.var_t9_dn10 = assign104700_e157028_d_n10;
        locals.var_t9_dn11 = assign104700_e157028_d_n11;
        locals.var_t9_dn14 = assign104700_e157028_d_n14;

        let (assign104710_e157039, assign104710_e157039_d_n0, assign104710_e157039_d_n2, assign104710_e157039_d_n4, assign104710_e157039_d_n5, assign104710_e157039_d_n6, assign104710_e157039_d_n7, assign104710_e157039_d_n8, assign104710_e157039_d_n9, assign104710_e157039_d_n10, assign104710_e157039_d_n11, assign104710_e157039_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104710_e157036: f64 = (10.0 * 2.220446049250313e-16);
        let assign104710_e157037: f64 = (locals.var_t0 + assign104710_e157036);
        (assign104710_e157037, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign104710_e157039;
        locals.var_t0_dn0 = assign104710_e157039_d_n0;
        locals.var_t0_dn2 = assign104710_e157039_d_n2;
        locals.var_t0_dn4 = assign104710_e157039_d_n4;
        locals.var_t0_dn5 = assign104710_e157039_d_n5;
        locals.var_t0_dn6 = assign104710_e157039_d_n6;
        locals.var_t0_dn7 = assign104710_e157039_d_n7;
        locals.var_t0_dn8 = assign104710_e157039_d_n8;
        locals.var_t0_dn9 = assign104710_e157039_d_n9;
        locals.var_t0_dn10 = assign104710_e157039_d_n10;
        locals.var_t0_dn11 = assign104710_e157039_d_n11;
        locals.var_t0_dn14 = assign104710_e157039_d_n14;

        let (assign104720_e157049, assign104720_e157049_d_n0, assign104720_e157049_d_n2, assign104720_e157049_d_n4, assign104720_e157049_d_n5, assign104720_e157049_d_n6, assign104720_e157049_d_n7, assign104720_e157049_d_n8, assign104720_e157049_d_n9, assign104720_e157049_d_n10, assign104720_e157049_d_n11, assign104720_e157049_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104720_e157046: f64 = (locals.var_kdep * locals.var_t0);
        let assign104720_e157047: f64 = (assign104720_e157046).sqrt();
        (assign104720_e157047, ((locals.var_kdep * locals.var_t0_dn0) / (2.0 * assign104720_e157047)), ((locals.var_kdep * locals.var_t0_dn2) / (2.0 * assign104720_e157047)), ((locals.var_kdep * locals.var_t0_dn4) / (2.0 * assign104720_e157047)), ((locals.var_kdep * locals.var_t0_dn5) / (2.0 * assign104720_e157047)), ((locals.var_kdep * locals.var_t0_dn6) / (2.0 * assign104720_e157047)), ((locals.var_kdep * locals.var_t0_dn7) / (2.0 * assign104720_e157047)), ((locals.var_kdep * locals.var_t0_dn8) / (2.0 * assign104720_e157047)), ((locals.var_kdep * locals.var_t0_dn9) / (2.0 * assign104720_e157047)), ((locals.var_kdep * locals.var_t0_dn10) / (2.0 * assign104720_e157047)), ((locals.var_kdep * locals.var_t0_dn11) / (2.0 * assign104720_e157047)), ((locals.var_kdep * locals.var_t0_dn14) / (2.0 * assign104720_e157047)),)
    } else {
        (locals.var_wdepl, locals.var_wdepl_dn0, locals.var_wdepl_dn2, locals.var_wdepl_dn4, locals.var_wdepl_dn5, locals.var_wdepl_dn6, locals.var_wdepl_dn7, locals.var_wdepl_dn8, locals.var_wdepl_dn9, locals.var_wdepl_dn10, locals.var_wdepl_dn11, locals.var_wdepl_dn14,)
    }
};
        locals.var_wdepl = assign104720_e157049;
        locals.var_wdepl_dn0 = assign104720_e157049_d_n0;
        locals.var_wdepl_dn2 = assign104720_e157049_d_n2;
        locals.var_wdepl_dn4 = assign104720_e157049_d_n4;
        locals.var_wdepl_dn5 = assign104720_e157049_d_n5;
        locals.var_wdepl_dn6 = assign104720_e157049_d_n6;
        locals.var_wdepl_dn7 = assign104720_e157049_d_n7;
        locals.var_wdepl_dn8 = assign104720_e157049_d_n8;
        locals.var_wdepl_dn9 = assign104720_e157049_d_n9;
        locals.var_wdepl_dn10 = assign104720_e157049_d_n10;
        locals.var_wdepl_dn11 = assign104720_e157049_d_n11;
        locals.var_wdepl_dn14 = assign104720_e157049_d_n14;

        let (assign104730_e157060, assign104730_e157060_d_n0, assign104730_e157060_d_n2, assign104730_e157060_d_n4, assign104730_e157060_d_n5, assign104730_e157060_d_n6, assign104730_e157060_d_n7, assign104730_e157060_d_n8, assign104730_e157060_d_n9, assign104730_e157060_d_n10, assign104730_e157060_d_n11, assign104730_e157060_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104730_e157056: f64 = (locals.var_vds__blk2354 - locals.var_vbs__blk2355);
        let assign104730_e157058: f64 = (assign104730_e157056 + p.p137);
        (assign104730_e157058, 0.0, 0.0, 0.0, 0.0, locals.var_vds__blk2354_dn6, 0.0, (locals.var_vds__blk2354_dn8 - locals.var_vbs__blk2355_dn8), (-locals.var_vbs__blk2355_dn9), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign104730_e157060;
        locals.var_t2_dn0 = assign104730_e157060_d_n0;
        locals.var_t2_dn2 = assign104730_e157060_d_n2;
        locals.var_t2_dn4 = assign104730_e157060_d_n4;
        locals.var_t2_dn5 = assign104730_e157060_d_n5;
        locals.var_t2_dn6 = assign104730_e157060_d_n6;
        locals.var_t2_dn7 = assign104730_e157060_d_n7;
        locals.var_t2_dn8 = assign104730_e157060_d_n8;
        locals.var_t2_dn9 = assign104730_e157060_d_n9;
        locals.var_t2_dn10 = assign104730_e157060_d_n10;
        locals.var_t2_dn11 = assign104730_e157060_d_n11;
        locals.var_t2_dn14 = assign104730_e157060_d_n14;

        let (assign104740_e157076, assign104740_e157076_d_n0, assign104740_e157076_d_n2, assign104740_e157076_d_n4, assign104740_e157076_d_n5, assign104740_e157076_d_n6, assign104740_e157076_d_n7, assign104740_e157076_d_n8, assign104740_e157076_d_n9, assign104740_e157076_d_n10, assign104740_e157076_d_n11, assign104740_e157076_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104740_e157067: f64 = (locals.var_t2 * locals.var_t2);
        let assign104740_e157070: f64 = (4.0 * 0.01);
        let assign104740_e157072: f64 = (assign104740_e157070 * 0.01);
        let assign104740_e157073: f64 = (assign104740_e157067 + assign104740_e157072);
        let assign104740_e157074: f64 = (assign104740_e157073).sqrt();
        (assign104740_e157074, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign104740_e157074)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign104740_e157074)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign104740_e157074)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign104740_e157074)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign104740_e157074)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign104740_e157074)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign104740_e157074)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign104740_e157074)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign104740_e157074)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign104740_e157074)), (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign104740_e157074)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign104740_e157076;
        locals.var_tmf2_dn0 = assign104740_e157076_d_n0;
        locals.var_tmf2_dn2 = assign104740_e157076_d_n2;
        locals.var_tmf2_dn4 = assign104740_e157076_d_n4;
        locals.var_tmf2_dn5 = assign104740_e157076_d_n5;
        locals.var_tmf2_dn6 = assign104740_e157076_d_n6;
        locals.var_tmf2_dn7 = assign104740_e157076_d_n7;
        locals.var_tmf2_dn8 = assign104740_e157076_d_n8;
        locals.var_tmf2_dn9 = assign104740_e157076_d_n9;
        locals.var_tmf2_dn10 = assign104740_e157076_d_n10;
        locals.var_tmf2_dn11 = assign104740_e157076_d_n11;
        locals.var_tmf2_dn14 = assign104740_e157076_d_n14;

        let (assign104750_e157089, assign104750_e157089_d_n0, assign104750_e157089_d_n2, assign104750_e157089_d_n4, assign104750_e157089_d_n5, assign104750_e157089_d_n6, assign104750_e157089_d_n7, assign104750_e157089_d_n8, assign104750_e157089_d_n9, assign104750_e157089_d_n10, assign104750_e157089_d_n11, assign104750_e157089_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104750_e157085: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign104750_e157086: f64 = (1.0 + assign104750_e157085);
        let assign104750_e157087: f64 = (0.5 * assign104750_e157086);
        (assign104750_e157087, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn11 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn14 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign104750_e157089;
        locals.var_t9_dn0 = assign104750_e157089_d_n0;
        locals.var_t9_dn2 = assign104750_e157089_d_n2;
        locals.var_t9_dn4 = assign104750_e157089_d_n4;
        locals.var_t9_dn5 = assign104750_e157089_d_n5;
        locals.var_t9_dn6 = assign104750_e157089_d_n6;
        locals.var_t9_dn7 = assign104750_e157089_d_n7;
        locals.var_t9_dn8 = assign104750_e157089_d_n8;
        locals.var_t9_dn9 = assign104750_e157089_d_n9;
        locals.var_t9_dn10 = assign104750_e157089_d_n10;
        locals.var_t9_dn11 = assign104750_e157089_d_n11;
        locals.var_t9_dn14 = assign104750_e157089_d_n14;

        let (assign104760_e157100, assign104760_e157100_d_n0, assign104760_e157100_d_n2, assign104760_e157100_d_n4, assign104760_e157100_d_n5, assign104760_e157100_d_n6, assign104760_e157100_d_n7, assign104760_e157100_d_n8, assign104760_e157100_d_n9, assign104760_e157100_d_n10, assign104760_e157100_d_n11, assign104760_e157100_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104760_e157097: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign104760_e157098: f64 = (0.5 * assign104760_e157097);
        (assign104760_e157098, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t2_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign104760_e157100;
        locals.var_t2_dn0 = assign104760_e157100_d_n0;
        locals.var_t2_dn2 = assign104760_e157100_d_n2;
        locals.var_t2_dn4 = assign104760_e157100_d_n4;
        locals.var_t2_dn5 = assign104760_e157100_d_n5;
        locals.var_t2_dn6 = assign104760_e157100_d_n6;
        locals.var_t2_dn7 = assign104760_e157100_d_n7;
        locals.var_t2_dn8 = assign104760_e157100_d_n8;
        locals.var_t2_dn9 = assign104760_e157100_d_n9;
        locals.var_t2_dn10 = assign104760_e157100_d_n10;
        locals.var_t2_dn11 = assign104760_e157100_d_n11;
        locals.var_t2_dn14 = assign104760_e157100_d_n14;

        let assign104770_e157103: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2381 = assign104770_e157103;

        let (assign104780_e157112, assign104780_e157112_d_n0, assign104780_e157112_d_n2, assign104780_e157112_d_n4, assign104780_e157112_d_n5, assign104780_e157112_d_n6, assign104780_e157112_d_n7, assign104780_e157112_d_n8, assign104780_e157112_d_n9, assign104780_e157112_d_n10, assign104780_e157112_d_n11, assign104780_e157112_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign104780_e157112;
        locals.var_t2_dn0 = assign104780_e157112_d_n0;
        locals.var_t2_dn2 = assign104780_e157112_d_n2;
        locals.var_t2_dn4 = assign104780_e157112_d_n4;
        locals.var_t2_dn5 = assign104780_e157112_d_n5;
        locals.var_t2_dn6 = assign104780_e157112_d_n6;
        locals.var_t2_dn7 = assign104780_e157112_d_n7;
        locals.var_t2_dn8 = assign104780_e157112_d_n8;
        locals.var_t2_dn9 = assign104780_e157112_d_n9;
        locals.var_t2_dn10 = assign104780_e157112_d_n10;
        locals.var_t2_dn11 = assign104780_e157112_d_n11;
        locals.var_t2_dn14 = assign104780_e157112_d_n14;

        let (assign104790_e157121, assign104790_e157121_d_n0, assign104790_e157121_d_n2, assign104790_e157121_d_n4, assign104790_e157121_d_n5, assign104790_e157121_d_n6, assign104790_e157121_d_n7, assign104790_e157121_d_n8, assign104790_e157121_d_n9, assign104790_e157121_d_n10, assign104790_e157121_d_n11, assign104790_e157121_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign104790_e157121;
        locals.var_t9_dn0 = assign104790_e157121_d_n0;
        locals.var_t9_dn2 = assign104790_e157121_d_n2;
        locals.var_t9_dn4 = assign104790_e157121_d_n4;
        locals.var_t9_dn5 = assign104790_e157121_d_n5;
        locals.var_t9_dn6 = assign104790_e157121_d_n6;
        locals.var_t9_dn7 = assign104790_e157121_d_n7;
        locals.var_t9_dn8 = assign104790_e157121_d_n8;
        locals.var_t9_dn9 = assign104790_e157121_d_n9;
        locals.var_t9_dn10 = assign104790_e157121_d_n10;
        locals.var_t9_dn11 = assign104790_e157121_d_n11;
        locals.var_t9_dn14 = assign104790_e157121_d_n14;

        let (assign104800_e157132, assign104800_e157132_d_n0, assign104800_e157132_d_n2, assign104800_e157132_d_n4, assign104800_e157132_d_n5, assign104800_e157132_d_n6, assign104800_e157132_d_n7, assign104800_e157132_d_n8, assign104800_e157132_d_n9, assign104800_e157132_d_n10, assign104800_e157132_d_n11, assign104800_e157132_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104800_e157129: f64 = (10.0 * 2.220446049250313e-16);
        let assign104800_e157130: f64 = (locals.var_t2 + assign104800_e157129);
        (assign104800_e157130, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign104800_e157132;
        locals.var_t2_dn0 = assign104800_e157132_d_n0;
        locals.var_t2_dn2 = assign104800_e157132_d_n2;
        locals.var_t2_dn4 = assign104800_e157132_d_n4;
        locals.var_t2_dn5 = assign104800_e157132_d_n5;
        locals.var_t2_dn6 = assign104800_e157132_d_n6;
        locals.var_t2_dn7 = assign104800_e157132_d_n7;
        locals.var_t2_dn8 = assign104800_e157132_d_n8;
        locals.var_t2_dn9 = assign104800_e157132_d_n9;
        locals.var_t2_dn10 = assign104800_e157132_d_n10;
        locals.var_t2_dn11 = assign104800_e157132_d_n11;
        locals.var_t2_dn14 = assign104800_e157132_d_n14;

        let (assign104810_e157142, assign104810_e157142_d_n0, assign104810_e157142_d_n2, assign104810_e157142_d_n4, assign104810_e157142_d_n5, assign104810_e157142_d_n6, assign104810_e157142_d_n7, assign104810_e157142_d_n8, assign104810_e157142_d_n9, assign104810_e157142_d_n10, assign104810_e157142_d_n11, assign104810_e157142_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104810_e157139: f64 = (locals.var_kjunc * locals.var_t2);
        let assign104810_e157140: f64 = (assign104810_e157139).sqrt();
        (assign104810_e157140, (((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign104810_e157140)), (((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign104810_e157140)), (((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign104810_e157140)), (((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign104810_e157140)), (((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign104810_e157140)), (((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign104810_e157140)), (((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign104810_e157140)), (((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign104810_e157140)), (((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign104810_e157140)), (((locals.var_kjunc_dn11 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn11)) / (2.0 * assign104810_e157140)), (((locals.var_kjunc_dn14 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn14)) / (2.0 * assign104810_e157140)),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn11, locals.var_wjunc0_dn14,)
    }
};
        locals.var_wjunc0 = assign104810_e157142;
        locals.var_wjunc0_dn0 = assign104810_e157142_d_n0;
        locals.var_wjunc0_dn2 = assign104810_e157142_d_n2;
        locals.var_wjunc0_dn4 = assign104810_e157142_d_n4;
        locals.var_wjunc0_dn5 = assign104810_e157142_d_n5;
        locals.var_wjunc0_dn6 = assign104810_e157142_d_n6;
        locals.var_wjunc0_dn7 = assign104810_e157142_d_n7;
        locals.var_wjunc0_dn8 = assign104810_e157142_d_n8;
        locals.var_wjunc0_dn9 = assign104810_e157142_d_n9;
        locals.var_wjunc0_dn10 = assign104810_e157142_d_n10;
        locals.var_wjunc0_dn11 = assign104810_e157142_d_n11;
        locals.var_wjunc0_dn14 = assign104810_e157142_d_n14;

        let (assign104820_e157155, assign104820_e157155_d_n0, assign104820_e157155_d_n2, assign104820_e157155_d_n4, assign104820_e157155_d_n5, assign104820_e157155_d_n6, assign104820_e157155_d_n7, assign104820_e157155_d_n8, assign104820_e157155_d_n9, assign104820_e157155_d_n10, assign104820_e157155_d_n11, assign104820_e157155_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104820_e157149: f64 = (locals.var_rd_xldld - locals.var_wjunc0);
        let assign104820_e157152: f64 = (0.01 * locals.var_rd_xldld);
        let assign104820_e157153: f64 = (assign104820_e157149 - assign104820_e157152);
        (assign104820_e157153, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn11), (-locals.var_wjunc0_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign104820_e157155;
        locals.var_tmf1_dn0 = assign104820_e157155_d_n0;
        locals.var_tmf1_dn2 = assign104820_e157155_d_n2;
        locals.var_tmf1_dn4 = assign104820_e157155_d_n4;
        locals.var_tmf1_dn5 = assign104820_e157155_d_n5;
        locals.var_tmf1_dn6 = assign104820_e157155_d_n6;
        locals.var_tmf1_dn7 = assign104820_e157155_d_n7;
        locals.var_tmf1_dn8 = assign104820_e157155_d_n8;
        locals.var_tmf1_dn9 = assign104820_e157155_d_n9;
        locals.var_tmf1_dn10 = assign104820_e157155_d_n10;
        locals.var_tmf1_dn11 = assign104820_e157155_d_n11;
        locals.var_tmf1_dn14 = assign104820_e157155_d_n14;

        let (assign104830_e157168, assign104830_e157168_d_n0, assign104830_e157168_d_n2, assign104830_e157168_d_n4, assign104830_e157168_d_n5, assign104830_e157168_d_n6, assign104830_e157168_d_n7, assign104830_e157168_d_n8, assign104830_e157168_d_n9, assign104830_e157168_d_n10, assign104830_e157168_d_n11, assign104830_e157168_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104830_e157162: f64 = (4.0 * locals.var_rd_xldld);
        let assign104830_e157165: f64 = (0.01 * locals.var_rd_xldld);
        let assign104830_e157166: f64 = (assign104830_e157162 * assign104830_e157165);
        (assign104830_e157166, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign104830_e157168;
        locals.var_tmf2_dn0 = assign104830_e157168_d_n0;
        locals.var_tmf2_dn2 = assign104830_e157168_d_n2;
        locals.var_tmf2_dn4 = assign104830_e157168_d_n4;
        locals.var_tmf2_dn5 = assign104830_e157168_d_n5;
        locals.var_tmf2_dn6 = assign104830_e157168_d_n6;
        locals.var_tmf2_dn7 = assign104830_e157168_d_n7;
        locals.var_tmf2_dn8 = assign104830_e157168_d_n8;
        locals.var_tmf2_dn9 = assign104830_e157168_d_n9;
        locals.var_tmf2_dn10 = assign104830_e157168_d_n10;
        locals.var_tmf2_dn11 = assign104830_e157168_d_n11;
        locals.var_tmf2_dn14 = assign104830_e157168_d_n14;

        let (assign104840_e157181, assign104840_e157181_d_n0, assign104840_e157181_d_n2, assign104840_e157181_d_n4, assign104840_e157181_d_n5, assign104840_e157181_d_n6, assign104840_e157181_d_n7, assign104840_e157181_d_n8, assign104840_e157181_d_n9, assign104840_e157181_d_n10, assign104840_e157181_d_n11, assign104840_e157181_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let (assign104840_e157179, assign104840_e157179_d_n0, assign104840_e157179_d_n2, assign104840_e157179_d_n4, assign104840_e157179_d_n5, assign104840_e157179_d_n6, assign104840_e157179_d_n7, assign104840_e157179_d_n8, assign104840_e157179_d_n9, assign104840_e157179_d_n10, assign104840_e157179_d_n11, assign104840_e157179_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign104840_e157178: f64 = (-locals.var_tmf2);
                (assign104840_e157178, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign104840_e157179, assign104840_e157179_d_n0, assign104840_e157179_d_n2, assign104840_e157179_d_n4, assign104840_e157179_d_n5, assign104840_e157179_d_n6, assign104840_e157179_d_n7, assign104840_e157179_d_n8, assign104840_e157179_d_n9, assign104840_e157179_d_n10, assign104840_e157179_d_n11, assign104840_e157179_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign104840_e157181;
        locals.var_tmf2_dn0 = assign104840_e157181_d_n0;
        locals.var_tmf2_dn2 = assign104840_e157181_d_n2;
        locals.var_tmf2_dn4 = assign104840_e157181_d_n4;
        locals.var_tmf2_dn5 = assign104840_e157181_d_n5;
        locals.var_tmf2_dn6 = assign104840_e157181_d_n6;
        locals.var_tmf2_dn7 = assign104840_e157181_d_n7;
        locals.var_tmf2_dn8 = assign104840_e157181_d_n8;
        locals.var_tmf2_dn9 = assign104840_e157181_d_n9;
        locals.var_tmf2_dn10 = assign104840_e157181_d_n10;
        locals.var_tmf2_dn11 = assign104840_e157181_d_n11;
        locals.var_tmf2_dn14 = assign104840_e157181_d_n14;

        let (assign104850_e157193, assign104850_e157193_d_n0, assign104850_e157193_d_n2, assign104850_e157193_d_n4, assign104850_e157193_d_n5, assign104850_e157193_d_n6, assign104850_e157193_d_n7, assign104850_e157193_d_n8, assign104850_e157193_d_n9, assign104850_e157193_d_n10, assign104850_e157193_d_n11, assign104850_e157193_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104850_e157188: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign104850_e157190: f64 = (assign104850_e157188 + locals.var_tmf2);
        let assign104850_e157191: f64 = (assign104850_e157190).sqrt();
        (assign104850_e157191, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign104850_e157191)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign104850_e157191)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign104850_e157191)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign104850_e157191)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign104850_e157191)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign104850_e157191)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign104850_e157191)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign104850_e157191)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign104850_e157191)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign104850_e157191)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign104850_e157191)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign104850_e157193;
        locals.var_tmf2_dn0 = assign104850_e157193_d_n0;
        locals.var_tmf2_dn2 = assign104850_e157193_d_n2;
        locals.var_tmf2_dn4 = assign104850_e157193_d_n4;
        locals.var_tmf2_dn5 = assign104850_e157193_d_n5;
        locals.var_tmf2_dn6 = assign104850_e157193_d_n6;
        locals.var_tmf2_dn7 = assign104850_e157193_d_n7;
        locals.var_tmf2_dn8 = assign104850_e157193_d_n8;
        locals.var_tmf2_dn9 = assign104850_e157193_d_n9;
        locals.var_tmf2_dn10 = assign104850_e157193_d_n10;
        locals.var_tmf2_dn11 = assign104850_e157193_d_n11;
        locals.var_tmf2_dn14 = assign104850_e157193_d_n14;

        let (assign104860_e157206, assign104860_e157206_d_n0, assign104860_e157206_d_n2, assign104860_e157206_d_n4, assign104860_e157206_d_n5, assign104860_e157206_d_n6, assign104860_e157206_d_n7, assign104860_e157206_d_n8, assign104860_e157206_d_n9, assign104860_e157206_d_n10, assign104860_e157206_d_n11, assign104860_e157206_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104860_e157202: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign104860_e157203: f64 = (1.0 + assign104860_e157202);
        let assign104860_e157204: f64 = (0.5 * assign104860_e157203);
        (assign104860_e157204, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign104860_e157206;
        locals.var_t0_dn0 = assign104860_e157206_d_n0;
        locals.var_t0_dn2 = assign104860_e157206_d_n2;
        locals.var_t0_dn4 = assign104860_e157206_d_n4;
        locals.var_t0_dn5 = assign104860_e157206_d_n5;
        locals.var_t0_dn6 = assign104860_e157206_d_n6;
        locals.var_t0_dn7 = assign104860_e157206_d_n7;
        locals.var_t0_dn8 = assign104860_e157206_d_n8;
        locals.var_t0_dn9 = assign104860_e157206_d_n9;
        locals.var_t0_dn10 = assign104860_e157206_d_n10;
        locals.var_t0_dn11 = assign104860_e157206_d_n11;
        locals.var_t0_dn14 = assign104860_e157206_d_n14;

    }

    pub(super) fn stamp_transient_block_383(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign104870_e157219, assign104870_e157219_d_n0, assign104870_e157219_d_n2, assign104870_e157219_d_n4, assign104870_e157219_d_n5, assign104870_e157219_d_n6, assign104870_e157219_d_n7, assign104870_e157219_d_n8, assign104870_e157219_d_n9, assign104870_e157219_d_n10, assign104870_e157219_d_n11, assign104870_e157219_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104870_e157215: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign104870_e157216: f64 = (0.5 * assign104870_e157215);
        let assign104870_e157217: f64 = (locals.var_rd_xldld - assign104870_e157216);
        (assign104870_e157217, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_wjunc, locals.var_wjunc_dn0, locals.var_wjunc_dn2, locals.var_wjunc_dn4, locals.var_wjunc_dn5, locals.var_wjunc_dn6, locals.var_wjunc_dn7, locals.var_wjunc_dn8, locals.var_wjunc_dn9, locals.var_wjunc_dn10, locals.var_wjunc_dn11, locals.var_wjunc_dn14,)
    }
};
        locals.var_wjunc = assign104870_e157219;
        locals.var_wjunc_dn0 = assign104870_e157219_d_n0;
        locals.var_wjunc_dn2 = assign104870_e157219_d_n2;
        locals.var_wjunc_dn4 = assign104870_e157219_d_n4;
        locals.var_wjunc_dn5 = assign104870_e157219_d_n5;
        locals.var_wjunc_dn6 = assign104870_e157219_d_n6;
        locals.var_wjunc_dn7 = assign104870_e157219_d_n7;
        locals.var_wjunc_dn8 = assign104870_e157219_d_n8;
        locals.var_wjunc_dn9 = assign104870_e157219_d_n9;
        locals.var_wjunc_dn10 = assign104870_e157219_d_n10;
        locals.var_wjunc_dn11 = assign104870_e157219_d_n11;
        locals.var_wjunc_dn14 = assign104870_e157219_d_n14;

        let (assign104880_e157228,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104880_e157226: f64 = (p.p419 + 1e-25);
        (assign104880_e157226,)
    } else {
        (locals.var_wrdrdjunc,)
    }
};
        locals.var_wrdrdjunc = assign104880_e157228;

        let (assign104890_e157247, assign104890_e157247_d_n0, assign104890_e157247_d_n2, assign104890_e157247_d_n4, assign104890_e157247_d_n5, assign104890_e157247_d_n6, assign104890_e157247_d_n7, assign104890_e157247_d_n8, assign104890_e157247_d_n9, assign104890_e157247_d_n10, assign104890_e157247_d_n11, assign104890_e157247_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104890_e157238: f64 = (locals.var_wdepl / locals.var_wrdrdjunc);
        let assign104890_e157241: f64 = (locals.var_wjunc / locals.var_rd_xldld);
        let assign104890_e157242: f64 = (assign104890_e157238 + assign104890_e157241);
        let assign104890_e157243: f64 = (locals.var_cx * assign104890_e157242);
        let assign104890_e157244: f64 = (1.0 - assign104890_e157243);
        let assign104890_e157245: f64 = (locals.var_xmax * assign104890_e157244);
        (assign104890_e157245, (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn0 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn0 / locals.var_rd_xldld))))), (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn2 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn2 / locals.var_rd_xldld))))), (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn4 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn4 / locals.var_rd_xldld))))), (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn5 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn5 / locals.var_rd_xldld))))), (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn6 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn6 / locals.var_rd_xldld))))), (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn7 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn7 / locals.var_rd_xldld))))), (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn8 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn8 / locals.var_rd_xldld))))), (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn9 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn9 / locals.var_rd_xldld))))), (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn10 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn10 / locals.var_rd_xldld))))), (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn11 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn11 / locals.var_rd_xldld))))), (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn14 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn14 / locals.var_rd_xldld))))),)
    } else {
        (locals.var_xov, locals.var_xov_dn0, locals.var_xov_dn2, locals.var_xov_dn4, locals.var_xov_dn5, locals.var_xov_dn6, locals.var_xov_dn7, locals.var_xov_dn8, locals.var_xov_dn9, locals.var_xov_dn10, locals.var_xov_dn11, locals.var_xov_dn14,)
    }
};
        locals.var_xov = assign104890_e157247;
        locals.var_xov_dn0 = assign104890_e157247_d_n0;
        locals.var_xov_dn2 = assign104890_e157247_d_n2;
        locals.var_xov_dn4 = assign104890_e157247_d_n4;
        locals.var_xov_dn5 = assign104890_e157247_d_n5;
        locals.var_xov_dn6 = assign104890_e157247_d_n6;
        locals.var_xov_dn7 = assign104890_e157247_d_n7;
        locals.var_xov_dn8 = assign104890_e157247_d_n8;
        locals.var_xov_dn9 = assign104890_e157247_d_n9;
        locals.var_xov_dn10 = assign104890_e157247_d_n10;
        locals.var_xov_dn11 = assign104890_e157247_d_n11;
        locals.var_xov_dn14 = assign104890_e157247_d_n14;

        let (assign104900_e157275, assign104900_e157275_d_n0, assign104900_e157275_d_n2, assign104900_e157275_d_n4, assign104900_e157275_d_n5, assign104900_e157275_d_n6, assign104900_e157275_d_n7, assign104900_e157275_d_n8, assign104900_e157275_d_n9, assign104900_e157275_d_n10, assign104900_e157275_d_n11, assign104900_e157275_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104900_e157254: f64 = (locals.var_xov * locals.var_xov);
        let assign104900_e157258: f64 = (1.0 - locals.var_uc_rdrcx);
        let assign104900_e157260: f64 = (assign104900_e157258 * locals.var_xmax);
        let assign104900_e157262: f64 = (assign104900_e157260 / 100.0);
        let assign104900_e157263: f64 = (4.0 * assign104900_e157262);
        let assign104900_e157266: f64 = (1.0 - locals.var_uc_rdrcx);
        let assign104900_e157268: f64 = (assign104900_e157266 * locals.var_xmax);
        let assign104900_e157270: f64 = (assign104900_e157268 / 100.0);
        let assign104900_e157271: f64 = (assign104900_e157263 * assign104900_e157270);
        let assign104900_e157272: f64 = (assign104900_e157254 + assign104900_e157271);
        let assign104900_e157273: f64 = (assign104900_e157272).sqrt();
        (assign104900_e157273, (((locals.var_xov_dn0 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn0)) / (2.0 * assign104900_e157273)), (((locals.var_xov_dn2 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn2)) / (2.0 * assign104900_e157273)), (((locals.var_xov_dn4 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn4)) / (2.0 * assign104900_e157273)), (((locals.var_xov_dn5 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn5)) / (2.0 * assign104900_e157273)), (((locals.var_xov_dn6 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn6)) / (2.0 * assign104900_e157273)), (((locals.var_xov_dn7 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn7)) / (2.0 * assign104900_e157273)), (((locals.var_xov_dn8 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn8)) / (2.0 * assign104900_e157273)), (((locals.var_xov_dn9 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn9)) / (2.0 * assign104900_e157273)), (((locals.var_xov_dn10 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn10)) / (2.0 * assign104900_e157273)), (((locals.var_xov_dn11 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn11)) / (2.0 * assign104900_e157273)), (((locals.var_xov_dn14 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn14)) / (2.0 * assign104900_e157273)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign104900_e157275;
        locals.var_tmf2_dn0 = assign104900_e157275_d_n0;
        locals.var_tmf2_dn2 = assign104900_e157275_d_n2;
        locals.var_tmf2_dn4 = assign104900_e157275_d_n4;
        locals.var_tmf2_dn5 = assign104900_e157275_d_n5;
        locals.var_tmf2_dn6 = assign104900_e157275_d_n6;
        locals.var_tmf2_dn7 = assign104900_e157275_d_n7;
        locals.var_tmf2_dn8 = assign104900_e157275_d_n8;
        locals.var_tmf2_dn9 = assign104900_e157275_d_n9;
        locals.var_tmf2_dn10 = assign104900_e157275_d_n10;
        locals.var_tmf2_dn11 = assign104900_e157275_d_n11;
        locals.var_tmf2_dn14 = assign104900_e157275_d_n14;

        let (assign104910_e157288, assign104910_e157288_d_n0, assign104910_e157288_d_n2, assign104910_e157288_d_n4, assign104910_e157288_d_n5, assign104910_e157288_d_n6, assign104910_e157288_d_n7, assign104910_e157288_d_n8, assign104910_e157288_d_n9, assign104910_e157288_d_n10, assign104910_e157288_d_n11, assign104910_e157288_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104910_e157284: f64 = (locals.var_xov / locals.var_tmf2);
        let assign104910_e157285: f64 = (1.0 + assign104910_e157284);
        let assign104910_e157286: f64 = (0.5 * assign104910_e157285);
        (assign104910_e157286, (0.5 * (((locals.var_xov_dn0 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_xov_dn2 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_xov_dn4 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_xov_dn5 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_xov_dn6 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_xov_dn7 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_xov_dn8 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_xov_dn9 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_xov_dn10 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_xov_dn11 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_xov_dn14 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign104910_e157288;
        locals.var_t9_dn0 = assign104910_e157288_d_n0;
        locals.var_t9_dn2 = assign104910_e157288_d_n2;
        locals.var_t9_dn4 = assign104910_e157288_d_n4;
        locals.var_t9_dn5 = assign104910_e157288_d_n5;
        locals.var_t9_dn6 = assign104910_e157288_d_n6;
        locals.var_t9_dn7 = assign104910_e157288_d_n7;
        locals.var_t9_dn8 = assign104910_e157288_d_n8;
        locals.var_t9_dn9 = assign104910_e157288_d_n9;
        locals.var_t9_dn10 = assign104910_e157288_d_n10;
        locals.var_t9_dn11 = assign104910_e157288_d_n11;
        locals.var_t9_dn14 = assign104910_e157288_d_n14;

        let (assign104920_e157299, assign104920_e157299_d_n0, assign104920_e157299_d_n2, assign104920_e157299_d_n4, assign104920_e157299_d_n5, assign104920_e157299_d_n6, assign104920_e157299_d_n7, assign104920_e157299_d_n8, assign104920_e157299_d_n9, assign104920_e157299_d_n10, assign104920_e157299_d_n11, assign104920_e157299_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104920_e157296: f64 = (locals.var_xov + locals.var_tmf2);
        let assign104920_e157297: f64 = (0.5 * assign104920_e157296);
        (assign104920_e157297, (0.5 * (locals.var_xov_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_xov_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_xov_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_xov_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_xov_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_xov_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_xov_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_xov_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_xov_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_xov_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_xov_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_xov, locals.var_xov_dn0, locals.var_xov_dn2, locals.var_xov_dn4, locals.var_xov_dn5, locals.var_xov_dn6, locals.var_xov_dn7, locals.var_xov_dn8, locals.var_xov_dn9, locals.var_xov_dn10, locals.var_xov_dn11, locals.var_xov_dn14,)
    }
};
        locals.var_xov = assign104920_e157299;
        locals.var_xov_dn0 = assign104920_e157299_d_n0;
        locals.var_xov_dn2 = assign104920_e157299_d_n2;
        locals.var_xov_dn4 = assign104920_e157299_d_n4;
        locals.var_xov_dn5 = assign104920_e157299_d_n5;
        locals.var_xov_dn6 = assign104920_e157299_d_n6;
        locals.var_xov_dn7 = assign104920_e157299_d_n7;
        locals.var_xov_dn8 = assign104920_e157299_d_n8;
        locals.var_xov_dn9 = assign104920_e157299_d_n9;
        locals.var_xov_dn10 = assign104920_e157299_d_n10;
        locals.var_xov_dn11 = assign104920_e157299_d_n11;
        locals.var_xov_dn14 = assign104920_e157299_d_n14;

        let assign104930_e157302: f64 = if locals.var_xov < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2382 = assign104930_e157302;

        let (assign104940_e157311, assign104940_e157311_d_n0, assign104940_e157311_d_n2, assign104940_e157311_d_n4, assign104940_e157311_d_n5, assign104940_e157311_d_n6, assign104940_e157311_d_n7, assign104940_e157311_d_n8, assign104940_e157311_d_n9, assign104940_e157311_d_n10, assign104940_e157311_d_n11, assign104940_e157311_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2382 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xov, locals.var_xov_dn0, locals.var_xov_dn2, locals.var_xov_dn4, locals.var_xov_dn5, locals.var_xov_dn6, locals.var_xov_dn7, locals.var_xov_dn8, locals.var_xov_dn9, locals.var_xov_dn10, locals.var_xov_dn11, locals.var_xov_dn14,)
    }
};
        locals.var_xov = assign104940_e157311;
        locals.var_xov_dn0 = assign104940_e157311_d_n0;
        locals.var_xov_dn2 = assign104940_e157311_d_n2;
        locals.var_xov_dn4 = assign104940_e157311_d_n4;
        locals.var_xov_dn5 = assign104940_e157311_d_n5;
        locals.var_xov_dn6 = assign104940_e157311_d_n6;
        locals.var_xov_dn7 = assign104940_e157311_d_n7;
        locals.var_xov_dn8 = assign104940_e157311_d_n8;
        locals.var_xov_dn9 = assign104940_e157311_d_n9;
        locals.var_xov_dn10 = assign104940_e157311_d_n10;
        locals.var_xov_dn11 = assign104940_e157311_d_n11;
        locals.var_xov_dn14 = assign104940_e157311_d_n14;

        let (assign104950_e157320, assign104950_e157320_d_n0, assign104950_e157320_d_n2, assign104950_e157320_d_n4, assign104950_e157320_d_n5, assign104950_e157320_d_n6, assign104950_e157320_d_n7, assign104950_e157320_d_n8, assign104950_e157320_d_n9, assign104950_e157320_d_n10, assign104950_e157320_d_n11, assign104950_e157320_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2382 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign104950_e157320;
        locals.var_t9_dn0 = assign104950_e157320_d_n0;
        locals.var_t9_dn2 = assign104950_e157320_d_n2;
        locals.var_t9_dn4 = assign104950_e157320_d_n4;
        locals.var_t9_dn5 = assign104950_e157320_d_n5;
        locals.var_t9_dn6 = assign104950_e157320_d_n6;
        locals.var_t9_dn7 = assign104950_e157320_d_n7;
        locals.var_t9_dn8 = assign104950_e157320_d_n8;
        locals.var_t9_dn9 = assign104950_e157320_d_n9;
        locals.var_t9_dn10 = assign104950_e157320_d_n10;
        locals.var_t9_dn11 = assign104950_e157320_d_n11;
        locals.var_t9_dn14 = assign104950_e157320_d_n14;

        let (assign104960_e157331, assign104960_e157331_d_n0, assign104960_e157331_d_n2, assign104960_e157331_d_n4, assign104960_e157331_d_n5, assign104960_e157331_d_n6, assign104960_e157331_d_n7, assign104960_e157331_d_n8, assign104960_e157331_d_n9, assign104960_e157331_d_n10, assign104960_e157331_d_n11, assign104960_e157331_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104960_e157328: f64 = (locals.var_ldrifte + p.p422);
        let assign104960_e157329: f64 = (1.6021918e-19 / assign104960_e157328);
        (assign104960_e157329, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign104960_e157331;
        locals.var_t1_dn0 = assign104960_e157331_d_n0;
        locals.var_t1_dn2 = assign104960_e157331_d_n2;
        locals.var_t1_dn4 = assign104960_e157331_d_n4;
        locals.var_t1_dn5 = assign104960_e157331_d_n5;
        locals.var_t1_dn6 = assign104960_e157331_d_n6;
        locals.var_t1_dn7 = assign104960_e157331_d_n7;
        locals.var_t1_dn8 = assign104960_e157331_d_n8;
        locals.var_t1_dn9 = assign104960_e157331_d_n9;
        locals.var_t1_dn10 = assign104960_e157331_d_n10;
        locals.var_t1_dn11 = assign104960_e157331_d_n11;
        locals.var_t1_dn14 = assign104960_e157331_d_n14;

        let (assign104970_e157344, assign104970_e157344_d_n0, assign104970_e157344_d_n2, assign104970_e157344_d_n4, assign104970_e157344_d_n5, assign104970_e157344_d_n6, assign104970_e157344_d_n7, assign104970_e157344_d_n8, assign104970_e157344_d_n9, assign104970_e157344_d_n10, assign104970_e157344_d_n11, assign104970_e157344_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) {
        let assign104970_e157338: f64 = (locals.var_t1 * locals.var_xov);
        let assign104970_e157340: f64 = (assign104970_e157338 * locals.var_mu__blk2356);
        let assign104970_e157342: f64 = (assign104970_e157340 * locals.var_carr);
        (assign104970_e157342, ((((((locals.var_t1_dn0 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn0)) * locals.var_mu__blk2356) + (assign104970_e157338 * locals.var_mu__blk2356_dn0)) * locals.var_carr) + (assign104970_e157340 * locals.var_carr_dn0)), ((((((locals.var_t1_dn2 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn2)) * locals.var_mu__blk2356) + (assign104970_e157338 * locals.var_mu__blk2356_dn2)) * locals.var_carr) + (assign104970_e157340 * locals.var_carr_dn2)), ((((((locals.var_t1_dn4 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn4)) * locals.var_mu__blk2356) + (assign104970_e157338 * locals.var_mu__blk2356_dn4)) * locals.var_carr) + (assign104970_e157340 * locals.var_carr_dn4)), ((((((locals.var_t1_dn5 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn5)) * locals.var_mu__blk2356) + (assign104970_e157338 * locals.var_mu__blk2356_dn5)) * locals.var_carr) + (assign104970_e157340 * locals.var_carr_dn5)), ((((((locals.var_t1_dn6 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn6)) * locals.var_mu__blk2356) + (assign104970_e157338 * locals.var_mu__blk2356_dn6)) * locals.var_carr) + (assign104970_e157340 * locals.var_carr_dn6)), ((((((locals.var_t1_dn7 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn7)) * locals.var_mu__blk2356) + (assign104970_e157338 * locals.var_mu__blk2356_dn7)) * locals.var_carr) + (assign104970_e157340 * locals.var_carr_dn7)), ((((((locals.var_t1_dn8 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn8)) * locals.var_mu__blk2356) + (assign104970_e157338 * locals.var_mu__blk2356_dn8)) * locals.var_carr) + (assign104970_e157340 * locals.var_carr_dn8)), ((((((locals.var_t1_dn9 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn9)) * locals.var_mu__blk2356) + (assign104970_e157338 * locals.var_mu__blk2356_dn9)) * locals.var_carr) + (assign104970_e157340 * locals.var_carr_dn9)), ((((((locals.var_t1_dn10 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn10)) * locals.var_mu__blk2356) + (assign104970_e157338 * locals.var_mu__blk2356_dn10)) * locals.var_carr) + (assign104970_e157340 * locals.var_carr_dn10)), ((((((locals.var_t1_dn11 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn11)) * locals.var_mu__blk2356) + (assign104970_e157338 * locals.var_mu__blk2356_dn11)) * locals.var_carr) + (assign104970_e157340 * locals.var_carr_dn11)), ((((((locals.var_t1_dn14 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn14)) * locals.var_mu__blk2356) + (assign104970_e157338 * locals.var_mu__blk2356_dn14)) * locals.var_carr) + (assign104970_e157340 * locals.var_carr_dn14)),)
    } else {
        (locals.var_gd, locals.var_gd_dn0, locals.var_gd_dn2, locals.var_gd_dn4, locals.var_gd_dn5, locals.var_gd_dn6, locals.var_gd_dn7, locals.var_gd_dn8, locals.var_gd_dn9, locals.var_gd_dn10, locals.var_gd_dn11, locals.var_gd_dn14,)
    }
};
        locals.var_gd = assign104970_e157344;
        locals.var_gd_dn0 = assign104970_e157344_d_n0;
        locals.var_gd_dn2 = assign104970_e157344_d_n2;
        locals.var_gd_dn4 = assign104970_e157344_d_n4;
        locals.var_gd_dn5 = assign104970_e157344_d_n5;
        locals.var_gd_dn6 = assign104970_e157344_d_n6;
        locals.var_gd_dn7 = assign104970_e157344_d_n7;
        locals.var_gd_dn8 = assign104970_e157344_d_n8;
        locals.var_gd_dn9 = assign104970_e157344_d_n9;
        locals.var_gd_dn10 = assign104970_e157344_d_n10;
        locals.var_gd_dn11 = assign104970_e157344_d_n11;
        locals.var_gd_dn14 = assign104970_e157344_d_n14;

        let assign104980_e157348: f64 = 1e-25;
        let assign104980_e157353: f64 = if ((locals.var_gd < assign104980_e157348) && (1e-25 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2383 = assign104980_e157353;

        let (assign104990_e157366, assign104990_e157366_d_n0, assign104990_e157366_d_n2, assign104990_e157366_d_n4, assign104990_e157366_d_n5, assign104990_e157366_d_n6, assign104990_e157366_d_n7, assign104990_e157366_d_n8, assign104990_e157366_d_n9, assign104990_e157366_d_n10, assign104990_e157366_d_n11, assign104990_e157366_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2383 != 0.0)) {
        let assign104990_e157362: f64 = 1e-25;
        let assign104990_e157364: f64 = (assign104990_e157362 - locals.var_gd);
        (assign104990_e157364, (-locals.var_gd_dn0), (-locals.var_gd_dn2), (-locals.var_gd_dn4), (-locals.var_gd_dn5), (-locals.var_gd_dn6), (-locals.var_gd_dn7), (-locals.var_gd_dn8), (-locals.var_gd_dn9), (-locals.var_gd_dn10), (-locals.var_gd_dn11), (-locals.var_gd_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign104990_e157366;
        locals.var_tmf1_dn0 = assign104990_e157366_d_n0;
        locals.var_tmf1_dn2 = assign104990_e157366_d_n2;
        locals.var_tmf1_dn4 = assign104990_e157366_d_n4;
        locals.var_tmf1_dn5 = assign104990_e157366_d_n5;
        locals.var_tmf1_dn6 = assign104990_e157366_d_n6;
        locals.var_tmf1_dn7 = assign104990_e157366_d_n7;
        locals.var_tmf1_dn8 = assign104990_e157366_d_n8;
        locals.var_tmf1_dn9 = assign104990_e157366_d_n9;
        locals.var_tmf1_dn10 = assign104990_e157366_d_n10;
        locals.var_tmf1_dn11 = assign104990_e157366_d_n11;
        locals.var_tmf1_dn14 = assign104990_e157366_d_n14;

        let (assign105000_e157377, assign105000_e157377_d_n0, assign105000_e157377_d_n2, assign105000_e157377_d_n4, assign105000_e157377_d_n5, assign105000_e157377_d_n6, assign105000_e157377_d_n7, assign105000_e157377_d_n8, assign105000_e157377_d_n9, assign105000_e157377_d_n10, assign105000_e157377_d_n11, assign105000_e157377_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2383 != 0.0)) {
        let assign105000_e157375: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign105000_e157375, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign105000_e157377;
        locals.var_x2_dn0 = assign105000_e157377_d_n0;
        locals.var_x2_dn2 = assign105000_e157377_d_n2;
        locals.var_x2_dn4 = assign105000_e157377_d_n4;
        locals.var_x2_dn5 = assign105000_e157377_d_n5;
        locals.var_x2_dn6 = assign105000_e157377_d_n6;
        locals.var_x2_dn7 = assign105000_e157377_d_n7;
        locals.var_x2_dn8 = assign105000_e157377_d_n8;
        locals.var_x2_dn9 = assign105000_e157377_d_n9;
        locals.var_x2_dn10 = assign105000_e157377_d_n10;
        locals.var_x2_dn11 = assign105000_e157377_d_n11;
        locals.var_x2_dn14 = assign105000_e157377_d_n14;

        let (assign105010_e157388, assign105010_e157388_d_n0, assign105010_e157388_d_n2, assign105010_e157388_d_n4, assign105010_e157388_d_n5, assign105010_e157388_d_n6, assign105010_e157388_d_n7, assign105010_e157388_d_n8, assign105010_e157388_d_n9, assign105010_e157388_d_n10, assign105010_e157388_d_n11, assign105010_e157388_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2383 != 0.0)) {
        let assign105010_e157386: f64 = (1e-25 * 1e-25);
        (assign105010_e157386, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign105010_e157388;
        locals.var_xmax2_dn0 = assign105010_e157388_d_n0;
        locals.var_xmax2_dn2 = assign105010_e157388_d_n2;
        locals.var_xmax2_dn4 = assign105010_e157388_d_n4;
        locals.var_xmax2_dn5 = assign105010_e157388_d_n5;
        locals.var_xmax2_dn6 = assign105010_e157388_d_n6;
        locals.var_xmax2_dn7 = assign105010_e157388_d_n7;
        locals.var_xmax2_dn8 = assign105010_e157388_d_n8;
        locals.var_xmax2_dn9 = assign105010_e157388_d_n9;
        locals.var_xmax2_dn10 = assign105010_e157388_d_n10;
        locals.var_xmax2_dn11 = assign105010_e157388_d_n11;
        locals.var_xmax2_dn14 = assign105010_e157388_d_n14;

        let (assign105020_e157397, assign105020_e157397_d_n0, assign105020_e157397_d_n2, assign105020_e157397_d_n4, assign105020_e157397_d_n5, assign105020_e157397_d_n6, assign105020_e157397_d_n7, assign105020_e157397_d_n8, assign105020_e157397_d_n9, assign105020_e157397_d_n10, assign105020_e157397_d_n11, assign105020_e157397_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2383 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign105020_e157397;
        locals.var_xp_dn0 = assign105020_e157397_d_n0;
        locals.var_xp_dn2 = assign105020_e157397_d_n2;
        locals.var_xp_dn4 = assign105020_e157397_d_n4;
        locals.var_xp_dn5 = assign105020_e157397_d_n5;
        locals.var_xp_dn6 = assign105020_e157397_d_n6;
        locals.var_xp_dn7 = assign105020_e157397_d_n7;
        locals.var_xp_dn8 = assign105020_e157397_d_n8;
        locals.var_xp_dn9 = assign105020_e157397_d_n9;
        locals.var_xp_dn10 = assign105020_e157397_d_n10;
        locals.var_xp_dn11 = assign105020_e157397_d_n11;
        locals.var_xp_dn14 = assign105020_e157397_d_n14;

        let (assign105030_e157406, assign105030_e157406_d_n0, assign105030_e157406_d_n2, assign105030_e157406_d_n4, assign105030_e157406_d_n5, assign105030_e157406_d_n6, assign105030_e157406_d_n7, assign105030_e157406_d_n8, assign105030_e157406_d_n9, assign105030_e157406_d_n10, assign105030_e157406_d_n11, assign105030_e157406_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2383 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign105030_e157406;
        locals.var_xmp_dn0 = assign105030_e157406_d_n0;
        locals.var_xmp_dn2 = assign105030_e157406_d_n2;
        locals.var_xmp_dn4 = assign105030_e157406_d_n4;
        locals.var_xmp_dn5 = assign105030_e157406_d_n5;
        locals.var_xmp_dn6 = assign105030_e157406_d_n6;
        locals.var_xmp_dn7 = assign105030_e157406_d_n7;
        locals.var_xmp_dn8 = assign105030_e157406_d_n8;
        locals.var_xmp_dn9 = assign105030_e157406_d_n9;
        locals.var_xmp_dn10 = assign105030_e157406_d_n10;
        locals.var_xmp_dn11 = assign105030_e157406_d_n11;
        locals.var_xmp_dn14 = assign105030_e157406_d_n14;

        let (assign105040_e157415,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2383 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign105040_e157415;

        let (assign105050_e157424,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2383 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign105050_e157424;

        let (assign105060_e157433, assign105060_e157433_d_n0, assign105060_e157433_d_n2, assign105060_e157433_d_n4, assign105060_e157433_d_n5, assign105060_e157433_d_n6, assign105060_e157433_d_n7, assign105060_e157433_d_n8, assign105060_e157433_d_n9, assign105060_e157433_d_n10, assign105060_e157433_d_n11, assign105060_e157433_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2383 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign105060_e157433;
        locals.var_arg_dn0 = assign105060_e157433_d_n0;
        locals.var_arg_dn2 = assign105060_e157433_d_n2;
        locals.var_arg_dn4 = assign105060_e157433_d_n4;
        locals.var_arg_dn5 = assign105060_e157433_d_n5;
        locals.var_arg_dn6 = assign105060_e157433_d_n6;
        locals.var_arg_dn7 = assign105060_e157433_d_n7;
        locals.var_arg_dn8 = assign105060_e157433_d_n8;
        locals.var_arg_dn9 = assign105060_e157433_d_n9;
        locals.var_arg_dn10 = assign105060_e157433_d_n10;
        locals.var_arg_dn11 = assign105060_e157433_d_n11;
        locals.var_arg_dn14 = assign105060_e157433_d_n14;

        let (assign105070_e157442, assign105070_e157442_d_n0, assign105070_e157442_d_n2, assign105070_e157442_d_n4, assign105070_e157442_d_n5, assign105070_e157442_d_n6, assign105070_e157442_d_n7, assign105070_e157442_d_n8, assign105070_e157442_d_n9, assign105070_e157442_d_n10, assign105070_e157442_d_n11, assign105070_e157442_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2383 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign105070_e157442;
        locals.var_dnm_dn0 = assign105070_e157442_d_n0;
        locals.var_dnm_dn2 = assign105070_e157442_d_n2;
        locals.var_dnm_dn4 = assign105070_e157442_d_n4;
        locals.var_dnm_dn5 = assign105070_e157442_d_n5;
        locals.var_dnm_dn6 = assign105070_e157442_d_n6;
        locals.var_dnm_dn7 = assign105070_e157442_d_n7;
        locals.var_dnm_dn8 = assign105070_e157442_d_n8;
        locals.var_dnm_dn9 = assign105070_e157442_d_n9;
        locals.var_dnm_dn10 = assign105070_e157442_d_n10;
        locals.var_dnm_dn11 = assign105070_e157442_d_n11;
        locals.var_dnm_dn14 = assign105070_e157442_d_n14;

        let (assign105080_e157453, assign105080_e157453_d_n0, assign105080_e157453_d_n2, assign105080_e157453_d_n4, assign105080_e157453_d_n5, assign105080_e157453_d_n6, assign105080_e157453_d_n7, assign105080_e157453_d_n8, assign105080_e157453_d_n9, assign105080_e157453_d_n10, assign105080_e157453_d_n11, assign105080_e157453_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2383 != 0.0)) {
        let assign105080_e157451: f64 = (locals.var_xp * locals.var_x2);
        (assign105080_e157451, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign105080_e157453;
        locals.var_xp_dn0 = assign105080_e157453_d_n0;
        locals.var_xp_dn2 = assign105080_e157453_d_n2;
        locals.var_xp_dn4 = assign105080_e157453_d_n4;
        locals.var_xp_dn5 = assign105080_e157453_d_n5;
        locals.var_xp_dn6 = assign105080_e157453_d_n6;
        locals.var_xp_dn7 = assign105080_e157453_d_n7;
        locals.var_xp_dn8 = assign105080_e157453_d_n8;
        locals.var_xp_dn9 = assign105080_e157453_d_n9;
        locals.var_xp_dn10 = assign105080_e157453_d_n10;
        locals.var_xp_dn11 = assign105080_e157453_d_n11;
        locals.var_xp_dn14 = assign105080_e157453_d_n14;

        let (assign105090_e157464, assign105090_e157464_d_n0, assign105090_e157464_d_n2, assign105090_e157464_d_n4, assign105090_e157464_d_n5, assign105090_e157464_d_n6, assign105090_e157464_d_n7, assign105090_e157464_d_n8, assign105090_e157464_d_n9, assign105090_e157464_d_n10, assign105090_e157464_d_n11, assign105090_e157464_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2383 != 0.0)) {
        let assign105090_e157462: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign105090_e157462, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign105090_e157464;
        locals.var_xmp_dn0 = assign105090_e157464_d_n0;
        locals.var_xmp_dn2 = assign105090_e157464_d_n2;
        locals.var_xmp_dn4 = assign105090_e157464_d_n4;
        locals.var_xmp_dn5 = assign105090_e157464_d_n5;
        locals.var_xmp_dn6 = assign105090_e157464_d_n6;
        locals.var_xmp_dn7 = assign105090_e157464_d_n7;
        locals.var_xmp_dn8 = assign105090_e157464_d_n8;
        locals.var_xmp_dn9 = assign105090_e157464_d_n9;
        locals.var_xmp_dn10 = assign105090_e157464_d_n10;
        locals.var_xmp_dn11 = assign105090_e157464_d_n11;
        locals.var_xmp_dn14 = assign105090_e157464_d_n14;

        let (assign105100_e157475, assign105100_e157475_d_n0, assign105100_e157475_d_n2, assign105100_e157475_d_n4, assign105100_e157475_d_n5, assign105100_e157475_d_n6, assign105100_e157475_d_n7, assign105100_e157475_d_n8, assign105100_e157475_d_n9, assign105100_e157475_d_n10, assign105100_e157475_d_n11, assign105100_e157475_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2383 != 0.0)) {
        let assign105100_e157473: f64 = (locals.var_xp * locals.var_x2);
        (assign105100_e157473, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign105100_e157475;
        locals.var_xp_dn0 = assign105100_e157475_d_n0;
        locals.var_xp_dn2 = assign105100_e157475_d_n2;
        locals.var_xp_dn4 = assign105100_e157475_d_n4;
        locals.var_xp_dn5 = assign105100_e157475_d_n5;
        locals.var_xp_dn6 = assign105100_e157475_d_n6;
        locals.var_xp_dn7 = assign105100_e157475_d_n7;
        locals.var_xp_dn8 = assign105100_e157475_d_n8;
        locals.var_xp_dn9 = assign105100_e157475_d_n9;
        locals.var_xp_dn10 = assign105100_e157475_d_n10;
        locals.var_xp_dn11 = assign105100_e157475_d_n11;
        locals.var_xp_dn14 = assign105100_e157475_d_n14;

        let (assign105110_e157486, assign105110_e157486_d_n0, assign105110_e157486_d_n2, assign105110_e157486_d_n4, assign105110_e157486_d_n5, assign105110_e157486_d_n6, assign105110_e157486_d_n7, assign105110_e157486_d_n8, assign105110_e157486_d_n9, assign105110_e157486_d_n10, assign105110_e157486_d_n11, assign105110_e157486_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2383 != 0.0)) {
        let assign105110_e157484: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign105110_e157484, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign105110_e157486;
        locals.var_xmp_dn0 = assign105110_e157486_d_n0;
        locals.var_xmp_dn2 = assign105110_e157486_d_n2;
        locals.var_xmp_dn4 = assign105110_e157486_d_n4;
        locals.var_xmp_dn5 = assign105110_e157486_d_n5;
        locals.var_xmp_dn6 = assign105110_e157486_d_n6;
        locals.var_xmp_dn7 = assign105110_e157486_d_n7;
        locals.var_xmp_dn8 = assign105110_e157486_d_n8;
        locals.var_xmp_dn9 = assign105110_e157486_d_n9;
        locals.var_xmp_dn10 = assign105110_e157486_d_n10;
        locals.var_xmp_dn11 = assign105110_e157486_d_n11;
        locals.var_xmp_dn14 = assign105110_e157486_d_n14;

        let (assign105120_e157497, assign105120_e157497_d_n0, assign105120_e157497_d_n2, assign105120_e157497_d_n4, assign105120_e157497_d_n5, assign105120_e157497_d_n6, assign105120_e157497_d_n7, assign105120_e157497_d_n8, assign105120_e157497_d_n9, assign105120_e157497_d_n10, assign105120_e157497_d_n11, assign105120_e157497_d_n14,) = {
    if (((locals.var_guard2338 != 0.0) && (locals.var_guard2358 == 0.0)) && (locals.var_guard2383 != 0.0)) {
        let assign105120_e157495: f64 = (locals.var_xp + locals.var_xmp);
        (assign105120_e157495, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign105120_e157497;
        locals.var_arg_dn0 = assign105120_e157497_d_n0;
        locals.var_arg_dn2 = assign105120_e157497_d_n2;
        locals.var_arg_dn4 = assign105120_e157497_d_n4;
        locals.var_arg_dn5 = assign105120_e157497_d_n5;
        locals.var_arg_dn6 = assign105120_e157497_d_n6;
        locals.var_arg_dn7 = assign105120_e157497_d_n7;
        locals.var_arg_dn8 = assign105120_e157497_d_n8;
        locals.var_arg_dn9 = assign105120_e157497_d_n9;
        locals.var_arg_dn10 = assign105120_e157497_d_n10;
        locals.var_arg_dn11 = assign105120_e157497_d_n11;
        locals.var_arg_dn14 = assign105120_e157497_d_n14;

    }
}
