#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_32(
        locals: &mut StampLocals,
    ) {
        let (assign13410_e12797, assign13410_e12797_d_n4, assign13410_e12797_d_n6, assign13410_e12797_d_n7, assign13410_e12797_d_n8, assign13410_e12797_d_n9,) = {
    if (locals.var_guard571 != 0.0) {
        let assign13410_e12791: f64 = (locals.var_q_d2_expnum * locals.var_q_temp1);
        let assign13410_e12794: f64 = (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum);
        let assign13410_e12795: f64 = (assign13410_e12791 - assign13410_e12794);
        (assign13410_e12795, (((locals.var_q_d2_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn4)) - ((locals.var_q_d1_lnexpnum_dn4 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn4))), (((locals.var_q_d2_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn6)) - ((locals.var_q_d1_lnexpnum_dn6 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn6))), (((locals.var_q_d2_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn7)) - ((locals.var_q_d1_lnexpnum_dn7 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn7))), (((locals.var_q_d2_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn8)) - ((locals.var_q_d1_lnexpnum_dn8 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn8))), (((locals.var_q_d2_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn9)) - ((locals.var_q_d1_lnexpnum_dn9 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn9))),)
    } else {
        (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9,)
    }
};
        locals.var_q_d2_lnexpnum = assign13410_e12797;
        locals.var_q_d2_lnexpnum_dn4 = assign13410_e12797_d_n4;
        locals.var_q_d2_lnexpnum_dn6 = assign13410_e12797_d_n6;
        locals.var_q_d2_lnexpnum_dn7 = assign13410_e12797_d_n7;
        locals.var_q_d2_lnexpnum_dn8 = assign13410_e12797_d_n8;
        locals.var_q_d2_lnexpnum_dn9 = assign13410_e12797_d_n9;

        let (assign13420_e12808, assign13420_e12808_d_n4, assign13420_e12808_d_n6, assign13420_e12808_d_n7, assign13420_e12808_d_n8, assign13420_e12808_d_n9,) = {
    if (locals.var_guard571 == 0.0) {
        let assign13420_e12802: f64 = (locals.var_q_k1q1 + 0.6931471805599);
        let assign13420_e12804: f64 = (-locals.var_q_k1q1);
        let assign13420_e12805: f64 = (assign13420_e12804).ln();
        let assign13420_e12806: f64 = (assign13420_e12802 + assign13420_e12805);
        (assign13420_e12806, (locals.var_q_k1q1_dn4 + ((-locals.var_q_k1q1_dn4) / assign13420_e12804)), (locals.var_q_k1q1_dn6 + ((-locals.var_q_k1q1_dn6) / assign13420_e12804)), (locals.var_q_k1q1_dn7 + ((-locals.var_q_k1q1_dn7) / assign13420_e12804)), (locals.var_q_k1q1_dn8 + ((-locals.var_q_k1q1_dn8) / assign13420_e12804)), (locals.var_q_k1q1_dn9 + ((-locals.var_q_k1q1_dn9) / assign13420_e12804)),)
    } else {
        (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9,)
    }
};
        locals.var_q_lnexpnum = assign13420_e12808;
        locals.var_q_lnexpnum_dn4 = assign13420_e12808_d_n4;
        locals.var_q_lnexpnum_dn6 = assign13420_e12808_d_n6;
        locals.var_q_lnexpnum_dn7 = assign13420_e12808_d_n7;
        locals.var_q_lnexpnum_dn8 = assign13420_e12808_d_n8;
        locals.var_q_lnexpnum_dn9 = assign13420_e12808_d_n9;

        let (assign13430_e12815, assign13430_e12815_d_n4, assign13430_e12815_d_n6, assign13430_e12815_d_n7, assign13430_e12815_d_n8, assign13430_e12815_d_n9,) = {
    if (locals.var_guard571 == 0.0) {
        let assign13430_e12813: f64 = (1.0 / locals.var_q1s);
        (assign13430_e12813, (-(locals.var_q1s_dn4 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn6 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn7 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn8 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn9 / (locals.var_q1s * locals.var_q1s))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign13430_e12815;
        locals.var_q_temp1_dn4 = assign13430_e12815_d_n4;
        locals.var_q_temp1_dn6 = assign13430_e12815_d_n6;
        locals.var_q_temp1_dn7 = assign13430_e12815_d_n7;
        locals.var_q_temp1_dn8 = assign13430_e12815_d_n8;
        locals.var_q_temp1_dn9 = assign13430_e12815_d_n9;

        let (assign13440_e12822, assign13440_e12822_d_n4, assign13440_e12822_d_n6, assign13440_e12822_d_n7, assign13440_e12822_d_n8, assign13440_e12822_d_n9,) = {
    if (locals.var_guard571 == 0.0) {
        let assign13440_e12820: f64 = (locals.var_k1 + locals.var_q_temp1);
        (assign13440_e12820, (locals.var_k1_dn4 + locals.var_q_temp1_dn4), (locals.var_k1_dn6 + locals.var_q_temp1_dn6), (locals.var_k1_dn7 + locals.var_q_temp1_dn7), (locals.var_k1_dn8 + locals.var_q_temp1_dn8), (locals.var_k1_dn9 + locals.var_q_temp1_dn9),)
    } else {
        (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9,)
    }
};
        locals.var_q_d1_lnexpnum = assign13440_e12822;
        locals.var_q_d1_lnexpnum_dn4 = assign13440_e12822_d_n4;
        locals.var_q_d1_lnexpnum_dn6 = assign13440_e12822_d_n6;
        locals.var_q_d1_lnexpnum_dn7 = assign13440_e12822_d_n7;
        locals.var_q_d1_lnexpnum_dn8 = assign13440_e12822_d_n8;
        locals.var_q_d1_lnexpnum_dn9 = assign13440_e12822_d_n9;

        let (assign13450_e12830, assign13450_e12830_d_n4, assign13450_e12830_d_n6, assign13450_e12830_d_n7, assign13450_e12830_d_n8, assign13450_e12830_d_n9,) = {
    if (locals.var_guard571 == 0.0) {
        let assign13450_e12826: f64 = (-locals.var_q_temp1);
        let assign13450_e12828: f64 = (assign13450_e12826 * locals.var_q_temp1);
        (assign13450_e12828, (((-locals.var_q_temp1_dn4) * locals.var_q_temp1) + (assign13450_e12826 * locals.var_q_temp1_dn4)), (((-locals.var_q_temp1_dn6) * locals.var_q_temp1) + (assign13450_e12826 * locals.var_q_temp1_dn6)), (((-locals.var_q_temp1_dn7) * locals.var_q_temp1) + (assign13450_e12826 * locals.var_q_temp1_dn7)), (((-locals.var_q_temp1_dn8) * locals.var_q_temp1) + (assign13450_e12826 * locals.var_q_temp1_dn8)), (((-locals.var_q_temp1_dn9) * locals.var_q_temp1) + (assign13450_e12826 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9,)
    }
};
        locals.var_q_d2_lnexpnum = assign13450_e12830;
        locals.var_q_d2_lnexpnum_dn4 = assign13450_e12830_d_n4;
        locals.var_q_d2_lnexpnum_dn6 = assign13450_e12830_d_n6;
        locals.var_q_d2_lnexpnum_dn7 = assign13450_e12830_d_n7;
        locals.var_q_d2_lnexpnum_dn8 = assign13450_e12830_d_n8;
        locals.var_q_d2_lnexpnum_dn9 = assign13450_e12830_d_n9;

        let assign13460_e12833: f64 = (locals.var_xg2x - locals.var_xg1x);
        let assign13460_e12835: f64 = (assign13460_e12833 + locals.var_q1s);
        let assign13460_e12838: f64 = (2.0 * locals.var_q_lnexpnum);
        let assign13460_e12839: f64 = (assign13460_e12835 + assign13460_e12838);
        let assign13460_e12841: f64 = (assign13460_e12839 - locals.var_q_ln_term);
        locals.var_q_q2_int = assign13460_e12841;
        locals.var_q_q2_int_dn4 = ((((locals.var_xg2x_dn4 - locals.var_xg1x_dn4) + locals.var_q1s_dn4) + (2.0 * locals.var_q_lnexpnum_dn4)) - locals.var_q_ln_term_dn4);
        locals.var_q_q2_int_dn6 = ((((locals.var_xg2x_dn6 - locals.var_xg1x_dn6) + locals.var_q1s_dn6) + (2.0 * locals.var_q_lnexpnum_dn6)) - locals.var_q_ln_term_dn6);
        locals.var_q_q2_int_dn7 = ((((locals.var_xg2x_dn7 - locals.var_xg1x_dn7) + locals.var_q1s_dn7) + (2.0 * locals.var_q_lnexpnum_dn7)) - locals.var_q_ln_term_dn7);
        locals.var_q_q2_int_dn8 = ((((locals.var_xg2x_dn8 - locals.var_xg1x_dn8) + locals.var_q1s_dn8) + (2.0 * locals.var_q_lnexpnum_dn8)) - locals.var_q_ln_term_dn8);
        locals.var_q_q2_int_dn9 = ((((locals.var_xg2x_dn9 - locals.var_xg1x_dn9) + locals.var_q1s_dn9) + (2.0 * locals.var_q_lnexpnum_dn9)) - locals.var_q_ln_term_dn9);

        let assign13470_e12845: f64 = (2.0 * locals.var_q_d1_lnexpnum);
        let assign13470_e12846: f64 = (1.0 + assign13470_e12845);
        let assign13470_e12848: f64 = (assign13470_e12846 - locals.var_q_d1_ln);
        locals.var_q_d1_q2 = assign13470_e12848;
        locals.var_q_d1_q2_dn4 = ((2.0 * locals.var_q_d1_lnexpnum_dn4) - locals.var_q_d1_ln_dn4);
        locals.var_q_d1_q2_dn6 = ((2.0 * locals.var_q_d1_lnexpnum_dn6) - locals.var_q_d1_ln_dn6);
        locals.var_q_d1_q2_dn7 = ((2.0 * locals.var_q_d1_lnexpnum_dn7) - locals.var_q_d1_ln_dn7);
        locals.var_q_d1_q2_dn8 = ((2.0 * locals.var_q_d1_lnexpnum_dn8) - locals.var_q_d1_ln_dn8);
        locals.var_q_d1_q2_dn9 = ((2.0 * locals.var_q_d1_lnexpnum_dn9) - locals.var_q_d1_ln_dn9);

        let assign13480_e12851: f64 = (2.0 * locals.var_q_d2_lnexpnum);
        let assign13480_e12853: f64 = (assign13480_e12851 - locals.var_q_d2_ln);
        locals.var_q_d2_q2 = assign13480_e12853;
        locals.var_q_d2_q2_dn4 = ((2.0 * locals.var_q_d2_lnexpnum_dn4) - locals.var_q_d2_ln_dn4);
        locals.var_q_d2_q2_dn6 = ((2.0 * locals.var_q_d2_lnexpnum_dn6) - locals.var_q_d2_ln_dn6);
        locals.var_q_d2_q2_dn7 = ((2.0 * locals.var_q_d2_lnexpnum_dn7) - locals.var_q_d2_ln_dn7);
        locals.var_q_d2_q2_dn8 = ((2.0 * locals.var_q_d2_lnexpnum_dn8) - locals.var_q_d2_ln_dn8);
        locals.var_q_d2_q2_dn9 = ((2.0 * locals.var_q_d2_lnexpnum_dn9) - locals.var_q_d2_ln_dn9);

        let assign13490_e12857: f64 = (locals.var_k2 * locals.var_q_q2_int);
        let assign13490_e12858: f64 = (locals.var_q_k1q1 + assign13490_e12857);
        locals.var_q_qi_int = assign13490_e12858;
        locals.var_q_qi_int_dn4 = (locals.var_q_k1q1_dn4 + ((locals.var_k2_dn4 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn4)));
        locals.var_q_qi_int_dn6 = (locals.var_q_k1q1_dn6 + ((locals.var_k2_dn6 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn6)));
        locals.var_q_qi_int_dn7 = (locals.var_q_k1q1_dn7 + ((locals.var_k2_dn7 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn7)));
        locals.var_q_qi_int_dn8 = (locals.var_q_k1q1_dn8 + ((locals.var_k2_dn8 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn8)));
        locals.var_q_qi_int_dn9 = (locals.var_q_k1q1_dn9 + ((locals.var_k2_dn9 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn9)));

        let assign13500_e12862: f64 = (locals.var_k2 * locals.var_q_d1_q2);
        let assign13500_e12863: f64 = (locals.var_k1 + assign13500_e12862);
        locals.var_q_d1_qi = assign13500_e12863;
        locals.var_q_d1_qi_dn4 = (locals.var_k1_dn4 + ((locals.var_k2_dn4 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn4)));
        locals.var_q_d1_qi_dn6 = (locals.var_k1_dn6 + ((locals.var_k2_dn6 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn6)));
        locals.var_q_d1_qi_dn7 = (locals.var_k1_dn7 + ((locals.var_k2_dn7 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn7)));
        locals.var_q_d1_qi_dn8 = (locals.var_k1_dn8 + ((locals.var_k2_dn8 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn8)));
        locals.var_q_d1_qi_dn9 = (locals.var_k1_dn9 + ((locals.var_k2_dn9 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn9)));

        let assign13510_e12866: f64 = (locals.var_k2 * locals.var_q_d2_q2);
        locals.var_q_d2_qi = assign13510_e12866;
        locals.var_q_d2_qi_dn4 = ((locals.var_k2_dn4 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn4));
        locals.var_q_d2_qi_dn6 = ((locals.var_k2_dn6 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn6));
        locals.var_q_d2_qi_dn7 = ((locals.var_k2_dn7 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn7));
        locals.var_q_d2_qi_dn8 = ((locals.var_k2_dn8 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn8));
        locals.var_q_d2_qi_dn9 = ((locals.var_k2_dn9 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn9));

        let assign13520_e12869: f64 = (locals.var_q_qi_int * locals.var_q_expnum);
        let assign13520_e12871: f64 = (assign13520_e12869 - locals.var_q_aexp);
        locals.var_q_zero = assign13520_e12871;
        locals.var_q_zero_dn4 = (((locals.var_q_qi_int_dn4 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4);
        locals.var_q_zero_dn6 = (((locals.var_q_qi_int_dn6 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6);
        locals.var_q_zero_dn7 = (((locals.var_q_qi_int_dn7 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7);
        locals.var_q_zero_dn8 = (((locals.var_q_qi_int_dn8 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8);
        locals.var_q_zero_dn9 = (((locals.var_q_qi_int_dn9 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9);

        let assign13530_e12874: f64 = (locals.var_q_d1_qi * locals.var_q_expnum);
        let assign13530_e12877: f64 = (locals.var_q_qi_int * locals.var_q_d1_expnum);
        let assign13530_e12878: f64 = (assign13530_e12874 + assign13530_e12877);
        let assign13530_e12880: f64 = (assign13530_e12878 + locals.var_q_aexp);
        locals.var_q_d1_zero = assign13530_e12880;
        locals.var_q_d1_zero_dn4 = ((((locals.var_q_d1_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn4)) + ((locals.var_q_qi_int_dn4 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4);
        locals.var_q_d1_zero_dn6 = ((((locals.var_q_d1_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn6)) + ((locals.var_q_qi_int_dn6 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6);
        locals.var_q_d1_zero_dn7 = ((((locals.var_q_d1_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn7)) + ((locals.var_q_qi_int_dn7 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7);
        locals.var_q_d1_zero_dn8 = ((((locals.var_q_d1_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn8)) + ((locals.var_q_qi_int_dn8 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8);
        locals.var_q_d1_zero_dn9 = ((((locals.var_q_d1_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn9)) + ((locals.var_q_qi_int_dn9 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9);

        let assign13540_e12883: f64 = (locals.var_q_d2_qi * locals.var_q_expnum);
        let assign13540_e12886: f64 = (2.0 * locals.var_q_d1_qi);
        let assign13540_e12888: f64 = (assign13540_e12886 * locals.var_q_d1_expnum);
        let assign13540_e12889: f64 = (assign13540_e12883 + assign13540_e12888);
        let assign13540_e12892: f64 = (locals.var_q_qi_int * locals.var_q_d2_expnum);
        let assign13540_e12893: f64 = (assign13540_e12889 + assign13540_e12892);
        let assign13540_e12895: f64 = (assign13540_e12893 - locals.var_q_aexp);
        locals.var_q_d2_zero = assign13540_e12895;
        locals.var_q_d2_zero_dn4 = (((((locals.var_q_d2_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_d1_qi_dn4) * locals.var_q_d1_expnum) + (assign13540_e12886 * locals.var_q_d1_expnum_dn4))) + ((locals.var_q_qi_int_dn4 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn4))) - locals.var_q_aexp_dn4);
        locals.var_q_d2_zero_dn6 = (((((locals.var_q_d2_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_d1_qi_dn6) * locals.var_q_d1_expnum) + (assign13540_e12886 * locals.var_q_d1_expnum_dn6))) + ((locals.var_q_qi_int_dn6 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn6))) - locals.var_q_aexp_dn6);
        locals.var_q_d2_zero_dn7 = (((((locals.var_q_d2_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_d1_qi_dn7) * locals.var_q_d1_expnum) + (assign13540_e12886 * locals.var_q_d1_expnum_dn7))) + ((locals.var_q_qi_int_dn7 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn7))) - locals.var_q_aexp_dn7);
        locals.var_q_d2_zero_dn8 = (((((locals.var_q_d2_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_d1_qi_dn8) * locals.var_q_d1_expnum) + (assign13540_e12886 * locals.var_q_d1_expnum_dn8))) + ((locals.var_q_qi_int_dn8 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn8))) - locals.var_q_aexp_dn8);
        locals.var_q_d2_zero_dn9 = (((((locals.var_q_d2_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_d1_qi_dn9) * locals.var_q_d1_expnum) + (assign13540_e12886 * locals.var_q_d1_expnum_dn9))) + ((locals.var_q_qi_int_dn9 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn9))) - locals.var_q_aexp_dn9);

        let assign13550_e12898: f64 = (locals.var_q_d1_zero * locals.var_q_d1_zero);
        let assign13550_e12901: f64 = (0.5 * locals.var_q_zero);
        let assign13550_e12903: f64 = (assign13550_e12901 * locals.var_q_d2_zero);
        let assign13550_e12904: f64 = (assign13550_e12898 - assign13550_e12903);
        locals.var_q_temp = assign13550_e12904;
        locals.var_q_temp_dn4 = (((locals.var_q_d1_zero_dn4 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn4)) - (((0.5 * locals.var_q_zero_dn4) * locals.var_q_d2_zero) + (assign13550_e12901 * locals.var_q_d2_zero_dn4)));
        locals.var_q_temp_dn6 = (((locals.var_q_d1_zero_dn6 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn6)) - (((0.5 * locals.var_q_zero_dn6) * locals.var_q_d2_zero) + (assign13550_e12901 * locals.var_q_d2_zero_dn6)));
        locals.var_q_temp_dn7 = (((locals.var_q_d1_zero_dn7 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn7)) - (((0.5 * locals.var_q_zero_dn7) * locals.var_q_d2_zero) + (assign13550_e12901 * locals.var_q_d2_zero_dn7)));
        locals.var_q_temp_dn8 = (((locals.var_q_d1_zero_dn8 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn8)) - (((0.5 * locals.var_q_zero_dn8) * locals.var_q_d2_zero) + (assign13550_e12901 * locals.var_q_d2_zero_dn8)));
        locals.var_q_temp_dn9 = (((locals.var_q_d1_zero_dn9 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn9)) - (((0.5 * locals.var_q_zero_dn9) * locals.var_q_d2_zero) + (assign13550_e12901 * locals.var_q_d2_zero_dn9)));

        let assign13560_e12906: f64 = (-locals.var_q_zero);
        let assign13560_e12908: f64 = (assign13560_e12906 * locals.var_q_d1_zero);
        let assign13560_e12910: f64 = (assign13560_e12908 * locals.var_q_temp);
        let assign13560_e12913: f64 = (locals.var_q_temp * locals.var_q_temp);
        let assign13560_e12915: f64 = (assign13560_e12913 + 1e-200);
        let assign13560_e12916: f64 = (assign13560_e12910 / assign13560_e12915);
        locals.var_q_eps2 = assign13560_e12916;
        locals.var_q_eps2_dn4 = ((((((((-locals.var_q_zero_dn4) * locals.var_q_d1_zero) + (assign13560_e12906 * locals.var_q_d1_zero_dn4)) * locals.var_q_temp) + (assign13560_e12908 * locals.var_q_temp_dn4)) * assign13560_e12915) - (assign13560_e12910 * ((locals.var_q_temp_dn4 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn4)))) / (assign13560_e12915 * assign13560_e12915));
        locals.var_q_eps2_dn6 = ((((((((-locals.var_q_zero_dn6) * locals.var_q_d1_zero) + (assign13560_e12906 * locals.var_q_d1_zero_dn6)) * locals.var_q_temp) + (assign13560_e12908 * locals.var_q_temp_dn6)) * assign13560_e12915) - (assign13560_e12910 * ((locals.var_q_temp_dn6 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn6)))) / (assign13560_e12915 * assign13560_e12915));
        locals.var_q_eps2_dn7 = ((((((((-locals.var_q_zero_dn7) * locals.var_q_d1_zero) + (assign13560_e12906 * locals.var_q_d1_zero_dn7)) * locals.var_q_temp) + (assign13560_e12908 * locals.var_q_temp_dn7)) * assign13560_e12915) - (assign13560_e12910 * ((locals.var_q_temp_dn7 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn7)))) / (assign13560_e12915 * assign13560_e12915));
        locals.var_q_eps2_dn8 = ((((((((-locals.var_q_zero_dn8) * locals.var_q_d1_zero) + (assign13560_e12906 * locals.var_q_d1_zero_dn8)) * locals.var_q_temp) + (assign13560_e12908 * locals.var_q_temp_dn8)) * assign13560_e12915) - (assign13560_e12910 * ((locals.var_q_temp_dn8 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn8)))) / (assign13560_e12915 * assign13560_e12915));
        locals.var_q_eps2_dn9 = ((((((((-locals.var_q_zero_dn9) * locals.var_q_d1_zero) + (assign13560_e12906 * locals.var_q_d1_zero_dn9)) * locals.var_q_temp) + (assign13560_e12908 * locals.var_q_temp_dn9)) * assign13560_e12915) - (assign13560_e12910 * ((locals.var_q_temp_dn9 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn9)))) / (assign13560_e12915 * assign13560_e12915));

        let assign13570_e12919: f64 = (locals.var_q1s + locals.var_q_eps2);
        locals.var_q1s = assign13570_e12919;
        locals.var_q1s_dn4 = (locals.var_q1s_dn4 + locals.var_q_eps2_dn4);
        locals.var_q1s_dn6 = (locals.var_q1s_dn6 + locals.var_q_eps2_dn6);
        locals.var_q1s_dn7 = (locals.var_q1s_dn7 + locals.var_q_eps2_dn7);
        locals.var_q1s_dn8 = (locals.var_q1s_dn8 + locals.var_q_eps2_dn8);
        locals.var_q1s_dn9 = (locals.var_q1s_dn9 + locals.var_q_eps2_dn9);

        let assign13580_e12922: f64 = (locals.var_k1 * locals.var_q1s);
        locals.var_q_k1q1 = assign13580_e12922;
        locals.var_q_k1q1_dn4 = ((locals.var_k1_dn4 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn4));
        locals.var_q_k1q1_dn6 = ((locals.var_k1_dn6 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn6));
        locals.var_q_k1q1_dn7 = ((locals.var_k1_dn7 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn7));
        locals.var_q_k1q1_dn8 = ((locals.var_k1_dn8 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn8));
        locals.var_q_k1q1_dn9 = ((locals.var_k1_dn9 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn9));

        let assign13590_e12925: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign13590_e12927: f64 = assign13590_e12925;
        let assign13590_e12929: f64 = if assign13590_e12927 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard572 = assign13590_e12929;

        let (assign13600_e12938, assign13600_e12938_d_n4, assign13600_e12938_d_n6, assign13600_e12938_d_n7, assign13600_e12938_d_n8, assign13600_e12938_d_n9,) = {
    if (locals.var_guard572 != 0.0) {
        let assign13600_e12933: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign13600_e12935: f64 = assign13600_e12933;
        let assign13600_e12936: f64 = (assign13600_e12935).exp();
        (assign13600_e12936, (assign13600_e12936 * (locals.var_xg1x_dn4 - locals.var_q1s_dn4)), (assign13600_e12936 * (locals.var_xg1x_dn6 - locals.var_q1s_dn6)), (assign13600_e12936 * (locals.var_xg1x_dn7 - locals.var_q1s_dn7)), (assign13600_e12936 * (locals.var_xg1x_dn8 - locals.var_q1s_dn8)), (assign13600_e12936 * (locals.var_xg1x_dn9 - locals.var_q1s_dn9)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign13600_e12938;
        locals.var_q_temp1_dn4 = assign13600_e12938_d_n4;
        locals.var_q_temp1_dn6 = assign13600_e12938_d_n6;
        locals.var_q_temp1_dn7 = assign13600_e12938_d_n7;
        locals.var_q_temp1_dn8 = assign13600_e12938_d_n8;
        locals.var_q_temp1_dn9 = assign13600_e12938_d_n9;

        let (assign13610_e12977, assign13610_e12977_d_n4, assign13610_e12977_d_n6, assign13610_e12977_d_n7, assign13610_e12977_d_n8, assign13610_e12977_d_n9,) = {
    if (locals.var_guard572 == 0.0) {
        let assign13610_e12945: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign13610_e12947: f64 = assign13610_e12945;
        let assign13610_e12949: f64 = (assign13610_e12947 - 80.0);
        let assign13610_e12954: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign13610_e12956: f64 = assign13610_e12954;
        let assign13610_e12958: f64 = (assign13610_e12956 - 80.0);
        let assign13610_e12959: f64 = (0.5 * assign13610_e12958);
        let assign13610_e12963: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign13610_e12965: f64 = assign13610_e12963;
        let assign13610_e12967: f64 = (assign13610_e12965 - 80.0);
        let assign13610_e12969: f64 = (assign13610_e12967 * 0.3333333333333);
        let assign13610_e12970: f64 = (1.0 + assign13610_e12969);
        let assign13610_e12971: f64 = (assign13610_e12959 * assign13610_e12970);
        let assign13610_e12972: f64 = (1.0 + assign13610_e12971);
        let assign13610_e12973: f64 = (assign13610_e12949 * assign13610_e12972);
        let assign13610_e12974: f64 = (1.0 + assign13610_e12973);
        let assign13610_e12975: f64 = (5.54062e34 * assign13610_e12974);
        (assign13610_e12975, (5.54062e34 * (((locals.var_xg1x_dn4 - locals.var_q1s_dn4) * assign13610_e12972) + (assign13610_e12949 * (((0.5 * (locals.var_xg1x_dn4 - locals.var_q1s_dn4)) * assign13610_e12970) + (assign13610_e12959 * ((locals.var_xg1x_dn4 - locals.var_q1s_dn4) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn6 - locals.var_q1s_dn6) * assign13610_e12972) + (assign13610_e12949 * (((0.5 * (locals.var_xg1x_dn6 - locals.var_q1s_dn6)) * assign13610_e12970) + (assign13610_e12959 * ((locals.var_xg1x_dn6 - locals.var_q1s_dn6) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn7 - locals.var_q1s_dn7) * assign13610_e12972) + (assign13610_e12949 * (((0.5 * (locals.var_xg1x_dn7 - locals.var_q1s_dn7)) * assign13610_e12970) + (assign13610_e12959 * ((locals.var_xg1x_dn7 - locals.var_q1s_dn7) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn8 - locals.var_q1s_dn8) * assign13610_e12972) + (assign13610_e12949 * (((0.5 * (locals.var_xg1x_dn8 - locals.var_q1s_dn8)) * assign13610_e12970) + (assign13610_e12959 * ((locals.var_xg1x_dn8 - locals.var_q1s_dn8) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn9 - locals.var_q1s_dn9) * assign13610_e12972) + (assign13610_e12949 * (((0.5 * (locals.var_xg1x_dn9 - locals.var_q1s_dn9)) * assign13610_e12970) + (assign13610_e12959 * ((locals.var_xg1x_dn9 - locals.var_q1s_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign13610_e12977;
        locals.var_q_temp1_dn4 = assign13610_e12977_d_n4;
        locals.var_q_temp1_dn6 = assign13610_e12977_d_n6;
        locals.var_q_temp1_dn7 = assign13610_e12977_d_n7;
        locals.var_q_temp1_dn8 = assign13610_e12977_d_n8;
        locals.var_q_temp1_dn9 = assign13610_e12977_d_n9;

        let assign13620_e12980: f64 = (locals.var_a0 * locals.var_q_temp1);
        locals.var_q_aexp = assign13620_e12980;
        locals.var_q_aexp_dn4 = ((locals.var_a0_dn4 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn4));
        locals.var_q_aexp_dn6 = ((locals.var_a0_dn6 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn6));
        locals.var_q_aexp_dn7 = ((locals.var_a0_dn7 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn7));
        locals.var_q_aexp_dn8 = ((locals.var_a0_dn8 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn8));
        locals.var_q_aexp_dn9 = ((locals.var_a0_dn9 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn9));

        let assign13630_e12983: f64 = (locals.var_q_k1q1 * locals.var_q_k1q1);
        let assign13630_e12985: f64 = (assign13630_e12983 - locals.var_q_aexp);
        locals.var_q_qsq = assign13630_e12985;
        locals.var_q_qsq_dn4 = (((locals.var_q_k1q1_dn4 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn4)) - locals.var_q_aexp_dn4);
        locals.var_q_qsq_dn6 = (((locals.var_q_k1q1_dn6 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn6)) - locals.var_q_aexp_dn6);
        locals.var_q_qsq_dn7 = (((locals.var_q_k1q1_dn7 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn7)) - locals.var_q_aexp_dn7);
        locals.var_q_qsq_dn8 = (((locals.var_q_k1q1_dn8 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn8)) - locals.var_q_aexp_dn8);
        locals.var_q_qsq_dn9 = (((locals.var_q_k1q1_dn9 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn9)) - locals.var_q_aexp_dn9);

        let assign13640_e12988: f64 = (2.0 * locals.var_k1);
        let assign13640_e12990: f64 = (assign13640_e12988 * locals.var_q_k1q1);
        let assign13640_e12992: f64 = (assign13640_e12990 + locals.var_q_aexp);
        locals.var_q_d1_qsq = assign13640_e12992;
        locals.var_q_d1_qsq_dn4 = ((((2.0 * locals.var_k1_dn4) * locals.var_q_k1q1) + (assign13640_e12988 * locals.var_q_k1q1_dn4)) + locals.var_q_aexp_dn4);
        locals.var_q_d1_qsq_dn6 = ((((2.0 * locals.var_k1_dn6) * locals.var_q_k1q1) + (assign13640_e12988 * locals.var_q_k1q1_dn6)) + locals.var_q_aexp_dn6);
        locals.var_q_d1_qsq_dn7 = ((((2.0 * locals.var_k1_dn7) * locals.var_q_k1q1) + (assign13640_e12988 * locals.var_q_k1q1_dn7)) + locals.var_q_aexp_dn7);
        locals.var_q_d1_qsq_dn8 = ((((2.0 * locals.var_k1_dn8) * locals.var_q_k1q1) + (assign13640_e12988 * locals.var_q_k1q1_dn8)) + locals.var_q_aexp_dn8);
        locals.var_q_d1_qsq_dn9 = ((((2.0 * locals.var_k1_dn9) * locals.var_q_k1q1) + (assign13640_e12988 * locals.var_q_k1q1_dn9)) + locals.var_q_aexp_dn9);

        let assign13650_e12995: f64 = (2.0 * locals.var_k1);
        let assign13650_e12997: f64 = (assign13650_e12995 * locals.var_k1);
        let assign13650_e12999: f64 = (assign13650_e12997 - locals.var_q_aexp);
        locals.var_q_d2_qsq = assign13650_e12999;
        locals.var_q_d2_qsq_dn4 = ((((2.0 * locals.var_k1_dn4) * locals.var_k1) + (assign13650_e12995 * locals.var_k1_dn4)) - locals.var_q_aexp_dn4);
        locals.var_q_d2_qsq_dn6 = ((((2.0 * locals.var_k1_dn6) * locals.var_k1) + (assign13650_e12995 * locals.var_k1_dn6)) - locals.var_q_aexp_dn6);
        locals.var_q_d2_qsq_dn7 = ((((2.0 * locals.var_k1_dn7) * locals.var_k1) + (assign13650_e12995 * locals.var_k1_dn7)) - locals.var_q_aexp_dn7);
        locals.var_q_d2_qsq_dn8 = ((((2.0 * locals.var_k1_dn8) * locals.var_k1) + (assign13650_e12995 * locals.var_k1_dn8)) - locals.var_q_aexp_dn8);
        locals.var_q_d2_qsq_dn9 = ((((2.0 * locals.var_k1_dn9) * locals.var_k1) + (assign13650_e12995 * locals.var_k1_dn9)) - locals.var_q_aexp_dn9);

        let assign13660_e13002: f64 = (-0.005);
        let assign13660_e13003: f64 = if locals.var_q_qsq < assign13660_e13002 { 1.0 } else { 0.0 };
        locals.var_guard573 = assign13660_e13003;

        let (assign13670_e13009, assign13670_e13009_d_n4, assign13670_e13009_d_n6, assign13670_e13009_d_n7, assign13670_e13009_d_n8, assign13670_e13009_d_n9,) = {
    if (locals.var_guard573 != 0.0) {
        let assign13670_e13006: f64 = (locals.var_q_qsq).abs();
        let assign13670_e13007: f64 = (assign13670_e13006).sqrt();
        (assign13670_e13007, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign13670_e13007)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign13670_e13007)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign13670_e13007)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign13670_e13007)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign13670_e13007)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign13670_e13009;
        locals.var_q_rac_qsq_dn4 = assign13670_e13009_d_n4;
        locals.var_q_rac_qsq_dn6 = assign13670_e13009_d_n6;
        locals.var_q_rac_qsq_dn7 = assign13670_e13009_d_n7;
        locals.var_q_rac_qsq_dn8 = assign13670_e13009_d_n8;
        locals.var_q_rac_qsq_dn9 = assign13670_e13009_d_n9;

        let (assign13680_e13018, assign13680_e13018_d_n4, assign13680_e13018_d_n6, assign13680_e13018_d_n7, assign13680_e13018_d_n8, assign13680_e13018_d_n9,) = {
    if (locals.var_guard573 != 0.0) {
        let assign13680_e13014: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign13680_e13015: f64 = (assign13680_e13014).tan();
        let assign13680_e13016: f64 = (locals.var_q_rac_qsq / assign13680_e13015);
        (assign13680_e13016, (((locals.var_q_rac_qsq_dn4 * assign13680_e13015) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn4) / ((assign13680_e13014).cos() * (assign13680_e13014).cos())))) / (assign13680_e13015 * assign13680_e13015)), (((locals.var_q_rac_qsq_dn6 * assign13680_e13015) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn6) / ((assign13680_e13014).cos() * (assign13680_e13014).cos())))) / (assign13680_e13015 * assign13680_e13015)), (((locals.var_q_rac_qsq_dn7 * assign13680_e13015) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn7) / ((assign13680_e13014).cos() * (assign13680_e13014).cos())))) / (assign13680_e13015 * assign13680_e13015)), (((locals.var_q_rac_qsq_dn8 * assign13680_e13015) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn8) / ((assign13680_e13014).cos() * (assign13680_e13014).cos())))) / (assign13680_e13015 * assign13680_e13015)), (((locals.var_q_rac_qsq_dn9 * assign13680_e13015) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn9) / ((assign13680_e13014).cos() * (assign13680_e13014).cos())))) / (assign13680_e13015 * assign13680_e13015)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign13680_e13018;
        locals.var_q_qcoth_dn4 = assign13680_e13018_d_n4;
        locals.var_q_qcoth_dn6 = assign13680_e13018_d_n6;
        locals.var_q_qcoth_dn7 = assign13680_e13018_d_n7;
        locals.var_q_qcoth_dn8 = assign13680_e13018_d_n8;
        locals.var_q_qcoth_dn9 = assign13680_e13018_d_n9;

        let (assign13690_e13026, assign13690_e13026_d_n4, assign13690_e13026_d_n6, assign13690_e13026_d_n7, assign13690_e13026_d_n8, assign13690_e13026_d_n9,) = {
    if (locals.var_guard573 != 0.0) {
        let assign13690_e13022: f64 = (0.25 * locals.var_q_d1_qsq);
        let assign13690_e13024: f64 = (assign13690_e13022 / locals.var_q_qsq);
        (assign13690_e13024, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign13690_e13022 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign13690_e13022 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign13690_e13022 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign13690_e13022 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign13690_e13022 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign13690_e13026;
        locals.var_q_temp1_dn4 = assign13690_e13026_d_n4;
        locals.var_q_temp1_dn6 = assign13690_e13026_d_n6;
        locals.var_q_temp1_dn7 = assign13690_e13026_d_n7;
        locals.var_q_temp1_dn8 = assign13690_e13026_d_n8;
        locals.var_q_temp1_dn9 = assign13690_e13026_d_n9;

        let (assign13700_e13038, assign13700_e13038_d_n4, assign13700_e13038_d_n6, assign13700_e13038_d_n7, assign13700_e13038_d_n8, assign13700_e13038_d_n9,) = {
    if (locals.var_guard573 != 0.0) {
        let assign13700_e13032: f64 = (2.0 - locals.var_q_qcoth);
        let assign13700_e13033: f64 = (locals.var_q_qcoth * assign13700_e13032);
        let assign13700_e13034: f64 = (locals.var_q_qsq + assign13700_e13033);
        let assign13700_e13036: f64 = (assign13700_e13034 * locals.var_q_temp1);
        (assign13700_e13036, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign13700_e13032) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign13700_e13034 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign13700_e13032) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign13700_e13034 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign13700_e13032) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign13700_e13034 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign13700_e13032) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign13700_e13034 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign13700_e13032) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign13700_e13034 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign13700_e13038;
        locals.var_q_d1_qcoth_dn4 = assign13700_e13038_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign13700_e13038_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign13700_e13038_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign13700_e13038_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign13700_e13038_d_n9;

        let (assign13710_e13058, assign13710_e13058_d_n4, assign13710_e13058_d_n6, assign13710_e13058_d_n7, assign13710_e13058_d_n8, assign13710_e13058_d_n9,) = {
    if (locals.var_guard573 != 0.0) {
        let assign13710_e13043: f64 = (2.0 * locals.var_q_d1_qcoth);
        let assign13710_e13046: f64 = (1.0 + locals.var_q_qcoth);
        let assign13710_e13047: f64 = (assign13710_e13043 * assign13710_e13046);
        let assign13710_e13048: f64 = (locals.var_q_d1_qsq - assign13710_e13047);
        let assign13710_e13050: f64 = (assign13710_e13048 * locals.var_q_temp1);
        let assign13710_e13053: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
        let assign13710_e13055: f64 = (assign13710_e13053 / locals.var_q_d1_qsq);
        let assign13710_e13056: f64 = (assign13710_e13050 + assign13710_e13055);
        (assign13710_e13056, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign13710_e13046) + (assign13710_e13043 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign13710_e13048 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign13710_e13053 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign13710_e13046) + (assign13710_e13043 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign13710_e13048 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign13710_e13053 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign13710_e13046) + (assign13710_e13043 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign13710_e13048 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign13710_e13053 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign13710_e13046) + (assign13710_e13043 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign13710_e13048 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign13710_e13053 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign13710_e13046) + (assign13710_e13043 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign13710_e13048 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign13710_e13053 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign13710_e13058;
        locals.var_q_d2_qcoth_dn4 = assign13710_e13058_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign13710_e13058_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign13710_e13058_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign13710_e13058_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign13710_e13058_d_n9;

        let (assign13720_e13066, assign13720_e13066_d_n4, assign13720_e13066_d_n6, assign13720_e13066_d_n7, assign13720_e13066_d_n8, assign13720_e13066_d_n9,) = {
    if (locals.var_guard573 != 0.0) {
        let assign13720_e13063: f64 = (0.5 * locals.var_q_qcoth);
        let assign13720_e13064: f64 = (1.0 - assign13720_e13063);
        (assign13720_e13064, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign13720_e13066;
        locals.var_q_temp2_dn4 = assign13720_e13066_d_n4;
        locals.var_q_temp2_dn6 = assign13720_e13066_d_n6;
        locals.var_q_temp2_dn7 = assign13720_e13066_d_n7;
        locals.var_q_temp2_dn8 = assign13720_e13066_d_n8;
        locals.var_q_temp2_dn9 = assign13720_e13066_d_n9;

        let (assign13730_e13074, assign13730_e13074_d_n4, assign13730_e13074_d_n6, assign13730_e13074_d_n7, assign13730_e13074_d_n8, assign13730_e13074_d_n9,) = {
    if (locals.var_guard573 != 0.0) {
        let assign13730_e13070: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
        let assign13730_e13072: f64 = (assign13730_e13070 * locals.var_q_temp2);
        (assign13730_e13072, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13730_e13070 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13730_e13070 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13730_e13070 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13730_e13070 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13730_e13070 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign13730_e13074;
        locals.var_q_d1_ln_dn4 = assign13730_e13074_d_n4;
        locals.var_q_d1_ln_dn6 = assign13730_e13074_d_n6;
        locals.var_q_d1_ln_dn7 = assign13730_e13074_d_n7;
        locals.var_q_d1_ln_dn8 = assign13730_e13074_d_n8;
        locals.var_q_d1_ln_dn9 = assign13730_e13074_d_n9;

        let (assign13740_e13090, assign13740_e13090_d_n4, assign13740_e13090_d_n6, assign13740_e13090_d_n7, assign13740_e13090_d_n8, assign13740_e13090_d_n9,) = {
    if (locals.var_guard573 != 0.0) {
        let assign13740_e13078: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
        let assign13740_e13083: f64 = (0.5 * locals.var_q_d1_qcoth);
        let assign13740_e13084: f64 = (locals.var_q_d1_ln + assign13740_e13083);
        let assign13740_e13085: f64 = (locals.var_q_d1_qsq * assign13740_e13084);
        let assign13740_e13086: f64 = (assign13740_e13078 - assign13740_e13085);
        let assign13740_e13088: f64 = (assign13740_e13086 / locals.var_q_qsq);
        (assign13740_e13088, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign13740_e13084) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign13740_e13086 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign13740_e13084) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign13740_e13086 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign13740_e13084) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign13740_e13086 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign13740_e13084) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign13740_e13086 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign13740_e13084) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign13740_e13086 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign13740_e13090;
        locals.var_q_d2_ln_dn4 = assign13740_e13090_d_n4;
        locals.var_q_d2_ln_dn6 = assign13740_e13090_d_n6;
        locals.var_q_d2_ln_dn7 = assign13740_e13090_d_n7;
        locals.var_q_d2_ln_dn8 = assign13740_e13090_d_n8;
        locals.var_q_d2_ln_dn9 = assign13740_e13090_d_n9;

        let assign13750_e13093: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard574 = assign13750_e13093;

        let (assign13760_e13102, assign13760_e13102_d_n4, assign13760_e13102_d_n6, assign13760_e13102_d_n7, assign13760_e13102_d_n8, assign13760_e13102_d_n9,) = {
    if ((locals.var_guard573 == 0.0) && (locals.var_guard574 != 0.0)) {
        let assign13760_e13099: f64 = (locals.var_q_qsq).abs();
        let assign13760_e13100: f64 = (assign13760_e13099).sqrt();
        (assign13760_e13100, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign13760_e13100)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign13760_e13100)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign13760_e13100)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign13760_e13100)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign13760_e13100)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign13760_e13102;
        locals.var_q_rac_qsq_dn4 = assign13760_e13102_d_n4;
        locals.var_q_rac_qsq_dn6 = assign13760_e13102_d_n6;
        locals.var_q_rac_qsq_dn7 = assign13760_e13102_d_n7;
        locals.var_q_rac_qsq_dn8 = assign13760_e13102_d_n8;
        locals.var_q_rac_qsq_dn9 = assign13760_e13102_d_n9;

        let (assign13770_e13111, assign13770_e13111_d_n4, assign13770_e13111_d_n6, assign13770_e13111_d_n7, assign13770_e13111_d_n8, assign13770_e13111_d_n9,) = {
    if ((locals.var_guard573 == 0.0) && (locals.var_guard574 != 0.0)) {
        let assign13770_e13108: f64 = (-locals.var_q_rac_qsq);
        let assign13770_e13109: f64 = (assign13770_e13108).exp();
        (assign13770_e13109, (assign13770_e13109 * (-locals.var_q_rac_qsq_dn4)), (assign13770_e13109 * (-locals.var_q_rac_qsq_dn6)), (assign13770_e13109 * (-locals.var_q_rac_qsq_dn7)), (assign13770_e13109 * (-locals.var_q_rac_qsq_dn8)), (assign13770_e13109 * (-locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_invexpq, locals.var_q_invexpq_dn4, locals.var_q_invexpq_dn6, locals.var_q_invexpq_dn7, locals.var_q_invexpq_dn8, locals.var_q_invexpq_dn9,)
    }
};
        locals.var_q_invexpq = assign13770_e13111;
        locals.var_q_invexpq_dn4 = assign13770_e13111_d_n4;
        locals.var_q_invexpq_dn6 = assign13770_e13111_d_n6;
        locals.var_q_invexpq_dn7 = assign13770_e13111_d_n7;
        locals.var_q_invexpq_dn8 = assign13770_e13111_d_n8;
        locals.var_q_invexpq_dn9 = assign13770_e13111_d_n9;

        let (assign13780_e13126, assign13780_e13126_d_n4, assign13780_e13126_d_n6, assign13780_e13126_d_n7, assign13780_e13126_d_n8, assign13780_e13126_d_n9,) = {
    if ((locals.var_guard573 == 0.0) && (locals.var_guard574 != 0.0)) {
        let assign13780_e13119: f64 = (1.0 + locals.var_q_invexpq);
        let assign13780_e13120: f64 = (locals.var_q_rac_qsq * assign13780_e13119);
        let assign13780_e13123: f64 = (1.0 - locals.var_q_invexpq);
        let assign13780_e13124: f64 = (assign13780_e13120 / assign13780_e13123);
        (assign13780_e13124, (((((locals.var_q_rac_qsq_dn4 * assign13780_e13119) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn4)) * assign13780_e13123) - (assign13780_e13120 * (-locals.var_q_invexpq_dn4))) / (assign13780_e13123 * assign13780_e13123)), (((((locals.var_q_rac_qsq_dn6 * assign13780_e13119) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn6)) * assign13780_e13123) - (assign13780_e13120 * (-locals.var_q_invexpq_dn6))) / (assign13780_e13123 * assign13780_e13123)), (((((locals.var_q_rac_qsq_dn7 * assign13780_e13119) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn7)) * assign13780_e13123) - (assign13780_e13120 * (-locals.var_q_invexpq_dn7))) / (assign13780_e13123 * assign13780_e13123)), (((((locals.var_q_rac_qsq_dn8 * assign13780_e13119) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn8)) * assign13780_e13123) - (assign13780_e13120 * (-locals.var_q_invexpq_dn8))) / (assign13780_e13123 * assign13780_e13123)), (((((locals.var_q_rac_qsq_dn9 * assign13780_e13119) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn9)) * assign13780_e13123) - (assign13780_e13120 * (-locals.var_q_invexpq_dn9))) / (assign13780_e13123 * assign13780_e13123)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign13780_e13126;
        locals.var_q_qcoth_dn4 = assign13780_e13126_d_n4;
        locals.var_q_qcoth_dn6 = assign13780_e13126_d_n6;
        locals.var_q_qcoth_dn7 = assign13780_e13126_d_n7;
        locals.var_q_qcoth_dn8 = assign13780_e13126_d_n8;
        locals.var_q_qcoth_dn9 = assign13780_e13126_d_n9;

    }

    pub(super) fn stamp_transient_block_33(
        locals: &mut StampLocals,
    ) {
        let (assign13790_e13137, assign13790_e13137_d_n4, assign13790_e13137_d_n6, assign13790_e13137_d_n7, assign13790_e13137_d_n8, assign13790_e13137_d_n9,) = {
    if ((locals.var_guard573 == 0.0) && (locals.var_guard574 != 0.0)) {
        let assign13790_e13133: f64 = (0.25 * locals.var_q_d1_qsq);
        let assign13790_e13135: f64 = (assign13790_e13133 / locals.var_q_qsq);
        (assign13790_e13135, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign13790_e13133 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign13790_e13133 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign13790_e13133 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign13790_e13133 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign13790_e13133 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign13790_e13137;
        locals.var_q_temp1_dn4 = assign13790_e13137_d_n4;
        locals.var_q_temp1_dn6 = assign13790_e13137_d_n6;
        locals.var_q_temp1_dn7 = assign13790_e13137_d_n7;
        locals.var_q_temp1_dn8 = assign13790_e13137_d_n8;
        locals.var_q_temp1_dn9 = assign13790_e13137_d_n9;

        let (assign13800_e13152, assign13800_e13152_d_n4, assign13800_e13152_d_n6, assign13800_e13152_d_n7, assign13800_e13152_d_n8, assign13800_e13152_d_n9,) = {
    if ((locals.var_guard573 == 0.0) && (locals.var_guard574 != 0.0)) {
        let assign13800_e13146: f64 = (2.0 - locals.var_q_qcoth);
        let assign13800_e13147: f64 = (locals.var_q_qcoth * assign13800_e13146);
        let assign13800_e13148: f64 = (locals.var_q_qsq + assign13800_e13147);
        let assign13800_e13150: f64 = (assign13800_e13148 * locals.var_q_temp1);
        (assign13800_e13150, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign13800_e13146) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign13800_e13148 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign13800_e13146) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign13800_e13148 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign13800_e13146) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign13800_e13148 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign13800_e13146) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign13800_e13148 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign13800_e13146) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign13800_e13148 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign13800_e13152;
        locals.var_q_d1_qcoth_dn4 = assign13800_e13152_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign13800_e13152_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign13800_e13152_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign13800_e13152_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign13800_e13152_d_n9;

        let (assign13810_e13175, assign13810_e13175_d_n4, assign13810_e13175_d_n6, assign13810_e13175_d_n7, assign13810_e13175_d_n8, assign13810_e13175_d_n9,) = {
    if ((locals.var_guard573 == 0.0) && (locals.var_guard574 != 0.0)) {
        let assign13810_e13160: f64 = (2.0 * locals.var_q_d1_qcoth);
        let assign13810_e13163: f64 = (1.0 + locals.var_q_qcoth);
        let assign13810_e13164: f64 = (assign13810_e13160 * assign13810_e13163);
        let assign13810_e13165: f64 = (locals.var_q_d1_qsq - assign13810_e13164);
        let assign13810_e13167: f64 = (assign13810_e13165 * locals.var_q_temp1);
        let assign13810_e13170: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
        let assign13810_e13172: f64 = (assign13810_e13170 / locals.var_q_d1_qsq);
        let assign13810_e13173: f64 = (assign13810_e13167 + assign13810_e13172);
        (assign13810_e13173, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign13810_e13163) + (assign13810_e13160 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign13810_e13165 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign13810_e13170 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign13810_e13163) + (assign13810_e13160 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign13810_e13165 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign13810_e13170 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign13810_e13163) + (assign13810_e13160 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign13810_e13165 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign13810_e13170 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign13810_e13163) + (assign13810_e13160 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign13810_e13165 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign13810_e13170 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign13810_e13163) + (assign13810_e13160 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign13810_e13165 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign13810_e13170 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign13810_e13175;
        locals.var_q_d2_qcoth_dn4 = assign13810_e13175_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign13810_e13175_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign13810_e13175_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign13810_e13175_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign13810_e13175_d_n9;

        let (assign13820_e13186, assign13820_e13186_d_n4, assign13820_e13186_d_n6, assign13820_e13186_d_n7, assign13820_e13186_d_n8, assign13820_e13186_d_n9,) = {
    if ((locals.var_guard573 == 0.0) && (locals.var_guard574 != 0.0)) {
        let assign13820_e13183: f64 = (0.5 * locals.var_q_qcoth);
        let assign13820_e13184: f64 = (1.0 - assign13820_e13183);
        (assign13820_e13184, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign13820_e13186;
        locals.var_q_temp2_dn4 = assign13820_e13186_d_n4;
        locals.var_q_temp2_dn6 = assign13820_e13186_d_n6;
        locals.var_q_temp2_dn7 = assign13820_e13186_d_n7;
        locals.var_q_temp2_dn8 = assign13820_e13186_d_n8;
        locals.var_q_temp2_dn9 = assign13820_e13186_d_n9;

        let (assign13830_e13197, assign13830_e13197_d_n4, assign13830_e13197_d_n6, assign13830_e13197_d_n7, assign13830_e13197_d_n8, assign13830_e13197_d_n9,) = {
    if ((locals.var_guard573 == 0.0) && (locals.var_guard574 != 0.0)) {
        let assign13830_e13193: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
        let assign13830_e13195: f64 = (assign13830_e13193 * locals.var_q_temp2);
        (assign13830_e13195, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13830_e13193 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13830_e13193 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13830_e13193 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13830_e13193 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13830_e13193 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign13830_e13197;
        locals.var_q_d1_ln_dn4 = assign13830_e13197_d_n4;
        locals.var_q_d1_ln_dn6 = assign13830_e13197_d_n6;
        locals.var_q_d1_ln_dn7 = assign13830_e13197_d_n7;
        locals.var_q_d1_ln_dn8 = assign13830_e13197_d_n8;
        locals.var_q_d1_ln_dn9 = assign13830_e13197_d_n9;

        let (assign13840_e13216, assign13840_e13216_d_n4, assign13840_e13216_d_n6, assign13840_e13216_d_n7, assign13840_e13216_d_n8, assign13840_e13216_d_n9,) = {
    if ((locals.var_guard573 == 0.0) && (locals.var_guard574 != 0.0)) {
        let assign13840_e13204: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
        let assign13840_e13209: f64 = (0.5 * locals.var_q_d1_qcoth);
        let assign13840_e13210: f64 = (locals.var_q_d1_ln + assign13840_e13209);
        let assign13840_e13211: f64 = (locals.var_q_d1_qsq * assign13840_e13210);
        let assign13840_e13212: f64 = (assign13840_e13204 - assign13840_e13211);
        let assign13840_e13214: f64 = (assign13840_e13212 / locals.var_q_qsq);
        (assign13840_e13214, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign13840_e13210) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign13840_e13212 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign13840_e13210) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign13840_e13212 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign13840_e13210) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign13840_e13212 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign13840_e13210) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign13840_e13212 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign13840_e13210) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign13840_e13212 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign13840_e13216;
        locals.var_q_d2_ln_dn4 = assign13840_e13216_d_n4;
        locals.var_q_d2_ln_dn6 = assign13840_e13216_d_n6;
        locals.var_q_d2_ln_dn7 = assign13840_e13216_d_n7;
        locals.var_q_d2_ln_dn8 = assign13840_e13216_d_n8;
        locals.var_q_d2_ln_dn9 = assign13840_e13216_d_n9;

        let (assign13850_e13242, assign13850_e13242_d_n4, assign13850_e13242_d_n6, assign13850_e13242_d_n7, assign13850_e13242_d_n8, assign13850_e13242_d_n9,) = {
    if ((locals.var_guard573 == 0.0) && (locals.var_guard574 == 0.0)) {
        let assign13850_e13226: f64 = (locals.var_q_qsq * 0.0166666666667);
        let assign13850_e13230: f64 = (locals.var_q_qsq * 0.0238095238095);
        let assign13850_e13234: f64 = (locals.var_q_qsq * 0.025);
        let assign13850_e13235: f64 = (1.0 - assign13850_e13234);
        let assign13850_e13236: f64 = (assign13850_e13230 * assign13850_e13235);
        let assign13850_e13237: f64 = (1.0 - assign13850_e13236);
        let assign13850_e13238: f64 = (assign13850_e13226 * assign13850_e13237);
        let assign13850_e13239: f64 = (1.0 - assign13850_e13238);
        let assign13850_e13240: f64 = (0.1666666666667 * assign13850_e13239);
        (assign13850_e13240, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0166666666667) * assign13850_e13237) + (assign13850_e13226 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign13850_e13235) + (assign13850_e13230 * (-(locals.var_q_qsq_dn4 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0166666666667) * assign13850_e13237) + (assign13850_e13226 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign13850_e13235) + (assign13850_e13230 * (-(locals.var_q_qsq_dn6 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0166666666667) * assign13850_e13237) + (assign13850_e13226 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign13850_e13235) + (assign13850_e13230 * (-(locals.var_q_qsq_dn7 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0166666666667) * assign13850_e13237) + (assign13850_e13226 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign13850_e13235) + (assign13850_e13230 * (-(locals.var_q_qsq_dn8 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0166666666667) * assign13850_e13237) + (assign13850_e13226 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign13850_e13235) + (assign13850_e13230 * (-(locals.var_q_qsq_dn9 * 0.025))))))))),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign13850_e13242;
        locals.var_q_temp3_dn4 = assign13850_e13242_d_n4;
        locals.var_q_temp3_dn6 = assign13850_e13242_d_n6;
        locals.var_q_temp3_dn7 = assign13850_e13242_d_n7;
        locals.var_q_temp3_dn8 = assign13850_e13242_d_n8;
        locals.var_q_temp3_dn9 = assign13850_e13242_d_n9;

        let (assign13860_e13254, assign13860_e13254_d_n4, assign13860_e13254_d_n6, assign13860_e13254_d_n7, assign13860_e13254_d_n8, assign13860_e13254_d_n9,) = {
    if ((locals.var_guard573 == 0.0) && (locals.var_guard574 == 0.0)) {
        let assign13860_e13251: f64 = (locals.var_q_qsq * locals.var_q_temp3);
        let assign13860_e13252: f64 = (2.0 + assign13860_e13251);
        (assign13860_e13252, ((locals.var_q_qsq_dn4 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn4)), ((locals.var_q_qsq_dn6 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn6)), ((locals.var_q_qsq_dn7 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn7)), ((locals.var_q_qsq_dn8 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn8)), ((locals.var_q_qsq_dn9 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn9)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign13860_e13254;
        locals.var_q_qcoth_dn4 = assign13860_e13254_d_n4;
        locals.var_q_qcoth_dn6 = assign13860_e13254_d_n6;
        locals.var_q_qcoth_dn7 = assign13860_e13254_d_n7;
        locals.var_q_qcoth_dn8 = assign13860_e13254_d_n8;
        locals.var_q_qcoth_dn9 = assign13860_e13254_d_n9;

        let (assign13870_e13280, assign13870_e13280_d_n4, assign13870_e13280_d_n6, assign13870_e13280_d_n7, assign13870_e13280_d_n8, assign13870_e13280_d_n9,) = {
    if ((locals.var_guard573 == 0.0) && (locals.var_guard574 == 0.0)) {
        let assign13870_e13264: f64 = (locals.var_q_qsq * 0.0333333333333);
        let assign13870_e13268: f64 = (locals.var_q_qsq * 0.0357142857143);
        let assign13870_e13272: f64 = (locals.var_q_qsq * 0.0333333333333);
        let assign13870_e13273: f64 = (1.0 - assign13870_e13272);
        let assign13870_e13274: f64 = (assign13870_e13268 * assign13870_e13273);
        let assign13870_e13275: f64 = (1.0 - assign13870_e13274);
        let assign13870_e13276: f64 = (assign13870_e13264 * assign13870_e13275);
        let assign13870_e13277: f64 = (1.0 - assign13870_e13276);
        let assign13870_e13278: f64 = (0.1666666666667 * assign13870_e13277);
        (assign13870_e13278, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0333333333333) * assign13870_e13275) + (assign13870_e13264 * (-(((locals.var_q_qsq_dn4 * 0.0357142857143) * assign13870_e13273) + (assign13870_e13268 * (-(locals.var_q_qsq_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0333333333333) * assign13870_e13275) + (assign13870_e13264 * (-(((locals.var_q_qsq_dn6 * 0.0357142857143) * assign13870_e13273) + (assign13870_e13268 * (-(locals.var_q_qsq_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0333333333333) * assign13870_e13275) + (assign13870_e13264 * (-(((locals.var_q_qsq_dn7 * 0.0357142857143) * assign13870_e13273) + (assign13870_e13268 * (-(locals.var_q_qsq_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0333333333333) * assign13870_e13275) + (assign13870_e13264 * (-(((locals.var_q_qsq_dn8 * 0.0357142857143) * assign13870_e13273) + (assign13870_e13268 * (-(locals.var_q_qsq_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0333333333333) * assign13870_e13275) + (assign13870_e13264 * (-(((locals.var_q_qsq_dn9 * 0.0357142857143) * assign13870_e13273) + (assign13870_e13268 * (-(locals.var_q_qsq_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign13870_e13280;
        locals.var_q_temp1_dn4 = assign13870_e13280_d_n4;
        locals.var_q_temp1_dn6 = assign13870_e13280_d_n6;
        locals.var_q_temp1_dn7 = assign13870_e13280_d_n7;
        locals.var_q_temp1_dn8 = assign13870_e13280_d_n8;
        locals.var_q_temp1_dn9 = assign13870_e13280_d_n9;

        let (assign13880_e13290, assign13880_e13290_d_n4, assign13880_e13290_d_n6, assign13880_e13290_d_n7, assign13880_e13290_d_n8, assign13880_e13290_d_n9,) = {
    if ((locals.var_guard573 == 0.0) && (locals.var_guard574 == 0.0)) {
        let assign13880_e13288: f64 = (locals.var_q_d1_qsq * locals.var_q_temp1);
        (assign13880_e13288, ((locals.var_q_d1_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn4)), ((locals.var_q_d1_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn6)), ((locals.var_q_d1_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn7)), ((locals.var_q_d1_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn8)), ((locals.var_q_d1_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign13880_e13290;
        locals.var_q_d1_qcoth_dn4 = assign13880_e13290_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign13880_e13290_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign13880_e13290_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign13880_e13290_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign13880_e13290_d_n9;

        let (assign13890_e13316, assign13890_e13316_d_n4, assign13890_e13316_d_n6, assign13890_e13316_d_n7, assign13890_e13316_d_n8, assign13890_e13316_d_n9,) = {
    if ((locals.var_guard573 == 0.0) && (locals.var_guard574 == 0.0)) {
        let assign13890_e13300: f64 = (locals.var_q_qsq * 0.0714285714286);
        let assign13890_e13304: f64 = (0.05 * locals.var_q_qsq);
        let assign13890_e13308: f64 = (0.0420875420875421 * locals.var_q_qsq);
        let assign13890_e13309: f64 = (1.0 - assign13890_e13308);
        let assign13890_e13310: f64 = (assign13890_e13304 * assign13890_e13309);
        let assign13890_e13311: f64 = (1.0 - assign13890_e13310);
        let assign13890_e13312: f64 = (assign13890_e13300 * assign13890_e13311);
        let assign13890_e13313: f64 = (1.0 - assign13890_e13312);
        let assign13890_e13314: f64 = (0.0055555555556 * assign13890_e13313);
        (assign13890_e13314, (0.0055555555556 * (-(((locals.var_q_qsq_dn4 * 0.0714285714286) * assign13890_e13311) + (assign13890_e13300 * (-(((0.05 * locals.var_q_qsq_dn4) * assign13890_e13309) + (assign13890_e13304 * (-(0.0420875420875421 * locals.var_q_qsq_dn4))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn6 * 0.0714285714286) * assign13890_e13311) + (assign13890_e13300 * (-(((0.05 * locals.var_q_qsq_dn6) * assign13890_e13309) + (assign13890_e13304 * (-(0.0420875420875421 * locals.var_q_qsq_dn6))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn7 * 0.0714285714286) * assign13890_e13311) + (assign13890_e13300 * (-(((0.05 * locals.var_q_qsq_dn7) * assign13890_e13309) + (assign13890_e13304 * (-(0.0420875420875421 * locals.var_q_qsq_dn7))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn8 * 0.0714285714286) * assign13890_e13311) + (assign13890_e13300 * (-(((0.05 * locals.var_q_qsq_dn8) * assign13890_e13309) + (assign13890_e13304 * (-(0.0420875420875421 * locals.var_q_qsq_dn8))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn9 * 0.0714285714286) * assign13890_e13311) + (assign13890_e13300 * (-(((0.05 * locals.var_q_qsq_dn9) * assign13890_e13309) + (assign13890_e13304 * (-(0.0420875420875421 * locals.var_q_qsq_dn9))))))))),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign13890_e13316;
        locals.var_q_temp2_dn4 = assign13890_e13316_d_n4;
        locals.var_q_temp2_dn6 = assign13890_e13316_d_n6;
        locals.var_q_temp2_dn7 = assign13890_e13316_d_n7;
        locals.var_q_temp2_dn8 = assign13890_e13316_d_n8;
        locals.var_q_temp2_dn9 = assign13890_e13316_d_n9;

        let (assign13900_e13332, assign13900_e13332_d_n4, assign13900_e13332_d_n6, assign13900_e13332_d_n7, assign13900_e13332_d_n8, assign13900_e13332_d_n9,) = {
    if ((locals.var_guard573 == 0.0) && (locals.var_guard574 == 0.0)) {
        let assign13900_e13324: f64 = (locals.var_q_d2_qsq * locals.var_q_temp1);
        let assign13900_e13327: f64 = (locals.var_q_d1_qsq * locals.var_q_d1_qsq);
        let assign13900_e13329: f64 = (assign13900_e13327 * locals.var_q_temp2);
        let assign13900_e13330: f64 = (assign13900_e13324 - assign13900_e13329);
        (assign13900_e13330, (((locals.var_q_d2_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn4)) - ((((locals.var_q_d1_qsq_dn4 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn4)) * locals.var_q_temp2) + (assign13900_e13327 * locals.var_q_temp2_dn4))), (((locals.var_q_d2_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn6)) - ((((locals.var_q_d1_qsq_dn6 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn6)) * locals.var_q_temp2) + (assign13900_e13327 * locals.var_q_temp2_dn6))), (((locals.var_q_d2_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn7)) - ((((locals.var_q_d1_qsq_dn7 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn7)) * locals.var_q_temp2) + (assign13900_e13327 * locals.var_q_temp2_dn7))), (((locals.var_q_d2_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn8)) - ((((locals.var_q_d1_qsq_dn8 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn8)) * locals.var_q_temp2) + (assign13900_e13327 * locals.var_q_temp2_dn8))), (((locals.var_q_d2_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn9)) - ((((locals.var_q_d1_qsq_dn9 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn9)) * locals.var_q_temp2) + (assign13900_e13327 * locals.var_q_temp2_dn9))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign13900_e13332;
        locals.var_q_d2_qcoth_dn4 = assign13900_e13332_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign13900_e13332_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign13900_e13332_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign13900_e13332_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign13900_e13332_d_n9;

        let (assign13910_e13345, assign13910_e13345_d_n4, assign13910_e13345_d_n6, assign13910_e13345_d_n7, assign13910_e13345_d_n8, assign13910_e13345_d_n9,) = {
    if ((locals.var_guard573 == 0.0) && (locals.var_guard574 == 0.0)) {
        let assign13910_e13339: f64 = (-0.5);
        let assign13910_e13341: f64 = (assign13910_e13339 * locals.var_q_d1_qsq);
        let assign13910_e13343: f64 = (assign13910_e13341 * locals.var_q_temp3);
        (assign13910_e13343, (((assign13910_e13339 * locals.var_q_d1_qsq_dn4) * locals.var_q_temp3) + (assign13910_e13341 * locals.var_q_temp3_dn4)), (((assign13910_e13339 * locals.var_q_d1_qsq_dn6) * locals.var_q_temp3) + (assign13910_e13341 * locals.var_q_temp3_dn6)), (((assign13910_e13339 * locals.var_q_d1_qsq_dn7) * locals.var_q_temp3) + (assign13910_e13341 * locals.var_q_temp3_dn7)), (((assign13910_e13339 * locals.var_q_d1_qsq_dn8) * locals.var_q_temp3) + (assign13910_e13341 * locals.var_q_temp3_dn8)), (((assign13910_e13339 * locals.var_q_d1_qsq_dn9) * locals.var_q_temp3) + (assign13910_e13341 * locals.var_q_temp3_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign13910_e13345;
        locals.var_q_d1_ln_dn4 = assign13910_e13345_d_n4;
        locals.var_q_d1_ln_dn6 = assign13910_e13345_d_n6;
        locals.var_q_d1_ln_dn7 = assign13910_e13345_d_n7;
        locals.var_q_d1_ln_dn8 = assign13910_e13345_d_n8;
        locals.var_q_d1_ln_dn9 = assign13910_e13345_d_n9;

        let (assign13920_e13378, assign13920_e13378_d_n4, assign13920_e13378_d_n6, assign13920_e13378_d_n7, assign13920_e13378_d_n8, assign13920_e13378_d_n9,) = {
    if ((locals.var_guard573 == 0.0) && (locals.var_guard574 == 0.0)) {
        let assign13920_e13352: f64 = (-0.5);
        let assign13920_e13354: f64 = (assign13920_e13352 * locals.var_q_d2_qsq);
        let assign13920_e13356: f64 = (assign13920_e13354 * locals.var_q_temp3);
        let assign13920_e13359: f64 = (0.25 * 0.0055555555556);
        let assign13920_e13361: f64 = (assign13920_e13359 * locals.var_q_d1_qsq);
        let assign13920_e13363: f64 = (assign13920_e13361 * locals.var_q_d1_qsq);
        let assign13920_e13367: f64 = (locals.var_q_qsq * 0.0238095238095);
        let assign13920_e13371: f64 = (0.075 * locals.var_q_qsq);
        let assign13920_e13372: f64 = (2.0 - assign13920_e13371);
        let assign13920_e13373: f64 = (assign13920_e13367 * assign13920_e13372);
        let assign13920_e13374: f64 = (1.0 - assign13920_e13373);
        let assign13920_e13375: f64 = (assign13920_e13363 * assign13920_e13374);
        let assign13920_e13376: f64 = (assign13920_e13356 + assign13920_e13375);
        (assign13920_e13376, ((((assign13920_e13352 * locals.var_q_d2_qsq_dn4) * locals.var_q_temp3) + (assign13920_e13354 * locals.var_q_temp3_dn4)) + (((((assign13920_e13359 * locals.var_q_d1_qsq_dn4) * locals.var_q_d1_qsq) + (assign13920_e13361 * locals.var_q_d1_qsq_dn4)) * assign13920_e13374) + (assign13920_e13363 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign13920_e13372) + (assign13920_e13367 * (-(0.075 * locals.var_q_qsq_dn4)))))))), ((((assign13920_e13352 * locals.var_q_d2_qsq_dn6) * locals.var_q_temp3) + (assign13920_e13354 * locals.var_q_temp3_dn6)) + (((((assign13920_e13359 * locals.var_q_d1_qsq_dn6) * locals.var_q_d1_qsq) + (assign13920_e13361 * locals.var_q_d1_qsq_dn6)) * assign13920_e13374) + (assign13920_e13363 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign13920_e13372) + (assign13920_e13367 * (-(0.075 * locals.var_q_qsq_dn6)))))))), ((((assign13920_e13352 * locals.var_q_d2_qsq_dn7) * locals.var_q_temp3) + (assign13920_e13354 * locals.var_q_temp3_dn7)) + (((((assign13920_e13359 * locals.var_q_d1_qsq_dn7) * locals.var_q_d1_qsq) + (assign13920_e13361 * locals.var_q_d1_qsq_dn7)) * assign13920_e13374) + (assign13920_e13363 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign13920_e13372) + (assign13920_e13367 * (-(0.075 * locals.var_q_qsq_dn7)))))))), ((((assign13920_e13352 * locals.var_q_d2_qsq_dn8) * locals.var_q_temp3) + (assign13920_e13354 * locals.var_q_temp3_dn8)) + (((((assign13920_e13359 * locals.var_q_d1_qsq_dn8) * locals.var_q_d1_qsq) + (assign13920_e13361 * locals.var_q_d1_qsq_dn8)) * assign13920_e13374) + (assign13920_e13363 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign13920_e13372) + (assign13920_e13367 * (-(0.075 * locals.var_q_qsq_dn8)))))))), ((((assign13920_e13352 * locals.var_q_d2_qsq_dn9) * locals.var_q_temp3) + (assign13920_e13354 * locals.var_q_temp3_dn9)) + (((((assign13920_e13359 * locals.var_q_d1_qsq_dn9) * locals.var_q_d1_qsq) + (assign13920_e13361 * locals.var_q_d1_qsq_dn9)) * assign13920_e13374) + (assign13920_e13363 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign13920_e13372) + (assign13920_e13367 * (-(0.075 * locals.var_q_qsq_dn9)))))))),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign13920_e13378;
        locals.var_q_d2_ln_dn4 = assign13920_e13378_d_n4;
        locals.var_q_d2_ln_dn6 = assign13920_e13378_d_n6;
        locals.var_q_d2_ln_dn7 = assign13920_e13378_d_n7;
        locals.var_q_d2_ln_dn8 = assign13920_e13378_d_n8;
        locals.var_q_d2_ln_dn9 = assign13920_e13378_d_n9;

        let assign13930_e13381: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard575 = assign13930_e13381;

        let (assign13940_e13395, assign13940_e13395_d_n4, assign13940_e13395_d_n6, assign13940_e13395_d_n7, assign13940_e13395_d_n8, assign13940_e13395_d_n9,) = {
    if (locals.var_guard575 != 0.0) {
        let assign13940_e13385: f64 = (4.0 * locals.var_q_qsq);
        let assign13940_e13390: f64 = (2.0 - locals.var_q_invexpq);
        let assign13940_e13391: f64 = (locals.var_q_invexpq * assign13940_e13390);
        let assign13940_e13392: f64 = (1.0 - assign13940_e13391);
        let assign13940_e13393: f64 = (assign13940_e13385 / assign13940_e13392);
        (assign13940_e13393, ((((4.0 * locals.var_q_qsq_dn4) * assign13940_e13392) - (assign13940_e13385 * (-((locals.var_q_invexpq_dn4 * assign13940_e13390) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn4)))))) / (assign13940_e13392 * assign13940_e13392)), ((((4.0 * locals.var_q_qsq_dn6) * assign13940_e13392) - (assign13940_e13385 * (-((locals.var_q_invexpq_dn6 * assign13940_e13390) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn6)))))) / (assign13940_e13392 * assign13940_e13392)), ((((4.0 * locals.var_q_qsq_dn7) * assign13940_e13392) - (assign13940_e13385 * (-((locals.var_q_invexpq_dn7 * assign13940_e13390) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn7)))))) / (assign13940_e13392 * assign13940_e13392)), ((((4.0 * locals.var_q_qsq_dn8) * assign13940_e13392) - (assign13940_e13385 * (-((locals.var_q_invexpq_dn8 * assign13940_e13390) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn8)))))) / (assign13940_e13392 * assign13940_e13392)), ((((4.0 * locals.var_q_qsq_dn9) * assign13940_e13392) - (assign13940_e13385 * (-((locals.var_q_invexpq_dn9 * assign13940_e13390) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn9)))))) / (assign13940_e13392 * assign13940_e13392)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign13940_e13395;
        locals.var_q_temp2_dn4 = assign13940_e13395_d_n4;
        locals.var_q_temp2_dn6 = assign13940_e13395_d_n6;
        locals.var_q_temp2_dn7 = assign13940_e13395_d_n7;
        locals.var_q_temp2_dn8 = assign13940_e13395_d_n8;
        locals.var_q_temp2_dn9 = assign13940_e13395_d_n9;

        let (assign13950_e13401, assign13950_e13401_d_n4, assign13950_e13401_d_n6, assign13950_e13401_d_n7, assign13950_e13401_d_n8, assign13950_e13401_d_n9,) = {
    if (locals.var_guard575 != 0.0) {
        let assign13950_e13399: f64 = (locals.var_q_temp2 * locals.var_q_invexpq);
        (assign13950_e13399, ((locals.var_q_temp2_dn4 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn4)), ((locals.var_q_temp2_dn6 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn6)), ((locals.var_q_temp2_dn7 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn7)), ((locals.var_q_temp2_dn8 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn8)), ((locals.var_q_temp2_dn9 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn9)),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign13950_e13401;
        locals.var_q_sh_term_dn4 = assign13950_e13401_d_n4;
        locals.var_q_sh_term_dn6 = assign13950_e13401_d_n6;
        locals.var_q_sh_term_dn7 = assign13950_e13401_d_n7;
        locals.var_q_sh_term_dn8 = assign13950_e13401_d_n8;
        locals.var_q_sh_term_dn9 = assign13950_e13401_d_n9;

        let (assign13960_e13408, assign13960_e13408_d_n4, assign13960_e13408_d_n6, assign13960_e13408_d_n7, assign13960_e13408_d_n8, assign13960_e13408_d_n9,) = {
    if (locals.var_guard575 != 0.0) {
        let assign13960_e13404: f64 = (locals.var_q_temp2).ln();
        let assign13960_e13406: f64 = (assign13960_e13404 - locals.var_q_rac_qsq);
        (assign13960_e13406, ((locals.var_q_temp2_dn4 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn4), ((locals.var_q_temp2_dn6 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn6), ((locals.var_q_temp2_dn7 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn7), ((locals.var_q_temp2_dn8 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn8), ((locals.var_q_temp2_dn9 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn9),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign13960_e13408;
        locals.var_q_ln_term_dn4 = assign13960_e13408_d_n4;
        locals.var_q_ln_term_dn6 = assign13960_e13408_d_n6;
        locals.var_q_ln_term_dn7 = assign13960_e13408_d_n7;
        locals.var_q_ln_term_dn8 = assign13960_e13408_d_n8;
        locals.var_q_ln_term_dn9 = assign13960_e13408_d_n9;

        let assign13970_e13411: f64 = (-0.005);
        let assign13970_e13412: f64 = if locals.var_q_qsq < assign13970_e13411 { 1.0 } else { 0.0 };
        locals.var_guard576 = assign13970_e13412;

        let (assign13980_e13422, assign13980_e13422_d_n4, assign13980_e13422_d_n6, assign13980_e13422_d_n7, assign13980_e13422_d_n8, assign13980_e13422_d_n9,) = {
    if ((locals.var_guard575 == 0.0) && (locals.var_guard576 != 0.0)) {
        let assign13980_e13419: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign13980_e13420: f64 = (assign13980_e13419).sin();
        (assign13980_e13420, ((assign13980_e13419).cos() * (0.5 * locals.var_q_rac_qsq_dn4)), ((assign13980_e13419).cos() * (0.5 * locals.var_q_rac_qsq_dn6)), ((assign13980_e13419).cos() * (0.5 * locals.var_q_rac_qsq_dn7)), ((assign13980_e13419).cos() * (0.5 * locals.var_q_rac_qsq_dn8)), ((assign13980_e13419).cos() * (0.5 * locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign13980_e13422;
        locals.var_q_temp2_dn4 = assign13980_e13422_d_n4;
        locals.var_q_temp2_dn6 = assign13980_e13422_d_n6;
        locals.var_q_temp2_dn7 = assign13980_e13422_d_n7;
        locals.var_q_temp2_dn8 = assign13980_e13422_d_n8;
        locals.var_q_temp2_dn9 = assign13980_e13422_d_n9;

        let (assign13990_e13434, assign13990_e13434_d_n4, assign13990_e13434_d_n6, assign13990_e13434_d_n7, assign13990_e13434_d_n8, assign13990_e13434_d_n9,) = {
    if ((locals.var_guard575 == 0.0) && (locals.var_guard576 != 0.0)) {
        let assign13990_e13428: f64 = (-locals.var_q_qsq);
        let assign13990_e13431: f64 = (locals.var_q_temp2 * locals.var_q_temp2);
        let assign13990_e13432: f64 = (assign13990_e13428 / assign13990_e13431);
        (assign13990_e13432, ((((-locals.var_q_qsq_dn4) * assign13990_e13431) - (assign13990_e13428 * ((locals.var_q_temp2_dn4 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn4)))) / (assign13990_e13431 * assign13990_e13431)), ((((-locals.var_q_qsq_dn6) * assign13990_e13431) - (assign13990_e13428 * ((locals.var_q_temp2_dn6 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn6)))) / (assign13990_e13431 * assign13990_e13431)), ((((-locals.var_q_qsq_dn7) * assign13990_e13431) - (assign13990_e13428 * ((locals.var_q_temp2_dn7 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn7)))) / (assign13990_e13431 * assign13990_e13431)), ((((-locals.var_q_qsq_dn8) * assign13990_e13431) - (assign13990_e13428 * ((locals.var_q_temp2_dn8 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn8)))) / (assign13990_e13431 * assign13990_e13431)), ((((-locals.var_q_qsq_dn9) * assign13990_e13431) - (assign13990_e13428 * ((locals.var_q_temp2_dn9 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn9)))) / (assign13990_e13431 * assign13990_e13431)),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign13990_e13434;
        locals.var_q_sh_term_dn4 = assign13990_e13434_d_n4;
        locals.var_q_sh_term_dn6 = assign13990_e13434_d_n6;
        locals.var_q_sh_term_dn7 = assign13990_e13434_d_n7;
        locals.var_q_sh_term_dn8 = assign13990_e13434_d_n8;
        locals.var_q_sh_term_dn9 = assign13990_e13434_d_n9;

        let (assign14000_e13442, assign14000_e13442_d_n4, assign14000_e13442_d_n6, assign14000_e13442_d_n7, assign14000_e13442_d_n8, assign14000_e13442_d_n9,) = {
    if ((locals.var_guard575 == 0.0) && (locals.var_guard576 != 0.0)) {
        let assign14000_e13440: f64 = (locals.var_q_sh_term).ln();
        (assign14000_e13440, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign14000_e13442;
        locals.var_q_ln_term_dn4 = assign14000_e13442_d_n4;
        locals.var_q_ln_term_dn6 = assign14000_e13442_d_n6;
        locals.var_q_ln_term_dn7 = assign14000_e13442_d_n7;
        locals.var_q_ln_term_dn8 = assign14000_e13442_d_n8;
        locals.var_q_ln_term_dn9 = assign14000_e13442_d_n9;

        let (assign14010_e13466, assign14010_e13466_d_n4, assign14010_e13466_d_n6, assign14010_e13466_d_n7, assign14010_e13466_d_n8, assign14010_e13466_d_n9,) = {
    if ((locals.var_guard575 == 0.0) && (locals.var_guard576 == 0.0)) {
        let assign14010_e13451: f64 = (locals.var_q_qsq * 0.3333333333333);
        let assign14010_e13455: f64 = (0.05 * locals.var_q_qsq);
        let assign14010_e13459: f64 = (0.0396825396825397 * locals.var_q_qsq);
        let assign14010_e13460: f64 = (1.0 - assign14010_e13459);
        let assign14010_e13461: f64 = (assign14010_e13455 * assign14010_e13460);
        let assign14010_e13462: f64 = (1.0 - assign14010_e13461);
        let assign14010_e13463: f64 = (assign14010_e13451 * assign14010_e13462);
        let assign14010_e13464: f64 = (4.0 - assign14010_e13463);
        (assign14010_e13464, (-(((locals.var_q_qsq_dn4 * 0.3333333333333) * assign14010_e13462) + (assign14010_e13451 * (-(((0.05 * locals.var_q_qsq_dn4) * assign14010_e13460) + (assign14010_e13455 * (-(0.0396825396825397 * locals.var_q_qsq_dn4)))))))), (-(((locals.var_q_qsq_dn6 * 0.3333333333333) * assign14010_e13462) + (assign14010_e13451 * (-(((0.05 * locals.var_q_qsq_dn6) * assign14010_e13460) + (assign14010_e13455 * (-(0.0396825396825397 * locals.var_q_qsq_dn6)))))))), (-(((locals.var_q_qsq_dn7 * 0.3333333333333) * assign14010_e13462) + (assign14010_e13451 * (-(((0.05 * locals.var_q_qsq_dn7) * assign14010_e13460) + (assign14010_e13455 * (-(0.0396825396825397 * locals.var_q_qsq_dn7)))))))), (-(((locals.var_q_qsq_dn8 * 0.3333333333333) * assign14010_e13462) + (assign14010_e13451 * (-(((0.05 * locals.var_q_qsq_dn8) * assign14010_e13460) + (assign14010_e13455 * (-(0.0396825396825397 * locals.var_q_qsq_dn8)))))))), (-(((locals.var_q_qsq_dn9 * 0.3333333333333) * assign14010_e13462) + (assign14010_e13451 * (-(((0.05 * locals.var_q_qsq_dn9) * assign14010_e13460) + (assign14010_e13455 * (-(0.0396825396825397 * locals.var_q_qsq_dn9)))))))),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign14010_e13466;
        locals.var_q_sh_term_dn4 = assign14010_e13466_d_n4;
        locals.var_q_sh_term_dn6 = assign14010_e13466_d_n6;
        locals.var_q_sh_term_dn7 = assign14010_e13466_d_n7;
        locals.var_q_sh_term_dn8 = assign14010_e13466_d_n8;
        locals.var_q_sh_term_dn9 = assign14010_e13466_d_n9;

        let (assign14020_e13475, assign14020_e13475_d_n4, assign14020_e13475_d_n6, assign14020_e13475_d_n7, assign14020_e13475_d_n8, assign14020_e13475_d_n9,) = {
    if ((locals.var_guard575 == 0.0) && (locals.var_guard576 == 0.0)) {
        let assign14020_e13473: f64 = (locals.var_q_sh_term).ln();
        (assign14020_e13473, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign14020_e13475;
        locals.var_q_ln_term_dn4 = assign14020_e13475_d_n4;
        locals.var_q_ln_term_dn6 = assign14020_e13475_d_n6;
        locals.var_q_ln_term_dn7 = assign14020_e13475_d_n7;
        locals.var_q_ln_term_dn8 = assign14020_e13475_d_n8;
        locals.var_q_ln_term_dn9 = assign14020_e13475_d_n9;

        let assign14030_e13478: f64 = (1.01 * locals.var_q_k1q1);
        let assign14030_e13480: f64 = (assign14030_e13478 + locals.var_q_qcoth);
        let assign14030_e13482: f64 = if assign14030_e13480 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard577 = assign14030_e13482;

        let (assign14040_e13488, assign14040_e13488_d_n4, assign14040_e13488_d_n6, assign14040_e13488_d_n7, assign14040_e13488_d_n8, assign14040_e13488_d_n9,) = {
    if (locals.var_guard577 != 0.0) {
        let assign14040_e13486: f64 = (locals.var_q_k1q1 + locals.var_q_qcoth);
        (assign14040_e13486, (locals.var_q_k1q1_dn4 + locals.var_q_qcoth_dn4), (locals.var_q_k1q1_dn6 + locals.var_q_qcoth_dn6), (locals.var_q_k1q1_dn7 + locals.var_q_qcoth_dn7), (locals.var_q_k1q1_dn8 + locals.var_q_qcoth_dn8), (locals.var_q_k1q1_dn9 + locals.var_q_qcoth_dn9),)
    } else {
        (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9,)
    }
};
        locals.var_q_expnum = assign14040_e13488;
        locals.var_q_expnum_dn4 = assign14040_e13488_d_n4;
        locals.var_q_expnum_dn6 = assign14040_e13488_d_n6;
        locals.var_q_expnum_dn7 = assign14040_e13488_d_n7;
        locals.var_q_expnum_dn8 = assign14040_e13488_d_n8;
        locals.var_q_expnum_dn9 = assign14040_e13488_d_n9;

        let (assign14050_e13494, assign14050_e13494_d_n4, assign14050_e13494_d_n6, assign14050_e13494_d_n7, assign14050_e13494_d_n8, assign14050_e13494_d_n9,) = {
    if (locals.var_guard577 != 0.0) {
        let assign14050_e13492: f64 = (locals.var_k1 + locals.var_q_d1_qcoth);
        (assign14050_e13492, (locals.var_k1_dn4 + locals.var_q_d1_qcoth_dn4), (locals.var_k1_dn6 + locals.var_q_d1_qcoth_dn6), (locals.var_k1_dn7 + locals.var_q_d1_qcoth_dn7), (locals.var_k1_dn8 + locals.var_q_d1_qcoth_dn8), (locals.var_k1_dn9 + locals.var_q_d1_qcoth_dn9),)
    } else {
        (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9,)
    }
};
        locals.var_q_d1_expnum = assign14050_e13494;
        locals.var_q_d1_expnum_dn4 = assign14050_e13494_d_n4;
        locals.var_q_d1_expnum_dn6 = assign14050_e13494_d_n6;
        locals.var_q_d1_expnum_dn7 = assign14050_e13494_d_n7;
        locals.var_q_d1_expnum_dn8 = assign14050_e13494_d_n8;
        locals.var_q_d1_expnum_dn9 = assign14050_e13494_d_n9;

        let (assign14060_e13498, assign14060_e13498_d_n4, assign14060_e13498_d_n6, assign14060_e13498_d_n7, assign14060_e13498_d_n8, assign14060_e13498_d_n9,) = {
    if (locals.var_guard577 != 0.0) {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    } else {
        (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9,)
    }
};
        locals.var_q_d2_expnum = assign14060_e13498;
        locals.var_q_d2_expnum_dn4 = assign14060_e13498_d_n4;
        locals.var_q_d2_expnum_dn6 = assign14060_e13498_d_n6;
        locals.var_q_d2_expnum_dn7 = assign14060_e13498_d_n7;
        locals.var_q_d2_expnum_dn8 = assign14060_e13498_d_n8;
        locals.var_q_d2_expnum_dn9 = assign14060_e13498_d_n9;

        let (assign14070_e13507, assign14070_e13507_d_n4, assign14070_e13507_d_n6, assign14070_e13507_d_n7, assign14070_e13507_d_n8, assign14070_e13507_d_n9,) = {
    if (locals.var_guard577 == 0.0) {
        let assign14070_e13504: f64 = (locals.var_q_k1q1 - locals.var_q_qcoth);
        let assign14070_e13505: f64 = (1.0 / assign14070_e13504);
        (assign14070_e13505, (-((locals.var_q_k1q1_dn4 - locals.var_q_qcoth_dn4) / (assign14070_e13504 * assign14070_e13504))), (-((locals.var_q_k1q1_dn6 - locals.var_q_qcoth_dn6) / (assign14070_e13504 * assign14070_e13504))), (-((locals.var_q_k1q1_dn7 - locals.var_q_qcoth_dn7) / (assign14070_e13504 * assign14070_e13504))), (-((locals.var_q_k1q1_dn8 - locals.var_q_qcoth_dn8) / (assign14070_e13504 * assign14070_e13504))), (-((locals.var_q_k1q1_dn9 - locals.var_q_qcoth_dn9) / (assign14070_e13504 * assign14070_e13504))),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign14070_e13507;
        locals.var_q_temp2_dn4 = assign14070_e13507_d_n4;
        locals.var_q_temp2_dn6 = assign14070_e13507_d_n6;
        locals.var_q_temp2_dn7 = assign14070_e13507_d_n7;
        locals.var_q_temp2_dn8 = assign14070_e13507_d_n8;
        locals.var_q_temp2_dn9 = assign14070_e13507_d_n9;

        let (assign14080_e13514, assign14080_e13514_d_n4, assign14080_e13514_d_n6, assign14080_e13514_d_n7, assign14080_e13514_d_n8, assign14080_e13514_d_n9,) = {
    if (locals.var_guard577 == 0.0) {
        let assign14080_e13512: f64 = (locals.var_q_d1_qcoth - locals.var_k1);
        (assign14080_e13512, (locals.var_q_d1_qcoth_dn4 - locals.var_k1_dn4), (locals.var_q_d1_qcoth_dn6 - locals.var_k1_dn6), (locals.var_q_d1_qcoth_dn7 - locals.var_k1_dn7), (locals.var_q_d1_qcoth_dn8 - locals.var_k1_dn8), (locals.var_q_d1_qcoth_dn9 - locals.var_k1_dn9),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign14080_e13514;
        locals.var_q_temp3_dn4 = assign14080_e13514_d_n4;
        locals.var_q_temp3_dn6 = assign14080_e13514_d_n6;
        locals.var_q_temp3_dn7 = assign14080_e13514_d_n7;
        locals.var_q_temp3_dn8 = assign14080_e13514_d_n8;
        locals.var_q_temp3_dn9 = assign14080_e13514_d_n9;

        let (assign14090_e13523, assign14090_e13523_d_n4, assign14090_e13523_d_n6, assign14090_e13523_d_n7, assign14090_e13523_d_n8, assign14090_e13523_d_n9,) = {
    if (locals.var_guard577 == 0.0) {
        let assign14090_e13519: f64 = (locals.var_q_aexp - locals.var_q_sh_term);
        let assign14090_e13521: f64 = (assign14090_e13519 * locals.var_q_temp2);
        (assign14090_e13521, (((locals.var_q_aexp_dn4 - locals.var_q_sh_term_dn4) * locals.var_q_temp2) + (assign14090_e13519 * locals.var_q_temp2_dn4)), (((locals.var_q_aexp_dn6 - locals.var_q_sh_term_dn6) * locals.var_q_temp2) + (assign14090_e13519 * locals.var_q_temp2_dn6)), (((locals.var_q_aexp_dn7 - locals.var_q_sh_term_dn7) * locals.var_q_temp2) + (assign14090_e13519 * locals.var_q_temp2_dn7)), (((locals.var_q_aexp_dn8 - locals.var_q_sh_term_dn8) * locals.var_q_temp2) + (assign14090_e13519 * locals.var_q_temp2_dn8)), (((locals.var_q_aexp_dn9 - locals.var_q_sh_term_dn9) * locals.var_q_temp2) + (assign14090_e13519 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9,)
    }
};
        locals.var_q_expnum = assign14090_e13523;
        locals.var_q_expnum_dn4 = assign14090_e13523_d_n4;
        locals.var_q_expnum_dn6 = assign14090_e13523_d_n6;
        locals.var_q_expnum_dn7 = assign14090_e13523_d_n7;
        locals.var_q_expnum_dn8 = assign14090_e13523_d_n8;
        locals.var_q_expnum_dn9 = assign14090_e13523_d_n9;

    }

    pub(super) fn stamp_transient_block_34(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14100_e13538, assign14100_e13538_d_n4, assign14100_e13538_d_n6, assign14100_e13538_d_n7, assign14100_e13538_d_n8, assign14100_e13538_d_n9,) = {
    if (locals.var_guard577 == 0.0) {
        let assign14100_e13528: f64 = (locals.var_q_temp3 * locals.var_q_expnum);
        let assign14100_e13530: f64 = (assign14100_e13528 - locals.var_q_aexp);
        let assign14100_e13533: f64 = (locals.var_q_d1_ln * locals.var_q_sh_term);
        let assign14100_e13534: f64 = (assign14100_e13530 - assign14100_e13533);
        let assign14100_e13536: f64 = (assign14100_e13534 * locals.var_q_temp2);
        (assign14100_e13536, ((((((locals.var_q_temp3_dn4 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4) - ((locals.var_q_d1_ln_dn4 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign14100_e13534 * locals.var_q_temp2_dn4)), ((((((locals.var_q_temp3_dn6 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6) - ((locals.var_q_d1_ln_dn6 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign14100_e13534 * locals.var_q_temp2_dn6)), ((((((locals.var_q_temp3_dn7 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7) - ((locals.var_q_d1_ln_dn7 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign14100_e13534 * locals.var_q_temp2_dn7)), ((((((locals.var_q_temp3_dn8 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8) - ((locals.var_q_d1_ln_dn8 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign14100_e13534 * locals.var_q_temp2_dn8)), ((((((locals.var_q_temp3_dn9 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9) - ((locals.var_q_d1_ln_dn9 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign14100_e13534 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9,)
    }
};
        locals.var_q_d1_expnum = assign14100_e13538;
        locals.var_q_d1_expnum_dn4 = assign14100_e13538_d_n4;
        locals.var_q_d1_expnum_dn6 = assign14100_e13538_d_n6;
        locals.var_q_d1_expnum_dn7 = assign14100_e13538_d_n7;
        locals.var_q_d1_expnum_dn8 = assign14100_e13538_d_n8;
        locals.var_q_d1_expnum_dn9 = assign14100_e13538_d_n9;

        let (assign14110_e13563, assign14110_e13563_d_n4, assign14110_e13563_d_n6, assign14110_e13563_d_n7, assign14110_e13563_d_n8, assign14110_e13563_d_n9,) = {
    if (locals.var_guard577 == 0.0) {
        let assign14110_e13543: f64 = (locals.var_q_d2_qcoth * locals.var_q_expnum);
        let assign14110_e13546: f64 = (2.0 * locals.var_q_temp3);
        let assign14110_e13548: f64 = (assign14110_e13546 * locals.var_q_d1_expnum);
        let assign14110_e13549: f64 = (assign14110_e13543 + assign14110_e13548);
        let assign14110_e13551: f64 = (assign14110_e13549 + locals.var_q_aexp);
        let assign14110_e13555: f64 = (locals.var_q_d1_ln * locals.var_q_d1_ln);
        let assign14110_e13556: f64 = (locals.var_q_d2_ln + assign14110_e13555);
        let assign14110_e13558: f64 = (assign14110_e13556 * locals.var_q_sh_term);
        let assign14110_e13559: f64 = (assign14110_e13551 - assign14110_e13558);
        let assign14110_e13561: f64 = (assign14110_e13559 * locals.var_q_temp2);
        (assign14110_e13561, (((((((locals.var_q_d2_qcoth_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_temp3_dn4) * locals.var_q_d1_expnum) + (assign14110_e13546 * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4) - (((locals.var_q_d2_ln_dn4 + ((locals.var_q_d1_ln_dn4 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn4))) * locals.var_q_sh_term) + (assign14110_e13556 * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign14110_e13559 * locals.var_q_temp2_dn4)), (((((((locals.var_q_d2_qcoth_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_temp3_dn6) * locals.var_q_d1_expnum) + (assign14110_e13546 * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6) - (((locals.var_q_d2_ln_dn6 + ((locals.var_q_d1_ln_dn6 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn6))) * locals.var_q_sh_term) + (assign14110_e13556 * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign14110_e13559 * locals.var_q_temp2_dn6)), (((((((locals.var_q_d2_qcoth_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_temp3_dn7) * locals.var_q_d1_expnum) + (assign14110_e13546 * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7) - (((locals.var_q_d2_ln_dn7 + ((locals.var_q_d1_ln_dn7 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn7))) * locals.var_q_sh_term) + (assign14110_e13556 * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign14110_e13559 * locals.var_q_temp2_dn7)), (((((((locals.var_q_d2_qcoth_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_temp3_dn8) * locals.var_q_d1_expnum) + (assign14110_e13546 * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8) - (((locals.var_q_d2_ln_dn8 + ((locals.var_q_d1_ln_dn8 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn8))) * locals.var_q_sh_term) + (assign14110_e13556 * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign14110_e13559 * locals.var_q_temp2_dn8)), (((((((locals.var_q_d2_qcoth_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_temp3_dn9) * locals.var_q_d1_expnum) + (assign14110_e13546 * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9) - (((locals.var_q_d2_ln_dn9 + ((locals.var_q_d1_ln_dn9 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn9))) * locals.var_q_sh_term) + (assign14110_e13556 * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign14110_e13559 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9,)
    }
};
        locals.var_q_d2_expnum = assign14110_e13563;
        locals.var_q_d2_expnum_dn4 = assign14110_e13563_d_n4;
        locals.var_q_d2_expnum_dn6 = assign14110_e13563_d_n6;
        locals.var_q_d2_expnum_dn7 = assign14110_e13563_d_n7;
        locals.var_q_d2_expnum_dn8 = assign14110_e13563_d_n8;
        locals.var_q_d2_expnum_dn9 = assign14110_e13563_d_n9;

        let assign14120_e13566: f64 = if locals.var_q_expnum > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard578 = assign14120_e13566;

        let (assign14130_e13571, assign14130_e13571_d_n4, assign14130_e13571_d_n6, assign14130_e13571_d_n7, assign14130_e13571_d_n8, assign14130_e13571_d_n9,) = {
    if (locals.var_guard578 != 0.0) {
        let assign14130_e13569: f64 = (locals.var_q_expnum).ln();
        (assign14130_e13569, (locals.var_q_expnum_dn4 / locals.var_q_expnum), (locals.var_q_expnum_dn6 / locals.var_q_expnum), (locals.var_q_expnum_dn7 / locals.var_q_expnum), (locals.var_q_expnum_dn8 / locals.var_q_expnum), (locals.var_q_expnum_dn9 / locals.var_q_expnum),)
    } else {
        (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9,)
    }
};
        locals.var_q_lnexpnum = assign14130_e13571;
        locals.var_q_lnexpnum_dn4 = assign14130_e13571_d_n4;
        locals.var_q_lnexpnum_dn6 = assign14130_e13571_d_n6;
        locals.var_q_lnexpnum_dn7 = assign14130_e13571_d_n7;
        locals.var_q_lnexpnum_dn8 = assign14130_e13571_d_n8;
        locals.var_q_lnexpnum_dn9 = assign14130_e13571_d_n9;

        let (assign14140_e13577, assign14140_e13577_d_n4, assign14140_e13577_d_n6, assign14140_e13577_d_n7, assign14140_e13577_d_n8, assign14140_e13577_d_n9,) = {
    if (locals.var_guard578 != 0.0) {
        let assign14140_e13575: f64 = (1.0 / locals.var_q_expnum);
        (assign14140_e13575, (-(locals.var_q_expnum_dn4 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn6 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn7 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn8 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn9 / (locals.var_q_expnum * locals.var_q_expnum))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign14140_e13577;
        locals.var_q_temp1_dn4 = assign14140_e13577_d_n4;
        locals.var_q_temp1_dn6 = assign14140_e13577_d_n6;
        locals.var_q_temp1_dn7 = assign14140_e13577_d_n7;
        locals.var_q_temp1_dn8 = assign14140_e13577_d_n8;
        locals.var_q_temp1_dn9 = assign14140_e13577_d_n9;

        let (assign14150_e13583, assign14150_e13583_d_n4, assign14150_e13583_d_n6, assign14150_e13583_d_n7, assign14150_e13583_d_n8, assign14150_e13583_d_n9,) = {
    if (locals.var_guard578 != 0.0) {
        let assign14150_e13581: f64 = (locals.var_q_d1_expnum * locals.var_q_temp1);
        (assign14150_e13581, ((locals.var_q_d1_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn4)), ((locals.var_q_d1_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn6)), ((locals.var_q_d1_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn7)), ((locals.var_q_d1_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn8)), ((locals.var_q_d1_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9,)
    }
};
        locals.var_q_d1_lnexpnum = assign14150_e13583;
        locals.var_q_d1_lnexpnum_dn4 = assign14150_e13583_d_n4;
        locals.var_q_d1_lnexpnum_dn6 = assign14150_e13583_d_n6;
        locals.var_q_d1_lnexpnum_dn7 = assign14150_e13583_d_n7;
        locals.var_q_d1_lnexpnum_dn8 = assign14150_e13583_d_n8;
        locals.var_q_d1_lnexpnum_dn9 = assign14150_e13583_d_n9;

        let (assign14160_e13593, assign14160_e13593_d_n4, assign14160_e13593_d_n6, assign14160_e13593_d_n7, assign14160_e13593_d_n8, assign14160_e13593_d_n9,) = {
    if (locals.var_guard578 != 0.0) {
        let assign14160_e13587: f64 = (locals.var_q_d2_expnum * locals.var_q_temp1);
        let assign14160_e13590: f64 = (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum);
        let assign14160_e13591: f64 = (assign14160_e13587 - assign14160_e13590);
        (assign14160_e13591, (((locals.var_q_d2_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn4)) - ((locals.var_q_d1_lnexpnum_dn4 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn4))), (((locals.var_q_d2_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn6)) - ((locals.var_q_d1_lnexpnum_dn6 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn6))), (((locals.var_q_d2_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn7)) - ((locals.var_q_d1_lnexpnum_dn7 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn7))), (((locals.var_q_d2_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn8)) - ((locals.var_q_d1_lnexpnum_dn8 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn8))), (((locals.var_q_d2_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn9)) - ((locals.var_q_d1_lnexpnum_dn9 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn9))),)
    } else {
        (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9,)
    }
};
        locals.var_q_d2_lnexpnum = assign14160_e13593;
        locals.var_q_d2_lnexpnum_dn4 = assign14160_e13593_d_n4;
        locals.var_q_d2_lnexpnum_dn6 = assign14160_e13593_d_n6;
        locals.var_q_d2_lnexpnum_dn7 = assign14160_e13593_d_n7;
        locals.var_q_d2_lnexpnum_dn8 = assign14160_e13593_d_n8;
        locals.var_q_d2_lnexpnum_dn9 = assign14160_e13593_d_n9;

        let (assign14170_e13604, assign14170_e13604_d_n4, assign14170_e13604_d_n6, assign14170_e13604_d_n7, assign14170_e13604_d_n8, assign14170_e13604_d_n9,) = {
    if (locals.var_guard578 == 0.0) {
        let assign14170_e13598: f64 = (locals.var_q_k1q1 + 0.6931471805599);
        let assign14170_e13600: f64 = (-locals.var_q_k1q1);
        let assign14170_e13601: f64 = (assign14170_e13600).ln();
        let assign14170_e13602: f64 = (assign14170_e13598 + assign14170_e13601);
        (assign14170_e13602, (locals.var_q_k1q1_dn4 + ((-locals.var_q_k1q1_dn4) / assign14170_e13600)), (locals.var_q_k1q1_dn6 + ((-locals.var_q_k1q1_dn6) / assign14170_e13600)), (locals.var_q_k1q1_dn7 + ((-locals.var_q_k1q1_dn7) / assign14170_e13600)), (locals.var_q_k1q1_dn8 + ((-locals.var_q_k1q1_dn8) / assign14170_e13600)), (locals.var_q_k1q1_dn9 + ((-locals.var_q_k1q1_dn9) / assign14170_e13600)),)
    } else {
        (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9,)
    }
};
        locals.var_q_lnexpnum = assign14170_e13604;
        locals.var_q_lnexpnum_dn4 = assign14170_e13604_d_n4;
        locals.var_q_lnexpnum_dn6 = assign14170_e13604_d_n6;
        locals.var_q_lnexpnum_dn7 = assign14170_e13604_d_n7;
        locals.var_q_lnexpnum_dn8 = assign14170_e13604_d_n8;
        locals.var_q_lnexpnum_dn9 = assign14170_e13604_d_n9;

        let (assign14180_e13611, assign14180_e13611_d_n4, assign14180_e13611_d_n6, assign14180_e13611_d_n7, assign14180_e13611_d_n8, assign14180_e13611_d_n9,) = {
    if (locals.var_guard578 == 0.0) {
        let assign14180_e13609: f64 = (1.0 / locals.var_q1s);
        (assign14180_e13609, (-(locals.var_q1s_dn4 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn6 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn7 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn8 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn9 / (locals.var_q1s * locals.var_q1s))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign14180_e13611;
        locals.var_q_temp1_dn4 = assign14180_e13611_d_n4;
        locals.var_q_temp1_dn6 = assign14180_e13611_d_n6;
        locals.var_q_temp1_dn7 = assign14180_e13611_d_n7;
        locals.var_q_temp1_dn8 = assign14180_e13611_d_n8;
        locals.var_q_temp1_dn9 = assign14180_e13611_d_n9;

        let (assign14190_e13618, assign14190_e13618_d_n4, assign14190_e13618_d_n6, assign14190_e13618_d_n7, assign14190_e13618_d_n8, assign14190_e13618_d_n9,) = {
    if (locals.var_guard578 == 0.0) {
        let assign14190_e13616: f64 = (locals.var_k1 + locals.var_q_temp1);
        (assign14190_e13616, (locals.var_k1_dn4 + locals.var_q_temp1_dn4), (locals.var_k1_dn6 + locals.var_q_temp1_dn6), (locals.var_k1_dn7 + locals.var_q_temp1_dn7), (locals.var_k1_dn8 + locals.var_q_temp1_dn8), (locals.var_k1_dn9 + locals.var_q_temp1_dn9),)
    } else {
        (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9,)
    }
};
        locals.var_q_d1_lnexpnum = assign14190_e13618;
        locals.var_q_d1_lnexpnum_dn4 = assign14190_e13618_d_n4;
        locals.var_q_d1_lnexpnum_dn6 = assign14190_e13618_d_n6;
        locals.var_q_d1_lnexpnum_dn7 = assign14190_e13618_d_n7;
        locals.var_q_d1_lnexpnum_dn8 = assign14190_e13618_d_n8;
        locals.var_q_d1_lnexpnum_dn9 = assign14190_e13618_d_n9;

        let (assign14200_e13626, assign14200_e13626_d_n4, assign14200_e13626_d_n6, assign14200_e13626_d_n7, assign14200_e13626_d_n8, assign14200_e13626_d_n9,) = {
    if (locals.var_guard578 == 0.0) {
        let assign14200_e13622: f64 = (-locals.var_q_temp1);
        let assign14200_e13624: f64 = (assign14200_e13622 * locals.var_q_temp1);
        (assign14200_e13624, (((-locals.var_q_temp1_dn4) * locals.var_q_temp1) + (assign14200_e13622 * locals.var_q_temp1_dn4)), (((-locals.var_q_temp1_dn6) * locals.var_q_temp1) + (assign14200_e13622 * locals.var_q_temp1_dn6)), (((-locals.var_q_temp1_dn7) * locals.var_q_temp1) + (assign14200_e13622 * locals.var_q_temp1_dn7)), (((-locals.var_q_temp1_dn8) * locals.var_q_temp1) + (assign14200_e13622 * locals.var_q_temp1_dn8)), (((-locals.var_q_temp1_dn9) * locals.var_q_temp1) + (assign14200_e13622 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9,)
    }
};
        locals.var_q_d2_lnexpnum = assign14200_e13626;
        locals.var_q_d2_lnexpnum_dn4 = assign14200_e13626_d_n4;
        locals.var_q_d2_lnexpnum_dn6 = assign14200_e13626_d_n6;
        locals.var_q_d2_lnexpnum_dn7 = assign14200_e13626_d_n7;
        locals.var_q_d2_lnexpnum_dn8 = assign14200_e13626_d_n8;
        locals.var_q_d2_lnexpnum_dn9 = assign14200_e13626_d_n9;

        let assign14210_e13629: f64 = (locals.var_xg2x - locals.var_xg1x);
        let assign14210_e13631: f64 = (assign14210_e13629 + locals.var_q1s);
        let assign14210_e13634: f64 = (2.0 * locals.var_q_lnexpnum);
        let assign14210_e13635: f64 = (assign14210_e13631 + assign14210_e13634);
        let assign14210_e13637: f64 = (assign14210_e13635 - locals.var_q_ln_term);
        locals.var_q_q2_int = assign14210_e13637;
        locals.var_q_q2_int_dn4 = ((((locals.var_xg2x_dn4 - locals.var_xg1x_dn4) + locals.var_q1s_dn4) + (2.0 * locals.var_q_lnexpnum_dn4)) - locals.var_q_ln_term_dn4);
        locals.var_q_q2_int_dn6 = ((((locals.var_xg2x_dn6 - locals.var_xg1x_dn6) + locals.var_q1s_dn6) + (2.0 * locals.var_q_lnexpnum_dn6)) - locals.var_q_ln_term_dn6);
        locals.var_q_q2_int_dn7 = ((((locals.var_xg2x_dn7 - locals.var_xg1x_dn7) + locals.var_q1s_dn7) + (2.0 * locals.var_q_lnexpnum_dn7)) - locals.var_q_ln_term_dn7);
        locals.var_q_q2_int_dn8 = ((((locals.var_xg2x_dn8 - locals.var_xg1x_dn8) + locals.var_q1s_dn8) + (2.0 * locals.var_q_lnexpnum_dn8)) - locals.var_q_ln_term_dn8);
        locals.var_q_q2_int_dn9 = ((((locals.var_xg2x_dn9 - locals.var_xg1x_dn9) + locals.var_q1s_dn9) + (2.0 * locals.var_q_lnexpnum_dn9)) - locals.var_q_ln_term_dn9);

        let assign14220_e13641: f64 = (2.0 * locals.var_q_d1_lnexpnum);
        let assign14220_e13642: f64 = (1.0 + assign14220_e13641);
        let assign14220_e13644: f64 = (assign14220_e13642 - locals.var_q_d1_ln);
        locals.var_q_d1_q2 = assign14220_e13644;
        locals.var_q_d1_q2_dn4 = ((2.0 * locals.var_q_d1_lnexpnum_dn4) - locals.var_q_d1_ln_dn4);
        locals.var_q_d1_q2_dn6 = ((2.0 * locals.var_q_d1_lnexpnum_dn6) - locals.var_q_d1_ln_dn6);
        locals.var_q_d1_q2_dn7 = ((2.0 * locals.var_q_d1_lnexpnum_dn7) - locals.var_q_d1_ln_dn7);
        locals.var_q_d1_q2_dn8 = ((2.0 * locals.var_q_d1_lnexpnum_dn8) - locals.var_q_d1_ln_dn8);
        locals.var_q_d1_q2_dn9 = ((2.0 * locals.var_q_d1_lnexpnum_dn9) - locals.var_q_d1_ln_dn9);

        let assign14230_e13647: f64 = (2.0 * locals.var_q_d2_lnexpnum);
        let assign14230_e13649: f64 = (assign14230_e13647 - locals.var_q_d2_ln);
        locals.var_q_d2_q2 = assign14230_e13649;
        locals.var_q_d2_q2_dn4 = ((2.0 * locals.var_q_d2_lnexpnum_dn4) - locals.var_q_d2_ln_dn4);
        locals.var_q_d2_q2_dn6 = ((2.0 * locals.var_q_d2_lnexpnum_dn6) - locals.var_q_d2_ln_dn6);
        locals.var_q_d2_q2_dn7 = ((2.0 * locals.var_q_d2_lnexpnum_dn7) - locals.var_q_d2_ln_dn7);
        locals.var_q_d2_q2_dn8 = ((2.0 * locals.var_q_d2_lnexpnum_dn8) - locals.var_q_d2_ln_dn8);
        locals.var_q_d2_q2_dn9 = ((2.0 * locals.var_q_d2_lnexpnum_dn9) - locals.var_q_d2_ln_dn9);

        let assign14240_e13653: f64 = (locals.var_k2 * locals.var_q_q2_int);
        let assign14240_e13654: f64 = (locals.var_q_k1q1 + assign14240_e13653);
        locals.var_q_qi_int = assign14240_e13654;
        locals.var_q_qi_int_dn4 = (locals.var_q_k1q1_dn4 + ((locals.var_k2_dn4 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn4)));
        locals.var_q_qi_int_dn6 = (locals.var_q_k1q1_dn6 + ((locals.var_k2_dn6 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn6)));
        locals.var_q_qi_int_dn7 = (locals.var_q_k1q1_dn7 + ((locals.var_k2_dn7 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn7)));
        locals.var_q_qi_int_dn8 = (locals.var_q_k1q1_dn8 + ((locals.var_k2_dn8 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn8)));
        locals.var_q_qi_int_dn9 = (locals.var_q_k1q1_dn9 + ((locals.var_k2_dn9 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn9)));

        let assign14250_e13658: f64 = (locals.var_k2 * locals.var_q_d1_q2);
        let assign14250_e13659: f64 = (locals.var_k1 + assign14250_e13658);
        locals.var_q_d1_qi = assign14250_e13659;
        locals.var_q_d1_qi_dn4 = (locals.var_k1_dn4 + ((locals.var_k2_dn4 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn4)));
        locals.var_q_d1_qi_dn6 = (locals.var_k1_dn6 + ((locals.var_k2_dn6 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn6)));
        locals.var_q_d1_qi_dn7 = (locals.var_k1_dn7 + ((locals.var_k2_dn7 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn7)));
        locals.var_q_d1_qi_dn8 = (locals.var_k1_dn8 + ((locals.var_k2_dn8 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn8)));
        locals.var_q_d1_qi_dn9 = (locals.var_k1_dn9 + ((locals.var_k2_dn9 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn9)));

        let assign14260_e13662: f64 = (locals.var_k2 * locals.var_q_d2_q2);
        locals.var_q_d2_qi = assign14260_e13662;
        locals.var_q_d2_qi_dn4 = ((locals.var_k2_dn4 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn4));
        locals.var_q_d2_qi_dn6 = ((locals.var_k2_dn6 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn6));
        locals.var_q_d2_qi_dn7 = ((locals.var_k2_dn7 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn7));
        locals.var_q_d2_qi_dn8 = ((locals.var_k2_dn8 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn8));
        locals.var_q_d2_qi_dn9 = ((locals.var_k2_dn9 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn9));

        let assign14270_e13665: f64 = (locals.var_q_qi_int * locals.var_q_expnum);
        let assign14270_e13667: f64 = (assign14270_e13665 - locals.var_q_aexp);
        locals.var_q_zero = assign14270_e13667;
        locals.var_q_zero_dn4 = (((locals.var_q_qi_int_dn4 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4);
        locals.var_q_zero_dn6 = (((locals.var_q_qi_int_dn6 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6);
        locals.var_q_zero_dn7 = (((locals.var_q_qi_int_dn7 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7);
        locals.var_q_zero_dn8 = (((locals.var_q_qi_int_dn8 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8);
        locals.var_q_zero_dn9 = (((locals.var_q_qi_int_dn9 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9);

        let assign14280_e13670: f64 = (locals.var_q_d1_qi * locals.var_q_expnum);
        let assign14280_e13673: f64 = (locals.var_q_qi_int * locals.var_q_d1_expnum);
        let assign14280_e13674: f64 = (assign14280_e13670 + assign14280_e13673);
        let assign14280_e13676: f64 = (assign14280_e13674 + locals.var_q_aexp);
        locals.var_q_d1_zero = assign14280_e13676;
        locals.var_q_d1_zero_dn4 = ((((locals.var_q_d1_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn4)) + ((locals.var_q_qi_int_dn4 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4);
        locals.var_q_d1_zero_dn6 = ((((locals.var_q_d1_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn6)) + ((locals.var_q_qi_int_dn6 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6);
        locals.var_q_d1_zero_dn7 = ((((locals.var_q_d1_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn7)) + ((locals.var_q_qi_int_dn7 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7);
        locals.var_q_d1_zero_dn8 = ((((locals.var_q_d1_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn8)) + ((locals.var_q_qi_int_dn8 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8);
        locals.var_q_d1_zero_dn9 = ((((locals.var_q_d1_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn9)) + ((locals.var_q_qi_int_dn9 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9);

        let assign14290_e13679: f64 = (locals.var_q_d2_qi * locals.var_q_expnum);
        let assign14290_e13682: f64 = (2.0 * locals.var_q_d1_qi);
        let assign14290_e13684: f64 = (assign14290_e13682 * locals.var_q_d1_expnum);
        let assign14290_e13685: f64 = (assign14290_e13679 + assign14290_e13684);
        let assign14290_e13688: f64 = (locals.var_q_qi_int * locals.var_q_d2_expnum);
        let assign14290_e13689: f64 = (assign14290_e13685 + assign14290_e13688);
        let assign14290_e13691: f64 = (assign14290_e13689 - locals.var_q_aexp);
        locals.var_q_d2_zero = assign14290_e13691;
        locals.var_q_d2_zero_dn4 = (((((locals.var_q_d2_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_d1_qi_dn4) * locals.var_q_d1_expnum) + (assign14290_e13682 * locals.var_q_d1_expnum_dn4))) + ((locals.var_q_qi_int_dn4 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn4))) - locals.var_q_aexp_dn4);
        locals.var_q_d2_zero_dn6 = (((((locals.var_q_d2_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_d1_qi_dn6) * locals.var_q_d1_expnum) + (assign14290_e13682 * locals.var_q_d1_expnum_dn6))) + ((locals.var_q_qi_int_dn6 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn6))) - locals.var_q_aexp_dn6);
        locals.var_q_d2_zero_dn7 = (((((locals.var_q_d2_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_d1_qi_dn7) * locals.var_q_d1_expnum) + (assign14290_e13682 * locals.var_q_d1_expnum_dn7))) + ((locals.var_q_qi_int_dn7 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn7))) - locals.var_q_aexp_dn7);
        locals.var_q_d2_zero_dn8 = (((((locals.var_q_d2_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_d1_qi_dn8) * locals.var_q_d1_expnum) + (assign14290_e13682 * locals.var_q_d1_expnum_dn8))) + ((locals.var_q_qi_int_dn8 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn8))) - locals.var_q_aexp_dn8);
        locals.var_q_d2_zero_dn9 = (((((locals.var_q_d2_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_d1_qi_dn9) * locals.var_q_d1_expnum) + (assign14290_e13682 * locals.var_q_d1_expnum_dn9))) + ((locals.var_q_qi_int_dn9 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn9))) - locals.var_q_aexp_dn9);

        let assign14300_e13694: f64 = (locals.var_q_d1_zero * locals.var_q_d1_zero);
        let assign14300_e13697: f64 = (0.5 * locals.var_q_zero);
        let assign14300_e13699: f64 = (assign14300_e13697 * locals.var_q_d2_zero);
        let assign14300_e13700: f64 = (assign14300_e13694 - assign14300_e13699);
        locals.var_q_temp = assign14300_e13700;
        locals.var_q_temp_dn4 = (((locals.var_q_d1_zero_dn4 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn4)) - (((0.5 * locals.var_q_zero_dn4) * locals.var_q_d2_zero) + (assign14300_e13697 * locals.var_q_d2_zero_dn4)));
        locals.var_q_temp_dn6 = (((locals.var_q_d1_zero_dn6 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn6)) - (((0.5 * locals.var_q_zero_dn6) * locals.var_q_d2_zero) + (assign14300_e13697 * locals.var_q_d2_zero_dn6)));
        locals.var_q_temp_dn7 = (((locals.var_q_d1_zero_dn7 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn7)) - (((0.5 * locals.var_q_zero_dn7) * locals.var_q_d2_zero) + (assign14300_e13697 * locals.var_q_d2_zero_dn7)));
        locals.var_q_temp_dn8 = (((locals.var_q_d1_zero_dn8 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn8)) - (((0.5 * locals.var_q_zero_dn8) * locals.var_q_d2_zero) + (assign14300_e13697 * locals.var_q_d2_zero_dn8)));
        locals.var_q_temp_dn9 = (((locals.var_q_d1_zero_dn9 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn9)) - (((0.5 * locals.var_q_zero_dn9) * locals.var_q_d2_zero) + (assign14300_e13697 * locals.var_q_d2_zero_dn9)));

        let assign14310_e13702: f64 = (-locals.var_q_zero);
        let assign14310_e13704: f64 = (assign14310_e13702 * locals.var_q_d1_zero);
        let assign14310_e13706: f64 = (assign14310_e13704 * locals.var_q_temp);
        let assign14310_e13709: f64 = (locals.var_q_temp * locals.var_q_temp);
        let assign14310_e13711: f64 = (assign14310_e13709 + 1e-200);
        let assign14310_e13712: f64 = (assign14310_e13706 / assign14310_e13711);
        locals.var_q_eps2 = assign14310_e13712;
        locals.var_q_eps2_dn4 = ((((((((-locals.var_q_zero_dn4) * locals.var_q_d1_zero) + (assign14310_e13702 * locals.var_q_d1_zero_dn4)) * locals.var_q_temp) + (assign14310_e13704 * locals.var_q_temp_dn4)) * assign14310_e13711) - (assign14310_e13706 * ((locals.var_q_temp_dn4 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn4)))) / (assign14310_e13711 * assign14310_e13711));
        locals.var_q_eps2_dn6 = ((((((((-locals.var_q_zero_dn6) * locals.var_q_d1_zero) + (assign14310_e13702 * locals.var_q_d1_zero_dn6)) * locals.var_q_temp) + (assign14310_e13704 * locals.var_q_temp_dn6)) * assign14310_e13711) - (assign14310_e13706 * ((locals.var_q_temp_dn6 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn6)))) / (assign14310_e13711 * assign14310_e13711));
        locals.var_q_eps2_dn7 = ((((((((-locals.var_q_zero_dn7) * locals.var_q_d1_zero) + (assign14310_e13702 * locals.var_q_d1_zero_dn7)) * locals.var_q_temp) + (assign14310_e13704 * locals.var_q_temp_dn7)) * assign14310_e13711) - (assign14310_e13706 * ((locals.var_q_temp_dn7 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn7)))) / (assign14310_e13711 * assign14310_e13711));
        locals.var_q_eps2_dn8 = ((((((((-locals.var_q_zero_dn8) * locals.var_q_d1_zero) + (assign14310_e13702 * locals.var_q_d1_zero_dn8)) * locals.var_q_temp) + (assign14310_e13704 * locals.var_q_temp_dn8)) * assign14310_e13711) - (assign14310_e13706 * ((locals.var_q_temp_dn8 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn8)))) / (assign14310_e13711 * assign14310_e13711));
        locals.var_q_eps2_dn9 = ((((((((-locals.var_q_zero_dn9) * locals.var_q_d1_zero) + (assign14310_e13702 * locals.var_q_d1_zero_dn9)) * locals.var_q_temp) + (assign14310_e13704 * locals.var_q_temp_dn9)) * assign14310_e13711) - (assign14310_e13706 * ((locals.var_q_temp_dn9 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn9)))) / (assign14310_e13711 * assign14310_e13711));

        let assign14320_e13715: f64 = (locals.var_q1s + locals.var_q_eps2);
        locals.var_q1s = assign14320_e13715;
        locals.var_q1s_dn4 = (locals.var_q1s_dn4 + locals.var_q_eps2_dn4);
        locals.var_q1s_dn6 = (locals.var_q1s_dn6 + locals.var_q_eps2_dn6);
        locals.var_q1s_dn7 = (locals.var_q1s_dn7 + locals.var_q_eps2_dn7);
        locals.var_q1s_dn8 = (locals.var_q1s_dn8 + locals.var_q_eps2_dn8);
        locals.var_q1s_dn9 = (locals.var_q1s_dn9 + locals.var_q_eps2_dn9);

        let assign14330_e13718: f64 = if p.p10 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard579 = assign14330_e13718;

        let assign14340_e13720: f64 = (locals.var_q_eps2).abs();
        let assign14340_e13722: f64 = if assign14340_e13720 > 0.01 { 1.0 } else { 0.0 };
        locals.var_guard580 = assign14340_e13722;

        let (assign14350_e13730, assign14350_e13730_d_n4, assign14350_e13730_d_n6, assign14350_e13730_d_n7, assign14350_e13730_d_n8, assign14350_e13730_d_n9,) = {
    if ((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) {
        let assign14350_e13728: f64 = (locals.var_k1 * locals.var_q1s);
        (assign14350_e13728, ((locals.var_k1_dn4 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn4)), ((locals.var_k1_dn6 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn6)), ((locals.var_k1_dn7 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn7)), ((locals.var_k1_dn8 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn8)), ((locals.var_k1_dn9 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn9)),)
    } else {
        (locals.var_q_k1q1, locals.var_q_k1q1_dn4, locals.var_q_k1q1_dn6, locals.var_q_k1q1_dn7, locals.var_q_k1q1_dn8, locals.var_q_k1q1_dn9,)
    }
};
        locals.var_q_k1q1 = assign14350_e13730;
        locals.var_q_k1q1_dn4 = assign14350_e13730_d_n4;
        locals.var_q_k1q1_dn6 = assign14350_e13730_d_n6;
        locals.var_q_k1q1_dn7 = assign14350_e13730_d_n7;
        locals.var_q_k1q1_dn8 = assign14350_e13730_d_n8;
        locals.var_q_k1q1_dn9 = assign14350_e13730_d_n9;

        let assign14360_e13733: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign14360_e13735: f64 = assign14360_e13733;
        let assign14360_e13737: f64 = if assign14360_e13735 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard581 = assign14360_e13737;

        let (assign14370_e13750, assign14370_e13750_d_n4, assign14370_e13750_d_n6, assign14370_e13750_d_n7, assign14370_e13750_d_n8, assign14370_e13750_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard581 != 0.0)) {
        let assign14370_e13745: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign14370_e13747: f64 = assign14370_e13745;
        let assign14370_e13748: f64 = (assign14370_e13747).exp();
        (assign14370_e13748, (assign14370_e13748 * (locals.var_xg1x_dn4 - locals.var_q1s_dn4)), (assign14370_e13748 * (locals.var_xg1x_dn6 - locals.var_q1s_dn6)), (assign14370_e13748 * (locals.var_xg1x_dn7 - locals.var_q1s_dn7)), (assign14370_e13748 * (locals.var_xg1x_dn8 - locals.var_q1s_dn8)), (assign14370_e13748 * (locals.var_xg1x_dn9 - locals.var_q1s_dn9)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign14370_e13750;
        locals.var_q_temp1_dn4 = assign14370_e13750_d_n4;
        locals.var_q_temp1_dn6 = assign14370_e13750_d_n6;
        locals.var_q_temp1_dn7 = assign14370_e13750_d_n7;
        locals.var_q_temp1_dn8 = assign14370_e13750_d_n8;
        locals.var_q_temp1_dn9 = assign14370_e13750_d_n9;

        let (assign14380_e13793, assign14380_e13793_d_n4, assign14380_e13793_d_n6, assign14380_e13793_d_n7, assign14380_e13793_d_n8, assign14380_e13793_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard581 == 0.0)) {
        let assign14380_e13761: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign14380_e13763: f64 = assign14380_e13761;
        let assign14380_e13765: f64 = (assign14380_e13763 - 80.0);
        let assign14380_e13770: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign14380_e13772: f64 = assign14380_e13770;
        let assign14380_e13774: f64 = (assign14380_e13772 - 80.0);
        let assign14380_e13775: f64 = (0.5 * assign14380_e13774);
        let assign14380_e13779: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign14380_e13781: f64 = assign14380_e13779;
        let assign14380_e13783: f64 = (assign14380_e13781 - 80.0);
        let assign14380_e13785: f64 = (assign14380_e13783 * 0.3333333333333);
        let assign14380_e13786: f64 = (1.0 + assign14380_e13785);
        let assign14380_e13787: f64 = (assign14380_e13775 * assign14380_e13786);
        let assign14380_e13788: f64 = (1.0 + assign14380_e13787);
        let assign14380_e13789: f64 = (assign14380_e13765 * assign14380_e13788);
        let assign14380_e13790: f64 = (1.0 + assign14380_e13789);
        let assign14380_e13791: f64 = (5.54062e34 * assign14380_e13790);
        (assign14380_e13791, (5.54062e34 * (((locals.var_xg1x_dn4 - locals.var_q1s_dn4) * assign14380_e13788) + (assign14380_e13765 * (((0.5 * (locals.var_xg1x_dn4 - locals.var_q1s_dn4)) * assign14380_e13786) + (assign14380_e13775 * ((locals.var_xg1x_dn4 - locals.var_q1s_dn4) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn6 - locals.var_q1s_dn6) * assign14380_e13788) + (assign14380_e13765 * (((0.5 * (locals.var_xg1x_dn6 - locals.var_q1s_dn6)) * assign14380_e13786) + (assign14380_e13775 * ((locals.var_xg1x_dn6 - locals.var_q1s_dn6) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn7 - locals.var_q1s_dn7) * assign14380_e13788) + (assign14380_e13765 * (((0.5 * (locals.var_xg1x_dn7 - locals.var_q1s_dn7)) * assign14380_e13786) + (assign14380_e13775 * ((locals.var_xg1x_dn7 - locals.var_q1s_dn7) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn8 - locals.var_q1s_dn8) * assign14380_e13788) + (assign14380_e13765 * (((0.5 * (locals.var_xg1x_dn8 - locals.var_q1s_dn8)) * assign14380_e13786) + (assign14380_e13775 * ((locals.var_xg1x_dn8 - locals.var_q1s_dn8) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn9 - locals.var_q1s_dn9) * assign14380_e13788) + (assign14380_e13765 * (((0.5 * (locals.var_xg1x_dn9 - locals.var_q1s_dn9)) * assign14380_e13786) + (assign14380_e13775 * ((locals.var_xg1x_dn9 - locals.var_q1s_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign14380_e13793;
        locals.var_q_temp1_dn4 = assign14380_e13793_d_n4;
        locals.var_q_temp1_dn6 = assign14380_e13793_d_n6;
        locals.var_q_temp1_dn7 = assign14380_e13793_d_n7;
        locals.var_q_temp1_dn8 = assign14380_e13793_d_n8;
        locals.var_q_temp1_dn9 = assign14380_e13793_d_n9;

        let (assign14390_e13801, assign14390_e13801_d_n4, assign14390_e13801_d_n6, assign14390_e13801_d_n7, assign14390_e13801_d_n8, assign14390_e13801_d_n9,) = {
    if ((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) {
        let assign14390_e13799: f64 = (locals.var_a0 * locals.var_q_temp1);
        (assign14390_e13799, ((locals.var_a0_dn4 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn4)), ((locals.var_a0_dn6 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn6)), ((locals.var_a0_dn7 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn7)), ((locals.var_a0_dn8 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn8)), ((locals.var_a0_dn9 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_aexp, locals.var_q_aexp_dn4, locals.var_q_aexp_dn6, locals.var_q_aexp_dn7, locals.var_q_aexp_dn8, locals.var_q_aexp_dn9,)
    }
};
        locals.var_q_aexp = assign14390_e13801;
        locals.var_q_aexp_dn4 = assign14390_e13801_d_n4;
        locals.var_q_aexp_dn6 = assign14390_e13801_d_n6;
        locals.var_q_aexp_dn7 = assign14390_e13801_d_n7;
        locals.var_q_aexp_dn8 = assign14390_e13801_d_n8;
        locals.var_q_aexp_dn9 = assign14390_e13801_d_n9;

        let (assign14400_e13811, assign14400_e13811_d_n4, assign14400_e13811_d_n6, assign14400_e13811_d_n7, assign14400_e13811_d_n8, assign14400_e13811_d_n9,) = {
    if ((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) {
        let assign14400_e13807: f64 = (locals.var_q_k1q1 * locals.var_q_k1q1);
        let assign14400_e13809: f64 = (assign14400_e13807 - locals.var_q_aexp);
        (assign14400_e13809, (((locals.var_q_k1q1_dn4 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn4)) - locals.var_q_aexp_dn4), (((locals.var_q_k1q1_dn6 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn6)) - locals.var_q_aexp_dn6), (((locals.var_q_k1q1_dn7 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn7)) - locals.var_q_aexp_dn7), (((locals.var_q_k1q1_dn8 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn8)) - locals.var_q_aexp_dn8), (((locals.var_q_k1q1_dn9 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn9)) - locals.var_q_aexp_dn9),)
    } else {
        (locals.var_q_qsq, locals.var_q_qsq_dn4, locals.var_q_qsq_dn6, locals.var_q_qsq_dn7, locals.var_q_qsq_dn8, locals.var_q_qsq_dn9,)
    }
};
        locals.var_q_qsq = assign14400_e13811;
        locals.var_q_qsq_dn4 = assign14400_e13811_d_n4;
        locals.var_q_qsq_dn6 = assign14400_e13811_d_n6;
        locals.var_q_qsq_dn7 = assign14400_e13811_d_n7;
        locals.var_q_qsq_dn8 = assign14400_e13811_d_n8;
        locals.var_q_qsq_dn9 = assign14400_e13811_d_n9;

        let (assign14410_e13823, assign14410_e13823_d_n4, assign14410_e13823_d_n6, assign14410_e13823_d_n7, assign14410_e13823_d_n8, assign14410_e13823_d_n9,) = {
    if ((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) {
        let assign14410_e13817: f64 = (2.0 * locals.var_k1);
        let assign14410_e13819: f64 = (assign14410_e13817 * locals.var_q_k1q1);
        let assign14410_e13821: f64 = (assign14410_e13819 + locals.var_q_aexp);
        (assign14410_e13821, ((((2.0 * locals.var_k1_dn4) * locals.var_q_k1q1) + (assign14410_e13817 * locals.var_q_k1q1_dn4)) + locals.var_q_aexp_dn4), ((((2.0 * locals.var_k1_dn6) * locals.var_q_k1q1) + (assign14410_e13817 * locals.var_q_k1q1_dn6)) + locals.var_q_aexp_dn6), ((((2.0 * locals.var_k1_dn7) * locals.var_q_k1q1) + (assign14410_e13817 * locals.var_q_k1q1_dn7)) + locals.var_q_aexp_dn7), ((((2.0 * locals.var_k1_dn8) * locals.var_q_k1q1) + (assign14410_e13817 * locals.var_q_k1q1_dn8)) + locals.var_q_aexp_dn8), ((((2.0 * locals.var_k1_dn9) * locals.var_q_k1q1) + (assign14410_e13817 * locals.var_q_k1q1_dn9)) + locals.var_q_aexp_dn9),)
    } else {
        (locals.var_q_d1_qsq, locals.var_q_d1_qsq_dn4, locals.var_q_d1_qsq_dn6, locals.var_q_d1_qsq_dn7, locals.var_q_d1_qsq_dn8, locals.var_q_d1_qsq_dn9,)
    }
};
        locals.var_q_d1_qsq = assign14410_e13823;
        locals.var_q_d1_qsq_dn4 = assign14410_e13823_d_n4;
        locals.var_q_d1_qsq_dn6 = assign14410_e13823_d_n6;
        locals.var_q_d1_qsq_dn7 = assign14410_e13823_d_n7;
        locals.var_q_d1_qsq_dn8 = assign14410_e13823_d_n8;
        locals.var_q_d1_qsq_dn9 = assign14410_e13823_d_n9;

        let (assign14420_e13835, assign14420_e13835_d_n4, assign14420_e13835_d_n6, assign14420_e13835_d_n7, assign14420_e13835_d_n8, assign14420_e13835_d_n9,) = {
    if ((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) {
        let assign14420_e13829: f64 = (2.0 * locals.var_k1);
        let assign14420_e13831: f64 = (assign14420_e13829 * locals.var_k1);
        let assign14420_e13833: f64 = (assign14420_e13831 - locals.var_q_aexp);
        (assign14420_e13833, ((((2.0 * locals.var_k1_dn4) * locals.var_k1) + (assign14420_e13829 * locals.var_k1_dn4)) - locals.var_q_aexp_dn4), ((((2.0 * locals.var_k1_dn6) * locals.var_k1) + (assign14420_e13829 * locals.var_k1_dn6)) - locals.var_q_aexp_dn6), ((((2.0 * locals.var_k1_dn7) * locals.var_k1) + (assign14420_e13829 * locals.var_k1_dn7)) - locals.var_q_aexp_dn7), ((((2.0 * locals.var_k1_dn8) * locals.var_k1) + (assign14420_e13829 * locals.var_k1_dn8)) - locals.var_q_aexp_dn8), ((((2.0 * locals.var_k1_dn9) * locals.var_k1) + (assign14420_e13829 * locals.var_k1_dn9)) - locals.var_q_aexp_dn9),)
    } else {
        (locals.var_q_d2_qsq, locals.var_q_d2_qsq_dn4, locals.var_q_d2_qsq_dn6, locals.var_q_d2_qsq_dn7, locals.var_q_d2_qsq_dn8, locals.var_q_d2_qsq_dn9,)
    }
};
        locals.var_q_d2_qsq = assign14420_e13835;
        locals.var_q_d2_qsq_dn4 = assign14420_e13835_d_n4;
        locals.var_q_d2_qsq_dn6 = assign14420_e13835_d_n6;
        locals.var_q_d2_qsq_dn7 = assign14420_e13835_d_n7;
        locals.var_q_d2_qsq_dn8 = assign14420_e13835_d_n8;
        locals.var_q_d2_qsq_dn9 = assign14420_e13835_d_n9;

        let assign14430_e13838: f64 = (-0.005);
        let assign14430_e13839: f64 = if locals.var_q_qsq < assign14430_e13838 { 1.0 } else { 0.0 };
        locals.var_guard582 = assign14430_e13839;

        let (assign14440_e13849, assign14440_e13849_d_n4, assign14440_e13849_d_n6, assign14440_e13849_d_n7, assign14440_e13849_d_n8, assign14440_e13849_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 != 0.0)) {
        let assign14440_e13846: f64 = (locals.var_q_qsq).abs();
        let assign14440_e13847: f64 = (assign14440_e13846).sqrt();
        (assign14440_e13847, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign14440_e13847)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign14440_e13847)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign14440_e13847)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign14440_e13847)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign14440_e13847)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign14440_e13849;
        locals.var_q_rac_qsq_dn4 = assign14440_e13849_d_n4;
        locals.var_q_rac_qsq_dn6 = assign14440_e13849_d_n6;
        locals.var_q_rac_qsq_dn7 = assign14440_e13849_d_n7;
        locals.var_q_rac_qsq_dn8 = assign14440_e13849_d_n8;
        locals.var_q_rac_qsq_dn9 = assign14440_e13849_d_n9;

        let (assign14450_e13862, assign14450_e13862_d_n4, assign14450_e13862_d_n6, assign14450_e13862_d_n7, assign14450_e13862_d_n8, assign14450_e13862_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 != 0.0)) {
        let assign14450_e13858: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign14450_e13859: f64 = (assign14450_e13858).tan();
        let assign14450_e13860: f64 = (locals.var_q_rac_qsq / assign14450_e13859);
        (assign14450_e13860, (((locals.var_q_rac_qsq_dn4 * assign14450_e13859) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn4) / ((assign14450_e13858).cos() * (assign14450_e13858).cos())))) / (assign14450_e13859 * assign14450_e13859)), (((locals.var_q_rac_qsq_dn6 * assign14450_e13859) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn6) / ((assign14450_e13858).cos() * (assign14450_e13858).cos())))) / (assign14450_e13859 * assign14450_e13859)), (((locals.var_q_rac_qsq_dn7 * assign14450_e13859) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn7) / ((assign14450_e13858).cos() * (assign14450_e13858).cos())))) / (assign14450_e13859 * assign14450_e13859)), (((locals.var_q_rac_qsq_dn8 * assign14450_e13859) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn8) / ((assign14450_e13858).cos() * (assign14450_e13858).cos())))) / (assign14450_e13859 * assign14450_e13859)), (((locals.var_q_rac_qsq_dn9 * assign14450_e13859) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn9) / ((assign14450_e13858).cos() * (assign14450_e13858).cos())))) / (assign14450_e13859 * assign14450_e13859)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign14450_e13862;
        locals.var_q_qcoth_dn4 = assign14450_e13862_d_n4;
        locals.var_q_qcoth_dn6 = assign14450_e13862_d_n6;
        locals.var_q_qcoth_dn7 = assign14450_e13862_d_n7;
        locals.var_q_qcoth_dn8 = assign14450_e13862_d_n8;
        locals.var_q_qcoth_dn9 = assign14450_e13862_d_n9;

        let (assign14460_e13874, assign14460_e13874_d_n4, assign14460_e13874_d_n6, assign14460_e13874_d_n7, assign14460_e13874_d_n8, assign14460_e13874_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 != 0.0)) {
        let assign14460_e13870: f64 = (0.25 * locals.var_q_d1_qsq);
        let assign14460_e13872: f64 = (assign14460_e13870 / locals.var_q_qsq);
        (assign14460_e13872, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign14460_e13870 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign14460_e13870 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign14460_e13870 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign14460_e13870 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign14460_e13870 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign14460_e13874;
        locals.var_q_temp1_dn4 = assign14460_e13874_d_n4;
        locals.var_q_temp1_dn6 = assign14460_e13874_d_n6;
        locals.var_q_temp1_dn7 = assign14460_e13874_d_n7;
        locals.var_q_temp1_dn8 = assign14460_e13874_d_n8;
        locals.var_q_temp1_dn9 = assign14460_e13874_d_n9;

        let (assign14470_e13890, assign14470_e13890_d_n4, assign14470_e13890_d_n6, assign14470_e13890_d_n7, assign14470_e13890_d_n8, assign14470_e13890_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 != 0.0)) {
        let assign14470_e13884: f64 = (2.0 - locals.var_q_qcoth);
        let assign14470_e13885: f64 = (locals.var_q_qcoth * assign14470_e13884);
        let assign14470_e13886: f64 = (locals.var_q_qsq + assign14470_e13885);
        let assign14470_e13888: f64 = (assign14470_e13886 * locals.var_q_temp1);
        (assign14470_e13888, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign14470_e13884) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign14470_e13886 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign14470_e13884) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign14470_e13886 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign14470_e13884) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign14470_e13886 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign14470_e13884) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign14470_e13886 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign14470_e13884) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign14470_e13886 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign14470_e13890;
        locals.var_q_d1_qcoth_dn4 = assign14470_e13890_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign14470_e13890_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign14470_e13890_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign14470_e13890_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign14470_e13890_d_n9;

    }

    pub(super) fn stamp_transient_block_35(
        locals: &mut StampLocals,
    ) {
        let (assign14480_e13914, assign14480_e13914_d_n4, assign14480_e13914_d_n6, assign14480_e13914_d_n7, assign14480_e13914_d_n8, assign14480_e13914_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 != 0.0)) {
        let assign14480_e13899: f64 = (2.0 * locals.var_q_d1_qcoth);
        let assign14480_e13902: f64 = (1.0 + locals.var_q_qcoth);
        let assign14480_e13903: f64 = (assign14480_e13899 * assign14480_e13902);
        let assign14480_e13904: f64 = (locals.var_q_d1_qsq - assign14480_e13903);
        let assign14480_e13906: f64 = (assign14480_e13904 * locals.var_q_temp1);
        let assign14480_e13909: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
        let assign14480_e13911: f64 = (assign14480_e13909 / locals.var_q_d1_qsq);
        let assign14480_e13912: f64 = (assign14480_e13906 + assign14480_e13911);
        (assign14480_e13912, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign14480_e13902) + (assign14480_e13899 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign14480_e13904 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign14480_e13909 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign14480_e13902) + (assign14480_e13899 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign14480_e13904 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign14480_e13909 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign14480_e13902) + (assign14480_e13899 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign14480_e13904 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign14480_e13909 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign14480_e13902) + (assign14480_e13899 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign14480_e13904 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign14480_e13909 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign14480_e13902) + (assign14480_e13899 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign14480_e13904 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign14480_e13909 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign14480_e13914;
        locals.var_q_d2_qcoth_dn4 = assign14480_e13914_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign14480_e13914_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign14480_e13914_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign14480_e13914_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign14480_e13914_d_n9;

        let (assign14490_e13926, assign14490_e13926_d_n4, assign14490_e13926_d_n6, assign14490_e13926_d_n7, assign14490_e13926_d_n8, assign14490_e13926_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 != 0.0)) {
        let assign14490_e13923: f64 = (0.5 * locals.var_q_qcoth);
        let assign14490_e13924: f64 = (1.0 - assign14490_e13923);
        (assign14490_e13924, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign14490_e13926;
        locals.var_q_temp2_dn4 = assign14490_e13926_d_n4;
        locals.var_q_temp2_dn6 = assign14490_e13926_d_n6;
        locals.var_q_temp2_dn7 = assign14490_e13926_d_n7;
        locals.var_q_temp2_dn8 = assign14490_e13926_d_n8;
        locals.var_q_temp2_dn9 = assign14490_e13926_d_n9;

        let (assign14500_e13938, assign14500_e13938_d_n4, assign14500_e13938_d_n6, assign14500_e13938_d_n7, assign14500_e13938_d_n8, assign14500_e13938_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 != 0.0)) {
        let assign14500_e13934: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
        let assign14500_e13936: f64 = (assign14500_e13934 * locals.var_q_temp2);
        (assign14500_e13936, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign14500_e13934 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign14500_e13934 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign14500_e13934 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign14500_e13934 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign14500_e13934 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign14500_e13938;
        locals.var_q_d1_ln_dn4 = assign14500_e13938_d_n4;
        locals.var_q_d1_ln_dn6 = assign14500_e13938_d_n6;
        locals.var_q_d1_ln_dn7 = assign14500_e13938_d_n7;
        locals.var_q_d1_ln_dn8 = assign14500_e13938_d_n8;
        locals.var_q_d1_ln_dn9 = assign14500_e13938_d_n9;

        let (assign14510_e13958, assign14510_e13958_d_n4, assign14510_e13958_d_n6, assign14510_e13958_d_n7, assign14510_e13958_d_n8, assign14510_e13958_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 != 0.0)) {
        let assign14510_e13946: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
        let assign14510_e13951: f64 = (0.5 * locals.var_q_d1_qcoth);
        let assign14510_e13952: f64 = (locals.var_q_d1_ln + assign14510_e13951);
        let assign14510_e13953: f64 = (locals.var_q_d1_qsq * assign14510_e13952);
        let assign14510_e13954: f64 = (assign14510_e13946 - assign14510_e13953);
        let assign14510_e13956: f64 = (assign14510_e13954 / locals.var_q_qsq);
        (assign14510_e13956, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign14510_e13952) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign14510_e13954 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign14510_e13952) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign14510_e13954 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign14510_e13952) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign14510_e13954 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign14510_e13952) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign14510_e13954 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign14510_e13952) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign14510_e13954 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign14510_e13958;
        locals.var_q_d2_ln_dn4 = assign14510_e13958_d_n4;
        locals.var_q_d2_ln_dn6 = assign14510_e13958_d_n6;
        locals.var_q_d2_ln_dn7 = assign14510_e13958_d_n7;
        locals.var_q_d2_ln_dn8 = assign14510_e13958_d_n8;
        locals.var_q_d2_ln_dn9 = assign14510_e13958_d_n9;

        let assign14520_e13961: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard583 = assign14520_e13961;

        let (assign14530_e13974, assign14530_e13974_d_n4, assign14530_e13974_d_n6, assign14530_e13974_d_n7, assign14530_e13974_d_n8, assign14530_e13974_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) {
        let assign14530_e13971: f64 = (locals.var_q_qsq).abs();
        let assign14530_e13972: f64 = (assign14530_e13971).sqrt();
        (assign14530_e13972, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign14530_e13972)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign14530_e13972)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign14530_e13972)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign14530_e13972)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign14530_e13972)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign14530_e13974;
        locals.var_q_rac_qsq_dn4 = assign14530_e13974_d_n4;
        locals.var_q_rac_qsq_dn6 = assign14530_e13974_d_n6;
        locals.var_q_rac_qsq_dn7 = assign14530_e13974_d_n7;
        locals.var_q_rac_qsq_dn8 = assign14530_e13974_d_n8;
        locals.var_q_rac_qsq_dn9 = assign14530_e13974_d_n9;

        let (assign14540_e13987, assign14540_e13987_d_n4, assign14540_e13987_d_n6, assign14540_e13987_d_n7, assign14540_e13987_d_n8, assign14540_e13987_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) {
        let assign14540_e13984: f64 = (-locals.var_q_rac_qsq);
        let assign14540_e13985: f64 = (assign14540_e13984).exp();
        (assign14540_e13985, (assign14540_e13985 * (-locals.var_q_rac_qsq_dn4)), (assign14540_e13985 * (-locals.var_q_rac_qsq_dn6)), (assign14540_e13985 * (-locals.var_q_rac_qsq_dn7)), (assign14540_e13985 * (-locals.var_q_rac_qsq_dn8)), (assign14540_e13985 * (-locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_invexpq, locals.var_q_invexpq_dn4, locals.var_q_invexpq_dn6, locals.var_q_invexpq_dn7, locals.var_q_invexpq_dn8, locals.var_q_invexpq_dn9,)
    }
};
        locals.var_q_invexpq = assign14540_e13987;
        locals.var_q_invexpq_dn4 = assign14540_e13987_d_n4;
        locals.var_q_invexpq_dn6 = assign14540_e13987_d_n6;
        locals.var_q_invexpq_dn7 = assign14540_e13987_d_n7;
        locals.var_q_invexpq_dn8 = assign14540_e13987_d_n8;
        locals.var_q_invexpq_dn9 = assign14540_e13987_d_n9;

        let (assign14550_e14006, assign14550_e14006_d_n4, assign14550_e14006_d_n6, assign14550_e14006_d_n7, assign14550_e14006_d_n8, assign14550_e14006_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) {
        let assign14550_e13999: f64 = (1.0 + locals.var_q_invexpq);
        let assign14550_e14000: f64 = (locals.var_q_rac_qsq * assign14550_e13999);
        let assign14550_e14003: f64 = (1.0 - locals.var_q_invexpq);
        let assign14550_e14004: f64 = (assign14550_e14000 / assign14550_e14003);
        (assign14550_e14004, (((((locals.var_q_rac_qsq_dn4 * assign14550_e13999) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn4)) * assign14550_e14003) - (assign14550_e14000 * (-locals.var_q_invexpq_dn4))) / (assign14550_e14003 * assign14550_e14003)), (((((locals.var_q_rac_qsq_dn6 * assign14550_e13999) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn6)) * assign14550_e14003) - (assign14550_e14000 * (-locals.var_q_invexpq_dn6))) / (assign14550_e14003 * assign14550_e14003)), (((((locals.var_q_rac_qsq_dn7 * assign14550_e13999) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn7)) * assign14550_e14003) - (assign14550_e14000 * (-locals.var_q_invexpq_dn7))) / (assign14550_e14003 * assign14550_e14003)), (((((locals.var_q_rac_qsq_dn8 * assign14550_e13999) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn8)) * assign14550_e14003) - (assign14550_e14000 * (-locals.var_q_invexpq_dn8))) / (assign14550_e14003 * assign14550_e14003)), (((((locals.var_q_rac_qsq_dn9 * assign14550_e13999) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn9)) * assign14550_e14003) - (assign14550_e14000 * (-locals.var_q_invexpq_dn9))) / (assign14550_e14003 * assign14550_e14003)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign14550_e14006;
        locals.var_q_qcoth_dn4 = assign14550_e14006_d_n4;
        locals.var_q_qcoth_dn6 = assign14550_e14006_d_n6;
        locals.var_q_qcoth_dn7 = assign14550_e14006_d_n7;
        locals.var_q_qcoth_dn8 = assign14550_e14006_d_n8;
        locals.var_q_qcoth_dn9 = assign14550_e14006_d_n9;

        let (assign14560_e14021, assign14560_e14021_d_n4, assign14560_e14021_d_n6, assign14560_e14021_d_n7, assign14560_e14021_d_n8, assign14560_e14021_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) {
        let assign14560_e14017: f64 = (0.25 * locals.var_q_d1_qsq);
        let assign14560_e14019: f64 = (assign14560_e14017 / locals.var_q_qsq);
        (assign14560_e14019, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign14560_e14017 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign14560_e14017 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign14560_e14017 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign14560_e14017 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign14560_e14017 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign14560_e14021;
        locals.var_q_temp1_dn4 = assign14560_e14021_d_n4;
        locals.var_q_temp1_dn6 = assign14560_e14021_d_n6;
        locals.var_q_temp1_dn7 = assign14560_e14021_d_n7;
        locals.var_q_temp1_dn8 = assign14560_e14021_d_n8;
        locals.var_q_temp1_dn9 = assign14560_e14021_d_n9;

        let (assign14570_e14040, assign14570_e14040_d_n4, assign14570_e14040_d_n6, assign14570_e14040_d_n7, assign14570_e14040_d_n8, assign14570_e14040_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) {
        let assign14570_e14034: f64 = (2.0 - locals.var_q_qcoth);
        let assign14570_e14035: f64 = (locals.var_q_qcoth * assign14570_e14034);
        let assign14570_e14036: f64 = (locals.var_q_qsq + assign14570_e14035);
        let assign14570_e14038: f64 = (assign14570_e14036 * locals.var_q_temp1);
        (assign14570_e14038, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign14570_e14034) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign14570_e14036 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign14570_e14034) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign14570_e14036 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign14570_e14034) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign14570_e14036 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign14570_e14034) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign14570_e14036 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign14570_e14034) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign14570_e14036 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign14570_e14040;
        locals.var_q_d1_qcoth_dn4 = assign14570_e14040_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign14570_e14040_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign14570_e14040_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign14570_e14040_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign14570_e14040_d_n9;

        let (assign14580_e14067, assign14580_e14067_d_n4, assign14580_e14067_d_n6, assign14580_e14067_d_n7, assign14580_e14067_d_n8, assign14580_e14067_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) {
        let assign14580_e14052: f64 = (2.0 * locals.var_q_d1_qcoth);
        let assign14580_e14055: f64 = (1.0 + locals.var_q_qcoth);
        let assign14580_e14056: f64 = (assign14580_e14052 * assign14580_e14055);
        let assign14580_e14057: f64 = (locals.var_q_d1_qsq - assign14580_e14056);
        let assign14580_e14059: f64 = (assign14580_e14057 * locals.var_q_temp1);
        let assign14580_e14062: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
        let assign14580_e14064: f64 = (assign14580_e14062 / locals.var_q_d1_qsq);
        let assign14580_e14065: f64 = (assign14580_e14059 + assign14580_e14064);
        (assign14580_e14065, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign14580_e14055) + (assign14580_e14052 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign14580_e14057 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign14580_e14062 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign14580_e14055) + (assign14580_e14052 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign14580_e14057 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign14580_e14062 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign14580_e14055) + (assign14580_e14052 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign14580_e14057 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign14580_e14062 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign14580_e14055) + (assign14580_e14052 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign14580_e14057 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign14580_e14062 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign14580_e14055) + (assign14580_e14052 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign14580_e14057 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign14580_e14062 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign14580_e14067;
        locals.var_q_d2_qcoth_dn4 = assign14580_e14067_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign14580_e14067_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign14580_e14067_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign14580_e14067_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign14580_e14067_d_n9;

        let (assign14590_e14082, assign14590_e14082_d_n4, assign14590_e14082_d_n6, assign14590_e14082_d_n7, assign14590_e14082_d_n8, assign14590_e14082_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) {
        let assign14590_e14079: f64 = (0.5 * locals.var_q_qcoth);
        let assign14590_e14080: f64 = (1.0 - assign14590_e14079);
        (assign14590_e14080, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign14590_e14082;
        locals.var_q_temp2_dn4 = assign14590_e14082_d_n4;
        locals.var_q_temp2_dn6 = assign14590_e14082_d_n6;
        locals.var_q_temp2_dn7 = assign14590_e14082_d_n7;
        locals.var_q_temp2_dn8 = assign14590_e14082_d_n8;
        locals.var_q_temp2_dn9 = assign14590_e14082_d_n9;

        let (assign14600_e14097, assign14600_e14097_d_n4, assign14600_e14097_d_n6, assign14600_e14097_d_n7, assign14600_e14097_d_n8, assign14600_e14097_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) {
        let assign14600_e14093: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
        let assign14600_e14095: f64 = (assign14600_e14093 * locals.var_q_temp2);
        (assign14600_e14095, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign14600_e14093 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign14600_e14093 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign14600_e14093 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign14600_e14093 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign14600_e14093 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign14600_e14097;
        locals.var_q_d1_ln_dn4 = assign14600_e14097_d_n4;
        locals.var_q_d1_ln_dn6 = assign14600_e14097_d_n6;
        locals.var_q_d1_ln_dn7 = assign14600_e14097_d_n7;
        locals.var_q_d1_ln_dn8 = assign14600_e14097_d_n8;
        locals.var_q_d1_ln_dn9 = assign14600_e14097_d_n9;

        let (assign14610_e14120, assign14610_e14120_d_n4, assign14610_e14120_d_n6, assign14610_e14120_d_n7, assign14610_e14120_d_n8, assign14610_e14120_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) {
        let assign14610_e14108: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
        let assign14610_e14113: f64 = (0.5 * locals.var_q_d1_qcoth);
        let assign14610_e14114: f64 = (locals.var_q_d1_ln + assign14610_e14113);
        let assign14610_e14115: f64 = (locals.var_q_d1_qsq * assign14610_e14114);
        let assign14610_e14116: f64 = (assign14610_e14108 - assign14610_e14115);
        let assign14610_e14118: f64 = (assign14610_e14116 / locals.var_q_qsq);
        (assign14610_e14118, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign14610_e14114) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign14610_e14116 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign14610_e14114) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign14610_e14116 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign14610_e14114) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign14610_e14116 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign14610_e14114) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign14610_e14116 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign14610_e14114) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign14610_e14116 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign14610_e14120;
        locals.var_q_d2_ln_dn4 = assign14610_e14120_d_n4;
        locals.var_q_d2_ln_dn6 = assign14610_e14120_d_n6;
        locals.var_q_d2_ln_dn7 = assign14610_e14120_d_n7;
        locals.var_q_d2_ln_dn8 = assign14610_e14120_d_n8;
        locals.var_q_d2_ln_dn9 = assign14610_e14120_d_n9;

        let (assign14620_e14150, assign14620_e14150_d_n4, assign14620_e14150_d_n6, assign14620_e14150_d_n7, assign14620_e14150_d_n8, assign14620_e14150_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 == 0.0)) {
        let assign14620_e14134: f64 = (locals.var_q_qsq * 0.0166666666667);
        let assign14620_e14138: f64 = (locals.var_q_qsq * 0.0238095238095);
        let assign14620_e14142: f64 = (locals.var_q_qsq * 0.025);
        let assign14620_e14143: f64 = (1.0 - assign14620_e14142);
        let assign14620_e14144: f64 = (assign14620_e14138 * assign14620_e14143);
        let assign14620_e14145: f64 = (1.0 - assign14620_e14144);
        let assign14620_e14146: f64 = (assign14620_e14134 * assign14620_e14145);
        let assign14620_e14147: f64 = (1.0 - assign14620_e14146);
        let assign14620_e14148: f64 = (0.1666666666667 * assign14620_e14147);
        (assign14620_e14148, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0166666666667) * assign14620_e14145) + (assign14620_e14134 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign14620_e14143) + (assign14620_e14138 * (-(locals.var_q_qsq_dn4 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0166666666667) * assign14620_e14145) + (assign14620_e14134 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign14620_e14143) + (assign14620_e14138 * (-(locals.var_q_qsq_dn6 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0166666666667) * assign14620_e14145) + (assign14620_e14134 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign14620_e14143) + (assign14620_e14138 * (-(locals.var_q_qsq_dn7 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0166666666667) * assign14620_e14145) + (assign14620_e14134 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign14620_e14143) + (assign14620_e14138 * (-(locals.var_q_qsq_dn8 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0166666666667) * assign14620_e14145) + (assign14620_e14134 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign14620_e14143) + (assign14620_e14138 * (-(locals.var_q_qsq_dn9 * 0.025))))))))),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign14620_e14150;
        locals.var_q_temp3_dn4 = assign14620_e14150_d_n4;
        locals.var_q_temp3_dn6 = assign14620_e14150_d_n6;
        locals.var_q_temp3_dn7 = assign14620_e14150_d_n7;
        locals.var_q_temp3_dn8 = assign14620_e14150_d_n8;
        locals.var_q_temp3_dn9 = assign14620_e14150_d_n9;

        let (assign14630_e14166, assign14630_e14166_d_n4, assign14630_e14166_d_n6, assign14630_e14166_d_n7, assign14630_e14166_d_n8, assign14630_e14166_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 == 0.0)) {
        let assign14630_e14163: f64 = (locals.var_q_qsq * locals.var_q_temp3);
        let assign14630_e14164: f64 = (2.0 + assign14630_e14163);
        (assign14630_e14164, ((locals.var_q_qsq_dn4 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn4)), ((locals.var_q_qsq_dn6 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn6)), ((locals.var_q_qsq_dn7 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn7)), ((locals.var_q_qsq_dn8 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn8)), ((locals.var_q_qsq_dn9 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn9)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign14630_e14166;
        locals.var_q_qcoth_dn4 = assign14630_e14166_d_n4;
        locals.var_q_qcoth_dn6 = assign14630_e14166_d_n6;
        locals.var_q_qcoth_dn7 = assign14630_e14166_d_n7;
        locals.var_q_qcoth_dn8 = assign14630_e14166_d_n8;
        locals.var_q_qcoth_dn9 = assign14630_e14166_d_n9;

        let (assign14640_e14196, assign14640_e14196_d_n4, assign14640_e14196_d_n6, assign14640_e14196_d_n7, assign14640_e14196_d_n8, assign14640_e14196_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 == 0.0)) {
        let assign14640_e14180: f64 = (locals.var_q_qsq * 0.0333333333333);
        let assign14640_e14184: f64 = (locals.var_q_qsq * 0.0357142857143);
        let assign14640_e14188: f64 = (locals.var_q_qsq * 0.0333333333333);
        let assign14640_e14189: f64 = (1.0 - assign14640_e14188);
        let assign14640_e14190: f64 = (assign14640_e14184 * assign14640_e14189);
        let assign14640_e14191: f64 = (1.0 - assign14640_e14190);
        let assign14640_e14192: f64 = (assign14640_e14180 * assign14640_e14191);
        let assign14640_e14193: f64 = (1.0 - assign14640_e14192);
        let assign14640_e14194: f64 = (0.1666666666667 * assign14640_e14193);
        (assign14640_e14194, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0333333333333) * assign14640_e14191) + (assign14640_e14180 * (-(((locals.var_q_qsq_dn4 * 0.0357142857143) * assign14640_e14189) + (assign14640_e14184 * (-(locals.var_q_qsq_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0333333333333) * assign14640_e14191) + (assign14640_e14180 * (-(((locals.var_q_qsq_dn6 * 0.0357142857143) * assign14640_e14189) + (assign14640_e14184 * (-(locals.var_q_qsq_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0333333333333) * assign14640_e14191) + (assign14640_e14180 * (-(((locals.var_q_qsq_dn7 * 0.0357142857143) * assign14640_e14189) + (assign14640_e14184 * (-(locals.var_q_qsq_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0333333333333) * assign14640_e14191) + (assign14640_e14180 * (-(((locals.var_q_qsq_dn8 * 0.0357142857143) * assign14640_e14189) + (assign14640_e14184 * (-(locals.var_q_qsq_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0333333333333) * assign14640_e14191) + (assign14640_e14180 * (-(((locals.var_q_qsq_dn9 * 0.0357142857143) * assign14640_e14189) + (assign14640_e14184 * (-(locals.var_q_qsq_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign14640_e14196;
        locals.var_q_temp1_dn4 = assign14640_e14196_d_n4;
        locals.var_q_temp1_dn6 = assign14640_e14196_d_n6;
        locals.var_q_temp1_dn7 = assign14640_e14196_d_n7;
        locals.var_q_temp1_dn8 = assign14640_e14196_d_n8;
        locals.var_q_temp1_dn9 = assign14640_e14196_d_n9;

        let (assign14650_e14210, assign14650_e14210_d_n4, assign14650_e14210_d_n6, assign14650_e14210_d_n7, assign14650_e14210_d_n8, assign14650_e14210_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 == 0.0)) {
        let assign14650_e14208: f64 = (locals.var_q_d1_qsq * locals.var_q_temp1);
        (assign14650_e14208, ((locals.var_q_d1_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn4)), ((locals.var_q_d1_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn6)), ((locals.var_q_d1_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn7)), ((locals.var_q_d1_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn8)), ((locals.var_q_d1_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign14650_e14210;
        locals.var_q_d1_qcoth_dn4 = assign14650_e14210_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign14650_e14210_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign14650_e14210_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign14650_e14210_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign14650_e14210_d_n9;

        let (assign14660_e14240, assign14660_e14240_d_n4, assign14660_e14240_d_n6, assign14660_e14240_d_n7, assign14660_e14240_d_n8, assign14660_e14240_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 == 0.0)) {
        let assign14660_e14224: f64 = (locals.var_q_qsq * 0.0714285714286);
        let assign14660_e14228: f64 = (0.05 * locals.var_q_qsq);
        let assign14660_e14232: f64 = (0.0420875420875421 * locals.var_q_qsq);
        let assign14660_e14233: f64 = (1.0 - assign14660_e14232);
        let assign14660_e14234: f64 = (assign14660_e14228 * assign14660_e14233);
        let assign14660_e14235: f64 = (1.0 - assign14660_e14234);
        let assign14660_e14236: f64 = (assign14660_e14224 * assign14660_e14235);
        let assign14660_e14237: f64 = (1.0 - assign14660_e14236);
        let assign14660_e14238: f64 = (0.0055555555556 * assign14660_e14237);
        (assign14660_e14238, (0.0055555555556 * (-(((locals.var_q_qsq_dn4 * 0.0714285714286) * assign14660_e14235) + (assign14660_e14224 * (-(((0.05 * locals.var_q_qsq_dn4) * assign14660_e14233) + (assign14660_e14228 * (-(0.0420875420875421 * locals.var_q_qsq_dn4))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn6 * 0.0714285714286) * assign14660_e14235) + (assign14660_e14224 * (-(((0.05 * locals.var_q_qsq_dn6) * assign14660_e14233) + (assign14660_e14228 * (-(0.0420875420875421 * locals.var_q_qsq_dn6))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn7 * 0.0714285714286) * assign14660_e14235) + (assign14660_e14224 * (-(((0.05 * locals.var_q_qsq_dn7) * assign14660_e14233) + (assign14660_e14228 * (-(0.0420875420875421 * locals.var_q_qsq_dn7))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn8 * 0.0714285714286) * assign14660_e14235) + (assign14660_e14224 * (-(((0.05 * locals.var_q_qsq_dn8) * assign14660_e14233) + (assign14660_e14228 * (-(0.0420875420875421 * locals.var_q_qsq_dn8))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn9 * 0.0714285714286) * assign14660_e14235) + (assign14660_e14224 * (-(((0.05 * locals.var_q_qsq_dn9) * assign14660_e14233) + (assign14660_e14228 * (-(0.0420875420875421 * locals.var_q_qsq_dn9))))))))),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign14660_e14240;
        locals.var_q_temp2_dn4 = assign14660_e14240_d_n4;
        locals.var_q_temp2_dn6 = assign14660_e14240_d_n6;
        locals.var_q_temp2_dn7 = assign14660_e14240_d_n7;
        locals.var_q_temp2_dn8 = assign14660_e14240_d_n8;
        locals.var_q_temp2_dn9 = assign14660_e14240_d_n9;

        let (assign14670_e14260, assign14670_e14260_d_n4, assign14670_e14260_d_n6, assign14670_e14260_d_n7, assign14670_e14260_d_n8, assign14670_e14260_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 == 0.0)) {
        let assign14670_e14252: f64 = (locals.var_q_d2_qsq * locals.var_q_temp1);
        let assign14670_e14255: f64 = (locals.var_q_d1_qsq * locals.var_q_d1_qsq);
        let assign14670_e14257: f64 = (assign14670_e14255 * locals.var_q_temp2);
        let assign14670_e14258: f64 = (assign14670_e14252 - assign14670_e14257);
        (assign14670_e14258, (((locals.var_q_d2_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn4)) - ((((locals.var_q_d1_qsq_dn4 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn4)) * locals.var_q_temp2) + (assign14670_e14255 * locals.var_q_temp2_dn4))), (((locals.var_q_d2_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn6)) - ((((locals.var_q_d1_qsq_dn6 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn6)) * locals.var_q_temp2) + (assign14670_e14255 * locals.var_q_temp2_dn6))), (((locals.var_q_d2_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn7)) - ((((locals.var_q_d1_qsq_dn7 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn7)) * locals.var_q_temp2) + (assign14670_e14255 * locals.var_q_temp2_dn7))), (((locals.var_q_d2_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn8)) - ((((locals.var_q_d1_qsq_dn8 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn8)) * locals.var_q_temp2) + (assign14670_e14255 * locals.var_q_temp2_dn8))), (((locals.var_q_d2_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn9)) - ((((locals.var_q_d1_qsq_dn9 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn9)) * locals.var_q_temp2) + (assign14670_e14255 * locals.var_q_temp2_dn9))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign14670_e14260;
        locals.var_q_d2_qcoth_dn4 = assign14670_e14260_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign14670_e14260_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign14670_e14260_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign14670_e14260_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign14670_e14260_d_n9;

        let (assign14680_e14277, assign14680_e14277_d_n4, assign14680_e14277_d_n6, assign14680_e14277_d_n7, assign14680_e14277_d_n8, assign14680_e14277_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 == 0.0)) {
        let assign14680_e14271: f64 = (-0.5);
        let assign14680_e14273: f64 = (assign14680_e14271 * locals.var_q_d1_qsq);
        let assign14680_e14275: f64 = (assign14680_e14273 * locals.var_q_temp3);
        (assign14680_e14275, (((assign14680_e14271 * locals.var_q_d1_qsq_dn4) * locals.var_q_temp3) + (assign14680_e14273 * locals.var_q_temp3_dn4)), (((assign14680_e14271 * locals.var_q_d1_qsq_dn6) * locals.var_q_temp3) + (assign14680_e14273 * locals.var_q_temp3_dn6)), (((assign14680_e14271 * locals.var_q_d1_qsq_dn7) * locals.var_q_temp3) + (assign14680_e14273 * locals.var_q_temp3_dn7)), (((assign14680_e14271 * locals.var_q_d1_qsq_dn8) * locals.var_q_temp3) + (assign14680_e14273 * locals.var_q_temp3_dn8)), (((assign14680_e14271 * locals.var_q_d1_qsq_dn9) * locals.var_q_temp3) + (assign14680_e14273 * locals.var_q_temp3_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign14680_e14277;
        locals.var_q_d1_ln_dn4 = assign14680_e14277_d_n4;
        locals.var_q_d1_ln_dn6 = assign14680_e14277_d_n6;
        locals.var_q_d1_ln_dn7 = assign14680_e14277_d_n7;
        locals.var_q_d1_ln_dn8 = assign14680_e14277_d_n8;
        locals.var_q_d1_ln_dn9 = assign14680_e14277_d_n9;

        let (assign14690_e14314, assign14690_e14314_d_n4, assign14690_e14314_d_n6, assign14690_e14314_d_n7, assign14690_e14314_d_n8, assign14690_e14314_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 == 0.0)) {
        let assign14690_e14288: f64 = (-0.5);
        let assign14690_e14290: f64 = (assign14690_e14288 * locals.var_q_d2_qsq);
        let assign14690_e14292: f64 = (assign14690_e14290 * locals.var_q_temp3);
        let assign14690_e14295: f64 = (0.25 * 0.0055555555556);
        let assign14690_e14297: f64 = (assign14690_e14295 * locals.var_q_d1_qsq);
        let assign14690_e14299: f64 = (assign14690_e14297 * locals.var_q_d1_qsq);
        let assign14690_e14303: f64 = (locals.var_q_qsq * 0.0238095238095);
        let assign14690_e14307: f64 = (0.075 * locals.var_q_qsq);
        let assign14690_e14308: f64 = (2.0 - assign14690_e14307);
        let assign14690_e14309: f64 = (assign14690_e14303 * assign14690_e14308);
        let assign14690_e14310: f64 = (1.0 - assign14690_e14309);
        let assign14690_e14311: f64 = (assign14690_e14299 * assign14690_e14310);
        let assign14690_e14312: f64 = (assign14690_e14292 + assign14690_e14311);
        (assign14690_e14312, ((((assign14690_e14288 * locals.var_q_d2_qsq_dn4) * locals.var_q_temp3) + (assign14690_e14290 * locals.var_q_temp3_dn4)) + (((((assign14690_e14295 * locals.var_q_d1_qsq_dn4) * locals.var_q_d1_qsq) + (assign14690_e14297 * locals.var_q_d1_qsq_dn4)) * assign14690_e14310) + (assign14690_e14299 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign14690_e14308) + (assign14690_e14303 * (-(0.075 * locals.var_q_qsq_dn4)))))))), ((((assign14690_e14288 * locals.var_q_d2_qsq_dn6) * locals.var_q_temp3) + (assign14690_e14290 * locals.var_q_temp3_dn6)) + (((((assign14690_e14295 * locals.var_q_d1_qsq_dn6) * locals.var_q_d1_qsq) + (assign14690_e14297 * locals.var_q_d1_qsq_dn6)) * assign14690_e14310) + (assign14690_e14299 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign14690_e14308) + (assign14690_e14303 * (-(0.075 * locals.var_q_qsq_dn6)))))))), ((((assign14690_e14288 * locals.var_q_d2_qsq_dn7) * locals.var_q_temp3) + (assign14690_e14290 * locals.var_q_temp3_dn7)) + (((((assign14690_e14295 * locals.var_q_d1_qsq_dn7) * locals.var_q_d1_qsq) + (assign14690_e14297 * locals.var_q_d1_qsq_dn7)) * assign14690_e14310) + (assign14690_e14299 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign14690_e14308) + (assign14690_e14303 * (-(0.075 * locals.var_q_qsq_dn7)))))))), ((((assign14690_e14288 * locals.var_q_d2_qsq_dn8) * locals.var_q_temp3) + (assign14690_e14290 * locals.var_q_temp3_dn8)) + (((((assign14690_e14295 * locals.var_q_d1_qsq_dn8) * locals.var_q_d1_qsq) + (assign14690_e14297 * locals.var_q_d1_qsq_dn8)) * assign14690_e14310) + (assign14690_e14299 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign14690_e14308) + (assign14690_e14303 * (-(0.075 * locals.var_q_qsq_dn8)))))))), ((((assign14690_e14288 * locals.var_q_d2_qsq_dn9) * locals.var_q_temp3) + (assign14690_e14290 * locals.var_q_temp3_dn9)) + (((((assign14690_e14295 * locals.var_q_d1_qsq_dn9) * locals.var_q_d1_qsq) + (assign14690_e14297 * locals.var_q_d1_qsq_dn9)) * assign14690_e14310) + (assign14690_e14299 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign14690_e14308) + (assign14690_e14303 * (-(0.075 * locals.var_q_qsq_dn9)))))))),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign14690_e14314;
        locals.var_q_d2_ln_dn4 = assign14690_e14314_d_n4;
        locals.var_q_d2_ln_dn6 = assign14690_e14314_d_n6;
        locals.var_q_d2_ln_dn7 = assign14690_e14314_d_n7;
        locals.var_q_d2_ln_dn8 = assign14690_e14314_d_n8;
        locals.var_q_d2_ln_dn9 = assign14690_e14314_d_n9;

        let assign14700_e14317: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard584 = assign14700_e14317;

        let (assign14710_e14335, assign14710_e14335_d_n4, assign14710_e14335_d_n6, assign14710_e14335_d_n7, assign14710_e14335_d_n8, assign14710_e14335_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard584 != 0.0)) {
        let assign14710_e14325: f64 = (4.0 * locals.var_q_qsq);
        let assign14710_e14330: f64 = (2.0 - locals.var_q_invexpq);
        let assign14710_e14331: f64 = (locals.var_q_invexpq * assign14710_e14330);
        let assign14710_e14332: f64 = (1.0 - assign14710_e14331);
        let assign14710_e14333: f64 = (assign14710_e14325 / assign14710_e14332);
        (assign14710_e14333, ((((4.0 * locals.var_q_qsq_dn4) * assign14710_e14332) - (assign14710_e14325 * (-((locals.var_q_invexpq_dn4 * assign14710_e14330) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn4)))))) / (assign14710_e14332 * assign14710_e14332)), ((((4.0 * locals.var_q_qsq_dn6) * assign14710_e14332) - (assign14710_e14325 * (-((locals.var_q_invexpq_dn6 * assign14710_e14330) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn6)))))) / (assign14710_e14332 * assign14710_e14332)), ((((4.0 * locals.var_q_qsq_dn7) * assign14710_e14332) - (assign14710_e14325 * (-((locals.var_q_invexpq_dn7 * assign14710_e14330) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn7)))))) / (assign14710_e14332 * assign14710_e14332)), ((((4.0 * locals.var_q_qsq_dn8) * assign14710_e14332) - (assign14710_e14325 * (-((locals.var_q_invexpq_dn8 * assign14710_e14330) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn8)))))) / (assign14710_e14332 * assign14710_e14332)), ((((4.0 * locals.var_q_qsq_dn9) * assign14710_e14332) - (assign14710_e14325 * (-((locals.var_q_invexpq_dn9 * assign14710_e14330) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn9)))))) / (assign14710_e14332 * assign14710_e14332)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign14710_e14335;
        locals.var_q_temp2_dn4 = assign14710_e14335_d_n4;
        locals.var_q_temp2_dn6 = assign14710_e14335_d_n6;
        locals.var_q_temp2_dn7 = assign14710_e14335_d_n7;
        locals.var_q_temp2_dn8 = assign14710_e14335_d_n8;
        locals.var_q_temp2_dn9 = assign14710_e14335_d_n9;

        let (assign14720_e14345, assign14720_e14345_d_n4, assign14720_e14345_d_n6, assign14720_e14345_d_n7, assign14720_e14345_d_n8, assign14720_e14345_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard584 != 0.0)) {
        let assign14720_e14343: f64 = (locals.var_q_temp2 * locals.var_q_invexpq);
        (assign14720_e14343, ((locals.var_q_temp2_dn4 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn4)), ((locals.var_q_temp2_dn6 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn6)), ((locals.var_q_temp2_dn7 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn7)), ((locals.var_q_temp2_dn8 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn8)), ((locals.var_q_temp2_dn9 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn9)),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign14720_e14345;
        locals.var_q_sh_term_dn4 = assign14720_e14345_d_n4;
        locals.var_q_sh_term_dn6 = assign14720_e14345_d_n6;
        locals.var_q_sh_term_dn7 = assign14720_e14345_d_n7;
        locals.var_q_sh_term_dn8 = assign14720_e14345_d_n8;
        locals.var_q_sh_term_dn9 = assign14720_e14345_d_n9;

        let (assign14730_e14356, assign14730_e14356_d_n4, assign14730_e14356_d_n6, assign14730_e14356_d_n7, assign14730_e14356_d_n8, assign14730_e14356_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard584 != 0.0)) {
        let assign14730_e14352: f64 = (locals.var_q_temp2).ln();
        let assign14730_e14354: f64 = (assign14730_e14352 - locals.var_q_rac_qsq);
        (assign14730_e14354, ((locals.var_q_temp2_dn4 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn4), ((locals.var_q_temp2_dn6 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn6), ((locals.var_q_temp2_dn7 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn7), ((locals.var_q_temp2_dn8 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn8), ((locals.var_q_temp2_dn9 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn9),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign14730_e14356;
        locals.var_q_ln_term_dn4 = assign14730_e14356_d_n4;
        locals.var_q_ln_term_dn6 = assign14730_e14356_d_n6;
        locals.var_q_ln_term_dn7 = assign14730_e14356_d_n7;
        locals.var_q_ln_term_dn8 = assign14730_e14356_d_n8;
        locals.var_q_ln_term_dn9 = assign14730_e14356_d_n9;

        let assign14740_e14359: f64 = (-0.005);
        let assign14740_e14360: f64 = if locals.var_q_qsq < assign14740_e14359 { 1.0 } else { 0.0 };
        locals.var_guard585 = assign14740_e14360;

        let (assign14750_e14374, assign14750_e14374_d_n4, assign14750_e14374_d_n6, assign14750_e14374_d_n7, assign14750_e14374_d_n8, assign14750_e14374_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard584 == 0.0)) && (locals.var_guard585 != 0.0)) {
        let assign14750_e14371: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign14750_e14372: f64 = (assign14750_e14371).sin();
        (assign14750_e14372, ((assign14750_e14371).cos() * (0.5 * locals.var_q_rac_qsq_dn4)), ((assign14750_e14371).cos() * (0.5 * locals.var_q_rac_qsq_dn6)), ((assign14750_e14371).cos() * (0.5 * locals.var_q_rac_qsq_dn7)), ((assign14750_e14371).cos() * (0.5 * locals.var_q_rac_qsq_dn8)), ((assign14750_e14371).cos() * (0.5 * locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign14750_e14374;
        locals.var_q_temp2_dn4 = assign14750_e14374_d_n4;
        locals.var_q_temp2_dn6 = assign14750_e14374_d_n6;
        locals.var_q_temp2_dn7 = assign14750_e14374_d_n7;
        locals.var_q_temp2_dn8 = assign14750_e14374_d_n8;
        locals.var_q_temp2_dn9 = assign14750_e14374_d_n9;

        let (assign14760_e14390, assign14760_e14390_d_n4, assign14760_e14390_d_n6, assign14760_e14390_d_n7, assign14760_e14390_d_n8, assign14760_e14390_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard584 == 0.0)) && (locals.var_guard585 != 0.0)) {
        let assign14760_e14384: f64 = (-locals.var_q_qsq);
        let assign14760_e14387: f64 = (locals.var_q_temp2 * locals.var_q_temp2);
        let assign14760_e14388: f64 = (assign14760_e14384 / assign14760_e14387);
        (assign14760_e14388, ((((-locals.var_q_qsq_dn4) * assign14760_e14387) - (assign14760_e14384 * ((locals.var_q_temp2_dn4 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn4)))) / (assign14760_e14387 * assign14760_e14387)), ((((-locals.var_q_qsq_dn6) * assign14760_e14387) - (assign14760_e14384 * ((locals.var_q_temp2_dn6 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn6)))) / (assign14760_e14387 * assign14760_e14387)), ((((-locals.var_q_qsq_dn7) * assign14760_e14387) - (assign14760_e14384 * ((locals.var_q_temp2_dn7 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn7)))) / (assign14760_e14387 * assign14760_e14387)), ((((-locals.var_q_qsq_dn8) * assign14760_e14387) - (assign14760_e14384 * ((locals.var_q_temp2_dn8 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn8)))) / (assign14760_e14387 * assign14760_e14387)), ((((-locals.var_q_qsq_dn9) * assign14760_e14387) - (assign14760_e14384 * ((locals.var_q_temp2_dn9 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn9)))) / (assign14760_e14387 * assign14760_e14387)),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign14760_e14390;
        locals.var_q_sh_term_dn4 = assign14760_e14390_d_n4;
        locals.var_q_sh_term_dn6 = assign14760_e14390_d_n6;
        locals.var_q_sh_term_dn7 = assign14760_e14390_d_n7;
        locals.var_q_sh_term_dn8 = assign14760_e14390_d_n8;
        locals.var_q_sh_term_dn9 = assign14760_e14390_d_n9;

        let (assign14770_e14402, assign14770_e14402_d_n4, assign14770_e14402_d_n6, assign14770_e14402_d_n7, assign14770_e14402_d_n8, assign14770_e14402_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard584 == 0.0)) && (locals.var_guard585 != 0.0)) {
        let assign14770_e14400: f64 = (locals.var_q_sh_term).ln();
        (assign14770_e14400, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign14770_e14402;
        locals.var_q_ln_term_dn4 = assign14770_e14402_d_n4;
        locals.var_q_ln_term_dn6 = assign14770_e14402_d_n6;
        locals.var_q_ln_term_dn7 = assign14770_e14402_d_n7;
        locals.var_q_ln_term_dn8 = assign14770_e14402_d_n8;
        locals.var_q_ln_term_dn9 = assign14770_e14402_d_n9;

    }

    pub(super) fn stamp_transient_block_36(
        locals: &mut StampLocals,
    ) {
        let (assign14780_e14430, assign14780_e14430_d_n4, assign14780_e14430_d_n6, assign14780_e14430_d_n7, assign14780_e14430_d_n8, assign14780_e14430_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard584 == 0.0)) && (locals.var_guard585 == 0.0)) {
        let assign14780_e14415: f64 = (locals.var_q_qsq * 0.3333333333333);
        let assign14780_e14419: f64 = (0.05 * locals.var_q_qsq);
        let assign14780_e14423: f64 = (0.0396825396825397 * locals.var_q_qsq);
        let assign14780_e14424: f64 = (1.0 - assign14780_e14423);
        let assign14780_e14425: f64 = (assign14780_e14419 * assign14780_e14424);
        let assign14780_e14426: f64 = (1.0 - assign14780_e14425);
        let assign14780_e14427: f64 = (assign14780_e14415 * assign14780_e14426);
        let assign14780_e14428: f64 = (4.0 - assign14780_e14427);
        (assign14780_e14428, (-(((locals.var_q_qsq_dn4 * 0.3333333333333) * assign14780_e14426) + (assign14780_e14415 * (-(((0.05 * locals.var_q_qsq_dn4) * assign14780_e14424) + (assign14780_e14419 * (-(0.0396825396825397 * locals.var_q_qsq_dn4)))))))), (-(((locals.var_q_qsq_dn6 * 0.3333333333333) * assign14780_e14426) + (assign14780_e14415 * (-(((0.05 * locals.var_q_qsq_dn6) * assign14780_e14424) + (assign14780_e14419 * (-(0.0396825396825397 * locals.var_q_qsq_dn6)))))))), (-(((locals.var_q_qsq_dn7 * 0.3333333333333) * assign14780_e14426) + (assign14780_e14415 * (-(((0.05 * locals.var_q_qsq_dn7) * assign14780_e14424) + (assign14780_e14419 * (-(0.0396825396825397 * locals.var_q_qsq_dn7)))))))), (-(((locals.var_q_qsq_dn8 * 0.3333333333333) * assign14780_e14426) + (assign14780_e14415 * (-(((0.05 * locals.var_q_qsq_dn8) * assign14780_e14424) + (assign14780_e14419 * (-(0.0396825396825397 * locals.var_q_qsq_dn8)))))))), (-(((locals.var_q_qsq_dn9 * 0.3333333333333) * assign14780_e14426) + (assign14780_e14415 * (-(((0.05 * locals.var_q_qsq_dn9) * assign14780_e14424) + (assign14780_e14419 * (-(0.0396825396825397 * locals.var_q_qsq_dn9)))))))),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign14780_e14430;
        locals.var_q_sh_term_dn4 = assign14780_e14430_d_n4;
        locals.var_q_sh_term_dn6 = assign14780_e14430_d_n6;
        locals.var_q_sh_term_dn7 = assign14780_e14430_d_n7;
        locals.var_q_sh_term_dn8 = assign14780_e14430_d_n8;
        locals.var_q_sh_term_dn9 = assign14780_e14430_d_n9;

        let (assign14790_e14443, assign14790_e14443_d_n4, assign14790_e14443_d_n6, assign14790_e14443_d_n7, assign14790_e14443_d_n8, assign14790_e14443_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard584 == 0.0)) && (locals.var_guard585 == 0.0)) {
        let assign14790_e14441: f64 = (locals.var_q_sh_term).ln();
        (assign14790_e14441, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign14790_e14443;
        locals.var_q_ln_term_dn4 = assign14790_e14443_d_n4;
        locals.var_q_ln_term_dn6 = assign14790_e14443_d_n6;
        locals.var_q_ln_term_dn7 = assign14790_e14443_d_n7;
        locals.var_q_ln_term_dn8 = assign14790_e14443_d_n8;
        locals.var_q_ln_term_dn9 = assign14790_e14443_d_n9;

        let assign14800_e14446: f64 = (1.01 * locals.var_q_k1q1);
        let assign14800_e14448: f64 = (assign14800_e14446 + locals.var_q_qcoth);
        let assign14800_e14450: f64 = if assign14800_e14448 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard586 = assign14800_e14450;

        let (assign14810_e14460, assign14810_e14460_d_n4, assign14810_e14460_d_n6, assign14810_e14460_d_n7, assign14810_e14460_d_n8, assign14810_e14460_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard586 != 0.0)) {
        let assign14810_e14458: f64 = (locals.var_q_k1q1 + locals.var_q_qcoth);
        (assign14810_e14458, (locals.var_q_k1q1_dn4 + locals.var_q_qcoth_dn4), (locals.var_q_k1q1_dn6 + locals.var_q_qcoth_dn6), (locals.var_q_k1q1_dn7 + locals.var_q_qcoth_dn7), (locals.var_q_k1q1_dn8 + locals.var_q_qcoth_dn8), (locals.var_q_k1q1_dn9 + locals.var_q_qcoth_dn9),)
    } else {
        (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9,)
    }
};
        locals.var_q_expnum = assign14810_e14460;
        locals.var_q_expnum_dn4 = assign14810_e14460_d_n4;
        locals.var_q_expnum_dn6 = assign14810_e14460_d_n6;
        locals.var_q_expnum_dn7 = assign14810_e14460_d_n7;
        locals.var_q_expnum_dn8 = assign14810_e14460_d_n8;
        locals.var_q_expnum_dn9 = assign14810_e14460_d_n9;

        let (assign14820_e14470, assign14820_e14470_d_n4, assign14820_e14470_d_n6, assign14820_e14470_d_n7, assign14820_e14470_d_n8, assign14820_e14470_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard586 != 0.0)) {
        let assign14820_e14468: f64 = (locals.var_k1 + locals.var_q_d1_qcoth);
        (assign14820_e14468, (locals.var_k1_dn4 + locals.var_q_d1_qcoth_dn4), (locals.var_k1_dn6 + locals.var_q_d1_qcoth_dn6), (locals.var_k1_dn7 + locals.var_q_d1_qcoth_dn7), (locals.var_k1_dn8 + locals.var_q_d1_qcoth_dn8), (locals.var_k1_dn9 + locals.var_q_d1_qcoth_dn9),)
    } else {
        (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9,)
    }
};
        locals.var_q_d1_expnum = assign14820_e14470;
        locals.var_q_d1_expnum_dn4 = assign14820_e14470_d_n4;
        locals.var_q_d1_expnum_dn6 = assign14820_e14470_d_n6;
        locals.var_q_d1_expnum_dn7 = assign14820_e14470_d_n7;
        locals.var_q_d1_expnum_dn8 = assign14820_e14470_d_n8;
        locals.var_q_d1_expnum_dn9 = assign14820_e14470_d_n9;

        let (assign14830_e14478, assign14830_e14478_d_n4, assign14830_e14478_d_n6, assign14830_e14478_d_n7, assign14830_e14478_d_n8, assign14830_e14478_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard586 != 0.0)) {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    } else {
        (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9,)
    }
};
        locals.var_q_d2_expnum = assign14830_e14478;
        locals.var_q_d2_expnum_dn4 = assign14830_e14478_d_n4;
        locals.var_q_d2_expnum_dn6 = assign14830_e14478_d_n6;
        locals.var_q_d2_expnum_dn7 = assign14830_e14478_d_n7;
        locals.var_q_d2_expnum_dn8 = assign14830_e14478_d_n8;
        locals.var_q_d2_expnum_dn9 = assign14830_e14478_d_n9;

        let (assign14840_e14491, assign14840_e14491_d_n4, assign14840_e14491_d_n6, assign14840_e14491_d_n7, assign14840_e14491_d_n8, assign14840_e14491_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard586 == 0.0)) {
        let assign14840_e14488: f64 = (locals.var_q_k1q1 - locals.var_q_qcoth);
        let assign14840_e14489: f64 = (1.0 / assign14840_e14488);
        (assign14840_e14489, (-((locals.var_q_k1q1_dn4 - locals.var_q_qcoth_dn4) / (assign14840_e14488 * assign14840_e14488))), (-((locals.var_q_k1q1_dn6 - locals.var_q_qcoth_dn6) / (assign14840_e14488 * assign14840_e14488))), (-((locals.var_q_k1q1_dn7 - locals.var_q_qcoth_dn7) / (assign14840_e14488 * assign14840_e14488))), (-((locals.var_q_k1q1_dn8 - locals.var_q_qcoth_dn8) / (assign14840_e14488 * assign14840_e14488))), (-((locals.var_q_k1q1_dn9 - locals.var_q_qcoth_dn9) / (assign14840_e14488 * assign14840_e14488))),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign14840_e14491;
        locals.var_q_temp2_dn4 = assign14840_e14491_d_n4;
        locals.var_q_temp2_dn6 = assign14840_e14491_d_n6;
        locals.var_q_temp2_dn7 = assign14840_e14491_d_n7;
        locals.var_q_temp2_dn8 = assign14840_e14491_d_n8;
        locals.var_q_temp2_dn9 = assign14840_e14491_d_n9;

        let (assign14850_e14502, assign14850_e14502_d_n4, assign14850_e14502_d_n6, assign14850_e14502_d_n7, assign14850_e14502_d_n8, assign14850_e14502_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard586 == 0.0)) {
        let assign14850_e14500: f64 = (locals.var_q_d1_qcoth - locals.var_k1);
        (assign14850_e14500, (locals.var_q_d1_qcoth_dn4 - locals.var_k1_dn4), (locals.var_q_d1_qcoth_dn6 - locals.var_k1_dn6), (locals.var_q_d1_qcoth_dn7 - locals.var_k1_dn7), (locals.var_q_d1_qcoth_dn8 - locals.var_k1_dn8), (locals.var_q_d1_qcoth_dn9 - locals.var_k1_dn9),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign14850_e14502;
        locals.var_q_temp3_dn4 = assign14850_e14502_d_n4;
        locals.var_q_temp3_dn6 = assign14850_e14502_d_n6;
        locals.var_q_temp3_dn7 = assign14850_e14502_d_n7;
        locals.var_q_temp3_dn8 = assign14850_e14502_d_n8;
        locals.var_q_temp3_dn9 = assign14850_e14502_d_n9;

        let (assign14860_e14515, assign14860_e14515_d_n4, assign14860_e14515_d_n6, assign14860_e14515_d_n7, assign14860_e14515_d_n8, assign14860_e14515_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard586 == 0.0)) {
        let assign14860_e14511: f64 = (locals.var_q_aexp - locals.var_q_sh_term);
        let assign14860_e14513: f64 = (assign14860_e14511 * locals.var_q_temp2);
        (assign14860_e14513, (((locals.var_q_aexp_dn4 - locals.var_q_sh_term_dn4) * locals.var_q_temp2) + (assign14860_e14511 * locals.var_q_temp2_dn4)), (((locals.var_q_aexp_dn6 - locals.var_q_sh_term_dn6) * locals.var_q_temp2) + (assign14860_e14511 * locals.var_q_temp2_dn6)), (((locals.var_q_aexp_dn7 - locals.var_q_sh_term_dn7) * locals.var_q_temp2) + (assign14860_e14511 * locals.var_q_temp2_dn7)), (((locals.var_q_aexp_dn8 - locals.var_q_sh_term_dn8) * locals.var_q_temp2) + (assign14860_e14511 * locals.var_q_temp2_dn8)), (((locals.var_q_aexp_dn9 - locals.var_q_sh_term_dn9) * locals.var_q_temp2) + (assign14860_e14511 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9,)
    }
};
        locals.var_q_expnum = assign14860_e14515;
        locals.var_q_expnum_dn4 = assign14860_e14515_d_n4;
        locals.var_q_expnum_dn6 = assign14860_e14515_d_n6;
        locals.var_q_expnum_dn7 = assign14860_e14515_d_n7;
        locals.var_q_expnum_dn8 = assign14860_e14515_d_n8;
        locals.var_q_expnum_dn9 = assign14860_e14515_d_n9;

        let (assign14870_e14534, assign14870_e14534_d_n4, assign14870_e14534_d_n6, assign14870_e14534_d_n7, assign14870_e14534_d_n8, assign14870_e14534_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard586 == 0.0)) {
        let assign14870_e14524: f64 = (locals.var_q_temp3 * locals.var_q_expnum);
        let assign14870_e14526: f64 = (assign14870_e14524 - locals.var_q_aexp);
        let assign14870_e14529: f64 = (locals.var_q_d1_ln * locals.var_q_sh_term);
        let assign14870_e14530: f64 = (assign14870_e14526 - assign14870_e14529);
        let assign14870_e14532: f64 = (assign14870_e14530 * locals.var_q_temp2);
        (assign14870_e14532, ((((((locals.var_q_temp3_dn4 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4) - ((locals.var_q_d1_ln_dn4 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign14870_e14530 * locals.var_q_temp2_dn4)), ((((((locals.var_q_temp3_dn6 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6) - ((locals.var_q_d1_ln_dn6 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign14870_e14530 * locals.var_q_temp2_dn6)), ((((((locals.var_q_temp3_dn7 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7) - ((locals.var_q_d1_ln_dn7 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign14870_e14530 * locals.var_q_temp2_dn7)), ((((((locals.var_q_temp3_dn8 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8) - ((locals.var_q_d1_ln_dn8 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign14870_e14530 * locals.var_q_temp2_dn8)), ((((((locals.var_q_temp3_dn9 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9) - ((locals.var_q_d1_ln_dn9 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign14870_e14530 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9,)
    }
};
        locals.var_q_d1_expnum = assign14870_e14534;
        locals.var_q_d1_expnum_dn4 = assign14870_e14534_d_n4;
        locals.var_q_d1_expnum_dn6 = assign14870_e14534_d_n6;
        locals.var_q_d1_expnum_dn7 = assign14870_e14534_d_n7;
        locals.var_q_d1_expnum_dn8 = assign14870_e14534_d_n8;
        locals.var_q_d1_expnum_dn9 = assign14870_e14534_d_n9;

        let (assign14880_e14563, assign14880_e14563_d_n4, assign14880_e14563_d_n6, assign14880_e14563_d_n7, assign14880_e14563_d_n8, assign14880_e14563_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard586 == 0.0)) {
        let assign14880_e14543: f64 = (locals.var_q_d2_qcoth * locals.var_q_expnum);
        let assign14880_e14546: f64 = (2.0 * locals.var_q_temp3);
        let assign14880_e14548: f64 = (assign14880_e14546 * locals.var_q_d1_expnum);
        let assign14880_e14549: f64 = (assign14880_e14543 + assign14880_e14548);
        let assign14880_e14551: f64 = (assign14880_e14549 + locals.var_q_aexp);
        let assign14880_e14555: f64 = (locals.var_q_d1_ln * locals.var_q_d1_ln);
        let assign14880_e14556: f64 = (locals.var_q_d2_ln + assign14880_e14555);
        let assign14880_e14558: f64 = (assign14880_e14556 * locals.var_q_sh_term);
        let assign14880_e14559: f64 = (assign14880_e14551 - assign14880_e14558);
        let assign14880_e14561: f64 = (assign14880_e14559 * locals.var_q_temp2);
        (assign14880_e14561, (((((((locals.var_q_d2_qcoth_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_temp3_dn4) * locals.var_q_d1_expnum) + (assign14880_e14546 * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4) - (((locals.var_q_d2_ln_dn4 + ((locals.var_q_d1_ln_dn4 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn4))) * locals.var_q_sh_term) + (assign14880_e14556 * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign14880_e14559 * locals.var_q_temp2_dn4)), (((((((locals.var_q_d2_qcoth_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_temp3_dn6) * locals.var_q_d1_expnum) + (assign14880_e14546 * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6) - (((locals.var_q_d2_ln_dn6 + ((locals.var_q_d1_ln_dn6 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn6))) * locals.var_q_sh_term) + (assign14880_e14556 * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign14880_e14559 * locals.var_q_temp2_dn6)), (((((((locals.var_q_d2_qcoth_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_temp3_dn7) * locals.var_q_d1_expnum) + (assign14880_e14546 * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7) - (((locals.var_q_d2_ln_dn7 + ((locals.var_q_d1_ln_dn7 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn7))) * locals.var_q_sh_term) + (assign14880_e14556 * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign14880_e14559 * locals.var_q_temp2_dn7)), (((((((locals.var_q_d2_qcoth_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_temp3_dn8) * locals.var_q_d1_expnum) + (assign14880_e14546 * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8) - (((locals.var_q_d2_ln_dn8 + ((locals.var_q_d1_ln_dn8 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn8))) * locals.var_q_sh_term) + (assign14880_e14556 * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign14880_e14559 * locals.var_q_temp2_dn8)), (((((((locals.var_q_d2_qcoth_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_temp3_dn9) * locals.var_q_d1_expnum) + (assign14880_e14546 * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9) - (((locals.var_q_d2_ln_dn9 + ((locals.var_q_d1_ln_dn9 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn9))) * locals.var_q_sh_term) + (assign14880_e14556 * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign14880_e14559 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9,)
    }
};
        locals.var_q_d2_expnum = assign14880_e14563;
        locals.var_q_d2_expnum_dn4 = assign14880_e14563_d_n4;
        locals.var_q_d2_expnum_dn6 = assign14880_e14563_d_n6;
        locals.var_q_d2_expnum_dn7 = assign14880_e14563_d_n7;
        locals.var_q_d2_expnum_dn8 = assign14880_e14563_d_n8;
        locals.var_q_d2_expnum_dn9 = assign14880_e14563_d_n9;

        let assign14890_e14566: f64 = if locals.var_q_expnum > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard587 = assign14890_e14566;

        let (assign14900_e14575, assign14900_e14575_d_n4, assign14900_e14575_d_n6, assign14900_e14575_d_n7, assign14900_e14575_d_n8, assign14900_e14575_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard587 != 0.0)) {
        let assign14900_e14573: f64 = (locals.var_q_expnum).ln();
        (assign14900_e14573, (locals.var_q_expnum_dn4 / locals.var_q_expnum), (locals.var_q_expnum_dn6 / locals.var_q_expnum), (locals.var_q_expnum_dn7 / locals.var_q_expnum), (locals.var_q_expnum_dn8 / locals.var_q_expnum), (locals.var_q_expnum_dn9 / locals.var_q_expnum),)
    } else {
        (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9,)
    }
};
        locals.var_q_lnexpnum = assign14900_e14575;
        locals.var_q_lnexpnum_dn4 = assign14900_e14575_d_n4;
        locals.var_q_lnexpnum_dn6 = assign14900_e14575_d_n6;
        locals.var_q_lnexpnum_dn7 = assign14900_e14575_d_n7;
        locals.var_q_lnexpnum_dn8 = assign14900_e14575_d_n8;
        locals.var_q_lnexpnum_dn9 = assign14900_e14575_d_n9;

        let (assign14910_e14585, assign14910_e14585_d_n4, assign14910_e14585_d_n6, assign14910_e14585_d_n7, assign14910_e14585_d_n8, assign14910_e14585_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard587 != 0.0)) {
        let assign14910_e14583: f64 = (1.0 / locals.var_q_expnum);
        (assign14910_e14583, (-(locals.var_q_expnum_dn4 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn6 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn7 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn8 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn9 / (locals.var_q_expnum * locals.var_q_expnum))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign14910_e14585;
        locals.var_q_temp1_dn4 = assign14910_e14585_d_n4;
        locals.var_q_temp1_dn6 = assign14910_e14585_d_n6;
        locals.var_q_temp1_dn7 = assign14910_e14585_d_n7;
        locals.var_q_temp1_dn8 = assign14910_e14585_d_n8;
        locals.var_q_temp1_dn9 = assign14910_e14585_d_n9;

        let (assign14920_e14595, assign14920_e14595_d_n4, assign14920_e14595_d_n6, assign14920_e14595_d_n7, assign14920_e14595_d_n8, assign14920_e14595_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard587 != 0.0)) {
        let assign14920_e14593: f64 = (locals.var_q_d1_expnum * locals.var_q_temp1);
        (assign14920_e14593, ((locals.var_q_d1_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn4)), ((locals.var_q_d1_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn6)), ((locals.var_q_d1_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn7)), ((locals.var_q_d1_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn8)), ((locals.var_q_d1_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9,)
    }
};
        locals.var_q_d1_lnexpnum = assign14920_e14595;
        locals.var_q_d1_lnexpnum_dn4 = assign14920_e14595_d_n4;
        locals.var_q_d1_lnexpnum_dn6 = assign14920_e14595_d_n6;
        locals.var_q_d1_lnexpnum_dn7 = assign14920_e14595_d_n7;
        locals.var_q_d1_lnexpnum_dn8 = assign14920_e14595_d_n8;
        locals.var_q_d1_lnexpnum_dn9 = assign14920_e14595_d_n9;

        let (assign14930_e14609, assign14930_e14609_d_n4, assign14930_e14609_d_n6, assign14930_e14609_d_n7, assign14930_e14609_d_n8, assign14930_e14609_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard587 != 0.0)) {
        let assign14930_e14603: f64 = (locals.var_q_d2_expnum * locals.var_q_temp1);
        let assign14930_e14606: f64 = (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum);
        let assign14930_e14607: f64 = (assign14930_e14603 - assign14930_e14606);
        (assign14930_e14607, (((locals.var_q_d2_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn4)) - ((locals.var_q_d1_lnexpnum_dn4 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn4))), (((locals.var_q_d2_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn6)) - ((locals.var_q_d1_lnexpnum_dn6 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn6))), (((locals.var_q_d2_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn7)) - ((locals.var_q_d1_lnexpnum_dn7 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn7))), (((locals.var_q_d2_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn8)) - ((locals.var_q_d1_lnexpnum_dn8 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn8))), (((locals.var_q_d2_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn9)) - ((locals.var_q_d1_lnexpnum_dn9 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn9))),)
    } else {
        (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9,)
    }
};
        locals.var_q_d2_lnexpnum = assign14930_e14609;
        locals.var_q_d2_lnexpnum_dn4 = assign14930_e14609_d_n4;
        locals.var_q_d2_lnexpnum_dn6 = assign14930_e14609_d_n6;
        locals.var_q_d2_lnexpnum_dn7 = assign14930_e14609_d_n7;
        locals.var_q_d2_lnexpnum_dn8 = assign14930_e14609_d_n8;
        locals.var_q_d2_lnexpnum_dn9 = assign14930_e14609_d_n9;

        let (assign14940_e14624, assign14940_e14624_d_n4, assign14940_e14624_d_n6, assign14940_e14624_d_n7, assign14940_e14624_d_n8, assign14940_e14624_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard587 == 0.0)) {
        let assign14940_e14618: f64 = (locals.var_q_k1q1 + 0.6931471805599);
        let assign14940_e14620: f64 = (-locals.var_q_k1q1);
        let assign14940_e14621: f64 = (assign14940_e14620).ln();
        let assign14940_e14622: f64 = (assign14940_e14618 + assign14940_e14621);
        (assign14940_e14622, (locals.var_q_k1q1_dn4 + ((-locals.var_q_k1q1_dn4) / assign14940_e14620)), (locals.var_q_k1q1_dn6 + ((-locals.var_q_k1q1_dn6) / assign14940_e14620)), (locals.var_q_k1q1_dn7 + ((-locals.var_q_k1q1_dn7) / assign14940_e14620)), (locals.var_q_k1q1_dn8 + ((-locals.var_q_k1q1_dn8) / assign14940_e14620)), (locals.var_q_k1q1_dn9 + ((-locals.var_q_k1q1_dn9) / assign14940_e14620)),)
    } else {
        (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9,)
    }
};
        locals.var_q_lnexpnum = assign14940_e14624;
        locals.var_q_lnexpnum_dn4 = assign14940_e14624_d_n4;
        locals.var_q_lnexpnum_dn6 = assign14940_e14624_d_n6;
        locals.var_q_lnexpnum_dn7 = assign14940_e14624_d_n7;
        locals.var_q_lnexpnum_dn8 = assign14940_e14624_d_n8;
        locals.var_q_lnexpnum_dn9 = assign14940_e14624_d_n9;

        let (assign14950_e14635, assign14950_e14635_d_n4, assign14950_e14635_d_n6, assign14950_e14635_d_n7, assign14950_e14635_d_n8, assign14950_e14635_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard587 == 0.0)) {
        let assign14950_e14633: f64 = (1.0 / locals.var_q1s);
        (assign14950_e14633, (-(locals.var_q1s_dn4 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn6 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn7 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn8 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn9 / (locals.var_q1s * locals.var_q1s))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign14950_e14635;
        locals.var_q_temp1_dn4 = assign14950_e14635_d_n4;
        locals.var_q_temp1_dn6 = assign14950_e14635_d_n6;
        locals.var_q_temp1_dn7 = assign14950_e14635_d_n7;
        locals.var_q_temp1_dn8 = assign14950_e14635_d_n8;
        locals.var_q_temp1_dn9 = assign14950_e14635_d_n9;

        let (assign14960_e14646, assign14960_e14646_d_n4, assign14960_e14646_d_n6, assign14960_e14646_d_n7, assign14960_e14646_d_n8, assign14960_e14646_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard587 == 0.0)) {
        let assign14960_e14644: f64 = (locals.var_k1 + locals.var_q_temp1);
        (assign14960_e14644, (locals.var_k1_dn4 + locals.var_q_temp1_dn4), (locals.var_k1_dn6 + locals.var_q_temp1_dn6), (locals.var_k1_dn7 + locals.var_q_temp1_dn7), (locals.var_k1_dn8 + locals.var_q_temp1_dn8), (locals.var_k1_dn9 + locals.var_q_temp1_dn9),)
    } else {
        (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9,)
    }
};
        locals.var_q_d1_lnexpnum = assign14960_e14646;
        locals.var_q_d1_lnexpnum_dn4 = assign14960_e14646_d_n4;
        locals.var_q_d1_lnexpnum_dn6 = assign14960_e14646_d_n6;
        locals.var_q_d1_lnexpnum_dn7 = assign14960_e14646_d_n7;
        locals.var_q_d1_lnexpnum_dn8 = assign14960_e14646_d_n8;
        locals.var_q_d1_lnexpnum_dn9 = assign14960_e14646_d_n9;

        let (assign14970_e14658, assign14970_e14658_d_n4, assign14970_e14658_d_n6, assign14970_e14658_d_n7, assign14970_e14658_d_n8, assign14970_e14658_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard587 == 0.0)) {
        let assign14970_e14654: f64 = (-locals.var_q_temp1);
        let assign14970_e14656: f64 = (assign14970_e14654 * locals.var_q_temp1);
        (assign14970_e14656, (((-locals.var_q_temp1_dn4) * locals.var_q_temp1) + (assign14970_e14654 * locals.var_q_temp1_dn4)), (((-locals.var_q_temp1_dn6) * locals.var_q_temp1) + (assign14970_e14654 * locals.var_q_temp1_dn6)), (((-locals.var_q_temp1_dn7) * locals.var_q_temp1) + (assign14970_e14654 * locals.var_q_temp1_dn7)), (((-locals.var_q_temp1_dn8) * locals.var_q_temp1) + (assign14970_e14654 * locals.var_q_temp1_dn8)), (((-locals.var_q_temp1_dn9) * locals.var_q_temp1) + (assign14970_e14654 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9,)
    }
};
        locals.var_q_d2_lnexpnum = assign14970_e14658;
        locals.var_q_d2_lnexpnum_dn4 = assign14970_e14658_d_n4;
        locals.var_q_d2_lnexpnum_dn6 = assign14970_e14658_d_n6;
        locals.var_q_d2_lnexpnum_dn7 = assign14970_e14658_d_n7;
        locals.var_q_d2_lnexpnum_dn8 = assign14970_e14658_d_n8;
        locals.var_q_d2_lnexpnum_dn9 = assign14970_e14658_d_n9;

        let (assign14980_e14674, assign14980_e14674_d_n4, assign14980_e14674_d_n6, assign14980_e14674_d_n7, assign14980_e14674_d_n8, assign14980_e14674_d_n9,) = {
    if ((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) {
        let assign14980_e14664: f64 = (locals.var_xg2x - locals.var_xg1x);
        let assign14980_e14666: f64 = (assign14980_e14664 + locals.var_q1s);
        let assign14980_e14669: f64 = (2.0 * locals.var_q_lnexpnum);
        let assign14980_e14670: f64 = (assign14980_e14666 + assign14980_e14669);
        let assign14980_e14672: f64 = (assign14980_e14670 - locals.var_q_ln_term);
        (assign14980_e14672, ((((locals.var_xg2x_dn4 - locals.var_xg1x_dn4) + locals.var_q1s_dn4) + (2.0 * locals.var_q_lnexpnum_dn4)) - locals.var_q_ln_term_dn4), ((((locals.var_xg2x_dn6 - locals.var_xg1x_dn6) + locals.var_q1s_dn6) + (2.0 * locals.var_q_lnexpnum_dn6)) - locals.var_q_ln_term_dn6), ((((locals.var_xg2x_dn7 - locals.var_xg1x_dn7) + locals.var_q1s_dn7) + (2.0 * locals.var_q_lnexpnum_dn7)) - locals.var_q_ln_term_dn7), ((((locals.var_xg2x_dn8 - locals.var_xg1x_dn8) + locals.var_q1s_dn8) + (2.0 * locals.var_q_lnexpnum_dn8)) - locals.var_q_ln_term_dn8), ((((locals.var_xg2x_dn9 - locals.var_xg1x_dn9) + locals.var_q1s_dn9) + (2.0 * locals.var_q_lnexpnum_dn9)) - locals.var_q_ln_term_dn9),)
    } else {
        (locals.var_q_q2_int, locals.var_q_q2_int_dn4, locals.var_q_q2_int_dn6, locals.var_q_q2_int_dn7, locals.var_q_q2_int_dn8, locals.var_q_q2_int_dn9,)
    }
};
        locals.var_q_q2_int = assign14980_e14674;
        locals.var_q_q2_int_dn4 = assign14980_e14674_d_n4;
        locals.var_q_q2_int_dn6 = assign14980_e14674_d_n6;
        locals.var_q_q2_int_dn7 = assign14980_e14674_d_n7;
        locals.var_q_q2_int_dn8 = assign14980_e14674_d_n8;
        locals.var_q_q2_int_dn9 = assign14980_e14674_d_n9;

        let (assign14990_e14686, assign14990_e14686_d_n4, assign14990_e14686_d_n6, assign14990_e14686_d_n7, assign14990_e14686_d_n8, assign14990_e14686_d_n9,) = {
    if ((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) {
        let assign14990_e14681: f64 = (2.0 * locals.var_q_d1_lnexpnum);
        let assign14990_e14682: f64 = (1.0 + assign14990_e14681);
        let assign14990_e14684: f64 = (assign14990_e14682 - locals.var_q_d1_ln);
        (assign14990_e14684, ((2.0 * locals.var_q_d1_lnexpnum_dn4) - locals.var_q_d1_ln_dn4), ((2.0 * locals.var_q_d1_lnexpnum_dn6) - locals.var_q_d1_ln_dn6), ((2.0 * locals.var_q_d1_lnexpnum_dn7) - locals.var_q_d1_ln_dn7), ((2.0 * locals.var_q_d1_lnexpnum_dn8) - locals.var_q_d1_ln_dn8), ((2.0 * locals.var_q_d1_lnexpnum_dn9) - locals.var_q_d1_ln_dn9),)
    } else {
        (locals.var_q_d1_q2, locals.var_q_d1_q2_dn4, locals.var_q_d1_q2_dn6, locals.var_q_d1_q2_dn7, locals.var_q_d1_q2_dn8, locals.var_q_d1_q2_dn9,)
    }
};
        locals.var_q_d1_q2 = assign14990_e14686;
        locals.var_q_d1_q2_dn4 = assign14990_e14686_d_n4;
        locals.var_q_d1_q2_dn6 = assign14990_e14686_d_n6;
        locals.var_q_d1_q2_dn7 = assign14990_e14686_d_n7;
        locals.var_q_d1_q2_dn8 = assign14990_e14686_d_n8;
        locals.var_q_d1_q2_dn9 = assign14990_e14686_d_n9;

        let (assign15000_e14696, assign15000_e14696_d_n4, assign15000_e14696_d_n6, assign15000_e14696_d_n7, assign15000_e14696_d_n8, assign15000_e14696_d_n9,) = {
    if ((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) {
        let assign15000_e14692: f64 = (2.0 * locals.var_q_d2_lnexpnum);
        let assign15000_e14694: f64 = (assign15000_e14692 - locals.var_q_d2_ln);
        (assign15000_e14694, ((2.0 * locals.var_q_d2_lnexpnum_dn4) - locals.var_q_d2_ln_dn4), ((2.0 * locals.var_q_d2_lnexpnum_dn6) - locals.var_q_d2_ln_dn6), ((2.0 * locals.var_q_d2_lnexpnum_dn7) - locals.var_q_d2_ln_dn7), ((2.0 * locals.var_q_d2_lnexpnum_dn8) - locals.var_q_d2_ln_dn8), ((2.0 * locals.var_q_d2_lnexpnum_dn9) - locals.var_q_d2_ln_dn9),)
    } else {
        (locals.var_q_d2_q2, locals.var_q_d2_q2_dn4, locals.var_q_d2_q2_dn6, locals.var_q_d2_q2_dn7, locals.var_q_d2_q2_dn8, locals.var_q_d2_q2_dn9,)
    }
};
        locals.var_q_d2_q2 = assign15000_e14696;
        locals.var_q_d2_q2_dn4 = assign15000_e14696_d_n4;
        locals.var_q_d2_q2_dn6 = assign15000_e14696_d_n6;
        locals.var_q_d2_q2_dn7 = assign15000_e14696_d_n7;
        locals.var_q_d2_q2_dn8 = assign15000_e14696_d_n8;
        locals.var_q_d2_q2_dn9 = assign15000_e14696_d_n9;

        let (assign15010_e14706, assign15010_e14706_d_n4, assign15010_e14706_d_n6, assign15010_e14706_d_n7, assign15010_e14706_d_n8, assign15010_e14706_d_n9,) = {
    if ((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) {
        let assign15010_e14703: f64 = (locals.var_k2 * locals.var_q_q2_int);
        let assign15010_e14704: f64 = (locals.var_q_k1q1 + assign15010_e14703);
        (assign15010_e14704, (locals.var_q_k1q1_dn4 + ((locals.var_k2_dn4 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn4))), (locals.var_q_k1q1_dn6 + ((locals.var_k2_dn6 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn6))), (locals.var_q_k1q1_dn7 + ((locals.var_k2_dn7 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn7))), (locals.var_q_k1q1_dn8 + ((locals.var_k2_dn8 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn8))), (locals.var_q_k1q1_dn9 + ((locals.var_k2_dn9 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn9))),)
    } else {
        (locals.var_q_qi_int, locals.var_q_qi_int_dn4, locals.var_q_qi_int_dn6, locals.var_q_qi_int_dn7, locals.var_q_qi_int_dn8, locals.var_q_qi_int_dn9,)
    }
};
        locals.var_q_qi_int = assign15010_e14706;
        locals.var_q_qi_int_dn4 = assign15010_e14706_d_n4;
        locals.var_q_qi_int_dn6 = assign15010_e14706_d_n6;
        locals.var_q_qi_int_dn7 = assign15010_e14706_d_n7;
        locals.var_q_qi_int_dn8 = assign15010_e14706_d_n8;
        locals.var_q_qi_int_dn9 = assign15010_e14706_d_n9;

        let (assign15020_e14716, assign15020_e14716_d_n4, assign15020_e14716_d_n6, assign15020_e14716_d_n7, assign15020_e14716_d_n8, assign15020_e14716_d_n9,) = {
    if ((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) {
        let assign15020_e14713: f64 = (locals.var_k2 * locals.var_q_d1_q2);
        let assign15020_e14714: f64 = (locals.var_k1 + assign15020_e14713);
        (assign15020_e14714, (locals.var_k1_dn4 + ((locals.var_k2_dn4 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn4))), (locals.var_k1_dn6 + ((locals.var_k2_dn6 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn6))), (locals.var_k1_dn7 + ((locals.var_k2_dn7 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn7))), (locals.var_k1_dn8 + ((locals.var_k2_dn8 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn8))), (locals.var_k1_dn9 + ((locals.var_k2_dn9 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn9))),)
    } else {
        (locals.var_q_d1_qi, locals.var_q_d1_qi_dn4, locals.var_q_d1_qi_dn6, locals.var_q_d1_qi_dn7, locals.var_q_d1_qi_dn8, locals.var_q_d1_qi_dn9,)
    }
};
        locals.var_q_d1_qi = assign15020_e14716;
        locals.var_q_d1_qi_dn4 = assign15020_e14716_d_n4;
        locals.var_q_d1_qi_dn6 = assign15020_e14716_d_n6;
        locals.var_q_d1_qi_dn7 = assign15020_e14716_d_n7;
        locals.var_q_d1_qi_dn8 = assign15020_e14716_d_n8;
        locals.var_q_d1_qi_dn9 = assign15020_e14716_d_n9;

        let (assign15030_e14724, assign15030_e14724_d_n4, assign15030_e14724_d_n6, assign15030_e14724_d_n7, assign15030_e14724_d_n8, assign15030_e14724_d_n9,) = {
    if ((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) {
        let assign15030_e14722: f64 = (locals.var_k2 * locals.var_q_d2_q2);
        (assign15030_e14722, ((locals.var_k2_dn4 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn4)), ((locals.var_k2_dn6 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn6)), ((locals.var_k2_dn7 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn7)), ((locals.var_k2_dn8 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn8)), ((locals.var_k2_dn9 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn9)),)
    } else {
        (locals.var_q_d2_qi, locals.var_q_d2_qi_dn4, locals.var_q_d2_qi_dn6, locals.var_q_d2_qi_dn7, locals.var_q_d2_qi_dn8, locals.var_q_d2_qi_dn9,)
    }
};
        locals.var_q_d2_qi = assign15030_e14724;
        locals.var_q_d2_qi_dn4 = assign15030_e14724_d_n4;
        locals.var_q_d2_qi_dn6 = assign15030_e14724_d_n6;
        locals.var_q_d2_qi_dn7 = assign15030_e14724_d_n7;
        locals.var_q_d2_qi_dn8 = assign15030_e14724_d_n8;
        locals.var_q_d2_qi_dn9 = assign15030_e14724_d_n9;

        let (assign15040_e14734, assign15040_e14734_d_n4, assign15040_e14734_d_n6, assign15040_e14734_d_n7, assign15040_e14734_d_n8, assign15040_e14734_d_n9,) = {
    if ((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) {
        let assign15040_e14730: f64 = (locals.var_q_qi_int * locals.var_q_expnum);
        let assign15040_e14732: f64 = (assign15040_e14730 - locals.var_q_aexp);
        (assign15040_e14732, (((locals.var_q_qi_int_dn4 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4), (((locals.var_q_qi_int_dn6 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6), (((locals.var_q_qi_int_dn7 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7), (((locals.var_q_qi_int_dn8 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8), (((locals.var_q_qi_int_dn9 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9),)
    } else {
        (locals.var_q_zero, locals.var_q_zero_dn4, locals.var_q_zero_dn6, locals.var_q_zero_dn7, locals.var_q_zero_dn8, locals.var_q_zero_dn9,)
    }
};
        locals.var_q_zero = assign15040_e14734;
        locals.var_q_zero_dn4 = assign15040_e14734_d_n4;
        locals.var_q_zero_dn6 = assign15040_e14734_d_n6;
        locals.var_q_zero_dn7 = assign15040_e14734_d_n7;
        locals.var_q_zero_dn8 = assign15040_e14734_d_n8;
        locals.var_q_zero_dn9 = assign15040_e14734_d_n9;

        let (assign15050_e14748, assign15050_e14748_d_n4, assign15050_e14748_d_n6, assign15050_e14748_d_n7, assign15050_e14748_d_n8, assign15050_e14748_d_n9,) = {
    if ((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) {
        let assign15050_e14740: f64 = (locals.var_q_d1_qi * locals.var_q_expnum);
        let assign15050_e14743: f64 = (locals.var_q_qi_int * locals.var_q_d1_expnum);
        let assign15050_e14744: f64 = (assign15050_e14740 + assign15050_e14743);
        let assign15050_e14746: f64 = (assign15050_e14744 + locals.var_q_aexp);
        (assign15050_e14746, ((((locals.var_q_d1_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn4)) + ((locals.var_q_qi_int_dn4 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4), ((((locals.var_q_d1_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn6)) + ((locals.var_q_qi_int_dn6 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6), ((((locals.var_q_d1_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn7)) + ((locals.var_q_qi_int_dn7 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7), ((((locals.var_q_d1_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn8)) + ((locals.var_q_qi_int_dn8 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8), ((((locals.var_q_d1_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn9)) + ((locals.var_q_qi_int_dn9 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9),)
    } else {
        (locals.var_q_d1_zero, locals.var_q_d1_zero_dn4, locals.var_q_d1_zero_dn6, locals.var_q_d1_zero_dn7, locals.var_q_d1_zero_dn8, locals.var_q_d1_zero_dn9,)
    }
};
        locals.var_q_d1_zero = assign15050_e14748;
        locals.var_q_d1_zero_dn4 = assign15050_e14748_d_n4;
        locals.var_q_d1_zero_dn6 = assign15050_e14748_d_n6;
        locals.var_q_d1_zero_dn7 = assign15050_e14748_d_n7;
        locals.var_q_d1_zero_dn8 = assign15050_e14748_d_n8;
        locals.var_q_d1_zero_dn9 = assign15050_e14748_d_n9;

        let (assign15060_e14768, assign15060_e14768_d_n4, assign15060_e14768_d_n6, assign15060_e14768_d_n7, assign15060_e14768_d_n8, assign15060_e14768_d_n9,) = {
    if ((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) {
        let assign15060_e14754: f64 = (locals.var_q_d2_qi * locals.var_q_expnum);
        let assign15060_e14757: f64 = (2.0 * locals.var_q_d1_qi);
        let assign15060_e14759: f64 = (assign15060_e14757 * locals.var_q_d1_expnum);
        let assign15060_e14760: f64 = (assign15060_e14754 + assign15060_e14759);
        let assign15060_e14763: f64 = (locals.var_q_qi_int * locals.var_q_d2_expnum);
        let assign15060_e14764: f64 = (assign15060_e14760 + assign15060_e14763);
        let assign15060_e14766: f64 = (assign15060_e14764 - locals.var_q_aexp);
        (assign15060_e14766, (((((locals.var_q_d2_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_d1_qi_dn4) * locals.var_q_d1_expnum) + (assign15060_e14757 * locals.var_q_d1_expnum_dn4))) + ((locals.var_q_qi_int_dn4 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn4))) - locals.var_q_aexp_dn4), (((((locals.var_q_d2_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_d1_qi_dn6) * locals.var_q_d1_expnum) + (assign15060_e14757 * locals.var_q_d1_expnum_dn6))) + ((locals.var_q_qi_int_dn6 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn6))) - locals.var_q_aexp_dn6), (((((locals.var_q_d2_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_d1_qi_dn7) * locals.var_q_d1_expnum) + (assign15060_e14757 * locals.var_q_d1_expnum_dn7))) + ((locals.var_q_qi_int_dn7 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn7))) - locals.var_q_aexp_dn7), (((((locals.var_q_d2_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_d1_qi_dn8) * locals.var_q_d1_expnum) + (assign15060_e14757 * locals.var_q_d1_expnum_dn8))) + ((locals.var_q_qi_int_dn8 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn8))) - locals.var_q_aexp_dn8), (((((locals.var_q_d2_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_d1_qi_dn9) * locals.var_q_d1_expnum) + (assign15060_e14757 * locals.var_q_d1_expnum_dn9))) + ((locals.var_q_qi_int_dn9 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn9))) - locals.var_q_aexp_dn9),)
    } else {
        (locals.var_q_d2_zero, locals.var_q_d2_zero_dn4, locals.var_q_d2_zero_dn6, locals.var_q_d2_zero_dn7, locals.var_q_d2_zero_dn8, locals.var_q_d2_zero_dn9,)
    }
};
        locals.var_q_d2_zero = assign15060_e14768;
        locals.var_q_d2_zero_dn4 = assign15060_e14768_d_n4;
        locals.var_q_d2_zero_dn6 = assign15060_e14768_d_n6;
        locals.var_q_d2_zero_dn7 = assign15060_e14768_d_n7;
        locals.var_q_d2_zero_dn8 = assign15060_e14768_d_n8;
        locals.var_q_d2_zero_dn9 = assign15060_e14768_d_n9;

        let (assign15070_e14782, assign15070_e14782_d_n4, assign15070_e14782_d_n6, assign15070_e14782_d_n7, assign15070_e14782_d_n8, assign15070_e14782_d_n9,) = {
    if ((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) {
        let assign15070_e14774: f64 = (locals.var_q_d1_zero * locals.var_q_d1_zero);
        let assign15070_e14777: f64 = (0.5 * locals.var_q_zero);
        let assign15070_e14779: f64 = (assign15070_e14777 * locals.var_q_d2_zero);
        let assign15070_e14780: f64 = (assign15070_e14774 - assign15070_e14779);
        (assign15070_e14780, (((locals.var_q_d1_zero_dn4 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn4)) - (((0.5 * locals.var_q_zero_dn4) * locals.var_q_d2_zero) + (assign15070_e14777 * locals.var_q_d2_zero_dn4))), (((locals.var_q_d1_zero_dn6 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn6)) - (((0.5 * locals.var_q_zero_dn6) * locals.var_q_d2_zero) + (assign15070_e14777 * locals.var_q_d2_zero_dn6))), (((locals.var_q_d1_zero_dn7 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn7)) - (((0.5 * locals.var_q_zero_dn7) * locals.var_q_d2_zero) + (assign15070_e14777 * locals.var_q_d2_zero_dn7))), (((locals.var_q_d1_zero_dn8 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn8)) - (((0.5 * locals.var_q_zero_dn8) * locals.var_q_d2_zero) + (assign15070_e14777 * locals.var_q_d2_zero_dn8))), (((locals.var_q_d1_zero_dn9 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn9)) - (((0.5 * locals.var_q_zero_dn9) * locals.var_q_d2_zero) + (assign15070_e14777 * locals.var_q_d2_zero_dn9))),)
    } else {
        (locals.var_q_temp, locals.var_q_temp_dn4, locals.var_q_temp_dn6, locals.var_q_temp_dn7, locals.var_q_temp_dn8, locals.var_q_temp_dn9,)
    }
};
        locals.var_q_temp = assign15070_e14782;
        locals.var_q_temp_dn4 = assign15070_e14782_d_n4;
        locals.var_q_temp_dn6 = assign15070_e14782_d_n6;
        locals.var_q_temp_dn7 = assign15070_e14782_d_n7;
        locals.var_q_temp_dn8 = assign15070_e14782_d_n8;
        locals.var_q_temp_dn9 = assign15070_e14782_d_n9;

        let (assign15080_e14799, assign15080_e14799_d_n4, assign15080_e14799_d_n6, assign15080_e14799_d_n7, assign15080_e14799_d_n8, assign15080_e14799_d_n9,) = {
    if ((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) {
        let assign15080_e14787: f64 = (-locals.var_q_zero);
        let assign15080_e14789: f64 = (assign15080_e14787 * locals.var_q_d1_zero);
        let assign15080_e14791: f64 = (assign15080_e14789 * locals.var_q_temp);
        let assign15080_e14794: f64 = (locals.var_q_temp * locals.var_q_temp);
        let assign15080_e14796: f64 = (assign15080_e14794 + 1e-200);
        let assign15080_e14797: f64 = (assign15080_e14791 / assign15080_e14796);
        (assign15080_e14797, ((((((((-locals.var_q_zero_dn4) * locals.var_q_d1_zero) + (assign15080_e14787 * locals.var_q_d1_zero_dn4)) * locals.var_q_temp) + (assign15080_e14789 * locals.var_q_temp_dn4)) * assign15080_e14796) - (assign15080_e14791 * ((locals.var_q_temp_dn4 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn4)))) / (assign15080_e14796 * assign15080_e14796)), ((((((((-locals.var_q_zero_dn6) * locals.var_q_d1_zero) + (assign15080_e14787 * locals.var_q_d1_zero_dn6)) * locals.var_q_temp) + (assign15080_e14789 * locals.var_q_temp_dn6)) * assign15080_e14796) - (assign15080_e14791 * ((locals.var_q_temp_dn6 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn6)))) / (assign15080_e14796 * assign15080_e14796)), ((((((((-locals.var_q_zero_dn7) * locals.var_q_d1_zero) + (assign15080_e14787 * locals.var_q_d1_zero_dn7)) * locals.var_q_temp) + (assign15080_e14789 * locals.var_q_temp_dn7)) * assign15080_e14796) - (assign15080_e14791 * ((locals.var_q_temp_dn7 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn7)))) / (assign15080_e14796 * assign15080_e14796)), ((((((((-locals.var_q_zero_dn8) * locals.var_q_d1_zero) + (assign15080_e14787 * locals.var_q_d1_zero_dn8)) * locals.var_q_temp) + (assign15080_e14789 * locals.var_q_temp_dn8)) * assign15080_e14796) - (assign15080_e14791 * ((locals.var_q_temp_dn8 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn8)))) / (assign15080_e14796 * assign15080_e14796)), ((((((((-locals.var_q_zero_dn9) * locals.var_q_d1_zero) + (assign15080_e14787 * locals.var_q_d1_zero_dn9)) * locals.var_q_temp) + (assign15080_e14789 * locals.var_q_temp_dn9)) * assign15080_e14796) - (assign15080_e14791 * ((locals.var_q_temp_dn9 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn9)))) / (assign15080_e14796 * assign15080_e14796)),)
    } else {
        (locals.var_q_eps2, locals.var_q_eps2_dn4, locals.var_q_eps2_dn6, locals.var_q_eps2_dn7, locals.var_q_eps2_dn8, locals.var_q_eps2_dn9,)
    }
};
        locals.var_q_eps2 = assign15080_e14799;
        locals.var_q_eps2_dn4 = assign15080_e14799_d_n4;
        locals.var_q_eps2_dn6 = assign15080_e14799_d_n6;
        locals.var_q_eps2_dn7 = assign15080_e14799_d_n7;
        locals.var_q_eps2_dn8 = assign15080_e14799_d_n8;
        locals.var_q_eps2_dn9 = assign15080_e14799_d_n9;

        let (assign15090_e14807, assign15090_e14807_d_n4, assign15090_e14807_d_n6, assign15090_e14807_d_n7, assign15090_e14807_d_n8, assign15090_e14807_d_n9,) = {
    if ((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) {
        let assign15090_e14805: f64 = (locals.var_q1s + locals.var_q_eps2);
        (assign15090_e14805, (locals.var_q1s_dn4 + locals.var_q_eps2_dn4), (locals.var_q1s_dn6 + locals.var_q_eps2_dn6), (locals.var_q1s_dn7 + locals.var_q_eps2_dn7), (locals.var_q1s_dn8 + locals.var_q_eps2_dn8), (locals.var_q1s_dn9 + locals.var_q_eps2_dn9),)
    } else {
        (locals.var_q1s, locals.var_q1s_dn4, locals.var_q1s_dn6, locals.var_q1s_dn7, locals.var_q1s_dn8, locals.var_q1s_dn9,)
    }
};
        locals.var_q1s = assign15090_e14807;
        locals.var_q1s_dn4 = assign15090_e14807_d_n4;
        locals.var_q1s_dn6 = assign15090_e14807_d_n6;
        locals.var_q1s_dn7 = assign15090_e14807_d_n7;
        locals.var_q1s_dn8 = assign15090_e14807_d_n8;
        locals.var_q1s_dn9 = assign15090_e14807_d_n9;

    }

    pub(super) fn stamp_transient_block_37(
        locals: &mut StampLocals,
    ) {
        let assign15100_e14810: f64 = (locals.var_k1 * locals.var_q1s);
        locals.var_k1q1s = assign15100_e14810;
        locals.var_k1q1s_dn4 = ((locals.var_k1_dn4 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn4));
        locals.var_k1q1s_dn6 = ((locals.var_k1_dn6 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn6));
        locals.var_k1q1s_dn7 = ((locals.var_k1_dn7 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn7));
        locals.var_k1q1s_dn8 = ((locals.var_k1_dn8 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn8));
        locals.var_k1q1s_dn9 = ((locals.var_k1_dn9 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn9));

        let assign15110_e14813: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign15110_e14815: f64 = assign15110_e14813;
        let assign15110_e14817: f64 = if assign15110_e14815 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard588 = assign15110_e14817;

        let (assign15120_e14826, assign15120_e14826_d_n4, assign15120_e14826_d_n6, assign15120_e14826_d_n7, assign15120_e14826_d_n8, assign15120_e14826_d_n9,) = {
    if (locals.var_guard588 != 0.0) {
        let assign15120_e14821: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign15120_e14823: f64 = assign15120_e14821;
        let assign15120_e14824: f64 = (assign15120_e14823).exp();
        (assign15120_e14824, (assign15120_e14824 * (locals.var_xg1x_dn4 - locals.var_q1s_dn4)), (assign15120_e14824 * (locals.var_xg1x_dn6 - locals.var_q1s_dn6)), (assign15120_e14824 * (locals.var_xg1x_dn7 - locals.var_q1s_dn7)), (assign15120_e14824 * (locals.var_xg1x_dn8 - locals.var_q1s_dn8)), (assign15120_e14824 * (locals.var_xg1x_dn9 - locals.var_q1s_dn9)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign15120_e14826;
        locals.var_q_temp1_dn4 = assign15120_e14826_d_n4;
        locals.var_q_temp1_dn6 = assign15120_e14826_d_n6;
        locals.var_q_temp1_dn7 = assign15120_e14826_d_n7;
        locals.var_q_temp1_dn8 = assign15120_e14826_d_n8;
        locals.var_q_temp1_dn9 = assign15120_e14826_d_n9;

        let (assign15130_e14865, assign15130_e14865_d_n4, assign15130_e14865_d_n6, assign15130_e14865_d_n7, assign15130_e14865_d_n8, assign15130_e14865_d_n9,) = {
    if (locals.var_guard588 == 0.0) {
        let assign15130_e14833: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign15130_e14835: f64 = assign15130_e14833;
        let assign15130_e14837: f64 = (assign15130_e14835 - 80.0);
        let assign15130_e14842: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign15130_e14844: f64 = assign15130_e14842;
        let assign15130_e14846: f64 = (assign15130_e14844 - 80.0);
        let assign15130_e14847: f64 = (0.5 * assign15130_e14846);
        let assign15130_e14851: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign15130_e14853: f64 = assign15130_e14851;
        let assign15130_e14855: f64 = (assign15130_e14853 - 80.0);
        let assign15130_e14857: f64 = (assign15130_e14855 * 0.3333333333333);
        let assign15130_e14858: f64 = (1.0 + assign15130_e14857);
        let assign15130_e14859: f64 = (assign15130_e14847 * assign15130_e14858);
        let assign15130_e14860: f64 = (1.0 + assign15130_e14859);
        let assign15130_e14861: f64 = (assign15130_e14837 * assign15130_e14860);
        let assign15130_e14862: f64 = (1.0 + assign15130_e14861);
        let assign15130_e14863: f64 = (5.54062e34 * assign15130_e14862);
        (assign15130_e14863, (5.54062e34 * (((locals.var_xg1x_dn4 - locals.var_q1s_dn4) * assign15130_e14860) + (assign15130_e14837 * (((0.5 * (locals.var_xg1x_dn4 - locals.var_q1s_dn4)) * assign15130_e14858) + (assign15130_e14847 * ((locals.var_xg1x_dn4 - locals.var_q1s_dn4) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn6 - locals.var_q1s_dn6) * assign15130_e14860) + (assign15130_e14837 * (((0.5 * (locals.var_xg1x_dn6 - locals.var_q1s_dn6)) * assign15130_e14858) + (assign15130_e14847 * ((locals.var_xg1x_dn6 - locals.var_q1s_dn6) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn7 - locals.var_q1s_dn7) * assign15130_e14860) + (assign15130_e14837 * (((0.5 * (locals.var_xg1x_dn7 - locals.var_q1s_dn7)) * assign15130_e14858) + (assign15130_e14847 * ((locals.var_xg1x_dn7 - locals.var_q1s_dn7) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn8 - locals.var_q1s_dn8) * assign15130_e14860) + (assign15130_e14837 * (((0.5 * (locals.var_xg1x_dn8 - locals.var_q1s_dn8)) * assign15130_e14858) + (assign15130_e14847 * ((locals.var_xg1x_dn8 - locals.var_q1s_dn8) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn9 - locals.var_q1s_dn9) * assign15130_e14860) + (assign15130_e14837 * (((0.5 * (locals.var_xg1x_dn9 - locals.var_q1s_dn9)) * assign15130_e14858) + (assign15130_e14847 * ((locals.var_xg1x_dn9 - locals.var_q1s_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign15130_e14865;
        locals.var_q_temp1_dn4 = assign15130_e14865_d_n4;
        locals.var_q_temp1_dn6 = assign15130_e14865_d_n6;
        locals.var_q_temp1_dn7 = assign15130_e14865_d_n7;
        locals.var_q_temp1_dn8 = assign15130_e14865_d_n8;
        locals.var_q_temp1_dn9 = assign15130_e14865_d_n9;

        let assign15140_e14868: f64 = (locals.var_a0 * locals.var_q_temp1);
        locals.var_aexp1s = assign15140_e14868;
        locals.var_aexp1s_dn4 = ((locals.var_a0_dn4 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn4));
        locals.var_aexp1s_dn6 = ((locals.var_a0_dn6 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn6));
        locals.var_aexp1s_dn7 = ((locals.var_a0_dn7 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn7));
        locals.var_aexp1s_dn8 = ((locals.var_a0_dn8 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn8));
        locals.var_aexp1s_dn9 = ((locals.var_a0_dn9 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn9));

        let assign15150_e14871: f64 = (locals.var_k1q1s * locals.var_k1q1s);
        let assign15150_e14873: f64 = (assign15150_e14871 - locals.var_aexp1s);
        locals.var_qsqs = assign15150_e14873;
        locals.var_qsqs_dn4 = (((locals.var_k1q1s_dn4 * locals.var_k1q1s) + (locals.var_k1q1s * locals.var_k1q1s_dn4)) - locals.var_aexp1s_dn4);
        locals.var_qsqs_dn6 = (((locals.var_k1q1s_dn6 * locals.var_k1q1s) + (locals.var_k1q1s * locals.var_k1q1s_dn6)) - locals.var_aexp1s_dn6);
        locals.var_qsqs_dn7 = (((locals.var_k1q1s_dn7 * locals.var_k1q1s) + (locals.var_k1q1s * locals.var_k1q1s_dn7)) - locals.var_aexp1s_dn7);
        locals.var_qsqs_dn8 = (((locals.var_k1q1s_dn8 * locals.var_k1q1s) + (locals.var_k1q1s * locals.var_k1q1s_dn8)) - locals.var_aexp1s_dn8);
        locals.var_qsqs_dn9 = (((locals.var_k1q1s_dn9 * locals.var_k1q1s) + (locals.var_k1q1s * locals.var_k1q1s_dn9)) - locals.var_aexp1s_dn9);

        let assign15160_e14876: f64 = if locals.var_aexp1s <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard589 = assign15160_e14876;

        let (assign15170_e14880, assign15170_e14880_d_n4, assign15170_e14880_d_n6, assign15170_e14880_d_n7, assign15170_e14880_d_n8, assign15170_e14880_d_n9,) = {
    if (locals.var_guard589 != 0.0) {
        (1e-80, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qis, locals.var_qis_dn4, locals.var_qis_dn6, locals.var_qis_dn7, locals.var_qis_dn8, locals.var_qis_dn9,)
    }
};
        locals.var_qis = assign15170_e14880;
        locals.var_qis_dn4 = assign15170_e14880_d_n4;
        locals.var_qis_dn6 = assign15170_e14880_d_n6;
        locals.var_qis_dn7 = assign15170_e14880_d_n7;
        locals.var_qis_dn8 = assign15170_e14880_d_n8;
        locals.var_qis_dn9 = assign15170_e14880_d_n9;

        let (assign15180_e14886, assign15180_e14886_d_n4, assign15180_e14886_d_n6, assign15180_e14886_d_n7, assign15180_e14886_d_n8, assign15180_e14886_d_n9,) = {
    if (locals.var_guard589 != 0.0) {
        let assign15180_e14884: f64 = (locals.var_qis - locals.var_k1q1s);
        (assign15180_e14884, (locals.var_qis_dn4 - locals.var_k1q1s_dn4), (locals.var_qis_dn6 - locals.var_k1q1s_dn6), (locals.var_qis_dn7 - locals.var_k1q1s_dn7), (locals.var_qis_dn8 - locals.var_k1q1s_dn8), (locals.var_qis_dn9 - locals.var_k1q1s_dn9),)
    } else {
        (locals.var_k2q2s, locals.var_k2q2s_dn4, locals.var_k2q2s_dn6, locals.var_k2q2s_dn7, locals.var_k2q2s_dn8, locals.var_k2q2s_dn9,)
    }
};
        locals.var_k2q2s = assign15180_e14886;
        locals.var_k2q2s_dn4 = assign15180_e14886_d_n4;
        locals.var_k2q2s_dn6 = assign15180_e14886_d_n6;
        locals.var_k2q2s_dn7 = assign15180_e14886_d_n7;
        locals.var_k2q2s_dn8 = assign15180_e14886_d_n8;
        locals.var_k2q2s_dn9 = assign15180_e14886_d_n9;

        let (assign15190_e14892, assign15190_e14892_d_n4, assign15190_e14892_d_n6, assign15190_e14892_d_n7, assign15190_e14892_d_n8, assign15190_e14892_d_n9,) = {
    if (locals.var_guard589 != 0.0) {
        let assign15190_e14890: f64 = (locals.var_k2q2s / locals.var_k2);
        (assign15190_e14890, (((locals.var_k2q2s_dn4 * locals.var_k2) - (locals.var_k2q2s * locals.var_k2_dn4)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2s_dn6 * locals.var_k2) - (locals.var_k2q2s * locals.var_k2_dn6)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2s_dn7 * locals.var_k2) - (locals.var_k2q2s * locals.var_k2_dn7)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2s_dn8 * locals.var_k2) - (locals.var_k2q2s * locals.var_k2_dn8)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2s_dn9 * locals.var_k2) - (locals.var_k2q2s * locals.var_k2_dn9)) / (locals.var_k2 * locals.var_k2)),)
    } else {
        (locals.var_q2s, locals.var_q2s_dn4, locals.var_q2s_dn6, locals.var_q2s_dn7, locals.var_q2s_dn8, locals.var_q2s_dn9,)
    }
};
        locals.var_q2s = assign15190_e14892;
        locals.var_q2s_dn4 = assign15190_e14892_d_n4;
        locals.var_q2s_dn6 = assign15190_e14892_d_n6;
        locals.var_q2s_dn7 = assign15190_e14892_d_n7;
        locals.var_q2s_dn8 = assign15190_e14892_d_n8;
        locals.var_q2s_dn9 = assign15190_e14892_d_n9;

        let assign15200_e14895: f64 = (-0.005);
        let assign15200_e14896: f64 = if locals.var_qsqs < assign15200_e14895 { 1.0 } else { 0.0 };
        locals.var_guard590 = assign15200_e14896;

        let (assign15210_e14905, assign15210_e14905_d_n4, assign15210_e14905_d_n6, assign15210_e14905_d_n7, assign15210_e14905_d_n8, assign15210_e14905_d_n9,) = {
    if ((locals.var_guard589 == 0.0) && (locals.var_guard590 != 0.0)) {
        let assign15210_e14902: f64 = (locals.var_qsqs).abs();
        let assign15210_e14903: f64 = (assign15210_e14902).sqrt();
        (assign15210_e14903, (if locals.var_qsqs >= 0.0 { locals.var_qsqs_dn4 } else { (-locals.var_qsqs_dn4) } / (2.0 * assign15210_e14903)), (if locals.var_qsqs >= 0.0 { locals.var_qsqs_dn6 } else { (-locals.var_qsqs_dn6) } / (2.0 * assign15210_e14903)), (if locals.var_qsqs >= 0.0 { locals.var_qsqs_dn7 } else { (-locals.var_qsqs_dn7) } / (2.0 * assign15210_e14903)), (if locals.var_qsqs >= 0.0 { locals.var_qsqs_dn8 } else { (-locals.var_qsqs_dn8) } / (2.0 * assign15210_e14903)), (if locals.var_qsqs >= 0.0 { locals.var_qsqs_dn9 } else { (-locals.var_qsqs_dn9) } / (2.0 * assign15210_e14903)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign15210_e14905;
        locals.var_q_rac_qsq_dn4 = assign15210_e14905_d_n4;
        locals.var_q_rac_qsq_dn6 = assign15210_e14905_d_n6;
        locals.var_q_rac_qsq_dn7 = assign15210_e14905_d_n7;
        locals.var_q_rac_qsq_dn8 = assign15210_e14905_d_n8;
        locals.var_q_rac_qsq_dn9 = assign15210_e14905_d_n9;

        let (assign15220_e14917, assign15220_e14917_d_n4, assign15220_e14917_d_n6, assign15220_e14917_d_n7, assign15220_e14917_d_n8, assign15220_e14917_d_n9,) = {
    if ((locals.var_guard589 == 0.0) && (locals.var_guard590 != 0.0)) {
        let assign15220_e14913: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign15220_e14914: f64 = (assign15220_e14913).tan();
        let assign15220_e14915: f64 = (locals.var_q_rac_qsq / assign15220_e14914);
        (assign15220_e14915, (((locals.var_q_rac_qsq_dn4 * assign15220_e14914) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn4) / ((assign15220_e14913).cos() * (assign15220_e14913).cos())))) / (assign15220_e14914 * assign15220_e14914)), (((locals.var_q_rac_qsq_dn6 * assign15220_e14914) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn6) / ((assign15220_e14913).cos() * (assign15220_e14913).cos())))) / (assign15220_e14914 * assign15220_e14914)), (((locals.var_q_rac_qsq_dn7 * assign15220_e14914) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn7) / ((assign15220_e14913).cos() * (assign15220_e14913).cos())))) / (assign15220_e14914 * assign15220_e14914)), (((locals.var_q_rac_qsq_dn8 * assign15220_e14914) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn8) / ((assign15220_e14913).cos() * (assign15220_e14913).cos())))) / (assign15220_e14914 * assign15220_e14914)), (((locals.var_q_rac_qsq_dn9 * assign15220_e14914) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn9) / ((assign15220_e14913).cos() * (assign15220_e14913).cos())))) / (assign15220_e14914 * assign15220_e14914)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign15220_e14917;
        locals.var_q_qcoth_dn4 = assign15220_e14917_d_n4;
        locals.var_q_qcoth_dn6 = assign15220_e14917_d_n6;
        locals.var_q_qcoth_dn7 = assign15220_e14917_d_n7;
        locals.var_q_qcoth_dn8 = assign15220_e14917_d_n8;
        locals.var_q_qcoth_dn9 = assign15220_e14917_d_n9;

        let assign15230_e14920: f64 = if locals.var_qsqs > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard591 = assign15230_e14920;

        let (assign15240_e14932, assign15240_e14932_d_n4, assign15240_e14932_d_n6, assign15240_e14932_d_n7, assign15240_e14932_d_n8, assign15240_e14932_d_n9,) = {
    if (((locals.var_guard589 == 0.0) && (locals.var_guard590 == 0.0)) && (locals.var_guard591 != 0.0)) {
        let assign15240_e14929: f64 = (locals.var_qsqs).abs();
        let assign15240_e14930: f64 = (assign15240_e14929).sqrt();
        (assign15240_e14930, (if locals.var_qsqs >= 0.0 { locals.var_qsqs_dn4 } else { (-locals.var_qsqs_dn4) } / (2.0 * assign15240_e14930)), (if locals.var_qsqs >= 0.0 { locals.var_qsqs_dn6 } else { (-locals.var_qsqs_dn6) } / (2.0 * assign15240_e14930)), (if locals.var_qsqs >= 0.0 { locals.var_qsqs_dn7 } else { (-locals.var_qsqs_dn7) } / (2.0 * assign15240_e14930)), (if locals.var_qsqs >= 0.0 { locals.var_qsqs_dn8 } else { (-locals.var_qsqs_dn8) } / (2.0 * assign15240_e14930)), (if locals.var_qsqs >= 0.0 { locals.var_qsqs_dn9 } else { (-locals.var_qsqs_dn9) } / (2.0 * assign15240_e14930)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign15240_e14932;
        locals.var_q_rac_qsq_dn4 = assign15240_e14932_d_n4;
        locals.var_q_rac_qsq_dn6 = assign15240_e14932_d_n6;
        locals.var_q_rac_qsq_dn7 = assign15240_e14932_d_n7;
        locals.var_q_rac_qsq_dn8 = assign15240_e14932_d_n8;
        locals.var_q_rac_qsq_dn9 = assign15240_e14932_d_n9;

        let (assign15250_e14944, assign15250_e14944_d_n4, assign15250_e14944_d_n6, assign15250_e14944_d_n7, assign15250_e14944_d_n8, assign15250_e14944_d_n9,) = {
    if (((locals.var_guard589 == 0.0) && (locals.var_guard590 == 0.0)) && (locals.var_guard591 != 0.0)) {
        let assign15250_e14941: f64 = (-locals.var_q_rac_qsq);
        let assign15250_e14942: f64 = (assign15250_e14941).exp();
        (assign15250_e14942, (assign15250_e14942 * (-locals.var_q_rac_qsq_dn4)), (assign15250_e14942 * (-locals.var_q_rac_qsq_dn6)), (assign15250_e14942 * (-locals.var_q_rac_qsq_dn7)), (assign15250_e14942 * (-locals.var_q_rac_qsq_dn8)), (assign15250_e14942 * (-locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_invexpq, locals.var_q_invexpq_dn4, locals.var_q_invexpq_dn6, locals.var_q_invexpq_dn7, locals.var_q_invexpq_dn8, locals.var_q_invexpq_dn9,)
    }
};
        locals.var_q_invexpq = assign15250_e14944;
        locals.var_q_invexpq_dn4 = assign15250_e14944_d_n4;
        locals.var_q_invexpq_dn6 = assign15250_e14944_d_n6;
        locals.var_q_invexpq_dn7 = assign15250_e14944_d_n7;
        locals.var_q_invexpq_dn8 = assign15250_e14944_d_n8;
        locals.var_q_invexpq_dn9 = assign15250_e14944_d_n9;

        let (assign15260_e14962, assign15260_e14962_d_n4, assign15260_e14962_d_n6, assign15260_e14962_d_n7, assign15260_e14962_d_n8, assign15260_e14962_d_n9,) = {
    if (((locals.var_guard589 == 0.0) && (locals.var_guard590 == 0.0)) && (locals.var_guard591 != 0.0)) {
        let assign15260_e14955: f64 = (1.0 + locals.var_q_invexpq);
        let assign15260_e14956: f64 = (locals.var_q_rac_qsq * assign15260_e14955);
        let assign15260_e14959: f64 = (1.0 - locals.var_q_invexpq);
        let assign15260_e14960: f64 = (assign15260_e14956 / assign15260_e14959);
        (assign15260_e14960, (((((locals.var_q_rac_qsq_dn4 * assign15260_e14955) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn4)) * assign15260_e14959) - (assign15260_e14956 * (-locals.var_q_invexpq_dn4))) / (assign15260_e14959 * assign15260_e14959)), (((((locals.var_q_rac_qsq_dn6 * assign15260_e14955) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn6)) * assign15260_e14959) - (assign15260_e14956 * (-locals.var_q_invexpq_dn6))) / (assign15260_e14959 * assign15260_e14959)), (((((locals.var_q_rac_qsq_dn7 * assign15260_e14955) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn7)) * assign15260_e14959) - (assign15260_e14956 * (-locals.var_q_invexpq_dn7))) / (assign15260_e14959 * assign15260_e14959)), (((((locals.var_q_rac_qsq_dn8 * assign15260_e14955) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn8)) * assign15260_e14959) - (assign15260_e14956 * (-locals.var_q_invexpq_dn8))) / (assign15260_e14959 * assign15260_e14959)), (((((locals.var_q_rac_qsq_dn9 * assign15260_e14955) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn9)) * assign15260_e14959) - (assign15260_e14956 * (-locals.var_q_invexpq_dn9))) / (assign15260_e14959 * assign15260_e14959)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign15260_e14962;
        locals.var_q_qcoth_dn4 = assign15260_e14962_d_n4;
        locals.var_q_qcoth_dn6 = assign15260_e14962_d_n6;
        locals.var_q_qcoth_dn7 = assign15260_e14962_d_n7;
        locals.var_q_qcoth_dn8 = assign15260_e14962_d_n8;
        locals.var_q_qcoth_dn9 = assign15260_e14962_d_n9;

        let (assign15270_e14989, assign15270_e14989_d_n4, assign15270_e14989_d_n6, assign15270_e14989_d_n7, assign15270_e14989_d_n8, assign15270_e14989_d_n9,) = {
    if (((locals.var_guard589 == 0.0) && (locals.var_guard590 == 0.0)) && (locals.var_guard591 == 0.0)) {
        let assign15270_e14974: f64 = (locals.var_qsqs * 0.1666666666667);
        let assign15270_e14978: f64 = (locals.var_qsqs * 0.0166666666667);
        let assign15270_e14982: f64 = (locals.var_qsqs * 0.0238095238095);
        let assign15270_e14983: f64 = (1.0 - assign15270_e14982);
        let assign15270_e14984: f64 = (assign15270_e14978 * assign15270_e14983);
        let assign15270_e14985: f64 = (1.0 - assign15270_e14984);
        let assign15270_e14986: f64 = (assign15270_e14974 * assign15270_e14985);
        let assign15270_e14987: f64 = (2.0 + assign15270_e14986);
        (assign15270_e14987, (((locals.var_qsqs_dn4 * 0.1666666666667) * assign15270_e14985) + (assign15270_e14974 * (-(((locals.var_qsqs_dn4 * 0.0166666666667) * assign15270_e14983) + (assign15270_e14978 * (-(locals.var_qsqs_dn4 * 0.0238095238095))))))), (((locals.var_qsqs_dn6 * 0.1666666666667) * assign15270_e14985) + (assign15270_e14974 * (-(((locals.var_qsqs_dn6 * 0.0166666666667) * assign15270_e14983) + (assign15270_e14978 * (-(locals.var_qsqs_dn6 * 0.0238095238095))))))), (((locals.var_qsqs_dn7 * 0.1666666666667) * assign15270_e14985) + (assign15270_e14974 * (-(((locals.var_qsqs_dn7 * 0.0166666666667) * assign15270_e14983) + (assign15270_e14978 * (-(locals.var_qsqs_dn7 * 0.0238095238095))))))), (((locals.var_qsqs_dn8 * 0.1666666666667) * assign15270_e14985) + (assign15270_e14974 * (-(((locals.var_qsqs_dn8 * 0.0166666666667) * assign15270_e14983) + (assign15270_e14978 * (-(locals.var_qsqs_dn8 * 0.0238095238095))))))), (((locals.var_qsqs_dn9 * 0.1666666666667) * assign15270_e14985) + (assign15270_e14974 * (-(((locals.var_qsqs_dn9 * 0.0166666666667) * assign15270_e14983) + (assign15270_e14978 * (-(locals.var_qsqs_dn9 * 0.0238095238095))))))),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign15270_e14989;
        locals.var_q_qcoth_dn4 = assign15270_e14989_d_n4;
        locals.var_q_qcoth_dn6 = assign15270_e14989_d_n6;
        locals.var_q_qcoth_dn7 = assign15270_e14989_d_n7;
        locals.var_q_qcoth_dn8 = assign15270_e14989_d_n8;
        locals.var_q_qcoth_dn9 = assign15270_e14989_d_n9;

        let assign15280_e14992: f64 = (1.01 * locals.var_k1q1s);
        let assign15280_e14994: f64 = (assign15280_e14992 + locals.var_q_qcoth);
        let assign15280_e14996: f64 = if assign15280_e14994 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard592 = assign15280_e14996;

        let (assign15290_e15005, assign15290_e15005_d_n4, assign15290_e15005_d_n6, assign15290_e15005_d_n7, assign15290_e15005_d_n8, assign15290_e15005_d_n9,) = {
    if ((locals.var_guard589 == 0.0) && (locals.var_guard592 != 0.0)) {
        let assign15290_e15003: f64 = (locals.var_k1q1s + locals.var_q_qcoth);
        (assign15290_e15003, (locals.var_k1q1s_dn4 + locals.var_q_qcoth_dn4), (locals.var_k1q1s_dn6 + locals.var_q_qcoth_dn6), (locals.var_k1q1s_dn7 + locals.var_q_qcoth_dn7), (locals.var_k1q1s_dn8 + locals.var_q_qcoth_dn8), (locals.var_k1q1s_dn9 + locals.var_q_qcoth_dn9),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign15290_e15005;
        locals.var_q_temp1_dn4 = assign15290_e15005_d_n4;
        locals.var_q_temp1_dn6 = assign15290_e15005_d_n6;
        locals.var_q_temp1_dn7 = assign15290_e15005_d_n7;
        locals.var_q_temp1_dn8 = assign15290_e15005_d_n8;
        locals.var_q_temp1_dn9 = assign15290_e15005_d_n9;

        let assign15300_e15008: f64 = (locals.var_aexp1s * locals.var_k1q1s);
        let assign15300_e15011: f64 = (0.9 * locals.var_k1q1s);
        let assign15300_e15013: f64 = (assign15300_e15011 * locals.var_k1q1s);
        let assign15300_e15015: f64 = (assign15300_e15013 * locals.var_q_temp1);
        let assign15300_e15016: f64 = if assign15300_e15008 < assign15300_e15015 { 1.0 } else { 0.0 };
        locals.var_guard593 = assign15300_e15016;

        let (assign15310_e15029, assign15310_e15029_d_n4, assign15310_e15029_d_n6, assign15310_e15029_d_n7, assign15310_e15029_d_n8, assign15310_e15029_d_n9,) = {
    if (((locals.var_guard589 == 0.0) && (locals.var_guard592 != 0.0)) && (locals.var_guard593 != 0.0)) {
        let assign15310_e15025: f64 = (locals.var_aexp1s / locals.var_q_temp1);
        let assign15310_e15027: f64 = (assign15310_e15025 + 1e-80);
        (assign15310_e15027, (((locals.var_aexp1s_dn4 * locals.var_q_temp1) - (locals.var_aexp1s * locals.var_q_temp1_dn4)) / (locals.var_q_temp1 * locals.var_q_temp1)), (((locals.var_aexp1s_dn6 * locals.var_q_temp1) - (locals.var_aexp1s * locals.var_q_temp1_dn6)) / (locals.var_q_temp1 * locals.var_q_temp1)), (((locals.var_aexp1s_dn7 * locals.var_q_temp1) - (locals.var_aexp1s * locals.var_q_temp1_dn7)) / (locals.var_q_temp1 * locals.var_q_temp1)), (((locals.var_aexp1s_dn8 * locals.var_q_temp1) - (locals.var_aexp1s * locals.var_q_temp1_dn8)) / (locals.var_q_temp1 * locals.var_q_temp1)), (((locals.var_aexp1s_dn9 * locals.var_q_temp1) - (locals.var_aexp1s * locals.var_q_temp1_dn9)) / (locals.var_q_temp1 * locals.var_q_temp1)),)
    } else {
        (locals.var_qis, locals.var_qis_dn4, locals.var_qis_dn6, locals.var_qis_dn7, locals.var_qis_dn8, locals.var_qis_dn9,)
    }
};
        locals.var_qis = assign15310_e15029;
        locals.var_qis_dn4 = assign15310_e15029_d_n4;
        locals.var_qis_dn6 = assign15310_e15029_d_n6;
        locals.var_qis_dn7 = assign15310_e15029_d_n7;
        locals.var_qis_dn8 = assign15310_e15029_d_n8;
        locals.var_qis_dn9 = assign15310_e15029_d_n9;

        let (assign15320_e15040, assign15320_e15040_d_n4, assign15320_e15040_d_n6, assign15320_e15040_d_n7, assign15320_e15040_d_n8, assign15320_e15040_d_n9,) = {
    if (((locals.var_guard589 == 0.0) && (locals.var_guard592 != 0.0)) && (locals.var_guard593 != 0.0)) {
        let assign15320_e15038: f64 = (locals.var_qis - locals.var_k1q1s);
        (assign15320_e15038, (locals.var_qis_dn4 - locals.var_k1q1s_dn4), (locals.var_qis_dn6 - locals.var_k1q1s_dn6), (locals.var_qis_dn7 - locals.var_k1q1s_dn7), (locals.var_qis_dn8 - locals.var_k1q1s_dn8), (locals.var_qis_dn9 - locals.var_k1q1s_dn9),)
    } else {
        (locals.var_k2q2s, locals.var_k2q2s_dn4, locals.var_k2q2s_dn6, locals.var_k2q2s_dn7, locals.var_k2q2s_dn8, locals.var_k2q2s_dn9,)
    }
};
        locals.var_k2q2s = assign15320_e15040;
        locals.var_k2q2s_dn4 = assign15320_e15040_d_n4;
        locals.var_k2q2s_dn6 = assign15320_e15040_d_n6;
        locals.var_k2q2s_dn7 = assign15320_e15040_d_n7;
        locals.var_k2q2s_dn8 = assign15320_e15040_d_n8;
        locals.var_k2q2s_dn9 = assign15320_e15040_d_n9;

        let (assign15330_e15051, assign15330_e15051_d_n4, assign15330_e15051_d_n6, assign15330_e15051_d_n7, assign15330_e15051_d_n8, assign15330_e15051_d_n9,) = {
    if (((locals.var_guard589 == 0.0) && (locals.var_guard592 != 0.0)) && (locals.var_guard593 != 0.0)) {
        let assign15330_e15049: f64 = (locals.var_k2q2s / locals.var_k2);
        (assign15330_e15049, (((locals.var_k2q2s_dn4 * locals.var_k2) - (locals.var_k2q2s * locals.var_k2_dn4)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2s_dn6 * locals.var_k2) - (locals.var_k2q2s * locals.var_k2_dn6)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2s_dn7 * locals.var_k2) - (locals.var_k2q2s * locals.var_k2_dn7)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2s_dn8 * locals.var_k2) - (locals.var_k2q2s * locals.var_k2_dn8)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2s_dn9 * locals.var_k2) - (locals.var_k2q2s * locals.var_k2_dn9)) / (locals.var_k2 * locals.var_k2)),)
    } else {
        (locals.var_q2s, locals.var_q2s_dn4, locals.var_q2s_dn6, locals.var_q2s_dn7, locals.var_q2s_dn8, locals.var_q2s_dn9,)
    }
};
        locals.var_q2s = assign15330_e15051;
        locals.var_q2s_dn4 = assign15330_e15051_d_n4;
        locals.var_q2s_dn6 = assign15330_e15051_d_n6;
        locals.var_q2s_dn7 = assign15330_e15051_d_n7;
        locals.var_q2s_dn8 = assign15330_e15051_d_n8;
        locals.var_q2s_dn9 = assign15330_e15051_d_n9;

        let assign15340_e15054: f64 = if locals.var_qsqs > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard594 = assign15340_e15054;

        let (assign15350_e15079, assign15350_e15079_d_n4, assign15350_e15079_d_n6, assign15350_e15079_d_n7, assign15350_e15079_d_n8, assign15350_e15079_d_n9,) = {
    if ((((locals.var_guard589 == 0.0) && (locals.var_guard592 != 0.0)) && (locals.var_guard593 == 0.0)) && (locals.var_guard594 != 0.0)) {
        let assign15350_e15066: f64 = (4.0 * locals.var_qsqs);
        let assign15350_e15071: f64 = (2.0 - locals.var_q_invexpq);
        let assign15350_e15072: f64 = (locals.var_q_invexpq * assign15350_e15071);
        let assign15350_e15073: f64 = (1.0 - assign15350_e15072);
        let assign15350_e15074: f64 = (assign15350_e15066 / assign15350_e15073);
        let assign15350_e15075: f64 = (assign15350_e15074).ln();
        let assign15350_e15077: f64 = (assign15350_e15075 - locals.var_q_rac_qsq);
        (assign15350_e15077, ((((((4.0 * locals.var_qsqs_dn4) * assign15350_e15073) - (assign15350_e15066 * (-((locals.var_q_invexpq_dn4 * assign15350_e15071) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn4)))))) / (assign15350_e15073 * assign15350_e15073)) / assign15350_e15074) - locals.var_q_rac_qsq_dn4), ((((((4.0 * locals.var_qsqs_dn6) * assign15350_e15073) - (assign15350_e15066 * (-((locals.var_q_invexpq_dn6 * assign15350_e15071) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn6)))))) / (assign15350_e15073 * assign15350_e15073)) / assign15350_e15074) - locals.var_q_rac_qsq_dn6), ((((((4.0 * locals.var_qsqs_dn7) * assign15350_e15073) - (assign15350_e15066 * (-((locals.var_q_invexpq_dn7 * assign15350_e15071) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn7)))))) / (assign15350_e15073 * assign15350_e15073)) / assign15350_e15074) - locals.var_q_rac_qsq_dn7), ((((((4.0 * locals.var_qsqs_dn8) * assign15350_e15073) - (assign15350_e15066 * (-((locals.var_q_invexpq_dn8 * assign15350_e15071) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn8)))))) / (assign15350_e15073 * assign15350_e15073)) / assign15350_e15074) - locals.var_q_rac_qsq_dn8), ((((((4.0 * locals.var_qsqs_dn9) * assign15350_e15073) - (assign15350_e15066 * (-((locals.var_q_invexpq_dn9 * assign15350_e15071) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn9)))))) / (assign15350_e15073 * assign15350_e15073)) / assign15350_e15074) - locals.var_q_rac_qsq_dn9),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign15350_e15079;
        locals.var_q_temp2_dn4 = assign15350_e15079_d_n4;
        locals.var_q_temp2_dn6 = assign15350_e15079_d_n6;
        locals.var_q_temp2_dn7 = assign15350_e15079_d_n7;
        locals.var_q_temp2_dn8 = assign15350_e15079_d_n8;
        locals.var_q_temp2_dn9 = assign15350_e15079_d_n9;

        let assign15360_e15082: f64 = (-0.005);
        let assign15360_e15083: f64 = if locals.var_qsqs < assign15360_e15082 { 1.0 } else { 0.0 };
        locals.var_guard595 = assign15360_e15083;

        let (assign15370_e15101, assign15370_e15101_d_n4, assign15370_e15101_d_n6, assign15370_e15101_d_n7, assign15370_e15101_d_n8, assign15370_e15101_d_n9,) = {
    if (((((locals.var_guard589 == 0.0) && (locals.var_guard592 != 0.0)) && (locals.var_guard593 == 0.0)) && (locals.var_guard594 == 0.0)) && (locals.var_guard595 != 0.0)) {
        let assign15370_e15098: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign15370_e15099: f64 = (assign15370_e15098).sin();
        (assign15370_e15099, ((assign15370_e15098).cos() * (0.5 * locals.var_q_rac_qsq_dn4)), ((assign15370_e15098).cos() * (0.5 * locals.var_q_rac_qsq_dn6)), ((assign15370_e15098).cos() * (0.5 * locals.var_q_rac_qsq_dn7)), ((assign15370_e15098).cos() * (0.5 * locals.var_q_rac_qsq_dn8)), ((assign15370_e15098).cos() * (0.5 * locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign15370_e15101;
        locals.var_q_temp3_dn4 = assign15370_e15101_d_n4;
        locals.var_q_temp3_dn6 = assign15370_e15101_d_n6;
        locals.var_q_temp3_dn7 = assign15370_e15101_d_n7;
        locals.var_q_temp3_dn8 = assign15370_e15101_d_n8;
        locals.var_q_temp3_dn9 = assign15370_e15101_d_n9;

        let (assign15380_e15122, assign15380_e15122_d_n4, assign15380_e15122_d_n6, assign15380_e15122_d_n7, assign15380_e15122_d_n8, assign15380_e15122_d_n9,) = {
    if (((((locals.var_guard589 == 0.0) && (locals.var_guard592 != 0.0)) && (locals.var_guard593 == 0.0)) && (locals.var_guard594 == 0.0)) && (locals.var_guard595 != 0.0)) {
        let assign15380_e15115: f64 = (-locals.var_qsqs);
        let assign15380_e15118: f64 = (locals.var_q_temp3 * locals.var_q_temp3);
        let assign15380_e15119: f64 = (assign15380_e15115 / assign15380_e15118);
        let assign15380_e15120: f64 = (assign15380_e15119).ln();
        (assign15380_e15120, (((((-locals.var_qsqs_dn4) * assign15380_e15118) - (assign15380_e15115 * ((locals.var_q_temp3_dn4 * locals.var_q_temp3) + (locals.var_q_temp3 * locals.var_q_temp3_dn4)))) / (assign15380_e15118 * assign15380_e15118)) / assign15380_e15119), (((((-locals.var_qsqs_dn6) * assign15380_e15118) - (assign15380_e15115 * ((locals.var_q_temp3_dn6 * locals.var_q_temp3) + (locals.var_q_temp3 * locals.var_q_temp3_dn6)))) / (assign15380_e15118 * assign15380_e15118)) / assign15380_e15119), (((((-locals.var_qsqs_dn7) * assign15380_e15118) - (assign15380_e15115 * ((locals.var_q_temp3_dn7 * locals.var_q_temp3) + (locals.var_q_temp3 * locals.var_q_temp3_dn7)))) / (assign15380_e15118 * assign15380_e15118)) / assign15380_e15119), (((((-locals.var_qsqs_dn8) * assign15380_e15118) - (assign15380_e15115 * ((locals.var_q_temp3_dn8 * locals.var_q_temp3) + (locals.var_q_temp3 * locals.var_q_temp3_dn8)))) / (assign15380_e15118 * assign15380_e15118)) / assign15380_e15119), (((((-locals.var_qsqs_dn9) * assign15380_e15118) - (assign15380_e15115 * ((locals.var_q_temp3_dn9 * locals.var_q_temp3) + (locals.var_q_temp3 * locals.var_q_temp3_dn9)))) / (assign15380_e15118 * assign15380_e15118)) / assign15380_e15119),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign15380_e15122;
        locals.var_q_temp2_dn4 = assign15380_e15122_d_n4;
        locals.var_q_temp2_dn6 = assign15380_e15122_d_n6;
        locals.var_q_temp2_dn7 = assign15380_e15122_d_n7;
        locals.var_q_temp2_dn8 = assign15380_e15122_d_n8;
        locals.var_q_temp2_dn9 = assign15380_e15122_d_n9;

        let (assign15390_e15155, assign15390_e15155_d_n4, assign15390_e15155_d_n6, assign15390_e15155_d_n7, assign15390_e15155_d_n8, assign15390_e15155_d_n9,) = {
    if (((((locals.var_guard589 == 0.0) && (locals.var_guard592 != 0.0)) && (locals.var_guard593 == 0.0)) && (locals.var_guard594 == 0.0)) && (locals.var_guard595 == 0.0)) {
        let assign15390_e15139: f64 = (locals.var_qsqs * 0.3333333333333);
        let assign15390_e15143: f64 = (0.05 * locals.var_qsqs);
        let assign15390_e15147: f64 = (0.0396825396825397 * locals.var_qsqs);
        let assign15390_e15148: f64 = (1.0 - assign15390_e15147);
        let assign15390_e15149: f64 = (assign15390_e15143 * assign15390_e15148);
        let assign15390_e15150: f64 = (1.0 - assign15390_e15149);
        let assign15390_e15151: f64 = (assign15390_e15139 * assign15390_e15150);
        let assign15390_e15152: f64 = (4.0 - assign15390_e15151);
        let assign15390_e15153: f64 = (assign15390_e15152).ln();
        (assign15390_e15153, ((-(((locals.var_qsqs_dn4 * 0.3333333333333) * assign15390_e15150) + (assign15390_e15139 * (-(((0.05 * locals.var_qsqs_dn4) * assign15390_e15148) + (assign15390_e15143 * (-(0.0396825396825397 * locals.var_qsqs_dn4)))))))) / assign15390_e15152), ((-(((locals.var_qsqs_dn6 * 0.3333333333333) * assign15390_e15150) + (assign15390_e15139 * (-(((0.05 * locals.var_qsqs_dn6) * assign15390_e15148) + (assign15390_e15143 * (-(0.0396825396825397 * locals.var_qsqs_dn6)))))))) / assign15390_e15152), ((-(((locals.var_qsqs_dn7 * 0.3333333333333) * assign15390_e15150) + (assign15390_e15139 * (-(((0.05 * locals.var_qsqs_dn7) * assign15390_e15148) + (assign15390_e15143 * (-(0.0396825396825397 * locals.var_qsqs_dn7)))))))) / assign15390_e15152), ((-(((locals.var_qsqs_dn8 * 0.3333333333333) * assign15390_e15150) + (assign15390_e15139 * (-(((0.05 * locals.var_qsqs_dn8) * assign15390_e15148) + (assign15390_e15143 * (-(0.0396825396825397 * locals.var_qsqs_dn8)))))))) / assign15390_e15152), ((-(((locals.var_qsqs_dn9 * 0.3333333333333) * assign15390_e15150) + (assign15390_e15139 * (-(((0.05 * locals.var_qsqs_dn9) * assign15390_e15148) + (assign15390_e15143 * (-(0.0396825396825397 * locals.var_qsqs_dn9)))))))) / assign15390_e15152),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign15390_e15155;
        locals.var_q_temp2_dn4 = assign15390_e15155_d_n4;
        locals.var_q_temp2_dn6 = assign15390_e15155_d_n6;
        locals.var_q_temp2_dn7 = assign15390_e15155_d_n7;
        locals.var_q_temp2_dn8 = assign15390_e15155_d_n8;
        locals.var_q_temp2_dn9 = assign15390_e15155_d_n9;

        let (assign15400_e15176, assign15400_e15176_d_n4, assign15400_e15176_d_n6, assign15400_e15176_d_n7, assign15400_e15176_d_n8, assign15400_e15176_d_n9,) = {
    if (((locals.var_guard589 == 0.0) && (locals.var_guard592 != 0.0)) && (locals.var_guard593 == 0.0)) {
        let assign15400_e15165: f64 = (locals.var_xg2x - locals.var_xg1x);
        let assign15400_e15167: f64 = (assign15400_e15165 + locals.var_q1s);
        let assign15400_e15170: f64 = (locals.var_q_temp1).ln();
        let assign15400_e15171: f64 = (2.0 * assign15400_e15170);
        let assign15400_e15172: f64 = (assign15400_e15167 + assign15400_e15171);
        let assign15400_e15174: f64 = (assign15400_e15172 - locals.var_q_temp2);
        (assign15400_e15174, ((((locals.var_xg2x_dn4 - locals.var_xg1x_dn4) + locals.var_q1s_dn4) + (2.0 * (locals.var_q_temp1_dn4 / locals.var_q_temp1))) - locals.var_q_temp2_dn4), ((((locals.var_xg2x_dn6 - locals.var_xg1x_dn6) + locals.var_q1s_dn6) + (2.0 * (locals.var_q_temp1_dn6 / locals.var_q_temp1))) - locals.var_q_temp2_dn6), ((((locals.var_xg2x_dn7 - locals.var_xg1x_dn7) + locals.var_q1s_dn7) + (2.0 * (locals.var_q_temp1_dn7 / locals.var_q_temp1))) - locals.var_q_temp2_dn7), ((((locals.var_xg2x_dn8 - locals.var_xg1x_dn8) + locals.var_q1s_dn8) + (2.0 * (locals.var_q_temp1_dn8 / locals.var_q_temp1))) - locals.var_q_temp2_dn8), ((((locals.var_xg2x_dn9 - locals.var_xg1x_dn9) + locals.var_q1s_dn9) + (2.0 * (locals.var_q_temp1_dn9 / locals.var_q_temp1))) - locals.var_q_temp2_dn9),)
    } else {
        (locals.var_q2s, locals.var_q2s_dn4, locals.var_q2s_dn6, locals.var_q2s_dn7, locals.var_q2s_dn8, locals.var_q2s_dn9,)
    }
};
        locals.var_q2s = assign15400_e15176;
        locals.var_q2s_dn4 = assign15400_e15176_d_n4;
        locals.var_q2s_dn6 = assign15400_e15176_d_n6;
        locals.var_q2s_dn7 = assign15400_e15176_d_n7;
        locals.var_q2s_dn8 = assign15400_e15176_d_n8;
        locals.var_q2s_dn9 = assign15400_e15176_d_n9;

        let (assign15410_e15188, assign15410_e15188_d_n4, assign15410_e15188_d_n6, assign15410_e15188_d_n7, assign15410_e15188_d_n8, assign15410_e15188_d_n9,) = {
    if (((locals.var_guard589 == 0.0) && (locals.var_guard592 != 0.0)) && (locals.var_guard593 == 0.0)) {
        let assign15410_e15186: f64 = (locals.var_k2 * locals.var_q2s);
        (assign15410_e15186, ((locals.var_k2_dn4 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn4)), ((locals.var_k2_dn6 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn6)), ((locals.var_k2_dn7 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn7)), ((locals.var_k2_dn8 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn8)), ((locals.var_k2_dn9 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn9)),)
    } else {
        (locals.var_k2q2s, locals.var_k2q2s_dn4, locals.var_k2q2s_dn6, locals.var_k2q2s_dn7, locals.var_k2q2s_dn8, locals.var_k2q2s_dn9,)
    }
};
        locals.var_k2q2s = assign15410_e15188;
        locals.var_k2q2s_dn4 = assign15410_e15188_d_n4;
        locals.var_k2q2s_dn6 = assign15410_e15188_d_n6;
        locals.var_k2q2s_dn7 = assign15410_e15188_d_n7;
        locals.var_k2q2s_dn8 = assign15410_e15188_d_n8;
        locals.var_k2q2s_dn9 = assign15410_e15188_d_n9;

        let (assign15420_e15200, assign15420_e15200_d_n4, assign15420_e15200_d_n6, assign15420_e15200_d_n7, assign15420_e15200_d_n8, assign15420_e15200_d_n9,) = {
    if (((locals.var_guard589 == 0.0) && (locals.var_guard592 != 0.0)) && (locals.var_guard593 == 0.0)) {
        let assign15420_e15198: f64 = (locals.var_k1q1s + locals.var_k2q2s);
        (assign15420_e15198, (locals.var_k1q1s_dn4 + locals.var_k2q2s_dn4), (locals.var_k1q1s_dn6 + locals.var_k2q2s_dn6), (locals.var_k1q1s_dn7 + locals.var_k2q2s_dn7), (locals.var_k1q1s_dn8 + locals.var_k2q2s_dn8), (locals.var_k1q1s_dn9 + locals.var_k2q2s_dn9),)
    } else {
        (locals.var_qis, locals.var_qis_dn4, locals.var_qis_dn6, locals.var_qis_dn7, locals.var_qis_dn8, locals.var_qis_dn9,)
    }
};
        locals.var_qis = assign15420_e15200;
        locals.var_qis_dn4 = assign15420_e15200_d_n4;
        locals.var_qis_dn6 = assign15420_e15200_d_n6;
        locals.var_qis_dn7 = assign15420_e15200_d_n7;
        locals.var_qis_dn8 = assign15420_e15200_d_n8;
        locals.var_qis_dn9 = assign15420_e15200_d_n9;

        let assign15430_e15203: f64 = if locals.var_qsqs > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard596 = assign15430_e15203;

        let assign15440_e15206: f64 = locals.var_q1s;
        let assign15440_e15208: f64 = (assign15440_e15206 - locals.var_xg1x);
        let assign15440_e15210: f64 = (assign15440_e15208 - locals.var_q_rac_qsq);
        let assign15440_e15212: f64 = if assign15440_e15210 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard597 = assign15440_e15212;

        let (assign15450_e15231, assign15450_e15231_d_n4, assign15450_e15231_d_n6, assign15450_e15231_d_n7, assign15450_e15231_d_n8, assign15450_e15231_d_n9,) = {
    if ((((locals.var_guard589 == 0.0) && (locals.var_guard592 == 0.0)) && (locals.var_guard596 != 0.0)) && (locals.var_guard597 != 0.0)) {
        let assign15450_e15224: f64 = locals.var_q1s;
        let assign15450_e15226: f64 = (assign15450_e15224 - locals.var_xg1x);
        let assign15450_e15228: f64 = (assign15450_e15226 - locals.var_q_rac_qsq);
        let assign15450_e15229: f64 = (assign15450_e15228).exp();
        (assign15450_e15229, (assign15450_e15229 * ((locals.var_q1s_dn4 - locals.var_xg1x_dn4) - locals.var_q_rac_qsq_dn4)), (assign15450_e15229 * ((locals.var_q1s_dn6 - locals.var_xg1x_dn6) - locals.var_q_rac_qsq_dn6)), (assign15450_e15229 * ((locals.var_q1s_dn7 - locals.var_xg1x_dn7) - locals.var_q_rac_qsq_dn7)), (assign15450_e15229 * ((locals.var_q1s_dn8 - locals.var_xg1x_dn8) - locals.var_q_rac_qsq_dn8)), (assign15450_e15229 * ((locals.var_q1s_dn9 - locals.var_xg1x_dn9) - locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign15450_e15231;
        locals.var_q_temp3_dn4 = assign15450_e15231_d_n4;
        locals.var_q_temp3_dn6 = assign15450_e15231_d_n6;
        locals.var_q_temp3_dn7 = assign15450_e15231_d_n7;
        locals.var_q_temp3_dn8 = assign15450_e15231_d_n8;
        locals.var_q_temp3_dn9 = assign15450_e15231_d_n9;

        let (assign15460_e15284, assign15460_e15284_d_n4, assign15460_e15284_d_n6, assign15460_e15284_d_n7, assign15460_e15284_d_n8, assign15460_e15284_d_n9,) = {
    if ((((locals.var_guard589 == 0.0) && (locals.var_guard592 == 0.0)) && (locals.var_guard596 != 0.0)) && (locals.var_guard597 == 0.0)) {
        let assign15460_e15246: f64 = locals.var_q1s;
        let assign15460_e15248: f64 = (assign15460_e15246 - locals.var_xg1x);
        let assign15460_e15250: f64 = (assign15460_e15248 - locals.var_q_rac_qsq);
        let assign15460_e15252: f64 = (assign15460_e15250 - 80.0);
        let assign15460_e15257: f64 = locals.var_q1s;
        let assign15460_e15259: f64 = (assign15460_e15257 - locals.var_xg1x);
        let assign15460_e15261: f64 = (assign15460_e15259 - locals.var_q_rac_qsq);
        let assign15460_e15263: f64 = (assign15460_e15261 - 80.0);
        let assign15460_e15264: f64 = (0.5 * assign15460_e15263);
        let assign15460_e15268: f64 = locals.var_q1s;
        let assign15460_e15270: f64 = (assign15460_e15268 - locals.var_xg1x);
        let assign15460_e15272: f64 = (assign15460_e15270 - locals.var_q_rac_qsq);
        let assign15460_e15274: f64 = (assign15460_e15272 - 80.0);
        let assign15460_e15276: f64 = (assign15460_e15274 * 0.3333333333333);
        let assign15460_e15277: f64 = (1.0 + assign15460_e15276);
        let assign15460_e15278: f64 = (assign15460_e15264 * assign15460_e15277);
        let assign15460_e15279: f64 = (1.0 + assign15460_e15278);
        let assign15460_e15280: f64 = (assign15460_e15252 * assign15460_e15279);
        let assign15460_e15281: f64 = (1.0 + assign15460_e15280);
        let assign15460_e15282: f64 = (5.54062e34 * assign15460_e15281);
        (assign15460_e15282, (5.54062e34 * ((((locals.var_q1s_dn4 - locals.var_xg1x_dn4) - locals.var_q_rac_qsq_dn4) * assign15460_e15279) + (assign15460_e15252 * (((0.5 * ((locals.var_q1s_dn4 - locals.var_xg1x_dn4) - locals.var_q_rac_qsq_dn4)) * assign15460_e15277) + (assign15460_e15264 * (((locals.var_q1s_dn4 - locals.var_xg1x_dn4) - locals.var_q_rac_qsq_dn4) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_q1s_dn6 - locals.var_xg1x_dn6) - locals.var_q_rac_qsq_dn6) * assign15460_e15279) + (assign15460_e15252 * (((0.5 * ((locals.var_q1s_dn6 - locals.var_xg1x_dn6) - locals.var_q_rac_qsq_dn6)) * assign15460_e15277) + (assign15460_e15264 * (((locals.var_q1s_dn6 - locals.var_xg1x_dn6) - locals.var_q_rac_qsq_dn6) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_q1s_dn7 - locals.var_xg1x_dn7) - locals.var_q_rac_qsq_dn7) * assign15460_e15279) + (assign15460_e15252 * (((0.5 * ((locals.var_q1s_dn7 - locals.var_xg1x_dn7) - locals.var_q_rac_qsq_dn7)) * assign15460_e15277) + (assign15460_e15264 * (((locals.var_q1s_dn7 - locals.var_xg1x_dn7) - locals.var_q_rac_qsq_dn7) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_q1s_dn8 - locals.var_xg1x_dn8) - locals.var_q_rac_qsq_dn8) * assign15460_e15279) + (assign15460_e15252 * (((0.5 * ((locals.var_q1s_dn8 - locals.var_xg1x_dn8) - locals.var_q_rac_qsq_dn8)) * assign15460_e15277) + (assign15460_e15264 * (((locals.var_q1s_dn8 - locals.var_xg1x_dn8) - locals.var_q_rac_qsq_dn8) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_q1s_dn9 - locals.var_xg1x_dn9) - locals.var_q_rac_qsq_dn9) * assign15460_e15279) + (assign15460_e15252 * (((0.5 * ((locals.var_q1s_dn9 - locals.var_xg1x_dn9) - locals.var_q_rac_qsq_dn9)) * assign15460_e15277) + (assign15460_e15264 * (((locals.var_q1s_dn9 - locals.var_xg1x_dn9) - locals.var_q_rac_qsq_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign15460_e15284;
        locals.var_q_temp3_dn4 = assign15460_e15284_d_n4;
        locals.var_q_temp3_dn6 = assign15460_e15284_d_n6;
        locals.var_q_temp3_dn7 = assign15460_e15284_d_n7;
        locals.var_q_temp3_dn8 = assign15460_e15284_d_n8;
        locals.var_q_temp3_dn9 = assign15460_e15284_d_n9;

    }

    pub(super) fn stamp_transient_block_38(
        locals: &mut StampLocals,
    ) {
        let (assign15470_e15296, assign15470_e15296_d_n4, assign15470_e15296_d_n6, assign15470_e15296_d_n7, assign15470_e15296_d_n8, assign15470_e15296_d_n9,) = {
    if (((locals.var_guard589 == 0.0) && (locals.var_guard592 == 0.0)) && (locals.var_guard596 != 0.0)) {
        let assign15470_e15294: f64 = (locals.var_q_temp3 / locals.var_a0);
        (assign15470_e15294, (((locals.var_q_temp3_dn4 * locals.var_a0) - (locals.var_q_temp3 * locals.var_a0_dn4)) / (locals.var_a0 * locals.var_a0)), (((locals.var_q_temp3_dn6 * locals.var_a0) - (locals.var_q_temp3 * locals.var_a0_dn6)) / (locals.var_a0 * locals.var_a0)), (((locals.var_q_temp3_dn7 * locals.var_a0) - (locals.var_q_temp3 * locals.var_a0_dn7)) / (locals.var_a0 * locals.var_a0)), (((locals.var_q_temp3_dn8 * locals.var_a0) - (locals.var_q_temp3 * locals.var_a0_dn8)) / (locals.var_a0 * locals.var_a0)), (((locals.var_q_temp3_dn9 * locals.var_a0) - (locals.var_q_temp3 * locals.var_a0_dn9)) / (locals.var_a0 * locals.var_a0)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign15470_e15296;
        locals.var_q_temp2_dn4 = assign15470_e15296_d_n4;
        locals.var_q_temp2_dn6 = assign15470_e15296_d_n6;
        locals.var_q_temp2_dn7 = assign15470_e15296_d_n7;
        locals.var_q_temp2_dn8 = assign15470_e15296_d_n8;
        locals.var_q_temp2_dn9 = assign15470_e15296_d_n9;

        let (assign15480_e15318, assign15480_e15318_d_n4, assign15480_e15318_d_n6, assign15480_e15318_d_n7, assign15480_e15318_d_n8, assign15480_e15318_d_n9,) = {
    if (((locals.var_guard589 == 0.0) && (locals.var_guard592 == 0.0)) && (locals.var_guard596 != 0.0)) {
        let assign15480_e15306: f64 = (4.0 * locals.var_qsqs);
        let assign15480_e15308: f64 = (assign15480_e15306 * locals.var_q_temp2);
        let assign15480_e15313: f64 = (2.0 - locals.var_q_invexpq);
        let assign15480_e15314: f64 = (locals.var_q_invexpq * assign15480_e15313);
        let assign15480_e15315: f64 = (1.0 - assign15480_e15314);
        let assign15480_e15316: f64 = (assign15480_e15308 / assign15480_e15315);
        (assign15480_e15316, ((((((4.0 * locals.var_qsqs_dn4) * locals.var_q_temp2) + (assign15480_e15306 * locals.var_q_temp2_dn4)) * assign15480_e15315) - (assign15480_e15308 * (-((locals.var_q_invexpq_dn4 * assign15480_e15313) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn4)))))) / (assign15480_e15315 * assign15480_e15315)), ((((((4.0 * locals.var_qsqs_dn6) * locals.var_q_temp2) + (assign15480_e15306 * locals.var_q_temp2_dn6)) * assign15480_e15315) - (assign15480_e15308 * (-((locals.var_q_invexpq_dn6 * assign15480_e15313) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn6)))))) / (assign15480_e15315 * assign15480_e15315)), ((((((4.0 * locals.var_qsqs_dn7) * locals.var_q_temp2) + (assign15480_e15306 * locals.var_q_temp2_dn7)) * assign15480_e15315) - (assign15480_e15308 * (-((locals.var_q_invexpq_dn7 * assign15480_e15313) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn7)))))) / (assign15480_e15315 * assign15480_e15315)), ((((((4.0 * locals.var_qsqs_dn8) * locals.var_q_temp2) + (assign15480_e15306 * locals.var_q_temp2_dn8)) * assign15480_e15315) - (assign15480_e15308 * (-((locals.var_q_invexpq_dn8 * assign15480_e15313) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn8)))))) / (assign15480_e15315 * assign15480_e15315)), ((((((4.0 * locals.var_qsqs_dn9) * locals.var_q_temp2) + (assign15480_e15306 * locals.var_q_temp2_dn9)) * assign15480_e15315) - (assign15480_e15308 * (-((locals.var_q_invexpq_dn9 * assign15480_e15313) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn9)))))) / (assign15480_e15315 * assign15480_e15315)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign15480_e15318;
        locals.var_q_temp1_dn4 = assign15480_e15318_d_n4;
        locals.var_q_temp1_dn6 = assign15480_e15318_d_n6;
        locals.var_q_temp1_dn7 = assign15480_e15318_d_n7;
        locals.var_q_temp1_dn8 = assign15480_e15318_d_n8;
        locals.var_q_temp1_dn9 = assign15480_e15318_d_n9;

        let assign15490_e15321: f64 = (-0.005);
        let assign15490_e15322: f64 = if locals.var_qsqs < assign15490_e15321 { 1.0 } else { 0.0 };
        locals.var_guard598 = assign15490_e15322;

        let (assign15500_e15338, assign15500_e15338_d_n4, assign15500_e15338_d_n6, assign15500_e15338_d_n7, assign15500_e15338_d_n8, assign15500_e15338_d_n9,) = {
    if ((((locals.var_guard589 == 0.0) && (locals.var_guard592 == 0.0)) && (locals.var_guard596 == 0.0)) && (locals.var_guard598 != 0.0)) {
        let assign15500_e15335: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign15500_e15336: f64 = (assign15500_e15335).sin();
        (assign15500_e15336, ((assign15500_e15335).cos() * (0.5 * locals.var_q_rac_qsq_dn4)), ((assign15500_e15335).cos() * (0.5 * locals.var_q_rac_qsq_dn6)), ((assign15500_e15335).cos() * (0.5 * locals.var_q_rac_qsq_dn7)), ((assign15500_e15335).cos() * (0.5 * locals.var_q_rac_qsq_dn8)), ((assign15500_e15335).cos() * (0.5 * locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign15500_e15338;
        locals.var_q_temp2_dn4 = assign15500_e15338_d_n4;
        locals.var_q_temp2_dn6 = assign15500_e15338_d_n6;
        locals.var_q_temp2_dn7 = assign15500_e15338_d_n7;
        locals.var_q_temp2_dn8 = assign15500_e15338_d_n8;
        locals.var_q_temp2_dn9 = assign15500_e15338_d_n9;

        let (assign15510_e15358, assign15510_e15358_d_n4, assign15510_e15358_d_n6, assign15510_e15358_d_n7, assign15510_e15358_d_n8, assign15510_e15358_d_n9,) = {
    if ((((locals.var_guard589 == 0.0) && (locals.var_guard592 == 0.0)) && (locals.var_guard596 == 0.0)) && (locals.var_guard598 != 0.0)) {
        let assign15510_e15350: f64 = (-locals.var_qsqs);
        let assign15510_e15353: f64 = (locals.var_q_temp2 * locals.var_q_temp2);
        let assign15510_e15354: f64 = (assign15510_e15350 / assign15510_e15353);
        let assign15510_e15356: f64 = (assign15510_e15354 / locals.var_aexp1s);
        (assign15510_e15356, (((((((-locals.var_qsqs_dn4) * assign15510_e15353) - (assign15510_e15350 * ((locals.var_q_temp2_dn4 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn4)))) / (assign15510_e15353 * assign15510_e15353)) * locals.var_aexp1s) - (assign15510_e15354 * locals.var_aexp1s_dn4)) / (locals.var_aexp1s * locals.var_aexp1s)), (((((((-locals.var_qsqs_dn6) * assign15510_e15353) - (assign15510_e15350 * ((locals.var_q_temp2_dn6 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn6)))) / (assign15510_e15353 * assign15510_e15353)) * locals.var_aexp1s) - (assign15510_e15354 * locals.var_aexp1s_dn6)) / (locals.var_aexp1s * locals.var_aexp1s)), (((((((-locals.var_qsqs_dn7) * assign15510_e15353) - (assign15510_e15350 * ((locals.var_q_temp2_dn7 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn7)))) / (assign15510_e15353 * assign15510_e15353)) * locals.var_aexp1s) - (assign15510_e15354 * locals.var_aexp1s_dn7)) / (locals.var_aexp1s * locals.var_aexp1s)), (((((((-locals.var_qsqs_dn8) * assign15510_e15353) - (assign15510_e15350 * ((locals.var_q_temp2_dn8 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn8)))) / (assign15510_e15353 * assign15510_e15353)) * locals.var_aexp1s) - (assign15510_e15354 * locals.var_aexp1s_dn8)) / (locals.var_aexp1s * locals.var_aexp1s)), (((((((-locals.var_qsqs_dn9) * assign15510_e15353) - (assign15510_e15350 * ((locals.var_q_temp2_dn9 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn9)))) / (assign15510_e15353 * assign15510_e15353)) * locals.var_aexp1s) - (assign15510_e15354 * locals.var_aexp1s_dn9)) / (locals.var_aexp1s * locals.var_aexp1s)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign15510_e15358;
        locals.var_q_temp1_dn4 = assign15510_e15358_d_n4;
        locals.var_q_temp1_dn6 = assign15510_e15358_d_n6;
        locals.var_q_temp1_dn7 = assign15510_e15358_d_n7;
        locals.var_q_temp1_dn8 = assign15510_e15358_d_n8;
        locals.var_q_temp1_dn9 = assign15510_e15358_d_n9;

        let (assign15520_e15390, assign15520_e15390_d_n4, assign15520_e15390_d_n6, assign15520_e15390_d_n7, assign15520_e15390_d_n8, assign15520_e15390_d_n9,) = {
    if ((((locals.var_guard589 == 0.0) && (locals.var_guard592 == 0.0)) && (locals.var_guard596 == 0.0)) && (locals.var_guard598 == 0.0)) {
        let assign15520_e15373: f64 = (locals.var_qsqs * 0.3333333333333);
        let assign15520_e15377: f64 = (0.05 * locals.var_qsqs);
        let assign15520_e15381: f64 = (0.0396825396825397 * locals.var_qsqs);
        let assign15520_e15382: f64 = (1.0 - assign15520_e15381);
        let assign15520_e15383: f64 = (assign15520_e15377 * assign15520_e15382);
        let assign15520_e15384: f64 = (1.0 - assign15520_e15383);
        let assign15520_e15385: f64 = (assign15520_e15373 * assign15520_e15384);
        let assign15520_e15386: f64 = (4.0 - assign15520_e15385);
        let assign15520_e15388: f64 = (assign15520_e15386 / locals.var_aexp1s);
        (assign15520_e15388, ((((-(((locals.var_qsqs_dn4 * 0.3333333333333) * assign15520_e15384) + (assign15520_e15373 * (-(((0.05 * locals.var_qsqs_dn4) * assign15520_e15382) + (assign15520_e15377 * (-(0.0396825396825397 * locals.var_qsqs_dn4)))))))) * locals.var_aexp1s) - (assign15520_e15386 * locals.var_aexp1s_dn4)) / (locals.var_aexp1s * locals.var_aexp1s)), ((((-(((locals.var_qsqs_dn6 * 0.3333333333333) * assign15520_e15384) + (assign15520_e15373 * (-(((0.05 * locals.var_qsqs_dn6) * assign15520_e15382) + (assign15520_e15377 * (-(0.0396825396825397 * locals.var_qsqs_dn6)))))))) * locals.var_aexp1s) - (assign15520_e15386 * locals.var_aexp1s_dn6)) / (locals.var_aexp1s * locals.var_aexp1s)), ((((-(((locals.var_qsqs_dn7 * 0.3333333333333) * assign15520_e15384) + (assign15520_e15373 * (-(((0.05 * locals.var_qsqs_dn7) * assign15520_e15382) + (assign15520_e15377 * (-(0.0396825396825397 * locals.var_qsqs_dn7)))))))) * locals.var_aexp1s) - (assign15520_e15386 * locals.var_aexp1s_dn7)) / (locals.var_aexp1s * locals.var_aexp1s)), ((((-(((locals.var_qsqs_dn8 * 0.3333333333333) * assign15520_e15384) + (assign15520_e15373 * (-(((0.05 * locals.var_qsqs_dn8) * assign15520_e15382) + (assign15520_e15377 * (-(0.0396825396825397 * locals.var_qsqs_dn8)))))))) * locals.var_aexp1s) - (assign15520_e15386 * locals.var_aexp1s_dn8)) / (locals.var_aexp1s * locals.var_aexp1s)), ((((-(((locals.var_qsqs_dn9 * 0.3333333333333) * assign15520_e15384) + (assign15520_e15373 * (-(((0.05 * locals.var_qsqs_dn9) * assign15520_e15382) + (assign15520_e15377 * (-(0.0396825396825397 * locals.var_qsqs_dn9)))))))) * locals.var_aexp1s) - (assign15520_e15386 * locals.var_aexp1s_dn9)) / (locals.var_aexp1s * locals.var_aexp1s)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign15520_e15390;
        locals.var_q_temp1_dn4 = assign15520_e15390_d_n4;
        locals.var_q_temp1_dn6 = assign15520_e15390_d_n6;
        locals.var_q_temp1_dn7 = assign15520_e15390_d_n7;
        locals.var_q_temp1_dn8 = assign15520_e15390_d_n8;
        locals.var_q_temp1_dn9 = assign15520_e15390_d_n9;

        let (assign15530_e15406, assign15530_e15406_d_n4, assign15530_e15406_d_n6, assign15530_e15406_d_n7, assign15530_e15406_d_n8, assign15530_e15406_d_n9,) = {
    if ((locals.var_guard589 == 0.0) && (locals.var_guard592 == 0.0)) {
        let assign15530_e15398: f64 = (locals.var_k1q1s - locals.var_q_qcoth);
        let assign15530_e15401: f64 = (1.0 - locals.var_q_temp1);
        let assign15530_e15402: f64 = (assign15530_e15398 / assign15530_e15401);
        let assign15530_e15404: f64 = (assign15530_e15402 + 1e-80);
        (assign15530_e15404, ((((locals.var_k1q1s_dn4 - locals.var_q_qcoth_dn4) * assign15530_e15401) - (assign15530_e15398 * (-locals.var_q_temp1_dn4))) / (assign15530_e15401 * assign15530_e15401)), ((((locals.var_k1q1s_dn6 - locals.var_q_qcoth_dn6) * assign15530_e15401) - (assign15530_e15398 * (-locals.var_q_temp1_dn6))) / (assign15530_e15401 * assign15530_e15401)), ((((locals.var_k1q1s_dn7 - locals.var_q_qcoth_dn7) * assign15530_e15401) - (assign15530_e15398 * (-locals.var_q_temp1_dn7))) / (assign15530_e15401 * assign15530_e15401)), ((((locals.var_k1q1s_dn8 - locals.var_q_qcoth_dn8) * assign15530_e15401) - (assign15530_e15398 * (-locals.var_q_temp1_dn8))) / (assign15530_e15401 * assign15530_e15401)), ((((locals.var_k1q1s_dn9 - locals.var_q_qcoth_dn9) * assign15530_e15401) - (assign15530_e15398 * (-locals.var_q_temp1_dn9))) / (assign15530_e15401 * assign15530_e15401)),)
    } else {
        (locals.var_qis, locals.var_qis_dn4, locals.var_qis_dn6, locals.var_qis_dn7, locals.var_qis_dn8, locals.var_qis_dn9,)
    }
};
        locals.var_qis = assign15530_e15406;
        locals.var_qis_dn4 = assign15530_e15406_d_n4;
        locals.var_qis_dn6 = assign15530_e15406_d_n6;
        locals.var_qis_dn7 = assign15530_e15406_d_n7;
        locals.var_qis_dn8 = assign15530_e15406_d_n8;
        locals.var_qis_dn9 = assign15530_e15406_d_n9;

        let (assign15540_e15416, assign15540_e15416_d_n4, assign15540_e15416_d_n6, assign15540_e15416_d_n7, assign15540_e15416_d_n8, assign15540_e15416_d_n9,) = {
    if ((locals.var_guard589 == 0.0) && (locals.var_guard592 == 0.0)) {
        let assign15540_e15414: f64 = (locals.var_qis - locals.var_k1q1s);
        (assign15540_e15414, (locals.var_qis_dn4 - locals.var_k1q1s_dn4), (locals.var_qis_dn6 - locals.var_k1q1s_dn6), (locals.var_qis_dn7 - locals.var_k1q1s_dn7), (locals.var_qis_dn8 - locals.var_k1q1s_dn8), (locals.var_qis_dn9 - locals.var_k1q1s_dn9),)
    } else {
        (locals.var_k2q2s, locals.var_k2q2s_dn4, locals.var_k2q2s_dn6, locals.var_k2q2s_dn7, locals.var_k2q2s_dn8, locals.var_k2q2s_dn9,)
    }
};
        locals.var_k2q2s = assign15540_e15416;
        locals.var_k2q2s_dn4 = assign15540_e15416_d_n4;
        locals.var_k2q2s_dn6 = assign15540_e15416_d_n6;
        locals.var_k2q2s_dn7 = assign15540_e15416_d_n7;
        locals.var_k2q2s_dn8 = assign15540_e15416_d_n8;
        locals.var_k2q2s_dn9 = assign15540_e15416_d_n9;

        let (assign15550_e15426, assign15550_e15426_d_n4, assign15550_e15426_d_n6, assign15550_e15426_d_n7, assign15550_e15426_d_n8, assign15550_e15426_d_n9,) = {
    if ((locals.var_guard589 == 0.0) && (locals.var_guard592 == 0.0)) {
        let assign15550_e15424: f64 = (locals.var_k2q2s / locals.var_k2);
        (assign15550_e15424, (((locals.var_k2q2s_dn4 * locals.var_k2) - (locals.var_k2q2s * locals.var_k2_dn4)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2s_dn6 * locals.var_k2) - (locals.var_k2q2s * locals.var_k2_dn6)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2s_dn7 * locals.var_k2) - (locals.var_k2q2s * locals.var_k2_dn7)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2s_dn8 * locals.var_k2) - (locals.var_k2q2s * locals.var_k2_dn8)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2s_dn9 * locals.var_k2) - (locals.var_k2q2s * locals.var_k2_dn9)) / (locals.var_k2 * locals.var_k2)),)
    } else {
        (locals.var_q2s, locals.var_q2s_dn4, locals.var_q2s_dn6, locals.var_q2s_dn7, locals.var_q2s_dn8, locals.var_q2s_dn9,)
    }
};
        locals.var_q2s = assign15550_e15426;
        locals.var_q2s_dn4 = assign15550_e15426_d_n4;
        locals.var_q2s_dn6 = assign15550_e15426_d_n6;
        locals.var_q2s_dn7 = assign15550_e15426_d_n7;
        locals.var_q2s_dn8 = assign15550_e15426_d_n8;
        locals.var_q2s_dn9 = assign15550_e15426_d_n9;

        let assign15560_e15429: f64 = (locals.var_xg2x - locals.var_q2s);
        let assign15560_e15431: f64 = assign15560_e15429;
        let assign15560_e15433: f64 = if assign15560_e15431 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard599 = assign15560_e15433;

        let (assign15570_e15442, assign15570_e15442_d_n4, assign15570_e15442_d_n6, assign15570_e15442_d_n7, assign15570_e15442_d_n8, assign15570_e15442_d_n9,) = {
    if (locals.var_guard599 != 0.0) {
        let assign15570_e15437: f64 = (locals.var_xg2x - locals.var_q2s);
        let assign15570_e15439: f64 = assign15570_e15437;
        let assign15570_e15440: f64 = (assign15570_e15439).exp();
        (assign15570_e15440, (assign15570_e15440 * (locals.var_xg2x_dn4 - locals.var_q2s_dn4)), (assign15570_e15440 * (locals.var_xg2x_dn6 - locals.var_q2s_dn6)), (assign15570_e15440 * (locals.var_xg2x_dn7 - locals.var_q2s_dn7)), (assign15570_e15440 * (locals.var_xg2x_dn8 - locals.var_q2s_dn8)), (assign15570_e15440 * (locals.var_xg2x_dn9 - locals.var_q2s_dn9)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign15570_e15442;
        locals.var_q_temp1_dn4 = assign15570_e15442_d_n4;
        locals.var_q_temp1_dn6 = assign15570_e15442_d_n6;
        locals.var_q_temp1_dn7 = assign15570_e15442_d_n7;
        locals.var_q_temp1_dn8 = assign15570_e15442_d_n8;
        locals.var_q_temp1_dn9 = assign15570_e15442_d_n9;

        let (assign15580_e15481, assign15580_e15481_d_n4, assign15580_e15481_d_n6, assign15580_e15481_d_n7, assign15580_e15481_d_n8, assign15580_e15481_d_n9,) = {
    if (locals.var_guard599 == 0.0) {
        let assign15580_e15449: f64 = (locals.var_xg2x - locals.var_q2s);
        let assign15580_e15451: f64 = assign15580_e15449;
        let assign15580_e15453: f64 = (assign15580_e15451 - 80.0);
        let assign15580_e15458: f64 = (locals.var_xg2x - locals.var_q2s);
        let assign15580_e15460: f64 = assign15580_e15458;
        let assign15580_e15462: f64 = (assign15580_e15460 - 80.0);
        let assign15580_e15463: f64 = (0.5 * assign15580_e15462);
        let assign15580_e15467: f64 = (locals.var_xg2x - locals.var_q2s);
        let assign15580_e15469: f64 = assign15580_e15467;
        let assign15580_e15471: f64 = (assign15580_e15469 - 80.0);
        let assign15580_e15473: f64 = (assign15580_e15471 * 0.3333333333333);
        let assign15580_e15474: f64 = (1.0 + assign15580_e15473);
        let assign15580_e15475: f64 = (assign15580_e15463 * assign15580_e15474);
        let assign15580_e15476: f64 = (1.0 + assign15580_e15475);
        let assign15580_e15477: f64 = (assign15580_e15453 * assign15580_e15476);
        let assign15580_e15478: f64 = (1.0 + assign15580_e15477);
        let assign15580_e15479: f64 = (5.54062e34 * assign15580_e15478);
        (assign15580_e15479, (5.54062e34 * (((locals.var_xg2x_dn4 - locals.var_q2s_dn4) * assign15580_e15476) + (assign15580_e15453 * (((0.5 * (locals.var_xg2x_dn4 - locals.var_q2s_dn4)) * assign15580_e15474) + (assign15580_e15463 * ((locals.var_xg2x_dn4 - locals.var_q2s_dn4) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg2x_dn6 - locals.var_q2s_dn6) * assign15580_e15476) + (assign15580_e15453 * (((0.5 * (locals.var_xg2x_dn6 - locals.var_q2s_dn6)) * assign15580_e15474) + (assign15580_e15463 * ((locals.var_xg2x_dn6 - locals.var_q2s_dn6) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg2x_dn7 - locals.var_q2s_dn7) * assign15580_e15476) + (assign15580_e15453 * (((0.5 * (locals.var_xg2x_dn7 - locals.var_q2s_dn7)) * assign15580_e15474) + (assign15580_e15463 * ((locals.var_xg2x_dn7 - locals.var_q2s_dn7) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg2x_dn8 - locals.var_q2s_dn8) * assign15580_e15476) + (assign15580_e15453 * (((0.5 * (locals.var_xg2x_dn8 - locals.var_q2s_dn8)) * assign15580_e15474) + (assign15580_e15463 * ((locals.var_xg2x_dn8 - locals.var_q2s_dn8) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg2x_dn9 - locals.var_q2s_dn9) * assign15580_e15476) + (assign15580_e15453 * (((0.5 * (locals.var_xg2x_dn9 - locals.var_q2s_dn9)) * assign15580_e15474) + (assign15580_e15463 * ((locals.var_xg2x_dn9 - locals.var_q2s_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign15580_e15481;
        locals.var_q_temp1_dn4 = assign15580_e15481_d_n4;
        locals.var_q_temp1_dn6 = assign15580_e15481_d_n6;
        locals.var_q_temp1_dn7 = assign15580_e15481_d_n7;
        locals.var_q_temp1_dn8 = assign15580_e15481_d_n8;
        locals.var_q_temp1_dn9 = assign15580_e15481_d_n9;

        let assign15590_e15484: f64 = (locals.var_a0 * locals.var_q_temp1);
        locals.var_aexp2s = assign15590_e15484;
        locals.var_aexp2s_dn4 = ((locals.var_a0_dn4 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn4));
        locals.var_aexp2s_dn6 = ((locals.var_a0_dn6 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn6));
        locals.var_aexp2s_dn7 = ((locals.var_a0_dn7 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn7));
        locals.var_aexp2s_dn8 = ((locals.var_a0_dn8 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn8));
        locals.var_aexp2s_dn9 = ((locals.var_a0_dn9 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn9));

        locals.var_a1s = 0.0;
        locals.var_a1s_dn4 = 0.0;
        locals.var_a1s_dn6 = 0.0;
        locals.var_a1s_dn7 = 0.0;
        locals.var_a1s_dn8 = 0.0;
        locals.var_a1s_dn9 = 0.0;

        locals.var_a2s = 0.0;
        locals.var_a2s_dn4 = 0.0;
        locals.var_a2s_dn6 = 0.0;
        locals.var_a2s_dn7 = 0.0;
        locals.var_a2s_dn8 = 0.0;
        locals.var_a2s_dn9 = 0.0;

        locals.var_b1s = 0.0;
        locals.var_b1s_dn4 = 0.0;
        locals.var_b1s_dn6 = 0.0;
        locals.var_b1s_dn7 = 0.0;
        locals.var_b1s_dn8 = 0.0;
        locals.var_b1s_dn9 = 0.0;

        locals.var_b2s = 0.0;
        locals.var_b2s_dn4 = 0.0;
        locals.var_b2s_dn6 = 0.0;
        locals.var_b2s_dn7 = 0.0;
        locals.var_b2s_dn8 = 0.0;
        locals.var_b2s_dn9 = 0.0;

        locals.var_sums = 0.0;
        locals.var_sums_dn4 = 0.0;
        locals.var_sums_dn6 = 0.0;
        locals.var_sums_dn7 = 0.0;
        locals.var_sums_dn8 = 0.0;
        locals.var_sums_dn9 = 0.0;

        locals.var_dqsqs_dxn_qi = 0.0;
        locals.var_dqsqs_dxn_qi_dn4 = 0.0;
        locals.var_dqsqs_dxn_qi_dn6 = 0.0;
        locals.var_dqsqs_dxn_qi_dn7 = 0.0;
        locals.var_dqsqs_dxn_qi_dn8 = 0.0;
        locals.var_dqsqs_dxn_qi_dn9 = 0.0;

        let assign15660_e15493: f64 = if locals.var_qis > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard600 = assign15660_e15493;

        let (assign15670_e15499, assign15670_e15499_d_n4, assign15670_e15499_d_n6, assign15670_e15499_d_n7, assign15670_e15499_d_n8, assign15670_e15499_d_n9,) = {
    if (locals.var_guard600 != 0.0) {
        let assign15670_e15497: f64 = (locals.var_aexp1s * locals.var_inv_k1);
        (assign15670_e15497, ((locals.var_aexp1s_dn4 * locals.var_inv_k1) + (locals.var_aexp1s * locals.var_inv_k1_dn4)), ((locals.var_aexp1s_dn6 * locals.var_inv_k1) + (locals.var_aexp1s * locals.var_inv_k1_dn6)), ((locals.var_aexp1s_dn7 * locals.var_inv_k1) + (locals.var_aexp1s * locals.var_inv_k1_dn7)), ((locals.var_aexp1s_dn8 * locals.var_inv_k1) + (locals.var_aexp1s * locals.var_inv_k1_dn8)), ((locals.var_aexp1s_dn9 * locals.var_inv_k1) + (locals.var_aexp1s * locals.var_inv_k1_dn9)),)
    } else {
        (locals.var_b1s, locals.var_b1s_dn4, locals.var_b1s_dn6, locals.var_b1s_dn7, locals.var_b1s_dn8, locals.var_b1s_dn9,)
    }
};
        locals.var_b1s = assign15670_e15499;
        locals.var_b1s_dn4 = assign15670_e15499_d_n4;
        locals.var_b1s_dn6 = assign15670_e15499_d_n6;
        locals.var_b1s_dn7 = assign15670_e15499_d_n7;
        locals.var_b1s_dn8 = assign15670_e15499_d_n8;
        locals.var_b1s_dn9 = assign15670_e15499_d_n9;

        let (assign15680_e15505, assign15680_e15505_d_n4, assign15680_e15505_d_n6, assign15680_e15505_d_n7, assign15680_e15505_d_n8, assign15680_e15505_d_n9,) = {
    if (locals.var_guard600 != 0.0) {
        let assign15680_e15503: f64 = (locals.var_aexp2s * locals.var_inv_k2);
        (assign15680_e15503, ((locals.var_aexp2s_dn4 * locals.var_inv_k2) + (locals.var_aexp2s * locals.var_inv_k2_dn4)), ((locals.var_aexp2s_dn6 * locals.var_inv_k2) + (locals.var_aexp2s * locals.var_inv_k2_dn6)), ((locals.var_aexp2s_dn7 * locals.var_inv_k2) + (locals.var_aexp2s * locals.var_inv_k2_dn7)), ((locals.var_aexp2s_dn8 * locals.var_inv_k2) + (locals.var_aexp2s * locals.var_inv_k2_dn8)), ((locals.var_aexp2s_dn9 * locals.var_inv_k2) + (locals.var_aexp2s * locals.var_inv_k2_dn9)),)
    } else {
        (locals.var_b2s, locals.var_b2s_dn4, locals.var_b2s_dn6, locals.var_b2s_dn7, locals.var_b2s_dn8, locals.var_b2s_dn9,)
    }
};
        locals.var_b2s = assign15680_e15505;
        locals.var_b2s_dn4 = assign15680_e15505_d_n4;
        locals.var_b2s_dn6 = assign15680_e15505_d_n6;
        locals.var_b2s_dn7 = assign15680_e15505_d_n7;
        locals.var_b2s_dn8 = assign15680_e15505_d_n8;
        locals.var_b2s_dn9 = assign15680_e15505_d_n9;

        let (assign15690_e15513, assign15690_e15513_d_n4, assign15690_e15513_d_n6, assign15690_e15513_d_n7, assign15690_e15513_d_n8, assign15690_e15513_d_n9,) = {
    if (locals.var_guard600 != 0.0) {
        let assign15690_e15510: f64 = (2.0 * locals.var_k1q1s);
        let assign15690_e15511: f64 = (locals.var_b1s + assign15690_e15510);
        (assign15690_e15511, (locals.var_b1s_dn4 + (2.0 * locals.var_k1q1s_dn4)), (locals.var_b1s_dn6 + (2.0 * locals.var_k1q1s_dn6)), (locals.var_b1s_dn7 + (2.0 * locals.var_k1q1s_dn7)), (locals.var_b1s_dn8 + (2.0 * locals.var_k1q1s_dn8)), (locals.var_b1s_dn9 + (2.0 * locals.var_k1q1s_dn9)),)
    } else {
        (locals.var_a1s, locals.var_a1s_dn4, locals.var_a1s_dn6, locals.var_a1s_dn7, locals.var_a1s_dn8, locals.var_a1s_dn9,)
    }
};
        locals.var_a1s = assign15690_e15513;
        locals.var_a1s_dn4 = assign15690_e15513_d_n4;
        locals.var_a1s_dn6 = assign15690_e15513_d_n6;
        locals.var_a1s_dn7 = assign15690_e15513_d_n7;
        locals.var_a1s_dn8 = assign15690_e15513_d_n8;
        locals.var_a1s_dn9 = assign15690_e15513_d_n9;

        let (assign15700_e15521, assign15700_e15521_d_n4, assign15700_e15521_d_n6, assign15700_e15521_d_n7, assign15700_e15521_d_n8, assign15700_e15521_d_n9,) = {
    if (locals.var_guard600 != 0.0) {
        let assign15700_e15518: f64 = (2.0 * locals.var_k2q2s);
        let assign15700_e15519: f64 = (locals.var_b2s + assign15700_e15518);
        (assign15700_e15519, (locals.var_b2s_dn4 + (2.0 * locals.var_k2q2s_dn4)), (locals.var_b2s_dn6 + (2.0 * locals.var_k2q2s_dn6)), (locals.var_b2s_dn7 + (2.0 * locals.var_k2q2s_dn7)), (locals.var_b2s_dn8 + (2.0 * locals.var_k2q2s_dn8)), (locals.var_b2s_dn9 + (2.0 * locals.var_k2q2s_dn9)),)
    } else {
        (locals.var_a2s, locals.var_a2s_dn4, locals.var_a2s_dn6, locals.var_a2s_dn7, locals.var_a2s_dn8, locals.var_a2s_dn9,)
    }
};
        locals.var_a2s = assign15700_e15521;
        locals.var_a2s_dn4 = assign15700_e15521_d_n4;
        locals.var_a2s_dn6 = assign15700_e15521_d_n6;
        locals.var_a2s_dn7 = assign15700_e15521_d_n7;
        locals.var_a2s_dn8 = assign15700_e15521_d_n8;
        locals.var_a2s_dn9 = assign15700_e15521_d_n9;

        let (assign15710_e15531, assign15710_e15531_d_n4, assign15710_e15531_d_n6, assign15710_e15531_d_n7, assign15710_e15531_d_n8, assign15710_e15531_d_n9,) = {
    if (locals.var_guard600 != 0.0) {
        let assign15710_e15525: f64 = (2.0 * locals.var_qis);
        let assign15710_e15527: f64 = (assign15710_e15525 + locals.var_b1s);
        let assign15710_e15529: f64 = (assign15710_e15527 + locals.var_b2s);
        (assign15710_e15529, (((2.0 * locals.var_qis_dn4) + locals.var_b1s_dn4) + locals.var_b2s_dn4), (((2.0 * locals.var_qis_dn6) + locals.var_b1s_dn6) + locals.var_b2s_dn6), (((2.0 * locals.var_qis_dn7) + locals.var_b1s_dn7) + locals.var_b2s_dn7), (((2.0 * locals.var_qis_dn8) + locals.var_b1s_dn8) + locals.var_b2s_dn8), (((2.0 * locals.var_qis_dn9) + locals.var_b1s_dn9) + locals.var_b2s_dn9),)
    } else {
        (locals.var_sums, locals.var_sums_dn4, locals.var_sums_dn6, locals.var_sums_dn7, locals.var_sums_dn8, locals.var_sums_dn9,)
    }
};
        locals.var_sums = assign15710_e15531;
        locals.var_sums_dn4 = assign15710_e15531_d_n4;
        locals.var_sums_dn6 = assign15710_e15531_d_n6;
        locals.var_sums_dn7 = assign15710_e15531_d_n7;
        locals.var_sums_dn8 = assign15710_e15531_d_n8;
        locals.var_sums_dn9 = assign15710_e15531_d_n9;

        let assign15720_e15533: f64 = (locals.var_qsqs).abs();
        let assign15720_e15535: f64 = if assign15720_e15533 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard601 = assign15720_e15535;

        let (assign15730_e15559, assign15730_e15559_d_n4, assign15730_e15559_d_n6, assign15730_e15559_d_n7, assign15730_e15559_d_n8, assign15730_e15559_d_n9,) = {
    if ((locals.var_guard600 != 0.0) && (locals.var_guard601 != 0.0)) {
        let assign15730_e15541: f64 = (locals.var_a1s * locals.var_a2s);
        let assign15730_e15545: f64 = (locals.var_q1s + 2.0);
        let assign15730_e15546: f64 = (2.0 * assign15730_e15545);
        let assign15730_e15548: f64 = (assign15730_e15546 * locals.var_a2s);
        let assign15730_e15549: f64 = (assign15730_e15541 + assign15730_e15548);
        let assign15730_e15553: f64 = (locals.var_q2s + 2.0);
        let assign15730_e15554: f64 = (2.0 * assign15730_e15553);
        let assign15730_e15556: f64 = (assign15730_e15554 * locals.var_a1s);
        let assign15730_e15557: f64 = (assign15730_e15549 + assign15730_e15556);
        (assign15730_e15557, ((((locals.var_a1s_dn4 * locals.var_a2s) + (locals.var_a1s * locals.var_a2s_dn4)) + (((2.0 * locals.var_q1s_dn4) * locals.var_a2s) + (assign15730_e15546 * locals.var_a2s_dn4))) + (((2.0 * locals.var_q2s_dn4) * locals.var_a1s) + (assign15730_e15554 * locals.var_a1s_dn4))), ((((locals.var_a1s_dn6 * locals.var_a2s) + (locals.var_a1s * locals.var_a2s_dn6)) + (((2.0 * locals.var_q1s_dn6) * locals.var_a2s) + (assign15730_e15546 * locals.var_a2s_dn6))) + (((2.0 * locals.var_q2s_dn6) * locals.var_a1s) + (assign15730_e15554 * locals.var_a1s_dn6))), ((((locals.var_a1s_dn7 * locals.var_a2s) + (locals.var_a1s * locals.var_a2s_dn7)) + (((2.0 * locals.var_q1s_dn7) * locals.var_a2s) + (assign15730_e15546 * locals.var_a2s_dn7))) + (((2.0 * locals.var_q2s_dn7) * locals.var_a1s) + (assign15730_e15554 * locals.var_a1s_dn7))), ((((locals.var_a1s_dn8 * locals.var_a2s) + (locals.var_a1s * locals.var_a2s_dn8)) + (((2.0 * locals.var_q1s_dn8) * locals.var_a2s) + (assign15730_e15546 * locals.var_a2s_dn8))) + (((2.0 * locals.var_q2s_dn8) * locals.var_a1s) + (assign15730_e15554 * locals.var_a1s_dn8))), ((((locals.var_a1s_dn9 * locals.var_a2s) + (locals.var_a1s * locals.var_a2s_dn9)) + (((2.0 * locals.var_q1s_dn9) * locals.var_a2s) + (assign15730_e15546 * locals.var_a2s_dn9))) + (((2.0 * locals.var_q2s_dn9) * locals.var_a1s) + (assign15730_e15554 * locals.var_a1s_dn9))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign15730_e15559;
        locals.var_temp1_dn4 = assign15730_e15559_d_n4;
        locals.var_temp1_dn6 = assign15730_e15559_d_n6;
        locals.var_temp1_dn7 = assign15730_e15559_d_n7;
        locals.var_temp1_dn8 = assign15730_e15559_d_n8;
        locals.var_temp1_dn9 = assign15730_e15559_d_n9;

        let (assign15740_e15574, assign15740_e15574_d_n4, assign15740_e15574_d_n6, assign15740_e15574_d_n7, assign15740_e15574_d_n8, assign15740_e15574_d_n9,) = {
    if ((locals.var_guard600 != 0.0) && (locals.var_guard601 != 0.0)) {
        let assign15740_e15564: f64 = (-4.0);
        let assign15740_e15566: f64 = (assign15740_e15564 * locals.var_qsqs);
        let assign15740_e15568: f64 = (assign15740_e15566 * locals.var_sums);
        let assign15740_e15571: f64 = (locals.var_qis * locals.var_temp1);
        let assign15740_e15572: f64 = (assign15740_e15568 / assign15740_e15571);
        (assign15740_e15572, ((((((assign15740_e15564 * locals.var_qsqs_dn4) * locals.var_sums) + (assign15740_e15566 * locals.var_sums_dn4)) * assign15740_e15571) - (assign15740_e15568 * ((locals.var_qis_dn4 * locals.var_temp1) + (locals.var_qis * locals.var_temp1_dn4)))) / (assign15740_e15571 * assign15740_e15571)), ((((((assign15740_e15564 * locals.var_qsqs_dn6) * locals.var_sums) + (assign15740_e15566 * locals.var_sums_dn6)) * assign15740_e15571) - (assign15740_e15568 * ((locals.var_qis_dn6 * locals.var_temp1) + (locals.var_qis * locals.var_temp1_dn6)))) / (assign15740_e15571 * assign15740_e15571)), ((((((assign15740_e15564 * locals.var_qsqs_dn7) * locals.var_sums) + (assign15740_e15566 * locals.var_sums_dn7)) * assign15740_e15571) - (assign15740_e15568 * ((locals.var_qis_dn7 * locals.var_temp1) + (locals.var_qis * locals.var_temp1_dn7)))) / (assign15740_e15571 * assign15740_e15571)), ((((((assign15740_e15564 * locals.var_qsqs_dn8) * locals.var_sums) + (assign15740_e15566 * locals.var_sums_dn8)) * assign15740_e15571) - (assign15740_e15568 * ((locals.var_qis_dn8 * locals.var_temp1) + (locals.var_qis * locals.var_temp1_dn8)))) / (assign15740_e15571 * assign15740_e15571)), ((((((assign15740_e15564 * locals.var_qsqs_dn9) * locals.var_sums) + (assign15740_e15566 * locals.var_sums_dn9)) * assign15740_e15571) - (assign15740_e15568 * ((locals.var_qis_dn9 * locals.var_temp1) + (locals.var_qis * locals.var_temp1_dn9)))) / (assign15740_e15571 * assign15740_e15571)),)
    } else {
        (locals.var_dqsqs_dxn_qi, locals.var_dqsqs_dxn_qi_dn4, locals.var_dqsqs_dxn_qi_dn6, locals.var_dqsqs_dxn_qi_dn7, locals.var_dqsqs_dxn_qi_dn8, locals.var_dqsqs_dxn_qi_dn9,)
    }
};
        locals.var_dqsqs_dxn_qi = assign15740_e15574;
        locals.var_dqsqs_dxn_qi_dn4 = assign15740_e15574_d_n4;
        locals.var_dqsqs_dxn_qi_dn6 = assign15740_e15574_d_n6;
        locals.var_dqsqs_dxn_qi_dn7 = assign15740_e15574_d_n7;
        locals.var_dqsqs_dxn_qi_dn8 = assign15740_e15574_d_n8;
        locals.var_dqsqs_dxn_qi_dn9 = assign15740_e15574_d_n9;

        let (assign15750_e15599, assign15750_e15599_d_n4, assign15750_e15599_d_n6, assign15750_e15599_d_n7, assign15750_e15599_d_n8, assign15750_e15599_d_n9,) = {
    if ((locals.var_guard600 != 0.0) && (locals.var_guard601 == 0.0)) {
        let assign15750_e15583: f64 = (locals.var_qsqs * 0.0333333333333);
        let assign15750_e15587: f64 = (locals.var_qsqs * 0.0357142857143);
        let assign15750_e15591: f64 = (locals.var_qsqs * 0.0333333333333);
        let assign15750_e15592: f64 = (1.0 - assign15750_e15591);
        let assign15750_e15593: f64 = (assign15750_e15587 * assign15750_e15592);
        let assign15750_e15594: f64 = (1.0 - assign15750_e15593);
        let assign15750_e15595: f64 = (assign15750_e15583 * assign15750_e15594);
        let assign15750_e15596: f64 = (1.0 - assign15750_e15595);
        let assign15750_e15597: f64 = (0.1666666666667 * assign15750_e15596);
        (assign15750_e15597, (0.1666666666667 * (-(((locals.var_qsqs_dn4 * 0.0333333333333) * assign15750_e15594) + (assign15750_e15583 * (-(((locals.var_qsqs_dn4 * 0.0357142857143) * assign15750_e15592) + (assign15750_e15587 * (-(locals.var_qsqs_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_qsqs_dn6 * 0.0333333333333) * assign15750_e15594) + (assign15750_e15583 * (-(((locals.var_qsqs_dn6 * 0.0357142857143) * assign15750_e15592) + (assign15750_e15587 * (-(locals.var_qsqs_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_qsqs_dn7 * 0.0333333333333) * assign15750_e15594) + (assign15750_e15583 * (-(((locals.var_qsqs_dn7 * 0.0357142857143) * assign15750_e15592) + (assign15750_e15587 * (-(locals.var_qsqs_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_qsqs_dn8 * 0.0333333333333) * assign15750_e15594) + (assign15750_e15583 * (-(((locals.var_qsqs_dn8 * 0.0357142857143) * assign15750_e15592) + (assign15750_e15587 * (-(locals.var_qsqs_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_qsqs_dn9 * 0.0333333333333) * assign15750_e15594) + (assign15750_e15583 * (-(((locals.var_qsqs_dn9 * 0.0357142857143) * assign15750_e15592) + (assign15750_e15587 * (-(locals.var_qsqs_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign15750_e15599;
        locals.var_temp1_dn4 = assign15750_e15599_d_n4;
        locals.var_temp1_dn6 = assign15750_e15599_d_n6;
        locals.var_temp1_dn7 = assign15750_e15599_d_n7;
        locals.var_temp1_dn8 = assign15750_e15599_d_n8;
        locals.var_temp1_dn9 = assign15750_e15599_d_n9;

        let (assign15760_e15624, assign15760_e15624_d_n4, assign15760_e15624_d_n6, assign15760_e15624_d_n7, assign15760_e15624_d_n8, assign15760_e15624_d_n9,) = {
    if ((locals.var_guard600 != 0.0) && (locals.var_guard601 == 0.0)) {
        let assign15760_e15606: f64 = (locals.var_a1s * locals.var_aexp1s);
        let assign15760_e15609: f64 = (locals.var_a2s * locals.var_aexp2s);
        let assign15760_e15610: f64 = (assign15760_e15606 + assign15760_e15609);
        let assign15760_e15613: f64 = (locals.var_a1s * locals.var_a2s);
        let assign15760_e15615: f64 = (assign15760_e15613 * locals.var_qis);
        let assign15760_e15619: f64 = (locals.var_qis * locals.var_temp1);
        let assign15760_e15620: f64 = (1.0 + assign15760_e15619);
        let assign15760_e15621: f64 = (assign15760_e15615 * assign15760_e15620);
        let assign15760_e15622: f64 = (assign15760_e15610 + assign15760_e15621);
        (assign15760_e15622, ((((locals.var_a1s_dn4 * locals.var_aexp1s) + (locals.var_a1s * locals.var_aexp1s_dn4)) + ((locals.var_a2s_dn4 * locals.var_aexp2s) + (locals.var_a2s * locals.var_aexp2s_dn4))) + ((((((locals.var_a1s_dn4 * locals.var_a2s) + (locals.var_a1s * locals.var_a2s_dn4)) * locals.var_qis) + (assign15760_e15613 * locals.var_qis_dn4)) * assign15760_e15620) + (assign15760_e15615 * ((locals.var_qis_dn4 * locals.var_temp1) + (locals.var_qis * locals.var_temp1_dn4))))), ((((locals.var_a1s_dn6 * locals.var_aexp1s) + (locals.var_a1s * locals.var_aexp1s_dn6)) + ((locals.var_a2s_dn6 * locals.var_aexp2s) + (locals.var_a2s * locals.var_aexp2s_dn6))) + ((((((locals.var_a1s_dn6 * locals.var_a2s) + (locals.var_a1s * locals.var_a2s_dn6)) * locals.var_qis) + (assign15760_e15613 * locals.var_qis_dn6)) * assign15760_e15620) + (assign15760_e15615 * ((locals.var_qis_dn6 * locals.var_temp1) + (locals.var_qis * locals.var_temp1_dn6))))), ((((locals.var_a1s_dn7 * locals.var_aexp1s) + (locals.var_a1s * locals.var_aexp1s_dn7)) + ((locals.var_a2s_dn7 * locals.var_aexp2s) + (locals.var_a2s * locals.var_aexp2s_dn7))) + ((((((locals.var_a1s_dn7 * locals.var_a2s) + (locals.var_a1s * locals.var_a2s_dn7)) * locals.var_qis) + (assign15760_e15613 * locals.var_qis_dn7)) * assign15760_e15620) + (assign15760_e15615 * ((locals.var_qis_dn7 * locals.var_temp1) + (locals.var_qis * locals.var_temp1_dn7))))), ((((locals.var_a1s_dn8 * locals.var_aexp1s) + (locals.var_a1s * locals.var_aexp1s_dn8)) + ((locals.var_a2s_dn8 * locals.var_aexp2s) + (locals.var_a2s * locals.var_aexp2s_dn8))) + ((((((locals.var_a1s_dn8 * locals.var_a2s) + (locals.var_a1s * locals.var_a2s_dn8)) * locals.var_qis) + (assign15760_e15613 * locals.var_qis_dn8)) * assign15760_e15620) + (assign15760_e15615 * ((locals.var_qis_dn8 * locals.var_temp1) + (locals.var_qis * locals.var_temp1_dn8))))), ((((locals.var_a1s_dn9 * locals.var_aexp1s) + (locals.var_a1s * locals.var_aexp1s_dn9)) + ((locals.var_a2s_dn9 * locals.var_aexp2s) + (locals.var_a2s * locals.var_aexp2s_dn9))) + ((((((locals.var_a1s_dn9 * locals.var_a2s) + (locals.var_a1s * locals.var_a2s_dn9)) * locals.var_qis) + (assign15760_e15613 * locals.var_qis_dn9)) * assign15760_e15620) + (assign15760_e15615 * ((locals.var_qis_dn9 * locals.var_temp1) + (locals.var_qis * locals.var_temp1_dn9))))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign15760_e15624;
        locals.var_temp2_dn4 = assign15760_e15624_d_n4;
        locals.var_temp2_dn6 = assign15760_e15624_d_n6;
        locals.var_temp2_dn7 = assign15760_e15624_d_n7;
        locals.var_temp2_dn8 = assign15760_e15624_d_n8;
        locals.var_temp2_dn9 = assign15760_e15624_d_n9;

        let (assign15770_e15639, assign15770_e15639_d_n4, assign15770_e15639_d_n6, assign15770_e15639_d_n7, assign15770_e15639_d_n8, assign15770_e15639_d_n9,) = {
    if ((locals.var_guard600 != 0.0) && (locals.var_guard601 == 0.0)) {
        let assign15770_e15631: f64 = (locals.var_aexp1s * locals.var_aexp2s);
        let assign15770_e15633: f64 = (assign15770_e15631 * locals.var_sums);
        let assign15770_e15636: f64 = (locals.var_qis * locals.var_temp2);
        let assign15770_e15637: f64 = (assign15770_e15633 / assign15770_e15636);
        (assign15770_e15637, (((((((locals.var_aexp1s_dn4 * locals.var_aexp2s) + (locals.var_aexp1s * locals.var_aexp2s_dn4)) * locals.var_sums) + (assign15770_e15631 * locals.var_sums_dn4)) * assign15770_e15636) - (assign15770_e15633 * ((locals.var_qis_dn4 * locals.var_temp2) + (locals.var_qis * locals.var_temp2_dn4)))) / (assign15770_e15636 * assign15770_e15636)), (((((((locals.var_aexp1s_dn6 * locals.var_aexp2s) + (locals.var_aexp1s * locals.var_aexp2s_dn6)) * locals.var_sums) + (assign15770_e15631 * locals.var_sums_dn6)) * assign15770_e15636) - (assign15770_e15633 * ((locals.var_qis_dn6 * locals.var_temp2) + (locals.var_qis * locals.var_temp2_dn6)))) / (assign15770_e15636 * assign15770_e15636)), (((((((locals.var_aexp1s_dn7 * locals.var_aexp2s) + (locals.var_aexp1s * locals.var_aexp2s_dn7)) * locals.var_sums) + (assign15770_e15631 * locals.var_sums_dn7)) * assign15770_e15636) - (assign15770_e15633 * ((locals.var_qis_dn7 * locals.var_temp2) + (locals.var_qis * locals.var_temp2_dn7)))) / (assign15770_e15636 * assign15770_e15636)), (((((((locals.var_aexp1s_dn8 * locals.var_aexp2s) + (locals.var_aexp1s * locals.var_aexp2s_dn8)) * locals.var_sums) + (assign15770_e15631 * locals.var_sums_dn8)) * assign15770_e15636) - (assign15770_e15633 * ((locals.var_qis_dn8 * locals.var_temp2) + (locals.var_qis * locals.var_temp2_dn8)))) / (assign15770_e15636 * assign15770_e15636)), (((((((locals.var_aexp1s_dn9 * locals.var_aexp2s) + (locals.var_aexp1s * locals.var_aexp2s_dn9)) * locals.var_sums) + (assign15770_e15631 * locals.var_sums_dn9)) * assign15770_e15636) - (assign15770_e15633 * ((locals.var_qis_dn9 * locals.var_temp2) + (locals.var_qis * locals.var_temp2_dn9)))) / (assign15770_e15636 * assign15770_e15636)),)
    } else {
        (locals.var_dqsqs_dxn_qi, locals.var_dqsqs_dxn_qi_dn4, locals.var_dqsqs_dxn_qi_dn6, locals.var_dqsqs_dxn_qi_dn7, locals.var_dqsqs_dxn_qi_dn8, locals.var_dqsqs_dxn_qi_dn9,)
    }
};
        locals.var_dqsqs_dxn_qi = assign15770_e15639;
        locals.var_dqsqs_dxn_qi_dn4 = assign15770_e15639_d_n4;
        locals.var_dqsqs_dxn_qi_dn6 = assign15770_e15639_d_n6;
        locals.var_dqsqs_dxn_qi_dn7 = assign15770_e15639_d_n7;
        locals.var_dqsqs_dxn_qi_dn8 = assign15770_e15639_d_n8;
        locals.var_dqsqs_dxn_qi_dn9 = assign15770_e15639_d_n9;

        let assign15780_e15641: f64 = (locals.var_qis).ln();
        locals.var_xdrifts = assign15780_e15641;
        locals.var_xdrifts_dn4 = (locals.var_qis_dn4 / locals.var_qis);
        locals.var_xdrifts_dn6 = (locals.var_qis_dn6 / locals.var_qis);
        locals.var_xdrifts_dn7 = (locals.var_qis_dn7 / locals.var_qis);
        locals.var_xdrifts_dn8 = (locals.var_qis_dn8 / locals.var_qis);
        locals.var_xdrifts_dn9 = (locals.var_qis_dn9 / locals.var_qis);

        let assign15790_e15644: f64 = (locals.var_k1q1s / 2.0);
        let assign15790_e15646: f64 = if assign15790_e15644 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard602 = assign15790_e15646;

        let (assign15800_e15656, assign15800_e15656_d_n4, assign15800_e15656_d_n6, assign15800_e15656_d_n7, assign15800_e15656_d_n8, assign15800_e15656_d_n9,) = {
    if (locals.var_guard602 != 0.0) {
        let assign15800_e15651: f64 = (locals.var_k1q1s / 2.0);
        let assign15800_e15652: f64 = (assign15800_e15651).exp();
        let assign15800_e15653: f64 = (1.0 + assign15800_e15652);
        let assign15800_e15654: f64 = (assign15800_e15653).ln();
        (assign15800_e15654, ((assign15800_e15652 * (locals.var_k1q1s_dn4 / 2.0)) / assign15800_e15653), ((assign15800_e15652 * (locals.var_k1q1s_dn6 / 2.0)) / assign15800_e15653), ((assign15800_e15652 * (locals.var_k1q1s_dn7 / 2.0)) / assign15800_e15653), ((assign15800_e15652 * (locals.var_k1q1s_dn8 / 2.0)) / assign15800_e15653), ((assign15800_e15652 * (locals.var_k1q1s_dn9 / 2.0)) / assign15800_e15653),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign15800_e15656;
        locals.var_temp1_dn4 = assign15800_e15656_d_n4;
        locals.var_temp1_dn6 = assign15800_e15656_d_n6;
        locals.var_temp1_dn7 = assign15800_e15656_d_n7;
        locals.var_temp1_dn8 = assign15800_e15656_d_n8;
        locals.var_temp1_dn9 = assign15800_e15656_d_n9;

        let (assign15810_e15663, assign15810_e15663_d_n4, assign15810_e15663_d_n6, assign15810_e15663_d_n7, assign15810_e15663_d_n8, assign15810_e15663_d_n9,) = {
    if (locals.var_guard602 == 0.0) {
        let assign15810_e15661: f64 = (locals.var_k1q1s / 2.0);
        (assign15810_e15661, (locals.var_k1q1s_dn4 / 2.0), (locals.var_k1q1s_dn6 / 2.0), (locals.var_k1q1s_dn7 / 2.0), (locals.var_k1q1s_dn8 / 2.0), (locals.var_k1q1s_dn9 / 2.0),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign15810_e15663;
        locals.var_temp1_dn4 = assign15810_e15663_d_n4;
        locals.var_temp1_dn6 = assign15810_e15663_d_n6;
        locals.var_temp1_dn7 = assign15810_e15663_d_n7;
        locals.var_temp1_dn8 = assign15810_e15663_d_n8;
        locals.var_temp1_dn9 = assign15810_e15663_d_n9;

        let assign15820_e15666: f64 = (2.0 * locals.var_temp1);
        locals.var_esurf1s = assign15820_e15666;
        locals.var_esurf1s_dn4 = (2.0 * locals.var_temp1_dn4);
        locals.var_esurf1s_dn6 = (2.0 * locals.var_temp1_dn6);
        locals.var_esurf1s_dn7 = (2.0 * locals.var_temp1_dn7);
        locals.var_esurf1s_dn8 = (2.0 * locals.var_temp1_dn8);
        locals.var_esurf1s_dn9 = (2.0 * locals.var_temp1_dn9);

        let assign15830_e15669: f64 = (locals.var_k2q2s / 2.0);
        let assign15830_e15671: f64 = if assign15830_e15669 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard603 = assign15830_e15671;

    }

    pub(super) fn stamp_transient_block_39(
        locals: &mut StampLocals,
    ) {
        let (assign15840_e15681, assign15840_e15681_d_n4, assign15840_e15681_d_n6, assign15840_e15681_d_n7, assign15840_e15681_d_n8, assign15840_e15681_d_n9,) = {
    if (locals.var_guard603 != 0.0) {
        let assign15840_e15676: f64 = (locals.var_k2q2s / 2.0);
        let assign15840_e15677: f64 = (assign15840_e15676).exp();
        let assign15840_e15678: f64 = (1.0 + assign15840_e15677);
        let assign15840_e15679: f64 = (assign15840_e15678).ln();
        (assign15840_e15679, ((assign15840_e15677 * (locals.var_k2q2s_dn4 / 2.0)) / assign15840_e15678), ((assign15840_e15677 * (locals.var_k2q2s_dn6 / 2.0)) / assign15840_e15678), ((assign15840_e15677 * (locals.var_k2q2s_dn7 / 2.0)) / assign15840_e15678), ((assign15840_e15677 * (locals.var_k2q2s_dn8 / 2.0)) / assign15840_e15678), ((assign15840_e15677 * (locals.var_k2q2s_dn9 / 2.0)) / assign15840_e15678),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign15840_e15681;
        locals.var_temp2_dn4 = assign15840_e15681_d_n4;
        locals.var_temp2_dn6 = assign15840_e15681_d_n6;
        locals.var_temp2_dn7 = assign15840_e15681_d_n7;
        locals.var_temp2_dn8 = assign15840_e15681_d_n8;
        locals.var_temp2_dn9 = assign15840_e15681_d_n9;

        let (assign15850_e15688, assign15850_e15688_d_n4, assign15850_e15688_d_n6, assign15850_e15688_d_n7, assign15850_e15688_d_n8, assign15850_e15688_d_n9,) = {
    if (locals.var_guard603 == 0.0) {
        let assign15850_e15686: f64 = (locals.var_k2q2s / 2.0);
        (assign15850_e15686, (locals.var_k2q2s_dn4 / 2.0), (locals.var_k2q2s_dn6 / 2.0), (locals.var_k2q2s_dn7 / 2.0), (locals.var_k2q2s_dn8 / 2.0), (locals.var_k2q2s_dn9 / 2.0),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign15850_e15688;
        locals.var_temp2_dn4 = assign15850_e15688_d_n4;
        locals.var_temp2_dn6 = assign15850_e15688_d_n6;
        locals.var_temp2_dn7 = assign15850_e15688_d_n7;
        locals.var_temp2_dn8 = assign15850_e15688_d_n8;
        locals.var_temp2_dn9 = assign15850_e15688_d_n9;

        let assign15860_e15691: f64 = (2.0 * locals.var_temp2);
        locals.var_esurf2s = assign15860_e15691;
        locals.var_esurf2s_dn4 = (2.0 * locals.var_temp2_dn4);
        locals.var_esurf2s_dn6 = (2.0 * locals.var_temp2_dn6);
        locals.var_esurf2s_dn7 = (2.0 * locals.var_temp2_dn7);
        locals.var_esurf2s_dn8 = (2.0 * locals.var_temp2_dn8);
        locals.var_esurf2s_dn9 = (2.0 * locals.var_temp2_dn9);

        let assign15870_e15694: f64 = (locals.var_esurf2s - locals.var_k2q2s);
        locals.var_ecpl1s = assign15870_e15694;
        locals.var_ecpl1s_dn4 = (locals.var_esurf2s_dn4 - locals.var_k2q2s_dn4);
        locals.var_ecpl1s_dn6 = (locals.var_esurf2s_dn6 - locals.var_k2q2s_dn6);
        locals.var_ecpl1s_dn7 = (locals.var_esurf2s_dn7 - locals.var_k2q2s_dn7);
        locals.var_ecpl1s_dn8 = (locals.var_esurf2s_dn8 - locals.var_k2q2s_dn8);
        locals.var_ecpl1s_dn9 = (locals.var_esurf2s_dn9 - locals.var_k2q2s_dn9);

        let assign15880_e15697: f64 = (locals.var_esurf1s - locals.var_k1q1s);
        locals.var_ecpl2s = assign15880_e15697;
        locals.var_ecpl2s_dn4 = (locals.var_esurf1s_dn4 - locals.var_k1q1s_dn4);
        locals.var_ecpl2s_dn6 = (locals.var_esurf1s_dn6 - locals.var_k1q1s_dn6);
        locals.var_ecpl2s_dn7 = (locals.var_esurf1s_dn7 - locals.var_k1q1s_dn7);
        locals.var_ecpl2s_dn8 = (locals.var_esurf1s_dn8 - locals.var_k1q1s_dn8);
        locals.var_ecpl2s_dn9 = (locals.var_esurf1s_dn9 - locals.var_k1q1s_dn9);

        let assign15890_e15700: f64 = (locals.var_eta_mu * locals.var_esurf1s);
        let assign15890_e15703: f64 = (locals.var_one_m_eta * locals.var_ecpl1s);
        let assign15890_e15704: f64 = (assign15890_e15700 + assign15890_e15703);
        locals.var_eeff1s = assign15890_e15704;
        locals.var_eeff1s_dn4 = ((locals.var_eta_mu * locals.var_esurf1s_dn4) + (locals.var_one_m_eta * locals.var_ecpl1s_dn4));
        locals.var_eeff1s_dn6 = ((locals.var_eta_mu * locals.var_esurf1s_dn6) + (locals.var_one_m_eta * locals.var_ecpl1s_dn6));
        locals.var_eeff1s_dn7 = ((locals.var_eta_mu * locals.var_esurf1s_dn7) + (locals.var_one_m_eta * locals.var_ecpl1s_dn7));
        locals.var_eeff1s_dn8 = ((locals.var_eta_mu * locals.var_esurf1s_dn8) + (locals.var_one_m_eta * locals.var_ecpl1s_dn8));
        locals.var_eeff1s_dn9 = ((locals.var_eta_mu * locals.var_esurf1s_dn9) + (locals.var_one_m_eta * locals.var_ecpl1s_dn9));

        let assign15900_e15707: f64 = (locals.var_eta_mu * locals.var_esurf2s);
        let assign15900_e15710: f64 = (locals.var_one_m_eta * locals.var_ecpl2s);
        let assign15900_e15711: f64 = (assign15900_e15707 + assign15900_e15710);
        locals.var_eeff2s = assign15900_e15711;
        locals.var_eeff2s_dn4 = ((locals.var_eta_mu * locals.var_esurf2s_dn4) + (locals.var_one_m_eta * locals.var_ecpl2s_dn4));
        locals.var_eeff2s_dn6 = ((locals.var_eta_mu * locals.var_esurf2s_dn6) + (locals.var_one_m_eta * locals.var_ecpl2s_dn6));
        locals.var_eeff2s_dn7 = ((locals.var_eta_mu * locals.var_esurf2s_dn7) + (locals.var_one_m_eta * locals.var_ecpl2s_dn7));
        locals.var_eeff2s_dn8 = ((locals.var_eta_mu * locals.var_esurf2s_dn8) + (locals.var_one_m_eta * locals.var_ecpl2s_dn8));
        locals.var_eeff2s_dn9 = ((locals.var_eta_mu * locals.var_esurf2s_dn9) + (locals.var_one_m_eta * locals.var_ecpl2s_dn9));

        let assign15910_e15715: f64 = (locals.var_esurf1s + locals.var_esurf2s);
        let assign15910_e15716: f64 = (locals.var_qis / assign15910_e15715);
        locals.var_temp = assign15910_e15716;
        locals.var_temp_dn4 = (((locals.var_qis_dn4 * assign15910_e15715) - (locals.var_qis * (locals.var_esurf1s_dn4 + locals.var_esurf2s_dn4))) / (assign15910_e15715 * assign15910_e15715));
        locals.var_temp_dn6 = (((locals.var_qis_dn6 * assign15910_e15715) - (locals.var_qis * (locals.var_esurf1s_dn6 + locals.var_esurf2s_dn6))) / (assign15910_e15715 * assign15910_e15715));
        locals.var_temp_dn7 = (((locals.var_qis_dn7 * assign15910_e15715) - (locals.var_qis * (locals.var_esurf1s_dn7 + locals.var_esurf2s_dn7))) / (assign15910_e15715 * assign15910_e15715));
        locals.var_temp_dn8 = (((locals.var_qis_dn8 * assign15910_e15715) - (locals.var_qis * (locals.var_esurf1s_dn8 + locals.var_esurf2s_dn8))) / (assign15910_e15715 * assign15910_e15715));
        locals.var_temp_dn9 = (((locals.var_qis_dn9 * assign15910_e15715) - (locals.var_qis * (locals.var_esurf1s_dn9 + locals.var_esurf2s_dn9))) / (assign15910_e15715 * assign15910_e15715));

        let assign15920_e15719: f64 = (locals.var_esurf1s * locals.var_temp);
        locals.var_qi1s = assign15920_e15719;
        locals.var_qi1s_dn4 = ((locals.var_esurf1s_dn4 * locals.var_temp) + (locals.var_esurf1s * locals.var_temp_dn4));
        locals.var_qi1s_dn6 = ((locals.var_esurf1s_dn6 * locals.var_temp) + (locals.var_esurf1s * locals.var_temp_dn6));
        locals.var_qi1s_dn7 = ((locals.var_esurf1s_dn7 * locals.var_temp) + (locals.var_esurf1s * locals.var_temp_dn7));
        locals.var_qi1s_dn8 = ((locals.var_esurf1s_dn8 * locals.var_temp) + (locals.var_esurf1s * locals.var_temp_dn8));
        locals.var_qi1s_dn9 = ((locals.var_esurf1s_dn9 * locals.var_temp) + (locals.var_esurf1s * locals.var_temp_dn9));

        let assign15930_e15722: f64 = (locals.var_esurf2s * locals.var_temp);
        locals.var_qi2s = assign15930_e15722;
        locals.var_qi2s_dn4 = ((locals.var_esurf2s_dn4 * locals.var_temp) + (locals.var_esurf2s * locals.var_temp_dn4));
        locals.var_qi2s_dn6 = ((locals.var_esurf2s_dn6 * locals.var_temp) + (locals.var_esurf2s * locals.var_temp_dn6));
        locals.var_qi2s_dn7 = ((locals.var_esurf2s_dn7 * locals.var_temp) + (locals.var_esurf2s * locals.var_temp_dn7));
        locals.var_qi2s_dn8 = ((locals.var_esurf2s_dn8 * locals.var_temp) + (locals.var_esurf2s * locals.var_temp_dn8));
        locals.var_qi2s_dn9 = ((locals.var_esurf2s_dn9 * locals.var_temp) + (locals.var_esurf2s * locals.var_temp_dn9));

        let assign15940_e15725: f64 = (locals.var_esurf1s * locals.var_betn1_t);
        let assign15940_e15728: f64 = (locals.var_stbet_i * locals.var_lnrtn);
        let assign15940_e15729: f64 = (assign15940_e15728).exp();
        let assign15940_e15730: f64 = (assign15940_e15725 * assign15940_e15729);
        locals.var_c1s = assign15940_e15730;
        locals.var_c1s_dn4 = ((((locals.var_esurf1s_dn4 * locals.var_betn1_t) + (locals.var_esurf1s * locals.var_betn1_t_dn4)) * assign15940_e15729) + (assign15940_e15725 * (assign15940_e15729 * (locals.var_stbet_i * locals.var_lnrtn_dn4))));
        locals.var_c1s_dn6 = ((((locals.var_esurf1s_dn6 * locals.var_betn1_t) + (locals.var_esurf1s * locals.var_betn1_t_dn6)) * assign15940_e15729) + (assign15940_e15725 * (assign15940_e15729 * (locals.var_stbet_i * locals.var_lnrtn_dn6))));
        locals.var_c1s_dn7 = ((((locals.var_esurf1s_dn7 * locals.var_betn1_t) + (locals.var_esurf1s * locals.var_betn1_t_dn7)) * assign15940_e15729) + (assign15940_e15725 * (assign15940_e15729 * (locals.var_stbet_i * locals.var_lnrtn_dn7))));
        locals.var_c1s_dn8 = ((((locals.var_esurf1s_dn8 * locals.var_betn1_t) + (locals.var_esurf1s * locals.var_betn1_t_dn8)) * assign15940_e15729) + (assign15940_e15725 * (assign15940_e15729 * (locals.var_stbet_i * locals.var_lnrtn_dn8))));
        locals.var_c1s_dn9 = ((((locals.var_esurf1s_dn9 * locals.var_betn1_t) + (locals.var_esurf1s * locals.var_betn1_t_dn9)) * assign15940_e15729) + (assign15940_e15725 * (assign15940_e15729 * (locals.var_stbet_i * locals.var_lnrtn_dn9))));

        let assign15950_e15733: f64 = (locals.var_esurf2s * locals.var_betn2_t);
        let assign15950_e15736: f64 = (locals.var_stbet_i * locals.var_lnrtn);
        let assign15950_e15737: f64 = (assign15950_e15736).exp();
        let assign15950_e15738: f64 = (assign15950_e15733 * assign15950_e15737);
        locals.var_c2s = assign15950_e15738;
        locals.var_c2s_dn4 = ((((locals.var_esurf2s_dn4 * locals.var_betn2_t) + (locals.var_esurf2s * locals.var_betn2_t_dn4)) * assign15950_e15737) + (assign15950_e15733 * (assign15950_e15737 * (locals.var_stbet_i * locals.var_lnrtn_dn4))));
        locals.var_c2s_dn6 = ((((locals.var_esurf2s_dn6 * locals.var_betn2_t) + (locals.var_esurf2s * locals.var_betn2_t_dn6)) * assign15950_e15737) + (assign15950_e15733 * (assign15950_e15737 * (locals.var_stbet_i * locals.var_lnrtn_dn6))));
        locals.var_c2s_dn7 = ((((locals.var_esurf2s_dn7 * locals.var_betn2_t) + (locals.var_esurf2s * locals.var_betn2_t_dn7)) * assign15950_e15737) + (assign15950_e15733 * (assign15950_e15737 * (locals.var_stbet_i * locals.var_lnrtn_dn7))));
        locals.var_c2s_dn8 = ((((locals.var_esurf2s_dn8 * locals.var_betn2_t) + (locals.var_esurf2s * locals.var_betn2_t_dn8)) * assign15950_e15737) + (assign15950_e15733 * (assign15950_e15737 * (locals.var_stbet_i * locals.var_lnrtn_dn8))));
        locals.var_c2s_dn9 = ((((locals.var_esurf2s_dn9 * locals.var_betn2_t) + (locals.var_esurf2s * locals.var_betn2_t_dn9)) * assign15950_e15737) + (assign15950_e15733 * (assign15950_e15737 * (locals.var_stbet_i * locals.var_lnrtn_dn9))));

        let assign15960_e15743: f64 = (locals.var_xcorb_i * locals.var_ecpl2s);
        let assign15960_e15744: f64 = (locals.var_ecpl1s + assign15960_e15743);
        let assign15960_e15745: f64 = (locals.var_xcor_i * assign15960_e15744);
        locals.var_temp1 = assign15960_e15745;
        locals.var_temp1_dn4 = ((locals.var_xcor_i_dn4 * assign15960_e15744) + (locals.var_xcor_i * (locals.var_ecpl1s_dn4 + (locals.var_xcorb_i * locals.var_ecpl2s_dn4))));
        locals.var_temp1_dn6 = ((locals.var_xcor_i_dn6 * assign15960_e15744) + (locals.var_xcor_i * (locals.var_ecpl1s_dn6 + (locals.var_xcorb_i * locals.var_ecpl2s_dn6))));
        locals.var_temp1_dn7 = ((locals.var_xcor_i_dn7 * assign15960_e15744) + (locals.var_xcor_i * (locals.var_ecpl1s_dn7 + (locals.var_xcorb_i * locals.var_ecpl2s_dn7))));
        locals.var_temp1_dn8 = ((locals.var_xcor_i_dn8 * assign15960_e15744) + (locals.var_xcor_i * (locals.var_ecpl1s_dn8 + (locals.var_xcorb_i * locals.var_ecpl2s_dn8))));
        locals.var_temp1_dn9 = ((locals.var_xcor_i_dn9 * assign15960_e15744) + (locals.var_xcor_i * (locals.var_ecpl1s_dn9 + (locals.var_xcorb_i * locals.var_ecpl2s_dn9))));

        let assign15970_e15749: f64 = (1.0 + locals.var_temp1);
        let assign15970_e15751: f64 = assign15970_e15749;
        let assign15970_e15754: f64 = (1.0 + locals.var_temp1);
        let assign15970_e15756: f64 = assign15970_e15754;
        let assign15970_e15759: f64 = (1.0 + locals.var_temp1);
        let assign15970_e15761: f64 = assign15970_e15759;
        let assign15970_e15762: f64 = (assign15970_e15756 * assign15970_e15761);
        let assign15970_e15764: f64 = (assign15970_e15762 + 0.01);
        let assign15970_e15765: f64 = (assign15970_e15764).sqrt();
        let assign15970_e15766: f64 = (assign15970_e15751 + assign15970_e15765);
        let assign15970_e15767: f64 = (0.5 * assign15970_e15766);
        locals.var_temp2 = assign15970_e15767;
        locals.var_temp2_dn4 = (0.5 * (locals.var_temp1_dn4 + (((locals.var_temp1_dn4 * assign15970_e15761) + (assign15970_e15756 * locals.var_temp1_dn4)) / (2.0 * assign15970_e15765))));
        locals.var_temp2_dn6 = (0.5 * (locals.var_temp1_dn6 + (((locals.var_temp1_dn6 * assign15970_e15761) + (assign15970_e15756 * locals.var_temp1_dn6)) / (2.0 * assign15970_e15765))));
        locals.var_temp2_dn7 = (0.5 * (locals.var_temp1_dn7 + (((locals.var_temp1_dn7 * assign15970_e15761) + (assign15970_e15756 * locals.var_temp1_dn7)) / (2.0 * assign15970_e15765))));
        locals.var_temp2_dn8 = (0.5 * (locals.var_temp1_dn8 + (((locals.var_temp1_dn8 * assign15970_e15761) + (assign15970_e15756 * locals.var_temp1_dn8)) / (2.0 * assign15970_e15765))));
        locals.var_temp2_dn9 = (0.5 * (locals.var_temp1_dn9 + (((locals.var_temp1_dn9 * assign15970_e15761) + (assign15970_e15756 * locals.var_temp1_dn9)) / (2.0 * assign15970_e15765))));

        let assign15980_e15772: f64 = (0.2 * locals.var_temp1);
        let assign15980_e15773: f64 = (1.0 + assign15980_e15772);
        let assign15980_e15775: f64 = assign15980_e15773;
        let assign15980_e15779: f64 = (0.2 * locals.var_temp1);
        let assign15980_e15780: f64 = (1.0 + assign15980_e15779);
        let assign15980_e15782: f64 = assign15980_e15780;
        let assign15980_e15786: f64 = (0.2 * locals.var_temp1);
        let assign15980_e15787: f64 = (1.0 + assign15980_e15786);
        let assign15980_e15789: f64 = assign15980_e15787;
        let assign15980_e15790: f64 = (assign15980_e15782 * assign15980_e15789);
        let assign15980_e15792: f64 = (assign15980_e15790 + 0.01);
        let assign15980_e15793: f64 = (assign15980_e15792).sqrt();
        let assign15980_e15794: f64 = (assign15980_e15775 + assign15980_e15793);
        let assign15980_e15795: f64 = (0.5 * assign15980_e15794);
        locals.var_temp3 = assign15980_e15795;
        locals.var_temp3_dn4 = (0.5 * ((0.2 * locals.var_temp1_dn4) + ((((0.2 * locals.var_temp1_dn4) * assign15980_e15789) + (assign15980_e15782 * (0.2 * locals.var_temp1_dn4))) / (2.0 * assign15980_e15793))));
        locals.var_temp3_dn6 = (0.5 * ((0.2 * locals.var_temp1_dn6) + ((((0.2 * locals.var_temp1_dn6) * assign15980_e15789) + (assign15980_e15782 * (0.2 * locals.var_temp1_dn6))) / (2.0 * assign15980_e15793))));
        locals.var_temp3_dn7 = (0.5 * ((0.2 * locals.var_temp1_dn7) + ((((0.2 * locals.var_temp1_dn7) * assign15980_e15789) + (assign15980_e15782 * (0.2 * locals.var_temp1_dn7))) / (2.0 * assign15980_e15793))));
        locals.var_temp3_dn8 = (0.5 * ((0.2 * locals.var_temp1_dn8) + ((((0.2 * locals.var_temp1_dn8) * assign15980_e15789) + (assign15980_e15782 * (0.2 * locals.var_temp1_dn8))) / (2.0 * assign15980_e15793))));
        locals.var_temp3_dn9 = (0.5 * ((0.2 * locals.var_temp1_dn9) + ((((0.2 * locals.var_temp1_dn9) * assign15980_e15789) + (assign15980_e15782 * (0.2 * locals.var_temp1_dn9))) / (2.0 * assign15980_e15793))));

        let assign15990_e15798: f64 = (locals.var_temp2 / locals.var_temp3);
        locals.var_fcors = assign15990_e15798;
        locals.var_fcors_dn4 = (((locals.var_temp2_dn4 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn4)) / (locals.var_temp3 * locals.var_temp3));
        locals.var_fcors_dn6 = (((locals.var_temp2_dn6 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn6)) / (locals.var_temp3 * locals.var_temp3));
        locals.var_fcors_dn7 = (((locals.var_temp2_dn7 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn7)) / (locals.var_temp3 * locals.var_temp3));
        locals.var_fcors_dn8 = (((locals.var_temp2_dn8 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn8)) / (locals.var_temp3 * locals.var_temp3));
        locals.var_fcors_dn9 = (((locals.var_temp2_dn9 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn9)) / (locals.var_temp3 * locals.var_temp3));

        let assign16000_e15803: f64 = (locals.var_csfi_i * locals.var_ecpl1s);
        let assign16000_e15804: f64 = (1.0 + assign16000_e15803);
        let assign16000_e15807: f64 = (locals.var_csbi_i * locals.var_ecpl2s);
        let assign16000_e15808: f64 = (assign16000_e15804 + assign16000_e15807);
        let assign16000_e15809: f64 = (locals.var_cs_i * assign16000_e15808);
        let assign16000_e15811: f64 = (-locals.var_thecs_i);
        let assign16000_e15815: f64 = (locals.var_qi1s * locals.var_inv_qi1cs);
        let assign16000_e15816: f64 = (1.0 + assign16000_e15815);
        let assign16000_e15819: f64 = (locals.var_qi2s * locals.var_inv_qi2cs);
        let assign16000_e15820: f64 = (assign16000_e15816 + assign16000_e15819);
        let assign16000_e15821: f64 = (assign16000_e15820).ln();
        let assign16000_e15822: f64 = (assign16000_e15811 * assign16000_e15821);
        let assign16000_e15823: f64 = (assign16000_e15822).exp();
        let assign16000_e15824: f64 = (assign16000_e15809 * assign16000_e15823);
        locals.var_gcss = assign16000_e15824;
        locals.var_gcss_dn4 = ((((locals.var_cs_i_dn4 * assign16000_e15808) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1s_dn4) + (locals.var_csbi_i * locals.var_ecpl2s_dn4)))) * assign16000_e15823) + (assign16000_e15809 * (assign16000_e15823 * (((-locals.var_thecs_i_dn4) * assign16000_e15821) + (assign16000_e15811 * (((locals.var_qi1s_dn4 * locals.var_inv_qi1cs) + (locals.var_qi2s_dn4 * locals.var_inv_qi2cs)) / assign16000_e15820))))));
        locals.var_gcss_dn6 = ((((locals.var_cs_i_dn6 * assign16000_e15808) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1s_dn6) + (locals.var_csbi_i * locals.var_ecpl2s_dn6)))) * assign16000_e15823) + (assign16000_e15809 * (assign16000_e15823 * (((-locals.var_thecs_i_dn6) * assign16000_e15821) + (assign16000_e15811 * (((locals.var_qi1s_dn6 * locals.var_inv_qi1cs) + (locals.var_qi2s_dn6 * locals.var_inv_qi2cs)) / assign16000_e15820))))));
        locals.var_gcss_dn7 = ((((locals.var_cs_i_dn7 * assign16000_e15808) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1s_dn7) + (locals.var_csbi_i * locals.var_ecpl2s_dn7)))) * assign16000_e15823) + (assign16000_e15809 * (assign16000_e15823 * (((-locals.var_thecs_i_dn7) * assign16000_e15821) + (assign16000_e15811 * (((locals.var_qi1s_dn7 * locals.var_inv_qi1cs) + (locals.var_qi2s_dn7 * locals.var_inv_qi2cs)) / assign16000_e15820))))));
        locals.var_gcss_dn8 = ((((locals.var_cs_i_dn8 * assign16000_e15808) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1s_dn8) + (locals.var_csbi_i * locals.var_ecpl2s_dn8)))) * assign16000_e15823) + (assign16000_e15809 * (assign16000_e15823 * (((-locals.var_thecs_i_dn8) * assign16000_e15821) + (assign16000_e15811 * (((locals.var_qi1s_dn8 * locals.var_inv_qi1cs) + (locals.var_qi2s_dn8 * locals.var_inv_qi2cs)) / assign16000_e15820))))));
        locals.var_gcss_dn9 = ((((locals.var_cs_i_dn9 * assign16000_e15808) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1s_dn9) + (locals.var_csbi_i * locals.var_ecpl2s_dn9)))) * assign16000_e15823) + (assign16000_e15809 * (assign16000_e15823 * (((-locals.var_thecs_i_dn9) * assign16000_e15821) + (assign16000_e15811 * (((locals.var_qi1s_dn9 * locals.var_inv_qi1cs) + (locals.var_qi2s_dn9 * locals.var_inv_qi2cs)) / assign16000_e15820))))));

        let assign16010_e15827: f64 = if locals.var_rsg_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard604 = assign16010_e15827;

        let (assign16020_e15831, assign16020_e15831_d_n4, assign16020_e15831_d_n6, assign16020_e15831_d_n7, assign16020_e15831_d_n8, assign16020_e15831_d_n9,) = {
    if (locals.var_guard604 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign16020_e15831;
        locals.var_temp3_dn4 = assign16020_e15831_d_n4;
        locals.var_temp3_dn6 = assign16020_e15831_d_n6;
        locals.var_temp3_dn7 = assign16020_e15831_d_n7;
        locals.var_temp3_dn8 = assign16020_e15831_d_n8;
        locals.var_temp3_dn9 = assign16020_e15831_d_n9;

        let assign16030_e15834: f64 = if locals.var_rsg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard605 = assign16030_e15834;

        let (assign16040_e15849, assign16040_e15849_d_n4, assign16040_e15849_d_n6, assign16040_e15849_d_n7, assign16040_e15849_d_n8, assign16040_e15849_d_n9,) = {
    if ((locals.var_guard604 == 0.0) && (locals.var_guard605 != 0.0)) {
        let assign16040_e15843: f64 = (locals.var_qis + 1e-12);
        let assign16040_e15844: f64 = (assign16040_e15843).ln();
        let assign16040_e15845: f64 = (locals.var_thersg_i * assign16040_e15844);
        let assign16040_e15846: f64 = (assign16040_e15845).exp();
        let assign16040_e15847: f64 = (locals.var_rsg_i * assign16040_e15846);
        (assign16040_e15847, (locals.var_rsg_i * (assign16040_e15846 * (locals.var_thersg_i * (locals.var_qis_dn4 / assign16040_e15843)))), (locals.var_rsg_i * (assign16040_e15846 * (locals.var_thersg_i * (locals.var_qis_dn6 / assign16040_e15843)))), (locals.var_rsg_i * (assign16040_e15846 * (locals.var_thersg_i * (locals.var_qis_dn7 / assign16040_e15843)))), (locals.var_rsg_i * (assign16040_e15846 * (locals.var_thersg_i * (locals.var_qis_dn8 / assign16040_e15843)))), (locals.var_rsg_i * (assign16040_e15846 * (locals.var_thersg_i * (locals.var_qis_dn9 / assign16040_e15843)))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign16040_e15849;
        locals.var_temp1_dn4 = assign16040_e15849_d_n4;
        locals.var_temp1_dn6 = assign16040_e15849_d_n6;
        locals.var_temp1_dn7 = assign16040_e15849_d_n7;
        locals.var_temp1_dn8 = assign16040_e15849_d_n8;
        locals.var_temp1_dn9 = assign16040_e15849_d_n9;

        let (assign16050_e15858, assign16050_e15858_d_n4, assign16050_e15858_d_n6, assign16050_e15858_d_n7, assign16050_e15858_d_n8, assign16050_e15858_d_n9,) = {
    if ((locals.var_guard604 == 0.0) && (locals.var_guard605 != 0.0)) {
        let assign16050_e15856: f64 = (1.0 - locals.var_temp1);
        (assign16050_e15856, (-locals.var_temp1_dn4), (-locals.var_temp1_dn6), (-locals.var_temp1_dn7), (-locals.var_temp1_dn8), (-locals.var_temp1_dn9),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign16050_e15858;
        locals.var_temp3_dn4 = assign16050_e15858_d_n4;
        locals.var_temp3_dn6 = assign16050_e15858_d_n6;
        locals.var_temp3_dn7 = assign16050_e15858_d_n7;
        locals.var_temp3_dn8 = assign16050_e15858_d_n8;
        locals.var_temp3_dn9 = assign16050_e15858_d_n9;

        let (assign16060_e15874, assign16060_e15874_d_n4, assign16060_e15874_d_n6, assign16060_e15874_d_n7, assign16060_e15874_d_n8, assign16060_e15874_d_n9,) = {
    if ((locals.var_guard604 == 0.0) && (locals.var_guard605 == 0.0)) {
        let assign16060_e15868: f64 = (locals.var_qis + 1e-12);
        let assign16060_e15869: f64 = (assign16060_e15868).ln();
        let assign16060_e15870: f64 = (locals.var_thersg_i * assign16060_e15869);
        let assign16060_e15871: f64 = (assign16060_e15870).exp();
        let assign16060_e15872: f64 = (locals.var_rsg_i * assign16060_e15871);
        (assign16060_e15872, (locals.var_rsg_i * (assign16060_e15871 * (locals.var_thersg_i * (locals.var_qis_dn4 / assign16060_e15868)))), (locals.var_rsg_i * (assign16060_e15871 * (locals.var_thersg_i * (locals.var_qis_dn6 / assign16060_e15868)))), (locals.var_rsg_i * (assign16060_e15871 * (locals.var_thersg_i * (locals.var_qis_dn7 / assign16060_e15868)))), (locals.var_rsg_i * (assign16060_e15871 * (locals.var_thersg_i * (locals.var_qis_dn8 / assign16060_e15868)))), (locals.var_rsg_i * (assign16060_e15871 * (locals.var_thersg_i * (locals.var_qis_dn9 / assign16060_e15868)))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign16060_e15874;
        locals.var_temp1_dn4 = assign16060_e15874_d_n4;
        locals.var_temp1_dn6 = assign16060_e15874_d_n6;
        locals.var_temp1_dn7 = assign16060_e15874_d_n7;
        locals.var_temp1_dn8 = assign16060_e15874_d_n8;
        locals.var_temp1_dn9 = assign16060_e15874_d_n9;

        let (assign16070_e15886, assign16070_e15886_d_n4, assign16070_e15886_d_n6, assign16070_e15886_d_n7, assign16070_e15886_d_n8, assign16070_e15886_d_n9,) = {
    if ((locals.var_guard604 == 0.0) && (locals.var_guard605 == 0.0)) {
        let assign16070_e15883: f64 = (1.0 + locals.var_temp1);
        let assign16070_e15884: f64 = (1.0 / assign16070_e15883);
        (assign16070_e15884, (-(locals.var_temp1_dn4 / (assign16070_e15883 * assign16070_e15883))), (-(locals.var_temp1_dn6 / (assign16070_e15883 * assign16070_e15883))), (-(locals.var_temp1_dn7 / (assign16070_e15883 * assign16070_e15883))), (-(locals.var_temp1_dn8 / (assign16070_e15883 * assign16070_e15883))), (-(locals.var_temp1_dn9 / (assign16070_e15883 * assign16070_e15883))),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign16070_e15886;
        locals.var_temp3_dn4 = assign16070_e15886_d_n4;
        locals.var_temp3_dn6 = assign16070_e15886_d_n6;
        locals.var_temp3_dn7 = assign16070_e15886_d_n7;
        locals.var_temp3_dn8 = assign16070_e15886_d_n8;
        locals.var_temp3_dn9 = assign16070_e15886_d_n9;

        let assign16080_e15889: f64 = (locals.var_frs * locals.var_csiprime);
        let assign16080_e15891: f64 = (assign16080_e15889 * 0.5);
        let assign16080_e15895: f64 = (locals.var_rsb_i * locals.var_xg20shift);
        let assign16080_e15896: f64 = (1.0 - assign16080_e15895);
        let assign16080_e15898: f64 = assign16080_e15896;
        let assign16080_e15902: f64 = (locals.var_rsb_i * locals.var_xg20shift);
        let assign16080_e15903: f64 = (1.0 - assign16080_e15902);
        let assign16080_e15905: f64 = assign16080_e15903;
        let assign16080_e15909: f64 = (locals.var_rsb_i * locals.var_xg20shift);
        let assign16080_e15910: f64 = (1.0 - assign16080_e15909);
        let assign16080_e15912: f64 = assign16080_e15910;
        let assign16080_e15913: f64 = (assign16080_e15905 * assign16080_e15912);
        let assign16080_e15915: f64 = (assign16080_e15913 + 0.01);
        let assign16080_e15916: f64 = (assign16080_e15915).sqrt();
        let assign16080_e15917: f64 = (assign16080_e15898 + assign16080_e15916);
        let assign16080_e15918: f64 = (assign16080_e15891 * assign16080_e15917);
        locals.var_frscsi = assign16080_e15918;
        locals.var_frscsi_dn4 = (((((locals.var_frs_dn4 * locals.var_csiprime) + (locals.var_frs * locals.var_csiprime_dn4)) * 0.5) * assign16080_e15917) + (assign16080_e15891 * ((-(locals.var_rsb_i * locals.var_xg20shift_dn4)) + ((((-(locals.var_rsb_i * locals.var_xg20shift_dn4)) * assign16080_e15912) + (assign16080_e15905 * (-(locals.var_rsb_i * locals.var_xg20shift_dn4)))) / (2.0 * assign16080_e15916)))));
        locals.var_frscsi_dn6 = (((((locals.var_frs_dn6 * locals.var_csiprime) + (locals.var_frs * locals.var_csiprime_dn6)) * 0.5) * assign16080_e15917) + (assign16080_e15891 * ((-(locals.var_rsb_i * locals.var_xg20shift_dn6)) + ((((-(locals.var_rsb_i * locals.var_xg20shift_dn6)) * assign16080_e15912) + (assign16080_e15905 * (-(locals.var_rsb_i * locals.var_xg20shift_dn6)))) / (2.0 * assign16080_e15916)))));
        locals.var_frscsi_dn7 = (((((locals.var_frs_dn7 * locals.var_csiprime) + (locals.var_frs * locals.var_csiprime_dn7)) * 0.5) * assign16080_e15917) + (assign16080_e15891 * ((-(locals.var_rsb_i * locals.var_xg20shift_dn7)) + ((((-(locals.var_rsb_i * locals.var_xg20shift_dn7)) * assign16080_e15912) + (assign16080_e15905 * (-(locals.var_rsb_i * locals.var_xg20shift_dn7)))) / (2.0 * assign16080_e15916)))));
        locals.var_frscsi_dn8 = (((((locals.var_frs_dn8 * locals.var_csiprime) + (locals.var_frs * locals.var_csiprime_dn8)) * 0.5) * assign16080_e15917) + (assign16080_e15891 * ((-(locals.var_rsb_i * locals.var_xg20shift_dn8)) + ((((-(locals.var_rsb_i * locals.var_xg20shift_dn8)) * assign16080_e15912) + (assign16080_e15905 * (-(locals.var_rsb_i * locals.var_xg20shift_dn8)))) / (2.0 * assign16080_e15916)))));
        locals.var_frscsi_dn9 = (((((locals.var_frs_dn9 * locals.var_csiprime) + (locals.var_frs * locals.var_csiprime_dn9)) * 0.5) * assign16080_e15917) + (assign16080_e15891 * ((-(locals.var_rsb_i * locals.var_xg20shift_dn9)) + ((((-(locals.var_rsb_i * locals.var_xg20shift_dn9)) * assign16080_e15912) + (assign16080_e15905 * (-(locals.var_rsb_i * locals.var_xg20shift_dn9)))) / (2.0 * assign16080_e15916)))));

        let assign16090_e15922: f64 = (locals.var_qis * locals.var_temp3);
        let assign16090_e15924: f64 = (assign16090_e15922 + locals.var_rsig_i);
        let assign16090_e15925: f64 = (locals.var_frscsi * assign16090_e15924);
        locals.var_grss = assign16090_e15925;
        locals.var_grss_dn4 = ((locals.var_frscsi_dn4 * assign16090_e15924) + (locals.var_frscsi * ((locals.var_qis_dn4 * locals.var_temp3) + (locals.var_qis * locals.var_temp3_dn4))));
        locals.var_grss_dn6 = ((locals.var_frscsi_dn6 * assign16090_e15924) + (locals.var_frscsi * ((locals.var_qis_dn6 * locals.var_temp3) + (locals.var_qis * locals.var_temp3_dn6))));
        locals.var_grss_dn7 = ((locals.var_frscsi_dn7 * assign16090_e15924) + (locals.var_frscsi * ((locals.var_qis_dn7 * locals.var_temp3) + (locals.var_qis * locals.var_temp3_dn7))));
        locals.var_grss_dn8 = ((locals.var_frscsi_dn8 * assign16090_e15924) + (locals.var_frscsi * ((locals.var_qis_dn8 * locals.var_temp3) + (locals.var_qis * locals.var_temp3_dn8))));
        locals.var_grss_dn9 = ((locals.var_frscsi_dn9 * assign16090_e15924) + (locals.var_frscsi * ((locals.var_qis_dn9 * locals.var_temp3) + (locals.var_qis * locals.var_temp3_dn9))));

        let assign16100_e15930: f64 = (locals.var_fmue * locals.var_eeff1s);
        let assign16100_e15932: f64 = (assign16100_e15930 + 1e-6);
        let assign16100_e15933: f64 = (assign16100_e15932).ln();
        let assign16100_e15934: f64 = (locals.var_themu_i * assign16100_e15933);
        let assign16100_e15935: f64 = (assign16100_e15934).exp();
        let assign16100_e15936: f64 = (1.0 + assign16100_e15935);
        let assign16100_e15938: f64 = (assign16100_e15936 + locals.var_gcss);
        let assign16100_e15941: f64 = (locals.var_betn1_i * locals.var_grss);
        let assign16100_e15942: f64 = (assign16100_e15938 + assign16100_e15941);
        locals.var_gmob1s = assign16100_e15942;
        locals.var_gmob1s_dn4 = (((assign16100_e15935 * ((locals.var_themu_i_dn4 * assign16100_e15933) + (locals.var_themu_i * (((locals.var_fmue_dn4 * locals.var_eeff1s) + (locals.var_fmue * locals.var_eeff1s_dn4)) / assign16100_e15932)))) + locals.var_gcss_dn4) + ((locals.var_betn1_i_dn4 * locals.var_grss) + (locals.var_betn1_i * locals.var_grss_dn4)));
        locals.var_gmob1s_dn6 = (((assign16100_e15935 * ((locals.var_themu_i_dn6 * assign16100_e15933) + (locals.var_themu_i * (((locals.var_fmue_dn6 * locals.var_eeff1s) + (locals.var_fmue * locals.var_eeff1s_dn6)) / assign16100_e15932)))) + locals.var_gcss_dn6) + ((locals.var_betn1_i_dn6 * locals.var_grss) + (locals.var_betn1_i * locals.var_grss_dn6)));
        locals.var_gmob1s_dn7 = (((assign16100_e15935 * ((locals.var_themu_i_dn7 * assign16100_e15933) + (locals.var_themu_i * (((locals.var_fmue_dn7 * locals.var_eeff1s) + (locals.var_fmue * locals.var_eeff1s_dn7)) / assign16100_e15932)))) + locals.var_gcss_dn7) + ((locals.var_betn1_i_dn7 * locals.var_grss) + (locals.var_betn1_i * locals.var_grss_dn7)));
        locals.var_gmob1s_dn8 = (((assign16100_e15935 * ((locals.var_themu_i_dn8 * assign16100_e15933) + (locals.var_themu_i * (((locals.var_fmue_dn8 * locals.var_eeff1s) + (locals.var_fmue * locals.var_eeff1s_dn8)) / assign16100_e15932)))) + locals.var_gcss_dn8) + ((locals.var_betn1_i_dn8 * locals.var_grss) + (locals.var_betn1_i * locals.var_grss_dn8)));
        locals.var_gmob1s_dn9 = (((assign16100_e15935 * ((locals.var_themu_i_dn9 * assign16100_e15933) + (locals.var_themu_i * (((locals.var_fmue_dn9 * locals.var_eeff1s) + (locals.var_fmue * locals.var_eeff1s_dn9)) / assign16100_e15932)))) + locals.var_gcss_dn9) + ((locals.var_betn1_i_dn9 * locals.var_grss) + (locals.var_betn1_i * locals.var_grss_dn9)));

        let assign16110_e15947: f64 = (locals.var_fmue * locals.var_eeff2s);
        let assign16110_e15949: f64 = (assign16110_e15947 + 1e-6);
        let assign16110_e15950: f64 = (assign16110_e15949).ln();
        let assign16110_e15951: f64 = (locals.var_themu_i * assign16110_e15950);
        let assign16110_e15952: f64 = (assign16110_e15951).exp();
        let assign16110_e15953: f64 = (1.0 + assign16110_e15952);
        let assign16110_e15955: f64 = (assign16110_e15953 + locals.var_gcss);
        let assign16110_e15958: f64 = (locals.var_betn2_i * locals.var_grss);
        let assign16110_e15959: f64 = (assign16110_e15955 + assign16110_e15958);
        locals.var_gmob2s = assign16110_e15959;
        locals.var_gmob2s_dn4 = (((assign16110_e15952 * ((locals.var_themu_i_dn4 * assign16110_e15950) + (locals.var_themu_i * (((locals.var_fmue_dn4 * locals.var_eeff2s) + (locals.var_fmue * locals.var_eeff2s_dn4)) / assign16110_e15949)))) + locals.var_gcss_dn4) + ((locals.var_betn2_i_dn4 * locals.var_grss) + (locals.var_betn2_i * locals.var_grss_dn4)));
        locals.var_gmob2s_dn6 = (((assign16110_e15952 * ((locals.var_themu_i_dn6 * assign16110_e15950) + (locals.var_themu_i * (((locals.var_fmue_dn6 * locals.var_eeff2s) + (locals.var_fmue * locals.var_eeff2s_dn6)) / assign16110_e15949)))) + locals.var_gcss_dn6) + ((locals.var_betn2_i_dn6 * locals.var_grss) + (locals.var_betn2_i * locals.var_grss_dn6)));
        locals.var_gmob2s_dn7 = (((assign16110_e15952 * ((locals.var_themu_i_dn7 * assign16110_e15950) + (locals.var_themu_i * (((locals.var_fmue_dn7 * locals.var_eeff2s) + (locals.var_fmue * locals.var_eeff2s_dn7)) / assign16110_e15949)))) + locals.var_gcss_dn7) + ((locals.var_betn2_i_dn7 * locals.var_grss) + (locals.var_betn2_i * locals.var_grss_dn7)));
        locals.var_gmob2s_dn8 = (((assign16110_e15952 * ((locals.var_themu_i_dn8 * assign16110_e15950) + (locals.var_themu_i * (((locals.var_fmue_dn8 * locals.var_eeff2s) + (locals.var_fmue * locals.var_eeff2s_dn8)) / assign16110_e15949)))) + locals.var_gcss_dn8) + ((locals.var_betn2_i_dn8 * locals.var_grss) + (locals.var_betn2_i * locals.var_grss_dn8)));
        locals.var_gmob2s_dn9 = (((assign16110_e15952 * ((locals.var_themu_i_dn9 * assign16110_e15950) + (locals.var_themu_i * (((locals.var_fmue_dn9 * locals.var_eeff2s) + (locals.var_fmue * locals.var_eeff2s_dn9)) / assign16110_e15949)))) + locals.var_gcss_dn9) + ((locals.var_betn2_i_dn9 * locals.var_grss) + (locals.var_betn2_i * locals.var_grss_dn9)));

        let assign16120_e15963: f64 = (locals.var_c1s + locals.var_c2s);
        let assign16120_e15964: f64 = (locals.var_fcors * assign16120_e15963);
        let assign16120_e15967: f64 = (locals.var_c1s / locals.var_gmob1s);
        let assign16120_e15970: f64 = (locals.var_c2s / locals.var_gmob2s);
        let assign16120_e15971: f64 = (assign16120_e15967 + assign16120_e15970);
        let assign16120_e15972: f64 = (assign16120_e15964 / assign16120_e15971);
        locals.var_gmobs = assign16120_e15972;
        locals.var_gmobs_dn4 = (((((locals.var_fcors_dn4 * assign16120_e15963) + (locals.var_fcors * (locals.var_c1s_dn4 + locals.var_c2s_dn4))) * assign16120_e15971) - (assign16120_e15964 * ((((locals.var_c1s_dn4 * locals.var_gmob1s) - (locals.var_c1s * locals.var_gmob1s_dn4)) / (locals.var_gmob1s * locals.var_gmob1s)) + (((locals.var_c2s_dn4 * locals.var_gmob2s) - (locals.var_c2s * locals.var_gmob2s_dn4)) / (locals.var_gmob2s * locals.var_gmob2s))))) / (assign16120_e15971 * assign16120_e15971));
        locals.var_gmobs_dn6 = (((((locals.var_fcors_dn6 * assign16120_e15963) + (locals.var_fcors * (locals.var_c1s_dn6 + locals.var_c2s_dn6))) * assign16120_e15971) - (assign16120_e15964 * ((((locals.var_c1s_dn6 * locals.var_gmob1s) - (locals.var_c1s * locals.var_gmob1s_dn6)) / (locals.var_gmob1s * locals.var_gmob1s)) + (((locals.var_c2s_dn6 * locals.var_gmob2s) - (locals.var_c2s * locals.var_gmob2s_dn6)) / (locals.var_gmob2s * locals.var_gmob2s))))) / (assign16120_e15971 * assign16120_e15971));
        locals.var_gmobs_dn7 = (((((locals.var_fcors_dn7 * assign16120_e15963) + (locals.var_fcors * (locals.var_c1s_dn7 + locals.var_c2s_dn7))) * assign16120_e15971) - (assign16120_e15964 * ((((locals.var_c1s_dn7 * locals.var_gmob1s) - (locals.var_c1s * locals.var_gmob1s_dn7)) / (locals.var_gmob1s * locals.var_gmob1s)) + (((locals.var_c2s_dn7 * locals.var_gmob2s) - (locals.var_c2s * locals.var_gmob2s_dn7)) / (locals.var_gmob2s * locals.var_gmob2s))))) / (assign16120_e15971 * assign16120_e15971));
        locals.var_gmobs_dn8 = (((((locals.var_fcors_dn8 * assign16120_e15963) + (locals.var_fcors * (locals.var_c1s_dn8 + locals.var_c2s_dn8))) * assign16120_e15971) - (assign16120_e15964 * ((((locals.var_c1s_dn8 * locals.var_gmob1s) - (locals.var_c1s * locals.var_gmob1s_dn8)) / (locals.var_gmob1s * locals.var_gmob1s)) + (((locals.var_c2s_dn8 * locals.var_gmob2s) - (locals.var_c2s * locals.var_gmob2s_dn8)) / (locals.var_gmob2s * locals.var_gmob2s))))) / (assign16120_e15971 * assign16120_e15971));
        locals.var_gmobs_dn9 = (((((locals.var_fcors_dn9 * assign16120_e15963) + (locals.var_fcors * (locals.var_c1s_dn9 + locals.var_c2s_dn9))) * assign16120_e15971) - (assign16120_e15964 * ((((locals.var_c1s_dn9 * locals.var_gmob1s) - (locals.var_c1s * locals.var_gmob1s_dn9)) / (locals.var_gmob1s * locals.var_gmob1s)) + (((locals.var_c2s_dn9 * locals.var_gmob2s) - (locals.var_c2s * locals.var_gmob2s_dn9)) / (locals.var_gmob2s * locals.var_gmob2s))))) / (assign16120_e15971 * assign16120_e15971));

        let assign16130_e15974: f64 = (locals.var_dx_wi).abs();
        let assign16130_e15976: f64 = if assign16130_e15974 > 0.007 { 1.0 } else { 0.0 };
        locals.var_guard606 = assign16130_e15976;

        let assign16140_e15979: f64 = if locals.var_dx_wi > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard607 = assign16140_e15979;

        let (assign16150_e15987, assign16150_e15987_d_n4, assign16150_e15987_d_n6, assign16150_e15987_d_n7, assign16150_e15987_d_n8, assign16150_e15987_d_n9,) = {
    if ((locals.var_guard606 != 0.0) && (locals.var_guard607 != 0.0)) {
        let assign16150_e15984: f64 = (-locals.var_dx_wi);
        let assign16150_e15985: f64 = (assign16150_e15984).exp();
        (assign16150_e15985, (assign16150_e15985 * (-locals.var_dx_wi_dn4)), (assign16150_e15985 * (-locals.var_dx_wi_dn6)), (assign16150_e15985 * (-locals.var_dx_wi_dn7)), (assign16150_e15985 * (-locals.var_dx_wi_dn8)), (assign16150_e15985 * (-locals.var_dx_wi_dn9)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign16150_e15987;
        locals.var_temp_dn4 = assign16150_e15987_d_n4;
        locals.var_temp_dn6 = assign16150_e15987_d_n6;
        locals.var_temp_dn7 = assign16150_e15987_d_n7;
        locals.var_temp_dn8 = assign16150_e15987_d_n8;
        locals.var_temp_dn9 = assign16150_e15987_d_n9;

        let (assign16160_e15997, assign16160_e15997_d_n4, assign16160_e15997_d_n6, assign16160_e15997_d_n7, assign16160_e15997_d_n8, assign16160_e15997_d_n9,) = {
    if ((locals.var_guard606 != 0.0) && (locals.var_guard607 != 0.0)) {
        let assign16160_e15994: f64 = (1.0 - locals.var_temp);
        let assign16160_e15995: f64 = (locals.var_dx_wi / assign16160_e15994);
        (assign16160_e15995, (((locals.var_dx_wi_dn4 * assign16160_e15994) - (locals.var_dx_wi * (-locals.var_temp_dn4))) / (assign16160_e15994 * assign16160_e15994)), (((locals.var_dx_wi_dn6 * assign16160_e15994) - (locals.var_dx_wi * (-locals.var_temp_dn6))) / (assign16160_e15994 * assign16160_e15994)), (((locals.var_dx_wi_dn7 * assign16160_e15994) - (locals.var_dx_wi * (-locals.var_temp_dn7))) / (assign16160_e15994 * assign16160_e15994)), (((locals.var_dx_wi_dn8 * assign16160_e15994) - (locals.var_dx_wi * (-locals.var_temp_dn8))) / (assign16160_e15994 * assign16160_e15994)), (((locals.var_dx_wi_dn9 * assign16160_e15994) - (locals.var_dx_wi * (-locals.var_temp_dn9))) / (assign16160_e15994 * assign16160_e15994)),)
    } else {
        (locals.var_s1, locals.var_s1_dn4, locals.var_s1_dn6, locals.var_s1_dn7, locals.var_s1_dn8, locals.var_s1_dn9,)
    }
};
        locals.var_s1 = assign16160_e15997;
        locals.var_s1_dn4 = assign16160_e15997_d_n4;
        locals.var_s1_dn6 = assign16160_e15997_d_n6;
        locals.var_s1_dn7 = assign16160_e15997_d_n7;
        locals.var_s1_dn8 = assign16160_e15997_d_n8;
        locals.var_s1_dn9 = assign16160_e15997_d_n9;

        let (assign16170_e16005, assign16170_e16005_d_n4, assign16170_e16005_d_n6, assign16170_e16005_d_n7, assign16170_e16005_d_n8, assign16170_e16005_d_n9,) = {
    if ((locals.var_guard606 != 0.0) && (locals.var_guard607 != 0.0)) {
        let assign16170_e16003: f64 = (locals.var_temp * locals.var_s1);
        (assign16170_e16003, ((locals.var_temp_dn4 * locals.var_s1) + (locals.var_temp * locals.var_s1_dn4)), ((locals.var_temp_dn6 * locals.var_s1) + (locals.var_temp * locals.var_s1_dn6)), ((locals.var_temp_dn7 * locals.var_s1) + (locals.var_temp * locals.var_s1_dn7)), ((locals.var_temp_dn8 * locals.var_s1) + (locals.var_temp * locals.var_s1_dn8)), ((locals.var_temp_dn9 * locals.var_s1) + (locals.var_temp * locals.var_s1_dn9)),)
    } else {
        (locals.var_s2, locals.var_s2_dn4, locals.var_s2_dn6, locals.var_s2_dn7, locals.var_s2_dn8, locals.var_s2_dn9,)
    }
};
        locals.var_s2 = assign16170_e16005;
        locals.var_s2_dn4 = assign16170_e16005_d_n4;
        locals.var_s2_dn6 = assign16170_e16005_d_n6;
        locals.var_s2_dn7 = assign16170_e16005_d_n7;
        locals.var_s2_dn8 = assign16170_e16005_d_n8;
        locals.var_s2_dn9 = assign16170_e16005_d_n9;

        let (assign16180_e16020, assign16180_e16020_d_n4, assign16180_e16020_d_n6, assign16180_e16020_d_n7, assign16180_e16020_d_n8, assign16180_e16020_d_n9,) = {
    if ((locals.var_guard606 != 0.0) && (locals.var_guard607 != 0.0)) {
        let assign16180_e16012: f64 = (locals.var_qis * locals.var_s1);
        let assign16180_e16013: f64 = (locals.var_a0 / assign16180_e16012);
        let assign16180_e16014: f64 = (assign16180_e16013).ln();
        let assign16180_e16016: f64 = (assign16180_e16014 - 0.6931471805599);
        let assign16180_e16018: f64 = (assign16180_e16016 + locals.var_x1_wi0);
        (assign16180_e16018, (((((locals.var_a0_dn4 * assign16180_e16012) - (locals.var_a0 * ((locals.var_qis_dn4 * locals.var_s1) + (locals.var_qis * locals.var_s1_dn4)))) / (assign16180_e16012 * assign16180_e16012)) / assign16180_e16013) + locals.var_x1_wi0_dn4), (((((locals.var_a0_dn6 * assign16180_e16012) - (locals.var_a0 * ((locals.var_qis_dn6 * locals.var_s1) + (locals.var_qis * locals.var_s1_dn6)))) / (assign16180_e16012 * assign16180_e16012)) / assign16180_e16013) + locals.var_x1_wi0_dn6), (((((locals.var_a0_dn7 * assign16180_e16012) - (locals.var_a0 * ((locals.var_qis_dn7 * locals.var_s1) + (locals.var_qis * locals.var_s1_dn7)))) / (assign16180_e16012 * assign16180_e16012)) / assign16180_e16013) + locals.var_x1_wi0_dn7), (((((locals.var_a0_dn8 * assign16180_e16012) - (locals.var_a0 * ((locals.var_qis_dn8 * locals.var_s1) + (locals.var_qis * locals.var_s1_dn8)))) / (assign16180_e16012 * assign16180_e16012)) / assign16180_e16013) + locals.var_x1_wi0_dn8), (((((locals.var_a0_dn9 * assign16180_e16012) - (locals.var_a0 * ((locals.var_qis_dn9 * locals.var_s1) + (locals.var_qis * locals.var_s1_dn9)))) / (assign16180_e16012 * assign16180_e16012)) / assign16180_e16013) + locals.var_x1_wi0_dn9),)
    } else {
        (locals.var_deltaxinf, locals.var_deltaxinf_dn4, locals.var_deltaxinf_dn6, locals.var_deltaxinf_dn7, locals.var_deltaxinf_dn8, locals.var_deltaxinf_dn9,)
    }
};
        locals.var_deltaxinf = assign16180_e16020;
        locals.var_deltaxinf_dn4 = assign16180_e16020_d_n4;
        locals.var_deltaxinf_dn6 = assign16180_e16020_d_n6;
        locals.var_deltaxinf_dn7 = assign16180_e16020_d_n7;
        locals.var_deltaxinf_dn8 = assign16180_e16020_d_n8;
        locals.var_deltaxinf_dn9 = assign16180_e16020_d_n9;

        let (assign16190_e16028, assign16190_e16028_d_n4, assign16190_e16028_d_n6, assign16190_e16028_d_n7, assign16190_e16028_d_n8, assign16190_e16028_d_n9,) = {
    if ((locals.var_guard606 != 0.0) && (locals.var_guard607 == 0.0)) {
        let assign16190_e16026: f64 = (locals.var_dx_wi).exp();
        (assign16190_e16026, (assign16190_e16026 * locals.var_dx_wi_dn4), (assign16190_e16026 * locals.var_dx_wi_dn6), (assign16190_e16026 * locals.var_dx_wi_dn7), (assign16190_e16026 * locals.var_dx_wi_dn8), (assign16190_e16026 * locals.var_dx_wi_dn9),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign16190_e16028;
        locals.var_temp_dn4 = assign16190_e16028_d_n4;
        locals.var_temp_dn6 = assign16190_e16028_d_n6;
        locals.var_temp_dn7 = assign16190_e16028_d_n7;
        locals.var_temp_dn8 = assign16190_e16028_d_n8;
        locals.var_temp_dn9 = assign16190_e16028_d_n9;

        let (assign16200_e16039, assign16200_e16039_d_n4, assign16200_e16039_d_n6, assign16200_e16039_d_n7, assign16200_e16039_d_n8, assign16200_e16039_d_n9,) = {
    if ((locals.var_guard606 != 0.0) && (locals.var_guard607 == 0.0)) {
        let assign16200_e16036: f64 = (locals.var_temp - 1.0);
        let assign16200_e16037: f64 = (locals.var_dx_wi / assign16200_e16036);
        (assign16200_e16037, (((locals.var_dx_wi_dn4 * assign16200_e16036) - (locals.var_dx_wi * locals.var_temp_dn4)) / (assign16200_e16036 * assign16200_e16036)), (((locals.var_dx_wi_dn6 * assign16200_e16036) - (locals.var_dx_wi * locals.var_temp_dn6)) / (assign16200_e16036 * assign16200_e16036)), (((locals.var_dx_wi_dn7 * assign16200_e16036) - (locals.var_dx_wi * locals.var_temp_dn7)) / (assign16200_e16036 * assign16200_e16036)), (((locals.var_dx_wi_dn8 * assign16200_e16036) - (locals.var_dx_wi * locals.var_temp_dn8)) / (assign16200_e16036 * assign16200_e16036)), (((locals.var_dx_wi_dn9 * assign16200_e16036) - (locals.var_dx_wi * locals.var_temp_dn9)) / (assign16200_e16036 * assign16200_e16036)),)
    } else {
        (locals.var_s2, locals.var_s2_dn4, locals.var_s2_dn6, locals.var_s2_dn7, locals.var_s2_dn8, locals.var_s2_dn9,)
    }
};
        locals.var_s2 = assign16200_e16039;
        locals.var_s2_dn4 = assign16200_e16039_d_n4;
        locals.var_s2_dn6 = assign16200_e16039_d_n6;
        locals.var_s2_dn7 = assign16200_e16039_d_n7;
        locals.var_s2_dn8 = assign16200_e16039_d_n8;
        locals.var_s2_dn9 = assign16200_e16039_d_n9;

        let (assign16210_e16048, assign16210_e16048_d_n4, assign16210_e16048_d_n6, assign16210_e16048_d_n7, assign16210_e16048_d_n8, assign16210_e16048_d_n9,) = {
    if ((locals.var_guard606 != 0.0) && (locals.var_guard607 == 0.0)) {
        let assign16210_e16046: f64 = (locals.var_temp * locals.var_s2);
        (assign16210_e16046, ((locals.var_temp_dn4 * locals.var_s2) + (locals.var_temp * locals.var_s2_dn4)), ((locals.var_temp_dn6 * locals.var_s2) + (locals.var_temp * locals.var_s2_dn6)), ((locals.var_temp_dn7 * locals.var_s2) + (locals.var_temp * locals.var_s2_dn7)), ((locals.var_temp_dn8 * locals.var_s2) + (locals.var_temp * locals.var_s2_dn8)), ((locals.var_temp_dn9 * locals.var_s2) + (locals.var_temp * locals.var_s2_dn9)),)
    } else {
        (locals.var_s1, locals.var_s1_dn4, locals.var_s1_dn6, locals.var_s1_dn7, locals.var_s1_dn8, locals.var_s1_dn9,)
    }
};
        locals.var_s1 = assign16210_e16048;
        locals.var_s1_dn4 = assign16210_e16048_d_n4;
        locals.var_s1_dn6 = assign16210_e16048_d_n6;
        locals.var_s1_dn7 = assign16210_e16048_d_n7;
        locals.var_s1_dn8 = assign16210_e16048_d_n8;
        locals.var_s1_dn9 = assign16210_e16048_d_n9;

        let (assign16220_e16064, assign16220_e16064_d_n4, assign16220_e16064_d_n6, assign16220_e16064_d_n7, assign16220_e16064_d_n8, assign16220_e16064_d_n9,) = {
    if ((locals.var_guard606 != 0.0) && (locals.var_guard607 == 0.0)) {
        let assign16220_e16056: f64 = (locals.var_qis * locals.var_s2);
        let assign16220_e16057: f64 = (locals.var_a0 / assign16220_e16056);
        let assign16220_e16058: f64 = (assign16220_e16057).ln();
        let assign16220_e16060: f64 = (assign16220_e16058 - 0.6931471805599);
        let assign16220_e16062: f64 = (assign16220_e16060 + locals.var_x2_wi0);
        (assign16220_e16062, (((((locals.var_a0_dn4 * assign16220_e16056) - (locals.var_a0 * ((locals.var_qis_dn4 * locals.var_s2) + (locals.var_qis * locals.var_s2_dn4)))) / (assign16220_e16056 * assign16220_e16056)) / assign16220_e16057) + locals.var_x2_wi0_dn4), (((((locals.var_a0_dn6 * assign16220_e16056) - (locals.var_a0 * ((locals.var_qis_dn6 * locals.var_s2) + (locals.var_qis * locals.var_s2_dn6)))) / (assign16220_e16056 * assign16220_e16056)) / assign16220_e16057) + locals.var_x2_wi0_dn6), (((((locals.var_a0_dn7 * assign16220_e16056) - (locals.var_a0 * ((locals.var_qis_dn7 * locals.var_s2) + (locals.var_qis * locals.var_s2_dn7)))) / (assign16220_e16056 * assign16220_e16056)) / assign16220_e16057) + locals.var_x2_wi0_dn7), (((((locals.var_a0_dn8 * assign16220_e16056) - (locals.var_a0 * ((locals.var_qis_dn8 * locals.var_s2) + (locals.var_qis * locals.var_s2_dn8)))) / (assign16220_e16056 * assign16220_e16056)) / assign16220_e16057) + locals.var_x2_wi0_dn8), (((((locals.var_a0_dn9 * assign16220_e16056) - (locals.var_a0 * ((locals.var_qis_dn9 * locals.var_s2) + (locals.var_qis * locals.var_s2_dn9)))) / (assign16220_e16056 * assign16220_e16056)) / assign16220_e16057) + locals.var_x2_wi0_dn9),)
    } else {
        (locals.var_deltaxinf, locals.var_deltaxinf_dn4, locals.var_deltaxinf_dn6, locals.var_deltaxinf_dn7, locals.var_deltaxinf_dn8, locals.var_deltaxinf_dn9,)
    }
};
        locals.var_deltaxinf = assign16220_e16064;
        locals.var_deltaxinf_dn4 = assign16220_e16064_d_n4;
        locals.var_deltaxinf_dn6 = assign16220_e16064_d_n6;
        locals.var_deltaxinf_dn7 = assign16220_e16064_d_n7;
        locals.var_deltaxinf_dn8 = assign16220_e16064_d_n8;
        locals.var_deltaxinf_dn9 = assign16220_e16064_d_n9;

    }

    pub(super) fn stamp_transient_block_40(
        locals: &mut StampLocals,
    ) {
        let (assign16230_e16079, assign16230_e16079_d_n4, assign16230_e16079_d_n6, assign16230_e16079_d_n7, assign16230_e16079_d_n8, assign16230_e16079_d_n9,) = {
    if (locals.var_guard606 != 0.0) {
        let assign16230_e16067: f64 = (-locals.var_dx_wi);
        let assign16230_e16071: f64 = (1.0 - locals.var_s1);
        let assign16230_e16074: f64 = (locals.var_dx_wi * locals.var_inv_k2);
        let assign16230_e16075: f64 = (assign16230_e16071 - assign16230_e16074);
        let assign16230_e16076: f64 = (locals.var_keq * assign16230_e16075);
        let assign16230_e16077: f64 = (assign16230_e16067 / assign16230_e16076);
        (assign16230_e16077, ((((-locals.var_dx_wi_dn4) * assign16230_e16076) - (assign16230_e16067 * ((locals.var_keq_dn4 * assign16230_e16075) + (locals.var_keq * ((-locals.var_s1_dn4) - ((locals.var_dx_wi_dn4 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn4))))))) / (assign16230_e16076 * assign16230_e16076)), ((((-locals.var_dx_wi_dn6) * assign16230_e16076) - (assign16230_e16067 * ((locals.var_keq_dn6 * assign16230_e16075) + (locals.var_keq * ((-locals.var_s1_dn6) - ((locals.var_dx_wi_dn6 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn6))))))) / (assign16230_e16076 * assign16230_e16076)), ((((-locals.var_dx_wi_dn7) * assign16230_e16076) - (assign16230_e16067 * ((locals.var_keq_dn7 * assign16230_e16075) + (locals.var_keq * ((-locals.var_s1_dn7) - ((locals.var_dx_wi_dn7 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn7))))))) / (assign16230_e16076 * assign16230_e16076)), ((((-locals.var_dx_wi_dn8) * assign16230_e16076) - (assign16230_e16067 * ((locals.var_keq_dn8 * assign16230_e16075) + (locals.var_keq * ((-locals.var_s1_dn8) - ((locals.var_dx_wi_dn8 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn8))))))) / (assign16230_e16076 * assign16230_e16076)), ((((-locals.var_dx_wi_dn9) * assign16230_e16076) - (assign16230_e16067 * ((locals.var_keq_dn9 * assign16230_e16075) + (locals.var_keq * ((-locals.var_s1_dn9) - ((locals.var_dx_wi_dn9 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn9))))))) / (assign16230_e16076 * assign16230_e16076)),)
    } else {
        (locals.var_q1chapinf, locals.var_q1chapinf_dn4, locals.var_q1chapinf_dn6, locals.var_q1chapinf_dn7, locals.var_q1chapinf_dn8, locals.var_q1chapinf_dn9,)
    }
};
        locals.var_q1chapinf = assign16230_e16079;
        locals.var_q1chapinf_dn4 = assign16230_e16079_d_n4;
        locals.var_q1chapinf_dn6 = assign16230_e16079_d_n6;
        locals.var_q1chapinf_dn7 = assign16230_e16079_d_n7;
        locals.var_q1chapinf_dn8 = assign16230_e16079_d_n8;
        locals.var_q1chapinf_dn9 = assign16230_e16079_d_n9;

        let (assign16240_e16093, assign16240_e16093_d_n4, assign16240_e16093_d_n6, assign16240_e16093_d_n7, assign16240_e16093_d_n8, assign16240_e16093_d_n9,) = {
    if (locals.var_guard606 != 0.0) {
        let assign16240_e16085: f64 = (1.0 - locals.var_s2);
        let assign16240_e16088: f64 = (locals.var_dx_wi * locals.var_inv_k1);
        let assign16240_e16089: f64 = (assign16240_e16085 + assign16240_e16088);
        let assign16240_e16090: f64 = (locals.var_keq * assign16240_e16089);
        let assign16240_e16091: f64 = (locals.var_dx_wi / assign16240_e16090);
        (assign16240_e16091, (((locals.var_dx_wi_dn4 * assign16240_e16090) - (locals.var_dx_wi * ((locals.var_keq_dn4 * assign16240_e16089) + (locals.var_keq * ((-locals.var_s2_dn4) + ((locals.var_dx_wi_dn4 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn4))))))) / (assign16240_e16090 * assign16240_e16090)), (((locals.var_dx_wi_dn6 * assign16240_e16090) - (locals.var_dx_wi * ((locals.var_keq_dn6 * assign16240_e16089) + (locals.var_keq * ((-locals.var_s2_dn6) + ((locals.var_dx_wi_dn6 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn6))))))) / (assign16240_e16090 * assign16240_e16090)), (((locals.var_dx_wi_dn7 * assign16240_e16090) - (locals.var_dx_wi * ((locals.var_keq_dn7 * assign16240_e16089) + (locals.var_keq * ((-locals.var_s2_dn7) + ((locals.var_dx_wi_dn7 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn7))))))) / (assign16240_e16090 * assign16240_e16090)), (((locals.var_dx_wi_dn8 * assign16240_e16090) - (locals.var_dx_wi * ((locals.var_keq_dn8 * assign16240_e16089) + (locals.var_keq * ((-locals.var_s2_dn8) + ((locals.var_dx_wi_dn8 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn8))))))) / (assign16240_e16090 * assign16240_e16090)), (((locals.var_dx_wi_dn9 * assign16240_e16090) - (locals.var_dx_wi * ((locals.var_keq_dn9 * assign16240_e16089) + (locals.var_keq * ((-locals.var_s2_dn9) + ((locals.var_dx_wi_dn9 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn9))))))) / (assign16240_e16090 * assign16240_e16090)),)
    } else {
        (locals.var_q2chapinf, locals.var_q2chapinf_dn4, locals.var_q2chapinf_dn6, locals.var_q2chapinf_dn7, locals.var_q2chapinf_dn8, locals.var_q2chapinf_dn9,)
    }
};
        locals.var_q2chapinf = assign16240_e16093;
        locals.var_q2chapinf_dn4 = assign16240_e16093_d_n4;
        locals.var_q2chapinf_dn6 = assign16240_e16093_d_n6;
        locals.var_q2chapinf_dn7 = assign16240_e16093_d_n7;
        locals.var_q2chapinf_dn8 = assign16240_e16093_d_n8;
        locals.var_q2chapinf_dn9 = assign16240_e16093_d_n9;

        let (assign16250_e16113, assign16250_e16113_d_n4, assign16250_e16113_d_n6, assign16250_e16113_d_n7, assign16250_e16113_d_n8, assign16250_e16113_d_n9,) = {
    if (locals.var_guard606 != 0.0) {
        let assign16250_e16098: f64 = (locals.var_s2 * locals.var_inv_k2);
        let assign16250_e16100: f64 = (assign16250_e16098 + 0.5);
        let assign16250_e16102: f64 = (assign16250_e16100 / locals.var_q2chapinf);
        let assign16250_e16105: f64 = (locals.var_s1 * locals.var_inv_k1);
        let assign16250_e16107: f64 = (assign16250_e16105 + 0.5);
        let assign16250_e16109: f64 = (assign16250_e16107 / locals.var_q1chapinf);
        let assign16250_e16110: f64 = (assign16250_e16102 - assign16250_e16109);
        let assign16250_e16111: f64 = (locals.var_dx_wi / assign16250_e16110);
        (assign16250_e16111, (((locals.var_dx_wi_dn4 * assign16250_e16110) - (locals.var_dx_wi * ((((((locals.var_s2_dn4 * locals.var_inv_k2) + (locals.var_s2 * locals.var_inv_k2_dn4)) * locals.var_q2chapinf) - (assign16250_e16100 * locals.var_q2chapinf_dn4)) / (locals.var_q2chapinf * locals.var_q2chapinf)) - (((((locals.var_s1_dn4 * locals.var_inv_k1) + (locals.var_s1 * locals.var_inv_k1_dn4)) * locals.var_q1chapinf) - (assign16250_e16107 * locals.var_q1chapinf_dn4)) / (locals.var_q1chapinf * locals.var_q1chapinf))))) / (assign16250_e16110 * assign16250_e16110)), (((locals.var_dx_wi_dn6 * assign16250_e16110) - (locals.var_dx_wi * ((((((locals.var_s2_dn6 * locals.var_inv_k2) + (locals.var_s2 * locals.var_inv_k2_dn6)) * locals.var_q2chapinf) - (assign16250_e16100 * locals.var_q2chapinf_dn6)) / (locals.var_q2chapinf * locals.var_q2chapinf)) - (((((locals.var_s1_dn6 * locals.var_inv_k1) + (locals.var_s1 * locals.var_inv_k1_dn6)) * locals.var_q1chapinf) - (assign16250_e16107 * locals.var_q1chapinf_dn6)) / (locals.var_q1chapinf * locals.var_q1chapinf))))) / (assign16250_e16110 * assign16250_e16110)), (((locals.var_dx_wi_dn7 * assign16250_e16110) - (locals.var_dx_wi * ((((((locals.var_s2_dn7 * locals.var_inv_k2) + (locals.var_s2 * locals.var_inv_k2_dn7)) * locals.var_q2chapinf) - (assign16250_e16100 * locals.var_q2chapinf_dn7)) / (locals.var_q2chapinf * locals.var_q2chapinf)) - (((((locals.var_s1_dn7 * locals.var_inv_k1) + (locals.var_s1 * locals.var_inv_k1_dn7)) * locals.var_q1chapinf) - (assign16250_e16107 * locals.var_q1chapinf_dn7)) / (locals.var_q1chapinf * locals.var_q1chapinf))))) / (assign16250_e16110 * assign16250_e16110)), (((locals.var_dx_wi_dn8 * assign16250_e16110) - (locals.var_dx_wi * ((((((locals.var_s2_dn8 * locals.var_inv_k2) + (locals.var_s2 * locals.var_inv_k2_dn8)) * locals.var_q2chapinf) - (assign16250_e16100 * locals.var_q2chapinf_dn8)) / (locals.var_q2chapinf * locals.var_q2chapinf)) - (((((locals.var_s1_dn8 * locals.var_inv_k1) + (locals.var_s1 * locals.var_inv_k1_dn8)) * locals.var_q1chapinf) - (assign16250_e16107 * locals.var_q1chapinf_dn8)) / (locals.var_q1chapinf * locals.var_q1chapinf))))) / (assign16250_e16110 * assign16250_e16110)), (((locals.var_dx_wi_dn9 * assign16250_e16110) - (locals.var_dx_wi * ((((((locals.var_s2_dn9 * locals.var_inv_k2) + (locals.var_s2 * locals.var_inv_k2_dn9)) * locals.var_q2chapinf) - (assign16250_e16100 * locals.var_q2chapinf_dn9)) / (locals.var_q2chapinf * locals.var_q2chapinf)) - (((((locals.var_s1_dn9 * locals.var_inv_k1) + (locals.var_s1 * locals.var_inv_k1_dn9)) * locals.var_q1chapinf) - (assign16250_e16107 * locals.var_q1chapinf_dn9)) / (locals.var_q1chapinf * locals.var_q1chapinf))))) / (assign16250_e16110 * assign16250_e16110)),)
    } else {
        (locals.var_dinf, locals.var_dinf_dn4, locals.var_dinf_dn6, locals.var_dinf_dn7, locals.var_dinf_dn8, locals.var_dinf_dn9,)
    }
};
        locals.var_dinf = assign16250_e16113;
        locals.var_dinf_dn4 = assign16250_e16113_d_n4;
        locals.var_dinf_dn6 = assign16250_e16113_d_n6;
        locals.var_dinf_dn7 = assign16250_e16113_d_n7;
        locals.var_dinf_dn8 = assign16250_e16113_d_n8;
        locals.var_dinf_dn9 = assign16250_e16113_d_n9;

        let (assign16260_e16122, assign16260_e16122_d_n4, assign16260_e16122_d_n6, assign16260_e16122_d_n7, assign16260_e16122_d_n8, assign16260_e16122_d_n9,) = {
    if (locals.var_guard606 == 0.0) {
        let assign16260_e16118: f64 = (0.5 * 0.1666666666667);
        let assign16260_e16120: f64 = (assign16260_e16118 * locals.var_dx_wisq);
        (assign16260_e16120, (assign16260_e16118 * locals.var_dx_wisq_dn4), (assign16260_e16118 * locals.var_dx_wisq_dn6), (assign16260_e16118 * locals.var_dx_wisq_dn7), (assign16260_e16118 * locals.var_dx_wisq_dn8), (assign16260_e16118 * locals.var_dx_wisq_dn9),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign16260_e16122;
        locals.var_temp_dn4 = assign16260_e16122_d_n4;
        locals.var_temp_dn6 = assign16260_e16122_d_n6;
        locals.var_temp_dn7 = assign16260_e16122_d_n7;
        locals.var_temp_dn8 = assign16260_e16122_d_n8;
        locals.var_temp_dn9 = assign16260_e16122_d_n9;

        let (assign16270_e16129, assign16270_e16129_d_n4, assign16270_e16129_d_n6, assign16270_e16129_d_n7, assign16270_e16129_d_n8, assign16270_e16129_d_n9,) = {
    if (locals.var_guard606 == 0.0) {
        let assign16270_e16127: f64 = (0.5 * locals.var_dx_wi);
        (assign16270_e16127, (0.5 * locals.var_dx_wi_dn4), (0.5 * locals.var_dx_wi_dn6), (0.5 * locals.var_dx_wi_dn7), (0.5 * locals.var_dx_wi_dn8), (0.5 * locals.var_dx_wi_dn9),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign16270_e16129;
        locals.var_temp1_dn4 = assign16270_e16129_d_n4;
        locals.var_temp1_dn6 = assign16270_e16129_d_n6;
        locals.var_temp1_dn7 = assign16270_e16129_d_n7;
        locals.var_temp1_dn8 = assign16270_e16129_d_n8;
        locals.var_temp1_dn9 = assign16270_e16129_d_n9;

        let (assign16280_e16138, assign16280_e16138_d_n4, assign16280_e16138_d_n6, assign16280_e16138_d_n7, assign16280_e16138_d_n8, assign16280_e16138_d_n9,) = {
    if (locals.var_guard606 == 0.0) {
        let assign16280_e16134: f64 = (1.0 + locals.var_temp1);
        let assign16280_e16136: f64 = (assign16280_e16134 + locals.var_temp);
        (assign16280_e16136, (locals.var_temp1_dn4 + locals.var_temp_dn4), (locals.var_temp1_dn6 + locals.var_temp_dn6), (locals.var_temp1_dn7 + locals.var_temp_dn7), (locals.var_temp1_dn8 + locals.var_temp_dn8), (locals.var_temp1_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_s1, locals.var_s1_dn4, locals.var_s1_dn6, locals.var_s1_dn7, locals.var_s1_dn8, locals.var_s1_dn9,)
    }
};
        locals.var_s1 = assign16280_e16138;
        locals.var_s1_dn4 = assign16280_e16138_d_n4;
        locals.var_s1_dn6 = assign16280_e16138_d_n6;
        locals.var_s1_dn7 = assign16280_e16138_d_n7;
        locals.var_s1_dn8 = assign16280_e16138_d_n8;
        locals.var_s1_dn9 = assign16280_e16138_d_n9;

        let (assign16290_e16147, assign16290_e16147_d_n4, assign16290_e16147_d_n6, assign16290_e16147_d_n7, assign16290_e16147_d_n8, assign16290_e16147_d_n9,) = {
    if (locals.var_guard606 == 0.0) {
        let assign16290_e16143: f64 = (1.0 - locals.var_temp1);
        let assign16290_e16145: f64 = (assign16290_e16143 + locals.var_temp);
        (assign16290_e16145, ((-locals.var_temp1_dn4) + locals.var_temp_dn4), ((-locals.var_temp1_dn6) + locals.var_temp_dn6), ((-locals.var_temp1_dn7) + locals.var_temp_dn7), ((-locals.var_temp1_dn8) + locals.var_temp_dn8), ((-locals.var_temp1_dn9) + locals.var_temp_dn9),)
    } else {
        (locals.var_s2, locals.var_s2_dn4, locals.var_s2_dn6, locals.var_s2_dn7, locals.var_s2_dn8, locals.var_s2_dn9,)
    }
};
        locals.var_s2 = assign16290_e16147;
        locals.var_s2_dn4 = assign16290_e16147_d_n4;
        locals.var_s2_dn6 = assign16290_e16147_d_n6;
        locals.var_s2_dn7 = assign16290_e16147_d_n7;
        locals.var_s2_dn8 = assign16290_e16147_d_n8;
        locals.var_s2_dn9 = assign16290_e16147_d_n9;

        let (assign16300_e16154, assign16300_e16154_d_n4, assign16300_e16154_d_n6, assign16300_e16154_d_n7, assign16300_e16154_d_n8, assign16300_e16154_d_n9,) = {
    if (locals.var_guard606 == 0.0) {
        let assign16300_e16152: f64 = (0.1666666666667 * locals.var_temp1);
        (assign16300_e16152, (0.1666666666667 * locals.var_temp1_dn4), (0.1666666666667 * locals.var_temp1_dn6), (0.1666666666667 * locals.var_temp1_dn7), (0.1666666666667 * locals.var_temp1_dn8), (0.1666666666667 * locals.var_temp1_dn9),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign16300_e16154;
        locals.var_temp2_dn4 = assign16300_e16154_d_n4;
        locals.var_temp2_dn6 = assign16300_e16154_d_n6;
        locals.var_temp2_dn7 = assign16300_e16154_d_n7;
        locals.var_temp2_dn8 = assign16300_e16154_d_n8;
        locals.var_temp2_dn9 = assign16300_e16154_d_n9;

        let (assign16310_e16167, assign16310_e16167_d_n4, assign16310_e16167_d_n6, assign16310_e16167_d_n7, assign16310_e16167_d_n8, assign16310_e16167_d_n9,) = {
    if (locals.var_guard606 == 0.0) {
        let assign16310_e16161: f64 = (0.5 + locals.var_inv_k2);
        let assign16310_e16163: f64 = (assign16310_e16161 + locals.var_temp2);
        let assign16310_e16164: f64 = (locals.var_keq * assign16310_e16163);
        let assign16310_e16165: f64 = (1.0 / assign16310_e16164);
        (assign16310_e16165, (-(((locals.var_keq_dn4 * assign16310_e16163) + (locals.var_keq * (locals.var_inv_k2_dn4 + locals.var_temp2_dn4))) / (assign16310_e16164 * assign16310_e16164))), (-(((locals.var_keq_dn6 * assign16310_e16163) + (locals.var_keq * (locals.var_inv_k2_dn6 + locals.var_temp2_dn6))) / (assign16310_e16164 * assign16310_e16164))), (-(((locals.var_keq_dn7 * assign16310_e16163) + (locals.var_keq * (locals.var_inv_k2_dn7 + locals.var_temp2_dn7))) / (assign16310_e16164 * assign16310_e16164))), (-(((locals.var_keq_dn8 * assign16310_e16163) + (locals.var_keq * (locals.var_inv_k2_dn8 + locals.var_temp2_dn8))) / (assign16310_e16164 * assign16310_e16164))), (-(((locals.var_keq_dn9 * assign16310_e16163) + (locals.var_keq * (locals.var_inv_k2_dn9 + locals.var_temp2_dn9))) / (assign16310_e16164 * assign16310_e16164))),)
    } else {
        (locals.var_q1chapinf, locals.var_q1chapinf_dn4, locals.var_q1chapinf_dn6, locals.var_q1chapinf_dn7, locals.var_q1chapinf_dn8, locals.var_q1chapinf_dn9,)
    }
};
        locals.var_q1chapinf = assign16310_e16167;
        locals.var_q1chapinf_dn4 = assign16310_e16167_d_n4;
        locals.var_q1chapinf_dn6 = assign16310_e16167_d_n6;
        locals.var_q1chapinf_dn7 = assign16310_e16167_d_n7;
        locals.var_q1chapinf_dn8 = assign16310_e16167_d_n8;
        locals.var_q1chapinf_dn9 = assign16310_e16167_d_n9;

        let (assign16320_e16180, assign16320_e16180_d_n4, assign16320_e16180_d_n6, assign16320_e16180_d_n7, assign16320_e16180_d_n8, assign16320_e16180_d_n9,) = {
    if (locals.var_guard606 == 0.0) {
        let assign16320_e16174: f64 = (0.5 + locals.var_inv_k1);
        let assign16320_e16176: f64 = (assign16320_e16174 - locals.var_temp2);
        let assign16320_e16177: f64 = (locals.var_keq * assign16320_e16176);
        let assign16320_e16178: f64 = (1.0 / assign16320_e16177);
        (assign16320_e16178, (-(((locals.var_keq_dn4 * assign16320_e16176) + (locals.var_keq * (locals.var_inv_k1_dn4 - locals.var_temp2_dn4))) / (assign16320_e16177 * assign16320_e16177))), (-(((locals.var_keq_dn6 * assign16320_e16176) + (locals.var_keq * (locals.var_inv_k1_dn6 - locals.var_temp2_dn6))) / (assign16320_e16177 * assign16320_e16177))), (-(((locals.var_keq_dn7 * assign16320_e16176) + (locals.var_keq * (locals.var_inv_k1_dn7 - locals.var_temp2_dn7))) / (assign16320_e16177 * assign16320_e16177))), (-(((locals.var_keq_dn8 * assign16320_e16176) + (locals.var_keq * (locals.var_inv_k1_dn8 - locals.var_temp2_dn8))) / (assign16320_e16177 * assign16320_e16177))), (-(((locals.var_keq_dn9 * assign16320_e16176) + (locals.var_keq * (locals.var_inv_k1_dn9 - locals.var_temp2_dn9))) / (assign16320_e16177 * assign16320_e16177))),)
    } else {
        (locals.var_q2chapinf, locals.var_q2chapinf_dn4, locals.var_q2chapinf_dn6, locals.var_q2chapinf_dn7, locals.var_q2chapinf_dn8, locals.var_q2chapinf_dn9,)
    }
};
        locals.var_q2chapinf = assign16320_e16180;
        locals.var_q2chapinf_dn4 = assign16320_e16180_d_n4;
        locals.var_q2chapinf_dn6 = assign16320_e16180_d_n6;
        locals.var_q2chapinf_dn7 = assign16320_e16180_d_n7;
        locals.var_q2chapinf_dn8 = assign16320_e16180_d_n8;
        locals.var_q2chapinf_dn9 = assign16320_e16180_d_n9;

        let (assign16330_e16202, assign16330_e16202_d_n4, assign16330_e16202_d_n6, assign16330_e16202_d_n7, assign16330_e16202_d_n8, assign16330_e16202_d_n9,) = {
    if (locals.var_guard606 == 0.0) {
        let assign16330_e16188: f64 = (0.5 * locals.var_temp);
        let assign16330_e16189: f64 = (1.0 - assign16330_e16188);
        let assign16330_e16190: f64 = (locals.var_qis * assign16330_e16189);
        let assign16330_e16191: f64 = (locals.var_a0 / assign16330_e16190);
        let assign16330_e16192: f64 = (assign16330_e16191).ln();
        let assign16330_e16194: f64 = (assign16330_e16192 - 0.6931471805599);
        let assign16330_e16198: f64 = (locals.var_x1_wi0 + locals.var_x2_wi0);
        let assign16330_e16199: f64 = (0.5 * assign16330_e16198);
        let assign16330_e16200: f64 = (assign16330_e16194 + assign16330_e16199);
        (assign16330_e16200, (((((locals.var_a0_dn4 * assign16330_e16190) - (locals.var_a0 * ((locals.var_qis_dn4 * assign16330_e16189) + (locals.var_qis * (-(0.5 * locals.var_temp_dn4)))))) / (assign16330_e16190 * assign16330_e16190)) / assign16330_e16191) + (0.5 * (locals.var_x1_wi0_dn4 + locals.var_x2_wi0_dn4))), (((((locals.var_a0_dn6 * assign16330_e16190) - (locals.var_a0 * ((locals.var_qis_dn6 * assign16330_e16189) + (locals.var_qis * (-(0.5 * locals.var_temp_dn6)))))) / (assign16330_e16190 * assign16330_e16190)) / assign16330_e16191) + (0.5 * (locals.var_x1_wi0_dn6 + locals.var_x2_wi0_dn6))), (((((locals.var_a0_dn7 * assign16330_e16190) - (locals.var_a0 * ((locals.var_qis_dn7 * assign16330_e16189) + (locals.var_qis * (-(0.5 * locals.var_temp_dn7)))))) / (assign16330_e16190 * assign16330_e16190)) / assign16330_e16191) + (0.5 * (locals.var_x1_wi0_dn7 + locals.var_x2_wi0_dn7))), (((((locals.var_a0_dn8 * assign16330_e16190) - (locals.var_a0 * ((locals.var_qis_dn8 * assign16330_e16189) + (locals.var_qis * (-(0.5 * locals.var_temp_dn8)))))) / (assign16330_e16190 * assign16330_e16190)) / assign16330_e16191) + (0.5 * (locals.var_x1_wi0_dn8 + locals.var_x2_wi0_dn8))), (((((locals.var_a0_dn9 * assign16330_e16190) - (locals.var_a0 * ((locals.var_qis_dn9 * assign16330_e16189) + (locals.var_qis * (-(0.5 * locals.var_temp_dn9)))))) / (assign16330_e16190 * assign16330_e16190)) / assign16330_e16191) + (0.5 * (locals.var_x1_wi0_dn9 + locals.var_x2_wi0_dn9))),)
    } else {
        (locals.var_deltaxinf, locals.var_deltaxinf_dn4, locals.var_deltaxinf_dn6, locals.var_deltaxinf_dn7, locals.var_deltaxinf_dn8, locals.var_deltaxinf_dn9,)
    }
};
        locals.var_deltaxinf = assign16330_e16202;
        locals.var_deltaxinf_dn4 = assign16330_e16202_d_n4;
        locals.var_deltaxinf_dn6 = assign16330_e16202_d_n6;
        locals.var_deltaxinf_dn7 = assign16330_e16202_d_n7;
        locals.var_deltaxinf_dn8 = assign16330_e16202_d_n8;
        locals.var_deltaxinf_dn9 = assign16330_e16202_d_n9;

        let (assign16340_e16240, assign16340_e16240_d_n4, assign16340_e16240_d_n6, assign16340_e16240_d_n7, assign16340_e16240_d_n8, assign16340_e16240_d_n9,) = {
    if (locals.var_guard606 == 0.0) {
        let assign16340_e16206: f64 = (-12.0);
        let assign16340_e16210: f64 = (3.0 * locals.var_keq);
        let assign16340_e16211: f64 = (4.0 - assign16340_e16210);
        let assign16340_e16214: f64 = (12.0 * locals.var_keq);
        let assign16340_e16217: f64 = (locals.var_k1 * locals.var_k2);
        let assign16340_e16218: f64 = (assign16340_e16214 / assign16340_e16217);
        let assign16340_e16219: f64 = (assign16340_e16211 + assign16340_e16218);
        let assign16340_e16223: f64 = (locals.var_inv_k1 - locals.var_inv_k2);
        let assign16340_e16224: f64 = (locals.var_keq * assign16340_e16223);
        let assign16340_e16226: f64 = (assign16340_e16224 * locals.var_dx_wi);
        let assign16340_e16227: f64 = (assign16340_e16219 + assign16340_e16226);
        let assign16340_e16232: f64 = (0.25 * locals.var_keq);
        let assign16340_e16233: f64 = (0.2 - assign16340_e16232);
        let assign16340_e16234: f64 = (0.3333333333333 * assign16340_e16233);
        let assign16340_e16236: f64 = (assign16340_e16234 * locals.var_dx_wisq);
        let assign16340_e16237: f64 = (assign16340_e16227 + assign16340_e16236);
        let assign16340_e16238: f64 = (assign16340_e16206 / assign16340_e16237);
        (assign16340_e16238, (-((assign16340_e16206 * ((((-(3.0 * locals.var_keq_dn4)) + ((((12.0 * locals.var_keq_dn4) * assign16340_e16217) - (assign16340_e16214 * ((locals.var_k1_dn4 * locals.var_k2) + (locals.var_k1 * locals.var_k2_dn4)))) / (assign16340_e16217 * assign16340_e16217))) + ((((locals.var_keq_dn4 * assign16340_e16223) + (locals.var_keq * (locals.var_inv_k1_dn4 - locals.var_inv_k2_dn4))) * locals.var_dx_wi) + (assign16340_e16224 * locals.var_dx_wi_dn4))) + (((0.3333333333333 * (-(0.25 * locals.var_keq_dn4))) * locals.var_dx_wisq) + (assign16340_e16234 * locals.var_dx_wisq_dn4)))) / (assign16340_e16237 * assign16340_e16237))), (-((assign16340_e16206 * ((((-(3.0 * locals.var_keq_dn6)) + ((((12.0 * locals.var_keq_dn6) * assign16340_e16217) - (assign16340_e16214 * ((locals.var_k1_dn6 * locals.var_k2) + (locals.var_k1 * locals.var_k2_dn6)))) / (assign16340_e16217 * assign16340_e16217))) + ((((locals.var_keq_dn6 * assign16340_e16223) + (locals.var_keq * (locals.var_inv_k1_dn6 - locals.var_inv_k2_dn6))) * locals.var_dx_wi) + (assign16340_e16224 * locals.var_dx_wi_dn6))) + (((0.3333333333333 * (-(0.25 * locals.var_keq_dn6))) * locals.var_dx_wisq) + (assign16340_e16234 * locals.var_dx_wisq_dn6)))) / (assign16340_e16237 * assign16340_e16237))), (-((assign16340_e16206 * ((((-(3.0 * locals.var_keq_dn7)) + ((((12.0 * locals.var_keq_dn7) * assign16340_e16217) - (assign16340_e16214 * ((locals.var_k1_dn7 * locals.var_k2) + (locals.var_k1 * locals.var_k2_dn7)))) / (assign16340_e16217 * assign16340_e16217))) + ((((locals.var_keq_dn7 * assign16340_e16223) + (locals.var_keq * (locals.var_inv_k1_dn7 - locals.var_inv_k2_dn7))) * locals.var_dx_wi) + (assign16340_e16224 * locals.var_dx_wi_dn7))) + (((0.3333333333333 * (-(0.25 * locals.var_keq_dn7))) * locals.var_dx_wisq) + (assign16340_e16234 * locals.var_dx_wisq_dn7)))) / (assign16340_e16237 * assign16340_e16237))), (-((assign16340_e16206 * ((((-(3.0 * locals.var_keq_dn8)) + ((((12.0 * locals.var_keq_dn8) * assign16340_e16217) - (assign16340_e16214 * ((locals.var_k1_dn8 * locals.var_k2) + (locals.var_k1 * locals.var_k2_dn8)))) / (assign16340_e16217 * assign16340_e16217))) + ((((locals.var_keq_dn8 * assign16340_e16223) + (locals.var_keq * (locals.var_inv_k1_dn8 - locals.var_inv_k2_dn8))) * locals.var_dx_wi) + (assign16340_e16224 * locals.var_dx_wi_dn8))) + (((0.3333333333333 * (-(0.25 * locals.var_keq_dn8))) * locals.var_dx_wisq) + (assign16340_e16234 * locals.var_dx_wisq_dn8)))) / (assign16340_e16237 * assign16340_e16237))), (-((assign16340_e16206 * ((((-(3.0 * locals.var_keq_dn9)) + ((((12.0 * locals.var_keq_dn9) * assign16340_e16217) - (assign16340_e16214 * ((locals.var_k1_dn9 * locals.var_k2) + (locals.var_k1 * locals.var_k2_dn9)))) / (assign16340_e16217 * assign16340_e16217))) + ((((locals.var_keq_dn9 * assign16340_e16223) + (locals.var_keq * (locals.var_inv_k1_dn9 - locals.var_inv_k2_dn9))) * locals.var_dx_wi) + (assign16340_e16224 * locals.var_dx_wi_dn9))) + (((0.3333333333333 * (-(0.25 * locals.var_keq_dn9))) * locals.var_dx_wisq) + (assign16340_e16234 * locals.var_dx_wisq_dn9)))) / (assign16340_e16237 * assign16340_e16237))),)
    } else {
        (locals.var_dinf, locals.var_dinf_dn4, locals.var_dinf_dn6, locals.var_dinf_dn7, locals.var_dinf_dn8, locals.var_dinf_dn9,)
    }
};
        locals.var_dinf = assign16340_e16240;
        locals.var_dinf_dn4 = assign16340_e16240_d_n4;
        locals.var_dinf_dn6 = assign16340_e16240_d_n6;
        locals.var_dinf_dn7 = assign16340_e16240_d_n7;
        locals.var_dinf_dn8 = assign16340_e16240_d_n8;
        locals.var_dinf_dn9 = assign16340_e16240_d_n9;

        let assign16350_e16243: f64 = (1.0 / locals.var_dinf);
        locals.var_inv_dinf = assign16350_e16243;
        locals.var_inv_dinf_dn4 = (-(locals.var_dinf_dn4 / (locals.var_dinf * locals.var_dinf)));
        locals.var_inv_dinf_dn6 = (-(locals.var_dinf_dn6 / (locals.var_dinf * locals.var_dinf)));
        locals.var_inv_dinf_dn7 = (-(locals.var_dinf_dn7 / (locals.var_dinf * locals.var_dinf)));
        locals.var_inv_dinf_dn8 = (-(locals.var_dinf_dn8 / (locals.var_dinf * locals.var_dinf)));
        locals.var_inv_dinf_dn9 = (-(locals.var_dinf_dn9 / (locals.var_dinf * locals.var_dinf)));

        let assign16360_e16246: f64 = if locals.var_qis > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard608 = assign16360_e16246;

        let (assign16370_e16256, assign16370_e16256_d_n4, assign16370_e16256_d_n6, assign16370_e16256_d_n7, assign16370_e16256_d_n8, assign16370_e16256_d_n9,) = {
    if (locals.var_guard608 != 0.0) {
        let assign16370_e16250: f64 = (100.0 * locals.var_esurf1s);
        let assign16370_e16253: f64 = (100.0 + locals.var_esurf1s);
        let assign16370_e16254: f64 = (assign16370_e16250 / assign16370_e16253);
        (assign16370_e16254, ((((100.0 * locals.var_esurf1s_dn4) * assign16370_e16253) - (assign16370_e16250 * locals.var_esurf1s_dn4)) / (assign16370_e16253 * assign16370_e16253)), ((((100.0 * locals.var_esurf1s_dn6) * assign16370_e16253) - (assign16370_e16250 * locals.var_esurf1s_dn6)) / (assign16370_e16253 * assign16370_e16253)), ((((100.0 * locals.var_esurf1s_dn7) * assign16370_e16253) - (assign16370_e16250 * locals.var_esurf1s_dn7)) / (assign16370_e16253 * assign16370_e16253)), ((((100.0 * locals.var_esurf1s_dn8) * assign16370_e16253) - (assign16370_e16250 * locals.var_esurf1s_dn8)) / (assign16370_e16253 * assign16370_e16253)), ((((100.0 * locals.var_esurf1s_dn9) * assign16370_e16253) - (assign16370_e16250 * locals.var_esurf1s_dn9)) / (assign16370_e16253 * assign16370_e16253)),)
    } else {
        (locals.var_wsat1, locals.var_wsat1_dn4, locals.var_wsat1_dn6, locals.var_wsat1_dn7, locals.var_wsat1_dn8, locals.var_wsat1_dn9,)
    }
};
        locals.var_wsat1 = assign16370_e16256;
        locals.var_wsat1_dn4 = assign16370_e16256_d_n4;
        locals.var_wsat1_dn6 = assign16370_e16256_d_n6;
        locals.var_wsat1_dn7 = assign16370_e16256_d_n7;
        locals.var_wsat1_dn8 = assign16370_e16256_d_n8;
        locals.var_wsat1_dn9 = assign16370_e16256_d_n9;

        let assign16380_e16259: f64 = if locals.var_thesat1_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard609 = assign16380_e16259;

        let (assign16390_e16271, assign16390_e16271_d_n4, assign16390_e16271_d_n6, assign16390_e16271_d_n7, assign16390_e16271_d_n8, assign16390_e16271_d_n9,) = {
    if ((locals.var_guard608 != 0.0) && (locals.var_guard609 != 0.0)) {
        let assign16390_e16267: f64 = (locals.var_thesat1_i * locals.var_wsat1);
        let assign16390_e16268: f64 = (1.0 - assign16390_e16267);
        let assign16390_e16269: f64 = (1.0 / assign16390_e16268);
        (assign16390_e16269, (-((-(locals.var_thesat1_i * locals.var_wsat1_dn4)) / (assign16390_e16268 * assign16390_e16268))), (-((-(locals.var_thesat1_i * locals.var_wsat1_dn6)) / (assign16390_e16268 * assign16390_e16268))), (-((-(locals.var_thesat1_i * locals.var_wsat1_dn7)) / (assign16390_e16268 * assign16390_e16268))), (-((-(locals.var_thesat1_i * locals.var_wsat1_dn8)) / (assign16390_e16268 * assign16390_e16268))), (-((-(locals.var_thesat1_i * locals.var_wsat1_dn9)) / (assign16390_e16268 * assign16390_e16268))),)
    } else {
        (locals.var_sat_fact1, locals.var_sat_fact1_dn4, locals.var_sat_fact1_dn6, locals.var_sat_fact1_dn7, locals.var_sat_fact1_dn8, locals.var_sat_fact1_dn9,)
    }
};
        locals.var_sat_fact1 = assign16390_e16271;
        locals.var_sat_fact1_dn4 = assign16390_e16271_d_n4;
        locals.var_sat_fact1_dn6 = assign16390_e16271_d_n6;
        locals.var_sat_fact1_dn7 = assign16390_e16271_d_n7;
        locals.var_sat_fact1_dn8 = assign16390_e16271_d_n8;
        locals.var_sat_fact1_dn9 = assign16390_e16271_d_n9;

        let (assign16400_e16282, assign16400_e16282_d_n4, assign16400_e16282_d_n6, assign16400_e16282_d_n7, assign16400_e16282_d_n8, assign16400_e16282_d_n9,) = {
    if ((locals.var_guard608 != 0.0) && (locals.var_guard609 == 0.0)) {
        let assign16400_e16279: f64 = (locals.var_thesat1_i * locals.var_wsat1);
        let assign16400_e16280: f64 = (1.0 + assign16400_e16279);
        (assign16400_e16280, (locals.var_thesat1_i * locals.var_wsat1_dn4), (locals.var_thesat1_i * locals.var_wsat1_dn6), (locals.var_thesat1_i * locals.var_wsat1_dn7), (locals.var_thesat1_i * locals.var_wsat1_dn8), (locals.var_thesat1_i * locals.var_wsat1_dn9),)
    } else {
        (locals.var_sat_fact1, locals.var_sat_fact1_dn4, locals.var_sat_fact1_dn6, locals.var_sat_fact1_dn7, locals.var_sat_fact1_dn8, locals.var_sat_fact1_dn9,)
    }
};
        locals.var_sat_fact1 = assign16400_e16282;
        locals.var_sat_fact1_dn4 = assign16400_e16282_d_n4;
        locals.var_sat_fact1_dn6 = assign16400_e16282_d_n6;
        locals.var_sat_fact1_dn7 = assign16400_e16282_d_n7;
        locals.var_sat_fact1_dn8 = assign16400_e16282_d_n8;
        locals.var_sat_fact1_dn9 = assign16400_e16282_d_n9;

        let (assign16410_e16292, assign16410_e16292_d_n4, assign16410_e16292_d_n6, assign16410_e16292_d_n7, assign16410_e16292_d_n8, assign16410_e16292_d_n9,) = {
    if (locals.var_guard608 != 0.0) {
        let assign16410_e16286: f64 = (100.0 * locals.var_esurf2s);
        let assign16410_e16289: f64 = (100.0 + locals.var_esurf2s);
        let assign16410_e16290: f64 = (assign16410_e16286 / assign16410_e16289);
        (assign16410_e16290, ((((100.0 * locals.var_esurf2s_dn4) * assign16410_e16289) - (assign16410_e16286 * locals.var_esurf2s_dn4)) / (assign16410_e16289 * assign16410_e16289)), ((((100.0 * locals.var_esurf2s_dn6) * assign16410_e16289) - (assign16410_e16286 * locals.var_esurf2s_dn6)) / (assign16410_e16289 * assign16410_e16289)), ((((100.0 * locals.var_esurf2s_dn7) * assign16410_e16289) - (assign16410_e16286 * locals.var_esurf2s_dn7)) / (assign16410_e16289 * assign16410_e16289)), ((((100.0 * locals.var_esurf2s_dn8) * assign16410_e16289) - (assign16410_e16286 * locals.var_esurf2s_dn8)) / (assign16410_e16289 * assign16410_e16289)), ((((100.0 * locals.var_esurf2s_dn9) * assign16410_e16289) - (assign16410_e16286 * locals.var_esurf2s_dn9)) / (assign16410_e16289 * assign16410_e16289)),)
    } else {
        (locals.var_wsat2, locals.var_wsat2_dn4, locals.var_wsat2_dn6, locals.var_wsat2_dn7, locals.var_wsat2_dn8, locals.var_wsat2_dn9,)
    }
};
        locals.var_wsat2 = assign16410_e16292;
        locals.var_wsat2_dn4 = assign16410_e16292_d_n4;
        locals.var_wsat2_dn6 = assign16410_e16292_d_n6;
        locals.var_wsat2_dn7 = assign16410_e16292_d_n7;
        locals.var_wsat2_dn8 = assign16410_e16292_d_n8;
        locals.var_wsat2_dn9 = assign16410_e16292_d_n9;

        let assign16420_e16295: f64 = if locals.var_thesat2_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard610 = assign16420_e16295;

        let (assign16430_e16307, assign16430_e16307_d_n4, assign16430_e16307_d_n6, assign16430_e16307_d_n7, assign16430_e16307_d_n8, assign16430_e16307_d_n9,) = {
    if ((locals.var_guard608 != 0.0) && (locals.var_guard610 != 0.0)) {
        let assign16430_e16303: f64 = (locals.var_thesat2_i * locals.var_wsat2);
        let assign16430_e16304: f64 = (1.0 - assign16430_e16303);
        let assign16430_e16305: f64 = (1.0 / assign16430_e16304);
        (assign16430_e16305, (-((-(locals.var_thesat2_i * locals.var_wsat2_dn4)) / (assign16430_e16304 * assign16430_e16304))), (-((-(locals.var_thesat2_i * locals.var_wsat2_dn6)) / (assign16430_e16304 * assign16430_e16304))), (-((-(locals.var_thesat2_i * locals.var_wsat2_dn7)) / (assign16430_e16304 * assign16430_e16304))), (-((-(locals.var_thesat2_i * locals.var_wsat2_dn8)) / (assign16430_e16304 * assign16430_e16304))), (-((-(locals.var_thesat2_i * locals.var_wsat2_dn9)) / (assign16430_e16304 * assign16430_e16304))),)
    } else {
        (locals.var_sat_fact2, locals.var_sat_fact2_dn4, locals.var_sat_fact2_dn6, locals.var_sat_fact2_dn7, locals.var_sat_fact2_dn8, locals.var_sat_fact2_dn9,)
    }
};
        locals.var_sat_fact2 = assign16430_e16307;
        locals.var_sat_fact2_dn4 = assign16430_e16307_d_n4;
        locals.var_sat_fact2_dn6 = assign16430_e16307_d_n6;
        locals.var_sat_fact2_dn7 = assign16430_e16307_d_n7;
        locals.var_sat_fact2_dn8 = assign16430_e16307_d_n8;
        locals.var_sat_fact2_dn9 = assign16430_e16307_d_n9;

        let (assign16440_e16318, assign16440_e16318_d_n4, assign16440_e16318_d_n6, assign16440_e16318_d_n7, assign16440_e16318_d_n8, assign16440_e16318_d_n9,) = {
    if ((locals.var_guard608 != 0.0) && (locals.var_guard610 == 0.0)) {
        let assign16440_e16315: f64 = (locals.var_thesat2_i * locals.var_wsat2);
        let assign16440_e16316: f64 = (1.0 + assign16440_e16315);
        (assign16440_e16316, (locals.var_thesat2_i * locals.var_wsat2_dn4), (locals.var_thesat2_i * locals.var_wsat2_dn6), (locals.var_thesat2_i * locals.var_wsat2_dn7), (locals.var_thesat2_i * locals.var_wsat2_dn8), (locals.var_thesat2_i * locals.var_wsat2_dn9),)
    } else {
        (locals.var_sat_fact2, locals.var_sat_fact2_dn4, locals.var_sat_fact2_dn6, locals.var_sat_fact2_dn7, locals.var_sat_fact2_dn8, locals.var_sat_fact2_dn9,)
    }
};
        locals.var_sat_fact2 = assign16440_e16318;
        locals.var_sat_fact2_dn4 = assign16440_e16318_d_n4;
        locals.var_sat_fact2_dn6 = assign16440_e16318_d_n6;
        locals.var_sat_fact2_dn7 = assign16440_e16318_d_n7;
        locals.var_sat_fact2_dn8 = assign16440_e16318_d_n8;
        locals.var_sat_fact2_dn9 = assign16440_e16318_d_n9;

        let (assign16450_e16338, assign16450_e16338_d_n4, assign16450_e16338_d_n6, assign16450_e16338_d_n7, assign16450_e16338_d_n8, assign16450_e16338_d_n9,) = {
    if (locals.var_guard608 != 0.0) {
        let assign16450_e16322: f64 = (locals.var_dqsqs_dxn_qi * locals.var_sums);
        let assign16450_e16325: f64 = (locals.var_a1s * locals.var_a2s);
        let assign16450_e16326: f64 = (assign16450_e16322 / assign16450_e16325);
        let assign16450_e16329: f64 = (locals.var_aexp1s / locals.var_a1s);
        let assign16450_e16332: f64 = (locals.var_aexp2s / locals.var_a2s);
        let assign16450_e16333: f64 = (assign16450_e16329 + assign16450_e16332);
        let assign16450_e16335: f64 = (assign16450_e16333 / locals.var_qis);
        let assign16450_e16336: f64 = (assign16450_e16326 - assign16450_e16335);
        (assign16450_e16336, ((((((locals.var_dqsqs_dxn_qi_dn4 * locals.var_sums) + (locals.var_dqsqs_dxn_qi * locals.var_sums_dn4)) * assign16450_e16325) - (assign16450_e16322 * ((locals.var_a1s_dn4 * locals.var_a2s) + (locals.var_a1s * locals.var_a2s_dn4)))) / (assign16450_e16325 * assign16450_e16325)) - (((((((locals.var_aexp1s_dn4 * locals.var_a1s) - (locals.var_aexp1s * locals.var_a1s_dn4)) / (locals.var_a1s * locals.var_a1s)) + (((locals.var_aexp2s_dn4 * locals.var_a2s) - (locals.var_aexp2s * locals.var_a2s_dn4)) / (locals.var_a2s * locals.var_a2s))) * locals.var_qis) - (assign16450_e16333 * locals.var_qis_dn4)) / (locals.var_qis * locals.var_qis))), ((((((locals.var_dqsqs_dxn_qi_dn6 * locals.var_sums) + (locals.var_dqsqs_dxn_qi * locals.var_sums_dn6)) * assign16450_e16325) - (assign16450_e16322 * ((locals.var_a1s_dn6 * locals.var_a2s) + (locals.var_a1s * locals.var_a2s_dn6)))) / (assign16450_e16325 * assign16450_e16325)) - (((((((locals.var_aexp1s_dn6 * locals.var_a1s) - (locals.var_aexp1s * locals.var_a1s_dn6)) / (locals.var_a1s * locals.var_a1s)) + (((locals.var_aexp2s_dn6 * locals.var_a2s) - (locals.var_aexp2s * locals.var_a2s_dn6)) / (locals.var_a2s * locals.var_a2s))) * locals.var_qis) - (assign16450_e16333 * locals.var_qis_dn6)) / (locals.var_qis * locals.var_qis))), ((((((locals.var_dqsqs_dxn_qi_dn7 * locals.var_sums) + (locals.var_dqsqs_dxn_qi * locals.var_sums_dn7)) * assign16450_e16325) - (assign16450_e16322 * ((locals.var_a1s_dn7 * locals.var_a2s) + (locals.var_a1s * locals.var_a2s_dn7)))) / (assign16450_e16325 * assign16450_e16325)) - (((((((locals.var_aexp1s_dn7 * locals.var_a1s) - (locals.var_aexp1s * locals.var_a1s_dn7)) / (locals.var_a1s * locals.var_a1s)) + (((locals.var_aexp2s_dn7 * locals.var_a2s) - (locals.var_aexp2s * locals.var_a2s_dn7)) / (locals.var_a2s * locals.var_a2s))) * locals.var_qis) - (assign16450_e16333 * locals.var_qis_dn7)) / (locals.var_qis * locals.var_qis))), ((((((locals.var_dqsqs_dxn_qi_dn8 * locals.var_sums) + (locals.var_dqsqs_dxn_qi * locals.var_sums_dn8)) * assign16450_e16325) - (assign16450_e16322 * ((locals.var_a1s_dn8 * locals.var_a2s) + (locals.var_a1s * locals.var_a2s_dn8)))) / (assign16450_e16325 * assign16450_e16325)) - (((((((locals.var_aexp1s_dn8 * locals.var_a1s) - (locals.var_aexp1s * locals.var_a1s_dn8)) / (locals.var_a1s * locals.var_a1s)) + (((locals.var_aexp2s_dn8 * locals.var_a2s) - (locals.var_aexp2s * locals.var_a2s_dn8)) / (locals.var_a2s * locals.var_a2s))) * locals.var_qis) - (assign16450_e16333 * locals.var_qis_dn8)) / (locals.var_qis * locals.var_qis))), ((((((locals.var_dqsqs_dxn_qi_dn9 * locals.var_sums) + (locals.var_dqsqs_dxn_qi * locals.var_sums_dn9)) * assign16450_e16325) - (assign16450_e16322 * ((locals.var_a1s_dn9 * locals.var_a2s) + (locals.var_a1s * locals.var_a2s_dn9)))) / (assign16450_e16325 * assign16450_e16325)) - (((((((locals.var_aexp1s_dn9 * locals.var_a1s) - (locals.var_aexp1s * locals.var_a1s_dn9)) / (locals.var_a1s * locals.var_a1s)) + (((locals.var_aexp2s_dn9 * locals.var_a2s) - (locals.var_aexp2s * locals.var_a2s_dn9)) / (locals.var_a2s * locals.var_a2s))) * locals.var_qis) - (assign16450_e16333 * locals.var_qis_dn9)) / (locals.var_qis * locals.var_qis))),)
    } else {
        (locals.var_dqis_dxn_qi, locals.var_dqis_dxn_qi_dn4, locals.var_dqis_dxn_qi_dn6, locals.var_dqis_dxn_qi_dn7, locals.var_dqis_dxn_qi_dn8, locals.var_dqis_dxn_qi_dn9,)
    }
};
        locals.var_dqis_dxn_qi = assign16450_e16338;
        locals.var_dqis_dxn_qi_dn4 = assign16450_e16338_d_n4;
        locals.var_dqis_dxn_qi_dn6 = assign16450_e16338_d_n6;
        locals.var_dqis_dxn_qi_dn7 = assign16450_e16338_d_n7;
        locals.var_dqis_dxn_qi_dn8 = assign16450_e16338_d_n8;
        locals.var_dqis_dxn_qi_dn9 = assign16450_e16338_d_n9;

        let (assign16460_e16348, assign16460_e16348_d_n4, assign16460_e16348_d_n6, assign16460_e16348_d_n7, assign16460_e16348_d_n8, assign16460_e16348_d_n9,) = {
    if (locals.var_guard608 != 0.0) {
        let assign16460_e16342: f64 = (locals.var_dqis_dxn_qi * locals.var_qis);
        let assign16460_e16345: f64 = (locals.var_dqis_dxn_qi + 1.0);
        let assign16460_e16346: f64 = (assign16460_e16342 / assign16460_e16345);
        (assign16460_e16346, (((((locals.var_dqis_dxn_qi_dn4 * locals.var_qis) + (locals.var_dqis_dxn_qi * locals.var_qis_dn4)) * assign16460_e16345) - (assign16460_e16342 * locals.var_dqis_dxn_qi_dn4)) / (assign16460_e16345 * assign16460_e16345)), (((((locals.var_dqis_dxn_qi_dn6 * locals.var_qis) + (locals.var_dqis_dxn_qi * locals.var_qis_dn6)) * assign16460_e16345) - (assign16460_e16342 * locals.var_dqis_dxn_qi_dn6)) / (assign16460_e16345 * assign16460_e16345)), (((((locals.var_dqis_dxn_qi_dn7 * locals.var_qis) + (locals.var_dqis_dxn_qi * locals.var_qis_dn7)) * assign16460_e16345) - (assign16460_e16342 * locals.var_dqis_dxn_qi_dn7)) / (assign16460_e16345 * assign16460_e16345)), (((((locals.var_dqis_dxn_qi_dn8 * locals.var_qis) + (locals.var_dqis_dxn_qi * locals.var_qis_dn8)) * assign16460_e16345) - (assign16460_e16342 * locals.var_dqis_dxn_qi_dn8)) / (assign16460_e16345 * assign16460_e16345)), (((((locals.var_dqis_dxn_qi_dn9 * locals.var_qis) + (locals.var_dqis_dxn_qi * locals.var_qis_dn9)) * assign16460_e16345) - (assign16460_e16342 * locals.var_dqis_dxn_qi_dn9)) / (assign16460_e16345 * assign16460_e16345)),)
    } else {
        (locals.var_ds, locals.var_ds_dn4, locals.var_ds_dn6, locals.var_ds_dn7, locals.var_ds_dn8, locals.var_ds_dn9,)
    }
};
        locals.var_ds = assign16460_e16348;
        locals.var_ds_dn4 = assign16460_e16348_d_n4;
        locals.var_ds_dn6 = assign16460_e16348_d_n6;
        locals.var_ds_dn7 = assign16460_e16348_d_n7;
        locals.var_ds_dn8 = assign16460_e16348_d_n8;
        locals.var_ds_dn9 = assign16460_e16348_d_n9;

        let (assign16470_e16354, assign16470_e16354_d_n4, assign16470_e16354_d_n6, assign16470_e16354_d_n7, assign16470_e16354_d_n8, assign16470_e16354_d_n9,) = {
    if (locals.var_guard608 != 0.0) {
        let assign16470_e16352: f64 = (locals.var_dinf - locals.var_ds);
        (assign16470_e16352, (locals.var_dinf_dn4 - locals.var_ds_dn4), (locals.var_dinf_dn6 - locals.var_ds_dn6), (locals.var_dinf_dn7 - locals.var_ds_dn7), (locals.var_dinf_dn8 - locals.var_ds_dn8), (locals.var_dinf_dn9 - locals.var_ds_dn9),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign16470_e16354;
        locals.var_temp1_dn4 = assign16470_e16354_d_n4;
        locals.var_temp1_dn6 = assign16470_e16354_d_n6;
        locals.var_temp1_dn7 = assign16470_e16354_d_n7;
        locals.var_temp1_dn8 = assign16470_e16354_d_n8;
        locals.var_temp1_dn9 = assign16470_e16354_d_n9;

        let (assign16480_e16364, assign16480_e16364_d_n4, assign16480_e16364_d_n6, assign16480_e16364_d_n7, assign16480_e16364_d_n8, assign16480_e16364_d_n9,) = {
    if (locals.var_guard608 != 0.0) {
        let assign16480_e16359: f64 = (locals.var_dinf * locals.var_deltaxinf);
        let assign16480_e16360: f64 = (locals.var_qis + assign16480_e16359);
        let assign16480_e16362: f64 = (assign16480_e16360 / locals.var_temp1);
        (assign16480_e16362, ((((locals.var_qis_dn4 + ((locals.var_dinf_dn4 * locals.var_deltaxinf) + (locals.var_dinf * locals.var_deltaxinf_dn4))) * locals.var_temp1) - (assign16480_e16360 * locals.var_temp1_dn4)) / (locals.var_temp1 * locals.var_temp1)), ((((locals.var_qis_dn6 + ((locals.var_dinf_dn6 * locals.var_deltaxinf) + (locals.var_dinf * locals.var_deltaxinf_dn6))) * locals.var_temp1) - (assign16480_e16360 * locals.var_temp1_dn6)) / (locals.var_temp1 * locals.var_temp1)), ((((locals.var_qis_dn7 + ((locals.var_dinf_dn7 * locals.var_deltaxinf) + (locals.var_dinf * locals.var_deltaxinf_dn7))) * locals.var_temp1) - (assign16480_e16360 * locals.var_temp1_dn7)) / (locals.var_temp1 * locals.var_temp1)), ((((locals.var_qis_dn8 + ((locals.var_dinf_dn8 * locals.var_deltaxinf) + (locals.var_dinf * locals.var_deltaxinf_dn8))) * locals.var_temp1) - (assign16480_e16360 * locals.var_temp1_dn8)) / (locals.var_temp1 * locals.var_temp1)), ((((locals.var_qis_dn9 + ((locals.var_dinf_dn9 * locals.var_deltaxinf) + (locals.var_dinf * locals.var_deltaxinf_dn9))) * locals.var_temp1) - (assign16480_e16360 * locals.var_temp1_dn9)) / (locals.var_temp1 * locals.var_temp1)),)
    } else {
        (locals.var_deltaxi, locals.var_deltaxi_dn4, locals.var_deltaxi_dn6, locals.var_deltaxi_dn7, locals.var_deltaxi_dn8, locals.var_deltaxi_dn9,)
    }
};
        locals.var_deltaxi = assign16480_e16364;
        locals.var_deltaxi_dn4 = assign16480_e16364_d_n4;
        locals.var_deltaxi_dn6 = assign16480_e16364_d_n6;
        locals.var_deltaxi_dn7 = assign16480_e16364_d_n7;
        locals.var_deltaxi_dn8 = assign16480_e16364_d_n8;
        locals.var_deltaxi_dn9 = assign16480_e16364_d_n9;

        let (assign16490_e16377, assign16490_e16377_d_n4, assign16490_e16377_d_n6, assign16490_e16377_d_n7, assign16490_e16377_d_n8, assign16490_e16377_d_n9,) = {
    if (locals.var_guard608 != 0.0) {
        let assign16490_e16370: f64 = (locals.var_deltaxi * locals.var_deltaxi);
        let assign16490_e16372: f64 = (assign16490_e16370 + 1e-6);
        let assign16490_e16373: f64 = (assign16490_e16372).sqrt();
        let assign16490_e16374: f64 = (locals.var_deltaxi + assign16490_e16373);
        let assign16490_e16375: f64 = (0.5 * assign16490_e16374);
        (assign16490_e16375, (0.5 * (locals.var_deltaxi_dn4 + (((locals.var_deltaxi_dn4 * locals.var_deltaxi) + (locals.var_deltaxi * locals.var_deltaxi_dn4)) / (2.0 * assign16490_e16373)))), (0.5 * (locals.var_deltaxi_dn6 + (((locals.var_deltaxi_dn6 * locals.var_deltaxi) + (locals.var_deltaxi * locals.var_deltaxi_dn6)) / (2.0 * assign16490_e16373)))), (0.5 * (locals.var_deltaxi_dn7 + (((locals.var_deltaxi_dn7 * locals.var_deltaxi) + (locals.var_deltaxi * locals.var_deltaxi_dn7)) / (2.0 * assign16490_e16373)))), (0.5 * (locals.var_deltaxi_dn8 + (((locals.var_deltaxi_dn8 * locals.var_deltaxi) + (locals.var_deltaxi * locals.var_deltaxi_dn8)) / (2.0 * assign16490_e16373)))), (0.5 * (locals.var_deltaxi_dn9 + (((locals.var_deltaxi_dn9 * locals.var_deltaxi) + (locals.var_deltaxi * locals.var_deltaxi_dn9)) / (2.0 * assign16490_e16373)))),)
    } else {
        (locals.var_deltaxi, locals.var_deltaxi_dn4, locals.var_deltaxi_dn6, locals.var_deltaxi_dn7, locals.var_deltaxi_dn8, locals.var_deltaxi_dn9,)
    }
};
        locals.var_deltaxi = assign16490_e16377;
        locals.var_deltaxi_dn4 = assign16490_e16377_d_n4;
        locals.var_deltaxi_dn6 = assign16490_e16377_d_n6;
        locals.var_deltaxi_dn7 = assign16490_e16377_d_n7;
        locals.var_deltaxi_dn8 = assign16490_e16377_d_n8;
        locals.var_deltaxi_dn9 = assign16490_e16377_d_n9;

        let (assign16500_e16389, assign16500_e16389_d_n4, assign16500_e16389_d_n6, assign16500_e16389_d_n7, assign16500_e16389_d_n8, assign16500_e16389_d_n9,) = {
    if (locals.var_guard608 != 0.0) {
        let assign16500_e16381: f64 = (locals.var_sat_phit_loc / locals.var_gmobs);
        let assign16500_e16383: f64 = (assign16500_e16381 * 0.5);
        let assign16500_e16386: f64 = (locals.var_sat_fact1 + locals.var_sat_fact2);
        let assign16500_e16387: f64 = (assign16500_e16383 * assign16500_e16386);
        (assign16500_e16387, ((((((locals.var_sat_phit_loc_dn4 * locals.var_gmobs) - (locals.var_sat_phit_loc * locals.var_gmobs_dn4)) / (locals.var_gmobs * locals.var_gmobs)) * 0.5) * assign16500_e16386) + (assign16500_e16383 * (locals.var_sat_fact1_dn4 + locals.var_sat_fact2_dn4))), ((((((locals.var_sat_phit_loc_dn6 * locals.var_gmobs) - (locals.var_sat_phit_loc * locals.var_gmobs_dn6)) / (locals.var_gmobs * locals.var_gmobs)) * 0.5) * assign16500_e16386) + (assign16500_e16383 * (locals.var_sat_fact1_dn6 + locals.var_sat_fact2_dn6))), ((((((locals.var_sat_phit_loc_dn7 * locals.var_gmobs) - (locals.var_sat_phit_loc * locals.var_gmobs_dn7)) / (locals.var_gmobs * locals.var_gmobs)) * 0.5) * assign16500_e16386) + (assign16500_e16383 * (locals.var_sat_fact1_dn7 + locals.var_sat_fact2_dn7))), ((((((locals.var_sat_phit_loc_dn8 * locals.var_gmobs) - (locals.var_sat_phit_loc * locals.var_gmobs_dn8)) / (locals.var_gmobs * locals.var_gmobs)) * 0.5) * assign16500_e16386) + (assign16500_e16383 * (locals.var_sat_fact1_dn8 + locals.var_sat_fact2_dn8))), ((((((locals.var_sat_phit_loc_dn9 * locals.var_gmobs) - (locals.var_sat_phit_loc * locals.var_gmobs_dn9)) / (locals.var_gmobs * locals.var_gmobs)) * 0.5) * assign16500_e16386) + (assign16500_e16383 * (locals.var_sat_fact1_dn9 + locals.var_sat_fact2_dn9))),)
    } else {
        (locals.var_gamma, locals.var_gamma_dn4, locals.var_gamma_dn6, locals.var_gamma_dn7, locals.var_gamma_dn8, locals.var_gamma_dn9,)
    }
};
        locals.var_gamma = assign16500_e16389;
        locals.var_gamma_dn4 = assign16500_e16389_d_n4;
        locals.var_gamma_dn6 = assign16500_e16389_d_n6;
        locals.var_gamma_dn7 = assign16500_e16389_d_n7;
        locals.var_gamma_dn8 = assign16500_e16389_d_n8;
        locals.var_gamma_dn9 = assign16500_e16389_d_n9;

        let (assign16510_e16397, assign16510_e16397_d_n4, assign16510_e16397_d_n6, assign16510_e16397_d_n7, assign16510_e16397_d_n8, assign16510_e16397_d_n9,) = {
    if (locals.var_guard608 != 0.0) {
        let assign16510_e16394: f64 = (locals.var_qis / locals.var_ds);
        let assign16510_e16395: f64 = (1.0 - assign16510_e16394);
        (assign16510_e16395, (-(((locals.var_qis_dn4 * locals.var_ds) - (locals.var_qis * locals.var_ds_dn4)) / (locals.var_ds * locals.var_ds))), (-(((locals.var_qis_dn6 * locals.var_ds) - (locals.var_qis * locals.var_ds_dn6)) / (locals.var_ds * locals.var_ds))), (-(((locals.var_qis_dn7 * locals.var_ds) - (locals.var_qis * locals.var_ds_dn7)) / (locals.var_ds * locals.var_ds))), (-(((locals.var_qis_dn8 * locals.var_ds) - (locals.var_qis * locals.var_ds_dn8)) / (locals.var_ds * locals.var_ds))), (-(((locals.var_qis_dn9 * locals.var_ds) - (locals.var_qis * locals.var_ds_dn9)) / (locals.var_ds * locals.var_ds))),)
    } else {
        (locals.var_vs, locals.var_vs_dn4, locals.var_vs_dn6, locals.var_vs_dn7, locals.var_vs_dn8, locals.var_vs_dn9,)
    }
};
        locals.var_vs = assign16510_e16397;
        locals.var_vs_dn4 = assign16510_e16397_d_n4;
        locals.var_vs_dn6 = assign16510_e16397_d_n6;
        locals.var_vs_dn7 = assign16510_e16397_d_n7;
        locals.var_vs_dn8 = assign16510_e16397_d_n8;
        locals.var_vs_dn9 = assign16510_e16397_d_n9;

        let (assign16520_e16403, assign16520_e16403_d_n4, assign16520_e16403_d_n6, assign16520_e16403_d_n7, assign16520_e16403_d_n8, assign16520_e16403_d_n9,) = {
    if (locals.var_guard608 != 0.0) {
        let assign16520_e16401: f64 = (1.0 + locals.var_deltaxinf);
        (assign16520_e16401, locals.var_deltaxinf_dn4, locals.var_deltaxinf_dn6, locals.var_deltaxinf_dn7, locals.var_deltaxinf_dn8, locals.var_deltaxinf_dn9,)
    } else {
        (locals.var_vd, locals.var_vd_dn4, locals.var_vd_dn6, locals.var_vd_dn7, locals.var_vd_dn8, locals.var_vd_dn9,)
    }
};
        locals.var_vd = assign16520_e16403;
        locals.var_vd_dn4 = assign16520_e16403_d_n4;
        locals.var_vd_dn6 = assign16520_e16403_d_n6;
        locals.var_vd_dn7 = assign16520_e16403_d_n7;
        locals.var_vd_dn8 = assign16520_e16403_d_n8;
        locals.var_vd_dn9 = assign16520_e16403_d_n9;

        let (assign16530_e16419, assign16530_e16419_d_n4, assign16530_e16419_d_n6, assign16530_e16419_d_n7, assign16530_e16419_d_n8, assign16530_e16419_d_n9,) = {
    if (locals.var_guard608 != 0.0) {
        let assign16530_e16407: f64 = (2.0 * locals.var_ds);
        let assign16530_e16409: f64 = (assign16530_e16407 - locals.var_qis);
        let assign16530_e16411: f64 = (assign16530_e16409 * locals.var_inv_dinf);
        let assign16530_e16413: f64 = (assign16530_e16411 - 2.0);
        let assign16530_e16415: f64 = (assign16530_e16413 - locals.var_deltaxinf);
        let assign16530_e16417: f64 = (assign16530_e16415 * locals.var_deltaxi);
        (assign16530_e16417, (((((((2.0 * locals.var_ds_dn4) - locals.var_qis_dn4) * locals.var_inv_dinf) + (assign16530_e16409 * locals.var_inv_dinf_dn4)) - locals.var_deltaxinf_dn4) * locals.var_deltaxi) + (assign16530_e16415 * locals.var_deltaxi_dn4)), (((((((2.0 * locals.var_ds_dn6) - locals.var_qis_dn6) * locals.var_inv_dinf) + (assign16530_e16409 * locals.var_inv_dinf_dn6)) - locals.var_deltaxinf_dn6) * locals.var_deltaxi) + (assign16530_e16415 * locals.var_deltaxi_dn6)), (((((((2.0 * locals.var_ds_dn7) - locals.var_qis_dn7) * locals.var_inv_dinf) + (assign16530_e16409 * locals.var_inv_dinf_dn7)) - locals.var_deltaxinf_dn7) * locals.var_deltaxi) + (assign16530_e16415 * locals.var_deltaxi_dn7)), (((((((2.0 * locals.var_ds_dn8) - locals.var_qis_dn8) * locals.var_inv_dinf) + (assign16530_e16409 * locals.var_inv_dinf_dn8)) - locals.var_deltaxinf_dn8) * locals.var_deltaxi) + (assign16530_e16415 * locals.var_deltaxi_dn8)), (((((((2.0 * locals.var_ds_dn9) - locals.var_qis_dn9) * locals.var_inv_dinf) + (assign16530_e16409 * locals.var_inv_dinf_dn9)) - locals.var_deltaxinf_dn9) * locals.var_deltaxi) + (assign16530_e16415 * locals.var_deltaxi_dn9)),)
    } else {
        (locals.var_wd, locals.var_wd_dn4, locals.var_wd_dn6, locals.var_wd_dn7, locals.var_wd_dn8, locals.var_wd_dn9,)
    }
};
        locals.var_wd = assign16530_e16419;
        locals.var_wd_dn4 = assign16530_e16419_d_n4;
        locals.var_wd_dn6 = assign16530_e16419_d_n6;
        locals.var_wd_dn7 = assign16530_e16419_d_n7;
        locals.var_wd_dn8 = assign16530_e16419_d_n8;
        locals.var_wd_dn9 = assign16530_e16419_d_n9;

        let assign16540_e16422: f64 = if locals.var_gamma > 1e-14 { 1.0 } else { 0.0 };
        locals.var_guard611 = assign16540_e16422;

    }

    pub(super) fn stamp_transient_block_41(
        locals: &mut StampLocals,
    ) {
        let (assign16550_e16432, assign16550_e16432_d_n4, assign16550_e16432_d_n6, assign16550_e16432_d_n7, assign16550_e16432_d_n8, assign16550_e16432_d_n9,) = {
    if ((locals.var_guard608 != 0.0) && (locals.var_guard611 != 0.0)) {
        let assign16550_e16429: f64 = (locals.var_gamma * locals.var_gamma);
        let assign16550_e16430: f64 = (2.0 / assign16550_e16429);
        (assign16550_e16430, (-((2.0 * ((locals.var_gamma_dn4 * locals.var_gamma) + (locals.var_gamma * locals.var_gamma_dn4))) / (assign16550_e16429 * assign16550_e16429))), (-((2.0 * ((locals.var_gamma_dn6 * locals.var_gamma) + (locals.var_gamma * locals.var_gamma_dn6))) / (assign16550_e16429 * assign16550_e16429))), (-((2.0 * ((locals.var_gamma_dn7 * locals.var_gamma) + (locals.var_gamma * locals.var_gamma_dn7))) / (assign16550_e16429 * assign16550_e16429))), (-((2.0 * ((locals.var_gamma_dn8 * locals.var_gamma) + (locals.var_gamma * locals.var_gamma_dn8))) / (assign16550_e16429 * assign16550_e16429))), (-((2.0 * ((locals.var_gamma_dn9 * locals.var_gamma) + (locals.var_gamma * locals.var_gamma_dn9))) / (assign16550_e16429 * assign16550_e16429))),)
    } else {
        (locals.var_ps_cub, locals.var_ps_cub_dn4, locals.var_ps_cub_dn6, locals.var_ps_cub_dn7, locals.var_ps_cub_dn8, locals.var_ps_cub_dn9,)
    }
};
        locals.var_ps_cub = assign16550_e16432;
        locals.var_ps_cub_dn4 = assign16550_e16432_d_n4;
        locals.var_ps_cub_dn6 = assign16550_e16432_d_n6;
        locals.var_ps_cub_dn7 = assign16550_e16432_d_n7;
        locals.var_ps_cub_dn8 = assign16550_e16432_d_n8;
        locals.var_ps_cub_dn9 = assign16550_e16432_d_n9;

        let (assign16560_e16440, assign16560_e16440_d_n4, assign16560_e16440_d_n6, assign16560_e16440_d_n7, assign16560_e16440_d_n8, assign16560_e16440_d_n9,) = {
    if ((locals.var_guard608 != 0.0) && (locals.var_guard611 != 0.0)) {
        let assign16560_e16438: f64 = (locals.var_ps_cub * locals.var_vs);
        (assign16560_e16438, ((locals.var_ps_cub_dn4 * locals.var_vs) + (locals.var_ps_cub * locals.var_vs_dn4)), ((locals.var_ps_cub_dn6 * locals.var_vs) + (locals.var_ps_cub * locals.var_vs_dn6)), ((locals.var_ps_cub_dn7 * locals.var_vs) + (locals.var_ps_cub * locals.var_vs_dn7)), ((locals.var_ps_cub_dn8 * locals.var_vs) + (locals.var_ps_cub * locals.var_vs_dn8)), ((locals.var_ps_cub_dn9 * locals.var_vs) + (locals.var_ps_cub * locals.var_vs_dn9)),)
    } else {
        (locals.var_qs_cub, locals.var_qs_cub_dn4, locals.var_qs_cub_dn6, locals.var_qs_cub_dn7, locals.var_qs_cub_dn8, locals.var_qs_cub_dn9,)
    }
};
        locals.var_qs_cub = assign16560_e16440;
        locals.var_qs_cub_dn4 = assign16560_e16440_d_n4;
        locals.var_qs_cub_dn6 = assign16560_e16440_d_n6;
        locals.var_qs_cub_dn7 = assign16560_e16440_d_n7;
        locals.var_qs_cub_dn8 = assign16560_e16440_d_n8;
        locals.var_qs_cub_dn9 = assign16560_e16440_d_n9;

        let (assign16570_e16448, assign16570_e16448_d_n4, assign16570_e16448_d_n6, assign16570_e16448_d_n7, assign16570_e16448_d_n8, assign16570_e16448_d_n9,) = {
    if ((locals.var_guard608 != 0.0) && (locals.var_guard611 != 0.0)) {
        let assign16570_e16446: f64 = (locals.var_ps_cub + locals.var_wd);
        (assign16570_e16446, (locals.var_ps_cub_dn4 + locals.var_wd_dn4), (locals.var_ps_cub_dn6 + locals.var_wd_dn6), (locals.var_ps_cub_dn7 + locals.var_wd_dn7), (locals.var_ps_cub_dn8 + locals.var_wd_dn8), (locals.var_ps_cub_dn9 + locals.var_wd_dn9),)
    } else {
        (locals.var_pd_cub, locals.var_pd_cub_dn4, locals.var_pd_cub_dn6, locals.var_pd_cub_dn7, locals.var_pd_cub_dn8, locals.var_pd_cub_dn9,)
    }
};
        locals.var_pd_cub = assign16570_e16448;
        locals.var_pd_cub_dn4 = assign16570_e16448_d_n4;
        locals.var_pd_cub_dn6 = assign16570_e16448_d_n6;
        locals.var_pd_cub_dn7 = assign16570_e16448_d_n7;
        locals.var_pd_cub_dn8 = assign16570_e16448_d_n8;
        locals.var_pd_cub_dn9 = assign16570_e16448_d_n9;

        let (assign16580_e16456, assign16580_e16456_d_n4, assign16580_e16456_d_n6, assign16580_e16456_d_n7, assign16580_e16456_d_n8, assign16580_e16456_d_n9,) = {
    if ((locals.var_guard608 != 0.0) && (locals.var_guard611 != 0.0)) {
        let assign16580_e16454: f64 = (locals.var_ps_cub * locals.var_vd);
        (assign16580_e16454, ((locals.var_ps_cub_dn4 * locals.var_vd) + (locals.var_ps_cub * locals.var_vd_dn4)), ((locals.var_ps_cub_dn6 * locals.var_vd) + (locals.var_ps_cub * locals.var_vd_dn6)), ((locals.var_ps_cub_dn7 * locals.var_vd) + (locals.var_ps_cub * locals.var_vd_dn7)), ((locals.var_ps_cub_dn8 * locals.var_vd) + (locals.var_ps_cub * locals.var_vd_dn8)), ((locals.var_ps_cub_dn9 * locals.var_vd) + (locals.var_ps_cub * locals.var_vd_dn9)),)
    } else {
        (locals.var_qd_cub, locals.var_qd_cub_dn4, locals.var_qd_cub_dn6, locals.var_qd_cub_dn7, locals.var_qd_cub_dn8, locals.var_qd_cub_dn9,)
    }
};
        locals.var_qd_cub = assign16580_e16456;
        locals.var_qd_cub_dn4 = assign16580_e16456_d_n4;
        locals.var_qd_cub_dn6 = assign16580_e16456_d_n6;
        locals.var_qd_cub_dn7 = assign16580_e16456_d_n7;
        locals.var_qd_cub_dn8 = assign16580_e16456_d_n8;
        locals.var_qd_cub_dn9 = assign16580_e16456_d_n9;

        let (assign16590_e16475, assign16590_e16475_d_n4, assign16590_e16475_d_n6, assign16590_e16475_d_n7, assign16590_e16475_d_n8, assign16590_e16475_d_n9,) = {
    if ((locals.var_guard608 != 0.0) && (locals.var_guard611 != 0.0)) {
        let assign16590_e16462: f64 = (locals.var_qs_cub * locals.var_qs_cub);
        let assign16590_e16465: f64 = (0.148148148148 * locals.var_ps_cub);
        let assign16590_e16467: f64 = (assign16590_e16465 * locals.var_ps_cub);
        let assign16590_e16469: f64 = (assign16590_e16467 * locals.var_ps_cub);
        let assign16590_e16470: f64 = (assign16590_e16462 + assign16590_e16469);
        let assign16590_e16472: f64 = (assign16590_e16470 + 1e-20);
        let assign16590_e16473: f64 = (assign16590_e16472).sqrt();
        (assign16590_e16473, ((((locals.var_qs_cub_dn4 * locals.var_qs_cub) + (locals.var_qs_cub * locals.var_qs_cub_dn4)) + (((((0.148148148148 * locals.var_ps_cub_dn4) * locals.var_ps_cub) + (assign16590_e16465 * locals.var_ps_cub_dn4)) * locals.var_ps_cub) + (assign16590_e16467 * locals.var_ps_cub_dn4))) / (2.0 * assign16590_e16473)), ((((locals.var_qs_cub_dn6 * locals.var_qs_cub) + (locals.var_qs_cub * locals.var_qs_cub_dn6)) + (((((0.148148148148 * locals.var_ps_cub_dn6) * locals.var_ps_cub) + (assign16590_e16465 * locals.var_ps_cub_dn6)) * locals.var_ps_cub) + (assign16590_e16467 * locals.var_ps_cub_dn6))) / (2.0 * assign16590_e16473)), ((((locals.var_qs_cub_dn7 * locals.var_qs_cub) + (locals.var_qs_cub * locals.var_qs_cub_dn7)) + (((((0.148148148148 * locals.var_ps_cub_dn7) * locals.var_ps_cub) + (assign16590_e16465 * locals.var_ps_cub_dn7)) * locals.var_ps_cub) + (assign16590_e16467 * locals.var_ps_cub_dn7))) / (2.0 * assign16590_e16473)), ((((locals.var_qs_cub_dn8 * locals.var_qs_cub) + (locals.var_qs_cub * locals.var_qs_cub_dn8)) + (((((0.148148148148 * locals.var_ps_cub_dn8) * locals.var_ps_cub) + (assign16590_e16465 * locals.var_ps_cub_dn8)) * locals.var_ps_cub) + (assign16590_e16467 * locals.var_ps_cub_dn8))) / (2.0 * assign16590_e16473)), ((((locals.var_qs_cub_dn9 * locals.var_qs_cub) + (locals.var_qs_cub * locals.var_qs_cub_dn9)) + (((((0.148148148148 * locals.var_ps_cub_dn9) * locals.var_ps_cub) + (assign16590_e16465 * locals.var_ps_cub_dn9)) * locals.var_ps_cub) + (assign16590_e16467 * locals.var_ps_cub_dn9))) / (2.0 * assign16590_e16473)),)
    } else {
        (locals.var_racs, locals.var_racs_dn4, locals.var_racs_dn6, locals.var_racs_dn7, locals.var_racs_dn8, locals.var_racs_dn9,)
    }
};
        locals.var_racs = assign16590_e16475;
        locals.var_racs_dn4 = assign16590_e16475_d_n4;
        locals.var_racs_dn6 = assign16590_e16475_d_n6;
        locals.var_racs_dn7 = assign16590_e16475_d_n7;
        locals.var_racs_dn8 = assign16590_e16475_d_n8;
        locals.var_racs_dn9 = assign16590_e16475_d_n9;

        let (assign16600_e16494, assign16600_e16494_d_n4, assign16600_e16494_d_n6, assign16600_e16494_d_n7, assign16600_e16494_d_n8, assign16600_e16494_d_n9,) = {
    if ((locals.var_guard608 != 0.0) && (locals.var_guard611 != 0.0)) {
        let assign16600_e16481: f64 = (locals.var_qd_cub * locals.var_qd_cub);
        let assign16600_e16484: f64 = (0.148148148148 * locals.var_pd_cub);
        let assign16600_e16486: f64 = (assign16600_e16484 * locals.var_pd_cub);
        let assign16600_e16488: f64 = (assign16600_e16486 * locals.var_pd_cub);
        let assign16600_e16489: f64 = (assign16600_e16481 + assign16600_e16488);
        let assign16600_e16491: f64 = (assign16600_e16489 + 1e-20);
        let assign16600_e16492: f64 = (assign16600_e16491).sqrt();
        (assign16600_e16492, ((((locals.var_qd_cub_dn4 * locals.var_qd_cub) + (locals.var_qd_cub * locals.var_qd_cub_dn4)) + (((((0.148148148148 * locals.var_pd_cub_dn4) * locals.var_pd_cub) + (assign16600_e16484 * locals.var_pd_cub_dn4)) * locals.var_pd_cub) + (assign16600_e16486 * locals.var_pd_cub_dn4))) / (2.0 * assign16600_e16492)), ((((locals.var_qd_cub_dn6 * locals.var_qd_cub) + (locals.var_qd_cub * locals.var_qd_cub_dn6)) + (((((0.148148148148 * locals.var_pd_cub_dn6) * locals.var_pd_cub) + (assign16600_e16484 * locals.var_pd_cub_dn6)) * locals.var_pd_cub) + (assign16600_e16486 * locals.var_pd_cub_dn6))) / (2.0 * assign16600_e16492)), ((((locals.var_qd_cub_dn7 * locals.var_qd_cub) + (locals.var_qd_cub * locals.var_qd_cub_dn7)) + (((((0.148148148148 * locals.var_pd_cub_dn7) * locals.var_pd_cub) + (assign16600_e16484 * locals.var_pd_cub_dn7)) * locals.var_pd_cub) + (assign16600_e16486 * locals.var_pd_cub_dn7))) / (2.0 * assign16600_e16492)), ((((locals.var_qd_cub_dn8 * locals.var_qd_cub) + (locals.var_qd_cub * locals.var_qd_cub_dn8)) + (((((0.148148148148 * locals.var_pd_cub_dn8) * locals.var_pd_cub) + (assign16600_e16484 * locals.var_pd_cub_dn8)) * locals.var_pd_cub) + (assign16600_e16486 * locals.var_pd_cub_dn8))) / (2.0 * assign16600_e16492)), ((((locals.var_qd_cub_dn9 * locals.var_qd_cub) + (locals.var_qd_cub * locals.var_qd_cub_dn9)) + (((((0.148148148148 * locals.var_pd_cub_dn9) * locals.var_pd_cub) + (assign16600_e16484 * locals.var_pd_cub_dn9)) * locals.var_pd_cub) + (assign16600_e16486 * locals.var_pd_cub_dn9))) / (2.0 * assign16600_e16492)),)
    } else {
        (locals.var_racd, locals.var_racd_dn4, locals.var_racd_dn6, locals.var_racd_dn7, locals.var_racd_dn8, locals.var_racd_dn9,)
    }
};
        locals.var_racd = assign16600_e16494;
        locals.var_racd_dn4 = assign16600_e16494_d_n4;
        locals.var_racd_dn6 = assign16600_e16494_d_n6;
        locals.var_racd_dn7 = assign16600_e16494_d_n7;
        locals.var_racd_dn8 = assign16600_e16494_d_n8;
        locals.var_racd_dn9 = assign16600_e16494_d_n9;

        let (assign16610_e16518, assign16610_e16518_d_n4, assign16610_e16518_d_n6, assign16610_e16518_d_n7, assign16610_e16518_d_n8, assign16610_e16518_d_n9,) = {
    if ((locals.var_guard608 != 0.0) && (locals.var_guard611 != 0.0)) {
        let assign16610_e16502: f64 = (locals.var_racs + locals.var_qs_cub);
        let assign16610_e16503: f64 = (0.5 * assign16610_e16502);
        let assign16610_e16504: f64 = (assign16610_e16503).ln();
        let assign16610_e16505: f64 = (0.3333333333333 * assign16610_e16504);
        let assign16610_e16506: f64 = (assign16610_e16505).exp();
        let assign16610_e16511: f64 = (locals.var_racs - locals.var_qs_cub);
        let assign16610_e16512: f64 = (0.5 * assign16610_e16511);
        let assign16610_e16513: f64 = (assign16610_e16512).ln();
        let assign16610_e16514: f64 = (0.3333333333333 * assign16610_e16513);
        let assign16610_e16515: f64 = (assign16610_e16514).exp();
        let assign16610_e16516: f64 = (assign16610_e16506 - assign16610_e16515);
        (assign16610_e16516, ((assign16610_e16506 * (0.3333333333333 * ((0.5 * (locals.var_racs_dn4 + locals.var_qs_cub_dn4)) / assign16610_e16503))) - (assign16610_e16515 * (0.3333333333333 * ((0.5 * (locals.var_racs_dn4 - locals.var_qs_cub_dn4)) / assign16610_e16512)))), ((assign16610_e16506 * (0.3333333333333 * ((0.5 * (locals.var_racs_dn6 + locals.var_qs_cub_dn6)) / assign16610_e16503))) - (assign16610_e16515 * (0.3333333333333 * ((0.5 * (locals.var_racs_dn6 - locals.var_qs_cub_dn6)) / assign16610_e16512)))), ((assign16610_e16506 * (0.3333333333333 * ((0.5 * (locals.var_racs_dn7 + locals.var_qs_cub_dn7)) / assign16610_e16503))) - (assign16610_e16515 * (0.3333333333333 * ((0.5 * (locals.var_racs_dn7 - locals.var_qs_cub_dn7)) / assign16610_e16512)))), ((assign16610_e16506 * (0.3333333333333 * ((0.5 * (locals.var_racs_dn8 + locals.var_qs_cub_dn8)) / assign16610_e16503))) - (assign16610_e16515 * (0.3333333333333 * ((0.5 * (locals.var_racs_dn8 - locals.var_qs_cub_dn8)) / assign16610_e16512)))), ((assign16610_e16506 * (0.3333333333333 * ((0.5 * (locals.var_racs_dn9 + locals.var_qs_cub_dn9)) / assign16610_e16503))) - (assign16610_e16515 * (0.3333333333333 * ((0.5 * (locals.var_racs_dn9 - locals.var_qs_cub_dn9)) / assign16610_e16512)))),)
    } else {
        (locals.var_deltaxsats, locals.var_deltaxsats_dn4, locals.var_deltaxsats_dn6, locals.var_deltaxsats_dn7, locals.var_deltaxsats_dn8, locals.var_deltaxsats_dn9,)
    }
};
        locals.var_deltaxsats = assign16610_e16518;
        locals.var_deltaxsats_dn4 = assign16610_e16518_d_n4;
        locals.var_deltaxsats_dn6 = assign16610_e16518_d_n6;
        locals.var_deltaxsats_dn7 = assign16610_e16518_d_n7;
        locals.var_deltaxsats_dn8 = assign16610_e16518_d_n8;
        locals.var_deltaxsats_dn9 = assign16610_e16518_d_n9;

        let (assign16620_e16542, assign16620_e16542_d_n4, assign16620_e16542_d_n6, assign16620_e16542_d_n7, assign16620_e16542_d_n8, assign16620_e16542_d_n9,) = {
    if ((locals.var_guard608 != 0.0) && (locals.var_guard611 != 0.0)) {
        let assign16620_e16526: f64 = (locals.var_racd + locals.var_qd_cub);
        let assign16620_e16527: f64 = (0.5 * assign16620_e16526);
        let assign16620_e16528: f64 = (assign16620_e16527).ln();
        let assign16620_e16529: f64 = (0.3333333333333 * assign16620_e16528);
        let assign16620_e16530: f64 = (assign16620_e16529).exp();
        let assign16620_e16535: f64 = (locals.var_racd - locals.var_qd_cub);
        let assign16620_e16536: f64 = (0.5 * assign16620_e16535);
        let assign16620_e16537: f64 = (assign16620_e16536).ln();
        let assign16620_e16538: f64 = (0.3333333333333 * assign16620_e16537);
        let assign16620_e16539: f64 = (assign16620_e16538).exp();
        let assign16620_e16540: f64 = (assign16620_e16530 - assign16620_e16539);
        (assign16620_e16540, ((assign16620_e16530 * (0.3333333333333 * ((0.5 * (locals.var_racd_dn4 + locals.var_qd_cub_dn4)) / assign16620_e16527))) - (assign16620_e16539 * (0.3333333333333 * ((0.5 * (locals.var_racd_dn4 - locals.var_qd_cub_dn4)) / assign16620_e16536)))), ((assign16620_e16530 * (0.3333333333333 * ((0.5 * (locals.var_racd_dn6 + locals.var_qd_cub_dn6)) / assign16620_e16527))) - (assign16620_e16539 * (0.3333333333333 * ((0.5 * (locals.var_racd_dn6 - locals.var_qd_cub_dn6)) / assign16620_e16536)))), ((assign16620_e16530 * (0.3333333333333 * ((0.5 * (locals.var_racd_dn7 + locals.var_qd_cub_dn7)) / assign16620_e16527))) - (assign16620_e16539 * (0.3333333333333 * ((0.5 * (locals.var_racd_dn7 - locals.var_qd_cub_dn7)) / assign16620_e16536)))), ((assign16620_e16530 * (0.3333333333333 * ((0.5 * (locals.var_racd_dn8 + locals.var_qd_cub_dn8)) / assign16620_e16527))) - (assign16620_e16539 * (0.3333333333333 * ((0.5 * (locals.var_racd_dn8 - locals.var_qd_cub_dn8)) / assign16620_e16536)))), ((assign16620_e16530 * (0.3333333333333 * ((0.5 * (locals.var_racd_dn9 + locals.var_qd_cub_dn9)) / assign16620_e16527))) - (assign16620_e16539 * (0.3333333333333 * ((0.5 * (locals.var_racd_dn9 - locals.var_qd_cub_dn9)) / assign16620_e16536)))),)
    } else {
        (locals.var_deltaxsatd, locals.var_deltaxsatd_dn4, locals.var_deltaxsatd_dn6, locals.var_deltaxsatd_dn7, locals.var_deltaxsatd_dn8, locals.var_deltaxsatd_dn9,)
    }
};
        locals.var_deltaxsatd = assign16620_e16542;
        locals.var_deltaxsatd_dn4 = assign16620_e16542_d_n4;
        locals.var_deltaxsatd_dn6 = assign16620_e16542_d_n6;
        locals.var_deltaxsatd_dn7 = assign16620_e16542_d_n7;
        locals.var_deltaxsatd_dn8 = assign16620_e16542_d_n8;
        locals.var_deltaxsatd_dn9 = assign16620_e16542_d_n9;

        let (assign16630_e16549, assign16630_e16549_d_n4, assign16630_e16549_d_n6, assign16630_e16549_d_n7, assign16630_e16549_d_n8, assign16630_e16549_d_n9,) = {
    if ((locals.var_guard608 != 0.0) && (locals.var_guard611 == 0.0)) {
        (locals.var_vs, locals.var_vs_dn4, locals.var_vs_dn6, locals.var_vs_dn7, locals.var_vs_dn8, locals.var_vs_dn9,)
    } else {
        (locals.var_deltaxsats, locals.var_deltaxsats_dn4, locals.var_deltaxsats_dn6, locals.var_deltaxsats_dn7, locals.var_deltaxsats_dn8, locals.var_deltaxsats_dn9,)
    }
};
        locals.var_deltaxsats = assign16630_e16549;
        locals.var_deltaxsats_dn4 = assign16630_e16549_d_n4;
        locals.var_deltaxsats_dn6 = assign16630_e16549_d_n6;
        locals.var_deltaxsats_dn7 = assign16630_e16549_d_n7;
        locals.var_deltaxsats_dn8 = assign16630_e16549_d_n8;
        locals.var_deltaxsats_dn9 = assign16630_e16549_d_n9;

        let (assign16640_e16556, assign16640_e16556_d_n4, assign16640_e16556_d_n6, assign16640_e16556_d_n7, assign16640_e16556_d_n8, assign16640_e16556_d_n9,) = {
    if ((locals.var_guard608 != 0.0) && (locals.var_guard611 == 0.0)) {
        (locals.var_vd, locals.var_vd_dn4, locals.var_vd_dn6, locals.var_vd_dn7, locals.var_vd_dn8, locals.var_vd_dn9,)
    } else {
        (locals.var_deltaxsatd, locals.var_deltaxsatd_dn4, locals.var_deltaxsatd_dn6, locals.var_deltaxsatd_dn7, locals.var_deltaxsatd_dn8, locals.var_deltaxsatd_dn9,)
    }
};
        locals.var_deltaxsatd = assign16640_e16556;
        locals.var_deltaxsatd_dn4 = assign16640_e16556_d_n4;
        locals.var_deltaxsatd_dn6 = assign16640_e16556_d_n6;
        locals.var_deltaxsatd_dn7 = assign16640_e16556_d_n7;
        locals.var_deltaxsatd_dn8 = assign16640_e16556_d_n8;
        locals.var_deltaxsatd_dn9 = assign16640_e16556_d_n9;

        let (assign16650_e16562, assign16650_e16562_d_n4, assign16650_e16562_d_n6, assign16650_e16562_d_n7, assign16650_e16562_d_n8, assign16650_e16562_d_n9,) = {
    if (locals.var_guard608 != 0.0) {
        let assign16650_e16560: f64 = (locals.var_temp1 * locals.var_temp1);
        (assign16650_e16560, ((locals.var_temp1_dn4 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn4)), ((locals.var_temp1_dn6 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn6)), ((locals.var_temp1_dn7 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn7)), ((locals.var_temp1_dn8 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn8)), ((locals.var_temp1_dn9 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn9)),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign16650_e16562;
        locals.var_temp3_dn4 = assign16650_e16562_d_n4;
        locals.var_temp3_dn6 = assign16650_e16562_d_n6;
        locals.var_temp3_dn7 = assign16650_e16562_d_n7;
        locals.var_temp3_dn8 = assign16650_e16562_d_n8;
        locals.var_temp3_dn9 = assign16650_e16562_d_n9;

        let (assign16660_e16585, assign16660_e16585_d_n4, assign16660_e16585_d_n6, assign16660_e16585_d_n7, assign16660_e16585_d_n8, assign16660_e16585_d_n9,) = {
    if (locals.var_guard608 != 0.0) {
        let assign16660_e16566: f64 = (0.94 * 0.5);
        let assign16660_e16569: f64 = (locals.var_deltaxsats + locals.var_deltaxsatd);
        let assign16660_e16572: f64 = (locals.var_deltaxsats - locals.var_deltaxsatd);
        let assign16660_e16575: f64 = (locals.var_deltaxsats - locals.var_deltaxsatd);
        let assign16660_e16576: f64 = (assign16660_e16572 * assign16660_e16575);
        let assign16660_e16579: f64 = (10.0 * locals.var_temp3);
        let assign16660_e16580: f64 = (assign16660_e16576 + assign16660_e16579);
        let assign16660_e16581: f64 = (assign16660_e16580).sqrt();
        let assign16660_e16582: f64 = (assign16660_e16569 + assign16660_e16581);
        let assign16660_e16583: f64 = (assign16660_e16566 * assign16660_e16582);
        (assign16660_e16583, (assign16660_e16566 * ((locals.var_deltaxsats_dn4 + locals.var_deltaxsatd_dn4) + (((((locals.var_deltaxsats_dn4 - locals.var_deltaxsatd_dn4) * assign16660_e16575) + (assign16660_e16572 * (locals.var_deltaxsats_dn4 - locals.var_deltaxsatd_dn4))) + (10.0 * locals.var_temp3_dn4)) / (2.0 * assign16660_e16581)))), (assign16660_e16566 * ((locals.var_deltaxsats_dn6 + locals.var_deltaxsatd_dn6) + (((((locals.var_deltaxsats_dn6 - locals.var_deltaxsatd_dn6) * assign16660_e16575) + (assign16660_e16572 * (locals.var_deltaxsats_dn6 - locals.var_deltaxsatd_dn6))) + (10.0 * locals.var_temp3_dn6)) / (2.0 * assign16660_e16581)))), (assign16660_e16566 * ((locals.var_deltaxsats_dn7 + locals.var_deltaxsatd_dn7) + (((((locals.var_deltaxsats_dn7 - locals.var_deltaxsatd_dn7) * assign16660_e16575) + (assign16660_e16572 * (locals.var_deltaxsats_dn7 - locals.var_deltaxsatd_dn7))) + (10.0 * locals.var_temp3_dn7)) / (2.0 * assign16660_e16581)))), (assign16660_e16566 * ((locals.var_deltaxsats_dn8 + locals.var_deltaxsatd_dn8) + (((((locals.var_deltaxsats_dn8 - locals.var_deltaxsatd_dn8) * assign16660_e16575) + (assign16660_e16572 * (locals.var_deltaxsats_dn8 - locals.var_deltaxsatd_dn8))) + (10.0 * locals.var_temp3_dn8)) / (2.0 * assign16660_e16581)))), (assign16660_e16566 * ((locals.var_deltaxsats_dn9 + locals.var_deltaxsatd_dn9) + (((((locals.var_deltaxsats_dn9 - locals.var_deltaxsatd_dn9) * assign16660_e16575) + (assign16660_e16572 * (locals.var_deltaxsats_dn9 - locals.var_deltaxsatd_dn9))) + (10.0 * locals.var_temp3_dn9)) / (2.0 * assign16660_e16581)))),)
    } else {
        (locals.var_deltaxsat, locals.var_deltaxsat_dn4, locals.var_deltaxsat_dn6, locals.var_deltaxsat_dn7, locals.var_deltaxsat_dn8, locals.var_deltaxsat_dn9,)
    }
};
        locals.var_deltaxsat = assign16660_e16585;
        locals.var_deltaxsat_dn4 = assign16660_e16585_d_n4;
        locals.var_deltaxsat_dn6 = assign16660_e16585_d_n6;
        locals.var_deltaxsat_dn7 = assign16660_e16585_d_n7;
        locals.var_deltaxsat_dn8 = assign16660_e16585_d_n8;
        locals.var_deltaxsat_dn9 = assign16660_e16585_d_n9;

        let (assign16670_e16593, assign16670_e16593_d_n4, assign16670_e16593_d_n6, assign16670_e16593_d_n7, assign16670_e16593_d_n8, assign16670_e16593_d_n9,) = {
    if (locals.var_guard608 != 0.0) {
        let assign16670_e16590: f64 = (locals.var_ds * locals.var_deltaxsat);
        let assign16670_e16591: f64 = (locals.var_qis + assign16670_e16590);
        (assign16670_e16591, (locals.var_qis_dn4 + ((locals.var_ds_dn4 * locals.var_deltaxsat) + (locals.var_ds * locals.var_deltaxsat_dn4))), (locals.var_qis_dn6 + ((locals.var_ds_dn6 * locals.var_deltaxsat) + (locals.var_ds * locals.var_deltaxsat_dn6))), (locals.var_qis_dn7 + ((locals.var_ds_dn7 * locals.var_deltaxsat) + (locals.var_ds * locals.var_deltaxsat_dn7))), (locals.var_qis_dn8 + ((locals.var_ds_dn8 * locals.var_deltaxsat) + (locals.var_ds * locals.var_deltaxsat_dn8))), (locals.var_qis_dn9 + ((locals.var_ds_dn9 * locals.var_deltaxsat) + (locals.var_ds * locals.var_deltaxsat_dn9))),)
    } else {
        (locals.var_qidsats, locals.var_qidsats_dn4, locals.var_qidsats_dn6, locals.var_qidsats_dn7, locals.var_qidsats_dn8, locals.var_qidsats_dn9,)
    }
};
        locals.var_qidsats = assign16670_e16593;
        locals.var_qidsats_dn4 = assign16670_e16593_d_n4;
        locals.var_qidsats_dn6 = assign16670_e16593_d_n6;
        locals.var_qidsats_dn7 = assign16670_e16593_d_n7;
        locals.var_qidsats_dn8 = assign16670_e16593_d_n8;
        locals.var_qidsats_dn9 = assign16670_e16593_d_n9;

        let (assign16680_e16601, assign16680_e16601_d_n4, assign16680_e16601_d_n6, assign16680_e16601_d_n7, assign16680_e16601_d_n8, assign16680_e16601_d_n9,) = {
    if (locals.var_guard608 != 0.0) {
        let assign16680_e16598: f64 = (locals.var_deltaxsat - locals.var_deltaxinf);
        let assign16680_e16599: f64 = (locals.var_dinf * assign16680_e16598);
        (assign16680_e16599, ((locals.var_dinf_dn4 * assign16680_e16598) + (locals.var_dinf * (locals.var_deltaxsat_dn4 - locals.var_deltaxinf_dn4))), ((locals.var_dinf_dn6 * assign16680_e16598) + (locals.var_dinf * (locals.var_deltaxsat_dn6 - locals.var_deltaxinf_dn6))), ((locals.var_dinf_dn7 * assign16680_e16598) + (locals.var_dinf * (locals.var_deltaxsat_dn7 - locals.var_deltaxinf_dn7))), ((locals.var_dinf_dn8 * assign16680_e16598) + (locals.var_dinf * (locals.var_deltaxsat_dn8 - locals.var_deltaxinf_dn8))), ((locals.var_dinf_dn9 * assign16680_e16598) + (locals.var_dinf * (locals.var_deltaxsat_dn9 - locals.var_deltaxinf_dn9))),)
    } else {
        (locals.var_qidsatd, locals.var_qidsatd_dn4, locals.var_qidsatd_dn6, locals.var_qidsatd_dn7, locals.var_qidsatd_dn8, locals.var_qidsatd_dn9,)
    }
};
        locals.var_qidsatd = assign16680_e16601;
        locals.var_qidsatd_dn4 = assign16680_e16601_d_n4;
        locals.var_qidsatd_dn6 = assign16680_e16601_d_n6;
        locals.var_qidsatd_dn7 = assign16680_e16601_d_n7;
        locals.var_qidsatd_dn8 = assign16680_e16601_d_n8;
        locals.var_qidsatd_dn9 = assign16680_e16601_d_n9;

        let (assign16690_e16622, assign16690_e16622_d_n4, assign16690_e16622_d_n6, assign16690_e16622_d_n7, assign16690_e16622_d_n8, assign16690_e16622_d_n9,) = {
    if (locals.var_guard608 != 0.0) {
        let assign16690_e16606: f64 = (locals.var_qidsats + locals.var_qidsatd);
        let assign16690_e16609: f64 = (locals.var_qidsats - locals.var_qidsatd);
        let assign16690_e16612: f64 = (locals.var_qidsats - locals.var_qidsatd);
        let assign16690_e16613: f64 = (assign16690_e16609 * assign16690_e16612);
        let assign16690_e16616: f64 = (36.0 * locals.var_temp3);
        let assign16690_e16617: f64 = (assign16690_e16613 + assign16690_e16616);
        let assign16690_e16618: f64 = (assign16690_e16617).sqrt();
        let assign16690_e16619: f64 = (assign16690_e16606 + assign16690_e16618);
        let assign16690_e16620: f64 = (0.5 * assign16690_e16619);
        (assign16690_e16620, (0.5 * ((locals.var_qidsats_dn4 + locals.var_qidsatd_dn4) + (((((locals.var_qidsats_dn4 - locals.var_qidsatd_dn4) * assign16690_e16612) + (assign16690_e16609 * (locals.var_qidsats_dn4 - locals.var_qidsatd_dn4))) + (36.0 * locals.var_temp3_dn4)) / (2.0 * assign16690_e16618)))), (0.5 * ((locals.var_qidsats_dn6 + locals.var_qidsatd_dn6) + (((((locals.var_qidsats_dn6 - locals.var_qidsatd_dn6) * assign16690_e16612) + (assign16690_e16609 * (locals.var_qidsats_dn6 - locals.var_qidsatd_dn6))) + (36.0 * locals.var_temp3_dn6)) / (2.0 * assign16690_e16618)))), (0.5 * ((locals.var_qidsats_dn7 + locals.var_qidsatd_dn7) + (((((locals.var_qidsats_dn7 - locals.var_qidsatd_dn7) * assign16690_e16612) + (assign16690_e16609 * (locals.var_qidsats_dn7 - locals.var_qidsatd_dn7))) + (36.0 * locals.var_temp3_dn7)) / (2.0 * assign16690_e16618)))), (0.5 * ((locals.var_qidsats_dn8 + locals.var_qidsatd_dn8) + (((((locals.var_qidsats_dn8 - locals.var_qidsatd_dn8) * assign16690_e16612) + (assign16690_e16609 * (locals.var_qidsats_dn8 - locals.var_qidsatd_dn8))) + (36.0 * locals.var_temp3_dn8)) / (2.0 * assign16690_e16618)))), (0.5 * ((locals.var_qidsats_dn9 + locals.var_qidsatd_dn9) + (((((locals.var_qidsats_dn9 - locals.var_qidsatd_dn9) * assign16690_e16612) + (assign16690_e16609 * (locals.var_qidsats_dn9 - locals.var_qidsatd_dn9))) + (36.0 * locals.var_temp3_dn9)) / (2.0 * assign16690_e16618)))),)
    } else {
        (locals.var_qidsat, locals.var_qidsat_dn4, locals.var_qidsat_dn6, locals.var_qidsat_dn7, locals.var_qidsat_dn8, locals.var_qidsat_dn9,)
    }
};
        locals.var_qidsat = assign16690_e16622;
        locals.var_qidsat_dn4 = assign16690_e16622_d_n4;
        locals.var_qidsat_dn6 = assign16690_e16622_d_n6;
        locals.var_qidsat_dn7 = assign16690_e16622_d_n7;
        locals.var_qidsat_dn8 = assign16690_e16622_d_n8;
        locals.var_qidsat_dn9 = assign16690_e16622_d_n9;

        let (assign16700_e16627, assign16700_e16627_d_n4, assign16700_e16627_d_n6, assign16700_e16627_d_n7, assign16700_e16627_d_n8, assign16700_e16627_d_n9,) = {
    if (locals.var_guard608 == 0.0) {
        (locals.var_dinf, locals.var_dinf_dn4, locals.var_dinf_dn6, locals.var_dinf_dn7, locals.var_dinf_dn8, locals.var_dinf_dn9,)
    } else {
        (locals.var_ds, locals.var_ds_dn4, locals.var_ds_dn6, locals.var_ds_dn7, locals.var_ds_dn8, locals.var_ds_dn9,)
    }
};
        locals.var_ds = assign16700_e16627;
        locals.var_ds_dn4 = assign16700_e16627_d_n4;
        locals.var_ds_dn6 = assign16700_e16627_d_n6;
        locals.var_ds_dn7 = assign16700_e16627_d_n7;
        locals.var_ds_dn8 = assign16700_e16627_d_n8;
        locals.var_ds_dn9 = assign16700_e16627_d_n9;

        let (assign16710_e16636, assign16710_e16636_d_n4, assign16710_e16636_d_n6, assign16710_e16636_d_n7, assign16710_e16636_d_n8, assign16710_e16636_d_n9,) = {
    if (locals.var_guard608 == 0.0) {
        let assign16710_e16633: f64 = (1.0 + locals.var_deltaxinf);
        let assign16710_e16634: f64 = (0.94 * assign16710_e16633);
        (assign16710_e16634, (0.94 * locals.var_deltaxinf_dn4), (0.94 * locals.var_deltaxinf_dn6), (0.94 * locals.var_deltaxinf_dn7), (0.94 * locals.var_deltaxinf_dn8), (0.94 * locals.var_deltaxinf_dn9),)
    } else {
        (locals.var_deltaxsat, locals.var_deltaxsat_dn4, locals.var_deltaxsat_dn6, locals.var_deltaxsat_dn7, locals.var_deltaxsat_dn8, locals.var_deltaxsat_dn9,)
    }
};
        locals.var_deltaxsat = assign16710_e16636;
        locals.var_deltaxsat_dn4 = assign16710_e16636_d_n4;
        locals.var_deltaxsat_dn6 = assign16710_e16636_d_n6;
        locals.var_deltaxsat_dn7 = assign16710_e16636_d_n7;
        locals.var_deltaxsat_dn8 = assign16710_e16636_d_n8;
        locals.var_deltaxsat_dn9 = assign16710_e16636_d_n9;

        let (assign16720_e16651, assign16720_e16651_d_n4, assign16720_e16651_d_n6, assign16720_e16651_d_n7, assign16720_e16651_d_n8, assign16720_e16651_d_n9,) = {
    if (locals.var_guard608 == 0.0) {
        let assign16720_e16641: f64 = (0.5 * locals.var_qis);
        let assign16720_e16646: f64 = (0.5 * locals.var_deltaxinf);
        let assign16720_e16647: f64 = (locals.var_deltaxsat - assign16720_e16646);
        let assign16720_e16648: f64 = (locals.var_dinf * assign16720_e16647);
        let assign16720_e16649: f64 = (assign16720_e16641 + assign16720_e16648);
        (assign16720_e16649, ((0.5 * locals.var_qis_dn4) + ((locals.var_dinf_dn4 * assign16720_e16647) + (locals.var_dinf * (locals.var_deltaxsat_dn4 - (0.5 * locals.var_deltaxinf_dn4))))), ((0.5 * locals.var_qis_dn6) + ((locals.var_dinf_dn6 * assign16720_e16647) + (locals.var_dinf * (locals.var_deltaxsat_dn6 - (0.5 * locals.var_deltaxinf_dn6))))), ((0.5 * locals.var_qis_dn7) + ((locals.var_dinf_dn7 * assign16720_e16647) + (locals.var_dinf * (locals.var_deltaxsat_dn7 - (0.5 * locals.var_deltaxinf_dn7))))), ((0.5 * locals.var_qis_dn8) + ((locals.var_dinf_dn8 * assign16720_e16647) + (locals.var_dinf * (locals.var_deltaxsat_dn8 - (0.5 * locals.var_deltaxinf_dn8))))), ((0.5 * locals.var_qis_dn9) + ((locals.var_dinf_dn9 * assign16720_e16647) + (locals.var_dinf * (locals.var_deltaxsat_dn9 - (0.5 * locals.var_deltaxinf_dn9))))),)
    } else {
        (locals.var_qidsat, locals.var_qidsat_dn4, locals.var_qidsat_dn6, locals.var_qidsat_dn7, locals.var_qidsat_dn8, locals.var_qidsat_dn9,)
    }
};
        locals.var_qidsat = assign16720_e16651;
        locals.var_qidsat_dn4 = assign16720_e16651_d_n4;
        locals.var_qidsat_dn6 = assign16720_e16651_d_n6;
        locals.var_qidsat_dn7 = assign16720_e16651_d_n7;
        locals.var_qidsat_dn8 = assign16720_e16651_d_n8;
        locals.var_qidsat_dn9 = assign16720_e16651_d_n9;

        let assign16730_e16654: f64 = (locals.var_qidsat - 0.5);
        let assign16730_e16656: f64 = if assign16730_e16654 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard612 = assign16730_e16656;

        let (assign16740_e16666, assign16740_e16666_d_n4, assign16740_e16666_d_n6, assign16740_e16666_d_n7, assign16740_e16666_d_n8, assign16740_e16666_d_n9,) = {
    if (locals.var_guard612 != 0.0) {
        let assign16740_e16661: f64 = (locals.var_qidsat - 0.5);
        let assign16740_e16662: f64 = (assign16740_e16661).exp();
        let assign16740_e16663: f64 = (1.0 + assign16740_e16662);
        let assign16740_e16664: f64 = (assign16740_e16663).ln();
        (assign16740_e16664, ((assign16740_e16662 * locals.var_qidsat_dn4) / assign16740_e16663), ((assign16740_e16662 * locals.var_qidsat_dn6) / assign16740_e16663), ((assign16740_e16662 * locals.var_qidsat_dn7) / assign16740_e16663), ((assign16740_e16662 * locals.var_qidsat_dn8) / assign16740_e16663), ((assign16740_e16662 * locals.var_qidsat_dn9) / assign16740_e16663),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign16740_e16666;
        locals.var_temp1_dn4 = assign16740_e16666_d_n4;
        locals.var_temp1_dn6 = assign16740_e16666_d_n6;
        locals.var_temp1_dn7 = assign16740_e16666_d_n7;
        locals.var_temp1_dn8 = assign16740_e16666_d_n8;
        locals.var_temp1_dn9 = assign16740_e16666_d_n9;

        let (assign16750_e16673, assign16750_e16673_d_n4, assign16750_e16673_d_n6, assign16750_e16673_d_n7, assign16750_e16673_d_n8, assign16750_e16673_d_n9,) = {
    if (locals.var_guard612 == 0.0) {
        let assign16750_e16671: f64 = (locals.var_qidsat - 0.5);
        (assign16750_e16671, locals.var_qidsat_dn4, locals.var_qidsat_dn6, locals.var_qidsat_dn7, locals.var_qidsat_dn8, locals.var_qidsat_dn9,)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign16750_e16673;
        locals.var_temp1_dn4 = assign16750_e16673_d_n4;
        locals.var_temp1_dn6 = assign16750_e16673_d_n6;
        locals.var_temp1_dn7 = assign16750_e16673_d_n7;
        locals.var_temp1_dn8 = assign16750_e16673_d_n8;
        locals.var_temp1_dn9 = assign16750_e16673_d_n9;

        let assign16760_e16676: f64 = (locals.var_temp1 + 0.5);
        locals.var_temp2 = assign16760_e16676;
        locals.var_temp2_dn4 = locals.var_temp1_dn4;
        locals.var_temp2_dn6 = locals.var_temp1_dn6;
        locals.var_temp2_dn7 = locals.var_temp1_dn7;
        locals.var_temp2_dn8 = locals.var_temp1_dn8;
        locals.var_temp2_dn9 = locals.var_temp1_dn9;

        let assign16770_e16680: f64 = (locals.var_qis / locals.var_temp2);
        let assign16770_e16681: f64 = (assign16770_e16680).ln();
        let assign16770_e16682: f64 = (locals.var_deltaxsat + assign16770_e16681);
        locals.var_temp3 = assign16770_e16682;
        locals.var_temp3_dn4 = (locals.var_deltaxsat_dn4 + ((((locals.var_qis_dn4 * locals.var_temp2) - (locals.var_qis * locals.var_temp2_dn4)) / (locals.var_temp2 * locals.var_temp2)) / assign16770_e16680));
        locals.var_temp3_dn6 = (locals.var_deltaxsat_dn6 + ((((locals.var_qis_dn6 * locals.var_temp2) - (locals.var_qis * locals.var_temp2_dn6)) / (locals.var_temp2 * locals.var_temp2)) / assign16770_e16680));
        locals.var_temp3_dn7 = (locals.var_deltaxsat_dn7 + ((((locals.var_qis_dn7 * locals.var_temp2) - (locals.var_qis * locals.var_temp2_dn7)) / (locals.var_temp2 * locals.var_temp2)) / assign16770_e16680));
        locals.var_temp3_dn8 = (locals.var_deltaxsat_dn8 + ((((locals.var_qis_dn8 * locals.var_temp2) - (locals.var_qis * locals.var_temp2_dn8)) / (locals.var_temp2 * locals.var_temp2)) / assign16770_e16680));
        locals.var_temp3_dn9 = (locals.var_deltaxsat_dn9 + ((((locals.var_qis_dn9 * locals.var_temp2) - (locals.var_qis * locals.var_temp2_dn9)) / (locals.var_temp2 * locals.var_temp2)) / assign16770_e16680));

        let assign16780_e16685: f64 = (locals.var_temp3 - 6.0);
        let assign16780_e16687: f64 = if assign16780_e16685 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard613 = assign16780_e16687;

        let (assign16790_e16697, assign16790_e16697_d_n4, assign16790_e16697_d_n6, assign16790_e16697_d_n7, assign16790_e16697_d_n8, assign16790_e16697_d_n9,) = {
    if (locals.var_guard613 != 0.0) {
        let assign16790_e16692: f64 = (locals.var_temp3 - 6.0);
        let assign16790_e16693: f64 = (assign16790_e16692).exp();
        let assign16790_e16694: f64 = (1.0 + assign16790_e16693);
        let assign16790_e16695: f64 = (assign16790_e16694).ln();
        (assign16790_e16695, ((assign16790_e16693 * locals.var_temp3_dn4) / assign16790_e16694), ((assign16790_e16693 * locals.var_temp3_dn6) / assign16790_e16694), ((assign16790_e16693 * locals.var_temp3_dn7) / assign16790_e16694), ((assign16790_e16693 * locals.var_temp3_dn8) / assign16790_e16694), ((assign16790_e16693 * locals.var_temp3_dn9) / assign16790_e16694),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign16790_e16697;
        locals.var_temp1_dn4 = assign16790_e16697_d_n4;
        locals.var_temp1_dn6 = assign16790_e16697_d_n6;
        locals.var_temp1_dn7 = assign16790_e16697_d_n7;
        locals.var_temp1_dn8 = assign16790_e16697_d_n8;
        locals.var_temp1_dn9 = assign16790_e16697_d_n9;

        let (assign16800_e16704, assign16800_e16704_d_n4, assign16800_e16704_d_n6, assign16800_e16704_d_n7, assign16800_e16704_d_n8, assign16800_e16704_d_n9,) = {
    if (locals.var_guard613 == 0.0) {
        let assign16800_e16702: f64 = (locals.var_temp3 - 6.0);
        (assign16800_e16702, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign16800_e16704;
        locals.var_temp1_dn4 = assign16800_e16704_d_n4;
        locals.var_temp1_dn6 = assign16800_e16704_d_n6;
        locals.var_temp1_dn7 = assign16800_e16704_d_n7;
        locals.var_temp1_dn8 = assign16800_e16704_d_n8;
        locals.var_temp1_dn9 = assign16800_e16704_d_n9;

        let assign16810_e16707: f64 = (locals.var_temp1 + 6.0);
        locals.var_temp3 = assign16810_e16707;
        locals.var_temp3_dn4 = locals.var_temp1_dn4;
        locals.var_temp3_dn6 = locals.var_temp1_dn6;
        locals.var_temp3_dn7 = locals.var_temp1_dn7;
        locals.var_temp3_dn8 = locals.var_temp1_dn8;
        locals.var_temp3_dn9 = locals.var_temp1_dn9;

        let assign16820_e16710: f64 = (locals.var_xsatmax - locals.var_temp3);
        let assign16820_e16712: f64 = if assign16820_e16710 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard614 = assign16820_e16712;

        let (assign16830_e16722, assign16830_e16722_d_n4, assign16830_e16722_d_n6, assign16830_e16722_d_n7, assign16830_e16722_d_n8, assign16830_e16722_d_n9,) = {
    if (locals.var_guard614 != 0.0) {
        let assign16830_e16717: f64 = (locals.var_xsatmax - locals.var_temp3);
        let assign16830_e16718: f64 = (assign16830_e16717).exp();
        let assign16830_e16719: f64 = (1.0 + assign16830_e16718);
        let assign16830_e16720: f64 = (assign16830_e16719).ln();
        (assign16830_e16720, ((assign16830_e16718 * (locals.var_xsatmax_dn4 - locals.var_temp3_dn4)) / assign16830_e16719), ((assign16830_e16718 * (locals.var_xsatmax_dn6 - locals.var_temp3_dn6)) / assign16830_e16719), ((assign16830_e16718 * (locals.var_xsatmax_dn7 - locals.var_temp3_dn7)) / assign16830_e16719), ((assign16830_e16718 * (locals.var_xsatmax_dn8 - locals.var_temp3_dn8)) / assign16830_e16719), ((assign16830_e16718 * (locals.var_xsatmax_dn9 - locals.var_temp3_dn9)) / assign16830_e16719),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign16830_e16722;
        locals.var_temp1_dn4 = assign16830_e16722_d_n4;
        locals.var_temp1_dn6 = assign16830_e16722_d_n6;
        locals.var_temp1_dn7 = assign16830_e16722_d_n7;
        locals.var_temp1_dn8 = assign16830_e16722_d_n8;
        locals.var_temp1_dn9 = assign16830_e16722_d_n9;

        let (assign16840_e16729, assign16840_e16729_d_n4, assign16840_e16729_d_n6, assign16840_e16729_d_n7, assign16840_e16729_d_n8, assign16840_e16729_d_n9,) = {
    if (locals.var_guard614 == 0.0) {
        let assign16840_e16727: f64 = (locals.var_xsatmax - locals.var_temp3);
        (assign16840_e16727, (locals.var_xsatmax_dn4 - locals.var_temp3_dn4), (locals.var_xsatmax_dn6 - locals.var_temp3_dn6), (locals.var_xsatmax_dn7 - locals.var_temp3_dn7), (locals.var_xsatmax_dn8 - locals.var_temp3_dn8), (locals.var_xsatmax_dn9 - locals.var_temp3_dn9),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign16840_e16729;
        locals.var_temp1_dn4 = assign16840_e16729_d_n4;
        locals.var_temp1_dn6 = assign16840_e16729_d_n6;
        locals.var_temp1_dn7 = assign16840_e16729_d_n7;
        locals.var_temp1_dn8 = assign16840_e16729_d_n8;
        locals.var_temp1_dn9 = assign16840_e16729_d_n9;

        let assign16850_e16732: f64 = (locals.var_xsatmax - locals.var_temp1);
        locals.var_xndssat = assign16850_e16732;
        locals.var_xndssat_dn4 = (locals.var_xsatmax_dn4 - locals.var_temp1_dn4);
        locals.var_xndssat_dn6 = (locals.var_xsatmax_dn6 - locals.var_temp1_dn6);
        locals.var_xndssat_dn7 = (locals.var_xsatmax_dn7 - locals.var_temp1_dn7);
        locals.var_xndssat_dn8 = (locals.var_xsatmax_dn8 - locals.var_temp1_dn8);
        locals.var_xndssat_dn9 = (locals.var_xsatmax_dn9 - locals.var_temp1_dn9);

        let assign16860_e16735: f64 = (locals.var_xd / locals.var_xndssat);
        locals.var_temp1 = assign16860_e16735;
        locals.var_temp1_dn4 = (((locals.var_xd_dn4 * locals.var_xndssat) - (locals.var_xd * locals.var_xndssat_dn4)) / (locals.var_xndssat * locals.var_xndssat));
        locals.var_temp1_dn6 = (((locals.var_xd_dn6 * locals.var_xndssat) - (locals.var_xd * locals.var_xndssat_dn6)) / (locals.var_xndssat * locals.var_xndssat));
        locals.var_temp1_dn7 = (((locals.var_xd_dn7 * locals.var_xndssat) - (locals.var_xd * locals.var_xndssat_dn7)) / (locals.var_xndssat * locals.var_xndssat));
        locals.var_temp1_dn8 = (((locals.var_xd_dn8 * locals.var_xndssat) - (locals.var_xd * locals.var_xndssat_dn8)) / (locals.var_xndssat * locals.var_xndssat));
        locals.var_temp1_dn9 = (((locals.var_xd_dn9 * locals.var_xndssat) - (locals.var_xd * locals.var_xndssat_dn9)) / (locals.var_xndssat * locals.var_xndssat));

        let assign16870_e16738: f64 = (locals.var_temp1 * locals.var_temp1);
        locals.var_temp2 = assign16870_e16738;
        locals.var_temp2_dn4 = ((locals.var_temp1_dn4 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn4));
        locals.var_temp2_dn6 = ((locals.var_temp1_dn6 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn6));
        locals.var_temp2_dn7 = ((locals.var_temp1_dn7 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn7));
        locals.var_temp2_dn8 = ((locals.var_temp1_dn8 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn8));
        locals.var_temp2_dn9 = ((locals.var_temp1_dn9 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn9));

        let assign16880_e16741: f64 = (locals.var_temp2 * locals.var_temp2);
        locals.var_temp3 = assign16880_e16741;
        locals.var_temp3_dn4 = ((locals.var_temp2_dn4 * locals.var_temp2) + (locals.var_temp2 * locals.var_temp2_dn4));
        locals.var_temp3_dn6 = ((locals.var_temp2_dn6 * locals.var_temp2) + (locals.var_temp2 * locals.var_temp2_dn6));
        locals.var_temp3_dn7 = ((locals.var_temp2_dn7 * locals.var_temp2) + (locals.var_temp2 * locals.var_temp2_dn7));
        locals.var_temp3_dn8 = ((locals.var_temp2_dn8 * locals.var_temp2) + (locals.var_temp2 * locals.var_temp2_dn8));
        locals.var_temp3_dn9 = ((locals.var_temp2_dn9 * locals.var_temp2) + (locals.var_temp2 * locals.var_temp2_dn9));

        let assign16890_e16744: f64 = (locals.var_temp3 * locals.var_temp3);
        locals.var_temp4 = assign16890_e16744;
        locals.var_temp4_dn4 = ((locals.var_temp3_dn4 * locals.var_temp3) + (locals.var_temp3 * locals.var_temp3_dn4));
        locals.var_temp4_dn6 = ((locals.var_temp3_dn6 * locals.var_temp3) + (locals.var_temp3 * locals.var_temp3_dn6));
        locals.var_temp4_dn7 = ((locals.var_temp3_dn7 * locals.var_temp3) + (locals.var_temp3 * locals.var_temp3_dn7));
        locals.var_temp4_dn8 = ((locals.var_temp3_dn8 * locals.var_temp3) + (locals.var_temp3 * locals.var_temp3_dn8));
        locals.var_temp4_dn9 = ((locals.var_temp3_dn9 * locals.var_temp3) + (locals.var_temp3 * locals.var_temp3_dn9));

    }

    pub(super) fn stamp_transient_block_42(
        locals: &mut StampLocals,
    ) {
        let assign16900_e16749: f64 = (locals.var_gamax_loc * locals.var_temp3);
        let assign16900_e16750: f64 = (1.0 + assign16900_e16749);
        let assign16900_e16751: f64 = (assign16900_e16750).ln();
        let assign16900_e16752: f64 = (2.666666666667 * assign16900_e16751);
        let assign16900_e16753: f64 = (assign16900_e16752).exp();
        locals.var_temp = assign16900_e16753;
        locals.var_temp_dn4 = (assign16900_e16753 * (2.666666666667 * ((locals.var_gamax_loc * locals.var_temp3_dn4) / assign16900_e16750)));
        locals.var_temp_dn6 = (assign16900_e16753 * (2.666666666667 * ((locals.var_gamax_loc * locals.var_temp3_dn6) / assign16900_e16750)));
        locals.var_temp_dn7 = (assign16900_e16753 * (2.666666666667 * ((locals.var_gamax_loc * locals.var_temp3_dn7) / assign16900_e16750)));
        locals.var_temp_dn8 = (assign16900_e16753 * (2.666666666667 * ((locals.var_gamax_loc * locals.var_temp3_dn8) / assign16900_e16750)));
        locals.var_temp_dn9 = (assign16900_e16753 * (2.666666666667 * ((locals.var_gamax_loc * locals.var_temp3_dn9) / assign16900_e16750)));

        let assign16910_e16756: f64 = (-0.0625);
        let assign16910_e16760: f64 = (locals.var_temp4 * locals.var_temp4);
        let assign16910_e16761: f64 = (locals.var_temp + assign16910_e16760);
        let assign16910_e16762: f64 = (assign16910_e16761).ln();
        let assign16910_e16763: f64 = (assign16910_e16756 * assign16910_e16762);
        let assign16910_e16764: f64 = (assign16910_e16763).exp();
        let assign16910_e16765: f64 = (locals.var_xd * assign16910_e16764);
        locals.var_xdeff = assign16910_e16765;
        locals.var_xdeff_dn4 = ((locals.var_xd_dn4 * assign16910_e16764) + (locals.var_xd * (assign16910_e16764 * (assign16910_e16756 * ((locals.var_temp_dn4 + ((locals.var_temp4_dn4 * locals.var_temp4) + (locals.var_temp4 * locals.var_temp4_dn4))) / assign16910_e16761)))));
        locals.var_xdeff_dn6 = ((locals.var_xd_dn6 * assign16910_e16764) + (locals.var_xd * (assign16910_e16764 * (assign16910_e16756 * ((locals.var_temp_dn6 + ((locals.var_temp4_dn6 * locals.var_temp4) + (locals.var_temp4 * locals.var_temp4_dn6))) / assign16910_e16761)))));
        locals.var_xdeff_dn7 = ((locals.var_xd_dn7 * assign16910_e16764) + (locals.var_xd * (assign16910_e16764 * (assign16910_e16756 * ((locals.var_temp_dn7 + ((locals.var_temp4_dn7 * locals.var_temp4) + (locals.var_temp4 * locals.var_temp4_dn7))) / assign16910_e16761)))));
        locals.var_xdeff_dn8 = ((locals.var_xd_dn8 * assign16910_e16764) + (locals.var_xd * (assign16910_e16764 * (assign16910_e16756 * ((locals.var_temp_dn8 + ((locals.var_temp4_dn8 * locals.var_temp4) + (locals.var_temp4 * locals.var_temp4_dn8))) / assign16910_e16761)))));
        locals.var_xdeff_dn9 = ((locals.var_xd_dn9 * assign16910_e16764) + (locals.var_xd * (assign16910_e16764 * (assign16910_e16756 * ((locals.var_temp_dn9 + ((locals.var_temp4_dn9 * locals.var_temp4) + (locals.var_temp4 * locals.var_temp4_dn9))) / assign16910_e16761)))));

        let assign16920_e16769: f64 = (locals.var_k1 + 1.0);
        let assign16920_e16770: f64 = (1.0 / assign16920_e16769);
        locals.var_q_temp1 = assign16920_e16770;
        locals.var_q_temp1_dn4 = (-(locals.var_k1_dn4 / (assign16920_e16769 * assign16920_e16769)));
        locals.var_q_temp1_dn6 = (-(locals.var_k1_dn6 / (assign16920_e16769 * assign16920_e16769)));
        locals.var_q_temp1_dn7 = (-(locals.var_k1_dn7 / (assign16920_e16769 * assign16920_e16769)));
        locals.var_q_temp1_dn8 = (-(locals.var_k1_dn8 / (assign16920_e16769 * assign16920_e16769)));
        locals.var_q_temp1_dn9 = (-(locals.var_k1_dn9 / (assign16920_e16769 * assign16920_e16769)));

        let assign16930_e16774: f64 = (locals.var_k2 + 1.0);
        let assign16930_e16775: f64 = (1.0 / assign16930_e16774);
        locals.var_q_temp2 = assign16930_e16775;
        locals.var_q_temp2_dn4 = (-(locals.var_k2_dn4 / (assign16930_e16774 * assign16930_e16774)));
        locals.var_q_temp2_dn6 = (-(locals.var_k2_dn6 / (assign16930_e16774 * assign16930_e16774)));
        locals.var_q_temp2_dn7 = (-(locals.var_k2_dn7 / (assign16930_e16774 * assign16930_e16774)));
        locals.var_q_temp2_dn8 = (-(locals.var_k2_dn8 / (assign16930_e16774 * assign16930_e16774)));
        locals.var_q_temp2_dn9 = (-(locals.var_k2_dn9 / (assign16930_e16774 * assign16930_e16774)));

        let assign16940_e16779: f64 = (locals.var_k2 * locals.var_q_temp2);
        let assign16940_e16780: f64 = (locals.var_k1 + assign16940_e16779);
        let assign16940_e16782: f64 = (assign16940_e16780 * locals.var_diff_min);
        let assign16940_e16784: f64 = (assign16940_e16782 / locals.var_a0);
        let assign16940_e16785: f64 = (assign16940_e16784).ln();
        let assign16940_e16787: f64 = (assign16940_e16785 + locals.var_xdeff);
        let assign16940_e16789: f64 = (assign16940_e16787 + 3.0);
        locals.var_q_x1sat = assign16940_e16789;
        locals.var_q_x1sat_dn4 = ((((((((locals.var_k1_dn4 + ((locals.var_k2_dn4 * locals.var_q_temp2) + (locals.var_k2 * locals.var_q_temp2_dn4))) * locals.var_diff_min) + (assign16940_e16780 * locals.var_diff_min_dn4)) * locals.var_a0) - (assign16940_e16782 * locals.var_a0_dn4)) / (locals.var_a0 * locals.var_a0)) / assign16940_e16784) + locals.var_xdeff_dn4);
        locals.var_q_x1sat_dn6 = ((((((((locals.var_k1_dn6 + ((locals.var_k2_dn6 * locals.var_q_temp2) + (locals.var_k2 * locals.var_q_temp2_dn6))) * locals.var_diff_min) + (assign16940_e16780 * locals.var_diff_min_dn6)) * locals.var_a0) - (assign16940_e16782 * locals.var_a0_dn6)) / (locals.var_a0 * locals.var_a0)) / assign16940_e16784) + locals.var_xdeff_dn6);
        locals.var_q_x1sat_dn7 = ((((((((locals.var_k1_dn7 + ((locals.var_k2_dn7 * locals.var_q_temp2) + (locals.var_k2 * locals.var_q_temp2_dn7))) * locals.var_diff_min) + (assign16940_e16780 * locals.var_diff_min_dn7)) * locals.var_a0) - (assign16940_e16782 * locals.var_a0_dn7)) / (locals.var_a0 * locals.var_a0)) / assign16940_e16784) + locals.var_xdeff_dn7);
        locals.var_q_x1sat_dn8 = ((((((((locals.var_k1_dn8 + ((locals.var_k2_dn8 * locals.var_q_temp2) + (locals.var_k2 * locals.var_q_temp2_dn8))) * locals.var_diff_min) + (assign16940_e16780 * locals.var_diff_min_dn8)) * locals.var_a0) - (assign16940_e16782 * locals.var_a0_dn8)) / (locals.var_a0 * locals.var_a0)) / assign16940_e16784) + locals.var_xdeff_dn8);
        locals.var_q_x1sat_dn9 = ((((((((locals.var_k1_dn9 + ((locals.var_k2_dn9 * locals.var_q_temp2) + (locals.var_k2 * locals.var_q_temp2_dn9))) * locals.var_diff_min) + (assign16940_e16780 * locals.var_diff_min_dn9)) * locals.var_a0) - (assign16940_e16782 * locals.var_a0_dn9)) / (locals.var_a0 * locals.var_a0)) / assign16940_e16784) + locals.var_xdeff_dn9);

        let assign16950_e16793: f64 = (locals.var_k1 * locals.var_q_temp1);
        let assign16950_e16794: f64 = (locals.var_k2 + assign16950_e16793);
        let assign16950_e16796: f64 = (assign16950_e16794 * locals.var_diff_min);
        let assign16950_e16798: f64 = (assign16950_e16796 / locals.var_a0);
        let assign16950_e16799: f64 = (assign16950_e16798).ln();
        let assign16950_e16801: f64 = (assign16950_e16799 + locals.var_xdeff);
        let assign16950_e16803: f64 = (assign16950_e16801 + 3.0);
        locals.var_q_x2sat = assign16950_e16803;
        locals.var_q_x2sat_dn4 = ((((((((locals.var_k2_dn4 + ((locals.var_k1_dn4 * locals.var_q_temp1) + (locals.var_k1 * locals.var_q_temp1_dn4))) * locals.var_diff_min) + (assign16950_e16794 * locals.var_diff_min_dn4)) * locals.var_a0) - (assign16950_e16796 * locals.var_a0_dn4)) / (locals.var_a0 * locals.var_a0)) / assign16950_e16798) + locals.var_xdeff_dn4);
        locals.var_q_x2sat_dn6 = ((((((((locals.var_k2_dn6 + ((locals.var_k1_dn6 * locals.var_q_temp1) + (locals.var_k1 * locals.var_q_temp1_dn6))) * locals.var_diff_min) + (assign16950_e16794 * locals.var_diff_min_dn6)) * locals.var_a0) - (assign16950_e16796 * locals.var_a0_dn6)) / (locals.var_a0 * locals.var_a0)) / assign16950_e16798) + locals.var_xdeff_dn6);
        locals.var_q_x2sat_dn7 = ((((((((locals.var_k2_dn7 + ((locals.var_k1_dn7 * locals.var_q_temp1) + (locals.var_k1 * locals.var_q_temp1_dn7))) * locals.var_diff_min) + (assign16950_e16794 * locals.var_diff_min_dn7)) * locals.var_a0) - (assign16950_e16796 * locals.var_a0_dn7)) / (locals.var_a0 * locals.var_a0)) / assign16950_e16798) + locals.var_xdeff_dn7);
        locals.var_q_x2sat_dn8 = ((((((((locals.var_k2_dn8 + ((locals.var_k1_dn8 * locals.var_q_temp1) + (locals.var_k1 * locals.var_q_temp1_dn8))) * locals.var_diff_min) + (assign16950_e16794 * locals.var_diff_min_dn8)) * locals.var_a0) - (assign16950_e16796 * locals.var_a0_dn8)) / (locals.var_a0 * locals.var_a0)) / assign16950_e16798) + locals.var_xdeff_dn8);
        locals.var_q_x2sat_dn9 = ((((((((locals.var_k2_dn9 + ((locals.var_k1_dn9 * locals.var_q_temp1) + (locals.var_k1 * locals.var_q_temp1_dn9))) * locals.var_diff_min) + (assign16950_e16794 * locals.var_diff_min_dn9)) * locals.var_a0) - (assign16950_e16796 * locals.var_a0_dn9)) / (locals.var_a0 * locals.var_a0)) / assign16950_e16798) + locals.var_xdeff_dn9);

        let assign16960_e16806: f64 = (locals.var_q_x1sat - locals.var_x1_wi0);
        let assign16960_e16808: f64 = (assign16960_e16806 * 0.3333333333333);
        let assign16960_e16810: f64 = if assign16960_e16808 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard615 = assign16960_e16810;

        let (assign16970_e16822, assign16970_e16822_d_n4, assign16970_e16822_d_n6, assign16970_e16822_d_n7, assign16970_e16822_d_n8, assign16970_e16822_d_n9,) = {
    if (locals.var_guard615 != 0.0) {
        let assign16970_e16815: f64 = (locals.var_q_x1sat - locals.var_x1_wi0);
        let assign16970_e16817: f64 = (assign16970_e16815 * 0.3333333333333);
        let assign16970_e16818: f64 = (assign16970_e16817).exp();
        let assign16970_e16819: f64 = (1.0 + assign16970_e16818);
        let assign16970_e16820: f64 = (assign16970_e16819).ln();
        (assign16970_e16820, ((assign16970_e16818 * ((locals.var_q_x1sat_dn4 - locals.var_x1_wi0_dn4) * 0.3333333333333)) / assign16970_e16819), ((assign16970_e16818 * ((locals.var_q_x1sat_dn6 - locals.var_x1_wi0_dn6) * 0.3333333333333)) / assign16970_e16819), ((assign16970_e16818 * ((locals.var_q_x1sat_dn7 - locals.var_x1_wi0_dn7) * 0.3333333333333)) / assign16970_e16819), ((assign16970_e16818 * ((locals.var_q_x1sat_dn8 - locals.var_x1_wi0_dn8) * 0.3333333333333)) / assign16970_e16819), ((assign16970_e16818 * ((locals.var_q_x1sat_dn9 - locals.var_x1_wi0_dn9) * 0.3333333333333)) / assign16970_e16819),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign16970_e16822;
        locals.var_q_temp3_dn4 = assign16970_e16822_d_n4;
        locals.var_q_temp3_dn6 = assign16970_e16822_d_n6;
        locals.var_q_temp3_dn7 = assign16970_e16822_d_n7;
        locals.var_q_temp3_dn8 = assign16970_e16822_d_n8;
        locals.var_q_temp3_dn9 = assign16970_e16822_d_n9;

        let (assign16980_e16831, assign16980_e16831_d_n4, assign16980_e16831_d_n6, assign16980_e16831_d_n7, assign16980_e16831_d_n8, assign16980_e16831_d_n9,) = {
    if (locals.var_guard615 == 0.0) {
        let assign16980_e16827: f64 = (locals.var_q_x1sat - locals.var_x1_wi0);
        let assign16980_e16829: f64 = (assign16980_e16827 * 0.3333333333333);
        (assign16980_e16829, ((locals.var_q_x1sat_dn4 - locals.var_x1_wi0_dn4) * 0.3333333333333), ((locals.var_q_x1sat_dn6 - locals.var_x1_wi0_dn6) * 0.3333333333333), ((locals.var_q_x1sat_dn7 - locals.var_x1_wi0_dn7) * 0.3333333333333), ((locals.var_q_x1sat_dn8 - locals.var_x1_wi0_dn8) * 0.3333333333333), ((locals.var_q_x1sat_dn9 - locals.var_x1_wi0_dn9) * 0.3333333333333),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign16980_e16831;
        locals.var_q_temp3_dn4 = assign16980_e16831_d_n4;
        locals.var_q_temp3_dn6 = assign16980_e16831_d_n6;
        locals.var_q_temp3_dn7 = assign16980_e16831_d_n7;
        locals.var_q_temp3_dn8 = assign16980_e16831_d_n8;
        locals.var_q_temp3_dn9 = assign16980_e16831_d_n9;

        let assign16990_e16835: f64 = (3.0 * locals.var_q_temp3);
        let assign16990_e16836: f64 = (locals.var_q_x1sat - assign16990_e16835);
        locals.var_q_x1 = assign16990_e16836;
        locals.var_q_x1_dn4 = (locals.var_q_x1sat_dn4 - (3.0 * locals.var_q_temp3_dn4));
        locals.var_q_x1_dn6 = (locals.var_q_x1sat_dn6 - (3.0 * locals.var_q_temp3_dn6));
        locals.var_q_x1_dn7 = (locals.var_q_x1sat_dn7 - (3.0 * locals.var_q_temp3_dn7));
        locals.var_q_x1_dn8 = (locals.var_q_x1sat_dn8 - (3.0 * locals.var_q_temp3_dn8));
        locals.var_q_x1_dn9 = (locals.var_q_x1sat_dn9 - (3.0 * locals.var_q_temp3_dn9));

        let assign17000_e16839: f64 = (locals.var_q_x2sat - locals.var_x2_wi0);
        let assign17000_e16841: f64 = (assign17000_e16839 * 0.3333333333333);
        let assign17000_e16843: f64 = if assign17000_e16841 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard616 = assign17000_e16843;

        let (assign17010_e16855, assign17010_e16855_d_n4, assign17010_e16855_d_n6, assign17010_e16855_d_n7, assign17010_e16855_d_n8, assign17010_e16855_d_n9,) = {
    if (locals.var_guard616 != 0.0) {
        let assign17010_e16848: f64 = (locals.var_q_x2sat - locals.var_x2_wi0);
        let assign17010_e16850: f64 = (assign17010_e16848 * 0.3333333333333);
        let assign17010_e16851: f64 = (assign17010_e16850).exp();
        let assign17010_e16852: f64 = (1.0 + assign17010_e16851);
        let assign17010_e16853: f64 = (assign17010_e16852).ln();
        (assign17010_e16853, ((assign17010_e16851 * ((locals.var_q_x2sat_dn4 - locals.var_x2_wi0_dn4) * 0.3333333333333)) / assign17010_e16852), ((assign17010_e16851 * ((locals.var_q_x2sat_dn6 - locals.var_x2_wi0_dn6) * 0.3333333333333)) / assign17010_e16852), ((assign17010_e16851 * ((locals.var_q_x2sat_dn7 - locals.var_x2_wi0_dn7) * 0.3333333333333)) / assign17010_e16852), ((assign17010_e16851 * ((locals.var_q_x2sat_dn8 - locals.var_x2_wi0_dn8) * 0.3333333333333)) / assign17010_e16852), ((assign17010_e16851 * ((locals.var_q_x2sat_dn9 - locals.var_x2_wi0_dn9) * 0.3333333333333)) / assign17010_e16852),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign17010_e16855;
        locals.var_q_temp3_dn4 = assign17010_e16855_d_n4;
        locals.var_q_temp3_dn6 = assign17010_e16855_d_n6;
        locals.var_q_temp3_dn7 = assign17010_e16855_d_n7;
        locals.var_q_temp3_dn8 = assign17010_e16855_d_n8;
        locals.var_q_temp3_dn9 = assign17010_e16855_d_n9;

        let (assign17020_e16864, assign17020_e16864_d_n4, assign17020_e16864_d_n6, assign17020_e16864_d_n7, assign17020_e16864_d_n8, assign17020_e16864_d_n9,) = {
    if (locals.var_guard616 == 0.0) {
        let assign17020_e16860: f64 = (locals.var_q_x2sat - locals.var_x2_wi0);
        let assign17020_e16862: f64 = (assign17020_e16860 * 0.3333333333333);
        (assign17020_e16862, ((locals.var_q_x2sat_dn4 - locals.var_x2_wi0_dn4) * 0.3333333333333), ((locals.var_q_x2sat_dn6 - locals.var_x2_wi0_dn6) * 0.3333333333333), ((locals.var_q_x2sat_dn7 - locals.var_x2_wi0_dn7) * 0.3333333333333), ((locals.var_q_x2sat_dn8 - locals.var_x2_wi0_dn8) * 0.3333333333333), ((locals.var_q_x2sat_dn9 - locals.var_x2_wi0_dn9) * 0.3333333333333),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign17020_e16864;
        locals.var_q_temp3_dn4 = assign17020_e16864_d_n4;
        locals.var_q_temp3_dn6 = assign17020_e16864_d_n6;
        locals.var_q_temp3_dn7 = assign17020_e16864_d_n7;
        locals.var_q_temp3_dn8 = assign17020_e16864_d_n8;
        locals.var_q_temp3_dn9 = assign17020_e16864_d_n9;

        let assign17030_e16868: f64 = (3.0 * locals.var_q_temp3);
        let assign17030_e16869: f64 = (locals.var_q_x2sat - assign17030_e16868);
        locals.var_q_x2 = assign17030_e16869;
        locals.var_q_x2_dn4 = (locals.var_q_x2sat_dn4 - (3.0 * locals.var_q_temp3_dn4));
        locals.var_q_x2_dn6 = (locals.var_q_x2sat_dn6 - (3.0 * locals.var_q_temp3_dn6));
        locals.var_q_x2_dn7 = (locals.var_q_x2sat_dn7 - (3.0 * locals.var_q_temp3_dn7));
        locals.var_q_x2_dn8 = (locals.var_q_x2sat_dn8 - (3.0 * locals.var_q_temp3_dn8));
        locals.var_q_x2_dn9 = (locals.var_q_x2sat_dn9 - (3.0 * locals.var_q_temp3_dn9));

        let assign17040_e16872: f64 = (locals.var_k1 * locals.var_xg1x);
        let assign17040_e16874: f64 = (assign17040_e16872 + locals.var_q_x2);
        let assign17040_e16876: f64 = (assign17040_e16874 * locals.var_q_temp1);
        locals.var_q_x1_wi = assign17040_e16876;
        locals.var_q_x1_wi_dn4 = (((((locals.var_k1_dn4 * locals.var_xg1x) + (locals.var_k1 * locals.var_xg1x_dn4)) + locals.var_q_x2_dn4) * locals.var_q_temp1) + (assign17040_e16874 * locals.var_q_temp1_dn4));
        locals.var_q_x1_wi_dn6 = (((((locals.var_k1_dn6 * locals.var_xg1x) + (locals.var_k1 * locals.var_xg1x_dn6)) + locals.var_q_x2_dn6) * locals.var_q_temp1) + (assign17040_e16874 * locals.var_q_temp1_dn6));
        locals.var_q_x1_wi_dn7 = (((((locals.var_k1_dn7 * locals.var_xg1x) + (locals.var_k1 * locals.var_xg1x_dn7)) + locals.var_q_x2_dn7) * locals.var_q_temp1) + (assign17040_e16874 * locals.var_q_temp1_dn7));
        locals.var_q_x1_wi_dn8 = (((((locals.var_k1_dn8 * locals.var_xg1x) + (locals.var_k1 * locals.var_xg1x_dn8)) + locals.var_q_x2_dn8) * locals.var_q_temp1) + (assign17040_e16874 * locals.var_q_temp1_dn8));
        locals.var_q_x1_wi_dn9 = (((((locals.var_k1_dn9 * locals.var_xg1x) + (locals.var_k1 * locals.var_xg1x_dn9)) + locals.var_q_x2_dn9) * locals.var_q_temp1) + (assign17040_e16874 * locals.var_q_temp1_dn9));

        let assign17050_e16879: f64 = (locals.var_k2 * locals.var_xg2x);
        let assign17050_e16881: f64 = (assign17050_e16879 + locals.var_q_x1);
        let assign17050_e16883: f64 = (assign17050_e16881 * locals.var_q_temp2);
        locals.var_q_x2_wi = assign17050_e16883;
        locals.var_q_x2_wi_dn4 = (((((locals.var_k2_dn4 * locals.var_xg2x) + (locals.var_k2 * locals.var_xg2x_dn4)) + locals.var_q_x1_dn4) * locals.var_q_temp2) + (assign17050_e16881 * locals.var_q_temp2_dn4));
        locals.var_q_x2_wi_dn6 = (((((locals.var_k2_dn6 * locals.var_xg2x) + (locals.var_k2 * locals.var_xg2x_dn6)) + locals.var_q_x1_dn6) * locals.var_q_temp2) + (assign17050_e16881 * locals.var_q_temp2_dn6));
        locals.var_q_x2_wi_dn7 = (((((locals.var_k2_dn7 * locals.var_xg2x) + (locals.var_k2 * locals.var_xg2x_dn7)) + locals.var_q_x1_dn7) * locals.var_q_temp2) + (assign17050_e16881 * locals.var_q_temp2_dn7));
        locals.var_q_x2_wi_dn8 = (((((locals.var_k2_dn8 * locals.var_xg2x) + (locals.var_k2 * locals.var_xg2x_dn8)) + locals.var_q_x1_dn8) * locals.var_q_temp2) + (assign17050_e16881 * locals.var_q_temp2_dn8));
        locals.var_q_x2_wi_dn9 = (((((locals.var_k2_dn9 * locals.var_xg2x) + (locals.var_k2 * locals.var_xg2x_dn9)) + locals.var_q_x1_dn9) * locals.var_q_temp2) + (assign17050_e16881 * locals.var_q_temp2_dn9));

        let assign17060_e16886: f64 = (locals.var_q_x1sat - locals.var_q_x1_wi);
        let assign17060_e16888: f64 = (assign17060_e16886 * 0.3333333333333);
        let assign17060_e16890: f64 = if assign17060_e16888 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard617 = assign17060_e16890;

        let (assign17070_e16902, assign17070_e16902_d_n4, assign17070_e16902_d_n6, assign17070_e16902_d_n7, assign17070_e16902_d_n8, assign17070_e16902_d_n9,) = {
    if (locals.var_guard617 != 0.0) {
        let assign17070_e16895: f64 = (locals.var_q_x1sat - locals.var_q_x1_wi);
        let assign17070_e16897: f64 = (assign17070_e16895 * 0.3333333333333);
        let assign17070_e16898: f64 = (assign17070_e16897).exp();
        let assign17070_e16899: f64 = (1.0 + assign17070_e16898);
        let assign17070_e16900: f64 = (assign17070_e16899).ln();
        (assign17070_e16900, ((assign17070_e16898 * ((locals.var_q_x1sat_dn4 - locals.var_q_x1_wi_dn4) * 0.3333333333333)) / assign17070_e16899), ((assign17070_e16898 * ((locals.var_q_x1sat_dn6 - locals.var_q_x1_wi_dn6) * 0.3333333333333)) / assign17070_e16899), ((assign17070_e16898 * ((locals.var_q_x1sat_dn7 - locals.var_q_x1_wi_dn7) * 0.3333333333333)) / assign17070_e16899), ((assign17070_e16898 * ((locals.var_q_x1sat_dn8 - locals.var_q_x1_wi_dn8) * 0.3333333333333)) / assign17070_e16899), ((assign17070_e16898 * ((locals.var_q_x1sat_dn9 - locals.var_q_x1_wi_dn9) * 0.3333333333333)) / assign17070_e16899),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign17070_e16902;
        locals.var_q_temp3_dn4 = assign17070_e16902_d_n4;
        locals.var_q_temp3_dn6 = assign17070_e16902_d_n6;
        locals.var_q_temp3_dn7 = assign17070_e16902_d_n7;
        locals.var_q_temp3_dn8 = assign17070_e16902_d_n8;
        locals.var_q_temp3_dn9 = assign17070_e16902_d_n9;

        let (assign17080_e16911, assign17080_e16911_d_n4, assign17080_e16911_d_n6, assign17080_e16911_d_n7, assign17080_e16911_d_n8, assign17080_e16911_d_n9,) = {
    if (locals.var_guard617 == 0.0) {
        let assign17080_e16907: f64 = (locals.var_q_x1sat - locals.var_q_x1_wi);
        let assign17080_e16909: f64 = (assign17080_e16907 * 0.3333333333333);
        (assign17080_e16909, ((locals.var_q_x1sat_dn4 - locals.var_q_x1_wi_dn4) * 0.3333333333333), ((locals.var_q_x1sat_dn6 - locals.var_q_x1_wi_dn6) * 0.3333333333333), ((locals.var_q_x1sat_dn7 - locals.var_q_x1_wi_dn7) * 0.3333333333333), ((locals.var_q_x1sat_dn8 - locals.var_q_x1_wi_dn8) * 0.3333333333333), ((locals.var_q_x1sat_dn9 - locals.var_q_x1_wi_dn9) * 0.3333333333333),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign17080_e16911;
        locals.var_q_temp3_dn4 = assign17080_e16911_d_n4;
        locals.var_q_temp3_dn6 = assign17080_e16911_d_n6;
        locals.var_q_temp3_dn7 = assign17080_e16911_d_n7;
        locals.var_q_temp3_dn8 = assign17080_e16911_d_n8;
        locals.var_q_temp3_dn9 = assign17080_e16911_d_n9;

        let assign17090_e16915: f64 = (3.0 * locals.var_q_temp3);
        let assign17090_e16916: f64 = (locals.var_q_x1sat - assign17090_e16915);
        locals.var_q_x1 = assign17090_e16916;
        locals.var_q_x1_dn4 = (locals.var_q_x1sat_dn4 - (3.0 * locals.var_q_temp3_dn4));
        locals.var_q_x1_dn6 = (locals.var_q_x1sat_dn6 - (3.0 * locals.var_q_temp3_dn6));
        locals.var_q_x1_dn7 = (locals.var_q_x1sat_dn7 - (3.0 * locals.var_q_temp3_dn7));
        locals.var_q_x1_dn8 = (locals.var_q_x1sat_dn8 - (3.0 * locals.var_q_temp3_dn8));
        locals.var_q_x1_dn9 = (locals.var_q_x1sat_dn9 - (3.0 * locals.var_q_temp3_dn9));

        let assign17100_e16919: f64 = (locals.var_q_x2sat - locals.var_q_x2_wi);
        let assign17100_e16921: f64 = (assign17100_e16919 * 0.3333333333333);
        let assign17100_e16923: f64 = if assign17100_e16921 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard618 = assign17100_e16923;

        let (assign17110_e16935, assign17110_e16935_d_n4, assign17110_e16935_d_n6, assign17110_e16935_d_n7, assign17110_e16935_d_n8, assign17110_e16935_d_n9,) = {
    if (locals.var_guard618 != 0.0) {
        let assign17110_e16928: f64 = (locals.var_q_x2sat - locals.var_q_x2_wi);
        let assign17110_e16930: f64 = (assign17110_e16928 * 0.3333333333333);
        let assign17110_e16931: f64 = (assign17110_e16930).exp();
        let assign17110_e16932: f64 = (1.0 + assign17110_e16931);
        let assign17110_e16933: f64 = (assign17110_e16932).ln();
        (assign17110_e16933, ((assign17110_e16931 * ((locals.var_q_x2sat_dn4 - locals.var_q_x2_wi_dn4) * 0.3333333333333)) / assign17110_e16932), ((assign17110_e16931 * ((locals.var_q_x2sat_dn6 - locals.var_q_x2_wi_dn6) * 0.3333333333333)) / assign17110_e16932), ((assign17110_e16931 * ((locals.var_q_x2sat_dn7 - locals.var_q_x2_wi_dn7) * 0.3333333333333)) / assign17110_e16932), ((assign17110_e16931 * ((locals.var_q_x2sat_dn8 - locals.var_q_x2_wi_dn8) * 0.3333333333333)) / assign17110_e16932), ((assign17110_e16931 * ((locals.var_q_x2sat_dn9 - locals.var_q_x2_wi_dn9) * 0.3333333333333)) / assign17110_e16932),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign17110_e16935;
        locals.var_q_temp3_dn4 = assign17110_e16935_d_n4;
        locals.var_q_temp3_dn6 = assign17110_e16935_d_n6;
        locals.var_q_temp3_dn7 = assign17110_e16935_d_n7;
        locals.var_q_temp3_dn8 = assign17110_e16935_d_n8;
        locals.var_q_temp3_dn9 = assign17110_e16935_d_n9;

        let (assign17120_e16944, assign17120_e16944_d_n4, assign17120_e16944_d_n6, assign17120_e16944_d_n7, assign17120_e16944_d_n8, assign17120_e16944_d_n9,) = {
    if (locals.var_guard618 == 0.0) {
        let assign17120_e16940: f64 = (locals.var_q_x2sat - locals.var_q_x2_wi);
        let assign17120_e16942: f64 = (assign17120_e16940 * 0.3333333333333);
        (assign17120_e16942, ((locals.var_q_x2sat_dn4 - locals.var_q_x2_wi_dn4) * 0.3333333333333), ((locals.var_q_x2sat_dn6 - locals.var_q_x2_wi_dn6) * 0.3333333333333), ((locals.var_q_x2sat_dn7 - locals.var_q_x2_wi_dn7) * 0.3333333333333), ((locals.var_q_x2sat_dn8 - locals.var_q_x2_wi_dn8) * 0.3333333333333), ((locals.var_q_x2sat_dn9 - locals.var_q_x2_wi_dn9) * 0.3333333333333),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign17120_e16944;
        locals.var_q_temp3_dn4 = assign17120_e16944_d_n4;
        locals.var_q_temp3_dn6 = assign17120_e16944_d_n6;
        locals.var_q_temp3_dn7 = assign17120_e16944_d_n7;
        locals.var_q_temp3_dn8 = assign17120_e16944_d_n8;
        locals.var_q_temp3_dn9 = assign17120_e16944_d_n9;

        let assign17130_e16948: f64 = (3.0 * locals.var_q_temp3);
        let assign17130_e16949: f64 = (locals.var_q_x2sat - assign17130_e16948);
        locals.var_q_x2 = assign17130_e16949;
        locals.var_q_x2_dn4 = (locals.var_q_x2sat_dn4 - (3.0 * locals.var_q_temp3_dn4));
        locals.var_q_x2_dn6 = (locals.var_q_x2sat_dn6 - (3.0 * locals.var_q_temp3_dn6));
        locals.var_q_x2_dn7 = (locals.var_q_x2sat_dn7 - (3.0 * locals.var_q_temp3_dn7));
        locals.var_q_x2_dn8 = (locals.var_q_x2sat_dn8 - (3.0 * locals.var_q_temp3_dn8));
        locals.var_q_x2_dn9 = (locals.var_q_x2sat_dn9 - (3.0 * locals.var_q_temp3_dn9));

        let assign17140_e16952: f64 = (locals.var_xg1x - locals.var_q_x1);
        locals.var_q1d = assign17140_e16952;
        locals.var_q1d_dn4 = (locals.var_xg1x_dn4 - locals.var_q_x1_dn4);
        locals.var_q1d_dn6 = (locals.var_xg1x_dn6 - locals.var_q_x1_dn6);
        locals.var_q1d_dn7 = (locals.var_xg1x_dn7 - locals.var_q_x1_dn7);
        locals.var_q1d_dn8 = (locals.var_xg1x_dn8 - locals.var_q_x1_dn8);
        locals.var_q1d_dn9 = (locals.var_xg1x_dn9 - locals.var_q_x1_dn9);

        let assign17150_e16955: f64 = (locals.var_xg2x - locals.var_q_x2);
        locals.var_q2d = assign17150_e16955;
        locals.var_q2d_dn4 = (locals.var_xg2x_dn4 - locals.var_q_x2_dn4);
        locals.var_q2d_dn6 = (locals.var_xg2x_dn6 - locals.var_q_x2_dn6);
        locals.var_q2d_dn7 = (locals.var_xg2x_dn7 - locals.var_q_x2_dn7);
        locals.var_q2d_dn8 = (locals.var_xg2x_dn8 - locals.var_q_x2_dn8);
        locals.var_q2d_dn9 = (locals.var_xg2x_dn9 - locals.var_q_x2_dn9);

        locals.var_q_rac_qsq = 0.0;
        locals.var_q_rac_qsq_dn4 = 0.0;
        locals.var_q_rac_qsq_dn6 = 0.0;
        locals.var_q_rac_qsq_dn7 = 0.0;
        locals.var_q_rac_qsq_dn8 = 0.0;
        locals.var_q_rac_qsq_dn9 = 0.0;

        locals.var_q_invexpq = 0.0;
        locals.var_q_invexpq_dn4 = 0.0;
        locals.var_q_invexpq_dn6 = 0.0;
        locals.var_q_invexpq_dn7 = 0.0;
        locals.var_q_invexpq_dn8 = 0.0;
        locals.var_q_invexpq_dn9 = 0.0;

        let assign17180_e16960: f64 = (locals.var_k1 * locals.var_q1d);
        locals.var_q_k1q1 = assign17180_e16960;
        locals.var_q_k1q1_dn4 = ((locals.var_k1_dn4 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn4));
        locals.var_q_k1q1_dn6 = ((locals.var_k1_dn6 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn6));
        locals.var_q_k1q1_dn7 = ((locals.var_k1_dn7 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn7));
        locals.var_q_k1q1_dn8 = ((locals.var_k1_dn8 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn8));
        locals.var_q_k1q1_dn9 = ((locals.var_k1_dn9 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn9));

        let assign17190_e16963: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign17190_e16965: f64 = (assign17190_e16963 - locals.var_xdeff);
        let assign17190_e16967: f64 = if assign17190_e16965 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard619 = assign17190_e16967;

        let (assign17200_e16976, assign17200_e16976_d_n4, assign17200_e16976_d_n6, assign17200_e16976_d_n7, assign17200_e16976_d_n8, assign17200_e16976_d_n9,) = {
    if (locals.var_guard619 != 0.0) {
        let assign17200_e16971: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign17200_e16973: f64 = (assign17200_e16971 - locals.var_xdeff);
        let assign17200_e16974: f64 = (assign17200_e16973).exp();
        (assign17200_e16974, (assign17200_e16974 * ((locals.var_xg1x_dn4 - locals.var_q1d_dn4) - locals.var_xdeff_dn4)), (assign17200_e16974 * ((locals.var_xg1x_dn6 - locals.var_q1d_dn6) - locals.var_xdeff_dn6)), (assign17200_e16974 * ((locals.var_xg1x_dn7 - locals.var_q1d_dn7) - locals.var_xdeff_dn7)), (assign17200_e16974 * ((locals.var_xg1x_dn8 - locals.var_q1d_dn8) - locals.var_xdeff_dn8)), (assign17200_e16974 * ((locals.var_xg1x_dn9 - locals.var_q1d_dn9) - locals.var_xdeff_dn9)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign17200_e16976;
        locals.var_q_temp1_dn4 = assign17200_e16976_d_n4;
        locals.var_q_temp1_dn6 = assign17200_e16976_d_n6;
        locals.var_q_temp1_dn7 = assign17200_e16976_d_n7;
        locals.var_q_temp1_dn8 = assign17200_e16976_d_n8;
        locals.var_q_temp1_dn9 = assign17200_e16976_d_n9;

        let (assign17210_e17015, assign17210_e17015_d_n4, assign17210_e17015_d_n6, assign17210_e17015_d_n7, assign17210_e17015_d_n8, assign17210_e17015_d_n9,) = {
    if (locals.var_guard619 == 0.0) {
        let assign17210_e16983: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign17210_e16985: f64 = (assign17210_e16983 - locals.var_xdeff);
        let assign17210_e16987: f64 = (assign17210_e16985 - 80.0);
        let assign17210_e16992: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign17210_e16994: f64 = (assign17210_e16992 - locals.var_xdeff);
        let assign17210_e16996: f64 = (assign17210_e16994 - 80.0);
        let assign17210_e16997: f64 = (0.5 * assign17210_e16996);
        let assign17210_e17001: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign17210_e17003: f64 = (assign17210_e17001 - locals.var_xdeff);
        let assign17210_e17005: f64 = (assign17210_e17003 - 80.0);
        let assign17210_e17007: f64 = (assign17210_e17005 * 0.3333333333333);
        let assign17210_e17008: f64 = (1.0 + assign17210_e17007);
        let assign17210_e17009: f64 = (assign17210_e16997 * assign17210_e17008);
        let assign17210_e17010: f64 = (1.0 + assign17210_e17009);
        let assign17210_e17011: f64 = (assign17210_e16987 * assign17210_e17010);
        let assign17210_e17012: f64 = (1.0 + assign17210_e17011);
        let assign17210_e17013: f64 = (5.54062e34 * assign17210_e17012);
        (assign17210_e17013, (5.54062e34 * ((((locals.var_xg1x_dn4 - locals.var_q1d_dn4) - locals.var_xdeff_dn4) * assign17210_e17010) + (assign17210_e16987 * (((0.5 * ((locals.var_xg1x_dn4 - locals.var_q1d_dn4) - locals.var_xdeff_dn4)) * assign17210_e17008) + (assign17210_e16997 * (((locals.var_xg1x_dn4 - locals.var_q1d_dn4) - locals.var_xdeff_dn4) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x_dn6 - locals.var_q1d_dn6) - locals.var_xdeff_dn6) * assign17210_e17010) + (assign17210_e16987 * (((0.5 * ((locals.var_xg1x_dn6 - locals.var_q1d_dn6) - locals.var_xdeff_dn6)) * assign17210_e17008) + (assign17210_e16997 * (((locals.var_xg1x_dn6 - locals.var_q1d_dn6) - locals.var_xdeff_dn6) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x_dn7 - locals.var_q1d_dn7) - locals.var_xdeff_dn7) * assign17210_e17010) + (assign17210_e16987 * (((0.5 * ((locals.var_xg1x_dn7 - locals.var_q1d_dn7) - locals.var_xdeff_dn7)) * assign17210_e17008) + (assign17210_e16997 * (((locals.var_xg1x_dn7 - locals.var_q1d_dn7) - locals.var_xdeff_dn7) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x_dn8 - locals.var_q1d_dn8) - locals.var_xdeff_dn8) * assign17210_e17010) + (assign17210_e16987 * (((0.5 * ((locals.var_xg1x_dn8 - locals.var_q1d_dn8) - locals.var_xdeff_dn8)) * assign17210_e17008) + (assign17210_e16997 * (((locals.var_xg1x_dn8 - locals.var_q1d_dn8) - locals.var_xdeff_dn8) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x_dn9 - locals.var_q1d_dn9) - locals.var_xdeff_dn9) * assign17210_e17010) + (assign17210_e16987 * (((0.5 * ((locals.var_xg1x_dn9 - locals.var_q1d_dn9) - locals.var_xdeff_dn9)) * assign17210_e17008) + (assign17210_e16997 * (((locals.var_xg1x_dn9 - locals.var_q1d_dn9) - locals.var_xdeff_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign17210_e17015;
        locals.var_q_temp1_dn4 = assign17210_e17015_d_n4;
        locals.var_q_temp1_dn6 = assign17210_e17015_d_n6;
        locals.var_q_temp1_dn7 = assign17210_e17015_d_n7;
        locals.var_q_temp1_dn8 = assign17210_e17015_d_n8;
        locals.var_q_temp1_dn9 = assign17210_e17015_d_n9;

        let assign17220_e17018: f64 = (locals.var_a0 * locals.var_q_temp1);
        locals.var_q_aexp = assign17220_e17018;
        locals.var_q_aexp_dn4 = ((locals.var_a0_dn4 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn4));
        locals.var_q_aexp_dn6 = ((locals.var_a0_dn6 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn6));
        locals.var_q_aexp_dn7 = ((locals.var_a0_dn7 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn7));
        locals.var_q_aexp_dn8 = ((locals.var_a0_dn8 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn8));
        locals.var_q_aexp_dn9 = ((locals.var_a0_dn9 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn9));

        let assign17230_e17021: f64 = (locals.var_q_k1q1 * locals.var_q_k1q1);
        let assign17230_e17023: f64 = (assign17230_e17021 - locals.var_q_aexp);
        locals.var_q_qsq = assign17230_e17023;
        locals.var_q_qsq_dn4 = (((locals.var_q_k1q1_dn4 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn4)) - locals.var_q_aexp_dn4);
        locals.var_q_qsq_dn6 = (((locals.var_q_k1q1_dn6 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn6)) - locals.var_q_aexp_dn6);
        locals.var_q_qsq_dn7 = (((locals.var_q_k1q1_dn7 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn7)) - locals.var_q_aexp_dn7);
        locals.var_q_qsq_dn8 = (((locals.var_q_k1q1_dn8 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn8)) - locals.var_q_aexp_dn8);
        locals.var_q_qsq_dn9 = (((locals.var_q_k1q1_dn9 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn9)) - locals.var_q_aexp_dn9);

        let assign17240_e17026: f64 = (2.0 * locals.var_k1);
        let assign17240_e17028: f64 = (assign17240_e17026 * locals.var_q_k1q1);
        let assign17240_e17030: f64 = (assign17240_e17028 + locals.var_q_aexp);
        locals.var_q_d1_qsq = assign17240_e17030;
        locals.var_q_d1_qsq_dn4 = ((((2.0 * locals.var_k1_dn4) * locals.var_q_k1q1) + (assign17240_e17026 * locals.var_q_k1q1_dn4)) + locals.var_q_aexp_dn4);
        locals.var_q_d1_qsq_dn6 = ((((2.0 * locals.var_k1_dn6) * locals.var_q_k1q1) + (assign17240_e17026 * locals.var_q_k1q1_dn6)) + locals.var_q_aexp_dn6);
        locals.var_q_d1_qsq_dn7 = ((((2.0 * locals.var_k1_dn7) * locals.var_q_k1q1) + (assign17240_e17026 * locals.var_q_k1q1_dn7)) + locals.var_q_aexp_dn7);
        locals.var_q_d1_qsq_dn8 = ((((2.0 * locals.var_k1_dn8) * locals.var_q_k1q1) + (assign17240_e17026 * locals.var_q_k1q1_dn8)) + locals.var_q_aexp_dn8);
        locals.var_q_d1_qsq_dn9 = ((((2.0 * locals.var_k1_dn9) * locals.var_q_k1q1) + (assign17240_e17026 * locals.var_q_k1q1_dn9)) + locals.var_q_aexp_dn9);

        let assign17250_e17033: f64 = (2.0 * locals.var_k1);
        let assign17250_e17035: f64 = (assign17250_e17033 * locals.var_k1);
        let assign17250_e17037: f64 = (assign17250_e17035 - locals.var_q_aexp);
        locals.var_q_d2_qsq = assign17250_e17037;
        locals.var_q_d2_qsq_dn4 = ((((2.0 * locals.var_k1_dn4) * locals.var_k1) + (assign17250_e17033 * locals.var_k1_dn4)) - locals.var_q_aexp_dn4);
        locals.var_q_d2_qsq_dn6 = ((((2.0 * locals.var_k1_dn6) * locals.var_k1) + (assign17250_e17033 * locals.var_k1_dn6)) - locals.var_q_aexp_dn6);
        locals.var_q_d2_qsq_dn7 = ((((2.0 * locals.var_k1_dn7) * locals.var_k1) + (assign17250_e17033 * locals.var_k1_dn7)) - locals.var_q_aexp_dn7);
        locals.var_q_d2_qsq_dn8 = ((((2.0 * locals.var_k1_dn8) * locals.var_k1) + (assign17250_e17033 * locals.var_k1_dn8)) - locals.var_q_aexp_dn8);
        locals.var_q_d2_qsq_dn9 = ((((2.0 * locals.var_k1_dn9) * locals.var_k1) + (assign17250_e17033 * locals.var_k1_dn9)) - locals.var_q_aexp_dn9);

        let assign17260_e17040: f64 = (-0.005);
        let assign17260_e17041: f64 = if locals.var_q_qsq < assign17260_e17040 { 1.0 } else { 0.0 };
        locals.var_guard620 = assign17260_e17041;

        let (assign17270_e17047, assign17270_e17047_d_n4, assign17270_e17047_d_n6, assign17270_e17047_d_n7, assign17270_e17047_d_n8, assign17270_e17047_d_n9,) = {
    if (locals.var_guard620 != 0.0) {
        let assign17270_e17044: f64 = (locals.var_q_qsq).abs();
        let assign17270_e17045: f64 = (assign17270_e17044).sqrt();
        (assign17270_e17045, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign17270_e17045)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign17270_e17045)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign17270_e17045)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign17270_e17045)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign17270_e17045)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign17270_e17047;
        locals.var_q_rac_qsq_dn4 = assign17270_e17047_d_n4;
        locals.var_q_rac_qsq_dn6 = assign17270_e17047_d_n6;
        locals.var_q_rac_qsq_dn7 = assign17270_e17047_d_n7;
        locals.var_q_rac_qsq_dn8 = assign17270_e17047_d_n8;
        locals.var_q_rac_qsq_dn9 = assign17270_e17047_d_n9;

        let (assign17280_e17056, assign17280_e17056_d_n4, assign17280_e17056_d_n6, assign17280_e17056_d_n7, assign17280_e17056_d_n8, assign17280_e17056_d_n9,) = {
    if (locals.var_guard620 != 0.0) {
        let assign17280_e17052: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign17280_e17053: f64 = (assign17280_e17052).tan();
        let assign17280_e17054: f64 = (locals.var_q_rac_qsq / assign17280_e17053);
        (assign17280_e17054, (((locals.var_q_rac_qsq_dn4 * assign17280_e17053) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn4) / ((assign17280_e17052).cos() * (assign17280_e17052).cos())))) / (assign17280_e17053 * assign17280_e17053)), (((locals.var_q_rac_qsq_dn6 * assign17280_e17053) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn6) / ((assign17280_e17052).cos() * (assign17280_e17052).cos())))) / (assign17280_e17053 * assign17280_e17053)), (((locals.var_q_rac_qsq_dn7 * assign17280_e17053) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn7) / ((assign17280_e17052).cos() * (assign17280_e17052).cos())))) / (assign17280_e17053 * assign17280_e17053)), (((locals.var_q_rac_qsq_dn8 * assign17280_e17053) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn8) / ((assign17280_e17052).cos() * (assign17280_e17052).cos())))) / (assign17280_e17053 * assign17280_e17053)), (((locals.var_q_rac_qsq_dn9 * assign17280_e17053) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn9) / ((assign17280_e17052).cos() * (assign17280_e17052).cos())))) / (assign17280_e17053 * assign17280_e17053)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign17280_e17056;
        locals.var_q_qcoth_dn4 = assign17280_e17056_d_n4;
        locals.var_q_qcoth_dn6 = assign17280_e17056_d_n6;
        locals.var_q_qcoth_dn7 = assign17280_e17056_d_n7;
        locals.var_q_qcoth_dn8 = assign17280_e17056_d_n8;
        locals.var_q_qcoth_dn9 = assign17280_e17056_d_n9;

        let (assign17290_e17064, assign17290_e17064_d_n4, assign17290_e17064_d_n6, assign17290_e17064_d_n7, assign17290_e17064_d_n8, assign17290_e17064_d_n9,) = {
    if (locals.var_guard620 != 0.0) {
        let assign17290_e17060: f64 = (0.25 * locals.var_q_d1_qsq);
        let assign17290_e17062: f64 = (assign17290_e17060 / locals.var_q_qsq);
        (assign17290_e17062, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign17290_e17060 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign17290_e17060 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign17290_e17060 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign17290_e17060 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign17290_e17060 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign17290_e17064;
        locals.var_q_temp1_dn4 = assign17290_e17064_d_n4;
        locals.var_q_temp1_dn6 = assign17290_e17064_d_n6;
        locals.var_q_temp1_dn7 = assign17290_e17064_d_n7;
        locals.var_q_temp1_dn8 = assign17290_e17064_d_n8;
        locals.var_q_temp1_dn9 = assign17290_e17064_d_n9;

        let (assign17300_e17076, assign17300_e17076_d_n4, assign17300_e17076_d_n6, assign17300_e17076_d_n7, assign17300_e17076_d_n8, assign17300_e17076_d_n9,) = {
    if (locals.var_guard620 != 0.0) {
        let assign17300_e17070: f64 = (2.0 - locals.var_q_qcoth);
        let assign17300_e17071: f64 = (locals.var_q_qcoth * assign17300_e17070);
        let assign17300_e17072: f64 = (locals.var_q_qsq + assign17300_e17071);
        let assign17300_e17074: f64 = (assign17300_e17072 * locals.var_q_temp1);
        (assign17300_e17074, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign17300_e17070) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign17300_e17072 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign17300_e17070) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign17300_e17072 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign17300_e17070) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign17300_e17072 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign17300_e17070) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign17300_e17072 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign17300_e17070) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign17300_e17072 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign17300_e17076;
        locals.var_q_d1_qcoth_dn4 = assign17300_e17076_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign17300_e17076_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign17300_e17076_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign17300_e17076_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign17300_e17076_d_n9;

        let (assign17310_e17096, assign17310_e17096_d_n4, assign17310_e17096_d_n6, assign17310_e17096_d_n7, assign17310_e17096_d_n8, assign17310_e17096_d_n9,) = {
    if (locals.var_guard620 != 0.0) {
        let assign17310_e17081: f64 = (2.0 * locals.var_q_d1_qcoth);
        let assign17310_e17084: f64 = (1.0 + locals.var_q_qcoth);
        let assign17310_e17085: f64 = (assign17310_e17081 * assign17310_e17084);
        let assign17310_e17086: f64 = (locals.var_q_d1_qsq - assign17310_e17085);
        let assign17310_e17088: f64 = (assign17310_e17086 * locals.var_q_temp1);
        let assign17310_e17091: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
        let assign17310_e17093: f64 = (assign17310_e17091 / locals.var_q_d1_qsq);
        let assign17310_e17094: f64 = (assign17310_e17088 + assign17310_e17093);
        (assign17310_e17094, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign17310_e17084) + (assign17310_e17081 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign17310_e17086 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign17310_e17091 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign17310_e17084) + (assign17310_e17081 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign17310_e17086 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign17310_e17091 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign17310_e17084) + (assign17310_e17081 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign17310_e17086 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign17310_e17091 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign17310_e17084) + (assign17310_e17081 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign17310_e17086 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign17310_e17091 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign17310_e17084) + (assign17310_e17081 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign17310_e17086 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign17310_e17091 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign17310_e17096;
        locals.var_q_d2_qcoth_dn4 = assign17310_e17096_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign17310_e17096_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign17310_e17096_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign17310_e17096_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign17310_e17096_d_n9;

    }

    pub(super) fn stamp_transient_block_43(
        locals: &mut StampLocals,
    ) {
        let (assign17320_e17104, assign17320_e17104_d_n4, assign17320_e17104_d_n6, assign17320_e17104_d_n7, assign17320_e17104_d_n8, assign17320_e17104_d_n9,) = {
    if (locals.var_guard620 != 0.0) {
        let assign17320_e17101: f64 = (0.5 * locals.var_q_qcoth);
        let assign17320_e17102: f64 = (1.0 - assign17320_e17101);
        (assign17320_e17102, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign17320_e17104;
        locals.var_q_temp2_dn4 = assign17320_e17104_d_n4;
        locals.var_q_temp2_dn6 = assign17320_e17104_d_n6;
        locals.var_q_temp2_dn7 = assign17320_e17104_d_n7;
        locals.var_q_temp2_dn8 = assign17320_e17104_d_n8;
        locals.var_q_temp2_dn9 = assign17320_e17104_d_n9;

        let (assign17330_e17112, assign17330_e17112_d_n4, assign17330_e17112_d_n6, assign17330_e17112_d_n7, assign17330_e17112_d_n8, assign17330_e17112_d_n9,) = {
    if (locals.var_guard620 != 0.0) {
        let assign17330_e17108: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
        let assign17330_e17110: f64 = (assign17330_e17108 * locals.var_q_temp2);
        (assign17330_e17110, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign17330_e17108 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign17330_e17108 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign17330_e17108 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign17330_e17108 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign17330_e17108 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign17330_e17112;
        locals.var_q_d1_ln_dn4 = assign17330_e17112_d_n4;
        locals.var_q_d1_ln_dn6 = assign17330_e17112_d_n6;
        locals.var_q_d1_ln_dn7 = assign17330_e17112_d_n7;
        locals.var_q_d1_ln_dn8 = assign17330_e17112_d_n8;
        locals.var_q_d1_ln_dn9 = assign17330_e17112_d_n9;

        let (assign17340_e17128, assign17340_e17128_d_n4, assign17340_e17128_d_n6, assign17340_e17128_d_n7, assign17340_e17128_d_n8, assign17340_e17128_d_n9,) = {
    if (locals.var_guard620 != 0.0) {
        let assign17340_e17116: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
        let assign17340_e17121: f64 = (0.5 * locals.var_q_d1_qcoth);
        let assign17340_e17122: f64 = (locals.var_q_d1_ln + assign17340_e17121);
        let assign17340_e17123: f64 = (locals.var_q_d1_qsq * assign17340_e17122);
        let assign17340_e17124: f64 = (assign17340_e17116 - assign17340_e17123);
        let assign17340_e17126: f64 = (assign17340_e17124 / locals.var_q_qsq);
        (assign17340_e17126, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign17340_e17122) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign17340_e17124 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign17340_e17122) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign17340_e17124 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign17340_e17122) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign17340_e17124 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign17340_e17122) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign17340_e17124 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign17340_e17122) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign17340_e17124 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign17340_e17128;
        locals.var_q_d2_ln_dn4 = assign17340_e17128_d_n4;
        locals.var_q_d2_ln_dn6 = assign17340_e17128_d_n6;
        locals.var_q_d2_ln_dn7 = assign17340_e17128_d_n7;
        locals.var_q_d2_ln_dn8 = assign17340_e17128_d_n8;
        locals.var_q_d2_ln_dn9 = assign17340_e17128_d_n9;

        let assign17350_e17131: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard621 = assign17350_e17131;

        let (assign17360_e17140, assign17360_e17140_d_n4, assign17360_e17140_d_n6, assign17360_e17140_d_n7, assign17360_e17140_d_n8, assign17360_e17140_d_n9,) = {
    if ((locals.var_guard620 == 0.0) && (locals.var_guard621 != 0.0)) {
        let assign17360_e17137: f64 = (locals.var_q_qsq).abs();
        let assign17360_e17138: f64 = (assign17360_e17137).sqrt();
        (assign17360_e17138, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign17360_e17138)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign17360_e17138)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign17360_e17138)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign17360_e17138)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign17360_e17138)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign17360_e17140;
        locals.var_q_rac_qsq_dn4 = assign17360_e17140_d_n4;
        locals.var_q_rac_qsq_dn6 = assign17360_e17140_d_n6;
        locals.var_q_rac_qsq_dn7 = assign17360_e17140_d_n7;
        locals.var_q_rac_qsq_dn8 = assign17360_e17140_d_n8;
        locals.var_q_rac_qsq_dn9 = assign17360_e17140_d_n9;

        let (assign17370_e17149, assign17370_e17149_d_n4, assign17370_e17149_d_n6, assign17370_e17149_d_n7, assign17370_e17149_d_n8, assign17370_e17149_d_n9,) = {
    if ((locals.var_guard620 == 0.0) && (locals.var_guard621 != 0.0)) {
        let assign17370_e17146: f64 = (-locals.var_q_rac_qsq);
        let assign17370_e17147: f64 = (assign17370_e17146).exp();
        (assign17370_e17147, (assign17370_e17147 * (-locals.var_q_rac_qsq_dn4)), (assign17370_e17147 * (-locals.var_q_rac_qsq_dn6)), (assign17370_e17147 * (-locals.var_q_rac_qsq_dn7)), (assign17370_e17147 * (-locals.var_q_rac_qsq_dn8)), (assign17370_e17147 * (-locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_invexpq, locals.var_q_invexpq_dn4, locals.var_q_invexpq_dn6, locals.var_q_invexpq_dn7, locals.var_q_invexpq_dn8, locals.var_q_invexpq_dn9,)
    }
};
        locals.var_q_invexpq = assign17370_e17149;
        locals.var_q_invexpq_dn4 = assign17370_e17149_d_n4;
        locals.var_q_invexpq_dn6 = assign17370_e17149_d_n6;
        locals.var_q_invexpq_dn7 = assign17370_e17149_d_n7;
        locals.var_q_invexpq_dn8 = assign17370_e17149_d_n8;
        locals.var_q_invexpq_dn9 = assign17370_e17149_d_n9;

        let (assign17380_e17164, assign17380_e17164_d_n4, assign17380_e17164_d_n6, assign17380_e17164_d_n7, assign17380_e17164_d_n8, assign17380_e17164_d_n9,) = {
    if ((locals.var_guard620 == 0.0) && (locals.var_guard621 != 0.0)) {
        let assign17380_e17157: f64 = (1.0 + locals.var_q_invexpq);
        let assign17380_e17158: f64 = (locals.var_q_rac_qsq * assign17380_e17157);
        let assign17380_e17161: f64 = (1.0 - locals.var_q_invexpq);
        let assign17380_e17162: f64 = (assign17380_e17158 / assign17380_e17161);
        (assign17380_e17162, (((((locals.var_q_rac_qsq_dn4 * assign17380_e17157) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn4)) * assign17380_e17161) - (assign17380_e17158 * (-locals.var_q_invexpq_dn4))) / (assign17380_e17161 * assign17380_e17161)), (((((locals.var_q_rac_qsq_dn6 * assign17380_e17157) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn6)) * assign17380_e17161) - (assign17380_e17158 * (-locals.var_q_invexpq_dn6))) / (assign17380_e17161 * assign17380_e17161)), (((((locals.var_q_rac_qsq_dn7 * assign17380_e17157) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn7)) * assign17380_e17161) - (assign17380_e17158 * (-locals.var_q_invexpq_dn7))) / (assign17380_e17161 * assign17380_e17161)), (((((locals.var_q_rac_qsq_dn8 * assign17380_e17157) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn8)) * assign17380_e17161) - (assign17380_e17158 * (-locals.var_q_invexpq_dn8))) / (assign17380_e17161 * assign17380_e17161)), (((((locals.var_q_rac_qsq_dn9 * assign17380_e17157) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn9)) * assign17380_e17161) - (assign17380_e17158 * (-locals.var_q_invexpq_dn9))) / (assign17380_e17161 * assign17380_e17161)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign17380_e17164;
        locals.var_q_qcoth_dn4 = assign17380_e17164_d_n4;
        locals.var_q_qcoth_dn6 = assign17380_e17164_d_n6;
        locals.var_q_qcoth_dn7 = assign17380_e17164_d_n7;
        locals.var_q_qcoth_dn8 = assign17380_e17164_d_n8;
        locals.var_q_qcoth_dn9 = assign17380_e17164_d_n9;

        let (assign17390_e17175, assign17390_e17175_d_n4, assign17390_e17175_d_n6, assign17390_e17175_d_n7, assign17390_e17175_d_n8, assign17390_e17175_d_n9,) = {
    if ((locals.var_guard620 == 0.0) && (locals.var_guard621 != 0.0)) {
        let assign17390_e17171: f64 = (0.25 * locals.var_q_d1_qsq);
        let assign17390_e17173: f64 = (assign17390_e17171 / locals.var_q_qsq);
        (assign17390_e17173, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign17390_e17171 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign17390_e17171 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign17390_e17171 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign17390_e17171 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign17390_e17171 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign17390_e17175;
        locals.var_q_temp1_dn4 = assign17390_e17175_d_n4;
        locals.var_q_temp1_dn6 = assign17390_e17175_d_n6;
        locals.var_q_temp1_dn7 = assign17390_e17175_d_n7;
        locals.var_q_temp1_dn8 = assign17390_e17175_d_n8;
        locals.var_q_temp1_dn9 = assign17390_e17175_d_n9;

        let (assign17400_e17190, assign17400_e17190_d_n4, assign17400_e17190_d_n6, assign17400_e17190_d_n7, assign17400_e17190_d_n8, assign17400_e17190_d_n9,) = {
    if ((locals.var_guard620 == 0.0) && (locals.var_guard621 != 0.0)) {
        let assign17400_e17184: f64 = (2.0 - locals.var_q_qcoth);
        let assign17400_e17185: f64 = (locals.var_q_qcoth * assign17400_e17184);
        let assign17400_e17186: f64 = (locals.var_q_qsq + assign17400_e17185);
        let assign17400_e17188: f64 = (assign17400_e17186 * locals.var_q_temp1);
        (assign17400_e17188, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign17400_e17184) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign17400_e17186 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign17400_e17184) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign17400_e17186 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign17400_e17184) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign17400_e17186 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign17400_e17184) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign17400_e17186 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign17400_e17184) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign17400_e17186 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign17400_e17190;
        locals.var_q_d1_qcoth_dn4 = assign17400_e17190_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign17400_e17190_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign17400_e17190_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign17400_e17190_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign17400_e17190_d_n9;

        let (assign17410_e17213, assign17410_e17213_d_n4, assign17410_e17213_d_n6, assign17410_e17213_d_n7, assign17410_e17213_d_n8, assign17410_e17213_d_n9,) = {
    if ((locals.var_guard620 == 0.0) && (locals.var_guard621 != 0.0)) {
        let assign17410_e17198: f64 = (2.0 * locals.var_q_d1_qcoth);
        let assign17410_e17201: f64 = (1.0 + locals.var_q_qcoth);
        let assign17410_e17202: f64 = (assign17410_e17198 * assign17410_e17201);
        let assign17410_e17203: f64 = (locals.var_q_d1_qsq - assign17410_e17202);
        let assign17410_e17205: f64 = (assign17410_e17203 * locals.var_q_temp1);
        let assign17410_e17208: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
        let assign17410_e17210: f64 = (assign17410_e17208 / locals.var_q_d1_qsq);
        let assign17410_e17211: f64 = (assign17410_e17205 + assign17410_e17210);
        (assign17410_e17211, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign17410_e17201) + (assign17410_e17198 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign17410_e17203 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign17410_e17208 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign17410_e17201) + (assign17410_e17198 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign17410_e17203 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign17410_e17208 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign17410_e17201) + (assign17410_e17198 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign17410_e17203 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign17410_e17208 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign17410_e17201) + (assign17410_e17198 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign17410_e17203 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign17410_e17208 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign17410_e17201) + (assign17410_e17198 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign17410_e17203 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign17410_e17208 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign17410_e17213;
        locals.var_q_d2_qcoth_dn4 = assign17410_e17213_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign17410_e17213_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign17410_e17213_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign17410_e17213_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign17410_e17213_d_n9;

        let (assign17420_e17224, assign17420_e17224_d_n4, assign17420_e17224_d_n6, assign17420_e17224_d_n7, assign17420_e17224_d_n8, assign17420_e17224_d_n9,) = {
    if ((locals.var_guard620 == 0.0) && (locals.var_guard621 != 0.0)) {
        let assign17420_e17221: f64 = (0.5 * locals.var_q_qcoth);
        let assign17420_e17222: f64 = (1.0 - assign17420_e17221);
        (assign17420_e17222, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign17420_e17224;
        locals.var_q_temp2_dn4 = assign17420_e17224_d_n4;
        locals.var_q_temp2_dn6 = assign17420_e17224_d_n6;
        locals.var_q_temp2_dn7 = assign17420_e17224_d_n7;
        locals.var_q_temp2_dn8 = assign17420_e17224_d_n8;
        locals.var_q_temp2_dn9 = assign17420_e17224_d_n9;

        let (assign17430_e17235, assign17430_e17235_d_n4, assign17430_e17235_d_n6, assign17430_e17235_d_n7, assign17430_e17235_d_n8, assign17430_e17235_d_n9,) = {
    if ((locals.var_guard620 == 0.0) && (locals.var_guard621 != 0.0)) {
        let assign17430_e17231: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
        let assign17430_e17233: f64 = (assign17430_e17231 * locals.var_q_temp2);
        (assign17430_e17233, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign17430_e17231 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign17430_e17231 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign17430_e17231 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign17430_e17231 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign17430_e17231 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign17430_e17235;
        locals.var_q_d1_ln_dn4 = assign17430_e17235_d_n4;
        locals.var_q_d1_ln_dn6 = assign17430_e17235_d_n6;
        locals.var_q_d1_ln_dn7 = assign17430_e17235_d_n7;
        locals.var_q_d1_ln_dn8 = assign17430_e17235_d_n8;
        locals.var_q_d1_ln_dn9 = assign17430_e17235_d_n9;

        let (assign17440_e17254, assign17440_e17254_d_n4, assign17440_e17254_d_n6, assign17440_e17254_d_n7, assign17440_e17254_d_n8, assign17440_e17254_d_n9,) = {
    if ((locals.var_guard620 == 0.0) && (locals.var_guard621 != 0.0)) {
        let assign17440_e17242: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
        let assign17440_e17247: f64 = (0.5 * locals.var_q_d1_qcoth);
        let assign17440_e17248: f64 = (locals.var_q_d1_ln + assign17440_e17247);
        let assign17440_e17249: f64 = (locals.var_q_d1_qsq * assign17440_e17248);
        let assign17440_e17250: f64 = (assign17440_e17242 - assign17440_e17249);
        let assign17440_e17252: f64 = (assign17440_e17250 / locals.var_q_qsq);
        (assign17440_e17252, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign17440_e17248) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign17440_e17250 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign17440_e17248) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign17440_e17250 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign17440_e17248) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign17440_e17250 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign17440_e17248) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign17440_e17250 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign17440_e17248) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign17440_e17250 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign17440_e17254;
        locals.var_q_d2_ln_dn4 = assign17440_e17254_d_n4;
        locals.var_q_d2_ln_dn6 = assign17440_e17254_d_n6;
        locals.var_q_d2_ln_dn7 = assign17440_e17254_d_n7;
        locals.var_q_d2_ln_dn8 = assign17440_e17254_d_n8;
        locals.var_q_d2_ln_dn9 = assign17440_e17254_d_n9;

        let (assign17450_e17280, assign17450_e17280_d_n4, assign17450_e17280_d_n6, assign17450_e17280_d_n7, assign17450_e17280_d_n8, assign17450_e17280_d_n9,) = {
    if ((locals.var_guard620 == 0.0) && (locals.var_guard621 == 0.0)) {
        let assign17450_e17264: f64 = (locals.var_q_qsq * 0.0166666666667);
        let assign17450_e17268: f64 = (locals.var_q_qsq * 0.0238095238095);
        let assign17450_e17272: f64 = (locals.var_q_qsq * 0.025);
        let assign17450_e17273: f64 = (1.0 - assign17450_e17272);
        let assign17450_e17274: f64 = (assign17450_e17268 * assign17450_e17273);
        let assign17450_e17275: f64 = (1.0 - assign17450_e17274);
        let assign17450_e17276: f64 = (assign17450_e17264 * assign17450_e17275);
        let assign17450_e17277: f64 = (1.0 - assign17450_e17276);
        let assign17450_e17278: f64 = (0.1666666666667 * assign17450_e17277);
        (assign17450_e17278, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0166666666667) * assign17450_e17275) + (assign17450_e17264 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign17450_e17273) + (assign17450_e17268 * (-(locals.var_q_qsq_dn4 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0166666666667) * assign17450_e17275) + (assign17450_e17264 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign17450_e17273) + (assign17450_e17268 * (-(locals.var_q_qsq_dn6 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0166666666667) * assign17450_e17275) + (assign17450_e17264 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign17450_e17273) + (assign17450_e17268 * (-(locals.var_q_qsq_dn7 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0166666666667) * assign17450_e17275) + (assign17450_e17264 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign17450_e17273) + (assign17450_e17268 * (-(locals.var_q_qsq_dn8 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0166666666667) * assign17450_e17275) + (assign17450_e17264 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign17450_e17273) + (assign17450_e17268 * (-(locals.var_q_qsq_dn9 * 0.025))))))))),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign17450_e17280;
        locals.var_q_temp3_dn4 = assign17450_e17280_d_n4;
        locals.var_q_temp3_dn6 = assign17450_e17280_d_n6;
        locals.var_q_temp3_dn7 = assign17450_e17280_d_n7;
        locals.var_q_temp3_dn8 = assign17450_e17280_d_n8;
        locals.var_q_temp3_dn9 = assign17450_e17280_d_n9;

        let (assign17460_e17292, assign17460_e17292_d_n4, assign17460_e17292_d_n6, assign17460_e17292_d_n7, assign17460_e17292_d_n8, assign17460_e17292_d_n9,) = {
    if ((locals.var_guard620 == 0.0) && (locals.var_guard621 == 0.0)) {
        let assign17460_e17289: f64 = (locals.var_q_qsq * locals.var_q_temp3);
        let assign17460_e17290: f64 = (2.0 + assign17460_e17289);
        (assign17460_e17290, ((locals.var_q_qsq_dn4 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn4)), ((locals.var_q_qsq_dn6 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn6)), ((locals.var_q_qsq_dn7 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn7)), ((locals.var_q_qsq_dn8 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn8)), ((locals.var_q_qsq_dn9 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn9)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign17460_e17292;
        locals.var_q_qcoth_dn4 = assign17460_e17292_d_n4;
        locals.var_q_qcoth_dn6 = assign17460_e17292_d_n6;
        locals.var_q_qcoth_dn7 = assign17460_e17292_d_n7;
        locals.var_q_qcoth_dn8 = assign17460_e17292_d_n8;
        locals.var_q_qcoth_dn9 = assign17460_e17292_d_n9;

        let (assign17470_e17318, assign17470_e17318_d_n4, assign17470_e17318_d_n6, assign17470_e17318_d_n7, assign17470_e17318_d_n8, assign17470_e17318_d_n9,) = {
    if ((locals.var_guard620 == 0.0) && (locals.var_guard621 == 0.0)) {
        let assign17470_e17302: f64 = (locals.var_q_qsq * 0.0333333333333);
        let assign17470_e17306: f64 = (locals.var_q_qsq * 0.0357142857143);
        let assign17470_e17310: f64 = (locals.var_q_qsq * 0.0333333333333);
        let assign17470_e17311: f64 = (1.0 - assign17470_e17310);
        let assign17470_e17312: f64 = (assign17470_e17306 * assign17470_e17311);
        let assign17470_e17313: f64 = (1.0 - assign17470_e17312);
        let assign17470_e17314: f64 = (assign17470_e17302 * assign17470_e17313);
        let assign17470_e17315: f64 = (1.0 - assign17470_e17314);
        let assign17470_e17316: f64 = (0.1666666666667 * assign17470_e17315);
        (assign17470_e17316, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0333333333333) * assign17470_e17313) + (assign17470_e17302 * (-(((locals.var_q_qsq_dn4 * 0.0357142857143) * assign17470_e17311) + (assign17470_e17306 * (-(locals.var_q_qsq_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0333333333333) * assign17470_e17313) + (assign17470_e17302 * (-(((locals.var_q_qsq_dn6 * 0.0357142857143) * assign17470_e17311) + (assign17470_e17306 * (-(locals.var_q_qsq_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0333333333333) * assign17470_e17313) + (assign17470_e17302 * (-(((locals.var_q_qsq_dn7 * 0.0357142857143) * assign17470_e17311) + (assign17470_e17306 * (-(locals.var_q_qsq_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0333333333333) * assign17470_e17313) + (assign17470_e17302 * (-(((locals.var_q_qsq_dn8 * 0.0357142857143) * assign17470_e17311) + (assign17470_e17306 * (-(locals.var_q_qsq_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0333333333333) * assign17470_e17313) + (assign17470_e17302 * (-(((locals.var_q_qsq_dn9 * 0.0357142857143) * assign17470_e17311) + (assign17470_e17306 * (-(locals.var_q_qsq_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign17470_e17318;
        locals.var_q_temp1_dn4 = assign17470_e17318_d_n4;
        locals.var_q_temp1_dn6 = assign17470_e17318_d_n6;
        locals.var_q_temp1_dn7 = assign17470_e17318_d_n7;
        locals.var_q_temp1_dn8 = assign17470_e17318_d_n8;
        locals.var_q_temp1_dn9 = assign17470_e17318_d_n9;

        let (assign17480_e17328, assign17480_e17328_d_n4, assign17480_e17328_d_n6, assign17480_e17328_d_n7, assign17480_e17328_d_n8, assign17480_e17328_d_n9,) = {
    if ((locals.var_guard620 == 0.0) && (locals.var_guard621 == 0.0)) {
        let assign17480_e17326: f64 = (locals.var_q_d1_qsq * locals.var_q_temp1);
        (assign17480_e17326, ((locals.var_q_d1_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn4)), ((locals.var_q_d1_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn6)), ((locals.var_q_d1_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn7)), ((locals.var_q_d1_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn8)), ((locals.var_q_d1_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign17480_e17328;
        locals.var_q_d1_qcoth_dn4 = assign17480_e17328_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign17480_e17328_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign17480_e17328_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign17480_e17328_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign17480_e17328_d_n9;

        let (assign17490_e17354, assign17490_e17354_d_n4, assign17490_e17354_d_n6, assign17490_e17354_d_n7, assign17490_e17354_d_n8, assign17490_e17354_d_n9,) = {
    if ((locals.var_guard620 == 0.0) && (locals.var_guard621 == 0.0)) {
        let assign17490_e17338: f64 = (locals.var_q_qsq * 0.0714285714286);
        let assign17490_e17342: f64 = (0.05 * locals.var_q_qsq);
        let assign17490_e17346: f64 = (0.0420875420875421 * locals.var_q_qsq);
        let assign17490_e17347: f64 = (1.0 - assign17490_e17346);
        let assign17490_e17348: f64 = (assign17490_e17342 * assign17490_e17347);
        let assign17490_e17349: f64 = (1.0 - assign17490_e17348);
        let assign17490_e17350: f64 = (assign17490_e17338 * assign17490_e17349);
        let assign17490_e17351: f64 = (1.0 - assign17490_e17350);
        let assign17490_e17352: f64 = (0.0055555555556 * assign17490_e17351);
        (assign17490_e17352, (0.0055555555556 * (-(((locals.var_q_qsq_dn4 * 0.0714285714286) * assign17490_e17349) + (assign17490_e17338 * (-(((0.05 * locals.var_q_qsq_dn4) * assign17490_e17347) + (assign17490_e17342 * (-(0.0420875420875421 * locals.var_q_qsq_dn4))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn6 * 0.0714285714286) * assign17490_e17349) + (assign17490_e17338 * (-(((0.05 * locals.var_q_qsq_dn6) * assign17490_e17347) + (assign17490_e17342 * (-(0.0420875420875421 * locals.var_q_qsq_dn6))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn7 * 0.0714285714286) * assign17490_e17349) + (assign17490_e17338 * (-(((0.05 * locals.var_q_qsq_dn7) * assign17490_e17347) + (assign17490_e17342 * (-(0.0420875420875421 * locals.var_q_qsq_dn7))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn8 * 0.0714285714286) * assign17490_e17349) + (assign17490_e17338 * (-(((0.05 * locals.var_q_qsq_dn8) * assign17490_e17347) + (assign17490_e17342 * (-(0.0420875420875421 * locals.var_q_qsq_dn8))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn9 * 0.0714285714286) * assign17490_e17349) + (assign17490_e17338 * (-(((0.05 * locals.var_q_qsq_dn9) * assign17490_e17347) + (assign17490_e17342 * (-(0.0420875420875421 * locals.var_q_qsq_dn9))))))))),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign17490_e17354;
        locals.var_q_temp2_dn4 = assign17490_e17354_d_n4;
        locals.var_q_temp2_dn6 = assign17490_e17354_d_n6;
        locals.var_q_temp2_dn7 = assign17490_e17354_d_n7;
        locals.var_q_temp2_dn8 = assign17490_e17354_d_n8;
        locals.var_q_temp2_dn9 = assign17490_e17354_d_n9;

        let (assign17500_e17370, assign17500_e17370_d_n4, assign17500_e17370_d_n6, assign17500_e17370_d_n7, assign17500_e17370_d_n8, assign17500_e17370_d_n9,) = {
    if ((locals.var_guard620 == 0.0) && (locals.var_guard621 == 0.0)) {
        let assign17500_e17362: f64 = (locals.var_q_d2_qsq * locals.var_q_temp1);
        let assign17500_e17365: f64 = (locals.var_q_d1_qsq * locals.var_q_d1_qsq);
        let assign17500_e17367: f64 = (assign17500_e17365 * locals.var_q_temp2);
        let assign17500_e17368: f64 = (assign17500_e17362 - assign17500_e17367);
        (assign17500_e17368, (((locals.var_q_d2_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn4)) - ((((locals.var_q_d1_qsq_dn4 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn4)) * locals.var_q_temp2) + (assign17500_e17365 * locals.var_q_temp2_dn4))), (((locals.var_q_d2_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn6)) - ((((locals.var_q_d1_qsq_dn6 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn6)) * locals.var_q_temp2) + (assign17500_e17365 * locals.var_q_temp2_dn6))), (((locals.var_q_d2_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn7)) - ((((locals.var_q_d1_qsq_dn7 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn7)) * locals.var_q_temp2) + (assign17500_e17365 * locals.var_q_temp2_dn7))), (((locals.var_q_d2_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn8)) - ((((locals.var_q_d1_qsq_dn8 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn8)) * locals.var_q_temp2) + (assign17500_e17365 * locals.var_q_temp2_dn8))), (((locals.var_q_d2_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn9)) - ((((locals.var_q_d1_qsq_dn9 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn9)) * locals.var_q_temp2) + (assign17500_e17365 * locals.var_q_temp2_dn9))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign17500_e17370;
        locals.var_q_d2_qcoth_dn4 = assign17500_e17370_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign17500_e17370_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign17500_e17370_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign17500_e17370_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign17500_e17370_d_n9;

        let (assign17510_e17383, assign17510_e17383_d_n4, assign17510_e17383_d_n6, assign17510_e17383_d_n7, assign17510_e17383_d_n8, assign17510_e17383_d_n9,) = {
    if ((locals.var_guard620 == 0.0) && (locals.var_guard621 == 0.0)) {
        let assign17510_e17377: f64 = (-0.5);
        let assign17510_e17379: f64 = (assign17510_e17377 * locals.var_q_d1_qsq);
        let assign17510_e17381: f64 = (assign17510_e17379 * locals.var_q_temp3);
        (assign17510_e17381, (((assign17510_e17377 * locals.var_q_d1_qsq_dn4) * locals.var_q_temp3) + (assign17510_e17379 * locals.var_q_temp3_dn4)), (((assign17510_e17377 * locals.var_q_d1_qsq_dn6) * locals.var_q_temp3) + (assign17510_e17379 * locals.var_q_temp3_dn6)), (((assign17510_e17377 * locals.var_q_d1_qsq_dn7) * locals.var_q_temp3) + (assign17510_e17379 * locals.var_q_temp3_dn7)), (((assign17510_e17377 * locals.var_q_d1_qsq_dn8) * locals.var_q_temp3) + (assign17510_e17379 * locals.var_q_temp3_dn8)), (((assign17510_e17377 * locals.var_q_d1_qsq_dn9) * locals.var_q_temp3) + (assign17510_e17379 * locals.var_q_temp3_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign17510_e17383;
        locals.var_q_d1_ln_dn4 = assign17510_e17383_d_n4;
        locals.var_q_d1_ln_dn6 = assign17510_e17383_d_n6;
        locals.var_q_d1_ln_dn7 = assign17510_e17383_d_n7;
        locals.var_q_d1_ln_dn8 = assign17510_e17383_d_n8;
        locals.var_q_d1_ln_dn9 = assign17510_e17383_d_n9;

        let (assign17520_e17416, assign17520_e17416_d_n4, assign17520_e17416_d_n6, assign17520_e17416_d_n7, assign17520_e17416_d_n8, assign17520_e17416_d_n9,) = {
    if ((locals.var_guard620 == 0.0) && (locals.var_guard621 == 0.0)) {
        let assign17520_e17390: f64 = (-0.5);
        let assign17520_e17392: f64 = (assign17520_e17390 * locals.var_q_d2_qsq);
        let assign17520_e17394: f64 = (assign17520_e17392 * locals.var_q_temp3);
        let assign17520_e17397: f64 = (0.25 * 0.0055555555556);
        let assign17520_e17399: f64 = (assign17520_e17397 * locals.var_q_d1_qsq);
        let assign17520_e17401: f64 = (assign17520_e17399 * locals.var_q_d1_qsq);
        let assign17520_e17405: f64 = (locals.var_q_qsq * 0.0238095238095);
        let assign17520_e17409: f64 = (0.075 * locals.var_q_qsq);
        let assign17520_e17410: f64 = (2.0 - assign17520_e17409);
        let assign17520_e17411: f64 = (assign17520_e17405 * assign17520_e17410);
        let assign17520_e17412: f64 = (1.0 - assign17520_e17411);
        let assign17520_e17413: f64 = (assign17520_e17401 * assign17520_e17412);
        let assign17520_e17414: f64 = (assign17520_e17394 + assign17520_e17413);
        (assign17520_e17414, ((((assign17520_e17390 * locals.var_q_d2_qsq_dn4) * locals.var_q_temp3) + (assign17520_e17392 * locals.var_q_temp3_dn4)) + (((((assign17520_e17397 * locals.var_q_d1_qsq_dn4) * locals.var_q_d1_qsq) + (assign17520_e17399 * locals.var_q_d1_qsq_dn4)) * assign17520_e17412) + (assign17520_e17401 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign17520_e17410) + (assign17520_e17405 * (-(0.075 * locals.var_q_qsq_dn4)))))))), ((((assign17520_e17390 * locals.var_q_d2_qsq_dn6) * locals.var_q_temp3) + (assign17520_e17392 * locals.var_q_temp3_dn6)) + (((((assign17520_e17397 * locals.var_q_d1_qsq_dn6) * locals.var_q_d1_qsq) + (assign17520_e17399 * locals.var_q_d1_qsq_dn6)) * assign17520_e17412) + (assign17520_e17401 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign17520_e17410) + (assign17520_e17405 * (-(0.075 * locals.var_q_qsq_dn6)))))))), ((((assign17520_e17390 * locals.var_q_d2_qsq_dn7) * locals.var_q_temp3) + (assign17520_e17392 * locals.var_q_temp3_dn7)) + (((((assign17520_e17397 * locals.var_q_d1_qsq_dn7) * locals.var_q_d1_qsq) + (assign17520_e17399 * locals.var_q_d1_qsq_dn7)) * assign17520_e17412) + (assign17520_e17401 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign17520_e17410) + (assign17520_e17405 * (-(0.075 * locals.var_q_qsq_dn7)))))))), ((((assign17520_e17390 * locals.var_q_d2_qsq_dn8) * locals.var_q_temp3) + (assign17520_e17392 * locals.var_q_temp3_dn8)) + (((((assign17520_e17397 * locals.var_q_d1_qsq_dn8) * locals.var_q_d1_qsq) + (assign17520_e17399 * locals.var_q_d1_qsq_dn8)) * assign17520_e17412) + (assign17520_e17401 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign17520_e17410) + (assign17520_e17405 * (-(0.075 * locals.var_q_qsq_dn8)))))))), ((((assign17520_e17390 * locals.var_q_d2_qsq_dn9) * locals.var_q_temp3) + (assign17520_e17392 * locals.var_q_temp3_dn9)) + (((((assign17520_e17397 * locals.var_q_d1_qsq_dn9) * locals.var_q_d1_qsq) + (assign17520_e17399 * locals.var_q_d1_qsq_dn9)) * assign17520_e17412) + (assign17520_e17401 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign17520_e17410) + (assign17520_e17405 * (-(0.075 * locals.var_q_qsq_dn9)))))))),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign17520_e17416;
        locals.var_q_d2_ln_dn4 = assign17520_e17416_d_n4;
        locals.var_q_d2_ln_dn6 = assign17520_e17416_d_n6;
        locals.var_q_d2_ln_dn7 = assign17520_e17416_d_n7;
        locals.var_q_d2_ln_dn8 = assign17520_e17416_d_n8;
        locals.var_q_d2_ln_dn9 = assign17520_e17416_d_n9;

        let assign17530_e17419: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard622 = assign17530_e17419;

        let (assign17540_e17433, assign17540_e17433_d_n4, assign17540_e17433_d_n6, assign17540_e17433_d_n7, assign17540_e17433_d_n8, assign17540_e17433_d_n9,) = {
    if (locals.var_guard622 != 0.0) {
        let assign17540_e17423: f64 = (4.0 * locals.var_q_qsq);
        let assign17540_e17428: f64 = (2.0 - locals.var_q_invexpq);
        let assign17540_e17429: f64 = (locals.var_q_invexpq * assign17540_e17428);
        let assign17540_e17430: f64 = (1.0 - assign17540_e17429);
        let assign17540_e17431: f64 = (assign17540_e17423 / assign17540_e17430);
        (assign17540_e17431, ((((4.0 * locals.var_q_qsq_dn4) * assign17540_e17430) - (assign17540_e17423 * (-((locals.var_q_invexpq_dn4 * assign17540_e17428) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn4)))))) / (assign17540_e17430 * assign17540_e17430)), ((((4.0 * locals.var_q_qsq_dn6) * assign17540_e17430) - (assign17540_e17423 * (-((locals.var_q_invexpq_dn6 * assign17540_e17428) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn6)))))) / (assign17540_e17430 * assign17540_e17430)), ((((4.0 * locals.var_q_qsq_dn7) * assign17540_e17430) - (assign17540_e17423 * (-((locals.var_q_invexpq_dn7 * assign17540_e17428) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn7)))))) / (assign17540_e17430 * assign17540_e17430)), ((((4.0 * locals.var_q_qsq_dn8) * assign17540_e17430) - (assign17540_e17423 * (-((locals.var_q_invexpq_dn8 * assign17540_e17428) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn8)))))) / (assign17540_e17430 * assign17540_e17430)), ((((4.0 * locals.var_q_qsq_dn9) * assign17540_e17430) - (assign17540_e17423 * (-((locals.var_q_invexpq_dn9 * assign17540_e17428) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn9)))))) / (assign17540_e17430 * assign17540_e17430)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign17540_e17433;
        locals.var_q_temp2_dn4 = assign17540_e17433_d_n4;
        locals.var_q_temp2_dn6 = assign17540_e17433_d_n6;
        locals.var_q_temp2_dn7 = assign17540_e17433_d_n7;
        locals.var_q_temp2_dn8 = assign17540_e17433_d_n8;
        locals.var_q_temp2_dn9 = assign17540_e17433_d_n9;

        let (assign17550_e17439, assign17550_e17439_d_n4, assign17550_e17439_d_n6, assign17550_e17439_d_n7, assign17550_e17439_d_n8, assign17550_e17439_d_n9,) = {
    if (locals.var_guard622 != 0.0) {
        let assign17550_e17437: f64 = (locals.var_q_temp2 * locals.var_q_invexpq);
        (assign17550_e17437, ((locals.var_q_temp2_dn4 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn4)), ((locals.var_q_temp2_dn6 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn6)), ((locals.var_q_temp2_dn7 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn7)), ((locals.var_q_temp2_dn8 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn8)), ((locals.var_q_temp2_dn9 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn9)),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign17550_e17439;
        locals.var_q_sh_term_dn4 = assign17550_e17439_d_n4;
        locals.var_q_sh_term_dn6 = assign17550_e17439_d_n6;
        locals.var_q_sh_term_dn7 = assign17550_e17439_d_n7;
        locals.var_q_sh_term_dn8 = assign17550_e17439_d_n8;
        locals.var_q_sh_term_dn9 = assign17550_e17439_d_n9;

        let (assign17560_e17446, assign17560_e17446_d_n4, assign17560_e17446_d_n6, assign17560_e17446_d_n7, assign17560_e17446_d_n8, assign17560_e17446_d_n9,) = {
    if (locals.var_guard622 != 0.0) {
        let assign17560_e17442: f64 = (locals.var_q_temp2).ln();
        let assign17560_e17444: f64 = (assign17560_e17442 - locals.var_q_rac_qsq);
        (assign17560_e17444, ((locals.var_q_temp2_dn4 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn4), ((locals.var_q_temp2_dn6 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn6), ((locals.var_q_temp2_dn7 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn7), ((locals.var_q_temp2_dn8 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn8), ((locals.var_q_temp2_dn9 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn9),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign17560_e17446;
        locals.var_q_ln_term_dn4 = assign17560_e17446_d_n4;
        locals.var_q_ln_term_dn6 = assign17560_e17446_d_n6;
        locals.var_q_ln_term_dn7 = assign17560_e17446_d_n7;
        locals.var_q_ln_term_dn8 = assign17560_e17446_d_n8;
        locals.var_q_ln_term_dn9 = assign17560_e17446_d_n9;

        let assign17570_e17449: f64 = (-0.005);
        let assign17570_e17450: f64 = if locals.var_q_qsq < assign17570_e17449 { 1.0 } else { 0.0 };
        locals.var_guard623 = assign17570_e17450;

        let (assign17580_e17460, assign17580_e17460_d_n4, assign17580_e17460_d_n6, assign17580_e17460_d_n7, assign17580_e17460_d_n8, assign17580_e17460_d_n9,) = {
    if ((locals.var_guard622 == 0.0) && (locals.var_guard623 != 0.0)) {
        let assign17580_e17457: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign17580_e17458: f64 = (assign17580_e17457).sin();
        (assign17580_e17458, ((assign17580_e17457).cos() * (0.5 * locals.var_q_rac_qsq_dn4)), ((assign17580_e17457).cos() * (0.5 * locals.var_q_rac_qsq_dn6)), ((assign17580_e17457).cos() * (0.5 * locals.var_q_rac_qsq_dn7)), ((assign17580_e17457).cos() * (0.5 * locals.var_q_rac_qsq_dn8)), ((assign17580_e17457).cos() * (0.5 * locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign17580_e17460;
        locals.var_q_temp2_dn4 = assign17580_e17460_d_n4;
        locals.var_q_temp2_dn6 = assign17580_e17460_d_n6;
        locals.var_q_temp2_dn7 = assign17580_e17460_d_n7;
        locals.var_q_temp2_dn8 = assign17580_e17460_d_n8;
        locals.var_q_temp2_dn9 = assign17580_e17460_d_n9;

        let (assign17590_e17472, assign17590_e17472_d_n4, assign17590_e17472_d_n6, assign17590_e17472_d_n7, assign17590_e17472_d_n8, assign17590_e17472_d_n9,) = {
    if ((locals.var_guard622 == 0.0) && (locals.var_guard623 != 0.0)) {
        let assign17590_e17466: f64 = (-locals.var_q_qsq);
        let assign17590_e17469: f64 = (locals.var_q_temp2 * locals.var_q_temp2);
        let assign17590_e17470: f64 = (assign17590_e17466 / assign17590_e17469);
        (assign17590_e17470, ((((-locals.var_q_qsq_dn4) * assign17590_e17469) - (assign17590_e17466 * ((locals.var_q_temp2_dn4 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn4)))) / (assign17590_e17469 * assign17590_e17469)), ((((-locals.var_q_qsq_dn6) * assign17590_e17469) - (assign17590_e17466 * ((locals.var_q_temp2_dn6 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn6)))) / (assign17590_e17469 * assign17590_e17469)), ((((-locals.var_q_qsq_dn7) * assign17590_e17469) - (assign17590_e17466 * ((locals.var_q_temp2_dn7 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn7)))) / (assign17590_e17469 * assign17590_e17469)), ((((-locals.var_q_qsq_dn8) * assign17590_e17469) - (assign17590_e17466 * ((locals.var_q_temp2_dn8 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn8)))) / (assign17590_e17469 * assign17590_e17469)), ((((-locals.var_q_qsq_dn9) * assign17590_e17469) - (assign17590_e17466 * ((locals.var_q_temp2_dn9 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn9)))) / (assign17590_e17469 * assign17590_e17469)),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign17590_e17472;
        locals.var_q_sh_term_dn4 = assign17590_e17472_d_n4;
        locals.var_q_sh_term_dn6 = assign17590_e17472_d_n6;
        locals.var_q_sh_term_dn7 = assign17590_e17472_d_n7;
        locals.var_q_sh_term_dn8 = assign17590_e17472_d_n8;
        locals.var_q_sh_term_dn9 = assign17590_e17472_d_n9;

        let (assign17600_e17480, assign17600_e17480_d_n4, assign17600_e17480_d_n6, assign17600_e17480_d_n7, assign17600_e17480_d_n8, assign17600_e17480_d_n9,) = {
    if ((locals.var_guard622 == 0.0) && (locals.var_guard623 != 0.0)) {
        let assign17600_e17478: f64 = (locals.var_q_sh_term).ln();
        (assign17600_e17478, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign17600_e17480;
        locals.var_q_ln_term_dn4 = assign17600_e17480_d_n4;
        locals.var_q_ln_term_dn6 = assign17600_e17480_d_n6;
        locals.var_q_ln_term_dn7 = assign17600_e17480_d_n7;
        locals.var_q_ln_term_dn8 = assign17600_e17480_d_n8;
        locals.var_q_ln_term_dn9 = assign17600_e17480_d_n9;

        let (assign17610_e17504, assign17610_e17504_d_n4, assign17610_e17504_d_n6, assign17610_e17504_d_n7, assign17610_e17504_d_n8, assign17610_e17504_d_n9,) = {
    if ((locals.var_guard622 == 0.0) && (locals.var_guard623 == 0.0)) {
        let assign17610_e17489: f64 = (locals.var_q_qsq * 0.3333333333333);
        let assign17610_e17493: f64 = (0.05 * locals.var_q_qsq);
        let assign17610_e17497: f64 = (0.0396825396825397 * locals.var_q_qsq);
        let assign17610_e17498: f64 = (1.0 - assign17610_e17497);
        let assign17610_e17499: f64 = (assign17610_e17493 * assign17610_e17498);
        let assign17610_e17500: f64 = (1.0 - assign17610_e17499);
        let assign17610_e17501: f64 = (assign17610_e17489 * assign17610_e17500);
        let assign17610_e17502: f64 = (4.0 - assign17610_e17501);
        (assign17610_e17502, (-(((locals.var_q_qsq_dn4 * 0.3333333333333) * assign17610_e17500) + (assign17610_e17489 * (-(((0.05 * locals.var_q_qsq_dn4) * assign17610_e17498) + (assign17610_e17493 * (-(0.0396825396825397 * locals.var_q_qsq_dn4)))))))), (-(((locals.var_q_qsq_dn6 * 0.3333333333333) * assign17610_e17500) + (assign17610_e17489 * (-(((0.05 * locals.var_q_qsq_dn6) * assign17610_e17498) + (assign17610_e17493 * (-(0.0396825396825397 * locals.var_q_qsq_dn6)))))))), (-(((locals.var_q_qsq_dn7 * 0.3333333333333) * assign17610_e17500) + (assign17610_e17489 * (-(((0.05 * locals.var_q_qsq_dn7) * assign17610_e17498) + (assign17610_e17493 * (-(0.0396825396825397 * locals.var_q_qsq_dn7)))))))), (-(((locals.var_q_qsq_dn8 * 0.3333333333333) * assign17610_e17500) + (assign17610_e17489 * (-(((0.05 * locals.var_q_qsq_dn8) * assign17610_e17498) + (assign17610_e17493 * (-(0.0396825396825397 * locals.var_q_qsq_dn8)))))))), (-(((locals.var_q_qsq_dn9 * 0.3333333333333) * assign17610_e17500) + (assign17610_e17489 * (-(((0.05 * locals.var_q_qsq_dn9) * assign17610_e17498) + (assign17610_e17493 * (-(0.0396825396825397 * locals.var_q_qsq_dn9)))))))),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign17610_e17504;
        locals.var_q_sh_term_dn4 = assign17610_e17504_d_n4;
        locals.var_q_sh_term_dn6 = assign17610_e17504_d_n6;
        locals.var_q_sh_term_dn7 = assign17610_e17504_d_n7;
        locals.var_q_sh_term_dn8 = assign17610_e17504_d_n8;
        locals.var_q_sh_term_dn9 = assign17610_e17504_d_n9;

    }

    pub(super) fn stamp_transient_block_44(
        locals: &mut StampLocals,
    ) {
        let (assign17620_e17513, assign17620_e17513_d_n4, assign17620_e17513_d_n6, assign17620_e17513_d_n7, assign17620_e17513_d_n8, assign17620_e17513_d_n9,) = {
    if ((locals.var_guard622 == 0.0) && (locals.var_guard623 == 0.0)) {
        let assign17620_e17511: f64 = (locals.var_q_sh_term).ln();
        (assign17620_e17511, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign17620_e17513;
        locals.var_q_ln_term_dn4 = assign17620_e17513_d_n4;
        locals.var_q_ln_term_dn6 = assign17620_e17513_d_n6;
        locals.var_q_ln_term_dn7 = assign17620_e17513_d_n7;
        locals.var_q_ln_term_dn8 = assign17620_e17513_d_n8;
        locals.var_q_ln_term_dn9 = assign17620_e17513_d_n9;

        let assign17630_e17516: f64 = (1.01 * locals.var_q_k1q1);
        let assign17630_e17518: f64 = (assign17630_e17516 + locals.var_q_qcoth);
        let assign17630_e17520: f64 = if assign17630_e17518 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard624 = assign17630_e17520;

        let (assign17640_e17526, assign17640_e17526_d_n4, assign17640_e17526_d_n6, assign17640_e17526_d_n7, assign17640_e17526_d_n8, assign17640_e17526_d_n9,) = {
    if (locals.var_guard624 != 0.0) {
        let assign17640_e17524: f64 = (locals.var_q_k1q1 + locals.var_q_qcoth);
        (assign17640_e17524, (locals.var_q_k1q1_dn4 + locals.var_q_qcoth_dn4), (locals.var_q_k1q1_dn6 + locals.var_q_qcoth_dn6), (locals.var_q_k1q1_dn7 + locals.var_q_qcoth_dn7), (locals.var_q_k1q1_dn8 + locals.var_q_qcoth_dn8), (locals.var_q_k1q1_dn9 + locals.var_q_qcoth_dn9),)
    } else {
        (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9,)
    }
};
        locals.var_q_expnum = assign17640_e17526;
        locals.var_q_expnum_dn4 = assign17640_e17526_d_n4;
        locals.var_q_expnum_dn6 = assign17640_e17526_d_n6;
        locals.var_q_expnum_dn7 = assign17640_e17526_d_n7;
        locals.var_q_expnum_dn8 = assign17640_e17526_d_n8;
        locals.var_q_expnum_dn9 = assign17640_e17526_d_n9;

        let (assign17650_e17532, assign17650_e17532_d_n4, assign17650_e17532_d_n6, assign17650_e17532_d_n7, assign17650_e17532_d_n8, assign17650_e17532_d_n9,) = {
    if (locals.var_guard624 != 0.0) {
        let assign17650_e17530: f64 = (locals.var_k1 + locals.var_q_d1_qcoth);
        (assign17650_e17530, (locals.var_k1_dn4 + locals.var_q_d1_qcoth_dn4), (locals.var_k1_dn6 + locals.var_q_d1_qcoth_dn6), (locals.var_k1_dn7 + locals.var_q_d1_qcoth_dn7), (locals.var_k1_dn8 + locals.var_q_d1_qcoth_dn8), (locals.var_k1_dn9 + locals.var_q_d1_qcoth_dn9),)
    } else {
        (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9,)
    }
};
        locals.var_q_d1_expnum = assign17650_e17532;
        locals.var_q_d1_expnum_dn4 = assign17650_e17532_d_n4;
        locals.var_q_d1_expnum_dn6 = assign17650_e17532_d_n6;
        locals.var_q_d1_expnum_dn7 = assign17650_e17532_d_n7;
        locals.var_q_d1_expnum_dn8 = assign17650_e17532_d_n8;
        locals.var_q_d1_expnum_dn9 = assign17650_e17532_d_n9;

        let (assign17660_e17536, assign17660_e17536_d_n4, assign17660_e17536_d_n6, assign17660_e17536_d_n7, assign17660_e17536_d_n8, assign17660_e17536_d_n9,) = {
    if (locals.var_guard624 != 0.0) {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    } else {
        (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9,)
    }
};
        locals.var_q_d2_expnum = assign17660_e17536;
        locals.var_q_d2_expnum_dn4 = assign17660_e17536_d_n4;
        locals.var_q_d2_expnum_dn6 = assign17660_e17536_d_n6;
        locals.var_q_d2_expnum_dn7 = assign17660_e17536_d_n7;
        locals.var_q_d2_expnum_dn8 = assign17660_e17536_d_n8;
        locals.var_q_d2_expnum_dn9 = assign17660_e17536_d_n9;

        let (assign17670_e17545, assign17670_e17545_d_n4, assign17670_e17545_d_n6, assign17670_e17545_d_n7, assign17670_e17545_d_n8, assign17670_e17545_d_n9,) = {
    if (locals.var_guard624 == 0.0) {
        let assign17670_e17542: f64 = (locals.var_q_k1q1 - locals.var_q_qcoth);
        let assign17670_e17543: f64 = (1.0 / assign17670_e17542);
        (assign17670_e17543, (-((locals.var_q_k1q1_dn4 - locals.var_q_qcoth_dn4) / (assign17670_e17542 * assign17670_e17542))), (-((locals.var_q_k1q1_dn6 - locals.var_q_qcoth_dn6) / (assign17670_e17542 * assign17670_e17542))), (-((locals.var_q_k1q1_dn7 - locals.var_q_qcoth_dn7) / (assign17670_e17542 * assign17670_e17542))), (-((locals.var_q_k1q1_dn8 - locals.var_q_qcoth_dn8) / (assign17670_e17542 * assign17670_e17542))), (-((locals.var_q_k1q1_dn9 - locals.var_q_qcoth_dn9) / (assign17670_e17542 * assign17670_e17542))),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign17670_e17545;
        locals.var_q_temp2_dn4 = assign17670_e17545_d_n4;
        locals.var_q_temp2_dn6 = assign17670_e17545_d_n6;
        locals.var_q_temp2_dn7 = assign17670_e17545_d_n7;
        locals.var_q_temp2_dn8 = assign17670_e17545_d_n8;
        locals.var_q_temp2_dn9 = assign17670_e17545_d_n9;

        let (assign17680_e17552, assign17680_e17552_d_n4, assign17680_e17552_d_n6, assign17680_e17552_d_n7, assign17680_e17552_d_n8, assign17680_e17552_d_n9,) = {
    if (locals.var_guard624 == 0.0) {
        let assign17680_e17550: f64 = (locals.var_q_d1_qcoth - locals.var_k1);
        (assign17680_e17550, (locals.var_q_d1_qcoth_dn4 - locals.var_k1_dn4), (locals.var_q_d1_qcoth_dn6 - locals.var_k1_dn6), (locals.var_q_d1_qcoth_dn7 - locals.var_k1_dn7), (locals.var_q_d1_qcoth_dn8 - locals.var_k1_dn8), (locals.var_q_d1_qcoth_dn9 - locals.var_k1_dn9),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign17680_e17552;
        locals.var_q_temp3_dn4 = assign17680_e17552_d_n4;
        locals.var_q_temp3_dn6 = assign17680_e17552_d_n6;
        locals.var_q_temp3_dn7 = assign17680_e17552_d_n7;
        locals.var_q_temp3_dn8 = assign17680_e17552_d_n8;
        locals.var_q_temp3_dn9 = assign17680_e17552_d_n9;

        let (assign17690_e17561, assign17690_e17561_d_n4, assign17690_e17561_d_n6, assign17690_e17561_d_n7, assign17690_e17561_d_n8, assign17690_e17561_d_n9,) = {
    if (locals.var_guard624 == 0.0) {
        let assign17690_e17557: f64 = (locals.var_q_aexp - locals.var_q_sh_term);
        let assign17690_e17559: f64 = (assign17690_e17557 * locals.var_q_temp2);
        (assign17690_e17559, (((locals.var_q_aexp_dn4 - locals.var_q_sh_term_dn4) * locals.var_q_temp2) + (assign17690_e17557 * locals.var_q_temp2_dn4)), (((locals.var_q_aexp_dn6 - locals.var_q_sh_term_dn6) * locals.var_q_temp2) + (assign17690_e17557 * locals.var_q_temp2_dn6)), (((locals.var_q_aexp_dn7 - locals.var_q_sh_term_dn7) * locals.var_q_temp2) + (assign17690_e17557 * locals.var_q_temp2_dn7)), (((locals.var_q_aexp_dn8 - locals.var_q_sh_term_dn8) * locals.var_q_temp2) + (assign17690_e17557 * locals.var_q_temp2_dn8)), (((locals.var_q_aexp_dn9 - locals.var_q_sh_term_dn9) * locals.var_q_temp2) + (assign17690_e17557 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9,)
    }
};
        locals.var_q_expnum = assign17690_e17561;
        locals.var_q_expnum_dn4 = assign17690_e17561_d_n4;
        locals.var_q_expnum_dn6 = assign17690_e17561_d_n6;
        locals.var_q_expnum_dn7 = assign17690_e17561_d_n7;
        locals.var_q_expnum_dn8 = assign17690_e17561_d_n8;
        locals.var_q_expnum_dn9 = assign17690_e17561_d_n9;

        let (assign17700_e17576, assign17700_e17576_d_n4, assign17700_e17576_d_n6, assign17700_e17576_d_n7, assign17700_e17576_d_n8, assign17700_e17576_d_n9,) = {
    if (locals.var_guard624 == 0.0) {
        let assign17700_e17566: f64 = (locals.var_q_temp3 * locals.var_q_expnum);
        let assign17700_e17568: f64 = (assign17700_e17566 - locals.var_q_aexp);
        let assign17700_e17571: f64 = (locals.var_q_d1_ln * locals.var_q_sh_term);
        let assign17700_e17572: f64 = (assign17700_e17568 - assign17700_e17571);
        let assign17700_e17574: f64 = (assign17700_e17572 * locals.var_q_temp2);
        (assign17700_e17574, ((((((locals.var_q_temp3_dn4 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4) - ((locals.var_q_d1_ln_dn4 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign17700_e17572 * locals.var_q_temp2_dn4)), ((((((locals.var_q_temp3_dn6 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6) - ((locals.var_q_d1_ln_dn6 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign17700_e17572 * locals.var_q_temp2_dn6)), ((((((locals.var_q_temp3_dn7 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7) - ((locals.var_q_d1_ln_dn7 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign17700_e17572 * locals.var_q_temp2_dn7)), ((((((locals.var_q_temp3_dn8 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8) - ((locals.var_q_d1_ln_dn8 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign17700_e17572 * locals.var_q_temp2_dn8)), ((((((locals.var_q_temp3_dn9 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9) - ((locals.var_q_d1_ln_dn9 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign17700_e17572 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9,)
    }
};
        locals.var_q_d1_expnum = assign17700_e17576;
        locals.var_q_d1_expnum_dn4 = assign17700_e17576_d_n4;
        locals.var_q_d1_expnum_dn6 = assign17700_e17576_d_n6;
        locals.var_q_d1_expnum_dn7 = assign17700_e17576_d_n7;
        locals.var_q_d1_expnum_dn8 = assign17700_e17576_d_n8;
        locals.var_q_d1_expnum_dn9 = assign17700_e17576_d_n9;

        let (assign17710_e17601, assign17710_e17601_d_n4, assign17710_e17601_d_n6, assign17710_e17601_d_n7, assign17710_e17601_d_n8, assign17710_e17601_d_n9,) = {
    if (locals.var_guard624 == 0.0) {
        let assign17710_e17581: f64 = (locals.var_q_d2_qcoth * locals.var_q_expnum);
        let assign17710_e17584: f64 = (2.0 * locals.var_q_temp3);
        let assign17710_e17586: f64 = (assign17710_e17584 * locals.var_q_d1_expnum);
        let assign17710_e17587: f64 = (assign17710_e17581 + assign17710_e17586);
        let assign17710_e17589: f64 = (assign17710_e17587 + locals.var_q_aexp);
        let assign17710_e17593: f64 = (locals.var_q_d1_ln * locals.var_q_d1_ln);
        let assign17710_e17594: f64 = (locals.var_q_d2_ln + assign17710_e17593);
        let assign17710_e17596: f64 = (assign17710_e17594 * locals.var_q_sh_term);
        let assign17710_e17597: f64 = (assign17710_e17589 - assign17710_e17596);
        let assign17710_e17599: f64 = (assign17710_e17597 * locals.var_q_temp2);
        (assign17710_e17599, (((((((locals.var_q_d2_qcoth_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_temp3_dn4) * locals.var_q_d1_expnum) + (assign17710_e17584 * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4) - (((locals.var_q_d2_ln_dn4 + ((locals.var_q_d1_ln_dn4 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn4))) * locals.var_q_sh_term) + (assign17710_e17594 * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign17710_e17597 * locals.var_q_temp2_dn4)), (((((((locals.var_q_d2_qcoth_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_temp3_dn6) * locals.var_q_d1_expnum) + (assign17710_e17584 * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6) - (((locals.var_q_d2_ln_dn6 + ((locals.var_q_d1_ln_dn6 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn6))) * locals.var_q_sh_term) + (assign17710_e17594 * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign17710_e17597 * locals.var_q_temp2_dn6)), (((((((locals.var_q_d2_qcoth_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_temp3_dn7) * locals.var_q_d1_expnum) + (assign17710_e17584 * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7) - (((locals.var_q_d2_ln_dn7 + ((locals.var_q_d1_ln_dn7 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn7))) * locals.var_q_sh_term) + (assign17710_e17594 * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign17710_e17597 * locals.var_q_temp2_dn7)), (((((((locals.var_q_d2_qcoth_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_temp3_dn8) * locals.var_q_d1_expnum) + (assign17710_e17584 * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8) - (((locals.var_q_d2_ln_dn8 + ((locals.var_q_d1_ln_dn8 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn8))) * locals.var_q_sh_term) + (assign17710_e17594 * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign17710_e17597 * locals.var_q_temp2_dn8)), (((((((locals.var_q_d2_qcoth_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_temp3_dn9) * locals.var_q_d1_expnum) + (assign17710_e17584 * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9) - (((locals.var_q_d2_ln_dn9 + ((locals.var_q_d1_ln_dn9 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn9))) * locals.var_q_sh_term) + (assign17710_e17594 * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign17710_e17597 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9,)
    }
};
        locals.var_q_d2_expnum = assign17710_e17601;
        locals.var_q_d2_expnum_dn4 = assign17710_e17601_d_n4;
        locals.var_q_d2_expnum_dn6 = assign17710_e17601_d_n6;
        locals.var_q_d2_expnum_dn7 = assign17710_e17601_d_n7;
        locals.var_q_d2_expnum_dn8 = assign17710_e17601_d_n8;
        locals.var_q_d2_expnum_dn9 = assign17710_e17601_d_n9;

        let assign17720_e17604: f64 = if locals.var_q_expnum > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard625 = assign17720_e17604;

        let (assign17730_e17609, assign17730_e17609_d_n4, assign17730_e17609_d_n6, assign17730_e17609_d_n7, assign17730_e17609_d_n8, assign17730_e17609_d_n9,) = {
    if (locals.var_guard625 != 0.0) {
        let assign17730_e17607: f64 = (locals.var_q_expnum).ln();
        (assign17730_e17607, (locals.var_q_expnum_dn4 / locals.var_q_expnum), (locals.var_q_expnum_dn6 / locals.var_q_expnum), (locals.var_q_expnum_dn7 / locals.var_q_expnum), (locals.var_q_expnum_dn8 / locals.var_q_expnum), (locals.var_q_expnum_dn9 / locals.var_q_expnum),)
    } else {
        (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9,)
    }
};
        locals.var_q_lnexpnum = assign17730_e17609;
        locals.var_q_lnexpnum_dn4 = assign17730_e17609_d_n4;
        locals.var_q_lnexpnum_dn6 = assign17730_e17609_d_n6;
        locals.var_q_lnexpnum_dn7 = assign17730_e17609_d_n7;
        locals.var_q_lnexpnum_dn8 = assign17730_e17609_d_n8;
        locals.var_q_lnexpnum_dn9 = assign17730_e17609_d_n9;

        let (assign17740_e17615, assign17740_e17615_d_n4, assign17740_e17615_d_n6, assign17740_e17615_d_n7, assign17740_e17615_d_n8, assign17740_e17615_d_n9,) = {
    if (locals.var_guard625 != 0.0) {
        let assign17740_e17613: f64 = (1.0 / locals.var_q_expnum);
        (assign17740_e17613, (-(locals.var_q_expnum_dn4 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn6 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn7 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn8 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn9 / (locals.var_q_expnum * locals.var_q_expnum))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign17740_e17615;
        locals.var_q_temp1_dn4 = assign17740_e17615_d_n4;
        locals.var_q_temp1_dn6 = assign17740_e17615_d_n6;
        locals.var_q_temp1_dn7 = assign17740_e17615_d_n7;
        locals.var_q_temp1_dn8 = assign17740_e17615_d_n8;
        locals.var_q_temp1_dn9 = assign17740_e17615_d_n9;

        let (assign17750_e17621, assign17750_e17621_d_n4, assign17750_e17621_d_n6, assign17750_e17621_d_n7, assign17750_e17621_d_n8, assign17750_e17621_d_n9,) = {
    if (locals.var_guard625 != 0.0) {
        let assign17750_e17619: f64 = (locals.var_q_d1_expnum * locals.var_q_temp1);
        (assign17750_e17619, ((locals.var_q_d1_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn4)), ((locals.var_q_d1_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn6)), ((locals.var_q_d1_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn7)), ((locals.var_q_d1_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn8)), ((locals.var_q_d1_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9,)
    }
};
        locals.var_q_d1_lnexpnum = assign17750_e17621;
        locals.var_q_d1_lnexpnum_dn4 = assign17750_e17621_d_n4;
        locals.var_q_d1_lnexpnum_dn6 = assign17750_e17621_d_n6;
        locals.var_q_d1_lnexpnum_dn7 = assign17750_e17621_d_n7;
        locals.var_q_d1_lnexpnum_dn8 = assign17750_e17621_d_n8;
        locals.var_q_d1_lnexpnum_dn9 = assign17750_e17621_d_n9;

        let (assign17760_e17631, assign17760_e17631_d_n4, assign17760_e17631_d_n6, assign17760_e17631_d_n7, assign17760_e17631_d_n8, assign17760_e17631_d_n9,) = {
    if (locals.var_guard625 != 0.0) {
        let assign17760_e17625: f64 = (locals.var_q_d2_expnum * locals.var_q_temp1);
        let assign17760_e17628: f64 = (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum);
        let assign17760_e17629: f64 = (assign17760_e17625 - assign17760_e17628);
        (assign17760_e17629, (((locals.var_q_d2_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn4)) - ((locals.var_q_d1_lnexpnum_dn4 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn4))), (((locals.var_q_d2_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn6)) - ((locals.var_q_d1_lnexpnum_dn6 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn6))), (((locals.var_q_d2_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn7)) - ((locals.var_q_d1_lnexpnum_dn7 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn7))), (((locals.var_q_d2_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn8)) - ((locals.var_q_d1_lnexpnum_dn8 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn8))), (((locals.var_q_d2_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn9)) - ((locals.var_q_d1_lnexpnum_dn9 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn9))),)
    } else {
        (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9,)
    }
};
        locals.var_q_d2_lnexpnum = assign17760_e17631;
        locals.var_q_d2_lnexpnum_dn4 = assign17760_e17631_d_n4;
        locals.var_q_d2_lnexpnum_dn6 = assign17760_e17631_d_n6;
        locals.var_q_d2_lnexpnum_dn7 = assign17760_e17631_d_n7;
        locals.var_q_d2_lnexpnum_dn8 = assign17760_e17631_d_n8;
        locals.var_q_d2_lnexpnum_dn9 = assign17760_e17631_d_n9;

        let (assign17770_e17642, assign17770_e17642_d_n4, assign17770_e17642_d_n6, assign17770_e17642_d_n7, assign17770_e17642_d_n8, assign17770_e17642_d_n9,) = {
    if (locals.var_guard625 == 0.0) {
        let assign17770_e17636: f64 = (locals.var_q_k1q1 + 0.6931471805599);
        let assign17770_e17638: f64 = (-locals.var_q_k1q1);
        let assign17770_e17639: f64 = (assign17770_e17638).ln();
        let assign17770_e17640: f64 = (assign17770_e17636 + assign17770_e17639);
        (assign17770_e17640, (locals.var_q_k1q1_dn4 + ((-locals.var_q_k1q1_dn4) / assign17770_e17638)), (locals.var_q_k1q1_dn6 + ((-locals.var_q_k1q1_dn6) / assign17770_e17638)), (locals.var_q_k1q1_dn7 + ((-locals.var_q_k1q1_dn7) / assign17770_e17638)), (locals.var_q_k1q1_dn8 + ((-locals.var_q_k1q1_dn8) / assign17770_e17638)), (locals.var_q_k1q1_dn9 + ((-locals.var_q_k1q1_dn9) / assign17770_e17638)),)
    } else {
        (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9,)
    }
};
        locals.var_q_lnexpnum = assign17770_e17642;
        locals.var_q_lnexpnum_dn4 = assign17770_e17642_d_n4;
        locals.var_q_lnexpnum_dn6 = assign17770_e17642_d_n6;
        locals.var_q_lnexpnum_dn7 = assign17770_e17642_d_n7;
        locals.var_q_lnexpnum_dn8 = assign17770_e17642_d_n8;
        locals.var_q_lnexpnum_dn9 = assign17770_e17642_d_n9;

        let (assign17780_e17649, assign17780_e17649_d_n4, assign17780_e17649_d_n6, assign17780_e17649_d_n7, assign17780_e17649_d_n8, assign17780_e17649_d_n9,) = {
    if (locals.var_guard625 == 0.0) {
        let assign17780_e17647: f64 = (1.0 / locals.var_q1d);
        (assign17780_e17647, (-(locals.var_q1d_dn4 / (locals.var_q1d * locals.var_q1d))), (-(locals.var_q1d_dn6 / (locals.var_q1d * locals.var_q1d))), (-(locals.var_q1d_dn7 / (locals.var_q1d * locals.var_q1d))), (-(locals.var_q1d_dn8 / (locals.var_q1d * locals.var_q1d))), (-(locals.var_q1d_dn9 / (locals.var_q1d * locals.var_q1d))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign17780_e17649;
        locals.var_q_temp1_dn4 = assign17780_e17649_d_n4;
        locals.var_q_temp1_dn6 = assign17780_e17649_d_n6;
        locals.var_q_temp1_dn7 = assign17780_e17649_d_n7;
        locals.var_q_temp1_dn8 = assign17780_e17649_d_n8;
        locals.var_q_temp1_dn9 = assign17780_e17649_d_n9;

        let (assign17790_e17656, assign17790_e17656_d_n4, assign17790_e17656_d_n6, assign17790_e17656_d_n7, assign17790_e17656_d_n8, assign17790_e17656_d_n9,) = {
    if (locals.var_guard625 == 0.0) {
        let assign17790_e17654: f64 = (locals.var_k1 + locals.var_q_temp1);
        (assign17790_e17654, (locals.var_k1_dn4 + locals.var_q_temp1_dn4), (locals.var_k1_dn6 + locals.var_q_temp1_dn6), (locals.var_k1_dn7 + locals.var_q_temp1_dn7), (locals.var_k1_dn8 + locals.var_q_temp1_dn8), (locals.var_k1_dn9 + locals.var_q_temp1_dn9),)
    } else {
        (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9,)
    }
};
        locals.var_q_d1_lnexpnum = assign17790_e17656;
        locals.var_q_d1_lnexpnum_dn4 = assign17790_e17656_d_n4;
        locals.var_q_d1_lnexpnum_dn6 = assign17790_e17656_d_n6;
        locals.var_q_d1_lnexpnum_dn7 = assign17790_e17656_d_n7;
        locals.var_q_d1_lnexpnum_dn8 = assign17790_e17656_d_n8;
        locals.var_q_d1_lnexpnum_dn9 = assign17790_e17656_d_n9;

        let (assign17800_e17664, assign17800_e17664_d_n4, assign17800_e17664_d_n6, assign17800_e17664_d_n7, assign17800_e17664_d_n8, assign17800_e17664_d_n9,) = {
    if (locals.var_guard625 == 0.0) {
        let assign17800_e17660: f64 = (-locals.var_q_temp1);
        let assign17800_e17662: f64 = (assign17800_e17660 * locals.var_q_temp1);
        (assign17800_e17662, (((-locals.var_q_temp1_dn4) * locals.var_q_temp1) + (assign17800_e17660 * locals.var_q_temp1_dn4)), (((-locals.var_q_temp1_dn6) * locals.var_q_temp1) + (assign17800_e17660 * locals.var_q_temp1_dn6)), (((-locals.var_q_temp1_dn7) * locals.var_q_temp1) + (assign17800_e17660 * locals.var_q_temp1_dn7)), (((-locals.var_q_temp1_dn8) * locals.var_q_temp1) + (assign17800_e17660 * locals.var_q_temp1_dn8)), (((-locals.var_q_temp1_dn9) * locals.var_q_temp1) + (assign17800_e17660 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9,)
    }
};
        locals.var_q_d2_lnexpnum = assign17800_e17664;
        locals.var_q_d2_lnexpnum_dn4 = assign17800_e17664_d_n4;
        locals.var_q_d2_lnexpnum_dn6 = assign17800_e17664_d_n6;
        locals.var_q_d2_lnexpnum_dn7 = assign17800_e17664_d_n7;
        locals.var_q_d2_lnexpnum_dn8 = assign17800_e17664_d_n8;
        locals.var_q_d2_lnexpnum_dn9 = assign17800_e17664_d_n9;

        let assign17810_e17667: f64 = (locals.var_xg2x - locals.var_xg1x);
        let assign17810_e17669: f64 = (assign17810_e17667 + locals.var_q1d);
        let assign17810_e17672: f64 = (2.0 * locals.var_q_lnexpnum);
        let assign17810_e17673: f64 = (assign17810_e17669 + assign17810_e17672);
        let assign17810_e17675: f64 = (assign17810_e17673 - locals.var_q_ln_term);
        locals.var_q_q2_int = assign17810_e17675;
        locals.var_q_q2_int_dn4 = ((((locals.var_xg2x_dn4 - locals.var_xg1x_dn4) + locals.var_q1d_dn4) + (2.0 * locals.var_q_lnexpnum_dn4)) - locals.var_q_ln_term_dn4);
        locals.var_q_q2_int_dn6 = ((((locals.var_xg2x_dn6 - locals.var_xg1x_dn6) + locals.var_q1d_dn6) + (2.0 * locals.var_q_lnexpnum_dn6)) - locals.var_q_ln_term_dn6);
        locals.var_q_q2_int_dn7 = ((((locals.var_xg2x_dn7 - locals.var_xg1x_dn7) + locals.var_q1d_dn7) + (2.0 * locals.var_q_lnexpnum_dn7)) - locals.var_q_ln_term_dn7);
        locals.var_q_q2_int_dn8 = ((((locals.var_xg2x_dn8 - locals.var_xg1x_dn8) + locals.var_q1d_dn8) + (2.0 * locals.var_q_lnexpnum_dn8)) - locals.var_q_ln_term_dn8);
        locals.var_q_q2_int_dn9 = ((((locals.var_xg2x_dn9 - locals.var_xg1x_dn9) + locals.var_q1d_dn9) + (2.0 * locals.var_q_lnexpnum_dn9)) - locals.var_q_ln_term_dn9);

        let assign17820_e17679: f64 = (2.0 * locals.var_q_d1_lnexpnum);
        let assign17820_e17680: f64 = (1.0 + assign17820_e17679);
        let assign17820_e17682: f64 = (assign17820_e17680 - locals.var_q_d1_ln);
        locals.var_q_d1_q2 = assign17820_e17682;
        locals.var_q_d1_q2_dn4 = ((2.0 * locals.var_q_d1_lnexpnum_dn4) - locals.var_q_d1_ln_dn4);
        locals.var_q_d1_q2_dn6 = ((2.0 * locals.var_q_d1_lnexpnum_dn6) - locals.var_q_d1_ln_dn6);
        locals.var_q_d1_q2_dn7 = ((2.0 * locals.var_q_d1_lnexpnum_dn7) - locals.var_q_d1_ln_dn7);
        locals.var_q_d1_q2_dn8 = ((2.0 * locals.var_q_d1_lnexpnum_dn8) - locals.var_q_d1_ln_dn8);
        locals.var_q_d1_q2_dn9 = ((2.0 * locals.var_q_d1_lnexpnum_dn9) - locals.var_q_d1_ln_dn9);

        let assign17830_e17685: f64 = (2.0 * locals.var_q_d2_lnexpnum);
        let assign17830_e17687: f64 = (assign17830_e17685 - locals.var_q_d2_ln);
        locals.var_q_d2_q2 = assign17830_e17687;
        locals.var_q_d2_q2_dn4 = ((2.0 * locals.var_q_d2_lnexpnum_dn4) - locals.var_q_d2_ln_dn4);
        locals.var_q_d2_q2_dn6 = ((2.0 * locals.var_q_d2_lnexpnum_dn6) - locals.var_q_d2_ln_dn6);
        locals.var_q_d2_q2_dn7 = ((2.0 * locals.var_q_d2_lnexpnum_dn7) - locals.var_q_d2_ln_dn7);
        locals.var_q_d2_q2_dn8 = ((2.0 * locals.var_q_d2_lnexpnum_dn8) - locals.var_q_d2_ln_dn8);
        locals.var_q_d2_q2_dn9 = ((2.0 * locals.var_q_d2_lnexpnum_dn9) - locals.var_q_d2_ln_dn9);

        let assign17840_e17691: f64 = (locals.var_k2 * locals.var_q_q2_int);
        let assign17840_e17692: f64 = (locals.var_q_k1q1 + assign17840_e17691);
        locals.var_q_qi_int = assign17840_e17692;
        locals.var_q_qi_int_dn4 = (locals.var_q_k1q1_dn4 + ((locals.var_k2_dn4 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn4)));
        locals.var_q_qi_int_dn6 = (locals.var_q_k1q1_dn6 + ((locals.var_k2_dn6 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn6)));
        locals.var_q_qi_int_dn7 = (locals.var_q_k1q1_dn7 + ((locals.var_k2_dn7 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn7)));
        locals.var_q_qi_int_dn8 = (locals.var_q_k1q1_dn8 + ((locals.var_k2_dn8 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn8)));
        locals.var_q_qi_int_dn9 = (locals.var_q_k1q1_dn9 + ((locals.var_k2_dn9 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn9)));

        let assign17850_e17696: f64 = (locals.var_k2 * locals.var_q_d1_q2);
        let assign17850_e17697: f64 = (locals.var_k1 + assign17850_e17696);
        locals.var_q_d1_qi = assign17850_e17697;
        locals.var_q_d1_qi_dn4 = (locals.var_k1_dn4 + ((locals.var_k2_dn4 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn4)));
        locals.var_q_d1_qi_dn6 = (locals.var_k1_dn6 + ((locals.var_k2_dn6 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn6)));
        locals.var_q_d1_qi_dn7 = (locals.var_k1_dn7 + ((locals.var_k2_dn7 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn7)));
        locals.var_q_d1_qi_dn8 = (locals.var_k1_dn8 + ((locals.var_k2_dn8 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn8)));
        locals.var_q_d1_qi_dn9 = (locals.var_k1_dn9 + ((locals.var_k2_dn9 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn9)));

        let assign17860_e17700: f64 = (locals.var_k2 * locals.var_q_d2_q2);
        locals.var_q_d2_qi = assign17860_e17700;
        locals.var_q_d2_qi_dn4 = ((locals.var_k2_dn4 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn4));
        locals.var_q_d2_qi_dn6 = ((locals.var_k2_dn6 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn6));
        locals.var_q_d2_qi_dn7 = ((locals.var_k2_dn7 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn7));
        locals.var_q_d2_qi_dn8 = ((locals.var_k2_dn8 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn8));
        locals.var_q_d2_qi_dn9 = ((locals.var_k2_dn9 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn9));

        let assign17870_e17703: f64 = (locals.var_q_qi_int * locals.var_q_expnum);
        let assign17870_e17705: f64 = (assign17870_e17703 - locals.var_q_aexp);
        locals.var_q_zero = assign17870_e17705;
        locals.var_q_zero_dn4 = (((locals.var_q_qi_int_dn4 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4);
        locals.var_q_zero_dn6 = (((locals.var_q_qi_int_dn6 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6);
        locals.var_q_zero_dn7 = (((locals.var_q_qi_int_dn7 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7);
        locals.var_q_zero_dn8 = (((locals.var_q_qi_int_dn8 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8);
        locals.var_q_zero_dn9 = (((locals.var_q_qi_int_dn9 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9);

        let assign17880_e17708: f64 = (locals.var_q_d1_qi * locals.var_q_expnum);
        let assign17880_e17711: f64 = (locals.var_q_qi_int * locals.var_q_d1_expnum);
        let assign17880_e17712: f64 = (assign17880_e17708 + assign17880_e17711);
        let assign17880_e17714: f64 = (assign17880_e17712 + locals.var_q_aexp);
        locals.var_q_d1_zero = assign17880_e17714;
        locals.var_q_d1_zero_dn4 = ((((locals.var_q_d1_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn4)) + ((locals.var_q_qi_int_dn4 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4);
        locals.var_q_d1_zero_dn6 = ((((locals.var_q_d1_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn6)) + ((locals.var_q_qi_int_dn6 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6);
        locals.var_q_d1_zero_dn7 = ((((locals.var_q_d1_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn7)) + ((locals.var_q_qi_int_dn7 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7);
        locals.var_q_d1_zero_dn8 = ((((locals.var_q_d1_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn8)) + ((locals.var_q_qi_int_dn8 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8);
        locals.var_q_d1_zero_dn9 = ((((locals.var_q_d1_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn9)) + ((locals.var_q_qi_int_dn9 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9);

        let assign17890_e17717: f64 = (locals.var_q_d2_qi * locals.var_q_expnum);
        let assign17890_e17720: f64 = (2.0 * locals.var_q_d1_qi);
        let assign17890_e17722: f64 = (assign17890_e17720 * locals.var_q_d1_expnum);
        let assign17890_e17723: f64 = (assign17890_e17717 + assign17890_e17722);
        let assign17890_e17726: f64 = (locals.var_q_qi_int * locals.var_q_d2_expnum);
        let assign17890_e17727: f64 = (assign17890_e17723 + assign17890_e17726);
        let assign17890_e17729: f64 = (assign17890_e17727 - locals.var_q_aexp);
        locals.var_q_d2_zero = assign17890_e17729;
        locals.var_q_d2_zero_dn4 = (((((locals.var_q_d2_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_d1_qi_dn4) * locals.var_q_d1_expnum) + (assign17890_e17720 * locals.var_q_d1_expnum_dn4))) + ((locals.var_q_qi_int_dn4 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn4))) - locals.var_q_aexp_dn4);
        locals.var_q_d2_zero_dn6 = (((((locals.var_q_d2_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_d1_qi_dn6) * locals.var_q_d1_expnum) + (assign17890_e17720 * locals.var_q_d1_expnum_dn6))) + ((locals.var_q_qi_int_dn6 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn6))) - locals.var_q_aexp_dn6);
        locals.var_q_d2_zero_dn7 = (((((locals.var_q_d2_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_d1_qi_dn7) * locals.var_q_d1_expnum) + (assign17890_e17720 * locals.var_q_d1_expnum_dn7))) + ((locals.var_q_qi_int_dn7 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn7))) - locals.var_q_aexp_dn7);
        locals.var_q_d2_zero_dn8 = (((((locals.var_q_d2_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_d1_qi_dn8) * locals.var_q_d1_expnum) + (assign17890_e17720 * locals.var_q_d1_expnum_dn8))) + ((locals.var_q_qi_int_dn8 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn8))) - locals.var_q_aexp_dn8);
        locals.var_q_d2_zero_dn9 = (((((locals.var_q_d2_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_d1_qi_dn9) * locals.var_q_d1_expnum) + (assign17890_e17720 * locals.var_q_d1_expnum_dn9))) + ((locals.var_q_qi_int_dn9 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn9))) - locals.var_q_aexp_dn9);

        let assign17900_e17732: f64 = (locals.var_q_d1_zero * locals.var_q_d1_zero);
        let assign17900_e17735: f64 = (0.5 * locals.var_q_zero);
        let assign17900_e17737: f64 = (assign17900_e17735 * locals.var_q_d2_zero);
        let assign17900_e17738: f64 = (assign17900_e17732 - assign17900_e17737);
        locals.var_q_temp = assign17900_e17738;
        locals.var_q_temp_dn4 = (((locals.var_q_d1_zero_dn4 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn4)) - (((0.5 * locals.var_q_zero_dn4) * locals.var_q_d2_zero) + (assign17900_e17735 * locals.var_q_d2_zero_dn4)));
        locals.var_q_temp_dn6 = (((locals.var_q_d1_zero_dn6 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn6)) - (((0.5 * locals.var_q_zero_dn6) * locals.var_q_d2_zero) + (assign17900_e17735 * locals.var_q_d2_zero_dn6)));
        locals.var_q_temp_dn7 = (((locals.var_q_d1_zero_dn7 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn7)) - (((0.5 * locals.var_q_zero_dn7) * locals.var_q_d2_zero) + (assign17900_e17735 * locals.var_q_d2_zero_dn7)));
        locals.var_q_temp_dn8 = (((locals.var_q_d1_zero_dn8 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn8)) - (((0.5 * locals.var_q_zero_dn8) * locals.var_q_d2_zero) + (assign17900_e17735 * locals.var_q_d2_zero_dn8)));
        locals.var_q_temp_dn9 = (((locals.var_q_d1_zero_dn9 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn9)) - (((0.5 * locals.var_q_zero_dn9) * locals.var_q_d2_zero) + (assign17900_e17735 * locals.var_q_d2_zero_dn9)));

        let assign17910_e17740: f64 = (-locals.var_q_zero);
        let assign17910_e17742: f64 = (assign17910_e17740 * locals.var_q_d1_zero);
        let assign17910_e17744: f64 = (assign17910_e17742 * locals.var_q_temp);
        let assign17910_e17747: f64 = (locals.var_q_temp * locals.var_q_temp);
        let assign17910_e17749: f64 = (assign17910_e17747 + 1e-200);
        let assign17910_e17750: f64 = (assign17910_e17744 / assign17910_e17749);
        locals.var_q_eps2 = assign17910_e17750;
        locals.var_q_eps2_dn4 = ((((((((-locals.var_q_zero_dn4) * locals.var_q_d1_zero) + (assign17910_e17740 * locals.var_q_d1_zero_dn4)) * locals.var_q_temp) + (assign17910_e17742 * locals.var_q_temp_dn4)) * assign17910_e17749) - (assign17910_e17744 * ((locals.var_q_temp_dn4 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn4)))) / (assign17910_e17749 * assign17910_e17749));
        locals.var_q_eps2_dn6 = ((((((((-locals.var_q_zero_dn6) * locals.var_q_d1_zero) + (assign17910_e17740 * locals.var_q_d1_zero_dn6)) * locals.var_q_temp) + (assign17910_e17742 * locals.var_q_temp_dn6)) * assign17910_e17749) - (assign17910_e17744 * ((locals.var_q_temp_dn6 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn6)))) / (assign17910_e17749 * assign17910_e17749));
        locals.var_q_eps2_dn7 = ((((((((-locals.var_q_zero_dn7) * locals.var_q_d1_zero) + (assign17910_e17740 * locals.var_q_d1_zero_dn7)) * locals.var_q_temp) + (assign17910_e17742 * locals.var_q_temp_dn7)) * assign17910_e17749) - (assign17910_e17744 * ((locals.var_q_temp_dn7 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn7)))) / (assign17910_e17749 * assign17910_e17749));
        locals.var_q_eps2_dn8 = ((((((((-locals.var_q_zero_dn8) * locals.var_q_d1_zero) + (assign17910_e17740 * locals.var_q_d1_zero_dn8)) * locals.var_q_temp) + (assign17910_e17742 * locals.var_q_temp_dn8)) * assign17910_e17749) - (assign17910_e17744 * ((locals.var_q_temp_dn8 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn8)))) / (assign17910_e17749 * assign17910_e17749));
        locals.var_q_eps2_dn9 = ((((((((-locals.var_q_zero_dn9) * locals.var_q_d1_zero) + (assign17910_e17740 * locals.var_q_d1_zero_dn9)) * locals.var_q_temp) + (assign17910_e17742 * locals.var_q_temp_dn9)) * assign17910_e17749) - (assign17910_e17744 * ((locals.var_q_temp_dn9 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn9)))) / (assign17910_e17749 * assign17910_e17749));

        let assign17920_e17753: f64 = (locals.var_q1d + locals.var_q_eps2);
        locals.var_q1d = assign17920_e17753;
        locals.var_q1d_dn4 = (locals.var_q1d_dn4 + locals.var_q_eps2_dn4);
        locals.var_q1d_dn6 = (locals.var_q1d_dn6 + locals.var_q_eps2_dn6);
        locals.var_q1d_dn7 = (locals.var_q1d_dn7 + locals.var_q_eps2_dn7);
        locals.var_q1d_dn8 = (locals.var_q1d_dn8 + locals.var_q_eps2_dn8);
        locals.var_q1d_dn9 = (locals.var_q1d_dn9 + locals.var_q_eps2_dn9);

        let assign17930_e17756: f64 = (locals.var_k1 * locals.var_q1d);
        locals.var_q_k1q1 = assign17930_e17756;
        locals.var_q_k1q1_dn4 = ((locals.var_k1_dn4 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn4));
        locals.var_q_k1q1_dn6 = ((locals.var_k1_dn6 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn6));
        locals.var_q_k1q1_dn7 = ((locals.var_k1_dn7 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn7));
        locals.var_q_k1q1_dn8 = ((locals.var_k1_dn8 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn8));
        locals.var_q_k1q1_dn9 = ((locals.var_k1_dn9 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn9));

        let assign17940_e17759: f64 = (locals.var_k2 * locals.var_q2d);
        locals.var_q_k2q2 = assign17940_e17759;
        locals.var_q_k2q2_dn4 = ((locals.var_k2_dn4 * locals.var_q2d) + (locals.var_k2 * locals.var_q2d_dn4));
        locals.var_q_k2q2_dn6 = ((locals.var_k2_dn6 * locals.var_q2d) + (locals.var_k2 * locals.var_q2d_dn6));
        locals.var_q_k2q2_dn7 = ((locals.var_k2_dn7 * locals.var_q2d) + (locals.var_k2 * locals.var_q2d_dn7));
        locals.var_q_k2q2_dn8 = ((locals.var_k2_dn8 * locals.var_q2d) + (locals.var_k2 * locals.var_q2d_dn8));
        locals.var_q_k2q2_dn9 = ((locals.var_k2_dn9 * locals.var_q2d) + (locals.var_k2 * locals.var_q2d_dn9));

        let assign17950_e17762: f64 = (locals.var_q_k1q1 + locals.var_q_k2q2);
        locals.var_q_qi_int = assign17950_e17762;
        locals.var_q_qi_int_dn4 = (locals.var_q_k1q1_dn4 + locals.var_q_k2q2_dn4);
        locals.var_q_qi_int_dn6 = (locals.var_q_k1q1_dn6 + locals.var_q_k2q2_dn6);
        locals.var_q_qi_int_dn7 = (locals.var_q_k1q1_dn7 + locals.var_q_k2q2_dn7);
        locals.var_q_qi_int_dn8 = (locals.var_q_k1q1_dn8 + locals.var_q_k2q2_dn8);
        locals.var_q_qi_int_dn9 = (locals.var_q_k1q1_dn9 + locals.var_q_k2q2_dn9);

        let assign17960_e17766: f64 = (0.065345483024 * locals.var_q_qi_int);
        let assign17960_e17767: f64 = (1.0 + assign17960_e17766);
        locals.var_q_a = assign17960_e17767;
        locals.var_q_a_dn4 = (0.065345483024 * locals.var_q_qi_int_dn4);
        locals.var_q_a_dn6 = (0.065345483024 * locals.var_q_qi_int_dn6);
        locals.var_q_a_dn7 = (0.065345483024 * locals.var_q_qi_int_dn7);
        locals.var_q_a_dn8 = (0.065345483024 * locals.var_q_qi_int_dn8);
        locals.var_q_a_dn9 = (0.065345483024 * locals.var_q_qi_int_dn9);

        let assign17970_e17771: f64 = (8.5797362674 * locals.var_q_qi_int);
        let assign17970_e17772: f64 = (39.478417604 + assign17970_e17771);
        let assign17970_e17775: f64 = (locals.var_q_k1q1 * locals.var_q_k2q2);
        let assign17970_e17776: f64 = (assign17970_e17772 + assign17970_e17775);
        locals.var_q_b = assign17970_e17776;
        locals.var_q_b_dn4 = ((8.5797362674 * locals.var_q_qi_int_dn4) + ((locals.var_q_k1q1_dn4 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn4)));
        locals.var_q_b_dn6 = ((8.5797362674 * locals.var_q_qi_int_dn6) + ((locals.var_q_k1q1_dn6 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn6)));
        locals.var_q_b_dn7 = ((8.5797362674 * locals.var_q_qi_int_dn7) + ((locals.var_q_k1q1_dn7 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn7)));
        locals.var_q_b_dn8 = ((8.5797362674 * locals.var_q_qi_int_dn8) + ((locals.var_q_k1q1_dn8 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn8)));
        locals.var_q_b_dn9 = ((8.5797362674 * locals.var_q_qi_int_dn9) + ((locals.var_q_k1q1_dn9 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn9)));

        let assign17980_e17780: f64 = (2.0 * locals.var_q_qi_int);
        let assign17980_e17783: f64 = (locals.var_q_k1q1 * locals.var_q_k2q2);
        let assign17980_e17784: f64 = (assign17980_e17780 + assign17980_e17783);
        let assign17980_e17785: f64 = (39.478417604 * assign17980_e17784);
        locals.var_q_c = assign17980_e17785;
        locals.var_q_c_dn4 = (39.478417604 * ((2.0 * locals.var_q_qi_int_dn4) + ((locals.var_q_k1q1_dn4 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn4))));
        locals.var_q_c_dn6 = (39.478417604 * ((2.0 * locals.var_q_qi_int_dn6) + ((locals.var_q_k1q1_dn6 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn6))));
        locals.var_q_c_dn7 = (39.478417604 * ((2.0 * locals.var_q_qi_int_dn7) + ((locals.var_q_k1q1_dn7 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn7))));
        locals.var_q_c_dn8 = (39.478417604 * ((2.0 * locals.var_q_qi_int_dn8) + ((locals.var_q_k1q1_dn8 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn8))));
        locals.var_q_c_dn9 = (39.478417604 * ((2.0 * locals.var_q_qi_int_dn9) + ((locals.var_q_k1q1_dn9 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn9))));

        let assign17990_e17788: f64 = (locals.var_q_b * locals.var_q_b);
        let assign17990_e17791: f64 = (4.0 * locals.var_q_a);
        let assign17990_e17793: f64 = (assign17990_e17791 * locals.var_q_c);
        let assign17990_e17794: f64 = (assign17990_e17788 - assign17990_e17793);
        let assign17990_e17795: f64 = (assign17990_e17794).sqrt();
        locals.var_q_disc = assign17990_e17795;
        locals.var_q_disc_dn4 = ((((locals.var_q_b_dn4 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn4)) - (((4.0 * locals.var_q_a_dn4) * locals.var_q_c) + (assign17990_e17791 * locals.var_q_c_dn4))) / (2.0 * assign17990_e17795));
        locals.var_q_disc_dn6 = ((((locals.var_q_b_dn6 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn6)) - (((4.0 * locals.var_q_a_dn6) * locals.var_q_c) + (assign17990_e17791 * locals.var_q_c_dn6))) / (2.0 * assign17990_e17795));
        locals.var_q_disc_dn7 = ((((locals.var_q_b_dn7 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn7)) - (((4.0 * locals.var_q_a_dn7) * locals.var_q_c) + (assign17990_e17791 * locals.var_q_c_dn7))) / (2.0 * assign17990_e17795));
        locals.var_q_disc_dn8 = ((((locals.var_q_b_dn8 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn8)) - (((4.0 * locals.var_q_a_dn8) * locals.var_q_c) + (assign17990_e17791 * locals.var_q_c_dn8))) / (2.0 * assign17990_e17795));
        locals.var_q_disc_dn9 = ((((locals.var_q_b_dn9 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn9)) - (((4.0 * locals.var_q_a_dn9) * locals.var_q_c) + (assign17990_e17791 * locals.var_q_c_dn9))) / (2.0 * assign17990_e17795));

        let assign18000_e17798: f64 = (locals.var_q_disc - locals.var_q_b);
        let assign18000_e17801: f64 = (2.0 * locals.var_q_a);
        let assign18000_e17802: f64 = (assign18000_e17798 / assign18000_e17801);
        locals.var_q_qsq = assign18000_e17802;
        locals.var_q_qsq_dn4 = ((((locals.var_q_disc_dn4 - locals.var_q_b_dn4) * assign18000_e17801) - (assign18000_e17798 * (2.0 * locals.var_q_a_dn4))) / (assign18000_e17801 * assign18000_e17801));
        locals.var_q_qsq_dn6 = ((((locals.var_q_disc_dn6 - locals.var_q_b_dn6) * assign18000_e17801) - (assign18000_e17798 * (2.0 * locals.var_q_a_dn6))) / (assign18000_e17801 * assign18000_e17801));
        locals.var_q_qsq_dn7 = ((((locals.var_q_disc_dn7 - locals.var_q_b_dn7) * assign18000_e17801) - (assign18000_e17798 * (2.0 * locals.var_q_a_dn7))) / (assign18000_e17801 * assign18000_e17801));
        locals.var_q_qsq_dn8 = ((((locals.var_q_disc_dn8 - locals.var_q_b_dn8) * assign18000_e17801) - (assign18000_e17798 * (2.0 * locals.var_q_a_dn8))) / (assign18000_e17801 * assign18000_e17801));
        locals.var_q_qsq_dn9 = ((((locals.var_q_disc_dn9 - locals.var_q_b_dn9) * assign18000_e17801) - (assign18000_e17798 * (2.0 * locals.var_q_a_dn9))) / (assign18000_e17801 * assign18000_e17801));

        let assign18010_e17805: f64 = (locals.var_q_k1q1 * locals.var_q_k1q1);
        let assign18010_e17807: f64 = (assign18010_e17805 - locals.var_q_qsq);
        locals.var_q_delta = assign18010_e17807;
        locals.var_q_delta_dn4 = (((locals.var_q_k1q1_dn4 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn4)) - locals.var_q_qsq_dn4);
        locals.var_q_delta_dn6 = (((locals.var_q_k1q1_dn6 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn6)) - locals.var_q_qsq_dn6);
        locals.var_q_delta_dn7 = (((locals.var_q_k1q1_dn7 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn7)) - locals.var_q_qsq_dn7);
        locals.var_q_delta_dn8 = (((locals.var_q_k1q1_dn8 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn8)) - locals.var_q_qsq_dn8);
        locals.var_q_delta_dn9 = (((locals.var_q_k1q1_dn9 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn9)) - locals.var_q_qsq_dn9);

        let assign18020_e17810: f64 = if locals.var_q_delta > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard626 = assign18020_e17810;

    }

    pub(super) fn stamp_transient_block_45(
        locals: &mut StampLocals,
    ) {
        let (assign18030_e17825, assign18030_e17825_d_n4, assign18030_e17825_d_n6, assign18030_e17825_d_n7, assign18030_e17825_d_n8, assign18030_e17825_d_n9,) = {
    if (locals.var_guard626 != 0.0) {
        let assign18030_e17815: f64 = (locals.var_q_delta / locals.var_a0);
        let assign18030_e17816: f64 = (assign18030_e17815).ln();
        let assign18030_e17818: f64 = (assign18030_e17816 + locals.var_xdeff);
        let assign18030_e17820: f64 = (assign18030_e17818 - locals.var_xg1x);
        let assign18030_e17822: f64 = (assign18030_e17820 + locals.var_q1d);
        let assign18030_e17823: f64 = (locals.var_q_delta * assign18030_e17822);
        (assign18030_e17823, ((locals.var_q_delta_dn4 * assign18030_e17822) + (locals.var_q_delta * (((((((locals.var_q_delta_dn4 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn4)) / (locals.var_a0 * locals.var_a0)) / assign18030_e17815) + locals.var_xdeff_dn4) - locals.var_xg1x_dn4) + locals.var_q1d_dn4))), ((locals.var_q_delta_dn6 * assign18030_e17822) + (locals.var_q_delta * (((((((locals.var_q_delta_dn6 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn6)) / (locals.var_a0 * locals.var_a0)) / assign18030_e17815) + locals.var_xdeff_dn6) - locals.var_xg1x_dn6) + locals.var_q1d_dn6))), ((locals.var_q_delta_dn7 * assign18030_e17822) + (locals.var_q_delta * (((((((locals.var_q_delta_dn7 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn7)) / (locals.var_a0 * locals.var_a0)) / assign18030_e17815) + locals.var_xdeff_dn7) - locals.var_xg1x_dn7) + locals.var_q1d_dn7))), ((locals.var_q_delta_dn8 * assign18030_e17822) + (locals.var_q_delta * (((((((locals.var_q_delta_dn8 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn8)) / (locals.var_a0 * locals.var_a0)) / assign18030_e17815) + locals.var_xdeff_dn8) - locals.var_xg1x_dn8) + locals.var_q1d_dn8))), ((locals.var_q_delta_dn9 * assign18030_e17822) + (locals.var_q_delta * (((((((locals.var_q_delta_dn9 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn9)) / (locals.var_a0 * locals.var_a0)) / assign18030_e17815) + locals.var_xdeff_dn9) - locals.var_xg1x_dn9) + locals.var_q1d_dn9))),)
    } else {
        (locals.var_q_zero, locals.var_q_zero_dn4, locals.var_q_zero_dn6, locals.var_q_zero_dn7, locals.var_q_zero_dn8, locals.var_q_zero_dn9,)
    }
};
        locals.var_q_zero = assign18030_e17825;
        locals.var_q_zero_dn4 = assign18030_e17825_d_n4;
        locals.var_q_zero_dn6 = assign18030_e17825_d_n6;
        locals.var_q_zero_dn7 = assign18030_e17825_d_n7;
        locals.var_q_zero_dn8 = assign18030_e17825_d_n8;
        locals.var_q_zero_dn9 = assign18030_e17825_d_n9;

        let (assign18040_e17835, assign18040_e17835_d_n4, assign18040_e17835_d_n6, assign18040_e17835_d_n7, assign18040_e17835_d_n8, assign18040_e17835_d_n9,) = {
    if (locals.var_guard626 != 0.0) {
        let assign18040_e17829: f64 = (2.0 * locals.var_k1);
        let assign18040_e17831: f64 = (assign18040_e17829 * locals.var_q_k1q1);
        let assign18040_e17833: f64 = (assign18040_e17831 + locals.var_q_delta);
        (assign18040_e17833, ((((2.0 * locals.var_k1_dn4) * locals.var_q_k1q1) + (assign18040_e17829 * locals.var_q_k1q1_dn4)) + locals.var_q_delta_dn4), ((((2.0 * locals.var_k1_dn6) * locals.var_q_k1q1) + (assign18040_e17829 * locals.var_q_k1q1_dn6)) + locals.var_q_delta_dn6), ((((2.0 * locals.var_k1_dn7) * locals.var_q_k1q1) + (assign18040_e17829 * locals.var_q_k1q1_dn7)) + locals.var_q_delta_dn7), ((((2.0 * locals.var_k1_dn8) * locals.var_q_k1q1) + (assign18040_e17829 * locals.var_q_k1q1_dn8)) + locals.var_q_delta_dn8), ((((2.0 * locals.var_k1_dn9) * locals.var_q_k1q1) + (assign18040_e17829 * locals.var_q_k1q1_dn9)) + locals.var_q_delta_dn9),)
    } else {
        (locals.var_q_d1_zero, locals.var_q_d1_zero_dn4, locals.var_q_d1_zero_dn6, locals.var_q_d1_zero_dn7, locals.var_q_d1_zero_dn8, locals.var_q_d1_zero_dn9,)
    }
};
        locals.var_q_d1_zero = assign18040_e17835;
        locals.var_q_d1_zero_dn4 = assign18040_e17835_d_n4;
        locals.var_q_d1_zero_dn6 = assign18040_e17835_d_n6;
        locals.var_q_d1_zero_dn7 = assign18040_e17835_d_n7;
        locals.var_q_d1_zero_dn8 = assign18040_e17835_d_n8;
        locals.var_q_d1_zero_dn9 = assign18040_e17835_d_n9;

        let (assign18050_e17843,) = {
    if (locals.var_guard626 != 0.0) {
        let assign18050_e17839: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign18050_e17841: f64 = (assign18050_e17839 - locals.var_q_x1sat);
        (assign18050_e17841,)
    } else {
        (locals.var_q_dx1,)
    }
};
        locals.var_q_dx1 = assign18050_e17843;

        let assign18060_e17853: f64 = (locals.var_q_dx1 + 2.3025850929941);
        let assign18060_e17855: f64 = (locals.var_k1).ln();
        let assign18060_e17856: f64 = (assign18060_e17853 + assign18060_e17855);
        let assign18060_e17863: f64 = if ((((locals.var_q_zero < 0.0) && (locals.var_q_d1_zero > 0.0)) && (assign18060_e17856 > 0.0)) || (locals.var_q_dx1 > 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard627 = assign18060_e17863;

        let (assign18070_e17873, assign18070_e17873_d_n4, assign18070_e17873_d_n6, assign18070_e17873_d_n7, assign18070_e17873_d_n8, assign18070_e17873_d_n9,) = {
    if ((locals.var_guard626 != 0.0) && (locals.var_guard627 != 0.0)) {
        let assign18070_e17870: f64 = (locals.var_q_zero / locals.var_q_d1_zero);
        let assign18070_e17871: f64 = (locals.var_q1d - assign18070_e17870);
        (assign18070_e17871, (locals.var_q1d_dn4 - (((locals.var_q_zero_dn4 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn4)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1d_dn6 - (((locals.var_q_zero_dn6 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn6)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1d_dn7 - (((locals.var_q_zero_dn7 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn7)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1d_dn8 - (((locals.var_q_zero_dn8 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn8)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1d_dn9 - (((locals.var_q_zero_dn9 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn9)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))),)
    } else {
        (locals.var_q1d, locals.var_q1d_dn4, locals.var_q1d_dn6, locals.var_q1d_dn7, locals.var_q1d_dn8, locals.var_q1d_dn9,)
    }
};
        locals.var_q1d = assign18070_e17873;
        locals.var_q1d_dn4 = assign18070_e17873_d_n4;
        locals.var_q1d_dn6 = assign18070_e17873_d_n6;
        locals.var_q1d_dn7 = assign18070_e17873_d_n7;
        locals.var_q1d_dn8 = assign18070_e17873_d_n8;
        locals.var_q1d_dn9 = assign18070_e17873_d_n9;

        let assign18080_e17876: f64 = (locals.var_k1 * locals.var_q1d);
        locals.var_q_k1q1 = assign18080_e17876;
        locals.var_q_k1q1_dn4 = ((locals.var_k1_dn4 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn4));
        locals.var_q_k1q1_dn6 = ((locals.var_k1_dn6 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn6));
        locals.var_q_k1q1_dn7 = ((locals.var_k1_dn7 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn7));
        locals.var_q_k1q1_dn8 = ((locals.var_k1_dn8 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn8));
        locals.var_q_k1q1_dn9 = ((locals.var_k1_dn9 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn9));

        let assign18090_e17879: f64 = (locals.var_k2 * locals.var_q2d);
        locals.var_q_k2q2 = assign18090_e17879;
        locals.var_q_k2q2_dn4 = ((locals.var_k2_dn4 * locals.var_q2d) + (locals.var_k2 * locals.var_q2d_dn4));
        locals.var_q_k2q2_dn6 = ((locals.var_k2_dn6 * locals.var_q2d) + (locals.var_k2 * locals.var_q2d_dn6));
        locals.var_q_k2q2_dn7 = ((locals.var_k2_dn7 * locals.var_q2d) + (locals.var_k2 * locals.var_q2d_dn7));
        locals.var_q_k2q2_dn8 = ((locals.var_k2_dn8 * locals.var_q2d) + (locals.var_k2 * locals.var_q2d_dn8));
        locals.var_q_k2q2_dn9 = ((locals.var_k2_dn9 * locals.var_q2d) + (locals.var_k2 * locals.var_q2d_dn9));

        let assign18100_e17882: f64 = (locals.var_q_k1q1 + locals.var_q_k2q2);
        locals.var_q_qi_int = assign18100_e17882;
        locals.var_q_qi_int_dn4 = (locals.var_q_k1q1_dn4 + locals.var_q_k2q2_dn4);
        locals.var_q_qi_int_dn6 = (locals.var_q_k1q1_dn6 + locals.var_q_k2q2_dn6);
        locals.var_q_qi_int_dn7 = (locals.var_q_k1q1_dn7 + locals.var_q_k2q2_dn7);
        locals.var_q_qi_int_dn8 = (locals.var_q_k1q1_dn8 + locals.var_q_k2q2_dn8);
        locals.var_q_qi_int_dn9 = (locals.var_q_k1q1_dn9 + locals.var_q_k2q2_dn9);

        let assign18110_e17886: f64 = (0.065345483024 * locals.var_q_qi_int);
        let assign18110_e17887: f64 = (1.0 + assign18110_e17886);
        locals.var_q_a = assign18110_e17887;
        locals.var_q_a_dn4 = (0.065345483024 * locals.var_q_qi_int_dn4);
        locals.var_q_a_dn6 = (0.065345483024 * locals.var_q_qi_int_dn6);
        locals.var_q_a_dn7 = (0.065345483024 * locals.var_q_qi_int_dn7);
        locals.var_q_a_dn8 = (0.065345483024 * locals.var_q_qi_int_dn8);
        locals.var_q_a_dn9 = (0.065345483024 * locals.var_q_qi_int_dn9);

        let assign18120_e17891: f64 = (8.5797362674 * locals.var_q_qi_int);
        let assign18120_e17892: f64 = (39.478417604 + assign18120_e17891);
        let assign18120_e17895: f64 = (locals.var_q_k1q1 * locals.var_q_k2q2);
        let assign18120_e17896: f64 = (assign18120_e17892 + assign18120_e17895);
        locals.var_q_b = assign18120_e17896;
        locals.var_q_b_dn4 = ((8.5797362674 * locals.var_q_qi_int_dn4) + ((locals.var_q_k1q1_dn4 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn4)));
        locals.var_q_b_dn6 = ((8.5797362674 * locals.var_q_qi_int_dn6) + ((locals.var_q_k1q1_dn6 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn6)));
        locals.var_q_b_dn7 = ((8.5797362674 * locals.var_q_qi_int_dn7) + ((locals.var_q_k1q1_dn7 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn7)));
        locals.var_q_b_dn8 = ((8.5797362674 * locals.var_q_qi_int_dn8) + ((locals.var_q_k1q1_dn8 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn8)));
        locals.var_q_b_dn9 = ((8.5797362674 * locals.var_q_qi_int_dn9) + ((locals.var_q_k1q1_dn9 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn9)));

        let assign18130_e17900: f64 = (2.0 * locals.var_q_qi_int);
        let assign18130_e17903: f64 = (locals.var_q_k1q1 * locals.var_q_k2q2);
        let assign18130_e17904: f64 = (assign18130_e17900 + assign18130_e17903);
        let assign18130_e17905: f64 = (39.478417604 * assign18130_e17904);
        locals.var_q_c = assign18130_e17905;
        locals.var_q_c_dn4 = (39.478417604 * ((2.0 * locals.var_q_qi_int_dn4) + ((locals.var_q_k1q1_dn4 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn4))));
        locals.var_q_c_dn6 = (39.478417604 * ((2.0 * locals.var_q_qi_int_dn6) + ((locals.var_q_k1q1_dn6 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn6))));
        locals.var_q_c_dn7 = (39.478417604 * ((2.0 * locals.var_q_qi_int_dn7) + ((locals.var_q_k1q1_dn7 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn7))));
        locals.var_q_c_dn8 = (39.478417604 * ((2.0 * locals.var_q_qi_int_dn8) + ((locals.var_q_k1q1_dn8 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn8))));
        locals.var_q_c_dn9 = (39.478417604 * ((2.0 * locals.var_q_qi_int_dn9) + ((locals.var_q_k1q1_dn9 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn9))));

        let assign18140_e17908: f64 = (locals.var_q_b * locals.var_q_b);
        let assign18140_e17911: f64 = (4.0 * locals.var_q_a);
        let assign18140_e17913: f64 = (assign18140_e17911 * locals.var_q_c);
        let assign18140_e17914: f64 = (assign18140_e17908 - assign18140_e17913);
        let assign18140_e17915: f64 = (assign18140_e17914).sqrt();
        locals.var_q_disc = assign18140_e17915;
        locals.var_q_disc_dn4 = ((((locals.var_q_b_dn4 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn4)) - (((4.0 * locals.var_q_a_dn4) * locals.var_q_c) + (assign18140_e17911 * locals.var_q_c_dn4))) / (2.0 * assign18140_e17915));
        locals.var_q_disc_dn6 = ((((locals.var_q_b_dn6 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn6)) - (((4.0 * locals.var_q_a_dn6) * locals.var_q_c) + (assign18140_e17911 * locals.var_q_c_dn6))) / (2.0 * assign18140_e17915));
        locals.var_q_disc_dn7 = ((((locals.var_q_b_dn7 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn7)) - (((4.0 * locals.var_q_a_dn7) * locals.var_q_c) + (assign18140_e17911 * locals.var_q_c_dn7))) / (2.0 * assign18140_e17915));
        locals.var_q_disc_dn8 = ((((locals.var_q_b_dn8 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn8)) - (((4.0 * locals.var_q_a_dn8) * locals.var_q_c) + (assign18140_e17911 * locals.var_q_c_dn8))) / (2.0 * assign18140_e17915));
        locals.var_q_disc_dn9 = ((((locals.var_q_b_dn9 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn9)) - (((4.0 * locals.var_q_a_dn9) * locals.var_q_c) + (assign18140_e17911 * locals.var_q_c_dn9))) / (2.0 * assign18140_e17915));

        let assign18150_e17918: f64 = (locals.var_q_disc - locals.var_q_b);
        let assign18150_e17921: f64 = (2.0 * locals.var_q_a);
        let assign18150_e17922: f64 = (assign18150_e17918 / assign18150_e17921);
        locals.var_q_qsq = assign18150_e17922;
        locals.var_q_qsq_dn4 = ((((locals.var_q_disc_dn4 - locals.var_q_b_dn4) * assign18150_e17921) - (assign18150_e17918 * (2.0 * locals.var_q_a_dn4))) / (assign18150_e17921 * assign18150_e17921));
        locals.var_q_qsq_dn6 = ((((locals.var_q_disc_dn6 - locals.var_q_b_dn6) * assign18150_e17921) - (assign18150_e17918 * (2.0 * locals.var_q_a_dn6))) / (assign18150_e17921 * assign18150_e17921));
        locals.var_q_qsq_dn7 = ((((locals.var_q_disc_dn7 - locals.var_q_b_dn7) * assign18150_e17921) - (assign18150_e17918 * (2.0 * locals.var_q_a_dn7))) / (assign18150_e17921 * assign18150_e17921));
        locals.var_q_qsq_dn8 = ((((locals.var_q_disc_dn8 - locals.var_q_b_dn8) * assign18150_e17921) - (assign18150_e17918 * (2.0 * locals.var_q_a_dn8))) / (assign18150_e17921 * assign18150_e17921));
        locals.var_q_qsq_dn9 = ((((locals.var_q_disc_dn9 - locals.var_q_b_dn9) * assign18150_e17921) - (assign18150_e17918 * (2.0 * locals.var_q_a_dn9))) / (assign18150_e17921 * assign18150_e17921));

        let assign18160_e17925: f64 = (-0.005);
        let assign18160_e17926: f64 = if locals.var_q_qsq < assign18160_e17925 { 1.0 } else { 0.0 };
        locals.var_guard628 = assign18160_e17926;

        let (assign18170_e17932, assign18170_e17932_d_n4, assign18170_e17932_d_n6, assign18170_e17932_d_n7, assign18170_e17932_d_n8, assign18170_e17932_d_n9,) = {
    if (locals.var_guard628 != 0.0) {
        let assign18170_e17929: f64 = (locals.var_q_qsq).abs();
        let assign18170_e17930: f64 = (assign18170_e17929).sqrt();
        (assign18170_e17930, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign18170_e17930)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign18170_e17930)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign18170_e17930)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign18170_e17930)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign18170_e17930)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign18170_e17932;
        locals.var_q_rac_qsq_dn4 = assign18170_e17932_d_n4;
        locals.var_q_rac_qsq_dn6 = assign18170_e17932_d_n6;
        locals.var_q_rac_qsq_dn7 = assign18170_e17932_d_n7;
        locals.var_q_rac_qsq_dn8 = assign18170_e17932_d_n8;
        locals.var_q_rac_qsq_dn9 = assign18170_e17932_d_n9;

        let (assign18180_e17941, assign18180_e17941_d_n4, assign18180_e17941_d_n6, assign18180_e17941_d_n7, assign18180_e17941_d_n8, assign18180_e17941_d_n9,) = {
    if (locals.var_guard628 != 0.0) {
        let assign18180_e17937: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign18180_e17938: f64 = (assign18180_e17937).tan();
        let assign18180_e17939: f64 = (locals.var_q_rac_qsq / assign18180_e17938);
        (assign18180_e17939, (((locals.var_q_rac_qsq_dn4 * assign18180_e17938) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn4) / ((assign18180_e17937).cos() * (assign18180_e17937).cos())))) / (assign18180_e17938 * assign18180_e17938)), (((locals.var_q_rac_qsq_dn6 * assign18180_e17938) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn6) / ((assign18180_e17937).cos() * (assign18180_e17937).cos())))) / (assign18180_e17938 * assign18180_e17938)), (((locals.var_q_rac_qsq_dn7 * assign18180_e17938) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn7) / ((assign18180_e17937).cos() * (assign18180_e17937).cos())))) / (assign18180_e17938 * assign18180_e17938)), (((locals.var_q_rac_qsq_dn8 * assign18180_e17938) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn8) / ((assign18180_e17937).cos() * (assign18180_e17937).cos())))) / (assign18180_e17938 * assign18180_e17938)), (((locals.var_q_rac_qsq_dn9 * assign18180_e17938) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn9) / ((assign18180_e17937).cos() * (assign18180_e17937).cos())))) / (assign18180_e17938 * assign18180_e17938)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign18180_e17941;
        locals.var_q_qcoth_dn4 = assign18180_e17941_d_n4;
        locals.var_q_qcoth_dn6 = assign18180_e17941_d_n6;
        locals.var_q_qcoth_dn7 = assign18180_e17941_d_n7;
        locals.var_q_qcoth_dn8 = assign18180_e17941_d_n8;
        locals.var_q_qcoth_dn9 = assign18180_e17941_d_n9;

        let (assign18190_e17955, assign18190_e17955_d_n4, assign18190_e17955_d_n6, assign18190_e17955_d_n7, assign18190_e17955_d_n8, assign18190_e17955_d_n9,) = {
    if (locals.var_guard628 != 0.0) {
        let assign18190_e17948: f64 = (2.0 - locals.var_q_qcoth);
        let assign18190_e17949: f64 = (locals.var_q_qcoth * assign18190_e17948);
        let assign18190_e17950: f64 = (locals.var_q_qsq + assign18190_e17949);
        let assign18190_e17951: f64 = (0.25 * assign18190_e17950);
        let assign18190_e17953: f64 = (assign18190_e17951 / locals.var_q_qsq);
        (assign18190_e17953, ((((0.25 * (locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign18190_e17948) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4))))) * locals.var_q_qsq) - (assign18190_e17951 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign18190_e17948) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6))))) * locals.var_q_qsq) - (assign18190_e17951 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign18190_e17948) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7))))) * locals.var_q_qsq) - (assign18190_e17951 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign18190_e17948) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8))))) * locals.var_q_qsq) - (assign18190_e17951 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign18190_e17948) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9))))) * locals.var_q_qsq) - (assign18190_e17951 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign18190_e17955;
        locals.var_q_d1_qcoth_dn4 = assign18190_e17955_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign18190_e17955_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign18190_e17955_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign18190_e17955_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign18190_e17955_d_n9;

        let assign18200_e17958: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard629 = assign18200_e17958;

        let (assign18210_e17967, assign18210_e17967_d_n4, assign18210_e17967_d_n6, assign18210_e17967_d_n7, assign18210_e17967_d_n8, assign18210_e17967_d_n9,) = {
    if ((locals.var_guard628 == 0.0) && (locals.var_guard629 != 0.0)) {
        let assign18210_e17964: f64 = (locals.var_q_qsq).abs();
        let assign18210_e17965: f64 = (assign18210_e17964).sqrt();
        (assign18210_e17965, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign18210_e17965)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign18210_e17965)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign18210_e17965)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign18210_e17965)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign18210_e17965)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign18210_e17967;
        locals.var_q_rac_qsq_dn4 = assign18210_e17967_d_n4;
        locals.var_q_rac_qsq_dn6 = assign18210_e17967_d_n6;
        locals.var_q_rac_qsq_dn7 = assign18210_e17967_d_n7;
        locals.var_q_rac_qsq_dn8 = assign18210_e17967_d_n8;
        locals.var_q_rac_qsq_dn9 = assign18210_e17967_d_n9;

        let (assign18220_e17976, assign18220_e17976_d_n4, assign18220_e17976_d_n6, assign18220_e17976_d_n7, assign18220_e17976_d_n8, assign18220_e17976_d_n9,) = {
    if ((locals.var_guard628 == 0.0) && (locals.var_guard629 != 0.0)) {
        let assign18220_e17973: f64 = (-locals.var_q_rac_qsq);
        let assign18220_e17974: f64 = (assign18220_e17973).exp();
        (assign18220_e17974, (assign18220_e17974 * (-locals.var_q_rac_qsq_dn4)), (assign18220_e17974 * (-locals.var_q_rac_qsq_dn6)), (assign18220_e17974 * (-locals.var_q_rac_qsq_dn7)), (assign18220_e17974 * (-locals.var_q_rac_qsq_dn8)), (assign18220_e17974 * (-locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_invexpq, locals.var_q_invexpq_dn4, locals.var_q_invexpq_dn6, locals.var_q_invexpq_dn7, locals.var_q_invexpq_dn8, locals.var_q_invexpq_dn9,)
    }
};
        locals.var_q_invexpq = assign18220_e17976;
        locals.var_q_invexpq_dn4 = assign18220_e17976_d_n4;
        locals.var_q_invexpq_dn6 = assign18220_e17976_d_n6;
        locals.var_q_invexpq_dn7 = assign18220_e17976_d_n7;
        locals.var_q_invexpq_dn8 = assign18220_e17976_d_n8;
        locals.var_q_invexpq_dn9 = assign18220_e17976_d_n9;

        let (assign18230_e17991, assign18230_e17991_d_n4, assign18230_e17991_d_n6, assign18230_e17991_d_n7, assign18230_e17991_d_n8, assign18230_e17991_d_n9,) = {
    if ((locals.var_guard628 == 0.0) && (locals.var_guard629 != 0.0)) {
        let assign18230_e17984: f64 = (1.0 + locals.var_q_invexpq);
        let assign18230_e17985: f64 = (locals.var_q_rac_qsq * assign18230_e17984);
        let assign18230_e17988: f64 = (1.0 - locals.var_q_invexpq);
        let assign18230_e17989: f64 = (assign18230_e17985 / assign18230_e17988);
        (assign18230_e17989, (((((locals.var_q_rac_qsq_dn4 * assign18230_e17984) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn4)) * assign18230_e17988) - (assign18230_e17985 * (-locals.var_q_invexpq_dn4))) / (assign18230_e17988 * assign18230_e17988)), (((((locals.var_q_rac_qsq_dn6 * assign18230_e17984) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn6)) * assign18230_e17988) - (assign18230_e17985 * (-locals.var_q_invexpq_dn6))) / (assign18230_e17988 * assign18230_e17988)), (((((locals.var_q_rac_qsq_dn7 * assign18230_e17984) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn7)) * assign18230_e17988) - (assign18230_e17985 * (-locals.var_q_invexpq_dn7))) / (assign18230_e17988 * assign18230_e17988)), (((((locals.var_q_rac_qsq_dn8 * assign18230_e17984) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn8)) * assign18230_e17988) - (assign18230_e17985 * (-locals.var_q_invexpq_dn8))) / (assign18230_e17988 * assign18230_e17988)), (((((locals.var_q_rac_qsq_dn9 * assign18230_e17984) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn9)) * assign18230_e17988) - (assign18230_e17985 * (-locals.var_q_invexpq_dn9))) / (assign18230_e17988 * assign18230_e17988)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign18230_e17991;
        locals.var_q_qcoth_dn4 = assign18230_e17991_d_n4;
        locals.var_q_qcoth_dn6 = assign18230_e17991_d_n6;
        locals.var_q_qcoth_dn7 = assign18230_e17991_d_n7;
        locals.var_q_qcoth_dn8 = assign18230_e17991_d_n8;
        locals.var_q_qcoth_dn9 = assign18230_e17991_d_n9;

        let (assign18240_e18008, assign18240_e18008_d_n4, assign18240_e18008_d_n6, assign18240_e18008_d_n7, assign18240_e18008_d_n8, assign18240_e18008_d_n9,) = {
    if ((locals.var_guard628 == 0.0) && (locals.var_guard629 != 0.0)) {
        let assign18240_e18001: f64 = (2.0 - locals.var_q_qcoth);
        let assign18240_e18002: f64 = (locals.var_q_qcoth * assign18240_e18001);
        let assign18240_e18003: f64 = (locals.var_q_qsq + assign18240_e18002);
        let assign18240_e18004: f64 = (0.25 * assign18240_e18003);
        let assign18240_e18006: f64 = (assign18240_e18004 / locals.var_q_qsq);
        (assign18240_e18006, ((((0.25 * (locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign18240_e18001) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4))))) * locals.var_q_qsq) - (assign18240_e18004 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign18240_e18001) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6))))) * locals.var_q_qsq) - (assign18240_e18004 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign18240_e18001) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7))))) * locals.var_q_qsq) - (assign18240_e18004 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign18240_e18001) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8))))) * locals.var_q_qsq) - (assign18240_e18004 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign18240_e18001) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9))))) * locals.var_q_qsq) - (assign18240_e18004 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign18240_e18008;
        locals.var_q_d1_qcoth_dn4 = assign18240_e18008_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign18240_e18008_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign18240_e18008_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign18240_e18008_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign18240_e18008_d_n9;

        let (assign18250_e18032, assign18250_e18032_d_n4, assign18250_e18032_d_n6, assign18250_e18032_d_n7, assign18250_e18032_d_n8, assign18250_e18032_d_n9,) = {
    if ((locals.var_guard628 == 0.0) && (locals.var_guard629 == 0.0)) {
        let assign18250_e18017: f64 = (locals.var_q_qsq * 0.1666666666667);
        let assign18250_e18021: f64 = (locals.var_q_qsq * 0.0166666666667);
        let assign18250_e18025: f64 = (locals.var_q_qsq * 0.0238095238095);
        let assign18250_e18026: f64 = (1.0 - assign18250_e18025);
        let assign18250_e18027: f64 = (assign18250_e18021 * assign18250_e18026);
        let assign18250_e18028: f64 = (1.0 - assign18250_e18027);
        let assign18250_e18029: f64 = (assign18250_e18017 * assign18250_e18028);
        let assign18250_e18030: f64 = (2.0 + assign18250_e18029);
        (assign18250_e18030, (((locals.var_q_qsq_dn4 * 0.1666666666667) * assign18250_e18028) + (assign18250_e18017 * (-(((locals.var_q_qsq_dn4 * 0.0166666666667) * assign18250_e18026) + (assign18250_e18021 * (-(locals.var_q_qsq_dn4 * 0.0238095238095))))))), (((locals.var_q_qsq_dn6 * 0.1666666666667) * assign18250_e18028) + (assign18250_e18017 * (-(((locals.var_q_qsq_dn6 * 0.0166666666667) * assign18250_e18026) + (assign18250_e18021 * (-(locals.var_q_qsq_dn6 * 0.0238095238095))))))), (((locals.var_q_qsq_dn7 * 0.1666666666667) * assign18250_e18028) + (assign18250_e18017 * (-(((locals.var_q_qsq_dn7 * 0.0166666666667) * assign18250_e18026) + (assign18250_e18021 * (-(locals.var_q_qsq_dn7 * 0.0238095238095))))))), (((locals.var_q_qsq_dn8 * 0.1666666666667) * assign18250_e18028) + (assign18250_e18017 * (-(((locals.var_q_qsq_dn8 * 0.0166666666667) * assign18250_e18026) + (assign18250_e18021 * (-(locals.var_q_qsq_dn8 * 0.0238095238095))))))), (((locals.var_q_qsq_dn9 * 0.1666666666667) * assign18250_e18028) + (assign18250_e18017 * (-(((locals.var_q_qsq_dn9 * 0.0166666666667) * assign18250_e18026) + (assign18250_e18021 * (-(locals.var_q_qsq_dn9 * 0.0238095238095))))))),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign18250_e18032;
        locals.var_q_qcoth_dn4 = assign18250_e18032_d_n4;
        locals.var_q_qcoth_dn6 = assign18250_e18032_d_n6;
        locals.var_q_qcoth_dn7 = assign18250_e18032_d_n7;
        locals.var_q_qcoth_dn8 = assign18250_e18032_d_n8;
        locals.var_q_qcoth_dn9 = assign18250_e18032_d_n9;

        let (assign18260_e18058, assign18260_e18058_d_n4, assign18260_e18058_d_n6, assign18260_e18058_d_n7, assign18260_e18058_d_n8, assign18260_e18058_d_n9,) = {
    if ((locals.var_guard628 == 0.0) && (locals.var_guard629 == 0.0)) {
        let assign18260_e18042: f64 = (locals.var_q_qsq * 0.0333333333333);
        let assign18260_e18046: f64 = (locals.var_q_qsq * 0.0357142857143);
        let assign18260_e18050: f64 = (locals.var_q_qsq * 0.0333333333333);
        let assign18260_e18051: f64 = (1.0 - assign18260_e18050);
        let assign18260_e18052: f64 = (assign18260_e18046 * assign18260_e18051);
        let assign18260_e18053: f64 = (1.0 - assign18260_e18052);
        let assign18260_e18054: f64 = (assign18260_e18042 * assign18260_e18053);
        let assign18260_e18055: f64 = (1.0 - assign18260_e18054);
        let assign18260_e18056: f64 = (0.1666666666667 * assign18260_e18055);
        (assign18260_e18056, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0333333333333) * assign18260_e18053) + (assign18260_e18042 * (-(((locals.var_q_qsq_dn4 * 0.0357142857143) * assign18260_e18051) + (assign18260_e18046 * (-(locals.var_q_qsq_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0333333333333) * assign18260_e18053) + (assign18260_e18042 * (-(((locals.var_q_qsq_dn6 * 0.0357142857143) * assign18260_e18051) + (assign18260_e18046 * (-(locals.var_q_qsq_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0333333333333) * assign18260_e18053) + (assign18260_e18042 * (-(((locals.var_q_qsq_dn7 * 0.0357142857143) * assign18260_e18051) + (assign18260_e18046 * (-(locals.var_q_qsq_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0333333333333) * assign18260_e18053) + (assign18260_e18042 * (-(((locals.var_q_qsq_dn8 * 0.0357142857143) * assign18260_e18051) + (assign18260_e18046 * (-(locals.var_q_qsq_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0333333333333) * assign18260_e18053) + (assign18260_e18042 * (-(((locals.var_q_qsq_dn9 * 0.0357142857143) * assign18260_e18051) + (assign18260_e18046 * (-(locals.var_q_qsq_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign18260_e18058;
        locals.var_q_d1_qcoth_dn4 = assign18260_e18058_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign18260_e18058_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign18260_e18058_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign18260_e18058_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign18260_e18058_d_n9;

        let assign18270_e18062: f64 = (locals.var_q_qi_int * locals.var_q_qcoth);
        let assign18270_e18065: f64 = (locals.var_q_k1q1 * locals.var_q_k2q2);
        let assign18270_e18066: f64 = (assign18270_e18062 + assign18270_e18065);
        let assign18270_e18068: f64 = (assign18270_e18066 + locals.var_q_qsq);
        let assign18270_e18071: f64 = (locals.var_q_qi_int * locals.var_q_d1_qcoth);
        let assign18270_e18073: f64 = (assign18270_e18071 + 1.0);
        let assign18270_e18074: f64 = (assign18270_e18068 / assign18270_e18073);
        let assign18270_e18075: f64 = (locals.var_q_qsq - assign18270_e18074);
        locals.var_q_qsq = assign18270_e18075;
        locals.var_q_qsq_dn4 = (locals.var_q_qsq_dn4 - (((((((locals.var_q_qi_int_dn4 * locals.var_q_qcoth) + (locals.var_q_qi_int * locals.var_q_qcoth_dn4)) + ((locals.var_q_k1q1_dn4 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn4))) + locals.var_q_qsq_dn4) * assign18270_e18073) - (assign18270_e18068 * ((locals.var_q_qi_int_dn4 * locals.var_q_d1_qcoth) + (locals.var_q_qi_int * locals.var_q_d1_qcoth_dn4)))) / (assign18270_e18073 * assign18270_e18073)));
        locals.var_q_qsq_dn6 = (locals.var_q_qsq_dn6 - (((((((locals.var_q_qi_int_dn6 * locals.var_q_qcoth) + (locals.var_q_qi_int * locals.var_q_qcoth_dn6)) + ((locals.var_q_k1q1_dn6 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn6))) + locals.var_q_qsq_dn6) * assign18270_e18073) - (assign18270_e18068 * ((locals.var_q_qi_int_dn6 * locals.var_q_d1_qcoth) + (locals.var_q_qi_int * locals.var_q_d1_qcoth_dn6)))) / (assign18270_e18073 * assign18270_e18073)));
        locals.var_q_qsq_dn7 = (locals.var_q_qsq_dn7 - (((((((locals.var_q_qi_int_dn7 * locals.var_q_qcoth) + (locals.var_q_qi_int * locals.var_q_qcoth_dn7)) + ((locals.var_q_k1q1_dn7 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn7))) + locals.var_q_qsq_dn7) * assign18270_e18073) - (assign18270_e18068 * ((locals.var_q_qi_int_dn7 * locals.var_q_d1_qcoth) + (locals.var_q_qi_int * locals.var_q_d1_qcoth_dn7)))) / (assign18270_e18073 * assign18270_e18073)));
        locals.var_q_qsq_dn8 = (locals.var_q_qsq_dn8 - (((((((locals.var_q_qi_int_dn8 * locals.var_q_qcoth) + (locals.var_q_qi_int * locals.var_q_qcoth_dn8)) + ((locals.var_q_k1q1_dn8 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn8))) + locals.var_q_qsq_dn8) * assign18270_e18073) - (assign18270_e18068 * ((locals.var_q_qi_int_dn8 * locals.var_q_d1_qcoth) + (locals.var_q_qi_int * locals.var_q_d1_qcoth_dn8)))) / (assign18270_e18073 * assign18270_e18073)));
        locals.var_q_qsq_dn9 = (locals.var_q_qsq_dn9 - (((((((locals.var_q_qi_int_dn9 * locals.var_q_qcoth) + (locals.var_q_qi_int * locals.var_q_qcoth_dn9)) + ((locals.var_q_k1q1_dn9 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn9))) + locals.var_q_qsq_dn9) * assign18270_e18073) - (assign18270_e18068 * ((locals.var_q_qi_int_dn9 * locals.var_q_d1_qcoth) + (locals.var_q_qi_int * locals.var_q_d1_qcoth_dn9)))) / (assign18270_e18073 * assign18270_e18073)));

        let assign18280_e18078: f64 = (locals.var_q_k1q1 * locals.var_q_k1q1);
        let assign18280_e18080: f64 = (assign18280_e18078 - locals.var_q_qsq);
        locals.var_q_delta = assign18280_e18080;
        locals.var_q_delta_dn4 = (((locals.var_q_k1q1_dn4 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn4)) - locals.var_q_qsq_dn4);
        locals.var_q_delta_dn6 = (((locals.var_q_k1q1_dn6 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn6)) - locals.var_q_qsq_dn6);
        locals.var_q_delta_dn7 = (((locals.var_q_k1q1_dn7 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn7)) - locals.var_q_qsq_dn7);
        locals.var_q_delta_dn8 = (((locals.var_q_k1q1_dn8 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn8)) - locals.var_q_qsq_dn8);
        locals.var_q_delta_dn9 = (((locals.var_q_k1q1_dn9 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn9)) - locals.var_q_qsq_dn9);

        let assign18290_e18083: f64 = if locals.var_q_delta > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard630 = assign18290_e18083;

        let (assign18300_e18098, assign18300_e18098_d_n4, assign18300_e18098_d_n6, assign18300_e18098_d_n7, assign18300_e18098_d_n8, assign18300_e18098_d_n9,) = {
    if (locals.var_guard630 != 0.0) {
        let assign18300_e18088: f64 = (locals.var_q_delta / locals.var_a0);
        let assign18300_e18089: f64 = (assign18300_e18088).ln();
        let assign18300_e18091: f64 = (assign18300_e18089 + locals.var_xdeff);
        let assign18300_e18093: f64 = (assign18300_e18091 - locals.var_xg1x);
        let assign18300_e18095: f64 = (assign18300_e18093 + locals.var_q1d);
        let assign18300_e18096: f64 = (locals.var_q_delta * assign18300_e18095);
        (assign18300_e18096, ((locals.var_q_delta_dn4 * assign18300_e18095) + (locals.var_q_delta * (((((((locals.var_q_delta_dn4 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn4)) / (locals.var_a0 * locals.var_a0)) / assign18300_e18088) + locals.var_xdeff_dn4) - locals.var_xg1x_dn4) + locals.var_q1d_dn4))), ((locals.var_q_delta_dn6 * assign18300_e18095) + (locals.var_q_delta * (((((((locals.var_q_delta_dn6 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn6)) / (locals.var_a0 * locals.var_a0)) / assign18300_e18088) + locals.var_xdeff_dn6) - locals.var_xg1x_dn6) + locals.var_q1d_dn6))), ((locals.var_q_delta_dn7 * assign18300_e18095) + (locals.var_q_delta * (((((((locals.var_q_delta_dn7 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn7)) / (locals.var_a0 * locals.var_a0)) / assign18300_e18088) + locals.var_xdeff_dn7) - locals.var_xg1x_dn7) + locals.var_q1d_dn7))), ((locals.var_q_delta_dn8 * assign18300_e18095) + (locals.var_q_delta * (((((((locals.var_q_delta_dn8 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn8)) / (locals.var_a0 * locals.var_a0)) / assign18300_e18088) + locals.var_xdeff_dn8) - locals.var_xg1x_dn8) + locals.var_q1d_dn8))), ((locals.var_q_delta_dn9 * assign18300_e18095) + (locals.var_q_delta * (((((((locals.var_q_delta_dn9 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn9)) / (locals.var_a0 * locals.var_a0)) / assign18300_e18088) + locals.var_xdeff_dn9) - locals.var_xg1x_dn9) + locals.var_q1d_dn9))),)
    } else {
        (locals.var_q_zero, locals.var_q_zero_dn4, locals.var_q_zero_dn6, locals.var_q_zero_dn7, locals.var_q_zero_dn8, locals.var_q_zero_dn9,)
    }
};
        locals.var_q_zero = assign18300_e18098;
        locals.var_q_zero_dn4 = assign18300_e18098_d_n4;
        locals.var_q_zero_dn6 = assign18300_e18098_d_n6;
        locals.var_q_zero_dn7 = assign18300_e18098_d_n7;
        locals.var_q_zero_dn8 = assign18300_e18098_d_n8;
        locals.var_q_zero_dn9 = assign18300_e18098_d_n9;

        let (assign18310_e18108, assign18310_e18108_d_n4, assign18310_e18108_d_n6, assign18310_e18108_d_n7, assign18310_e18108_d_n8, assign18310_e18108_d_n9,) = {
    if (locals.var_guard630 != 0.0) {
        let assign18310_e18102: f64 = (2.0 * locals.var_k1);
        let assign18310_e18104: f64 = (assign18310_e18102 * locals.var_q_k1q1);
        let assign18310_e18106: f64 = (assign18310_e18104 + locals.var_q_delta);
        (assign18310_e18106, ((((2.0 * locals.var_k1_dn4) * locals.var_q_k1q1) + (assign18310_e18102 * locals.var_q_k1q1_dn4)) + locals.var_q_delta_dn4), ((((2.0 * locals.var_k1_dn6) * locals.var_q_k1q1) + (assign18310_e18102 * locals.var_q_k1q1_dn6)) + locals.var_q_delta_dn6), ((((2.0 * locals.var_k1_dn7) * locals.var_q_k1q1) + (assign18310_e18102 * locals.var_q_k1q1_dn7)) + locals.var_q_delta_dn7), ((((2.0 * locals.var_k1_dn8) * locals.var_q_k1q1) + (assign18310_e18102 * locals.var_q_k1q1_dn8)) + locals.var_q_delta_dn8), ((((2.0 * locals.var_k1_dn9) * locals.var_q_k1q1) + (assign18310_e18102 * locals.var_q_k1q1_dn9)) + locals.var_q_delta_dn9),)
    } else {
        (locals.var_q_d1_zero, locals.var_q_d1_zero_dn4, locals.var_q_d1_zero_dn6, locals.var_q_d1_zero_dn7, locals.var_q_d1_zero_dn8, locals.var_q_d1_zero_dn9,)
    }
};
        locals.var_q_d1_zero = assign18310_e18108;
        locals.var_q_d1_zero_dn4 = assign18310_e18108_d_n4;
        locals.var_q_d1_zero_dn6 = assign18310_e18108_d_n6;
        locals.var_q_d1_zero_dn7 = assign18310_e18108_d_n7;
        locals.var_q_d1_zero_dn8 = assign18310_e18108_d_n8;
        locals.var_q_d1_zero_dn9 = assign18310_e18108_d_n9;

        let (assign18320_e18116,) = {
    if (locals.var_guard630 != 0.0) {
        let assign18320_e18112: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign18320_e18114: f64 = (assign18320_e18112 - locals.var_q_x1sat);
        (assign18320_e18114,)
    } else {
        (locals.var_q_dx1,)
    }
};
        locals.var_q_dx1 = assign18320_e18116;

        let assign18330_e18126: f64 = (locals.var_q_dx1 + 2.3025850929941);
        let assign18330_e18128: f64 = (locals.var_k1).ln();
        let assign18330_e18129: f64 = (assign18330_e18126 + assign18330_e18128);
        let assign18330_e18136: f64 = if ((((locals.var_q_zero < 0.0) && (locals.var_q_d1_zero > 0.0)) && (assign18330_e18129 > 0.0)) || (locals.var_q_dx1 > 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard631 = assign18330_e18136;

        let (assign18340_e18146, assign18340_e18146_d_n4, assign18340_e18146_d_n6, assign18340_e18146_d_n7, assign18340_e18146_d_n8, assign18340_e18146_d_n9,) = {
    if ((locals.var_guard630 != 0.0) && (locals.var_guard631 != 0.0)) {
        let assign18340_e18143: f64 = (locals.var_q_zero / locals.var_q_d1_zero);
        let assign18340_e18144: f64 = (locals.var_q1d - assign18340_e18143);
        (assign18340_e18144, (locals.var_q1d_dn4 - (((locals.var_q_zero_dn4 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn4)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1d_dn6 - (((locals.var_q_zero_dn6 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn6)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1d_dn7 - (((locals.var_q_zero_dn7 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn7)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1d_dn8 - (((locals.var_q_zero_dn8 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn8)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1d_dn9 - (((locals.var_q_zero_dn9 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn9)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))),)
    } else {
        (locals.var_q1d, locals.var_q1d_dn4, locals.var_q1d_dn6, locals.var_q1d_dn7, locals.var_q1d_dn8, locals.var_q1d_dn9,)
    }
};
        locals.var_q1d = assign18340_e18146;
        locals.var_q1d_dn4 = assign18340_e18146_d_n4;
        locals.var_q1d_dn6 = assign18340_e18146_d_n6;
        locals.var_q1d_dn7 = assign18340_e18146_d_n7;
        locals.var_q1d_dn8 = assign18340_e18146_d_n8;
        locals.var_q1d_dn9 = assign18340_e18146_d_n9;

        let assign18350_e18149: f64 = (locals.var_k1 * locals.var_q1d);
        locals.var_q_k1q1 = assign18350_e18149;
        locals.var_q_k1q1_dn4 = ((locals.var_k1_dn4 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn4));
        locals.var_q_k1q1_dn6 = ((locals.var_k1_dn6 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn6));
        locals.var_q_k1q1_dn7 = ((locals.var_k1_dn7 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn7));
        locals.var_q_k1q1_dn8 = ((locals.var_k1_dn8 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn8));
        locals.var_q_k1q1_dn9 = ((locals.var_k1_dn9 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn9));

        let assign18360_e18152: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign18360_e18154: f64 = (assign18360_e18152 - locals.var_xdeff);
        let assign18360_e18156: f64 = if assign18360_e18154 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard632 = assign18360_e18156;

        let (assign18370_e18165, assign18370_e18165_d_n4, assign18370_e18165_d_n6, assign18370_e18165_d_n7, assign18370_e18165_d_n8, assign18370_e18165_d_n9,) = {
    if (locals.var_guard632 != 0.0) {
        let assign18370_e18160: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign18370_e18162: f64 = (assign18370_e18160 - locals.var_xdeff);
        let assign18370_e18163: f64 = (assign18370_e18162).exp();
        (assign18370_e18163, (assign18370_e18163 * ((locals.var_xg1x_dn4 - locals.var_q1d_dn4) - locals.var_xdeff_dn4)), (assign18370_e18163 * ((locals.var_xg1x_dn6 - locals.var_q1d_dn6) - locals.var_xdeff_dn6)), (assign18370_e18163 * ((locals.var_xg1x_dn7 - locals.var_q1d_dn7) - locals.var_xdeff_dn7)), (assign18370_e18163 * ((locals.var_xg1x_dn8 - locals.var_q1d_dn8) - locals.var_xdeff_dn8)), (assign18370_e18163 * ((locals.var_xg1x_dn9 - locals.var_q1d_dn9) - locals.var_xdeff_dn9)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign18370_e18165;
        locals.var_q_temp1_dn4 = assign18370_e18165_d_n4;
        locals.var_q_temp1_dn6 = assign18370_e18165_d_n6;
        locals.var_q_temp1_dn7 = assign18370_e18165_d_n7;
        locals.var_q_temp1_dn8 = assign18370_e18165_d_n8;
        locals.var_q_temp1_dn9 = assign18370_e18165_d_n9;

        let (assign18380_e18204, assign18380_e18204_d_n4, assign18380_e18204_d_n6, assign18380_e18204_d_n7, assign18380_e18204_d_n8, assign18380_e18204_d_n9,) = {
    if (locals.var_guard632 == 0.0) {
        let assign18380_e18172: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign18380_e18174: f64 = (assign18380_e18172 - locals.var_xdeff);
        let assign18380_e18176: f64 = (assign18380_e18174 - 80.0);
        let assign18380_e18181: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign18380_e18183: f64 = (assign18380_e18181 - locals.var_xdeff);
        let assign18380_e18185: f64 = (assign18380_e18183 - 80.0);
        let assign18380_e18186: f64 = (0.5 * assign18380_e18185);
        let assign18380_e18190: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign18380_e18192: f64 = (assign18380_e18190 - locals.var_xdeff);
        let assign18380_e18194: f64 = (assign18380_e18192 - 80.0);
        let assign18380_e18196: f64 = (assign18380_e18194 * 0.3333333333333);
        let assign18380_e18197: f64 = (1.0 + assign18380_e18196);
        let assign18380_e18198: f64 = (assign18380_e18186 * assign18380_e18197);
        let assign18380_e18199: f64 = (1.0 + assign18380_e18198);
        let assign18380_e18200: f64 = (assign18380_e18176 * assign18380_e18199);
        let assign18380_e18201: f64 = (1.0 + assign18380_e18200);
        let assign18380_e18202: f64 = (5.54062e34 * assign18380_e18201);
        (assign18380_e18202, (5.54062e34 * ((((locals.var_xg1x_dn4 - locals.var_q1d_dn4) - locals.var_xdeff_dn4) * assign18380_e18199) + (assign18380_e18176 * (((0.5 * ((locals.var_xg1x_dn4 - locals.var_q1d_dn4) - locals.var_xdeff_dn4)) * assign18380_e18197) + (assign18380_e18186 * (((locals.var_xg1x_dn4 - locals.var_q1d_dn4) - locals.var_xdeff_dn4) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x_dn6 - locals.var_q1d_dn6) - locals.var_xdeff_dn6) * assign18380_e18199) + (assign18380_e18176 * (((0.5 * ((locals.var_xg1x_dn6 - locals.var_q1d_dn6) - locals.var_xdeff_dn6)) * assign18380_e18197) + (assign18380_e18186 * (((locals.var_xg1x_dn6 - locals.var_q1d_dn6) - locals.var_xdeff_dn6) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x_dn7 - locals.var_q1d_dn7) - locals.var_xdeff_dn7) * assign18380_e18199) + (assign18380_e18176 * (((0.5 * ((locals.var_xg1x_dn7 - locals.var_q1d_dn7) - locals.var_xdeff_dn7)) * assign18380_e18197) + (assign18380_e18186 * (((locals.var_xg1x_dn7 - locals.var_q1d_dn7) - locals.var_xdeff_dn7) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x_dn8 - locals.var_q1d_dn8) - locals.var_xdeff_dn8) * assign18380_e18199) + (assign18380_e18176 * (((0.5 * ((locals.var_xg1x_dn8 - locals.var_q1d_dn8) - locals.var_xdeff_dn8)) * assign18380_e18197) + (assign18380_e18186 * (((locals.var_xg1x_dn8 - locals.var_q1d_dn8) - locals.var_xdeff_dn8) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x_dn9 - locals.var_q1d_dn9) - locals.var_xdeff_dn9) * assign18380_e18199) + (assign18380_e18176 * (((0.5 * ((locals.var_xg1x_dn9 - locals.var_q1d_dn9) - locals.var_xdeff_dn9)) * assign18380_e18197) + (assign18380_e18186 * (((locals.var_xg1x_dn9 - locals.var_q1d_dn9) - locals.var_xdeff_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign18380_e18204;
        locals.var_q_temp1_dn4 = assign18380_e18204_d_n4;
        locals.var_q_temp1_dn6 = assign18380_e18204_d_n6;
        locals.var_q_temp1_dn7 = assign18380_e18204_d_n7;
        locals.var_q_temp1_dn8 = assign18380_e18204_d_n8;
        locals.var_q_temp1_dn9 = assign18380_e18204_d_n9;

        let assign18390_e18207: f64 = (locals.var_a0 * locals.var_q_temp1);
        locals.var_q_aexp = assign18390_e18207;
        locals.var_q_aexp_dn4 = ((locals.var_a0_dn4 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn4));
        locals.var_q_aexp_dn6 = ((locals.var_a0_dn6 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn6));
        locals.var_q_aexp_dn7 = ((locals.var_a0_dn7 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn7));
        locals.var_q_aexp_dn8 = ((locals.var_a0_dn8 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn8));
        locals.var_q_aexp_dn9 = ((locals.var_a0_dn9 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn9));

        let assign18400_e18210: f64 = (locals.var_q_k1q1 * locals.var_q_k1q1);
        let assign18400_e18212: f64 = (assign18400_e18210 - locals.var_q_aexp);
        locals.var_q_qsq = assign18400_e18212;
        locals.var_q_qsq_dn4 = (((locals.var_q_k1q1_dn4 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn4)) - locals.var_q_aexp_dn4);
        locals.var_q_qsq_dn6 = (((locals.var_q_k1q1_dn6 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn6)) - locals.var_q_aexp_dn6);
        locals.var_q_qsq_dn7 = (((locals.var_q_k1q1_dn7 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn7)) - locals.var_q_aexp_dn7);
        locals.var_q_qsq_dn8 = (((locals.var_q_k1q1_dn8 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn8)) - locals.var_q_aexp_dn8);
        locals.var_q_qsq_dn9 = (((locals.var_q_k1q1_dn9 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn9)) - locals.var_q_aexp_dn9);

        let assign18410_e18215: f64 = (2.0 * locals.var_k1);
        let assign18410_e18217: f64 = (assign18410_e18215 * locals.var_q_k1q1);
        let assign18410_e18219: f64 = (assign18410_e18217 + locals.var_q_aexp);
        locals.var_q_d1_qsq = assign18410_e18219;
        locals.var_q_d1_qsq_dn4 = ((((2.0 * locals.var_k1_dn4) * locals.var_q_k1q1) + (assign18410_e18215 * locals.var_q_k1q1_dn4)) + locals.var_q_aexp_dn4);
        locals.var_q_d1_qsq_dn6 = ((((2.0 * locals.var_k1_dn6) * locals.var_q_k1q1) + (assign18410_e18215 * locals.var_q_k1q1_dn6)) + locals.var_q_aexp_dn6);
        locals.var_q_d1_qsq_dn7 = ((((2.0 * locals.var_k1_dn7) * locals.var_q_k1q1) + (assign18410_e18215 * locals.var_q_k1q1_dn7)) + locals.var_q_aexp_dn7);
        locals.var_q_d1_qsq_dn8 = ((((2.0 * locals.var_k1_dn8) * locals.var_q_k1q1) + (assign18410_e18215 * locals.var_q_k1q1_dn8)) + locals.var_q_aexp_dn8);
        locals.var_q_d1_qsq_dn9 = ((((2.0 * locals.var_k1_dn9) * locals.var_q_k1q1) + (assign18410_e18215 * locals.var_q_k1q1_dn9)) + locals.var_q_aexp_dn9);

    }

    pub(super) fn stamp_transient_block_46(
        locals: &mut StampLocals,
    ) {
        let assign18420_e18222: f64 = (2.0 * locals.var_k1);
        let assign18420_e18224: f64 = (assign18420_e18222 * locals.var_k1);
        let assign18420_e18226: f64 = (assign18420_e18224 - locals.var_q_aexp);
        locals.var_q_d2_qsq = assign18420_e18226;
        locals.var_q_d2_qsq_dn4 = ((((2.0 * locals.var_k1_dn4) * locals.var_k1) + (assign18420_e18222 * locals.var_k1_dn4)) - locals.var_q_aexp_dn4);
        locals.var_q_d2_qsq_dn6 = ((((2.0 * locals.var_k1_dn6) * locals.var_k1) + (assign18420_e18222 * locals.var_k1_dn6)) - locals.var_q_aexp_dn6);
        locals.var_q_d2_qsq_dn7 = ((((2.0 * locals.var_k1_dn7) * locals.var_k1) + (assign18420_e18222 * locals.var_k1_dn7)) - locals.var_q_aexp_dn7);
        locals.var_q_d2_qsq_dn8 = ((((2.0 * locals.var_k1_dn8) * locals.var_k1) + (assign18420_e18222 * locals.var_k1_dn8)) - locals.var_q_aexp_dn8);
        locals.var_q_d2_qsq_dn9 = ((((2.0 * locals.var_k1_dn9) * locals.var_k1) + (assign18420_e18222 * locals.var_k1_dn9)) - locals.var_q_aexp_dn9);

        let assign18430_e18229: f64 = (-0.005);
        let assign18430_e18230: f64 = if locals.var_q_qsq < assign18430_e18229 { 1.0 } else { 0.0 };
        locals.var_guard633 = assign18430_e18230;

        let (assign18440_e18236, assign18440_e18236_d_n4, assign18440_e18236_d_n6, assign18440_e18236_d_n7, assign18440_e18236_d_n8, assign18440_e18236_d_n9,) = {
    if (locals.var_guard633 != 0.0) {
        let assign18440_e18233: f64 = (locals.var_q_qsq).abs();
        let assign18440_e18234: f64 = (assign18440_e18233).sqrt();
        (assign18440_e18234, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign18440_e18234)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign18440_e18234)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign18440_e18234)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign18440_e18234)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign18440_e18234)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign18440_e18236;
        locals.var_q_rac_qsq_dn4 = assign18440_e18236_d_n4;
        locals.var_q_rac_qsq_dn6 = assign18440_e18236_d_n6;
        locals.var_q_rac_qsq_dn7 = assign18440_e18236_d_n7;
        locals.var_q_rac_qsq_dn8 = assign18440_e18236_d_n8;
        locals.var_q_rac_qsq_dn9 = assign18440_e18236_d_n9;

        let (assign18450_e18245, assign18450_e18245_d_n4, assign18450_e18245_d_n6, assign18450_e18245_d_n7, assign18450_e18245_d_n8, assign18450_e18245_d_n9,) = {
    if (locals.var_guard633 != 0.0) {
        let assign18450_e18241: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign18450_e18242: f64 = (assign18450_e18241).tan();
        let assign18450_e18243: f64 = (locals.var_q_rac_qsq / assign18450_e18242);
        (assign18450_e18243, (((locals.var_q_rac_qsq_dn4 * assign18450_e18242) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn4) / ((assign18450_e18241).cos() * (assign18450_e18241).cos())))) / (assign18450_e18242 * assign18450_e18242)), (((locals.var_q_rac_qsq_dn6 * assign18450_e18242) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn6) / ((assign18450_e18241).cos() * (assign18450_e18241).cos())))) / (assign18450_e18242 * assign18450_e18242)), (((locals.var_q_rac_qsq_dn7 * assign18450_e18242) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn7) / ((assign18450_e18241).cos() * (assign18450_e18241).cos())))) / (assign18450_e18242 * assign18450_e18242)), (((locals.var_q_rac_qsq_dn8 * assign18450_e18242) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn8) / ((assign18450_e18241).cos() * (assign18450_e18241).cos())))) / (assign18450_e18242 * assign18450_e18242)), (((locals.var_q_rac_qsq_dn9 * assign18450_e18242) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn9) / ((assign18450_e18241).cos() * (assign18450_e18241).cos())))) / (assign18450_e18242 * assign18450_e18242)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign18450_e18245;
        locals.var_q_qcoth_dn4 = assign18450_e18245_d_n4;
        locals.var_q_qcoth_dn6 = assign18450_e18245_d_n6;
        locals.var_q_qcoth_dn7 = assign18450_e18245_d_n7;
        locals.var_q_qcoth_dn8 = assign18450_e18245_d_n8;
        locals.var_q_qcoth_dn9 = assign18450_e18245_d_n9;

        let (assign18460_e18253, assign18460_e18253_d_n4, assign18460_e18253_d_n6, assign18460_e18253_d_n7, assign18460_e18253_d_n8, assign18460_e18253_d_n9,) = {
    if (locals.var_guard633 != 0.0) {
        let assign18460_e18249: f64 = (0.25 * locals.var_q_d1_qsq);
        let assign18460_e18251: f64 = (assign18460_e18249 / locals.var_q_qsq);
        (assign18460_e18251, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign18460_e18249 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign18460_e18249 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign18460_e18249 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign18460_e18249 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign18460_e18249 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign18460_e18253;
        locals.var_q_temp1_dn4 = assign18460_e18253_d_n4;
        locals.var_q_temp1_dn6 = assign18460_e18253_d_n6;
        locals.var_q_temp1_dn7 = assign18460_e18253_d_n7;
        locals.var_q_temp1_dn8 = assign18460_e18253_d_n8;
        locals.var_q_temp1_dn9 = assign18460_e18253_d_n9;

        let (assign18470_e18265, assign18470_e18265_d_n4, assign18470_e18265_d_n6, assign18470_e18265_d_n7, assign18470_e18265_d_n8, assign18470_e18265_d_n9,) = {
    if (locals.var_guard633 != 0.0) {
        let assign18470_e18259: f64 = (2.0 - locals.var_q_qcoth);
        let assign18470_e18260: f64 = (locals.var_q_qcoth * assign18470_e18259);
        let assign18470_e18261: f64 = (locals.var_q_qsq + assign18470_e18260);
        let assign18470_e18263: f64 = (assign18470_e18261 * locals.var_q_temp1);
        (assign18470_e18263, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign18470_e18259) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign18470_e18261 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign18470_e18259) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign18470_e18261 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign18470_e18259) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign18470_e18261 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign18470_e18259) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign18470_e18261 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign18470_e18259) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign18470_e18261 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign18470_e18265;
        locals.var_q_d1_qcoth_dn4 = assign18470_e18265_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign18470_e18265_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign18470_e18265_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign18470_e18265_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign18470_e18265_d_n9;

        let (assign18480_e18285, assign18480_e18285_d_n4, assign18480_e18285_d_n6, assign18480_e18285_d_n7, assign18480_e18285_d_n8, assign18480_e18285_d_n9,) = {
    if (locals.var_guard633 != 0.0) {
        let assign18480_e18270: f64 = (2.0 * locals.var_q_d1_qcoth);
        let assign18480_e18273: f64 = (1.0 + locals.var_q_qcoth);
        let assign18480_e18274: f64 = (assign18480_e18270 * assign18480_e18273);
        let assign18480_e18275: f64 = (locals.var_q_d1_qsq - assign18480_e18274);
        let assign18480_e18277: f64 = (assign18480_e18275 * locals.var_q_temp1);
        let assign18480_e18280: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
        let assign18480_e18282: f64 = (assign18480_e18280 / locals.var_q_d1_qsq);
        let assign18480_e18283: f64 = (assign18480_e18277 + assign18480_e18282);
        (assign18480_e18283, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign18480_e18273) + (assign18480_e18270 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign18480_e18275 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign18480_e18280 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign18480_e18273) + (assign18480_e18270 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign18480_e18275 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign18480_e18280 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign18480_e18273) + (assign18480_e18270 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign18480_e18275 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign18480_e18280 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign18480_e18273) + (assign18480_e18270 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign18480_e18275 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign18480_e18280 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign18480_e18273) + (assign18480_e18270 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign18480_e18275 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign18480_e18280 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign18480_e18285;
        locals.var_q_d2_qcoth_dn4 = assign18480_e18285_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign18480_e18285_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign18480_e18285_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign18480_e18285_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign18480_e18285_d_n9;

        let (assign18490_e18293, assign18490_e18293_d_n4, assign18490_e18293_d_n6, assign18490_e18293_d_n7, assign18490_e18293_d_n8, assign18490_e18293_d_n9,) = {
    if (locals.var_guard633 != 0.0) {
        let assign18490_e18290: f64 = (0.5 * locals.var_q_qcoth);
        let assign18490_e18291: f64 = (1.0 - assign18490_e18290);
        (assign18490_e18291, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign18490_e18293;
        locals.var_q_temp2_dn4 = assign18490_e18293_d_n4;
        locals.var_q_temp2_dn6 = assign18490_e18293_d_n6;
        locals.var_q_temp2_dn7 = assign18490_e18293_d_n7;
        locals.var_q_temp2_dn8 = assign18490_e18293_d_n8;
        locals.var_q_temp2_dn9 = assign18490_e18293_d_n9;

        let (assign18500_e18301, assign18500_e18301_d_n4, assign18500_e18301_d_n6, assign18500_e18301_d_n7, assign18500_e18301_d_n8, assign18500_e18301_d_n9,) = {
    if (locals.var_guard633 != 0.0) {
        let assign18500_e18297: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
        let assign18500_e18299: f64 = (assign18500_e18297 * locals.var_q_temp2);
        (assign18500_e18299, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign18500_e18297 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign18500_e18297 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign18500_e18297 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign18500_e18297 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign18500_e18297 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign18500_e18301;
        locals.var_q_d1_ln_dn4 = assign18500_e18301_d_n4;
        locals.var_q_d1_ln_dn6 = assign18500_e18301_d_n6;
        locals.var_q_d1_ln_dn7 = assign18500_e18301_d_n7;
        locals.var_q_d1_ln_dn8 = assign18500_e18301_d_n8;
        locals.var_q_d1_ln_dn9 = assign18500_e18301_d_n9;

        let (assign18510_e18317, assign18510_e18317_d_n4, assign18510_e18317_d_n6, assign18510_e18317_d_n7, assign18510_e18317_d_n8, assign18510_e18317_d_n9,) = {
    if (locals.var_guard633 != 0.0) {
        let assign18510_e18305: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
        let assign18510_e18310: f64 = (0.5 * locals.var_q_d1_qcoth);
        let assign18510_e18311: f64 = (locals.var_q_d1_ln + assign18510_e18310);
        let assign18510_e18312: f64 = (locals.var_q_d1_qsq * assign18510_e18311);
        let assign18510_e18313: f64 = (assign18510_e18305 - assign18510_e18312);
        let assign18510_e18315: f64 = (assign18510_e18313 / locals.var_q_qsq);
        (assign18510_e18315, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign18510_e18311) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign18510_e18313 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign18510_e18311) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign18510_e18313 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign18510_e18311) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign18510_e18313 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign18510_e18311) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign18510_e18313 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign18510_e18311) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign18510_e18313 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign18510_e18317;
        locals.var_q_d2_ln_dn4 = assign18510_e18317_d_n4;
        locals.var_q_d2_ln_dn6 = assign18510_e18317_d_n6;
        locals.var_q_d2_ln_dn7 = assign18510_e18317_d_n7;
        locals.var_q_d2_ln_dn8 = assign18510_e18317_d_n8;
        locals.var_q_d2_ln_dn9 = assign18510_e18317_d_n9;

        let assign18520_e18320: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard634 = assign18520_e18320;

        let (assign18530_e18329, assign18530_e18329_d_n4, assign18530_e18329_d_n6, assign18530_e18329_d_n7, assign18530_e18329_d_n8, assign18530_e18329_d_n9,) = {
    if ((locals.var_guard633 == 0.0) && (locals.var_guard634 != 0.0)) {
        let assign18530_e18326: f64 = (locals.var_q_qsq).abs();
        let assign18530_e18327: f64 = (assign18530_e18326).sqrt();
        (assign18530_e18327, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign18530_e18327)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign18530_e18327)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign18530_e18327)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign18530_e18327)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign18530_e18327)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign18530_e18329;
        locals.var_q_rac_qsq_dn4 = assign18530_e18329_d_n4;
        locals.var_q_rac_qsq_dn6 = assign18530_e18329_d_n6;
        locals.var_q_rac_qsq_dn7 = assign18530_e18329_d_n7;
        locals.var_q_rac_qsq_dn8 = assign18530_e18329_d_n8;
        locals.var_q_rac_qsq_dn9 = assign18530_e18329_d_n9;

        let (assign18540_e18338, assign18540_e18338_d_n4, assign18540_e18338_d_n6, assign18540_e18338_d_n7, assign18540_e18338_d_n8, assign18540_e18338_d_n9,) = {
    if ((locals.var_guard633 == 0.0) && (locals.var_guard634 != 0.0)) {
        let assign18540_e18335: f64 = (-locals.var_q_rac_qsq);
        let assign18540_e18336: f64 = (assign18540_e18335).exp();
        (assign18540_e18336, (assign18540_e18336 * (-locals.var_q_rac_qsq_dn4)), (assign18540_e18336 * (-locals.var_q_rac_qsq_dn6)), (assign18540_e18336 * (-locals.var_q_rac_qsq_dn7)), (assign18540_e18336 * (-locals.var_q_rac_qsq_dn8)), (assign18540_e18336 * (-locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_invexpq, locals.var_q_invexpq_dn4, locals.var_q_invexpq_dn6, locals.var_q_invexpq_dn7, locals.var_q_invexpq_dn8, locals.var_q_invexpq_dn9,)
    }
};
        locals.var_q_invexpq = assign18540_e18338;
        locals.var_q_invexpq_dn4 = assign18540_e18338_d_n4;
        locals.var_q_invexpq_dn6 = assign18540_e18338_d_n6;
        locals.var_q_invexpq_dn7 = assign18540_e18338_d_n7;
        locals.var_q_invexpq_dn8 = assign18540_e18338_d_n8;
        locals.var_q_invexpq_dn9 = assign18540_e18338_d_n9;

        let (assign18550_e18353, assign18550_e18353_d_n4, assign18550_e18353_d_n6, assign18550_e18353_d_n7, assign18550_e18353_d_n8, assign18550_e18353_d_n9,) = {
    if ((locals.var_guard633 == 0.0) && (locals.var_guard634 != 0.0)) {
        let assign18550_e18346: f64 = (1.0 + locals.var_q_invexpq);
        let assign18550_e18347: f64 = (locals.var_q_rac_qsq * assign18550_e18346);
        let assign18550_e18350: f64 = (1.0 - locals.var_q_invexpq);
        let assign18550_e18351: f64 = (assign18550_e18347 / assign18550_e18350);
        (assign18550_e18351, (((((locals.var_q_rac_qsq_dn4 * assign18550_e18346) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn4)) * assign18550_e18350) - (assign18550_e18347 * (-locals.var_q_invexpq_dn4))) / (assign18550_e18350 * assign18550_e18350)), (((((locals.var_q_rac_qsq_dn6 * assign18550_e18346) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn6)) * assign18550_e18350) - (assign18550_e18347 * (-locals.var_q_invexpq_dn6))) / (assign18550_e18350 * assign18550_e18350)), (((((locals.var_q_rac_qsq_dn7 * assign18550_e18346) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn7)) * assign18550_e18350) - (assign18550_e18347 * (-locals.var_q_invexpq_dn7))) / (assign18550_e18350 * assign18550_e18350)), (((((locals.var_q_rac_qsq_dn8 * assign18550_e18346) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn8)) * assign18550_e18350) - (assign18550_e18347 * (-locals.var_q_invexpq_dn8))) / (assign18550_e18350 * assign18550_e18350)), (((((locals.var_q_rac_qsq_dn9 * assign18550_e18346) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn9)) * assign18550_e18350) - (assign18550_e18347 * (-locals.var_q_invexpq_dn9))) / (assign18550_e18350 * assign18550_e18350)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign18550_e18353;
        locals.var_q_qcoth_dn4 = assign18550_e18353_d_n4;
        locals.var_q_qcoth_dn6 = assign18550_e18353_d_n6;
        locals.var_q_qcoth_dn7 = assign18550_e18353_d_n7;
        locals.var_q_qcoth_dn8 = assign18550_e18353_d_n8;
        locals.var_q_qcoth_dn9 = assign18550_e18353_d_n9;

        let (assign18560_e18364, assign18560_e18364_d_n4, assign18560_e18364_d_n6, assign18560_e18364_d_n7, assign18560_e18364_d_n8, assign18560_e18364_d_n9,) = {
    if ((locals.var_guard633 == 0.0) && (locals.var_guard634 != 0.0)) {
        let assign18560_e18360: f64 = (0.25 * locals.var_q_d1_qsq);
        let assign18560_e18362: f64 = (assign18560_e18360 / locals.var_q_qsq);
        (assign18560_e18362, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign18560_e18360 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign18560_e18360 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign18560_e18360 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign18560_e18360 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign18560_e18360 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign18560_e18364;
        locals.var_q_temp1_dn4 = assign18560_e18364_d_n4;
        locals.var_q_temp1_dn6 = assign18560_e18364_d_n6;
        locals.var_q_temp1_dn7 = assign18560_e18364_d_n7;
        locals.var_q_temp1_dn8 = assign18560_e18364_d_n8;
        locals.var_q_temp1_dn9 = assign18560_e18364_d_n9;

        let (assign18570_e18379, assign18570_e18379_d_n4, assign18570_e18379_d_n6, assign18570_e18379_d_n7, assign18570_e18379_d_n8, assign18570_e18379_d_n9,) = {
    if ((locals.var_guard633 == 0.0) && (locals.var_guard634 != 0.0)) {
        let assign18570_e18373: f64 = (2.0 - locals.var_q_qcoth);
        let assign18570_e18374: f64 = (locals.var_q_qcoth * assign18570_e18373);
        let assign18570_e18375: f64 = (locals.var_q_qsq + assign18570_e18374);
        let assign18570_e18377: f64 = (assign18570_e18375 * locals.var_q_temp1);
        (assign18570_e18377, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign18570_e18373) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign18570_e18375 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign18570_e18373) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign18570_e18375 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign18570_e18373) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign18570_e18375 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign18570_e18373) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign18570_e18375 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign18570_e18373) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign18570_e18375 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign18570_e18379;
        locals.var_q_d1_qcoth_dn4 = assign18570_e18379_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign18570_e18379_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign18570_e18379_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign18570_e18379_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign18570_e18379_d_n9;

        let (assign18580_e18402, assign18580_e18402_d_n4, assign18580_e18402_d_n6, assign18580_e18402_d_n7, assign18580_e18402_d_n8, assign18580_e18402_d_n9,) = {
    if ((locals.var_guard633 == 0.0) && (locals.var_guard634 != 0.0)) {
        let assign18580_e18387: f64 = (2.0 * locals.var_q_d1_qcoth);
        let assign18580_e18390: f64 = (1.0 + locals.var_q_qcoth);
        let assign18580_e18391: f64 = (assign18580_e18387 * assign18580_e18390);
        let assign18580_e18392: f64 = (locals.var_q_d1_qsq - assign18580_e18391);
        let assign18580_e18394: f64 = (assign18580_e18392 * locals.var_q_temp1);
        let assign18580_e18397: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
        let assign18580_e18399: f64 = (assign18580_e18397 / locals.var_q_d1_qsq);
        let assign18580_e18400: f64 = (assign18580_e18394 + assign18580_e18399);
        (assign18580_e18400, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign18580_e18390) + (assign18580_e18387 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign18580_e18392 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign18580_e18397 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign18580_e18390) + (assign18580_e18387 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign18580_e18392 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign18580_e18397 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign18580_e18390) + (assign18580_e18387 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign18580_e18392 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign18580_e18397 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign18580_e18390) + (assign18580_e18387 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign18580_e18392 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign18580_e18397 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign18580_e18390) + (assign18580_e18387 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign18580_e18392 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign18580_e18397 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign18580_e18402;
        locals.var_q_d2_qcoth_dn4 = assign18580_e18402_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign18580_e18402_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign18580_e18402_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign18580_e18402_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign18580_e18402_d_n9;

        let (assign18590_e18413, assign18590_e18413_d_n4, assign18590_e18413_d_n6, assign18590_e18413_d_n7, assign18590_e18413_d_n8, assign18590_e18413_d_n9,) = {
    if ((locals.var_guard633 == 0.0) && (locals.var_guard634 != 0.0)) {
        let assign18590_e18410: f64 = (0.5 * locals.var_q_qcoth);
        let assign18590_e18411: f64 = (1.0 - assign18590_e18410);
        (assign18590_e18411, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign18590_e18413;
        locals.var_q_temp2_dn4 = assign18590_e18413_d_n4;
        locals.var_q_temp2_dn6 = assign18590_e18413_d_n6;
        locals.var_q_temp2_dn7 = assign18590_e18413_d_n7;
        locals.var_q_temp2_dn8 = assign18590_e18413_d_n8;
        locals.var_q_temp2_dn9 = assign18590_e18413_d_n9;

        let (assign18600_e18424, assign18600_e18424_d_n4, assign18600_e18424_d_n6, assign18600_e18424_d_n7, assign18600_e18424_d_n8, assign18600_e18424_d_n9,) = {
    if ((locals.var_guard633 == 0.0) && (locals.var_guard634 != 0.0)) {
        let assign18600_e18420: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
        let assign18600_e18422: f64 = (assign18600_e18420 * locals.var_q_temp2);
        (assign18600_e18422, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign18600_e18420 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign18600_e18420 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign18600_e18420 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign18600_e18420 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign18600_e18420 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign18600_e18424;
        locals.var_q_d1_ln_dn4 = assign18600_e18424_d_n4;
        locals.var_q_d1_ln_dn6 = assign18600_e18424_d_n6;
        locals.var_q_d1_ln_dn7 = assign18600_e18424_d_n7;
        locals.var_q_d1_ln_dn8 = assign18600_e18424_d_n8;
        locals.var_q_d1_ln_dn9 = assign18600_e18424_d_n9;

        let (assign18610_e18443, assign18610_e18443_d_n4, assign18610_e18443_d_n6, assign18610_e18443_d_n7, assign18610_e18443_d_n8, assign18610_e18443_d_n9,) = {
    if ((locals.var_guard633 == 0.0) && (locals.var_guard634 != 0.0)) {
        let assign18610_e18431: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
        let assign18610_e18436: f64 = (0.5 * locals.var_q_d1_qcoth);
        let assign18610_e18437: f64 = (locals.var_q_d1_ln + assign18610_e18436);
        let assign18610_e18438: f64 = (locals.var_q_d1_qsq * assign18610_e18437);
        let assign18610_e18439: f64 = (assign18610_e18431 - assign18610_e18438);
        let assign18610_e18441: f64 = (assign18610_e18439 / locals.var_q_qsq);
        (assign18610_e18441, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign18610_e18437) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign18610_e18439 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign18610_e18437) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign18610_e18439 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign18610_e18437) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign18610_e18439 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign18610_e18437) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign18610_e18439 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign18610_e18437) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign18610_e18439 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign18610_e18443;
        locals.var_q_d2_ln_dn4 = assign18610_e18443_d_n4;
        locals.var_q_d2_ln_dn6 = assign18610_e18443_d_n6;
        locals.var_q_d2_ln_dn7 = assign18610_e18443_d_n7;
        locals.var_q_d2_ln_dn8 = assign18610_e18443_d_n8;
        locals.var_q_d2_ln_dn9 = assign18610_e18443_d_n9;

        let (assign18620_e18469, assign18620_e18469_d_n4, assign18620_e18469_d_n6, assign18620_e18469_d_n7, assign18620_e18469_d_n8, assign18620_e18469_d_n9,) = {
    if ((locals.var_guard633 == 0.0) && (locals.var_guard634 == 0.0)) {
        let assign18620_e18453: f64 = (locals.var_q_qsq * 0.0166666666667);
        let assign18620_e18457: f64 = (locals.var_q_qsq * 0.0238095238095);
        let assign18620_e18461: f64 = (locals.var_q_qsq * 0.025);
        let assign18620_e18462: f64 = (1.0 - assign18620_e18461);
        let assign18620_e18463: f64 = (assign18620_e18457 * assign18620_e18462);
        let assign18620_e18464: f64 = (1.0 - assign18620_e18463);
        let assign18620_e18465: f64 = (assign18620_e18453 * assign18620_e18464);
        let assign18620_e18466: f64 = (1.0 - assign18620_e18465);
        let assign18620_e18467: f64 = (0.1666666666667 * assign18620_e18466);
        (assign18620_e18467, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0166666666667) * assign18620_e18464) + (assign18620_e18453 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign18620_e18462) + (assign18620_e18457 * (-(locals.var_q_qsq_dn4 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0166666666667) * assign18620_e18464) + (assign18620_e18453 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign18620_e18462) + (assign18620_e18457 * (-(locals.var_q_qsq_dn6 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0166666666667) * assign18620_e18464) + (assign18620_e18453 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign18620_e18462) + (assign18620_e18457 * (-(locals.var_q_qsq_dn7 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0166666666667) * assign18620_e18464) + (assign18620_e18453 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign18620_e18462) + (assign18620_e18457 * (-(locals.var_q_qsq_dn8 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0166666666667) * assign18620_e18464) + (assign18620_e18453 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign18620_e18462) + (assign18620_e18457 * (-(locals.var_q_qsq_dn9 * 0.025))))))))),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign18620_e18469;
        locals.var_q_temp3_dn4 = assign18620_e18469_d_n4;
        locals.var_q_temp3_dn6 = assign18620_e18469_d_n6;
        locals.var_q_temp3_dn7 = assign18620_e18469_d_n7;
        locals.var_q_temp3_dn8 = assign18620_e18469_d_n8;
        locals.var_q_temp3_dn9 = assign18620_e18469_d_n9;

        let (assign18630_e18481, assign18630_e18481_d_n4, assign18630_e18481_d_n6, assign18630_e18481_d_n7, assign18630_e18481_d_n8, assign18630_e18481_d_n9,) = {
    if ((locals.var_guard633 == 0.0) && (locals.var_guard634 == 0.0)) {
        let assign18630_e18478: f64 = (locals.var_q_qsq * locals.var_q_temp3);
        let assign18630_e18479: f64 = (2.0 + assign18630_e18478);
        (assign18630_e18479, ((locals.var_q_qsq_dn4 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn4)), ((locals.var_q_qsq_dn6 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn6)), ((locals.var_q_qsq_dn7 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn7)), ((locals.var_q_qsq_dn8 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn8)), ((locals.var_q_qsq_dn9 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn9)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign18630_e18481;
        locals.var_q_qcoth_dn4 = assign18630_e18481_d_n4;
        locals.var_q_qcoth_dn6 = assign18630_e18481_d_n6;
        locals.var_q_qcoth_dn7 = assign18630_e18481_d_n7;
        locals.var_q_qcoth_dn8 = assign18630_e18481_d_n8;
        locals.var_q_qcoth_dn9 = assign18630_e18481_d_n9;

        let (assign18640_e18507, assign18640_e18507_d_n4, assign18640_e18507_d_n6, assign18640_e18507_d_n7, assign18640_e18507_d_n8, assign18640_e18507_d_n9,) = {
    if ((locals.var_guard633 == 0.0) && (locals.var_guard634 == 0.0)) {
        let assign18640_e18491: f64 = (locals.var_q_qsq * 0.0333333333333);
        let assign18640_e18495: f64 = (locals.var_q_qsq * 0.0357142857143);
        let assign18640_e18499: f64 = (locals.var_q_qsq * 0.0333333333333);
        let assign18640_e18500: f64 = (1.0 - assign18640_e18499);
        let assign18640_e18501: f64 = (assign18640_e18495 * assign18640_e18500);
        let assign18640_e18502: f64 = (1.0 - assign18640_e18501);
        let assign18640_e18503: f64 = (assign18640_e18491 * assign18640_e18502);
        let assign18640_e18504: f64 = (1.0 - assign18640_e18503);
        let assign18640_e18505: f64 = (0.1666666666667 * assign18640_e18504);
        (assign18640_e18505, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0333333333333) * assign18640_e18502) + (assign18640_e18491 * (-(((locals.var_q_qsq_dn4 * 0.0357142857143) * assign18640_e18500) + (assign18640_e18495 * (-(locals.var_q_qsq_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0333333333333) * assign18640_e18502) + (assign18640_e18491 * (-(((locals.var_q_qsq_dn6 * 0.0357142857143) * assign18640_e18500) + (assign18640_e18495 * (-(locals.var_q_qsq_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0333333333333) * assign18640_e18502) + (assign18640_e18491 * (-(((locals.var_q_qsq_dn7 * 0.0357142857143) * assign18640_e18500) + (assign18640_e18495 * (-(locals.var_q_qsq_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0333333333333) * assign18640_e18502) + (assign18640_e18491 * (-(((locals.var_q_qsq_dn8 * 0.0357142857143) * assign18640_e18500) + (assign18640_e18495 * (-(locals.var_q_qsq_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0333333333333) * assign18640_e18502) + (assign18640_e18491 * (-(((locals.var_q_qsq_dn9 * 0.0357142857143) * assign18640_e18500) + (assign18640_e18495 * (-(locals.var_q_qsq_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign18640_e18507;
        locals.var_q_temp1_dn4 = assign18640_e18507_d_n4;
        locals.var_q_temp1_dn6 = assign18640_e18507_d_n6;
        locals.var_q_temp1_dn7 = assign18640_e18507_d_n7;
        locals.var_q_temp1_dn8 = assign18640_e18507_d_n8;
        locals.var_q_temp1_dn9 = assign18640_e18507_d_n9;

        let (assign18650_e18517, assign18650_e18517_d_n4, assign18650_e18517_d_n6, assign18650_e18517_d_n7, assign18650_e18517_d_n8, assign18650_e18517_d_n9,) = {
    if ((locals.var_guard633 == 0.0) && (locals.var_guard634 == 0.0)) {
        let assign18650_e18515: f64 = (locals.var_q_d1_qsq * locals.var_q_temp1);
        (assign18650_e18515, ((locals.var_q_d1_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn4)), ((locals.var_q_d1_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn6)), ((locals.var_q_d1_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn7)), ((locals.var_q_d1_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn8)), ((locals.var_q_d1_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign18650_e18517;
        locals.var_q_d1_qcoth_dn4 = assign18650_e18517_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign18650_e18517_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign18650_e18517_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign18650_e18517_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign18650_e18517_d_n9;

        let (assign18660_e18543, assign18660_e18543_d_n4, assign18660_e18543_d_n6, assign18660_e18543_d_n7, assign18660_e18543_d_n8, assign18660_e18543_d_n9,) = {
    if ((locals.var_guard633 == 0.0) && (locals.var_guard634 == 0.0)) {
        let assign18660_e18527: f64 = (locals.var_q_qsq * 0.0714285714286);
        let assign18660_e18531: f64 = (0.05 * locals.var_q_qsq);
        let assign18660_e18535: f64 = (0.0420875420875421 * locals.var_q_qsq);
        let assign18660_e18536: f64 = (1.0 - assign18660_e18535);
        let assign18660_e18537: f64 = (assign18660_e18531 * assign18660_e18536);
        let assign18660_e18538: f64 = (1.0 - assign18660_e18537);
        let assign18660_e18539: f64 = (assign18660_e18527 * assign18660_e18538);
        let assign18660_e18540: f64 = (1.0 - assign18660_e18539);
        let assign18660_e18541: f64 = (0.0055555555556 * assign18660_e18540);
        (assign18660_e18541, (0.0055555555556 * (-(((locals.var_q_qsq_dn4 * 0.0714285714286) * assign18660_e18538) + (assign18660_e18527 * (-(((0.05 * locals.var_q_qsq_dn4) * assign18660_e18536) + (assign18660_e18531 * (-(0.0420875420875421 * locals.var_q_qsq_dn4))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn6 * 0.0714285714286) * assign18660_e18538) + (assign18660_e18527 * (-(((0.05 * locals.var_q_qsq_dn6) * assign18660_e18536) + (assign18660_e18531 * (-(0.0420875420875421 * locals.var_q_qsq_dn6))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn7 * 0.0714285714286) * assign18660_e18538) + (assign18660_e18527 * (-(((0.05 * locals.var_q_qsq_dn7) * assign18660_e18536) + (assign18660_e18531 * (-(0.0420875420875421 * locals.var_q_qsq_dn7))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn8 * 0.0714285714286) * assign18660_e18538) + (assign18660_e18527 * (-(((0.05 * locals.var_q_qsq_dn8) * assign18660_e18536) + (assign18660_e18531 * (-(0.0420875420875421 * locals.var_q_qsq_dn8))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn9 * 0.0714285714286) * assign18660_e18538) + (assign18660_e18527 * (-(((0.05 * locals.var_q_qsq_dn9) * assign18660_e18536) + (assign18660_e18531 * (-(0.0420875420875421 * locals.var_q_qsq_dn9))))))))),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign18660_e18543;
        locals.var_q_temp2_dn4 = assign18660_e18543_d_n4;
        locals.var_q_temp2_dn6 = assign18660_e18543_d_n6;
        locals.var_q_temp2_dn7 = assign18660_e18543_d_n7;
        locals.var_q_temp2_dn8 = assign18660_e18543_d_n8;
        locals.var_q_temp2_dn9 = assign18660_e18543_d_n9;

        let (assign18670_e18559, assign18670_e18559_d_n4, assign18670_e18559_d_n6, assign18670_e18559_d_n7, assign18670_e18559_d_n8, assign18670_e18559_d_n9,) = {
    if ((locals.var_guard633 == 0.0) && (locals.var_guard634 == 0.0)) {
        let assign18670_e18551: f64 = (locals.var_q_d2_qsq * locals.var_q_temp1);
        let assign18670_e18554: f64 = (locals.var_q_d1_qsq * locals.var_q_d1_qsq);
        let assign18670_e18556: f64 = (assign18670_e18554 * locals.var_q_temp2);
        let assign18670_e18557: f64 = (assign18670_e18551 - assign18670_e18556);
        (assign18670_e18557, (((locals.var_q_d2_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn4)) - ((((locals.var_q_d1_qsq_dn4 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn4)) * locals.var_q_temp2) + (assign18670_e18554 * locals.var_q_temp2_dn4))), (((locals.var_q_d2_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn6)) - ((((locals.var_q_d1_qsq_dn6 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn6)) * locals.var_q_temp2) + (assign18670_e18554 * locals.var_q_temp2_dn6))), (((locals.var_q_d2_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn7)) - ((((locals.var_q_d1_qsq_dn7 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn7)) * locals.var_q_temp2) + (assign18670_e18554 * locals.var_q_temp2_dn7))), (((locals.var_q_d2_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn8)) - ((((locals.var_q_d1_qsq_dn8 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn8)) * locals.var_q_temp2) + (assign18670_e18554 * locals.var_q_temp2_dn8))), (((locals.var_q_d2_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn9)) - ((((locals.var_q_d1_qsq_dn9 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn9)) * locals.var_q_temp2) + (assign18670_e18554 * locals.var_q_temp2_dn9))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign18670_e18559;
        locals.var_q_d2_qcoth_dn4 = assign18670_e18559_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign18670_e18559_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign18670_e18559_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign18670_e18559_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign18670_e18559_d_n9;

        let (assign18680_e18572, assign18680_e18572_d_n4, assign18680_e18572_d_n6, assign18680_e18572_d_n7, assign18680_e18572_d_n8, assign18680_e18572_d_n9,) = {
    if ((locals.var_guard633 == 0.0) && (locals.var_guard634 == 0.0)) {
        let assign18680_e18566: f64 = (-0.5);
        let assign18680_e18568: f64 = (assign18680_e18566 * locals.var_q_d1_qsq);
        let assign18680_e18570: f64 = (assign18680_e18568 * locals.var_q_temp3);
        (assign18680_e18570, (((assign18680_e18566 * locals.var_q_d1_qsq_dn4) * locals.var_q_temp3) + (assign18680_e18568 * locals.var_q_temp3_dn4)), (((assign18680_e18566 * locals.var_q_d1_qsq_dn6) * locals.var_q_temp3) + (assign18680_e18568 * locals.var_q_temp3_dn6)), (((assign18680_e18566 * locals.var_q_d1_qsq_dn7) * locals.var_q_temp3) + (assign18680_e18568 * locals.var_q_temp3_dn7)), (((assign18680_e18566 * locals.var_q_d1_qsq_dn8) * locals.var_q_temp3) + (assign18680_e18568 * locals.var_q_temp3_dn8)), (((assign18680_e18566 * locals.var_q_d1_qsq_dn9) * locals.var_q_temp3) + (assign18680_e18568 * locals.var_q_temp3_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign18680_e18572;
        locals.var_q_d1_ln_dn4 = assign18680_e18572_d_n4;
        locals.var_q_d1_ln_dn6 = assign18680_e18572_d_n6;
        locals.var_q_d1_ln_dn7 = assign18680_e18572_d_n7;
        locals.var_q_d1_ln_dn8 = assign18680_e18572_d_n8;
        locals.var_q_d1_ln_dn9 = assign18680_e18572_d_n9;

        let (assign18690_e18605, assign18690_e18605_d_n4, assign18690_e18605_d_n6, assign18690_e18605_d_n7, assign18690_e18605_d_n8, assign18690_e18605_d_n9,) = {
    if ((locals.var_guard633 == 0.0) && (locals.var_guard634 == 0.0)) {
        let assign18690_e18579: f64 = (-0.5);
        let assign18690_e18581: f64 = (assign18690_e18579 * locals.var_q_d2_qsq);
        let assign18690_e18583: f64 = (assign18690_e18581 * locals.var_q_temp3);
        let assign18690_e18586: f64 = (0.25 * 0.0055555555556);
        let assign18690_e18588: f64 = (assign18690_e18586 * locals.var_q_d1_qsq);
        let assign18690_e18590: f64 = (assign18690_e18588 * locals.var_q_d1_qsq);
        let assign18690_e18594: f64 = (locals.var_q_qsq * 0.0238095238095);
        let assign18690_e18598: f64 = (0.075 * locals.var_q_qsq);
        let assign18690_e18599: f64 = (2.0 - assign18690_e18598);
        let assign18690_e18600: f64 = (assign18690_e18594 * assign18690_e18599);
        let assign18690_e18601: f64 = (1.0 - assign18690_e18600);
        let assign18690_e18602: f64 = (assign18690_e18590 * assign18690_e18601);
        let assign18690_e18603: f64 = (assign18690_e18583 + assign18690_e18602);
        (assign18690_e18603, ((((assign18690_e18579 * locals.var_q_d2_qsq_dn4) * locals.var_q_temp3) + (assign18690_e18581 * locals.var_q_temp3_dn4)) + (((((assign18690_e18586 * locals.var_q_d1_qsq_dn4) * locals.var_q_d1_qsq) + (assign18690_e18588 * locals.var_q_d1_qsq_dn4)) * assign18690_e18601) + (assign18690_e18590 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign18690_e18599) + (assign18690_e18594 * (-(0.075 * locals.var_q_qsq_dn4)))))))), ((((assign18690_e18579 * locals.var_q_d2_qsq_dn6) * locals.var_q_temp3) + (assign18690_e18581 * locals.var_q_temp3_dn6)) + (((((assign18690_e18586 * locals.var_q_d1_qsq_dn6) * locals.var_q_d1_qsq) + (assign18690_e18588 * locals.var_q_d1_qsq_dn6)) * assign18690_e18601) + (assign18690_e18590 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign18690_e18599) + (assign18690_e18594 * (-(0.075 * locals.var_q_qsq_dn6)))))))), ((((assign18690_e18579 * locals.var_q_d2_qsq_dn7) * locals.var_q_temp3) + (assign18690_e18581 * locals.var_q_temp3_dn7)) + (((((assign18690_e18586 * locals.var_q_d1_qsq_dn7) * locals.var_q_d1_qsq) + (assign18690_e18588 * locals.var_q_d1_qsq_dn7)) * assign18690_e18601) + (assign18690_e18590 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign18690_e18599) + (assign18690_e18594 * (-(0.075 * locals.var_q_qsq_dn7)))))))), ((((assign18690_e18579 * locals.var_q_d2_qsq_dn8) * locals.var_q_temp3) + (assign18690_e18581 * locals.var_q_temp3_dn8)) + (((((assign18690_e18586 * locals.var_q_d1_qsq_dn8) * locals.var_q_d1_qsq) + (assign18690_e18588 * locals.var_q_d1_qsq_dn8)) * assign18690_e18601) + (assign18690_e18590 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign18690_e18599) + (assign18690_e18594 * (-(0.075 * locals.var_q_qsq_dn8)))))))), ((((assign18690_e18579 * locals.var_q_d2_qsq_dn9) * locals.var_q_temp3) + (assign18690_e18581 * locals.var_q_temp3_dn9)) + (((((assign18690_e18586 * locals.var_q_d1_qsq_dn9) * locals.var_q_d1_qsq) + (assign18690_e18588 * locals.var_q_d1_qsq_dn9)) * assign18690_e18601) + (assign18690_e18590 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign18690_e18599) + (assign18690_e18594 * (-(0.075 * locals.var_q_qsq_dn9)))))))),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign18690_e18605;
        locals.var_q_d2_ln_dn4 = assign18690_e18605_d_n4;
        locals.var_q_d2_ln_dn6 = assign18690_e18605_d_n6;
        locals.var_q_d2_ln_dn7 = assign18690_e18605_d_n7;
        locals.var_q_d2_ln_dn8 = assign18690_e18605_d_n8;
        locals.var_q_d2_ln_dn9 = assign18690_e18605_d_n9;

        let assign18700_e18608: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard635 = assign18700_e18608;

        let (assign18710_e18622, assign18710_e18622_d_n4, assign18710_e18622_d_n6, assign18710_e18622_d_n7, assign18710_e18622_d_n8, assign18710_e18622_d_n9,) = {
    if (locals.var_guard635 != 0.0) {
        let assign18710_e18612: f64 = (4.0 * locals.var_q_qsq);
        let assign18710_e18617: f64 = (2.0 - locals.var_q_invexpq);
        let assign18710_e18618: f64 = (locals.var_q_invexpq * assign18710_e18617);
        let assign18710_e18619: f64 = (1.0 - assign18710_e18618);
        let assign18710_e18620: f64 = (assign18710_e18612 / assign18710_e18619);
        (assign18710_e18620, ((((4.0 * locals.var_q_qsq_dn4) * assign18710_e18619) - (assign18710_e18612 * (-((locals.var_q_invexpq_dn4 * assign18710_e18617) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn4)))))) / (assign18710_e18619 * assign18710_e18619)), ((((4.0 * locals.var_q_qsq_dn6) * assign18710_e18619) - (assign18710_e18612 * (-((locals.var_q_invexpq_dn6 * assign18710_e18617) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn6)))))) / (assign18710_e18619 * assign18710_e18619)), ((((4.0 * locals.var_q_qsq_dn7) * assign18710_e18619) - (assign18710_e18612 * (-((locals.var_q_invexpq_dn7 * assign18710_e18617) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn7)))))) / (assign18710_e18619 * assign18710_e18619)), ((((4.0 * locals.var_q_qsq_dn8) * assign18710_e18619) - (assign18710_e18612 * (-((locals.var_q_invexpq_dn8 * assign18710_e18617) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn8)))))) / (assign18710_e18619 * assign18710_e18619)), ((((4.0 * locals.var_q_qsq_dn9) * assign18710_e18619) - (assign18710_e18612 * (-((locals.var_q_invexpq_dn9 * assign18710_e18617) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn9)))))) / (assign18710_e18619 * assign18710_e18619)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign18710_e18622;
        locals.var_q_temp2_dn4 = assign18710_e18622_d_n4;
        locals.var_q_temp2_dn6 = assign18710_e18622_d_n6;
        locals.var_q_temp2_dn7 = assign18710_e18622_d_n7;
        locals.var_q_temp2_dn8 = assign18710_e18622_d_n8;
        locals.var_q_temp2_dn9 = assign18710_e18622_d_n9;

    }

    pub(super) fn stamp_transient_block_47(
        locals: &mut StampLocals,
    ) {
        let (assign18720_e18628, assign18720_e18628_d_n4, assign18720_e18628_d_n6, assign18720_e18628_d_n7, assign18720_e18628_d_n8, assign18720_e18628_d_n9,) = {
    if (locals.var_guard635 != 0.0) {
        let assign18720_e18626: f64 = (locals.var_q_temp2 * locals.var_q_invexpq);
        (assign18720_e18626, ((locals.var_q_temp2_dn4 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn4)), ((locals.var_q_temp2_dn6 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn6)), ((locals.var_q_temp2_dn7 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn7)), ((locals.var_q_temp2_dn8 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn8)), ((locals.var_q_temp2_dn9 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn9)),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign18720_e18628;
        locals.var_q_sh_term_dn4 = assign18720_e18628_d_n4;
        locals.var_q_sh_term_dn6 = assign18720_e18628_d_n6;
        locals.var_q_sh_term_dn7 = assign18720_e18628_d_n7;
        locals.var_q_sh_term_dn8 = assign18720_e18628_d_n8;
        locals.var_q_sh_term_dn9 = assign18720_e18628_d_n9;

        let (assign18730_e18635, assign18730_e18635_d_n4, assign18730_e18635_d_n6, assign18730_e18635_d_n7, assign18730_e18635_d_n8, assign18730_e18635_d_n9,) = {
    if (locals.var_guard635 != 0.0) {
        let assign18730_e18631: f64 = (locals.var_q_temp2).ln();
        let assign18730_e18633: f64 = (assign18730_e18631 - locals.var_q_rac_qsq);
        (assign18730_e18633, ((locals.var_q_temp2_dn4 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn4), ((locals.var_q_temp2_dn6 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn6), ((locals.var_q_temp2_dn7 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn7), ((locals.var_q_temp2_dn8 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn8), ((locals.var_q_temp2_dn9 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn9),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign18730_e18635;
        locals.var_q_ln_term_dn4 = assign18730_e18635_d_n4;
        locals.var_q_ln_term_dn6 = assign18730_e18635_d_n6;
        locals.var_q_ln_term_dn7 = assign18730_e18635_d_n7;
        locals.var_q_ln_term_dn8 = assign18730_e18635_d_n8;
        locals.var_q_ln_term_dn9 = assign18730_e18635_d_n9;

        let assign18740_e18638: f64 = (-0.005);
        let assign18740_e18639: f64 = if locals.var_q_qsq < assign18740_e18638 { 1.0 } else { 0.0 };
        locals.var_guard636 = assign18740_e18639;

        let (assign18750_e18649, assign18750_e18649_d_n4, assign18750_e18649_d_n6, assign18750_e18649_d_n7, assign18750_e18649_d_n8, assign18750_e18649_d_n9,) = {
    if ((locals.var_guard635 == 0.0) && (locals.var_guard636 != 0.0)) {
        let assign18750_e18646: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign18750_e18647: f64 = (assign18750_e18646).sin();
        (assign18750_e18647, ((assign18750_e18646).cos() * (0.5 * locals.var_q_rac_qsq_dn4)), ((assign18750_e18646).cos() * (0.5 * locals.var_q_rac_qsq_dn6)), ((assign18750_e18646).cos() * (0.5 * locals.var_q_rac_qsq_dn7)), ((assign18750_e18646).cos() * (0.5 * locals.var_q_rac_qsq_dn8)), ((assign18750_e18646).cos() * (0.5 * locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign18750_e18649;
        locals.var_q_temp2_dn4 = assign18750_e18649_d_n4;
        locals.var_q_temp2_dn6 = assign18750_e18649_d_n6;
        locals.var_q_temp2_dn7 = assign18750_e18649_d_n7;
        locals.var_q_temp2_dn8 = assign18750_e18649_d_n8;
        locals.var_q_temp2_dn9 = assign18750_e18649_d_n9;

        let (assign18760_e18661, assign18760_e18661_d_n4, assign18760_e18661_d_n6, assign18760_e18661_d_n7, assign18760_e18661_d_n8, assign18760_e18661_d_n9,) = {
    if ((locals.var_guard635 == 0.0) && (locals.var_guard636 != 0.0)) {
        let assign18760_e18655: f64 = (-locals.var_q_qsq);
        let assign18760_e18658: f64 = (locals.var_q_temp2 * locals.var_q_temp2);
        let assign18760_e18659: f64 = (assign18760_e18655 / assign18760_e18658);
        (assign18760_e18659, ((((-locals.var_q_qsq_dn4) * assign18760_e18658) - (assign18760_e18655 * ((locals.var_q_temp2_dn4 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn4)))) / (assign18760_e18658 * assign18760_e18658)), ((((-locals.var_q_qsq_dn6) * assign18760_e18658) - (assign18760_e18655 * ((locals.var_q_temp2_dn6 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn6)))) / (assign18760_e18658 * assign18760_e18658)), ((((-locals.var_q_qsq_dn7) * assign18760_e18658) - (assign18760_e18655 * ((locals.var_q_temp2_dn7 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn7)))) / (assign18760_e18658 * assign18760_e18658)), ((((-locals.var_q_qsq_dn8) * assign18760_e18658) - (assign18760_e18655 * ((locals.var_q_temp2_dn8 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn8)))) / (assign18760_e18658 * assign18760_e18658)), ((((-locals.var_q_qsq_dn9) * assign18760_e18658) - (assign18760_e18655 * ((locals.var_q_temp2_dn9 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn9)))) / (assign18760_e18658 * assign18760_e18658)),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign18760_e18661;
        locals.var_q_sh_term_dn4 = assign18760_e18661_d_n4;
        locals.var_q_sh_term_dn6 = assign18760_e18661_d_n6;
        locals.var_q_sh_term_dn7 = assign18760_e18661_d_n7;
        locals.var_q_sh_term_dn8 = assign18760_e18661_d_n8;
        locals.var_q_sh_term_dn9 = assign18760_e18661_d_n9;

        let (assign18770_e18669, assign18770_e18669_d_n4, assign18770_e18669_d_n6, assign18770_e18669_d_n7, assign18770_e18669_d_n8, assign18770_e18669_d_n9,) = {
    if ((locals.var_guard635 == 0.0) && (locals.var_guard636 != 0.0)) {
        let assign18770_e18667: f64 = (locals.var_q_sh_term).ln();
        (assign18770_e18667, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign18770_e18669;
        locals.var_q_ln_term_dn4 = assign18770_e18669_d_n4;
        locals.var_q_ln_term_dn6 = assign18770_e18669_d_n6;
        locals.var_q_ln_term_dn7 = assign18770_e18669_d_n7;
        locals.var_q_ln_term_dn8 = assign18770_e18669_d_n8;
        locals.var_q_ln_term_dn9 = assign18770_e18669_d_n9;

        let (assign18780_e18693, assign18780_e18693_d_n4, assign18780_e18693_d_n6, assign18780_e18693_d_n7, assign18780_e18693_d_n8, assign18780_e18693_d_n9,) = {
    if ((locals.var_guard635 == 0.0) && (locals.var_guard636 == 0.0)) {
        let assign18780_e18678: f64 = (locals.var_q_qsq * 0.3333333333333);
        let assign18780_e18682: f64 = (0.05 * locals.var_q_qsq);
        let assign18780_e18686: f64 = (0.0396825396825397 * locals.var_q_qsq);
        let assign18780_e18687: f64 = (1.0 - assign18780_e18686);
        let assign18780_e18688: f64 = (assign18780_e18682 * assign18780_e18687);
        let assign18780_e18689: f64 = (1.0 - assign18780_e18688);
        let assign18780_e18690: f64 = (assign18780_e18678 * assign18780_e18689);
        let assign18780_e18691: f64 = (4.0 - assign18780_e18690);
        (assign18780_e18691, (-(((locals.var_q_qsq_dn4 * 0.3333333333333) * assign18780_e18689) + (assign18780_e18678 * (-(((0.05 * locals.var_q_qsq_dn4) * assign18780_e18687) + (assign18780_e18682 * (-(0.0396825396825397 * locals.var_q_qsq_dn4)))))))), (-(((locals.var_q_qsq_dn6 * 0.3333333333333) * assign18780_e18689) + (assign18780_e18678 * (-(((0.05 * locals.var_q_qsq_dn6) * assign18780_e18687) + (assign18780_e18682 * (-(0.0396825396825397 * locals.var_q_qsq_dn6)))))))), (-(((locals.var_q_qsq_dn7 * 0.3333333333333) * assign18780_e18689) + (assign18780_e18678 * (-(((0.05 * locals.var_q_qsq_dn7) * assign18780_e18687) + (assign18780_e18682 * (-(0.0396825396825397 * locals.var_q_qsq_dn7)))))))), (-(((locals.var_q_qsq_dn8 * 0.3333333333333) * assign18780_e18689) + (assign18780_e18678 * (-(((0.05 * locals.var_q_qsq_dn8) * assign18780_e18687) + (assign18780_e18682 * (-(0.0396825396825397 * locals.var_q_qsq_dn8)))))))), (-(((locals.var_q_qsq_dn9 * 0.3333333333333) * assign18780_e18689) + (assign18780_e18678 * (-(((0.05 * locals.var_q_qsq_dn9) * assign18780_e18687) + (assign18780_e18682 * (-(0.0396825396825397 * locals.var_q_qsq_dn9)))))))),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign18780_e18693;
        locals.var_q_sh_term_dn4 = assign18780_e18693_d_n4;
        locals.var_q_sh_term_dn6 = assign18780_e18693_d_n6;
        locals.var_q_sh_term_dn7 = assign18780_e18693_d_n7;
        locals.var_q_sh_term_dn8 = assign18780_e18693_d_n8;
        locals.var_q_sh_term_dn9 = assign18780_e18693_d_n9;

        let (assign18790_e18702, assign18790_e18702_d_n4, assign18790_e18702_d_n6, assign18790_e18702_d_n7, assign18790_e18702_d_n8, assign18790_e18702_d_n9,) = {
    if ((locals.var_guard635 == 0.0) && (locals.var_guard636 == 0.0)) {
        let assign18790_e18700: f64 = (locals.var_q_sh_term).ln();
        (assign18790_e18700, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign18790_e18702;
        locals.var_q_ln_term_dn4 = assign18790_e18702_d_n4;
        locals.var_q_ln_term_dn6 = assign18790_e18702_d_n6;
        locals.var_q_ln_term_dn7 = assign18790_e18702_d_n7;
        locals.var_q_ln_term_dn8 = assign18790_e18702_d_n8;
        locals.var_q_ln_term_dn9 = assign18790_e18702_d_n9;

        let assign18800_e18705: f64 = (1.01 * locals.var_q_k1q1);
        let assign18800_e18707: f64 = (assign18800_e18705 + locals.var_q_qcoth);
        let assign18800_e18709: f64 = if assign18800_e18707 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard637 = assign18800_e18709;

        let (assign18810_e18715, assign18810_e18715_d_n4, assign18810_e18715_d_n6, assign18810_e18715_d_n7, assign18810_e18715_d_n8, assign18810_e18715_d_n9,) = {
    if (locals.var_guard637 != 0.0) {
        let assign18810_e18713: f64 = (locals.var_q_k1q1 + locals.var_q_qcoth);
        (assign18810_e18713, (locals.var_q_k1q1_dn4 + locals.var_q_qcoth_dn4), (locals.var_q_k1q1_dn6 + locals.var_q_qcoth_dn6), (locals.var_q_k1q1_dn7 + locals.var_q_qcoth_dn7), (locals.var_q_k1q1_dn8 + locals.var_q_qcoth_dn8), (locals.var_q_k1q1_dn9 + locals.var_q_qcoth_dn9),)
    } else {
        (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9,)
    }
};
        locals.var_q_expnum = assign18810_e18715;
        locals.var_q_expnum_dn4 = assign18810_e18715_d_n4;
        locals.var_q_expnum_dn6 = assign18810_e18715_d_n6;
        locals.var_q_expnum_dn7 = assign18810_e18715_d_n7;
        locals.var_q_expnum_dn8 = assign18810_e18715_d_n8;
        locals.var_q_expnum_dn9 = assign18810_e18715_d_n9;

        let (assign18820_e18721, assign18820_e18721_d_n4, assign18820_e18721_d_n6, assign18820_e18721_d_n7, assign18820_e18721_d_n8, assign18820_e18721_d_n9,) = {
    if (locals.var_guard637 != 0.0) {
        let assign18820_e18719: f64 = (locals.var_k1 + locals.var_q_d1_qcoth);
        (assign18820_e18719, (locals.var_k1_dn4 + locals.var_q_d1_qcoth_dn4), (locals.var_k1_dn6 + locals.var_q_d1_qcoth_dn6), (locals.var_k1_dn7 + locals.var_q_d1_qcoth_dn7), (locals.var_k1_dn8 + locals.var_q_d1_qcoth_dn8), (locals.var_k1_dn9 + locals.var_q_d1_qcoth_dn9),)
    } else {
        (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9,)
    }
};
        locals.var_q_d1_expnum = assign18820_e18721;
        locals.var_q_d1_expnum_dn4 = assign18820_e18721_d_n4;
        locals.var_q_d1_expnum_dn6 = assign18820_e18721_d_n6;
        locals.var_q_d1_expnum_dn7 = assign18820_e18721_d_n7;
        locals.var_q_d1_expnum_dn8 = assign18820_e18721_d_n8;
        locals.var_q_d1_expnum_dn9 = assign18820_e18721_d_n9;

        let (assign18830_e18725, assign18830_e18725_d_n4, assign18830_e18725_d_n6, assign18830_e18725_d_n7, assign18830_e18725_d_n8, assign18830_e18725_d_n9,) = {
    if (locals.var_guard637 != 0.0) {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    } else {
        (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9,)
    }
};
        locals.var_q_d2_expnum = assign18830_e18725;
        locals.var_q_d2_expnum_dn4 = assign18830_e18725_d_n4;
        locals.var_q_d2_expnum_dn6 = assign18830_e18725_d_n6;
        locals.var_q_d2_expnum_dn7 = assign18830_e18725_d_n7;
        locals.var_q_d2_expnum_dn8 = assign18830_e18725_d_n8;
        locals.var_q_d2_expnum_dn9 = assign18830_e18725_d_n9;

        let (assign18840_e18734, assign18840_e18734_d_n4, assign18840_e18734_d_n6, assign18840_e18734_d_n7, assign18840_e18734_d_n8, assign18840_e18734_d_n9,) = {
    if (locals.var_guard637 == 0.0) {
        let assign18840_e18731: f64 = (locals.var_q_k1q1 - locals.var_q_qcoth);
        let assign18840_e18732: f64 = (1.0 / assign18840_e18731);
        (assign18840_e18732, (-((locals.var_q_k1q1_dn4 - locals.var_q_qcoth_dn4) / (assign18840_e18731 * assign18840_e18731))), (-((locals.var_q_k1q1_dn6 - locals.var_q_qcoth_dn6) / (assign18840_e18731 * assign18840_e18731))), (-((locals.var_q_k1q1_dn7 - locals.var_q_qcoth_dn7) / (assign18840_e18731 * assign18840_e18731))), (-((locals.var_q_k1q1_dn8 - locals.var_q_qcoth_dn8) / (assign18840_e18731 * assign18840_e18731))), (-((locals.var_q_k1q1_dn9 - locals.var_q_qcoth_dn9) / (assign18840_e18731 * assign18840_e18731))),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign18840_e18734;
        locals.var_q_temp2_dn4 = assign18840_e18734_d_n4;
        locals.var_q_temp2_dn6 = assign18840_e18734_d_n6;
        locals.var_q_temp2_dn7 = assign18840_e18734_d_n7;
        locals.var_q_temp2_dn8 = assign18840_e18734_d_n8;
        locals.var_q_temp2_dn9 = assign18840_e18734_d_n9;

        let (assign18850_e18741, assign18850_e18741_d_n4, assign18850_e18741_d_n6, assign18850_e18741_d_n7, assign18850_e18741_d_n8, assign18850_e18741_d_n9,) = {
    if (locals.var_guard637 == 0.0) {
        let assign18850_e18739: f64 = (locals.var_q_d1_qcoth - locals.var_k1);
        (assign18850_e18739, (locals.var_q_d1_qcoth_dn4 - locals.var_k1_dn4), (locals.var_q_d1_qcoth_dn6 - locals.var_k1_dn6), (locals.var_q_d1_qcoth_dn7 - locals.var_k1_dn7), (locals.var_q_d1_qcoth_dn8 - locals.var_k1_dn8), (locals.var_q_d1_qcoth_dn9 - locals.var_k1_dn9),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign18850_e18741;
        locals.var_q_temp3_dn4 = assign18850_e18741_d_n4;
        locals.var_q_temp3_dn6 = assign18850_e18741_d_n6;
        locals.var_q_temp3_dn7 = assign18850_e18741_d_n7;
        locals.var_q_temp3_dn8 = assign18850_e18741_d_n8;
        locals.var_q_temp3_dn9 = assign18850_e18741_d_n9;

        let (assign18860_e18750, assign18860_e18750_d_n4, assign18860_e18750_d_n6, assign18860_e18750_d_n7, assign18860_e18750_d_n8, assign18860_e18750_d_n9,) = {
    if (locals.var_guard637 == 0.0) {
        let assign18860_e18746: f64 = (locals.var_q_aexp - locals.var_q_sh_term);
        let assign18860_e18748: f64 = (assign18860_e18746 * locals.var_q_temp2);
        (assign18860_e18748, (((locals.var_q_aexp_dn4 - locals.var_q_sh_term_dn4) * locals.var_q_temp2) + (assign18860_e18746 * locals.var_q_temp2_dn4)), (((locals.var_q_aexp_dn6 - locals.var_q_sh_term_dn6) * locals.var_q_temp2) + (assign18860_e18746 * locals.var_q_temp2_dn6)), (((locals.var_q_aexp_dn7 - locals.var_q_sh_term_dn7) * locals.var_q_temp2) + (assign18860_e18746 * locals.var_q_temp2_dn7)), (((locals.var_q_aexp_dn8 - locals.var_q_sh_term_dn8) * locals.var_q_temp2) + (assign18860_e18746 * locals.var_q_temp2_dn8)), (((locals.var_q_aexp_dn9 - locals.var_q_sh_term_dn9) * locals.var_q_temp2) + (assign18860_e18746 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9,)
    }
};
        locals.var_q_expnum = assign18860_e18750;
        locals.var_q_expnum_dn4 = assign18860_e18750_d_n4;
        locals.var_q_expnum_dn6 = assign18860_e18750_d_n6;
        locals.var_q_expnum_dn7 = assign18860_e18750_d_n7;
        locals.var_q_expnum_dn8 = assign18860_e18750_d_n8;
        locals.var_q_expnum_dn9 = assign18860_e18750_d_n9;

        let (assign18870_e18765, assign18870_e18765_d_n4, assign18870_e18765_d_n6, assign18870_e18765_d_n7, assign18870_e18765_d_n8, assign18870_e18765_d_n9,) = {
    if (locals.var_guard637 == 0.0) {
        let assign18870_e18755: f64 = (locals.var_q_temp3 * locals.var_q_expnum);
        let assign18870_e18757: f64 = (assign18870_e18755 - locals.var_q_aexp);
        let assign18870_e18760: f64 = (locals.var_q_d1_ln * locals.var_q_sh_term);
        let assign18870_e18761: f64 = (assign18870_e18757 - assign18870_e18760);
        let assign18870_e18763: f64 = (assign18870_e18761 * locals.var_q_temp2);
        (assign18870_e18763, ((((((locals.var_q_temp3_dn4 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4) - ((locals.var_q_d1_ln_dn4 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign18870_e18761 * locals.var_q_temp2_dn4)), ((((((locals.var_q_temp3_dn6 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6) - ((locals.var_q_d1_ln_dn6 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign18870_e18761 * locals.var_q_temp2_dn6)), ((((((locals.var_q_temp3_dn7 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7) - ((locals.var_q_d1_ln_dn7 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign18870_e18761 * locals.var_q_temp2_dn7)), ((((((locals.var_q_temp3_dn8 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8) - ((locals.var_q_d1_ln_dn8 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign18870_e18761 * locals.var_q_temp2_dn8)), ((((((locals.var_q_temp3_dn9 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9) - ((locals.var_q_d1_ln_dn9 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign18870_e18761 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9,)
    }
};
        locals.var_q_d1_expnum = assign18870_e18765;
        locals.var_q_d1_expnum_dn4 = assign18870_e18765_d_n4;
        locals.var_q_d1_expnum_dn6 = assign18870_e18765_d_n6;
        locals.var_q_d1_expnum_dn7 = assign18870_e18765_d_n7;
        locals.var_q_d1_expnum_dn8 = assign18870_e18765_d_n8;
        locals.var_q_d1_expnum_dn9 = assign18870_e18765_d_n9;

        let (assign18880_e18790, assign18880_e18790_d_n4, assign18880_e18790_d_n6, assign18880_e18790_d_n7, assign18880_e18790_d_n8, assign18880_e18790_d_n9,) = {
    if (locals.var_guard637 == 0.0) {
        let assign18880_e18770: f64 = (locals.var_q_d2_qcoth * locals.var_q_expnum);
        let assign18880_e18773: f64 = (2.0 * locals.var_q_temp3);
        let assign18880_e18775: f64 = (assign18880_e18773 * locals.var_q_d1_expnum);
        let assign18880_e18776: f64 = (assign18880_e18770 + assign18880_e18775);
        let assign18880_e18778: f64 = (assign18880_e18776 + locals.var_q_aexp);
        let assign18880_e18782: f64 = (locals.var_q_d1_ln * locals.var_q_d1_ln);
        let assign18880_e18783: f64 = (locals.var_q_d2_ln + assign18880_e18782);
        let assign18880_e18785: f64 = (assign18880_e18783 * locals.var_q_sh_term);
        let assign18880_e18786: f64 = (assign18880_e18778 - assign18880_e18785);
        let assign18880_e18788: f64 = (assign18880_e18786 * locals.var_q_temp2);
        (assign18880_e18788, (((((((locals.var_q_d2_qcoth_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_temp3_dn4) * locals.var_q_d1_expnum) + (assign18880_e18773 * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4) - (((locals.var_q_d2_ln_dn4 + ((locals.var_q_d1_ln_dn4 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn4))) * locals.var_q_sh_term) + (assign18880_e18783 * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign18880_e18786 * locals.var_q_temp2_dn4)), (((((((locals.var_q_d2_qcoth_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_temp3_dn6) * locals.var_q_d1_expnum) + (assign18880_e18773 * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6) - (((locals.var_q_d2_ln_dn6 + ((locals.var_q_d1_ln_dn6 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn6))) * locals.var_q_sh_term) + (assign18880_e18783 * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign18880_e18786 * locals.var_q_temp2_dn6)), (((((((locals.var_q_d2_qcoth_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_temp3_dn7) * locals.var_q_d1_expnum) + (assign18880_e18773 * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7) - (((locals.var_q_d2_ln_dn7 + ((locals.var_q_d1_ln_dn7 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn7))) * locals.var_q_sh_term) + (assign18880_e18783 * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign18880_e18786 * locals.var_q_temp2_dn7)), (((((((locals.var_q_d2_qcoth_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_temp3_dn8) * locals.var_q_d1_expnum) + (assign18880_e18773 * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8) - (((locals.var_q_d2_ln_dn8 + ((locals.var_q_d1_ln_dn8 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn8))) * locals.var_q_sh_term) + (assign18880_e18783 * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign18880_e18786 * locals.var_q_temp2_dn8)), (((((((locals.var_q_d2_qcoth_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_temp3_dn9) * locals.var_q_d1_expnum) + (assign18880_e18773 * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9) - (((locals.var_q_d2_ln_dn9 + ((locals.var_q_d1_ln_dn9 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn9))) * locals.var_q_sh_term) + (assign18880_e18783 * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign18880_e18786 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9,)
    }
};
        locals.var_q_d2_expnum = assign18880_e18790;
        locals.var_q_d2_expnum_dn4 = assign18880_e18790_d_n4;
        locals.var_q_d2_expnum_dn6 = assign18880_e18790_d_n6;
        locals.var_q_d2_expnum_dn7 = assign18880_e18790_d_n7;
        locals.var_q_d2_expnum_dn8 = assign18880_e18790_d_n8;
        locals.var_q_d2_expnum_dn9 = assign18880_e18790_d_n9;

        let assign18890_e18793: f64 = if locals.var_q_expnum > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard638 = assign18890_e18793;

        let (assign18900_e18798, assign18900_e18798_d_n4, assign18900_e18798_d_n6, assign18900_e18798_d_n7, assign18900_e18798_d_n8, assign18900_e18798_d_n9,) = {
    if (locals.var_guard638 != 0.0) {
        let assign18900_e18796: f64 = (locals.var_q_expnum).ln();
        (assign18900_e18796, (locals.var_q_expnum_dn4 / locals.var_q_expnum), (locals.var_q_expnum_dn6 / locals.var_q_expnum), (locals.var_q_expnum_dn7 / locals.var_q_expnum), (locals.var_q_expnum_dn8 / locals.var_q_expnum), (locals.var_q_expnum_dn9 / locals.var_q_expnum),)
    } else {
        (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9,)
    }
};
        locals.var_q_lnexpnum = assign18900_e18798;
        locals.var_q_lnexpnum_dn4 = assign18900_e18798_d_n4;
        locals.var_q_lnexpnum_dn6 = assign18900_e18798_d_n6;
        locals.var_q_lnexpnum_dn7 = assign18900_e18798_d_n7;
        locals.var_q_lnexpnum_dn8 = assign18900_e18798_d_n8;
        locals.var_q_lnexpnum_dn9 = assign18900_e18798_d_n9;

        let (assign18910_e18804, assign18910_e18804_d_n4, assign18910_e18804_d_n6, assign18910_e18804_d_n7, assign18910_e18804_d_n8, assign18910_e18804_d_n9,) = {
    if (locals.var_guard638 != 0.0) {
        let assign18910_e18802: f64 = (1.0 / locals.var_q_expnum);
        (assign18910_e18802, (-(locals.var_q_expnum_dn4 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn6 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn7 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn8 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn9 / (locals.var_q_expnum * locals.var_q_expnum))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign18910_e18804;
        locals.var_q_temp1_dn4 = assign18910_e18804_d_n4;
        locals.var_q_temp1_dn6 = assign18910_e18804_d_n6;
        locals.var_q_temp1_dn7 = assign18910_e18804_d_n7;
        locals.var_q_temp1_dn8 = assign18910_e18804_d_n8;
        locals.var_q_temp1_dn9 = assign18910_e18804_d_n9;

        let (assign18920_e18810, assign18920_e18810_d_n4, assign18920_e18810_d_n6, assign18920_e18810_d_n7, assign18920_e18810_d_n8, assign18920_e18810_d_n9,) = {
    if (locals.var_guard638 != 0.0) {
        let assign18920_e18808: f64 = (locals.var_q_d1_expnum * locals.var_q_temp1);
        (assign18920_e18808, ((locals.var_q_d1_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn4)), ((locals.var_q_d1_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn6)), ((locals.var_q_d1_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn7)), ((locals.var_q_d1_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn8)), ((locals.var_q_d1_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9,)
    }
};
        locals.var_q_d1_lnexpnum = assign18920_e18810;
        locals.var_q_d1_lnexpnum_dn4 = assign18920_e18810_d_n4;
        locals.var_q_d1_lnexpnum_dn6 = assign18920_e18810_d_n6;
        locals.var_q_d1_lnexpnum_dn7 = assign18920_e18810_d_n7;
        locals.var_q_d1_lnexpnum_dn8 = assign18920_e18810_d_n8;
        locals.var_q_d1_lnexpnum_dn9 = assign18920_e18810_d_n9;

        let (assign18930_e18820, assign18930_e18820_d_n4, assign18930_e18820_d_n6, assign18930_e18820_d_n7, assign18930_e18820_d_n8, assign18930_e18820_d_n9,) = {
    if (locals.var_guard638 != 0.0) {
        let assign18930_e18814: f64 = (locals.var_q_d2_expnum * locals.var_q_temp1);
        let assign18930_e18817: f64 = (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum);
        let assign18930_e18818: f64 = (assign18930_e18814 - assign18930_e18817);
        (assign18930_e18818, (((locals.var_q_d2_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn4)) - ((locals.var_q_d1_lnexpnum_dn4 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn4))), (((locals.var_q_d2_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn6)) - ((locals.var_q_d1_lnexpnum_dn6 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn6))), (((locals.var_q_d2_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn7)) - ((locals.var_q_d1_lnexpnum_dn7 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn7))), (((locals.var_q_d2_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn8)) - ((locals.var_q_d1_lnexpnum_dn8 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn8))), (((locals.var_q_d2_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn9)) - ((locals.var_q_d1_lnexpnum_dn9 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn9))),)
    } else {
        (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9,)
    }
};
        locals.var_q_d2_lnexpnum = assign18930_e18820;
        locals.var_q_d2_lnexpnum_dn4 = assign18930_e18820_d_n4;
        locals.var_q_d2_lnexpnum_dn6 = assign18930_e18820_d_n6;
        locals.var_q_d2_lnexpnum_dn7 = assign18930_e18820_d_n7;
        locals.var_q_d2_lnexpnum_dn8 = assign18930_e18820_d_n8;
        locals.var_q_d2_lnexpnum_dn9 = assign18930_e18820_d_n9;

        let (assign18940_e18831, assign18940_e18831_d_n4, assign18940_e18831_d_n6, assign18940_e18831_d_n7, assign18940_e18831_d_n8, assign18940_e18831_d_n9,) = {
    if (locals.var_guard638 == 0.0) {
        let assign18940_e18825: f64 = (locals.var_q_k1q1 + 0.6931471805599);
        let assign18940_e18827: f64 = (-locals.var_q_k1q1);
        let assign18940_e18828: f64 = (assign18940_e18827).ln();
        let assign18940_e18829: f64 = (assign18940_e18825 + assign18940_e18828);
        (assign18940_e18829, (locals.var_q_k1q1_dn4 + ((-locals.var_q_k1q1_dn4) / assign18940_e18827)), (locals.var_q_k1q1_dn6 + ((-locals.var_q_k1q1_dn6) / assign18940_e18827)), (locals.var_q_k1q1_dn7 + ((-locals.var_q_k1q1_dn7) / assign18940_e18827)), (locals.var_q_k1q1_dn8 + ((-locals.var_q_k1q1_dn8) / assign18940_e18827)), (locals.var_q_k1q1_dn9 + ((-locals.var_q_k1q1_dn9) / assign18940_e18827)),)
    } else {
        (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9,)
    }
};
        locals.var_q_lnexpnum = assign18940_e18831;
        locals.var_q_lnexpnum_dn4 = assign18940_e18831_d_n4;
        locals.var_q_lnexpnum_dn6 = assign18940_e18831_d_n6;
        locals.var_q_lnexpnum_dn7 = assign18940_e18831_d_n7;
        locals.var_q_lnexpnum_dn8 = assign18940_e18831_d_n8;
        locals.var_q_lnexpnum_dn9 = assign18940_e18831_d_n9;

        let (assign18950_e18838, assign18950_e18838_d_n4, assign18950_e18838_d_n6, assign18950_e18838_d_n7, assign18950_e18838_d_n8, assign18950_e18838_d_n9,) = {
    if (locals.var_guard638 == 0.0) {
        let assign18950_e18836: f64 = (1.0 / locals.var_q1d);
        (assign18950_e18836, (-(locals.var_q1d_dn4 / (locals.var_q1d * locals.var_q1d))), (-(locals.var_q1d_dn6 / (locals.var_q1d * locals.var_q1d))), (-(locals.var_q1d_dn7 / (locals.var_q1d * locals.var_q1d))), (-(locals.var_q1d_dn8 / (locals.var_q1d * locals.var_q1d))), (-(locals.var_q1d_dn9 / (locals.var_q1d * locals.var_q1d))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign18950_e18838;
        locals.var_q_temp1_dn4 = assign18950_e18838_d_n4;
        locals.var_q_temp1_dn6 = assign18950_e18838_d_n6;
        locals.var_q_temp1_dn7 = assign18950_e18838_d_n7;
        locals.var_q_temp1_dn8 = assign18950_e18838_d_n8;
        locals.var_q_temp1_dn9 = assign18950_e18838_d_n9;

        let (assign18960_e18845, assign18960_e18845_d_n4, assign18960_e18845_d_n6, assign18960_e18845_d_n7, assign18960_e18845_d_n8, assign18960_e18845_d_n9,) = {
    if (locals.var_guard638 == 0.0) {
        let assign18960_e18843: f64 = (locals.var_k1 + locals.var_q_temp1);
        (assign18960_e18843, (locals.var_k1_dn4 + locals.var_q_temp1_dn4), (locals.var_k1_dn6 + locals.var_q_temp1_dn6), (locals.var_k1_dn7 + locals.var_q_temp1_dn7), (locals.var_k1_dn8 + locals.var_q_temp1_dn8), (locals.var_k1_dn9 + locals.var_q_temp1_dn9),)
    } else {
        (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9,)
    }
};
        locals.var_q_d1_lnexpnum = assign18960_e18845;
        locals.var_q_d1_lnexpnum_dn4 = assign18960_e18845_d_n4;
        locals.var_q_d1_lnexpnum_dn6 = assign18960_e18845_d_n6;
        locals.var_q_d1_lnexpnum_dn7 = assign18960_e18845_d_n7;
        locals.var_q_d1_lnexpnum_dn8 = assign18960_e18845_d_n8;
        locals.var_q_d1_lnexpnum_dn9 = assign18960_e18845_d_n9;

        let (assign18970_e18853, assign18970_e18853_d_n4, assign18970_e18853_d_n6, assign18970_e18853_d_n7, assign18970_e18853_d_n8, assign18970_e18853_d_n9,) = {
    if (locals.var_guard638 == 0.0) {
        let assign18970_e18849: f64 = (-locals.var_q_temp1);
        let assign18970_e18851: f64 = (assign18970_e18849 * locals.var_q_temp1);
        (assign18970_e18851, (((-locals.var_q_temp1_dn4) * locals.var_q_temp1) + (assign18970_e18849 * locals.var_q_temp1_dn4)), (((-locals.var_q_temp1_dn6) * locals.var_q_temp1) + (assign18970_e18849 * locals.var_q_temp1_dn6)), (((-locals.var_q_temp1_dn7) * locals.var_q_temp1) + (assign18970_e18849 * locals.var_q_temp1_dn7)), (((-locals.var_q_temp1_dn8) * locals.var_q_temp1) + (assign18970_e18849 * locals.var_q_temp1_dn8)), (((-locals.var_q_temp1_dn9) * locals.var_q_temp1) + (assign18970_e18849 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9,)
    }
};
        locals.var_q_d2_lnexpnum = assign18970_e18853;
        locals.var_q_d2_lnexpnum_dn4 = assign18970_e18853_d_n4;
        locals.var_q_d2_lnexpnum_dn6 = assign18970_e18853_d_n6;
        locals.var_q_d2_lnexpnum_dn7 = assign18970_e18853_d_n7;
        locals.var_q_d2_lnexpnum_dn8 = assign18970_e18853_d_n8;
        locals.var_q_d2_lnexpnum_dn9 = assign18970_e18853_d_n9;

        let assign18980_e18856: f64 = (locals.var_xg2x - locals.var_xg1x);
        let assign18980_e18858: f64 = (assign18980_e18856 + locals.var_q1d);
        let assign18980_e18861: f64 = (2.0 * locals.var_q_lnexpnum);
        let assign18980_e18862: f64 = (assign18980_e18858 + assign18980_e18861);
        let assign18980_e18864: f64 = (assign18980_e18862 - locals.var_q_ln_term);
        locals.var_q_q2_int = assign18980_e18864;
        locals.var_q_q2_int_dn4 = ((((locals.var_xg2x_dn4 - locals.var_xg1x_dn4) + locals.var_q1d_dn4) + (2.0 * locals.var_q_lnexpnum_dn4)) - locals.var_q_ln_term_dn4);
        locals.var_q_q2_int_dn6 = ((((locals.var_xg2x_dn6 - locals.var_xg1x_dn6) + locals.var_q1d_dn6) + (2.0 * locals.var_q_lnexpnum_dn6)) - locals.var_q_ln_term_dn6);
        locals.var_q_q2_int_dn7 = ((((locals.var_xg2x_dn7 - locals.var_xg1x_dn7) + locals.var_q1d_dn7) + (2.0 * locals.var_q_lnexpnum_dn7)) - locals.var_q_ln_term_dn7);
        locals.var_q_q2_int_dn8 = ((((locals.var_xg2x_dn8 - locals.var_xg1x_dn8) + locals.var_q1d_dn8) + (2.0 * locals.var_q_lnexpnum_dn8)) - locals.var_q_ln_term_dn8);
        locals.var_q_q2_int_dn9 = ((((locals.var_xg2x_dn9 - locals.var_xg1x_dn9) + locals.var_q1d_dn9) + (2.0 * locals.var_q_lnexpnum_dn9)) - locals.var_q_ln_term_dn9);

        let assign18990_e18868: f64 = (2.0 * locals.var_q_d1_lnexpnum);
        let assign18990_e18869: f64 = (1.0 + assign18990_e18868);
        let assign18990_e18871: f64 = (assign18990_e18869 - locals.var_q_d1_ln);
        locals.var_q_d1_q2 = assign18990_e18871;
        locals.var_q_d1_q2_dn4 = ((2.0 * locals.var_q_d1_lnexpnum_dn4) - locals.var_q_d1_ln_dn4);
        locals.var_q_d1_q2_dn6 = ((2.0 * locals.var_q_d1_lnexpnum_dn6) - locals.var_q_d1_ln_dn6);
        locals.var_q_d1_q2_dn7 = ((2.0 * locals.var_q_d1_lnexpnum_dn7) - locals.var_q_d1_ln_dn7);
        locals.var_q_d1_q2_dn8 = ((2.0 * locals.var_q_d1_lnexpnum_dn8) - locals.var_q_d1_ln_dn8);
        locals.var_q_d1_q2_dn9 = ((2.0 * locals.var_q_d1_lnexpnum_dn9) - locals.var_q_d1_ln_dn9);

        let assign19000_e18874: f64 = (2.0 * locals.var_q_d2_lnexpnum);
        let assign19000_e18876: f64 = (assign19000_e18874 - locals.var_q_d2_ln);
        locals.var_q_d2_q2 = assign19000_e18876;
        locals.var_q_d2_q2_dn4 = ((2.0 * locals.var_q_d2_lnexpnum_dn4) - locals.var_q_d2_ln_dn4);
        locals.var_q_d2_q2_dn6 = ((2.0 * locals.var_q_d2_lnexpnum_dn6) - locals.var_q_d2_ln_dn6);
        locals.var_q_d2_q2_dn7 = ((2.0 * locals.var_q_d2_lnexpnum_dn7) - locals.var_q_d2_ln_dn7);
        locals.var_q_d2_q2_dn8 = ((2.0 * locals.var_q_d2_lnexpnum_dn8) - locals.var_q_d2_ln_dn8);
        locals.var_q_d2_q2_dn9 = ((2.0 * locals.var_q_d2_lnexpnum_dn9) - locals.var_q_d2_ln_dn9);

        let assign19010_e18880: f64 = (locals.var_k2 * locals.var_q_q2_int);
        let assign19010_e18881: f64 = (locals.var_q_k1q1 + assign19010_e18880);
        locals.var_q_qi_int = assign19010_e18881;
        locals.var_q_qi_int_dn4 = (locals.var_q_k1q1_dn4 + ((locals.var_k2_dn4 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn4)));
        locals.var_q_qi_int_dn6 = (locals.var_q_k1q1_dn6 + ((locals.var_k2_dn6 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn6)));
        locals.var_q_qi_int_dn7 = (locals.var_q_k1q1_dn7 + ((locals.var_k2_dn7 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn7)));
        locals.var_q_qi_int_dn8 = (locals.var_q_k1q1_dn8 + ((locals.var_k2_dn8 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn8)));
        locals.var_q_qi_int_dn9 = (locals.var_q_k1q1_dn9 + ((locals.var_k2_dn9 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn9)));

        let assign19020_e18885: f64 = (locals.var_k2 * locals.var_q_d1_q2);
        let assign19020_e18886: f64 = (locals.var_k1 + assign19020_e18885);
        locals.var_q_d1_qi = assign19020_e18886;
        locals.var_q_d1_qi_dn4 = (locals.var_k1_dn4 + ((locals.var_k2_dn4 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn4)));
        locals.var_q_d1_qi_dn6 = (locals.var_k1_dn6 + ((locals.var_k2_dn6 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn6)));
        locals.var_q_d1_qi_dn7 = (locals.var_k1_dn7 + ((locals.var_k2_dn7 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn7)));
        locals.var_q_d1_qi_dn8 = (locals.var_k1_dn8 + ((locals.var_k2_dn8 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn8)));
        locals.var_q_d1_qi_dn9 = (locals.var_k1_dn9 + ((locals.var_k2_dn9 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn9)));

        let assign19030_e18889: f64 = (locals.var_k2 * locals.var_q_d2_q2);
        locals.var_q_d2_qi = assign19030_e18889;
        locals.var_q_d2_qi_dn4 = ((locals.var_k2_dn4 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn4));
        locals.var_q_d2_qi_dn6 = ((locals.var_k2_dn6 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn6));
        locals.var_q_d2_qi_dn7 = ((locals.var_k2_dn7 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn7));
        locals.var_q_d2_qi_dn8 = ((locals.var_k2_dn8 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn8));
        locals.var_q_d2_qi_dn9 = ((locals.var_k2_dn9 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn9));

        let assign19040_e18892: f64 = (locals.var_q_qi_int * locals.var_q_expnum);
        let assign19040_e18894: f64 = (assign19040_e18892 - locals.var_q_aexp);
        locals.var_q_zero = assign19040_e18894;
        locals.var_q_zero_dn4 = (((locals.var_q_qi_int_dn4 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4);
        locals.var_q_zero_dn6 = (((locals.var_q_qi_int_dn6 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6);
        locals.var_q_zero_dn7 = (((locals.var_q_qi_int_dn7 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7);
        locals.var_q_zero_dn8 = (((locals.var_q_qi_int_dn8 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8);
        locals.var_q_zero_dn9 = (((locals.var_q_qi_int_dn9 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9);

        let assign19050_e18897: f64 = (locals.var_q_d1_qi * locals.var_q_expnum);
        let assign19050_e18900: f64 = (locals.var_q_qi_int * locals.var_q_d1_expnum);
        let assign19050_e18901: f64 = (assign19050_e18897 + assign19050_e18900);
        let assign19050_e18903: f64 = (assign19050_e18901 + locals.var_q_aexp);
        locals.var_q_d1_zero = assign19050_e18903;
        locals.var_q_d1_zero_dn4 = ((((locals.var_q_d1_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn4)) + ((locals.var_q_qi_int_dn4 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4);
        locals.var_q_d1_zero_dn6 = ((((locals.var_q_d1_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn6)) + ((locals.var_q_qi_int_dn6 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6);
        locals.var_q_d1_zero_dn7 = ((((locals.var_q_d1_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn7)) + ((locals.var_q_qi_int_dn7 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7);
        locals.var_q_d1_zero_dn8 = ((((locals.var_q_d1_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn8)) + ((locals.var_q_qi_int_dn8 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8);
        locals.var_q_d1_zero_dn9 = ((((locals.var_q_d1_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn9)) + ((locals.var_q_qi_int_dn9 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9);

        let assign19060_e18906: f64 = (locals.var_q_d2_qi * locals.var_q_expnum);
        let assign19060_e18909: f64 = (2.0 * locals.var_q_d1_qi);
        let assign19060_e18911: f64 = (assign19060_e18909 * locals.var_q_d1_expnum);
        let assign19060_e18912: f64 = (assign19060_e18906 + assign19060_e18911);
        let assign19060_e18915: f64 = (locals.var_q_qi_int * locals.var_q_d2_expnum);
        let assign19060_e18916: f64 = (assign19060_e18912 + assign19060_e18915);
        let assign19060_e18918: f64 = (assign19060_e18916 - locals.var_q_aexp);
        locals.var_q_d2_zero = assign19060_e18918;
        locals.var_q_d2_zero_dn4 = (((((locals.var_q_d2_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_d1_qi_dn4) * locals.var_q_d1_expnum) + (assign19060_e18909 * locals.var_q_d1_expnum_dn4))) + ((locals.var_q_qi_int_dn4 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn4))) - locals.var_q_aexp_dn4);
        locals.var_q_d2_zero_dn6 = (((((locals.var_q_d2_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_d1_qi_dn6) * locals.var_q_d1_expnum) + (assign19060_e18909 * locals.var_q_d1_expnum_dn6))) + ((locals.var_q_qi_int_dn6 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn6))) - locals.var_q_aexp_dn6);
        locals.var_q_d2_zero_dn7 = (((((locals.var_q_d2_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_d1_qi_dn7) * locals.var_q_d1_expnum) + (assign19060_e18909 * locals.var_q_d1_expnum_dn7))) + ((locals.var_q_qi_int_dn7 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn7))) - locals.var_q_aexp_dn7);
        locals.var_q_d2_zero_dn8 = (((((locals.var_q_d2_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_d1_qi_dn8) * locals.var_q_d1_expnum) + (assign19060_e18909 * locals.var_q_d1_expnum_dn8))) + ((locals.var_q_qi_int_dn8 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn8))) - locals.var_q_aexp_dn8);
        locals.var_q_d2_zero_dn9 = (((((locals.var_q_d2_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_d1_qi_dn9) * locals.var_q_d1_expnum) + (assign19060_e18909 * locals.var_q_d1_expnum_dn9))) + ((locals.var_q_qi_int_dn9 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn9))) - locals.var_q_aexp_dn9);

        let assign19070_e18921: f64 = (locals.var_q_d1_zero * locals.var_q_d1_zero);
        let assign19070_e18924: f64 = (0.5 * locals.var_q_zero);
        let assign19070_e18926: f64 = (assign19070_e18924 * locals.var_q_d2_zero);
        let assign19070_e18927: f64 = (assign19070_e18921 - assign19070_e18926);
        locals.var_q_temp = assign19070_e18927;
        locals.var_q_temp_dn4 = (((locals.var_q_d1_zero_dn4 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn4)) - (((0.5 * locals.var_q_zero_dn4) * locals.var_q_d2_zero) + (assign19070_e18924 * locals.var_q_d2_zero_dn4)));
        locals.var_q_temp_dn6 = (((locals.var_q_d1_zero_dn6 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn6)) - (((0.5 * locals.var_q_zero_dn6) * locals.var_q_d2_zero) + (assign19070_e18924 * locals.var_q_d2_zero_dn6)));
        locals.var_q_temp_dn7 = (((locals.var_q_d1_zero_dn7 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn7)) - (((0.5 * locals.var_q_zero_dn7) * locals.var_q_d2_zero) + (assign19070_e18924 * locals.var_q_d2_zero_dn7)));
        locals.var_q_temp_dn8 = (((locals.var_q_d1_zero_dn8 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn8)) - (((0.5 * locals.var_q_zero_dn8) * locals.var_q_d2_zero) + (assign19070_e18924 * locals.var_q_d2_zero_dn8)));
        locals.var_q_temp_dn9 = (((locals.var_q_d1_zero_dn9 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn9)) - (((0.5 * locals.var_q_zero_dn9) * locals.var_q_d2_zero) + (assign19070_e18924 * locals.var_q_d2_zero_dn9)));

        let assign19080_e18929: f64 = (-locals.var_q_zero);
        let assign19080_e18931: f64 = (assign19080_e18929 * locals.var_q_d1_zero);
        let assign19080_e18933: f64 = (assign19080_e18931 * locals.var_q_temp);
        let assign19080_e18936: f64 = (locals.var_q_temp * locals.var_q_temp);
        let assign19080_e18938: f64 = (assign19080_e18936 + 1e-200);
        let assign19080_e18939: f64 = (assign19080_e18933 / assign19080_e18938);
        locals.var_q_eps2 = assign19080_e18939;
        locals.var_q_eps2_dn4 = ((((((((-locals.var_q_zero_dn4) * locals.var_q_d1_zero) + (assign19080_e18929 * locals.var_q_d1_zero_dn4)) * locals.var_q_temp) + (assign19080_e18931 * locals.var_q_temp_dn4)) * assign19080_e18938) - (assign19080_e18933 * ((locals.var_q_temp_dn4 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn4)))) / (assign19080_e18938 * assign19080_e18938));
        locals.var_q_eps2_dn6 = ((((((((-locals.var_q_zero_dn6) * locals.var_q_d1_zero) + (assign19080_e18929 * locals.var_q_d1_zero_dn6)) * locals.var_q_temp) + (assign19080_e18931 * locals.var_q_temp_dn6)) * assign19080_e18938) - (assign19080_e18933 * ((locals.var_q_temp_dn6 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn6)))) / (assign19080_e18938 * assign19080_e18938));
        locals.var_q_eps2_dn7 = ((((((((-locals.var_q_zero_dn7) * locals.var_q_d1_zero) + (assign19080_e18929 * locals.var_q_d1_zero_dn7)) * locals.var_q_temp) + (assign19080_e18931 * locals.var_q_temp_dn7)) * assign19080_e18938) - (assign19080_e18933 * ((locals.var_q_temp_dn7 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn7)))) / (assign19080_e18938 * assign19080_e18938));
        locals.var_q_eps2_dn8 = ((((((((-locals.var_q_zero_dn8) * locals.var_q_d1_zero) + (assign19080_e18929 * locals.var_q_d1_zero_dn8)) * locals.var_q_temp) + (assign19080_e18931 * locals.var_q_temp_dn8)) * assign19080_e18938) - (assign19080_e18933 * ((locals.var_q_temp_dn8 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn8)))) / (assign19080_e18938 * assign19080_e18938));
        locals.var_q_eps2_dn9 = ((((((((-locals.var_q_zero_dn9) * locals.var_q_d1_zero) + (assign19080_e18929 * locals.var_q_d1_zero_dn9)) * locals.var_q_temp) + (assign19080_e18931 * locals.var_q_temp_dn9)) * assign19080_e18938) - (assign19080_e18933 * ((locals.var_q_temp_dn9 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn9)))) / (assign19080_e18938 * assign19080_e18938));

        let assign19090_e18942: f64 = (locals.var_q1d + locals.var_q_eps2);
        locals.var_q1d = assign19090_e18942;
        locals.var_q1d_dn4 = (locals.var_q1d_dn4 + locals.var_q_eps2_dn4);
        locals.var_q1d_dn6 = (locals.var_q1d_dn6 + locals.var_q_eps2_dn6);
        locals.var_q1d_dn7 = (locals.var_q1d_dn7 + locals.var_q_eps2_dn7);
        locals.var_q1d_dn8 = (locals.var_q1d_dn8 + locals.var_q_eps2_dn8);
        locals.var_q1d_dn9 = (locals.var_q1d_dn9 + locals.var_q_eps2_dn9);

    }
}
