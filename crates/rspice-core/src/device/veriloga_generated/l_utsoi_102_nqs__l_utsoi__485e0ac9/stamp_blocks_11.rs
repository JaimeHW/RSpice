#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_51(
        locals: &mut StampLocals,
    ) {
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
        locals.var_q_d1_lnexpnum_rv = 0.0;

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
        locals.var_q_d2_lnexpnum_rv = 0.0;

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
        locals.var_q_lnexpnum_rv = 0.0;

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
        locals.var_q_temp1_rv = 0.0;

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
        locals.var_q_d1_lnexpnum_rv = 0.0;

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
        locals.var_q_d2_lnexpnum_rv = 0.0;

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
        locals.var_q_q2_int_rv = 0.0;

        let assign18990_e18868: f64 = (2.0 * locals.var_q_d1_lnexpnum);
        let assign18990_e18869: f64 = (1.0 + assign18990_e18868);
        let assign18990_e18871: f64 = (assign18990_e18869 - locals.var_q_d1_ln);
        locals.var_q_d1_q2 = assign18990_e18871;
        locals.var_q_d1_q2_dn4 = ((2.0 * locals.var_q_d1_lnexpnum_dn4) - locals.var_q_d1_ln_dn4);
        locals.var_q_d1_q2_dn6 = ((2.0 * locals.var_q_d1_lnexpnum_dn6) - locals.var_q_d1_ln_dn6);
        locals.var_q_d1_q2_dn7 = ((2.0 * locals.var_q_d1_lnexpnum_dn7) - locals.var_q_d1_ln_dn7);
        locals.var_q_d1_q2_dn8 = ((2.0 * locals.var_q_d1_lnexpnum_dn8) - locals.var_q_d1_ln_dn8);
        locals.var_q_d1_q2_dn9 = ((2.0 * locals.var_q_d1_lnexpnum_dn9) - locals.var_q_d1_ln_dn9);
        locals.var_q_d1_q2_rv = 0.0;

        let assign19000_e18874: f64 = (2.0 * locals.var_q_d2_lnexpnum);
        let assign19000_e18876: f64 = (assign19000_e18874 - locals.var_q_d2_ln);
        locals.var_q_d2_q2 = assign19000_e18876;
        locals.var_q_d2_q2_dn4 = ((2.0 * locals.var_q_d2_lnexpnum_dn4) - locals.var_q_d2_ln_dn4);
        locals.var_q_d2_q2_dn6 = ((2.0 * locals.var_q_d2_lnexpnum_dn6) - locals.var_q_d2_ln_dn6);
        locals.var_q_d2_q2_dn7 = ((2.0 * locals.var_q_d2_lnexpnum_dn7) - locals.var_q_d2_ln_dn7);
        locals.var_q_d2_q2_dn8 = ((2.0 * locals.var_q_d2_lnexpnum_dn8) - locals.var_q_d2_ln_dn8);
        locals.var_q_d2_q2_dn9 = ((2.0 * locals.var_q_d2_lnexpnum_dn9) - locals.var_q_d2_ln_dn9);
        locals.var_q_d2_q2_rv = 0.0;

        let assign19010_e18880: f64 = (locals.var_k2 * locals.var_q_q2_int);
        let assign19010_e18881: f64 = (locals.var_q_k1q1 + assign19010_e18880);
        locals.var_q_qi_int = assign19010_e18881;
        locals.var_q_qi_int_dn4 = (locals.var_q_k1q1_dn4 + ((locals.var_k2_dn4 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn4)));
        locals.var_q_qi_int_dn6 = (locals.var_q_k1q1_dn6 + ((locals.var_k2_dn6 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn6)));
        locals.var_q_qi_int_dn7 = (locals.var_q_k1q1_dn7 + ((locals.var_k2_dn7 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn7)));
        locals.var_q_qi_int_dn8 = (locals.var_q_k1q1_dn8 + ((locals.var_k2_dn8 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn8)));
        locals.var_q_qi_int_dn9 = (locals.var_q_k1q1_dn9 + ((locals.var_k2_dn9 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn9)));
        locals.var_q_qi_int_rv = 0.0;

        let assign19020_e18885: f64 = (locals.var_k2 * locals.var_q_d1_q2);
        let assign19020_e18886: f64 = (locals.var_k1 + assign19020_e18885);
        locals.var_q_d1_qi = assign19020_e18886;
        locals.var_q_d1_qi_dn4 = (locals.var_k1_dn4 + ((locals.var_k2_dn4 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn4)));
        locals.var_q_d1_qi_dn6 = (locals.var_k1_dn6 + ((locals.var_k2_dn6 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn6)));
        locals.var_q_d1_qi_dn7 = (locals.var_k1_dn7 + ((locals.var_k2_dn7 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn7)));
        locals.var_q_d1_qi_dn8 = (locals.var_k1_dn8 + ((locals.var_k2_dn8 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn8)));
        locals.var_q_d1_qi_dn9 = (locals.var_k1_dn9 + ((locals.var_k2_dn9 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn9)));
        locals.var_q_d1_qi_rv = 0.0;

        let assign19030_e18889: f64 = (locals.var_k2 * locals.var_q_d2_q2);
        locals.var_q_d2_qi = assign19030_e18889;
        locals.var_q_d2_qi_dn4 = ((locals.var_k2_dn4 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn4));
        locals.var_q_d2_qi_dn6 = ((locals.var_k2_dn6 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn6));
        locals.var_q_d2_qi_dn7 = ((locals.var_k2_dn7 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn7));
        locals.var_q_d2_qi_dn8 = ((locals.var_k2_dn8 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn8));
        locals.var_q_d2_qi_dn9 = ((locals.var_k2_dn9 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn9));
        locals.var_q_d2_qi_rv = 0.0;

        let assign19040_e18892: f64 = (locals.var_q_qi_int * locals.var_q_expnum);
        let assign19040_e18894: f64 = (assign19040_e18892 - locals.var_q_aexp);
        locals.var_q_zero = assign19040_e18894;
        locals.var_q_zero_dn4 = (((locals.var_q_qi_int_dn4 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4);
        locals.var_q_zero_dn6 = (((locals.var_q_qi_int_dn6 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6);
        locals.var_q_zero_dn7 = (((locals.var_q_qi_int_dn7 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7);
        locals.var_q_zero_dn8 = (((locals.var_q_qi_int_dn8 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8);
        locals.var_q_zero_dn9 = (((locals.var_q_qi_int_dn9 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9);
        locals.var_q_zero_rv = 0.0;

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
        locals.var_q_d1_zero_rv = 0.0;

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
        locals.var_q_d2_zero_rv = 0.0;

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
        locals.var_q_temp_rv = 0.0;

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
        locals.var_q_eps2_rv = 0.0;

        let assign19090_e18942: f64 = (locals.var_q1d + locals.var_q_eps2);
        locals.var_q1d = assign19090_e18942;
        locals.var_q1d_dn4 = (locals.var_q1d_dn4 + locals.var_q_eps2_dn4);
        locals.var_q1d_dn6 = (locals.var_q1d_dn6 + locals.var_q_eps2_dn6);
        locals.var_q1d_dn7 = (locals.var_q1d_dn7 + locals.var_q_eps2_dn7);
        locals.var_q1d_dn8 = (locals.var_q1d_dn8 + locals.var_q_eps2_dn8);
        locals.var_q1d_dn9 = (locals.var_q1d_dn9 + locals.var_q_eps2_dn9);
        locals.var_q1d_rv = 0.0;

        let assign19100_e18945: f64 = (locals.var_k1 * locals.var_q1d);
        locals.var_q_k1q1 = assign19100_e18945;
        locals.var_q_k1q1_dn4 = ((locals.var_k1_dn4 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn4));
        locals.var_q_k1q1_dn6 = ((locals.var_k1_dn6 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn6));
        locals.var_q_k1q1_dn7 = ((locals.var_k1_dn7 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn7));
        locals.var_q_k1q1_dn8 = ((locals.var_k1_dn8 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn8));
        locals.var_q_k1q1_dn9 = ((locals.var_k1_dn9 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn9));
        locals.var_q_k1q1_rv = 0.0;

        let assign19110_e18948: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign19110_e18950: f64 = (assign19110_e18948 - locals.var_xdeff);
        let assign19110_e18952: f64 = if assign19110_e18950 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard639 = assign19110_e18952;
        locals.var_guard639_rv = 0.0;

        let (assign19120_e18961, assign19120_e18961_d_n4, assign19120_e18961_d_n6, assign19120_e18961_d_n7, assign19120_e18961_d_n8, assign19120_e18961_d_n9,) = {
    if (locals.var_guard639 != 0.0) {
        let assign19120_e18956: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign19120_e18958: f64 = (assign19120_e18956 - locals.var_xdeff);
        let assign19120_e18959: f64 = (assign19120_e18958).exp();
        (assign19120_e18959, (assign19120_e18959 * ((locals.var_xg1x_dn4 - locals.var_q1d_dn4) - locals.var_xdeff_dn4)), (assign19120_e18959 * ((locals.var_xg1x_dn6 - locals.var_q1d_dn6) - locals.var_xdeff_dn6)), (assign19120_e18959 * ((locals.var_xg1x_dn7 - locals.var_q1d_dn7) - locals.var_xdeff_dn7)), (assign19120_e18959 * ((locals.var_xg1x_dn8 - locals.var_q1d_dn8) - locals.var_xdeff_dn8)), (assign19120_e18959 * ((locals.var_xg1x_dn9 - locals.var_q1d_dn9) - locals.var_xdeff_dn9)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign19120_e18961;
        locals.var_q_temp1_dn4 = assign19120_e18961_d_n4;
        locals.var_q_temp1_dn6 = assign19120_e18961_d_n6;
        locals.var_q_temp1_dn7 = assign19120_e18961_d_n7;
        locals.var_q_temp1_dn8 = assign19120_e18961_d_n8;
        locals.var_q_temp1_dn9 = assign19120_e18961_d_n9;
        locals.var_q_temp1_rv = 0.0;

        let (assign19130_e19000, assign19130_e19000_d_n4, assign19130_e19000_d_n6, assign19130_e19000_d_n7, assign19130_e19000_d_n8, assign19130_e19000_d_n9,) = {
    if (locals.var_guard639 == 0.0) {
        let assign19130_e18968: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign19130_e18970: f64 = (assign19130_e18968 - locals.var_xdeff);
        let assign19130_e18972: f64 = (assign19130_e18970 - 80.0);
        let assign19130_e18977: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign19130_e18979: f64 = (assign19130_e18977 - locals.var_xdeff);
        let assign19130_e18981: f64 = (assign19130_e18979 - 80.0);
        let assign19130_e18982: f64 = (0.5 * assign19130_e18981);
        let assign19130_e18986: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign19130_e18988: f64 = (assign19130_e18986 - locals.var_xdeff);
        let assign19130_e18990: f64 = (assign19130_e18988 - 80.0);
        let assign19130_e18992: f64 = (assign19130_e18990 * 0.3333333333333);
        let assign19130_e18993: f64 = (1.0 + assign19130_e18992);
        let assign19130_e18994: f64 = (assign19130_e18982 * assign19130_e18993);
        let assign19130_e18995: f64 = (1.0 + assign19130_e18994);
        let assign19130_e18996: f64 = (assign19130_e18972 * assign19130_e18995);
        let assign19130_e18997: f64 = (1.0 + assign19130_e18996);
        let assign19130_e18998: f64 = (5.54062e34 * assign19130_e18997);
        (assign19130_e18998, (5.54062e34 * ((((locals.var_xg1x_dn4 - locals.var_q1d_dn4) - locals.var_xdeff_dn4) * assign19130_e18995) + (assign19130_e18972 * (((0.5 * ((locals.var_xg1x_dn4 - locals.var_q1d_dn4) - locals.var_xdeff_dn4)) * assign19130_e18993) + (assign19130_e18982 * (((locals.var_xg1x_dn4 - locals.var_q1d_dn4) - locals.var_xdeff_dn4) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x_dn6 - locals.var_q1d_dn6) - locals.var_xdeff_dn6) * assign19130_e18995) + (assign19130_e18972 * (((0.5 * ((locals.var_xg1x_dn6 - locals.var_q1d_dn6) - locals.var_xdeff_dn6)) * assign19130_e18993) + (assign19130_e18982 * (((locals.var_xg1x_dn6 - locals.var_q1d_dn6) - locals.var_xdeff_dn6) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x_dn7 - locals.var_q1d_dn7) - locals.var_xdeff_dn7) * assign19130_e18995) + (assign19130_e18972 * (((0.5 * ((locals.var_xg1x_dn7 - locals.var_q1d_dn7) - locals.var_xdeff_dn7)) * assign19130_e18993) + (assign19130_e18982 * (((locals.var_xg1x_dn7 - locals.var_q1d_dn7) - locals.var_xdeff_dn7) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x_dn8 - locals.var_q1d_dn8) - locals.var_xdeff_dn8) * assign19130_e18995) + (assign19130_e18972 * (((0.5 * ((locals.var_xg1x_dn8 - locals.var_q1d_dn8) - locals.var_xdeff_dn8)) * assign19130_e18993) + (assign19130_e18982 * (((locals.var_xg1x_dn8 - locals.var_q1d_dn8) - locals.var_xdeff_dn8) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x_dn9 - locals.var_q1d_dn9) - locals.var_xdeff_dn9) * assign19130_e18995) + (assign19130_e18972 * (((0.5 * ((locals.var_xg1x_dn9 - locals.var_q1d_dn9) - locals.var_xdeff_dn9)) * assign19130_e18993) + (assign19130_e18982 * (((locals.var_xg1x_dn9 - locals.var_q1d_dn9) - locals.var_xdeff_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign19130_e19000;
        locals.var_q_temp1_dn4 = assign19130_e19000_d_n4;
        locals.var_q_temp1_dn6 = assign19130_e19000_d_n6;
        locals.var_q_temp1_dn7 = assign19130_e19000_d_n7;
        locals.var_q_temp1_dn8 = assign19130_e19000_d_n8;
        locals.var_q_temp1_dn9 = assign19130_e19000_d_n9;
        locals.var_q_temp1_rv = 0.0;

        let assign19140_e19003: f64 = (locals.var_a0 * locals.var_q_temp1);
        locals.var_q_aexp = assign19140_e19003;
        locals.var_q_aexp_dn4 = ((locals.var_a0_dn4 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn4));
        locals.var_q_aexp_dn6 = ((locals.var_a0_dn6 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn6));
        locals.var_q_aexp_dn7 = ((locals.var_a0_dn7 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn7));
        locals.var_q_aexp_dn8 = ((locals.var_a0_dn8 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn8));
        locals.var_q_aexp_dn9 = ((locals.var_a0_dn9 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn9));
        locals.var_q_aexp_rv = 0.0;

        let assign19150_e19006: f64 = (locals.var_q_k1q1 * locals.var_q_k1q1);
        let assign19150_e19008: f64 = (assign19150_e19006 - locals.var_q_aexp);
        locals.var_q_qsq = assign19150_e19008;
        locals.var_q_qsq_dn4 = (((locals.var_q_k1q1_dn4 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn4)) - locals.var_q_aexp_dn4);
        locals.var_q_qsq_dn6 = (((locals.var_q_k1q1_dn6 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn6)) - locals.var_q_aexp_dn6);
        locals.var_q_qsq_dn7 = (((locals.var_q_k1q1_dn7 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn7)) - locals.var_q_aexp_dn7);
        locals.var_q_qsq_dn8 = (((locals.var_q_k1q1_dn8 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn8)) - locals.var_q_aexp_dn8);
        locals.var_q_qsq_dn9 = (((locals.var_q_k1q1_dn9 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn9)) - locals.var_q_aexp_dn9);
        locals.var_q_qsq_rv = 0.0;

        let assign19160_e19011: f64 = (2.0 * locals.var_k1);
        let assign19160_e19013: f64 = (assign19160_e19011 * locals.var_q_k1q1);
        let assign19160_e19015: f64 = (assign19160_e19013 + locals.var_q_aexp);
        locals.var_q_d1_qsq = assign19160_e19015;
        locals.var_q_d1_qsq_dn4 = ((((2.0 * locals.var_k1_dn4) * locals.var_q_k1q1) + (assign19160_e19011 * locals.var_q_k1q1_dn4)) + locals.var_q_aexp_dn4);
        locals.var_q_d1_qsq_dn6 = ((((2.0 * locals.var_k1_dn6) * locals.var_q_k1q1) + (assign19160_e19011 * locals.var_q_k1q1_dn6)) + locals.var_q_aexp_dn6);
        locals.var_q_d1_qsq_dn7 = ((((2.0 * locals.var_k1_dn7) * locals.var_q_k1q1) + (assign19160_e19011 * locals.var_q_k1q1_dn7)) + locals.var_q_aexp_dn7);
        locals.var_q_d1_qsq_dn8 = ((((2.0 * locals.var_k1_dn8) * locals.var_q_k1q1) + (assign19160_e19011 * locals.var_q_k1q1_dn8)) + locals.var_q_aexp_dn8);
        locals.var_q_d1_qsq_dn9 = ((((2.0 * locals.var_k1_dn9) * locals.var_q_k1q1) + (assign19160_e19011 * locals.var_q_k1q1_dn9)) + locals.var_q_aexp_dn9);
        locals.var_q_d1_qsq_rv = 0.0;

        let assign19170_e19018: f64 = (2.0 * locals.var_k1);
        let assign19170_e19020: f64 = (assign19170_e19018 * locals.var_k1);
        let assign19170_e19022: f64 = (assign19170_e19020 - locals.var_q_aexp);
        locals.var_q_d2_qsq = assign19170_e19022;
        locals.var_q_d2_qsq_dn4 = ((((2.0 * locals.var_k1_dn4) * locals.var_k1) + (assign19170_e19018 * locals.var_k1_dn4)) - locals.var_q_aexp_dn4);
        locals.var_q_d2_qsq_dn6 = ((((2.0 * locals.var_k1_dn6) * locals.var_k1) + (assign19170_e19018 * locals.var_k1_dn6)) - locals.var_q_aexp_dn6);
        locals.var_q_d2_qsq_dn7 = ((((2.0 * locals.var_k1_dn7) * locals.var_k1) + (assign19170_e19018 * locals.var_k1_dn7)) - locals.var_q_aexp_dn7);
        locals.var_q_d2_qsq_dn8 = ((((2.0 * locals.var_k1_dn8) * locals.var_k1) + (assign19170_e19018 * locals.var_k1_dn8)) - locals.var_q_aexp_dn8);
        locals.var_q_d2_qsq_dn9 = ((((2.0 * locals.var_k1_dn9) * locals.var_k1) + (assign19170_e19018 * locals.var_k1_dn9)) - locals.var_q_aexp_dn9);
        locals.var_q_d2_qsq_rv = 0.0;

        let assign19180_e19025: f64 = (-0.005);
        let assign19180_e19026: f64 = if locals.var_q_qsq < assign19180_e19025 { 1.0 } else { 0.0 };
        locals.var_guard640 = assign19180_e19026;
        locals.var_guard640_rv = 0.0;

        let (assign19190_e19032, assign19190_e19032_d_n4, assign19190_e19032_d_n6, assign19190_e19032_d_n7, assign19190_e19032_d_n8, assign19190_e19032_d_n9,) = {
    if (locals.var_guard640 != 0.0) {
        let assign19190_e19029: f64 = (locals.var_q_qsq).abs();
        let assign19190_e19030: f64 = (assign19190_e19029).sqrt();
        (assign19190_e19030, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign19190_e19030)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign19190_e19030)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign19190_e19030)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign19190_e19030)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign19190_e19030)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign19190_e19032;
        locals.var_q_rac_qsq_dn4 = assign19190_e19032_d_n4;
        locals.var_q_rac_qsq_dn6 = assign19190_e19032_d_n6;
        locals.var_q_rac_qsq_dn7 = assign19190_e19032_d_n7;
        locals.var_q_rac_qsq_dn8 = assign19190_e19032_d_n8;
        locals.var_q_rac_qsq_dn9 = assign19190_e19032_d_n9;
        locals.var_q_rac_qsq_rv = 0.0;

        let (assign19200_e19041, assign19200_e19041_d_n4, assign19200_e19041_d_n6, assign19200_e19041_d_n7, assign19200_e19041_d_n8, assign19200_e19041_d_n9,) = {
    if (locals.var_guard640 != 0.0) {
        let assign19200_e19037: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign19200_e19038: f64 = (assign19200_e19037).tan();
        let assign19200_e19039: f64 = (locals.var_q_rac_qsq / assign19200_e19038);
        (assign19200_e19039, (((locals.var_q_rac_qsq_dn4 * assign19200_e19038) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn4) / ((assign19200_e19037).cos() * (assign19200_e19037).cos())))) / (assign19200_e19038 * assign19200_e19038)), (((locals.var_q_rac_qsq_dn6 * assign19200_e19038) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn6) / ((assign19200_e19037).cos() * (assign19200_e19037).cos())))) / (assign19200_e19038 * assign19200_e19038)), (((locals.var_q_rac_qsq_dn7 * assign19200_e19038) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn7) / ((assign19200_e19037).cos() * (assign19200_e19037).cos())))) / (assign19200_e19038 * assign19200_e19038)), (((locals.var_q_rac_qsq_dn8 * assign19200_e19038) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn8) / ((assign19200_e19037).cos() * (assign19200_e19037).cos())))) / (assign19200_e19038 * assign19200_e19038)), (((locals.var_q_rac_qsq_dn9 * assign19200_e19038) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn9) / ((assign19200_e19037).cos() * (assign19200_e19037).cos())))) / (assign19200_e19038 * assign19200_e19038)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign19200_e19041;
        locals.var_q_qcoth_dn4 = assign19200_e19041_d_n4;
        locals.var_q_qcoth_dn6 = assign19200_e19041_d_n6;
        locals.var_q_qcoth_dn7 = assign19200_e19041_d_n7;
        locals.var_q_qcoth_dn8 = assign19200_e19041_d_n8;
        locals.var_q_qcoth_dn9 = assign19200_e19041_d_n9;
        locals.var_q_qcoth_rv = 0.0;

        let (assign19210_e19049, assign19210_e19049_d_n4, assign19210_e19049_d_n6, assign19210_e19049_d_n7, assign19210_e19049_d_n8, assign19210_e19049_d_n9,) = {
    if (locals.var_guard640 != 0.0) {
        let assign19210_e19045: f64 = (0.25 * locals.var_q_d1_qsq);
        let assign19210_e19047: f64 = (assign19210_e19045 / locals.var_q_qsq);
        (assign19210_e19047, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign19210_e19045 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign19210_e19045 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign19210_e19045 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign19210_e19045 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign19210_e19045 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign19210_e19049;
        locals.var_q_temp1_dn4 = assign19210_e19049_d_n4;
        locals.var_q_temp1_dn6 = assign19210_e19049_d_n6;
        locals.var_q_temp1_dn7 = assign19210_e19049_d_n7;
        locals.var_q_temp1_dn8 = assign19210_e19049_d_n8;
        locals.var_q_temp1_dn9 = assign19210_e19049_d_n9;
        locals.var_q_temp1_rv = 0.0;

        let (assign19220_e19061, assign19220_e19061_d_n4, assign19220_e19061_d_n6, assign19220_e19061_d_n7, assign19220_e19061_d_n8, assign19220_e19061_d_n9,) = {
    if (locals.var_guard640 != 0.0) {
        let assign19220_e19055: f64 = (2.0 - locals.var_q_qcoth);
        let assign19220_e19056: f64 = (locals.var_q_qcoth * assign19220_e19055);
        let assign19220_e19057: f64 = (locals.var_q_qsq + assign19220_e19056);
        let assign19220_e19059: f64 = (assign19220_e19057 * locals.var_q_temp1);
        (assign19220_e19059, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign19220_e19055) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign19220_e19057 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign19220_e19055) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign19220_e19057 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign19220_e19055) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign19220_e19057 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign19220_e19055) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign19220_e19057 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign19220_e19055) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign19220_e19057 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign19220_e19061;
        locals.var_q_d1_qcoth_dn4 = assign19220_e19061_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign19220_e19061_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign19220_e19061_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign19220_e19061_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign19220_e19061_d_n9;
        locals.var_q_d1_qcoth_rv = 0.0;

        let (assign19230_e19081, assign19230_e19081_d_n4, assign19230_e19081_d_n6, assign19230_e19081_d_n7, assign19230_e19081_d_n8, assign19230_e19081_d_n9,) = {
    if (locals.var_guard640 != 0.0) {
        let assign19230_e19066: f64 = (2.0 * locals.var_q_d1_qcoth);
        let assign19230_e19069: f64 = (1.0 + locals.var_q_qcoth);
        let assign19230_e19070: f64 = (assign19230_e19066 * assign19230_e19069);
        let assign19230_e19071: f64 = (locals.var_q_d1_qsq - assign19230_e19070);
        let assign19230_e19073: f64 = (assign19230_e19071 * locals.var_q_temp1);
        let assign19230_e19076: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
        let assign19230_e19078: f64 = (assign19230_e19076 / locals.var_q_d1_qsq);
        let assign19230_e19079: f64 = (assign19230_e19073 + assign19230_e19078);
        (assign19230_e19079, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign19230_e19069) + (assign19230_e19066 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign19230_e19071 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign19230_e19076 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign19230_e19069) + (assign19230_e19066 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign19230_e19071 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign19230_e19076 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign19230_e19069) + (assign19230_e19066 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign19230_e19071 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign19230_e19076 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign19230_e19069) + (assign19230_e19066 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign19230_e19071 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign19230_e19076 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign19230_e19069) + (assign19230_e19066 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign19230_e19071 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign19230_e19076 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign19230_e19081;
        locals.var_q_d2_qcoth_dn4 = assign19230_e19081_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign19230_e19081_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign19230_e19081_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign19230_e19081_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign19230_e19081_d_n9;
        locals.var_q_d2_qcoth_rv = 0.0;

        let (assign19240_e19089, assign19240_e19089_d_n4, assign19240_e19089_d_n6, assign19240_e19089_d_n7, assign19240_e19089_d_n8, assign19240_e19089_d_n9,) = {
    if (locals.var_guard640 != 0.0) {
        let assign19240_e19086: f64 = (0.5 * locals.var_q_qcoth);
        let assign19240_e19087: f64 = (1.0 - assign19240_e19086);
        (assign19240_e19087, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign19240_e19089;
        locals.var_q_temp2_dn4 = assign19240_e19089_d_n4;
        locals.var_q_temp2_dn6 = assign19240_e19089_d_n6;
        locals.var_q_temp2_dn7 = assign19240_e19089_d_n7;
        locals.var_q_temp2_dn8 = assign19240_e19089_d_n8;
        locals.var_q_temp2_dn9 = assign19240_e19089_d_n9;
        locals.var_q_temp2_rv = 0.0;

        let (assign19250_e19097, assign19250_e19097_d_n4, assign19250_e19097_d_n6, assign19250_e19097_d_n7, assign19250_e19097_d_n8, assign19250_e19097_d_n9,) = {
    if (locals.var_guard640 != 0.0) {
        let assign19250_e19093: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
        let assign19250_e19095: f64 = (assign19250_e19093 * locals.var_q_temp2);
        (assign19250_e19095, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign19250_e19093 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign19250_e19093 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign19250_e19093 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign19250_e19093 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign19250_e19093 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign19250_e19097;
        locals.var_q_d1_ln_dn4 = assign19250_e19097_d_n4;
        locals.var_q_d1_ln_dn6 = assign19250_e19097_d_n6;
        locals.var_q_d1_ln_dn7 = assign19250_e19097_d_n7;
        locals.var_q_d1_ln_dn8 = assign19250_e19097_d_n8;
        locals.var_q_d1_ln_dn9 = assign19250_e19097_d_n9;
        locals.var_q_d1_ln_rv = 0.0;

        let (assign19260_e19113, assign19260_e19113_d_n4, assign19260_e19113_d_n6, assign19260_e19113_d_n7, assign19260_e19113_d_n8, assign19260_e19113_d_n9,) = {
    if (locals.var_guard640 != 0.0) {
        let assign19260_e19101: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
        let assign19260_e19106: f64 = (0.5 * locals.var_q_d1_qcoth);
        let assign19260_e19107: f64 = (locals.var_q_d1_ln + assign19260_e19106);
        let assign19260_e19108: f64 = (locals.var_q_d1_qsq * assign19260_e19107);
        let assign19260_e19109: f64 = (assign19260_e19101 - assign19260_e19108);
        let assign19260_e19111: f64 = (assign19260_e19109 / locals.var_q_qsq);
        (assign19260_e19111, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign19260_e19107) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign19260_e19109 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign19260_e19107) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign19260_e19109 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign19260_e19107) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign19260_e19109 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign19260_e19107) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign19260_e19109 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign19260_e19107) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign19260_e19109 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign19260_e19113;
        locals.var_q_d2_ln_dn4 = assign19260_e19113_d_n4;
        locals.var_q_d2_ln_dn6 = assign19260_e19113_d_n6;
        locals.var_q_d2_ln_dn7 = assign19260_e19113_d_n7;
        locals.var_q_d2_ln_dn8 = assign19260_e19113_d_n8;
        locals.var_q_d2_ln_dn9 = assign19260_e19113_d_n9;
        locals.var_q_d2_ln_rv = 0.0;

        let assign19270_e19116: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard641 = assign19270_e19116;
        locals.var_guard641_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_52(
        locals: &mut StampLocals,
    ) {
        let (assign19280_e19125, assign19280_e19125_d_n4, assign19280_e19125_d_n6, assign19280_e19125_d_n7, assign19280_e19125_d_n8, assign19280_e19125_d_n9,) = {
    if ((locals.var_guard640 == 0.0) && (locals.var_guard641 != 0.0)) {
        let assign19280_e19122: f64 = (locals.var_q_qsq).abs();
        let assign19280_e19123: f64 = (assign19280_e19122).sqrt();
        (assign19280_e19123, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign19280_e19123)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign19280_e19123)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign19280_e19123)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign19280_e19123)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign19280_e19123)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign19280_e19125;
        locals.var_q_rac_qsq_dn4 = assign19280_e19125_d_n4;
        locals.var_q_rac_qsq_dn6 = assign19280_e19125_d_n6;
        locals.var_q_rac_qsq_dn7 = assign19280_e19125_d_n7;
        locals.var_q_rac_qsq_dn8 = assign19280_e19125_d_n8;
        locals.var_q_rac_qsq_dn9 = assign19280_e19125_d_n9;
        locals.var_q_rac_qsq_rv = 0.0;

        let (assign19290_e19134, assign19290_e19134_d_n4, assign19290_e19134_d_n6, assign19290_e19134_d_n7, assign19290_e19134_d_n8, assign19290_e19134_d_n9,) = {
    if ((locals.var_guard640 == 0.0) && (locals.var_guard641 != 0.0)) {
        let assign19290_e19131: f64 = (-locals.var_q_rac_qsq);
        let assign19290_e19132: f64 = (assign19290_e19131).exp();
        (assign19290_e19132, (assign19290_e19132 * (-locals.var_q_rac_qsq_dn4)), (assign19290_e19132 * (-locals.var_q_rac_qsq_dn6)), (assign19290_e19132 * (-locals.var_q_rac_qsq_dn7)), (assign19290_e19132 * (-locals.var_q_rac_qsq_dn8)), (assign19290_e19132 * (-locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_invexpq, locals.var_q_invexpq_dn4, locals.var_q_invexpq_dn6, locals.var_q_invexpq_dn7, locals.var_q_invexpq_dn8, locals.var_q_invexpq_dn9,)
    }
};
        locals.var_q_invexpq = assign19290_e19134;
        locals.var_q_invexpq_dn4 = assign19290_e19134_d_n4;
        locals.var_q_invexpq_dn6 = assign19290_e19134_d_n6;
        locals.var_q_invexpq_dn7 = assign19290_e19134_d_n7;
        locals.var_q_invexpq_dn8 = assign19290_e19134_d_n8;
        locals.var_q_invexpq_dn9 = assign19290_e19134_d_n9;
        locals.var_q_invexpq_rv = 0.0;

        let (assign19300_e19149, assign19300_e19149_d_n4, assign19300_e19149_d_n6, assign19300_e19149_d_n7, assign19300_e19149_d_n8, assign19300_e19149_d_n9,) = {
    if ((locals.var_guard640 == 0.0) && (locals.var_guard641 != 0.0)) {
        let assign19300_e19142: f64 = (1.0 + locals.var_q_invexpq);
        let assign19300_e19143: f64 = (locals.var_q_rac_qsq * assign19300_e19142);
        let assign19300_e19146: f64 = (1.0 - locals.var_q_invexpq);
        let assign19300_e19147: f64 = (assign19300_e19143 / assign19300_e19146);
        (assign19300_e19147, (((((locals.var_q_rac_qsq_dn4 * assign19300_e19142) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn4)) * assign19300_e19146) - (assign19300_e19143 * (-locals.var_q_invexpq_dn4))) / (assign19300_e19146 * assign19300_e19146)), (((((locals.var_q_rac_qsq_dn6 * assign19300_e19142) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn6)) * assign19300_e19146) - (assign19300_e19143 * (-locals.var_q_invexpq_dn6))) / (assign19300_e19146 * assign19300_e19146)), (((((locals.var_q_rac_qsq_dn7 * assign19300_e19142) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn7)) * assign19300_e19146) - (assign19300_e19143 * (-locals.var_q_invexpq_dn7))) / (assign19300_e19146 * assign19300_e19146)), (((((locals.var_q_rac_qsq_dn8 * assign19300_e19142) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn8)) * assign19300_e19146) - (assign19300_e19143 * (-locals.var_q_invexpq_dn8))) / (assign19300_e19146 * assign19300_e19146)), (((((locals.var_q_rac_qsq_dn9 * assign19300_e19142) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn9)) * assign19300_e19146) - (assign19300_e19143 * (-locals.var_q_invexpq_dn9))) / (assign19300_e19146 * assign19300_e19146)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign19300_e19149;
        locals.var_q_qcoth_dn4 = assign19300_e19149_d_n4;
        locals.var_q_qcoth_dn6 = assign19300_e19149_d_n6;
        locals.var_q_qcoth_dn7 = assign19300_e19149_d_n7;
        locals.var_q_qcoth_dn8 = assign19300_e19149_d_n8;
        locals.var_q_qcoth_dn9 = assign19300_e19149_d_n9;
        locals.var_q_qcoth_rv = 0.0;

        let (assign19310_e19160, assign19310_e19160_d_n4, assign19310_e19160_d_n6, assign19310_e19160_d_n7, assign19310_e19160_d_n8, assign19310_e19160_d_n9,) = {
    if ((locals.var_guard640 == 0.0) && (locals.var_guard641 != 0.0)) {
        let assign19310_e19156: f64 = (0.25 * locals.var_q_d1_qsq);
        let assign19310_e19158: f64 = (assign19310_e19156 / locals.var_q_qsq);
        (assign19310_e19158, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign19310_e19156 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign19310_e19156 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign19310_e19156 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign19310_e19156 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign19310_e19156 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign19310_e19160;
        locals.var_q_temp1_dn4 = assign19310_e19160_d_n4;
        locals.var_q_temp1_dn6 = assign19310_e19160_d_n6;
        locals.var_q_temp1_dn7 = assign19310_e19160_d_n7;
        locals.var_q_temp1_dn8 = assign19310_e19160_d_n8;
        locals.var_q_temp1_dn9 = assign19310_e19160_d_n9;
        locals.var_q_temp1_rv = 0.0;

        let (assign19320_e19175, assign19320_e19175_d_n4, assign19320_e19175_d_n6, assign19320_e19175_d_n7, assign19320_e19175_d_n8, assign19320_e19175_d_n9,) = {
    if ((locals.var_guard640 == 0.0) && (locals.var_guard641 != 0.0)) {
        let assign19320_e19169: f64 = (2.0 - locals.var_q_qcoth);
        let assign19320_e19170: f64 = (locals.var_q_qcoth * assign19320_e19169);
        let assign19320_e19171: f64 = (locals.var_q_qsq + assign19320_e19170);
        let assign19320_e19173: f64 = (assign19320_e19171 * locals.var_q_temp1);
        (assign19320_e19173, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign19320_e19169) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign19320_e19171 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign19320_e19169) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign19320_e19171 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign19320_e19169) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign19320_e19171 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign19320_e19169) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign19320_e19171 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign19320_e19169) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign19320_e19171 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign19320_e19175;
        locals.var_q_d1_qcoth_dn4 = assign19320_e19175_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign19320_e19175_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign19320_e19175_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign19320_e19175_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign19320_e19175_d_n9;
        locals.var_q_d1_qcoth_rv = 0.0;

        let (assign19330_e19198, assign19330_e19198_d_n4, assign19330_e19198_d_n6, assign19330_e19198_d_n7, assign19330_e19198_d_n8, assign19330_e19198_d_n9,) = {
    if ((locals.var_guard640 == 0.0) && (locals.var_guard641 != 0.0)) {
        let assign19330_e19183: f64 = (2.0 * locals.var_q_d1_qcoth);
        let assign19330_e19186: f64 = (1.0 + locals.var_q_qcoth);
        let assign19330_e19187: f64 = (assign19330_e19183 * assign19330_e19186);
        let assign19330_e19188: f64 = (locals.var_q_d1_qsq - assign19330_e19187);
        let assign19330_e19190: f64 = (assign19330_e19188 * locals.var_q_temp1);
        let assign19330_e19193: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
        let assign19330_e19195: f64 = (assign19330_e19193 / locals.var_q_d1_qsq);
        let assign19330_e19196: f64 = (assign19330_e19190 + assign19330_e19195);
        (assign19330_e19196, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign19330_e19186) + (assign19330_e19183 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign19330_e19188 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign19330_e19193 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign19330_e19186) + (assign19330_e19183 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign19330_e19188 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign19330_e19193 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign19330_e19186) + (assign19330_e19183 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign19330_e19188 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign19330_e19193 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign19330_e19186) + (assign19330_e19183 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign19330_e19188 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign19330_e19193 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign19330_e19186) + (assign19330_e19183 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign19330_e19188 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign19330_e19193 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign19330_e19198;
        locals.var_q_d2_qcoth_dn4 = assign19330_e19198_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign19330_e19198_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign19330_e19198_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign19330_e19198_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign19330_e19198_d_n9;
        locals.var_q_d2_qcoth_rv = 0.0;

        let (assign19340_e19209, assign19340_e19209_d_n4, assign19340_e19209_d_n6, assign19340_e19209_d_n7, assign19340_e19209_d_n8, assign19340_e19209_d_n9,) = {
    if ((locals.var_guard640 == 0.0) && (locals.var_guard641 != 0.0)) {
        let assign19340_e19206: f64 = (0.5 * locals.var_q_qcoth);
        let assign19340_e19207: f64 = (1.0 - assign19340_e19206);
        (assign19340_e19207, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign19340_e19209;
        locals.var_q_temp2_dn4 = assign19340_e19209_d_n4;
        locals.var_q_temp2_dn6 = assign19340_e19209_d_n6;
        locals.var_q_temp2_dn7 = assign19340_e19209_d_n7;
        locals.var_q_temp2_dn8 = assign19340_e19209_d_n8;
        locals.var_q_temp2_dn9 = assign19340_e19209_d_n9;
        locals.var_q_temp2_rv = 0.0;

        let (assign19350_e19220, assign19350_e19220_d_n4, assign19350_e19220_d_n6, assign19350_e19220_d_n7, assign19350_e19220_d_n8, assign19350_e19220_d_n9,) = {
    if ((locals.var_guard640 == 0.0) && (locals.var_guard641 != 0.0)) {
        let assign19350_e19216: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
        let assign19350_e19218: f64 = (assign19350_e19216 * locals.var_q_temp2);
        (assign19350_e19218, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign19350_e19216 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign19350_e19216 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign19350_e19216 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign19350_e19216 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign19350_e19216 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign19350_e19220;
        locals.var_q_d1_ln_dn4 = assign19350_e19220_d_n4;
        locals.var_q_d1_ln_dn6 = assign19350_e19220_d_n6;
        locals.var_q_d1_ln_dn7 = assign19350_e19220_d_n7;
        locals.var_q_d1_ln_dn8 = assign19350_e19220_d_n8;
        locals.var_q_d1_ln_dn9 = assign19350_e19220_d_n9;
        locals.var_q_d1_ln_rv = 0.0;

        let (assign19360_e19239, assign19360_e19239_d_n4, assign19360_e19239_d_n6, assign19360_e19239_d_n7, assign19360_e19239_d_n8, assign19360_e19239_d_n9,) = {
    if ((locals.var_guard640 == 0.0) && (locals.var_guard641 != 0.0)) {
        let assign19360_e19227: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
        let assign19360_e19232: f64 = (0.5 * locals.var_q_d1_qcoth);
        let assign19360_e19233: f64 = (locals.var_q_d1_ln + assign19360_e19232);
        let assign19360_e19234: f64 = (locals.var_q_d1_qsq * assign19360_e19233);
        let assign19360_e19235: f64 = (assign19360_e19227 - assign19360_e19234);
        let assign19360_e19237: f64 = (assign19360_e19235 / locals.var_q_qsq);
        (assign19360_e19237, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign19360_e19233) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign19360_e19235 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign19360_e19233) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign19360_e19235 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign19360_e19233) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign19360_e19235 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign19360_e19233) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign19360_e19235 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign19360_e19233) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign19360_e19235 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign19360_e19239;
        locals.var_q_d2_ln_dn4 = assign19360_e19239_d_n4;
        locals.var_q_d2_ln_dn6 = assign19360_e19239_d_n6;
        locals.var_q_d2_ln_dn7 = assign19360_e19239_d_n7;
        locals.var_q_d2_ln_dn8 = assign19360_e19239_d_n8;
        locals.var_q_d2_ln_dn9 = assign19360_e19239_d_n9;
        locals.var_q_d2_ln_rv = 0.0;

        let (assign19370_e19265, assign19370_e19265_d_n4, assign19370_e19265_d_n6, assign19370_e19265_d_n7, assign19370_e19265_d_n8, assign19370_e19265_d_n9,) = {
    if ((locals.var_guard640 == 0.0) && (locals.var_guard641 == 0.0)) {
        let assign19370_e19249: f64 = (locals.var_q_qsq * 0.0166666666667);
        let assign19370_e19253: f64 = (locals.var_q_qsq * 0.0238095238095);
        let assign19370_e19257: f64 = (locals.var_q_qsq * 0.025);
        let assign19370_e19258: f64 = (1.0 - assign19370_e19257);
        let assign19370_e19259: f64 = (assign19370_e19253 * assign19370_e19258);
        let assign19370_e19260: f64 = (1.0 - assign19370_e19259);
        let assign19370_e19261: f64 = (assign19370_e19249 * assign19370_e19260);
        let assign19370_e19262: f64 = (1.0 - assign19370_e19261);
        let assign19370_e19263: f64 = (0.1666666666667 * assign19370_e19262);
        (assign19370_e19263, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0166666666667) * assign19370_e19260) + (assign19370_e19249 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign19370_e19258) + (assign19370_e19253 * (-(locals.var_q_qsq_dn4 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0166666666667) * assign19370_e19260) + (assign19370_e19249 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign19370_e19258) + (assign19370_e19253 * (-(locals.var_q_qsq_dn6 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0166666666667) * assign19370_e19260) + (assign19370_e19249 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign19370_e19258) + (assign19370_e19253 * (-(locals.var_q_qsq_dn7 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0166666666667) * assign19370_e19260) + (assign19370_e19249 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign19370_e19258) + (assign19370_e19253 * (-(locals.var_q_qsq_dn8 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0166666666667) * assign19370_e19260) + (assign19370_e19249 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign19370_e19258) + (assign19370_e19253 * (-(locals.var_q_qsq_dn9 * 0.025))))))))),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign19370_e19265;
        locals.var_q_temp3_dn4 = assign19370_e19265_d_n4;
        locals.var_q_temp3_dn6 = assign19370_e19265_d_n6;
        locals.var_q_temp3_dn7 = assign19370_e19265_d_n7;
        locals.var_q_temp3_dn8 = assign19370_e19265_d_n8;
        locals.var_q_temp3_dn9 = assign19370_e19265_d_n9;
        locals.var_q_temp3_rv = 0.0;

        let (assign19380_e19277, assign19380_e19277_d_n4, assign19380_e19277_d_n6, assign19380_e19277_d_n7, assign19380_e19277_d_n8, assign19380_e19277_d_n9,) = {
    if ((locals.var_guard640 == 0.0) && (locals.var_guard641 == 0.0)) {
        let assign19380_e19274: f64 = (locals.var_q_qsq * locals.var_q_temp3);
        let assign19380_e19275: f64 = (2.0 + assign19380_e19274);
        (assign19380_e19275, ((locals.var_q_qsq_dn4 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn4)), ((locals.var_q_qsq_dn6 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn6)), ((locals.var_q_qsq_dn7 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn7)), ((locals.var_q_qsq_dn8 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn8)), ((locals.var_q_qsq_dn9 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn9)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign19380_e19277;
        locals.var_q_qcoth_dn4 = assign19380_e19277_d_n4;
        locals.var_q_qcoth_dn6 = assign19380_e19277_d_n6;
        locals.var_q_qcoth_dn7 = assign19380_e19277_d_n7;
        locals.var_q_qcoth_dn8 = assign19380_e19277_d_n8;
        locals.var_q_qcoth_dn9 = assign19380_e19277_d_n9;
        locals.var_q_qcoth_rv = 0.0;

        let (assign19390_e19303, assign19390_e19303_d_n4, assign19390_e19303_d_n6, assign19390_e19303_d_n7, assign19390_e19303_d_n8, assign19390_e19303_d_n9,) = {
    if ((locals.var_guard640 == 0.0) && (locals.var_guard641 == 0.0)) {
        let assign19390_e19287: f64 = (locals.var_q_qsq * 0.0333333333333);
        let assign19390_e19291: f64 = (locals.var_q_qsq * 0.0357142857143);
        let assign19390_e19295: f64 = (locals.var_q_qsq * 0.0333333333333);
        let assign19390_e19296: f64 = (1.0 - assign19390_e19295);
        let assign19390_e19297: f64 = (assign19390_e19291 * assign19390_e19296);
        let assign19390_e19298: f64 = (1.0 - assign19390_e19297);
        let assign19390_e19299: f64 = (assign19390_e19287 * assign19390_e19298);
        let assign19390_e19300: f64 = (1.0 - assign19390_e19299);
        let assign19390_e19301: f64 = (0.1666666666667 * assign19390_e19300);
        (assign19390_e19301, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0333333333333) * assign19390_e19298) + (assign19390_e19287 * (-(((locals.var_q_qsq_dn4 * 0.0357142857143) * assign19390_e19296) + (assign19390_e19291 * (-(locals.var_q_qsq_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0333333333333) * assign19390_e19298) + (assign19390_e19287 * (-(((locals.var_q_qsq_dn6 * 0.0357142857143) * assign19390_e19296) + (assign19390_e19291 * (-(locals.var_q_qsq_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0333333333333) * assign19390_e19298) + (assign19390_e19287 * (-(((locals.var_q_qsq_dn7 * 0.0357142857143) * assign19390_e19296) + (assign19390_e19291 * (-(locals.var_q_qsq_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0333333333333) * assign19390_e19298) + (assign19390_e19287 * (-(((locals.var_q_qsq_dn8 * 0.0357142857143) * assign19390_e19296) + (assign19390_e19291 * (-(locals.var_q_qsq_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0333333333333) * assign19390_e19298) + (assign19390_e19287 * (-(((locals.var_q_qsq_dn9 * 0.0357142857143) * assign19390_e19296) + (assign19390_e19291 * (-(locals.var_q_qsq_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign19390_e19303;
        locals.var_q_temp1_dn4 = assign19390_e19303_d_n4;
        locals.var_q_temp1_dn6 = assign19390_e19303_d_n6;
        locals.var_q_temp1_dn7 = assign19390_e19303_d_n7;
        locals.var_q_temp1_dn8 = assign19390_e19303_d_n8;
        locals.var_q_temp1_dn9 = assign19390_e19303_d_n9;
        locals.var_q_temp1_rv = 0.0;

        let (assign19400_e19313, assign19400_e19313_d_n4, assign19400_e19313_d_n6, assign19400_e19313_d_n7, assign19400_e19313_d_n8, assign19400_e19313_d_n9,) = {
    if ((locals.var_guard640 == 0.0) && (locals.var_guard641 == 0.0)) {
        let assign19400_e19311: f64 = (locals.var_q_d1_qsq * locals.var_q_temp1);
        (assign19400_e19311, ((locals.var_q_d1_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn4)), ((locals.var_q_d1_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn6)), ((locals.var_q_d1_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn7)), ((locals.var_q_d1_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn8)), ((locals.var_q_d1_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign19400_e19313;
        locals.var_q_d1_qcoth_dn4 = assign19400_e19313_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign19400_e19313_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign19400_e19313_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign19400_e19313_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign19400_e19313_d_n9;
        locals.var_q_d1_qcoth_rv = 0.0;

        let (assign19410_e19339, assign19410_e19339_d_n4, assign19410_e19339_d_n6, assign19410_e19339_d_n7, assign19410_e19339_d_n8, assign19410_e19339_d_n9,) = {
    if ((locals.var_guard640 == 0.0) && (locals.var_guard641 == 0.0)) {
        let assign19410_e19323: f64 = (locals.var_q_qsq * 0.0714285714286);
        let assign19410_e19327: f64 = (0.05 * locals.var_q_qsq);
        let assign19410_e19331: f64 = (0.0420875420875421 * locals.var_q_qsq);
        let assign19410_e19332: f64 = (1.0 - assign19410_e19331);
        let assign19410_e19333: f64 = (assign19410_e19327 * assign19410_e19332);
        let assign19410_e19334: f64 = (1.0 - assign19410_e19333);
        let assign19410_e19335: f64 = (assign19410_e19323 * assign19410_e19334);
        let assign19410_e19336: f64 = (1.0 - assign19410_e19335);
        let assign19410_e19337: f64 = (0.0055555555556 * assign19410_e19336);
        (assign19410_e19337, (0.0055555555556 * (-(((locals.var_q_qsq_dn4 * 0.0714285714286) * assign19410_e19334) + (assign19410_e19323 * (-(((0.05 * locals.var_q_qsq_dn4) * assign19410_e19332) + (assign19410_e19327 * (-(0.0420875420875421 * locals.var_q_qsq_dn4))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn6 * 0.0714285714286) * assign19410_e19334) + (assign19410_e19323 * (-(((0.05 * locals.var_q_qsq_dn6) * assign19410_e19332) + (assign19410_e19327 * (-(0.0420875420875421 * locals.var_q_qsq_dn6))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn7 * 0.0714285714286) * assign19410_e19334) + (assign19410_e19323 * (-(((0.05 * locals.var_q_qsq_dn7) * assign19410_e19332) + (assign19410_e19327 * (-(0.0420875420875421 * locals.var_q_qsq_dn7))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn8 * 0.0714285714286) * assign19410_e19334) + (assign19410_e19323 * (-(((0.05 * locals.var_q_qsq_dn8) * assign19410_e19332) + (assign19410_e19327 * (-(0.0420875420875421 * locals.var_q_qsq_dn8))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn9 * 0.0714285714286) * assign19410_e19334) + (assign19410_e19323 * (-(((0.05 * locals.var_q_qsq_dn9) * assign19410_e19332) + (assign19410_e19327 * (-(0.0420875420875421 * locals.var_q_qsq_dn9))))))))),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign19410_e19339;
        locals.var_q_temp2_dn4 = assign19410_e19339_d_n4;
        locals.var_q_temp2_dn6 = assign19410_e19339_d_n6;
        locals.var_q_temp2_dn7 = assign19410_e19339_d_n7;
        locals.var_q_temp2_dn8 = assign19410_e19339_d_n8;
        locals.var_q_temp2_dn9 = assign19410_e19339_d_n9;
        locals.var_q_temp2_rv = 0.0;

        let (assign19420_e19355, assign19420_e19355_d_n4, assign19420_e19355_d_n6, assign19420_e19355_d_n7, assign19420_e19355_d_n8, assign19420_e19355_d_n9,) = {
    if ((locals.var_guard640 == 0.0) && (locals.var_guard641 == 0.0)) {
        let assign19420_e19347: f64 = (locals.var_q_d2_qsq * locals.var_q_temp1);
        let assign19420_e19350: f64 = (locals.var_q_d1_qsq * locals.var_q_d1_qsq);
        let assign19420_e19352: f64 = (assign19420_e19350 * locals.var_q_temp2);
        let assign19420_e19353: f64 = (assign19420_e19347 - assign19420_e19352);
        (assign19420_e19353, (((locals.var_q_d2_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn4)) - ((((locals.var_q_d1_qsq_dn4 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn4)) * locals.var_q_temp2) + (assign19420_e19350 * locals.var_q_temp2_dn4))), (((locals.var_q_d2_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn6)) - ((((locals.var_q_d1_qsq_dn6 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn6)) * locals.var_q_temp2) + (assign19420_e19350 * locals.var_q_temp2_dn6))), (((locals.var_q_d2_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn7)) - ((((locals.var_q_d1_qsq_dn7 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn7)) * locals.var_q_temp2) + (assign19420_e19350 * locals.var_q_temp2_dn7))), (((locals.var_q_d2_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn8)) - ((((locals.var_q_d1_qsq_dn8 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn8)) * locals.var_q_temp2) + (assign19420_e19350 * locals.var_q_temp2_dn8))), (((locals.var_q_d2_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn9)) - ((((locals.var_q_d1_qsq_dn9 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn9)) * locals.var_q_temp2) + (assign19420_e19350 * locals.var_q_temp2_dn9))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign19420_e19355;
        locals.var_q_d2_qcoth_dn4 = assign19420_e19355_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign19420_e19355_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign19420_e19355_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign19420_e19355_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign19420_e19355_d_n9;
        locals.var_q_d2_qcoth_rv = 0.0;

        let (assign19430_e19368, assign19430_e19368_d_n4, assign19430_e19368_d_n6, assign19430_e19368_d_n7, assign19430_e19368_d_n8, assign19430_e19368_d_n9,) = {
    if ((locals.var_guard640 == 0.0) && (locals.var_guard641 == 0.0)) {
        let assign19430_e19362: f64 = (-0.5);
        let assign19430_e19364: f64 = (assign19430_e19362 * locals.var_q_d1_qsq);
        let assign19430_e19366: f64 = (assign19430_e19364 * locals.var_q_temp3);
        (assign19430_e19366, (((assign19430_e19362 * locals.var_q_d1_qsq_dn4) * locals.var_q_temp3) + (assign19430_e19364 * locals.var_q_temp3_dn4)), (((assign19430_e19362 * locals.var_q_d1_qsq_dn6) * locals.var_q_temp3) + (assign19430_e19364 * locals.var_q_temp3_dn6)), (((assign19430_e19362 * locals.var_q_d1_qsq_dn7) * locals.var_q_temp3) + (assign19430_e19364 * locals.var_q_temp3_dn7)), (((assign19430_e19362 * locals.var_q_d1_qsq_dn8) * locals.var_q_temp3) + (assign19430_e19364 * locals.var_q_temp3_dn8)), (((assign19430_e19362 * locals.var_q_d1_qsq_dn9) * locals.var_q_temp3) + (assign19430_e19364 * locals.var_q_temp3_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign19430_e19368;
        locals.var_q_d1_ln_dn4 = assign19430_e19368_d_n4;
        locals.var_q_d1_ln_dn6 = assign19430_e19368_d_n6;
        locals.var_q_d1_ln_dn7 = assign19430_e19368_d_n7;
        locals.var_q_d1_ln_dn8 = assign19430_e19368_d_n8;
        locals.var_q_d1_ln_dn9 = assign19430_e19368_d_n9;
        locals.var_q_d1_ln_rv = 0.0;

        let (assign19440_e19401, assign19440_e19401_d_n4, assign19440_e19401_d_n6, assign19440_e19401_d_n7, assign19440_e19401_d_n8, assign19440_e19401_d_n9,) = {
    if ((locals.var_guard640 == 0.0) && (locals.var_guard641 == 0.0)) {
        let assign19440_e19375: f64 = (-0.5);
        let assign19440_e19377: f64 = (assign19440_e19375 * locals.var_q_d2_qsq);
        let assign19440_e19379: f64 = (assign19440_e19377 * locals.var_q_temp3);
        let assign19440_e19382: f64 = (0.25 * 0.0055555555556);
        let assign19440_e19384: f64 = (assign19440_e19382 * locals.var_q_d1_qsq);
        let assign19440_e19386: f64 = (assign19440_e19384 * locals.var_q_d1_qsq);
        let assign19440_e19390: f64 = (locals.var_q_qsq * 0.0238095238095);
        let assign19440_e19394: f64 = (0.075 * locals.var_q_qsq);
        let assign19440_e19395: f64 = (2.0 - assign19440_e19394);
        let assign19440_e19396: f64 = (assign19440_e19390 * assign19440_e19395);
        let assign19440_e19397: f64 = (1.0 - assign19440_e19396);
        let assign19440_e19398: f64 = (assign19440_e19386 * assign19440_e19397);
        let assign19440_e19399: f64 = (assign19440_e19379 + assign19440_e19398);
        (assign19440_e19399, ((((assign19440_e19375 * locals.var_q_d2_qsq_dn4) * locals.var_q_temp3) + (assign19440_e19377 * locals.var_q_temp3_dn4)) + (((((assign19440_e19382 * locals.var_q_d1_qsq_dn4) * locals.var_q_d1_qsq) + (assign19440_e19384 * locals.var_q_d1_qsq_dn4)) * assign19440_e19397) + (assign19440_e19386 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign19440_e19395) + (assign19440_e19390 * (-(0.075 * locals.var_q_qsq_dn4)))))))), ((((assign19440_e19375 * locals.var_q_d2_qsq_dn6) * locals.var_q_temp3) + (assign19440_e19377 * locals.var_q_temp3_dn6)) + (((((assign19440_e19382 * locals.var_q_d1_qsq_dn6) * locals.var_q_d1_qsq) + (assign19440_e19384 * locals.var_q_d1_qsq_dn6)) * assign19440_e19397) + (assign19440_e19386 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign19440_e19395) + (assign19440_e19390 * (-(0.075 * locals.var_q_qsq_dn6)))))))), ((((assign19440_e19375 * locals.var_q_d2_qsq_dn7) * locals.var_q_temp3) + (assign19440_e19377 * locals.var_q_temp3_dn7)) + (((((assign19440_e19382 * locals.var_q_d1_qsq_dn7) * locals.var_q_d1_qsq) + (assign19440_e19384 * locals.var_q_d1_qsq_dn7)) * assign19440_e19397) + (assign19440_e19386 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign19440_e19395) + (assign19440_e19390 * (-(0.075 * locals.var_q_qsq_dn7)))))))), ((((assign19440_e19375 * locals.var_q_d2_qsq_dn8) * locals.var_q_temp3) + (assign19440_e19377 * locals.var_q_temp3_dn8)) + (((((assign19440_e19382 * locals.var_q_d1_qsq_dn8) * locals.var_q_d1_qsq) + (assign19440_e19384 * locals.var_q_d1_qsq_dn8)) * assign19440_e19397) + (assign19440_e19386 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign19440_e19395) + (assign19440_e19390 * (-(0.075 * locals.var_q_qsq_dn8)))))))), ((((assign19440_e19375 * locals.var_q_d2_qsq_dn9) * locals.var_q_temp3) + (assign19440_e19377 * locals.var_q_temp3_dn9)) + (((((assign19440_e19382 * locals.var_q_d1_qsq_dn9) * locals.var_q_d1_qsq) + (assign19440_e19384 * locals.var_q_d1_qsq_dn9)) * assign19440_e19397) + (assign19440_e19386 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign19440_e19395) + (assign19440_e19390 * (-(0.075 * locals.var_q_qsq_dn9)))))))),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign19440_e19401;
        locals.var_q_d2_ln_dn4 = assign19440_e19401_d_n4;
        locals.var_q_d2_ln_dn6 = assign19440_e19401_d_n6;
        locals.var_q_d2_ln_dn7 = assign19440_e19401_d_n7;
        locals.var_q_d2_ln_dn8 = assign19440_e19401_d_n8;
        locals.var_q_d2_ln_dn9 = assign19440_e19401_d_n9;
        locals.var_q_d2_ln_rv = 0.0;

        let assign19450_e19404: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard642 = assign19450_e19404;
        locals.var_guard642_rv = 0.0;

        let (assign19460_e19418, assign19460_e19418_d_n4, assign19460_e19418_d_n6, assign19460_e19418_d_n7, assign19460_e19418_d_n8, assign19460_e19418_d_n9,) = {
    if (locals.var_guard642 != 0.0) {
        let assign19460_e19408: f64 = (4.0 * locals.var_q_qsq);
        let assign19460_e19413: f64 = (2.0 - locals.var_q_invexpq);
        let assign19460_e19414: f64 = (locals.var_q_invexpq * assign19460_e19413);
        let assign19460_e19415: f64 = (1.0 - assign19460_e19414);
        let assign19460_e19416: f64 = (assign19460_e19408 / assign19460_e19415);
        (assign19460_e19416, ((((4.0 * locals.var_q_qsq_dn4) * assign19460_e19415) - (assign19460_e19408 * (-((locals.var_q_invexpq_dn4 * assign19460_e19413) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn4)))))) / (assign19460_e19415 * assign19460_e19415)), ((((4.0 * locals.var_q_qsq_dn6) * assign19460_e19415) - (assign19460_e19408 * (-((locals.var_q_invexpq_dn6 * assign19460_e19413) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn6)))))) / (assign19460_e19415 * assign19460_e19415)), ((((4.0 * locals.var_q_qsq_dn7) * assign19460_e19415) - (assign19460_e19408 * (-((locals.var_q_invexpq_dn7 * assign19460_e19413) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn7)))))) / (assign19460_e19415 * assign19460_e19415)), ((((4.0 * locals.var_q_qsq_dn8) * assign19460_e19415) - (assign19460_e19408 * (-((locals.var_q_invexpq_dn8 * assign19460_e19413) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn8)))))) / (assign19460_e19415 * assign19460_e19415)), ((((4.0 * locals.var_q_qsq_dn9) * assign19460_e19415) - (assign19460_e19408 * (-((locals.var_q_invexpq_dn9 * assign19460_e19413) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn9)))))) / (assign19460_e19415 * assign19460_e19415)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign19460_e19418;
        locals.var_q_temp2_dn4 = assign19460_e19418_d_n4;
        locals.var_q_temp2_dn6 = assign19460_e19418_d_n6;
        locals.var_q_temp2_dn7 = assign19460_e19418_d_n7;
        locals.var_q_temp2_dn8 = assign19460_e19418_d_n8;
        locals.var_q_temp2_dn9 = assign19460_e19418_d_n9;
        locals.var_q_temp2_rv = 0.0;

        let (assign19470_e19424, assign19470_e19424_d_n4, assign19470_e19424_d_n6, assign19470_e19424_d_n7, assign19470_e19424_d_n8, assign19470_e19424_d_n9,) = {
    if (locals.var_guard642 != 0.0) {
        let assign19470_e19422: f64 = (locals.var_q_temp2 * locals.var_q_invexpq);
        (assign19470_e19422, ((locals.var_q_temp2_dn4 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn4)), ((locals.var_q_temp2_dn6 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn6)), ((locals.var_q_temp2_dn7 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn7)), ((locals.var_q_temp2_dn8 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn8)), ((locals.var_q_temp2_dn9 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn9)),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign19470_e19424;
        locals.var_q_sh_term_dn4 = assign19470_e19424_d_n4;
        locals.var_q_sh_term_dn6 = assign19470_e19424_d_n6;
        locals.var_q_sh_term_dn7 = assign19470_e19424_d_n7;
        locals.var_q_sh_term_dn8 = assign19470_e19424_d_n8;
        locals.var_q_sh_term_dn9 = assign19470_e19424_d_n9;
        locals.var_q_sh_term_rv = 0.0;

        let (assign19480_e19431, assign19480_e19431_d_n4, assign19480_e19431_d_n6, assign19480_e19431_d_n7, assign19480_e19431_d_n8, assign19480_e19431_d_n9,) = {
    if (locals.var_guard642 != 0.0) {
        let assign19480_e19427: f64 = (locals.var_q_temp2).ln();
        let assign19480_e19429: f64 = (assign19480_e19427 - locals.var_q_rac_qsq);
        (assign19480_e19429, ((locals.var_q_temp2_dn4 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn4), ((locals.var_q_temp2_dn6 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn6), ((locals.var_q_temp2_dn7 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn7), ((locals.var_q_temp2_dn8 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn8), ((locals.var_q_temp2_dn9 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn9),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign19480_e19431;
        locals.var_q_ln_term_dn4 = assign19480_e19431_d_n4;
        locals.var_q_ln_term_dn6 = assign19480_e19431_d_n6;
        locals.var_q_ln_term_dn7 = assign19480_e19431_d_n7;
        locals.var_q_ln_term_dn8 = assign19480_e19431_d_n8;
        locals.var_q_ln_term_dn9 = assign19480_e19431_d_n9;
        locals.var_q_ln_term_rv = 0.0;

        let assign19490_e19434: f64 = (-0.005);
        let assign19490_e19435: f64 = if locals.var_q_qsq < assign19490_e19434 { 1.0 } else { 0.0 };
        locals.var_guard643 = assign19490_e19435;
        locals.var_guard643_rv = 0.0;

        let (assign19500_e19445, assign19500_e19445_d_n4, assign19500_e19445_d_n6, assign19500_e19445_d_n7, assign19500_e19445_d_n8, assign19500_e19445_d_n9,) = {
    if ((locals.var_guard642 == 0.0) && (locals.var_guard643 != 0.0)) {
        let assign19500_e19442: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign19500_e19443: f64 = (assign19500_e19442).sin();
        (assign19500_e19443, ((assign19500_e19442).cos() * (0.5 * locals.var_q_rac_qsq_dn4)), ((assign19500_e19442).cos() * (0.5 * locals.var_q_rac_qsq_dn6)), ((assign19500_e19442).cos() * (0.5 * locals.var_q_rac_qsq_dn7)), ((assign19500_e19442).cos() * (0.5 * locals.var_q_rac_qsq_dn8)), ((assign19500_e19442).cos() * (0.5 * locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign19500_e19445;
        locals.var_q_temp2_dn4 = assign19500_e19445_d_n4;
        locals.var_q_temp2_dn6 = assign19500_e19445_d_n6;
        locals.var_q_temp2_dn7 = assign19500_e19445_d_n7;
        locals.var_q_temp2_dn8 = assign19500_e19445_d_n8;
        locals.var_q_temp2_dn9 = assign19500_e19445_d_n9;
        locals.var_q_temp2_rv = 0.0;

        let (assign19510_e19457, assign19510_e19457_d_n4, assign19510_e19457_d_n6, assign19510_e19457_d_n7, assign19510_e19457_d_n8, assign19510_e19457_d_n9,) = {
    if ((locals.var_guard642 == 0.0) && (locals.var_guard643 != 0.0)) {
        let assign19510_e19451: f64 = (-locals.var_q_qsq);
        let assign19510_e19454: f64 = (locals.var_q_temp2 * locals.var_q_temp2);
        let assign19510_e19455: f64 = (assign19510_e19451 / assign19510_e19454);
        (assign19510_e19455, ((((-locals.var_q_qsq_dn4) * assign19510_e19454) - (assign19510_e19451 * ((locals.var_q_temp2_dn4 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn4)))) / (assign19510_e19454 * assign19510_e19454)), ((((-locals.var_q_qsq_dn6) * assign19510_e19454) - (assign19510_e19451 * ((locals.var_q_temp2_dn6 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn6)))) / (assign19510_e19454 * assign19510_e19454)), ((((-locals.var_q_qsq_dn7) * assign19510_e19454) - (assign19510_e19451 * ((locals.var_q_temp2_dn7 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn7)))) / (assign19510_e19454 * assign19510_e19454)), ((((-locals.var_q_qsq_dn8) * assign19510_e19454) - (assign19510_e19451 * ((locals.var_q_temp2_dn8 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn8)))) / (assign19510_e19454 * assign19510_e19454)), ((((-locals.var_q_qsq_dn9) * assign19510_e19454) - (assign19510_e19451 * ((locals.var_q_temp2_dn9 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn9)))) / (assign19510_e19454 * assign19510_e19454)),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign19510_e19457;
        locals.var_q_sh_term_dn4 = assign19510_e19457_d_n4;
        locals.var_q_sh_term_dn6 = assign19510_e19457_d_n6;
        locals.var_q_sh_term_dn7 = assign19510_e19457_d_n7;
        locals.var_q_sh_term_dn8 = assign19510_e19457_d_n8;
        locals.var_q_sh_term_dn9 = assign19510_e19457_d_n9;
        locals.var_q_sh_term_rv = 0.0;

        let (assign19520_e19465, assign19520_e19465_d_n4, assign19520_e19465_d_n6, assign19520_e19465_d_n7, assign19520_e19465_d_n8, assign19520_e19465_d_n9,) = {
    if ((locals.var_guard642 == 0.0) && (locals.var_guard643 != 0.0)) {
        let assign19520_e19463: f64 = (locals.var_q_sh_term).ln();
        (assign19520_e19463, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign19520_e19465;
        locals.var_q_ln_term_dn4 = assign19520_e19465_d_n4;
        locals.var_q_ln_term_dn6 = assign19520_e19465_d_n6;
        locals.var_q_ln_term_dn7 = assign19520_e19465_d_n7;
        locals.var_q_ln_term_dn8 = assign19520_e19465_d_n8;
        locals.var_q_ln_term_dn9 = assign19520_e19465_d_n9;
        locals.var_q_ln_term_rv = 0.0;

        let (assign19530_e19489, assign19530_e19489_d_n4, assign19530_e19489_d_n6, assign19530_e19489_d_n7, assign19530_e19489_d_n8, assign19530_e19489_d_n9,) = {
    if ((locals.var_guard642 == 0.0) && (locals.var_guard643 == 0.0)) {
        let assign19530_e19474: f64 = (locals.var_q_qsq * 0.3333333333333);
        let assign19530_e19478: f64 = (0.05 * locals.var_q_qsq);
        let assign19530_e19482: f64 = (0.0396825396825397 * locals.var_q_qsq);
        let assign19530_e19483: f64 = (1.0 - assign19530_e19482);
        let assign19530_e19484: f64 = (assign19530_e19478 * assign19530_e19483);
        let assign19530_e19485: f64 = (1.0 - assign19530_e19484);
        let assign19530_e19486: f64 = (assign19530_e19474 * assign19530_e19485);
        let assign19530_e19487: f64 = (4.0 - assign19530_e19486);
        (assign19530_e19487, (-(((locals.var_q_qsq_dn4 * 0.3333333333333) * assign19530_e19485) + (assign19530_e19474 * (-(((0.05 * locals.var_q_qsq_dn4) * assign19530_e19483) + (assign19530_e19478 * (-(0.0396825396825397 * locals.var_q_qsq_dn4)))))))), (-(((locals.var_q_qsq_dn6 * 0.3333333333333) * assign19530_e19485) + (assign19530_e19474 * (-(((0.05 * locals.var_q_qsq_dn6) * assign19530_e19483) + (assign19530_e19478 * (-(0.0396825396825397 * locals.var_q_qsq_dn6)))))))), (-(((locals.var_q_qsq_dn7 * 0.3333333333333) * assign19530_e19485) + (assign19530_e19474 * (-(((0.05 * locals.var_q_qsq_dn7) * assign19530_e19483) + (assign19530_e19478 * (-(0.0396825396825397 * locals.var_q_qsq_dn7)))))))), (-(((locals.var_q_qsq_dn8 * 0.3333333333333) * assign19530_e19485) + (assign19530_e19474 * (-(((0.05 * locals.var_q_qsq_dn8) * assign19530_e19483) + (assign19530_e19478 * (-(0.0396825396825397 * locals.var_q_qsq_dn8)))))))), (-(((locals.var_q_qsq_dn9 * 0.3333333333333) * assign19530_e19485) + (assign19530_e19474 * (-(((0.05 * locals.var_q_qsq_dn9) * assign19530_e19483) + (assign19530_e19478 * (-(0.0396825396825397 * locals.var_q_qsq_dn9)))))))),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign19530_e19489;
        locals.var_q_sh_term_dn4 = assign19530_e19489_d_n4;
        locals.var_q_sh_term_dn6 = assign19530_e19489_d_n6;
        locals.var_q_sh_term_dn7 = assign19530_e19489_d_n7;
        locals.var_q_sh_term_dn8 = assign19530_e19489_d_n8;
        locals.var_q_sh_term_dn9 = assign19530_e19489_d_n9;
        locals.var_q_sh_term_rv = 0.0;

        let (assign19540_e19498, assign19540_e19498_d_n4, assign19540_e19498_d_n6, assign19540_e19498_d_n7, assign19540_e19498_d_n8, assign19540_e19498_d_n9,) = {
    if ((locals.var_guard642 == 0.0) && (locals.var_guard643 == 0.0)) {
        let assign19540_e19496: f64 = (locals.var_q_sh_term).ln();
        (assign19540_e19496, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign19540_e19498;
        locals.var_q_ln_term_dn4 = assign19540_e19498_d_n4;
        locals.var_q_ln_term_dn6 = assign19540_e19498_d_n6;
        locals.var_q_ln_term_dn7 = assign19540_e19498_d_n7;
        locals.var_q_ln_term_dn8 = assign19540_e19498_d_n8;
        locals.var_q_ln_term_dn9 = assign19540_e19498_d_n9;
        locals.var_q_ln_term_rv = 0.0;

        let assign19550_e19501: f64 = (1.01 * locals.var_q_k1q1);
        let assign19550_e19503: f64 = (assign19550_e19501 + locals.var_q_qcoth);
        let assign19550_e19505: f64 = if assign19550_e19503 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard644 = assign19550_e19505;
        locals.var_guard644_rv = 0.0;

        let (assign19560_e19511, assign19560_e19511_d_n4, assign19560_e19511_d_n6, assign19560_e19511_d_n7, assign19560_e19511_d_n8, assign19560_e19511_d_n9,) = {
    if (locals.var_guard644 != 0.0) {
        let assign19560_e19509: f64 = (locals.var_q_k1q1 + locals.var_q_qcoth);
        (assign19560_e19509, (locals.var_q_k1q1_dn4 + locals.var_q_qcoth_dn4), (locals.var_q_k1q1_dn6 + locals.var_q_qcoth_dn6), (locals.var_q_k1q1_dn7 + locals.var_q_qcoth_dn7), (locals.var_q_k1q1_dn8 + locals.var_q_qcoth_dn8), (locals.var_q_k1q1_dn9 + locals.var_q_qcoth_dn9),)
    } else {
        (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9,)
    }
};
        locals.var_q_expnum = assign19560_e19511;
        locals.var_q_expnum_dn4 = assign19560_e19511_d_n4;
        locals.var_q_expnum_dn6 = assign19560_e19511_d_n6;
        locals.var_q_expnum_dn7 = assign19560_e19511_d_n7;
        locals.var_q_expnum_dn8 = assign19560_e19511_d_n8;
        locals.var_q_expnum_dn9 = assign19560_e19511_d_n9;
        locals.var_q_expnum_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_53(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign19570_e19517, assign19570_e19517_d_n4, assign19570_e19517_d_n6, assign19570_e19517_d_n7, assign19570_e19517_d_n8, assign19570_e19517_d_n9,) = {
    if (locals.var_guard644 != 0.0) {
        let assign19570_e19515: f64 = (locals.var_k1 + locals.var_q_d1_qcoth);
        (assign19570_e19515, (locals.var_k1_dn4 + locals.var_q_d1_qcoth_dn4), (locals.var_k1_dn6 + locals.var_q_d1_qcoth_dn6), (locals.var_k1_dn7 + locals.var_q_d1_qcoth_dn7), (locals.var_k1_dn8 + locals.var_q_d1_qcoth_dn8), (locals.var_k1_dn9 + locals.var_q_d1_qcoth_dn9),)
    } else {
        (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9,)
    }
};
        locals.var_q_d1_expnum = assign19570_e19517;
        locals.var_q_d1_expnum_dn4 = assign19570_e19517_d_n4;
        locals.var_q_d1_expnum_dn6 = assign19570_e19517_d_n6;
        locals.var_q_d1_expnum_dn7 = assign19570_e19517_d_n7;
        locals.var_q_d1_expnum_dn8 = assign19570_e19517_d_n8;
        locals.var_q_d1_expnum_dn9 = assign19570_e19517_d_n9;
        locals.var_q_d1_expnum_rv = 0.0;

        let (assign19580_e19521, assign19580_e19521_d_n4, assign19580_e19521_d_n6, assign19580_e19521_d_n7, assign19580_e19521_d_n8, assign19580_e19521_d_n9,) = {
    if (locals.var_guard644 != 0.0) {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    } else {
        (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9,)
    }
};
        locals.var_q_d2_expnum = assign19580_e19521;
        locals.var_q_d2_expnum_dn4 = assign19580_e19521_d_n4;
        locals.var_q_d2_expnum_dn6 = assign19580_e19521_d_n6;
        locals.var_q_d2_expnum_dn7 = assign19580_e19521_d_n7;
        locals.var_q_d2_expnum_dn8 = assign19580_e19521_d_n8;
        locals.var_q_d2_expnum_dn9 = assign19580_e19521_d_n9;
        locals.var_q_d2_expnum_rv = 0.0;

        let (assign19590_e19530, assign19590_e19530_d_n4, assign19590_e19530_d_n6, assign19590_e19530_d_n7, assign19590_e19530_d_n8, assign19590_e19530_d_n9,) = {
    if (locals.var_guard644 == 0.0) {
        let assign19590_e19527: f64 = (locals.var_q_k1q1 - locals.var_q_qcoth);
        let assign19590_e19528: f64 = (1.0 / assign19590_e19527);
        (assign19590_e19528, (-((locals.var_q_k1q1_dn4 - locals.var_q_qcoth_dn4) / (assign19590_e19527 * assign19590_e19527))), (-((locals.var_q_k1q1_dn6 - locals.var_q_qcoth_dn6) / (assign19590_e19527 * assign19590_e19527))), (-((locals.var_q_k1q1_dn7 - locals.var_q_qcoth_dn7) / (assign19590_e19527 * assign19590_e19527))), (-((locals.var_q_k1q1_dn8 - locals.var_q_qcoth_dn8) / (assign19590_e19527 * assign19590_e19527))), (-((locals.var_q_k1q1_dn9 - locals.var_q_qcoth_dn9) / (assign19590_e19527 * assign19590_e19527))),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign19590_e19530;
        locals.var_q_temp2_dn4 = assign19590_e19530_d_n4;
        locals.var_q_temp2_dn6 = assign19590_e19530_d_n6;
        locals.var_q_temp2_dn7 = assign19590_e19530_d_n7;
        locals.var_q_temp2_dn8 = assign19590_e19530_d_n8;
        locals.var_q_temp2_dn9 = assign19590_e19530_d_n9;
        locals.var_q_temp2_rv = 0.0;

        let (assign19600_e19537, assign19600_e19537_d_n4, assign19600_e19537_d_n6, assign19600_e19537_d_n7, assign19600_e19537_d_n8, assign19600_e19537_d_n9,) = {
    if (locals.var_guard644 == 0.0) {
        let assign19600_e19535: f64 = (locals.var_q_d1_qcoth - locals.var_k1);
        (assign19600_e19535, (locals.var_q_d1_qcoth_dn4 - locals.var_k1_dn4), (locals.var_q_d1_qcoth_dn6 - locals.var_k1_dn6), (locals.var_q_d1_qcoth_dn7 - locals.var_k1_dn7), (locals.var_q_d1_qcoth_dn8 - locals.var_k1_dn8), (locals.var_q_d1_qcoth_dn9 - locals.var_k1_dn9),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign19600_e19537;
        locals.var_q_temp3_dn4 = assign19600_e19537_d_n4;
        locals.var_q_temp3_dn6 = assign19600_e19537_d_n6;
        locals.var_q_temp3_dn7 = assign19600_e19537_d_n7;
        locals.var_q_temp3_dn8 = assign19600_e19537_d_n8;
        locals.var_q_temp3_dn9 = assign19600_e19537_d_n9;
        locals.var_q_temp3_rv = 0.0;

        let (assign19610_e19546, assign19610_e19546_d_n4, assign19610_e19546_d_n6, assign19610_e19546_d_n7, assign19610_e19546_d_n8, assign19610_e19546_d_n9,) = {
    if (locals.var_guard644 == 0.0) {
        let assign19610_e19542: f64 = (locals.var_q_aexp - locals.var_q_sh_term);
        let assign19610_e19544: f64 = (assign19610_e19542 * locals.var_q_temp2);
        (assign19610_e19544, (((locals.var_q_aexp_dn4 - locals.var_q_sh_term_dn4) * locals.var_q_temp2) + (assign19610_e19542 * locals.var_q_temp2_dn4)), (((locals.var_q_aexp_dn6 - locals.var_q_sh_term_dn6) * locals.var_q_temp2) + (assign19610_e19542 * locals.var_q_temp2_dn6)), (((locals.var_q_aexp_dn7 - locals.var_q_sh_term_dn7) * locals.var_q_temp2) + (assign19610_e19542 * locals.var_q_temp2_dn7)), (((locals.var_q_aexp_dn8 - locals.var_q_sh_term_dn8) * locals.var_q_temp2) + (assign19610_e19542 * locals.var_q_temp2_dn8)), (((locals.var_q_aexp_dn9 - locals.var_q_sh_term_dn9) * locals.var_q_temp2) + (assign19610_e19542 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9,)
    }
};
        locals.var_q_expnum = assign19610_e19546;
        locals.var_q_expnum_dn4 = assign19610_e19546_d_n4;
        locals.var_q_expnum_dn6 = assign19610_e19546_d_n6;
        locals.var_q_expnum_dn7 = assign19610_e19546_d_n7;
        locals.var_q_expnum_dn8 = assign19610_e19546_d_n8;
        locals.var_q_expnum_dn9 = assign19610_e19546_d_n9;
        locals.var_q_expnum_rv = 0.0;

        let (assign19620_e19561, assign19620_e19561_d_n4, assign19620_e19561_d_n6, assign19620_e19561_d_n7, assign19620_e19561_d_n8, assign19620_e19561_d_n9,) = {
    if (locals.var_guard644 == 0.0) {
        let assign19620_e19551: f64 = (locals.var_q_temp3 * locals.var_q_expnum);
        let assign19620_e19553: f64 = (assign19620_e19551 - locals.var_q_aexp);
        let assign19620_e19556: f64 = (locals.var_q_d1_ln * locals.var_q_sh_term);
        let assign19620_e19557: f64 = (assign19620_e19553 - assign19620_e19556);
        let assign19620_e19559: f64 = (assign19620_e19557 * locals.var_q_temp2);
        (assign19620_e19559, ((((((locals.var_q_temp3_dn4 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4) - ((locals.var_q_d1_ln_dn4 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign19620_e19557 * locals.var_q_temp2_dn4)), ((((((locals.var_q_temp3_dn6 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6) - ((locals.var_q_d1_ln_dn6 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign19620_e19557 * locals.var_q_temp2_dn6)), ((((((locals.var_q_temp3_dn7 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7) - ((locals.var_q_d1_ln_dn7 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign19620_e19557 * locals.var_q_temp2_dn7)), ((((((locals.var_q_temp3_dn8 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8) - ((locals.var_q_d1_ln_dn8 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign19620_e19557 * locals.var_q_temp2_dn8)), ((((((locals.var_q_temp3_dn9 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9) - ((locals.var_q_d1_ln_dn9 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign19620_e19557 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9,)
    }
};
        locals.var_q_d1_expnum = assign19620_e19561;
        locals.var_q_d1_expnum_dn4 = assign19620_e19561_d_n4;
        locals.var_q_d1_expnum_dn6 = assign19620_e19561_d_n6;
        locals.var_q_d1_expnum_dn7 = assign19620_e19561_d_n7;
        locals.var_q_d1_expnum_dn8 = assign19620_e19561_d_n8;
        locals.var_q_d1_expnum_dn9 = assign19620_e19561_d_n9;
        locals.var_q_d1_expnum_rv = 0.0;

        let (assign19630_e19586, assign19630_e19586_d_n4, assign19630_e19586_d_n6, assign19630_e19586_d_n7, assign19630_e19586_d_n8, assign19630_e19586_d_n9,) = {
    if (locals.var_guard644 == 0.0) {
        let assign19630_e19566: f64 = (locals.var_q_d2_qcoth * locals.var_q_expnum);
        let assign19630_e19569: f64 = (2.0 * locals.var_q_temp3);
        let assign19630_e19571: f64 = (assign19630_e19569 * locals.var_q_d1_expnum);
        let assign19630_e19572: f64 = (assign19630_e19566 + assign19630_e19571);
        let assign19630_e19574: f64 = (assign19630_e19572 + locals.var_q_aexp);
        let assign19630_e19578: f64 = (locals.var_q_d1_ln * locals.var_q_d1_ln);
        let assign19630_e19579: f64 = (locals.var_q_d2_ln + assign19630_e19578);
        let assign19630_e19581: f64 = (assign19630_e19579 * locals.var_q_sh_term);
        let assign19630_e19582: f64 = (assign19630_e19574 - assign19630_e19581);
        let assign19630_e19584: f64 = (assign19630_e19582 * locals.var_q_temp2);
        (assign19630_e19584, (((((((locals.var_q_d2_qcoth_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_temp3_dn4) * locals.var_q_d1_expnum) + (assign19630_e19569 * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4) - (((locals.var_q_d2_ln_dn4 + ((locals.var_q_d1_ln_dn4 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn4))) * locals.var_q_sh_term) + (assign19630_e19579 * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign19630_e19582 * locals.var_q_temp2_dn4)), (((((((locals.var_q_d2_qcoth_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_temp3_dn6) * locals.var_q_d1_expnum) + (assign19630_e19569 * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6) - (((locals.var_q_d2_ln_dn6 + ((locals.var_q_d1_ln_dn6 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn6))) * locals.var_q_sh_term) + (assign19630_e19579 * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign19630_e19582 * locals.var_q_temp2_dn6)), (((((((locals.var_q_d2_qcoth_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_temp3_dn7) * locals.var_q_d1_expnum) + (assign19630_e19569 * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7) - (((locals.var_q_d2_ln_dn7 + ((locals.var_q_d1_ln_dn7 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn7))) * locals.var_q_sh_term) + (assign19630_e19579 * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign19630_e19582 * locals.var_q_temp2_dn7)), (((((((locals.var_q_d2_qcoth_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_temp3_dn8) * locals.var_q_d1_expnum) + (assign19630_e19569 * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8) - (((locals.var_q_d2_ln_dn8 + ((locals.var_q_d1_ln_dn8 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn8))) * locals.var_q_sh_term) + (assign19630_e19579 * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign19630_e19582 * locals.var_q_temp2_dn8)), (((((((locals.var_q_d2_qcoth_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_temp3_dn9) * locals.var_q_d1_expnum) + (assign19630_e19569 * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9) - (((locals.var_q_d2_ln_dn9 + ((locals.var_q_d1_ln_dn9 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn9))) * locals.var_q_sh_term) + (assign19630_e19579 * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign19630_e19582 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9,)
    }
};
        locals.var_q_d2_expnum = assign19630_e19586;
        locals.var_q_d2_expnum_dn4 = assign19630_e19586_d_n4;
        locals.var_q_d2_expnum_dn6 = assign19630_e19586_d_n6;
        locals.var_q_d2_expnum_dn7 = assign19630_e19586_d_n7;
        locals.var_q_d2_expnum_dn8 = assign19630_e19586_d_n8;
        locals.var_q_d2_expnum_dn9 = assign19630_e19586_d_n9;
        locals.var_q_d2_expnum_rv = 0.0;

        let assign19640_e19589: f64 = if locals.var_q_expnum > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard645 = assign19640_e19589;
        locals.var_guard645_rv = 0.0;

        let (assign19650_e19594, assign19650_e19594_d_n4, assign19650_e19594_d_n6, assign19650_e19594_d_n7, assign19650_e19594_d_n8, assign19650_e19594_d_n9,) = {
    if (locals.var_guard645 != 0.0) {
        let assign19650_e19592: f64 = (locals.var_q_expnum).ln();
        (assign19650_e19592, (locals.var_q_expnum_dn4 / locals.var_q_expnum), (locals.var_q_expnum_dn6 / locals.var_q_expnum), (locals.var_q_expnum_dn7 / locals.var_q_expnum), (locals.var_q_expnum_dn8 / locals.var_q_expnum), (locals.var_q_expnum_dn9 / locals.var_q_expnum),)
    } else {
        (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9,)
    }
};
        locals.var_q_lnexpnum = assign19650_e19594;
        locals.var_q_lnexpnum_dn4 = assign19650_e19594_d_n4;
        locals.var_q_lnexpnum_dn6 = assign19650_e19594_d_n6;
        locals.var_q_lnexpnum_dn7 = assign19650_e19594_d_n7;
        locals.var_q_lnexpnum_dn8 = assign19650_e19594_d_n8;
        locals.var_q_lnexpnum_dn9 = assign19650_e19594_d_n9;
        locals.var_q_lnexpnum_rv = 0.0;

        let (assign19660_e19600, assign19660_e19600_d_n4, assign19660_e19600_d_n6, assign19660_e19600_d_n7, assign19660_e19600_d_n8, assign19660_e19600_d_n9,) = {
    if (locals.var_guard645 != 0.0) {
        let assign19660_e19598: f64 = (1.0 / locals.var_q_expnum);
        (assign19660_e19598, (-(locals.var_q_expnum_dn4 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn6 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn7 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn8 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn9 / (locals.var_q_expnum * locals.var_q_expnum))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign19660_e19600;
        locals.var_q_temp1_dn4 = assign19660_e19600_d_n4;
        locals.var_q_temp1_dn6 = assign19660_e19600_d_n6;
        locals.var_q_temp1_dn7 = assign19660_e19600_d_n7;
        locals.var_q_temp1_dn8 = assign19660_e19600_d_n8;
        locals.var_q_temp1_dn9 = assign19660_e19600_d_n9;
        locals.var_q_temp1_rv = 0.0;

        let (assign19670_e19606, assign19670_e19606_d_n4, assign19670_e19606_d_n6, assign19670_e19606_d_n7, assign19670_e19606_d_n8, assign19670_e19606_d_n9,) = {
    if (locals.var_guard645 != 0.0) {
        let assign19670_e19604: f64 = (locals.var_q_d1_expnum * locals.var_q_temp1);
        (assign19670_e19604, ((locals.var_q_d1_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn4)), ((locals.var_q_d1_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn6)), ((locals.var_q_d1_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn7)), ((locals.var_q_d1_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn8)), ((locals.var_q_d1_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9,)
    }
};
        locals.var_q_d1_lnexpnum = assign19670_e19606;
        locals.var_q_d1_lnexpnum_dn4 = assign19670_e19606_d_n4;
        locals.var_q_d1_lnexpnum_dn6 = assign19670_e19606_d_n6;
        locals.var_q_d1_lnexpnum_dn7 = assign19670_e19606_d_n7;
        locals.var_q_d1_lnexpnum_dn8 = assign19670_e19606_d_n8;
        locals.var_q_d1_lnexpnum_dn9 = assign19670_e19606_d_n9;
        locals.var_q_d1_lnexpnum_rv = 0.0;

        let (assign19680_e19616, assign19680_e19616_d_n4, assign19680_e19616_d_n6, assign19680_e19616_d_n7, assign19680_e19616_d_n8, assign19680_e19616_d_n9,) = {
    if (locals.var_guard645 != 0.0) {
        let assign19680_e19610: f64 = (locals.var_q_d2_expnum * locals.var_q_temp1);
        let assign19680_e19613: f64 = (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum);
        let assign19680_e19614: f64 = (assign19680_e19610 - assign19680_e19613);
        (assign19680_e19614, (((locals.var_q_d2_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn4)) - ((locals.var_q_d1_lnexpnum_dn4 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn4))), (((locals.var_q_d2_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn6)) - ((locals.var_q_d1_lnexpnum_dn6 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn6))), (((locals.var_q_d2_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn7)) - ((locals.var_q_d1_lnexpnum_dn7 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn7))), (((locals.var_q_d2_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn8)) - ((locals.var_q_d1_lnexpnum_dn8 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn8))), (((locals.var_q_d2_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn9)) - ((locals.var_q_d1_lnexpnum_dn9 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn9))),)
    } else {
        (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9,)
    }
};
        locals.var_q_d2_lnexpnum = assign19680_e19616;
        locals.var_q_d2_lnexpnum_dn4 = assign19680_e19616_d_n4;
        locals.var_q_d2_lnexpnum_dn6 = assign19680_e19616_d_n6;
        locals.var_q_d2_lnexpnum_dn7 = assign19680_e19616_d_n7;
        locals.var_q_d2_lnexpnum_dn8 = assign19680_e19616_d_n8;
        locals.var_q_d2_lnexpnum_dn9 = assign19680_e19616_d_n9;
        locals.var_q_d2_lnexpnum_rv = 0.0;

        let (assign19690_e19627, assign19690_e19627_d_n4, assign19690_e19627_d_n6, assign19690_e19627_d_n7, assign19690_e19627_d_n8, assign19690_e19627_d_n9,) = {
    if (locals.var_guard645 == 0.0) {
        let assign19690_e19621: f64 = (locals.var_q_k1q1 + 0.6931471805599);
        let assign19690_e19623: f64 = (-locals.var_q_k1q1);
        let assign19690_e19624: f64 = (assign19690_e19623).ln();
        let assign19690_e19625: f64 = (assign19690_e19621 + assign19690_e19624);
        (assign19690_e19625, (locals.var_q_k1q1_dn4 + ((-locals.var_q_k1q1_dn4) / assign19690_e19623)), (locals.var_q_k1q1_dn6 + ((-locals.var_q_k1q1_dn6) / assign19690_e19623)), (locals.var_q_k1q1_dn7 + ((-locals.var_q_k1q1_dn7) / assign19690_e19623)), (locals.var_q_k1q1_dn8 + ((-locals.var_q_k1q1_dn8) / assign19690_e19623)), (locals.var_q_k1q1_dn9 + ((-locals.var_q_k1q1_dn9) / assign19690_e19623)),)
    } else {
        (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9,)
    }
};
        locals.var_q_lnexpnum = assign19690_e19627;
        locals.var_q_lnexpnum_dn4 = assign19690_e19627_d_n4;
        locals.var_q_lnexpnum_dn6 = assign19690_e19627_d_n6;
        locals.var_q_lnexpnum_dn7 = assign19690_e19627_d_n7;
        locals.var_q_lnexpnum_dn8 = assign19690_e19627_d_n8;
        locals.var_q_lnexpnum_dn9 = assign19690_e19627_d_n9;
        locals.var_q_lnexpnum_rv = 0.0;

        let (assign19700_e19634, assign19700_e19634_d_n4, assign19700_e19634_d_n6, assign19700_e19634_d_n7, assign19700_e19634_d_n8, assign19700_e19634_d_n9,) = {
    if (locals.var_guard645 == 0.0) {
        let assign19700_e19632: f64 = (1.0 / locals.var_q1d);
        (assign19700_e19632, (-(locals.var_q1d_dn4 / (locals.var_q1d * locals.var_q1d))), (-(locals.var_q1d_dn6 / (locals.var_q1d * locals.var_q1d))), (-(locals.var_q1d_dn7 / (locals.var_q1d * locals.var_q1d))), (-(locals.var_q1d_dn8 / (locals.var_q1d * locals.var_q1d))), (-(locals.var_q1d_dn9 / (locals.var_q1d * locals.var_q1d))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign19700_e19634;
        locals.var_q_temp1_dn4 = assign19700_e19634_d_n4;
        locals.var_q_temp1_dn6 = assign19700_e19634_d_n6;
        locals.var_q_temp1_dn7 = assign19700_e19634_d_n7;
        locals.var_q_temp1_dn8 = assign19700_e19634_d_n8;
        locals.var_q_temp1_dn9 = assign19700_e19634_d_n9;
        locals.var_q_temp1_rv = 0.0;

        let (assign19710_e19641, assign19710_e19641_d_n4, assign19710_e19641_d_n6, assign19710_e19641_d_n7, assign19710_e19641_d_n8, assign19710_e19641_d_n9,) = {
    if (locals.var_guard645 == 0.0) {
        let assign19710_e19639: f64 = (locals.var_k1 + locals.var_q_temp1);
        (assign19710_e19639, (locals.var_k1_dn4 + locals.var_q_temp1_dn4), (locals.var_k1_dn6 + locals.var_q_temp1_dn6), (locals.var_k1_dn7 + locals.var_q_temp1_dn7), (locals.var_k1_dn8 + locals.var_q_temp1_dn8), (locals.var_k1_dn9 + locals.var_q_temp1_dn9),)
    } else {
        (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9,)
    }
};
        locals.var_q_d1_lnexpnum = assign19710_e19641;
        locals.var_q_d1_lnexpnum_dn4 = assign19710_e19641_d_n4;
        locals.var_q_d1_lnexpnum_dn6 = assign19710_e19641_d_n6;
        locals.var_q_d1_lnexpnum_dn7 = assign19710_e19641_d_n7;
        locals.var_q_d1_lnexpnum_dn8 = assign19710_e19641_d_n8;
        locals.var_q_d1_lnexpnum_dn9 = assign19710_e19641_d_n9;
        locals.var_q_d1_lnexpnum_rv = 0.0;

        let (assign19720_e19649, assign19720_e19649_d_n4, assign19720_e19649_d_n6, assign19720_e19649_d_n7, assign19720_e19649_d_n8, assign19720_e19649_d_n9,) = {
    if (locals.var_guard645 == 0.0) {
        let assign19720_e19645: f64 = (-locals.var_q_temp1);
        let assign19720_e19647: f64 = (assign19720_e19645 * locals.var_q_temp1);
        (assign19720_e19647, (((-locals.var_q_temp1_dn4) * locals.var_q_temp1) + (assign19720_e19645 * locals.var_q_temp1_dn4)), (((-locals.var_q_temp1_dn6) * locals.var_q_temp1) + (assign19720_e19645 * locals.var_q_temp1_dn6)), (((-locals.var_q_temp1_dn7) * locals.var_q_temp1) + (assign19720_e19645 * locals.var_q_temp1_dn7)), (((-locals.var_q_temp1_dn8) * locals.var_q_temp1) + (assign19720_e19645 * locals.var_q_temp1_dn8)), (((-locals.var_q_temp1_dn9) * locals.var_q_temp1) + (assign19720_e19645 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9,)
    }
};
        locals.var_q_d2_lnexpnum = assign19720_e19649;
        locals.var_q_d2_lnexpnum_dn4 = assign19720_e19649_d_n4;
        locals.var_q_d2_lnexpnum_dn6 = assign19720_e19649_d_n6;
        locals.var_q_d2_lnexpnum_dn7 = assign19720_e19649_d_n7;
        locals.var_q_d2_lnexpnum_dn8 = assign19720_e19649_d_n8;
        locals.var_q_d2_lnexpnum_dn9 = assign19720_e19649_d_n9;
        locals.var_q_d2_lnexpnum_rv = 0.0;

        let assign19730_e19652: f64 = (locals.var_xg2x - locals.var_xg1x);
        let assign19730_e19654: f64 = (assign19730_e19652 + locals.var_q1d);
        let assign19730_e19657: f64 = (2.0 * locals.var_q_lnexpnum);
        let assign19730_e19658: f64 = (assign19730_e19654 + assign19730_e19657);
        let assign19730_e19660: f64 = (assign19730_e19658 - locals.var_q_ln_term);
        locals.var_q_q2_int = assign19730_e19660;
        locals.var_q_q2_int_dn4 = ((((locals.var_xg2x_dn4 - locals.var_xg1x_dn4) + locals.var_q1d_dn4) + (2.0 * locals.var_q_lnexpnum_dn4)) - locals.var_q_ln_term_dn4);
        locals.var_q_q2_int_dn6 = ((((locals.var_xg2x_dn6 - locals.var_xg1x_dn6) + locals.var_q1d_dn6) + (2.0 * locals.var_q_lnexpnum_dn6)) - locals.var_q_ln_term_dn6);
        locals.var_q_q2_int_dn7 = ((((locals.var_xg2x_dn7 - locals.var_xg1x_dn7) + locals.var_q1d_dn7) + (2.0 * locals.var_q_lnexpnum_dn7)) - locals.var_q_ln_term_dn7);
        locals.var_q_q2_int_dn8 = ((((locals.var_xg2x_dn8 - locals.var_xg1x_dn8) + locals.var_q1d_dn8) + (2.0 * locals.var_q_lnexpnum_dn8)) - locals.var_q_ln_term_dn8);
        locals.var_q_q2_int_dn9 = ((((locals.var_xg2x_dn9 - locals.var_xg1x_dn9) + locals.var_q1d_dn9) + (2.0 * locals.var_q_lnexpnum_dn9)) - locals.var_q_ln_term_dn9);
        locals.var_q_q2_int_rv = 0.0;

        let assign19740_e19664: f64 = (2.0 * locals.var_q_d1_lnexpnum);
        let assign19740_e19665: f64 = (1.0 + assign19740_e19664);
        let assign19740_e19667: f64 = (assign19740_e19665 - locals.var_q_d1_ln);
        locals.var_q_d1_q2 = assign19740_e19667;
        locals.var_q_d1_q2_dn4 = ((2.0 * locals.var_q_d1_lnexpnum_dn4) - locals.var_q_d1_ln_dn4);
        locals.var_q_d1_q2_dn6 = ((2.0 * locals.var_q_d1_lnexpnum_dn6) - locals.var_q_d1_ln_dn6);
        locals.var_q_d1_q2_dn7 = ((2.0 * locals.var_q_d1_lnexpnum_dn7) - locals.var_q_d1_ln_dn7);
        locals.var_q_d1_q2_dn8 = ((2.0 * locals.var_q_d1_lnexpnum_dn8) - locals.var_q_d1_ln_dn8);
        locals.var_q_d1_q2_dn9 = ((2.0 * locals.var_q_d1_lnexpnum_dn9) - locals.var_q_d1_ln_dn9);
        locals.var_q_d1_q2_rv = 0.0;

        let assign19750_e19670: f64 = (2.0 * locals.var_q_d2_lnexpnum);
        let assign19750_e19672: f64 = (assign19750_e19670 - locals.var_q_d2_ln);
        locals.var_q_d2_q2 = assign19750_e19672;
        locals.var_q_d2_q2_dn4 = ((2.0 * locals.var_q_d2_lnexpnum_dn4) - locals.var_q_d2_ln_dn4);
        locals.var_q_d2_q2_dn6 = ((2.0 * locals.var_q_d2_lnexpnum_dn6) - locals.var_q_d2_ln_dn6);
        locals.var_q_d2_q2_dn7 = ((2.0 * locals.var_q_d2_lnexpnum_dn7) - locals.var_q_d2_ln_dn7);
        locals.var_q_d2_q2_dn8 = ((2.0 * locals.var_q_d2_lnexpnum_dn8) - locals.var_q_d2_ln_dn8);
        locals.var_q_d2_q2_dn9 = ((2.0 * locals.var_q_d2_lnexpnum_dn9) - locals.var_q_d2_ln_dn9);
        locals.var_q_d2_q2_rv = 0.0;

        let assign19760_e19676: f64 = (locals.var_k2 * locals.var_q_q2_int);
        let assign19760_e19677: f64 = (locals.var_q_k1q1 + assign19760_e19676);
        locals.var_q_qi_int = assign19760_e19677;
        locals.var_q_qi_int_dn4 = (locals.var_q_k1q1_dn4 + ((locals.var_k2_dn4 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn4)));
        locals.var_q_qi_int_dn6 = (locals.var_q_k1q1_dn6 + ((locals.var_k2_dn6 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn6)));
        locals.var_q_qi_int_dn7 = (locals.var_q_k1q1_dn7 + ((locals.var_k2_dn7 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn7)));
        locals.var_q_qi_int_dn8 = (locals.var_q_k1q1_dn8 + ((locals.var_k2_dn8 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn8)));
        locals.var_q_qi_int_dn9 = (locals.var_q_k1q1_dn9 + ((locals.var_k2_dn9 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn9)));
        locals.var_q_qi_int_rv = 0.0;

        let assign19770_e19681: f64 = (locals.var_k2 * locals.var_q_d1_q2);
        let assign19770_e19682: f64 = (locals.var_k1 + assign19770_e19681);
        locals.var_q_d1_qi = assign19770_e19682;
        locals.var_q_d1_qi_dn4 = (locals.var_k1_dn4 + ((locals.var_k2_dn4 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn4)));
        locals.var_q_d1_qi_dn6 = (locals.var_k1_dn6 + ((locals.var_k2_dn6 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn6)));
        locals.var_q_d1_qi_dn7 = (locals.var_k1_dn7 + ((locals.var_k2_dn7 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn7)));
        locals.var_q_d1_qi_dn8 = (locals.var_k1_dn8 + ((locals.var_k2_dn8 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn8)));
        locals.var_q_d1_qi_dn9 = (locals.var_k1_dn9 + ((locals.var_k2_dn9 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn9)));
        locals.var_q_d1_qi_rv = 0.0;

        let assign19780_e19685: f64 = (locals.var_k2 * locals.var_q_d2_q2);
        locals.var_q_d2_qi = assign19780_e19685;
        locals.var_q_d2_qi_dn4 = ((locals.var_k2_dn4 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn4));
        locals.var_q_d2_qi_dn6 = ((locals.var_k2_dn6 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn6));
        locals.var_q_d2_qi_dn7 = ((locals.var_k2_dn7 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn7));
        locals.var_q_d2_qi_dn8 = ((locals.var_k2_dn8 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn8));
        locals.var_q_d2_qi_dn9 = ((locals.var_k2_dn9 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn9));
        locals.var_q_d2_qi_rv = 0.0;

        let assign19790_e19688: f64 = (locals.var_q_qi_int * locals.var_q_expnum);
        let assign19790_e19690: f64 = (assign19790_e19688 - locals.var_q_aexp);
        locals.var_q_zero = assign19790_e19690;
        locals.var_q_zero_dn4 = (((locals.var_q_qi_int_dn4 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4);
        locals.var_q_zero_dn6 = (((locals.var_q_qi_int_dn6 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6);
        locals.var_q_zero_dn7 = (((locals.var_q_qi_int_dn7 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7);
        locals.var_q_zero_dn8 = (((locals.var_q_qi_int_dn8 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8);
        locals.var_q_zero_dn9 = (((locals.var_q_qi_int_dn9 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9);
        locals.var_q_zero_rv = 0.0;

        let assign19800_e19693: f64 = (locals.var_q_d1_qi * locals.var_q_expnum);
        let assign19800_e19696: f64 = (locals.var_q_qi_int * locals.var_q_d1_expnum);
        let assign19800_e19697: f64 = (assign19800_e19693 + assign19800_e19696);
        let assign19800_e19699: f64 = (assign19800_e19697 + locals.var_q_aexp);
        locals.var_q_d1_zero = assign19800_e19699;
        locals.var_q_d1_zero_dn4 = ((((locals.var_q_d1_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn4)) + ((locals.var_q_qi_int_dn4 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4);
        locals.var_q_d1_zero_dn6 = ((((locals.var_q_d1_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn6)) + ((locals.var_q_qi_int_dn6 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6);
        locals.var_q_d1_zero_dn7 = ((((locals.var_q_d1_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn7)) + ((locals.var_q_qi_int_dn7 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7);
        locals.var_q_d1_zero_dn8 = ((((locals.var_q_d1_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn8)) + ((locals.var_q_qi_int_dn8 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8);
        locals.var_q_d1_zero_dn9 = ((((locals.var_q_d1_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn9)) + ((locals.var_q_qi_int_dn9 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9);
        locals.var_q_d1_zero_rv = 0.0;

        let assign19810_e19702: f64 = (locals.var_q_d2_qi * locals.var_q_expnum);
        let assign19810_e19705: f64 = (2.0 * locals.var_q_d1_qi);
        let assign19810_e19707: f64 = (assign19810_e19705 * locals.var_q_d1_expnum);
        let assign19810_e19708: f64 = (assign19810_e19702 + assign19810_e19707);
        let assign19810_e19711: f64 = (locals.var_q_qi_int * locals.var_q_d2_expnum);
        let assign19810_e19712: f64 = (assign19810_e19708 + assign19810_e19711);
        let assign19810_e19714: f64 = (assign19810_e19712 - locals.var_q_aexp);
        locals.var_q_d2_zero = assign19810_e19714;
        locals.var_q_d2_zero_dn4 = (((((locals.var_q_d2_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_d1_qi_dn4) * locals.var_q_d1_expnum) + (assign19810_e19705 * locals.var_q_d1_expnum_dn4))) + ((locals.var_q_qi_int_dn4 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn4))) - locals.var_q_aexp_dn4);
        locals.var_q_d2_zero_dn6 = (((((locals.var_q_d2_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_d1_qi_dn6) * locals.var_q_d1_expnum) + (assign19810_e19705 * locals.var_q_d1_expnum_dn6))) + ((locals.var_q_qi_int_dn6 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn6))) - locals.var_q_aexp_dn6);
        locals.var_q_d2_zero_dn7 = (((((locals.var_q_d2_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_d1_qi_dn7) * locals.var_q_d1_expnum) + (assign19810_e19705 * locals.var_q_d1_expnum_dn7))) + ((locals.var_q_qi_int_dn7 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn7))) - locals.var_q_aexp_dn7);
        locals.var_q_d2_zero_dn8 = (((((locals.var_q_d2_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_d1_qi_dn8) * locals.var_q_d1_expnum) + (assign19810_e19705 * locals.var_q_d1_expnum_dn8))) + ((locals.var_q_qi_int_dn8 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn8))) - locals.var_q_aexp_dn8);
        locals.var_q_d2_zero_dn9 = (((((locals.var_q_d2_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_d1_qi_dn9) * locals.var_q_d1_expnum) + (assign19810_e19705 * locals.var_q_d1_expnum_dn9))) + ((locals.var_q_qi_int_dn9 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn9))) - locals.var_q_aexp_dn9);
        locals.var_q_d2_zero_rv = 0.0;

        let assign19820_e19717: f64 = (locals.var_q_d1_zero * locals.var_q_d1_zero);
        let assign19820_e19720: f64 = (0.5 * locals.var_q_zero);
        let assign19820_e19722: f64 = (assign19820_e19720 * locals.var_q_d2_zero);
        let assign19820_e19723: f64 = (assign19820_e19717 - assign19820_e19722);
        locals.var_q_temp = assign19820_e19723;
        locals.var_q_temp_dn4 = (((locals.var_q_d1_zero_dn4 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn4)) - (((0.5 * locals.var_q_zero_dn4) * locals.var_q_d2_zero) + (assign19820_e19720 * locals.var_q_d2_zero_dn4)));
        locals.var_q_temp_dn6 = (((locals.var_q_d1_zero_dn6 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn6)) - (((0.5 * locals.var_q_zero_dn6) * locals.var_q_d2_zero) + (assign19820_e19720 * locals.var_q_d2_zero_dn6)));
        locals.var_q_temp_dn7 = (((locals.var_q_d1_zero_dn7 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn7)) - (((0.5 * locals.var_q_zero_dn7) * locals.var_q_d2_zero) + (assign19820_e19720 * locals.var_q_d2_zero_dn7)));
        locals.var_q_temp_dn8 = (((locals.var_q_d1_zero_dn8 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn8)) - (((0.5 * locals.var_q_zero_dn8) * locals.var_q_d2_zero) + (assign19820_e19720 * locals.var_q_d2_zero_dn8)));
        locals.var_q_temp_dn9 = (((locals.var_q_d1_zero_dn9 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn9)) - (((0.5 * locals.var_q_zero_dn9) * locals.var_q_d2_zero) + (assign19820_e19720 * locals.var_q_d2_zero_dn9)));
        locals.var_q_temp_rv = 0.0;

        let assign19830_e19725: f64 = (-locals.var_q_zero);
        let assign19830_e19727: f64 = (assign19830_e19725 * locals.var_q_d1_zero);
        let assign19830_e19729: f64 = (assign19830_e19727 * locals.var_q_temp);
        let assign19830_e19732: f64 = (locals.var_q_temp * locals.var_q_temp);
        let assign19830_e19734: f64 = (assign19830_e19732 + 1e-200);
        let assign19830_e19735: f64 = (assign19830_e19729 / assign19830_e19734);
        locals.var_q_eps2 = assign19830_e19735;
        locals.var_q_eps2_dn4 = ((((((((-locals.var_q_zero_dn4) * locals.var_q_d1_zero) + (assign19830_e19725 * locals.var_q_d1_zero_dn4)) * locals.var_q_temp) + (assign19830_e19727 * locals.var_q_temp_dn4)) * assign19830_e19734) - (assign19830_e19729 * ((locals.var_q_temp_dn4 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn4)))) / (assign19830_e19734 * assign19830_e19734));
        locals.var_q_eps2_dn6 = ((((((((-locals.var_q_zero_dn6) * locals.var_q_d1_zero) + (assign19830_e19725 * locals.var_q_d1_zero_dn6)) * locals.var_q_temp) + (assign19830_e19727 * locals.var_q_temp_dn6)) * assign19830_e19734) - (assign19830_e19729 * ((locals.var_q_temp_dn6 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn6)))) / (assign19830_e19734 * assign19830_e19734));
        locals.var_q_eps2_dn7 = ((((((((-locals.var_q_zero_dn7) * locals.var_q_d1_zero) + (assign19830_e19725 * locals.var_q_d1_zero_dn7)) * locals.var_q_temp) + (assign19830_e19727 * locals.var_q_temp_dn7)) * assign19830_e19734) - (assign19830_e19729 * ((locals.var_q_temp_dn7 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn7)))) / (assign19830_e19734 * assign19830_e19734));
        locals.var_q_eps2_dn8 = ((((((((-locals.var_q_zero_dn8) * locals.var_q_d1_zero) + (assign19830_e19725 * locals.var_q_d1_zero_dn8)) * locals.var_q_temp) + (assign19830_e19727 * locals.var_q_temp_dn8)) * assign19830_e19734) - (assign19830_e19729 * ((locals.var_q_temp_dn8 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn8)))) / (assign19830_e19734 * assign19830_e19734));
        locals.var_q_eps2_dn9 = ((((((((-locals.var_q_zero_dn9) * locals.var_q_d1_zero) + (assign19830_e19725 * locals.var_q_d1_zero_dn9)) * locals.var_q_temp) + (assign19830_e19727 * locals.var_q_temp_dn9)) * assign19830_e19734) - (assign19830_e19729 * ((locals.var_q_temp_dn9 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn9)))) / (assign19830_e19734 * assign19830_e19734));
        locals.var_q_eps2_rv = 0.0;

        let assign19840_e19738: f64 = (locals.var_q1d + locals.var_q_eps2);
        locals.var_q1d = assign19840_e19738;
        locals.var_q1d_dn4 = (locals.var_q1d_dn4 + locals.var_q_eps2_dn4);
        locals.var_q1d_dn6 = (locals.var_q1d_dn6 + locals.var_q_eps2_dn6);
        locals.var_q1d_dn7 = (locals.var_q1d_dn7 + locals.var_q_eps2_dn7);
        locals.var_q1d_dn8 = (locals.var_q1d_dn8 + locals.var_q_eps2_dn8);
        locals.var_q1d_dn9 = (locals.var_q1d_dn9 + locals.var_q_eps2_dn9);
        locals.var_q1d_rv = 0.0;

        let assign19850_e19741: f64 = if p.p10 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard646 = assign19850_e19741;
        locals.var_guard646_rv = 0.0;

        let assign19860_e19743: f64 = (locals.var_q_eps2).abs();
        let assign19860_e19745: f64 = if assign19860_e19743 > 0.01 { 1.0 } else { 0.0 };
        locals.var_guard647 = assign19860_e19745;
        locals.var_guard647_rv = 0.0;

        let (assign19870_e19753, assign19870_e19753_d_n4, assign19870_e19753_d_n6, assign19870_e19753_d_n7, assign19870_e19753_d_n8, assign19870_e19753_d_n9,) = {
    if ((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) {
        let assign19870_e19751: f64 = (locals.var_k1 * locals.var_q1d);
        (assign19870_e19751, ((locals.var_k1_dn4 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn4)), ((locals.var_k1_dn6 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn6)), ((locals.var_k1_dn7 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn7)), ((locals.var_k1_dn8 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn8)), ((locals.var_k1_dn9 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn9)),)
    } else {
        (locals.var_q_k1q1, locals.var_q_k1q1_dn4, locals.var_q_k1q1_dn6, locals.var_q_k1q1_dn7, locals.var_q_k1q1_dn8, locals.var_q_k1q1_dn9,)
    }
};
        locals.var_q_k1q1 = assign19870_e19753;
        locals.var_q_k1q1_dn4 = assign19870_e19753_d_n4;
        locals.var_q_k1q1_dn6 = assign19870_e19753_d_n6;
        locals.var_q_k1q1_dn7 = assign19870_e19753_d_n7;
        locals.var_q_k1q1_dn8 = assign19870_e19753_d_n8;
        locals.var_q_k1q1_dn9 = assign19870_e19753_d_n9;
        locals.var_q_k1q1_rv = 0.0;

        let assign19880_e19756: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign19880_e19758: f64 = (assign19880_e19756 - locals.var_xdeff);
        let assign19880_e19760: f64 = if assign19880_e19758 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard648 = assign19880_e19760;
        locals.var_guard648_rv = 0.0;

        let (assign19890_e19773, assign19890_e19773_d_n4, assign19890_e19773_d_n6, assign19890_e19773_d_n7, assign19890_e19773_d_n8, assign19890_e19773_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard648 != 0.0)) {
        let assign19890_e19768: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign19890_e19770: f64 = (assign19890_e19768 - locals.var_xdeff);
        let assign19890_e19771: f64 = (assign19890_e19770).exp();
        (assign19890_e19771, (assign19890_e19771 * ((locals.var_xg1x_dn4 - locals.var_q1d_dn4) - locals.var_xdeff_dn4)), (assign19890_e19771 * ((locals.var_xg1x_dn6 - locals.var_q1d_dn6) - locals.var_xdeff_dn6)), (assign19890_e19771 * ((locals.var_xg1x_dn7 - locals.var_q1d_dn7) - locals.var_xdeff_dn7)), (assign19890_e19771 * ((locals.var_xg1x_dn8 - locals.var_q1d_dn8) - locals.var_xdeff_dn8)), (assign19890_e19771 * ((locals.var_xg1x_dn9 - locals.var_q1d_dn9) - locals.var_xdeff_dn9)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign19890_e19773;
        locals.var_q_temp1_dn4 = assign19890_e19773_d_n4;
        locals.var_q_temp1_dn6 = assign19890_e19773_d_n6;
        locals.var_q_temp1_dn7 = assign19890_e19773_d_n7;
        locals.var_q_temp1_dn8 = assign19890_e19773_d_n8;
        locals.var_q_temp1_dn9 = assign19890_e19773_d_n9;
        locals.var_q_temp1_rv = 0.0;

        let (assign19900_e19816, assign19900_e19816_d_n4, assign19900_e19816_d_n6, assign19900_e19816_d_n7, assign19900_e19816_d_n8, assign19900_e19816_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard648 == 0.0)) {
        let assign19900_e19784: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign19900_e19786: f64 = (assign19900_e19784 - locals.var_xdeff);
        let assign19900_e19788: f64 = (assign19900_e19786 - 80.0);
        let assign19900_e19793: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign19900_e19795: f64 = (assign19900_e19793 - locals.var_xdeff);
        let assign19900_e19797: f64 = (assign19900_e19795 - 80.0);
        let assign19900_e19798: f64 = (0.5 * assign19900_e19797);
        let assign19900_e19802: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign19900_e19804: f64 = (assign19900_e19802 - locals.var_xdeff);
        let assign19900_e19806: f64 = (assign19900_e19804 - 80.0);
        let assign19900_e19808: f64 = (assign19900_e19806 * 0.3333333333333);
        let assign19900_e19809: f64 = (1.0 + assign19900_e19808);
        let assign19900_e19810: f64 = (assign19900_e19798 * assign19900_e19809);
        let assign19900_e19811: f64 = (1.0 + assign19900_e19810);
        let assign19900_e19812: f64 = (assign19900_e19788 * assign19900_e19811);
        let assign19900_e19813: f64 = (1.0 + assign19900_e19812);
        let assign19900_e19814: f64 = (5.54062e34 * assign19900_e19813);
        (assign19900_e19814, (5.54062e34 * ((((locals.var_xg1x_dn4 - locals.var_q1d_dn4) - locals.var_xdeff_dn4) * assign19900_e19811) + (assign19900_e19788 * (((0.5 * ((locals.var_xg1x_dn4 - locals.var_q1d_dn4) - locals.var_xdeff_dn4)) * assign19900_e19809) + (assign19900_e19798 * (((locals.var_xg1x_dn4 - locals.var_q1d_dn4) - locals.var_xdeff_dn4) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x_dn6 - locals.var_q1d_dn6) - locals.var_xdeff_dn6) * assign19900_e19811) + (assign19900_e19788 * (((0.5 * ((locals.var_xg1x_dn6 - locals.var_q1d_dn6) - locals.var_xdeff_dn6)) * assign19900_e19809) + (assign19900_e19798 * (((locals.var_xg1x_dn6 - locals.var_q1d_dn6) - locals.var_xdeff_dn6) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x_dn7 - locals.var_q1d_dn7) - locals.var_xdeff_dn7) * assign19900_e19811) + (assign19900_e19788 * (((0.5 * ((locals.var_xg1x_dn7 - locals.var_q1d_dn7) - locals.var_xdeff_dn7)) * assign19900_e19809) + (assign19900_e19798 * (((locals.var_xg1x_dn7 - locals.var_q1d_dn7) - locals.var_xdeff_dn7) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x_dn8 - locals.var_q1d_dn8) - locals.var_xdeff_dn8) * assign19900_e19811) + (assign19900_e19788 * (((0.5 * ((locals.var_xg1x_dn8 - locals.var_q1d_dn8) - locals.var_xdeff_dn8)) * assign19900_e19809) + (assign19900_e19798 * (((locals.var_xg1x_dn8 - locals.var_q1d_dn8) - locals.var_xdeff_dn8) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x_dn9 - locals.var_q1d_dn9) - locals.var_xdeff_dn9) * assign19900_e19811) + (assign19900_e19788 * (((0.5 * ((locals.var_xg1x_dn9 - locals.var_q1d_dn9) - locals.var_xdeff_dn9)) * assign19900_e19809) + (assign19900_e19798 * (((locals.var_xg1x_dn9 - locals.var_q1d_dn9) - locals.var_xdeff_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign19900_e19816;
        locals.var_q_temp1_dn4 = assign19900_e19816_d_n4;
        locals.var_q_temp1_dn6 = assign19900_e19816_d_n6;
        locals.var_q_temp1_dn7 = assign19900_e19816_d_n7;
        locals.var_q_temp1_dn8 = assign19900_e19816_d_n8;
        locals.var_q_temp1_dn9 = assign19900_e19816_d_n9;
        locals.var_q_temp1_rv = 0.0;

        let (assign19910_e19824, assign19910_e19824_d_n4, assign19910_e19824_d_n6, assign19910_e19824_d_n7, assign19910_e19824_d_n8, assign19910_e19824_d_n9,) = {
    if ((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) {
        let assign19910_e19822: f64 = (locals.var_a0 * locals.var_q_temp1);
        (assign19910_e19822, ((locals.var_a0_dn4 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn4)), ((locals.var_a0_dn6 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn6)), ((locals.var_a0_dn7 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn7)), ((locals.var_a0_dn8 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn8)), ((locals.var_a0_dn9 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_aexp, locals.var_q_aexp_dn4, locals.var_q_aexp_dn6, locals.var_q_aexp_dn7, locals.var_q_aexp_dn8, locals.var_q_aexp_dn9,)
    }
};
        locals.var_q_aexp = assign19910_e19824;
        locals.var_q_aexp_dn4 = assign19910_e19824_d_n4;
        locals.var_q_aexp_dn6 = assign19910_e19824_d_n6;
        locals.var_q_aexp_dn7 = assign19910_e19824_d_n7;
        locals.var_q_aexp_dn8 = assign19910_e19824_d_n8;
        locals.var_q_aexp_dn9 = assign19910_e19824_d_n9;
        locals.var_q_aexp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_54(
        locals: &mut StampLocals,
    ) {
        let (assign19920_e19834, assign19920_e19834_d_n4, assign19920_e19834_d_n6, assign19920_e19834_d_n7, assign19920_e19834_d_n8, assign19920_e19834_d_n9,) = {
    if ((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) {
        let assign19920_e19830: f64 = (locals.var_q_k1q1 * locals.var_q_k1q1);
        let assign19920_e19832: f64 = (assign19920_e19830 - locals.var_q_aexp);
        (assign19920_e19832, (((locals.var_q_k1q1_dn4 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn4)) - locals.var_q_aexp_dn4), (((locals.var_q_k1q1_dn6 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn6)) - locals.var_q_aexp_dn6), (((locals.var_q_k1q1_dn7 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn7)) - locals.var_q_aexp_dn7), (((locals.var_q_k1q1_dn8 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn8)) - locals.var_q_aexp_dn8), (((locals.var_q_k1q1_dn9 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn9)) - locals.var_q_aexp_dn9),)
    } else {
        (locals.var_q_qsq, locals.var_q_qsq_dn4, locals.var_q_qsq_dn6, locals.var_q_qsq_dn7, locals.var_q_qsq_dn8, locals.var_q_qsq_dn9,)
    }
};
        locals.var_q_qsq = assign19920_e19834;
        locals.var_q_qsq_dn4 = assign19920_e19834_d_n4;
        locals.var_q_qsq_dn6 = assign19920_e19834_d_n6;
        locals.var_q_qsq_dn7 = assign19920_e19834_d_n7;
        locals.var_q_qsq_dn8 = assign19920_e19834_d_n8;
        locals.var_q_qsq_dn9 = assign19920_e19834_d_n9;
        locals.var_q_qsq_rv = 0.0;

        let (assign19930_e19846, assign19930_e19846_d_n4, assign19930_e19846_d_n6, assign19930_e19846_d_n7, assign19930_e19846_d_n8, assign19930_e19846_d_n9,) = {
    if ((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) {
        let assign19930_e19840: f64 = (2.0 * locals.var_k1);
        let assign19930_e19842: f64 = (assign19930_e19840 * locals.var_q_k1q1);
        let assign19930_e19844: f64 = (assign19930_e19842 + locals.var_q_aexp);
        (assign19930_e19844, ((((2.0 * locals.var_k1_dn4) * locals.var_q_k1q1) + (assign19930_e19840 * locals.var_q_k1q1_dn4)) + locals.var_q_aexp_dn4), ((((2.0 * locals.var_k1_dn6) * locals.var_q_k1q1) + (assign19930_e19840 * locals.var_q_k1q1_dn6)) + locals.var_q_aexp_dn6), ((((2.0 * locals.var_k1_dn7) * locals.var_q_k1q1) + (assign19930_e19840 * locals.var_q_k1q1_dn7)) + locals.var_q_aexp_dn7), ((((2.0 * locals.var_k1_dn8) * locals.var_q_k1q1) + (assign19930_e19840 * locals.var_q_k1q1_dn8)) + locals.var_q_aexp_dn8), ((((2.0 * locals.var_k1_dn9) * locals.var_q_k1q1) + (assign19930_e19840 * locals.var_q_k1q1_dn9)) + locals.var_q_aexp_dn9),)
    } else {
        (locals.var_q_d1_qsq, locals.var_q_d1_qsq_dn4, locals.var_q_d1_qsq_dn6, locals.var_q_d1_qsq_dn7, locals.var_q_d1_qsq_dn8, locals.var_q_d1_qsq_dn9,)
    }
};
        locals.var_q_d1_qsq = assign19930_e19846;
        locals.var_q_d1_qsq_dn4 = assign19930_e19846_d_n4;
        locals.var_q_d1_qsq_dn6 = assign19930_e19846_d_n6;
        locals.var_q_d1_qsq_dn7 = assign19930_e19846_d_n7;
        locals.var_q_d1_qsq_dn8 = assign19930_e19846_d_n8;
        locals.var_q_d1_qsq_dn9 = assign19930_e19846_d_n9;
        locals.var_q_d1_qsq_rv = 0.0;

        let (assign19940_e19858, assign19940_e19858_d_n4, assign19940_e19858_d_n6, assign19940_e19858_d_n7, assign19940_e19858_d_n8, assign19940_e19858_d_n9,) = {
    if ((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) {
        let assign19940_e19852: f64 = (2.0 * locals.var_k1);
        let assign19940_e19854: f64 = (assign19940_e19852 * locals.var_k1);
        let assign19940_e19856: f64 = (assign19940_e19854 - locals.var_q_aexp);
        (assign19940_e19856, ((((2.0 * locals.var_k1_dn4) * locals.var_k1) + (assign19940_e19852 * locals.var_k1_dn4)) - locals.var_q_aexp_dn4), ((((2.0 * locals.var_k1_dn6) * locals.var_k1) + (assign19940_e19852 * locals.var_k1_dn6)) - locals.var_q_aexp_dn6), ((((2.0 * locals.var_k1_dn7) * locals.var_k1) + (assign19940_e19852 * locals.var_k1_dn7)) - locals.var_q_aexp_dn7), ((((2.0 * locals.var_k1_dn8) * locals.var_k1) + (assign19940_e19852 * locals.var_k1_dn8)) - locals.var_q_aexp_dn8), ((((2.0 * locals.var_k1_dn9) * locals.var_k1) + (assign19940_e19852 * locals.var_k1_dn9)) - locals.var_q_aexp_dn9),)
    } else {
        (locals.var_q_d2_qsq, locals.var_q_d2_qsq_dn4, locals.var_q_d2_qsq_dn6, locals.var_q_d2_qsq_dn7, locals.var_q_d2_qsq_dn8, locals.var_q_d2_qsq_dn9,)
    }
};
        locals.var_q_d2_qsq = assign19940_e19858;
        locals.var_q_d2_qsq_dn4 = assign19940_e19858_d_n4;
        locals.var_q_d2_qsq_dn6 = assign19940_e19858_d_n6;
        locals.var_q_d2_qsq_dn7 = assign19940_e19858_d_n7;
        locals.var_q_d2_qsq_dn8 = assign19940_e19858_d_n8;
        locals.var_q_d2_qsq_dn9 = assign19940_e19858_d_n9;
        locals.var_q_d2_qsq_rv = 0.0;

        let assign19950_e19861: f64 = (-0.005);
        let assign19950_e19862: f64 = if locals.var_q_qsq < assign19950_e19861 { 1.0 } else { 0.0 };
        locals.var_guard649 = assign19950_e19862;
        locals.var_guard649_rv = 0.0;

        let (assign19960_e19872, assign19960_e19872_d_n4, assign19960_e19872_d_n6, assign19960_e19872_d_n7, assign19960_e19872_d_n8, assign19960_e19872_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign19960_e19869: f64 = (locals.var_q_qsq).abs();
        let assign19960_e19870: f64 = (assign19960_e19869).sqrt();
        (assign19960_e19870, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign19960_e19870)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign19960_e19870)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign19960_e19870)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign19960_e19870)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign19960_e19870)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign19960_e19872;
        locals.var_q_rac_qsq_dn4 = assign19960_e19872_d_n4;
        locals.var_q_rac_qsq_dn6 = assign19960_e19872_d_n6;
        locals.var_q_rac_qsq_dn7 = assign19960_e19872_d_n7;
        locals.var_q_rac_qsq_dn8 = assign19960_e19872_d_n8;
        locals.var_q_rac_qsq_dn9 = assign19960_e19872_d_n9;
        locals.var_q_rac_qsq_rv = 0.0;

        let (assign19970_e19885, assign19970_e19885_d_n4, assign19970_e19885_d_n6, assign19970_e19885_d_n7, assign19970_e19885_d_n8, assign19970_e19885_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign19970_e19881: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign19970_e19882: f64 = (assign19970_e19881).tan();
        let assign19970_e19883: f64 = (locals.var_q_rac_qsq / assign19970_e19882);
        (assign19970_e19883, (((locals.var_q_rac_qsq_dn4 * assign19970_e19882) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn4) / ((assign19970_e19881).cos() * (assign19970_e19881).cos())))) / (assign19970_e19882 * assign19970_e19882)), (((locals.var_q_rac_qsq_dn6 * assign19970_e19882) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn6) / ((assign19970_e19881).cos() * (assign19970_e19881).cos())))) / (assign19970_e19882 * assign19970_e19882)), (((locals.var_q_rac_qsq_dn7 * assign19970_e19882) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn7) / ((assign19970_e19881).cos() * (assign19970_e19881).cos())))) / (assign19970_e19882 * assign19970_e19882)), (((locals.var_q_rac_qsq_dn8 * assign19970_e19882) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn8) / ((assign19970_e19881).cos() * (assign19970_e19881).cos())))) / (assign19970_e19882 * assign19970_e19882)), (((locals.var_q_rac_qsq_dn9 * assign19970_e19882) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn9) / ((assign19970_e19881).cos() * (assign19970_e19881).cos())))) / (assign19970_e19882 * assign19970_e19882)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign19970_e19885;
        locals.var_q_qcoth_dn4 = assign19970_e19885_d_n4;
        locals.var_q_qcoth_dn6 = assign19970_e19885_d_n6;
        locals.var_q_qcoth_dn7 = assign19970_e19885_d_n7;
        locals.var_q_qcoth_dn8 = assign19970_e19885_d_n8;
        locals.var_q_qcoth_dn9 = assign19970_e19885_d_n9;
        locals.var_q_qcoth_rv = 0.0;

        let (assign19980_e19897, assign19980_e19897_d_n4, assign19980_e19897_d_n6, assign19980_e19897_d_n7, assign19980_e19897_d_n8, assign19980_e19897_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign19980_e19893: f64 = (0.25 * locals.var_q_d1_qsq);
        let assign19980_e19895: f64 = (assign19980_e19893 / locals.var_q_qsq);
        (assign19980_e19895, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign19980_e19893 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign19980_e19893 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign19980_e19893 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign19980_e19893 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign19980_e19893 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign19980_e19897;
        locals.var_q_temp1_dn4 = assign19980_e19897_d_n4;
        locals.var_q_temp1_dn6 = assign19980_e19897_d_n6;
        locals.var_q_temp1_dn7 = assign19980_e19897_d_n7;
        locals.var_q_temp1_dn8 = assign19980_e19897_d_n8;
        locals.var_q_temp1_dn9 = assign19980_e19897_d_n9;
        locals.var_q_temp1_rv = 0.0;

        let (assign19990_e19913, assign19990_e19913_d_n4, assign19990_e19913_d_n6, assign19990_e19913_d_n7, assign19990_e19913_d_n8, assign19990_e19913_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign19990_e19907: f64 = (2.0 - locals.var_q_qcoth);
        let assign19990_e19908: f64 = (locals.var_q_qcoth * assign19990_e19907);
        let assign19990_e19909: f64 = (locals.var_q_qsq + assign19990_e19908);
        let assign19990_e19911: f64 = (assign19990_e19909 * locals.var_q_temp1);
        (assign19990_e19911, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign19990_e19907) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign19990_e19909 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign19990_e19907) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign19990_e19909 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign19990_e19907) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign19990_e19909 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign19990_e19907) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign19990_e19909 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign19990_e19907) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign19990_e19909 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign19990_e19913;
        locals.var_q_d1_qcoth_dn4 = assign19990_e19913_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign19990_e19913_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign19990_e19913_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign19990_e19913_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign19990_e19913_d_n9;
        locals.var_q_d1_qcoth_rv = 0.0;

        let (assign20000_e19937, assign20000_e19937_d_n4, assign20000_e19937_d_n6, assign20000_e19937_d_n7, assign20000_e19937_d_n8, assign20000_e19937_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign20000_e19922: f64 = (2.0 * locals.var_q_d1_qcoth);
        let assign20000_e19925: f64 = (1.0 + locals.var_q_qcoth);
        let assign20000_e19926: f64 = (assign20000_e19922 * assign20000_e19925);
        let assign20000_e19927: f64 = (locals.var_q_d1_qsq - assign20000_e19926);
        let assign20000_e19929: f64 = (assign20000_e19927 * locals.var_q_temp1);
        let assign20000_e19932: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
        let assign20000_e19934: f64 = (assign20000_e19932 / locals.var_q_d1_qsq);
        let assign20000_e19935: f64 = (assign20000_e19929 + assign20000_e19934);
        (assign20000_e19935, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign20000_e19925) + (assign20000_e19922 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign20000_e19927 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign20000_e19932 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign20000_e19925) + (assign20000_e19922 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign20000_e19927 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign20000_e19932 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign20000_e19925) + (assign20000_e19922 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign20000_e19927 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign20000_e19932 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign20000_e19925) + (assign20000_e19922 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign20000_e19927 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign20000_e19932 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign20000_e19925) + (assign20000_e19922 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign20000_e19927 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign20000_e19932 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign20000_e19937;
        locals.var_q_d2_qcoth_dn4 = assign20000_e19937_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign20000_e19937_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign20000_e19937_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign20000_e19937_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign20000_e19937_d_n9;
        locals.var_q_d2_qcoth_rv = 0.0;

        let (assign20010_e19949, assign20010_e19949_d_n4, assign20010_e19949_d_n6, assign20010_e19949_d_n7, assign20010_e19949_d_n8, assign20010_e19949_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign20010_e19946: f64 = (0.5 * locals.var_q_qcoth);
        let assign20010_e19947: f64 = (1.0 - assign20010_e19946);
        (assign20010_e19947, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign20010_e19949;
        locals.var_q_temp2_dn4 = assign20010_e19949_d_n4;
        locals.var_q_temp2_dn6 = assign20010_e19949_d_n6;
        locals.var_q_temp2_dn7 = assign20010_e19949_d_n7;
        locals.var_q_temp2_dn8 = assign20010_e19949_d_n8;
        locals.var_q_temp2_dn9 = assign20010_e19949_d_n9;
        locals.var_q_temp2_rv = 0.0;

        let (assign20020_e19961, assign20020_e19961_d_n4, assign20020_e19961_d_n6, assign20020_e19961_d_n7, assign20020_e19961_d_n8, assign20020_e19961_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign20020_e19957: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
        let assign20020_e19959: f64 = (assign20020_e19957 * locals.var_q_temp2);
        (assign20020_e19959, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign20020_e19957 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign20020_e19957 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign20020_e19957 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign20020_e19957 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign20020_e19957 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign20020_e19961;
        locals.var_q_d1_ln_dn4 = assign20020_e19961_d_n4;
        locals.var_q_d1_ln_dn6 = assign20020_e19961_d_n6;
        locals.var_q_d1_ln_dn7 = assign20020_e19961_d_n7;
        locals.var_q_d1_ln_dn8 = assign20020_e19961_d_n8;
        locals.var_q_d1_ln_dn9 = assign20020_e19961_d_n9;
        locals.var_q_d1_ln_rv = 0.0;

        let (assign20030_e19981, assign20030_e19981_d_n4, assign20030_e19981_d_n6, assign20030_e19981_d_n7, assign20030_e19981_d_n8, assign20030_e19981_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign20030_e19969: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
        let assign20030_e19974: f64 = (0.5 * locals.var_q_d1_qcoth);
        let assign20030_e19975: f64 = (locals.var_q_d1_ln + assign20030_e19974);
        let assign20030_e19976: f64 = (locals.var_q_d1_qsq * assign20030_e19975);
        let assign20030_e19977: f64 = (assign20030_e19969 - assign20030_e19976);
        let assign20030_e19979: f64 = (assign20030_e19977 / locals.var_q_qsq);
        (assign20030_e19979, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign20030_e19975) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign20030_e19977 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign20030_e19975) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign20030_e19977 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign20030_e19975) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign20030_e19977 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign20030_e19975) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign20030_e19977 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign20030_e19975) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign20030_e19977 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign20030_e19981;
        locals.var_q_d2_ln_dn4 = assign20030_e19981_d_n4;
        locals.var_q_d2_ln_dn6 = assign20030_e19981_d_n6;
        locals.var_q_d2_ln_dn7 = assign20030_e19981_d_n7;
        locals.var_q_d2_ln_dn8 = assign20030_e19981_d_n8;
        locals.var_q_d2_ln_dn9 = assign20030_e19981_d_n9;
        locals.var_q_d2_ln_rv = 0.0;

        let assign20040_e19984: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard650 = assign20040_e19984;
        locals.var_guard650_rv = 0.0;

        let (assign20050_e19997, assign20050_e19997_d_n4, assign20050_e19997_d_n6, assign20050_e19997_d_n7, assign20050_e19997_d_n8, assign20050_e19997_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard650 != 0.0)) {
        let assign20050_e19994: f64 = (locals.var_q_qsq).abs();
        let assign20050_e19995: f64 = (assign20050_e19994).sqrt();
        (assign20050_e19995, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign20050_e19995)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign20050_e19995)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign20050_e19995)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign20050_e19995)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign20050_e19995)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign20050_e19997;
        locals.var_q_rac_qsq_dn4 = assign20050_e19997_d_n4;
        locals.var_q_rac_qsq_dn6 = assign20050_e19997_d_n6;
        locals.var_q_rac_qsq_dn7 = assign20050_e19997_d_n7;
        locals.var_q_rac_qsq_dn8 = assign20050_e19997_d_n8;
        locals.var_q_rac_qsq_dn9 = assign20050_e19997_d_n9;
        locals.var_q_rac_qsq_rv = 0.0;

        let (assign20060_e20010, assign20060_e20010_d_n4, assign20060_e20010_d_n6, assign20060_e20010_d_n7, assign20060_e20010_d_n8, assign20060_e20010_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard650 != 0.0)) {
        let assign20060_e20007: f64 = (-locals.var_q_rac_qsq);
        let assign20060_e20008: f64 = (assign20060_e20007).exp();
        (assign20060_e20008, (assign20060_e20008 * (-locals.var_q_rac_qsq_dn4)), (assign20060_e20008 * (-locals.var_q_rac_qsq_dn6)), (assign20060_e20008 * (-locals.var_q_rac_qsq_dn7)), (assign20060_e20008 * (-locals.var_q_rac_qsq_dn8)), (assign20060_e20008 * (-locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_invexpq, locals.var_q_invexpq_dn4, locals.var_q_invexpq_dn6, locals.var_q_invexpq_dn7, locals.var_q_invexpq_dn8, locals.var_q_invexpq_dn9,)
    }
};
        locals.var_q_invexpq = assign20060_e20010;
        locals.var_q_invexpq_dn4 = assign20060_e20010_d_n4;
        locals.var_q_invexpq_dn6 = assign20060_e20010_d_n6;
        locals.var_q_invexpq_dn7 = assign20060_e20010_d_n7;
        locals.var_q_invexpq_dn8 = assign20060_e20010_d_n8;
        locals.var_q_invexpq_dn9 = assign20060_e20010_d_n9;
        locals.var_q_invexpq_rv = 0.0;

        let (assign20070_e20029, assign20070_e20029_d_n4, assign20070_e20029_d_n6, assign20070_e20029_d_n7, assign20070_e20029_d_n8, assign20070_e20029_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard650 != 0.0)) {
        let assign20070_e20022: f64 = (1.0 + locals.var_q_invexpq);
        let assign20070_e20023: f64 = (locals.var_q_rac_qsq * assign20070_e20022);
        let assign20070_e20026: f64 = (1.0 - locals.var_q_invexpq);
        let assign20070_e20027: f64 = (assign20070_e20023 / assign20070_e20026);
        (assign20070_e20027, (((((locals.var_q_rac_qsq_dn4 * assign20070_e20022) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn4)) * assign20070_e20026) - (assign20070_e20023 * (-locals.var_q_invexpq_dn4))) / (assign20070_e20026 * assign20070_e20026)), (((((locals.var_q_rac_qsq_dn6 * assign20070_e20022) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn6)) * assign20070_e20026) - (assign20070_e20023 * (-locals.var_q_invexpq_dn6))) / (assign20070_e20026 * assign20070_e20026)), (((((locals.var_q_rac_qsq_dn7 * assign20070_e20022) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn7)) * assign20070_e20026) - (assign20070_e20023 * (-locals.var_q_invexpq_dn7))) / (assign20070_e20026 * assign20070_e20026)), (((((locals.var_q_rac_qsq_dn8 * assign20070_e20022) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn8)) * assign20070_e20026) - (assign20070_e20023 * (-locals.var_q_invexpq_dn8))) / (assign20070_e20026 * assign20070_e20026)), (((((locals.var_q_rac_qsq_dn9 * assign20070_e20022) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn9)) * assign20070_e20026) - (assign20070_e20023 * (-locals.var_q_invexpq_dn9))) / (assign20070_e20026 * assign20070_e20026)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign20070_e20029;
        locals.var_q_qcoth_dn4 = assign20070_e20029_d_n4;
        locals.var_q_qcoth_dn6 = assign20070_e20029_d_n6;
        locals.var_q_qcoth_dn7 = assign20070_e20029_d_n7;
        locals.var_q_qcoth_dn8 = assign20070_e20029_d_n8;
        locals.var_q_qcoth_dn9 = assign20070_e20029_d_n9;
        locals.var_q_qcoth_rv = 0.0;

        let (assign20080_e20044, assign20080_e20044_d_n4, assign20080_e20044_d_n6, assign20080_e20044_d_n7, assign20080_e20044_d_n8, assign20080_e20044_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard650 != 0.0)) {
        let assign20080_e20040: f64 = (0.25 * locals.var_q_d1_qsq);
        let assign20080_e20042: f64 = (assign20080_e20040 / locals.var_q_qsq);
        (assign20080_e20042, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign20080_e20040 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign20080_e20040 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign20080_e20040 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign20080_e20040 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign20080_e20040 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign20080_e20044;
        locals.var_q_temp1_dn4 = assign20080_e20044_d_n4;
        locals.var_q_temp1_dn6 = assign20080_e20044_d_n6;
        locals.var_q_temp1_dn7 = assign20080_e20044_d_n7;
        locals.var_q_temp1_dn8 = assign20080_e20044_d_n8;
        locals.var_q_temp1_dn9 = assign20080_e20044_d_n9;
        locals.var_q_temp1_rv = 0.0;

        let (assign20090_e20063, assign20090_e20063_d_n4, assign20090_e20063_d_n6, assign20090_e20063_d_n7, assign20090_e20063_d_n8, assign20090_e20063_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard650 != 0.0)) {
        let assign20090_e20057: f64 = (2.0 - locals.var_q_qcoth);
        let assign20090_e20058: f64 = (locals.var_q_qcoth * assign20090_e20057);
        let assign20090_e20059: f64 = (locals.var_q_qsq + assign20090_e20058);
        let assign20090_e20061: f64 = (assign20090_e20059 * locals.var_q_temp1);
        (assign20090_e20061, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign20090_e20057) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign20090_e20059 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign20090_e20057) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign20090_e20059 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign20090_e20057) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign20090_e20059 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign20090_e20057) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign20090_e20059 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign20090_e20057) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign20090_e20059 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign20090_e20063;
        locals.var_q_d1_qcoth_dn4 = assign20090_e20063_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign20090_e20063_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign20090_e20063_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign20090_e20063_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign20090_e20063_d_n9;
        locals.var_q_d1_qcoth_rv = 0.0;

        let (assign20100_e20090, assign20100_e20090_d_n4, assign20100_e20090_d_n6, assign20100_e20090_d_n7, assign20100_e20090_d_n8, assign20100_e20090_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard650 != 0.0)) {
        let assign20100_e20075: f64 = (2.0 * locals.var_q_d1_qcoth);
        let assign20100_e20078: f64 = (1.0 + locals.var_q_qcoth);
        let assign20100_e20079: f64 = (assign20100_e20075 * assign20100_e20078);
        let assign20100_e20080: f64 = (locals.var_q_d1_qsq - assign20100_e20079);
        let assign20100_e20082: f64 = (assign20100_e20080 * locals.var_q_temp1);
        let assign20100_e20085: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
        let assign20100_e20087: f64 = (assign20100_e20085 / locals.var_q_d1_qsq);
        let assign20100_e20088: f64 = (assign20100_e20082 + assign20100_e20087);
        (assign20100_e20088, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign20100_e20078) + (assign20100_e20075 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign20100_e20080 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign20100_e20085 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign20100_e20078) + (assign20100_e20075 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign20100_e20080 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign20100_e20085 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign20100_e20078) + (assign20100_e20075 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign20100_e20080 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign20100_e20085 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign20100_e20078) + (assign20100_e20075 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign20100_e20080 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign20100_e20085 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign20100_e20078) + (assign20100_e20075 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign20100_e20080 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign20100_e20085 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign20100_e20090;
        locals.var_q_d2_qcoth_dn4 = assign20100_e20090_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign20100_e20090_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign20100_e20090_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign20100_e20090_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign20100_e20090_d_n9;
        locals.var_q_d2_qcoth_rv = 0.0;

        let (assign20110_e20105, assign20110_e20105_d_n4, assign20110_e20105_d_n6, assign20110_e20105_d_n7, assign20110_e20105_d_n8, assign20110_e20105_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard650 != 0.0)) {
        let assign20110_e20102: f64 = (0.5 * locals.var_q_qcoth);
        let assign20110_e20103: f64 = (1.0 - assign20110_e20102);
        (assign20110_e20103, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign20110_e20105;
        locals.var_q_temp2_dn4 = assign20110_e20105_d_n4;
        locals.var_q_temp2_dn6 = assign20110_e20105_d_n6;
        locals.var_q_temp2_dn7 = assign20110_e20105_d_n7;
        locals.var_q_temp2_dn8 = assign20110_e20105_d_n8;
        locals.var_q_temp2_dn9 = assign20110_e20105_d_n9;
        locals.var_q_temp2_rv = 0.0;

        let (assign20120_e20120, assign20120_e20120_d_n4, assign20120_e20120_d_n6, assign20120_e20120_d_n7, assign20120_e20120_d_n8, assign20120_e20120_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard650 != 0.0)) {
        let assign20120_e20116: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
        let assign20120_e20118: f64 = (assign20120_e20116 * locals.var_q_temp2);
        (assign20120_e20118, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign20120_e20116 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign20120_e20116 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign20120_e20116 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign20120_e20116 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign20120_e20116 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign20120_e20120;
        locals.var_q_d1_ln_dn4 = assign20120_e20120_d_n4;
        locals.var_q_d1_ln_dn6 = assign20120_e20120_d_n6;
        locals.var_q_d1_ln_dn7 = assign20120_e20120_d_n7;
        locals.var_q_d1_ln_dn8 = assign20120_e20120_d_n8;
        locals.var_q_d1_ln_dn9 = assign20120_e20120_d_n9;
        locals.var_q_d1_ln_rv = 0.0;

        let (assign20130_e20143, assign20130_e20143_d_n4, assign20130_e20143_d_n6, assign20130_e20143_d_n7, assign20130_e20143_d_n8, assign20130_e20143_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard650 != 0.0)) {
        let assign20130_e20131: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
        let assign20130_e20136: f64 = (0.5 * locals.var_q_d1_qcoth);
        let assign20130_e20137: f64 = (locals.var_q_d1_ln + assign20130_e20136);
        let assign20130_e20138: f64 = (locals.var_q_d1_qsq * assign20130_e20137);
        let assign20130_e20139: f64 = (assign20130_e20131 - assign20130_e20138);
        let assign20130_e20141: f64 = (assign20130_e20139 / locals.var_q_qsq);
        (assign20130_e20141, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign20130_e20137) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign20130_e20139 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign20130_e20137) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign20130_e20139 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign20130_e20137) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign20130_e20139 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign20130_e20137) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign20130_e20139 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign20130_e20137) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign20130_e20139 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign20130_e20143;
        locals.var_q_d2_ln_dn4 = assign20130_e20143_d_n4;
        locals.var_q_d2_ln_dn6 = assign20130_e20143_d_n6;
        locals.var_q_d2_ln_dn7 = assign20130_e20143_d_n7;
        locals.var_q_d2_ln_dn8 = assign20130_e20143_d_n8;
        locals.var_q_d2_ln_dn9 = assign20130_e20143_d_n9;
        locals.var_q_d2_ln_rv = 0.0;

        let (assign20140_e20173, assign20140_e20173_d_n4, assign20140_e20173_d_n6, assign20140_e20173_d_n7, assign20140_e20173_d_n8, assign20140_e20173_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard650 == 0.0)) {
        let assign20140_e20157: f64 = (locals.var_q_qsq * 0.0166666666667);
        let assign20140_e20161: f64 = (locals.var_q_qsq * 0.0238095238095);
        let assign20140_e20165: f64 = (locals.var_q_qsq * 0.025);
        let assign20140_e20166: f64 = (1.0 - assign20140_e20165);
        let assign20140_e20167: f64 = (assign20140_e20161 * assign20140_e20166);
        let assign20140_e20168: f64 = (1.0 - assign20140_e20167);
        let assign20140_e20169: f64 = (assign20140_e20157 * assign20140_e20168);
        let assign20140_e20170: f64 = (1.0 - assign20140_e20169);
        let assign20140_e20171: f64 = (0.1666666666667 * assign20140_e20170);
        (assign20140_e20171, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0166666666667) * assign20140_e20168) + (assign20140_e20157 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign20140_e20166) + (assign20140_e20161 * (-(locals.var_q_qsq_dn4 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0166666666667) * assign20140_e20168) + (assign20140_e20157 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign20140_e20166) + (assign20140_e20161 * (-(locals.var_q_qsq_dn6 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0166666666667) * assign20140_e20168) + (assign20140_e20157 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign20140_e20166) + (assign20140_e20161 * (-(locals.var_q_qsq_dn7 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0166666666667) * assign20140_e20168) + (assign20140_e20157 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign20140_e20166) + (assign20140_e20161 * (-(locals.var_q_qsq_dn8 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0166666666667) * assign20140_e20168) + (assign20140_e20157 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign20140_e20166) + (assign20140_e20161 * (-(locals.var_q_qsq_dn9 * 0.025))))))))),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign20140_e20173;
        locals.var_q_temp3_dn4 = assign20140_e20173_d_n4;
        locals.var_q_temp3_dn6 = assign20140_e20173_d_n6;
        locals.var_q_temp3_dn7 = assign20140_e20173_d_n7;
        locals.var_q_temp3_dn8 = assign20140_e20173_d_n8;
        locals.var_q_temp3_dn9 = assign20140_e20173_d_n9;
        locals.var_q_temp3_rv = 0.0;

        let (assign20150_e20189, assign20150_e20189_d_n4, assign20150_e20189_d_n6, assign20150_e20189_d_n7, assign20150_e20189_d_n8, assign20150_e20189_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard650 == 0.0)) {
        let assign20150_e20186: f64 = (locals.var_q_qsq * locals.var_q_temp3);
        let assign20150_e20187: f64 = (2.0 + assign20150_e20186);
        (assign20150_e20187, ((locals.var_q_qsq_dn4 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn4)), ((locals.var_q_qsq_dn6 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn6)), ((locals.var_q_qsq_dn7 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn7)), ((locals.var_q_qsq_dn8 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn8)), ((locals.var_q_qsq_dn9 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn9)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign20150_e20189;
        locals.var_q_qcoth_dn4 = assign20150_e20189_d_n4;
        locals.var_q_qcoth_dn6 = assign20150_e20189_d_n6;
        locals.var_q_qcoth_dn7 = assign20150_e20189_d_n7;
        locals.var_q_qcoth_dn8 = assign20150_e20189_d_n8;
        locals.var_q_qcoth_dn9 = assign20150_e20189_d_n9;
        locals.var_q_qcoth_rv = 0.0;

        let (assign20160_e20219, assign20160_e20219_d_n4, assign20160_e20219_d_n6, assign20160_e20219_d_n7, assign20160_e20219_d_n8, assign20160_e20219_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard650 == 0.0)) {
        let assign20160_e20203: f64 = (locals.var_q_qsq * 0.0333333333333);
        let assign20160_e20207: f64 = (locals.var_q_qsq * 0.0357142857143);
        let assign20160_e20211: f64 = (locals.var_q_qsq * 0.0333333333333);
        let assign20160_e20212: f64 = (1.0 - assign20160_e20211);
        let assign20160_e20213: f64 = (assign20160_e20207 * assign20160_e20212);
        let assign20160_e20214: f64 = (1.0 - assign20160_e20213);
        let assign20160_e20215: f64 = (assign20160_e20203 * assign20160_e20214);
        let assign20160_e20216: f64 = (1.0 - assign20160_e20215);
        let assign20160_e20217: f64 = (0.1666666666667 * assign20160_e20216);
        (assign20160_e20217, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0333333333333) * assign20160_e20214) + (assign20160_e20203 * (-(((locals.var_q_qsq_dn4 * 0.0357142857143) * assign20160_e20212) + (assign20160_e20207 * (-(locals.var_q_qsq_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0333333333333) * assign20160_e20214) + (assign20160_e20203 * (-(((locals.var_q_qsq_dn6 * 0.0357142857143) * assign20160_e20212) + (assign20160_e20207 * (-(locals.var_q_qsq_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0333333333333) * assign20160_e20214) + (assign20160_e20203 * (-(((locals.var_q_qsq_dn7 * 0.0357142857143) * assign20160_e20212) + (assign20160_e20207 * (-(locals.var_q_qsq_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0333333333333) * assign20160_e20214) + (assign20160_e20203 * (-(((locals.var_q_qsq_dn8 * 0.0357142857143) * assign20160_e20212) + (assign20160_e20207 * (-(locals.var_q_qsq_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0333333333333) * assign20160_e20214) + (assign20160_e20203 * (-(((locals.var_q_qsq_dn9 * 0.0357142857143) * assign20160_e20212) + (assign20160_e20207 * (-(locals.var_q_qsq_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign20160_e20219;
        locals.var_q_temp1_dn4 = assign20160_e20219_d_n4;
        locals.var_q_temp1_dn6 = assign20160_e20219_d_n6;
        locals.var_q_temp1_dn7 = assign20160_e20219_d_n7;
        locals.var_q_temp1_dn8 = assign20160_e20219_d_n8;
        locals.var_q_temp1_dn9 = assign20160_e20219_d_n9;
        locals.var_q_temp1_rv = 0.0;

        let (assign20170_e20233, assign20170_e20233_d_n4, assign20170_e20233_d_n6, assign20170_e20233_d_n7, assign20170_e20233_d_n8, assign20170_e20233_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard650 == 0.0)) {
        let assign20170_e20231: f64 = (locals.var_q_d1_qsq * locals.var_q_temp1);
        (assign20170_e20231, ((locals.var_q_d1_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn4)), ((locals.var_q_d1_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn6)), ((locals.var_q_d1_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn7)), ((locals.var_q_d1_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn8)), ((locals.var_q_d1_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign20170_e20233;
        locals.var_q_d1_qcoth_dn4 = assign20170_e20233_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign20170_e20233_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign20170_e20233_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign20170_e20233_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign20170_e20233_d_n9;
        locals.var_q_d1_qcoth_rv = 0.0;

        let (assign20180_e20263, assign20180_e20263_d_n4, assign20180_e20263_d_n6, assign20180_e20263_d_n7, assign20180_e20263_d_n8, assign20180_e20263_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard650 == 0.0)) {
        let assign20180_e20247: f64 = (locals.var_q_qsq * 0.0714285714286);
        let assign20180_e20251: f64 = (0.05 * locals.var_q_qsq);
        let assign20180_e20255: f64 = (0.0420875420875421 * locals.var_q_qsq);
        let assign20180_e20256: f64 = (1.0 - assign20180_e20255);
        let assign20180_e20257: f64 = (assign20180_e20251 * assign20180_e20256);
        let assign20180_e20258: f64 = (1.0 - assign20180_e20257);
        let assign20180_e20259: f64 = (assign20180_e20247 * assign20180_e20258);
        let assign20180_e20260: f64 = (1.0 - assign20180_e20259);
        let assign20180_e20261: f64 = (0.0055555555556 * assign20180_e20260);
        (assign20180_e20261, (0.0055555555556 * (-(((locals.var_q_qsq_dn4 * 0.0714285714286) * assign20180_e20258) + (assign20180_e20247 * (-(((0.05 * locals.var_q_qsq_dn4) * assign20180_e20256) + (assign20180_e20251 * (-(0.0420875420875421 * locals.var_q_qsq_dn4))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn6 * 0.0714285714286) * assign20180_e20258) + (assign20180_e20247 * (-(((0.05 * locals.var_q_qsq_dn6) * assign20180_e20256) + (assign20180_e20251 * (-(0.0420875420875421 * locals.var_q_qsq_dn6))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn7 * 0.0714285714286) * assign20180_e20258) + (assign20180_e20247 * (-(((0.05 * locals.var_q_qsq_dn7) * assign20180_e20256) + (assign20180_e20251 * (-(0.0420875420875421 * locals.var_q_qsq_dn7))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn8 * 0.0714285714286) * assign20180_e20258) + (assign20180_e20247 * (-(((0.05 * locals.var_q_qsq_dn8) * assign20180_e20256) + (assign20180_e20251 * (-(0.0420875420875421 * locals.var_q_qsq_dn8))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn9 * 0.0714285714286) * assign20180_e20258) + (assign20180_e20247 * (-(((0.05 * locals.var_q_qsq_dn9) * assign20180_e20256) + (assign20180_e20251 * (-(0.0420875420875421 * locals.var_q_qsq_dn9))))))))),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign20180_e20263;
        locals.var_q_temp2_dn4 = assign20180_e20263_d_n4;
        locals.var_q_temp2_dn6 = assign20180_e20263_d_n6;
        locals.var_q_temp2_dn7 = assign20180_e20263_d_n7;
        locals.var_q_temp2_dn8 = assign20180_e20263_d_n8;
        locals.var_q_temp2_dn9 = assign20180_e20263_d_n9;
        locals.var_q_temp2_rv = 0.0;

        let (assign20190_e20283, assign20190_e20283_d_n4, assign20190_e20283_d_n6, assign20190_e20283_d_n7, assign20190_e20283_d_n8, assign20190_e20283_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard650 == 0.0)) {
        let assign20190_e20275: f64 = (locals.var_q_d2_qsq * locals.var_q_temp1);
        let assign20190_e20278: f64 = (locals.var_q_d1_qsq * locals.var_q_d1_qsq);
        let assign20190_e20280: f64 = (assign20190_e20278 * locals.var_q_temp2);
        let assign20190_e20281: f64 = (assign20190_e20275 - assign20190_e20280);
        (assign20190_e20281, (((locals.var_q_d2_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn4)) - ((((locals.var_q_d1_qsq_dn4 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn4)) * locals.var_q_temp2) + (assign20190_e20278 * locals.var_q_temp2_dn4))), (((locals.var_q_d2_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn6)) - ((((locals.var_q_d1_qsq_dn6 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn6)) * locals.var_q_temp2) + (assign20190_e20278 * locals.var_q_temp2_dn6))), (((locals.var_q_d2_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn7)) - ((((locals.var_q_d1_qsq_dn7 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn7)) * locals.var_q_temp2) + (assign20190_e20278 * locals.var_q_temp2_dn7))), (((locals.var_q_d2_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn8)) - ((((locals.var_q_d1_qsq_dn8 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn8)) * locals.var_q_temp2) + (assign20190_e20278 * locals.var_q_temp2_dn8))), (((locals.var_q_d2_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn9)) - ((((locals.var_q_d1_qsq_dn9 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn9)) * locals.var_q_temp2) + (assign20190_e20278 * locals.var_q_temp2_dn9))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign20190_e20283;
        locals.var_q_d2_qcoth_dn4 = assign20190_e20283_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign20190_e20283_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign20190_e20283_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign20190_e20283_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign20190_e20283_d_n9;
        locals.var_q_d2_qcoth_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_55(
        locals: &mut StampLocals,
    ) {
        let (assign20200_e20300, assign20200_e20300_d_n4, assign20200_e20300_d_n6, assign20200_e20300_d_n7, assign20200_e20300_d_n8, assign20200_e20300_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard650 == 0.0)) {
        let assign20200_e20294: f64 = (-0.5);
        let assign20200_e20296: f64 = (assign20200_e20294 * locals.var_q_d1_qsq);
        let assign20200_e20298: f64 = (assign20200_e20296 * locals.var_q_temp3);
        (assign20200_e20298, (((assign20200_e20294 * locals.var_q_d1_qsq_dn4) * locals.var_q_temp3) + (assign20200_e20296 * locals.var_q_temp3_dn4)), (((assign20200_e20294 * locals.var_q_d1_qsq_dn6) * locals.var_q_temp3) + (assign20200_e20296 * locals.var_q_temp3_dn6)), (((assign20200_e20294 * locals.var_q_d1_qsq_dn7) * locals.var_q_temp3) + (assign20200_e20296 * locals.var_q_temp3_dn7)), (((assign20200_e20294 * locals.var_q_d1_qsq_dn8) * locals.var_q_temp3) + (assign20200_e20296 * locals.var_q_temp3_dn8)), (((assign20200_e20294 * locals.var_q_d1_qsq_dn9) * locals.var_q_temp3) + (assign20200_e20296 * locals.var_q_temp3_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign20200_e20300;
        locals.var_q_d1_ln_dn4 = assign20200_e20300_d_n4;
        locals.var_q_d1_ln_dn6 = assign20200_e20300_d_n6;
        locals.var_q_d1_ln_dn7 = assign20200_e20300_d_n7;
        locals.var_q_d1_ln_dn8 = assign20200_e20300_d_n8;
        locals.var_q_d1_ln_dn9 = assign20200_e20300_d_n9;
        locals.var_q_d1_ln_rv = 0.0;

        let (assign20210_e20337, assign20210_e20337_d_n4, assign20210_e20337_d_n6, assign20210_e20337_d_n7, assign20210_e20337_d_n8, assign20210_e20337_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard650 == 0.0)) {
        let assign20210_e20311: f64 = (-0.5);
        let assign20210_e20313: f64 = (assign20210_e20311 * locals.var_q_d2_qsq);
        let assign20210_e20315: f64 = (assign20210_e20313 * locals.var_q_temp3);
        let assign20210_e20318: f64 = (0.25 * 0.0055555555556);
        let assign20210_e20320: f64 = (assign20210_e20318 * locals.var_q_d1_qsq);
        let assign20210_e20322: f64 = (assign20210_e20320 * locals.var_q_d1_qsq);
        let assign20210_e20326: f64 = (locals.var_q_qsq * 0.0238095238095);
        let assign20210_e20330: f64 = (0.075 * locals.var_q_qsq);
        let assign20210_e20331: f64 = (2.0 - assign20210_e20330);
        let assign20210_e20332: f64 = (assign20210_e20326 * assign20210_e20331);
        let assign20210_e20333: f64 = (1.0 - assign20210_e20332);
        let assign20210_e20334: f64 = (assign20210_e20322 * assign20210_e20333);
        let assign20210_e20335: f64 = (assign20210_e20315 + assign20210_e20334);
        (assign20210_e20335, ((((assign20210_e20311 * locals.var_q_d2_qsq_dn4) * locals.var_q_temp3) + (assign20210_e20313 * locals.var_q_temp3_dn4)) + (((((assign20210_e20318 * locals.var_q_d1_qsq_dn4) * locals.var_q_d1_qsq) + (assign20210_e20320 * locals.var_q_d1_qsq_dn4)) * assign20210_e20333) + (assign20210_e20322 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign20210_e20331) + (assign20210_e20326 * (-(0.075 * locals.var_q_qsq_dn4)))))))), ((((assign20210_e20311 * locals.var_q_d2_qsq_dn6) * locals.var_q_temp3) + (assign20210_e20313 * locals.var_q_temp3_dn6)) + (((((assign20210_e20318 * locals.var_q_d1_qsq_dn6) * locals.var_q_d1_qsq) + (assign20210_e20320 * locals.var_q_d1_qsq_dn6)) * assign20210_e20333) + (assign20210_e20322 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign20210_e20331) + (assign20210_e20326 * (-(0.075 * locals.var_q_qsq_dn6)))))))), ((((assign20210_e20311 * locals.var_q_d2_qsq_dn7) * locals.var_q_temp3) + (assign20210_e20313 * locals.var_q_temp3_dn7)) + (((((assign20210_e20318 * locals.var_q_d1_qsq_dn7) * locals.var_q_d1_qsq) + (assign20210_e20320 * locals.var_q_d1_qsq_dn7)) * assign20210_e20333) + (assign20210_e20322 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign20210_e20331) + (assign20210_e20326 * (-(0.075 * locals.var_q_qsq_dn7)))))))), ((((assign20210_e20311 * locals.var_q_d2_qsq_dn8) * locals.var_q_temp3) + (assign20210_e20313 * locals.var_q_temp3_dn8)) + (((((assign20210_e20318 * locals.var_q_d1_qsq_dn8) * locals.var_q_d1_qsq) + (assign20210_e20320 * locals.var_q_d1_qsq_dn8)) * assign20210_e20333) + (assign20210_e20322 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign20210_e20331) + (assign20210_e20326 * (-(0.075 * locals.var_q_qsq_dn8)))))))), ((((assign20210_e20311 * locals.var_q_d2_qsq_dn9) * locals.var_q_temp3) + (assign20210_e20313 * locals.var_q_temp3_dn9)) + (((((assign20210_e20318 * locals.var_q_d1_qsq_dn9) * locals.var_q_d1_qsq) + (assign20210_e20320 * locals.var_q_d1_qsq_dn9)) * assign20210_e20333) + (assign20210_e20322 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign20210_e20331) + (assign20210_e20326 * (-(0.075 * locals.var_q_qsq_dn9)))))))),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign20210_e20337;
        locals.var_q_d2_ln_dn4 = assign20210_e20337_d_n4;
        locals.var_q_d2_ln_dn6 = assign20210_e20337_d_n6;
        locals.var_q_d2_ln_dn7 = assign20210_e20337_d_n7;
        locals.var_q_d2_ln_dn8 = assign20210_e20337_d_n8;
        locals.var_q_d2_ln_dn9 = assign20210_e20337_d_n9;
        locals.var_q_d2_ln_rv = 0.0;

        let assign20220_e20340: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard651 = assign20220_e20340;
        locals.var_guard651_rv = 0.0;

        let (assign20230_e20358, assign20230_e20358_d_n4, assign20230_e20358_d_n6, assign20230_e20358_d_n7, assign20230_e20358_d_n8, assign20230_e20358_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard651 != 0.0)) {
        let assign20230_e20348: f64 = (4.0 * locals.var_q_qsq);
        let assign20230_e20353: f64 = (2.0 - locals.var_q_invexpq);
        let assign20230_e20354: f64 = (locals.var_q_invexpq * assign20230_e20353);
        let assign20230_e20355: f64 = (1.0 - assign20230_e20354);
        let assign20230_e20356: f64 = (assign20230_e20348 / assign20230_e20355);
        (assign20230_e20356, ((((4.0 * locals.var_q_qsq_dn4) * assign20230_e20355) - (assign20230_e20348 * (-((locals.var_q_invexpq_dn4 * assign20230_e20353) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn4)))))) / (assign20230_e20355 * assign20230_e20355)), ((((4.0 * locals.var_q_qsq_dn6) * assign20230_e20355) - (assign20230_e20348 * (-((locals.var_q_invexpq_dn6 * assign20230_e20353) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn6)))))) / (assign20230_e20355 * assign20230_e20355)), ((((4.0 * locals.var_q_qsq_dn7) * assign20230_e20355) - (assign20230_e20348 * (-((locals.var_q_invexpq_dn7 * assign20230_e20353) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn7)))))) / (assign20230_e20355 * assign20230_e20355)), ((((4.0 * locals.var_q_qsq_dn8) * assign20230_e20355) - (assign20230_e20348 * (-((locals.var_q_invexpq_dn8 * assign20230_e20353) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn8)))))) / (assign20230_e20355 * assign20230_e20355)), ((((4.0 * locals.var_q_qsq_dn9) * assign20230_e20355) - (assign20230_e20348 * (-((locals.var_q_invexpq_dn9 * assign20230_e20353) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn9)))))) / (assign20230_e20355 * assign20230_e20355)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign20230_e20358;
        locals.var_q_temp2_dn4 = assign20230_e20358_d_n4;
        locals.var_q_temp2_dn6 = assign20230_e20358_d_n6;
        locals.var_q_temp2_dn7 = assign20230_e20358_d_n7;
        locals.var_q_temp2_dn8 = assign20230_e20358_d_n8;
        locals.var_q_temp2_dn9 = assign20230_e20358_d_n9;
        locals.var_q_temp2_rv = 0.0;

        let (assign20240_e20368, assign20240_e20368_d_n4, assign20240_e20368_d_n6, assign20240_e20368_d_n7, assign20240_e20368_d_n8, assign20240_e20368_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard651 != 0.0)) {
        let assign20240_e20366: f64 = (locals.var_q_temp2 * locals.var_q_invexpq);
        (assign20240_e20366, ((locals.var_q_temp2_dn4 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn4)), ((locals.var_q_temp2_dn6 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn6)), ((locals.var_q_temp2_dn7 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn7)), ((locals.var_q_temp2_dn8 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn8)), ((locals.var_q_temp2_dn9 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn9)),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign20240_e20368;
        locals.var_q_sh_term_dn4 = assign20240_e20368_d_n4;
        locals.var_q_sh_term_dn6 = assign20240_e20368_d_n6;
        locals.var_q_sh_term_dn7 = assign20240_e20368_d_n7;
        locals.var_q_sh_term_dn8 = assign20240_e20368_d_n8;
        locals.var_q_sh_term_dn9 = assign20240_e20368_d_n9;
        locals.var_q_sh_term_rv = 0.0;

        let (assign20250_e20379, assign20250_e20379_d_n4, assign20250_e20379_d_n6, assign20250_e20379_d_n7, assign20250_e20379_d_n8, assign20250_e20379_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard651 != 0.0)) {
        let assign20250_e20375: f64 = (locals.var_q_temp2).ln();
        let assign20250_e20377: f64 = (assign20250_e20375 - locals.var_q_rac_qsq);
        (assign20250_e20377, ((locals.var_q_temp2_dn4 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn4), ((locals.var_q_temp2_dn6 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn6), ((locals.var_q_temp2_dn7 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn7), ((locals.var_q_temp2_dn8 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn8), ((locals.var_q_temp2_dn9 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn9),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign20250_e20379;
        locals.var_q_ln_term_dn4 = assign20250_e20379_d_n4;
        locals.var_q_ln_term_dn6 = assign20250_e20379_d_n6;
        locals.var_q_ln_term_dn7 = assign20250_e20379_d_n7;
        locals.var_q_ln_term_dn8 = assign20250_e20379_d_n8;
        locals.var_q_ln_term_dn9 = assign20250_e20379_d_n9;
        locals.var_q_ln_term_rv = 0.0;

        let assign20260_e20382: f64 = (-0.005);
        let assign20260_e20383: f64 = if locals.var_q_qsq < assign20260_e20382 { 1.0 } else { 0.0 };
        locals.var_guard652 = assign20260_e20383;
        locals.var_guard652_rv = 0.0;

        let (assign20270_e20397, assign20270_e20397_d_n4, assign20270_e20397_d_n6, assign20270_e20397_d_n7, assign20270_e20397_d_n8, assign20270_e20397_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard651 == 0.0)) && (locals.var_guard652 != 0.0)) {
        let assign20270_e20394: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign20270_e20395: f64 = (assign20270_e20394).sin();
        (assign20270_e20395, ((assign20270_e20394).cos() * (0.5 * locals.var_q_rac_qsq_dn4)), ((assign20270_e20394).cos() * (0.5 * locals.var_q_rac_qsq_dn6)), ((assign20270_e20394).cos() * (0.5 * locals.var_q_rac_qsq_dn7)), ((assign20270_e20394).cos() * (0.5 * locals.var_q_rac_qsq_dn8)), ((assign20270_e20394).cos() * (0.5 * locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign20270_e20397;
        locals.var_q_temp2_dn4 = assign20270_e20397_d_n4;
        locals.var_q_temp2_dn6 = assign20270_e20397_d_n6;
        locals.var_q_temp2_dn7 = assign20270_e20397_d_n7;
        locals.var_q_temp2_dn8 = assign20270_e20397_d_n8;
        locals.var_q_temp2_dn9 = assign20270_e20397_d_n9;
        locals.var_q_temp2_rv = 0.0;

        let (assign20280_e20413, assign20280_e20413_d_n4, assign20280_e20413_d_n6, assign20280_e20413_d_n7, assign20280_e20413_d_n8, assign20280_e20413_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard651 == 0.0)) && (locals.var_guard652 != 0.0)) {
        let assign20280_e20407: f64 = (-locals.var_q_qsq);
        let assign20280_e20410: f64 = (locals.var_q_temp2 * locals.var_q_temp2);
        let assign20280_e20411: f64 = (assign20280_e20407 / assign20280_e20410);
        (assign20280_e20411, ((((-locals.var_q_qsq_dn4) * assign20280_e20410) - (assign20280_e20407 * ((locals.var_q_temp2_dn4 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn4)))) / (assign20280_e20410 * assign20280_e20410)), ((((-locals.var_q_qsq_dn6) * assign20280_e20410) - (assign20280_e20407 * ((locals.var_q_temp2_dn6 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn6)))) / (assign20280_e20410 * assign20280_e20410)), ((((-locals.var_q_qsq_dn7) * assign20280_e20410) - (assign20280_e20407 * ((locals.var_q_temp2_dn7 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn7)))) / (assign20280_e20410 * assign20280_e20410)), ((((-locals.var_q_qsq_dn8) * assign20280_e20410) - (assign20280_e20407 * ((locals.var_q_temp2_dn8 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn8)))) / (assign20280_e20410 * assign20280_e20410)), ((((-locals.var_q_qsq_dn9) * assign20280_e20410) - (assign20280_e20407 * ((locals.var_q_temp2_dn9 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn9)))) / (assign20280_e20410 * assign20280_e20410)),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign20280_e20413;
        locals.var_q_sh_term_dn4 = assign20280_e20413_d_n4;
        locals.var_q_sh_term_dn6 = assign20280_e20413_d_n6;
        locals.var_q_sh_term_dn7 = assign20280_e20413_d_n7;
        locals.var_q_sh_term_dn8 = assign20280_e20413_d_n8;
        locals.var_q_sh_term_dn9 = assign20280_e20413_d_n9;
        locals.var_q_sh_term_rv = 0.0;

        let (assign20290_e20425, assign20290_e20425_d_n4, assign20290_e20425_d_n6, assign20290_e20425_d_n7, assign20290_e20425_d_n8, assign20290_e20425_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard651 == 0.0)) && (locals.var_guard652 != 0.0)) {
        let assign20290_e20423: f64 = (locals.var_q_sh_term).ln();
        (assign20290_e20423, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign20290_e20425;
        locals.var_q_ln_term_dn4 = assign20290_e20425_d_n4;
        locals.var_q_ln_term_dn6 = assign20290_e20425_d_n6;
        locals.var_q_ln_term_dn7 = assign20290_e20425_d_n7;
        locals.var_q_ln_term_dn8 = assign20290_e20425_d_n8;
        locals.var_q_ln_term_dn9 = assign20290_e20425_d_n9;
        locals.var_q_ln_term_rv = 0.0;

        let (assign20300_e20453, assign20300_e20453_d_n4, assign20300_e20453_d_n6, assign20300_e20453_d_n7, assign20300_e20453_d_n8, assign20300_e20453_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard651 == 0.0)) && (locals.var_guard652 == 0.0)) {
        let assign20300_e20438: f64 = (locals.var_q_qsq * 0.3333333333333);
        let assign20300_e20442: f64 = (0.05 * locals.var_q_qsq);
        let assign20300_e20446: f64 = (0.0396825396825397 * locals.var_q_qsq);
        let assign20300_e20447: f64 = (1.0 - assign20300_e20446);
        let assign20300_e20448: f64 = (assign20300_e20442 * assign20300_e20447);
        let assign20300_e20449: f64 = (1.0 - assign20300_e20448);
        let assign20300_e20450: f64 = (assign20300_e20438 * assign20300_e20449);
        let assign20300_e20451: f64 = (4.0 - assign20300_e20450);
        (assign20300_e20451, (-(((locals.var_q_qsq_dn4 * 0.3333333333333) * assign20300_e20449) + (assign20300_e20438 * (-(((0.05 * locals.var_q_qsq_dn4) * assign20300_e20447) + (assign20300_e20442 * (-(0.0396825396825397 * locals.var_q_qsq_dn4)))))))), (-(((locals.var_q_qsq_dn6 * 0.3333333333333) * assign20300_e20449) + (assign20300_e20438 * (-(((0.05 * locals.var_q_qsq_dn6) * assign20300_e20447) + (assign20300_e20442 * (-(0.0396825396825397 * locals.var_q_qsq_dn6)))))))), (-(((locals.var_q_qsq_dn7 * 0.3333333333333) * assign20300_e20449) + (assign20300_e20438 * (-(((0.05 * locals.var_q_qsq_dn7) * assign20300_e20447) + (assign20300_e20442 * (-(0.0396825396825397 * locals.var_q_qsq_dn7)))))))), (-(((locals.var_q_qsq_dn8 * 0.3333333333333) * assign20300_e20449) + (assign20300_e20438 * (-(((0.05 * locals.var_q_qsq_dn8) * assign20300_e20447) + (assign20300_e20442 * (-(0.0396825396825397 * locals.var_q_qsq_dn8)))))))), (-(((locals.var_q_qsq_dn9 * 0.3333333333333) * assign20300_e20449) + (assign20300_e20438 * (-(((0.05 * locals.var_q_qsq_dn9) * assign20300_e20447) + (assign20300_e20442 * (-(0.0396825396825397 * locals.var_q_qsq_dn9)))))))),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign20300_e20453;
        locals.var_q_sh_term_dn4 = assign20300_e20453_d_n4;
        locals.var_q_sh_term_dn6 = assign20300_e20453_d_n6;
        locals.var_q_sh_term_dn7 = assign20300_e20453_d_n7;
        locals.var_q_sh_term_dn8 = assign20300_e20453_d_n8;
        locals.var_q_sh_term_dn9 = assign20300_e20453_d_n9;
        locals.var_q_sh_term_rv = 0.0;

        let (assign20310_e20466, assign20310_e20466_d_n4, assign20310_e20466_d_n6, assign20310_e20466_d_n7, assign20310_e20466_d_n8, assign20310_e20466_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard651 == 0.0)) && (locals.var_guard652 == 0.0)) {
        let assign20310_e20464: f64 = (locals.var_q_sh_term).ln();
        (assign20310_e20464, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign20310_e20466;
        locals.var_q_ln_term_dn4 = assign20310_e20466_d_n4;
        locals.var_q_ln_term_dn6 = assign20310_e20466_d_n6;
        locals.var_q_ln_term_dn7 = assign20310_e20466_d_n7;
        locals.var_q_ln_term_dn8 = assign20310_e20466_d_n8;
        locals.var_q_ln_term_dn9 = assign20310_e20466_d_n9;
        locals.var_q_ln_term_rv = 0.0;

        let assign20320_e20469: f64 = (1.01 * locals.var_q_k1q1);
        let assign20320_e20471: f64 = (assign20320_e20469 + locals.var_q_qcoth);
        let assign20320_e20473: f64 = if assign20320_e20471 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard653 = assign20320_e20473;
        locals.var_guard653_rv = 0.0;

        let (assign20330_e20483, assign20330_e20483_d_n4, assign20330_e20483_d_n6, assign20330_e20483_d_n7, assign20330_e20483_d_n8, assign20330_e20483_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard653 != 0.0)) {
        let assign20330_e20481: f64 = (locals.var_q_k1q1 + locals.var_q_qcoth);
        (assign20330_e20481, (locals.var_q_k1q1_dn4 + locals.var_q_qcoth_dn4), (locals.var_q_k1q1_dn6 + locals.var_q_qcoth_dn6), (locals.var_q_k1q1_dn7 + locals.var_q_qcoth_dn7), (locals.var_q_k1q1_dn8 + locals.var_q_qcoth_dn8), (locals.var_q_k1q1_dn9 + locals.var_q_qcoth_dn9),)
    } else {
        (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9,)
    }
};
        locals.var_q_expnum = assign20330_e20483;
        locals.var_q_expnum_dn4 = assign20330_e20483_d_n4;
        locals.var_q_expnum_dn6 = assign20330_e20483_d_n6;
        locals.var_q_expnum_dn7 = assign20330_e20483_d_n7;
        locals.var_q_expnum_dn8 = assign20330_e20483_d_n8;
        locals.var_q_expnum_dn9 = assign20330_e20483_d_n9;
        locals.var_q_expnum_rv = 0.0;

        let (assign20340_e20493, assign20340_e20493_d_n4, assign20340_e20493_d_n6, assign20340_e20493_d_n7, assign20340_e20493_d_n8, assign20340_e20493_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard653 != 0.0)) {
        let assign20340_e20491: f64 = (locals.var_k1 + locals.var_q_d1_qcoth);
        (assign20340_e20491, (locals.var_k1_dn4 + locals.var_q_d1_qcoth_dn4), (locals.var_k1_dn6 + locals.var_q_d1_qcoth_dn6), (locals.var_k1_dn7 + locals.var_q_d1_qcoth_dn7), (locals.var_k1_dn8 + locals.var_q_d1_qcoth_dn8), (locals.var_k1_dn9 + locals.var_q_d1_qcoth_dn9),)
    } else {
        (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9,)
    }
};
        locals.var_q_d1_expnum = assign20340_e20493;
        locals.var_q_d1_expnum_dn4 = assign20340_e20493_d_n4;
        locals.var_q_d1_expnum_dn6 = assign20340_e20493_d_n6;
        locals.var_q_d1_expnum_dn7 = assign20340_e20493_d_n7;
        locals.var_q_d1_expnum_dn8 = assign20340_e20493_d_n8;
        locals.var_q_d1_expnum_dn9 = assign20340_e20493_d_n9;
        locals.var_q_d1_expnum_rv = 0.0;

        let (assign20350_e20501, assign20350_e20501_d_n4, assign20350_e20501_d_n6, assign20350_e20501_d_n7, assign20350_e20501_d_n8, assign20350_e20501_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard653 != 0.0)) {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    } else {
        (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9,)
    }
};
        locals.var_q_d2_expnum = assign20350_e20501;
        locals.var_q_d2_expnum_dn4 = assign20350_e20501_d_n4;
        locals.var_q_d2_expnum_dn6 = assign20350_e20501_d_n6;
        locals.var_q_d2_expnum_dn7 = assign20350_e20501_d_n7;
        locals.var_q_d2_expnum_dn8 = assign20350_e20501_d_n8;
        locals.var_q_d2_expnum_dn9 = assign20350_e20501_d_n9;
        locals.var_q_d2_expnum_rv = 0.0;

        let (assign20360_e20514, assign20360_e20514_d_n4, assign20360_e20514_d_n6, assign20360_e20514_d_n7, assign20360_e20514_d_n8, assign20360_e20514_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard653 == 0.0)) {
        let assign20360_e20511: f64 = (locals.var_q_k1q1 - locals.var_q_qcoth);
        let assign20360_e20512: f64 = (1.0 / assign20360_e20511);
        (assign20360_e20512, (-((locals.var_q_k1q1_dn4 - locals.var_q_qcoth_dn4) / (assign20360_e20511 * assign20360_e20511))), (-((locals.var_q_k1q1_dn6 - locals.var_q_qcoth_dn6) / (assign20360_e20511 * assign20360_e20511))), (-((locals.var_q_k1q1_dn7 - locals.var_q_qcoth_dn7) / (assign20360_e20511 * assign20360_e20511))), (-((locals.var_q_k1q1_dn8 - locals.var_q_qcoth_dn8) / (assign20360_e20511 * assign20360_e20511))), (-((locals.var_q_k1q1_dn9 - locals.var_q_qcoth_dn9) / (assign20360_e20511 * assign20360_e20511))),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign20360_e20514;
        locals.var_q_temp2_dn4 = assign20360_e20514_d_n4;
        locals.var_q_temp2_dn6 = assign20360_e20514_d_n6;
        locals.var_q_temp2_dn7 = assign20360_e20514_d_n7;
        locals.var_q_temp2_dn8 = assign20360_e20514_d_n8;
        locals.var_q_temp2_dn9 = assign20360_e20514_d_n9;
        locals.var_q_temp2_rv = 0.0;

        let (assign20370_e20525, assign20370_e20525_d_n4, assign20370_e20525_d_n6, assign20370_e20525_d_n7, assign20370_e20525_d_n8, assign20370_e20525_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard653 == 0.0)) {
        let assign20370_e20523: f64 = (locals.var_q_d1_qcoth - locals.var_k1);
        (assign20370_e20523, (locals.var_q_d1_qcoth_dn4 - locals.var_k1_dn4), (locals.var_q_d1_qcoth_dn6 - locals.var_k1_dn6), (locals.var_q_d1_qcoth_dn7 - locals.var_k1_dn7), (locals.var_q_d1_qcoth_dn8 - locals.var_k1_dn8), (locals.var_q_d1_qcoth_dn9 - locals.var_k1_dn9),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign20370_e20525;
        locals.var_q_temp3_dn4 = assign20370_e20525_d_n4;
        locals.var_q_temp3_dn6 = assign20370_e20525_d_n6;
        locals.var_q_temp3_dn7 = assign20370_e20525_d_n7;
        locals.var_q_temp3_dn8 = assign20370_e20525_d_n8;
        locals.var_q_temp3_dn9 = assign20370_e20525_d_n9;
        locals.var_q_temp3_rv = 0.0;

        let (assign20380_e20538, assign20380_e20538_d_n4, assign20380_e20538_d_n6, assign20380_e20538_d_n7, assign20380_e20538_d_n8, assign20380_e20538_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard653 == 0.0)) {
        let assign20380_e20534: f64 = (locals.var_q_aexp - locals.var_q_sh_term);
        let assign20380_e20536: f64 = (assign20380_e20534 * locals.var_q_temp2);
        (assign20380_e20536, (((locals.var_q_aexp_dn4 - locals.var_q_sh_term_dn4) * locals.var_q_temp2) + (assign20380_e20534 * locals.var_q_temp2_dn4)), (((locals.var_q_aexp_dn6 - locals.var_q_sh_term_dn6) * locals.var_q_temp2) + (assign20380_e20534 * locals.var_q_temp2_dn6)), (((locals.var_q_aexp_dn7 - locals.var_q_sh_term_dn7) * locals.var_q_temp2) + (assign20380_e20534 * locals.var_q_temp2_dn7)), (((locals.var_q_aexp_dn8 - locals.var_q_sh_term_dn8) * locals.var_q_temp2) + (assign20380_e20534 * locals.var_q_temp2_dn8)), (((locals.var_q_aexp_dn9 - locals.var_q_sh_term_dn9) * locals.var_q_temp2) + (assign20380_e20534 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9,)
    }
};
        locals.var_q_expnum = assign20380_e20538;
        locals.var_q_expnum_dn4 = assign20380_e20538_d_n4;
        locals.var_q_expnum_dn6 = assign20380_e20538_d_n6;
        locals.var_q_expnum_dn7 = assign20380_e20538_d_n7;
        locals.var_q_expnum_dn8 = assign20380_e20538_d_n8;
        locals.var_q_expnum_dn9 = assign20380_e20538_d_n9;
        locals.var_q_expnum_rv = 0.0;

        let (assign20390_e20557, assign20390_e20557_d_n4, assign20390_e20557_d_n6, assign20390_e20557_d_n7, assign20390_e20557_d_n8, assign20390_e20557_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard653 == 0.0)) {
        let assign20390_e20547: f64 = (locals.var_q_temp3 * locals.var_q_expnum);
        let assign20390_e20549: f64 = (assign20390_e20547 - locals.var_q_aexp);
        let assign20390_e20552: f64 = (locals.var_q_d1_ln * locals.var_q_sh_term);
        let assign20390_e20553: f64 = (assign20390_e20549 - assign20390_e20552);
        let assign20390_e20555: f64 = (assign20390_e20553 * locals.var_q_temp2);
        (assign20390_e20555, ((((((locals.var_q_temp3_dn4 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4) - ((locals.var_q_d1_ln_dn4 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign20390_e20553 * locals.var_q_temp2_dn4)), ((((((locals.var_q_temp3_dn6 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6) - ((locals.var_q_d1_ln_dn6 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign20390_e20553 * locals.var_q_temp2_dn6)), ((((((locals.var_q_temp3_dn7 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7) - ((locals.var_q_d1_ln_dn7 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign20390_e20553 * locals.var_q_temp2_dn7)), ((((((locals.var_q_temp3_dn8 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8) - ((locals.var_q_d1_ln_dn8 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign20390_e20553 * locals.var_q_temp2_dn8)), ((((((locals.var_q_temp3_dn9 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9) - ((locals.var_q_d1_ln_dn9 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign20390_e20553 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9,)
    }
};
        locals.var_q_d1_expnum = assign20390_e20557;
        locals.var_q_d1_expnum_dn4 = assign20390_e20557_d_n4;
        locals.var_q_d1_expnum_dn6 = assign20390_e20557_d_n6;
        locals.var_q_d1_expnum_dn7 = assign20390_e20557_d_n7;
        locals.var_q_d1_expnum_dn8 = assign20390_e20557_d_n8;
        locals.var_q_d1_expnum_dn9 = assign20390_e20557_d_n9;
        locals.var_q_d1_expnum_rv = 0.0;

        let (assign20400_e20586, assign20400_e20586_d_n4, assign20400_e20586_d_n6, assign20400_e20586_d_n7, assign20400_e20586_d_n8, assign20400_e20586_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard653 == 0.0)) {
        let assign20400_e20566: f64 = (locals.var_q_d2_qcoth * locals.var_q_expnum);
        let assign20400_e20569: f64 = (2.0 * locals.var_q_temp3);
        let assign20400_e20571: f64 = (assign20400_e20569 * locals.var_q_d1_expnum);
        let assign20400_e20572: f64 = (assign20400_e20566 + assign20400_e20571);
        let assign20400_e20574: f64 = (assign20400_e20572 + locals.var_q_aexp);
        let assign20400_e20578: f64 = (locals.var_q_d1_ln * locals.var_q_d1_ln);
        let assign20400_e20579: f64 = (locals.var_q_d2_ln + assign20400_e20578);
        let assign20400_e20581: f64 = (assign20400_e20579 * locals.var_q_sh_term);
        let assign20400_e20582: f64 = (assign20400_e20574 - assign20400_e20581);
        let assign20400_e20584: f64 = (assign20400_e20582 * locals.var_q_temp2);
        (assign20400_e20584, (((((((locals.var_q_d2_qcoth_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_temp3_dn4) * locals.var_q_d1_expnum) + (assign20400_e20569 * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4) - (((locals.var_q_d2_ln_dn4 + ((locals.var_q_d1_ln_dn4 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn4))) * locals.var_q_sh_term) + (assign20400_e20579 * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign20400_e20582 * locals.var_q_temp2_dn4)), (((((((locals.var_q_d2_qcoth_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_temp3_dn6) * locals.var_q_d1_expnum) + (assign20400_e20569 * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6) - (((locals.var_q_d2_ln_dn6 + ((locals.var_q_d1_ln_dn6 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn6))) * locals.var_q_sh_term) + (assign20400_e20579 * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign20400_e20582 * locals.var_q_temp2_dn6)), (((((((locals.var_q_d2_qcoth_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_temp3_dn7) * locals.var_q_d1_expnum) + (assign20400_e20569 * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7) - (((locals.var_q_d2_ln_dn7 + ((locals.var_q_d1_ln_dn7 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn7))) * locals.var_q_sh_term) + (assign20400_e20579 * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign20400_e20582 * locals.var_q_temp2_dn7)), (((((((locals.var_q_d2_qcoth_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_temp3_dn8) * locals.var_q_d1_expnum) + (assign20400_e20569 * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8) - (((locals.var_q_d2_ln_dn8 + ((locals.var_q_d1_ln_dn8 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn8))) * locals.var_q_sh_term) + (assign20400_e20579 * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign20400_e20582 * locals.var_q_temp2_dn8)), (((((((locals.var_q_d2_qcoth_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_temp3_dn9) * locals.var_q_d1_expnum) + (assign20400_e20569 * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9) - (((locals.var_q_d2_ln_dn9 + ((locals.var_q_d1_ln_dn9 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn9))) * locals.var_q_sh_term) + (assign20400_e20579 * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign20400_e20582 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9,)
    }
};
        locals.var_q_d2_expnum = assign20400_e20586;
        locals.var_q_d2_expnum_dn4 = assign20400_e20586_d_n4;
        locals.var_q_d2_expnum_dn6 = assign20400_e20586_d_n6;
        locals.var_q_d2_expnum_dn7 = assign20400_e20586_d_n7;
        locals.var_q_d2_expnum_dn8 = assign20400_e20586_d_n8;
        locals.var_q_d2_expnum_dn9 = assign20400_e20586_d_n9;
        locals.var_q_d2_expnum_rv = 0.0;

        let assign20410_e20589: f64 = if locals.var_q_expnum > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard654 = assign20410_e20589;
        locals.var_guard654_rv = 0.0;

        let (assign20420_e20598, assign20420_e20598_d_n4, assign20420_e20598_d_n6, assign20420_e20598_d_n7, assign20420_e20598_d_n8, assign20420_e20598_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard654 != 0.0)) {
        let assign20420_e20596: f64 = (locals.var_q_expnum).ln();
        (assign20420_e20596, (locals.var_q_expnum_dn4 / locals.var_q_expnum), (locals.var_q_expnum_dn6 / locals.var_q_expnum), (locals.var_q_expnum_dn7 / locals.var_q_expnum), (locals.var_q_expnum_dn8 / locals.var_q_expnum), (locals.var_q_expnum_dn9 / locals.var_q_expnum),)
    } else {
        (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9,)
    }
};
        locals.var_q_lnexpnum = assign20420_e20598;
        locals.var_q_lnexpnum_dn4 = assign20420_e20598_d_n4;
        locals.var_q_lnexpnum_dn6 = assign20420_e20598_d_n6;
        locals.var_q_lnexpnum_dn7 = assign20420_e20598_d_n7;
        locals.var_q_lnexpnum_dn8 = assign20420_e20598_d_n8;
        locals.var_q_lnexpnum_dn9 = assign20420_e20598_d_n9;
        locals.var_q_lnexpnum_rv = 0.0;

        let (assign20430_e20608, assign20430_e20608_d_n4, assign20430_e20608_d_n6, assign20430_e20608_d_n7, assign20430_e20608_d_n8, assign20430_e20608_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard654 != 0.0)) {
        let assign20430_e20606: f64 = (1.0 / locals.var_q_expnum);
        (assign20430_e20606, (-(locals.var_q_expnum_dn4 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn6 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn7 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn8 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn9 / (locals.var_q_expnum * locals.var_q_expnum))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign20430_e20608;
        locals.var_q_temp1_dn4 = assign20430_e20608_d_n4;
        locals.var_q_temp1_dn6 = assign20430_e20608_d_n6;
        locals.var_q_temp1_dn7 = assign20430_e20608_d_n7;
        locals.var_q_temp1_dn8 = assign20430_e20608_d_n8;
        locals.var_q_temp1_dn9 = assign20430_e20608_d_n9;
        locals.var_q_temp1_rv = 0.0;

        let (assign20440_e20618, assign20440_e20618_d_n4, assign20440_e20618_d_n6, assign20440_e20618_d_n7, assign20440_e20618_d_n8, assign20440_e20618_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard654 != 0.0)) {
        let assign20440_e20616: f64 = (locals.var_q_d1_expnum * locals.var_q_temp1);
        (assign20440_e20616, ((locals.var_q_d1_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn4)), ((locals.var_q_d1_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn6)), ((locals.var_q_d1_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn7)), ((locals.var_q_d1_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn8)), ((locals.var_q_d1_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9,)
    }
};
        locals.var_q_d1_lnexpnum = assign20440_e20618;
        locals.var_q_d1_lnexpnum_dn4 = assign20440_e20618_d_n4;
        locals.var_q_d1_lnexpnum_dn6 = assign20440_e20618_d_n6;
        locals.var_q_d1_lnexpnum_dn7 = assign20440_e20618_d_n7;
        locals.var_q_d1_lnexpnum_dn8 = assign20440_e20618_d_n8;
        locals.var_q_d1_lnexpnum_dn9 = assign20440_e20618_d_n9;
        locals.var_q_d1_lnexpnum_rv = 0.0;

        let (assign20450_e20632, assign20450_e20632_d_n4, assign20450_e20632_d_n6, assign20450_e20632_d_n7, assign20450_e20632_d_n8, assign20450_e20632_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard654 != 0.0)) {
        let assign20450_e20626: f64 = (locals.var_q_d2_expnum * locals.var_q_temp1);
        let assign20450_e20629: f64 = (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum);
        let assign20450_e20630: f64 = (assign20450_e20626 - assign20450_e20629);
        (assign20450_e20630, (((locals.var_q_d2_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn4)) - ((locals.var_q_d1_lnexpnum_dn4 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn4))), (((locals.var_q_d2_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn6)) - ((locals.var_q_d1_lnexpnum_dn6 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn6))), (((locals.var_q_d2_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn7)) - ((locals.var_q_d1_lnexpnum_dn7 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn7))), (((locals.var_q_d2_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn8)) - ((locals.var_q_d1_lnexpnum_dn8 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn8))), (((locals.var_q_d2_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn9)) - ((locals.var_q_d1_lnexpnum_dn9 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn9))),)
    } else {
        (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9,)
    }
};
        locals.var_q_d2_lnexpnum = assign20450_e20632;
        locals.var_q_d2_lnexpnum_dn4 = assign20450_e20632_d_n4;
        locals.var_q_d2_lnexpnum_dn6 = assign20450_e20632_d_n6;
        locals.var_q_d2_lnexpnum_dn7 = assign20450_e20632_d_n7;
        locals.var_q_d2_lnexpnum_dn8 = assign20450_e20632_d_n8;
        locals.var_q_d2_lnexpnum_dn9 = assign20450_e20632_d_n9;
        locals.var_q_d2_lnexpnum_rv = 0.0;

        let (assign20460_e20647, assign20460_e20647_d_n4, assign20460_e20647_d_n6, assign20460_e20647_d_n7, assign20460_e20647_d_n8, assign20460_e20647_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard654 == 0.0)) {
        let assign20460_e20641: f64 = (locals.var_q_k1q1 + 0.6931471805599);
        let assign20460_e20643: f64 = (-locals.var_q_k1q1);
        let assign20460_e20644: f64 = (assign20460_e20643).ln();
        let assign20460_e20645: f64 = (assign20460_e20641 + assign20460_e20644);
        (assign20460_e20645, (locals.var_q_k1q1_dn4 + ((-locals.var_q_k1q1_dn4) / assign20460_e20643)), (locals.var_q_k1q1_dn6 + ((-locals.var_q_k1q1_dn6) / assign20460_e20643)), (locals.var_q_k1q1_dn7 + ((-locals.var_q_k1q1_dn7) / assign20460_e20643)), (locals.var_q_k1q1_dn8 + ((-locals.var_q_k1q1_dn8) / assign20460_e20643)), (locals.var_q_k1q1_dn9 + ((-locals.var_q_k1q1_dn9) / assign20460_e20643)),)
    } else {
        (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9,)
    }
};
        locals.var_q_lnexpnum = assign20460_e20647;
        locals.var_q_lnexpnum_dn4 = assign20460_e20647_d_n4;
        locals.var_q_lnexpnum_dn6 = assign20460_e20647_d_n6;
        locals.var_q_lnexpnum_dn7 = assign20460_e20647_d_n7;
        locals.var_q_lnexpnum_dn8 = assign20460_e20647_d_n8;
        locals.var_q_lnexpnum_dn9 = assign20460_e20647_d_n9;
        locals.var_q_lnexpnum_rv = 0.0;

        let (assign20470_e20658, assign20470_e20658_d_n4, assign20470_e20658_d_n6, assign20470_e20658_d_n7, assign20470_e20658_d_n8, assign20470_e20658_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard654 == 0.0)) {
        let assign20470_e20656: f64 = (1.0 / locals.var_q1d);
        (assign20470_e20656, (-(locals.var_q1d_dn4 / (locals.var_q1d * locals.var_q1d))), (-(locals.var_q1d_dn6 / (locals.var_q1d * locals.var_q1d))), (-(locals.var_q1d_dn7 / (locals.var_q1d * locals.var_q1d))), (-(locals.var_q1d_dn8 / (locals.var_q1d * locals.var_q1d))), (-(locals.var_q1d_dn9 / (locals.var_q1d * locals.var_q1d))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign20470_e20658;
        locals.var_q_temp1_dn4 = assign20470_e20658_d_n4;
        locals.var_q_temp1_dn6 = assign20470_e20658_d_n6;
        locals.var_q_temp1_dn7 = assign20470_e20658_d_n7;
        locals.var_q_temp1_dn8 = assign20470_e20658_d_n8;
        locals.var_q_temp1_dn9 = assign20470_e20658_d_n9;
        locals.var_q_temp1_rv = 0.0;

        let (assign20480_e20669, assign20480_e20669_d_n4, assign20480_e20669_d_n6, assign20480_e20669_d_n7, assign20480_e20669_d_n8, assign20480_e20669_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard654 == 0.0)) {
        let assign20480_e20667: f64 = (locals.var_k1 + locals.var_q_temp1);
        (assign20480_e20667, (locals.var_k1_dn4 + locals.var_q_temp1_dn4), (locals.var_k1_dn6 + locals.var_q_temp1_dn6), (locals.var_k1_dn7 + locals.var_q_temp1_dn7), (locals.var_k1_dn8 + locals.var_q_temp1_dn8), (locals.var_k1_dn9 + locals.var_q_temp1_dn9),)
    } else {
        (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9,)
    }
};
        locals.var_q_d1_lnexpnum = assign20480_e20669;
        locals.var_q_d1_lnexpnum_dn4 = assign20480_e20669_d_n4;
        locals.var_q_d1_lnexpnum_dn6 = assign20480_e20669_d_n6;
        locals.var_q_d1_lnexpnum_dn7 = assign20480_e20669_d_n7;
        locals.var_q_d1_lnexpnum_dn8 = assign20480_e20669_d_n8;
        locals.var_q_d1_lnexpnum_dn9 = assign20480_e20669_d_n9;
        locals.var_q_d1_lnexpnum_rv = 0.0;

        let (assign20490_e20681, assign20490_e20681_d_n4, assign20490_e20681_d_n6, assign20490_e20681_d_n7, assign20490_e20681_d_n8, assign20490_e20681_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard654 == 0.0)) {
        let assign20490_e20677: f64 = (-locals.var_q_temp1);
        let assign20490_e20679: f64 = (assign20490_e20677 * locals.var_q_temp1);
        (assign20490_e20679, (((-locals.var_q_temp1_dn4) * locals.var_q_temp1) + (assign20490_e20677 * locals.var_q_temp1_dn4)), (((-locals.var_q_temp1_dn6) * locals.var_q_temp1) + (assign20490_e20677 * locals.var_q_temp1_dn6)), (((-locals.var_q_temp1_dn7) * locals.var_q_temp1) + (assign20490_e20677 * locals.var_q_temp1_dn7)), (((-locals.var_q_temp1_dn8) * locals.var_q_temp1) + (assign20490_e20677 * locals.var_q_temp1_dn8)), (((-locals.var_q_temp1_dn9) * locals.var_q_temp1) + (assign20490_e20677 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9,)
    }
};
        locals.var_q_d2_lnexpnum = assign20490_e20681;
        locals.var_q_d2_lnexpnum_dn4 = assign20490_e20681_d_n4;
        locals.var_q_d2_lnexpnum_dn6 = assign20490_e20681_d_n6;
        locals.var_q_d2_lnexpnum_dn7 = assign20490_e20681_d_n7;
        locals.var_q_d2_lnexpnum_dn8 = assign20490_e20681_d_n8;
        locals.var_q_d2_lnexpnum_dn9 = assign20490_e20681_d_n9;
        locals.var_q_d2_lnexpnum_rv = 0.0;

        let (assign20500_e20697, assign20500_e20697_d_n4, assign20500_e20697_d_n6, assign20500_e20697_d_n7, assign20500_e20697_d_n8, assign20500_e20697_d_n9,) = {
    if ((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) {
        let assign20500_e20687: f64 = (locals.var_xg2x - locals.var_xg1x);
        let assign20500_e20689: f64 = (assign20500_e20687 + locals.var_q1d);
        let assign20500_e20692: f64 = (2.0 * locals.var_q_lnexpnum);
        let assign20500_e20693: f64 = (assign20500_e20689 + assign20500_e20692);
        let assign20500_e20695: f64 = (assign20500_e20693 - locals.var_q_ln_term);
        (assign20500_e20695, ((((locals.var_xg2x_dn4 - locals.var_xg1x_dn4) + locals.var_q1d_dn4) + (2.0 * locals.var_q_lnexpnum_dn4)) - locals.var_q_ln_term_dn4), ((((locals.var_xg2x_dn6 - locals.var_xg1x_dn6) + locals.var_q1d_dn6) + (2.0 * locals.var_q_lnexpnum_dn6)) - locals.var_q_ln_term_dn6), ((((locals.var_xg2x_dn7 - locals.var_xg1x_dn7) + locals.var_q1d_dn7) + (2.0 * locals.var_q_lnexpnum_dn7)) - locals.var_q_ln_term_dn7), ((((locals.var_xg2x_dn8 - locals.var_xg1x_dn8) + locals.var_q1d_dn8) + (2.0 * locals.var_q_lnexpnum_dn8)) - locals.var_q_ln_term_dn8), ((((locals.var_xg2x_dn9 - locals.var_xg1x_dn9) + locals.var_q1d_dn9) + (2.0 * locals.var_q_lnexpnum_dn9)) - locals.var_q_ln_term_dn9),)
    } else {
        (locals.var_q_q2_int, locals.var_q_q2_int_dn4, locals.var_q_q2_int_dn6, locals.var_q_q2_int_dn7, locals.var_q_q2_int_dn8, locals.var_q_q2_int_dn9,)
    }
};
        locals.var_q_q2_int = assign20500_e20697;
        locals.var_q_q2_int_dn4 = assign20500_e20697_d_n4;
        locals.var_q_q2_int_dn6 = assign20500_e20697_d_n6;
        locals.var_q_q2_int_dn7 = assign20500_e20697_d_n7;
        locals.var_q_q2_int_dn8 = assign20500_e20697_d_n8;
        locals.var_q_q2_int_dn9 = assign20500_e20697_d_n9;
        locals.var_q_q2_int_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_56(
        locals: &mut StampLocals,
    ) {
        let (assign20510_e20709, assign20510_e20709_d_n4, assign20510_e20709_d_n6, assign20510_e20709_d_n7, assign20510_e20709_d_n8, assign20510_e20709_d_n9,) = {
    if ((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) {
        let assign20510_e20704: f64 = (2.0 * locals.var_q_d1_lnexpnum);
        let assign20510_e20705: f64 = (1.0 + assign20510_e20704);
        let assign20510_e20707: f64 = (assign20510_e20705 - locals.var_q_d1_ln);
        (assign20510_e20707, ((2.0 * locals.var_q_d1_lnexpnum_dn4) - locals.var_q_d1_ln_dn4), ((2.0 * locals.var_q_d1_lnexpnum_dn6) - locals.var_q_d1_ln_dn6), ((2.0 * locals.var_q_d1_lnexpnum_dn7) - locals.var_q_d1_ln_dn7), ((2.0 * locals.var_q_d1_lnexpnum_dn8) - locals.var_q_d1_ln_dn8), ((2.0 * locals.var_q_d1_lnexpnum_dn9) - locals.var_q_d1_ln_dn9),)
    } else {
        (locals.var_q_d1_q2, locals.var_q_d1_q2_dn4, locals.var_q_d1_q2_dn6, locals.var_q_d1_q2_dn7, locals.var_q_d1_q2_dn8, locals.var_q_d1_q2_dn9,)
    }
};
        locals.var_q_d1_q2 = assign20510_e20709;
        locals.var_q_d1_q2_dn4 = assign20510_e20709_d_n4;
        locals.var_q_d1_q2_dn6 = assign20510_e20709_d_n6;
        locals.var_q_d1_q2_dn7 = assign20510_e20709_d_n7;
        locals.var_q_d1_q2_dn8 = assign20510_e20709_d_n8;
        locals.var_q_d1_q2_dn9 = assign20510_e20709_d_n9;
        locals.var_q_d1_q2_rv = 0.0;

        let (assign20520_e20719, assign20520_e20719_d_n4, assign20520_e20719_d_n6, assign20520_e20719_d_n7, assign20520_e20719_d_n8, assign20520_e20719_d_n9,) = {
    if ((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) {
        let assign20520_e20715: f64 = (2.0 * locals.var_q_d2_lnexpnum);
        let assign20520_e20717: f64 = (assign20520_e20715 - locals.var_q_d2_ln);
        (assign20520_e20717, ((2.0 * locals.var_q_d2_lnexpnum_dn4) - locals.var_q_d2_ln_dn4), ((2.0 * locals.var_q_d2_lnexpnum_dn6) - locals.var_q_d2_ln_dn6), ((2.0 * locals.var_q_d2_lnexpnum_dn7) - locals.var_q_d2_ln_dn7), ((2.0 * locals.var_q_d2_lnexpnum_dn8) - locals.var_q_d2_ln_dn8), ((2.0 * locals.var_q_d2_lnexpnum_dn9) - locals.var_q_d2_ln_dn9),)
    } else {
        (locals.var_q_d2_q2, locals.var_q_d2_q2_dn4, locals.var_q_d2_q2_dn6, locals.var_q_d2_q2_dn7, locals.var_q_d2_q2_dn8, locals.var_q_d2_q2_dn9,)
    }
};
        locals.var_q_d2_q2 = assign20520_e20719;
        locals.var_q_d2_q2_dn4 = assign20520_e20719_d_n4;
        locals.var_q_d2_q2_dn6 = assign20520_e20719_d_n6;
        locals.var_q_d2_q2_dn7 = assign20520_e20719_d_n7;
        locals.var_q_d2_q2_dn8 = assign20520_e20719_d_n8;
        locals.var_q_d2_q2_dn9 = assign20520_e20719_d_n9;
        locals.var_q_d2_q2_rv = 0.0;

        let (assign20530_e20729, assign20530_e20729_d_n4, assign20530_e20729_d_n6, assign20530_e20729_d_n7, assign20530_e20729_d_n8, assign20530_e20729_d_n9,) = {
    if ((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) {
        let assign20530_e20726: f64 = (locals.var_k2 * locals.var_q_q2_int);
        let assign20530_e20727: f64 = (locals.var_q_k1q1 + assign20530_e20726);
        (assign20530_e20727, (locals.var_q_k1q1_dn4 + ((locals.var_k2_dn4 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn4))), (locals.var_q_k1q1_dn6 + ((locals.var_k2_dn6 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn6))), (locals.var_q_k1q1_dn7 + ((locals.var_k2_dn7 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn7))), (locals.var_q_k1q1_dn8 + ((locals.var_k2_dn8 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn8))), (locals.var_q_k1q1_dn9 + ((locals.var_k2_dn9 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn9))),)
    } else {
        (locals.var_q_qi_int, locals.var_q_qi_int_dn4, locals.var_q_qi_int_dn6, locals.var_q_qi_int_dn7, locals.var_q_qi_int_dn8, locals.var_q_qi_int_dn9,)
    }
};
        locals.var_q_qi_int = assign20530_e20729;
        locals.var_q_qi_int_dn4 = assign20530_e20729_d_n4;
        locals.var_q_qi_int_dn6 = assign20530_e20729_d_n6;
        locals.var_q_qi_int_dn7 = assign20530_e20729_d_n7;
        locals.var_q_qi_int_dn8 = assign20530_e20729_d_n8;
        locals.var_q_qi_int_dn9 = assign20530_e20729_d_n9;
        locals.var_q_qi_int_rv = 0.0;

        let (assign20540_e20739, assign20540_e20739_d_n4, assign20540_e20739_d_n6, assign20540_e20739_d_n7, assign20540_e20739_d_n8, assign20540_e20739_d_n9,) = {
    if ((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) {
        let assign20540_e20736: f64 = (locals.var_k2 * locals.var_q_d1_q2);
        let assign20540_e20737: f64 = (locals.var_k1 + assign20540_e20736);
        (assign20540_e20737, (locals.var_k1_dn4 + ((locals.var_k2_dn4 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn4))), (locals.var_k1_dn6 + ((locals.var_k2_dn6 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn6))), (locals.var_k1_dn7 + ((locals.var_k2_dn7 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn7))), (locals.var_k1_dn8 + ((locals.var_k2_dn8 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn8))), (locals.var_k1_dn9 + ((locals.var_k2_dn9 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn9))),)
    } else {
        (locals.var_q_d1_qi, locals.var_q_d1_qi_dn4, locals.var_q_d1_qi_dn6, locals.var_q_d1_qi_dn7, locals.var_q_d1_qi_dn8, locals.var_q_d1_qi_dn9,)
    }
};
        locals.var_q_d1_qi = assign20540_e20739;
        locals.var_q_d1_qi_dn4 = assign20540_e20739_d_n4;
        locals.var_q_d1_qi_dn6 = assign20540_e20739_d_n6;
        locals.var_q_d1_qi_dn7 = assign20540_e20739_d_n7;
        locals.var_q_d1_qi_dn8 = assign20540_e20739_d_n8;
        locals.var_q_d1_qi_dn9 = assign20540_e20739_d_n9;
        locals.var_q_d1_qi_rv = 0.0;

        let (assign20550_e20747, assign20550_e20747_d_n4, assign20550_e20747_d_n6, assign20550_e20747_d_n7, assign20550_e20747_d_n8, assign20550_e20747_d_n9,) = {
    if ((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) {
        let assign20550_e20745: f64 = (locals.var_k2 * locals.var_q_d2_q2);
        (assign20550_e20745, ((locals.var_k2_dn4 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn4)), ((locals.var_k2_dn6 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn6)), ((locals.var_k2_dn7 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn7)), ((locals.var_k2_dn8 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn8)), ((locals.var_k2_dn9 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn9)),)
    } else {
        (locals.var_q_d2_qi, locals.var_q_d2_qi_dn4, locals.var_q_d2_qi_dn6, locals.var_q_d2_qi_dn7, locals.var_q_d2_qi_dn8, locals.var_q_d2_qi_dn9,)
    }
};
        locals.var_q_d2_qi = assign20550_e20747;
        locals.var_q_d2_qi_dn4 = assign20550_e20747_d_n4;
        locals.var_q_d2_qi_dn6 = assign20550_e20747_d_n6;
        locals.var_q_d2_qi_dn7 = assign20550_e20747_d_n7;
        locals.var_q_d2_qi_dn8 = assign20550_e20747_d_n8;
        locals.var_q_d2_qi_dn9 = assign20550_e20747_d_n9;
        locals.var_q_d2_qi_rv = 0.0;

        let (assign20560_e20757, assign20560_e20757_d_n4, assign20560_e20757_d_n6, assign20560_e20757_d_n7, assign20560_e20757_d_n8, assign20560_e20757_d_n9,) = {
    if ((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) {
        let assign20560_e20753: f64 = (locals.var_q_qi_int * locals.var_q_expnum);
        let assign20560_e20755: f64 = (assign20560_e20753 - locals.var_q_aexp);
        (assign20560_e20755, (((locals.var_q_qi_int_dn4 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4), (((locals.var_q_qi_int_dn6 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6), (((locals.var_q_qi_int_dn7 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7), (((locals.var_q_qi_int_dn8 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8), (((locals.var_q_qi_int_dn9 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9),)
    } else {
        (locals.var_q_zero, locals.var_q_zero_dn4, locals.var_q_zero_dn6, locals.var_q_zero_dn7, locals.var_q_zero_dn8, locals.var_q_zero_dn9,)
    }
};
        locals.var_q_zero = assign20560_e20757;
        locals.var_q_zero_dn4 = assign20560_e20757_d_n4;
        locals.var_q_zero_dn6 = assign20560_e20757_d_n6;
        locals.var_q_zero_dn7 = assign20560_e20757_d_n7;
        locals.var_q_zero_dn8 = assign20560_e20757_d_n8;
        locals.var_q_zero_dn9 = assign20560_e20757_d_n9;
        locals.var_q_zero_rv = 0.0;

        let (assign20570_e20771, assign20570_e20771_d_n4, assign20570_e20771_d_n6, assign20570_e20771_d_n7, assign20570_e20771_d_n8, assign20570_e20771_d_n9,) = {
    if ((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) {
        let assign20570_e20763: f64 = (locals.var_q_d1_qi * locals.var_q_expnum);
        let assign20570_e20766: f64 = (locals.var_q_qi_int * locals.var_q_d1_expnum);
        let assign20570_e20767: f64 = (assign20570_e20763 + assign20570_e20766);
        let assign20570_e20769: f64 = (assign20570_e20767 + locals.var_q_aexp);
        (assign20570_e20769, ((((locals.var_q_d1_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn4)) + ((locals.var_q_qi_int_dn4 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4), ((((locals.var_q_d1_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn6)) + ((locals.var_q_qi_int_dn6 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6), ((((locals.var_q_d1_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn7)) + ((locals.var_q_qi_int_dn7 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7), ((((locals.var_q_d1_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn8)) + ((locals.var_q_qi_int_dn8 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8), ((((locals.var_q_d1_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn9)) + ((locals.var_q_qi_int_dn9 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9),)
    } else {
        (locals.var_q_d1_zero, locals.var_q_d1_zero_dn4, locals.var_q_d1_zero_dn6, locals.var_q_d1_zero_dn7, locals.var_q_d1_zero_dn8, locals.var_q_d1_zero_dn9,)
    }
};
        locals.var_q_d1_zero = assign20570_e20771;
        locals.var_q_d1_zero_dn4 = assign20570_e20771_d_n4;
        locals.var_q_d1_zero_dn6 = assign20570_e20771_d_n6;
        locals.var_q_d1_zero_dn7 = assign20570_e20771_d_n7;
        locals.var_q_d1_zero_dn8 = assign20570_e20771_d_n8;
        locals.var_q_d1_zero_dn9 = assign20570_e20771_d_n9;
        locals.var_q_d1_zero_rv = 0.0;

        let (assign20580_e20791, assign20580_e20791_d_n4, assign20580_e20791_d_n6, assign20580_e20791_d_n7, assign20580_e20791_d_n8, assign20580_e20791_d_n9,) = {
    if ((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) {
        let assign20580_e20777: f64 = (locals.var_q_d2_qi * locals.var_q_expnum);
        let assign20580_e20780: f64 = (2.0 * locals.var_q_d1_qi);
        let assign20580_e20782: f64 = (assign20580_e20780 * locals.var_q_d1_expnum);
        let assign20580_e20783: f64 = (assign20580_e20777 + assign20580_e20782);
        let assign20580_e20786: f64 = (locals.var_q_qi_int * locals.var_q_d2_expnum);
        let assign20580_e20787: f64 = (assign20580_e20783 + assign20580_e20786);
        let assign20580_e20789: f64 = (assign20580_e20787 - locals.var_q_aexp);
        (assign20580_e20789, (((((locals.var_q_d2_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_d1_qi_dn4) * locals.var_q_d1_expnum) + (assign20580_e20780 * locals.var_q_d1_expnum_dn4))) + ((locals.var_q_qi_int_dn4 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn4))) - locals.var_q_aexp_dn4), (((((locals.var_q_d2_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_d1_qi_dn6) * locals.var_q_d1_expnum) + (assign20580_e20780 * locals.var_q_d1_expnum_dn6))) + ((locals.var_q_qi_int_dn6 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn6))) - locals.var_q_aexp_dn6), (((((locals.var_q_d2_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_d1_qi_dn7) * locals.var_q_d1_expnum) + (assign20580_e20780 * locals.var_q_d1_expnum_dn7))) + ((locals.var_q_qi_int_dn7 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn7))) - locals.var_q_aexp_dn7), (((((locals.var_q_d2_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_d1_qi_dn8) * locals.var_q_d1_expnum) + (assign20580_e20780 * locals.var_q_d1_expnum_dn8))) + ((locals.var_q_qi_int_dn8 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn8))) - locals.var_q_aexp_dn8), (((((locals.var_q_d2_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_d1_qi_dn9) * locals.var_q_d1_expnum) + (assign20580_e20780 * locals.var_q_d1_expnum_dn9))) + ((locals.var_q_qi_int_dn9 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn9))) - locals.var_q_aexp_dn9),)
    } else {
        (locals.var_q_d2_zero, locals.var_q_d2_zero_dn4, locals.var_q_d2_zero_dn6, locals.var_q_d2_zero_dn7, locals.var_q_d2_zero_dn8, locals.var_q_d2_zero_dn9,)
    }
};
        locals.var_q_d2_zero = assign20580_e20791;
        locals.var_q_d2_zero_dn4 = assign20580_e20791_d_n4;
        locals.var_q_d2_zero_dn6 = assign20580_e20791_d_n6;
        locals.var_q_d2_zero_dn7 = assign20580_e20791_d_n7;
        locals.var_q_d2_zero_dn8 = assign20580_e20791_d_n8;
        locals.var_q_d2_zero_dn9 = assign20580_e20791_d_n9;
        locals.var_q_d2_zero_rv = 0.0;

        let (assign20590_e20805, assign20590_e20805_d_n4, assign20590_e20805_d_n6, assign20590_e20805_d_n7, assign20590_e20805_d_n8, assign20590_e20805_d_n9,) = {
    if ((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) {
        let assign20590_e20797: f64 = (locals.var_q_d1_zero * locals.var_q_d1_zero);
        let assign20590_e20800: f64 = (0.5 * locals.var_q_zero);
        let assign20590_e20802: f64 = (assign20590_e20800 * locals.var_q_d2_zero);
        let assign20590_e20803: f64 = (assign20590_e20797 - assign20590_e20802);
        (assign20590_e20803, (((locals.var_q_d1_zero_dn4 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn4)) - (((0.5 * locals.var_q_zero_dn4) * locals.var_q_d2_zero) + (assign20590_e20800 * locals.var_q_d2_zero_dn4))), (((locals.var_q_d1_zero_dn6 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn6)) - (((0.5 * locals.var_q_zero_dn6) * locals.var_q_d2_zero) + (assign20590_e20800 * locals.var_q_d2_zero_dn6))), (((locals.var_q_d1_zero_dn7 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn7)) - (((0.5 * locals.var_q_zero_dn7) * locals.var_q_d2_zero) + (assign20590_e20800 * locals.var_q_d2_zero_dn7))), (((locals.var_q_d1_zero_dn8 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn8)) - (((0.5 * locals.var_q_zero_dn8) * locals.var_q_d2_zero) + (assign20590_e20800 * locals.var_q_d2_zero_dn8))), (((locals.var_q_d1_zero_dn9 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn9)) - (((0.5 * locals.var_q_zero_dn9) * locals.var_q_d2_zero) + (assign20590_e20800 * locals.var_q_d2_zero_dn9))),)
    } else {
        (locals.var_q_temp, locals.var_q_temp_dn4, locals.var_q_temp_dn6, locals.var_q_temp_dn7, locals.var_q_temp_dn8, locals.var_q_temp_dn9,)
    }
};
        locals.var_q_temp = assign20590_e20805;
        locals.var_q_temp_dn4 = assign20590_e20805_d_n4;
        locals.var_q_temp_dn6 = assign20590_e20805_d_n6;
        locals.var_q_temp_dn7 = assign20590_e20805_d_n7;
        locals.var_q_temp_dn8 = assign20590_e20805_d_n8;
        locals.var_q_temp_dn9 = assign20590_e20805_d_n9;
        locals.var_q_temp_rv = 0.0;

        let (assign20600_e20822, assign20600_e20822_d_n4, assign20600_e20822_d_n6, assign20600_e20822_d_n7, assign20600_e20822_d_n8, assign20600_e20822_d_n9,) = {
    if ((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) {
        let assign20600_e20810: f64 = (-locals.var_q_zero);
        let assign20600_e20812: f64 = (assign20600_e20810 * locals.var_q_d1_zero);
        let assign20600_e20814: f64 = (assign20600_e20812 * locals.var_q_temp);
        let assign20600_e20817: f64 = (locals.var_q_temp * locals.var_q_temp);
        let assign20600_e20819: f64 = (assign20600_e20817 + 1e-200);
        let assign20600_e20820: f64 = (assign20600_e20814 / assign20600_e20819);
        (assign20600_e20820, ((((((((-locals.var_q_zero_dn4) * locals.var_q_d1_zero) + (assign20600_e20810 * locals.var_q_d1_zero_dn4)) * locals.var_q_temp) + (assign20600_e20812 * locals.var_q_temp_dn4)) * assign20600_e20819) - (assign20600_e20814 * ((locals.var_q_temp_dn4 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn4)))) / (assign20600_e20819 * assign20600_e20819)), ((((((((-locals.var_q_zero_dn6) * locals.var_q_d1_zero) + (assign20600_e20810 * locals.var_q_d1_zero_dn6)) * locals.var_q_temp) + (assign20600_e20812 * locals.var_q_temp_dn6)) * assign20600_e20819) - (assign20600_e20814 * ((locals.var_q_temp_dn6 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn6)))) / (assign20600_e20819 * assign20600_e20819)), ((((((((-locals.var_q_zero_dn7) * locals.var_q_d1_zero) + (assign20600_e20810 * locals.var_q_d1_zero_dn7)) * locals.var_q_temp) + (assign20600_e20812 * locals.var_q_temp_dn7)) * assign20600_e20819) - (assign20600_e20814 * ((locals.var_q_temp_dn7 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn7)))) / (assign20600_e20819 * assign20600_e20819)), ((((((((-locals.var_q_zero_dn8) * locals.var_q_d1_zero) + (assign20600_e20810 * locals.var_q_d1_zero_dn8)) * locals.var_q_temp) + (assign20600_e20812 * locals.var_q_temp_dn8)) * assign20600_e20819) - (assign20600_e20814 * ((locals.var_q_temp_dn8 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn8)))) / (assign20600_e20819 * assign20600_e20819)), ((((((((-locals.var_q_zero_dn9) * locals.var_q_d1_zero) + (assign20600_e20810 * locals.var_q_d1_zero_dn9)) * locals.var_q_temp) + (assign20600_e20812 * locals.var_q_temp_dn9)) * assign20600_e20819) - (assign20600_e20814 * ((locals.var_q_temp_dn9 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn9)))) / (assign20600_e20819 * assign20600_e20819)),)
    } else {
        (locals.var_q_eps2, locals.var_q_eps2_dn4, locals.var_q_eps2_dn6, locals.var_q_eps2_dn7, locals.var_q_eps2_dn8, locals.var_q_eps2_dn9,)
    }
};
        locals.var_q_eps2 = assign20600_e20822;
        locals.var_q_eps2_dn4 = assign20600_e20822_d_n4;
        locals.var_q_eps2_dn6 = assign20600_e20822_d_n6;
        locals.var_q_eps2_dn7 = assign20600_e20822_d_n7;
        locals.var_q_eps2_dn8 = assign20600_e20822_d_n8;
        locals.var_q_eps2_dn9 = assign20600_e20822_d_n9;
        locals.var_q_eps2_rv = 0.0;

        let (assign20610_e20830, assign20610_e20830_d_n4, assign20610_e20830_d_n6, assign20610_e20830_d_n7, assign20610_e20830_d_n8, assign20610_e20830_d_n9,) = {
    if ((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) {
        let assign20610_e20828: f64 = (locals.var_q1d + locals.var_q_eps2);
        (assign20610_e20828, (locals.var_q1d_dn4 + locals.var_q_eps2_dn4), (locals.var_q1d_dn6 + locals.var_q_eps2_dn6), (locals.var_q1d_dn7 + locals.var_q_eps2_dn7), (locals.var_q1d_dn8 + locals.var_q_eps2_dn8), (locals.var_q1d_dn9 + locals.var_q_eps2_dn9),)
    } else {
        (locals.var_q1d, locals.var_q1d_dn4, locals.var_q1d_dn6, locals.var_q1d_dn7, locals.var_q1d_dn8, locals.var_q1d_dn9,)
    }
};
        locals.var_q1d = assign20610_e20830;
        locals.var_q1d_dn4 = assign20610_e20830_d_n4;
        locals.var_q1d_dn6 = assign20610_e20830_d_n6;
        locals.var_q1d_dn7 = assign20610_e20830_d_n7;
        locals.var_q1d_dn8 = assign20610_e20830_d_n8;
        locals.var_q1d_dn9 = assign20610_e20830_d_n9;
        locals.var_q1d_rv = 0.0;

        let assign20620_e20833: f64 = (locals.var_k1 * locals.var_q1d);
        locals.var_k1q1d = assign20620_e20833;
        locals.var_k1q1d_dn4 = ((locals.var_k1_dn4 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn4));
        locals.var_k1q1d_dn6 = ((locals.var_k1_dn6 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn6));
        locals.var_k1q1d_dn7 = ((locals.var_k1_dn7 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn7));
        locals.var_k1q1d_dn8 = ((locals.var_k1_dn8 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn8));
        locals.var_k1q1d_dn9 = ((locals.var_k1_dn9 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn9));
        locals.var_k1q1d_rv = 0.0;

        let assign20630_e20836: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign20630_e20838: f64 = (assign20630_e20836 - locals.var_xdeff);
        let assign20630_e20840: f64 = if assign20630_e20838 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard655 = assign20630_e20840;
        locals.var_guard655_rv = 0.0;

        let (assign20640_e20849, assign20640_e20849_d_n4, assign20640_e20849_d_n6, assign20640_e20849_d_n7, assign20640_e20849_d_n8, assign20640_e20849_d_n9,) = {
    if (locals.var_guard655 != 0.0) {
        let assign20640_e20844: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign20640_e20846: f64 = (assign20640_e20844 - locals.var_xdeff);
        let assign20640_e20847: f64 = (assign20640_e20846).exp();
        (assign20640_e20847, (assign20640_e20847 * ((locals.var_xg1x_dn4 - locals.var_q1d_dn4) - locals.var_xdeff_dn4)), (assign20640_e20847 * ((locals.var_xg1x_dn6 - locals.var_q1d_dn6) - locals.var_xdeff_dn6)), (assign20640_e20847 * ((locals.var_xg1x_dn7 - locals.var_q1d_dn7) - locals.var_xdeff_dn7)), (assign20640_e20847 * ((locals.var_xg1x_dn8 - locals.var_q1d_dn8) - locals.var_xdeff_dn8)), (assign20640_e20847 * ((locals.var_xg1x_dn9 - locals.var_q1d_dn9) - locals.var_xdeff_dn9)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign20640_e20849;
        locals.var_q_temp1_dn4 = assign20640_e20849_d_n4;
        locals.var_q_temp1_dn6 = assign20640_e20849_d_n6;
        locals.var_q_temp1_dn7 = assign20640_e20849_d_n7;
        locals.var_q_temp1_dn8 = assign20640_e20849_d_n8;
        locals.var_q_temp1_dn9 = assign20640_e20849_d_n9;
        locals.var_q_temp1_rv = 0.0;

        let (assign20650_e20888, assign20650_e20888_d_n4, assign20650_e20888_d_n6, assign20650_e20888_d_n7, assign20650_e20888_d_n8, assign20650_e20888_d_n9,) = {
    if (locals.var_guard655 == 0.0) {
        let assign20650_e20856: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign20650_e20858: f64 = (assign20650_e20856 - locals.var_xdeff);
        let assign20650_e20860: f64 = (assign20650_e20858 - 80.0);
        let assign20650_e20865: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign20650_e20867: f64 = (assign20650_e20865 - locals.var_xdeff);
        let assign20650_e20869: f64 = (assign20650_e20867 - 80.0);
        let assign20650_e20870: f64 = (0.5 * assign20650_e20869);
        let assign20650_e20874: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign20650_e20876: f64 = (assign20650_e20874 - locals.var_xdeff);
        let assign20650_e20878: f64 = (assign20650_e20876 - 80.0);
        let assign20650_e20880: f64 = (assign20650_e20878 * 0.3333333333333);
        let assign20650_e20881: f64 = (1.0 + assign20650_e20880);
        let assign20650_e20882: f64 = (assign20650_e20870 * assign20650_e20881);
        let assign20650_e20883: f64 = (1.0 + assign20650_e20882);
        let assign20650_e20884: f64 = (assign20650_e20860 * assign20650_e20883);
        let assign20650_e20885: f64 = (1.0 + assign20650_e20884);
        let assign20650_e20886: f64 = (5.54062e34 * assign20650_e20885);
        (assign20650_e20886, (5.54062e34 * ((((locals.var_xg1x_dn4 - locals.var_q1d_dn4) - locals.var_xdeff_dn4) * assign20650_e20883) + (assign20650_e20860 * (((0.5 * ((locals.var_xg1x_dn4 - locals.var_q1d_dn4) - locals.var_xdeff_dn4)) * assign20650_e20881) + (assign20650_e20870 * (((locals.var_xg1x_dn4 - locals.var_q1d_dn4) - locals.var_xdeff_dn4) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x_dn6 - locals.var_q1d_dn6) - locals.var_xdeff_dn6) * assign20650_e20883) + (assign20650_e20860 * (((0.5 * ((locals.var_xg1x_dn6 - locals.var_q1d_dn6) - locals.var_xdeff_dn6)) * assign20650_e20881) + (assign20650_e20870 * (((locals.var_xg1x_dn6 - locals.var_q1d_dn6) - locals.var_xdeff_dn6) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x_dn7 - locals.var_q1d_dn7) - locals.var_xdeff_dn7) * assign20650_e20883) + (assign20650_e20860 * (((0.5 * ((locals.var_xg1x_dn7 - locals.var_q1d_dn7) - locals.var_xdeff_dn7)) * assign20650_e20881) + (assign20650_e20870 * (((locals.var_xg1x_dn7 - locals.var_q1d_dn7) - locals.var_xdeff_dn7) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x_dn8 - locals.var_q1d_dn8) - locals.var_xdeff_dn8) * assign20650_e20883) + (assign20650_e20860 * (((0.5 * ((locals.var_xg1x_dn8 - locals.var_q1d_dn8) - locals.var_xdeff_dn8)) * assign20650_e20881) + (assign20650_e20870 * (((locals.var_xg1x_dn8 - locals.var_q1d_dn8) - locals.var_xdeff_dn8) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x_dn9 - locals.var_q1d_dn9) - locals.var_xdeff_dn9) * assign20650_e20883) + (assign20650_e20860 * (((0.5 * ((locals.var_xg1x_dn9 - locals.var_q1d_dn9) - locals.var_xdeff_dn9)) * assign20650_e20881) + (assign20650_e20870 * (((locals.var_xg1x_dn9 - locals.var_q1d_dn9) - locals.var_xdeff_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign20650_e20888;
        locals.var_q_temp1_dn4 = assign20650_e20888_d_n4;
        locals.var_q_temp1_dn6 = assign20650_e20888_d_n6;
        locals.var_q_temp1_dn7 = assign20650_e20888_d_n7;
        locals.var_q_temp1_dn8 = assign20650_e20888_d_n8;
        locals.var_q_temp1_dn9 = assign20650_e20888_d_n9;
        locals.var_q_temp1_rv = 0.0;

        let assign20660_e20891: f64 = (locals.var_a0 * locals.var_q_temp1);
        locals.var_aexp1d = assign20660_e20891;
        locals.var_aexp1d_dn4 = ((locals.var_a0_dn4 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn4));
        locals.var_aexp1d_dn6 = ((locals.var_a0_dn6 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn6));
        locals.var_aexp1d_dn7 = ((locals.var_a0_dn7 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn7));
        locals.var_aexp1d_dn8 = ((locals.var_a0_dn8 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn8));
        locals.var_aexp1d_dn9 = ((locals.var_a0_dn9 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn9));
        locals.var_aexp1d_rv = 0.0;

        let assign20670_e20894: f64 = (locals.var_k1q1d * locals.var_k1q1d);
        let assign20670_e20896: f64 = (assign20670_e20894 - locals.var_aexp1d);
        locals.var_qsqd = assign20670_e20896;
        locals.var_qsqd_dn4 = (((locals.var_k1q1d_dn4 * locals.var_k1q1d) + (locals.var_k1q1d * locals.var_k1q1d_dn4)) - locals.var_aexp1d_dn4);
        locals.var_qsqd_dn6 = (((locals.var_k1q1d_dn6 * locals.var_k1q1d) + (locals.var_k1q1d * locals.var_k1q1d_dn6)) - locals.var_aexp1d_dn6);
        locals.var_qsqd_dn7 = (((locals.var_k1q1d_dn7 * locals.var_k1q1d) + (locals.var_k1q1d * locals.var_k1q1d_dn7)) - locals.var_aexp1d_dn7);
        locals.var_qsqd_dn8 = (((locals.var_k1q1d_dn8 * locals.var_k1q1d) + (locals.var_k1q1d * locals.var_k1q1d_dn8)) - locals.var_aexp1d_dn8);
        locals.var_qsqd_dn9 = (((locals.var_k1q1d_dn9 * locals.var_k1q1d) + (locals.var_k1q1d * locals.var_k1q1d_dn9)) - locals.var_aexp1d_dn9);
        locals.var_qsqd_rv = 0.0;

        let assign20680_e20899: f64 = if locals.var_aexp1d <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard656 = assign20680_e20899;
        locals.var_guard656_rv = 0.0;

        let (assign20690_e20903, assign20690_e20903_d_n4, assign20690_e20903_d_n6, assign20690_e20903_d_n7, assign20690_e20903_d_n8, assign20690_e20903_d_n9,) = {
    if (locals.var_guard656 != 0.0) {
        (1e-80, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qid, locals.var_qid_dn4, locals.var_qid_dn6, locals.var_qid_dn7, locals.var_qid_dn8, locals.var_qid_dn9,)
    }
};
        locals.var_qid = assign20690_e20903;
        locals.var_qid_dn4 = assign20690_e20903_d_n4;
        locals.var_qid_dn6 = assign20690_e20903_d_n6;
        locals.var_qid_dn7 = assign20690_e20903_d_n7;
        locals.var_qid_dn8 = assign20690_e20903_d_n8;
        locals.var_qid_dn9 = assign20690_e20903_d_n9;
        locals.var_qid_rv = 0.0;

        let (assign20700_e20909, assign20700_e20909_d_n4, assign20700_e20909_d_n6, assign20700_e20909_d_n7, assign20700_e20909_d_n8, assign20700_e20909_d_n9,) = {
    if (locals.var_guard656 != 0.0) {
        let assign20700_e20907: f64 = (locals.var_qid - locals.var_k1q1d);
        (assign20700_e20907, (locals.var_qid_dn4 - locals.var_k1q1d_dn4), (locals.var_qid_dn6 - locals.var_k1q1d_dn6), (locals.var_qid_dn7 - locals.var_k1q1d_dn7), (locals.var_qid_dn8 - locals.var_k1q1d_dn8), (locals.var_qid_dn9 - locals.var_k1q1d_dn9),)
    } else {
        (locals.var_k2q2d, locals.var_k2q2d_dn4, locals.var_k2q2d_dn6, locals.var_k2q2d_dn7, locals.var_k2q2d_dn8, locals.var_k2q2d_dn9,)
    }
};
        locals.var_k2q2d = assign20700_e20909;
        locals.var_k2q2d_dn4 = assign20700_e20909_d_n4;
        locals.var_k2q2d_dn6 = assign20700_e20909_d_n6;
        locals.var_k2q2d_dn7 = assign20700_e20909_d_n7;
        locals.var_k2q2d_dn8 = assign20700_e20909_d_n8;
        locals.var_k2q2d_dn9 = assign20700_e20909_d_n9;
        locals.var_k2q2d_rv = 0.0;

        let (assign20710_e20915, assign20710_e20915_d_n4, assign20710_e20915_d_n6, assign20710_e20915_d_n7, assign20710_e20915_d_n8, assign20710_e20915_d_n9,) = {
    if (locals.var_guard656 != 0.0) {
        let assign20710_e20913: f64 = (locals.var_k2q2d / locals.var_k2);
        (assign20710_e20913, (((locals.var_k2q2d_dn4 * locals.var_k2) - (locals.var_k2q2d * locals.var_k2_dn4)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2d_dn6 * locals.var_k2) - (locals.var_k2q2d * locals.var_k2_dn6)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2d_dn7 * locals.var_k2) - (locals.var_k2q2d * locals.var_k2_dn7)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2d_dn8 * locals.var_k2) - (locals.var_k2q2d * locals.var_k2_dn8)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2d_dn9 * locals.var_k2) - (locals.var_k2q2d * locals.var_k2_dn9)) / (locals.var_k2 * locals.var_k2)),)
    } else {
        (locals.var_q2d, locals.var_q2d_dn4, locals.var_q2d_dn6, locals.var_q2d_dn7, locals.var_q2d_dn8, locals.var_q2d_dn9,)
    }
};
        locals.var_q2d = assign20710_e20915;
        locals.var_q2d_dn4 = assign20710_e20915_d_n4;
        locals.var_q2d_dn6 = assign20710_e20915_d_n6;
        locals.var_q2d_dn7 = assign20710_e20915_d_n7;
        locals.var_q2d_dn8 = assign20710_e20915_d_n8;
        locals.var_q2d_dn9 = assign20710_e20915_d_n9;
        locals.var_q2d_rv = 0.0;

        let assign20720_e20918: f64 = (-0.005);
        let assign20720_e20919: f64 = if locals.var_qsqd < assign20720_e20918 { 1.0 } else { 0.0 };
        locals.var_guard657 = assign20720_e20919;
        locals.var_guard657_rv = 0.0;

        let (assign20730_e20928, assign20730_e20928_d_n4, assign20730_e20928_d_n6, assign20730_e20928_d_n7, assign20730_e20928_d_n8, assign20730_e20928_d_n9,) = {
    if ((locals.var_guard656 == 0.0) && (locals.var_guard657 != 0.0)) {
        let assign20730_e20925: f64 = (locals.var_qsqd).abs();
        let assign20730_e20926: f64 = (assign20730_e20925).sqrt();
        (assign20730_e20926, (if locals.var_qsqd >= 0.0 { locals.var_qsqd_dn4 } else { (-locals.var_qsqd_dn4) } / (2.0 * assign20730_e20926)), (if locals.var_qsqd >= 0.0 { locals.var_qsqd_dn6 } else { (-locals.var_qsqd_dn6) } / (2.0 * assign20730_e20926)), (if locals.var_qsqd >= 0.0 { locals.var_qsqd_dn7 } else { (-locals.var_qsqd_dn7) } / (2.0 * assign20730_e20926)), (if locals.var_qsqd >= 0.0 { locals.var_qsqd_dn8 } else { (-locals.var_qsqd_dn8) } / (2.0 * assign20730_e20926)), (if locals.var_qsqd >= 0.0 { locals.var_qsqd_dn9 } else { (-locals.var_qsqd_dn9) } / (2.0 * assign20730_e20926)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign20730_e20928;
        locals.var_q_rac_qsq_dn4 = assign20730_e20928_d_n4;
        locals.var_q_rac_qsq_dn6 = assign20730_e20928_d_n6;
        locals.var_q_rac_qsq_dn7 = assign20730_e20928_d_n7;
        locals.var_q_rac_qsq_dn8 = assign20730_e20928_d_n8;
        locals.var_q_rac_qsq_dn9 = assign20730_e20928_d_n9;
        locals.var_q_rac_qsq_rv = 0.0;

        let (assign20740_e20940, assign20740_e20940_d_n4, assign20740_e20940_d_n6, assign20740_e20940_d_n7, assign20740_e20940_d_n8, assign20740_e20940_d_n9,) = {
    if ((locals.var_guard656 == 0.0) && (locals.var_guard657 != 0.0)) {
        let assign20740_e20936: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign20740_e20937: f64 = (assign20740_e20936).tan();
        let assign20740_e20938: f64 = (locals.var_q_rac_qsq / assign20740_e20937);
        (assign20740_e20938, (((locals.var_q_rac_qsq_dn4 * assign20740_e20937) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn4) / ((assign20740_e20936).cos() * (assign20740_e20936).cos())))) / (assign20740_e20937 * assign20740_e20937)), (((locals.var_q_rac_qsq_dn6 * assign20740_e20937) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn6) / ((assign20740_e20936).cos() * (assign20740_e20936).cos())))) / (assign20740_e20937 * assign20740_e20937)), (((locals.var_q_rac_qsq_dn7 * assign20740_e20937) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn7) / ((assign20740_e20936).cos() * (assign20740_e20936).cos())))) / (assign20740_e20937 * assign20740_e20937)), (((locals.var_q_rac_qsq_dn8 * assign20740_e20937) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn8) / ((assign20740_e20936).cos() * (assign20740_e20936).cos())))) / (assign20740_e20937 * assign20740_e20937)), (((locals.var_q_rac_qsq_dn9 * assign20740_e20937) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn9) / ((assign20740_e20936).cos() * (assign20740_e20936).cos())))) / (assign20740_e20937 * assign20740_e20937)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign20740_e20940;
        locals.var_q_qcoth_dn4 = assign20740_e20940_d_n4;
        locals.var_q_qcoth_dn6 = assign20740_e20940_d_n6;
        locals.var_q_qcoth_dn7 = assign20740_e20940_d_n7;
        locals.var_q_qcoth_dn8 = assign20740_e20940_d_n8;
        locals.var_q_qcoth_dn9 = assign20740_e20940_d_n9;
        locals.var_q_qcoth_rv = 0.0;

        let assign20750_e20943: f64 = if locals.var_qsqd > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard658 = assign20750_e20943;
        locals.var_guard658_rv = 0.0;

        let (assign20760_e20955, assign20760_e20955_d_n4, assign20760_e20955_d_n6, assign20760_e20955_d_n7, assign20760_e20955_d_n8, assign20760_e20955_d_n9,) = {
    if (((locals.var_guard656 == 0.0) && (locals.var_guard657 == 0.0)) && (locals.var_guard658 != 0.0)) {
        let assign20760_e20952: f64 = (locals.var_qsqd).abs();
        let assign20760_e20953: f64 = (assign20760_e20952).sqrt();
        (assign20760_e20953, (if locals.var_qsqd >= 0.0 { locals.var_qsqd_dn4 } else { (-locals.var_qsqd_dn4) } / (2.0 * assign20760_e20953)), (if locals.var_qsqd >= 0.0 { locals.var_qsqd_dn6 } else { (-locals.var_qsqd_dn6) } / (2.0 * assign20760_e20953)), (if locals.var_qsqd >= 0.0 { locals.var_qsqd_dn7 } else { (-locals.var_qsqd_dn7) } / (2.0 * assign20760_e20953)), (if locals.var_qsqd >= 0.0 { locals.var_qsqd_dn8 } else { (-locals.var_qsqd_dn8) } / (2.0 * assign20760_e20953)), (if locals.var_qsqd >= 0.0 { locals.var_qsqd_dn9 } else { (-locals.var_qsqd_dn9) } / (2.0 * assign20760_e20953)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign20760_e20955;
        locals.var_q_rac_qsq_dn4 = assign20760_e20955_d_n4;
        locals.var_q_rac_qsq_dn6 = assign20760_e20955_d_n6;
        locals.var_q_rac_qsq_dn7 = assign20760_e20955_d_n7;
        locals.var_q_rac_qsq_dn8 = assign20760_e20955_d_n8;
        locals.var_q_rac_qsq_dn9 = assign20760_e20955_d_n9;
        locals.var_q_rac_qsq_rv = 0.0;

        let (assign20770_e20967, assign20770_e20967_d_n4, assign20770_e20967_d_n6, assign20770_e20967_d_n7, assign20770_e20967_d_n8, assign20770_e20967_d_n9,) = {
    if (((locals.var_guard656 == 0.0) && (locals.var_guard657 == 0.0)) && (locals.var_guard658 != 0.0)) {
        let assign20770_e20964: f64 = (-locals.var_q_rac_qsq);
        let assign20770_e20965: f64 = (assign20770_e20964).exp();
        (assign20770_e20965, (assign20770_e20965 * (-locals.var_q_rac_qsq_dn4)), (assign20770_e20965 * (-locals.var_q_rac_qsq_dn6)), (assign20770_e20965 * (-locals.var_q_rac_qsq_dn7)), (assign20770_e20965 * (-locals.var_q_rac_qsq_dn8)), (assign20770_e20965 * (-locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_invexpq, locals.var_q_invexpq_dn4, locals.var_q_invexpq_dn6, locals.var_q_invexpq_dn7, locals.var_q_invexpq_dn8, locals.var_q_invexpq_dn9,)
    }
};
        locals.var_q_invexpq = assign20770_e20967;
        locals.var_q_invexpq_dn4 = assign20770_e20967_d_n4;
        locals.var_q_invexpq_dn6 = assign20770_e20967_d_n6;
        locals.var_q_invexpq_dn7 = assign20770_e20967_d_n7;
        locals.var_q_invexpq_dn8 = assign20770_e20967_d_n8;
        locals.var_q_invexpq_dn9 = assign20770_e20967_d_n9;
        locals.var_q_invexpq_rv = 0.0;

        let (assign20780_e20985, assign20780_e20985_d_n4, assign20780_e20985_d_n6, assign20780_e20985_d_n7, assign20780_e20985_d_n8, assign20780_e20985_d_n9,) = {
    if (((locals.var_guard656 == 0.0) && (locals.var_guard657 == 0.0)) && (locals.var_guard658 != 0.0)) {
        let assign20780_e20978: f64 = (1.0 + locals.var_q_invexpq);
        let assign20780_e20979: f64 = (locals.var_q_rac_qsq * assign20780_e20978);
        let assign20780_e20982: f64 = (1.0 - locals.var_q_invexpq);
        let assign20780_e20983: f64 = (assign20780_e20979 / assign20780_e20982);
        (assign20780_e20983, (((((locals.var_q_rac_qsq_dn4 * assign20780_e20978) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn4)) * assign20780_e20982) - (assign20780_e20979 * (-locals.var_q_invexpq_dn4))) / (assign20780_e20982 * assign20780_e20982)), (((((locals.var_q_rac_qsq_dn6 * assign20780_e20978) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn6)) * assign20780_e20982) - (assign20780_e20979 * (-locals.var_q_invexpq_dn6))) / (assign20780_e20982 * assign20780_e20982)), (((((locals.var_q_rac_qsq_dn7 * assign20780_e20978) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn7)) * assign20780_e20982) - (assign20780_e20979 * (-locals.var_q_invexpq_dn7))) / (assign20780_e20982 * assign20780_e20982)), (((((locals.var_q_rac_qsq_dn8 * assign20780_e20978) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn8)) * assign20780_e20982) - (assign20780_e20979 * (-locals.var_q_invexpq_dn8))) / (assign20780_e20982 * assign20780_e20982)), (((((locals.var_q_rac_qsq_dn9 * assign20780_e20978) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn9)) * assign20780_e20982) - (assign20780_e20979 * (-locals.var_q_invexpq_dn9))) / (assign20780_e20982 * assign20780_e20982)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign20780_e20985;
        locals.var_q_qcoth_dn4 = assign20780_e20985_d_n4;
        locals.var_q_qcoth_dn6 = assign20780_e20985_d_n6;
        locals.var_q_qcoth_dn7 = assign20780_e20985_d_n7;
        locals.var_q_qcoth_dn8 = assign20780_e20985_d_n8;
        locals.var_q_qcoth_dn9 = assign20780_e20985_d_n9;
        locals.var_q_qcoth_rv = 0.0;

        let (assign20790_e21012, assign20790_e21012_d_n4, assign20790_e21012_d_n6, assign20790_e21012_d_n7, assign20790_e21012_d_n8, assign20790_e21012_d_n9,) = {
    if (((locals.var_guard656 == 0.0) && (locals.var_guard657 == 0.0)) && (locals.var_guard658 == 0.0)) {
        let assign20790_e20997: f64 = (locals.var_qsqd * 0.1666666666667);
        let assign20790_e21001: f64 = (locals.var_qsqd * 0.0166666666667);
        let assign20790_e21005: f64 = (locals.var_qsqd * 0.0238095238095);
        let assign20790_e21006: f64 = (1.0 - assign20790_e21005);
        let assign20790_e21007: f64 = (assign20790_e21001 * assign20790_e21006);
        let assign20790_e21008: f64 = (1.0 - assign20790_e21007);
        let assign20790_e21009: f64 = (assign20790_e20997 * assign20790_e21008);
        let assign20790_e21010: f64 = (2.0 + assign20790_e21009);
        (assign20790_e21010, (((locals.var_qsqd_dn4 * 0.1666666666667) * assign20790_e21008) + (assign20790_e20997 * (-(((locals.var_qsqd_dn4 * 0.0166666666667) * assign20790_e21006) + (assign20790_e21001 * (-(locals.var_qsqd_dn4 * 0.0238095238095))))))), (((locals.var_qsqd_dn6 * 0.1666666666667) * assign20790_e21008) + (assign20790_e20997 * (-(((locals.var_qsqd_dn6 * 0.0166666666667) * assign20790_e21006) + (assign20790_e21001 * (-(locals.var_qsqd_dn6 * 0.0238095238095))))))), (((locals.var_qsqd_dn7 * 0.1666666666667) * assign20790_e21008) + (assign20790_e20997 * (-(((locals.var_qsqd_dn7 * 0.0166666666667) * assign20790_e21006) + (assign20790_e21001 * (-(locals.var_qsqd_dn7 * 0.0238095238095))))))), (((locals.var_qsqd_dn8 * 0.1666666666667) * assign20790_e21008) + (assign20790_e20997 * (-(((locals.var_qsqd_dn8 * 0.0166666666667) * assign20790_e21006) + (assign20790_e21001 * (-(locals.var_qsqd_dn8 * 0.0238095238095))))))), (((locals.var_qsqd_dn9 * 0.1666666666667) * assign20790_e21008) + (assign20790_e20997 * (-(((locals.var_qsqd_dn9 * 0.0166666666667) * assign20790_e21006) + (assign20790_e21001 * (-(locals.var_qsqd_dn9 * 0.0238095238095))))))),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign20790_e21012;
        locals.var_q_qcoth_dn4 = assign20790_e21012_d_n4;
        locals.var_q_qcoth_dn6 = assign20790_e21012_d_n6;
        locals.var_q_qcoth_dn7 = assign20790_e21012_d_n7;
        locals.var_q_qcoth_dn8 = assign20790_e21012_d_n8;
        locals.var_q_qcoth_dn9 = assign20790_e21012_d_n9;
        locals.var_q_qcoth_rv = 0.0;

        let assign20800_e21015: f64 = (1.01 * locals.var_k1q1d);
        let assign20800_e21017: f64 = (assign20800_e21015 + locals.var_q_qcoth);
        let assign20800_e21019: f64 = if assign20800_e21017 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard659 = assign20800_e21019;
        locals.var_guard659_rv = 0.0;

        let (assign20810_e21028, assign20810_e21028_d_n4, assign20810_e21028_d_n6, assign20810_e21028_d_n7, assign20810_e21028_d_n8, assign20810_e21028_d_n9,) = {
    if ((locals.var_guard656 == 0.0) && (locals.var_guard659 != 0.0)) {
        let assign20810_e21026: f64 = (locals.var_k1q1d + locals.var_q_qcoth);
        (assign20810_e21026, (locals.var_k1q1d_dn4 + locals.var_q_qcoth_dn4), (locals.var_k1q1d_dn6 + locals.var_q_qcoth_dn6), (locals.var_k1q1d_dn7 + locals.var_q_qcoth_dn7), (locals.var_k1q1d_dn8 + locals.var_q_qcoth_dn8), (locals.var_k1q1d_dn9 + locals.var_q_qcoth_dn9),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign20810_e21028;
        locals.var_q_temp1_dn4 = assign20810_e21028_d_n4;
        locals.var_q_temp1_dn6 = assign20810_e21028_d_n6;
        locals.var_q_temp1_dn7 = assign20810_e21028_d_n7;
        locals.var_q_temp1_dn8 = assign20810_e21028_d_n8;
        locals.var_q_temp1_dn9 = assign20810_e21028_d_n9;
        locals.var_q_temp1_rv = 0.0;

        let assign20820_e21031: f64 = (locals.var_aexp1d * locals.var_k1q1d);
        let assign20820_e21034: f64 = (0.9 * locals.var_k1q1d);
        let assign20820_e21036: f64 = (assign20820_e21034 * locals.var_k1q1d);
        let assign20820_e21038: f64 = (assign20820_e21036 * locals.var_q_temp1);
        let assign20820_e21039: f64 = if assign20820_e21031 < assign20820_e21038 { 1.0 } else { 0.0 };
        locals.var_guard660 = assign20820_e21039;
        locals.var_guard660_rv = 0.0;

        let (assign20830_e21052, assign20830_e21052_d_n4, assign20830_e21052_d_n6, assign20830_e21052_d_n7, assign20830_e21052_d_n8, assign20830_e21052_d_n9,) = {
    if (((locals.var_guard656 == 0.0) && (locals.var_guard659 != 0.0)) && (locals.var_guard660 != 0.0)) {
        let assign20830_e21048: f64 = (locals.var_aexp1d / locals.var_q_temp1);
        let assign20830_e21050: f64 = (assign20830_e21048 + 1e-80);
        (assign20830_e21050, (((locals.var_aexp1d_dn4 * locals.var_q_temp1) - (locals.var_aexp1d * locals.var_q_temp1_dn4)) / (locals.var_q_temp1 * locals.var_q_temp1)), (((locals.var_aexp1d_dn6 * locals.var_q_temp1) - (locals.var_aexp1d * locals.var_q_temp1_dn6)) / (locals.var_q_temp1 * locals.var_q_temp1)), (((locals.var_aexp1d_dn7 * locals.var_q_temp1) - (locals.var_aexp1d * locals.var_q_temp1_dn7)) / (locals.var_q_temp1 * locals.var_q_temp1)), (((locals.var_aexp1d_dn8 * locals.var_q_temp1) - (locals.var_aexp1d * locals.var_q_temp1_dn8)) / (locals.var_q_temp1 * locals.var_q_temp1)), (((locals.var_aexp1d_dn9 * locals.var_q_temp1) - (locals.var_aexp1d * locals.var_q_temp1_dn9)) / (locals.var_q_temp1 * locals.var_q_temp1)),)
    } else {
        (locals.var_qid, locals.var_qid_dn4, locals.var_qid_dn6, locals.var_qid_dn7, locals.var_qid_dn8, locals.var_qid_dn9,)
    }
};
        locals.var_qid = assign20830_e21052;
        locals.var_qid_dn4 = assign20830_e21052_d_n4;
        locals.var_qid_dn6 = assign20830_e21052_d_n6;
        locals.var_qid_dn7 = assign20830_e21052_d_n7;
        locals.var_qid_dn8 = assign20830_e21052_d_n8;
        locals.var_qid_dn9 = assign20830_e21052_d_n9;
        locals.var_qid_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_57(
        locals: &mut StampLocals,
    ) {
        let (assign20840_e21063, assign20840_e21063_d_n4, assign20840_e21063_d_n6, assign20840_e21063_d_n7, assign20840_e21063_d_n8, assign20840_e21063_d_n9,) = {
    if (((locals.var_guard656 == 0.0) && (locals.var_guard659 != 0.0)) && (locals.var_guard660 != 0.0)) {
        let assign20840_e21061: f64 = (locals.var_qid - locals.var_k1q1d);
        (assign20840_e21061, (locals.var_qid_dn4 - locals.var_k1q1d_dn4), (locals.var_qid_dn6 - locals.var_k1q1d_dn6), (locals.var_qid_dn7 - locals.var_k1q1d_dn7), (locals.var_qid_dn8 - locals.var_k1q1d_dn8), (locals.var_qid_dn9 - locals.var_k1q1d_dn9),)
    } else {
        (locals.var_k2q2d, locals.var_k2q2d_dn4, locals.var_k2q2d_dn6, locals.var_k2q2d_dn7, locals.var_k2q2d_dn8, locals.var_k2q2d_dn9,)
    }
};
        locals.var_k2q2d = assign20840_e21063;
        locals.var_k2q2d_dn4 = assign20840_e21063_d_n4;
        locals.var_k2q2d_dn6 = assign20840_e21063_d_n6;
        locals.var_k2q2d_dn7 = assign20840_e21063_d_n7;
        locals.var_k2q2d_dn8 = assign20840_e21063_d_n8;
        locals.var_k2q2d_dn9 = assign20840_e21063_d_n9;
        locals.var_k2q2d_rv = 0.0;

        let (assign20850_e21074, assign20850_e21074_d_n4, assign20850_e21074_d_n6, assign20850_e21074_d_n7, assign20850_e21074_d_n8, assign20850_e21074_d_n9,) = {
    if (((locals.var_guard656 == 0.0) && (locals.var_guard659 != 0.0)) && (locals.var_guard660 != 0.0)) {
        let assign20850_e21072: f64 = (locals.var_k2q2d / locals.var_k2);
        (assign20850_e21072, (((locals.var_k2q2d_dn4 * locals.var_k2) - (locals.var_k2q2d * locals.var_k2_dn4)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2d_dn6 * locals.var_k2) - (locals.var_k2q2d * locals.var_k2_dn6)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2d_dn7 * locals.var_k2) - (locals.var_k2q2d * locals.var_k2_dn7)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2d_dn8 * locals.var_k2) - (locals.var_k2q2d * locals.var_k2_dn8)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2d_dn9 * locals.var_k2) - (locals.var_k2q2d * locals.var_k2_dn9)) / (locals.var_k2 * locals.var_k2)),)
    } else {
        (locals.var_q2d, locals.var_q2d_dn4, locals.var_q2d_dn6, locals.var_q2d_dn7, locals.var_q2d_dn8, locals.var_q2d_dn9,)
    }
};
        locals.var_q2d = assign20850_e21074;
        locals.var_q2d_dn4 = assign20850_e21074_d_n4;
        locals.var_q2d_dn6 = assign20850_e21074_d_n6;
        locals.var_q2d_dn7 = assign20850_e21074_d_n7;
        locals.var_q2d_dn8 = assign20850_e21074_d_n8;
        locals.var_q2d_dn9 = assign20850_e21074_d_n9;
        locals.var_q2d_rv = 0.0;

        let assign20860_e21077: f64 = if locals.var_qsqd > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard661 = assign20860_e21077;
        locals.var_guard661_rv = 0.0;

        let (assign20870_e21102, assign20870_e21102_d_n4, assign20870_e21102_d_n6, assign20870_e21102_d_n7, assign20870_e21102_d_n8, assign20870_e21102_d_n9,) = {
    if ((((locals.var_guard656 == 0.0) && (locals.var_guard659 != 0.0)) && (locals.var_guard660 == 0.0)) && (locals.var_guard661 != 0.0)) {
        let assign20870_e21089: f64 = (4.0 * locals.var_qsqd);
        let assign20870_e21094: f64 = (2.0 - locals.var_q_invexpq);
        let assign20870_e21095: f64 = (locals.var_q_invexpq * assign20870_e21094);
        let assign20870_e21096: f64 = (1.0 - assign20870_e21095);
        let assign20870_e21097: f64 = (assign20870_e21089 / assign20870_e21096);
        let assign20870_e21098: f64 = (assign20870_e21097).ln();
        let assign20870_e21100: f64 = (assign20870_e21098 - locals.var_q_rac_qsq);
        (assign20870_e21100, ((((((4.0 * locals.var_qsqd_dn4) * assign20870_e21096) - (assign20870_e21089 * (-((locals.var_q_invexpq_dn4 * assign20870_e21094) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn4)))))) / (assign20870_e21096 * assign20870_e21096)) / assign20870_e21097) - locals.var_q_rac_qsq_dn4), ((((((4.0 * locals.var_qsqd_dn6) * assign20870_e21096) - (assign20870_e21089 * (-((locals.var_q_invexpq_dn6 * assign20870_e21094) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn6)))))) / (assign20870_e21096 * assign20870_e21096)) / assign20870_e21097) - locals.var_q_rac_qsq_dn6), ((((((4.0 * locals.var_qsqd_dn7) * assign20870_e21096) - (assign20870_e21089 * (-((locals.var_q_invexpq_dn7 * assign20870_e21094) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn7)))))) / (assign20870_e21096 * assign20870_e21096)) / assign20870_e21097) - locals.var_q_rac_qsq_dn7), ((((((4.0 * locals.var_qsqd_dn8) * assign20870_e21096) - (assign20870_e21089 * (-((locals.var_q_invexpq_dn8 * assign20870_e21094) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn8)))))) / (assign20870_e21096 * assign20870_e21096)) / assign20870_e21097) - locals.var_q_rac_qsq_dn8), ((((((4.0 * locals.var_qsqd_dn9) * assign20870_e21096) - (assign20870_e21089 * (-((locals.var_q_invexpq_dn9 * assign20870_e21094) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn9)))))) / (assign20870_e21096 * assign20870_e21096)) / assign20870_e21097) - locals.var_q_rac_qsq_dn9),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign20870_e21102;
        locals.var_q_temp2_dn4 = assign20870_e21102_d_n4;
        locals.var_q_temp2_dn6 = assign20870_e21102_d_n6;
        locals.var_q_temp2_dn7 = assign20870_e21102_d_n7;
        locals.var_q_temp2_dn8 = assign20870_e21102_d_n8;
        locals.var_q_temp2_dn9 = assign20870_e21102_d_n9;
        locals.var_q_temp2_rv = 0.0;

        let assign20880_e21105: f64 = (-0.005);
        let assign20880_e21106: f64 = if locals.var_qsqd < assign20880_e21105 { 1.0 } else { 0.0 };
        locals.var_guard662 = assign20880_e21106;
        locals.var_guard662_rv = 0.0;

        let (assign20890_e21124, assign20890_e21124_d_n4, assign20890_e21124_d_n6, assign20890_e21124_d_n7, assign20890_e21124_d_n8, assign20890_e21124_d_n9,) = {
    if (((((locals.var_guard656 == 0.0) && (locals.var_guard659 != 0.0)) && (locals.var_guard660 == 0.0)) && (locals.var_guard661 == 0.0)) && (locals.var_guard662 != 0.0)) {
        let assign20890_e21121: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign20890_e21122: f64 = (assign20890_e21121).sin();
        (assign20890_e21122, ((assign20890_e21121).cos() * (0.5 * locals.var_q_rac_qsq_dn4)), ((assign20890_e21121).cos() * (0.5 * locals.var_q_rac_qsq_dn6)), ((assign20890_e21121).cos() * (0.5 * locals.var_q_rac_qsq_dn7)), ((assign20890_e21121).cos() * (0.5 * locals.var_q_rac_qsq_dn8)), ((assign20890_e21121).cos() * (0.5 * locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign20890_e21124;
        locals.var_q_temp3_dn4 = assign20890_e21124_d_n4;
        locals.var_q_temp3_dn6 = assign20890_e21124_d_n6;
        locals.var_q_temp3_dn7 = assign20890_e21124_d_n7;
        locals.var_q_temp3_dn8 = assign20890_e21124_d_n8;
        locals.var_q_temp3_dn9 = assign20890_e21124_d_n9;
        locals.var_q_temp3_rv = 0.0;

        let (assign20900_e21145, assign20900_e21145_d_n4, assign20900_e21145_d_n6, assign20900_e21145_d_n7, assign20900_e21145_d_n8, assign20900_e21145_d_n9,) = {
    if (((((locals.var_guard656 == 0.0) && (locals.var_guard659 != 0.0)) && (locals.var_guard660 == 0.0)) && (locals.var_guard661 == 0.0)) && (locals.var_guard662 != 0.0)) {
        let assign20900_e21138: f64 = (-locals.var_qsqd);
        let assign20900_e21141: f64 = (locals.var_q_temp3 * locals.var_q_temp3);
        let assign20900_e21142: f64 = (assign20900_e21138 / assign20900_e21141);
        let assign20900_e21143: f64 = (assign20900_e21142).ln();
        (assign20900_e21143, (((((-locals.var_qsqd_dn4) * assign20900_e21141) - (assign20900_e21138 * ((locals.var_q_temp3_dn4 * locals.var_q_temp3) + (locals.var_q_temp3 * locals.var_q_temp3_dn4)))) / (assign20900_e21141 * assign20900_e21141)) / assign20900_e21142), (((((-locals.var_qsqd_dn6) * assign20900_e21141) - (assign20900_e21138 * ((locals.var_q_temp3_dn6 * locals.var_q_temp3) + (locals.var_q_temp3 * locals.var_q_temp3_dn6)))) / (assign20900_e21141 * assign20900_e21141)) / assign20900_e21142), (((((-locals.var_qsqd_dn7) * assign20900_e21141) - (assign20900_e21138 * ((locals.var_q_temp3_dn7 * locals.var_q_temp3) + (locals.var_q_temp3 * locals.var_q_temp3_dn7)))) / (assign20900_e21141 * assign20900_e21141)) / assign20900_e21142), (((((-locals.var_qsqd_dn8) * assign20900_e21141) - (assign20900_e21138 * ((locals.var_q_temp3_dn8 * locals.var_q_temp3) + (locals.var_q_temp3 * locals.var_q_temp3_dn8)))) / (assign20900_e21141 * assign20900_e21141)) / assign20900_e21142), (((((-locals.var_qsqd_dn9) * assign20900_e21141) - (assign20900_e21138 * ((locals.var_q_temp3_dn9 * locals.var_q_temp3) + (locals.var_q_temp3 * locals.var_q_temp3_dn9)))) / (assign20900_e21141 * assign20900_e21141)) / assign20900_e21142),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign20900_e21145;
        locals.var_q_temp2_dn4 = assign20900_e21145_d_n4;
        locals.var_q_temp2_dn6 = assign20900_e21145_d_n6;
        locals.var_q_temp2_dn7 = assign20900_e21145_d_n7;
        locals.var_q_temp2_dn8 = assign20900_e21145_d_n8;
        locals.var_q_temp2_dn9 = assign20900_e21145_d_n9;
        locals.var_q_temp2_rv = 0.0;

        let (assign20910_e21178, assign20910_e21178_d_n4, assign20910_e21178_d_n6, assign20910_e21178_d_n7, assign20910_e21178_d_n8, assign20910_e21178_d_n9,) = {
    if (((((locals.var_guard656 == 0.0) && (locals.var_guard659 != 0.0)) && (locals.var_guard660 == 0.0)) && (locals.var_guard661 == 0.0)) && (locals.var_guard662 == 0.0)) {
        let assign20910_e21162: f64 = (locals.var_qsqd * 0.3333333333333);
        let assign20910_e21166: f64 = (0.05 * locals.var_qsqd);
        let assign20910_e21170: f64 = (0.0396825396825397 * locals.var_qsqd);
        let assign20910_e21171: f64 = (1.0 - assign20910_e21170);
        let assign20910_e21172: f64 = (assign20910_e21166 * assign20910_e21171);
        let assign20910_e21173: f64 = (1.0 - assign20910_e21172);
        let assign20910_e21174: f64 = (assign20910_e21162 * assign20910_e21173);
        let assign20910_e21175: f64 = (4.0 - assign20910_e21174);
        let assign20910_e21176: f64 = (assign20910_e21175).ln();
        (assign20910_e21176, ((-(((locals.var_qsqd_dn4 * 0.3333333333333) * assign20910_e21173) + (assign20910_e21162 * (-(((0.05 * locals.var_qsqd_dn4) * assign20910_e21171) + (assign20910_e21166 * (-(0.0396825396825397 * locals.var_qsqd_dn4)))))))) / assign20910_e21175), ((-(((locals.var_qsqd_dn6 * 0.3333333333333) * assign20910_e21173) + (assign20910_e21162 * (-(((0.05 * locals.var_qsqd_dn6) * assign20910_e21171) + (assign20910_e21166 * (-(0.0396825396825397 * locals.var_qsqd_dn6)))))))) / assign20910_e21175), ((-(((locals.var_qsqd_dn7 * 0.3333333333333) * assign20910_e21173) + (assign20910_e21162 * (-(((0.05 * locals.var_qsqd_dn7) * assign20910_e21171) + (assign20910_e21166 * (-(0.0396825396825397 * locals.var_qsqd_dn7)))))))) / assign20910_e21175), ((-(((locals.var_qsqd_dn8 * 0.3333333333333) * assign20910_e21173) + (assign20910_e21162 * (-(((0.05 * locals.var_qsqd_dn8) * assign20910_e21171) + (assign20910_e21166 * (-(0.0396825396825397 * locals.var_qsqd_dn8)))))))) / assign20910_e21175), ((-(((locals.var_qsqd_dn9 * 0.3333333333333) * assign20910_e21173) + (assign20910_e21162 * (-(((0.05 * locals.var_qsqd_dn9) * assign20910_e21171) + (assign20910_e21166 * (-(0.0396825396825397 * locals.var_qsqd_dn9)))))))) / assign20910_e21175),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign20910_e21178;
        locals.var_q_temp2_dn4 = assign20910_e21178_d_n4;
        locals.var_q_temp2_dn6 = assign20910_e21178_d_n6;
        locals.var_q_temp2_dn7 = assign20910_e21178_d_n7;
        locals.var_q_temp2_dn8 = assign20910_e21178_d_n8;
        locals.var_q_temp2_dn9 = assign20910_e21178_d_n9;
        locals.var_q_temp2_rv = 0.0;

        let (assign20920_e21199, assign20920_e21199_d_n4, assign20920_e21199_d_n6, assign20920_e21199_d_n7, assign20920_e21199_d_n8, assign20920_e21199_d_n9,) = {
    if (((locals.var_guard656 == 0.0) && (locals.var_guard659 != 0.0)) && (locals.var_guard660 == 0.0)) {
        let assign20920_e21188: f64 = (locals.var_xg2x - locals.var_xg1x);
        let assign20920_e21190: f64 = (assign20920_e21188 + locals.var_q1d);
        let assign20920_e21193: f64 = (locals.var_q_temp1).ln();
        let assign20920_e21194: f64 = (2.0 * assign20920_e21193);
        let assign20920_e21195: f64 = (assign20920_e21190 + assign20920_e21194);
        let assign20920_e21197: f64 = (assign20920_e21195 - locals.var_q_temp2);
        (assign20920_e21197, ((((locals.var_xg2x_dn4 - locals.var_xg1x_dn4) + locals.var_q1d_dn4) + (2.0 * (locals.var_q_temp1_dn4 / locals.var_q_temp1))) - locals.var_q_temp2_dn4), ((((locals.var_xg2x_dn6 - locals.var_xg1x_dn6) + locals.var_q1d_dn6) + (2.0 * (locals.var_q_temp1_dn6 / locals.var_q_temp1))) - locals.var_q_temp2_dn6), ((((locals.var_xg2x_dn7 - locals.var_xg1x_dn7) + locals.var_q1d_dn7) + (2.0 * (locals.var_q_temp1_dn7 / locals.var_q_temp1))) - locals.var_q_temp2_dn7), ((((locals.var_xg2x_dn8 - locals.var_xg1x_dn8) + locals.var_q1d_dn8) + (2.0 * (locals.var_q_temp1_dn8 / locals.var_q_temp1))) - locals.var_q_temp2_dn8), ((((locals.var_xg2x_dn9 - locals.var_xg1x_dn9) + locals.var_q1d_dn9) + (2.0 * (locals.var_q_temp1_dn9 / locals.var_q_temp1))) - locals.var_q_temp2_dn9),)
    } else {
        (locals.var_q2d, locals.var_q2d_dn4, locals.var_q2d_dn6, locals.var_q2d_dn7, locals.var_q2d_dn8, locals.var_q2d_dn9,)
    }
};
        locals.var_q2d = assign20920_e21199;
        locals.var_q2d_dn4 = assign20920_e21199_d_n4;
        locals.var_q2d_dn6 = assign20920_e21199_d_n6;
        locals.var_q2d_dn7 = assign20920_e21199_d_n7;
        locals.var_q2d_dn8 = assign20920_e21199_d_n8;
        locals.var_q2d_dn9 = assign20920_e21199_d_n9;
        locals.var_q2d_rv = 0.0;

        let (assign20930_e21211, assign20930_e21211_d_n4, assign20930_e21211_d_n6, assign20930_e21211_d_n7, assign20930_e21211_d_n8, assign20930_e21211_d_n9,) = {
    if (((locals.var_guard656 == 0.0) && (locals.var_guard659 != 0.0)) && (locals.var_guard660 == 0.0)) {
        let assign20930_e21209: f64 = (locals.var_k2 * locals.var_q2d);
        (assign20930_e21209, ((locals.var_k2_dn4 * locals.var_q2d) + (locals.var_k2 * locals.var_q2d_dn4)), ((locals.var_k2_dn6 * locals.var_q2d) + (locals.var_k2 * locals.var_q2d_dn6)), ((locals.var_k2_dn7 * locals.var_q2d) + (locals.var_k2 * locals.var_q2d_dn7)), ((locals.var_k2_dn8 * locals.var_q2d) + (locals.var_k2 * locals.var_q2d_dn8)), ((locals.var_k2_dn9 * locals.var_q2d) + (locals.var_k2 * locals.var_q2d_dn9)),)
    } else {
        (locals.var_k2q2d, locals.var_k2q2d_dn4, locals.var_k2q2d_dn6, locals.var_k2q2d_dn7, locals.var_k2q2d_dn8, locals.var_k2q2d_dn9,)
    }
};
        locals.var_k2q2d = assign20930_e21211;
        locals.var_k2q2d_dn4 = assign20930_e21211_d_n4;
        locals.var_k2q2d_dn6 = assign20930_e21211_d_n6;
        locals.var_k2q2d_dn7 = assign20930_e21211_d_n7;
        locals.var_k2q2d_dn8 = assign20930_e21211_d_n8;
        locals.var_k2q2d_dn9 = assign20930_e21211_d_n9;
        locals.var_k2q2d_rv = 0.0;

        let (assign20940_e21223, assign20940_e21223_d_n4, assign20940_e21223_d_n6, assign20940_e21223_d_n7, assign20940_e21223_d_n8, assign20940_e21223_d_n9,) = {
    if (((locals.var_guard656 == 0.0) && (locals.var_guard659 != 0.0)) && (locals.var_guard660 == 0.0)) {
        let assign20940_e21221: f64 = (locals.var_k1q1d + locals.var_k2q2d);
        (assign20940_e21221, (locals.var_k1q1d_dn4 + locals.var_k2q2d_dn4), (locals.var_k1q1d_dn6 + locals.var_k2q2d_dn6), (locals.var_k1q1d_dn7 + locals.var_k2q2d_dn7), (locals.var_k1q1d_dn8 + locals.var_k2q2d_dn8), (locals.var_k1q1d_dn9 + locals.var_k2q2d_dn9),)
    } else {
        (locals.var_qid, locals.var_qid_dn4, locals.var_qid_dn6, locals.var_qid_dn7, locals.var_qid_dn8, locals.var_qid_dn9,)
    }
};
        locals.var_qid = assign20940_e21223;
        locals.var_qid_dn4 = assign20940_e21223_d_n4;
        locals.var_qid_dn6 = assign20940_e21223_d_n6;
        locals.var_qid_dn7 = assign20940_e21223_d_n7;
        locals.var_qid_dn8 = assign20940_e21223_d_n8;
        locals.var_qid_dn9 = assign20940_e21223_d_n9;
        locals.var_qid_rv = 0.0;

        let assign20950_e21226: f64 = if locals.var_qsqd > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard663 = assign20950_e21226;
        locals.var_guard663_rv = 0.0;

        let assign20960_e21229: f64 = (locals.var_q1d + locals.var_xdeff);
        let assign20960_e21231: f64 = (assign20960_e21229 - locals.var_xg1x);
        let assign20960_e21233: f64 = (assign20960_e21231 - locals.var_q_rac_qsq);
        let assign20960_e21235: f64 = if assign20960_e21233 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard664 = assign20960_e21235;
        locals.var_guard664_rv = 0.0;

        let (assign20970_e21254, assign20970_e21254_d_n4, assign20970_e21254_d_n6, assign20970_e21254_d_n7, assign20970_e21254_d_n8, assign20970_e21254_d_n9,) = {
    if ((((locals.var_guard656 == 0.0) && (locals.var_guard659 == 0.0)) && (locals.var_guard663 != 0.0)) && (locals.var_guard664 != 0.0)) {
        let assign20970_e21247: f64 = (locals.var_q1d + locals.var_xdeff);
        let assign20970_e21249: f64 = (assign20970_e21247 - locals.var_xg1x);
        let assign20970_e21251: f64 = (assign20970_e21249 - locals.var_q_rac_qsq);
        let assign20970_e21252: f64 = (assign20970_e21251).exp();
        (assign20970_e21252, (assign20970_e21252 * (((locals.var_q1d_dn4 + locals.var_xdeff_dn4) - locals.var_xg1x_dn4) - locals.var_q_rac_qsq_dn4)), (assign20970_e21252 * (((locals.var_q1d_dn6 + locals.var_xdeff_dn6) - locals.var_xg1x_dn6) - locals.var_q_rac_qsq_dn6)), (assign20970_e21252 * (((locals.var_q1d_dn7 + locals.var_xdeff_dn7) - locals.var_xg1x_dn7) - locals.var_q_rac_qsq_dn7)), (assign20970_e21252 * (((locals.var_q1d_dn8 + locals.var_xdeff_dn8) - locals.var_xg1x_dn8) - locals.var_q_rac_qsq_dn8)), (assign20970_e21252 * (((locals.var_q1d_dn9 + locals.var_xdeff_dn9) - locals.var_xg1x_dn9) - locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign20970_e21254;
        locals.var_q_temp3_dn4 = assign20970_e21254_d_n4;
        locals.var_q_temp3_dn6 = assign20970_e21254_d_n6;
        locals.var_q_temp3_dn7 = assign20970_e21254_d_n7;
        locals.var_q_temp3_dn8 = assign20970_e21254_d_n8;
        locals.var_q_temp3_dn9 = assign20970_e21254_d_n9;
        locals.var_q_temp3_rv = 0.0;

        let (assign20980_e21307, assign20980_e21307_d_n4, assign20980_e21307_d_n6, assign20980_e21307_d_n7, assign20980_e21307_d_n8, assign20980_e21307_d_n9,) = {
    if ((((locals.var_guard656 == 0.0) && (locals.var_guard659 == 0.0)) && (locals.var_guard663 != 0.0)) && (locals.var_guard664 == 0.0)) {
        let assign20980_e21269: f64 = (locals.var_q1d + locals.var_xdeff);
        let assign20980_e21271: f64 = (assign20980_e21269 - locals.var_xg1x);
        let assign20980_e21273: f64 = (assign20980_e21271 - locals.var_q_rac_qsq);
        let assign20980_e21275: f64 = (assign20980_e21273 - 80.0);
        let assign20980_e21280: f64 = (locals.var_q1d + locals.var_xdeff);
        let assign20980_e21282: f64 = (assign20980_e21280 - locals.var_xg1x);
        let assign20980_e21284: f64 = (assign20980_e21282 - locals.var_q_rac_qsq);
        let assign20980_e21286: f64 = (assign20980_e21284 - 80.0);
        let assign20980_e21287: f64 = (0.5 * assign20980_e21286);
        let assign20980_e21291: f64 = (locals.var_q1d + locals.var_xdeff);
        let assign20980_e21293: f64 = (assign20980_e21291 - locals.var_xg1x);
        let assign20980_e21295: f64 = (assign20980_e21293 - locals.var_q_rac_qsq);
        let assign20980_e21297: f64 = (assign20980_e21295 - 80.0);
        let assign20980_e21299: f64 = (assign20980_e21297 * 0.3333333333333);
        let assign20980_e21300: f64 = (1.0 + assign20980_e21299);
        let assign20980_e21301: f64 = (assign20980_e21287 * assign20980_e21300);
        let assign20980_e21302: f64 = (1.0 + assign20980_e21301);
        let assign20980_e21303: f64 = (assign20980_e21275 * assign20980_e21302);
        let assign20980_e21304: f64 = (1.0 + assign20980_e21303);
        let assign20980_e21305: f64 = (5.54062e34 * assign20980_e21304);
        (assign20980_e21305, (5.54062e34 * (((((locals.var_q1d_dn4 + locals.var_xdeff_dn4) - locals.var_xg1x_dn4) - locals.var_q_rac_qsq_dn4) * assign20980_e21302) + (assign20980_e21275 * (((0.5 * (((locals.var_q1d_dn4 + locals.var_xdeff_dn4) - locals.var_xg1x_dn4) - locals.var_q_rac_qsq_dn4)) * assign20980_e21300) + (assign20980_e21287 * ((((locals.var_q1d_dn4 + locals.var_xdeff_dn4) - locals.var_xg1x_dn4) - locals.var_q_rac_qsq_dn4) * 0.3333333333333)))))), (5.54062e34 * (((((locals.var_q1d_dn6 + locals.var_xdeff_dn6) - locals.var_xg1x_dn6) - locals.var_q_rac_qsq_dn6) * assign20980_e21302) + (assign20980_e21275 * (((0.5 * (((locals.var_q1d_dn6 + locals.var_xdeff_dn6) - locals.var_xg1x_dn6) - locals.var_q_rac_qsq_dn6)) * assign20980_e21300) + (assign20980_e21287 * ((((locals.var_q1d_dn6 + locals.var_xdeff_dn6) - locals.var_xg1x_dn6) - locals.var_q_rac_qsq_dn6) * 0.3333333333333)))))), (5.54062e34 * (((((locals.var_q1d_dn7 + locals.var_xdeff_dn7) - locals.var_xg1x_dn7) - locals.var_q_rac_qsq_dn7) * assign20980_e21302) + (assign20980_e21275 * (((0.5 * (((locals.var_q1d_dn7 + locals.var_xdeff_dn7) - locals.var_xg1x_dn7) - locals.var_q_rac_qsq_dn7)) * assign20980_e21300) + (assign20980_e21287 * ((((locals.var_q1d_dn7 + locals.var_xdeff_dn7) - locals.var_xg1x_dn7) - locals.var_q_rac_qsq_dn7) * 0.3333333333333)))))), (5.54062e34 * (((((locals.var_q1d_dn8 + locals.var_xdeff_dn8) - locals.var_xg1x_dn8) - locals.var_q_rac_qsq_dn8) * assign20980_e21302) + (assign20980_e21275 * (((0.5 * (((locals.var_q1d_dn8 + locals.var_xdeff_dn8) - locals.var_xg1x_dn8) - locals.var_q_rac_qsq_dn8)) * assign20980_e21300) + (assign20980_e21287 * ((((locals.var_q1d_dn8 + locals.var_xdeff_dn8) - locals.var_xg1x_dn8) - locals.var_q_rac_qsq_dn8) * 0.3333333333333)))))), (5.54062e34 * (((((locals.var_q1d_dn9 + locals.var_xdeff_dn9) - locals.var_xg1x_dn9) - locals.var_q_rac_qsq_dn9) * assign20980_e21302) + (assign20980_e21275 * (((0.5 * (((locals.var_q1d_dn9 + locals.var_xdeff_dn9) - locals.var_xg1x_dn9) - locals.var_q_rac_qsq_dn9)) * assign20980_e21300) + (assign20980_e21287 * ((((locals.var_q1d_dn9 + locals.var_xdeff_dn9) - locals.var_xg1x_dn9) - locals.var_q_rac_qsq_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign20980_e21307;
        locals.var_q_temp3_dn4 = assign20980_e21307_d_n4;
        locals.var_q_temp3_dn6 = assign20980_e21307_d_n6;
        locals.var_q_temp3_dn7 = assign20980_e21307_d_n7;
        locals.var_q_temp3_dn8 = assign20980_e21307_d_n8;
        locals.var_q_temp3_dn9 = assign20980_e21307_d_n9;
        locals.var_q_temp3_rv = 0.0;

        let (assign20990_e21319, assign20990_e21319_d_n4, assign20990_e21319_d_n6, assign20990_e21319_d_n7, assign20990_e21319_d_n8, assign20990_e21319_d_n9,) = {
    if (((locals.var_guard656 == 0.0) && (locals.var_guard659 == 0.0)) && (locals.var_guard663 != 0.0)) {
        let assign20990_e21317: f64 = (locals.var_q_temp3 / locals.var_a0);
        (assign20990_e21317, (((locals.var_q_temp3_dn4 * locals.var_a0) - (locals.var_q_temp3 * locals.var_a0_dn4)) / (locals.var_a0 * locals.var_a0)), (((locals.var_q_temp3_dn6 * locals.var_a0) - (locals.var_q_temp3 * locals.var_a0_dn6)) / (locals.var_a0 * locals.var_a0)), (((locals.var_q_temp3_dn7 * locals.var_a0) - (locals.var_q_temp3 * locals.var_a0_dn7)) / (locals.var_a0 * locals.var_a0)), (((locals.var_q_temp3_dn8 * locals.var_a0) - (locals.var_q_temp3 * locals.var_a0_dn8)) / (locals.var_a0 * locals.var_a0)), (((locals.var_q_temp3_dn9 * locals.var_a0) - (locals.var_q_temp3 * locals.var_a0_dn9)) / (locals.var_a0 * locals.var_a0)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign20990_e21319;
        locals.var_q_temp2_dn4 = assign20990_e21319_d_n4;
        locals.var_q_temp2_dn6 = assign20990_e21319_d_n6;
        locals.var_q_temp2_dn7 = assign20990_e21319_d_n7;
        locals.var_q_temp2_dn8 = assign20990_e21319_d_n8;
        locals.var_q_temp2_dn9 = assign20990_e21319_d_n9;
        locals.var_q_temp2_rv = 0.0;

        let (assign21000_e21341, assign21000_e21341_d_n4, assign21000_e21341_d_n6, assign21000_e21341_d_n7, assign21000_e21341_d_n8, assign21000_e21341_d_n9,) = {
    if (((locals.var_guard656 == 0.0) && (locals.var_guard659 == 0.0)) && (locals.var_guard663 != 0.0)) {
        let assign21000_e21329: f64 = (4.0 * locals.var_qsqd);
        let assign21000_e21331: f64 = (assign21000_e21329 * locals.var_q_temp2);
        let assign21000_e21336: f64 = (2.0 - locals.var_q_invexpq);
        let assign21000_e21337: f64 = (locals.var_q_invexpq * assign21000_e21336);
        let assign21000_e21338: f64 = (1.0 - assign21000_e21337);
        let assign21000_e21339: f64 = (assign21000_e21331 / assign21000_e21338);
        (assign21000_e21339, ((((((4.0 * locals.var_qsqd_dn4) * locals.var_q_temp2) + (assign21000_e21329 * locals.var_q_temp2_dn4)) * assign21000_e21338) - (assign21000_e21331 * (-((locals.var_q_invexpq_dn4 * assign21000_e21336) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn4)))))) / (assign21000_e21338 * assign21000_e21338)), ((((((4.0 * locals.var_qsqd_dn6) * locals.var_q_temp2) + (assign21000_e21329 * locals.var_q_temp2_dn6)) * assign21000_e21338) - (assign21000_e21331 * (-((locals.var_q_invexpq_dn6 * assign21000_e21336) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn6)))))) / (assign21000_e21338 * assign21000_e21338)), ((((((4.0 * locals.var_qsqd_dn7) * locals.var_q_temp2) + (assign21000_e21329 * locals.var_q_temp2_dn7)) * assign21000_e21338) - (assign21000_e21331 * (-((locals.var_q_invexpq_dn7 * assign21000_e21336) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn7)))))) / (assign21000_e21338 * assign21000_e21338)), ((((((4.0 * locals.var_qsqd_dn8) * locals.var_q_temp2) + (assign21000_e21329 * locals.var_q_temp2_dn8)) * assign21000_e21338) - (assign21000_e21331 * (-((locals.var_q_invexpq_dn8 * assign21000_e21336) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn8)))))) / (assign21000_e21338 * assign21000_e21338)), ((((((4.0 * locals.var_qsqd_dn9) * locals.var_q_temp2) + (assign21000_e21329 * locals.var_q_temp2_dn9)) * assign21000_e21338) - (assign21000_e21331 * (-((locals.var_q_invexpq_dn9 * assign21000_e21336) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn9)))))) / (assign21000_e21338 * assign21000_e21338)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign21000_e21341;
        locals.var_q_temp1_dn4 = assign21000_e21341_d_n4;
        locals.var_q_temp1_dn6 = assign21000_e21341_d_n6;
        locals.var_q_temp1_dn7 = assign21000_e21341_d_n7;
        locals.var_q_temp1_dn8 = assign21000_e21341_d_n8;
        locals.var_q_temp1_dn9 = assign21000_e21341_d_n9;
        locals.var_q_temp1_rv = 0.0;

        let assign21010_e21344: f64 = (-0.005);
        let assign21010_e21345: f64 = if locals.var_qsqd < assign21010_e21344 { 1.0 } else { 0.0 };
        locals.var_guard665 = assign21010_e21345;
        locals.var_guard665_rv = 0.0;

        let (assign21020_e21361, assign21020_e21361_d_n4, assign21020_e21361_d_n6, assign21020_e21361_d_n7, assign21020_e21361_d_n8, assign21020_e21361_d_n9,) = {
    if ((((locals.var_guard656 == 0.0) && (locals.var_guard659 == 0.0)) && (locals.var_guard663 == 0.0)) && (locals.var_guard665 != 0.0)) {
        let assign21020_e21358: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign21020_e21359: f64 = (assign21020_e21358).sin();
        (assign21020_e21359, ((assign21020_e21358).cos() * (0.5 * locals.var_q_rac_qsq_dn4)), ((assign21020_e21358).cos() * (0.5 * locals.var_q_rac_qsq_dn6)), ((assign21020_e21358).cos() * (0.5 * locals.var_q_rac_qsq_dn7)), ((assign21020_e21358).cos() * (0.5 * locals.var_q_rac_qsq_dn8)), ((assign21020_e21358).cos() * (0.5 * locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign21020_e21361;
        locals.var_q_temp2_dn4 = assign21020_e21361_d_n4;
        locals.var_q_temp2_dn6 = assign21020_e21361_d_n6;
        locals.var_q_temp2_dn7 = assign21020_e21361_d_n7;
        locals.var_q_temp2_dn8 = assign21020_e21361_d_n8;
        locals.var_q_temp2_dn9 = assign21020_e21361_d_n9;
        locals.var_q_temp2_rv = 0.0;

        let (assign21030_e21381, assign21030_e21381_d_n4, assign21030_e21381_d_n6, assign21030_e21381_d_n7, assign21030_e21381_d_n8, assign21030_e21381_d_n9,) = {
    if ((((locals.var_guard656 == 0.0) && (locals.var_guard659 == 0.0)) && (locals.var_guard663 == 0.0)) && (locals.var_guard665 != 0.0)) {
        let assign21030_e21373: f64 = (-locals.var_qsqd);
        let assign21030_e21376: f64 = (locals.var_q_temp2 * locals.var_q_temp2);
        let assign21030_e21377: f64 = (assign21030_e21373 / assign21030_e21376);
        let assign21030_e21379: f64 = (assign21030_e21377 / locals.var_aexp1d);
        (assign21030_e21379, (((((((-locals.var_qsqd_dn4) * assign21030_e21376) - (assign21030_e21373 * ((locals.var_q_temp2_dn4 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn4)))) / (assign21030_e21376 * assign21030_e21376)) * locals.var_aexp1d) - (assign21030_e21377 * locals.var_aexp1d_dn4)) / (locals.var_aexp1d * locals.var_aexp1d)), (((((((-locals.var_qsqd_dn6) * assign21030_e21376) - (assign21030_e21373 * ((locals.var_q_temp2_dn6 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn6)))) / (assign21030_e21376 * assign21030_e21376)) * locals.var_aexp1d) - (assign21030_e21377 * locals.var_aexp1d_dn6)) / (locals.var_aexp1d * locals.var_aexp1d)), (((((((-locals.var_qsqd_dn7) * assign21030_e21376) - (assign21030_e21373 * ((locals.var_q_temp2_dn7 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn7)))) / (assign21030_e21376 * assign21030_e21376)) * locals.var_aexp1d) - (assign21030_e21377 * locals.var_aexp1d_dn7)) / (locals.var_aexp1d * locals.var_aexp1d)), (((((((-locals.var_qsqd_dn8) * assign21030_e21376) - (assign21030_e21373 * ((locals.var_q_temp2_dn8 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn8)))) / (assign21030_e21376 * assign21030_e21376)) * locals.var_aexp1d) - (assign21030_e21377 * locals.var_aexp1d_dn8)) / (locals.var_aexp1d * locals.var_aexp1d)), (((((((-locals.var_qsqd_dn9) * assign21030_e21376) - (assign21030_e21373 * ((locals.var_q_temp2_dn9 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn9)))) / (assign21030_e21376 * assign21030_e21376)) * locals.var_aexp1d) - (assign21030_e21377 * locals.var_aexp1d_dn9)) / (locals.var_aexp1d * locals.var_aexp1d)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign21030_e21381;
        locals.var_q_temp1_dn4 = assign21030_e21381_d_n4;
        locals.var_q_temp1_dn6 = assign21030_e21381_d_n6;
        locals.var_q_temp1_dn7 = assign21030_e21381_d_n7;
        locals.var_q_temp1_dn8 = assign21030_e21381_d_n8;
        locals.var_q_temp1_dn9 = assign21030_e21381_d_n9;
        locals.var_q_temp1_rv = 0.0;

        let (assign21040_e21413, assign21040_e21413_d_n4, assign21040_e21413_d_n6, assign21040_e21413_d_n7, assign21040_e21413_d_n8, assign21040_e21413_d_n9,) = {
    if ((((locals.var_guard656 == 0.0) && (locals.var_guard659 == 0.0)) && (locals.var_guard663 == 0.0)) && (locals.var_guard665 == 0.0)) {
        let assign21040_e21396: f64 = (locals.var_qsqd * 0.3333333333333);
        let assign21040_e21400: f64 = (0.05 * locals.var_qsqd);
        let assign21040_e21404: f64 = (0.0396825396825397 * locals.var_qsqd);
        let assign21040_e21405: f64 = (1.0 - assign21040_e21404);
        let assign21040_e21406: f64 = (assign21040_e21400 * assign21040_e21405);
        let assign21040_e21407: f64 = (1.0 - assign21040_e21406);
        let assign21040_e21408: f64 = (assign21040_e21396 * assign21040_e21407);
        let assign21040_e21409: f64 = (4.0 - assign21040_e21408);
        let assign21040_e21411: f64 = (assign21040_e21409 / locals.var_aexp1d);
        (assign21040_e21411, ((((-(((locals.var_qsqd_dn4 * 0.3333333333333) * assign21040_e21407) + (assign21040_e21396 * (-(((0.05 * locals.var_qsqd_dn4) * assign21040_e21405) + (assign21040_e21400 * (-(0.0396825396825397 * locals.var_qsqd_dn4)))))))) * locals.var_aexp1d) - (assign21040_e21409 * locals.var_aexp1d_dn4)) / (locals.var_aexp1d * locals.var_aexp1d)), ((((-(((locals.var_qsqd_dn6 * 0.3333333333333) * assign21040_e21407) + (assign21040_e21396 * (-(((0.05 * locals.var_qsqd_dn6) * assign21040_e21405) + (assign21040_e21400 * (-(0.0396825396825397 * locals.var_qsqd_dn6)))))))) * locals.var_aexp1d) - (assign21040_e21409 * locals.var_aexp1d_dn6)) / (locals.var_aexp1d * locals.var_aexp1d)), ((((-(((locals.var_qsqd_dn7 * 0.3333333333333) * assign21040_e21407) + (assign21040_e21396 * (-(((0.05 * locals.var_qsqd_dn7) * assign21040_e21405) + (assign21040_e21400 * (-(0.0396825396825397 * locals.var_qsqd_dn7)))))))) * locals.var_aexp1d) - (assign21040_e21409 * locals.var_aexp1d_dn7)) / (locals.var_aexp1d * locals.var_aexp1d)), ((((-(((locals.var_qsqd_dn8 * 0.3333333333333) * assign21040_e21407) + (assign21040_e21396 * (-(((0.05 * locals.var_qsqd_dn8) * assign21040_e21405) + (assign21040_e21400 * (-(0.0396825396825397 * locals.var_qsqd_dn8)))))))) * locals.var_aexp1d) - (assign21040_e21409 * locals.var_aexp1d_dn8)) / (locals.var_aexp1d * locals.var_aexp1d)), ((((-(((locals.var_qsqd_dn9 * 0.3333333333333) * assign21040_e21407) + (assign21040_e21396 * (-(((0.05 * locals.var_qsqd_dn9) * assign21040_e21405) + (assign21040_e21400 * (-(0.0396825396825397 * locals.var_qsqd_dn9)))))))) * locals.var_aexp1d) - (assign21040_e21409 * locals.var_aexp1d_dn9)) / (locals.var_aexp1d * locals.var_aexp1d)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign21040_e21413;
        locals.var_q_temp1_dn4 = assign21040_e21413_d_n4;
        locals.var_q_temp1_dn6 = assign21040_e21413_d_n6;
        locals.var_q_temp1_dn7 = assign21040_e21413_d_n7;
        locals.var_q_temp1_dn8 = assign21040_e21413_d_n8;
        locals.var_q_temp1_dn9 = assign21040_e21413_d_n9;
        locals.var_q_temp1_rv = 0.0;

        let (assign21050_e21429, assign21050_e21429_d_n4, assign21050_e21429_d_n6, assign21050_e21429_d_n7, assign21050_e21429_d_n8, assign21050_e21429_d_n9,) = {
    if ((locals.var_guard656 == 0.0) && (locals.var_guard659 == 0.0)) {
        let assign21050_e21421: f64 = (locals.var_k1q1d - locals.var_q_qcoth);
        let assign21050_e21424: f64 = (1.0 - locals.var_q_temp1);
        let assign21050_e21425: f64 = (assign21050_e21421 / assign21050_e21424);
        let assign21050_e21427: f64 = (assign21050_e21425 + 1e-80);
        (assign21050_e21427, ((((locals.var_k1q1d_dn4 - locals.var_q_qcoth_dn4) * assign21050_e21424) - (assign21050_e21421 * (-locals.var_q_temp1_dn4))) / (assign21050_e21424 * assign21050_e21424)), ((((locals.var_k1q1d_dn6 - locals.var_q_qcoth_dn6) * assign21050_e21424) - (assign21050_e21421 * (-locals.var_q_temp1_dn6))) / (assign21050_e21424 * assign21050_e21424)), ((((locals.var_k1q1d_dn7 - locals.var_q_qcoth_dn7) * assign21050_e21424) - (assign21050_e21421 * (-locals.var_q_temp1_dn7))) / (assign21050_e21424 * assign21050_e21424)), ((((locals.var_k1q1d_dn8 - locals.var_q_qcoth_dn8) * assign21050_e21424) - (assign21050_e21421 * (-locals.var_q_temp1_dn8))) / (assign21050_e21424 * assign21050_e21424)), ((((locals.var_k1q1d_dn9 - locals.var_q_qcoth_dn9) * assign21050_e21424) - (assign21050_e21421 * (-locals.var_q_temp1_dn9))) / (assign21050_e21424 * assign21050_e21424)),)
    } else {
        (locals.var_qid, locals.var_qid_dn4, locals.var_qid_dn6, locals.var_qid_dn7, locals.var_qid_dn8, locals.var_qid_dn9,)
    }
};
        locals.var_qid = assign21050_e21429;
        locals.var_qid_dn4 = assign21050_e21429_d_n4;
        locals.var_qid_dn6 = assign21050_e21429_d_n6;
        locals.var_qid_dn7 = assign21050_e21429_d_n7;
        locals.var_qid_dn8 = assign21050_e21429_d_n8;
        locals.var_qid_dn9 = assign21050_e21429_d_n9;
        locals.var_qid_rv = 0.0;

        let (assign21060_e21439, assign21060_e21439_d_n4, assign21060_e21439_d_n6, assign21060_e21439_d_n7, assign21060_e21439_d_n8, assign21060_e21439_d_n9,) = {
    if ((locals.var_guard656 == 0.0) && (locals.var_guard659 == 0.0)) {
        let assign21060_e21437: f64 = (locals.var_qid - locals.var_k1q1d);
        (assign21060_e21437, (locals.var_qid_dn4 - locals.var_k1q1d_dn4), (locals.var_qid_dn6 - locals.var_k1q1d_dn6), (locals.var_qid_dn7 - locals.var_k1q1d_dn7), (locals.var_qid_dn8 - locals.var_k1q1d_dn8), (locals.var_qid_dn9 - locals.var_k1q1d_dn9),)
    } else {
        (locals.var_k2q2d, locals.var_k2q2d_dn4, locals.var_k2q2d_dn6, locals.var_k2q2d_dn7, locals.var_k2q2d_dn8, locals.var_k2q2d_dn9,)
    }
};
        locals.var_k2q2d = assign21060_e21439;
        locals.var_k2q2d_dn4 = assign21060_e21439_d_n4;
        locals.var_k2q2d_dn6 = assign21060_e21439_d_n6;
        locals.var_k2q2d_dn7 = assign21060_e21439_d_n7;
        locals.var_k2q2d_dn8 = assign21060_e21439_d_n8;
        locals.var_k2q2d_dn9 = assign21060_e21439_d_n9;
        locals.var_k2q2d_rv = 0.0;

        let (assign21070_e21449, assign21070_e21449_d_n4, assign21070_e21449_d_n6, assign21070_e21449_d_n7, assign21070_e21449_d_n8, assign21070_e21449_d_n9,) = {
    if ((locals.var_guard656 == 0.0) && (locals.var_guard659 == 0.0)) {
        let assign21070_e21447: f64 = (locals.var_k2q2d / locals.var_k2);
        (assign21070_e21447, (((locals.var_k2q2d_dn4 * locals.var_k2) - (locals.var_k2q2d * locals.var_k2_dn4)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2d_dn6 * locals.var_k2) - (locals.var_k2q2d * locals.var_k2_dn6)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2d_dn7 * locals.var_k2) - (locals.var_k2q2d * locals.var_k2_dn7)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2d_dn8 * locals.var_k2) - (locals.var_k2q2d * locals.var_k2_dn8)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2d_dn9 * locals.var_k2) - (locals.var_k2q2d * locals.var_k2_dn9)) / (locals.var_k2 * locals.var_k2)),)
    } else {
        (locals.var_q2d, locals.var_q2d_dn4, locals.var_q2d_dn6, locals.var_q2d_dn7, locals.var_q2d_dn8, locals.var_q2d_dn9,)
    }
};
        locals.var_q2d = assign21070_e21449;
        locals.var_q2d_dn4 = assign21070_e21449_d_n4;
        locals.var_q2d_dn6 = assign21070_e21449_d_n6;
        locals.var_q2d_dn7 = assign21070_e21449_d_n7;
        locals.var_q2d_dn8 = assign21070_e21449_d_n8;
        locals.var_q2d_dn9 = assign21070_e21449_d_n9;
        locals.var_q2d_rv = 0.0;

        let assign21080_e21452: f64 = (locals.var_xg2x - locals.var_q2d);
        let assign21080_e21454: f64 = (assign21080_e21452 - locals.var_xdeff);
        let assign21080_e21456: f64 = if assign21080_e21454 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard666 = assign21080_e21456;
        locals.var_guard666_rv = 0.0;

        let (assign21090_e21465, assign21090_e21465_d_n4, assign21090_e21465_d_n6, assign21090_e21465_d_n7, assign21090_e21465_d_n8, assign21090_e21465_d_n9,) = {
    if (locals.var_guard666 != 0.0) {
        let assign21090_e21460: f64 = (locals.var_xg2x - locals.var_q2d);
        let assign21090_e21462: f64 = (assign21090_e21460 - locals.var_xdeff);
        let assign21090_e21463: f64 = (assign21090_e21462).exp();
        (assign21090_e21463, (assign21090_e21463 * ((locals.var_xg2x_dn4 - locals.var_q2d_dn4) - locals.var_xdeff_dn4)), (assign21090_e21463 * ((locals.var_xg2x_dn6 - locals.var_q2d_dn6) - locals.var_xdeff_dn6)), (assign21090_e21463 * ((locals.var_xg2x_dn7 - locals.var_q2d_dn7) - locals.var_xdeff_dn7)), (assign21090_e21463 * ((locals.var_xg2x_dn8 - locals.var_q2d_dn8) - locals.var_xdeff_dn8)), (assign21090_e21463 * ((locals.var_xg2x_dn9 - locals.var_q2d_dn9) - locals.var_xdeff_dn9)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign21090_e21465;
        locals.var_q_temp1_dn4 = assign21090_e21465_d_n4;
        locals.var_q_temp1_dn6 = assign21090_e21465_d_n6;
        locals.var_q_temp1_dn7 = assign21090_e21465_d_n7;
        locals.var_q_temp1_dn8 = assign21090_e21465_d_n8;
        locals.var_q_temp1_dn9 = assign21090_e21465_d_n9;
        locals.var_q_temp1_rv = 0.0;

        let (assign21100_e21504, assign21100_e21504_d_n4, assign21100_e21504_d_n6, assign21100_e21504_d_n7, assign21100_e21504_d_n8, assign21100_e21504_d_n9,) = {
    if (locals.var_guard666 == 0.0) {
        let assign21100_e21472: f64 = (locals.var_xg2x - locals.var_q2d);
        let assign21100_e21474: f64 = (assign21100_e21472 - locals.var_xdeff);
        let assign21100_e21476: f64 = (assign21100_e21474 - 80.0);
        let assign21100_e21481: f64 = (locals.var_xg2x - locals.var_q2d);
        let assign21100_e21483: f64 = (assign21100_e21481 - locals.var_xdeff);
        let assign21100_e21485: f64 = (assign21100_e21483 - 80.0);
        let assign21100_e21486: f64 = (0.5 * assign21100_e21485);
        let assign21100_e21490: f64 = (locals.var_xg2x - locals.var_q2d);
        let assign21100_e21492: f64 = (assign21100_e21490 - locals.var_xdeff);
        let assign21100_e21494: f64 = (assign21100_e21492 - 80.0);
        let assign21100_e21496: f64 = (assign21100_e21494 * 0.3333333333333);
        let assign21100_e21497: f64 = (1.0 + assign21100_e21496);
        let assign21100_e21498: f64 = (assign21100_e21486 * assign21100_e21497);
        let assign21100_e21499: f64 = (1.0 + assign21100_e21498);
        let assign21100_e21500: f64 = (assign21100_e21476 * assign21100_e21499);
        let assign21100_e21501: f64 = (1.0 + assign21100_e21500);
        let assign21100_e21502: f64 = (5.54062e34 * assign21100_e21501);
        (assign21100_e21502, (5.54062e34 * ((((locals.var_xg2x_dn4 - locals.var_q2d_dn4) - locals.var_xdeff_dn4) * assign21100_e21499) + (assign21100_e21476 * (((0.5 * ((locals.var_xg2x_dn4 - locals.var_q2d_dn4) - locals.var_xdeff_dn4)) * assign21100_e21497) + (assign21100_e21486 * (((locals.var_xg2x_dn4 - locals.var_q2d_dn4) - locals.var_xdeff_dn4) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg2x_dn6 - locals.var_q2d_dn6) - locals.var_xdeff_dn6) * assign21100_e21499) + (assign21100_e21476 * (((0.5 * ((locals.var_xg2x_dn6 - locals.var_q2d_dn6) - locals.var_xdeff_dn6)) * assign21100_e21497) + (assign21100_e21486 * (((locals.var_xg2x_dn6 - locals.var_q2d_dn6) - locals.var_xdeff_dn6) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg2x_dn7 - locals.var_q2d_dn7) - locals.var_xdeff_dn7) * assign21100_e21499) + (assign21100_e21476 * (((0.5 * ((locals.var_xg2x_dn7 - locals.var_q2d_dn7) - locals.var_xdeff_dn7)) * assign21100_e21497) + (assign21100_e21486 * (((locals.var_xg2x_dn7 - locals.var_q2d_dn7) - locals.var_xdeff_dn7) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg2x_dn8 - locals.var_q2d_dn8) - locals.var_xdeff_dn8) * assign21100_e21499) + (assign21100_e21476 * (((0.5 * ((locals.var_xg2x_dn8 - locals.var_q2d_dn8) - locals.var_xdeff_dn8)) * assign21100_e21497) + (assign21100_e21486 * (((locals.var_xg2x_dn8 - locals.var_q2d_dn8) - locals.var_xdeff_dn8) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg2x_dn9 - locals.var_q2d_dn9) - locals.var_xdeff_dn9) * assign21100_e21499) + (assign21100_e21476 * (((0.5 * ((locals.var_xg2x_dn9 - locals.var_q2d_dn9) - locals.var_xdeff_dn9)) * assign21100_e21497) + (assign21100_e21486 * (((locals.var_xg2x_dn9 - locals.var_q2d_dn9) - locals.var_xdeff_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign21100_e21504;
        locals.var_q_temp1_dn4 = assign21100_e21504_d_n4;
        locals.var_q_temp1_dn6 = assign21100_e21504_d_n6;
        locals.var_q_temp1_dn7 = assign21100_e21504_d_n7;
        locals.var_q_temp1_dn8 = assign21100_e21504_d_n8;
        locals.var_q_temp1_dn9 = assign21100_e21504_d_n9;
        locals.var_q_temp1_rv = 0.0;

        let assign21110_e21507: f64 = (locals.var_a0 * locals.var_q_temp1);
        locals.var_aexp2d = assign21110_e21507;
        locals.var_aexp2d_dn4 = ((locals.var_a0_dn4 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn4));
        locals.var_aexp2d_dn6 = ((locals.var_a0_dn6 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn6));
        locals.var_aexp2d_dn7 = ((locals.var_a0_dn7 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn7));
        locals.var_aexp2d_dn8 = ((locals.var_a0_dn8 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn8));
        locals.var_aexp2d_dn9 = ((locals.var_a0_dn9 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn9));
        locals.var_aexp2d_rv = 0.0;

        locals.var_a1d = 0.0;
        locals.var_a1d_dn4 = 0.0;
        locals.var_a1d_dn6 = 0.0;
        locals.var_a1d_dn7 = 0.0;
        locals.var_a1d_dn8 = 0.0;
        locals.var_a1d_dn9 = 0.0;
        locals.var_a1d_rv = 0.0;

        locals.var_a2d = 0.0;
        locals.var_a2d_dn4 = 0.0;
        locals.var_a2d_dn6 = 0.0;
        locals.var_a2d_dn7 = 0.0;
        locals.var_a2d_dn8 = 0.0;
        locals.var_a2d_dn9 = 0.0;
        locals.var_a2d_rv = 0.0;

        locals.var_b1d = 0.0;
        locals.var_b1d_dn4 = 0.0;
        locals.var_b1d_dn6 = 0.0;
        locals.var_b1d_dn7 = 0.0;
        locals.var_b1d_dn8 = 0.0;
        locals.var_b1d_dn9 = 0.0;
        locals.var_b1d_rv = 0.0;

        locals.var_b2d = 0.0;
        locals.var_b2d_dn4 = 0.0;
        locals.var_b2d_dn6 = 0.0;
        locals.var_b2d_dn7 = 0.0;
        locals.var_b2d_dn8 = 0.0;
        locals.var_b2d_dn9 = 0.0;
        locals.var_b2d_rv = 0.0;

        locals.var_sumd = 0.0;
        locals.var_sumd_dn4 = 0.0;
        locals.var_sumd_dn6 = 0.0;
        locals.var_sumd_dn7 = 0.0;
        locals.var_sumd_dn8 = 0.0;
        locals.var_sumd_dn9 = 0.0;
        locals.var_sumd_rv = 0.0;

        locals.var_dqsqd_dxn_qi = 0.0;
        locals.var_dqsqd_dxn_qi_dn4 = 0.0;
        locals.var_dqsqd_dxn_qi_dn6 = 0.0;
        locals.var_dqsqd_dxn_qi_dn7 = 0.0;
        locals.var_dqsqd_dxn_qi_dn8 = 0.0;
        locals.var_dqsqd_dxn_qi_dn9 = 0.0;
        locals.var_dqsqd_dxn_qi_rv = 0.0;

        let assign21180_e21516: f64 = if locals.var_qis > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard667 = assign21180_e21516;
        locals.var_guard667_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_58(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign21190_e21522, assign21190_e21522_d_n4, assign21190_e21522_d_n6, assign21190_e21522_d_n7, assign21190_e21522_d_n8, assign21190_e21522_d_n9,) = {
    if (locals.var_guard667 != 0.0) {
        let assign21190_e21520: f64 = (locals.var_aexp1d * locals.var_inv_k1);
        (assign21190_e21520, ((locals.var_aexp1d_dn4 * locals.var_inv_k1) + (locals.var_aexp1d * locals.var_inv_k1_dn4)), ((locals.var_aexp1d_dn6 * locals.var_inv_k1) + (locals.var_aexp1d * locals.var_inv_k1_dn6)), ((locals.var_aexp1d_dn7 * locals.var_inv_k1) + (locals.var_aexp1d * locals.var_inv_k1_dn7)), ((locals.var_aexp1d_dn8 * locals.var_inv_k1) + (locals.var_aexp1d * locals.var_inv_k1_dn8)), ((locals.var_aexp1d_dn9 * locals.var_inv_k1) + (locals.var_aexp1d * locals.var_inv_k1_dn9)),)
    } else {
        (locals.var_b1d, locals.var_b1d_dn4, locals.var_b1d_dn6, locals.var_b1d_dn7, locals.var_b1d_dn8, locals.var_b1d_dn9,)
    }
};
        locals.var_b1d = assign21190_e21522;
        locals.var_b1d_dn4 = assign21190_e21522_d_n4;
        locals.var_b1d_dn6 = assign21190_e21522_d_n6;
        locals.var_b1d_dn7 = assign21190_e21522_d_n7;
        locals.var_b1d_dn8 = assign21190_e21522_d_n8;
        locals.var_b1d_dn9 = assign21190_e21522_d_n9;
        locals.var_b1d_rv = 0.0;

        let (assign21200_e21528, assign21200_e21528_d_n4, assign21200_e21528_d_n6, assign21200_e21528_d_n7, assign21200_e21528_d_n8, assign21200_e21528_d_n9,) = {
    if (locals.var_guard667 != 0.0) {
        let assign21200_e21526: f64 = (locals.var_aexp2d * locals.var_inv_k2);
        (assign21200_e21526, ((locals.var_aexp2d_dn4 * locals.var_inv_k2) + (locals.var_aexp2d * locals.var_inv_k2_dn4)), ((locals.var_aexp2d_dn6 * locals.var_inv_k2) + (locals.var_aexp2d * locals.var_inv_k2_dn6)), ((locals.var_aexp2d_dn7 * locals.var_inv_k2) + (locals.var_aexp2d * locals.var_inv_k2_dn7)), ((locals.var_aexp2d_dn8 * locals.var_inv_k2) + (locals.var_aexp2d * locals.var_inv_k2_dn8)), ((locals.var_aexp2d_dn9 * locals.var_inv_k2) + (locals.var_aexp2d * locals.var_inv_k2_dn9)),)
    } else {
        (locals.var_b2d, locals.var_b2d_dn4, locals.var_b2d_dn6, locals.var_b2d_dn7, locals.var_b2d_dn8, locals.var_b2d_dn9,)
    }
};
        locals.var_b2d = assign21200_e21528;
        locals.var_b2d_dn4 = assign21200_e21528_d_n4;
        locals.var_b2d_dn6 = assign21200_e21528_d_n6;
        locals.var_b2d_dn7 = assign21200_e21528_d_n7;
        locals.var_b2d_dn8 = assign21200_e21528_d_n8;
        locals.var_b2d_dn9 = assign21200_e21528_d_n9;
        locals.var_b2d_rv = 0.0;

        let (assign21210_e21536, assign21210_e21536_d_n4, assign21210_e21536_d_n6, assign21210_e21536_d_n7, assign21210_e21536_d_n8, assign21210_e21536_d_n9,) = {
    if (locals.var_guard667 != 0.0) {
        let assign21210_e21533: f64 = (2.0 * locals.var_k1q1d);
        let assign21210_e21534: f64 = (locals.var_b1d + assign21210_e21533);
        (assign21210_e21534, (locals.var_b1d_dn4 + (2.0 * locals.var_k1q1d_dn4)), (locals.var_b1d_dn6 + (2.0 * locals.var_k1q1d_dn6)), (locals.var_b1d_dn7 + (2.0 * locals.var_k1q1d_dn7)), (locals.var_b1d_dn8 + (2.0 * locals.var_k1q1d_dn8)), (locals.var_b1d_dn9 + (2.0 * locals.var_k1q1d_dn9)),)
    } else {
        (locals.var_a1d, locals.var_a1d_dn4, locals.var_a1d_dn6, locals.var_a1d_dn7, locals.var_a1d_dn8, locals.var_a1d_dn9,)
    }
};
        locals.var_a1d = assign21210_e21536;
        locals.var_a1d_dn4 = assign21210_e21536_d_n4;
        locals.var_a1d_dn6 = assign21210_e21536_d_n6;
        locals.var_a1d_dn7 = assign21210_e21536_d_n7;
        locals.var_a1d_dn8 = assign21210_e21536_d_n8;
        locals.var_a1d_dn9 = assign21210_e21536_d_n9;
        locals.var_a1d_rv = 0.0;

        let (assign21220_e21544, assign21220_e21544_d_n4, assign21220_e21544_d_n6, assign21220_e21544_d_n7, assign21220_e21544_d_n8, assign21220_e21544_d_n9,) = {
    if (locals.var_guard667 != 0.0) {
        let assign21220_e21541: f64 = (2.0 * locals.var_k2q2d);
        let assign21220_e21542: f64 = (locals.var_b2d + assign21220_e21541);
        (assign21220_e21542, (locals.var_b2d_dn4 + (2.0 * locals.var_k2q2d_dn4)), (locals.var_b2d_dn6 + (2.0 * locals.var_k2q2d_dn6)), (locals.var_b2d_dn7 + (2.0 * locals.var_k2q2d_dn7)), (locals.var_b2d_dn8 + (2.0 * locals.var_k2q2d_dn8)), (locals.var_b2d_dn9 + (2.0 * locals.var_k2q2d_dn9)),)
    } else {
        (locals.var_a2d, locals.var_a2d_dn4, locals.var_a2d_dn6, locals.var_a2d_dn7, locals.var_a2d_dn8, locals.var_a2d_dn9,)
    }
};
        locals.var_a2d = assign21220_e21544;
        locals.var_a2d_dn4 = assign21220_e21544_d_n4;
        locals.var_a2d_dn6 = assign21220_e21544_d_n6;
        locals.var_a2d_dn7 = assign21220_e21544_d_n7;
        locals.var_a2d_dn8 = assign21220_e21544_d_n8;
        locals.var_a2d_dn9 = assign21220_e21544_d_n9;
        locals.var_a2d_rv = 0.0;

        let (assign21230_e21554, assign21230_e21554_d_n4, assign21230_e21554_d_n6, assign21230_e21554_d_n7, assign21230_e21554_d_n8, assign21230_e21554_d_n9,) = {
    if (locals.var_guard667 != 0.0) {
        let assign21230_e21548: f64 = (2.0 * locals.var_qid);
        let assign21230_e21550: f64 = (assign21230_e21548 + locals.var_b1d);
        let assign21230_e21552: f64 = (assign21230_e21550 + locals.var_b2d);
        (assign21230_e21552, (((2.0 * locals.var_qid_dn4) + locals.var_b1d_dn4) + locals.var_b2d_dn4), (((2.0 * locals.var_qid_dn6) + locals.var_b1d_dn6) + locals.var_b2d_dn6), (((2.0 * locals.var_qid_dn7) + locals.var_b1d_dn7) + locals.var_b2d_dn7), (((2.0 * locals.var_qid_dn8) + locals.var_b1d_dn8) + locals.var_b2d_dn8), (((2.0 * locals.var_qid_dn9) + locals.var_b1d_dn9) + locals.var_b2d_dn9),)
    } else {
        (locals.var_sumd, locals.var_sumd_dn4, locals.var_sumd_dn6, locals.var_sumd_dn7, locals.var_sumd_dn8, locals.var_sumd_dn9,)
    }
};
        locals.var_sumd = assign21230_e21554;
        locals.var_sumd_dn4 = assign21230_e21554_d_n4;
        locals.var_sumd_dn6 = assign21230_e21554_d_n6;
        locals.var_sumd_dn7 = assign21230_e21554_d_n7;
        locals.var_sumd_dn8 = assign21230_e21554_d_n8;
        locals.var_sumd_dn9 = assign21230_e21554_d_n9;
        locals.var_sumd_rv = 0.0;

        let assign21240_e21556: f64 = (locals.var_qsqd).abs();
        let assign21240_e21558: f64 = if assign21240_e21556 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard668 = assign21240_e21558;
        locals.var_guard668_rv = 0.0;

        let (assign21250_e21582, assign21250_e21582_d_n4, assign21250_e21582_d_n6, assign21250_e21582_d_n7, assign21250_e21582_d_n8, assign21250_e21582_d_n9,) = {
    if ((locals.var_guard667 != 0.0) && (locals.var_guard668 != 0.0)) {
        let assign21250_e21564: f64 = (locals.var_a1d * locals.var_a2d);
        let assign21250_e21568: f64 = (locals.var_q1d + 2.0);
        let assign21250_e21569: f64 = (2.0 * assign21250_e21568);
        let assign21250_e21571: f64 = (assign21250_e21569 * locals.var_a2d);
        let assign21250_e21572: f64 = (assign21250_e21564 + assign21250_e21571);
        let assign21250_e21576: f64 = (locals.var_q2d + 2.0);
        let assign21250_e21577: f64 = (2.0 * assign21250_e21576);
        let assign21250_e21579: f64 = (assign21250_e21577 * locals.var_a1d);
        let assign21250_e21580: f64 = (assign21250_e21572 + assign21250_e21579);
        (assign21250_e21580, ((((locals.var_a1d_dn4 * locals.var_a2d) + (locals.var_a1d * locals.var_a2d_dn4)) + (((2.0 * locals.var_q1d_dn4) * locals.var_a2d) + (assign21250_e21569 * locals.var_a2d_dn4))) + (((2.0 * locals.var_q2d_dn4) * locals.var_a1d) + (assign21250_e21577 * locals.var_a1d_dn4))), ((((locals.var_a1d_dn6 * locals.var_a2d) + (locals.var_a1d * locals.var_a2d_dn6)) + (((2.0 * locals.var_q1d_dn6) * locals.var_a2d) + (assign21250_e21569 * locals.var_a2d_dn6))) + (((2.0 * locals.var_q2d_dn6) * locals.var_a1d) + (assign21250_e21577 * locals.var_a1d_dn6))), ((((locals.var_a1d_dn7 * locals.var_a2d) + (locals.var_a1d * locals.var_a2d_dn7)) + (((2.0 * locals.var_q1d_dn7) * locals.var_a2d) + (assign21250_e21569 * locals.var_a2d_dn7))) + (((2.0 * locals.var_q2d_dn7) * locals.var_a1d) + (assign21250_e21577 * locals.var_a1d_dn7))), ((((locals.var_a1d_dn8 * locals.var_a2d) + (locals.var_a1d * locals.var_a2d_dn8)) + (((2.0 * locals.var_q1d_dn8) * locals.var_a2d) + (assign21250_e21569 * locals.var_a2d_dn8))) + (((2.0 * locals.var_q2d_dn8) * locals.var_a1d) + (assign21250_e21577 * locals.var_a1d_dn8))), ((((locals.var_a1d_dn9 * locals.var_a2d) + (locals.var_a1d * locals.var_a2d_dn9)) + (((2.0 * locals.var_q1d_dn9) * locals.var_a2d) + (assign21250_e21569 * locals.var_a2d_dn9))) + (((2.0 * locals.var_q2d_dn9) * locals.var_a1d) + (assign21250_e21577 * locals.var_a1d_dn9))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign21250_e21582;
        locals.var_temp1_dn4 = assign21250_e21582_d_n4;
        locals.var_temp1_dn6 = assign21250_e21582_d_n6;
        locals.var_temp1_dn7 = assign21250_e21582_d_n7;
        locals.var_temp1_dn8 = assign21250_e21582_d_n8;
        locals.var_temp1_dn9 = assign21250_e21582_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign21260_e21597, assign21260_e21597_d_n4, assign21260_e21597_d_n6, assign21260_e21597_d_n7, assign21260_e21597_d_n8, assign21260_e21597_d_n9,) = {
    if ((locals.var_guard667 != 0.0) && (locals.var_guard668 != 0.0)) {
        let assign21260_e21587: f64 = (-4.0);
        let assign21260_e21589: f64 = (assign21260_e21587 * locals.var_qsqd);
        let assign21260_e21591: f64 = (assign21260_e21589 * locals.var_sumd);
        let assign21260_e21594: f64 = (locals.var_qid * locals.var_temp1);
        let assign21260_e21595: f64 = (assign21260_e21591 / assign21260_e21594);
        (assign21260_e21595, ((((((assign21260_e21587 * locals.var_qsqd_dn4) * locals.var_sumd) + (assign21260_e21589 * locals.var_sumd_dn4)) * assign21260_e21594) - (assign21260_e21591 * ((locals.var_qid_dn4 * locals.var_temp1) + (locals.var_qid * locals.var_temp1_dn4)))) / (assign21260_e21594 * assign21260_e21594)), ((((((assign21260_e21587 * locals.var_qsqd_dn6) * locals.var_sumd) + (assign21260_e21589 * locals.var_sumd_dn6)) * assign21260_e21594) - (assign21260_e21591 * ((locals.var_qid_dn6 * locals.var_temp1) + (locals.var_qid * locals.var_temp1_dn6)))) / (assign21260_e21594 * assign21260_e21594)), ((((((assign21260_e21587 * locals.var_qsqd_dn7) * locals.var_sumd) + (assign21260_e21589 * locals.var_sumd_dn7)) * assign21260_e21594) - (assign21260_e21591 * ((locals.var_qid_dn7 * locals.var_temp1) + (locals.var_qid * locals.var_temp1_dn7)))) / (assign21260_e21594 * assign21260_e21594)), ((((((assign21260_e21587 * locals.var_qsqd_dn8) * locals.var_sumd) + (assign21260_e21589 * locals.var_sumd_dn8)) * assign21260_e21594) - (assign21260_e21591 * ((locals.var_qid_dn8 * locals.var_temp1) + (locals.var_qid * locals.var_temp1_dn8)))) / (assign21260_e21594 * assign21260_e21594)), ((((((assign21260_e21587 * locals.var_qsqd_dn9) * locals.var_sumd) + (assign21260_e21589 * locals.var_sumd_dn9)) * assign21260_e21594) - (assign21260_e21591 * ((locals.var_qid_dn9 * locals.var_temp1) + (locals.var_qid * locals.var_temp1_dn9)))) / (assign21260_e21594 * assign21260_e21594)),)
    } else {
        (locals.var_dqsqd_dxn_qi, locals.var_dqsqd_dxn_qi_dn4, locals.var_dqsqd_dxn_qi_dn6, locals.var_dqsqd_dxn_qi_dn7, locals.var_dqsqd_dxn_qi_dn8, locals.var_dqsqd_dxn_qi_dn9,)
    }
};
        locals.var_dqsqd_dxn_qi = assign21260_e21597;
        locals.var_dqsqd_dxn_qi_dn4 = assign21260_e21597_d_n4;
        locals.var_dqsqd_dxn_qi_dn6 = assign21260_e21597_d_n6;
        locals.var_dqsqd_dxn_qi_dn7 = assign21260_e21597_d_n7;
        locals.var_dqsqd_dxn_qi_dn8 = assign21260_e21597_d_n8;
        locals.var_dqsqd_dxn_qi_dn9 = assign21260_e21597_d_n9;
        locals.var_dqsqd_dxn_qi_rv = 0.0;

        let (assign21270_e21622, assign21270_e21622_d_n4, assign21270_e21622_d_n6, assign21270_e21622_d_n7, assign21270_e21622_d_n8, assign21270_e21622_d_n9,) = {
    if ((locals.var_guard667 != 0.0) && (locals.var_guard668 == 0.0)) {
        let assign21270_e21606: f64 = (locals.var_qsqd * 0.0333333333333);
        let assign21270_e21610: f64 = (locals.var_qsqd * 0.0357142857143);
        let assign21270_e21614: f64 = (locals.var_qsqd * 0.0333333333333);
        let assign21270_e21615: f64 = (1.0 - assign21270_e21614);
        let assign21270_e21616: f64 = (assign21270_e21610 * assign21270_e21615);
        let assign21270_e21617: f64 = (1.0 - assign21270_e21616);
        let assign21270_e21618: f64 = (assign21270_e21606 * assign21270_e21617);
        let assign21270_e21619: f64 = (1.0 - assign21270_e21618);
        let assign21270_e21620: f64 = (0.1666666666667 * assign21270_e21619);
        (assign21270_e21620, (0.1666666666667 * (-(((locals.var_qsqd_dn4 * 0.0333333333333) * assign21270_e21617) + (assign21270_e21606 * (-(((locals.var_qsqd_dn4 * 0.0357142857143) * assign21270_e21615) + (assign21270_e21610 * (-(locals.var_qsqd_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_qsqd_dn6 * 0.0333333333333) * assign21270_e21617) + (assign21270_e21606 * (-(((locals.var_qsqd_dn6 * 0.0357142857143) * assign21270_e21615) + (assign21270_e21610 * (-(locals.var_qsqd_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_qsqd_dn7 * 0.0333333333333) * assign21270_e21617) + (assign21270_e21606 * (-(((locals.var_qsqd_dn7 * 0.0357142857143) * assign21270_e21615) + (assign21270_e21610 * (-(locals.var_qsqd_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_qsqd_dn8 * 0.0333333333333) * assign21270_e21617) + (assign21270_e21606 * (-(((locals.var_qsqd_dn8 * 0.0357142857143) * assign21270_e21615) + (assign21270_e21610 * (-(locals.var_qsqd_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_qsqd_dn9 * 0.0333333333333) * assign21270_e21617) + (assign21270_e21606 * (-(((locals.var_qsqd_dn9 * 0.0357142857143) * assign21270_e21615) + (assign21270_e21610 * (-(locals.var_qsqd_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign21270_e21622;
        locals.var_temp1_dn4 = assign21270_e21622_d_n4;
        locals.var_temp1_dn6 = assign21270_e21622_d_n6;
        locals.var_temp1_dn7 = assign21270_e21622_d_n7;
        locals.var_temp1_dn8 = assign21270_e21622_d_n8;
        locals.var_temp1_dn9 = assign21270_e21622_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign21280_e21647, assign21280_e21647_d_n4, assign21280_e21647_d_n6, assign21280_e21647_d_n7, assign21280_e21647_d_n8, assign21280_e21647_d_n9,) = {
    if ((locals.var_guard667 != 0.0) && (locals.var_guard668 == 0.0)) {
        let assign21280_e21629: f64 = (locals.var_a1d * locals.var_aexp1d);
        let assign21280_e21632: f64 = (locals.var_a2d * locals.var_aexp2d);
        let assign21280_e21633: f64 = (assign21280_e21629 + assign21280_e21632);
        let assign21280_e21636: f64 = (locals.var_a1d * locals.var_a2d);
        let assign21280_e21638: f64 = (assign21280_e21636 * locals.var_qid);
        let assign21280_e21642: f64 = (locals.var_qid * locals.var_temp1);
        let assign21280_e21643: f64 = (1.0 + assign21280_e21642);
        let assign21280_e21644: f64 = (assign21280_e21638 * assign21280_e21643);
        let assign21280_e21645: f64 = (assign21280_e21633 + assign21280_e21644);
        (assign21280_e21645, ((((locals.var_a1d_dn4 * locals.var_aexp1d) + (locals.var_a1d * locals.var_aexp1d_dn4)) + ((locals.var_a2d_dn4 * locals.var_aexp2d) + (locals.var_a2d * locals.var_aexp2d_dn4))) + ((((((locals.var_a1d_dn4 * locals.var_a2d) + (locals.var_a1d * locals.var_a2d_dn4)) * locals.var_qid) + (assign21280_e21636 * locals.var_qid_dn4)) * assign21280_e21643) + (assign21280_e21638 * ((locals.var_qid_dn4 * locals.var_temp1) + (locals.var_qid * locals.var_temp1_dn4))))), ((((locals.var_a1d_dn6 * locals.var_aexp1d) + (locals.var_a1d * locals.var_aexp1d_dn6)) + ((locals.var_a2d_dn6 * locals.var_aexp2d) + (locals.var_a2d * locals.var_aexp2d_dn6))) + ((((((locals.var_a1d_dn6 * locals.var_a2d) + (locals.var_a1d * locals.var_a2d_dn6)) * locals.var_qid) + (assign21280_e21636 * locals.var_qid_dn6)) * assign21280_e21643) + (assign21280_e21638 * ((locals.var_qid_dn6 * locals.var_temp1) + (locals.var_qid * locals.var_temp1_dn6))))), ((((locals.var_a1d_dn7 * locals.var_aexp1d) + (locals.var_a1d * locals.var_aexp1d_dn7)) + ((locals.var_a2d_dn7 * locals.var_aexp2d) + (locals.var_a2d * locals.var_aexp2d_dn7))) + ((((((locals.var_a1d_dn7 * locals.var_a2d) + (locals.var_a1d * locals.var_a2d_dn7)) * locals.var_qid) + (assign21280_e21636 * locals.var_qid_dn7)) * assign21280_e21643) + (assign21280_e21638 * ((locals.var_qid_dn7 * locals.var_temp1) + (locals.var_qid * locals.var_temp1_dn7))))), ((((locals.var_a1d_dn8 * locals.var_aexp1d) + (locals.var_a1d * locals.var_aexp1d_dn8)) + ((locals.var_a2d_dn8 * locals.var_aexp2d) + (locals.var_a2d * locals.var_aexp2d_dn8))) + ((((((locals.var_a1d_dn8 * locals.var_a2d) + (locals.var_a1d * locals.var_a2d_dn8)) * locals.var_qid) + (assign21280_e21636 * locals.var_qid_dn8)) * assign21280_e21643) + (assign21280_e21638 * ((locals.var_qid_dn8 * locals.var_temp1) + (locals.var_qid * locals.var_temp1_dn8))))), ((((locals.var_a1d_dn9 * locals.var_aexp1d) + (locals.var_a1d * locals.var_aexp1d_dn9)) + ((locals.var_a2d_dn9 * locals.var_aexp2d) + (locals.var_a2d * locals.var_aexp2d_dn9))) + ((((((locals.var_a1d_dn9 * locals.var_a2d) + (locals.var_a1d * locals.var_a2d_dn9)) * locals.var_qid) + (assign21280_e21636 * locals.var_qid_dn9)) * assign21280_e21643) + (assign21280_e21638 * ((locals.var_qid_dn9 * locals.var_temp1) + (locals.var_qid * locals.var_temp1_dn9))))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign21280_e21647;
        locals.var_temp2_dn4 = assign21280_e21647_d_n4;
        locals.var_temp2_dn6 = assign21280_e21647_d_n6;
        locals.var_temp2_dn7 = assign21280_e21647_d_n7;
        locals.var_temp2_dn8 = assign21280_e21647_d_n8;
        locals.var_temp2_dn9 = assign21280_e21647_d_n9;
        locals.var_temp2_rv = 0.0;

        let (assign21290_e21662, assign21290_e21662_d_n4, assign21290_e21662_d_n6, assign21290_e21662_d_n7, assign21290_e21662_d_n8, assign21290_e21662_d_n9,) = {
    if ((locals.var_guard667 != 0.0) && (locals.var_guard668 == 0.0)) {
        let assign21290_e21654: f64 = (locals.var_aexp1d * locals.var_aexp2d);
        let assign21290_e21656: f64 = (assign21290_e21654 * locals.var_sumd);
        let assign21290_e21659: f64 = (locals.var_qid * locals.var_temp2);
        let assign21290_e21660: f64 = (assign21290_e21656 / assign21290_e21659);
        (assign21290_e21660, (((((((locals.var_aexp1d_dn4 * locals.var_aexp2d) + (locals.var_aexp1d * locals.var_aexp2d_dn4)) * locals.var_sumd) + (assign21290_e21654 * locals.var_sumd_dn4)) * assign21290_e21659) - (assign21290_e21656 * ((locals.var_qid_dn4 * locals.var_temp2) + (locals.var_qid * locals.var_temp2_dn4)))) / (assign21290_e21659 * assign21290_e21659)), (((((((locals.var_aexp1d_dn6 * locals.var_aexp2d) + (locals.var_aexp1d * locals.var_aexp2d_dn6)) * locals.var_sumd) + (assign21290_e21654 * locals.var_sumd_dn6)) * assign21290_e21659) - (assign21290_e21656 * ((locals.var_qid_dn6 * locals.var_temp2) + (locals.var_qid * locals.var_temp2_dn6)))) / (assign21290_e21659 * assign21290_e21659)), (((((((locals.var_aexp1d_dn7 * locals.var_aexp2d) + (locals.var_aexp1d * locals.var_aexp2d_dn7)) * locals.var_sumd) + (assign21290_e21654 * locals.var_sumd_dn7)) * assign21290_e21659) - (assign21290_e21656 * ((locals.var_qid_dn7 * locals.var_temp2) + (locals.var_qid * locals.var_temp2_dn7)))) / (assign21290_e21659 * assign21290_e21659)), (((((((locals.var_aexp1d_dn8 * locals.var_aexp2d) + (locals.var_aexp1d * locals.var_aexp2d_dn8)) * locals.var_sumd) + (assign21290_e21654 * locals.var_sumd_dn8)) * assign21290_e21659) - (assign21290_e21656 * ((locals.var_qid_dn8 * locals.var_temp2) + (locals.var_qid * locals.var_temp2_dn8)))) / (assign21290_e21659 * assign21290_e21659)), (((((((locals.var_aexp1d_dn9 * locals.var_aexp2d) + (locals.var_aexp1d * locals.var_aexp2d_dn9)) * locals.var_sumd) + (assign21290_e21654 * locals.var_sumd_dn9)) * assign21290_e21659) - (assign21290_e21656 * ((locals.var_qid_dn9 * locals.var_temp2) + (locals.var_qid * locals.var_temp2_dn9)))) / (assign21290_e21659 * assign21290_e21659)),)
    } else {
        (locals.var_dqsqd_dxn_qi, locals.var_dqsqd_dxn_qi_dn4, locals.var_dqsqd_dxn_qi_dn6, locals.var_dqsqd_dxn_qi_dn7, locals.var_dqsqd_dxn_qi_dn8, locals.var_dqsqd_dxn_qi_dn9,)
    }
};
        locals.var_dqsqd_dxn_qi = assign21290_e21662;
        locals.var_dqsqd_dxn_qi_dn4 = assign21290_e21662_d_n4;
        locals.var_dqsqd_dxn_qi_dn6 = assign21290_e21662_d_n6;
        locals.var_dqsqd_dxn_qi_dn7 = assign21290_e21662_d_n7;
        locals.var_dqsqd_dxn_qi_dn8 = assign21290_e21662_d_n8;
        locals.var_dqsqd_dxn_qi_dn9 = assign21290_e21662_d_n9;
        locals.var_dqsqd_dxn_qi_rv = 0.0;

        let assign21300_e21665: f64 = (locals.var_qid).ln();
        let assign21300_e21666: f64 = (locals.var_xdeff + assign21300_e21665);
        locals.var_xdriftd = assign21300_e21666;
        locals.var_xdriftd_dn4 = (locals.var_xdeff_dn4 + (locals.var_qid_dn4 / locals.var_qid));
        locals.var_xdriftd_dn6 = (locals.var_xdeff_dn6 + (locals.var_qid_dn6 / locals.var_qid));
        locals.var_xdriftd_dn7 = (locals.var_xdeff_dn7 + (locals.var_qid_dn7 / locals.var_qid));
        locals.var_xdriftd_dn8 = (locals.var_xdeff_dn8 + (locals.var_qid_dn8 / locals.var_qid));
        locals.var_xdriftd_dn9 = (locals.var_xdeff_dn9 + (locals.var_qid_dn9 / locals.var_qid));
        locals.var_xdriftd_rv = 0.0;

        let assign21310_e21670: f64 = (locals.var_qis + locals.var_qid);
        let assign21310_e21671: f64 = (0.5 * assign21310_e21670);
        locals.var_qim = assign21310_e21671;
        locals.var_qim_dn4 = (0.5 * (locals.var_qis_dn4 + locals.var_qid_dn4));
        locals.var_qim_dn6 = (0.5 * (locals.var_qis_dn6 + locals.var_qid_dn6));
        locals.var_qim_dn7 = (0.5 * (locals.var_qis_dn7 + locals.var_qid_dn7));
        locals.var_qim_dn8 = (0.5 * (locals.var_qis_dn8 + locals.var_qid_dn8));
        locals.var_qim_dn9 = (0.5 * (locals.var_qis_dn9 + locals.var_qid_dn9));
        locals.var_qim_rv = 0.0;

        let assign21320_e21674: f64 = (locals.var_xdriftd - locals.var_xdrifts);
        locals.var_dxdrift = assign21320_e21674;
        locals.var_dxdrift_dn4 = (locals.var_xdriftd_dn4 - locals.var_xdrifts_dn4);
        locals.var_dxdrift_dn6 = (locals.var_xdriftd_dn6 - locals.var_xdrifts_dn6);
        locals.var_dxdrift_dn7 = (locals.var_xdriftd_dn7 - locals.var_xdrifts_dn7);
        locals.var_dxdrift_dn8 = (locals.var_xdriftd_dn8 - locals.var_xdrifts_dn8);
        locals.var_dxdrift_dn9 = (locals.var_xdriftd_dn9 - locals.var_xdrifts_dn9);
        locals.var_dxdrift_rv = 0.0;

        locals.var_ratio_pd = 1.0;
        locals.var_ratio_pd_dn4 = 0.0;
        locals.var_ratio_pd_dn6 = 0.0;
        locals.var_ratio_pd_dn7 = 0.0;
        locals.var_ratio_pd_dn8 = 0.0;
        locals.var_ratio_pd_dn9 = 0.0;
        locals.var_ratio_pd_rv = 0.0;

        let assign21340_e21678: f64 = if p.p9 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard669 = assign21340_e21678;
        locals.var_guard669_rv = 0.0;

        let (assign21350_e21688, assign21350_e21688_d_n4, assign21350_e21688_d_n6, assign21350_e21688_d_n7, assign21350_e21688_d_n8, assign21350_e21688_d_n9,) = {
    if (locals.var_guard669 != 0.0) {
        let assign21350_e21683: f64 = (locals.var_k1q1s + locals.var_k1q1d);
        let assign21350_e21684: f64 = (0.5 * assign21350_e21683);
        let assign21350_e21686: f64 = (assign21350_e21684 / locals.var_k1);
        (assign21350_e21686, ((((0.5 * (locals.var_k1q1s_dn4 + locals.var_k1q1d_dn4)) * locals.var_k1) - (assign21350_e21684 * locals.var_k1_dn4)) / (locals.var_k1 * locals.var_k1)), ((((0.5 * (locals.var_k1q1s_dn6 + locals.var_k1q1d_dn6)) * locals.var_k1) - (assign21350_e21684 * locals.var_k1_dn6)) / (locals.var_k1 * locals.var_k1)), ((((0.5 * (locals.var_k1q1s_dn7 + locals.var_k1q1d_dn7)) * locals.var_k1) - (assign21350_e21684 * locals.var_k1_dn7)) / (locals.var_k1 * locals.var_k1)), ((((0.5 * (locals.var_k1q1s_dn8 + locals.var_k1q1d_dn8)) * locals.var_k1) - (assign21350_e21684 * locals.var_k1_dn8)) / (locals.var_k1 * locals.var_k1)), ((((0.5 * (locals.var_k1q1s_dn9 + locals.var_k1q1d_dn9)) * locals.var_k1) - (assign21350_e21684 * locals.var_k1_dn9)) / (locals.var_k1 * locals.var_k1)),)
    } else {
        (locals.var_qim_pd, locals.var_qim_pd_dn4, locals.var_qim_pd_dn6, locals.var_qim_pd_dn7, locals.var_qim_pd_dn8, locals.var_qim_pd_dn9,)
    }
};
        locals.var_qim_pd = assign21350_e21688;
        locals.var_qim_pd_dn4 = assign21350_e21688_d_n4;
        locals.var_qim_pd_dn6 = assign21350_e21688_d_n6;
        locals.var_qim_pd_dn7 = assign21350_e21688_d_n7;
        locals.var_qim_pd_dn8 = assign21350_e21688_d_n8;
        locals.var_qim_pd_dn9 = assign21350_e21688_d_n9;
        locals.var_qim_pd_rv = 0.0;

        let (assign21360_e21707, assign21360_e21707_d_n4, assign21360_e21707_d_n6, assign21360_e21707_d_n7, assign21360_e21707_d_n8, assign21360_e21707_d_n9,) = {
    if (locals.var_guard669 != 0.0) {
        let assign21360_e21693: f64 = (locals.var_qim_pd + 1e-5);
        let assign21360_e21696: f64 = (locals.var_qim_pd - 1e-5);
        let assign21360_e21699: f64 = (locals.var_qim_pd - 1e-5);
        let assign21360_e21700: f64 = (assign21360_e21696 * assign21360_e21699);
        let assign21360_e21702: f64 = (assign21360_e21700 + 1.0);
        let assign21360_e21703: f64 = (assign21360_e21702).sqrt();
        let assign21360_e21704: f64 = (assign21360_e21693 + assign21360_e21703);
        let assign21360_e21705: f64 = (0.5 * assign21360_e21704);
        (assign21360_e21705, (0.5 * (locals.var_qim_pd_dn4 + (((locals.var_qim_pd_dn4 * assign21360_e21699) + (assign21360_e21696 * locals.var_qim_pd_dn4)) / (2.0 * assign21360_e21703)))), (0.5 * (locals.var_qim_pd_dn6 + (((locals.var_qim_pd_dn6 * assign21360_e21699) + (assign21360_e21696 * locals.var_qim_pd_dn6)) / (2.0 * assign21360_e21703)))), (0.5 * (locals.var_qim_pd_dn7 + (((locals.var_qim_pd_dn7 * assign21360_e21699) + (assign21360_e21696 * locals.var_qim_pd_dn7)) / (2.0 * assign21360_e21703)))), (0.5 * (locals.var_qim_pd_dn8 + (((locals.var_qim_pd_dn8 * assign21360_e21699) + (assign21360_e21696 * locals.var_qim_pd_dn8)) / (2.0 * assign21360_e21703)))), (0.5 * (locals.var_qim_pd_dn9 + (((locals.var_qim_pd_dn9 * assign21360_e21699) + (assign21360_e21696 * locals.var_qim_pd_dn9)) / (2.0 * assign21360_e21703)))),)
    } else {
        (locals.var_qim_pd, locals.var_qim_pd_dn4, locals.var_qim_pd_dn6, locals.var_qim_pd_dn7, locals.var_qim_pd_dn8, locals.var_qim_pd_dn9,)
    }
};
        locals.var_qim_pd = assign21360_e21707;
        locals.var_qim_pd_dn4 = assign21360_e21707_d_n4;
        locals.var_qim_pd_dn6 = assign21360_e21707_d_n6;
        locals.var_qim_pd_dn7 = assign21360_e21707_d_n7;
        locals.var_qim_pd_dn8 = assign21360_e21707_d_n8;
        locals.var_qim_pd_dn9 = assign21360_e21707_d_n9;
        locals.var_qim_pd_rv = 0.0;

        let (assign21370_e21724, assign21370_e21724_d_n4, assign21370_e21724_d_n6, assign21370_e21724_d_n7, assign21370_e21724_d_n8, assign21370_e21724_d_n9,) = {
    if (locals.var_guard669 != 0.0) {
        let assign21370_e21711: f64 = (locals.var_qim_pd / locals.var_inv_phit);
        let assign21370_e21714: f64 = (0.25 * locals.var_kp);
        let assign21370_e21716: f64 = (assign21370_e21714 * locals.var_kp);
        let assign21370_e21717: f64 = (assign21370_e21711 + assign21370_e21716);
        let assign21370_e21718: f64 = (assign21370_e21717).sqrt();
        let assign21370_e21721: f64 = (0.5 * locals.var_kp);
        let assign21370_e21722: f64 = (assign21370_e21718 - assign21370_e21721);
        (assign21370_e21722, ((((((locals.var_qim_pd_dn4 * locals.var_inv_phit) - (locals.var_qim_pd * locals.var_inv_phit_dn4)) / (locals.var_inv_phit * locals.var_inv_phit)) + (((0.25 * locals.var_kp_dn4) * locals.var_kp) + (assign21370_e21714 * locals.var_kp_dn4))) / (2.0 * assign21370_e21718)) - (0.5 * locals.var_kp_dn4)), ((((((locals.var_qim_pd_dn6 * locals.var_inv_phit) - (locals.var_qim_pd * locals.var_inv_phit_dn6)) / (locals.var_inv_phit * locals.var_inv_phit)) + (((0.25 * locals.var_kp_dn6) * locals.var_kp) + (assign21370_e21714 * locals.var_kp_dn6))) / (2.0 * assign21370_e21718)) - (0.5 * locals.var_kp_dn6)), ((((((locals.var_qim_pd_dn7 * locals.var_inv_phit) - (locals.var_qim_pd * locals.var_inv_phit_dn7)) / (locals.var_inv_phit * locals.var_inv_phit)) + (((0.25 * locals.var_kp_dn7) * locals.var_kp) + (assign21370_e21714 * locals.var_kp_dn7))) / (2.0 * assign21370_e21718)) - (0.5 * locals.var_kp_dn7)), ((((((locals.var_qim_pd_dn8 * locals.var_inv_phit) - (locals.var_qim_pd * locals.var_inv_phit_dn8)) / (locals.var_inv_phit * locals.var_inv_phit)) + (((0.25 * locals.var_kp_dn8) * locals.var_kp) + (assign21370_e21714 * locals.var_kp_dn8))) / (2.0 * assign21370_e21718)) - (0.5 * locals.var_kp_dn8)), ((((((locals.var_qim_pd_dn9 * locals.var_inv_phit) - (locals.var_qim_pd * locals.var_inv_phit_dn9)) / (locals.var_inv_phit * locals.var_inv_phit)) + (((0.25 * locals.var_kp_dn9) * locals.var_kp) + (assign21370_e21714 * locals.var_kp_dn9))) / (2.0 * assign21370_e21718)) - (0.5 * locals.var_kp_dn9)),)
    } else {
        (locals.var_temp0, locals.var_temp0_dn4, locals.var_temp0_dn6, locals.var_temp0_dn7, locals.var_temp0_dn8, locals.var_temp0_dn9,)
    }
};
        locals.var_temp0 = assign21370_e21724;
        locals.var_temp0_dn4 = assign21370_e21724_d_n4;
        locals.var_temp0_dn6 = assign21370_e21724_d_n6;
        locals.var_temp0_dn7 = assign21370_e21724_d_n7;
        locals.var_temp0_dn8 = assign21370_e21724_d_n8;
        locals.var_temp0_dn9 = assign21370_e21724_d_n9;
        locals.var_temp0_rv = 0.0;

        let (assign21380_e21732, assign21380_e21732_d_n4, assign21380_e21732_d_n6, assign21380_e21732_d_n7, assign21380_e21732_d_n8, assign21380_e21732_d_n9,) = {
    if (locals.var_guard669 != 0.0) {
        let assign21380_e21728: f64 = (locals.var_temp0).powf(2.0);
        let assign21380_e21730: f64 = (assign21380_e21728 * locals.var_inv_phit);
        (assign21380_e21730, ((if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_temp0).powf(2.0 - 1.0) * locals.var_temp0_dn4)) } } else { (assign21380_e21728 * (2.0 * (locals.var_temp0_dn4 / locals.var_temp0))) } * locals.var_inv_phit) + (assign21380_e21728 * locals.var_inv_phit_dn4)), ((if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_temp0).powf(2.0 - 1.0) * locals.var_temp0_dn6)) } } else { (assign21380_e21728 * (2.0 * (locals.var_temp0_dn6 / locals.var_temp0))) } * locals.var_inv_phit) + (assign21380_e21728 * locals.var_inv_phit_dn6)), ((if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_temp0).powf(2.0 - 1.0) * locals.var_temp0_dn7)) } } else { (assign21380_e21728 * (2.0 * (locals.var_temp0_dn7 / locals.var_temp0))) } * locals.var_inv_phit) + (assign21380_e21728 * locals.var_inv_phit_dn7)), ((if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_temp0).powf(2.0 - 1.0) * locals.var_temp0_dn8)) } } else { (assign21380_e21728 * (2.0 * (locals.var_temp0_dn8 / locals.var_temp0))) } * locals.var_inv_phit) + (assign21380_e21728 * locals.var_inv_phit_dn8)), ((if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_temp0).powf(2.0 - 1.0) * locals.var_temp0_dn9)) } } else { (assign21380_e21728 * (2.0 * (locals.var_temp0_dn9 / locals.var_temp0))) } * locals.var_inv_phit) + (assign21380_e21728 * locals.var_inv_phit_dn9)),)
    } else {
        (locals.var_xp_pd, locals.var_xp_pd_dn4, locals.var_xp_pd_dn6, locals.var_xp_pd_dn7, locals.var_xp_pd_dn8, locals.var_xp_pd_dn9,)
    }
};
        locals.var_xp_pd = assign21380_e21732;
        locals.var_xp_pd_dn4 = assign21380_e21732_d_n4;
        locals.var_xp_pd_dn6 = assign21380_e21732_d_n6;
        locals.var_xp_pd_dn7 = assign21380_e21732_d_n7;
        locals.var_xp_pd_dn8 = assign21380_e21732_d_n8;
        locals.var_xp_pd_dn9 = assign21380_e21732_d_n9;
        locals.var_xp_pd_rv = 0.0;

        let (assign21390_e21740, assign21390_e21740_d_n4, assign21390_e21740_d_n6, assign21390_e21740_d_n7, assign21390_e21740_d_n8, assign21390_e21740_d_n9,) = {
    if (locals.var_guard669 != 0.0) {
        let assign21390_e21737: f64 = (locals.var_xp_pd / locals.var_qim_pd);
        let assign21390_e21738: f64 = (1.0 - assign21390_e21737);
        (assign21390_e21738, (-(((locals.var_xp_pd_dn4 * locals.var_qim_pd) - (locals.var_xp_pd * locals.var_qim_pd_dn4)) / (locals.var_qim_pd * locals.var_qim_pd))), (-(((locals.var_xp_pd_dn6 * locals.var_qim_pd) - (locals.var_xp_pd * locals.var_qim_pd_dn6)) / (locals.var_qim_pd * locals.var_qim_pd))), (-(((locals.var_xp_pd_dn7 * locals.var_qim_pd) - (locals.var_xp_pd * locals.var_qim_pd_dn7)) / (locals.var_qim_pd * locals.var_qim_pd))), (-(((locals.var_xp_pd_dn8 * locals.var_qim_pd) - (locals.var_xp_pd * locals.var_qim_pd_dn8)) / (locals.var_qim_pd * locals.var_qim_pd))), (-(((locals.var_xp_pd_dn9 * locals.var_qim_pd) - (locals.var_xp_pd * locals.var_qim_pd_dn9)) / (locals.var_qim_pd * locals.var_qim_pd))),)
    } else {
        (locals.var_ratio_pd, locals.var_ratio_pd_dn4, locals.var_ratio_pd_dn6, locals.var_ratio_pd_dn7, locals.var_ratio_pd_dn8, locals.var_ratio_pd_dn9,)
    }
};
        locals.var_ratio_pd = assign21390_e21740;
        locals.var_ratio_pd_dn4 = assign21390_e21740_d_n4;
        locals.var_ratio_pd_dn6 = assign21390_e21740_d_n6;
        locals.var_ratio_pd_dn7 = assign21390_e21740_d_n7;
        locals.var_ratio_pd_dn8 = assign21390_e21740_d_n8;
        locals.var_ratio_pd_dn9 = assign21390_e21740_d_n9;
        locals.var_ratio_pd_rv = 0.0;

        let assign21400_e21743: f64 = (locals.var_k1q1d / 2.0);
        let assign21400_e21745: f64 = if assign21400_e21743 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard670 = assign21400_e21745;
        locals.var_guard670_rv = 0.0;

        let (assign21410_e21755, assign21410_e21755_d_n4, assign21410_e21755_d_n6, assign21410_e21755_d_n7, assign21410_e21755_d_n8, assign21410_e21755_d_n9,) = {
    if (locals.var_guard670 != 0.0) {
        let assign21410_e21750: f64 = (locals.var_k1q1d / 2.0);
        let assign21410_e21751: f64 = (assign21410_e21750).exp();
        let assign21410_e21752: f64 = (1.0 + assign21410_e21751);
        let assign21410_e21753: f64 = (assign21410_e21752).ln();
        (assign21410_e21753, ((assign21410_e21751 * (locals.var_k1q1d_dn4 / 2.0)) / assign21410_e21752), ((assign21410_e21751 * (locals.var_k1q1d_dn6 / 2.0)) / assign21410_e21752), ((assign21410_e21751 * (locals.var_k1q1d_dn7 / 2.0)) / assign21410_e21752), ((assign21410_e21751 * (locals.var_k1q1d_dn8 / 2.0)) / assign21410_e21752), ((assign21410_e21751 * (locals.var_k1q1d_dn9 / 2.0)) / assign21410_e21752),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign21410_e21755;
        locals.var_temp1_dn4 = assign21410_e21755_d_n4;
        locals.var_temp1_dn6 = assign21410_e21755_d_n6;
        locals.var_temp1_dn7 = assign21410_e21755_d_n7;
        locals.var_temp1_dn8 = assign21410_e21755_d_n8;
        locals.var_temp1_dn9 = assign21410_e21755_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign21420_e21762, assign21420_e21762_d_n4, assign21420_e21762_d_n6, assign21420_e21762_d_n7, assign21420_e21762_d_n8, assign21420_e21762_d_n9,) = {
    if (locals.var_guard670 == 0.0) {
        let assign21420_e21760: f64 = (locals.var_k1q1d / 2.0);
        (assign21420_e21760, (locals.var_k1q1d_dn4 / 2.0), (locals.var_k1q1d_dn6 / 2.0), (locals.var_k1q1d_dn7 / 2.0), (locals.var_k1q1d_dn8 / 2.0), (locals.var_k1q1d_dn9 / 2.0),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign21420_e21762;
        locals.var_temp1_dn4 = assign21420_e21762_d_n4;
        locals.var_temp1_dn6 = assign21420_e21762_d_n6;
        locals.var_temp1_dn7 = assign21420_e21762_d_n7;
        locals.var_temp1_dn8 = assign21420_e21762_d_n8;
        locals.var_temp1_dn9 = assign21420_e21762_d_n9;
        locals.var_temp1_rv = 0.0;

        let assign21430_e21765: f64 = (2.0 * locals.var_temp1);
        locals.var_esurf1d = assign21430_e21765;
        locals.var_esurf1d_dn4 = (2.0 * locals.var_temp1_dn4);
        locals.var_esurf1d_dn6 = (2.0 * locals.var_temp1_dn6);
        locals.var_esurf1d_dn7 = (2.0 * locals.var_temp1_dn7);
        locals.var_esurf1d_dn8 = (2.0 * locals.var_temp1_dn8);
        locals.var_esurf1d_dn9 = (2.0 * locals.var_temp1_dn9);
        locals.var_esurf1d_rv = 0.0;

        let assign21440_e21768: f64 = (locals.var_k2q2d / 2.0);
        let assign21440_e21770: f64 = if assign21440_e21768 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard671 = assign21440_e21770;
        locals.var_guard671_rv = 0.0;

        let (assign21450_e21780, assign21450_e21780_d_n4, assign21450_e21780_d_n6, assign21450_e21780_d_n7, assign21450_e21780_d_n8, assign21450_e21780_d_n9,) = {
    if (locals.var_guard671 != 0.0) {
        let assign21450_e21775: f64 = (locals.var_k2q2d / 2.0);
        let assign21450_e21776: f64 = (assign21450_e21775).exp();
        let assign21450_e21777: f64 = (1.0 + assign21450_e21776);
        let assign21450_e21778: f64 = (assign21450_e21777).ln();
        (assign21450_e21778, ((assign21450_e21776 * (locals.var_k2q2d_dn4 / 2.0)) / assign21450_e21777), ((assign21450_e21776 * (locals.var_k2q2d_dn6 / 2.0)) / assign21450_e21777), ((assign21450_e21776 * (locals.var_k2q2d_dn7 / 2.0)) / assign21450_e21777), ((assign21450_e21776 * (locals.var_k2q2d_dn8 / 2.0)) / assign21450_e21777), ((assign21450_e21776 * (locals.var_k2q2d_dn9 / 2.0)) / assign21450_e21777),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign21450_e21780;
        locals.var_temp2_dn4 = assign21450_e21780_d_n4;
        locals.var_temp2_dn6 = assign21450_e21780_d_n6;
        locals.var_temp2_dn7 = assign21450_e21780_d_n7;
        locals.var_temp2_dn8 = assign21450_e21780_d_n8;
        locals.var_temp2_dn9 = assign21450_e21780_d_n9;
        locals.var_temp2_rv = 0.0;

        let (assign21460_e21787, assign21460_e21787_d_n4, assign21460_e21787_d_n6, assign21460_e21787_d_n7, assign21460_e21787_d_n8, assign21460_e21787_d_n9,) = {
    if (locals.var_guard671 == 0.0) {
        let assign21460_e21785: f64 = (locals.var_k2q2d / 2.0);
        (assign21460_e21785, (locals.var_k2q2d_dn4 / 2.0), (locals.var_k2q2d_dn6 / 2.0), (locals.var_k2q2d_dn7 / 2.0), (locals.var_k2q2d_dn8 / 2.0), (locals.var_k2q2d_dn9 / 2.0),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign21460_e21787;
        locals.var_temp2_dn4 = assign21460_e21787_d_n4;
        locals.var_temp2_dn6 = assign21460_e21787_d_n6;
        locals.var_temp2_dn7 = assign21460_e21787_d_n7;
        locals.var_temp2_dn8 = assign21460_e21787_d_n8;
        locals.var_temp2_dn9 = assign21460_e21787_d_n9;
        locals.var_temp2_rv = 0.0;

        let assign21470_e21790: f64 = (2.0 * locals.var_temp2);
        locals.var_esurf2d = assign21470_e21790;
        locals.var_esurf2d_dn4 = (2.0 * locals.var_temp2_dn4);
        locals.var_esurf2d_dn6 = (2.0 * locals.var_temp2_dn6);
        locals.var_esurf2d_dn7 = (2.0 * locals.var_temp2_dn7);
        locals.var_esurf2d_dn8 = (2.0 * locals.var_temp2_dn8);
        locals.var_esurf2d_dn9 = (2.0 * locals.var_temp2_dn9);
        locals.var_esurf2d_rv = 0.0;

        let assign21480_e21793: f64 = (locals.var_esurf2d - locals.var_k2q2d);
        locals.var_ecpl1d = assign21480_e21793;
        locals.var_ecpl1d_dn4 = (locals.var_esurf2d_dn4 - locals.var_k2q2d_dn4);
        locals.var_ecpl1d_dn6 = (locals.var_esurf2d_dn6 - locals.var_k2q2d_dn6);
        locals.var_ecpl1d_dn7 = (locals.var_esurf2d_dn7 - locals.var_k2q2d_dn7);
        locals.var_ecpl1d_dn8 = (locals.var_esurf2d_dn8 - locals.var_k2q2d_dn8);
        locals.var_ecpl1d_dn9 = (locals.var_esurf2d_dn9 - locals.var_k2q2d_dn9);
        locals.var_ecpl1d_rv = 0.0;

        let assign21490_e21796: f64 = (locals.var_esurf1d - locals.var_k1q1d);
        locals.var_ecpl2d = assign21490_e21796;
        locals.var_ecpl2d_dn4 = (locals.var_esurf1d_dn4 - locals.var_k1q1d_dn4);
        locals.var_ecpl2d_dn6 = (locals.var_esurf1d_dn6 - locals.var_k1q1d_dn6);
        locals.var_ecpl2d_dn7 = (locals.var_esurf1d_dn7 - locals.var_k1q1d_dn7);
        locals.var_ecpl2d_dn8 = (locals.var_esurf1d_dn8 - locals.var_k1q1d_dn8);
        locals.var_ecpl2d_dn9 = (locals.var_esurf1d_dn9 - locals.var_k1q1d_dn9);
        locals.var_ecpl2d_rv = 0.0;

        let assign21500_e21799: f64 = (locals.var_eta_mu * locals.var_esurf1d);
        let assign21500_e21802: f64 = (locals.var_one_m_eta * locals.var_ecpl1d);
        let assign21500_e21803: f64 = (assign21500_e21799 + assign21500_e21802);
        locals.var_eeff1d = assign21500_e21803;
        locals.var_eeff1d_dn4 = ((locals.var_eta_mu * locals.var_esurf1d_dn4) + (locals.var_one_m_eta * locals.var_ecpl1d_dn4));
        locals.var_eeff1d_dn6 = ((locals.var_eta_mu * locals.var_esurf1d_dn6) + (locals.var_one_m_eta * locals.var_ecpl1d_dn6));
        locals.var_eeff1d_dn7 = ((locals.var_eta_mu * locals.var_esurf1d_dn7) + (locals.var_one_m_eta * locals.var_ecpl1d_dn7));
        locals.var_eeff1d_dn8 = ((locals.var_eta_mu * locals.var_esurf1d_dn8) + (locals.var_one_m_eta * locals.var_ecpl1d_dn8));
        locals.var_eeff1d_dn9 = ((locals.var_eta_mu * locals.var_esurf1d_dn9) + (locals.var_one_m_eta * locals.var_ecpl1d_dn9));
        locals.var_eeff1d_rv = 0.0;

        let assign21510_e21806: f64 = (locals.var_eta_mu * locals.var_esurf2d);
        let assign21510_e21809: f64 = (locals.var_one_m_eta * locals.var_ecpl2d);
        let assign21510_e21810: f64 = (assign21510_e21806 + assign21510_e21809);
        locals.var_eeff2d = assign21510_e21810;
        locals.var_eeff2d_dn4 = ((locals.var_eta_mu * locals.var_esurf2d_dn4) + (locals.var_one_m_eta * locals.var_ecpl2d_dn4));
        locals.var_eeff2d_dn6 = ((locals.var_eta_mu * locals.var_esurf2d_dn6) + (locals.var_one_m_eta * locals.var_ecpl2d_dn6));
        locals.var_eeff2d_dn7 = ((locals.var_eta_mu * locals.var_esurf2d_dn7) + (locals.var_one_m_eta * locals.var_ecpl2d_dn7));
        locals.var_eeff2d_dn8 = ((locals.var_eta_mu * locals.var_esurf2d_dn8) + (locals.var_one_m_eta * locals.var_ecpl2d_dn8));
        locals.var_eeff2d_dn9 = ((locals.var_eta_mu * locals.var_esurf2d_dn9) + (locals.var_one_m_eta * locals.var_ecpl2d_dn9));
        locals.var_eeff2d_rv = 0.0;

        let assign21520_e21814: f64 = (locals.var_esurf1s + locals.var_esurf1d);
        let assign21520_e21815: f64 = (0.5 * assign21520_e21814);
        locals.var_esurf1 = assign21520_e21815;
        locals.var_esurf1_dn4 = (0.5 * (locals.var_esurf1s_dn4 + locals.var_esurf1d_dn4));
        locals.var_esurf1_dn6 = (0.5 * (locals.var_esurf1s_dn6 + locals.var_esurf1d_dn6));
        locals.var_esurf1_dn7 = (0.5 * (locals.var_esurf1s_dn7 + locals.var_esurf1d_dn7));
        locals.var_esurf1_dn8 = (0.5 * (locals.var_esurf1s_dn8 + locals.var_esurf1d_dn8));
        locals.var_esurf1_dn9 = (0.5 * (locals.var_esurf1s_dn9 + locals.var_esurf1d_dn9));
        locals.var_esurf1_rv = 0.0;

        let assign21530_e21819: f64 = (locals.var_esurf2s + locals.var_esurf2d);
        let assign21530_e21820: f64 = (0.5 * assign21530_e21819);
        locals.var_esurf2 = assign21530_e21820;
        locals.var_esurf2_dn4 = (0.5 * (locals.var_esurf2s_dn4 + locals.var_esurf2d_dn4));
        locals.var_esurf2_dn6 = (0.5 * (locals.var_esurf2s_dn6 + locals.var_esurf2d_dn6));
        locals.var_esurf2_dn7 = (0.5 * (locals.var_esurf2s_dn7 + locals.var_esurf2d_dn7));
        locals.var_esurf2_dn8 = (0.5 * (locals.var_esurf2s_dn8 + locals.var_esurf2d_dn8));
        locals.var_esurf2_dn9 = (0.5 * (locals.var_esurf2s_dn9 + locals.var_esurf2d_dn9));
        locals.var_esurf2_rv = 0.0;

        let assign21540_e21824: f64 = (locals.var_esurf1 + locals.var_esurf2);
        let assign21540_e21825: f64 = (1.0 / assign21540_e21824);
        locals.var_temp = assign21540_e21825;
        locals.var_temp_dn4 = (-((locals.var_esurf1_dn4 + locals.var_esurf2_dn4) / (assign21540_e21824 * assign21540_e21824)));
        locals.var_temp_dn6 = (-((locals.var_esurf1_dn6 + locals.var_esurf2_dn6) / (assign21540_e21824 * assign21540_e21824)));
        locals.var_temp_dn7 = (-((locals.var_esurf1_dn7 + locals.var_esurf2_dn7) / (assign21540_e21824 * assign21540_e21824)));
        locals.var_temp_dn8 = (-((locals.var_esurf1_dn8 + locals.var_esurf2_dn8) / (assign21540_e21824 * assign21540_e21824)));
        locals.var_temp_dn9 = (-((locals.var_esurf1_dn9 + locals.var_esurf2_dn9) / (assign21540_e21824 * assign21540_e21824)));
        locals.var_temp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_59(
        locals: &mut StampLocals,
    ) {
        let assign21550_e21828: f64 = (locals.var_qim * locals.var_esurf1);
        let assign21550_e21830: f64 = (assign21550_e21828 * locals.var_temp);
        locals.var_qi1m = assign21550_e21830;
        locals.var_qi1m_dn4 = ((((locals.var_qim_dn4 * locals.var_esurf1) + (locals.var_qim * locals.var_esurf1_dn4)) * locals.var_temp) + (assign21550_e21828 * locals.var_temp_dn4));
        locals.var_qi1m_dn6 = ((((locals.var_qim_dn6 * locals.var_esurf1) + (locals.var_qim * locals.var_esurf1_dn6)) * locals.var_temp) + (assign21550_e21828 * locals.var_temp_dn6));
        locals.var_qi1m_dn7 = ((((locals.var_qim_dn7 * locals.var_esurf1) + (locals.var_qim * locals.var_esurf1_dn7)) * locals.var_temp) + (assign21550_e21828 * locals.var_temp_dn7));
        locals.var_qi1m_dn8 = ((((locals.var_qim_dn8 * locals.var_esurf1) + (locals.var_qim * locals.var_esurf1_dn8)) * locals.var_temp) + (assign21550_e21828 * locals.var_temp_dn8));
        locals.var_qi1m_dn9 = ((((locals.var_qim_dn9 * locals.var_esurf1) + (locals.var_qim * locals.var_esurf1_dn9)) * locals.var_temp) + (assign21550_e21828 * locals.var_temp_dn9));
        locals.var_qi1m_rv = 0.0;

        let assign21560_e21833: f64 = (locals.var_qim * locals.var_esurf2);
        let assign21560_e21835: f64 = (assign21560_e21833 * locals.var_temp);
        locals.var_qi2m = assign21560_e21835;
        locals.var_qi2m_dn4 = ((((locals.var_qim_dn4 * locals.var_esurf2) + (locals.var_qim * locals.var_esurf2_dn4)) * locals.var_temp) + (assign21560_e21833 * locals.var_temp_dn4));
        locals.var_qi2m_dn6 = ((((locals.var_qim_dn6 * locals.var_esurf2) + (locals.var_qim * locals.var_esurf2_dn6)) * locals.var_temp) + (assign21560_e21833 * locals.var_temp_dn6));
        locals.var_qi2m_dn7 = ((((locals.var_qim_dn7 * locals.var_esurf2) + (locals.var_qim * locals.var_esurf2_dn7)) * locals.var_temp) + (assign21560_e21833 * locals.var_temp_dn7));
        locals.var_qi2m_dn8 = ((((locals.var_qim_dn8 * locals.var_esurf2) + (locals.var_qim * locals.var_esurf2_dn8)) * locals.var_temp) + (assign21560_e21833 * locals.var_temp_dn8));
        locals.var_qi2m_dn9 = ((((locals.var_qim_dn9 * locals.var_esurf2) + (locals.var_qim * locals.var_esurf2_dn9)) * locals.var_temp) + (assign21560_e21833 * locals.var_temp_dn9));
        locals.var_qi2m_rv = 0.0;

        let assign21570_e21839: f64 = (locals.var_ecpl1s + locals.var_ecpl1d);
        let assign21570_e21840: f64 = (0.5 * assign21570_e21839);
        locals.var_ecpl1 = assign21570_e21840;
        locals.var_ecpl1_dn4 = (0.5 * (locals.var_ecpl1s_dn4 + locals.var_ecpl1d_dn4));
        locals.var_ecpl1_dn6 = (0.5 * (locals.var_ecpl1s_dn6 + locals.var_ecpl1d_dn6));
        locals.var_ecpl1_dn7 = (0.5 * (locals.var_ecpl1s_dn7 + locals.var_ecpl1d_dn7));
        locals.var_ecpl1_dn8 = (0.5 * (locals.var_ecpl1s_dn8 + locals.var_ecpl1d_dn8));
        locals.var_ecpl1_dn9 = (0.5 * (locals.var_ecpl1s_dn9 + locals.var_ecpl1d_dn9));
        locals.var_ecpl1_rv = 0.0;

        let assign21580_e21844: f64 = (locals.var_ecpl2s + locals.var_ecpl2d);
        let assign21580_e21845: f64 = (0.5 * assign21580_e21844);
        locals.var_ecpl2 = assign21580_e21845;
        locals.var_ecpl2_dn4 = (0.5 * (locals.var_ecpl2s_dn4 + locals.var_ecpl2d_dn4));
        locals.var_ecpl2_dn6 = (0.5 * (locals.var_ecpl2s_dn6 + locals.var_ecpl2d_dn6));
        locals.var_ecpl2_dn7 = (0.5 * (locals.var_ecpl2s_dn7 + locals.var_ecpl2d_dn7));
        locals.var_ecpl2_dn8 = (0.5 * (locals.var_ecpl2s_dn8 + locals.var_ecpl2d_dn8));
        locals.var_ecpl2_dn9 = (0.5 * (locals.var_ecpl2s_dn9 + locals.var_ecpl2d_dn9));
        locals.var_ecpl2_rv = 0.0;

        let assign21590_e21849: f64 = (locals.var_eeff1s + locals.var_eeff1d);
        let assign21590_e21850: f64 = (0.5 * assign21590_e21849);
        locals.var_eeff1 = assign21590_e21850;
        locals.var_eeff1_dn4 = (0.5 * (locals.var_eeff1s_dn4 + locals.var_eeff1d_dn4));
        locals.var_eeff1_dn6 = (0.5 * (locals.var_eeff1s_dn6 + locals.var_eeff1d_dn6));
        locals.var_eeff1_dn7 = (0.5 * (locals.var_eeff1s_dn7 + locals.var_eeff1d_dn7));
        locals.var_eeff1_dn8 = (0.5 * (locals.var_eeff1s_dn8 + locals.var_eeff1d_dn8));
        locals.var_eeff1_dn9 = (0.5 * (locals.var_eeff1s_dn9 + locals.var_eeff1d_dn9));
        locals.var_eeff1_rv = 0.0;

        let assign21600_e21854: f64 = (locals.var_eeff2s + locals.var_eeff2d);
        let assign21600_e21855: f64 = (0.5 * assign21600_e21854);
        locals.var_eeff2 = assign21600_e21855;
        locals.var_eeff2_dn4 = (0.5 * (locals.var_eeff2s_dn4 + locals.var_eeff2d_dn4));
        locals.var_eeff2_dn6 = (0.5 * (locals.var_eeff2s_dn6 + locals.var_eeff2d_dn6));
        locals.var_eeff2_dn7 = (0.5 * (locals.var_eeff2s_dn7 + locals.var_eeff2d_dn7));
        locals.var_eeff2_dn8 = (0.5 * (locals.var_eeff2s_dn8 + locals.var_eeff2d_dn8));
        locals.var_eeff2_dn9 = (0.5 * (locals.var_eeff2s_dn9 + locals.var_eeff2d_dn9));
        locals.var_eeff2_rv = 0.0;

        let assign21610_e21858: f64 = (locals.var_esurf1 * locals.var_betn1_t);
        let assign21610_e21861: f64 = (locals.var_stbet_i * locals.var_lnrtn);
        let assign21610_e21862: f64 = (assign21610_e21861).exp();
        let assign21610_e21863: f64 = (assign21610_e21858 * assign21610_e21862);
        let assign21610_e21865: f64 = (assign21610_e21863 * locals.var_ratio_pd);
        locals.var_c1 = assign21610_e21865;
        locals.var_c1_dn4 = ((((((locals.var_esurf1_dn4 * locals.var_betn1_t) + (locals.var_esurf1 * locals.var_betn1_t_dn4)) * assign21610_e21862) + (assign21610_e21858 * (assign21610_e21862 * (locals.var_stbet_i * locals.var_lnrtn_dn4)))) * locals.var_ratio_pd) + (assign21610_e21863 * locals.var_ratio_pd_dn4));
        locals.var_c1_dn6 = ((((((locals.var_esurf1_dn6 * locals.var_betn1_t) + (locals.var_esurf1 * locals.var_betn1_t_dn6)) * assign21610_e21862) + (assign21610_e21858 * (assign21610_e21862 * (locals.var_stbet_i * locals.var_lnrtn_dn6)))) * locals.var_ratio_pd) + (assign21610_e21863 * locals.var_ratio_pd_dn6));
        locals.var_c1_dn7 = ((((((locals.var_esurf1_dn7 * locals.var_betn1_t) + (locals.var_esurf1 * locals.var_betn1_t_dn7)) * assign21610_e21862) + (assign21610_e21858 * (assign21610_e21862 * (locals.var_stbet_i * locals.var_lnrtn_dn7)))) * locals.var_ratio_pd) + (assign21610_e21863 * locals.var_ratio_pd_dn7));
        locals.var_c1_dn8 = ((((((locals.var_esurf1_dn8 * locals.var_betn1_t) + (locals.var_esurf1 * locals.var_betn1_t_dn8)) * assign21610_e21862) + (assign21610_e21858 * (assign21610_e21862 * (locals.var_stbet_i * locals.var_lnrtn_dn8)))) * locals.var_ratio_pd) + (assign21610_e21863 * locals.var_ratio_pd_dn8));
        locals.var_c1_dn9 = ((((((locals.var_esurf1_dn9 * locals.var_betn1_t) + (locals.var_esurf1 * locals.var_betn1_t_dn9)) * assign21610_e21862) + (assign21610_e21858 * (assign21610_e21862 * (locals.var_stbet_i * locals.var_lnrtn_dn9)))) * locals.var_ratio_pd) + (assign21610_e21863 * locals.var_ratio_pd_dn9));
        locals.var_c1_rv = 0.0;

        let assign21620_e21868: f64 = (locals.var_esurf2 * locals.var_betn2_t);
        let assign21620_e21871: f64 = (locals.var_stbet_i * locals.var_lnrtn);
        let assign21620_e21872: f64 = (assign21620_e21871).exp();
        let assign21620_e21873: f64 = (assign21620_e21868 * assign21620_e21872);
        locals.var_c2 = assign21620_e21873;
        locals.var_c2_dn4 = ((((locals.var_esurf2_dn4 * locals.var_betn2_t) + (locals.var_esurf2 * locals.var_betn2_t_dn4)) * assign21620_e21872) + (assign21620_e21868 * (assign21620_e21872 * (locals.var_stbet_i * locals.var_lnrtn_dn4))));
        locals.var_c2_dn6 = ((((locals.var_esurf2_dn6 * locals.var_betn2_t) + (locals.var_esurf2 * locals.var_betn2_t_dn6)) * assign21620_e21872) + (assign21620_e21868 * (assign21620_e21872 * (locals.var_stbet_i * locals.var_lnrtn_dn6))));
        locals.var_c2_dn7 = ((((locals.var_esurf2_dn7 * locals.var_betn2_t) + (locals.var_esurf2 * locals.var_betn2_t_dn7)) * assign21620_e21872) + (assign21620_e21868 * (assign21620_e21872 * (locals.var_stbet_i * locals.var_lnrtn_dn7))));
        locals.var_c2_dn8 = ((((locals.var_esurf2_dn8 * locals.var_betn2_t) + (locals.var_esurf2 * locals.var_betn2_t_dn8)) * assign21620_e21872) + (assign21620_e21868 * (assign21620_e21872 * (locals.var_stbet_i * locals.var_lnrtn_dn8))));
        locals.var_c2_dn9 = ((((locals.var_esurf2_dn9 * locals.var_betn2_t) + (locals.var_esurf2 * locals.var_betn2_t_dn9)) * assign21620_e21872) + (assign21620_e21868 * (assign21620_e21872 * (locals.var_stbet_i * locals.var_lnrtn_dn9))));
        locals.var_c2_rv = 0.0;

        let assign21630_e21876: f64 = (locals.var_c1 + locals.var_c2);
        locals.var_csum = assign21630_e21876;
        locals.var_csum_dn4 = (locals.var_c1_dn4 + locals.var_c2_dn4);
        locals.var_csum_dn6 = (locals.var_c1_dn6 + locals.var_c2_dn6);
        locals.var_csum_dn7 = (locals.var_c1_dn7 + locals.var_c2_dn7);
        locals.var_csum_dn8 = (locals.var_c1_dn8 + locals.var_c2_dn8);
        locals.var_csum_dn9 = (locals.var_c1_dn9 + locals.var_c2_dn9);
        locals.var_csum_rv = 0.0;

        let assign21640_e21881: f64 = (locals.var_xcorb_i * locals.var_ecpl2);
        let assign21640_e21882: f64 = (locals.var_ecpl1 + assign21640_e21881);
        let assign21640_e21883: f64 = (locals.var_xcor_i * assign21640_e21882);
        locals.var_temp1 = assign21640_e21883;
        locals.var_temp1_dn4 = ((locals.var_xcor_i_dn4 * assign21640_e21882) + (locals.var_xcor_i * (locals.var_ecpl1_dn4 + (locals.var_xcorb_i * locals.var_ecpl2_dn4))));
        locals.var_temp1_dn6 = ((locals.var_xcor_i_dn6 * assign21640_e21882) + (locals.var_xcor_i * (locals.var_ecpl1_dn6 + (locals.var_xcorb_i * locals.var_ecpl2_dn6))));
        locals.var_temp1_dn7 = ((locals.var_xcor_i_dn7 * assign21640_e21882) + (locals.var_xcor_i * (locals.var_ecpl1_dn7 + (locals.var_xcorb_i * locals.var_ecpl2_dn7))));
        locals.var_temp1_dn8 = ((locals.var_xcor_i_dn8 * assign21640_e21882) + (locals.var_xcor_i * (locals.var_ecpl1_dn8 + (locals.var_xcorb_i * locals.var_ecpl2_dn8))));
        locals.var_temp1_dn9 = ((locals.var_xcor_i_dn9 * assign21640_e21882) + (locals.var_xcor_i * (locals.var_ecpl1_dn9 + (locals.var_xcorb_i * locals.var_ecpl2_dn9))));
        locals.var_temp1_rv = 0.0;

        let assign21650_e21887: f64 = (1.0 + locals.var_temp1);
        let assign21650_e21889: f64 = assign21650_e21887;
        let assign21650_e21892: f64 = (1.0 + locals.var_temp1);
        let assign21650_e21894: f64 = assign21650_e21892;
        let assign21650_e21897: f64 = (1.0 + locals.var_temp1);
        let assign21650_e21899: f64 = assign21650_e21897;
        let assign21650_e21900: f64 = (assign21650_e21894 * assign21650_e21899);
        let assign21650_e21902: f64 = (assign21650_e21900 + 0.01);
        let assign21650_e21903: f64 = (assign21650_e21902).sqrt();
        let assign21650_e21904: f64 = (assign21650_e21889 + assign21650_e21903);
        let assign21650_e21905: f64 = (0.5 * assign21650_e21904);
        locals.var_temp2 = assign21650_e21905;
        locals.var_temp2_dn4 = (0.5 * (locals.var_temp1_dn4 + (((locals.var_temp1_dn4 * assign21650_e21899) + (assign21650_e21894 * locals.var_temp1_dn4)) / (2.0 * assign21650_e21903))));
        locals.var_temp2_dn6 = (0.5 * (locals.var_temp1_dn6 + (((locals.var_temp1_dn6 * assign21650_e21899) + (assign21650_e21894 * locals.var_temp1_dn6)) / (2.0 * assign21650_e21903))));
        locals.var_temp2_dn7 = (0.5 * (locals.var_temp1_dn7 + (((locals.var_temp1_dn7 * assign21650_e21899) + (assign21650_e21894 * locals.var_temp1_dn7)) / (2.0 * assign21650_e21903))));
        locals.var_temp2_dn8 = (0.5 * (locals.var_temp1_dn8 + (((locals.var_temp1_dn8 * assign21650_e21899) + (assign21650_e21894 * locals.var_temp1_dn8)) / (2.0 * assign21650_e21903))));
        locals.var_temp2_dn9 = (0.5 * (locals.var_temp1_dn9 + (((locals.var_temp1_dn9 * assign21650_e21899) + (assign21650_e21894 * locals.var_temp1_dn9)) / (2.0 * assign21650_e21903))));
        locals.var_temp2_rv = 0.0;

        let assign21660_e21910: f64 = (0.2 * locals.var_temp1);
        let assign21660_e21911: f64 = (1.0 + assign21660_e21910);
        let assign21660_e21913: f64 = assign21660_e21911;
        let assign21660_e21917: f64 = (0.2 * locals.var_temp1);
        let assign21660_e21918: f64 = (1.0 + assign21660_e21917);
        let assign21660_e21920: f64 = assign21660_e21918;
        let assign21660_e21924: f64 = (0.2 * locals.var_temp1);
        let assign21660_e21925: f64 = (1.0 + assign21660_e21924);
        let assign21660_e21927: f64 = assign21660_e21925;
        let assign21660_e21928: f64 = (assign21660_e21920 * assign21660_e21927);
        let assign21660_e21930: f64 = (assign21660_e21928 + 0.01);
        let assign21660_e21931: f64 = (assign21660_e21930).sqrt();
        let assign21660_e21932: f64 = (assign21660_e21913 + assign21660_e21931);
        let assign21660_e21933: f64 = (0.5 * assign21660_e21932);
        locals.var_temp3 = assign21660_e21933;
        locals.var_temp3_dn4 = (0.5 * ((0.2 * locals.var_temp1_dn4) + ((((0.2 * locals.var_temp1_dn4) * assign21660_e21927) + (assign21660_e21920 * (0.2 * locals.var_temp1_dn4))) / (2.0 * assign21660_e21931))));
        locals.var_temp3_dn6 = (0.5 * ((0.2 * locals.var_temp1_dn6) + ((((0.2 * locals.var_temp1_dn6) * assign21660_e21927) + (assign21660_e21920 * (0.2 * locals.var_temp1_dn6))) / (2.0 * assign21660_e21931))));
        locals.var_temp3_dn7 = (0.5 * ((0.2 * locals.var_temp1_dn7) + ((((0.2 * locals.var_temp1_dn7) * assign21660_e21927) + (assign21660_e21920 * (0.2 * locals.var_temp1_dn7))) / (2.0 * assign21660_e21931))));
        locals.var_temp3_dn8 = (0.5 * ((0.2 * locals.var_temp1_dn8) + ((((0.2 * locals.var_temp1_dn8) * assign21660_e21927) + (assign21660_e21920 * (0.2 * locals.var_temp1_dn8))) / (2.0 * assign21660_e21931))));
        locals.var_temp3_dn9 = (0.5 * ((0.2 * locals.var_temp1_dn9) + ((((0.2 * locals.var_temp1_dn9) * assign21660_e21927) + (assign21660_e21920 * (0.2 * locals.var_temp1_dn9))) / (2.0 * assign21660_e21931))));
        locals.var_temp3_rv = 0.0;

        let assign21670_e21936: f64 = (locals.var_temp2 / locals.var_temp3);
        locals.var_fcor = assign21670_e21936;
        locals.var_fcor_dn4 = (((locals.var_temp2_dn4 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn4)) / (locals.var_temp3 * locals.var_temp3));
        locals.var_fcor_dn6 = (((locals.var_temp2_dn6 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn6)) / (locals.var_temp3 * locals.var_temp3));
        locals.var_fcor_dn7 = (((locals.var_temp2_dn7 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn7)) / (locals.var_temp3 * locals.var_temp3));
        locals.var_fcor_dn8 = (((locals.var_temp2_dn8 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn8)) / (locals.var_temp3 * locals.var_temp3));
        locals.var_fcor_dn9 = (((locals.var_temp2_dn9 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn9)) / (locals.var_temp3 * locals.var_temp3));
        locals.var_fcor_rv = 0.0;

        let assign21680_e21941: f64 = (locals.var_csfi_i * locals.var_ecpl1);
        let assign21680_e21942: f64 = (1.0 + assign21680_e21941);
        let assign21680_e21945: f64 = (locals.var_csbi_i * locals.var_ecpl2);
        let assign21680_e21946: f64 = (assign21680_e21942 + assign21680_e21945);
        let assign21680_e21947: f64 = (locals.var_cs_i * assign21680_e21946);
        let assign21680_e21949: f64 = (-locals.var_thecs_i);
        let assign21680_e21953: f64 = (locals.var_qi1m * locals.var_inv_qi1cs);
        let assign21680_e21954: f64 = (1.0 + assign21680_e21953);
        let assign21680_e21957: f64 = (locals.var_qi2m * locals.var_inv_qi2cs);
        let assign21680_e21958: f64 = (assign21680_e21954 + assign21680_e21957);
        let assign21680_e21959: f64 = (assign21680_e21958).ln();
        let assign21680_e21960: f64 = (assign21680_e21949 * assign21680_e21959);
        let assign21680_e21961: f64 = (assign21680_e21960).exp();
        let assign21680_e21962: f64 = (assign21680_e21947 * assign21680_e21961);
        locals.var_gcs = assign21680_e21962;
        locals.var_gcs_dn4 = ((((locals.var_cs_i_dn4 * assign21680_e21946) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1_dn4) + (locals.var_csbi_i * locals.var_ecpl2_dn4)))) * assign21680_e21961) + (assign21680_e21947 * (assign21680_e21961 * (((-locals.var_thecs_i_dn4) * assign21680_e21959) + (assign21680_e21949 * (((locals.var_qi1m_dn4 * locals.var_inv_qi1cs) + (locals.var_qi2m_dn4 * locals.var_inv_qi2cs)) / assign21680_e21958))))));
        locals.var_gcs_dn6 = ((((locals.var_cs_i_dn6 * assign21680_e21946) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1_dn6) + (locals.var_csbi_i * locals.var_ecpl2_dn6)))) * assign21680_e21961) + (assign21680_e21947 * (assign21680_e21961 * (((-locals.var_thecs_i_dn6) * assign21680_e21959) + (assign21680_e21949 * (((locals.var_qi1m_dn6 * locals.var_inv_qi1cs) + (locals.var_qi2m_dn6 * locals.var_inv_qi2cs)) / assign21680_e21958))))));
        locals.var_gcs_dn7 = ((((locals.var_cs_i_dn7 * assign21680_e21946) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1_dn7) + (locals.var_csbi_i * locals.var_ecpl2_dn7)))) * assign21680_e21961) + (assign21680_e21947 * (assign21680_e21961 * (((-locals.var_thecs_i_dn7) * assign21680_e21959) + (assign21680_e21949 * (((locals.var_qi1m_dn7 * locals.var_inv_qi1cs) + (locals.var_qi2m_dn7 * locals.var_inv_qi2cs)) / assign21680_e21958))))));
        locals.var_gcs_dn8 = ((((locals.var_cs_i_dn8 * assign21680_e21946) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1_dn8) + (locals.var_csbi_i * locals.var_ecpl2_dn8)))) * assign21680_e21961) + (assign21680_e21947 * (assign21680_e21961 * (((-locals.var_thecs_i_dn8) * assign21680_e21959) + (assign21680_e21949 * (((locals.var_qi1m_dn8 * locals.var_inv_qi1cs) + (locals.var_qi2m_dn8 * locals.var_inv_qi2cs)) / assign21680_e21958))))));
        locals.var_gcs_dn9 = ((((locals.var_cs_i_dn9 * assign21680_e21946) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1_dn9) + (locals.var_csbi_i * locals.var_ecpl2_dn9)))) * assign21680_e21961) + (assign21680_e21947 * (assign21680_e21961 * (((-locals.var_thecs_i_dn9) * assign21680_e21959) + (assign21680_e21949 * (((locals.var_qi1m_dn9 * locals.var_inv_qi1cs) + (locals.var_qi2m_dn9 * locals.var_inv_qi2cs)) / assign21680_e21958))))));
        locals.var_gcs_rv = 0.0;

        let assign21690_e21965: f64 = if locals.var_rsg_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard672 = assign21690_e21965;
        locals.var_guard672_rv = 0.0;

        let (assign21700_e21969, assign21700_e21969_d_n4, assign21700_e21969_d_n6, assign21700_e21969_d_n7, assign21700_e21969_d_n8, assign21700_e21969_d_n9,) = {
    if (locals.var_guard672 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign21700_e21969;
        locals.var_temp3_dn4 = assign21700_e21969_d_n4;
        locals.var_temp3_dn6 = assign21700_e21969_d_n6;
        locals.var_temp3_dn7 = assign21700_e21969_d_n7;
        locals.var_temp3_dn8 = assign21700_e21969_d_n8;
        locals.var_temp3_dn9 = assign21700_e21969_d_n9;
        locals.var_temp3_rv = 0.0;

        let assign21710_e21972: f64 = if locals.var_rsg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard673 = assign21710_e21972;
        locals.var_guard673_rv = 0.0;

        let (assign21720_e21987, assign21720_e21987_d_n4, assign21720_e21987_d_n6, assign21720_e21987_d_n7, assign21720_e21987_d_n8, assign21720_e21987_d_n9,) = {
    if ((locals.var_guard672 == 0.0) && (locals.var_guard673 != 0.0)) {
        let assign21720_e21981: f64 = (locals.var_qim + 1e-12);
        let assign21720_e21982: f64 = (assign21720_e21981).ln();
        let assign21720_e21983: f64 = (locals.var_thersg_i * assign21720_e21982);
        let assign21720_e21984: f64 = (assign21720_e21983).exp();
        let assign21720_e21985: f64 = (locals.var_rsg_i * assign21720_e21984);
        (assign21720_e21985, (locals.var_rsg_i * (assign21720_e21984 * (locals.var_thersg_i * (locals.var_qim_dn4 / assign21720_e21981)))), (locals.var_rsg_i * (assign21720_e21984 * (locals.var_thersg_i * (locals.var_qim_dn6 / assign21720_e21981)))), (locals.var_rsg_i * (assign21720_e21984 * (locals.var_thersg_i * (locals.var_qim_dn7 / assign21720_e21981)))), (locals.var_rsg_i * (assign21720_e21984 * (locals.var_thersg_i * (locals.var_qim_dn8 / assign21720_e21981)))), (locals.var_rsg_i * (assign21720_e21984 * (locals.var_thersg_i * (locals.var_qim_dn9 / assign21720_e21981)))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign21720_e21987;
        locals.var_temp1_dn4 = assign21720_e21987_d_n4;
        locals.var_temp1_dn6 = assign21720_e21987_d_n6;
        locals.var_temp1_dn7 = assign21720_e21987_d_n7;
        locals.var_temp1_dn8 = assign21720_e21987_d_n8;
        locals.var_temp1_dn9 = assign21720_e21987_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign21730_e21996, assign21730_e21996_d_n4, assign21730_e21996_d_n6, assign21730_e21996_d_n7, assign21730_e21996_d_n8, assign21730_e21996_d_n9,) = {
    if ((locals.var_guard672 == 0.0) && (locals.var_guard673 != 0.0)) {
        let assign21730_e21994: f64 = (1.0 - locals.var_temp1);
        (assign21730_e21994, (-locals.var_temp1_dn4), (-locals.var_temp1_dn6), (-locals.var_temp1_dn7), (-locals.var_temp1_dn8), (-locals.var_temp1_dn9),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign21730_e21996;
        locals.var_temp3_dn4 = assign21730_e21996_d_n4;
        locals.var_temp3_dn6 = assign21730_e21996_d_n6;
        locals.var_temp3_dn7 = assign21730_e21996_d_n7;
        locals.var_temp3_dn8 = assign21730_e21996_d_n8;
        locals.var_temp3_dn9 = assign21730_e21996_d_n9;
        locals.var_temp3_rv = 0.0;

        let (assign21740_e22012, assign21740_e22012_d_n4, assign21740_e22012_d_n6, assign21740_e22012_d_n7, assign21740_e22012_d_n8, assign21740_e22012_d_n9,) = {
    if ((locals.var_guard672 == 0.0) && (locals.var_guard673 == 0.0)) {
        let assign21740_e22006: f64 = (locals.var_qim + 1e-12);
        let assign21740_e22007: f64 = (assign21740_e22006).ln();
        let assign21740_e22008: f64 = (locals.var_thersg_i * assign21740_e22007);
        let assign21740_e22009: f64 = (assign21740_e22008).exp();
        let assign21740_e22010: f64 = (locals.var_rsg_i * assign21740_e22009);
        (assign21740_e22010, (locals.var_rsg_i * (assign21740_e22009 * (locals.var_thersg_i * (locals.var_qim_dn4 / assign21740_e22006)))), (locals.var_rsg_i * (assign21740_e22009 * (locals.var_thersg_i * (locals.var_qim_dn6 / assign21740_e22006)))), (locals.var_rsg_i * (assign21740_e22009 * (locals.var_thersg_i * (locals.var_qim_dn7 / assign21740_e22006)))), (locals.var_rsg_i * (assign21740_e22009 * (locals.var_thersg_i * (locals.var_qim_dn8 / assign21740_e22006)))), (locals.var_rsg_i * (assign21740_e22009 * (locals.var_thersg_i * (locals.var_qim_dn9 / assign21740_e22006)))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign21740_e22012;
        locals.var_temp1_dn4 = assign21740_e22012_d_n4;
        locals.var_temp1_dn6 = assign21740_e22012_d_n6;
        locals.var_temp1_dn7 = assign21740_e22012_d_n7;
        locals.var_temp1_dn8 = assign21740_e22012_d_n8;
        locals.var_temp1_dn9 = assign21740_e22012_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign21750_e22024, assign21750_e22024_d_n4, assign21750_e22024_d_n6, assign21750_e22024_d_n7, assign21750_e22024_d_n8, assign21750_e22024_d_n9,) = {
    if ((locals.var_guard672 == 0.0) && (locals.var_guard673 == 0.0)) {
        let assign21750_e22021: f64 = (1.0 + locals.var_temp1);
        let assign21750_e22022: f64 = (1.0 / assign21750_e22021);
        (assign21750_e22022, (-(locals.var_temp1_dn4 / (assign21750_e22021 * assign21750_e22021))), (-(locals.var_temp1_dn6 / (assign21750_e22021 * assign21750_e22021))), (-(locals.var_temp1_dn7 / (assign21750_e22021 * assign21750_e22021))), (-(locals.var_temp1_dn8 / (assign21750_e22021 * assign21750_e22021))), (-(locals.var_temp1_dn9 / (assign21750_e22021 * assign21750_e22021))),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign21750_e22024;
        locals.var_temp3_dn4 = assign21750_e22024_d_n4;
        locals.var_temp3_dn6 = assign21750_e22024_d_n6;
        locals.var_temp3_dn7 = assign21750_e22024_d_n7;
        locals.var_temp3_dn8 = assign21750_e22024_d_n8;
        locals.var_temp3_dn9 = assign21750_e22024_d_n9;
        locals.var_temp3_rv = 0.0;

        let assign21760_e22028: f64 = (locals.var_qim * locals.var_temp3);
        let assign21760_e22030: f64 = (assign21760_e22028 + locals.var_rsig_i);
        let assign21760_e22031: f64 = (locals.var_frscsi * assign21760_e22030);
        locals.var_grs = assign21760_e22031;
        locals.var_grs_dn4 = ((locals.var_frscsi_dn4 * assign21760_e22030) + (locals.var_frscsi * ((locals.var_qim_dn4 * locals.var_temp3) + (locals.var_qim * locals.var_temp3_dn4))));
        locals.var_grs_dn6 = ((locals.var_frscsi_dn6 * assign21760_e22030) + (locals.var_frscsi * ((locals.var_qim_dn6 * locals.var_temp3) + (locals.var_qim * locals.var_temp3_dn6))));
        locals.var_grs_dn7 = ((locals.var_frscsi_dn7 * assign21760_e22030) + (locals.var_frscsi * ((locals.var_qim_dn7 * locals.var_temp3) + (locals.var_qim * locals.var_temp3_dn7))));
        locals.var_grs_dn8 = ((locals.var_frscsi_dn8 * assign21760_e22030) + (locals.var_frscsi * ((locals.var_qim_dn8 * locals.var_temp3) + (locals.var_qim * locals.var_temp3_dn8))));
        locals.var_grs_dn9 = ((locals.var_frscsi_dn9 * assign21760_e22030) + (locals.var_frscsi * ((locals.var_qim_dn9 * locals.var_temp3) + (locals.var_qim * locals.var_temp3_dn9))));
        locals.var_grs_rv = 0.0;

        let assign21770_e22036: f64 = (locals.var_fmue * locals.var_eeff1);
        let assign21770_e22038: f64 = (assign21770_e22036 + 1e-6);
        let assign21770_e22039: f64 = (assign21770_e22038).ln();
        let assign21770_e22040: f64 = (locals.var_themu_i * assign21770_e22039);
        let assign21770_e22041: f64 = (assign21770_e22040).exp();
        let assign21770_e22042: f64 = (1.0 + assign21770_e22041);
        let assign21770_e22044: f64 = (assign21770_e22042 + locals.var_gcs);
        let assign21770_e22047: f64 = (locals.var_betn1_i * locals.var_grs);
        let assign21770_e22048: f64 = (assign21770_e22044 + assign21770_e22047);
        locals.var_gmob1 = assign21770_e22048;
        locals.var_gmob1_dn4 = (((assign21770_e22041 * ((locals.var_themu_i_dn4 * assign21770_e22039) + (locals.var_themu_i * (((locals.var_fmue_dn4 * locals.var_eeff1) + (locals.var_fmue * locals.var_eeff1_dn4)) / assign21770_e22038)))) + locals.var_gcs_dn4) + ((locals.var_betn1_i_dn4 * locals.var_grs) + (locals.var_betn1_i * locals.var_grs_dn4)));
        locals.var_gmob1_dn6 = (((assign21770_e22041 * ((locals.var_themu_i_dn6 * assign21770_e22039) + (locals.var_themu_i * (((locals.var_fmue_dn6 * locals.var_eeff1) + (locals.var_fmue * locals.var_eeff1_dn6)) / assign21770_e22038)))) + locals.var_gcs_dn6) + ((locals.var_betn1_i_dn6 * locals.var_grs) + (locals.var_betn1_i * locals.var_grs_dn6)));
        locals.var_gmob1_dn7 = (((assign21770_e22041 * ((locals.var_themu_i_dn7 * assign21770_e22039) + (locals.var_themu_i * (((locals.var_fmue_dn7 * locals.var_eeff1) + (locals.var_fmue * locals.var_eeff1_dn7)) / assign21770_e22038)))) + locals.var_gcs_dn7) + ((locals.var_betn1_i_dn7 * locals.var_grs) + (locals.var_betn1_i * locals.var_grs_dn7)));
        locals.var_gmob1_dn8 = (((assign21770_e22041 * ((locals.var_themu_i_dn8 * assign21770_e22039) + (locals.var_themu_i * (((locals.var_fmue_dn8 * locals.var_eeff1) + (locals.var_fmue * locals.var_eeff1_dn8)) / assign21770_e22038)))) + locals.var_gcs_dn8) + ((locals.var_betn1_i_dn8 * locals.var_grs) + (locals.var_betn1_i * locals.var_grs_dn8)));
        locals.var_gmob1_dn9 = (((assign21770_e22041 * ((locals.var_themu_i_dn9 * assign21770_e22039) + (locals.var_themu_i * (((locals.var_fmue_dn9 * locals.var_eeff1) + (locals.var_fmue * locals.var_eeff1_dn9)) / assign21770_e22038)))) + locals.var_gcs_dn9) + ((locals.var_betn1_i_dn9 * locals.var_grs) + (locals.var_betn1_i * locals.var_grs_dn9)));
        locals.var_gmob1_rv = 0.0;

        let assign21780_e22053: f64 = (locals.var_fmue * locals.var_eeff2);
        let assign21780_e22055: f64 = (assign21780_e22053 + 1e-6);
        let assign21780_e22056: f64 = (assign21780_e22055).ln();
        let assign21780_e22057: f64 = (locals.var_themu_i * assign21780_e22056);
        let assign21780_e22058: f64 = (assign21780_e22057).exp();
        let assign21780_e22059: f64 = (1.0 + assign21780_e22058);
        let assign21780_e22061: f64 = (assign21780_e22059 + locals.var_gcs);
        let assign21780_e22064: f64 = (locals.var_betn2_i * locals.var_grs);
        let assign21780_e22065: f64 = (assign21780_e22061 + assign21780_e22064);
        locals.var_gmob2 = assign21780_e22065;
        locals.var_gmob2_dn4 = (((assign21780_e22058 * ((locals.var_themu_i_dn4 * assign21780_e22056) + (locals.var_themu_i * (((locals.var_fmue_dn4 * locals.var_eeff2) + (locals.var_fmue * locals.var_eeff2_dn4)) / assign21780_e22055)))) + locals.var_gcs_dn4) + ((locals.var_betn2_i_dn4 * locals.var_grs) + (locals.var_betn2_i * locals.var_grs_dn4)));
        locals.var_gmob2_dn6 = (((assign21780_e22058 * ((locals.var_themu_i_dn6 * assign21780_e22056) + (locals.var_themu_i * (((locals.var_fmue_dn6 * locals.var_eeff2) + (locals.var_fmue * locals.var_eeff2_dn6)) / assign21780_e22055)))) + locals.var_gcs_dn6) + ((locals.var_betn2_i_dn6 * locals.var_grs) + (locals.var_betn2_i * locals.var_grs_dn6)));
        locals.var_gmob2_dn7 = (((assign21780_e22058 * ((locals.var_themu_i_dn7 * assign21780_e22056) + (locals.var_themu_i * (((locals.var_fmue_dn7 * locals.var_eeff2) + (locals.var_fmue * locals.var_eeff2_dn7)) / assign21780_e22055)))) + locals.var_gcs_dn7) + ((locals.var_betn2_i_dn7 * locals.var_grs) + (locals.var_betn2_i * locals.var_grs_dn7)));
        locals.var_gmob2_dn8 = (((assign21780_e22058 * ((locals.var_themu_i_dn8 * assign21780_e22056) + (locals.var_themu_i * (((locals.var_fmue_dn8 * locals.var_eeff2) + (locals.var_fmue * locals.var_eeff2_dn8)) / assign21780_e22055)))) + locals.var_gcs_dn8) + ((locals.var_betn2_i_dn8 * locals.var_grs) + (locals.var_betn2_i * locals.var_grs_dn8)));
        locals.var_gmob2_dn9 = (((assign21780_e22058 * ((locals.var_themu_i_dn9 * assign21780_e22056) + (locals.var_themu_i * (((locals.var_fmue_dn9 * locals.var_eeff2) + (locals.var_fmue * locals.var_eeff2_dn9)) / assign21780_e22055)))) + locals.var_gcs_dn9) + ((locals.var_betn2_i_dn9 * locals.var_grs) + (locals.var_betn2_i * locals.var_grs_dn9)));
        locals.var_gmob2_rv = 0.0;

        let assign21790_e22068: f64 = (locals.var_fcor * locals.var_csum);
        let assign21790_e22071: f64 = (locals.var_c1 / locals.var_gmob1);
        let assign21790_e22074: f64 = (locals.var_c2 / locals.var_gmob2);
        let assign21790_e22075: f64 = (assign21790_e22071 + assign21790_e22074);
        let assign21790_e22076: f64 = (assign21790_e22068 / assign21790_e22075);
        locals.var_gmob = assign21790_e22076;
        locals.var_gmob_dn4 = (((((locals.var_fcor_dn4 * locals.var_csum) + (locals.var_fcor * locals.var_csum_dn4)) * assign21790_e22075) - (assign21790_e22068 * ((((locals.var_c1_dn4 * locals.var_gmob1) - (locals.var_c1 * locals.var_gmob1_dn4)) / (locals.var_gmob1 * locals.var_gmob1)) + (((locals.var_c2_dn4 * locals.var_gmob2) - (locals.var_c2 * locals.var_gmob2_dn4)) / (locals.var_gmob2 * locals.var_gmob2))))) / (assign21790_e22075 * assign21790_e22075));
        locals.var_gmob_dn6 = (((((locals.var_fcor_dn6 * locals.var_csum) + (locals.var_fcor * locals.var_csum_dn6)) * assign21790_e22075) - (assign21790_e22068 * ((((locals.var_c1_dn6 * locals.var_gmob1) - (locals.var_c1 * locals.var_gmob1_dn6)) / (locals.var_gmob1 * locals.var_gmob1)) + (((locals.var_c2_dn6 * locals.var_gmob2) - (locals.var_c2 * locals.var_gmob2_dn6)) / (locals.var_gmob2 * locals.var_gmob2))))) / (assign21790_e22075 * assign21790_e22075));
        locals.var_gmob_dn7 = (((((locals.var_fcor_dn7 * locals.var_csum) + (locals.var_fcor * locals.var_csum_dn7)) * assign21790_e22075) - (assign21790_e22068 * ((((locals.var_c1_dn7 * locals.var_gmob1) - (locals.var_c1 * locals.var_gmob1_dn7)) / (locals.var_gmob1 * locals.var_gmob1)) + (((locals.var_c2_dn7 * locals.var_gmob2) - (locals.var_c2 * locals.var_gmob2_dn7)) / (locals.var_gmob2 * locals.var_gmob2))))) / (assign21790_e22075 * assign21790_e22075));
        locals.var_gmob_dn8 = (((((locals.var_fcor_dn8 * locals.var_csum) + (locals.var_fcor * locals.var_csum_dn8)) * assign21790_e22075) - (assign21790_e22068 * ((((locals.var_c1_dn8 * locals.var_gmob1) - (locals.var_c1 * locals.var_gmob1_dn8)) / (locals.var_gmob1 * locals.var_gmob1)) + (((locals.var_c2_dn8 * locals.var_gmob2) - (locals.var_c2 * locals.var_gmob2_dn8)) / (locals.var_gmob2 * locals.var_gmob2))))) / (assign21790_e22075 * assign21790_e22075));
        locals.var_gmob_dn9 = (((((locals.var_fcor_dn9 * locals.var_csum) + (locals.var_fcor * locals.var_csum_dn9)) * assign21790_e22075) - (assign21790_e22068 * ((((locals.var_c1_dn9 * locals.var_gmob1) - (locals.var_c1 * locals.var_gmob1_dn9)) / (locals.var_gmob1 * locals.var_gmob1)) + (((locals.var_c2_dn9 * locals.var_gmob2) - (locals.var_c2 * locals.var_gmob2_dn9)) / (locals.var_gmob2 * locals.var_gmob2))))) / (assign21790_e22075 * assign21790_e22075));
        locals.var_gmob_rv = 0.0;

        let assign21800_e22080: f64 = (4.0 + locals.var_qim);
        let assign21800_e22081: f64 = (1.0 / assign21800_e22080);
        locals.var_inv_qimstar1 = assign21800_e22081;
        locals.var_inv_qimstar1_dn4 = (-(locals.var_qim_dn4 / (assign21800_e22080 * assign21800_e22080)));
        locals.var_inv_qimstar1_dn6 = (-(locals.var_qim_dn6 / (assign21800_e22080 * assign21800_e22080)));
        locals.var_inv_qimstar1_dn7 = (-(locals.var_qim_dn7 / (assign21800_e22080 * assign21800_e22080)));
        locals.var_inv_qimstar1_dn8 = (-(locals.var_qim_dn8 / (assign21800_e22080 * assign21800_e22080)));
        locals.var_inv_qimstar1_dn9 = (-(locals.var_qim_dn9 / (assign21800_e22080 * assign21800_e22080)));
        locals.var_inv_qimstar1_rv = 0.0;

        let assign21810_e22084: f64 = if locals.var_alpb_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard674 = assign21810_e22084;
        locals.var_guard674_rv = 0.0;

        let (assign21820_e22094, assign21820_e22094_d_n4, assign21820_e22094_d_n6, assign21820_e22094_d_n7, assign21820_e22094_d_n8, assign21820_e22094_d_n9,) = {
    if (locals.var_guard674 != 0.0) {
        let assign21820_e22090: f64 = (locals.var_alpb_i * locals.var_qi2m);
        let assign21820_e22091: f64 = (1.0 + assign21820_e22090);
        let assign21820_e22092: f64 = (1.0 / assign21820_e22091);
        (assign21820_e22092, (-((locals.var_alpb_i * locals.var_qi2m_dn4) / (assign21820_e22091 * assign21820_e22091))), (-((locals.var_alpb_i * locals.var_qi2m_dn6) / (assign21820_e22091 * assign21820_e22091))), (-((locals.var_alpb_i * locals.var_qi2m_dn7) / (assign21820_e22091 * assign21820_e22091))), (-((locals.var_alpb_i * locals.var_qi2m_dn8) / (assign21820_e22091 * assign21820_e22091))), (-((locals.var_alpb_i * locals.var_qi2m_dn9) / (assign21820_e22091 * assign21820_e22091))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign21820_e22094;
        locals.var_temp_dn4 = assign21820_e22094_d_n4;
        locals.var_temp_dn6 = assign21820_e22094_d_n6;
        locals.var_temp_dn7 = assign21820_e22094_d_n7;
        locals.var_temp_dn8 = assign21820_e22094_d_n8;
        locals.var_temp_dn9 = assign21820_e22094_d_n9;
        locals.var_temp_rv = 0.0;

        let (assign21830_e22103, assign21830_e22103_d_n4, assign21830_e22103_d_n6, assign21830_e22103_d_n7, assign21830_e22103_d_n8, assign21830_e22103_d_n9,) = {
    if (locals.var_guard674 == 0.0) {
        let assign21830_e22100: f64 = (locals.var_alpb_i * locals.var_qi2m);
        let assign21830_e22101: f64 = (1.0 - assign21830_e22100);
        (assign21830_e22101, (-(locals.var_alpb_i * locals.var_qi2m_dn4)), (-(locals.var_alpb_i * locals.var_qi2m_dn6)), (-(locals.var_alpb_i * locals.var_qi2m_dn7)), (-(locals.var_alpb_i * locals.var_qi2m_dn8)), (-(locals.var_alpb_i * locals.var_qi2m_dn9)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign21830_e22103;
        locals.var_temp_dn4 = assign21830_e22103_d_n4;
        locals.var_temp_dn6 = assign21830_e22103_d_n6;
        locals.var_temp_dn7 = assign21830_e22103_d_n7;
        locals.var_temp_dn8 = assign21830_e22103_d_n8;
        locals.var_temp_dn9 = assign21830_e22103_d_n9;
        locals.var_temp_rv = 0.0;

        let assign21840_e22106: f64 = (locals.var_qim * locals.var_inv_qimstar1);
        let assign21840_e22108: f64 = (assign21840_e22106 * locals.var_temp);
        locals.var_r1 = assign21840_e22108;
        locals.var_r1_dn4 = ((((locals.var_qim_dn4 * locals.var_inv_qimstar1) + (locals.var_qim * locals.var_inv_qimstar1_dn4)) * locals.var_temp) + (assign21840_e22106 * locals.var_temp_dn4));
        locals.var_r1_dn6 = ((((locals.var_qim_dn6 * locals.var_inv_qimstar1) + (locals.var_qim * locals.var_inv_qimstar1_dn6)) * locals.var_temp) + (assign21840_e22106 * locals.var_temp_dn6));
        locals.var_r1_dn7 = ((((locals.var_qim_dn7 * locals.var_inv_qimstar1) + (locals.var_qim * locals.var_inv_qimstar1_dn7)) * locals.var_temp) + (assign21840_e22106 * locals.var_temp_dn7));
        locals.var_r1_dn8 = ((((locals.var_qim_dn8 * locals.var_inv_qimstar1) + (locals.var_qim * locals.var_inv_qimstar1_dn8)) * locals.var_temp) + (assign21840_e22106 * locals.var_temp_dn8));
        locals.var_r1_dn9 = ((((locals.var_qim_dn9 * locals.var_inv_qimstar1) + (locals.var_qim * locals.var_inv_qimstar1_dn9)) * locals.var_temp) + (assign21840_e22106 * locals.var_temp_dn9));
        locals.var_r1_rv = 0.0;

        let assign21850_e22112: f64 = (locals.var_xd - locals.var_xdeff);
        let assign21850_e22115: f64 = (locals.var_vp_i * locals.var_inv_phit);
        let assign21850_e22118: f64 = (locals.var_vpg_i * locals.var_qim);
        let assign21850_e22120: f64 = (assign21850_e22118 * locals.var_qim);
        let assign21850_e22121: f64 = (assign21850_e22115 + assign21850_e22120);
        let assign21850_e22122: f64 = (assign21850_e22112 / assign21850_e22121);
        let assign21850_e22123: f64 = (1.0 + assign21850_e22122);
        let assign21850_e22124: f64 = (assign21850_e22123).ln();
        let assign21850_e22126: f64 = (assign21850_e22124 * locals.var_r1);
        locals.var_dl_l_fact = assign21850_e22126;
        locals.var_dl_l_fact_dn4 = (((((((locals.var_xd_dn4 - locals.var_xdeff_dn4) * assign21850_e22121) - (assign21850_e22112 * ((locals.var_vp_i * locals.var_inv_phit_dn4) + (((locals.var_vpg_i * locals.var_qim_dn4) * locals.var_qim) + (assign21850_e22118 * locals.var_qim_dn4))))) / (assign21850_e22121 * assign21850_e22121)) / assign21850_e22123) * locals.var_r1) + (assign21850_e22124 * locals.var_r1_dn4));
        locals.var_dl_l_fact_dn6 = (((((((locals.var_xd_dn6 - locals.var_xdeff_dn6) * assign21850_e22121) - (assign21850_e22112 * ((locals.var_vp_i * locals.var_inv_phit_dn6) + (((locals.var_vpg_i * locals.var_qim_dn6) * locals.var_qim) + (assign21850_e22118 * locals.var_qim_dn6))))) / (assign21850_e22121 * assign21850_e22121)) / assign21850_e22123) * locals.var_r1) + (assign21850_e22124 * locals.var_r1_dn6));
        locals.var_dl_l_fact_dn7 = (((((((locals.var_xd_dn7 - locals.var_xdeff_dn7) * assign21850_e22121) - (assign21850_e22112 * ((locals.var_vp_i * locals.var_inv_phit_dn7) + (((locals.var_vpg_i * locals.var_qim_dn7) * locals.var_qim) + (assign21850_e22118 * locals.var_qim_dn7))))) / (assign21850_e22121 * assign21850_e22121)) / assign21850_e22123) * locals.var_r1) + (assign21850_e22124 * locals.var_r1_dn7));
        locals.var_dl_l_fact_dn8 = (((((((locals.var_xd_dn8 - locals.var_xdeff_dn8) * assign21850_e22121) - (assign21850_e22112 * ((locals.var_vp_i * locals.var_inv_phit_dn8) + (((locals.var_vpg_i * locals.var_qim_dn8) * locals.var_qim) + (assign21850_e22118 * locals.var_qim_dn8))))) / (assign21850_e22121 * assign21850_e22121)) / assign21850_e22123) * locals.var_r1) + (assign21850_e22124 * locals.var_r1_dn8));
        locals.var_dl_l_fact_dn9 = (((((((locals.var_xd_dn9 - locals.var_xdeff_dn9) * assign21850_e22121) - (assign21850_e22112 * ((locals.var_vp_i * locals.var_inv_phit_dn9) + (((locals.var_vpg_i * locals.var_qim_dn9) * locals.var_qim) + (assign21850_e22118 * locals.var_qim_dn9))))) / (assign21850_e22121 * assign21850_e22121)) / assign21850_e22123) * locals.var_r1) + (assign21850_e22124 * locals.var_r1_dn9));
        locals.var_dl_l_fact_rv = 0.0;

        let assign21860_e22129: f64 = (locals.var_alp_loc * locals.var_dl_l_fact);
        locals.var_dl_l = assign21860_e22129;
        locals.var_dl_l_dn4 = (locals.var_alp_loc * locals.var_dl_l_fact_dn4);
        locals.var_dl_l_dn6 = (locals.var_alp_loc * locals.var_dl_l_fact_dn6);
        locals.var_dl_l_dn7 = (locals.var_alp_loc * locals.var_dl_l_fact_dn7);
        locals.var_dl_l_dn8 = (locals.var_alp_loc * locals.var_dl_l_fact_dn8);
        locals.var_dl_l_dn9 = (locals.var_alp_loc * locals.var_dl_l_fact_dn9);
        locals.var_dl_l_rv = 0.0;

        let assign21870_e22135: f64 = (1.0 + locals.var_dl_l);
        let assign21870_e22136: f64 = (locals.var_dl_l * assign21870_e22135);
        let assign21870_e22137: f64 = (1.0 + assign21870_e22136);
        let assign21870_e22138: f64 = (1.0 / assign21870_e22137);
        locals.var_gdl = assign21870_e22138;
        locals.var_gdl_dn4 = (-(((locals.var_dl_l_dn4 * assign21870_e22135) + (locals.var_dl_l * locals.var_dl_l_dn4)) / (assign21870_e22137 * assign21870_e22137)));
        locals.var_gdl_dn6 = (-(((locals.var_dl_l_dn6 * assign21870_e22135) + (locals.var_dl_l * locals.var_dl_l_dn6)) / (assign21870_e22137 * assign21870_e22137)));
        locals.var_gdl_dn7 = (-(((locals.var_dl_l_dn7 * assign21870_e22135) + (locals.var_dl_l * locals.var_dl_l_dn7)) / (assign21870_e22137 * assign21870_e22137)));
        locals.var_gdl_dn8 = (-(((locals.var_dl_l_dn8 * assign21870_e22135) + (locals.var_dl_l * locals.var_dl_l_dn8)) / (assign21870_e22137 * assign21870_e22137)));
        locals.var_gdl_dn9 = (-(((locals.var_dl_l_dn9 * assign21870_e22135) + (locals.var_dl_l * locals.var_dl_l_dn9)) / (assign21870_e22137 * assign21870_e22137)));
        locals.var_gdl_rv = 0.0;

        let assign21880_e22141: f64 = (100.0 * locals.var_esurf1);
        let assign21880_e22144: f64 = (100.0 + locals.var_esurf1);
        let assign21880_e22145: f64 = (assign21880_e22141 / assign21880_e22144);
        locals.var_wsat1 = assign21880_e22145;
        locals.var_wsat1_dn4 = ((((100.0 * locals.var_esurf1_dn4) * assign21880_e22144) - (assign21880_e22141 * locals.var_esurf1_dn4)) / (assign21880_e22144 * assign21880_e22144));
        locals.var_wsat1_dn6 = ((((100.0 * locals.var_esurf1_dn6) * assign21880_e22144) - (assign21880_e22141 * locals.var_esurf1_dn6)) / (assign21880_e22144 * assign21880_e22144));
        locals.var_wsat1_dn7 = ((((100.0 * locals.var_esurf1_dn7) * assign21880_e22144) - (assign21880_e22141 * locals.var_esurf1_dn7)) / (assign21880_e22144 * assign21880_e22144));
        locals.var_wsat1_dn8 = ((((100.0 * locals.var_esurf1_dn8) * assign21880_e22144) - (assign21880_e22141 * locals.var_esurf1_dn8)) / (assign21880_e22144 * assign21880_e22144));
        locals.var_wsat1_dn9 = ((((100.0 * locals.var_esurf1_dn9) * assign21880_e22144) - (assign21880_e22141 * locals.var_esurf1_dn9)) / (assign21880_e22144 * assign21880_e22144));
        locals.var_wsat1_rv = 0.0;

        let assign21890_e22148: f64 = if locals.var_thesat1_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard675 = assign21890_e22148;
        locals.var_guard675_rv = 0.0;

        let (assign21900_e22158, assign21900_e22158_d_n4, assign21900_e22158_d_n6, assign21900_e22158_d_n7, assign21900_e22158_d_n8, assign21900_e22158_d_n9,) = {
    if (locals.var_guard675 != 0.0) {
        let assign21900_e22154: f64 = (locals.var_thesat1_i * locals.var_wsat1);
        let assign21900_e22155: f64 = (1.0 - assign21900_e22154);
        let assign21900_e22156: f64 = (1.0 / assign21900_e22155);
        (assign21900_e22156, (-((-(locals.var_thesat1_i * locals.var_wsat1_dn4)) / (assign21900_e22155 * assign21900_e22155))), (-((-(locals.var_thesat1_i * locals.var_wsat1_dn6)) / (assign21900_e22155 * assign21900_e22155))), (-((-(locals.var_thesat1_i * locals.var_wsat1_dn7)) / (assign21900_e22155 * assign21900_e22155))), (-((-(locals.var_thesat1_i * locals.var_wsat1_dn8)) / (assign21900_e22155 * assign21900_e22155))), (-((-(locals.var_thesat1_i * locals.var_wsat1_dn9)) / (assign21900_e22155 * assign21900_e22155))),)
    } else {
        (locals.var_sat_fact1, locals.var_sat_fact1_dn4, locals.var_sat_fact1_dn6, locals.var_sat_fact1_dn7, locals.var_sat_fact1_dn8, locals.var_sat_fact1_dn9,)
    }
};
        locals.var_sat_fact1 = assign21900_e22158;
        locals.var_sat_fact1_dn4 = assign21900_e22158_d_n4;
        locals.var_sat_fact1_dn6 = assign21900_e22158_d_n6;
        locals.var_sat_fact1_dn7 = assign21900_e22158_d_n7;
        locals.var_sat_fact1_dn8 = assign21900_e22158_d_n8;
        locals.var_sat_fact1_dn9 = assign21900_e22158_d_n9;
        locals.var_sat_fact1_rv = 0.0;

        let (assign21910_e22167, assign21910_e22167_d_n4, assign21910_e22167_d_n6, assign21910_e22167_d_n7, assign21910_e22167_d_n8, assign21910_e22167_d_n9,) = {
    if (locals.var_guard675 == 0.0) {
        let assign21910_e22164: f64 = (locals.var_thesat1_i * locals.var_wsat1);
        let assign21910_e22165: f64 = (1.0 + assign21910_e22164);
        (assign21910_e22165, (locals.var_thesat1_i * locals.var_wsat1_dn4), (locals.var_thesat1_i * locals.var_wsat1_dn6), (locals.var_thesat1_i * locals.var_wsat1_dn7), (locals.var_thesat1_i * locals.var_wsat1_dn8), (locals.var_thesat1_i * locals.var_wsat1_dn9),)
    } else {
        (locals.var_sat_fact1, locals.var_sat_fact1_dn4, locals.var_sat_fact1_dn6, locals.var_sat_fact1_dn7, locals.var_sat_fact1_dn8, locals.var_sat_fact1_dn9,)
    }
};
        locals.var_sat_fact1 = assign21910_e22167;
        locals.var_sat_fact1_dn4 = assign21910_e22167_d_n4;
        locals.var_sat_fact1_dn6 = assign21910_e22167_d_n6;
        locals.var_sat_fact1_dn7 = assign21910_e22167_d_n7;
        locals.var_sat_fact1_dn8 = assign21910_e22167_d_n8;
        locals.var_sat_fact1_dn9 = assign21910_e22167_d_n9;
        locals.var_sat_fact1_rv = 0.0;

        let assign21920_e22170: f64 = (100.0 * locals.var_esurf2);
        let assign21920_e22173: f64 = (100.0 + locals.var_esurf2);
        let assign21920_e22174: f64 = (assign21920_e22170 / assign21920_e22173);
        locals.var_wsat2 = assign21920_e22174;
        locals.var_wsat2_dn4 = ((((100.0 * locals.var_esurf2_dn4) * assign21920_e22173) - (assign21920_e22170 * locals.var_esurf2_dn4)) / (assign21920_e22173 * assign21920_e22173));
        locals.var_wsat2_dn6 = ((((100.0 * locals.var_esurf2_dn6) * assign21920_e22173) - (assign21920_e22170 * locals.var_esurf2_dn6)) / (assign21920_e22173 * assign21920_e22173));
        locals.var_wsat2_dn7 = ((((100.0 * locals.var_esurf2_dn7) * assign21920_e22173) - (assign21920_e22170 * locals.var_esurf2_dn7)) / (assign21920_e22173 * assign21920_e22173));
        locals.var_wsat2_dn8 = ((((100.0 * locals.var_esurf2_dn8) * assign21920_e22173) - (assign21920_e22170 * locals.var_esurf2_dn8)) / (assign21920_e22173 * assign21920_e22173));
        locals.var_wsat2_dn9 = ((((100.0 * locals.var_esurf2_dn9) * assign21920_e22173) - (assign21920_e22170 * locals.var_esurf2_dn9)) / (assign21920_e22173 * assign21920_e22173));
        locals.var_wsat2_rv = 0.0;

        let assign21930_e22177: f64 = if locals.var_thesat2_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard676 = assign21930_e22177;
        locals.var_guard676_rv = 0.0;

        let (assign21940_e22187, assign21940_e22187_d_n4, assign21940_e22187_d_n6, assign21940_e22187_d_n7, assign21940_e22187_d_n8, assign21940_e22187_d_n9,) = {
    if (locals.var_guard676 != 0.0) {
        let assign21940_e22183: f64 = (locals.var_thesat2_i * locals.var_wsat2);
        let assign21940_e22184: f64 = (1.0 - assign21940_e22183);
        let assign21940_e22185: f64 = (1.0 / assign21940_e22184);
        (assign21940_e22185, (-((-(locals.var_thesat2_i * locals.var_wsat2_dn4)) / (assign21940_e22184 * assign21940_e22184))), (-((-(locals.var_thesat2_i * locals.var_wsat2_dn6)) / (assign21940_e22184 * assign21940_e22184))), (-((-(locals.var_thesat2_i * locals.var_wsat2_dn7)) / (assign21940_e22184 * assign21940_e22184))), (-((-(locals.var_thesat2_i * locals.var_wsat2_dn8)) / (assign21940_e22184 * assign21940_e22184))), (-((-(locals.var_thesat2_i * locals.var_wsat2_dn9)) / (assign21940_e22184 * assign21940_e22184))),)
    } else {
        (locals.var_sat_fact2, locals.var_sat_fact2_dn4, locals.var_sat_fact2_dn6, locals.var_sat_fact2_dn7, locals.var_sat_fact2_dn8, locals.var_sat_fact2_dn9,)
    }
};
        locals.var_sat_fact2 = assign21940_e22187;
        locals.var_sat_fact2_dn4 = assign21940_e22187_d_n4;
        locals.var_sat_fact2_dn6 = assign21940_e22187_d_n6;
        locals.var_sat_fact2_dn7 = assign21940_e22187_d_n7;
        locals.var_sat_fact2_dn8 = assign21940_e22187_d_n8;
        locals.var_sat_fact2_dn9 = assign21940_e22187_d_n9;
        locals.var_sat_fact2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_60(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign21950_e22196, assign21950_e22196_d_n4, assign21950_e22196_d_n6, assign21950_e22196_d_n7, assign21950_e22196_d_n8, assign21950_e22196_d_n9,) = {
    if (locals.var_guard676 == 0.0) {
        let assign21950_e22193: f64 = (locals.var_thesat2_i * locals.var_wsat2);
        let assign21950_e22194: f64 = (1.0 + assign21950_e22193);
        (assign21950_e22194, (locals.var_thesat2_i * locals.var_wsat2_dn4), (locals.var_thesat2_i * locals.var_wsat2_dn6), (locals.var_thesat2_i * locals.var_wsat2_dn7), (locals.var_thesat2_i * locals.var_wsat2_dn8), (locals.var_thesat2_i * locals.var_wsat2_dn9),)
    } else {
        (locals.var_sat_fact2, locals.var_sat_fact2_dn4, locals.var_sat_fact2_dn6, locals.var_sat_fact2_dn7, locals.var_sat_fact2_dn8, locals.var_sat_fact2_dn9,)
    }
};
        locals.var_sat_fact2 = assign21950_e22196;
        locals.var_sat_fact2_dn4 = assign21950_e22196_d_n4;
        locals.var_sat_fact2_dn6 = assign21950_e22196_d_n6;
        locals.var_sat_fact2_dn7 = assign21950_e22196_d_n7;
        locals.var_sat_fact2_dn8 = assign21950_e22196_d_n8;
        locals.var_sat_fact2_dn9 = assign21950_e22196_d_n9;
        locals.var_sat_fact2_rv = 0.0;

        let assign21960_e22199: f64 = (locals.var_sat_phit_loc * locals.var_dxdrift);
        let assign21960_e22201: f64 = (assign21960_e22199 * 0.5);
        let assign21960_e22204: f64 = (locals.var_sat_fact1 + locals.var_sat_fact2);
        let assign21960_e22205: f64 = (assign21960_e22201 * assign21960_e22204);
        locals.var_ggamma = assign21960_e22205;
        locals.var_ggamma_dn4 = (((((locals.var_sat_phit_loc_dn4 * locals.var_dxdrift) + (locals.var_sat_phit_loc * locals.var_dxdrift_dn4)) * 0.5) * assign21960_e22204) + (assign21960_e22201 * (locals.var_sat_fact1_dn4 + locals.var_sat_fact2_dn4)));
        locals.var_ggamma_dn6 = (((((locals.var_sat_phit_loc_dn6 * locals.var_dxdrift) + (locals.var_sat_phit_loc * locals.var_dxdrift_dn6)) * 0.5) * assign21960_e22204) + (assign21960_e22201 * (locals.var_sat_fact1_dn6 + locals.var_sat_fact2_dn6)));
        locals.var_ggamma_dn7 = (((((locals.var_sat_phit_loc_dn7 * locals.var_dxdrift) + (locals.var_sat_phit_loc * locals.var_dxdrift_dn7)) * 0.5) * assign21960_e22204) + (assign21960_e22201 * (locals.var_sat_fact1_dn7 + locals.var_sat_fact2_dn7)));
        locals.var_ggamma_dn8 = (((((locals.var_sat_phit_loc_dn8 * locals.var_dxdrift) + (locals.var_sat_phit_loc * locals.var_dxdrift_dn8)) * 0.5) * assign21960_e22204) + (assign21960_e22201 * (locals.var_sat_fact1_dn8 + locals.var_sat_fact2_dn8)));
        locals.var_ggamma_dn9 = (((((locals.var_sat_phit_loc_dn9 * locals.var_dxdrift) + (locals.var_sat_phit_loc * locals.var_dxdrift_dn9)) * 0.5) * assign21960_e22204) + (assign21960_e22201 * (locals.var_sat_fact1_dn9 + locals.var_sat_fact2_dn9)));
        locals.var_ggamma_rv = 0.0;

        let assign21970_e22209: f64 = (locals.var_gmob * locals.var_gdl);
        let assign21970_e22210: f64 = (locals.var_ggamma / assign21970_e22209);
        locals.var_sqrt_zsat = assign21970_e22210;
        locals.var_sqrt_zsat_dn4 = (((locals.var_ggamma_dn4 * assign21970_e22209) - (locals.var_ggamma * ((locals.var_gmob_dn4 * locals.var_gdl) + (locals.var_gmob * locals.var_gdl_dn4)))) / (assign21970_e22209 * assign21970_e22209));
        locals.var_sqrt_zsat_dn6 = (((locals.var_ggamma_dn6 * assign21970_e22209) - (locals.var_ggamma * ((locals.var_gmob_dn6 * locals.var_gdl) + (locals.var_gmob * locals.var_gdl_dn6)))) / (assign21970_e22209 * assign21970_e22209));
        locals.var_sqrt_zsat_dn7 = (((locals.var_ggamma_dn7 * assign21970_e22209) - (locals.var_ggamma * ((locals.var_gmob_dn7 * locals.var_gdl) + (locals.var_gmob * locals.var_gdl_dn7)))) / (assign21970_e22209 * assign21970_e22209));
        locals.var_sqrt_zsat_dn8 = (((locals.var_ggamma_dn8 * assign21970_e22209) - (locals.var_ggamma * ((locals.var_gmob_dn8 * locals.var_gdl) + (locals.var_gmob * locals.var_gdl_dn8)))) / (assign21970_e22209 * assign21970_e22209));
        locals.var_sqrt_zsat_dn9 = (((locals.var_ggamma_dn9 * assign21970_e22209) - (locals.var_ggamma * ((locals.var_gmob_dn9 * locals.var_gdl) + (locals.var_gmob * locals.var_gdl_dn9)))) / (assign21970_e22209 * assign21970_e22209));
        locals.var_sqrt_zsat_rv = 0.0;

        let assign21980_e22213: f64 = (locals.var_sqrt_zsat * locals.var_sqrt_zsat);
        locals.var_zsat = assign21980_e22213;
        locals.var_zsat_dn4 = ((locals.var_sqrt_zsat_dn4 * locals.var_sqrt_zsat) + (locals.var_sqrt_zsat * locals.var_sqrt_zsat_dn4));
        locals.var_zsat_dn6 = ((locals.var_sqrt_zsat_dn6 * locals.var_sqrt_zsat) + (locals.var_sqrt_zsat * locals.var_sqrt_zsat_dn6));
        locals.var_zsat_dn7 = ((locals.var_sqrt_zsat_dn7 * locals.var_sqrt_zsat) + (locals.var_sqrt_zsat * locals.var_sqrt_zsat_dn7));
        locals.var_zsat_dn8 = ((locals.var_sqrt_zsat_dn8 * locals.var_sqrt_zsat) + (locals.var_sqrt_zsat * locals.var_sqrt_zsat_dn8));
        locals.var_zsat_dn9 = ((locals.var_sqrt_zsat_dn9 * locals.var_sqrt_zsat) + (locals.var_sqrt_zsat * locals.var_sqrt_zsat_dn9));
        locals.var_zsat_rv = 0.0;

        let assign21990_e22216: f64 = (1.0 + locals.var_zsat);
        let assign21990_e22217: f64 = (assign21990_e22216).sqrt();
        locals.var_vsat_fact = assign21990_e22217;
        locals.var_vsat_fact_dn4 = (locals.var_zsat_dn4 / (2.0 * assign21990_e22217));
        locals.var_vsat_fact_dn6 = (locals.var_zsat_dn6 / (2.0 * assign21990_e22217));
        locals.var_vsat_fact_dn7 = (locals.var_zsat_dn7 / (2.0 * assign21990_e22217));
        locals.var_vsat_fact_dn8 = (locals.var_zsat_dn8 / (2.0 * assign21990_e22217));
        locals.var_vsat_fact_dn9 = (locals.var_zsat_dn9 / (2.0 * assign21990_e22217));
        locals.var_vsat_fact_rv = 0.0;

        let assign22000_e22221: f64 = (1.5 * locals.var_zsat);
        let assign22000_e22222: f64 = (1.0 + assign22000_e22221);
        let assign22000_e22224: f64 = (assign22000_e22222 / locals.var_vsat_fact);
        locals.var_hsat = assign22000_e22224;
        locals.var_hsat_dn4 = ((((1.5 * locals.var_zsat_dn4) * locals.var_vsat_fact) - (assign22000_e22222 * locals.var_vsat_fact_dn4)) / (locals.var_vsat_fact * locals.var_vsat_fact));
        locals.var_hsat_dn6 = ((((1.5 * locals.var_zsat_dn6) * locals.var_vsat_fact) - (assign22000_e22222 * locals.var_vsat_fact_dn6)) / (locals.var_vsat_fact * locals.var_vsat_fact));
        locals.var_hsat_dn7 = ((((1.5 * locals.var_zsat_dn7) * locals.var_vsat_fact) - (assign22000_e22222 * locals.var_vsat_fact_dn7)) / (locals.var_vsat_fact * locals.var_vsat_fact));
        locals.var_hsat_dn8 = ((((1.5 * locals.var_zsat_dn8) * locals.var_vsat_fact) - (assign22000_e22222 * locals.var_vsat_fact_dn8)) / (locals.var_vsat_fact * locals.var_vsat_fact));
        locals.var_hsat_dn9 = ((((1.5 * locals.var_zsat_dn9) * locals.var_vsat_fact) - (assign22000_e22222 * locals.var_vsat_fact_dn9)) / (locals.var_vsat_fact * locals.var_vsat_fact));
        locals.var_hsat_rv = 0.0;

        let assign22010_e22227: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard677 = assign22010_e22227;
        locals.var_guard677_rv = 0.0;

        let (assign22020_e22244, assign22020_e22244_d_n4, assign22020_e22244_d_n6, assign22020_e22244_d_n7, assign22020_e22244_d_n8, assign22020_e22244_d_n9,) = {
    if (locals.var_guard677 != 0.0) {
        let assign22020_e22231: f64 = (0.6 * locals.var_qq);
        let assign22020_e22233: f64 = (-0.1666666666667);
        let assign22020_e22236: f64 = (locals.var_esurf1 * locals.var_esurf1);
        let assign22020_e22238: f64 = (assign22020_e22236 + 60.0);
        let assign22020_e22239: f64 = (assign22020_e22238).ln();
        let assign22020_e22240: f64 = (assign22020_e22233 * assign22020_e22239);
        let assign22020_e22241: f64 = (assign22020_e22240).exp();
        let assign22020_e22242: f64 = (assign22020_e22231 * assign22020_e22241);
        (assign22020_e22242, (((0.6 * locals.var_qq_dn4) * assign22020_e22241) + (assign22020_e22231 * (assign22020_e22241 * (assign22020_e22233 * (((locals.var_esurf1_dn4 * locals.var_esurf1) + (locals.var_esurf1 * locals.var_esurf1_dn4)) / assign22020_e22238))))), (((0.6 * locals.var_qq_dn6) * assign22020_e22241) + (assign22020_e22231 * (assign22020_e22241 * (assign22020_e22233 * (((locals.var_esurf1_dn6 * locals.var_esurf1) + (locals.var_esurf1 * locals.var_esurf1_dn6)) / assign22020_e22238))))), (((0.6 * locals.var_qq_dn7) * assign22020_e22241) + (assign22020_e22231 * (assign22020_e22241 * (assign22020_e22233 * (((locals.var_esurf1_dn7 * locals.var_esurf1) + (locals.var_esurf1 * locals.var_esurf1_dn7)) / assign22020_e22238))))), (((0.6 * locals.var_qq_dn8) * assign22020_e22241) + (assign22020_e22231 * (assign22020_e22241 * (assign22020_e22233 * (((locals.var_esurf1_dn8 * locals.var_esurf1) + (locals.var_esurf1 * locals.var_esurf1_dn8)) / assign22020_e22238))))), (((0.6 * locals.var_qq_dn9) * assign22020_e22241) + (assign22020_e22231 * (assign22020_e22241 * (assign22020_e22233 * (((locals.var_esurf1_dn9 * locals.var_esurf1) + (locals.var_esurf1 * locals.var_esurf1_dn9)) / assign22020_e22238))))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign22020_e22244;
        locals.var_temp1_dn4 = assign22020_e22244_d_n4;
        locals.var_temp1_dn6 = assign22020_e22244_d_n6;
        locals.var_temp1_dn7 = assign22020_e22244_d_n7;
        locals.var_temp1_dn8 = assign22020_e22244_d_n8;
        locals.var_temp1_dn9 = assign22020_e22244_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign22030_e22261, assign22030_e22261_d_n4, assign22030_e22261_d_n6, assign22030_e22261_d_n7, assign22030_e22261_d_n8, assign22030_e22261_d_n9,) = {
    if (locals.var_guard677 != 0.0) {
        let assign22030_e22248: f64 = (0.6 * locals.var_qq);
        let assign22030_e22250: f64 = (-0.1666666666667);
        let assign22030_e22253: f64 = (locals.var_esurf2 * locals.var_esurf2);
        let assign22030_e22255: f64 = (assign22030_e22253 + 60.0);
        let assign22030_e22256: f64 = (assign22030_e22255).ln();
        let assign22030_e22257: f64 = (assign22030_e22250 * assign22030_e22256);
        let assign22030_e22258: f64 = (assign22030_e22257).exp();
        let assign22030_e22259: f64 = (assign22030_e22248 * assign22030_e22258);
        (assign22030_e22259, (((0.6 * locals.var_qq_dn4) * assign22030_e22258) + (assign22030_e22248 * (assign22030_e22258 * (assign22030_e22250 * (((locals.var_esurf2_dn4 * locals.var_esurf2) + (locals.var_esurf2 * locals.var_esurf2_dn4)) / assign22030_e22255))))), (((0.6 * locals.var_qq_dn6) * assign22030_e22258) + (assign22030_e22248 * (assign22030_e22258 * (assign22030_e22250 * (((locals.var_esurf2_dn6 * locals.var_esurf2) + (locals.var_esurf2 * locals.var_esurf2_dn6)) / assign22030_e22255))))), (((0.6 * locals.var_qq_dn7) * assign22030_e22258) + (assign22030_e22248 * (assign22030_e22258 * (assign22030_e22250 * (((locals.var_esurf2_dn7 * locals.var_esurf2) + (locals.var_esurf2 * locals.var_esurf2_dn7)) / assign22030_e22255))))), (((0.6 * locals.var_qq_dn8) * assign22030_e22258) + (assign22030_e22248 * (assign22030_e22258 * (assign22030_e22250 * (((locals.var_esurf2_dn8 * locals.var_esurf2) + (locals.var_esurf2 * locals.var_esurf2_dn8)) / assign22030_e22255))))), (((0.6 * locals.var_qq_dn9) * assign22030_e22258) + (assign22030_e22248 * (assign22030_e22258 * (assign22030_e22250 * (((locals.var_esurf2_dn9 * locals.var_esurf2) + (locals.var_esurf2 * locals.var_esurf2_dn9)) / assign22030_e22255))))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign22030_e22261;
        locals.var_temp2_dn4 = assign22030_e22261_d_n4;
        locals.var_temp2_dn6 = assign22030_e22261_d_n6;
        locals.var_temp2_dn7 = assign22030_e22261_d_n7;
        locals.var_temp2_dn8 = assign22030_e22261_d_n8;
        locals.var_temp2_dn9 = assign22030_e22261_d_n9;
        locals.var_temp2_rv = 0.0;

        let (assign22040_e22271, assign22040_e22271_d_n4, assign22040_e22271_d_n6, assign22040_e22271_d_n7, assign22040_e22271_d_n8, assign22040_e22271_d_n9,) = {
    if (locals.var_guard677 != 0.0) {
        let assign22040_e22266: f64 = (locals.var_k1 * locals.var_temp1);
        let assign22040_e22267: f64 = (1.0 + assign22040_e22266);
        let assign22040_e22269: f64 = (assign22040_e22267 / locals.var_tox1fact);
        (assign22040_e22269, (((((locals.var_k1_dn4 * locals.var_temp1) + (locals.var_k1 * locals.var_temp1_dn4)) * locals.var_tox1fact) - (assign22040_e22267 * locals.var_tox1fact_dn4)) / (locals.var_tox1fact * locals.var_tox1fact)), (((((locals.var_k1_dn6 * locals.var_temp1) + (locals.var_k1 * locals.var_temp1_dn6)) * locals.var_tox1fact) - (assign22040_e22267 * locals.var_tox1fact_dn6)) / (locals.var_tox1fact * locals.var_tox1fact)), (((((locals.var_k1_dn7 * locals.var_temp1) + (locals.var_k1 * locals.var_temp1_dn7)) * locals.var_tox1fact) - (assign22040_e22267 * locals.var_tox1fact_dn7)) / (locals.var_tox1fact * locals.var_tox1fact)), (((((locals.var_k1_dn8 * locals.var_temp1) + (locals.var_k1 * locals.var_temp1_dn8)) * locals.var_tox1fact) - (assign22040_e22267 * locals.var_tox1fact_dn8)) / (locals.var_tox1fact * locals.var_tox1fact)), (((((locals.var_k1_dn9 * locals.var_temp1) + (locals.var_k1 * locals.var_temp1_dn9)) * locals.var_tox1fact) - (assign22040_e22267 * locals.var_tox1fact_dn9)) / (locals.var_tox1fact * locals.var_tox1fact)),)
    } else {
        (locals.var_qmfact1, locals.var_qmfact1_dn4, locals.var_qmfact1_dn6, locals.var_qmfact1_dn7, locals.var_qmfact1_dn8, locals.var_qmfact1_dn9,)
    }
};
        locals.var_qmfact1 = assign22040_e22271;
        locals.var_qmfact1_dn4 = assign22040_e22271_d_n4;
        locals.var_qmfact1_dn6 = assign22040_e22271_d_n6;
        locals.var_qmfact1_dn7 = assign22040_e22271_d_n7;
        locals.var_qmfact1_dn8 = assign22040_e22271_d_n8;
        locals.var_qmfact1_dn9 = assign22040_e22271_d_n9;
        locals.var_qmfact1_rv = 0.0;

        let (assign22050_e22281, assign22050_e22281_d_n4, assign22050_e22281_d_n6, assign22050_e22281_d_n7, assign22050_e22281_d_n8, assign22050_e22281_d_n9,) = {
    if (locals.var_guard677 != 0.0) {
        let assign22050_e22276: f64 = (locals.var_k2 * locals.var_temp2);
        let assign22050_e22277: f64 = (1.0 + assign22050_e22276);
        let assign22050_e22279: f64 = (assign22050_e22277 / locals.var_tox2fact);
        (assign22050_e22279, (((((locals.var_k2_dn4 * locals.var_temp2) + (locals.var_k2 * locals.var_temp2_dn4)) * locals.var_tox2fact) - (assign22050_e22277 * locals.var_tox2fact_dn4)) / (locals.var_tox2fact * locals.var_tox2fact)), (((((locals.var_k2_dn6 * locals.var_temp2) + (locals.var_k2 * locals.var_temp2_dn6)) * locals.var_tox2fact) - (assign22050_e22277 * locals.var_tox2fact_dn6)) / (locals.var_tox2fact * locals.var_tox2fact)), (((((locals.var_k2_dn7 * locals.var_temp2) + (locals.var_k2 * locals.var_temp2_dn7)) * locals.var_tox2fact) - (assign22050_e22277 * locals.var_tox2fact_dn7)) / (locals.var_tox2fact * locals.var_tox2fact)), (((((locals.var_k2_dn8 * locals.var_temp2) + (locals.var_k2 * locals.var_temp2_dn8)) * locals.var_tox2fact) - (assign22050_e22277 * locals.var_tox2fact_dn8)) / (locals.var_tox2fact * locals.var_tox2fact)), (((((locals.var_k2_dn9 * locals.var_temp2) + (locals.var_k2 * locals.var_temp2_dn9)) * locals.var_tox2fact) - (assign22050_e22277 * locals.var_tox2fact_dn9)) / (locals.var_tox2fact * locals.var_tox2fact)),)
    } else {
        (locals.var_qmfact2, locals.var_qmfact2_dn4, locals.var_qmfact2_dn6, locals.var_qmfact2_dn7, locals.var_qmfact2_dn8, locals.var_qmfact2_dn9,)
    }
};
        locals.var_qmfact2 = assign22050_e22281;
        locals.var_qmfact2_dn4 = assign22050_e22281_d_n4;
        locals.var_qmfact2_dn6 = assign22050_e22281_d_n6;
        locals.var_qmfact2_dn7 = assign22050_e22281_d_n7;
        locals.var_qmfact2_dn8 = assign22050_e22281_d_n8;
        locals.var_qmfact2_dn9 = assign22050_e22281_d_n9;
        locals.var_qmfact2_rv = 0.0;

        let (assign22060_e22286, assign22060_e22286_d_n4, assign22060_e22286_d_n6, assign22060_e22286_d_n7, assign22060_e22286_d_n8, assign22060_e22286_d_n9,) = {
    if (locals.var_guard677 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qmfact1, locals.var_qmfact1_dn4, locals.var_qmfact1_dn6, locals.var_qmfact1_dn7, locals.var_qmfact1_dn8, locals.var_qmfact1_dn9,)
    }
};
        locals.var_qmfact1 = assign22060_e22286;
        locals.var_qmfact1_dn4 = assign22060_e22286_d_n4;
        locals.var_qmfact1_dn6 = assign22060_e22286_d_n6;
        locals.var_qmfact1_dn7 = assign22060_e22286_d_n7;
        locals.var_qmfact1_dn8 = assign22060_e22286_d_n8;
        locals.var_qmfact1_dn9 = assign22060_e22286_d_n9;
        locals.var_qmfact1_rv = 0.0;

        let (assign22070_e22291, assign22070_e22291_d_n4, assign22070_e22291_d_n6, assign22070_e22291_d_n7, assign22070_e22291_d_n8, assign22070_e22291_d_n9,) = {
    if (locals.var_guard677 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qmfact2, locals.var_qmfact2_dn4, locals.var_qmfact2_dn6, locals.var_qmfact2_dn7, locals.var_qmfact2_dn8, locals.var_qmfact2_dn9,)
    }
};
        locals.var_qmfact2 = assign22070_e22291;
        locals.var_qmfact2_dn4 = assign22070_e22291_d_n4;
        locals.var_qmfact2_dn6 = assign22070_e22291_d_n6;
        locals.var_qmfact2_dn7 = assign22070_e22291_d_n7;
        locals.var_qmfact2_dn8 = assign22070_e22291_d_n8;
        locals.var_qmfact2_dn9 = assign22070_e22291_d_n9;
        locals.var_qmfact2_rv = 0.0;

        let assign22080_e22294: f64 = if locals.var_qis > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard678 = assign22080_e22294;
        locals.var_guard678_rv = 0.0;

        let assign22090_e22297: f64 = if locals.var_qid > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard679 = assign22090_e22297;
        locals.var_guard679_rv = 0.0;

        let assign22100_e22299: f64 = (locals.var_a2d).abs();
        let assign22100_e22301: f64 = if assign22100_e22299 < 0.01 { 1.0 } else { 0.0 };
        locals.var_guard680 = assign22100_e22301;
        locals.var_guard680_rv = 0.0;

        let (assign22110_e22321, assign22110_e22321_d_n4, assign22110_e22321_d_n6, assign22110_e22321_d_n7, assign22110_e22321_d_n8, assign22110_e22321_d_n9,) = {
    if (((locals.var_guard678 != 0.0) && (locals.var_guard679 != 0.0)) && (locals.var_guard680 != 0.0)) {
        let assign22110_e22309: f64 = (2.0 + locals.var_q1d);
        let assign22110_e22312: f64 = (0.5 * locals.var_a1d);
        let assign22110_e22313: f64 = (assign22110_e22309 + assign22110_e22312);
        let assign22110_e22316: f64 = (2.0 + locals.var_q2d);
        let assign22110_e22318: f64 = (assign22110_e22316 * locals.var_a1d);
        let assign22110_e22319: f64 = (assign22110_e22313 / assign22110_e22318);
        (assign22110_e22319, ((((locals.var_q1d_dn4 + (0.5 * locals.var_a1d_dn4)) * assign22110_e22318) - (assign22110_e22313 * ((locals.var_q2d_dn4 * locals.var_a1d) + (assign22110_e22316 * locals.var_a1d_dn4)))) / (assign22110_e22318 * assign22110_e22318)), ((((locals.var_q1d_dn6 + (0.5 * locals.var_a1d_dn6)) * assign22110_e22318) - (assign22110_e22313 * ((locals.var_q2d_dn6 * locals.var_a1d) + (assign22110_e22316 * locals.var_a1d_dn6)))) / (assign22110_e22318 * assign22110_e22318)), ((((locals.var_q1d_dn7 + (0.5 * locals.var_a1d_dn7)) * assign22110_e22318) - (assign22110_e22313 * ((locals.var_q2d_dn7 * locals.var_a1d) + (assign22110_e22316 * locals.var_a1d_dn7)))) / (assign22110_e22318 * assign22110_e22318)), ((((locals.var_q1d_dn8 + (0.5 * locals.var_a1d_dn8)) * assign22110_e22318) - (assign22110_e22313 * ((locals.var_q2d_dn8 * locals.var_a1d) + (assign22110_e22316 * locals.var_a1d_dn8)))) / (assign22110_e22318 * assign22110_e22318)), ((((locals.var_q1d_dn9 + (0.5 * locals.var_a1d_dn9)) * assign22110_e22318) - (assign22110_e22313 * ((locals.var_q2d_dn9 * locals.var_a1d) + (assign22110_e22316 * locals.var_a1d_dn9)))) / (assign22110_e22318 * assign22110_e22318)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign22110_e22321;
        locals.var_temp_dn4 = assign22110_e22321_d_n4;
        locals.var_temp_dn6 = assign22110_e22321_d_n6;
        locals.var_temp_dn7 = assign22110_e22321_d_n7;
        locals.var_temp_dn8 = assign22110_e22321_d_n8;
        locals.var_temp_dn9 = assign22110_e22321_d_n9;
        locals.var_temp_rv = 0.0;

        let (assign22120_e22331, assign22120_e22331_d_n4, assign22120_e22331_d_n6, assign22120_e22331_d_n7, assign22120_e22331_d_n8, assign22120_e22331_d_n9,) = {
    if (((locals.var_guard678 != 0.0) && (locals.var_guard679 != 0.0)) && (locals.var_guard680 != 0.0)) {
        let assign22120_e22329: f64 = (locals.var_temp * locals.var_a2d);
        (assign22120_e22329, ((locals.var_temp_dn4 * locals.var_a2d) + (locals.var_temp * locals.var_a2d_dn4)), ((locals.var_temp_dn6 * locals.var_a2d) + (locals.var_temp * locals.var_a2d_dn6)), ((locals.var_temp_dn7 * locals.var_a2d) + (locals.var_temp * locals.var_a2d_dn7)), ((locals.var_temp_dn8 * locals.var_a2d) + (locals.var_temp * locals.var_a2d_dn8)), ((locals.var_temp_dn9 * locals.var_a2d) + (locals.var_temp * locals.var_a2d_dn9)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign22120_e22331;
        locals.var_temp1_dn4 = assign22120_e22331_d_n4;
        locals.var_temp1_dn6 = assign22120_e22331_d_n6;
        locals.var_temp1_dn7 = assign22120_e22331_d_n7;
        locals.var_temp1_dn8 = assign22120_e22331_d_n8;
        locals.var_temp1_dn9 = assign22120_e22331_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign22130_e22341, assign22130_e22341_d_n4, assign22130_e22341_d_n6, assign22130_e22341_d_n7, assign22130_e22341_d_n8, assign22130_e22341_d_n9,) = {
    if (((locals.var_guard678 != 0.0) && (locals.var_guard679 != 0.0)) && (locals.var_guard680 != 0.0)) {
        let assign22130_e22339: f64 = (locals.var_temp1 * locals.var_temp1);
        (assign22130_e22339, ((locals.var_temp1_dn4 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn4)), ((locals.var_temp1_dn6 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn6)), ((locals.var_temp1_dn7 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn7)), ((locals.var_temp1_dn8 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn8)), ((locals.var_temp1_dn9 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn9)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign22130_e22341;
        locals.var_temp2_dn4 = assign22130_e22341_d_n4;
        locals.var_temp2_dn6 = assign22130_e22341_d_n6;
        locals.var_temp2_dn7 = assign22130_e22341_d_n7;
        locals.var_temp2_dn8 = assign22130_e22341_d_n8;
        locals.var_temp2_dn9 = assign22130_e22341_d_n9;
        locals.var_temp2_rv = 0.0;

        let (assign22140_e22353, assign22140_e22353_d_n4, assign22140_e22353_d_n6, assign22140_e22353_d_n7, assign22140_e22353_d_n8, assign22140_e22353_d_n9,) = {
    if (((locals.var_guard678 != 0.0) && (locals.var_guard679 != 0.0)) && (locals.var_guard680 != 0.0)) {
        let assign22140_e22349: f64 = (1.0 - locals.var_temp1);
        let assign22140_e22351: f64 = (assign22140_e22349 + locals.var_temp2);
        (assign22140_e22351, ((-locals.var_temp1_dn4) + locals.var_temp2_dn4), ((-locals.var_temp1_dn6) + locals.var_temp2_dn6), ((-locals.var_temp1_dn7) + locals.var_temp2_dn7), ((-locals.var_temp1_dn8) + locals.var_temp2_dn8), ((-locals.var_temp1_dn9) + locals.var_temp2_dn9),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign22140_e22353;
        locals.var_temp3_dn4 = assign22140_e22353_d_n4;
        locals.var_temp3_dn6 = assign22140_e22353_d_n6;
        locals.var_temp3_dn7 = assign22140_e22353_d_n7;
        locals.var_temp3_dn8 = assign22140_e22353_d_n8;
        locals.var_temp3_dn9 = assign22140_e22353_d_n9;
        locals.var_temp3_rv = 0.0;

        let (assign22150_e22365, assign22150_e22365_d_n4, assign22150_e22365_d_n6, assign22150_e22365_d_n7, assign22150_e22365_d_n8, assign22150_e22365_d_n9,) = {
    if (((locals.var_guard678 != 0.0) && (locals.var_guard679 != 0.0)) && (locals.var_guard680 != 0.0)) {
        let assign22150_e22362: f64 = (locals.var_temp1 * locals.var_temp2);
        let assign22150_e22363: f64 = (locals.var_temp3 - assign22150_e22362);
        (assign22150_e22363, (locals.var_temp3_dn4 - ((locals.var_temp1_dn4 * locals.var_temp2) + (locals.var_temp1 * locals.var_temp2_dn4))), (locals.var_temp3_dn6 - ((locals.var_temp1_dn6 * locals.var_temp2) + (locals.var_temp1 * locals.var_temp2_dn6))), (locals.var_temp3_dn7 - ((locals.var_temp1_dn7 * locals.var_temp2) + (locals.var_temp1 * locals.var_temp2_dn7))), (locals.var_temp3_dn8 - ((locals.var_temp1_dn8 * locals.var_temp2) + (locals.var_temp1 * locals.var_temp2_dn8))), (locals.var_temp3_dn9 - ((locals.var_temp1_dn9 * locals.var_temp2) + (locals.var_temp1 * locals.var_temp2_dn9))),)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
        locals.var_temp4 = assign22150_e22365;
        locals.var_temp4_dn4 = assign22150_e22365_d_n4;
        locals.var_temp4_dn6 = assign22150_e22365_d_n6;
        locals.var_temp4_dn7 = assign22150_e22365_d_n7;
        locals.var_temp4_dn8 = assign22150_e22365_d_n8;
        locals.var_temp4_dn9 = assign22150_e22365_d_n9;
        locals.var_temp4_rv = 0.0;

        let (assign22160_e22389, assign22160_e22389_d_n4, assign22160_e22389_d_n6, assign22160_e22389_d_n7, assign22160_e22389_d_n8, assign22160_e22389_d_n9,) = {
    if (((locals.var_guard678 != 0.0) && (locals.var_guard679 != 0.0)) && (locals.var_guard680 != 0.0)) {
        let assign22160_e22374: f64 = (2.0 * locals.var_qsqd);
        let assign22160_e22378: f64 = (1.0 / locals.var_a1d);
        let assign22160_e22379: f64 = (locals.var_temp - assign22160_e22378);
        let assign22160_e22380: f64 = (assign22160_e22374 * assign22160_e22379);
        let assign22160_e22382: f64 = (assign22160_e22380 * locals.var_temp4);
        let assign22160_e22383: f64 = (locals.var_k2q2d - assign22160_e22382);
        let assign22160_e22386: f64 = (2.0 + locals.var_q2d);
        let assign22160_e22387: f64 = (assign22160_e22383 / assign22160_e22386);
        (assign22160_e22387, ((((locals.var_k2q2d_dn4 - (((((2.0 * locals.var_qsqd_dn4) * assign22160_e22379) + (assign22160_e22374 * (locals.var_temp_dn4 - (-(locals.var_a1d_dn4 / (locals.var_a1d * locals.var_a1d)))))) * locals.var_temp4) + (assign22160_e22380 * locals.var_temp4_dn4))) * assign22160_e22386) - (assign22160_e22383 * locals.var_q2d_dn4)) / (assign22160_e22386 * assign22160_e22386)), ((((locals.var_k2q2d_dn6 - (((((2.0 * locals.var_qsqd_dn6) * assign22160_e22379) + (assign22160_e22374 * (locals.var_temp_dn6 - (-(locals.var_a1d_dn6 / (locals.var_a1d * locals.var_a1d)))))) * locals.var_temp4) + (assign22160_e22380 * locals.var_temp4_dn6))) * assign22160_e22386) - (assign22160_e22383 * locals.var_q2d_dn6)) / (assign22160_e22386 * assign22160_e22386)), ((((locals.var_k2q2d_dn7 - (((((2.0 * locals.var_qsqd_dn7) * assign22160_e22379) + (assign22160_e22374 * (locals.var_temp_dn7 - (-(locals.var_a1d_dn7 / (locals.var_a1d * locals.var_a1d)))))) * locals.var_temp4) + (assign22160_e22380 * locals.var_temp4_dn7))) * assign22160_e22386) - (assign22160_e22383 * locals.var_q2d_dn7)) / (assign22160_e22386 * assign22160_e22386)), ((((locals.var_k2q2d_dn8 - (((((2.0 * locals.var_qsqd_dn8) * assign22160_e22379) + (assign22160_e22374 * (locals.var_temp_dn8 - (-(locals.var_a1d_dn8 / (locals.var_a1d * locals.var_a1d)))))) * locals.var_temp4) + (assign22160_e22380 * locals.var_temp4_dn8))) * assign22160_e22386) - (assign22160_e22383 * locals.var_q2d_dn8)) / (assign22160_e22386 * assign22160_e22386)), ((((locals.var_k2q2d_dn9 - (((((2.0 * locals.var_qsqd_dn9) * assign22160_e22379) + (assign22160_e22374 * (locals.var_temp_dn9 - (-(locals.var_a1d_dn9 / (locals.var_a1d * locals.var_a1d)))))) * locals.var_temp4) + (assign22160_e22380 * locals.var_temp4_dn9))) * assign22160_e22386) - (assign22160_e22383 * locals.var_q2d_dn9)) / (assign22160_e22386 * assign22160_e22386)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign22160_e22389;
        locals.var_temp1_dn4 = assign22160_e22389_d_n4;
        locals.var_temp1_dn6 = assign22160_e22389_d_n6;
        locals.var_temp1_dn7 = assign22160_e22389_d_n7;
        locals.var_temp1_dn8 = assign22160_e22389_d_n8;
        locals.var_temp1_dn9 = assign22160_e22389_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign22170_e22407, assign22170_e22407_d_n4, assign22170_e22407_d_n6, assign22170_e22407_d_n7, assign22170_e22407_d_n8, assign22170_e22407_d_n9,) = {
    if (((locals.var_guard678 != 0.0) && (locals.var_guard679 != 0.0)) && (locals.var_guard680 != 0.0)) {
        let assign22170_e22397: f64 = (locals.var_dqsqd_dxn_qi * locals.var_qid);
        let assign22170_e22399: f64 = (assign22170_e22397 - locals.var_aexp1d);
        let assign22170_e22401: f64 = (assign22170_e22399 / locals.var_a1d);
        let assign22170_e22403: f64 = (assign22170_e22401 - locals.var_temp1);
        let assign22170_e22405: f64 = (assign22170_e22403 / locals.var_qid);
        (assign22170_e22405, ((((((((((locals.var_dqsqd_dxn_qi_dn4 * locals.var_qid) + (locals.var_dqsqd_dxn_qi * locals.var_qid_dn4)) - locals.var_aexp1d_dn4) * locals.var_a1d) - (assign22170_e22399 * locals.var_a1d_dn4)) / (locals.var_a1d * locals.var_a1d)) - locals.var_temp1_dn4) * locals.var_qid) - (assign22170_e22403 * locals.var_qid_dn4)) / (locals.var_qid * locals.var_qid)), ((((((((((locals.var_dqsqd_dxn_qi_dn6 * locals.var_qid) + (locals.var_dqsqd_dxn_qi * locals.var_qid_dn6)) - locals.var_aexp1d_dn6) * locals.var_a1d) - (assign22170_e22399 * locals.var_a1d_dn6)) / (locals.var_a1d * locals.var_a1d)) - locals.var_temp1_dn6) * locals.var_qid) - (assign22170_e22403 * locals.var_qid_dn6)) / (locals.var_qid * locals.var_qid)), ((((((((((locals.var_dqsqd_dxn_qi_dn7 * locals.var_qid) + (locals.var_dqsqd_dxn_qi * locals.var_qid_dn7)) - locals.var_aexp1d_dn7) * locals.var_a1d) - (assign22170_e22399 * locals.var_a1d_dn7)) / (locals.var_a1d * locals.var_a1d)) - locals.var_temp1_dn7) * locals.var_qid) - (assign22170_e22403 * locals.var_qid_dn7)) / (locals.var_qid * locals.var_qid)), ((((((((((locals.var_dqsqd_dxn_qi_dn8 * locals.var_qid) + (locals.var_dqsqd_dxn_qi * locals.var_qid_dn8)) - locals.var_aexp1d_dn8) * locals.var_a1d) - (assign22170_e22399 * locals.var_a1d_dn8)) / (locals.var_a1d * locals.var_a1d)) - locals.var_temp1_dn8) * locals.var_qid) - (assign22170_e22403 * locals.var_qid_dn8)) / (locals.var_qid * locals.var_qid)), ((((((((((locals.var_dqsqd_dxn_qi_dn9 * locals.var_qid) + (locals.var_dqsqd_dxn_qi * locals.var_qid_dn9)) - locals.var_aexp1d_dn9) * locals.var_a1d) - (assign22170_e22399 * locals.var_a1d_dn9)) / (locals.var_a1d * locals.var_a1d)) - locals.var_temp1_dn9) * locals.var_qid) - (assign22170_e22403 * locals.var_qid_dn9)) / (locals.var_qid * locals.var_qid)),)
    } else {
        (locals.var_dqid_dxn_qi, locals.var_dqid_dxn_qi_dn4, locals.var_dqid_dxn_qi_dn6, locals.var_dqid_dxn_qi_dn7, locals.var_dqid_dxn_qi_dn8, locals.var_dqid_dxn_qi_dn9,)
    }
};
        locals.var_dqid_dxn_qi = assign22170_e22407;
        locals.var_dqid_dxn_qi_dn4 = assign22170_e22407_d_n4;
        locals.var_dqid_dxn_qi_dn6 = assign22170_e22407_d_n6;
        locals.var_dqid_dxn_qi_dn7 = assign22170_e22407_d_n7;
        locals.var_dqid_dxn_qi_dn8 = assign22170_e22407_d_n8;
        locals.var_dqid_dxn_qi_dn9 = assign22170_e22407_d_n9;
        locals.var_dqid_dxn_qi_rv = 0.0;

        let (assign22180_e22421, assign22180_e22421_d_n4, assign22180_e22421_d_n6, assign22180_e22421_d_n7, assign22180_e22421_d_n8, assign22180_e22421_d_n9,) = {
    if (((locals.var_guard678 != 0.0) && (locals.var_guard679 != 0.0)) && (locals.var_guard680 != 0.0)) {
        let assign22180_e22415: f64 = (locals.var_dqid_dxn_qi * locals.var_qid);
        let assign22180_e22418: f64 = (locals.var_dqid_dxn_qi + 1.0);
        let assign22180_e22419: f64 = (assign22180_e22415 / assign22180_e22418);
        (assign22180_e22419, (((((locals.var_dqid_dxn_qi_dn4 * locals.var_qid) + (locals.var_dqid_dxn_qi * locals.var_qid_dn4)) * assign22180_e22418) - (assign22180_e22415 * locals.var_dqid_dxn_qi_dn4)) / (assign22180_e22418 * assign22180_e22418)), (((((locals.var_dqid_dxn_qi_dn6 * locals.var_qid) + (locals.var_dqid_dxn_qi * locals.var_qid_dn6)) * assign22180_e22418) - (assign22180_e22415 * locals.var_dqid_dxn_qi_dn6)) / (assign22180_e22418 * assign22180_e22418)), (((((locals.var_dqid_dxn_qi_dn7 * locals.var_qid) + (locals.var_dqid_dxn_qi * locals.var_qid_dn7)) * assign22180_e22418) - (assign22180_e22415 * locals.var_dqid_dxn_qi_dn7)) / (assign22180_e22418 * assign22180_e22418)), (((((locals.var_dqid_dxn_qi_dn8 * locals.var_qid) + (locals.var_dqid_dxn_qi * locals.var_qid_dn8)) * assign22180_e22418) - (assign22180_e22415 * locals.var_dqid_dxn_qi_dn8)) / (assign22180_e22418 * assign22180_e22418)), (((((locals.var_dqid_dxn_qi_dn9 * locals.var_qid) + (locals.var_dqid_dxn_qi * locals.var_qid_dn9)) * assign22180_e22418) - (assign22180_e22415 * locals.var_dqid_dxn_qi_dn9)) / (assign22180_e22418 * assign22180_e22418)),)
    } else {
        (locals.var_dd, locals.var_dd_dn4, locals.var_dd_dn6, locals.var_dd_dn7, locals.var_dd_dn8, locals.var_dd_dn9,)
    }
};
        locals.var_dd = assign22180_e22421;
        locals.var_dd_dn4 = assign22180_e22421_d_n4;
        locals.var_dd_dn6 = assign22180_e22421_d_n6;
        locals.var_dd_dn7 = assign22180_e22421_d_n7;
        locals.var_dd_dn8 = assign22180_e22421_d_n8;
        locals.var_dd_dn9 = assign22180_e22421_d_n9;
        locals.var_dd_rv = 0.0;

        let (assign22190_e22446, assign22190_e22446_d_n4, assign22190_e22446_d_n6, assign22190_e22446_d_n7, assign22190_e22446_d_n8, assign22190_e22446_d_n9,) = {
    if (((locals.var_guard678 != 0.0) && (locals.var_guard679 != 0.0)) && (locals.var_guard680 == 0.0)) {
        let assign22190_e22430: f64 = (locals.var_dqsqd_dxn_qi * locals.var_sumd);
        let assign22190_e22433: f64 = (locals.var_a1d * locals.var_a2d);
        let assign22190_e22434: f64 = (assign22190_e22430 / assign22190_e22433);
        let assign22190_e22437: f64 = (locals.var_aexp1d / locals.var_a1d);
        let assign22190_e22440: f64 = (locals.var_aexp2d / locals.var_a2d);
        let assign22190_e22441: f64 = (assign22190_e22437 + assign22190_e22440);
        let assign22190_e22443: f64 = (assign22190_e22441 / locals.var_qid);
        let assign22190_e22444: f64 = (assign22190_e22434 - assign22190_e22443);
        (assign22190_e22444, ((((((locals.var_dqsqd_dxn_qi_dn4 * locals.var_sumd) + (locals.var_dqsqd_dxn_qi * locals.var_sumd_dn4)) * assign22190_e22433) - (assign22190_e22430 * ((locals.var_a1d_dn4 * locals.var_a2d) + (locals.var_a1d * locals.var_a2d_dn4)))) / (assign22190_e22433 * assign22190_e22433)) - (((((((locals.var_aexp1d_dn4 * locals.var_a1d) - (locals.var_aexp1d * locals.var_a1d_dn4)) / (locals.var_a1d * locals.var_a1d)) + (((locals.var_aexp2d_dn4 * locals.var_a2d) - (locals.var_aexp2d * locals.var_a2d_dn4)) / (locals.var_a2d * locals.var_a2d))) * locals.var_qid) - (assign22190_e22441 * locals.var_qid_dn4)) / (locals.var_qid * locals.var_qid))), ((((((locals.var_dqsqd_dxn_qi_dn6 * locals.var_sumd) + (locals.var_dqsqd_dxn_qi * locals.var_sumd_dn6)) * assign22190_e22433) - (assign22190_e22430 * ((locals.var_a1d_dn6 * locals.var_a2d) + (locals.var_a1d * locals.var_a2d_dn6)))) / (assign22190_e22433 * assign22190_e22433)) - (((((((locals.var_aexp1d_dn6 * locals.var_a1d) - (locals.var_aexp1d * locals.var_a1d_dn6)) / (locals.var_a1d * locals.var_a1d)) + (((locals.var_aexp2d_dn6 * locals.var_a2d) - (locals.var_aexp2d * locals.var_a2d_dn6)) / (locals.var_a2d * locals.var_a2d))) * locals.var_qid) - (assign22190_e22441 * locals.var_qid_dn6)) / (locals.var_qid * locals.var_qid))), ((((((locals.var_dqsqd_dxn_qi_dn7 * locals.var_sumd) + (locals.var_dqsqd_dxn_qi * locals.var_sumd_dn7)) * assign22190_e22433) - (assign22190_e22430 * ((locals.var_a1d_dn7 * locals.var_a2d) + (locals.var_a1d * locals.var_a2d_dn7)))) / (assign22190_e22433 * assign22190_e22433)) - (((((((locals.var_aexp1d_dn7 * locals.var_a1d) - (locals.var_aexp1d * locals.var_a1d_dn7)) / (locals.var_a1d * locals.var_a1d)) + (((locals.var_aexp2d_dn7 * locals.var_a2d) - (locals.var_aexp2d * locals.var_a2d_dn7)) / (locals.var_a2d * locals.var_a2d))) * locals.var_qid) - (assign22190_e22441 * locals.var_qid_dn7)) / (locals.var_qid * locals.var_qid))), ((((((locals.var_dqsqd_dxn_qi_dn8 * locals.var_sumd) + (locals.var_dqsqd_dxn_qi * locals.var_sumd_dn8)) * assign22190_e22433) - (assign22190_e22430 * ((locals.var_a1d_dn8 * locals.var_a2d) + (locals.var_a1d * locals.var_a2d_dn8)))) / (assign22190_e22433 * assign22190_e22433)) - (((((((locals.var_aexp1d_dn8 * locals.var_a1d) - (locals.var_aexp1d * locals.var_a1d_dn8)) / (locals.var_a1d * locals.var_a1d)) + (((locals.var_aexp2d_dn8 * locals.var_a2d) - (locals.var_aexp2d * locals.var_a2d_dn8)) / (locals.var_a2d * locals.var_a2d))) * locals.var_qid) - (assign22190_e22441 * locals.var_qid_dn8)) / (locals.var_qid * locals.var_qid))), ((((((locals.var_dqsqd_dxn_qi_dn9 * locals.var_sumd) + (locals.var_dqsqd_dxn_qi * locals.var_sumd_dn9)) * assign22190_e22433) - (assign22190_e22430 * ((locals.var_a1d_dn9 * locals.var_a2d) + (locals.var_a1d * locals.var_a2d_dn9)))) / (assign22190_e22433 * assign22190_e22433)) - (((((((locals.var_aexp1d_dn9 * locals.var_a1d) - (locals.var_aexp1d * locals.var_a1d_dn9)) / (locals.var_a1d * locals.var_a1d)) + (((locals.var_aexp2d_dn9 * locals.var_a2d) - (locals.var_aexp2d * locals.var_a2d_dn9)) / (locals.var_a2d * locals.var_a2d))) * locals.var_qid) - (assign22190_e22441 * locals.var_qid_dn9)) / (locals.var_qid * locals.var_qid))),)
    } else {
        (locals.var_dqid_dxn_qi, locals.var_dqid_dxn_qi_dn4, locals.var_dqid_dxn_qi_dn6, locals.var_dqid_dxn_qi_dn7, locals.var_dqid_dxn_qi_dn8, locals.var_dqid_dxn_qi_dn9,)
    }
};
        locals.var_dqid_dxn_qi = assign22190_e22446;
        locals.var_dqid_dxn_qi_dn4 = assign22190_e22446_d_n4;
        locals.var_dqid_dxn_qi_dn6 = assign22190_e22446_d_n6;
        locals.var_dqid_dxn_qi_dn7 = assign22190_e22446_d_n7;
        locals.var_dqid_dxn_qi_dn8 = assign22190_e22446_d_n8;
        locals.var_dqid_dxn_qi_dn9 = assign22190_e22446_d_n9;
        locals.var_dqid_dxn_qi_rv = 0.0;

        let (assign22200_e22461, assign22200_e22461_d_n4, assign22200_e22461_d_n6, assign22200_e22461_d_n7, assign22200_e22461_d_n8, assign22200_e22461_d_n9,) = {
    if (((locals.var_guard678 != 0.0) && (locals.var_guard679 != 0.0)) && (locals.var_guard680 == 0.0)) {
        let assign22200_e22455: f64 = (locals.var_dqid_dxn_qi * locals.var_qid);
        let assign22200_e22458: f64 = (locals.var_dqid_dxn_qi + 1.0);
        let assign22200_e22459: f64 = (assign22200_e22455 / assign22200_e22458);
        (assign22200_e22459, (((((locals.var_dqid_dxn_qi_dn4 * locals.var_qid) + (locals.var_dqid_dxn_qi * locals.var_qid_dn4)) * assign22200_e22458) - (assign22200_e22455 * locals.var_dqid_dxn_qi_dn4)) / (assign22200_e22458 * assign22200_e22458)), (((((locals.var_dqid_dxn_qi_dn6 * locals.var_qid) + (locals.var_dqid_dxn_qi * locals.var_qid_dn6)) * assign22200_e22458) - (assign22200_e22455 * locals.var_dqid_dxn_qi_dn6)) / (assign22200_e22458 * assign22200_e22458)), (((((locals.var_dqid_dxn_qi_dn7 * locals.var_qid) + (locals.var_dqid_dxn_qi * locals.var_qid_dn7)) * assign22200_e22458) - (assign22200_e22455 * locals.var_dqid_dxn_qi_dn7)) / (assign22200_e22458 * assign22200_e22458)), (((((locals.var_dqid_dxn_qi_dn8 * locals.var_qid) + (locals.var_dqid_dxn_qi * locals.var_qid_dn8)) * assign22200_e22458) - (assign22200_e22455 * locals.var_dqid_dxn_qi_dn8)) / (assign22200_e22458 * assign22200_e22458)), (((((locals.var_dqid_dxn_qi_dn9 * locals.var_qid) + (locals.var_dqid_dxn_qi * locals.var_qid_dn9)) * assign22200_e22458) - (assign22200_e22455 * locals.var_dqid_dxn_qi_dn9)) / (assign22200_e22458 * assign22200_e22458)),)
    } else {
        (locals.var_dd, locals.var_dd_dn4, locals.var_dd_dn6, locals.var_dd_dn7, locals.var_dd_dn8, locals.var_dd_dn9,)
    }
};
        locals.var_dd = assign22200_e22461;
        locals.var_dd_dn4 = assign22200_e22461_d_n4;
        locals.var_dd_dn6 = assign22200_e22461_d_n6;
        locals.var_dd_dn7 = assign22200_e22461_d_n7;
        locals.var_dd_dn8 = assign22200_e22461_d_n8;
        locals.var_dd_dn9 = assign22200_e22461_d_n9;
        locals.var_dd_rv = 0.0;

        let (assign22210_e22468, assign22210_e22468_d_n4, assign22210_e22468_d_n6, assign22210_e22468_d_n7, assign22210_e22468_d_n8, assign22210_e22468_d_n9,) = {
    if ((locals.var_guard678 != 0.0) && (locals.var_guard679 == 0.0)) {
        (locals.var_dinf, locals.var_dinf_dn4, locals.var_dinf_dn6, locals.var_dinf_dn7, locals.var_dinf_dn8, locals.var_dinf_dn9,)
    } else {
        (locals.var_dd, locals.var_dd_dn4, locals.var_dd_dn6, locals.var_dd_dn7, locals.var_dd_dn8, locals.var_dd_dn9,)
    }
};
        locals.var_dd = assign22210_e22468;
        locals.var_dd_dn4 = assign22210_e22468_d_n4;
        locals.var_dd_dn6 = assign22210_e22468_d_n6;
        locals.var_dd_dn7 = assign22210_e22468_d_n7;
        locals.var_dd_dn8 = assign22210_e22468_d_n8;
        locals.var_dd_dn9 = assign22210_e22468_d_n9;
        locals.var_dd_rv = 0.0;

        let (assign22220_e22474, assign22220_e22474_d_n4, assign22220_e22474_d_n6, assign22220_e22474_d_n7, assign22220_e22474_d_n8, assign22220_e22474_d_n9,) = {
    if (locals.var_guard678 != 0.0) {
        let assign22220_e22472: f64 = (locals.var_dd - locals.var_ds);
        (assign22220_e22472, (locals.var_dd_dn4 - locals.var_ds_dn4), (locals.var_dd_dn6 - locals.var_ds_dn6), (locals.var_dd_dn7 - locals.var_ds_dn7), (locals.var_dd_dn8 - locals.var_ds_dn8), (locals.var_dd_dn9 - locals.var_ds_dn9),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign22220_e22474;
        locals.var_temp1_dn4 = assign22220_e22474_d_n4;
        locals.var_temp1_dn6 = assign22220_e22474_d_n6;
        locals.var_temp1_dn7 = assign22220_e22474_d_n7;
        locals.var_temp1_dn8 = assign22220_e22474_d_n8;
        locals.var_temp1_dn9 = assign22220_e22474_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign22230_e22484, assign22230_e22484_d_n4, assign22230_e22484_d_n6, assign22230_e22484_d_n7, assign22230_e22484_d_n8, assign22230_e22484_d_n9,) = {
    if (locals.var_guard678 != 0.0) {
        let assign22230_e22479: f64 = (36.0 * locals.var_temp1);
        let assign22230_e22481: f64 = (assign22230_e22479 * locals.var_temp1);
        let assign22230_e22482: f64 = (1.0 + assign22230_e22481);
        (assign22230_e22482, (((36.0 * locals.var_temp1_dn4) * locals.var_temp1) + (assign22230_e22479 * locals.var_temp1_dn4)), (((36.0 * locals.var_temp1_dn6) * locals.var_temp1) + (assign22230_e22479 * locals.var_temp1_dn6)), (((36.0 * locals.var_temp1_dn7) * locals.var_temp1) + (assign22230_e22479 * locals.var_temp1_dn7)), (((36.0 * locals.var_temp1_dn8) * locals.var_temp1) + (assign22230_e22479 * locals.var_temp1_dn8)), (((36.0 * locals.var_temp1_dn9) * locals.var_temp1) + (assign22230_e22479 * locals.var_temp1_dn9)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign22230_e22484;
        locals.var_temp2_dn4 = assign22230_e22484_d_n4;
        locals.var_temp2_dn6 = assign22230_e22484_d_n6;
        locals.var_temp2_dn7 = assign22230_e22484_d_n7;
        locals.var_temp2_dn8 = assign22230_e22484_d_n8;
        locals.var_temp2_dn9 = assign22230_e22484_d_n9;
        locals.var_temp2_rv = 0.0;

        let assign22240_e22486: f64 = (locals.var_temp1).abs();
        let assign22240_e22488: f64 = if assign22240_e22486 > 0.001 { 1.0 } else { 0.0 };
        locals.var_guard681 = assign22240_e22488;
        locals.var_guard681_rv = 0.0;

        let (assign22250_e22496, assign22250_e22496_d_n4, assign22250_e22496_d_n6, assign22250_e22496_d_n7, assign22250_e22496_d_n8, assign22250_e22496_d_n9,) = {
    if ((locals.var_guard678 != 0.0) && (locals.var_guard681 != 0.0)) {
        let assign22250_e22494: f64 = (locals.var_qid - locals.var_qis);
        (assign22250_e22494, (locals.var_qid_dn4 - locals.var_qis_dn4), (locals.var_qid_dn6 - locals.var_qis_dn6), (locals.var_qid_dn7 - locals.var_qis_dn7), (locals.var_qid_dn8 - locals.var_qis_dn8), (locals.var_qid_dn9 - locals.var_qis_dn9),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign22250_e22496;
        locals.var_temp3_dn4 = assign22250_e22496_d_n4;
        locals.var_temp3_dn6 = assign22250_e22496_d_n6;
        locals.var_temp3_dn7 = assign22250_e22496_d_n7;
        locals.var_temp3_dn8 = assign22250_e22496_d_n8;
        locals.var_temp3_dn9 = assign22250_e22496_d_n9;
        locals.var_temp3_rv = 0.0;

        let (assign22260_e22506, assign22260_e22506_d_n4, assign22260_e22506_d_n6, assign22260_e22506_d_n7, assign22260_e22506_d_n8, assign22260_e22506_d_n9,) = {
    if ((locals.var_guard678 != 0.0) && (locals.var_guard681 != 0.0)) {
        let assign22260_e22503: f64 = (locals.var_dd * locals.var_dxdrift);
        let assign22260_e22504: f64 = (locals.var_temp3 - assign22260_e22503);
        (assign22260_e22504, (locals.var_temp3_dn4 - ((locals.var_dd_dn4 * locals.var_dxdrift) + (locals.var_dd * locals.var_dxdrift_dn4))), (locals.var_temp3_dn6 - ((locals.var_dd_dn6 * locals.var_dxdrift) + (locals.var_dd * locals.var_dxdrift_dn6))), (locals.var_temp3_dn7 - ((locals.var_dd_dn7 * locals.var_dxdrift) + (locals.var_dd * locals.var_dxdrift_dn7))), (locals.var_temp3_dn8 - ((locals.var_dd_dn8 * locals.var_dxdrift) + (locals.var_dd * locals.var_dxdrift_dn8))), (locals.var_temp3_dn9 - ((locals.var_dd_dn9 * locals.var_dxdrift) + (locals.var_dd * locals.var_dxdrift_dn9))),)
    } else {
        (locals.var_ls, locals.var_ls_dn4, locals.var_ls_dn6, locals.var_ls_dn7, locals.var_ls_dn8, locals.var_ls_dn9,)
    }
};
        locals.var_ls = assign22260_e22506;
        locals.var_ls_dn4 = assign22260_e22506_d_n4;
        locals.var_ls_dn6 = assign22260_e22506_d_n6;
        locals.var_ls_dn7 = assign22260_e22506_d_n7;
        locals.var_ls_dn8 = assign22260_e22506_d_n8;
        locals.var_ls_dn9 = assign22260_e22506_d_n9;
        locals.var_ls_rv = 0.0;

        let (assign22270_e22516, assign22270_e22516_d_n4, assign22270_e22516_d_n6, assign22270_e22516_d_n7, assign22270_e22516_d_n8, assign22270_e22516_d_n9,) = {
    if ((locals.var_guard678 != 0.0) && (locals.var_guard681 != 0.0)) {
        let assign22270_e22513: f64 = (locals.var_ds * locals.var_dxdrift);
        let assign22270_e22514: f64 = (locals.var_temp3 - assign22270_e22513);
        (assign22270_e22514, (locals.var_temp3_dn4 - ((locals.var_ds_dn4 * locals.var_dxdrift) + (locals.var_ds * locals.var_dxdrift_dn4))), (locals.var_temp3_dn6 - ((locals.var_ds_dn6 * locals.var_dxdrift) + (locals.var_ds * locals.var_dxdrift_dn6))), (locals.var_temp3_dn7 - ((locals.var_ds_dn7 * locals.var_dxdrift) + (locals.var_ds * locals.var_dxdrift_dn7))), (locals.var_temp3_dn8 - ((locals.var_ds_dn8 * locals.var_dxdrift) + (locals.var_ds * locals.var_dxdrift_dn8))), (locals.var_temp3_dn9 - ((locals.var_ds_dn9 * locals.var_dxdrift) + (locals.var_ds * locals.var_dxdrift_dn9))),)
    } else {
        (locals.var_ld, locals.var_ld_dn4, locals.var_ld_dn6, locals.var_ld_dn7, locals.var_ld_dn8, locals.var_ld_dn9,)
    }
};
        locals.var_ld = assign22270_e22516;
        locals.var_ld_dn4 = assign22270_e22516_d_n4;
        locals.var_ld_dn6 = assign22270_e22516_d_n6;
        locals.var_ld_dn7 = assign22270_e22516_d_n7;
        locals.var_ld_dn8 = assign22270_e22516_d_n8;
        locals.var_ld_dn9 = assign22270_e22516_d_n9;
        locals.var_ld_rv = 0.0;

        let (assign22280_e22527, assign22280_e22527_d_n4, assign22280_e22527_d_n6, assign22280_e22527_d_n7, assign22280_e22527_d_n8, assign22280_e22527_d_n9,) = {
    if ((locals.var_guard678 != 0.0) && (locals.var_guard681 != 0.0)) {
        let assign22280_e22522: f64 = (locals.var_ls * locals.var_ls);
        let assign22280_e22524: f64 = (assign22280_e22522 + locals.var_temp2);
        let assign22280_e22525: f64 = (assign22280_e22524).sqrt();
        (assign22280_e22525, ((((locals.var_ls_dn4 * locals.var_ls) + (locals.var_ls * locals.var_ls_dn4)) + locals.var_temp2_dn4) / (2.0 * assign22280_e22525)), ((((locals.var_ls_dn6 * locals.var_ls) + (locals.var_ls * locals.var_ls_dn6)) + locals.var_temp2_dn6) / (2.0 * assign22280_e22525)), ((((locals.var_ls_dn7 * locals.var_ls) + (locals.var_ls * locals.var_ls_dn7)) + locals.var_temp2_dn7) / (2.0 * assign22280_e22525)), ((((locals.var_ls_dn8 * locals.var_ls) + (locals.var_ls * locals.var_ls_dn8)) + locals.var_temp2_dn8) / (2.0 * assign22280_e22525)), ((((locals.var_ls_dn9 * locals.var_ls) + (locals.var_ls * locals.var_ls_dn9)) + locals.var_temp2_dn9) / (2.0 * assign22280_e22525)),)
    } else {
        (locals.var_us, locals.var_us_dn4, locals.var_us_dn6, locals.var_us_dn7, locals.var_us_dn8, locals.var_us_dn9,)
    }
};
        locals.var_us = assign22280_e22527;
        locals.var_us_dn4 = assign22280_e22527_d_n4;
        locals.var_us_dn6 = assign22280_e22527_d_n6;
        locals.var_us_dn7 = assign22280_e22527_d_n7;
        locals.var_us_dn8 = assign22280_e22527_d_n8;
        locals.var_us_dn9 = assign22280_e22527_d_n9;
        locals.var_us_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_61(
        locals: &mut StampLocals,
    ) {
        let (assign22290_e22538, assign22290_e22538_d_n4, assign22290_e22538_d_n6, assign22290_e22538_d_n7, assign22290_e22538_d_n8, assign22290_e22538_d_n9,) = {
    if ((locals.var_guard678 != 0.0) && (locals.var_guard681 != 0.0)) {
        let assign22290_e22533: f64 = (locals.var_ld * locals.var_ld);
        let assign22290_e22535: f64 = (assign22290_e22533 + locals.var_temp2);
        let assign22290_e22536: f64 = (assign22290_e22535).sqrt();
        (assign22290_e22536, ((((locals.var_ld_dn4 * locals.var_ld) + (locals.var_ld * locals.var_ld_dn4)) + locals.var_temp2_dn4) / (2.0 * assign22290_e22536)), ((((locals.var_ld_dn6 * locals.var_ld) + (locals.var_ld * locals.var_ld_dn6)) + locals.var_temp2_dn6) / (2.0 * assign22290_e22536)), ((((locals.var_ld_dn7 * locals.var_ld) + (locals.var_ld * locals.var_ld_dn7)) + locals.var_temp2_dn7) / (2.0 * assign22290_e22536)), ((((locals.var_ld_dn8 * locals.var_ld) + (locals.var_ld * locals.var_ld_dn8)) + locals.var_temp2_dn8) / (2.0 * assign22290_e22536)), ((((locals.var_ld_dn9 * locals.var_ld) + (locals.var_ld * locals.var_ld_dn9)) + locals.var_temp2_dn9) / (2.0 * assign22290_e22536)),)
    } else {
        (locals.var_ud, locals.var_ud_dn4, locals.var_ud_dn6, locals.var_ud_dn7, locals.var_ud_dn8, locals.var_ud_dn9,)
    }
};
        locals.var_ud = assign22290_e22538;
        locals.var_ud_dn4 = assign22290_e22538_d_n4;
        locals.var_ud_dn6 = assign22290_e22538_d_n6;
        locals.var_ud_dn7 = assign22290_e22538_d_n7;
        locals.var_ud_dn8 = assign22290_e22538_d_n8;
        locals.var_ud_dn9 = assign22290_e22538_d_n9;
        locals.var_ud_rv = 0.0;

        let (assign22300_e22565, assign22300_e22565_d_n4, assign22300_e22565_d_n6, assign22300_e22565_d_n7, assign22300_e22565_d_n8, assign22300_e22565_d_n9,) = {
    if ((locals.var_guard678 != 0.0) && (locals.var_guard681 != 0.0)) {
        let assign22300_e22544: f64 = (0.25 / locals.var_temp1);
        let assign22300_e22547: f64 = (locals.var_ud * locals.var_ls);
        let assign22300_e22550: f64 = (locals.var_us * locals.var_ld);
        let assign22300_e22551: f64 = (assign22300_e22547 - assign22300_e22550);
        let assign22300_e22555: f64 = (locals.var_ld + locals.var_ud);
        let assign22300_e22558: f64 = (locals.var_ls + locals.var_us);
        let assign22300_e22559: f64 = (assign22300_e22555 / assign22300_e22558);
        let assign22300_e22560: f64 = (assign22300_e22559).ln();
        let assign22300_e22561: f64 = (locals.var_temp2 * assign22300_e22560);
        let assign22300_e22562: f64 = (assign22300_e22551 + assign22300_e22561);
        let assign22300_e22563: f64 = (assign22300_e22544 * assign22300_e22562);
        (assign22300_e22563, (((-((0.25 * locals.var_temp1_dn4) / (locals.var_temp1 * locals.var_temp1))) * assign22300_e22562) + (assign22300_e22544 * ((((locals.var_ud_dn4 * locals.var_ls) + (locals.var_ud * locals.var_ls_dn4)) - ((locals.var_us_dn4 * locals.var_ld) + (locals.var_us * locals.var_ld_dn4))) + ((locals.var_temp2_dn4 * assign22300_e22560) + (locals.var_temp2 * (((((locals.var_ld_dn4 + locals.var_ud_dn4) * assign22300_e22558) - (assign22300_e22555 * (locals.var_ls_dn4 + locals.var_us_dn4))) / (assign22300_e22558 * assign22300_e22558)) / assign22300_e22559)))))), (((-((0.25 * locals.var_temp1_dn6) / (locals.var_temp1 * locals.var_temp1))) * assign22300_e22562) + (assign22300_e22544 * ((((locals.var_ud_dn6 * locals.var_ls) + (locals.var_ud * locals.var_ls_dn6)) - ((locals.var_us_dn6 * locals.var_ld) + (locals.var_us * locals.var_ld_dn6))) + ((locals.var_temp2_dn6 * assign22300_e22560) + (locals.var_temp2 * (((((locals.var_ld_dn6 + locals.var_ud_dn6) * assign22300_e22558) - (assign22300_e22555 * (locals.var_ls_dn6 + locals.var_us_dn6))) / (assign22300_e22558 * assign22300_e22558)) / assign22300_e22559)))))), (((-((0.25 * locals.var_temp1_dn7) / (locals.var_temp1 * locals.var_temp1))) * assign22300_e22562) + (assign22300_e22544 * ((((locals.var_ud_dn7 * locals.var_ls) + (locals.var_ud * locals.var_ls_dn7)) - ((locals.var_us_dn7 * locals.var_ld) + (locals.var_us * locals.var_ld_dn7))) + ((locals.var_temp2_dn7 * assign22300_e22560) + (locals.var_temp2 * (((((locals.var_ld_dn7 + locals.var_ud_dn7) * assign22300_e22558) - (assign22300_e22555 * (locals.var_ls_dn7 + locals.var_us_dn7))) / (assign22300_e22558 * assign22300_e22558)) / assign22300_e22559)))))), (((-((0.25 * locals.var_temp1_dn8) / (locals.var_temp1 * locals.var_temp1))) * assign22300_e22562) + (assign22300_e22544 * ((((locals.var_ud_dn8 * locals.var_ls) + (locals.var_ud * locals.var_ls_dn8)) - ((locals.var_us_dn8 * locals.var_ld) + (locals.var_us * locals.var_ld_dn8))) + ((locals.var_temp2_dn8 * assign22300_e22560) + (locals.var_temp2 * (((((locals.var_ld_dn8 + locals.var_ud_dn8) * assign22300_e22558) - (assign22300_e22555 * (locals.var_ls_dn8 + locals.var_us_dn8))) / (assign22300_e22558 * assign22300_e22558)) / assign22300_e22559)))))), (((-((0.25 * locals.var_temp1_dn9) / (locals.var_temp1 * locals.var_temp1))) * assign22300_e22562) + (assign22300_e22544 * ((((locals.var_ud_dn9 * locals.var_ls) + (locals.var_ud * locals.var_ls_dn9)) - ((locals.var_us_dn9 * locals.var_ld) + (locals.var_us * locals.var_ld_dn9))) + ((locals.var_temp2_dn9 * assign22300_e22560) + (locals.var_temp2 * (((((locals.var_ld_dn9 + locals.var_ud_dn9) * assign22300_e22558) - (assign22300_e22555 * (locals.var_ls_dn9 + locals.var_us_dn9))) / (assign22300_e22558 * assign22300_e22558)) / assign22300_e22559)))))),)
    } else {
        (locals.var_idrift2, locals.var_idrift2_dn4, locals.var_idrift2_dn6, locals.var_idrift2_dn7, locals.var_idrift2_dn8, locals.var_idrift2_dn9,)
    }
};
        locals.var_idrift2 = assign22300_e22565;
        locals.var_idrift2_dn4 = assign22300_e22565_d_n4;
        locals.var_idrift2_dn6 = assign22300_e22565_d_n6;
        locals.var_idrift2_dn7 = assign22300_e22565_d_n7;
        locals.var_idrift2_dn8 = assign22300_e22565_d_n8;
        locals.var_idrift2_dn9 = assign22300_e22565_d_n9;
        locals.var_idrift2_rv = 0.0;

        let (assign22310_e22574, assign22310_e22574_d_n4, assign22310_e22574_d_n6, assign22310_e22574_d_n7, assign22310_e22574_d_n8, assign22310_e22574_d_n9,) = {
    if ((locals.var_guard678 != 0.0) && (locals.var_guard681 == 0.0)) {
        let assign22310_e22572: f64 = (locals.var_dxdrift * locals.var_temp1);
        (assign22310_e22572, ((locals.var_dxdrift_dn4 * locals.var_temp1) + (locals.var_dxdrift * locals.var_temp1_dn4)), ((locals.var_dxdrift_dn6 * locals.var_temp1) + (locals.var_dxdrift * locals.var_temp1_dn6)), ((locals.var_dxdrift_dn7 * locals.var_temp1) + (locals.var_dxdrift * locals.var_temp1_dn7)), ((locals.var_dxdrift_dn8 * locals.var_temp1) + (locals.var_dxdrift * locals.var_temp1_dn8)), ((locals.var_dxdrift_dn9 * locals.var_temp1) + (locals.var_dxdrift * locals.var_temp1_dn9)),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign22310_e22574;
        locals.var_temp3_dn4 = assign22310_e22574_d_n4;
        locals.var_temp3_dn6 = assign22310_e22574_d_n6;
        locals.var_temp3_dn7 = assign22310_e22574_d_n7;
        locals.var_temp3_dn8 = assign22310_e22574_d_n8;
        locals.var_temp3_dn9 = assign22310_e22574_d_n9;
        locals.var_temp3_rv = 0.0;

        let (assign22320_e22593, assign22320_e22593_d_n4, assign22320_e22593_d_n6, assign22320_e22593_d_n7, assign22320_e22593_d_n8, assign22320_e22593_d_n9,) = {
    if ((locals.var_guard678 != 0.0) && (locals.var_guard681 == 0.0)) {
        let assign22320_e22580: f64 = (-0.25);
        let assign22320_e22582: f64 = (assign22320_e22580 * 0.1666666666667);
        let assign22320_e22584: f64 = (assign22320_e22582 * locals.var_dxdrift);
        let assign22320_e22586: f64 = (assign22320_e22584 * locals.var_temp3);
        let assign22320_e22588: f64 = (assign22320_e22586 * locals.var_temp3);
        let assign22320_e22590: f64 = (locals.var_temp2).sqrt();
        let assign22320_e22591: f64 = (assign22320_e22588 / assign22320_e22590);
        (assign22320_e22591, ((((((((assign22320_e22582 * locals.var_dxdrift_dn4) * locals.var_temp3) + (assign22320_e22584 * locals.var_temp3_dn4)) * locals.var_temp3) + (assign22320_e22586 * locals.var_temp3_dn4)) * assign22320_e22590) - (assign22320_e22588 * (locals.var_temp2_dn4 / (2.0 * assign22320_e22590)))) / (assign22320_e22590 * assign22320_e22590)), ((((((((assign22320_e22582 * locals.var_dxdrift_dn6) * locals.var_temp3) + (assign22320_e22584 * locals.var_temp3_dn6)) * locals.var_temp3) + (assign22320_e22586 * locals.var_temp3_dn6)) * assign22320_e22590) - (assign22320_e22588 * (locals.var_temp2_dn6 / (2.0 * assign22320_e22590)))) / (assign22320_e22590 * assign22320_e22590)), ((((((((assign22320_e22582 * locals.var_dxdrift_dn7) * locals.var_temp3) + (assign22320_e22584 * locals.var_temp3_dn7)) * locals.var_temp3) + (assign22320_e22586 * locals.var_temp3_dn7)) * assign22320_e22590) - (assign22320_e22588 * (locals.var_temp2_dn7 / (2.0 * assign22320_e22590)))) / (assign22320_e22590 * assign22320_e22590)), ((((((((assign22320_e22582 * locals.var_dxdrift_dn8) * locals.var_temp3) + (assign22320_e22584 * locals.var_temp3_dn8)) * locals.var_temp3) + (assign22320_e22586 * locals.var_temp3_dn8)) * assign22320_e22590) - (assign22320_e22588 * (locals.var_temp2_dn8 / (2.0 * assign22320_e22590)))) / (assign22320_e22590 * assign22320_e22590)), ((((((((assign22320_e22582 * locals.var_dxdrift_dn9) * locals.var_temp3) + (assign22320_e22584 * locals.var_temp3_dn9)) * locals.var_temp3) + (assign22320_e22586 * locals.var_temp3_dn9)) * assign22320_e22590) - (assign22320_e22588 * (locals.var_temp2_dn9 / (2.0 * assign22320_e22590)))) / (assign22320_e22590 * assign22320_e22590)),)
    } else {
        (locals.var_idrift2, locals.var_idrift2_dn4, locals.var_idrift2_dn6, locals.var_idrift2_dn7, locals.var_idrift2_dn8, locals.var_idrift2_dn9,)
    }
};
        locals.var_idrift2 = assign22320_e22593;
        locals.var_idrift2_dn4 = assign22320_e22593_d_n4;
        locals.var_idrift2_dn6 = assign22320_e22593_d_n6;
        locals.var_idrift2_dn7 = assign22320_e22593_d_n7;
        locals.var_idrift2_dn8 = assign22320_e22593_d_n8;
        locals.var_idrift2_dn9 = assign22320_e22593_d_n9;
        locals.var_idrift2_rv = 0.0;

        let (assign22330_e22598, assign22330_e22598_d_n4, assign22330_e22598_d_n6, assign22330_e22598_d_n7, assign22330_e22598_d_n8, assign22330_e22598_d_n9,) = {
    if (locals.var_guard678 == 0.0) {
        (locals.var_dinf, locals.var_dinf_dn4, locals.var_dinf_dn6, locals.var_dinf_dn7, locals.var_dinf_dn8, locals.var_dinf_dn9,)
    } else {
        (locals.var_dd, locals.var_dd_dn4, locals.var_dd_dn6, locals.var_dd_dn7, locals.var_dd_dn8, locals.var_dd_dn9,)
    }
};
        locals.var_dd = assign22330_e22598;
        locals.var_dd_dn4 = assign22330_e22598_d_n4;
        locals.var_dd_dn6 = assign22330_e22598_d_n6;
        locals.var_dd_dn7 = assign22330_e22598_d_n7;
        locals.var_dd_dn8 = assign22330_e22598_d_n8;
        locals.var_dd_dn9 = assign22330_e22598_d_n9;
        locals.var_dd_rv = 0.0;

        let (assign22340_e22603, assign22340_e22603_d_n4, assign22340_e22603_d_n6, assign22340_e22603_d_n7, assign22340_e22603_d_n8, assign22340_e22603_d_n9,) = {
    if (locals.var_guard678 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_idrift2, locals.var_idrift2_dn4, locals.var_idrift2_dn6, locals.var_idrift2_dn7, locals.var_idrift2_dn8, locals.var_idrift2_dn9,)
    }
};
        locals.var_idrift2 = assign22340_e22603;
        locals.var_idrift2_dn4 = assign22340_e22603_d_n4;
        locals.var_idrift2_dn6 = assign22340_e22603_d_n6;
        locals.var_idrift2_dn7 = assign22340_e22603_d_n7;
        locals.var_idrift2_dn8 = assign22340_e22603_d_n8;
        locals.var_idrift2_dn9 = assign22340_e22603_d_n9;
        locals.var_idrift2_rv = 0.0;

        let assign22350_e22606: f64 = (locals.var_qim * locals.var_dxdrift);
        let assign22350_e22608: f64 = (assign22350_e22606 + locals.var_idrift2);
        let assign22350_e22610: f64 = (assign22350_e22608 + locals.var_qis);
        let assign22350_e22612: f64 = (assign22350_e22610 - locals.var_qid);
        locals.var_norm_ids = assign22350_e22612;
        locals.var_norm_ids_dn4 = (((((locals.var_qim_dn4 * locals.var_dxdrift) + (locals.var_qim * locals.var_dxdrift_dn4)) + locals.var_idrift2_dn4) + locals.var_qis_dn4) - locals.var_qid_dn4);
        locals.var_norm_ids_dn6 = (((((locals.var_qim_dn6 * locals.var_dxdrift) + (locals.var_qim * locals.var_dxdrift_dn6)) + locals.var_idrift2_dn6) + locals.var_qis_dn6) - locals.var_qid_dn6);
        locals.var_norm_ids_dn7 = (((((locals.var_qim_dn7 * locals.var_dxdrift) + (locals.var_qim * locals.var_dxdrift_dn7)) + locals.var_idrift2_dn7) + locals.var_qis_dn7) - locals.var_qid_dn7);
        locals.var_norm_ids_dn8 = (((((locals.var_qim_dn8 * locals.var_dxdrift) + (locals.var_qim * locals.var_dxdrift_dn8)) + locals.var_idrift2_dn8) + locals.var_qis_dn8) - locals.var_qid_dn8);
        locals.var_norm_ids_dn9 = (((((locals.var_qim_dn9 * locals.var_dxdrift) + (locals.var_qim * locals.var_dxdrift_dn9)) + locals.var_idrift2_dn9) + locals.var_qis_dn9) - locals.var_qid_dn9);
        locals.var_norm_ids_rv = 0.0;

        let assign22360_e22615: f64 = if locals.var_qis > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard682 = assign22360_e22615;
        locals.var_guard682_rv = 0.0;

        let assign22370_e22618: f64 = if locals.var_norm_ids > 1e-30 { 1.0 } else { 0.0 };
        locals.var_guard683 = assign22370_e22618;
        locals.var_guard683_rv = 0.0;

        let (assign22380_e22630, assign22380_e22630_d_n4, assign22380_e22630_d_n6, assign22380_e22630_d_n7, assign22380_e22630_d_n8, assign22380_e22630_d_n9,) = {
    if ((locals.var_guard682 != 0.0) && (locals.var_guard683 != 0.0)) {
        let assign22380_e22625: f64 = (locals.var_aexp1s / locals.var_qis);
        let assign22380_e22627: f64 = (assign22380_e22625 - locals.var_dqsqs_dxn_qi);
        let assign22380_e22628: f64 = (locals.var_a1s / assign22380_e22627);
        (assign22380_e22628, (((locals.var_a1s_dn4 * assign22380_e22627) - (locals.var_a1s * ((((locals.var_aexp1s_dn4 * locals.var_qis) - (locals.var_aexp1s * locals.var_qis_dn4)) / (locals.var_qis * locals.var_qis)) - locals.var_dqsqs_dxn_qi_dn4))) / (assign22380_e22627 * assign22380_e22627)), (((locals.var_a1s_dn6 * assign22380_e22627) - (locals.var_a1s * ((((locals.var_aexp1s_dn6 * locals.var_qis) - (locals.var_aexp1s * locals.var_qis_dn6)) / (locals.var_qis * locals.var_qis)) - locals.var_dqsqs_dxn_qi_dn6))) / (assign22380_e22627 * assign22380_e22627)), (((locals.var_a1s_dn7 * assign22380_e22627) - (locals.var_a1s * ((((locals.var_aexp1s_dn7 * locals.var_qis) - (locals.var_aexp1s * locals.var_qis_dn7)) / (locals.var_qis * locals.var_qis)) - locals.var_dqsqs_dxn_qi_dn7))) / (assign22380_e22627 * assign22380_e22627)), (((locals.var_a1s_dn8 * assign22380_e22627) - (locals.var_a1s * ((((locals.var_aexp1s_dn8 * locals.var_qis) - (locals.var_aexp1s * locals.var_qis_dn8)) / (locals.var_qis * locals.var_qis)) - locals.var_dqsqs_dxn_qi_dn8))) / (assign22380_e22627 * assign22380_e22627)), (((locals.var_a1s_dn9 * assign22380_e22627) - (locals.var_a1s * ((((locals.var_aexp1s_dn9 * locals.var_qis) - (locals.var_aexp1s * locals.var_qis_dn9)) / (locals.var_qis * locals.var_qis)) - locals.var_dqsqs_dxn_qi_dn9))) / (assign22380_e22627 * assign22380_e22627)),)
    } else {
        (locals.var_q1s_chap, locals.var_q1s_chap_dn4, locals.var_q1s_chap_dn6, locals.var_q1s_chap_dn7, locals.var_q1s_chap_dn8, locals.var_q1s_chap_dn9,)
    }
};
        locals.var_q1s_chap = assign22380_e22630;
        locals.var_q1s_chap_dn4 = assign22380_e22630_d_n4;
        locals.var_q1s_chap_dn6 = assign22380_e22630_d_n6;
        locals.var_q1s_chap_dn7 = assign22380_e22630_d_n7;
        locals.var_q1s_chap_dn8 = assign22380_e22630_d_n8;
        locals.var_q1s_chap_dn9 = assign22380_e22630_d_n9;
        locals.var_q1s_chap_rv = 0.0;

        let (assign22390_e22642, assign22390_e22642_d_n4, assign22390_e22642_d_n6, assign22390_e22642_d_n7, assign22390_e22642_d_n8, assign22390_e22642_d_n9,) = {
    if ((locals.var_guard682 != 0.0) && (locals.var_guard683 != 0.0)) {
        let assign22390_e22637: f64 = (locals.var_aexp1d / locals.var_qid);
        let assign22390_e22639: f64 = (assign22390_e22637 - locals.var_dqsqd_dxn_qi);
        let assign22390_e22640: f64 = (locals.var_a1d / assign22390_e22639);
        (assign22390_e22640, (((locals.var_a1d_dn4 * assign22390_e22639) - (locals.var_a1d * ((((locals.var_aexp1d_dn4 * locals.var_qid) - (locals.var_aexp1d * locals.var_qid_dn4)) / (locals.var_qid * locals.var_qid)) - locals.var_dqsqd_dxn_qi_dn4))) / (assign22390_e22639 * assign22390_e22639)), (((locals.var_a1d_dn6 * assign22390_e22639) - (locals.var_a1d * ((((locals.var_aexp1d_dn6 * locals.var_qid) - (locals.var_aexp1d * locals.var_qid_dn6)) / (locals.var_qid * locals.var_qid)) - locals.var_dqsqd_dxn_qi_dn6))) / (assign22390_e22639 * assign22390_e22639)), (((locals.var_a1d_dn7 * assign22390_e22639) - (locals.var_a1d * ((((locals.var_aexp1d_dn7 * locals.var_qid) - (locals.var_aexp1d * locals.var_qid_dn7)) / (locals.var_qid * locals.var_qid)) - locals.var_dqsqd_dxn_qi_dn7))) / (assign22390_e22639 * assign22390_e22639)), (((locals.var_a1d_dn8 * assign22390_e22639) - (locals.var_a1d * ((((locals.var_aexp1d_dn8 * locals.var_qid) - (locals.var_aexp1d * locals.var_qid_dn8)) / (locals.var_qid * locals.var_qid)) - locals.var_dqsqd_dxn_qi_dn8))) / (assign22390_e22639 * assign22390_e22639)), (((locals.var_a1d_dn9 * assign22390_e22639) - (locals.var_a1d * ((((locals.var_aexp1d_dn9 * locals.var_qid) - (locals.var_aexp1d * locals.var_qid_dn9)) / (locals.var_qid * locals.var_qid)) - locals.var_dqsqd_dxn_qi_dn9))) / (assign22390_e22639 * assign22390_e22639)),)
    } else {
        (locals.var_q1d_chap, locals.var_q1d_chap_dn4, locals.var_q1d_chap_dn6, locals.var_q1d_chap_dn7, locals.var_q1d_chap_dn8, locals.var_q1d_chap_dn9,)
    }
};
        locals.var_q1d_chap = assign22390_e22642;
        locals.var_q1d_chap_dn4 = assign22390_e22642_d_n4;
        locals.var_q1d_chap_dn6 = assign22390_e22642_d_n6;
        locals.var_q1d_chap_dn7 = assign22390_e22642_d_n7;
        locals.var_q1d_chap_dn8 = assign22390_e22642_d_n8;
        locals.var_q1d_chap_dn9 = assign22390_e22642_d_n9;
        locals.var_q1d_chap_rv = 0.0;

        let (assign22400_e22652, assign22400_e22652_d_n4, assign22400_e22652_d_n6, assign22400_e22652_d_n7, assign22400_e22652_d_n8, assign22400_e22652_d_n9,) = {
    if ((locals.var_guard682 != 0.0) && (locals.var_guard683 != 0.0)) {
        let assign22400_e22648: f64 = (locals.var_q1s_chap - locals.var_q1d_chap);
        let assign22400_e22650: f64 = (assign22400_e22648 / locals.var_norm_ids);
        (assign22400_e22650, ((((locals.var_q1s_chap_dn4 - locals.var_q1d_chap_dn4) * locals.var_norm_ids) - (assign22400_e22648 * locals.var_norm_ids_dn4)) / (locals.var_norm_ids * locals.var_norm_ids)), ((((locals.var_q1s_chap_dn6 - locals.var_q1d_chap_dn6) * locals.var_norm_ids) - (assign22400_e22648 * locals.var_norm_ids_dn6)) / (locals.var_norm_ids * locals.var_norm_ids)), ((((locals.var_q1s_chap_dn7 - locals.var_q1d_chap_dn7) * locals.var_norm_ids) - (assign22400_e22648 * locals.var_norm_ids_dn7)) / (locals.var_norm_ids * locals.var_norm_ids)), ((((locals.var_q1s_chap_dn8 - locals.var_q1d_chap_dn8) * locals.var_norm_ids) - (assign22400_e22648 * locals.var_norm_ids_dn8)) / (locals.var_norm_ids * locals.var_norm_ids)), ((((locals.var_q1s_chap_dn9 - locals.var_q1d_chap_dn9) * locals.var_norm_ids) - (assign22400_e22648 * locals.var_norm_ids_dn9)) / (locals.var_norm_ids * locals.var_norm_ids)),)
    } else {
        (locals.var_inv_k1h1_0, locals.var_inv_k1h1_0_dn4, locals.var_inv_k1h1_0_dn6, locals.var_inv_k1h1_0_dn7, locals.var_inv_k1h1_0_dn8, locals.var_inv_k1h1_0_dn9,)
    }
};
        locals.var_inv_k1h1_0 = assign22400_e22652;
        locals.var_inv_k1h1_0_dn4 = assign22400_e22652_d_n4;
        locals.var_inv_k1h1_0_dn6 = assign22400_e22652_d_n6;
        locals.var_inv_k1h1_0_dn7 = assign22400_e22652_d_n7;
        locals.var_inv_k1h1_0_dn8 = assign22400_e22652_d_n8;
        locals.var_inv_k1h1_0_dn9 = assign22400_e22652_d_n9;
        locals.var_inv_k1h1_0_rv = 0.0;

        let (assign22410_e22664, assign22410_e22664_d_n4, assign22410_e22664_d_n6, assign22410_e22664_d_n7, assign22410_e22664_d_n8, assign22410_e22664_d_n9,) = {
    if ((locals.var_guard682 != 0.0) && (locals.var_guard683 != 0.0)) {
        let assign22410_e22659: f64 = (locals.var_aexp2s / locals.var_qis);
        let assign22410_e22661: f64 = (assign22410_e22659 - locals.var_dqsqs_dxn_qi);
        let assign22410_e22662: f64 = (locals.var_a2s / assign22410_e22661);
        (assign22410_e22662, (((locals.var_a2s_dn4 * assign22410_e22661) - (locals.var_a2s * ((((locals.var_aexp2s_dn4 * locals.var_qis) - (locals.var_aexp2s * locals.var_qis_dn4)) / (locals.var_qis * locals.var_qis)) - locals.var_dqsqs_dxn_qi_dn4))) / (assign22410_e22661 * assign22410_e22661)), (((locals.var_a2s_dn6 * assign22410_e22661) - (locals.var_a2s * ((((locals.var_aexp2s_dn6 * locals.var_qis) - (locals.var_aexp2s * locals.var_qis_dn6)) / (locals.var_qis * locals.var_qis)) - locals.var_dqsqs_dxn_qi_dn6))) / (assign22410_e22661 * assign22410_e22661)), (((locals.var_a2s_dn7 * assign22410_e22661) - (locals.var_a2s * ((((locals.var_aexp2s_dn7 * locals.var_qis) - (locals.var_aexp2s * locals.var_qis_dn7)) / (locals.var_qis * locals.var_qis)) - locals.var_dqsqs_dxn_qi_dn7))) / (assign22410_e22661 * assign22410_e22661)), (((locals.var_a2s_dn8 * assign22410_e22661) - (locals.var_a2s * ((((locals.var_aexp2s_dn8 * locals.var_qis) - (locals.var_aexp2s * locals.var_qis_dn8)) / (locals.var_qis * locals.var_qis)) - locals.var_dqsqs_dxn_qi_dn8))) / (assign22410_e22661 * assign22410_e22661)), (((locals.var_a2s_dn9 * assign22410_e22661) - (locals.var_a2s * ((((locals.var_aexp2s_dn9 * locals.var_qis) - (locals.var_aexp2s * locals.var_qis_dn9)) / (locals.var_qis * locals.var_qis)) - locals.var_dqsqs_dxn_qi_dn9))) / (assign22410_e22661 * assign22410_e22661)),)
    } else {
        (locals.var_q2s_chap, locals.var_q2s_chap_dn4, locals.var_q2s_chap_dn6, locals.var_q2s_chap_dn7, locals.var_q2s_chap_dn8, locals.var_q2s_chap_dn9,)
    }
};
        locals.var_q2s_chap = assign22410_e22664;
        locals.var_q2s_chap_dn4 = assign22410_e22664_d_n4;
        locals.var_q2s_chap_dn6 = assign22410_e22664_d_n6;
        locals.var_q2s_chap_dn7 = assign22410_e22664_d_n7;
        locals.var_q2s_chap_dn8 = assign22410_e22664_d_n8;
        locals.var_q2s_chap_dn9 = assign22410_e22664_d_n9;
        locals.var_q2s_chap_rv = 0.0;

        let (assign22420_e22676, assign22420_e22676_d_n4, assign22420_e22676_d_n6, assign22420_e22676_d_n7, assign22420_e22676_d_n8, assign22420_e22676_d_n9,) = {
    if ((locals.var_guard682 != 0.0) && (locals.var_guard683 != 0.0)) {
        let assign22420_e22671: f64 = (locals.var_aexp2d / locals.var_qid);
        let assign22420_e22673: f64 = (assign22420_e22671 - locals.var_dqsqd_dxn_qi);
        let assign22420_e22674: f64 = (locals.var_a2d / assign22420_e22673);
        (assign22420_e22674, (((locals.var_a2d_dn4 * assign22420_e22673) - (locals.var_a2d * ((((locals.var_aexp2d_dn4 * locals.var_qid) - (locals.var_aexp2d * locals.var_qid_dn4)) / (locals.var_qid * locals.var_qid)) - locals.var_dqsqd_dxn_qi_dn4))) / (assign22420_e22673 * assign22420_e22673)), (((locals.var_a2d_dn6 * assign22420_e22673) - (locals.var_a2d * ((((locals.var_aexp2d_dn6 * locals.var_qid) - (locals.var_aexp2d * locals.var_qid_dn6)) / (locals.var_qid * locals.var_qid)) - locals.var_dqsqd_dxn_qi_dn6))) / (assign22420_e22673 * assign22420_e22673)), (((locals.var_a2d_dn7 * assign22420_e22673) - (locals.var_a2d * ((((locals.var_aexp2d_dn7 * locals.var_qid) - (locals.var_aexp2d * locals.var_qid_dn7)) / (locals.var_qid * locals.var_qid)) - locals.var_dqsqd_dxn_qi_dn7))) / (assign22420_e22673 * assign22420_e22673)), (((locals.var_a2d_dn8 * assign22420_e22673) - (locals.var_a2d * ((((locals.var_aexp2d_dn8 * locals.var_qid) - (locals.var_aexp2d * locals.var_qid_dn8)) / (locals.var_qid * locals.var_qid)) - locals.var_dqsqd_dxn_qi_dn8))) / (assign22420_e22673 * assign22420_e22673)), (((locals.var_a2d_dn9 * assign22420_e22673) - (locals.var_a2d * ((((locals.var_aexp2d_dn9 * locals.var_qid) - (locals.var_aexp2d * locals.var_qid_dn9)) / (locals.var_qid * locals.var_qid)) - locals.var_dqsqd_dxn_qi_dn9))) / (assign22420_e22673 * assign22420_e22673)),)
    } else {
        (locals.var_q2d_chap, locals.var_q2d_chap_dn4, locals.var_q2d_chap_dn6, locals.var_q2d_chap_dn7, locals.var_q2d_chap_dn8, locals.var_q2d_chap_dn9,)
    }
};
        locals.var_q2d_chap = assign22420_e22676;
        locals.var_q2d_chap_dn4 = assign22420_e22676_d_n4;
        locals.var_q2d_chap_dn6 = assign22420_e22676_d_n6;
        locals.var_q2d_chap_dn7 = assign22420_e22676_d_n7;
        locals.var_q2d_chap_dn8 = assign22420_e22676_d_n8;
        locals.var_q2d_chap_dn9 = assign22420_e22676_d_n9;
        locals.var_q2d_chap_rv = 0.0;

        let (assign22430_e22686, assign22430_e22686_d_n4, assign22430_e22686_d_n6, assign22430_e22686_d_n7, assign22430_e22686_d_n8, assign22430_e22686_d_n9,) = {
    if ((locals.var_guard682 != 0.0) && (locals.var_guard683 != 0.0)) {
        let assign22430_e22682: f64 = (locals.var_q2s_chap - locals.var_q2d_chap);
        let assign22430_e22684: f64 = (assign22430_e22682 / locals.var_norm_ids);
        (assign22430_e22684, ((((locals.var_q2s_chap_dn4 - locals.var_q2d_chap_dn4) * locals.var_norm_ids) - (assign22430_e22682 * locals.var_norm_ids_dn4)) / (locals.var_norm_ids * locals.var_norm_ids)), ((((locals.var_q2s_chap_dn6 - locals.var_q2d_chap_dn6) * locals.var_norm_ids) - (assign22430_e22682 * locals.var_norm_ids_dn6)) / (locals.var_norm_ids * locals.var_norm_ids)), ((((locals.var_q2s_chap_dn7 - locals.var_q2d_chap_dn7) * locals.var_norm_ids) - (assign22430_e22682 * locals.var_norm_ids_dn7)) / (locals.var_norm_ids * locals.var_norm_ids)), ((((locals.var_q2s_chap_dn8 - locals.var_q2d_chap_dn8) * locals.var_norm_ids) - (assign22430_e22682 * locals.var_norm_ids_dn8)) / (locals.var_norm_ids * locals.var_norm_ids)), ((((locals.var_q2s_chap_dn9 - locals.var_q2d_chap_dn9) * locals.var_norm_ids) - (assign22430_e22682 * locals.var_norm_ids_dn9)) / (locals.var_norm_ids * locals.var_norm_ids)),)
    } else {
        (locals.var_inv_k2h2_0, locals.var_inv_k2h2_0_dn4, locals.var_inv_k2h2_0_dn6, locals.var_inv_k2h2_0_dn7, locals.var_inv_k2h2_0_dn8, locals.var_inv_k2h2_0_dn9,)
    }
};
        locals.var_inv_k2h2_0 = assign22430_e22686;
        locals.var_inv_k2h2_0_dn4 = assign22430_e22686_d_n4;
        locals.var_inv_k2h2_0_dn6 = assign22430_e22686_d_n6;
        locals.var_inv_k2h2_0_dn7 = assign22430_e22686_d_n7;
        locals.var_inv_k2h2_0_dn8 = assign22430_e22686_d_n8;
        locals.var_inv_k2h2_0_dn9 = assign22430_e22686_d_n9;
        locals.var_inv_k2h2_0_rv = 0.0;

        let (assign22440_e22693, assign22440_e22693_d_n4, assign22440_e22693_d_n6, assign22440_e22693_d_n7, assign22440_e22693_d_n8, assign22440_e22693_d_n9,) = {
    if ((locals.var_guard682 != 0.0) && (locals.var_guard683 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_inv_k1h1_0, locals.var_inv_k1h1_0_dn4, locals.var_inv_k1h1_0_dn6, locals.var_inv_k1h1_0_dn7, locals.var_inv_k1h1_0_dn8, locals.var_inv_k1h1_0_dn9,)
    }
};
        locals.var_inv_k1h1_0 = assign22440_e22693;
        locals.var_inv_k1h1_0_dn4 = assign22440_e22693_d_n4;
        locals.var_inv_k1h1_0_dn6 = assign22440_e22693_d_n6;
        locals.var_inv_k1h1_0_dn7 = assign22440_e22693_d_n7;
        locals.var_inv_k1h1_0_dn8 = assign22440_e22693_d_n8;
        locals.var_inv_k1h1_0_dn9 = assign22440_e22693_d_n9;
        locals.var_inv_k1h1_0_rv = 0.0;

        let (assign22450_e22700, assign22450_e22700_d_n4, assign22450_e22700_d_n6, assign22450_e22700_d_n7, assign22450_e22700_d_n8, assign22450_e22700_d_n9,) = {
    if ((locals.var_guard682 != 0.0) && (locals.var_guard683 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_inv_k2h2_0, locals.var_inv_k2h2_0_dn4, locals.var_inv_k2h2_0_dn6, locals.var_inv_k2h2_0_dn7, locals.var_inv_k2h2_0_dn8, locals.var_inv_k2h2_0_dn9,)
    }
};
        locals.var_inv_k2h2_0 = assign22450_e22700;
        locals.var_inv_k2h2_0_dn4 = assign22450_e22700_d_n4;
        locals.var_inv_k2h2_0_dn6 = assign22450_e22700_d_n6;
        locals.var_inv_k2h2_0_dn7 = assign22450_e22700_d_n7;
        locals.var_inv_k2h2_0_dn8 = assign22450_e22700_d_n8;
        locals.var_inv_k2h2_0_dn9 = assign22450_e22700_d_n9;
        locals.var_inv_k2h2_0_rv = 0.0;

        let (assign22460_e22714, assign22460_e22714_d_n4, assign22460_e22714_d_n6, assign22460_e22714_d_n7, assign22460_e22714_d_n8, assign22460_e22714_d_n9,) = {
    if (locals.var_guard682 == 0.0) {
        let assign22460_e22704: f64 = (-2.0);
        let assign22460_e22706: f64 = (assign22460_e22704 * locals.var_s1);
        let assign22460_e22709: f64 = (locals.var_inv_k1 / locals.var_q1chapinf);
        let assign22460_e22711: f64 = (assign22460_e22709 + locals.var_inv_dinf);
        let assign22460_e22712: f64 = (assign22460_e22706 * assign22460_e22711);
        (assign22460_e22712, (((assign22460_e22704 * locals.var_s1_dn4) * assign22460_e22711) + (assign22460_e22706 * ((((locals.var_inv_k1_dn4 * locals.var_q1chapinf) - (locals.var_inv_k1 * locals.var_q1chapinf_dn4)) / (locals.var_q1chapinf * locals.var_q1chapinf)) + locals.var_inv_dinf_dn4))), (((assign22460_e22704 * locals.var_s1_dn6) * assign22460_e22711) + (assign22460_e22706 * ((((locals.var_inv_k1_dn6 * locals.var_q1chapinf) - (locals.var_inv_k1 * locals.var_q1chapinf_dn6)) / (locals.var_q1chapinf * locals.var_q1chapinf)) + locals.var_inv_dinf_dn6))), (((assign22460_e22704 * locals.var_s1_dn7) * assign22460_e22711) + (assign22460_e22706 * ((((locals.var_inv_k1_dn7 * locals.var_q1chapinf) - (locals.var_inv_k1 * locals.var_q1chapinf_dn7)) / (locals.var_q1chapinf * locals.var_q1chapinf)) + locals.var_inv_dinf_dn7))), (((assign22460_e22704 * locals.var_s1_dn8) * assign22460_e22711) + (assign22460_e22706 * ((((locals.var_inv_k1_dn8 * locals.var_q1chapinf) - (locals.var_inv_k1 * locals.var_q1chapinf_dn8)) / (locals.var_q1chapinf * locals.var_q1chapinf)) + locals.var_inv_dinf_dn8))), (((assign22460_e22704 * locals.var_s1_dn9) * assign22460_e22711) + (assign22460_e22706 * ((((locals.var_inv_k1_dn9 * locals.var_q1chapinf) - (locals.var_inv_k1 * locals.var_q1chapinf_dn9)) / (locals.var_q1chapinf * locals.var_q1chapinf)) + locals.var_inv_dinf_dn9))),)
    } else {
        (locals.var_zeta1, locals.var_zeta1_dn4, locals.var_zeta1_dn6, locals.var_zeta1_dn7, locals.var_zeta1_dn8, locals.var_zeta1_dn9,)
    }
};
        locals.var_zeta1 = assign22460_e22714;
        locals.var_zeta1_dn4 = assign22460_e22714_d_n4;
        locals.var_zeta1_dn6 = assign22460_e22714_d_n6;
        locals.var_zeta1_dn7 = assign22460_e22714_d_n7;
        locals.var_zeta1_dn8 = assign22460_e22714_d_n8;
        locals.var_zeta1_dn9 = assign22460_e22714_d_n9;
        locals.var_zeta1_rv = 0.0;

        let (assign22470_e22728, assign22470_e22728_d_n4, assign22470_e22728_d_n6, assign22470_e22728_d_n7, assign22470_e22728_d_n8, assign22470_e22728_d_n9,) = {
    if (locals.var_guard682 == 0.0) {
        let assign22470_e22718: f64 = (-2.0);
        let assign22470_e22720: f64 = (assign22470_e22718 * locals.var_s2);
        let assign22470_e22723: f64 = (locals.var_inv_k2 / locals.var_q2chapinf);
        let assign22470_e22725: f64 = (assign22470_e22723 + locals.var_inv_dinf);
        let assign22470_e22726: f64 = (assign22470_e22720 * assign22470_e22725);
        (assign22470_e22726, (((assign22470_e22718 * locals.var_s2_dn4) * assign22470_e22725) + (assign22470_e22720 * ((((locals.var_inv_k2_dn4 * locals.var_q2chapinf) - (locals.var_inv_k2 * locals.var_q2chapinf_dn4)) / (locals.var_q2chapinf * locals.var_q2chapinf)) + locals.var_inv_dinf_dn4))), (((assign22470_e22718 * locals.var_s2_dn6) * assign22470_e22725) + (assign22470_e22720 * ((((locals.var_inv_k2_dn6 * locals.var_q2chapinf) - (locals.var_inv_k2 * locals.var_q2chapinf_dn6)) / (locals.var_q2chapinf * locals.var_q2chapinf)) + locals.var_inv_dinf_dn6))), (((assign22470_e22718 * locals.var_s2_dn7) * assign22470_e22725) + (assign22470_e22720 * ((((locals.var_inv_k2_dn7 * locals.var_q2chapinf) - (locals.var_inv_k2 * locals.var_q2chapinf_dn7)) / (locals.var_q2chapinf * locals.var_q2chapinf)) + locals.var_inv_dinf_dn7))), (((assign22470_e22718 * locals.var_s2_dn8) * assign22470_e22725) + (assign22470_e22720 * ((((locals.var_inv_k2_dn8 * locals.var_q2chapinf) - (locals.var_inv_k2 * locals.var_q2chapinf_dn8)) / (locals.var_q2chapinf * locals.var_q2chapinf)) + locals.var_inv_dinf_dn8))), (((assign22470_e22718 * locals.var_s2_dn9) * assign22470_e22725) + (assign22470_e22720 * ((((locals.var_inv_k2_dn9 * locals.var_q2chapinf) - (locals.var_inv_k2 * locals.var_q2chapinf_dn9)) / (locals.var_q2chapinf * locals.var_q2chapinf)) + locals.var_inv_dinf_dn9))),)
    } else {
        (locals.var_zeta2, locals.var_zeta2_dn4, locals.var_zeta2_dn6, locals.var_zeta2_dn7, locals.var_zeta2_dn8, locals.var_zeta2_dn9,)
    }
};
        locals.var_zeta2 = assign22470_e22728;
        locals.var_zeta2_dn4 = assign22470_e22728_d_n4;
        locals.var_zeta2_dn6 = assign22470_e22728_d_n6;
        locals.var_zeta2_dn7 = assign22470_e22728_d_n7;
        locals.var_zeta2_dn8 = assign22470_e22728_d_n8;
        locals.var_zeta2_dn9 = assign22470_e22728_d_n9;
        locals.var_zeta2_rv = 0.0;

        let (assign22480_e22737, assign22480_e22737_d_n4, assign22480_e22737_d_n6, assign22480_e22737_d_n7, assign22480_e22737_d_n8, assign22480_e22737_d_n9,) = {
    if (locals.var_guard682 == 0.0) {
        let assign22480_e22733: f64 = (locals.var_zeta2 - locals.var_zeta1);
        let assign22480_e22735: f64 = (assign22480_e22733 * locals.var_inv_dinf);
        (assign22480_e22735, (((locals.var_zeta2_dn4 - locals.var_zeta1_dn4) * locals.var_inv_dinf) + (assign22480_e22733 * locals.var_inv_dinf_dn4)), (((locals.var_zeta2_dn6 - locals.var_zeta1_dn6) * locals.var_inv_dinf) + (assign22480_e22733 * locals.var_inv_dinf_dn6)), (((locals.var_zeta2_dn7 - locals.var_zeta1_dn7) * locals.var_inv_dinf) + (assign22480_e22733 * locals.var_inv_dinf_dn7)), (((locals.var_zeta2_dn8 - locals.var_zeta1_dn8) * locals.var_inv_dinf) + (assign22480_e22733 * locals.var_inv_dinf_dn8)), (((locals.var_zeta2_dn9 - locals.var_zeta1_dn9) * locals.var_inv_dinf) + (assign22480_e22733 * locals.var_inv_dinf_dn9)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign22480_e22737;
        locals.var_temp_dn4 = assign22480_e22737_d_n4;
        locals.var_temp_dn6 = assign22480_e22737_d_n6;
        locals.var_temp_dn7 = assign22480_e22737_d_n7;
        locals.var_temp_dn8 = assign22480_e22737_d_n8;
        locals.var_temp_dn9 = assign22480_e22737_d_n9;
        locals.var_temp_rv = 0.0;

        let (assign22490_e22744, assign22490_e22744_d_n4, assign22490_e22744_d_n6, assign22490_e22744_d_n7, assign22490_e22744_d_n8, assign22490_e22744_d_n9,) = {
    if (locals.var_guard682 == 0.0) {
        let assign22490_e22742: f64 = (locals.var_zeta1 * locals.var_inv_k1);
        (assign22490_e22742, ((locals.var_zeta1_dn4 * locals.var_inv_k1) + (locals.var_zeta1 * locals.var_inv_k1_dn4)), ((locals.var_zeta1_dn6 * locals.var_inv_k1) + (locals.var_zeta1 * locals.var_inv_k1_dn6)), ((locals.var_zeta1_dn7 * locals.var_inv_k1) + (locals.var_zeta1 * locals.var_inv_k1_dn7)), ((locals.var_zeta1_dn8 * locals.var_inv_k1) + (locals.var_zeta1 * locals.var_inv_k1_dn8)), ((locals.var_zeta1_dn9 * locals.var_inv_k1) + (locals.var_zeta1 * locals.var_inv_k1_dn9)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign22490_e22744;
        locals.var_temp1_dn4 = assign22490_e22744_d_n4;
        locals.var_temp1_dn6 = assign22490_e22744_d_n6;
        locals.var_temp1_dn7 = assign22490_e22744_d_n7;
        locals.var_temp1_dn8 = assign22490_e22744_d_n8;
        locals.var_temp1_dn9 = assign22490_e22744_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign22500_e22751, assign22500_e22751_d_n4, assign22500_e22751_d_n6, assign22500_e22751_d_n7, assign22500_e22751_d_n8, assign22500_e22751_d_n9,) = {
    if (locals.var_guard682 == 0.0) {
        let assign22500_e22749: f64 = (locals.var_zeta2 * locals.var_inv_k2);
        (assign22500_e22749, ((locals.var_zeta2_dn4 * locals.var_inv_k2) + (locals.var_zeta2 * locals.var_inv_k2_dn4)), ((locals.var_zeta2_dn6 * locals.var_inv_k2) + (locals.var_zeta2 * locals.var_inv_k2_dn6)), ((locals.var_zeta2_dn7 * locals.var_inv_k2) + (locals.var_zeta2 * locals.var_inv_k2_dn7)), ((locals.var_zeta2_dn8 * locals.var_inv_k2) + (locals.var_zeta2 * locals.var_inv_k2_dn8)), ((locals.var_zeta2_dn9 * locals.var_inv_k2) + (locals.var_zeta2 * locals.var_inv_k2_dn9)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign22500_e22751;
        locals.var_temp2_dn4 = assign22500_e22751_d_n4;
        locals.var_temp2_dn6 = assign22500_e22751_d_n6;
        locals.var_temp2_dn7 = assign22500_e22751_d_n7;
        locals.var_temp2_dn8 = assign22500_e22751_d_n8;
        locals.var_temp2_dn9 = assign22500_e22751_d_n9;
        locals.var_temp2_rv = 0.0;

        let (assign22510_e22758, assign22510_e22758_d_n4, assign22510_e22758_d_n6, assign22510_e22758_d_n7, assign22510_e22758_d_n8, assign22510_e22758_d_n9,) = {
    if (locals.var_guard682 == 0.0) {
        let assign22510_e22756: f64 = (locals.var_temp1 + locals.var_temp2);
        (assign22510_e22756, (locals.var_temp1_dn4 + locals.var_temp2_dn4), (locals.var_temp1_dn6 + locals.var_temp2_dn6), (locals.var_temp1_dn7 + locals.var_temp2_dn7), (locals.var_temp1_dn8 + locals.var_temp2_dn8), (locals.var_temp1_dn9 + locals.var_temp2_dn9),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign22510_e22758;
        locals.var_temp3_dn4 = assign22510_e22758_d_n4;
        locals.var_temp3_dn6 = assign22510_e22758_d_n6;
        locals.var_temp3_dn7 = assign22510_e22758_d_n7;
        locals.var_temp3_dn8 = assign22510_e22758_d_n8;
        locals.var_temp3_dn9 = assign22510_e22758_d_n9;
        locals.var_temp3_rv = 0.0;

        let (assign22520_e22773, assign22520_e22773_d_n4, assign22520_e22773_d_n6, assign22520_e22773_d_n7, assign22520_e22773_d_n8, assign22520_e22773_d_n9,) = {
    if (locals.var_guard682 == 0.0) {
        let assign22520_e22765: f64 = (locals.var_s1 * locals.var_inv_k1);
        let assign22520_e22768: f64 = (locals.var_s2 * locals.var_inv_k2);
        let assign22520_e22769: f64 = (assign22520_e22765 + assign22520_e22768);
        let assign22520_e22770: f64 = (2.0 * assign22520_e22769);
        let assign22520_e22771: f64 = (3.0 + assign22520_e22770);
        (assign22520_e22771, (2.0 * (((locals.var_s1_dn4 * locals.var_inv_k1) + (locals.var_s1 * locals.var_inv_k1_dn4)) + ((locals.var_s2_dn4 * locals.var_inv_k2) + (locals.var_s2 * locals.var_inv_k2_dn4)))), (2.0 * (((locals.var_s1_dn6 * locals.var_inv_k1) + (locals.var_s1 * locals.var_inv_k1_dn6)) + ((locals.var_s2_dn6 * locals.var_inv_k2) + (locals.var_s2 * locals.var_inv_k2_dn6)))), (2.0 * (((locals.var_s1_dn7 * locals.var_inv_k1) + (locals.var_s1 * locals.var_inv_k1_dn7)) + ((locals.var_s2_dn7 * locals.var_inv_k2) + (locals.var_s2 * locals.var_inv_k2_dn7)))), (2.0 * (((locals.var_s1_dn8 * locals.var_inv_k1) + (locals.var_s1 * locals.var_inv_k1_dn8)) + ((locals.var_s2_dn8 * locals.var_inv_k2) + (locals.var_s2 * locals.var_inv_k2_dn8)))), (2.0 * (((locals.var_s1_dn9 * locals.var_inv_k1) + (locals.var_s1 * locals.var_inv_k1_dn9)) + ((locals.var_s2_dn9 * locals.var_inv_k2) + (locals.var_s2 * locals.var_inv_k2_dn9)))),)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
        locals.var_temp4 = assign22520_e22773;
        locals.var_temp4_dn4 = assign22520_e22773_d_n4;
        locals.var_temp4_dn6 = assign22520_e22773_d_n6;
        locals.var_temp4_dn7 = assign22520_e22773_d_n7;
        locals.var_temp4_dn8 = assign22520_e22773_d_n8;
        locals.var_temp4_dn9 = assign22520_e22773_d_n9;
        locals.var_temp4_rv = 0.0;

        let (assign22530_e22786, assign22530_e22786_d_n4, assign22530_e22786_d_n6, assign22530_e22786_d_n7, assign22530_e22786_d_n8, assign22530_e22786_d_n9,) = {
    if (locals.var_guard682 == 0.0) {
        let assign22530_e22778: f64 = (locals.var_temp2 + locals.var_temp);
        let assign22530_e22781: f64 = (locals.var_temp3 / locals.var_q1chapinf);
        let assign22530_e22782: f64 = (assign22530_e22778 - assign22530_e22781);
        let assign22530_e22784: f64 = (assign22530_e22782 / locals.var_temp4);
        (assign22530_e22784, (((((locals.var_temp2_dn4 + locals.var_temp_dn4) - (((locals.var_temp3_dn4 * locals.var_q1chapinf) - (locals.var_temp3 * locals.var_q1chapinf_dn4)) / (locals.var_q1chapinf * locals.var_q1chapinf))) * locals.var_temp4) - (assign22530_e22782 * locals.var_temp4_dn4)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp2_dn6 + locals.var_temp_dn6) - (((locals.var_temp3_dn6 * locals.var_q1chapinf) - (locals.var_temp3 * locals.var_q1chapinf_dn6)) / (locals.var_q1chapinf * locals.var_q1chapinf))) * locals.var_temp4) - (assign22530_e22782 * locals.var_temp4_dn6)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp2_dn7 + locals.var_temp_dn7) - (((locals.var_temp3_dn7 * locals.var_q1chapinf) - (locals.var_temp3 * locals.var_q1chapinf_dn7)) / (locals.var_q1chapinf * locals.var_q1chapinf))) * locals.var_temp4) - (assign22530_e22782 * locals.var_temp4_dn7)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp2_dn8 + locals.var_temp_dn8) - (((locals.var_temp3_dn8 * locals.var_q1chapinf) - (locals.var_temp3 * locals.var_q1chapinf_dn8)) / (locals.var_q1chapinf * locals.var_q1chapinf))) * locals.var_temp4) - (assign22530_e22782 * locals.var_temp4_dn8)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp2_dn9 + locals.var_temp_dn9) - (((locals.var_temp3_dn9 * locals.var_q1chapinf) - (locals.var_temp3 * locals.var_q1chapinf_dn9)) / (locals.var_q1chapinf * locals.var_q1chapinf))) * locals.var_temp4) - (assign22530_e22782 * locals.var_temp4_dn9)) / (locals.var_temp4 * locals.var_temp4)),)
    } else {
        (locals.var_ksi1, locals.var_ksi1_dn4, locals.var_ksi1_dn6, locals.var_ksi1_dn7, locals.var_ksi1_dn8, locals.var_ksi1_dn9,)
    }
};
        locals.var_ksi1 = assign22530_e22786;
        locals.var_ksi1_dn4 = assign22530_e22786_d_n4;
        locals.var_ksi1_dn6 = assign22530_e22786_d_n6;
        locals.var_ksi1_dn7 = assign22530_e22786_d_n7;
        locals.var_ksi1_dn8 = assign22530_e22786_d_n8;
        locals.var_ksi1_dn9 = assign22530_e22786_d_n9;
        locals.var_ksi1_rv = 0.0;

        let (assign22540_e22799, assign22540_e22799_d_n4, assign22540_e22799_d_n6, assign22540_e22799_d_n7, assign22540_e22799_d_n8, assign22540_e22799_d_n9,) = {
    if (locals.var_guard682 == 0.0) {
        let assign22540_e22791: f64 = (locals.var_temp1 - locals.var_temp);
        let assign22540_e22794: f64 = (locals.var_temp3 / locals.var_q2chapinf);
        let assign22540_e22795: f64 = (assign22540_e22791 - assign22540_e22794);
        let assign22540_e22797: f64 = (assign22540_e22795 / locals.var_temp4);
        (assign22540_e22797, (((((locals.var_temp1_dn4 - locals.var_temp_dn4) - (((locals.var_temp3_dn4 * locals.var_q2chapinf) - (locals.var_temp3 * locals.var_q2chapinf_dn4)) / (locals.var_q2chapinf * locals.var_q2chapinf))) * locals.var_temp4) - (assign22540_e22795 * locals.var_temp4_dn4)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp1_dn6 - locals.var_temp_dn6) - (((locals.var_temp3_dn6 * locals.var_q2chapinf) - (locals.var_temp3 * locals.var_q2chapinf_dn6)) / (locals.var_q2chapinf * locals.var_q2chapinf))) * locals.var_temp4) - (assign22540_e22795 * locals.var_temp4_dn6)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp1_dn7 - locals.var_temp_dn7) - (((locals.var_temp3_dn7 * locals.var_q2chapinf) - (locals.var_temp3 * locals.var_q2chapinf_dn7)) / (locals.var_q2chapinf * locals.var_q2chapinf))) * locals.var_temp4) - (assign22540_e22795 * locals.var_temp4_dn7)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp1_dn8 - locals.var_temp_dn8) - (((locals.var_temp3_dn8 * locals.var_q2chapinf) - (locals.var_temp3 * locals.var_q2chapinf_dn8)) / (locals.var_q2chapinf * locals.var_q2chapinf))) * locals.var_temp4) - (assign22540_e22795 * locals.var_temp4_dn8)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp1_dn9 - locals.var_temp_dn9) - (((locals.var_temp3_dn9 * locals.var_q2chapinf) - (locals.var_temp3 * locals.var_q2chapinf_dn9)) / (locals.var_q2chapinf * locals.var_q2chapinf))) * locals.var_temp4) - (assign22540_e22795 * locals.var_temp4_dn9)) / (locals.var_temp4 * locals.var_temp4)),)
    } else {
        (locals.var_ksi2, locals.var_ksi2_dn4, locals.var_ksi2_dn6, locals.var_ksi2_dn7, locals.var_ksi2_dn8, locals.var_ksi2_dn9,)
    }
};
        locals.var_ksi2 = assign22540_e22799;
        locals.var_ksi2_dn4 = assign22540_e22799_d_n4;
        locals.var_ksi2_dn6 = assign22540_e22799_d_n6;
        locals.var_ksi2_dn7 = assign22540_e22799_d_n7;
        locals.var_ksi2_dn8 = assign22540_e22799_d_n8;
        locals.var_ksi2_dn9 = assign22540_e22799_d_n9;
        locals.var_ksi2_rv = 0.0;

        let (assign22550_e22811, assign22550_e22811_d_n4, assign22550_e22811_d_n6, assign22550_e22811_d_n7, assign22550_e22811_d_n8, assign22550_e22811_d_n9,) = {
    if (locals.var_guard682 == 0.0) {
        let assign22550_e22803: f64 = (-locals.var_q1chapinf);
        let assign22550_e22806: f64 = (locals.var_ksi1 * locals.var_q1chapinf);
        let assign22550_e22808: f64 = (assign22550_e22806 + locals.var_inv_dinf);
        let assign22550_e22809: f64 = (assign22550_e22803 * assign22550_e22808);
        (assign22550_e22809, (((-locals.var_q1chapinf_dn4) * assign22550_e22808) + (assign22550_e22803 * (((locals.var_ksi1_dn4 * locals.var_q1chapinf) + (locals.var_ksi1 * locals.var_q1chapinf_dn4)) + locals.var_inv_dinf_dn4))), (((-locals.var_q1chapinf_dn6) * assign22550_e22808) + (assign22550_e22803 * (((locals.var_ksi1_dn6 * locals.var_q1chapinf) + (locals.var_ksi1 * locals.var_q1chapinf_dn6)) + locals.var_inv_dinf_dn6))), (((-locals.var_q1chapinf_dn7) * assign22550_e22808) + (assign22550_e22803 * (((locals.var_ksi1_dn7 * locals.var_q1chapinf) + (locals.var_ksi1 * locals.var_q1chapinf_dn7)) + locals.var_inv_dinf_dn7))), (((-locals.var_q1chapinf_dn8) * assign22550_e22808) + (assign22550_e22803 * (((locals.var_ksi1_dn8 * locals.var_q1chapinf) + (locals.var_ksi1 * locals.var_q1chapinf_dn8)) + locals.var_inv_dinf_dn8))), (((-locals.var_q1chapinf_dn9) * assign22550_e22808) + (assign22550_e22803 * (((locals.var_ksi1_dn9 * locals.var_q1chapinf) + (locals.var_ksi1 * locals.var_q1chapinf_dn9)) + locals.var_inv_dinf_dn9))),)
    } else {
        (locals.var_inv_k1h1_0, locals.var_inv_k1h1_0_dn4, locals.var_inv_k1h1_0_dn6, locals.var_inv_k1h1_0_dn7, locals.var_inv_k1h1_0_dn8, locals.var_inv_k1h1_0_dn9,)
    }
};
        locals.var_inv_k1h1_0 = assign22550_e22811;
        locals.var_inv_k1h1_0_dn4 = assign22550_e22811_d_n4;
        locals.var_inv_k1h1_0_dn6 = assign22550_e22811_d_n6;
        locals.var_inv_k1h1_0_dn7 = assign22550_e22811_d_n7;
        locals.var_inv_k1h1_0_dn8 = assign22550_e22811_d_n8;
        locals.var_inv_k1h1_0_dn9 = assign22550_e22811_d_n9;
        locals.var_inv_k1h1_0_rv = 0.0;

        let (assign22560_e22823, assign22560_e22823_d_n4, assign22560_e22823_d_n6, assign22560_e22823_d_n7, assign22560_e22823_d_n8, assign22560_e22823_d_n9,) = {
    if (locals.var_guard682 == 0.0) {
        let assign22560_e22815: f64 = (-locals.var_q2chapinf);
        let assign22560_e22818: f64 = (locals.var_ksi2 * locals.var_q2chapinf);
        let assign22560_e22820: f64 = (assign22560_e22818 + locals.var_inv_dinf);
        let assign22560_e22821: f64 = (assign22560_e22815 * assign22560_e22820);
        (assign22560_e22821, (((-locals.var_q2chapinf_dn4) * assign22560_e22820) + (assign22560_e22815 * (((locals.var_ksi2_dn4 * locals.var_q2chapinf) + (locals.var_ksi2 * locals.var_q2chapinf_dn4)) + locals.var_inv_dinf_dn4))), (((-locals.var_q2chapinf_dn6) * assign22560_e22820) + (assign22560_e22815 * (((locals.var_ksi2_dn6 * locals.var_q2chapinf) + (locals.var_ksi2 * locals.var_q2chapinf_dn6)) + locals.var_inv_dinf_dn6))), (((-locals.var_q2chapinf_dn7) * assign22560_e22820) + (assign22560_e22815 * (((locals.var_ksi2_dn7 * locals.var_q2chapinf) + (locals.var_ksi2 * locals.var_q2chapinf_dn7)) + locals.var_inv_dinf_dn7))), (((-locals.var_q2chapinf_dn8) * assign22560_e22820) + (assign22560_e22815 * (((locals.var_ksi2_dn8 * locals.var_q2chapinf) + (locals.var_ksi2 * locals.var_q2chapinf_dn8)) + locals.var_inv_dinf_dn8))), (((-locals.var_q2chapinf_dn9) * assign22560_e22820) + (assign22560_e22815 * (((locals.var_ksi2_dn9 * locals.var_q2chapinf) + (locals.var_ksi2 * locals.var_q2chapinf_dn9)) + locals.var_inv_dinf_dn9))),)
    } else {
        (locals.var_inv_k2h2_0, locals.var_inv_k2h2_0_dn4, locals.var_inv_k2h2_0_dn6, locals.var_inv_k2h2_0_dn7, locals.var_inv_k2h2_0_dn8, locals.var_inv_k2h2_0_dn9,)
    }
};
        locals.var_inv_k2h2_0 = assign22560_e22823;
        locals.var_inv_k2h2_0_dn4 = assign22560_e22823_d_n4;
        locals.var_inv_k2h2_0_dn6 = assign22560_e22823_d_n6;
        locals.var_inv_k2h2_0_dn7 = assign22560_e22823_d_n7;
        locals.var_inv_k2h2_0_dn8 = assign22560_e22823_d_n8;
        locals.var_inv_k2h2_0_dn9 = assign22560_e22823_d_n9;
        locals.var_inv_k2h2_0_rv = 0.0;

        let assign22570_e22826: f64 = (locals.var_inv_k1h1_0 * locals.var_hsat);
        locals.var_inv_k1h1 = assign22570_e22826;
        locals.var_inv_k1h1_dn4 = ((locals.var_inv_k1h1_0_dn4 * locals.var_hsat) + (locals.var_inv_k1h1_0 * locals.var_hsat_dn4));
        locals.var_inv_k1h1_dn6 = ((locals.var_inv_k1h1_0_dn6 * locals.var_hsat) + (locals.var_inv_k1h1_0 * locals.var_hsat_dn6));
        locals.var_inv_k1h1_dn7 = ((locals.var_inv_k1h1_0_dn7 * locals.var_hsat) + (locals.var_inv_k1h1_0 * locals.var_hsat_dn7));
        locals.var_inv_k1h1_dn8 = ((locals.var_inv_k1h1_0_dn8 * locals.var_hsat) + (locals.var_inv_k1h1_0 * locals.var_hsat_dn8));
        locals.var_inv_k1h1_dn9 = ((locals.var_inv_k1h1_0_dn9 * locals.var_hsat) + (locals.var_inv_k1h1_0 * locals.var_hsat_dn9));
        locals.var_inv_k1h1_rv = 0.0;

        let assign22580_e22829: f64 = (locals.var_inv_k2h2_0 * locals.var_hsat);
        locals.var_inv_k2h2 = assign22580_e22829;
        locals.var_inv_k2h2_dn4 = ((locals.var_inv_k2h2_0_dn4 * locals.var_hsat) + (locals.var_inv_k2h2_0 * locals.var_hsat_dn4));
        locals.var_inv_k2h2_dn6 = ((locals.var_inv_k2h2_0_dn6 * locals.var_hsat) + (locals.var_inv_k2h2_0 * locals.var_hsat_dn6));
        locals.var_inv_k2h2_dn7 = ((locals.var_inv_k2h2_0_dn7 * locals.var_hsat) + (locals.var_inv_k2h2_0 * locals.var_hsat_dn7));
        locals.var_inv_k2h2_dn8 = ((locals.var_inv_k2h2_0_dn8 * locals.var_hsat) + (locals.var_inv_k2h2_0 * locals.var_hsat_dn8));
        locals.var_inv_k2h2_dn9 = ((locals.var_inv_k2h2_0_dn9 * locals.var_hsat) + (locals.var_inv_k2h2_0 * locals.var_hsat_dn9));
        locals.var_inv_k2h2_rv = 0.0;

        let assign22590_e22833: f64 = (locals.var_k1q1d - locals.var_k1q1s);
        let assign22590_e22834: f64 = (0.5 * assign22590_e22833);
        locals.var_delta_k1q1 = assign22590_e22834;
        locals.var_delta_k1q1_dn4 = (0.5 * (locals.var_k1q1d_dn4 - locals.var_k1q1s_dn4));
        locals.var_delta_k1q1_dn6 = (0.5 * (locals.var_k1q1d_dn6 - locals.var_k1q1s_dn6));
        locals.var_delta_k1q1_dn7 = (0.5 * (locals.var_k1q1d_dn7 - locals.var_k1q1s_dn7));
        locals.var_delta_k1q1_dn8 = (0.5 * (locals.var_k1q1d_dn8 - locals.var_k1q1s_dn8));
        locals.var_delta_k1q1_dn9 = (0.5 * (locals.var_k1q1d_dn9 - locals.var_k1q1s_dn9));
        locals.var_delta_k1q1_rv = 0.0;

        let assign22600_e22838: f64 = (locals.var_k2q2d - locals.var_k2q2s);
        let assign22600_e22839: f64 = (0.5 * assign22600_e22838);
        locals.var_delta_k2q2 = assign22600_e22839;
        locals.var_delta_k2q2_dn4 = (0.5 * (locals.var_k2q2d_dn4 - locals.var_k2q2s_dn4));
        locals.var_delta_k2q2_dn6 = (0.5 * (locals.var_k2q2d_dn6 - locals.var_k2q2s_dn6));
        locals.var_delta_k2q2_dn7 = (0.5 * (locals.var_k2q2d_dn7 - locals.var_k2q2s_dn7));
        locals.var_delta_k2q2_dn8 = (0.5 * (locals.var_k2q2d_dn8 - locals.var_k2q2s_dn8));
        locals.var_delta_k2q2_dn9 = (0.5 * (locals.var_k2q2d_dn9 - locals.var_k2q2s_dn9));
        locals.var_delta_k2q2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_62(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign22610_e22842: f64 = (locals.var_delta_k1q1 * locals.var_inv_k1h1);
        locals.var_prod1 = assign22610_e22842;
        locals.var_prod1_dn4 = ((locals.var_delta_k1q1_dn4 * locals.var_inv_k1h1) + (locals.var_delta_k1q1 * locals.var_inv_k1h1_dn4));
        locals.var_prod1_dn6 = ((locals.var_delta_k1q1_dn6 * locals.var_inv_k1h1) + (locals.var_delta_k1q1 * locals.var_inv_k1h1_dn6));
        locals.var_prod1_dn7 = ((locals.var_delta_k1q1_dn7 * locals.var_inv_k1h1) + (locals.var_delta_k1q1 * locals.var_inv_k1h1_dn7));
        locals.var_prod1_dn8 = ((locals.var_delta_k1q1_dn8 * locals.var_inv_k1h1) + (locals.var_delta_k1q1 * locals.var_inv_k1h1_dn8));
        locals.var_prod1_dn9 = ((locals.var_delta_k1q1_dn9 * locals.var_inv_k1h1) + (locals.var_delta_k1q1 * locals.var_inv_k1h1_dn9));
        locals.var_prod1_rv = 0.0;

        let assign22620_e22845: f64 = (locals.var_delta_k2q2 * locals.var_inv_k2h2);
        locals.var_prod2 = assign22620_e22845;
        locals.var_prod2_dn4 = ((locals.var_delta_k2q2_dn4 * locals.var_inv_k2h2) + (locals.var_delta_k2q2 * locals.var_inv_k2h2_dn4));
        locals.var_prod2_dn6 = ((locals.var_delta_k2q2_dn6 * locals.var_inv_k2h2) + (locals.var_delta_k2q2 * locals.var_inv_k2h2_dn6));
        locals.var_prod2_dn7 = ((locals.var_delta_k2q2_dn7 * locals.var_inv_k2h2) + (locals.var_delta_k2q2 * locals.var_inv_k2h2_dn7));
        locals.var_prod2_dn8 = ((locals.var_delta_k2q2_dn8 * locals.var_inv_k2h2) + (locals.var_delta_k2q2 * locals.var_inv_k2h2_dn8));
        locals.var_prod2_dn9 = ((locals.var_delta_k2q2_dn9 * locals.var_inv_k2h2) + (locals.var_delta_k2q2 * locals.var_inv_k2h2_dn9));
        locals.var_prod2_rv = 0.0;

        locals.var_xg20shift_dc = locals.var_xg20shift;
        locals.var_xg20shift_dc_dn4 = locals.var_xg20shift_dn4;
        locals.var_xg20shift_dc_dn6 = locals.var_xg20shift_dn6;
        locals.var_xg20shift_dc_dn7 = locals.var_xg20shift_dn7;
        locals.var_xg20shift_dc_dn8 = locals.var_xg20shift_dn8;
        locals.var_xg20shift_dc_dn9 = locals.var_xg20shift_dn9;
        locals.var_xg20shift_dc_rv = 0.0;

        locals.var_diff_min_dc = locals.var_diff_min;
        locals.var_diff_min_dc_dn4 = locals.var_diff_min_dn4;
        locals.var_diff_min_dc_dn6 = locals.var_diff_min_dn6;
        locals.var_diff_min_dc_dn7 = locals.var_diff_min_dn7;
        locals.var_diff_min_dc_dn8 = locals.var_diff_min_dn8;
        locals.var_diff_min_dc_dn9 = locals.var_diff_min_dn9;
        locals.var_diff_min_dc_rv = 0.0;

        locals.var_a0_dc = locals.var_a0;
        locals.var_a0_dc_dn4 = locals.var_a0_dn4;
        locals.var_a0_dc_dn6 = locals.var_a0_dn6;
        locals.var_a0_dc_dn7 = locals.var_a0_dn7;
        locals.var_a0_dc_dn8 = locals.var_a0_dn8;
        locals.var_a0_dc_dn9 = locals.var_a0_dn9;
        locals.var_a0_dc_rv = 0.0;

        locals.var_inv_k1_dc = locals.var_inv_k1;
        locals.var_inv_k1_dc_dn4 = locals.var_inv_k1_dn4;
        locals.var_inv_k1_dc_dn6 = locals.var_inv_k1_dn6;
        locals.var_inv_k1_dc_dn7 = locals.var_inv_k1_dn7;
        locals.var_inv_k1_dc_dn8 = locals.var_inv_k1_dn8;
        locals.var_inv_k1_dc_dn9 = locals.var_inv_k1_dn9;
        locals.var_inv_k1_dc_rv = 0.0;

        locals.var_inv_k2_dc = locals.var_inv_k2;
        locals.var_inv_k2_dc_dn4 = locals.var_inv_k2_dn4;
        locals.var_inv_k2_dc_dn6 = locals.var_inv_k2_dn6;
        locals.var_inv_k2_dc_dn7 = locals.var_inv_k2_dn7;
        locals.var_inv_k2_dc_dn8 = locals.var_inv_k2_dn8;
        locals.var_inv_k2_dc_dn9 = locals.var_inv_k2_dn9;
        locals.var_inv_k2_dc_rv = 0.0;

        locals.var_keq_dc = locals.var_keq;
        locals.var_keq_dc_dn4 = locals.var_keq_dn4;
        locals.var_keq_dc_dn6 = locals.var_keq_dn6;
        locals.var_keq_dc_dn7 = locals.var_keq_dn7;
        locals.var_keq_dc_dn8 = locals.var_keq_dn8;
        locals.var_keq_dc_dn9 = locals.var_keq_dn9;
        locals.var_keq_dc_rv = 0.0;

        locals.var_dx_wi_dc = locals.var_dx_wi;
        locals.var_dx_wi_dc_dn4 = locals.var_dx_wi_dn4;
        locals.var_dx_wi_dc_dn6 = locals.var_dx_wi_dn6;
        locals.var_dx_wi_dc_dn7 = locals.var_dx_wi_dn7;
        locals.var_dx_wi_dc_dn8 = locals.var_dx_wi_dn8;
        locals.var_dx_wi_dc_dn9 = locals.var_dx_wi_dn9;
        locals.var_dx_wi_dc_rv = 0.0;

        locals.var_csiprime_dc = locals.var_csiprime;
        locals.var_csiprime_dc_dn4 = locals.var_csiprime_dn4;
        locals.var_csiprime_dc_dn6 = locals.var_csiprime_dn6;
        locals.var_csiprime_dc_dn7 = locals.var_csiprime_dn7;
        locals.var_csiprime_dc_dn8 = locals.var_csiprime_dn8;
        locals.var_csiprime_dc_dn9 = locals.var_csiprime_dn9;
        locals.var_csiprime_dc_rv = 0.0;

        locals.var_dx_wi_1d_dc = locals.var_dx_wi_1d;
        locals.var_dx_wi_1d_dc_dn4 = locals.var_dx_wi_1d_dn4;
        locals.var_dx_wi_1d_dc_dn6 = locals.var_dx_wi_1d_dn6;
        locals.var_dx_wi_1d_dc_dn7 = locals.var_dx_wi_1d_dn7;
        locals.var_dx_wi_1d_dc_dn8 = locals.var_dx_wi_1d_dn8;
        locals.var_dx_wi_1d_dc_dn9 = locals.var_dx_wi_1d_dn9;
        locals.var_dx_wi_1d_dc_rv = 0.0;

        locals.var_q1s_dc = locals.var_q1s;
        locals.var_q1s_dc_dn4 = locals.var_q1s_dn4;
        locals.var_q1s_dc_dn6 = locals.var_q1s_dn6;
        locals.var_q1s_dc_dn7 = locals.var_q1s_dn7;
        locals.var_q1s_dc_dn8 = locals.var_q1s_dn8;
        locals.var_q1s_dc_dn9 = locals.var_q1s_dn9;
        locals.var_q1s_dc_rv = 0.0;

        locals.var_dleff_dc = locals.var_dleff;
        locals.var_dleff_dc_dn4 = locals.var_dleff_dn4;
        locals.var_dleff_dc_dn6 = locals.var_dleff_dn6;
        locals.var_dleff_dc_dn7 = locals.var_dleff_dn7;
        locals.var_dleff_dc_dn8 = locals.var_dleff_dn8;
        locals.var_dleff_dc_dn9 = locals.var_dleff_dn9;
        locals.var_dleff_dc_rv = 0.0;

        locals.var_xedge_dc = locals.var_xedge;
        locals.var_xedge_dc_dn4 = locals.var_xedge_dn4;
        locals.var_xedge_dc_dn6 = locals.var_xedge_dn6;
        locals.var_xedge_dc_dn7 = locals.var_xedge_dn7;
        locals.var_xedge_dc_dn8 = locals.var_xedge_dn8;
        locals.var_xedge_dc_dn9 = locals.var_xedge_dn9;
        locals.var_xedge_dc_rv = 0.0;

        locals.var_sce1_dc = locals.var_sce1;
        locals.var_sce1_dc_dn4 = locals.var_sce1_dn4;
        locals.var_sce1_dc_dn6 = locals.var_sce1_dn6;
        locals.var_sce1_dc_dn7 = locals.var_sce1_dn7;
        locals.var_sce1_dc_dn8 = locals.var_sce1_dn8;
        locals.var_sce1_dc_dn9 = locals.var_sce1_dn9;
        locals.var_sce1_dc_rv = 0.0;

        locals.var_sce2_dc = locals.var_sce2;
        locals.var_sce2_dc_dn4 = locals.var_sce2_dn4;
        locals.var_sce2_dc_dn6 = locals.var_sce2_dn6;
        locals.var_sce2_dc_dn7 = locals.var_sce2_dn7;
        locals.var_sce2_dc_dn8 = locals.var_sce2_dn8;
        locals.var_sce2_dc_dn9 = locals.var_sce2_dn9;
        locals.var_sce2_dc_rv = 0.0;

        locals.var_dxg1_dibl_dc = locals.var_dxg1_dibl;
        locals.var_dxg1_dibl_dc_dn4 = locals.var_dxg1_dibl_dn4;
        locals.var_dxg1_dibl_dc_dn6 = locals.var_dxg1_dibl_dn6;
        locals.var_dxg1_dibl_dc_dn7 = locals.var_dxg1_dibl_dn7;
        locals.var_dxg1_dibl_dc_dn8 = locals.var_dxg1_dibl_dn8;
        locals.var_dxg1_dibl_dc_dn9 = locals.var_dxg1_dibl_dn9;
        locals.var_dxg1_dibl_dc_rv = 0.0;

        locals.var_xg2_dc = locals.var_xg2;
        locals.var_xg2_dc_dn4 = locals.var_xg2_dn4;
        locals.var_xg2_dc_dn6 = locals.var_xg2_dn6;
        locals.var_xg2_dc_dn7 = locals.var_xg2_dn7;
        locals.var_xg2_dc_dn8 = locals.var_xg2_dn8;
        locals.var_xg2_dc_dn9 = locals.var_xg2_dn9;
        locals.var_xg2_dc_rv = 0.0;

        locals.var_xg2x_dc = locals.var_xg2x;
        locals.var_xg2x_dc_dn4 = locals.var_xg2x_dn4;
        locals.var_xg2x_dc_dn6 = locals.var_xg2x_dn6;
        locals.var_xg2x_dc_dn7 = locals.var_xg2x_dn7;
        locals.var_xg2x_dc_dn8 = locals.var_xg2x_dn8;
        locals.var_xg2x_dc_dn9 = locals.var_xg2x_dn9;
        locals.var_xg2x_dc_rv = 0.0;

        locals.var_xg1x_dc = locals.var_xg1x;
        locals.var_xg1x_dc_dn4 = locals.var_xg1x_dn4;
        locals.var_xg1x_dc_dn6 = locals.var_xg1x_dn6;
        locals.var_xg1x_dc_dn7 = locals.var_xg1x_dn7;
        locals.var_xg1x_dc_dn8 = locals.var_xg1x_dn8;
        locals.var_xg1x_dc_dn9 = locals.var_xg1x_dn9;
        locals.var_xg1x_dc_rv = 0.0;

        locals.var_k1_dc = locals.var_k1;
        locals.var_k1_dc_dn4 = locals.var_k1_dn4;
        locals.var_k1_dc_dn6 = locals.var_k1_dn6;
        locals.var_k1_dc_dn7 = locals.var_k1_dn7;
        locals.var_k1_dc_dn8 = locals.var_k1_dn8;
        locals.var_k1_dc_dn9 = locals.var_k1_dn9;
        locals.var_k1_dc_rv = 0.0;

        locals.var_k2_dc = locals.var_k2;
        locals.var_k2_dc_dn4 = locals.var_k2_dn4;
        locals.var_k2_dc_dn6 = locals.var_k2_dn6;
        locals.var_k2_dc_dn7 = locals.var_k2_dn7;
        locals.var_k2_dc_dn8 = locals.var_k2_dn8;
        locals.var_k2_dc_dn9 = locals.var_k2_dn9;
        locals.var_k2_dc_rv = 0.0;

        locals.var_qis_dc = locals.var_qis;
        locals.var_qis_dc_dn4 = locals.var_qis_dn4;
        locals.var_qis_dc_dn6 = locals.var_qis_dn6;
        locals.var_qis_dc_dn7 = locals.var_qis_dn7;
        locals.var_qis_dc_dn8 = locals.var_qis_dn8;
        locals.var_qis_dc_dn9 = locals.var_qis_dn9;
        locals.var_qis_dc_rv = 0.0;

        locals.var_k1q1s_dc = locals.var_k1q1s;
        locals.var_k1q1s_dc_dn4 = locals.var_k1q1s_dn4;
        locals.var_k1q1s_dc_dn6 = locals.var_k1q1s_dn6;
        locals.var_k1q1s_dc_dn7 = locals.var_k1q1s_dn7;
        locals.var_k1q1s_dc_dn8 = locals.var_k1q1s_dn8;
        locals.var_k1q1s_dc_dn9 = locals.var_k1q1s_dn9;
        locals.var_k1q1s_dc_rv = 0.0;

        locals.var_k2q2s_dc = locals.var_k2q2s;
        locals.var_k2q2s_dc_dn4 = locals.var_k2q2s_dn4;
        locals.var_k2q2s_dc_dn6 = locals.var_k2q2s_dn6;
        locals.var_k2q2s_dc_dn7 = locals.var_k2q2s_dn7;
        locals.var_k2q2s_dc_dn8 = locals.var_k2q2s_dn8;
        locals.var_k2q2s_dc_dn9 = locals.var_k2q2s_dn9;
        locals.var_k2q2s_dc_rv = 0.0;

        locals.var_xdrifts_dc = locals.var_xdrifts;
        locals.var_xdrifts_dc_dn4 = locals.var_xdrifts_dn4;
        locals.var_xdrifts_dc_dn6 = locals.var_xdrifts_dn6;
        locals.var_xdrifts_dc_dn7 = locals.var_xdrifts_dn7;
        locals.var_xdrifts_dc_dn8 = locals.var_xdrifts_dn8;
        locals.var_xdrifts_dc_dn9 = locals.var_xdrifts_dn9;
        locals.var_xdrifts_dc_rv = 0.0;

        locals.var_ds_dc = locals.var_ds;
        locals.var_ds_dc_dn4 = locals.var_ds_dn4;
        locals.var_ds_dc_dn6 = locals.var_ds_dn6;
        locals.var_ds_dc_dn7 = locals.var_ds_dn7;
        locals.var_ds_dc_dn8 = locals.var_ds_dn8;
        locals.var_ds_dc_dn9 = locals.var_ds_dn9;
        locals.var_ds_dc_rv = 0.0;

        locals.var_k1q1d_dc = locals.var_k1q1d;
        locals.var_k1q1d_dc_dn4 = locals.var_k1q1d_dn4;
        locals.var_k1q1d_dc_dn6 = locals.var_k1q1d_dn6;
        locals.var_k1q1d_dc_dn7 = locals.var_k1q1d_dn7;
        locals.var_k1q1d_dc_dn8 = locals.var_k1q1d_dn8;
        locals.var_k1q1d_dc_dn9 = locals.var_k1q1d_dn9;
        locals.var_k1q1d_dc_rv = 0.0;

        locals.var_k2q2d_dc = locals.var_k2q2d;
        locals.var_k2q2d_dc_dn4 = locals.var_k2q2d_dn4;
        locals.var_k2q2d_dc_dn6 = locals.var_k2q2d_dn6;
        locals.var_k2q2d_dc_dn7 = locals.var_k2q2d_dn7;
        locals.var_k2q2d_dc_dn8 = locals.var_k2q2d_dn8;
        locals.var_k2q2d_dc_dn9 = locals.var_k2q2d_dn9;
        locals.var_k2q2d_dc_rv = 0.0;

        locals.var_xdeff_dc = locals.var_xdeff;
        locals.var_xdeff_dc_dn4 = locals.var_xdeff_dn4;
        locals.var_xdeff_dc_dn6 = locals.var_xdeff_dn6;
        locals.var_xdeff_dc_dn7 = locals.var_xdeff_dn7;
        locals.var_xdeff_dc_dn8 = locals.var_xdeff_dn8;
        locals.var_xdeff_dc_dn9 = locals.var_xdeff_dn9;
        locals.var_xdeff_dc_rv = 0.0;

        locals.var_q1d_dc = locals.var_q1d;
        locals.var_q1d_dc_dn4 = locals.var_q1d_dn4;
        locals.var_q1d_dc_dn6 = locals.var_q1d_dn6;
        locals.var_q1d_dc_dn7 = locals.var_q1d_dn7;
        locals.var_q1d_dc_dn8 = locals.var_q1d_dn8;
        locals.var_q1d_dc_dn9 = locals.var_q1d_dn9;
        locals.var_q1d_dc_rv = 0.0;

        locals.var_qid_dc = locals.var_qid;
        locals.var_qid_dc_dn4 = locals.var_qid_dn4;
        locals.var_qid_dc_dn6 = locals.var_qid_dn6;
        locals.var_qid_dc_dn7 = locals.var_qid_dn7;
        locals.var_qid_dc_dn8 = locals.var_qid_dn8;
        locals.var_qid_dc_dn9 = locals.var_qid_dn9;
        locals.var_qid_dc_rv = 0.0;

        locals.var_xdriftd_dc = locals.var_xdriftd;
        locals.var_xdriftd_dc_dn4 = locals.var_xdriftd_dn4;
        locals.var_xdriftd_dc_dn6 = locals.var_xdriftd_dn6;
        locals.var_xdriftd_dc_dn7 = locals.var_xdriftd_dn7;
        locals.var_xdriftd_dc_dn8 = locals.var_xdriftd_dn8;
        locals.var_xdriftd_dc_dn9 = locals.var_xdriftd_dn9;
        locals.var_xdriftd_dc_rv = 0.0;

        locals.var_qim_dc = locals.var_qim;
        locals.var_qim_dc_dn4 = locals.var_qim_dn4;
        locals.var_qim_dc_dn6 = locals.var_qim_dn6;
        locals.var_qim_dc_dn7 = locals.var_qim_dn7;
        locals.var_qim_dc_dn8 = locals.var_qim_dn8;
        locals.var_qim_dc_dn9 = locals.var_qim_dn9;
        locals.var_qim_dc_rv = 0.0;

        locals.var_ratio_pd_dc = locals.var_ratio_pd;
        locals.var_ratio_pd_dc_dn4 = locals.var_ratio_pd_dn4;
        locals.var_ratio_pd_dc_dn6 = locals.var_ratio_pd_dn6;
        locals.var_ratio_pd_dc_dn7 = locals.var_ratio_pd_dn7;
        locals.var_ratio_pd_dc_dn8 = locals.var_ratio_pd_dn8;
        locals.var_ratio_pd_dc_dn9 = locals.var_ratio_pd_dn9;
        locals.var_ratio_pd_dc_rv = 0.0;

        locals.var_esurf1_dc = locals.var_esurf1;
        locals.var_esurf1_dc_dn4 = locals.var_esurf1_dn4;
        locals.var_esurf1_dc_dn6 = locals.var_esurf1_dn6;
        locals.var_esurf1_dc_dn7 = locals.var_esurf1_dn7;
        locals.var_esurf1_dc_dn8 = locals.var_esurf1_dn8;
        locals.var_esurf1_dc_dn9 = locals.var_esurf1_dn9;
        locals.var_esurf1_dc_rv = 0.0;

        locals.var_esurf2_dc = locals.var_esurf2;
        locals.var_esurf2_dc_dn4 = locals.var_esurf2_dn4;
        locals.var_esurf2_dc_dn6 = locals.var_esurf2_dn6;
        locals.var_esurf2_dc_dn7 = locals.var_esurf2_dn7;
        locals.var_esurf2_dc_dn8 = locals.var_esurf2_dn8;
        locals.var_esurf2_dc_dn9 = locals.var_esurf2_dn9;
        locals.var_esurf2_dc_rv = 0.0;

        locals.var_qi1m_dc = locals.var_qi1m;
        locals.var_qi1m_dc_dn4 = locals.var_qi1m_dn4;
        locals.var_qi1m_dc_dn6 = locals.var_qi1m_dn6;
        locals.var_qi1m_dc_dn7 = locals.var_qi1m_dn7;
        locals.var_qi1m_dc_dn8 = locals.var_qi1m_dn8;
        locals.var_qi1m_dc_dn9 = locals.var_qi1m_dn9;
        locals.var_qi1m_dc_rv = 0.0;

        locals.var_qi2m_dc = locals.var_qi2m;
        locals.var_qi2m_dc_dn4 = locals.var_qi2m_dn4;
        locals.var_qi2m_dc_dn6 = locals.var_qi2m_dn6;
        locals.var_qi2m_dc_dn7 = locals.var_qi2m_dn7;
        locals.var_qi2m_dc_dn8 = locals.var_qi2m_dn8;
        locals.var_qi2m_dc_dn9 = locals.var_qi2m_dn9;
        locals.var_qi2m_dc_rv = 0.0;

        locals.var_csum_dc = locals.var_csum;
        locals.var_csum_dc_dn4 = locals.var_csum_dn4;
        locals.var_csum_dc_dn6 = locals.var_csum_dn6;
        locals.var_csum_dc_dn7 = locals.var_csum_dn7;
        locals.var_csum_dc_dn8 = locals.var_csum_dn8;
        locals.var_csum_dc_dn9 = locals.var_csum_dn9;
        locals.var_csum_dc_rv = 0.0;

        locals.var_gmob_dc = locals.var_gmob;
        locals.var_gmob_dc_dn4 = locals.var_gmob_dn4;
        locals.var_gmob_dc_dn6 = locals.var_gmob_dn6;
        locals.var_gmob_dc_dn7 = locals.var_gmob_dn7;
        locals.var_gmob_dc_dn8 = locals.var_gmob_dn8;
        locals.var_gmob_dc_dn9 = locals.var_gmob_dn9;
        locals.var_gmob_dc_rv = 0.0;

        locals.var_inv_qimstar1_dc = locals.var_inv_qimstar1;
        locals.var_inv_qimstar1_dc_dn4 = locals.var_inv_qimstar1_dn4;
        locals.var_inv_qimstar1_dc_dn6 = locals.var_inv_qimstar1_dn6;
        locals.var_inv_qimstar1_dc_dn7 = locals.var_inv_qimstar1_dn7;
        locals.var_inv_qimstar1_dc_dn8 = locals.var_inv_qimstar1_dn8;
        locals.var_inv_qimstar1_dc_dn9 = locals.var_inv_qimstar1_dn9;
        locals.var_inv_qimstar1_dc_rv = 0.0;

        locals.var_dl_l_fact_dc = locals.var_dl_l_fact;
        locals.var_dl_l_fact_dc_dn4 = locals.var_dl_l_fact_dn4;
        locals.var_dl_l_fact_dc_dn6 = locals.var_dl_l_fact_dn6;
        locals.var_dl_l_fact_dc_dn7 = locals.var_dl_l_fact_dn7;
        locals.var_dl_l_fact_dc_dn8 = locals.var_dl_l_fact_dn8;
        locals.var_dl_l_fact_dc_dn9 = locals.var_dl_l_fact_dn9;
        locals.var_dl_l_fact_dc_rv = 0.0;

        locals.var_gdl_dc = locals.var_gdl;
        locals.var_gdl_dc_dn4 = locals.var_gdl_dn4;
        locals.var_gdl_dc_dn6 = locals.var_gdl_dn6;
        locals.var_gdl_dc_dn7 = locals.var_gdl_dn7;
        locals.var_gdl_dc_dn8 = locals.var_gdl_dn8;
        locals.var_gdl_dc_dn9 = locals.var_gdl_dn9;
        locals.var_gdl_dc_rv = 0.0;

        locals.var_vsat_fact_dc = locals.var_vsat_fact;
        locals.var_vsat_fact_dc_dn4 = locals.var_vsat_fact_dn4;
        locals.var_vsat_fact_dc_dn6 = locals.var_vsat_fact_dn6;
        locals.var_vsat_fact_dc_dn7 = locals.var_vsat_fact_dn7;
        locals.var_vsat_fact_dc_dn8 = locals.var_vsat_fact_dn8;
        locals.var_vsat_fact_dc_dn9 = locals.var_vsat_fact_dn9;
        locals.var_vsat_fact_dc_rv = 0.0;

        locals.var_zsat_dc = locals.var_zsat;
        locals.var_zsat_dc_dn4 = locals.var_zsat_dn4;
        locals.var_zsat_dc_dn6 = locals.var_zsat_dn6;
        locals.var_zsat_dc_dn7 = locals.var_zsat_dn7;
        locals.var_zsat_dc_dn8 = locals.var_zsat_dn8;
        locals.var_zsat_dc_dn9 = locals.var_zsat_dn9;
        locals.var_zsat_dc_rv = 0.0;

        locals.var_hsat_dc = locals.var_hsat;
        locals.var_hsat_dc_dn4 = locals.var_hsat_dn4;
        locals.var_hsat_dc_dn6 = locals.var_hsat_dn6;
        locals.var_hsat_dc_dn7 = locals.var_hsat_dn7;
        locals.var_hsat_dc_dn8 = locals.var_hsat_dn8;
        locals.var_hsat_dc_dn9 = locals.var_hsat_dn9;
        locals.var_hsat_dc_rv = 0.0;

        locals.var_qmfact1_dc = locals.var_qmfact1;
        locals.var_qmfact1_dc_dn4 = locals.var_qmfact1_dn4;
        locals.var_qmfact1_dc_dn6 = locals.var_qmfact1_dn6;
        locals.var_qmfact1_dc_dn7 = locals.var_qmfact1_dn7;
        locals.var_qmfact1_dc_dn8 = locals.var_qmfact1_dn8;
        locals.var_qmfact1_dc_dn9 = locals.var_qmfact1_dn9;
        locals.var_qmfact1_dc_rv = 0.0;

        locals.var_qmfact2_dc = locals.var_qmfact2;
        locals.var_qmfact2_dc_dn4 = locals.var_qmfact2_dn4;
        locals.var_qmfact2_dc_dn6 = locals.var_qmfact2_dn6;
        locals.var_qmfact2_dc_dn7 = locals.var_qmfact2_dn7;
        locals.var_qmfact2_dc_dn8 = locals.var_qmfact2_dn8;
        locals.var_qmfact2_dc_dn9 = locals.var_qmfact2_dn9;
        locals.var_qmfact2_dc_rv = 0.0;

        locals.var_dd_dc = locals.var_dd;
        locals.var_dd_dc_dn4 = locals.var_dd_dn4;
        locals.var_dd_dc_dn6 = locals.var_dd_dn6;
        locals.var_dd_dc_dn7 = locals.var_dd_dn7;
        locals.var_dd_dc_dn8 = locals.var_dd_dn8;
        locals.var_dd_dc_dn9 = locals.var_dd_dn9;
        locals.var_dd_dc_rv = 0.0;

        locals.var_norm_ids_dc = locals.var_norm_ids;
        locals.var_norm_ids_dc_dn4 = locals.var_norm_ids_dn4;
        locals.var_norm_ids_dc_dn6 = locals.var_norm_ids_dn6;
        locals.var_norm_ids_dc_dn7 = locals.var_norm_ids_dn7;
        locals.var_norm_ids_dc_dn8 = locals.var_norm_ids_dn8;
        locals.var_norm_ids_dc_dn9 = locals.var_norm_ids_dn9;
        locals.var_norm_ids_dc_rv = 0.0;

        locals.var_inv_k1h1_0_dc = locals.var_inv_k1h1_0;
        locals.var_inv_k1h1_0_dc_dn4 = locals.var_inv_k1h1_0_dn4;
        locals.var_inv_k1h1_0_dc_dn6 = locals.var_inv_k1h1_0_dn6;
        locals.var_inv_k1h1_0_dc_dn7 = locals.var_inv_k1h1_0_dn7;
        locals.var_inv_k1h1_0_dc_dn8 = locals.var_inv_k1h1_0_dn8;
        locals.var_inv_k1h1_0_dc_dn9 = locals.var_inv_k1h1_0_dn9;
        locals.var_inv_k1h1_0_dc_rv = 0.0;

        locals.var_delta_k1q1_dc = locals.var_delta_k1q1;
        locals.var_delta_k1q1_dc_dn4 = locals.var_delta_k1q1_dn4;
        locals.var_delta_k1q1_dc_dn6 = locals.var_delta_k1q1_dn6;
        locals.var_delta_k1q1_dc_dn7 = locals.var_delta_k1q1_dn7;
        locals.var_delta_k1q1_dc_dn8 = locals.var_delta_k1q1_dn8;
        locals.var_delta_k1q1_dc_dn9 = locals.var_delta_k1q1_dn9;
        locals.var_delta_k1q1_dc_rv = 0.0;

        locals.var_delta_k2q2_dc = locals.var_delta_k2q2;
        locals.var_delta_k2q2_dc_dn4 = locals.var_delta_k2q2_dn4;
        locals.var_delta_k2q2_dc_dn6 = locals.var_delta_k2q2_dn6;
        locals.var_delta_k2q2_dc_dn7 = locals.var_delta_k2q2_dn7;
        locals.var_delta_k2q2_dc_dn8 = locals.var_delta_k2q2_dn8;
        locals.var_delta_k2q2_dc_dn9 = locals.var_delta_k2q2_dn9;
        locals.var_delta_k2q2_dc_rv = 0.0;

        locals.var_prod1_dc = locals.var_prod1;
        locals.var_prod1_dc_dn4 = locals.var_prod1_dn4;
        locals.var_prod1_dc_dn6 = locals.var_prod1_dn6;
        locals.var_prod1_dc_dn7 = locals.var_prod1_dn7;
        locals.var_prod1_dc_dn8 = locals.var_prod1_dn8;
        locals.var_prod1_dc_dn9 = locals.var_prod1_dn9;
        locals.var_prod1_dc_rv = 0.0;

        locals.var_prod2_dc = locals.var_prod2;
        locals.var_prod2_dc_dn4 = locals.var_prod2_dn4;
        locals.var_prod2_dc_dn6 = locals.var_prod2_dn6;
        locals.var_prod2_dc_dn7 = locals.var_prod2_dn7;
        locals.var_prod2_dc_dn8 = locals.var_prod2_dn8;
        locals.var_prod2_dc_dn9 = locals.var_prod2_dn9;
        locals.var_prod2_dc_rv = 0.0;

        let assign23200_e22905: f64 = (locals.var_csum_dc * p.p35);
        let assign23200_e22908: f64 = (locals.var_esurf1_dc + locals.var_esurf2_dc);
        let assign23200_e22909: f64 = (assign23200_e22905 / assign23200_e22908);
        locals.var_betneff = assign23200_e22909;
        locals.var_betneff_dn4 = ((((locals.var_csum_dc_dn4 * p.p35) * assign23200_e22908) - (assign23200_e22905 * (locals.var_esurf1_dc_dn4 + locals.var_esurf2_dc_dn4))) / (assign23200_e22908 * assign23200_e22908));
        locals.var_betneff_dn6 = ((((locals.var_csum_dc_dn6 * p.p35) * assign23200_e22908) - (assign23200_e22905 * (locals.var_esurf1_dc_dn6 + locals.var_esurf2_dc_dn6))) / (assign23200_e22908 * assign23200_e22908));
        locals.var_betneff_dn7 = ((((locals.var_csum_dc_dn7 * p.p35) * assign23200_e22908) - (assign23200_e22905 * (locals.var_esurf1_dc_dn7 + locals.var_esurf2_dc_dn7))) / (assign23200_e22908 * assign23200_e22908));
        locals.var_betneff_dn8 = ((((locals.var_csum_dc_dn8 * p.p35) * assign23200_e22908) - (assign23200_e22905 * (locals.var_esurf1_dc_dn8 + locals.var_esurf2_dc_dn8))) / (assign23200_e22908 * assign23200_e22908));
        locals.var_betneff_dn9 = ((((locals.var_csum_dc_dn9 * p.p35) * assign23200_e22908) - (assign23200_e22905 * (locals.var_esurf1_dc_dn9 + locals.var_esurf2_dc_dn9))) / (assign23200_e22908 * assign23200_e22908));
        locals.var_betneff_rv = 0.0;

        let assign23210_e22913: f64 = (locals.var_alp1_phit * locals.var_inv_qimstar1_dc);
        let assign23210_e22914: f64 = (locals.var_alp_i + assign23210_e22913);
        let assign23210_e22916: f64 = (assign23210_e22914 * locals.var_dl_l_fact_dc);
        locals.var_dl1_l = assign23210_e22916;
        locals.var_dl1_l_dn4 = ((((locals.var_alp1_phit_dn4 * locals.var_inv_qimstar1_dc) + (locals.var_alp1_phit * locals.var_inv_qimstar1_dc_dn4)) * locals.var_dl_l_fact_dc) + (assign23210_e22914 * locals.var_dl_l_fact_dc_dn4));
        locals.var_dl1_l_dn6 = ((((locals.var_alp1_phit_dn6 * locals.var_inv_qimstar1_dc) + (locals.var_alp1_phit * locals.var_inv_qimstar1_dc_dn6)) * locals.var_dl_l_fact_dc) + (assign23210_e22914 * locals.var_dl_l_fact_dc_dn6));
        locals.var_dl1_l_dn7 = ((((locals.var_alp1_phit_dn7 * locals.var_inv_qimstar1_dc) + (locals.var_alp1_phit * locals.var_inv_qimstar1_dc_dn7)) * locals.var_dl_l_fact_dc) + (assign23210_e22914 * locals.var_dl_l_fact_dc_dn7));
        locals.var_dl1_l_dn8 = ((((locals.var_alp1_phit_dn8 * locals.var_inv_qimstar1_dc) + (locals.var_alp1_phit * locals.var_inv_qimstar1_dc_dn8)) * locals.var_dl_l_fact_dc) + (assign23210_e22914 * locals.var_dl_l_fact_dc_dn8));
        locals.var_dl1_l_dn9 = ((((locals.var_alp1_phit_dn9 * locals.var_inv_qimstar1_dc) + (locals.var_alp1_phit * locals.var_inv_qimstar1_dc_dn9)) * locals.var_dl_l_fact_dc) + (assign23210_e22914 * locals.var_dl_l_fact_dc_dn9));
        locals.var_dl1_l_rv = 0.0;

        let assign23220_e22921: f64 = (1.0 + locals.var_dl1_l);
        let assign23220_e22922: f64 = (locals.var_dl1_l * assign23220_e22921);
        let assign23220_e22923: f64 = (1.0 + assign23220_e22922);
        let assign23220_e22925: f64 = (assign23220_e22923 * locals.var_gdl_dc);
        locals.var_fdl = assign23220_e22925;
        locals.var_fdl_dn4 = ((((locals.var_dl1_l_dn4 * assign23220_e22921) + (locals.var_dl1_l * locals.var_dl1_l_dn4)) * locals.var_gdl_dc) + (assign23220_e22923 * locals.var_gdl_dc_dn4));
        locals.var_fdl_dn6 = ((((locals.var_dl1_l_dn6 * assign23220_e22921) + (locals.var_dl1_l * locals.var_dl1_l_dn6)) * locals.var_gdl_dc) + (assign23220_e22923 * locals.var_gdl_dc_dn6));
        locals.var_fdl_dn7 = ((((locals.var_dl1_l_dn7 * assign23220_e22921) + (locals.var_dl1_l * locals.var_dl1_l_dn7)) * locals.var_gdl_dc) + (assign23220_e22923 * locals.var_gdl_dc_dn7));
        locals.var_fdl_dn8 = ((((locals.var_dl1_l_dn8 * assign23220_e22921) + (locals.var_dl1_l * locals.var_dl1_l_dn8)) * locals.var_gdl_dc) + (assign23220_e22923 * locals.var_gdl_dc_dn8));
        locals.var_fdl_dn9 = ((((locals.var_dl1_l_dn9 * assign23220_e22921) + (locals.var_dl1_l * locals.var_dl1_l_dn9)) * locals.var_gdl_dc) + (assign23220_e22923 * locals.var_gdl_dc_dn9));
        locals.var_fdl_rv = 0.0;

        let assign23230_e22928: f64 = (locals.var_gmob_dc * locals.var_gdl_dc);
        let assign23230_e22930: f64 = (assign23230_e22928 * locals.var_vsat_fact_dc);
        locals.var_gvsat = assign23230_e22930;
        locals.var_gvsat_dn4 = ((((locals.var_gmob_dc_dn4 * locals.var_gdl_dc) + (locals.var_gmob_dc * locals.var_gdl_dc_dn4)) * locals.var_vsat_fact_dc) + (assign23230_e22928 * locals.var_vsat_fact_dc_dn4));
        locals.var_gvsat_dn6 = ((((locals.var_gmob_dc_dn6 * locals.var_gdl_dc) + (locals.var_gmob_dc * locals.var_gdl_dc_dn6)) * locals.var_vsat_fact_dc) + (assign23230_e22928 * locals.var_vsat_fact_dc_dn6));
        locals.var_gvsat_dn7 = ((((locals.var_gmob_dc_dn7 * locals.var_gdl_dc) + (locals.var_gmob_dc * locals.var_gdl_dc_dn7)) * locals.var_vsat_fact_dc) + (assign23230_e22928 * locals.var_vsat_fact_dc_dn7));
        locals.var_gvsat_dn8 = ((((locals.var_gmob_dc_dn8 * locals.var_gdl_dc) + (locals.var_gmob_dc * locals.var_gdl_dc_dn8)) * locals.var_vsat_fact_dc) + (assign23230_e22928 * locals.var_vsat_fact_dc_dn8));
        locals.var_gvsat_dn9 = ((((locals.var_gmob_dc_dn9 * locals.var_gdl_dc) + (locals.var_gmob_dc * locals.var_gdl_dc_dn9)) * locals.var_vsat_fact_dc) + (assign23230_e22928 * locals.var_vsat_fact_dc_dn9));
        locals.var_gvsat_rv = 0.0;

        let assign23240_e22933: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard684 = assign23240_e22933;
        locals.var_guard684_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_63(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23250_e22947, assign23250_e22947_d_n4, assign23250_e22947_d_n6, assign23250_e22947_d_n7, assign23250_e22947_d_n8, assign23250_e22947_d_n9,) = {
    if (locals.var_guard684 != 0.0) {
        let assign23250_e22937: f64 = (locals.var_esurf1_dc + locals.var_esurf2_dc);
        let assign23250_e22940: f64 = (locals.var_esurf1_dc / locals.var_qmfact1_dc);
        let assign23250_e22943: f64 = (locals.var_esurf2_dc / locals.var_qmfact2_dc);
        let assign23250_e22944: f64 = (assign23250_e22940 + assign23250_e22943);
        let assign23250_e22945: f64 = (assign23250_e22937 / assign23250_e22944);
        (assign23250_e22945, ((((locals.var_esurf1_dc_dn4 + locals.var_esurf2_dc_dn4) * assign23250_e22944) - (assign23250_e22937 * ((((locals.var_esurf1_dc_dn4 * locals.var_qmfact1_dc) - (locals.var_esurf1_dc * locals.var_qmfact1_dc_dn4)) / (locals.var_qmfact1_dc * locals.var_qmfact1_dc)) + (((locals.var_esurf2_dc_dn4 * locals.var_qmfact2_dc) - (locals.var_esurf2_dc * locals.var_qmfact2_dc_dn4)) / (locals.var_qmfact2_dc * locals.var_qmfact2_dc))))) / (assign23250_e22944 * assign23250_e22944)), ((((locals.var_esurf1_dc_dn6 + locals.var_esurf2_dc_dn6) * assign23250_e22944) - (assign23250_e22937 * ((((locals.var_esurf1_dc_dn6 * locals.var_qmfact1_dc) - (locals.var_esurf1_dc * locals.var_qmfact1_dc_dn6)) / (locals.var_qmfact1_dc * locals.var_qmfact1_dc)) + (((locals.var_esurf2_dc_dn6 * locals.var_qmfact2_dc) - (locals.var_esurf2_dc * locals.var_qmfact2_dc_dn6)) / (locals.var_qmfact2_dc * locals.var_qmfact2_dc))))) / (assign23250_e22944 * assign23250_e22944)), ((((locals.var_esurf1_dc_dn7 + locals.var_esurf2_dc_dn7) * assign23250_e22944) - (assign23250_e22937 * ((((locals.var_esurf1_dc_dn7 * locals.var_qmfact1_dc) - (locals.var_esurf1_dc * locals.var_qmfact1_dc_dn7)) / (locals.var_qmfact1_dc * locals.var_qmfact1_dc)) + (((locals.var_esurf2_dc_dn7 * locals.var_qmfact2_dc) - (locals.var_esurf2_dc * locals.var_qmfact2_dc_dn7)) / (locals.var_qmfact2_dc * locals.var_qmfact2_dc))))) / (assign23250_e22944 * assign23250_e22944)), ((((locals.var_esurf1_dc_dn8 + locals.var_esurf2_dc_dn8) * assign23250_e22944) - (assign23250_e22937 * ((((locals.var_esurf1_dc_dn8 * locals.var_qmfact1_dc) - (locals.var_esurf1_dc * locals.var_qmfact1_dc_dn8)) / (locals.var_qmfact1_dc * locals.var_qmfact1_dc)) + (((locals.var_esurf2_dc_dn8 * locals.var_qmfact2_dc) - (locals.var_esurf2_dc * locals.var_qmfact2_dc_dn8)) / (locals.var_qmfact2_dc * locals.var_qmfact2_dc))))) / (assign23250_e22944 * assign23250_e22944)), ((((locals.var_esurf1_dc_dn9 + locals.var_esurf2_dc_dn9) * assign23250_e22944) - (assign23250_e22937 * ((((locals.var_esurf1_dc_dn9 * locals.var_qmfact1_dc) - (locals.var_esurf1_dc * locals.var_qmfact1_dc_dn9)) / (locals.var_qmfact1_dc * locals.var_qmfact1_dc)) + (((locals.var_esurf2_dc_dn9 * locals.var_qmfact2_dc) - (locals.var_esurf2_dc * locals.var_qmfact2_dc_dn9)) / (locals.var_qmfact2_dc * locals.var_qmfact2_dc))))) / (assign23250_e22944 * assign23250_e22944)),)
    } else {
        (locals.var_qmfact, locals.var_qmfact_dn4, locals.var_qmfact_dn6, locals.var_qmfact_dn7, locals.var_qmfact_dn8, locals.var_qmfact_dn9,)
    }
};
        locals.var_qmfact = assign23250_e22947;
        locals.var_qmfact_dn4 = assign23250_e22947_d_n4;
        locals.var_qmfact_dn6 = assign23250_e22947_d_n6;
        locals.var_qmfact_dn7 = assign23250_e22947_d_n7;
        locals.var_qmfact_dn8 = assign23250_e22947_d_n8;
        locals.var_qmfact_dn9 = assign23250_e22947_d_n9;
        locals.var_qmfact_rv = 0.0;

        let (assign23260_e22952, assign23260_e22952_d_n4, assign23260_e22952_d_n6, assign23260_e22952_d_n7, assign23260_e22952_d_n8, assign23260_e22952_d_n9,) = {
    if (locals.var_guard684 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qmfact, locals.var_qmfact_dn4, locals.var_qmfact_dn6, locals.var_qmfact_dn7, locals.var_qmfact_dn8, locals.var_qmfact_dn9,)
    }
};
        locals.var_qmfact = assign23260_e22952;
        locals.var_qmfact_dn4 = assign23260_e22952_d_n4;
        locals.var_qmfact_dn6 = assign23260_e22952_d_n6;
        locals.var_qmfact_dn7 = assign23260_e22952_d_n7;
        locals.var_qmfact_dn8 = assign23260_e22952_d_n8;
        locals.var_qmfact_dn9 = assign23260_e22952_d_n9;
        locals.var_qmfact_rv = 0.0;

        let assign23270_e22955: f64 = (locals.var_phit * locals.var_phit);
        let assign23270_e22957: f64 = (assign23270_e22955 * locals.var_betneff);
        locals.var_fact_ids = assign23270_e22957;
        locals.var_fact_ids_dn4 = ((((locals.var_phit_dn4 * locals.var_phit) + (locals.var_phit * locals.var_phit_dn4)) * locals.var_betneff) + (assign23270_e22955 * locals.var_betneff_dn4));
        locals.var_fact_ids_dn6 = ((((locals.var_phit_dn6 * locals.var_phit) + (locals.var_phit * locals.var_phit_dn6)) * locals.var_betneff) + (assign23270_e22955 * locals.var_betneff_dn6));
        locals.var_fact_ids_dn7 = ((((locals.var_phit_dn7 * locals.var_phit) + (locals.var_phit * locals.var_phit_dn7)) * locals.var_betneff) + (assign23270_e22955 * locals.var_betneff_dn7));
        locals.var_fact_ids_dn8 = ((((locals.var_phit_dn8 * locals.var_phit) + (locals.var_phit * locals.var_phit_dn8)) * locals.var_betneff) + (assign23270_e22955 * locals.var_betneff_dn8));
        locals.var_fact_ids_dn9 = ((((locals.var_phit_dn9 * locals.var_phit) + (locals.var_phit * locals.var_phit_dn9)) * locals.var_betneff) + (assign23270_e22955 * locals.var_betneff_dn9));
        locals.var_fact_ids_rv = 0.0;

        let assign23280_e22960: f64 = (locals.var_fact_ids * locals.var_csiprime_dc);
        let assign23280_e22962: f64 = (assign23280_e22960 * locals.var_norm_ids_dc);
        let assign23280_e22964: f64 = (assign23280_e22962 * locals.var_fdl);
        let assign23280_e22966: f64 = (assign23280_e22964 / locals.var_gvsat);
        let assign23280_e22968: f64 = (assign23280_e22966 / locals.var_qmfact);
        locals.var_ids = assign23280_e22968;
        locals.var_ids_dn4 = ((((((((((((locals.var_fact_ids_dn4 * locals.var_csiprime_dc) + (locals.var_fact_ids * locals.var_csiprime_dc_dn4)) * locals.var_norm_ids_dc) + (assign23280_e22960 * locals.var_norm_ids_dc_dn4)) * locals.var_fdl) + (assign23280_e22962 * locals.var_fdl_dn4)) * locals.var_gvsat) - (assign23280_e22964 * locals.var_gvsat_dn4)) / (locals.var_gvsat * locals.var_gvsat)) * locals.var_qmfact) - (assign23280_e22966 * locals.var_qmfact_dn4)) / (locals.var_qmfact * locals.var_qmfact));
        locals.var_ids_dn6 = ((((((((((((locals.var_fact_ids_dn6 * locals.var_csiprime_dc) + (locals.var_fact_ids * locals.var_csiprime_dc_dn6)) * locals.var_norm_ids_dc) + (assign23280_e22960 * locals.var_norm_ids_dc_dn6)) * locals.var_fdl) + (assign23280_e22962 * locals.var_fdl_dn6)) * locals.var_gvsat) - (assign23280_e22964 * locals.var_gvsat_dn6)) / (locals.var_gvsat * locals.var_gvsat)) * locals.var_qmfact) - (assign23280_e22966 * locals.var_qmfact_dn6)) / (locals.var_qmfact * locals.var_qmfact));
        locals.var_ids_dn7 = ((((((((((((locals.var_fact_ids_dn7 * locals.var_csiprime_dc) + (locals.var_fact_ids * locals.var_csiprime_dc_dn7)) * locals.var_norm_ids_dc) + (assign23280_e22960 * locals.var_norm_ids_dc_dn7)) * locals.var_fdl) + (assign23280_e22962 * locals.var_fdl_dn7)) * locals.var_gvsat) - (assign23280_e22964 * locals.var_gvsat_dn7)) / (locals.var_gvsat * locals.var_gvsat)) * locals.var_qmfact) - (assign23280_e22966 * locals.var_qmfact_dn7)) / (locals.var_qmfact * locals.var_qmfact));
        locals.var_ids_dn8 = ((((((((((((locals.var_fact_ids_dn8 * locals.var_csiprime_dc) + (locals.var_fact_ids * locals.var_csiprime_dc_dn8)) * locals.var_norm_ids_dc) + (assign23280_e22960 * locals.var_norm_ids_dc_dn8)) * locals.var_fdl) + (assign23280_e22962 * locals.var_fdl_dn8)) * locals.var_gvsat) - (assign23280_e22964 * locals.var_gvsat_dn8)) / (locals.var_gvsat * locals.var_gvsat)) * locals.var_qmfact) - (assign23280_e22966 * locals.var_qmfact_dn8)) / (locals.var_qmfact * locals.var_qmfact));
        locals.var_ids_dn9 = ((((((((((((locals.var_fact_ids_dn9 * locals.var_csiprime_dc) + (locals.var_fact_ids * locals.var_csiprime_dc_dn9)) * locals.var_norm_ids_dc) + (assign23280_e22960 * locals.var_norm_ids_dc_dn9)) * locals.var_fdl) + (assign23280_e22962 * locals.var_fdl_dn9)) * locals.var_gvsat) - (assign23280_e22964 * locals.var_gvsat_dn9)) / (locals.var_gvsat * locals.var_gvsat)) * locals.var_qmfact) - (assign23280_e22966 * locals.var_qmfact_dn9)) / (locals.var_qmfact * locals.var_qmfact));
        locals.var_ids_rv = 0.0;

        let assign23290_e22970: f64 = (-locals.var_vgsu);
        let assign23290_e22972: f64 = (assign23290_e22970 * locals.var_inv_phit0);
        locals.var_xgs_ov = assign23290_e22972;
        locals.var_xgs_ov_dn4 = (assign23290_e22970 * locals.var_inv_phit0_dn4);
        locals.var_xgs_ov_dn6 = (((-locals.var_vgsu_dn6) * locals.var_inv_phit0) + (assign23290_e22970 * locals.var_inv_phit0_dn6));
        locals.var_xgs_ov_dn7 = (assign23290_e22970 * locals.var_inv_phit0_dn7);
        locals.var_xgs_ov_dn8 = (assign23290_e22970 * locals.var_inv_phit0_dn8);
        locals.var_xgs_ov_dn9 = (((-locals.var_vgsu_dn9) * locals.var_inv_phit0) + (assign23290_e22970 * locals.var_inv_phit0_dn9));
        locals.var_xgs_ov_rv = 0.0;

        let assign23300_e22974: f64 = (-locals.var_vgdu);
        let assign23300_e22976: f64 = (assign23300_e22974 * locals.var_inv_phit0);
        locals.var_xgd_ov = assign23300_e22976;
        locals.var_xgd_ov_dn4 = (assign23300_e22974 * locals.var_inv_phit0_dn4);
        locals.var_xgd_ov_dn6 = (((-locals.var_vgdu_dn6) * locals.var_inv_phit0) + (assign23300_e22974 * locals.var_inv_phit0_dn6));
        locals.var_xgd_ov_dn7 = (((-locals.var_vgdu_dn7) * locals.var_inv_phit0) + (assign23300_e22974 * locals.var_inv_phit0_dn7));
        locals.var_xgd_ov_dn8 = (assign23300_e22974 * locals.var_inv_phit0_dn8);
        locals.var_xgd_ov_dn9 = (((-locals.var_vgdu_dn9) * locals.var_inv_phit0) + (assign23300_e22974 * locals.var_inv_phit0_dn9));
        locals.var_xgd_ov_rv = 0.0;

        let assign23310_e22979: f64 = (p.p14 * locals.var_dvfbov_i);
        let assign23310_e22981: f64 = (assign23310_e22979 * locals.var_inv_phit0);
        let assign23310_e22983: f64 = (assign23310_e22981 + locals.var_eg_2phit0);
        locals.var_temp = assign23310_e22983;
        locals.var_temp_dn4 = ((assign23310_e22979 * locals.var_inv_phit0_dn4) + locals.var_eg_2phit0_dn4);
        locals.var_temp_dn6 = ((assign23310_e22979 * locals.var_inv_phit0_dn6) + locals.var_eg_2phit0_dn6);
        locals.var_temp_dn7 = ((assign23310_e22979 * locals.var_inv_phit0_dn7) + locals.var_eg_2phit0_dn7);
        locals.var_temp_dn8 = ((assign23310_e22979 * locals.var_inv_phit0_dn8) + locals.var_eg_2phit0_dn8);
        locals.var_temp_dn9 = ((assign23310_e22979 * locals.var_inv_phit0_dn9) + locals.var_eg_2phit0_dn9);
        locals.var_temp_rv = 0.0;

        let assign23320_e22986: f64 = (locals.var_xgs_ov + locals.var_temp);
        locals.var_xgs_ovcv = assign23320_e22986;
        locals.var_xgs_ovcv_dn4 = (locals.var_xgs_ov_dn4 + locals.var_temp_dn4);
        locals.var_xgs_ovcv_dn6 = (locals.var_xgs_ov_dn6 + locals.var_temp_dn6);
        locals.var_xgs_ovcv_dn7 = (locals.var_xgs_ov_dn7 + locals.var_temp_dn7);
        locals.var_xgs_ovcv_dn8 = (locals.var_xgs_ov_dn8 + locals.var_temp_dn8);
        locals.var_xgs_ovcv_dn9 = (locals.var_xgs_ov_dn9 + locals.var_temp_dn9);
        locals.var_xgs_ovcv_rv = 0.0;

        let assign23330_e22989: f64 = (locals.var_xgd_ov + locals.var_temp);
        locals.var_xgd_ovcv = assign23330_e22989;
        locals.var_xgd_ovcv_dn4 = (locals.var_xgd_ov_dn4 + locals.var_temp_dn4);
        locals.var_xgd_ovcv_dn6 = (locals.var_xgd_ov_dn6 + locals.var_temp_dn6);
        locals.var_xgd_ovcv_dn7 = (locals.var_xgd_ov_dn7 + locals.var_temp_dn7);
        locals.var_xgd_ovcv_dn8 = (locals.var_xgd_ov_dn8 + locals.var_temp_dn8);
        locals.var_xgd_ovcv_dn9 = (locals.var_xgd_ov_dn9 + locals.var_temp_dn9);
        locals.var_xgd_ovcv_rv = 0.0;

        locals.var_xs_ov = 0.0;
        locals.var_xs_ov_dn4 = 0.0;
        locals.var_xs_ov_dn6 = 0.0;
        locals.var_xs_ov_dn7 = 0.0;
        locals.var_xs_ov_dn8 = 0.0;
        locals.var_xs_ov_dn9 = 0.0;
        locals.var_xs_ov_rv = 0.0;

        locals.var_xd_ov = 0.0;
        locals.var_xd_ov_dn4 = 0.0;
        locals.var_xd_ov_dn6 = 0.0;
        locals.var_xd_ov_dn7 = 0.0;
        locals.var_xd_ov_dn8 = 0.0;
        locals.var_xd_ov_dn9 = 0.0;
        locals.var_xd_ov_rv = 0.0;

        locals.var_xs_ovcv = 0.0;
        locals.var_xs_ovcv_dn4 = 0.0;
        locals.var_xs_ovcv_dn6 = 0.0;
        locals.var_xs_ovcv_dn7 = 0.0;
        locals.var_xs_ovcv_dn8 = 0.0;
        locals.var_xs_ovcv_dn9 = 0.0;
        locals.var_xs_ovcv_rv = 0.0;

        locals.var_xd_ovcv = 0.0;
        locals.var_xd_ovcv_dn4 = 0.0;
        locals.var_xd_ovcv_dn6 = 0.0;
        locals.var_xd_ovcv_dn7 = 0.0;
        locals.var_xd_ovcv_dn8 = 0.0;
        locals.var_xd_ovcv_dn9 = 0.0;
        locals.var_xd_ovcv_rv = 0.0;

        let assign23380_e22996: f64 = (2.0 * 1.602176565e-19);
        let assign23380_e22998: f64 = (assign23380_e22996 * locals.var_nov_i);
        let assign23380_e23000: f64 = (assign23380_e22998 * locals.var_epsch);
        let assign23380_e23002: f64 = (assign23380_e23000 * locals.var_inv_phit0);
        let assign23380_e23003: f64 = (assign23380_e23002).sqrt();
        let assign23380_e23005: f64 = (assign23380_e23003 / locals.var_cox1prime);
        locals.var_gov = assign23380_e23005;
        locals.var_gov_dn4 = (((assign23380_e23000 * locals.var_inv_phit0_dn4) / (2.0 * assign23380_e23003)) / locals.var_cox1prime);
        locals.var_gov_dn6 = (((assign23380_e23000 * locals.var_inv_phit0_dn6) / (2.0 * assign23380_e23003)) / locals.var_cox1prime);
        locals.var_gov_dn7 = (((assign23380_e23000 * locals.var_inv_phit0_dn7) / (2.0 * assign23380_e23003)) / locals.var_cox1prime);
        locals.var_gov_dn8 = (((assign23380_e23000 * locals.var_inv_phit0_dn8) / (2.0 * assign23380_e23003)) / locals.var_cox1prime);
        locals.var_gov_dn9 = (((assign23380_e23000 * locals.var_inv_phit0_dn9) / (2.0 * assign23380_e23003)) / locals.var_cox1prime);
        locals.var_gov_rv = 0.0;

        let assign23390_e23008: f64 = (locals.var_gov * locals.var_gov);
        locals.var_gov2 = assign23390_e23008;
        locals.var_gov2_dn4 = ((locals.var_gov_dn4 * locals.var_gov) + (locals.var_gov * locals.var_gov_dn4));
        locals.var_gov2_dn6 = ((locals.var_gov_dn6 * locals.var_gov) + (locals.var_gov * locals.var_gov_dn6));
        locals.var_gov2_dn7 = ((locals.var_gov_dn7 * locals.var_gov) + (locals.var_gov * locals.var_gov_dn7));
        locals.var_gov2_dn8 = ((locals.var_gov_dn8 * locals.var_gov) + (locals.var_gov * locals.var_gov_dn8));
        locals.var_gov2_dn9 = ((locals.var_gov_dn9 * locals.var_gov) + (locals.var_gov * locals.var_gov_dn9));
        locals.var_gov2_rv = 0.0;

        let assign23400_e23012: f64 = (locals.var_gov / 1.4142135623731);
        let assign23400_e23013: f64 = (1.0 + assign23400_e23012);
        locals.var_xi_ov = assign23400_e23013;
        locals.var_xi_ov_dn4 = (locals.var_gov_dn4 / 1.4142135623731);
        locals.var_xi_ov_dn6 = (locals.var_gov_dn6 / 1.4142135623731);
        locals.var_xi_ov_dn7 = (locals.var_gov_dn7 / 1.4142135623731);
        locals.var_xi_ov_dn8 = (locals.var_gov_dn8 / 1.4142135623731);
        locals.var_xi_ov_dn9 = (locals.var_gov_dn9 / 1.4142135623731);
        locals.var_xi_ov_rv = 0.0;

        let assign23410_e23016: f64 = (1e-5 * locals.var_xi_ov);
        locals.var_x_mrg_ov = assign23410_e23016;
        locals.var_x_mrg_ov_dn4 = (1e-5 * locals.var_xi_ov_dn4);
        locals.var_x_mrg_ov_dn6 = (1e-5 * locals.var_xi_ov_dn6);
        locals.var_x_mrg_ov_dn7 = (1e-5 * locals.var_xi_ov_dn7);
        locals.var_x_mrg_ov_dn8 = (1e-5 * locals.var_xi_ov_dn8);
        locals.var_x_mrg_ov_dn9 = (1e-5 * locals.var_xi_ov_dn9);
        locals.var_x_mrg_ov_rv = 0.0;

        let assign23420_e23019: f64 = (1.0 / locals.var_xi_ov);
        locals.var_inv_xi_ov = assign23420_e23019;
        locals.var_inv_xi_ov_dn4 = (-(locals.var_xi_ov_dn4 / (locals.var_xi_ov * locals.var_xi_ov)));
        locals.var_inv_xi_ov_dn6 = (-(locals.var_xi_ov_dn6 / (locals.var_xi_ov * locals.var_xi_ov)));
        locals.var_inv_xi_ov_dn7 = (-(locals.var_xi_ov_dn7 / (locals.var_xi_ov * locals.var_xi_ov)));
        locals.var_inv_xi_ov_dn8 = (-(locals.var_xi_ov_dn8 / (locals.var_xi_ov * locals.var_xi_ov)));
        locals.var_inv_xi_ov_dn9 = (-(locals.var_xi_ov_dn9 / (locals.var_xi_ov * locals.var_xi_ov)));
        locals.var_inv_xi_ov_rv = 0.0;

        let assign23430_e23024: f64 = (locals.var_gov * 0.7324648775608221);
        let assign23430_e23025: f64 = (1.25 + assign23430_e23024);
        let assign23430_e23026: f64 = (1.0 / assign23430_e23025);
        locals.var_inv_xg1 = assign23430_e23026;
        locals.var_inv_xg1_dn4 = (-((locals.var_gov_dn4 * 0.7324648775608221) / (assign23430_e23025 * assign23430_e23025)));
        locals.var_inv_xg1_dn6 = (-((locals.var_gov_dn6 * 0.7324648775608221) / (assign23430_e23025 * assign23430_e23025)));
        locals.var_inv_xg1_dn7 = (-((locals.var_gov_dn7 * 0.7324648775608221) / (assign23430_e23025 * assign23430_e23025)));
        locals.var_inv_xg1_dn8 = (-((locals.var_gov_dn8 * 0.7324648775608221) / (assign23430_e23025 * assign23430_e23025)));
        locals.var_inv_xg1_dn9 = (-((locals.var_gov_dn9 * 0.7324648775608221) / (assign23430_e23025 * assign23430_e23025)));
        locals.var_inv_xg1_rv = 0.0;

        let assign23440_e23045: f64 = if (((p.p3 > 0.0) && ((locals.var_igovinv_i > 0.0) || (locals.var_igovacc_i > 0.0))) || ((p.p4 > 0.0) && (locals.var_agidl_i > 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard685 = assign23440_e23045;
        locals.var_guard685_rv = 0.0;

        let assign23450_e23047: f64 = (locals.var_xgs_ov).abs();
        let assign23450_e23049: f64 = if assign23450_e23047 <= locals.var_x_mrg_ov { 1.0 } else { 0.0 };
        locals.var_guard686 = assign23450_e23049;
        locals.var_guard686_rv = 0.0;

        let (assign23460_e23058, assign23460_e23058_d_n4, assign23460_e23058_d_n6, assign23460_e23058_d_n7, assign23460_e23058_d_n8, assign23460_e23058_d_n9,) = {
    if ((locals.var_guard685 != 0.0) && (locals.var_guard686 != 0.0)) {
        let assign23460_e23054: f64 = (-locals.var_xgs_ov);
        let assign23460_e23056: f64 = (assign23460_e23054 * locals.var_inv_xi_ov);
        (assign23460_e23056, (((-locals.var_xgs_ov_dn4) * locals.var_inv_xi_ov) + (assign23460_e23054 * locals.var_inv_xi_ov_dn4)), (((-locals.var_xgs_ov_dn6) * locals.var_inv_xi_ov) + (assign23460_e23054 * locals.var_inv_xi_ov_dn6)), (((-locals.var_xgs_ov_dn7) * locals.var_inv_xi_ov) + (assign23460_e23054 * locals.var_inv_xi_ov_dn7)), (((-locals.var_xgs_ov_dn8) * locals.var_inv_xi_ov) + (assign23460_e23054 * locals.var_inv_xi_ov_dn8)), (((-locals.var_xgs_ov_dn9) * locals.var_inv_xi_ov) + (assign23460_e23054 * locals.var_inv_xi_ov_dn9)),)
    } else {
        (locals.var_xs_ov, locals.var_xs_ov_dn4, locals.var_xs_ov_dn6, locals.var_xs_ov_dn7, locals.var_xs_ov_dn8, locals.var_xs_ov_dn9,)
    }
};
        locals.var_xs_ov = assign23460_e23058;
        locals.var_xs_ov_dn4 = assign23460_e23058_d_n4;
        locals.var_xs_ov_dn6 = assign23460_e23058_d_n6;
        locals.var_xs_ov_dn7 = assign23460_e23058_d_n7;
        locals.var_xs_ov_dn8 = assign23460_e23058_d_n8;
        locals.var_xs_ov_dn9 = assign23460_e23058_d_n9;
        locals.var_xs_ov_rv = 0.0;

        let assign23470_e23061: f64 = (-locals.var_x_mrg_ov);
        let assign23470_e23062: f64 = if locals.var_xgs_ov < assign23470_e23061 { 1.0 } else { 0.0 };
        locals.var_guard687 = assign23470_e23062;
        locals.var_guard687_rv = 0.0;

        let (assign23480_e23072, assign23480_e23072_d_n4, assign23480_e23072_d_n6, assign23480_e23072_d_n7, assign23480_e23072_d_n8, assign23480_e23072_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign23480_e23070: f64 = (-locals.var_xgs_ov);
        (assign23480_e23070, (-locals.var_xgs_ov_dn4), (-locals.var_xgs_ov_dn6), (-locals.var_xgs_ov_dn7), (-locals.var_xgs_ov_dn8), (-locals.var_xgs_ov_dn9),)
    } else {
        (locals.var_sp_ov_ygf, locals.var_sp_ov_ygf_dn4, locals.var_sp_ov_ygf_dn6, locals.var_sp_ov_ygf_dn7, locals.var_sp_ov_ygf_dn8, locals.var_sp_ov_ygf_dn9,)
    }
};
        locals.var_sp_ov_ygf = assign23480_e23072;
        locals.var_sp_ov_ygf_dn4 = assign23480_e23072_d_n4;
        locals.var_sp_ov_ygf_dn6 = assign23480_e23072_d_n6;
        locals.var_sp_ov_ygf_dn7 = assign23480_e23072_d_n7;
        locals.var_sp_ov_ygf_dn8 = assign23480_e23072_d_n8;
        locals.var_sp_ov_ygf_dn9 = assign23480_e23072_d_n9;
        locals.var_sp_ov_ygf_rv = 0.0;

        let (assign23490_e23085, assign23490_e23085_d_n4, assign23490_e23085_d_n6, assign23490_e23085_d_n7, assign23490_e23085_d_n8, assign23490_e23085_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign23490_e23081: f64 = (1.25 * locals.var_sp_ov_ygf);
        let assign23490_e23083: f64 = (assign23490_e23081 * locals.var_inv_xi_ov);
        (assign23490_e23083, (((1.25 * locals.var_sp_ov_ygf_dn4) * locals.var_inv_xi_ov) + (assign23490_e23081 * locals.var_inv_xi_ov_dn4)), (((1.25 * locals.var_sp_ov_ygf_dn6) * locals.var_inv_xi_ov) + (assign23490_e23081 * locals.var_inv_xi_ov_dn6)), (((1.25 * locals.var_sp_ov_ygf_dn7) * locals.var_inv_xi_ov) + (assign23490_e23081 * locals.var_inv_xi_ov_dn7)), (((1.25 * locals.var_sp_ov_ygf_dn8) * locals.var_inv_xi_ov) + (assign23490_e23081 * locals.var_inv_xi_ov_dn8)), (((1.25 * locals.var_sp_ov_ygf_dn9) * locals.var_inv_xi_ov) + (assign23490_e23081 * locals.var_inv_xi_ov_dn9)),)
    } else {
        (locals.var_sp_ov_z, locals.var_sp_ov_z_dn4, locals.var_sp_ov_z_dn6, locals.var_sp_ov_z_dn7, locals.var_sp_ov_z_dn8, locals.var_sp_ov_z_dn9,)
    }
};
        locals.var_sp_ov_z = assign23490_e23085;
        locals.var_sp_ov_z_dn4 = assign23490_e23085_d_n4;
        locals.var_sp_ov_z_dn6 = assign23490_e23085_d_n6;
        locals.var_sp_ov_z_dn7 = assign23490_e23085_d_n7;
        locals.var_sp_ov_z_dn8 = assign23490_e23085_d_n8;
        locals.var_sp_ov_z_dn9 = assign23490_e23085_d_n9;
        locals.var_sp_ov_z_rv = 0.0;

        let (assign23500_e23109, assign23500_e23109_d_n4, assign23500_e23109_d_n6, assign23500_e23109_d_n7, assign23500_e23109_d_n8, assign23500_e23109_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign23500_e23095: f64 = (locals.var_sp_ov_z + 10.0);
        let assign23500_e23098: f64 = (locals.var_sp_ov_z - 6.0);
        let assign23500_e23101: f64 = (locals.var_sp_ov_z - 6.0);
        let assign23500_e23102: f64 = (assign23500_e23098 * assign23500_e23101);
        let assign23500_e23104: f64 = (assign23500_e23102 + 64.0);
        let assign23500_e23105: f64 = (assign23500_e23104).sqrt();
        let assign23500_e23106: f64 = (assign23500_e23095 - assign23500_e23105);
        let assign23500_e23107: f64 = (0.5 * assign23500_e23106);
        (assign23500_e23107, (0.5 * (locals.var_sp_ov_z_dn4 - (((locals.var_sp_ov_z_dn4 * assign23500_e23101) + (assign23500_e23098 * locals.var_sp_ov_z_dn4)) / (2.0 * assign23500_e23105)))), (0.5 * (locals.var_sp_ov_z_dn6 - (((locals.var_sp_ov_z_dn6 * assign23500_e23101) + (assign23500_e23098 * locals.var_sp_ov_z_dn6)) / (2.0 * assign23500_e23105)))), (0.5 * (locals.var_sp_ov_z_dn7 - (((locals.var_sp_ov_z_dn7 * assign23500_e23101) + (assign23500_e23098 * locals.var_sp_ov_z_dn7)) / (2.0 * assign23500_e23105)))), (0.5 * (locals.var_sp_ov_z_dn8 - (((locals.var_sp_ov_z_dn8 * assign23500_e23101) + (assign23500_e23098 * locals.var_sp_ov_z_dn8)) / (2.0 * assign23500_e23105)))), (0.5 * (locals.var_sp_ov_z_dn9 - (((locals.var_sp_ov_z_dn9 * assign23500_e23101) + (assign23500_e23098 * locals.var_sp_ov_z_dn9)) / (2.0 * assign23500_e23105)))),)
    } else {
        (locals.var_sp_ov_eta, locals.var_sp_ov_eta_dn4, locals.var_sp_ov_eta_dn6, locals.var_sp_ov_eta_dn7, locals.var_sp_ov_eta_dn8, locals.var_sp_ov_eta_dn9,)
    }
};
        locals.var_sp_ov_eta = assign23500_e23109;
        locals.var_sp_ov_eta_dn4 = assign23500_e23109_d_n4;
        locals.var_sp_ov_eta_dn6 = assign23500_e23109_d_n6;
        locals.var_sp_ov_eta_dn7 = assign23500_e23109_d_n7;
        locals.var_sp_ov_eta_dn8 = assign23500_e23109_d_n8;
        locals.var_sp_ov_eta_dn9 = assign23500_e23109_d_n9;
        locals.var_sp_ov_eta_rv = 0.0;

        let (assign23510_e23130, assign23510_e23130_d_n4, assign23510_e23130_d_n6, assign23510_e23130_d_n7, assign23510_e23130_d_n8, assign23510_e23130_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign23510_e23118: f64 = (locals.var_sp_ov_ygf - locals.var_sp_ov_eta);
        let assign23510_e23121: f64 = (locals.var_sp_ov_ygf - locals.var_sp_ov_eta);
        let assign23510_e23122: f64 = (assign23510_e23118 * assign23510_e23121);
        let assign23510_e23126: f64 = (locals.var_sp_ov_eta + 1.0);
        let assign23510_e23127: f64 = (locals.var_gov2 * assign23510_e23126);
        let assign23510_e23128: f64 = (assign23510_e23122 + assign23510_e23127);
        (assign23510_e23128, ((((locals.var_sp_ov_ygf_dn4 - locals.var_sp_ov_eta_dn4) * assign23510_e23121) + (assign23510_e23118 * (locals.var_sp_ov_ygf_dn4 - locals.var_sp_ov_eta_dn4))) + ((locals.var_gov2_dn4 * assign23510_e23126) + (locals.var_gov2 * locals.var_sp_ov_eta_dn4))), ((((locals.var_sp_ov_ygf_dn6 - locals.var_sp_ov_eta_dn6) * assign23510_e23121) + (assign23510_e23118 * (locals.var_sp_ov_ygf_dn6 - locals.var_sp_ov_eta_dn6))) + ((locals.var_gov2_dn6 * assign23510_e23126) + (locals.var_gov2 * locals.var_sp_ov_eta_dn6))), ((((locals.var_sp_ov_ygf_dn7 - locals.var_sp_ov_eta_dn7) * assign23510_e23121) + (assign23510_e23118 * (locals.var_sp_ov_ygf_dn7 - locals.var_sp_ov_eta_dn7))) + ((locals.var_gov2_dn7 * assign23510_e23126) + (locals.var_gov2 * locals.var_sp_ov_eta_dn7))), ((((locals.var_sp_ov_ygf_dn8 - locals.var_sp_ov_eta_dn8) * assign23510_e23121) + (assign23510_e23118 * (locals.var_sp_ov_ygf_dn8 - locals.var_sp_ov_eta_dn8))) + ((locals.var_gov2_dn8 * assign23510_e23126) + (locals.var_gov2 * locals.var_sp_ov_eta_dn8))), ((((locals.var_sp_ov_ygf_dn9 - locals.var_sp_ov_eta_dn9) * assign23510_e23121) + (assign23510_e23118 * (locals.var_sp_ov_ygf_dn9 - locals.var_sp_ov_eta_dn9))) + ((locals.var_gov2_dn9 * assign23510_e23126) + (locals.var_gov2 * locals.var_sp_ov_eta_dn9))),)
    } else {
        (locals.var_sp_ov_a, locals.var_sp_ov_a_dn4, locals.var_sp_ov_a_dn6, locals.var_sp_ov_a_dn7, locals.var_sp_ov_a_dn8, locals.var_sp_ov_a_dn9,)
    }
};
        locals.var_sp_ov_a = assign23510_e23130;
        locals.var_sp_ov_a_dn4 = assign23510_e23130_d_n4;
        locals.var_sp_ov_a_dn6 = assign23510_e23130_d_n6;
        locals.var_sp_ov_a_dn7 = assign23510_e23130_d_n7;
        locals.var_sp_ov_a_dn8 = assign23510_e23130_d_n8;
        locals.var_sp_ov_a_dn9 = assign23510_e23130_d_n9;
        locals.var_sp_ov_a_rv = 0.0;

        let (assign23520_e23145, assign23520_e23145_d_n4, assign23520_e23145_d_n6, assign23520_e23145_d_n7, assign23520_e23145_d_n8, assign23520_e23145_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign23520_e23140: f64 = (locals.var_sp_ov_ygf - locals.var_sp_ov_eta);
        let assign23520_e23141: f64 = (2.0 * assign23520_e23140);
        let assign23520_e23143: f64 = (assign23520_e23141 - locals.var_gov2);
        (assign23520_e23143, ((2.0 * (locals.var_sp_ov_ygf_dn4 - locals.var_sp_ov_eta_dn4)) - locals.var_gov2_dn4), ((2.0 * (locals.var_sp_ov_ygf_dn6 - locals.var_sp_ov_eta_dn6)) - locals.var_gov2_dn6), ((2.0 * (locals.var_sp_ov_ygf_dn7 - locals.var_sp_ov_eta_dn7)) - locals.var_gov2_dn7), ((2.0 * (locals.var_sp_ov_ygf_dn8 - locals.var_sp_ov_eta_dn8)) - locals.var_gov2_dn8), ((2.0 * (locals.var_sp_ov_ygf_dn9 - locals.var_sp_ov_eta_dn9)) - locals.var_gov2_dn9),)
    } else {
        (locals.var_sp_ov_c, locals.var_sp_ov_c_dn4, locals.var_sp_ov_c_dn6, locals.var_sp_ov_c_dn7, locals.var_sp_ov_c_dn8, locals.var_sp_ov_c_dn9,)
    }
};
        locals.var_sp_ov_c = assign23520_e23145;
        locals.var_sp_ov_c_dn4 = assign23520_e23145_d_n4;
        locals.var_sp_ov_c_dn6 = assign23520_e23145_d_n6;
        locals.var_sp_ov_c_dn7 = assign23520_e23145_d_n7;
        locals.var_sp_ov_c_dn8 = assign23520_e23145_d_n8;
        locals.var_sp_ov_c_dn9 = assign23520_e23145_d_n9;
        locals.var_sp_ov_c_rv = 0.0;

        let (assign23530_e23159, assign23530_e23159_d_n4, assign23530_e23159_d_n6, assign23530_e23159_d_n7, assign23530_e23159_d_n8, assign23530_e23159_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign23530_e23154: f64 = (locals.var_sp_ov_a / locals.var_gov2);
        let assign23530_e23155: f64 = (assign23530_e23154).ln();
        let assign23530_e23157: f64 = (assign23530_e23155 - locals.var_sp_ov_eta);
        (assign23530_e23157, (((((locals.var_sp_ov_a_dn4 * locals.var_gov2) - (locals.var_sp_ov_a * locals.var_gov2_dn4)) / (locals.var_gov2 * locals.var_gov2)) / assign23530_e23154) - locals.var_sp_ov_eta_dn4), (((((locals.var_sp_ov_a_dn6 * locals.var_gov2) - (locals.var_sp_ov_a * locals.var_gov2_dn6)) / (locals.var_gov2 * locals.var_gov2)) / assign23530_e23154) - locals.var_sp_ov_eta_dn6), (((((locals.var_sp_ov_a_dn7 * locals.var_gov2) - (locals.var_sp_ov_a * locals.var_gov2_dn7)) / (locals.var_gov2 * locals.var_gov2)) / assign23530_e23154) - locals.var_sp_ov_eta_dn7), (((((locals.var_sp_ov_a_dn8 * locals.var_gov2) - (locals.var_sp_ov_a * locals.var_gov2_dn8)) / (locals.var_gov2 * locals.var_gov2)) / assign23530_e23154) - locals.var_sp_ov_eta_dn8), (((((locals.var_sp_ov_a_dn9 * locals.var_gov2) - (locals.var_sp_ov_a * locals.var_gov2_dn9)) / (locals.var_gov2 * locals.var_gov2)) / assign23530_e23154) - locals.var_sp_ov_eta_dn9),)
    } else {
        (locals.var_sp_ov_tau, locals.var_sp_ov_tau_dn4, locals.var_sp_ov_tau_dn6, locals.var_sp_ov_tau_dn7, locals.var_sp_ov_tau_dn8, locals.var_sp_ov_tau_dn9,)
    }
};
        locals.var_sp_ov_tau = assign23530_e23159;
        locals.var_sp_ov_tau_dn4 = assign23530_e23159_d_n4;
        locals.var_sp_ov_tau_dn6 = assign23530_e23159_d_n6;
        locals.var_sp_ov_tau_dn7 = assign23530_e23159_d_n7;
        locals.var_sp_ov_tau_dn8 = assign23530_e23159_d_n8;
        locals.var_sp_ov_tau_dn9 = assign23530_e23159_d_n9;
        locals.var_sp_ov_tau_rv = 0.0;

        let (assign23540_e23170, assign23540_e23170_d_n4, assign23540_e23170_d_n6, assign23540_e23170_d_n7, assign23540_e23170_d_n8, assign23540_e23170_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign23540_e23168: f64 = (locals.var_sp_ov_a + locals.var_sp_ov_c);
        (assign23540_e23168, (locals.var_sp_ov_a_dn4 + locals.var_sp_ov_c_dn4), (locals.var_sp_ov_a_dn6 + locals.var_sp_ov_c_dn6), (locals.var_sp_ov_a_dn7 + locals.var_sp_ov_c_dn7), (locals.var_sp_ov_a_dn8 + locals.var_sp_ov_c_dn8), (locals.var_sp_ov_a_dn9 + locals.var_sp_ov_c_dn9),)
    } else {
        (locals.var_sp_ov_nu, locals.var_sp_ov_nu_dn4, locals.var_sp_ov_nu_dn6, locals.var_sp_ov_nu_dn7, locals.var_sp_ov_nu_dn8, locals.var_sp_ov_nu_dn9,)
    }
};
        locals.var_sp_ov_nu = assign23540_e23170;
        locals.var_sp_ov_nu_dn4 = assign23540_e23170_d_n4;
        locals.var_sp_ov_nu_dn6 = assign23540_e23170_d_n6;
        locals.var_sp_ov_nu_dn7 = assign23540_e23170_d_n7;
        locals.var_sp_ov_nu_dn8 = assign23540_e23170_d_n8;
        locals.var_sp_ov_nu_dn9 = assign23540_e23170_d_n9;
        locals.var_sp_ov_nu_rv = 0.0;

        let (assign23550_e23191, assign23550_e23191_d_n4, assign23550_e23191_d_n6, assign23550_e23191_d_n7, assign23550_e23191_d_n8, assign23550_e23191_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign23550_e23179: f64 = (locals.var_sp_ov_nu * locals.var_sp_ov_nu);
        let assign23550_e23183: f64 = (0.5 * locals.var_sp_ov_c);
        let assign23550_e23185: f64 = (assign23550_e23183 * locals.var_sp_ov_c);
        let assign23550_e23187: f64 = (assign23550_e23185 - locals.var_sp_ov_a);
        let assign23550_e23188: f64 = (locals.var_sp_ov_tau * assign23550_e23187);
        let assign23550_e23189: f64 = (assign23550_e23179 + assign23550_e23188);
        (assign23550_e23189, (((locals.var_sp_ov_nu_dn4 * locals.var_sp_ov_nu) + (locals.var_sp_ov_nu * locals.var_sp_ov_nu_dn4)) + ((locals.var_sp_ov_tau_dn4 * assign23550_e23187) + (locals.var_sp_ov_tau * ((((0.5 * locals.var_sp_ov_c_dn4) * locals.var_sp_ov_c) + (assign23550_e23183 * locals.var_sp_ov_c_dn4)) - locals.var_sp_ov_a_dn4)))), (((locals.var_sp_ov_nu_dn6 * locals.var_sp_ov_nu) + (locals.var_sp_ov_nu * locals.var_sp_ov_nu_dn6)) + ((locals.var_sp_ov_tau_dn6 * assign23550_e23187) + (locals.var_sp_ov_tau * ((((0.5 * locals.var_sp_ov_c_dn6) * locals.var_sp_ov_c) + (assign23550_e23183 * locals.var_sp_ov_c_dn6)) - locals.var_sp_ov_a_dn6)))), (((locals.var_sp_ov_nu_dn7 * locals.var_sp_ov_nu) + (locals.var_sp_ov_nu * locals.var_sp_ov_nu_dn7)) + ((locals.var_sp_ov_tau_dn7 * assign23550_e23187) + (locals.var_sp_ov_tau * ((((0.5 * locals.var_sp_ov_c_dn7) * locals.var_sp_ov_c) + (assign23550_e23183 * locals.var_sp_ov_c_dn7)) - locals.var_sp_ov_a_dn7)))), (((locals.var_sp_ov_nu_dn8 * locals.var_sp_ov_nu) + (locals.var_sp_ov_nu * locals.var_sp_ov_nu_dn8)) + ((locals.var_sp_ov_tau_dn8 * assign23550_e23187) + (locals.var_sp_ov_tau * ((((0.5 * locals.var_sp_ov_c_dn8) * locals.var_sp_ov_c) + (assign23550_e23183 * locals.var_sp_ov_c_dn8)) - locals.var_sp_ov_a_dn8)))), (((locals.var_sp_ov_nu_dn9 * locals.var_sp_ov_nu) + (locals.var_sp_ov_nu * locals.var_sp_ov_nu_dn9)) + ((locals.var_sp_ov_tau_dn9 * assign23550_e23187) + (locals.var_sp_ov_tau * ((((0.5 * locals.var_sp_ov_c_dn9) * locals.var_sp_ov_c) + (assign23550_e23183 * locals.var_sp_ov_c_dn9)) - locals.var_sp_ov_a_dn9)))),)
    } else {
        (locals.var_sp_ov_mutau, locals.var_sp_ov_mutau_dn4, locals.var_sp_ov_mutau_dn6, locals.var_sp_ov_mutau_dn7, locals.var_sp_ov_mutau_dn8, locals.var_sp_ov_mutau_dn9,)
    }
};
        locals.var_sp_ov_mutau = assign23550_e23191;
        locals.var_sp_ov_mutau_dn4 = assign23550_e23191_d_n4;
        locals.var_sp_ov_mutau_dn6 = assign23550_e23191_d_n6;
        locals.var_sp_ov_mutau_dn7 = assign23550_e23191_d_n7;
        locals.var_sp_ov_mutau_dn8 = assign23550_e23191_d_n8;
        locals.var_sp_ov_mutau_dn9 = assign23550_e23191_d_n9;
        locals.var_sp_ov_mutau_rv = 0.0;

        let (assign23560_e23218, assign23560_e23218_d_n4, assign23560_e23218_d_n6, assign23560_e23218_d_n7, assign23560_e23218_d_n8, assign23560_e23218_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign23560_e23201: f64 = (locals.var_sp_ov_nu / locals.var_sp_ov_mutau);
        let assign23560_e23203: f64 = (assign23560_e23201 * locals.var_sp_ov_tau);
        let assign23560_e23205: f64 = (assign23560_e23203 * locals.var_sp_ov_tau);
        let assign23560_e23207: f64 = (assign23560_e23205 * locals.var_sp_ov_c);
        let assign23560_e23210: f64 = (locals.var_sp_ov_c * locals.var_sp_ov_c);
        let assign23560_e23212: f64 = (assign23560_e23210 * 0.3333333333333);
        let assign23560_e23214: f64 = (assign23560_e23212 - locals.var_sp_ov_a);
        let assign23560_e23215: f64 = (assign23560_e23207 * assign23560_e23214);
        let assign23560_e23216: f64 = (locals.var_sp_ov_mutau + assign23560_e23215);
        (assign23560_e23216, (locals.var_sp_ov_mutau_dn4 + (((((((((((locals.var_sp_ov_nu_dn4 * locals.var_sp_ov_mutau) - (locals.var_sp_ov_nu * locals.var_sp_ov_mutau_dn4)) / (locals.var_sp_ov_mutau * locals.var_sp_ov_mutau)) * locals.var_sp_ov_tau) + (assign23560_e23201 * locals.var_sp_ov_tau_dn4)) * locals.var_sp_ov_tau) + (assign23560_e23203 * locals.var_sp_ov_tau_dn4)) * locals.var_sp_ov_c) + (assign23560_e23205 * locals.var_sp_ov_c_dn4)) * assign23560_e23214) + (assign23560_e23207 * ((((locals.var_sp_ov_c_dn4 * locals.var_sp_ov_c) + (locals.var_sp_ov_c * locals.var_sp_ov_c_dn4)) * 0.3333333333333) - locals.var_sp_ov_a_dn4)))), (locals.var_sp_ov_mutau_dn6 + (((((((((((locals.var_sp_ov_nu_dn6 * locals.var_sp_ov_mutau) - (locals.var_sp_ov_nu * locals.var_sp_ov_mutau_dn6)) / (locals.var_sp_ov_mutau * locals.var_sp_ov_mutau)) * locals.var_sp_ov_tau) + (assign23560_e23201 * locals.var_sp_ov_tau_dn6)) * locals.var_sp_ov_tau) + (assign23560_e23203 * locals.var_sp_ov_tau_dn6)) * locals.var_sp_ov_c) + (assign23560_e23205 * locals.var_sp_ov_c_dn6)) * assign23560_e23214) + (assign23560_e23207 * ((((locals.var_sp_ov_c_dn6 * locals.var_sp_ov_c) + (locals.var_sp_ov_c * locals.var_sp_ov_c_dn6)) * 0.3333333333333) - locals.var_sp_ov_a_dn6)))), (locals.var_sp_ov_mutau_dn7 + (((((((((((locals.var_sp_ov_nu_dn7 * locals.var_sp_ov_mutau) - (locals.var_sp_ov_nu * locals.var_sp_ov_mutau_dn7)) / (locals.var_sp_ov_mutau * locals.var_sp_ov_mutau)) * locals.var_sp_ov_tau) + (assign23560_e23201 * locals.var_sp_ov_tau_dn7)) * locals.var_sp_ov_tau) + (assign23560_e23203 * locals.var_sp_ov_tau_dn7)) * locals.var_sp_ov_c) + (assign23560_e23205 * locals.var_sp_ov_c_dn7)) * assign23560_e23214) + (assign23560_e23207 * ((((locals.var_sp_ov_c_dn7 * locals.var_sp_ov_c) + (locals.var_sp_ov_c * locals.var_sp_ov_c_dn7)) * 0.3333333333333) - locals.var_sp_ov_a_dn7)))), (locals.var_sp_ov_mutau_dn8 + (((((((((((locals.var_sp_ov_nu_dn8 * locals.var_sp_ov_mutau) - (locals.var_sp_ov_nu * locals.var_sp_ov_mutau_dn8)) / (locals.var_sp_ov_mutau * locals.var_sp_ov_mutau)) * locals.var_sp_ov_tau) + (assign23560_e23201 * locals.var_sp_ov_tau_dn8)) * locals.var_sp_ov_tau) + (assign23560_e23203 * locals.var_sp_ov_tau_dn8)) * locals.var_sp_ov_c) + (assign23560_e23205 * locals.var_sp_ov_c_dn8)) * assign23560_e23214) + (assign23560_e23207 * ((((locals.var_sp_ov_c_dn8 * locals.var_sp_ov_c) + (locals.var_sp_ov_c * locals.var_sp_ov_c_dn8)) * 0.3333333333333) - locals.var_sp_ov_a_dn8)))), (locals.var_sp_ov_mutau_dn9 + (((((((((((locals.var_sp_ov_nu_dn9 * locals.var_sp_ov_mutau) - (locals.var_sp_ov_nu * locals.var_sp_ov_mutau_dn9)) / (locals.var_sp_ov_mutau * locals.var_sp_ov_mutau)) * locals.var_sp_ov_tau) + (assign23560_e23201 * locals.var_sp_ov_tau_dn9)) * locals.var_sp_ov_tau) + (assign23560_e23203 * locals.var_sp_ov_tau_dn9)) * locals.var_sp_ov_c) + (assign23560_e23205 * locals.var_sp_ov_c_dn9)) * assign23560_e23214) + (assign23560_e23207 * ((((locals.var_sp_ov_c_dn9 * locals.var_sp_ov_c) + (locals.var_sp_ov_c * locals.var_sp_ov_c_dn9)) * 0.3333333333333) - locals.var_sp_ov_a_dn9)))),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign23560_e23218;
        locals.var_sp_ov_temp_dn4 = assign23560_e23218_d_n4;
        locals.var_sp_ov_temp_dn6 = assign23560_e23218_d_n6;
        locals.var_sp_ov_temp_dn7 = assign23560_e23218_d_n7;
        locals.var_sp_ov_temp_dn8 = assign23560_e23218_d_n8;
        locals.var_sp_ov_temp_dn9 = assign23560_e23218_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let (assign23570_e23235, assign23570_e23235_d_n4, assign23570_e23235_d_n6, assign23570_e23235_d_n7, assign23570_e23235_d_n8, assign23570_e23235_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign23570_e23228: f64 = (locals.var_sp_ov_a * locals.var_sp_ov_nu);
        let assign23570_e23230: f64 = (assign23570_e23228 * locals.var_sp_ov_tau);
        let assign23570_e23232: f64 = (assign23570_e23230 / locals.var_sp_ov_temp);
        let assign23570_e23233: f64 = (locals.var_sp_ov_eta + assign23570_e23232);
        (assign23570_e23233, (locals.var_sp_ov_eta_dn4 + (((((((locals.var_sp_ov_a_dn4 * locals.var_sp_ov_nu) + (locals.var_sp_ov_a * locals.var_sp_ov_nu_dn4)) * locals.var_sp_ov_tau) + (assign23570_e23228 * locals.var_sp_ov_tau_dn4)) * locals.var_sp_ov_temp) - (assign23570_e23230 * locals.var_sp_ov_temp_dn4)) / (locals.var_sp_ov_temp * locals.var_sp_ov_temp))), (locals.var_sp_ov_eta_dn6 + (((((((locals.var_sp_ov_a_dn6 * locals.var_sp_ov_nu) + (locals.var_sp_ov_a * locals.var_sp_ov_nu_dn6)) * locals.var_sp_ov_tau) + (assign23570_e23228 * locals.var_sp_ov_tau_dn6)) * locals.var_sp_ov_temp) - (assign23570_e23230 * locals.var_sp_ov_temp_dn6)) / (locals.var_sp_ov_temp * locals.var_sp_ov_temp))), (locals.var_sp_ov_eta_dn7 + (((((((locals.var_sp_ov_a_dn7 * locals.var_sp_ov_nu) + (locals.var_sp_ov_a * locals.var_sp_ov_nu_dn7)) * locals.var_sp_ov_tau) + (assign23570_e23228 * locals.var_sp_ov_tau_dn7)) * locals.var_sp_ov_temp) - (assign23570_e23230 * locals.var_sp_ov_temp_dn7)) / (locals.var_sp_ov_temp * locals.var_sp_ov_temp))), (locals.var_sp_ov_eta_dn8 + (((((((locals.var_sp_ov_a_dn8 * locals.var_sp_ov_nu) + (locals.var_sp_ov_a * locals.var_sp_ov_nu_dn8)) * locals.var_sp_ov_tau) + (assign23570_e23228 * locals.var_sp_ov_tau_dn8)) * locals.var_sp_ov_temp) - (assign23570_e23230 * locals.var_sp_ov_temp_dn8)) / (locals.var_sp_ov_temp * locals.var_sp_ov_temp))), (locals.var_sp_ov_eta_dn9 + (((((((locals.var_sp_ov_a_dn9 * locals.var_sp_ov_nu) + (locals.var_sp_ov_a * locals.var_sp_ov_nu_dn9)) * locals.var_sp_ov_tau) + (assign23570_e23228 * locals.var_sp_ov_tau_dn9)) * locals.var_sp_ov_temp) - (assign23570_e23230 * locals.var_sp_ov_temp_dn9)) / (locals.var_sp_ov_temp * locals.var_sp_ov_temp))),)
    } else {
        (locals.var_sp_ov_y0, locals.var_sp_ov_y0_dn4, locals.var_sp_ov_y0_dn6, locals.var_sp_ov_y0_dn7, locals.var_sp_ov_y0_dn8, locals.var_sp_ov_y0_dn9,)
    }
};
        locals.var_sp_ov_y0 = assign23570_e23235;
        locals.var_sp_ov_y0_dn4 = assign23570_e23235_d_n4;
        locals.var_sp_ov_y0_dn6 = assign23570_e23235_d_n6;
        locals.var_sp_ov_y0_dn7 = assign23570_e23235_d_n7;
        locals.var_sp_ov_y0_dn8 = assign23570_e23235_d_n8;
        locals.var_sp_ov_y0_dn9 = assign23570_e23235_d_n9;
        locals.var_sp_ov_y0_rv = 0.0;

        let assign23580_e23237: f64 = (locals.var_sp_ov_y0).abs();
        let assign23580_e23239: f64 = if assign23580_e23237 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard688 = assign23580_e23239;
        locals.var_guard688_rv = 0.0;

        let (assign23590_e23251, assign23590_e23251_d_n4, assign23590_e23251_d_n6, assign23590_e23251_d_n7, assign23590_e23251_d_n8, assign23590_e23251_d_n9,) = {
    if ((((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) && (locals.var_guard688 != 0.0)) {
        let assign23590_e23249: f64 = (locals.var_sp_ov_y0).exp();
        (assign23590_e23249, (assign23590_e23249 * locals.var_sp_ov_y0_dn4), (assign23590_e23249 * locals.var_sp_ov_y0_dn6), (assign23590_e23249 * locals.var_sp_ov_y0_dn7), (assign23590_e23249 * locals.var_sp_ov_y0_dn8), (assign23590_e23249 * locals.var_sp_ov_y0_dn9),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign23590_e23251;
        locals.var_sp_ov_d0_dn4 = assign23590_e23251_d_n4;
        locals.var_sp_ov_d0_dn6 = assign23590_e23251_d_n6;
        locals.var_sp_ov_d0_dn7 = assign23590_e23251_d_n7;
        locals.var_sp_ov_d0_dn8 = assign23590_e23251_d_n8;
        locals.var_sp_ov_d0_dn9 = assign23590_e23251_d_n9;
        locals.var_sp_ov_d0_rv = 0.0;

        let assign23600_e23254: f64 = (-80.0);
        let assign23600_e23255: f64 = if locals.var_sp_ov_y0 < assign23600_e23254 { 1.0 } else { 0.0 };
        locals.var_guard689 = assign23600_e23255;
        locals.var_guard689_rv = 0.0;

        let (assign23610_e23294, assign23610_e23294_d_n4, assign23610_e23294_d_n6, assign23610_e23294_d_n7, assign23610_e23294_d_n8, assign23610_e23294_d_n9,) = {
    if (((((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) && (locals.var_guard688 == 0.0)) && (locals.var_guard689 != 0.0)) {
        let assign23610_e23270: f64 = (-locals.var_sp_ov_y0);
        let assign23610_e23272: f64 = (assign23610_e23270 - 80.0);
        let assign23610_e23276: f64 = (-locals.var_sp_ov_y0);
        let assign23610_e23278: f64 = (assign23610_e23276 - 80.0);
        let assign23610_e23279: f64 = (0.5 * assign23610_e23278);
        let assign23610_e23282: f64 = (-locals.var_sp_ov_y0);
        let assign23610_e23284: f64 = (assign23610_e23282 - 80.0);
        let assign23610_e23286: f64 = (assign23610_e23284 * 0.3333333333333);
        let assign23610_e23287: f64 = (1.0 + assign23610_e23286);
        let assign23610_e23288: f64 = (assign23610_e23279 * assign23610_e23287);
        let assign23610_e23289: f64 = (1.0 + assign23610_e23288);
        let assign23610_e23290: f64 = (assign23610_e23272 * assign23610_e23289);
        let assign23610_e23291: f64 = (1.0 + assign23610_e23290);
        let assign23610_e23292: f64 = (1.80485e-35 / assign23610_e23291);
        (assign23610_e23292, (-((1.80485e-35 * (((-locals.var_sp_ov_y0_dn4) * assign23610_e23289) + (assign23610_e23272 * (((0.5 * (-locals.var_sp_ov_y0_dn4)) * assign23610_e23287) + (assign23610_e23279 * ((-locals.var_sp_ov_y0_dn4) * 0.3333333333333)))))) / (assign23610_e23291 * assign23610_e23291))), (-((1.80485e-35 * (((-locals.var_sp_ov_y0_dn6) * assign23610_e23289) + (assign23610_e23272 * (((0.5 * (-locals.var_sp_ov_y0_dn6)) * assign23610_e23287) + (assign23610_e23279 * ((-locals.var_sp_ov_y0_dn6) * 0.3333333333333)))))) / (assign23610_e23291 * assign23610_e23291))), (-((1.80485e-35 * (((-locals.var_sp_ov_y0_dn7) * assign23610_e23289) + (assign23610_e23272 * (((0.5 * (-locals.var_sp_ov_y0_dn7)) * assign23610_e23287) + (assign23610_e23279 * ((-locals.var_sp_ov_y0_dn7) * 0.3333333333333)))))) / (assign23610_e23291 * assign23610_e23291))), (-((1.80485e-35 * (((-locals.var_sp_ov_y0_dn8) * assign23610_e23289) + (assign23610_e23272 * (((0.5 * (-locals.var_sp_ov_y0_dn8)) * assign23610_e23287) + (assign23610_e23279 * ((-locals.var_sp_ov_y0_dn8) * 0.3333333333333)))))) / (assign23610_e23291 * assign23610_e23291))), (-((1.80485e-35 * (((-locals.var_sp_ov_y0_dn9) * assign23610_e23289) + (assign23610_e23272 * (((0.5 * (-locals.var_sp_ov_y0_dn9)) * assign23610_e23287) + (assign23610_e23279 * ((-locals.var_sp_ov_y0_dn9) * 0.3333333333333)))))) / (assign23610_e23291 * assign23610_e23291))),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign23610_e23294;
        locals.var_sp_ov_d0_dn4 = assign23610_e23294_d_n4;
        locals.var_sp_ov_d0_dn6 = assign23610_e23294_d_n6;
        locals.var_sp_ov_d0_dn7 = assign23610_e23294_d_n7;
        locals.var_sp_ov_d0_dn8 = assign23610_e23294_d_n8;
        locals.var_sp_ov_d0_dn9 = assign23610_e23294_d_n9;
        locals.var_sp_ov_d0_rv = 0.0;

        let (assign23620_e23331, assign23620_e23331_d_n4, assign23620_e23331_d_n6, assign23620_e23331_d_n7, assign23620_e23331_d_n8, assign23620_e23331_d_n9,) = {
    if (((((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) && (locals.var_guard688 == 0.0)) && (locals.var_guard689 == 0.0)) {
        let assign23620_e23311: f64 = (locals.var_sp_ov_y0 - 80.0);
        let assign23620_e23316: f64 = (locals.var_sp_ov_y0 - 80.0);
        let assign23620_e23317: f64 = (0.5 * assign23620_e23316);
        let assign23620_e23321: f64 = (locals.var_sp_ov_y0 - 80.0);
        let assign23620_e23323: f64 = (assign23620_e23321 * 0.3333333333333);
        let assign23620_e23324: f64 = (1.0 + assign23620_e23323);
        let assign23620_e23325: f64 = (assign23620_e23317 * assign23620_e23324);
        let assign23620_e23326: f64 = (1.0 + assign23620_e23325);
        let assign23620_e23327: f64 = (assign23620_e23311 * assign23620_e23326);
        let assign23620_e23328: f64 = (1.0 + assign23620_e23327);
        let assign23620_e23329: f64 = (5.54062e34 * assign23620_e23328);
        (assign23620_e23329, (5.54062e34 * ((locals.var_sp_ov_y0_dn4 * assign23620_e23326) + (assign23620_e23311 * (((0.5 * locals.var_sp_ov_y0_dn4) * assign23620_e23324) + (assign23620_e23317 * (locals.var_sp_ov_y0_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_sp_ov_y0_dn6 * assign23620_e23326) + (assign23620_e23311 * (((0.5 * locals.var_sp_ov_y0_dn6) * assign23620_e23324) + (assign23620_e23317 * (locals.var_sp_ov_y0_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_sp_ov_y0_dn7 * assign23620_e23326) + (assign23620_e23311 * (((0.5 * locals.var_sp_ov_y0_dn7) * assign23620_e23324) + (assign23620_e23317 * (locals.var_sp_ov_y0_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_sp_ov_y0_dn8 * assign23620_e23326) + (assign23620_e23311 * (((0.5 * locals.var_sp_ov_y0_dn8) * assign23620_e23324) + (assign23620_e23317 * (locals.var_sp_ov_y0_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_sp_ov_y0_dn9 * assign23620_e23326) + (assign23620_e23311 * (((0.5 * locals.var_sp_ov_y0_dn9) * assign23620_e23324) + (assign23620_e23317 * (locals.var_sp_ov_y0_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign23620_e23331;
        locals.var_sp_ov_d0_dn4 = assign23620_e23331_d_n4;
        locals.var_sp_ov_d0_dn6 = assign23620_e23331_d_n6;
        locals.var_sp_ov_d0_dn7 = assign23620_e23331_d_n7;
        locals.var_sp_ov_d0_dn8 = assign23620_e23331_d_n8;
        locals.var_sp_ov_d0_dn9 = assign23620_e23331_d_n9;
        locals.var_sp_ov_d0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_64(
        locals: &mut StampLocals,
    ) {
        let (assign23630_e23342, assign23630_e23342_d_n4, assign23630_e23342_d_n6, assign23630_e23342_d_n7, assign23630_e23342_d_n8, assign23630_e23342_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign23630_e23340: f64 = (locals.var_sp_ov_ygf - locals.var_sp_ov_y0);
        (assign23630_e23340, (locals.var_sp_ov_ygf_dn4 - locals.var_sp_ov_y0_dn4), (locals.var_sp_ov_ygf_dn6 - locals.var_sp_ov_y0_dn6), (locals.var_sp_ov_ygf_dn7 - locals.var_sp_ov_y0_dn7), (locals.var_sp_ov_ygf_dn8 - locals.var_sp_ov_y0_dn8), (locals.var_sp_ov_ygf_dn9 - locals.var_sp_ov_y0_dn9),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign23630_e23342;
        locals.var_sp_ov_temp_dn4 = assign23630_e23342_d_n4;
        locals.var_sp_ov_temp_dn6 = assign23630_e23342_d_n6;
        locals.var_sp_ov_temp_dn7 = assign23630_e23342_d_n7;
        locals.var_sp_ov_temp_dn8 = assign23630_e23342_d_n8;
        locals.var_sp_ov_temp_dn9 = assign23630_e23342_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let (assign23640_e23359, assign23640_e23359_d_n4, assign23640_e23359_d_n6, assign23640_e23359_d_n7, assign23640_e23359_d_n8, assign23640_e23359_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign23640_e23351: f64 = (2.0 * locals.var_sp_ov_temp);
        let assign23640_e23355: f64 = (locals.var_sp_ov_d0 - 1.0);
        let assign23640_e23356: f64 = (locals.var_gov2 * assign23640_e23355);
        let assign23640_e23357: f64 = (assign23640_e23351 + assign23640_e23356);
        (assign23640_e23357, ((2.0 * locals.var_sp_ov_temp_dn4) + ((locals.var_gov2_dn4 * assign23640_e23355) + (locals.var_gov2 * locals.var_sp_ov_d0_dn4))), ((2.0 * locals.var_sp_ov_temp_dn6) + ((locals.var_gov2_dn6 * assign23640_e23355) + (locals.var_gov2 * locals.var_sp_ov_d0_dn6))), ((2.0 * locals.var_sp_ov_temp_dn7) + ((locals.var_gov2_dn7 * assign23640_e23355) + (locals.var_gov2 * locals.var_sp_ov_d0_dn7))), ((2.0 * locals.var_sp_ov_temp_dn8) + ((locals.var_gov2_dn8 * assign23640_e23355) + (locals.var_gov2 * locals.var_sp_ov_d0_dn8))), ((2.0 * locals.var_sp_ov_temp_dn9) + ((locals.var_gov2_dn9 * assign23640_e23355) + (locals.var_gov2 * locals.var_sp_ov_d0_dn9))),)
    } else {
        (locals.var_sp_ov_p, locals.var_sp_ov_p_dn4, locals.var_sp_ov_p_dn6, locals.var_sp_ov_p_dn7, locals.var_sp_ov_p_dn8, locals.var_sp_ov_p_dn9,)
    }
};
        locals.var_sp_ov_p = assign23640_e23359;
        locals.var_sp_ov_p_dn4 = assign23640_e23359_d_n4;
        locals.var_sp_ov_p_dn6 = assign23640_e23359_d_n6;
        locals.var_sp_ov_p_dn7 = assign23640_e23359_d_n7;
        locals.var_sp_ov_p_dn8 = assign23640_e23359_d_n8;
        locals.var_sp_ov_p_dn9 = assign23640_e23359_d_n9;
        locals.var_sp_ov_p_rv = 0.0;

        let (assign23650_e23378, assign23650_e23378_d_n4, assign23650_e23378_d_n6, assign23650_e23378_d_n7, assign23650_e23378_d_n8, assign23650_e23378_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign23650_e23368: f64 = (locals.var_sp_ov_temp * locals.var_sp_ov_temp);
        let assign23650_e23372: f64 = (locals.var_sp_ov_y0 + 1.0);
        let assign23650_e23374: f64 = (assign23650_e23372 - locals.var_sp_ov_d0);
        let assign23650_e23375: f64 = (locals.var_gov2 * assign23650_e23374);
        let assign23650_e23376: f64 = (assign23650_e23368 + assign23650_e23375);
        (assign23650_e23376, (((locals.var_sp_ov_temp_dn4 * locals.var_sp_ov_temp) + (locals.var_sp_ov_temp * locals.var_sp_ov_temp_dn4)) + ((locals.var_gov2_dn4 * assign23650_e23374) + (locals.var_gov2 * (locals.var_sp_ov_y0_dn4 - locals.var_sp_ov_d0_dn4)))), (((locals.var_sp_ov_temp_dn6 * locals.var_sp_ov_temp) + (locals.var_sp_ov_temp * locals.var_sp_ov_temp_dn6)) + ((locals.var_gov2_dn6 * assign23650_e23374) + (locals.var_gov2 * (locals.var_sp_ov_y0_dn6 - locals.var_sp_ov_d0_dn6)))), (((locals.var_sp_ov_temp_dn7 * locals.var_sp_ov_temp) + (locals.var_sp_ov_temp * locals.var_sp_ov_temp_dn7)) + ((locals.var_gov2_dn7 * assign23650_e23374) + (locals.var_gov2 * (locals.var_sp_ov_y0_dn7 - locals.var_sp_ov_d0_dn7)))), (((locals.var_sp_ov_temp_dn8 * locals.var_sp_ov_temp) + (locals.var_sp_ov_temp * locals.var_sp_ov_temp_dn8)) + ((locals.var_gov2_dn8 * assign23650_e23374) + (locals.var_gov2 * (locals.var_sp_ov_y0_dn8 - locals.var_sp_ov_d0_dn8)))), (((locals.var_sp_ov_temp_dn9 * locals.var_sp_ov_temp) + (locals.var_sp_ov_temp * locals.var_sp_ov_temp_dn9)) + ((locals.var_gov2_dn9 * assign23650_e23374) + (locals.var_gov2 * (locals.var_sp_ov_y0_dn9 - locals.var_sp_ov_d0_dn9)))),)
    } else {
        (locals.var_sp_ov_q, locals.var_sp_ov_q_dn4, locals.var_sp_ov_q_dn6, locals.var_sp_ov_q_dn7, locals.var_sp_ov_q_dn8, locals.var_sp_ov_q_dn9,)
    }
};
        locals.var_sp_ov_q = assign23650_e23378;
        locals.var_sp_ov_q_dn4 = assign23650_e23378_d_n4;
        locals.var_sp_ov_q_dn6 = assign23650_e23378_d_n6;
        locals.var_sp_ov_q_dn7 = assign23650_e23378_d_n7;
        locals.var_sp_ov_q_dn8 = assign23650_e23378_d_n8;
        locals.var_sp_ov_q_dn9 = assign23650_e23378_d_n9;
        locals.var_sp_ov_q_rv = 0.0;

        let (assign23660_e23393, assign23660_e23393_d_n4, assign23660_e23393_d_n6, assign23660_e23393_d_n7, assign23660_e23393_d_n8, assign23660_e23393_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign23660_e23388: f64 = (locals.var_gov2 * 0.5);
        let assign23660_e23390: f64 = (assign23660_e23388 * locals.var_sp_ov_d0);
        let assign23660_e23391: f64 = (1.0 - assign23660_e23390);
        (assign23660_e23391, (-(((locals.var_gov2_dn4 * 0.5) * locals.var_sp_ov_d0) + (assign23660_e23388 * locals.var_sp_ov_d0_dn4))), (-(((locals.var_gov2_dn6 * 0.5) * locals.var_sp_ov_d0) + (assign23660_e23388 * locals.var_sp_ov_d0_dn6))), (-(((locals.var_gov2_dn7 * 0.5) * locals.var_sp_ov_d0) + (assign23660_e23388 * locals.var_sp_ov_d0_dn7))), (-(((locals.var_gov2_dn8 * 0.5) * locals.var_sp_ov_d0) + (assign23660_e23388 * locals.var_sp_ov_d0_dn8))), (-(((locals.var_gov2_dn9 * 0.5) * locals.var_sp_ov_d0) + (assign23660_e23388 * locals.var_sp_ov_d0_dn9))),)
    } else {
        (locals.var_sp_ov_xi, locals.var_sp_ov_xi_dn4, locals.var_sp_ov_xi_dn6, locals.var_sp_ov_xi_dn7, locals.var_sp_ov_xi_dn8, locals.var_sp_ov_xi_dn9,)
    }
};
        locals.var_sp_ov_xi = assign23660_e23393;
        locals.var_sp_ov_xi_dn4 = assign23660_e23393_d_n4;
        locals.var_sp_ov_xi_dn6 = assign23660_e23393_d_n6;
        locals.var_sp_ov_xi_dn7 = assign23660_e23393_d_n7;
        locals.var_sp_ov_xi_dn8 = assign23660_e23393_d_n8;
        locals.var_sp_ov_xi_dn9 = assign23660_e23393_d_n9;
        locals.var_sp_ov_xi_rv = 0.0;

        let (assign23670_e23410, assign23670_e23410_d_n4, assign23670_e23410_d_n6, assign23670_e23410_d_n7, assign23670_e23410_d_n8, assign23670_e23410_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign23670_e23402: f64 = (locals.var_sp_ov_p * locals.var_sp_ov_p);
        let assign23670_e23406: f64 = (locals.var_sp_ov_xi * locals.var_sp_ov_q);
        let assign23670_e23407: f64 = (4.0 * assign23670_e23406);
        let assign23670_e23408: f64 = (assign23670_e23402 - assign23670_e23407);
        (assign23670_e23408, (((locals.var_sp_ov_p_dn4 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn4)) - (4.0 * ((locals.var_sp_ov_xi_dn4 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn4)))), (((locals.var_sp_ov_p_dn6 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn6)) - (4.0 * ((locals.var_sp_ov_xi_dn6 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn6)))), (((locals.var_sp_ov_p_dn7 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn7)) - (4.0 * ((locals.var_sp_ov_xi_dn7 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn7)))), (((locals.var_sp_ov_p_dn8 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn8)) - (4.0 * ((locals.var_sp_ov_xi_dn8 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn8)))), (((locals.var_sp_ov_p_dn9 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn9)) - (4.0 * ((locals.var_sp_ov_xi_dn9 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn9)))),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign23670_e23410;
        locals.var_sp_ov_temp_dn4 = assign23670_e23410_d_n4;
        locals.var_sp_ov_temp_dn6 = assign23670_e23410_d_n6;
        locals.var_sp_ov_temp_dn7 = assign23670_e23410_d_n7;
        locals.var_sp_ov_temp_dn8 = assign23670_e23410_d_n8;
        locals.var_sp_ov_temp_dn9 = assign23670_e23410_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let (assign23680_e23426, assign23680_e23426_d_n4, assign23680_e23426_d_n6, assign23680_e23426_d_n7, assign23680_e23426_d_n8, assign23680_e23426_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign23680_e23419: f64 = (2.0 * locals.var_sp_ov_q);
        let assign23680_e23422: f64 = (locals.var_sp_ov_temp).sqrt();
        let assign23680_e23423: f64 = (locals.var_sp_ov_p + assign23680_e23422);
        let assign23680_e23424: f64 = (assign23680_e23419 / assign23680_e23423);
        (assign23680_e23424, ((((2.0 * locals.var_sp_ov_q_dn4) * assign23680_e23423) - (assign23680_e23419 * (locals.var_sp_ov_p_dn4 + (locals.var_sp_ov_temp_dn4 / (2.0 * assign23680_e23422))))) / (assign23680_e23423 * assign23680_e23423)), ((((2.0 * locals.var_sp_ov_q_dn6) * assign23680_e23423) - (assign23680_e23419 * (locals.var_sp_ov_p_dn6 + (locals.var_sp_ov_temp_dn6 / (2.0 * assign23680_e23422))))) / (assign23680_e23423 * assign23680_e23423)), ((((2.0 * locals.var_sp_ov_q_dn7) * assign23680_e23423) - (assign23680_e23419 * (locals.var_sp_ov_p_dn7 + (locals.var_sp_ov_temp_dn7 / (2.0 * assign23680_e23422))))) / (assign23680_e23423 * assign23680_e23423)), ((((2.0 * locals.var_sp_ov_q_dn8) * assign23680_e23423) - (assign23680_e23419 * (locals.var_sp_ov_p_dn8 + (locals.var_sp_ov_temp_dn8 / (2.0 * assign23680_e23422))))) / (assign23680_e23423 * assign23680_e23423)), ((((2.0 * locals.var_sp_ov_q_dn9) * assign23680_e23423) - (assign23680_e23419 * (locals.var_sp_ov_p_dn9 + (locals.var_sp_ov_temp_dn9 / (2.0 * assign23680_e23422))))) / (assign23680_e23423 * assign23680_e23423)),)
    } else {
        (locals.var_sp_ov_w, locals.var_sp_ov_w_dn4, locals.var_sp_ov_w_dn6, locals.var_sp_ov_w_dn7, locals.var_sp_ov_w_dn8, locals.var_sp_ov_w_dn9,)
    }
};
        locals.var_sp_ov_w = assign23680_e23426;
        locals.var_sp_ov_w_dn4 = assign23680_e23426_d_n4;
        locals.var_sp_ov_w_dn6 = assign23680_e23426_d_n6;
        locals.var_sp_ov_w_dn7 = assign23680_e23426_d_n7;
        locals.var_sp_ov_w_dn8 = assign23680_e23426_d_n8;
        locals.var_sp_ov_w_dn9 = assign23680_e23426_d_n9;
        locals.var_sp_ov_w_rv = 0.0;

        let (assign23690_e23438, assign23690_e23438_d_n4, assign23690_e23438_d_n6, assign23690_e23438_d_n7, assign23690_e23438_d_n8, assign23690_e23438_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign23690_e23435: f64 = (locals.var_sp_ov_y0 + locals.var_sp_ov_w);
        let assign23690_e23436: f64 = (-assign23690_e23435);
        (assign23690_e23436, (-(locals.var_sp_ov_y0_dn4 + locals.var_sp_ov_w_dn4)), (-(locals.var_sp_ov_y0_dn6 + locals.var_sp_ov_w_dn6)), (-(locals.var_sp_ov_y0_dn7 + locals.var_sp_ov_w_dn7)), (-(locals.var_sp_ov_y0_dn8 + locals.var_sp_ov_w_dn8)), (-(locals.var_sp_ov_y0_dn9 + locals.var_sp_ov_w_dn9)),)
    } else {
        (locals.var_xs_ov, locals.var_xs_ov_dn4, locals.var_xs_ov_dn6, locals.var_xs_ov_dn7, locals.var_xs_ov_dn8, locals.var_xs_ov_dn9,)
    }
};
        locals.var_xs_ov = assign23690_e23438;
        locals.var_xs_ov_dn4 = assign23690_e23438_d_n4;
        locals.var_xs_ov_dn6 = assign23690_e23438_d_n6;
        locals.var_xs_ov_dn7 = assign23690_e23438_d_n7;
        locals.var_xs_ov_dn8 = assign23690_e23438_d_n8;
        locals.var_xs_ov_dn9 = assign23690_e23438_d_n9;
        locals.var_xs_ov_rv = 0.0;

        let (assign23700_e23456, assign23700_e23456_d_n4, assign23700_e23456_d_n6, assign23700_e23456_d_n7, assign23700_e23456_d_n8, assign23700_e23456_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 == 0.0)) {
        let assign23700_e23448: f64 = (locals.var_xi_ov * 1.25);
        let assign23700_e23450: f64 = (assign23700_e23448 * locals.var_inv_xg1);
        let assign23700_e23452: f64 = (assign23700_e23450 - 1.0);
        let assign23700_e23454: f64 = (assign23700_e23452 * locals.var_inv_xg1);
        (assign23700_e23454, (((((locals.var_xi_ov_dn4 * 1.25) * locals.var_inv_xg1) + (assign23700_e23448 * locals.var_inv_xg1_dn4)) * locals.var_inv_xg1) + (assign23700_e23452 * locals.var_inv_xg1_dn4)), (((((locals.var_xi_ov_dn6 * 1.25) * locals.var_inv_xg1) + (assign23700_e23448 * locals.var_inv_xg1_dn6)) * locals.var_inv_xg1) + (assign23700_e23452 * locals.var_inv_xg1_dn6)), (((((locals.var_xi_ov_dn7 * 1.25) * locals.var_inv_xg1) + (assign23700_e23448 * locals.var_inv_xg1_dn7)) * locals.var_inv_xg1) + (assign23700_e23452 * locals.var_inv_xg1_dn7)), (((((locals.var_xi_ov_dn8 * 1.25) * locals.var_inv_xg1) + (assign23700_e23448 * locals.var_inv_xg1_dn8)) * locals.var_inv_xg1) + (assign23700_e23452 * locals.var_inv_xg1_dn8)), (((((locals.var_xi_ov_dn9 * 1.25) * locals.var_inv_xg1) + (assign23700_e23448 * locals.var_inv_xg1_dn9)) * locals.var_inv_xg1) + (assign23700_e23452 * locals.var_inv_xg1_dn9)),)
    } else {
        (locals.var_sp_ov_afac, locals.var_sp_ov_afac_dn4, locals.var_sp_ov_afac_dn6, locals.var_sp_ov_afac_dn7, locals.var_sp_ov_afac_dn8, locals.var_sp_ov_afac_dn9,)
    }
};
        locals.var_sp_ov_afac = assign23700_e23456;
        locals.var_sp_ov_afac_dn4 = assign23700_e23456_d_n4;
        locals.var_sp_ov_afac_dn6 = assign23700_e23456_d_n6;
        locals.var_sp_ov_afac_dn7 = assign23700_e23456_d_n7;
        locals.var_sp_ov_afac_dn8 = assign23700_e23456_d_n8;
        locals.var_sp_ov_afac_dn9 = assign23700_e23456_d_n9;
        locals.var_sp_ov_afac_rv = 0.0;

        let (assign23710_e23474, assign23710_e23474_d_n4, assign23710_e23474_d_n6, assign23710_e23474_d_n7, assign23710_e23474_d_n8, assign23710_e23474_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 == 0.0)) {
        let assign23710_e23466: f64 = (locals.var_xgs_ov * locals.var_inv_xi_ov);
        let assign23710_e23470: f64 = (locals.var_sp_ov_afac * locals.var_xgs_ov);
        let assign23710_e23471: f64 = (1.0 + assign23710_e23470);
        let assign23710_e23472: f64 = (assign23710_e23466 * assign23710_e23471);
        (assign23710_e23472, ((((locals.var_xgs_ov_dn4 * locals.var_inv_xi_ov) + (locals.var_xgs_ov * locals.var_inv_xi_ov_dn4)) * assign23710_e23471) + (assign23710_e23466 * ((locals.var_sp_ov_afac_dn4 * locals.var_xgs_ov) + (locals.var_sp_ov_afac * locals.var_xgs_ov_dn4)))), ((((locals.var_xgs_ov_dn6 * locals.var_inv_xi_ov) + (locals.var_xgs_ov * locals.var_inv_xi_ov_dn6)) * assign23710_e23471) + (assign23710_e23466 * ((locals.var_sp_ov_afac_dn6 * locals.var_xgs_ov) + (locals.var_sp_ov_afac * locals.var_xgs_ov_dn6)))), ((((locals.var_xgs_ov_dn7 * locals.var_inv_xi_ov) + (locals.var_xgs_ov * locals.var_inv_xi_ov_dn7)) * assign23710_e23471) + (assign23710_e23466 * ((locals.var_sp_ov_afac_dn7 * locals.var_xgs_ov) + (locals.var_sp_ov_afac * locals.var_xgs_ov_dn7)))), ((((locals.var_xgs_ov_dn8 * locals.var_inv_xi_ov) + (locals.var_xgs_ov * locals.var_inv_xi_ov_dn8)) * assign23710_e23471) + (assign23710_e23466 * ((locals.var_sp_ov_afac_dn8 * locals.var_xgs_ov) + (locals.var_sp_ov_afac * locals.var_xgs_ov_dn8)))), ((((locals.var_xgs_ov_dn9 * locals.var_inv_xi_ov) + (locals.var_xgs_ov * locals.var_inv_xi_ov_dn9)) * assign23710_e23471) + (assign23710_e23466 * ((locals.var_sp_ov_afac_dn9 * locals.var_xgs_ov) + (locals.var_sp_ov_afac * locals.var_xgs_ov_dn9)))),)
    } else {
        (locals.var_sp_ov_xbar, locals.var_sp_ov_xbar_dn4, locals.var_sp_ov_xbar_dn6, locals.var_sp_ov_xbar_dn7, locals.var_sp_ov_xbar_dn8, locals.var_sp_ov_xbar_dn9,)
    }
};
        locals.var_sp_ov_xbar = assign23710_e23474;
        locals.var_sp_ov_xbar_dn4 = assign23710_e23474_d_n4;
        locals.var_sp_ov_xbar_dn6 = assign23710_e23474_d_n6;
        locals.var_sp_ov_xbar_dn7 = assign23710_e23474_d_n7;
        locals.var_sp_ov_xbar_dn8 = assign23710_e23474_d_n8;
        locals.var_sp_ov_xbar_dn9 = assign23710_e23474_d_n9;
        locals.var_sp_ov_xbar_rv = 0.0;

        let assign23720_e23476: f64 = (-locals.var_sp_ov_xbar);
        let assign23720_e23477: f64 = (assign23720_e23476).abs();
        let assign23720_e23479: f64 = if assign23720_e23477 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard690 = assign23720_e23479;
        locals.var_guard690_rv = 0.0;

        let (assign23730_e23493, assign23730_e23493_d_n4, assign23730_e23493_d_n6, assign23730_e23493_d_n7, assign23730_e23493_d_n8, assign23730_e23493_d_n9,) = {
    if ((((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 == 0.0)) && (locals.var_guard690 != 0.0)) {
        let assign23730_e23490: f64 = (-locals.var_sp_ov_xbar);
        let assign23730_e23491: f64 = (assign23730_e23490).exp();
        (assign23730_e23491, (assign23730_e23491 * (-locals.var_sp_ov_xbar_dn4)), (assign23730_e23491 * (-locals.var_sp_ov_xbar_dn6)), (assign23730_e23491 * (-locals.var_sp_ov_xbar_dn7)), (assign23730_e23491 * (-locals.var_sp_ov_xbar_dn8)), (assign23730_e23491 * (-locals.var_sp_ov_xbar_dn9)),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign23730_e23493;
        locals.var_sp_ov_temp_dn4 = assign23730_e23493_d_n4;
        locals.var_sp_ov_temp_dn6 = assign23730_e23493_d_n6;
        locals.var_sp_ov_temp_dn7 = assign23730_e23493_d_n7;
        locals.var_sp_ov_temp_dn8 = assign23730_e23493_d_n8;
        locals.var_sp_ov_temp_dn9 = assign23730_e23493_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let assign23740_e23495: f64 = (-locals.var_sp_ov_xbar);
        let assign23740_e23497: f64 = (-80.0);
        let assign23740_e23498: f64 = if assign23740_e23495 < assign23740_e23497 { 1.0 } else { 0.0 };
        locals.var_guard691 = assign23740_e23498;
        locals.var_guard691_rv = 0.0;

        let (assign23750_e23541, assign23750_e23541_d_n4, assign23750_e23541_d_n6, assign23750_e23541_d_n7, assign23750_e23541_d_n8, assign23750_e23541_d_n9,) = {
    if (((((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 == 0.0)) && (locals.var_guard690 == 0.0)) && (locals.var_guard691 != 0.0)) {
        let assign23750_e23514: f64 = (-locals.var_sp_ov_xbar);
        let assign23750_e23515: f64 = (-assign23750_e23514);
        let assign23750_e23517: f64 = (assign23750_e23515 - 80.0);
        let assign23750_e23521: f64 = (-locals.var_sp_ov_xbar);
        let assign23750_e23522: f64 = (-assign23750_e23521);
        let assign23750_e23524: f64 = (assign23750_e23522 - 80.0);
        let assign23750_e23525: f64 = (0.5 * assign23750_e23524);
        let assign23750_e23528: f64 = (-locals.var_sp_ov_xbar);
        let assign23750_e23529: f64 = (-assign23750_e23528);
        let assign23750_e23531: f64 = (assign23750_e23529 - 80.0);
        let assign23750_e23533: f64 = (assign23750_e23531 * 0.3333333333333);
        let assign23750_e23534: f64 = (1.0 + assign23750_e23533);
        let assign23750_e23535: f64 = (assign23750_e23525 * assign23750_e23534);
        let assign23750_e23536: f64 = (1.0 + assign23750_e23535);
        let assign23750_e23537: f64 = (assign23750_e23517 * assign23750_e23536);
        let assign23750_e23538: f64 = (1.0 + assign23750_e23537);
        let assign23750_e23539: f64 = (1.80485e-35 / assign23750_e23538);
        (assign23750_e23539, (-((1.80485e-35 * (((-(-locals.var_sp_ov_xbar_dn4)) * assign23750_e23536) + (assign23750_e23517 * (((0.5 * (-(-locals.var_sp_ov_xbar_dn4))) * assign23750_e23534) + (assign23750_e23525 * ((-(-locals.var_sp_ov_xbar_dn4)) * 0.3333333333333)))))) / (assign23750_e23538 * assign23750_e23538))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_xbar_dn6)) * assign23750_e23536) + (assign23750_e23517 * (((0.5 * (-(-locals.var_sp_ov_xbar_dn6))) * assign23750_e23534) + (assign23750_e23525 * ((-(-locals.var_sp_ov_xbar_dn6)) * 0.3333333333333)))))) / (assign23750_e23538 * assign23750_e23538))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_xbar_dn7)) * assign23750_e23536) + (assign23750_e23517 * (((0.5 * (-(-locals.var_sp_ov_xbar_dn7))) * assign23750_e23534) + (assign23750_e23525 * ((-(-locals.var_sp_ov_xbar_dn7)) * 0.3333333333333)))))) / (assign23750_e23538 * assign23750_e23538))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_xbar_dn8)) * assign23750_e23536) + (assign23750_e23517 * (((0.5 * (-(-locals.var_sp_ov_xbar_dn8))) * assign23750_e23534) + (assign23750_e23525 * ((-(-locals.var_sp_ov_xbar_dn8)) * 0.3333333333333)))))) / (assign23750_e23538 * assign23750_e23538))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_xbar_dn9)) * assign23750_e23536) + (assign23750_e23517 * (((0.5 * (-(-locals.var_sp_ov_xbar_dn9))) * assign23750_e23534) + (assign23750_e23525 * ((-(-locals.var_sp_ov_xbar_dn9)) * 0.3333333333333)))))) / (assign23750_e23538 * assign23750_e23538))),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign23750_e23541;
        locals.var_sp_ov_temp_dn4 = assign23750_e23541_d_n4;
        locals.var_sp_ov_temp_dn6 = assign23750_e23541_d_n6;
        locals.var_sp_ov_temp_dn7 = assign23750_e23541_d_n7;
        locals.var_sp_ov_temp_dn8 = assign23750_e23541_d_n8;
        locals.var_sp_ov_temp_dn9 = assign23750_e23541_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let (assign23760_e23582, assign23760_e23582_d_n4, assign23760_e23582_d_n6, assign23760_e23582_d_n7, assign23760_e23582_d_n8, assign23760_e23582_d_n9,) = {
    if (((((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 == 0.0)) && (locals.var_guard690 == 0.0)) && (locals.var_guard691 == 0.0)) {
        let assign23760_e23558: f64 = (-locals.var_sp_ov_xbar);
        let assign23760_e23560: f64 = (assign23760_e23558 - 80.0);
        let assign23760_e23564: f64 = (-locals.var_sp_ov_xbar);
        let assign23760_e23566: f64 = (assign23760_e23564 - 80.0);
        let assign23760_e23567: f64 = (0.5 * assign23760_e23566);
        let assign23760_e23570: f64 = (-locals.var_sp_ov_xbar);
        let assign23760_e23572: f64 = (assign23760_e23570 - 80.0);
        let assign23760_e23574: f64 = (assign23760_e23572 * 0.3333333333333);
        let assign23760_e23575: f64 = (1.0 + assign23760_e23574);
        let assign23760_e23576: f64 = (assign23760_e23567 * assign23760_e23575);
        let assign23760_e23577: f64 = (1.0 + assign23760_e23576);
        let assign23760_e23578: f64 = (assign23760_e23560 * assign23760_e23577);
        let assign23760_e23579: f64 = (1.0 + assign23760_e23578);
        let assign23760_e23580: f64 = (5.54062e34 * assign23760_e23579);
        (assign23760_e23580, (5.54062e34 * (((-locals.var_sp_ov_xbar_dn4) * assign23760_e23577) + (assign23760_e23560 * (((0.5 * (-locals.var_sp_ov_xbar_dn4)) * assign23760_e23575) + (assign23760_e23567 * ((-locals.var_sp_ov_xbar_dn4) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_xbar_dn6) * assign23760_e23577) + (assign23760_e23560 * (((0.5 * (-locals.var_sp_ov_xbar_dn6)) * assign23760_e23575) + (assign23760_e23567 * ((-locals.var_sp_ov_xbar_dn6) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_xbar_dn7) * assign23760_e23577) + (assign23760_e23560 * (((0.5 * (-locals.var_sp_ov_xbar_dn7)) * assign23760_e23575) + (assign23760_e23567 * ((-locals.var_sp_ov_xbar_dn7) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_xbar_dn8) * assign23760_e23577) + (assign23760_e23560 * (((0.5 * (-locals.var_sp_ov_xbar_dn8)) * assign23760_e23575) + (assign23760_e23567 * ((-locals.var_sp_ov_xbar_dn8) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_xbar_dn9) * assign23760_e23577) + (assign23760_e23560 * (((0.5 * (-locals.var_sp_ov_xbar_dn9)) * assign23760_e23575) + (assign23760_e23567 * ((-locals.var_sp_ov_xbar_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign23760_e23582;
        locals.var_sp_ov_temp_dn4 = assign23760_e23582_d_n4;
        locals.var_sp_ov_temp_dn6 = assign23760_e23582_d_n6;
        locals.var_sp_ov_temp_dn7 = assign23760_e23582_d_n7;
        locals.var_sp_ov_temp_dn8 = assign23760_e23582_d_n8;
        locals.var_sp_ov_temp_dn9 = assign23760_e23582_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let (assign23770_e23594, assign23770_e23594_d_n4, assign23770_e23594_d_n6, assign23770_e23594_d_n7, assign23770_e23594_d_n8, assign23770_e23594_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 == 0.0)) {
        let assign23770_e23592: f64 = (1.0 - locals.var_sp_ov_temp);
        (assign23770_e23592, (-locals.var_sp_ov_temp_dn4), (-locals.var_sp_ov_temp_dn6), (-locals.var_sp_ov_temp_dn7), (-locals.var_sp_ov_temp_dn8), (-locals.var_sp_ov_temp_dn9),)
    } else {
        (locals.var_sp_ov_w, locals.var_sp_ov_w_dn4, locals.var_sp_ov_w_dn6, locals.var_sp_ov_w_dn7, locals.var_sp_ov_w_dn8, locals.var_sp_ov_w_dn9,)
    }
};
        locals.var_sp_ov_w = assign23770_e23594;
        locals.var_sp_ov_w_dn4 = assign23770_e23594_d_n4;
        locals.var_sp_ov_w_dn6 = assign23770_e23594_d_n6;
        locals.var_sp_ov_w_dn7 = assign23770_e23594_d_n7;
        locals.var_sp_ov_w_dn8 = assign23770_e23594_d_n8;
        locals.var_sp_ov_w_dn9 = assign23770_e23594_d_n9;
        locals.var_sp_ov_w_rv = 0.0;

        let (assign23780_e23619, assign23780_e23619_d_n4, assign23780_e23619_d_n6, assign23780_e23619_d_n7, assign23780_e23619_d_n8, assign23780_e23619_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 == 0.0)) {
        let assign23780_e23605: f64 = (locals.var_gov2 * 0.5);
        let assign23780_e23606: f64 = (locals.var_xgs_ov + assign23780_e23605);
        let assign23780_e23611: f64 = (locals.var_gov2 * 0.25);
        let assign23780_e23612: f64 = (locals.var_xgs_ov + assign23780_e23611);
        let assign23780_e23614: f64 = (assign23780_e23612 - locals.var_sp_ov_w);
        let assign23780_e23615: f64 = (assign23780_e23614).sqrt();
        let assign23780_e23616: f64 = (locals.var_gov * assign23780_e23615);
        let assign23780_e23617: f64 = (assign23780_e23606 - assign23780_e23616);
        (assign23780_e23617, ((locals.var_xgs_ov_dn4 + (locals.var_gov2_dn4 * 0.5)) - ((locals.var_gov_dn4 * assign23780_e23615) + (locals.var_gov * (((locals.var_xgs_ov_dn4 + (locals.var_gov2_dn4 * 0.25)) - locals.var_sp_ov_w_dn4) / (2.0 * assign23780_e23615))))), ((locals.var_xgs_ov_dn6 + (locals.var_gov2_dn6 * 0.5)) - ((locals.var_gov_dn6 * assign23780_e23615) + (locals.var_gov * (((locals.var_xgs_ov_dn6 + (locals.var_gov2_dn6 * 0.25)) - locals.var_sp_ov_w_dn6) / (2.0 * assign23780_e23615))))), ((locals.var_xgs_ov_dn7 + (locals.var_gov2_dn7 * 0.5)) - ((locals.var_gov_dn7 * assign23780_e23615) + (locals.var_gov * (((locals.var_xgs_ov_dn7 + (locals.var_gov2_dn7 * 0.25)) - locals.var_sp_ov_w_dn7) / (2.0 * assign23780_e23615))))), ((locals.var_xgs_ov_dn8 + (locals.var_gov2_dn8 * 0.5)) - ((locals.var_gov_dn8 * assign23780_e23615) + (locals.var_gov * (((locals.var_xgs_ov_dn8 + (locals.var_gov2_dn8 * 0.25)) - locals.var_sp_ov_w_dn8) / (2.0 * assign23780_e23615))))), ((locals.var_xgs_ov_dn9 + (locals.var_gov2_dn9 * 0.5)) - ((locals.var_gov_dn9 * assign23780_e23615) + (locals.var_gov * (((locals.var_xgs_ov_dn9 + (locals.var_gov2_dn9 * 0.25)) - locals.var_sp_ov_w_dn9) / (2.0 * assign23780_e23615))))),)
    } else {
        (locals.var_sp_ov_x0, locals.var_sp_ov_x0_dn4, locals.var_sp_ov_x0_dn6, locals.var_sp_ov_x0_dn7, locals.var_sp_ov_x0_dn8, locals.var_sp_ov_x0_dn9,)
    }
};
        locals.var_sp_ov_x0 = assign23780_e23619;
        locals.var_sp_ov_x0_dn4 = assign23780_e23619_d_n4;
        locals.var_sp_ov_x0_dn6 = assign23780_e23619_d_n6;
        locals.var_sp_ov_x0_dn7 = assign23780_e23619_d_n7;
        locals.var_sp_ov_x0_dn8 = assign23780_e23619_d_n8;
        locals.var_sp_ov_x0_dn9 = assign23780_e23619_d_n9;
        locals.var_sp_ov_x0_rv = 0.0;

        let assign23790_e23621: f64 = (-locals.var_sp_ov_x0);
        let assign23790_e23622: f64 = (assign23790_e23621).abs();
        let assign23790_e23624: f64 = if assign23790_e23622 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard692 = assign23790_e23624;
        locals.var_guard692_rv = 0.0;

        let (assign23800_e23638, assign23800_e23638_d_n4, assign23800_e23638_d_n6, assign23800_e23638_d_n7, assign23800_e23638_d_n8, assign23800_e23638_d_n9,) = {
    if ((((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 == 0.0)) && (locals.var_guard692 != 0.0)) {
        let assign23800_e23635: f64 = (-locals.var_sp_ov_x0);
        let assign23800_e23636: f64 = (assign23800_e23635).exp();
        (assign23800_e23636, (assign23800_e23636 * (-locals.var_sp_ov_x0_dn4)), (assign23800_e23636 * (-locals.var_sp_ov_x0_dn6)), (assign23800_e23636 * (-locals.var_sp_ov_x0_dn7)), (assign23800_e23636 * (-locals.var_sp_ov_x0_dn8)), (assign23800_e23636 * (-locals.var_sp_ov_x0_dn9)),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign23800_e23638;
        locals.var_sp_ov_d0_dn4 = assign23800_e23638_d_n4;
        locals.var_sp_ov_d0_dn6 = assign23800_e23638_d_n6;
        locals.var_sp_ov_d0_dn7 = assign23800_e23638_d_n7;
        locals.var_sp_ov_d0_dn8 = assign23800_e23638_d_n8;
        locals.var_sp_ov_d0_dn9 = assign23800_e23638_d_n9;
        locals.var_sp_ov_d0_rv = 0.0;

        let assign23810_e23640: f64 = (-locals.var_sp_ov_x0);
        let assign23810_e23642: f64 = (-80.0);
        let assign23810_e23643: f64 = if assign23810_e23640 < assign23810_e23642 { 1.0 } else { 0.0 };
        locals.var_guard693 = assign23810_e23643;
        locals.var_guard693_rv = 0.0;

        let (assign23820_e23686, assign23820_e23686_d_n4, assign23820_e23686_d_n6, assign23820_e23686_d_n7, assign23820_e23686_d_n8, assign23820_e23686_d_n9,) = {
    if (((((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 == 0.0)) && (locals.var_guard692 == 0.0)) && (locals.var_guard693 != 0.0)) {
        let assign23820_e23659: f64 = (-locals.var_sp_ov_x0);
        let assign23820_e23660: f64 = (-assign23820_e23659);
        let assign23820_e23662: f64 = (assign23820_e23660 - 80.0);
        let assign23820_e23666: f64 = (-locals.var_sp_ov_x0);
        let assign23820_e23667: f64 = (-assign23820_e23666);
        let assign23820_e23669: f64 = (assign23820_e23667 - 80.0);
        let assign23820_e23670: f64 = (0.5 * assign23820_e23669);
        let assign23820_e23673: f64 = (-locals.var_sp_ov_x0);
        let assign23820_e23674: f64 = (-assign23820_e23673);
        let assign23820_e23676: f64 = (assign23820_e23674 - 80.0);
        let assign23820_e23678: f64 = (assign23820_e23676 * 0.3333333333333);
        let assign23820_e23679: f64 = (1.0 + assign23820_e23678);
        let assign23820_e23680: f64 = (assign23820_e23670 * assign23820_e23679);
        let assign23820_e23681: f64 = (1.0 + assign23820_e23680);
        let assign23820_e23682: f64 = (assign23820_e23662 * assign23820_e23681);
        let assign23820_e23683: f64 = (1.0 + assign23820_e23682);
        let assign23820_e23684: f64 = (1.80485e-35 / assign23820_e23683);
        (assign23820_e23684, (-((1.80485e-35 * (((-(-locals.var_sp_ov_x0_dn4)) * assign23820_e23681) + (assign23820_e23662 * (((0.5 * (-(-locals.var_sp_ov_x0_dn4))) * assign23820_e23679) + (assign23820_e23670 * ((-(-locals.var_sp_ov_x0_dn4)) * 0.3333333333333)))))) / (assign23820_e23683 * assign23820_e23683))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_x0_dn6)) * assign23820_e23681) + (assign23820_e23662 * (((0.5 * (-(-locals.var_sp_ov_x0_dn6))) * assign23820_e23679) + (assign23820_e23670 * ((-(-locals.var_sp_ov_x0_dn6)) * 0.3333333333333)))))) / (assign23820_e23683 * assign23820_e23683))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_x0_dn7)) * assign23820_e23681) + (assign23820_e23662 * (((0.5 * (-(-locals.var_sp_ov_x0_dn7))) * assign23820_e23679) + (assign23820_e23670 * ((-(-locals.var_sp_ov_x0_dn7)) * 0.3333333333333)))))) / (assign23820_e23683 * assign23820_e23683))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_x0_dn8)) * assign23820_e23681) + (assign23820_e23662 * (((0.5 * (-(-locals.var_sp_ov_x0_dn8))) * assign23820_e23679) + (assign23820_e23670 * ((-(-locals.var_sp_ov_x0_dn8)) * 0.3333333333333)))))) / (assign23820_e23683 * assign23820_e23683))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_x0_dn9)) * assign23820_e23681) + (assign23820_e23662 * (((0.5 * (-(-locals.var_sp_ov_x0_dn9))) * assign23820_e23679) + (assign23820_e23670 * ((-(-locals.var_sp_ov_x0_dn9)) * 0.3333333333333)))))) / (assign23820_e23683 * assign23820_e23683))),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign23820_e23686;
        locals.var_sp_ov_d0_dn4 = assign23820_e23686_d_n4;
        locals.var_sp_ov_d0_dn6 = assign23820_e23686_d_n6;
        locals.var_sp_ov_d0_dn7 = assign23820_e23686_d_n7;
        locals.var_sp_ov_d0_dn8 = assign23820_e23686_d_n8;
        locals.var_sp_ov_d0_dn9 = assign23820_e23686_d_n9;
        locals.var_sp_ov_d0_rv = 0.0;

        let (assign23830_e23727, assign23830_e23727_d_n4, assign23830_e23727_d_n6, assign23830_e23727_d_n7, assign23830_e23727_d_n8, assign23830_e23727_d_n9,) = {
    if (((((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 == 0.0)) && (locals.var_guard692 == 0.0)) && (locals.var_guard693 == 0.0)) {
        let assign23830_e23703: f64 = (-locals.var_sp_ov_x0);
        let assign23830_e23705: f64 = (assign23830_e23703 - 80.0);
        let assign23830_e23709: f64 = (-locals.var_sp_ov_x0);
        let assign23830_e23711: f64 = (assign23830_e23709 - 80.0);
        let assign23830_e23712: f64 = (0.5 * assign23830_e23711);
        let assign23830_e23715: f64 = (-locals.var_sp_ov_x0);
        let assign23830_e23717: f64 = (assign23830_e23715 - 80.0);
        let assign23830_e23719: f64 = (assign23830_e23717 * 0.3333333333333);
        let assign23830_e23720: f64 = (1.0 + assign23830_e23719);
        let assign23830_e23721: f64 = (assign23830_e23712 * assign23830_e23720);
        let assign23830_e23722: f64 = (1.0 + assign23830_e23721);
        let assign23830_e23723: f64 = (assign23830_e23705 * assign23830_e23722);
        let assign23830_e23724: f64 = (1.0 + assign23830_e23723);
        let assign23830_e23725: f64 = (5.54062e34 * assign23830_e23724);
        (assign23830_e23725, (5.54062e34 * (((-locals.var_sp_ov_x0_dn4) * assign23830_e23722) + (assign23830_e23705 * (((0.5 * (-locals.var_sp_ov_x0_dn4)) * assign23830_e23720) + (assign23830_e23712 * ((-locals.var_sp_ov_x0_dn4) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_x0_dn6) * assign23830_e23722) + (assign23830_e23705 * (((0.5 * (-locals.var_sp_ov_x0_dn6)) * assign23830_e23720) + (assign23830_e23712 * ((-locals.var_sp_ov_x0_dn6) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_x0_dn7) * assign23830_e23722) + (assign23830_e23705 * (((0.5 * (-locals.var_sp_ov_x0_dn7)) * assign23830_e23720) + (assign23830_e23712 * ((-locals.var_sp_ov_x0_dn7) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_x0_dn8) * assign23830_e23722) + (assign23830_e23705 * (((0.5 * (-locals.var_sp_ov_x0_dn8)) * assign23830_e23720) + (assign23830_e23712 * ((-locals.var_sp_ov_x0_dn8) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_x0_dn9) * assign23830_e23722) + (assign23830_e23705 * (((0.5 * (-locals.var_sp_ov_x0_dn9)) * assign23830_e23720) + (assign23830_e23712 * ((-locals.var_sp_ov_x0_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign23830_e23727;
        locals.var_sp_ov_d0_dn4 = assign23830_e23727_d_n4;
        locals.var_sp_ov_d0_dn6 = assign23830_e23727_d_n6;
        locals.var_sp_ov_d0_dn7 = assign23830_e23727_d_n7;
        locals.var_sp_ov_d0_dn8 = assign23830_e23727_d_n8;
        locals.var_sp_ov_d0_dn9 = assign23830_e23727_d_n9;
        locals.var_sp_ov_d0_rv = 0.0;

        let (assign23840_e23747, assign23840_e23747_d_n4, assign23840_e23747_d_n6, assign23840_e23747_d_n7, assign23840_e23747_d_n8, assign23840_e23747_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 == 0.0)) {
        let assign23840_e23738: f64 = (locals.var_xgs_ov - locals.var_sp_ov_x0);
        let assign23840_e23739: f64 = (2.0 * assign23840_e23738);
        let assign23840_e23743: f64 = (1.0 - locals.var_sp_ov_d0);
        let assign23840_e23744: f64 = (locals.var_gov2 * assign23840_e23743);
        let assign23840_e23745: f64 = (assign23840_e23739 + assign23840_e23744);
        (assign23840_e23745, ((2.0 * (locals.var_xgs_ov_dn4 - locals.var_sp_ov_x0_dn4)) + ((locals.var_gov2_dn4 * assign23840_e23743) + (locals.var_gov2 * (-locals.var_sp_ov_d0_dn4)))), ((2.0 * (locals.var_xgs_ov_dn6 - locals.var_sp_ov_x0_dn6)) + ((locals.var_gov2_dn6 * assign23840_e23743) + (locals.var_gov2 * (-locals.var_sp_ov_d0_dn6)))), ((2.0 * (locals.var_xgs_ov_dn7 - locals.var_sp_ov_x0_dn7)) + ((locals.var_gov2_dn7 * assign23840_e23743) + (locals.var_gov2 * (-locals.var_sp_ov_d0_dn7)))), ((2.0 * (locals.var_xgs_ov_dn8 - locals.var_sp_ov_x0_dn8)) + ((locals.var_gov2_dn8 * assign23840_e23743) + (locals.var_gov2 * (-locals.var_sp_ov_d0_dn8)))), ((2.0 * (locals.var_xgs_ov_dn9 - locals.var_sp_ov_x0_dn9)) + ((locals.var_gov2_dn9 * assign23840_e23743) + (locals.var_gov2 * (-locals.var_sp_ov_d0_dn9)))),)
    } else {
        (locals.var_sp_ov_p, locals.var_sp_ov_p_dn4, locals.var_sp_ov_p_dn6, locals.var_sp_ov_p_dn7, locals.var_sp_ov_p_dn8, locals.var_sp_ov_p_dn9,)
    }
};
        locals.var_sp_ov_p = assign23840_e23747;
        locals.var_sp_ov_p_dn4 = assign23840_e23747_d_n4;
        locals.var_sp_ov_p_dn6 = assign23840_e23747_d_n6;
        locals.var_sp_ov_p_dn7 = assign23840_e23747_d_n7;
        locals.var_sp_ov_p_dn8 = assign23840_e23747_d_n8;
        locals.var_sp_ov_p_dn9 = assign23840_e23747_d_n9;
        locals.var_sp_ov_p_rv = 0.0;

        let (assign23850_e23771, assign23850_e23771_d_n4, assign23850_e23771_d_n6, assign23850_e23771_d_n7, assign23850_e23771_d_n8, assign23850_e23771_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 == 0.0)) {
        let assign23850_e23757: f64 = (locals.var_xgs_ov - locals.var_sp_ov_x0);
        let assign23850_e23760: f64 = (locals.var_xgs_ov - locals.var_sp_ov_x0);
        let assign23850_e23761: f64 = (assign23850_e23757 * assign23850_e23760);
        let assign23850_e23765: f64 = (locals.var_sp_ov_x0 - 1.0);
        let assign23850_e23767: f64 = (assign23850_e23765 + locals.var_sp_ov_d0);
        let assign23850_e23768: f64 = (locals.var_gov2 * assign23850_e23767);
        let assign23850_e23769: f64 = (assign23850_e23761 - assign23850_e23768);
        (assign23850_e23769, ((((locals.var_xgs_ov_dn4 - locals.var_sp_ov_x0_dn4) * assign23850_e23760) + (assign23850_e23757 * (locals.var_xgs_ov_dn4 - locals.var_sp_ov_x0_dn4))) - ((locals.var_gov2_dn4 * assign23850_e23767) + (locals.var_gov2 * (locals.var_sp_ov_x0_dn4 + locals.var_sp_ov_d0_dn4)))), ((((locals.var_xgs_ov_dn6 - locals.var_sp_ov_x0_dn6) * assign23850_e23760) + (assign23850_e23757 * (locals.var_xgs_ov_dn6 - locals.var_sp_ov_x0_dn6))) - ((locals.var_gov2_dn6 * assign23850_e23767) + (locals.var_gov2 * (locals.var_sp_ov_x0_dn6 + locals.var_sp_ov_d0_dn6)))), ((((locals.var_xgs_ov_dn7 - locals.var_sp_ov_x0_dn7) * assign23850_e23760) + (assign23850_e23757 * (locals.var_xgs_ov_dn7 - locals.var_sp_ov_x0_dn7))) - ((locals.var_gov2_dn7 * assign23850_e23767) + (locals.var_gov2 * (locals.var_sp_ov_x0_dn7 + locals.var_sp_ov_d0_dn7)))), ((((locals.var_xgs_ov_dn8 - locals.var_sp_ov_x0_dn8) * assign23850_e23760) + (assign23850_e23757 * (locals.var_xgs_ov_dn8 - locals.var_sp_ov_x0_dn8))) - ((locals.var_gov2_dn8 * assign23850_e23767) + (locals.var_gov2 * (locals.var_sp_ov_x0_dn8 + locals.var_sp_ov_d0_dn8)))), ((((locals.var_xgs_ov_dn9 - locals.var_sp_ov_x0_dn9) * assign23850_e23760) + (assign23850_e23757 * (locals.var_xgs_ov_dn9 - locals.var_sp_ov_x0_dn9))) - ((locals.var_gov2_dn9 * assign23850_e23767) + (locals.var_gov2 * (locals.var_sp_ov_x0_dn9 + locals.var_sp_ov_d0_dn9)))),)
    } else {
        (locals.var_sp_ov_q, locals.var_sp_ov_q_dn4, locals.var_sp_ov_q_dn6, locals.var_sp_ov_q_dn7, locals.var_sp_ov_q_dn8, locals.var_sp_ov_q_dn9,)
    }
};
        locals.var_sp_ov_q = assign23850_e23771;
        locals.var_sp_ov_q_dn4 = assign23850_e23771_d_n4;
        locals.var_sp_ov_q_dn6 = assign23850_e23771_d_n6;
        locals.var_sp_ov_q_dn7 = assign23850_e23771_d_n7;
        locals.var_sp_ov_q_dn8 = assign23850_e23771_d_n8;
        locals.var_sp_ov_q_dn9 = assign23850_e23771_d_n9;
        locals.var_sp_ov_q_rv = 0.0;

        let (assign23860_e23787, assign23860_e23787_d_n4, assign23860_e23787_d_n6, assign23860_e23787_d_n7, assign23860_e23787_d_n8, assign23860_e23787_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 == 0.0)) {
        let assign23860_e23782: f64 = (locals.var_gov2 * 0.5);
        let assign23860_e23784: f64 = (assign23860_e23782 * locals.var_sp_ov_d0);
        let assign23860_e23785: f64 = (1.0 - assign23860_e23784);
        (assign23860_e23785, (-(((locals.var_gov2_dn4 * 0.5) * locals.var_sp_ov_d0) + (assign23860_e23782 * locals.var_sp_ov_d0_dn4))), (-(((locals.var_gov2_dn6 * 0.5) * locals.var_sp_ov_d0) + (assign23860_e23782 * locals.var_sp_ov_d0_dn6))), (-(((locals.var_gov2_dn7 * 0.5) * locals.var_sp_ov_d0) + (assign23860_e23782 * locals.var_sp_ov_d0_dn7))), (-(((locals.var_gov2_dn8 * 0.5) * locals.var_sp_ov_d0) + (assign23860_e23782 * locals.var_sp_ov_d0_dn8))), (-(((locals.var_gov2_dn9 * 0.5) * locals.var_sp_ov_d0) + (assign23860_e23782 * locals.var_sp_ov_d0_dn9))),)
    } else {
        (locals.var_sp_ov_xi, locals.var_sp_ov_xi_dn4, locals.var_sp_ov_xi_dn6, locals.var_sp_ov_xi_dn7, locals.var_sp_ov_xi_dn8, locals.var_sp_ov_xi_dn9,)
    }
};
        locals.var_sp_ov_xi = assign23860_e23787;
        locals.var_sp_ov_xi_dn4 = assign23860_e23787_d_n4;
        locals.var_sp_ov_xi_dn6 = assign23860_e23787_d_n6;
        locals.var_sp_ov_xi_dn7 = assign23860_e23787_d_n7;
        locals.var_sp_ov_xi_dn8 = assign23860_e23787_d_n8;
        locals.var_sp_ov_xi_dn9 = assign23860_e23787_d_n9;
        locals.var_sp_ov_xi_rv = 0.0;

        let (assign23870_e23805, assign23870_e23805_d_n4, assign23870_e23805_d_n6, assign23870_e23805_d_n7, assign23870_e23805_d_n8, assign23870_e23805_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 == 0.0)) {
        let assign23870_e23797: f64 = (locals.var_sp_ov_p * locals.var_sp_ov_p);
        let assign23870_e23801: f64 = (locals.var_sp_ov_xi * locals.var_sp_ov_q);
        let assign23870_e23802: f64 = (4.0 * assign23870_e23801);
        let assign23870_e23803: f64 = (assign23870_e23797 - assign23870_e23802);
        (assign23870_e23803, (((locals.var_sp_ov_p_dn4 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn4)) - (4.0 * ((locals.var_sp_ov_xi_dn4 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn4)))), (((locals.var_sp_ov_p_dn6 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn6)) - (4.0 * ((locals.var_sp_ov_xi_dn6 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn6)))), (((locals.var_sp_ov_p_dn7 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn7)) - (4.0 * ((locals.var_sp_ov_xi_dn7 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn7)))), (((locals.var_sp_ov_p_dn8 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn8)) - (4.0 * ((locals.var_sp_ov_xi_dn8 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn8)))), (((locals.var_sp_ov_p_dn9 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn9)) - (4.0 * ((locals.var_sp_ov_xi_dn9 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn9)))),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign23870_e23805;
        locals.var_sp_ov_temp_dn4 = assign23870_e23805_d_n4;
        locals.var_sp_ov_temp_dn6 = assign23870_e23805_d_n6;
        locals.var_sp_ov_temp_dn7 = assign23870_e23805_d_n7;
        locals.var_sp_ov_temp_dn8 = assign23870_e23805_d_n8;
        locals.var_sp_ov_temp_dn9 = assign23870_e23805_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let (assign23880_e23822, assign23880_e23822_d_n4, assign23880_e23822_d_n6, assign23880_e23822_d_n7, assign23880_e23822_d_n8, assign23880_e23822_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 == 0.0)) {
        let assign23880_e23815: f64 = (2.0 * locals.var_sp_ov_q);
        let assign23880_e23818: f64 = (locals.var_sp_ov_temp).sqrt();
        let assign23880_e23819: f64 = (locals.var_sp_ov_p + assign23880_e23818);
        let assign23880_e23820: f64 = (assign23880_e23815 / assign23880_e23819);
        (assign23880_e23820, ((((2.0 * locals.var_sp_ov_q_dn4) * assign23880_e23819) - (assign23880_e23815 * (locals.var_sp_ov_p_dn4 + (locals.var_sp_ov_temp_dn4 / (2.0 * assign23880_e23818))))) / (assign23880_e23819 * assign23880_e23819)), ((((2.0 * locals.var_sp_ov_q_dn6) * assign23880_e23819) - (assign23880_e23815 * (locals.var_sp_ov_p_dn6 + (locals.var_sp_ov_temp_dn6 / (2.0 * assign23880_e23818))))) / (assign23880_e23819 * assign23880_e23819)), ((((2.0 * locals.var_sp_ov_q_dn7) * assign23880_e23819) - (assign23880_e23815 * (locals.var_sp_ov_p_dn7 + (locals.var_sp_ov_temp_dn7 / (2.0 * assign23880_e23818))))) / (assign23880_e23819 * assign23880_e23819)), ((((2.0 * locals.var_sp_ov_q_dn8) * assign23880_e23819) - (assign23880_e23815 * (locals.var_sp_ov_p_dn8 + (locals.var_sp_ov_temp_dn8 / (2.0 * assign23880_e23818))))) / (assign23880_e23819 * assign23880_e23819)), ((((2.0 * locals.var_sp_ov_q_dn9) * assign23880_e23819) - (assign23880_e23815 * (locals.var_sp_ov_p_dn9 + (locals.var_sp_ov_temp_dn9 / (2.0 * assign23880_e23818))))) / (assign23880_e23819 * assign23880_e23819)),)
    } else {
        (locals.var_sp_ov_u, locals.var_sp_ov_u_dn4, locals.var_sp_ov_u_dn6, locals.var_sp_ov_u_dn7, locals.var_sp_ov_u_dn8, locals.var_sp_ov_u_dn9,)
    }
};
        locals.var_sp_ov_u = assign23880_e23822;
        locals.var_sp_ov_u_dn4 = assign23880_e23822_d_n4;
        locals.var_sp_ov_u_dn6 = assign23880_e23822_d_n6;
        locals.var_sp_ov_u_dn7 = assign23880_e23822_d_n7;
        locals.var_sp_ov_u_dn8 = assign23880_e23822_d_n8;
        locals.var_sp_ov_u_dn9 = assign23880_e23822_d_n9;
        locals.var_sp_ov_u_rv = 0.0;

        let (assign23890_e23834, assign23890_e23834_d_n4, assign23890_e23834_d_n6, assign23890_e23834_d_n7, assign23890_e23834_d_n8, assign23890_e23834_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 == 0.0)) {
        let assign23890_e23832: f64 = (locals.var_sp_ov_x0 + locals.var_sp_ov_u);
        (assign23890_e23832, (locals.var_sp_ov_x0_dn4 + locals.var_sp_ov_u_dn4), (locals.var_sp_ov_x0_dn6 + locals.var_sp_ov_u_dn6), (locals.var_sp_ov_x0_dn7 + locals.var_sp_ov_u_dn7), (locals.var_sp_ov_x0_dn8 + locals.var_sp_ov_u_dn8), (locals.var_sp_ov_x0_dn9 + locals.var_sp_ov_u_dn9),)
    } else {
        (locals.var_xs_ov, locals.var_xs_ov_dn4, locals.var_xs_ov_dn6, locals.var_xs_ov_dn7, locals.var_xs_ov_dn8, locals.var_xs_ov_dn9,)
    }
};
        locals.var_xs_ov = assign23890_e23834;
        locals.var_xs_ov_dn4 = assign23890_e23834_d_n4;
        locals.var_xs_ov_dn6 = assign23890_e23834_d_n6;
        locals.var_xs_ov_dn7 = assign23890_e23834_d_n7;
        locals.var_xs_ov_dn8 = assign23890_e23834_d_n8;
        locals.var_xs_ov_dn9 = assign23890_e23834_d_n9;
        locals.var_xs_ov_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_65(
        locals: &mut StampLocals,
    ) {
        let (assign23900_e23842, assign23900_e23842_d_n4, assign23900_e23842_d_n6, assign23900_e23842_d_n7, assign23900_e23842_d_n8, assign23900_e23842_d_n9,) = {
    if ((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) {
        let assign23900_e23840: f64 = (-locals.var_xs_ov);
        (assign23900_e23840, (-locals.var_xs_ov_dn4), (-locals.var_xs_ov_dn6), (-locals.var_xs_ov_dn7), (-locals.var_xs_ov_dn8), (-locals.var_xs_ov_dn9),)
    } else {
        (locals.var_xs_ov, locals.var_xs_ov_dn4, locals.var_xs_ov_dn6, locals.var_xs_ov_dn7, locals.var_xs_ov_dn8, locals.var_xs_ov_dn9,)
    }
};
        locals.var_xs_ov = assign23900_e23842;
        locals.var_xs_ov_dn4 = assign23900_e23842_d_n4;
        locals.var_xs_ov_dn6 = assign23900_e23842_d_n6;
        locals.var_xs_ov_dn7 = assign23900_e23842_d_n7;
        locals.var_xs_ov_dn8 = assign23900_e23842_d_n8;
        locals.var_xs_ov_dn9 = assign23900_e23842_d_n9;
        locals.var_xs_ov_rv = 0.0;

        let assign23910_e23845: f64 = if locals.var_cov_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard694 = assign23910_e23845;
        locals.var_guard694_rv = 0.0;

        let assign23920_e23847: f64 = (locals.var_xgs_ovcv).abs();
        let assign23920_e23849: f64 = if assign23920_e23847 <= locals.var_x_mrg_ov { 1.0 } else { 0.0 };
        locals.var_guard695 = assign23920_e23849;
        locals.var_guard695_rv = 0.0;

        let (assign23930_e23858, assign23930_e23858_d_n4, assign23930_e23858_d_n6, assign23930_e23858_d_n7, assign23930_e23858_d_n8, assign23930_e23858_d_n9,) = {
    if ((locals.var_guard694 != 0.0) && (locals.var_guard695 != 0.0)) {
        let assign23930_e23854: f64 = (-locals.var_xgs_ovcv);
        let assign23930_e23856: f64 = (assign23930_e23854 * locals.var_inv_xi_ov);
        (assign23930_e23856, (((-locals.var_xgs_ovcv_dn4) * locals.var_inv_xi_ov) + (assign23930_e23854 * locals.var_inv_xi_ov_dn4)), (((-locals.var_xgs_ovcv_dn6) * locals.var_inv_xi_ov) + (assign23930_e23854 * locals.var_inv_xi_ov_dn6)), (((-locals.var_xgs_ovcv_dn7) * locals.var_inv_xi_ov) + (assign23930_e23854 * locals.var_inv_xi_ov_dn7)), (((-locals.var_xgs_ovcv_dn8) * locals.var_inv_xi_ov) + (assign23930_e23854 * locals.var_inv_xi_ov_dn8)), (((-locals.var_xgs_ovcv_dn9) * locals.var_inv_xi_ov) + (assign23930_e23854 * locals.var_inv_xi_ov_dn9)),)
    } else {
        (locals.var_xs_ovcv, locals.var_xs_ovcv_dn4, locals.var_xs_ovcv_dn6, locals.var_xs_ovcv_dn7, locals.var_xs_ovcv_dn8, locals.var_xs_ovcv_dn9,)
    }
};
        locals.var_xs_ovcv = assign23930_e23858;
        locals.var_xs_ovcv_dn4 = assign23930_e23858_d_n4;
        locals.var_xs_ovcv_dn6 = assign23930_e23858_d_n6;
        locals.var_xs_ovcv_dn7 = assign23930_e23858_d_n7;
        locals.var_xs_ovcv_dn8 = assign23930_e23858_d_n8;
        locals.var_xs_ovcv_dn9 = assign23930_e23858_d_n9;
        locals.var_xs_ovcv_rv = 0.0;

        let assign23940_e23861: f64 = (-locals.var_x_mrg_ov);
        let assign23940_e23862: f64 = if locals.var_xgs_ovcv < assign23940_e23861 { 1.0 } else { 0.0 };
        locals.var_guard696 = assign23940_e23862;
        locals.var_guard696_rv = 0.0;

        let (assign23950_e23872, assign23950_e23872_d_n4, assign23950_e23872_d_n6, assign23950_e23872_d_n7, assign23950_e23872_d_n8, assign23950_e23872_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) {
        let assign23950_e23870: f64 = (-locals.var_xgs_ovcv);
        (assign23950_e23870, (-locals.var_xgs_ovcv_dn4), (-locals.var_xgs_ovcv_dn6), (-locals.var_xgs_ovcv_dn7), (-locals.var_xgs_ovcv_dn8), (-locals.var_xgs_ovcv_dn9),)
    } else {
        (locals.var_sp_ov_ygf, locals.var_sp_ov_ygf_dn4, locals.var_sp_ov_ygf_dn6, locals.var_sp_ov_ygf_dn7, locals.var_sp_ov_ygf_dn8, locals.var_sp_ov_ygf_dn9,)
    }
};
        locals.var_sp_ov_ygf = assign23950_e23872;
        locals.var_sp_ov_ygf_dn4 = assign23950_e23872_d_n4;
        locals.var_sp_ov_ygf_dn6 = assign23950_e23872_d_n6;
        locals.var_sp_ov_ygf_dn7 = assign23950_e23872_d_n7;
        locals.var_sp_ov_ygf_dn8 = assign23950_e23872_d_n8;
        locals.var_sp_ov_ygf_dn9 = assign23950_e23872_d_n9;
        locals.var_sp_ov_ygf_rv = 0.0;

        let (assign23960_e23885, assign23960_e23885_d_n4, assign23960_e23885_d_n6, assign23960_e23885_d_n7, assign23960_e23885_d_n8, assign23960_e23885_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) {
        let assign23960_e23881: f64 = (1.25 * locals.var_sp_ov_ygf);
        let assign23960_e23883: f64 = (assign23960_e23881 * locals.var_inv_xi_ov);
        (assign23960_e23883, (((1.25 * locals.var_sp_ov_ygf_dn4) * locals.var_inv_xi_ov) + (assign23960_e23881 * locals.var_inv_xi_ov_dn4)), (((1.25 * locals.var_sp_ov_ygf_dn6) * locals.var_inv_xi_ov) + (assign23960_e23881 * locals.var_inv_xi_ov_dn6)), (((1.25 * locals.var_sp_ov_ygf_dn7) * locals.var_inv_xi_ov) + (assign23960_e23881 * locals.var_inv_xi_ov_dn7)), (((1.25 * locals.var_sp_ov_ygf_dn8) * locals.var_inv_xi_ov) + (assign23960_e23881 * locals.var_inv_xi_ov_dn8)), (((1.25 * locals.var_sp_ov_ygf_dn9) * locals.var_inv_xi_ov) + (assign23960_e23881 * locals.var_inv_xi_ov_dn9)),)
    } else {
        (locals.var_sp_ov_z, locals.var_sp_ov_z_dn4, locals.var_sp_ov_z_dn6, locals.var_sp_ov_z_dn7, locals.var_sp_ov_z_dn8, locals.var_sp_ov_z_dn9,)
    }
};
        locals.var_sp_ov_z = assign23960_e23885;
        locals.var_sp_ov_z_dn4 = assign23960_e23885_d_n4;
        locals.var_sp_ov_z_dn6 = assign23960_e23885_d_n6;
        locals.var_sp_ov_z_dn7 = assign23960_e23885_d_n7;
        locals.var_sp_ov_z_dn8 = assign23960_e23885_d_n8;
        locals.var_sp_ov_z_dn9 = assign23960_e23885_d_n9;
        locals.var_sp_ov_z_rv = 0.0;

        let (assign23970_e23909, assign23970_e23909_d_n4, assign23970_e23909_d_n6, assign23970_e23909_d_n7, assign23970_e23909_d_n8, assign23970_e23909_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) {
        let assign23970_e23895: f64 = (locals.var_sp_ov_z + 10.0);
        let assign23970_e23898: f64 = (locals.var_sp_ov_z - 6.0);
        let assign23970_e23901: f64 = (locals.var_sp_ov_z - 6.0);
        let assign23970_e23902: f64 = (assign23970_e23898 * assign23970_e23901);
        let assign23970_e23904: f64 = (assign23970_e23902 + 64.0);
        let assign23970_e23905: f64 = (assign23970_e23904).sqrt();
        let assign23970_e23906: f64 = (assign23970_e23895 - assign23970_e23905);
        let assign23970_e23907: f64 = (0.5 * assign23970_e23906);
        (assign23970_e23907, (0.5 * (locals.var_sp_ov_z_dn4 - (((locals.var_sp_ov_z_dn4 * assign23970_e23901) + (assign23970_e23898 * locals.var_sp_ov_z_dn4)) / (2.0 * assign23970_e23905)))), (0.5 * (locals.var_sp_ov_z_dn6 - (((locals.var_sp_ov_z_dn6 * assign23970_e23901) + (assign23970_e23898 * locals.var_sp_ov_z_dn6)) / (2.0 * assign23970_e23905)))), (0.5 * (locals.var_sp_ov_z_dn7 - (((locals.var_sp_ov_z_dn7 * assign23970_e23901) + (assign23970_e23898 * locals.var_sp_ov_z_dn7)) / (2.0 * assign23970_e23905)))), (0.5 * (locals.var_sp_ov_z_dn8 - (((locals.var_sp_ov_z_dn8 * assign23970_e23901) + (assign23970_e23898 * locals.var_sp_ov_z_dn8)) / (2.0 * assign23970_e23905)))), (0.5 * (locals.var_sp_ov_z_dn9 - (((locals.var_sp_ov_z_dn9 * assign23970_e23901) + (assign23970_e23898 * locals.var_sp_ov_z_dn9)) / (2.0 * assign23970_e23905)))),)
    } else {
        (locals.var_sp_ov_eta, locals.var_sp_ov_eta_dn4, locals.var_sp_ov_eta_dn6, locals.var_sp_ov_eta_dn7, locals.var_sp_ov_eta_dn8, locals.var_sp_ov_eta_dn9,)
    }
};
        locals.var_sp_ov_eta = assign23970_e23909;
        locals.var_sp_ov_eta_dn4 = assign23970_e23909_d_n4;
        locals.var_sp_ov_eta_dn6 = assign23970_e23909_d_n6;
        locals.var_sp_ov_eta_dn7 = assign23970_e23909_d_n7;
        locals.var_sp_ov_eta_dn8 = assign23970_e23909_d_n8;
        locals.var_sp_ov_eta_dn9 = assign23970_e23909_d_n9;
        locals.var_sp_ov_eta_rv = 0.0;

        let (assign23980_e23930, assign23980_e23930_d_n4, assign23980_e23930_d_n6, assign23980_e23930_d_n7, assign23980_e23930_d_n8, assign23980_e23930_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) {
        let assign23980_e23918: f64 = (locals.var_sp_ov_ygf - locals.var_sp_ov_eta);
        let assign23980_e23921: f64 = (locals.var_sp_ov_ygf - locals.var_sp_ov_eta);
        let assign23980_e23922: f64 = (assign23980_e23918 * assign23980_e23921);
        let assign23980_e23926: f64 = (locals.var_sp_ov_eta + 1.0);
        let assign23980_e23927: f64 = (locals.var_gov2 * assign23980_e23926);
        let assign23980_e23928: f64 = (assign23980_e23922 + assign23980_e23927);
        (assign23980_e23928, ((((locals.var_sp_ov_ygf_dn4 - locals.var_sp_ov_eta_dn4) * assign23980_e23921) + (assign23980_e23918 * (locals.var_sp_ov_ygf_dn4 - locals.var_sp_ov_eta_dn4))) + ((locals.var_gov2_dn4 * assign23980_e23926) + (locals.var_gov2 * locals.var_sp_ov_eta_dn4))), ((((locals.var_sp_ov_ygf_dn6 - locals.var_sp_ov_eta_dn6) * assign23980_e23921) + (assign23980_e23918 * (locals.var_sp_ov_ygf_dn6 - locals.var_sp_ov_eta_dn6))) + ((locals.var_gov2_dn6 * assign23980_e23926) + (locals.var_gov2 * locals.var_sp_ov_eta_dn6))), ((((locals.var_sp_ov_ygf_dn7 - locals.var_sp_ov_eta_dn7) * assign23980_e23921) + (assign23980_e23918 * (locals.var_sp_ov_ygf_dn7 - locals.var_sp_ov_eta_dn7))) + ((locals.var_gov2_dn7 * assign23980_e23926) + (locals.var_gov2 * locals.var_sp_ov_eta_dn7))), ((((locals.var_sp_ov_ygf_dn8 - locals.var_sp_ov_eta_dn8) * assign23980_e23921) + (assign23980_e23918 * (locals.var_sp_ov_ygf_dn8 - locals.var_sp_ov_eta_dn8))) + ((locals.var_gov2_dn8 * assign23980_e23926) + (locals.var_gov2 * locals.var_sp_ov_eta_dn8))), ((((locals.var_sp_ov_ygf_dn9 - locals.var_sp_ov_eta_dn9) * assign23980_e23921) + (assign23980_e23918 * (locals.var_sp_ov_ygf_dn9 - locals.var_sp_ov_eta_dn9))) + ((locals.var_gov2_dn9 * assign23980_e23926) + (locals.var_gov2 * locals.var_sp_ov_eta_dn9))),)
    } else {
        (locals.var_sp_ov_a, locals.var_sp_ov_a_dn4, locals.var_sp_ov_a_dn6, locals.var_sp_ov_a_dn7, locals.var_sp_ov_a_dn8, locals.var_sp_ov_a_dn9,)
    }
};
        locals.var_sp_ov_a = assign23980_e23930;
        locals.var_sp_ov_a_dn4 = assign23980_e23930_d_n4;
        locals.var_sp_ov_a_dn6 = assign23980_e23930_d_n6;
        locals.var_sp_ov_a_dn7 = assign23980_e23930_d_n7;
        locals.var_sp_ov_a_dn8 = assign23980_e23930_d_n8;
        locals.var_sp_ov_a_dn9 = assign23980_e23930_d_n9;
        locals.var_sp_ov_a_rv = 0.0;

        let (assign23990_e23945, assign23990_e23945_d_n4, assign23990_e23945_d_n6, assign23990_e23945_d_n7, assign23990_e23945_d_n8, assign23990_e23945_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) {
        let assign23990_e23940: f64 = (locals.var_sp_ov_ygf - locals.var_sp_ov_eta);
        let assign23990_e23941: f64 = (2.0 * assign23990_e23940);
        let assign23990_e23943: f64 = (assign23990_e23941 - locals.var_gov2);
        (assign23990_e23943, ((2.0 * (locals.var_sp_ov_ygf_dn4 - locals.var_sp_ov_eta_dn4)) - locals.var_gov2_dn4), ((2.0 * (locals.var_sp_ov_ygf_dn6 - locals.var_sp_ov_eta_dn6)) - locals.var_gov2_dn6), ((2.0 * (locals.var_sp_ov_ygf_dn7 - locals.var_sp_ov_eta_dn7)) - locals.var_gov2_dn7), ((2.0 * (locals.var_sp_ov_ygf_dn8 - locals.var_sp_ov_eta_dn8)) - locals.var_gov2_dn8), ((2.0 * (locals.var_sp_ov_ygf_dn9 - locals.var_sp_ov_eta_dn9)) - locals.var_gov2_dn9),)
    } else {
        (locals.var_sp_ov_c, locals.var_sp_ov_c_dn4, locals.var_sp_ov_c_dn6, locals.var_sp_ov_c_dn7, locals.var_sp_ov_c_dn8, locals.var_sp_ov_c_dn9,)
    }
};
        locals.var_sp_ov_c = assign23990_e23945;
        locals.var_sp_ov_c_dn4 = assign23990_e23945_d_n4;
        locals.var_sp_ov_c_dn6 = assign23990_e23945_d_n6;
        locals.var_sp_ov_c_dn7 = assign23990_e23945_d_n7;
        locals.var_sp_ov_c_dn8 = assign23990_e23945_d_n8;
        locals.var_sp_ov_c_dn9 = assign23990_e23945_d_n9;
        locals.var_sp_ov_c_rv = 0.0;

        let (assign24000_e23959, assign24000_e23959_d_n4, assign24000_e23959_d_n6, assign24000_e23959_d_n7, assign24000_e23959_d_n8, assign24000_e23959_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) {
        let assign24000_e23954: f64 = (locals.var_sp_ov_a / locals.var_gov2);
        let assign24000_e23955: f64 = (assign24000_e23954).ln();
        let assign24000_e23957: f64 = (assign24000_e23955 - locals.var_sp_ov_eta);
        (assign24000_e23957, (((((locals.var_sp_ov_a_dn4 * locals.var_gov2) - (locals.var_sp_ov_a * locals.var_gov2_dn4)) / (locals.var_gov2 * locals.var_gov2)) / assign24000_e23954) - locals.var_sp_ov_eta_dn4), (((((locals.var_sp_ov_a_dn6 * locals.var_gov2) - (locals.var_sp_ov_a * locals.var_gov2_dn6)) / (locals.var_gov2 * locals.var_gov2)) / assign24000_e23954) - locals.var_sp_ov_eta_dn6), (((((locals.var_sp_ov_a_dn7 * locals.var_gov2) - (locals.var_sp_ov_a * locals.var_gov2_dn7)) / (locals.var_gov2 * locals.var_gov2)) / assign24000_e23954) - locals.var_sp_ov_eta_dn7), (((((locals.var_sp_ov_a_dn8 * locals.var_gov2) - (locals.var_sp_ov_a * locals.var_gov2_dn8)) / (locals.var_gov2 * locals.var_gov2)) / assign24000_e23954) - locals.var_sp_ov_eta_dn8), (((((locals.var_sp_ov_a_dn9 * locals.var_gov2) - (locals.var_sp_ov_a * locals.var_gov2_dn9)) / (locals.var_gov2 * locals.var_gov2)) / assign24000_e23954) - locals.var_sp_ov_eta_dn9),)
    } else {
        (locals.var_sp_ov_tau, locals.var_sp_ov_tau_dn4, locals.var_sp_ov_tau_dn6, locals.var_sp_ov_tau_dn7, locals.var_sp_ov_tau_dn8, locals.var_sp_ov_tau_dn9,)
    }
};
        locals.var_sp_ov_tau = assign24000_e23959;
        locals.var_sp_ov_tau_dn4 = assign24000_e23959_d_n4;
        locals.var_sp_ov_tau_dn6 = assign24000_e23959_d_n6;
        locals.var_sp_ov_tau_dn7 = assign24000_e23959_d_n7;
        locals.var_sp_ov_tau_dn8 = assign24000_e23959_d_n8;
        locals.var_sp_ov_tau_dn9 = assign24000_e23959_d_n9;
        locals.var_sp_ov_tau_rv = 0.0;

        let (assign24010_e23970, assign24010_e23970_d_n4, assign24010_e23970_d_n6, assign24010_e23970_d_n7, assign24010_e23970_d_n8, assign24010_e23970_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) {
        let assign24010_e23968: f64 = (locals.var_sp_ov_a + locals.var_sp_ov_c);
        (assign24010_e23968, (locals.var_sp_ov_a_dn4 + locals.var_sp_ov_c_dn4), (locals.var_sp_ov_a_dn6 + locals.var_sp_ov_c_dn6), (locals.var_sp_ov_a_dn7 + locals.var_sp_ov_c_dn7), (locals.var_sp_ov_a_dn8 + locals.var_sp_ov_c_dn8), (locals.var_sp_ov_a_dn9 + locals.var_sp_ov_c_dn9),)
    } else {
        (locals.var_sp_ov_nu, locals.var_sp_ov_nu_dn4, locals.var_sp_ov_nu_dn6, locals.var_sp_ov_nu_dn7, locals.var_sp_ov_nu_dn8, locals.var_sp_ov_nu_dn9,)
    }
};
        locals.var_sp_ov_nu = assign24010_e23970;
        locals.var_sp_ov_nu_dn4 = assign24010_e23970_d_n4;
        locals.var_sp_ov_nu_dn6 = assign24010_e23970_d_n6;
        locals.var_sp_ov_nu_dn7 = assign24010_e23970_d_n7;
        locals.var_sp_ov_nu_dn8 = assign24010_e23970_d_n8;
        locals.var_sp_ov_nu_dn9 = assign24010_e23970_d_n9;
        locals.var_sp_ov_nu_rv = 0.0;

        let (assign24020_e23991, assign24020_e23991_d_n4, assign24020_e23991_d_n6, assign24020_e23991_d_n7, assign24020_e23991_d_n8, assign24020_e23991_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) {
        let assign24020_e23979: f64 = (locals.var_sp_ov_nu * locals.var_sp_ov_nu);
        let assign24020_e23983: f64 = (0.5 * locals.var_sp_ov_c);
        let assign24020_e23985: f64 = (assign24020_e23983 * locals.var_sp_ov_c);
        let assign24020_e23987: f64 = (assign24020_e23985 - locals.var_sp_ov_a);
        let assign24020_e23988: f64 = (locals.var_sp_ov_tau * assign24020_e23987);
        let assign24020_e23989: f64 = (assign24020_e23979 + assign24020_e23988);
        (assign24020_e23989, (((locals.var_sp_ov_nu_dn4 * locals.var_sp_ov_nu) + (locals.var_sp_ov_nu * locals.var_sp_ov_nu_dn4)) + ((locals.var_sp_ov_tau_dn4 * assign24020_e23987) + (locals.var_sp_ov_tau * ((((0.5 * locals.var_sp_ov_c_dn4) * locals.var_sp_ov_c) + (assign24020_e23983 * locals.var_sp_ov_c_dn4)) - locals.var_sp_ov_a_dn4)))), (((locals.var_sp_ov_nu_dn6 * locals.var_sp_ov_nu) + (locals.var_sp_ov_nu * locals.var_sp_ov_nu_dn6)) + ((locals.var_sp_ov_tau_dn6 * assign24020_e23987) + (locals.var_sp_ov_tau * ((((0.5 * locals.var_sp_ov_c_dn6) * locals.var_sp_ov_c) + (assign24020_e23983 * locals.var_sp_ov_c_dn6)) - locals.var_sp_ov_a_dn6)))), (((locals.var_sp_ov_nu_dn7 * locals.var_sp_ov_nu) + (locals.var_sp_ov_nu * locals.var_sp_ov_nu_dn7)) + ((locals.var_sp_ov_tau_dn7 * assign24020_e23987) + (locals.var_sp_ov_tau * ((((0.5 * locals.var_sp_ov_c_dn7) * locals.var_sp_ov_c) + (assign24020_e23983 * locals.var_sp_ov_c_dn7)) - locals.var_sp_ov_a_dn7)))), (((locals.var_sp_ov_nu_dn8 * locals.var_sp_ov_nu) + (locals.var_sp_ov_nu * locals.var_sp_ov_nu_dn8)) + ((locals.var_sp_ov_tau_dn8 * assign24020_e23987) + (locals.var_sp_ov_tau * ((((0.5 * locals.var_sp_ov_c_dn8) * locals.var_sp_ov_c) + (assign24020_e23983 * locals.var_sp_ov_c_dn8)) - locals.var_sp_ov_a_dn8)))), (((locals.var_sp_ov_nu_dn9 * locals.var_sp_ov_nu) + (locals.var_sp_ov_nu * locals.var_sp_ov_nu_dn9)) + ((locals.var_sp_ov_tau_dn9 * assign24020_e23987) + (locals.var_sp_ov_tau * ((((0.5 * locals.var_sp_ov_c_dn9) * locals.var_sp_ov_c) + (assign24020_e23983 * locals.var_sp_ov_c_dn9)) - locals.var_sp_ov_a_dn9)))),)
    } else {
        (locals.var_sp_ov_mutau, locals.var_sp_ov_mutau_dn4, locals.var_sp_ov_mutau_dn6, locals.var_sp_ov_mutau_dn7, locals.var_sp_ov_mutau_dn8, locals.var_sp_ov_mutau_dn9,)
    }
};
        locals.var_sp_ov_mutau = assign24020_e23991;
        locals.var_sp_ov_mutau_dn4 = assign24020_e23991_d_n4;
        locals.var_sp_ov_mutau_dn6 = assign24020_e23991_d_n6;
        locals.var_sp_ov_mutau_dn7 = assign24020_e23991_d_n7;
        locals.var_sp_ov_mutau_dn8 = assign24020_e23991_d_n8;
        locals.var_sp_ov_mutau_dn9 = assign24020_e23991_d_n9;
        locals.var_sp_ov_mutau_rv = 0.0;

        let (assign24030_e24018, assign24030_e24018_d_n4, assign24030_e24018_d_n6, assign24030_e24018_d_n7, assign24030_e24018_d_n8, assign24030_e24018_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) {
        let assign24030_e24001: f64 = (locals.var_sp_ov_nu / locals.var_sp_ov_mutau);
        let assign24030_e24003: f64 = (assign24030_e24001 * locals.var_sp_ov_tau);
        let assign24030_e24005: f64 = (assign24030_e24003 * locals.var_sp_ov_tau);
        let assign24030_e24007: f64 = (assign24030_e24005 * locals.var_sp_ov_c);
        let assign24030_e24010: f64 = (locals.var_sp_ov_c * locals.var_sp_ov_c);
        let assign24030_e24012: f64 = (assign24030_e24010 * 0.3333333333333);
        let assign24030_e24014: f64 = (assign24030_e24012 - locals.var_sp_ov_a);
        let assign24030_e24015: f64 = (assign24030_e24007 * assign24030_e24014);
        let assign24030_e24016: f64 = (locals.var_sp_ov_mutau + assign24030_e24015);
        (assign24030_e24016, (locals.var_sp_ov_mutau_dn4 + (((((((((((locals.var_sp_ov_nu_dn4 * locals.var_sp_ov_mutau) - (locals.var_sp_ov_nu * locals.var_sp_ov_mutau_dn4)) / (locals.var_sp_ov_mutau * locals.var_sp_ov_mutau)) * locals.var_sp_ov_tau) + (assign24030_e24001 * locals.var_sp_ov_tau_dn4)) * locals.var_sp_ov_tau) + (assign24030_e24003 * locals.var_sp_ov_tau_dn4)) * locals.var_sp_ov_c) + (assign24030_e24005 * locals.var_sp_ov_c_dn4)) * assign24030_e24014) + (assign24030_e24007 * ((((locals.var_sp_ov_c_dn4 * locals.var_sp_ov_c) + (locals.var_sp_ov_c * locals.var_sp_ov_c_dn4)) * 0.3333333333333) - locals.var_sp_ov_a_dn4)))), (locals.var_sp_ov_mutau_dn6 + (((((((((((locals.var_sp_ov_nu_dn6 * locals.var_sp_ov_mutau) - (locals.var_sp_ov_nu * locals.var_sp_ov_mutau_dn6)) / (locals.var_sp_ov_mutau * locals.var_sp_ov_mutau)) * locals.var_sp_ov_tau) + (assign24030_e24001 * locals.var_sp_ov_tau_dn6)) * locals.var_sp_ov_tau) + (assign24030_e24003 * locals.var_sp_ov_tau_dn6)) * locals.var_sp_ov_c) + (assign24030_e24005 * locals.var_sp_ov_c_dn6)) * assign24030_e24014) + (assign24030_e24007 * ((((locals.var_sp_ov_c_dn6 * locals.var_sp_ov_c) + (locals.var_sp_ov_c * locals.var_sp_ov_c_dn6)) * 0.3333333333333) - locals.var_sp_ov_a_dn6)))), (locals.var_sp_ov_mutau_dn7 + (((((((((((locals.var_sp_ov_nu_dn7 * locals.var_sp_ov_mutau) - (locals.var_sp_ov_nu * locals.var_sp_ov_mutau_dn7)) / (locals.var_sp_ov_mutau * locals.var_sp_ov_mutau)) * locals.var_sp_ov_tau) + (assign24030_e24001 * locals.var_sp_ov_tau_dn7)) * locals.var_sp_ov_tau) + (assign24030_e24003 * locals.var_sp_ov_tau_dn7)) * locals.var_sp_ov_c) + (assign24030_e24005 * locals.var_sp_ov_c_dn7)) * assign24030_e24014) + (assign24030_e24007 * ((((locals.var_sp_ov_c_dn7 * locals.var_sp_ov_c) + (locals.var_sp_ov_c * locals.var_sp_ov_c_dn7)) * 0.3333333333333) - locals.var_sp_ov_a_dn7)))), (locals.var_sp_ov_mutau_dn8 + (((((((((((locals.var_sp_ov_nu_dn8 * locals.var_sp_ov_mutau) - (locals.var_sp_ov_nu * locals.var_sp_ov_mutau_dn8)) / (locals.var_sp_ov_mutau * locals.var_sp_ov_mutau)) * locals.var_sp_ov_tau) + (assign24030_e24001 * locals.var_sp_ov_tau_dn8)) * locals.var_sp_ov_tau) + (assign24030_e24003 * locals.var_sp_ov_tau_dn8)) * locals.var_sp_ov_c) + (assign24030_e24005 * locals.var_sp_ov_c_dn8)) * assign24030_e24014) + (assign24030_e24007 * ((((locals.var_sp_ov_c_dn8 * locals.var_sp_ov_c) + (locals.var_sp_ov_c * locals.var_sp_ov_c_dn8)) * 0.3333333333333) - locals.var_sp_ov_a_dn8)))), (locals.var_sp_ov_mutau_dn9 + (((((((((((locals.var_sp_ov_nu_dn9 * locals.var_sp_ov_mutau) - (locals.var_sp_ov_nu * locals.var_sp_ov_mutau_dn9)) / (locals.var_sp_ov_mutau * locals.var_sp_ov_mutau)) * locals.var_sp_ov_tau) + (assign24030_e24001 * locals.var_sp_ov_tau_dn9)) * locals.var_sp_ov_tau) + (assign24030_e24003 * locals.var_sp_ov_tau_dn9)) * locals.var_sp_ov_c) + (assign24030_e24005 * locals.var_sp_ov_c_dn9)) * assign24030_e24014) + (assign24030_e24007 * ((((locals.var_sp_ov_c_dn9 * locals.var_sp_ov_c) + (locals.var_sp_ov_c * locals.var_sp_ov_c_dn9)) * 0.3333333333333) - locals.var_sp_ov_a_dn9)))),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign24030_e24018;
        locals.var_sp_ov_temp_dn4 = assign24030_e24018_d_n4;
        locals.var_sp_ov_temp_dn6 = assign24030_e24018_d_n6;
        locals.var_sp_ov_temp_dn7 = assign24030_e24018_d_n7;
        locals.var_sp_ov_temp_dn8 = assign24030_e24018_d_n8;
        locals.var_sp_ov_temp_dn9 = assign24030_e24018_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let (assign24040_e24035, assign24040_e24035_d_n4, assign24040_e24035_d_n6, assign24040_e24035_d_n7, assign24040_e24035_d_n8, assign24040_e24035_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) {
        let assign24040_e24028: f64 = (locals.var_sp_ov_a * locals.var_sp_ov_nu);
        let assign24040_e24030: f64 = (assign24040_e24028 * locals.var_sp_ov_tau);
        let assign24040_e24032: f64 = (assign24040_e24030 / locals.var_sp_ov_temp);
        let assign24040_e24033: f64 = (locals.var_sp_ov_eta + assign24040_e24032);
        (assign24040_e24033, (locals.var_sp_ov_eta_dn4 + (((((((locals.var_sp_ov_a_dn4 * locals.var_sp_ov_nu) + (locals.var_sp_ov_a * locals.var_sp_ov_nu_dn4)) * locals.var_sp_ov_tau) + (assign24040_e24028 * locals.var_sp_ov_tau_dn4)) * locals.var_sp_ov_temp) - (assign24040_e24030 * locals.var_sp_ov_temp_dn4)) / (locals.var_sp_ov_temp * locals.var_sp_ov_temp))), (locals.var_sp_ov_eta_dn6 + (((((((locals.var_sp_ov_a_dn6 * locals.var_sp_ov_nu) + (locals.var_sp_ov_a * locals.var_sp_ov_nu_dn6)) * locals.var_sp_ov_tau) + (assign24040_e24028 * locals.var_sp_ov_tau_dn6)) * locals.var_sp_ov_temp) - (assign24040_e24030 * locals.var_sp_ov_temp_dn6)) / (locals.var_sp_ov_temp * locals.var_sp_ov_temp))), (locals.var_sp_ov_eta_dn7 + (((((((locals.var_sp_ov_a_dn7 * locals.var_sp_ov_nu) + (locals.var_sp_ov_a * locals.var_sp_ov_nu_dn7)) * locals.var_sp_ov_tau) + (assign24040_e24028 * locals.var_sp_ov_tau_dn7)) * locals.var_sp_ov_temp) - (assign24040_e24030 * locals.var_sp_ov_temp_dn7)) / (locals.var_sp_ov_temp * locals.var_sp_ov_temp))), (locals.var_sp_ov_eta_dn8 + (((((((locals.var_sp_ov_a_dn8 * locals.var_sp_ov_nu) + (locals.var_sp_ov_a * locals.var_sp_ov_nu_dn8)) * locals.var_sp_ov_tau) + (assign24040_e24028 * locals.var_sp_ov_tau_dn8)) * locals.var_sp_ov_temp) - (assign24040_e24030 * locals.var_sp_ov_temp_dn8)) / (locals.var_sp_ov_temp * locals.var_sp_ov_temp))), (locals.var_sp_ov_eta_dn9 + (((((((locals.var_sp_ov_a_dn9 * locals.var_sp_ov_nu) + (locals.var_sp_ov_a * locals.var_sp_ov_nu_dn9)) * locals.var_sp_ov_tau) + (assign24040_e24028 * locals.var_sp_ov_tau_dn9)) * locals.var_sp_ov_temp) - (assign24040_e24030 * locals.var_sp_ov_temp_dn9)) / (locals.var_sp_ov_temp * locals.var_sp_ov_temp))),)
    } else {
        (locals.var_sp_ov_y0, locals.var_sp_ov_y0_dn4, locals.var_sp_ov_y0_dn6, locals.var_sp_ov_y0_dn7, locals.var_sp_ov_y0_dn8, locals.var_sp_ov_y0_dn9,)
    }
};
        locals.var_sp_ov_y0 = assign24040_e24035;
        locals.var_sp_ov_y0_dn4 = assign24040_e24035_d_n4;
        locals.var_sp_ov_y0_dn6 = assign24040_e24035_d_n6;
        locals.var_sp_ov_y0_dn7 = assign24040_e24035_d_n7;
        locals.var_sp_ov_y0_dn8 = assign24040_e24035_d_n8;
        locals.var_sp_ov_y0_dn9 = assign24040_e24035_d_n9;
        locals.var_sp_ov_y0_rv = 0.0;

        let assign24050_e24037: f64 = (locals.var_sp_ov_y0).abs();
        let assign24050_e24039: f64 = if assign24050_e24037 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard697 = assign24050_e24039;
        locals.var_guard697_rv = 0.0;

        let (assign24060_e24051, assign24060_e24051_d_n4, assign24060_e24051_d_n6, assign24060_e24051_d_n7, assign24060_e24051_d_n8, assign24060_e24051_d_n9,) = {
    if ((((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) && (locals.var_guard697 != 0.0)) {
        let assign24060_e24049: f64 = (locals.var_sp_ov_y0).exp();
        (assign24060_e24049, (assign24060_e24049 * locals.var_sp_ov_y0_dn4), (assign24060_e24049 * locals.var_sp_ov_y0_dn6), (assign24060_e24049 * locals.var_sp_ov_y0_dn7), (assign24060_e24049 * locals.var_sp_ov_y0_dn8), (assign24060_e24049 * locals.var_sp_ov_y0_dn9),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign24060_e24051;
        locals.var_sp_ov_d0_dn4 = assign24060_e24051_d_n4;
        locals.var_sp_ov_d0_dn6 = assign24060_e24051_d_n6;
        locals.var_sp_ov_d0_dn7 = assign24060_e24051_d_n7;
        locals.var_sp_ov_d0_dn8 = assign24060_e24051_d_n8;
        locals.var_sp_ov_d0_dn9 = assign24060_e24051_d_n9;
        locals.var_sp_ov_d0_rv = 0.0;

        let assign24070_e24054: f64 = (-80.0);
        let assign24070_e24055: f64 = if locals.var_sp_ov_y0 < assign24070_e24054 { 1.0 } else { 0.0 };
        locals.var_guard698 = assign24070_e24055;
        locals.var_guard698_rv = 0.0;

        let (assign24080_e24094, assign24080_e24094_d_n4, assign24080_e24094_d_n6, assign24080_e24094_d_n7, assign24080_e24094_d_n8, assign24080_e24094_d_n9,) = {
    if (((((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) && (locals.var_guard697 == 0.0)) && (locals.var_guard698 != 0.0)) {
        let assign24080_e24070: f64 = (-locals.var_sp_ov_y0);
        let assign24080_e24072: f64 = (assign24080_e24070 - 80.0);
        let assign24080_e24076: f64 = (-locals.var_sp_ov_y0);
        let assign24080_e24078: f64 = (assign24080_e24076 - 80.0);
        let assign24080_e24079: f64 = (0.5 * assign24080_e24078);
        let assign24080_e24082: f64 = (-locals.var_sp_ov_y0);
        let assign24080_e24084: f64 = (assign24080_e24082 - 80.0);
        let assign24080_e24086: f64 = (assign24080_e24084 * 0.3333333333333);
        let assign24080_e24087: f64 = (1.0 + assign24080_e24086);
        let assign24080_e24088: f64 = (assign24080_e24079 * assign24080_e24087);
        let assign24080_e24089: f64 = (1.0 + assign24080_e24088);
        let assign24080_e24090: f64 = (assign24080_e24072 * assign24080_e24089);
        let assign24080_e24091: f64 = (1.0 + assign24080_e24090);
        let assign24080_e24092: f64 = (1.80485e-35 / assign24080_e24091);
        (assign24080_e24092, (-((1.80485e-35 * (((-locals.var_sp_ov_y0_dn4) * assign24080_e24089) + (assign24080_e24072 * (((0.5 * (-locals.var_sp_ov_y0_dn4)) * assign24080_e24087) + (assign24080_e24079 * ((-locals.var_sp_ov_y0_dn4) * 0.3333333333333)))))) / (assign24080_e24091 * assign24080_e24091))), (-((1.80485e-35 * (((-locals.var_sp_ov_y0_dn6) * assign24080_e24089) + (assign24080_e24072 * (((0.5 * (-locals.var_sp_ov_y0_dn6)) * assign24080_e24087) + (assign24080_e24079 * ((-locals.var_sp_ov_y0_dn6) * 0.3333333333333)))))) / (assign24080_e24091 * assign24080_e24091))), (-((1.80485e-35 * (((-locals.var_sp_ov_y0_dn7) * assign24080_e24089) + (assign24080_e24072 * (((0.5 * (-locals.var_sp_ov_y0_dn7)) * assign24080_e24087) + (assign24080_e24079 * ((-locals.var_sp_ov_y0_dn7) * 0.3333333333333)))))) / (assign24080_e24091 * assign24080_e24091))), (-((1.80485e-35 * (((-locals.var_sp_ov_y0_dn8) * assign24080_e24089) + (assign24080_e24072 * (((0.5 * (-locals.var_sp_ov_y0_dn8)) * assign24080_e24087) + (assign24080_e24079 * ((-locals.var_sp_ov_y0_dn8) * 0.3333333333333)))))) / (assign24080_e24091 * assign24080_e24091))), (-((1.80485e-35 * (((-locals.var_sp_ov_y0_dn9) * assign24080_e24089) + (assign24080_e24072 * (((0.5 * (-locals.var_sp_ov_y0_dn9)) * assign24080_e24087) + (assign24080_e24079 * ((-locals.var_sp_ov_y0_dn9) * 0.3333333333333)))))) / (assign24080_e24091 * assign24080_e24091))),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign24080_e24094;
        locals.var_sp_ov_d0_dn4 = assign24080_e24094_d_n4;
        locals.var_sp_ov_d0_dn6 = assign24080_e24094_d_n6;
        locals.var_sp_ov_d0_dn7 = assign24080_e24094_d_n7;
        locals.var_sp_ov_d0_dn8 = assign24080_e24094_d_n8;
        locals.var_sp_ov_d0_dn9 = assign24080_e24094_d_n9;
        locals.var_sp_ov_d0_rv = 0.0;

        let (assign24090_e24131, assign24090_e24131_d_n4, assign24090_e24131_d_n6, assign24090_e24131_d_n7, assign24090_e24131_d_n8, assign24090_e24131_d_n9,) = {
    if (((((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) && (locals.var_guard697 == 0.0)) && (locals.var_guard698 == 0.0)) {
        let assign24090_e24111: f64 = (locals.var_sp_ov_y0 - 80.0);
        let assign24090_e24116: f64 = (locals.var_sp_ov_y0 - 80.0);
        let assign24090_e24117: f64 = (0.5 * assign24090_e24116);
        let assign24090_e24121: f64 = (locals.var_sp_ov_y0 - 80.0);
        let assign24090_e24123: f64 = (assign24090_e24121 * 0.3333333333333);
        let assign24090_e24124: f64 = (1.0 + assign24090_e24123);
        let assign24090_e24125: f64 = (assign24090_e24117 * assign24090_e24124);
        let assign24090_e24126: f64 = (1.0 + assign24090_e24125);
        let assign24090_e24127: f64 = (assign24090_e24111 * assign24090_e24126);
        let assign24090_e24128: f64 = (1.0 + assign24090_e24127);
        let assign24090_e24129: f64 = (5.54062e34 * assign24090_e24128);
        (assign24090_e24129, (5.54062e34 * ((locals.var_sp_ov_y0_dn4 * assign24090_e24126) + (assign24090_e24111 * (((0.5 * locals.var_sp_ov_y0_dn4) * assign24090_e24124) + (assign24090_e24117 * (locals.var_sp_ov_y0_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_sp_ov_y0_dn6 * assign24090_e24126) + (assign24090_e24111 * (((0.5 * locals.var_sp_ov_y0_dn6) * assign24090_e24124) + (assign24090_e24117 * (locals.var_sp_ov_y0_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_sp_ov_y0_dn7 * assign24090_e24126) + (assign24090_e24111 * (((0.5 * locals.var_sp_ov_y0_dn7) * assign24090_e24124) + (assign24090_e24117 * (locals.var_sp_ov_y0_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_sp_ov_y0_dn8 * assign24090_e24126) + (assign24090_e24111 * (((0.5 * locals.var_sp_ov_y0_dn8) * assign24090_e24124) + (assign24090_e24117 * (locals.var_sp_ov_y0_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_sp_ov_y0_dn9 * assign24090_e24126) + (assign24090_e24111 * (((0.5 * locals.var_sp_ov_y0_dn9) * assign24090_e24124) + (assign24090_e24117 * (locals.var_sp_ov_y0_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign24090_e24131;
        locals.var_sp_ov_d0_dn4 = assign24090_e24131_d_n4;
        locals.var_sp_ov_d0_dn6 = assign24090_e24131_d_n6;
        locals.var_sp_ov_d0_dn7 = assign24090_e24131_d_n7;
        locals.var_sp_ov_d0_dn8 = assign24090_e24131_d_n8;
        locals.var_sp_ov_d0_dn9 = assign24090_e24131_d_n9;
        locals.var_sp_ov_d0_rv = 0.0;

        let (assign24100_e24142, assign24100_e24142_d_n4, assign24100_e24142_d_n6, assign24100_e24142_d_n7, assign24100_e24142_d_n8, assign24100_e24142_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) {
        let assign24100_e24140: f64 = (locals.var_sp_ov_ygf - locals.var_sp_ov_y0);
        (assign24100_e24140, (locals.var_sp_ov_ygf_dn4 - locals.var_sp_ov_y0_dn4), (locals.var_sp_ov_ygf_dn6 - locals.var_sp_ov_y0_dn6), (locals.var_sp_ov_ygf_dn7 - locals.var_sp_ov_y0_dn7), (locals.var_sp_ov_ygf_dn8 - locals.var_sp_ov_y0_dn8), (locals.var_sp_ov_ygf_dn9 - locals.var_sp_ov_y0_dn9),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign24100_e24142;
        locals.var_sp_ov_temp_dn4 = assign24100_e24142_d_n4;
        locals.var_sp_ov_temp_dn6 = assign24100_e24142_d_n6;
        locals.var_sp_ov_temp_dn7 = assign24100_e24142_d_n7;
        locals.var_sp_ov_temp_dn8 = assign24100_e24142_d_n8;
        locals.var_sp_ov_temp_dn9 = assign24100_e24142_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let (assign24110_e24159, assign24110_e24159_d_n4, assign24110_e24159_d_n6, assign24110_e24159_d_n7, assign24110_e24159_d_n8, assign24110_e24159_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) {
        let assign24110_e24151: f64 = (2.0 * locals.var_sp_ov_temp);
        let assign24110_e24155: f64 = (locals.var_sp_ov_d0 - 1.0);
        let assign24110_e24156: f64 = (locals.var_gov2 * assign24110_e24155);
        let assign24110_e24157: f64 = (assign24110_e24151 + assign24110_e24156);
        (assign24110_e24157, ((2.0 * locals.var_sp_ov_temp_dn4) + ((locals.var_gov2_dn4 * assign24110_e24155) + (locals.var_gov2 * locals.var_sp_ov_d0_dn4))), ((2.0 * locals.var_sp_ov_temp_dn6) + ((locals.var_gov2_dn6 * assign24110_e24155) + (locals.var_gov2 * locals.var_sp_ov_d0_dn6))), ((2.0 * locals.var_sp_ov_temp_dn7) + ((locals.var_gov2_dn7 * assign24110_e24155) + (locals.var_gov2 * locals.var_sp_ov_d0_dn7))), ((2.0 * locals.var_sp_ov_temp_dn8) + ((locals.var_gov2_dn8 * assign24110_e24155) + (locals.var_gov2 * locals.var_sp_ov_d0_dn8))), ((2.0 * locals.var_sp_ov_temp_dn9) + ((locals.var_gov2_dn9 * assign24110_e24155) + (locals.var_gov2 * locals.var_sp_ov_d0_dn9))),)
    } else {
        (locals.var_sp_ov_p, locals.var_sp_ov_p_dn4, locals.var_sp_ov_p_dn6, locals.var_sp_ov_p_dn7, locals.var_sp_ov_p_dn8, locals.var_sp_ov_p_dn9,)
    }
};
        locals.var_sp_ov_p = assign24110_e24159;
        locals.var_sp_ov_p_dn4 = assign24110_e24159_d_n4;
        locals.var_sp_ov_p_dn6 = assign24110_e24159_d_n6;
        locals.var_sp_ov_p_dn7 = assign24110_e24159_d_n7;
        locals.var_sp_ov_p_dn8 = assign24110_e24159_d_n8;
        locals.var_sp_ov_p_dn9 = assign24110_e24159_d_n9;
        locals.var_sp_ov_p_rv = 0.0;

        let (assign24120_e24178, assign24120_e24178_d_n4, assign24120_e24178_d_n6, assign24120_e24178_d_n7, assign24120_e24178_d_n8, assign24120_e24178_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) {
        let assign24120_e24168: f64 = (locals.var_sp_ov_temp * locals.var_sp_ov_temp);
        let assign24120_e24172: f64 = (locals.var_sp_ov_y0 + 1.0);
        let assign24120_e24174: f64 = (assign24120_e24172 - locals.var_sp_ov_d0);
        let assign24120_e24175: f64 = (locals.var_gov2 * assign24120_e24174);
        let assign24120_e24176: f64 = (assign24120_e24168 + assign24120_e24175);
        (assign24120_e24176, (((locals.var_sp_ov_temp_dn4 * locals.var_sp_ov_temp) + (locals.var_sp_ov_temp * locals.var_sp_ov_temp_dn4)) + ((locals.var_gov2_dn4 * assign24120_e24174) + (locals.var_gov2 * (locals.var_sp_ov_y0_dn4 - locals.var_sp_ov_d0_dn4)))), (((locals.var_sp_ov_temp_dn6 * locals.var_sp_ov_temp) + (locals.var_sp_ov_temp * locals.var_sp_ov_temp_dn6)) + ((locals.var_gov2_dn6 * assign24120_e24174) + (locals.var_gov2 * (locals.var_sp_ov_y0_dn6 - locals.var_sp_ov_d0_dn6)))), (((locals.var_sp_ov_temp_dn7 * locals.var_sp_ov_temp) + (locals.var_sp_ov_temp * locals.var_sp_ov_temp_dn7)) + ((locals.var_gov2_dn7 * assign24120_e24174) + (locals.var_gov2 * (locals.var_sp_ov_y0_dn7 - locals.var_sp_ov_d0_dn7)))), (((locals.var_sp_ov_temp_dn8 * locals.var_sp_ov_temp) + (locals.var_sp_ov_temp * locals.var_sp_ov_temp_dn8)) + ((locals.var_gov2_dn8 * assign24120_e24174) + (locals.var_gov2 * (locals.var_sp_ov_y0_dn8 - locals.var_sp_ov_d0_dn8)))), (((locals.var_sp_ov_temp_dn9 * locals.var_sp_ov_temp) + (locals.var_sp_ov_temp * locals.var_sp_ov_temp_dn9)) + ((locals.var_gov2_dn9 * assign24120_e24174) + (locals.var_gov2 * (locals.var_sp_ov_y0_dn9 - locals.var_sp_ov_d0_dn9)))),)
    } else {
        (locals.var_sp_ov_q, locals.var_sp_ov_q_dn4, locals.var_sp_ov_q_dn6, locals.var_sp_ov_q_dn7, locals.var_sp_ov_q_dn8, locals.var_sp_ov_q_dn9,)
    }
};
        locals.var_sp_ov_q = assign24120_e24178;
        locals.var_sp_ov_q_dn4 = assign24120_e24178_d_n4;
        locals.var_sp_ov_q_dn6 = assign24120_e24178_d_n6;
        locals.var_sp_ov_q_dn7 = assign24120_e24178_d_n7;
        locals.var_sp_ov_q_dn8 = assign24120_e24178_d_n8;
        locals.var_sp_ov_q_dn9 = assign24120_e24178_d_n9;
        locals.var_sp_ov_q_rv = 0.0;

        let (assign24130_e24193, assign24130_e24193_d_n4, assign24130_e24193_d_n6, assign24130_e24193_d_n7, assign24130_e24193_d_n8, assign24130_e24193_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) {
        let assign24130_e24188: f64 = (locals.var_gov2 * 0.5);
        let assign24130_e24190: f64 = (assign24130_e24188 * locals.var_sp_ov_d0);
        let assign24130_e24191: f64 = (1.0 - assign24130_e24190);
        (assign24130_e24191, (-(((locals.var_gov2_dn4 * 0.5) * locals.var_sp_ov_d0) + (assign24130_e24188 * locals.var_sp_ov_d0_dn4))), (-(((locals.var_gov2_dn6 * 0.5) * locals.var_sp_ov_d0) + (assign24130_e24188 * locals.var_sp_ov_d0_dn6))), (-(((locals.var_gov2_dn7 * 0.5) * locals.var_sp_ov_d0) + (assign24130_e24188 * locals.var_sp_ov_d0_dn7))), (-(((locals.var_gov2_dn8 * 0.5) * locals.var_sp_ov_d0) + (assign24130_e24188 * locals.var_sp_ov_d0_dn8))), (-(((locals.var_gov2_dn9 * 0.5) * locals.var_sp_ov_d0) + (assign24130_e24188 * locals.var_sp_ov_d0_dn9))),)
    } else {
        (locals.var_sp_ov_xi, locals.var_sp_ov_xi_dn4, locals.var_sp_ov_xi_dn6, locals.var_sp_ov_xi_dn7, locals.var_sp_ov_xi_dn8, locals.var_sp_ov_xi_dn9,)
    }
};
        locals.var_sp_ov_xi = assign24130_e24193;
        locals.var_sp_ov_xi_dn4 = assign24130_e24193_d_n4;
        locals.var_sp_ov_xi_dn6 = assign24130_e24193_d_n6;
        locals.var_sp_ov_xi_dn7 = assign24130_e24193_d_n7;
        locals.var_sp_ov_xi_dn8 = assign24130_e24193_d_n8;
        locals.var_sp_ov_xi_dn9 = assign24130_e24193_d_n9;
        locals.var_sp_ov_xi_rv = 0.0;

        let (assign24140_e24210, assign24140_e24210_d_n4, assign24140_e24210_d_n6, assign24140_e24210_d_n7, assign24140_e24210_d_n8, assign24140_e24210_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) {
        let assign24140_e24202: f64 = (locals.var_sp_ov_p * locals.var_sp_ov_p);
        let assign24140_e24206: f64 = (locals.var_sp_ov_xi * locals.var_sp_ov_q);
        let assign24140_e24207: f64 = (4.0 * assign24140_e24206);
        let assign24140_e24208: f64 = (assign24140_e24202 - assign24140_e24207);
        (assign24140_e24208, (((locals.var_sp_ov_p_dn4 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn4)) - (4.0 * ((locals.var_sp_ov_xi_dn4 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn4)))), (((locals.var_sp_ov_p_dn6 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn6)) - (4.0 * ((locals.var_sp_ov_xi_dn6 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn6)))), (((locals.var_sp_ov_p_dn7 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn7)) - (4.0 * ((locals.var_sp_ov_xi_dn7 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn7)))), (((locals.var_sp_ov_p_dn8 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn8)) - (4.0 * ((locals.var_sp_ov_xi_dn8 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn8)))), (((locals.var_sp_ov_p_dn9 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn9)) - (4.0 * ((locals.var_sp_ov_xi_dn9 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn9)))),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign24140_e24210;
        locals.var_sp_ov_temp_dn4 = assign24140_e24210_d_n4;
        locals.var_sp_ov_temp_dn6 = assign24140_e24210_d_n6;
        locals.var_sp_ov_temp_dn7 = assign24140_e24210_d_n7;
        locals.var_sp_ov_temp_dn8 = assign24140_e24210_d_n8;
        locals.var_sp_ov_temp_dn9 = assign24140_e24210_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let (assign24150_e24226, assign24150_e24226_d_n4, assign24150_e24226_d_n6, assign24150_e24226_d_n7, assign24150_e24226_d_n8, assign24150_e24226_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) {
        let assign24150_e24219: f64 = (2.0 * locals.var_sp_ov_q);
        let assign24150_e24222: f64 = (locals.var_sp_ov_temp).sqrt();
        let assign24150_e24223: f64 = (locals.var_sp_ov_p + assign24150_e24222);
        let assign24150_e24224: f64 = (assign24150_e24219 / assign24150_e24223);
        (assign24150_e24224, ((((2.0 * locals.var_sp_ov_q_dn4) * assign24150_e24223) - (assign24150_e24219 * (locals.var_sp_ov_p_dn4 + (locals.var_sp_ov_temp_dn4 / (2.0 * assign24150_e24222))))) / (assign24150_e24223 * assign24150_e24223)), ((((2.0 * locals.var_sp_ov_q_dn6) * assign24150_e24223) - (assign24150_e24219 * (locals.var_sp_ov_p_dn6 + (locals.var_sp_ov_temp_dn6 / (2.0 * assign24150_e24222))))) / (assign24150_e24223 * assign24150_e24223)), ((((2.0 * locals.var_sp_ov_q_dn7) * assign24150_e24223) - (assign24150_e24219 * (locals.var_sp_ov_p_dn7 + (locals.var_sp_ov_temp_dn7 / (2.0 * assign24150_e24222))))) / (assign24150_e24223 * assign24150_e24223)), ((((2.0 * locals.var_sp_ov_q_dn8) * assign24150_e24223) - (assign24150_e24219 * (locals.var_sp_ov_p_dn8 + (locals.var_sp_ov_temp_dn8 / (2.0 * assign24150_e24222))))) / (assign24150_e24223 * assign24150_e24223)), ((((2.0 * locals.var_sp_ov_q_dn9) * assign24150_e24223) - (assign24150_e24219 * (locals.var_sp_ov_p_dn9 + (locals.var_sp_ov_temp_dn9 / (2.0 * assign24150_e24222))))) / (assign24150_e24223 * assign24150_e24223)),)
    } else {
        (locals.var_sp_ov_w, locals.var_sp_ov_w_dn4, locals.var_sp_ov_w_dn6, locals.var_sp_ov_w_dn7, locals.var_sp_ov_w_dn8, locals.var_sp_ov_w_dn9,)
    }
};
        locals.var_sp_ov_w = assign24150_e24226;
        locals.var_sp_ov_w_dn4 = assign24150_e24226_d_n4;
        locals.var_sp_ov_w_dn6 = assign24150_e24226_d_n6;
        locals.var_sp_ov_w_dn7 = assign24150_e24226_d_n7;
        locals.var_sp_ov_w_dn8 = assign24150_e24226_d_n8;
        locals.var_sp_ov_w_dn9 = assign24150_e24226_d_n9;
        locals.var_sp_ov_w_rv = 0.0;

        let (assign24160_e24238, assign24160_e24238_d_n4, assign24160_e24238_d_n6, assign24160_e24238_d_n7, assign24160_e24238_d_n8, assign24160_e24238_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) {
        let assign24160_e24235: f64 = (locals.var_sp_ov_y0 + locals.var_sp_ov_w);
        let assign24160_e24236: f64 = (-assign24160_e24235);
        (assign24160_e24236, (-(locals.var_sp_ov_y0_dn4 + locals.var_sp_ov_w_dn4)), (-(locals.var_sp_ov_y0_dn6 + locals.var_sp_ov_w_dn6)), (-(locals.var_sp_ov_y0_dn7 + locals.var_sp_ov_w_dn7)), (-(locals.var_sp_ov_y0_dn8 + locals.var_sp_ov_w_dn8)), (-(locals.var_sp_ov_y0_dn9 + locals.var_sp_ov_w_dn9)),)
    } else {
        (locals.var_xs_ovcv, locals.var_xs_ovcv_dn4, locals.var_xs_ovcv_dn6, locals.var_xs_ovcv_dn7, locals.var_xs_ovcv_dn8, locals.var_xs_ovcv_dn9,)
    }
};
        locals.var_xs_ovcv = assign24160_e24238;
        locals.var_xs_ovcv_dn4 = assign24160_e24238_d_n4;
        locals.var_xs_ovcv_dn6 = assign24160_e24238_d_n6;
        locals.var_xs_ovcv_dn7 = assign24160_e24238_d_n7;
        locals.var_xs_ovcv_dn8 = assign24160_e24238_d_n8;
        locals.var_xs_ovcv_dn9 = assign24160_e24238_d_n9;
        locals.var_xs_ovcv_rv = 0.0;

        let (assign24170_e24256, assign24170_e24256_d_n4, assign24170_e24256_d_n6, assign24170_e24256_d_n7, assign24170_e24256_d_n8, assign24170_e24256_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 == 0.0)) {
        let assign24170_e24248: f64 = (locals.var_xi_ov * 1.25);
        let assign24170_e24250: f64 = (assign24170_e24248 * locals.var_inv_xg1);
        let assign24170_e24252: f64 = (assign24170_e24250 - 1.0);
        let assign24170_e24254: f64 = (assign24170_e24252 * locals.var_inv_xg1);
        (assign24170_e24254, (((((locals.var_xi_ov_dn4 * 1.25) * locals.var_inv_xg1) + (assign24170_e24248 * locals.var_inv_xg1_dn4)) * locals.var_inv_xg1) + (assign24170_e24252 * locals.var_inv_xg1_dn4)), (((((locals.var_xi_ov_dn6 * 1.25) * locals.var_inv_xg1) + (assign24170_e24248 * locals.var_inv_xg1_dn6)) * locals.var_inv_xg1) + (assign24170_e24252 * locals.var_inv_xg1_dn6)), (((((locals.var_xi_ov_dn7 * 1.25) * locals.var_inv_xg1) + (assign24170_e24248 * locals.var_inv_xg1_dn7)) * locals.var_inv_xg1) + (assign24170_e24252 * locals.var_inv_xg1_dn7)), (((((locals.var_xi_ov_dn8 * 1.25) * locals.var_inv_xg1) + (assign24170_e24248 * locals.var_inv_xg1_dn8)) * locals.var_inv_xg1) + (assign24170_e24252 * locals.var_inv_xg1_dn8)), (((((locals.var_xi_ov_dn9 * 1.25) * locals.var_inv_xg1) + (assign24170_e24248 * locals.var_inv_xg1_dn9)) * locals.var_inv_xg1) + (assign24170_e24252 * locals.var_inv_xg1_dn9)),)
    } else {
        (locals.var_sp_ov_afac, locals.var_sp_ov_afac_dn4, locals.var_sp_ov_afac_dn6, locals.var_sp_ov_afac_dn7, locals.var_sp_ov_afac_dn8, locals.var_sp_ov_afac_dn9,)
    }
};
        locals.var_sp_ov_afac = assign24170_e24256;
        locals.var_sp_ov_afac_dn4 = assign24170_e24256_d_n4;
        locals.var_sp_ov_afac_dn6 = assign24170_e24256_d_n6;
        locals.var_sp_ov_afac_dn7 = assign24170_e24256_d_n7;
        locals.var_sp_ov_afac_dn8 = assign24170_e24256_d_n8;
        locals.var_sp_ov_afac_dn9 = assign24170_e24256_d_n9;
        locals.var_sp_ov_afac_rv = 0.0;

        let (assign24180_e24274, assign24180_e24274_d_n4, assign24180_e24274_d_n6, assign24180_e24274_d_n7, assign24180_e24274_d_n8, assign24180_e24274_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 == 0.0)) {
        let assign24180_e24266: f64 = (locals.var_xgs_ovcv * locals.var_inv_xi_ov);
        let assign24180_e24270: f64 = (locals.var_sp_ov_afac * locals.var_xgs_ovcv);
        let assign24180_e24271: f64 = (1.0 + assign24180_e24270);
        let assign24180_e24272: f64 = (assign24180_e24266 * assign24180_e24271);
        (assign24180_e24272, ((((locals.var_xgs_ovcv_dn4 * locals.var_inv_xi_ov) + (locals.var_xgs_ovcv * locals.var_inv_xi_ov_dn4)) * assign24180_e24271) + (assign24180_e24266 * ((locals.var_sp_ov_afac_dn4 * locals.var_xgs_ovcv) + (locals.var_sp_ov_afac * locals.var_xgs_ovcv_dn4)))), ((((locals.var_xgs_ovcv_dn6 * locals.var_inv_xi_ov) + (locals.var_xgs_ovcv * locals.var_inv_xi_ov_dn6)) * assign24180_e24271) + (assign24180_e24266 * ((locals.var_sp_ov_afac_dn6 * locals.var_xgs_ovcv) + (locals.var_sp_ov_afac * locals.var_xgs_ovcv_dn6)))), ((((locals.var_xgs_ovcv_dn7 * locals.var_inv_xi_ov) + (locals.var_xgs_ovcv * locals.var_inv_xi_ov_dn7)) * assign24180_e24271) + (assign24180_e24266 * ((locals.var_sp_ov_afac_dn7 * locals.var_xgs_ovcv) + (locals.var_sp_ov_afac * locals.var_xgs_ovcv_dn7)))), ((((locals.var_xgs_ovcv_dn8 * locals.var_inv_xi_ov) + (locals.var_xgs_ovcv * locals.var_inv_xi_ov_dn8)) * assign24180_e24271) + (assign24180_e24266 * ((locals.var_sp_ov_afac_dn8 * locals.var_xgs_ovcv) + (locals.var_sp_ov_afac * locals.var_xgs_ovcv_dn8)))), ((((locals.var_xgs_ovcv_dn9 * locals.var_inv_xi_ov) + (locals.var_xgs_ovcv * locals.var_inv_xi_ov_dn9)) * assign24180_e24271) + (assign24180_e24266 * ((locals.var_sp_ov_afac_dn9 * locals.var_xgs_ovcv) + (locals.var_sp_ov_afac * locals.var_xgs_ovcv_dn9)))),)
    } else {
        (locals.var_sp_ov_xbar, locals.var_sp_ov_xbar_dn4, locals.var_sp_ov_xbar_dn6, locals.var_sp_ov_xbar_dn7, locals.var_sp_ov_xbar_dn8, locals.var_sp_ov_xbar_dn9,)
    }
};
        locals.var_sp_ov_xbar = assign24180_e24274;
        locals.var_sp_ov_xbar_dn4 = assign24180_e24274_d_n4;
        locals.var_sp_ov_xbar_dn6 = assign24180_e24274_d_n6;
        locals.var_sp_ov_xbar_dn7 = assign24180_e24274_d_n7;
        locals.var_sp_ov_xbar_dn8 = assign24180_e24274_d_n8;
        locals.var_sp_ov_xbar_dn9 = assign24180_e24274_d_n9;
        locals.var_sp_ov_xbar_rv = 0.0;

        let assign24190_e24276: f64 = (-locals.var_sp_ov_xbar);
        let assign24190_e24277: f64 = (assign24190_e24276).abs();
        let assign24190_e24279: f64 = if assign24190_e24277 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard699 = assign24190_e24279;
        locals.var_guard699_rv = 0.0;

        let (assign24200_e24293, assign24200_e24293_d_n4, assign24200_e24293_d_n6, assign24200_e24293_d_n7, assign24200_e24293_d_n8, assign24200_e24293_d_n9,) = {
    if ((((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 == 0.0)) && (locals.var_guard699 != 0.0)) {
        let assign24200_e24290: f64 = (-locals.var_sp_ov_xbar);
        let assign24200_e24291: f64 = (assign24200_e24290).exp();
        (assign24200_e24291, (assign24200_e24291 * (-locals.var_sp_ov_xbar_dn4)), (assign24200_e24291 * (-locals.var_sp_ov_xbar_dn6)), (assign24200_e24291 * (-locals.var_sp_ov_xbar_dn7)), (assign24200_e24291 * (-locals.var_sp_ov_xbar_dn8)), (assign24200_e24291 * (-locals.var_sp_ov_xbar_dn9)),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign24200_e24293;
        locals.var_sp_ov_temp_dn4 = assign24200_e24293_d_n4;
        locals.var_sp_ov_temp_dn6 = assign24200_e24293_d_n6;
        locals.var_sp_ov_temp_dn7 = assign24200_e24293_d_n7;
        locals.var_sp_ov_temp_dn8 = assign24200_e24293_d_n8;
        locals.var_sp_ov_temp_dn9 = assign24200_e24293_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_66(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign24210_e24295: f64 = (-locals.var_sp_ov_xbar);
        let assign24210_e24297: f64 = (-80.0);
        let assign24210_e24298: f64 = if assign24210_e24295 < assign24210_e24297 { 1.0 } else { 0.0 };
        locals.var_guard700 = assign24210_e24298;
        locals.var_guard700_rv = 0.0;

        let (assign24220_e24341, assign24220_e24341_d_n4, assign24220_e24341_d_n6, assign24220_e24341_d_n7, assign24220_e24341_d_n8, assign24220_e24341_d_n9,) = {
    if (((((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 == 0.0)) && (locals.var_guard699 == 0.0)) && (locals.var_guard700 != 0.0)) {
        let assign24220_e24314: f64 = (-locals.var_sp_ov_xbar);
        let assign24220_e24315: f64 = (-assign24220_e24314);
        let assign24220_e24317: f64 = (assign24220_e24315 - 80.0);
        let assign24220_e24321: f64 = (-locals.var_sp_ov_xbar);
        let assign24220_e24322: f64 = (-assign24220_e24321);
        let assign24220_e24324: f64 = (assign24220_e24322 - 80.0);
        let assign24220_e24325: f64 = (0.5 * assign24220_e24324);
        let assign24220_e24328: f64 = (-locals.var_sp_ov_xbar);
        let assign24220_e24329: f64 = (-assign24220_e24328);
        let assign24220_e24331: f64 = (assign24220_e24329 - 80.0);
        let assign24220_e24333: f64 = (assign24220_e24331 * 0.3333333333333);
        let assign24220_e24334: f64 = (1.0 + assign24220_e24333);
        let assign24220_e24335: f64 = (assign24220_e24325 * assign24220_e24334);
        let assign24220_e24336: f64 = (1.0 + assign24220_e24335);
        let assign24220_e24337: f64 = (assign24220_e24317 * assign24220_e24336);
        let assign24220_e24338: f64 = (1.0 + assign24220_e24337);
        let assign24220_e24339: f64 = (1.80485e-35 / assign24220_e24338);
        (assign24220_e24339, (-((1.80485e-35 * (((-(-locals.var_sp_ov_xbar_dn4)) * assign24220_e24336) + (assign24220_e24317 * (((0.5 * (-(-locals.var_sp_ov_xbar_dn4))) * assign24220_e24334) + (assign24220_e24325 * ((-(-locals.var_sp_ov_xbar_dn4)) * 0.3333333333333)))))) / (assign24220_e24338 * assign24220_e24338))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_xbar_dn6)) * assign24220_e24336) + (assign24220_e24317 * (((0.5 * (-(-locals.var_sp_ov_xbar_dn6))) * assign24220_e24334) + (assign24220_e24325 * ((-(-locals.var_sp_ov_xbar_dn6)) * 0.3333333333333)))))) / (assign24220_e24338 * assign24220_e24338))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_xbar_dn7)) * assign24220_e24336) + (assign24220_e24317 * (((0.5 * (-(-locals.var_sp_ov_xbar_dn7))) * assign24220_e24334) + (assign24220_e24325 * ((-(-locals.var_sp_ov_xbar_dn7)) * 0.3333333333333)))))) / (assign24220_e24338 * assign24220_e24338))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_xbar_dn8)) * assign24220_e24336) + (assign24220_e24317 * (((0.5 * (-(-locals.var_sp_ov_xbar_dn8))) * assign24220_e24334) + (assign24220_e24325 * ((-(-locals.var_sp_ov_xbar_dn8)) * 0.3333333333333)))))) / (assign24220_e24338 * assign24220_e24338))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_xbar_dn9)) * assign24220_e24336) + (assign24220_e24317 * (((0.5 * (-(-locals.var_sp_ov_xbar_dn9))) * assign24220_e24334) + (assign24220_e24325 * ((-(-locals.var_sp_ov_xbar_dn9)) * 0.3333333333333)))))) / (assign24220_e24338 * assign24220_e24338))),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign24220_e24341;
        locals.var_sp_ov_temp_dn4 = assign24220_e24341_d_n4;
        locals.var_sp_ov_temp_dn6 = assign24220_e24341_d_n6;
        locals.var_sp_ov_temp_dn7 = assign24220_e24341_d_n7;
        locals.var_sp_ov_temp_dn8 = assign24220_e24341_d_n8;
        locals.var_sp_ov_temp_dn9 = assign24220_e24341_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let (assign24230_e24382, assign24230_e24382_d_n4, assign24230_e24382_d_n6, assign24230_e24382_d_n7, assign24230_e24382_d_n8, assign24230_e24382_d_n9,) = {
    if (((((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 == 0.0)) && (locals.var_guard699 == 0.0)) && (locals.var_guard700 == 0.0)) {
        let assign24230_e24358: f64 = (-locals.var_sp_ov_xbar);
        let assign24230_e24360: f64 = (assign24230_e24358 - 80.0);
        let assign24230_e24364: f64 = (-locals.var_sp_ov_xbar);
        let assign24230_e24366: f64 = (assign24230_e24364 - 80.0);
        let assign24230_e24367: f64 = (0.5 * assign24230_e24366);
        let assign24230_e24370: f64 = (-locals.var_sp_ov_xbar);
        let assign24230_e24372: f64 = (assign24230_e24370 - 80.0);
        let assign24230_e24374: f64 = (assign24230_e24372 * 0.3333333333333);
        let assign24230_e24375: f64 = (1.0 + assign24230_e24374);
        let assign24230_e24376: f64 = (assign24230_e24367 * assign24230_e24375);
        let assign24230_e24377: f64 = (1.0 + assign24230_e24376);
        let assign24230_e24378: f64 = (assign24230_e24360 * assign24230_e24377);
        let assign24230_e24379: f64 = (1.0 + assign24230_e24378);
        let assign24230_e24380: f64 = (5.54062e34 * assign24230_e24379);
        (assign24230_e24380, (5.54062e34 * (((-locals.var_sp_ov_xbar_dn4) * assign24230_e24377) + (assign24230_e24360 * (((0.5 * (-locals.var_sp_ov_xbar_dn4)) * assign24230_e24375) + (assign24230_e24367 * ((-locals.var_sp_ov_xbar_dn4) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_xbar_dn6) * assign24230_e24377) + (assign24230_e24360 * (((0.5 * (-locals.var_sp_ov_xbar_dn6)) * assign24230_e24375) + (assign24230_e24367 * ((-locals.var_sp_ov_xbar_dn6) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_xbar_dn7) * assign24230_e24377) + (assign24230_e24360 * (((0.5 * (-locals.var_sp_ov_xbar_dn7)) * assign24230_e24375) + (assign24230_e24367 * ((-locals.var_sp_ov_xbar_dn7) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_xbar_dn8) * assign24230_e24377) + (assign24230_e24360 * (((0.5 * (-locals.var_sp_ov_xbar_dn8)) * assign24230_e24375) + (assign24230_e24367 * ((-locals.var_sp_ov_xbar_dn8) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_xbar_dn9) * assign24230_e24377) + (assign24230_e24360 * (((0.5 * (-locals.var_sp_ov_xbar_dn9)) * assign24230_e24375) + (assign24230_e24367 * ((-locals.var_sp_ov_xbar_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign24230_e24382;
        locals.var_sp_ov_temp_dn4 = assign24230_e24382_d_n4;
        locals.var_sp_ov_temp_dn6 = assign24230_e24382_d_n6;
        locals.var_sp_ov_temp_dn7 = assign24230_e24382_d_n7;
        locals.var_sp_ov_temp_dn8 = assign24230_e24382_d_n8;
        locals.var_sp_ov_temp_dn9 = assign24230_e24382_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let (assign24240_e24394, assign24240_e24394_d_n4, assign24240_e24394_d_n6, assign24240_e24394_d_n7, assign24240_e24394_d_n8, assign24240_e24394_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 == 0.0)) {
        let assign24240_e24392: f64 = (1.0 - locals.var_sp_ov_temp);
        (assign24240_e24392, (-locals.var_sp_ov_temp_dn4), (-locals.var_sp_ov_temp_dn6), (-locals.var_sp_ov_temp_dn7), (-locals.var_sp_ov_temp_dn8), (-locals.var_sp_ov_temp_dn9),)
    } else {
        (locals.var_sp_ov_w, locals.var_sp_ov_w_dn4, locals.var_sp_ov_w_dn6, locals.var_sp_ov_w_dn7, locals.var_sp_ov_w_dn8, locals.var_sp_ov_w_dn9,)
    }
};
        locals.var_sp_ov_w = assign24240_e24394;
        locals.var_sp_ov_w_dn4 = assign24240_e24394_d_n4;
        locals.var_sp_ov_w_dn6 = assign24240_e24394_d_n6;
        locals.var_sp_ov_w_dn7 = assign24240_e24394_d_n7;
        locals.var_sp_ov_w_dn8 = assign24240_e24394_d_n8;
        locals.var_sp_ov_w_dn9 = assign24240_e24394_d_n9;
        locals.var_sp_ov_w_rv = 0.0;

        let (assign24250_e24419, assign24250_e24419_d_n4, assign24250_e24419_d_n6, assign24250_e24419_d_n7, assign24250_e24419_d_n8, assign24250_e24419_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 == 0.0)) {
        let assign24250_e24405: f64 = (locals.var_gov2 * 0.5);
        let assign24250_e24406: f64 = (locals.var_xgs_ovcv + assign24250_e24405);
        let assign24250_e24411: f64 = (locals.var_gov2 * 0.25);
        let assign24250_e24412: f64 = (locals.var_xgs_ovcv + assign24250_e24411);
        let assign24250_e24414: f64 = (assign24250_e24412 - locals.var_sp_ov_w);
        let assign24250_e24415: f64 = (assign24250_e24414).sqrt();
        let assign24250_e24416: f64 = (locals.var_gov * assign24250_e24415);
        let assign24250_e24417: f64 = (assign24250_e24406 - assign24250_e24416);
        (assign24250_e24417, ((locals.var_xgs_ovcv_dn4 + (locals.var_gov2_dn4 * 0.5)) - ((locals.var_gov_dn4 * assign24250_e24415) + (locals.var_gov * (((locals.var_xgs_ovcv_dn4 + (locals.var_gov2_dn4 * 0.25)) - locals.var_sp_ov_w_dn4) / (2.0 * assign24250_e24415))))), ((locals.var_xgs_ovcv_dn6 + (locals.var_gov2_dn6 * 0.5)) - ((locals.var_gov_dn6 * assign24250_e24415) + (locals.var_gov * (((locals.var_xgs_ovcv_dn6 + (locals.var_gov2_dn6 * 0.25)) - locals.var_sp_ov_w_dn6) / (2.0 * assign24250_e24415))))), ((locals.var_xgs_ovcv_dn7 + (locals.var_gov2_dn7 * 0.5)) - ((locals.var_gov_dn7 * assign24250_e24415) + (locals.var_gov * (((locals.var_xgs_ovcv_dn7 + (locals.var_gov2_dn7 * 0.25)) - locals.var_sp_ov_w_dn7) / (2.0 * assign24250_e24415))))), ((locals.var_xgs_ovcv_dn8 + (locals.var_gov2_dn8 * 0.5)) - ((locals.var_gov_dn8 * assign24250_e24415) + (locals.var_gov * (((locals.var_xgs_ovcv_dn8 + (locals.var_gov2_dn8 * 0.25)) - locals.var_sp_ov_w_dn8) / (2.0 * assign24250_e24415))))), ((locals.var_xgs_ovcv_dn9 + (locals.var_gov2_dn9 * 0.5)) - ((locals.var_gov_dn9 * assign24250_e24415) + (locals.var_gov * (((locals.var_xgs_ovcv_dn9 + (locals.var_gov2_dn9 * 0.25)) - locals.var_sp_ov_w_dn9) / (2.0 * assign24250_e24415))))),)
    } else {
        (locals.var_sp_ov_x0, locals.var_sp_ov_x0_dn4, locals.var_sp_ov_x0_dn6, locals.var_sp_ov_x0_dn7, locals.var_sp_ov_x0_dn8, locals.var_sp_ov_x0_dn9,)
    }
};
        locals.var_sp_ov_x0 = assign24250_e24419;
        locals.var_sp_ov_x0_dn4 = assign24250_e24419_d_n4;
        locals.var_sp_ov_x0_dn6 = assign24250_e24419_d_n6;
        locals.var_sp_ov_x0_dn7 = assign24250_e24419_d_n7;
        locals.var_sp_ov_x0_dn8 = assign24250_e24419_d_n8;
        locals.var_sp_ov_x0_dn9 = assign24250_e24419_d_n9;
        locals.var_sp_ov_x0_rv = 0.0;

        let assign24260_e24421: f64 = (-locals.var_sp_ov_x0);
        let assign24260_e24422: f64 = (assign24260_e24421).abs();
        let assign24260_e24424: f64 = if assign24260_e24422 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard701 = assign24260_e24424;
        locals.var_guard701_rv = 0.0;

        let (assign24270_e24438, assign24270_e24438_d_n4, assign24270_e24438_d_n6, assign24270_e24438_d_n7, assign24270_e24438_d_n8, assign24270_e24438_d_n9,) = {
    if ((((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 == 0.0)) && (locals.var_guard701 != 0.0)) {
        let assign24270_e24435: f64 = (-locals.var_sp_ov_x0);
        let assign24270_e24436: f64 = (assign24270_e24435).exp();
        (assign24270_e24436, (assign24270_e24436 * (-locals.var_sp_ov_x0_dn4)), (assign24270_e24436 * (-locals.var_sp_ov_x0_dn6)), (assign24270_e24436 * (-locals.var_sp_ov_x0_dn7)), (assign24270_e24436 * (-locals.var_sp_ov_x0_dn8)), (assign24270_e24436 * (-locals.var_sp_ov_x0_dn9)),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign24270_e24438;
        locals.var_sp_ov_d0_dn4 = assign24270_e24438_d_n4;
        locals.var_sp_ov_d0_dn6 = assign24270_e24438_d_n6;
        locals.var_sp_ov_d0_dn7 = assign24270_e24438_d_n7;
        locals.var_sp_ov_d0_dn8 = assign24270_e24438_d_n8;
        locals.var_sp_ov_d0_dn9 = assign24270_e24438_d_n9;
        locals.var_sp_ov_d0_rv = 0.0;

        let assign24280_e24440: f64 = (-locals.var_sp_ov_x0);
        let assign24280_e24442: f64 = (-80.0);
        let assign24280_e24443: f64 = if assign24280_e24440 < assign24280_e24442 { 1.0 } else { 0.0 };
        locals.var_guard702 = assign24280_e24443;
        locals.var_guard702_rv = 0.0;

        let (assign24290_e24486, assign24290_e24486_d_n4, assign24290_e24486_d_n6, assign24290_e24486_d_n7, assign24290_e24486_d_n8, assign24290_e24486_d_n9,) = {
    if (((((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 == 0.0)) && (locals.var_guard701 == 0.0)) && (locals.var_guard702 != 0.0)) {
        let assign24290_e24459: f64 = (-locals.var_sp_ov_x0);
        let assign24290_e24460: f64 = (-assign24290_e24459);
        let assign24290_e24462: f64 = (assign24290_e24460 - 80.0);
        let assign24290_e24466: f64 = (-locals.var_sp_ov_x0);
        let assign24290_e24467: f64 = (-assign24290_e24466);
        let assign24290_e24469: f64 = (assign24290_e24467 - 80.0);
        let assign24290_e24470: f64 = (0.5 * assign24290_e24469);
        let assign24290_e24473: f64 = (-locals.var_sp_ov_x0);
        let assign24290_e24474: f64 = (-assign24290_e24473);
        let assign24290_e24476: f64 = (assign24290_e24474 - 80.0);
        let assign24290_e24478: f64 = (assign24290_e24476 * 0.3333333333333);
        let assign24290_e24479: f64 = (1.0 + assign24290_e24478);
        let assign24290_e24480: f64 = (assign24290_e24470 * assign24290_e24479);
        let assign24290_e24481: f64 = (1.0 + assign24290_e24480);
        let assign24290_e24482: f64 = (assign24290_e24462 * assign24290_e24481);
        let assign24290_e24483: f64 = (1.0 + assign24290_e24482);
        let assign24290_e24484: f64 = (1.80485e-35 / assign24290_e24483);
        (assign24290_e24484, (-((1.80485e-35 * (((-(-locals.var_sp_ov_x0_dn4)) * assign24290_e24481) + (assign24290_e24462 * (((0.5 * (-(-locals.var_sp_ov_x0_dn4))) * assign24290_e24479) + (assign24290_e24470 * ((-(-locals.var_sp_ov_x0_dn4)) * 0.3333333333333)))))) / (assign24290_e24483 * assign24290_e24483))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_x0_dn6)) * assign24290_e24481) + (assign24290_e24462 * (((0.5 * (-(-locals.var_sp_ov_x0_dn6))) * assign24290_e24479) + (assign24290_e24470 * ((-(-locals.var_sp_ov_x0_dn6)) * 0.3333333333333)))))) / (assign24290_e24483 * assign24290_e24483))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_x0_dn7)) * assign24290_e24481) + (assign24290_e24462 * (((0.5 * (-(-locals.var_sp_ov_x0_dn7))) * assign24290_e24479) + (assign24290_e24470 * ((-(-locals.var_sp_ov_x0_dn7)) * 0.3333333333333)))))) / (assign24290_e24483 * assign24290_e24483))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_x0_dn8)) * assign24290_e24481) + (assign24290_e24462 * (((0.5 * (-(-locals.var_sp_ov_x0_dn8))) * assign24290_e24479) + (assign24290_e24470 * ((-(-locals.var_sp_ov_x0_dn8)) * 0.3333333333333)))))) / (assign24290_e24483 * assign24290_e24483))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_x0_dn9)) * assign24290_e24481) + (assign24290_e24462 * (((0.5 * (-(-locals.var_sp_ov_x0_dn9))) * assign24290_e24479) + (assign24290_e24470 * ((-(-locals.var_sp_ov_x0_dn9)) * 0.3333333333333)))))) / (assign24290_e24483 * assign24290_e24483))),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign24290_e24486;
        locals.var_sp_ov_d0_dn4 = assign24290_e24486_d_n4;
        locals.var_sp_ov_d0_dn6 = assign24290_e24486_d_n6;
        locals.var_sp_ov_d0_dn7 = assign24290_e24486_d_n7;
        locals.var_sp_ov_d0_dn8 = assign24290_e24486_d_n8;
        locals.var_sp_ov_d0_dn9 = assign24290_e24486_d_n9;
        locals.var_sp_ov_d0_rv = 0.0;

        let (assign24300_e24527, assign24300_e24527_d_n4, assign24300_e24527_d_n6, assign24300_e24527_d_n7, assign24300_e24527_d_n8, assign24300_e24527_d_n9,) = {
    if (((((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 == 0.0)) && (locals.var_guard701 == 0.0)) && (locals.var_guard702 == 0.0)) {
        let assign24300_e24503: f64 = (-locals.var_sp_ov_x0);
        let assign24300_e24505: f64 = (assign24300_e24503 - 80.0);
        let assign24300_e24509: f64 = (-locals.var_sp_ov_x0);
        let assign24300_e24511: f64 = (assign24300_e24509 - 80.0);
        let assign24300_e24512: f64 = (0.5 * assign24300_e24511);
        let assign24300_e24515: f64 = (-locals.var_sp_ov_x0);
        let assign24300_e24517: f64 = (assign24300_e24515 - 80.0);
        let assign24300_e24519: f64 = (assign24300_e24517 * 0.3333333333333);
        let assign24300_e24520: f64 = (1.0 + assign24300_e24519);
        let assign24300_e24521: f64 = (assign24300_e24512 * assign24300_e24520);
        let assign24300_e24522: f64 = (1.0 + assign24300_e24521);
        let assign24300_e24523: f64 = (assign24300_e24505 * assign24300_e24522);
        let assign24300_e24524: f64 = (1.0 + assign24300_e24523);
        let assign24300_e24525: f64 = (5.54062e34 * assign24300_e24524);
        (assign24300_e24525, (5.54062e34 * (((-locals.var_sp_ov_x0_dn4) * assign24300_e24522) + (assign24300_e24505 * (((0.5 * (-locals.var_sp_ov_x0_dn4)) * assign24300_e24520) + (assign24300_e24512 * ((-locals.var_sp_ov_x0_dn4) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_x0_dn6) * assign24300_e24522) + (assign24300_e24505 * (((0.5 * (-locals.var_sp_ov_x0_dn6)) * assign24300_e24520) + (assign24300_e24512 * ((-locals.var_sp_ov_x0_dn6) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_x0_dn7) * assign24300_e24522) + (assign24300_e24505 * (((0.5 * (-locals.var_sp_ov_x0_dn7)) * assign24300_e24520) + (assign24300_e24512 * ((-locals.var_sp_ov_x0_dn7) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_x0_dn8) * assign24300_e24522) + (assign24300_e24505 * (((0.5 * (-locals.var_sp_ov_x0_dn8)) * assign24300_e24520) + (assign24300_e24512 * ((-locals.var_sp_ov_x0_dn8) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_x0_dn9) * assign24300_e24522) + (assign24300_e24505 * (((0.5 * (-locals.var_sp_ov_x0_dn9)) * assign24300_e24520) + (assign24300_e24512 * ((-locals.var_sp_ov_x0_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign24300_e24527;
        locals.var_sp_ov_d0_dn4 = assign24300_e24527_d_n4;
        locals.var_sp_ov_d0_dn6 = assign24300_e24527_d_n6;
        locals.var_sp_ov_d0_dn7 = assign24300_e24527_d_n7;
        locals.var_sp_ov_d0_dn8 = assign24300_e24527_d_n8;
        locals.var_sp_ov_d0_dn9 = assign24300_e24527_d_n9;
        locals.var_sp_ov_d0_rv = 0.0;

        let (assign24310_e24547, assign24310_e24547_d_n4, assign24310_e24547_d_n6, assign24310_e24547_d_n7, assign24310_e24547_d_n8, assign24310_e24547_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 == 0.0)) {
        let assign24310_e24538: f64 = (locals.var_xgs_ovcv - locals.var_sp_ov_x0);
        let assign24310_e24539: f64 = (2.0 * assign24310_e24538);
        let assign24310_e24543: f64 = (1.0 - locals.var_sp_ov_d0);
        let assign24310_e24544: f64 = (locals.var_gov2 * assign24310_e24543);
        let assign24310_e24545: f64 = (assign24310_e24539 + assign24310_e24544);
        (assign24310_e24545, ((2.0 * (locals.var_xgs_ovcv_dn4 - locals.var_sp_ov_x0_dn4)) + ((locals.var_gov2_dn4 * assign24310_e24543) + (locals.var_gov2 * (-locals.var_sp_ov_d0_dn4)))), ((2.0 * (locals.var_xgs_ovcv_dn6 - locals.var_sp_ov_x0_dn6)) + ((locals.var_gov2_dn6 * assign24310_e24543) + (locals.var_gov2 * (-locals.var_sp_ov_d0_dn6)))), ((2.0 * (locals.var_xgs_ovcv_dn7 - locals.var_sp_ov_x0_dn7)) + ((locals.var_gov2_dn7 * assign24310_e24543) + (locals.var_gov2 * (-locals.var_sp_ov_d0_dn7)))), ((2.0 * (locals.var_xgs_ovcv_dn8 - locals.var_sp_ov_x0_dn8)) + ((locals.var_gov2_dn8 * assign24310_e24543) + (locals.var_gov2 * (-locals.var_sp_ov_d0_dn8)))), ((2.0 * (locals.var_xgs_ovcv_dn9 - locals.var_sp_ov_x0_dn9)) + ((locals.var_gov2_dn9 * assign24310_e24543) + (locals.var_gov2 * (-locals.var_sp_ov_d0_dn9)))),)
    } else {
        (locals.var_sp_ov_p, locals.var_sp_ov_p_dn4, locals.var_sp_ov_p_dn6, locals.var_sp_ov_p_dn7, locals.var_sp_ov_p_dn8, locals.var_sp_ov_p_dn9,)
    }
};
        locals.var_sp_ov_p = assign24310_e24547;
        locals.var_sp_ov_p_dn4 = assign24310_e24547_d_n4;
        locals.var_sp_ov_p_dn6 = assign24310_e24547_d_n6;
        locals.var_sp_ov_p_dn7 = assign24310_e24547_d_n7;
        locals.var_sp_ov_p_dn8 = assign24310_e24547_d_n8;
        locals.var_sp_ov_p_dn9 = assign24310_e24547_d_n9;
        locals.var_sp_ov_p_rv = 0.0;

        let (assign24320_e24571, assign24320_e24571_d_n4, assign24320_e24571_d_n6, assign24320_e24571_d_n7, assign24320_e24571_d_n8, assign24320_e24571_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 == 0.0)) {
        let assign24320_e24557: f64 = (locals.var_xgs_ovcv - locals.var_sp_ov_x0);
        let assign24320_e24560: f64 = (locals.var_xgs_ovcv - locals.var_sp_ov_x0);
        let assign24320_e24561: f64 = (assign24320_e24557 * assign24320_e24560);
        let assign24320_e24565: f64 = (locals.var_sp_ov_x0 - 1.0);
        let assign24320_e24567: f64 = (assign24320_e24565 + locals.var_sp_ov_d0);
        let assign24320_e24568: f64 = (locals.var_gov2 * assign24320_e24567);
        let assign24320_e24569: f64 = (assign24320_e24561 - assign24320_e24568);
        (assign24320_e24569, ((((locals.var_xgs_ovcv_dn4 - locals.var_sp_ov_x0_dn4) * assign24320_e24560) + (assign24320_e24557 * (locals.var_xgs_ovcv_dn4 - locals.var_sp_ov_x0_dn4))) - ((locals.var_gov2_dn4 * assign24320_e24567) + (locals.var_gov2 * (locals.var_sp_ov_x0_dn4 + locals.var_sp_ov_d0_dn4)))), ((((locals.var_xgs_ovcv_dn6 - locals.var_sp_ov_x0_dn6) * assign24320_e24560) + (assign24320_e24557 * (locals.var_xgs_ovcv_dn6 - locals.var_sp_ov_x0_dn6))) - ((locals.var_gov2_dn6 * assign24320_e24567) + (locals.var_gov2 * (locals.var_sp_ov_x0_dn6 + locals.var_sp_ov_d0_dn6)))), ((((locals.var_xgs_ovcv_dn7 - locals.var_sp_ov_x0_dn7) * assign24320_e24560) + (assign24320_e24557 * (locals.var_xgs_ovcv_dn7 - locals.var_sp_ov_x0_dn7))) - ((locals.var_gov2_dn7 * assign24320_e24567) + (locals.var_gov2 * (locals.var_sp_ov_x0_dn7 + locals.var_sp_ov_d0_dn7)))), ((((locals.var_xgs_ovcv_dn8 - locals.var_sp_ov_x0_dn8) * assign24320_e24560) + (assign24320_e24557 * (locals.var_xgs_ovcv_dn8 - locals.var_sp_ov_x0_dn8))) - ((locals.var_gov2_dn8 * assign24320_e24567) + (locals.var_gov2 * (locals.var_sp_ov_x0_dn8 + locals.var_sp_ov_d0_dn8)))), ((((locals.var_xgs_ovcv_dn9 - locals.var_sp_ov_x0_dn9) * assign24320_e24560) + (assign24320_e24557 * (locals.var_xgs_ovcv_dn9 - locals.var_sp_ov_x0_dn9))) - ((locals.var_gov2_dn9 * assign24320_e24567) + (locals.var_gov2 * (locals.var_sp_ov_x0_dn9 + locals.var_sp_ov_d0_dn9)))),)
    } else {
        (locals.var_sp_ov_q, locals.var_sp_ov_q_dn4, locals.var_sp_ov_q_dn6, locals.var_sp_ov_q_dn7, locals.var_sp_ov_q_dn8, locals.var_sp_ov_q_dn9,)
    }
};
        locals.var_sp_ov_q = assign24320_e24571;
        locals.var_sp_ov_q_dn4 = assign24320_e24571_d_n4;
        locals.var_sp_ov_q_dn6 = assign24320_e24571_d_n6;
        locals.var_sp_ov_q_dn7 = assign24320_e24571_d_n7;
        locals.var_sp_ov_q_dn8 = assign24320_e24571_d_n8;
        locals.var_sp_ov_q_dn9 = assign24320_e24571_d_n9;
        locals.var_sp_ov_q_rv = 0.0;

        let (assign24330_e24587, assign24330_e24587_d_n4, assign24330_e24587_d_n6, assign24330_e24587_d_n7, assign24330_e24587_d_n8, assign24330_e24587_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 == 0.0)) {
        let assign24330_e24582: f64 = (locals.var_gov2 * 0.5);
        let assign24330_e24584: f64 = (assign24330_e24582 * locals.var_sp_ov_d0);
        let assign24330_e24585: f64 = (1.0 - assign24330_e24584);
        (assign24330_e24585, (-(((locals.var_gov2_dn4 * 0.5) * locals.var_sp_ov_d0) + (assign24330_e24582 * locals.var_sp_ov_d0_dn4))), (-(((locals.var_gov2_dn6 * 0.5) * locals.var_sp_ov_d0) + (assign24330_e24582 * locals.var_sp_ov_d0_dn6))), (-(((locals.var_gov2_dn7 * 0.5) * locals.var_sp_ov_d0) + (assign24330_e24582 * locals.var_sp_ov_d0_dn7))), (-(((locals.var_gov2_dn8 * 0.5) * locals.var_sp_ov_d0) + (assign24330_e24582 * locals.var_sp_ov_d0_dn8))), (-(((locals.var_gov2_dn9 * 0.5) * locals.var_sp_ov_d0) + (assign24330_e24582 * locals.var_sp_ov_d0_dn9))),)
    } else {
        (locals.var_sp_ov_xi, locals.var_sp_ov_xi_dn4, locals.var_sp_ov_xi_dn6, locals.var_sp_ov_xi_dn7, locals.var_sp_ov_xi_dn8, locals.var_sp_ov_xi_dn9,)
    }
};
        locals.var_sp_ov_xi = assign24330_e24587;
        locals.var_sp_ov_xi_dn4 = assign24330_e24587_d_n4;
        locals.var_sp_ov_xi_dn6 = assign24330_e24587_d_n6;
        locals.var_sp_ov_xi_dn7 = assign24330_e24587_d_n7;
        locals.var_sp_ov_xi_dn8 = assign24330_e24587_d_n8;
        locals.var_sp_ov_xi_dn9 = assign24330_e24587_d_n9;
        locals.var_sp_ov_xi_rv = 0.0;

        let (assign24340_e24605, assign24340_e24605_d_n4, assign24340_e24605_d_n6, assign24340_e24605_d_n7, assign24340_e24605_d_n8, assign24340_e24605_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 == 0.0)) {
        let assign24340_e24597: f64 = (locals.var_sp_ov_p * locals.var_sp_ov_p);
        let assign24340_e24601: f64 = (locals.var_sp_ov_xi * locals.var_sp_ov_q);
        let assign24340_e24602: f64 = (4.0 * assign24340_e24601);
        let assign24340_e24603: f64 = (assign24340_e24597 - assign24340_e24602);
        (assign24340_e24603, (((locals.var_sp_ov_p_dn4 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn4)) - (4.0 * ((locals.var_sp_ov_xi_dn4 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn4)))), (((locals.var_sp_ov_p_dn6 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn6)) - (4.0 * ((locals.var_sp_ov_xi_dn6 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn6)))), (((locals.var_sp_ov_p_dn7 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn7)) - (4.0 * ((locals.var_sp_ov_xi_dn7 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn7)))), (((locals.var_sp_ov_p_dn8 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn8)) - (4.0 * ((locals.var_sp_ov_xi_dn8 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn8)))), (((locals.var_sp_ov_p_dn9 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn9)) - (4.0 * ((locals.var_sp_ov_xi_dn9 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn9)))),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign24340_e24605;
        locals.var_sp_ov_temp_dn4 = assign24340_e24605_d_n4;
        locals.var_sp_ov_temp_dn6 = assign24340_e24605_d_n6;
        locals.var_sp_ov_temp_dn7 = assign24340_e24605_d_n7;
        locals.var_sp_ov_temp_dn8 = assign24340_e24605_d_n8;
        locals.var_sp_ov_temp_dn9 = assign24340_e24605_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let (assign24350_e24622, assign24350_e24622_d_n4, assign24350_e24622_d_n6, assign24350_e24622_d_n7, assign24350_e24622_d_n8, assign24350_e24622_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 == 0.0)) {
        let assign24350_e24615: f64 = (2.0 * locals.var_sp_ov_q);
        let assign24350_e24618: f64 = (locals.var_sp_ov_temp).sqrt();
        let assign24350_e24619: f64 = (locals.var_sp_ov_p + assign24350_e24618);
        let assign24350_e24620: f64 = (assign24350_e24615 / assign24350_e24619);
        (assign24350_e24620, ((((2.0 * locals.var_sp_ov_q_dn4) * assign24350_e24619) - (assign24350_e24615 * (locals.var_sp_ov_p_dn4 + (locals.var_sp_ov_temp_dn4 / (2.0 * assign24350_e24618))))) / (assign24350_e24619 * assign24350_e24619)), ((((2.0 * locals.var_sp_ov_q_dn6) * assign24350_e24619) - (assign24350_e24615 * (locals.var_sp_ov_p_dn6 + (locals.var_sp_ov_temp_dn6 / (2.0 * assign24350_e24618))))) / (assign24350_e24619 * assign24350_e24619)), ((((2.0 * locals.var_sp_ov_q_dn7) * assign24350_e24619) - (assign24350_e24615 * (locals.var_sp_ov_p_dn7 + (locals.var_sp_ov_temp_dn7 / (2.0 * assign24350_e24618))))) / (assign24350_e24619 * assign24350_e24619)), ((((2.0 * locals.var_sp_ov_q_dn8) * assign24350_e24619) - (assign24350_e24615 * (locals.var_sp_ov_p_dn8 + (locals.var_sp_ov_temp_dn8 / (2.0 * assign24350_e24618))))) / (assign24350_e24619 * assign24350_e24619)), ((((2.0 * locals.var_sp_ov_q_dn9) * assign24350_e24619) - (assign24350_e24615 * (locals.var_sp_ov_p_dn9 + (locals.var_sp_ov_temp_dn9 / (2.0 * assign24350_e24618))))) / (assign24350_e24619 * assign24350_e24619)),)
    } else {
        (locals.var_sp_ov_u, locals.var_sp_ov_u_dn4, locals.var_sp_ov_u_dn6, locals.var_sp_ov_u_dn7, locals.var_sp_ov_u_dn8, locals.var_sp_ov_u_dn9,)
    }
};
        locals.var_sp_ov_u = assign24350_e24622;
        locals.var_sp_ov_u_dn4 = assign24350_e24622_d_n4;
        locals.var_sp_ov_u_dn6 = assign24350_e24622_d_n6;
        locals.var_sp_ov_u_dn7 = assign24350_e24622_d_n7;
        locals.var_sp_ov_u_dn8 = assign24350_e24622_d_n8;
        locals.var_sp_ov_u_dn9 = assign24350_e24622_d_n9;
        locals.var_sp_ov_u_rv = 0.0;

        let (assign24360_e24634, assign24360_e24634_d_n4, assign24360_e24634_d_n6, assign24360_e24634_d_n7, assign24360_e24634_d_n8, assign24360_e24634_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 == 0.0)) {
        let assign24360_e24632: f64 = (locals.var_sp_ov_x0 + locals.var_sp_ov_u);
        (assign24360_e24632, (locals.var_sp_ov_x0_dn4 + locals.var_sp_ov_u_dn4), (locals.var_sp_ov_x0_dn6 + locals.var_sp_ov_u_dn6), (locals.var_sp_ov_x0_dn7 + locals.var_sp_ov_u_dn7), (locals.var_sp_ov_x0_dn8 + locals.var_sp_ov_u_dn8), (locals.var_sp_ov_x0_dn9 + locals.var_sp_ov_u_dn9),)
    } else {
        (locals.var_xs_ovcv, locals.var_xs_ovcv_dn4, locals.var_xs_ovcv_dn6, locals.var_xs_ovcv_dn7, locals.var_xs_ovcv_dn8, locals.var_xs_ovcv_dn9,)
    }
};
        locals.var_xs_ovcv = assign24360_e24634;
        locals.var_xs_ovcv_dn4 = assign24360_e24634_d_n4;
        locals.var_xs_ovcv_dn6 = assign24360_e24634_d_n6;
        locals.var_xs_ovcv_dn7 = assign24360_e24634_d_n7;
        locals.var_xs_ovcv_dn8 = assign24360_e24634_d_n8;
        locals.var_xs_ovcv_dn9 = assign24360_e24634_d_n9;
        locals.var_xs_ovcv_rv = 0.0;

        let (assign24370_e24642, assign24370_e24642_d_n4, assign24370_e24642_d_n6, assign24370_e24642_d_n7, assign24370_e24642_d_n8, assign24370_e24642_d_n9,) = {
    if ((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) {
        let assign24370_e24640: f64 = (-locals.var_xs_ovcv);
        (assign24370_e24640, (-locals.var_xs_ovcv_dn4), (-locals.var_xs_ovcv_dn6), (-locals.var_xs_ovcv_dn7), (-locals.var_xs_ovcv_dn8), (-locals.var_xs_ovcv_dn9),)
    } else {
        (locals.var_xs_ovcv, locals.var_xs_ovcv_dn4, locals.var_xs_ovcv_dn6, locals.var_xs_ovcv_dn7, locals.var_xs_ovcv_dn8, locals.var_xs_ovcv_dn9,)
    }
};
        locals.var_xs_ovcv = assign24370_e24642;
        locals.var_xs_ovcv_dn4 = assign24370_e24642_d_n4;
        locals.var_xs_ovcv_dn6 = assign24370_e24642_d_n6;
        locals.var_xs_ovcv_dn7 = assign24370_e24642_d_n7;
        locals.var_xs_ovcv_dn8 = assign24370_e24642_d_n8;
        locals.var_xs_ovcv_dn9 = assign24370_e24642_d_n9;
        locals.var_xs_ovcv_rv = 0.0;

        let assign24380_e24645: f64 = (2.0 * 1.602176565e-19);
        let assign24380_e24647: f64 = (assign24380_e24645 * locals.var_novd_i);
        let assign24380_e24649: f64 = (assign24380_e24647 * locals.var_epsch);
        let assign24380_e24651: f64 = (assign24380_e24649 * locals.var_inv_phit0);
        let assign24380_e24652: f64 = (assign24380_e24651).sqrt();
        let assign24380_e24654: f64 = (assign24380_e24652 / locals.var_cox1prime);
        locals.var_gov = assign24380_e24654;
        locals.var_gov_dn4 = (((assign24380_e24649 * locals.var_inv_phit0_dn4) / (2.0 * assign24380_e24652)) / locals.var_cox1prime);
        locals.var_gov_dn6 = (((assign24380_e24649 * locals.var_inv_phit0_dn6) / (2.0 * assign24380_e24652)) / locals.var_cox1prime);
        locals.var_gov_dn7 = (((assign24380_e24649 * locals.var_inv_phit0_dn7) / (2.0 * assign24380_e24652)) / locals.var_cox1prime);
        locals.var_gov_dn8 = (((assign24380_e24649 * locals.var_inv_phit0_dn8) / (2.0 * assign24380_e24652)) / locals.var_cox1prime);
        locals.var_gov_dn9 = (((assign24380_e24649 * locals.var_inv_phit0_dn9) / (2.0 * assign24380_e24652)) / locals.var_cox1prime);
        locals.var_gov_rv = 0.0;

        let assign24390_e24657: f64 = (locals.var_gov * locals.var_gov);
        locals.var_gov2 = assign24390_e24657;
        locals.var_gov2_dn4 = ((locals.var_gov_dn4 * locals.var_gov) + (locals.var_gov * locals.var_gov_dn4));
        locals.var_gov2_dn6 = ((locals.var_gov_dn6 * locals.var_gov) + (locals.var_gov * locals.var_gov_dn6));
        locals.var_gov2_dn7 = ((locals.var_gov_dn7 * locals.var_gov) + (locals.var_gov * locals.var_gov_dn7));
        locals.var_gov2_dn8 = ((locals.var_gov_dn8 * locals.var_gov) + (locals.var_gov * locals.var_gov_dn8));
        locals.var_gov2_dn9 = ((locals.var_gov_dn9 * locals.var_gov) + (locals.var_gov * locals.var_gov_dn9));
        locals.var_gov2_rv = 0.0;

        let assign24400_e24661: f64 = (locals.var_gov / 1.4142135623731);
        let assign24400_e24662: f64 = (1.0 + assign24400_e24661);
        locals.var_xi_ov = assign24400_e24662;
        locals.var_xi_ov_dn4 = (locals.var_gov_dn4 / 1.4142135623731);
        locals.var_xi_ov_dn6 = (locals.var_gov_dn6 / 1.4142135623731);
        locals.var_xi_ov_dn7 = (locals.var_gov_dn7 / 1.4142135623731);
        locals.var_xi_ov_dn8 = (locals.var_gov_dn8 / 1.4142135623731);
        locals.var_xi_ov_dn9 = (locals.var_gov_dn9 / 1.4142135623731);
        locals.var_xi_ov_rv = 0.0;

        let assign24410_e24665: f64 = (1e-5 * locals.var_xi_ov);
        locals.var_x_mrg_ov = assign24410_e24665;
        locals.var_x_mrg_ov_dn4 = (1e-5 * locals.var_xi_ov_dn4);
        locals.var_x_mrg_ov_dn6 = (1e-5 * locals.var_xi_ov_dn6);
        locals.var_x_mrg_ov_dn7 = (1e-5 * locals.var_xi_ov_dn7);
        locals.var_x_mrg_ov_dn8 = (1e-5 * locals.var_xi_ov_dn8);
        locals.var_x_mrg_ov_dn9 = (1e-5 * locals.var_xi_ov_dn9);
        locals.var_x_mrg_ov_rv = 0.0;

        let assign24420_e24668: f64 = (1.0 / locals.var_xi_ov);
        locals.var_inv_xi_ov = assign24420_e24668;
        locals.var_inv_xi_ov_dn4 = (-(locals.var_xi_ov_dn4 / (locals.var_xi_ov * locals.var_xi_ov)));
        locals.var_inv_xi_ov_dn6 = (-(locals.var_xi_ov_dn6 / (locals.var_xi_ov * locals.var_xi_ov)));
        locals.var_inv_xi_ov_dn7 = (-(locals.var_xi_ov_dn7 / (locals.var_xi_ov * locals.var_xi_ov)));
        locals.var_inv_xi_ov_dn8 = (-(locals.var_xi_ov_dn8 / (locals.var_xi_ov * locals.var_xi_ov)));
        locals.var_inv_xi_ov_dn9 = (-(locals.var_xi_ov_dn9 / (locals.var_xi_ov * locals.var_xi_ov)));
        locals.var_inv_xi_ov_rv = 0.0;

        let assign24430_e24673: f64 = (locals.var_gov * 0.7324648775608221);
        let assign24430_e24674: f64 = (1.25 + assign24430_e24673);
        let assign24430_e24675: f64 = (1.0 / assign24430_e24674);
        locals.var_inv_xg1 = assign24430_e24675;
        locals.var_inv_xg1_dn4 = (-((locals.var_gov_dn4 * 0.7324648775608221) / (assign24430_e24674 * assign24430_e24674)));
        locals.var_inv_xg1_dn6 = (-((locals.var_gov_dn6 * 0.7324648775608221) / (assign24430_e24674 * assign24430_e24674)));
        locals.var_inv_xg1_dn7 = (-((locals.var_gov_dn7 * 0.7324648775608221) / (assign24430_e24674 * assign24430_e24674)));
        locals.var_inv_xg1_dn8 = (-((locals.var_gov_dn8 * 0.7324648775608221) / (assign24430_e24674 * assign24430_e24674)));
        locals.var_inv_xg1_dn9 = (-((locals.var_gov_dn9 * 0.7324648775608221) / (assign24430_e24674 * assign24430_e24674)));
        locals.var_inv_xg1_rv = 0.0;

        let assign24440_e24694: f64 = if (((p.p3 > 0.0) && ((locals.var_igovinvd_i > 0.0) || (locals.var_igovaccd_i > 0.0))) || ((p.p4 > 0.0) && (locals.var_agidld_i > 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard703 = assign24440_e24694;
        locals.var_guard703_rv = 0.0;

        let assign24450_e24696: f64 = (locals.var_xgd_ov).abs();
        let assign24450_e24698: f64 = if assign24450_e24696 <= locals.var_x_mrg_ov { 1.0 } else { 0.0 };
        locals.var_guard704 = assign24450_e24698;
        locals.var_guard704_rv = 0.0;

        let (assign24460_e24707, assign24460_e24707_d_n4, assign24460_e24707_d_n6, assign24460_e24707_d_n7, assign24460_e24707_d_n8, assign24460_e24707_d_n9,) = {
    if ((locals.var_guard703 != 0.0) && (locals.var_guard704 != 0.0)) {
        let assign24460_e24703: f64 = (-locals.var_xgd_ov);
        let assign24460_e24705: f64 = (assign24460_e24703 * locals.var_inv_xi_ov);
        (assign24460_e24705, (((-locals.var_xgd_ov_dn4) * locals.var_inv_xi_ov) + (assign24460_e24703 * locals.var_inv_xi_ov_dn4)), (((-locals.var_xgd_ov_dn6) * locals.var_inv_xi_ov) + (assign24460_e24703 * locals.var_inv_xi_ov_dn6)), (((-locals.var_xgd_ov_dn7) * locals.var_inv_xi_ov) + (assign24460_e24703 * locals.var_inv_xi_ov_dn7)), (((-locals.var_xgd_ov_dn8) * locals.var_inv_xi_ov) + (assign24460_e24703 * locals.var_inv_xi_ov_dn8)), (((-locals.var_xgd_ov_dn9) * locals.var_inv_xi_ov) + (assign24460_e24703 * locals.var_inv_xi_ov_dn9)),)
    } else {
        (locals.var_xd_ov, locals.var_xd_ov_dn4, locals.var_xd_ov_dn6, locals.var_xd_ov_dn7, locals.var_xd_ov_dn8, locals.var_xd_ov_dn9,)
    }
};
        locals.var_xd_ov = assign24460_e24707;
        locals.var_xd_ov_dn4 = assign24460_e24707_d_n4;
        locals.var_xd_ov_dn6 = assign24460_e24707_d_n6;
        locals.var_xd_ov_dn7 = assign24460_e24707_d_n7;
        locals.var_xd_ov_dn8 = assign24460_e24707_d_n8;
        locals.var_xd_ov_dn9 = assign24460_e24707_d_n9;
        locals.var_xd_ov_rv = 0.0;

        let assign24470_e24710: f64 = (-locals.var_x_mrg_ov);
        let assign24470_e24711: f64 = if locals.var_xgd_ov < assign24470_e24710 { 1.0 } else { 0.0 };
        locals.var_guard705 = assign24470_e24711;
        locals.var_guard705_rv = 0.0;

        let (assign24480_e24721, assign24480_e24721_d_n4, assign24480_e24721_d_n6, assign24480_e24721_d_n7, assign24480_e24721_d_n8, assign24480_e24721_d_n9,) = {
    if (((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 != 0.0)) {
        let assign24480_e24719: f64 = (-locals.var_xgd_ov);
        (assign24480_e24719, (-locals.var_xgd_ov_dn4), (-locals.var_xgd_ov_dn6), (-locals.var_xgd_ov_dn7), (-locals.var_xgd_ov_dn8), (-locals.var_xgd_ov_dn9),)
    } else {
        (locals.var_sp_ov_ygf, locals.var_sp_ov_ygf_dn4, locals.var_sp_ov_ygf_dn6, locals.var_sp_ov_ygf_dn7, locals.var_sp_ov_ygf_dn8, locals.var_sp_ov_ygf_dn9,)
    }
};
        locals.var_sp_ov_ygf = assign24480_e24721;
        locals.var_sp_ov_ygf_dn4 = assign24480_e24721_d_n4;
        locals.var_sp_ov_ygf_dn6 = assign24480_e24721_d_n6;
        locals.var_sp_ov_ygf_dn7 = assign24480_e24721_d_n7;
        locals.var_sp_ov_ygf_dn8 = assign24480_e24721_d_n8;
        locals.var_sp_ov_ygf_dn9 = assign24480_e24721_d_n9;
        locals.var_sp_ov_ygf_rv = 0.0;

        let (assign24490_e24734, assign24490_e24734_d_n4, assign24490_e24734_d_n6, assign24490_e24734_d_n7, assign24490_e24734_d_n8, assign24490_e24734_d_n9,) = {
    if (((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 != 0.0)) {
        let assign24490_e24730: f64 = (1.25 * locals.var_sp_ov_ygf);
        let assign24490_e24732: f64 = (assign24490_e24730 * locals.var_inv_xi_ov);
        (assign24490_e24732, (((1.25 * locals.var_sp_ov_ygf_dn4) * locals.var_inv_xi_ov) + (assign24490_e24730 * locals.var_inv_xi_ov_dn4)), (((1.25 * locals.var_sp_ov_ygf_dn6) * locals.var_inv_xi_ov) + (assign24490_e24730 * locals.var_inv_xi_ov_dn6)), (((1.25 * locals.var_sp_ov_ygf_dn7) * locals.var_inv_xi_ov) + (assign24490_e24730 * locals.var_inv_xi_ov_dn7)), (((1.25 * locals.var_sp_ov_ygf_dn8) * locals.var_inv_xi_ov) + (assign24490_e24730 * locals.var_inv_xi_ov_dn8)), (((1.25 * locals.var_sp_ov_ygf_dn9) * locals.var_inv_xi_ov) + (assign24490_e24730 * locals.var_inv_xi_ov_dn9)),)
    } else {
        (locals.var_sp_ov_z, locals.var_sp_ov_z_dn4, locals.var_sp_ov_z_dn6, locals.var_sp_ov_z_dn7, locals.var_sp_ov_z_dn8, locals.var_sp_ov_z_dn9,)
    }
};
        locals.var_sp_ov_z = assign24490_e24734;
        locals.var_sp_ov_z_dn4 = assign24490_e24734_d_n4;
        locals.var_sp_ov_z_dn6 = assign24490_e24734_d_n6;
        locals.var_sp_ov_z_dn7 = assign24490_e24734_d_n7;
        locals.var_sp_ov_z_dn8 = assign24490_e24734_d_n8;
        locals.var_sp_ov_z_dn9 = assign24490_e24734_d_n9;
        locals.var_sp_ov_z_rv = 0.0;

        let (assign24500_e24758, assign24500_e24758_d_n4, assign24500_e24758_d_n6, assign24500_e24758_d_n7, assign24500_e24758_d_n8, assign24500_e24758_d_n9,) = {
    if (((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 != 0.0)) {
        let assign24500_e24744: f64 = (locals.var_sp_ov_z + 10.0);
        let assign24500_e24747: f64 = (locals.var_sp_ov_z - 6.0);
        let assign24500_e24750: f64 = (locals.var_sp_ov_z - 6.0);
        let assign24500_e24751: f64 = (assign24500_e24747 * assign24500_e24750);
        let assign24500_e24753: f64 = (assign24500_e24751 + 64.0);
        let assign24500_e24754: f64 = (assign24500_e24753).sqrt();
        let assign24500_e24755: f64 = (assign24500_e24744 - assign24500_e24754);
        let assign24500_e24756: f64 = (0.5 * assign24500_e24755);
        (assign24500_e24756, (0.5 * (locals.var_sp_ov_z_dn4 - (((locals.var_sp_ov_z_dn4 * assign24500_e24750) + (assign24500_e24747 * locals.var_sp_ov_z_dn4)) / (2.0 * assign24500_e24754)))), (0.5 * (locals.var_sp_ov_z_dn6 - (((locals.var_sp_ov_z_dn6 * assign24500_e24750) + (assign24500_e24747 * locals.var_sp_ov_z_dn6)) / (2.0 * assign24500_e24754)))), (0.5 * (locals.var_sp_ov_z_dn7 - (((locals.var_sp_ov_z_dn7 * assign24500_e24750) + (assign24500_e24747 * locals.var_sp_ov_z_dn7)) / (2.0 * assign24500_e24754)))), (0.5 * (locals.var_sp_ov_z_dn8 - (((locals.var_sp_ov_z_dn8 * assign24500_e24750) + (assign24500_e24747 * locals.var_sp_ov_z_dn8)) / (2.0 * assign24500_e24754)))), (0.5 * (locals.var_sp_ov_z_dn9 - (((locals.var_sp_ov_z_dn9 * assign24500_e24750) + (assign24500_e24747 * locals.var_sp_ov_z_dn9)) / (2.0 * assign24500_e24754)))),)
    } else {
        (locals.var_sp_ov_eta, locals.var_sp_ov_eta_dn4, locals.var_sp_ov_eta_dn6, locals.var_sp_ov_eta_dn7, locals.var_sp_ov_eta_dn8, locals.var_sp_ov_eta_dn9,)
    }
};
        locals.var_sp_ov_eta = assign24500_e24758;
        locals.var_sp_ov_eta_dn4 = assign24500_e24758_d_n4;
        locals.var_sp_ov_eta_dn6 = assign24500_e24758_d_n6;
        locals.var_sp_ov_eta_dn7 = assign24500_e24758_d_n7;
        locals.var_sp_ov_eta_dn8 = assign24500_e24758_d_n8;
        locals.var_sp_ov_eta_dn9 = assign24500_e24758_d_n9;
        locals.var_sp_ov_eta_rv = 0.0;

        let (assign24510_e24779, assign24510_e24779_d_n4, assign24510_e24779_d_n6, assign24510_e24779_d_n7, assign24510_e24779_d_n8, assign24510_e24779_d_n9,) = {
    if (((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 != 0.0)) {
        let assign24510_e24767: f64 = (locals.var_sp_ov_ygf - locals.var_sp_ov_eta);
        let assign24510_e24770: f64 = (locals.var_sp_ov_ygf - locals.var_sp_ov_eta);
        let assign24510_e24771: f64 = (assign24510_e24767 * assign24510_e24770);
        let assign24510_e24775: f64 = (locals.var_sp_ov_eta + 1.0);
        let assign24510_e24776: f64 = (locals.var_gov2 * assign24510_e24775);
        let assign24510_e24777: f64 = (assign24510_e24771 + assign24510_e24776);
        (assign24510_e24777, ((((locals.var_sp_ov_ygf_dn4 - locals.var_sp_ov_eta_dn4) * assign24510_e24770) + (assign24510_e24767 * (locals.var_sp_ov_ygf_dn4 - locals.var_sp_ov_eta_dn4))) + ((locals.var_gov2_dn4 * assign24510_e24775) + (locals.var_gov2 * locals.var_sp_ov_eta_dn4))), ((((locals.var_sp_ov_ygf_dn6 - locals.var_sp_ov_eta_dn6) * assign24510_e24770) + (assign24510_e24767 * (locals.var_sp_ov_ygf_dn6 - locals.var_sp_ov_eta_dn6))) + ((locals.var_gov2_dn6 * assign24510_e24775) + (locals.var_gov2 * locals.var_sp_ov_eta_dn6))), ((((locals.var_sp_ov_ygf_dn7 - locals.var_sp_ov_eta_dn7) * assign24510_e24770) + (assign24510_e24767 * (locals.var_sp_ov_ygf_dn7 - locals.var_sp_ov_eta_dn7))) + ((locals.var_gov2_dn7 * assign24510_e24775) + (locals.var_gov2 * locals.var_sp_ov_eta_dn7))), ((((locals.var_sp_ov_ygf_dn8 - locals.var_sp_ov_eta_dn8) * assign24510_e24770) + (assign24510_e24767 * (locals.var_sp_ov_ygf_dn8 - locals.var_sp_ov_eta_dn8))) + ((locals.var_gov2_dn8 * assign24510_e24775) + (locals.var_gov2 * locals.var_sp_ov_eta_dn8))), ((((locals.var_sp_ov_ygf_dn9 - locals.var_sp_ov_eta_dn9) * assign24510_e24770) + (assign24510_e24767 * (locals.var_sp_ov_ygf_dn9 - locals.var_sp_ov_eta_dn9))) + ((locals.var_gov2_dn9 * assign24510_e24775) + (locals.var_gov2 * locals.var_sp_ov_eta_dn9))),)
    } else {
        (locals.var_sp_ov_a, locals.var_sp_ov_a_dn4, locals.var_sp_ov_a_dn6, locals.var_sp_ov_a_dn7, locals.var_sp_ov_a_dn8, locals.var_sp_ov_a_dn9,)
    }
};
        locals.var_sp_ov_a = assign24510_e24779;
        locals.var_sp_ov_a_dn4 = assign24510_e24779_d_n4;
        locals.var_sp_ov_a_dn6 = assign24510_e24779_d_n6;
        locals.var_sp_ov_a_dn7 = assign24510_e24779_d_n7;
        locals.var_sp_ov_a_dn8 = assign24510_e24779_d_n8;
        locals.var_sp_ov_a_dn9 = assign24510_e24779_d_n9;
        locals.var_sp_ov_a_rv = 0.0;

    }
}
