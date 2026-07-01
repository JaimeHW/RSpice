#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_32(
        locals: &mut StampLocals,
    ) {
        let (assign13310_e12667, assign13310_e12667_d_n4, assign13310_e12667_d_n6, assign13310_e12667_d_n7, assign13310_e12667_d_n8, assign13310_e12667_d_n9,) = {
    if (locals.var_guard571 != 0.0) {
        let assign13310_e12661: f64 = (locals.var_q_d2_expnum * locals.var_q_temp1);
        let assign13310_e12664: f64 = (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum);
        let assign13310_e12665: f64 = (assign13310_e12661 - assign13310_e12664);
        (assign13310_e12665, (((locals.var_q_d2_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn4)) - ((locals.var_q_d1_lnexpnum_dn4 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn4))), (((locals.var_q_d2_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn6)) - ((locals.var_q_d1_lnexpnum_dn6 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn6))), (((locals.var_q_d2_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn7)) - ((locals.var_q_d1_lnexpnum_dn7 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn7))), (((locals.var_q_d2_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn8)) - ((locals.var_q_d1_lnexpnum_dn8 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn8))), (((locals.var_q_d2_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn9)) - ((locals.var_q_d1_lnexpnum_dn9 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn9))),)
    } else {
        (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9,)
    }
};
        locals.var_q_d2_lnexpnum = assign13310_e12667;
        locals.var_q_d2_lnexpnum_dn4 = assign13310_e12667_d_n4;
        locals.var_q_d2_lnexpnum_dn6 = assign13310_e12667_d_n6;
        locals.var_q_d2_lnexpnum_dn7 = assign13310_e12667_d_n7;
        locals.var_q_d2_lnexpnum_dn8 = assign13310_e12667_d_n8;
        locals.var_q_d2_lnexpnum_dn9 = assign13310_e12667_d_n9;

        let (assign13320_e12678, assign13320_e12678_d_n4, assign13320_e12678_d_n6, assign13320_e12678_d_n7, assign13320_e12678_d_n8, assign13320_e12678_d_n9,) = {
    if (locals.var_guard571 == 0.0) {
        let assign13320_e12672: f64 = (locals.var_q_k1q1 + 0.6931471805599);
        let assign13320_e12674: f64 = (-locals.var_q_k1q1);
        let assign13320_e12675: f64 = (assign13320_e12674).ln();
        let assign13320_e12676: f64 = (assign13320_e12672 + assign13320_e12675);
        (assign13320_e12676, (locals.var_q_k1q1_dn4 + ((-locals.var_q_k1q1_dn4) / assign13320_e12674)), (locals.var_q_k1q1_dn6 + ((-locals.var_q_k1q1_dn6) / assign13320_e12674)), (locals.var_q_k1q1_dn7 + ((-locals.var_q_k1q1_dn7) / assign13320_e12674)), (locals.var_q_k1q1_dn8 + ((-locals.var_q_k1q1_dn8) / assign13320_e12674)), (locals.var_q_k1q1_dn9 + ((-locals.var_q_k1q1_dn9) / assign13320_e12674)),)
    } else {
        (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9,)
    }
};
        locals.var_q_lnexpnum = assign13320_e12678;
        locals.var_q_lnexpnum_dn4 = assign13320_e12678_d_n4;
        locals.var_q_lnexpnum_dn6 = assign13320_e12678_d_n6;
        locals.var_q_lnexpnum_dn7 = assign13320_e12678_d_n7;
        locals.var_q_lnexpnum_dn8 = assign13320_e12678_d_n8;
        locals.var_q_lnexpnum_dn9 = assign13320_e12678_d_n9;

        let (assign13330_e12685, assign13330_e12685_d_n4, assign13330_e12685_d_n6, assign13330_e12685_d_n7, assign13330_e12685_d_n8, assign13330_e12685_d_n9,) = {
    if (locals.var_guard571 == 0.0) {
        let assign13330_e12683: f64 = (1.0 / locals.var_q1s);
        (assign13330_e12683, (-(locals.var_q1s_dn4 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn6 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn7 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn8 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn9 / (locals.var_q1s * locals.var_q1s))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign13330_e12685;
        locals.var_q_temp1_dn4 = assign13330_e12685_d_n4;
        locals.var_q_temp1_dn6 = assign13330_e12685_d_n6;
        locals.var_q_temp1_dn7 = assign13330_e12685_d_n7;
        locals.var_q_temp1_dn8 = assign13330_e12685_d_n8;
        locals.var_q_temp1_dn9 = assign13330_e12685_d_n9;

        let (assign13340_e12692, assign13340_e12692_d_n4, assign13340_e12692_d_n6, assign13340_e12692_d_n7, assign13340_e12692_d_n8, assign13340_e12692_d_n9,) = {
    if (locals.var_guard571 == 0.0) {
        let assign13340_e12690: f64 = (locals.var_k1 + locals.var_q_temp1);
        (assign13340_e12690, (locals.var_k1_dn4 + locals.var_q_temp1_dn4), (locals.var_k1_dn6 + locals.var_q_temp1_dn6), (locals.var_k1_dn7 + locals.var_q_temp1_dn7), (locals.var_k1_dn8 + locals.var_q_temp1_dn8), (locals.var_k1_dn9 + locals.var_q_temp1_dn9),)
    } else {
        (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9,)
    }
};
        locals.var_q_d1_lnexpnum = assign13340_e12692;
        locals.var_q_d1_lnexpnum_dn4 = assign13340_e12692_d_n4;
        locals.var_q_d1_lnexpnum_dn6 = assign13340_e12692_d_n6;
        locals.var_q_d1_lnexpnum_dn7 = assign13340_e12692_d_n7;
        locals.var_q_d1_lnexpnum_dn8 = assign13340_e12692_d_n8;
        locals.var_q_d1_lnexpnum_dn9 = assign13340_e12692_d_n9;

        let (assign13350_e12700, assign13350_e12700_d_n4, assign13350_e12700_d_n6, assign13350_e12700_d_n7, assign13350_e12700_d_n8, assign13350_e12700_d_n9,) = {
    if (locals.var_guard571 == 0.0) {
        let assign13350_e12696: f64 = (-locals.var_q_temp1);
        let assign13350_e12698: f64 = (assign13350_e12696 * locals.var_q_temp1);
        (assign13350_e12698, (((-locals.var_q_temp1_dn4) * locals.var_q_temp1) + (assign13350_e12696 * locals.var_q_temp1_dn4)), (((-locals.var_q_temp1_dn6) * locals.var_q_temp1) + (assign13350_e12696 * locals.var_q_temp1_dn6)), (((-locals.var_q_temp1_dn7) * locals.var_q_temp1) + (assign13350_e12696 * locals.var_q_temp1_dn7)), (((-locals.var_q_temp1_dn8) * locals.var_q_temp1) + (assign13350_e12696 * locals.var_q_temp1_dn8)), (((-locals.var_q_temp1_dn9) * locals.var_q_temp1) + (assign13350_e12696 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9,)
    }
};
        locals.var_q_d2_lnexpnum = assign13350_e12700;
        locals.var_q_d2_lnexpnum_dn4 = assign13350_e12700_d_n4;
        locals.var_q_d2_lnexpnum_dn6 = assign13350_e12700_d_n6;
        locals.var_q_d2_lnexpnum_dn7 = assign13350_e12700_d_n7;
        locals.var_q_d2_lnexpnum_dn8 = assign13350_e12700_d_n8;
        locals.var_q_d2_lnexpnum_dn9 = assign13350_e12700_d_n9;

        let assign13360_e12703: f64 = (locals.var_xg2x - locals.var_xg1x);
        let assign13360_e12705: f64 = (assign13360_e12703 + locals.var_q1s);
        let assign13360_e12708: f64 = (2.0 * locals.var_q_lnexpnum);
        let assign13360_e12709: f64 = (assign13360_e12705 + assign13360_e12708);
        let assign13360_e12711: f64 = (assign13360_e12709 - locals.var_q_ln_term);
        locals.var_q_q2_int = assign13360_e12711;
        locals.var_q_q2_int_dn4 = ((((locals.var_xg2x_dn4 - locals.var_xg1x_dn4) + locals.var_q1s_dn4) + (2.0 * locals.var_q_lnexpnum_dn4)) - locals.var_q_ln_term_dn4);
        locals.var_q_q2_int_dn6 = ((((locals.var_xg2x_dn6 - locals.var_xg1x_dn6) + locals.var_q1s_dn6) + (2.0 * locals.var_q_lnexpnum_dn6)) - locals.var_q_ln_term_dn6);
        locals.var_q_q2_int_dn7 = ((((locals.var_xg2x_dn7 - locals.var_xg1x_dn7) + locals.var_q1s_dn7) + (2.0 * locals.var_q_lnexpnum_dn7)) - locals.var_q_ln_term_dn7);
        locals.var_q_q2_int_dn8 = ((((locals.var_xg2x_dn8 - locals.var_xg1x_dn8) + locals.var_q1s_dn8) + (2.0 * locals.var_q_lnexpnum_dn8)) - locals.var_q_ln_term_dn8);
        locals.var_q_q2_int_dn9 = ((((locals.var_xg2x_dn9 - locals.var_xg1x_dn9) + locals.var_q1s_dn9) + (2.0 * locals.var_q_lnexpnum_dn9)) - locals.var_q_ln_term_dn9);

        let assign13370_e12715: f64 = (2.0 * locals.var_q_d1_lnexpnum);
        let assign13370_e12716: f64 = (1.0 + assign13370_e12715);
        let assign13370_e12718: f64 = (assign13370_e12716 - locals.var_q_d1_ln);
        locals.var_q_d1_q2 = assign13370_e12718;
        locals.var_q_d1_q2_dn4 = ((2.0 * locals.var_q_d1_lnexpnum_dn4) - locals.var_q_d1_ln_dn4);
        locals.var_q_d1_q2_dn6 = ((2.0 * locals.var_q_d1_lnexpnum_dn6) - locals.var_q_d1_ln_dn6);
        locals.var_q_d1_q2_dn7 = ((2.0 * locals.var_q_d1_lnexpnum_dn7) - locals.var_q_d1_ln_dn7);
        locals.var_q_d1_q2_dn8 = ((2.0 * locals.var_q_d1_lnexpnum_dn8) - locals.var_q_d1_ln_dn8);
        locals.var_q_d1_q2_dn9 = ((2.0 * locals.var_q_d1_lnexpnum_dn9) - locals.var_q_d1_ln_dn9);

        let assign13380_e12721: f64 = (2.0 * locals.var_q_d2_lnexpnum);
        let assign13380_e12723: f64 = (assign13380_e12721 - locals.var_q_d2_ln);
        locals.var_q_d2_q2 = assign13380_e12723;
        locals.var_q_d2_q2_dn4 = ((2.0 * locals.var_q_d2_lnexpnum_dn4) - locals.var_q_d2_ln_dn4);
        locals.var_q_d2_q2_dn6 = ((2.0 * locals.var_q_d2_lnexpnum_dn6) - locals.var_q_d2_ln_dn6);
        locals.var_q_d2_q2_dn7 = ((2.0 * locals.var_q_d2_lnexpnum_dn7) - locals.var_q_d2_ln_dn7);
        locals.var_q_d2_q2_dn8 = ((2.0 * locals.var_q_d2_lnexpnum_dn8) - locals.var_q_d2_ln_dn8);
        locals.var_q_d2_q2_dn9 = ((2.0 * locals.var_q_d2_lnexpnum_dn9) - locals.var_q_d2_ln_dn9);

        let assign13390_e12727: f64 = (locals.var_k2 * locals.var_q_q2_int);
        let assign13390_e12728: f64 = (locals.var_q_k1q1 + assign13390_e12727);
        locals.var_q_qi_int = assign13390_e12728;
        locals.var_q_qi_int_dn4 = (locals.var_q_k1q1_dn4 + ((locals.var_k2_dn4 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn4)));
        locals.var_q_qi_int_dn6 = (locals.var_q_k1q1_dn6 + ((locals.var_k2_dn6 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn6)));
        locals.var_q_qi_int_dn7 = (locals.var_q_k1q1_dn7 + ((locals.var_k2_dn7 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn7)));
        locals.var_q_qi_int_dn8 = (locals.var_q_k1q1_dn8 + ((locals.var_k2_dn8 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn8)));
        locals.var_q_qi_int_dn9 = (locals.var_q_k1q1_dn9 + ((locals.var_k2_dn9 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn9)));

        let assign13400_e12732: f64 = (locals.var_k2 * locals.var_q_d1_q2);
        let assign13400_e12733: f64 = (locals.var_k1 + assign13400_e12732);
        locals.var_q_d1_qi = assign13400_e12733;
        locals.var_q_d1_qi_dn4 = (locals.var_k1_dn4 + ((locals.var_k2_dn4 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn4)));
        locals.var_q_d1_qi_dn6 = (locals.var_k1_dn6 + ((locals.var_k2_dn6 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn6)));
        locals.var_q_d1_qi_dn7 = (locals.var_k1_dn7 + ((locals.var_k2_dn7 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn7)));
        locals.var_q_d1_qi_dn8 = (locals.var_k1_dn8 + ((locals.var_k2_dn8 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn8)));
        locals.var_q_d1_qi_dn9 = (locals.var_k1_dn9 + ((locals.var_k2_dn9 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn9)));

        let assign13410_e12736: f64 = (locals.var_k2 * locals.var_q_d2_q2);
        locals.var_q_d2_qi = assign13410_e12736;
        locals.var_q_d2_qi_dn4 = ((locals.var_k2_dn4 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn4));
        locals.var_q_d2_qi_dn6 = ((locals.var_k2_dn6 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn6));
        locals.var_q_d2_qi_dn7 = ((locals.var_k2_dn7 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn7));
        locals.var_q_d2_qi_dn8 = ((locals.var_k2_dn8 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn8));
        locals.var_q_d2_qi_dn9 = ((locals.var_k2_dn9 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn9));

        let assign13420_e12739: f64 = (locals.var_q_qi_int * locals.var_q_expnum);
        let assign13420_e12741: f64 = (assign13420_e12739 - locals.var_q_aexp);
        locals.var_q_zero = assign13420_e12741;
        locals.var_q_zero_dn4 = (((locals.var_q_qi_int_dn4 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4);
        locals.var_q_zero_dn6 = (((locals.var_q_qi_int_dn6 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6);
        locals.var_q_zero_dn7 = (((locals.var_q_qi_int_dn7 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7);
        locals.var_q_zero_dn8 = (((locals.var_q_qi_int_dn8 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8);
        locals.var_q_zero_dn9 = (((locals.var_q_qi_int_dn9 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9);

        let assign13430_e12744: f64 = (locals.var_q_d1_qi * locals.var_q_expnum);
        let assign13430_e12747: f64 = (locals.var_q_qi_int * locals.var_q_d1_expnum);
        let assign13430_e12748: f64 = (assign13430_e12744 + assign13430_e12747);
        let assign13430_e12750: f64 = (assign13430_e12748 + locals.var_q_aexp);
        locals.var_q_d1_zero = assign13430_e12750;
        locals.var_q_d1_zero_dn4 = ((((locals.var_q_d1_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn4)) + ((locals.var_q_qi_int_dn4 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4);
        locals.var_q_d1_zero_dn6 = ((((locals.var_q_d1_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn6)) + ((locals.var_q_qi_int_dn6 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6);
        locals.var_q_d1_zero_dn7 = ((((locals.var_q_d1_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn7)) + ((locals.var_q_qi_int_dn7 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7);
        locals.var_q_d1_zero_dn8 = ((((locals.var_q_d1_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn8)) + ((locals.var_q_qi_int_dn8 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8);
        locals.var_q_d1_zero_dn9 = ((((locals.var_q_d1_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn9)) + ((locals.var_q_qi_int_dn9 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9);

        let assign13440_e12753: f64 = (locals.var_q_d2_qi * locals.var_q_expnum);
        let assign13440_e12756: f64 = (2.0 * locals.var_q_d1_qi);
        let assign13440_e12758: f64 = (assign13440_e12756 * locals.var_q_d1_expnum);
        let assign13440_e12759: f64 = (assign13440_e12753 + assign13440_e12758);
        let assign13440_e12762: f64 = (locals.var_q_qi_int * locals.var_q_d2_expnum);
        let assign13440_e12763: f64 = (assign13440_e12759 + assign13440_e12762);
        let assign13440_e12765: f64 = (assign13440_e12763 - locals.var_q_aexp);
        locals.var_q_d2_zero = assign13440_e12765;
        locals.var_q_d2_zero_dn4 = (((((locals.var_q_d2_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_d1_qi_dn4) * locals.var_q_d1_expnum) + (assign13440_e12756 * locals.var_q_d1_expnum_dn4))) + ((locals.var_q_qi_int_dn4 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn4))) - locals.var_q_aexp_dn4);
        locals.var_q_d2_zero_dn6 = (((((locals.var_q_d2_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_d1_qi_dn6) * locals.var_q_d1_expnum) + (assign13440_e12756 * locals.var_q_d1_expnum_dn6))) + ((locals.var_q_qi_int_dn6 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn6))) - locals.var_q_aexp_dn6);
        locals.var_q_d2_zero_dn7 = (((((locals.var_q_d2_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_d1_qi_dn7) * locals.var_q_d1_expnum) + (assign13440_e12756 * locals.var_q_d1_expnum_dn7))) + ((locals.var_q_qi_int_dn7 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn7))) - locals.var_q_aexp_dn7);
        locals.var_q_d2_zero_dn8 = (((((locals.var_q_d2_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_d1_qi_dn8) * locals.var_q_d1_expnum) + (assign13440_e12756 * locals.var_q_d1_expnum_dn8))) + ((locals.var_q_qi_int_dn8 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn8))) - locals.var_q_aexp_dn8);
        locals.var_q_d2_zero_dn9 = (((((locals.var_q_d2_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_d1_qi_dn9) * locals.var_q_d1_expnum) + (assign13440_e12756 * locals.var_q_d1_expnum_dn9))) + ((locals.var_q_qi_int_dn9 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn9))) - locals.var_q_aexp_dn9);

        let assign13450_e12768: f64 = (locals.var_q_d1_zero * locals.var_q_d1_zero);
        let assign13450_e12771: f64 = (0.5 * locals.var_q_zero);
        let assign13450_e12773: f64 = (assign13450_e12771 * locals.var_q_d2_zero);
        let assign13450_e12774: f64 = (assign13450_e12768 - assign13450_e12773);
        locals.var_q_temp = assign13450_e12774;
        locals.var_q_temp_dn4 = (((locals.var_q_d1_zero_dn4 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn4)) - (((0.5 * locals.var_q_zero_dn4) * locals.var_q_d2_zero) + (assign13450_e12771 * locals.var_q_d2_zero_dn4)));
        locals.var_q_temp_dn6 = (((locals.var_q_d1_zero_dn6 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn6)) - (((0.5 * locals.var_q_zero_dn6) * locals.var_q_d2_zero) + (assign13450_e12771 * locals.var_q_d2_zero_dn6)));
        locals.var_q_temp_dn7 = (((locals.var_q_d1_zero_dn7 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn7)) - (((0.5 * locals.var_q_zero_dn7) * locals.var_q_d2_zero) + (assign13450_e12771 * locals.var_q_d2_zero_dn7)));
        locals.var_q_temp_dn8 = (((locals.var_q_d1_zero_dn8 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn8)) - (((0.5 * locals.var_q_zero_dn8) * locals.var_q_d2_zero) + (assign13450_e12771 * locals.var_q_d2_zero_dn8)));
        locals.var_q_temp_dn9 = (((locals.var_q_d1_zero_dn9 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn9)) - (((0.5 * locals.var_q_zero_dn9) * locals.var_q_d2_zero) + (assign13450_e12771 * locals.var_q_d2_zero_dn9)));

        let assign13460_e12776: f64 = (-locals.var_q_zero);
        let assign13460_e12778: f64 = (assign13460_e12776 * locals.var_q_d1_zero);
        let assign13460_e12780: f64 = (assign13460_e12778 * locals.var_q_temp);
        let assign13460_e12783: f64 = (locals.var_q_temp * locals.var_q_temp);
        let assign13460_e12785: f64 = (assign13460_e12783 + 1e-200);
        let assign13460_e12786: f64 = (assign13460_e12780 / assign13460_e12785);
        locals.var_q_eps2 = assign13460_e12786;
        locals.var_q_eps2_dn4 = ((((((((-locals.var_q_zero_dn4) * locals.var_q_d1_zero) + (assign13460_e12776 * locals.var_q_d1_zero_dn4)) * locals.var_q_temp) + (assign13460_e12778 * locals.var_q_temp_dn4)) * assign13460_e12785) - (assign13460_e12780 * ((locals.var_q_temp_dn4 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn4)))) / (assign13460_e12785 * assign13460_e12785));
        locals.var_q_eps2_dn6 = ((((((((-locals.var_q_zero_dn6) * locals.var_q_d1_zero) + (assign13460_e12776 * locals.var_q_d1_zero_dn6)) * locals.var_q_temp) + (assign13460_e12778 * locals.var_q_temp_dn6)) * assign13460_e12785) - (assign13460_e12780 * ((locals.var_q_temp_dn6 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn6)))) / (assign13460_e12785 * assign13460_e12785));
        locals.var_q_eps2_dn7 = ((((((((-locals.var_q_zero_dn7) * locals.var_q_d1_zero) + (assign13460_e12776 * locals.var_q_d1_zero_dn7)) * locals.var_q_temp) + (assign13460_e12778 * locals.var_q_temp_dn7)) * assign13460_e12785) - (assign13460_e12780 * ((locals.var_q_temp_dn7 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn7)))) / (assign13460_e12785 * assign13460_e12785));
        locals.var_q_eps2_dn8 = ((((((((-locals.var_q_zero_dn8) * locals.var_q_d1_zero) + (assign13460_e12776 * locals.var_q_d1_zero_dn8)) * locals.var_q_temp) + (assign13460_e12778 * locals.var_q_temp_dn8)) * assign13460_e12785) - (assign13460_e12780 * ((locals.var_q_temp_dn8 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn8)))) / (assign13460_e12785 * assign13460_e12785));
        locals.var_q_eps2_dn9 = ((((((((-locals.var_q_zero_dn9) * locals.var_q_d1_zero) + (assign13460_e12776 * locals.var_q_d1_zero_dn9)) * locals.var_q_temp) + (assign13460_e12778 * locals.var_q_temp_dn9)) * assign13460_e12785) - (assign13460_e12780 * ((locals.var_q_temp_dn9 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn9)))) / (assign13460_e12785 * assign13460_e12785));

        let assign13470_e12789: f64 = (locals.var_q1s + locals.var_q_eps2);
        locals.var_q1s = assign13470_e12789;
        locals.var_q1s_dn4 = (locals.var_q1s_dn4 + locals.var_q_eps2_dn4);
        locals.var_q1s_dn6 = (locals.var_q1s_dn6 + locals.var_q_eps2_dn6);
        locals.var_q1s_dn7 = (locals.var_q1s_dn7 + locals.var_q_eps2_dn7);
        locals.var_q1s_dn8 = (locals.var_q1s_dn8 + locals.var_q_eps2_dn8);
        locals.var_q1s_dn9 = (locals.var_q1s_dn9 + locals.var_q_eps2_dn9);

        let assign13480_e12792: f64 = (locals.var_k1 * locals.var_q1s);
        locals.var_q_k1q1 = assign13480_e12792;
        locals.var_q_k1q1_dn4 = ((locals.var_k1_dn4 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn4));
        locals.var_q_k1q1_dn6 = ((locals.var_k1_dn6 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn6));
        locals.var_q_k1q1_dn7 = ((locals.var_k1_dn7 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn7));
        locals.var_q_k1q1_dn8 = ((locals.var_k1_dn8 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn8));
        locals.var_q_k1q1_dn9 = ((locals.var_k1_dn9 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn9));

        let assign13490_e12795: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign13490_e12797: f64 = assign13490_e12795;
        let assign13490_e12799: f64 = if assign13490_e12797 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard572 = assign13490_e12799;

        let (assign13500_e12808, assign13500_e12808_d_n4, assign13500_e12808_d_n6, assign13500_e12808_d_n7, assign13500_e12808_d_n8, assign13500_e12808_d_n9,) = {
    if (locals.var_guard572 != 0.0) {
        let assign13500_e12803: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign13500_e12805: f64 = assign13500_e12803;
        let assign13500_e12806: f64 = (assign13500_e12805).exp();
        (assign13500_e12806, (assign13500_e12806 * (locals.var_xg1x_dn4 - locals.var_q1s_dn4)), (assign13500_e12806 * (locals.var_xg1x_dn6 - locals.var_q1s_dn6)), (assign13500_e12806 * (locals.var_xg1x_dn7 - locals.var_q1s_dn7)), (assign13500_e12806 * (locals.var_xg1x_dn8 - locals.var_q1s_dn8)), (assign13500_e12806 * (locals.var_xg1x_dn9 - locals.var_q1s_dn9)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign13500_e12808;
        locals.var_q_temp1_dn4 = assign13500_e12808_d_n4;
        locals.var_q_temp1_dn6 = assign13500_e12808_d_n6;
        locals.var_q_temp1_dn7 = assign13500_e12808_d_n7;
        locals.var_q_temp1_dn8 = assign13500_e12808_d_n8;
        locals.var_q_temp1_dn9 = assign13500_e12808_d_n9;

        let (assign13510_e12847, assign13510_e12847_d_n4, assign13510_e12847_d_n6, assign13510_e12847_d_n7, assign13510_e12847_d_n8, assign13510_e12847_d_n9,) = {
    if (locals.var_guard572 == 0.0) {
        let assign13510_e12815: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign13510_e12817: f64 = assign13510_e12815;
        let assign13510_e12819: f64 = (assign13510_e12817 - 80.0);
        let assign13510_e12824: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign13510_e12826: f64 = assign13510_e12824;
        let assign13510_e12828: f64 = (assign13510_e12826 - 80.0);
        let assign13510_e12829: f64 = (0.5 * assign13510_e12828);
        let assign13510_e12833: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign13510_e12835: f64 = assign13510_e12833;
        let assign13510_e12837: f64 = (assign13510_e12835 - 80.0);
        let assign13510_e12839: f64 = (assign13510_e12837 * 0.3333333333333);
        let assign13510_e12840: f64 = (1.0 + assign13510_e12839);
        let assign13510_e12841: f64 = (assign13510_e12829 * assign13510_e12840);
        let assign13510_e12842: f64 = (1.0 + assign13510_e12841);
        let assign13510_e12843: f64 = (assign13510_e12819 * assign13510_e12842);
        let assign13510_e12844: f64 = (1.0 + assign13510_e12843);
        let assign13510_e12845: f64 = (5.54062e34 * assign13510_e12844);
        (assign13510_e12845, (5.54062e34 * (((locals.var_xg1x_dn4 - locals.var_q1s_dn4) * assign13510_e12842) + (assign13510_e12819 * (((0.5 * (locals.var_xg1x_dn4 - locals.var_q1s_dn4)) * assign13510_e12840) + (assign13510_e12829 * ((locals.var_xg1x_dn4 - locals.var_q1s_dn4) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn6 - locals.var_q1s_dn6) * assign13510_e12842) + (assign13510_e12819 * (((0.5 * (locals.var_xg1x_dn6 - locals.var_q1s_dn6)) * assign13510_e12840) + (assign13510_e12829 * ((locals.var_xg1x_dn6 - locals.var_q1s_dn6) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn7 - locals.var_q1s_dn7) * assign13510_e12842) + (assign13510_e12819 * (((0.5 * (locals.var_xg1x_dn7 - locals.var_q1s_dn7)) * assign13510_e12840) + (assign13510_e12829 * ((locals.var_xg1x_dn7 - locals.var_q1s_dn7) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn8 - locals.var_q1s_dn8) * assign13510_e12842) + (assign13510_e12819 * (((0.5 * (locals.var_xg1x_dn8 - locals.var_q1s_dn8)) * assign13510_e12840) + (assign13510_e12829 * ((locals.var_xg1x_dn8 - locals.var_q1s_dn8) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn9 - locals.var_q1s_dn9) * assign13510_e12842) + (assign13510_e12819 * (((0.5 * (locals.var_xg1x_dn9 - locals.var_q1s_dn9)) * assign13510_e12840) + (assign13510_e12829 * ((locals.var_xg1x_dn9 - locals.var_q1s_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign13510_e12847;
        locals.var_q_temp1_dn4 = assign13510_e12847_d_n4;
        locals.var_q_temp1_dn6 = assign13510_e12847_d_n6;
        locals.var_q_temp1_dn7 = assign13510_e12847_d_n7;
        locals.var_q_temp1_dn8 = assign13510_e12847_d_n8;
        locals.var_q_temp1_dn9 = assign13510_e12847_d_n9;

        let assign13520_e12850: f64 = (locals.var_a0 * locals.var_q_temp1);
        locals.var_q_aexp = assign13520_e12850;
        locals.var_q_aexp_dn4 = ((locals.var_a0_dn4 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn4));
        locals.var_q_aexp_dn6 = ((locals.var_a0_dn6 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn6));
        locals.var_q_aexp_dn7 = ((locals.var_a0_dn7 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn7));
        locals.var_q_aexp_dn8 = ((locals.var_a0_dn8 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn8));
        locals.var_q_aexp_dn9 = ((locals.var_a0_dn9 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn9));

        let assign13530_e12853: f64 = (locals.var_q_k1q1 * locals.var_q_k1q1);
        let assign13530_e12855: f64 = (assign13530_e12853 - locals.var_q_aexp);
        locals.var_q_qsq = assign13530_e12855;
        locals.var_q_qsq_dn4 = (((locals.var_q_k1q1_dn4 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn4)) - locals.var_q_aexp_dn4);
        locals.var_q_qsq_dn6 = (((locals.var_q_k1q1_dn6 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn6)) - locals.var_q_aexp_dn6);
        locals.var_q_qsq_dn7 = (((locals.var_q_k1q1_dn7 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn7)) - locals.var_q_aexp_dn7);
        locals.var_q_qsq_dn8 = (((locals.var_q_k1q1_dn8 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn8)) - locals.var_q_aexp_dn8);
        locals.var_q_qsq_dn9 = (((locals.var_q_k1q1_dn9 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn9)) - locals.var_q_aexp_dn9);

        let assign13540_e12858: f64 = (2.0 * locals.var_k1);
        let assign13540_e12860: f64 = (assign13540_e12858 * locals.var_q_k1q1);
        let assign13540_e12862: f64 = (assign13540_e12860 + locals.var_q_aexp);
        locals.var_q_d1_qsq = assign13540_e12862;
        locals.var_q_d1_qsq_dn4 = ((((2.0 * locals.var_k1_dn4) * locals.var_q_k1q1) + (assign13540_e12858 * locals.var_q_k1q1_dn4)) + locals.var_q_aexp_dn4);
        locals.var_q_d1_qsq_dn6 = ((((2.0 * locals.var_k1_dn6) * locals.var_q_k1q1) + (assign13540_e12858 * locals.var_q_k1q1_dn6)) + locals.var_q_aexp_dn6);
        locals.var_q_d1_qsq_dn7 = ((((2.0 * locals.var_k1_dn7) * locals.var_q_k1q1) + (assign13540_e12858 * locals.var_q_k1q1_dn7)) + locals.var_q_aexp_dn7);
        locals.var_q_d1_qsq_dn8 = ((((2.0 * locals.var_k1_dn8) * locals.var_q_k1q1) + (assign13540_e12858 * locals.var_q_k1q1_dn8)) + locals.var_q_aexp_dn8);
        locals.var_q_d1_qsq_dn9 = ((((2.0 * locals.var_k1_dn9) * locals.var_q_k1q1) + (assign13540_e12858 * locals.var_q_k1q1_dn9)) + locals.var_q_aexp_dn9);

        let assign13550_e12865: f64 = (2.0 * locals.var_k1);
        let assign13550_e12867: f64 = (assign13550_e12865 * locals.var_k1);
        let assign13550_e12869: f64 = (assign13550_e12867 - locals.var_q_aexp);
        locals.var_q_d2_qsq = assign13550_e12869;
        locals.var_q_d2_qsq_dn4 = ((((2.0 * locals.var_k1_dn4) * locals.var_k1) + (assign13550_e12865 * locals.var_k1_dn4)) - locals.var_q_aexp_dn4);
        locals.var_q_d2_qsq_dn6 = ((((2.0 * locals.var_k1_dn6) * locals.var_k1) + (assign13550_e12865 * locals.var_k1_dn6)) - locals.var_q_aexp_dn6);
        locals.var_q_d2_qsq_dn7 = ((((2.0 * locals.var_k1_dn7) * locals.var_k1) + (assign13550_e12865 * locals.var_k1_dn7)) - locals.var_q_aexp_dn7);
        locals.var_q_d2_qsq_dn8 = ((((2.0 * locals.var_k1_dn8) * locals.var_k1) + (assign13550_e12865 * locals.var_k1_dn8)) - locals.var_q_aexp_dn8);
        locals.var_q_d2_qsq_dn9 = ((((2.0 * locals.var_k1_dn9) * locals.var_k1) + (assign13550_e12865 * locals.var_k1_dn9)) - locals.var_q_aexp_dn9);

        let assign13560_e12872: f64 = (-0.005);
        let assign13560_e12873: f64 = if locals.var_q_qsq < assign13560_e12872 { 1.0 } else { 0.0 };
        locals.var_guard573 = assign13560_e12873;

        let (assign13570_e12879, assign13570_e12879_d_n4, assign13570_e12879_d_n6, assign13570_e12879_d_n7, assign13570_e12879_d_n8, assign13570_e12879_d_n9,) = {
    if (locals.var_guard573 != 0.0) {
        let assign13570_e12876: f64 = (locals.var_q_qsq).abs();
        let assign13570_e12877: f64 = (assign13570_e12876).sqrt();
        (assign13570_e12877, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign13570_e12877)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign13570_e12877)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign13570_e12877)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign13570_e12877)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign13570_e12877)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign13570_e12879;
        locals.var_q_rac_qsq_dn4 = assign13570_e12879_d_n4;
        locals.var_q_rac_qsq_dn6 = assign13570_e12879_d_n6;
        locals.var_q_rac_qsq_dn7 = assign13570_e12879_d_n7;
        locals.var_q_rac_qsq_dn8 = assign13570_e12879_d_n8;
        locals.var_q_rac_qsq_dn9 = assign13570_e12879_d_n9;

        let (assign13580_e12888, assign13580_e12888_d_n4, assign13580_e12888_d_n6, assign13580_e12888_d_n7, assign13580_e12888_d_n8, assign13580_e12888_d_n9,) = {
    if (locals.var_guard573 != 0.0) {
        let assign13580_e12884: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign13580_e12885: f64 = (assign13580_e12884).tan();
        let assign13580_e12886: f64 = (locals.var_q_rac_qsq / assign13580_e12885);
        (assign13580_e12886, (((locals.var_q_rac_qsq_dn4 * assign13580_e12885) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn4) / ((assign13580_e12884).cos() * (assign13580_e12884).cos())))) / (assign13580_e12885 * assign13580_e12885)), (((locals.var_q_rac_qsq_dn6 * assign13580_e12885) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn6) / ((assign13580_e12884).cos() * (assign13580_e12884).cos())))) / (assign13580_e12885 * assign13580_e12885)), (((locals.var_q_rac_qsq_dn7 * assign13580_e12885) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn7) / ((assign13580_e12884).cos() * (assign13580_e12884).cos())))) / (assign13580_e12885 * assign13580_e12885)), (((locals.var_q_rac_qsq_dn8 * assign13580_e12885) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn8) / ((assign13580_e12884).cos() * (assign13580_e12884).cos())))) / (assign13580_e12885 * assign13580_e12885)), (((locals.var_q_rac_qsq_dn9 * assign13580_e12885) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn9) / ((assign13580_e12884).cos() * (assign13580_e12884).cos())))) / (assign13580_e12885 * assign13580_e12885)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign13580_e12888;
        locals.var_q_qcoth_dn4 = assign13580_e12888_d_n4;
        locals.var_q_qcoth_dn6 = assign13580_e12888_d_n6;
        locals.var_q_qcoth_dn7 = assign13580_e12888_d_n7;
        locals.var_q_qcoth_dn8 = assign13580_e12888_d_n8;
        locals.var_q_qcoth_dn9 = assign13580_e12888_d_n9;

        let (assign13590_e12896, assign13590_e12896_d_n4, assign13590_e12896_d_n6, assign13590_e12896_d_n7, assign13590_e12896_d_n8, assign13590_e12896_d_n9,) = {
    if (locals.var_guard573 != 0.0) {
        let assign13590_e12892: f64 = (0.25 * locals.var_q_d1_qsq);
        let assign13590_e12894: f64 = (assign13590_e12892 / locals.var_q_qsq);
        (assign13590_e12894, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign13590_e12892 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign13590_e12892 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign13590_e12892 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign13590_e12892 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign13590_e12892 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign13590_e12896;
        locals.var_q_temp1_dn4 = assign13590_e12896_d_n4;
        locals.var_q_temp1_dn6 = assign13590_e12896_d_n6;
        locals.var_q_temp1_dn7 = assign13590_e12896_d_n7;
        locals.var_q_temp1_dn8 = assign13590_e12896_d_n8;
        locals.var_q_temp1_dn9 = assign13590_e12896_d_n9;

        let (assign13600_e12908, assign13600_e12908_d_n4, assign13600_e12908_d_n6, assign13600_e12908_d_n7, assign13600_e12908_d_n8, assign13600_e12908_d_n9,) = {
    if (locals.var_guard573 != 0.0) {
        let assign13600_e12902: f64 = (2.0 - locals.var_q_qcoth);
        let assign13600_e12903: f64 = (locals.var_q_qcoth * assign13600_e12902);
        let assign13600_e12904: f64 = (locals.var_q_qsq + assign13600_e12903);
        let assign13600_e12906: f64 = (assign13600_e12904 * locals.var_q_temp1);
        (assign13600_e12906, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign13600_e12902) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign13600_e12904 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign13600_e12902) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign13600_e12904 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign13600_e12902) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign13600_e12904 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign13600_e12902) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign13600_e12904 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign13600_e12902) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign13600_e12904 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign13600_e12908;
        locals.var_q_d1_qcoth_dn4 = assign13600_e12908_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign13600_e12908_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign13600_e12908_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign13600_e12908_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign13600_e12908_d_n9;

        let (assign13610_e12928, assign13610_e12928_d_n4, assign13610_e12928_d_n6, assign13610_e12928_d_n7, assign13610_e12928_d_n8, assign13610_e12928_d_n9,) = {
    if (locals.var_guard573 != 0.0) {
        let assign13610_e12913: f64 = (2.0 * locals.var_q_d1_qcoth);
        let assign13610_e12916: f64 = (1.0 + locals.var_q_qcoth);
        let assign13610_e12917: f64 = (assign13610_e12913 * assign13610_e12916);
        let assign13610_e12918: f64 = (locals.var_q_d1_qsq - assign13610_e12917);
        let assign13610_e12920: f64 = (assign13610_e12918 * locals.var_q_temp1);
        let assign13610_e12923: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
        let assign13610_e12925: f64 = (assign13610_e12923 / locals.var_q_d1_qsq);
        let assign13610_e12926: f64 = (assign13610_e12920 + assign13610_e12925);
        (assign13610_e12926, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign13610_e12916) + (assign13610_e12913 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign13610_e12918 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign13610_e12923 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign13610_e12916) + (assign13610_e12913 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign13610_e12918 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign13610_e12923 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign13610_e12916) + (assign13610_e12913 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign13610_e12918 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign13610_e12923 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign13610_e12916) + (assign13610_e12913 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign13610_e12918 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign13610_e12923 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign13610_e12916) + (assign13610_e12913 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign13610_e12918 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign13610_e12923 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign13610_e12928;
        locals.var_q_d2_qcoth_dn4 = assign13610_e12928_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign13610_e12928_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign13610_e12928_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign13610_e12928_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign13610_e12928_d_n9;

        let (assign13620_e12936, assign13620_e12936_d_n4, assign13620_e12936_d_n6, assign13620_e12936_d_n7, assign13620_e12936_d_n8, assign13620_e12936_d_n9,) = {
    if (locals.var_guard573 != 0.0) {
        let assign13620_e12933: f64 = (0.5 * locals.var_q_qcoth);
        let assign13620_e12934: f64 = (1.0 - assign13620_e12933);
        (assign13620_e12934, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign13620_e12936;
        locals.var_q_temp2_dn4 = assign13620_e12936_d_n4;
        locals.var_q_temp2_dn6 = assign13620_e12936_d_n6;
        locals.var_q_temp2_dn7 = assign13620_e12936_d_n7;
        locals.var_q_temp2_dn8 = assign13620_e12936_d_n8;
        locals.var_q_temp2_dn9 = assign13620_e12936_d_n9;

        let (assign13630_e12944, assign13630_e12944_d_n4, assign13630_e12944_d_n6, assign13630_e12944_d_n7, assign13630_e12944_d_n8, assign13630_e12944_d_n9,) = {
    if (locals.var_guard573 != 0.0) {
        let assign13630_e12940: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
        let assign13630_e12942: f64 = (assign13630_e12940 * locals.var_q_temp2);
        (assign13630_e12942, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13630_e12940 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13630_e12940 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13630_e12940 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13630_e12940 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13630_e12940 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign13630_e12944;
        locals.var_q_d1_ln_dn4 = assign13630_e12944_d_n4;
        locals.var_q_d1_ln_dn6 = assign13630_e12944_d_n6;
        locals.var_q_d1_ln_dn7 = assign13630_e12944_d_n7;
        locals.var_q_d1_ln_dn8 = assign13630_e12944_d_n8;
        locals.var_q_d1_ln_dn9 = assign13630_e12944_d_n9;

        let (assign13640_e12960, assign13640_e12960_d_n4, assign13640_e12960_d_n6, assign13640_e12960_d_n7, assign13640_e12960_d_n8, assign13640_e12960_d_n9,) = {
    if (locals.var_guard573 != 0.0) {
        let assign13640_e12948: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
        let assign13640_e12953: f64 = (0.5 * locals.var_q_d1_qcoth);
        let assign13640_e12954: f64 = (locals.var_q_d1_ln + assign13640_e12953);
        let assign13640_e12955: f64 = (locals.var_q_d1_qsq * assign13640_e12954);
        let assign13640_e12956: f64 = (assign13640_e12948 - assign13640_e12955);
        let assign13640_e12958: f64 = (assign13640_e12956 / locals.var_q_qsq);
        (assign13640_e12958, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign13640_e12954) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign13640_e12956 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign13640_e12954) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign13640_e12956 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign13640_e12954) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign13640_e12956 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign13640_e12954) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign13640_e12956 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign13640_e12954) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign13640_e12956 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign13640_e12960;
        locals.var_q_d2_ln_dn4 = assign13640_e12960_d_n4;
        locals.var_q_d2_ln_dn6 = assign13640_e12960_d_n6;
        locals.var_q_d2_ln_dn7 = assign13640_e12960_d_n7;
        locals.var_q_d2_ln_dn8 = assign13640_e12960_d_n8;
        locals.var_q_d2_ln_dn9 = assign13640_e12960_d_n9;

        let assign13650_e12963: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard574 = assign13650_e12963;

        let (assign13660_e12972, assign13660_e12972_d_n4, assign13660_e12972_d_n6, assign13660_e12972_d_n7, assign13660_e12972_d_n8, assign13660_e12972_d_n9,) = {
    if ((locals.var_guard573 == 0.0) && (locals.var_guard574 != 0.0)) {
        let assign13660_e12969: f64 = (locals.var_q_qsq).abs();
        let assign13660_e12970: f64 = (assign13660_e12969).sqrt();
        (assign13660_e12970, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign13660_e12970)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign13660_e12970)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign13660_e12970)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign13660_e12970)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign13660_e12970)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign13660_e12972;
        locals.var_q_rac_qsq_dn4 = assign13660_e12972_d_n4;
        locals.var_q_rac_qsq_dn6 = assign13660_e12972_d_n6;
        locals.var_q_rac_qsq_dn7 = assign13660_e12972_d_n7;
        locals.var_q_rac_qsq_dn8 = assign13660_e12972_d_n8;
        locals.var_q_rac_qsq_dn9 = assign13660_e12972_d_n9;

        let (assign13670_e12981, assign13670_e12981_d_n4, assign13670_e12981_d_n6, assign13670_e12981_d_n7, assign13670_e12981_d_n8, assign13670_e12981_d_n9,) = {
    if ((locals.var_guard573 == 0.0) && (locals.var_guard574 != 0.0)) {
        let assign13670_e12978: f64 = (-locals.var_q_rac_qsq);
        let assign13670_e12979: f64 = (assign13670_e12978).exp();
        (assign13670_e12979, (assign13670_e12979 * (-locals.var_q_rac_qsq_dn4)), (assign13670_e12979 * (-locals.var_q_rac_qsq_dn6)), (assign13670_e12979 * (-locals.var_q_rac_qsq_dn7)), (assign13670_e12979 * (-locals.var_q_rac_qsq_dn8)), (assign13670_e12979 * (-locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_invexpq, locals.var_q_invexpq_dn4, locals.var_q_invexpq_dn6, locals.var_q_invexpq_dn7, locals.var_q_invexpq_dn8, locals.var_q_invexpq_dn9,)
    }
};
        locals.var_q_invexpq = assign13670_e12981;
        locals.var_q_invexpq_dn4 = assign13670_e12981_d_n4;
        locals.var_q_invexpq_dn6 = assign13670_e12981_d_n6;
        locals.var_q_invexpq_dn7 = assign13670_e12981_d_n7;
        locals.var_q_invexpq_dn8 = assign13670_e12981_d_n8;
        locals.var_q_invexpq_dn9 = assign13670_e12981_d_n9;

        let (assign13680_e12996, assign13680_e12996_d_n4, assign13680_e12996_d_n6, assign13680_e12996_d_n7, assign13680_e12996_d_n8, assign13680_e12996_d_n9,) = {
    if ((locals.var_guard573 == 0.0) && (locals.var_guard574 != 0.0)) {
        let assign13680_e12989: f64 = (1.0 + locals.var_q_invexpq);
        let assign13680_e12990: f64 = (locals.var_q_rac_qsq * assign13680_e12989);
        let assign13680_e12993: f64 = (1.0 - locals.var_q_invexpq);
        let assign13680_e12994: f64 = (assign13680_e12990 / assign13680_e12993);
        (assign13680_e12994, (((((locals.var_q_rac_qsq_dn4 * assign13680_e12989) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn4)) * assign13680_e12993) - (assign13680_e12990 * (-locals.var_q_invexpq_dn4))) / (assign13680_e12993 * assign13680_e12993)), (((((locals.var_q_rac_qsq_dn6 * assign13680_e12989) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn6)) * assign13680_e12993) - (assign13680_e12990 * (-locals.var_q_invexpq_dn6))) / (assign13680_e12993 * assign13680_e12993)), (((((locals.var_q_rac_qsq_dn7 * assign13680_e12989) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn7)) * assign13680_e12993) - (assign13680_e12990 * (-locals.var_q_invexpq_dn7))) / (assign13680_e12993 * assign13680_e12993)), (((((locals.var_q_rac_qsq_dn8 * assign13680_e12989) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn8)) * assign13680_e12993) - (assign13680_e12990 * (-locals.var_q_invexpq_dn8))) / (assign13680_e12993 * assign13680_e12993)), (((((locals.var_q_rac_qsq_dn9 * assign13680_e12989) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn9)) * assign13680_e12993) - (assign13680_e12990 * (-locals.var_q_invexpq_dn9))) / (assign13680_e12993 * assign13680_e12993)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign13680_e12996;
        locals.var_q_qcoth_dn4 = assign13680_e12996_d_n4;
        locals.var_q_qcoth_dn6 = assign13680_e12996_d_n6;
        locals.var_q_qcoth_dn7 = assign13680_e12996_d_n7;
        locals.var_q_qcoth_dn8 = assign13680_e12996_d_n8;
        locals.var_q_qcoth_dn9 = assign13680_e12996_d_n9;

    }

    pub(super) fn stamp_transient_block_33(
        locals: &mut StampLocals,
    ) {
        let (assign13690_e13007, assign13690_e13007_d_n4, assign13690_e13007_d_n6, assign13690_e13007_d_n7, assign13690_e13007_d_n8, assign13690_e13007_d_n9,) = {
    if ((locals.var_guard573 == 0.0) && (locals.var_guard574 != 0.0)) {
        let assign13690_e13003: f64 = (0.25 * locals.var_q_d1_qsq);
        let assign13690_e13005: f64 = (assign13690_e13003 / locals.var_q_qsq);
        (assign13690_e13005, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign13690_e13003 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign13690_e13003 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign13690_e13003 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign13690_e13003 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign13690_e13003 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign13690_e13007;
        locals.var_q_temp1_dn4 = assign13690_e13007_d_n4;
        locals.var_q_temp1_dn6 = assign13690_e13007_d_n6;
        locals.var_q_temp1_dn7 = assign13690_e13007_d_n7;
        locals.var_q_temp1_dn8 = assign13690_e13007_d_n8;
        locals.var_q_temp1_dn9 = assign13690_e13007_d_n9;

        let (assign13700_e13022, assign13700_e13022_d_n4, assign13700_e13022_d_n6, assign13700_e13022_d_n7, assign13700_e13022_d_n8, assign13700_e13022_d_n9,) = {
    if ((locals.var_guard573 == 0.0) && (locals.var_guard574 != 0.0)) {
        let assign13700_e13016: f64 = (2.0 - locals.var_q_qcoth);
        let assign13700_e13017: f64 = (locals.var_q_qcoth * assign13700_e13016);
        let assign13700_e13018: f64 = (locals.var_q_qsq + assign13700_e13017);
        let assign13700_e13020: f64 = (assign13700_e13018 * locals.var_q_temp1);
        (assign13700_e13020, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign13700_e13016) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign13700_e13018 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign13700_e13016) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign13700_e13018 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign13700_e13016) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign13700_e13018 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign13700_e13016) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign13700_e13018 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign13700_e13016) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign13700_e13018 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign13700_e13022;
        locals.var_q_d1_qcoth_dn4 = assign13700_e13022_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign13700_e13022_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign13700_e13022_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign13700_e13022_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign13700_e13022_d_n9;

        let (assign13710_e13045, assign13710_e13045_d_n4, assign13710_e13045_d_n6, assign13710_e13045_d_n7, assign13710_e13045_d_n8, assign13710_e13045_d_n9,) = {
    if ((locals.var_guard573 == 0.0) && (locals.var_guard574 != 0.0)) {
        let assign13710_e13030: f64 = (2.0 * locals.var_q_d1_qcoth);
        let assign13710_e13033: f64 = (1.0 + locals.var_q_qcoth);
        let assign13710_e13034: f64 = (assign13710_e13030 * assign13710_e13033);
        let assign13710_e13035: f64 = (locals.var_q_d1_qsq - assign13710_e13034);
        let assign13710_e13037: f64 = (assign13710_e13035 * locals.var_q_temp1);
        let assign13710_e13040: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
        let assign13710_e13042: f64 = (assign13710_e13040 / locals.var_q_d1_qsq);
        let assign13710_e13043: f64 = (assign13710_e13037 + assign13710_e13042);
        (assign13710_e13043, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign13710_e13033) + (assign13710_e13030 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign13710_e13035 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign13710_e13040 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign13710_e13033) + (assign13710_e13030 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign13710_e13035 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign13710_e13040 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign13710_e13033) + (assign13710_e13030 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign13710_e13035 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign13710_e13040 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign13710_e13033) + (assign13710_e13030 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign13710_e13035 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign13710_e13040 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign13710_e13033) + (assign13710_e13030 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign13710_e13035 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign13710_e13040 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign13710_e13045;
        locals.var_q_d2_qcoth_dn4 = assign13710_e13045_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign13710_e13045_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign13710_e13045_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign13710_e13045_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign13710_e13045_d_n9;

        let (assign13720_e13056, assign13720_e13056_d_n4, assign13720_e13056_d_n6, assign13720_e13056_d_n7, assign13720_e13056_d_n8, assign13720_e13056_d_n9,) = {
    if ((locals.var_guard573 == 0.0) && (locals.var_guard574 != 0.0)) {
        let assign13720_e13053: f64 = (0.5 * locals.var_q_qcoth);
        let assign13720_e13054: f64 = (1.0 - assign13720_e13053);
        (assign13720_e13054, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign13720_e13056;
        locals.var_q_temp2_dn4 = assign13720_e13056_d_n4;
        locals.var_q_temp2_dn6 = assign13720_e13056_d_n6;
        locals.var_q_temp2_dn7 = assign13720_e13056_d_n7;
        locals.var_q_temp2_dn8 = assign13720_e13056_d_n8;
        locals.var_q_temp2_dn9 = assign13720_e13056_d_n9;

        let (assign13730_e13067, assign13730_e13067_d_n4, assign13730_e13067_d_n6, assign13730_e13067_d_n7, assign13730_e13067_d_n8, assign13730_e13067_d_n9,) = {
    if ((locals.var_guard573 == 0.0) && (locals.var_guard574 != 0.0)) {
        let assign13730_e13063: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
        let assign13730_e13065: f64 = (assign13730_e13063 * locals.var_q_temp2);
        (assign13730_e13065, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13730_e13063 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13730_e13063 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13730_e13063 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13730_e13063 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13730_e13063 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign13730_e13067;
        locals.var_q_d1_ln_dn4 = assign13730_e13067_d_n4;
        locals.var_q_d1_ln_dn6 = assign13730_e13067_d_n6;
        locals.var_q_d1_ln_dn7 = assign13730_e13067_d_n7;
        locals.var_q_d1_ln_dn8 = assign13730_e13067_d_n8;
        locals.var_q_d1_ln_dn9 = assign13730_e13067_d_n9;

        let (assign13740_e13086, assign13740_e13086_d_n4, assign13740_e13086_d_n6, assign13740_e13086_d_n7, assign13740_e13086_d_n8, assign13740_e13086_d_n9,) = {
    if ((locals.var_guard573 == 0.0) && (locals.var_guard574 != 0.0)) {
        let assign13740_e13074: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
        let assign13740_e13079: f64 = (0.5 * locals.var_q_d1_qcoth);
        let assign13740_e13080: f64 = (locals.var_q_d1_ln + assign13740_e13079);
        let assign13740_e13081: f64 = (locals.var_q_d1_qsq * assign13740_e13080);
        let assign13740_e13082: f64 = (assign13740_e13074 - assign13740_e13081);
        let assign13740_e13084: f64 = (assign13740_e13082 / locals.var_q_qsq);
        (assign13740_e13084, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign13740_e13080) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign13740_e13082 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign13740_e13080) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign13740_e13082 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign13740_e13080) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign13740_e13082 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign13740_e13080) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign13740_e13082 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign13740_e13080) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign13740_e13082 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign13740_e13086;
        locals.var_q_d2_ln_dn4 = assign13740_e13086_d_n4;
        locals.var_q_d2_ln_dn6 = assign13740_e13086_d_n6;
        locals.var_q_d2_ln_dn7 = assign13740_e13086_d_n7;
        locals.var_q_d2_ln_dn8 = assign13740_e13086_d_n8;
        locals.var_q_d2_ln_dn9 = assign13740_e13086_d_n9;

        let (assign13750_e13112, assign13750_e13112_d_n4, assign13750_e13112_d_n6, assign13750_e13112_d_n7, assign13750_e13112_d_n8, assign13750_e13112_d_n9,) = {
    if ((locals.var_guard573 == 0.0) && (locals.var_guard574 == 0.0)) {
        let assign13750_e13096: f64 = (locals.var_q_qsq * 0.0166666666667);
        let assign13750_e13100: f64 = (locals.var_q_qsq * 0.0238095238095);
        let assign13750_e13104: f64 = (locals.var_q_qsq * 0.025);
        let assign13750_e13105: f64 = (1.0 - assign13750_e13104);
        let assign13750_e13106: f64 = (assign13750_e13100 * assign13750_e13105);
        let assign13750_e13107: f64 = (1.0 - assign13750_e13106);
        let assign13750_e13108: f64 = (assign13750_e13096 * assign13750_e13107);
        let assign13750_e13109: f64 = (1.0 - assign13750_e13108);
        let assign13750_e13110: f64 = (0.1666666666667 * assign13750_e13109);
        (assign13750_e13110, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0166666666667) * assign13750_e13107) + (assign13750_e13096 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign13750_e13105) + (assign13750_e13100 * (-(locals.var_q_qsq_dn4 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0166666666667) * assign13750_e13107) + (assign13750_e13096 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign13750_e13105) + (assign13750_e13100 * (-(locals.var_q_qsq_dn6 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0166666666667) * assign13750_e13107) + (assign13750_e13096 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign13750_e13105) + (assign13750_e13100 * (-(locals.var_q_qsq_dn7 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0166666666667) * assign13750_e13107) + (assign13750_e13096 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign13750_e13105) + (assign13750_e13100 * (-(locals.var_q_qsq_dn8 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0166666666667) * assign13750_e13107) + (assign13750_e13096 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign13750_e13105) + (assign13750_e13100 * (-(locals.var_q_qsq_dn9 * 0.025))))))))),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign13750_e13112;
        locals.var_q_temp3_dn4 = assign13750_e13112_d_n4;
        locals.var_q_temp3_dn6 = assign13750_e13112_d_n6;
        locals.var_q_temp3_dn7 = assign13750_e13112_d_n7;
        locals.var_q_temp3_dn8 = assign13750_e13112_d_n8;
        locals.var_q_temp3_dn9 = assign13750_e13112_d_n9;

        let (assign13760_e13124, assign13760_e13124_d_n4, assign13760_e13124_d_n6, assign13760_e13124_d_n7, assign13760_e13124_d_n8, assign13760_e13124_d_n9,) = {
    if ((locals.var_guard573 == 0.0) && (locals.var_guard574 == 0.0)) {
        let assign13760_e13121: f64 = (locals.var_q_qsq * locals.var_q_temp3);
        let assign13760_e13122: f64 = (2.0 + assign13760_e13121);
        (assign13760_e13122, ((locals.var_q_qsq_dn4 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn4)), ((locals.var_q_qsq_dn6 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn6)), ((locals.var_q_qsq_dn7 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn7)), ((locals.var_q_qsq_dn8 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn8)), ((locals.var_q_qsq_dn9 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn9)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign13760_e13124;
        locals.var_q_qcoth_dn4 = assign13760_e13124_d_n4;
        locals.var_q_qcoth_dn6 = assign13760_e13124_d_n6;
        locals.var_q_qcoth_dn7 = assign13760_e13124_d_n7;
        locals.var_q_qcoth_dn8 = assign13760_e13124_d_n8;
        locals.var_q_qcoth_dn9 = assign13760_e13124_d_n9;

        let (assign13770_e13150, assign13770_e13150_d_n4, assign13770_e13150_d_n6, assign13770_e13150_d_n7, assign13770_e13150_d_n8, assign13770_e13150_d_n9,) = {
    if ((locals.var_guard573 == 0.0) && (locals.var_guard574 == 0.0)) {
        let assign13770_e13134: f64 = (locals.var_q_qsq * 0.0333333333333);
        let assign13770_e13138: f64 = (locals.var_q_qsq * 0.0357142857143);
        let assign13770_e13142: f64 = (locals.var_q_qsq * 0.0333333333333);
        let assign13770_e13143: f64 = (1.0 - assign13770_e13142);
        let assign13770_e13144: f64 = (assign13770_e13138 * assign13770_e13143);
        let assign13770_e13145: f64 = (1.0 - assign13770_e13144);
        let assign13770_e13146: f64 = (assign13770_e13134 * assign13770_e13145);
        let assign13770_e13147: f64 = (1.0 - assign13770_e13146);
        let assign13770_e13148: f64 = (0.1666666666667 * assign13770_e13147);
        (assign13770_e13148, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0333333333333) * assign13770_e13145) + (assign13770_e13134 * (-(((locals.var_q_qsq_dn4 * 0.0357142857143) * assign13770_e13143) + (assign13770_e13138 * (-(locals.var_q_qsq_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0333333333333) * assign13770_e13145) + (assign13770_e13134 * (-(((locals.var_q_qsq_dn6 * 0.0357142857143) * assign13770_e13143) + (assign13770_e13138 * (-(locals.var_q_qsq_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0333333333333) * assign13770_e13145) + (assign13770_e13134 * (-(((locals.var_q_qsq_dn7 * 0.0357142857143) * assign13770_e13143) + (assign13770_e13138 * (-(locals.var_q_qsq_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0333333333333) * assign13770_e13145) + (assign13770_e13134 * (-(((locals.var_q_qsq_dn8 * 0.0357142857143) * assign13770_e13143) + (assign13770_e13138 * (-(locals.var_q_qsq_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0333333333333) * assign13770_e13145) + (assign13770_e13134 * (-(((locals.var_q_qsq_dn9 * 0.0357142857143) * assign13770_e13143) + (assign13770_e13138 * (-(locals.var_q_qsq_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign13770_e13150;
        locals.var_q_temp1_dn4 = assign13770_e13150_d_n4;
        locals.var_q_temp1_dn6 = assign13770_e13150_d_n6;
        locals.var_q_temp1_dn7 = assign13770_e13150_d_n7;
        locals.var_q_temp1_dn8 = assign13770_e13150_d_n8;
        locals.var_q_temp1_dn9 = assign13770_e13150_d_n9;

        let (assign13780_e13160, assign13780_e13160_d_n4, assign13780_e13160_d_n6, assign13780_e13160_d_n7, assign13780_e13160_d_n8, assign13780_e13160_d_n9,) = {
    if ((locals.var_guard573 == 0.0) && (locals.var_guard574 == 0.0)) {
        let assign13780_e13158: f64 = (locals.var_q_d1_qsq * locals.var_q_temp1);
        (assign13780_e13158, ((locals.var_q_d1_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn4)), ((locals.var_q_d1_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn6)), ((locals.var_q_d1_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn7)), ((locals.var_q_d1_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn8)), ((locals.var_q_d1_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign13780_e13160;
        locals.var_q_d1_qcoth_dn4 = assign13780_e13160_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign13780_e13160_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign13780_e13160_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign13780_e13160_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign13780_e13160_d_n9;

        let (assign13790_e13186, assign13790_e13186_d_n4, assign13790_e13186_d_n6, assign13790_e13186_d_n7, assign13790_e13186_d_n8, assign13790_e13186_d_n9,) = {
    if ((locals.var_guard573 == 0.0) && (locals.var_guard574 == 0.0)) {
        let assign13790_e13170: f64 = (locals.var_q_qsq * 0.0714285714286);
        let assign13790_e13174: f64 = (0.05 * locals.var_q_qsq);
        let assign13790_e13178: f64 = (0.0420875420875421 * locals.var_q_qsq);
        let assign13790_e13179: f64 = (1.0 - assign13790_e13178);
        let assign13790_e13180: f64 = (assign13790_e13174 * assign13790_e13179);
        let assign13790_e13181: f64 = (1.0 - assign13790_e13180);
        let assign13790_e13182: f64 = (assign13790_e13170 * assign13790_e13181);
        let assign13790_e13183: f64 = (1.0 - assign13790_e13182);
        let assign13790_e13184: f64 = (0.0055555555556 * assign13790_e13183);
        (assign13790_e13184, (0.0055555555556 * (-(((locals.var_q_qsq_dn4 * 0.0714285714286) * assign13790_e13181) + (assign13790_e13170 * (-(((0.05 * locals.var_q_qsq_dn4) * assign13790_e13179) + (assign13790_e13174 * (-(0.0420875420875421 * locals.var_q_qsq_dn4))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn6 * 0.0714285714286) * assign13790_e13181) + (assign13790_e13170 * (-(((0.05 * locals.var_q_qsq_dn6) * assign13790_e13179) + (assign13790_e13174 * (-(0.0420875420875421 * locals.var_q_qsq_dn6))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn7 * 0.0714285714286) * assign13790_e13181) + (assign13790_e13170 * (-(((0.05 * locals.var_q_qsq_dn7) * assign13790_e13179) + (assign13790_e13174 * (-(0.0420875420875421 * locals.var_q_qsq_dn7))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn8 * 0.0714285714286) * assign13790_e13181) + (assign13790_e13170 * (-(((0.05 * locals.var_q_qsq_dn8) * assign13790_e13179) + (assign13790_e13174 * (-(0.0420875420875421 * locals.var_q_qsq_dn8))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn9 * 0.0714285714286) * assign13790_e13181) + (assign13790_e13170 * (-(((0.05 * locals.var_q_qsq_dn9) * assign13790_e13179) + (assign13790_e13174 * (-(0.0420875420875421 * locals.var_q_qsq_dn9))))))))),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign13790_e13186;
        locals.var_q_temp2_dn4 = assign13790_e13186_d_n4;
        locals.var_q_temp2_dn6 = assign13790_e13186_d_n6;
        locals.var_q_temp2_dn7 = assign13790_e13186_d_n7;
        locals.var_q_temp2_dn8 = assign13790_e13186_d_n8;
        locals.var_q_temp2_dn9 = assign13790_e13186_d_n9;

        let (assign13800_e13202, assign13800_e13202_d_n4, assign13800_e13202_d_n6, assign13800_e13202_d_n7, assign13800_e13202_d_n8, assign13800_e13202_d_n9,) = {
    if ((locals.var_guard573 == 0.0) && (locals.var_guard574 == 0.0)) {
        let assign13800_e13194: f64 = (locals.var_q_d2_qsq * locals.var_q_temp1);
        let assign13800_e13197: f64 = (locals.var_q_d1_qsq * locals.var_q_d1_qsq);
        let assign13800_e13199: f64 = (assign13800_e13197 * locals.var_q_temp2);
        let assign13800_e13200: f64 = (assign13800_e13194 - assign13800_e13199);
        (assign13800_e13200, (((locals.var_q_d2_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn4)) - ((((locals.var_q_d1_qsq_dn4 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn4)) * locals.var_q_temp2) + (assign13800_e13197 * locals.var_q_temp2_dn4))), (((locals.var_q_d2_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn6)) - ((((locals.var_q_d1_qsq_dn6 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn6)) * locals.var_q_temp2) + (assign13800_e13197 * locals.var_q_temp2_dn6))), (((locals.var_q_d2_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn7)) - ((((locals.var_q_d1_qsq_dn7 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn7)) * locals.var_q_temp2) + (assign13800_e13197 * locals.var_q_temp2_dn7))), (((locals.var_q_d2_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn8)) - ((((locals.var_q_d1_qsq_dn8 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn8)) * locals.var_q_temp2) + (assign13800_e13197 * locals.var_q_temp2_dn8))), (((locals.var_q_d2_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn9)) - ((((locals.var_q_d1_qsq_dn9 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn9)) * locals.var_q_temp2) + (assign13800_e13197 * locals.var_q_temp2_dn9))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign13800_e13202;
        locals.var_q_d2_qcoth_dn4 = assign13800_e13202_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign13800_e13202_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign13800_e13202_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign13800_e13202_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign13800_e13202_d_n9;

        let (assign13810_e13215, assign13810_e13215_d_n4, assign13810_e13215_d_n6, assign13810_e13215_d_n7, assign13810_e13215_d_n8, assign13810_e13215_d_n9,) = {
    if ((locals.var_guard573 == 0.0) && (locals.var_guard574 == 0.0)) {
        let assign13810_e13209: f64 = (-0.5);
        let assign13810_e13211: f64 = (assign13810_e13209 * locals.var_q_d1_qsq);
        let assign13810_e13213: f64 = (assign13810_e13211 * locals.var_q_temp3);
        (assign13810_e13213, (((assign13810_e13209 * locals.var_q_d1_qsq_dn4) * locals.var_q_temp3) + (assign13810_e13211 * locals.var_q_temp3_dn4)), (((assign13810_e13209 * locals.var_q_d1_qsq_dn6) * locals.var_q_temp3) + (assign13810_e13211 * locals.var_q_temp3_dn6)), (((assign13810_e13209 * locals.var_q_d1_qsq_dn7) * locals.var_q_temp3) + (assign13810_e13211 * locals.var_q_temp3_dn7)), (((assign13810_e13209 * locals.var_q_d1_qsq_dn8) * locals.var_q_temp3) + (assign13810_e13211 * locals.var_q_temp3_dn8)), (((assign13810_e13209 * locals.var_q_d1_qsq_dn9) * locals.var_q_temp3) + (assign13810_e13211 * locals.var_q_temp3_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign13810_e13215;
        locals.var_q_d1_ln_dn4 = assign13810_e13215_d_n4;
        locals.var_q_d1_ln_dn6 = assign13810_e13215_d_n6;
        locals.var_q_d1_ln_dn7 = assign13810_e13215_d_n7;
        locals.var_q_d1_ln_dn8 = assign13810_e13215_d_n8;
        locals.var_q_d1_ln_dn9 = assign13810_e13215_d_n9;

        let (assign13820_e13248, assign13820_e13248_d_n4, assign13820_e13248_d_n6, assign13820_e13248_d_n7, assign13820_e13248_d_n8, assign13820_e13248_d_n9,) = {
    if ((locals.var_guard573 == 0.0) && (locals.var_guard574 == 0.0)) {
        let assign13820_e13222: f64 = (-0.5);
        let assign13820_e13224: f64 = (assign13820_e13222 * locals.var_q_d2_qsq);
        let assign13820_e13226: f64 = (assign13820_e13224 * locals.var_q_temp3);
        let assign13820_e13229: f64 = (0.25 * 0.0055555555556);
        let assign13820_e13231: f64 = (assign13820_e13229 * locals.var_q_d1_qsq);
        let assign13820_e13233: f64 = (assign13820_e13231 * locals.var_q_d1_qsq);
        let assign13820_e13237: f64 = (locals.var_q_qsq * 0.0238095238095);
        let assign13820_e13241: f64 = (0.075 * locals.var_q_qsq);
        let assign13820_e13242: f64 = (2.0 - assign13820_e13241);
        let assign13820_e13243: f64 = (assign13820_e13237 * assign13820_e13242);
        let assign13820_e13244: f64 = (1.0 - assign13820_e13243);
        let assign13820_e13245: f64 = (assign13820_e13233 * assign13820_e13244);
        let assign13820_e13246: f64 = (assign13820_e13226 + assign13820_e13245);
        (assign13820_e13246, ((((assign13820_e13222 * locals.var_q_d2_qsq_dn4) * locals.var_q_temp3) + (assign13820_e13224 * locals.var_q_temp3_dn4)) + (((((assign13820_e13229 * locals.var_q_d1_qsq_dn4) * locals.var_q_d1_qsq) + (assign13820_e13231 * locals.var_q_d1_qsq_dn4)) * assign13820_e13244) + (assign13820_e13233 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign13820_e13242) + (assign13820_e13237 * (-(0.075 * locals.var_q_qsq_dn4)))))))), ((((assign13820_e13222 * locals.var_q_d2_qsq_dn6) * locals.var_q_temp3) + (assign13820_e13224 * locals.var_q_temp3_dn6)) + (((((assign13820_e13229 * locals.var_q_d1_qsq_dn6) * locals.var_q_d1_qsq) + (assign13820_e13231 * locals.var_q_d1_qsq_dn6)) * assign13820_e13244) + (assign13820_e13233 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign13820_e13242) + (assign13820_e13237 * (-(0.075 * locals.var_q_qsq_dn6)))))))), ((((assign13820_e13222 * locals.var_q_d2_qsq_dn7) * locals.var_q_temp3) + (assign13820_e13224 * locals.var_q_temp3_dn7)) + (((((assign13820_e13229 * locals.var_q_d1_qsq_dn7) * locals.var_q_d1_qsq) + (assign13820_e13231 * locals.var_q_d1_qsq_dn7)) * assign13820_e13244) + (assign13820_e13233 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign13820_e13242) + (assign13820_e13237 * (-(0.075 * locals.var_q_qsq_dn7)))))))), ((((assign13820_e13222 * locals.var_q_d2_qsq_dn8) * locals.var_q_temp3) + (assign13820_e13224 * locals.var_q_temp3_dn8)) + (((((assign13820_e13229 * locals.var_q_d1_qsq_dn8) * locals.var_q_d1_qsq) + (assign13820_e13231 * locals.var_q_d1_qsq_dn8)) * assign13820_e13244) + (assign13820_e13233 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign13820_e13242) + (assign13820_e13237 * (-(0.075 * locals.var_q_qsq_dn8)))))))), ((((assign13820_e13222 * locals.var_q_d2_qsq_dn9) * locals.var_q_temp3) + (assign13820_e13224 * locals.var_q_temp3_dn9)) + (((((assign13820_e13229 * locals.var_q_d1_qsq_dn9) * locals.var_q_d1_qsq) + (assign13820_e13231 * locals.var_q_d1_qsq_dn9)) * assign13820_e13244) + (assign13820_e13233 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign13820_e13242) + (assign13820_e13237 * (-(0.075 * locals.var_q_qsq_dn9)))))))),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign13820_e13248;
        locals.var_q_d2_ln_dn4 = assign13820_e13248_d_n4;
        locals.var_q_d2_ln_dn6 = assign13820_e13248_d_n6;
        locals.var_q_d2_ln_dn7 = assign13820_e13248_d_n7;
        locals.var_q_d2_ln_dn8 = assign13820_e13248_d_n8;
        locals.var_q_d2_ln_dn9 = assign13820_e13248_d_n9;

        let assign13830_e13251: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard575 = assign13830_e13251;

        let (assign13840_e13265, assign13840_e13265_d_n4, assign13840_e13265_d_n6, assign13840_e13265_d_n7, assign13840_e13265_d_n8, assign13840_e13265_d_n9,) = {
    if (locals.var_guard575 != 0.0) {
        let assign13840_e13255: f64 = (4.0 * locals.var_q_qsq);
        let assign13840_e13260: f64 = (2.0 - locals.var_q_invexpq);
        let assign13840_e13261: f64 = (locals.var_q_invexpq * assign13840_e13260);
        let assign13840_e13262: f64 = (1.0 - assign13840_e13261);
        let assign13840_e13263: f64 = (assign13840_e13255 / assign13840_e13262);
        (assign13840_e13263, ((((4.0 * locals.var_q_qsq_dn4) * assign13840_e13262) - (assign13840_e13255 * (-((locals.var_q_invexpq_dn4 * assign13840_e13260) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn4)))))) / (assign13840_e13262 * assign13840_e13262)), ((((4.0 * locals.var_q_qsq_dn6) * assign13840_e13262) - (assign13840_e13255 * (-((locals.var_q_invexpq_dn6 * assign13840_e13260) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn6)))))) / (assign13840_e13262 * assign13840_e13262)), ((((4.0 * locals.var_q_qsq_dn7) * assign13840_e13262) - (assign13840_e13255 * (-((locals.var_q_invexpq_dn7 * assign13840_e13260) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn7)))))) / (assign13840_e13262 * assign13840_e13262)), ((((4.0 * locals.var_q_qsq_dn8) * assign13840_e13262) - (assign13840_e13255 * (-((locals.var_q_invexpq_dn8 * assign13840_e13260) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn8)))))) / (assign13840_e13262 * assign13840_e13262)), ((((4.0 * locals.var_q_qsq_dn9) * assign13840_e13262) - (assign13840_e13255 * (-((locals.var_q_invexpq_dn9 * assign13840_e13260) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn9)))))) / (assign13840_e13262 * assign13840_e13262)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign13840_e13265;
        locals.var_q_temp2_dn4 = assign13840_e13265_d_n4;
        locals.var_q_temp2_dn6 = assign13840_e13265_d_n6;
        locals.var_q_temp2_dn7 = assign13840_e13265_d_n7;
        locals.var_q_temp2_dn8 = assign13840_e13265_d_n8;
        locals.var_q_temp2_dn9 = assign13840_e13265_d_n9;

        let (assign13850_e13271, assign13850_e13271_d_n4, assign13850_e13271_d_n6, assign13850_e13271_d_n7, assign13850_e13271_d_n8, assign13850_e13271_d_n9,) = {
    if (locals.var_guard575 != 0.0) {
        let assign13850_e13269: f64 = (locals.var_q_temp2 * locals.var_q_invexpq);
        (assign13850_e13269, ((locals.var_q_temp2_dn4 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn4)), ((locals.var_q_temp2_dn6 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn6)), ((locals.var_q_temp2_dn7 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn7)), ((locals.var_q_temp2_dn8 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn8)), ((locals.var_q_temp2_dn9 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn9)),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign13850_e13271;
        locals.var_q_sh_term_dn4 = assign13850_e13271_d_n4;
        locals.var_q_sh_term_dn6 = assign13850_e13271_d_n6;
        locals.var_q_sh_term_dn7 = assign13850_e13271_d_n7;
        locals.var_q_sh_term_dn8 = assign13850_e13271_d_n8;
        locals.var_q_sh_term_dn9 = assign13850_e13271_d_n9;

        let (assign13860_e13278, assign13860_e13278_d_n4, assign13860_e13278_d_n6, assign13860_e13278_d_n7, assign13860_e13278_d_n8, assign13860_e13278_d_n9,) = {
    if (locals.var_guard575 != 0.0) {
        let assign13860_e13274: f64 = (locals.var_q_temp2).ln();
        let assign13860_e13276: f64 = (assign13860_e13274 - locals.var_q_rac_qsq);
        (assign13860_e13276, ((locals.var_q_temp2_dn4 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn4), ((locals.var_q_temp2_dn6 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn6), ((locals.var_q_temp2_dn7 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn7), ((locals.var_q_temp2_dn8 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn8), ((locals.var_q_temp2_dn9 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn9),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign13860_e13278;
        locals.var_q_ln_term_dn4 = assign13860_e13278_d_n4;
        locals.var_q_ln_term_dn6 = assign13860_e13278_d_n6;
        locals.var_q_ln_term_dn7 = assign13860_e13278_d_n7;
        locals.var_q_ln_term_dn8 = assign13860_e13278_d_n8;
        locals.var_q_ln_term_dn9 = assign13860_e13278_d_n9;

        let assign13870_e13281: f64 = (-0.005);
        let assign13870_e13282: f64 = if locals.var_q_qsq < assign13870_e13281 { 1.0 } else { 0.0 };
        locals.var_guard576 = assign13870_e13282;

        let (assign13880_e13292, assign13880_e13292_d_n4, assign13880_e13292_d_n6, assign13880_e13292_d_n7, assign13880_e13292_d_n8, assign13880_e13292_d_n9,) = {
    if ((locals.var_guard575 == 0.0) && (locals.var_guard576 != 0.0)) {
        let assign13880_e13289: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign13880_e13290: f64 = (assign13880_e13289).sin();
        (assign13880_e13290, ((assign13880_e13289).cos() * (0.5 * locals.var_q_rac_qsq_dn4)), ((assign13880_e13289).cos() * (0.5 * locals.var_q_rac_qsq_dn6)), ((assign13880_e13289).cos() * (0.5 * locals.var_q_rac_qsq_dn7)), ((assign13880_e13289).cos() * (0.5 * locals.var_q_rac_qsq_dn8)), ((assign13880_e13289).cos() * (0.5 * locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign13880_e13292;
        locals.var_q_temp2_dn4 = assign13880_e13292_d_n4;
        locals.var_q_temp2_dn6 = assign13880_e13292_d_n6;
        locals.var_q_temp2_dn7 = assign13880_e13292_d_n7;
        locals.var_q_temp2_dn8 = assign13880_e13292_d_n8;
        locals.var_q_temp2_dn9 = assign13880_e13292_d_n9;

        let (assign13890_e13304, assign13890_e13304_d_n4, assign13890_e13304_d_n6, assign13890_e13304_d_n7, assign13890_e13304_d_n8, assign13890_e13304_d_n9,) = {
    if ((locals.var_guard575 == 0.0) && (locals.var_guard576 != 0.0)) {
        let assign13890_e13298: f64 = (-locals.var_q_qsq);
        let assign13890_e13301: f64 = (locals.var_q_temp2 * locals.var_q_temp2);
        let assign13890_e13302: f64 = (assign13890_e13298 / assign13890_e13301);
        (assign13890_e13302, ((((-locals.var_q_qsq_dn4) * assign13890_e13301) - (assign13890_e13298 * ((locals.var_q_temp2_dn4 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn4)))) / (assign13890_e13301 * assign13890_e13301)), ((((-locals.var_q_qsq_dn6) * assign13890_e13301) - (assign13890_e13298 * ((locals.var_q_temp2_dn6 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn6)))) / (assign13890_e13301 * assign13890_e13301)), ((((-locals.var_q_qsq_dn7) * assign13890_e13301) - (assign13890_e13298 * ((locals.var_q_temp2_dn7 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn7)))) / (assign13890_e13301 * assign13890_e13301)), ((((-locals.var_q_qsq_dn8) * assign13890_e13301) - (assign13890_e13298 * ((locals.var_q_temp2_dn8 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn8)))) / (assign13890_e13301 * assign13890_e13301)), ((((-locals.var_q_qsq_dn9) * assign13890_e13301) - (assign13890_e13298 * ((locals.var_q_temp2_dn9 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn9)))) / (assign13890_e13301 * assign13890_e13301)),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign13890_e13304;
        locals.var_q_sh_term_dn4 = assign13890_e13304_d_n4;
        locals.var_q_sh_term_dn6 = assign13890_e13304_d_n6;
        locals.var_q_sh_term_dn7 = assign13890_e13304_d_n7;
        locals.var_q_sh_term_dn8 = assign13890_e13304_d_n8;
        locals.var_q_sh_term_dn9 = assign13890_e13304_d_n9;

        let (assign13900_e13312, assign13900_e13312_d_n4, assign13900_e13312_d_n6, assign13900_e13312_d_n7, assign13900_e13312_d_n8, assign13900_e13312_d_n9,) = {
    if ((locals.var_guard575 == 0.0) && (locals.var_guard576 != 0.0)) {
        let assign13900_e13310: f64 = (locals.var_q_sh_term).ln();
        (assign13900_e13310, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign13900_e13312;
        locals.var_q_ln_term_dn4 = assign13900_e13312_d_n4;
        locals.var_q_ln_term_dn6 = assign13900_e13312_d_n6;
        locals.var_q_ln_term_dn7 = assign13900_e13312_d_n7;
        locals.var_q_ln_term_dn8 = assign13900_e13312_d_n8;
        locals.var_q_ln_term_dn9 = assign13900_e13312_d_n9;

        let (assign13910_e13336, assign13910_e13336_d_n4, assign13910_e13336_d_n6, assign13910_e13336_d_n7, assign13910_e13336_d_n8, assign13910_e13336_d_n9,) = {
    if ((locals.var_guard575 == 0.0) && (locals.var_guard576 == 0.0)) {
        let assign13910_e13321: f64 = (locals.var_q_qsq * 0.3333333333333);
        let assign13910_e13325: f64 = (0.05 * locals.var_q_qsq);
        let assign13910_e13329: f64 = (0.0396825396825397 * locals.var_q_qsq);
        let assign13910_e13330: f64 = (1.0 - assign13910_e13329);
        let assign13910_e13331: f64 = (assign13910_e13325 * assign13910_e13330);
        let assign13910_e13332: f64 = (1.0 - assign13910_e13331);
        let assign13910_e13333: f64 = (assign13910_e13321 * assign13910_e13332);
        let assign13910_e13334: f64 = (4.0 - assign13910_e13333);
        (assign13910_e13334, (-(((locals.var_q_qsq_dn4 * 0.3333333333333) * assign13910_e13332) + (assign13910_e13321 * (-(((0.05 * locals.var_q_qsq_dn4) * assign13910_e13330) + (assign13910_e13325 * (-(0.0396825396825397 * locals.var_q_qsq_dn4)))))))), (-(((locals.var_q_qsq_dn6 * 0.3333333333333) * assign13910_e13332) + (assign13910_e13321 * (-(((0.05 * locals.var_q_qsq_dn6) * assign13910_e13330) + (assign13910_e13325 * (-(0.0396825396825397 * locals.var_q_qsq_dn6)))))))), (-(((locals.var_q_qsq_dn7 * 0.3333333333333) * assign13910_e13332) + (assign13910_e13321 * (-(((0.05 * locals.var_q_qsq_dn7) * assign13910_e13330) + (assign13910_e13325 * (-(0.0396825396825397 * locals.var_q_qsq_dn7)))))))), (-(((locals.var_q_qsq_dn8 * 0.3333333333333) * assign13910_e13332) + (assign13910_e13321 * (-(((0.05 * locals.var_q_qsq_dn8) * assign13910_e13330) + (assign13910_e13325 * (-(0.0396825396825397 * locals.var_q_qsq_dn8)))))))), (-(((locals.var_q_qsq_dn9 * 0.3333333333333) * assign13910_e13332) + (assign13910_e13321 * (-(((0.05 * locals.var_q_qsq_dn9) * assign13910_e13330) + (assign13910_e13325 * (-(0.0396825396825397 * locals.var_q_qsq_dn9)))))))),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign13910_e13336;
        locals.var_q_sh_term_dn4 = assign13910_e13336_d_n4;
        locals.var_q_sh_term_dn6 = assign13910_e13336_d_n6;
        locals.var_q_sh_term_dn7 = assign13910_e13336_d_n7;
        locals.var_q_sh_term_dn8 = assign13910_e13336_d_n8;
        locals.var_q_sh_term_dn9 = assign13910_e13336_d_n9;

        let (assign13920_e13345, assign13920_e13345_d_n4, assign13920_e13345_d_n6, assign13920_e13345_d_n7, assign13920_e13345_d_n8, assign13920_e13345_d_n9,) = {
    if ((locals.var_guard575 == 0.0) && (locals.var_guard576 == 0.0)) {
        let assign13920_e13343: f64 = (locals.var_q_sh_term).ln();
        (assign13920_e13343, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign13920_e13345;
        locals.var_q_ln_term_dn4 = assign13920_e13345_d_n4;
        locals.var_q_ln_term_dn6 = assign13920_e13345_d_n6;
        locals.var_q_ln_term_dn7 = assign13920_e13345_d_n7;
        locals.var_q_ln_term_dn8 = assign13920_e13345_d_n8;
        locals.var_q_ln_term_dn9 = assign13920_e13345_d_n9;

        let assign13930_e13348: f64 = (1.01 * locals.var_q_k1q1);
        let assign13930_e13350: f64 = (assign13930_e13348 + locals.var_q_qcoth);
        let assign13930_e13352: f64 = if assign13930_e13350 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard577 = assign13930_e13352;

        let (assign13940_e13358, assign13940_e13358_d_n4, assign13940_e13358_d_n6, assign13940_e13358_d_n7, assign13940_e13358_d_n8, assign13940_e13358_d_n9,) = {
    if (locals.var_guard577 != 0.0) {
        let assign13940_e13356: f64 = (locals.var_q_k1q1 + locals.var_q_qcoth);
        (assign13940_e13356, (locals.var_q_k1q1_dn4 + locals.var_q_qcoth_dn4), (locals.var_q_k1q1_dn6 + locals.var_q_qcoth_dn6), (locals.var_q_k1q1_dn7 + locals.var_q_qcoth_dn7), (locals.var_q_k1q1_dn8 + locals.var_q_qcoth_dn8), (locals.var_q_k1q1_dn9 + locals.var_q_qcoth_dn9),)
    } else {
        (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9,)
    }
};
        locals.var_q_expnum = assign13940_e13358;
        locals.var_q_expnum_dn4 = assign13940_e13358_d_n4;
        locals.var_q_expnum_dn6 = assign13940_e13358_d_n6;
        locals.var_q_expnum_dn7 = assign13940_e13358_d_n7;
        locals.var_q_expnum_dn8 = assign13940_e13358_d_n8;
        locals.var_q_expnum_dn9 = assign13940_e13358_d_n9;

        let (assign13950_e13364, assign13950_e13364_d_n4, assign13950_e13364_d_n6, assign13950_e13364_d_n7, assign13950_e13364_d_n8, assign13950_e13364_d_n9,) = {
    if (locals.var_guard577 != 0.0) {
        let assign13950_e13362: f64 = (locals.var_k1 + locals.var_q_d1_qcoth);
        (assign13950_e13362, (locals.var_k1_dn4 + locals.var_q_d1_qcoth_dn4), (locals.var_k1_dn6 + locals.var_q_d1_qcoth_dn6), (locals.var_k1_dn7 + locals.var_q_d1_qcoth_dn7), (locals.var_k1_dn8 + locals.var_q_d1_qcoth_dn8), (locals.var_k1_dn9 + locals.var_q_d1_qcoth_dn9),)
    } else {
        (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9,)
    }
};
        locals.var_q_d1_expnum = assign13950_e13364;
        locals.var_q_d1_expnum_dn4 = assign13950_e13364_d_n4;
        locals.var_q_d1_expnum_dn6 = assign13950_e13364_d_n6;
        locals.var_q_d1_expnum_dn7 = assign13950_e13364_d_n7;
        locals.var_q_d1_expnum_dn8 = assign13950_e13364_d_n8;
        locals.var_q_d1_expnum_dn9 = assign13950_e13364_d_n9;

        let (assign13960_e13368, assign13960_e13368_d_n4, assign13960_e13368_d_n6, assign13960_e13368_d_n7, assign13960_e13368_d_n8, assign13960_e13368_d_n9,) = {
    if (locals.var_guard577 != 0.0) {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    } else {
        (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9,)
    }
};
        locals.var_q_d2_expnum = assign13960_e13368;
        locals.var_q_d2_expnum_dn4 = assign13960_e13368_d_n4;
        locals.var_q_d2_expnum_dn6 = assign13960_e13368_d_n6;
        locals.var_q_d2_expnum_dn7 = assign13960_e13368_d_n7;
        locals.var_q_d2_expnum_dn8 = assign13960_e13368_d_n8;
        locals.var_q_d2_expnum_dn9 = assign13960_e13368_d_n9;

        let (assign13970_e13377, assign13970_e13377_d_n4, assign13970_e13377_d_n6, assign13970_e13377_d_n7, assign13970_e13377_d_n8, assign13970_e13377_d_n9,) = {
    if (locals.var_guard577 == 0.0) {
        let assign13970_e13374: f64 = (locals.var_q_k1q1 - locals.var_q_qcoth);
        let assign13970_e13375: f64 = (1.0 / assign13970_e13374);
        (assign13970_e13375, (-((locals.var_q_k1q1_dn4 - locals.var_q_qcoth_dn4) / (assign13970_e13374 * assign13970_e13374))), (-((locals.var_q_k1q1_dn6 - locals.var_q_qcoth_dn6) / (assign13970_e13374 * assign13970_e13374))), (-((locals.var_q_k1q1_dn7 - locals.var_q_qcoth_dn7) / (assign13970_e13374 * assign13970_e13374))), (-((locals.var_q_k1q1_dn8 - locals.var_q_qcoth_dn8) / (assign13970_e13374 * assign13970_e13374))), (-((locals.var_q_k1q1_dn9 - locals.var_q_qcoth_dn9) / (assign13970_e13374 * assign13970_e13374))),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign13970_e13377;
        locals.var_q_temp2_dn4 = assign13970_e13377_d_n4;
        locals.var_q_temp2_dn6 = assign13970_e13377_d_n6;
        locals.var_q_temp2_dn7 = assign13970_e13377_d_n7;
        locals.var_q_temp2_dn8 = assign13970_e13377_d_n8;
        locals.var_q_temp2_dn9 = assign13970_e13377_d_n9;

        let (assign13980_e13384, assign13980_e13384_d_n4, assign13980_e13384_d_n6, assign13980_e13384_d_n7, assign13980_e13384_d_n8, assign13980_e13384_d_n9,) = {
    if (locals.var_guard577 == 0.0) {
        let assign13980_e13382: f64 = (locals.var_q_d1_qcoth - locals.var_k1);
        (assign13980_e13382, (locals.var_q_d1_qcoth_dn4 - locals.var_k1_dn4), (locals.var_q_d1_qcoth_dn6 - locals.var_k1_dn6), (locals.var_q_d1_qcoth_dn7 - locals.var_k1_dn7), (locals.var_q_d1_qcoth_dn8 - locals.var_k1_dn8), (locals.var_q_d1_qcoth_dn9 - locals.var_k1_dn9),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign13980_e13384;
        locals.var_q_temp3_dn4 = assign13980_e13384_d_n4;
        locals.var_q_temp3_dn6 = assign13980_e13384_d_n6;
        locals.var_q_temp3_dn7 = assign13980_e13384_d_n7;
        locals.var_q_temp3_dn8 = assign13980_e13384_d_n8;
        locals.var_q_temp3_dn9 = assign13980_e13384_d_n9;

        let (assign13990_e13393, assign13990_e13393_d_n4, assign13990_e13393_d_n6, assign13990_e13393_d_n7, assign13990_e13393_d_n8, assign13990_e13393_d_n9,) = {
    if (locals.var_guard577 == 0.0) {
        let assign13990_e13389: f64 = (locals.var_q_aexp - locals.var_q_sh_term);
        let assign13990_e13391: f64 = (assign13990_e13389 * locals.var_q_temp2);
        (assign13990_e13391, (((locals.var_q_aexp_dn4 - locals.var_q_sh_term_dn4) * locals.var_q_temp2) + (assign13990_e13389 * locals.var_q_temp2_dn4)), (((locals.var_q_aexp_dn6 - locals.var_q_sh_term_dn6) * locals.var_q_temp2) + (assign13990_e13389 * locals.var_q_temp2_dn6)), (((locals.var_q_aexp_dn7 - locals.var_q_sh_term_dn7) * locals.var_q_temp2) + (assign13990_e13389 * locals.var_q_temp2_dn7)), (((locals.var_q_aexp_dn8 - locals.var_q_sh_term_dn8) * locals.var_q_temp2) + (assign13990_e13389 * locals.var_q_temp2_dn8)), (((locals.var_q_aexp_dn9 - locals.var_q_sh_term_dn9) * locals.var_q_temp2) + (assign13990_e13389 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9,)
    }
};
        locals.var_q_expnum = assign13990_e13393;
        locals.var_q_expnum_dn4 = assign13990_e13393_d_n4;
        locals.var_q_expnum_dn6 = assign13990_e13393_d_n6;
        locals.var_q_expnum_dn7 = assign13990_e13393_d_n7;
        locals.var_q_expnum_dn8 = assign13990_e13393_d_n8;
        locals.var_q_expnum_dn9 = assign13990_e13393_d_n9;

    }

    pub(super) fn stamp_transient_block_34(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14000_e13408, assign14000_e13408_d_n4, assign14000_e13408_d_n6, assign14000_e13408_d_n7, assign14000_e13408_d_n8, assign14000_e13408_d_n9,) = {
    if (locals.var_guard577 == 0.0) {
        let assign14000_e13398: f64 = (locals.var_q_temp3 * locals.var_q_expnum);
        let assign14000_e13400: f64 = (assign14000_e13398 - locals.var_q_aexp);
        let assign14000_e13403: f64 = (locals.var_q_d1_ln * locals.var_q_sh_term);
        let assign14000_e13404: f64 = (assign14000_e13400 - assign14000_e13403);
        let assign14000_e13406: f64 = (assign14000_e13404 * locals.var_q_temp2);
        (assign14000_e13406, ((((((locals.var_q_temp3_dn4 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4) - ((locals.var_q_d1_ln_dn4 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign14000_e13404 * locals.var_q_temp2_dn4)), ((((((locals.var_q_temp3_dn6 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6) - ((locals.var_q_d1_ln_dn6 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign14000_e13404 * locals.var_q_temp2_dn6)), ((((((locals.var_q_temp3_dn7 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7) - ((locals.var_q_d1_ln_dn7 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign14000_e13404 * locals.var_q_temp2_dn7)), ((((((locals.var_q_temp3_dn8 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8) - ((locals.var_q_d1_ln_dn8 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign14000_e13404 * locals.var_q_temp2_dn8)), ((((((locals.var_q_temp3_dn9 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9) - ((locals.var_q_d1_ln_dn9 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign14000_e13404 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9,)
    }
};
        locals.var_q_d1_expnum = assign14000_e13408;
        locals.var_q_d1_expnum_dn4 = assign14000_e13408_d_n4;
        locals.var_q_d1_expnum_dn6 = assign14000_e13408_d_n6;
        locals.var_q_d1_expnum_dn7 = assign14000_e13408_d_n7;
        locals.var_q_d1_expnum_dn8 = assign14000_e13408_d_n8;
        locals.var_q_d1_expnum_dn9 = assign14000_e13408_d_n9;

        let (assign14010_e13433, assign14010_e13433_d_n4, assign14010_e13433_d_n6, assign14010_e13433_d_n7, assign14010_e13433_d_n8, assign14010_e13433_d_n9,) = {
    if (locals.var_guard577 == 0.0) {
        let assign14010_e13413: f64 = (locals.var_q_d2_qcoth * locals.var_q_expnum);
        let assign14010_e13416: f64 = (2.0 * locals.var_q_temp3);
        let assign14010_e13418: f64 = (assign14010_e13416 * locals.var_q_d1_expnum);
        let assign14010_e13419: f64 = (assign14010_e13413 + assign14010_e13418);
        let assign14010_e13421: f64 = (assign14010_e13419 + locals.var_q_aexp);
        let assign14010_e13425: f64 = (locals.var_q_d1_ln * locals.var_q_d1_ln);
        let assign14010_e13426: f64 = (locals.var_q_d2_ln + assign14010_e13425);
        let assign14010_e13428: f64 = (assign14010_e13426 * locals.var_q_sh_term);
        let assign14010_e13429: f64 = (assign14010_e13421 - assign14010_e13428);
        let assign14010_e13431: f64 = (assign14010_e13429 * locals.var_q_temp2);
        (assign14010_e13431, (((((((locals.var_q_d2_qcoth_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_temp3_dn4) * locals.var_q_d1_expnum) + (assign14010_e13416 * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4) - (((locals.var_q_d2_ln_dn4 + ((locals.var_q_d1_ln_dn4 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn4))) * locals.var_q_sh_term) + (assign14010_e13426 * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign14010_e13429 * locals.var_q_temp2_dn4)), (((((((locals.var_q_d2_qcoth_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_temp3_dn6) * locals.var_q_d1_expnum) + (assign14010_e13416 * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6) - (((locals.var_q_d2_ln_dn6 + ((locals.var_q_d1_ln_dn6 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn6))) * locals.var_q_sh_term) + (assign14010_e13426 * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign14010_e13429 * locals.var_q_temp2_dn6)), (((((((locals.var_q_d2_qcoth_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_temp3_dn7) * locals.var_q_d1_expnum) + (assign14010_e13416 * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7) - (((locals.var_q_d2_ln_dn7 + ((locals.var_q_d1_ln_dn7 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn7))) * locals.var_q_sh_term) + (assign14010_e13426 * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign14010_e13429 * locals.var_q_temp2_dn7)), (((((((locals.var_q_d2_qcoth_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_temp3_dn8) * locals.var_q_d1_expnum) + (assign14010_e13416 * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8) - (((locals.var_q_d2_ln_dn8 + ((locals.var_q_d1_ln_dn8 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn8))) * locals.var_q_sh_term) + (assign14010_e13426 * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign14010_e13429 * locals.var_q_temp2_dn8)), (((((((locals.var_q_d2_qcoth_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_temp3_dn9) * locals.var_q_d1_expnum) + (assign14010_e13416 * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9) - (((locals.var_q_d2_ln_dn9 + ((locals.var_q_d1_ln_dn9 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn9))) * locals.var_q_sh_term) + (assign14010_e13426 * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign14010_e13429 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9,)
    }
};
        locals.var_q_d2_expnum = assign14010_e13433;
        locals.var_q_d2_expnum_dn4 = assign14010_e13433_d_n4;
        locals.var_q_d2_expnum_dn6 = assign14010_e13433_d_n6;
        locals.var_q_d2_expnum_dn7 = assign14010_e13433_d_n7;
        locals.var_q_d2_expnum_dn8 = assign14010_e13433_d_n8;
        locals.var_q_d2_expnum_dn9 = assign14010_e13433_d_n9;

        let assign14020_e13436: f64 = if locals.var_q_expnum > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard578 = assign14020_e13436;

        let (assign14030_e13441, assign14030_e13441_d_n4, assign14030_e13441_d_n6, assign14030_e13441_d_n7, assign14030_e13441_d_n8, assign14030_e13441_d_n9,) = {
    if (locals.var_guard578 != 0.0) {
        let assign14030_e13439: f64 = (locals.var_q_expnum).ln();
        (assign14030_e13439, (locals.var_q_expnum_dn4 / locals.var_q_expnum), (locals.var_q_expnum_dn6 / locals.var_q_expnum), (locals.var_q_expnum_dn7 / locals.var_q_expnum), (locals.var_q_expnum_dn8 / locals.var_q_expnum), (locals.var_q_expnum_dn9 / locals.var_q_expnum),)
    } else {
        (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9,)
    }
};
        locals.var_q_lnexpnum = assign14030_e13441;
        locals.var_q_lnexpnum_dn4 = assign14030_e13441_d_n4;
        locals.var_q_lnexpnum_dn6 = assign14030_e13441_d_n6;
        locals.var_q_lnexpnum_dn7 = assign14030_e13441_d_n7;
        locals.var_q_lnexpnum_dn8 = assign14030_e13441_d_n8;
        locals.var_q_lnexpnum_dn9 = assign14030_e13441_d_n9;

        let (assign14040_e13447, assign14040_e13447_d_n4, assign14040_e13447_d_n6, assign14040_e13447_d_n7, assign14040_e13447_d_n8, assign14040_e13447_d_n9,) = {
    if (locals.var_guard578 != 0.0) {
        let assign14040_e13445: f64 = (1.0 / locals.var_q_expnum);
        (assign14040_e13445, (-(locals.var_q_expnum_dn4 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn6 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn7 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn8 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn9 / (locals.var_q_expnum * locals.var_q_expnum))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign14040_e13447;
        locals.var_q_temp1_dn4 = assign14040_e13447_d_n4;
        locals.var_q_temp1_dn6 = assign14040_e13447_d_n6;
        locals.var_q_temp1_dn7 = assign14040_e13447_d_n7;
        locals.var_q_temp1_dn8 = assign14040_e13447_d_n8;
        locals.var_q_temp1_dn9 = assign14040_e13447_d_n9;

        let (assign14050_e13453, assign14050_e13453_d_n4, assign14050_e13453_d_n6, assign14050_e13453_d_n7, assign14050_e13453_d_n8, assign14050_e13453_d_n9,) = {
    if (locals.var_guard578 != 0.0) {
        let assign14050_e13451: f64 = (locals.var_q_d1_expnum * locals.var_q_temp1);
        (assign14050_e13451, ((locals.var_q_d1_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn4)), ((locals.var_q_d1_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn6)), ((locals.var_q_d1_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn7)), ((locals.var_q_d1_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn8)), ((locals.var_q_d1_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9,)
    }
};
        locals.var_q_d1_lnexpnum = assign14050_e13453;
        locals.var_q_d1_lnexpnum_dn4 = assign14050_e13453_d_n4;
        locals.var_q_d1_lnexpnum_dn6 = assign14050_e13453_d_n6;
        locals.var_q_d1_lnexpnum_dn7 = assign14050_e13453_d_n7;
        locals.var_q_d1_lnexpnum_dn8 = assign14050_e13453_d_n8;
        locals.var_q_d1_lnexpnum_dn9 = assign14050_e13453_d_n9;

        let (assign14060_e13463, assign14060_e13463_d_n4, assign14060_e13463_d_n6, assign14060_e13463_d_n7, assign14060_e13463_d_n8, assign14060_e13463_d_n9,) = {
    if (locals.var_guard578 != 0.0) {
        let assign14060_e13457: f64 = (locals.var_q_d2_expnum * locals.var_q_temp1);
        let assign14060_e13460: f64 = (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum);
        let assign14060_e13461: f64 = (assign14060_e13457 - assign14060_e13460);
        (assign14060_e13461, (((locals.var_q_d2_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn4)) - ((locals.var_q_d1_lnexpnum_dn4 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn4))), (((locals.var_q_d2_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn6)) - ((locals.var_q_d1_lnexpnum_dn6 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn6))), (((locals.var_q_d2_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn7)) - ((locals.var_q_d1_lnexpnum_dn7 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn7))), (((locals.var_q_d2_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn8)) - ((locals.var_q_d1_lnexpnum_dn8 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn8))), (((locals.var_q_d2_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn9)) - ((locals.var_q_d1_lnexpnum_dn9 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn9))),)
    } else {
        (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9,)
    }
};
        locals.var_q_d2_lnexpnum = assign14060_e13463;
        locals.var_q_d2_lnexpnum_dn4 = assign14060_e13463_d_n4;
        locals.var_q_d2_lnexpnum_dn6 = assign14060_e13463_d_n6;
        locals.var_q_d2_lnexpnum_dn7 = assign14060_e13463_d_n7;
        locals.var_q_d2_lnexpnum_dn8 = assign14060_e13463_d_n8;
        locals.var_q_d2_lnexpnum_dn9 = assign14060_e13463_d_n9;

        let (assign14070_e13474, assign14070_e13474_d_n4, assign14070_e13474_d_n6, assign14070_e13474_d_n7, assign14070_e13474_d_n8, assign14070_e13474_d_n9,) = {
    if (locals.var_guard578 == 0.0) {
        let assign14070_e13468: f64 = (locals.var_q_k1q1 + 0.6931471805599);
        let assign14070_e13470: f64 = (-locals.var_q_k1q1);
        let assign14070_e13471: f64 = (assign14070_e13470).ln();
        let assign14070_e13472: f64 = (assign14070_e13468 + assign14070_e13471);
        (assign14070_e13472, (locals.var_q_k1q1_dn4 + ((-locals.var_q_k1q1_dn4) / assign14070_e13470)), (locals.var_q_k1q1_dn6 + ((-locals.var_q_k1q1_dn6) / assign14070_e13470)), (locals.var_q_k1q1_dn7 + ((-locals.var_q_k1q1_dn7) / assign14070_e13470)), (locals.var_q_k1q1_dn8 + ((-locals.var_q_k1q1_dn8) / assign14070_e13470)), (locals.var_q_k1q1_dn9 + ((-locals.var_q_k1q1_dn9) / assign14070_e13470)),)
    } else {
        (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9,)
    }
};
        locals.var_q_lnexpnum = assign14070_e13474;
        locals.var_q_lnexpnum_dn4 = assign14070_e13474_d_n4;
        locals.var_q_lnexpnum_dn6 = assign14070_e13474_d_n6;
        locals.var_q_lnexpnum_dn7 = assign14070_e13474_d_n7;
        locals.var_q_lnexpnum_dn8 = assign14070_e13474_d_n8;
        locals.var_q_lnexpnum_dn9 = assign14070_e13474_d_n9;

        let (assign14080_e13481, assign14080_e13481_d_n4, assign14080_e13481_d_n6, assign14080_e13481_d_n7, assign14080_e13481_d_n8, assign14080_e13481_d_n9,) = {
    if (locals.var_guard578 == 0.0) {
        let assign14080_e13479: f64 = (1.0 / locals.var_q1s);
        (assign14080_e13479, (-(locals.var_q1s_dn4 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn6 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn7 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn8 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn9 / (locals.var_q1s * locals.var_q1s))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign14080_e13481;
        locals.var_q_temp1_dn4 = assign14080_e13481_d_n4;
        locals.var_q_temp1_dn6 = assign14080_e13481_d_n6;
        locals.var_q_temp1_dn7 = assign14080_e13481_d_n7;
        locals.var_q_temp1_dn8 = assign14080_e13481_d_n8;
        locals.var_q_temp1_dn9 = assign14080_e13481_d_n9;

        let (assign14090_e13488, assign14090_e13488_d_n4, assign14090_e13488_d_n6, assign14090_e13488_d_n7, assign14090_e13488_d_n8, assign14090_e13488_d_n9,) = {
    if (locals.var_guard578 == 0.0) {
        let assign14090_e13486: f64 = (locals.var_k1 + locals.var_q_temp1);
        (assign14090_e13486, (locals.var_k1_dn4 + locals.var_q_temp1_dn4), (locals.var_k1_dn6 + locals.var_q_temp1_dn6), (locals.var_k1_dn7 + locals.var_q_temp1_dn7), (locals.var_k1_dn8 + locals.var_q_temp1_dn8), (locals.var_k1_dn9 + locals.var_q_temp1_dn9),)
    } else {
        (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9,)
    }
};
        locals.var_q_d1_lnexpnum = assign14090_e13488;
        locals.var_q_d1_lnexpnum_dn4 = assign14090_e13488_d_n4;
        locals.var_q_d1_lnexpnum_dn6 = assign14090_e13488_d_n6;
        locals.var_q_d1_lnexpnum_dn7 = assign14090_e13488_d_n7;
        locals.var_q_d1_lnexpnum_dn8 = assign14090_e13488_d_n8;
        locals.var_q_d1_lnexpnum_dn9 = assign14090_e13488_d_n9;

        let (assign14100_e13496, assign14100_e13496_d_n4, assign14100_e13496_d_n6, assign14100_e13496_d_n7, assign14100_e13496_d_n8, assign14100_e13496_d_n9,) = {
    if (locals.var_guard578 == 0.0) {
        let assign14100_e13492: f64 = (-locals.var_q_temp1);
        let assign14100_e13494: f64 = (assign14100_e13492 * locals.var_q_temp1);
        (assign14100_e13494, (((-locals.var_q_temp1_dn4) * locals.var_q_temp1) + (assign14100_e13492 * locals.var_q_temp1_dn4)), (((-locals.var_q_temp1_dn6) * locals.var_q_temp1) + (assign14100_e13492 * locals.var_q_temp1_dn6)), (((-locals.var_q_temp1_dn7) * locals.var_q_temp1) + (assign14100_e13492 * locals.var_q_temp1_dn7)), (((-locals.var_q_temp1_dn8) * locals.var_q_temp1) + (assign14100_e13492 * locals.var_q_temp1_dn8)), (((-locals.var_q_temp1_dn9) * locals.var_q_temp1) + (assign14100_e13492 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9,)
    }
};
        locals.var_q_d2_lnexpnum = assign14100_e13496;
        locals.var_q_d2_lnexpnum_dn4 = assign14100_e13496_d_n4;
        locals.var_q_d2_lnexpnum_dn6 = assign14100_e13496_d_n6;
        locals.var_q_d2_lnexpnum_dn7 = assign14100_e13496_d_n7;
        locals.var_q_d2_lnexpnum_dn8 = assign14100_e13496_d_n8;
        locals.var_q_d2_lnexpnum_dn9 = assign14100_e13496_d_n9;

        let assign14110_e13499: f64 = (locals.var_xg2x - locals.var_xg1x);
        let assign14110_e13501: f64 = (assign14110_e13499 + locals.var_q1s);
        let assign14110_e13504: f64 = (2.0 * locals.var_q_lnexpnum);
        let assign14110_e13505: f64 = (assign14110_e13501 + assign14110_e13504);
        let assign14110_e13507: f64 = (assign14110_e13505 - locals.var_q_ln_term);
        locals.var_q_q2_int = assign14110_e13507;
        locals.var_q_q2_int_dn4 = ((((locals.var_xg2x_dn4 - locals.var_xg1x_dn4) + locals.var_q1s_dn4) + (2.0 * locals.var_q_lnexpnum_dn4)) - locals.var_q_ln_term_dn4);
        locals.var_q_q2_int_dn6 = ((((locals.var_xg2x_dn6 - locals.var_xg1x_dn6) + locals.var_q1s_dn6) + (2.0 * locals.var_q_lnexpnum_dn6)) - locals.var_q_ln_term_dn6);
        locals.var_q_q2_int_dn7 = ((((locals.var_xg2x_dn7 - locals.var_xg1x_dn7) + locals.var_q1s_dn7) + (2.0 * locals.var_q_lnexpnum_dn7)) - locals.var_q_ln_term_dn7);
        locals.var_q_q2_int_dn8 = ((((locals.var_xg2x_dn8 - locals.var_xg1x_dn8) + locals.var_q1s_dn8) + (2.0 * locals.var_q_lnexpnum_dn8)) - locals.var_q_ln_term_dn8);
        locals.var_q_q2_int_dn9 = ((((locals.var_xg2x_dn9 - locals.var_xg1x_dn9) + locals.var_q1s_dn9) + (2.0 * locals.var_q_lnexpnum_dn9)) - locals.var_q_ln_term_dn9);

        let assign14120_e13511: f64 = (2.0 * locals.var_q_d1_lnexpnum);
        let assign14120_e13512: f64 = (1.0 + assign14120_e13511);
        let assign14120_e13514: f64 = (assign14120_e13512 - locals.var_q_d1_ln);
        locals.var_q_d1_q2 = assign14120_e13514;
        locals.var_q_d1_q2_dn4 = ((2.0 * locals.var_q_d1_lnexpnum_dn4) - locals.var_q_d1_ln_dn4);
        locals.var_q_d1_q2_dn6 = ((2.0 * locals.var_q_d1_lnexpnum_dn6) - locals.var_q_d1_ln_dn6);
        locals.var_q_d1_q2_dn7 = ((2.0 * locals.var_q_d1_lnexpnum_dn7) - locals.var_q_d1_ln_dn7);
        locals.var_q_d1_q2_dn8 = ((2.0 * locals.var_q_d1_lnexpnum_dn8) - locals.var_q_d1_ln_dn8);
        locals.var_q_d1_q2_dn9 = ((2.0 * locals.var_q_d1_lnexpnum_dn9) - locals.var_q_d1_ln_dn9);

        let assign14130_e13517: f64 = (2.0 * locals.var_q_d2_lnexpnum);
        let assign14130_e13519: f64 = (assign14130_e13517 - locals.var_q_d2_ln);
        locals.var_q_d2_q2 = assign14130_e13519;
        locals.var_q_d2_q2_dn4 = ((2.0 * locals.var_q_d2_lnexpnum_dn4) - locals.var_q_d2_ln_dn4);
        locals.var_q_d2_q2_dn6 = ((2.0 * locals.var_q_d2_lnexpnum_dn6) - locals.var_q_d2_ln_dn6);
        locals.var_q_d2_q2_dn7 = ((2.0 * locals.var_q_d2_lnexpnum_dn7) - locals.var_q_d2_ln_dn7);
        locals.var_q_d2_q2_dn8 = ((2.0 * locals.var_q_d2_lnexpnum_dn8) - locals.var_q_d2_ln_dn8);
        locals.var_q_d2_q2_dn9 = ((2.0 * locals.var_q_d2_lnexpnum_dn9) - locals.var_q_d2_ln_dn9);

        let assign14140_e13523: f64 = (locals.var_k2 * locals.var_q_q2_int);
        let assign14140_e13524: f64 = (locals.var_q_k1q1 + assign14140_e13523);
        locals.var_q_qi_int = assign14140_e13524;
        locals.var_q_qi_int_dn4 = (locals.var_q_k1q1_dn4 + ((locals.var_k2_dn4 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn4)));
        locals.var_q_qi_int_dn6 = (locals.var_q_k1q1_dn6 + ((locals.var_k2_dn6 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn6)));
        locals.var_q_qi_int_dn7 = (locals.var_q_k1q1_dn7 + ((locals.var_k2_dn7 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn7)));
        locals.var_q_qi_int_dn8 = (locals.var_q_k1q1_dn8 + ((locals.var_k2_dn8 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn8)));
        locals.var_q_qi_int_dn9 = (locals.var_q_k1q1_dn9 + ((locals.var_k2_dn9 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn9)));

        let assign14150_e13528: f64 = (locals.var_k2 * locals.var_q_d1_q2);
        let assign14150_e13529: f64 = (locals.var_k1 + assign14150_e13528);
        locals.var_q_d1_qi = assign14150_e13529;
        locals.var_q_d1_qi_dn4 = (locals.var_k1_dn4 + ((locals.var_k2_dn4 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn4)));
        locals.var_q_d1_qi_dn6 = (locals.var_k1_dn6 + ((locals.var_k2_dn6 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn6)));
        locals.var_q_d1_qi_dn7 = (locals.var_k1_dn7 + ((locals.var_k2_dn7 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn7)));
        locals.var_q_d1_qi_dn8 = (locals.var_k1_dn8 + ((locals.var_k2_dn8 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn8)));
        locals.var_q_d1_qi_dn9 = (locals.var_k1_dn9 + ((locals.var_k2_dn9 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn9)));

        let assign14160_e13532: f64 = (locals.var_k2 * locals.var_q_d2_q2);
        locals.var_q_d2_qi = assign14160_e13532;
        locals.var_q_d2_qi_dn4 = ((locals.var_k2_dn4 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn4));
        locals.var_q_d2_qi_dn6 = ((locals.var_k2_dn6 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn6));
        locals.var_q_d2_qi_dn7 = ((locals.var_k2_dn7 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn7));
        locals.var_q_d2_qi_dn8 = ((locals.var_k2_dn8 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn8));
        locals.var_q_d2_qi_dn9 = ((locals.var_k2_dn9 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn9));

        let assign14170_e13535: f64 = (locals.var_q_qi_int * locals.var_q_expnum);
        let assign14170_e13537: f64 = (assign14170_e13535 - locals.var_q_aexp);
        locals.var_q_zero = assign14170_e13537;
        locals.var_q_zero_dn4 = (((locals.var_q_qi_int_dn4 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4);
        locals.var_q_zero_dn6 = (((locals.var_q_qi_int_dn6 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6);
        locals.var_q_zero_dn7 = (((locals.var_q_qi_int_dn7 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7);
        locals.var_q_zero_dn8 = (((locals.var_q_qi_int_dn8 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8);
        locals.var_q_zero_dn9 = (((locals.var_q_qi_int_dn9 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9);

        let assign14180_e13540: f64 = (locals.var_q_d1_qi * locals.var_q_expnum);
        let assign14180_e13543: f64 = (locals.var_q_qi_int * locals.var_q_d1_expnum);
        let assign14180_e13544: f64 = (assign14180_e13540 + assign14180_e13543);
        let assign14180_e13546: f64 = (assign14180_e13544 + locals.var_q_aexp);
        locals.var_q_d1_zero = assign14180_e13546;
        locals.var_q_d1_zero_dn4 = ((((locals.var_q_d1_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn4)) + ((locals.var_q_qi_int_dn4 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4);
        locals.var_q_d1_zero_dn6 = ((((locals.var_q_d1_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn6)) + ((locals.var_q_qi_int_dn6 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6);
        locals.var_q_d1_zero_dn7 = ((((locals.var_q_d1_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn7)) + ((locals.var_q_qi_int_dn7 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7);
        locals.var_q_d1_zero_dn8 = ((((locals.var_q_d1_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn8)) + ((locals.var_q_qi_int_dn8 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8);
        locals.var_q_d1_zero_dn9 = ((((locals.var_q_d1_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn9)) + ((locals.var_q_qi_int_dn9 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9);

        let assign14190_e13549: f64 = (locals.var_q_d2_qi * locals.var_q_expnum);
        let assign14190_e13552: f64 = (2.0 * locals.var_q_d1_qi);
        let assign14190_e13554: f64 = (assign14190_e13552 * locals.var_q_d1_expnum);
        let assign14190_e13555: f64 = (assign14190_e13549 + assign14190_e13554);
        let assign14190_e13558: f64 = (locals.var_q_qi_int * locals.var_q_d2_expnum);
        let assign14190_e13559: f64 = (assign14190_e13555 + assign14190_e13558);
        let assign14190_e13561: f64 = (assign14190_e13559 - locals.var_q_aexp);
        locals.var_q_d2_zero = assign14190_e13561;
        locals.var_q_d2_zero_dn4 = (((((locals.var_q_d2_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_d1_qi_dn4) * locals.var_q_d1_expnum) + (assign14190_e13552 * locals.var_q_d1_expnum_dn4))) + ((locals.var_q_qi_int_dn4 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn4))) - locals.var_q_aexp_dn4);
        locals.var_q_d2_zero_dn6 = (((((locals.var_q_d2_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_d1_qi_dn6) * locals.var_q_d1_expnum) + (assign14190_e13552 * locals.var_q_d1_expnum_dn6))) + ((locals.var_q_qi_int_dn6 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn6))) - locals.var_q_aexp_dn6);
        locals.var_q_d2_zero_dn7 = (((((locals.var_q_d2_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_d1_qi_dn7) * locals.var_q_d1_expnum) + (assign14190_e13552 * locals.var_q_d1_expnum_dn7))) + ((locals.var_q_qi_int_dn7 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn7))) - locals.var_q_aexp_dn7);
        locals.var_q_d2_zero_dn8 = (((((locals.var_q_d2_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_d1_qi_dn8) * locals.var_q_d1_expnum) + (assign14190_e13552 * locals.var_q_d1_expnum_dn8))) + ((locals.var_q_qi_int_dn8 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn8))) - locals.var_q_aexp_dn8);
        locals.var_q_d2_zero_dn9 = (((((locals.var_q_d2_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_d1_qi_dn9) * locals.var_q_d1_expnum) + (assign14190_e13552 * locals.var_q_d1_expnum_dn9))) + ((locals.var_q_qi_int_dn9 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn9))) - locals.var_q_aexp_dn9);

        let assign14200_e13564: f64 = (locals.var_q_d1_zero * locals.var_q_d1_zero);
        let assign14200_e13567: f64 = (0.5 * locals.var_q_zero);
        let assign14200_e13569: f64 = (assign14200_e13567 * locals.var_q_d2_zero);
        let assign14200_e13570: f64 = (assign14200_e13564 - assign14200_e13569);
        locals.var_q_temp = assign14200_e13570;
        locals.var_q_temp_dn4 = (((locals.var_q_d1_zero_dn4 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn4)) - (((0.5 * locals.var_q_zero_dn4) * locals.var_q_d2_zero) + (assign14200_e13567 * locals.var_q_d2_zero_dn4)));
        locals.var_q_temp_dn6 = (((locals.var_q_d1_zero_dn6 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn6)) - (((0.5 * locals.var_q_zero_dn6) * locals.var_q_d2_zero) + (assign14200_e13567 * locals.var_q_d2_zero_dn6)));
        locals.var_q_temp_dn7 = (((locals.var_q_d1_zero_dn7 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn7)) - (((0.5 * locals.var_q_zero_dn7) * locals.var_q_d2_zero) + (assign14200_e13567 * locals.var_q_d2_zero_dn7)));
        locals.var_q_temp_dn8 = (((locals.var_q_d1_zero_dn8 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn8)) - (((0.5 * locals.var_q_zero_dn8) * locals.var_q_d2_zero) + (assign14200_e13567 * locals.var_q_d2_zero_dn8)));
        locals.var_q_temp_dn9 = (((locals.var_q_d1_zero_dn9 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn9)) - (((0.5 * locals.var_q_zero_dn9) * locals.var_q_d2_zero) + (assign14200_e13567 * locals.var_q_d2_zero_dn9)));

        let assign14210_e13572: f64 = (-locals.var_q_zero);
        let assign14210_e13574: f64 = (assign14210_e13572 * locals.var_q_d1_zero);
        let assign14210_e13576: f64 = (assign14210_e13574 * locals.var_q_temp);
        let assign14210_e13579: f64 = (locals.var_q_temp * locals.var_q_temp);
        let assign14210_e13581: f64 = (assign14210_e13579 + 1e-200);
        let assign14210_e13582: f64 = (assign14210_e13576 / assign14210_e13581);
        locals.var_q_eps2 = assign14210_e13582;
        locals.var_q_eps2_dn4 = ((((((((-locals.var_q_zero_dn4) * locals.var_q_d1_zero) + (assign14210_e13572 * locals.var_q_d1_zero_dn4)) * locals.var_q_temp) + (assign14210_e13574 * locals.var_q_temp_dn4)) * assign14210_e13581) - (assign14210_e13576 * ((locals.var_q_temp_dn4 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn4)))) / (assign14210_e13581 * assign14210_e13581));
        locals.var_q_eps2_dn6 = ((((((((-locals.var_q_zero_dn6) * locals.var_q_d1_zero) + (assign14210_e13572 * locals.var_q_d1_zero_dn6)) * locals.var_q_temp) + (assign14210_e13574 * locals.var_q_temp_dn6)) * assign14210_e13581) - (assign14210_e13576 * ((locals.var_q_temp_dn6 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn6)))) / (assign14210_e13581 * assign14210_e13581));
        locals.var_q_eps2_dn7 = ((((((((-locals.var_q_zero_dn7) * locals.var_q_d1_zero) + (assign14210_e13572 * locals.var_q_d1_zero_dn7)) * locals.var_q_temp) + (assign14210_e13574 * locals.var_q_temp_dn7)) * assign14210_e13581) - (assign14210_e13576 * ((locals.var_q_temp_dn7 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn7)))) / (assign14210_e13581 * assign14210_e13581));
        locals.var_q_eps2_dn8 = ((((((((-locals.var_q_zero_dn8) * locals.var_q_d1_zero) + (assign14210_e13572 * locals.var_q_d1_zero_dn8)) * locals.var_q_temp) + (assign14210_e13574 * locals.var_q_temp_dn8)) * assign14210_e13581) - (assign14210_e13576 * ((locals.var_q_temp_dn8 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn8)))) / (assign14210_e13581 * assign14210_e13581));
        locals.var_q_eps2_dn9 = ((((((((-locals.var_q_zero_dn9) * locals.var_q_d1_zero) + (assign14210_e13572 * locals.var_q_d1_zero_dn9)) * locals.var_q_temp) + (assign14210_e13574 * locals.var_q_temp_dn9)) * assign14210_e13581) - (assign14210_e13576 * ((locals.var_q_temp_dn9 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn9)))) / (assign14210_e13581 * assign14210_e13581));

        let assign14220_e13585: f64 = (locals.var_q1s + locals.var_q_eps2);
        locals.var_q1s = assign14220_e13585;
        locals.var_q1s_dn4 = (locals.var_q1s_dn4 + locals.var_q_eps2_dn4);
        locals.var_q1s_dn6 = (locals.var_q1s_dn6 + locals.var_q_eps2_dn6);
        locals.var_q1s_dn7 = (locals.var_q1s_dn7 + locals.var_q_eps2_dn7);
        locals.var_q1s_dn8 = (locals.var_q1s_dn8 + locals.var_q_eps2_dn8);
        locals.var_q1s_dn9 = (locals.var_q1s_dn9 + locals.var_q_eps2_dn9);

        let assign14230_e13588: f64 = if p.p10 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard579 = assign14230_e13588;

        let assign14240_e13590: f64 = (locals.var_q_eps2).abs();
        let assign14240_e13592: f64 = if assign14240_e13590 > 0.01 { 1.0 } else { 0.0 };
        locals.var_guard580 = assign14240_e13592;

        let (assign14250_e13600, assign14250_e13600_d_n4, assign14250_e13600_d_n6, assign14250_e13600_d_n7, assign14250_e13600_d_n8, assign14250_e13600_d_n9,) = {
    if ((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) {
        let assign14250_e13598: f64 = (locals.var_k1 * locals.var_q1s);
        (assign14250_e13598, ((locals.var_k1_dn4 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn4)), ((locals.var_k1_dn6 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn6)), ((locals.var_k1_dn7 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn7)), ((locals.var_k1_dn8 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn8)), ((locals.var_k1_dn9 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn9)),)
    } else {
        (locals.var_q_k1q1, locals.var_q_k1q1_dn4, locals.var_q_k1q1_dn6, locals.var_q_k1q1_dn7, locals.var_q_k1q1_dn8, locals.var_q_k1q1_dn9,)
    }
};
        locals.var_q_k1q1 = assign14250_e13600;
        locals.var_q_k1q1_dn4 = assign14250_e13600_d_n4;
        locals.var_q_k1q1_dn6 = assign14250_e13600_d_n6;
        locals.var_q_k1q1_dn7 = assign14250_e13600_d_n7;
        locals.var_q_k1q1_dn8 = assign14250_e13600_d_n8;
        locals.var_q_k1q1_dn9 = assign14250_e13600_d_n9;

        let assign14260_e13603: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign14260_e13605: f64 = assign14260_e13603;
        let assign14260_e13607: f64 = if assign14260_e13605 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard581 = assign14260_e13607;

        let (assign14270_e13620, assign14270_e13620_d_n4, assign14270_e13620_d_n6, assign14270_e13620_d_n7, assign14270_e13620_d_n8, assign14270_e13620_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard581 != 0.0)) {
        let assign14270_e13615: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign14270_e13617: f64 = assign14270_e13615;
        let assign14270_e13618: f64 = (assign14270_e13617).exp();
        (assign14270_e13618, (assign14270_e13618 * (locals.var_xg1x_dn4 - locals.var_q1s_dn4)), (assign14270_e13618 * (locals.var_xg1x_dn6 - locals.var_q1s_dn6)), (assign14270_e13618 * (locals.var_xg1x_dn7 - locals.var_q1s_dn7)), (assign14270_e13618 * (locals.var_xg1x_dn8 - locals.var_q1s_dn8)), (assign14270_e13618 * (locals.var_xg1x_dn9 - locals.var_q1s_dn9)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign14270_e13620;
        locals.var_q_temp1_dn4 = assign14270_e13620_d_n4;
        locals.var_q_temp1_dn6 = assign14270_e13620_d_n6;
        locals.var_q_temp1_dn7 = assign14270_e13620_d_n7;
        locals.var_q_temp1_dn8 = assign14270_e13620_d_n8;
        locals.var_q_temp1_dn9 = assign14270_e13620_d_n9;

        let (assign14280_e13663, assign14280_e13663_d_n4, assign14280_e13663_d_n6, assign14280_e13663_d_n7, assign14280_e13663_d_n8, assign14280_e13663_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard581 == 0.0)) {
        let assign14280_e13631: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign14280_e13633: f64 = assign14280_e13631;
        let assign14280_e13635: f64 = (assign14280_e13633 - 80.0);
        let assign14280_e13640: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign14280_e13642: f64 = assign14280_e13640;
        let assign14280_e13644: f64 = (assign14280_e13642 - 80.0);
        let assign14280_e13645: f64 = (0.5 * assign14280_e13644);
        let assign14280_e13649: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign14280_e13651: f64 = assign14280_e13649;
        let assign14280_e13653: f64 = (assign14280_e13651 - 80.0);
        let assign14280_e13655: f64 = (assign14280_e13653 * 0.3333333333333);
        let assign14280_e13656: f64 = (1.0 + assign14280_e13655);
        let assign14280_e13657: f64 = (assign14280_e13645 * assign14280_e13656);
        let assign14280_e13658: f64 = (1.0 + assign14280_e13657);
        let assign14280_e13659: f64 = (assign14280_e13635 * assign14280_e13658);
        let assign14280_e13660: f64 = (1.0 + assign14280_e13659);
        let assign14280_e13661: f64 = (5.54062e34 * assign14280_e13660);
        (assign14280_e13661, (5.54062e34 * (((locals.var_xg1x_dn4 - locals.var_q1s_dn4) * assign14280_e13658) + (assign14280_e13635 * (((0.5 * (locals.var_xg1x_dn4 - locals.var_q1s_dn4)) * assign14280_e13656) + (assign14280_e13645 * ((locals.var_xg1x_dn4 - locals.var_q1s_dn4) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn6 - locals.var_q1s_dn6) * assign14280_e13658) + (assign14280_e13635 * (((0.5 * (locals.var_xg1x_dn6 - locals.var_q1s_dn6)) * assign14280_e13656) + (assign14280_e13645 * ((locals.var_xg1x_dn6 - locals.var_q1s_dn6) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn7 - locals.var_q1s_dn7) * assign14280_e13658) + (assign14280_e13635 * (((0.5 * (locals.var_xg1x_dn7 - locals.var_q1s_dn7)) * assign14280_e13656) + (assign14280_e13645 * ((locals.var_xg1x_dn7 - locals.var_q1s_dn7) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn8 - locals.var_q1s_dn8) * assign14280_e13658) + (assign14280_e13635 * (((0.5 * (locals.var_xg1x_dn8 - locals.var_q1s_dn8)) * assign14280_e13656) + (assign14280_e13645 * ((locals.var_xg1x_dn8 - locals.var_q1s_dn8) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn9 - locals.var_q1s_dn9) * assign14280_e13658) + (assign14280_e13635 * (((0.5 * (locals.var_xg1x_dn9 - locals.var_q1s_dn9)) * assign14280_e13656) + (assign14280_e13645 * ((locals.var_xg1x_dn9 - locals.var_q1s_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign14280_e13663;
        locals.var_q_temp1_dn4 = assign14280_e13663_d_n4;
        locals.var_q_temp1_dn6 = assign14280_e13663_d_n6;
        locals.var_q_temp1_dn7 = assign14280_e13663_d_n7;
        locals.var_q_temp1_dn8 = assign14280_e13663_d_n8;
        locals.var_q_temp1_dn9 = assign14280_e13663_d_n9;

        let (assign14290_e13671, assign14290_e13671_d_n4, assign14290_e13671_d_n6, assign14290_e13671_d_n7, assign14290_e13671_d_n8, assign14290_e13671_d_n9,) = {
    if ((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) {
        let assign14290_e13669: f64 = (locals.var_a0 * locals.var_q_temp1);
        (assign14290_e13669, ((locals.var_a0_dn4 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn4)), ((locals.var_a0_dn6 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn6)), ((locals.var_a0_dn7 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn7)), ((locals.var_a0_dn8 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn8)), ((locals.var_a0_dn9 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_aexp, locals.var_q_aexp_dn4, locals.var_q_aexp_dn6, locals.var_q_aexp_dn7, locals.var_q_aexp_dn8, locals.var_q_aexp_dn9,)
    }
};
        locals.var_q_aexp = assign14290_e13671;
        locals.var_q_aexp_dn4 = assign14290_e13671_d_n4;
        locals.var_q_aexp_dn6 = assign14290_e13671_d_n6;
        locals.var_q_aexp_dn7 = assign14290_e13671_d_n7;
        locals.var_q_aexp_dn8 = assign14290_e13671_d_n8;
        locals.var_q_aexp_dn9 = assign14290_e13671_d_n9;

        let (assign14300_e13681, assign14300_e13681_d_n4, assign14300_e13681_d_n6, assign14300_e13681_d_n7, assign14300_e13681_d_n8, assign14300_e13681_d_n9,) = {
    if ((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) {
        let assign14300_e13677: f64 = (locals.var_q_k1q1 * locals.var_q_k1q1);
        let assign14300_e13679: f64 = (assign14300_e13677 - locals.var_q_aexp);
        (assign14300_e13679, (((locals.var_q_k1q1_dn4 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn4)) - locals.var_q_aexp_dn4), (((locals.var_q_k1q1_dn6 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn6)) - locals.var_q_aexp_dn6), (((locals.var_q_k1q1_dn7 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn7)) - locals.var_q_aexp_dn7), (((locals.var_q_k1q1_dn8 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn8)) - locals.var_q_aexp_dn8), (((locals.var_q_k1q1_dn9 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn9)) - locals.var_q_aexp_dn9),)
    } else {
        (locals.var_q_qsq, locals.var_q_qsq_dn4, locals.var_q_qsq_dn6, locals.var_q_qsq_dn7, locals.var_q_qsq_dn8, locals.var_q_qsq_dn9,)
    }
};
        locals.var_q_qsq = assign14300_e13681;
        locals.var_q_qsq_dn4 = assign14300_e13681_d_n4;
        locals.var_q_qsq_dn6 = assign14300_e13681_d_n6;
        locals.var_q_qsq_dn7 = assign14300_e13681_d_n7;
        locals.var_q_qsq_dn8 = assign14300_e13681_d_n8;
        locals.var_q_qsq_dn9 = assign14300_e13681_d_n9;

        let (assign14310_e13693, assign14310_e13693_d_n4, assign14310_e13693_d_n6, assign14310_e13693_d_n7, assign14310_e13693_d_n8, assign14310_e13693_d_n9,) = {
    if ((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) {
        let assign14310_e13687: f64 = (2.0 * locals.var_k1);
        let assign14310_e13689: f64 = (assign14310_e13687 * locals.var_q_k1q1);
        let assign14310_e13691: f64 = (assign14310_e13689 + locals.var_q_aexp);
        (assign14310_e13691, ((((2.0 * locals.var_k1_dn4) * locals.var_q_k1q1) + (assign14310_e13687 * locals.var_q_k1q1_dn4)) + locals.var_q_aexp_dn4), ((((2.0 * locals.var_k1_dn6) * locals.var_q_k1q1) + (assign14310_e13687 * locals.var_q_k1q1_dn6)) + locals.var_q_aexp_dn6), ((((2.0 * locals.var_k1_dn7) * locals.var_q_k1q1) + (assign14310_e13687 * locals.var_q_k1q1_dn7)) + locals.var_q_aexp_dn7), ((((2.0 * locals.var_k1_dn8) * locals.var_q_k1q1) + (assign14310_e13687 * locals.var_q_k1q1_dn8)) + locals.var_q_aexp_dn8), ((((2.0 * locals.var_k1_dn9) * locals.var_q_k1q1) + (assign14310_e13687 * locals.var_q_k1q1_dn9)) + locals.var_q_aexp_dn9),)
    } else {
        (locals.var_q_d1_qsq, locals.var_q_d1_qsq_dn4, locals.var_q_d1_qsq_dn6, locals.var_q_d1_qsq_dn7, locals.var_q_d1_qsq_dn8, locals.var_q_d1_qsq_dn9,)
    }
};
        locals.var_q_d1_qsq = assign14310_e13693;
        locals.var_q_d1_qsq_dn4 = assign14310_e13693_d_n4;
        locals.var_q_d1_qsq_dn6 = assign14310_e13693_d_n6;
        locals.var_q_d1_qsq_dn7 = assign14310_e13693_d_n7;
        locals.var_q_d1_qsq_dn8 = assign14310_e13693_d_n8;
        locals.var_q_d1_qsq_dn9 = assign14310_e13693_d_n9;

        let (assign14320_e13705, assign14320_e13705_d_n4, assign14320_e13705_d_n6, assign14320_e13705_d_n7, assign14320_e13705_d_n8, assign14320_e13705_d_n9,) = {
    if ((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) {
        let assign14320_e13699: f64 = (2.0 * locals.var_k1);
        let assign14320_e13701: f64 = (assign14320_e13699 * locals.var_k1);
        let assign14320_e13703: f64 = (assign14320_e13701 - locals.var_q_aexp);
        (assign14320_e13703, ((((2.0 * locals.var_k1_dn4) * locals.var_k1) + (assign14320_e13699 * locals.var_k1_dn4)) - locals.var_q_aexp_dn4), ((((2.0 * locals.var_k1_dn6) * locals.var_k1) + (assign14320_e13699 * locals.var_k1_dn6)) - locals.var_q_aexp_dn6), ((((2.0 * locals.var_k1_dn7) * locals.var_k1) + (assign14320_e13699 * locals.var_k1_dn7)) - locals.var_q_aexp_dn7), ((((2.0 * locals.var_k1_dn8) * locals.var_k1) + (assign14320_e13699 * locals.var_k1_dn8)) - locals.var_q_aexp_dn8), ((((2.0 * locals.var_k1_dn9) * locals.var_k1) + (assign14320_e13699 * locals.var_k1_dn9)) - locals.var_q_aexp_dn9),)
    } else {
        (locals.var_q_d2_qsq, locals.var_q_d2_qsq_dn4, locals.var_q_d2_qsq_dn6, locals.var_q_d2_qsq_dn7, locals.var_q_d2_qsq_dn8, locals.var_q_d2_qsq_dn9,)
    }
};
        locals.var_q_d2_qsq = assign14320_e13705;
        locals.var_q_d2_qsq_dn4 = assign14320_e13705_d_n4;
        locals.var_q_d2_qsq_dn6 = assign14320_e13705_d_n6;
        locals.var_q_d2_qsq_dn7 = assign14320_e13705_d_n7;
        locals.var_q_d2_qsq_dn8 = assign14320_e13705_d_n8;
        locals.var_q_d2_qsq_dn9 = assign14320_e13705_d_n9;

        let assign14330_e13708: f64 = (-0.005);
        let assign14330_e13709: f64 = if locals.var_q_qsq < assign14330_e13708 { 1.0 } else { 0.0 };
        locals.var_guard582 = assign14330_e13709;

        let (assign14340_e13719, assign14340_e13719_d_n4, assign14340_e13719_d_n6, assign14340_e13719_d_n7, assign14340_e13719_d_n8, assign14340_e13719_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 != 0.0)) {
        let assign14340_e13716: f64 = (locals.var_q_qsq).abs();
        let assign14340_e13717: f64 = (assign14340_e13716).sqrt();
        (assign14340_e13717, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign14340_e13717)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign14340_e13717)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign14340_e13717)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign14340_e13717)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign14340_e13717)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign14340_e13719;
        locals.var_q_rac_qsq_dn4 = assign14340_e13719_d_n4;
        locals.var_q_rac_qsq_dn6 = assign14340_e13719_d_n6;
        locals.var_q_rac_qsq_dn7 = assign14340_e13719_d_n7;
        locals.var_q_rac_qsq_dn8 = assign14340_e13719_d_n8;
        locals.var_q_rac_qsq_dn9 = assign14340_e13719_d_n9;

        let (assign14350_e13732, assign14350_e13732_d_n4, assign14350_e13732_d_n6, assign14350_e13732_d_n7, assign14350_e13732_d_n8, assign14350_e13732_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 != 0.0)) {
        let assign14350_e13728: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign14350_e13729: f64 = (assign14350_e13728).tan();
        let assign14350_e13730: f64 = (locals.var_q_rac_qsq / assign14350_e13729);
        (assign14350_e13730, (((locals.var_q_rac_qsq_dn4 * assign14350_e13729) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn4) / ((assign14350_e13728).cos() * (assign14350_e13728).cos())))) / (assign14350_e13729 * assign14350_e13729)), (((locals.var_q_rac_qsq_dn6 * assign14350_e13729) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn6) / ((assign14350_e13728).cos() * (assign14350_e13728).cos())))) / (assign14350_e13729 * assign14350_e13729)), (((locals.var_q_rac_qsq_dn7 * assign14350_e13729) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn7) / ((assign14350_e13728).cos() * (assign14350_e13728).cos())))) / (assign14350_e13729 * assign14350_e13729)), (((locals.var_q_rac_qsq_dn8 * assign14350_e13729) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn8) / ((assign14350_e13728).cos() * (assign14350_e13728).cos())))) / (assign14350_e13729 * assign14350_e13729)), (((locals.var_q_rac_qsq_dn9 * assign14350_e13729) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn9) / ((assign14350_e13728).cos() * (assign14350_e13728).cos())))) / (assign14350_e13729 * assign14350_e13729)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign14350_e13732;
        locals.var_q_qcoth_dn4 = assign14350_e13732_d_n4;
        locals.var_q_qcoth_dn6 = assign14350_e13732_d_n6;
        locals.var_q_qcoth_dn7 = assign14350_e13732_d_n7;
        locals.var_q_qcoth_dn8 = assign14350_e13732_d_n8;
        locals.var_q_qcoth_dn9 = assign14350_e13732_d_n9;

        let (assign14360_e13744, assign14360_e13744_d_n4, assign14360_e13744_d_n6, assign14360_e13744_d_n7, assign14360_e13744_d_n8, assign14360_e13744_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 != 0.0)) {
        let assign14360_e13740: f64 = (0.25 * locals.var_q_d1_qsq);
        let assign14360_e13742: f64 = (assign14360_e13740 / locals.var_q_qsq);
        (assign14360_e13742, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign14360_e13740 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign14360_e13740 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign14360_e13740 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign14360_e13740 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign14360_e13740 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign14360_e13744;
        locals.var_q_temp1_dn4 = assign14360_e13744_d_n4;
        locals.var_q_temp1_dn6 = assign14360_e13744_d_n6;
        locals.var_q_temp1_dn7 = assign14360_e13744_d_n7;
        locals.var_q_temp1_dn8 = assign14360_e13744_d_n8;
        locals.var_q_temp1_dn9 = assign14360_e13744_d_n9;

        let (assign14370_e13760, assign14370_e13760_d_n4, assign14370_e13760_d_n6, assign14370_e13760_d_n7, assign14370_e13760_d_n8, assign14370_e13760_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 != 0.0)) {
        let assign14370_e13754: f64 = (2.0 - locals.var_q_qcoth);
        let assign14370_e13755: f64 = (locals.var_q_qcoth * assign14370_e13754);
        let assign14370_e13756: f64 = (locals.var_q_qsq + assign14370_e13755);
        let assign14370_e13758: f64 = (assign14370_e13756 * locals.var_q_temp1);
        (assign14370_e13758, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign14370_e13754) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign14370_e13756 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign14370_e13754) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign14370_e13756 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign14370_e13754) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign14370_e13756 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign14370_e13754) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign14370_e13756 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign14370_e13754) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign14370_e13756 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign14370_e13760;
        locals.var_q_d1_qcoth_dn4 = assign14370_e13760_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign14370_e13760_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign14370_e13760_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign14370_e13760_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign14370_e13760_d_n9;

    }

    pub(super) fn stamp_transient_block_35(
        locals: &mut StampLocals,
    ) {
        let (assign14380_e13784, assign14380_e13784_d_n4, assign14380_e13784_d_n6, assign14380_e13784_d_n7, assign14380_e13784_d_n8, assign14380_e13784_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 != 0.0)) {
        let assign14380_e13769: f64 = (2.0 * locals.var_q_d1_qcoth);
        let assign14380_e13772: f64 = (1.0 + locals.var_q_qcoth);
        let assign14380_e13773: f64 = (assign14380_e13769 * assign14380_e13772);
        let assign14380_e13774: f64 = (locals.var_q_d1_qsq - assign14380_e13773);
        let assign14380_e13776: f64 = (assign14380_e13774 * locals.var_q_temp1);
        let assign14380_e13779: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
        let assign14380_e13781: f64 = (assign14380_e13779 / locals.var_q_d1_qsq);
        let assign14380_e13782: f64 = (assign14380_e13776 + assign14380_e13781);
        (assign14380_e13782, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign14380_e13772) + (assign14380_e13769 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign14380_e13774 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign14380_e13779 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign14380_e13772) + (assign14380_e13769 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign14380_e13774 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign14380_e13779 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign14380_e13772) + (assign14380_e13769 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign14380_e13774 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign14380_e13779 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign14380_e13772) + (assign14380_e13769 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign14380_e13774 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign14380_e13779 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign14380_e13772) + (assign14380_e13769 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign14380_e13774 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign14380_e13779 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign14380_e13784;
        locals.var_q_d2_qcoth_dn4 = assign14380_e13784_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign14380_e13784_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign14380_e13784_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign14380_e13784_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign14380_e13784_d_n9;

        let (assign14390_e13796, assign14390_e13796_d_n4, assign14390_e13796_d_n6, assign14390_e13796_d_n7, assign14390_e13796_d_n8, assign14390_e13796_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 != 0.0)) {
        let assign14390_e13793: f64 = (0.5 * locals.var_q_qcoth);
        let assign14390_e13794: f64 = (1.0 - assign14390_e13793);
        (assign14390_e13794, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign14390_e13796;
        locals.var_q_temp2_dn4 = assign14390_e13796_d_n4;
        locals.var_q_temp2_dn6 = assign14390_e13796_d_n6;
        locals.var_q_temp2_dn7 = assign14390_e13796_d_n7;
        locals.var_q_temp2_dn8 = assign14390_e13796_d_n8;
        locals.var_q_temp2_dn9 = assign14390_e13796_d_n9;

        let (assign14400_e13808, assign14400_e13808_d_n4, assign14400_e13808_d_n6, assign14400_e13808_d_n7, assign14400_e13808_d_n8, assign14400_e13808_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 != 0.0)) {
        let assign14400_e13804: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
        let assign14400_e13806: f64 = (assign14400_e13804 * locals.var_q_temp2);
        (assign14400_e13806, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign14400_e13804 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign14400_e13804 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign14400_e13804 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign14400_e13804 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign14400_e13804 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign14400_e13808;
        locals.var_q_d1_ln_dn4 = assign14400_e13808_d_n4;
        locals.var_q_d1_ln_dn6 = assign14400_e13808_d_n6;
        locals.var_q_d1_ln_dn7 = assign14400_e13808_d_n7;
        locals.var_q_d1_ln_dn8 = assign14400_e13808_d_n8;
        locals.var_q_d1_ln_dn9 = assign14400_e13808_d_n9;

        let (assign14410_e13828, assign14410_e13828_d_n4, assign14410_e13828_d_n6, assign14410_e13828_d_n7, assign14410_e13828_d_n8, assign14410_e13828_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 != 0.0)) {
        let assign14410_e13816: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
        let assign14410_e13821: f64 = (0.5 * locals.var_q_d1_qcoth);
        let assign14410_e13822: f64 = (locals.var_q_d1_ln + assign14410_e13821);
        let assign14410_e13823: f64 = (locals.var_q_d1_qsq * assign14410_e13822);
        let assign14410_e13824: f64 = (assign14410_e13816 - assign14410_e13823);
        let assign14410_e13826: f64 = (assign14410_e13824 / locals.var_q_qsq);
        (assign14410_e13826, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign14410_e13822) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign14410_e13824 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign14410_e13822) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign14410_e13824 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign14410_e13822) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign14410_e13824 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign14410_e13822) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign14410_e13824 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign14410_e13822) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign14410_e13824 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign14410_e13828;
        locals.var_q_d2_ln_dn4 = assign14410_e13828_d_n4;
        locals.var_q_d2_ln_dn6 = assign14410_e13828_d_n6;
        locals.var_q_d2_ln_dn7 = assign14410_e13828_d_n7;
        locals.var_q_d2_ln_dn8 = assign14410_e13828_d_n8;
        locals.var_q_d2_ln_dn9 = assign14410_e13828_d_n9;

        let assign14420_e13831: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard583 = assign14420_e13831;

        let (assign14430_e13844, assign14430_e13844_d_n4, assign14430_e13844_d_n6, assign14430_e13844_d_n7, assign14430_e13844_d_n8, assign14430_e13844_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) {
        let assign14430_e13841: f64 = (locals.var_q_qsq).abs();
        let assign14430_e13842: f64 = (assign14430_e13841).sqrt();
        (assign14430_e13842, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign14430_e13842)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign14430_e13842)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign14430_e13842)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign14430_e13842)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign14430_e13842)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign14430_e13844;
        locals.var_q_rac_qsq_dn4 = assign14430_e13844_d_n4;
        locals.var_q_rac_qsq_dn6 = assign14430_e13844_d_n6;
        locals.var_q_rac_qsq_dn7 = assign14430_e13844_d_n7;
        locals.var_q_rac_qsq_dn8 = assign14430_e13844_d_n8;
        locals.var_q_rac_qsq_dn9 = assign14430_e13844_d_n9;

        let (assign14440_e13857, assign14440_e13857_d_n4, assign14440_e13857_d_n6, assign14440_e13857_d_n7, assign14440_e13857_d_n8, assign14440_e13857_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) {
        let assign14440_e13854: f64 = (-locals.var_q_rac_qsq);
        let assign14440_e13855: f64 = (assign14440_e13854).exp();
        (assign14440_e13855, (assign14440_e13855 * (-locals.var_q_rac_qsq_dn4)), (assign14440_e13855 * (-locals.var_q_rac_qsq_dn6)), (assign14440_e13855 * (-locals.var_q_rac_qsq_dn7)), (assign14440_e13855 * (-locals.var_q_rac_qsq_dn8)), (assign14440_e13855 * (-locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_invexpq, locals.var_q_invexpq_dn4, locals.var_q_invexpq_dn6, locals.var_q_invexpq_dn7, locals.var_q_invexpq_dn8, locals.var_q_invexpq_dn9,)
    }
};
        locals.var_q_invexpq = assign14440_e13857;
        locals.var_q_invexpq_dn4 = assign14440_e13857_d_n4;
        locals.var_q_invexpq_dn6 = assign14440_e13857_d_n6;
        locals.var_q_invexpq_dn7 = assign14440_e13857_d_n7;
        locals.var_q_invexpq_dn8 = assign14440_e13857_d_n8;
        locals.var_q_invexpq_dn9 = assign14440_e13857_d_n9;

        let (assign14450_e13876, assign14450_e13876_d_n4, assign14450_e13876_d_n6, assign14450_e13876_d_n7, assign14450_e13876_d_n8, assign14450_e13876_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) {
        let assign14450_e13869: f64 = (1.0 + locals.var_q_invexpq);
        let assign14450_e13870: f64 = (locals.var_q_rac_qsq * assign14450_e13869);
        let assign14450_e13873: f64 = (1.0 - locals.var_q_invexpq);
        let assign14450_e13874: f64 = (assign14450_e13870 / assign14450_e13873);
        (assign14450_e13874, (((((locals.var_q_rac_qsq_dn4 * assign14450_e13869) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn4)) * assign14450_e13873) - (assign14450_e13870 * (-locals.var_q_invexpq_dn4))) / (assign14450_e13873 * assign14450_e13873)), (((((locals.var_q_rac_qsq_dn6 * assign14450_e13869) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn6)) * assign14450_e13873) - (assign14450_e13870 * (-locals.var_q_invexpq_dn6))) / (assign14450_e13873 * assign14450_e13873)), (((((locals.var_q_rac_qsq_dn7 * assign14450_e13869) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn7)) * assign14450_e13873) - (assign14450_e13870 * (-locals.var_q_invexpq_dn7))) / (assign14450_e13873 * assign14450_e13873)), (((((locals.var_q_rac_qsq_dn8 * assign14450_e13869) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn8)) * assign14450_e13873) - (assign14450_e13870 * (-locals.var_q_invexpq_dn8))) / (assign14450_e13873 * assign14450_e13873)), (((((locals.var_q_rac_qsq_dn9 * assign14450_e13869) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn9)) * assign14450_e13873) - (assign14450_e13870 * (-locals.var_q_invexpq_dn9))) / (assign14450_e13873 * assign14450_e13873)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign14450_e13876;
        locals.var_q_qcoth_dn4 = assign14450_e13876_d_n4;
        locals.var_q_qcoth_dn6 = assign14450_e13876_d_n6;
        locals.var_q_qcoth_dn7 = assign14450_e13876_d_n7;
        locals.var_q_qcoth_dn8 = assign14450_e13876_d_n8;
        locals.var_q_qcoth_dn9 = assign14450_e13876_d_n9;

        let (assign14460_e13891, assign14460_e13891_d_n4, assign14460_e13891_d_n6, assign14460_e13891_d_n7, assign14460_e13891_d_n8, assign14460_e13891_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) {
        let assign14460_e13887: f64 = (0.25 * locals.var_q_d1_qsq);
        let assign14460_e13889: f64 = (assign14460_e13887 / locals.var_q_qsq);
        (assign14460_e13889, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign14460_e13887 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign14460_e13887 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign14460_e13887 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign14460_e13887 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign14460_e13887 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign14460_e13891;
        locals.var_q_temp1_dn4 = assign14460_e13891_d_n4;
        locals.var_q_temp1_dn6 = assign14460_e13891_d_n6;
        locals.var_q_temp1_dn7 = assign14460_e13891_d_n7;
        locals.var_q_temp1_dn8 = assign14460_e13891_d_n8;
        locals.var_q_temp1_dn9 = assign14460_e13891_d_n9;

        let (assign14470_e13910, assign14470_e13910_d_n4, assign14470_e13910_d_n6, assign14470_e13910_d_n7, assign14470_e13910_d_n8, assign14470_e13910_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) {
        let assign14470_e13904: f64 = (2.0 - locals.var_q_qcoth);
        let assign14470_e13905: f64 = (locals.var_q_qcoth * assign14470_e13904);
        let assign14470_e13906: f64 = (locals.var_q_qsq + assign14470_e13905);
        let assign14470_e13908: f64 = (assign14470_e13906 * locals.var_q_temp1);
        (assign14470_e13908, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign14470_e13904) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign14470_e13906 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign14470_e13904) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign14470_e13906 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign14470_e13904) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign14470_e13906 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign14470_e13904) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign14470_e13906 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign14470_e13904) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign14470_e13906 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign14470_e13910;
        locals.var_q_d1_qcoth_dn4 = assign14470_e13910_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign14470_e13910_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign14470_e13910_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign14470_e13910_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign14470_e13910_d_n9;

        let (assign14480_e13937, assign14480_e13937_d_n4, assign14480_e13937_d_n6, assign14480_e13937_d_n7, assign14480_e13937_d_n8, assign14480_e13937_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) {
        let assign14480_e13922: f64 = (2.0 * locals.var_q_d1_qcoth);
        let assign14480_e13925: f64 = (1.0 + locals.var_q_qcoth);
        let assign14480_e13926: f64 = (assign14480_e13922 * assign14480_e13925);
        let assign14480_e13927: f64 = (locals.var_q_d1_qsq - assign14480_e13926);
        let assign14480_e13929: f64 = (assign14480_e13927 * locals.var_q_temp1);
        let assign14480_e13932: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
        let assign14480_e13934: f64 = (assign14480_e13932 / locals.var_q_d1_qsq);
        let assign14480_e13935: f64 = (assign14480_e13929 + assign14480_e13934);
        (assign14480_e13935, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign14480_e13925) + (assign14480_e13922 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign14480_e13927 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign14480_e13932 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign14480_e13925) + (assign14480_e13922 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign14480_e13927 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign14480_e13932 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign14480_e13925) + (assign14480_e13922 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign14480_e13927 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign14480_e13932 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign14480_e13925) + (assign14480_e13922 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign14480_e13927 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign14480_e13932 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign14480_e13925) + (assign14480_e13922 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign14480_e13927 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign14480_e13932 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign14480_e13937;
        locals.var_q_d2_qcoth_dn4 = assign14480_e13937_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign14480_e13937_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign14480_e13937_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign14480_e13937_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign14480_e13937_d_n9;

        let (assign14490_e13952, assign14490_e13952_d_n4, assign14490_e13952_d_n6, assign14490_e13952_d_n7, assign14490_e13952_d_n8, assign14490_e13952_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) {
        let assign14490_e13949: f64 = (0.5 * locals.var_q_qcoth);
        let assign14490_e13950: f64 = (1.0 - assign14490_e13949);
        (assign14490_e13950, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign14490_e13952;
        locals.var_q_temp2_dn4 = assign14490_e13952_d_n4;
        locals.var_q_temp2_dn6 = assign14490_e13952_d_n6;
        locals.var_q_temp2_dn7 = assign14490_e13952_d_n7;
        locals.var_q_temp2_dn8 = assign14490_e13952_d_n8;
        locals.var_q_temp2_dn9 = assign14490_e13952_d_n9;

        let (assign14500_e13967, assign14500_e13967_d_n4, assign14500_e13967_d_n6, assign14500_e13967_d_n7, assign14500_e13967_d_n8, assign14500_e13967_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) {
        let assign14500_e13963: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
        let assign14500_e13965: f64 = (assign14500_e13963 * locals.var_q_temp2);
        (assign14500_e13965, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign14500_e13963 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign14500_e13963 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign14500_e13963 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign14500_e13963 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign14500_e13963 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign14500_e13967;
        locals.var_q_d1_ln_dn4 = assign14500_e13967_d_n4;
        locals.var_q_d1_ln_dn6 = assign14500_e13967_d_n6;
        locals.var_q_d1_ln_dn7 = assign14500_e13967_d_n7;
        locals.var_q_d1_ln_dn8 = assign14500_e13967_d_n8;
        locals.var_q_d1_ln_dn9 = assign14500_e13967_d_n9;

        let (assign14510_e13990, assign14510_e13990_d_n4, assign14510_e13990_d_n6, assign14510_e13990_d_n7, assign14510_e13990_d_n8, assign14510_e13990_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) {
        let assign14510_e13978: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
        let assign14510_e13983: f64 = (0.5 * locals.var_q_d1_qcoth);
        let assign14510_e13984: f64 = (locals.var_q_d1_ln + assign14510_e13983);
        let assign14510_e13985: f64 = (locals.var_q_d1_qsq * assign14510_e13984);
        let assign14510_e13986: f64 = (assign14510_e13978 - assign14510_e13985);
        let assign14510_e13988: f64 = (assign14510_e13986 / locals.var_q_qsq);
        (assign14510_e13988, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign14510_e13984) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign14510_e13986 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign14510_e13984) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign14510_e13986 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign14510_e13984) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign14510_e13986 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign14510_e13984) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign14510_e13986 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign14510_e13984) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign14510_e13986 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign14510_e13990;
        locals.var_q_d2_ln_dn4 = assign14510_e13990_d_n4;
        locals.var_q_d2_ln_dn6 = assign14510_e13990_d_n6;
        locals.var_q_d2_ln_dn7 = assign14510_e13990_d_n7;
        locals.var_q_d2_ln_dn8 = assign14510_e13990_d_n8;
        locals.var_q_d2_ln_dn9 = assign14510_e13990_d_n9;

        let (assign14520_e14020, assign14520_e14020_d_n4, assign14520_e14020_d_n6, assign14520_e14020_d_n7, assign14520_e14020_d_n8, assign14520_e14020_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 == 0.0)) {
        let assign14520_e14004: f64 = (locals.var_q_qsq * 0.0166666666667);
        let assign14520_e14008: f64 = (locals.var_q_qsq * 0.0238095238095);
        let assign14520_e14012: f64 = (locals.var_q_qsq * 0.025);
        let assign14520_e14013: f64 = (1.0 - assign14520_e14012);
        let assign14520_e14014: f64 = (assign14520_e14008 * assign14520_e14013);
        let assign14520_e14015: f64 = (1.0 - assign14520_e14014);
        let assign14520_e14016: f64 = (assign14520_e14004 * assign14520_e14015);
        let assign14520_e14017: f64 = (1.0 - assign14520_e14016);
        let assign14520_e14018: f64 = (0.1666666666667 * assign14520_e14017);
        (assign14520_e14018, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0166666666667) * assign14520_e14015) + (assign14520_e14004 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign14520_e14013) + (assign14520_e14008 * (-(locals.var_q_qsq_dn4 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0166666666667) * assign14520_e14015) + (assign14520_e14004 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign14520_e14013) + (assign14520_e14008 * (-(locals.var_q_qsq_dn6 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0166666666667) * assign14520_e14015) + (assign14520_e14004 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign14520_e14013) + (assign14520_e14008 * (-(locals.var_q_qsq_dn7 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0166666666667) * assign14520_e14015) + (assign14520_e14004 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign14520_e14013) + (assign14520_e14008 * (-(locals.var_q_qsq_dn8 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0166666666667) * assign14520_e14015) + (assign14520_e14004 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign14520_e14013) + (assign14520_e14008 * (-(locals.var_q_qsq_dn9 * 0.025))))))))),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign14520_e14020;
        locals.var_q_temp3_dn4 = assign14520_e14020_d_n4;
        locals.var_q_temp3_dn6 = assign14520_e14020_d_n6;
        locals.var_q_temp3_dn7 = assign14520_e14020_d_n7;
        locals.var_q_temp3_dn8 = assign14520_e14020_d_n8;
        locals.var_q_temp3_dn9 = assign14520_e14020_d_n9;

        let (assign14530_e14036, assign14530_e14036_d_n4, assign14530_e14036_d_n6, assign14530_e14036_d_n7, assign14530_e14036_d_n8, assign14530_e14036_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 == 0.0)) {
        let assign14530_e14033: f64 = (locals.var_q_qsq * locals.var_q_temp3);
        let assign14530_e14034: f64 = (2.0 + assign14530_e14033);
        (assign14530_e14034, ((locals.var_q_qsq_dn4 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn4)), ((locals.var_q_qsq_dn6 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn6)), ((locals.var_q_qsq_dn7 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn7)), ((locals.var_q_qsq_dn8 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn8)), ((locals.var_q_qsq_dn9 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn9)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign14530_e14036;
        locals.var_q_qcoth_dn4 = assign14530_e14036_d_n4;
        locals.var_q_qcoth_dn6 = assign14530_e14036_d_n6;
        locals.var_q_qcoth_dn7 = assign14530_e14036_d_n7;
        locals.var_q_qcoth_dn8 = assign14530_e14036_d_n8;
        locals.var_q_qcoth_dn9 = assign14530_e14036_d_n9;

        let (assign14540_e14066, assign14540_e14066_d_n4, assign14540_e14066_d_n6, assign14540_e14066_d_n7, assign14540_e14066_d_n8, assign14540_e14066_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 == 0.0)) {
        let assign14540_e14050: f64 = (locals.var_q_qsq * 0.0333333333333);
        let assign14540_e14054: f64 = (locals.var_q_qsq * 0.0357142857143);
        let assign14540_e14058: f64 = (locals.var_q_qsq * 0.0333333333333);
        let assign14540_e14059: f64 = (1.0 - assign14540_e14058);
        let assign14540_e14060: f64 = (assign14540_e14054 * assign14540_e14059);
        let assign14540_e14061: f64 = (1.0 - assign14540_e14060);
        let assign14540_e14062: f64 = (assign14540_e14050 * assign14540_e14061);
        let assign14540_e14063: f64 = (1.0 - assign14540_e14062);
        let assign14540_e14064: f64 = (0.1666666666667 * assign14540_e14063);
        (assign14540_e14064, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0333333333333) * assign14540_e14061) + (assign14540_e14050 * (-(((locals.var_q_qsq_dn4 * 0.0357142857143) * assign14540_e14059) + (assign14540_e14054 * (-(locals.var_q_qsq_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0333333333333) * assign14540_e14061) + (assign14540_e14050 * (-(((locals.var_q_qsq_dn6 * 0.0357142857143) * assign14540_e14059) + (assign14540_e14054 * (-(locals.var_q_qsq_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0333333333333) * assign14540_e14061) + (assign14540_e14050 * (-(((locals.var_q_qsq_dn7 * 0.0357142857143) * assign14540_e14059) + (assign14540_e14054 * (-(locals.var_q_qsq_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0333333333333) * assign14540_e14061) + (assign14540_e14050 * (-(((locals.var_q_qsq_dn8 * 0.0357142857143) * assign14540_e14059) + (assign14540_e14054 * (-(locals.var_q_qsq_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0333333333333) * assign14540_e14061) + (assign14540_e14050 * (-(((locals.var_q_qsq_dn9 * 0.0357142857143) * assign14540_e14059) + (assign14540_e14054 * (-(locals.var_q_qsq_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign14540_e14066;
        locals.var_q_temp1_dn4 = assign14540_e14066_d_n4;
        locals.var_q_temp1_dn6 = assign14540_e14066_d_n6;
        locals.var_q_temp1_dn7 = assign14540_e14066_d_n7;
        locals.var_q_temp1_dn8 = assign14540_e14066_d_n8;
        locals.var_q_temp1_dn9 = assign14540_e14066_d_n9;

        let (assign14550_e14080, assign14550_e14080_d_n4, assign14550_e14080_d_n6, assign14550_e14080_d_n7, assign14550_e14080_d_n8, assign14550_e14080_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 == 0.0)) {
        let assign14550_e14078: f64 = (locals.var_q_d1_qsq * locals.var_q_temp1);
        (assign14550_e14078, ((locals.var_q_d1_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn4)), ((locals.var_q_d1_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn6)), ((locals.var_q_d1_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn7)), ((locals.var_q_d1_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn8)), ((locals.var_q_d1_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign14550_e14080;
        locals.var_q_d1_qcoth_dn4 = assign14550_e14080_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign14550_e14080_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign14550_e14080_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign14550_e14080_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign14550_e14080_d_n9;

        let (assign14560_e14110, assign14560_e14110_d_n4, assign14560_e14110_d_n6, assign14560_e14110_d_n7, assign14560_e14110_d_n8, assign14560_e14110_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 == 0.0)) {
        let assign14560_e14094: f64 = (locals.var_q_qsq * 0.0714285714286);
        let assign14560_e14098: f64 = (0.05 * locals.var_q_qsq);
        let assign14560_e14102: f64 = (0.0420875420875421 * locals.var_q_qsq);
        let assign14560_e14103: f64 = (1.0 - assign14560_e14102);
        let assign14560_e14104: f64 = (assign14560_e14098 * assign14560_e14103);
        let assign14560_e14105: f64 = (1.0 - assign14560_e14104);
        let assign14560_e14106: f64 = (assign14560_e14094 * assign14560_e14105);
        let assign14560_e14107: f64 = (1.0 - assign14560_e14106);
        let assign14560_e14108: f64 = (0.0055555555556 * assign14560_e14107);
        (assign14560_e14108, (0.0055555555556 * (-(((locals.var_q_qsq_dn4 * 0.0714285714286) * assign14560_e14105) + (assign14560_e14094 * (-(((0.05 * locals.var_q_qsq_dn4) * assign14560_e14103) + (assign14560_e14098 * (-(0.0420875420875421 * locals.var_q_qsq_dn4))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn6 * 0.0714285714286) * assign14560_e14105) + (assign14560_e14094 * (-(((0.05 * locals.var_q_qsq_dn6) * assign14560_e14103) + (assign14560_e14098 * (-(0.0420875420875421 * locals.var_q_qsq_dn6))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn7 * 0.0714285714286) * assign14560_e14105) + (assign14560_e14094 * (-(((0.05 * locals.var_q_qsq_dn7) * assign14560_e14103) + (assign14560_e14098 * (-(0.0420875420875421 * locals.var_q_qsq_dn7))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn8 * 0.0714285714286) * assign14560_e14105) + (assign14560_e14094 * (-(((0.05 * locals.var_q_qsq_dn8) * assign14560_e14103) + (assign14560_e14098 * (-(0.0420875420875421 * locals.var_q_qsq_dn8))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn9 * 0.0714285714286) * assign14560_e14105) + (assign14560_e14094 * (-(((0.05 * locals.var_q_qsq_dn9) * assign14560_e14103) + (assign14560_e14098 * (-(0.0420875420875421 * locals.var_q_qsq_dn9))))))))),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign14560_e14110;
        locals.var_q_temp2_dn4 = assign14560_e14110_d_n4;
        locals.var_q_temp2_dn6 = assign14560_e14110_d_n6;
        locals.var_q_temp2_dn7 = assign14560_e14110_d_n7;
        locals.var_q_temp2_dn8 = assign14560_e14110_d_n8;
        locals.var_q_temp2_dn9 = assign14560_e14110_d_n9;

        let (assign14570_e14130, assign14570_e14130_d_n4, assign14570_e14130_d_n6, assign14570_e14130_d_n7, assign14570_e14130_d_n8, assign14570_e14130_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 == 0.0)) {
        let assign14570_e14122: f64 = (locals.var_q_d2_qsq * locals.var_q_temp1);
        let assign14570_e14125: f64 = (locals.var_q_d1_qsq * locals.var_q_d1_qsq);
        let assign14570_e14127: f64 = (assign14570_e14125 * locals.var_q_temp2);
        let assign14570_e14128: f64 = (assign14570_e14122 - assign14570_e14127);
        (assign14570_e14128, (((locals.var_q_d2_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn4)) - ((((locals.var_q_d1_qsq_dn4 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn4)) * locals.var_q_temp2) + (assign14570_e14125 * locals.var_q_temp2_dn4))), (((locals.var_q_d2_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn6)) - ((((locals.var_q_d1_qsq_dn6 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn6)) * locals.var_q_temp2) + (assign14570_e14125 * locals.var_q_temp2_dn6))), (((locals.var_q_d2_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn7)) - ((((locals.var_q_d1_qsq_dn7 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn7)) * locals.var_q_temp2) + (assign14570_e14125 * locals.var_q_temp2_dn7))), (((locals.var_q_d2_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn8)) - ((((locals.var_q_d1_qsq_dn8 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn8)) * locals.var_q_temp2) + (assign14570_e14125 * locals.var_q_temp2_dn8))), (((locals.var_q_d2_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn9)) - ((((locals.var_q_d1_qsq_dn9 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn9)) * locals.var_q_temp2) + (assign14570_e14125 * locals.var_q_temp2_dn9))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign14570_e14130;
        locals.var_q_d2_qcoth_dn4 = assign14570_e14130_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign14570_e14130_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign14570_e14130_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign14570_e14130_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign14570_e14130_d_n9;

        let (assign14580_e14147, assign14580_e14147_d_n4, assign14580_e14147_d_n6, assign14580_e14147_d_n7, assign14580_e14147_d_n8, assign14580_e14147_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 == 0.0)) {
        let assign14580_e14141: f64 = (-0.5);
        let assign14580_e14143: f64 = (assign14580_e14141 * locals.var_q_d1_qsq);
        let assign14580_e14145: f64 = (assign14580_e14143 * locals.var_q_temp3);
        (assign14580_e14145, (((assign14580_e14141 * locals.var_q_d1_qsq_dn4) * locals.var_q_temp3) + (assign14580_e14143 * locals.var_q_temp3_dn4)), (((assign14580_e14141 * locals.var_q_d1_qsq_dn6) * locals.var_q_temp3) + (assign14580_e14143 * locals.var_q_temp3_dn6)), (((assign14580_e14141 * locals.var_q_d1_qsq_dn7) * locals.var_q_temp3) + (assign14580_e14143 * locals.var_q_temp3_dn7)), (((assign14580_e14141 * locals.var_q_d1_qsq_dn8) * locals.var_q_temp3) + (assign14580_e14143 * locals.var_q_temp3_dn8)), (((assign14580_e14141 * locals.var_q_d1_qsq_dn9) * locals.var_q_temp3) + (assign14580_e14143 * locals.var_q_temp3_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign14580_e14147;
        locals.var_q_d1_ln_dn4 = assign14580_e14147_d_n4;
        locals.var_q_d1_ln_dn6 = assign14580_e14147_d_n6;
        locals.var_q_d1_ln_dn7 = assign14580_e14147_d_n7;
        locals.var_q_d1_ln_dn8 = assign14580_e14147_d_n8;
        locals.var_q_d1_ln_dn9 = assign14580_e14147_d_n9;

        let (assign14590_e14184, assign14590_e14184_d_n4, assign14590_e14184_d_n6, assign14590_e14184_d_n7, assign14590_e14184_d_n8, assign14590_e14184_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 == 0.0)) {
        let assign14590_e14158: f64 = (-0.5);
        let assign14590_e14160: f64 = (assign14590_e14158 * locals.var_q_d2_qsq);
        let assign14590_e14162: f64 = (assign14590_e14160 * locals.var_q_temp3);
        let assign14590_e14165: f64 = (0.25 * 0.0055555555556);
        let assign14590_e14167: f64 = (assign14590_e14165 * locals.var_q_d1_qsq);
        let assign14590_e14169: f64 = (assign14590_e14167 * locals.var_q_d1_qsq);
        let assign14590_e14173: f64 = (locals.var_q_qsq * 0.0238095238095);
        let assign14590_e14177: f64 = (0.075 * locals.var_q_qsq);
        let assign14590_e14178: f64 = (2.0 - assign14590_e14177);
        let assign14590_e14179: f64 = (assign14590_e14173 * assign14590_e14178);
        let assign14590_e14180: f64 = (1.0 - assign14590_e14179);
        let assign14590_e14181: f64 = (assign14590_e14169 * assign14590_e14180);
        let assign14590_e14182: f64 = (assign14590_e14162 + assign14590_e14181);
        (assign14590_e14182, ((((assign14590_e14158 * locals.var_q_d2_qsq_dn4) * locals.var_q_temp3) + (assign14590_e14160 * locals.var_q_temp3_dn4)) + (((((assign14590_e14165 * locals.var_q_d1_qsq_dn4) * locals.var_q_d1_qsq) + (assign14590_e14167 * locals.var_q_d1_qsq_dn4)) * assign14590_e14180) + (assign14590_e14169 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign14590_e14178) + (assign14590_e14173 * (-(0.075 * locals.var_q_qsq_dn4)))))))), ((((assign14590_e14158 * locals.var_q_d2_qsq_dn6) * locals.var_q_temp3) + (assign14590_e14160 * locals.var_q_temp3_dn6)) + (((((assign14590_e14165 * locals.var_q_d1_qsq_dn6) * locals.var_q_d1_qsq) + (assign14590_e14167 * locals.var_q_d1_qsq_dn6)) * assign14590_e14180) + (assign14590_e14169 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign14590_e14178) + (assign14590_e14173 * (-(0.075 * locals.var_q_qsq_dn6)))))))), ((((assign14590_e14158 * locals.var_q_d2_qsq_dn7) * locals.var_q_temp3) + (assign14590_e14160 * locals.var_q_temp3_dn7)) + (((((assign14590_e14165 * locals.var_q_d1_qsq_dn7) * locals.var_q_d1_qsq) + (assign14590_e14167 * locals.var_q_d1_qsq_dn7)) * assign14590_e14180) + (assign14590_e14169 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign14590_e14178) + (assign14590_e14173 * (-(0.075 * locals.var_q_qsq_dn7)))))))), ((((assign14590_e14158 * locals.var_q_d2_qsq_dn8) * locals.var_q_temp3) + (assign14590_e14160 * locals.var_q_temp3_dn8)) + (((((assign14590_e14165 * locals.var_q_d1_qsq_dn8) * locals.var_q_d1_qsq) + (assign14590_e14167 * locals.var_q_d1_qsq_dn8)) * assign14590_e14180) + (assign14590_e14169 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign14590_e14178) + (assign14590_e14173 * (-(0.075 * locals.var_q_qsq_dn8)))))))), ((((assign14590_e14158 * locals.var_q_d2_qsq_dn9) * locals.var_q_temp3) + (assign14590_e14160 * locals.var_q_temp3_dn9)) + (((((assign14590_e14165 * locals.var_q_d1_qsq_dn9) * locals.var_q_d1_qsq) + (assign14590_e14167 * locals.var_q_d1_qsq_dn9)) * assign14590_e14180) + (assign14590_e14169 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign14590_e14178) + (assign14590_e14173 * (-(0.075 * locals.var_q_qsq_dn9)))))))),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign14590_e14184;
        locals.var_q_d2_ln_dn4 = assign14590_e14184_d_n4;
        locals.var_q_d2_ln_dn6 = assign14590_e14184_d_n6;
        locals.var_q_d2_ln_dn7 = assign14590_e14184_d_n7;
        locals.var_q_d2_ln_dn8 = assign14590_e14184_d_n8;
        locals.var_q_d2_ln_dn9 = assign14590_e14184_d_n9;

        let assign14600_e14187: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard584 = assign14600_e14187;

        let (assign14610_e14205, assign14610_e14205_d_n4, assign14610_e14205_d_n6, assign14610_e14205_d_n7, assign14610_e14205_d_n8, assign14610_e14205_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard584 != 0.0)) {
        let assign14610_e14195: f64 = (4.0 * locals.var_q_qsq);
        let assign14610_e14200: f64 = (2.0 - locals.var_q_invexpq);
        let assign14610_e14201: f64 = (locals.var_q_invexpq * assign14610_e14200);
        let assign14610_e14202: f64 = (1.0 - assign14610_e14201);
        let assign14610_e14203: f64 = (assign14610_e14195 / assign14610_e14202);
        (assign14610_e14203, ((((4.0 * locals.var_q_qsq_dn4) * assign14610_e14202) - (assign14610_e14195 * (-((locals.var_q_invexpq_dn4 * assign14610_e14200) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn4)))))) / (assign14610_e14202 * assign14610_e14202)), ((((4.0 * locals.var_q_qsq_dn6) * assign14610_e14202) - (assign14610_e14195 * (-((locals.var_q_invexpq_dn6 * assign14610_e14200) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn6)))))) / (assign14610_e14202 * assign14610_e14202)), ((((4.0 * locals.var_q_qsq_dn7) * assign14610_e14202) - (assign14610_e14195 * (-((locals.var_q_invexpq_dn7 * assign14610_e14200) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn7)))))) / (assign14610_e14202 * assign14610_e14202)), ((((4.0 * locals.var_q_qsq_dn8) * assign14610_e14202) - (assign14610_e14195 * (-((locals.var_q_invexpq_dn8 * assign14610_e14200) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn8)))))) / (assign14610_e14202 * assign14610_e14202)), ((((4.0 * locals.var_q_qsq_dn9) * assign14610_e14202) - (assign14610_e14195 * (-((locals.var_q_invexpq_dn9 * assign14610_e14200) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn9)))))) / (assign14610_e14202 * assign14610_e14202)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign14610_e14205;
        locals.var_q_temp2_dn4 = assign14610_e14205_d_n4;
        locals.var_q_temp2_dn6 = assign14610_e14205_d_n6;
        locals.var_q_temp2_dn7 = assign14610_e14205_d_n7;
        locals.var_q_temp2_dn8 = assign14610_e14205_d_n8;
        locals.var_q_temp2_dn9 = assign14610_e14205_d_n9;

        let (assign14620_e14215, assign14620_e14215_d_n4, assign14620_e14215_d_n6, assign14620_e14215_d_n7, assign14620_e14215_d_n8, assign14620_e14215_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard584 != 0.0)) {
        let assign14620_e14213: f64 = (locals.var_q_temp2 * locals.var_q_invexpq);
        (assign14620_e14213, ((locals.var_q_temp2_dn4 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn4)), ((locals.var_q_temp2_dn6 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn6)), ((locals.var_q_temp2_dn7 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn7)), ((locals.var_q_temp2_dn8 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn8)), ((locals.var_q_temp2_dn9 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn9)),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign14620_e14215;
        locals.var_q_sh_term_dn4 = assign14620_e14215_d_n4;
        locals.var_q_sh_term_dn6 = assign14620_e14215_d_n6;
        locals.var_q_sh_term_dn7 = assign14620_e14215_d_n7;
        locals.var_q_sh_term_dn8 = assign14620_e14215_d_n8;
        locals.var_q_sh_term_dn9 = assign14620_e14215_d_n9;

        let (assign14630_e14226, assign14630_e14226_d_n4, assign14630_e14226_d_n6, assign14630_e14226_d_n7, assign14630_e14226_d_n8, assign14630_e14226_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard584 != 0.0)) {
        let assign14630_e14222: f64 = (locals.var_q_temp2).ln();
        let assign14630_e14224: f64 = (assign14630_e14222 - locals.var_q_rac_qsq);
        (assign14630_e14224, ((locals.var_q_temp2_dn4 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn4), ((locals.var_q_temp2_dn6 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn6), ((locals.var_q_temp2_dn7 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn7), ((locals.var_q_temp2_dn8 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn8), ((locals.var_q_temp2_dn9 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn9),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign14630_e14226;
        locals.var_q_ln_term_dn4 = assign14630_e14226_d_n4;
        locals.var_q_ln_term_dn6 = assign14630_e14226_d_n6;
        locals.var_q_ln_term_dn7 = assign14630_e14226_d_n7;
        locals.var_q_ln_term_dn8 = assign14630_e14226_d_n8;
        locals.var_q_ln_term_dn9 = assign14630_e14226_d_n9;

        let assign14640_e14229: f64 = (-0.005);
        let assign14640_e14230: f64 = if locals.var_q_qsq < assign14640_e14229 { 1.0 } else { 0.0 };
        locals.var_guard585 = assign14640_e14230;

        let (assign14650_e14244, assign14650_e14244_d_n4, assign14650_e14244_d_n6, assign14650_e14244_d_n7, assign14650_e14244_d_n8, assign14650_e14244_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard584 == 0.0)) && (locals.var_guard585 != 0.0)) {
        let assign14650_e14241: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign14650_e14242: f64 = (assign14650_e14241).sin();
        (assign14650_e14242, ((assign14650_e14241).cos() * (0.5 * locals.var_q_rac_qsq_dn4)), ((assign14650_e14241).cos() * (0.5 * locals.var_q_rac_qsq_dn6)), ((assign14650_e14241).cos() * (0.5 * locals.var_q_rac_qsq_dn7)), ((assign14650_e14241).cos() * (0.5 * locals.var_q_rac_qsq_dn8)), ((assign14650_e14241).cos() * (0.5 * locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign14650_e14244;
        locals.var_q_temp2_dn4 = assign14650_e14244_d_n4;
        locals.var_q_temp2_dn6 = assign14650_e14244_d_n6;
        locals.var_q_temp2_dn7 = assign14650_e14244_d_n7;
        locals.var_q_temp2_dn8 = assign14650_e14244_d_n8;
        locals.var_q_temp2_dn9 = assign14650_e14244_d_n9;

        let (assign14660_e14260, assign14660_e14260_d_n4, assign14660_e14260_d_n6, assign14660_e14260_d_n7, assign14660_e14260_d_n8, assign14660_e14260_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard584 == 0.0)) && (locals.var_guard585 != 0.0)) {
        let assign14660_e14254: f64 = (-locals.var_q_qsq);
        let assign14660_e14257: f64 = (locals.var_q_temp2 * locals.var_q_temp2);
        let assign14660_e14258: f64 = (assign14660_e14254 / assign14660_e14257);
        (assign14660_e14258, ((((-locals.var_q_qsq_dn4) * assign14660_e14257) - (assign14660_e14254 * ((locals.var_q_temp2_dn4 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn4)))) / (assign14660_e14257 * assign14660_e14257)), ((((-locals.var_q_qsq_dn6) * assign14660_e14257) - (assign14660_e14254 * ((locals.var_q_temp2_dn6 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn6)))) / (assign14660_e14257 * assign14660_e14257)), ((((-locals.var_q_qsq_dn7) * assign14660_e14257) - (assign14660_e14254 * ((locals.var_q_temp2_dn7 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn7)))) / (assign14660_e14257 * assign14660_e14257)), ((((-locals.var_q_qsq_dn8) * assign14660_e14257) - (assign14660_e14254 * ((locals.var_q_temp2_dn8 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn8)))) / (assign14660_e14257 * assign14660_e14257)), ((((-locals.var_q_qsq_dn9) * assign14660_e14257) - (assign14660_e14254 * ((locals.var_q_temp2_dn9 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn9)))) / (assign14660_e14257 * assign14660_e14257)),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign14660_e14260;
        locals.var_q_sh_term_dn4 = assign14660_e14260_d_n4;
        locals.var_q_sh_term_dn6 = assign14660_e14260_d_n6;
        locals.var_q_sh_term_dn7 = assign14660_e14260_d_n7;
        locals.var_q_sh_term_dn8 = assign14660_e14260_d_n8;
        locals.var_q_sh_term_dn9 = assign14660_e14260_d_n9;

        let (assign14670_e14272, assign14670_e14272_d_n4, assign14670_e14272_d_n6, assign14670_e14272_d_n7, assign14670_e14272_d_n8, assign14670_e14272_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard584 == 0.0)) && (locals.var_guard585 != 0.0)) {
        let assign14670_e14270: f64 = (locals.var_q_sh_term).ln();
        (assign14670_e14270, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign14670_e14272;
        locals.var_q_ln_term_dn4 = assign14670_e14272_d_n4;
        locals.var_q_ln_term_dn6 = assign14670_e14272_d_n6;
        locals.var_q_ln_term_dn7 = assign14670_e14272_d_n7;
        locals.var_q_ln_term_dn8 = assign14670_e14272_d_n8;
        locals.var_q_ln_term_dn9 = assign14670_e14272_d_n9;

    }

    pub(super) fn stamp_transient_block_36(
        locals: &mut StampLocals,
    ) {
        let (assign14680_e14300, assign14680_e14300_d_n4, assign14680_e14300_d_n6, assign14680_e14300_d_n7, assign14680_e14300_d_n8, assign14680_e14300_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard584 == 0.0)) && (locals.var_guard585 == 0.0)) {
        let assign14680_e14285: f64 = (locals.var_q_qsq * 0.3333333333333);
        let assign14680_e14289: f64 = (0.05 * locals.var_q_qsq);
        let assign14680_e14293: f64 = (0.0396825396825397 * locals.var_q_qsq);
        let assign14680_e14294: f64 = (1.0 - assign14680_e14293);
        let assign14680_e14295: f64 = (assign14680_e14289 * assign14680_e14294);
        let assign14680_e14296: f64 = (1.0 - assign14680_e14295);
        let assign14680_e14297: f64 = (assign14680_e14285 * assign14680_e14296);
        let assign14680_e14298: f64 = (4.0 - assign14680_e14297);
        (assign14680_e14298, (-(((locals.var_q_qsq_dn4 * 0.3333333333333) * assign14680_e14296) + (assign14680_e14285 * (-(((0.05 * locals.var_q_qsq_dn4) * assign14680_e14294) + (assign14680_e14289 * (-(0.0396825396825397 * locals.var_q_qsq_dn4)))))))), (-(((locals.var_q_qsq_dn6 * 0.3333333333333) * assign14680_e14296) + (assign14680_e14285 * (-(((0.05 * locals.var_q_qsq_dn6) * assign14680_e14294) + (assign14680_e14289 * (-(0.0396825396825397 * locals.var_q_qsq_dn6)))))))), (-(((locals.var_q_qsq_dn7 * 0.3333333333333) * assign14680_e14296) + (assign14680_e14285 * (-(((0.05 * locals.var_q_qsq_dn7) * assign14680_e14294) + (assign14680_e14289 * (-(0.0396825396825397 * locals.var_q_qsq_dn7)))))))), (-(((locals.var_q_qsq_dn8 * 0.3333333333333) * assign14680_e14296) + (assign14680_e14285 * (-(((0.05 * locals.var_q_qsq_dn8) * assign14680_e14294) + (assign14680_e14289 * (-(0.0396825396825397 * locals.var_q_qsq_dn8)))))))), (-(((locals.var_q_qsq_dn9 * 0.3333333333333) * assign14680_e14296) + (assign14680_e14285 * (-(((0.05 * locals.var_q_qsq_dn9) * assign14680_e14294) + (assign14680_e14289 * (-(0.0396825396825397 * locals.var_q_qsq_dn9)))))))),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign14680_e14300;
        locals.var_q_sh_term_dn4 = assign14680_e14300_d_n4;
        locals.var_q_sh_term_dn6 = assign14680_e14300_d_n6;
        locals.var_q_sh_term_dn7 = assign14680_e14300_d_n7;
        locals.var_q_sh_term_dn8 = assign14680_e14300_d_n8;
        locals.var_q_sh_term_dn9 = assign14680_e14300_d_n9;

        let (assign14690_e14313, assign14690_e14313_d_n4, assign14690_e14313_d_n6, assign14690_e14313_d_n7, assign14690_e14313_d_n8, assign14690_e14313_d_n9,) = {
    if ((((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard584 == 0.0)) && (locals.var_guard585 == 0.0)) {
        let assign14690_e14311: f64 = (locals.var_q_sh_term).ln();
        (assign14690_e14311, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign14690_e14313;
        locals.var_q_ln_term_dn4 = assign14690_e14313_d_n4;
        locals.var_q_ln_term_dn6 = assign14690_e14313_d_n6;
        locals.var_q_ln_term_dn7 = assign14690_e14313_d_n7;
        locals.var_q_ln_term_dn8 = assign14690_e14313_d_n8;
        locals.var_q_ln_term_dn9 = assign14690_e14313_d_n9;

        let assign14700_e14316: f64 = (1.01 * locals.var_q_k1q1);
        let assign14700_e14318: f64 = (assign14700_e14316 + locals.var_q_qcoth);
        let assign14700_e14320: f64 = if assign14700_e14318 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard586 = assign14700_e14320;

        let (assign14710_e14330, assign14710_e14330_d_n4, assign14710_e14330_d_n6, assign14710_e14330_d_n7, assign14710_e14330_d_n8, assign14710_e14330_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard586 != 0.0)) {
        let assign14710_e14328: f64 = (locals.var_q_k1q1 + locals.var_q_qcoth);
        (assign14710_e14328, (locals.var_q_k1q1_dn4 + locals.var_q_qcoth_dn4), (locals.var_q_k1q1_dn6 + locals.var_q_qcoth_dn6), (locals.var_q_k1q1_dn7 + locals.var_q_qcoth_dn7), (locals.var_q_k1q1_dn8 + locals.var_q_qcoth_dn8), (locals.var_q_k1q1_dn9 + locals.var_q_qcoth_dn9),)
    } else {
        (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9,)
    }
};
        locals.var_q_expnum = assign14710_e14330;
        locals.var_q_expnum_dn4 = assign14710_e14330_d_n4;
        locals.var_q_expnum_dn6 = assign14710_e14330_d_n6;
        locals.var_q_expnum_dn7 = assign14710_e14330_d_n7;
        locals.var_q_expnum_dn8 = assign14710_e14330_d_n8;
        locals.var_q_expnum_dn9 = assign14710_e14330_d_n9;

        let (assign14720_e14340, assign14720_e14340_d_n4, assign14720_e14340_d_n6, assign14720_e14340_d_n7, assign14720_e14340_d_n8, assign14720_e14340_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard586 != 0.0)) {
        let assign14720_e14338: f64 = (locals.var_k1 + locals.var_q_d1_qcoth);
        (assign14720_e14338, (locals.var_k1_dn4 + locals.var_q_d1_qcoth_dn4), (locals.var_k1_dn6 + locals.var_q_d1_qcoth_dn6), (locals.var_k1_dn7 + locals.var_q_d1_qcoth_dn7), (locals.var_k1_dn8 + locals.var_q_d1_qcoth_dn8), (locals.var_k1_dn9 + locals.var_q_d1_qcoth_dn9),)
    } else {
        (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9,)
    }
};
        locals.var_q_d1_expnum = assign14720_e14340;
        locals.var_q_d1_expnum_dn4 = assign14720_e14340_d_n4;
        locals.var_q_d1_expnum_dn6 = assign14720_e14340_d_n6;
        locals.var_q_d1_expnum_dn7 = assign14720_e14340_d_n7;
        locals.var_q_d1_expnum_dn8 = assign14720_e14340_d_n8;
        locals.var_q_d1_expnum_dn9 = assign14720_e14340_d_n9;

        let (assign14730_e14348, assign14730_e14348_d_n4, assign14730_e14348_d_n6, assign14730_e14348_d_n7, assign14730_e14348_d_n8, assign14730_e14348_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard586 != 0.0)) {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    } else {
        (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9,)
    }
};
        locals.var_q_d2_expnum = assign14730_e14348;
        locals.var_q_d2_expnum_dn4 = assign14730_e14348_d_n4;
        locals.var_q_d2_expnum_dn6 = assign14730_e14348_d_n6;
        locals.var_q_d2_expnum_dn7 = assign14730_e14348_d_n7;
        locals.var_q_d2_expnum_dn8 = assign14730_e14348_d_n8;
        locals.var_q_d2_expnum_dn9 = assign14730_e14348_d_n9;

        let (assign14740_e14361, assign14740_e14361_d_n4, assign14740_e14361_d_n6, assign14740_e14361_d_n7, assign14740_e14361_d_n8, assign14740_e14361_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard586 == 0.0)) {
        let assign14740_e14358: f64 = (locals.var_q_k1q1 - locals.var_q_qcoth);
        let assign14740_e14359: f64 = (1.0 / assign14740_e14358);
        (assign14740_e14359, (-((locals.var_q_k1q1_dn4 - locals.var_q_qcoth_dn4) / (assign14740_e14358 * assign14740_e14358))), (-((locals.var_q_k1q1_dn6 - locals.var_q_qcoth_dn6) / (assign14740_e14358 * assign14740_e14358))), (-((locals.var_q_k1q1_dn7 - locals.var_q_qcoth_dn7) / (assign14740_e14358 * assign14740_e14358))), (-((locals.var_q_k1q1_dn8 - locals.var_q_qcoth_dn8) / (assign14740_e14358 * assign14740_e14358))), (-((locals.var_q_k1q1_dn9 - locals.var_q_qcoth_dn9) / (assign14740_e14358 * assign14740_e14358))),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign14740_e14361;
        locals.var_q_temp2_dn4 = assign14740_e14361_d_n4;
        locals.var_q_temp2_dn6 = assign14740_e14361_d_n6;
        locals.var_q_temp2_dn7 = assign14740_e14361_d_n7;
        locals.var_q_temp2_dn8 = assign14740_e14361_d_n8;
        locals.var_q_temp2_dn9 = assign14740_e14361_d_n9;

        let (assign14750_e14372, assign14750_e14372_d_n4, assign14750_e14372_d_n6, assign14750_e14372_d_n7, assign14750_e14372_d_n8, assign14750_e14372_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard586 == 0.0)) {
        let assign14750_e14370: f64 = (locals.var_q_d1_qcoth - locals.var_k1);
        (assign14750_e14370, (locals.var_q_d1_qcoth_dn4 - locals.var_k1_dn4), (locals.var_q_d1_qcoth_dn6 - locals.var_k1_dn6), (locals.var_q_d1_qcoth_dn7 - locals.var_k1_dn7), (locals.var_q_d1_qcoth_dn8 - locals.var_k1_dn8), (locals.var_q_d1_qcoth_dn9 - locals.var_k1_dn9),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign14750_e14372;
        locals.var_q_temp3_dn4 = assign14750_e14372_d_n4;
        locals.var_q_temp3_dn6 = assign14750_e14372_d_n6;
        locals.var_q_temp3_dn7 = assign14750_e14372_d_n7;
        locals.var_q_temp3_dn8 = assign14750_e14372_d_n8;
        locals.var_q_temp3_dn9 = assign14750_e14372_d_n9;

        let (assign14760_e14385, assign14760_e14385_d_n4, assign14760_e14385_d_n6, assign14760_e14385_d_n7, assign14760_e14385_d_n8, assign14760_e14385_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard586 == 0.0)) {
        let assign14760_e14381: f64 = (locals.var_q_aexp - locals.var_q_sh_term);
        let assign14760_e14383: f64 = (assign14760_e14381 * locals.var_q_temp2);
        (assign14760_e14383, (((locals.var_q_aexp_dn4 - locals.var_q_sh_term_dn4) * locals.var_q_temp2) + (assign14760_e14381 * locals.var_q_temp2_dn4)), (((locals.var_q_aexp_dn6 - locals.var_q_sh_term_dn6) * locals.var_q_temp2) + (assign14760_e14381 * locals.var_q_temp2_dn6)), (((locals.var_q_aexp_dn7 - locals.var_q_sh_term_dn7) * locals.var_q_temp2) + (assign14760_e14381 * locals.var_q_temp2_dn7)), (((locals.var_q_aexp_dn8 - locals.var_q_sh_term_dn8) * locals.var_q_temp2) + (assign14760_e14381 * locals.var_q_temp2_dn8)), (((locals.var_q_aexp_dn9 - locals.var_q_sh_term_dn9) * locals.var_q_temp2) + (assign14760_e14381 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9,)
    }
};
        locals.var_q_expnum = assign14760_e14385;
        locals.var_q_expnum_dn4 = assign14760_e14385_d_n4;
        locals.var_q_expnum_dn6 = assign14760_e14385_d_n6;
        locals.var_q_expnum_dn7 = assign14760_e14385_d_n7;
        locals.var_q_expnum_dn8 = assign14760_e14385_d_n8;
        locals.var_q_expnum_dn9 = assign14760_e14385_d_n9;

        let (assign14770_e14404, assign14770_e14404_d_n4, assign14770_e14404_d_n6, assign14770_e14404_d_n7, assign14770_e14404_d_n8, assign14770_e14404_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard586 == 0.0)) {
        let assign14770_e14394: f64 = (locals.var_q_temp3 * locals.var_q_expnum);
        let assign14770_e14396: f64 = (assign14770_e14394 - locals.var_q_aexp);
        let assign14770_e14399: f64 = (locals.var_q_d1_ln * locals.var_q_sh_term);
        let assign14770_e14400: f64 = (assign14770_e14396 - assign14770_e14399);
        let assign14770_e14402: f64 = (assign14770_e14400 * locals.var_q_temp2);
        (assign14770_e14402, ((((((locals.var_q_temp3_dn4 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4) - ((locals.var_q_d1_ln_dn4 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign14770_e14400 * locals.var_q_temp2_dn4)), ((((((locals.var_q_temp3_dn6 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6) - ((locals.var_q_d1_ln_dn6 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign14770_e14400 * locals.var_q_temp2_dn6)), ((((((locals.var_q_temp3_dn7 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7) - ((locals.var_q_d1_ln_dn7 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign14770_e14400 * locals.var_q_temp2_dn7)), ((((((locals.var_q_temp3_dn8 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8) - ((locals.var_q_d1_ln_dn8 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign14770_e14400 * locals.var_q_temp2_dn8)), ((((((locals.var_q_temp3_dn9 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9) - ((locals.var_q_d1_ln_dn9 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign14770_e14400 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9,)
    }
};
        locals.var_q_d1_expnum = assign14770_e14404;
        locals.var_q_d1_expnum_dn4 = assign14770_e14404_d_n4;
        locals.var_q_d1_expnum_dn6 = assign14770_e14404_d_n6;
        locals.var_q_d1_expnum_dn7 = assign14770_e14404_d_n7;
        locals.var_q_d1_expnum_dn8 = assign14770_e14404_d_n8;
        locals.var_q_d1_expnum_dn9 = assign14770_e14404_d_n9;

        let (assign14780_e14433, assign14780_e14433_d_n4, assign14780_e14433_d_n6, assign14780_e14433_d_n7, assign14780_e14433_d_n8, assign14780_e14433_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard586 == 0.0)) {
        let assign14780_e14413: f64 = (locals.var_q_d2_qcoth * locals.var_q_expnum);
        let assign14780_e14416: f64 = (2.0 * locals.var_q_temp3);
        let assign14780_e14418: f64 = (assign14780_e14416 * locals.var_q_d1_expnum);
        let assign14780_e14419: f64 = (assign14780_e14413 + assign14780_e14418);
        let assign14780_e14421: f64 = (assign14780_e14419 + locals.var_q_aexp);
        let assign14780_e14425: f64 = (locals.var_q_d1_ln * locals.var_q_d1_ln);
        let assign14780_e14426: f64 = (locals.var_q_d2_ln + assign14780_e14425);
        let assign14780_e14428: f64 = (assign14780_e14426 * locals.var_q_sh_term);
        let assign14780_e14429: f64 = (assign14780_e14421 - assign14780_e14428);
        let assign14780_e14431: f64 = (assign14780_e14429 * locals.var_q_temp2);
        (assign14780_e14431, (((((((locals.var_q_d2_qcoth_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_temp3_dn4) * locals.var_q_d1_expnum) + (assign14780_e14416 * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4) - (((locals.var_q_d2_ln_dn4 + ((locals.var_q_d1_ln_dn4 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn4))) * locals.var_q_sh_term) + (assign14780_e14426 * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign14780_e14429 * locals.var_q_temp2_dn4)), (((((((locals.var_q_d2_qcoth_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_temp3_dn6) * locals.var_q_d1_expnum) + (assign14780_e14416 * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6) - (((locals.var_q_d2_ln_dn6 + ((locals.var_q_d1_ln_dn6 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn6))) * locals.var_q_sh_term) + (assign14780_e14426 * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign14780_e14429 * locals.var_q_temp2_dn6)), (((((((locals.var_q_d2_qcoth_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_temp3_dn7) * locals.var_q_d1_expnum) + (assign14780_e14416 * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7) - (((locals.var_q_d2_ln_dn7 + ((locals.var_q_d1_ln_dn7 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn7))) * locals.var_q_sh_term) + (assign14780_e14426 * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign14780_e14429 * locals.var_q_temp2_dn7)), (((((((locals.var_q_d2_qcoth_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_temp3_dn8) * locals.var_q_d1_expnum) + (assign14780_e14416 * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8) - (((locals.var_q_d2_ln_dn8 + ((locals.var_q_d1_ln_dn8 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn8))) * locals.var_q_sh_term) + (assign14780_e14426 * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign14780_e14429 * locals.var_q_temp2_dn8)), (((((((locals.var_q_d2_qcoth_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_temp3_dn9) * locals.var_q_d1_expnum) + (assign14780_e14416 * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9) - (((locals.var_q_d2_ln_dn9 + ((locals.var_q_d1_ln_dn9 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn9))) * locals.var_q_sh_term) + (assign14780_e14426 * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign14780_e14429 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9,)
    }
};
        locals.var_q_d2_expnum = assign14780_e14433;
        locals.var_q_d2_expnum_dn4 = assign14780_e14433_d_n4;
        locals.var_q_d2_expnum_dn6 = assign14780_e14433_d_n6;
        locals.var_q_d2_expnum_dn7 = assign14780_e14433_d_n7;
        locals.var_q_d2_expnum_dn8 = assign14780_e14433_d_n8;
        locals.var_q_d2_expnum_dn9 = assign14780_e14433_d_n9;

        let assign14790_e14436: f64 = if locals.var_q_expnum > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard587 = assign14790_e14436;

        let (assign14800_e14445, assign14800_e14445_d_n4, assign14800_e14445_d_n6, assign14800_e14445_d_n7, assign14800_e14445_d_n8, assign14800_e14445_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard587 != 0.0)) {
        let assign14800_e14443: f64 = (locals.var_q_expnum).ln();
        (assign14800_e14443, (locals.var_q_expnum_dn4 / locals.var_q_expnum), (locals.var_q_expnum_dn6 / locals.var_q_expnum), (locals.var_q_expnum_dn7 / locals.var_q_expnum), (locals.var_q_expnum_dn8 / locals.var_q_expnum), (locals.var_q_expnum_dn9 / locals.var_q_expnum),)
    } else {
        (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9,)
    }
};
        locals.var_q_lnexpnum = assign14800_e14445;
        locals.var_q_lnexpnum_dn4 = assign14800_e14445_d_n4;
        locals.var_q_lnexpnum_dn6 = assign14800_e14445_d_n6;
        locals.var_q_lnexpnum_dn7 = assign14800_e14445_d_n7;
        locals.var_q_lnexpnum_dn8 = assign14800_e14445_d_n8;
        locals.var_q_lnexpnum_dn9 = assign14800_e14445_d_n9;

        let (assign14810_e14455, assign14810_e14455_d_n4, assign14810_e14455_d_n6, assign14810_e14455_d_n7, assign14810_e14455_d_n8, assign14810_e14455_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard587 != 0.0)) {
        let assign14810_e14453: f64 = (1.0 / locals.var_q_expnum);
        (assign14810_e14453, (-(locals.var_q_expnum_dn4 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn6 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn7 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn8 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn9 / (locals.var_q_expnum * locals.var_q_expnum))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign14810_e14455;
        locals.var_q_temp1_dn4 = assign14810_e14455_d_n4;
        locals.var_q_temp1_dn6 = assign14810_e14455_d_n6;
        locals.var_q_temp1_dn7 = assign14810_e14455_d_n7;
        locals.var_q_temp1_dn8 = assign14810_e14455_d_n8;
        locals.var_q_temp1_dn9 = assign14810_e14455_d_n9;

        let (assign14820_e14465, assign14820_e14465_d_n4, assign14820_e14465_d_n6, assign14820_e14465_d_n7, assign14820_e14465_d_n8, assign14820_e14465_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard587 != 0.0)) {
        let assign14820_e14463: f64 = (locals.var_q_d1_expnum * locals.var_q_temp1);
        (assign14820_e14463, ((locals.var_q_d1_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn4)), ((locals.var_q_d1_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn6)), ((locals.var_q_d1_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn7)), ((locals.var_q_d1_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn8)), ((locals.var_q_d1_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9,)
    }
};
        locals.var_q_d1_lnexpnum = assign14820_e14465;
        locals.var_q_d1_lnexpnum_dn4 = assign14820_e14465_d_n4;
        locals.var_q_d1_lnexpnum_dn6 = assign14820_e14465_d_n6;
        locals.var_q_d1_lnexpnum_dn7 = assign14820_e14465_d_n7;
        locals.var_q_d1_lnexpnum_dn8 = assign14820_e14465_d_n8;
        locals.var_q_d1_lnexpnum_dn9 = assign14820_e14465_d_n9;

        let (assign14830_e14479, assign14830_e14479_d_n4, assign14830_e14479_d_n6, assign14830_e14479_d_n7, assign14830_e14479_d_n8, assign14830_e14479_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard587 != 0.0)) {
        let assign14830_e14473: f64 = (locals.var_q_d2_expnum * locals.var_q_temp1);
        let assign14830_e14476: f64 = (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum);
        let assign14830_e14477: f64 = (assign14830_e14473 - assign14830_e14476);
        (assign14830_e14477, (((locals.var_q_d2_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn4)) - ((locals.var_q_d1_lnexpnum_dn4 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn4))), (((locals.var_q_d2_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn6)) - ((locals.var_q_d1_lnexpnum_dn6 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn6))), (((locals.var_q_d2_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn7)) - ((locals.var_q_d1_lnexpnum_dn7 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn7))), (((locals.var_q_d2_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn8)) - ((locals.var_q_d1_lnexpnum_dn8 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn8))), (((locals.var_q_d2_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn9)) - ((locals.var_q_d1_lnexpnum_dn9 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn9))),)
    } else {
        (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9,)
    }
};
        locals.var_q_d2_lnexpnum = assign14830_e14479;
        locals.var_q_d2_lnexpnum_dn4 = assign14830_e14479_d_n4;
        locals.var_q_d2_lnexpnum_dn6 = assign14830_e14479_d_n6;
        locals.var_q_d2_lnexpnum_dn7 = assign14830_e14479_d_n7;
        locals.var_q_d2_lnexpnum_dn8 = assign14830_e14479_d_n8;
        locals.var_q_d2_lnexpnum_dn9 = assign14830_e14479_d_n9;

        let (assign14840_e14494, assign14840_e14494_d_n4, assign14840_e14494_d_n6, assign14840_e14494_d_n7, assign14840_e14494_d_n8, assign14840_e14494_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard587 == 0.0)) {
        let assign14840_e14488: f64 = (locals.var_q_k1q1 + 0.6931471805599);
        let assign14840_e14490: f64 = (-locals.var_q_k1q1);
        let assign14840_e14491: f64 = (assign14840_e14490).ln();
        let assign14840_e14492: f64 = (assign14840_e14488 + assign14840_e14491);
        (assign14840_e14492, (locals.var_q_k1q1_dn4 + ((-locals.var_q_k1q1_dn4) / assign14840_e14490)), (locals.var_q_k1q1_dn6 + ((-locals.var_q_k1q1_dn6) / assign14840_e14490)), (locals.var_q_k1q1_dn7 + ((-locals.var_q_k1q1_dn7) / assign14840_e14490)), (locals.var_q_k1q1_dn8 + ((-locals.var_q_k1q1_dn8) / assign14840_e14490)), (locals.var_q_k1q1_dn9 + ((-locals.var_q_k1q1_dn9) / assign14840_e14490)),)
    } else {
        (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9,)
    }
};
        locals.var_q_lnexpnum = assign14840_e14494;
        locals.var_q_lnexpnum_dn4 = assign14840_e14494_d_n4;
        locals.var_q_lnexpnum_dn6 = assign14840_e14494_d_n6;
        locals.var_q_lnexpnum_dn7 = assign14840_e14494_d_n7;
        locals.var_q_lnexpnum_dn8 = assign14840_e14494_d_n8;
        locals.var_q_lnexpnum_dn9 = assign14840_e14494_d_n9;

        let (assign14850_e14505, assign14850_e14505_d_n4, assign14850_e14505_d_n6, assign14850_e14505_d_n7, assign14850_e14505_d_n8, assign14850_e14505_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard587 == 0.0)) {
        let assign14850_e14503: f64 = (1.0 / locals.var_q1s);
        (assign14850_e14503, (-(locals.var_q1s_dn4 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn6 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn7 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn8 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn9 / (locals.var_q1s * locals.var_q1s))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign14850_e14505;
        locals.var_q_temp1_dn4 = assign14850_e14505_d_n4;
        locals.var_q_temp1_dn6 = assign14850_e14505_d_n6;
        locals.var_q_temp1_dn7 = assign14850_e14505_d_n7;
        locals.var_q_temp1_dn8 = assign14850_e14505_d_n8;
        locals.var_q_temp1_dn9 = assign14850_e14505_d_n9;

        let (assign14860_e14516, assign14860_e14516_d_n4, assign14860_e14516_d_n6, assign14860_e14516_d_n7, assign14860_e14516_d_n8, assign14860_e14516_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard587 == 0.0)) {
        let assign14860_e14514: f64 = (locals.var_k1 + locals.var_q_temp1);
        (assign14860_e14514, (locals.var_k1_dn4 + locals.var_q_temp1_dn4), (locals.var_k1_dn6 + locals.var_q_temp1_dn6), (locals.var_k1_dn7 + locals.var_q_temp1_dn7), (locals.var_k1_dn8 + locals.var_q_temp1_dn8), (locals.var_k1_dn9 + locals.var_q_temp1_dn9),)
    } else {
        (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9,)
    }
};
        locals.var_q_d1_lnexpnum = assign14860_e14516;
        locals.var_q_d1_lnexpnum_dn4 = assign14860_e14516_d_n4;
        locals.var_q_d1_lnexpnum_dn6 = assign14860_e14516_d_n6;
        locals.var_q_d1_lnexpnum_dn7 = assign14860_e14516_d_n7;
        locals.var_q_d1_lnexpnum_dn8 = assign14860_e14516_d_n8;
        locals.var_q_d1_lnexpnum_dn9 = assign14860_e14516_d_n9;

        let (assign14870_e14528, assign14870_e14528_d_n4, assign14870_e14528_d_n6, assign14870_e14528_d_n7, assign14870_e14528_d_n8, assign14870_e14528_d_n9,) = {
    if (((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) && (locals.var_guard587 == 0.0)) {
        let assign14870_e14524: f64 = (-locals.var_q_temp1);
        let assign14870_e14526: f64 = (assign14870_e14524 * locals.var_q_temp1);
        (assign14870_e14526, (((-locals.var_q_temp1_dn4) * locals.var_q_temp1) + (assign14870_e14524 * locals.var_q_temp1_dn4)), (((-locals.var_q_temp1_dn6) * locals.var_q_temp1) + (assign14870_e14524 * locals.var_q_temp1_dn6)), (((-locals.var_q_temp1_dn7) * locals.var_q_temp1) + (assign14870_e14524 * locals.var_q_temp1_dn7)), (((-locals.var_q_temp1_dn8) * locals.var_q_temp1) + (assign14870_e14524 * locals.var_q_temp1_dn8)), (((-locals.var_q_temp1_dn9) * locals.var_q_temp1) + (assign14870_e14524 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9,)
    }
};
        locals.var_q_d2_lnexpnum = assign14870_e14528;
        locals.var_q_d2_lnexpnum_dn4 = assign14870_e14528_d_n4;
        locals.var_q_d2_lnexpnum_dn6 = assign14870_e14528_d_n6;
        locals.var_q_d2_lnexpnum_dn7 = assign14870_e14528_d_n7;
        locals.var_q_d2_lnexpnum_dn8 = assign14870_e14528_d_n8;
        locals.var_q_d2_lnexpnum_dn9 = assign14870_e14528_d_n9;

        let (assign14880_e14544, assign14880_e14544_d_n4, assign14880_e14544_d_n6, assign14880_e14544_d_n7, assign14880_e14544_d_n8, assign14880_e14544_d_n9,) = {
    if ((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) {
        let assign14880_e14534: f64 = (locals.var_xg2x - locals.var_xg1x);
        let assign14880_e14536: f64 = (assign14880_e14534 + locals.var_q1s);
        let assign14880_e14539: f64 = (2.0 * locals.var_q_lnexpnum);
        let assign14880_e14540: f64 = (assign14880_e14536 + assign14880_e14539);
        let assign14880_e14542: f64 = (assign14880_e14540 - locals.var_q_ln_term);
        (assign14880_e14542, ((((locals.var_xg2x_dn4 - locals.var_xg1x_dn4) + locals.var_q1s_dn4) + (2.0 * locals.var_q_lnexpnum_dn4)) - locals.var_q_ln_term_dn4), ((((locals.var_xg2x_dn6 - locals.var_xg1x_dn6) + locals.var_q1s_dn6) + (2.0 * locals.var_q_lnexpnum_dn6)) - locals.var_q_ln_term_dn6), ((((locals.var_xg2x_dn7 - locals.var_xg1x_dn7) + locals.var_q1s_dn7) + (2.0 * locals.var_q_lnexpnum_dn7)) - locals.var_q_ln_term_dn7), ((((locals.var_xg2x_dn8 - locals.var_xg1x_dn8) + locals.var_q1s_dn8) + (2.0 * locals.var_q_lnexpnum_dn8)) - locals.var_q_ln_term_dn8), ((((locals.var_xg2x_dn9 - locals.var_xg1x_dn9) + locals.var_q1s_dn9) + (2.0 * locals.var_q_lnexpnum_dn9)) - locals.var_q_ln_term_dn9),)
    } else {
        (locals.var_q_q2_int, locals.var_q_q2_int_dn4, locals.var_q_q2_int_dn6, locals.var_q_q2_int_dn7, locals.var_q_q2_int_dn8, locals.var_q_q2_int_dn9,)
    }
};
        locals.var_q_q2_int = assign14880_e14544;
        locals.var_q_q2_int_dn4 = assign14880_e14544_d_n4;
        locals.var_q_q2_int_dn6 = assign14880_e14544_d_n6;
        locals.var_q_q2_int_dn7 = assign14880_e14544_d_n7;
        locals.var_q_q2_int_dn8 = assign14880_e14544_d_n8;
        locals.var_q_q2_int_dn9 = assign14880_e14544_d_n9;

        let (assign14890_e14556, assign14890_e14556_d_n4, assign14890_e14556_d_n6, assign14890_e14556_d_n7, assign14890_e14556_d_n8, assign14890_e14556_d_n9,) = {
    if ((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) {
        let assign14890_e14551: f64 = (2.0 * locals.var_q_d1_lnexpnum);
        let assign14890_e14552: f64 = (1.0 + assign14890_e14551);
        let assign14890_e14554: f64 = (assign14890_e14552 - locals.var_q_d1_ln);
        (assign14890_e14554, ((2.0 * locals.var_q_d1_lnexpnum_dn4) - locals.var_q_d1_ln_dn4), ((2.0 * locals.var_q_d1_lnexpnum_dn6) - locals.var_q_d1_ln_dn6), ((2.0 * locals.var_q_d1_lnexpnum_dn7) - locals.var_q_d1_ln_dn7), ((2.0 * locals.var_q_d1_lnexpnum_dn8) - locals.var_q_d1_ln_dn8), ((2.0 * locals.var_q_d1_lnexpnum_dn9) - locals.var_q_d1_ln_dn9),)
    } else {
        (locals.var_q_d1_q2, locals.var_q_d1_q2_dn4, locals.var_q_d1_q2_dn6, locals.var_q_d1_q2_dn7, locals.var_q_d1_q2_dn8, locals.var_q_d1_q2_dn9,)
    }
};
        locals.var_q_d1_q2 = assign14890_e14556;
        locals.var_q_d1_q2_dn4 = assign14890_e14556_d_n4;
        locals.var_q_d1_q2_dn6 = assign14890_e14556_d_n6;
        locals.var_q_d1_q2_dn7 = assign14890_e14556_d_n7;
        locals.var_q_d1_q2_dn8 = assign14890_e14556_d_n8;
        locals.var_q_d1_q2_dn9 = assign14890_e14556_d_n9;

        let (assign14900_e14566, assign14900_e14566_d_n4, assign14900_e14566_d_n6, assign14900_e14566_d_n7, assign14900_e14566_d_n8, assign14900_e14566_d_n9,) = {
    if ((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) {
        let assign14900_e14562: f64 = (2.0 * locals.var_q_d2_lnexpnum);
        let assign14900_e14564: f64 = (assign14900_e14562 - locals.var_q_d2_ln);
        (assign14900_e14564, ((2.0 * locals.var_q_d2_lnexpnum_dn4) - locals.var_q_d2_ln_dn4), ((2.0 * locals.var_q_d2_lnexpnum_dn6) - locals.var_q_d2_ln_dn6), ((2.0 * locals.var_q_d2_lnexpnum_dn7) - locals.var_q_d2_ln_dn7), ((2.0 * locals.var_q_d2_lnexpnum_dn8) - locals.var_q_d2_ln_dn8), ((2.0 * locals.var_q_d2_lnexpnum_dn9) - locals.var_q_d2_ln_dn9),)
    } else {
        (locals.var_q_d2_q2, locals.var_q_d2_q2_dn4, locals.var_q_d2_q2_dn6, locals.var_q_d2_q2_dn7, locals.var_q_d2_q2_dn8, locals.var_q_d2_q2_dn9,)
    }
};
        locals.var_q_d2_q2 = assign14900_e14566;
        locals.var_q_d2_q2_dn4 = assign14900_e14566_d_n4;
        locals.var_q_d2_q2_dn6 = assign14900_e14566_d_n6;
        locals.var_q_d2_q2_dn7 = assign14900_e14566_d_n7;
        locals.var_q_d2_q2_dn8 = assign14900_e14566_d_n8;
        locals.var_q_d2_q2_dn9 = assign14900_e14566_d_n9;

        let (assign14910_e14576, assign14910_e14576_d_n4, assign14910_e14576_d_n6, assign14910_e14576_d_n7, assign14910_e14576_d_n8, assign14910_e14576_d_n9,) = {
    if ((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) {
        let assign14910_e14573: f64 = (locals.var_k2 * locals.var_q_q2_int);
        let assign14910_e14574: f64 = (locals.var_q_k1q1 + assign14910_e14573);
        (assign14910_e14574, (locals.var_q_k1q1_dn4 + ((locals.var_k2_dn4 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn4))), (locals.var_q_k1q1_dn6 + ((locals.var_k2_dn6 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn6))), (locals.var_q_k1q1_dn7 + ((locals.var_k2_dn7 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn7))), (locals.var_q_k1q1_dn8 + ((locals.var_k2_dn8 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn8))), (locals.var_q_k1q1_dn9 + ((locals.var_k2_dn9 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn9))),)
    } else {
        (locals.var_q_qi_int, locals.var_q_qi_int_dn4, locals.var_q_qi_int_dn6, locals.var_q_qi_int_dn7, locals.var_q_qi_int_dn8, locals.var_q_qi_int_dn9,)
    }
};
        locals.var_q_qi_int = assign14910_e14576;
        locals.var_q_qi_int_dn4 = assign14910_e14576_d_n4;
        locals.var_q_qi_int_dn6 = assign14910_e14576_d_n6;
        locals.var_q_qi_int_dn7 = assign14910_e14576_d_n7;
        locals.var_q_qi_int_dn8 = assign14910_e14576_d_n8;
        locals.var_q_qi_int_dn9 = assign14910_e14576_d_n9;

        let (assign14920_e14586, assign14920_e14586_d_n4, assign14920_e14586_d_n6, assign14920_e14586_d_n7, assign14920_e14586_d_n8, assign14920_e14586_d_n9,) = {
    if ((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) {
        let assign14920_e14583: f64 = (locals.var_k2 * locals.var_q_d1_q2);
        let assign14920_e14584: f64 = (locals.var_k1 + assign14920_e14583);
        (assign14920_e14584, (locals.var_k1_dn4 + ((locals.var_k2_dn4 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn4))), (locals.var_k1_dn6 + ((locals.var_k2_dn6 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn6))), (locals.var_k1_dn7 + ((locals.var_k2_dn7 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn7))), (locals.var_k1_dn8 + ((locals.var_k2_dn8 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn8))), (locals.var_k1_dn9 + ((locals.var_k2_dn9 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn9))),)
    } else {
        (locals.var_q_d1_qi, locals.var_q_d1_qi_dn4, locals.var_q_d1_qi_dn6, locals.var_q_d1_qi_dn7, locals.var_q_d1_qi_dn8, locals.var_q_d1_qi_dn9,)
    }
};
        locals.var_q_d1_qi = assign14920_e14586;
        locals.var_q_d1_qi_dn4 = assign14920_e14586_d_n4;
        locals.var_q_d1_qi_dn6 = assign14920_e14586_d_n6;
        locals.var_q_d1_qi_dn7 = assign14920_e14586_d_n7;
        locals.var_q_d1_qi_dn8 = assign14920_e14586_d_n8;
        locals.var_q_d1_qi_dn9 = assign14920_e14586_d_n9;

        let (assign14930_e14594, assign14930_e14594_d_n4, assign14930_e14594_d_n6, assign14930_e14594_d_n7, assign14930_e14594_d_n8, assign14930_e14594_d_n9,) = {
    if ((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) {
        let assign14930_e14592: f64 = (locals.var_k2 * locals.var_q_d2_q2);
        (assign14930_e14592, ((locals.var_k2_dn4 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn4)), ((locals.var_k2_dn6 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn6)), ((locals.var_k2_dn7 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn7)), ((locals.var_k2_dn8 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn8)), ((locals.var_k2_dn9 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn9)),)
    } else {
        (locals.var_q_d2_qi, locals.var_q_d2_qi_dn4, locals.var_q_d2_qi_dn6, locals.var_q_d2_qi_dn7, locals.var_q_d2_qi_dn8, locals.var_q_d2_qi_dn9,)
    }
};
        locals.var_q_d2_qi = assign14930_e14594;
        locals.var_q_d2_qi_dn4 = assign14930_e14594_d_n4;
        locals.var_q_d2_qi_dn6 = assign14930_e14594_d_n6;
        locals.var_q_d2_qi_dn7 = assign14930_e14594_d_n7;
        locals.var_q_d2_qi_dn8 = assign14930_e14594_d_n8;
        locals.var_q_d2_qi_dn9 = assign14930_e14594_d_n9;

        let (assign14940_e14604, assign14940_e14604_d_n4, assign14940_e14604_d_n6, assign14940_e14604_d_n7, assign14940_e14604_d_n8, assign14940_e14604_d_n9,) = {
    if ((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) {
        let assign14940_e14600: f64 = (locals.var_q_qi_int * locals.var_q_expnum);
        let assign14940_e14602: f64 = (assign14940_e14600 - locals.var_q_aexp);
        (assign14940_e14602, (((locals.var_q_qi_int_dn4 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4), (((locals.var_q_qi_int_dn6 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6), (((locals.var_q_qi_int_dn7 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7), (((locals.var_q_qi_int_dn8 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8), (((locals.var_q_qi_int_dn9 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9),)
    } else {
        (locals.var_q_zero, locals.var_q_zero_dn4, locals.var_q_zero_dn6, locals.var_q_zero_dn7, locals.var_q_zero_dn8, locals.var_q_zero_dn9,)
    }
};
        locals.var_q_zero = assign14940_e14604;
        locals.var_q_zero_dn4 = assign14940_e14604_d_n4;
        locals.var_q_zero_dn6 = assign14940_e14604_d_n6;
        locals.var_q_zero_dn7 = assign14940_e14604_d_n7;
        locals.var_q_zero_dn8 = assign14940_e14604_d_n8;
        locals.var_q_zero_dn9 = assign14940_e14604_d_n9;

        let (assign14950_e14618, assign14950_e14618_d_n4, assign14950_e14618_d_n6, assign14950_e14618_d_n7, assign14950_e14618_d_n8, assign14950_e14618_d_n9,) = {
    if ((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) {
        let assign14950_e14610: f64 = (locals.var_q_d1_qi * locals.var_q_expnum);
        let assign14950_e14613: f64 = (locals.var_q_qi_int * locals.var_q_d1_expnum);
        let assign14950_e14614: f64 = (assign14950_e14610 + assign14950_e14613);
        let assign14950_e14616: f64 = (assign14950_e14614 + locals.var_q_aexp);
        (assign14950_e14616, ((((locals.var_q_d1_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn4)) + ((locals.var_q_qi_int_dn4 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4), ((((locals.var_q_d1_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn6)) + ((locals.var_q_qi_int_dn6 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6), ((((locals.var_q_d1_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn7)) + ((locals.var_q_qi_int_dn7 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7), ((((locals.var_q_d1_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn8)) + ((locals.var_q_qi_int_dn8 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8), ((((locals.var_q_d1_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn9)) + ((locals.var_q_qi_int_dn9 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9),)
    } else {
        (locals.var_q_d1_zero, locals.var_q_d1_zero_dn4, locals.var_q_d1_zero_dn6, locals.var_q_d1_zero_dn7, locals.var_q_d1_zero_dn8, locals.var_q_d1_zero_dn9,)
    }
};
        locals.var_q_d1_zero = assign14950_e14618;
        locals.var_q_d1_zero_dn4 = assign14950_e14618_d_n4;
        locals.var_q_d1_zero_dn6 = assign14950_e14618_d_n6;
        locals.var_q_d1_zero_dn7 = assign14950_e14618_d_n7;
        locals.var_q_d1_zero_dn8 = assign14950_e14618_d_n8;
        locals.var_q_d1_zero_dn9 = assign14950_e14618_d_n9;

        let (assign14960_e14638, assign14960_e14638_d_n4, assign14960_e14638_d_n6, assign14960_e14638_d_n7, assign14960_e14638_d_n8, assign14960_e14638_d_n9,) = {
    if ((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) {
        let assign14960_e14624: f64 = (locals.var_q_d2_qi * locals.var_q_expnum);
        let assign14960_e14627: f64 = (2.0 * locals.var_q_d1_qi);
        let assign14960_e14629: f64 = (assign14960_e14627 * locals.var_q_d1_expnum);
        let assign14960_e14630: f64 = (assign14960_e14624 + assign14960_e14629);
        let assign14960_e14633: f64 = (locals.var_q_qi_int * locals.var_q_d2_expnum);
        let assign14960_e14634: f64 = (assign14960_e14630 + assign14960_e14633);
        let assign14960_e14636: f64 = (assign14960_e14634 - locals.var_q_aexp);
        (assign14960_e14636, (((((locals.var_q_d2_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_d1_qi_dn4) * locals.var_q_d1_expnum) + (assign14960_e14627 * locals.var_q_d1_expnum_dn4))) + ((locals.var_q_qi_int_dn4 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn4))) - locals.var_q_aexp_dn4), (((((locals.var_q_d2_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_d1_qi_dn6) * locals.var_q_d1_expnum) + (assign14960_e14627 * locals.var_q_d1_expnum_dn6))) + ((locals.var_q_qi_int_dn6 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn6))) - locals.var_q_aexp_dn6), (((((locals.var_q_d2_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_d1_qi_dn7) * locals.var_q_d1_expnum) + (assign14960_e14627 * locals.var_q_d1_expnum_dn7))) + ((locals.var_q_qi_int_dn7 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn7))) - locals.var_q_aexp_dn7), (((((locals.var_q_d2_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_d1_qi_dn8) * locals.var_q_d1_expnum) + (assign14960_e14627 * locals.var_q_d1_expnum_dn8))) + ((locals.var_q_qi_int_dn8 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn8))) - locals.var_q_aexp_dn8), (((((locals.var_q_d2_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_d1_qi_dn9) * locals.var_q_d1_expnum) + (assign14960_e14627 * locals.var_q_d1_expnum_dn9))) + ((locals.var_q_qi_int_dn9 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn9))) - locals.var_q_aexp_dn9),)
    } else {
        (locals.var_q_d2_zero, locals.var_q_d2_zero_dn4, locals.var_q_d2_zero_dn6, locals.var_q_d2_zero_dn7, locals.var_q_d2_zero_dn8, locals.var_q_d2_zero_dn9,)
    }
};
        locals.var_q_d2_zero = assign14960_e14638;
        locals.var_q_d2_zero_dn4 = assign14960_e14638_d_n4;
        locals.var_q_d2_zero_dn6 = assign14960_e14638_d_n6;
        locals.var_q_d2_zero_dn7 = assign14960_e14638_d_n7;
        locals.var_q_d2_zero_dn8 = assign14960_e14638_d_n8;
        locals.var_q_d2_zero_dn9 = assign14960_e14638_d_n9;

        let (assign14970_e14652, assign14970_e14652_d_n4, assign14970_e14652_d_n6, assign14970_e14652_d_n7, assign14970_e14652_d_n8, assign14970_e14652_d_n9,) = {
    if ((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) {
        let assign14970_e14644: f64 = (locals.var_q_d1_zero * locals.var_q_d1_zero);
        let assign14970_e14647: f64 = (0.5 * locals.var_q_zero);
        let assign14970_e14649: f64 = (assign14970_e14647 * locals.var_q_d2_zero);
        let assign14970_e14650: f64 = (assign14970_e14644 - assign14970_e14649);
        (assign14970_e14650, (((locals.var_q_d1_zero_dn4 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn4)) - (((0.5 * locals.var_q_zero_dn4) * locals.var_q_d2_zero) + (assign14970_e14647 * locals.var_q_d2_zero_dn4))), (((locals.var_q_d1_zero_dn6 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn6)) - (((0.5 * locals.var_q_zero_dn6) * locals.var_q_d2_zero) + (assign14970_e14647 * locals.var_q_d2_zero_dn6))), (((locals.var_q_d1_zero_dn7 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn7)) - (((0.5 * locals.var_q_zero_dn7) * locals.var_q_d2_zero) + (assign14970_e14647 * locals.var_q_d2_zero_dn7))), (((locals.var_q_d1_zero_dn8 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn8)) - (((0.5 * locals.var_q_zero_dn8) * locals.var_q_d2_zero) + (assign14970_e14647 * locals.var_q_d2_zero_dn8))), (((locals.var_q_d1_zero_dn9 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn9)) - (((0.5 * locals.var_q_zero_dn9) * locals.var_q_d2_zero) + (assign14970_e14647 * locals.var_q_d2_zero_dn9))),)
    } else {
        (locals.var_q_temp, locals.var_q_temp_dn4, locals.var_q_temp_dn6, locals.var_q_temp_dn7, locals.var_q_temp_dn8, locals.var_q_temp_dn9,)
    }
};
        locals.var_q_temp = assign14970_e14652;
        locals.var_q_temp_dn4 = assign14970_e14652_d_n4;
        locals.var_q_temp_dn6 = assign14970_e14652_d_n6;
        locals.var_q_temp_dn7 = assign14970_e14652_d_n7;
        locals.var_q_temp_dn8 = assign14970_e14652_d_n8;
        locals.var_q_temp_dn9 = assign14970_e14652_d_n9;

        let (assign14980_e14669, assign14980_e14669_d_n4, assign14980_e14669_d_n6, assign14980_e14669_d_n7, assign14980_e14669_d_n8, assign14980_e14669_d_n9,) = {
    if ((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) {
        let assign14980_e14657: f64 = (-locals.var_q_zero);
        let assign14980_e14659: f64 = (assign14980_e14657 * locals.var_q_d1_zero);
        let assign14980_e14661: f64 = (assign14980_e14659 * locals.var_q_temp);
        let assign14980_e14664: f64 = (locals.var_q_temp * locals.var_q_temp);
        let assign14980_e14666: f64 = (assign14980_e14664 + 1e-200);
        let assign14980_e14667: f64 = (assign14980_e14661 / assign14980_e14666);
        (assign14980_e14667, ((((((((-locals.var_q_zero_dn4) * locals.var_q_d1_zero) + (assign14980_e14657 * locals.var_q_d1_zero_dn4)) * locals.var_q_temp) + (assign14980_e14659 * locals.var_q_temp_dn4)) * assign14980_e14666) - (assign14980_e14661 * ((locals.var_q_temp_dn4 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn4)))) / (assign14980_e14666 * assign14980_e14666)), ((((((((-locals.var_q_zero_dn6) * locals.var_q_d1_zero) + (assign14980_e14657 * locals.var_q_d1_zero_dn6)) * locals.var_q_temp) + (assign14980_e14659 * locals.var_q_temp_dn6)) * assign14980_e14666) - (assign14980_e14661 * ((locals.var_q_temp_dn6 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn6)))) / (assign14980_e14666 * assign14980_e14666)), ((((((((-locals.var_q_zero_dn7) * locals.var_q_d1_zero) + (assign14980_e14657 * locals.var_q_d1_zero_dn7)) * locals.var_q_temp) + (assign14980_e14659 * locals.var_q_temp_dn7)) * assign14980_e14666) - (assign14980_e14661 * ((locals.var_q_temp_dn7 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn7)))) / (assign14980_e14666 * assign14980_e14666)), ((((((((-locals.var_q_zero_dn8) * locals.var_q_d1_zero) + (assign14980_e14657 * locals.var_q_d1_zero_dn8)) * locals.var_q_temp) + (assign14980_e14659 * locals.var_q_temp_dn8)) * assign14980_e14666) - (assign14980_e14661 * ((locals.var_q_temp_dn8 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn8)))) / (assign14980_e14666 * assign14980_e14666)), ((((((((-locals.var_q_zero_dn9) * locals.var_q_d1_zero) + (assign14980_e14657 * locals.var_q_d1_zero_dn9)) * locals.var_q_temp) + (assign14980_e14659 * locals.var_q_temp_dn9)) * assign14980_e14666) - (assign14980_e14661 * ((locals.var_q_temp_dn9 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn9)))) / (assign14980_e14666 * assign14980_e14666)),)
    } else {
        (locals.var_q_eps2, locals.var_q_eps2_dn4, locals.var_q_eps2_dn6, locals.var_q_eps2_dn7, locals.var_q_eps2_dn8, locals.var_q_eps2_dn9,)
    }
};
        locals.var_q_eps2 = assign14980_e14669;
        locals.var_q_eps2_dn4 = assign14980_e14669_d_n4;
        locals.var_q_eps2_dn6 = assign14980_e14669_d_n6;
        locals.var_q_eps2_dn7 = assign14980_e14669_d_n7;
        locals.var_q_eps2_dn8 = assign14980_e14669_d_n8;
        locals.var_q_eps2_dn9 = assign14980_e14669_d_n9;

        let (assign14990_e14677, assign14990_e14677_d_n4, assign14990_e14677_d_n6, assign14990_e14677_d_n7, assign14990_e14677_d_n8, assign14990_e14677_d_n9,) = {
    if ((locals.var_guard579 != 0.0) && (locals.var_guard580 != 0.0)) {
        let assign14990_e14675: f64 = (locals.var_q1s + locals.var_q_eps2);
        (assign14990_e14675, (locals.var_q1s_dn4 + locals.var_q_eps2_dn4), (locals.var_q1s_dn6 + locals.var_q_eps2_dn6), (locals.var_q1s_dn7 + locals.var_q_eps2_dn7), (locals.var_q1s_dn8 + locals.var_q_eps2_dn8), (locals.var_q1s_dn9 + locals.var_q_eps2_dn9),)
    } else {
        (locals.var_q1s, locals.var_q1s_dn4, locals.var_q1s_dn6, locals.var_q1s_dn7, locals.var_q1s_dn8, locals.var_q1s_dn9,)
    }
};
        locals.var_q1s = assign14990_e14677;
        locals.var_q1s_dn4 = assign14990_e14677_d_n4;
        locals.var_q1s_dn6 = assign14990_e14677_d_n6;
        locals.var_q1s_dn7 = assign14990_e14677_d_n7;
        locals.var_q1s_dn8 = assign14990_e14677_d_n8;
        locals.var_q1s_dn9 = assign14990_e14677_d_n9;

    }

    pub(super) fn stamp_transient_block_37(
        locals: &mut StampLocals,
    ) {
        let assign15000_e14680: f64 = (locals.var_k1 * locals.var_q1s);
        locals.var_k1q1s = assign15000_e14680;
        locals.var_k1q1s_dn4 = ((locals.var_k1_dn4 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn4));
        locals.var_k1q1s_dn6 = ((locals.var_k1_dn6 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn6));
        locals.var_k1q1s_dn7 = ((locals.var_k1_dn7 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn7));
        locals.var_k1q1s_dn8 = ((locals.var_k1_dn8 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn8));
        locals.var_k1q1s_dn9 = ((locals.var_k1_dn9 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn9));

        let assign15010_e14683: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign15010_e14685: f64 = assign15010_e14683;
        let assign15010_e14687: f64 = if assign15010_e14685 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard588 = assign15010_e14687;

        let (assign15020_e14696, assign15020_e14696_d_n4, assign15020_e14696_d_n6, assign15020_e14696_d_n7, assign15020_e14696_d_n8, assign15020_e14696_d_n9,) = {
    if (locals.var_guard588 != 0.0) {
        let assign15020_e14691: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign15020_e14693: f64 = assign15020_e14691;
        let assign15020_e14694: f64 = (assign15020_e14693).exp();
        (assign15020_e14694, (assign15020_e14694 * (locals.var_xg1x_dn4 - locals.var_q1s_dn4)), (assign15020_e14694 * (locals.var_xg1x_dn6 - locals.var_q1s_dn6)), (assign15020_e14694 * (locals.var_xg1x_dn7 - locals.var_q1s_dn7)), (assign15020_e14694 * (locals.var_xg1x_dn8 - locals.var_q1s_dn8)), (assign15020_e14694 * (locals.var_xg1x_dn9 - locals.var_q1s_dn9)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign15020_e14696;
        locals.var_q_temp1_dn4 = assign15020_e14696_d_n4;
        locals.var_q_temp1_dn6 = assign15020_e14696_d_n6;
        locals.var_q_temp1_dn7 = assign15020_e14696_d_n7;
        locals.var_q_temp1_dn8 = assign15020_e14696_d_n8;
        locals.var_q_temp1_dn9 = assign15020_e14696_d_n9;

        let (assign15030_e14735, assign15030_e14735_d_n4, assign15030_e14735_d_n6, assign15030_e14735_d_n7, assign15030_e14735_d_n8, assign15030_e14735_d_n9,) = {
    if (locals.var_guard588 == 0.0) {
        let assign15030_e14703: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign15030_e14705: f64 = assign15030_e14703;
        let assign15030_e14707: f64 = (assign15030_e14705 - 80.0);
        let assign15030_e14712: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign15030_e14714: f64 = assign15030_e14712;
        let assign15030_e14716: f64 = (assign15030_e14714 - 80.0);
        let assign15030_e14717: f64 = (0.5 * assign15030_e14716);
        let assign15030_e14721: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign15030_e14723: f64 = assign15030_e14721;
        let assign15030_e14725: f64 = (assign15030_e14723 - 80.0);
        let assign15030_e14727: f64 = (assign15030_e14725 * 0.3333333333333);
        let assign15030_e14728: f64 = (1.0 + assign15030_e14727);
        let assign15030_e14729: f64 = (assign15030_e14717 * assign15030_e14728);
        let assign15030_e14730: f64 = (1.0 + assign15030_e14729);
        let assign15030_e14731: f64 = (assign15030_e14707 * assign15030_e14730);
        let assign15030_e14732: f64 = (1.0 + assign15030_e14731);
        let assign15030_e14733: f64 = (5.54062e34 * assign15030_e14732);
        (assign15030_e14733, (5.54062e34 * (((locals.var_xg1x_dn4 - locals.var_q1s_dn4) * assign15030_e14730) + (assign15030_e14707 * (((0.5 * (locals.var_xg1x_dn4 - locals.var_q1s_dn4)) * assign15030_e14728) + (assign15030_e14717 * ((locals.var_xg1x_dn4 - locals.var_q1s_dn4) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn6 - locals.var_q1s_dn6) * assign15030_e14730) + (assign15030_e14707 * (((0.5 * (locals.var_xg1x_dn6 - locals.var_q1s_dn6)) * assign15030_e14728) + (assign15030_e14717 * ((locals.var_xg1x_dn6 - locals.var_q1s_dn6) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn7 - locals.var_q1s_dn7) * assign15030_e14730) + (assign15030_e14707 * (((0.5 * (locals.var_xg1x_dn7 - locals.var_q1s_dn7)) * assign15030_e14728) + (assign15030_e14717 * ((locals.var_xg1x_dn7 - locals.var_q1s_dn7) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn8 - locals.var_q1s_dn8) * assign15030_e14730) + (assign15030_e14707 * (((0.5 * (locals.var_xg1x_dn8 - locals.var_q1s_dn8)) * assign15030_e14728) + (assign15030_e14717 * ((locals.var_xg1x_dn8 - locals.var_q1s_dn8) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn9 - locals.var_q1s_dn9) * assign15030_e14730) + (assign15030_e14707 * (((0.5 * (locals.var_xg1x_dn9 - locals.var_q1s_dn9)) * assign15030_e14728) + (assign15030_e14717 * ((locals.var_xg1x_dn9 - locals.var_q1s_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign15030_e14735;
        locals.var_q_temp1_dn4 = assign15030_e14735_d_n4;
        locals.var_q_temp1_dn6 = assign15030_e14735_d_n6;
        locals.var_q_temp1_dn7 = assign15030_e14735_d_n7;
        locals.var_q_temp1_dn8 = assign15030_e14735_d_n8;
        locals.var_q_temp1_dn9 = assign15030_e14735_d_n9;

        let assign15040_e14738: f64 = (locals.var_a0 * locals.var_q_temp1);
        locals.var_aexp1s = assign15040_e14738;
        locals.var_aexp1s_dn4 = ((locals.var_a0_dn4 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn4));
        locals.var_aexp1s_dn6 = ((locals.var_a0_dn6 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn6));
        locals.var_aexp1s_dn7 = ((locals.var_a0_dn7 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn7));
        locals.var_aexp1s_dn8 = ((locals.var_a0_dn8 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn8));
        locals.var_aexp1s_dn9 = ((locals.var_a0_dn9 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn9));

        let assign15050_e14741: f64 = (locals.var_k1q1s * locals.var_k1q1s);
        let assign15050_e14743: f64 = (assign15050_e14741 - locals.var_aexp1s);
        locals.var_qsqs = assign15050_e14743;
        locals.var_qsqs_dn4 = (((locals.var_k1q1s_dn4 * locals.var_k1q1s) + (locals.var_k1q1s * locals.var_k1q1s_dn4)) - locals.var_aexp1s_dn4);
        locals.var_qsqs_dn6 = (((locals.var_k1q1s_dn6 * locals.var_k1q1s) + (locals.var_k1q1s * locals.var_k1q1s_dn6)) - locals.var_aexp1s_dn6);
        locals.var_qsqs_dn7 = (((locals.var_k1q1s_dn7 * locals.var_k1q1s) + (locals.var_k1q1s * locals.var_k1q1s_dn7)) - locals.var_aexp1s_dn7);
        locals.var_qsqs_dn8 = (((locals.var_k1q1s_dn8 * locals.var_k1q1s) + (locals.var_k1q1s * locals.var_k1q1s_dn8)) - locals.var_aexp1s_dn8);
        locals.var_qsqs_dn9 = (((locals.var_k1q1s_dn9 * locals.var_k1q1s) + (locals.var_k1q1s * locals.var_k1q1s_dn9)) - locals.var_aexp1s_dn9);

        let assign15060_e14746: f64 = if locals.var_aexp1s <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard589 = assign15060_e14746;

        let (assign15070_e14750, assign15070_e14750_d_n4, assign15070_e14750_d_n6, assign15070_e14750_d_n7, assign15070_e14750_d_n8, assign15070_e14750_d_n9,) = {
    if (locals.var_guard589 != 0.0) {
        (1e-80, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qis, locals.var_qis_dn4, locals.var_qis_dn6, locals.var_qis_dn7, locals.var_qis_dn8, locals.var_qis_dn9,)
    }
};
        locals.var_qis = assign15070_e14750;
        locals.var_qis_dn4 = assign15070_e14750_d_n4;
        locals.var_qis_dn6 = assign15070_e14750_d_n6;
        locals.var_qis_dn7 = assign15070_e14750_d_n7;
        locals.var_qis_dn8 = assign15070_e14750_d_n8;
        locals.var_qis_dn9 = assign15070_e14750_d_n9;

        let (assign15080_e14756, assign15080_e14756_d_n4, assign15080_e14756_d_n6, assign15080_e14756_d_n7, assign15080_e14756_d_n8, assign15080_e14756_d_n9,) = {
    if (locals.var_guard589 != 0.0) {
        let assign15080_e14754: f64 = (locals.var_qis - locals.var_k1q1s);
        (assign15080_e14754, (locals.var_qis_dn4 - locals.var_k1q1s_dn4), (locals.var_qis_dn6 - locals.var_k1q1s_dn6), (locals.var_qis_dn7 - locals.var_k1q1s_dn7), (locals.var_qis_dn8 - locals.var_k1q1s_dn8), (locals.var_qis_dn9 - locals.var_k1q1s_dn9),)
    } else {
        (locals.var_k2q2s, locals.var_k2q2s_dn4, locals.var_k2q2s_dn6, locals.var_k2q2s_dn7, locals.var_k2q2s_dn8, locals.var_k2q2s_dn9,)
    }
};
        locals.var_k2q2s = assign15080_e14756;
        locals.var_k2q2s_dn4 = assign15080_e14756_d_n4;
        locals.var_k2q2s_dn6 = assign15080_e14756_d_n6;
        locals.var_k2q2s_dn7 = assign15080_e14756_d_n7;
        locals.var_k2q2s_dn8 = assign15080_e14756_d_n8;
        locals.var_k2q2s_dn9 = assign15080_e14756_d_n9;

        let (assign15090_e14762, assign15090_e14762_d_n4, assign15090_e14762_d_n6, assign15090_e14762_d_n7, assign15090_e14762_d_n8, assign15090_e14762_d_n9,) = {
    if (locals.var_guard589 != 0.0) {
        let assign15090_e14760: f64 = (locals.var_k2q2s / locals.var_k2);
        (assign15090_e14760, (((locals.var_k2q2s_dn4 * locals.var_k2) - (locals.var_k2q2s * locals.var_k2_dn4)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2s_dn6 * locals.var_k2) - (locals.var_k2q2s * locals.var_k2_dn6)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2s_dn7 * locals.var_k2) - (locals.var_k2q2s * locals.var_k2_dn7)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2s_dn8 * locals.var_k2) - (locals.var_k2q2s * locals.var_k2_dn8)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2s_dn9 * locals.var_k2) - (locals.var_k2q2s * locals.var_k2_dn9)) / (locals.var_k2 * locals.var_k2)),)
    } else {
        (locals.var_q2s, locals.var_q2s_dn4, locals.var_q2s_dn6, locals.var_q2s_dn7, locals.var_q2s_dn8, locals.var_q2s_dn9,)
    }
};
        locals.var_q2s = assign15090_e14762;
        locals.var_q2s_dn4 = assign15090_e14762_d_n4;
        locals.var_q2s_dn6 = assign15090_e14762_d_n6;
        locals.var_q2s_dn7 = assign15090_e14762_d_n7;
        locals.var_q2s_dn8 = assign15090_e14762_d_n8;
        locals.var_q2s_dn9 = assign15090_e14762_d_n9;

        let assign15100_e14765: f64 = (-0.005);
        let assign15100_e14766: f64 = if locals.var_qsqs < assign15100_e14765 { 1.0 } else { 0.0 };
        locals.var_guard590 = assign15100_e14766;

        let (assign15110_e14775, assign15110_e14775_d_n4, assign15110_e14775_d_n6, assign15110_e14775_d_n7, assign15110_e14775_d_n8, assign15110_e14775_d_n9,) = {
    if ((locals.var_guard589 == 0.0) && (locals.var_guard590 != 0.0)) {
        let assign15110_e14772: f64 = (locals.var_qsqs).abs();
        let assign15110_e14773: f64 = (assign15110_e14772).sqrt();
        (assign15110_e14773, (if locals.var_qsqs >= 0.0 { locals.var_qsqs_dn4 } else { (-locals.var_qsqs_dn4) } / (2.0 * assign15110_e14773)), (if locals.var_qsqs >= 0.0 { locals.var_qsqs_dn6 } else { (-locals.var_qsqs_dn6) } / (2.0 * assign15110_e14773)), (if locals.var_qsqs >= 0.0 { locals.var_qsqs_dn7 } else { (-locals.var_qsqs_dn7) } / (2.0 * assign15110_e14773)), (if locals.var_qsqs >= 0.0 { locals.var_qsqs_dn8 } else { (-locals.var_qsqs_dn8) } / (2.0 * assign15110_e14773)), (if locals.var_qsqs >= 0.0 { locals.var_qsqs_dn9 } else { (-locals.var_qsqs_dn9) } / (2.0 * assign15110_e14773)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign15110_e14775;
        locals.var_q_rac_qsq_dn4 = assign15110_e14775_d_n4;
        locals.var_q_rac_qsq_dn6 = assign15110_e14775_d_n6;
        locals.var_q_rac_qsq_dn7 = assign15110_e14775_d_n7;
        locals.var_q_rac_qsq_dn8 = assign15110_e14775_d_n8;
        locals.var_q_rac_qsq_dn9 = assign15110_e14775_d_n9;

        let (assign15120_e14787, assign15120_e14787_d_n4, assign15120_e14787_d_n6, assign15120_e14787_d_n7, assign15120_e14787_d_n8, assign15120_e14787_d_n9,) = {
    if ((locals.var_guard589 == 0.0) && (locals.var_guard590 != 0.0)) {
        let assign15120_e14783: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign15120_e14784: f64 = (assign15120_e14783).tan();
        let assign15120_e14785: f64 = (locals.var_q_rac_qsq / assign15120_e14784);
        (assign15120_e14785, (((locals.var_q_rac_qsq_dn4 * assign15120_e14784) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn4) / ((assign15120_e14783).cos() * (assign15120_e14783).cos())))) / (assign15120_e14784 * assign15120_e14784)), (((locals.var_q_rac_qsq_dn6 * assign15120_e14784) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn6) / ((assign15120_e14783).cos() * (assign15120_e14783).cos())))) / (assign15120_e14784 * assign15120_e14784)), (((locals.var_q_rac_qsq_dn7 * assign15120_e14784) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn7) / ((assign15120_e14783).cos() * (assign15120_e14783).cos())))) / (assign15120_e14784 * assign15120_e14784)), (((locals.var_q_rac_qsq_dn8 * assign15120_e14784) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn8) / ((assign15120_e14783).cos() * (assign15120_e14783).cos())))) / (assign15120_e14784 * assign15120_e14784)), (((locals.var_q_rac_qsq_dn9 * assign15120_e14784) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn9) / ((assign15120_e14783).cos() * (assign15120_e14783).cos())))) / (assign15120_e14784 * assign15120_e14784)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign15120_e14787;
        locals.var_q_qcoth_dn4 = assign15120_e14787_d_n4;
        locals.var_q_qcoth_dn6 = assign15120_e14787_d_n6;
        locals.var_q_qcoth_dn7 = assign15120_e14787_d_n7;
        locals.var_q_qcoth_dn8 = assign15120_e14787_d_n8;
        locals.var_q_qcoth_dn9 = assign15120_e14787_d_n9;

        let assign15130_e14790: f64 = if locals.var_qsqs > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard591 = assign15130_e14790;

        let (assign15140_e14802, assign15140_e14802_d_n4, assign15140_e14802_d_n6, assign15140_e14802_d_n7, assign15140_e14802_d_n8, assign15140_e14802_d_n9,) = {
    if (((locals.var_guard589 == 0.0) && (locals.var_guard590 == 0.0)) && (locals.var_guard591 != 0.0)) {
        let assign15140_e14799: f64 = (locals.var_qsqs).abs();
        let assign15140_e14800: f64 = (assign15140_e14799).sqrt();
        (assign15140_e14800, (if locals.var_qsqs >= 0.0 { locals.var_qsqs_dn4 } else { (-locals.var_qsqs_dn4) } / (2.0 * assign15140_e14800)), (if locals.var_qsqs >= 0.0 { locals.var_qsqs_dn6 } else { (-locals.var_qsqs_dn6) } / (2.0 * assign15140_e14800)), (if locals.var_qsqs >= 0.0 { locals.var_qsqs_dn7 } else { (-locals.var_qsqs_dn7) } / (2.0 * assign15140_e14800)), (if locals.var_qsqs >= 0.0 { locals.var_qsqs_dn8 } else { (-locals.var_qsqs_dn8) } / (2.0 * assign15140_e14800)), (if locals.var_qsqs >= 0.0 { locals.var_qsqs_dn9 } else { (-locals.var_qsqs_dn9) } / (2.0 * assign15140_e14800)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign15140_e14802;
        locals.var_q_rac_qsq_dn4 = assign15140_e14802_d_n4;
        locals.var_q_rac_qsq_dn6 = assign15140_e14802_d_n6;
        locals.var_q_rac_qsq_dn7 = assign15140_e14802_d_n7;
        locals.var_q_rac_qsq_dn8 = assign15140_e14802_d_n8;
        locals.var_q_rac_qsq_dn9 = assign15140_e14802_d_n9;

        let (assign15150_e14814, assign15150_e14814_d_n4, assign15150_e14814_d_n6, assign15150_e14814_d_n7, assign15150_e14814_d_n8, assign15150_e14814_d_n9,) = {
    if (((locals.var_guard589 == 0.0) && (locals.var_guard590 == 0.0)) && (locals.var_guard591 != 0.0)) {
        let assign15150_e14811: f64 = (-locals.var_q_rac_qsq);
        let assign15150_e14812: f64 = (assign15150_e14811).exp();
        (assign15150_e14812, (assign15150_e14812 * (-locals.var_q_rac_qsq_dn4)), (assign15150_e14812 * (-locals.var_q_rac_qsq_dn6)), (assign15150_e14812 * (-locals.var_q_rac_qsq_dn7)), (assign15150_e14812 * (-locals.var_q_rac_qsq_dn8)), (assign15150_e14812 * (-locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_invexpq, locals.var_q_invexpq_dn4, locals.var_q_invexpq_dn6, locals.var_q_invexpq_dn7, locals.var_q_invexpq_dn8, locals.var_q_invexpq_dn9,)
    }
};
        locals.var_q_invexpq = assign15150_e14814;
        locals.var_q_invexpq_dn4 = assign15150_e14814_d_n4;
        locals.var_q_invexpq_dn6 = assign15150_e14814_d_n6;
        locals.var_q_invexpq_dn7 = assign15150_e14814_d_n7;
        locals.var_q_invexpq_dn8 = assign15150_e14814_d_n8;
        locals.var_q_invexpq_dn9 = assign15150_e14814_d_n9;

        let (assign15160_e14832, assign15160_e14832_d_n4, assign15160_e14832_d_n6, assign15160_e14832_d_n7, assign15160_e14832_d_n8, assign15160_e14832_d_n9,) = {
    if (((locals.var_guard589 == 0.0) && (locals.var_guard590 == 0.0)) && (locals.var_guard591 != 0.0)) {
        let assign15160_e14825: f64 = (1.0 + locals.var_q_invexpq);
        let assign15160_e14826: f64 = (locals.var_q_rac_qsq * assign15160_e14825);
        let assign15160_e14829: f64 = (1.0 - locals.var_q_invexpq);
        let assign15160_e14830: f64 = (assign15160_e14826 / assign15160_e14829);
        (assign15160_e14830, (((((locals.var_q_rac_qsq_dn4 * assign15160_e14825) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn4)) * assign15160_e14829) - (assign15160_e14826 * (-locals.var_q_invexpq_dn4))) / (assign15160_e14829 * assign15160_e14829)), (((((locals.var_q_rac_qsq_dn6 * assign15160_e14825) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn6)) * assign15160_e14829) - (assign15160_e14826 * (-locals.var_q_invexpq_dn6))) / (assign15160_e14829 * assign15160_e14829)), (((((locals.var_q_rac_qsq_dn7 * assign15160_e14825) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn7)) * assign15160_e14829) - (assign15160_e14826 * (-locals.var_q_invexpq_dn7))) / (assign15160_e14829 * assign15160_e14829)), (((((locals.var_q_rac_qsq_dn8 * assign15160_e14825) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn8)) * assign15160_e14829) - (assign15160_e14826 * (-locals.var_q_invexpq_dn8))) / (assign15160_e14829 * assign15160_e14829)), (((((locals.var_q_rac_qsq_dn9 * assign15160_e14825) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn9)) * assign15160_e14829) - (assign15160_e14826 * (-locals.var_q_invexpq_dn9))) / (assign15160_e14829 * assign15160_e14829)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign15160_e14832;
        locals.var_q_qcoth_dn4 = assign15160_e14832_d_n4;
        locals.var_q_qcoth_dn6 = assign15160_e14832_d_n6;
        locals.var_q_qcoth_dn7 = assign15160_e14832_d_n7;
        locals.var_q_qcoth_dn8 = assign15160_e14832_d_n8;
        locals.var_q_qcoth_dn9 = assign15160_e14832_d_n9;

        let (assign15170_e14859, assign15170_e14859_d_n4, assign15170_e14859_d_n6, assign15170_e14859_d_n7, assign15170_e14859_d_n8, assign15170_e14859_d_n9,) = {
    if (((locals.var_guard589 == 0.0) && (locals.var_guard590 == 0.0)) && (locals.var_guard591 == 0.0)) {
        let assign15170_e14844: f64 = (locals.var_qsqs * 0.1666666666667);
        let assign15170_e14848: f64 = (locals.var_qsqs * 0.0166666666667);
        let assign15170_e14852: f64 = (locals.var_qsqs * 0.0238095238095);
        let assign15170_e14853: f64 = (1.0 - assign15170_e14852);
        let assign15170_e14854: f64 = (assign15170_e14848 * assign15170_e14853);
        let assign15170_e14855: f64 = (1.0 - assign15170_e14854);
        let assign15170_e14856: f64 = (assign15170_e14844 * assign15170_e14855);
        let assign15170_e14857: f64 = (2.0 + assign15170_e14856);
        (assign15170_e14857, (((locals.var_qsqs_dn4 * 0.1666666666667) * assign15170_e14855) + (assign15170_e14844 * (-(((locals.var_qsqs_dn4 * 0.0166666666667) * assign15170_e14853) + (assign15170_e14848 * (-(locals.var_qsqs_dn4 * 0.0238095238095))))))), (((locals.var_qsqs_dn6 * 0.1666666666667) * assign15170_e14855) + (assign15170_e14844 * (-(((locals.var_qsqs_dn6 * 0.0166666666667) * assign15170_e14853) + (assign15170_e14848 * (-(locals.var_qsqs_dn6 * 0.0238095238095))))))), (((locals.var_qsqs_dn7 * 0.1666666666667) * assign15170_e14855) + (assign15170_e14844 * (-(((locals.var_qsqs_dn7 * 0.0166666666667) * assign15170_e14853) + (assign15170_e14848 * (-(locals.var_qsqs_dn7 * 0.0238095238095))))))), (((locals.var_qsqs_dn8 * 0.1666666666667) * assign15170_e14855) + (assign15170_e14844 * (-(((locals.var_qsqs_dn8 * 0.0166666666667) * assign15170_e14853) + (assign15170_e14848 * (-(locals.var_qsqs_dn8 * 0.0238095238095))))))), (((locals.var_qsqs_dn9 * 0.1666666666667) * assign15170_e14855) + (assign15170_e14844 * (-(((locals.var_qsqs_dn9 * 0.0166666666667) * assign15170_e14853) + (assign15170_e14848 * (-(locals.var_qsqs_dn9 * 0.0238095238095))))))),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign15170_e14859;
        locals.var_q_qcoth_dn4 = assign15170_e14859_d_n4;
        locals.var_q_qcoth_dn6 = assign15170_e14859_d_n6;
        locals.var_q_qcoth_dn7 = assign15170_e14859_d_n7;
        locals.var_q_qcoth_dn8 = assign15170_e14859_d_n8;
        locals.var_q_qcoth_dn9 = assign15170_e14859_d_n9;

        let assign15180_e14862: f64 = (1.01 * locals.var_k1q1s);
        let assign15180_e14864: f64 = (assign15180_e14862 + locals.var_q_qcoth);
        let assign15180_e14866: f64 = if assign15180_e14864 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard592 = assign15180_e14866;

        let (assign15190_e14875, assign15190_e14875_d_n4, assign15190_e14875_d_n6, assign15190_e14875_d_n7, assign15190_e14875_d_n8, assign15190_e14875_d_n9,) = {
    if ((locals.var_guard589 == 0.0) && (locals.var_guard592 != 0.0)) {
        let assign15190_e14873: f64 = (locals.var_k1q1s + locals.var_q_qcoth);
        (assign15190_e14873, (locals.var_k1q1s_dn4 + locals.var_q_qcoth_dn4), (locals.var_k1q1s_dn6 + locals.var_q_qcoth_dn6), (locals.var_k1q1s_dn7 + locals.var_q_qcoth_dn7), (locals.var_k1q1s_dn8 + locals.var_q_qcoth_dn8), (locals.var_k1q1s_dn9 + locals.var_q_qcoth_dn9),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign15190_e14875;
        locals.var_q_temp1_dn4 = assign15190_e14875_d_n4;
        locals.var_q_temp1_dn6 = assign15190_e14875_d_n6;
        locals.var_q_temp1_dn7 = assign15190_e14875_d_n7;
        locals.var_q_temp1_dn8 = assign15190_e14875_d_n8;
        locals.var_q_temp1_dn9 = assign15190_e14875_d_n9;

        let assign15200_e14878: f64 = (locals.var_aexp1s * locals.var_k1q1s);
        let assign15200_e14881: f64 = (0.9 * locals.var_k1q1s);
        let assign15200_e14883: f64 = (assign15200_e14881 * locals.var_k1q1s);
        let assign15200_e14885: f64 = (assign15200_e14883 * locals.var_q_temp1);
        let assign15200_e14886: f64 = if assign15200_e14878 < assign15200_e14885 { 1.0 } else { 0.0 };
        locals.var_guard593 = assign15200_e14886;

        let (assign15210_e14899, assign15210_e14899_d_n4, assign15210_e14899_d_n6, assign15210_e14899_d_n7, assign15210_e14899_d_n8, assign15210_e14899_d_n9,) = {
    if (((locals.var_guard589 == 0.0) && (locals.var_guard592 != 0.0)) && (locals.var_guard593 != 0.0)) {
        let assign15210_e14895: f64 = (locals.var_aexp1s / locals.var_q_temp1);
        let assign15210_e14897: f64 = (assign15210_e14895 + 1e-80);
        (assign15210_e14897, (((locals.var_aexp1s_dn4 * locals.var_q_temp1) - (locals.var_aexp1s * locals.var_q_temp1_dn4)) / (locals.var_q_temp1 * locals.var_q_temp1)), (((locals.var_aexp1s_dn6 * locals.var_q_temp1) - (locals.var_aexp1s * locals.var_q_temp1_dn6)) / (locals.var_q_temp1 * locals.var_q_temp1)), (((locals.var_aexp1s_dn7 * locals.var_q_temp1) - (locals.var_aexp1s * locals.var_q_temp1_dn7)) / (locals.var_q_temp1 * locals.var_q_temp1)), (((locals.var_aexp1s_dn8 * locals.var_q_temp1) - (locals.var_aexp1s * locals.var_q_temp1_dn8)) / (locals.var_q_temp1 * locals.var_q_temp1)), (((locals.var_aexp1s_dn9 * locals.var_q_temp1) - (locals.var_aexp1s * locals.var_q_temp1_dn9)) / (locals.var_q_temp1 * locals.var_q_temp1)),)
    } else {
        (locals.var_qis, locals.var_qis_dn4, locals.var_qis_dn6, locals.var_qis_dn7, locals.var_qis_dn8, locals.var_qis_dn9,)
    }
};
        locals.var_qis = assign15210_e14899;
        locals.var_qis_dn4 = assign15210_e14899_d_n4;
        locals.var_qis_dn6 = assign15210_e14899_d_n6;
        locals.var_qis_dn7 = assign15210_e14899_d_n7;
        locals.var_qis_dn8 = assign15210_e14899_d_n8;
        locals.var_qis_dn9 = assign15210_e14899_d_n9;

        let (assign15220_e14910, assign15220_e14910_d_n4, assign15220_e14910_d_n6, assign15220_e14910_d_n7, assign15220_e14910_d_n8, assign15220_e14910_d_n9,) = {
    if (((locals.var_guard589 == 0.0) && (locals.var_guard592 != 0.0)) && (locals.var_guard593 != 0.0)) {
        let assign15220_e14908: f64 = (locals.var_qis - locals.var_k1q1s);
        (assign15220_e14908, (locals.var_qis_dn4 - locals.var_k1q1s_dn4), (locals.var_qis_dn6 - locals.var_k1q1s_dn6), (locals.var_qis_dn7 - locals.var_k1q1s_dn7), (locals.var_qis_dn8 - locals.var_k1q1s_dn8), (locals.var_qis_dn9 - locals.var_k1q1s_dn9),)
    } else {
        (locals.var_k2q2s, locals.var_k2q2s_dn4, locals.var_k2q2s_dn6, locals.var_k2q2s_dn7, locals.var_k2q2s_dn8, locals.var_k2q2s_dn9,)
    }
};
        locals.var_k2q2s = assign15220_e14910;
        locals.var_k2q2s_dn4 = assign15220_e14910_d_n4;
        locals.var_k2q2s_dn6 = assign15220_e14910_d_n6;
        locals.var_k2q2s_dn7 = assign15220_e14910_d_n7;
        locals.var_k2q2s_dn8 = assign15220_e14910_d_n8;
        locals.var_k2q2s_dn9 = assign15220_e14910_d_n9;

        let (assign15230_e14921, assign15230_e14921_d_n4, assign15230_e14921_d_n6, assign15230_e14921_d_n7, assign15230_e14921_d_n8, assign15230_e14921_d_n9,) = {
    if (((locals.var_guard589 == 0.0) && (locals.var_guard592 != 0.0)) && (locals.var_guard593 != 0.0)) {
        let assign15230_e14919: f64 = (locals.var_k2q2s / locals.var_k2);
        (assign15230_e14919, (((locals.var_k2q2s_dn4 * locals.var_k2) - (locals.var_k2q2s * locals.var_k2_dn4)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2s_dn6 * locals.var_k2) - (locals.var_k2q2s * locals.var_k2_dn6)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2s_dn7 * locals.var_k2) - (locals.var_k2q2s * locals.var_k2_dn7)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2s_dn8 * locals.var_k2) - (locals.var_k2q2s * locals.var_k2_dn8)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2s_dn9 * locals.var_k2) - (locals.var_k2q2s * locals.var_k2_dn9)) / (locals.var_k2 * locals.var_k2)),)
    } else {
        (locals.var_q2s, locals.var_q2s_dn4, locals.var_q2s_dn6, locals.var_q2s_dn7, locals.var_q2s_dn8, locals.var_q2s_dn9,)
    }
};
        locals.var_q2s = assign15230_e14921;
        locals.var_q2s_dn4 = assign15230_e14921_d_n4;
        locals.var_q2s_dn6 = assign15230_e14921_d_n6;
        locals.var_q2s_dn7 = assign15230_e14921_d_n7;
        locals.var_q2s_dn8 = assign15230_e14921_d_n8;
        locals.var_q2s_dn9 = assign15230_e14921_d_n9;

        let assign15240_e14924: f64 = if locals.var_qsqs > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard594 = assign15240_e14924;

        let (assign15250_e14949, assign15250_e14949_d_n4, assign15250_e14949_d_n6, assign15250_e14949_d_n7, assign15250_e14949_d_n8, assign15250_e14949_d_n9,) = {
    if ((((locals.var_guard589 == 0.0) && (locals.var_guard592 != 0.0)) && (locals.var_guard593 == 0.0)) && (locals.var_guard594 != 0.0)) {
        let assign15250_e14936: f64 = (4.0 * locals.var_qsqs);
        let assign15250_e14941: f64 = (2.0 - locals.var_q_invexpq);
        let assign15250_e14942: f64 = (locals.var_q_invexpq * assign15250_e14941);
        let assign15250_e14943: f64 = (1.0 - assign15250_e14942);
        let assign15250_e14944: f64 = (assign15250_e14936 / assign15250_e14943);
        let assign15250_e14945: f64 = (assign15250_e14944).ln();
        let assign15250_e14947: f64 = (assign15250_e14945 - locals.var_q_rac_qsq);
        (assign15250_e14947, ((((((4.0 * locals.var_qsqs_dn4) * assign15250_e14943) - (assign15250_e14936 * (-((locals.var_q_invexpq_dn4 * assign15250_e14941) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn4)))))) / (assign15250_e14943 * assign15250_e14943)) / assign15250_e14944) - locals.var_q_rac_qsq_dn4), ((((((4.0 * locals.var_qsqs_dn6) * assign15250_e14943) - (assign15250_e14936 * (-((locals.var_q_invexpq_dn6 * assign15250_e14941) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn6)))))) / (assign15250_e14943 * assign15250_e14943)) / assign15250_e14944) - locals.var_q_rac_qsq_dn6), ((((((4.0 * locals.var_qsqs_dn7) * assign15250_e14943) - (assign15250_e14936 * (-((locals.var_q_invexpq_dn7 * assign15250_e14941) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn7)))))) / (assign15250_e14943 * assign15250_e14943)) / assign15250_e14944) - locals.var_q_rac_qsq_dn7), ((((((4.0 * locals.var_qsqs_dn8) * assign15250_e14943) - (assign15250_e14936 * (-((locals.var_q_invexpq_dn8 * assign15250_e14941) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn8)))))) / (assign15250_e14943 * assign15250_e14943)) / assign15250_e14944) - locals.var_q_rac_qsq_dn8), ((((((4.0 * locals.var_qsqs_dn9) * assign15250_e14943) - (assign15250_e14936 * (-((locals.var_q_invexpq_dn9 * assign15250_e14941) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn9)))))) / (assign15250_e14943 * assign15250_e14943)) / assign15250_e14944) - locals.var_q_rac_qsq_dn9),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign15250_e14949;
        locals.var_q_temp2_dn4 = assign15250_e14949_d_n4;
        locals.var_q_temp2_dn6 = assign15250_e14949_d_n6;
        locals.var_q_temp2_dn7 = assign15250_e14949_d_n7;
        locals.var_q_temp2_dn8 = assign15250_e14949_d_n8;
        locals.var_q_temp2_dn9 = assign15250_e14949_d_n9;

        let assign15260_e14952: f64 = (-0.005);
        let assign15260_e14953: f64 = if locals.var_qsqs < assign15260_e14952 { 1.0 } else { 0.0 };
        locals.var_guard595 = assign15260_e14953;

        let (assign15270_e14971, assign15270_e14971_d_n4, assign15270_e14971_d_n6, assign15270_e14971_d_n7, assign15270_e14971_d_n8, assign15270_e14971_d_n9,) = {
    if (((((locals.var_guard589 == 0.0) && (locals.var_guard592 != 0.0)) && (locals.var_guard593 == 0.0)) && (locals.var_guard594 == 0.0)) && (locals.var_guard595 != 0.0)) {
        let assign15270_e14968: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign15270_e14969: f64 = (assign15270_e14968).sin();
        (assign15270_e14969, ((assign15270_e14968).cos() * (0.5 * locals.var_q_rac_qsq_dn4)), ((assign15270_e14968).cos() * (0.5 * locals.var_q_rac_qsq_dn6)), ((assign15270_e14968).cos() * (0.5 * locals.var_q_rac_qsq_dn7)), ((assign15270_e14968).cos() * (0.5 * locals.var_q_rac_qsq_dn8)), ((assign15270_e14968).cos() * (0.5 * locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign15270_e14971;
        locals.var_q_temp3_dn4 = assign15270_e14971_d_n4;
        locals.var_q_temp3_dn6 = assign15270_e14971_d_n6;
        locals.var_q_temp3_dn7 = assign15270_e14971_d_n7;
        locals.var_q_temp3_dn8 = assign15270_e14971_d_n8;
        locals.var_q_temp3_dn9 = assign15270_e14971_d_n9;

        let (assign15280_e14992, assign15280_e14992_d_n4, assign15280_e14992_d_n6, assign15280_e14992_d_n7, assign15280_e14992_d_n8, assign15280_e14992_d_n9,) = {
    if (((((locals.var_guard589 == 0.0) && (locals.var_guard592 != 0.0)) && (locals.var_guard593 == 0.0)) && (locals.var_guard594 == 0.0)) && (locals.var_guard595 != 0.0)) {
        let assign15280_e14985: f64 = (-locals.var_qsqs);
        let assign15280_e14988: f64 = (locals.var_q_temp3 * locals.var_q_temp3);
        let assign15280_e14989: f64 = (assign15280_e14985 / assign15280_e14988);
        let assign15280_e14990: f64 = (assign15280_e14989).ln();
        (assign15280_e14990, (((((-locals.var_qsqs_dn4) * assign15280_e14988) - (assign15280_e14985 * ((locals.var_q_temp3_dn4 * locals.var_q_temp3) + (locals.var_q_temp3 * locals.var_q_temp3_dn4)))) / (assign15280_e14988 * assign15280_e14988)) / assign15280_e14989), (((((-locals.var_qsqs_dn6) * assign15280_e14988) - (assign15280_e14985 * ((locals.var_q_temp3_dn6 * locals.var_q_temp3) + (locals.var_q_temp3 * locals.var_q_temp3_dn6)))) / (assign15280_e14988 * assign15280_e14988)) / assign15280_e14989), (((((-locals.var_qsqs_dn7) * assign15280_e14988) - (assign15280_e14985 * ((locals.var_q_temp3_dn7 * locals.var_q_temp3) + (locals.var_q_temp3 * locals.var_q_temp3_dn7)))) / (assign15280_e14988 * assign15280_e14988)) / assign15280_e14989), (((((-locals.var_qsqs_dn8) * assign15280_e14988) - (assign15280_e14985 * ((locals.var_q_temp3_dn8 * locals.var_q_temp3) + (locals.var_q_temp3 * locals.var_q_temp3_dn8)))) / (assign15280_e14988 * assign15280_e14988)) / assign15280_e14989), (((((-locals.var_qsqs_dn9) * assign15280_e14988) - (assign15280_e14985 * ((locals.var_q_temp3_dn9 * locals.var_q_temp3) + (locals.var_q_temp3 * locals.var_q_temp3_dn9)))) / (assign15280_e14988 * assign15280_e14988)) / assign15280_e14989),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign15280_e14992;
        locals.var_q_temp2_dn4 = assign15280_e14992_d_n4;
        locals.var_q_temp2_dn6 = assign15280_e14992_d_n6;
        locals.var_q_temp2_dn7 = assign15280_e14992_d_n7;
        locals.var_q_temp2_dn8 = assign15280_e14992_d_n8;
        locals.var_q_temp2_dn9 = assign15280_e14992_d_n9;

        let (assign15290_e15025, assign15290_e15025_d_n4, assign15290_e15025_d_n6, assign15290_e15025_d_n7, assign15290_e15025_d_n8, assign15290_e15025_d_n9,) = {
    if (((((locals.var_guard589 == 0.0) && (locals.var_guard592 != 0.0)) && (locals.var_guard593 == 0.0)) && (locals.var_guard594 == 0.0)) && (locals.var_guard595 == 0.0)) {
        let assign15290_e15009: f64 = (locals.var_qsqs * 0.3333333333333);
        let assign15290_e15013: f64 = (0.05 * locals.var_qsqs);
        let assign15290_e15017: f64 = (0.0396825396825397 * locals.var_qsqs);
        let assign15290_e15018: f64 = (1.0 - assign15290_e15017);
        let assign15290_e15019: f64 = (assign15290_e15013 * assign15290_e15018);
        let assign15290_e15020: f64 = (1.0 - assign15290_e15019);
        let assign15290_e15021: f64 = (assign15290_e15009 * assign15290_e15020);
        let assign15290_e15022: f64 = (4.0 - assign15290_e15021);
        let assign15290_e15023: f64 = (assign15290_e15022).ln();
        (assign15290_e15023, ((-(((locals.var_qsqs_dn4 * 0.3333333333333) * assign15290_e15020) + (assign15290_e15009 * (-(((0.05 * locals.var_qsqs_dn4) * assign15290_e15018) + (assign15290_e15013 * (-(0.0396825396825397 * locals.var_qsqs_dn4)))))))) / assign15290_e15022), ((-(((locals.var_qsqs_dn6 * 0.3333333333333) * assign15290_e15020) + (assign15290_e15009 * (-(((0.05 * locals.var_qsqs_dn6) * assign15290_e15018) + (assign15290_e15013 * (-(0.0396825396825397 * locals.var_qsqs_dn6)))))))) / assign15290_e15022), ((-(((locals.var_qsqs_dn7 * 0.3333333333333) * assign15290_e15020) + (assign15290_e15009 * (-(((0.05 * locals.var_qsqs_dn7) * assign15290_e15018) + (assign15290_e15013 * (-(0.0396825396825397 * locals.var_qsqs_dn7)))))))) / assign15290_e15022), ((-(((locals.var_qsqs_dn8 * 0.3333333333333) * assign15290_e15020) + (assign15290_e15009 * (-(((0.05 * locals.var_qsqs_dn8) * assign15290_e15018) + (assign15290_e15013 * (-(0.0396825396825397 * locals.var_qsqs_dn8)))))))) / assign15290_e15022), ((-(((locals.var_qsqs_dn9 * 0.3333333333333) * assign15290_e15020) + (assign15290_e15009 * (-(((0.05 * locals.var_qsqs_dn9) * assign15290_e15018) + (assign15290_e15013 * (-(0.0396825396825397 * locals.var_qsqs_dn9)))))))) / assign15290_e15022),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign15290_e15025;
        locals.var_q_temp2_dn4 = assign15290_e15025_d_n4;
        locals.var_q_temp2_dn6 = assign15290_e15025_d_n6;
        locals.var_q_temp2_dn7 = assign15290_e15025_d_n7;
        locals.var_q_temp2_dn8 = assign15290_e15025_d_n8;
        locals.var_q_temp2_dn9 = assign15290_e15025_d_n9;

        let (assign15300_e15046, assign15300_e15046_d_n4, assign15300_e15046_d_n6, assign15300_e15046_d_n7, assign15300_e15046_d_n8, assign15300_e15046_d_n9,) = {
    if (((locals.var_guard589 == 0.0) && (locals.var_guard592 != 0.0)) && (locals.var_guard593 == 0.0)) {
        let assign15300_e15035: f64 = (locals.var_xg2x - locals.var_xg1x);
        let assign15300_e15037: f64 = (assign15300_e15035 + locals.var_q1s);
        let assign15300_e15040: f64 = (locals.var_q_temp1).ln();
        let assign15300_e15041: f64 = (2.0 * assign15300_e15040);
        let assign15300_e15042: f64 = (assign15300_e15037 + assign15300_e15041);
        let assign15300_e15044: f64 = (assign15300_e15042 - locals.var_q_temp2);
        (assign15300_e15044, ((((locals.var_xg2x_dn4 - locals.var_xg1x_dn4) + locals.var_q1s_dn4) + (2.0 * (locals.var_q_temp1_dn4 / locals.var_q_temp1))) - locals.var_q_temp2_dn4), ((((locals.var_xg2x_dn6 - locals.var_xg1x_dn6) + locals.var_q1s_dn6) + (2.0 * (locals.var_q_temp1_dn6 / locals.var_q_temp1))) - locals.var_q_temp2_dn6), ((((locals.var_xg2x_dn7 - locals.var_xg1x_dn7) + locals.var_q1s_dn7) + (2.0 * (locals.var_q_temp1_dn7 / locals.var_q_temp1))) - locals.var_q_temp2_dn7), ((((locals.var_xg2x_dn8 - locals.var_xg1x_dn8) + locals.var_q1s_dn8) + (2.0 * (locals.var_q_temp1_dn8 / locals.var_q_temp1))) - locals.var_q_temp2_dn8), ((((locals.var_xg2x_dn9 - locals.var_xg1x_dn9) + locals.var_q1s_dn9) + (2.0 * (locals.var_q_temp1_dn9 / locals.var_q_temp1))) - locals.var_q_temp2_dn9),)
    } else {
        (locals.var_q2s, locals.var_q2s_dn4, locals.var_q2s_dn6, locals.var_q2s_dn7, locals.var_q2s_dn8, locals.var_q2s_dn9,)
    }
};
        locals.var_q2s = assign15300_e15046;
        locals.var_q2s_dn4 = assign15300_e15046_d_n4;
        locals.var_q2s_dn6 = assign15300_e15046_d_n6;
        locals.var_q2s_dn7 = assign15300_e15046_d_n7;
        locals.var_q2s_dn8 = assign15300_e15046_d_n8;
        locals.var_q2s_dn9 = assign15300_e15046_d_n9;

        let (assign15310_e15058, assign15310_e15058_d_n4, assign15310_e15058_d_n6, assign15310_e15058_d_n7, assign15310_e15058_d_n8, assign15310_e15058_d_n9,) = {
    if (((locals.var_guard589 == 0.0) && (locals.var_guard592 != 0.0)) && (locals.var_guard593 == 0.0)) {
        let assign15310_e15056: f64 = (locals.var_k2 * locals.var_q2s);
        (assign15310_e15056, ((locals.var_k2_dn4 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn4)), ((locals.var_k2_dn6 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn6)), ((locals.var_k2_dn7 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn7)), ((locals.var_k2_dn8 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn8)), ((locals.var_k2_dn9 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn9)),)
    } else {
        (locals.var_k2q2s, locals.var_k2q2s_dn4, locals.var_k2q2s_dn6, locals.var_k2q2s_dn7, locals.var_k2q2s_dn8, locals.var_k2q2s_dn9,)
    }
};
        locals.var_k2q2s = assign15310_e15058;
        locals.var_k2q2s_dn4 = assign15310_e15058_d_n4;
        locals.var_k2q2s_dn6 = assign15310_e15058_d_n6;
        locals.var_k2q2s_dn7 = assign15310_e15058_d_n7;
        locals.var_k2q2s_dn8 = assign15310_e15058_d_n8;
        locals.var_k2q2s_dn9 = assign15310_e15058_d_n9;

        let (assign15320_e15070, assign15320_e15070_d_n4, assign15320_e15070_d_n6, assign15320_e15070_d_n7, assign15320_e15070_d_n8, assign15320_e15070_d_n9,) = {
    if (((locals.var_guard589 == 0.0) && (locals.var_guard592 != 0.0)) && (locals.var_guard593 == 0.0)) {
        let assign15320_e15068: f64 = (locals.var_k1q1s + locals.var_k2q2s);
        (assign15320_e15068, (locals.var_k1q1s_dn4 + locals.var_k2q2s_dn4), (locals.var_k1q1s_dn6 + locals.var_k2q2s_dn6), (locals.var_k1q1s_dn7 + locals.var_k2q2s_dn7), (locals.var_k1q1s_dn8 + locals.var_k2q2s_dn8), (locals.var_k1q1s_dn9 + locals.var_k2q2s_dn9),)
    } else {
        (locals.var_qis, locals.var_qis_dn4, locals.var_qis_dn6, locals.var_qis_dn7, locals.var_qis_dn8, locals.var_qis_dn9,)
    }
};
        locals.var_qis = assign15320_e15070;
        locals.var_qis_dn4 = assign15320_e15070_d_n4;
        locals.var_qis_dn6 = assign15320_e15070_d_n6;
        locals.var_qis_dn7 = assign15320_e15070_d_n7;
        locals.var_qis_dn8 = assign15320_e15070_d_n8;
        locals.var_qis_dn9 = assign15320_e15070_d_n9;

        let assign15330_e15073: f64 = if locals.var_qsqs > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard596 = assign15330_e15073;

        let assign15340_e15076: f64 = locals.var_q1s;
        let assign15340_e15078: f64 = (assign15340_e15076 - locals.var_xg1x);
        let assign15340_e15080: f64 = (assign15340_e15078 - locals.var_q_rac_qsq);
        let assign15340_e15082: f64 = if assign15340_e15080 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard597 = assign15340_e15082;

        let (assign15350_e15101, assign15350_e15101_d_n4, assign15350_e15101_d_n6, assign15350_e15101_d_n7, assign15350_e15101_d_n8, assign15350_e15101_d_n9,) = {
    if ((((locals.var_guard589 == 0.0) && (locals.var_guard592 == 0.0)) && (locals.var_guard596 != 0.0)) && (locals.var_guard597 != 0.0)) {
        let assign15350_e15094: f64 = locals.var_q1s;
        let assign15350_e15096: f64 = (assign15350_e15094 - locals.var_xg1x);
        let assign15350_e15098: f64 = (assign15350_e15096 - locals.var_q_rac_qsq);
        let assign15350_e15099: f64 = (assign15350_e15098).exp();
        (assign15350_e15099, (assign15350_e15099 * ((locals.var_q1s_dn4 - locals.var_xg1x_dn4) - locals.var_q_rac_qsq_dn4)), (assign15350_e15099 * ((locals.var_q1s_dn6 - locals.var_xg1x_dn6) - locals.var_q_rac_qsq_dn6)), (assign15350_e15099 * ((locals.var_q1s_dn7 - locals.var_xg1x_dn7) - locals.var_q_rac_qsq_dn7)), (assign15350_e15099 * ((locals.var_q1s_dn8 - locals.var_xg1x_dn8) - locals.var_q_rac_qsq_dn8)), (assign15350_e15099 * ((locals.var_q1s_dn9 - locals.var_xg1x_dn9) - locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign15350_e15101;
        locals.var_q_temp3_dn4 = assign15350_e15101_d_n4;
        locals.var_q_temp3_dn6 = assign15350_e15101_d_n6;
        locals.var_q_temp3_dn7 = assign15350_e15101_d_n7;
        locals.var_q_temp3_dn8 = assign15350_e15101_d_n8;
        locals.var_q_temp3_dn9 = assign15350_e15101_d_n9;

        let (assign15360_e15154, assign15360_e15154_d_n4, assign15360_e15154_d_n6, assign15360_e15154_d_n7, assign15360_e15154_d_n8, assign15360_e15154_d_n9,) = {
    if ((((locals.var_guard589 == 0.0) && (locals.var_guard592 == 0.0)) && (locals.var_guard596 != 0.0)) && (locals.var_guard597 == 0.0)) {
        let assign15360_e15116: f64 = locals.var_q1s;
        let assign15360_e15118: f64 = (assign15360_e15116 - locals.var_xg1x);
        let assign15360_e15120: f64 = (assign15360_e15118 - locals.var_q_rac_qsq);
        let assign15360_e15122: f64 = (assign15360_e15120 - 80.0);
        let assign15360_e15127: f64 = locals.var_q1s;
        let assign15360_e15129: f64 = (assign15360_e15127 - locals.var_xg1x);
        let assign15360_e15131: f64 = (assign15360_e15129 - locals.var_q_rac_qsq);
        let assign15360_e15133: f64 = (assign15360_e15131 - 80.0);
        let assign15360_e15134: f64 = (0.5 * assign15360_e15133);
        let assign15360_e15138: f64 = locals.var_q1s;
        let assign15360_e15140: f64 = (assign15360_e15138 - locals.var_xg1x);
        let assign15360_e15142: f64 = (assign15360_e15140 - locals.var_q_rac_qsq);
        let assign15360_e15144: f64 = (assign15360_e15142 - 80.0);
        let assign15360_e15146: f64 = (assign15360_e15144 * 0.3333333333333);
        let assign15360_e15147: f64 = (1.0 + assign15360_e15146);
        let assign15360_e15148: f64 = (assign15360_e15134 * assign15360_e15147);
        let assign15360_e15149: f64 = (1.0 + assign15360_e15148);
        let assign15360_e15150: f64 = (assign15360_e15122 * assign15360_e15149);
        let assign15360_e15151: f64 = (1.0 + assign15360_e15150);
        let assign15360_e15152: f64 = (5.54062e34 * assign15360_e15151);
        (assign15360_e15152, (5.54062e34 * ((((locals.var_q1s_dn4 - locals.var_xg1x_dn4) - locals.var_q_rac_qsq_dn4) * assign15360_e15149) + (assign15360_e15122 * (((0.5 * ((locals.var_q1s_dn4 - locals.var_xg1x_dn4) - locals.var_q_rac_qsq_dn4)) * assign15360_e15147) + (assign15360_e15134 * (((locals.var_q1s_dn4 - locals.var_xg1x_dn4) - locals.var_q_rac_qsq_dn4) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_q1s_dn6 - locals.var_xg1x_dn6) - locals.var_q_rac_qsq_dn6) * assign15360_e15149) + (assign15360_e15122 * (((0.5 * ((locals.var_q1s_dn6 - locals.var_xg1x_dn6) - locals.var_q_rac_qsq_dn6)) * assign15360_e15147) + (assign15360_e15134 * (((locals.var_q1s_dn6 - locals.var_xg1x_dn6) - locals.var_q_rac_qsq_dn6) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_q1s_dn7 - locals.var_xg1x_dn7) - locals.var_q_rac_qsq_dn7) * assign15360_e15149) + (assign15360_e15122 * (((0.5 * ((locals.var_q1s_dn7 - locals.var_xg1x_dn7) - locals.var_q_rac_qsq_dn7)) * assign15360_e15147) + (assign15360_e15134 * (((locals.var_q1s_dn7 - locals.var_xg1x_dn7) - locals.var_q_rac_qsq_dn7) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_q1s_dn8 - locals.var_xg1x_dn8) - locals.var_q_rac_qsq_dn8) * assign15360_e15149) + (assign15360_e15122 * (((0.5 * ((locals.var_q1s_dn8 - locals.var_xg1x_dn8) - locals.var_q_rac_qsq_dn8)) * assign15360_e15147) + (assign15360_e15134 * (((locals.var_q1s_dn8 - locals.var_xg1x_dn8) - locals.var_q_rac_qsq_dn8) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_q1s_dn9 - locals.var_xg1x_dn9) - locals.var_q_rac_qsq_dn9) * assign15360_e15149) + (assign15360_e15122 * (((0.5 * ((locals.var_q1s_dn9 - locals.var_xg1x_dn9) - locals.var_q_rac_qsq_dn9)) * assign15360_e15147) + (assign15360_e15134 * (((locals.var_q1s_dn9 - locals.var_xg1x_dn9) - locals.var_q_rac_qsq_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign15360_e15154;
        locals.var_q_temp3_dn4 = assign15360_e15154_d_n4;
        locals.var_q_temp3_dn6 = assign15360_e15154_d_n6;
        locals.var_q_temp3_dn7 = assign15360_e15154_d_n7;
        locals.var_q_temp3_dn8 = assign15360_e15154_d_n8;
        locals.var_q_temp3_dn9 = assign15360_e15154_d_n9;

    }

    pub(super) fn stamp_transient_block_38(
        locals: &mut StampLocals,
    ) {
        let (assign15370_e15166, assign15370_e15166_d_n4, assign15370_e15166_d_n6, assign15370_e15166_d_n7, assign15370_e15166_d_n8, assign15370_e15166_d_n9,) = {
    if (((locals.var_guard589 == 0.0) && (locals.var_guard592 == 0.0)) && (locals.var_guard596 != 0.0)) {
        let assign15370_e15164: f64 = (locals.var_q_temp3 / locals.var_a0);
        (assign15370_e15164, (((locals.var_q_temp3_dn4 * locals.var_a0) - (locals.var_q_temp3 * locals.var_a0_dn4)) / (locals.var_a0 * locals.var_a0)), (((locals.var_q_temp3_dn6 * locals.var_a0) - (locals.var_q_temp3 * locals.var_a0_dn6)) / (locals.var_a0 * locals.var_a0)), (((locals.var_q_temp3_dn7 * locals.var_a0) - (locals.var_q_temp3 * locals.var_a0_dn7)) / (locals.var_a0 * locals.var_a0)), (((locals.var_q_temp3_dn8 * locals.var_a0) - (locals.var_q_temp3 * locals.var_a0_dn8)) / (locals.var_a0 * locals.var_a0)), (((locals.var_q_temp3_dn9 * locals.var_a0) - (locals.var_q_temp3 * locals.var_a0_dn9)) / (locals.var_a0 * locals.var_a0)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign15370_e15166;
        locals.var_q_temp2_dn4 = assign15370_e15166_d_n4;
        locals.var_q_temp2_dn6 = assign15370_e15166_d_n6;
        locals.var_q_temp2_dn7 = assign15370_e15166_d_n7;
        locals.var_q_temp2_dn8 = assign15370_e15166_d_n8;
        locals.var_q_temp2_dn9 = assign15370_e15166_d_n9;

        let (assign15380_e15188, assign15380_e15188_d_n4, assign15380_e15188_d_n6, assign15380_e15188_d_n7, assign15380_e15188_d_n8, assign15380_e15188_d_n9,) = {
    if (((locals.var_guard589 == 0.0) && (locals.var_guard592 == 0.0)) && (locals.var_guard596 != 0.0)) {
        let assign15380_e15176: f64 = (4.0 * locals.var_qsqs);
        let assign15380_e15178: f64 = (assign15380_e15176 * locals.var_q_temp2);
        let assign15380_e15183: f64 = (2.0 - locals.var_q_invexpq);
        let assign15380_e15184: f64 = (locals.var_q_invexpq * assign15380_e15183);
        let assign15380_e15185: f64 = (1.0 - assign15380_e15184);
        let assign15380_e15186: f64 = (assign15380_e15178 / assign15380_e15185);
        (assign15380_e15186, ((((((4.0 * locals.var_qsqs_dn4) * locals.var_q_temp2) + (assign15380_e15176 * locals.var_q_temp2_dn4)) * assign15380_e15185) - (assign15380_e15178 * (-((locals.var_q_invexpq_dn4 * assign15380_e15183) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn4)))))) / (assign15380_e15185 * assign15380_e15185)), ((((((4.0 * locals.var_qsqs_dn6) * locals.var_q_temp2) + (assign15380_e15176 * locals.var_q_temp2_dn6)) * assign15380_e15185) - (assign15380_e15178 * (-((locals.var_q_invexpq_dn6 * assign15380_e15183) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn6)))))) / (assign15380_e15185 * assign15380_e15185)), ((((((4.0 * locals.var_qsqs_dn7) * locals.var_q_temp2) + (assign15380_e15176 * locals.var_q_temp2_dn7)) * assign15380_e15185) - (assign15380_e15178 * (-((locals.var_q_invexpq_dn7 * assign15380_e15183) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn7)))))) / (assign15380_e15185 * assign15380_e15185)), ((((((4.0 * locals.var_qsqs_dn8) * locals.var_q_temp2) + (assign15380_e15176 * locals.var_q_temp2_dn8)) * assign15380_e15185) - (assign15380_e15178 * (-((locals.var_q_invexpq_dn8 * assign15380_e15183) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn8)))))) / (assign15380_e15185 * assign15380_e15185)), ((((((4.0 * locals.var_qsqs_dn9) * locals.var_q_temp2) + (assign15380_e15176 * locals.var_q_temp2_dn9)) * assign15380_e15185) - (assign15380_e15178 * (-((locals.var_q_invexpq_dn9 * assign15380_e15183) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn9)))))) / (assign15380_e15185 * assign15380_e15185)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign15380_e15188;
        locals.var_q_temp1_dn4 = assign15380_e15188_d_n4;
        locals.var_q_temp1_dn6 = assign15380_e15188_d_n6;
        locals.var_q_temp1_dn7 = assign15380_e15188_d_n7;
        locals.var_q_temp1_dn8 = assign15380_e15188_d_n8;
        locals.var_q_temp1_dn9 = assign15380_e15188_d_n9;

        let assign15390_e15191: f64 = (-0.005);
        let assign15390_e15192: f64 = if locals.var_qsqs < assign15390_e15191 { 1.0 } else { 0.0 };
        locals.var_guard598 = assign15390_e15192;

        let (assign15400_e15208, assign15400_e15208_d_n4, assign15400_e15208_d_n6, assign15400_e15208_d_n7, assign15400_e15208_d_n8, assign15400_e15208_d_n9,) = {
    if ((((locals.var_guard589 == 0.0) && (locals.var_guard592 == 0.0)) && (locals.var_guard596 == 0.0)) && (locals.var_guard598 != 0.0)) {
        let assign15400_e15205: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign15400_e15206: f64 = (assign15400_e15205).sin();
        (assign15400_e15206, ((assign15400_e15205).cos() * (0.5 * locals.var_q_rac_qsq_dn4)), ((assign15400_e15205).cos() * (0.5 * locals.var_q_rac_qsq_dn6)), ((assign15400_e15205).cos() * (0.5 * locals.var_q_rac_qsq_dn7)), ((assign15400_e15205).cos() * (0.5 * locals.var_q_rac_qsq_dn8)), ((assign15400_e15205).cos() * (0.5 * locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign15400_e15208;
        locals.var_q_temp2_dn4 = assign15400_e15208_d_n4;
        locals.var_q_temp2_dn6 = assign15400_e15208_d_n6;
        locals.var_q_temp2_dn7 = assign15400_e15208_d_n7;
        locals.var_q_temp2_dn8 = assign15400_e15208_d_n8;
        locals.var_q_temp2_dn9 = assign15400_e15208_d_n9;

        let (assign15410_e15228, assign15410_e15228_d_n4, assign15410_e15228_d_n6, assign15410_e15228_d_n7, assign15410_e15228_d_n8, assign15410_e15228_d_n9,) = {
    if ((((locals.var_guard589 == 0.0) && (locals.var_guard592 == 0.0)) && (locals.var_guard596 == 0.0)) && (locals.var_guard598 != 0.0)) {
        let assign15410_e15220: f64 = (-locals.var_qsqs);
        let assign15410_e15223: f64 = (locals.var_q_temp2 * locals.var_q_temp2);
        let assign15410_e15224: f64 = (assign15410_e15220 / assign15410_e15223);
        let assign15410_e15226: f64 = (assign15410_e15224 / locals.var_aexp1s);
        (assign15410_e15226, (((((((-locals.var_qsqs_dn4) * assign15410_e15223) - (assign15410_e15220 * ((locals.var_q_temp2_dn4 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn4)))) / (assign15410_e15223 * assign15410_e15223)) * locals.var_aexp1s) - (assign15410_e15224 * locals.var_aexp1s_dn4)) / (locals.var_aexp1s * locals.var_aexp1s)), (((((((-locals.var_qsqs_dn6) * assign15410_e15223) - (assign15410_e15220 * ((locals.var_q_temp2_dn6 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn6)))) / (assign15410_e15223 * assign15410_e15223)) * locals.var_aexp1s) - (assign15410_e15224 * locals.var_aexp1s_dn6)) / (locals.var_aexp1s * locals.var_aexp1s)), (((((((-locals.var_qsqs_dn7) * assign15410_e15223) - (assign15410_e15220 * ((locals.var_q_temp2_dn7 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn7)))) / (assign15410_e15223 * assign15410_e15223)) * locals.var_aexp1s) - (assign15410_e15224 * locals.var_aexp1s_dn7)) / (locals.var_aexp1s * locals.var_aexp1s)), (((((((-locals.var_qsqs_dn8) * assign15410_e15223) - (assign15410_e15220 * ((locals.var_q_temp2_dn8 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn8)))) / (assign15410_e15223 * assign15410_e15223)) * locals.var_aexp1s) - (assign15410_e15224 * locals.var_aexp1s_dn8)) / (locals.var_aexp1s * locals.var_aexp1s)), (((((((-locals.var_qsqs_dn9) * assign15410_e15223) - (assign15410_e15220 * ((locals.var_q_temp2_dn9 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn9)))) / (assign15410_e15223 * assign15410_e15223)) * locals.var_aexp1s) - (assign15410_e15224 * locals.var_aexp1s_dn9)) / (locals.var_aexp1s * locals.var_aexp1s)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign15410_e15228;
        locals.var_q_temp1_dn4 = assign15410_e15228_d_n4;
        locals.var_q_temp1_dn6 = assign15410_e15228_d_n6;
        locals.var_q_temp1_dn7 = assign15410_e15228_d_n7;
        locals.var_q_temp1_dn8 = assign15410_e15228_d_n8;
        locals.var_q_temp1_dn9 = assign15410_e15228_d_n9;

        let (assign15420_e15260, assign15420_e15260_d_n4, assign15420_e15260_d_n6, assign15420_e15260_d_n7, assign15420_e15260_d_n8, assign15420_e15260_d_n9,) = {
    if ((((locals.var_guard589 == 0.0) && (locals.var_guard592 == 0.0)) && (locals.var_guard596 == 0.0)) && (locals.var_guard598 == 0.0)) {
        let assign15420_e15243: f64 = (locals.var_qsqs * 0.3333333333333);
        let assign15420_e15247: f64 = (0.05 * locals.var_qsqs);
        let assign15420_e15251: f64 = (0.0396825396825397 * locals.var_qsqs);
        let assign15420_e15252: f64 = (1.0 - assign15420_e15251);
        let assign15420_e15253: f64 = (assign15420_e15247 * assign15420_e15252);
        let assign15420_e15254: f64 = (1.0 - assign15420_e15253);
        let assign15420_e15255: f64 = (assign15420_e15243 * assign15420_e15254);
        let assign15420_e15256: f64 = (4.0 - assign15420_e15255);
        let assign15420_e15258: f64 = (assign15420_e15256 / locals.var_aexp1s);
        (assign15420_e15258, ((((-(((locals.var_qsqs_dn4 * 0.3333333333333) * assign15420_e15254) + (assign15420_e15243 * (-(((0.05 * locals.var_qsqs_dn4) * assign15420_e15252) + (assign15420_e15247 * (-(0.0396825396825397 * locals.var_qsqs_dn4)))))))) * locals.var_aexp1s) - (assign15420_e15256 * locals.var_aexp1s_dn4)) / (locals.var_aexp1s * locals.var_aexp1s)), ((((-(((locals.var_qsqs_dn6 * 0.3333333333333) * assign15420_e15254) + (assign15420_e15243 * (-(((0.05 * locals.var_qsqs_dn6) * assign15420_e15252) + (assign15420_e15247 * (-(0.0396825396825397 * locals.var_qsqs_dn6)))))))) * locals.var_aexp1s) - (assign15420_e15256 * locals.var_aexp1s_dn6)) / (locals.var_aexp1s * locals.var_aexp1s)), ((((-(((locals.var_qsqs_dn7 * 0.3333333333333) * assign15420_e15254) + (assign15420_e15243 * (-(((0.05 * locals.var_qsqs_dn7) * assign15420_e15252) + (assign15420_e15247 * (-(0.0396825396825397 * locals.var_qsqs_dn7)))))))) * locals.var_aexp1s) - (assign15420_e15256 * locals.var_aexp1s_dn7)) / (locals.var_aexp1s * locals.var_aexp1s)), ((((-(((locals.var_qsqs_dn8 * 0.3333333333333) * assign15420_e15254) + (assign15420_e15243 * (-(((0.05 * locals.var_qsqs_dn8) * assign15420_e15252) + (assign15420_e15247 * (-(0.0396825396825397 * locals.var_qsqs_dn8)))))))) * locals.var_aexp1s) - (assign15420_e15256 * locals.var_aexp1s_dn8)) / (locals.var_aexp1s * locals.var_aexp1s)), ((((-(((locals.var_qsqs_dn9 * 0.3333333333333) * assign15420_e15254) + (assign15420_e15243 * (-(((0.05 * locals.var_qsqs_dn9) * assign15420_e15252) + (assign15420_e15247 * (-(0.0396825396825397 * locals.var_qsqs_dn9)))))))) * locals.var_aexp1s) - (assign15420_e15256 * locals.var_aexp1s_dn9)) / (locals.var_aexp1s * locals.var_aexp1s)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign15420_e15260;
        locals.var_q_temp1_dn4 = assign15420_e15260_d_n4;
        locals.var_q_temp1_dn6 = assign15420_e15260_d_n6;
        locals.var_q_temp1_dn7 = assign15420_e15260_d_n7;
        locals.var_q_temp1_dn8 = assign15420_e15260_d_n8;
        locals.var_q_temp1_dn9 = assign15420_e15260_d_n9;

        let (assign15430_e15276, assign15430_e15276_d_n4, assign15430_e15276_d_n6, assign15430_e15276_d_n7, assign15430_e15276_d_n8, assign15430_e15276_d_n9,) = {
    if ((locals.var_guard589 == 0.0) && (locals.var_guard592 == 0.0)) {
        let assign15430_e15268: f64 = (locals.var_k1q1s - locals.var_q_qcoth);
        let assign15430_e15271: f64 = (1.0 - locals.var_q_temp1);
        let assign15430_e15272: f64 = (assign15430_e15268 / assign15430_e15271);
        let assign15430_e15274: f64 = (assign15430_e15272 + 1e-80);
        (assign15430_e15274, ((((locals.var_k1q1s_dn4 - locals.var_q_qcoth_dn4) * assign15430_e15271) - (assign15430_e15268 * (-locals.var_q_temp1_dn4))) / (assign15430_e15271 * assign15430_e15271)), ((((locals.var_k1q1s_dn6 - locals.var_q_qcoth_dn6) * assign15430_e15271) - (assign15430_e15268 * (-locals.var_q_temp1_dn6))) / (assign15430_e15271 * assign15430_e15271)), ((((locals.var_k1q1s_dn7 - locals.var_q_qcoth_dn7) * assign15430_e15271) - (assign15430_e15268 * (-locals.var_q_temp1_dn7))) / (assign15430_e15271 * assign15430_e15271)), ((((locals.var_k1q1s_dn8 - locals.var_q_qcoth_dn8) * assign15430_e15271) - (assign15430_e15268 * (-locals.var_q_temp1_dn8))) / (assign15430_e15271 * assign15430_e15271)), ((((locals.var_k1q1s_dn9 - locals.var_q_qcoth_dn9) * assign15430_e15271) - (assign15430_e15268 * (-locals.var_q_temp1_dn9))) / (assign15430_e15271 * assign15430_e15271)),)
    } else {
        (locals.var_qis, locals.var_qis_dn4, locals.var_qis_dn6, locals.var_qis_dn7, locals.var_qis_dn8, locals.var_qis_dn9,)
    }
};
        locals.var_qis = assign15430_e15276;
        locals.var_qis_dn4 = assign15430_e15276_d_n4;
        locals.var_qis_dn6 = assign15430_e15276_d_n6;
        locals.var_qis_dn7 = assign15430_e15276_d_n7;
        locals.var_qis_dn8 = assign15430_e15276_d_n8;
        locals.var_qis_dn9 = assign15430_e15276_d_n9;

        let (assign15440_e15286, assign15440_e15286_d_n4, assign15440_e15286_d_n6, assign15440_e15286_d_n7, assign15440_e15286_d_n8, assign15440_e15286_d_n9,) = {
    if ((locals.var_guard589 == 0.0) && (locals.var_guard592 == 0.0)) {
        let assign15440_e15284: f64 = (locals.var_qis - locals.var_k1q1s);
        (assign15440_e15284, (locals.var_qis_dn4 - locals.var_k1q1s_dn4), (locals.var_qis_dn6 - locals.var_k1q1s_dn6), (locals.var_qis_dn7 - locals.var_k1q1s_dn7), (locals.var_qis_dn8 - locals.var_k1q1s_dn8), (locals.var_qis_dn9 - locals.var_k1q1s_dn9),)
    } else {
        (locals.var_k2q2s, locals.var_k2q2s_dn4, locals.var_k2q2s_dn6, locals.var_k2q2s_dn7, locals.var_k2q2s_dn8, locals.var_k2q2s_dn9,)
    }
};
        locals.var_k2q2s = assign15440_e15286;
        locals.var_k2q2s_dn4 = assign15440_e15286_d_n4;
        locals.var_k2q2s_dn6 = assign15440_e15286_d_n6;
        locals.var_k2q2s_dn7 = assign15440_e15286_d_n7;
        locals.var_k2q2s_dn8 = assign15440_e15286_d_n8;
        locals.var_k2q2s_dn9 = assign15440_e15286_d_n9;

        let (assign15450_e15296, assign15450_e15296_d_n4, assign15450_e15296_d_n6, assign15450_e15296_d_n7, assign15450_e15296_d_n8, assign15450_e15296_d_n9,) = {
    if ((locals.var_guard589 == 0.0) && (locals.var_guard592 == 0.0)) {
        let assign15450_e15294: f64 = (locals.var_k2q2s / locals.var_k2);
        (assign15450_e15294, (((locals.var_k2q2s_dn4 * locals.var_k2) - (locals.var_k2q2s * locals.var_k2_dn4)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2s_dn6 * locals.var_k2) - (locals.var_k2q2s * locals.var_k2_dn6)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2s_dn7 * locals.var_k2) - (locals.var_k2q2s * locals.var_k2_dn7)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2s_dn8 * locals.var_k2) - (locals.var_k2q2s * locals.var_k2_dn8)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2s_dn9 * locals.var_k2) - (locals.var_k2q2s * locals.var_k2_dn9)) / (locals.var_k2 * locals.var_k2)),)
    } else {
        (locals.var_q2s, locals.var_q2s_dn4, locals.var_q2s_dn6, locals.var_q2s_dn7, locals.var_q2s_dn8, locals.var_q2s_dn9,)
    }
};
        locals.var_q2s = assign15450_e15296;
        locals.var_q2s_dn4 = assign15450_e15296_d_n4;
        locals.var_q2s_dn6 = assign15450_e15296_d_n6;
        locals.var_q2s_dn7 = assign15450_e15296_d_n7;
        locals.var_q2s_dn8 = assign15450_e15296_d_n8;
        locals.var_q2s_dn9 = assign15450_e15296_d_n9;

        let assign15460_e15299: f64 = (locals.var_xg2x - locals.var_q2s);
        let assign15460_e15301: f64 = assign15460_e15299;
        let assign15460_e15303: f64 = if assign15460_e15301 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard599 = assign15460_e15303;

        let (assign15470_e15312, assign15470_e15312_d_n4, assign15470_e15312_d_n6, assign15470_e15312_d_n7, assign15470_e15312_d_n8, assign15470_e15312_d_n9,) = {
    if (locals.var_guard599 != 0.0) {
        let assign15470_e15307: f64 = (locals.var_xg2x - locals.var_q2s);
        let assign15470_e15309: f64 = assign15470_e15307;
        let assign15470_e15310: f64 = (assign15470_e15309).exp();
        (assign15470_e15310, (assign15470_e15310 * (locals.var_xg2x_dn4 - locals.var_q2s_dn4)), (assign15470_e15310 * (locals.var_xg2x_dn6 - locals.var_q2s_dn6)), (assign15470_e15310 * (locals.var_xg2x_dn7 - locals.var_q2s_dn7)), (assign15470_e15310 * (locals.var_xg2x_dn8 - locals.var_q2s_dn8)), (assign15470_e15310 * (locals.var_xg2x_dn9 - locals.var_q2s_dn9)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign15470_e15312;
        locals.var_q_temp1_dn4 = assign15470_e15312_d_n4;
        locals.var_q_temp1_dn6 = assign15470_e15312_d_n6;
        locals.var_q_temp1_dn7 = assign15470_e15312_d_n7;
        locals.var_q_temp1_dn8 = assign15470_e15312_d_n8;
        locals.var_q_temp1_dn9 = assign15470_e15312_d_n9;

        let (assign15480_e15351, assign15480_e15351_d_n4, assign15480_e15351_d_n6, assign15480_e15351_d_n7, assign15480_e15351_d_n8, assign15480_e15351_d_n9,) = {
    if (locals.var_guard599 == 0.0) {
        let assign15480_e15319: f64 = (locals.var_xg2x - locals.var_q2s);
        let assign15480_e15321: f64 = assign15480_e15319;
        let assign15480_e15323: f64 = (assign15480_e15321 - 80.0);
        let assign15480_e15328: f64 = (locals.var_xg2x - locals.var_q2s);
        let assign15480_e15330: f64 = assign15480_e15328;
        let assign15480_e15332: f64 = (assign15480_e15330 - 80.0);
        let assign15480_e15333: f64 = (0.5 * assign15480_e15332);
        let assign15480_e15337: f64 = (locals.var_xg2x - locals.var_q2s);
        let assign15480_e15339: f64 = assign15480_e15337;
        let assign15480_e15341: f64 = (assign15480_e15339 - 80.0);
        let assign15480_e15343: f64 = (assign15480_e15341 * 0.3333333333333);
        let assign15480_e15344: f64 = (1.0 + assign15480_e15343);
        let assign15480_e15345: f64 = (assign15480_e15333 * assign15480_e15344);
        let assign15480_e15346: f64 = (1.0 + assign15480_e15345);
        let assign15480_e15347: f64 = (assign15480_e15323 * assign15480_e15346);
        let assign15480_e15348: f64 = (1.0 + assign15480_e15347);
        let assign15480_e15349: f64 = (5.54062e34 * assign15480_e15348);
        (assign15480_e15349, (5.54062e34 * (((locals.var_xg2x_dn4 - locals.var_q2s_dn4) * assign15480_e15346) + (assign15480_e15323 * (((0.5 * (locals.var_xg2x_dn4 - locals.var_q2s_dn4)) * assign15480_e15344) + (assign15480_e15333 * ((locals.var_xg2x_dn4 - locals.var_q2s_dn4) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg2x_dn6 - locals.var_q2s_dn6) * assign15480_e15346) + (assign15480_e15323 * (((0.5 * (locals.var_xg2x_dn6 - locals.var_q2s_dn6)) * assign15480_e15344) + (assign15480_e15333 * ((locals.var_xg2x_dn6 - locals.var_q2s_dn6) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg2x_dn7 - locals.var_q2s_dn7) * assign15480_e15346) + (assign15480_e15323 * (((0.5 * (locals.var_xg2x_dn7 - locals.var_q2s_dn7)) * assign15480_e15344) + (assign15480_e15333 * ((locals.var_xg2x_dn7 - locals.var_q2s_dn7) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg2x_dn8 - locals.var_q2s_dn8) * assign15480_e15346) + (assign15480_e15323 * (((0.5 * (locals.var_xg2x_dn8 - locals.var_q2s_dn8)) * assign15480_e15344) + (assign15480_e15333 * ((locals.var_xg2x_dn8 - locals.var_q2s_dn8) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg2x_dn9 - locals.var_q2s_dn9) * assign15480_e15346) + (assign15480_e15323 * (((0.5 * (locals.var_xg2x_dn9 - locals.var_q2s_dn9)) * assign15480_e15344) + (assign15480_e15333 * ((locals.var_xg2x_dn9 - locals.var_q2s_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign15480_e15351;
        locals.var_q_temp1_dn4 = assign15480_e15351_d_n4;
        locals.var_q_temp1_dn6 = assign15480_e15351_d_n6;
        locals.var_q_temp1_dn7 = assign15480_e15351_d_n7;
        locals.var_q_temp1_dn8 = assign15480_e15351_d_n8;
        locals.var_q_temp1_dn9 = assign15480_e15351_d_n9;

        let assign15490_e15354: f64 = (locals.var_a0 * locals.var_q_temp1);
        locals.var_aexp2s = assign15490_e15354;
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

        let assign15560_e15363: f64 = if locals.var_qis > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard600 = assign15560_e15363;

        let (assign15570_e15369, assign15570_e15369_d_n4, assign15570_e15369_d_n6, assign15570_e15369_d_n7, assign15570_e15369_d_n8, assign15570_e15369_d_n9,) = {
    if (locals.var_guard600 != 0.0) {
        let assign15570_e15367: f64 = (locals.var_aexp1s * locals.var_inv_k1);
        (assign15570_e15367, ((locals.var_aexp1s_dn4 * locals.var_inv_k1) + (locals.var_aexp1s * locals.var_inv_k1_dn4)), ((locals.var_aexp1s_dn6 * locals.var_inv_k1) + (locals.var_aexp1s * locals.var_inv_k1_dn6)), ((locals.var_aexp1s_dn7 * locals.var_inv_k1) + (locals.var_aexp1s * locals.var_inv_k1_dn7)), ((locals.var_aexp1s_dn8 * locals.var_inv_k1) + (locals.var_aexp1s * locals.var_inv_k1_dn8)), ((locals.var_aexp1s_dn9 * locals.var_inv_k1) + (locals.var_aexp1s * locals.var_inv_k1_dn9)),)
    } else {
        (locals.var_b1s, locals.var_b1s_dn4, locals.var_b1s_dn6, locals.var_b1s_dn7, locals.var_b1s_dn8, locals.var_b1s_dn9,)
    }
};
        locals.var_b1s = assign15570_e15369;
        locals.var_b1s_dn4 = assign15570_e15369_d_n4;
        locals.var_b1s_dn6 = assign15570_e15369_d_n6;
        locals.var_b1s_dn7 = assign15570_e15369_d_n7;
        locals.var_b1s_dn8 = assign15570_e15369_d_n8;
        locals.var_b1s_dn9 = assign15570_e15369_d_n9;

        let (assign15580_e15375, assign15580_e15375_d_n4, assign15580_e15375_d_n6, assign15580_e15375_d_n7, assign15580_e15375_d_n8, assign15580_e15375_d_n9,) = {
    if (locals.var_guard600 != 0.0) {
        let assign15580_e15373: f64 = (locals.var_aexp2s * locals.var_inv_k2);
        (assign15580_e15373, ((locals.var_aexp2s_dn4 * locals.var_inv_k2) + (locals.var_aexp2s * locals.var_inv_k2_dn4)), ((locals.var_aexp2s_dn6 * locals.var_inv_k2) + (locals.var_aexp2s * locals.var_inv_k2_dn6)), ((locals.var_aexp2s_dn7 * locals.var_inv_k2) + (locals.var_aexp2s * locals.var_inv_k2_dn7)), ((locals.var_aexp2s_dn8 * locals.var_inv_k2) + (locals.var_aexp2s * locals.var_inv_k2_dn8)), ((locals.var_aexp2s_dn9 * locals.var_inv_k2) + (locals.var_aexp2s * locals.var_inv_k2_dn9)),)
    } else {
        (locals.var_b2s, locals.var_b2s_dn4, locals.var_b2s_dn6, locals.var_b2s_dn7, locals.var_b2s_dn8, locals.var_b2s_dn9,)
    }
};
        locals.var_b2s = assign15580_e15375;
        locals.var_b2s_dn4 = assign15580_e15375_d_n4;
        locals.var_b2s_dn6 = assign15580_e15375_d_n6;
        locals.var_b2s_dn7 = assign15580_e15375_d_n7;
        locals.var_b2s_dn8 = assign15580_e15375_d_n8;
        locals.var_b2s_dn9 = assign15580_e15375_d_n9;

        let (assign15590_e15383, assign15590_e15383_d_n4, assign15590_e15383_d_n6, assign15590_e15383_d_n7, assign15590_e15383_d_n8, assign15590_e15383_d_n9,) = {
    if (locals.var_guard600 != 0.0) {
        let assign15590_e15380: f64 = (2.0 * locals.var_k1q1s);
        let assign15590_e15381: f64 = (locals.var_b1s + assign15590_e15380);
        (assign15590_e15381, (locals.var_b1s_dn4 + (2.0 * locals.var_k1q1s_dn4)), (locals.var_b1s_dn6 + (2.0 * locals.var_k1q1s_dn6)), (locals.var_b1s_dn7 + (2.0 * locals.var_k1q1s_dn7)), (locals.var_b1s_dn8 + (2.0 * locals.var_k1q1s_dn8)), (locals.var_b1s_dn9 + (2.0 * locals.var_k1q1s_dn9)),)
    } else {
        (locals.var_a1s, locals.var_a1s_dn4, locals.var_a1s_dn6, locals.var_a1s_dn7, locals.var_a1s_dn8, locals.var_a1s_dn9,)
    }
};
        locals.var_a1s = assign15590_e15383;
        locals.var_a1s_dn4 = assign15590_e15383_d_n4;
        locals.var_a1s_dn6 = assign15590_e15383_d_n6;
        locals.var_a1s_dn7 = assign15590_e15383_d_n7;
        locals.var_a1s_dn8 = assign15590_e15383_d_n8;
        locals.var_a1s_dn9 = assign15590_e15383_d_n9;

        let (assign15600_e15391, assign15600_e15391_d_n4, assign15600_e15391_d_n6, assign15600_e15391_d_n7, assign15600_e15391_d_n8, assign15600_e15391_d_n9,) = {
    if (locals.var_guard600 != 0.0) {
        let assign15600_e15388: f64 = (2.0 * locals.var_k2q2s);
        let assign15600_e15389: f64 = (locals.var_b2s + assign15600_e15388);
        (assign15600_e15389, (locals.var_b2s_dn4 + (2.0 * locals.var_k2q2s_dn4)), (locals.var_b2s_dn6 + (2.0 * locals.var_k2q2s_dn6)), (locals.var_b2s_dn7 + (2.0 * locals.var_k2q2s_dn7)), (locals.var_b2s_dn8 + (2.0 * locals.var_k2q2s_dn8)), (locals.var_b2s_dn9 + (2.0 * locals.var_k2q2s_dn9)),)
    } else {
        (locals.var_a2s, locals.var_a2s_dn4, locals.var_a2s_dn6, locals.var_a2s_dn7, locals.var_a2s_dn8, locals.var_a2s_dn9,)
    }
};
        locals.var_a2s = assign15600_e15391;
        locals.var_a2s_dn4 = assign15600_e15391_d_n4;
        locals.var_a2s_dn6 = assign15600_e15391_d_n6;
        locals.var_a2s_dn7 = assign15600_e15391_d_n7;
        locals.var_a2s_dn8 = assign15600_e15391_d_n8;
        locals.var_a2s_dn9 = assign15600_e15391_d_n9;

        let (assign15610_e15401, assign15610_e15401_d_n4, assign15610_e15401_d_n6, assign15610_e15401_d_n7, assign15610_e15401_d_n8, assign15610_e15401_d_n9,) = {
    if (locals.var_guard600 != 0.0) {
        let assign15610_e15395: f64 = (2.0 * locals.var_qis);
        let assign15610_e15397: f64 = (assign15610_e15395 + locals.var_b1s);
        let assign15610_e15399: f64 = (assign15610_e15397 + locals.var_b2s);
        (assign15610_e15399, (((2.0 * locals.var_qis_dn4) + locals.var_b1s_dn4) + locals.var_b2s_dn4), (((2.0 * locals.var_qis_dn6) + locals.var_b1s_dn6) + locals.var_b2s_dn6), (((2.0 * locals.var_qis_dn7) + locals.var_b1s_dn7) + locals.var_b2s_dn7), (((2.0 * locals.var_qis_dn8) + locals.var_b1s_dn8) + locals.var_b2s_dn8), (((2.0 * locals.var_qis_dn9) + locals.var_b1s_dn9) + locals.var_b2s_dn9),)
    } else {
        (locals.var_sums, locals.var_sums_dn4, locals.var_sums_dn6, locals.var_sums_dn7, locals.var_sums_dn8, locals.var_sums_dn9,)
    }
};
        locals.var_sums = assign15610_e15401;
        locals.var_sums_dn4 = assign15610_e15401_d_n4;
        locals.var_sums_dn6 = assign15610_e15401_d_n6;
        locals.var_sums_dn7 = assign15610_e15401_d_n7;
        locals.var_sums_dn8 = assign15610_e15401_d_n8;
        locals.var_sums_dn9 = assign15610_e15401_d_n9;

        let assign15620_e15403: f64 = (locals.var_qsqs).abs();
        let assign15620_e15405: f64 = if assign15620_e15403 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard601 = assign15620_e15405;

        let (assign15630_e15429, assign15630_e15429_d_n4, assign15630_e15429_d_n6, assign15630_e15429_d_n7, assign15630_e15429_d_n8, assign15630_e15429_d_n9,) = {
    if ((locals.var_guard600 != 0.0) && (locals.var_guard601 != 0.0)) {
        let assign15630_e15411: f64 = (locals.var_a1s * locals.var_a2s);
        let assign15630_e15415: f64 = (locals.var_q1s + 2.0);
        let assign15630_e15416: f64 = (2.0 * assign15630_e15415);
        let assign15630_e15418: f64 = (assign15630_e15416 * locals.var_a2s);
        let assign15630_e15419: f64 = (assign15630_e15411 + assign15630_e15418);
        let assign15630_e15423: f64 = (locals.var_q2s + 2.0);
        let assign15630_e15424: f64 = (2.0 * assign15630_e15423);
        let assign15630_e15426: f64 = (assign15630_e15424 * locals.var_a1s);
        let assign15630_e15427: f64 = (assign15630_e15419 + assign15630_e15426);
        (assign15630_e15427, ((((locals.var_a1s_dn4 * locals.var_a2s) + (locals.var_a1s * locals.var_a2s_dn4)) + (((2.0 * locals.var_q1s_dn4) * locals.var_a2s) + (assign15630_e15416 * locals.var_a2s_dn4))) + (((2.0 * locals.var_q2s_dn4) * locals.var_a1s) + (assign15630_e15424 * locals.var_a1s_dn4))), ((((locals.var_a1s_dn6 * locals.var_a2s) + (locals.var_a1s * locals.var_a2s_dn6)) + (((2.0 * locals.var_q1s_dn6) * locals.var_a2s) + (assign15630_e15416 * locals.var_a2s_dn6))) + (((2.0 * locals.var_q2s_dn6) * locals.var_a1s) + (assign15630_e15424 * locals.var_a1s_dn6))), ((((locals.var_a1s_dn7 * locals.var_a2s) + (locals.var_a1s * locals.var_a2s_dn7)) + (((2.0 * locals.var_q1s_dn7) * locals.var_a2s) + (assign15630_e15416 * locals.var_a2s_dn7))) + (((2.0 * locals.var_q2s_dn7) * locals.var_a1s) + (assign15630_e15424 * locals.var_a1s_dn7))), ((((locals.var_a1s_dn8 * locals.var_a2s) + (locals.var_a1s * locals.var_a2s_dn8)) + (((2.0 * locals.var_q1s_dn8) * locals.var_a2s) + (assign15630_e15416 * locals.var_a2s_dn8))) + (((2.0 * locals.var_q2s_dn8) * locals.var_a1s) + (assign15630_e15424 * locals.var_a1s_dn8))), ((((locals.var_a1s_dn9 * locals.var_a2s) + (locals.var_a1s * locals.var_a2s_dn9)) + (((2.0 * locals.var_q1s_dn9) * locals.var_a2s) + (assign15630_e15416 * locals.var_a2s_dn9))) + (((2.0 * locals.var_q2s_dn9) * locals.var_a1s) + (assign15630_e15424 * locals.var_a1s_dn9))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign15630_e15429;
        locals.var_temp1_dn4 = assign15630_e15429_d_n4;
        locals.var_temp1_dn6 = assign15630_e15429_d_n6;
        locals.var_temp1_dn7 = assign15630_e15429_d_n7;
        locals.var_temp1_dn8 = assign15630_e15429_d_n8;
        locals.var_temp1_dn9 = assign15630_e15429_d_n9;

        let (assign15640_e15444, assign15640_e15444_d_n4, assign15640_e15444_d_n6, assign15640_e15444_d_n7, assign15640_e15444_d_n8, assign15640_e15444_d_n9,) = {
    if ((locals.var_guard600 != 0.0) && (locals.var_guard601 != 0.0)) {
        let assign15640_e15434: f64 = (-4.0);
        let assign15640_e15436: f64 = (assign15640_e15434 * locals.var_qsqs);
        let assign15640_e15438: f64 = (assign15640_e15436 * locals.var_sums);
        let assign15640_e15441: f64 = (locals.var_qis * locals.var_temp1);
        let assign15640_e15442: f64 = (assign15640_e15438 / assign15640_e15441);
        (assign15640_e15442, ((((((assign15640_e15434 * locals.var_qsqs_dn4) * locals.var_sums) + (assign15640_e15436 * locals.var_sums_dn4)) * assign15640_e15441) - (assign15640_e15438 * ((locals.var_qis_dn4 * locals.var_temp1) + (locals.var_qis * locals.var_temp1_dn4)))) / (assign15640_e15441 * assign15640_e15441)), ((((((assign15640_e15434 * locals.var_qsqs_dn6) * locals.var_sums) + (assign15640_e15436 * locals.var_sums_dn6)) * assign15640_e15441) - (assign15640_e15438 * ((locals.var_qis_dn6 * locals.var_temp1) + (locals.var_qis * locals.var_temp1_dn6)))) / (assign15640_e15441 * assign15640_e15441)), ((((((assign15640_e15434 * locals.var_qsqs_dn7) * locals.var_sums) + (assign15640_e15436 * locals.var_sums_dn7)) * assign15640_e15441) - (assign15640_e15438 * ((locals.var_qis_dn7 * locals.var_temp1) + (locals.var_qis * locals.var_temp1_dn7)))) / (assign15640_e15441 * assign15640_e15441)), ((((((assign15640_e15434 * locals.var_qsqs_dn8) * locals.var_sums) + (assign15640_e15436 * locals.var_sums_dn8)) * assign15640_e15441) - (assign15640_e15438 * ((locals.var_qis_dn8 * locals.var_temp1) + (locals.var_qis * locals.var_temp1_dn8)))) / (assign15640_e15441 * assign15640_e15441)), ((((((assign15640_e15434 * locals.var_qsqs_dn9) * locals.var_sums) + (assign15640_e15436 * locals.var_sums_dn9)) * assign15640_e15441) - (assign15640_e15438 * ((locals.var_qis_dn9 * locals.var_temp1) + (locals.var_qis * locals.var_temp1_dn9)))) / (assign15640_e15441 * assign15640_e15441)),)
    } else {
        (locals.var_dqsqs_dxn_qi, locals.var_dqsqs_dxn_qi_dn4, locals.var_dqsqs_dxn_qi_dn6, locals.var_dqsqs_dxn_qi_dn7, locals.var_dqsqs_dxn_qi_dn8, locals.var_dqsqs_dxn_qi_dn9,)
    }
};
        locals.var_dqsqs_dxn_qi = assign15640_e15444;
        locals.var_dqsqs_dxn_qi_dn4 = assign15640_e15444_d_n4;
        locals.var_dqsqs_dxn_qi_dn6 = assign15640_e15444_d_n6;
        locals.var_dqsqs_dxn_qi_dn7 = assign15640_e15444_d_n7;
        locals.var_dqsqs_dxn_qi_dn8 = assign15640_e15444_d_n8;
        locals.var_dqsqs_dxn_qi_dn9 = assign15640_e15444_d_n9;

        let (assign15650_e15469, assign15650_e15469_d_n4, assign15650_e15469_d_n6, assign15650_e15469_d_n7, assign15650_e15469_d_n8, assign15650_e15469_d_n9,) = {
    if ((locals.var_guard600 != 0.0) && (locals.var_guard601 == 0.0)) {
        let assign15650_e15453: f64 = (locals.var_qsqs * 0.0333333333333);
        let assign15650_e15457: f64 = (locals.var_qsqs * 0.0357142857143);
        let assign15650_e15461: f64 = (locals.var_qsqs * 0.0333333333333);
        let assign15650_e15462: f64 = (1.0 - assign15650_e15461);
        let assign15650_e15463: f64 = (assign15650_e15457 * assign15650_e15462);
        let assign15650_e15464: f64 = (1.0 - assign15650_e15463);
        let assign15650_e15465: f64 = (assign15650_e15453 * assign15650_e15464);
        let assign15650_e15466: f64 = (1.0 - assign15650_e15465);
        let assign15650_e15467: f64 = (0.1666666666667 * assign15650_e15466);
        (assign15650_e15467, (0.1666666666667 * (-(((locals.var_qsqs_dn4 * 0.0333333333333) * assign15650_e15464) + (assign15650_e15453 * (-(((locals.var_qsqs_dn4 * 0.0357142857143) * assign15650_e15462) + (assign15650_e15457 * (-(locals.var_qsqs_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_qsqs_dn6 * 0.0333333333333) * assign15650_e15464) + (assign15650_e15453 * (-(((locals.var_qsqs_dn6 * 0.0357142857143) * assign15650_e15462) + (assign15650_e15457 * (-(locals.var_qsqs_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_qsqs_dn7 * 0.0333333333333) * assign15650_e15464) + (assign15650_e15453 * (-(((locals.var_qsqs_dn7 * 0.0357142857143) * assign15650_e15462) + (assign15650_e15457 * (-(locals.var_qsqs_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_qsqs_dn8 * 0.0333333333333) * assign15650_e15464) + (assign15650_e15453 * (-(((locals.var_qsqs_dn8 * 0.0357142857143) * assign15650_e15462) + (assign15650_e15457 * (-(locals.var_qsqs_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_qsqs_dn9 * 0.0333333333333) * assign15650_e15464) + (assign15650_e15453 * (-(((locals.var_qsqs_dn9 * 0.0357142857143) * assign15650_e15462) + (assign15650_e15457 * (-(locals.var_qsqs_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign15650_e15469;
        locals.var_temp1_dn4 = assign15650_e15469_d_n4;
        locals.var_temp1_dn6 = assign15650_e15469_d_n6;
        locals.var_temp1_dn7 = assign15650_e15469_d_n7;
        locals.var_temp1_dn8 = assign15650_e15469_d_n8;
        locals.var_temp1_dn9 = assign15650_e15469_d_n9;

        let (assign15660_e15494, assign15660_e15494_d_n4, assign15660_e15494_d_n6, assign15660_e15494_d_n7, assign15660_e15494_d_n8, assign15660_e15494_d_n9,) = {
    if ((locals.var_guard600 != 0.0) && (locals.var_guard601 == 0.0)) {
        let assign15660_e15476: f64 = (locals.var_a1s * locals.var_aexp1s);
        let assign15660_e15479: f64 = (locals.var_a2s * locals.var_aexp2s);
        let assign15660_e15480: f64 = (assign15660_e15476 + assign15660_e15479);
        let assign15660_e15483: f64 = (locals.var_a1s * locals.var_a2s);
        let assign15660_e15485: f64 = (assign15660_e15483 * locals.var_qis);
        let assign15660_e15489: f64 = (locals.var_qis * locals.var_temp1);
        let assign15660_e15490: f64 = (1.0 + assign15660_e15489);
        let assign15660_e15491: f64 = (assign15660_e15485 * assign15660_e15490);
        let assign15660_e15492: f64 = (assign15660_e15480 + assign15660_e15491);
        (assign15660_e15492, ((((locals.var_a1s_dn4 * locals.var_aexp1s) + (locals.var_a1s * locals.var_aexp1s_dn4)) + ((locals.var_a2s_dn4 * locals.var_aexp2s) + (locals.var_a2s * locals.var_aexp2s_dn4))) + ((((((locals.var_a1s_dn4 * locals.var_a2s) + (locals.var_a1s * locals.var_a2s_dn4)) * locals.var_qis) + (assign15660_e15483 * locals.var_qis_dn4)) * assign15660_e15490) + (assign15660_e15485 * ((locals.var_qis_dn4 * locals.var_temp1) + (locals.var_qis * locals.var_temp1_dn4))))), ((((locals.var_a1s_dn6 * locals.var_aexp1s) + (locals.var_a1s * locals.var_aexp1s_dn6)) + ((locals.var_a2s_dn6 * locals.var_aexp2s) + (locals.var_a2s * locals.var_aexp2s_dn6))) + ((((((locals.var_a1s_dn6 * locals.var_a2s) + (locals.var_a1s * locals.var_a2s_dn6)) * locals.var_qis) + (assign15660_e15483 * locals.var_qis_dn6)) * assign15660_e15490) + (assign15660_e15485 * ((locals.var_qis_dn6 * locals.var_temp1) + (locals.var_qis * locals.var_temp1_dn6))))), ((((locals.var_a1s_dn7 * locals.var_aexp1s) + (locals.var_a1s * locals.var_aexp1s_dn7)) + ((locals.var_a2s_dn7 * locals.var_aexp2s) + (locals.var_a2s * locals.var_aexp2s_dn7))) + ((((((locals.var_a1s_dn7 * locals.var_a2s) + (locals.var_a1s * locals.var_a2s_dn7)) * locals.var_qis) + (assign15660_e15483 * locals.var_qis_dn7)) * assign15660_e15490) + (assign15660_e15485 * ((locals.var_qis_dn7 * locals.var_temp1) + (locals.var_qis * locals.var_temp1_dn7))))), ((((locals.var_a1s_dn8 * locals.var_aexp1s) + (locals.var_a1s * locals.var_aexp1s_dn8)) + ((locals.var_a2s_dn8 * locals.var_aexp2s) + (locals.var_a2s * locals.var_aexp2s_dn8))) + ((((((locals.var_a1s_dn8 * locals.var_a2s) + (locals.var_a1s * locals.var_a2s_dn8)) * locals.var_qis) + (assign15660_e15483 * locals.var_qis_dn8)) * assign15660_e15490) + (assign15660_e15485 * ((locals.var_qis_dn8 * locals.var_temp1) + (locals.var_qis * locals.var_temp1_dn8))))), ((((locals.var_a1s_dn9 * locals.var_aexp1s) + (locals.var_a1s * locals.var_aexp1s_dn9)) + ((locals.var_a2s_dn9 * locals.var_aexp2s) + (locals.var_a2s * locals.var_aexp2s_dn9))) + ((((((locals.var_a1s_dn9 * locals.var_a2s) + (locals.var_a1s * locals.var_a2s_dn9)) * locals.var_qis) + (assign15660_e15483 * locals.var_qis_dn9)) * assign15660_e15490) + (assign15660_e15485 * ((locals.var_qis_dn9 * locals.var_temp1) + (locals.var_qis * locals.var_temp1_dn9))))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign15660_e15494;
        locals.var_temp2_dn4 = assign15660_e15494_d_n4;
        locals.var_temp2_dn6 = assign15660_e15494_d_n6;
        locals.var_temp2_dn7 = assign15660_e15494_d_n7;
        locals.var_temp2_dn8 = assign15660_e15494_d_n8;
        locals.var_temp2_dn9 = assign15660_e15494_d_n9;

        let (assign15670_e15509, assign15670_e15509_d_n4, assign15670_e15509_d_n6, assign15670_e15509_d_n7, assign15670_e15509_d_n8, assign15670_e15509_d_n9,) = {
    if ((locals.var_guard600 != 0.0) && (locals.var_guard601 == 0.0)) {
        let assign15670_e15501: f64 = (locals.var_aexp1s * locals.var_aexp2s);
        let assign15670_e15503: f64 = (assign15670_e15501 * locals.var_sums);
        let assign15670_e15506: f64 = (locals.var_qis * locals.var_temp2);
        let assign15670_e15507: f64 = (assign15670_e15503 / assign15670_e15506);
        (assign15670_e15507, (((((((locals.var_aexp1s_dn4 * locals.var_aexp2s) + (locals.var_aexp1s * locals.var_aexp2s_dn4)) * locals.var_sums) + (assign15670_e15501 * locals.var_sums_dn4)) * assign15670_e15506) - (assign15670_e15503 * ((locals.var_qis_dn4 * locals.var_temp2) + (locals.var_qis * locals.var_temp2_dn4)))) / (assign15670_e15506 * assign15670_e15506)), (((((((locals.var_aexp1s_dn6 * locals.var_aexp2s) + (locals.var_aexp1s * locals.var_aexp2s_dn6)) * locals.var_sums) + (assign15670_e15501 * locals.var_sums_dn6)) * assign15670_e15506) - (assign15670_e15503 * ((locals.var_qis_dn6 * locals.var_temp2) + (locals.var_qis * locals.var_temp2_dn6)))) / (assign15670_e15506 * assign15670_e15506)), (((((((locals.var_aexp1s_dn7 * locals.var_aexp2s) + (locals.var_aexp1s * locals.var_aexp2s_dn7)) * locals.var_sums) + (assign15670_e15501 * locals.var_sums_dn7)) * assign15670_e15506) - (assign15670_e15503 * ((locals.var_qis_dn7 * locals.var_temp2) + (locals.var_qis * locals.var_temp2_dn7)))) / (assign15670_e15506 * assign15670_e15506)), (((((((locals.var_aexp1s_dn8 * locals.var_aexp2s) + (locals.var_aexp1s * locals.var_aexp2s_dn8)) * locals.var_sums) + (assign15670_e15501 * locals.var_sums_dn8)) * assign15670_e15506) - (assign15670_e15503 * ((locals.var_qis_dn8 * locals.var_temp2) + (locals.var_qis * locals.var_temp2_dn8)))) / (assign15670_e15506 * assign15670_e15506)), (((((((locals.var_aexp1s_dn9 * locals.var_aexp2s) + (locals.var_aexp1s * locals.var_aexp2s_dn9)) * locals.var_sums) + (assign15670_e15501 * locals.var_sums_dn9)) * assign15670_e15506) - (assign15670_e15503 * ((locals.var_qis_dn9 * locals.var_temp2) + (locals.var_qis * locals.var_temp2_dn9)))) / (assign15670_e15506 * assign15670_e15506)),)
    } else {
        (locals.var_dqsqs_dxn_qi, locals.var_dqsqs_dxn_qi_dn4, locals.var_dqsqs_dxn_qi_dn6, locals.var_dqsqs_dxn_qi_dn7, locals.var_dqsqs_dxn_qi_dn8, locals.var_dqsqs_dxn_qi_dn9,)
    }
};
        locals.var_dqsqs_dxn_qi = assign15670_e15509;
        locals.var_dqsqs_dxn_qi_dn4 = assign15670_e15509_d_n4;
        locals.var_dqsqs_dxn_qi_dn6 = assign15670_e15509_d_n6;
        locals.var_dqsqs_dxn_qi_dn7 = assign15670_e15509_d_n7;
        locals.var_dqsqs_dxn_qi_dn8 = assign15670_e15509_d_n8;
        locals.var_dqsqs_dxn_qi_dn9 = assign15670_e15509_d_n9;

        let assign15680_e15511: f64 = (locals.var_qis).ln();
        locals.var_xdrifts = assign15680_e15511;
        locals.var_xdrifts_dn4 = (locals.var_qis_dn4 / locals.var_qis);
        locals.var_xdrifts_dn6 = (locals.var_qis_dn6 / locals.var_qis);
        locals.var_xdrifts_dn7 = (locals.var_qis_dn7 / locals.var_qis);
        locals.var_xdrifts_dn8 = (locals.var_qis_dn8 / locals.var_qis);
        locals.var_xdrifts_dn9 = (locals.var_qis_dn9 / locals.var_qis);

        let assign15690_e15514: f64 = (locals.var_k1q1s / 2.0);
        let assign15690_e15516: f64 = if assign15690_e15514 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard602 = assign15690_e15516;

        let (assign15700_e15526, assign15700_e15526_d_n4, assign15700_e15526_d_n6, assign15700_e15526_d_n7, assign15700_e15526_d_n8, assign15700_e15526_d_n9,) = {
    if (locals.var_guard602 != 0.0) {
        let assign15700_e15521: f64 = (locals.var_k1q1s / 2.0);
        let assign15700_e15522: f64 = (assign15700_e15521).exp();
        let assign15700_e15523: f64 = (1.0 + assign15700_e15522);
        let assign15700_e15524: f64 = (assign15700_e15523).ln();
        (assign15700_e15524, ((assign15700_e15522 * (locals.var_k1q1s_dn4 / 2.0)) / assign15700_e15523), ((assign15700_e15522 * (locals.var_k1q1s_dn6 / 2.0)) / assign15700_e15523), ((assign15700_e15522 * (locals.var_k1q1s_dn7 / 2.0)) / assign15700_e15523), ((assign15700_e15522 * (locals.var_k1q1s_dn8 / 2.0)) / assign15700_e15523), ((assign15700_e15522 * (locals.var_k1q1s_dn9 / 2.0)) / assign15700_e15523),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign15700_e15526;
        locals.var_temp1_dn4 = assign15700_e15526_d_n4;
        locals.var_temp1_dn6 = assign15700_e15526_d_n6;
        locals.var_temp1_dn7 = assign15700_e15526_d_n7;
        locals.var_temp1_dn8 = assign15700_e15526_d_n8;
        locals.var_temp1_dn9 = assign15700_e15526_d_n9;

        let (assign15710_e15533, assign15710_e15533_d_n4, assign15710_e15533_d_n6, assign15710_e15533_d_n7, assign15710_e15533_d_n8, assign15710_e15533_d_n9,) = {
    if (locals.var_guard602 == 0.0) {
        let assign15710_e15531: f64 = (locals.var_k1q1s / 2.0);
        (assign15710_e15531, (locals.var_k1q1s_dn4 / 2.0), (locals.var_k1q1s_dn6 / 2.0), (locals.var_k1q1s_dn7 / 2.0), (locals.var_k1q1s_dn8 / 2.0), (locals.var_k1q1s_dn9 / 2.0),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign15710_e15533;
        locals.var_temp1_dn4 = assign15710_e15533_d_n4;
        locals.var_temp1_dn6 = assign15710_e15533_d_n6;
        locals.var_temp1_dn7 = assign15710_e15533_d_n7;
        locals.var_temp1_dn8 = assign15710_e15533_d_n8;
        locals.var_temp1_dn9 = assign15710_e15533_d_n9;

        let assign15720_e15536: f64 = (2.0 * locals.var_temp1);
        locals.var_esurf1s = assign15720_e15536;
        locals.var_esurf1s_dn4 = (2.0 * locals.var_temp1_dn4);
        locals.var_esurf1s_dn6 = (2.0 * locals.var_temp1_dn6);
        locals.var_esurf1s_dn7 = (2.0 * locals.var_temp1_dn7);
        locals.var_esurf1s_dn8 = (2.0 * locals.var_temp1_dn8);
        locals.var_esurf1s_dn9 = (2.0 * locals.var_temp1_dn9);

        let assign15730_e15539: f64 = (locals.var_k2q2s / 2.0);
        let assign15730_e15541: f64 = if assign15730_e15539 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard603 = assign15730_e15541;

    }

    pub(super) fn stamp_transient_block_39(
        locals: &mut StampLocals,
    ) {
        let (assign15740_e15551, assign15740_e15551_d_n4, assign15740_e15551_d_n6, assign15740_e15551_d_n7, assign15740_e15551_d_n8, assign15740_e15551_d_n9,) = {
    if (locals.var_guard603 != 0.0) {
        let assign15740_e15546: f64 = (locals.var_k2q2s / 2.0);
        let assign15740_e15547: f64 = (assign15740_e15546).exp();
        let assign15740_e15548: f64 = (1.0 + assign15740_e15547);
        let assign15740_e15549: f64 = (assign15740_e15548).ln();
        (assign15740_e15549, ((assign15740_e15547 * (locals.var_k2q2s_dn4 / 2.0)) / assign15740_e15548), ((assign15740_e15547 * (locals.var_k2q2s_dn6 / 2.0)) / assign15740_e15548), ((assign15740_e15547 * (locals.var_k2q2s_dn7 / 2.0)) / assign15740_e15548), ((assign15740_e15547 * (locals.var_k2q2s_dn8 / 2.0)) / assign15740_e15548), ((assign15740_e15547 * (locals.var_k2q2s_dn9 / 2.0)) / assign15740_e15548),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign15740_e15551;
        locals.var_temp2_dn4 = assign15740_e15551_d_n4;
        locals.var_temp2_dn6 = assign15740_e15551_d_n6;
        locals.var_temp2_dn7 = assign15740_e15551_d_n7;
        locals.var_temp2_dn8 = assign15740_e15551_d_n8;
        locals.var_temp2_dn9 = assign15740_e15551_d_n9;

        let (assign15750_e15558, assign15750_e15558_d_n4, assign15750_e15558_d_n6, assign15750_e15558_d_n7, assign15750_e15558_d_n8, assign15750_e15558_d_n9,) = {
    if (locals.var_guard603 == 0.0) {
        let assign15750_e15556: f64 = (locals.var_k2q2s / 2.0);
        (assign15750_e15556, (locals.var_k2q2s_dn4 / 2.0), (locals.var_k2q2s_dn6 / 2.0), (locals.var_k2q2s_dn7 / 2.0), (locals.var_k2q2s_dn8 / 2.0), (locals.var_k2q2s_dn9 / 2.0),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign15750_e15558;
        locals.var_temp2_dn4 = assign15750_e15558_d_n4;
        locals.var_temp2_dn6 = assign15750_e15558_d_n6;
        locals.var_temp2_dn7 = assign15750_e15558_d_n7;
        locals.var_temp2_dn8 = assign15750_e15558_d_n8;
        locals.var_temp2_dn9 = assign15750_e15558_d_n9;

        let assign15760_e15561: f64 = (2.0 * locals.var_temp2);
        locals.var_esurf2s = assign15760_e15561;
        locals.var_esurf2s_dn4 = (2.0 * locals.var_temp2_dn4);
        locals.var_esurf2s_dn6 = (2.0 * locals.var_temp2_dn6);
        locals.var_esurf2s_dn7 = (2.0 * locals.var_temp2_dn7);
        locals.var_esurf2s_dn8 = (2.0 * locals.var_temp2_dn8);
        locals.var_esurf2s_dn9 = (2.0 * locals.var_temp2_dn9);

        let assign15770_e15564: f64 = (locals.var_esurf2s - locals.var_k2q2s);
        locals.var_ecpl1s = assign15770_e15564;
        locals.var_ecpl1s_dn4 = (locals.var_esurf2s_dn4 - locals.var_k2q2s_dn4);
        locals.var_ecpl1s_dn6 = (locals.var_esurf2s_dn6 - locals.var_k2q2s_dn6);
        locals.var_ecpl1s_dn7 = (locals.var_esurf2s_dn7 - locals.var_k2q2s_dn7);
        locals.var_ecpl1s_dn8 = (locals.var_esurf2s_dn8 - locals.var_k2q2s_dn8);
        locals.var_ecpl1s_dn9 = (locals.var_esurf2s_dn9 - locals.var_k2q2s_dn9);

        let assign15780_e15567: f64 = (locals.var_esurf1s - locals.var_k1q1s);
        locals.var_ecpl2s = assign15780_e15567;
        locals.var_ecpl2s_dn4 = (locals.var_esurf1s_dn4 - locals.var_k1q1s_dn4);
        locals.var_ecpl2s_dn6 = (locals.var_esurf1s_dn6 - locals.var_k1q1s_dn6);
        locals.var_ecpl2s_dn7 = (locals.var_esurf1s_dn7 - locals.var_k1q1s_dn7);
        locals.var_ecpl2s_dn8 = (locals.var_esurf1s_dn8 - locals.var_k1q1s_dn8);
        locals.var_ecpl2s_dn9 = (locals.var_esurf1s_dn9 - locals.var_k1q1s_dn9);

        let assign15790_e15570: f64 = (locals.var_eta_mu * locals.var_esurf1s);
        let assign15790_e15573: f64 = (locals.var_one_m_eta * locals.var_ecpl1s);
        let assign15790_e15574: f64 = (assign15790_e15570 + assign15790_e15573);
        locals.var_eeff1s = assign15790_e15574;
        locals.var_eeff1s_dn4 = ((locals.var_eta_mu * locals.var_esurf1s_dn4) + (locals.var_one_m_eta * locals.var_ecpl1s_dn4));
        locals.var_eeff1s_dn6 = ((locals.var_eta_mu * locals.var_esurf1s_dn6) + (locals.var_one_m_eta * locals.var_ecpl1s_dn6));
        locals.var_eeff1s_dn7 = ((locals.var_eta_mu * locals.var_esurf1s_dn7) + (locals.var_one_m_eta * locals.var_ecpl1s_dn7));
        locals.var_eeff1s_dn8 = ((locals.var_eta_mu * locals.var_esurf1s_dn8) + (locals.var_one_m_eta * locals.var_ecpl1s_dn8));
        locals.var_eeff1s_dn9 = ((locals.var_eta_mu * locals.var_esurf1s_dn9) + (locals.var_one_m_eta * locals.var_ecpl1s_dn9));

        let assign15800_e15577: f64 = (locals.var_eta_mu * locals.var_esurf2s);
        let assign15800_e15580: f64 = (locals.var_one_m_eta * locals.var_ecpl2s);
        let assign15800_e15581: f64 = (assign15800_e15577 + assign15800_e15580);
        locals.var_eeff2s = assign15800_e15581;
        locals.var_eeff2s_dn4 = ((locals.var_eta_mu * locals.var_esurf2s_dn4) + (locals.var_one_m_eta * locals.var_ecpl2s_dn4));
        locals.var_eeff2s_dn6 = ((locals.var_eta_mu * locals.var_esurf2s_dn6) + (locals.var_one_m_eta * locals.var_ecpl2s_dn6));
        locals.var_eeff2s_dn7 = ((locals.var_eta_mu * locals.var_esurf2s_dn7) + (locals.var_one_m_eta * locals.var_ecpl2s_dn7));
        locals.var_eeff2s_dn8 = ((locals.var_eta_mu * locals.var_esurf2s_dn8) + (locals.var_one_m_eta * locals.var_ecpl2s_dn8));
        locals.var_eeff2s_dn9 = ((locals.var_eta_mu * locals.var_esurf2s_dn9) + (locals.var_one_m_eta * locals.var_ecpl2s_dn9));

        let assign15810_e15585: f64 = (locals.var_esurf1s + locals.var_esurf2s);
        let assign15810_e15586: f64 = (locals.var_qis / assign15810_e15585);
        locals.var_temp = assign15810_e15586;
        locals.var_temp_dn4 = (((locals.var_qis_dn4 * assign15810_e15585) - (locals.var_qis * (locals.var_esurf1s_dn4 + locals.var_esurf2s_dn4))) / (assign15810_e15585 * assign15810_e15585));
        locals.var_temp_dn6 = (((locals.var_qis_dn6 * assign15810_e15585) - (locals.var_qis * (locals.var_esurf1s_dn6 + locals.var_esurf2s_dn6))) / (assign15810_e15585 * assign15810_e15585));
        locals.var_temp_dn7 = (((locals.var_qis_dn7 * assign15810_e15585) - (locals.var_qis * (locals.var_esurf1s_dn7 + locals.var_esurf2s_dn7))) / (assign15810_e15585 * assign15810_e15585));
        locals.var_temp_dn8 = (((locals.var_qis_dn8 * assign15810_e15585) - (locals.var_qis * (locals.var_esurf1s_dn8 + locals.var_esurf2s_dn8))) / (assign15810_e15585 * assign15810_e15585));
        locals.var_temp_dn9 = (((locals.var_qis_dn9 * assign15810_e15585) - (locals.var_qis * (locals.var_esurf1s_dn9 + locals.var_esurf2s_dn9))) / (assign15810_e15585 * assign15810_e15585));

        let assign15820_e15589: f64 = (locals.var_esurf1s * locals.var_temp);
        locals.var_qi1s = assign15820_e15589;
        locals.var_qi1s_dn4 = ((locals.var_esurf1s_dn4 * locals.var_temp) + (locals.var_esurf1s * locals.var_temp_dn4));
        locals.var_qi1s_dn6 = ((locals.var_esurf1s_dn6 * locals.var_temp) + (locals.var_esurf1s * locals.var_temp_dn6));
        locals.var_qi1s_dn7 = ((locals.var_esurf1s_dn7 * locals.var_temp) + (locals.var_esurf1s * locals.var_temp_dn7));
        locals.var_qi1s_dn8 = ((locals.var_esurf1s_dn8 * locals.var_temp) + (locals.var_esurf1s * locals.var_temp_dn8));
        locals.var_qi1s_dn9 = ((locals.var_esurf1s_dn9 * locals.var_temp) + (locals.var_esurf1s * locals.var_temp_dn9));

        let assign15830_e15592: f64 = (locals.var_esurf2s * locals.var_temp);
        locals.var_qi2s = assign15830_e15592;
        locals.var_qi2s_dn4 = ((locals.var_esurf2s_dn4 * locals.var_temp) + (locals.var_esurf2s * locals.var_temp_dn4));
        locals.var_qi2s_dn6 = ((locals.var_esurf2s_dn6 * locals.var_temp) + (locals.var_esurf2s * locals.var_temp_dn6));
        locals.var_qi2s_dn7 = ((locals.var_esurf2s_dn7 * locals.var_temp) + (locals.var_esurf2s * locals.var_temp_dn7));
        locals.var_qi2s_dn8 = ((locals.var_esurf2s_dn8 * locals.var_temp) + (locals.var_esurf2s * locals.var_temp_dn8));
        locals.var_qi2s_dn9 = ((locals.var_esurf2s_dn9 * locals.var_temp) + (locals.var_esurf2s * locals.var_temp_dn9));

        let assign15840_e15595: f64 = (locals.var_esurf1s * locals.var_betn1_t);
        let assign15840_e15598: f64 = (locals.var_stbet_i * locals.var_lnrtn);
        let assign15840_e15599: f64 = (assign15840_e15598).exp();
        let assign15840_e15600: f64 = (assign15840_e15595 * assign15840_e15599);
        locals.var_c1s = assign15840_e15600;
        locals.var_c1s_dn4 = ((((locals.var_esurf1s_dn4 * locals.var_betn1_t) + (locals.var_esurf1s * locals.var_betn1_t_dn4)) * assign15840_e15599) + (assign15840_e15595 * (assign15840_e15599 * (locals.var_stbet_i * locals.var_lnrtn_dn4))));
        locals.var_c1s_dn6 = ((((locals.var_esurf1s_dn6 * locals.var_betn1_t) + (locals.var_esurf1s * locals.var_betn1_t_dn6)) * assign15840_e15599) + (assign15840_e15595 * (assign15840_e15599 * (locals.var_stbet_i * locals.var_lnrtn_dn6))));
        locals.var_c1s_dn7 = ((((locals.var_esurf1s_dn7 * locals.var_betn1_t) + (locals.var_esurf1s * locals.var_betn1_t_dn7)) * assign15840_e15599) + (assign15840_e15595 * (assign15840_e15599 * (locals.var_stbet_i * locals.var_lnrtn_dn7))));
        locals.var_c1s_dn8 = ((((locals.var_esurf1s_dn8 * locals.var_betn1_t) + (locals.var_esurf1s * locals.var_betn1_t_dn8)) * assign15840_e15599) + (assign15840_e15595 * (assign15840_e15599 * (locals.var_stbet_i * locals.var_lnrtn_dn8))));
        locals.var_c1s_dn9 = ((((locals.var_esurf1s_dn9 * locals.var_betn1_t) + (locals.var_esurf1s * locals.var_betn1_t_dn9)) * assign15840_e15599) + (assign15840_e15595 * (assign15840_e15599 * (locals.var_stbet_i * locals.var_lnrtn_dn9))));

        let assign15850_e15603: f64 = (locals.var_esurf2s * locals.var_betn2_t);
        let assign15850_e15606: f64 = (locals.var_stbet_i * locals.var_lnrtn);
        let assign15850_e15607: f64 = (assign15850_e15606).exp();
        let assign15850_e15608: f64 = (assign15850_e15603 * assign15850_e15607);
        locals.var_c2s = assign15850_e15608;
        locals.var_c2s_dn4 = ((((locals.var_esurf2s_dn4 * locals.var_betn2_t) + (locals.var_esurf2s * locals.var_betn2_t_dn4)) * assign15850_e15607) + (assign15850_e15603 * (assign15850_e15607 * (locals.var_stbet_i * locals.var_lnrtn_dn4))));
        locals.var_c2s_dn6 = ((((locals.var_esurf2s_dn6 * locals.var_betn2_t) + (locals.var_esurf2s * locals.var_betn2_t_dn6)) * assign15850_e15607) + (assign15850_e15603 * (assign15850_e15607 * (locals.var_stbet_i * locals.var_lnrtn_dn6))));
        locals.var_c2s_dn7 = ((((locals.var_esurf2s_dn7 * locals.var_betn2_t) + (locals.var_esurf2s * locals.var_betn2_t_dn7)) * assign15850_e15607) + (assign15850_e15603 * (assign15850_e15607 * (locals.var_stbet_i * locals.var_lnrtn_dn7))));
        locals.var_c2s_dn8 = ((((locals.var_esurf2s_dn8 * locals.var_betn2_t) + (locals.var_esurf2s * locals.var_betn2_t_dn8)) * assign15850_e15607) + (assign15850_e15603 * (assign15850_e15607 * (locals.var_stbet_i * locals.var_lnrtn_dn8))));
        locals.var_c2s_dn9 = ((((locals.var_esurf2s_dn9 * locals.var_betn2_t) + (locals.var_esurf2s * locals.var_betn2_t_dn9)) * assign15850_e15607) + (assign15850_e15603 * (assign15850_e15607 * (locals.var_stbet_i * locals.var_lnrtn_dn9))));

        let assign15860_e15613: f64 = (locals.var_xcorb_i * locals.var_ecpl2s);
        let assign15860_e15614: f64 = (locals.var_ecpl1s + assign15860_e15613);
        let assign15860_e15615: f64 = (locals.var_xcor_i * assign15860_e15614);
        locals.var_temp1 = assign15860_e15615;
        locals.var_temp1_dn4 = ((locals.var_xcor_i_dn4 * assign15860_e15614) + (locals.var_xcor_i * (locals.var_ecpl1s_dn4 + (locals.var_xcorb_i * locals.var_ecpl2s_dn4))));
        locals.var_temp1_dn6 = ((locals.var_xcor_i_dn6 * assign15860_e15614) + (locals.var_xcor_i * (locals.var_ecpl1s_dn6 + (locals.var_xcorb_i * locals.var_ecpl2s_dn6))));
        locals.var_temp1_dn7 = ((locals.var_xcor_i_dn7 * assign15860_e15614) + (locals.var_xcor_i * (locals.var_ecpl1s_dn7 + (locals.var_xcorb_i * locals.var_ecpl2s_dn7))));
        locals.var_temp1_dn8 = ((locals.var_xcor_i_dn8 * assign15860_e15614) + (locals.var_xcor_i * (locals.var_ecpl1s_dn8 + (locals.var_xcorb_i * locals.var_ecpl2s_dn8))));
        locals.var_temp1_dn9 = ((locals.var_xcor_i_dn9 * assign15860_e15614) + (locals.var_xcor_i * (locals.var_ecpl1s_dn9 + (locals.var_xcorb_i * locals.var_ecpl2s_dn9))));

        let assign15870_e15619: f64 = (1.0 + locals.var_temp1);
        let assign15870_e15621: f64 = assign15870_e15619;
        let assign15870_e15624: f64 = (1.0 + locals.var_temp1);
        let assign15870_e15626: f64 = assign15870_e15624;
        let assign15870_e15629: f64 = (1.0 + locals.var_temp1);
        let assign15870_e15631: f64 = assign15870_e15629;
        let assign15870_e15632: f64 = (assign15870_e15626 * assign15870_e15631);
        let assign15870_e15634: f64 = (assign15870_e15632 + 0.01);
        let assign15870_e15635: f64 = (assign15870_e15634).sqrt();
        let assign15870_e15636: f64 = (assign15870_e15621 + assign15870_e15635);
        let assign15870_e15637: f64 = (0.5 * assign15870_e15636);
        locals.var_temp2 = assign15870_e15637;
        locals.var_temp2_dn4 = (0.5 * (locals.var_temp1_dn4 + (((locals.var_temp1_dn4 * assign15870_e15631) + (assign15870_e15626 * locals.var_temp1_dn4)) / (2.0 * assign15870_e15635))));
        locals.var_temp2_dn6 = (0.5 * (locals.var_temp1_dn6 + (((locals.var_temp1_dn6 * assign15870_e15631) + (assign15870_e15626 * locals.var_temp1_dn6)) / (2.0 * assign15870_e15635))));
        locals.var_temp2_dn7 = (0.5 * (locals.var_temp1_dn7 + (((locals.var_temp1_dn7 * assign15870_e15631) + (assign15870_e15626 * locals.var_temp1_dn7)) / (2.0 * assign15870_e15635))));
        locals.var_temp2_dn8 = (0.5 * (locals.var_temp1_dn8 + (((locals.var_temp1_dn8 * assign15870_e15631) + (assign15870_e15626 * locals.var_temp1_dn8)) / (2.0 * assign15870_e15635))));
        locals.var_temp2_dn9 = (0.5 * (locals.var_temp1_dn9 + (((locals.var_temp1_dn9 * assign15870_e15631) + (assign15870_e15626 * locals.var_temp1_dn9)) / (2.0 * assign15870_e15635))));

        let assign15880_e15642: f64 = (0.2 * locals.var_temp1);
        let assign15880_e15643: f64 = (1.0 + assign15880_e15642);
        let assign15880_e15645: f64 = assign15880_e15643;
        let assign15880_e15649: f64 = (0.2 * locals.var_temp1);
        let assign15880_e15650: f64 = (1.0 + assign15880_e15649);
        let assign15880_e15652: f64 = assign15880_e15650;
        let assign15880_e15656: f64 = (0.2 * locals.var_temp1);
        let assign15880_e15657: f64 = (1.0 + assign15880_e15656);
        let assign15880_e15659: f64 = assign15880_e15657;
        let assign15880_e15660: f64 = (assign15880_e15652 * assign15880_e15659);
        let assign15880_e15662: f64 = (assign15880_e15660 + 0.01);
        let assign15880_e15663: f64 = (assign15880_e15662).sqrt();
        let assign15880_e15664: f64 = (assign15880_e15645 + assign15880_e15663);
        let assign15880_e15665: f64 = (0.5 * assign15880_e15664);
        locals.var_temp3 = assign15880_e15665;
        locals.var_temp3_dn4 = (0.5 * ((0.2 * locals.var_temp1_dn4) + ((((0.2 * locals.var_temp1_dn4) * assign15880_e15659) + (assign15880_e15652 * (0.2 * locals.var_temp1_dn4))) / (2.0 * assign15880_e15663))));
        locals.var_temp3_dn6 = (0.5 * ((0.2 * locals.var_temp1_dn6) + ((((0.2 * locals.var_temp1_dn6) * assign15880_e15659) + (assign15880_e15652 * (0.2 * locals.var_temp1_dn6))) / (2.0 * assign15880_e15663))));
        locals.var_temp3_dn7 = (0.5 * ((0.2 * locals.var_temp1_dn7) + ((((0.2 * locals.var_temp1_dn7) * assign15880_e15659) + (assign15880_e15652 * (0.2 * locals.var_temp1_dn7))) / (2.0 * assign15880_e15663))));
        locals.var_temp3_dn8 = (0.5 * ((0.2 * locals.var_temp1_dn8) + ((((0.2 * locals.var_temp1_dn8) * assign15880_e15659) + (assign15880_e15652 * (0.2 * locals.var_temp1_dn8))) / (2.0 * assign15880_e15663))));
        locals.var_temp3_dn9 = (0.5 * ((0.2 * locals.var_temp1_dn9) + ((((0.2 * locals.var_temp1_dn9) * assign15880_e15659) + (assign15880_e15652 * (0.2 * locals.var_temp1_dn9))) / (2.0 * assign15880_e15663))));

        let assign15890_e15668: f64 = (locals.var_temp2 / locals.var_temp3);
        locals.var_fcors = assign15890_e15668;
        locals.var_fcors_dn4 = (((locals.var_temp2_dn4 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn4)) / (locals.var_temp3 * locals.var_temp3));
        locals.var_fcors_dn6 = (((locals.var_temp2_dn6 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn6)) / (locals.var_temp3 * locals.var_temp3));
        locals.var_fcors_dn7 = (((locals.var_temp2_dn7 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn7)) / (locals.var_temp3 * locals.var_temp3));
        locals.var_fcors_dn8 = (((locals.var_temp2_dn8 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn8)) / (locals.var_temp3 * locals.var_temp3));
        locals.var_fcors_dn9 = (((locals.var_temp2_dn9 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn9)) / (locals.var_temp3 * locals.var_temp3));

        let assign15900_e15673: f64 = (locals.var_csfi_i * locals.var_ecpl1s);
        let assign15900_e15674: f64 = (1.0 + assign15900_e15673);
        let assign15900_e15677: f64 = (locals.var_csbi_i * locals.var_ecpl2s);
        let assign15900_e15678: f64 = (assign15900_e15674 + assign15900_e15677);
        let assign15900_e15679: f64 = (locals.var_cs_i * assign15900_e15678);
        let assign15900_e15681: f64 = (-locals.var_thecs_i);
        let assign15900_e15685: f64 = (locals.var_qi1s * locals.var_inv_qi1cs);
        let assign15900_e15686: f64 = (1.0 + assign15900_e15685);
        let assign15900_e15689: f64 = (locals.var_qi2s * locals.var_inv_qi2cs);
        let assign15900_e15690: f64 = (assign15900_e15686 + assign15900_e15689);
        let assign15900_e15691: f64 = (assign15900_e15690).ln();
        let assign15900_e15692: f64 = (assign15900_e15681 * assign15900_e15691);
        let assign15900_e15693: f64 = (assign15900_e15692).exp();
        let assign15900_e15694: f64 = (assign15900_e15679 * assign15900_e15693);
        locals.var_gcss = assign15900_e15694;
        locals.var_gcss_dn4 = ((((locals.var_cs_i_dn4 * assign15900_e15678) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1s_dn4) + (locals.var_csbi_i * locals.var_ecpl2s_dn4)))) * assign15900_e15693) + (assign15900_e15679 * (assign15900_e15693 * (((-locals.var_thecs_i_dn4) * assign15900_e15691) + (assign15900_e15681 * (((locals.var_qi1s_dn4 * locals.var_inv_qi1cs) + (locals.var_qi2s_dn4 * locals.var_inv_qi2cs)) / assign15900_e15690))))));
        locals.var_gcss_dn6 = ((((locals.var_cs_i_dn6 * assign15900_e15678) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1s_dn6) + (locals.var_csbi_i * locals.var_ecpl2s_dn6)))) * assign15900_e15693) + (assign15900_e15679 * (assign15900_e15693 * (((-locals.var_thecs_i_dn6) * assign15900_e15691) + (assign15900_e15681 * (((locals.var_qi1s_dn6 * locals.var_inv_qi1cs) + (locals.var_qi2s_dn6 * locals.var_inv_qi2cs)) / assign15900_e15690))))));
        locals.var_gcss_dn7 = ((((locals.var_cs_i_dn7 * assign15900_e15678) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1s_dn7) + (locals.var_csbi_i * locals.var_ecpl2s_dn7)))) * assign15900_e15693) + (assign15900_e15679 * (assign15900_e15693 * (((-locals.var_thecs_i_dn7) * assign15900_e15691) + (assign15900_e15681 * (((locals.var_qi1s_dn7 * locals.var_inv_qi1cs) + (locals.var_qi2s_dn7 * locals.var_inv_qi2cs)) / assign15900_e15690))))));
        locals.var_gcss_dn8 = ((((locals.var_cs_i_dn8 * assign15900_e15678) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1s_dn8) + (locals.var_csbi_i * locals.var_ecpl2s_dn8)))) * assign15900_e15693) + (assign15900_e15679 * (assign15900_e15693 * (((-locals.var_thecs_i_dn8) * assign15900_e15691) + (assign15900_e15681 * (((locals.var_qi1s_dn8 * locals.var_inv_qi1cs) + (locals.var_qi2s_dn8 * locals.var_inv_qi2cs)) / assign15900_e15690))))));
        locals.var_gcss_dn9 = ((((locals.var_cs_i_dn9 * assign15900_e15678) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1s_dn9) + (locals.var_csbi_i * locals.var_ecpl2s_dn9)))) * assign15900_e15693) + (assign15900_e15679 * (assign15900_e15693 * (((-locals.var_thecs_i_dn9) * assign15900_e15691) + (assign15900_e15681 * (((locals.var_qi1s_dn9 * locals.var_inv_qi1cs) + (locals.var_qi2s_dn9 * locals.var_inv_qi2cs)) / assign15900_e15690))))));

        let assign15910_e15697: f64 = if locals.var_rsg_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard604 = assign15910_e15697;

        let (assign15920_e15701, assign15920_e15701_d_n4, assign15920_e15701_d_n6, assign15920_e15701_d_n7, assign15920_e15701_d_n8, assign15920_e15701_d_n9,) = {
    if (locals.var_guard604 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign15920_e15701;
        locals.var_temp3_dn4 = assign15920_e15701_d_n4;
        locals.var_temp3_dn6 = assign15920_e15701_d_n6;
        locals.var_temp3_dn7 = assign15920_e15701_d_n7;
        locals.var_temp3_dn8 = assign15920_e15701_d_n8;
        locals.var_temp3_dn9 = assign15920_e15701_d_n9;

        let assign15930_e15704: f64 = if locals.var_rsg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard605 = assign15930_e15704;

        let (assign15940_e15719, assign15940_e15719_d_n4, assign15940_e15719_d_n6, assign15940_e15719_d_n7, assign15940_e15719_d_n8, assign15940_e15719_d_n9,) = {
    if ((locals.var_guard604 == 0.0) && (locals.var_guard605 != 0.0)) {
        let assign15940_e15713: f64 = (locals.var_qis + 1e-12);
        let assign15940_e15714: f64 = (assign15940_e15713).ln();
        let assign15940_e15715: f64 = (locals.var_thersg_i * assign15940_e15714);
        let assign15940_e15716: f64 = (assign15940_e15715).exp();
        let assign15940_e15717: f64 = (locals.var_rsg_i * assign15940_e15716);
        (assign15940_e15717, (locals.var_rsg_i * (assign15940_e15716 * (locals.var_thersg_i * (locals.var_qis_dn4 / assign15940_e15713)))), (locals.var_rsg_i * (assign15940_e15716 * (locals.var_thersg_i * (locals.var_qis_dn6 / assign15940_e15713)))), (locals.var_rsg_i * (assign15940_e15716 * (locals.var_thersg_i * (locals.var_qis_dn7 / assign15940_e15713)))), (locals.var_rsg_i * (assign15940_e15716 * (locals.var_thersg_i * (locals.var_qis_dn8 / assign15940_e15713)))), (locals.var_rsg_i * (assign15940_e15716 * (locals.var_thersg_i * (locals.var_qis_dn9 / assign15940_e15713)))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign15940_e15719;
        locals.var_temp1_dn4 = assign15940_e15719_d_n4;
        locals.var_temp1_dn6 = assign15940_e15719_d_n6;
        locals.var_temp1_dn7 = assign15940_e15719_d_n7;
        locals.var_temp1_dn8 = assign15940_e15719_d_n8;
        locals.var_temp1_dn9 = assign15940_e15719_d_n9;

        let (assign15950_e15728, assign15950_e15728_d_n4, assign15950_e15728_d_n6, assign15950_e15728_d_n7, assign15950_e15728_d_n8, assign15950_e15728_d_n9,) = {
    if ((locals.var_guard604 == 0.0) && (locals.var_guard605 != 0.0)) {
        let assign15950_e15726: f64 = (1.0 - locals.var_temp1);
        (assign15950_e15726, (-locals.var_temp1_dn4), (-locals.var_temp1_dn6), (-locals.var_temp1_dn7), (-locals.var_temp1_dn8), (-locals.var_temp1_dn9),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign15950_e15728;
        locals.var_temp3_dn4 = assign15950_e15728_d_n4;
        locals.var_temp3_dn6 = assign15950_e15728_d_n6;
        locals.var_temp3_dn7 = assign15950_e15728_d_n7;
        locals.var_temp3_dn8 = assign15950_e15728_d_n8;
        locals.var_temp3_dn9 = assign15950_e15728_d_n9;

        let (assign15960_e15744, assign15960_e15744_d_n4, assign15960_e15744_d_n6, assign15960_e15744_d_n7, assign15960_e15744_d_n8, assign15960_e15744_d_n9,) = {
    if ((locals.var_guard604 == 0.0) && (locals.var_guard605 == 0.0)) {
        let assign15960_e15738: f64 = (locals.var_qis + 1e-12);
        let assign15960_e15739: f64 = (assign15960_e15738).ln();
        let assign15960_e15740: f64 = (locals.var_thersg_i * assign15960_e15739);
        let assign15960_e15741: f64 = (assign15960_e15740).exp();
        let assign15960_e15742: f64 = (locals.var_rsg_i * assign15960_e15741);
        (assign15960_e15742, (locals.var_rsg_i * (assign15960_e15741 * (locals.var_thersg_i * (locals.var_qis_dn4 / assign15960_e15738)))), (locals.var_rsg_i * (assign15960_e15741 * (locals.var_thersg_i * (locals.var_qis_dn6 / assign15960_e15738)))), (locals.var_rsg_i * (assign15960_e15741 * (locals.var_thersg_i * (locals.var_qis_dn7 / assign15960_e15738)))), (locals.var_rsg_i * (assign15960_e15741 * (locals.var_thersg_i * (locals.var_qis_dn8 / assign15960_e15738)))), (locals.var_rsg_i * (assign15960_e15741 * (locals.var_thersg_i * (locals.var_qis_dn9 / assign15960_e15738)))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign15960_e15744;
        locals.var_temp1_dn4 = assign15960_e15744_d_n4;
        locals.var_temp1_dn6 = assign15960_e15744_d_n6;
        locals.var_temp1_dn7 = assign15960_e15744_d_n7;
        locals.var_temp1_dn8 = assign15960_e15744_d_n8;
        locals.var_temp1_dn9 = assign15960_e15744_d_n9;

        let (assign15970_e15756, assign15970_e15756_d_n4, assign15970_e15756_d_n6, assign15970_e15756_d_n7, assign15970_e15756_d_n8, assign15970_e15756_d_n9,) = {
    if ((locals.var_guard604 == 0.0) && (locals.var_guard605 == 0.0)) {
        let assign15970_e15753: f64 = (1.0 + locals.var_temp1);
        let assign15970_e15754: f64 = (1.0 / assign15970_e15753);
        (assign15970_e15754, (-(locals.var_temp1_dn4 / (assign15970_e15753 * assign15970_e15753))), (-(locals.var_temp1_dn6 / (assign15970_e15753 * assign15970_e15753))), (-(locals.var_temp1_dn7 / (assign15970_e15753 * assign15970_e15753))), (-(locals.var_temp1_dn8 / (assign15970_e15753 * assign15970_e15753))), (-(locals.var_temp1_dn9 / (assign15970_e15753 * assign15970_e15753))),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign15970_e15756;
        locals.var_temp3_dn4 = assign15970_e15756_d_n4;
        locals.var_temp3_dn6 = assign15970_e15756_d_n6;
        locals.var_temp3_dn7 = assign15970_e15756_d_n7;
        locals.var_temp3_dn8 = assign15970_e15756_d_n8;
        locals.var_temp3_dn9 = assign15970_e15756_d_n9;

        let assign15980_e15759: f64 = (locals.var_frs * locals.var_csiprime);
        let assign15980_e15761: f64 = (assign15980_e15759 * 0.5);
        let assign15980_e15765: f64 = (locals.var_rsb_i * locals.var_xg20shift);
        let assign15980_e15766: f64 = (1.0 - assign15980_e15765);
        let assign15980_e15768: f64 = assign15980_e15766;
        let assign15980_e15772: f64 = (locals.var_rsb_i * locals.var_xg20shift);
        let assign15980_e15773: f64 = (1.0 - assign15980_e15772);
        let assign15980_e15775: f64 = assign15980_e15773;
        let assign15980_e15779: f64 = (locals.var_rsb_i * locals.var_xg20shift);
        let assign15980_e15780: f64 = (1.0 - assign15980_e15779);
        let assign15980_e15782: f64 = assign15980_e15780;
        let assign15980_e15783: f64 = (assign15980_e15775 * assign15980_e15782);
        let assign15980_e15785: f64 = (assign15980_e15783 + 0.01);
        let assign15980_e15786: f64 = (assign15980_e15785).sqrt();
        let assign15980_e15787: f64 = (assign15980_e15768 + assign15980_e15786);
        let assign15980_e15788: f64 = (assign15980_e15761 * assign15980_e15787);
        locals.var_frscsi = assign15980_e15788;
        locals.var_frscsi_dn4 = (((((locals.var_frs_dn4 * locals.var_csiprime) + (locals.var_frs * locals.var_csiprime_dn4)) * 0.5) * assign15980_e15787) + (assign15980_e15761 * ((-(locals.var_rsb_i * locals.var_xg20shift_dn4)) + ((((-(locals.var_rsb_i * locals.var_xg20shift_dn4)) * assign15980_e15782) + (assign15980_e15775 * (-(locals.var_rsb_i * locals.var_xg20shift_dn4)))) / (2.0 * assign15980_e15786)))));
        locals.var_frscsi_dn6 = (((((locals.var_frs_dn6 * locals.var_csiprime) + (locals.var_frs * locals.var_csiprime_dn6)) * 0.5) * assign15980_e15787) + (assign15980_e15761 * ((-(locals.var_rsb_i * locals.var_xg20shift_dn6)) + ((((-(locals.var_rsb_i * locals.var_xg20shift_dn6)) * assign15980_e15782) + (assign15980_e15775 * (-(locals.var_rsb_i * locals.var_xg20shift_dn6)))) / (2.0 * assign15980_e15786)))));
        locals.var_frscsi_dn7 = (((((locals.var_frs_dn7 * locals.var_csiprime) + (locals.var_frs * locals.var_csiprime_dn7)) * 0.5) * assign15980_e15787) + (assign15980_e15761 * ((-(locals.var_rsb_i * locals.var_xg20shift_dn7)) + ((((-(locals.var_rsb_i * locals.var_xg20shift_dn7)) * assign15980_e15782) + (assign15980_e15775 * (-(locals.var_rsb_i * locals.var_xg20shift_dn7)))) / (2.0 * assign15980_e15786)))));
        locals.var_frscsi_dn8 = (((((locals.var_frs_dn8 * locals.var_csiprime) + (locals.var_frs * locals.var_csiprime_dn8)) * 0.5) * assign15980_e15787) + (assign15980_e15761 * ((-(locals.var_rsb_i * locals.var_xg20shift_dn8)) + ((((-(locals.var_rsb_i * locals.var_xg20shift_dn8)) * assign15980_e15782) + (assign15980_e15775 * (-(locals.var_rsb_i * locals.var_xg20shift_dn8)))) / (2.0 * assign15980_e15786)))));
        locals.var_frscsi_dn9 = (((((locals.var_frs_dn9 * locals.var_csiprime) + (locals.var_frs * locals.var_csiprime_dn9)) * 0.5) * assign15980_e15787) + (assign15980_e15761 * ((-(locals.var_rsb_i * locals.var_xg20shift_dn9)) + ((((-(locals.var_rsb_i * locals.var_xg20shift_dn9)) * assign15980_e15782) + (assign15980_e15775 * (-(locals.var_rsb_i * locals.var_xg20shift_dn9)))) / (2.0 * assign15980_e15786)))));

        let assign15990_e15792: f64 = (locals.var_qis * locals.var_temp3);
        let assign15990_e15794: f64 = (assign15990_e15792 + locals.var_rsig_i);
        let assign15990_e15795: f64 = (locals.var_frscsi * assign15990_e15794);
        locals.var_grss = assign15990_e15795;
        locals.var_grss_dn4 = ((locals.var_frscsi_dn4 * assign15990_e15794) + (locals.var_frscsi * ((locals.var_qis_dn4 * locals.var_temp3) + (locals.var_qis * locals.var_temp3_dn4))));
        locals.var_grss_dn6 = ((locals.var_frscsi_dn6 * assign15990_e15794) + (locals.var_frscsi * ((locals.var_qis_dn6 * locals.var_temp3) + (locals.var_qis * locals.var_temp3_dn6))));
        locals.var_grss_dn7 = ((locals.var_frscsi_dn7 * assign15990_e15794) + (locals.var_frscsi * ((locals.var_qis_dn7 * locals.var_temp3) + (locals.var_qis * locals.var_temp3_dn7))));
        locals.var_grss_dn8 = ((locals.var_frscsi_dn8 * assign15990_e15794) + (locals.var_frscsi * ((locals.var_qis_dn8 * locals.var_temp3) + (locals.var_qis * locals.var_temp3_dn8))));
        locals.var_grss_dn9 = ((locals.var_frscsi_dn9 * assign15990_e15794) + (locals.var_frscsi * ((locals.var_qis_dn9 * locals.var_temp3) + (locals.var_qis * locals.var_temp3_dn9))));

        let assign16000_e15800: f64 = (locals.var_fmue * locals.var_eeff1s);
        let assign16000_e15802: f64 = (assign16000_e15800 + 1e-6);
        let assign16000_e15803: f64 = (assign16000_e15802).ln();
        let assign16000_e15804: f64 = (locals.var_themu_i * assign16000_e15803);
        let assign16000_e15805: f64 = (assign16000_e15804).exp();
        let assign16000_e15806: f64 = (1.0 + assign16000_e15805);
        let assign16000_e15808: f64 = (assign16000_e15806 + locals.var_gcss);
        let assign16000_e15811: f64 = (locals.var_betn1_i * locals.var_grss);
        let assign16000_e15812: f64 = (assign16000_e15808 + assign16000_e15811);
        locals.var_gmob1s = assign16000_e15812;
        locals.var_gmob1s_dn4 = (((assign16000_e15805 * ((locals.var_themu_i_dn4 * assign16000_e15803) + (locals.var_themu_i * (((locals.var_fmue_dn4 * locals.var_eeff1s) + (locals.var_fmue * locals.var_eeff1s_dn4)) / assign16000_e15802)))) + locals.var_gcss_dn4) + ((locals.var_betn1_i_dn4 * locals.var_grss) + (locals.var_betn1_i * locals.var_grss_dn4)));
        locals.var_gmob1s_dn6 = (((assign16000_e15805 * ((locals.var_themu_i_dn6 * assign16000_e15803) + (locals.var_themu_i * (((locals.var_fmue_dn6 * locals.var_eeff1s) + (locals.var_fmue * locals.var_eeff1s_dn6)) / assign16000_e15802)))) + locals.var_gcss_dn6) + ((locals.var_betn1_i_dn6 * locals.var_grss) + (locals.var_betn1_i * locals.var_grss_dn6)));
        locals.var_gmob1s_dn7 = (((assign16000_e15805 * ((locals.var_themu_i_dn7 * assign16000_e15803) + (locals.var_themu_i * (((locals.var_fmue_dn7 * locals.var_eeff1s) + (locals.var_fmue * locals.var_eeff1s_dn7)) / assign16000_e15802)))) + locals.var_gcss_dn7) + ((locals.var_betn1_i_dn7 * locals.var_grss) + (locals.var_betn1_i * locals.var_grss_dn7)));
        locals.var_gmob1s_dn8 = (((assign16000_e15805 * ((locals.var_themu_i_dn8 * assign16000_e15803) + (locals.var_themu_i * (((locals.var_fmue_dn8 * locals.var_eeff1s) + (locals.var_fmue * locals.var_eeff1s_dn8)) / assign16000_e15802)))) + locals.var_gcss_dn8) + ((locals.var_betn1_i_dn8 * locals.var_grss) + (locals.var_betn1_i * locals.var_grss_dn8)));
        locals.var_gmob1s_dn9 = (((assign16000_e15805 * ((locals.var_themu_i_dn9 * assign16000_e15803) + (locals.var_themu_i * (((locals.var_fmue_dn9 * locals.var_eeff1s) + (locals.var_fmue * locals.var_eeff1s_dn9)) / assign16000_e15802)))) + locals.var_gcss_dn9) + ((locals.var_betn1_i_dn9 * locals.var_grss) + (locals.var_betn1_i * locals.var_grss_dn9)));

        let assign16010_e15817: f64 = (locals.var_fmue * locals.var_eeff2s);
        let assign16010_e15819: f64 = (assign16010_e15817 + 1e-6);
        let assign16010_e15820: f64 = (assign16010_e15819).ln();
        let assign16010_e15821: f64 = (locals.var_themu_i * assign16010_e15820);
        let assign16010_e15822: f64 = (assign16010_e15821).exp();
        let assign16010_e15823: f64 = (1.0 + assign16010_e15822);
        let assign16010_e15825: f64 = (assign16010_e15823 + locals.var_gcss);
        let assign16010_e15828: f64 = (locals.var_betn2_i * locals.var_grss);
        let assign16010_e15829: f64 = (assign16010_e15825 + assign16010_e15828);
        locals.var_gmob2s = assign16010_e15829;
        locals.var_gmob2s_dn4 = (((assign16010_e15822 * ((locals.var_themu_i_dn4 * assign16010_e15820) + (locals.var_themu_i * (((locals.var_fmue_dn4 * locals.var_eeff2s) + (locals.var_fmue * locals.var_eeff2s_dn4)) / assign16010_e15819)))) + locals.var_gcss_dn4) + ((locals.var_betn2_i_dn4 * locals.var_grss) + (locals.var_betn2_i * locals.var_grss_dn4)));
        locals.var_gmob2s_dn6 = (((assign16010_e15822 * ((locals.var_themu_i_dn6 * assign16010_e15820) + (locals.var_themu_i * (((locals.var_fmue_dn6 * locals.var_eeff2s) + (locals.var_fmue * locals.var_eeff2s_dn6)) / assign16010_e15819)))) + locals.var_gcss_dn6) + ((locals.var_betn2_i_dn6 * locals.var_grss) + (locals.var_betn2_i * locals.var_grss_dn6)));
        locals.var_gmob2s_dn7 = (((assign16010_e15822 * ((locals.var_themu_i_dn7 * assign16010_e15820) + (locals.var_themu_i * (((locals.var_fmue_dn7 * locals.var_eeff2s) + (locals.var_fmue * locals.var_eeff2s_dn7)) / assign16010_e15819)))) + locals.var_gcss_dn7) + ((locals.var_betn2_i_dn7 * locals.var_grss) + (locals.var_betn2_i * locals.var_grss_dn7)));
        locals.var_gmob2s_dn8 = (((assign16010_e15822 * ((locals.var_themu_i_dn8 * assign16010_e15820) + (locals.var_themu_i * (((locals.var_fmue_dn8 * locals.var_eeff2s) + (locals.var_fmue * locals.var_eeff2s_dn8)) / assign16010_e15819)))) + locals.var_gcss_dn8) + ((locals.var_betn2_i_dn8 * locals.var_grss) + (locals.var_betn2_i * locals.var_grss_dn8)));
        locals.var_gmob2s_dn9 = (((assign16010_e15822 * ((locals.var_themu_i_dn9 * assign16010_e15820) + (locals.var_themu_i * (((locals.var_fmue_dn9 * locals.var_eeff2s) + (locals.var_fmue * locals.var_eeff2s_dn9)) / assign16010_e15819)))) + locals.var_gcss_dn9) + ((locals.var_betn2_i_dn9 * locals.var_grss) + (locals.var_betn2_i * locals.var_grss_dn9)));

        let assign16020_e15833: f64 = (locals.var_c1s + locals.var_c2s);
        let assign16020_e15834: f64 = (locals.var_fcors * assign16020_e15833);
        let assign16020_e15837: f64 = (locals.var_c1s / locals.var_gmob1s);
        let assign16020_e15840: f64 = (locals.var_c2s / locals.var_gmob2s);
        let assign16020_e15841: f64 = (assign16020_e15837 + assign16020_e15840);
        let assign16020_e15842: f64 = (assign16020_e15834 / assign16020_e15841);
        locals.var_gmobs = assign16020_e15842;
        locals.var_gmobs_dn4 = (((((locals.var_fcors_dn4 * assign16020_e15833) + (locals.var_fcors * (locals.var_c1s_dn4 + locals.var_c2s_dn4))) * assign16020_e15841) - (assign16020_e15834 * ((((locals.var_c1s_dn4 * locals.var_gmob1s) - (locals.var_c1s * locals.var_gmob1s_dn4)) / (locals.var_gmob1s * locals.var_gmob1s)) + (((locals.var_c2s_dn4 * locals.var_gmob2s) - (locals.var_c2s * locals.var_gmob2s_dn4)) / (locals.var_gmob2s * locals.var_gmob2s))))) / (assign16020_e15841 * assign16020_e15841));
        locals.var_gmobs_dn6 = (((((locals.var_fcors_dn6 * assign16020_e15833) + (locals.var_fcors * (locals.var_c1s_dn6 + locals.var_c2s_dn6))) * assign16020_e15841) - (assign16020_e15834 * ((((locals.var_c1s_dn6 * locals.var_gmob1s) - (locals.var_c1s * locals.var_gmob1s_dn6)) / (locals.var_gmob1s * locals.var_gmob1s)) + (((locals.var_c2s_dn6 * locals.var_gmob2s) - (locals.var_c2s * locals.var_gmob2s_dn6)) / (locals.var_gmob2s * locals.var_gmob2s))))) / (assign16020_e15841 * assign16020_e15841));
        locals.var_gmobs_dn7 = (((((locals.var_fcors_dn7 * assign16020_e15833) + (locals.var_fcors * (locals.var_c1s_dn7 + locals.var_c2s_dn7))) * assign16020_e15841) - (assign16020_e15834 * ((((locals.var_c1s_dn7 * locals.var_gmob1s) - (locals.var_c1s * locals.var_gmob1s_dn7)) / (locals.var_gmob1s * locals.var_gmob1s)) + (((locals.var_c2s_dn7 * locals.var_gmob2s) - (locals.var_c2s * locals.var_gmob2s_dn7)) / (locals.var_gmob2s * locals.var_gmob2s))))) / (assign16020_e15841 * assign16020_e15841));
        locals.var_gmobs_dn8 = (((((locals.var_fcors_dn8 * assign16020_e15833) + (locals.var_fcors * (locals.var_c1s_dn8 + locals.var_c2s_dn8))) * assign16020_e15841) - (assign16020_e15834 * ((((locals.var_c1s_dn8 * locals.var_gmob1s) - (locals.var_c1s * locals.var_gmob1s_dn8)) / (locals.var_gmob1s * locals.var_gmob1s)) + (((locals.var_c2s_dn8 * locals.var_gmob2s) - (locals.var_c2s * locals.var_gmob2s_dn8)) / (locals.var_gmob2s * locals.var_gmob2s))))) / (assign16020_e15841 * assign16020_e15841));
        locals.var_gmobs_dn9 = (((((locals.var_fcors_dn9 * assign16020_e15833) + (locals.var_fcors * (locals.var_c1s_dn9 + locals.var_c2s_dn9))) * assign16020_e15841) - (assign16020_e15834 * ((((locals.var_c1s_dn9 * locals.var_gmob1s) - (locals.var_c1s * locals.var_gmob1s_dn9)) / (locals.var_gmob1s * locals.var_gmob1s)) + (((locals.var_c2s_dn9 * locals.var_gmob2s) - (locals.var_c2s * locals.var_gmob2s_dn9)) / (locals.var_gmob2s * locals.var_gmob2s))))) / (assign16020_e15841 * assign16020_e15841));

        let assign16030_e15844: f64 = (locals.var_dx_wi).abs();
        let assign16030_e15846: f64 = if assign16030_e15844 > 0.007 { 1.0 } else { 0.0 };
        locals.var_guard606 = assign16030_e15846;

        let assign16040_e15849: f64 = if locals.var_dx_wi > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard607 = assign16040_e15849;

        let (assign16050_e15857, assign16050_e15857_d_n4, assign16050_e15857_d_n6, assign16050_e15857_d_n7, assign16050_e15857_d_n8, assign16050_e15857_d_n9,) = {
    if ((locals.var_guard606 != 0.0) && (locals.var_guard607 != 0.0)) {
        let assign16050_e15854: f64 = (-locals.var_dx_wi);
        let assign16050_e15855: f64 = (assign16050_e15854).exp();
        (assign16050_e15855, (assign16050_e15855 * (-locals.var_dx_wi_dn4)), (assign16050_e15855 * (-locals.var_dx_wi_dn6)), (assign16050_e15855 * (-locals.var_dx_wi_dn7)), (assign16050_e15855 * (-locals.var_dx_wi_dn8)), (assign16050_e15855 * (-locals.var_dx_wi_dn9)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign16050_e15857;
        locals.var_temp_dn4 = assign16050_e15857_d_n4;
        locals.var_temp_dn6 = assign16050_e15857_d_n6;
        locals.var_temp_dn7 = assign16050_e15857_d_n7;
        locals.var_temp_dn8 = assign16050_e15857_d_n8;
        locals.var_temp_dn9 = assign16050_e15857_d_n9;

        let (assign16060_e15867, assign16060_e15867_d_n4, assign16060_e15867_d_n6, assign16060_e15867_d_n7, assign16060_e15867_d_n8, assign16060_e15867_d_n9,) = {
    if ((locals.var_guard606 != 0.0) && (locals.var_guard607 != 0.0)) {
        let assign16060_e15864: f64 = (1.0 - locals.var_temp);
        let assign16060_e15865: f64 = (locals.var_dx_wi / assign16060_e15864);
        (assign16060_e15865, (((locals.var_dx_wi_dn4 * assign16060_e15864) - (locals.var_dx_wi * (-locals.var_temp_dn4))) / (assign16060_e15864 * assign16060_e15864)), (((locals.var_dx_wi_dn6 * assign16060_e15864) - (locals.var_dx_wi * (-locals.var_temp_dn6))) / (assign16060_e15864 * assign16060_e15864)), (((locals.var_dx_wi_dn7 * assign16060_e15864) - (locals.var_dx_wi * (-locals.var_temp_dn7))) / (assign16060_e15864 * assign16060_e15864)), (((locals.var_dx_wi_dn8 * assign16060_e15864) - (locals.var_dx_wi * (-locals.var_temp_dn8))) / (assign16060_e15864 * assign16060_e15864)), (((locals.var_dx_wi_dn9 * assign16060_e15864) - (locals.var_dx_wi * (-locals.var_temp_dn9))) / (assign16060_e15864 * assign16060_e15864)),)
    } else {
        (locals.var_s1, locals.var_s1_dn4, locals.var_s1_dn6, locals.var_s1_dn7, locals.var_s1_dn8, locals.var_s1_dn9,)
    }
};
        locals.var_s1 = assign16060_e15867;
        locals.var_s1_dn4 = assign16060_e15867_d_n4;
        locals.var_s1_dn6 = assign16060_e15867_d_n6;
        locals.var_s1_dn7 = assign16060_e15867_d_n7;
        locals.var_s1_dn8 = assign16060_e15867_d_n8;
        locals.var_s1_dn9 = assign16060_e15867_d_n9;

        let (assign16070_e15875, assign16070_e15875_d_n4, assign16070_e15875_d_n6, assign16070_e15875_d_n7, assign16070_e15875_d_n8, assign16070_e15875_d_n9,) = {
    if ((locals.var_guard606 != 0.0) && (locals.var_guard607 != 0.0)) {
        let assign16070_e15873: f64 = (locals.var_temp * locals.var_s1);
        (assign16070_e15873, ((locals.var_temp_dn4 * locals.var_s1) + (locals.var_temp * locals.var_s1_dn4)), ((locals.var_temp_dn6 * locals.var_s1) + (locals.var_temp * locals.var_s1_dn6)), ((locals.var_temp_dn7 * locals.var_s1) + (locals.var_temp * locals.var_s1_dn7)), ((locals.var_temp_dn8 * locals.var_s1) + (locals.var_temp * locals.var_s1_dn8)), ((locals.var_temp_dn9 * locals.var_s1) + (locals.var_temp * locals.var_s1_dn9)),)
    } else {
        (locals.var_s2, locals.var_s2_dn4, locals.var_s2_dn6, locals.var_s2_dn7, locals.var_s2_dn8, locals.var_s2_dn9,)
    }
};
        locals.var_s2 = assign16070_e15875;
        locals.var_s2_dn4 = assign16070_e15875_d_n4;
        locals.var_s2_dn6 = assign16070_e15875_d_n6;
        locals.var_s2_dn7 = assign16070_e15875_d_n7;
        locals.var_s2_dn8 = assign16070_e15875_d_n8;
        locals.var_s2_dn9 = assign16070_e15875_d_n9;

        let (assign16080_e15890, assign16080_e15890_d_n4, assign16080_e15890_d_n6, assign16080_e15890_d_n7, assign16080_e15890_d_n8, assign16080_e15890_d_n9,) = {
    if ((locals.var_guard606 != 0.0) && (locals.var_guard607 != 0.0)) {
        let assign16080_e15882: f64 = (locals.var_qis * locals.var_s1);
        let assign16080_e15883: f64 = (locals.var_a0 / assign16080_e15882);
        let assign16080_e15884: f64 = (assign16080_e15883).ln();
        let assign16080_e15886: f64 = (assign16080_e15884 - 0.6931471805599);
        let assign16080_e15888: f64 = (assign16080_e15886 + locals.var_x1_wi0);
        (assign16080_e15888, (((((locals.var_a0_dn4 * assign16080_e15882) - (locals.var_a0 * ((locals.var_qis_dn4 * locals.var_s1) + (locals.var_qis * locals.var_s1_dn4)))) / (assign16080_e15882 * assign16080_e15882)) / assign16080_e15883) + locals.var_x1_wi0_dn4), (((((locals.var_a0_dn6 * assign16080_e15882) - (locals.var_a0 * ((locals.var_qis_dn6 * locals.var_s1) + (locals.var_qis * locals.var_s1_dn6)))) / (assign16080_e15882 * assign16080_e15882)) / assign16080_e15883) + locals.var_x1_wi0_dn6), (((((locals.var_a0_dn7 * assign16080_e15882) - (locals.var_a0 * ((locals.var_qis_dn7 * locals.var_s1) + (locals.var_qis * locals.var_s1_dn7)))) / (assign16080_e15882 * assign16080_e15882)) / assign16080_e15883) + locals.var_x1_wi0_dn7), (((((locals.var_a0_dn8 * assign16080_e15882) - (locals.var_a0 * ((locals.var_qis_dn8 * locals.var_s1) + (locals.var_qis * locals.var_s1_dn8)))) / (assign16080_e15882 * assign16080_e15882)) / assign16080_e15883) + locals.var_x1_wi0_dn8), (((((locals.var_a0_dn9 * assign16080_e15882) - (locals.var_a0 * ((locals.var_qis_dn9 * locals.var_s1) + (locals.var_qis * locals.var_s1_dn9)))) / (assign16080_e15882 * assign16080_e15882)) / assign16080_e15883) + locals.var_x1_wi0_dn9),)
    } else {
        (locals.var_deltaxinf, locals.var_deltaxinf_dn4, locals.var_deltaxinf_dn6, locals.var_deltaxinf_dn7, locals.var_deltaxinf_dn8, locals.var_deltaxinf_dn9,)
    }
};
        locals.var_deltaxinf = assign16080_e15890;
        locals.var_deltaxinf_dn4 = assign16080_e15890_d_n4;
        locals.var_deltaxinf_dn6 = assign16080_e15890_d_n6;
        locals.var_deltaxinf_dn7 = assign16080_e15890_d_n7;
        locals.var_deltaxinf_dn8 = assign16080_e15890_d_n8;
        locals.var_deltaxinf_dn9 = assign16080_e15890_d_n9;

        let (assign16090_e15898, assign16090_e15898_d_n4, assign16090_e15898_d_n6, assign16090_e15898_d_n7, assign16090_e15898_d_n8, assign16090_e15898_d_n9,) = {
    if ((locals.var_guard606 != 0.0) && (locals.var_guard607 == 0.0)) {
        let assign16090_e15896: f64 = (locals.var_dx_wi).exp();
        (assign16090_e15896, (assign16090_e15896 * locals.var_dx_wi_dn4), (assign16090_e15896 * locals.var_dx_wi_dn6), (assign16090_e15896 * locals.var_dx_wi_dn7), (assign16090_e15896 * locals.var_dx_wi_dn8), (assign16090_e15896 * locals.var_dx_wi_dn9),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign16090_e15898;
        locals.var_temp_dn4 = assign16090_e15898_d_n4;
        locals.var_temp_dn6 = assign16090_e15898_d_n6;
        locals.var_temp_dn7 = assign16090_e15898_d_n7;
        locals.var_temp_dn8 = assign16090_e15898_d_n8;
        locals.var_temp_dn9 = assign16090_e15898_d_n9;

        let (assign16100_e15909, assign16100_e15909_d_n4, assign16100_e15909_d_n6, assign16100_e15909_d_n7, assign16100_e15909_d_n8, assign16100_e15909_d_n9,) = {
    if ((locals.var_guard606 != 0.0) && (locals.var_guard607 == 0.0)) {
        let assign16100_e15906: f64 = (locals.var_temp - 1.0);
        let assign16100_e15907: f64 = (locals.var_dx_wi / assign16100_e15906);
        (assign16100_e15907, (((locals.var_dx_wi_dn4 * assign16100_e15906) - (locals.var_dx_wi * locals.var_temp_dn4)) / (assign16100_e15906 * assign16100_e15906)), (((locals.var_dx_wi_dn6 * assign16100_e15906) - (locals.var_dx_wi * locals.var_temp_dn6)) / (assign16100_e15906 * assign16100_e15906)), (((locals.var_dx_wi_dn7 * assign16100_e15906) - (locals.var_dx_wi * locals.var_temp_dn7)) / (assign16100_e15906 * assign16100_e15906)), (((locals.var_dx_wi_dn8 * assign16100_e15906) - (locals.var_dx_wi * locals.var_temp_dn8)) / (assign16100_e15906 * assign16100_e15906)), (((locals.var_dx_wi_dn9 * assign16100_e15906) - (locals.var_dx_wi * locals.var_temp_dn9)) / (assign16100_e15906 * assign16100_e15906)),)
    } else {
        (locals.var_s2, locals.var_s2_dn4, locals.var_s2_dn6, locals.var_s2_dn7, locals.var_s2_dn8, locals.var_s2_dn9,)
    }
};
        locals.var_s2 = assign16100_e15909;
        locals.var_s2_dn4 = assign16100_e15909_d_n4;
        locals.var_s2_dn6 = assign16100_e15909_d_n6;
        locals.var_s2_dn7 = assign16100_e15909_d_n7;
        locals.var_s2_dn8 = assign16100_e15909_d_n8;
        locals.var_s2_dn9 = assign16100_e15909_d_n9;

        let (assign16110_e15918, assign16110_e15918_d_n4, assign16110_e15918_d_n6, assign16110_e15918_d_n7, assign16110_e15918_d_n8, assign16110_e15918_d_n9,) = {
    if ((locals.var_guard606 != 0.0) && (locals.var_guard607 == 0.0)) {
        let assign16110_e15916: f64 = (locals.var_temp * locals.var_s2);
        (assign16110_e15916, ((locals.var_temp_dn4 * locals.var_s2) + (locals.var_temp * locals.var_s2_dn4)), ((locals.var_temp_dn6 * locals.var_s2) + (locals.var_temp * locals.var_s2_dn6)), ((locals.var_temp_dn7 * locals.var_s2) + (locals.var_temp * locals.var_s2_dn7)), ((locals.var_temp_dn8 * locals.var_s2) + (locals.var_temp * locals.var_s2_dn8)), ((locals.var_temp_dn9 * locals.var_s2) + (locals.var_temp * locals.var_s2_dn9)),)
    } else {
        (locals.var_s1, locals.var_s1_dn4, locals.var_s1_dn6, locals.var_s1_dn7, locals.var_s1_dn8, locals.var_s1_dn9,)
    }
};
        locals.var_s1 = assign16110_e15918;
        locals.var_s1_dn4 = assign16110_e15918_d_n4;
        locals.var_s1_dn6 = assign16110_e15918_d_n6;
        locals.var_s1_dn7 = assign16110_e15918_d_n7;
        locals.var_s1_dn8 = assign16110_e15918_d_n8;
        locals.var_s1_dn9 = assign16110_e15918_d_n9;

        let (assign16120_e15934, assign16120_e15934_d_n4, assign16120_e15934_d_n6, assign16120_e15934_d_n7, assign16120_e15934_d_n8, assign16120_e15934_d_n9,) = {
    if ((locals.var_guard606 != 0.0) && (locals.var_guard607 == 0.0)) {
        let assign16120_e15926: f64 = (locals.var_qis * locals.var_s2);
        let assign16120_e15927: f64 = (locals.var_a0 / assign16120_e15926);
        let assign16120_e15928: f64 = (assign16120_e15927).ln();
        let assign16120_e15930: f64 = (assign16120_e15928 - 0.6931471805599);
        let assign16120_e15932: f64 = (assign16120_e15930 + locals.var_x2_wi0);
        (assign16120_e15932, (((((locals.var_a0_dn4 * assign16120_e15926) - (locals.var_a0 * ((locals.var_qis_dn4 * locals.var_s2) + (locals.var_qis * locals.var_s2_dn4)))) / (assign16120_e15926 * assign16120_e15926)) / assign16120_e15927) + locals.var_x2_wi0_dn4), (((((locals.var_a0_dn6 * assign16120_e15926) - (locals.var_a0 * ((locals.var_qis_dn6 * locals.var_s2) + (locals.var_qis * locals.var_s2_dn6)))) / (assign16120_e15926 * assign16120_e15926)) / assign16120_e15927) + locals.var_x2_wi0_dn6), (((((locals.var_a0_dn7 * assign16120_e15926) - (locals.var_a0 * ((locals.var_qis_dn7 * locals.var_s2) + (locals.var_qis * locals.var_s2_dn7)))) / (assign16120_e15926 * assign16120_e15926)) / assign16120_e15927) + locals.var_x2_wi0_dn7), (((((locals.var_a0_dn8 * assign16120_e15926) - (locals.var_a0 * ((locals.var_qis_dn8 * locals.var_s2) + (locals.var_qis * locals.var_s2_dn8)))) / (assign16120_e15926 * assign16120_e15926)) / assign16120_e15927) + locals.var_x2_wi0_dn8), (((((locals.var_a0_dn9 * assign16120_e15926) - (locals.var_a0 * ((locals.var_qis_dn9 * locals.var_s2) + (locals.var_qis * locals.var_s2_dn9)))) / (assign16120_e15926 * assign16120_e15926)) / assign16120_e15927) + locals.var_x2_wi0_dn9),)
    } else {
        (locals.var_deltaxinf, locals.var_deltaxinf_dn4, locals.var_deltaxinf_dn6, locals.var_deltaxinf_dn7, locals.var_deltaxinf_dn8, locals.var_deltaxinf_dn9,)
    }
};
        locals.var_deltaxinf = assign16120_e15934;
        locals.var_deltaxinf_dn4 = assign16120_e15934_d_n4;
        locals.var_deltaxinf_dn6 = assign16120_e15934_d_n6;
        locals.var_deltaxinf_dn7 = assign16120_e15934_d_n7;
        locals.var_deltaxinf_dn8 = assign16120_e15934_d_n8;
        locals.var_deltaxinf_dn9 = assign16120_e15934_d_n9;

    }

    pub(super) fn stamp_transient_block_40(
        locals: &mut StampLocals,
    ) {
        let (assign16130_e15949, assign16130_e15949_d_n4, assign16130_e15949_d_n6, assign16130_e15949_d_n7, assign16130_e15949_d_n8, assign16130_e15949_d_n9,) = {
    if (locals.var_guard606 != 0.0) {
        let assign16130_e15937: f64 = (-locals.var_dx_wi);
        let assign16130_e15941: f64 = (1.0 - locals.var_s1);
        let assign16130_e15944: f64 = (locals.var_dx_wi * locals.var_inv_k2);
        let assign16130_e15945: f64 = (assign16130_e15941 - assign16130_e15944);
        let assign16130_e15946: f64 = (locals.var_keq * assign16130_e15945);
        let assign16130_e15947: f64 = (assign16130_e15937 / assign16130_e15946);
        (assign16130_e15947, ((((-locals.var_dx_wi_dn4) * assign16130_e15946) - (assign16130_e15937 * ((locals.var_keq_dn4 * assign16130_e15945) + (locals.var_keq * ((-locals.var_s1_dn4) - ((locals.var_dx_wi_dn4 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn4))))))) / (assign16130_e15946 * assign16130_e15946)), ((((-locals.var_dx_wi_dn6) * assign16130_e15946) - (assign16130_e15937 * ((locals.var_keq_dn6 * assign16130_e15945) + (locals.var_keq * ((-locals.var_s1_dn6) - ((locals.var_dx_wi_dn6 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn6))))))) / (assign16130_e15946 * assign16130_e15946)), ((((-locals.var_dx_wi_dn7) * assign16130_e15946) - (assign16130_e15937 * ((locals.var_keq_dn7 * assign16130_e15945) + (locals.var_keq * ((-locals.var_s1_dn7) - ((locals.var_dx_wi_dn7 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn7))))))) / (assign16130_e15946 * assign16130_e15946)), ((((-locals.var_dx_wi_dn8) * assign16130_e15946) - (assign16130_e15937 * ((locals.var_keq_dn8 * assign16130_e15945) + (locals.var_keq * ((-locals.var_s1_dn8) - ((locals.var_dx_wi_dn8 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn8))))))) / (assign16130_e15946 * assign16130_e15946)), ((((-locals.var_dx_wi_dn9) * assign16130_e15946) - (assign16130_e15937 * ((locals.var_keq_dn9 * assign16130_e15945) + (locals.var_keq * ((-locals.var_s1_dn9) - ((locals.var_dx_wi_dn9 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn9))))))) / (assign16130_e15946 * assign16130_e15946)),)
    } else {
        (locals.var_q1chapinf, locals.var_q1chapinf_dn4, locals.var_q1chapinf_dn6, locals.var_q1chapinf_dn7, locals.var_q1chapinf_dn8, locals.var_q1chapinf_dn9,)
    }
};
        locals.var_q1chapinf = assign16130_e15949;
        locals.var_q1chapinf_dn4 = assign16130_e15949_d_n4;
        locals.var_q1chapinf_dn6 = assign16130_e15949_d_n6;
        locals.var_q1chapinf_dn7 = assign16130_e15949_d_n7;
        locals.var_q1chapinf_dn8 = assign16130_e15949_d_n8;
        locals.var_q1chapinf_dn9 = assign16130_e15949_d_n9;

        let (assign16140_e15963, assign16140_e15963_d_n4, assign16140_e15963_d_n6, assign16140_e15963_d_n7, assign16140_e15963_d_n8, assign16140_e15963_d_n9,) = {
    if (locals.var_guard606 != 0.0) {
        let assign16140_e15955: f64 = (1.0 - locals.var_s2);
        let assign16140_e15958: f64 = (locals.var_dx_wi * locals.var_inv_k1);
        let assign16140_e15959: f64 = (assign16140_e15955 + assign16140_e15958);
        let assign16140_e15960: f64 = (locals.var_keq * assign16140_e15959);
        let assign16140_e15961: f64 = (locals.var_dx_wi / assign16140_e15960);
        (assign16140_e15961, (((locals.var_dx_wi_dn4 * assign16140_e15960) - (locals.var_dx_wi * ((locals.var_keq_dn4 * assign16140_e15959) + (locals.var_keq * ((-locals.var_s2_dn4) + ((locals.var_dx_wi_dn4 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn4))))))) / (assign16140_e15960 * assign16140_e15960)), (((locals.var_dx_wi_dn6 * assign16140_e15960) - (locals.var_dx_wi * ((locals.var_keq_dn6 * assign16140_e15959) + (locals.var_keq * ((-locals.var_s2_dn6) + ((locals.var_dx_wi_dn6 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn6))))))) / (assign16140_e15960 * assign16140_e15960)), (((locals.var_dx_wi_dn7 * assign16140_e15960) - (locals.var_dx_wi * ((locals.var_keq_dn7 * assign16140_e15959) + (locals.var_keq * ((-locals.var_s2_dn7) + ((locals.var_dx_wi_dn7 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn7))))))) / (assign16140_e15960 * assign16140_e15960)), (((locals.var_dx_wi_dn8 * assign16140_e15960) - (locals.var_dx_wi * ((locals.var_keq_dn8 * assign16140_e15959) + (locals.var_keq * ((-locals.var_s2_dn8) + ((locals.var_dx_wi_dn8 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn8))))))) / (assign16140_e15960 * assign16140_e15960)), (((locals.var_dx_wi_dn9 * assign16140_e15960) - (locals.var_dx_wi * ((locals.var_keq_dn9 * assign16140_e15959) + (locals.var_keq * ((-locals.var_s2_dn9) + ((locals.var_dx_wi_dn9 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn9))))))) / (assign16140_e15960 * assign16140_e15960)),)
    } else {
        (locals.var_q2chapinf, locals.var_q2chapinf_dn4, locals.var_q2chapinf_dn6, locals.var_q2chapinf_dn7, locals.var_q2chapinf_dn8, locals.var_q2chapinf_dn9,)
    }
};
        locals.var_q2chapinf = assign16140_e15963;
        locals.var_q2chapinf_dn4 = assign16140_e15963_d_n4;
        locals.var_q2chapinf_dn6 = assign16140_e15963_d_n6;
        locals.var_q2chapinf_dn7 = assign16140_e15963_d_n7;
        locals.var_q2chapinf_dn8 = assign16140_e15963_d_n8;
        locals.var_q2chapinf_dn9 = assign16140_e15963_d_n9;

        let (assign16150_e15983, assign16150_e15983_d_n4, assign16150_e15983_d_n6, assign16150_e15983_d_n7, assign16150_e15983_d_n8, assign16150_e15983_d_n9,) = {
    if (locals.var_guard606 != 0.0) {
        let assign16150_e15968: f64 = (locals.var_s2 * locals.var_inv_k2);
        let assign16150_e15970: f64 = (assign16150_e15968 + 0.5);
        let assign16150_e15972: f64 = (assign16150_e15970 / locals.var_q2chapinf);
        let assign16150_e15975: f64 = (locals.var_s1 * locals.var_inv_k1);
        let assign16150_e15977: f64 = (assign16150_e15975 + 0.5);
        let assign16150_e15979: f64 = (assign16150_e15977 / locals.var_q1chapinf);
        let assign16150_e15980: f64 = (assign16150_e15972 - assign16150_e15979);
        let assign16150_e15981: f64 = (locals.var_dx_wi / assign16150_e15980);
        (assign16150_e15981, (((locals.var_dx_wi_dn4 * assign16150_e15980) - (locals.var_dx_wi * ((((((locals.var_s2_dn4 * locals.var_inv_k2) + (locals.var_s2 * locals.var_inv_k2_dn4)) * locals.var_q2chapinf) - (assign16150_e15970 * locals.var_q2chapinf_dn4)) / (locals.var_q2chapinf * locals.var_q2chapinf)) - (((((locals.var_s1_dn4 * locals.var_inv_k1) + (locals.var_s1 * locals.var_inv_k1_dn4)) * locals.var_q1chapinf) - (assign16150_e15977 * locals.var_q1chapinf_dn4)) / (locals.var_q1chapinf * locals.var_q1chapinf))))) / (assign16150_e15980 * assign16150_e15980)), (((locals.var_dx_wi_dn6 * assign16150_e15980) - (locals.var_dx_wi * ((((((locals.var_s2_dn6 * locals.var_inv_k2) + (locals.var_s2 * locals.var_inv_k2_dn6)) * locals.var_q2chapinf) - (assign16150_e15970 * locals.var_q2chapinf_dn6)) / (locals.var_q2chapinf * locals.var_q2chapinf)) - (((((locals.var_s1_dn6 * locals.var_inv_k1) + (locals.var_s1 * locals.var_inv_k1_dn6)) * locals.var_q1chapinf) - (assign16150_e15977 * locals.var_q1chapinf_dn6)) / (locals.var_q1chapinf * locals.var_q1chapinf))))) / (assign16150_e15980 * assign16150_e15980)), (((locals.var_dx_wi_dn7 * assign16150_e15980) - (locals.var_dx_wi * ((((((locals.var_s2_dn7 * locals.var_inv_k2) + (locals.var_s2 * locals.var_inv_k2_dn7)) * locals.var_q2chapinf) - (assign16150_e15970 * locals.var_q2chapinf_dn7)) / (locals.var_q2chapinf * locals.var_q2chapinf)) - (((((locals.var_s1_dn7 * locals.var_inv_k1) + (locals.var_s1 * locals.var_inv_k1_dn7)) * locals.var_q1chapinf) - (assign16150_e15977 * locals.var_q1chapinf_dn7)) / (locals.var_q1chapinf * locals.var_q1chapinf))))) / (assign16150_e15980 * assign16150_e15980)), (((locals.var_dx_wi_dn8 * assign16150_e15980) - (locals.var_dx_wi * ((((((locals.var_s2_dn8 * locals.var_inv_k2) + (locals.var_s2 * locals.var_inv_k2_dn8)) * locals.var_q2chapinf) - (assign16150_e15970 * locals.var_q2chapinf_dn8)) / (locals.var_q2chapinf * locals.var_q2chapinf)) - (((((locals.var_s1_dn8 * locals.var_inv_k1) + (locals.var_s1 * locals.var_inv_k1_dn8)) * locals.var_q1chapinf) - (assign16150_e15977 * locals.var_q1chapinf_dn8)) / (locals.var_q1chapinf * locals.var_q1chapinf))))) / (assign16150_e15980 * assign16150_e15980)), (((locals.var_dx_wi_dn9 * assign16150_e15980) - (locals.var_dx_wi * ((((((locals.var_s2_dn9 * locals.var_inv_k2) + (locals.var_s2 * locals.var_inv_k2_dn9)) * locals.var_q2chapinf) - (assign16150_e15970 * locals.var_q2chapinf_dn9)) / (locals.var_q2chapinf * locals.var_q2chapinf)) - (((((locals.var_s1_dn9 * locals.var_inv_k1) + (locals.var_s1 * locals.var_inv_k1_dn9)) * locals.var_q1chapinf) - (assign16150_e15977 * locals.var_q1chapinf_dn9)) / (locals.var_q1chapinf * locals.var_q1chapinf))))) / (assign16150_e15980 * assign16150_e15980)),)
    } else {
        (locals.var_dinf, locals.var_dinf_dn4, locals.var_dinf_dn6, locals.var_dinf_dn7, locals.var_dinf_dn8, locals.var_dinf_dn9,)
    }
};
        locals.var_dinf = assign16150_e15983;
        locals.var_dinf_dn4 = assign16150_e15983_d_n4;
        locals.var_dinf_dn6 = assign16150_e15983_d_n6;
        locals.var_dinf_dn7 = assign16150_e15983_d_n7;
        locals.var_dinf_dn8 = assign16150_e15983_d_n8;
        locals.var_dinf_dn9 = assign16150_e15983_d_n9;

        let (assign16160_e15992, assign16160_e15992_d_n4, assign16160_e15992_d_n6, assign16160_e15992_d_n7, assign16160_e15992_d_n8, assign16160_e15992_d_n9,) = {
    if (locals.var_guard606 == 0.0) {
        let assign16160_e15988: f64 = (0.5 * 0.1666666666667);
        let assign16160_e15990: f64 = (assign16160_e15988 * locals.var_dx_wisq);
        (assign16160_e15990, (assign16160_e15988 * locals.var_dx_wisq_dn4), (assign16160_e15988 * locals.var_dx_wisq_dn6), (assign16160_e15988 * locals.var_dx_wisq_dn7), (assign16160_e15988 * locals.var_dx_wisq_dn8), (assign16160_e15988 * locals.var_dx_wisq_dn9),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign16160_e15992;
        locals.var_temp_dn4 = assign16160_e15992_d_n4;
        locals.var_temp_dn6 = assign16160_e15992_d_n6;
        locals.var_temp_dn7 = assign16160_e15992_d_n7;
        locals.var_temp_dn8 = assign16160_e15992_d_n8;
        locals.var_temp_dn9 = assign16160_e15992_d_n9;

        let (assign16170_e15999, assign16170_e15999_d_n4, assign16170_e15999_d_n6, assign16170_e15999_d_n7, assign16170_e15999_d_n8, assign16170_e15999_d_n9,) = {
    if (locals.var_guard606 == 0.0) {
        let assign16170_e15997: f64 = (0.5 * locals.var_dx_wi);
        (assign16170_e15997, (0.5 * locals.var_dx_wi_dn4), (0.5 * locals.var_dx_wi_dn6), (0.5 * locals.var_dx_wi_dn7), (0.5 * locals.var_dx_wi_dn8), (0.5 * locals.var_dx_wi_dn9),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign16170_e15999;
        locals.var_temp1_dn4 = assign16170_e15999_d_n4;
        locals.var_temp1_dn6 = assign16170_e15999_d_n6;
        locals.var_temp1_dn7 = assign16170_e15999_d_n7;
        locals.var_temp1_dn8 = assign16170_e15999_d_n8;
        locals.var_temp1_dn9 = assign16170_e15999_d_n9;

        let (assign16180_e16008, assign16180_e16008_d_n4, assign16180_e16008_d_n6, assign16180_e16008_d_n7, assign16180_e16008_d_n8, assign16180_e16008_d_n9,) = {
    if (locals.var_guard606 == 0.0) {
        let assign16180_e16004: f64 = (1.0 + locals.var_temp1);
        let assign16180_e16006: f64 = (assign16180_e16004 + locals.var_temp);
        (assign16180_e16006, (locals.var_temp1_dn4 + locals.var_temp_dn4), (locals.var_temp1_dn6 + locals.var_temp_dn6), (locals.var_temp1_dn7 + locals.var_temp_dn7), (locals.var_temp1_dn8 + locals.var_temp_dn8), (locals.var_temp1_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_s1, locals.var_s1_dn4, locals.var_s1_dn6, locals.var_s1_dn7, locals.var_s1_dn8, locals.var_s1_dn9,)
    }
};
        locals.var_s1 = assign16180_e16008;
        locals.var_s1_dn4 = assign16180_e16008_d_n4;
        locals.var_s1_dn6 = assign16180_e16008_d_n6;
        locals.var_s1_dn7 = assign16180_e16008_d_n7;
        locals.var_s1_dn8 = assign16180_e16008_d_n8;
        locals.var_s1_dn9 = assign16180_e16008_d_n9;

        let (assign16190_e16017, assign16190_e16017_d_n4, assign16190_e16017_d_n6, assign16190_e16017_d_n7, assign16190_e16017_d_n8, assign16190_e16017_d_n9,) = {
    if (locals.var_guard606 == 0.0) {
        let assign16190_e16013: f64 = (1.0 - locals.var_temp1);
        let assign16190_e16015: f64 = (assign16190_e16013 + locals.var_temp);
        (assign16190_e16015, ((-locals.var_temp1_dn4) + locals.var_temp_dn4), ((-locals.var_temp1_dn6) + locals.var_temp_dn6), ((-locals.var_temp1_dn7) + locals.var_temp_dn7), ((-locals.var_temp1_dn8) + locals.var_temp_dn8), ((-locals.var_temp1_dn9) + locals.var_temp_dn9),)
    } else {
        (locals.var_s2, locals.var_s2_dn4, locals.var_s2_dn6, locals.var_s2_dn7, locals.var_s2_dn8, locals.var_s2_dn9,)
    }
};
        locals.var_s2 = assign16190_e16017;
        locals.var_s2_dn4 = assign16190_e16017_d_n4;
        locals.var_s2_dn6 = assign16190_e16017_d_n6;
        locals.var_s2_dn7 = assign16190_e16017_d_n7;
        locals.var_s2_dn8 = assign16190_e16017_d_n8;
        locals.var_s2_dn9 = assign16190_e16017_d_n9;

        let (assign16200_e16024, assign16200_e16024_d_n4, assign16200_e16024_d_n6, assign16200_e16024_d_n7, assign16200_e16024_d_n8, assign16200_e16024_d_n9,) = {
    if (locals.var_guard606 == 0.0) {
        let assign16200_e16022: f64 = (0.1666666666667 * locals.var_temp1);
        (assign16200_e16022, (0.1666666666667 * locals.var_temp1_dn4), (0.1666666666667 * locals.var_temp1_dn6), (0.1666666666667 * locals.var_temp1_dn7), (0.1666666666667 * locals.var_temp1_dn8), (0.1666666666667 * locals.var_temp1_dn9),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign16200_e16024;
        locals.var_temp2_dn4 = assign16200_e16024_d_n4;
        locals.var_temp2_dn6 = assign16200_e16024_d_n6;
        locals.var_temp2_dn7 = assign16200_e16024_d_n7;
        locals.var_temp2_dn8 = assign16200_e16024_d_n8;
        locals.var_temp2_dn9 = assign16200_e16024_d_n9;

        let (assign16210_e16037, assign16210_e16037_d_n4, assign16210_e16037_d_n6, assign16210_e16037_d_n7, assign16210_e16037_d_n8, assign16210_e16037_d_n9,) = {
    if (locals.var_guard606 == 0.0) {
        let assign16210_e16031: f64 = (0.5 + locals.var_inv_k2);
        let assign16210_e16033: f64 = (assign16210_e16031 + locals.var_temp2);
        let assign16210_e16034: f64 = (locals.var_keq * assign16210_e16033);
        let assign16210_e16035: f64 = (1.0 / assign16210_e16034);
        (assign16210_e16035, (-(((locals.var_keq_dn4 * assign16210_e16033) + (locals.var_keq * (locals.var_inv_k2_dn4 + locals.var_temp2_dn4))) / (assign16210_e16034 * assign16210_e16034))), (-(((locals.var_keq_dn6 * assign16210_e16033) + (locals.var_keq * (locals.var_inv_k2_dn6 + locals.var_temp2_dn6))) / (assign16210_e16034 * assign16210_e16034))), (-(((locals.var_keq_dn7 * assign16210_e16033) + (locals.var_keq * (locals.var_inv_k2_dn7 + locals.var_temp2_dn7))) / (assign16210_e16034 * assign16210_e16034))), (-(((locals.var_keq_dn8 * assign16210_e16033) + (locals.var_keq * (locals.var_inv_k2_dn8 + locals.var_temp2_dn8))) / (assign16210_e16034 * assign16210_e16034))), (-(((locals.var_keq_dn9 * assign16210_e16033) + (locals.var_keq * (locals.var_inv_k2_dn9 + locals.var_temp2_dn9))) / (assign16210_e16034 * assign16210_e16034))),)
    } else {
        (locals.var_q1chapinf, locals.var_q1chapinf_dn4, locals.var_q1chapinf_dn6, locals.var_q1chapinf_dn7, locals.var_q1chapinf_dn8, locals.var_q1chapinf_dn9,)
    }
};
        locals.var_q1chapinf = assign16210_e16037;
        locals.var_q1chapinf_dn4 = assign16210_e16037_d_n4;
        locals.var_q1chapinf_dn6 = assign16210_e16037_d_n6;
        locals.var_q1chapinf_dn7 = assign16210_e16037_d_n7;
        locals.var_q1chapinf_dn8 = assign16210_e16037_d_n8;
        locals.var_q1chapinf_dn9 = assign16210_e16037_d_n9;

        let (assign16220_e16050, assign16220_e16050_d_n4, assign16220_e16050_d_n6, assign16220_e16050_d_n7, assign16220_e16050_d_n8, assign16220_e16050_d_n9,) = {
    if (locals.var_guard606 == 0.0) {
        let assign16220_e16044: f64 = (0.5 + locals.var_inv_k1);
        let assign16220_e16046: f64 = (assign16220_e16044 - locals.var_temp2);
        let assign16220_e16047: f64 = (locals.var_keq * assign16220_e16046);
        let assign16220_e16048: f64 = (1.0 / assign16220_e16047);
        (assign16220_e16048, (-(((locals.var_keq_dn4 * assign16220_e16046) + (locals.var_keq * (locals.var_inv_k1_dn4 - locals.var_temp2_dn4))) / (assign16220_e16047 * assign16220_e16047))), (-(((locals.var_keq_dn6 * assign16220_e16046) + (locals.var_keq * (locals.var_inv_k1_dn6 - locals.var_temp2_dn6))) / (assign16220_e16047 * assign16220_e16047))), (-(((locals.var_keq_dn7 * assign16220_e16046) + (locals.var_keq * (locals.var_inv_k1_dn7 - locals.var_temp2_dn7))) / (assign16220_e16047 * assign16220_e16047))), (-(((locals.var_keq_dn8 * assign16220_e16046) + (locals.var_keq * (locals.var_inv_k1_dn8 - locals.var_temp2_dn8))) / (assign16220_e16047 * assign16220_e16047))), (-(((locals.var_keq_dn9 * assign16220_e16046) + (locals.var_keq * (locals.var_inv_k1_dn9 - locals.var_temp2_dn9))) / (assign16220_e16047 * assign16220_e16047))),)
    } else {
        (locals.var_q2chapinf, locals.var_q2chapinf_dn4, locals.var_q2chapinf_dn6, locals.var_q2chapinf_dn7, locals.var_q2chapinf_dn8, locals.var_q2chapinf_dn9,)
    }
};
        locals.var_q2chapinf = assign16220_e16050;
        locals.var_q2chapinf_dn4 = assign16220_e16050_d_n4;
        locals.var_q2chapinf_dn6 = assign16220_e16050_d_n6;
        locals.var_q2chapinf_dn7 = assign16220_e16050_d_n7;
        locals.var_q2chapinf_dn8 = assign16220_e16050_d_n8;
        locals.var_q2chapinf_dn9 = assign16220_e16050_d_n9;

        let (assign16230_e16072, assign16230_e16072_d_n4, assign16230_e16072_d_n6, assign16230_e16072_d_n7, assign16230_e16072_d_n8, assign16230_e16072_d_n9,) = {
    if (locals.var_guard606 == 0.0) {
        let assign16230_e16058: f64 = (0.5 * locals.var_temp);
        let assign16230_e16059: f64 = (1.0 - assign16230_e16058);
        let assign16230_e16060: f64 = (locals.var_qis * assign16230_e16059);
        let assign16230_e16061: f64 = (locals.var_a0 / assign16230_e16060);
        let assign16230_e16062: f64 = (assign16230_e16061).ln();
        let assign16230_e16064: f64 = (assign16230_e16062 - 0.6931471805599);
        let assign16230_e16068: f64 = (locals.var_x1_wi0 + locals.var_x2_wi0);
        let assign16230_e16069: f64 = (0.5 * assign16230_e16068);
        let assign16230_e16070: f64 = (assign16230_e16064 + assign16230_e16069);
        (assign16230_e16070, (((((locals.var_a0_dn4 * assign16230_e16060) - (locals.var_a0 * ((locals.var_qis_dn4 * assign16230_e16059) + (locals.var_qis * (-(0.5 * locals.var_temp_dn4)))))) / (assign16230_e16060 * assign16230_e16060)) / assign16230_e16061) + (0.5 * (locals.var_x1_wi0_dn4 + locals.var_x2_wi0_dn4))), (((((locals.var_a0_dn6 * assign16230_e16060) - (locals.var_a0 * ((locals.var_qis_dn6 * assign16230_e16059) + (locals.var_qis * (-(0.5 * locals.var_temp_dn6)))))) / (assign16230_e16060 * assign16230_e16060)) / assign16230_e16061) + (0.5 * (locals.var_x1_wi0_dn6 + locals.var_x2_wi0_dn6))), (((((locals.var_a0_dn7 * assign16230_e16060) - (locals.var_a0 * ((locals.var_qis_dn7 * assign16230_e16059) + (locals.var_qis * (-(0.5 * locals.var_temp_dn7)))))) / (assign16230_e16060 * assign16230_e16060)) / assign16230_e16061) + (0.5 * (locals.var_x1_wi0_dn7 + locals.var_x2_wi0_dn7))), (((((locals.var_a0_dn8 * assign16230_e16060) - (locals.var_a0 * ((locals.var_qis_dn8 * assign16230_e16059) + (locals.var_qis * (-(0.5 * locals.var_temp_dn8)))))) / (assign16230_e16060 * assign16230_e16060)) / assign16230_e16061) + (0.5 * (locals.var_x1_wi0_dn8 + locals.var_x2_wi0_dn8))), (((((locals.var_a0_dn9 * assign16230_e16060) - (locals.var_a0 * ((locals.var_qis_dn9 * assign16230_e16059) + (locals.var_qis * (-(0.5 * locals.var_temp_dn9)))))) / (assign16230_e16060 * assign16230_e16060)) / assign16230_e16061) + (0.5 * (locals.var_x1_wi0_dn9 + locals.var_x2_wi0_dn9))),)
    } else {
        (locals.var_deltaxinf, locals.var_deltaxinf_dn4, locals.var_deltaxinf_dn6, locals.var_deltaxinf_dn7, locals.var_deltaxinf_dn8, locals.var_deltaxinf_dn9,)
    }
};
        locals.var_deltaxinf = assign16230_e16072;
        locals.var_deltaxinf_dn4 = assign16230_e16072_d_n4;
        locals.var_deltaxinf_dn6 = assign16230_e16072_d_n6;
        locals.var_deltaxinf_dn7 = assign16230_e16072_d_n7;
        locals.var_deltaxinf_dn8 = assign16230_e16072_d_n8;
        locals.var_deltaxinf_dn9 = assign16230_e16072_d_n9;

        let (assign16240_e16110, assign16240_e16110_d_n4, assign16240_e16110_d_n6, assign16240_e16110_d_n7, assign16240_e16110_d_n8, assign16240_e16110_d_n9,) = {
    if (locals.var_guard606 == 0.0) {
        let assign16240_e16076: f64 = (-12.0);
        let assign16240_e16080: f64 = (3.0 * locals.var_keq);
        let assign16240_e16081: f64 = (4.0 - assign16240_e16080);
        let assign16240_e16084: f64 = (12.0 * locals.var_keq);
        let assign16240_e16087: f64 = (locals.var_k1 * locals.var_k2);
        let assign16240_e16088: f64 = (assign16240_e16084 / assign16240_e16087);
        let assign16240_e16089: f64 = (assign16240_e16081 + assign16240_e16088);
        let assign16240_e16093: f64 = (locals.var_inv_k1 - locals.var_inv_k2);
        let assign16240_e16094: f64 = (locals.var_keq * assign16240_e16093);
        let assign16240_e16096: f64 = (assign16240_e16094 * locals.var_dx_wi);
        let assign16240_e16097: f64 = (assign16240_e16089 + assign16240_e16096);
        let assign16240_e16102: f64 = (0.25 * locals.var_keq);
        let assign16240_e16103: f64 = (0.2 - assign16240_e16102);
        let assign16240_e16104: f64 = (0.3333333333333 * assign16240_e16103);
        let assign16240_e16106: f64 = (assign16240_e16104 * locals.var_dx_wisq);
        let assign16240_e16107: f64 = (assign16240_e16097 + assign16240_e16106);
        let assign16240_e16108: f64 = (assign16240_e16076 / assign16240_e16107);
        (assign16240_e16108, (-((assign16240_e16076 * ((((-(3.0 * locals.var_keq_dn4)) + ((((12.0 * locals.var_keq_dn4) * assign16240_e16087) - (assign16240_e16084 * ((locals.var_k1_dn4 * locals.var_k2) + (locals.var_k1 * locals.var_k2_dn4)))) / (assign16240_e16087 * assign16240_e16087))) + ((((locals.var_keq_dn4 * assign16240_e16093) + (locals.var_keq * (locals.var_inv_k1_dn4 - locals.var_inv_k2_dn4))) * locals.var_dx_wi) + (assign16240_e16094 * locals.var_dx_wi_dn4))) + (((0.3333333333333 * (-(0.25 * locals.var_keq_dn4))) * locals.var_dx_wisq) + (assign16240_e16104 * locals.var_dx_wisq_dn4)))) / (assign16240_e16107 * assign16240_e16107))), (-((assign16240_e16076 * ((((-(3.0 * locals.var_keq_dn6)) + ((((12.0 * locals.var_keq_dn6) * assign16240_e16087) - (assign16240_e16084 * ((locals.var_k1_dn6 * locals.var_k2) + (locals.var_k1 * locals.var_k2_dn6)))) / (assign16240_e16087 * assign16240_e16087))) + ((((locals.var_keq_dn6 * assign16240_e16093) + (locals.var_keq * (locals.var_inv_k1_dn6 - locals.var_inv_k2_dn6))) * locals.var_dx_wi) + (assign16240_e16094 * locals.var_dx_wi_dn6))) + (((0.3333333333333 * (-(0.25 * locals.var_keq_dn6))) * locals.var_dx_wisq) + (assign16240_e16104 * locals.var_dx_wisq_dn6)))) / (assign16240_e16107 * assign16240_e16107))), (-((assign16240_e16076 * ((((-(3.0 * locals.var_keq_dn7)) + ((((12.0 * locals.var_keq_dn7) * assign16240_e16087) - (assign16240_e16084 * ((locals.var_k1_dn7 * locals.var_k2) + (locals.var_k1 * locals.var_k2_dn7)))) / (assign16240_e16087 * assign16240_e16087))) + ((((locals.var_keq_dn7 * assign16240_e16093) + (locals.var_keq * (locals.var_inv_k1_dn7 - locals.var_inv_k2_dn7))) * locals.var_dx_wi) + (assign16240_e16094 * locals.var_dx_wi_dn7))) + (((0.3333333333333 * (-(0.25 * locals.var_keq_dn7))) * locals.var_dx_wisq) + (assign16240_e16104 * locals.var_dx_wisq_dn7)))) / (assign16240_e16107 * assign16240_e16107))), (-((assign16240_e16076 * ((((-(3.0 * locals.var_keq_dn8)) + ((((12.0 * locals.var_keq_dn8) * assign16240_e16087) - (assign16240_e16084 * ((locals.var_k1_dn8 * locals.var_k2) + (locals.var_k1 * locals.var_k2_dn8)))) / (assign16240_e16087 * assign16240_e16087))) + ((((locals.var_keq_dn8 * assign16240_e16093) + (locals.var_keq * (locals.var_inv_k1_dn8 - locals.var_inv_k2_dn8))) * locals.var_dx_wi) + (assign16240_e16094 * locals.var_dx_wi_dn8))) + (((0.3333333333333 * (-(0.25 * locals.var_keq_dn8))) * locals.var_dx_wisq) + (assign16240_e16104 * locals.var_dx_wisq_dn8)))) / (assign16240_e16107 * assign16240_e16107))), (-((assign16240_e16076 * ((((-(3.0 * locals.var_keq_dn9)) + ((((12.0 * locals.var_keq_dn9) * assign16240_e16087) - (assign16240_e16084 * ((locals.var_k1_dn9 * locals.var_k2) + (locals.var_k1 * locals.var_k2_dn9)))) / (assign16240_e16087 * assign16240_e16087))) + ((((locals.var_keq_dn9 * assign16240_e16093) + (locals.var_keq * (locals.var_inv_k1_dn9 - locals.var_inv_k2_dn9))) * locals.var_dx_wi) + (assign16240_e16094 * locals.var_dx_wi_dn9))) + (((0.3333333333333 * (-(0.25 * locals.var_keq_dn9))) * locals.var_dx_wisq) + (assign16240_e16104 * locals.var_dx_wisq_dn9)))) / (assign16240_e16107 * assign16240_e16107))),)
    } else {
        (locals.var_dinf, locals.var_dinf_dn4, locals.var_dinf_dn6, locals.var_dinf_dn7, locals.var_dinf_dn8, locals.var_dinf_dn9,)
    }
};
        locals.var_dinf = assign16240_e16110;
        locals.var_dinf_dn4 = assign16240_e16110_d_n4;
        locals.var_dinf_dn6 = assign16240_e16110_d_n6;
        locals.var_dinf_dn7 = assign16240_e16110_d_n7;
        locals.var_dinf_dn8 = assign16240_e16110_d_n8;
        locals.var_dinf_dn9 = assign16240_e16110_d_n9;

        let assign16250_e16113: f64 = (1.0 / locals.var_dinf);
        locals.var_inv_dinf = assign16250_e16113;
        locals.var_inv_dinf_dn4 = (-(locals.var_dinf_dn4 / (locals.var_dinf * locals.var_dinf)));
        locals.var_inv_dinf_dn6 = (-(locals.var_dinf_dn6 / (locals.var_dinf * locals.var_dinf)));
        locals.var_inv_dinf_dn7 = (-(locals.var_dinf_dn7 / (locals.var_dinf * locals.var_dinf)));
        locals.var_inv_dinf_dn8 = (-(locals.var_dinf_dn8 / (locals.var_dinf * locals.var_dinf)));
        locals.var_inv_dinf_dn9 = (-(locals.var_dinf_dn9 / (locals.var_dinf * locals.var_dinf)));

        let assign16260_e16116: f64 = if locals.var_qis > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard608 = assign16260_e16116;

        let (assign16270_e16126, assign16270_e16126_d_n4, assign16270_e16126_d_n6, assign16270_e16126_d_n7, assign16270_e16126_d_n8, assign16270_e16126_d_n9,) = {
    if (locals.var_guard608 != 0.0) {
        let assign16270_e16120: f64 = (100.0 * locals.var_esurf1s);
        let assign16270_e16123: f64 = (100.0 + locals.var_esurf1s);
        let assign16270_e16124: f64 = (assign16270_e16120 / assign16270_e16123);
        (assign16270_e16124, ((((100.0 * locals.var_esurf1s_dn4) * assign16270_e16123) - (assign16270_e16120 * locals.var_esurf1s_dn4)) / (assign16270_e16123 * assign16270_e16123)), ((((100.0 * locals.var_esurf1s_dn6) * assign16270_e16123) - (assign16270_e16120 * locals.var_esurf1s_dn6)) / (assign16270_e16123 * assign16270_e16123)), ((((100.0 * locals.var_esurf1s_dn7) * assign16270_e16123) - (assign16270_e16120 * locals.var_esurf1s_dn7)) / (assign16270_e16123 * assign16270_e16123)), ((((100.0 * locals.var_esurf1s_dn8) * assign16270_e16123) - (assign16270_e16120 * locals.var_esurf1s_dn8)) / (assign16270_e16123 * assign16270_e16123)), ((((100.0 * locals.var_esurf1s_dn9) * assign16270_e16123) - (assign16270_e16120 * locals.var_esurf1s_dn9)) / (assign16270_e16123 * assign16270_e16123)),)
    } else {
        (locals.var_wsat1, locals.var_wsat1_dn4, locals.var_wsat1_dn6, locals.var_wsat1_dn7, locals.var_wsat1_dn8, locals.var_wsat1_dn9,)
    }
};
        locals.var_wsat1 = assign16270_e16126;
        locals.var_wsat1_dn4 = assign16270_e16126_d_n4;
        locals.var_wsat1_dn6 = assign16270_e16126_d_n6;
        locals.var_wsat1_dn7 = assign16270_e16126_d_n7;
        locals.var_wsat1_dn8 = assign16270_e16126_d_n8;
        locals.var_wsat1_dn9 = assign16270_e16126_d_n9;

        let assign16280_e16129: f64 = if locals.var_thesat1_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard609 = assign16280_e16129;

        let (assign16290_e16141, assign16290_e16141_d_n4, assign16290_e16141_d_n6, assign16290_e16141_d_n7, assign16290_e16141_d_n8, assign16290_e16141_d_n9,) = {
    if ((locals.var_guard608 != 0.0) && (locals.var_guard609 != 0.0)) {
        let assign16290_e16137: f64 = (locals.var_thesat1_i * locals.var_wsat1);
        let assign16290_e16138: f64 = (1.0 - assign16290_e16137);
        let assign16290_e16139: f64 = (1.0 / assign16290_e16138);
        (assign16290_e16139, (-((-(locals.var_thesat1_i * locals.var_wsat1_dn4)) / (assign16290_e16138 * assign16290_e16138))), (-((-(locals.var_thesat1_i * locals.var_wsat1_dn6)) / (assign16290_e16138 * assign16290_e16138))), (-((-(locals.var_thesat1_i * locals.var_wsat1_dn7)) / (assign16290_e16138 * assign16290_e16138))), (-((-(locals.var_thesat1_i * locals.var_wsat1_dn8)) / (assign16290_e16138 * assign16290_e16138))), (-((-(locals.var_thesat1_i * locals.var_wsat1_dn9)) / (assign16290_e16138 * assign16290_e16138))),)
    } else {
        (locals.var_sat_fact1, locals.var_sat_fact1_dn4, locals.var_sat_fact1_dn6, locals.var_sat_fact1_dn7, locals.var_sat_fact1_dn8, locals.var_sat_fact1_dn9,)
    }
};
        locals.var_sat_fact1 = assign16290_e16141;
        locals.var_sat_fact1_dn4 = assign16290_e16141_d_n4;
        locals.var_sat_fact1_dn6 = assign16290_e16141_d_n6;
        locals.var_sat_fact1_dn7 = assign16290_e16141_d_n7;
        locals.var_sat_fact1_dn8 = assign16290_e16141_d_n8;
        locals.var_sat_fact1_dn9 = assign16290_e16141_d_n9;

        let (assign16300_e16152, assign16300_e16152_d_n4, assign16300_e16152_d_n6, assign16300_e16152_d_n7, assign16300_e16152_d_n8, assign16300_e16152_d_n9,) = {
    if ((locals.var_guard608 != 0.0) && (locals.var_guard609 == 0.0)) {
        let assign16300_e16149: f64 = (locals.var_thesat1_i * locals.var_wsat1);
        let assign16300_e16150: f64 = (1.0 + assign16300_e16149);
        (assign16300_e16150, (locals.var_thesat1_i * locals.var_wsat1_dn4), (locals.var_thesat1_i * locals.var_wsat1_dn6), (locals.var_thesat1_i * locals.var_wsat1_dn7), (locals.var_thesat1_i * locals.var_wsat1_dn8), (locals.var_thesat1_i * locals.var_wsat1_dn9),)
    } else {
        (locals.var_sat_fact1, locals.var_sat_fact1_dn4, locals.var_sat_fact1_dn6, locals.var_sat_fact1_dn7, locals.var_sat_fact1_dn8, locals.var_sat_fact1_dn9,)
    }
};
        locals.var_sat_fact1 = assign16300_e16152;
        locals.var_sat_fact1_dn4 = assign16300_e16152_d_n4;
        locals.var_sat_fact1_dn6 = assign16300_e16152_d_n6;
        locals.var_sat_fact1_dn7 = assign16300_e16152_d_n7;
        locals.var_sat_fact1_dn8 = assign16300_e16152_d_n8;
        locals.var_sat_fact1_dn9 = assign16300_e16152_d_n9;

        let (assign16310_e16162, assign16310_e16162_d_n4, assign16310_e16162_d_n6, assign16310_e16162_d_n7, assign16310_e16162_d_n8, assign16310_e16162_d_n9,) = {
    if (locals.var_guard608 != 0.0) {
        let assign16310_e16156: f64 = (100.0 * locals.var_esurf2s);
        let assign16310_e16159: f64 = (100.0 + locals.var_esurf2s);
        let assign16310_e16160: f64 = (assign16310_e16156 / assign16310_e16159);
        (assign16310_e16160, ((((100.0 * locals.var_esurf2s_dn4) * assign16310_e16159) - (assign16310_e16156 * locals.var_esurf2s_dn4)) / (assign16310_e16159 * assign16310_e16159)), ((((100.0 * locals.var_esurf2s_dn6) * assign16310_e16159) - (assign16310_e16156 * locals.var_esurf2s_dn6)) / (assign16310_e16159 * assign16310_e16159)), ((((100.0 * locals.var_esurf2s_dn7) * assign16310_e16159) - (assign16310_e16156 * locals.var_esurf2s_dn7)) / (assign16310_e16159 * assign16310_e16159)), ((((100.0 * locals.var_esurf2s_dn8) * assign16310_e16159) - (assign16310_e16156 * locals.var_esurf2s_dn8)) / (assign16310_e16159 * assign16310_e16159)), ((((100.0 * locals.var_esurf2s_dn9) * assign16310_e16159) - (assign16310_e16156 * locals.var_esurf2s_dn9)) / (assign16310_e16159 * assign16310_e16159)),)
    } else {
        (locals.var_wsat2, locals.var_wsat2_dn4, locals.var_wsat2_dn6, locals.var_wsat2_dn7, locals.var_wsat2_dn8, locals.var_wsat2_dn9,)
    }
};
        locals.var_wsat2 = assign16310_e16162;
        locals.var_wsat2_dn4 = assign16310_e16162_d_n4;
        locals.var_wsat2_dn6 = assign16310_e16162_d_n6;
        locals.var_wsat2_dn7 = assign16310_e16162_d_n7;
        locals.var_wsat2_dn8 = assign16310_e16162_d_n8;
        locals.var_wsat2_dn9 = assign16310_e16162_d_n9;

        let assign16320_e16165: f64 = if locals.var_thesat2_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard610 = assign16320_e16165;

        let (assign16330_e16177, assign16330_e16177_d_n4, assign16330_e16177_d_n6, assign16330_e16177_d_n7, assign16330_e16177_d_n8, assign16330_e16177_d_n9,) = {
    if ((locals.var_guard608 != 0.0) && (locals.var_guard610 != 0.0)) {
        let assign16330_e16173: f64 = (locals.var_thesat2_i * locals.var_wsat2);
        let assign16330_e16174: f64 = (1.0 - assign16330_e16173);
        let assign16330_e16175: f64 = (1.0 / assign16330_e16174);
        (assign16330_e16175, (-((-(locals.var_thesat2_i * locals.var_wsat2_dn4)) / (assign16330_e16174 * assign16330_e16174))), (-((-(locals.var_thesat2_i * locals.var_wsat2_dn6)) / (assign16330_e16174 * assign16330_e16174))), (-((-(locals.var_thesat2_i * locals.var_wsat2_dn7)) / (assign16330_e16174 * assign16330_e16174))), (-((-(locals.var_thesat2_i * locals.var_wsat2_dn8)) / (assign16330_e16174 * assign16330_e16174))), (-((-(locals.var_thesat2_i * locals.var_wsat2_dn9)) / (assign16330_e16174 * assign16330_e16174))),)
    } else {
        (locals.var_sat_fact2, locals.var_sat_fact2_dn4, locals.var_sat_fact2_dn6, locals.var_sat_fact2_dn7, locals.var_sat_fact2_dn8, locals.var_sat_fact2_dn9,)
    }
};
        locals.var_sat_fact2 = assign16330_e16177;
        locals.var_sat_fact2_dn4 = assign16330_e16177_d_n4;
        locals.var_sat_fact2_dn6 = assign16330_e16177_d_n6;
        locals.var_sat_fact2_dn7 = assign16330_e16177_d_n7;
        locals.var_sat_fact2_dn8 = assign16330_e16177_d_n8;
        locals.var_sat_fact2_dn9 = assign16330_e16177_d_n9;

        let (assign16340_e16188, assign16340_e16188_d_n4, assign16340_e16188_d_n6, assign16340_e16188_d_n7, assign16340_e16188_d_n8, assign16340_e16188_d_n9,) = {
    if ((locals.var_guard608 != 0.0) && (locals.var_guard610 == 0.0)) {
        let assign16340_e16185: f64 = (locals.var_thesat2_i * locals.var_wsat2);
        let assign16340_e16186: f64 = (1.0 + assign16340_e16185);
        (assign16340_e16186, (locals.var_thesat2_i * locals.var_wsat2_dn4), (locals.var_thesat2_i * locals.var_wsat2_dn6), (locals.var_thesat2_i * locals.var_wsat2_dn7), (locals.var_thesat2_i * locals.var_wsat2_dn8), (locals.var_thesat2_i * locals.var_wsat2_dn9),)
    } else {
        (locals.var_sat_fact2, locals.var_sat_fact2_dn4, locals.var_sat_fact2_dn6, locals.var_sat_fact2_dn7, locals.var_sat_fact2_dn8, locals.var_sat_fact2_dn9,)
    }
};
        locals.var_sat_fact2 = assign16340_e16188;
        locals.var_sat_fact2_dn4 = assign16340_e16188_d_n4;
        locals.var_sat_fact2_dn6 = assign16340_e16188_d_n6;
        locals.var_sat_fact2_dn7 = assign16340_e16188_d_n7;
        locals.var_sat_fact2_dn8 = assign16340_e16188_d_n8;
        locals.var_sat_fact2_dn9 = assign16340_e16188_d_n9;

        let (assign16350_e16208, assign16350_e16208_d_n4, assign16350_e16208_d_n6, assign16350_e16208_d_n7, assign16350_e16208_d_n8, assign16350_e16208_d_n9,) = {
    if (locals.var_guard608 != 0.0) {
        let assign16350_e16192: f64 = (locals.var_dqsqs_dxn_qi * locals.var_sums);
        let assign16350_e16195: f64 = (locals.var_a1s * locals.var_a2s);
        let assign16350_e16196: f64 = (assign16350_e16192 / assign16350_e16195);
        let assign16350_e16199: f64 = (locals.var_aexp1s / locals.var_a1s);
        let assign16350_e16202: f64 = (locals.var_aexp2s / locals.var_a2s);
        let assign16350_e16203: f64 = (assign16350_e16199 + assign16350_e16202);
        let assign16350_e16205: f64 = (assign16350_e16203 / locals.var_qis);
        let assign16350_e16206: f64 = (assign16350_e16196 - assign16350_e16205);
        (assign16350_e16206, ((((((locals.var_dqsqs_dxn_qi_dn4 * locals.var_sums) + (locals.var_dqsqs_dxn_qi * locals.var_sums_dn4)) * assign16350_e16195) - (assign16350_e16192 * ((locals.var_a1s_dn4 * locals.var_a2s) + (locals.var_a1s * locals.var_a2s_dn4)))) / (assign16350_e16195 * assign16350_e16195)) - (((((((locals.var_aexp1s_dn4 * locals.var_a1s) - (locals.var_aexp1s * locals.var_a1s_dn4)) / (locals.var_a1s * locals.var_a1s)) + (((locals.var_aexp2s_dn4 * locals.var_a2s) - (locals.var_aexp2s * locals.var_a2s_dn4)) / (locals.var_a2s * locals.var_a2s))) * locals.var_qis) - (assign16350_e16203 * locals.var_qis_dn4)) / (locals.var_qis * locals.var_qis))), ((((((locals.var_dqsqs_dxn_qi_dn6 * locals.var_sums) + (locals.var_dqsqs_dxn_qi * locals.var_sums_dn6)) * assign16350_e16195) - (assign16350_e16192 * ((locals.var_a1s_dn6 * locals.var_a2s) + (locals.var_a1s * locals.var_a2s_dn6)))) / (assign16350_e16195 * assign16350_e16195)) - (((((((locals.var_aexp1s_dn6 * locals.var_a1s) - (locals.var_aexp1s * locals.var_a1s_dn6)) / (locals.var_a1s * locals.var_a1s)) + (((locals.var_aexp2s_dn6 * locals.var_a2s) - (locals.var_aexp2s * locals.var_a2s_dn6)) / (locals.var_a2s * locals.var_a2s))) * locals.var_qis) - (assign16350_e16203 * locals.var_qis_dn6)) / (locals.var_qis * locals.var_qis))), ((((((locals.var_dqsqs_dxn_qi_dn7 * locals.var_sums) + (locals.var_dqsqs_dxn_qi * locals.var_sums_dn7)) * assign16350_e16195) - (assign16350_e16192 * ((locals.var_a1s_dn7 * locals.var_a2s) + (locals.var_a1s * locals.var_a2s_dn7)))) / (assign16350_e16195 * assign16350_e16195)) - (((((((locals.var_aexp1s_dn7 * locals.var_a1s) - (locals.var_aexp1s * locals.var_a1s_dn7)) / (locals.var_a1s * locals.var_a1s)) + (((locals.var_aexp2s_dn7 * locals.var_a2s) - (locals.var_aexp2s * locals.var_a2s_dn7)) / (locals.var_a2s * locals.var_a2s))) * locals.var_qis) - (assign16350_e16203 * locals.var_qis_dn7)) / (locals.var_qis * locals.var_qis))), ((((((locals.var_dqsqs_dxn_qi_dn8 * locals.var_sums) + (locals.var_dqsqs_dxn_qi * locals.var_sums_dn8)) * assign16350_e16195) - (assign16350_e16192 * ((locals.var_a1s_dn8 * locals.var_a2s) + (locals.var_a1s * locals.var_a2s_dn8)))) / (assign16350_e16195 * assign16350_e16195)) - (((((((locals.var_aexp1s_dn8 * locals.var_a1s) - (locals.var_aexp1s * locals.var_a1s_dn8)) / (locals.var_a1s * locals.var_a1s)) + (((locals.var_aexp2s_dn8 * locals.var_a2s) - (locals.var_aexp2s * locals.var_a2s_dn8)) / (locals.var_a2s * locals.var_a2s))) * locals.var_qis) - (assign16350_e16203 * locals.var_qis_dn8)) / (locals.var_qis * locals.var_qis))), ((((((locals.var_dqsqs_dxn_qi_dn9 * locals.var_sums) + (locals.var_dqsqs_dxn_qi * locals.var_sums_dn9)) * assign16350_e16195) - (assign16350_e16192 * ((locals.var_a1s_dn9 * locals.var_a2s) + (locals.var_a1s * locals.var_a2s_dn9)))) / (assign16350_e16195 * assign16350_e16195)) - (((((((locals.var_aexp1s_dn9 * locals.var_a1s) - (locals.var_aexp1s * locals.var_a1s_dn9)) / (locals.var_a1s * locals.var_a1s)) + (((locals.var_aexp2s_dn9 * locals.var_a2s) - (locals.var_aexp2s * locals.var_a2s_dn9)) / (locals.var_a2s * locals.var_a2s))) * locals.var_qis) - (assign16350_e16203 * locals.var_qis_dn9)) / (locals.var_qis * locals.var_qis))),)
    } else {
        (locals.var_dqis_dxn_qi, locals.var_dqis_dxn_qi_dn4, locals.var_dqis_dxn_qi_dn6, locals.var_dqis_dxn_qi_dn7, locals.var_dqis_dxn_qi_dn8, locals.var_dqis_dxn_qi_dn9,)
    }
};
        locals.var_dqis_dxn_qi = assign16350_e16208;
        locals.var_dqis_dxn_qi_dn4 = assign16350_e16208_d_n4;
        locals.var_dqis_dxn_qi_dn6 = assign16350_e16208_d_n6;
        locals.var_dqis_dxn_qi_dn7 = assign16350_e16208_d_n7;
        locals.var_dqis_dxn_qi_dn8 = assign16350_e16208_d_n8;
        locals.var_dqis_dxn_qi_dn9 = assign16350_e16208_d_n9;

        let (assign16360_e16218, assign16360_e16218_d_n4, assign16360_e16218_d_n6, assign16360_e16218_d_n7, assign16360_e16218_d_n8, assign16360_e16218_d_n9,) = {
    if (locals.var_guard608 != 0.0) {
        let assign16360_e16212: f64 = (locals.var_dqis_dxn_qi * locals.var_qis);
        let assign16360_e16215: f64 = (locals.var_dqis_dxn_qi + 1.0);
        let assign16360_e16216: f64 = (assign16360_e16212 / assign16360_e16215);
        (assign16360_e16216, (((((locals.var_dqis_dxn_qi_dn4 * locals.var_qis) + (locals.var_dqis_dxn_qi * locals.var_qis_dn4)) * assign16360_e16215) - (assign16360_e16212 * locals.var_dqis_dxn_qi_dn4)) / (assign16360_e16215 * assign16360_e16215)), (((((locals.var_dqis_dxn_qi_dn6 * locals.var_qis) + (locals.var_dqis_dxn_qi * locals.var_qis_dn6)) * assign16360_e16215) - (assign16360_e16212 * locals.var_dqis_dxn_qi_dn6)) / (assign16360_e16215 * assign16360_e16215)), (((((locals.var_dqis_dxn_qi_dn7 * locals.var_qis) + (locals.var_dqis_dxn_qi * locals.var_qis_dn7)) * assign16360_e16215) - (assign16360_e16212 * locals.var_dqis_dxn_qi_dn7)) / (assign16360_e16215 * assign16360_e16215)), (((((locals.var_dqis_dxn_qi_dn8 * locals.var_qis) + (locals.var_dqis_dxn_qi * locals.var_qis_dn8)) * assign16360_e16215) - (assign16360_e16212 * locals.var_dqis_dxn_qi_dn8)) / (assign16360_e16215 * assign16360_e16215)), (((((locals.var_dqis_dxn_qi_dn9 * locals.var_qis) + (locals.var_dqis_dxn_qi * locals.var_qis_dn9)) * assign16360_e16215) - (assign16360_e16212 * locals.var_dqis_dxn_qi_dn9)) / (assign16360_e16215 * assign16360_e16215)),)
    } else {
        (locals.var_ds, locals.var_ds_dn4, locals.var_ds_dn6, locals.var_ds_dn7, locals.var_ds_dn8, locals.var_ds_dn9,)
    }
};
        locals.var_ds = assign16360_e16218;
        locals.var_ds_dn4 = assign16360_e16218_d_n4;
        locals.var_ds_dn6 = assign16360_e16218_d_n6;
        locals.var_ds_dn7 = assign16360_e16218_d_n7;
        locals.var_ds_dn8 = assign16360_e16218_d_n8;
        locals.var_ds_dn9 = assign16360_e16218_d_n9;

        let (assign16370_e16224, assign16370_e16224_d_n4, assign16370_e16224_d_n6, assign16370_e16224_d_n7, assign16370_e16224_d_n8, assign16370_e16224_d_n9,) = {
    if (locals.var_guard608 != 0.0) {
        let assign16370_e16222: f64 = (locals.var_dinf - locals.var_ds);
        (assign16370_e16222, (locals.var_dinf_dn4 - locals.var_ds_dn4), (locals.var_dinf_dn6 - locals.var_ds_dn6), (locals.var_dinf_dn7 - locals.var_ds_dn7), (locals.var_dinf_dn8 - locals.var_ds_dn8), (locals.var_dinf_dn9 - locals.var_ds_dn9),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign16370_e16224;
        locals.var_temp1_dn4 = assign16370_e16224_d_n4;
        locals.var_temp1_dn6 = assign16370_e16224_d_n6;
        locals.var_temp1_dn7 = assign16370_e16224_d_n7;
        locals.var_temp1_dn8 = assign16370_e16224_d_n8;
        locals.var_temp1_dn9 = assign16370_e16224_d_n9;

        let (assign16380_e16234, assign16380_e16234_d_n4, assign16380_e16234_d_n6, assign16380_e16234_d_n7, assign16380_e16234_d_n8, assign16380_e16234_d_n9,) = {
    if (locals.var_guard608 != 0.0) {
        let assign16380_e16229: f64 = (locals.var_dinf * locals.var_deltaxinf);
        let assign16380_e16230: f64 = (locals.var_qis + assign16380_e16229);
        let assign16380_e16232: f64 = (assign16380_e16230 / locals.var_temp1);
        (assign16380_e16232, ((((locals.var_qis_dn4 + ((locals.var_dinf_dn4 * locals.var_deltaxinf) + (locals.var_dinf * locals.var_deltaxinf_dn4))) * locals.var_temp1) - (assign16380_e16230 * locals.var_temp1_dn4)) / (locals.var_temp1 * locals.var_temp1)), ((((locals.var_qis_dn6 + ((locals.var_dinf_dn6 * locals.var_deltaxinf) + (locals.var_dinf * locals.var_deltaxinf_dn6))) * locals.var_temp1) - (assign16380_e16230 * locals.var_temp1_dn6)) / (locals.var_temp1 * locals.var_temp1)), ((((locals.var_qis_dn7 + ((locals.var_dinf_dn7 * locals.var_deltaxinf) + (locals.var_dinf * locals.var_deltaxinf_dn7))) * locals.var_temp1) - (assign16380_e16230 * locals.var_temp1_dn7)) / (locals.var_temp1 * locals.var_temp1)), ((((locals.var_qis_dn8 + ((locals.var_dinf_dn8 * locals.var_deltaxinf) + (locals.var_dinf * locals.var_deltaxinf_dn8))) * locals.var_temp1) - (assign16380_e16230 * locals.var_temp1_dn8)) / (locals.var_temp1 * locals.var_temp1)), ((((locals.var_qis_dn9 + ((locals.var_dinf_dn9 * locals.var_deltaxinf) + (locals.var_dinf * locals.var_deltaxinf_dn9))) * locals.var_temp1) - (assign16380_e16230 * locals.var_temp1_dn9)) / (locals.var_temp1 * locals.var_temp1)),)
    } else {
        (locals.var_deltaxi, locals.var_deltaxi_dn4, locals.var_deltaxi_dn6, locals.var_deltaxi_dn7, locals.var_deltaxi_dn8, locals.var_deltaxi_dn9,)
    }
};
        locals.var_deltaxi = assign16380_e16234;
        locals.var_deltaxi_dn4 = assign16380_e16234_d_n4;
        locals.var_deltaxi_dn6 = assign16380_e16234_d_n6;
        locals.var_deltaxi_dn7 = assign16380_e16234_d_n7;
        locals.var_deltaxi_dn8 = assign16380_e16234_d_n8;
        locals.var_deltaxi_dn9 = assign16380_e16234_d_n9;

        let (assign16390_e16247, assign16390_e16247_d_n4, assign16390_e16247_d_n6, assign16390_e16247_d_n7, assign16390_e16247_d_n8, assign16390_e16247_d_n9,) = {
    if (locals.var_guard608 != 0.0) {
        let assign16390_e16240: f64 = (locals.var_deltaxi * locals.var_deltaxi);
        let assign16390_e16242: f64 = (assign16390_e16240 + 1e-6);
        let assign16390_e16243: f64 = (assign16390_e16242).sqrt();
        let assign16390_e16244: f64 = (locals.var_deltaxi + assign16390_e16243);
        let assign16390_e16245: f64 = (0.5 * assign16390_e16244);
        (assign16390_e16245, (0.5 * (locals.var_deltaxi_dn4 + (((locals.var_deltaxi_dn4 * locals.var_deltaxi) + (locals.var_deltaxi * locals.var_deltaxi_dn4)) / (2.0 * assign16390_e16243)))), (0.5 * (locals.var_deltaxi_dn6 + (((locals.var_deltaxi_dn6 * locals.var_deltaxi) + (locals.var_deltaxi * locals.var_deltaxi_dn6)) / (2.0 * assign16390_e16243)))), (0.5 * (locals.var_deltaxi_dn7 + (((locals.var_deltaxi_dn7 * locals.var_deltaxi) + (locals.var_deltaxi * locals.var_deltaxi_dn7)) / (2.0 * assign16390_e16243)))), (0.5 * (locals.var_deltaxi_dn8 + (((locals.var_deltaxi_dn8 * locals.var_deltaxi) + (locals.var_deltaxi * locals.var_deltaxi_dn8)) / (2.0 * assign16390_e16243)))), (0.5 * (locals.var_deltaxi_dn9 + (((locals.var_deltaxi_dn9 * locals.var_deltaxi) + (locals.var_deltaxi * locals.var_deltaxi_dn9)) / (2.0 * assign16390_e16243)))),)
    } else {
        (locals.var_deltaxi, locals.var_deltaxi_dn4, locals.var_deltaxi_dn6, locals.var_deltaxi_dn7, locals.var_deltaxi_dn8, locals.var_deltaxi_dn9,)
    }
};
        locals.var_deltaxi = assign16390_e16247;
        locals.var_deltaxi_dn4 = assign16390_e16247_d_n4;
        locals.var_deltaxi_dn6 = assign16390_e16247_d_n6;
        locals.var_deltaxi_dn7 = assign16390_e16247_d_n7;
        locals.var_deltaxi_dn8 = assign16390_e16247_d_n8;
        locals.var_deltaxi_dn9 = assign16390_e16247_d_n9;

        let (assign16400_e16259, assign16400_e16259_d_n4, assign16400_e16259_d_n6, assign16400_e16259_d_n7, assign16400_e16259_d_n8, assign16400_e16259_d_n9,) = {
    if (locals.var_guard608 != 0.0) {
        let assign16400_e16251: f64 = (locals.var_sat_phit_loc / locals.var_gmobs);
        let assign16400_e16253: f64 = (assign16400_e16251 * 0.5);
        let assign16400_e16256: f64 = (locals.var_sat_fact1 + locals.var_sat_fact2);
        let assign16400_e16257: f64 = (assign16400_e16253 * assign16400_e16256);
        (assign16400_e16257, ((((((locals.var_sat_phit_loc_dn4 * locals.var_gmobs) - (locals.var_sat_phit_loc * locals.var_gmobs_dn4)) / (locals.var_gmobs * locals.var_gmobs)) * 0.5) * assign16400_e16256) + (assign16400_e16253 * (locals.var_sat_fact1_dn4 + locals.var_sat_fact2_dn4))), ((((((locals.var_sat_phit_loc_dn6 * locals.var_gmobs) - (locals.var_sat_phit_loc * locals.var_gmobs_dn6)) / (locals.var_gmobs * locals.var_gmobs)) * 0.5) * assign16400_e16256) + (assign16400_e16253 * (locals.var_sat_fact1_dn6 + locals.var_sat_fact2_dn6))), ((((((locals.var_sat_phit_loc_dn7 * locals.var_gmobs) - (locals.var_sat_phit_loc * locals.var_gmobs_dn7)) / (locals.var_gmobs * locals.var_gmobs)) * 0.5) * assign16400_e16256) + (assign16400_e16253 * (locals.var_sat_fact1_dn7 + locals.var_sat_fact2_dn7))), ((((((locals.var_sat_phit_loc_dn8 * locals.var_gmobs) - (locals.var_sat_phit_loc * locals.var_gmobs_dn8)) / (locals.var_gmobs * locals.var_gmobs)) * 0.5) * assign16400_e16256) + (assign16400_e16253 * (locals.var_sat_fact1_dn8 + locals.var_sat_fact2_dn8))), ((((((locals.var_sat_phit_loc_dn9 * locals.var_gmobs) - (locals.var_sat_phit_loc * locals.var_gmobs_dn9)) / (locals.var_gmobs * locals.var_gmobs)) * 0.5) * assign16400_e16256) + (assign16400_e16253 * (locals.var_sat_fact1_dn9 + locals.var_sat_fact2_dn9))),)
    } else {
        (locals.var_gamma, locals.var_gamma_dn4, locals.var_gamma_dn6, locals.var_gamma_dn7, locals.var_gamma_dn8, locals.var_gamma_dn9,)
    }
};
        locals.var_gamma = assign16400_e16259;
        locals.var_gamma_dn4 = assign16400_e16259_d_n4;
        locals.var_gamma_dn6 = assign16400_e16259_d_n6;
        locals.var_gamma_dn7 = assign16400_e16259_d_n7;
        locals.var_gamma_dn8 = assign16400_e16259_d_n8;
        locals.var_gamma_dn9 = assign16400_e16259_d_n9;

        let (assign16410_e16267, assign16410_e16267_d_n4, assign16410_e16267_d_n6, assign16410_e16267_d_n7, assign16410_e16267_d_n8, assign16410_e16267_d_n9,) = {
    if (locals.var_guard608 != 0.0) {
        let assign16410_e16264: f64 = (locals.var_qis / locals.var_ds);
        let assign16410_e16265: f64 = (1.0 - assign16410_e16264);
        (assign16410_e16265, (-(((locals.var_qis_dn4 * locals.var_ds) - (locals.var_qis * locals.var_ds_dn4)) / (locals.var_ds * locals.var_ds))), (-(((locals.var_qis_dn6 * locals.var_ds) - (locals.var_qis * locals.var_ds_dn6)) / (locals.var_ds * locals.var_ds))), (-(((locals.var_qis_dn7 * locals.var_ds) - (locals.var_qis * locals.var_ds_dn7)) / (locals.var_ds * locals.var_ds))), (-(((locals.var_qis_dn8 * locals.var_ds) - (locals.var_qis * locals.var_ds_dn8)) / (locals.var_ds * locals.var_ds))), (-(((locals.var_qis_dn9 * locals.var_ds) - (locals.var_qis * locals.var_ds_dn9)) / (locals.var_ds * locals.var_ds))),)
    } else {
        (locals.var_vs, locals.var_vs_dn4, locals.var_vs_dn6, locals.var_vs_dn7, locals.var_vs_dn8, locals.var_vs_dn9,)
    }
};
        locals.var_vs = assign16410_e16267;
        locals.var_vs_dn4 = assign16410_e16267_d_n4;
        locals.var_vs_dn6 = assign16410_e16267_d_n6;
        locals.var_vs_dn7 = assign16410_e16267_d_n7;
        locals.var_vs_dn8 = assign16410_e16267_d_n8;
        locals.var_vs_dn9 = assign16410_e16267_d_n9;

        let (assign16420_e16273, assign16420_e16273_d_n4, assign16420_e16273_d_n6, assign16420_e16273_d_n7, assign16420_e16273_d_n8, assign16420_e16273_d_n9,) = {
    if (locals.var_guard608 != 0.0) {
        let assign16420_e16271: f64 = (1.0 + locals.var_deltaxinf);
        (assign16420_e16271, locals.var_deltaxinf_dn4, locals.var_deltaxinf_dn6, locals.var_deltaxinf_dn7, locals.var_deltaxinf_dn8, locals.var_deltaxinf_dn9,)
    } else {
        (locals.var_vd, locals.var_vd_dn4, locals.var_vd_dn6, locals.var_vd_dn7, locals.var_vd_dn8, locals.var_vd_dn9,)
    }
};
        locals.var_vd = assign16420_e16273;
        locals.var_vd_dn4 = assign16420_e16273_d_n4;
        locals.var_vd_dn6 = assign16420_e16273_d_n6;
        locals.var_vd_dn7 = assign16420_e16273_d_n7;
        locals.var_vd_dn8 = assign16420_e16273_d_n8;
        locals.var_vd_dn9 = assign16420_e16273_d_n9;

        let (assign16430_e16289, assign16430_e16289_d_n4, assign16430_e16289_d_n6, assign16430_e16289_d_n7, assign16430_e16289_d_n8, assign16430_e16289_d_n9,) = {
    if (locals.var_guard608 != 0.0) {
        let assign16430_e16277: f64 = (2.0 * locals.var_ds);
        let assign16430_e16279: f64 = (assign16430_e16277 - locals.var_qis);
        let assign16430_e16281: f64 = (assign16430_e16279 * locals.var_inv_dinf);
        let assign16430_e16283: f64 = (assign16430_e16281 - 2.0);
        let assign16430_e16285: f64 = (assign16430_e16283 - locals.var_deltaxinf);
        let assign16430_e16287: f64 = (assign16430_e16285 * locals.var_deltaxi);
        (assign16430_e16287, (((((((2.0 * locals.var_ds_dn4) - locals.var_qis_dn4) * locals.var_inv_dinf) + (assign16430_e16279 * locals.var_inv_dinf_dn4)) - locals.var_deltaxinf_dn4) * locals.var_deltaxi) + (assign16430_e16285 * locals.var_deltaxi_dn4)), (((((((2.0 * locals.var_ds_dn6) - locals.var_qis_dn6) * locals.var_inv_dinf) + (assign16430_e16279 * locals.var_inv_dinf_dn6)) - locals.var_deltaxinf_dn6) * locals.var_deltaxi) + (assign16430_e16285 * locals.var_deltaxi_dn6)), (((((((2.0 * locals.var_ds_dn7) - locals.var_qis_dn7) * locals.var_inv_dinf) + (assign16430_e16279 * locals.var_inv_dinf_dn7)) - locals.var_deltaxinf_dn7) * locals.var_deltaxi) + (assign16430_e16285 * locals.var_deltaxi_dn7)), (((((((2.0 * locals.var_ds_dn8) - locals.var_qis_dn8) * locals.var_inv_dinf) + (assign16430_e16279 * locals.var_inv_dinf_dn8)) - locals.var_deltaxinf_dn8) * locals.var_deltaxi) + (assign16430_e16285 * locals.var_deltaxi_dn8)), (((((((2.0 * locals.var_ds_dn9) - locals.var_qis_dn9) * locals.var_inv_dinf) + (assign16430_e16279 * locals.var_inv_dinf_dn9)) - locals.var_deltaxinf_dn9) * locals.var_deltaxi) + (assign16430_e16285 * locals.var_deltaxi_dn9)),)
    } else {
        (locals.var_wd, locals.var_wd_dn4, locals.var_wd_dn6, locals.var_wd_dn7, locals.var_wd_dn8, locals.var_wd_dn9,)
    }
};
        locals.var_wd = assign16430_e16289;
        locals.var_wd_dn4 = assign16430_e16289_d_n4;
        locals.var_wd_dn6 = assign16430_e16289_d_n6;
        locals.var_wd_dn7 = assign16430_e16289_d_n7;
        locals.var_wd_dn8 = assign16430_e16289_d_n8;
        locals.var_wd_dn9 = assign16430_e16289_d_n9;

        let assign16440_e16292: f64 = if locals.var_gamma > 1e-14 { 1.0 } else { 0.0 };
        locals.var_guard611 = assign16440_e16292;

    }

    pub(super) fn stamp_transient_block_41(
        locals: &mut StampLocals,
    ) {
        let (assign16450_e16302, assign16450_e16302_d_n4, assign16450_e16302_d_n6, assign16450_e16302_d_n7, assign16450_e16302_d_n8, assign16450_e16302_d_n9,) = {
    if ((locals.var_guard608 != 0.0) && (locals.var_guard611 != 0.0)) {
        let assign16450_e16299: f64 = (locals.var_gamma * locals.var_gamma);
        let assign16450_e16300: f64 = (2.0 / assign16450_e16299);
        (assign16450_e16300, (-((2.0 * ((locals.var_gamma_dn4 * locals.var_gamma) + (locals.var_gamma * locals.var_gamma_dn4))) / (assign16450_e16299 * assign16450_e16299))), (-((2.0 * ((locals.var_gamma_dn6 * locals.var_gamma) + (locals.var_gamma * locals.var_gamma_dn6))) / (assign16450_e16299 * assign16450_e16299))), (-((2.0 * ((locals.var_gamma_dn7 * locals.var_gamma) + (locals.var_gamma * locals.var_gamma_dn7))) / (assign16450_e16299 * assign16450_e16299))), (-((2.0 * ((locals.var_gamma_dn8 * locals.var_gamma) + (locals.var_gamma * locals.var_gamma_dn8))) / (assign16450_e16299 * assign16450_e16299))), (-((2.0 * ((locals.var_gamma_dn9 * locals.var_gamma) + (locals.var_gamma * locals.var_gamma_dn9))) / (assign16450_e16299 * assign16450_e16299))),)
    } else {
        (locals.var_ps_cub, locals.var_ps_cub_dn4, locals.var_ps_cub_dn6, locals.var_ps_cub_dn7, locals.var_ps_cub_dn8, locals.var_ps_cub_dn9,)
    }
};
        locals.var_ps_cub = assign16450_e16302;
        locals.var_ps_cub_dn4 = assign16450_e16302_d_n4;
        locals.var_ps_cub_dn6 = assign16450_e16302_d_n6;
        locals.var_ps_cub_dn7 = assign16450_e16302_d_n7;
        locals.var_ps_cub_dn8 = assign16450_e16302_d_n8;
        locals.var_ps_cub_dn9 = assign16450_e16302_d_n9;

        let (assign16460_e16310, assign16460_e16310_d_n4, assign16460_e16310_d_n6, assign16460_e16310_d_n7, assign16460_e16310_d_n8, assign16460_e16310_d_n9,) = {
    if ((locals.var_guard608 != 0.0) && (locals.var_guard611 != 0.0)) {
        let assign16460_e16308: f64 = (locals.var_ps_cub * locals.var_vs);
        (assign16460_e16308, ((locals.var_ps_cub_dn4 * locals.var_vs) + (locals.var_ps_cub * locals.var_vs_dn4)), ((locals.var_ps_cub_dn6 * locals.var_vs) + (locals.var_ps_cub * locals.var_vs_dn6)), ((locals.var_ps_cub_dn7 * locals.var_vs) + (locals.var_ps_cub * locals.var_vs_dn7)), ((locals.var_ps_cub_dn8 * locals.var_vs) + (locals.var_ps_cub * locals.var_vs_dn8)), ((locals.var_ps_cub_dn9 * locals.var_vs) + (locals.var_ps_cub * locals.var_vs_dn9)),)
    } else {
        (locals.var_qs_cub, locals.var_qs_cub_dn4, locals.var_qs_cub_dn6, locals.var_qs_cub_dn7, locals.var_qs_cub_dn8, locals.var_qs_cub_dn9,)
    }
};
        locals.var_qs_cub = assign16460_e16310;
        locals.var_qs_cub_dn4 = assign16460_e16310_d_n4;
        locals.var_qs_cub_dn6 = assign16460_e16310_d_n6;
        locals.var_qs_cub_dn7 = assign16460_e16310_d_n7;
        locals.var_qs_cub_dn8 = assign16460_e16310_d_n8;
        locals.var_qs_cub_dn9 = assign16460_e16310_d_n9;

        let (assign16470_e16318, assign16470_e16318_d_n4, assign16470_e16318_d_n6, assign16470_e16318_d_n7, assign16470_e16318_d_n8, assign16470_e16318_d_n9,) = {
    if ((locals.var_guard608 != 0.0) && (locals.var_guard611 != 0.0)) {
        let assign16470_e16316: f64 = (locals.var_ps_cub + locals.var_wd);
        (assign16470_e16316, (locals.var_ps_cub_dn4 + locals.var_wd_dn4), (locals.var_ps_cub_dn6 + locals.var_wd_dn6), (locals.var_ps_cub_dn7 + locals.var_wd_dn7), (locals.var_ps_cub_dn8 + locals.var_wd_dn8), (locals.var_ps_cub_dn9 + locals.var_wd_dn9),)
    } else {
        (locals.var_pd_cub, locals.var_pd_cub_dn4, locals.var_pd_cub_dn6, locals.var_pd_cub_dn7, locals.var_pd_cub_dn8, locals.var_pd_cub_dn9,)
    }
};
        locals.var_pd_cub = assign16470_e16318;
        locals.var_pd_cub_dn4 = assign16470_e16318_d_n4;
        locals.var_pd_cub_dn6 = assign16470_e16318_d_n6;
        locals.var_pd_cub_dn7 = assign16470_e16318_d_n7;
        locals.var_pd_cub_dn8 = assign16470_e16318_d_n8;
        locals.var_pd_cub_dn9 = assign16470_e16318_d_n9;

        let (assign16480_e16326, assign16480_e16326_d_n4, assign16480_e16326_d_n6, assign16480_e16326_d_n7, assign16480_e16326_d_n8, assign16480_e16326_d_n9,) = {
    if ((locals.var_guard608 != 0.0) && (locals.var_guard611 != 0.0)) {
        let assign16480_e16324: f64 = (locals.var_ps_cub * locals.var_vd);
        (assign16480_e16324, ((locals.var_ps_cub_dn4 * locals.var_vd) + (locals.var_ps_cub * locals.var_vd_dn4)), ((locals.var_ps_cub_dn6 * locals.var_vd) + (locals.var_ps_cub * locals.var_vd_dn6)), ((locals.var_ps_cub_dn7 * locals.var_vd) + (locals.var_ps_cub * locals.var_vd_dn7)), ((locals.var_ps_cub_dn8 * locals.var_vd) + (locals.var_ps_cub * locals.var_vd_dn8)), ((locals.var_ps_cub_dn9 * locals.var_vd) + (locals.var_ps_cub * locals.var_vd_dn9)),)
    } else {
        (locals.var_qd_cub, locals.var_qd_cub_dn4, locals.var_qd_cub_dn6, locals.var_qd_cub_dn7, locals.var_qd_cub_dn8, locals.var_qd_cub_dn9,)
    }
};
        locals.var_qd_cub = assign16480_e16326;
        locals.var_qd_cub_dn4 = assign16480_e16326_d_n4;
        locals.var_qd_cub_dn6 = assign16480_e16326_d_n6;
        locals.var_qd_cub_dn7 = assign16480_e16326_d_n7;
        locals.var_qd_cub_dn8 = assign16480_e16326_d_n8;
        locals.var_qd_cub_dn9 = assign16480_e16326_d_n9;

        let (assign16490_e16345, assign16490_e16345_d_n4, assign16490_e16345_d_n6, assign16490_e16345_d_n7, assign16490_e16345_d_n8, assign16490_e16345_d_n9,) = {
    if ((locals.var_guard608 != 0.0) && (locals.var_guard611 != 0.0)) {
        let assign16490_e16332: f64 = (locals.var_qs_cub * locals.var_qs_cub);
        let assign16490_e16335: f64 = (0.148148148148 * locals.var_ps_cub);
        let assign16490_e16337: f64 = (assign16490_e16335 * locals.var_ps_cub);
        let assign16490_e16339: f64 = (assign16490_e16337 * locals.var_ps_cub);
        let assign16490_e16340: f64 = (assign16490_e16332 + assign16490_e16339);
        let assign16490_e16342: f64 = (assign16490_e16340 + 1e-20);
        let assign16490_e16343: f64 = (assign16490_e16342).sqrt();
        (assign16490_e16343, ((((locals.var_qs_cub_dn4 * locals.var_qs_cub) + (locals.var_qs_cub * locals.var_qs_cub_dn4)) + (((((0.148148148148 * locals.var_ps_cub_dn4) * locals.var_ps_cub) + (assign16490_e16335 * locals.var_ps_cub_dn4)) * locals.var_ps_cub) + (assign16490_e16337 * locals.var_ps_cub_dn4))) / (2.0 * assign16490_e16343)), ((((locals.var_qs_cub_dn6 * locals.var_qs_cub) + (locals.var_qs_cub * locals.var_qs_cub_dn6)) + (((((0.148148148148 * locals.var_ps_cub_dn6) * locals.var_ps_cub) + (assign16490_e16335 * locals.var_ps_cub_dn6)) * locals.var_ps_cub) + (assign16490_e16337 * locals.var_ps_cub_dn6))) / (2.0 * assign16490_e16343)), ((((locals.var_qs_cub_dn7 * locals.var_qs_cub) + (locals.var_qs_cub * locals.var_qs_cub_dn7)) + (((((0.148148148148 * locals.var_ps_cub_dn7) * locals.var_ps_cub) + (assign16490_e16335 * locals.var_ps_cub_dn7)) * locals.var_ps_cub) + (assign16490_e16337 * locals.var_ps_cub_dn7))) / (2.0 * assign16490_e16343)), ((((locals.var_qs_cub_dn8 * locals.var_qs_cub) + (locals.var_qs_cub * locals.var_qs_cub_dn8)) + (((((0.148148148148 * locals.var_ps_cub_dn8) * locals.var_ps_cub) + (assign16490_e16335 * locals.var_ps_cub_dn8)) * locals.var_ps_cub) + (assign16490_e16337 * locals.var_ps_cub_dn8))) / (2.0 * assign16490_e16343)), ((((locals.var_qs_cub_dn9 * locals.var_qs_cub) + (locals.var_qs_cub * locals.var_qs_cub_dn9)) + (((((0.148148148148 * locals.var_ps_cub_dn9) * locals.var_ps_cub) + (assign16490_e16335 * locals.var_ps_cub_dn9)) * locals.var_ps_cub) + (assign16490_e16337 * locals.var_ps_cub_dn9))) / (2.0 * assign16490_e16343)),)
    } else {
        (locals.var_racs, locals.var_racs_dn4, locals.var_racs_dn6, locals.var_racs_dn7, locals.var_racs_dn8, locals.var_racs_dn9,)
    }
};
        locals.var_racs = assign16490_e16345;
        locals.var_racs_dn4 = assign16490_e16345_d_n4;
        locals.var_racs_dn6 = assign16490_e16345_d_n6;
        locals.var_racs_dn7 = assign16490_e16345_d_n7;
        locals.var_racs_dn8 = assign16490_e16345_d_n8;
        locals.var_racs_dn9 = assign16490_e16345_d_n9;

        let (assign16500_e16364, assign16500_e16364_d_n4, assign16500_e16364_d_n6, assign16500_e16364_d_n7, assign16500_e16364_d_n8, assign16500_e16364_d_n9,) = {
    if ((locals.var_guard608 != 0.0) && (locals.var_guard611 != 0.0)) {
        let assign16500_e16351: f64 = (locals.var_qd_cub * locals.var_qd_cub);
        let assign16500_e16354: f64 = (0.148148148148 * locals.var_pd_cub);
        let assign16500_e16356: f64 = (assign16500_e16354 * locals.var_pd_cub);
        let assign16500_e16358: f64 = (assign16500_e16356 * locals.var_pd_cub);
        let assign16500_e16359: f64 = (assign16500_e16351 + assign16500_e16358);
        let assign16500_e16361: f64 = (assign16500_e16359 + 1e-20);
        let assign16500_e16362: f64 = (assign16500_e16361).sqrt();
        (assign16500_e16362, ((((locals.var_qd_cub_dn4 * locals.var_qd_cub) + (locals.var_qd_cub * locals.var_qd_cub_dn4)) + (((((0.148148148148 * locals.var_pd_cub_dn4) * locals.var_pd_cub) + (assign16500_e16354 * locals.var_pd_cub_dn4)) * locals.var_pd_cub) + (assign16500_e16356 * locals.var_pd_cub_dn4))) / (2.0 * assign16500_e16362)), ((((locals.var_qd_cub_dn6 * locals.var_qd_cub) + (locals.var_qd_cub * locals.var_qd_cub_dn6)) + (((((0.148148148148 * locals.var_pd_cub_dn6) * locals.var_pd_cub) + (assign16500_e16354 * locals.var_pd_cub_dn6)) * locals.var_pd_cub) + (assign16500_e16356 * locals.var_pd_cub_dn6))) / (2.0 * assign16500_e16362)), ((((locals.var_qd_cub_dn7 * locals.var_qd_cub) + (locals.var_qd_cub * locals.var_qd_cub_dn7)) + (((((0.148148148148 * locals.var_pd_cub_dn7) * locals.var_pd_cub) + (assign16500_e16354 * locals.var_pd_cub_dn7)) * locals.var_pd_cub) + (assign16500_e16356 * locals.var_pd_cub_dn7))) / (2.0 * assign16500_e16362)), ((((locals.var_qd_cub_dn8 * locals.var_qd_cub) + (locals.var_qd_cub * locals.var_qd_cub_dn8)) + (((((0.148148148148 * locals.var_pd_cub_dn8) * locals.var_pd_cub) + (assign16500_e16354 * locals.var_pd_cub_dn8)) * locals.var_pd_cub) + (assign16500_e16356 * locals.var_pd_cub_dn8))) / (2.0 * assign16500_e16362)), ((((locals.var_qd_cub_dn9 * locals.var_qd_cub) + (locals.var_qd_cub * locals.var_qd_cub_dn9)) + (((((0.148148148148 * locals.var_pd_cub_dn9) * locals.var_pd_cub) + (assign16500_e16354 * locals.var_pd_cub_dn9)) * locals.var_pd_cub) + (assign16500_e16356 * locals.var_pd_cub_dn9))) / (2.0 * assign16500_e16362)),)
    } else {
        (locals.var_racd, locals.var_racd_dn4, locals.var_racd_dn6, locals.var_racd_dn7, locals.var_racd_dn8, locals.var_racd_dn9,)
    }
};
        locals.var_racd = assign16500_e16364;
        locals.var_racd_dn4 = assign16500_e16364_d_n4;
        locals.var_racd_dn6 = assign16500_e16364_d_n6;
        locals.var_racd_dn7 = assign16500_e16364_d_n7;
        locals.var_racd_dn8 = assign16500_e16364_d_n8;
        locals.var_racd_dn9 = assign16500_e16364_d_n9;

        let (assign16510_e16388, assign16510_e16388_d_n4, assign16510_e16388_d_n6, assign16510_e16388_d_n7, assign16510_e16388_d_n8, assign16510_e16388_d_n9,) = {
    if ((locals.var_guard608 != 0.0) && (locals.var_guard611 != 0.0)) {
        let assign16510_e16372: f64 = (locals.var_racs + locals.var_qs_cub);
        let assign16510_e16373: f64 = (0.5 * assign16510_e16372);
        let assign16510_e16374: f64 = (assign16510_e16373).ln();
        let assign16510_e16375: f64 = (0.3333333333333 * assign16510_e16374);
        let assign16510_e16376: f64 = (assign16510_e16375).exp();
        let assign16510_e16381: f64 = (locals.var_racs - locals.var_qs_cub);
        let assign16510_e16382: f64 = (0.5 * assign16510_e16381);
        let assign16510_e16383: f64 = (assign16510_e16382).ln();
        let assign16510_e16384: f64 = (0.3333333333333 * assign16510_e16383);
        let assign16510_e16385: f64 = (assign16510_e16384).exp();
        let assign16510_e16386: f64 = (assign16510_e16376 - assign16510_e16385);
        (assign16510_e16386, ((assign16510_e16376 * (0.3333333333333 * ((0.5 * (locals.var_racs_dn4 + locals.var_qs_cub_dn4)) / assign16510_e16373))) - (assign16510_e16385 * (0.3333333333333 * ((0.5 * (locals.var_racs_dn4 - locals.var_qs_cub_dn4)) / assign16510_e16382)))), ((assign16510_e16376 * (0.3333333333333 * ((0.5 * (locals.var_racs_dn6 + locals.var_qs_cub_dn6)) / assign16510_e16373))) - (assign16510_e16385 * (0.3333333333333 * ((0.5 * (locals.var_racs_dn6 - locals.var_qs_cub_dn6)) / assign16510_e16382)))), ((assign16510_e16376 * (0.3333333333333 * ((0.5 * (locals.var_racs_dn7 + locals.var_qs_cub_dn7)) / assign16510_e16373))) - (assign16510_e16385 * (0.3333333333333 * ((0.5 * (locals.var_racs_dn7 - locals.var_qs_cub_dn7)) / assign16510_e16382)))), ((assign16510_e16376 * (0.3333333333333 * ((0.5 * (locals.var_racs_dn8 + locals.var_qs_cub_dn8)) / assign16510_e16373))) - (assign16510_e16385 * (0.3333333333333 * ((0.5 * (locals.var_racs_dn8 - locals.var_qs_cub_dn8)) / assign16510_e16382)))), ((assign16510_e16376 * (0.3333333333333 * ((0.5 * (locals.var_racs_dn9 + locals.var_qs_cub_dn9)) / assign16510_e16373))) - (assign16510_e16385 * (0.3333333333333 * ((0.5 * (locals.var_racs_dn9 - locals.var_qs_cub_dn9)) / assign16510_e16382)))),)
    } else {
        (locals.var_deltaxsats, locals.var_deltaxsats_dn4, locals.var_deltaxsats_dn6, locals.var_deltaxsats_dn7, locals.var_deltaxsats_dn8, locals.var_deltaxsats_dn9,)
    }
};
        locals.var_deltaxsats = assign16510_e16388;
        locals.var_deltaxsats_dn4 = assign16510_e16388_d_n4;
        locals.var_deltaxsats_dn6 = assign16510_e16388_d_n6;
        locals.var_deltaxsats_dn7 = assign16510_e16388_d_n7;
        locals.var_deltaxsats_dn8 = assign16510_e16388_d_n8;
        locals.var_deltaxsats_dn9 = assign16510_e16388_d_n9;

        let (assign16520_e16412, assign16520_e16412_d_n4, assign16520_e16412_d_n6, assign16520_e16412_d_n7, assign16520_e16412_d_n8, assign16520_e16412_d_n9,) = {
    if ((locals.var_guard608 != 0.0) && (locals.var_guard611 != 0.0)) {
        let assign16520_e16396: f64 = (locals.var_racd + locals.var_qd_cub);
        let assign16520_e16397: f64 = (0.5 * assign16520_e16396);
        let assign16520_e16398: f64 = (assign16520_e16397).ln();
        let assign16520_e16399: f64 = (0.3333333333333 * assign16520_e16398);
        let assign16520_e16400: f64 = (assign16520_e16399).exp();
        let assign16520_e16405: f64 = (locals.var_racd - locals.var_qd_cub);
        let assign16520_e16406: f64 = (0.5 * assign16520_e16405);
        let assign16520_e16407: f64 = (assign16520_e16406).ln();
        let assign16520_e16408: f64 = (0.3333333333333 * assign16520_e16407);
        let assign16520_e16409: f64 = (assign16520_e16408).exp();
        let assign16520_e16410: f64 = (assign16520_e16400 - assign16520_e16409);
        (assign16520_e16410, ((assign16520_e16400 * (0.3333333333333 * ((0.5 * (locals.var_racd_dn4 + locals.var_qd_cub_dn4)) / assign16520_e16397))) - (assign16520_e16409 * (0.3333333333333 * ((0.5 * (locals.var_racd_dn4 - locals.var_qd_cub_dn4)) / assign16520_e16406)))), ((assign16520_e16400 * (0.3333333333333 * ((0.5 * (locals.var_racd_dn6 + locals.var_qd_cub_dn6)) / assign16520_e16397))) - (assign16520_e16409 * (0.3333333333333 * ((0.5 * (locals.var_racd_dn6 - locals.var_qd_cub_dn6)) / assign16520_e16406)))), ((assign16520_e16400 * (0.3333333333333 * ((0.5 * (locals.var_racd_dn7 + locals.var_qd_cub_dn7)) / assign16520_e16397))) - (assign16520_e16409 * (0.3333333333333 * ((0.5 * (locals.var_racd_dn7 - locals.var_qd_cub_dn7)) / assign16520_e16406)))), ((assign16520_e16400 * (0.3333333333333 * ((0.5 * (locals.var_racd_dn8 + locals.var_qd_cub_dn8)) / assign16520_e16397))) - (assign16520_e16409 * (0.3333333333333 * ((0.5 * (locals.var_racd_dn8 - locals.var_qd_cub_dn8)) / assign16520_e16406)))), ((assign16520_e16400 * (0.3333333333333 * ((0.5 * (locals.var_racd_dn9 + locals.var_qd_cub_dn9)) / assign16520_e16397))) - (assign16520_e16409 * (0.3333333333333 * ((0.5 * (locals.var_racd_dn9 - locals.var_qd_cub_dn9)) / assign16520_e16406)))),)
    } else {
        (locals.var_deltaxsatd, locals.var_deltaxsatd_dn4, locals.var_deltaxsatd_dn6, locals.var_deltaxsatd_dn7, locals.var_deltaxsatd_dn8, locals.var_deltaxsatd_dn9,)
    }
};
        locals.var_deltaxsatd = assign16520_e16412;
        locals.var_deltaxsatd_dn4 = assign16520_e16412_d_n4;
        locals.var_deltaxsatd_dn6 = assign16520_e16412_d_n6;
        locals.var_deltaxsatd_dn7 = assign16520_e16412_d_n7;
        locals.var_deltaxsatd_dn8 = assign16520_e16412_d_n8;
        locals.var_deltaxsatd_dn9 = assign16520_e16412_d_n9;

        let (assign16530_e16419, assign16530_e16419_d_n4, assign16530_e16419_d_n6, assign16530_e16419_d_n7, assign16530_e16419_d_n8, assign16530_e16419_d_n9,) = {
    if ((locals.var_guard608 != 0.0) && (locals.var_guard611 == 0.0)) {
        (locals.var_vs, locals.var_vs_dn4, locals.var_vs_dn6, locals.var_vs_dn7, locals.var_vs_dn8, locals.var_vs_dn9,)
    } else {
        (locals.var_deltaxsats, locals.var_deltaxsats_dn4, locals.var_deltaxsats_dn6, locals.var_deltaxsats_dn7, locals.var_deltaxsats_dn8, locals.var_deltaxsats_dn9,)
    }
};
        locals.var_deltaxsats = assign16530_e16419;
        locals.var_deltaxsats_dn4 = assign16530_e16419_d_n4;
        locals.var_deltaxsats_dn6 = assign16530_e16419_d_n6;
        locals.var_deltaxsats_dn7 = assign16530_e16419_d_n7;
        locals.var_deltaxsats_dn8 = assign16530_e16419_d_n8;
        locals.var_deltaxsats_dn9 = assign16530_e16419_d_n9;

        let (assign16540_e16426, assign16540_e16426_d_n4, assign16540_e16426_d_n6, assign16540_e16426_d_n7, assign16540_e16426_d_n8, assign16540_e16426_d_n9,) = {
    if ((locals.var_guard608 != 0.0) && (locals.var_guard611 == 0.0)) {
        (locals.var_vd, locals.var_vd_dn4, locals.var_vd_dn6, locals.var_vd_dn7, locals.var_vd_dn8, locals.var_vd_dn9,)
    } else {
        (locals.var_deltaxsatd, locals.var_deltaxsatd_dn4, locals.var_deltaxsatd_dn6, locals.var_deltaxsatd_dn7, locals.var_deltaxsatd_dn8, locals.var_deltaxsatd_dn9,)
    }
};
        locals.var_deltaxsatd = assign16540_e16426;
        locals.var_deltaxsatd_dn4 = assign16540_e16426_d_n4;
        locals.var_deltaxsatd_dn6 = assign16540_e16426_d_n6;
        locals.var_deltaxsatd_dn7 = assign16540_e16426_d_n7;
        locals.var_deltaxsatd_dn8 = assign16540_e16426_d_n8;
        locals.var_deltaxsatd_dn9 = assign16540_e16426_d_n9;

        let (assign16550_e16432, assign16550_e16432_d_n4, assign16550_e16432_d_n6, assign16550_e16432_d_n7, assign16550_e16432_d_n8, assign16550_e16432_d_n9,) = {
    if (locals.var_guard608 != 0.0) {
        let assign16550_e16430: f64 = (locals.var_temp1 * locals.var_temp1);
        (assign16550_e16430, ((locals.var_temp1_dn4 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn4)), ((locals.var_temp1_dn6 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn6)), ((locals.var_temp1_dn7 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn7)), ((locals.var_temp1_dn8 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn8)), ((locals.var_temp1_dn9 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn9)),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign16550_e16432;
        locals.var_temp3_dn4 = assign16550_e16432_d_n4;
        locals.var_temp3_dn6 = assign16550_e16432_d_n6;
        locals.var_temp3_dn7 = assign16550_e16432_d_n7;
        locals.var_temp3_dn8 = assign16550_e16432_d_n8;
        locals.var_temp3_dn9 = assign16550_e16432_d_n9;

        let (assign16560_e16455, assign16560_e16455_d_n4, assign16560_e16455_d_n6, assign16560_e16455_d_n7, assign16560_e16455_d_n8, assign16560_e16455_d_n9,) = {
    if (locals.var_guard608 != 0.0) {
        let assign16560_e16436: f64 = (0.94 * 0.5);
        let assign16560_e16439: f64 = (locals.var_deltaxsats + locals.var_deltaxsatd);
        let assign16560_e16442: f64 = (locals.var_deltaxsats - locals.var_deltaxsatd);
        let assign16560_e16445: f64 = (locals.var_deltaxsats - locals.var_deltaxsatd);
        let assign16560_e16446: f64 = (assign16560_e16442 * assign16560_e16445);
        let assign16560_e16449: f64 = (10.0 * locals.var_temp3);
        let assign16560_e16450: f64 = (assign16560_e16446 + assign16560_e16449);
        let assign16560_e16451: f64 = (assign16560_e16450).sqrt();
        let assign16560_e16452: f64 = (assign16560_e16439 + assign16560_e16451);
        let assign16560_e16453: f64 = (assign16560_e16436 * assign16560_e16452);
        (assign16560_e16453, (assign16560_e16436 * ((locals.var_deltaxsats_dn4 + locals.var_deltaxsatd_dn4) + (((((locals.var_deltaxsats_dn4 - locals.var_deltaxsatd_dn4) * assign16560_e16445) + (assign16560_e16442 * (locals.var_deltaxsats_dn4 - locals.var_deltaxsatd_dn4))) + (10.0 * locals.var_temp3_dn4)) / (2.0 * assign16560_e16451)))), (assign16560_e16436 * ((locals.var_deltaxsats_dn6 + locals.var_deltaxsatd_dn6) + (((((locals.var_deltaxsats_dn6 - locals.var_deltaxsatd_dn6) * assign16560_e16445) + (assign16560_e16442 * (locals.var_deltaxsats_dn6 - locals.var_deltaxsatd_dn6))) + (10.0 * locals.var_temp3_dn6)) / (2.0 * assign16560_e16451)))), (assign16560_e16436 * ((locals.var_deltaxsats_dn7 + locals.var_deltaxsatd_dn7) + (((((locals.var_deltaxsats_dn7 - locals.var_deltaxsatd_dn7) * assign16560_e16445) + (assign16560_e16442 * (locals.var_deltaxsats_dn7 - locals.var_deltaxsatd_dn7))) + (10.0 * locals.var_temp3_dn7)) / (2.0 * assign16560_e16451)))), (assign16560_e16436 * ((locals.var_deltaxsats_dn8 + locals.var_deltaxsatd_dn8) + (((((locals.var_deltaxsats_dn8 - locals.var_deltaxsatd_dn8) * assign16560_e16445) + (assign16560_e16442 * (locals.var_deltaxsats_dn8 - locals.var_deltaxsatd_dn8))) + (10.0 * locals.var_temp3_dn8)) / (2.0 * assign16560_e16451)))), (assign16560_e16436 * ((locals.var_deltaxsats_dn9 + locals.var_deltaxsatd_dn9) + (((((locals.var_deltaxsats_dn9 - locals.var_deltaxsatd_dn9) * assign16560_e16445) + (assign16560_e16442 * (locals.var_deltaxsats_dn9 - locals.var_deltaxsatd_dn9))) + (10.0 * locals.var_temp3_dn9)) / (2.0 * assign16560_e16451)))),)
    } else {
        (locals.var_deltaxsat, locals.var_deltaxsat_dn4, locals.var_deltaxsat_dn6, locals.var_deltaxsat_dn7, locals.var_deltaxsat_dn8, locals.var_deltaxsat_dn9,)
    }
};
        locals.var_deltaxsat = assign16560_e16455;
        locals.var_deltaxsat_dn4 = assign16560_e16455_d_n4;
        locals.var_deltaxsat_dn6 = assign16560_e16455_d_n6;
        locals.var_deltaxsat_dn7 = assign16560_e16455_d_n7;
        locals.var_deltaxsat_dn8 = assign16560_e16455_d_n8;
        locals.var_deltaxsat_dn9 = assign16560_e16455_d_n9;

        let (assign16570_e16463, assign16570_e16463_d_n4, assign16570_e16463_d_n6, assign16570_e16463_d_n7, assign16570_e16463_d_n8, assign16570_e16463_d_n9,) = {
    if (locals.var_guard608 != 0.0) {
        let assign16570_e16460: f64 = (locals.var_ds * locals.var_deltaxsat);
        let assign16570_e16461: f64 = (locals.var_qis + assign16570_e16460);
        (assign16570_e16461, (locals.var_qis_dn4 + ((locals.var_ds_dn4 * locals.var_deltaxsat) + (locals.var_ds * locals.var_deltaxsat_dn4))), (locals.var_qis_dn6 + ((locals.var_ds_dn6 * locals.var_deltaxsat) + (locals.var_ds * locals.var_deltaxsat_dn6))), (locals.var_qis_dn7 + ((locals.var_ds_dn7 * locals.var_deltaxsat) + (locals.var_ds * locals.var_deltaxsat_dn7))), (locals.var_qis_dn8 + ((locals.var_ds_dn8 * locals.var_deltaxsat) + (locals.var_ds * locals.var_deltaxsat_dn8))), (locals.var_qis_dn9 + ((locals.var_ds_dn9 * locals.var_deltaxsat) + (locals.var_ds * locals.var_deltaxsat_dn9))),)
    } else {
        (locals.var_qidsats, locals.var_qidsats_dn4, locals.var_qidsats_dn6, locals.var_qidsats_dn7, locals.var_qidsats_dn8, locals.var_qidsats_dn9,)
    }
};
        locals.var_qidsats = assign16570_e16463;
        locals.var_qidsats_dn4 = assign16570_e16463_d_n4;
        locals.var_qidsats_dn6 = assign16570_e16463_d_n6;
        locals.var_qidsats_dn7 = assign16570_e16463_d_n7;
        locals.var_qidsats_dn8 = assign16570_e16463_d_n8;
        locals.var_qidsats_dn9 = assign16570_e16463_d_n9;

        let (assign16580_e16471, assign16580_e16471_d_n4, assign16580_e16471_d_n6, assign16580_e16471_d_n7, assign16580_e16471_d_n8, assign16580_e16471_d_n9,) = {
    if (locals.var_guard608 != 0.0) {
        let assign16580_e16468: f64 = (locals.var_deltaxsat - locals.var_deltaxinf);
        let assign16580_e16469: f64 = (locals.var_dinf * assign16580_e16468);
        (assign16580_e16469, ((locals.var_dinf_dn4 * assign16580_e16468) + (locals.var_dinf * (locals.var_deltaxsat_dn4 - locals.var_deltaxinf_dn4))), ((locals.var_dinf_dn6 * assign16580_e16468) + (locals.var_dinf * (locals.var_deltaxsat_dn6 - locals.var_deltaxinf_dn6))), ((locals.var_dinf_dn7 * assign16580_e16468) + (locals.var_dinf * (locals.var_deltaxsat_dn7 - locals.var_deltaxinf_dn7))), ((locals.var_dinf_dn8 * assign16580_e16468) + (locals.var_dinf * (locals.var_deltaxsat_dn8 - locals.var_deltaxinf_dn8))), ((locals.var_dinf_dn9 * assign16580_e16468) + (locals.var_dinf * (locals.var_deltaxsat_dn9 - locals.var_deltaxinf_dn9))),)
    } else {
        (locals.var_qidsatd, locals.var_qidsatd_dn4, locals.var_qidsatd_dn6, locals.var_qidsatd_dn7, locals.var_qidsatd_dn8, locals.var_qidsatd_dn9,)
    }
};
        locals.var_qidsatd = assign16580_e16471;
        locals.var_qidsatd_dn4 = assign16580_e16471_d_n4;
        locals.var_qidsatd_dn6 = assign16580_e16471_d_n6;
        locals.var_qidsatd_dn7 = assign16580_e16471_d_n7;
        locals.var_qidsatd_dn8 = assign16580_e16471_d_n8;
        locals.var_qidsatd_dn9 = assign16580_e16471_d_n9;

        let (assign16590_e16492, assign16590_e16492_d_n4, assign16590_e16492_d_n6, assign16590_e16492_d_n7, assign16590_e16492_d_n8, assign16590_e16492_d_n9,) = {
    if (locals.var_guard608 != 0.0) {
        let assign16590_e16476: f64 = (locals.var_qidsats + locals.var_qidsatd);
        let assign16590_e16479: f64 = (locals.var_qidsats - locals.var_qidsatd);
        let assign16590_e16482: f64 = (locals.var_qidsats - locals.var_qidsatd);
        let assign16590_e16483: f64 = (assign16590_e16479 * assign16590_e16482);
        let assign16590_e16486: f64 = (36.0 * locals.var_temp3);
        let assign16590_e16487: f64 = (assign16590_e16483 + assign16590_e16486);
        let assign16590_e16488: f64 = (assign16590_e16487).sqrt();
        let assign16590_e16489: f64 = (assign16590_e16476 + assign16590_e16488);
        let assign16590_e16490: f64 = (0.5 * assign16590_e16489);
        (assign16590_e16490, (0.5 * ((locals.var_qidsats_dn4 + locals.var_qidsatd_dn4) + (((((locals.var_qidsats_dn4 - locals.var_qidsatd_dn4) * assign16590_e16482) + (assign16590_e16479 * (locals.var_qidsats_dn4 - locals.var_qidsatd_dn4))) + (36.0 * locals.var_temp3_dn4)) / (2.0 * assign16590_e16488)))), (0.5 * ((locals.var_qidsats_dn6 + locals.var_qidsatd_dn6) + (((((locals.var_qidsats_dn6 - locals.var_qidsatd_dn6) * assign16590_e16482) + (assign16590_e16479 * (locals.var_qidsats_dn6 - locals.var_qidsatd_dn6))) + (36.0 * locals.var_temp3_dn6)) / (2.0 * assign16590_e16488)))), (0.5 * ((locals.var_qidsats_dn7 + locals.var_qidsatd_dn7) + (((((locals.var_qidsats_dn7 - locals.var_qidsatd_dn7) * assign16590_e16482) + (assign16590_e16479 * (locals.var_qidsats_dn7 - locals.var_qidsatd_dn7))) + (36.0 * locals.var_temp3_dn7)) / (2.0 * assign16590_e16488)))), (0.5 * ((locals.var_qidsats_dn8 + locals.var_qidsatd_dn8) + (((((locals.var_qidsats_dn8 - locals.var_qidsatd_dn8) * assign16590_e16482) + (assign16590_e16479 * (locals.var_qidsats_dn8 - locals.var_qidsatd_dn8))) + (36.0 * locals.var_temp3_dn8)) / (2.0 * assign16590_e16488)))), (0.5 * ((locals.var_qidsats_dn9 + locals.var_qidsatd_dn9) + (((((locals.var_qidsats_dn9 - locals.var_qidsatd_dn9) * assign16590_e16482) + (assign16590_e16479 * (locals.var_qidsats_dn9 - locals.var_qidsatd_dn9))) + (36.0 * locals.var_temp3_dn9)) / (2.0 * assign16590_e16488)))),)
    } else {
        (locals.var_qidsat, locals.var_qidsat_dn4, locals.var_qidsat_dn6, locals.var_qidsat_dn7, locals.var_qidsat_dn8, locals.var_qidsat_dn9,)
    }
};
        locals.var_qidsat = assign16590_e16492;
        locals.var_qidsat_dn4 = assign16590_e16492_d_n4;
        locals.var_qidsat_dn6 = assign16590_e16492_d_n6;
        locals.var_qidsat_dn7 = assign16590_e16492_d_n7;
        locals.var_qidsat_dn8 = assign16590_e16492_d_n8;
        locals.var_qidsat_dn9 = assign16590_e16492_d_n9;

        let (assign16600_e16497, assign16600_e16497_d_n4, assign16600_e16497_d_n6, assign16600_e16497_d_n7, assign16600_e16497_d_n8, assign16600_e16497_d_n9,) = {
    if (locals.var_guard608 == 0.0) {
        (locals.var_dinf, locals.var_dinf_dn4, locals.var_dinf_dn6, locals.var_dinf_dn7, locals.var_dinf_dn8, locals.var_dinf_dn9,)
    } else {
        (locals.var_ds, locals.var_ds_dn4, locals.var_ds_dn6, locals.var_ds_dn7, locals.var_ds_dn8, locals.var_ds_dn9,)
    }
};
        locals.var_ds = assign16600_e16497;
        locals.var_ds_dn4 = assign16600_e16497_d_n4;
        locals.var_ds_dn6 = assign16600_e16497_d_n6;
        locals.var_ds_dn7 = assign16600_e16497_d_n7;
        locals.var_ds_dn8 = assign16600_e16497_d_n8;
        locals.var_ds_dn9 = assign16600_e16497_d_n9;

        let (assign16610_e16506, assign16610_e16506_d_n4, assign16610_e16506_d_n6, assign16610_e16506_d_n7, assign16610_e16506_d_n8, assign16610_e16506_d_n9,) = {
    if (locals.var_guard608 == 0.0) {
        let assign16610_e16503: f64 = (1.0 + locals.var_deltaxinf);
        let assign16610_e16504: f64 = (0.94 * assign16610_e16503);
        (assign16610_e16504, (0.94 * locals.var_deltaxinf_dn4), (0.94 * locals.var_deltaxinf_dn6), (0.94 * locals.var_deltaxinf_dn7), (0.94 * locals.var_deltaxinf_dn8), (0.94 * locals.var_deltaxinf_dn9),)
    } else {
        (locals.var_deltaxsat, locals.var_deltaxsat_dn4, locals.var_deltaxsat_dn6, locals.var_deltaxsat_dn7, locals.var_deltaxsat_dn8, locals.var_deltaxsat_dn9,)
    }
};
        locals.var_deltaxsat = assign16610_e16506;
        locals.var_deltaxsat_dn4 = assign16610_e16506_d_n4;
        locals.var_deltaxsat_dn6 = assign16610_e16506_d_n6;
        locals.var_deltaxsat_dn7 = assign16610_e16506_d_n7;
        locals.var_deltaxsat_dn8 = assign16610_e16506_d_n8;
        locals.var_deltaxsat_dn9 = assign16610_e16506_d_n9;

        let (assign16620_e16521, assign16620_e16521_d_n4, assign16620_e16521_d_n6, assign16620_e16521_d_n7, assign16620_e16521_d_n8, assign16620_e16521_d_n9,) = {
    if (locals.var_guard608 == 0.0) {
        let assign16620_e16511: f64 = (0.5 * locals.var_qis);
        let assign16620_e16516: f64 = (0.5 * locals.var_deltaxinf);
        let assign16620_e16517: f64 = (locals.var_deltaxsat - assign16620_e16516);
        let assign16620_e16518: f64 = (locals.var_dinf * assign16620_e16517);
        let assign16620_e16519: f64 = (assign16620_e16511 + assign16620_e16518);
        (assign16620_e16519, ((0.5 * locals.var_qis_dn4) + ((locals.var_dinf_dn4 * assign16620_e16517) + (locals.var_dinf * (locals.var_deltaxsat_dn4 - (0.5 * locals.var_deltaxinf_dn4))))), ((0.5 * locals.var_qis_dn6) + ((locals.var_dinf_dn6 * assign16620_e16517) + (locals.var_dinf * (locals.var_deltaxsat_dn6 - (0.5 * locals.var_deltaxinf_dn6))))), ((0.5 * locals.var_qis_dn7) + ((locals.var_dinf_dn7 * assign16620_e16517) + (locals.var_dinf * (locals.var_deltaxsat_dn7 - (0.5 * locals.var_deltaxinf_dn7))))), ((0.5 * locals.var_qis_dn8) + ((locals.var_dinf_dn8 * assign16620_e16517) + (locals.var_dinf * (locals.var_deltaxsat_dn8 - (0.5 * locals.var_deltaxinf_dn8))))), ((0.5 * locals.var_qis_dn9) + ((locals.var_dinf_dn9 * assign16620_e16517) + (locals.var_dinf * (locals.var_deltaxsat_dn9 - (0.5 * locals.var_deltaxinf_dn9))))),)
    } else {
        (locals.var_qidsat, locals.var_qidsat_dn4, locals.var_qidsat_dn6, locals.var_qidsat_dn7, locals.var_qidsat_dn8, locals.var_qidsat_dn9,)
    }
};
        locals.var_qidsat = assign16620_e16521;
        locals.var_qidsat_dn4 = assign16620_e16521_d_n4;
        locals.var_qidsat_dn6 = assign16620_e16521_d_n6;
        locals.var_qidsat_dn7 = assign16620_e16521_d_n7;
        locals.var_qidsat_dn8 = assign16620_e16521_d_n8;
        locals.var_qidsat_dn9 = assign16620_e16521_d_n9;

        let assign16630_e16524: f64 = (locals.var_qidsat - 0.5);
        let assign16630_e16526: f64 = if assign16630_e16524 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard612 = assign16630_e16526;

        let (assign16640_e16536, assign16640_e16536_d_n4, assign16640_e16536_d_n6, assign16640_e16536_d_n7, assign16640_e16536_d_n8, assign16640_e16536_d_n9,) = {
    if (locals.var_guard612 != 0.0) {
        let assign16640_e16531: f64 = (locals.var_qidsat - 0.5);
        let assign16640_e16532: f64 = (assign16640_e16531).exp();
        let assign16640_e16533: f64 = (1.0 + assign16640_e16532);
        let assign16640_e16534: f64 = (assign16640_e16533).ln();
        (assign16640_e16534, ((assign16640_e16532 * locals.var_qidsat_dn4) / assign16640_e16533), ((assign16640_e16532 * locals.var_qidsat_dn6) / assign16640_e16533), ((assign16640_e16532 * locals.var_qidsat_dn7) / assign16640_e16533), ((assign16640_e16532 * locals.var_qidsat_dn8) / assign16640_e16533), ((assign16640_e16532 * locals.var_qidsat_dn9) / assign16640_e16533),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign16640_e16536;
        locals.var_temp1_dn4 = assign16640_e16536_d_n4;
        locals.var_temp1_dn6 = assign16640_e16536_d_n6;
        locals.var_temp1_dn7 = assign16640_e16536_d_n7;
        locals.var_temp1_dn8 = assign16640_e16536_d_n8;
        locals.var_temp1_dn9 = assign16640_e16536_d_n9;

        let (assign16650_e16543, assign16650_e16543_d_n4, assign16650_e16543_d_n6, assign16650_e16543_d_n7, assign16650_e16543_d_n8, assign16650_e16543_d_n9,) = {
    if (locals.var_guard612 == 0.0) {
        let assign16650_e16541: f64 = (locals.var_qidsat - 0.5);
        (assign16650_e16541, locals.var_qidsat_dn4, locals.var_qidsat_dn6, locals.var_qidsat_dn7, locals.var_qidsat_dn8, locals.var_qidsat_dn9,)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign16650_e16543;
        locals.var_temp1_dn4 = assign16650_e16543_d_n4;
        locals.var_temp1_dn6 = assign16650_e16543_d_n6;
        locals.var_temp1_dn7 = assign16650_e16543_d_n7;
        locals.var_temp1_dn8 = assign16650_e16543_d_n8;
        locals.var_temp1_dn9 = assign16650_e16543_d_n9;

        let assign16660_e16546: f64 = (locals.var_temp1 + 0.5);
        locals.var_temp2 = assign16660_e16546;
        locals.var_temp2_dn4 = locals.var_temp1_dn4;
        locals.var_temp2_dn6 = locals.var_temp1_dn6;
        locals.var_temp2_dn7 = locals.var_temp1_dn7;
        locals.var_temp2_dn8 = locals.var_temp1_dn8;
        locals.var_temp2_dn9 = locals.var_temp1_dn9;

        let assign16670_e16550: f64 = (locals.var_qis / locals.var_temp2);
        let assign16670_e16551: f64 = (assign16670_e16550).ln();
        let assign16670_e16552: f64 = (locals.var_deltaxsat + assign16670_e16551);
        locals.var_temp3 = assign16670_e16552;
        locals.var_temp3_dn4 = (locals.var_deltaxsat_dn4 + ((((locals.var_qis_dn4 * locals.var_temp2) - (locals.var_qis * locals.var_temp2_dn4)) / (locals.var_temp2 * locals.var_temp2)) / assign16670_e16550));
        locals.var_temp3_dn6 = (locals.var_deltaxsat_dn6 + ((((locals.var_qis_dn6 * locals.var_temp2) - (locals.var_qis * locals.var_temp2_dn6)) / (locals.var_temp2 * locals.var_temp2)) / assign16670_e16550));
        locals.var_temp3_dn7 = (locals.var_deltaxsat_dn7 + ((((locals.var_qis_dn7 * locals.var_temp2) - (locals.var_qis * locals.var_temp2_dn7)) / (locals.var_temp2 * locals.var_temp2)) / assign16670_e16550));
        locals.var_temp3_dn8 = (locals.var_deltaxsat_dn8 + ((((locals.var_qis_dn8 * locals.var_temp2) - (locals.var_qis * locals.var_temp2_dn8)) / (locals.var_temp2 * locals.var_temp2)) / assign16670_e16550));
        locals.var_temp3_dn9 = (locals.var_deltaxsat_dn9 + ((((locals.var_qis_dn9 * locals.var_temp2) - (locals.var_qis * locals.var_temp2_dn9)) / (locals.var_temp2 * locals.var_temp2)) / assign16670_e16550));

        let assign16680_e16555: f64 = (locals.var_temp3 - 6.0);
        let assign16680_e16557: f64 = if assign16680_e16555 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard613 = assign16680_e16557;

        let (assign16690_e16567, assign16690_e16567_d_n4, assign16690_e16567_d_n6, assign16690_e16567_d_n7, assign16690_e16567_d_n8, assign16690_e16567_d_n9,) = {
    if (locals.var_guard613 != 0.0) {
        let assign16690_e16562: f64 = (locals.var_temp3 - 6.0);
        let assign16690_e16563: f64 = (assign16690_e16562).exp();
        let assign16690_e16564: f64 = (1.0 + assign16690_e16563);
        let assign16690_e16565: f64 = (assign16690_e16564).ln();
        (assign16690_e16565, ((assign16690_e16563 * locals.var_temp3_dn4) / assign16690_e16564), ((assign16690_e16563 * locals.var_temp3_dn6) / assign16690_e16564), ((assign16690_e16563 * locals.var_temp3_dn7) / assign16690_e16564), ((assign16690_e16563 * locals.var_temp3_dn8) / assign16690_e16564), ((assign16690_e16563 * locals.var_temp3_dn9) / assign16690_e16564),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign16690_e16567;
        locals.var_temp1_dn4 = assign16690_e16567_d_n4;
        locals.var_temp1_dn6 = assign16690_e16567_d_n6;
        locals.var_temp1_dn7 = assign16690_e16567_d_n7;
        locals.var_temp1_dn8 = assign16690_e16567_d_n8;
        locals.var_temp1_dn9 = assign16690_e16567_d_n9;

        let (assign16700_e16574, assign16700_e16574_d_n4, assign16700_e16574_d_n6, assign16700_e16574_d_n7, assign16700_e16574_d_n8, assign16700_e16574_d_n9,) = {
    if (locals.var_guard613 == 0.0) {
        let assign16700_e16572: f64 = (locals.var_temp3 - 6.0);
        (assign16700_e16572, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign16700_e16574;
        locals.var_temp1_dn4 = assign16700_e16574_d_n4;
        locals.var_temp1_dn6 = assign16700_e16574_d_n6;
        locals.var_temp1_dn7 = assign16700_e16574_d_n7;
        locals.var_temp1_dn8 = assign16700_e16574_d_n8;
        locals.var_temp1_dn9 = assign16700_e16574_d_n9;

        let assign16710_e16577: f64 = (locals.var_temp1 + 6.0);
        locals.var_temp3 = assign16710_e16577;
        locals.var_temp3_dn4 = locals.var_temp1_dn4;
        locals.var_temp3_dn6 = locals.var_temp1_dn6;
        locals.var_temp3_dn7 = locals.var_temp1_dn7;
        locals.var_temp3_dn8 = locals.var_temp1_dn8;
        locals.var_temp3_dn9 = locals.var_temp1_dn9;

        let assign16720_e16580: f64 = (locals.var_xsatmax - locals.var_temp3);
        let assign16720_e16582: f64 = if assign16720_e16580 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard614 = assign16720_e16582;

        let (assign16730_e16592, assign16730_e16592_d_n4, assign16730_e16592_d_n6, assign16730_e16592_d_n7, assign16730_e16592_d_n8, assign16730_e16592_d_n9,) = {
    if (locals.var_guard614 != 0.0) {
        let assign16730_e16587: f64 = (locals.var_xsatmax - locals.var_temp3);
        let assign16730_e16588: f64 = (assign16730_e16587).exp();
        let assign16730_e16589: f64 = (1.0 + assign16730_e16588);
        let assign16730_e16590: f64 = (assign16730_e16589).ln();
        (assign16730_e16590, ((assign16730_e16588 * (locals.var_xsatmax_dn4 - locals.var_temp3_dn4)) / assign16730_e16589), ((assign16730_e16588 * (locals.var_xsatmax_dn6 - locals.var_temp3_dn6)) / assign16730_e16589), ((assign16730_e16588 * (locals.var_xsatmax_dn7 - locals.var_temp3_dn7)) / assign16730_e16589), ((assign16730_e16588 * (locals.var_xsatmax_dn8 - locals.var_temp3_dn8)) / assign16730_e16589), ((assign16730_e16588 * (locals.var_xsatmax_dn9 - locals.var_temp3_dn9)) / assign16730_e16589),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign16730_e16592;
        locals.var_temp1_dn4 = assign16730_e16592_d_n4;
        locals.var_temp1_dn6 = assign16730_e16592_d_n6;
        locals.var_temp1_dn7 = assign16730_e16592_d_n7;
        locals.var_temp1_dn8 = assign16730_e16592_d_n8;
        locals.var_temp1_dn9 = assign16730_e16592_d_n9;

        let (assign16740_e16599, assign16740_e16599_d_n4, assign16740_e16599_d_n6, assign16740_e16599_d_n7, assign16740_e16599_d_n8, assign16740_e16599_d_n9,) = {
    if (locals.var_guard614 == 0.0) {
        let assign16740_e16597: f64 = (locals.var_xsatmax - locals.var_temp3);
        (assign16740_e16597, (locals.var_xsatmax_dn4 - locals.var_temp3_dn4), (locals.var_xsatmax_dn6 - locals.var_temp3_dn6), (locals.var_xsatmax_dn7 - locals.var_temp3_dn7), (locals.var_xsatmax_dn8 - locals.var_temp3_dn8), (locals.var_xsatmax_dn9 - locals.var_temp3_dn9),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign16740_e16599;
        locals.var_temp1_dn4 = assign16740_e16599_d_n4;
        locals.var_temp1_dn6 = assign16740_e16599_d_n6;
        locals.var_temp1_dn7 = assign16740_e16599_d_n7;
        locals.var_temp1_dn8 = assign16740_e16599_d_n8;
        locals.var_temp1_dn9 = assign16740_e16599_d_n9;

        let assign16750_e16602: f64 = (locals.var_xsatmax - locals.var_temp1);
        locals.var_xndssat = assign16750_e16602;
        locals.var_xndssat_dn4 = (locals.var_xsatmax_dn4 - locals.var_temp1_dn4);
        locals.var_xndssat_dn6 = (locals.var_xsatmax_dn6 - locals.var_temp1_dn6);
        locals.var_xndssat_dn7 = (locals.var_xsatmax_dn7 - locals.var_temp1_dn7);
        locals.var_xndssat_dn8 = (locals.var_xsatmax_dn8 - locals.var_temp1_dn8);
        locals.var_xndssat_dn9 = (locals.var_xsatmax_dn9 - locals.var_temp1_dn9);

        let assign16760_e16605: f64 = (locals.var_xd / locals.var_xndssat);
        locals.var_temp1 = assign16760_e16605;
        locals.var_temp1_dn4 = (((locals.var_xd_dn4 * locals.var_xndssat) - (locals.var_xd * locals.var_xndssat_dn4)) / (locals.var_xndssat * locals.var_xndssat));
        locals.var_temp1_dn6 = (((locals.var_xd_dn6 * locals.var_xndssat) - (locals.var_xd * locals.var_xndssat_dn6)) / (locals.var_xndssat * locals.var_xndssat));
        locals.var_temp1_dn7 = (((locals.var_xd_dn7 * locals.var_xndssat) - (locals.var_xd * locals.var_xndssat_dn7)) / (locals.var_xndssat * locals.var_xndssat));
        locals.var_temp1_dn8 = (((locals.var_xd_dn8 * locals.var_xndssat) - (locals.var_xd * locals.var_xndssat_dn8)) / (locals.var_xndssat * locals.var_xndssat));
        locals.var_temp1_dn9 = (((locals.var_xd_dn9 * locals.var_xndssat) - (locals.var_xd * locals.var_xndssat_dn9)) / (locals.var_xndssat * locals.var_xndssat));

        let assign16770_e16608: f64 = (locals.var_temp1 * locals.var_temp1);
        locals.var_temp2 = assign16770_e16608;
        locals.var_temp2_dn4 = ((locals.var_temp1_dn4 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn4));
        locals.var_temp2_dn6 = ((locals.var_temp1_dn6 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn6));
        locals.var_temp2_dn7 = ((locals.var_temp1_dn7 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn7));
        locals.var_temp2_dn8 = ((locals.var_temp1_dn8 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn8));
        locals.var_temp2_dn9 = ((locals.var_temp1_dn9 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn9));

        let assign16780_e16611: f64 = (locals.var_temp2 * locals.var_temp2);
        locals.var_temp3 = assign16780_e16611;
        locals.var_temp3_dn4 = ((locals.var_temp2_dn4 * locals.var_temp2) + (locals.var_temp2 * locals.var_temp2_dn4));
        locals.var_temp3_dn6 = ((locals.var_temp2_dn6 * locals.var_temp2) + (locals.var_temp2 * locals.var_temp2_dn6));
        locals.var_temp3_dn7 = ((locals.var_temp2_dn7 * locals.var_temp2) + (locals.var_temp2 * locals.var_temp2_dn7));
        locals.var_temp3_dn8 = ((locals.var_temp2_dn8 * locals.var_temp2) + (locals.var_temp2 * locals.var_temp2_dn8));
        locals.var_temp3_dn9 = ((locals.var_temp2_dn9 * locals.var_temp2) + (locals.var_temp2 * locals.var_temp2_dn9));

        let assign16790_e16614: f64 = (locals.var_temp3 * locals.var_temp3);
        locals.var_temp4 = assign16790_e16614;
        locals.var_temp4_dn4 = ((locals.var_temp3_dn4 * locals.var_temp3) + (locals.var_temp3 * locals.var_temp3_dn4));
        locals.var_temp4_dn6 = ((locals.var_temp3_dn6 * locals.var_temp3) + (locals.var_temp3 * locals.var_temp3_dn6));
        locals.var_temp4_dn7 = ((locals.var_temp3_dn7 * locals.var_temp3) + (locals.var_temp3 * locals.var_temp3_dn7));
        locals.var_temp4_dn8 = ((locals.var_temp3_dn8 * locals.var_temp3) + (locals.var_temp3 * locals.var_temp3_dn8));
        locals.var_temp4_dn9 = ((locals.var_temp3_dn9 * locals.var_temp3) + (locals.var_temp3 * locals.var_temp3_dn9));

    }

    pub(super) fn stamp_transient_block_42(
        locals: &mut StampLocals,
    ) {
        let assign16800_e16619: f64 = (locals.var_gamax_loc * locals.var_temp3);
        let assign16800_e16620: f64 = (1.0 + assign16800_e16619);
        let assign16800_e16621: f64 = (assign16800_e16620).ln();
        let assign16800_e16622: f64 = (2.666666666667 * assign16800_e16621);
        let assign16800_e16623: f64 = (assign16800_e16622).exp();
        locals.var_temp = assign16800_e16623;
        locals.var_temp_dn4 = (assign16800_e16623 * (2.666666666667 * ((locals.var_gamax_loc * locals.var_temp3_dn4) / assign16800_e16620)));
        locals.var_temp_dn6 = (assign16800_e16623 * (2.666666666667 * ((locals.var_gamax_loc * locals.var_temp3_dn6) / assign16800_e16620)));
        locals.var_temp_dn7 = (assign16800_e16623 * (2.666666666667 * ((locals.var_gamax_loc * locals.var_temp3_dn7) / assign16800_e16620)));
        locals.var_temp_dn8 = (assign16800_e16623 * (2.666666666667 * ((locals.var_gamax_loc * locals.var_temp3_dn8) / assign16800_e16620)));
        locals.var_temp_dn9 = (assign16800_e16623 * (2.666666666667 * ((locals.var_gamax_loc * locals.var_temp3_dn9) / assign16800_e16620)));

        let assign16810_e16626: f64 = (-0.0625);
        let assign16810_e16630: f64 = (locals.var_temp4 * locals.var_temp4);
        let assign16810_e16631: f64 = (locals.var_temp + assign16810_e16630);
        let assign16810_e16632: f64 = (assign16810_e16631).ln();
        let assign16810_e16633: f64 = (assign16810_e16626 * assign16810_e16632);
        let assign16810_e16634: f64 = (assign16810_e16633).exp();
        let assign16810_e16635: f64 = (locals.var_xd * assign16810_e16634);
        locals.var_xdeff = assign16810_e16635;
        locals.var_xdeff_dn4 = ((locals.var_xd_dn4 * assign16810_e16634) + (locals.var_xd * (assign16810_e16634 * (assign16810_e16626 * ((locals.var_temp_dn4 + ((locals.var_temp4_dn4 * locals.var_temp4) + (locals.var_temp4 * locals.var_temp4_dn4))) / assign16810_e16631)))));
        locals.var_xdeff_dn6 = ((locals.var_xd_dn6 * assign16810_e16634) + (locals.var_xd * (assign16810_e16634 * (assign16810_e16626 * ((locals.var_temp_dn6 + ((locals.var_temp4_dn6 * locals.var_temp4) + (locals.var_temp4 * locals.var_temp4_dn6))) / assign16810_e16631)))));
        locals.var_xdeff_dn7 = ((locals.var_xd_dn7 * assign16810_e16634) + (locals.var_xd * (assign16810_e16634 * (assign16810_e16626 * ((locals.var_temp_dn7 + ((locals.var_temp4_dn7 * locals.var_temp4) + (locals.var_temp4 * locals.var_temp4_dn7))) / assign16810_e16631)))));
        locals.var_xdeff_dn8 = ((locals.var_xd_dn8 * assign16810_e16634) + (locals.var_xd * (assign16810_e16634 * (assign16810_e16626 * ((locals.var_temp_dn8 + ((locals.var_temp4_dn8 * locals.var_temp4) + (locals.var_temp4 * locals.var_temp4_dn8))) / assign16810_e16631)))));
        locals.var_xdeff_dn9 = ((locals.var_xd_dn9 * assign16810_e16634) + (locals.var_xd * (assign16810_e16634 * (assign16810_e16626 * ((locals.var_temp_dn9 + ((locals.var_temp4_dn9 * locals.var_temp4) + (locals.var_temp4 * locals.var_temp4_dn9))) / assign16810_e16631)))));

        let assign16820_e16639: f64 = (locals.var_k1 + 1.0);
        let assign16820_e16640: f64 = (1.0 / assign16820_e16639);
        locals.var_q_temp1 = assign16820_e16640;
        locals.var_q_temp1_dn4 = (-(locals.var_k1_dn4 / (assign16820_e16639 * assign16820_e16639)));
        locals.var_q_temp1_dn6 = (-(locals.var_k1_dn6 / (assign16820_e16639 * assign16820_e16639)));
        locals.var_q_temp1_dn7 = (-(locals.var_k1_dn7 / (assign16820_e16639 * assign16820_e16639)));
        locals.var_q_temp1_dn8 = (-(locals.var_k1_dn8 / (assign16820_e16639 * assign16820_e16639)));
        locals.var_q_temp1_dn9 = (-(locals.var_k1_dn9 / (assign16820_e16639 * assign16820_e16639)));

        let assign16830_e16644: f64 = (locals.var_k2 + 1.0);
        let assign16830_e16645: f64 = (1.0 / assign16830_e16644);
        locals.var_q_temp2 = assign16830_e16645;
        locals.var_q_temp2_dn4 = (-(locals.var_k2_dn4 / (assign16830_e16644 * assign16830_e16644)));
        locals.var_q_temp2_dn6 = (-(locals.var_k2_dn6 / (assign16830_e16644 * assign16830_e16644)));
        locals.var_q_temp2_dn7 = (-(locals.var_k2_dn7 / (assign16830_e16644 * assign16830_e16644)));
        locals.var_q_temp2_dn8 = (-(locals.var_k2_dn8 / (assign16830_e16644 * assign16830_e16644)));
        locals.var_q_temp2_dn9 = (-(locals.var_k2_dn9 / (assign16830_e16644 * assign16830_e16644)));

        let assign16840_e16649: f64 = (locals.var_k2 * locals.var_q_temp2);
        let assign16840_e16650: f64 = (locals.var_k1 + assign16840_e16649);
        let assign16840_e16652: f64 = (assign16840_e16650 * locals.var_diff_min);
        let assign16840_e16654: f64 = (assign16840_e16652 / locals.var_a0);
        let assign16840_e16655: f64 = (assign16840_e16654).ln();
        let assign16840_e16657: f64 = (assign16840_e16655 + locals.var_xdeff);
        let assign16840_e16659: f64 = (assign16840_e16657 + 3.0);
        locals.var_q_x1sat = assign16840_e16659;
        locals.var_q_x1sat_dn4 = ((((((((locals.var_k1_dn4 + ((locals.var_k2_dn4 * locals.var_q_temp2) + (locals.var_k2 * locals.var_q_temp2_dn4))) * locals.var_diff_min) + (assign16840_e16650 * locals.var_diff_min_dn4)) * locals.var_a0) - (assign16840_e16652 * locals.var_a0_dn4)) / (locals.var_a0 * locals.var_a0)) / assign16840_e16654) + locals.var_xdeff_dn4);
        locals.var_q_x1sat_dn6 = ((((((((locals.var_k1_dn6 + ((locals.var_k2_dn6 * locals.var_q_temp2) + (locals.var_k2 * locals.var_q_temp2_dn6))) * locals.var_diff_min) + (assign16840_e16650 * locals.var_diff_min_dn6)) * locals.var_a0) - (assign16840_e16652 * locals.var_a0_dn6)) / (locals.var_a0 * locals.var_a0)) / assign16840_e16654) + locals.var_xdeff_dn6);
        locals.var_q_x1sat_dn7 = ((((((((locals.var_k1_dn7 + ((locals.var_k2_dn7 * locals.var_q_temp2) + (locals.var_k2 * locals.var_q_temp2_dn7))) * locals.var_diff_min) + (assign16840_e16650 * locals.var_diff_min_dn7)) * locals.var_a0) - (assign16840_e16652 * locals.var_a0_dn7)) / (locals.var_a0 * locals.var_a0)) / assign16840_e16654) + locals.var_xdeff_dn7);
        locals.var_q_x1sat_dn8 = ((((((((locals.var_k1_dn8 + ((locals.var_k2_dn8 * locals.var_q_temp2) + (locals.var_k2 * locals.var_q_temp2_dn8))) * locals.var_diff_min) + (assign16840_e16650 * locals.var_diff_min_dn8)) * locals.var_a0) - (assign16840_e16652 * locals.var_a0_dn8)) / (locals.var_a0 * locals.var_a0)) / assign16840_e16654) + locals.var_xdeff_dn8);
        locals.var_q_x1sat_dn9 = ((((((((locals.var_k1_dn9 + ((locals.var_k2_dn9 * locals.var_q_temp2) + (locals.var_k2 * locals.var_q_temp2_dn9))) * locals.var_diff_min) + (assign16840_e16650 * locals.var_diff_min_dn9)) * locals.var_a0) - (assign16840_e16652 * locals.var_a0_dn9)) / (locals.var_a0 * locals.var_a0)) / assign16840_e16654) + locals.var_xdeff_dn9);

        let assign16850_e16663: f64 = (locals.var_k1 * locals.var_q_temp1);
        let assign16850_e16664: f64 = (locals.var_k2 + assign16850_e16663);
        let assign16850_e16666: f64 = (assign16850_e16664 * locals.var_diff_min);
        let assign16850_e16668: f64 = (assign16850_e16666 / locals.var_a0);
        let assign16850_e16669: f64 = (assign16850_e16668).ln();
        let assign16850_e16671: f64 = (assign16850_e16669 + locals.var_xdeff);
        let assign16850_e16673: f64 = (assign16850_e16671 + 3.0);
        locals.var_q_x2sat = assign16850_e16673;
        locals.var_q_x2sat_dn4 = ((((((((locals.var_k2_dn4 + ((locals.var_k1_dn4 * locals.var_q_temp1) + (locals.var_k1 * locals.var_q_temp1_dn4))) * locals.var_diff_min) + (assign16850_e16664 * locals.var_diff_min_dn4)) * locals.var_a0) - (assign16850_e16666 * locals.var_a0_dn4)) / (locals.var_a0 * locals.var_a0)) / assign16850_e16668) + locals.var_xdeff_dn4);
        locals.var_q_x2sat_dn6 = ((((((((locals.var_k2_dn6 + ((locals.var_k1_dn6 * locals.var_q_temp1) + (locals.var_k1 * locals.var_q_temp1_dn6))) * locals.var_diff_min) + (assign16850_e16664 * locals.var_diff_min_dn6)) * locals.var_a0) - (assign16850_e16666 * locals.var_a0_dn6)) / (locals.var_a0 * locals.var_a0)) / assign16850_e16668) + locals.var_xdeff_dn6);
        locals.var_q_x2sat_dn7 = ((((((((locals.var_k2_dn7 + ((locals.var_k1_dn7 * locals.var_q_temp1) + (locals.var_k1 * locals.var_q_temp1_dn7))) * locals.var_diff_min) + (assign16850_e16664 * locals.var_diff_min_dn7)) * locals.var_a0) - (assign16850_e16666 * locals.var_a0_dn7)) / (locals.var_a0 * locals.var_a0)) / assign16850_e16668) + locals.var_xdeff_dn7);
        locals.var_q_x2sat_dn8 = ((((((((locals.var_k2_dn8 + ((locals.var_k1_dn8 * locals.var_q_temp1) + (locals.var_k1 * locals.var_q_temp1_dn8))) * locals.var_diff_min) + (assign16850_e16664 * locals.var_diff_min_dn8)) * locals.var_a0) - (assign16850_e16666 * locals.var_a0_dn8)) / (locals.var_a0 * locals.var_a0)) / assign16850_e16668) + locals.var_xdeff_dn8);
        locals.var_q_x2sat_dn9 = ((((((((locals.var_k2_dn9 + ((locals.var_k1_dn9 * locals.var_q_temp1) + (locals.var_k1 * locals.var_q_temp1_dn9))) * locals.var_diff_min) + (assign16850_e16664 * locals.var_diff_min_dn9)) * locals.var_a0) - (assign16850_e16666 * locals.var_a0_dn9)) / (locals.var_a0 * locals.var_a0)) / assign16850_e16668) + locals.var_xdeff_dn9);

        let assign16860_e16676: f64 = (locals.var_q_x1sat - locals.var_x1_wi0);
        let assign16860_e16678: f64 = (assign16860_e16676 * 0.3333333333333);
        let assign16860_e16680: f64 = if assign16860_e16678 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard615 = assign16860_e16680;

        let (assign16870_e16692, assign16870_e16692_d_n4, assign16870_e16692_d_n6, assign16870_e16692_d_n7, assign16870_e16692_d_n8, assign16870_e16692_d_n9,) = {
    if (locals.var_guard615 != 0.0) {
        let assign16870_e16685: f64 = (locals.var_q_x1sat - locals.var_x1_wi0);
        let assign16870_e16687: f64 = (assign16870_e16685 * 0.3333333333333);
        let assign16870_e16688: f64 = (assign16870_e16687).exp();
        let assign16870_e16689: f64 = (1.0 + assign16870_e16688);
        let assign16870_e16690: f64 = (assign16870_e16689).ln();
        (assign16870_e16690, ((assign16870_e16688 * ((locals.var_q_x1sat_dn4 - locals.var_x1_wi0_dn4) * 0.3333333333333)) / assign16870_e16689), ((assign16870_e16688 * ((locals.var_q_x1sat_dn6 - locals.var_x1_wi0_dn6) * 0.3333333333333)) / assign16870_e16689), ((assign16870_e16688 * ((locals.var_q_x1sat_dn7 - locals.var_x1_wi0_dn7) * 0.3333333333333)) / assign16870_e16689), ((assign16870_e16688 * ((locals.var_q_x1sat_dn8 - locals.var_x1_wi0_dn8) * 0.3333333333333)) / assign16870_e16689), ((assign16870_e16688 * ((locals.var_q_x1sat_dn9 - locals.var_x1_wi0_dn9) * 0.3333333333333)) / assign16870_e16689),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign16870_e16692;
        locals.var_q_temp3_dn4 = assign16870_e16692_d_n4;
        locals.var_q_temp3_dn6 = assign16870_e16692_d_n6;
        locals.var_q_temp3_dn7 = assign16870_e16692_d_n7;
        locals.var_q_temp3_dn8 = assign16870_e16692_d_n8;
        locals.var_q_temp3_dn9 = assign16870_e16692_d_n9;

        let (assign16880_e16701, assign16880_e16701_d_n4, assign16880_e16701_d_n6, assign16880_e16701_d_n7, assign16880_e16701_d_n8, assign16880_e16701_d_n9,) = {
    if (locals.var_guard615 == 0.0) {
        let assign16880_e16697: f64 = (locals.var_q_x1sat - locals.var_x1_wi0);
        let assign16880_e16699: f64 = (assign16880_e16697 * 0.3333333333333);
        (assign16880_e16699, ((locals.var_q_x1sat_dn4 - locals.var_x1_wi0_dn4) * 0.3333333333333), ((locals.var_q_x1sat_dn6 - locals.var_x1_wi0_dn6) * 0.3333333333333), ((locals.var_q_x1sat_dn7 - locals.var_x1_wi0_dn7) * 0.3333333333333), ((locals.var_q_x1sat_dn8 - locals.var_x1_wi0_dn8) * 0.3333333333333), ((locals.var_q_x1sat_dn9 - locals.var_x1_wi0_dn9) * 0.3333333333333),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign16880_e16701;
        locals.var_q_temp3_dn4 = assign16880_e16701_d_n4;
        locals.var_q_temp3_dn6 = assign16880_e16701_d_n6;
        locals.var_q_temp3_dn7 = assign16880_e16701_d_n7;
        locals.var_q_temp3_dn8 = assign16880_e16701_d_n8;
        locals.var_q_temp3_dn9 = assign16880_e16701_d_n9;

        let assign16890_e16705: f64 = (3.0 * locals.var_q_temp3);
        let assign16890_e16706: f64 = (locals.var_q_x1sat - assign16890_e16705);
        locals.var_q_x1 = assign16890_e16706;
        locals.var_q_x1_dn4 = (locals.var_q_x1sat_dn4 - (3.0 * locals.var_q_temp3_dn4));
        locals.var_q_x1_dn6 = (locals.var_q_x1sat_dn6 - (3.0 * locals.var_q_temp3_dn6));
        locals.var_q_x1_dn7 = (locals.var_q_x1sat_dn7 - (3.0 * locals.var_q_temp3_dn7));
        locals.var_q_x1_dn8 = (locals.var_q_x1sat_dn8 - (3.0 * locals.var_q_temp3_dn8));
        locals.var_q_x1_dn9 = (locals.var_q_x1sat_dn9 - (3.0 * locals.var_q_temp3_dn9));

        let assign16900_e16709: f64 = (locals.var_q_x2sat - locals.var_x2_wi0);
        let assign16900_e16711: f64 = (assign16900_e16709 * 0.3333333333333);
        let assign16900_e16713: f64 = if assign16900_e16711 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard616 = assign16900_e16713;

        let (assign16910_e16725, assign16910_e16725_d_n4, assign16910_e16725_d_n6, assign16910_e16725_d_n7, assign16910_e16725_d_n8, assign16910_e16725_d_n9,) = {
    if (locals.var_guard616 != 0.0) {
        let assign16910_e16718: f64 = (locals.var_q_x2sat - locals.var_x2_wi0);
        let assign16910_e16720: f64 = (assign16910_e16718 * 0.3333333333333);
        let assign16910_e16721: f64 = (assign16910_e16720).exp();
        let assign16910_e16722: f64 = (1.0 + assign16910_e16721);
        let assign16910_e16723: f64 = (assign16910_e16722).ln();
        (assign16910_e16723, ((assign16910_e16721 * ((locals.var_q_x2sat_dn4 - locals.var_x2_wi0_dn4) * 0.3333333333333)) / assign16910_e16722), ((assign16910_e16721 * ((locals.var_q_x2sat_dn6 - locals.var_x2_wi0_dn6) * 0.3333333333333)) / assign16910_e16722), ((assign16910_e16721 * ((locals.var_q_x2sat_dn7 - locals.var_x2_wi0_dn7) * 0.3333333333333)) / assign16910_e16722), ((assign16910_e16721 * ((locals.var_q_x2sat_dn8 - locals.var_x2_wi0_dn8) * 0.3333333333333)) / assign16910_e16722), ((assign16910_e16721 * ((locals.var_q_x2sat_dn9 - locals.var_x2_wi0_dn9) * 0.3333333333333)) / assign16910_e16722),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign16910_e16725;
        locals.var_q_temp3_dn4 = assign16910_e16725_d_n4;
        locals.var_q_temp3_dn6 = assign16910_e16725_d_n6;
        locals.var_q_temp3_dn7 = assign16910_e16725_d_n7;
        locals.var_q_temp3_dn8 = assign16910_e16725_d_n8;
        locals.var_q_temp3_dn9 = assign16910_e16725_d_n9;

        let (assign16920_e16734, assign16920_e16734_d_n4, assign16920_e16734_d_n6, assign16920_e16734_d_n7, assign16920_e16734_d_n8, assign16920_e16734_d_n9,) = {
    if (locals.var_guard616 == 0.0) {
        let assign16920_e16730: f64 = (locals.var_q_x2sat - locals.var_x2_wi0);
        let assign16920_e16732: f64 = (assign16920_e16730 * 0.3333333333333);
        (assign16920_e16732, ((locals.var_q_x2sat_dn4 - locals.var_x2_wi0_dn4) * 0.3333333333333), ((locals.var_q_x2sat_dn6 - locals.var_x2_wi0_dn6) * 0.3333333333333), ((locals.var_q_x2sat_dn7 - locals.var_x2_wi0_dn7) * 0.3333333333333), ((locals.var_q_x2sat_dn8 - locals.var_x2_wi0_dn8) * 0.3333333333333), ((locals.var_q_x2sat_dn9 - locals.var_x2_wi0_dn9) * 0.3333333333333),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign16920_e16734;
        locals.var_q_temp3_dn4 = assign16920_e16734_d_n4;
        locals.var_q_temp3_dn6 = assign16920_e16734_d_n6;
        locals.var_q_temp3_dn7 = assign16920_e16734_d_n7;
        locals.var_q_temp3_dn8 = assign16920_e16734_d_n8;
        locals.var_q_temp3_dn9 = assign16920_e16734_d_n9;

        let assign16930_e16738: f64 = (3.0 * locals.var_q_temp3);
        let assign16930_e16739: f64 = (locals.var_q_x2sat - assign16930_e16738);
        locals.var_q_x2 = assign16930_e16739;
        locals.var_q_x2_dn4 = (locals.var_q_x2sat_dn4 - (3.0 * locals.var_q_temp3_dn4));
        locals.var_q_x2_dn6 = (locals.var_q_x2sat_dn6 - (3.0 * locals.var_q_temp3_dn6));
        locals.var_q_x2_dn7 = (locals.var_q_x2sat_dn7 - (3.0 * locals.var_q_temp3_dn7));
        locals.var_q_x2_dn8 = (locals.var_q_x2sat_dn8 - (3.0 * locals.var_q_temp3_dn8));
        locals.var_q_x2_dn9 = (locals.var_q_x2sat_dn9 - (3.0 * locals.var_q_temp3_dn9));

        let assign16940_e16742: f64 = (locals.var_k1 * locals.var_xg1x);
        let assign16940_e16744: f64 = (assign16940_e16742 + locals.var_q_x2);
        let assign16940_e16746: f64 = (assign16940_e16744 * locals.var_q_temp1);
        locals.var_q_x1_wi = assign16940_e16746;
        locals.var_q_x1_wi_dn4 = (((((locals.var_k1_dn4 * locals.var_xg1x) + (locals.var_k1 * locals.var_xg1x_dn4)) + locals.var_q_x2_dn4) * locals.var_q_temp1) + (assign16940_e16744 * locals.var_q_temp1_dn4));
        locals.var_q_x1_wi_dn6 = (((((locals.var_k1_dn6 * locals.var_xg1x) + (locals.var_k1 * locals.var_xg1x_dn6)) + locals.var_q_x2_dn6) * locals.var_q_temp1) + (assign16940_e16744 * locals.var_q_temp1_dn6));
        locals.var_q_x1_wi_dn7 = (((((locals.var_k1_dn7 * locals.var_xg1x) + (locals.var_k1 * locals.var_xg1x_dn7)) + locals.var_q_x2_dn7) * locals.var_q_temp1) + (assign16940_e16744 * locals.var_q_temp1_dn7));
        locals.var_q_x1_wi_dn8 = (((((locals.var_k1_dn8 * locals.var_xg1x) + (locals.var_k1 * locals.var_xg1x_dn8)) + locals.var_q_x2_dn8) * locals.var_q_temp1) + (assign16940_e16744 * locals.var_q_temp1_dn8));
        locals.var_q_x1_wi_dn9 = (((((locals.var_k1_dn9 * locals.var_xg1x) + (locals.var_k1 * locals.var_xg1x_dn9)) + locals.var_q_x2_dn9) * locals.var_q_temp1) + (assign16940_e16744 * locals.var_q_temp1_dn9));

        let assign16950_e16749: f64 = (locals.var_k2 * locals.var_xg2x);
        let assign16950_e16751: f64 = (assign16950_e16749 + locals.var_q_x1);
        let assign16950_e16753: f64 = (assign16950_e16751 * locals.var_q_temp2);
        locals.var_q_x2_wi = assign16950_e16753;
        locals.var_q_x2_wi_dn4 = (((((locals.var_k2_dn4 * locals.var_xg2x) + (locals.var_k2 * locals.var_xg2x_dn4)) + locals.var_q_x1_dn4) * locals.var_q_temp2) + (assign16950_e16751 * locals.var_q_temp2_dn4));
        locals.var_q_x2_wi_dn6 = (((((locals.var_k2_dn6 * locals.var_xg2x) + (locals.var_k2 * locals.var_xg2x_dn6)) + locals.var_q_x1_dn6) * locals.var_q_temp2) + (assign16950_e16751 * locals.var_q_temp2_dn6));
        locals.var_q_x2_wi_dn7 = (((((locals.var_k2_dn7 * locals.var_xg2x) + (locals.var_k2 * locals.var_xg2x_dn7)) + locals.var_q_x1_dn7) * locals.var_q_temp2) + (assign16950_e16751 * locals.var_q_temp2_dn7));
        locals.var_q_x2_wi_dn8 = (((((locals.var_k2_dn8 * locals.var_xg2x) + (locals.var_k2 * locals.var_xg2x_dn8)) + locals.var_q_x1_dn8) * locals.var_q_temp2) + (assign16950_e16751 * locals.var_q_temp2_dn8));
        locals.var_q_x2_wi_dn9 = (((((locals.var_k2_dn9 * locals.var_xg2x) + (locals.var_k2 * locals.var_xg2x_dn9)) + locals.var_q_x1_dn9) * locals.var_q_temp2) + (assign16950_e16751 * locals.var_q_temp2_dn9));

        let assign16960_e16756: f64 = (locals.var_q_x1sat - locals.var_q_x1_wi);
        let assign16960_e16758: f64 = (assign16960_e16756 * 0.3333333333333);
        let assign16960_e16760: f64 = if assign16960_e16758 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard617 = assign16960_e16760;

        let (assign16970_e16772, assign16970_e16772_d_n4, assign16970_e16772_d_n6, assign16970_e16772_d_n7, assign16970_e16772_d_n8, assign16970_e16772_d_n9,) = {
    if (locals.var_guard617 != 0.0) {
        let assign16970_e16765: f64 = (locals.var_q_x1sat - locals.var_q_x1_wi);
        let assign16970_e16767: f64 = (assign16970_e16765 * 0.3333333333333);
        let assign16970_e16768: f64 = (assign16970_e16767).exp();
        let assign16970_e16769: f64 = (1.0 + assign16970_e16768);
        let assign16970_e16770: f64 = (assign16970_e16769).ln();
        (assign16970_e16770, ((assign16970_e16768 * ((locals.var_q_x1sat_dn4 - locals.var_q_x1_wi_dn4) * 0.3333333333333)) / assign16970_e16769), ((assign16970_e16768 * ((locals.var_q_x1sat_dn6 - locals.var_q_x1_wi_dn6) * 0.3333333333333)) / assign16970_e16769), ((assign16970_e16768 * ((locals.var_q_x1sat_dn7 - locals.var_q_x1_wi_dn7) * 0.3333333333333)) / assign16970_e16769), ((assign16970_e16768 * ((locals.var_q_x1sat_dn8 - locals.var_q_x1_wi_dn8) * 0.3333333333333)) / assign16970_e16769), ((assign16970_e16768 * ((locals.var_q_x1sat_dn9 - locals.var_q_x1_wi_dn9) * 0.3333333333333)) / assign16970_e16769),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign16970_e16772;
        locals.var_q_temp3_dn4 = assign16970_e16772_d_n4;
        locals.var_q_temp3_dn6 = assign16970_e16772_d_n6;
        locals.var_q_temp3_dn7 = assign16970_e16772_d_n7;
        locals.var_q_temp3_dn8 = assign16970_e16772_d_n8;
        locals.var_q_temp3_dn9 = assign16970_e16772_d_n9;

        let (assign16980_e16781, assign16980_e16781_d_n4, assign16980_e16781_d_n6, assign16980_e16781_d_n7, assign16980_e16781_d_n8, assign16980_e16781_d_n9,) = {
    if (locals.var_guard617 == 0.0) {
        let assign16980_e16777: f64 = (locals.var_q_x1sat - locals.var_q_x1_wi);
        let assign16980_e16779: f64 = (assign16980_e16777 * 0.3333333333333);
        (assign16980_e16779, ((locals.var_q_x1sat_dn4 - locals.var_q_x1_wi_dn4) * 0.3333333333333), ((locals.var_q_x1sat_dn6 - locals.var_q_x1_wi_dn6) * 0.3333333333333), ((locals.var_q_x1sat_dn7 - locals.var_q_x1_wi_dn7) * 0.3333333333333), ((locals.var_q_x1sat_dn8 - locals.var_q_x1_wi_dn8) * 0.3333333333333), ((locals.var_q_x1sat_dn9 - locals.var_q_x1_wi_dn9) * 0.3333333333333),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign16980_e16781;
        locals.var_q_temp3_dn4 = assign16980_e16781_d_n4;
        locals.var_q_temp3_dn6 = assign16980_e16781_d_n6;
        locals.var_q_temp3_dn7 = assign16980_e16781_d_n7;
        locals.var_q_temp3_dn8 = assign16980_e16781_d_n8;
        locals.var_q_temp3_dn9 = assign16980_e16781_d_n9;

        let assign16990_e16785: f64 = (3.0 * locals.var_q_temp3);
        let assign16990_e16786: f64 = (locals.var_q_x1sat - assign16990_e16785);
        locals.var_q_x1 = assign16990_e16786;
        locals.var_q_x1_dn4 = (locals.var_q_x1sat_dn4 - (3.0 * locals.var_q_temp3_dn4));
        locals.var_q_x1_dn6 = (locals.var_q_x1sat_dn6 - (3.0 * locals.var_q_temp3_dn6));
        locals.var_q_x1_dn7 = (locals.var_q_x1sat_dn7 - (3.0 * locals.var_q_temp3_dn7));
        locals.var_q_x1_dn8 = (locals.var_q_x1sat_dn8 - (3.0 * locals.var_q_temp3_dn8));
        locals.var_q_x1_dn9 = (locals.var_q_x1sat_dn9 - (3.0 * locals.var_q_temp3_dn9));

        let assign17000_e16789: f64 = (locals.var_q_x2sat - locals.var_q_x2_wi);
        let assign17000_e16791: f64 = (assign17000_e16789 * 0.3333333333333);
        let assign17000_e16793: f64 = if assign17000_e16791 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard618 = assign17000_e16793;

        let (assign17010_e16805, assign17010_e16805_d_n4, assign17010_e16805_d_n6, assign17010_e16805_d_n7, assign17010_e16805_d_n8, assign17010_e16805_d_n9,) = {
    if (locals.var_guard618 != 0.0) {
        let assign17010_e16798: f64 = (locals.var_q_x2sat - locals.var_q_x2_wi);
        let assign17010_e16800: f64 = (assign17010_e16798 * 0.3333333333333);
        let assign17010_e16801: f64 = (assign17010_e16800).exp();
        let assign17010_e16802: f64 = (1.0 + assign17010_e16801);
        let assign17010_e16803: f64 = (assign17010_e16802).ln();
        (assign17010_e16803, ((assign17010_e16801 * ((locals.var_q_x2sat_dn4 - locals.var_q_x2_wi_dn4) * 0.3333333333333)) / assign17010_e16802), ((assign17010_e16801 * ((locals.var_q_x2sat_dn6 - locals.var_q_x2_wi_dn6) * 0.3333333333333)) / assign17010_e16802), ((assign17010_e16801 * ((locals.var_q_x2sat_dn7 - locals.var_q_x2_wi_dn7) * 0.3333333333333)) / assign17010_e16802), ((assign17010_e16801 * ((locals.var_q_x2sat_dn8 - locals.var_q_x2_wi_dn8) * 0.3333333333333)) / assign17010_e16802), ((assign17010_e16801 * ((locals.var_q_x2sat_dn9 - locals.var_q_x2_wi_dn9) * 0.3333333333333)) / assign17010_e16802),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign17010_e16805;
        locals.var_q_temp3_dn4 = assign17010_e16805_d_n4;
        locals.var_q_temp3_dn6 = assign17010_e16805_d_n6;
        locals.var_q_temp3_dn7 = assign17010_e16805_d_n7;
        locals.var_q_temp3_dn8 = assign17010_e16805_d_n8;
        locals.var_q_temp3_dn9 = assign17010_e16805_d_n9;

        let (assign17020_e16814, assign17020_e16814_d_n4, assign17020_e16814_d_n6, assign17020_e16814_d_n7, assign17020_e16814_d_n8, assign17020_e16814_d_n9,) = {
    if (locals.var_guard618 == 0.0) {
        let assign17020_e16810: f64 = (locals.var_q_x2sat - locals.var_q_x2_wi);
        let assign17020_e16812: f64 = (assign17020_e16810 * 0.3333333333333);
        (assign17020_e16812, ((locals.var_q_x2sat_dn4 - locals.var_q_x2_wi_dn4) * 0.3333333333333), ((locals.var_q_x2sat_dn6 - locals.var_q_x2_wi_dn6) * 0.3333333333333), ((locals.var_q_x2sat_dn7 - locals.var_q_x2_wi_dn7) * 0.3333333333333), ((locals.var_q_x2sat_dn8 - locals.var_q_x2_wi_dn8) * 0.3333333333333), ((locals.var_q_x2sat_dn9 - locals.var_q_x2_wi_dn9) * 0.3333333333333),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign17020_e16814;
        locals.var_q_temp3_dn4 = assign17020_e16814_d_n4;
        locals.var_q_temp3_dn6 = assign17020_e16814_d_n6;
        locals.var_q_temp3_dn7 = assign17020_e16814_d_n7;
        locals.var_q_temp3_dn8 = assign17020_e16814_d_n8;
        locals.var_q_temp3_dn9 = assign17020_e16814_d_n9;

        let assign17030_e16818: f64 = (3.0 * locals.var_q_temp3);
        let assign17030_e16819: f64 = (locals.var_q_x2sat - assign17030_e16818);
        locals.var_q_x2 = assign17030_e16819;
        locals.var_q_x2_dn4 = (locals.var_q_x2sat_dn4 - (3.0 * locals.var_q_temp3_dn4));
        locals.var_q_x2_dn6 = (locals.var_q_x2sat_dn6 - (3.0 * locals.var_q_temp3_dn6));
        locals.var_q_x2_dn7 = (locals.var_q_x2sat_dn7 - (3.0 * locals.var_q_temp3_dn7));
        locals.var_q_x2_dn8 = (locals.var_q_x2sat_dn8 - (3.0 * locals.var_q_temp3_dn8));
        locals.var_q_x2_dn9 = (locals.var_q_x2sat_dn9 - (3.0 * locals.var_q_temp3_dn9));

        let assign17040_e16822: f64 = (locals.var_xg1x - locals.var_q_x1);
        locals.var_q1d = assign17040_e16822;
        locals.var_q1d_dn4 = (locals.var_xg1x_dn4 - locals.var_q_x1_dn4);
        locals.var_q1d_dn6 = (locals.var_xg1x_dn6 - locals.var_q_x1_dn6);
        locals.var_q1d_dn7 = (locals.var_xg1x_dn7 - locals.var_q_x1_dn7);
        locals.var_q1d_dn8 = (locals.var_xg1x_dn8 - locals.var_q_x1_dn8);
        locals.var_q1d_dn9 = (locals.var_xg1x_dn9 - locals.var_q_x1_dn9);

        let assign17050_e16825: f64 = (locals.var_xg2x - locals.var_q_x2);
        locals.var_q2d = assign17050_e16825;
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

        let assign17080_e16830: f64 = (locals.var_k1 * locals.var_q1d);
        locals.var_q_k1q1 = assign17080_e16830;
        locals.var_q_k1q1_dn4 = ((locals.var_k1_dn4 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn4));
        locals.var_q_k1q1_dn6 = ((locals.var_k1_dn6 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn6));
        locals.var_q_k1q1_dn7 = ((locals.var_k1_dn7 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn7));
        locals.var_q_k1q1_dn8 = ((locals.var_k1_dn8 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn8));
        locals.var_q_k1q1_dn9 = ((locals.var_k1_dn9 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn9));

        let assign17090_e16833: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign17090_e16835: f64 = (assign17090_e16833 - locals.var_xdeff);
        let assign17090_e16837: f64 = if assign17090_e16835 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard619 = assign17090_e16837;

        let (assign17100_e16846, assign17100_e16846_d_n4, assign17100_e16846_d_n6, assign17100_e16846_d_n7, assign17100_e16846_d_n8, assign17100_e16846_d_n9,) = {
    if (locals.var_guard619 != 0.0) {
        let assign17100_e16841: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign17100_e16843: f64 = (assign17100_e16841 - locals.var_xdeff);
        let assign17100_e16844: f64 = (assign17100_e16843).exp();
        (assign17100_e16844, (assign17100_e16844 * ((locals.var_xg1x_dn4 - locals.var_q1d_dn4) - locals.var_xdeff_dn4)), (assign17100_e16844 * ((locals.var_xg1x_dn6 - locals.var_q1d_dn6) - locals.var_xdeff_dn6)), (assign17100_e16844 * ((locals.var_xg1x_dn7 - locals.var_q1d_dn7) - locals.var_xdeff_dn7)), (assign17100_e16844 * ((locals.var_xg1x_dn8 - locals.var_q1d_dn8) - locals.var_xdeff_dn8)), (assign17100_e16844 * ((locals.var_xg1x_dn9 - locals.var_q1d_dn9) - locals.var_xdeff_dn9)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign17100_e16846;
        locals.var_q_temp1_dn4 = assign17100_e16846_d_n4;
        locals.var_q_temp1_dn6 = assign17100_e16846_d_n6;
        locals.var_q_temp1_dn7 = assign17100_e16846_d_n7;
        locals.var_q_temp1_dn8 = assign17100_e16846_d_n8;
        locals.var_q_temp1_dn9 = assign17100_e16846_d_n9;

        let (assign17110_e16885, assign17110_e16885_d_n4, assign17110_e16885_d_n6, assign17110_e16885_d_n7, assign17110_e16885_d_n8, assign17110_e16885_d_n9,) = {
    if (locals.var_guard619 == 0.0) {
        let assign17110_e16853: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign17110_e16855: f64 = (assign17110_e16853 - locals.var_xdeff);
        let assign17110_e16857: f64 = (assign17110_e16855 - 80.0);
        let assign17110_e16862: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign17110_e16864: f64 = (assign17110_e16862 - locals.var_xdeff);
        let assign17110_e16866: f64 = (assign17110_e16864 - 80.0);
        let assign17110_e16867: f64 = (0.5 * assign17110_e16866);
        let assign17110_e16871: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign17110_e16873: f64 = (assign17110_e16871 - locals.var_xdeff);
        let assign17110_e16875: f64 = (assign17110_e16873 - 80.0);
        let assign17110_e16877: f64 = (assign17110_e16875 * 0.3333333333333);
        let assign17110_e16878: f64 = (1.0 + assign17110_e16877);
        let assign17110_e16879: f64 = (assign17110_e16867 * assign17110_e16878);
        let assign17110_e16880: f64 = (1.0 + assign17110_e16879);
        let assign17110_e16881: f64 = (assign17110_e16857 * assign17110_e16880);
        let assign17110_e16882: f64 = (1.0 + assign17110_e16881);
        let assign17110_e16883: f64 = (5.54062e34 * assign17110_e16882);
        (assign17110_e16883, (5.54062e34 * ((((locals.var_xg1x_dn4 - locals.var_q1d_dn4) - locals.var_xdeff_dn4) * assign17110_e16880) + (assign17110_e16857 * (((0.5 * ((locals.var_xg1x_dn4 - locals.var_q1d_dn4) - locals.var_xdeff_dn4)) * assign17110_e16878) + (assign17110_e16867 * (((locals.var_xg1x_dn4 - locals.var_q1d_dn4) - locals.var_xdeff_dn4) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x_dn6 - locals.var_q1d_dn6) - locals.var_xdeff_dn6) * assign17110_e16880) + (assign17110_e16857 * (((0.5 * ((locals.var_xg1x_dn6 - locals.var_q1d_dn6) - locals.var_xdeff_dn6)) * assign17110_e16878) + (assign17110_e16867 * (((locals.var_xg1x_dn6 - locals.var_q1d_dn6) - locals.var_xdeff_dn6) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x_dn7 - locals.var_q1d_dn7) - locals.var_xdeff_dn7) * assign17110_e16880) + (assign17110_e16857 * (((0.5 * ((locals.var_xg1x_dn7 - locals.var_q1d_dn7) - locals.var_xdeff_dn7)) * assign17110_e16878) + (assign17110_e16867 * (((locals.var_xg1x_dn7 - locals.var_q1d_dn7) - locals.var_xdeff_dn7) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x_dn8 - locals.var_q1d_dn8) - locals.var_xdeff_dn8) * assign17110_e16880) + (assign17110_e16857 * (((0.5 * ((locals.var_xg1x_dn8 - locals.var_q1d_dn8) - locals.var_xdeff_dn8)) * assign17110_e16878) + (assign17110_e16867 * (((locals.var_xg1x_dn8 - locals.var_q1d_dn8) - locals.var_xdeff_dn8) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x_dn9 - locals.var_q1d_dn9) - locals.var_xdeff_dn9) * assign17110_e16880) + (assign17110_e16857 * (((0.5 * ((locals.var_xg1x_dn9 - locals.var_q1d_dn9) - locals.var_xdeff_dn9)) * assign17110_e16878) + (assign17110_e16867 * (((locals.var_xg1x_dn9 - locals.var_q1d_dn9) - locals.var_xdeff_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign17110_e16885;
        locals.var_q_temp1_dn4 = assign17110_e16885_d_n4;
        locals.var_q_temp1_dn6 = assign17110_e16885_d_n6;
        locals.var_q_temp1_dn7 = assign17110_e16885_d_n7;
        locals.var_q_temp1_dn8 = assign17110_e16885_d_n8;
        locals.var_q_temp1_dn9 = assign17110_e16885_d_n9;

        let assign17120_e16888: f64 = (locals.var_a0 * locals.var_q_temp1);
        locals.var_q_aexp = assign17120_e16888;
        locals.var_q_aexp_dn4 = ((locals.var_a0_dn4 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn4));
        locals.var_q_aexp_dn6 = ((locals.var_a0_dn6 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn6));
        locals.var_q_aexp_dn7 = ((locals.var_a0_dn7 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn7));
        locals.var_q_aexp_dn8 = ((locals.var_a0_dn8 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn8));
        locals.var_q_aexp_dn9 = ((locals.var_a0_dn9 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn9));

        let assign17130_e16891: f64 = (locals.var_q_k1q1 * locals.var_q_k1q1);
        let assign17130_e16893: f64 = (assign17130_e16891 - locals.var_q_aexp);
        locals.var_q_qsq = assign17130_e16893;
        locals.var_q_qsq_dn4 = (((locals.var_q_k1q1_dn4 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn4)) - locals.var_q_aexp_dn4);
        locals.var_q_qsq_dn6 = (((locals.var_q_k1q1_dn6 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn6)) - locals.var_q_aexp_dn6);
        locals.var_q_qsq_dn7 = (((locals.var_q_k1q1_dn7 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn7)) - locals.var_q_aexp_dn7);
        locals.var_q_qsq_dn8 = (((locals.var_q_k1q1_dn8 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn8)) - locals.var_q_aexp_dn8);
        locals.var_q_qsq_dn9 = (((locals.var_q_k1q1_dn9 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn9)) - locals.var_q_aexp_dn9);

        let assign17140_e16896: f64 = (2.0 * locals.var_k1);
        let assign17140_e16898: f64 = (assign17140_e16896 * locals.var_q_k1q1);
        let assign17140_e16900: f64 = (assign17140_e16898 + locals.var_q_aexp);
        locals.var_q_d1_qsq = assign17140_e16900;
        locals.var_q_d1_qsq_dn4 = ((((2.0 * locals.var_k1_dn4) * locals.var_q_k1q1) + (assign17140_e16896 * locals.var_q_k1q1_dn4)) + locals.var_q_aexp_dn4);
        locals.var_q_d1_qsq_dn6 = ((((2.0 * locals.var_k1_dn6) * locals.var_q_k1q1) + (assign17140_e16896 * locals.var_q_k1q1_dn6)) + locals.var_q_aexp_dn6);
        locals.var_q_d1_qsq_dn7 = ((((2.0 * locals.var_k1_dn7) * locals.var_q_k1q1) + (assign17140_e16896 * locals.var_q_k1q1_dn7)) + locals.var_q_aexp_dn7);
        locals.var_q_d1_qsq_dn8 = ((((2.0 * locals.var_k1_dn8) * locals.var_q_k1q1) + (assign17140_e16896 * locals.var_q_k1q1_dn8)) + locals.var_q_aexp_dn8);
        locals.var_q_d1_qsq_dn9 = ((((2.0 * locals.var_k1_dn9) * locals.var_q_k1q1) + (assign17140_e16896 * locals.var_q_k1q1_dn9)) + locals.var_q_aexp_dn9);

        let assign17150_e16903: f64 = (2.0 * locals.var_k1);
        let assign17150_e16905: f64 = (assign17150_e16903 * locals.var_k1);
        let assign17150_e16907: f64 = (assign17150_e16905 - locals.var_q_aexp);
        locals.var_q_d2_qsq = assign17150_e16907;
        locals.var_q_d2_qsq_dn4 = ((((2.0 * locals.var_k1_dn4) * locals.var_k1) + (assign17150_e16903 * locals.var_k1_dn4)) - locals.var_q_aexp_dn4);
        locals.var_q_d2_qsq_dn6 = ((((2.0 * locals.var_k1_dn6) * locals.var_k1) + (assign17150_e16903 * locals.var_k1_dn6)) - locals.var_q_aexp_dn6);
        locals.var_q_d2_qsq_dn7 = ((((2.0 * locals.var_k1_dn7) * locals.var_k1) + (assign17150_e16903 * locals.var_k1_dn7)) - locals.var_q_aexp_dn7);
        locals.var_q_d2_qsq_dn8 = ((((2.0 * locals.var_k1_dn8) * locals.var_k1) + (assign17150_e16903 * locals.var_k1_dn8)) - locals.var_q_aexp_dn8);
        locals.var_q_d2_qsq_dn9 = ((((2.0 * locals.var_k1_dn9) * locals.var_k1) + (assign17150_e16903 * locals.var_k1_dn9)) - locals.var_q_aexp_dn9);

        let assign17160_e16910: f64 = (-0.005);
        let assign17160_e16911: f64 = if locals.var_q_qsq < assign17160_e16910 { 1.0 } else { 0.0 };
        locals.var_guard620 = assign17160_e16911;

        let (assign17170_e16917, assign17170_e16917_d_n4, assign17170_e16917_d_n6, assign17170_e16917_d_n7, assign17170_e16917_d_n8, assign17170_e16917_d_n9,) = {
    if (locals.var_guard620 != 0.0) {
        let assign17170_e16914: f64 = (locals.var_q_qsq).abs();
        let assign17170_e16915: f64 = (assign17170_e16914).sqrt();
        (assign17170_e16915, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign17170_e16915)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign17170_e16915)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign17170_e16915)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign17170_e16915)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign17170_e16915)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign17170_e16917;
        locals.var_q_rac_qsq_dn4 = assign17170_e16917_d_n4;
        locals.var_q_rac_qsq_dn6 = assign17170_e16917_d_n6;
        locals.var_q_rac_qsq_dn7 = assign17170_e16917_d_n7;
        locals.var_q_rac_qsq_dn8 = assign17170_e16917_d_n8;
        locals.var_q_rac_qsq_dn9 = assign17170_e16917_d_n9;

        let (assign17180_e16926, assign17180_e16926_d_n4, assign17180_e16926_d_n6, assign17180_e16926_d_n7, assign17180_e16926_d_n8, assign17180_e16926_d_n9,) = {
    if (locals.var_guard620 != 0.0) {
        let assign17180_e16922: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign17180_e16923: f64 = (assign17180_e16922).tan();
        let assign17180_e16924: f64 = (locals.var_q_rac_qsq / assign17180_e16923);
        (assign17180_e16924, (((locals.var_q_rac_qsq_dn4 * assign17180_e16923) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn4) / ((assign17180_e16922).cos() * (assign17180_e16922).cos())))) / (assign17180_e16923 * assign17180_e16923)), (((locals.var_q_rac_qsq_dn6 * assign17180_e16923) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn6) / ((assign17180_e16922).cos() * (assign17180_e16922).cos())))) / (assign17180_e16923 * assign17180_e16923)), (((locals.var_q_rac_qsq_dn7 * assign17180_e16923) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn7) / ((assign17180_e16922).cos() * (assign17180_e16922).cos())))) / (assign17180_e16923 * assign17180_e16923)), (((locals.var_q_rac_qsq_dn8 * assign17180_e16923) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn8) / ((assign17180_e16922).cos() * (assign17180_e16922).cos())))) / (assign17180_e16923 * assign17180_e16923)), (((locals.var_q_rac_qsq_dn9 * assign17180_e16923) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn9) / ((assign17180_e16922).cos() * (assign17180_e16922).cos())))) / (assign17180_e16923 * assign17180_e16923)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign17180_e16926;
        locals.var_q_qcoth_dn4 = assign17180_e16926_d_n4;
        locals.var_q_qcoth_dn6 = assign17180_e16926_d_n6;
        locals.var_q_qcoth_dn7 = assign17180_e16926_d_n7;
        locals.var_q_qcoth_dn8 = assign17180_e16926_d_n8;
        locals.var_q_qcoth_dn9 = assign17180_e16926_d_n9;

        let (assign17190_e16934, assign17190_e16934_d_n4, assign17190_e16934_d_n6, assign17190_e16934_d_n7, assign17190_e16934_d_n8, assign17190_e16934_d_n9,) = {
    if (locals.var_guard620 != 0.0) {
        let assign17190_e16930: f64 = (0.25 * locals.var_q_d1_qsq);
        let assign17190_e16932: f64 = (assign17190_e16930 / locals.var_q_qsq);
        (assign17190_e16932, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign17190_e16930 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign17190_e16930 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign17190_e16930 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign17190_e16930 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign17190_e16930 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign17190_e16934;
        locals.var_q_temp1_dn4 = assign17190_e16934_d_n4;
        locals.var_q_temp1_dn6 = assign17190_e16934_d_n6;
        locals.var_q_temp1_dn7 = assign17190_e16934_d_n7;
        locals.var_q_temp1_dn8 = assign17190_e16934_d_n8;
        locals.var_q_temp1_dn9 = assign17190_e16934_d_n9;

        let (assign17200_e16946, assign17200_e16946_d_n4, assign17200_e16946_d_n6, assign17200_e16946_d_n7, assign17200_e16946_d_n8, assign17200_e16946_d_n9,) = {
    if (locals.var_guard620 != 0.0) {
        let assign17200_e16940: f64 = (2.0 - locals.var_q_qcoth);
        let assign17200_e16941: f64 = (locals.var_q_qcoth * assign17200_e16940);
        let assign17200_e16942: f64 = (locals.var_q_qsq + assign17200_e16941);
        let assign17200_e16944: f64 = (assign17200_e16942 * locals.var_q_temp1);
        (assign17200_e16944, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign17200_e16940) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign17200_e16942 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign17200_e16940) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign17200_e16942 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign17200_e16940) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign17200_e16942 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign17200_e16940) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign17200_e16942 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign17200_e16940) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign17200_e16942 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign17200_e16946;
        locals.var_q_d1_qcoth_dn4 = assign17200_e16946_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign17200_e16946_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign17200_e16946_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign17200_e16946_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign17200_e16946_d_n9;

        let (assign17210_e16966, assign17210_e16966_d_n4, assign17210_e16966_d_n6, assign17210_e16966_d_n7, assign17210_e16966_d_n8, assign17210_e16966_d_n9,) = {
    if (locals.var_guard620 != 0.0) {
        let assign17210_e16951: f64 = (2.0 * locals.var_q_d1_qcoth);
        let assign17210_e16954: f64 = (1.0 + locals.var_q_qcoth);
        let assign17210_e16955: f64 = (assign17210_e16951 * assign17210_e16954);
        let assign17210_e16956: f64 = (locals.var_q_d1_qsq - assign17210_e16955);
        let assign17210_e16958: f64 = (assign17210_e16956 * locals.var_q_temp1);
        let assign17210_e16961: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
        let assign17210_e16963: f64 = (assign17210_e16961 / locals.var_q_d1_qsq);
        let assign17210_e16964: f64 = (assign17210_e16958 + assign17210_e16963);
        (assign17210_e16964, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign17210_e16954) + (assign17210_e16951 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign17210_e16956 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign17210_e16961 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign17210_e16954) + (assign17210_e16951 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign17210_e16956 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign17210_e16961 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign17210_e16954) + (assign17210_e16951 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign17210_e16956 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign17210_e16961 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign17210_e16954) + (assign17210_e16951 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign17210_e16956 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign17210_e16961 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign17210_e16954) + (assign17210_e16951 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign17210_e16956 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign17210_e16961 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign17210_e16966;
        locals.var_q_d2_qcoth_dn4 = assign17210_e16966_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign17210_e16966_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign17210_e16966_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign17210_e16966_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign17210_e16966_d_n9;

    }

    pub(super) fn stamp_transient_block_43(
        locals: &mut StampLocals,
    ) {
        let (assign17220_e16974, assign17220_e16974_d_n4, assign17220_e16974_d_n6, assign17220_e16974_d_n7, assign17220_e16974_d_n8, assign17220_e16974_d_n9,) = {
    if (locals.var_guard620 != 0.0) {
        let assign17220_e16971: f64 = (0.5 * locals.var_q_qcoth);
        let assign17220_e16972: f64 = (1.0 - assign17220_e16971);
        (assign17220_e16972, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign17220_e16974;
        locals.var_q_temp2_dn4 = assign17220_e16974_d_n4;
        locals.var_q_temp2_dn6 = assign17220_e16974_d_n6;
        locals.var_q_temp2_dn7 = assign17220_e16974_d_n7;
        locals.var_q_temp2_dn8 = assign17220_e16974_d_n8;
        locals.var_q_temp2_dn9 = assign17220_e16974_d_n9;

        let (assign17230_e16982, assign17230_e16982_d_n4, assign17230_e16982_d_n6, assign17230_e16982_d_n7, assign17230_e16982_d_n8, assign17230_e16982_d_n9,) = {
    if (locals.var_guard620 != 0.0) {
        let assign17230_e16978: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
        let assign17230_e16980: f64 = (assign17230_e16978 * locals.var_q_temp2);
        (assign17230_e16980, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign17230_e16978 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign17230_e16978 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign17230_e16978 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign17230_e16978 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign17230_e16978 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign17230_e16982;
        locals.var_q_d1_ln_dn4 = assign17230_e16982_d_n4;
        locals.var_q_d1_ln_dn6 = assign17230_e16982_d_n6;
        locals.var_q_d1_ln_dn7 = assign17230_e16982_d_n7;
        locals.var_q_d1_ln_dn8 = assign17230_e16982_d_n8;
        locals.var_q_d1_ln_dn9 = assign17230_e16982_d_n9;

        let (assign17240_e16998, assign17240_e16998_d_n4, assign17240_e16998_d_n6, assign17240_e16998_d_n7, assign17240_e16998_d_n8, assign17240_e16998_d_n9,) = {
    if (locals.var_guard620 != 0.0) {
        let assign17240_e16986: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
        let assign17240_e16991: f64 = (0.5 * locals.var_q_d1_qcoth);
        let assign17240_e16992: f64 = (locals.var_q_d1_ln + assign17240_e16991);
        let assign17240_e16993: f64 = (locals.var_q_d1_qsq * assign17240_e16992);
        let assign17240_e16994: f64 = (assign17240_e16986 - assign17240_e16993);
        let assign17240_e16996: f64 = (assign17240_e16994 / locals.var_q_qsq);
        (assign17240_e16996, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign17240_e16992) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign17240_e16994 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign17240_e16992) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign17240_e16994 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign17240_e16992) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign17240_e16994 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign17240_e16992) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign17240_e16994 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign17240_e16992) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign17240_e16994 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign17240_e16998;
        locals.var_q_d2_ln_dn4 = assign17240_e16998_d_n4;
        locals.var_q_d2_ln_dn6 = assign17240_e16998_d_n6;
        locals.var_q_d2_ln_dn7 = assign17240_e16998_d_n7;
        locals.var_q_d2_ln_dn8 = assign17240_e16998_d_n8;
        locals.var_q_d2_ln_dn9 = assign17240_e16998_d_n9;

        let assign17250_e17001: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard621 = assign17250_e17001;

        let (assign17260_e17010, assign17260_e17010_d_n4, assign17260_e17010_d_n6, assign17260_e17010_d_n7, assign17260_e17010_d_n8, assign17260_e17010_d_n9,) = {
    if ((locals.var_guard620 == 0.0) && (locals.var_guard621 != 0.0)) {
        let assign17260_e17007: f64 = (locals.var_q_qsq).abs();
        let assign17260_e17008: f64 = (assign17260_e17007).sqrt();
        (assign17260_e17008, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign17260_e17008)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign17260_e17008)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign17260_e17008)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign17260_e17008)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign17260_e17008)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign17260_e17010;
        locals.var_q_rac_qsq_dn4 = assign17260_e17010_d_n4;
        locals.var_q_rac_qsq_dn6 = assign17260_e17010_d_n6;
        locals.var_q_rac_qsq_dn7 = assign17260_e17010_d_n7;
        locals.var_q_rac_qsq_dn8 = assign17260_e17010_d_n8;
        locals.var_q_rac_qsq_dn9 = assign17260_e17010_d_n9;

        let (assign17270_e17019, assign17270_e17019_d_n4, assign17270_e17019_d_n6, assign17270_e17019_d_n7, assign17270_e17019_d_n8, assign17270_e17019_d_n9,) = {
    if ((locals.var_guard620 == 0.0) && (locals.var_guard621 != 0.0)) {
        let assign17270_e17016: f64 = (-locals.var_q_rac_qsq);
        let assign17270_e17017: f64 = (assign17270_e17016).exp();
        (assign17270_e17017, (assign17270_e17017 * (-locals.var_q_rac_qsq_dn4)), (assign17270_e17017 * (-locals.var_q_rac_qsq_dn6)), (assign17270_e17017 * (-locals.var_q_rac_qsq_dn7)), (assign17270_e17017 * (-locals.var_q_rac_qsq_dn8)), (assign17270_e17017 * (-locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_invexpq, locals.var_q_invexpq_dn4, locals.var_q_invexpq_dn6, locals.var_q_invexpq_dn7, locals.var_q_invexpq_dn8, locals.var_q_invexpq_dn9,)
    }
};
        locals.var_q_invexpq = assign17270_e17019;
        locals.var_q_invexpq_dn4 = assign17270_e17019_d_n4;
        locals.var_q_invexpq_dn6 = assign17270_e17019_d_n6;
        locals.var_q_invexpq_dn7 = assign17270_e17019_d_n7;
        locals.var_q_invexpq_dn8 = assign17270_e17019_d_n8;
        locals.var_q_invexpq_dn9 = assign17270_e17019_d_n9;

        let (assign17280_e17034, assign17280_e17034_d_n4, assign17280_e17034_d_n6, assign17280_e17034_d_n7, assign17280_e17034_d_n8, assign17280_e17034_d_n9,) = {
    if ((locals.var_guard620 == 0.0) && (locals.var_guard621 != 0.0)) {
        let assign17280_e17027: f64 = (1.0 + locals.var_q_invexpq);
        let assign17280_e17028: f64 = (locals.var_q_rac_qsq * assign17280_e17027);
        let assign17280_e17031: f64 = (1.0 - locals.var_q_invexpq);
        let assign17280_e17032: f64 = (assign17280_e17028 / assign17280_e17031);
        (assign17280_e17032, (((((locals.var_q_rac_qsq_dn4 * assign17280_e17027) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn4)) * assign17280_e17031) - (assign17280_e17028 * (-locals.var_q_invexpq_dn4))) / (assign17280_e17031 * assign17280_e17031)), (((((locals.var_q_rac_qsq_dn6 * assign17280_e17027) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn6)) * assign17280_e17031) - (assign17280_e17028 * (-locals.var_q_invexpq_dn6))) / (assign17280_e17031 * assign17280_e17031)), (((((locals.var_q_rac_qsq_dn7 * assign17280_e17027) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn7)) * assign17280_e17031) - (assign17280_e17028 * (-locals.var_q_invexpq_dn7))) / (assign17280_e17031 * assign17280_e17031)), (((((locals.var_q_rac_qsq_dn8 * assign17280_e17027) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn8)) * assign17280_e17031) - (assign17280_e17028 * (-locals.var_q_invexpq_dn8))) / (assign17280_e17031 * assign17280_e17031)), (((((locals.var_q_rac_qsq_dn9 * assign17280_e17027) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn9)) * assign17280_e17031) - (assign17280_e17028 * (-locals.var_q_invexpq_dn9))) / (assign17280_e17031 * assign17280_e17031)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign17280_e17034;
        locals.var_q_qcoth_dn4 = assign17280_e17034_d_n4;
        locals.var_q_qcoth_dn6 = assign17280_e17034_d_n6;
        locals.var_q_qcoth_dn7 = assign17280_e17034_d_n7;
        locals.var_q_qcoth_dn8 = assign17280_e17034_d_n8;
        locals.var_q_qcoth_dn9 = assign17280_e17034_d_n9;

        let (assign17290_e17045, assign17290_e17045_d_n4, assign17290_e17045_d_n6, assign17290_e17045_d_n7, assign17290_e17045_d_n8, assign17290_e17045_d_n9,) = {
    if ((locals.var_guard620 == 0.0) && (locals.var_guard621 != 0.0)) {
        let assign17290_e17041: f64 = (0.25 * locals.var_q_d1_qsq);
        let assign17290_e17043: f64 = (assign17290_e17041 / locals.var_q_qsq);
        (assign17290_e17043, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign17290_e17041 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign17290_e17041 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign17290_e17041 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign17290_e17041 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign17290_e17041 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign17290_e17045;
        locals.var_q_temp1_dn4 = assign17290_e17045_d_n4;
        locals.var_q_temp1_dn6 = assign17290_e17045_d_n6;
        locals.var_q_temp1_dn7 = assign17290_e17045_d_n7;
        locals.var_q_temp1_dn8 = assign17290_e17045_d_n8;
        locals.var_q_temp1_dn9 = assign17290_e17045_d_n9;

        let (assign17300_e17060, assign17300_e17060_d_n4, assign17300_e17060_d_n6, assign17300_e17060_d_n7, assign17300_e17060_d_n8, assign17300_e17060_d_n9,) = {
    if ((locals.var_guard620 == 0.0) && (locals.var_guard621 != 0.0)) {
        let assign17300_e17054: f64 = (2.0 - locals.var_q_qcoth);
        let assign17300_e17055: f64 = (locals.var_q_qcoth * assign17300_e17054);
        let assign17300_e17056: f64 = (locals.var_q_qsq + assign17300_e17055);
        let assign17300_e17058: f64 = (assign17300_e17056 * locals.var_q_temp1);
        (assign17300_e17058, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign17300_e17054) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign17300_e17056 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign17300_e17054) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign17300_e17056 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign17300_e17054) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign17300_e17056 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign17300_e17054) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign17300_e17056 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign17300_e17054) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign17300_e17056 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign17300_e17060;
        locals.var_q_d1_qcoth_dn4 = assign17300_e17060_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign17300_e17060_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign17300_e17060_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign17300_e17060_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign17300_e17060_d_n9;

        let (assign17310_e17083, assign17310_e17083_d_n4, assign17310_e17083_d_n6, assign17310_e17083_d_n7, assign17310_e17083_d_n8, assign17310_e17083_d_n9,) = {
    if ((locals.var_guard620 == 0.0) && (locals.var_guard621 != 0.0)) {
        let assign17310_e17068: f64 = (2.0 * locals.var_q_d1_qcoth);
        let assign17310_e17071: f64 = (1.0 + locals.var_q_qcoth);
        let assign17310_e17072: f64 = (assign17310_e17068 * assign17310_e17071);
        let assign17310_e17073: f64 = (locals.var_q_d1_qsq - assign17310_e17072);
        let assign17310_e17075: f64 = (assign17310_e17073 * locals.var_q_temp1);
        let assign17310_e17078: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
        let assign17310_e17080: f64 = (assign17310_e17078 / locals.var_q_d1_qsq);
        let assign17310_e17081: f64 = (assign17310_e17075 + assign17310_e17080);
        (assign17310_e17081, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign17310_e17071) + (assign17310_e17068 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign17310_e17073 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign17310_e17078 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign17310_e17071) + (assign17310_e17068 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign17310_e17073 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign17310_e17078 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign17310_e17071) + (assign17310_e17068 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign17310_e17073 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign17310_e17078 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign17310_e17071) + (assign17310_e17068 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign17310_e17073 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign17310_e17078 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign17310_e17071) + (assign17310_e17068 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign17310_e17073 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign17310_e17078 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign17310_e17083;
        locals.var_q_d2_qcoth_dn4 = assign17310_e17083_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign17310_e17083_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign17310_e17083_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign17310_e17083_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign17310_e17083_d_n9;

        let (assign17320_e17094, assign17320_e17094_d_n4, assign17320_e17094_d_n6, assign17320_e17094_d_n7, assign17320_e17094_d_n8, assign17320_e17094_d_n9,) = {
    if ((locals.var_guard620 == 0.0) && (locals.var_guard621 != 0.0)) {
        let assign17320_e17091: f64 = (0.5 * locals.var_q_qcoth);
        let assign17320_e17092: f64 = (1.0 - assign17320_e17091);
        (assign17320_e17092, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign17320_e17094;
        locals.var_q_temp2_dn4 = assign17320_e17094_d_n4;
        locals.var_q_temp2_dn6 = assign17320_e17094_d_n6;
        locals.var_q_temp2_dn7 = assign17320_e17094_d_n7;
        locals.var_q_temp2_dn8 = assign17320_e17094_d_n8;
        locals.var_q_temp2_dn9 = assign17320_e17094_d_n9;

        let (assign17330_e17105, assign17330_e17105_d_n4, assign17330_e17105_d_n6, assign17330_e17105_d_n7, assign17330_e17105_d_n8, assign17330_e17105_d_n9,) = {
    if ((locals.var_guard620 == 0.0) && (locals.var_guard621 != 0.0)) {
        let assign17330_e17101: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
        let assign17330_e17103: f64 = (assign17330_e17101 * locals.var_q_temp2);
        (assign17330_e17103, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign17330_e17101 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign17330_e17101 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign17330_e17101 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign17330_e17101 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign17330_e17101 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign17330_e17105;
        locals.var_q_d1_ln_dn4 = assign17330_e17105_d_n4;
        locals.var_q_d1_ln_dn6 = assign17330_e17105_d_n6;
        locals.var_q_d1_ln_dn7 = assign17330_e17105_d_n7;
        locals.var_q_d1_ln_dn8 = assign17330_e17105_d_n8;
        locals.var_q_d1_ln_dn9 = assign17330_e17105_d_n9;

        let (assign17340_e17124, assign17340_e17124_d_n4, assign17340_e17124_d_n6, assign17340_e17124_d_n7, assign17340_e17124_d_n8, assign17340_e17124_d_n9,) = {
    if ((locals.var_guard620 == 0.0) && (locals.var_guard621 != 0.0)) {
        let assign17340_e17112: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
        let assign17340_e17117: f64 = (0.5 * locals.var_q_d1_qcoth);
        let assign17340_e17118: f64 = (locals.var_q_d1_ln + assign17340_e17117);
        let assign17340_e17119: f64 = (locals.var_q_d1_qsq * assign17340_e17118);
        let assign17340_e17120: f64 = (assign17340_e17112 - assign17340_e17119);
        let assign17340_e17122: f64 = (assign17340_e17120 / locals.var_q_qsq);
        (assign17340_e17122, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign17340_e17118) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign17340_e17120 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign17340_e17118) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign17340_e17120 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign17340_e17118) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign17340_e17120 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign17340_e17118) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign17340_e17120 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign17340_e17118) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign17340_e17120 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign17340_e17124;
        locals.var_q_d2_ln_dn4 = assign17340_e17124_d_n4;
        locals.var_q_d2_ln_dn6 = assign17340_e17124_d_n6;
        locals.var_q_d2_ln_dn7 = assign17340_e17124_d_n7;
        locals.var_q_d2_ln_dn8 = assign17340_e17124_d_n8;
        locals.var_q_d2_ln_dn9 = assign17340_e17124_d_n9;

        let (assign17350_e17150, assign17350_e17150_d_n4, assign17350_e17150_d_n6, assign17350_e17150_d_n7, assign17350_e17150_d_n8, assign17350_e17150_d_n9,) = {
    if ((locals.var_guard620 == 0.0) && (locals.var_guard621 == 0.0)) {
        let assign17350_e17134: f64 = (locals.var_q_qsq * 0.0166666666667);
        let assign17350_e17138: f64 = (locals.var_q_qsq * 0.0238095238095);
        let assign17350_e17142: f64 = (locals.var_q_qsq * 0.025);
        let assign17350_e17143: f64 = (1.0 - assign17350_e17142);
        let assign17350_e17144: f64 = (assign17350_e17138 * assign17350_e17143);
        let assign17350_e17145: f64 = (1.0 - assign17350_e17144);
        let assign17350_e17146: f64 = (assign17350_e17134 * assign17350_e17145);
        let assign17350_e17147: f64 = (1.0 - assign17350_e17146);
        let assign17350_e17148: f64 = (0.1666666666667 * assign17350_e17147);
        (assign17350_e17148, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0166666666667) * assign17350_e17145) + (assign17350_e17134 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign17350_e17143) + (assign17350_e17138 * (-(locals.var_q_qsq_dn4 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0166666666667) * assign17350_e17145) + (assign17350_e17134 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign17350_e17143) + (assign17350_e17138 * (-(locals.var_q_qsq_dn6 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0166666666667) * assign17350_e17145) + (assign17350_e17134 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign17350_e17143) + (assign17350_e17138 * (-(locals.var_q_qsq_dn7 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0166666666667) * assign17350_e17145) + (assign17350_e17134 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign17350_e17143) + (assign17350_e17138 * (-(locals.var_q_qsq_dn8 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0166666666667) * assign17350_e17145) + (assign17350_e17134 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign17350_e17143) + (assign17350_e17138 * (-(locals.var_q_qsq_dn9 * 0.025))))))))),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign17350_e17150;
        locals.var_q_temp3_dn4 = assign17350_e17150_d_n4;
        locals.var_q_temp3_dn6 = assign17350_e17150_d_n6;
        locals.var_q_temp3_dn7 = assign17350_e17150_d_n7;
        locals.var_q_temp3_dn8 = assign17350_e17150_d_n8;
        locals.var_q_temp3_dn9 = assign17350_e17150_d_n9;

        let (assign17360_e17162, assign17360_e17162_d_n4, assign17360_e17162_d_n6, assign17360_e17162_d_n7, assign17360_e17162_d_n8, assign17360_e17162_d_n9,) = {
    if ((locals.var_guard620 == 0.0) && (locals.var_guard621 == 0.0)) {
        let assign17360_e17159: f64 = (locals.var_q_qsq * locals.var_q_temp3);
        let assign17360_e17160: f64 = (2.0 + assign17360_e17159);
        (assign17360_e17160, ((locals.var_q_qsq_dn4 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn4)), ((locals.var_q_qsq_dn6 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn6)), ((locals.var_q_qsq_dn7 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn7)), ((locals.var_q_qsq_dn8 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn8)), ((locals.var_q_qsq_dn9 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn9)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign17360_e17162;
        locals.var_q_qcoth_dn4 = assign17360_e17162_d_n4;
        locals.var_q_qcoth_dn6 = assign17360_e17162_d_n6;
        locals.var_q_qcoth_dn7 = assign17360_e17162_d_n7;
        locals.var_q_qcoth_dn8 = assign17360_e17162_d_n8;
        locals.var_q_qcoth_dn9 = assign17360_e17162_d_n9;

        let (assign17370_e17188, assign17370_e17188_d_n4, assign17370_e17188_d_n6, assign17370_e17188_d_n7, assign17370_e17188_d_n8, assign17370_e17188_d_n9,) = {
    if ((locals.var_guard620 == 0.0) && (locals.var_guard621 == 0.0)) {
        let assign17370_e17172: f64 = (locals.var_q_qsq * 0.0333333333333);
        let assign17370_e17176: f64 = (locals.var_q_qsq * 0.0357142857143);
        let assign17370_e17180: f64 = (locals.var_q_qsq * 0.0333333333333);
        let assign17370_e17181: f64 = (1.0 - assign17370_e17180);
        let assign17370_e17182: f64 = (assign17370_e17176 * assign17370_e17181);
        let assign17370_e17183: f64 = (1.0 - assign17370_e17182);
        let assign17370_e17184: f64 = (assign17370_e17172 * assign17370_e17183);
        let assign17370_e17185: f64 = (1.0 - assign17370_e17184);
        let assign17370_e17186: f64 = (0.1666666666667 * assign17370_e17185);
        (assign17370_e17186, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0333333333333) * assign17370_e17183) + (assign17370_e17172 * (-(((locals.var_q_qsq_dn4 * 0.0357142857143) * assign17370_e17181) + (assign17370_e17176 * (-(locals.var_q_qsq_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0333333333333) * assign17370_e17183) + (assign17370_e17172 * (-(((locals.var_q_qsq_dn6 * 0.0357142857143) * assign17370_e17181) + (assign17370_e17176 * (-(locals.var_q_qsq_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0333333333333) * assign17370_e17183) + (assign17370_e17172 * (-(((locals.var_q_qsq_dn7 * 0.0357142857143) * assign17370_e17181) + (assign17370_e17176 * (-(locals.var_q_qsq_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0333333333333) * assign17370_e17183) + (assign17370_e17172 * (-(((locals.var_q_qsq_dn8 * 0.0357142857143) * assign17370_e17181) + (assign17370_e17176 * (-(locals.var_q_qsq_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0333333333333) * assign17370_e17183) + (assign17370_e17172 * (-(((locals.var_q_qsq_dn9 * 0.0357142857143) * assign17370_e17181) + (assign17370_e17176 * (-(locals.var_q_qsq_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign17370_e17188;
        locals.var_q_temp1_dn4 = assign17370_e17188_d_n4;
        locals.var_q_temp1_dn6 = assign17370_e17188_d_n6;
        locals.var_q_temp1_dn7 = assign17370_e17188_d_n7;
        locals.var_q_temp1_dn8 = assign17370_e17188_d_n8;
        locals.var_q_temp1_dn9 = assign17370_e17188_d_n9;

        let (assign17380_e17198, assign17380_e17198_d_n4, assign17380_e17198_d_n6, assign17380_e17198_d_n7, assign17380_e17198_d_n8, assign17380_e17198_d_n9,) = {
    if ((locals.var_guard620 == 0.0) && (locals.var_guard621 == 0.0)) {
        let assign17380_e17196: f64 = (locals.var_q_d1_qsq * locals.var_q_temp1);
        (assign17380_e17196, ((locals.var_q_d1_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn4)), ((locals.var_q_d1_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn6)), ((locals.var_q_d1_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn7)), ((locals.var_q_d1_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn8)), ((locals.var_q_d1_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign17380_e17198;
        locals.var_q_d1_qcoth_dn4 = assign17380_e17198_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign17380_e17198_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign17380_e17198_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign17380_e17198_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign17380_e17198_d_n9;

        let (assign17390_e17224, assign17390_e17224_d_n4, assign17390_e17224_d_n6, assign17390_e17224_d_n7, assign17390_e17224_d_n8, assign17390_e17224_d_n9,) = {
    if ((locals.var_guard620 == 0.0) && (locals.var_guard621 == 0.0)) {
        let assign17390_e17208: f64 = (locals.var_q_qsq * 0.0714285714286);
        let assign17390_e17212: f64 = (0.05 * locals.var_q_qsq);
        let assign17390_e17216: f64 = (0.0420875420875421 * locals.var_q_qsq);
        let assign17390_e17217: f64 = (1.0 - assign17390_e17216);
        let assign17390_e17218: f64 = (assign17390_e17212 * assign17390_e17217);
        let assign17390_e17219: f64 = (1.0 - assign17390_e17218);
        let assign17390_e17220: f64 = (assign17390_e17208 * assign17390_e17219);
        let assign17390_e17221: f64 = (1.0 - assign17390_e17220);
        let assign17390_e17222: f64 = (0.0055555555556 * assign17390_e17221);
        (assign17390_e17222, (0.0055555555556 * (-(((locals.var_q_qsq_dn4 * 0.0714285714286) * assign17390_e17219) + (assign17390_e17208 * (-(((0.05 * locals.var_q_qsq_dn4) * assign17390_e17217) + (assign17390_e17212 * (-(0.0420875420875421 * locals.var_q_qsq_dn4))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn6 * 0.0714285714286) * assign17390_e17219) + (assign17390_e17208 * (-(((0.05 * locals.var_q_qsq_dn6) * assign17390_e17217) + (assign17390_e17212 * (-(0.0420875420875421 * locals.var_q_qsq_dn6))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn7 * 0.0714285714286) * assign17390_e17219) + (assign17390_e17208 * (-(((0.05 * locals.var_q_qsq_dn7) * assign17390_e17217) + (assign17390_e17212 * (-(0.0420875420875421 * locals.var_q_qsq_dn7))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn8 * 0.0714285714286) * assign17390_e17219) + (assign17390_e17208 * (-(((0.05 * locals.var_q_qsq_dn8) * assign17390_e17217) + (assign17390_e17212 * (-(0.0420875420875421 * locals.var_q_qsq_dn8))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn9 * 0.0714285714286) * assign17390_e17219) + (assign17390_e17208 * (-(((0.05 * locals.var_q_qsq_dn9) * assign17390_e17217) + (assign17390_e17212 * (-(0.0420875420875421 * locals.var_q_qsq_dn9))))))))),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign17390_e17224;
        locals.var_q_temp2_dn4 = assign17390_e17224_d_n4;
        locals.var_q_temp2_dn6 = assign17390_e17224_d_n6;
        locals.var_q_temp2_dn7 = assign17390_e17224_d_n7;
        locals.var_q_temp2_dn8 = assign17390_e17224_d_n8;
        locals.var_q_temp2_dn9 = assign17390_e17224_d_n9;

        let (assign17400_e17240, assign17400_e17240_d_n4, assign17400_e17240_d_n6, assign17400_e17240_d_n7, assign17400_e17240_d_n8, assign17400_e17240_d_n9,) = {
    if ((locals.var_guard620 == 0.0) && (locals.var_guard621 == 0.0)) {
        let assign17400_e17232: f64 = (locals.var_q_d2_qsq * locals.var_q_temp1);
        let assign17400_e17235: f64 = (locals.var_q_d1_qsq * locals.var_q_d1_qsq);
        let assign17400_e17237: f64 = (assign17400_e17235 * locals.var_q_temp2);
        let assign17400_e17238: f64 = (assign17400_e17232 - assign17400_e17237);
        (assign17400_e17238, (((locals.var_q_d2_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn4)) - ((((locals.var_q_d1_qsq_dn4 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn4)) * locals.var_q_temp2) + (assign17400_e17235 * locals.var_q_temp2_dn4))), (((locals.var_q_d2_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn6)) - ((((locals.var_q_d1_qsq_dn6 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn6)) * locals.var_q_temp2) + (assign17400_e17235 * locals.var_q_temp2_dn6))), (((locals.var_q_d2_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn7)) - ((((locals.var_q_d1_qsq_dn7 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn7)) * locals.var_q_temp2) + (assign17400_e17235 * locals.var_q_temp2_dn7))), (((locals.var_q_d2_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn8)) - ((((locals.var_q_d1_qsq_dn8 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn8)) * locals.var_q_temp2) + (assign17400_e17235 * locals.var_q_temp2_dn8))), (((locals.var_q_d2_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn9)) - ((((locals.var_q_d1_qsq_dn9 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn9)) * locals.var_q_temp2) + (assign17400_e17235 * locals.var_q_temp2_dn9))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign17400_e17240;
        locals.var_q_d2_qcoth_dn4 = assign17400_e17240_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign17400_e17240_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign17400_e17240_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign17400_e17240_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign17400_e17240_d_n9;

        let (assign17410_e17253, assign17410_e17253_d_n4, assign17410_e17253_d_n6, assign17410_e17253_d_n7, assign17410_e17253_d_n8, assign17410_e17253_d_n9,) = {
    if ((locals.var_guard620 == 0.0) && (locals.var_guard621 == 0.0)) {
        let assign17410_e17247: f64 = (-0.5);
        let assign17410_e17249: f64 = (assign17410_e17247 * locals.var_q_d1_qsq);
        let assign17410_e17251: f64 = (assign17410_e17249 * locals.var_q_temp3);
        (assign17410_e17251, (((assign17410_e17247 * locals.var_q_d1_qsq_dn4) * locals.var_q_temp3) + (assign17410_e17249 * locals.var_q_temp3_dn4)), (((assign17410_e17247 * locals.var_q_d1_qsq_dn6) * locals.var_q_temp3) + (assign17410_e17249 * locals.var_q_temp3_dn6)), (((assign17410_e17247 * locals.var_q_d1_qsq_dn7) * locals.var_q_temp3) + (assign17410_e17249 * locals.var_q_temp3_dn7)), (((assign17410_e17247 * locals.var_q_d1_qsq_dn8) * locals.var_q_temp3) + (assign17410_e17249 * locals.var_q_temp3_dn8)), (((assign17410_e17247 * locals.var_q_d1_qsq_dn9) * locals.var_q_temp3) + (assign17410_e17249 * locals.var_q_temp3_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign17410_e17253;
        locals.var_q_d1_ln_dn4 = assign17410_e17253_d_n4;
        locals.var_q_d1_ln_dn6 = assign17410_e17253_d_n6;
        locals.var_q_d1_ln_dn7 = assign17410_e17253_d_n7;
        locals.var_q_d1_ln_dn8 = assign17410_e17253_d_n8;
        locals.var_q_d1_ln_dn9 = assign17410_e17253_d_n9;

        let (assign17420_e17286, assign17420_e17286_d_n4, assign17420_e17286_d_n6, assign17420_e17286_d_n7, assign17420_e17286_d_n8, assign17420_e17286_d_n9,) = {
    if ((locals.var_guard620 == 0.0) && (locals.var_guard621 == 0.0)) {
        let assign17420_e17260: f64 = (-0.5);
        let assign17420_e17262: f64 = (assign17420_e17260 * locals.var_q_d2_qsq);
        let assign17420_e17264: f64 = (assign17420_e17262 * locals.var_q_temp3);
        let assign17420_e17267: f64 = (0.25 * 0.0055555555556);
        let assign17420_e17269: f64 = (assign17420_e17267 * locals.var_q_d1_qsq);
        let assign17420_e17271: f64 = (assign17420_e17269 * locals.var_q_d1_qsq);
        let assign17420_e17275: f64 = (locals.var_q_qsq * 0.0238095238095);
        let assign17420_e17279: f64 = (0.075 * locals.var_q_qsq);
        let assign17420_e17280: f64 = (2.0 - assign17420_e17279);
        let assign17420_e17281: f64 = (assign17420_e17275 * assign17420_e17280);
        let assign17420_e17282: f64 = (1.0 - assign17420_e17281);
        let assign17420_e17283: f64 = (assign17420_e17271 * assign17420_e17282);
        let assign17420_e17284: f64 = (assign17420_e17264 + assign17420_e17283);
        (assign17420_e17284, ((((assign17420_e17260 * locals.var_q_d2_qsq_dn4) * locals.var_q_temp3) + (assign17420_e17262 * locals.var_q_temp3_dn4)) + (((((assign17420_e17267 * locals.var_q_d1_qsq_dn4) * locals.var_q_d1_qsq) + (assign17420_e17269 * locals.var_q_d1_qsq_dn4)) * assign17420_e17282) + (assign17420_e17271 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign17420_e17280) + (assign17420_e17275 * (-(0.075 * locals.var_q_qsq_dn4)))))))), ((((assign17420_e17260 * locals.var_q_d2_qsq_dn6) * locals.var_q_temp3) + (assign17420_e17262 * locals.var_q_temp3_dn6)) + (((((assign17420_e17267 * locals.var_q_d1_qsq_dn6) * locals.var_q_d1_qsq) + (assign17420_e17269 * locals.var_q_d1_qsq_dn6)) * assign17420_e17282) + (assign17420_e17271 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign17420_e17280) + (assign17420_e17275 * (-(0.075 * locals.var_q_qsq_dn6)))))))), ((((assign17420_e17260 * locals.var_q_d2_qsq_dn7) * locals.var_q_temp3) + (assign17420_e17262 * locals.var_q_temp3_dn7)) + (((((assign17420_e17267 * locals.var_q_d1_qsq_dn7) * locals.var_q_d1_qsq) + (assign17420_e17269 * locals.var_q_d1_qsq_dn7)) * assign17420_e17282) + (assign17420_e17271 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign17420_e17280) + (assign17420_e17275 * (-(0.075 * locals.var_q_qsq_dn7)))))))), ((((assign17420_e17260 * locals.var_q_d2_qsq_dn8) * locals.var_q_temp3) + (assign17420_e17262 * locals.var_q_temp3_dn8)) + (((((assign17420_e17267 * locals.var_q_d1_qsq_dn8) * locals.var_q_d1_qsq) + (assign17420_e17269 * locals.var_q_d1_qsq_dn8)) * assign17420_e17282) + (assign17420_e17271 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign17420_e17280) + (assign17420_e17275 * (-(0.075 * locals.var_q_qsq_dn8)))))))), ((((assign17420_e17260 * locals.var_q_d2_qsq_dn9) * locals.var_q_temp3) + (assign17420_e17262 * locals.var_q_temp3_dn9)) + (((((assign17420_e17267 * locals.var_q_d1_qsq_dn9) * locals.var_q_d1_qsq) + (assign17420_e17269 * locals.var_q_d1_qsq_dn9)) * assign17420_e17282) + (assign17420_e17271 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign17420_e17280) + (assign17420_e17275 * (-(0.075 * locals.var_q_qsq_dn9)))))))),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign17420_e17286;
        locals.var_q_d2_ln_dn4 = assign17420_e17286_d_n4;
        locals.var_q_d2_ln_dn6 = assign17420_e17286_d_n6;
        locals.var_q_d2_ln_dn7 = assign17420_e17286_d_n7;
        locals.var_q_d2_ln_dn8 = assign17420_e17286_d_n8;
        locals.var_q_d2_ln_dn9 = assign17420_e17286_d_n9;

        let assign17430_e17289: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard622 = assign17430_e17289;

        let (assign17440_e17303, assign17440_e17303_d_n4, assign17440_e17303_d_n6, assign17440_e17303_d_n7, assign17440_e17303_d_n8, assign17440_e17303_d_n9,) = {
    if (locals.var_guard622 != 0.0) {
        let assign17440_e17293: f64 = (4.0 * locals.var_q_qsq);
        let assign17440_e17298: f64 = (2.0 - locals.var_q_invexpq);
        let assign17440_e17299: f64 = (locals.var_q_invexpq * assign17440_e17298);
        let assign17440_e17300: f64 = (1.0 - assign17440_e17299);
        let assign17440_e17301: f64 = (assign17440_e17293 / assign17440_e17300);
        (assign17440_e17301, ((((4.0 * locals.var_q_qsq_dn4) * assign17440_e17300) - (assign17440_e17293 * (-((locals.var_q_invexpq_dn4 * assign17440_e17298) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn4)))))) / (assign17440_e17300 * assign17440_e17300)), ((((4.0 * locals.var_q_qsq_dn6) * assign17440_e17300) - (assign17440_e17293 * (-((locals.var_q_invexpq_dn6 * assign17440_e17298) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn6)))))) / (assign17440_e17300 * assign17440_e17300)), ((((4.0 * locals.var_q_qsq_dn7) * assign17440_e17300) - (assign17440_e17293 * (-((locals.var_q_invexpq_dn7 * assign17440_e17298) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn7)))))) / (assign17440_e17300 * assign17440_e17300)), ((((4.0 * locals.var_q_qsq_dn8) * assign17440_e17300) - (assign17440_e17293 * (-((locals.var_q_invexpq_dn8 * assign17440_e17298) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn8)))))) / (assign17440_e17300 * assign17440_e17300)), ((((4.0 * locals.var_q_qsq_dn9) * assign17440_e17300) - (assign17440_e17293 * (-((locals.var_q_invexpq_dn9 * assign17440_e17298) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn9)))))) / (assign17440_e17300 * assign17440_e17300)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign17440_e17303;
        locals.var_q_temp2_dn4 = assign17440_e17303_d_n4;
        locals.var_q_temp2_dn6 = assign17440_e17303_d_n6;
        locals.var_q_temp2_dn7 = assign17440_e17303_d_n7;
        locals.var_q_temp2_dn8 = assign17440_e17303_d_n8;
        locals.var_q_temp2_dn9 = assign17440_e17303_d_n9;

        let (assign17450_e17309, assign17450_e17309_d_n4, assign17450_e17309_d_n6, assign17450_e17309_d_n7, assign17450_e17309_d_n8, assign17450_e17309_d_n9,) = {
    if (locals.var_guard622 != 0.0) {
        let assign17450_e17307: f64 = (locals.var_q_temp2 * locals.var_q_invexpq);
        (assign17450_e17307, ((locals.var_q_temp2_dn4 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn4)), ((locals.var_q_temp2_dn6 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn6)), ((locals.var_q_temp2_dn7 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn7)), ((locals.var_q_temp2_dn8 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn8)), ((locals.var_q_temp2_dn9 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn9)),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign17450_e17309;
        locals.var_q_sh_term_dn4 = assign17450_e17309_d_n4;
        locals.var_q_sh_term_dn6 = assign17450_e17309_d_n6;
        locals.var_q_sh_term_dn7 = assign17450_e17309_d_n7;
        locals.var_q_sh_term_dn8 = assign17450_e17309_d_n8;
        locals.var_q_sh_term_dn9 = assign17450_e17309_d_n9;

        let (assign17460_e17316, assign17460_e17316_d_n4, assign17460_e17316_d_n6, assign17460_e17316_d_n7, assign17460_e17316_d_n8, assign17460_e17316_d_n9,) = {
    if (locals.var_guard622 != 0.0) {
        let assign17460_e17312: f64 = (locals.var_q_temp2).ln();
        let assign17460_e17314: f64 = (assign17460_e17312 - locals.var_q_rac_qsq);
        (assign17460_e17314, ((locals.var_q_temp2_dn4 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn4), ((locals.var_q_temp2_dn6 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn6), ((locals.var_q_temp2_dn7 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn7), ((locals.var_q_temp2_dn8 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn8), ((locals.var_q_temp2_dn9 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn9),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign17460_e17316;
        locals.var_q_ln_term_dn4 = assign17460_e17316_d_n4;
        locals.var_q_ln_term_dn6 = assign17460_e17316_d_n6;
        locals.var_q_ln_term_dn7 = assign17460_e17316_d_n7;
        locals.var_q_ln_term_dn8 = assign17460_e17316_d_n8;
        locals.var_q_ln_term_dn9 = assign17460_e17316_d_n9;

        let assign17470_e17319: f64 = (-0.005);
        let assign17470_e17320: f64 = if locals.var_q_qsq < assign17470_e17319 { 1.0 } else { 0.0 };
        locals.var_guard623 = assign17470_e17320;

        let (assign17480_e17330, assign17480_e17330_d_n4, assign17480_e17330_d_n6, assign17480_e17330_d_n7, assign17480_e17330_d_n8, assign17480_e17330_d_n9,) = {
    if ((locals.var_guard622 == 0.0) && (locals.var_guard623 != 0.0)) {
        let assign17480_e17327: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign17480_e17328: f64 = (assign17480_e17327).sin();
        (assign17480_e17328, ((assign17480_e17327).cos() * (0.5 * locals.var_q_rac_qsq_dn4)), ((assign17480_e17327).cos() * (0.5 * locals.var_q_rac_qsq_dn6)), ((assign17480_e17327).cos() * (0.5 * locals.var_q_rac_qsq_dn7)), ((assign17480_e17327).cos() * (0.5 * locals.var_q_rac_qsq_dn8)), ((assign17480_e17327).cos() * (0.5 * locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign17480_e17330;
        locals.var_q_temp2_dn4 = assign17480_e17330_d_n4;
        locals.var_q_temp2_dn6 = assign17480_e17330_d_n6;
        locals.var_q_temp2_dn7 = assign17480_e17330_d_n7;
        locals.var_q_temp2_dn8 = assign17480_e17330_d_n8;
        locals.var_q_temp2_dn9 = assign17480_e17330_d_n9;

        let (assign17490_e17342, assign17490_e17342_d_n4, assign17490_e17342_d_n6, assign17490_e17342_d_n7, assign17490_e17342_d_n8, assign17490_e17342_d_n9,) = {
    if ((locals.var_guard622 == 0.0) && (locals.var_guard623 != 0.0)) {
        let assign17490_e17336: f64 = (-locals.var_q_qsq);
        let assign17490_e17339: f64 = (locals.var_q_temp2 * locals.var_q_temp2);
        let assign17490_e17340: f64 = (assign17490_e17336 / assign17490_e17339);
        (assign17490_e17340, ((((-locals.var_q_qsq_dn4) * assign17490_e17339) - (assign17490_e17336 * ((locals.var_q_temp2_dn4 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn4)))) / (assign17490_e17339 * assign17490_e17339)), ((((-locals.var_q_qsq_dn6) * assign17490_e17339) - (assign17490_e17336 * ((locals.var_q_temp2_dn6 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn6)))) / (assign17490_e17339 * assign17490_e17339)), ((((-locals.var_q_qsq_dn7) * assign17490_e17339) - (assign17490_e17336 * ((locals.var_q_temp2_dn7 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn7)))) / (assign17490_e17339 * assign17490_e17339)), ((((-locals.var_q_qsq_dn8) * assign17490_e17339) - (assign17490_e17336 * ((locals.var_q_temp2_dn8 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn8)))) / (assign17490_e17339 * assign17490_e17339)), ((((-locals.var_q_qsq_dn9) * assign17490_e17339) - (assign17490_e17336 * ((locals.var_q_temp2_dn9 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn9)))) / (assign17490_e17339 * assign17490_e17339)),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign17490_e17342;
        locals.var_q_sh_term_dn4 = assign17490_e17342_d_n4;
        locals.var_q_sh_term_dn6 = assign17490_e17342_d_n6;
        locals.var_q_sh_term_dn7 = assign17490_e17342_d_n7;
        locals.var_q_sh_term_dn8 = assign17490_e17342_d_n8;
        locals.var_q_sh_term_dn9 = assign17490_e17342_d_n9;

        let (assign17500_e17350, assign17500_e17350_d_n4, assign17500_e17350_d_n6, assign17500_e17350_d_n7, assign17500_e17350_d_n8, assign17500_e17350_d_n9,) = {
    if ((locals.var_guard622 == 0.0) && (locals.var_guard623 != 0.0)) {
        let assign17500_e17348: f64 = (locals.var_q_sh_term).ln();
        (assign17500_e17348, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign17500_e17350;
        locals.var_q_ln_term_dn4 = assign17500_e17350_d_n4;
        locals.var_q_ln_term_dn6 = assign17500_e17350_d_n6;
        locals.var_q_ln_term_dn7 = assign17500_e17350_d_n7;
        locals.var_q_ln_term_dn8 = assign17500_e17350_d_n8;
        locals.var_q_ln_term_dn9 = assign17500_e17350_d_n9;

        let (assign17510_e17374, assign17510_e17374_d_n4, assign17510_e17374_d_n6, assign17510_e17374_d_n7, assign17510_e17374_d_n8, assign17510_e17374_d_n9,) = {
    if ((locals.var_guard622 == 0.0) && (locals.var_guard623 == 0.0)) {
        let assign17510_e17359: f64 = (locals.var_q_qsq * 0.3333333333333);
        let assign17510_e17363: f64 = (0.05 * locals.var_q_qsq);
        let assign17510_e17367: f64 = (0.0396825396825397 * locals.var_q_qsq);
        let assign17510_e17368: f64 = (1.0 - assign17510_e17367);
        let assign17510_e17369: f64 = (assign17510_e17363 * assign17510_e17368);
        let assign17510_e17370: f64 = (1.0 - assign17510_e17369);
        let assign17510_e17371: f64 = (assign17510_e17359 * assign17510_e17370);
        let assign17510_e17372: f64 = (4.0 - assign17510_e17371);
        (assign17510_e17372, (-(((locals.var_q_qsq_dn4 * 0.3333333333333) * assign17510_e17370) + (assign17510_e17359 * (-(((0.05 * locals.var_q_qsq_dn4) * assign17510_e17368) + (assign17510_e17363 * (-(0.0396825396825397 * locals.var_q_qsq_dn4)))))))), (-(((locals.var_q_qsq_dn6 * 0.3333333333333) * assign17510_e17370) + (assign17510_e17359 * (-(((0.05 * locals.var_q_qsq_dn6) * assign17510_e17368) + (assign17510_e17363 * (-(0.0396825396825397 * locals.var_q_qsq_dn6)))))))), (-(((locals.var_q_qsq_dn7 * 0.3333333333333) * assign17510_e17370) + (assign17510_e17359 * (-(((0.05 * locals.var_q_qsq_dn7) * assign17510_e17368) + (assign17510_e17363 * (-(0.0396825396825397 * locals.var_q_qsq_dn7)))))))), (-(((locals.var_q_qsq_dn8 * 0.3333333333333) * assign17510_e17370) + (assign17510_e17359 * (-(((0.05 * locals.var_q_qsq_dn8) * assign17510_e17368) + (assign17510_e17363 * (-(0.0396825396825397 * locals.var_q_qsq_dn8)))))))), (-(((locals.var_q_qsq_dn9 * 0.3333333333333) * assign17510_e17370) + (assign17510_e17359 * (-(((0.05 * locals.var_q_qsq_dn9) * assign17510_e17368) + (assign17510_e17363 * (-(0.0396825396825397 * locals.var_q_qsq_dn9)))))))),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign17510_e17374;
        locals.var_q_sh_term_dn4 = assign17510_e17374_d_n4;
        locals.var_q_sh_term_dn6 = assign17510_e17374_d_n6;
        locals.var_q_sh_term_dn7 = assign17510_e17374_d_n7;
        locals.var_q_sh_term_dn8 = assign17510_e17374_d_n8;
        locals.var_q_sh_term_dn9 = assign17510_e17374_d_n9;

    }

    pub(super) fn stamp_transient_block_44(
        locals: &mut StampLocals,
    ) {
        let (assign17520_e17383, assign17520_e17383_d_n4, assign17520_e17383_d_n6, assign17520_e17383_d_n7, assign17520_e17383_d_n8, assign17520_e17383_d_n9,) = {
    if ((locals.var_guard622 == 0.0) && (locals.var_guard623 == 0.0)) {
        let assign17520_e17381: f64 = (locals.var_q_sh_term).ln();
        (assign17520_e17381, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign17520_e17383;
        locals.var_q_ln_term_dn4 = assign17520_e17383_d_n4;
        locals.var_q_ln_term_dn6 = assign17520_e17383_d_n6;
        locals.var_q_ln_term_dn7 = assign17520_e17383_d_n7;
        locals.var_q_ln_term_dn8 = assign17520_e17383_d_n8;
        locals.var_q_ln_term_dn9 = assign17520_e17383_d_n9;

        let assign17530_e17386: f64 = (1.01 * locals.var_q_k1q1);
        let assign17530_e17388: f64 = (assign17530_e17386 + locals.var_q_qcoth);
        let assign17530_e17390: f64 = if assign17530_e17388 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard624 = assign17530_e17390;

        let (assign17540_e17396, assign17540_e17396_d_n4, assign17540_e17396_d_n6, assign17540_e17396_d_n7, assign17540_e17396_d_n8, assign17540_e17396_d_n9,) = {
    if (locals.var_guard624 != 0.0) {
        let assign17540_e17394: f64 = (locals.var_q_k1q1 + locals.var_q_qcoth);
        (assign17540_e17394, (locals.var_q_k1q1_dn4 + locals.var_q_qcoth_dn4), (locals.var_q_k1q1_dn6 + locals.var_q_qcoth_dn6), (locals.var_q_k1q1_dn7 + locals.var_q_qcoth_dn7), (locals.var_q_k1q1_dn8 + locals.var_q_qcoth_dn8), (locals.var_q_k1q1_dn9 + locals.var_q_qcoth_dn9),)
    } else {
        (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9,)
    }
};
        locals.var_q_expnum = assign17540_e17396;
        locals.var_q_expnum_dn4 = assign17540_e17396_d_n4;
        locals.var_q_expnum_dn6 = assign17540_e17396_d_n6;
        locals.var_q_expnum_dn7 = assign17540_e17396_d_n7;
        locals.var_q_expnum_dn8 = assign17540_e17396_d_n8;
        locals.var_q_expnum_dn9 = assign17540_e17396_d_n9;

        let (assign17550_e17402, assign17550_e17402_d_n4, assign17550_e17402_d_n6, assign17550_e17402_d_n7, assign17550_e17402_d_n8, assign17550_e17402_d_n9,) = {
    if (locals.var_guard624 != 0.0) {
        let assign17550_e17400: f64 = (locals.var_k1 + locals.var_q_d1_qcoth);
        (assign17550_e17400, (locals.var_k1_dn4 + locals.var_q_d1_qcoth_dn4), (locals.var_k1_dn6 + locals.var_q_d1_qcoth_dn6), (locals.var_k1_dn7 + locals.var_q_d1_qcoth_dn7), (locals.var_k1_dn8 + locals.var_q_d1_qcoth_dn8), (locals.var_k1_dn9 + locals.var_q_d1_qcoth_dn9),)
    } else {
        (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9,)
    }
};
        locals.var_q_d1_expnum = assign17550_e17402;
        locals.var_q_d1_expnum_dn4 = assign17550_e17402_d_n4;
        locals.var_q_d1_expnum_dn6 = assign17550_e17402_d_n6;
        locals.var_q_d1_expnum_dn7 = assign17550_e17402_d_n7;
        locals.var_q_d1_expnum_dn8 = assign17550_e17402_d_n8;
        locals.var_q_d1_expnum_dn9 = assign17550_e17402_d_n9;

        let (assign17560_e17406, assign17560_e17406_d_n4, assign17560_e17406_d_n6, assign17560_e17406_d_n7, assign17560_e17406_d_n8, assign17560_e17406_d_n9,) = {
    if (locals.var_guard624 != 0.0) {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    } else {
        (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9,)
    }
};
        locals.var_q_d2_expnum = assign17560_e17406;
        locals.var_q_d2_expnum_dn4 = assign17560_e17406_d_n4;
        locals.var_q_d2_expnum_dn6 = assign17560_e17406_d_n6;
        locals.var_q_d2_expnum_dn7 = assign17560_e17406_d_n7;
        locals.var_q_d2_expnum_dn8 = assign17560_e17406_d_n8;
        locals.var_q_d2_expnum_dn9 = assign17560_e17406_d_n9;

        let (assign17570_e17415, assign17570_e17415_d_n4, assign17570_e17415_d_n6, assign17570_e17415_d_n7, assign17570_e17415_d_n8, assign17570_e17415_d_n9,) = {
    if (locals.var_guard624 == 0.0) {
        let assign17570_e17412: f64 = (locals.var_q_k1q1 - locals.var_q_qcoth);
        let assign17570_e17413: f64 = (1.0 / assign17570_e17412);
        (assign17570_e17413, (-((locals.var_q_k1q1_dn4 - locals.var_q_qcoth_dn4) / (assign17570_e17412 * assign17570_e17412))), (-((locals.var_q_k1q1_dn6 - locals.var_q_qcoth_dn6) / (assign17570_e17412 * assign17570_e17412))), (-((locals.var_q_k1q1_dn7 - locals.var_q_qcoth_dn7) / (assign17570_e17412 * assign17570_e17412))), (-((locals.var_q_k1q1_dn8 - locals.var_q_qcoth_dn8) / (assign17570_e17412 * assign17570_e17412))), (-((locals.var_q_k1q1_dn9 - locals.var_q_qcoth_dn9) / (assign17570_e17412 * assign17570_e17412))),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign17570_e17415;
        locals.var_q_temp2_dn4 = assign17570_e17415_d_n4;
        locals.var_q_temp2_dn6 = assign17570_e17415_d_n6;
        locals.var_q_temp2_dn7 = assign17570_e17415_d_n7;
        locals.var_q_temp2_dn8 = assign17570_e17415_d_n8;
        locals.var_q_temp2_dn9 = assign17570_e17415_d_n9;

        let (assign17580_e17422, assign17580_e17422_d_n4, assign17580_e17422_d_n6, assign17580_e17422_d_n7, assign17580_e17422_d_n8, assign17580_e17422_d_n9,) = {
    if (locals.var_guard624 == 0.0) {
        let assign17580_e17420: f64 = (locals.var_q_d1_qcoth - locals.var_k1);
        (assign17580_e17420, (locals.var_q_d1_qcoth_dn4 - locals.var_k1_dn4), (locals.var_q_d1_qcoth_dn6 - locals.var_k1_dn6), (locals.var_q_d1_qcoth_dn7 - locals.var_k1_dn7), (locals.var_q_d1_qcoth_dn8 - locals.var_k1_dn8), (locals.var_q_d1_qcoth_dn9 - locals.var_k1_dn9),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign17580_e17422;
        locals.var_q_temp3_dn4 = assign17580_e17422_d_n4;
        locals.var_q_temp3_dn6 = assign17580_e17422_d_n6;
        locals.var_q_temp3_dn7 = assign17580_e17422_d_n7;
        locals.var_q_temp3_dn8 = assign17580_e17422_d_n8;
        locals.var_q_temp3_dn9 = assign17580_e17422_d_n9;

        let (assign17590_e17431, assign17590_e17431_d_n4, assign17590_e17431_d_n6, assign17590_e17431_d_n7, assign17590_e17431_d_n8, assign17590_e17431_d_n9,) = {
    if (locals.var_guard624 == 0.0) {
        let assign17590_e17427: f64 = (locals.var_q_aexp - locals.var_q_sh_term);
        let assign17590_e17429: f64 = (assign17590_e17427 * locals.var_q_temp2);
        (assign17590_e17429, (((locals.var_q_aexp_dn4 - locals.var_q_sh_term_dn4) * locals.var_q_temp2) + (assign17590_e17427 * locals.var_q_temp2_dn4)), (((locals.var_q_aexp_dn6 - locals.var_q_sh_term_dn6) * locals.var_q_temp2) + (assign17590_e17427 * locals.var_q_temp2_dn6)), (((locals.var_q_aexp_dn7 - locals.var_q_sh_term_dn7) * locals.var_q_temp2) + (assign17590_e17427 * locals.var_q_temp2_dn7)), (((locals.var_q_aexp_dn8 - locals.var_q_sh_term_dn8) * locals.var_q_temp2) + (assign17590_e17427 * locals.var_q_temp2_dn8)), (((locals.var_q_aexp_dn9 - locals.var_q_sh_term_dn9) * locals.var_q_temp2) + (assign17590_e17427 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9,)
    }
};
        locals.var_q_expnum = assign17590_e17431;
        locals.var_q_expnum_dn4 = assign17590_e17431_d_n4;
        locals.var_q_expnum_dn6 = assign17590_e17431_d_n6;
        locals.var_q_expnum_dn7 = assign17590_e17431_d_n7;
        locals.var_q_expnum_dn8 = assign17590_e17431_d_n8;
        locals.var_q_expnum_dn9 = assign17590_e17431_d_n9;

        let (assign17600_e17446, assign17600_e17446_d_n4, assign17600_e17446_d_n6, assign17600_e17446_d_n7, assign17600_e17446_d_n8, assign17600_e17446_d_n9,) = {
    if (locals.var_guard624 == 0.0) {
        let assign17600_e17436: f64 = (locals.var_q_temp3 * locals.var_q_expnum);
        let assign17600_e17438: f64 = (assign17600_e17436 - locals.var_q_aexp);
        let assign17600_e17441: f64 = (locals.var_q_d1_ln * locals.var_q_sh_term);
        let assign17600_e17442: f64 = (assign17600_e17438 - assign17600_e17441);
        let assign17600_e17444: f64 = (assign17600_e17442 * locals.var_q_temp2);
        (assign17600_e17444, ((((((locals.var_q_temp3_dn4 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4) - ((locals.var_q_d1_ln_dn4 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign17600_e17442 * locals.var_q_temp2_dn4)), ((((((locals.var_q_temp3_dn6 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6) - ((locals.var_q_d1_ln_dn6 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign17600_e17442 * locals.var_q_temp2_dn6)), ((((((locals.var_q_temp3_dn7 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7) - ((locals.var_q_d1_ln_dn7 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign17600_e17442 * locals.var_q_temp2_dn7)), ((((((locals.var_q_temp3_dn8 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8) - ((locals.var_q_d1_ln_dn8 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign17600_e17442 * locals.var_q_temp2_dn8)), ((((((locals.var_q_temp3_dn9 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9) - ((locals.var_q_d1_ln_dn9 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign17600_e17442 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9,)
    }
};
        locals.var_q_d1_expnum = assign17600_e17446;
        locals.var_q_d1_expnum_dn4 = assign17600_e17446_d_n4;
        locals.var_q_d1_expnum_dn6 = assign17600_e17446_d_n6;
        locals.var_q_d1_expnum_dn7 = assign17600_e17446_d_n7;
        locals.var_q_d1_expnum_dn8 = assign17600_e17446_d_n8;
        locals.var_q_d1_expnum_dn9 = assign17600_e17446_d_n9;

        let (assign17610_e17471, assign17610_e17471_d_n4, assign17610_e17471_d_n6, assign17610_e17471_d_n7, assign17610_e17471_d_n8, assign17610_e17471_d_n9,) = {
    if (locals.var_guard624 == 0.0) {
        let assign17610_e17451: f64 = (locals.var_q_d2_qcoth * locals.var_q_expnum);
        let assign17610_e17454: f64 = (2.0 * locals.var_q_temp3);
        let assign17610_e17456: f64 = (assign17610_e17454 * locals.var_q_d1_expnum);
        let assign17610_e17457: f64 = (assign17610_e17451 + assign17610_e17456);
        let assign17610_e17459: f64 = (assign17610_e17457 + locals.var_q_aexp);
        let assign17610_e17463: f64 = (locals.var_q_d1_ln * locals.var_q_d1_ln);
        let assign17610_e17464: f64 = (locals.var_q_d2_ln + assign17610_e17463);
        let assign17610_e17466: f64 = (assign17610_e17464 * locals.var_q_sh_term);
        let assign17610_e17467: f64 = (assign17610_e17459 - assign17610_e17466);
        let assign17610_e17469: f64 = (assign17610_e17467 * locals.var_q_temp2);
        (assign17610_e17469, (((((((locals.var_q_d2_qcoth_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_temp3_dn4) * locals.var_q_d1_expnum) + (assign17610_e17454 * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4) - (((locals.var_q_d2_ln_dn4 + ((locals.var_q_d1_ln_dn4 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn4))) * locals.var_q_sh_term) + (assign17610_e17464 * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign17610_e17467 * locals.var_q_temp2_dn4)), (((((((locals.var_q_d2_qcoth_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_temp3_dn6) * locals.var_q_d1_expnum) + (assign17610_e17454 * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6) - (((locals.var_q_d2_ln_dn6 + ((locals.var_q_d1_ln_dn6 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn6))) * locals.var_q_sh_term) + (assign17610_e17464 * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign17610_e17467 * locals.var_q_temp2_dn6)), (((((((locals.var_q_d2_qcoth_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_temp3_dn7) * locals.var_q_d1_expnum) + (assign17610_e17454 * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7) - (((locals.var_q_d2_ln_dn7 + ((locals.var_q_d1_ln_dn7 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn7))) * locals.var_q_sh_term) + (assign17610_e17464 * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign17610_e17467 * locals.var_q_temp2_dn7)), (((((((locals.var_q_d2_qcoth_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_temp3_dn8) * locals.var_q_d1_expnum) + (assign17610_e17454 * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8) - (((locals.var_q_d2_ln_dn8 + ((locals.var_q_d1_ln_dn8 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn8))) * locals.var_q_sh_term) + (assign17610_e17464 * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign17610_e17467 * locals.var_q_temp2_dn8)), (((((((locals.var_q_d2_qcoth_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_temp3_dn9) * locals.var_q_d1_expnum) + (assign17610_e17454 * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9) - (((locals.var_q_d2_ln_dn9 + ((locals.var_q_d1_ln_dn9 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn9))) * locals.var_q_sh_term) + (assign17610_e17464 * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign17610_e17467 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9,)
    }
};
        locals.var_q_d2_expnum = assign17610_e17471;
        locals.var_q_d2_expnum_dn4 = assign17610_e17471_d_n4;
        locals.var_q_d2_expnum_dn6 = assign17610_e17471_d_n6;
        locals.var_q_d2_expnum_dn7 = assign17610_e17471_d_n7;
        locals.var_q_d2_expnum_dn8 = assign17610_e17471_d_n8;
        locals.var_q_d2_expnum_dn9 = assign17610_e17471_d_n9;

        let assign17620_e17474: f64 = if locals.var_q_expnum > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard625 = assign17620_e17474;

        let (assign17630_e17479, assign17630_e17479_d_n4, assign17630_e17479_d_n6, assign17630_e17479_d_n7, assign17630_e17479_d_n8, assign17630_e17479_d_n9,) = {
    if (locals.var_guard625 != 0.0) {
        let assign17630_e17477: f64 = (locals.var_q_expnum).ln();
        (assign17630_e17477, (locals.var_q_expnum_dn4 / locals.var_q_expnum), (locals.var_q_expnum_dn6 / locals.var_q_expnum), (locals.var_q_expnum_dn7 / locals.var_q_expnum), (locals.var_q_expnum_dn8 / locals.var_q_expnum), (locals.var_q_expnum_dn9 / locals.var_q_expnum),)
    } else {
        (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9,)
    }
};
        locals.var_q_lnexpnum = assign17630_e17479;
        locals.var_q_lnexpnum_dn4 = assign17630_e17479_d_n4;
        locals.var_q_lnexpnum_dn6 = assign17630_e17479_d_n6;
        locals.var_q_lnexpnum_dn7 = assign17630_e17479_d_n7;
        locals.var_q_lnexpnum_dn8 = assign17630_e17479_d_n8;
        locals.var_q_lnexpnum_dn9 = assign17630_e17479_d_n9;

        let (assign17640_e17485, assign17640_e17485_d_n4, assign17640_e17485_d_n6, assign17640_e17485_d_n7, assign17640_e17485_d_n8, assign17640_e17485_d_n9,) = {
    if (locals.var_guard625 != 0.0) {
        let assign17640_e17483: f64 = (1.0 / locals.var_q_expnum);
        (assign17640_e17483, (-(locals.var_q_expnum_dn4 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn6 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn7 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn8 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn9 / (locals.var_q_expnum * locals.var_q_expnum))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign17640_e17485;
        locals.var_q_temp1_dn4 = assign17640_e17485_d_n4;
        locals.var_q_temp1_dn6 = assign17640_e17485_d_n6;
        locals.var_q_temp1_dn7 = assign17640_e17485_d_n7;
        locals.var_q_temp1_dn8 = assign17640_e17485_d_n8;
        locals.var_q_temp1_dn9 = assign17640_e17485_d_n9;

        let (assign17650_e17491, assign17650_e17491_d_n4, assign17650_e17491_d_n6, assign17650_e17491_d_n7, assign17650_e17491_d_n8, assign17650_e17491_d_n9,) = {
    if (locals.var_guard625 != 0.0) {
        let assign17650_e17489: f64 = (locals.var_q_d1_expnum * locals.var_q_temp1);
        (assign17650_e17489, ((locals.var_q_d1_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn4)), ((locals.var_q_d1_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn6)), ((locals.var_q_d1_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn7)), ((locals.var_q_d1_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn8)), ((locals.var_q_d1_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9,)
    }
};
        locals.var_q_d1_lnexpnum = assign17650_e17491;
        locals.var_q_d1_lnexpnum_dn4 = assign17650_e17491_d_n4;
        locals.var_q_d1_lnexpnum_dn6 = assign17650_e17491_d_n6;
        locals.var_q_d1_lnexpnum_dn7 = assign17650_e17491_d_n7;
        locals.var_q_d1_lnexpnum_dn8 = assign17650_e17491_d_n8;
        locals.var_q_d1_lnexpnum_dn9 = assign17650_e17491_d_n9;

        let (assign17660_e17501, assign17660_e17501_d_n4, assign17660_e17501_d_n6, assign17660_e17501_d_n7, assign17660_e17501_d_n8, assign17660_e17501_d_n9,) = {
    if (locals.var_guard625 != 0.0) {
        let assign17660_e17495: f64 = (locals.var_q_d2_expnum * locals.var_q_temp1);
        let assign17660_e17498: f64 = (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum);
        let assign17660_e17499: f64 = (assign17660_e17495 - assign17660_e17498);
        (assign17660_e17499, (((locals.var_q_d2_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn4)) - ((locals.var_q_d1_lnexpnum_dn4 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn4))), (((locals.var_q_d2_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn6)) - ((locals.var_q_d1_lnexpnum_dn6 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn6))), (((locals.var_q_d2_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn7)) - ((locals.var_q_d1_lnexpnum_dn7 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn7))), (((locals.var_q_d2_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn8)) - ((locals.var_q_d1_lnexpnum_dn8 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn8))), (((locals.var_q_d2_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn9)) - ((locals.var_q_d1_lnexpnum_dn9 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn9))),)
    } else {
        (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9,)
    }
};
        locals.var_q_d2_lnexpnum = assign17660_e17501;
        locals.var_q_d2_lnexpnum_dn4 = assign17660_e17501_d_n4;
        locals.var_q_d2_lnexpnum_dn6 = assign17660_e17501_d_n6;
        locals.var_q_d2_lnexpnum_dn7 = assign17660_e17501_d_n7;
        locals.var_q_d2_lnexpnum_dn8 = assign17660_e17501_d_n8;
        locals.var_q_d2_lnexpnum_dn9 = assign17660_e17501_d_n9;

        let (assign17670_e17512, assign17670_e17512_d_n4, assign17670_e17512_d_n6, assign17670_e17512_d_n7, assign17670_e17512_d_n8, assign17670_e17512_d_n9,) = {
    if (locals.var_guard625 == 0.0) {
        let assign17670_e17506: f64 = (locals.var_q_k1q1 + 0.6931471805599);
        let assign17670_e17508: f64 = (-locals.var_q_k1q1);
        let assign17670_e17509: f64 = (assign17670_e17508).ln();
        let assign17670_e17510: f64 = (assign17670_e17506 + assign17670_e17509);
        (assign17670_e17510, (locals.var_q_k1q1_dn4 + ((-locals.var_q_k1q1_dn4) / assign17670_e17508)), (locals.var_q_k1q1_dn6 + ((-locals.var_q_k1q1_dn6) / assign17670_e17508)), (locals.var_q_k1q1_dn7 + ((-locals.var_q_k1q1_dn7) / assign17670_e17508)), (locals.var_q_k1q1_dn8 + ((-locals.var_q_k1q1_dn8) / assign17670_e17508)), (locals.var_q_k1q1_dn9 + ((-locals.var_q_k1q1_dn9) / assign17670_e17508)),)
    } else {
        (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9,)
    }
};
        locals.var_q_lnexpnum = assign17670_e17512;
        locals.var_q_lnexpnum_dn4 = assign17670_e17512_d_n4;
        locals.var_q_lnexpnum_dn6 = assign17670_e17512_d_n6;
        locals.var_q_lnexpnum_dn7 = assign17670_e17512_d_n7;
        locals.var_q_lnexpnum_dn8 = assign17670_e17512_d_n8;
        locals.var_q_lnexpnum_dn9 = assign17670_e17512_d_n9;

        let (assign17680_e17519, assign17680_e17519_d_n4, assign17680_e17519_d_n6, assign17680_e17519_d_n7, assign17680_e17519_d_n8, assign17680_e17519_d_n9,) = {
    if (locals.var_guard625 == 0.0) {
        let assign17680_e17517: f64 = (1.0 / locals.var_q1d);
        (assign17680_e17517, (-(locals.var_q1d_dn4 / (locals.var_q1d * locals.var_q1d))), (-(locals.var_q1d_dn6 / (locals.var_q1d * locals.var_q1d))), (-(locals.var_q1d_dn7 / (locals.var_q1d * locals.var_q1d))), (-(locals.var_q1d_dn8 / (locals.var_q1d * locals.var_q1d))), (-(locals.var_q1d_dn9 / (locals.var_q1d * locals.var_q1d))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign17680_e17519;
        locals.var_q_temp1_dn4 = assign17680_e17519_d_n4;
        locals.var_q_temp1_dn6 = assign17680_e17519_d_n6;
        locals.var_q_temp1_dn7 = assign17680_e17519_d_n7;
        locals.var_q_temp1_dn8 = assign17680_e17519_d_n8;
        locals.var_q_temp1_dn9 = assign17680_e17519_d_n9;

        let (assign17690_e17526, assign17690_e17526_d_n4, assign17690_e17526_d_n6, assign17690_e17526_d_n7, assign17690_e17526_d_n8, assign17690_e17526_d_n9,) = {
    if (locals.var_guard625 == 0.0) {
        let assign17690_e17524: f64 = (locals.var_k1 + locals.var_q_temp1);
        (assign17690_e17524, (locals.var_k1_dn4 + locals.var_q_temp1_dn4), (locals.var_k1_dn6 + locals.var_q_temp1_dn6), (locals.var_k1_dn7 + locals.var_q_temp1_dn7), (locals.var_k1_dn8 + locals.var_q_temp1_dn8), (locals.var_k1_dn9 + locals.var_q_temp1_dn9),)
    } else {
        (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9,)
    }
};
        locals.var_q_d1_lnexpnum = assign17690_e17526;
        locals.var_q_d1_lnexpnum_dn4 = assign17690_e17526_d_n4;
        locals.var_q_d1_lnexpnum_dn6 = assign17690_e17526_d_n6;
        locals.var_q_d1_lnexpnum_dn7 = assign17690_e17526_d_n7;
        locals.var_q_d1_lnexpnum_dn8 = assign17690_e17526_d_n8;
        locals.var_q_d1_lnexpnum_dn9 = assign17690_e17526_d_n9;

        let (assign17700_e17534, assign17700_e17534_d_n4, assign17700_e17534_d_n6, assign17700_e17534_d_n7, assign17700_e17534_d_n8, assign17700_e17534_d_n9,) = {
    if (locals.var_guard625 == 0.0) {
        let assign17700_e17530: f64 = (-locals.var_q_temp1);
        let assign17700_e17532: f64 = (assign17700_e17530 * locals.var_q_temp1);
        (assign17700_e17532, (((-locals.var_q_temp1_dn4) * locals.var_q_temp1) + (assign17700_e17530 * locals.var_q_temp1_dn4)), (((-locals.var_q_temp1_dn6) * locals.var_q_temp1) + (assign17700_e17530 * locals.var_q_temp1_dn6)), (((-locals.var_q_temp1_dn7) * locals.var_q_temp1) + (assign17700_e17530 * locals.var_q_temp1_dn7)), (((-locals.var_q_temp1_dn8) * locals.var_q_temp1) + (assign17700_e17530 * locals.var_q_temp1_dn8)), (((-locals.var_q_temp1_dn9) * locals.var_q_temp1) + (assign17700_e17530 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9,)
    }
};
        locals.var_q_d2_lnexpnum = assign17700_e17534;
        locals.var_q_d2_lnexpnum_dn4 = assign17700_e17534_d_n4;
        locals.var_q_d2_lnexpnum_dn6 = assign17700_e17534_d_n6;
        locals.var_q_d2_lnexpnum_dn7 = assign17700_e17534_d_n7;
        locals.var_q_d2_lnexpnum_dn8 = assign17700_e17534_d_n8;
        locals.var_q_d2_lnexpnum_dn9 = assign17700_e17534_d_n9;

        let assign17710_e17537: f64 = (locals.var_xg2x - locals.var_xg1x);
        let assign17710_e17539: f64 = (assign17710_e17537 + locals.var_q1d);
        let assign17710_e17542: f64 = (2.0 * locals.var_q_lnexpnum);
        let assign17710_e17543: f64 = (assign17710_e17539 + assign17710_e17542);
        let assign17710_e17545: f64 = (assign17710_e17543 - locals.var_q_ln_term);
        locals.var_q_q2_int = assign17710_e17545;
        locals.var_q_q2_int_dn4 = ((((locals.var_xg2x_dn4 - locals.var_xg1x_dn4) + locals.var_q1d_dn4) + (2.0 * locals.var_q_lnexpnum_dn4)) - locals.var_q_ln_term_dn4);
        locals.var_q_q2_int_dn6 = ((((locals.var_xg2x_dn6 - locals.var_xg1x_dn6) + locals.var_q1d_dn6) + (2.0 * locals.var_q_lnexpnum_dn6)) - locals.var_q_ln_term_dn6);
        locals.var_q_q2_int_dn7 = ((((locals.var_xg2x_dn7 - locals.var_xg1x_dn7) + locals.var_q1d_dn7) + (2.0 * locals.var_q_lnexpnum_dn7)) - locals.var_q_ln_term_dn7);
        locals.var_q_q2_int_dn8 = ((((locals.var_xg2x_dn8 - locals.var_xg1x_dn8) + locals.var_q1d_dn8) + (2.0 * locals.var_q_lnexpnum_dn8)) - locals.var_q_ln_term_dn8);
        locals.var_q_q2_int_dn9 = ((((locals.var_xg2x_dn9 - locals.var_xg1x_dn9) + locals.var_q1d_dn9) + (2.0 * locals.var_q_lnexpnum_dn9)) - locals.var_q_ln_term_dn9);

        let assign17720_e17549: f64 = (2.0 * locals.var_q_d1_lnexpnum);
        let assign17720_e17550: f64 = (1.0 + assign17720_e17549);
        let assign17720_e17552: f64 = (assign17720_e17550 - locals.var_q_d1_ln);
        locals.var_q_d1_q2 = assign17720_e17552;
        locals.var_q_d1_q2_dn4 = ((2.0 * locals.var_q_d1_lnexpnum_dn4) - locals.var_q_d1_ln_dn4);
        locals.var_q_d1_q2_dn6 = ((2.0 * locals.var_q_d1_lnexpnum_dn6) - locals.var_q_d1_ln_dn6);
        locals.var_q_d1_q2_dn7 = ((2.0 * locals.var_q_d1_lnexpnum_dn7) - locals.var_q_d1_ln_dn7);
        locals.var_q_d1_q2_dn8 = ((2.0 * locals.var_q_d1_lnexpnum_dn8) - locals.var_q_d1_ln_dn8);
        locals.var_q_d1_q2_dn9 = ((2.0 * locals.var_q_d1_lnexpnum_dn9) - locals.var_q_d1_ln_dn9);

        let assign17730_e17555: f64 = (2.0 * locals.var_q_d2_lnexpnum);
        let assign17730_e17557: f64 = (assign17730_e17555 - locals.var_q_d2_ln);
        locals.var_q_d2_q2 = assign17730_e17557;
        locals.var_q_d2_q2_dn4 = ((2.0 * locals.var_q_d2_lnexpnum_dn4) - locals.var_q_d2_ln_dn4);
        locals.var_q_d2_q2_dn6 = ((2.0 * locals.var_q_d2_lnexpnum_dn6) - locals.var_q_d2_ln_dn6);
        locals.var_q_d2_q2_dn7 = ((2.0 * locals.var_q_d2_lnexpnum_dn7) - locals.var_q_d2_ln_dn7);
        locals.var_q_d2_q2_dn8 = ((2.0 * locals.var_q_d2_lnexpnum_dn8) - locals.var_q_d2_ln_dn8);
        locals.var_q_d2_q2_dn9 = ((2.0 * locals.var_q_d2_lnexpnum_dn9) - locals.var_q_d2_ln_dn9);

        let assign17740_e17561: f64 = (locals.var_k2 * locals.var_q_q2_int);
        let assign17740_e17562: f64 = (locals.var_q_k1q1 + assign17740_e17561);
        locals.var_q_qi_int = assign17740_e17562;
        locals.var_q_qi_int_dn4 = (locals.var_q_k1q1_dn4 + ((locals.var_k2_dn4 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn4)));
        locals.var_q_qi_int_dn6 = (locals.var_q_k1q1_dn6 + ((locals.var_k2_dn6 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn6)));
        locals.var_q_qi_int_dn7 = (locals.var_q_k1q1_dn7 + ((locals.var_k2_dn7 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn7)));
        locals.var_q_qi_int_dn8 = (locals.var_q_k1q1_dn8 + ((locals.var_k2_dn8 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn8)));
        locals.var_q_qi_int_dn9 = (locals.var_q_k1q1_dn9 + ((locals.var_k2_dn9 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn9)));

        let assign17750_e17566: f64 = (locals.var_k2 * locals.var_q_d1_q2);
        let assign17750_e17567: f64 = (locals.var_k1 + assign17750_e17566);
        locals.var_q_d1_qi = assign17750_e17567;
        locals.var_q_d1_qi_dn4 = (locals.var_k1_dn4 + ((locals.var_k2_dn4 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn4)));
        locals.var_q_d1_qi_dn6 = (locals.var_k1_dn6 + ((locals.var_k2_dn6 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn6)));
        locals.var_q_d1_qi_dn7 = (locals.var_k1_dn7 + ((locals.var_k2_dn7 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn7)));
        locals.var_q_d1_qi_dn8 = (locals.var_k1_dn8 + ((locals.var_k2_dn8 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn8)));
        locals.var_q_d1_qi_dn9 = (locals.var_k1_dn9 + ((locals.var_k2_dn9 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn9)));

        let assign17760_e17570: f64 = (locals.var_k2 * locals.var_q_d2_q2);
        locals.var_q_d2_qi = assign17760_e17570;
        locals.var_q_d2_qi_dn4 = ((locals.var_k2_dn4 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn4));
        locals.var_q_d2_qi_dn6 = ((locals.var_k2_dn6 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn6));
        locals.var_q_d2_qi_dn7 = ((locals.var_k2_dn7 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn7));
        locals.var_q_d2_qi_dn8 = ((locals.var_k2_dn8 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn8));
        locals.var_q_d2_qi_dn9 = ((locals.var_k2_dn9 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn9));

        let assign17770_e17573: f64 = (locals.var_q_qi_int * locals.var_q_expnum);
        let assign17770_e17575: f64 = (assign17770_e17573 - locals.var_q_aexp);
        locals.var_q_zero = assign17770_e17575;
        locals.var_q_zero_dn4 = (((locals.var_q_qi_int_dn4 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4);
        locals.var_q_zero_dn6 = (((locals.var_q_qi_int_dn6 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6);
        locals.var_q_zero_dn7 = (((locals.var_q_qi_int_dn7 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7);
        locals.var_q_zero_dn8 = (((locals.var_q_qi_int_dn8 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8);
        locals.var_q_zero_dn9 = (((locals.var_q_qi_int_dn9 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9);

        let assign17780_e17578: f64 = (locals.var_q_d1_qi * locals.var_q_expnum);
        let assign17780_e17581: f64 = (locals.var_q_qi_int * locals.var_q_d1_expnum);
        let assign17780_e17582: f64 = (assign17780_e17578 + assign17780_e17581);
        let assign17780_e17584: f64 = (assign17780_e17582 + locals.var_q_aexp);
        locals.var_q_d1_zero = assign17780_e17584;
        locals.var_q_d1_zero_dn4 = ((((locals.var_q_d1_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn4)) + ((locals.var_q_qi_int_dn4 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4);
        locals.var_q_d1_zero_dn6 = ((((locals.var_q_d1_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn6)) + ((locals.var_q_qi_int_dn6 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6);
        locals.var_q_d1_zero_dn7 = ((((locals.var_q_d1_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn7)) + ((locals.var_q_qi_int_dn7 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7);
        locals.var_q_d1_zero_dn8 = ((((locals.var_q_d1_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn8)) + ((locals.var_q_qi_int_dn8 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8);
        locals.var_q_d1_zero_dn9 = ((((locals.var_q_d1_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn9)) + ((locals.var_q_qi_int_dn9 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9);

        let assign17790_e17587: f64 = (locals.var_q_d2_qi * locals.var_q_expnum);
        let assign17790_e17590: f64 = (2.0 * locals.var_q_d1_qi);
        let assign17790_e17592: f64 = (assign17790_e17590 * locals.var_q_d1_expnum);
        let assign17790_e17593: f64 = (assign17790_e17587 + assign17790_e17592);
        let assign17790_e17596: f64 = (locals.var_q_qi_int * locals.var_q_d2_expnum);
        let assign17790_e17597: f64 = (assign17790_e17593 + assign17790_e17596);
        let assign17790_e17599: f64 = (assign17790_e17597 - locals.var_q_aexp);
        locals.var_q_d2_zero = assign17790_e17599;
        locals.var_q_d2_zero_dn4 = (((((locals.var_q_d2_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_d1_qi_dn4) * locals.var_q_d1_expnum) + (assign17790_e17590 * locals.var_q_d1_expnum_dn4))) + ((locals.var_q_qi_int_dn4 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn4))) - locals.var_q_aexp_dn4);
        locals.var_q_d2_zero_dn6 = (((((locals.var_q_d2_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_d1_qi_dn6) * locals.var_q_d1_expnum) + (assign17790_e17590 * locals.var_q_d1_expnum_dn6))) + ((locals.var_q_qi_int_dn6 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn6))) - locals.var_q_aexp_dn6);
        locals.var_q_d2_zero_dn7 = (((((locals.var_q_d2_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_d1_qi_dn7) * locals.var_q_d1_expnum) + (assign17790_e17590 * locals.var_q_d1_expnum_dn7))) + ((locals.var_q_qi_int_dn7 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn7))) - locals.var_q_aexp_dn7);
        locals.var_q_d2_zero_dn8 = (((((locals.var_q_d2_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_d1_qi_dn8) * locals.var_q_d1_expnum) + (assign17790_e17590 * locals.var_q_d1_expnum_dn8))) + ((locals.var_q_qi_int_dn8 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn8))) - locals.var_q_aexp_dn8);
        locals.var_q_d2_zero_dn9 = (((((locals.var_q_d2_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_d1_qi_dn9) * locals.var_q_d1_expnum) + (assign17790_e17590 * locals.var_q_d1_expnum_dn9))) + ((locals.var_q_qi_int_dn9 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn9))) - locals.var_q_aexp_dn9);

        let assign17800_e17602: f64 = (locals.var_q_d1_zero * locals.var_q_d1_zero);
        let assign17800_e17605: f64 = (0.5 * locals.var_q_zero);
        let assign17800_e17607: f64 = (assign17800_e17605 * locals.var_q_d2_zero);
        let assign17800_e17608: f64 = (assign17800_e17602 - assign17800_e17607);
        locals.var_q_temp = assign17800_e17608;
        locals.var_q_temp_dn4 = (((locals.var_q_d1_zero_dn4 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn4)) - (((0.5 * locals.var_q_zero_dn4) * locals.var_q_d2_zero) + (assign17800_e17605 * locals.var_q_d2_zero_dn4)));
        locals.var_q_temp_dn6 = (((locals.var_q_d1_zero_dn6 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn6)) - (((0.5 * locals.var_q_zero_dn6) * locals.var_q_d2_zero) + (assign17800_e17605 * locals.var_q_d2_zero_dn6)));
        locals.var_q_temp_dn7 = (((locals.var_q_d1_zero_dn7 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn7)) - (((0.5 * locals.var_q_zero_dn7) * locals.var_q_d2_zero) + (assign17800_e17605 * locals.var_q_d2_zero_dn7)));
        locals.var_q_temp_dn8 = (((locals.var_q_d1_zero_dn8 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn8)) - (((0.5 * locals.var_q_zero_dn8) * locals.var_q_d2_zero) + (assign17800_e17605 * locals.var_q_d2_zero_dn8)));
        locals.var_q_temp_dn9 = (((locals.var_q_d1_zero_dn9 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn9)) - (((0.5 * locals.var_q_zero_dn9) * locals.var_q_d2_zero) + (assign17800_e17605 * locals.var_q_d2_zero_dn9)));

        let assign17810_e17610: f64 = (-locals.var_q_zero);
        let assign17810_e17612: f64 = (assign17810_e17610 * locals.var_q_d1_zero);
        let assign17810_e17614: f64 = (assign17810_e17612 * locals.var_q_temp);
        let assign17810_e17617: f64 = (locals.var_q_temp * locals.var_q_temp);
        let assign17810_e17619: f64 = (assign17810_e17617 + 1e-200);
        let assign17810_e17620: f64 = (assign17810_e17614 / assign17810_e17619);
        locals.var_q_eps2 = assign17810_e17620;
        locals.var_q_eps2_dn4 = ((((((((-locals.var_q_zero_dn4) * locals.var_q_d1_zero) + (assign17810_e17610 * locals.var_q_d1_zero_dn4)) * locals.var_q_temp) + (assign17810_e17612 * locals.var_q_temp_dn4)) * assign17810_e17619) - (assign17810_e17614 * ((locals.var_q_temp_dn4 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn4)))) / (assign17810_e17619 * assign17810_e17619));
        locals.var_q_eps2_dn6 = ((((((((-locals.var_q_zero_dn6) * locals.var_q_d1_zero) + (assign17810_e17610 * locals.var_q_d1_zero_dn6)) * locals.var_q_temp) + (assign17810_e17612 * locals.var_q_temp_dn6)) * assign17810_e17619) - (assign17810_e17614 * ((locals.var_q_temp_dn6 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn6)))) / (assign17810_e17619 * assign17810_e17619));
        locals.var_q_eps2_dn7 = ((((((((-locals.var_q_zero_dn7) * locals.var_q_d1_zero) + (assign17810_e17610 * locals.var_q_d1_zero_dn7)) * locals.var_q_temp) + (assign17810_e17612 * locals.var_q_temp_dn7)) * assign17810_e17619) - (assign17810_e17614 * ((locals.var_q_temp_dn7 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn7)))) / (assign17810_e17619 * assign17810_e17619));
        locals.var_q_eps2_dn8 = ((((((((-locals.var_q_zero_dn8) * locals.var_q_d1_zero) + (assign17810_e17610 * locals.var_q_d1_zero_dn8)) * locals.var_q_temp) + (assign17810_e17612 * locals.var_q_temp_dn8)) * assign17810_e17619) - (assign17810_e17614 * ((locals.var_q_temp_dn8 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn8)))) / (assign17810_e17619 * assign17810_e17619));
        locals.var_q_eps2_dn9 = ((((((((-locals.var_q_zero_dn9) * locals.var_q_d1_zero) + (assign17810_e17610 * locals.var_q_d1_zero_dn9)) * locals.var_q_temp) + (assign17810_e17612 * locals.var_q_temp_dn9)) * assign17810_e17619) - (assign17810_e17614 * ((locals.var_q_temp_dn9 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn9)))) / (assign17810_e17619 * assign17810_e17619));

        let assign17820_e17623: f64 = (locals.var_q1d + locals.var_q_eps2);
        locals.var_q1d = assign17820_e17623;
        locals.var_q1d_dn4 = (locals.var_q1d_dn4 + locals.var_q_eps2_dn4);
        locals.var_q1d_dn6 = (locals.var_q1d_dn6 + locals.var_q_eps2_dn6);
        locals.var_q1d_dn7 = (locals.var_q1d_dn7 + locals.var_q_eps2_dn7);
        locals.var_q1d_dn8 = (locals.var_q1d_dn8 + locals.var_q_eps2_dn8);
        locals.var_q1d_dn9 = (locals.var_q1d_dn9 + locals.var_q_eps2_dn9);

        let assign17830_e17626: f64 = (locals.var_k1 * locals.var_q1d);
        locals.var_q_k1q1 = assign17830_e17626;
        locals.var_q_k1q1_dn4 = ((locals.var_k1_dn4 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn4));
        locals.var_q_k1q1_dn6 = ((locals.var_k1_dn6 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn6));
        locals.var_q_k1q1_dn7 = ((locals.var_k1_dn7 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn7));
        locals.var_q_k1q1_dn8 = ((locals.var_k1_dn8 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn8));
        locals.var_q_k1q1_dn9 = ((locals.var_k1_dn9 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn9));

        let assign17840_e17629: f64 = (locals.var_k2 * locals.var_q2d);
        locals.var_q_k2q2 = assign17840_e17629;
        locals.var_q_k2q2_dn4 = ((locals.var_k2_dn4 * locals.var_q2d) + (locals.var_k2 * locals.var_q2d_dn4));
        locals.var_q_k2q2_dn6 = ((locals.var_k2_dn6 * locals.var_q2d) + (locals.var_k2 * locals.var_q2d_dn6));
        locals.var_q_k2q2_dn7 = ((locals.var_k2_dn7 * locals.var_q2d) + (locals.var_k2 * locals.var_q2d_dn7));
        locals.var_q_k2q2_dn8 = ((locals.var_k2_dn8 * locals.var_q2d) + (locals.var_k2 * locals.var_q2d_dn8));
        locals.var_q_k2q2_dn9 = ((locals.var_k2_dn9 * locals.var_q2d) + (locals.var_k2 * locals.var_q2d_dn9));

        let assign17850_e17632: f64 = (locals.var_q_k1q1 + locals.var_q_k2q2);
        locals.var_q_qi_int = assign17850_e17632;
        locals.var_q_qi_int_dn4 = (locals.var_q_k1q1_dn4 + locals.var_q_k2q2_dn4);
        locals.var_q_qi_int_dn6 = (locals.var_q_k1q1_dn6 + locals.var_q_k2q2_dn6);
        locals.var_q_qi_int_dn7 = (locals.var_q_k1q1_dn7 + locals.var_q_k2q2_dn7);
        locals.var_q_qi_int_dn8 = (locals.var_q_k1q1_dn8 + locals.var_q_k2q2_dn8);
        locals.var_q_qi_int_dn9 = (locals.var_q_k1q1_dn9 + locals.var_q_k2q2_dn9);

        let assign17860_e17636: f64 = (0.065345483024 * locals.var_q_qi_int);
        let assign17860_e17637: f64 = (1.0 + assign17860_e17636);
        locals.var_q_a = assign17860_e17637;
        locals.var_q_a_dn4 = (0.065345483024 * locals.var_q_qi_int_dn4);
        locals.var_q_a_dn6 = (0.065345483024 * locals.var_q_qi_int_dn6);
        locals.var_q_a_dn7 = (0.065345483024 * locals.var_q_qi_int_dn7);
        locals.var_q_a_dn8 = (0.065345483024 * locals.var_q_qi_int_dn8);
        locals.var_q_a_dn9 = (0.065345483024 * locals.var_q_qi_int_dn9);

        let assign17870_e17641: f64 = (8.5797362674 * locals.var_q_qi_int);
        let assign17870_e17642: f64 = (39.478417604 + assign17870_e17641);
        let assign17870_e17645: f64 = (locals.var_q_k1q1 * locals.var_q_k2q2);
        let assign17870_e17646: f64 = (assign17870_e17642 + assign17870_e17645);
        locals.var_q_b = assign17870_e17646;
        locals.var_q_b_dn4 = ((8.5797362674 * locals.var_q_qi_int_dn4) + ((locals.var_q_k1q1_dn4 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn4)));
        locals.var_q_b_dn6 = ((8.5797362674 * locals.var_q_qi_int_dn6) + ((locals.var_q_k1q1_dn6 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn6)));
        locals.var_q_b_dn7 = ((8.5797362674 * locals.var_q_qi_int_dn7) + ((locals.var_q_k1q1_dn7 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn7)));
        locals.var_q_b_dn8 = ((8.5797362674 * locals.var_q_qi_int_dn8) + ((locals.var_q_k1q1_dn8 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn8)));
        locals.var_q_b_dn9 = ((8.5797362674 * locals.var_q_qi_int_dn9) + ((locals.var_q_k1q1_dn9 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn9)));

        let assign17880_e17650: f64 = (2.0 * locals.var_q_qi_int);
        let assign17880_e17653: f64 = (locals.var_q_k1q1 * locals.var_q_k2q2);
        let assign17880_e17654: f64 = (assign17880_e17650 + assign17880_e17653);
        let assign17880_e17655: f64 = (39.478417604 * assign17880_e17654);
        locals.var_q_c = assign17880_e17655;
        locals.var_q_c_dn4 = (39.478417604 * ((2.0 * locals.var_q_qi_int_dn4) + ((locals.var_q_k1q1_dn4 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn4))));
        locals.var_q_c_dn6 = (39.478417604 * ((2.0 * locals.var_q_qi_int_dn6) + ((locals.var_q_k1q1_dn6 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn6))));
        locals.var_q_c_dn7 = (39.478417604 * ((2.0 * locals.var_q_qi_int_dn7) + ((locals.var_q_k1q1_dn7 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn7))));
        locals.var_q_c_dn8 = (39.478417604 * ((2.0 * locals.var_q_qi_int_dn8) + ((locals.var_q_k1q1_dn8 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn8))));
        locals.var_q_c_dn9 = (39.478417604 * ((2.0 * locals.var_q_qi_int_dn9) + ((locals.var_q_k1q1_dn9 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn9))));

        let assign17890_e17658: f64 = (locals.var_q_b * locals.var_q_b);
        let assign17890_e17661: f64 = (4.0 * locals.var_q_a);
        let assign17890_e17663: f64 = (assign17890_e17661 * locals.var_q_c);
        let assign17890_e17664: f64 = (assign17890_e17658 - assign17890_e17663);
        let assign17890_e17665: f64 = (assign17890_e17664).sqrt();
        locals.var_q_disc = assign17890_e17665;
        locals.var_q_disc_dn4 = ((((locals.var_q_b_dn4 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn4)) - (((4.0 * locals.var_q_a_dn4) * locals.var_q_c) + (assign17890_e17661 * locals.var_q_c_dn4))) / (2.0 * assign17890_e17665));
        locals.var_q_disc_dn6 = ((((locals.var_q_b_dn6 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn6)) - (((4.0 * locals.var_q_a_dn6) * locals.var_q_c) + (assign17890_e17661 * locals.var_q_c_dn6))) / (2.0 * assign17890_e17665));
        locals.var_q_disc_dn7 = ((((locals.var_q_b_dn7 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn7)) - (((4.0 * locals.var_q_a_dn7) * locals.var_q_c) + (assign17890_e17661 * locals.var_q_c_dn7))) / (2.0 * assign17890_e17665));
        locals.var_q_disc_dn8 = ((((locals.var_q_b_dn8 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn8)) - (((4.0 * locals.var_q_a_dn8) * locals.var_q_c) + (assign17890_e17661 * locals.var_q_c_dn8))) / (2.0 * assign17890_e17665));
        locals.var_q_disc_dn9 = ((((locals.var_q_b_dn9 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn9)) - (((4.0 * locals.var_q_a_dn9) * locals.var_q_c) + (assign17890_e17661 * locals.var_q_c_dn9))) / (2.0 * assign17890_e17665));

        let assign17900_e17668: f64 = (locals.var_q_disc - locals.var_q_b);
        let assign17900_e17671: f64 = (2.0 * locals.var_q_a);
        let assign17900_e17672: f64 = (assign17900_e17668 / assign17900_e17671);
        locals.var_q_qsq = assign17900_e17672;
        locals.var_q_qsq_dn4 = ((((locals.var_q_disc_dn4 - locals.var_q_b_dn4) * assign17900_e17671) - (assign17900_e17668 * (2.0 * locals.var_q_a_dn4))) / (assign17900_e17671 * assign17900_e17671));
        locals.var_q_qsq_dn6 = ((((locals.var_q_disc_dn6 - locals.var_q_b_dn6) * assign17900_e17671) - (assign17900_e17668 * (2.0 * locals.var_q_a_dn6))) / (assign17900_e17671 * assign17900_e17671));
        locals.var_q_qsq_dn7 = ((((locals.var_q_disc_dn7 - locals.var_q_b_dn7) * assign17900_e17671) - (assign17900_e17668 * (2.0 * locals.var_q_a_dn7))) / (assign17900_e17671 * assign17900_e17671));
        locals.var_q_qsq_dn8 = ((((locals.var_q_disc_dn8 - locals.var_q_b_dn8) * assign17900_e17671) - (assign17900_e17668 * (2.0 * locals.var_q_a_dn8))) / (assign17900_e17671 * assign17900_e17671));
        locals.var_q_qsq_dn9 = ((((locals.var_q_disc_dn9 - locals.var_q_b_dn9) * assign17900_e17671) - (assign17900_e17668 * (2.0 * locals.var_q_a_dn9))) / (assign17900_e17671 * assign17900_e17671));

        let assign17910_e17675: f64 = (locals.var_q_k1q1 * locals.var_q_k1q1);
        let assign17910_e17677: f64 = (assign17910_e17675 - locals.var_q_qsq);
        locals.var_q_delta = assign17910_e17677;
        locals.var_q_delta_dn4 = (((locals.var_q_k1q1_dn4 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn4)) - locals.var_q_qsq_dn4);
        locals.var_q_delta_dn6 = (((locals.var_q_k1q1_dn6 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn6)) - locals.var_q_qsq_dn6);
        locals.var_q_delta_dn7 = (((locals.var_q_k1q1_dn7 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn7)) - locals.var_q_qsq_dn7);
        locals.var_q_delta_dn8 = (((locals.var_q_k1q1_dn8 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn8)) - locals.var_q_qsq_dn8);
        locals.var_q_delta_dn9 = (((locals.var_q_k1q1_dn9 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn9)) - locals.var_q_qsq_dn9);

        let assign17920_e17680: f64 = if locals.var_q_delta > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard626 = assign17920_e17680;

    }

    pub(super) fn stamp_transient_block_45(
        locals: &mut StampLocals,
    ) {
        let (assign17930_e17695, assign17930_e17695_d_n4, assign17930_e17695_d_n6, assign17930_e17695_d_n7, assign17930_e17695_d_n8, assign17930_e17695_d_n9,) = {
    if (locals.var_guard626 != 0.0) {
        let assign17930_e17685: f64 = (locals.var_q_delta / locals.var_a0);
        let assign17930_e17686: f64 = (assign17930_e17685).ln();
        let assign17930_e17688: f64 = (assign17930_e17686 + locals.var_xdeff);
        let assign17930_e17690: f64 = (assign17930_e17688 - locals.var_xg1x);
        let assign17930_e17692: f64 = (assign17930_e17690 + locals.var_q1d);
        let assign17930_e17693: f64 = (locals.var_q_delta * assign17930_e17692);
        (assign17930_e17693, ((locals.var_q_delta_dn4 * assign17930_e17692) + (locals.var_q_delta * (((((((locals.var_q_delta_dn4 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn4)) / (locals.var_a0 * locals.var_a0)) / assign17930_e17685) + locals.var_xdeff_dn4) - locals.var_xg1x_dn4) + locals.var_q1d_dn4))), ((locals.var_q_delta_dn6 * assign17930_e17692) + (locals.var_q_delta * (((((((locals.var_q_delta_dn6 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn6)) / (locals.var_a0 * locals.var_a0)) / assign17930_e17685) + locals.var_xdeff_dn6) - locals.var_xg1x_dn6) + locals.var_q1d_dn6))), ((locals.var_q_delta_dn7 * assign17930_e17692) + (locals.var_q_delta * (((((((locals.var_q_delta_dn7 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn7)) / (locals.var_a0 * locals.var_a0)) / assign17930_e17685) + locals.var_xdeff_dn7) - locals.var_xg1x_dn7) + locals.var_q1d_dn7))), ((locals.var_q_delta_dn8 * assign17930_e17692) + (locals.var_q_delta * (((((((locals.var_q_delta_dn8 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn8)) / (locals.var_a0 * locals.var_a0)) / assign17930_e17685) + locals.var_xdeff_dn8) - locals.var_xg1x_dn8) + locals.var_q1d_dn8))), ((locals.var_q_delta_dn9 * assign17930_e17692) + (locals.var_q_delta * (((((((locals.var_q_delta_dn9 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn9)) / (locals.var_a0 * locals.var_a0)) / assign17930_e17685) + locals.var_xdeff_dn9) - locals.var_xg1x_dn9) + locals.var_q1d_dn9))),)
    } else {
        (locals.var_q_zero, locals.var_q_zero_dn4, locals.var_q_zero_dn6, locals.var_q_zero_dn7, locals.var_q_zero_dn8, locals.var_q_zero_dn9,)
    }
};
        locals.var_q_zero = assign17930_e17695;
        locals.var_q_zero_dn4 = assign17930_e17695_d_n4;
        locals.var_q_zero_dn6 = assign17930_e17695_d_n6;
        locals.var_q_zero_dn7 = assign17930_e17695_d_n7;
        locals.var_q_zero_dn8 = assign17930_e17695_d_n8;
        locals.var_q_zero_dn9 = assign17930_e17695_d_n9;

        let (assign17940_e17705, assign17940_e17705_d_n4, assign17940_e17705_d_n6, assign17940_e17705_d_n7, assign17940_e17705_d_n8, assign17940_e17705_d_n9,) = {
    if (locals.var_guard626 != 0.0) {
        let assign17940_e17699: f64 = (2.0 * locals.var_k1);
        let assign17940_e17701: f64 = (assign17940_e17699 * locals.var_q_k1q1);
        let assign17940_e17703: f64 = (assign17940_e17701 + locals.var_q_delta);
        (assign17940_e17703, ((((2.0 * locals.var_k1_dn4) * locals.var_q_k1q1) + (assign17940_e17699 * locals.var_q_k1q1_dn4)) + locals.var_q_delta_dn4), ((((2.0 * locals.var_k1_dn6) * locals.var_q_k1q1) + (assign17940_e17699 * locals.var_q_k1q1_dn6)) + locals.var_q_delta_dn6), ((((2.0 * locals.var_k1_dn7) * locals.var_q_k1q1) + (assign17940_e17699 * locals.var_q_k1q1_dn7)) + locals.var_q_delta_dn7), ((((2.0 * locals.var_k1_dn8) * locals.var_q_k1q1) + (assign17940_e17699 * locals.var_q_k1q1_dn8)) + locals.var_q_delta_dn8), ((((2.0 * locals.var_k1_dn9) * locals.var_q_k1q1) + (assign17940_e17699 * locals.var_q_k1q1_dn9)) + locals.var_q_delta_dn9),)
    } else {
        (locals.var_q_d1_zero, locals.var_q_d1_zero_dn4, locals.var_q_d1_zero_dn6, locals.var_q_d1_zero_dn7, locals.var_q_d1_zero_dn8, locals.var_q_d1_zero_dn9,)
    }
};
        locals.var_q_d1_zero = assign17940_e17705;
        locals.var_q_d1_zero_dn4 = assign17940_e17705_d_n4;
        locals.var_q_d1_zero_dn6 = assign17940_e17705_d_n6;
        locals.var_q_d1_zero_dn7 = assign17940_e17705_d_n7;
        locals.var_q_d1_zero_dn8 = assign17940_e17705_d_n8;
        locals.var_q_d1_zero_dn9 = assign17940_e17705_d_n9;

        let (assign17950_e17713,) = {
    if (locals.var_guard626 != 0.0) {
        let assign17950_e17709: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign17950_e17711: f64 = (assign17950_e17709 - locals.var_q_x1sat);
        (assign17950_e17711,)
    } else {
        (locals.var_q_dx1,)
    }
};
        locals.var_q_dx1 = assign17950_e17713;

        let assign17960_e17723: f64 = (locals.var_q_dx1 + 2.3025850929941);
        let assign17960_e17725: f64 = (locals.var_k1).ln();
        let assign17960_e17726: f64 = (assign17960_e17723 + assign17960_e17725);
        let assign17960_e17733: f64 = if ((((locals.var_q_zero < 0.0) && (locals.var_q_d1_zero > 0.0)) && (assign17960_e17726 > 0.0)) || (locals.var_q_dx1 > 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard627 = assign17960_e17733;

        let (assign17970_e17743, assign17970_e17743_d_n4, assign17970_e17743_d_n6, assign17970_e17743_d_n7, assign17970_e17743_d_n8, assign17970_e17743_d_n9,) = {
    if ((locals.var_guard626 != 0.0) && (locals.var_guard627 != 0.0)) {
        let assign17970_e17740: f64 = (locals.var_q_zero / locals.var_q_d1_zero);
        let assign17970_e17741: f64 = (locals.var_q1d - assign17970_e17740);
        (assign17970_e17741, (locals.var_q1d_dn4 - (((locals.var_q_zero_dn4 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn4)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1d_dn6 - (((locals.var_q_zero_dn6 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn6)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1d_dn7 - (((locals.var_q_zero_dn7 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn7)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1d_dn8 - (((locals.var_q_zero_dn8 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn8)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1d_dn9 - (((locals.var_q_zero_dn9 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn9)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))),)
    } else {
        (locals.var_q1d, locals.var_q1d_dn4, locals.var_q1d_dn6, locals.var_q1d_dn7, locals.var_q1d_dn8, locals.var_q1d_dn9,)
    }
};
        locals.var_q1d = assign17970_e17743;
        locals.var_q1d_dn4 = assign17970_e17743_d_n4;
        locals.var_q1d_dn6 = assign17970_e17743_d_n6;
        locals.var_q1d_dn7 = assign17970_e17743_d_n7;
        locals.var_q1d_dn8 = assign17970_e17743_d_n8;
        locals.var_q1d_dn9 = assign17970_e17743_d_n9;

        let assign17980_e17746: f64 = (locals.var_k1 * locals.var_q1d);
        locals.var_q_k1q1 = assign17980_e17746;
        locals.var_q_k1q1_dn4 = ((locals.var_k1_dn4 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn4));
        locals.var_q_k1q1_dn6 = ((locals.var_k1_dn6 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn6));
        locals.var_q_k1q1_dn7 = ((locals.var_k1_dn7 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn7));
        locals.var_q_k1q1_dn8 = ((locals.var_k1_dn8 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn8));
        locals.var_q_k1q1_dn9 = ((locals.var_k1_dn9 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn9));

        let assign17990_e17749: f64 = (locals.var_k2 * locals.var_q2d);
        locals.var_q_k2q2 = assign17990_e17749;
        locals.var_q_k2q2_dn4 = ((locals.var_k2_dn4 * locals.var_q2d) + (locals.var_k2 * locals.var_q2d_dn4));
        locals.var_q_k2q2_dn6 = ((locals.var_k2_dn6 * locals.var_q2d) + (locals.var_k2 * locals.var_q2d_dn6));
        locals.var_q_k2q2_dn7 = ((locals.var_k2_dn7 * locals.var_q2d) + (locals.var_k2 * locals.var_q2d_dn7));
        locals.var_q_k2q2_dn8 = ((locals.var_k2_dn8 * locals.var_q2d) + (locals.var_k2 * locals.var_q2d_dn8));
        locals.var_q_k2q2_dn9 = ((locals.var_k2_dn9 * locals.var_q2d) + (locals.var_k2 * locals.var_q2d_dn9));

        let assign18000_e17752: f64 = (locals.var_q_k1q1 + locals.var_q_k2q2);
        locals.var_q_qi_int = assign18000_e17752;
        locals.var_q_qi_int_dn4 = (locals.var_q_k1q1_dn4 + locals.var_q_k2q2_dn4);
        locals.var_q_qi_int_dn6 = (locals.var_q_k1q1_dn6 + locals.var_q_k2q2_dn6);
        locals.var_q_qi_int_dn7 = (locals.var_q_k1q1_dn7 + locals.var_q_k2q2_dn7);
        locals.var_q_qi_int_dn8 = (locals.var_q_k1q1_dn8 + locals.var_q_k2q2_dn8);
        locals.var_q_qi_int_dn9 = (locals.var_q_k1q1_dn9 + locals.var_q_k2q2_dn9);

        let assign18010_e17756: f64 = (0.065345483024 * locals.var_q_qi_int);
        let assign18010_e17757: f64 = (1.0 + assign18010_e17756);
        locals.var_q_a = assign18010_e17757;
        locals.var_q_a_dn4 = (0.065345483024 * locals.var_q_qi_int_dn4);
        locals.var_q_a_dn6 = (0.065345483024 * locals.var_q_qi_int_dn6);
        locals.var_q_a_dn7 = (0.065345483024 * locals.var_q_qi_int_dn7);
        locals.var_q_a_dn8 = (0.065345483024 * locals.var_q_qi_int_dn8);
        locals.var_q_a_dn9 = (0.065345483024 * locals.var_q_qi_int_dn9);

        let assign18020_e17761: f64 = (8.5797362674 * locals.var_q_qi_int);
        let assign18020_e17762: f64 = (39.478417604 + assign18020_e17761);
        let assign18020_e17765: f64 = (locals.var_q_k1q1 * locals.var_q_k2q2);
        let assign18020_e17766: f64 = (assign18020_e17762 + assign18020_e17765);
        locals.var_q_b = assign18020_e17766;
        locals.var_q_b_dn4 = ((8.5797362674 * locals.var_q_qi_int_dn4) + ((locals.var_q_k1q1_dn4 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn4)));
        locals.var_q_b_dn6 = ((8.5797362674 * locals.var_q_qi_int_dn6) + ((locals.var_q_k1q1_dn6 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn6)));
        locals.var_q_b_dn7 = ((8.5797362674 * locals.var_q_qi_int_dn7) + ((locals.var_q_k1q1_dn7 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn7)));
        locals.var_q_b_dn8 = ((8.5797362674 * locals.var_q_qi_int_dn8) + ((locals.var_q_k1q1_dn8 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn8)));
        locals.var_q_b_dn9 = ((8.5797362674 * locals.var_q_qi_int_dn9) + ((locals.var_q_k1q1_dn9 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn9)));

        let assign18030_e17770: f64 = (2.0 * locals.var_q_qi_int);
        let assign18030_e17773: f64 = (locals.var_q_k1q1 * locals.var_q_k2q2);
        let assign18030_e17774: f64 = (assign18030_e17770 + assign18030_e17773);
        let assign18030_e17775: f64 = (39.478417604 * assign18030_e17774);
        locals.var_q_c = assign18030_e17775;
        locals.var_q_c_dn4 = (39.478417604 * ((2.0 * locals.var_q_qi_int_dn4) + ((locals.var_q_k1q1_dn4 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn4))));
        locals.var_q_c_dn6 = (39.478417604 * ((2.0 * locals.var_q_qi_int_dn6) + ((locals.var_q_k1q1_dn6 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn6))));
        locals.var_q_c_dn7 = (39.478417604 * ((2.0 * locals.var_q_qi_int_dn7) + ((locals.var_q_k1q1_dn7 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn7))));
        locals.var_q_c_dn8 = (39.478417604 * ((2.0 * locals.var_q_qi_int_dn8) + ((locals.var_q_k1q1_dn8 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn8))));
        locals.var_q_c_dn9 = (39.478417604 * ((2.0 * locals.var_q_qi_int_dn9) + ((locals.var_q_k1q1_dn9 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn9))));

        let assign18040_e17778: f64 = (locals.var_q_b * locals.var_q_b);
        let assign18040_e17781: f64 = (4.0 * locals.var_q_a);
        let assign18040_e17783: f64 = (assign18040_e17781 * locals.var_q_c);
        let assign18040_e17784: f64 = (assign18040_e17778 - assign18040_e17783);
        let assign18040_e17785: f64 = (assign18040_e17784).sqrt();
        locals.var_q_disc = assign18040_e17785;
        locals.var_q_disc_dn4 = ((((locals.var_q_b_dn4 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn4)) - (((4.0 * locals.var_q_a_dn4) * locals.var_q_c) + (assign18040_e17781 * locals.var_q_c_dn4))) / (2.0 * assign18040_e17785));
        locals.var_q_disc_dn6 = ((((locals.var_q_b_dn6 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn6)) - (((4.0 * locals.var_q_a_dn6) * locals.var_q_c) + (assign18040_e17781 * locals.var_q_c_dn6))) / (2.0 * assign18040_e17785));
        locals.var_q_disc_dn7 = ((((locals.var_q_b_dn7 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn7)) - (((4.0 * locals.var_q_a_dn7) * locals.var_q_c) + (assign18040_e17781 * locals.var_q_c_dn7))) / (2.0 * assign18040_e17785));
        locals.var_q_disc_dn8 = ((((locals.var_q_b_dn8 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn8)) - (((4.0 * locals.var_q_a_dn8) * locals.var_q_c) + (assign18040_e17781 * locals.var_q_c_dn8))) / (2.0 * assign18040_e17785));
        locals.var_q_disc_dn9 = ((((locals.var_q_b_dn9 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn9)) - (((4.0 * locals.var_q_a_dn9) * locals.var_q_c) + (assign18040_e17781 * locals.var_q_c_dn9))) / (2.0 * assign18040_e17785));

        let assign18050_e17788: f64 = (locals.var_q_disc - locals.var_q_b);
        let assign18050_e17791: f64 = (2.0 * locals.var_q_a);
        let assign18050_e17792: f64 = (assign18050_e17788 / assign18050_e17791);
        locals.var_q_qsq = assign18050_e17792;
        locals.var_q_qsq_dn4 = ((((locals.var_q_disc_dn4 - locals.var_q_b_dn4) * assign18050_e17791) - (assign18050_e17788 * (2.0 * locals.var_q_a_dn4))) / (assign18050_e17791 * assign18050_e17791));
        locals.var_q_qsq_dn6 = ((((locals.var_q_disc_dn6 - locals.var_q_b_dn6) * assign18050_e17791) - (assign18050_e17788 * (2.0 * locals.var_q_a_dn6))) / (assign18050_e17791 * assign18050_e17791));
        locals.var_q_qsq_dn7 = ((((locals.var_q_disc_dn7 - locals.var_q_b_dn7) * assign18050_e17791) - (assign18050_e17788 * (2.0 * locals.var_q_a_dn7))) / (assign18050_e17791 * assign18050_e17791));
        locals.var_q_qsq_dn8 = ((((locals.var_q_disc_dn8 - locals.var_q_b_dn8) * assign18050_e17791) - (assign18050_e17788 * (2.0 * locals.var_q_a_dn8))) / (assign18050_e17791 * assign18050_e17791));
        locals.var_q_qsq_dn9 = ((((locals.var_q_disc_dn9 - locals.var_q_b_dn9) * assign18050_e17791) - (assign18050_e17788 * (2.0 * locals.var_q_a_dn9))) / (assign18050_e17791 * assign18050_e17791));

        let assign18060_e17795: f64 = (-0.005);
        let assign18060_e17796: f64 = if locals.var_q_qsq < assign18060_e17795 { 1.0 } else { 0.0 };
        locals.var_guard628 = assign18060_e17796;

        let (assign18070_e17802, assign18070_e17802_d_n4, assign18070_e17802_d_n6, assign18070_e17802_d_n7, assign18070_e17802_d_n8, assign18070_e17802_d_n9,) = {
    if (locals.var_guard628 != 0.0) {
        let assign18070_e17799: f64 = (locals.var_q_qsq).abs();
        let assign18070_e17800: f64 = (assign18070_e17799).sqrt();
        (assign18070_e17800, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign18070_e17800)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign18070_e17800)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign18070_e17800)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign18070_e17800)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign18070_e17800)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign18070_e17802;
        locals.var_q_rac_qsq_dn4 = assign18070_e17802_d_n4;
        locals.var_q_rac_qsq_dn6 = assign18070_e17802_d_n6;
        locals.var_q_rac_qsq_dn7 = assign18070_e17802_d_n7;
        locals.var_q_rac_qsq_dn8 = assign18070_e17802_d_n8;
        locals.var_q_rac_qsq_dn9 = assign18070_e17802_d_n9;

        let (assign18080_e17811, assign18080_e17811_d_n4, assign18080_e17811_d_n6, assign18080_e17811_d_n7, assign18080_e17811_d_n8, assign18080_e17811_d_n9,) = {
    if (locals.var_guard628 != 0.0) {
        let assign18080_e17807: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign18080_e17808: f64 = (assign18080_e17807).tan();
        let assign18080_e17809: f64 = (locals.var_q_rac_qsq / assign18080_e17808);
        (assign18080_e17809, (((locals.var_q_rac_qsq_dn4 * assign18080_e17808) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn4) / ((assign18080_e17807).cos() * (assign18080_e17807).cos())))) / (assign18080_e17808 * assign18080_e17808)), (((locals.var_q_rac_qsq_dn6 * assign18080_e17808) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn6) / ((assign18080_e17807).cos() * (assign18080_e17807).cos())))) / (assign18080_e17808 * assign18080_e17808)), (((locals.var_q_rac_qsq_dn7 * assign18080_e17808) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn7) / ((assign18080_e17807).cos() * (assign18080_e17807).cos())))) / (assign18080_e17808 * assign18080_e17808)), (((locals.var_q_rac_qsq_dn8 * assign18080_e17808) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn8) / ((assign18080_e17807).cos() * (assign18080_e17807).cos())))) / (assign18080_e17808 * assign18080_e17808)), (((locals.var_q_rac_qsq_dn9 * assign18080_e17808) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn9) / ((assign18080_e17807).cos() * (assign18080_e17807).cos())))) / (assign18080_e17808 * assign18080_e17808)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign18080_e17811;
        locals.var_q_qcoth_dn4 = assign18080_e17811_d_n4;
        locals.var_q_qcoth_dn6 = assign18080_e17811_d_n6;
        locals.var_q_qcoth_dn7 = assign18080_e17811_d_n7;
        locals.var_q_qcoth_dn8 = assign18080_e17811_d_n8;
        locals.var_q_qcoth_dn9 = assign18080_e17811_d_n9;

        let (assign18090_e17825, assign18090_e17825_d_n4, assign18090_e17825_d_n6, assign18090_e17825_d_n7, assign18090_e17825_d_n8, assign18090_e17825_d_n9,) = {
    if (locals.var_guard628 != 0.0) {
        let assign18090_e17818: f64 = (2.0 - locals.var_q_qcoth);
        let assign18090_e17819: f64 = (locals.var_q_qcoth * assign18090_e17818);
        let assign18090_e17820: f64 = (locals.var_q_qsq + assign18090_e17819);
        let assign18090_e17821: f64 = (0.25 * assign18090_e17820);
        let assign18090_e17823: f64 = (assign18090_e17821 / locals.var_q_qsq);
        (assign18090_e17823, ((((0.25 * (locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign18090_e17818) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4))))) * locals.var_q_qsq) - (assign18090_e17821 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign18090_e17818) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6))))) * locals.var_q_qsq) - (assign18090_e17821 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign18090_e17818) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7))))) * locals.var_q_qsq) - (assign18090_e17821 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign18090_e17818) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8))))) * locals.var_q_qsq) - (assign18090_e17821 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign18090_e17818) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9))))) * locals.var_q_qsq) - (assign18090_e17821 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign18090_e17825;
        locals.var_q_d1_qcoth_dn4 = assign18090_e17825_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign18090_e17825_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign18090_e17825_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign18090_e17825_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign18090_e17825_d_n9;

        let assign18100_e17828: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard629 = assign18100_e17828;

        let (assign18110_e17837, assign18110_e17837_d_n4, assign18110_e17837_d_n6, assign18110_e17837_d_n7, assign18110_e17837_d_n8, assign18110_e17837_d_n9,) = {
    if ((locals.var_guard628 == 0.0) && (locals.var_guard629 != 0.0)) {
        let assign18110_e17834: f64 = (locals.var_q_qsq).abs();
        let assign18110_e17835: f64 = (assign18110_e17834).sqrt();
        (assign18110_e17835, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign18110_e17835)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign18110_e17835)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign18110_e17835)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign18110_e17835)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign18110_e17835)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign18110_e17837;
        locals.var_q_rac_qsq_dn4 = assign18110_e17837_d_n4;
        locals.var_q_rac_qsq_dn6 = assign18110_e17837_d_n6;
        locals.var_q_rac_qsq_dn7 = assign18110_e17837_d_n7;
        locals.var_q_rac_qsq_dn8 = assign18110_e17837_d_n8;
        locals.var_q_rac_qsq_dn9 = assign18110_e17837_d_n9;

        let (assign18120_e17846, assign18120_e17846_d_n4, assign18120_e17846_d_n6, assign18120_e17846_d_n7, assign18120_e17846_d_n8, assign18120_e17846_d_n9,) = {
    if ((locals.var_guard628 == 0.0) && (locals.var_guard629 != 0.0)) {
        let assign18120_e17843: f64 = (-locals.var_q_rac_qsq);
        let assign18120_e17844: f64 = (assign18120_e17843).exp();
        (assign18120_e17844, (assign18120_e17844 * (-locals.var_q_rac_qsq_dn4)), (assign18120_e17844 * (-locals.var_q_rac_qsq_dn6)), (assign18120_e17844 * (-locals.var_q_rac_qsq_dn7)), (assign18120_e17844 * (-locals.var_q_rac_qsq_dn8)), (assign18120_e17844 * (-locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_invexpq, locals.var_q_invexpq_dn4, locals.var_q_invexpq_dn6, locals.var_q_invexpq_dn7, locals.var_q_invexpq_dn8, locals.var_q_invexpq_dn9,)
    }
};
        locals.var_q_invexpq = assign18120_e17846;
        locals.var_q_invexpq_dn4 = assign18120_e17846_d_n4;
        locals.var_q_invexpq_dn6 = assign18120_e17846_d_n6;
        locals.var_q_invexpq_dn7 = assign18120_e17846_d_n7;
        locals.var_q_invexpq_dn8 = assign18120_e17846_d_n8;
        locals.var_q_invexpq_dn9 = assign18120_e17846_d_n9;

        let (assign18130_e17861, assign18130_e17861_d_n4, assign18130_e17861_d_n6, assign18130_e17861_d_n7, assign18130_e17861_d_n8, assign18130_e17861_d_n9,) = {
    if ((locals.var_guard628 == 0.0) && (locals.var_guard629 != 0.0)) {
        let assign18130_e17854: f64 = (1.0 + locals.var_q_invexpq);
        let assign18130_e17855: f64 = (locals.var_q_rac_qsq * assign18130_e17854);
        let assign18130_e17858: f64 = (1.0 - locals.var_q_invexpq);
        let assign18130_e17859: f64 = (assign18130_e17855 / assign18130_e17858);
        (assign18130_e17859, (((((locals.var_q_rac_qsq_dn4 * assign18130_e17854) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn4)) * assign18130_e17858) - (assign18130_e17855 * (-locals.var_q_invexpq_dn4))) / (assign18130_e17858 * assign18130_e17858)), (((((locals.var_q_rac_qsq_dn6 * assign18130_e17854) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn6)) * assign18130_e17858) - (assign18130_e17855 * (-locals.var_q_invexpq_dn6))) / (assign18130_e17858 * assign18130_e17858)), (((((locals.var_q_rac_qsq_dn7 * assign18130_e17854) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn7)) * assign18130_e17858) - (assign18130_e17855 * (-locals.var_q_invexpq_dn7))) / (assign18130_e17858 * assign18130_e17858)), (((((locals.var_q_rac_qsq_dn8 * assign18130_e17854) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn8)) * assign18130_e17858) - (assign18130_e17855 * (-locals.var_q_invexpq_dn8))) / (assign18130_e17858 * assign18130_e17858)), (((((locals.var_q_rac_qsq_dn9 * assign18130_e17854) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn9)) * assign18130_e17858) - (assign18130_e17855 * (-locals.var_q_invexpq_dn9))) / (assign18130_e17858 * assign18130_e17858)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign18130_e17861;
        locals.var_q_qcoth_dn4 = assign18130_e17861_d_n4;
        locals.var_q_qcoth_dn6 = assign18130_e17861_d_n6;
        locals.var_q_qcoth_dn7 = assign18130_e17861_d_n7;
        locals.var_q_qcoth_dn8 = assign18130_e17861_d_n8;
        locals.var_q_qcoth_dn9 = assign18130_e17861_d_n9;

        let (assign18140_e17878, assign18140_e17878_d_n4, assign18140_e17878_d_n6, assign18140_e17878_d_n7, assign18140_e17878_d_n8, assign18140_e17878_d_n9,) = {
    if ((locals.var_guard628 == 0.0) && (locals.var_guard629 != 0.0)) {
        let assign18140_e17871: f64 = (2.0 - locals.var_q_qcoth);
        let assign18140_e17872: f64 = (locals.var_q_qcoth * assign18140_e17871);
        let assign18140_e17873: f64 = (locals.var_q_qsq + assign18140_e17872);
        let assign18140_e17874: f64 = (0.25 * assign18140_e17873);
        let assign18140_e17876: f64 = (assign18140_e17874 / locals.var_q_qsq);
        (assign18140_e17876, ((((0.25 * (locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign18140_e17871) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4))))) * locals.var_q_qsq) - (assign18140_e17874 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign18140_e17871) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6))))) * locals.var_q_qsq) - (assign18140_e17874 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign18140_e17871) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7))))) * locals.var_q_qsq) - (assign18140_e17874 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign18140_e17871) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8))))) * locals.var_q_qsq) - (assign18140_e17874 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign18140_e17871) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9))))) * locals.var_q_qsq) - (assign18140_e17874 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign18140_e17878;
        locals.var_q_d1_qcoth_dn4 = assign18140_e17878_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign18140_e17878_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign18140_e17878_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign18140_e17878_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign18140_e17878_d_n9;

        let (assign18150_e17902, assign18150_e17902_d_n4, assign18150_e17902_d_n6, assign18150_e17902_d_n7, assign18150_e17902_d_n8, assign18150_e17902_d_n9,) = {
    if ((locals.var_guard628 == 0.0) && (locals.var_guard629 == 0.0)) {
        let assign18150_e17887: f64 = (locals.var_q_qsq * 0.1666666666667);
        let assign18150_e17891: f64 = (locals.var_q_qsq * 0.0166666666667);
        let assign18150_e17895: f64 = (locals.var_q_qsq * 0.0238095238095);
        let assign18150_e17896: f64 = (1.0 - assign18150_e17895);
        let assign18150_e17897: f64 = (assign18150_e17891 * assign18150_e17896);
        let assign18150_e17898: f64 = (1.0 - assign18150_e17897);
        let assign18150_e17899: f64 = (assign18150_e17887 * assign18150_e17898);
        let assign18150_e17900: f64 = (2.0 + assign18150_e17899);
        (assign18150_e17900, (((locals.var_q_qsq_dn4 * 0.1666666666667) * assign18150_e17898) + (assign18150_e17887 * (-(((locals.var_q_qsq_dn4 * 0.0166666666667) * assign18150_e17896) + (assign18150_e17891 * (-(locals.var_q_qsq_dn4 * 0.0238095238095))))))), (((locals.var_q_qsq_dn6 * 0.1666666666667) * assign18150_e17898) + (assign18150_e17887 * (-(((locals.var_q_qsq_dn6 * 0.0166666666667) * assign18150_e17896) + (assign18150_e17891 * (-(locals.var_q_qsq_dn6 * 0.0238095238095))))))), (((locals.var_q_qsq_dn7 * 0.1666666666667) * assign18150_e17898) + (assign18150_e17887 * (-(((locals.var_q_qsq_dn7 * 0.0166666666667) * assign18150_e17896) + (assign18150_e17891 * (-(locals.var_q_qsq_dn7 * 0.0238095238095))))))), (((locals.var_q_qsq_dn8 * 0.1666666666667) * assign18150_e17898) + (assign18150_e17887 * (-(((locals.var_q_qsq_dn8 * 0.0166666666667) * assign18150_e17896) + (assign18150_e17891 * (-(locals.var_q_qsq_dn8 * 0.0238095238095))))))), (((locals.var_q_qsq_dn9 * 0.1666666666667) * assign18150_e17898) + (assign18150_e17887 * (-(((locals.var_q_qsq_dn9 * 0.0166666666667) * assign18150_e17896) + (assign18150_e17891 * (-(locals.var_q_qsq_dn9 * 0.0238095238095))))))),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign18150_e17902;
        locals.var_q_qcoth_dn4 = assign18150_e17902_d_n4;
        locals.var_q_qcoth_dn6 = assign18150_e17902_d_n6;
        locals.var_q_qcoth_dn7 = assign18150_e17902_d_n7;
        locals.var_q_qcoth_dn8 = assign18150_e17902_d_n8;
        locals.var_q_qcoth_dn9 = assign18150_e17902_d_n9;

        let (assign18160_e17928, assign18160_e17928_d_n4, assign18160_e17928_d_n6, assign18160_e17928_d_n7, assign18160_e17928_d_n8, assign18160_e17928_d_n9,) = {
    if ((locals.var_guard628 == 0.0) && (locals.var_guard629 == 0.0)) {
        let assign18160_e17912: f64 = (locals.var_q_qsq * 0.0333333333333);
        let assign18160_e17916: f64 = (locals.var_q_qsq * 0.0357142857143);
        let assign18160_e17920: f64 = (locals.var_q_qsq * 0.0333333333333);
        let assign18160_e17921: f64 = (1.0 - assign18160_e17920);
        let assign18160_e17922: f64 = (assign18160_e17916 * assign18160_e17921);
        let assign18160_e17923: f64 = (1.0 - assign18160_e17922);
        let assign18160_e17924: f64 = (assign18160_e17912 * assign18160_e17923);
        let assign18160_e17925: f64 = (1.0 - assign18160_e17924);
        let assign18160_e17926: f64 = (0.1666666666667 * assign18160_e17925);
        (assign18160_e17926, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0333333333333) * assign18160_e17923) + (assign18160_e17912 * (-(((locals.var_q_qsq_dn4 * 0.0357142857143) * assign18160_e17921) + (assign18160_e17916 * (-(locals.var_q_qsq_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0333333333333) * assign18160_e17923) + (assign18160_e17912 * (-(((locals.var_q_qsq_dn6 * 0.0357142857143) * assign18160_e17921) + (assign18160_e17916 * (-(locals.var_q_qsq_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0333333333333) * assign18160_e17923) + (assign18160_e17912 * (-(((locals.var_q_qsq_dn7 * 0.0357142857143) * assign18160_e17921) + (assign18160_e17916 * (-(locals.var_q_qsq_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0333333333333) * assign18160_e17923) + (assign18160_e17912 * (-(((locals.var_q_qsq_dn8 * 0.0357142857143) * assign18160_e17921) + (assign18160_e17916 * (-(locals.var_q_qsq_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0333333333333) * assign18160_e17923) + (assign18160_e17912 * (-(((locals.var_q_qsq_dn9 * 0.0357142857143) * assign18160_e17921) + (assign18160_e17916 * (-(locals.var_q_qsq_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign18160_e17928;
        locals.var_q_d1_qcoth_dn4 = assign18160_e17928_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign18160_e17928_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign18160_e17928_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign18160_e17928_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign18160_e17928_d_n9;

        let assign18170_e17932: f64 = (locals.var_q_qi_int * locals.var_q_qcoth);
        let assign18170_e17935: f64 = (locals.var_q_k1q1 * locals.var_q_k2q2);
        let assign18170_e17936: f64 = (assign18170_e17932 + assign18170_e17935);
        let assign18170_e17938: f64 = (assign18170_e17936 + locals.var_q_qsq);
        let assign18170_e17941: f64 = (locals.var_q_qi_int * locals.var_q_d1_qcoth);
        let assign18170_e17943: f64 = (assign18170_e17941 + 1.0);
        let assign18170_e17944: f64 = (assign18170_e17938 / assign18170_e17943);
        let assign18170_e17945: f64 = (locals.var_q_qsq - assign18170_e17944);
        locals.var_q_qsq = assign18170_e17945;
        locals.var_q_qsq_dn4 = (locals.var_q_qsq_dn4 - (((((((locals.var_q_qi_int_dn4 * locals.var_q_qcoth) + (locals.var_q_qi_int * locals.var_q_qcoth_dn4)) + ((locals.var_q_k1q1_dn4 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn4))) + locals.var_q_qsq_dn4) * assign18170_e17943) - (assign18170_e17938 * ((locals.var_q_qi_int_dn4 * locals.var_q_d1_qcoth) + (locals.var_q_qi_int * locals.var_q_d1_qcoth_dn4)))) / (assign18170_e17943 * assign18170_e17943)));
        locals.var_q_qsq_dn6 = (locals.var_q_qsq_dn6 - (((((((locals.var_q_qi_int_dn6 * locals.var_q_qcoth) + (locals.var_q_qi_int * locals.var_q_qcoth_dn6)) + ((locals.var_q_k1q1_dn6 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn6))) + locals.var_q_qsq_dn6) * assign18170_e17943) - (assign18170_e17938 * ((locals.var_q_qi_int_dn6 * locals.var_q_d1_qcoth) + (locals.var_q_qi_int * locals.var_q_d1_qcoth_dn6)))) / (assign18170_e17943 * assign18170_e17943)));
        locals.var_q_qsq_dn7 = (locals.var_q_qsq_dn7 - (((((((locals.var_q_qi_int_dn7 * locals.var_q_qcoth) + (locals.var_q_qi_int * locals.var_q_qcoth_dn7)) + ((locals.var_q_k1q1_dn7 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn7))) + locals.var_q_qsq_dn7) * assign18170_e17943) - (assign18170_e17938 * ((locals.var_q_qi_int_dn7 * locals.var_q_d1_qcoth) + (locals.var_q_qi_int * locals.var_q_d1_qcoth_dn7)))) / (assign18170_e17943 * assign18170_e17943)));
        locals.var_q_qsq_dn8 = (locals.var_q_qsq_dn8 - (((((((locals.var_q_qi_int_dn8 * locals.var_q_qcoth) + (locals.var_q_qi_int * locals.var_q_qcoth_dn8)) + ((locals.var_q_k1q1_dn8 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn8))) + locals.var_q_qsq_dn8) * assign18170_e17943) - (assign18170_e17938 * ((locals.var_q_qi_int_dn8 * locals.var_q_d1_qcoth) + (locals.var_q_qi_int * locals.var_q_d1_qcoth_dn8)))) / (assign18170_e17943 * assign18170_e17943)));
        locals.var_q_qsq_dn9 = (locals.var_q_qsq_dn9 - (((((((locals.var_q_qi_int_dn9 * locals.var_q_qcoth) + (locals.var_q_qi_int * locals.var_q_qcoth_dn9)) + ((locals.var_q_k1q1_dn9 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn9))) + locals.var_q_qsq_dn9) * assign18170_e17943) - (assign18170_e17938 * ((locals.var_q_qi_int_dn9 * locals.var_q_d1_qcoth) + (locals.var_q_qi_int * locals.var_q_d1_qcoth_dn9)))) / (assign18170_e17943 * assign18170_e17943)));

        let assign18180_e17948: f64 = (locals.var_q_k1q1 * locals.var_q_k1q1);
        let assign18180_e17950: f64 = (assign18180_e17948 - locals.var_q_qsq);
        locals.var_q_delta = assign18180_e17950;
        locals.var_q_delta_dn4 = (((locals.var_q_k1q1_dn4 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn4)) - locals.var_q_qsq_dn4);
        locals.var_q_delta_dn6 = (((locals.var_q_k1q1_dn6 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn6)) - locals.var_q_qsq_dn6);
        locals.var_q_delta_dn7 = (((locals.var_q_k1q1_dn7 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn7)) - locals.var_q_qsq_dn7);
        locals.var_q_delta_dn8 = (((locals.var_q_k1q1_dn8 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn8)) - locals.var_q_qsq_dn8);
        locals.var_q_delta_dn9 = (((locals.var_q_k1q1_dn9 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn9)) - locals.var_q_qsq_dn9);

        let assign18190_e17953: f64 = if locals.var_q_delta > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard630 = assign18190_e17953;

        let (assign18200_e17968, assign18200_e17968_d_n4, assign18200_e17968_d_n6, assign18200_e17968_d_n7, assign18200_e17968_d_n8, assign18200_e17968_d_n9,) = {
    if (locals.var_guard630 != 0.0) {
        let assign18200_e17958: f64 = (locals.var_q_delta / locals.var_a0);
        let assign18200_e17959: f64 = (assign18200_e17958).ln();
        let assign18200_e17961: f64 = (assign18200_e17959 + locals.var_xdeff);
        let assign18200_e17963: f64 = (assign18200_e17961 - locals.var_xg1x);
        let assign18200_e17965: f64 = (assign18200_e17963 + locals.var_q1d);
        let assign18200_e17966: f64 = (locals.var_q_delta * assign18200_e17965);
        (assign18200_e17966, ((locals.var_q_delta_dn4 * assign18200_e17965) + (locals.var_q_delta * (((((((locals.var_q_delta_dn4 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn4)) / (locals.var_a0 * locals.var_a0)) / assign18200_e17958) + locals.var_xdeff_dn4) - locals.var_xg1x_dn4) + locals.var_q1d_dn4))), ((locals.var_q_delta_dn6 * assign18200_e17965) + (locals.var_q_delta * (((((((locals.var_q_delta_dn6 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn6)) / (locals.var_a0 * locals.var_a0)) / assign18200_e17958) + locals.var_xdeff_dn6) - locals.var_xg1x_dn6) + locals.var_q1d_dn6))), ((locals.var_q_delta_dn7 * assign18200_e17965) + (locals.var_q_delta * (((((((locals.var_q_delta_dn7 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn7)) / (locals.var_a0 * locals.var_a0)) / assign18200_e17958) + locals.var_xdeff_dn7) - locals.var_xg1x_dn7) + locals.var_q1d_dn7))), ((locals.var_q_delta_dn8 * assign18200_e17965) + (locals.var_q_delta * (((((((locals.var_q_delta_dn8 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn8)) / (locals.var_a0 * locals.var_a0)) / assign18200_e17958) + locals.var_xdeff_dn8) - locals.var_xg1x_dn8) + locals.var_q1d_dn8))), ((locals.var_q_delta_dn9 * assign18200_e17965) + (locals.var_q_delta * (((((((locals.var_q_delta_dn9 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn9)) / (locals.var_a0 * locals.var_a0)) / assign18200_e17958) + locals.var_xdeff_dn9) - locals.var_xg1x_dn9) + locals.var_q1d_dn9))),)
    } else {
        (locals.var_q_zero, locals.var_q_zero_dn4, locals.var_q_zero_dn6, locals.var_q_zero_dn7, locals.var_q_zero_dn8, locals.var_q_zero_dn9,)
    }
};
        locals.var_q_zero = assign18200_e17968;
        locals.var_q_zero_dn4 = assign18200_e17968_d_n4;
        locals.var_q_zero_dn6 = assign18200_e17968_d_n6;
        locals.var_q_zero_dn7 = assign18200_e17968_d_n7;
        locals.var_q_zero_dn8 = assign18200_e17968_d_n8;
        locals.var_q_zero_dn9 = assign18200_e17968_d_n9;

        let (assign18210_e17978, assign18210_e17978_d_n4, assign18210_e17978_d_n6, assign18210_e17978_d_n7, assign18210_e17978_d_n8, assign18210_e17978_d_n9,) = {
    if (locals.var_guard630 != 0.0) {
        let assign18210_e17972: f64 = (2.0 * locals.var_k1);
        let assign18210_e17974: f64 = (assign18210_e17972 * locals.var_q_k1q1);
        let assign18210_e17976: f64 = (assign18210_e17974 + locals.var_q_delta);
        (assign18210_e17976, ((((2.0 * locals.var_k1_dn4) * locals.var_q_k1q1) + (assign18210_e17972 * locals.var_q_k1q1_dn4)) + locals.var_q_delta_dn4), ((((2.0 * locals.var_k1_dn6) * locals.var_q_k1q1) + (assign18210_e17972 * locals.var_q_k1q1_dn6)) + locals.var_q_delta_dn6), ((((2.0 * locals.var_k1_dn7) * locals.var_q_k1q1) + (assign18210_e17972 * locals.var_q_k1q1_dn7)) + locals.var_q_delta_dn7), ((((2.0 * locals.var_k1_dn8) * locals.var_q_k1q1) + (assign18210_e17972 * locals.var_q_k1q1_dn8)) + locals.var_q_delta_dn8), ((((2.0 * locals.var_k1_dn9) * locals.var_q_k1q1) + (assign18210_e17972 * locals.var_q_k1q1_dn9)) + locals.var_q_delta_dn9),)
    } else {
        (locals.var_q_d1_zero, locals.var_q_d1_zero_dn4, locals.var_q_d1_zero_dn6, locals.var_q_d1_zero_dn7, locals.var_q_d1_zero_dn8, locals.var_q_d1_zero_dn9,)
    }
};
        locals.var_q_d1_zero = assign18210_e17978;
        locals.var_q_d1_zero_dn4 = assign18210_e17978_d_n4;
        locals.var_q_d1_zero_dn6 = assign18210_e17978_d_n6;
        locals.var_q_d1_zero_dn7 = assign18210_e17978_d_n7;
        locals.var_q_d1_zero_dn8 = assign18210_e17978_d_n8;
        locals.var_q_d1_zero_dn9 = assign18210_e17978_d_n9;

        let (assign18220_e17986,) = {
    if (locals.var_guard630 != 0.0) {
        let assign18220_e17982: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign18220_e17984: f64 = (assign18220_e17982 - locals.var_q_x1sat);
        (assign18220_e17984,)
    } else {
        (locals.var_q_dx1,)
    }
};
        locals.var_q_dx1 = assign18220_e17986;

        let assign18230_e17996: f64 = (locals.var_q_dx1 + 2.3025850929941);
        let assign18230_e17998: f64 = (locals.var_k1).ln();
        let assign18230_e17999: f64 = (assign18230_e17996 + assign18230_e17998);
        let assign18230_e18006: f64 = if ((((locals.var_q_zero < 0.0) && (locals.var_q_d1_zero > 0.0)) && (assign18230_e17999 > 0.0)) || (locals.var_q_dx1 > 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard631 = assign18230_e18006;

        let (assign18240_e18016, assign18240_e18016_d_n4, assign18240_e18016_d_n6, assign18240_e18016_d_n7, assign18240_e18016_d_n8, assign18240_e18016_d_n9,) = {
    if ((locals.var_guard630 != 0.0) && (locals.var_guard631 != 0.0)) {
        let assign18240_e18013: f64 = (locals.var_q_zero / locals.var_q_d1_zero);
        let assign18240_e18014: f64 = (locals.var_q1d - assign18240_e18013);
        (assign18240_e18014, (locals.var_q1d_dn4 - (((locals.var_q_zero_dn4 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn4)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1d_dn6 - (((locals.var_q_zero_dn6 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn6)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1d_dn7 - (((locals.var_q_zero_dn7 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn7)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1d_dn8 - (((locals.var_q_zero_dn8 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn8)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1d_dn9 - (((locals.var_q_zero_dn9 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn9)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))),)
    } else {
        (locals.var_q1d, locals.var_q1d_dn4, locals.var_q1d_dn6, locals.var_q1d_dn7, locals.var_q1d_dn8, locals.var_q1d_dn9,)
    }
};
        locals.var_q1d = assign18240_e18016;
        locals.var_q1d_dn4 = assign18240_e18016_d_n4;
        locals.var_q1d_dn6 = assign18240_e18016_d_n6;
        locals.var_q1d_dn7 = assign18240_e18016_d_n7;
        locals.var_q1d_dn8 = assign18240_e18016_d_n8;
        locals.var_q1d_dn9 = assign18240_e18016_d_n9;

        let assign18250_e18019: f64 = (locals.var_k1 * locals.var_q1d);
        locals.var_q_k1q1 = assign18250_e18019;
        locals.var_q_k1q1_dn4 = ((locals.var_k1_dn4 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn4));
        locals.var_q_k1q1_dn6 = ((locals.var_k1_dn6 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn6));
        locals.var_q_k1q1_dn7 = ((locals.var_k1_dn7 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn7));
        locals.var_q_k1q1_dn8 = ((locals.var_k1_dn8 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn8));
        locals.var_q_k1q1_dn9 = ((locals.var_k1_dn9 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn9));

        let assign18260_e18022: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign18260_e18024: f64 = (assign18260_e18022 - locals.var_xdeff);
        let assign18260_e18026: f64 = if assign18260_e18024 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard632 = assign18260_e18026;

        let (assign18270_e18035, assign18270_e18035_d_n4, assign18270_e18035_d_n6, assign18270_e18035_d_n7, assign18270_e18035_d_n8, assign18270_e18035_d_n9,) = {
    if (locals.var_guard632 != 0.0) {
        let assign18270_e18030: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign18270_e18032: f64 = (assign18270_e18030 - locals.var_xdeff);
        let assign18270_e18033: f64 = (assign18270_e18032).exp();
        (assign18270_e18033, (assign18270_e18033 * ((locals.var_xg1x_dn4 - locals.var_q1d_dn4) - locals.var_xdeff_dn4)), (assign18270_e18033 * ((locals.var_xg1x_dn6 - locals.var_q1d_dn6) - locals.var_xdeff_dn6)), (assign18270_e18033 * ((locals.var_xg1x_dn7 - locals.var_q1d_dn7) - locals.var_xdeff_dn7)), (assign18270_e18033 * ((locals.var_xg1x_dn8 - locals.var_q1d_dn8) - locals.var_xdeff_dn8)), (assign18270_e18033 * ((locals.var_xg1x_dn9 - locals.var_q1d_dn9) - locals.var_xdeff_dn9)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign18270_e18035;
        locals.var_q_temp1_dn4 = assign18270_e18035_d_n4;
        locals.var_q_temp1_dn6 = assign18270_e18035_d_n6;
        locals.var_q_temp1_dn7 = assign18270_e18035_d_n7;
        locals.var_q_temp1_dn8 = assign18270_e18035_d_n8;
        locals.var_q_temp1_dn9 = assign18270_e18035_d_n9;

        let (assign18280_e18074, assign18280_e18074_d_n4, assign18280_e18074_d_n6, assign18280_e18074_d_n7, assign18280_e18074_d_n8, assign18280_e18074_d_n9,) = {
    if (locals.var_guard632 == 0.0) {
        let assign18280_e18042: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign18280_e18044: f64 = (assign18280_e18042 - locals.var_xdeff);
        let assign18280_e18046: f64 = (assign18280_e18044 - 80.0);
        let assign18280_e18051: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign18280_e18053: f64 = (assign18280_e18051 - locals.var_xdeff);
        let assign18280_e18055: f64 = (assign18280_e18053 - 80.0);
        let assign18280_e18056: f64 = (0.5 * assign18280_e18055);
        let assign18280_e18060: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign18280_e18062: f64 = (assign18280_e18060 - locals.var_xdeff);
        let assign18280_e18064: f64 = (assign18280_e18062 - 80.0);
        let assign18280_e18066: f64 = (assign18280_e18064 * 0.3333333333333);
        let assign18280_e18067: f64 = (1.0 + assign18280_e18066);
        let assign18280_e18068: f64 = (assign18280_e18056 * assign18280_e18067);
        let assign18280_e18069: f64 = (1.0 + assign18280_e18068);
        let assign18280_e18070: f64 = (assign18280_e18046 * assign18280_e18069);
        let assign18280_e18071: f64 = (1.0 + assign18280_e18070);
        let assign18280_e18072: f64 = (5.54062e34 * assign18280_e18071);
        (assign18280_e18072, (5.54062e34 * ((((locals.var_xg1x_dn4 - locals.var_q1d_dn4) - locals.var_xdeff_dn4) * assign18280_e18069) + (assign18280_e18046 * (((0.5 * ((locals.var_xg1x_dn4 - locals.var_q1d_dn4) - locals.var_xdeff_dn4)) * assign18280_e18067) + (assign18280_e18056 * (((locals.var_xg1x_dn4 - locals.var_q1d_dn4) - locals.var_xdeff_dn4) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x_dn6 - locals.var_q1d_dn6) - locals.var_xdeff_dn6) * assign18280_e18069) + (assign18280_e18046 * (((0.5 * ((locals.var_xg1x_dn6 - locals.var_q1d_dn6) - locals.var_xdeff_dn6)) * assign18280_e18067) + (assign18280_e18056 * (((locals.var_xg1x_dn6 - locals.var_q1d_dn6) - locals.var_xdeff_dn6) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x_dn7 - locals.var_q1d_dn7) - locals.var_xdeff_dn7) * assign18280_e18069) + (assign18280_e18046 * (((0.5 * ((locals.var_xg1x_dn7 - locals.var_q1d_dn7) - locals.var_xdeff_dn7)) * assign18280_e18067) + (assign18280_e18056 * (((locals.var_xg1x_dn7 - locals.var_q1d_dn7) - locals.var_xdeff_dn7) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x_dn8 - locals.var_q1d_dn8) - locals.var_xdeff_dn8) * assign18280_e18069) + (assign18280_e18046 * (((0.5 * ((locals.var_xg1x_dn8 - locals.var_q1d_dn8) - locals.var_xdeff_dn8)) * assign18280_e18067) + (assign18280_e18056 * (((locals.var_xg1x_dn8 - locals.var_q1d_dn8) - locals.var_xdeff_dn8) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x_dn9 - locals.var_q1d_dn9) - locals.var_xdeff_dn9) * assign18280_e18069) + (assign18280_e18046 * (((0.5 * ((locals.var_xg1x_dn9 - locals.var_q1d_dn9) - locals.var_xdeff_dn9)) * assign18280_e18067) + (assign18280_e18056 * (((locals.var_xg1x_dn9 - locals.var_q1d_dn9) - locals.var_xdeff_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign18280_e18074;
        locals.var_q_temp1_dn4 = assign18280_e18074_d_n4;
        locals.var_q_temp1_dn6 = assign18280_e18074_d_n6;
        locals.var_q_temp1_dn7 = assign18280_e18074_d_n7;
        locals.var_q_temp1_dn8 = assign18280_e18074_d_n8;
        locals.var_q_temp1_dn9 = assign18280_e18074_d_n9;

        let assign18290_e18077: f64 = (locals.var_a0 * locals.var_q_temp1);
        locals.var_q_aexp = assign18290_e18077;
        locals.var_q_aexp_dn4 = ((locals.var_a0_dn4 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn4));
        locals.var_q_aexp_dn6 = ((locals.var_a0_dn6 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn6));
        locals.var_q_aexp_dn7 = ((locals.var_a0_dn7 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn7));
        locals.var_q_aexp_dn8 = ((locals.var_a0_dn8 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn8));
        locals.var_q_aexp_dn9 = ((locals.var_a0_dn9 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn9));

        let assign18300_e18080: f64 = (locals.var_q_k1q1 * locals.var_q_k1q1);
        let assign18300_e18082: f64 = (assign18300_e18080 - locals.var_q_aexp);
        locals.var_q_qsq = assign18300_e18082;
        locals.var_q_qsq_dn4 = (((locals.var_q_k1q1_dn4 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn4)) - locals.var_q_aexp_dn4);
        locals.var_q_qsq_dn6 = (((locals.var_q_k1q1_dn6 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn6)) - locals.var_q_aexp_dn6);
        locals.var_q_qsq_dn7 = (((locals.var_q_k1q1_dn7 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn7)) - locals.var_q_aexp_dn7);
        locals.var_q_qsq_dn8 = (((locals.var_q_k1q1_dn8 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn8)) - locals.var_q_aexp_dn8);
        locals.var_q_qsq_dn9 = (((locals.var_q_k1q1_dn9 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn9)) - locals.var_q_aexp_dn9);

        let assign18310_e18085: f64 = (2.0 * locals.var_k1);
        let assign18310_e18087: f64 = (assign18310_e18085 * locals.var_q_k1q1);
        let assign18310_e18089: f64 = (assign18310_e18087 + locals.var_q_aexp);
        locals.var_q_d1_qsq = assign18310_e18089;
        locals.var_q_d1_qsq_dn4 = ((((2.0 * locals.var_k1_dn4) * locals.var_q_k1q1) + (assign18310_e18085 * locals.var_q_k1q1_dn4)) + locals.var_q_aexp_dn4);
        locals.var_q_d1_qsq_dn6 = ((((2.0 * locals.var_k1_dn6) * locals.var_q_k1q1) + (assign18310_e18085 * locals.var_q_k1q1_dn6)) + locals.var_q_aexp_dn6);
        locals.var_q_d1_qsq_dn7 = ((((2.0 * locals.var_k1_dn7) * locals.var_q_k1q1) + (assign18310_e18085 * locals.var_q_k1q1_dn7)) + locals.var_q_aexp_dn7);
        locals.var_q_d1_qsq_dn8 = ((((2.0 * locals.var_k1_dn8) * locals.var_q_k1q1) + (assign18310_e18085 * locals.var_q_k1q1_dn8)) + locals.var_q_aexp_dn8);
        locals.var_q_d1_qsq_dn9 = ((((2.0 * locals.var_k1_dn9) * locals.var_q_k1q1) + (assign18310_e18085 * locals.var_q_k1q1_dn9)) + locals.var_q_aexp_dn9);

    }

    pub(super) fn stamp_transient_block_46(
        locals: &mut StampLocals,
    ) {
        let assign18320_e18092: f64 = (2.0 * locals.var_k1);
        let assign18320_e18094: f64 = (assign18320_e18092 * locals.var_k1);
        let assign18320_e18096: f64 = (assign18320_e18094 - locals.var_q_aexp);
        locals.var_q_d2_qsq = assign18320_e18096;
        locals.var_q_d2_qsq_dn4 = ((((2.0 * locals.var_k1_dn4) * locals.var_k1) + (assign18320_e18092 * locals.var_k1_dn4)) - locals.var_q_aexp_dn4);
        locals.var_q_d2_qsq_dn6 = ((((2.0 * locals.var_k1_dn6) * locals.var_k1) + (assign18320_e18092 * locals.var_k1_dn6)) - locals.var_q_aexp_dn6);
        locals.var_q_d2_qsq_dn7 = ((((2.0 * locals.var_k1_dn7) * locals.var_k1) + (assign18320_e18092 * locals.var_k1_dn7)) - locals.var_q_aexp_dn7);
        locals.var_q_d2_qsq_dn8 = ((((2.0 * locals.var_k1_dn8) * locals.var_k1) + (assign18320_e18092 * locals.var_k1_dn8)) - locals.var_q_aexp_dn8);
        locals.var_q_d2_qsq_dn9 = ((((2.0 * locals.var_k1_dn9) * locals.var_k1) + (assign18320_e18092 * locals.var_k1_dn9)) - locals.var_q_aexp_dn9);

        let assign18330_e18099: f64 = (-0.005);
        let assign18330_e18100: f64 = if locals.var_q_qsq < assign18330_e18099 { 1.0 } else { 0.0 };
        locals.var_guard633 = assign18330_e18100;

        let (assign18340_e18106, assign18340_e18106_d_n4, assign18340_e18106_d_n6, assign18340_e18106_d_n7, assign18340_e18106_d_n8, assign18340_e18106_d_n9,) = {
    if (locals.var_guard633 != 0.0) {
        let assign18340_e18103: f64 = (locals.var_q_qsq).abs();
        let assign18340_e18104: f64 = (assign18340_e18103).sqrt();
        (assign18340_e18104, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign18340_e18104)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign18340_e18104)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign18340_e18104)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign18340_e18104)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign18340_e18104)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign18340_e18106;
        locals.var_q_rac_qsq_dn4 = assign18340_e18106_d_n4;
        locals.var_q_rac_qsq_dn6 = assign18340_e18106_d_n6;
        locals.var_q_rac_qsq_dn7 = assign18340_e18106_d_n7;
        locals.var_q_rac_qsq_dn8 = assign18340_e18106_d_n8;
        locals.var_q_rac_qsq_dn9 = assign18340_e18106_d_n9;

        let (assign18350_e18115, assign18350_e18115_d_n4, assign18350_e18115_d_n6, assign18350_e18115_d_n7, assign18350_e18115_d_n8, assign18350_e18115_d_n9,) = {
    if (locals.var_guard633 != 0.0) {
        let assign18350_e18111: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign18350_e18112: f64 = (assign18350_e18111).tan();
        let assign18350_e18113: f64 = (locals.var_q_rac_qsq / assign18350_e18112);
        (assign18350_e18113, (((locals.var_q_rac_qsq_dn4 * assign18350_e18112) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn4) / ((assign18350_e18111).cos() * (assign18350_e18111).cos())))) / (assign18350_e18112 * assign18350_e18112)), (((locals.var_q_rac_qsq_dn6 * assign18350_e18112) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn6) / ((assign18350_e18111).cos() * (assign18350_e18111).cos())))) / (assign18350_e18112 * assign18350_e18112)), (((locals.var_q_rac_qsq_dn7 * assign18350_e18112) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn7) / ((assign18350_e18111).cos() * (assign18350_e18111).cos())))) / (assign18350_e18112 * assign18350_e18112)), (((locals.var_q_rac_qsq_dn8 * assign18350_e18112) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn8) / ((assign18350_e18111).cos() * (assign18350_e18111).cos())))) / (assign18350_e18112 * assign18350_e18112)), (((locals.var_q_rac_qsq_dn9 * assign18350_e18112) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn9) / ((assign18350_e18111).cos() * (assign18350_e18111).cos())))) / (assign18350_e18112 * assign18350_e18112)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign18350_e18115;
        locals.var_q_qcoth_dn4 = assign18350_e18115_d_n4;
        locals.var_q_qcoth_dn6 = assign18350_e18115_d_n6;
        locals.var_q_qcoth_dn7 = assign18350_e18115_d_n7;
        locals.var_q_qcoth_dn8 = assign18350_e18115_d_n8;
        locals.var_q_qcoth_dn9 = assign18350_e18115_d_n9;

        let (assign18360_e18123, assign18360_e18123_d_n4, assign18360_e18123_d_n6, assign18360_e18123_d_n7, assign18360_e18123_d_n8, assign18360_e18123_d_n9,) = {
    if (locals.var_guard633 != 0.0) {
        let assign18360_e18119: f64 = (0.25 * locals.var_q_d1_qsq);
        let assign18360_e18121: f64 = (assign18360_e18119 / locals.var_q_qsq);
        (assign18360_e18121, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign18360_e18119 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign18360_e18119 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign18360_e18119 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign18360_e18119 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign18360_e18119 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign18360_e18123;
        locals.var_q_temp1_dn4 = assign18360_e18123_d_n4;
        locals.var_q_temp1_dn6 = assign18360_e18123_d_n6;
        locals.var_q_temp1_dn7 = assign18360_e18123_d_n7;
        locals.var_q_temp1_dn8 = assign18360_e18123_d_n8;
        locals.var_q_temp1_dn9 = assign18360_e18123_d_n9;

        let (assign18370_e18135, assign18370_e18135_d_n4, assign18370_e18135_d_n6, assign18370_e18135_d_n7, assign18370_e18135_d_n8, assign18370_e18135_d_n9,) = {
    if (locals.var_guard633 != 0.0) {
        let assign18370_e18129: f64 = (2.0 - locals.var_q_qcoth);
        let assign18370_e18130: f64 = (locals.var_q_qcoth * assign18370_e18129);
        let assign18370_e18131: f64 = (locals.var_q_qsq + assign18370_e18130);
        let assign18370_e18133: f64 = (assign18370_e18131 * locals.var_q_temp1);
        (assign18370_e18133, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign18370_e18129) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign18370_e18131 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign18370_e18129) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign18370_e18131 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign18370_e18129) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign18370_e18131 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign18370_e18129) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign18370_e18131 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign18370_e18129) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign18370_e18131 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign18370_e18135;
        locals.var_q_d1_qcoth_dn4 = assign18370_e18135_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign18370_e18135_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign18370_e18135_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign18370_e18135_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign18370_e18135_d_n9;

        let (assign18380_e18155, assign18380_e18155_d_n4, assign18380_e18155_d_n6, assign18380_e18155_d_n7, assign18380_e18155_d_n8, assign18380_e18155_d_n9,) = {
    if (locals.var_guard633 != 0.0) {
        let assign18380_e18140: f64 = (2.0 * locals.var_q_d1_qcoth);
        let assign18380_e18143: f64 = (1.0 + locals.var_q_qcoth);
        let assign18380_e18144: f64 = (assign18380_e18140 * assign18380_e18143);
        let assign18380_e18145: f64 = (locals.var_q_d1_qsq - assign18380_e18144);
        let assign18380_e18147: f64 = (assign18380_e18145 * locals.var_q_temp1);
        let assign18380_e18150: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
        let assign18380_e18152: f64 = (assign18380_e18150 / locals.var_q_d1_qsq);
        let assign18380_e18153: f64 = (assign18380_e18147 + assign18380_e18152);
        (assign18380_e18153, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign18380_e18143) + (assign18380_e18140 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign18380_e18145 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign18380_e18150 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign18380_e18143) + (assign18380_e18140 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign18380_e18145 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign18380_e18150 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign18380_e18143) + (assign18380_e18140 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign18380_e18145 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign18380_e18150 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign18380_e18143) + (assign18380_e18140 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign18380_e18145 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign18380_e18150 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign18380_e18143) + (assign18380_e18140 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign18380_e18145 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign18380_e18150 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign18380_e18155;
        locals.var_q_d2_qcoth_dn4 = assign18380_e18155_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign18380_e18155_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign18380_e18155_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign18380_e18155_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign18380_e18155_d_n9;

        let (assign18390_e18163, assign18390_e18163_d_n4, assign18390_e18163_d_n6, assign18390_e18163_d_n7, assign18390_e18163_d_n8, assign18390_e18163_d_n9,) = {
    if (locals.var_guard633 != 0.0) {
        let assign18390_e18160: f64 = (0.5 * locals.var_q_qcoth);
        let assign18390_e18161: f64 = (1.0 - assign18390_e18160);
        (assign18390_e18161, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign18390_e18163;
        locals.var_q_temp2_dn4 = assign18390_e18163_d_n4;
        locals.var_q_temp2_dn6 = assign18390_e18163_d_n6;
        locals.var_q_temp2_dn7 = assign18390_e18163_d_n7;
        locals.var_q_temp2_dn8 = assign18390_e18163_d_n8;
        locals.var_q_temp2_dn9 = assign18390_e18163_d_n9;

        let (assign18400_e18171, assign18400_e18171_d_n4, assign18400_e18171_d_n6, assign18400_e18171_d_n7, assign18400_e18171_d_n8, assign18400_e18171_d_n9,) = {
    if (locals.var_guard633 != 0.0) {
        let assign18400_e18167: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
        let assign18400_e18169: f64 = (assign18400_e18167 * locals.var_q_temp2);
        (assign18400_e18169, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign18400_e18167 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign18400_e18167 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign18400_e18167 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign18400_e18167 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign18400_e18167 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign18400_e18171;
        locals.var_q_d1_ln_dn4 = assign18400_e18171_d_n4;
        locals.var_q_d1_ln_dn6 = assign18400_e18171_d_n6;
        locals.var_q_d1_ln_dn7 = assign18400_e18171_d_n7;
        locals.var_q_d1_ln_dn8 = assign18400_e18171_d_n8;
        locals.var_q_d1_ln_dn9 = assign18400_e18171_d_n9;

        let (assign18410_e18187, assign18410_e18187_d_n4, assign18410_e18187_d_n6, assign18410_e18187_d_n7, assign18410_e18187_d_n8, assign18410_e18187_d_n9,) = {
    if (locals.var_guard633 != 0.0) {
        let assign18410_e18175: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
        let assign18410_e18180: f64 = (0.5 * locals.var_q_d1_qcoth);
        let assign18410_e18181: f64 = (locals.var_q_d1_ln + assign18410_e18180);
        let assign18410_e18182: f64 = (locals.var_q_d1_qsq * assign18410_e18181);
        let assign18410_e18183: f64 = (assign18410_e18175 - assign18410_e18182);
        let assign18410_e18185: f64 = (assign18410_e18183 / locals.var_q_qsq);
        (assign18410_e18185, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign18410_e18181) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign18410_e18183 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign18410_e18181) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign18410_e18183 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign18410_e18181) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign18410_e18183 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign18410_e18181) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign18410_e18183 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign18410_e18181) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign18410_e18183 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign18410_e18187;
        locals.var_q_d2_ln_dn4 = assign18410_e18187_d_n4;
        locals.var_q_d2_ln_dn6 = assign18410_e18187_d_n6;
        locals.var_q_d2_ln_dn7 = assign18410_e18187_d_n7;
        locals.var_q_d2_ln_dn8 = assign18410_e18187_d_n8;
        locals.var_q_d2_ln_dn9 = assign18410_e18187_d_n9;

        let assign18420_e18190: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard634 = assign18420_e18190;

        let (assign18430_e18199, assign18430_e18199_d_n4, assign18430_e18199_d_n6, assign18430_e18199_d_n7, assign18430_e18199_d_n8, assign18430_e18199_d_n9,) = {
    if ((locals.var_guard633 == 0.0) && (locals.var_guard634 != 0.0)) {
        let assign18430_e18196: f64 = (locals.var_q_qsq).abs();
        let assign18430_e18197: f64 = (assign18430_e18196).sqrt();
        (assign18430_e18197, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign18430_e18197)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign18430_e18197)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign18430_e18197)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign18430_e18197)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign18430_e18197)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign18430_e18199;
        locals.var_q_rac_qsq_dn4 = assign18430_e18199_d_n4;
        locals.var_q_rac_qsq_dn6 = assign18430_e18199_d_n6;
        locals.var_q_rac_qsq_dn7 = assign18430_e18199_d_n7;
        locals.var_q_rac_qsq_dn8 = assign18430_e18199_d_n8;
        locals.var_q_rac_qsq_dn9 = assign18430_e18199_d_n9;

        let (assign18440_e18208, assign18440_e18208_d_n4, assign18440_e18208_d_n6, assign18440_e18208_d_n7, assign18440_e18208_d_n8, assign18440_e18208_d_n9,) = {
    if ((locals.var_guard633 == 0.0) && (locals.var_guard634 != 0.0)) {
        let assign18440_e18205: f64 = (-locals.var_q_rac_qsq);
        let assign18440_e18206: f64 = (assign18440_e18205).exp();
        (assign18440_e18206, (assign18440_e18206 * (-locals.var_q_rac_qsq_dn4)), (assign18440_e18206 * (-locals.var_q_rac_qsq_dn6)), (assign18440_e18206 * (-locals.var_q_rac_qsq_dn7)), (assign18440_e18206 * (-locals.var_q_rac_qsq_dn8)), (assign18440_e18206 * (-locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_invexpq, locals.var_q_invexpq_dn4, locals.var_q_invexpq_dn6, locals.var_q_invexpq_dn7, locals.var_q_invexpq_dn8, locals.var_q_invexpq_dn9,)
    }
};
        locals.var_q_invexpq = assign18440_e18208;
        locals.var_q_invexpq_dn4 = assign18440_e18208_d_n4;
        locals.var_q_invexpq_dn6 = assign18440_e18208_d_n6;
        locals.var_q_invexpq_dn7 = assign18440_e18208_d_n7;
        locals.var_q_invexpq_dn8 = assign18440_e18208_d_n8;
        locals.var_q_invexpq_dn9 = assign18440_e18208_d_n9;

        let (assign18450_e18223, assign18450_e18223_d_n4, assign18450_e18223_d_n6, assign18450_e18223_d_n7, assign18450_e18223_d_n8, assign18450_e18223_d_n9,) = {
    if ((locals.var_guard633 == 0.0) && (locals.var_guard634 != 0.0)) {
        let assign18450_e18216: f64 = (1.0 + locals.var_q_invexpq);
        let assign18450_e18217: f64 = (locals.var_q_rac_qsq * assign18450_e18216);
        let assign18450_e18220: f64 = (1.0 - locals.var_q_invexpq);
        let assign18450_e18221: f64 = (assign18450_e18217 / assign18450_e18220);
        (assign18450_e18221, (((((locals.var_q_rac_qsq_dn4 * assign18450_e18216) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn4)) * assign18450_e18220) - (assign18450_e18217 * (-locals.var_q_invexpq_dn4))) / (assign18450_e18220 * assign18450_e18220)), (((((locals.var_q_rac_qsq_dn6 * assign18450_e18216) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn6)) * assign18450_e18220) - (assign18450_e18217 * (-locals.var_q_invexpq_dn6))) / (assign18450_e18220 * assign18450_e18220)), (((((locals.var_q_rac_qsq_dn7 * assign18450_e18216) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn7)) * assign18450_e18220) - (assign18450_e18217 * (-locals.var_q_invexpq_dn7))) / (assign18450_e18220 * assign18450_e18220)), (((((locals.var_q_rac_qsq_dn8 * assign18450_e18216) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn8)) * assign18450_e18220) - (assign18450_e18217 * (-locals.var_q_invexpq_dn8))) / (assign18450_e18220 * assign18450_e18220)), (((((locals.var_q_rac_qsq_dn9 * assign18450_e18216) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn9)) * assign18450_e18220) - (assign18450_e18217 * (-locals.var_q_invexpq_dn9))) / (assign18450_e18220 * assign18450_e18220)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign18450_e18223;
        locals.var_q_qcoth_dn4 = assign18450_e18223_d_n4;
        locals.var_q_qcoth_dn6 = assign18450_e18223_d_n6;
        locals.var_q_qcoth_dn7 = assign18450_e18223_d_n7;
        locals.var_q_qcoth_dn8 = assign18450_e18223_d_n8;
        locals.var_q_qcoth_dn9 = assign18450_e18223_d_n9;

        let (assign18460_e18234, assign18460_e18234_d_n4, assign18460_e18234_d_n6, assign18460_e18234_d_n7, assign18460_e18234_d_n8, assign18460_e18234_d_n9,) = {
    if ((locals.var_guard633 == 0.0) && (locals.var_guard634 != 0.0)) {
        let assign18460_e18230: f64 = (0.25 * locals.var_q_d1_qsq);
        let assign18460_e18232: f64 = (assign18460_e18230 / locals.var_q_qsq);
        (assign18460_e18232, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign18460_e18230 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign18460_e18230 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign18460_e18230 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign18460_e18230 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign18460_e18230 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign18460_e18234;
        locals.var_q_temp1_dn4 = assign18460_e18234_d_n4;
        locals.var_q_temp1_dn6 = assign18460_e18234_d_n6;
        locals.var_q_temp1_dn7 = assign18460_e18234_d_n7;
        locals.var_q_temp1_dn8 = assign18460_e18234_d_n8;
        locals.var_q_temp1_dn9 = assign18460_e18234_d_n9;

        let (assign18470_e18249, assign18470_e18249_d_n4, assign18470_e18249_d_n6, assign18470_e18249_d_n7, assign18470_e18249_d_n8, assign18470_e18249_d_n9,) = {
    if ((locals.var_guard633 == 0.0) && (locals.var_guard634 != 0.0)) {
        let assign18470_e18243: f64 = (2.0 - locals.var_q_qcoth);
        let assign18470_e18244: f64 = (locals.var_q_qcoth * assign18470_e18243);
        let assign18470_e18245: f64 = (locals.var_q_qsq + assign18470_e18244);
        let assign18470_e18247: f64 = (assign18470_e18245 * locals.var_q_temp1);
        (assign18470_e18247, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign18470_e18243) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign18470_e18245 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign18470_e18243) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign18470_e18245 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign18470_e18243) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign18470_e18245 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign18470_e18243) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign18470_e18245 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign18470_e18243) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign18470_e18245 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign18470_e18249;
        locals.var_q_d1_qcoth_dn4 = assign18470_e18249_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign18470_e18249_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign18470_e18249_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign18470_e18249_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign18470_e18249_d_n9;

        let (assign18480_e18272, assign18480_e18272_d_n4, assign18480_e18272_d_n6, assign18480_e18272_d_n7, assign18480_e18272_d_n8, assign18480_e18272_d_n9,) = {
    if ((locals.var_guard633 == 0.0) && (locals.var_guard634 != 0.0)) {
        let assign18480_e18257: f64 = (2.0 * locals.var_q_d1_qcoth);
        let assign18480_e18260: f64 = (1.0 + locals.var_q_qcoth);
        let assign18480_e18261: f64 = (assign18480_e18257 * assign18480_e18260);
        let assign18480_e18262: f64 = (locals.var_q_d1_qsq - assign18480_e18261);
        let assign18480_e18264: f64 = (assign18480_e18262 * locals.var_q_temp1);
        let assign18480_e18267: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
        let assign18480_e18269: f64 = (assign18480_e18267 / locals.var_q_d1_qsq);
        let assign18480_e18270: f64 = (assign18480_e18264 + assign18480_e18269);
        (assign18480_e18270, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign18480_e18260) + (assign18480_e18257 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign18480_e18262 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign18480_e18267 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign18480_e18260) + (assign18480_e18257 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign18480_e18262 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign18480_e18267 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign18480_e18260) + (assign18480_e18257 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign18480_e18262 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign18480_e18267 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign18480_e18260) + (assign18480_e18257 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign18480_e18262 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign18480_e18267 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign18480_e18260) + (assign18480_e18257 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign18480_e18262 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign18480_e18267 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign18480_e18272;
        locals.var_q_d2_qcoth_dn4 = assign18480_e18272_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign18480_e18272_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign18480_e18272_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign18480_e18272_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign18480_e18272_d_n9;

        let (assign18490_e18283, assign18490_e18283_d_n4, assign18490_e18283_d_n6, assign18490_e18283_d_n7, assign18490_e18283_d_n8, assign18490_e18283_d_n9,) = {
    if ((locals.var_guard633 == 0.0) && (locals.var_guard634 != 0.0)) {
        let assign18490_e18280: f64 = (0.5 * locals.var_q_qcoth);
        let assign18490_e18281: f64 = (1.0 - assign18490_e18280);
        (assign18490_e18281, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign18490_e18283;
        locals.var_q_temp2_dn4 = assign18490_e18283_d_n4;
        locals.var_q_temp2_dn6 = assign18490_e18283_d_n6;
        locals.var_q_temp2_dn7 = assign18490_e18283_d_n7;
        locals.var_q_temp2_dn8 = assign18490_e18283_d_n8;
        locals.var_q_temp2_dn9 = assign18490_e18283_d_n9;

        let (assign18500_e18294, assign18500_e18294_d_n4, assign18500_e18294_d_n6, assign18500_e18294_d_n7, assign18500_e18294_d_n8, assign18500_e18294_d_n9,) = {
    if ((locals.var_guard633 == 0.0) && (locals.var_guard634 != 0.0)) {
        let assign18500_e18290: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
        let assign18500_e18292: f64 = (assign18500_e18290 * locals.var_q_temp2);
        (assign18500_e18292, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign18500_e18290 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign18500_e18290 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign18500_e18290 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign18500_e18290 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign18500_e18290 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign18500_e18294;
        locals.var_q_d1_ln_dn4 = assign18500_e18294_d_n4;
        locals.var_q_d1_ln_dn6 = assign18500_e18294_d_n6;
        locals.var_q_d1_ln_dn7 = assign18500_e18294_d_n7;
        locals.var_q_d1_ln_dn8 = assign18500_e18294_d_n8;
        locals.var_q_d1_ln_dn9 = assign18500_e18294_d_n9;

        let (assign18510_e18313, assign18510_e18313_d_n4, assign18510_e18313_d_n6, assign18510_e18313_d_n7, assign18510_e18313_d_n8, assign18510_e18313_d_n9,) = {
    if ((locals.var_guard633 == 0.0) && (locals.var_guard634 != 0.0)) {
        let assign18510_e18301: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
        let assign18510_e18306: f64 = (0.5 * locals.var_q_d1_qcoth);
        let assign18510_e18307: f64 = (locals.var_q_d1_ln + assign18510_e18306);
        let assign18510_e18308: f64 = (locals.var_q_d1_qsq * assign18510_e18307);
        let assign18510_e18309: f64 = (assign18510_e18301 - assign18510_e18308);
        let assign18510_e18311: f64 = (assign18510_e18309 / locals.var_q_qsq);
        (assign18510_e18311, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign18510_e18307) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign18510_e18309 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign18510_e18307) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign18510_e18309 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign18510_e18307) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign18510_e18309 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign18510_e18307) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign18510_e18309 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign18510_e18307) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign18510_e18309 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign18510_e18313;
        locals.var_q_d2_ln_dn4 = assign18510_e18313_d_n4;
        locals.var_q_d2_ln_dn6 = assign18510_e18313_d_n6;
        locals.var_q_d2_ln_dn7 = assign18510_e18313_d_n7;
        locals.var_q_d2_ln_dn8 = assign18510_e18313_d_n8;
        locals.var_q_d2_ln_dn9 = assign18510_e18313_d_n9;

        let (assign18520_e18339, assign18520_e18339_d_n4, assign18520_e18339_d_n6, assign18520_e18339_d_n7, assign18520_e18339_d_n8, assign18520_e18339_d_n9,) = {
    if ((locals.var_guard633 == 0.0) && (locals.var_guard634 == 0.0)) {
        let assign18520_e18323: f64 = (locals.var_q_qsq * 0.0166666666667);
        let assign18520_e18327: f64 = (locals.var_q_qsq * 0.0238095238095);
        let assign18520_e18331: f64 = (locals.var_q_qsq * 0.025);
        let assign18520_e18332: f64 = (1.0 - assign18520_e18331);
        let assign18520_e18333: f64 = (assign18520_e18327 * assign18520_e18332);
        let assign18520_e18334: f64 = (1.0 - assign18520_e18333);
        let assign18520_e18335: f64 = (assign18520_e18323 * assign18520_e18334);
        let assign18520_e18336: f64 = (1.0 - assign18520_e18335);
        let assign18520_e18337: f64 = (0.1666666666667 * assign18520_e18336);
        (assign18520_e18337, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0166666666667) * assign18520_e18334) + (assign18520_e18323 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign18520_e18332) + (assign18520_e18327 * (-(locals.var_q_qsq_dn4 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0166666666667) * assign18520_e18334) + (assign18520_e18323 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign18520_e18332) + (assign18520_e18327 * (-(locals.var_q_qsq_dn6 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0166666666667) * assign18520_e18334) + (assign18520_e18323 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign18520_e18332) + (assign18520_e18327 * (-(locals.var_q_qsq_dn7 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0166666666667) * assign18520_e18334) + (assign18520_e18323 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign18520_e18332) + (assign18520_e18327 * (-(locals.var_q_qsq_dn8 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0166666666667) * assign18520_e18334) + (assign18520_e18323 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign18520_e18332) + (assign18520_e18327 * (-(locals.var_q_qsq_dn9 * 0.025))))))))),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign18520_e18339;
        locals.var_q_temp3_dn4 = assign18520_e18339_d_n4;
        locals.var_q_temp3_dn6 = assign18520_e18339_d_n6;
        locals.var_q_temp3_dn7 = assign18520_e18339_d_n7;
        locals.var_q_temp3_dn8 = assign18520_e18339_d_n8;
        locals.var_q_temp3_dn9 = assign18520_e18339_d_n9;

        let (assign18530_e18351, assign18530_e18351_d_n4, assign18530_e18351_d_n6, assign18530_e18351_d_n7, assign18530_e18351_d_n8, assign18530_e18351_d_n9,) = {
    if ((locals.var_guard633 == 0.0) && (locals.var_guard634 == 0.0)) {
        let assign18530_e18348: f64 = (locals.var_q_qsq * locals.var_q_temp3);
        let assign18530_e18349: f64 = (2.0 + assign18530_e18348);
        (assign18530_e18349, ((locals.var_q_qsq_dn4 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn4)), ((locals.var_q_qsq_dn6 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn6)), ((locals.var_q_qsq_dn7 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn7)), ((locals.var_q_qsq_dn8 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn8)), ((locals.var_q_qsq_dn9 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn9)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign18530_e18351;
        locals.var_q_qcoth_dn4 = assign18530_e18351_d_n4;
        locals.var_q_qcoth_dn6 = assign18530_e18351_d_n6;
        locals.var_q_qcoth_dn7 = assign18530_e18351_d_n7;
        locals.var_q_qcoth_dn8 = assign18530_e18351_d_n8;
        locals.var_q_qcoth_dn9 = assign18530_e18351_d_n9;

        let (assign18540_e18377, assign18540_e18377_d_n4, assign18540_e18377_d_n6, assign18540_e18377_d_n7, assign18540_e18377_d_n8, assign18540_e18377_d_n9,) = {
    if ((locals.var_guard633 == 0.0) && (locals.var_guard634 == 0.0)) {
        let assign18540_e18361: f64 = (locals.var_q_qsq * 0.0333333333333);
        let assign18540_e18365: f64 = (locals.var_q_qsq * 0.0357142857143);
        let assign18540_e18369: f64 = (locals.var_q_qsq * 0.0333333333333);
        let assign18540_e18370: f64 = (1.0 - assign18540_e18369);
        let assign18540_e18371: f64 = (assign18540_e18365 * assign18540_e18370);
        let assign18540_e18372: f64 = (1.0 - assign18540_e18371);
        let assign18540_e18373: f64 = (assign18540_e18361 * assign18540_e18372);
        let assign18540_e18374: f64 = (1.0 - assign18540_e18373);
        let assign18540_e18375: f64 = (0.1666666666667 * assign18540_e18374);
        (assign18540_e18375, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0333333333333) * assign18540_e18372) + (assign18540_e18361 * (-(((locals.var_q_qsq_dn4 * 0.0357142857143) * assign18540_e18370) + (assign18540_e18365 * (-(locals.var_q_qsq_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0333333333333) * assign18540_e18372) + (assign18540_e18361 * (-(((locals.var_q_qsq_dn6 * 0.0357142857143) * assign18540_e18370) + (assign18540_e18365 * (-(locals.var_q_qsq_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0333333333333) * assign18540_e18372) + (assign18540_e18361 * (-(((locals.var_q_qsq_dn7 * 0.0357142857143) * assign18540_e18370) + (assign18540_e18365 * (-(locals.var_q_qsq_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0333333333333) * assign18540_e18372) + (assign18540_e18361 * (-(((locals.var_q_qsq_dn8 * 0.0357142857143) * assign18540_e18370) + (assign18540_e18365 * (-(locals.var_q_qsq_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0333333333333) * assign18540_e18372) + (assign18540_e18361 * (-(((locals.var_q_qsq_dn9 * 0.0357142857143) * assign18540_e18370) + (assign18540_e18365 * (-(locals.var_q_qsq_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign18540_e18377;
        locals.var_q_temp1_dn4 = assign18540_e18377_d_n4;
        locals.var_q_temp1_dn6 = assign18540_e18377_d_n6;
        locals.var_q_temp1_dn7 = assign18540_e18377_d_n7;
        locals.var_q_temp1_dn8 = assign18540_e18377_d_n8;
        locals.var_q_temp1_dn9 = assign18540_e18377_d_n9;

        let (assign18550_e18387, assign18550_e18387_d_n4, assign18550_e18387_d_n6, assign18550_e18387_d_n7, assign18550_e18387_d_n8, assign18550_e18387_d_n9,) = {
    if ((locals.var_guard633 == 0.0) && (locals.var_guard634 == 0.0)) {
        let assign18550_e18385: f64 = (locals.var_q_d1_qsq * locals.var_q_temp1);
        (assign18550_e18385, ((locals.var_q_d1_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn4)), ((locals.var_q_d1_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn6)), ((locals.var_q_d1_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn7)), ((locals.var_q_d1_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn8)), ((locals.var_q_d1_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign18550_e18387;
        locals.var_q_d1_qcoth_dn4 = assign18550_e18387_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign18550_e18387_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign18550_e18387_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign18550_e18387_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign18550_e18387_d_n9;

        let (assign18560_e18413, assign18560_e18413_d_n4, assign18560_e18413_d_n6, assign18560_e18413_d_n7, assign18560_e18413_d_n8, assign18560_e18413_d_n9,) = {
    if ((locals.var_guard633 == 0.0) && (locals.var_guard634 == 0.0)) {
        let assign18560_e18397: f64 = (locals.var_q_qsq * 0.0714285714286);
        let assign18560_e18401: f64 = (0.05 * locals.var_q_qsq);
        let assign18560_e18405: f64 = (0.0420875420875421 * locals.var_q_qsq);
        let assign18560_e18406: f64 = (1.0 - assign18560_e18405);
        let assign18560_e18407: f64 = (assign18560_e18401 * assign18560_e18406);
        let assign18560_e18408: f64 = (1.0 - assign18560_e18407);
        let assign18560_e18409: f64 = (assign18560_e18397 * assign18560_e18408);
        let assign18560_e18410: f64 = (1.0 - assign18560_e18409);
        let assign18560_e18411: f64 = (0.0055555555556 * assign18560_e18410);
        (assign18560_e18411, (0.0055555555556 * (-(((locals.var_q_qsq_dn4 * 0.0714285714286) * assign18560_e18408) + (assign18560_e18397 * (-(((0.05 * locals.var_q_qsq_dn4) * assign18560_e18406) + (assign18560_e18401 * (-(0.0420875420875421 * locals.var_q_qsq_dn4))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn6 * 0.0714285714286) * assign18560_e18408) + (assign18560_e18397 * (-(((0.05 * locals.var_q_qsq_dn6) * assign18560_e18406) + (assign18560_e18401 * (-(0.0420875420875421 * locals.var_q_qsq_dn6))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn7 * 0.0714285714286) * assign18560_e18408) + (assign18560_e18397 * (-(((0.05 * locals.var_q_qsq_dn7) * assign18560_e18406) + (assign18560_e18401 * (-(0.0420875420875421 * locals.var_q_qsq_dn7))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn8 * 0.0714285714286) * assign18560_e18408) + (assign18560_e18397 * (-(((0.05 * locals.var_q_qsq_dn8) * assign18560_e18406) + (assign18560_e18401 * (-(0.0420875420875421 * locals.var_q_qsq_dn8))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn9 * 0.0714285714286) * assign18560_e18408) + (assign18560_e18397 * (-(((0.05 * locals.var_q_qsq_dn9) * assign18560_e18406) + (assign18560_e18401 * (-(0.0420875420875421 * locals.var_q_qsq_dn9))))))))),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign18560_e18413;
        locals.var_q_temp2_dn4 = assign18560_e18413_d_n4;
        locals.var_q_temp2_dn6 = assign18560_e18413_d_n6;
        locals.var_q_temp2_dn7 = assign18560_e18413_d_n7;
        locals.var_q_temp2_dn8 = assign18560_e18413_d_n8;
        locals.var_q_temp2_dn9 = assign18560_e18413_d_n9;

        let (assign18570_e18429, assign18570_e18429_d_n4, assign18570_e18429_d_n6, assign18570_e18429_d_n7, assign18570_e18429_d_n8, assign18570_e18429_d_n9,) = {
    if ((locals.var_guard633 == 0.0) && (locals.var_guard634 == 0.0)) {
        let assign18570_e18421: f64 = (locals.var_q_d2_qsq * locals.var_q_temp1);
        let assign18570_e18424: f64 = (locals.var_q_d1_qsq * locals.var_q_d1_qsq);
        let assign18570_e18426: f64 = (assign18570_e18424 * locals.var_q_temp2);
        let assign18570_e18427: f64 = (assign18570_e18421 - assign18570_e18426);
        (assign18570_e18427, (((locals.var_q_d2_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn4)) - ((((locals.var_q_d1_qsq_dn4 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn4)) * locals.var_q_temp2) + (assign18570_e18424 * locals.var_q_temp2_dn4))), (((locals.var_q_d2_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn6)) - ((((locals.var_q_d1_qsq_dn6 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn6)) * locals.var_q_temp2) + (assign18570_e18424 * locals.var_q_temp2_dn6))), (((locals.var_q_d2_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn7)) - ((((locals.var_q_d1_qsq_dn7 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn7)) * locals.var_q_temp2) + (assign18570_e18424 * locals.var_q_temp2_dn7))), (((locals.var_q_d2_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn8)) - ((((locals.var_q_d1_qsq_dn8 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn8)) * locals.var_q_temp2) + (assign18570_e18424 * locals.var_q_temp2_dn8))), (((locals.var_q_d2_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn9)) - ((((locals.var_q_d1_qsq_dn9 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn9)) * locals.var_q_temp2) + (assign18570_e18424 * locals.var_q_temp2_dn9))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign18570_e18429;
        locals.var_q_d2_qcoth_dn4 = assign18570_e18429_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign18570_e18429_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign18570_e18429_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign18570_e18429_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign18570_e18429_d_n9;

        let (assign18580_e18442, assign18580_e18442_d_n4, assign18580_e18442_d_n6, assign18580_e18442_d_n7, assign18580_e18442_d_n8, assign18580_e18442_d_n9,) = {
    if ((locals.var_guard633 == 0.0) && (locals.var_guard634 == 0.0)) {
        let assign18580_e18436: f64 = (-0.5);
        let assign18580_e18438: f64 = (assign18580_e18436 * locals.var_q_d1_qsq);
        let assign18580_e18440: f64 = (assign18580_e18438 * locals.var_q_temp3);
        (assign18580_e18440, (((assign18580_e18436 * locals.var_q_d1_qsq_dn4) * locals.var_q_temp3) + (assign18580_e18438 * locals.var_q_temp3_dn4)), (((assign18580_e18436 * locals.var_q_d1_qsq_dn6) * locals.var_q_temp3) + (assign18580_e18438 * locals.var_q_temp3_dn6)), (((assign18580_e18436 * locals.var_q_d1_qsq_dn7) * locals.var_q_temp3) + (assign18580_e18438 * locals.var_q_temp3_dn7)), (((assign18580_e18436 * locals.var_q_d1_qsq_dn8) * locals.var_q_temp3) + (assign18580_e18438 * locals.var_q_temp3_dn8)), (((assign18580_e18436 * locals.var_q_d1_qsq_dn9) * locals.var_q_temp3) + (assign18580_e18438 * locals.var_q_temp3_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign18580_e18442;
        locals.var_q_d1_ln_dn4 = assign18580_e18442_d_n4;
        locals.var_q_d1_ln_dn6 = assign18580_e18442_d_n6;
        locals.var_q_d1_ln_dn7 = assign18580_e18442_d_n7;
        locals.var_q_d1_ln_dn8 = assign18580_e18442_d_n8;
        locals.var_q_d1_ln_dn9 = assign18580_e18442_d_n9;

        let (assign18590_e18475, assign18590_e18475_d_n4, assign18590_e18475_d_n6, assign18590_e18475_d_n7, assign18590_e18475_d_n8, assign18590_e18475_d_n9,) = {
    if ((locals.var_guard633 == 0.0) && (locals.var_guard634 == 0.0)) {
        let assign18590_e18449: f64 = (-0.5);
        let assign18590_e18451: f64 = (assign18590_e18449 * locals.var_q_d2_qsq);
        let assign18590_e18453: f64 = (assign18590_e18451 * locals.var_q_temp3);
        let assign18590_e18456: f64 = (0.25 * 0.0055555555556);
        let assign18590_e18458: f64 = (assign18590_e18456 * locals.var_q_d1_qsq);
        let assign18590_e18460: f64 = (assign18590_e18458 * locals.var_q_d1_qsq);
        let assign18590_e18464: f64 = (locals.var_q_qsq * 0.0238095238095);
        let assign18590_e18468: f64 = (0.075 * locals.var_q_qsq);
        let assign18590_e18469: f64 = (2.0 - assign18590_e18468);
        let assign18590_e18470: f64 = (assign18590_e18464 * assign18590_e18469);
        let assign18590_e18471: f64 = (1.0 - assign18590_e18470);
        let assign18590_e18472: f64 = (assign18590_e18460 * assign18590_e18471);
        let assign18590_e18473: f64 = (assign18590_e18453 + assign18590_e18472);
        (assign18590_e18473, ((((assign18590_e18449 * locals.var_q_d2_qsq_dn4) * locals.var_q_temp3) + (assign18590_e18451 * locals.var_q_temp3_dn4)) + (((((assign18590_e18456 * locals.var_q_d1_qsq_dn4) * locals.var_q_d1_qsq) + (assign18590_e18458 * locals.var_q_d1_qsq_dn4)) * assign18590_e18471) + (assign18590_e18460 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign18590_e18469) + (assign18590_e18464 * (-(0.075 * locals.var_q_qsq_dn4)))))))), ((((assign18590_e18449 * locals.var_q_d2_qsq_dn6) * locals.var_q_temp3) + (assign18590_e18451 * locals.var_q_temp3_dn6)) + (((((assign18590_e18456 * locals.var_q_d1_qsq_dn6) * locals.var_q_d1_qsq) + (assign18590_e18458 * locals.var_q_d1_qsq_dn6)) * assign18590_e18471) + (assign18590_e18460 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign18590_e18469) + (assign18590_e18464 * (-(0.075 * locals.var_q_qsq_dn6)))))))), ((((assign18590_e18449 * locals.var_q_d2_qsq_dn7) * locals.var_q_temp3) + (assign18590_e18451 * locals.var_q_temp3_dn7)) + (((((assign18590_e18456 * locals.var_q_d1_qsq_dn7) * locals.var_q_d1_qsq) + (assign18590_e18458 * locals.var_q_d1_qsq_dn7)) * assign18590_e18471) + (assign18590_e18460 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign18590_e18469) + (assign18590_e18464 * (-(0.075 * locals.var_q_qsq_dn7)))))))), ((((assign18590_e18449 * locals.var_q_d2_qsq_dn8) * locals.var_q_temp3) + (assign18590_e18451 * locals.var_q_temp3_dn8)) + (((((assign18590_e18456 * locals.var_q_d1_qsq_dn8) * locals.var_q_d1_qsq) + (assign18590_e18458 * locals.var_q_d1_qsq_dn8)) * assign18590_e18471) + (assign18590_e18460 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign18590_e18469) + (assign18590_e18464 * (-(0.075 * locals.var_q_qsq_dn8)))))))), ((((assign18590_e18449 * locals.var_q_d2_qsq_dn9) * locals.var_q_temp3) + (assign18590_e18451 * locals.var_q_temp3_dn9)) + (((((assign18590_e18456 * locals.var_q_d1_qsq_dn9) * locals.var_q_d1_qsq) + (assign18590_e18458 * locals.var_q_d1_qsq_dn9)) * assign18590_e18471) + (assign18590_e18460 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign18590_e18469) + (assign18590_e18464 * (-(0.075 * locals.var_q_qsq_dn9)))))))),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign18590_e18475;
        locals.var_q_d2_ln_dn4 = assign18590_e18475_d_n4;
        locals.var_q_d2_ln_dn6 = assign18590_e18475_d_n6;
        locals.var_q_d2_ln_dn7 = assign18590_e18475_d_n7;
        locals.var_q_d2_ln_dn8 = assign18590_e18475_d_n8;
        locals.var_q_d2_ln_dn9 = assign18590_e18475_d_n9;

        let assign18600_e18478: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard635 = assign18600_e18478;

        let (assign18610_e18492, assign18610_e18492_d_n4, assign18610_e18492_d_n6, assign18610_e18492_d_n7, assign18610_e18492_d_n8, assign18610_e18492_d_n9,) = {
    if (locals.var_guard635 != 0.0) {
        let assign18610_e18482: f64 = (4.0 * locals.var_q_qsq);
        let assign18610_e18487: f64 = (2.0 - locals.var_q_invexpq);
        let assign18610_e18488: f64 = (locals.var_q_invexpq * assign18610_e18487);
        let assign18610_e18489: f64 = (1.0 - assign18610_e18488);
        let assign18610_e18490: f64 = (assign18610_e18482 / assign18610_e18489);
        (assign18610_e18490, ((((4.0 * locals.var_q_qsq_dn4) * assign18610_e18489) - (assign18610_e18482 * (-((locals.var_q_invexpq_dn4 * assign18610_e18487) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn4)))))) / (assign18610_e18489 * assign18610_e18489)), ((((4.0 * locals.var_q_qsq_dn6) * assign18610_e18489) - (assign18610_e18482 * (-((locals.var_q_invexpq_dn6 * assign18610_e18487) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn6)))))) / (assign18610_e18489 * assign18610_e18489)), ((((4.0 * locals.var_q_qsq_dn7) * assign18610_e18489) - (assign18610_e18482 * (-((locals.var_q_invexpq_dn7 * assign18610_e18487) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn7)))))) / (assign18610_e18489 * assign18610_e18489)), ((((4.0 * locals.var_q_qsq_dn8) * assign18610_e18489) - (assign18610_e18482 * (-((locals.var_q_invexpq_dn8 * assign18610_e18487) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn8)))))) / (assign18610_e18489 * assign18610_e18489)), ((((4.0 * locals.var_q_qsq_dn9) * assign18610_e18489) - (assign18610_e18482 * (-((locals.var_q_invexpq_dn9 * assign18610_e18487) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn9)))))) / (assign18610_e18489 * assign18610_e18489)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign18610_e18492;
        locals.var_q_temp2_dn4 = assign18610_e18492_d_n4;
        locals.var_q_temp2_dn6 = assign18610_e18492_d_n6;
        locals.var_q_temp2_dn7 = assign18610_e18492_d_n7;
        locals.var_q_temp2_dn8 = assign18610_e18492_d_n8;
        locals.var_q_temp2_dn9 = assign18610_e18492_d_n9;

    }

    pub(super) fn stamp_transient_block_47(
        locals: &mut StampLocals,
    ) {
        let (assign18620_e18498, assign18620_e18498_d_n4, assign18620_e18498_d_n6, assign18620_e18498_d_n7, assign18620_e18498_d_n8, assign18620_e18498_d_n9,) = {
    if (locals.var_guard635 != 0.0) {
        let assign18620_e18496: f64 = (locals.var_q_temp2 * locals.var_q_invexpq);
        (assign18620_e18496, ((locals.var_q_temp2_dn4 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn4)), ((locals.var_q_temp2_dn6 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn6)), ((locals.var_q_temp2_dn7 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn7)), ((locals.var_q_temp2_dn8 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn8)), ((locals.var_q_temp2_dn9 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn9)),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign18620_e18498;
        locals.var_q_sh_term_dn4 = assign18620_e18498_d_n4;
        locals.var_q_sh_term_dn6 = assign18620_e18498_d_n6;
        locals.var_q_sh_term_dn7 = assign18620_e18498_d_n7;
        locals.var_q_sh_term_dn8 = assign18620_e18498_d_n8;
        locals.var_q_sh_term_dn9 = assign18620_e18498_d_n9;

        let (assign18630_e18505, assign18630_e18505_d_n4, assign18630_e18505_d_n6, assign18630_e18505_d_n7, assign18630_e18505_d_n8, assign18630_e18505_d_n9,) = {
    if (locals.var_guard635 != 0.0) {
        let assign18630_e18501: f64 = (locals.var_q_temp2).ln();
        let assign18630_e18503: f64 = (assign18630_e18501 - locals.var_q_rac_qsq);
        (assign18630_e18503, ((locals.var_q_temp2_dn4 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn4), ((locals.var_q_temp2_dn6 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn6), ((locals.var_q_temp2_dn7 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn7), ((locals.var_q_temp2_dn8 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn8), ((locals.var_q_temp2_dn9 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn9),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign18630_e18505;
        locals.var_q_ln_term_dn4 = assign18630_e18505_d_n4;
        locals.var_q_ln_term_dn6 = assign18630_e18505_d_n6;
        locals.var_q_ln_term_dn7 = assign18630_e18505_d_n7;
        locals.var_q_ln_term_dn8 = assign18630_e18505_d_n8;
        locals.var_q_ln_term_dn9 = assign18630_e18505_d_n9;

        let assign18640_e18508: f64 = (-0.005);
        let assign18640_e18509: f64 = if locals.var_q_qsq < assign18640_e18508 { 1.0 } else { 0.0 };
        locals.var_guard636 = assign18640_e18509;

        let (assign18650_e18519, assign18650_e18519_d_n4, assign18650_e18519_d_n6, assign18650_e18519_d_n7, assign18650_e18519_d_n8, assign18650_e18519_d_n9,) = {
    if ((locals.var_guard635 == 0.0) && (locals.var_guard636 != 0.0)) {
        let assign18650_e18516: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign18650_e18517: f64 = (assign18650_e18516).sin();
        (assign18650_e18517, ((assign18650_e18516).cos() * (0.5 * locals.var_q_rac_qsq_dn4)), ((assign18650_e18516).cos() * (0.5 * locals.var_q_rac_qsq_dn6)), ((assign18650_e18516).cos() * (0.5 * locals.var_q_rac_qsq_dn7)), ((assign18650_e18516).cos() * (0.5 * locals.var_q_rac_qsq_dn8)), ((assign18650_e18516).cos() * (0.5 * locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign18650_e18519;
        locals.var_q_temp2_dn4 = assign18650_e18519_d_n4;
        locals.var_q_temp2_dn6 = assign18650_e18519_d_n6;
        locals.var_q_temp2_dn7 = assign18650_e18519_d_n7;
        locals.var_q_temp2_dn8 = assign18650_e18519_d_n8;
        locals.var_q_temp2_dn9 = assign18650_e18519_d_n9;

        let (assign18660_e18531, assign18660_e18531_d_n4, assign18660_e18531_d_n6, assign18660_e18531_d_n7, assign18660_e18531_d_n8, assign18660_e18531_d_n9,) = {
    if ((locals.var_guard635 == 0.0) && (locals.var_guard636 != 0.0)) {
        let assign18660_e18525: f64 = (-locals.var_q_qsq);
        let assign18660_e18528: f64 = (locals.var_q_temp2 * locals.var_q_temp2);
        let assign18660_e18529: f64 = (assign18660_e18525 / assign18660_e18528);
        (assign18660_e18529, ((((-locals.var_q_qsq_dn4) * assign18660_e18528) - (assign18660_e18525 * ((locals.var_q_temp2_dn4 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn4)))) / (assign18660_e18528 * assign18660_e18528)), ((((-locals.var_q_qsq_dn6) * assign18660_e18528) - (assign18660_e18525 * ((locals.var_q_temp2_dn6 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn6)))) / (assign18660_e18528 * assign18660_e18528)), ((((-locals.var_q_qsq_dn7) * assign18660_e18528) - (assign18660_e18525 * ((locals.var_q_temp2_dn7 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn7)))) / (assign18660_e18528 * assign18660_e18528)), ((((-locals.var_q_qsq_dn8) * assign18660_e18528) - (assign18660_e18525 * ((locals.var_q_temp2_dn8 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn8)))) / (assign18660_e18528 * assign18660_e18528)), ((((-locals.var_q_qsq_dn9) * assign18660_e18528) - (assign18660_e18525 * ((locals.var_q_temp2_dn9 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn9)))) / (assign18660_e18528 * assign18660_e18528)),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign18660_e18531;
        locals.var_q_sh_term_dn4 = assign18660_e18531_d_n4;
        locals.var_q_sh_term_dn6 = assign18660_e18531_d_n6;
        locals.var_q_sh_term_dn7 = assign18660_e18531_d_n7;
        locals.var_q_sh_term_dn8 = assign18660_e18531_d_n8;
        locals.var_q_sh_term_dn9 = assign18660_e18531_d_n9;

        let (assign18670_e18539, assign18670_e18539_d_n4, assign18670_e18539_d_n6, assign18670_e18539_d_n7, assign18670_e18539_d_n8, assign18670_e18539_d_n9,) = {
    if ((locals.var_guard635 == 0.0) && (locals.var_guard636 != 0.0)) {
        let assign18670_e18537: f64 = (locals.var_q_sh_term).ln();
        (assign18670_e18537, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign18670_e18539;
        locals.var_q_ln_term_dn4 = assign18670_e18539_d_n4;
        locals.var_q_ln_term_dn6 = assign18670_e18539_d_n6;
        locals.var_q_ln_term_dn7 = assign18670_e18539_d_n7;
        locals.var_q_ln_term_dn8 = assign18670_e18539_d_n8;
        locals.var_q_ln_term_dn9 = assign18670_e18539_d_n9;

        let (assign18680_e18563, assign18680_e18563_d_n4, assign18680_e18563_d_n6, assign18680_e18563_d_n7, assign18680_e18563_d_n8, assign18680_e18563_d_n9,) = {
    if ((locals.var_guard635 == 0.0) && (locals.var_guard636 == 0.0)) {
        let assign18680_e18548: f64 = (locals.var_q_qsq * 0.3333333333333);
        let assign18680_e18552: f64 = (0.05 * locals.var_q_qsq);
        let assign18680_e18556: f64 = (0.0396825396825397 * locals.var_q_qsq);
        let assign18680_e18557: f64 = (1.0 - assign18680_e18556);
        let assign18680_e18558: f64 = (assign18680_e18552 * assign18680_e18557);
        let assign18680_e18559: f64 = (1.0 - assign18680_e18558);
        let assign18680_e18560: f64 = (assign18680_e18548 * assign18680_e18559);
        let assign18680_e18561: f64 = (4.0 - assign18680_e18560);
        (assign18680_e18561, (-(((locals.var_q_qsq_dn4 * 0.3333333333333) * assign18680_e18559) + (assign18680_e18548 * (-(((0.05 * locals.var_q_qsq_dn4) * assign18680_e18557) + (assign18680_e18552 * (-(0.0396825396825397 * locals.var_q_qsq_dn4)))))))), (-(((locals.var_q_qsq_dn6 * 0.3333333333333) * assign18680_e18559) + (assign18680_e18548 * (-(((0.05 * locals.var_q_qsq_dn6) * assign18680_e18557) + (assign18680_e18552 * (-(0.0396825396825397 * locals.var_q_qsq_dn6)))))))), (-(((locals.var_q_qsq_dn7 * 0.3333333333333) * assign18680_e18559) + (assign18680_e18548 * (-(((0.05 * locals.var_q_qsq_dn7) * assign18680_e18557) + (assign18680_e18552 * (-(0.0396825396825397 * locals.var_q_qsq_dn7)))))))), (-(((locals.var_q_qsq_dn8 * 0.3333333333333) * assign18680_e18559) + (assign18680_e18548 * (-(((0.05 * locals.var_q_qsq_dn8) * assign18680_e18557) + (assign18680_e18552 * (-(0.0396825396825397 * locals.var_q_qsq_dn8)))))))), (-(((locals.var_q_qsq_dn9 * 0.3333333333333) * assign18680_e18559) + (assign18680_e18548 * (-(((0.05 * locals.var_q_qsq_dn9) * assign18680_e18557) + (assign18680_e18552 * (-(0.0396825396825397 * locals.var_q_qsq_dn9)))))))),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign18680_e18563;
        locals.var_q_sh_term_dn4 = assign18680_e18563_d_n4;
        locals.var_q_sh_term_dn6 = assign18680_e18563_d_n6;
        locals.var_q_sh_term_dn7 = assign18680_e18563_d_n7;
        locals.var_q_sh_term_dn8 = assign18680_e18563_d_n8;
        locals.var_q_sh_term_dn9 = assign18680_e18563_d_n9;

        let (assign18690_e18572, assign18690_e18572_d_n4, assign18690_e18572_d_n6, assign18690_e18572_d_n7, assign18690_e18572_d_n8, assign18690_e18572_d_n9,) = {
    if ((locals.var_guard635 == 0.0) && (locals.var_guard636 == 0.0)) {
        let assign18690_e18570: f64 = (locals.var_q_sh_term).ln();
        (assign18690_e18570, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign18690_e18572;
        locals.var_q_ln_term_dn4 = assign18690_e18572_d_n4;
        locals.var_q_ln_term_dn6 = assign18690_e18572_d_n6;
        locals.var_q_ln_term_dn7 = assign18690_e18572_d_n7;
        locals.var_q_ln_term_dn8 = assign18690_e18572_d_n8;
        locals.var_q_ln_term_dn9 = assign18690_e18572_d_n9;

        let assign18700_e18575: f64 = (1.01 * locals.var_q_k1q1);
        let assign18700_e18577: f64 = (assign18700_e18575 + locals.var_q_qcoth);
        let assign18700_e18579: f64 = if assign18700_e18577 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard637 = assign18700_e18579;

        let (assign18710_e18585, assign18710_e18585_d_n4, assign18710_e18585_d_n6, assign18710_e18585_d_n7, assign18710_e18585_d_n8, assign18710_e18585_d_n9,) = {
    if (locals.var_guard637 != 0.0) {
        let assign18710_e18583: f64 = (locals.var_q_k1q1 + locals.var_q_qcoth);
        (assign18710_e18583, (locals.var_q_k1q1_dn4 + locals.var_q_qcoth_dn4), (locals.var_q_k1q1_dn6 + locals.var_q_qcoth_dn6), (locals.var_q_k1q1_dn7 + locals.var_q_qcoth_dn7), (locals.var_q_k1q1_dn8 + locals.var_q_qcoth_dn8), (locals.var_q_k1q1_dn9 + locals.var_q_qcoth_dn9),)
    } else {
        (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9,)
    }
};
        locals.var_q_expnum = assign18710_e18585;
        locals.var_q_expnum_dn4 = assign18710_e18585_d_n4;
        locals.var_q_expnum_dn6 = assign18710_e18585_d_n6;
        locals.var_q_expnum_dn7 = assign18710_e18585_d_n7;
        locals.var_q_expnum_dn8 = assign18710_e18585_d_n8;
        locals.var_q_expnum_dn9 = assign18710_e18585_d_n9;

        let (assign18720_e18591, assign18720_e18591_d_n4, assign18720_e18591_d_n6, assign18720_e18591_d_n7, assign18720_e18591_d_n8, assign18720_e18591_d_n9,) = {
    if (locals.var_guard637 != 0.0) {
        let assign18720_e18589: f64 = (locals.var_k1 + locals.var_q_d1_qcoth);
        (assign18720_e18589, (locals.var_k1_dn4 + locals.var_q_d1_qcoth_dn4), (locals.var_k1_dn6 + locals.var_q_d1_qcoth_dn6), (locals.var_k1_dn7 + locals.var_q_d1_qcoth_dn7), (locals.var_k1_dn8 + locals.var_q_d1_qcoth_dn8), (locals.var_k1_dn9 + locals.var_q_d1_qcoth_dn9),)
    } else {
        (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9,)
    }
};
        locals.var_q_d1_expnum = assign18720_e18591;
        locals.var_q_d1_expnum_dn4 = assign18720_e18591_d_n4;
        locals.var_q_d1_expnum_dn6 = assign18720_e18591_d_n6;
        locals.var_q_d1_expnum_dn7 = assign18720_e18591_d_n7;
        locals.var_q_d1_expnum_dn8 = assign18720_e18591_d_n8;
        locals.var_q_d1_expnum_dn9 = assign18720_e18591_d_n9;

        let (assign18730_e18595, assign18730_e18595_d_n4, assign18730_e18595_d_n6, assign18730_e18595_d_n7, assign18730_e18595_d_n8, assign18730_e18595_d_n9,) = {
    if (locals.var_guard637 != 0.0) {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    } else {
        (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9,)
    }
};
        locals.var_q_d2_expnum = assign18730_e18595;
        locals.var_q_d2_expnum_dn4 = assign18730_e18595_d_n4;
        locals.var_q_d2_expnum_dn6 = assign18730_e18595_d_n6;
        locals.var_q_d2_expnum_dn7 = assign18730_e18595_d_n7;
        locals.var_q_d2_expnum_dn8 = assign18730_e18595_d_n8;
        locals.var_q_d2_expnum_dn9 = assign18730_e18595_d_n9;

        let (assign18740_e18604, assign18740_e18604_d_n4, assign18740_e18604_d_n6, assign18740_e18604_d_n7, assign18740_e18604_d_n8, assign18740_e18604_d_n9,) = {
    if (locals.var_guard637 == 0.0) {
        let assign18740_e18601: f64 = (locals.var_q_k1q1 - locals.var_q_qcoth);
        let assign18740_e18602: f64 = (1.0 / assign18740_e18601);
        (assign18740_e18602, (-((locals.var_q_k1q1_dn4 - locals.var_q_qcoth_dn4) / (assign18740_e18601 * assign18740_e18601))), (-((locals.var_q_k1q1_dn6 - locals.var_q_qcoth_dn6) / (assign18740_e18601 * assign18740_e18601))), (-((locals.var_q_k1q1_dn7 - locals.var_q_qcoth_dn7) / (assign18740_e18601 * assign18740_e18601))), (-((locals.var_q_k1q1_dn8 - locals.var_q_qcoth_dn8) / (assign18740_e18601 * assign18740_e18601))), (-((locals.var_q_k1q1_dn9 - locals.var_q_qcoth_dn9) / (assign18740_e18601 * assign18740_e18601))),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign18740_e18604;
        locals.var_q_temp2_dn4 = assign18740_e18604_d_n4;
        locals.var_q_temp2_dn6 = assign18740_e18604_d_n6;
        locals.var_q_temp2_dn7 = assign18740_e18604_d_n7;
        locals.var_q_temp2_dn8 = assign18740_e18604_d_n8;
        locals.var_q_temp2_dn9 = assign18740_e18604_d_n9;

        let (assign18750_e18611, assign18750_e18611_d_n4, assign18750_e18611_d_n6, assign18750_e18611_d_n7, assign18750_e18611_d_n8, assign18750_e18611_d_n9,) = {
    if (locals.var_guard637 == 0.0) {
        let assign18750_e18609: f64 = (locals.var_q_d1_qcoth - locals.var_k1);
        (assign18750_e18609, (locals.var_q_d1_qcoth_dn4 - locals.var_k1_dn4), (locals.var_q_d1_qcoth_dn6 - locals.var_k1_dn6), (locals.var_q_d1_qcoth_dn7 - locals.var_k1_dn7), (locals.var_q_d1_qcoth_dn8 - locals.var_k1_dn8), (locals.var_q_d1_qcoth_dn9 - locals.var_k1_dn9),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign18750_e18611;
        locals.var_q_temp3_dn4 = assign18750_e18611_d_n4;
        locals.var_q_temp3_dn6 = assign18750_e18611_d_n6;
        locals.var_q_temp3_dn7 = assign18750_e18611_d_n7;
        locals.var_q_temp3_dn8 = assign18750_e18611_d_n8;
        locals.var_q_temp3_dn9 = assign18750_e18611_d_n9;

        let (assign18760_e18620, assign18760_e18620_d_n4, assign18760_e18620_d_n6, assign18760_e18620_d_n7, assign18760_e18620_d_n8, assign18760_e18620_d_n9,) = {
    if (locals.var_guard637 == 0.0) {
        let assign18760_e18616: f64 = (locals.var_q_aexp - locals.var_q_sh_term);
        let assign18760_e18618: f64 = (assign18760_e18616 * locals.var_q_temp2);
        (assign18760_e18618, (((locals.var_q_aexp_dn4 - locals.var_q_sh_term_dn4) * locals.var_q_temp2) + (assign18760_e18616 * locals.var_q_temp2_dn4)), (((locals.var_q_aexp_dn6 - locals.var_q_sh_term_dn6) * locals.var_q_temp2) + (assign18760_e18616 * locals.var_q_temp2_dn6)), (((locals.var_q_aexp_dn7 - locals.var_q_sh_term_dn7) * locals.var_q_temp2) + (assign18760_e18616 * locals.var_q_temp2_dn7)), (((locals.var_q_aexp_dn8 - locals.var_q_sh_term_dn8) * locals.var_q_temp2) + (assign18760_e18616 * locals.var_q_temp2_dn8)), (((locals.var_q_aexp_dn9 - locals.var_q_sh_term_dn9) * locals.var_q_temp2) + (assign18760_e18616 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9,)
    }
};
        locals.var_q_expnum = assign18760_e18620;
        locals.var_q_expnum_dn4 = assign18760_e18620_d_n4;
        locals.var_q_expnum_dn6 = assign18760_e18620_d_n6;
        locals.var_q_expnum_dn7 = assign18760_e18620_d_n7;
        locals.var_q_expnum_dn8 = assign18760_e18620_d_n8;
        locals.var_q_expnum_dn9 = assign18760_e18620_d_n9;

        let (assign18770_e18635, assign18770_e18635_d_n4, assign18770_e18635_d_n6, assign18770_e18635_d_n7, assign18770_e18635_d_n8, assign18770_e18635_d_n9,) = {
    if (locals.var_guard637 == 0.0) {
        let assign18770_e18625: f64 = (locals.var_q_temp3 * locals.var_q_expnum);
        let assign18770_e18627: f64 = (assign18770_e18625 - locals.var_q_aexp);
        let assign18770_e18630: f64 = (locals.var_q_d1_ln * locals.var_q_sh_term);
        let assign18770_e18631: f64 = (assign18770_e18627 - assign18770_e18630);
        let assign18770_e18633: f64 = (assign18770_e18631 * locals.var_q_temp2);
        (assign18770_e18633, ((((((locals.var_q_temp3_dn4 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4) - ((locals.var_q_d1_ln_dn4 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign18770_e18631 * locals.var_q_temp2_dn4)), ((((((locals.var_q_temp3_dn6 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6) - ((locals.var_q_d1_ln_dn6 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign18770_e18631 * locals.var_q_temp2_dn6)), ((((((locals.var_q_temp3_dn7 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7) - ((locals.var_q_d1_ln_dn7 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign18770_e18631 * locals.var_q_temp2_dn7)), ((((((locals.var_q_temp3_dn8 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8) - ((locals.var_q_d1_ln_dn8 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign18770_e18631 * locals.var_q_temp2_dn8)), ((((((locals.var_q_temp3_dn9 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9) - ((locals.var_q_d1_ln_dn9 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign18770_e18631 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9,)
    }
};
        locals.var_q_d1_expnum = assign18770_e18635;
        locals.var_q_d1_expnum_dn4 = assign18770_e18635_d_n4;
        locals.var_q_d1_expnum_dn6 = assign18770_e18635_d_n6;
        locals.var_q_d1_expnum_dn7 = assign18770_e18635_d_n7;
        locals.var_q_d1_expnum_dn8 = assign18770_e18635_d_n8;
        locals.var_q_d1_expnum_dn9 = assign18770_e18635_d_n9;

        let (assign18780_e18660, assign18780_e18660_d_n4, assign18780_e18660_d_n6, assign18780_e18660_d_n7, assign18780_e18660_d_n8, assign18780_e18660_d_n9,) = {
    if (locals.var_guard637 == 0.0) {
        let assign18780_e18640: f64 = (locals.var_q_d2_qcoth * locals.var_q_expnum);
        let assign18780_e18643: f64 = (2.0 * locals.var_q_temp3);
        let assign18780_e18645: f64 = (assign18780_e18643 * locals.var_q_d1_expnum);
        let assign18780_e18646: f64 = (assign18780_e18640 + assign18780_e18645);
        let assign18780_e18648: f64 = (assign18780_e18646 + locals.var_q_aexp);
        let assign18780_e18652: f64 = (locals.var_q_d1_ln * locals.var_q_d1_ln);
        let assign18780_e18653: f64 = (locals.var_q_d2_ln + assign18780_e18652);
        let assign18780_e18655: f64 = (assign18780_e18653 * locals.var_q_sh_term);
        let assign18780_e18656: f64 = (assign18780_e18648 - assign18780_e18655);
        let assign18780_e18658: f64 = (assign18780_e18656 * locals.var_q_temp2);
        (assign18780_e18658, (((((((locals.var_q_d2_qcoth_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_temp3_dn4) * locals.var_q_d1_expnum) + (assign18780_e18643 * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4) - (((locals.var_q_d2_ln_dn4 + ((locals.var_q_d1_ln_dn4 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn4))) * locals.var_q_sh_term) + (assign18780_e18653 * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign18780_e18656 * locals.var_q_temp2_dn4)), (((((((locals.var_q_d2_qcoth_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_temp3_dn6) * locals.var_q_d1_expnum) + (assign18780_e18643 * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6) - (((locals.var_q_d2_ln_dn6 + ((locals.var_q_d1_ln_dn6 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn6))) * locals.var_q_sh_term) + (assign18780_e18653 * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign18780_e18656 * locals.var_q_temp2_dn6)), (((((((locals.var_q_d2_qcoth_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_temp3_dn7) * locals.var_q_d1_expnum) + (assign18780_e18643 * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7) - (((locals.var_q_d2_ln_dn7 + ((locals.var_q_d1_ln_dn7 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn7))) * locals.var_q_sh_term) + (assign18780_e18653 * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign18780_e18656 * locals.var_q_temp2_dn7)), (((((((locals.var_q_d2_qcoth_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_temp3_dn8) * locals.var_q_d1_expnum) + (assign18780_e18643 * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8) - (((locals.var_q_d2_ln_dn8 + ((locals.var_q_d1_ln_dn8 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn8))) * locals.var_q_sh_term) + (assign18780_e18653 * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign18780_e18656 * locals.var_q_temp2_dn8)), (((((((locals.var_q_d2_qcoth_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_temp3_dn9) * locals.var_q_d1_expnum) + (assign18780_e18643 * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9) - (((locals.var_q_d2_ln_dn9 + ((locals.var_q_d1_ln_dn9 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn9))) * locals.var_q_sh_term) + (assign18780_e18653 * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign18780_e18656 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9,)
    }
};
        locals.var_q_d2_expnum = assign18780_e18660;
        locals.var_q_d2_expnum_dn4 = assign18780_e18660_d_n4;
        locals.var_q_d2_expnum_dn6 = assign18780_e18660_d_n6;
        locals.var_q_d2_expnum_dn7 = assign18780_e18660_d_n7;
        locals.var_q_d2_expnum_dn8 = assign18780_e18660_d_n8;
        locals.var_q_d2_expnum_dn9 = assign18780_e18660_d_n9;

        let assign18790_e18663: f64 = if locals.var_q_expnum > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard638 = assign18790_e18663;

        let (assign18800_e18668, assign18800_e18668_d_n4, assign18800_e18668_d_n6, assign18800_e18668_d_n7, assign18800_e18668_d_n8, assign18800_e18668_d_n9,) = {
    if (locals.var_guard638 != 0.0) {
        let assign18800_e18666: f64 = (locals.var_q_expnum).ln();
        (assign18800_e18666, (locals.var_q_expnum_dn4 / locals.var_q_expnum), (locals.var_q_expnum_dn6 / locals.var_q_expnum), (locals.var_q_expnum_dn7 / locals.var_q_expnum), (locals.var_q_expnum_dn8 / locals.var_q_expnum), (locals.var_q_expnum_dn9 / locals.var_q_expnum),)
    } else {
        (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9,)
    }
};
        locals.var_q_lnexpnum = assign18800_e18668;
        locals.var_q_lnexpnum_dn4 = assign18800_e18668_d_n4;
        locals.var_q_lnexpnum_dn6 = assign18800_e18668_d_n6;
        locals.var_q_lnexpnum_dn7 = assign18800_e18668_d_n7;
        locals.var_q_lnexpnum_dn8 = assign18800_e18668_d_n8;
        locals.var_q_lnexpnum_dn9 = assign18800_e18668_d_n9;

        let (assign18810_e18674, assign18810_e18674_d_n4, assign18810_e18674_d_n6, assign18810_e18674_d_n7, assign18810_e18674_d_n8, assign18810_e18674_d_n9,) = {
    if (locals.var_guard638 != 0.0) {
        let assign18810_e18672: f64 = (1.0 / locals.var_q_expnum);
        (assign18810_e18672, (-(locals.var_q_expnum_dn4 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn6 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn7 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn8 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn9 / (locals.var_q_expnum * locals.var_q_expnum))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign18810_e18674;
        locals.var_q_temp1_dn4 = assign18810_e18674_d_n4;
        locals.var_q_temp1_dn6 = assign18810_e18674_d_n6;
        locals.var_q_temp1_dn7 = assign18810_e18674_d_n7;
        locals.var_q_temp1_dn8 = assign18810_e18674_d_n8;
        locals.var_q_temp1_dn9 = assign18810_e18674_d_n9;

        let (assign18820_e18680, assign18820_e18680_d_n4, assign18820_e18680_d_n6, assign18820_e18680_d_n7, assign18820_e18680_d_n8, assign18820_e18680_d_n9,) = {
    if (locals.var_guard638 != 0.0) {
        let assign18820_e18678: f64 = (locals.var_q_d1_expnum * locals.var_q_temp1);
        (assign18820_e18678, ((locals.var_q_d1_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn4)), ((locals.var_q_d1_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn6)), ((locals.var_q_d1_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn7)), ((locals.var_q_d1_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn8)), ((locals.var_q_d1_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9,)
    }
};
        locals.var_q_d1_lnexpnum = assign18820_e18680;
        locals.var_q_d1_lnexpnum_dn4 = assign18820_e18680_d_n4;
        locals.var_q_d1_lnexpnum_dn6 = assign18820_e18680_d_n6;
        locals.var_q_d1_lnexpnum_dn7 = assign18820_e18680_d_n7;
        locals.var_q_d1_lnexpnum_dn8 = assign18820_e18680_d_n8;
        locals.var_q_d1_lnexpnum_dn9 = assign18820_e18680_d_n9;

        let (assign18830_e18690, assign18830_e18690_d_n4, assign18830_e18690_d_n6, assign18830_e18690_d_n7, assign18830_e18690_d_n8, assign18830_e18690_d_n9,) = {
    if (locals.var_guard638 != 0.0) {
        let assign18830_e18684: f64 = (locals.var_q_d2_expnum * locals.var_q_temp1);
        let assign18830_e18687: f64 = (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum);
        let assign18830_e18688: f64 = (assign18830_e18684 - assign18830_e18687);
        (assign18830_e18688, (((locals.var_q_d2_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn4)) - ((locals.var_q_d1_lnexpnum_dn4 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn4))), (((locals.var_q_d2_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn6)) - ((locals.var_q_d1_lnexpnum_dn6 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn6))), (((locals.var_q_d2_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn7)) - ((locals.var_q_d1_lnexpnum_dn7 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn7))), (((locals.var_q_d2_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn8)) - ((locals.var_q_d1_lnexpnum_dn8 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn8))), (((locals.var_q_d2_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn9)) - ((locals.var_q_d1_lnexpnum_dn9 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn9))),)
    } else {
        (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9,)
    }
};
        locals.var_q_d2_lnexpnum = assign18830_e18690;
        locals.var_q_d2_lnexpnum_dn4 = assign18830_e18690_d_n4;
        locals.var_q_d2_lnexpnum_dn6 = assign18830_e18690_d_n6;
        locals.var_q_d2_lnexpnum_dn7 = assign18830_e18690_d_n7;
        locals.var_q_d2_lnexpnum_dn8 = assign18830_e18690_d_n8;
        locals.var_q_d2_lnexpnum_dn9 = assign18830_e18690_d_n9;

        let (assign18840_e18701, assign18840_e18701_d_n4, assign18840_e18701_d_n6, assign18840_e18701_d_n7, assign18840_e18701_d_n8, assign18840_e18701_d_n9,) = {
    if (locals.var_guard638 == 0.0) {
        let assign18840_e18695: f64 = (locals.var_q_k1q1 + 0.6931471805599);
        let assign18840_e18697: f64 = (-locals.var_q_k1q1);
        let assign18840_e18698: f64 = (assign18840_e18697).ln();
        let assign18840_e18699: f64 = (assign18840_e18695 + assign18840_e18698);
        (assign18840_e18699, (locals.var_q_k1q1_dn4 + ((-locals.var_q_k1q1_dn4) / assign18840_e18697)), (locals.var_q_k1q1_dn6 + ((-locals.var_q_k1q1_dn6) / assign18840_e18697)), (locals.var_q_k1q1_dn7 + ((-locals.var_q_k1q1_dn7) / assign18840_e18697)), (locals.var_q_k1q1_dn8 + ((-locals.var_q_k1q1_dn8) / assign18840_e18697)), (locals.var_q_k1q1_dn9 + ((-locals.var_q_k1q1_dn9) / assign18840_e18697)),)
    } else {
        (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9,)
    }
};
        locals.var_q_lnexpnum = assign18840_e18701;
        locals.var_q_lnexpnum_dn4 = assign18840_e18701_d_n4;
        locals.var_q_lnexpnum_dn6 = assign18840_e18701_d_n6;
        locals.var_q_lnexpnum_dn7 = assign18840_e18701_d_n7;
        locals.var_q_lnexpnum_dn8 = assign18840_e18701_d_n8;
        locals.var_q_lnexpnum_dn9 = assign18840_e18701_d_n9;

        let (assign18850_e18708, assign18850_e18708_d_n4, assign18850_e18708_d_n6, assign18850_e18708_d_n7, assign18850_e18708_d_n8, assign18850_e18708_d_n9,) = {
    if (locals.var_guard638 == 0.0) {
        let assign18850_e18706: f64 = (1.0 / locals.var_q1d);
        (assign18850_e18706, (-(locals.var_q1d_dn4 / (locals.var_q1d * locals.var_q1d))), (-(locals.var_q1d_dn6 / (locals.var_q1d * locals.var_q1d))), (-(locals.var_q1d_dn7 / (locals.var_q1d * locals.var_q1d))), (-(locals.var_q1d_dn8 / (locals.var_q1d * locals.var_q1d))), (-(locals.var_q1d_dn9 / (locals.var_q1d * locals.var_q1d))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign18850_e18708;
        locals.var_q_temp1_dn4 = assign18850_e18708_d_n4;
        locals.var_q_temp1_dn6 = assign18850_e18708_d_n6;
        locals.var_q_temp1_dn7 = assign18850_e18708_d_n7;
        locals.var_q_temp1_dn8 = assign18850_e18708_d_n8;
        locals.var_q_temp1_dn9 = assign18850_e18708_d_n9;

        let (assign18860_e18715, assign18860_e18715_d_n4, assign18860_e18715_d_n6, assign18860_e18715_d_n7, assign18860_e18715_d_n8, assign18860_e18715_d_n9,) = {
    if (locals.var_guard638 == 0.0) {
        let assign18860_e18713: f64 = (locals.var_k1 + locals.var_q_temp1);
        (assign18860_e18713, (locals.var_k1_dn4 + locals.var_q_temp1_dn4), (locals.var_k1_dn6 + locals.var_q_temp1_dn6), (locals.var_k1_dn7 + locals.var_q_temp1_dn7), (locals.var_k1_dn8 + locals.var_q_temp1_dn8), (locals.var_k1_dn9 + locals.var_q_temp1_dn9),)
    } else {
        (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9,)
    }
};
        locals.var_q_d1_lnexpnum = assign18860_e18715;
        locals.var_q_d1_lnexpnum_dn4 = assign18860_e18715_d_n4;
        locals.var_q_d1_lnexpnum_dn6 = assign18860_e18715_d_n6;
        locals.var_q_d1_lnexpnum_dn7 = assign18860_e18715_d_n7;
        locals.var_q_d1_lnexpnum_dn8 = assign18860_e18715_d_n8;
        locals.var_q_d1_lnexpnum_dn9 = assign18860_e18715_d_n9;

        let (assign18870_e18723, assign18870_e18723_d_n4, assign18870_e18723_d_n6, assign18870_e18723_d_n7, assign18870_e18723_d_n8, assign18870_e18723_d_n9,) = {
    if (locals.var_guard638 == 0.0) {
        let assign18870_e18719: f64 = (-locals.var_q_temp1);
        let assign18870_e18721: f64 = (assign18870_e18719 * locals.var_q_temp1);
        (assign18870_e18721, (((-locals.var_q_temp1_dn4) * locals.var_q_temp1) + (assign18870_e18719 * locals.var_q_temp1_dn4)), (((-locals.var_q_temp1_dn6) * locals.var_q_temp1) + (assign18870_e18719 * locals.var_q_temp1_dn6)), (((-locals.var_q_temp1_dn7) * locals.var_q_temp1) + (assign18870_e18719 * locals.var_q_temp1_dn7)), (((-locals.var_q_temp1_dn8) * locals.var_q_temp1) + (assign18870_e18719 * locals.var_q_temp1_dn8)), (((-locals.var_q_temp1_dn9) * locals.var_q_temp1) + (assign18870_e18719 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9,)
    }
};
        locals.var_q_d2_lnexpnum = assign18870_e18723;
        locals.var_q_d2_lnexpnum_dn4 = assign18870_e18723_d_n4;
        locals.var_q_d2_lnexpnum_dn6 = assign18870_e18723_d_n6;
        locals.var_q_d2_lnexpnum_dn7 = assign18870_e18723_d_n7;
        locals.var_q_d2_lnexpnum_dn8 = assign18870_e18723_d_n8;
        locals.var_q_d2_lnexpnum_dn9 = assign18870_e18723_d_n9;

        let assign18880_e18726: f64 = (locals.var_xg2x - locals.var_xg1x);
        let assign18880_e18728: f64 = (assign18880_e18726 + locals.var_q1d);
        let assign18880_e18731: f64 = (2.0 * locals.var_q_lnexpnum);
        let assign18880_e18732: f64 = (assign18880_e18728 + assign18880_e18731);
        let assign18880_e18734: f64 = (assign18880_e18732 - locals.var_q_ln_term);
        locals.var_q_q2_int = assign18880_e18734;
        locals.var_q_q2_int_dn4 = ((((locals.var_xg2x_dn4 - locals.var_xg1x_dn4) + locals.var_q1d_dn4) + (2.0 * locals.var_q_lnexpnum_dn4)) - locals.var_q_ln_term_dn4);
        locals.var_q_q2_int_dn6 = ((((locals.var_xg2x_dn6 - locals.var_xg1x_dn6) + locals.var_q1d_dn6) + (2.0 * locals.var_q_lnexpnum_dn6)) - locals.var_q_ln_term_dn6);
        locals.var_q_q2_int_dn7 = ((((locals.var_xg2x_dn7 - locals.var_xg1x_dn7) + locals.var_q1d_dn7) + (2.0 * locals.var_q_lnexpnum_dn7)) - locals.var_q_ln_term_dn7);
        locals.var_q_q2_int_dn8 = ((((locals.var_xg2x_dn8 - locals.var_xg1x_dn8) + locals.var_q1d_dn8) + (2.0 * locals.var_q_lnexpnum_dn8)) - locals.var_q_ln_term_dn8);
        locals.var_q_q2_int_dn9 = ((((locals.var_xg2x_dn9 - locals.var_xg1x_dn9) + locals.var_q1d_dn9) + (2.0 * locals.var_q_lnexpnum_dn9)) - locals.var_q_ln_term_dn9);

        let assign18890_e18738: f64 = (2.0 * locals.var_q_d1_lnexpnum);
        let assign18890_e18739: f64 = (1.0 + assign18890_e18738);
        let assign18890_e18741: f64 = (assign18890_e18739 - locals.var_q_d1_ln);
        locals.var_q_d1_q2 = assign18890_e18741;
        locals.var_q_d1_q2_dn4 = ((2.0 * locals.var_q_d1_lnexpnum_dn4) - locals.var_q_d1_ln_dn4);
        locals.var_q_d1_q2_dn6 = ((2.0 * locals.var_q_d1_lnexpnum_dn6) - locals.var_q_d1_ln_dn6);
        locals.var_q_d1_q2_dn7 = ((2.0 * locals.var_q_d1_lnexpnum_dn7) - locals.var_q_d1_ln_dn7);
        locals.var_q_d1_q2_dn8 = ((2.0 * locals.var_q_d1_lnexpnum_dn8) - locals.var_q_d1_ln_dn8);
        locals.var_q_d1_q2_dn9 = ((2.0 * locals.var_q_d1_lnexpnum_dn9) - locals.var_q_d1_ln_dn9);

        let assign18900_e18744: f64 = (2.0 * locals.var_q_d2_lnexpnum);
        let assign18900_e18746: f64 = (assign18900_e18744 - locals.var_q_d2_ln);
        locals.var_q_d2_q2 = assign18900_e18746;
        locals.var_q_d2_q2_dn4 = ((2.0 * locals.var_q_d2_lnexpnum_dn4) - locals.var_q_d2_ln_dn4);
        locals.var_q_d2_q2_dn6 = ((2.0 * locals.var_q_d2_lnexpnum_dn6) - locals.var_q_d2_ln_dn6);
        locals.var_q_d2_q2_dn7 = ((2.0 * locals.var_q_d2_lnexpnum_dn7) - locals.var_q_d2_ln_dn7);
        locals.var_q_d2_q2_dn8 = ((2.0 * locals.var_q_d2_lnexpnum_dn8) - locals.var_q_d2_ln_dn8);
        locals.var_q_d2_q2_dn9 = ((2.0 * locals.var_q_d2_lnexpnum_dn9) - locals.var_q_d2_ln_dn9);

        let assign18910_e18750: f64 = (locals.var_k2 * locals.var_q_q2_int);
        let assign18910_e18751: f64 = (locals.var_q_k1q1 + assign18910_e18750);
        locals.var_q_qi_int = assign18910_e18751;
        locals.var_q_qi_int_dn4 = (locals.var_q_k1q1_dn4 + ((locals.var_k2_dn4 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn4)));
        locals.var_q_qi_int_dn6 = (locals.var_q_k1q1_dn6 + ((locals.var_k2_dn6 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn6)));
        locals.var_q_qi_int_dn7 = (locals.var_q_k1q1_dn7 + ((locals.var_k2_dn7 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn7)));
        locals.var_q_qi_int_dn8 = (locals.var_q_k1q1_dn8 + ((locals.var_k2_dn8 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn8)));
        locals.var_q_qi_int_dn9 = (locals.var_q_k1q1_dn9 + ((locals.var_k2_dn9 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn9)));

        let assign18920_e18755: f64 = (locals.var_k2 * locals.var_q_d1_q2);
        let assign18920_e18756: f64 = (locals.var_k1 + assign18920_e18755);
        locals.var_q_d1_qi = assign18920_e18756;
        locals.var_q_d1_qi_dn4 = (locals.var_k1_dn4 + ((locals.var_k2_dn4 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn4)));
        locals.var_q_d1_qi_dn6 = (locals.var_k1_dn6 + ((locals.var_k2_dn6 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn6)));
        locals.var_q_d1_qi_dn7 = (locals.var_k1_dn7 + ((locals.var_k2_dn7 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn7)));
        locals.var_q_d1_qi_dn8 = (locals.var_k1_dn8 + ((locals.var_k2_dn8 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn8)));
        locals.var_q_d1_qi_dn9 = (locals.var_k1_dn9 + ((locals.var_k2_dn9 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn9)));

        let assign18930_e18759: f64 = (locals.var_k2 * locals.var_q_d2_q2);
        locals.var_q_d2_qi = assign18930_e18759;
        locals.var_q_d2_qi_dn4 = ((locals.var_k2_dn4 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn4));
        locals.var_q_d2_qi_dn6 = ((locals.var_k2_dn6 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn6));
        locals.var_q_d2_qi_dn7 = ((locals.var_k2_dn7 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn7));
        locals.var_q_d2_qi_dn8 = ((locals.var_k2_dn8 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn8));
        locals.var_q_d2_qi_dn9 = ((locals.var_k2_dn9 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn9));

        let assign18940_e18762: f64 = (locals.var_q_qi_int * locals.var_q_expnum);
        let assign18940_e18764: f64 = (assign18940_e18762 - locals.var_q_aexp);
        locals.var_q_zero = assign18940_e18764;
        locals.var_q_zero_dn4 = (((locals.var_q_qi_int_dn4 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4);
        locals.var_q_zero_dn6 = (((locals.var_q_qi_int_dn6 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6);
        locals.var_q_zero_dn7 = (((locals.var_q_qi_int_dn7 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7);
        locals.var_q_zero_dn8 = (((locals.var_q_qi_int_dn8 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8);
        locals.var_q_zero_dn9 = (((locals.var_q_qi_int_dn9 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9);

        let assign18950_e18767: f64 = (locals.var_q_d1_qi * locals.var_q_expnum);
        let assign18950_e18770: f64 = (locals.var_q_qi_int * locals.var_q_d1_expnum);
        let assign18950_e18771: f64 = (assign18950_e18767 + assign18950_e18770);
        let assign18950_e18773: f64 = (assign18950_e18771 + locals.var_q_aexp);
        locals.var_q_d1_zero = assign18950_e18773;
        locals.var_q_d1_zero_dn4 = ((((locals.var_q_d1_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn4)) + ((locals.var_q_qi_int_dn4 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4);
        locals.var_q_d1_zero_dn6 = ((((locals.var_q_d1_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn6)) + ((locals.var_q_qi_int_dn6 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6);
        locals.var_q_d1_zero_dn7 = ((((locals.var_q_d1_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn7)) + ((locals.var_q_qi_int_dn7 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7);
        locals.var_q_d1_zero_dn8 = ((((locals.var_q_d1_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn8)) + ((locals.var_q_qi_int_dn8 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8);
        locals.var_q_d1_zero_dn9 = ((((locals.var_q_d1_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn9)) + ((locals.var_q_qi_int_dn9 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9);

        let assign18960_e18776: f64 = (locals.var_q_d2_qi * locals.var_q_expnum);
        let assign18960_e18779: f64 = (2.0 * locals.var_q_d1_qi);
        let assign18960_e18781: f64 = (assign18960_e18779 * locals.var_q_d1_expnum);
        let assign18960_e18782: f64 = (assign18960_e18776 + assign18960_e18781);
        let assign18960_e18785: f64 = (locals.var_q_qi_int * locals.var_q_d2_expnum);
        let assign18960_e18786: f64 = (assign18960_e18782 + assign18960_e18785);
        let assign18960_e18788: f64 = (assign18960_e18786 - locals.var_q_aexp);
        locals.var_q_d2_zero = assign18960_e18788;
        locals.var_q_d2_zero_dn4 = (((((locals.var_q_d2_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_d1_qi_dn4) * locals.var_q_d1_expnum) + (assign18960_e18779 * locals.var_q_d1_expnum_dn4))) + ((locals.var_q_qi_int_dn4 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn4))) - locals.var_q_aexp_dn4);
        locals.var_q_d2_zero_dn6 = (((((locals.var_q_d2_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_d1_qi_dn6) * locals.var_q_d1_expnum) + (assign18960_e18779 * locals.var_q_d1_expnum_dn6))) + ((locals.var_q_qi_int_dn6 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn6))) - locals.var_q_aexp_dn6);
        locals.var_q_d2_zero_dn7 = (((((locals.var_q_d2_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_d1_qi_dn7) * locals.var_q_d1_expnum) + (assign18960_e18779 * locals.var_q_d1_expnum_dn7))) + ((locals.var_q_qi_int_dn7 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn7))) - locals.var_q_aexp_dn7);
        locals.var_q_d2_zero_dn8 = (((((locals.var_q_d2_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_d1_qi_dn8) * locals.var_q_d1_expnum) + (assign18960_e18779 * locals.var_q_d1_expnum_dn8))) + ((locals.var_q_qi_int_dn8 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn8))) - locals.var_q_aexp_dn8);
        locals.var_q_d2_zero_dn9 = (((((locals.var_q_d2_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_d1_qi_dn9) * locals.var_q_d1_expnum) + (assign18960_e18779 * locals.var_q_d1_expnum_dn9))) + ((locals.var_q_qi_int_dn9 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn9))) - locals.var_q_aexp_dn9);

        let assign18970_e18791: f64 = (locals.var_q_d1_zero * locals.var_q_d1_zero);
        let assign18970_e18794: f64 = (0.5 * locals.var_q_zero);
        let assign18970_e18796: f64 = (assign18970_e18794 * locals.var_q_d2_zero);
        let assign18970_e18797: f64 = (assign18970_e18791 - assign18970_e18796);
        locals.var_q_temp = assign18970_e18797;
        locals.var_q_temp_dn4 = (((locals.var_q_d1_zero_dn4 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn4)) - (((0.5 * locals.var_q_zero_dn4) * locals.var_q_d2_zero) + (assign18970_e18794 * locals.var_q_d2_zero_dn4)));
        locals.var_q_temp_dn6 = (((locals.var_q_d1_zero_dn6 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn6)) - (((0.5 * locals.var_q_zero_dn6) * locals.var_q_d2_zero) + (assign18970_e18794 * locals.var_q_d2_zero_dn6)));
        locals.var_q_temp_dn7 = (((locals.var_q_d1_zero_dn7 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn7)) - (((0.5 * locals.var_q_zero_dn7) * locals.var_q_d2_zero) + (assign18970_e18794 * locals.var_q_d2_zero_dn7)));
        locals.var_q_temp_dn8 = (((locals.var_q_d1_zero_dn8 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn8)) - (((0.5 * locals.var_q_zero_dn8) * locals.var_q_d2_zero) + (assign18970_e18794 * locals.var_q_d2_zero_dn8)));
        locals.var_q_temp_dn9 = (((locals.var_q_d1_zero_dn9 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn9)) - (((0.5 * locals.var_q_zero_dn9) * locals.var_q_d2_zero) + (assign18970_e18794 * locals.var_q_d2_zero_dn9)));

        let assign18980_e18799: f64 = (-locals.var_q_zero);
        let assign18980_e18801: f64 = (assign18980_e18799 * locals.var_q_d1_zero);
        let assign18980_e18803: f64 = (assign18980_e18801 * locals.var_q_temp);
        let assign18980_e18806: f64 = (locals.var_q_temp * locals.var_q_temp);
        let assign18980_e18808: f64 = (assign18980_e18806 + 1e-200);
        let assign18980_e18809: f64 = (assign18980_e18803 / assign18980_e18808);
        locals.var_q_eps2 = assign18980_e18809;
        locals.var_q_eps2_dn4 = ((((((((-locals.var_q_zero_dn4) * locals.var_q_d1_zero) + (assign18980_e18799 * locals.var_q_d1_zero_dn4)) * locals.var_q_temp) + (assign18980_e18801 * locals.var_q_temp_dn4)) * assign18980_e18808) - (assign18980_e18803 * ((locals.var_q_temp_dn4 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn4)))) / (assign18980_e18808 * assign18980_e18808));
        locals.var_q_eps2_dn6 = ((((((((-locals.var_q_zero_dn6) * locals.var_q_d1_zero) + (assign18980_e18799 * locals.var_q_d1_zero_dn6)) * locals.var_q_temp) + (assign18980_e18801 * locals.var_q_temp_dn6)) * assign18980_e18808) - (assign18980_e18803 * ((locals.var_q_temp_dn6 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn6)))) / (assign18980_e18808 * assign18980_e18808));
        locals.var_q_eps2_dn7 = ((((((((-locals.var_q_zero_dn7) * locals.var_q_d1_zero) + (assign18980_e18799 * locals.var_q_d1_zero_dn7)) * locals.var_q_temp) + (assign18980_e18801 * locals.var_q_temp_dn7)) * assign18980_e18808) - (assign18980_e18803 * ((locals.var_q_temp_dn7 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn7)))) / (assign18980_e18808 * assign18980_e18808));
        locals.var_q_eps2_dn8 = ((((((((-locals.var_q_zero_dn8) * locals.var_q_d1_zero) + (assign18980_e18799 * locals.var_q_d1_zero_dn8)) * locals.var_q_temp) + (assign18980_e18801 * locals.var_q_temp_dn8)) * assign18980_e18808) - (assign18980_e18803 * ((locals.var_q_temp_dn8 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn8)))) / (assign18980_e18808 * assign18980_e18808));
        locals.var_q_eps2_dn9 = ((((((((-locals.var_q_zero_dn9) * locals.var_q_d1_zero) + (assign18980_e18799 * locals.var_q_d1_zero_dn9)) * locals.var_q_temp) + (assign18980_e18801 * locals.var_q_temp_dn9)) * assign18980_e18808) - (assign18980_e18803 * ((locals.var_q_temp_dn9 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn9)))) / (assign18980_e18808 * assign18980_e18808));

        let assign18990_e18812: f64 = (locals.var_q1d + locals.var_q_eps2);
        locals.var_q1d = assign18990_e18812;
        locals.var_q1d_dn4 = (locals.var_q1d_dn4 + locals.var_q_eps2_dn4);
        locals.var_q1d_dn6 = (locals.var_q1d_dn6 + locals.var_q_eps2_dn6);
        locals.var_q1d_dn7 = (locals.var_q1d_dn7 + locals.var_q_eps2_dn7);
        locals.var_q1d_dn8 = (locals.var_q1d_dn8 + locals.var_q_eps2_dn8);
        locals.var_q1d_dn9 = (locals.var_q1d_dn9 + locals.var_q_eps2_dn9);

    }
}
